"""Todozi core utilities.

Main Todozi operations and utilities.
"""

from typing import List, Optional
from .models import Task
from .storage import Storage


class TDZ:
    """Core Todozi operations."""

    def __init__(self):
        self.storage: Optional[Storage] = None

    async def initialize(self):
        """Initialize TDZ."""
        self.storage = await Storage.new()

    async def add_task(self, task: Task) -> str:
        """Add a task."""
        if not self.storage:
            await self.initialize()
        await self.storage.add_task_to_project(task)
        return task.id

    async def get_task(self, task_id: str) -> Optional[Task]:
        """Get a task by ID."""
        if not self.storage:
            await self.initialize()
        # Implementation depends on storage
        return None

    async def list_tasks(self, project: Optional[str] = None) -> List[Task]:
        """List tasks."""
        if not self.storage:
            await self.initialize()
        from .models import TaskFilters
        filters = TaskFilters(project=project) if project else TaskFilters()
        return self.storage.list_tasks_across_projects(filters)
