#!/usr/bin/env python3
"""
Example 10: Advanced Features and Integrations
Demonstrates advanced Todozi capabilities and integration patterns
"""

from todozi import TodoziClient
import json

def main():
    # Initialize the Todozi client
    tdz = TodoziClient()

    print("🚀 Advanced Features and Integrations Demo")

    # Advanced Task Creation with Context
    print("\n🎯 Advanced Task Creation")

    # Create tasks with rich context
    complex_task_id = tdz.task("Refactor legacy authentication module to use JWT tokens with refresh capabilities")
    print(f"Created complex task: {complex_task_id}")

    # AI-powered task breakdown
    print("\n🤖 AI-Powered Task Breakdown")

    # Create a complex project and let AI help break it down
    project_task = "Build a complete e-commerce platform with payment integration"

    # Create main project
    project_id = tdz.create_project("E-commerce Platform", "Full-featured online store")
    tdz.set_project("E-commerce Platform")

    # Create subtasks that AI would typically help generate
    subtasks = [
        "Design database schema for products, orders, and users",
        "Implement user registration and authentication system",
        "Create product catalog with search and filtering",
        "Build shopping cart functionality",
        "Integrate payment gateway (Stripe/PayPal)",
        "Implement order management system",
        "Add inventory tracking and management",
        "Create admin dashboard for store management",
        "Set up email notifications for orders",
        "Implement product reviews and ratings system"
    ]

    print("Breaking down complex project into manageable tasks:")
    subtask_ids = []
    for i, subtask in enumerate(subtasks, 1):
        task_id = tdz.task(subtask)
        subtask_ids.append(task_id)
        print(f"  {i:2d}. ✅ {subtask}")

    # Knowledge Integration
    print("
🧠 Knowledge Integration"    # Capture domain knowledge
    knowledge_items = [
        ("Microservices architecture", "Break down monolithic apps into smaller, independent services"),
        ("JWT Authentication", "JSON Web Tokens for secure, stateless authentication"),
        ("Payment Security", "PCI compliance and secure payment processing requirements"),
        ("Database Indexing", "Optimize query performance with proper indexing strategies")
    ]

    for topic, insight in knowledge_items:
        memory_id = tdz.remember(f"Learned about {topic}", insight)
        print(f"  📚 Captured knowledge: {topic}")

    # Advanced Search and Discovery
    print("
🔍 Advanced Search and Discovery"    # Demonstrate different search types
    searches = [
        ("payment", "Payment-related tasks"),
        ("security", "Security-focused work"),
        ("database", "Data layer tasks"),
        ("user experience", "UX/UI related tasks")
    ]

    for query, description in searches:
        results = tdz.find(query)
        print(f"  '{query}' search: {len(results)} results - {description}")

    # AI Semantic Search
    print("
🤖 AI Semantic Search"    semantic_searches = [
        "online store functionality",
        "customer experience features",
        "backend infrastructure",
        "business logic implementation"
    ]

    for query in semantic_searches:
        results = tdz.ai_find(query)
        print(f"  AI search '{query}': {len(results)} matches")

    # Workflow Automation Simulation
    print("
⚙️  Workflow Automation Simulation"    # Simulate automated task assignment based on content analysis
    automation_rules = [
        ("payment", "AI", "Payment tasks assigned to AI for security analysis"),
        ("design", "Human", "Design tasks require human creativity"),
        ("testing", "AI", "Testing can be largely automated"),
        ("deployment", "Human", "Deployment needs human oversight")
    ]

    print("Automated task assignment based on content:")
    for keyword, assignee, reason in automation_rules:
        tasks = tdz.find(keyword)
        if tasks:
            assignment = "🤖" if assignee == "AI" else "👤"
            print(f"  {assignment} {len(tasks)} '{keyword}' tasks → {assignee} ({reason})")

    # Project Analytics
    print("
📊 Project Analytics"    all_tasks = tdz.all()
    project_tasks = tdz.project_tasks("E-commerce Platform")

    print(f"Project Progress: {len(project_tasks)}/{len(all_tasks)} tasks")
    print(".1f")

    # Export/Import Simulation
    print("
💾 Data Export/Import Simulation"    # Simulate exporting project data
    project_data = {
        "project": "E-commerce Platform",
        "tasks": len(project_tasks),
        "completed": len([t for t in project_tasks if t.status.lower() in ["done", "completed"]]),
        "in_progress": len([t for t in project_tasks if t.status.lower() == "inprogress"]),
        "pending": len([t for t in project_tasks if t.status.lower() == "todo"])
    }

    print("Project data export:")
    print(json.dumps(project_data, indent=2))

    print("\n🎉 Advanced features demonstration complete!")
    print("💡 Todozi supports complex project management, AI assistance, and workflow automation")

if __name__ == "__main__":
    main()
