#!/usr/bin/env python3
"""
Example 2: Todozi Project and Task Management
Demonstrates:
1. Storage initialization
2. Project creation and management
3. Task creation with status and priority
4. Task updates and completion
5. Project-based task organization
"""

import asyncio
from pathlib import Path
from todozi.storage import (
    Storage,
    init_storage,
    create_default_agents,
    save_project,
    load_project,
    load_project_task_container,
    save_project_task_container,
    Project,
    Task,
    TaskUpdate,
    Status,
    Priority,
)
from todozi.models import Ok

async def main():
    # Initialize storage and create default agents
    print("🔧 Initializing Todozi storage...")
    await init_storage()
    create_default_agents()
    print("✅ Storage initialized with default agents")

    # Create a new storage instance
    storage = await Storage.new()
    print(f"📁 Storage directory: {Path.home() / '.todozi'}")

    # Create a new project
    project_name = "example_project"
    print(f"\n📂 Creating project: {project_name}")
    project = Project(name=project_name, description="Example project for demonstration")
    storage.create_project(project.name, project.description)
    
    # Load and display the created project
    loaded_project = storage.get_project(project_name)
    print(f"✅ Created project: {loaded_project.name}")
    print(f"   Description: {loaded_project.description}")
    print(f"   Archived: {loaded_project.archived}")

    # Load the project's task container
    container = load_project_task_container(project_name)
    print(f"📦 Loaded task container for project: {container.project_name}")

    # Create a new task
    task = Task(
        action="Write comprehensive documentation",
        context_notes="Include API endpoints and usage examples",
        status=Status.Todo,
        priority=Priority.High,
        parent_project=project_name,
    )
    print(f"\n📝 Adding task: {task.action}")
    await storage.add_task_to_project(task)
    print(f"✅ Task added with ID: {task.id}")

    # List all tasks in the project
    print(f"\n📋 Tasks in project '{project_name}':")
    tasks = storage.list_tasks_in_project(project_name, TaskFilters())
    for t in tasks:
        print(f"   [{t.id}] {t.action} (Status: {t.status.name}, Priority: {t.priority.name})")

    # Update the task status to InProgress
    print(f"\n🔄 Updating task {task.id} status to InProgress")
    updates = TaskUpdate()
    updates.status = Status.InProgress
    await storage.update_task_in_project(task.id, updates)
    
    # Show the updated task
    updated_task = storage.get_task_from_project(project_name, task.id)
    print(f"✅ Task updated: {updated_task.action} (Status: {updated_task.status.name})")

    # Complete the task
    print(f"\n✅ Completing task {task.id}")
    storage.complete_task_in_project(task.id)

    # Show final task list
    print(f"\n📋 Final task list in project '{project_name}':")
    final_tasks = storage.list_tasks_in_project(project_name, TaskFilters())
    for t in final_tasks:
        print(f"   [{t.id}] {t.action} (Status: {t.status.name}, Priority: {t.priority.name})")

    # Demonstrate project statistics
    stats = storage.get_project_stats(project_name)
    print(f"\n📊 Project statistics for '{project_name}':")
    print(f"   Total tasks: {stats.total_tasks}")
    print(f"   Active tasks: {stats.active_tasks}")
    print(f"   Completed tasks: {stats.completed_tasks}")
    print(f"   Archived tasks: {stats.archived_tasks}")
    print(f"   Deleted tasks: {stats.deleted_tasks}")

    # Create another task and add to the same project
    task2 = Task(
        action="Implement user authentication",
        context_notes="Use OAuth2 for secure authentication",
        status=Status.Todo,
        priority=Priority.Critical,
        parent_project=project_name,
    )
    print(f"\n📝 Adding second task: {task2.action}")
    await storage.add_task_to_project(task2)

    # Show all projects
    print("\n📂 All projects:")
    projects = storage.list_projects()
    for p in projects:
        print(f"   {p.name}: {p.description or 'No description'}")

    # Search tasks across all projects
    print("\n🔍 Searching for tasks containing 'documentation':")
    search_results = storage.search_tasks("documentation")
    for t in search_results:
        print(f"   Found: {t.action} (in project: {t.parent_project})")

    print("\n✨ Example completed successfully!")

if __name__ == "__main__":
    asyncio.run(main())