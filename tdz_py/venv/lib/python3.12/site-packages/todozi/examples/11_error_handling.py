#!/usr/bin/env python3
"""
Example 11: Error Handling and Best Practices
Demonstrates proper error handling and best practices for Todozi integration
"""

from todozi import TodoziClient
import sys

def main():
    print("🛡️  Error Handling and Best Practices Demo")

    try:
        # Initialize the Todozi client with error handling
        tdz = TodoziClient()
        print("✅ Todozi client initialized successfully")

    except RuntimeError as e:
        print(f"❌ Failed to initialize Todozi client: {e}")
        print("💡 Make sure the Rust extension is properly installed")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Unexpected error during initialization: {e}")
        sys.exit(1)

    # Example 1: Safe task creation with validation
    def create_task_safe(action, priority="medium"):
        """Safely create a task with validation and error handling"""
        if not action or not action.strip():
            print("❌ Error: Task action cannot be empty")
            return None

        if len(action) > 200:
            print("⚠️  Warning: Task action is very long, consider breaking it down")
            action = action[:200] + "..."

        try:
            if priority.lower() == "urgent":
                task_id = tdz.urgent(action)
            elif priority.lower() == "high":
                task_id = tdz.high(action)
            elif priority.lower() == "low":
                task_id = tdz.low(action)
            else:
                task_id = tdz.task(action)

            print(f"✅ Created {priority} priority task: {task_id}")
            return task_id

        except Exception as e:
            print(f"❌ Failed to create task '{action}': {e}")
            return None

    print("\n🛡️  Example 1: Safe Task Creation")
    # Test with valid input
    task1 = create_task_safe("Implement user authentication", "high")

    # Test with edge cases
    task2 = create_task_safe("", "medium")  # Empty task
    task3 = create_task_safe("A" * 250, "urgent")  # Very long task
    task4 = create_task_safe("Review pull request #123")  # Normal case

    # Example 2: Safe search with fallback
    def search_tasks_safe(query, use_ai=False, max_results=10):
        """Safely search tasks with fallback options"""
        if not query or not query.strip():
            print("❌ Error: Search query cannot be empty")
            return []

        try:
            if use_ai:
                results = tdz.ai_find(query)
                search_type = "AI semantic"
            else:
                results = tdz.find(query)
                search_type = "keyword"

            # Limit results
            limited_results = results[:max_results]

            print(f"✅ {search_type} search for '{query}': {len(limited_results)} results")
            return limited_results

        except Exception as e:
            print(f"❌ Search failed for '{query}': {e}")
            # Fallback to basic search if AI search fails
            if use_ai:
                print("🔄 Falling back to keyword search...")
                try:
                    return search_tasks_safe(query, use_ai=False, max_results=max_results)
                except:
                    pass
            return []

    print("\n🔍 Example 2: Safe Search with Fallback")
    results1 = search_tasks_safe("authentication")
    results2 = search_tasks_safe("user", use_ai=True)
    results3 = search_tasks_safe("", use_ai=True)  # Empty query
    results4 = search_tasks_safe("nonexistent_keyword_xyz")

    # Example 3: Safe project management
    def manage_project_safe(project_name, description=None):
        """Safely create and manage projects"""
        if not project_name or not project_name.strip():
            print("❌ Error: Project name cannot be empty")
            return None

        try:
            # Check if project already exists
            existing_projects = tdz.list_projects()
            project_names = [p.name for p in existing_projects]

            if project_name in project_names:
                print(f"ℹ️  Project '{project_name}' already exists")
                return project_name

            # Create new project
            project_id = tdz.create_project(project_name, description or f"Project: {project_name}")
            print(f"✅ Created project: {project_name}")
            return project_id

        except Exception as e:
            print(f"❌ Failed to create/manage project '{project_name}': {e}")
            return None

    print("\n📁 Example 3: Safe Project Management")
    proj1 = manage_project_safe("Error Handling Demo")
    proj2 = manage_project_safe("")  # Empty name
    proj3 = manage_project_safe("Error Handling Demo")  # Duplicate

    # Example 4: Batch operations with error handling
    def batch_create_tasks_safe(task_list):
        """Safely create multiple tasks with individual error handling"""
        if not task_list:
            print("❌ Error: Task list cannot be empty")
            return []

        successful = []
        failed = []

        print(f"📦 Creating {len(task_list)} tasks...")

        for i, task_data in enumerate(task_list, 1):
            try:
                if isinstance(task_data, str):
                    task_id = tdz.task(task_data)
                    action = task_data
                elif isinstance(task_data, dict):
                    action = task_data.get('action', '')
                    priority = task_data.get('priority', 'medium')
                    task_id = create_task_safe(action, priority)
                else:
                    raise ValueError("Invalid task data format")

                if task_id:
                    successful.append(task_id)
                    print(f"  ✅ {i:2d}. {action}")
                else:
                    failed.append(f"Task {i}: {action}")
                    print(f"  ❌ {i:2d}. {action}")

            except Exception as e:
                failed.append(f"Task {i}: {str(e)}")
                print(f"  ❌ {i:2d}. Error: {e}")

        print(f"\n📊 Batch creation complete: {len(successful)} successful, {len(failed)} failed")
        return successful, failed

    print("\n📦 Example 4: Batch Operations with Error Handling")
    task_batch = [
        "Add input validation",
        "Write comprehensive tests",
        {"action": "Implement error logging", "priority": "high"},
        {"action": "Create user documentation", "priority": "low"},
        "",  # Empty task
        "Deploy to production environment"
    ]

    successful, failed = batch_create_tasks_safe(task_batch)

    # Example 5: Graceful degradation
    def get_stats_safe():
        """Get statistics with graceful fallback"""
        try:
            stats = tdz.stats()
            print("✅ Basic stats retrieved")
            return stats
        except Exception as e:
            print(f"⚠️  Basic stats failed: {e}")

        try:
            detailed_stats = tdz.detailed_stats()
            print("✅ Detailed stats retrieved as fallback")
            return detailed_stats
        except Exception as e:
            print(f"⚠️  Detailed stats also failed: {e}")

        # Manual fallback
        try:
            all_tasks = tdz.all()
            manual_stats = {
                "total_tasks": len(all_tasks),
                "todo": len([t for t in all_tasks if t.status.lower() == "todo"]),
                "inprogress": len([t for t in all_tasks if t.status.lower() == "inprogress"]),
                "done": len([t for t in all_tasks if t.status.lower() in ["done", "completed"]])
            }
            print("✅ Manual stats calculated as final fallback")
            return manual_stats
        except Exception as e:
            print(f"❌ All stats methods failed: {e}")
            return {"error": "Unable to retrieve statistics"}

    print("\n📊 Example 5: Graceful Degradation")
    stats = get_stats_safe()
    print(f"Final stats: {stats}")

    print("\n🎉 Error handling and best practices demo complete!")
    print("💡 Key takeaways:")
    print("  • Always validate input parameters")
    print("  • Use try-catch blocks for all API calls")
    print("  • Provide meaningful error messages")
    print("  • Implement fallback strategies when possible")
    print("  • Log errors for debugging")

if __name__ == "__main__":
    main()
