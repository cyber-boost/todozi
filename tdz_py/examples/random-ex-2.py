# example_usage.py
import asyncio
from todozi import Task, Assignee, Priority, Status
from todozi.storage import Storage
from todozi.todozi import process_chat_message_extended, execute_task

async def main():
    # Initialize storage
    storage = await Storage.new()
    
    # Example 1: Create tasks manually
    print("=== Creating Tasks Manually ===")
    task1 = Task(
        action="Fix critical bug in login flow",
        time="2 hours",
        priority=Priority.High,
        parent_project="web-app",
        status=Status.Todo,
        assignee=Assignee.ai()
    )
    
    task2 = Task(
        action="Design new user dashboard",
        time="1 day",
        priority=Priority.Medium,
        parent_project="web-app",
        status=Status.Todo,
        assignee=Assignee.human()
    )
    
    # Add tasks to storage
    await storage.add_task_to_project(task1)
    await storage.add_task_to_project(task2)
    print(f"Created tasks: {task1.id}, {task2.id}")
    
    # Example 2: Process natural language input
    print("\n=== Processing Natural Language ===")
    chat_input = """
    I need to <todozi>Review pull request #45; 1 hour; high; backend; todo; assignee=human</todozi>
    Also, please <todozi>Update documentation for API v2; 3 hours; medium; docs; todo; assignee=ai</todozi>
    """
    
    content = process_chat_message_extended(chat_input, "user123")
    print(f"Extracted {len(content.tasks)} tasks from chat input")
    
    for task in content.tasks:
        await storage.add_task_to_project(task)
        print(f"- {task.action} (Assigned to: {task.assignee.kind.name})")
    
    # Example 3: Execute tasks
    print("\n=== Executing Tasks ===")
    all_tasks = storage.list_tasks_across_projects(storage.create_task_filters())
    
    for task in all_tasks[:3]:  # Execute first 3 tasks
        try:
            result = await execute_task(storage, task)
            print(f"Executed: {task.action}")
            print(f"Result: {result}")
        except Exception as e:
            print(f"Failed to execute {task.action}: {e}")
    
    # Example 4: Show task statistics
    print("\n=== Task Statistics ===")
    active_tasks = storage.list_tasks_across_projects(
        storage.create_task_filters(status=Status.TODO)
    )
    completed_tasks = storage.list_tasks_across_projects(
        storage.create_task_filters(status=Status.DONE)
    )
    print(f"Active tasks: {len(active_tasks)}")
    print(f"Completed tasks: {len(completed_tasks)}")

if __name__ == "__main__":
    asyncio.run(main())