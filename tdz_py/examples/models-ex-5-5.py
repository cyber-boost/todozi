#!/usr/bin/env python3
"""
Example 5: Chat Processing and AI Task Assignment

This example shows how to:
1. Process natural language messages to extract structured tasks, memories, and ideas
2. Automatically assign tasks to specialized AI agents
3. Use the chat processing system with shorthand tags
4. Work with the queue system for task execution
"""

import asyncio
from datetime import datetime, timezone
from typing import List

# Import Todozi components
from todozi.todozi import (
    process_chat_message_extended,
    execute_agent_task,
    transform_shorthand_tags,
    ChatContent,
    Task,
    Memory,
    Idea,
    AgentAssignment,
    Assignee
)
from todozi.storage import Storage
from todozi.models import Priority, Status, MemoryImportance, MemoryTerm, MemoryType


async def process_developer_chat_session():
    """Process a realistic developer chat session with AI agent assignments."""
    
    # Initialize storage
    storage = await Storage.new()
    
    # Example: A developer's natural language planning session
    developer_chat = """
    I need to work on the authentication system today. Here's my plan:

    <tz>Implement OAuth2 login; 6 hours; high; python-auth-system; todo; assignee=agent:coder</tz>
    
    <tz>Write unit tests for auth middleware; 2 hours; medium; python-auth-system; todo; assignee=agent:tester</tz>
    
    <tz>Document the auth flow; 1 hour; low; python-auth-system; todo; assignee=agent:writer</tz>
    
    <mm>standard; Redis session storage decision; Using Redis for session management provides better scalability; High availability requirement; high; long; architecture,scaling</mm>
    
    <id>Multi-factor authentication feature; private; high; Could add 2FA for enterprise customers</id>
    
    <e>JWT token expiration issue; Tokens expiring too quickly in testing; medium; logic; auth-service; Testing environment has shorter expiry</e>
    
    <fe>focused; 8; Making good progress on authentication system; coding; productive,determined</fe>
    
    <ch>def authenticate_user(token: str) -> bool:
        # Validate JWT token
        try:
            payload = jwt.decode(token, SECRET_KEY, algorithms=["HS256"])
            return bool(payload.get("user_id"))
        except jwt.ExpiredSignatureError:
            return False</ch>
    """
    
    print("🔍 Processing developer chat session...")
    print("=" * 50)
    
    # Process the chat message
    user_id = "developer_001"
    content = process_chat_message_extended(developer_chat, user_id)
    
    # Display extracted content
    print(f"📋 Tasks extracted: {len(content.tasks)}")
    print(f"🧠 Memories extracted: {len(content.memories)}")
    print(f"💡 Ideas extracted: {len(content.ideas)}")
    print(f"❌ Errors extracted: {len(content.errors)}")
    print(f"🧩 Code chunks extracted: {len(content.code_chunks)}")
    print(f"😊 Feelings extracted: {len(content.feelings)}")
    print()
    
    # Process and assign tasks to agents
    agent_results = []
    for task in content.tasks:
        # Check if task has agent assignment
        if task.assignee and hasattr(task.assignee, 'kind') and task.assignee.kind.name == 'Agent':
            agent_name = task.assignee.name or "default"
            result = await execute_agent_task(task, agent_name)
            agent_results.append((task.action, agent_name, result))
    
    # Display agent assignments
    if agent_results:
        print("🤖 Agent Assignments:")
        print("-" * 30)
        for action, agent, result in agent_results:
            print(f"📋 Task: {action}")
            print(f"🤖 Agent: {agent}")
            print(f"✅ Result: {result}")
            print()
    
    # Process other content types
    if content.memories:
        print("🧠 Important Memories:")
        for memory in content.memories:
            print(f"📝 {memory.moment}: {memory.meaning}")
            if memory.reason:
                print(f"   Reason: {memory.reason}")
            print(f"   Importance: {memory.importance.value}")
            print()
    
    if content.ideas:
        print("💡 Ideas Captured:")
        for idea in content.ideas:
            print(f"💡 {idea.idea}")
            print(f"   Share level: {idea.share.value}")
            print(f"   Importance: {idea.importance.value}")
            print()
    
    return content


async def process_shorthand_chat_example():
    """Demonstrate shorthand tag processing."""
    
    print("🚀 Shorthand Tag Example:")
    print("=" * 40)
    
    # Shorthand tags are more concise
    shorthand_chat = """
    Quick planning session:
    
    <tz>Bug fix; 1h; high; urgent-project; todo</tz>
    <mm>standard; Quick insight; Important note; Reference; medium; short</mm>
    <id>Quick improvement; private; medium</id>
    <fe>happy; 7; Quick win!</fe>
    """
    
    # Transform shorthand to full tags
    transformed = transform_shorthand_tags(shorthand_chat)
    print("Transformed chat:")
    print(transformed)
    print()
    
    # Process the transformed chat
    content = process_chat_message_extended(transformed, "quick_user")
    
    print(f"Quick content extracted: {len(content.tasks)} tasks, {len(content.memories)} memories")
    return content


async def create_custom_agent_workflow():
    """Create a custom workflow with specialized agent assignments."""
    
    print("🎯 Custom Agent Workflow:")
    print("=" * 40)
    
    # Manual task creation with specific agent assignments
    security_task = Task(
        action="Security audit of authentication endpoints",
        time="4 hours",
        priority=Priority.High,
        parent_project="security-review",
        status=Status.Todo,
        assignee=Assignee.agent("detective"),  # Paranoid code detective
        tags=["security", "audit", "authentication"]
    )
    
    performance_task = Task(
        action="Performance optimization of database queries",
        time="3 hours", 
        priority=Priority.Medium,
        parent_project="performance-tune",
        status=Status.Todo,
        assignee=Assignee.agent("tuner"),  # OCD beautician/optimizer
        tags=["performance", "database", "optimization"]
    )
    
    documentation_task = Task(
        action="Write comprehensive API documentation",
        time="2 hours",
        priority=Priority.Low, 
        parent_project="api-docs",
        status=Status.Todo,
        assignee=Assignee.agent("writer"),  # Condescending but thorough teacher
        tags=["documentation", "api", "user-guide"]
    )
    
    tasks = [security_task, performance_task, documentation_task]
    
    # Execute each task with its assigned agent
    for task in tasks:
        if task.assignee and task.assignee.name:
            result = await execute_agent_task(task, task.assignee.name)
            print(f"🔧 {task.assignee.name} agent assigned to: {task.action}")
            print(f"   Result: {result}")
            print()
    
    return tasks


async def main():
    """Run all examples."""
    
    print("🎯 EXAMPLE 5: Chat Processing and AI Task Assignment")
    print("=" * 60)
    
    # Example 1: Process developer chat session
    print("1. Developer Chat Session Processing")
    await process_developer_chat_session()
    print()
    
    # Example 2: Shorthand tag demonstration
    print("2. Shorthand Tag Processing")
    await process_shorthand_chat_example()
    print()
    
    # Example 3: Custom agent workflow
    print("3. Custom Agent Workflow")
    await create_custom_agent_workflow()
    print()
    
    print("✅ Example 5 completed successfully!")
    print("💡 Key takeaways:")
    print("   - Todozi can extract structured data from natural language")
    print("   - AI agents can be automatically assigned to tasks")
    print("   - Shorthand tags provide concise syntax")
    print("   - Specialized agents handle different types of work")


if __name__ == "__main__":
    asyncio.run(main())