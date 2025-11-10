import asyncio
from datetime import datetime, timezone
from todozi.storage import Storage
from todozi.models import Task, Priority, Status, Assignee, TaskFilters, Memory, MemoryImportance, MemoryType, Idea, ShareLevel
from todozi.api import create_api_key, create_api_key_with_user_id, list_api_keys
from todozi.agent import AgentManager, create_planner_agent, create_coder_agent, create_tester_agent
from todozi.search import SearchEngine, SearchOptions
from todozi.error import ErrorManager, ErrorSeverity, ErrorCategory

async def main():
    # Initialize storage and managers
    storage = await Storage.new()
    agent_manager = AgentManager()
    await agent_manager.load_agents()
    search_engine = SearchEngine()
    error_manager = ErrorManager()
    
    # Create API keys for team members
    print("=== Setting up API Keys ===")
    dev_key = create_api_key_with_user_id("developer_001")
    pm_key = create_api_key_with_user_id("project_manager_001")
    qa_key = create_api_key_with_user_id("qa_engineer_001")
    
    print(f"✅ Created API keys for team members")
    
    # Create project
    print("\n=== Creating Project ===")
    storage.create_project(
        name="ecommerce_platform",
        description="Multi-tenant e-commerce platform with microservices architecture"
    )
    
    # Create specialized agents for different roles
    print("\n=== Setting up Specialized Agents ===")
    
    # Frontend developer agent
    frontend_agent = create_coder_agent()
    frontend_agent.id = "frontend_dev_001"
    frontend_agent.name = "Frontend Developer"
    frontend_agent.description = "Specialized in React, TypeScript, and responsive design"
    frontend_agent.specializations = ["react", "typescript", "css", "frontend"]
    await agent_manager.create_agent(frontend_agent)
    
    # Backend developer agent
    backend_agent = create_coder_agent()
    backend_agent.id = "backend_dev_001"
    backend_agent.name = "Backend Developer"
    backend_agent.description = "Specialized in Python, Django, and REST APIs"
    backend_agent.specializations = ["python", "django", "api", "database"]
    await agent_manager.create_agent(backend_agent)
    
    # QA agent
    qa_agent = create_tester_agent()
    qa_agent.id = "qa_engineer_001"
    qa_agent.name = "QA Engineer"
    qa_agent.description = "Specialized in automated testing and quality assurance"
    qa_agent.specializations = ["testing", "automation", "quality", "selenium"]
    await agent_manager.create_agent(qa_agent)
    
    # Project manager agent
    planner_agent = create_planner_agent()
    planner_agent.id = "project_manager_001"
    planner_agent.name = "Project Manager"
    planner_agent.description = "Manages project timeline, resources, and deliverables"
    await agent_manager.create_agent(planner_agent)
    
    print(f"✅ Created 4 specialized agents")
    
    # Create tasks for the project
    print("\n=== Creating Project Tasks ===")
    
    # High-priority backend tasks
    backend_task = Task.new_full(
        user_id="developer_001",
        action="Implement user authentication service with JWT",
        time="3 days",
        priority=Priority.High,
        parent_project="ecommerce_platform",
        status=Status.Todo,
        assignee=Assignee.agent("backend_dev_001"),
        tags=["backend", "security", "authentication"],
        dependencies=[],
        context_notes="Use Django REST framework with JWT token-based auth"
    )
    
    database_task = Task.new_full(
        user_id="developer_001",
        action="Design and implement PostgreSQL database schema",
        time="2 days",
        priority=Priority.High,
        parent_project="ecommerce_platform",
        status=Status.Todo,
        assignee=Assignee.agent("backend_dev_001"),
        tags=["backend", "database", "design"],
        dependencies=[],
        context_notes="Design schemas for users, products, orders, and payments"
    )
    
    # Frontend tasks
    frontend_task = Task.new_full(
        user_id="developer_001",
        action="Create responsive product listing page",
        time="2 days",
        priority=Priority.Medium,
        parent_project="ecommerce_platform",
        status=Status.Todo,
        assignee=Assignee.agent("frontend_dev_001"),
        tags=["frontend", "ui", "responsive"],
        dependencies=[],
        context_notes="Use React with TypeScript and Tailwind CSS"
    )
    
    # QA tasks
    qa_task = Task.new_full(
        user_id="qa_engineer_001",
        action="Set up automated testing framework",
        time="1 day",
        priority=Priority.High,
        parent_project="ecommerce_platform",
        status=Status.Todo,
        assignee=Assignee.agent("qa_engineer_001"),
        tags=["testing", "automation", "setup"],
        dependencies=[],
        context_notes="Configure pytest and Selenium for web testing"
    )
    
    # Save tasks to storage
    await storage.add_task_to_project(backend_task)
    await storage.add_task_to_project(database_task)
    await storage.add_task_to_project(frontend_task)
    await storage.add_task_to_project(qa_task)
    
    print(f"✅ Created 4 project tasks")
    
    # Track important decisions as memories
    print("\n=== Recording Project Memories ===")
    
    from todozi.memory import MemoryManager
    memory_manager = MemoryManager()
    await memory_manager.load_memories()
    
    # Technical decision memory
    tech_decision = Memory(
        user_id="project_manager_001",
        project_id="ecommerce_platform",
        status=Status.Active,
        moment="Technology Stack Decision",
        meaning="Chosen tech stack: React frontend, Django backend, PostgreSQL database",
        reason="Balanced approach with proven technologies, good for rapid development and scalability",
        importance=MemoryImportance.High,
        term=MemoryTerm.Long,
        memory_type=MemoryType.Standard,
        tags=["technology", "architecture", "decision"]
    )
    await memory_manager.create_memory(tech_decision)
    
    # Meeting memory
    meeting_memory = Memory(
        user_id="project_manager_001",
        project_id="ecommerce_platform",
        status=Status.Active,
        moment="Initial Project Planning Meeting",
        meaning="Team agreed on 2-week sprints with daily standups at 9 AM",
        reason="Establish clear communication rhythm and deliverable cadence",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type=MemoryType.Standard,
        tags=["meeting", "planning", "agile"]
    )
    await memory_manager.create_memory(meeting_memory)
    
    # Risk memory
    risk_memory = Memory(
        user_id="project_manager_001",
        project_id="ecommerce_platform",
        status=Status.Active,
        moment="Risk Assessment",
        meaning="Identified key risks: team availability, third-party payment gateway integration",
        reason="Proactive risk management to prevent project delays",
        importance=MemoryImportance.High,
        term=MemoryTerm.Long,
        memory_type=MemoryType.Standard,
        tags=["risk", "planning", "mitigation"]
    )
    await memory_manager.create_memory(risk_memory)
    
    print(f"✅ Recorded 3 project memories")
    
    # Capture ideas for future consideration
    print("\n=== Capturing Project Ideas ===")
    
    from todozi.idea import IdeaManager
    idea_manager = IdeaManager()
    await idea_manager.load_ideas()
    
    # Feature idea
    feature_idea = Idea(
        idea="Implement real-time inventory updates using WebSockets",
        project_id="ecommerce_platform",
        share=ShareLevel.Team,
        importance=IdeaImportance.Medium,
        tags=["feature", "websocket", "real-time"],
        context="Would improve user experience significantly"
    )
    await idea_manager.create_idea(feature_idea)
    
    # Optimization idea
    optimization_idea = Idea(
        idea="Use Redis for session management and caching",
        project_id="ecommerce_platform",
        share=ShareLevel.Team,
        importance=IdeaImportance.High,
        tags=["optimization", "performance", "redis"],
        context="Could reduce database load by 40%"
    )
    await idea_manager.create_idea(optimization_idea)
    
    print(f"✅ Captured 2 project ideas")
    
    # Track an error that occurred
    print("\n=== Tracking Development Error ===")
    
    integration_error = Error.new(
        title="Payment Gateway API Integration Error",
        description="Stripe API returning 401 unauthorized in development environment",
        severity=ErrorSeverity.High,
        category=ErrorCategory.Network,
        source="backend_dev_001",
        context="Error occurred during payment service implementation",
        tags=["payment", "stripe", "api", "authentication"]
    )
    error_manager.save_error(integration_error)
    
    print(f"✅ Tracked payment integration error")
    
    # Search for related tasks
    print("\n=== Semantic Search for Related Tasks ===")
    
    # Update search index with current content
    all_tasks = storage.list_tasks_across_projects(TaskFilters())
    all_memories = memory_manager.get_all_memories()
    all_ideas = idea_manager.get_all_ideas()
    all_errors = error_manager.list_errors()
    
    search_content = ChatContent(
        tasks=all_tasks,
        memories=all_memories,
        ideas=all_ideas,
        errors=all_errors
    )
    search_engine.update_index(search_content)
    
    # Search for authentication-related content
    auth_results = search_engine.search("authentication", SearchOptions(limit=10))
    
    print(f"Found {len(auth_results.task_results)} tasks related to authentication:")
    for task_result in auth_results.task_results[:3]:
        print(f"  - {task_result.task.action} (Priority: {task_result.task.priority})")
    
    print(f"\nFound {len(auth_results.memory_results)} memories related to authentication:")
    for memory_result in auth_results.memory_results[:2]:
        print(f"  - {memory_result.memory.moment}")
    
    # Show project statistics
    print("\n=== Project Statistics ===")
    
    project_stats = storage.get_project_stats("ecommerce_platform")
    print(f"Project: ecommerce_platform")
    print(f"  Total tasks: {project_stats.total_tasks}")
    print(f"  Active tasks: {project_stats.active_tasks}")
    print(f"  Completed tasks: {project_stats.completed_tasks}")
    
    # Task breakdown by priority
    all_project_tasks = storage.get_project_tasks("ecommerce_platform")
    priority_counts = {}
    for task in all_project_tasks:
        priority_name = task.priority.name
        priority_counts[priority_name] = priority_counts.get(priority_name, 0) + 1
    
    print("\nTasks by Priority:")
    for priority, count in priority_counts.items():
        print(f"  {priority}: {count}")
    
    # Show agent assignments
    print("\n=== Agent Assignments ===")
    
    agent_tasks = {}
    for task in all_project_tasks:
        if task.assignee and task.assignee.kind == AssigneeType.Agent:
            agent_name = task.assignee.name or "Unknown Agent"
            if agent_name not in agent_tasks:
                agent_tasks[agent_name] = []
            agent_tasks[agent_name].append(task)
    
    for agent_name, tasks in agent_tasks.items():
        print(f"\n{agent_name} assigned to {len(tasks)} tasks:")
        for task in tasks:
            print(f"  - {task.action} (Status: {task.status.name})")
    
    # Generate project summary
    print("\n=== Project Summary ===")
    
    print(f"""
E-commerce Platform Project Summary:
=====================================
Status: Active Development
Team Size: 4 specialized agents
Timeline: 2-week sprints

Key Components:
- Backend: Django REST API with PostgreSQL
- Frontend: React with TypeScript
- Authentication: JWT-based auth system
- Testing: Automated testing framework

Current Sprint:
- {project_stats.active_tasks} active tasks
- {len(agent_tasks)} agent assignments
- {len(all_memories)} documented decisions
- {len(all_ideas)} improvement ideas

Next Steps:
1. Complete authentication service
2. Set up database schema
3. Implement product listing page
4. Configure testing framework

Risks to Monitor:
- API integration issues (1 error tracked)
- Third-party dependencies
- Timeline constraints
""")
    
    # Demonstrate API key usage
    print("\n=== API Key Usage Example ===")
    
    api_keys = list_api_keys()
    print(f"Total API keys created: {len(api_keys)}")
    
    # List all tasks with filters
    print("\n=== Filtered Task List ===")
    
    high_priority_filters = TaskFilters(
        project="ecommerce_platform",
        priority=Priority.High
    )
    high_priority_tasks = storage.list_tasks_across_projects(high_priority_filters)
    
    print(f"High Priority Tasks ({len(high_priority_tasks)}):")
    for task in high_priority_tasks:
        assignee_info = f" ({task.assignee.name})" if task.assignee and task.assignee.kind == AssigneeType.Agent else ""
        print(f"  - {task.action}{assignee_info} - {task.time}")
    
    print("\n✅ Todozi workflow demonstration complete!")

if __name__ == "__main__":
    asyncio.run(main())