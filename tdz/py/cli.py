"""CLI interface for Todozi (Python stub).

This is a placeholder for CLI functionality.
The main CLI is implemented in Rust and handles command-line interactions.
"""

from typing import Optional


class TodoziCLI:
    """Command-line interface handler."""

    def __init__(self):
        pass

    async def init(self):
        """Initialize Todozi storage."""
        from .storage import init_storage
        await init_storage()
        print("✅ Todozi initialized")

    async def add_task(self, action: str, **kwargs):
        """Add a new task via CLI."""
        from .models import Task, Priority, Status
        from .storage import Storage

        storage = await Storage.new()
        task = Task.new(
            user_id="default",
            action=action,
            time=kwargs.get("time", "ASAP"),
            priority=Priority(kwargs.get("priority", "medium")),
            parent_project=kwargs.get("project", "general"),
            status=Status.TODO
        )
        await storage.add_task_to_project(task)
        print(f"✅ Task created: {task.id}")

    async def list_tasks(self, project: Optional[str] = None):
        """List tasks via CLI."""
        from .storage import Storage
        from .models import TaskFilters

        storage = await Storage.new()
        filters = TaskFilters(project=project) if project else TaskFilters()
        tasks = storage.list_tasks_across_projects(filters)

        print(f"\n📋 Found {len(tasks)} tasks:\n")
        for task in tasks:
            print(f"  [{task.id}] {task.action} ({task.status.value})")
