#!/usr/bin/env python3
"""
Example 5: Complete Project Management with Todozi
Demonstrates the full workflow of project management with AI assistance
"""

import asyncio
from datetime import datetime, timezone
from typing import List, Optional
from lib import (
    Done, Ready, Task, Idea, Memory, Error, Priority, Status,
    AssigneeType, ShareLevel, MemoryImportance, IdeaImportance,
    ContentType, ChatContent
)


class ProjectManager:
    """Complete project management example using Todozi"""
    
    def __init__(self):
        self.embedding_service = None
    
    async def initialize_system(self):
        """Initialize Todozi system with auto-registration"""
        print("🚀 Initializing Todozi system...")
        await Done.init_with_auto_registration()
        self.embedding_service = await Done.embedding_service()
        print("✅ System initialized!")
    
    async def create_software_project(self, project_name: str, description: str):
        """Create a software development project"""
        await Done.create_project(project_name, description)
        print(f"✅ Created project: {project_name}")
        return project_name
    
    async def ai_plan_feature_development(self, project_name: str, feature_description: str):
        """Use AI to plan feature development tasks"""
        print(f"🤖 AI planning feature: {feature_description}")
        
        # Use AI planning
        planned_tasks = await Done.plan_tasks(
            goal=f"Implement {feature_description}",
            complexity="high",
            timeline="2 weeks",
            context=f"Project: {project_name}"
        )
        
        # Move all planned tasks to the correct project
        for task in planned_tasks:
            await Done.update_task_full(task.id, Done.create_update().with_parent_project(project_name))
        
        print(f"✅ AI planned {len(planned_tasks)} tasks")
        return planned_tasks
    
    async def create_manual_tasks(self, project_name: str):
        """Create manual tasks for the project"""
        manual_tasks = [
            ("Set up development environment", Priority.High),
            ("Write unit tests", Priority.Medium),
            ("Code review", Priority.Medium),
            ("Documentation", Priority.Low)
        ]
        
        created_tasks = []
        for action, priority in manual_tasks:
            task = await Done.create_task(
                action=action,
                priority=priority,
                project=project_name,
                time="1-2 days",
                context=f"Manual task for {project_name}"
            )
            created_tasks.append(task)
        
        print(f"✅ Created {len(created_tasks)} manual tasks")
        return created_tasks
    
    async def capture_development_insights(self, project_name: str):
        """Capture important insights and memories during development"""
        
        # Capture a breakthrough idea
        breakthrough_idea = await Done.create_idea(
            idea="Use microservices architecture for better scalability",
            context=f"Architectural decision for {project_name}"
        )
        print(f"💡 Captured breakthrough idea: {breakthrough_idea.action}")
        
        # Capture important memory
        memory_task = await Done.create_memory(
            moment="Performance optimization breakthrough",
            meaning="Found 50% performance improvement using caching",
            reason="Critical for production deployment"
        )
        print(f"🧠 Captured important memory: {memory_task.action}")
        
        return breakthrough_idea, memory_task
    
    async def log_development_error(self, project_name: str):
        """Log and track development errors"""
        # In a real implementation, this would create proper error tracking
        print(f"❌ Simulated error logging for {project_name}")
        # Error tracking would be implemented here
    
    async def semantic_search_project_content(self, project_name: str, query: str):
        """Use semantic search to find relevant project content"""
        print(f"🔍 Semantic search for '{query}' in {project_name}")
        
        # Search across all content types
        similar_tasks = await Done.find_tasks_ai(query)
        
        print(f"📊 Found {len(similar_tasks)} relevant tasks")
        for i, task in enumerate(similar_tasks[:3], 1):
            print(f"  {i}. {task.action} (Priority: {task.priority})")
        
        return similar_tasks
    
    async def get_project_stats(self, project_name: str):
        """Get comprehensive project statistics"""
        print(f"📊 Generating statistics for {project_name}")
        
        # Get all tasks in project
        filters = Done.create_filters().with_project(project_name).build()
        project_tasks = await Done.search_with_filters(filters)
        
        # Calculate statistics
        total_tasks = len(project_tasks)
        completed_tasks = len([t for t in project_tasks if t.status == Status.Done])
        in_progress = len([t for t in project_tasks if t.status == Status.InProgress])
        high_priority = len([t for t in project_tasks if t.priority == Priority.High])
        
        completion_rate = (completed_tasks / total_tasks * 100) if total_tasks > 0 else 0
        
        stats = {
            "total_tasks": total_tasks,
            "completed": completed_tasks,
            "in_progress": in_progress,
            "high_priority": high_priority,
            "completion_rate": f"{completion_rate:.1f}%"
        }
        
        print("Project Statistics:")
        for key, value in stats.items():
            print(f"  {key.replace('_', ' ').title()}: {value}")
        
        return stats
    
    async def generate_project_report(self, project_name: str):
        """Generate a comprehensive project report"""
        print(f"📋 Generating report for {project_name}")
        
        # Get project data
        tasks = await Done.search_with_filters(
            Done.create_filters().with_project(project_name).build()
        )
        
        # Get AI insights
        similar_patterns = await self.embedding_service.cluster_content()
        
        report = {
            "project": project_name,
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "task_count": len(tasks),
            "completion_rate": await Done.completion_rate(),
            "clusters": len(similar_patterns),
            "breakthrough_ideas": await Done.breakthrough_percentage()
        }
        
        print("Project Report:")
        for key, value in report.items():
            print(f"  {key.replace('_', ' ').title()}: {value}")
        
        return report


async def main():
    """Main demonstration function"""
    manager = ProjectManager()
    
    try:
        # 1. Initialize system
        await manager.initialize_system()
        
        # 2. Create a project
        project_name = await manager.create_software_project(
            "NextGen E-Commerce Platform",
            "Modern e-commerce platform with AI recommendations"
        )
        
        # 3. AI-powered planning
        planned_tasks = await manager.ai_plan_feature_development(
            project_name,
            "user recommendation system"
        )
        
        # 4. Add manual tasks
        manual_tasks = await manager.create_manual_tasks(project_name)
        
        # 5. Capture insights
        await manager.capture_development_insights(project_name)
        
        # 6. Semantic search demonstration
        await manager.semantic_search_project_content(project_name, "recommendation algorithm")
        
        # 7. Get statistics
        await manager.get_project_stats(project_name)
        
        # 8. Generate final report
        await manager.generate_project_report(project_name)
        
        print("\n🎉 Project management demonstration completed successfully!")
        print("The Todozi system is now ready for production use.")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        # Log error properly in production
        error_task = await Done.create_task(
            action=f"Error handling: {str(e)}",
            priority=Priority.High,
            project="system_errors"
        )
        print(f"📝 Error logged as task: {error_task.id}")


if __name__ == "__main__":
    # Run the demonstration
    asyncio.run(main())