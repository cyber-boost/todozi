#!/usr/bin/env python3
"""Test script to verify TUI data loading"""

import asyncio
import sys
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent))

from todozi.storage import Storage, TaskFilters


async def test_tui_data_loading():
    """Test that the TUI can load data from storage"""

    print("=" * 80)
    print("Testing Todozi TUI Data Loading")
    print("=" * 80)

    # Initialize storage
    print("\n1. Initializing storage...")
    storage = await Storage.new()
    print("   ✓ Storage initialized")

    # Load tasks
    print("\n2. Loading tasks...")
    tasks = storage.list_tasks_across_projects(TaskFilters())
    print(f"   ✓ Found {len(tasks)} tasks")

    # Display sample tasks
    if tasks:
        print("\n3. Sample tasks:")
        for i, task in enumerate(tasks[:5], 1):
            print(f"   {i}. [{task.status.name}] {task.action[:50]}")
            print(
                f"      Priority: {task.priority.name}, Project: {task.parent_project}"
            )

    # Load projects
    print("\n4. Loading projects...")
    projects = storage.list_projects()
    print(f"   ✓ Found {len(projects)} projects")

    # Display sample projects
    if projects:
        print("\n5. Sample projects:")
        for i, project in enumerate(projects[:5], 1):
            print(f"   {i}. {project.name} - {project.description or 'No description'}")

    print("\n" + "=" * 80)
    print("Test Summary:")
    print(f"  • Tasks loaded: {len(tasks)}")
    print(f"  • Projects loaded: {len(projects)}")
    print(f"  • Storage working: ✓")
    print("=" * 80)

    return len(tasks) > 0 and len(projects) > 0

    return len(tasks) > 0 and len(projects) > 0

if __name__ == "__main__":
    result = asyncio.run(test_tui_data_loading())
    sys.exit(0 if result else 1)
