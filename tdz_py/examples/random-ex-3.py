#!/usr/bin/env python3
"""
Example 5: Advanced Task Management with AI Agents and Memory Integration
Demonstrates creating complex tasks with agent assignments and memory storage.
"""

import asyncio
from datetime import datetime, timezone
from todozi import (
    TodoziHandler, Storage, Task, AgentAssignment, Memory, Idea,
    Priority, Status, MemoryImportance, MemoryTerm, MemoryType
)


async def create_enhanced_project_with_agents():
    """Create a software development project with AI agents and contextual memories"""
    
    # Initialize storage and handler
    storage = await Storage.new()
    handler = TodoziHandler(storage)
    
    print("🚀 Creating Enhanced Software Development Project")
    print("=" * 60)
    
    # 1. Create the main project
    print("📁 Creating project: 'AI-Powered Web Application'")
    await handler.handle_project_command(
        handler.types.CreateProject(
            name="ai-web-app",
            description="Build an AI-powered web application with React and FastAPI"
        )
    )
    
    # 2. Create development tasks with specific agent assignments
    development_tasks = [
        {
            "action": "Design database schema for user management",
            "time": "3 hours",
            "priority": Priority.HIGH,
            "project": "ai-web-app",
            "agent": "architect"
        },
        {
            "action": "Implement user authentication API endpoints",
            "time": "6 hours", 
            "priority": Priority.HIGH,
            "project": "ai-web-app",
            "agent": "mason"
        },
        {
            "action": "Create React frontend components for login",
            "time": "4 hours",
            "priority": Priority.MEDIUM,
            "project": "ai-web-app",
            "agent": "framer"
        },
        {
            "action": "Write comprehensive test suite",
            "time": "5 hours",
            "priority": Priority.MEDIUM,
            "project": "ai-web-app",
            "agent": "tester"
        }
    ]
    
    # 3. Create tasks and assign to appropriate agents
    task_results = []
    for task_data in development_tasks:
        print(f"\n📋 Creating task: {task_data['action']}")
        
        # Create the task
        task_result = await handler.handle_add_command(
            handler.types.AddTask(
                action=task_data["action"],
                time=task_data["time"],
                priority=task_data["priority"],
                project=task_data["project"],
                status="todo",
                assignee=f"agent={task_data['agent']}"
            )
        )
        
        # Store task ID for agent assignment
        task_id = extract_task_id_from_result(str(task_result))
        if task_id:
            task_results.append({
                "task_id": task_id,
                "agent_id": task_data["agent"],
                "project_id": task_data["project"]
            })
    
    # 4. Create formal agent assignments
    print("\n🤖 Creating AI Agent Assignments")
    print("-" * 40)
    
    for assignment_data in task_results:
        print(f"🔗 Assigning {assignment_data['agent_id']} to task {assignment_data['task_id']}")
        
        # This would typically be done through the agent assignment system
        # await handler.save_agent_assignment(assignment_data)
    
    # 5. Create project memories for context
    print("\n🧠 Storing Project Memories")
    print("-" * 40)
    
    project_memories = [
        {
            "moment": "Project kickoff meeting",
            "meaning": "Team decided to use modern tech stack with React and FastAPI",
            "importance": MemoryImportance.HIGH,
            "reason": "Provides context for technology decisions"
        },
        {
            "moment": "Database design discussion", 
            "meaning": "Chose PostgreSQL for its excellent JSON support and scalability",
            "importance": MemoryImportance.HIGH,
            "reason": "Important architectural decision"
        },
        {
            "moment": "Security considerations",
            "meaning": "Implement JWT tokens with 1-hour expiration for auth",
            "importance": MemoryImportance.CRITICAL,
            "reason": "Critical security implementation detail"
        }
    ]
    
    for memory_data in project_memories:
        print(f"💾 Storing memory: {memory_data['moment']}")
        await handler.handle_memory_command(
            handler.types.CreateMemory(
                moment=memory_data["moment"],
                meaning=memory_data["meaning"],
                reason=memory_data["reason"],
                importance=memory_data["importance"],
                memory_type="standard"
            )
        )
    
    # 6. Create development ideas
    print("\n💡 Capturing Development Ideas")
    print("-" * 40)
    
    ideas = [
        {
            "idea": "Implement AI-powered code suggestions in the IDE",
            "importance": "high",
            "share": "team"
        },
        {
            "idea": "Add real-time collaboration features using WebSockets",
            "importance": "medium", 
            "share": "public"
        }
    ]
    
    for idea_data in ideas:
        print(f"✨ Capturing idea: {idea_data['idea']}")
        await handler.handle_idea_command(
            handler.types.CreateIdea(
                idea=idea_data["idea"],
                importance=idea_data["importance"],
                share=idea_data["share"]
            )
        )
    
    # 7. Perform semantic search to show related content
    print("\n🔍 Performing Semantic Search")
    print("-" * 40)
    
    search_results = await handler.handle_search_all_command(
        handler.types.SearchAll(
            query="authentication security",
            types="tasks,memories,ideas"
        )
    )
    
    print(f"📊 Found {len(search_results.task_results)} related tasks")
    print(f"🧠 Found {len(search_results.memory_results)} related memories") 
    print(f"💡 Found {len(search_results.idea_results)} related ideas")
    
    # 8. Show project statistics
    print("\n📈 Project Statistics")
    print("-" * 40)
    
    await handler.handle_stats_command(handler.types.Stats())
    
    return {
        "tasks_created": len(development_tasks),
        "agents_assigned": len(task_results),
        "memories_stored": len(project_memories),
        "ideas_captured": len(ideas)
    }


def extract_task_id_from_result(result_str: str) -> str:
    """Extract task ID from handler result string"""
    import re
    match = re.search(r'Task created: ([a-f0-9-]+)', result_str)
    return match.group(1) if match else "unknown"


async def demonstrate_workflow_execution():
    """Demonstrate executing the created workflow"""
    
    storage = await Storage.new()
    handler = TodoziHandler(storage)
    
    print("\n🎯 Demonstrating Workflow Execution")
    print("=" * 60)
    
    # Get all tasks for the project
    tasks = storage.list_tasks_across_projects(
        handler.types.TaskFilters(project="ai-web-app")
    )
    
    print(f"📋 Found {len(tasks)} tasks in project")
    
    # Simulate AI task execution
    for i, task in enumerate(tasks[:2], 1):  # Just demonstrate with first 2 tasks
        print(f"\n{i}. Executing AI task: {task.action}")
        
        # This would typically call the AI execution system
        # result = await handler.execute_ai_task(task)
        print(f"   ✅ Task queued for AI processing")
        print(f"   ⏱️  Estimated time: {task.time}")
        print(f"   🎯 Priority: {task.priority}")
        
        # Update task status to in progress
        await handler.handle_update_command(
            id=task.id,
            status="in_progress"
        )
    
    print("\n🚀 Workflow execution demonstration completed!")


async def main():
    """Main demonstration function"""
    
    print("Todozi Example 5: Advanced Project Management with AI Agents")
    print("=" * 70)
    
    try:
        # Create the enhanced project
        results = await create_enhanced_project_with_agents()
        
        print(f"\n🎉 Project Creation Summary:")
        print(f"   • Tasks created: {results['tasks_created']}")
        print(f"   • AI agents assigned: {results['agents_assigned']}")
        print(f"   • Context memories stored: {results['memories_stored']}")
        print(f"   • Development ideas captured: {results['ideas_captured']}")
        
        # Demonstrate workflow execution
        await demonstrate_workflow_execution()
        
        print(f"\n✅ Example completed successfully!")
        print("💡 Next steps:")
        print("   - Check agent assignments: todozi agent list")
        print("   - View project tasks: todozi list --project ai-web-app")
        print("   - Search project content: todozi search-all 'authentication'")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return 1
    
    return 0


if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)