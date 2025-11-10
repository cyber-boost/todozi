#!/usr/bin/env python3
"""
Example 5: Processing AI Conversations with Todozi Content Tools

This shows how to extract structured data (tasks, memories, ideas, etc.)
from AI assistant conversations using the Todozi content processor.
"""

import asyncio
from datetime import datetime, timezone
from todozi.tdz_tls import TdzContentProcessorTool, initialize_tdz_content_processor

async def process_ai_conversation():
    """Process a conversation with an AI assistant to extract Todozi data."""
    
    # Initialize the processor state
    state = await initialize_tdz_content_processor()
    processor = TdzContentProcessorTool(state)
    
    # Sample AI conversation with embedded Todozi tags
    conversation = """
I've analyzed your project requirements. Here's what we need to do:

<todozi>Research web frameworks; 2 hours; high; web-project; todo</todozi>
<todozi>Design database schema; 4 hours; critical; web-project; todo</todozi>

Some important insights from our discussion:
<memory>standard; User prefers Python frameworks; This affects framework selection; Important for decision-making; high; long; framework-choice</memory>

Great idea that came up:
<idea>Use microservices architecture; team; high; Better scalability for future growth</idea>

Also, this error needs attention:
<error>Database connection timeout; Connection times out after 30 seconds; critical; network; api-service</error>

For training purposes:
<train>instruction; Explain microservices benefits; Microservices provide better scalability, independent deployment, and technology diversity; architecture-example; microservices,architecture; 0.9; training</train>

The overall plan seems solid. Let me know if you want me to elaborate on any part!
"""

    # Process the conversation
    result = await processor.execute({
        "content": conversation,
        "session_id": "project-planning-001",
        "extract_checklist": True,
        "auto_session": True
    })

    if result.success:
        print("✅ Conversation processed successfully!")
        print("=" * 60)
        print("PROCESSED OUTPUT:")
        print("=" * 60)
        print(result.output)
        print("=" * 60)
        
        # Show what was extracted
        print("\n📊 EXTRACTED DATA SUMMARY:")
        print("=" * 60)
        
        # Tasks extracted
        tasks = [item for item in state.checklist_items if "task" in item.source.lower()]
        print(f"📋 Tasks found: {len(tasks)}")
        for task in tasks:
            print(f"   • {task.content} ({task.priority})")
        
        # Recent actions processed
        print(f"\n⚡ Recent actions: {len(state.recent_actions)}")
        for action in state.recent_actions[-3:]:  # Last 3 actions
            status = "✅" if action.success else "❌"
            print(f"   {status} {action.action_type}: {action.description}")
            
        # Active sessions
        active_sessions = [s for s in state.active_sessions.values() 
                          if s.last_activity > datetime.now(timezone.utc) - timedelta(hours=24)]
        print(f"\n💬 Active sessions: {len(active_sessions)}")
        for session in active_sessions:
            print(f"   📋 {session.topic}: {session.message_count} messages")
            
    else:
        print(f"❌ Processing failed: {result.output}")

async def process_json_ai_response():
    """Process a JSON response from an AI API that includes tool calls."""
    
    state = await initialize_tdz_content_processor()
    processor = TdzContentProcessorTool(state)
    
    # Sample JSON response from an AI API with tool calls
    json_response = '''
    {
        "content": "I'll help you organize your project tasks.",
        "tool_calls": [
            {
                "function": {
                    "name": "create_todozi_task",
                    "arguments": {
                        "action": "Set up development environment",
                        "time": "1 hour",
                        "priority": "high"
                    }
                }
            },
            {
                "function": {
                    "name": "create_todozi_memory",
                    "arguments": {
                        "moment": "Development setup preferences",
                        "meaning": "User prefers Docker containers",
                        "reason": "Consistency across environments"
                    }
                }
            }
        ]
    }
    '''
    
    result = await processor.execute({
        "content": json_response,
        "session_id": "api-integration-001",
        "extract_checklist": True
    })
    
    if result.success:
        print("\n" + "=" * 60)
        print("JSON API RESPONSE PROCESSED:")
        print("=" * 60)
        print(result.output)

async def demonstrate_natural_language_extraction():
    """Show how the processor extracts tasks from natural language."""
    
    state = await initialize_tdz_content_processor()
    processor = TdzContentProcessorTool(state)
    
    # Natural language without explicit tags
    natural_text = """
We need to fix the authentication bug by tomorrow. Don't forget to update the documentation 
and make sure to test all user flows. Important: we should also review the security audit findings.
"""
    
    # Extract checklist items from natural language
    checklist_items = processor.extract_checklist_items(natural_text)
    
    print("\n" + "=" * 60)
    print("NATURAL LANGUAGE EXTRACTION:")
    print("=" * 60)
    print(f"📝 Original text: {natural_text.strip()}")
    print(f"\n🔍 Checklist items found: {len(checklist_items)}")
    
    for i, item in enumerate(checklist_items, 1):
        print(f"   {i}. {item.content} (Priority: {item.priority})")

async def main():
    """Run all demonstration examples."""
    print("🚀 Todozi Content Processor Examples")
    print("=" * 60)
    
    await process_ai_conversation()
    await process_json_ai_response()
    await demonstrate_natural_language_extraction()
    
    print("\n" + "=" * 60)
    print("✅ All examples completed!")
    print("💡 Use these patterns to integrate Todozi with your AI workflows")

if __name__ == "__main__":
    # Note: timedelta import needed for the example
    from datetime import timedelta
    asyncio.run(main())