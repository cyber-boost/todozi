#!/usr/bin/env python3
"""
Example 5: Complete Project Task Workflow
Demonstrates project-based storage, caching, error handling, and agent assignment.
"""

import asyncio
from datetime import datetime, timezone
from pathlib import Path
import json

# Import from the storage module
from todozi.storage import Storage, ProjectTaskContainer, Task, TaskUpdate, Priority, Status, Assignee
from todozi.error import TodoziError
from todozi.models import Project

async def demonstrate_project_workflow():
    """Complete workflow showing project-based task management with caching."""
    
    print("🚀 Example 5: Complete Project Task Workflow")
    print("=" * 50)
    
    try:
        # Initialize storage
        storage = await Storage.new()
        
        # 1. Create a new project
        project_name = "customer-portal-v2"
        print(f"📁 Creating project: {project_name}")
        storage.create_project(project_name, "Customer portal rewrite with React")
        
        # 2. Add multiple tasks to the project
        tasks_data = [
            ("Design database schema", "2 days", Priority.High, "Database design and relationships"),
            ("Implement authentication API", "3 days", Priority.Critical, "OAuth2 flow with JWT"),
            ("Create React components", "5 days", Priority.Medium, "UI components and state management"),
            ("Write unit tests", "2 days", Priority.Medium, "Test coverage for critical paths"),
            ("Set up CI/CD pipeline", "1 day", Priority.Low, "Automated testing and deployment"),
        ]
        
        for action, time_est, priority, context in tasks_data:
            task = Task(
                action=action,
                context_notes=context,
                priority=priority,
                status=Status.Todo,
                parent_project=project_name,
            )
            print(f"📋 Adding task: {action} ({priority.name})")
            await storage.add_task_to_project(task)
        
        # 3. Show project stats
        stats = storage.get_project_stats(project_name)
        print(f"\n📊 Project Statistics for {project_name}:")
        print(f"   Total tasks: {stats.total_tasks}")
        print(f"   Active tasks: {stats.active_tasks}")
        print(f"   Completed tasks: {stats.completed_tasks}")
        
        # 4. Demonstrate task retrieval and updates
        print(f"\n🔄 Demonstrating task operations:")
        
        # Get first task and update it
        project_tasks = storage.list_tasks_in_project(project_name, storage.TaskFilters.default())
        if project_tasks:
            first_task = project_tasks[0]
            print(f"   Original task: {first_task.action} - {first_task.status.name}")
            
            # Update task status to InProgress
            updates = TaskUpdate(status=Status.InProgress)
            await storage.update_task_in_project(first_task.id, updates)
            
            # Verify update
            updated_task = storage.get_task_from_project(project_name, first_task.id)
            print(f"   Updated task: {updated_task.action} - {updated_task.status.name}")
        
        # 5. Demonstrate caching by listing projects multiple times
        print(f"\n💾 Demonstrating LRU caching:")
        for i in range(3):
            projects = storage.list_projects()
            print(f"   List #{i+1}: Found {len(projects)} projects")
        
        # 6. Complete a task
        if project_tasks:
            task_to_complete = project_tasks[1]  # Second task
            storage.complete_task_in_project(task_to_complete.id)
            completed_task = storage.get_task_from_project(project_name, task_to_complete.id)
            print(f"\n✅ Completed task: {completed_task.action} - {completed_task.status.name}")
        
        # 7. Demonstrate search across all projects
        search_query = "React"
        print(f"\n🔍 Searching for '{search_query}' across all projects:")
        search_results = storage.search_tasks(search_query)
        for task in search_results:
            print(f"   Found: {task.action} in {task.parent_project}")
        
        # 8. Show updated project stats
        updated_stats = storage.get_project_stats(project_name)
        print(f"\n📈 Updated Project Stats:")
        print(f"   Total tasks: {updated_stats.total_tasks}")
        print(f"   Active tasks: {updated_stats.active_tasks}")
        print(f"   Completed tasks: {updated_stats.completed_tasks}")
        
        # 9. Demonstrate error handling
        print(f"\n⚠️  Demonstrating error handling:")
        try:
            # Try to get a non-existent task
            storage.get_task_from_project(project_name, "nonexistent-task-id")
        except TodoziError as e:
            print(f"   Error correctly caught: {e.message}")
        
        # 10. Cleanup (comment out to preserve data)
        # storage.delete_project(project_name)
        # print(f"🧹 Cleaned up project: {project_name}")
        
    except Exception as e:
        print(f"❌ Error in workflow: {e}")
        return False
    
    print("\n🎉 Example completed successfully!")
    return True

async def demonstrate_agent_assignment():
    """Show how to assign tasks to specialized agents."""
    print(f"\n🤖 Agent Assignment Demonstration")
    print("=" * 40)
    
    try:
        storage = await Storage.new()
        
        # Get available agents
        agents = storage.get_available_agents()
        print(f"Available agents: {len(agents)}")
        
        # Create a coding task and assign to coder agent
        coding_task = Task(
            action="Implement payment gateway integration",
            context_notes="Stripe API integration with error handling",
            priority=Priority.High,
            status=Status.Todo,
            parent_project="ecommerce-platform",
            assignee=Assignee.Ai  # Could be specific agent like Assignee.agent("coder")
        )
        
        print(f"💻 Coding task assigned to: {coding_task.assignee}")
        await storage.add_task_to_project(coding_task)
        
        # Show AI-specific tasks
        ai_tasks = storage.get_ai_tasks()
        print(f"🤖 AI-assigned tasks: {len(ai_tasks)}")
        
    except Exception as e:
        print(f"❌ Error in agent demonstration: {e}")

async def main():
    """Run the complete demonstration."""
    
    # Run project workflow
    success = await demonstrate_project_workflow()
    
    if success:
        # Run agent assignment demo
        await demonstrate_agent_assignment()
        
        print(f"\n📋 Summary:")
        print(f"✅ Project-based storage with caching")
        print(f"✅ LRU caching for performance")
        print(f"✅ Consistent error handling")
        print(f"✅ Agent assignment system")
        print(f"✅ Status-based task organization")
        
    return success

if __name__ == "__main__":
    # Run the demonstration
    result = asyncio.run(main())
    
    if result:
        print(f"\n🏁 Example 5 completed successfully!")
    else:
        print(f"\n💥 Example 5 encountered errors!")