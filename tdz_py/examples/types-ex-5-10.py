#!/usr/bin/env python3
"""
Example 5: Custom Task Workflow with Todozi Storage Integration

This example shows how to:
1. Create a custom workflow for managing project tasks
2. Integrate with Todozi's storage system
3. Handle errors gracefully
4. Use embedded content processing
"""

import asyncio
import sys
from pathlib import Path
from typing import List, Dict, Any

# Import Todozi components
from todozi.storage import Storage, Task, Priority, Status
from todozi.models import TaskUpdate, TaskFilters
from todozi.error import TodoziError, TaskNotFoundError
from todozi.todozi import process_chat_message_extended, ChatContent


class ProjectWorkflow:
    """A custom workflow manager for project tasks."""
    
    def __init__(self, project_name: str):
        self.project_name = project_name
        self.storage = None
    
    async def initialize(self) -> None:
        """Initialize the workflow with storage."""
        try:
            self.storage = await Storage.new()
            print(f"✅ Workflow initialized for project: {self.project_name}")
        except Exception as e:
            print(f"❌ Failed to initialize workflow: {e}")
            raise
    
    async def create_complex_task(self, 
                                action: str,
                                time: str,
                                priority: Priority,
                                dependencies: List[str] = None,
                                context: str = None) -> str:
        """Create a task with dependencies and context."""
        if not self.storage:
            raise RuntimeError("Workflow not initialized")
        
        try:
            task = Task(
                action=action,
                time=time,
                priority=priority,
                parent_project=self.project_name,
                status=Status.Todo,
                dependencies=dependencies or [],
                context_notes=context
            )
            
            await self.storage.add_task_to_project(task)
            print(f"✅ Created task: {task.id} - {action}")
            return task.id
            
        except TodoziError as e:
            print(f"❌ Failed to create task: {e}")
            raise
    
    async def create_tasks_from_chat(self, message: str) -> List[str]:
        """Create multiple tasks from a chat message."""
        try:
            # Process the chat message to extract structured content
            chat_content = process_chat_message_extended(message, "workflow_user")
            
            task_ids = []
            for task in chat_content.tasks:
                # Set the project for all extracted tasks
                task.parent_project = self.project_name
                
                # Save task to storage
                await self.storage.add_task_to_project(task)
                task_ids.append(task.id)
                print(f"✅ Created task from chat: {task.id} - {task.action}")
            
            return task_ids
            
        except Exception as e:
            print(f"❌ Failed to create tasks from chat: {e}")
            return []
    
    async def update_task_progress(self, task_id: str, progress: int) -> None:
        """Update task progress with validation."""
        if not 0 <= progress <= 100:
            raise ValueError("Progress must be between 0 and 100")
        
        try:
            updates = TaskUpdate(progress=progress)
            
            if progress == 100:
                updates.status = Status.Done
            
            await self.storage.update_task_in_project(task_id, updates)
            print(f"✅ Updated task {task_id} progress to {progress}%")
            
        except TaskNotFoundError:
            print(f"❌ Task {task_id} not found")
        except TodoziError as e:
            print(f"❌ Failed to update task: {e}")
    
    async def get_project_stats(self) -> Dict[str, Any]:
        """Get comprehensive project statistics."""
        if not self.storage:
            return {}
        
        try:
            filters = TaskFilters(project=self.project_name)
            all_tasks = self.storage.list_tasks_across_projects(filters)
            active_tasks = [t for t in all_tasks if t.status == Status.Todo]
            completed_tasks = [t for t in all_tasks if t.status == Status.Done]
            
            stats = {
                "total_tasks": len(all_tasks),
                "active_tasks": len(active_tasks),
                "completed_tasks": len(completed_tasks),
                "completion_rate": (len(completed_tasks) / len(all_tasks)) * 100 if all_tasks else 0,
                "priority_distribution": self._get_priority_distribution(all_tasks)
            }
            
            return stats
            
        except Exception as e:
            print(f"❌ Failed to get project stats: {e}")
            return {}
    
    def _get_priority_distribution(self, tasks: List[Task]) -> Dict[str, int]:
        """Calculate priority distribution for tasks."""
        distribution = {priority.name: 0 for priority in Priority}
        
        for task in tasks:
            priority_name = task.priority.name if hasattr(task.priority, 'name') else str(task.priority)
            distribution[priority_name] = distribution.get(priority_name, 0) + 1
        
        return distribution
    
    async def export_tasks_markdown(self, output_path: Path) -> None:
        """Export tasks to a markdown file."""
        try:
            filters = TaskFilters(project=self.project_name)
            tasks = self.storage.list_tasks_across_projects(filters)
            
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(f"# Project: {self.project_name}\n\n")
                f.write(f"Total Tasks: {len(tasks)}\n\n")
                
                for i, task in enumerate(tasks, 1):
                    status_emoji = "🟢" if task.status == Status.Todo else "✅"
                    priority_str = task.priority.name if hasattr(task.priority, 'name') else str(task.priority)
                    
                    f.write(f"## {i}. {status_emoji} {task.action}\n")
                    f.write(f"- **ID**: {task.id}\n")
                    f.write(f"- **Priority**: {priority_str}\n")
                    f.write(f"- **Status**: {task.status.name if hasattr(task.status, 'name') else str(task.status)}\n")
                    f.write(f"- **Time Estimate**: {task.time}\n")
                    
                    if task.progress is not None:
                        f.write(f"- **Progress**: {task.progress}%\n")
                    
                    if task.context_notes:
                        f.write(f"- **Context**: {task.context_notes}\n")
                    
                    if task.dependencies:
                        f.write(f"- **Dependencies**: {', '.join(task.dependencies)}\n")
                    
                    f.write("\n")
            
            print(f"✅ Exported {len(tasks)} tasks to {output_path}")
            
        except Exception as e:
            print(f"❌ Failed to export tasks: {e}")


async def main():
    """Demonstrate the custom workflow."""
    
    # Initialize workflow
    workflow = ProjectWorkflow("example-project")
    
    try:
        await workflow.initialize()
        
        # Create individual tasks
        task1_id = await workflow.create_complex_task(
            action="Design database schema",
            time="3 hours",
            priority=Priority.High,
            context="Use PostgreSQL with proper indexing"
        )
        
        task2_id = await workflow.create_complex_task(
            action="Implement authentication system",
            time="5 hours",
            priority=Priority.Critical,
            dependencies=[task1_id],
            context="OAuth2 with JWT tokens"
        )
        
        # Create tasks from chat message
        chat_message = """
        <todozi>Write API documentation; 2 hours; medium; example-project; todo</todozi>
        <todozi>Test authentication endpoints; 1 hour; high; example-project; todo; dependencies={task2_id}</todozi>
        """.replace("{task2_id}", task2_id)
        
        chat_task_ids = await workflow.create_tasks_from_chat(chat_message)
        
        # Update progress
        await workflow.update_task_progress(task1_id, 50)
        await workflow.update_task_progress(task2_id, 25)
        
        # Get project statistics
        stats = await workflow.get_project_stats()
        print("\n📊 Project Statistics:")
        for key, value in stats.items():
            print(f"  {key}: {value}")
        
        # Export to markdown
        output_file = Path("project_report.md")
        await workflow.export_tasks_markdown(output_file)
        
        print(f"\n🎉 Workflow completed successfully!")
        print(f"📄 Project report saved to: {output_file}")
        
    except Exception as e:
        print(f"❌ Workflow failed: {e}")
        return 1
    
    return 0


if __name__ == "__main__":
    # Run the workflow
    exit_code = asyncio.run(main())
    sys.exit(exit_code)