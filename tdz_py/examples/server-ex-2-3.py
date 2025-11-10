#!/usr/bin/env python3
"""
todozi_project_workflow.py

A comprehensive example showing how to use Todozi to manage a software development project.
This script demonstrates:
- Project initialization and setup
- Task creation with different priorities
- Natural language task extraction
- Semantic search functionality
- Agent assignments
- Progress tracking
"""

import asyncio
import json
import time
from pathlib import Path

# Import Todozi components
from todozi.storage import Storage, Project, Task, Priority, Status, Assignee
from todozi.todozi import process_chat_message_extended
from todozi.server import TodoziServer, ServerConfig


async def initialize_workspace():
    """Initialize a new Todozi workspace for our project"""
    print("🚀 Initializing Todozi workspace...")
    
    storage = await Storage.new()
    
    # Create main project
    web_app_project = Project(
        name="web-app",
        description="E-commerce web application",
    )
    storage.create_project(web_app_project.name, web_app_project.description)
    
    # Create supporting projects
    mobile_app_project = Project(
        name="mobile-app",
        description="React Native mobile application",
    )
    storage.create_project(mobile_app_project.name, mobile_app_project.description)
    
    api_project = Project(
        name="api",
        description="RESTful backend API",
    )
    storage.create_project(api_project.name, api_project.description)
    
    print("✅ Workspace initialized with projects:")
    print(f"  - {web_app_project.name}: {web_app_project.description}")
    print(f"  - {mobile_app_project.name}: {mobile_app_project.description}")
    print(f"  - {api_project.name}: {api_project.description}")
    
    return storage


async def create_initial_tasks(storage):
    """Create initial tasks for the project using Todozi format"""
    print("\n📋 Creating initial tasks...")
    
    # Web app tasks
    web_tasks = [
        "<todozi>Set up React development environment; 2 days; high; web-app; todo; assignee=human; tags=setup,react</todozi>",
        "<todozi>Design UI components library; 1 week; high; web-app; todo; assignee=human; tags=design,ui</todozi>",
        "<todozi>Implement user authentication; 3 days; critical; web-app; in_progress; assignee=human; tags=auth,security</todozi>",
        "<todozi>Create product catalog pages; 2 days; medium; web-app; todo; assignee=human; tags=frontend</todozi>",
    ]
    
    # API tasks
    api_tasks = [
        "<todozi>Design database schema; 2 days; high; api; todo; assignee=human; tags=database,design</todozi>",
        "<todozi>Implement user endpoints; 3 days; high; api; in_progress; assignee=human; tags=backend,auth</todozi>",
        "<todozi>Create product management API; 5 days; medium; api; todo; assignee=ai; tags=api,products</todozi>",
        "<todozi>Set up API documentation; 1 day; low; api; todo; assignee=human; tags=docs</todozi>",
    ]
    
    # Mobile app tasks
    mobile_tasks = [
        "<todozi>Set up React Native project; 2 days; high; mobile-app; todo; assignee=human; tags=setup,react-native</todozi>",
        "<todozi>Implement navigation; 3 days; medium; mobile-app; todo; assignee=human; tags=mobile,ui</todozi>",
        "<todozi>Integrate with backend API; 4 days; high; mobile-app; todo; assignee=human; tags=integration,api</todozi>",
    ]
    
    all_task_strings = web_tasks + api_tasks + mobile_tasks
    
    # Process and add tasks
    for task_str in all_task_strings:
        try:
            content = process_chat_message_extended(task_str, "project_manager")
            if content.tasks:
                task = content.tasks[0]
                task.user_id = "project_manager"
                await storage.add_task_to_project(task)
                print(f"  ✅ Created: {task.action} (Project: {task.parent_project}, Priority: {task.priority})")
        except Exception as e:
            print(f"  ❌ Failed to create task from: {task_str[:50]}... - {e}")
    
    return len(all_task_strings)


async def process_team_meeting_notes(storage):
    """Demonstrate natural language processing of meeting notes"""
    print("\n💬 Processing team meeting notes...")
    
    meeting_notes = """
    Team meeting action items:
    
    <todozi>Add SSL certificate support; 1 week; critical; api; todo; assignee=ai; tags=security,ssl</todozi>
    
    We need to improve the checkout flow:
    <memory>standard; Checkout flow is too complex; Users abandon cart at payment step; High priority for conversion; high; long; conversion,ux</memory>
    
    <idea>Implement one-click checkout; private; high; This could significantly increase conversion; tags=ux,feature</idea>
    
    <todozi_agent>task_123; coder; api; Improve code quality</todozi_agent>
    
    Performance issues:
    <error>Database slow queries; API responses >2s; critical; database; api_service; Optimize product queries; database,performance</error>
    
    Training data for AI:
    <train>instruction; Generate API response format; {"status": "success", "data": {...}}; API response examples; json,api; 0.9; examples</train>
    """
    
    # Process the meeting notes
    content = process_chat_message_extended(meeting_notes, "project_manager")
    
    print(f"  📊 Extracted from meeting notes:")
    print(f"    Tasks: {len(content.tasks)}")
    print(f"    Memories: {len(content.memories)}")
    print(f"    Ideas: {len(content.ideas)}")
    print(f"    Agent assignments: {len(content.agent_assignments)}")
    print(f"    Errors: {len(content.errors)}")
    print(f"    Training data: {len(content.training_data)}")
    
    # Add the extracted task
    if content.tasks:
        task = content.tasks[0]
        task.user_id = "project_manager"
        await storage.add_task_to_project(task)
        print(f"  ✅ Added task from meeting: {task.action}")
    
    # Save other extracted content (simplified for demo)
    if content.memories:
        print(f"  💾 Saved memory about: {content.memories[0].moment}")
    if content.ideas:
        print(f"  💡 Saved idea: {content.ideas[0].idea}")
    if content.errors:
        print(f"  ⚠️  Logged error: {content.errors[0].title}")
    
    return content


async def demonstrate_semantic_search(storage):
    """Show Todozi's semantic search capabilities"""
    print("\n🔍 Demonstrating semantic search...")
    
    # Search for similar tasks
    search_queries = [
        "authentication",
        "database performance",
        "UI components",
        "API integration",
    ]
    
    for query in search_queries:
        try:
            results = await storage.search_tasks_semantic(query, 3)
            print(f"\n  🔎 Search for '{query}':")
            if results:
                for result in results[:2]:  # Show top 2
                    print(f"    - {result.task.action} (Score: {result.score:.2f})")
                    print(f"      Project: {result.task.parent_project}")
                    print(f"      Priority: {result.task.priority}")
            else:
                print(f"    No results found")
        except Exception as e:
            print(f"    Search failed: {e}")


async def simulate_task_progress(storage):
    """Simulate progress on tasks"""
    print("\n📈 Updating task progress...")
    
    # Get all tasks
    all_tasks = storage.list_tasks_across_projects()
    
    # Update some tasks to show progress
    updates_made = 0
    for task in all_tasks[:3]:  # Update first 3 tasks as example
        if "Set up" in task.action:
            task.status = Status.Done
            task.progress = 100
        elif "Implement" in task.action and task.status == Status.InProgress:
            task.progress = 50
        elif "Design" in task.action:
            task.progress = 25
            task.status = Status.InProgress
        
        await storage.update_task_in_project(task.id, task)
        updates_made += 1
        print(f"  📝 Updated: {task.action} -> {task.status} ({task.progress}% complete)")
    
    return updates_made


async def start_todozi_server():
    """Start the Todozi HTTP server for API access"""
    print("\n🌐 Starting Todozi HTTP server...")
    
    config = ServerConfig(host="127.0.0.1", port=8636, max_connections=100)
    server = TodoziServer(config)
    
    print("  📡 Server running at http://127.0.0.1:8636")
    print("  📖 API endpoints available:")
    print("    GET  /tasks - List all tasks")
    print("    POST /tasks - Create new task")
    print("    GET  /tasks/search?q={query} - Search tasks")
    print("    GET  /projects - List projects")
    print("    GET  /health - Health check")
    
    # In a real scenario, you'd run: await server.start()
    # For this demo, we'll just show what would be available
    print("  💡 In a real deployment, run: await server.start()")
    
    # Show example API usage
    print("\n  📋 Example API usage:")
    print("    curl http://127.0.0.1:8636/tasks")
    print("    curl -X POST http://127.0.0.1:8636/tasks -H 'Content-Type: application/json' \\")
    print("         -d '{\"action\":\"New task\",\"time\":\"1 hour\",\"priority\":\"medium\",\"project\":\"web-app\"}'")
    
    return server


async def generate_project_report(storage):
    """Generate a project status report"""
    print("\n📊 Generating Project Status Report...")
    
    # Get statistics
    all_tasks = storage.list_tasks_across_projects()
    
    # Group by project and status
    project_stats = {}
    for task in all_tasks:
        project = task.parent_project
        if project not in project_stats:
            project_stats[project] = {"total": 0, "todo": 0, "in_progress": 0, "done": 0}
        project_stats[project]["total"] += 1
        project_stats[project][task.status.value.lower()] += 1
    
    print("\n  📈 Project Status Summary:")
    for project, stats in project_stats.items():
        total = stats["total"]
        done = stats.get("done", 0)
        completion_rate = (done / total * 100) if total > 0 else 0
        
        print(f"\n  📁 {project}:")
        print(f"    Total Tasks: {total}")
        print(f"    To Do: {stats.get('todo', 0)}")
        print(f"    In Progress: {stats.get('in_progress', 0)}")
        print(f"    Done: {done}")
        print(f"    Completion Rate: {completion_rate:.1f}%")
    
    # Show high priority tasks
    high_priority_tasks = [t for t in all_tasks if t.priority == Priority.Critical or t.priority == Priority.High]
    
    if high_priority_tasks:
        print(f"\n  🔥 High Priority Tasks ({len(high_priority_tasks)}):")
        for task in high_priority_tasks[:5]:
            status_emoji = {"todo": "📝", "in_progress": "🔄", "done": "✅"}.get(task.status.value.lower(), "❓")
            print(f"    {status_emoji} {task.action} ({task.parent_project})")
    
    return project_stats


async def main():
    """Main workflow demonstration"""
    print("=" * 60)
    print("🎯 Todozi Project Management Workflow Example")
    print("=" * 60)
    
    # 1. Initialize workspace
    storage = await initialize_workspace()
    
    # 2. Create initial tasks
    task_count = await create_initial_tasks(storage)
    print(f"\n📊 Total tasks created: {task_count}")
    
    # 3. Process meeting notes
    meeting_content = await process_team_meeting_notes(storage)
    
    # 4. Demonstrate semantic search
    await demonstrate_semantic_search(storage)
    
    # 5. Update task progress
    updated = await simulate_task_progress(storage)
    print(f"\n📝 Tasks updated: {updated}")
    
    # 6. Start server (info only)
    server = await start_todozi_server()
    
    # 7. Generate project report
    report = await generate_project_report(storage)
    
    print("\n" + "=" * 60)
    print("✨ Workflow Complete!")
    print("=" * 60)
    print("\n💡 Key Features Demonstrated:")
    print("  📁 Project Management")
    print("  📋 Task Creation with Todozi Format")
    print("  💬 Natural Language Processing")
    print("  🔍 Semantic Search")
    print("  📊 Progress Tracking")
    print("  🌐 HTTP API Server")
    print("  📈 Project Reporting")
    
    print("\n🚀 Next Steps:")
    print("  1. Start the server: python -m todozi.server")
    print("  2. Use CLI: todozi list tasks")
    print("  3. Add tasks: todozi add task --action='New feature' --time='2 days' --priority='high' --project='web-app'")
    print("  4. Process notes: todozi chat \"<todozi>Action; time; priority; project; status</todozi>\"")
    print("  5. Search: todozi search tasks \"authentication\"")


if __name__ == "__main__":
    # Run the demonstration
    asyncio.run(main())