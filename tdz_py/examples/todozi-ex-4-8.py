import asyncio
from todozi import process_chat_message_extended, Storage
from todozi.models import Task, Assignee

async def demo_ai_workflow():
    # 1. Process natural language input
    message = """
    I need to:
    <todozi>Investigate memory leak in production; ASAP; critical; backend; todo</todozi>
    <todozi>Prepare Q3 roadmap presentation; 2 days; high; planning; todo</todozi>
    <memory>critical; Production incident; Memory usage spiked to 90%; Identified potential leak in cache module; high; short</memory>
    """
    
    # 2. Parse structured content
    content = process_chat_message_extended(message, "user_123")
    print(f"Parsed {len(content.tasks)} tasks and {len(content.memories)} memories")
    
    # 3. Process with AI assignment
    storage = await Storage.get_instance()
    for task in content.tasks:
        # AI will automatically route to appropriate agent
        if "investigate" in task.action.lower():
            task.assignee = Assignee.ai()
        elif "roadmap" in task.action.lower():
            task.assignee = Assignee.human()
        
        # Save and process
        await storage.add_task_to_project(task)
        print(f"Created task: {task.action}")

# Run the demo
asyncio.run(demo_ai_workflow())