#!/usr/bin/env python3
"""
AI-Human Collaborative Bug Fixing Workflow Example
Demonstrates how to coordinate between AI analysis and human implementation
"""

import asyncio
from todozi import TodoziHandler, Storage
from todozi.models import Task, Priority, Status, Assignee
from todozi.todozi import parse_todozi_format, process_chat_message_extended


async def collaborative_bug_fix_workflow():
    """Demonstrates a collaborative workflow where AI analyzes and humans implement fixes"""
    
    # Initialize storage and handler
    storage = await Storage.new()
    handler = TodoziHandler(storage)
    
    print("🚀 Starting AI-Human Collaborative Bug Fixing Workflow")
    print("=" * 60)
    
    # Step 1: AI analyzes the bug
    ai_analysis_task = """
    <todozi>
    Analyze database connection timeout bug; 2 hours; high; database-optimization; todo;
    assignee=ai; tags=bug,analysis,database; 
    context_notes=Database connections timing out after 5 seconds in production
    </todozi>
    """
    
    # Step 2: Human implements the fix based on AI analysis
    human_implementation_task = """
    <todozi>
    Implement connection pool optimization; 4 hours; critical; database-optimization; todo;
    assignee=human; tags=implementation,database,optimization;
    dependencies=Analyze database connection timeout bug;
    context_notes=Based on AI analysis, increase pool size and timeout settings
    </todozi>
    """
    
    # Step 3: Collaborative testing
    collaborative_testing_task = """
    <todozi>
    Test connection optimization changes; 3 hours; high; database-optimization; todo;
    assignee=collaborative; tags=testing,qa,collaboration;
    dependencies=Implement connection pool optimization;
    context_notes=AI generates test cases, human performs integration testing
    </todozi>
    """
    
    # Step 4: Agent-assisted deployment
    agent_deployment_task = """
    <todozi>
    Deploy database optimization to staging; 1 hour; medium; deployment; todo;
    assignee=agent=deploy-bot; tags=deployment,staging;
    dependencies=Test connection optimization changes;
    context_notes=Automated deployment with rollback capabilities
    </todozi>
    """
    
    # Process all tasks together
    workflow_message = f"""
    {ai_analysis_task}
    {human_implementation_task}
    {collaborative_testing_task}
    {agent_deployment_task}
    
    <memory>
    standard; Database connection analysis; AI identified connection pool bottleneck; 
    Root cause analysis; high; long; database,performance,insight
    </memory>
    
    <error>
    Database timeout issue; Connections failing after 5 seconds; high; database; 
    production-db; Affecting 20% of users during peak hours; critical,urgent
    </error>
    
    <idea>
    Implement adaptive connection pooling; private; high; 
    Connection pool that adjusts size based on load patterns
    </idea>
    """
    
    print("📋 Processing collaborative workflow tasks...")
    
    # Extract and process all content types
    content = process_chat_message_extended(workflow_message, "workflow-manager")
    
    print(f"✅ Extracted workflow components:")
    print(f"   📊 Tasks: {len(content.tasks)}")
    print(f"   🧠 Memories: {len(content.memories)}")
    print(f"   ❌ Errors: {len(content.errors)}")
    print(f"   💡 Ideas: {len(content.ideas)}")
    
    # Save tasks to storage
    for task in content.tasks:
        await handler.storage.add_task_to_project(task)
        print(f"   ✅ Saved task: {task.action} (Assignee: {task.assignee})")
    
    # Execute the workflow
    print("\n🔄 Executing workflow tasks...")
    
    for task in content.tasks:
        try:
            result = await handler.execute_task(handler.storage, task)
            print(f"   ✅ {result}")
        except Exception as e:
            print(f"   ❌ Failed to execute {task.action}: {e}")
    
    # Demonstrate task dependencies
    print("\n🔗 Task Dependency Chain:")
    for i, task in enumerate(content.tasks, 1):
        dep_str = " → ".join(task.dependencies) if task.dependencies else "None"
        print(f"   {i}. {task.action}")
        print(f"      Dependencies: {dep_str}")
        print(f"      Assignee: {task.assignee}")
        print(f"      Status: {task.status}")
    
    # Show how AI and human work together
    print("\n🤖 AI-Human Collaboration Pattern:")
    ai_tasks = [t for t in content.tasks if t.assignee and t.assignee.kind.name == "Ai"]
    human_tasks = [t for t in content.tasks if t.assignee and t.assignee.kind.name == "Human"]
    collaborative_tasks = [t for t in content.tasks if t.assignee and t.assignee.kind.name == "Collaborative"]
    agent_tasks = [t for t in content.tasks if t.assignee and t.assignee.kind.name == "Agent"]
    
    print(f"   AI Analysis Tasks: {len(ai_tasks)}")
    print(f"   Human Implementation Tasks: {len(human_tasks)}")
    print(f"   Collaborative Tasks: {len(collaborative_tasks)}")
    print(f"   Agent Tasks: {len(agent_tasks)}")
    
    return content


async def monitor_workflow_progress():
    """Monitor and report on workflow progress"""
    
    storage = await Storage.new()
    
    print("\n📊 Workflow Progress Monitoring")
    print("=" * 60)
    
    # Get all active tasks
    from todozi.models import TaskFilters, Status
    active_tasks = storage.list_tasks_across_projects(TaskFilters(status=Status.Todo))
    
    print(f"Active tasks in system: {len(active_tasks)}")
    
    # Group by assignee type
    assignee_groups = {}
    for task in active_tasks:
        if task.assignee:
            assignee_type = task.assignee.kind.name
            assignee_groups.setdefault(assignee_type, []).append(task)
        else:
            assignee_groups.setdefault("unassigned", []).append(task)
    
    for assignee_type, tasks in assignee_groups.items():
        print(f"\n{assignee_type.upper()} Tasks ({len(tasks)}):")
        for task in tasks:
            progress = f"{task.progress}%" if task.progress else "Not started"
            print(f"   • {task.action} - Progress: {progress}")


async def generate_workflow_report():
    """Generate a comprehensive workflow report"""
    
    storage = await Storage.new()
    
    print("\n📈 Workflow Analytics Report")
    print("=" * 60)
    
    all_tasks = storage.list_tasks_across_projects(TaskFilters())
    
    # Calculate statistics
    total_tasks = len(all_tasks)
    completed_tasks = len([t for t in all_tasks if t.status.name == "Done"])
    in_progress_tasks = len([t for t in all_tasks if t.status.name == "InProgress"])
    todo_tasks = len([t for t in all_tasks if t.status.name == "Todo"])
    
    print(f"Total Tasks: {total_tasks}")
    print(f"Completed: {completed_tasks} ({completed_tasks/total_tasks*100:.1f}%)")
    print(f"In Progress: {in_progress_tasks} ({in_progress_tasks/total_tasks*100:.1f}%)")
    print(f"To Do: {todo_tasks} ({todo_tasks/total_tasks*100:.1f}%)")
    
    # Priority breakdown
    priority_counts = {}
    for task in all_tasks:
        priority = task.priority.name
        priority_counts[priority] = priority_counts.get(priority, 0) + 1
    
    print("\nPriority Distribution:")
    for priority, count in priority_counts.items():
        print(f"   {priority}: {count} tasks")


async def main():
    """Main workflow demonstration"""
    
    try:
        # Run the collaborative workflow
        workflow_content = await collaborative_bug_fix_workflow()
        
        # Monitor progress
        await monitor_workflow_progress()
        
        # Generate report
        await generate_workflow_report()
        
        print("\n✅ Collaborative workflow demonstration completed!")
        print("💡 This example shows how AI and human teams can work together effectively")
        print("   using the todozi task management system.")
        
    except Exception as e:
        print(f"❌ Error running workflow: {e}")
        import traceback
        traceback.print_exc()


if __name__ == "__main__":
    asyncio.run(main())