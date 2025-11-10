# example_memory_usage.py
import asyncio
from datetime import datetime, timezone
from memory import (
    MemoryManager, Memory, MemoryImportance, MemoryTerm, 
    EmotionalMemoryType, MemoryUpdate, parse_memory_format
)

async def main():
    # Initialize the memory manager
    manager = MemoryManager()
    
    # Example 1: Create different types of memories
    print("=== Creating Memories ===")
    
    # Standard memory
    standard_memory = Memory(
        user_id="user_123",
        moment="Team meeting discussion",
        meaning="Discussed Q4 project priorities",
        reason="Important for planning next quarter",
        importance=MemoryImportance.High,
        term=MemoryTerm.Long,
        memory_type="Standard",
        tags=["meeting", "planning", "Q4"]
    )
    standard_id = await manager.create_memory(standard_memory)
    print(f"Created standard memory with ID: {standard_id}")
    
    # Secret memory (AI-only)
    secret_memory = Memory(
        user_id="user_123",
        moment="API key rotation schedule",
        meaning="Internal security procedure",
        reason="Confidential operational detail",
        importance=MemoryImportance.Critical,
        term=MemoryTerm.Short,
        memory_type="Secret",
        tags=["security", "confidential"]
    )
    secret_id = await manager.create_memory(secret_memory)
    print(f"Created secret memory with ID: {secret_id}")
    
    # Emotional memory
    emotional_memory = Memory(
        user_id="user_123",
        moment="Received positive feedback",
        meaning="Team appreciated my contribution",
        reason="Boosts motivation and confidence",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type=EmotionalMemoryType("happy"),
        tags=["feedback", "motivation"]
    )
    emotional_id = await manager.create_memory(emotional_memory)
    print(f"Created emotional memory with ID: {emotional_id}")
    
    # Example 2: Retrieve memories
    print("\n=== Retrieving Memories ===")
    
    # Get specific memory
    retrieved = manager.get_memory(standard_id)
    if retrieved:
        print(f"Retrieved: {retrieved.moment} - {retrieved.meaning}")
    
    # Get all memories
    all_memories = manager.get_all_memories()
    print(f"Total memories: {len(all_memories)}")
    
    # Example 3: Search memories
    print("\n=== Searching Memories ===")
    search_results = manager.search_memories("project")
    print(f"Found {len(search_results)} memories related to 'project'")
    for mem in search_results:
        print(f" - {mem.moment}")
    
    # Example 4: Filter memories
    print("\n=== Filtering Memories ===")
    high_importance = manager.get_memories_by_importance(MemoryImportance.High)
    print(f"High importance memories: {len(high_importance)}")
    
    secret_memories = manager.get_secret_memories()
    print(f"Secret memories: {len(secret_memories)}")
    
    emotional_happy = manager.get_emotional_memories("happy")
    print(f"Happy emotional memories: {len(emotional_happy)}")
    
    # Example 5: Update memory
    print("\n=== Updating Memory ===")
    update = MemoryUpdate.builder().with_moment("Updated team meeting discussion").with_tags(["meeting", "Q4", "updated"])
    updated_memory = await manager.update_memory(standard_id, update)
    print(f"Updated memory: {updated_memory.moment}")
    print(f"New tags: {updated_memory.tags}")
    
    # Example 6: Memory statistics
    print("\n=== Memory Statistics ===")
    stats = manager.get_memory_statistics()
    print(f"Total memories: {stats.total_memories}")
    print(f"Critical memories: {stats.critical_memories}")
    print(f"Long-term percentage: {stats.long_term_percentage:.1f}%")
    
    # Example 7: Parse memory from text format
    print("\n=== Parsing Memory Format ===")
    memory_text = "<memory>standard; 2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long; client,development,iterative</memory>"
    parsed_memory = parse_memory_format(memory_text, "user_456")
    print(f"Parsed memory: {parsed_memory.moment}")
    print(f"Importance: {parsed_memory.importance}")
    print(f"Tags: {parsed_memory.tags}")
    
    # Example 8: Delete memory
    print("\n=== Deleting Memory ===")
    await manager.delete_memory(secret_id)
    remaining = manager.get_memory(secret_id)
    print(f"Secret memory after deletion: {'Found' if remaining else 'Not found'}")

if __name__ == "__main__":
    asyncio.run(main())