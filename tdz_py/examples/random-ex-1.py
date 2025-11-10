"""
Example 2: Todozi Project Management with Agents and Semantic Search

This example demonstrates:
1. Setting up a project with agents
2. Creating tasks with different priorities
3. Using semantic search to find similar tasks
4. Managing memories and ideas
5. Using agent assignments for task completion
"""

import asyncio
from datetime import datetime, timezone
from pathlib import Path

# Import all necessary components from the todozi package
from todozi import (
    # Core components
    Storage, Task, Priority, Status, Assignee,
    Project, QueueItem, QueueStatus,
    
    # Agent system
    Agent, AgentManager, AgentAssignment, AgentStatus,
    
    # Search and embedding
    SearchEngine, SearchOptions, SearchResults,
    TodoziEmbeddingService, TodoziEmbeddingConfig,
    
    # Memory and ideas
    Memory, MemoryManager, MemoryImportance, MemoryType,
    Idea, IdeaManager, IdeaImportance, ShareLevel,
    
    # Error handling
    TodoziError, TaskNotFoundError,
    
    # Models
    TaskFilters, TaskUpdate,
)

async def setup_project_example():
    """Initialize a sample project with agents and tasks"""
    print("🚀 Initializing Todozi Project Example\n")
    
    # Initialize storage
    storage = await Storage.new()
    
    # Create a new project
    project_name = "AI-Assistant Development"
    project = Project(
        name=project_name,
        description="Building an AI-powered coding assistant",
    )
    storage.create_project(project.name, project.description)
    print(f"✅ Created project: {project_name}")
    
    # Initialize agent manager
    agent_manager = AgentManager()
    await agent_manager.load_agents()
    
    # Create specialized agents for the project
    frontend_agent = Agent(
        id="frontend_dev",
        name="Frontend Developer",
        description="Specializes in React/TypeScript development",
        capabilities=["frontend", "react", "typescript", "ui"],
        specializations=["web_development", "user_interfaces"],
    )
    
    backend_agent = Agent(
        id="backend_dev",
        name="Backend Developer",
        description="Specializes in Python/FastAPI development",
        capabilities=["backend", "python", "fastapi", "api"],
        specializations=["server_development", "rest_apis"],
    )
    
    qa_agent = Agent(
        id="qa_specialist",
        name="QA Specialist",
        description="Focuses on testing and quality assurance",
        capabilities=["testing", "qa", "automation"],
        specializations=["quality_assurance", "test_automation"],
    )
    
    # Save agents
    for agent in [frontend_agent, backend_agent, qa_agent]:
        await agent_manager.create_agent(agent)
    print(f"✅ Created {len(agent_manager.get_all_agents())} agents")
    
    # Create tasks with different priorities and assignees
    tasks = [
        Task(
            action="Design API endpoints for user authentication",
            time="4 hours",
            priority=Priority.High,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.backend(),
            tags=["api", "auth", "backend"],
        ),
        Task(
            action="Implement login form with React hooks",
            time="3 hours",
            priority=Priority.High,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.frontend(),
            tags=["react", "frontend", "auth"],
        ),
        Task(
            action="Write integration tests for authentication flow",
            time="2 hours",
            priority=Priority.Medium,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.ai(),
            tags=["testing", "integration", "qa"],
        ),
        Task(
            action="Set up CI/CD pipeline with GitHub Actions",
            time="2 hours",
            priority=Priority.Medium,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.collaborative(),
            tags=["devops", "cicd", "automation"],
        ),
        Task(
            action="Create user dashboard components",
            time="5 hours",
            priority=Priority.Medium,
            parent_project=project_name,
            status=Status.InProgress,
            assignee=Assignee.frontend(),
            tags=["dashboard", "frontend", "react"],
        ),
        Task(
            action="Optimize database queries for user data",
            time="3 hours",
            priority=Priority.Low,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.backend(),
            tags=["database", "optimization", "backend"],
        ),
    ]
    
    # Save tasks to storage
    for task in tasks:
        await storage.add_task_to_project(task)
    print(f"✅ Created {len(tasks)} tasks")
    
    return storage, agent_manager, project, tasks

async def semantic_search_example(storage: Storage):
    """Demonstrate semantic search capabilities"""
    print("\n🔍 Semantic Search Example\n")
    
    # Initialize embedding service
    config = TodoziEmbeddingConfig(
        model_name="sentence-transformers/all-MiniLM-L6-v2"
    )
    embedding_service = TodoziEmbeddingService(config)
    await embedding_service.initialize()
    
    # Add existing tasks to embedding index
    for task in storage.list_tasks_across_projects(TaskFilters()):
        await embedding_service.add_task(task)
    
    # Perform semantic searches
    search_queries = [
        "authentication and login",
        "database performance",
        "user interface design",
        "testing and quality",
    ]
    
    for query in search_queries:
        print(f"\n📝 Searching for: '{query}'")
        results = await embedding_service.semantic_search(query, None, 3)
        
        if results:
            print(f"Found {len(results)} similar tasks:")
            for result in results:
                task = storage.get_task_from_any_project(result.content_id)
                print(f"  📋 {task.action} (similarity: {result.similarity_score:.3f})")
        else:
            print("  No similar tasks found")

async def memory_and_ideas_example():
    """Demonstrate memory and idea management"""
    print("\n💭 Memory and Ideas Example\n")
    
    # Initialize managers
    memory_manager = MemoryManager()
    idea_manager = IdeaManager()
    await memory_manager.load_memories()
    await idea_manager.load_ideas()
    
    # Create memories related to the project
    memories = [
        Memory(
            user_id="system",
            project_id="AI-Assistant Development",
            moment="Users expect fast login with social providers",
            meaning="Consider OAuth integration for Google/GitHub",
            reason="User feedback from MVP testing",
            importance=MemoryImportance.High,
            term=MemoryTerm.Long,
            memory_type=MemoryType.Human,
            tags=["ux", "feedback", "mvp"],
        ),
        Memory(
            user_id="system",
            project_id="AI-Assistant Development",
            moment="React 18 introduced concurrent features",
            meaning="Need to update React expertise and patterns",
            reason="Technology stack evaluation",
            importance=MemoryImportance.Medium,
            term=MemoryTerm.Short,
            memory_type=MemoryType.Standard,
            tags=["react", "technology", "learning"],
        ),
        Memory(
            user_id="system",
            project_id="AI-Assistant Development",
            moment="Testing revealed race conditions in auth flow",
            meaning="Implement proper error handling and retries",
            reason="Bug analysis",
            importance=MemoryImportance.High,
            term=MemoryTerm.Short,
            memory_type=MemoryType.Emotional,
            emotion="frustrated",
            tags=["bug", "testing", "critical"],
        ),
    ]
    
    # Create innovative ideas
    ideas = [
        Idea(
            idea="Add real-time collaboration features for multiple users",
            project_id="AI-Assistant Development",
            share=ShareLevel.Team,
            importance=IdeaImportance.High,
            tags=["collaboration", "feature", "real-time"],
        ),
        Idea(
            idea="Implement AI-powered code completion for our project",
            project_id="AI-Assistant Development",
            share=ShareLevel.Public,
            importance=IdeaImportance.Medium,
            tags=["ai", "productivity", "code-completion"],
        ),
        Idea(
            idea="Create a plugin system for custom integrations",
            project_id="AI-Assistant Development",
            share=ShareLevel.Private,
            importance=IdeaImportance.Low,
            tags=["plugins", "extensibility", "architecture"],
        ),
    ]
    
    # Save memories and ideas
    for memory in memories:
        await memory_manager.create_memory(memory)
    for idea in ideas:
        await idea_manager.create_idea(idea)
    
    print(f"✅ Created {len(memories)} memories")
    print(f"✅ Created {len(ideas)} ideas")
    
    # Display high-importance items
    print("\n🌟 High Priority Memories:")
    for mem in memory_manager.get_all_memories():
        if mem.importance == MemoryImportance.High:
            print(f"  💡 {mem.moment}: {mem.meaning}")
    
    print("\n💡 High Importance Ideas:")
    for idea in idea_manager.get_all_ideas():
        if idea.importance == IdeaImportance.High:
            print(f"  💡 {idea.idea}")

async def agent_assignment_example(agent_manager: AgentManager, tasks: list[Task]):
    """Demonstrate agent task assignment"""
    print("\n🤖 Agent Assignment Example\n")
    
    # Get available agents
    available_agents = agent_manager.get_available_agents()
    print(f"Available agents: {[a.name for a in available_agents]}")
    
    # Assign tasks to agents
    assignments = []
    for task in tasks:
        if task.status == Status.Todo and task.assignee:
            # Find suitable agent
            suitable_agent = None
            if task.assignee.kind == AssigneeType.Ai:
                suitable_agent = next(
                    (a for a in available_agents 
                     if "backend" in a.capabilities.lower() and "api" in task.tags), 
                    None
                )
            elif task.assignee.kind == AssigneeType.Human:
                if "frontend" in task.tags:
                    suitable_agent = next(
                        (a for a in available_agents 
                         if a.name == "Frontend Developer"), 
                        None
                    )
                elif "backend" in task.tags:
                    suitable_agent = next(
                        (a for a in available_agents 
                         if a.name == "Backend Developer"), 
                        None
                    )
            
            if suitable_agent:
                assignment = AgentAssignment(
                    agent_id=suitable_agent.id,
                    task_id=task.id,
                    project_id=task.parent_project,
                )
                assignments.append(assignment)
                print(f"  🔗 Assigned '{task.action}' to {suitable_agent.name}")
    
    return assignments

async def project_analytics(storage: Storage, project_name: str):
    """Display project analytics"""
    print(f"\n📊 Project Analytics: {project_name}\n")
    
    # Get project statistics
    stats = storage.get_project_stats(project_name)
    print(f"📈 Total Tasks: {stats.total_tasks}")
    print(f"✅ Active Tasks: {stats.active_tasks}")
    print(f"✅ Completed Tasks: {stats.completed_tasks}")
    print(f"📁 Archived Tasks: {stats.archived_tasks}")
    
    # Task breakdown by priority
    all_tasks = storage.list_tasks_in_project(project_name, TaskFilters())
    priority_counts = {}
    for task in all_tasks:
        priority_str = str(task.priority)
        priority_counts[priority_str] = priority_counts.get(priority_str, 0) + 1
    
    print("\n📊 Tasks by Priority:")
    for priority, count in priority_counts.items():
        print(f"  {priority}: {count}")
    
    # Task breakdown by assignee
    assignee_counts = {}
    for task in all_tasks:
        assignee_str = str(task.assignee) if task.assignee else "unassigned"
        assignee_counts[assignee_str] = assignee_counts.get(assignee_str, 0) + 1
    
    print("\n👥 Tasks by Assignee:")
    for assignee, count in assignee_counts.items():
        print(f"  {assignee}: {count}")

async def workflow_integration_example(storage: Storage):
    """Demonstrate integrated workflow with queue and search"""
    print("\n🔄 Integrated Workflow Example\n")
    
    # Create queue items for pending tasks
    pending_tasks = storage.list_tasks_across_projects(
        TaskFilters(status=Status.Todo)
    )
    
    for task in pending_tasks[:3]:  # Take first 3 as example
        queue_item = QueueItem(
            title=f"Process: {task.action}",
            description=f"Task for {task.parent_project}",
            priority=task.priority,
            project_id=task.parent_project,
            status=QueueStatus.Backlog,
        )
        await storage.add_queue_item(queue_item)
        print(f"  📋 Queued: {task.action}")
    
    # Use search engine to find related tasks
    search_engine = SearchEngine()
    
    # Build a chat content object for indexing
    from todozi import ChatContent
    chat_content = ChatContent(
        tasks=pending_tasks,
        memories=[],
        ideas=[],
        agent_assignments=[],
        code_chunks=[],
        errors=[],
        training_data=[],
    )
    
    search_engine.update_index(chat_content)
    
    # Search for tasks related to "authentication"
    search_results = search_engine.search(
        "authentication", 
        SearchOptions(limit=5)
    )
    
    print(f"\n🔍 Found {len(search_results.task_results)} tasks related to authentication:")
    for result in search_results.task_results:
        print(f"  📋 {result.action}")

async def main():
    """Run the complete example"""
    try:
        # Setup project and initial data
        storage, agent_manager, project, tasks = await setup_project_example()
        
        # Demonstrate semantic search
        await semantic_search_example(storage)
        
        # Demonstrate memories and ideas
        await memory_and_ideas_example()
        
        # Demonstrate agent assignments
        assignments = await agent_assignment_example(agent_manager, tasks)
        
        # Show project analytics
        await project_analytics(storage, project.name)
        
        # Demonstrate integrated workflow
        await workflow_integration_example(storage)
        
        print("\n✅ Example 2 completed successfully!")
        print("\n💡 Key Features Demonstrated:")
        print("   ✓ Project and task management")
        print("   ✓ Agent system with task assignment")
        print("   ✓ Semantic search with embeddings")
        print("   ✓ Memory and idea management")
        print("   ✓ Project analytics and reporting")
        print("   ✓ Queue-based task processing")
        
    except TodoziError as e:
        print(f"\n❌ Todozi Error: {e.message}")
        return 1
    except Exception as e:
        print(f"\n❌ Unexpected Error: {e}")
        return 1
    
    return 0

if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)