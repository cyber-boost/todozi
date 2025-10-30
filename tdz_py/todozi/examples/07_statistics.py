#!/usr/bin/env python3
"""
Example 7: Statistics and Analytics
Demonstrates getting task statistics and detailed analytics
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("📊 Statistics and Analytics Demo")

    # Create some tasks with different statuses to demonstrate stats
    print("\n📝 Creating sample tasks with different statuses...")

    # Create tasks and manipulate their status
    task1 = tdz.task("Complete project proposal")
    task2 = tdz.task("Review code changes")
    task3 = tdz.task("Update documentation")
    task4 = tdz.task("Fix critical bug")
    task5 = tdz.task("Plan team meeting")

    # Start some tasks
    tdz.start(task1)
    tdz.start(task2)

    # Complete one task
    tdz.done(task3)

    print("✅ Created tasks with mixed statuses (todo, inprogress, done)")

    # Get basic statistics
    print("\n📈 Basic Statistics:")
    try:
        stats = tdz.stats()
        print("Quick stats:", stats)
    except Exception as e:
        print(f"Basic stats: {e}")

    # Get detailed statistics
    print("\n📊 Detailed Statistics:")
    try:
        detailed_stats = tdz.detailed_stats()
        print("Detailed stats:", detailed_stats)
    except Exception as e:
        print(f"Detailed stats: {e}")

    # Show current task breakdown
    print("\n📋 Current Task Status Breakdown:")
    all_tasks = tdz.all()

    status_counts = {}
    priority_counts = {}

    for task in all_tasks:
        # Count by status
        status = task.status.lower()
        status_counts[status] = status_counts.get(status, 0) + 1

        # Count by priority (if available)
        if hasattr(task, 'priority'):
            priority = task.priority.lower()
            priority_counts[priority] = priority_counts.get(priority, 0) + 1

    print("By Status:")
    for status, count in status_counts.items():
        status_emoji = {"todo": "📝", "inprogress": "🔄", "done": "✅", "completed": "✅"}.get(status, "📝")
        print(f"  {status_emoji} {status}: {count}")

    if priority_counts:
        print("\nBy Priority:")
        for priority, count in priority_counts.items():
            priority_emoji = {"urgent": "🔴", "high": "🟠", "medium": "🟡", "low": "🟢"}.get(priority, "🟡")
            print(f"  {priority_emoji} {priority}: {count}")

    print(f"\n📊 Total Tasks: {len(all_tasks)}")
    print("✅ Statistics analysis complete!")

if __name__ == "__main__":
    main()
