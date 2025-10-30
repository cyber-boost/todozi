#!/usr/bin/env python3
"""
Example 1: Basic Task Creation
Demonstrates how to create tasks with different priority levels
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🎯 Creating tasks with different priorities...")

    # Create tasks with different priority levels
    task_id = tdz.task("Review the quarterly budget report")
    print(f"📝 Created regular task: {task_id}")

    urgent_id = tdz.urgent("Fix critical security vulnerability in authentication")
    print(f"🔴 Created urgent task: {urgent_id}")

    high_id = tdz.high("Prepare presentation for board meeting")
    print(f"🟠 Created high priority task: {high_id}")

    low_id = tdz.low("Update README documentation")
    print(f"🟢 Created low priority task: {low_id}")

    print("\n✅ All tasks created successfully!")
    print("💡 Tip: Use tdz.all() to see all your tasks")

if __name__ == "__main__":
    main()
