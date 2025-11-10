#!/usr/bin/env python3
"""
task_workflow_example.py

Demonstrates task workflow management using Todozi's queue system.
Shows how to plan tasks, track progress across different statuses,
and manage work sessions for time tracking.
"""

import asyncio
from datetime import datetime, timezone
from todozi.todozi import parse_todozi_format, process_chat_message_extended
from todozi.storage import Storage
from todozi.types import QueueItem, QueueStatus, QueueCommands


class TaskWorkflowManager:
    def __init__(self):
        self.storage = asyncio.run(Storage.new())
    
    async def plan_complex_task(self, task_description: str) -> str:
        """Plan a complex task by breaking it down into queue items"""
        
        # Parse task using todozi format
        todozi_text = f"<todozi>{task_description}; 8 hours; high; development-project; todo; assignee=human; tags=complex,multi-step</todozi>"
        
        try:
            # Process the task
            task = parse_todozi_format(todozi_text)
            await self.storage.add_task_to_project(task)
            
            # Plan queue items for the complex task
            queue_items = [
                ("Analysis Phase", "Analyze requirements and create specifications", "high"),
                ("Design Phase", "Design architecture and interfaces", "high"),
                ("Implementation Phase", "Code the core functionality", "medium"),
                ("Testing Phase", "Write tests and validate functionality", "medium"),
                ("Documentation Phase", "Document code and usage", "low"),
            ]
            
            for item_name, item_description, priority in queue_items:
                queue_item = QueueItem(
                    task_name=item_name,
                    task_description=f"{item_description} for {task.action}",
                    priority=priority,
                    project_id=task.parent_project,
                    status=QueueStatus.BACKLOG,
                    created_at=datetime.now(timezone.utc),
                    updated_at=datetime.now(timezone.utc),
                )
                
                self.storage.add_queue_item(queue_item)
                print(f"✅ Planned queue item: {item_name}")
            
            return task.id
            
        except Exception as e:
            print(f"❌ Failed to plan task: {e}")
            return None
    
    async def start_work_session(self, task_id: str, phase_name: str) -> str:
        """Start a work session for a specific task phase"""
        
        # Find the queue item for this phase
        queue_items = self.storage.list_queue_items()
        target_item = None
        
        for item in queue_items:
            if (item.task_name == phase_name and 
                item.project_id == "development-project"):
                target_item = item
                break
        
        if not target_item:
            print(f"❌ Queue item not found for phase: {phase_name}")
            return None
        
        # Start the session
        session_id = self.storage.start_queue_session(target_item.id)
        
        print(f"🚀 Started work session for: {phase_name}")
        print(f"📋 Session ID: {session_id}")
        print(f"⏰ Started at: {datetime.now(timezone.utc)}")
        
        return session_id
    
    async def complete_work_phase(self, session_id: str, results: str = None) -> None:
        """Complete a work phase and move to next status"""
        
        # End the session
        self.storage.end_queue_session(session_id)
        
        # Get session details
        session = self.storage.get_queue_session(session_id)
        if session:
            print(f"✅ Completed work session: {session_id}")
            print(f"⏰ Duration: {session.duration_seconds} seconds")
            
            if results:
                print(f"📊 Results: {results}")
        
        # Update queue item status to complete
        queue_items = self.storage.list_queue_items()
        for item in queue_items:
            if item.id == session.queue_item_id:
                item.status = QueueStatus.COMPLETE
                item.updated_at = datetime.now(timezone.utc)
                print(f"📋 Queue item status updated to: {item.status.value}")
                break
    
    async def track_task_progress(self, task_id: str) -> None:
        """Track and display progress for a task"""
        
        # Get the main task
        try:
            task = self.storage.get_task_from_any_project(task_id)
            print(f"\n📊 Progress Tracking for: {task.action}")
            print("══════════════════════════════════════")
        except Exception:
            print(f"❌ Task {task_id} not found")
            return
        
        # Get queue items for this task's project
        queue_items = self.storage.list_queue_items()
        project_items = [item for item in queue_items if item.project_id == task.parent_project]
        
        status_counts = {
            QueueStatus.BACKLOG: 0,
            QueueStatus.ACTIVE: 0,
            QueueStatus.COMPLETE: 0,
        }
        
        for item in project_items:
            status_counts[item.status] += 1
        
        total_phases = len(project_items)
        completed_phases = status_counts[QueueStatus.COMPLETE]
        progress_percentage = (completed_phases / total_phases * 100) if total_phases > 0 else 0
        
        print(f"📈 Overall Progress: {progress_percentage:.1f}%")
        print(f"📋 Total Phases: {total_phases}")
        print(f"✅ Completed: {completed_phases}")
        print(f"⏳ Active: {status_counts[QueueStatus.ACTIVE]}")
        print(f"📥 Backlog: {status_counts[QueueStatus.BACKLOG]}")
        
        # Show phase details
        print("\n🎯 Phase Details:")
        for i, item in enumerate(project_items, 1):
            status_emoji = {
                QueueStatus.BACKLOG: "📥",
                QueueStatus.ACTIVE: "🔄",
                QueueStatus.COMPLETE: "✅",
            }.get(item.status, "❓")
            
            print(f"  {i}. {status_emoji} {item.task_name} - {item.status.value}")
    
    async def manage_multiple_tasks_chat(self, chat_message: str) -> None:
        """Process multiple tasks from a chat message"""
        
        print(f"\n💬 Processing chat message:")
        print(f"   {chat_message}")
        
        content = process_chat_message_extended(chat_message, "workflow_user")
        
        if content.tasks:
            print(f"\n📋 Found {len(content.tasks)} tasks in message:")
            
            for i, task in enumerate(content.tasks, 1):
                print(f"  {i}. {task.action}")
                
                # Create each task
                await self.storage.add_task_to_project(task)
                
                # Create basic queue item for each task
                queue_item = QueueItem(
                    task_name=f"Implement: {task.action}",
                    task_description=task.context_notes or task.action,
                    priority=task.priority.value if hasattr(task.priority, 'value') else "medium",
                    project_id=task.parent_project,
                    status=QueueStatus.BACKLOG,
                    created_at=datetime.now(timezone.utc),
                    updated_at=datetime.now(timezone.utc),
                )
                
                self.storage.add_queue_item(queue_item)
            
            print(f"✅ Successfully created {len(content.tasks)} tasks and queue items")


async def main():
    """Main workflow demonstration"""
    
    manager = TaskWorkflowManager()
    
    print("🚀 Todozi Task Workflow Management Demo")
    print("══════════════════════════════════════")
    
    # Example 1: Plan a complex task
    print("\n1. 📋 Planning Complex Task")
    task_id = await manager.plan_complex_task(
        "Build authentication system; 8 hours; high; development-project; todo"
    )
    
    if task_id:
        # Example 2: Work through phases
        print("\n2. 🔄 Working Through Phases")
        
        # Start analysis phase
        session1 = await manager.start_work_session(task_id, "Analysis Phase")
        await asyncio.sleep(1)  # Simulate work
        await manager.complete_work_phase(session1, "Requirements analyzed")
        
        # Start design phase  
        session2 = await manager.start_work_session(task_id, "Design Phase")
        await asyncio.sleep(1)  # Simulate work
        await manager.complete_work_phase(session2, "Architecture designed")
        
        # Track progress
        await manager.track_task_progress(task_id)
    
    # Example 3: Process multiple tasks from chat
    print("\n3. 💬 Processing Multi-Task Chat")
    
    multi_task_chat = """
    Need to complete these features:
    <todozi>Add user registration; 2 hours; high; auth-system; todo; tags=auth,backend</todozi>
    <todozi>Implement password reset; 1 hour; medium; auth-system; todo; tags=auth,security</todozi>
    <todozi>Add email verification; 3 hours; medium; auth-system; todo; tags=auth,email</todozi>
    """
    
    await manager.manage_multiple_tasks_chat(multi_task_chat)
    
    # Example 4: Show final status
    print("\n4. 📊 Final Workflow Status")
    
    # Show all active queue items
    active_items = manager.storage.list_active_items()
    if active_items:
        print("🔄 Active Queue Items:")
        for item in active_items:
            print(f"  • {item.task_name} (Priority: {item.priority})")
    
    # Show backlog
    backlog_items = manager.storage.list_backlog_items()
    if backlog_items:
        print("📥 Backlog Queue Items:")
        for item in backlog_items:
            print(f"  • {item.task_name} (Priority: {item.priority})")
    
    print("\n✅ Workflow demo completed!")


if __name__ == "__main__":
    asyncio.run(main())