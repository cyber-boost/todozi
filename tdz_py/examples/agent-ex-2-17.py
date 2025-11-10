import asyncio
from datetime import datetime, timezone
from todozi.storage import Storage
from todozi.models import Task, Priority, Status, Assignee, Project
from todozi.agent import AgentManager, AgentStatus
from todozi.memory import MemoryManager, MemoryImportance, MemoryTerm, MemoryType
from todozi.idea import IdeaManager, IdeaImportance, ShareLevel
from todozi.todozi import process_chat_message_extended, ChatContent
from todozi.queue import add_queue_item, list_queue_items_by_status, QueueStatus
from todozi.error import ErrorManager, ErrorSeverity, ErrorCategory

async def manage_web_project():
    """Example: Managing a web application development project"""
    
    # Initialize storage and managers
    storage = await Storage.new()
    agent_manager = AgentManager()
    await agent_manager.load_agents()
    
    memory_manager = MemoryManager()
    await memory_manager.load_memories()
    
    idea_manager = IdeaManager()
    await idea_manager.load_ideas()
    
    # Create a project
    project = Project(
        name="web-dashboard",
        description="Customer analytics dashboard",
        archived=False
    )
    storage.create_project(project.name, project.description)
    
    print("🚀 Starting Web Dashboard Project Management")
    print("=" * 50)
    
    # 1. Create initial tasks through natural language
    print("\n📝 Processing project requirements through chat:")
    chat_message = """
    I need to <todozi>Design database schema; 3 days; high; web-dashboard; todo; assignee=human; tags=database,design</todozi>
    and <todozi>Implement authentication system; 5 days; critical; web-dashboard; todo; assignee=agent=coder; tags=security,backend</todozi>
    Also <todozi>Create UI mockups; 2 days; medium; web-dashboard; todo; assignee=agent=designer; tags=ui,frontend</todozi>
    """
    
    content = process_chat_message_extended(chat_message, "project_manager")
    
    # Add tasks to storage
    for task in content.tasks:
        await storage.add_task_to_project(task)
        print(f"✅ Created task: {task.action} (Assigned to: {task.assignee})")
    
    # 2. Store project memories
    print("\n🧠 Storing important project memories:")
    
    memory1 = await memory_manager.create_memory(
        moment="Project kickoff meeting",
        meaning="Client emphasized need for real-time data visualization",
        reason="Critical requirement for dashboard functionality",
        importance=MemoryImportance.HIGH,
        term=MemoryTerm.LONG,
        memory_type=MemoryType.STANDARD,
        tags=["requirements", "client"]
    )
    
    memory2 = await memory_manager.create_memory(
        moment="Technical decision",
        meaning="Chose React for frontend due to team expertise",
        reason="Reduces development time and learning curve",
        importance=MemoryImportance.MEDIUM,
        term=MemoryTerm.SHORT,
        memory_type=MemoryType.STANDARD,
        tags=["technology", "frontend"]
    )
    
    print(f"✅ Stored {len([memory1, memory2])} important memories")
    
    # 3. Capture ideas during development
    print("\n💡 Capturing innovative ideas:")
    
    idea1 = await idea_manager.create_idea(
        idea="Add predictive analytics using machine learning",
        share=ShareLevel.TEAM,
        importance=IdeaImportance.HIGH,
        tags=["ai", "analytics"],
        context="Could provide valuable insights for users"
    )
    
    idea2 = await idea_manager.create_idea(
        idea="Implement dark mode theme switcher",
        share=ShareLevel.PUBLIC,
        importance=IdeaImportance.MEDIUM,
        tags=["ui", "accessibility"]
    )
    
    print(f"✅ Captured {len([idea1, idea2])} ideas for future consideration")
    
    # 4. Assign complex task to specialized agent
    print("\n🤖 Assigning complex database design to architect agent:")
    
    db_task = Task(
        action="Design database schema for analytics dashboard",
        time="3 days",
        priority=Priority.High,
        parent_project="web-dashboard",
        status=Status.Todo,
        assignee=Assignee.agent("architect"),
        tags=["database", "schema", "design"]
    )
    
    await storage.add_task_to_project(db_task)
    assignment = await agent_manager.assign_task_to_agent(
        task_id=db_task.id,
        agent_id="architect",
        project_id="web-dashboard"
    )
    print(f"✅ Assigned database design to architect agent")
    
    # 5. Queue management for workflow
    print("\n📋 Managing task queue:")
    
    # Add high-priority items to active queue
    from todozi.models import QueueItem
    
    critical_task = QueueItem(
        task_name="Fix security vulnerability",
        task_description="URGENT: Patch authentication bypass issue",
        priority=Priority.Critical.value,
        project_id="web-dashboard"
    )
    add_queue_item(critical_task)
    
    # Show queue status
    backlog_items = list_queue_items_by_status(QueueStatus.Backlog)
    print(f"📦 Backlog has {len(backlog_items)} items:")
    for item in backlog_items[:3]:
        print(f"  - {item.task_name}: {item.task_description[:50]}...")
    
    # 6. Track and resolve errors
    print("\n❌ Tracking project errors:")
    
    error_manager = ErrorManager()
    
    error1 = await error_manager.create_error(
        title="Database connection timeout",
        description="Connection to analytics DB fails after 30 seconds",
        severity=ErrorSeverity.HIGH,
        category=ErrorCategory.Network,
        source="backend-service",
        context="Occurs during peak load hours",
        tags=["database", "performance", "urgent"]
    )
    
    print(f"📊 Created error record: {error1.title}")
    
    # 7. Generate project statistics
    print("\n📊 Project Statistics:")
    
    all_tasks = storage.list_tasks_across_projects()
    stats = storage.get_project_stats("web-dashboard")
    
    print(f"Total tasks: {stats.total_tasks}")
    print(f"Active tasks: {stats.active_tasks}")
    print(f"Completed tasks: {stats.completed_tasks}")
    
    # Show task breakdown
    ai_tasks = [t for t in all_tasks if t.assignee and t.assignee.kind == AssigneeType.Ai]
    human_tasks = [t for t in all_tasks if t.assignee and t.assignee.kind == AssigneeType.Human]
    
    print(f"\nTask Distribution:")
    print(f"AI-assigned: {len(ai_tasks)}")
    print(f"Human-assigned: {len(human_tasks)}")
    print(f"Unassigned: {len(all_tasks) - len(ai_tasks) - len(human_tasks)}")
    
    # 8. Semantic search for related tasks
    print("\n🔍 Finding similar tasks using semantic search:")
    
    search_results = await storage.search_tasks_semantic("database design", 5)
    print(f"Found {len(search_results)} similar tasks:")
    for result in search_results[:3]:
        print(f"  - {result.task.action} (Score: {result.score:.2f})")
    
    # 9. Progress tracking
    print("\n📈 Updating task progress:")
    
    # Complete the UI mockups task
    ui_task = None
    for task in all_tasks:
        if "mockup" in task.action.lower():
            ui_task = task
            break
    
    if ui_task:
        from todozi.models import TaskUpdate
        update = TaskUpdate().with_status(Status.Done).with_progress(100)
        await storage.update_task_in_project(ui_task.id, update)
        print(f"✅ Completed: {ui_task.action}")
    
    # 10. Generate summary report
    print("\n📄 Project Summary Report:")
    print("=" * 50)
    
    final_stats = storage.get_project_stats("web-dashboard")
    completion_rate = (final_stats.completed_tasks / final_stats.total_tasks * 100) if final_stats.total_tasks > 0 else 0
    
    print(f"Project: web-dashboard")
    print(f"Completion Rate: {completion_rate:.1f}%")
    print(f"Total Tasks: {final_stats.total_tasks}")
    print(f"Active: {final_stats.active_tasks}")
    print(f"Completed: {final_stats.completed_tasks}")
    print(f"Archived: {final_stats.archived_tasks}")
    
    # Show agent utilization
    agent_stats = agent_manager.get_agent_statistics()
    print(f"\nAgent Utilization:")
    print(f"Available agents: {agent_stats.available_agents}")
    print(f"Total assignments: {agent_stats.total_assignments}")
    print(f"Completed assignments: {agent_stats.completed_assignments}")
    
    print("\n🎉 Web Dashboard Project Management Demo Complete!")

# Run the example
if __name__ == "__main__":
    asyncio.run(manage_web_project())