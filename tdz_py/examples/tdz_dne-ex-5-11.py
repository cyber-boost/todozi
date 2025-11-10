"""
Example 5: Batch processing multiple Todozi commands for task management automation
Demonstrates how to process multiple commands in a single request to the Todozi API.
"""

import asyncio
from tdz_dne import Result, process_tdz_commands

async def batch_process_tasks():
    """Process multiple todozi commands in a single batch for task management automation."""
    
    # Example: A project planning session with multiple commands
    batch_commands = """
    Let me plan my upcoming project. Here's what I need to do:

    <tdz>create; task; Setup project infrastructure; project=web-app; priority=high</tdz>
    
    <tdz>create; task; Design database schema; project=web-app; priority=medium</tdz>
    
    <tdz>create; task; Create user authentication system; project=web-app; priority=high</tdz>
    
    <tdz>create; task; Implement frontend components; project=web-app; priority=medium</tdz>
    
    <tdz>list; tasks; project=web-app</tdz>
    
    <tdz>search; tasks; authentication</tdz>
    
    <tdz>create; feeling; excited; intensity=8; description=Starting new project; context=development</tdz>
    """
    
    # Configuration (replace with your actual Todozi server details)
    base_url = "http://localhost:8636"  # Your Todozi server
    api_key = "your-api-key-here"  # Your API key
    
    print("🏁 Starting batch processing of Todozi commands...")
    print("=" * 60)
    
    # Process all commands in the batch
    result = await process_tdz_commands(
        text=batch_commands,
        base_url=base_url,
        api_key=api_key,
        timeout_total=60  # Give it more time for multiple operations
    )
    
    if result.is_ok:
        responses = result.unwrap()
        print(f"✅ Successfully processed {len(responses)} commands")
        print()
        
        for i, response in enumerate(responses, 1):
            print(f"📊 Response {i}:")
            if isinstance(response, dict):
                if 'message' in response:
                    print(f"   📝 Message: {response['message']}")
                if 'id' in response:
                    print(f"   🆔 ID: {response['id']}")
                if 'tasks' in response:
                    print(f"   📋 Tasks found: {len(response['tasks'])}")
                    for task in response['tasks'][:3]:  # Show first 3 tasks
                        action = task.get('action', 'Unknown')
                        status = task.get('status', 'Unknown')
                        print(f"      - {action} ({status})")
                if 'task' in response:
                    task = response['task']
                    action = task.get('action', 'Unknown')
                    priority = task.get('priority', 'Unknown')
                    print(f"   ✅ Created task: {action} (Priority: {priority})")
            print()
    else:
        error = result.unwrap()  # This will be the TodoziError
        print(f"❌ Error processing batch: {error.message}")
    
    print("=" * 60)
    print("🎯 Batch processing complete!")


async def create_complex_project_workflow():
    """Example of a more complex project workflow with error handling"""
    
    complex_workflow = """
    <tdz>create; project; name=Customer Portal; description=New customer-facing web portal</tdz>
    
    <tdz>create; task; Setup CI/CD pipeline; project=Customer Portal; time=2 days; priority=high</tdz>
    
    <tdz>create; task; Develop login module; project=Customer Portal; time=3 days; priority=high; assignee=backend-team</tdz>
    
    <tdz>create; task; Design responsive UI; project=Customer Portal; time=4 days; priority=medium; assignee=frontend-team</tdz>
    
    <tdz>create; task; Write API documentation; project=Customer Portal; time=1 day; priority=low</tdz>
    
    <tdz>list; tasks; project=Customer Portal</tdz>
    
    <tdz>stats</tdz>
    """
    
    base_url = "http://localhost:8636"
    api_key = "your-api-key-here"
    
    print("🔄 Starting complex project workflow...")
    
    result = await process_tdz_commands(
        text=complex_workflow,
        base_url=base_url,
        api_key=api_key
    )
    
    if result.is_ok:
        responses = result.unwrap()
        
        # Analyze results
        tasks_created = 0
        projects_created = 0
        listings_returned = 0
        
        for response in responses:
            if isinstance(response, dict):
                if 'task' in response:
                    tasks_created += 1
                elif 'project' in response:
                    projects_created += 1
                elif 'tasks' in response:
                    listings_returned += 1
        
        print(f"📊 Workflow Summary:")
        print(f"   📁 Projects created: {projects_created}")
        print(f"   📋 Tasks created: {tasks_created}")
        print(f"   📈 Listings returned: {listings_returned}")
        
    else:
        print(f"❌ Workflow execution failed: {result.unwrap().message}")


# Utility function for command templating
def generate_todozi_command(command: str, target: str, **params) -> str:
    """Generate a todozi command string from parameters."""
    parts = [f"<tdz>{command}; {target}"]
    
    for key, value in params.items():
        if value:
            parts.append(f"{key}={value}")
    
    return "".join(parts) + "</tdz>"


async def dynamic_batch_creation():
    """Example of dynamically generating todozi commands"""
    
    # Dynamic project creation
    projects = [
        {"name": "Marketing Campaign", "description": "Q2 marketing initiatives"},
        {"name": "Internal Tools", "description": "Developer productivity tools"},
        {"name": "Data Analytics", "description": "Business intelligence dashboards"}
    ]
    
    batch = []
    
    # Add project creation commands
    for project in projects:
        batch.append(generate_todozi_command(
            "create", "project",
            name=project["name"],
            description=project["description"]
        ))
    
    # Add some sample tasks for the first project
    marketing_tasks = [
        {"action": "Design campaign visuals", "priority": "high", "time": "5 days"},
        {"action": "Write campaign copy", "priority": "medium", "time": "3 days"},
        {"action": "Setup analytics tracking", "priority": "medium", "time": "2 days"}
    ]
    
    for task in marketing_tasks:
        batch.append(generate_todozi_command(
            "create", "task",
            action=task["action"],
            project="Marketing Campaign",
            priority=task["priority"],
            time=task["time"]
        ))
    
    # Final list command to see what we created
    batch.append("<tdz>list; projects</tdz>")
    
    batch_text = "\n\n".join(batch)
    
    print("🔄 Processing dynamically generated batch...")
    print("Generated commands:")
    print(batch_text[:200] + "..." if len(batch_text) > 200 else batch_text)
    print()


if __name__ == "__main__":
    # Run the examples
    async def main():
        print("🚀 Todozi Batch Processing Examples")
        print("=" * 50)
        
        # Example 1: Basic batch processing
        await batch_process_tasks()
        print()
        
        # Example 2: Complex workflow
        await create_complex_project_workflow()
        print()
        
        # Example 3: Dynamic generation
        await dynamic_batch_creation()
    
    asyncio.run(main())