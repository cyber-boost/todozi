#!/usr/bin/env python3
"""
Example: Managing a Web Development Project with Todozi
"""

import asyncio
from pathlib import Path
from todozi.types import *
from todozi.todozi import process_chat_message_extended
from todozi.storage import Storage

async def project_workflow_example():
    """Complete workflow example for managing a web development project"""
    
    # Initialize storage
    storage = await Storage.new()
    
    print("🚀 Starting Web Development Project Management with Todozi")
    print("=" * 60)
    
    # 1. Create the project
    print("\n📁 Creating project...")
    project_cmd = CreateProject(
        name="web-portal",
        description="Customer web portal with authentication"
    )
    
    # Create task for project setup
    setup_task = AddTask(
        action="Initialize project structure and repositories",
        time="2 hours",
        priority="high",
        project="web-portal",
        status="todo",
        tags="setup,infrastructure"
    )
    
    # 2. Add tasks using chat processing
    print("\n📋 Adding project tasks via chat...")
    chat_message = """
    I need to complete the following for the web portal project:
    
    <todozi>Design database schema; 4 hours; high; web-portal; todo; tags=database,design</todozi>
    <todozi>Implement user authentication; 8 hours; critical; web-portal; todo; tags=auth,security</todozi>
    <todozi>Create responsive UI layout; 12 hours; medium; web-portal; todo; tags=frontend,css</todozi>
    <todozi>Set up CI/CD pipeline; 6 hours; medium; web-portal; todo; tags=devops,automation</todozi>
    """
    
    # Process the chat message
    content = process_chat_message_extended(chat_message, "project_manager")
    
    # Save tasks to storage
    for task in content.tasks:
        await storage.add_task_to_project(task)
        print(f"✅ Task created: {task.action}")
    
    # 3. Create memory entries for project knowledge
    print("\n🧠 Storing project memories...")
    memory_chat = """
    Project kickoff notes and decisions:
    
    <memory>standard; Database Decision; PostgreSQL chosen for ACID compliance and JSON support; Critical for transaction integrity; high; long; database,architecture</memory>
    <memory>standard; UI Framework; React with TypeScript selected for team expertise; Important for maintainability; medium; long; frontend,decisions</memory>
    <memory>human; Stakeholder Meeting; Client wants responsive design with mobile-first approach; Key requirement for project success; high; long; requirements,client</memory>
    """
    
    memory_content = process_chat_message_extended(memory_chat, "project_manager")
    
    # 4. Create ideas for future consideration
    print("\n💡 Capturing project ideas...")
    idea_chat = """
    Future enhancement ideas:
    
    <idea>Implement real-time notifications using WebSockets; team; high; tags=features,realtime</idea>
    <idea>Add data analytics dashboard for users; private; medium; tags=analytics,features</idea>
    <idea>Mobile app companion; private; low; tags=mobile,future</idea>
    """
    
    idea_content = process_chat_message_extended(idea_chat, "project_manager")
    
    # 5. Assign agents to specific tasks
    print("\n🤖 Assigning specialized agents...")
    agent_assignment = """
    <todozi_agent>task_001; architect; web-portal</todozi_agent>
    <todozi_agent>task_002; coder; web-portal</todozi_agent>
    <todozi_agent>task_003; designer; web-portal</todozi_agent>
    """
    
    agent_content = process_chat_message_extended(agent_assignment, "project_manager")
    
    # 6. Track project progress
    print("\n📊 Project Status Summary:")
    print(f"Tasks Created: {len(content.tasks)}")
    print(f"Memories Stored: {len(memory_content.memories)}")
    print(f"Ideas Captured: {len(idea_content.ideas)}")
    print(f"Agent Assignments: {len(agent_content.agent_assignments)}")
    
    # 7. Demonstrate search functionality
    print("\n🔍 Searching project content...")
    from todozi.types import SearchEngine, SearchOptions
    
    # Create search engine and index content
    search_engine = SearchEngine()
    
    # Combine all content for search
    all_content = ChatContent(
        tasks=content.tasks,
        memories=memory_content.memories,
        ideas=idea_content.ideas,
        agent_assignments=agent_content.agent_assignments
    )
    search_engine.update_index(all_content)
    
    # Search for database-related items
    search_results = search_engine.search(
        "database",
        SearchOptions(limit=10)
    )
    
    print(f"Found {len(search_results.task_results)} task(s) related to 'database':")
    for result in search_results.task_results[:3]:
        print(f"  - {result.action}")
    
    # 8. Update task status (simulating progress)
    print("\n📝 Updating task progress...")
    
    # Get first task and update its status
    if content.tasks:
        first_task = content.tasks[0]
        print(f"Updating task: {first_task.action}")
        
        # Create task update
        from todozi.models import TaskUpdate, Status
        update = TaskUpdate().with_status(Status.InProgress)
        
        await storage.update_task_in_project(first_task.id, update)
        print(f"✅ Task marked as in-progress")
    
    # 9. Export project data
    print("\n📤 Exporting project data...")
    
    # Get all tasks for the project
    from todozi.models import TaskFilters
    project_tasks = storage.list_tasks_across_projects(
        TaskFilters(project="web-portal")
    )
    
    print(f"Project 'web-portal' has {len(project_tasks)} tasks:")
    for task in project_tasks:
        status_emoji = "🔄" if str(task.status) == "in_progress" else "📝"
        priority_emoji = {
            "low": "🟢", "medium": "🟡", 
            "high": "🟠", "critical": "🔴"
        }.get(str(task.priority), "⚪")
        
        print(f"  {status_emoji} {priority_emoji} {task.action}")
        print(f"      Tags: {', '.join(task.tags) if task.tags else 'none'}")
    
    # 10. Generate project report
    print("\n📊 Project Report Generated")
    print("=" * 60)
    print("Summary:")
    print(f"- Total Tasks: {len(content.tasks)}")
    print(f"- High Priority Tasks: {len([t for t in content.tasks if str(t.priority) == 'high'])}")
    print(f"- Critical Tasks: {len([t for t in content.tasks if str(t.priority) == 'critical'])}")
    print(f"- Team Knowledge Items: {len(memory_content.memories)}")
    print(f"- Future Ideas: {len(idea_content.ideas)}")
    print(f"\nNext Steps:")
    print("- Review and prioritize remaining tasks")
    print("- Assign team members based on skills")
    print("- Set up development milestones")
    print("- Schedule regular progress reviews")

if __name__ == "__main__":
    asyncio.run(project_workflow_example())