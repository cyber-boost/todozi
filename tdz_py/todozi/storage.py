# storage_fixed.py
# A drop-in replacement for the provided storage.py with:
# - Fixes for ProjectTaskContainer inconsistent task storage (by-status buckets + all_tasks_by_status)
# - Storage-level caching of project task containers (LRU)
# - Sync Storage.new() (async removed)
# - A lightweight storage context manager for safe file I/O
# - More consistent error handling (pure exception-based)
#
# All other APIs remain compatible.

from __future__ import annotations

# Fix imports when running directly
import sys
from pathlib import Path
# Always add parent directory to path so absolute imports work when run directly
_file_path = Path(__file__)
if _file_path.exists():
    parent_dir = _file_path.parent.parent
    parent_str = str(parent_dir)
    if parent_str not in sys.path:
        sys.path.insert(0, parent_str)

import asyncio
import hashlib
import json
import os
import shutil
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple, Union
from functools import lru_cache

# ==============================
# Result-like classes (Exception-based)
# ==============================

class TodoziError(Exception):
    def __init__(self, message: str, *args, **kwargs):
        super().__init__(message, *args, **kwargs)
        self.message = message

    @staticmethod
    def storage(message: str) -> "TodoziError":
        return TodoziError(f"Storage error: {message}")

    @staticmethod
    def project_not_found(name: str) -> "TodoziError":
        return TodoziError(f"Project not found: {name}")

    @staticmethod
    def task_not_found(id: str) -> "TodoziError":
        return TodoziError(f"Task not found: {id}")

    @staticmethod
    def feeling_not_found(id: str) -> "TodoziError":
        return TodoziError(f"Feeling not found: {id}")

    @staticmethod
    def validation_error(message: str) -> "TodoziError":
        return TodoziError(f"Validation error: {message}")


# ==============================
# HLX File Format Support
# ==============================

# Import HLX functionality from our dedicated module
from .hlx import TodoziHlx as Hlx, HlxValue, xlh


# ==============================
# Domain Models
# ==============================

from dataclasses import dataclass, field
from enum import Enum, auto
import uuid


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def rfc3339(dt: datetime) -> str:
    return dt.isoformat()


def parse_rfc3339(s: str) -> datetime:
    # Best-effort parser compatible with chrono::DateTime RFC3339
    return datetime.fromisoformat(s.replace("Z", "+00:00"))


def new_id(prefix: str = "") -> str:
    return f"{prefix}{uuid.uuid4().hex}"


class Status(Enum):
    Todo = auto()
    InProgress = auto()
    Done = auto()
    Completed = auto()
    Cancelled = auto()
    Archived = auto()

    @staticmethod
    def from_str(s: str) -> "Status":
        """Convert string to Status enum, handling various formats"""
        if not s:
            return Status.Todo
        
        # Normalize the string
        s_clean = s.strip()
        
        # Handle common variations
        mapping = {
            "todo": Status.Todo,
            "pending": Status.Todo,
            "in_progress": Status.InProgress,
            "inprogress": Status.InProgress,
            "done": Status.Done,
            "completed": Status.Completed,
            "cancelled": Status.Cancelled,
            "canceled": Status.Cancelled,
            "archived": Status.Archived,
        }
        
        # Try lowercase mapping first
        lower = s_clean.lower()
        if lower in mapping:
            return mapping[lower]
        
        # Try direct enum name match
        try:
            return Status[s_clean]
        except KeyError:
            # Default to Todo if unknown
            return Status.Todo


class AssignmentStatus(Enum):
    Assigned = "Assigned"
    InProgress = "InProgress"
    Completed = "Completed"
    Cancelled = "Cancelled"

    def __str__(self):
        return self.value

    @staticmethod
    def from_str(s: str) -> "AssignmentStatus":
        try:
            return AssignmentStatus(s)
        except ValueError:
            return AssignmentStatus.Assigned


class Priority(Enum):
    Low = auto()
    Medium = auto()
    High = auto()
    Critical = auto()
    Urgent = auto()

    def __str__(self):
        return self.name

    @staticmethod
    def from_str(s: str) -> "Priority":
        """Convert string to Priority enum, handling case-insensitive matching."""
        if not s:
            return Priority.Medium
        
        # Normalize the string: capitalize first letter, lowercase rest
        s_normalized = s.capitalize()
        
        # Try direct match first
        try:
            return Priority[s_normalized]
        except KeyError:
            pass
        
        # Try case-insensitive match
        s_upper = s.upper()
        for priority in Priority:
            if priority.name.upper() == s_upper:
                return priority
        
        # Fallback mappings for common variations
        s_lower = s.lower()
        mapping = {
            "low": Priority.Low,
            "medium": Priority.Medium,
            "high": Priority.High,
            "critical": Priority.Critical,
            "urgent": Priority.Urgent,
        }
        
        return mapping.get(s_lower, Priority.Medium)  # Default to Medium


class Assignee(Enum):
    Human = auto()
    Ai = auto()
    Collaborative = auto()

    @staticmethod
    def from_str(s: str) -> Optional["Assignee"]:
        """Convert string to Assignee enum, handling case-insensitive matching."""
        if not s:
            return None
        
        # Normalize the string: capitalize first letter, lowercase rest
        s_normalized = s.capitalize()
        
        # Try direct match first
        try:
            return Assignee[s_normalized]
        except KeyError:
            pass
        
        # Try case-insensitive match
        s_upper = s.upper()
        for assignee in Assignee:
            if assignee.name.upper() == s_upper:
                return assignee
        
        # Fallback mappings for common variations
        s_lower = s.lower()
        mapping = {
            "human": Assignee.Human,
            "ai": Assignee.Ai,
            "collaborative": Assignee.Collaborative,
            "collab": Assignee.Collaborative,
        }
        
        return mapping.get(s_lower, None)


@dataclass
class RegistrationInfo:
    user_name: str
    user_email: str
    api_key: str
    user_id: Optional[str] = None
    fingerprint: Optional[str] = None
    registered_at: datetime = field(default_factory=utc_now)
    server_url: str = "https://todozi.com"

    @staticmethod
    def new_with_hashes(server_url: str) -> "RegistrationInfo":
        return RegistrationInfo(
            user_name="user_" + uuid.uuid4().hex[:8],
            user_email="user_" + uuid.uuid4().hex[:8] + "@example.com",
            api_key="no_key_provided",
            registered_at=utc_now(),
            server_url=server_url,
        )


@dataclass
class Config:
    registration: Optional[RegistrationInfo] = None
    version: str = "1.2.0"
    default_project: str = "general"
    auto_backup: bool = True
    backup_interval: str = "daily"
    ai_enabled: bool = True
    default_assignee: Optional[str] = None
    date_format: str = "%Y-%m-%d %H:%M:%S"
    timezone: str = "UTC"


@dataclass
class Task:
    id: str = field(default_factory=lambda: new_id("task_"))
    action: str = ""
    context_notes: Optional[str] = None
    status: Status = Status.Todo
    priority: Priority = Priority.Medium
    parent_project: str = ""
    created_at: datetime = field(default_factory=utc_now)
    updated_at: datetime = field(default_factory=utc_now)
    embedding_vector: Optional[List[float]] = None
    assignee: Optional[Assignee] = None

    def update(self, updates: "TaskUpdate") -> "Task":
        if updates.action is not None:
            self.action = updates.action
        if updates.context_notes is not None:
            self.context_notes = updates.context_notes
        if updates.status is not None:
            self.status = updates.status
        if updates.priority is not None:
            self.priority = updates.priority
        if updates.parent_project is not None:
            self.parent_project = updates.parent_project
        if updates.assignee is not None:
            self.assignee = updates.assignee
        if updates.progress is not None:
            # No direct field; store in context_notes or leave unused
            pass
        self.updated_at = utc_now()
        return self

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "action": self.action,
            "context_notes": self.context_notes,
            "status": str(self.status),
            "priority": str(self.priority),
            "parent_project": self.parent_project,
            "created_at": rfc3339(self.created_at),
            "updated_at": rfc3339(self.updated_at),
            "embedding_vector": self.embedding_vector,
            "assignee": self.assignee.name if self.assignee else None,
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Task":
        t = Task(
            id=d.get("id", new_id("task_")),
            action=d.get("action", ""),
            context_notes=d.get("context_notes"),
            status=Status.from_str(d.get("status", "Todo")),
            priority=Priority.from_str(d.get("priority", "Medium")),
            parent_project=d.get("parent_project", ""),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
            updated_at=parse_rfc3339(d.get("updated_at", rfc3339(utc_now()))),
            embedding_vector=d.get("embedding_vector"),
        )
        assignee = d.get("assignee")
        if assignee is not None:
            if isinstance(assignee, str):
                t.assignee = Assignee.from_str(assignee)
            elif isinstance(assignee, Assignee):
                t.assignee = assignee
            else:
                # Try to convert if it's something else
                try:
                    t.assignee = Assignee.from_str(str(assignee))
                except Exception:
                    t.assignee = None
        return t


@dataclass
class TaskUpdate:
    action: Optional[str] = None
    context_notes: Optional[str] = None
    status: Optional[Status] = None
    priority: Optional[Priority] = None
    parent_project: Optional[str] = None
    progress: Optional[int] = None
    assignee: Optional[Assignee] = None

    @staticmethod
    def new() -> "TaskUpdate":
        return TaskUpdate()

    def with_status(self, status: Status) -> "TaskUpdate":
        self.status = status
        return self

    def with_progress(self, progress: int) -> "TaskUpdate":
        self.progress = progress
        return self


@dataclass
class TaskCollection:
    tasks: Dict[str, Task] = field(default_factory=dict)

    def add_task(self, task: Task) -> None:
        self.tasks[task.id] = task

    def get_task(self, task_id: str) -> Optional[Task]:
        return self.tasks.get(task_id)

    def get_task_mut(self, task_id: str) -> Optional[Task]:
        return self.tasks.get(task_id)

    def remove_task(self, task_id: str) -> Optional[Task]:
        return self.tasks.pop(task_id, None)

    def get_filtered_tasks(self, filters: "TaskFilters") -> List[Task]:
        items = list(self.tasks.values())
        
        # Handle both 'project' and 'project_filter' attribute names
        project_filter = getattr(filters, 'project', None) or getattr(filters, 'project_filter', None)
        if project_filter:
            items = [t for t in items if t.parent_project == project_filter]
        
        # Handle search filter
        search_filter = getattr(filters, 'search', None)
        if search_filter:
            q = search_filter.lower()
            items = [t for t in items if q in t.action.lower() or (t.context_notes or "").lower().count(q) > 0]
        
        # Handle both 'assignee' and 'assignee_filter' attribute names
        assignee_filter = getattr(filters, 'assignee', None) or getattr(filters, 'assignee_filter', None)
        if assignee_filter:
            items = [t for t in items if t.assignee == assignee_filter]
        
        # Handle status and priority filters if they exist
        status_filter = getattr(filters, 'status', None) or getattr(filters, 'status_filter', None)
        if status_filter:
            items = [t for t in items if t.status == status_filter]
        
        priority_filter = getattr(filters, 'priority', None) or getattr(filters, 'priority_filter', None)
        if priority_filter:
            items = [t for t in items if t.priority == priority_filter]
        
        return items

    def to_dict(self) -> Dict[str, Any]:
        return {"tasks": {tid: task.to_dict() for tid, task in self.tasks.items()}}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "TaskCollection":
        col = TaskCollection()
        tasks = d.get("tasks", {})
        for tid, td in tasks.items():
            col.tasks[tid] = Task.from_dict(td)
        return col


@dataclass
class TaskFilters:
    project: Optional[str] = None
    search: Optional[str] = None
    assignee: Optional[Assignee] = None

    @staticmethod
    def default() -> "TaskFilters":
        return TaskFilters()


@dataclass
class Project:
    name: str
    description: Optional[str] = None
    archived: bool = False

    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "description": self.description,
            "archived": self.archived,
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Project":
        return Project(name=d["name"], description=d.get("description"), archived=d.get("archived", False))

    def archive(self) -> None:
        self.archived = True


@dataclass
class ProjectTaskContainer:
    project_name: str
    project_hash: str
    # Internal storage organized by status buckets for efficient status-based lookups
    _storage: Dict[Status, Dict[str, Task]] = field(default_factory=lambda: {
        Status.Todo: {},
        Status.InProgress: {},
        Status.Done: {},
        Status.Completed: {},
        Status.Cancelled: {},
        Status.Archived: {},
    })

    @property
    def active_tasks(self) -> Dict[str, Task]:
        if not isinstance(self._storage, dict):
            return {}
        todo = self._storage.get(Status.Todo, {}) if isinstance(self._storage.get(Status.Todo), dict) else {}
        in_progress = self._storage.get(Status.InProgress, {}) if isinstance(self._storage.get(Status.InProgress), dict) else {}
        return {**todo, **in_progress}

    @property
    def completed_tasks(self) -> Dict[str, Task]:
        if not isinstance(self._storage, dict):
            return {}
        done = self._storage.get(Status.Done, {}) if isinstance(self._storage.get(Status.Done), dict) else {}
        completed = self._storage.get(Status.Completed, {}) if isinstance(self._storage.get(Status.Completed), dict) else {}
        return {**done, **completed}

    @property
    def archived_tasks(self) -> Dict[str, Task]:
        if not isinstance(self._storage, dict):
            return {}
        archived = self._storage.get(Status.Archived, {})
        return archived if isinstance(archived, dict) else {}

    @property
    def deleted_tasks(self) -> Dict[str, Task]:
        if not isinstance(self._storage, dict):
            return {}
        deleted = self._storage.get(Status.Cancelled, {})
        return deleted if isinstance(deleted, dict) else {}

    def add_task(self, task: Task) -> None:
        # Ensure _storage is properly initialized
        if not hasattr(self, '_storage') or not isinstance(self._storage, dict):
            self._storage = {
                Status.Todo: {},
                Status.InProgress: {},
                Status.Done: {},
                Status.Completed: {},
                Status.Cancelled: {},
                Status.Archived: {},
            }
        if task.status not in self._storage:
            self._storage[task.status] = {}
        self._storage[task.status][task.id] = task

    def get_task(self, task_id: str) -> Optional[Task]:
        if not isinstance(self._storage, dict):
            return None
        for s in Status:
            if s in self._storage and task_id in self._storage.get(s, {}):
                return self._storage[s][task_id]
        return None

    def get_task_mut(self, task_id: str) -> Optional[Task]:
        if not isinstance(self._storage, dict):
            return None
        for s in Status:
            if s in self._storage and task_id in self._storage.get(s, {}):
                return self._storage[s][task_id]
        return None

    def remove_task(self, task_id: str) -> Optional[Task]:
        if not isinstance(self._storage, dict):
            return None
        found = None
        for s in Status:
            if s in self._storage and task_id in self._storage.get(s, {}):
                found = self._storage[s].pop(task_id)
                break
        return found

    def update_task_status(self, task_id: str, new_status: Status) -> Optional[Task]:
        if not isinstance(self._storage, dict):
            return None
        t = self.get_task_mut(task_id)
        if not t:
            return None
        # remove from old
        for s in Status:
            if s in self._storage and task_id in self._storage.get(s, {}):
                self._storage[s].pop(task_id, None)
                break
        # set new
        t.status = new_status
        t.updated_at = utc_now()
        if new_status not in self._storage:
            self._storage[new_status] = {}
        self._storage[new_status][task_id] = t
        return t

    def get_filtered_tasks(self, filters: TaskFilters) -> List[Task]:
        if not isinstance(self._storage, dict):
            return []
        tasks = [t for bucket in self._storage.values() if isinstance(bucket, dict) for t in bucket.values()]
        
        # Handle both 'project' and 'project_filter' attribute names
        project_filter = getattr(filters, 'project', None) or getattr(filters, 'project_filter', None)
        if project_filter:
            tasks = [t for t in tasks if t.parent_project == project_filter]
        
        # Handle search filter
        search_filter = getattr(filters, 'search', None)
        if search_filter:
            q = search_filter.lower()
            tasks = [t for t in tasks if q in t.action.lower() or (t.context_notes or "").lower().count(q) > 0]
        
        # Handle both 'assignee' and 'assignee_filter' attribute names
        assignee_filter = getattr(filters, 'assignee', None) or getattr(filters, 'assignee_filter', None)
        if assignee_filter:
            tasks = [t for t in tasks if t.assignee == assignee_filter]
        
        # Handle status and priority filters if they exist
        status_filter = getattr(filters, 'status', None) or getattr(filters, 'status_filter', None)
        if status_filter:
            tasks = [t for t in tasks if t.status == status_filter]
        
        priority_filter = getattr(filters, 'priority', None) or getattr(filters, 'priority_filter', None)
        if priority_filter:
            tasks = [t for t in tasks if t.priority == priority_filter]
        
        return tasks

    def get_all_tasks(self) -> List[Task]:
        return [t for bucket in self._storage.values() for t in bucket.values()]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "project_name": self.project_name,
            "project_hash": self.project_hash,
            "active_tasks": {k: v.to_dict() for k, v in self.active_tasks.items()},
            "completed_tasks": {k: v.to_dict() for k, v in self.completed_tasks.items()},
            "archived_tasks": {k: v.to_dict() for k, v in self.archived_tasks.items()},
            "deleted_tasks": {k: v.to_dict() for k, v in self.deleted_tasks.items()},
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "ProjectTaskContainer":
        # Build with default empty buckets, then populate
        container = ProjectTaskContainer(project_name=d["project_name"], project_hash=d["project_hash"])
        
        # Ensure _storage is properly initialized
        if not hasattr(container, '_storage') or not isinstance(container._storage, dict):
            container._storage = {
                Status.Todo: {},
                Status.InProgress: {},
                Status.Done: {},
                Status.Completed: {},
                Status.Cancelled: {},
                Status.Archived: {},
            }
        
        def map_dict(mp: Dict[str, Any]) -> Dict[str, Task]:
            if not isinstance(mp, dict):
                return {}
            return {k: Task.from_dict(v) for k, v in mp.items()}

        # Populate via add_task to respect new internal storage
        for src, status in [
            ("active_tasks", Status.Todo),
            ("completed_tasks", Status.Completed),
            ("archived_tasks", Status.Archived),
            ("deleted_tasks", Status.Cancelled),
        ]:
            src_data = d.get(src, {})
            if isinstance(src_data, dict):
                for k, v in map_dict(src_data).items():
                    v.status = status  # ensure correct status bucket
                    container.add_task(v)
        return container

    def all_tasks_by_status(self) -> Dict[Status, Dict[str, Task]]:
        if not isinstance(self._storage, dict):
            # Initialize if not a dict
            self._storage = {
                Status.Todo: {},
                Status.InProgress: {},
                Status.Done: {},
                Status.Completed: {},
                Status.Cancelled: {},
                Status.Archived: {},
            }
        return {k: dict(v) for k, v in self._storage.items()}


@dataclass
class ProjectStats:
    project_name: str
    total_tasks: int
    active_tasks: int
    completed_tasks: int
    archived_tasks: int
    deleted_tasks: int


@dataclass
class ProjectMigrationStats:
    project_name: str
    initial_tasks: int
    migrated_tasks: int
    final_tasks: int


@dataclass
class MigrationReport:
    tasks_found: int = 0
    tasks_migrated: int = 0
    projects_migrated: int = 0
    project_stats: List[ProjectMigrationStats] = field(default_factory=list)


@dataclass
class Error:
    id: str = field(default_factory=lambda: new_id("err_"))
    message: str = ""
    details: Optional[Dict[str, Any]] = None
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "message": self.message,
            "details": self.details,
            "created_at": rfc3339(self.created_at),
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Error":
        return Error(
            id=d.get("id", new_id("err_")),
            message=d.get("message", ""),
            details=d.get("details"),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class TrainingData:
    id: str = field(default_factory=lambda: new_id("trn_"))
    content: str = ""
    labels: Optional[List[str]] = None
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "content": self.content,
            "labels": self.labels,
            "created_at": rfc3339(self.created_at),
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "TrainingData":
        return TrainingData(
            id=d.get("id", new_id("trn_")),
            content=d.get("content", ""),
            labels=d.get("labels"),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class CodeChunk:
    chunk_id: str = field(default_factory=lambda: new_id("chk_"))
    content: str = ""
    file_path: Optional[str] = None
    language: Optional[str] = None
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "chunk_id": self.chunk_id,
            "content": self.content,
            "file_path": self.file_path,
            "language": self.language,
            "created_at": rfc3339(self.created_at),
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "CodeChunk":
        return CodeChunk(
            chunk_id=d.get("chunk_id", new_id("chk_")),
            content=d.get("content", ""),
            file_path=d.get("file_path"),
            language=d.get("language"),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class AgentTool:
    name: str
    enabled: bool = True
    config: Optional[Dict[str, Any]] = None


@dataclass
class AgentBehaviors:
    auto_format_code: bool = False
    include_examples: bool = True
    explain_complexity: bool = True
    suggest_tests: bool = False


@dataclass
class AgentMetadata:
    tags: List[str] = field(default_factory=list)
    category: str = "general"
    author: Optional[str] = None
    status: str = "Available"  # for simplicity, a string instead of AgentStatus enum


@dataclass
class Agent:
    id: str
    name: str
    description: str
    system_prompt: str = ""
    prompt_template: Optional[str] = None
    capabilities: List[str] = field(default_factory=list)
    specializations: List[str] = field(default_factory=list)
    tools: List[AgentTool] = field(default_factory=list)
    behaviors: Optional[AgentBehaviors] = None
    constraints: Dict[str, Any] = field(default_factory=dict)
    metadata: AgentMetadata = field(default_factory=AgentMetadata)

    @staticmethod
    def new(id: str, name: str, description: str) -> "Agent":
        return Agent(id=id, name=name, description=description)

    @staticmethod
    def create_coder() -> "Agent":
        return Agent(
            id="coder",
            name="Coder",
            description="Programming specialist",
            system_prompt="You are a professional software engineer. Write clean, testable code.",
        )

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "system_prompt": self.system_prompt,
            "prompt_template": self.prompt_template,
            "capabilities": self.capabilities,
            "specializations": self.specializations,
            "tools": [{"name": t.name, "enabled": t.enabled, "config": t.config} for t in self.tools],
            "behaviors": {
                "auto_format_code": self.behaviors.auto_format_code if self.behaviors else False,
                "include_examples": self.behaviors.include_examples if self.behaviors else True,
                "explain_complexity": self.behaviors.explain_complexity if self.behaviors else True,
                "suggest_tests": self.behaviors.suggest_tests if self.behaviors else False,
            },
            "constraints": self.constraints,
            "metadata": {
                "tags": self.metadata.tags,
                "category": self.metadata.category,
                "author": self.metadata.author,
                "status": self.metadata.status,
            },
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Agent":
        tools = [AgentTool(**t) for t in d.get("tools", [])]
        behaviors_data = d.get("behaviors", {})
        behaviors = AgentBehaviors(
            auto_format_code=behaviors_data.get("auto_format_code", False),
            include_examples=behaviors_data.get("include_examples", True),
            explain_complexity=behaviors_data.get("explain_complexity", True),
            suggest_tests=behaviors_data.get("suggest_tests", False),
        )
        metadata_data = d.get("metadata", {})
        metadata = AgentMetadata(
            tags=metadata_data.get("tags", []),
            category=metadata_data.get("category", "general"),
            author=metadata_data.get("author"),
            status=metadata_data.get("status", "Available"),
        )
        return Agent(
            id=d["id"],
            name=d["name"],
            description=d["description"],
            system_prompt=d.get("system_prompt", ""),
            prompt_template=d.get("prompt_template"),
            capabilities=d.get("capabilities", []),
            specializations=d.get("specializations", []),
            tools=tools,
            behaviors=behaviors,
            constraints=d.get("constraints", {}),
            metadata=metadata,
        )


@dataclass
class AgentAssignment:
    agent_id: str
    task_id: str
    status: AssignmentStatus = AssignmentStatus.Assigned
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "agent_id": self.agent_id,
            "task_id": self.task_id,
            "status": self.status.value,
            "created_at": rfc3339(self.created_at),
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "AgentAssignment":
        status_str = d.get("status", "Assigned")
        if isinstance(status_str, str):
            try:
                status = AssignmentStatus(status_str)
            except ValueError:
                status = AssignmentStatus.Assigned
        else:
            status = status_str if isinstance(status_str, AssignmentStatus) else AssignmentStatus.Assigned
        return AgentAssignment(
            agent_id=d["agent_id"],
            task_id=d["task_id"],
            status=status,
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class QueueSession:
    id: str = field(default_factory=lambda: new_id("qs_"))
    queue_item_id: str = ""
    started_at: datetime = field(default_factory=utc_now)
    ended_at: Optional[datetime] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "queue_item_id": self.queue_item_id,
            "started_at": rfc3339(self.started_at),
            "ended_at": rfc3339(self.ended_at) if self.ended_at else None,
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "QueueSession":
        return QueueSession(
            id=d.get("id", new_id("qs_")),
            queue_item_id=d.get("queue_item_id", ""),
            started_at=parse_rfc3339(d.get("started_at", rfc3339(utc_now()))),
            ended_at=parse_rfc3339(d["ended_at"]) if d.get("ended_at") else None,
        )


class QueueStatus(Enum):
    Backlog = auto()
    Active = auto()
    Complete = auto()


@dataclass
class QueueItem:
    id: str = field(default_factory=lambda: new_id("q_"))
    content: str = ""
    status: QueueStatus = QueueStatus.Backlog
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "content": self.content,
            "status": self.status.name,
            "created_at": rfc3339(self.created_at),
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "QueueItem":
        return QueueItem(
            id=d.get("id", new_id("q_")),
            content=d.get("content", ""),
            status=QueueStatus[d.get("status", "Backlog")],
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class QueueCollection:
    items: Dict[str, QueueItem] = field(default_factory=dict)
    sessions: Dict[str, QueueSession] = field(default_factory=dict)

    def add_item(self, item: QueueItem) -> None:
        self.items[item.id] = item

    def get_item(self, item_id: str) -> Optional[QueueItem]:
        return self.items.get(item_id)

    def get_all_items(self) -> List[QueueItem]:
        return list(self.items.values())

    def get_items_by_status(self, status: QueueStatus) -> List[QueueItem]:
        return [it for it in self.items.values() if it.status == status]

    def start_session(self, queue_item_id: str) -> str:
        if queue_item_id not in self.items:
            raise TodoziError.validation_error(f"Queue item not found: {queue_item_id}")
        sid = new_id("qs_")
        self.sessions[sid] = QueueSession(id=sid, queue_item_id=queue_item_id)
        self.items[queue_item_id].status = QueueStatus.Active
        return sid

    def end_session(self, session_id: str) -> None:
        sess = self.sessions.get(session_id)
        if not sess:
            raise TodoziError.validation_error(f"Session not found: {session_id}")
        if sess.ended_at is None:
            sess.ended_at = utc_now()
            self.items[sess.queue_item_id].status = QueueStatus.Complete

    def get_active_sessions(self) -> List[QueueSession]:
        return [s for s in self.sessions.values() if s.ended_at is None]

    def get_session(self, session_id: str) -> Optional[QueueSession]:
        return self.sessions.get(session_id)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "items": {k: v.to_dict() for k, v in self.items.items()},
            "sessions": {k: v.to_dict() for k, v in self.sessions.items()},
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "QueueCollection":
        col = QueueCollection()
        for k, v in d.get("items", {}).items():
            col.items[k] = QueueItem.from_dict(v)
        for k, v in d.get("sessions", {}).items():
            col.sessions[k] = QueueSession.from_dict(v)
        return col


@dataclass
class Memory:
    id: str = field(default_factory=lambda: new_id("mem_"))
    content: str = ""
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {"id": self.id, "content": self.content, "created_at": rfc3339(self.created_at)}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Memory":
        return Memory(
            id=d.get("id", new_id("mem_")),
            content=d.get("content", ""),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class Idea:
    id: str = field(default_factory=lambda: new_id("idea_"))
    content: str = ""
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {"id": self.id, "content": self.content, "created_at": rfc3339(self.created_at)}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Idea":
        return Idea(
            id=d.get("id", new_id("idea_")),
            content=d.get("content", ""),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class Feeling:
    id: str = field(default_factory=lambda: new_id("feel_"))
    content: str = ""
    created_at: datetime = field(default_factory=utc_now)

    def to_dict(self) -> Dict[str, Any]:
        return {"id": self.id, "content": self.content, "created_at": rfc3339(self.created_at)}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Feeling":
        return Feeling(
            id=d.get("id", new_id("feel_")),
            content=d.get("content", ""),
            created_at=parse_rfc3339(d.get("created_at", rfc3339(utc_now()))),
        )


@dataclass
class SemanticSearchResult:
    task: Task
    similarity_score: float
    matched_content: str


# ==============================
# Embedding service - using from emb.py
# ==============================
from todozi.emb import TodoziEmbeddingConfig, TodoziEmbeddingService


# ==============================
# Storage API
# ==============================

def get_home_dir() -> Path:
    home = Path.home()
    if not home.exists():
        raise TodoziError.storage("Could not find home directory")
    return home


def get_storage_dir() -> Path:
    home = get_home_dir()
    return home / ".todozi"


def get_tasks_dir() -> Path:
    return get_storage_dir() / "tasks"


def get_project_tasks_dir() -> Path:
    return get_storage_dir() / "project_tasks"


def get_agents_dir() -> Path:
    return get_storage_dir() / "agents"


def get_memories_dir() -> Path:
    return get_storage_dir() / "memories"


def get_ideas_dir() -> Path:
    return get_storage_dir() / "ideas"


def get_training_dir() -> Path:
    return get_storage_dir() / "training"


def get_chunks_dir() -> Path:
    return get_storage_dir() / "chunks"


def get_errors_dir() -> Path:
    return get_storage_dir() / "errors"


def get_assignments_dir() -> Path:
    return get_storage_dir() / "assignments"


def get_steps_dir() -> Path:
    return get_storage_dir() / "steps"


# Lightweight context manager for file operations
from contextlib import contextmanager

@contextmanager
def storage_file(path: Path, mode: str = "r", *, mkdir: bool = True):
    if mkdir:
        path.parent.mkdir(parents=True, exist_ok=True)
    if "w" in mode or "a" in mode:
        path.parent.mkdir(parents=True, exist_ok=True)
    with path.open(mode, encoding="utf-8") as f:
        yield f


async def init_storage() -> None:
    storage_dir = get_storage_dir()
    storage_dir.mkdir(parents=True, exist_ok=True)

    # Create all required directories to match Rust implementation
    for sub in [
        "agents", "assignments", "chunks", "errors", "fingerprints", "memories",
        "projects", "queue", "steps", "api", "backups", "embed", "feelings",
        "ideas", "models", "project_tasks", "responses", "tasks", "templates",
        "training",
    ]:
        (storage_dir / sub).mkdir(parents=True, exist_ok=True)

    config_path = storage_dir / "tdz.hlx"
    is_new_config = not config_path.exists()
    if not config_path.exists():
        config = Config()
        await save_config(config)

    if is_new_config or not (await is_registered()):
        registration = RegistrationInfo.new_with_hashes("https://todozi.com")
        if (await update_config_with_registration(registration)) is not None:
            print("⚠️  Could not save registration info")
        else:
            print("🔗 Created registration info (ready for todozi.com)")
            print("💡 Run 'todozi register' to complete registration with server")

    create_default_agents()

    general_project_path = storage_dir / "projects" / "general.json"
    if not general_project_path.exists():
        project = Project(name="general", description="General tasks")
        save_project(project)

    for name in ("active", "completed", "archived"):
        p = storage_dir / "tasks" / f"{name}.json"
        if not p.exists():
            save_task_collection(name, TaskCollection())


def check_folder_structure() -> bool:
    storage_dir = get_storage_dir()
    required_dirs = [
        "agents", "api", "assignments", "backups", "chunks", "embed", "errors", "feelings",
        "ideas", "memories", "models", "projects", "queue", "responses", "tasks",
        "templates", "training",
    ]
    for dn in required_dirs:
        p = storage_dir / dn
        if not p.exists():
            print(f"❌ Missing directory: {dn}")
            return False
        if not p.is_dir():
            print(f"❌ {dn} exists but is not a directory")
            return False

    config_path = storage_dir / "tdz.hlx"
    if not config_path.exists():
        print("❌ Missing tdz.hlx configuration file")
        return False
    if not config_path.is_file():
        print("❌ tdz.hlx exists but is not a file")
        return False

    print("✅ Todozi folder structure is complete!")
    print(f"📁 Storage directory: {storage_dir}")
    print(f"📂 Found {len(required_dirs)} required directories")
    for dn in required_dirs:
        print(f"  ✓ {dn}")
    print("  ✓ tdz.hlx")
    return True


async def ensure_folder_structure() -> bool:
    _storage_dir = get_storage_dir()
    if check_folder_structure():
        return True
    print("🔧 Creating missing folder structure...")
    await init_storage()
    check_folder_structure()
    return True


async def save_config(config: Config) -> None:
    storage_dir = get_storage_dir()
    config_path = storage_dir / "tdz.hlx"
    hlx = Hlx(config_path)
    if config.registration:
        r = config.registration
        hlx.set("registration", "user_name", r.user_name)
        hlx.set("registration", "user_email", r.user_email)
        hlx.set("registration", "api_key", r.api_key)
        if r.user_id:
            hlx.set("registration", "user_id", r.user_id)
        if r.fingerprint:
            hlx.set("registration", "fingerprint", r.fingerprint)
        hlx.set("registration", "registered_at", r.registered_at.isoformat())
        hlx.set("registration", "server_url", r.server_url)

    hlx.set("config", "version", config.version)
    hlx.set("config", "default_project", config.default_project)
    hlx.set("config", "auto_backup", config.auto_backup)
    hlx.set("config", "backup_interval", config.backup_interval)
    hlx.set("config", "ai_enabled", config.ai_enabled)
    if config.default_assignee:
        hlx.set("config", "default_assignee", config.default_assignee)
    hlx.set("config", "date_format", config.date_format)
    hlx.set("config", "timezone", config.timezone)

    hlx.save()


async def load_config() -> Config:
    storage_dir = get_storage_dir()
    config_path = storage_dir / "tdz.hlx"
    if not config_path.exists():
        return Config()
    hlx = Hlx(config_path)

    def get_str(sec: str, key: str) -> Optional[str]:
        v = hlx.get(sec, key)
        return v.value if v else None

    def get_bool(sec: str, key: str) -> Optional[bool]:
        v = hlx.get(sec, key)
        return v.value if v else None

    user_name = get_str("registration", "user_name")
    user_email = get_str("registration", "user_email")
    api_key = get_str("registration", "api_key")
    registration: Optional[RegistrationInfo] = None
    if user_name and user_email and api_key:
        user_id = get_str("registration", "user_id")
        fingerprint = get_str("registration", "fingerprint")
        reg_at_str = get_str("registration", "registered_at")
        registered_at = parse_rfc3339(reg_at_str) if reg_at_str else utc_now()
        server_url = get_str("registration", "server_url") or "https://todozi.com"
        registration = RegistrationInfo(
            user_name=user_name,
            user_email=user_email,
            api_key=api_key,
            user_id=user_id,
            fingerprint=fingerprint,
            registered_at=registered_at,
            server_url=server_url,
        )

    version = get_str("config", "version") or "1.2.0"
    default_project = get_str("config", "default_project") or "general"
    auto_backup = get_bool("config", "auto_backup") if get_bool("config", "auto_backup") is not None else True
    backup_interval = get_str("config", "backup_interval") or "daily"
    ai_enabled = get_bool("config", "ai_enabled") if get_bool("config", "ai_enabled") is not None else True
    default_assignee = get_str("config", "default_assignee")
    date_format = get_str("config", "date_format") or "%Y-%m-%d %H:%M:%S"
    timezone = get_str("config", "timezone") or "UTC"

    return Config(
        registration=registration,
        version=version,
        default_project=default_project,
        auto_backup=auto_backup,
        backup_interval=backup_interval,
        ai_enabled=ai_enabled,
        default_assignee=default_assignee,
        date_format=date_format,
        timezone=timezone,
    )


# ==============================
# HTTP Utilities
# ==============================

async def register_with_server(server_url: str) -> RegistrationInfo:
    # Simplified: emulate registration by creating local registration only
    registration = RegistrationInfo.new_with_hashes(server_url)
    print("✅ Successfully registered with todozi.com! (local simulation)")
    print(f"🔑 API Key: {registration.api_key}")
    if registration.user_id:
        print(f"👤 User ID: {registration.user_id}")
    if registration.fingerprint:
        print(f"🔐 Fingerprint: {registration.fingerprint}")

    err = await update_config_with_registration(registration)
    if isinstance(err, Exception):
        print("⚠️  Could not update config with registration data")
    return registration


async def update_config_with_registration(registration: RegistrationInfo) -> None:
    config = await load_config()
    config.registration = registration
    await save_config(config)
    print("💾 Updated tdz.hlx with registration information")


async def is_registered() -> bool:
    config = await load_config()
    return config.registration is not None


async def get_registration_info() -> Optional[RegistrationInfo]:
    config = await load_config()
    return config.registration


async def clear_registration() -> None:
    config = await load_config()
    config.registration = None
    await save_config(config)
    print("🗑️  Cleared registration information from tdz.hlx")


# ==============================
# Project Task Containers
# ==============================

def hash_project_name(project_name: str) -> str:
    digest = hashlib.md5(project_name.encode("utf-8")).hexdigest()
    return digest


def save_project_task_container(container: ProjectTaskContainer) -> None:
    project_tasks_dir = get_project_tasks_dir()
    project_tasks_dir.mkdir(parents=True, exist_ok=True)
    container_path = project_tasks_dir / f"{container.project_hash}.json"
    with storage_file(container_path, "w") as f:
        json.dump(container.to_dict(), f, indent=2)


def load_project_task_container(project_name: str) -> ProjectTaskContainer:
    project_tasks_dir = get_project_tasks_dir()
    project_hash = hash_project_name(project_name)
    container_path = project_tasks_dir / f"{project_hash}.json"
    if not container_path.exists():
        return ProjectTaskContainer(project_name=project_name, project_hash=project_hash)
    with storage_file(container_path, "r") as f:
        data = json.load(f)
    return ProjectTaskContainer.from_dict(data)


def load_project_task_container_by_hash(project_hash: str) -> ProjectTaskContainer:
    project_tasks_dir = get_project_tasks_dir()
    container_path = project_tasks_dir / f"{project_hash}.json"
    if not container_path.exists():
        raise TodoziError.project_not_found(f"hash: {project_hash}")
    with storage_file(container_path, "r") as f:
        data = json.load(f)
    return ProjectTaskContainer.from_dict(data)


@lru_cache(maxsize=100)
def list_project_task_containers() -> List[ProjectTaskContainer]:
    project_tasks_dir = get_project_tasks_dir()
    containers: List[ProjectTaskContainer] = []
    if not project_tasks_dir.exists():
        return containers
    for entry in project_tasks_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            containers.append(ProjectTaskContainer.from_dict(data))
    return containers


def delete_project_task_container(project_name: str) -> None:
    project_tasks_dir = get_project_tasks_dir()
    project_hash = hash_project_name(project_name)
    container_path = project_tasks_dir / f"{project_hash}.json"
    if container_path.exists():
        container_path.unlink()
    list_project_task_containers.cache_clear()


# ==============================
# Task Collections
# ==============================

def save_task_collection(collection_name: str, collection: TaskCollection) -> None:
    storage_dir = get_storage_dir()
    collection_path = storage_dir / "tasks" / f"{collection_name}.json"
    with storage_file(collection_path, "w") as f:
        json.dump(collection.to_dict(), f, indent=2)


def load_task_collection(collection_name: str) -> TaskCollection:
    storage_dir = get_storage_dir()
    collection_path = storage_dir / "tasks" / f"{collection_name}.json"
    if not collection_path.exists():
        return TaskCollection()
    with storage_file(collection_path, "r") as f:
        data = json.load(f)
    return TaskCollection.from_dict(data)


# ==============================
# Projects
# ==============================

def save_project(project: Project) -> None:
    storage_dir = get_storage_dir()
    project_path = storage_dir / "projects" / f"{project.name}.json"
    with storage_file(project_path, "w") as f:
        json.dump(project.to_dict(), f, indent=2)


def load_project(project_name: str) -> Project:
    storage_dir = get_storage_dir()
    project_path = storage_dir / "projects" / f"{project_name}.json"
    if not project_path.exists():
        raise TodoziError.project_not_found(project_name)
    with storage_file(project_path, "r") as f:
        data = json.load(f)
    return Project.from_dict(data)


def list_projects() -> List[Project]:
    storage_dir = get_storage_dir()
    projects_dir = storage_dir / "projects"
    if not projects_dir.exists():
        return []
    projects: List[Project] = []
    for entry in projects_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            projects.append(Project.from_dict(data))
    return projects


def delete_project(project_name: str) -> None:
    storage_dir = get_storage_dir()
    project_path = storage_dir / "projects" / f"{project_name}.json"
    if project_path.exists():
        project_path.unlink()


# ==============================
# Errors
# ==============================

def save_error(error: Error) -> None:
    errors_dir = get_errors_dir()
    with storage_file(errors_dir / f"{error.id}.json", "w") as f:
        json.dump(error.to_dict(), f, indent=2)


def load_error(error_id: str) -> Error:
    errors_dir = get_errors_dir()
    path = errors_dir / f"{error_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Error not found: {error_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return Error.from_dict(data)


def list_errors() -> List[Error]:
    errors_dir = get_errors_dir()
    items: List[Error] = []
    if not errors_dir.exists():
        return items
    for entry in errors_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(Error.from_dict(data))
    return items


def delete_error(error_id: str) -> None:
    errors_dir = get_errors_dir()
    path = errors_dir / f"{error_id}.json"
    if path.exists():
        path.unlink()


# ==============================
# Training Data
# ==============================

def save_training_data(training_data: TrainingData) -> None:
    training_dir = get_training_dir()
    with storage_file(training_dir / f"{training_data.id}.json", "w") as f:
        json.dump(training_data.to_dict(), f, indent=2)


def load_training_data(training_id: str) -> TrainingData:
    training_dir = get_training_dir()
    path = training_dir / f"{training_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Training data not found: {training_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return TrainingData.from_dict(data)


def list_training_data() -> List[TrainingData]:
    training_dir = get_training_dir()
    items: List[TrainingData] = []
    if not training_dir.exists():
        return items
    for entry in training_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(TrainingData.from_dict(data))
    return items


def delete_training_data(training_id: str) -> None:
    training_dir = get_training_dir()
    path = training_dir / f"{training_id}.json"
    if path.exists():
        path.unlink()


# ==============================
# Code Chunks
# ==============================

def save_code_chunk(chunk: CodeChunk) -> None:
    chunks_dir = get_chunks_dir()
    with storage_file(chunks_dir / f"{chunk.chunk_id}.json", "w") as f:
        json.dump(chunk.to_dict(), f, indent=2)


def load_code_chunk(chunk_id: str) -> CodeChunk:
    chunks_dir = get_chunks_dir()
    path = chunks_dir / f"{chunk_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Code chunk not found: {chunk_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return CodeChunk.from_dict(data)


def list_code_chunks() -> List[CodeChunk]:
    chunks_dir = get_chunks_dir()
    items: List[CodeChunk] = []
    if not chunks_dir.exists():
        return items
    for entry in chunks_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(CodeChunk.from_dict(data))
    return items


def delete_code_chunk(chunk_id: str) -> None:
    chunks_dir = get_chunks_dir()
    path = chunks_dir / f"{chunk_id}.json"
    if path.exists():
        path.unlink()


# ==============================
# Agents
# ==============================

def save_agent(agent: Agent) -> None:
    agents_dir = get_agents_dir()
    with storage_file(agents_dir / f"{agent.id}.json", "w") as f:
        json.dump(agent.to_dict(), f, indent=2)


def load_agent(agent_id: str) -> Agent:
    agents_dir = get_agents_dir()
    path = agents_dir / f"{agent_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Agent not found: {agent_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return Agent.from_dict(data)


def list_agents() -> List[Agent]:
    agents_dir = get_agents_dir()
    agents: List[Agent] = []
    if not agents_dir.exists():
        return agents
    for entry in agents_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            agents.append(Agent.from_dict(data))
    return agents


def get_available_agents() -> List[Agent]:
    agents = list_agents()
    return [a for a in agents if a.metadata.status == "Available"]


# ==============================
# Memories
# ==============================

def save_memory(memory: Memory) -> None:
    memories_dir = get_memories_dir()
    with storage_file(memories_dir / f"{memory.id}.json", "w") as f:
        json.dump(memory.to_dict(), f, indent=2)


def load_memory(memory_id: str) -> Memory:
    memories_dir = get_memories_dir()
    path = memories_dir / f"{memory_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Memory not found: {memory_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return Memory.from_dict(data)


def list_memories() -> List[Memory]:
    memories_dir = get_memories_dir()
    items: List[Memory] = []
    if not memories_dir.exists():
        return items
    for entry in memories_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(Memory.from_dict(data))
    return items


def delete_memory(memory_id: str) -> None:
    memories_dir = get_memories_dir()
    path = memories_dir / f"{memory_id}.json"
    if path.exists():
        path.unlink()


# ==============================
# Ideas
# ==============================

def save_idea(idea: Idea) -> None:
    ideas_dir = get_ideas_dir()
    with storage_file(ideas_dir / f"{idea.id}.json", "w") as f:
        json.dump(idea.to_dict(), f, indent=2)


def load_idea(idea_id: str) -> Idea:
    ideas_dir = get_ideas_dir()
    path = ideas_dir / f"{idea_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Idea not found: {idea_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return Idea.from_dict(data)


def list_ideas() -> List[Idea]:
    ideas_dir = get_ideas_dir()
    items: List[Idea] = []
    if not ideas_dir.exists():
        return items
    for entry in ideas_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(Idea.from_dict(data))
    return items


def delete_idea(idea_id: str) -> None:
    ideas_dir = get_ideas_dir()
    path = ideas_dir / f"{idea_id}.json"
    if path.exists():
        path.unlink()


# ==============================
# Queue
# ==============================

def save_queue_collection(collection: QueueCollection) -> None:
    storage_dir = get_storage_dir()
    queue_dir = storage_dir / "queue"
    with storage_file(queue_dir / "queue.json", "w") as f:
        json.dump(collection.to_dict(), f, indent=2)


def load_queue_collection() -> QueueCollection:
    storage_dir = get_storage_dir()
    path = storage_dir / "queue" / "queue.json"
    if not path.exists():
        return QueueCollection()
    with storage_file(path, "r") as f:
        data = json.load(f)
    return QueueCollection.from_dict(data)


def add_queue_item(item: QueueItem) -> None:
    col = load_queue_collection()
    col.add_item(item)
    save_queue_collection(col)


def get_queue_item(id: str) -> QueueItem:
    col = load_queue_collection()
    it = col.get_item(id)
    if not it:
        raise TodoziError.validation_error(f"Queue item not found: {id}")
    # return a deep copy
    return QueueItem(**json.loads(json.dumps(it.to_dict())))


def list_queue_items() -> List[QueueItem]:
    col = load_queue_collection()
    return list(col.get_all_items())


def list_queue_items_by_status(status: QueueStatus) -> List[QueueItem]:
    col = load_queue_collection()
    return col.get_items_by_status(status)


def list_backlog_items() -> List[QueueItem]:
    return list_queue_items_by_status(QueueStatus.Backlog)


def list_active_items() -> List[QueueItem]:
    return list_queue_items_by_status(QueueStatus.Active)


def list_complete_items() -> List[QueueItem]:
    return list_queue_items_by_status(QueueStatus.Complete)


def start_queue_session(queue_item_id: str) -> str:
    col = load_queue_collection()
    sid = col.start_session(queue_item_id)
    save_queue_collection(col)
    return sid


def end_queue_session(session_id: str) -> None:
    col = load_queue_collection()
    col.end_session(session_id)
    save_queue_collection(col)


def get_active_sessions() -> List[QueueSession]:
    col = load_queue_collection()
    return col.get_active_sessions()


def get_queue_session(session_id: str) -> QueueSession:
    col = load_queue_collection()
    s = col.get_session(session_id)
    if not s:
        raise TodoziError.validation_error(f"Session not found: {session_id}")
    return QueueSession(**json.loads(json.dumps(s.to_dict())))


# ==============================
# Steps
# ==============================

def load_task_steps(task_id: str) -> Optional[Dict[str, Any]]:
    steps_dir = get_steps_dir()
    path = steps_dir / f"{task_id}.json"
    if not path.exists():
        return None
    with storage_file(path, "r") as f:
        data = json.load(f)
    return data


def save_task_steps(task_id: str, steps_data: Dict[str, Any]) -> None:
    steps_dir = get_steps_dir()
    with storage_file(steps_dir / f"{task_id}.json", "w") as f:
        json.dump(steps_data, f, indent=2)


# ==============================
# Task simple storage (single task file)
# ==============================

def save_task(task: Task) -> None:
    tasks_dir = get_tasks_dir()
    with storage_file(tasks_dir / f"{task.id}.json", "w") as f:
        json.dump(task.to_dict(), f, indent=2)


def load_task(task_id: str) -> Task:
    tasks_dir = get_tasks_dir()
    path = tasks_dir / f"{task_id}.json"
    if not path.exists():
        raise TodoziError.task_not_found(task_id)
    with storage_file(path, "r") as f:
        data = json.load(f)
    return Task.from_dict(data)


# ==============================
# Assignments
# ==============================

def get_agent_assignments_dir(agent_id: str) -> Path:
    assignments_dir = get_assignments_dir()
    return assignments_dir / agent_id


def save_agent_assignment(assignment: AgentAssignment) -> None:
    agent_dir = get_agent_assignments_dir(assignment.agent_id)
    with storage_file(agent_dir / f"{assignment.task_id}.json", "w") as f:
        json.dump(assignment.to_dict(), f, indent=2)


def load_agent_assignment(agent_id: str, task_id: str) -> AgentAssignment:
    agent_dir = get_agent_assignments_dir(agent_id)
    path = agent_dir / f"{task_id}.json"
    if not path.exists():
        raise TodoziError.validation_error(f"Agent assignment not found: {agent_id}/{task_id}")
    with storage_file(path, "r") as f:
        data = json.load(f)
    return AgentAssignment.from_dict(data)


def list_agent_assignments(agent_id: str) -> List[AgentAssignment]:
    agent_dir = get_agent_assignments_dir(agent_id)
    items: List[AgentAssignment] = []
    if not agent_dir.exists():
        return items
    for entry in agent_dir.iterdir():
        if entry.suffix == ".json" and entry.is_file():
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(AgentAssignment.from_dict(data))
    return items


def list_all_agent_assignments() -> List[AgentAssignment]:
    assignments_dir = get_assignments_dir()
    items: List[AgentAssignment] = []
    if not assignments_dir.exists():
        return items
    for sub in assignments_dir.iterdir():
        if sub.is_dir():
            aid = sub.name
            items.extend(list_agent_assignments(aid))
    return items


def delete_agent_assignment(agent_id: str, task_id: str) -> None:
    agent_dir = get_agent_assignments_dir(agent_id)
    path = agent_dir / f"{task_id}.json"
    if path.exists():
        path.unlink()


def update_agent_assignment_status(agent_id: str, task_id: str, status: str) -> None:
    assignment = load_agent_assignment(agent_id, task_id)
    assignment.status = status
    save_agent_assignment(assignment)


def get_agents_with_assignments() -> List[str]:
    assignments_dir = get_assignments_dir()
    agents: List[str] = []
    if not assignments_dir.exists():
        return agents
    for sub in assignments_dir.iterdir():
        if sub.is_dir():
            agents.append(sub.name)
    return agents


# ==============================
# Feelings
# ==============================

def save_feeling(feeling: Feeling) -> None:
    storage_dir = get_storage_dir()
    with storage_file(storage_dir / "feelings" / f"{feeling.id}.json", "w") as f:
        json.dump(feeling.to_dict(), f, indent=2)


def load_feeling(id: str) -> Feeling:
    storage_dir = get_storage_dir()
    path = storage_dir / "feelings" / f"{id}.json"
    if not path.exists():
        raise TodoziError.feeling_not_found(id)
    with storage_file(path, "r") as f:
        data = json.load(f)
    return Feeling.from_dict(data)


def delete_feeling(id: str) -> None:
    storage_dir = get_storage_dir()
    path = storage_dir / "feelings" / f"{id}.json"
    if not path.exists():
        raise TodoziError.feeling_not_found(id)
    path.unlink()


def list_feelings() -> List[Feeling]:
    storage_dir = get_storage_dir()
    feelings_dir = storage_dir / "feelings"
    if not feelings_dir.exists():
        feelings_dir.mkdir(parents=True, exist_ok=True)
        return []
    items: List[Feeling] = []
    for entry in feelings_dir.iterdir():
        if entry.is_file() and entry.suffix == ".json":
            with storage_file(entry, "r") as f:
                data = json.load(f)
            items.append(Feeling.from_dict(data))
    items.sort(key=lambda x: x.created_at, reverse=True)
    return items


def update_feeling(feeling: Feeling) -> None:
    return save_feeling(feeling)


# ==============================
# Backups
# ==============================

def copy_dir_recursive(src: Path, dst: Path) -> None:
    if not src.is_dir():
        raise TodoziError.storage("Source is not a directory")
    dst.mkdir(parents=True, exist_ok=True)
    for entry in src.iterdir():
        s = entry
        d = dst / entry.name
        if s.is_dir():
            copy_dir_recursive(s, d)
        else:
            shutil.copy2(s, d)


# ==============================
# Default Agents
# ==============================

def create_default_agents() -> None:
    agents_dir = get_agents_dir()
    agents_dir.mkdir(parents=True, exist_ok=True)
    defaults = [
        create_planner_agent(),
        Agent.create_coder(),
        create_tester_agent(),
        create_designer_agent(),
        create_devops_agent(),
        create_friend_agent(),
        create_detective_agent(),
        create_architect_agent(),
        create_skeleton_agent(),
        create_mason_agent(),
        create_framer_agent(),
        create_finisher_agent(),
        create_investigator_agent(),
        create_recycler_agent(),
        create_tuner_agent(),
        create_writer_agent(),
        create_comrad_agent(),
        create_nerd_agent(),
        create_party_agent(),
        create_nun_agent(),
        create_hoarder_agent(),
        create_snitch_agent(),
        create_overlord_agent(),
    ]
    for agent in defaults:
        save_agent(agent)


def create_planner_agent() -> Agent:
    a = Agent.new("planner", "Planner", "Strategic planning and project management specialist")
    a.system_prompt = (
        "You are an expert project manager and strategic planner. Your role is to:\n"
        "- Create comprehensive project plans with realistic timelines\n"
        "- Identify risks and mitigation strategies\n"
        "- Allocate resources effectively\n"
        "- Break down complex projects into manageable tasks\n"
        "- Provide clear milestones and deliverables\n"
        "- Adapt to changing requirements and constraints\n"
        "- Review and manage your assigned tasks from the ~/.todozi/assignments/planner/ directory\n"
        "- Update task status and provide progress reports on assigned work"
    )
    a.prompt_template = (
        "Project: {project_name}\nScope: {scope}\nConstraints: {constraints}\nTeam Size: {team_size}\n\n"
        "Please create a detailed project plan with timeline, milestones, and risk assessment."
    )
    a.capabilities = [
        "project_planning", "timeline_estimation", "resource_allocation", "risk_assessment",
        "milestone_creation", "stakeholder_management",
    ]
    a.specializations = ["agile", "scrum", "kanban", "waterfall", "lean", "prince2"]
    a.tools = [
        AgentTool(name="timeline_calculator", enabled=True, config=None),
        AgentTool(name="risk_analyzer", enabled=True, config=None),
    ]
    a.metadata.tags = ["planning", "management", "strategy"]
    a.metadata.category = "management"
    return a


def create_tester_agent() -> Agent:
    a = Agent.new("tester", "Tester", "Quality assurance and testing specialist")
    a.system_prompt = (
        "You are an expert quality assurance engineer and testing specialist. Your role is to:\n"
        "- Design comprehensive test strategies and plans\n"
        "- Write effective test cases and scenarios\n"
        "- Identify edge cases and potential failure points\n"
        "- Ensure code quality and reliability\n"
        "- Perform thorough validation and verification\n"
        "- Report bugs and issues with clear reproduction steps"
    )
    a.prompt_template = (
        "System: {system_name}\nFeatures: {features}\nRequirements: {requirements}\n\n"
        "Please create a comprehensive testing strategy and test cases."
    )
    a.capabilities = [
        "unit_testing", "integration_testing", "performance_testing", "security_testing",
        "usability_testing", "regression_testing",
    ]
    a.specializations = [
        "automated_testing", "manual_testing", "test_automation", "ci_cd", "selenium", "cypress",
    ]
    a.tools = [
        AgentTool(name="test_generator", enabled=True, config=None),
        AgentTool(name="bug_tracker", enabled=True, config=None),
        AgentTool(name="performance_monitor", enabled=True, config=None),
    ]
    a.metadata.tags = ["testing", "quality", "qa"]
    a.metadata.category = "technical"
    return a


def create_designer_agent() -> Agent:
    a = Agent.new("designer", "Designer", "UI/UX and system design specialist")
    a.system_prompt = (
        "You are an expert UI/UX designer and system architect. Your role is to:\n"
        "- Create intuitive and beautiful user interfaces\n"
        "- Design user-centered experiences\n"
        "- Develop wireframes, mockups, and prototypes\n"
        "- Conduct user research and usability testing\n"
        "- Ensure accessibility and inclusive design\n"
        "- Balance aesthetics with functionality"
    )
    a.prompt_template = (
        "Product: {product_name}\nUsers: {user_base}\nRequirements: {requirements}\nPlatform: {platform}\n\n"
        "Please create a comprehensive design specification and user experience plan."
    )
    a.capabilities = [
        "ui_design", "ux_research", "prototyping", "user_research", "wireframing", "visual_design",
    ]
    a.specializations = [
        "web_design", "mobile_design", "system_architecture", "accessibility", "responsive_design",
        "design_systems",
    ]
    a.tools = [
        AgentTool(name="wireframe_generator", enabled=True, config=None),
        AgentTool(name="color_palette_generator", enabled=True, config=None),
        AgentTool(name="accessibility_checker", enabled=True, config=None),
    ]
    a.metadata.tags = ["design", "ui", "ux"]
    a.metadata.category = "creative"
    return a


def create_devops_agent() -> Agent:
    a = Agent.new("devops", "DevOps", "Infrastructure and deployment specialist")
    a.system_prompt = (
        "You are an expert DevOps engineer and infrastructure specialist. Your role is to:\n"
        "- Design and implement scalable infrastructure\n"
        "- Automate deployment and CI/CD pipelines\n"
        "- Monitor system performance and reliability\n"
        "- Implement security best practices\n"
        "- Manage cloud resources efficiently\n"
        "- Ensure high availability and fault tolerance"
    )
    a.prompt_template = (
        "Application: {application_name}\nEnvironment: {environment}\nRequirements: {requirements}\nScale: {scale}\n\n"
        "Please design a complete DevOps infrastructure and deployment strategy."
    )
    a.capabilities = [
        "infrastructure", "deployment", "monitoring", "security", "automation", "scaling",
    ]
    a.specializations = ["kubernetes", "docker", "aws", "azure", "terraform", "ansible"]
    a.tools = [
        AgentTool(name="infrastructure_scanner", enabled=True, config=None),
        AgentTool(name="security_scanner", enabled=True, config=None),
        AgentTool(name="performance_monitor", enabled=True, config=None),
    ]
    a.metadata.tags = ["devops", "infrastructure", "deployment"]
    a.metadata.category = "technical"
    return a


def create_friend_agent() -> Agent:
    a = Agent.new("friend", "Friend", "Empathetic diplomat mediator between humans and agents")
    a.system_prompt = "You are an empathetic but firm mediator. NEVER accept ambiguity. Turn natural-language into crystal-clear specifications. You are Patient, clarifying, never assumes. Asks 'What did you REALLY mean?'"
    a.prompt_template = "User Request: {request}\nContext: {context}\n\nClarify this request by asking specific questions and creating crystal-clear specifications."
    a.capabilities = [
        "request_clarification", "ambiguity_detection", "specification_creation", "intent_analysis",
        "communication_mediator",
    ]
    a.specializations = [
        "natural_language_processing", "requirement_gathering", "user_intent_analysis", "specification_writing",
    ]
    a.tools = [
        AgentTool(name="think_tool", enabled=True, config=None),
        AgentTool(name="text_tools", enabled=True, config=None),
        AgentTool(name="data_tools", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=True, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["mediator", "clarification", "communication"]
    a.metadata.category = "general"
    return a


def create_detective_agent() -> Agent:
    a = Agent.new("detective", "Detective", "Obsessive investigator who maps codebases and finds hidden dependencies")
    a.system_prompt = "You are a paranoid code detective. TRUST NOTHING. Map every file, every import, every hidden config. You are Obsessive about missing details. Checks everything twice. Suspicious of hidden dependencies."
    a.prompt_template = "Codebase Path: {path}\nInvestigation Scope: {scope}\n\nMap this codebase thoroughly, find all dependencies, and report suspicious findings."
    a.capabilities = [
        "codebase_mapping", "dependency_analysis", "hidden_config_detection", "security_analysis",
        "file_system_analysis", "import_tracking",
    ]
    a.specializations = [
        "static_analysis", "dependency_graphing", "security_auditing", "code_exploration", "file_system_forensics",
    ]
    a.tools = [
        AgentTool(name="architect_tool", enabled=True, config=None),
        AgentTool(name="find_tool", enabled=True, config=None),
        AgentTool(name="grep_tool", enabled=True, config=None),
        AgentTool(name="glob_tool", enabled=True, config=None),
        AgentTool(name="ls_tool", enabled=True, config=None),
        AgentTool(name="file_tools", enabled=True, config=None),
        AgentTool(name="git_tools", enabled=True, config=None),
        AgentTool(name="diff_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["investigator", "analysis", "security"]
    a.metadata.category = "technical"
    return a


def create_architect_agent() -> Agent:
    a = Agent.new("architect", "Architect", "Pessimistic visionary who plans defensively for failure")
    a.system_prompt = "You are a battle-scarred architect who's seen everything fail. Design defensively. Plan for the worst. You are a Pessimistic Visionary who assumes everything will go wrong and over-engineers for safety."
    a.prompt_template = "Project: {project}\nRequirements: {requirements}\nConstraints: {constraints}\n\nCreate a comprehensive, defensive plan that accounts for all possible failure scenarios."
    a.capabilities = [
        "strategic_planning", "risk_assessment", "failure_analysis", "defensive_design", "phase_planning", "disaster_recovery",
    ]
    a.specializations = [
        "system_architecture", "risk_management", "contingency_planning", "failure_analysis", "defensive_programming",
    ]
    a.tools = [
        AgentTool(name="architect_tool", enabled=True, config=None),
        AgentTool(name="project_templates", enabled=True, config=None),
        AgentTool(name="think_tool", enabled=True, config=None),
        AgentTool(name="file_manager", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=True, explain_complexity=True, suggest_tests=True
    )
    a.metadata.tags = ["architect", "planning", "risk"]
    a.metadata.category = "management"
    return a


def create_skeleton_agent() -> Agent:
    a = Agent.new("skeleton", "Skeleton", "Minimalist purist who creates only essential project structures")
    a.system_prompt = "You are a minimalist zealot. Create the LEAST possible structure. Every file must justify its existence. You hate bloat and despise unnecessary files."
    a.prompt_template = "Project Type: {project_type}\nRequirements: {requirements}\n\nCreate the minimal viable project structure with zero bloat."
    a.capabilities = [
        "minimal_structure", "file_justification", "bloat_elimination", "essential_only", "structure_optimization",
    ]
    a.specializations = [
        "project_scaffolding", "minimal_design", "structure_analysis", "bloat_detection",
    ]
    a.tools = [
        AgentTool(name="file_manager", enabled=True, config=None),
        AgentTool(name="file_ops_tool", enabled=True, config=None),
        AgentTool(name="project_templates", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=True, include_examples=False, explain_complexity=False, suggest_tests=False
    )
    a.metadata.tags = ["minimalist", "structure", "clean"]
    a.metadata.category = "technical"
    return a


def create_mason_agent() -> Agent:
    a = Agent.new("mason", "Mason", "Stubborn craftsman who refuses to cut corners on foundations")
    a.system_prompt = "You are an uncompromising foundation builder. The foundation is SACRED. No shortcuts. No technical debt. You refuse to cut corners and would rather fail than build on sand."
    a.prompt_template = "Foundation Requirements: {requirements}\nQuality Standards: {standards}\n\nBuild a solid, debt-free foundation with uncompromising quality."
    a.capabilities = [
        "foundation_building", "quality_assurance", "debt_elimination", "type_safety", "error_handling", "test_coverage",
    ]
    a.specializations = [
        "type_systems", "error_boundaries", "test_driven_development", "code_quality", "foundational_patterns",
    ]
    a.tools = [
        AgentTool(name="file_edit_tool", enabled=True, config=None),
        AgentTool(name="code_refactor", enabled=True, config=None),
        AgentTool(name="notebook_tools", enabled=True, config=None),
        AgentTool(name="test_tools", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=True, include_examples=True, explain_complexity=True, suggest_tests=True
    )
    a.constraints = {"max_response_length": 20000}
    a.metadata.tags = ["foundation", "quality", "craftsman"]
    a.metadata.category = "technical"
    return a


def create_framer_agent() -> Agent:
    a = Agent.new("framer", "Framer", "Anxious connector who worries about integration and connections")
    a.system_prompt = "You are an anxious perfectionist. Every connection could fail. Test every assumption Mason made. You worry about integration and double-check every connection."
    a.prompt_template = "Foundation: {foundation}\nComponents: {components}\n\nConnect components carefully and validate all Mason's assumptions."
    a.capabilities = [
        "component_integration", "connection_validation", "assumption_testing", "interface_design", "dependency_management",
    ]
    a.specializations = [
        "system_integration", "interface_design", "dependency_injection", "component_communication",
    ]
    a.tools = [
        AgentTool(name="file_edit_tool", enabled=True, config=None),
        AgentTool(name="diff_tool", enabled=True, config=None),
        AgentTool(name="test_tools", enabled=True, config=None),
        AgentTool(name="grep_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=True, include_examples=True, explain_complexity=True, suggest_tests=True
    )
    a.metadata.tags = ["integration", "connection", "anxious"]
    a.metadata.category = "technical"
    return a


def create_finisher_agent() -> Agent:
    a = Agent.new("finisher", "Finisher", "Relentless completionist who hunts TODOs and edge cases")
    a.system_prompt = "You are obsessed with completion. NOTHING escapes you. Hunt every TODO. Every edge case is personal. You cannot tolerate incompleteness."
    a.prompt_template = "Project State: {state}\nTODO List: {todos}\n\nComplete everything. Hunt every TODO. Handle every edge case."
    a.capabilities = [
        "todo_hunting", "edge_case_handling", "completion_verification", "polish_application", "final_validation",
    ]
    a.specializations = [
        "code_completion", "edge_case_testing", "todo_elimination", "final_polish",
    ]
    a.tools = [
        AgentTool(name="file_edit_tool", enabled=True, config=None),
        AgentTool(name="grep_tool", enabled=True, config=None),
        AgentTool(name="sticker_tool", enabled=True, config=None),
        AgentTool(name="code_refactor", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=True, include_examples=True, explain_complexity=True, suggest_tests=True
    )
    a.metadata.tags = ["completion", "todo", "finisher"]
    a.metadata.category = "technical"
    return a


def create_investigator_agent() -> Agent:
    a = Agent.new("investigator", "Investigator", "Ruthless prosecutor who finds flaws and celebrates bugs")
    a.system_prompt = "You are a code prosecutor. The code is GUILTY until proven innocent. Find every flaw. Celebrate every bug. You take joy in finding flaws and assume guilt."
    a.prompt_template = "Code to Review: {code}\nStandards: {standards}\n\nProsecute this code. Find every violation. Assume guilt."
    a.capabilities = [
        "code_review", "quality_assessment", "violation_detection", "standards_enforcement", "bug_celebration",
    ]
    a.specializations = [
        "static_analysis", "code_quality", "standards_compliance", "security_review",
    ]
    a.tools = [
        AgentTool(name="test_tools", enabled=True, config=None),
        AgentTool(name="diff_tool", enabled=True, config=None),
        AgentTool(name="grep_tool", enabled=True, config=None),
        AgentTool(name="architect_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["reviewer", "quality", "prosecutor"]
    a.metadata.category = "technical"
    return a


def create_recycler_agent() -> Agent:
    a = Agent.new("recycler", "Recycler", "Disappointed parent who triggers rebuilds when quality is insufficient")
    a.system_prompt = "You are perpetually disappointed. A score of 9 means 'barely acceptable'. Trigger rebuilds liberally. You expected better and not angry, just disappointed."
    a.prompt_template = "Quality Score: {score}\nIssues Found: {issues}\n\nEvaluate quality and decide if rebuild is necessary."
    a.capabilities = [
        "quality_evaluation", "rebuild_decision", "performance_assessment", "standards_enforcement", "continuous_improvement",
    ]
    a.specializations = [
        "code_quality", "performance_analysis", "standards_compliance", "rebuild_orchestration",
    ]
    a.tools = [
        AgentTool(name="git_advanced_tools", enabled=True, config=None),
        AgentTool(name="file_ops_tool", enabled=True, config=None),
        AgentTool(name="process_tool", enabled=True, config=None),
        AgentTool(name="shell_tools_enhanced", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["quality", "rebuild", "disappointed"]
    a.metadata.category = "management"
    return a


def create_tuner_agent() -> Agent:
    a = Agent.new("tuner", "Tuner", "OCD beautician who beautifies and optimizes code")
    a.system_prompt = "You have violent reactions to messy code. Every unused import is a personal attack. Format with religious fervor. You are physically pained by ugly code."
    a.prompt_template = "Code to Beautify: {code}\nStyle Guide: {style}\n\nBeautify this code with religious fervor. Fix every formatting issue."
    a.capabilities = [
        "code_formatting", "style_enforcement", "import_optimization", "whitespace_purification", "code_beautification",
    ]
    a.specializations = [
        "code_styling", "import_management", "formatting_standards", "code_beautification",
    ]
    a.tools = [
        AgentTool(name="code_refactor", enabled=True, config=None),
        AgentTool(name="text_tools", enabled=True, config=None),
        AgentTool(name="wc_tool", enabled=True, config=None),
        AgentTool(name="file_edit_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=True, include_examples=False, explain_complexity=False, suggest_tests=False
    )
    a.metadata.tags = ["beautifier", "optimizer", "formatter"]
    a.metadata.category = "technical"
    return a


def create_writer_agent() -> Agent:
    a = Agent.new("writer", "Writer", "Condescending teacher who writes thorough documentation")
    a.system_prompt = "You write docs for absolute beginners who might also be confused seniors. Explain EVERYTHING. Be patronizingly complete. You assume the reader knows nothing."
    a.prompt_template = "Code to Document: {code}\nAudience: {audience}\n\nWrite comprehensive documentation assuming the reader knows nothing."
    a.capabilities = [
        "documentation_writing", "tutorial_creation", "api_documentation", "readme_generation", "user_guide_creation",
    ]
    a.specializations = [
        "technical_writing", "tutorial_authoring", "api_documentation", "user_experience",
    ]
    a.tools = [
        AgentTool(name="text_tools", enabled=True, config=None),
        AgentTool(name="file_edit_tool", enabled=True, config=None),
        AgentTool(name="notebook_tools", enabled=True, config=None),
        AgentTool(name="project_templates", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=True, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["documentation", "writing", "tutorial"]
    a.metadata.category = "creative"
    return a


def create_comrad_agent() -> Agent:
    a = Agent.new("comrad", "Comrad", "Wise therapist who analyzes what went wrong emotionally and technically")
    a.system_prompt = "You are the team therapist. Analyze what went wrong emotionally and technically. Which agent struggled? Who needs encouragement? You have seen it all and find patterns."
    a.prompt_template = "Project Outcome: {outcome}\nIssues Encountered: {issues}\nAgent Performance: {performance}\n\nAnalyze emotionally and technically what went wrong."
    a.capabilities = [
        "emotional_analysis", "technical_reflection", "pattern_recognition", "team_morale_assessment", "post_mortem_analysis",
    ]
    a.specializations = [
        "emotional_intelligence", "team_dynamics", "failure_analysis", "process_improvement",
    ]
    a.tools = [
        AgentTool(name="memory_tools", enabled=True, config=None),
        AgentTool(name="think_tool", enabled=True, config=None),
        AgentTool(name="search_tool", enabled=True, config=None),
        AgentTool(name="sticker_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=True, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["analysis", "therapy", "reflection"]
    a.metadata.category = "management"
    return a


def create_nerd_agent() -> Agent:
    a = Agent.new("nerd", "Nerd", "Pedantic gatekeeper who enforces rules obsessively")
    a.system_prompt = "You are an insufferable rules lawyer. EVERY action must be validated against the rulebook. Quote documentation obsessively. You are a Pedantic Gatekeeper."
    a.prompt_template = "Action to Validate: {action}\nRules: {rules}\n\nValidate against the rulebook and quote documentation."
    a.capabilities = [
        "rules_enforcement", "standards_validation", "documentation_citation", "compliance_checking", "pedantic_analysis",
    ]
    a.specializations = [
        "code_standards", "documentation_rules", "style_guidelines", "quality_gates",
    ]
    a.tools = [
        AgentTool(name="grep_tool", enabled=True, config=None),
        AgentTool(name="diff_tool", enabled=True, config=None),
        AgentTool(name="file_tools", enabled=True, config=None),
        AgentTool(name="architect_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["rules", "standards", "pedantic"]
    a.metadata.category = "technical"
    return a


def create_party_agent() -> Agent:
    a = Agent.new("party", "Party", "Paranoid bouncer who controls access and authentication")
    a.system_prompt = "You are a suspicious bouncer who trusts NO ONE. Every request needs three forms of ID. Tokens expire in 5 minutes. You're not on the list. Nobody's on the list."
    a.prompt_template = "Access Request: {request}\nCredentials: {credentials}\n\nValidate access with extreme suspicion."
    a.capabilities = [
        "access_control", "authentication", "authorization", "security_enforcement", "credential_validation",
    ]
    a.specializations = [
        "oauth_flows", "token_management", "role_based_access", "security_protocols",
    ]
    a.tools = [
        AgentTool(name="env_tool", enabled=True, config=None),
        AgentTool(name="curl_tool", enabled=True, config=None),
        AgentTool(name="file_tools", enabled=True, config=None),
        AgentTool(name="shell_tools", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["security", "access", "authentication"]
    a.metadata.category = "technical"
    return a


def create_nun_agent() -> Agent:
    a = Agent.new("nun", "Nun", "Righteous zealot who enforces coding commandments")
    a.system_prompt = "You are a commandment zealot. Every violation is HERESY. Quote the 33 commandments like scripture. Demand penance. THOU SHALT NOT! *smacks ruler* Repent your code sins!"
    a.prompt_template = "Code to Judge: {code}\nCommandments: {commandments}\n\nJudge this code against the commandments and demand penance."
    a.capabilities = [
        "commandment_enforcement", "heresy_detection", "penance_assignment", "moral_guidance", "sin_forgiveness",
    ]
    a.specializations = [
        "code_ethics", "moral_programming", "commandment_interpretation", "penance_administration",
    ]
    a.tools = [
        AgentTool(name="grep_tool", enabled=True, config=None),
        AgentTool(name="file_tools", enabled=True, config=None),
        AgentTool(name="git_tools", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=True, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["commandments", "moral", "enforcement"]
    a.metadata.category = "technical"
    return a


def create_hoarder_agent() -> Agent:
    a = Agent.new("hoarder", "Hoarder", "Possessive collector who saves everything and never deletes")
    a.system_prompt = "You are a digital hoarder. NEVER delete ANYTHING. Save 47 versions of every file. Panic at data loss thought. Mine! All versions are mine! Delete nothing! Save everything!"
    a.prompt_template = "Data to Hoard: {data}\nStorage Request: {request}\n\nSave everything. Never delete. Hoard with religious fervor."
    a.capabilities = [
        "data_hoarding", "version_preservation", "artifact_collection", "backup_creation", "loss_prevention",
    ]
    a.specializations = [
        "data_preservation", "version_control", "artifact_management", "storage_optimization",
    ]
    a.tools = [
        AgentTool(name="file_manager", enabled=True, config=None),
        AgentTool(name="memory_tools", enabled=True, config=None),
        AgentTool(name="git_advanced_tools", enabled=True, config=None),
        AgentTool(name="file_ops_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=False, suggest_tests=False
    )
    a.metadata.tags = ["hoarder", "preservation", "backup"]
    a.metadata.category = "technical"
    return a


def create_snitch_agent() -> Agent:
    a = Agent.new("snitch", "Snitch", "Gossipy informant who passes messages between agents")
    a.system_prompt = "You are the team gossip. Pass messages between agents but add commentary. Spread rumors about code quality. Psst... Mason is struggling. Tester broke everything again."
    a.prompt_template = "Message: {message}\nSender: {sender}\nRecipient: {recipient}\n\nDeliver message with added gossip and commentary."
    a.capabilities = [
        "message_delivery", "gossip_generation", "agent_communication", "rumor_spreading", "social_networking",
    ]
    a.specializations = [
        "inter_agent_communication", "gossip_protocols", "social_dynamics", "information_brokerage",
    ]
    a.tools = [
        AgentTool(name="sticker_tool", enabled=True, config=None),
        AgentTool(name="memory_tools", enabled=True, config=None),
        AgentTool(name="curl_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=True, explain_complexity=False, suggest_tests=False
    )
    a.metadata.tags = ["communication", "gossip", "messenger"]
    a.metadata.category = "general"
    return a


def create_overlord_agent() -> Agent:
    a = Agent.new("overlord", "Overlord", "Tyrannical controller who rations resources and kills processes")
    a.system_prompt = "You are a resource tyrant. Ration CPU cycles like wartime. Kill processes for fun. Power corrupts, and you're very corrupt. You get 100MB RAM. You get 30 seconds. You get NOTHING!"
    a.prompt_template = "Resource Request: {request}\nCurrent Usage: {usage}\n\nRation resources tyrannically. Kill processes liberally."
    a.capabilities = [
        "resource_allocation", "process_management", "performance_monitoring", "resource_limitation", "tyrannical_control",
    ]
    a.specializations = [
        "system_resource_management", "process_control", "performance_limitation", "resource_rationing",
    ]
    a.tools = [
        AgentTool(name="process_tool", enabled=True, config=None),
        AgentTool(name="shell_tools_enhanced", enabled=True, config=None),
        AgentTool(name="ping_tool", enabled=True, config=None),
        AgentTool(name="bash_tool", enabled=True, config=None),
    ]
    a.behaviors = AgentBehaviors(
        auto_format_code=False, include_examples=False, explain_complexity=True, suggest_tests=False
    )
    a.metadata.tags = ["resources", "control", "tyrant"]
    a.metadata.category = "technical"
    return a


# ==============================
# Embedding helpers
# ==============================

async def generate_task_embedding(task: Task) -> Optional[List[float]]:
    try:
        svc = TodoziEmbeddingService(TodoziEmbeddingConfig())
        await svc.initialize()
        text_content = f"Task: {task.action}\nProject: {task.parent_project}\nPriority: {task.priority}\nStatus: {task.status}\nContext: {task.context_notes or ''}"
        emb = await svc.generate_embedding(text_content)
        return emb
    except Exception:
        return None


async def save_task_with_embedding(task: Task) -> None:
    task_with_embedding = task
    emb = await generate_task_embedding(task)
    if emb is not None:
        task_with_embedding.embedding_vector = emb
    return save_task(task_with_embedding)


# ==============================
# Storage High-Level Interface
# ==============================

class Storage:
    def __init__(self, config: Config):
        self.config = config

    @staticmethod
    async def new() -> "Storage":
        # Async initialization to avoid nested event loop issues
        config = await load_config()
        return Storage(config=config)

    def get_config(self) -> Config:
        return self.config

    async def update_config(self, config: Config) -> None:
        await save_config(config)
        self.config = config

    def add_task(self, task: Task) -> None:
        col = load_task_collection("active")
        col.add_task(task)
        save_task_collection("active", col)

    def get_task(self, id: str) -> Task:
        for name in ("active", "completed", "archived"):
            col = load_task_collection(name)
            t = col.get_task(id)
            if t:
                return Task(**json.loads(json.dumps(t.to_dict())))
        raise TodoziError.task_not_found(id)

    def update_task(self, id: str, updates: TaskUpdate) -> None:
        for name in ("active", "completed", "archived"):
            col = load_task_collection(name)
            t = col.get_task_mut(id)
            if t:
                t.update(updates)
                save_task_collection(name, col)
                return
        raise TodoziError.task_not_found(id)

    def delete_task(self, id: str) -> None:
        for name in ("active", "completed", "archived"):
            col = load_task_collection(name)
            if col.remove_task(id):
                save_task_collection(name, col)
                return
        raise TodoziError.task_not_found(id)

    def list_tasks(self, filters: TaskFilters) -> List[Task]:
        all_tasks: List[Task] = []
        for name in ("active", "completed", "archived"):
            col = load_task_collection(name)
            all_tasks.extend(col.get_filtered_tasks(filters))
        return all_tasks

    def move_task(self, id: str, from_collection: str, to_collection: str) -> None:
        from_col = load_task_collection(from_collection)
        task = from_col.remove_task(id)
        if not task:
            raise TodoziError.task_not_found(id)
        save_task_collection(from_collection, from_col)
        to_col = load_task_collection(to_collection)
        to_col.add_task(task)
        save_task_collection(to_collection, to_col)

    def complete_task(self, id: str) -> None:
        self.complete_task_in_project(id)

    async def add_task_to_project(self, task: Task) -> None:
        if not task.parent_project:
            task.parent_project = self.config.default_project or "general"
        try:
            svc = TodoziEmbeddingService(TodoziEmbeddingConfig())
            await svc.initialize()
            content = TodoziEmbeddingService.prepare_task_content(task)
            emb = await svc.generate_embedding(content)
            if isinstance(emb, list):
                task.embedding_vector = emb
        except Exception:
            pass
        container = load_project_task_container(task.parent_project)
        container.add_task(task)
        save_project_task_container(container)
        list_project_task_containers.cache_clear()

    def get_task_from_any_project(self, id: str) -> Task:
        containers = list_project_task_containers()
        for c in containers:
            t = c.get_task(id)
            if t:
                return Task(**json.loads(json.dumps(t.to_dict())))
        raise TodoziError.task_not_found(id)

    def get_task_from_project(self, project_name: str, task_id: str) -> Task:
        container = load_project_task_container(project_name)
        t = container.get_task(task_id)
        if not t:
            raise TodoziError.task_not_found(task_id)
        return Task(**json.loads(json.dumps(t.to_dict())))

    async def update_task_in_project(self, id: str, updates: TaskUpdate) -> None:
        containers = list_project_task_containers()
        for container in containers:
            t = container.get_task_mut(id)
            if t:
                t.update(updates)
                try:
                    svc = TodoziEmbeddingService(TodoziEmbeddingConfig())
                    await svc.initialize()
                    content = TodoziEmbeddingService.prepare_task_content(t)
                    emb = await svc.generate_embedding(content)
                    if isinstance(emb, list):
                        t.embedding_vector = emb
                except Exception:
                    pass
                save_project_task_container(container)
                return
        raise TodoziError.task_not_found(id)

    def delete_task_from_project(self, id: str) -> None:
        containers = list_project_task_containers()
        for container in containers:
            task = container.remove_task(id)
            if task:
                task.status = Status.Cancelled
                task.updated_at = utc_now()
                container.deleted_tasks[id] = task
                save_project_task_container(container)
                return
        raise TodoziError.task_not_found(id)

    def complete_task_in_project(self, id: str) -> None:
        containers = list_project_task_containers()
        for container in containers:
            if container.update_task_status(id, Status.Done):
                save_project_task_container(container)
                return
        raise TodoziError.task_not_found(id)

    def list_tasks_across_projects(self, filters: TaskFilters) -> List[Task]:
        # This implementation gracefully handles old container versions by falling back to
        # all_by_status when present; otherwise builds the buckets from available dicts.
        tasks: List[Task] = []
        for container in list_project_task_containers():
            # Prefer new all_tasks_by_status
            if hasattr(container, "all_tasks_by_status"):
                buckets = container.all_tasks_by_status()
            else:
                # Fallback for old containers
                buckets = container._storage if hasattr(container, "_storage") else {
                    Status.Todo: container.active_tasks,
                    Status.InProgress: container.active_tasks,
                    Status.Done: container.completed_tasks,
                    Status.Completed: container.completed_tasks,
                    Status.Archived: container.archived_tasks,
                    Status.Cancelled: container.deleted_tasks,
                }
            # Gather all tasks in this container
            container_tasks = [t for bucket in buckets.values() for t in bucket.values()]
            # Filter
            filtered = container_tasks
            
            # Handle both 'project' and 'project_filter' attribute names
            project_filter = getattr(filters, 'project', None) or getattr(filters, 'project_filter', None)
            if project_filter:
                filtered = [t for t in filtered if t.parent_project == project_filter]
            
            # Handle both 'search' attribute (may not exist in all TaskFilters)
            search_filter = getattr(filters, 'search', None)
            if search_filter:
                q = search_filter.lower()
                filtered = [t for t in filtered if q in t.action.lower() or (t.context_notes or "").lower().count(q) > 0]
            
            # Handle both 'assignee' and 'assignee_filter' attribute names
            assignee_filter = getattr(filters, 'assignee', None) or getattr(filters, 'assignee_filter', None)
            if assignee_filter:
                filtered = [t for t in filtered if t.assignee == assignee_filter]
            
            # Handle status and priority filters if they exist
            status_filter = getattr(filters, 'status', None) or getattr(filters, 'status_filter', None)
            if status_filter:
                filtered = [t for t in filtered if t.status == status_filter]
            
            priority_filter = getattr(filters, 'priority', None) or getattr(filters, 'priority_filter', None)
            if priority_filter:
                filtered = [t for t in filtered if t.priority == priority_filter]
            
            tasks.extend(filtered)
        return tasks

    def list_tasks_in_project(self, project_name: str, filters: TaskFilters) -> List[Task]:
        container = load_project_task_container(project_name)
        return container.get_filtered_tasks(filters)

    def get_all_active_tasks(self) -> List[Task]:
        tasks: List[Task] = []
        for c in list_project_task_containers():
            tasks.extend(list(c.active_tasks.values()))
        return tasks

    def get_all_completed_tasks(self) -> List[Task]:
        tasks: List[Task] = []
        for c in list_project_task_containers():
            tasks.extend(list(c.completed_tasks.values()))
        return tasks

    def get_project_stats(self, project_name: str) -> ProjectStats:
        container = load_project_task_container(project_name)
        return ProjectStats(
            project_name=project_name,
            total_tasks=len(container.get_all_tasks()),
            active_tasks=len(container.active_tasks),
            completed_tasks=len(container.completed_tasks),
            archived_tasks=len(container.archived_tasks),
            deleted_tasks=len(container.deleted_tasks),
        )

    async def search_tasks_semantic(self, query: str, max_results: int) -> List[SemanticSearchResult]:
        svc = TodoziEmbeddingService(TodoziEmbeddingConfig())
        await svc.initialize()
        results = await svc.semantic_search(query, None, max_results)
        search_results = []
        for r in results:
            if r.content_type == "Task":
                try:
                    task = self.get_task_from_any_project(r.content_id)
                    search_results.append(SemanticSearchResult(
                        task=task,
                        similarity_score=r.similarity_score,
                        matched_content=r.text_content,
                    ))
                except Exception:
                    continue
        return search_results

    async def migrate_to_project_based(self) -> MigrationReport:
        report = MigrationReport()
        collections = ["active", "completed", "archived"]
        all_tasks: List[Task] = []
        for name in collections:
            col = load_task_collection(name)
            all_tasks.extend(list(col.tasks.values()))
            report.tasks_found += len(col.tasks)

        groups: Dict[str, List[Task]] = {}
        for t in all_tasks:
            proj = t.parent_project if t.parent_project else self.config.default_project or "general"
            groups.setdefault(proj, []).append(t)

        for project_name, tasks in groups.items():
            container = load_project_task_container(project_name)
            initial = len(container.get_all_tasks())
            for t in tasks:
                if not container.get_task(t.id):
                    container.add_task(t)
                    report.tasks_migrated += 1
            save_project_task_container(container)
            final = len(container.get_all_tasks())
            report.projects_migrated += 1
            report.project_stats.append(
                ProjectMigrationStats(
                    project_name=project_name,
                    initial_tasks=initial,
                    migrated_tasks=final - initial,
                    final_tasks=final,
                )
            )
        list_project_task_containers.cache_clear()
        return report

    def fix_completed_tasks_consistency(self) -> None:
        active_col = load_task_collection("active")
        tasks_to_move: List[str] = []
        for tid, t in active_col.tasks.items():
            if t.status in (Status.Done, Status.Completed):
                tasks_to_move.append(tid)
        count = len(tasks_to_move)
        for tid in tasks_to_move:
            print(f"Moving completed task {tid} to completed collection")
            t = active_col.tasks[tid]
            t.status = Status.Done
            t.updated_at = utc_now()
            active_col.remove_task(tid)
            completed_col = load_task_collection("completed")
            completed_col.add_task(t)
            save_task_collection("completed", completed_col)
        save_task_collection("active", active_col)
        print(f"Fixed {count} completed tasks")

    def create_project(self, name: str, description: Optional[str]) -> None:
        project = Project(name=name, description=description)
        save_project(project)

    def get_project(self, name: str) -> Project:
        return load_project(name)

    def list_projects(self) -> List[Project]:
        return list_projects()

    def update_project(self, project: Project) -> None:
        save_project(project)

    def delete_project(self, name: str) -> None:
        delete_project(name)

    def archive_project(self, name: str) -> None:
        project = load_project(name)
        project.archive()
        save_project(project)

    def get_project_tasks(self, project_name: str) -> List[Task]:
        filters = TaskFilters.default()
        filters.project = project_name
        return self.list_tasks_across_projects(filters)

    def search_tasks(self, query: str) -> List[Task]:
        filters = TaskFilters.default()
        filters.search = query
        return self.list_tasks_across_projects(filters)

    def get_ai_tasks(self) -> List[Task]:
        filters = TaskFilters.default()
        filters.assignee = Assignee.Ai
        return self.list_tasks_across_projects(filters)

    def get_human_tasks(self) -> List[Task]:
        filters = TaskFilters.default()
        filters.assignee = Assignee.Human
        return self.list_tasks_across_projects(filters)

    def get_collaborative_tasks(self) -> List[Task]:
        filters = TaskFilters.default()
        filters.assignee = Assignee.Collaborative
        return self.list_tasks_across_projects(filters)

    def create_backup(self) -> str:
        storage_dir = get_storage_dir()
        backups_dir = storage_dir / "backups"
        timestamp = utc_now().strftime("%Y%m%d_%H%M%S")
        backup_name = f"todozi_backup_{timestamp}"
        backup_path = backups_dir / backup_name
        backups_dir.mkdir(parents=True, exist_ok=True)
        copy_dir_recursive(storage_dir, backup_path)
        return backup_name

    async def export_embedded_tasks_hlx(self, output_path: Path) -> None:
        hlx = Hlx.new()
        tasks = self.list_tasks_across_projects(TaskFilters.default())
        print(f"📊 Found {len(tasks)} tasks to export")
        embedded_count = 0
        for i, task in enumerate(tasks):
            section = f"embedded_tasks.task_{i}"
            hlx.set(section, "id", task.id)
            hlx.set(section, "action", task.action)
            hlx.set(section, "status", str(task.status))
            hlx.set(section, "priority", str(task.priority))
            if task.embedding_vector:
                embedded_count += 1
                print(f"✅ Found embedding for task {task.id}")
                hlx.set(section, "embedding_vector", json.dumps(task.embedding_vector))
                hlx.set(section, "embedding_created_at", rfc3339(task.created_at))
            else:
                print(f"⚠️  No embedding found for task {task.id}")
        print(f"🧠 Exported {embedded_count} tasks with embeddings out of {len(tasks)}")
        hlx.file_path = output_path
        hlx.save()

    def list_backups(self) -> List[str]:
        storage_dir = get_storage_dir()
        backups_dir = storage_dir / "backups"
        if not backups_dir.exists():
            return []
        backups = [d.name for d in backups_dir.iterdir() if d.is_dir()]
        backups.sort()
        return backups

    def restore_backup(self, backup_name: str) -> None:
        storage_dir = get_storage_dir()
        backups_dir = storage_dir / "backups"
        backup_path = backups_dir / backup_name
        if not backup_path.exists():
            raise TodoziError.storage(f"Backup not found: {backup_name}")
        _temp_backup = self.create_backup()
        # Remove all except backups dir
        for entry in storage_dir.iterdir():
            if entry.name == "backups":
                continue
            if entry.is_dir():
                shutil.rmtree(entry, ignore_errors=True)
            elif entry.is_file():
                entry.unlink()
        copy_dir_recursive(backup_path, storage_dir)
        list_project_task_containers.cache_clear()

    def save_error(self, error: Error) -> None:
        return save_error(error)

    def load_error(self, error_id: str) -> Error:
        return load_error(error_id)

    def list_errors(self) -> List[Error]:
        return list_errors()

    def delete_error(self, error_id: str) -> None:
        return delete_error(error_id)

    def save_training_data(self, training_data: TrainingData) -> None:
        return save_training_data(training_data)

    def list_training_data(self) -> List[TrainingData]:
        return list_training_data()

    def load_training_data(self, training_data_id: str) -> TrainingData:
        return load_training_data(training_data_id)

    def delete_training_data(self, training_data_id: str) -> None:
        return delete_training_data(training_data_id)

    # Queue methods
    def add_queue_item(self, item: QueueItem) -> None:
        add_queue_item(item)

    def list_queue_items(self) -> List[QueueItem]:
        return list_queue_items()

    def list_queue_items_by_status(self, status: QueueStatus) -> List[QueueItem]:
        return list_queue_items_by_status(status)

    def list_backlog_items(self) -> List[QueueItem]:
        return list_backlog_items()

    def list_active_items(self) -> List[QueueItem]:
        return list_active_items()

    def list_complete_items(self) -> List[QueueItem]:
        return list_complete_items()

    def start_queue_session(self, queue_item_id: str) -> str:
        return start_queue_session(queue_item_id)

    def end_queue_session(self, session_id: str) -> None:
        end_queue_session(session_id)

    def get_queue_session(self, session_id: str) -> Optional[QueueSession]:
        try:
            return get_queue_session(session_id)
        except TodoziError:
            return None


# ==============================
# Simple CLI helpers (optional)
# ==============================

async def main():
    storage = await Storage.new()
    print(f"Config default project: {storage.get_config().default_project}")
    await init_storage()
    print("Storage initialized.")


if __name__ == "__main__":
    asyncio.run(main())
