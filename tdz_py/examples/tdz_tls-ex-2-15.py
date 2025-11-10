import asyncio
from todozi.tdz_tls import (
    TdzContentProcessorTool, 
    TodoziProcessorState, 
    initialize_tdz_content_processor
)
from datetime import datetime, timezone

async def process_team_meeting_notes():
    """
    Example: Process meeting notes to extract actionable items
    """
    # Initialize the processor state and tool
    state = await initialize_tdz_content_processor()
    processor = TdzContentProcessorTool(state)
    
    # Sample meeting notes with embedded tags
    meeting_notes = """
    Team sync meeting notes - 2025-01-17
    ========================================
    
    Action items from today's discussion:
    <todozi>Review pull request; 2 hours; high; dev-tools; todo; assignee=ai</todozi>
    <todozi>Update documentation; 1 hour; medium; docs; todo; assignee=human</todozi>
    
    Important insights:
    <memory>standard; Database performance bottleneck; Need to optimize queries; Critical for Q1 goals; high; long; database,performance</memory>
    
    New ideas to explore:
    <idea>Implement automated testing pipeline; private; high; Improve code quality,automation</idea>
    <idea>Create developer onboarding guide; team; medium; knowledge sharing</idea>
    """
    
    # Process the content
    result = await processor.execute({
        "content": meeting_notes,
        "session_id": "meeting-2025-01-17",
        "extract_checklist": True,
        "auto_session": True
    })
    
    if result.success:
        print("✅ Processing completed successfully!")
        print("\n" + "="*50)
        print(result.output)
        print("="*50)
        
        # Show what was extracted
        print("\n📋 Extracted Summary:")
        print(f"• Tasks extracted: {len(state.checklist_items)}")
        print(f"• Recent actions: {len(state.recent_actions)}")
        print(f"• Active sessions: {len(state.active_sessions)}")
        
        # Display extracted checklist items
        if state.checklist_items:
            print("\n📝 Generated Checklist Items:")
            for item in state.checklist_items[-5:]:  # Show last 5 items
                print(f"  ☐ {item.content} (Priority: {item.priority})")
        
        # Display processed content details
        last_processed = state.processed_contents[-1]
        print(f"\n📊 Processing Stats:")
        print(f"  • Content length: {last_processed.processing_stats.content_length} chars")
        print(f"  • Processing time: {last_processed.processing_stats.processing_time_ms}ms")
        print(f"  • Tool calls found: {last_processed.processing_stats.tool_calls_found}")
    else:
        print(f"❌ Processing failed: {result.output}")

async def process_with_tool_calls():
    """
    Example: Process content with actual tool calls
    """
    state = await initialize_tdz_content_processor()
    processor = TdzContentProcessorTool(state)
    
    # Content that includes tool calls in JSON format
    content_with_tools = """{
      "content": "We need to implement user authentication",
      "tool_calls": [
        {
          "function": {
            "name": "create_task",
            "arguments": {
              "action": "Implement OAuth2 authentication",
              "priority": "high",
              "parent_project": "auth-module"
            }
          }
        }
      ]
    }"""
    
    result = await processor.execute({
        "content": content_with_tools,
        "session_id": "auth-implementation",
        "extract_checklist": True,
        "auto_session": True
    })
    
    print("\n🔧 Tool Call Processing Example:")
    print(result.output)

async def demonstrate_natural_language_extraction():
    """
    Example: Extract tasks from plain text without explicit tags
    """
    state = await initialize_tdz_content_processor()
    processor = TdzContentProcessorTool(state)
    
    natural_text = """
    Quick update: We need to fix the critical bug in production. 
    Don't forget to add unit tests for the new API endpoints.
    Make sure to update the documentation before the release.
    """
    
    result = await processor.execute({
        "content": natural_text,
        "session_id": "daily-standup",
        "extract_checklist": True,
        "auto_session": True
    })
    
    print("\n🌱 Natural Language Extraction:")
    print(result.output)
    
    # Show extracted checklist items from natural patterns
    if state.checklist_items:
        print("\n📝 Extracted from Natural Language:")
        for item in state.checklist_items:
            if item.source == "natural_language":
                print(f"  • {item.content}")

async def main():
    """Run all examples"""
    await process_team_meeting_notes()
    print("\n" + "\n" + "="*60 + "\n")
    await process_with_tool_calls()
    print("\n" + "\n" + "="*60 + "\n")
    await demonstrate_natural_language_extraction()

if __name__ == "__main__":
    asyncio.run(main())