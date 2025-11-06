"""Data models and enums for Todozi.

This module contains all the core data structures used throughout the Todozi system.
"""

from dataclasses import dataclass, field, asdict
from datetime import datetime
from enum import Enum
from typing import Optional, List, Dict, Any
import uuid as uuid_lib
import hashlib
import random
import json


# ============================================================================
# Enums
# ============================================================================

class Priority(str, Enum):
    """Task priority levels."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"
    URGENT = "urgent"


class Status(str, Enum):
    """Task status."""
    TODO = "todo"
    PENDING = "pending"
    IN_PROGRESS = "in_progress"
    BLOCKED = "blocked"
    REVIEW = "review"
    DONE = "done"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    DEFERRED = "deferred"


class Assignee(str, Enum):
    """Task assignee types."""
    AI = "ai"
    HUMAN = "human"
    COLLABORATIVE = "collaborative"

    @staticmethod
    def agent(name: str) -> str:
        """Create an agent assignee."""
        return f"agent:{name}"


class MemoryImportance(str, Enum):
    """Memory importance levels."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class MemoryTerm(str, Enum):
    """Memory term lengths."""
    SHORT = "short"
    LONG = "long"


class MemoryType(str, Enum):
    """Memory types."""
    STANDARD = "standard"
    SECRET = "secret"
    HUMAN = "human"
    SHORT = "short"
    LONG = "long"

    @staticmethod
    def emotional(emotion: str) -> str:
        """Create an emotional memory type."""
        return f"emotional:{emotion}"


class CoreEmotion(str, Enum):
    """Core emotions."""
    HAPPY = "happy"
    SAD = "sad"
    ANGRY = "angry"
    FEARFUL = "fearful"
    SURPRISED = "surprised"
    DISGUSTED = "disgusted"
    EXCITED = "excited"
    ANXIOUS = "anxious"
    CONFIDENT = "confident"
    FRUSTRATED = "frustrated"
    MOTIVATED = "motivated"
    OVERWHELMED = "overwhelmed"
    CURIOUS = "curious"
    SATISFIED = "satisfied"
    DISAPPOINTED = "disappointed"
    GRATEFUL = "grateful"
    PROUD = "proud"
    ASHAMED = "ashamed"
    HOPEFUL = "hopeful"
    RESIGNED = "resigned"


class ShareLevel(str, Enum):
    """Sharing levels for ideas."""
    PRIVATE = "private"
    TEAM = "team"
    PUBLIC = "public"


class IdeaImportance(str, Enum):
    """Idea importance levels."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    BREAKTHROUGH = "breakthrough"


class ItemStatus(str, Enum):
    """Item status for memories and ideas."""
    ACTIVE = "active"
    ARCHIVED = "archived"
    DELETED = "deleted"


class ErrorSeverity(str, Enum):
    """Error severity levels."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class ErrorCategory(str, Enum):
    """Error categories."""
    NETWORK = "network"
    DATABASE = "database"
    AUTHENTICATION = "authentication"
    AUTHORIZATION = "authorization"
    VALIDATION = "validation"
    PERFORMANCE = "performance"
    SECURITY = "security"
    INTEGRATION = "integration"
    CONFIGURATION = "configuration"
    RUNTIME = "runtime"
    COMPILATION = "compilation"
    DEPENDENCY = "dependency"
    USER_ERROR = "user_error"
    SYSTEM_ERROR = "system_error"


class TrainingDataType(str, Enum):
    """Training data types."""
    INSTRUCTION = "instruction"
    COMPLETION = "completion"
    CONVERSATION = "conversation"
    CODE = "code"
    ANALYSIS = "analysis"
    PLANNING = "planning"
    REVIEW = "review"
    DOCUMENTATION = "documentation"
    EXAMPLE = "example"
    TEST = "test"
    VALIDATION = "validation"


class ProjectStatus(str, Enum):
    """Project status."""
    ACTIVE = "active"
    ARCHIVED = "archived"
    COMPLETED = "completed"


class AgentStatus(str, Enum):
    """Agent status."""
    ACTIVE = "active"
    INACTIVE = "inactive"
    BUSY = "busy"
    AVAILABLE = "available"


class AssignmentStatus(str, Enum):
    """Assignment status."""
    ASSIGNED = "assigned"
    ACCEPTED = "accepted"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    REJECTED = "rejected"


class QueueStatus(str, Enum):
    """Queue item status."""
    BACKLOG = "backlog"
    ACTIVE = "active"
    COMPLETE = "complete"


class ReminderPriority(str, Enum):
    """Reminder priority levels."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class ReminderStatus(str, Enum):
    """Reminder status."""
    PENDING = "pending"
    ACTIVE = "active"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    OVERDUE = "overdue"


class SummaryPriority(str, Enum):
    """Summary priority levels."""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


# ============================================================================
# Task Models
# ============================================================================

@dataclass
class Task:
    """Main task structure."""
    id: str
    user_id: str
    action: str
    time: str
    priority: Priority
    parent_project: str
    status: Status
    assignee: Optional[str] = None
    tags: List[str] = field(default_factory=list)
    dependencies: List[str] = field(default_factory=list)
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    @staticmethod
    def new(user_id: str, action: str, time: str, priority: Priority,
            parent_project: str, status: Status) -> 'Task':
        """Create a new task."""
        task_id = f"task_{str(uuid_lib.uuid4())[:8]}"
        now = datetime.utcnow()
        return Task(
            id=task_id,
            user_id=user_id,
            action=action,
            time=time,
            priority=priority,
            parent_project=parent_project,
            status=status,
            created_at=now,
            updated_at=now
        )

    def complete(self):
        """Mark the task as completed."""
        self.status = Status.DONE
        self.progress = 100
        self.updated_at = datetime.utcnow()

    def is_completed(self) -> bool:
        """Check if the task is completed."""
        return self.status == Status.DONE

    def is_active(self) -> bool:
        """Check if the task is active."""
        return self.status not in [Status.DONE, Status.CANCELLED]

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        return asdict(self)


@dataclass
class TaskUpdate:
    """Task update structure."""
    action: Optional[str] = None
    time: Optional[str] = None
    priority: Optional[Priority] = None
    parent_project: Optional[str] = None
    status: Optional[Status] = None
    assignee: Optional[str] = None
    tags: Optional[List[str]] = None
    dependencies: Optional[List[str]] = None
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None


@dataclass
class TaskFilters:
    """Task filtering options."""
    project: Optional[str] = None
    status: Optional[Status] = None
    priority: Optional[Priority] = None
    assignee: Optional[str] = None
    tags: Optional[List[str]] = None
    search: Optional[str] = None


@dataclass
class TaskCollection:
    """Collection of tasks."""
    version: str = "1.2.0"
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())
    tasks: Dict[str, Task] = field(default_factory=dict)

    def add_task(self, task: Task):
        """Add a task to the collection."""
        self.tasks[task.id] = task
        self.updated_at = datetime.utcnow()

    def get_task(self, task_id: str) -> Optional[Task]:
        """Get a task by ID."""
        return self.tasks.get(task_id)

    def remove_task(self, task_id: str) -> Optional[Task]:
        """Remove a task by ID."""
        task = self.tasks.pop(task_id, None)
        if task:
            self.updated_at = datetime.utcnow()
        return task

    def get_all_tasks(self) -> List[Task]:
        """Get all tasks."""
        return list(self.tasks.values())


# ============================================================================
# Project Models
# ============================================================================

@dataclass
class Project:
    """Project structure."""
    name: str
    description: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())
    status: ProjectStatus = ProjectStatus.ACTIVE
    tasks: List[str] = field(default_factory=list)

    def add_task(self, task_id: str):
        """Add a task to the project."""
        if task_id not in self.tasks:
            self.tasks.append(task_id)
            self.updated_at = datetime.utcnow()

    def remove_task(self, task_id: str):
        """Remove a task from the project."""
        if task_id in self.tasks:
            self.tasks.remove(task_id)
            self.updated_at = datetime.utcnow()

    def archive(self):
        """Archive the project."""
        self.status = ProjectStatus.ARCHIVED
        self.updated_at = datetime.utcnow()

    def complete(self):
        """Complete the project."""
        self.status = ProjectStatus.COMPLETED
        self.updated_at = datetime.utcnow()


# ============================================================================
# Configuration Models
# ============================================================================

@dataclass
class RegistrationInfo:
    """Registration information."""
    user_name: str
    user_email: str
    api_key: str
    server_url: str
    user_id: Optional[str] = None
    fingerprint: Optional[str] = None
    registered_at: datetime = field(default_factory=lambda: datetime.utcnow())

    @staticmethod
    def new_with_hashes(server_url: str) -> 'RegistrationInfo':
        """Create registration info with generated hashes."""
        user_id = f"user_{str(uuid_lib.uuid4())[:8]}"
        email_hash = f"hash_{str(uuid_lib.uuid4())[:8]}@example.com"
        return RegistrationInfo(
            user_name=user_id,
            user_email=email_hash,
            api_key="",
            server_url=server_url
        )


@dataclass
class Config:
    """Configuration structure."""
    version: str = "1.2.0"
    default_project: str = "general"
    auto_backup: bool = True
    backup_interval: str = "daily"
    ai_enabled: bool = True
    default_assignee: Optional[str] = "collaborative"
    date_format: str = "%Y-%m-%d %H:%M:%S"
    timezone: str = "UTC"
    registration: Optional[RegistrationInfo] = None


# ============================================================================
# Agent Models
# ============================================================================

@dataclass
class ModelConfig:
    """AI model configuration."""
    provider: str = "anthropic"
    name: str = "claude-3-opus-20240229"
    temperature: float = 0.2
    max_tokens: int = 4096


@dataclass
class AgentTool:
    """Agent tool configuration."""
    name: str
    enabled: bool = True
    config: Optional[Dict[str, Any]] = None


@dataclass
class AgentBehaviors:
    """Agent behaviors."""
    auto_format_code: bool = True
    include_examples: bool = True
    explain_complexity: bool = True
    suggest_tests: bool = True


@dataclass
class RateLimit:
    """Rate limiting configuration."""
    requests_per_minute: Optional[int] = 10
    tokens_per_hour: Optional[int] = 100000


@dataclass
class AgentConstraints:
    """Agent constraints."""
    max_response_length: Optional[int] = 10000
    timeout_seconds: Optional[int] = 300
    rate_limit: Optional[RateLimit] = field(default_factory=RateLimit)


@dataclass
class AgentMetadata:
    """Agent metadata."""
    author: str = "system"
    tags: List[str] = field(default_factory=lambda: ["ai", "assistant"])
    category: str = "general"
    status: AgentStatus = AgentStatus.AVAILABLE


@dataclass
class Agent:
    """AI agent structure."""
    id: str
    name: str
    description: str
    version: str = "1.0.0"
    model: ModelConfig = field(default_factory=ModelConfig)
    system_prompt: str = ""
    prompt_template: Optional[str] = None
    capabilities: List[str] = field(default_factory=list)
    specializations: List[str] = field(default_factory=list)
    tools: List[AgentTool] = field(default_factory=list)
    behaviors: AgentBehaviors = field(default_factory=AgentBehaviors)
    constraints: AgentConstraints = field(default_factory=AgentConstraints)
    metadata: AgentMetadata = field(default_factory=AgentMetadata)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    @staticmethod
    def new(agent_id: str, name: str, description: str) -> 'Agent':
        """Create a new agent."""
        return Agent(
            id=agent_id,
            name=name,
            description=description,
            system_prompt=f"You are {agent_id}, an AI assistant specialized in {description}."
        )

    def has_capability(self, capability: str) -> bool:
        """Check if agent has a capability."""
        return capability in self.capabilities

    def has_specialization(self, specialization: str) -> bool:
        """Check if agent has a specialization."""
        return specialization in self.specializations

    def is_available(self) -> bool:
        """Check if agent is available."""
        return self.metadata.status == AgentStatus.AVAILABLE


@dataclass
class AgentAssignment:
    """Agent assignment to a task."""
    agent_id: str
    task_id: str
    project_id: str
    assigned_at: datetime = field(default_factory=lambda: datetime.utcnow())
    status: AssignmentStatus = AssignmentStatus.ASSIGNED


# ============================================================================
# Memory and Idea Models
# ============================================================================

@dataclass
class Memory:
    """Memory structure."""
    id: str
    user_id: str
    moment: str
    meaning: str
    reason: str
    importance: MemoryImportance = MemoryImportance.MEDIUM
    term: MemoryTerm = MemoryTerm.SHORT
    memory_type: MemoryType = MemoryType.STANDARD
    project_id: Optional[str] = None
    status: ItemStatus = ItemStatus.ACTIVE
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())


@dataclass
class Idea:
    """Idea structure."""
    id: str
    idea: str
    project_id: Optional[str] = None
    status: ItemStatus = ItemStatus.ACTIVE
    share: ShareLevel = ShareLevel.PRIVATE
    importance: IdeaImportance = IdeaImportance.MEDIUM
    tags: List[str] = field(default_factory=list)
    context: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())


# ============================================================================
# Error and Training Models
# ============================================================================

@dataclass
class Error:
    """Error structure."""
    id: str
    title: str
    description: str
    severity: ErrorSeverity
    category: ErrorCategory
    source: str
    context: Optional[str] = None
    tags: List[str] = field(default_factory=list)
    resolved: bool = False
    resolution: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())
    resolved_at: Optional[datetime] = None


@dataclass
class TrainingData:
    """Training data structure."""
    id: str
    data_type: TrainingDataType
    prompt: str
    completion: str
    source: str
    context: Optional[str] = None
    tags: List[str] = field(default_factory=list)
    quality_score: Optional[float] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())


# ============================================================================
# Queue Models
# ============================================================================

@dataclass
class QueueItem:
    """Queue item structure."""
    id: str
    task_name: str
    task_description: str
    priority: Priority
    status: QueueStatus = QueueStatus.BACKLOG
    project_id: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    @staticmethod
    def new(task_name: str, task_description: str, priority: Priority,
            project_id: Optional[str] = None) -> 'QueueItem':
        """Create a new queue item."""
        item_id = f"queue_{str(uuid_lib.uuid4())[:8]}"
        return QueueItem(
            id=item_id,
            task_name=task_name,
            task_description=task_description,
            priority=priority,
            project_id=project_id
        )

    def start(self):
        """Mark the item as active."""
        self.status = QueueStatus.ACTIVE
        self.updated_at = datetime.utcnow()

    def complete(self):
        """Mark the item as complete."""
        self.status = QueueStatus.COMPLETE
        self.updated_at = datetime.utcnow()


@dataclass
class QueueSession:
    """Queue session for tracking active work."""
    id: str
    queue_item_id: str
    start_time: datetime = field(default_factory=lambda: datetime.utcnow())
    end_time: Optional[datetime] = None
    duration_seconds: Optional[int] = None
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    def end(self):
        """End the session."""
        self.end_time = datetime.utcnow()
        self.duration_seconds = int((self.end_time - self.start_time).total_seconds())
        self.updated_at = self.end_time


@dataclass
class QueueCollection:
    """Collection of queue items."""
    version: str = "1.0.0"
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())
    items: Dict[str, QueueItem] = field(default_factory=dict)
    sessions: Dict[str, QueueSession] = field(default_factory=dict)

    def add_item(self, item: QueueItem):
        """Add a queue item."""
        self.items[item.id] = item
        self.updated_at = datetime.utcnow()

    def get_backlog_items(self) -> List[QueueItem]:
        """Get backlog items."""
        return [i for i in self.items.values() if i.status == QueueStatus.BACKLOG]

    def get_active_items(self) -> List[QueueItem]:
        """Get active items."""
        return [i for i in self.items.values() if i.status == QueueStatus.ACTIVE]


# ============================================================================
# API Key Models
# ============================================================================

@dataclass
class ApiKey:
    """API key structure."""
    user_id: str
    public_key: str
    private_key: str
    active: bool = True
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    @staticmethod
    def new() -> 'ApiKey':
        """Create a new API key with generated hashes."""
        user_id = f"user_{str(uuid_lib.uuid4())[:8]}"
        time_str = str(int(datetime.utcnow().timestamp()))
        rand_str = str(random.randint(100000, 999999))
        input_data = f"{time_str}{rand_str}"
        public_key = hashlib.sha256(input_data.encode()).hexdigest()
        private_key = hashlib.sha512(public_key.encode()).hexdigest()
        return ApiKey(
            user_id=user_id,
            public_key=public_key,
            private_key=private_key
        )

    def is_active(self) -> bool:
        """Check if the API key is active."""
        return self.active


@dataclass
class ApiKeyCollection:
    """Collection of API keys."""
    version: str = "1.0.0"
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())
    keys: Dict[str, ApiKey] = field(default_factory=dict)

    def add_key(self, key: ApiKey):
        """Add an API key."""
        self.keys[key.user_id] = key
        self.updated_at = datetime.utcnow()

    def get_key(self, user_id: str) -> Optional[ApiKey]:
        """Get an API key by user ID."""
        return self.keys.get(user_id)


# ============================================================================
# Other Models
# ============================================================================

@dataclass
class Tag:
    """Tag structure."""
    id: str
    name: str
    description: Optional[str] = None
    color: Optional[str] = None
    category: Optional[str] = None
    usage_count: int = 0
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())


@dataclass
class Feeling:
    """Feeling structure."""
    id: str
    emotion: str
    intensity: int
    description: str
    context: str
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())


@dataclass
class Summary:
    """Summary structure."""
    id: str
    content: str
    priority: SummaryPriority
    context: Optional[str] = None
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())


@dataclass
class Reminder:
    """Reminder structure."""
    id: str
    content: str
    remind_at: datetime
    priority: ReminderPriority
    status: ReminderStatus = ReminderStatus.PENDING
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    def is_overdue(self) -> bool:
        """Check if reminder is overdue."""
        return self.remind_at < datetime.utcnow() and self.status == ReminderStatus.PENDING


# ============================================================================
# Statistics and Reports
# ============================================================================

@dataclass
class ProjectStats:
    """Project statistics."""
    project_name: str
    total_tasks: int
    active_tasks: int
    completed_tasks: int
    archived_tasks: int
    deleted_tasks: int


@dataclass
class SemanticSearchResult:
    """Semantic search result."""
    task: Task
    similarity_score: float
    matched_content: str


@dataclass
class MigrationReport:
    """Migration report."""
    tasks_found: int = 0
    tasks_migrated: int = 0
    projects_migrated: int = 0
    project_stats: List['ProjectMigrationStats'] = field(default_factory=list)
    errors: List[str] = field(default_factory=list)


@dataclass
class ProjectMigrationStats:
    """Project migration statistics."""
    project_name: str
    initial_tasks: int
    migrated_tasks: int
    final_tasks: int


@dataclass
class ProjectTaskContainer:
    """Container for project tasks."""
    project_name: str
    project_hash: str
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())
    active_tasks: Dict[str, Task] = field(default_factory=dict)
    completed_tasks: Dict[str, Task] = field(default_factory=dict)
    archived_tasks: Dict[str, Task] = field(default_factory=dict)
    deleted_tasks: Dict[str, Task] = field(default_factory=dict)

    @staticmethod
    def new(project_name: str) -> 'ProjectTaskContainer':
        """Create a new project task container."""
        project_hash = hashlib.md5(project_name.encode()).hexdigest()
        return ProjectTaskContainer(
            project_name=project_name,
            project_hash=project_hash
        )

    def add_task(self, task: Task):
        """Add a task to the appropriate container."""
        if task.status in [Status.TODO, Status.PENDING, Status.IN_PROGRESS, Status.BLOCKED, Status.REVIEW]:
            self.active_tasks[task.id] = task
        elif task.status in [Status.DONE, Status.COMPLETED]:
            self.completed_tasks[task.id] = task
        elif task.status == Status.CANCELLED:
            self.archived_tasks[task.id] = task
        elif task.status == Status.DEFERRED:
            self.archived_tasks[task.id] = task
        self.updated_at = datetime.utcnow()

    def get_task(self, task_id: str) -> Optional[Task]:
        """Get a task by ID."""
        return (self.active_tasks.get(task_id) or
                self.completed_tasks.get(task_id) or
                self.archived_tasks.get(task_id) or
                self.deleted_tasks.get(task_id))

    def get_all_tasks(self) -> List[Task]:
        """Get all tasks."""
        all_tasks = []
        all_tasks.extend(self.active_tasks.values())
        all_tasks.extend(self.completed_tasks.values())
        all_tasks.extend(self.archived_tasks.values())
        all_tasks.extend(self.deleted_tasks.values())
        return all_tasks


def hash_project_name(project_name: str) -> str:
    """Hash a project name using MD5."""
    return hashlib.md5(project_name.encode()).hexdigest()
