#!/usr/bin/env python3
"""
Example 6: Search Functionality
Demonstrates keyword search and AI-powered semantic search
"""

from todozi import TodoziClient

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🔍 Search Functionality Demo")

    # First, create some diverse tasks to search through
    print("\n📝 Creating sample tasks for search testing...")

    sample_tasks = [
        "Fix bug in user authentication module",
        "Implement password reset functionality",
        "Design new dashboard layout",
        "Optimize database query performance",
        "Write unit tests for API endpoints",
        "Create user onboarding flow",
        "Implement email notification system",
        "Design mobile app user interface",
        "Set up CI/CD pipeline",
        "Write documentation for new features"
    ]

    created_tasks = []
    for task in sample_tasks:
        task_id = tdz.task(task)
        created_tasks.append(task_id)

    print(f"✅ Created {len(created_tasks)} sample tasks")

    # Demonstrate keyword search
    print("\n🔤 Keyword Search Examples:")

    # Search for tasks containing "design"
    design_tasks = tdz.find("design")
    print(f"Tasks containing 'design': {len(design_tasks)}")
    for task in design_tasks:
        print(f"  • {task.action}")

    # Search for tasks containing "API"
    api_tasks = tdz.find("API")
    print(f"\nTasks containing 'API': {len(api_tasks)}")
    for task in api_tasks:
        print(f"  • {task.action}")

    # Search for tasks containing "test"
    test_tasks = tdz.find("test")
    print(f"\nTasks containing 'test': {len(test_tasks)}")
    for task in test_tasks:
        print(f"  • {task.action}")

    # Demonstrate AI-powered semantic search
    print("\n🤖 AI-Powered Semantic Search Examples:")

    # Semantic search for user experience related tasks
    ux_tasks = tdz.ai_find("user experience and interface design")
    print(f"Semantic search for 'user experience': {len(ux_tasks)} results")
    for task in ux_tasks:
        print(f"  • {task.action}")

    # Semantic search for development workflow tasks
    workflow_tasks = tdz.ai_find("development workflow and automation")
    print(f"\nSemantic search for 'development workflow': {len(workflow_tasks)} results")
    for task in workflow_tasks:
        print(f"  • {task.action}")

    # Semantic search for security related tasks
    security_tasks = tdz.ai_find("security and authentication")
    print(f"\nSemantic search for 'security': {len(security_tasks)} results")
    for task in security_tasks:
        print(f"  • {task.action}")

    print("\n✅ Search functionality demonstrated!")
    print("💡 Keyword search finds exact matches")
    print("💡 AI search understands meaning and context")

if __name__ == "__main__":
    main()
