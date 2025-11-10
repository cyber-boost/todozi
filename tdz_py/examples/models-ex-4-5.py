# example_usage.py
import asyncio
from datetime import datetime
from todozi.models import Task, Priority, Status, Assignee
from todozi.storage import Storage

async def main():
    # Initialize storage
    storage = await Storage.new()
    
    # Create a new task using the Task model
    task_result = Task.new_full(
        user_id="user_123",
        action="Implement user authentication",
        time="2 hours",
        priority=Priority.HIGH,
        parent_project="web-app",
        status=Status.TODO,
        assignee=Assignee.human(),
        tags=["auth", "security"],
        dependencies=[],
        context_notes="Use OAuth 2.0 flow",
        progress=None
    )
    
    if hasattr(task_result, 'value'):
        task = task_result.value
        
        # Add task to project-based storage
        await storage.add_task_to_project(task)
        print(f"✅ Created task: {task.action}")
        print(f"   ID: {task.id}")
        print(f"   Project: {task.parent_project}")
        print(f"   Priority: {task.priority}")
        print(f"   Status: {task.status}")
        print(f"   Assignee: {task.assignee}")
        print(f"   Tags: {task.tags}")
        
        # Retrieve and display the task
        retrieved_task = storage.get_task_from_project("web-app", task.id)
        print(f"\n🔍 Retrieved task from project:")
        print(f"   Action: {retrieved_task.action}")
        print(f"   Created: {retrieved_task.created_at}")
        
        # List all tasks in the project
        project_tasks = storage.list_tasks_in_project("web-app", None)
        print(f"\n📋 Tasks in 'web-app' project:")
        for t in project_tasks:
            print(f"   - {t.action} ({t.status})")
    else:
        print(f"❌ Error creating task: {task_result.error}")

if __name__ == "__main__":
    asyncio.run(main())