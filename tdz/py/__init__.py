"""Todozi Python SDK.

This package provides Python bindings for the Todozi task management system.
"""

from .models import (
    Task, TaskUpdate, TaskFilters, TaskCollection,
    Project, Agent, Memory, Idea, Error, TrainingData,
    Priority, Status, Config, RegistrationInfo
)
from .error import (
    TodoziError, TaskNotFound, ProjectNotFound, ValidationError,
    StorageError, ConfigError
)
from .storage import Storage, init_storage, get_storage_dir
from .agent import AgentManager
from .api import ApiKeyManager
from .search import SearchEngine

__version__ = "1.2.0"

__all__ = [
    # Models
    "Task",
    "TaskUpdate",
    "TaskFilters",
    "TaskCollection",
    "Project",
    "Agent",
    "Memory",
    "Idea",
    "Error",
    "TrainingData",
    "Priority",
    "Status",
    "Config",
    "RegistrationInfo",

    # Errors
    "TodoziError",
    "TaskNotFound",
    "ProjectNotFound",
    "ValidationError",
    "StorageError",
    "ConfigError",

    # Storage
    "Storage",
    "init_storage",
    "get_storage_dir",

    # Managers
    "AgentManager",
    "ApiKeyManager",
    "SearchEngine",
]


# Simple API functions for quick access
class Todozi:
    """Simple Todozi interface."""

    @staticmethod
    async def init():
        """Initialize Todozi storage."""
        await init_storage()

    @staticmethod
    async def create_task(action: str, priority: Priority = Priority.MEDIUM,
                         project: str = "general") -> Task:
        """Create a new task."""
        storage = await Storage.new()
        task = Task.new(
            user_id="default",
            action=action,
            time="ASAP",
            priority=priority,
            parent_project=project,
            status=Status.TODO
        )
        await storage.add_task_to_project(task)
        return task

    @staticmethod
    async def search_tasks(query: str) -> list:
        """Search for tasks."""
        engine = SearchEngine()
        return await engine.search_tasks(query)

    @staticmethod
    async def list_tasks() -> list:
        """List all tasks."""
        storage = await Storage.new()
        return storage.list_tasks_across_projects(TaskFilters())
