"""Storage operations for Todozi.

This module handles all file-based storage operations including reading and writing
tasks, projects, agents, memories, ideas, and configuration.
"""

import json
import os
from pathlib import Path
from typing import Optional, List, Dict, Any
from datetime import datetime

from .models import (
    Task, TaskCollection, Project, Agent, Memory, Idea, Error, TrainingData,
    Feeling, Tag, Config, RegistrationInfo, QueueCollection, QueueItem,
    ApiKeyCollection, AgentAssignment, ProjectTaskContainer
)
from .error import StorageError, ValidationError


def get_storage_dir() -> Path:
    """Get the Todozi storage directory."""
    home = Path.home()
    return home / ".todozi"


def get_tasks_dir() -> Path:
    """Get the tasks directory."""
    return get_storage_dir() / "tasks"


async def init_storage():
    """Initialize the storage directory structure."""
    storage_dir = get_storage_dir()

    # Create all required directories
    directories = [
        "tasks", "projects", "templates", "backups", "agents", "memories",
        "ideas", "training", "chunks", "errors", "assignments", "feelings",
        "queue", "api", "models", "responses", "embed"
    ]

    for dir_name in directories:
        (storage_dir / dir_name).mkdir(parents=True, exist_ok=True)

    # Initialize config if it doesn't exist
    config_path = storage_dir / "tdz.hlx"
    if not config_path.exists():
        config = Config()
        await save_config(config)

    # Create default registration info
    registration = RegistrationInfo.new_with_hashes("https://todozi.com")
    await update_config_with_registration(registration)

    # Create default agents
    create_default_agents()

    # Create default project
    project_path = storage_dir / "projects" / "general.json"
    if not project_path.exists():
        project = Project(name="general", description="General tasks")
        save_project(project)

    # Create default task collections
    for collection_name in ["active", "completed", "archived"]:
        path = storage_dir / "tasks" / f"{collection_name}.json"
        if not path.exists():
            collection = TaskCollection()
            save_task_collection(collection_name, collection)


def check_folder_structure() -> bool:
    """Check if the folder structure is complete."""
    storage_dir = get_storage_dir()

    required_dirs = [
        "agents", "api", "assignments", "backups", "chunks", "embed",
        "errors", "feelings", "ideas", "memories", "models", "projects",
        "queue", "responses", "tasks", "templates", "training"
    ]

    for dir_name in required_dirs:
        dir_path = storage_dir / dir_name
        if not dir_path.exists():
            print(f"❌ Missing directory: {dir_name}")
            return False
        if not dir_path.is_dir():
            print(f"❌ {dir_name} exists but is not a directory")
            return False

    config_path = storage_dir / "tdz.hlx"
    if not config_path.exists():
        print("❌ Missing tdz.hlx configuration file")
        return False

    print("✅ Todozi folder structure is complete!")
    print(f"📁 Storage directory: {storage_dir}")
    print(f"📂 Found {len(required_dirs)} required directories")
    for dir_name in required_dirs:
        print(f"  ✓ {dir_name}")
    print("  ✓ tdz.hlx")
    return True


async def ensure_folder_structure() -> bool:
    """Ensure the folder structure exists."""
    if check_folder_structure():
        return True

    print("🔧 Creating missing folder structure...")
    await init_storage()
    return check_folder_structure()


# ============================================================================
# Config Operations
# ============================================================================

async def save_config(config: Config):
    """Save configuration to file."""
    storage_dir = get_storage_dir()
    config_path = storage_dir / "config.json"

    # Convert config to dict
    config_dict = {
        "version": config.version,
        "default_project": config.default_project,
        "auto_backup": config.auto_backup,
        "backup_interval": config.backup_interval,
        "ai_enabled": config.ai_enabled,
        "default_assignee": config.default_assignee,
        "date_format": config.date_format,
        "timezone": config.timezone,
    }

    if config.registration:
        config_dict["registration"] = {
            "user_name": config.registration.user_name,
            "user_email": config.registration.user_email,
            "api_key": config.registration.api_key,
            "user_id": config.registration.user_id,
            "fingerprint": config.registration.fingerprint,
            "registered_at": config.registration.registered_at.isoformat(),
            "server_url": config.registration.server_url,
        }

    with open(config_path, 'w') as f:
        json.dump(config_dict, f, indent=2)


async def load_config() -> Config:
    """Load configuration from file."""
    storage_dir = get_storage_dir()
    config_path = storage_dir / "config.json"

    if not config_path.exists():
        return Config()

    with open(config_path, 'r') as f:
        data = json.load(f)

    registration = None
    if "registration" in data:
        reg_data = data["registration"]
        registration = RegistrationInfo(
            user_name=reg_data.get("user_name", ""),
            user_email=reg_data.get("user_email", ""),
            api_key=reg_data.get("api_key", ""),
            server_url=reg_data.get("server_url", "https://todozi.com"),
            user_id=reg_data.get("user_id"),
            fingerprint=reg_data.get("fingerprint"),
            registered_at=datetime.fromisoformat(reg_data.get("registered_at",
                                                              datetime.utcnow().isoformat()))
        )

    return Config(
        version=data.get("version", "1.2.0"),
        default_project=data.get("default_project", "general"),
        auto_backup=data.get("auto_backup", True),
        backup_interval=data.get("backup_interval", "daily"),
        ai_enabled=data.get("ai_enabled", True),
        default_assignee=data.get("default_assignee", "collaborative"),
        date_format=data.get("date_format", "%Y-%m-%d %H:%M:%S"),
        timezone=data.get("timezone", "UTC"),
        registration=registration
    )


async def update_config_with_registration(registration: RegistrationInfo):
    """Update config with registration information."""
    config = await load_config()
    config.registration = registration
    await save_config(config)


async def is_registered() -> bool:
    """Check if the user is registered."""
    config = await load_config()
    return config.registration is not None and config.registration.api_key != ""


# ============================================================================
# Task Operations
# ============================================================================

def save_task_collection(name: str, collection: TaskCollection):
    """Save a task collection to file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "tasks" / f"{name}.json"

    # Convert to dict
    tasks_dict = {}
    for task_id, task in collection.tasks.items():
        tasks_dict[task_id] = {
            "id": task.id,
            "user_id": task.user_id,
            "action": task.action,
            "time": task.time,
            "priority": task.priority.value,
            "parent_project": task.parent_project,
            "status": task.status.value,
            "assignee": task.assignee,
            "tags": task.tags,
            "dependencies": task.dependencies,
            "context_notes": task.context_notes,
            "progress": task.progress,
            "embedding_vector": task.embedding_vector,
            "created_at": task.created_at.isoformat(),
            "updated_at": task.updated_at.isoformat(),
        }

    data = {
        "version": collection.version,
        "created_at": collection.created_at.isoformat(),
        "updated_at": collection.updated_at.isoformat(),
        "tasks": tasks_dict
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def load_task_collection(name: str) -> TaskCollection:
    """Load a task collection from file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "tasks" / f"{name}.json"

    if not file_path.exists():
        return TaskCollection()

    with open(file_path, 'r') as f:
        data = json.load(f)

    collection = TaskCollection(
        version=data.get("version", "1.2.0"),
        created_at=datetime.fromisoformat(data.get("created_at",
                                                   datetime.utcnow().isoformat())),
        updated_at=datetime.fromisoformat(data.get("updated_at",
                                                   datetime.utcnow().isoformat()))
    )

    for task_data in data.get("tasks", {}).values():
        from .models import Priority, Status
        task = Task(
            id=task_data["id"],
            user_id=task_data["user_id"],
            action=task_data["action"],
            time=task_data["time"],
            priority=Priority(task_data["priority"]),
            parent_project=task_data["parent_project"],
            status=Status(task_data["status"]),
            assignee=task_data.get("assignee"),
            tags=task_data.get("tags", []),
            dependencies=task_data.get("dependencies", []),
            context_notes=task_data.get("context_notes"),
            progress=task_data.get("progress"),
            embedding_vector=task_data.get("embedding_vector"),
            created_at=datetime.fromisoformat(task_data["created_at"]),
            updated_at=datetime.fromisoformat(task_data["updated_at"])
        )
        collection.add_task(task)

    return collection


# ============================================================================
# Project Operations
# ============================================================================

def save_project(project: Project):
    """Save a project to file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "projects" / f"{project.name}.json"

    from .models import ProjectStatus
    data = {
        "name": project.name,
        "description": project.description,
        "created_at": project.created_at.isoformat(),
        "updated_at": project.updated_at.isoformat(),
        "status": project.status.value,
        "tasks": project.tasks
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def load_project(name: str) -> Optional[Project]:
    """Load a project from file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "projects" / f"{name}.json"

    if not file_path.exists():
        return None

    with open(file_path, 'r') as f:
        data = json.load(f)

    from .models import ProjectStatus
    return Project(
        name=data["name"],
        description=data.get("description"),
        created_at=datetime.fromisoformat(data["created_at"]),
        updated_at=datetime.fromisoformat(data["updated_at"]),
        status=ProjectStatus(data.get("status", "active")),
        tasks=data.get("tasks", [])
    )


def list_projects() -> List[Project]:
    """List all projects."""
    storage_dir = get_storage_dir()
    projects_dir = storage_dir / "projects"

    if not projects_dir.exists():
        return []

    projects = []
    for file_path in projects_dir.glob("*.json"):
        project = load_project(file_path.stem)
        if project:
            projects.append(project)

    return projects


def delete_project(name: str):
    """Delete a project."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "projects" / f"{name}.json"

    if file_path.exists():
        file_path.unlink()


# ============================================================================
# Agent Operations
# ============================================================================

def save_agent(agent: Agent):
    """Save an agent to file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "agents" / f"{agent.id}.json"

    # Convert agent to dict (simplified)
    data = {
        "id": agent.id,
        "name": agent.name,
        "description": agent.description,
        "version": agent.version,
        "created_at": agent.created_at.isoformat(),
        "updated_at": agent.updated_at.isoformat(),
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def list_agents() -> List[Agent]:
    """List all agents."""
    storage_dir = get_storage_dir()
    agents_dir = storage_dir / "agents"

    if not agents_dir.exists():
        return []

    agents = []
    for file_path in agents_dir.glob("*.json"):
        with open(file_path, 'r') as f:
            data = json.load(f)
            agent = Agent(
                id=data["id"],
                name=data["name"],
                description=data["description"],
                version=data.get("version", "1.0.0"),
                created_at=datetime.fromisoformat(data["created_at"]),
                updated_at=datetime.fromisoformat(data["updated_at"])
            )
            agents.append(agent)

    return agents


def create_default_agents():
    """Create default agents."""
    storage_dir = get_storage_dir()
    agents_dir = storage_dir / "agents"
    agents_dir.mkdir(parents=True, exist_ok=True)

    # Create a default coder agent
    coder = Agent.new("coder", "Coder", "Software development specialist")
    save_agent(coder)


# ============================================================================
# Memory Operations
# ============================================================================

def save_memory(memory: Memory):
    """Save a memory to file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "memories" / f"{memory.id}.json"

    data = {
        "id": memory.id,
        "user_id": memory.user_id,
        "moment": memory.moment,
        "meaning": memory.meaning,
        "reason": memory.reason,
        "created_at": memory.created_at.isoformat(),
        "updated_at": memory.updated_at.isoformat(),
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def list_memories() -> List[Memory]:
    """List all memories."""
    storage_dir = get_storage_dir()
    memories_dir = storage_dir / "memories"

    if not memories_dir.exists():
        return []

    memories = []
    for file_path in memories_dir.glob("*.json"):
        with open(file_path, 'r') as f:
            data = json.load(f)
            from .models import MemoryImportance, MemoryTerm, MemoryType, ItemStatus
            memory = Memory(
                id=data["id"],
                user_id=data["user_id"],
                moment=data["moment"],
                meaning=data["meaning"],
                reason=data["reason"],
                created_at=datetime.fromisoformat(data["created_at"]),
                updated_at=datetime.fromisoformat(data["updated_at"])
            )
            memories.append(memory)

    return memories


# ============================================================================
# Idea Operations
# ============================================================================

def save_idea(idea: Idea):
    """Save an idea to file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "ideas" / f"{idea.id}.json"

    data = {
        "id": idea.id,
        "idea": idea.idea,
        "created_at": idea.created_at.isoformat(),
        "updated_at": idea.updated_at.isoformat(),
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def list_ideas() -> List[Idea]:
    """List all ideas."""
    storage_dir = get_storage_dir()
    ideas_dir = storage_dir / "ideas"

    if not ideas_dir.exists():
        return []

    ideas = []
    for file_path in ideas_dir.glob("*.json"):
        with open(file_path, 'r') as f:
            data = json.load(f)
            from .models import ShareLevel, IdeaImportance, ItemStatus
            idea = Idea(
                id=data["id"],
                idea=data["idea"],
                created_at=datetime.fromisoformat(data["created_at"]),
                updated_at=datetime.fromisoformat(data["updated_at"])
            )
            ideas.append(idea)

    return ideas


# ============================================================================
# Queue Operations
# ============================================================================

def save_queue_collection(collection: QueueCollection):
    """Save queue collection to file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "queue" / "queue.json"

    data = {
        "version": collection.version,
        "created_at": collection.created_at.isoformat(),
        "updated_at": collection.updated_at.isoformat(),
        "items": {item_id: {
            "id": item.id,
            "task_name": item.task_name,
            "task_description": item.task_description,
            "priority": item.priority.value,
            "status": item.status.value,
            "created_at": item.created_at.isoformat(),
            "updated_at": item.updated_at.isoformat(),
        } for item_id, item in collection.items.items()}
    }

    with open(file_path, 'w') as f:
        json.dump(data, f, indent=2)


def load_queue_collection() -> QueueCollection:
    """Load queue collection from file."""
    storage_dir = get_storage_dir()
    file_path = storage_dir / "queue" / "queue.json"

    if not file_path.exists():
        return QueueCollection()

    with open(file_path, 'r') as f:
        data = json.load(f)

    collection = QueueCollection()
    # Load items from data...
    return collection


def add_queue_item(item: QueueItem):
    """Add an item to the queue."""
    collection = load_queue_collection()
    collection.add_item(item)
    save_queue_collection(collection)


def list_queue_items() -> List[QueueItem]:
    """List all queue items."""
    collection = load_queue_collection()
    return list(collection.items.values())


def list_backlog_items() -> List[QueueItem]:
    """List backlog items."""
    collection = load_queue_collection()
    return collection.get_backlog_items()


def list_active_items() -> List[QueueItem]:
    """List active items."""
    collection = load_queue_collection()
    return collection.get_active_items()


def start_queue_session(item_id: str) -> str:
    """Start a queue session."""
    collection = load_queue_collection()
    session_id = collection.start_session(item_id)
    save_queue_collection(collection)
    return session_id


def end_queue_session(session_id: str):
    """End a queue session."""
    collection = load_queue_collection()
    collection.end_session(session_id)
    save_queue_collection(collection)


# ============================================================================
# Storage Class (Main Interface)
# ============================================================================

class Storage:
    """Main storage interface."""

    def __init__(self):
        self.storage_dir = get_storage_dir()

    @staticmethod
    async def new() -> 'Storage':
        """Create a new storage instance."""
        await init_storage()
        return Storage()

    async def add_task_to_project(self, task: Task):
        """Add a task to a project."""
        collection = load_task_collection("active")
        collection.add_task(task)
        save_task_collection("active", collection)

    async def update_task_in_project(self, task_id: str, updates):
        """Update a task in a project."""
        collection = load_task_collection("active")
        task = collection.get_task(task_id)
        if task:
            # Apply updates
            if updates.action is not None:
                task.action = updates.action
            if updates.status is not None:
                task.status = updates.status
            # ... apply other updates
            task.updated_at = datetime.utcnow()
            save_task_collection("active", collection)

    def delete_task_from_project(self, task_id: str):
        """Delete a task from a project."""
        collection = load_task_collection("active")
        collection.remove_task(task_id)
        save_task_collection("active", collection)

    def list_tasks_across_projects(self, filters) -> List[Task]:
        """List tasks across all projects."""
        collection = load_task_collection("active")
        return collection.get_all_tasks()

    def create_project(self, name: str, description: Optional[str]):
        """Create a new project."""
        project = Project(name=name, description=description)
        save_project(project)

    def list_projects(self) -> List[Project]:
        """List all projects."""
        return list_projects()

    def delete_project(self, name: str):
        """Delete a project."""
        delete_project(name)
