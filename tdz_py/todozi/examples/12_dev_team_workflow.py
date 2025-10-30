#!/usr/bin/env python3
"""
Example 12: Development Team Workflow
Demonstrates a complete development workflow using Todozi for agile project management
"""

from todozi import TodoziClient
import json
from datetime import datetime

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🚀 Development Team Workflow Demo")
    print("=" * 50)

    # Phase 1: Sprint Planning
    print("\n📅 Phase 1: Sprint Planning")

    sprint_name = f"Sprint {datetime.now().strftime('%Y-%m-%d')}"
    sprint_project = tdz.create_project(sprint_name, "Two-week development sprint")
    tdz.set_project(sprint_name)

    print(f"✅ Created sprint project: {sprint_name}")

    # Define sprint backlog
    sprint_backlog = {
        "Features": [
            "Implement user registration with email verification",
            "Create dashboard with real-time metrics",
            "Add export functionality for reports",
            "Implement dark mode toggle"
        ],
        "Bugs": [
            "Fix mobile responsiveness issues",
            "Resolve memory leak in data processing",
            "Fix authentication token expiration"
        ],
        "Technical Debt": [
            "Refactor legacy API endpoints",
            "Update dependencies to latest versions",
            "Improve error handling across modules"
        ]
    }

    # Create backlog items
    backlog_tasks = {}
    for category, tasks in sprint_backlog.items():
        print(f"\n📝 Creating {category} tasks:")
        backlog_tasks[category] = []
        for task in tasks:
            task_id = tdz.task(f"[{category}] {task}")
            backlog_tasks[category].append(task_id)
            print(f"  ✅ {task}")

    # Phase 2: Sprint Grooming (Refinement)
    print("
🎯 Phase 2: Sprint Grooming"    # Refine and prioritize tasks
    print("Refining and prioritizing backlog items...")

    # Mark high-priority items
    high_priority_items = [
        "Fix mobile responsiveness issues",
        "Implement user registration with email verification",
        "Resolve memory leak in data processing"
    ]

    for item in high_priority_items:
        tasks = tdz.find(item)
        if tasks:
            # Recreate with high priority (in practice, you'd update existing)
            tdz.high(f"[URGENT] {item}")
            print(f"  🟠 Prioritized: {item}")

    # Assign AI tasks
    ai_tasks = [
        "Update dependencies to latest versions",
        "Improve error handling across modules"
    ]

    for task in ai_tasks:
        tasks = tdz.find(task)
        if tasks:
            tdz.ai_task(f"[AI] {task}")
            print(f"  🤖 Assigned to AI: {task}")

    # Phase 3: Daily Standup Simulation
    print("
🌅 Phase 3: Daily Standup"    # Start working on sprint tasks
    print("Team members starting work on assigned tasks...")

    # Developer 1: Frontend work
    frontend_tasks = [
        "Implement dark mode toggle",
        "Fix mobile responsiveness issues"
    ]

    for task in frontend_tasks:
        tasks = tdz.find(task)
        if tasks:
            tdz.start(tasks[0].id)
            print(f"  👨‍💻 Dev1 started: {task}")

    # Developer 2: Backend work
    backend_tasks = [
        "Implement user registration with email verification",
        "Refactor legacy API endpoints"
    ]

    for task in backend_tasks:
        tasks = tdz.find(task)
        if tasks:
            tdz.start(tasks[0].id)
            print(f"  👨‍💻 Dev2 started: {task}")

    # AI Assistant: Infrastructure work
    infra_tasks = [
        "Update dependencies to latest versions",
        "Resolve memory leak in data processing"
    ]

    for task in infra_tasks:
        tasks = tdz.find(task)
        if tasks:
            tdz.start(tasks[0].id)
            print(f"  🤖 AI started: {task}")

    # Phase 4: Progress Tracking
    print("
📊 Phase 4: Progress Tracking"    all_sprint_tasks = tdz.project_tasks(sprint_name)

    status_counts = {}
    for task in all_sprint_tasks:
        status = task.status.lower()
        status_counts[status] = status_counts.get(status, 0) + 1

    print("Sprint Progress:")
    total_tasks = len(all_sprint_tasks)
    completed = status_counts.get('done', 0) + status_counts.get('completed', 0)
    in_progress = status_counts.get('inprogress', 0)
    todo = status_counts.get('todo', 0)

    print(f"  📊 Total: {total_tasks} tasks")
    print(f"  ✅ Completed: {completed}")
    print(f"  🔄 In Progress: {in_progress}")
    print(f"  📝 Todo: {todo}")
    print(".1f")

    # Phase 5: Code Review and Completion
    print("
🔍 Phase 5: Code Review & Completion"    # Simulate completing some tasks
    tasks_to_complete = [
        "Fix mobile responsiveness issues",
        "Update dependencies to latest versions",
        "Implement dark mode toggle"
    ]

    for task in tasks_to_complete:
        tasks = tdz.find(task)
        if tasks:
            tdz.done(tasks[0].id)
            print(f"  ✅ Completed: {task}")

            # Capture learning from completed work
            if "mobile" in task.lower():
                tdz.remember(
                    f"Completed {task}",
                    "Learned that CSS Grid provides better mobile responsiveness than flexbox for complex layouts"
                )
            elif "dependencies" in task.lower():
                tdz.remember(
                    f"Completed {task}",
                    "Updated framework from v2.1 to v3.0, required migration of deprecated APIs"
                )

    # Phase 6: Sprint Retrospective
    print("
🤔 Phase 6: Sprint Retrospective"    # Capture lessons learned
    retrospective_items = [
        ("What went well", [
            "AI assistance improved code quality for infrastructure tasks",
            "Early prioritization helped focus on critical features",
            "Daily standup meetings kept everyone aligned"
        ]),
        ("What could improve", [
            "Need better test coverage for new features",
            "Consider pair programming for complex architectural changes",
            "Improve estimation accuracy for technical debt items"
        ]),
        ("Action items", [
            "Implement automated testing pipeline",
            "Schedule architecture review for upcoming features",
            "Create development guidelines document"
        ])
    ]

    for category, items in retrospective_items:
        print(f"\n📝 {category}:")
        for item in items:
            if category == "Action items":
                task_id = tdz.task(f"[Follow-up] {item}")
                print(f"  🎯 Created action: {item}")
            else:
                idea_id = tdz.idea(f"{category}: {item}")
                print(f"  💡 Captured: {item}")

    # Phase 7: Sprint Review and Planning
    print("
🎯 Phase 7: Sprint Review & Next Planning"    # Generate sprint summary
    final_tasks = tdz.project_tasks(sprint_name)
    final_status = {}

    for task in final_tasks:
        status = task.status.lower()
        final_status[status] = final_status.get(status, 0) + 1

    sprint_summary = {
        "sprint": sprint_name,
        "total_tasks": len(final_tasks),
        "completed": final_status.get('done', 0) + final_status.get('completed', 0),
        "in_progress": final_status.get('inprogress', 0),
        "remaining": final_status.get('todo', 0),
        "completion_rate": (final_status.get('done', 0) + final_status.get('completed', 0)) / len(final_tasks) * 100,
        "memories_captured": len(tdz.list_memories()),
        "ideas_captured": len(tdz.list_ideas())
    }

    print("Sprint Summary:")
    print(json.dumps(sprint_summary, indent=2))

    # Plan next sprint items
    print("
🚀 Planning Next Sprint..."    next_sprint_items = [
        "Implement automated testing pipeline",
        "Add user profile management",
        "Create admin dashboard",
        "Set up monitoring and alerting"
    ]

    next_sprint = f"Next Sprint {datetime.now().strftime('%Y-%m-%d')}"
    next_project = tdz.create_project(next_sprint, "Follow-up sprint based on retrospective")

    print(f"✅ Created next sprint project: {next_sprint}")
    print("Next sprint backlog:")
    for item in next_sprint_items:
        task_id = tdz.task(item)
        print(f"  📝 {item}")

    print("\n🎉 Development workflow complete!")
    print("💡 Key benefits of using Todozi for development:")
    print("  • Structured project management")
    print("  • AI-assisted task assignment")
    print("  • Knowledge capture and sharing")
    print("  • Progress tracking and analytics")
    print("  • Retrospective-driven improvement")

if __name__ == "__main__":
    main()
