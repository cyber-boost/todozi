#!/usr/bin/env python3
"""
Example 8: Complete Task Lifecycle
Demonstrates the full lifecycle of task management from creation to completion
"""

from todozi import TodoziClient
import time

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🔄 Complete Task Lifecycle Demo")

    # Phase 1: Task Creation
    print("\n📝 Phase 1: Creating a new feature task")
    task_id = tdz.task("Implement user profile customization feature")
    print(f"Created task with ID: {task_id}")

    # Show initial state
    all_tasks = tdz.all()
    task = next((t for t in all_tasks if t.id == task_id), None)
    if task:
        print(f"Initial status: {task.status}")

    # Phase 2: Task Planning and Assignment
    print("\n🎯 Phase 2: Planning and AI assignment")
    ai_task_id = tdz.ai_task("Design the database schema for user profiles")
    human_task_id = tdz.human_task("Create wireframes for profile customization UI")

    print(f"AI task created: {ai_task_id}")
    print(f"Human task created: {human_task_id}")

    # Phase 3: Starting Work
    print(f"\n▶️  Phase 3: Starting work on main task")
    start_result = tdz.start(task_id)
    print(f"Started working: {start_result}")

    # Phase 4: Progress Updates
    print("\n📈 Phase 4: Progress tracking")
    print("Working on implementation...")

    # Simulate work progress
    time.sleep(0.5)

    # Phase 5: Task Completion
    print("
✅ Phase 5: Completing tasks"    done_result = tdz.done(task_id)
    print(f"Main task completed: {done_result}")

    # Complete the related tasks
    tdz.done(ai_task_id)
    tdz.done(human_task_id)
    print("Related tasks also completed")

    # Phase 6: Verification
    print("
🔍 Phase 6: Verifying completion"    all_tasks = tdz.all()

    # Find our completed tasks
    completed_tasks = [t for t in all_tasks if t.id in [task_id, ai_task_id, human_task_id]]

    print("Final status of our tasks:")
    for task in completed_tasks:
        status_emoji = {"done": "✅", "completed": "✅", "inprogress": "🔄", "todo": "📝"}.get(task.status.lower(), "📝")
        print(f"  {status_emoji} {task.action}")

    # Phase 7: Learning and Documentation
    print("
🧠 Phase 7: Capturing learnings"    memory_id = tdz.remember(
        "Completed user profile feature implementation",
        "Learned that early UI prototyping helps identify requirements gaps"
    )
    print(f"Captured learning: {memory_id}")

    # Phase 8: Next Steps Planning
    print("
🚀 Phase 8: Planning next steps"    next_task_id = tdz.task("Add unit tests for profile customization")
    followup_id = tdz.task("Deploy profile feature to staging environment")

    print("Created follow-up tasks:")
    print(f"  • Testing task: {next_task_id}")
    print(f"  • Deployment task: {followup_id}")

    print("\n🎉 Task lifecycle complete!")
    print("💡 This demonstrates the full workflow from idea to deployment")

if __name__ == "__main__":
    main()
