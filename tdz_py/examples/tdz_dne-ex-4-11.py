# example4_todozi_client.py
import asyncio
import os
from tdz_dne import (
    TdzCommand,
    execute_tdz_command,
    parse_tdz_command,
    process_tdz_commands,
    HttpMethod
)

async def main():
    # Server configuration
    BASE_URL = os.getenv("TODOZI_BASE_URL", "http://localhost:8636")
    API_KEY = os.getenv("TODOZI_API_KEY", "your_api_key_here")

    # Example 1: Create a new task using direct command execution
    print("=== Creating a New Task ===")
    create_task_cmd = TdzCommand(
        command="create",
        target="task",
        parameters=[],
        options={
            "action": "Implement user authentication",
            "time": "4 hours",
            "priority": "high",
            "project": "web-app",
            "status": "todo"
        }
    )
    
    result = await execute_tdz_command(
        command=create_task_cmd,
        base_url=BASE_URL,
        api_key=API_KEY
    )
    
    if result.is_ok:
        task_data = result.unwrap()
        task_id = task_data.get("task", {}).get("id", "unknown")
        print(f"✅ Task created with ID: {task_id}")
    else:
        print(f"❌ Failed to create task: {result.unwrap()}")

    # Example 2: List all tasks in a project
    print("\n=== Listing All Tasks ===")
    list_tasks_cmd = TdzCommand(
        command="list",
        target="tasks",
        parameters=[],
        options={}
    )
    
    result = await execute_tdz_command(
        command=list_tasks_cmd,
        base_url=BASE_URL,
        api_key=API_KEY
    )
    
    if result.is_ok:
        tasks = result.unwrap()
        print(f"📋 Found {len(tasks)} tasks:")
        for task in tasks:
            task_info = task.get("task", {})
            print(f"  - [{task_info.get('id')}] {task_info.get('action')} ({task_info.get('status')})")
    else:
        print(f"❌ Failed to list tasks: {result.unwrap()}")

    # Example 3: Update task status using command string parsing
    print("\n=== Updating Task Status ===")
    # First, we need a task ID - let's assume we have one from previous step
    task_to_update = task_id if 'task_id' in locals() else "task_12345"  # Fallback for demo
    
    update_command_text = f"<tdz>update; task; {task_to_update}; status=done</tdz>"
    parsed_commands = parse_tdz_command(update_command_text)
    
    if parsed_commands.is_ok:
        commands = parsed_commands.unwrap()
        for cmd in commands:
            result = await execute_tdz_command(
                command=cmd,
                base_url=BASE_URL,
                api_key=API_KEY
            )
            if result.is_ok:
                print(f"✅ Task {cmd.parameters[0]} updated successfully")
            else:
                print(f"❌ Failed to update task: {result.unwrap()}")
    else:
        print(f"❌ Failed to parse command: {parsed_commands.unwrap()}")

    # Example 4: Search tasks by keyword
    print("\n=== Searching Tasks ===")
    search_cmd = TdzCommand(
        command="search",
        target="tasks",
        parameters=["authentication"],
        options={}
    )
    
    result = await execute_tdz_command(
        command=search_cmd,
        base_url=BASE_URL,
        api_key=API_KEY
    )
    
    if result.is_ok:
        search_results = result.unwrap()
        print(f"🔍 Found {len(search_results)} matching tasks:")
        for item in search_results:
            task = item.get("task", {})
            print(f"  - [{task.get('id')}] {task.get('action')}")
    else:
        print(f"❌ Search failed: {result.unwrap()}")

    # Example 5: Batch processing multiple commands
    print("\n=== Batch Processing Commands ===")
    batch_commands = """
    <tdz>create; task; Implement login page; 2 hours; medium; web-app; todo</tdz>
    <tdz>create; task; Add logout functionality; 1 hour; medium; web-app; todo</tdz>
    <tdz>list; tasks</tdz>
    """
    
    result = await process_tdz_commands(
        text=batch_commands,
        base_url=BASE_URL,
        api_key=API_KEY
    )
    
    if result.is_ok:
        responses = result.unwrap()
        print(f"✅ Processed {len(responses)} commands:")
        for i, response in enumerate(responses):
            if isinstance(response, list):  # List response
                print(f"  Command {i+1} (List): {len(response)} items")
            elif isinstance(response, dict) and "task" in response:
                task_id = response["task"].get("id", "unknown")
                print(f"  Command {i+1} (Create): Task {task_id} created")
            else:
                print(f"  Command {i+1}: {response}")
    else:
        print(f"❌ Batch processing failed: {result.unwrap()}")

if __name__ == "__main__":
    asyncio.run(main())