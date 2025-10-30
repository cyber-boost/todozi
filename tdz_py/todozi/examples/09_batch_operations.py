#!/usr/bin/env python3
"""
Example 9: Batch Operations
Demonstrates creating multiple tasks at once and bulk operations
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("📦 Batch Operations Demo")

    # Batch task creation - Sprint Planning
    print("\n🏃 Sprint Planning: Creating multiple tasks at once")

    sprint_tasks = [
        # High Priority Features
        "Implement user authentication system",
        "Design main dashboard layout",
        "Create user onboarding flow",

        # Medium Priority Tasks
        "Set up automated testing pipeline",
        "Optimize database queries",
        "Add error handling and logging",

        # Low Priority Tasks
        "Update API documentation",
        "Create deployment scripts",
        "Add monitoring and alerts"
    ]

    created_task_ids = []
    for task in sprint_tasks:
        task_id = tdz.task(task)
        created_task_ids.append(task_id)
        print(f"  ✅ {task}")

    print(f"\n🎯 Created {len(created_task_ids)} sprint tasks")

    # Batch priority assignment
    print("\n🎨 Batch Priority Assignment")

    # Mark urgent tasks
    urgent_tasks = ["Implement user authentication system"]
    for task_name in urgent_tasks:
        tasks = tdz.find(task_name)
        if tasks:
            tdz.urgent(tasks[0].action)  # Recreate with urgent priority
            print(f"  🔴 Made urgent: {task_name}")

    # Mark high priority tasks
    high_priority_tasks = ["Design main dashboard layout", "Create user onboarding flow"]
    for task_name in high_priority_tasks:
        tasks = tdz.find(task_name)
        if tasks:
            tdz.high(tasks[0].action)  # Recreate with high priority
            print(f"  🟠 Made high priority: {task_name}")

    # Batch project assignment
    print("\n📁 Batch Project Assignment")

    project_name = "Q1 Product Launch"
    project_id = tdz.create_project(project_name, "Major product launch for Q1")

    # Set project context and recreate tasks in project
    tdz.set_project(project_name)

    print(f"  📋 Created project: {project_name}")

    # Bulk status updates
    print("\n🔄 Bulk Status Updates")

    # Start working on high priority tasks
    high_tasks = tdz.find("high")  # This might not work, let's use a different approach
    all_tasks = tdz.all()

    started_count = 0
    for task in all_tasks[-5:]:  # Start the last 5 tasks as an example
        if task.status.lower() == "todo":
            tdz.start(task.id)
            started_count += 1

    print(f"  ▶️  Started work on {started_count} tasks")

    # Bulk completion simulation
    print("\n✅ Bulk Completion")

    # Complete some tasks
    completed_count = 0
    tasks_to_complete = all_tasks[-3:]  # Complete the last 3 tasks

    for task in tasks_to_complete:
        if task.status.lower() in ["todo", "inprogress"]:
            tdz.done(task.id)
            completed_count += 1

    print(f"  ✅ Completed {completed_count} tasks")

    # Show final project status
    print("
📊 Final Project Status"    project_tasks = tdz.project_tasks(project_name)
    print(f"Total tasks in '{project_name}': {len(project_tasks)}")

    status_summary = {}
    for task in project_tasks:
        status = task.status.lower()
        status_summary[status] = status_summary.get(status, 0) + 1

    for status, count in status_summary.items():
        status_emoji = {"todo": "📝", "inprogress": "🔄", "done": "✅"}.get(status, "📝")
        print(f"  {status_emoji} {status}: {count}")

    print("\n🎉 Batch operations complete!")

if __name__ == "__main__":
    main()
