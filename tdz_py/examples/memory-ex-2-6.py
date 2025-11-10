import asyncio
from memory import (
    MemoryManager,
    Memory,
    MemoryImportance,
    MemoryTerm,
    MemoryType,
    EmotionalMemoryType,
)
from datetime import datetime, timezone

async def main():
    # Initialize the MemoryManager
    manager = MemoryManager()

    # 1. Create a standard memory
    standard_memory = Memory(
        user_id="user123",
        moment="2025-01-20 10:00:00",
        meaning="Completed the first chapter of the book",
        reason="Progress tracking for personal reading goal",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type=MemoryType.STANDARD,
        tags=["reading", "personal_goal"]
    )
    standard_id = await manager.create_memory(standard_memory)
    print(f"Created standard memory with ID: {standard_id}")

    # 2. Create a secret memory (AI-only)
    secret_memory = Memory(
        user_id="user123",
        moment="2025-01-20 11:00:00",
        meaning="Secret: Discovered a workaround for the bug in the legacy system",
        reason="Record for future debugging, but keep it secret",
        importance=MemoryImportance.High,
        term=MemoryTerm.Short,
        memory_type=MemoryType.SECRET,
        tags=["bug", "workaround", "internal"]
    )
    secret_id = await manager.create_memory(memory=secret_memory)
    print(f"Created secret memory with ID: {secret_id}")

    # 3. Create a human-visible memory
    human_memory = Memory(
        user_id="user123",
        moment="2025-01-20 12:00:00",
        meaning="Team meeting: Discussed project timeline and next steps",
        reason="For team visibility and reference",
        importance=MemoryImportance.High,
        term=MemoryTerm.Short,
        memory_type=MemoryType.HUMAN,
        tags=["meeting", "project", "team"]
    )
    human_id = await manager.create_memory(memory=human_memory)
    print(f"Created human memory with ID: {human_id}")

    # 4. Create an emotional memory
    emotional_memory = Memory(
        user_id="user123",
        moment="2025-01-20 13:00:00",
        meaning="Successfully fixed the critical bug that was blocking the release",
        reason="Feeling relieved and proud",
        importance=MemoryImportance.High,
        term=MemoryTerm.Short,
        memory_type=EmotionalMemoryType(emotion="happy"),
        tags=["bug_fix", "release", "achievement"]
    )
    emotional_id = await manager.create_memory(memory=emotional_memory)
    print(f"Created emotional memory with ID: {emotional_id}")

    # 5. Update the standard memory to add more context and change importance
    update = MemoryUpdate.builder()
    update.meaning = "Completed the first chapter of the book and took notes"
    update.importance = MemoryImportance.High
    update.tags = ["reading", "personal_goal", "notes"]
    updated_memory = await manager.update_memory(standard_id, update)
    print(f"Updated memory: {updated_memory.id}")

    # 6. Search memories by tag
    memories_by_tag = manager.get_memories_by_tag("bug")
    print("\nMemories with tag 'bug':")
    for mem in memories_by_tag:
        print(f"  - {mem.id}: {mem.meaning}")

    # 7. Search memories by importance
    high_importance = manager.get_memories_by_importance(MemoryImportance.High)
    print("\nHigh importance memories:")
    for mem in high_importance:
        print(f"  - {mem.id}: {mem.meaning}")

    # 8. Search memories by term
    short_term = manager.get_short_term_memories()
    print(f"\nTotal short-term memories: {len(short_term)}")
    for mem in short_term:
        print(f"  - {mem.id}: {mem.meaning}")

    # 9. Search memories by type (emotional)
    emotional_memories = manager.get_emotional_memories("happy")
    print("\nHappy memories:")
    for mem in emotional_memories:
        print(f"  - {mem.id}: {mem.meaning}")

    # 10. Get memory statistics
    stats = manager.get_memory_statistics()
    print("\nMemory Statistics:")
    print(f"  Total memories: {stats.total_memories}")
    print(f"  Short-term memories: {stats.short_term_memories}")
    print(f"  Long-term memories: {stats.long_term_memories}")
    print(f"  Critical memories: {stats.critical_memories}")
    print(f"  Unique tags: {stats.unique_tags}")
    print(f"  Secret memories: {stats.secret_memories}")
    print(f"  Human memories: {stats.human_memories}")
    print(f"  Emotional memories: {stats.emotional_memories}")
    print(f"  Standard memories: {stats.standard_memories}")

    # 11. List all memories (for demonstration, but note that in a real scenario we might have many)
    all_memories = manager.get_all_memories()
    print("\nAll memories:")
    for mem in all_memories:
        print(f"  [{mem.id}] {mem.meaning} (Type: {mem.memory_type}, Importance: {mem.importance})")

if __name__ == "__main__":
    asyncio.run(main())