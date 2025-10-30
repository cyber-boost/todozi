#!/usr/bin/env python3
"""
Example 2: AI Assignment Features
Demonstrates how to assign tasks to AI, humans, or collaborative teams
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🤖 Creating tasks with AI assignments...")

    # Create tasks assigned to different agents
    ai_task_id = tdz.ai_task("Analyze customer feedback data for sentiment trends")
    print(f"🤖 Created AI-assigned task: {ai_task_id}")

    human_task_id = tdz.human_task("Conduct user interviews for new feature validation")
    print(f"👤 Created human-assigned task: {human_task_id}")

    collab_task_id = tdz.collab_task("Design and implement the new dashboard UI")
    print(f"🤝 Created collaborative task: {collab_task_id}")

    print("\n✅ AI assignment tasks created!")
    print("💡 AI tasks are automatically optimized for AI processing")
    print("💡 Human tasks are prioritized for human expertise")
    print("💡 Collaborative tasks involve both AI and human coordination")

if __name__ == "__main__":
    main()
