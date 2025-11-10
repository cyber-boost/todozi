import asyncio
from pathlib import Path
import sys

# Add the parent directory to the path to import Todozi modules
parent_dir = Path(__file__).parent.parent
if str(parent_dir) not in sys.path:
    sys.path.insert(0, str(parent_dir))

from cli import TodoziHandler
from storage import Storage
from models import (
    Task, Priority, Status, QueueItem, Memory, Idea, Agent,
    MemoryImportance, MemoryType, IdeaImportance, ShareLevel
)
from types import (
    PlanQueue, ListQueue, BacklogQueue, ActiveQueue, StartQueue,
    CreateMemory, CreateIdea, CreateAgent, ListAgents
)
from error import TodoziError


async def advanced_todozi_example():
    """Demonstrate advanced Todozi features"""
    
    # Initialize storage and handler
    storage = await Storage.new()
    handler = TodoziHandler(storage)
    
    print("=== Advanced Todozi Operations Example ===\n")
    
    # 1. Create tasks with dependencies and tags
    print("1. Creating tasks with dependencies and tags:")
    
    # Main project task
    await handler.handle_add_command(AddTask(
        action="Build REST API server",
        time="2 days",
        priority=Priority.Critical,
        project="api-project",
        status=Status.Todo,
        tags=["backend", "api", "server"],
        context="Build a scalable REST API with authentication"
    ))
    
    # Dependent task
    await handler.handle_add_command(AddTask(
        action="Design API endpoints",
        time="4 hours",
        priority=Priority.High,
        project="api-project",
        status=Status.Todo,
        tags=["design", "api"],
        dependencies=["Build REST API server"],  # Depends on main task
        context="Design all API endpoints including CRUD operations"
    ))
    
    # Another dependent task
    await handler.handle_add_command(AddTask(
        action="Set up CI/CD pipeline",
        time="1 day",
        priority=Priority.Medium,
        project="api-project",
        status=Status.Todo,
        tags=["devops", "ci-cd"],
        dependencies=["Build REST API server"],
        context="Configure automated testing and deployment"
    ))
    
    # List tasks in the project
    print("\n2. Tasks in 'api-project':")
    await handler.handle_list_command(ListTasks(project="api-project"))
    
    # 3. Queue management - Plan and execute queue items
    print("\n3. Queue Management:")
    
    # Plan queue items
    await handler.handle_queue_command(PlanQueue(
        task_name="Database schema design",
        task_description="Design PostgreSQL schema for user management",
        priority=Priority.High,
        project_id="api-project"
    ))
    
    await handler.handle_queue_command(PlanQueue(
        task_name="API documentation",
        task_description="Write comprehensive API documentation",
        priority=Priority.Medium,
        project_id="api-project"
    ))
    
    # List backlog items
    print("\n4. Queue backlog items:")
    await handler.handle_queue_command(BacklogQueue())
    
    # Start working on a queue item
    print("\n5. Starting queue session:")
    queue_items = storage.list_backlog_items()
    if queue_items:
        first_item = queue_items[0]
        session_id = storage.start_queue_session(first_item.id)
        print(f"Started session {session_id} for item: {first_item.task_name}")
        
        # End the session
        storage.end_queue_session(session_id)
        print(f"Ended session {session_id}")
    
    # 6. Memory management
    print("\n6. Memory Management:")
    
    # Create a technical memory
    await handler.handle_memory_command(CreateMemory(
        moment="Decided to use PostgreSQL",
        meaning="PostgreSQL provides better ACID compliance and JSON support",
        reason="Database choice for the project",
        importance=MemoryImportance.High,
        term="long",
        memory_type="standard",
        tags=["database", "decision"]
    ))
    
    # Create an emotional memory
    await handler.handle_memory_command(CreateMemory(
        moment="Feeling excited about API design",
        meaning="The API design is taking shape nicely",
        reason="Positive progress on architecture",
        importance=MemoryImportance.Medium,
        emotion="excited",
        tags=["emotion", "progress"]
    ))
    
    # List memories
    print("\n7. All memories:")
    await handler.handle_memory_command(ListMemories())
    
    # 8. Idea management
    print("\n8. Idea Management:")
    
    # Create and share ideas
    await handler.handle_idea_command(CreateIdea(
        idea="Implement GraphQL endpoint alongside REST",
        share=ShareLevel.Team,
        importance=IdeaImportance.High,
        tags=["graphql", "api-enhancement"],
        context="Consider GraphQL for flexible queries"
    ))
    
    await handler.handle_idea_command(CreateIdea(
        idea="Use Redis for caching frequently accessed data",
        share=ShareLevel.Private,
        importance=IdeaImportance.Medium,
        tags=["performance", "caching"]
    ))
    
    # List all ideas
    print("\n9. All ideas:")
    await handler.handle_idea_command(ListIdeas())
    
    # 9. Agent operations
    print("\n10. Agent Operations:")
    
    # Create a specialized API development agent
    await handler.handle_agent_command(CreateAgent(
        id="api-specialist",
        name="API Specialist",
        description="Specializes in REST API design and implementation",
        category="backend",
        capabilities=["api-design", "database", "authentication", "documentation"],
        specializations=["rest", "postgresql", "oauth2"],
        model_provider="openai",
        model_name="gpt-4",
        temperature=0.3,
        max_tokens=2048,
        tools=["database", "api", "documentation"]
    ))
    
    # List all agents
    print("\n11. All available agents:")
    await handler.handle_agent_command(ListAgents())
    
    # 10. Show statistics
    print("\n12. System Statistics:")
    await handler.handle_stats_command(Stats())
    
    # 11. Search functionality
    print("\n13. Search for 'api' related tasks:")
    await handler.handle_search_command(SearchTasks(query="api"))
    
    # 12. Task completion workflow
    print("\n14. Task Completion Workflow:")
    
    # Get all tasks
    tasks = storage.list_tasks_across_projects()
    if tasks:
        # Find and complete a task
        api_task = next((t for t in tasks if "API" in t.action), None)
        if api_task:
            print(f"Completing task: {api_task.action}")
            handler.complete_task(api_task.id)
            
            # Show task details
            print("\n15. Completed task details:")
            await handler.handle_show_command(ShowTask(id=api_task.id))
    
    # 13. Error tracking
    print("\n16. Error Tracking:")
    
    # Create and track an error
    from types import CreateError
    await handler.handle_error_command(CreateError(
        title="Database connection failed",
        description="Unable to establish connection to PostgreSQL server",
        severity="high",
        category="database",
        source="api-server",
        context="Connection attempt during server startup",
        tags=["database", "connection", "critical"]
    ))
    
    # List unresolved errors
    from types import ListErrors
    await handler.handle_error_command(ListErrors(unresolved_only=True))
    
    print("\n=== Example completed successfully! ===")


if __name__ == "__main__":
    try:
        asyncio.run(advanced_todozi_example())
    except TodoziError as e:
        print(f"Todozi Error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")
        import traceback
        traceback.print_exc()