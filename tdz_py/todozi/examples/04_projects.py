#!/usr/bin/env python3
"""
Example 4: Project Management
Demonstrates creating projects, setting project context, and managing project tasks
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("📁 Project Management Demo")

    # Create some projects
    print("\n🏗️  Creating projects...")
    project1_id = tdz.create_project("Mobile App Development", "Building the next-gen mobile app")
    print(f"Created project: Mobile App Development")

    project2_id = tdz.create_project("Website Redesign", "Modernizing the company website")
    print(f"Created project: Website Redesign")

    project3_id = tdz.create_project("API Integration", "Connecting with third-party services")
    print(f"Created project: API Integration")

    # List all projects
    print("\n📋 All projects:")
    projects = tdz.list_projects()
    for project in projects:
        print(f"  • {project}")

    # Set project context and create tasks within projects
    print("\n🎯 Setting project context to 'Mobile App Development'...")
    tdz.set_project("Mobile App Development")

    # Create tasks that will be associated with the current project
    task1_id = tdz.task("Design user authentication flow")
    task2_id = tdz.task("Implement push notification system")
    task3_id = tdz.task("Create offline data synchronization")

    print(f"Created {3} tasks in current project context")

    # Get tasks for a specific project
    print("\n📝 Tasks in 'Mobile App Development' project:")
    mobile_tasks = tdz.project_tasks("Mobile App Development")
    for task in mobile_tasks:
        print(f"  • {task.action}")

    # Switch to another project
    print("\n🔄 Switching to 'Website Redesign' project...")
    tdz.set_project("Website Redesign")

    # Create tasks in the new project context
    web_task1 = tdz.task("Create new homepage mockups")
    web_task2 = tdz.task("Implement responsive design")

    print("Created 2 tasks in Website Redesign project")

if __name__ == "__main__":
    main()
