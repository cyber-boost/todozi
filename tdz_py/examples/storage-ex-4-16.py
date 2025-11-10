# example_project_task_management.py
import asyncio
from datetime import datetime
from pathlib import Path
import sys

# Add parent directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent))

from todozi.storage import Storage, Project, Task, Status, Priority
from todozi.models import TaskUpdate


async def main():
    # Initialize storage system
    await Storage.new()
    
    # Create a new project
    storage = Storage.new()
    storage.create_project("Website Redesign", "Redesign company website")
    
    # Create tasks for the project
    task1 = Task(
        action="Design homepage mockup",
        parent_project="Website Redesign",
        priority=Priority.High,
        status=Status.Todo
    )
    
    task2 = Task(
        action="Implement frontend components",
        parent_project="Website Redesign",
        priority=Priority.Medium,
        status=Status.Todo
    )
    
    # Add tasks to project
    await storage.add_task_to_project(task1)
    await storage.add_task_to_project(task2)
    
    # List all tasks in project
    tasks = storage.list_tasks_in_project("Website Redesign", None)
    print("Initial tasks:")
    for task in tasks:
        print(f"- {task.action} ({task.status})")
    
    # Update a task status
    update = TaskUpdate().with_status(Status.InProgress)
    await storage.update_task_in_project(task1.id, update)
    
    # Complete a task
    storage.complete_task_in_project(task2.id)
    
    # Show updated tasks
    tasks = storage.list_tasks_in_project("Website Redesign", None)
    print("\nAfter updates:")
    for task in tasks:
        print(f"- {task.action} ({task.status})")
    
    # Show project statistics
    stats = storage.get_project_stats("Website Redesign")
    print(f"\nProject Stats:")
    print(f"- Total: {stats.total_tasks}")
    print(f"- Active: {stats.active_tasks}")
    print(f"- Completed: {stats.completed_tasks}")


if __name__ == "__main__":
    asyncio.run(main())