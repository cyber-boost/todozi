#!/usr/bin/env python3
"""
Example 3: Task Management Operations
Demonstrates finding, starting, and completing tasks
"""

from todozi import TodoziClient
import time

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("📋 Task Management Operations Demo")

    # First, let's create a few test tasks
    print("\n📝 Creating test tasks...")
    task1_id = tdz.task("Write unit tests for user authentication")
    task2_id = tdz.task("Update API documentation")
    task3_id = tdz.task("Optimize database queries")

    print(f"Created tasks: {task1_id}, {task2_id}, {task3_id}")

    # Demonstrate finding tasks
    print("\n🔍 Finding tasks containing 'API'...")
    api_tasks = tdz.find("API")
    print(f"Found {len(api_tasks)} tasks:")
    for task in api_tasks:
        print(f"  • {task.action} (Status: {task.status})")

    # Start working on a task
    print(f"\n▶️  Starting work on task: {task1_id}")
    result = tdz.start(task1_id)
    print(f"Started: {result}")

    # Wait a moment to simulate work
    print("⏳ Working on task...")
    time.sleep(1)

    # Mark task as completed
    print(f"\n✅ Marking task as done: {task1_id}")
    done_result = tdz.done(task1_id)
    print(f"Completed: {done_result}")

    # Show all tasks to see the status change
    print("\n📊 Current task status:")
    all_tasks = tdz.all()
    for task in all_tasks[-3:]:  # Show last 3 tasks
        status_emoji = {"todo": "📝", "inprogress": "🔄", "done": "✅"}.get(task.status.lower(), "📝")
        print(f"  {status_emoji} {task.action}")

if __name__ == "__main__":
    main()
