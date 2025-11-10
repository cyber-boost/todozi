# example1_memory_demo.py
# A self-contained, executable example that uses memory.py to demonstrate:
# - Creating standard, secret, human, and emotional memories
# - Updating memories
# - Searching memories
# - Listing memories by type/importance/term
# - Computing statistics

from __future__ import annotations

import asyncio
import sys
from pathlib import Path

# Ensure this script can import memory.py when run directly
THIS_FILE = Path(__file__).resolve()
ROOT = THIS_FILE.parent
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from memory import (
    MemoryManager,
    Memory,
    MemoryUpdate,
    EmotionalMemoryType,
    parse_memory_format,
    MemoryImportance,
    MemoryTerm,
    TodoziError,
)

# Optional types that some functions may use when filtering
try:
    from typing import Literal
    MemoryTypeLiteral = Literal["Standard", "Secret", "Human", "Short", "Long"]
except Exception:
    # If Literal isn't available, we can still filter by string equality
    MemoryTypeLiteral = object  # type: ignore


async def main() -> None:
    print("Example 1: Memory Manager Demo")
    print("=" * 60)

    # Initialize manager (no persistence in this demo)
    mm = MemoryManager()

    # --------------------------------------------------------------------------
    # Create some memories
    # --------------------------------------------------------------------------
    print("\n1) Creating memories...")

    # Standard memory
    m1 = Memory(
        user_id="user_123",
        moment="Kickoff meeting started on time",
        meaning="Team is aligned and ready",
        reason="Signals good process health",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type="Standard",
        tags=["meeting", "alignment"],
    )
    m1_id = await mm.create_memory(m1)
    print(f"  - Created standard memory: {m1_id}")

    # Secret memory
    m2 = Memory(
        user_id="user_123",
        moment="Noted a potential risk in vendor contract",
        meaning="May affect SLAs if unaddressed",
        reason="Need legal review before signing",
        importance=MemoryImportance.High,
        term=MemoryTerm.Long,
        memory_type="Secret",
        tags=["legal", "risk", "vendor"],
    )
    m2_id = await mm.create_memory(m2)
    print(f"  - Created secret memory: {m2_id}")

    # Human-visible memory
    m3 = Memory(
        user_id="user_123",
        moment="Client prefers asynchronous updates",
        meaning="Reduce meeting overhead",
        reason="Improve delivery cadence",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type="Human",
        tags=["client", "communication"],
    )
    m3_id = await mm.create_memory(m3)
    print(f"  - Created human-visible memory: {m3_id}")

    # Emotional memory (using EmotionalMemoryType)
    m4 = Memory(
        user_id="user_123",
        moment="Delivered a complex feature ahead of schedule",
        meaning="Team morale boosted",
        reason="Proves our capacity planning works",
        importance=MemoryImportance.High,
        term=MemoryTerm.Short,
        memory_type=EmotionalMemoryType("excited"),
        tags=["morale", "feature", "delivery"],
    )
    m4_id = await mm.create_memory(m4)
    print(f"  - Created emotional memory (excited): {m4_id}")

    # Another emotional memory
    m5 = Memory(
        user_id="user_123",
        moment="Missed a dependency handoff",
        meaning="Slight delay in the next iteration",
        reason="Unclear ownership",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type=EmotionalMemoryType("frustrated"),
        tags=["dependency", "process"],
    )
    m5_id = await mm.create_memory(m5)
    print(f"  - Created emotional memory (frustrated): {m5_id}")

    # --------------------------------------------------------------------------
    # Update a memory
    # --------------------------------------------------------------------------
    print("\n2) Updating memory...")
    updated = await mm.update_memory(
        m4_id,
        MemoryUpdate(moment="Delivered a complex feature ahead of schedule (v2)", tags=["morale", "feature", "delivery", "velocity"]),
    )
    print(f"  - Updated memory {m4_id}: tags={updated.tags}")

    # --------------------------------------------------------------------------
    # Search memories
    # --------------------------------------------------------------------------
    print("\n3) Searching memories...")
    results = mm.search_memories("meeting")
    print(f"  - Query 'meeting' matched {len(results)} memory(ies):")
    for r in results:
        print(f"    • {r.moment[:50]}...")

    # --------------------------------------------------------------------------
    # List by importance / term / type
    # --------------------------------------------------------------------------
    print("\n4) Listing by importance, term, and type...")

    high_importance = mm.get_memories_by_importance(MemoryImportance.High)
    print(f"  - High importance: {len(high_importance)} item(s)")

    short_term = mm.get_memories_by_term(MemoryTerm.Short)
    print(f"  - Short term: {len(short_term)} item(s)")

    long_term = mm.get_memories_by_term(MemoryTerm.Long)
    print(f"  - Long term: {len(long_term)} item(s)")

    secret_memories = mm.get_secret_memories()
    print(f"  - Secret memories: {len(secret_memories)} item(s)")

    human_memories = mm.get_human_memories()
    print(f"  - Human-visible memories: {len(human_memories)} item(s)")

    excited_memories = mm.get_emotional_memories("excited")
    print(f"  - Emotional (excited): {len(excited_memories)} item(s)")

    frustrated_memories = mm.get_emotional_memories("frustrated")
    print(f"  - Emotional (frustrated): {len(frustrated_memories)} item(s)")

    # --------------------------------------------------------------------------
    # Tag utilities
    # --------------------------------------------------------------------------
    print("\n5) Tags and tag statistics...")
    all_tags = mm.get_all_tags()
    print(f"  - All tags: {all_tags}")

    tag_stats = mm.get_tag_statistics()
    print(f"  - Tag statistics: {tag_stats}")

    # --------------------------------------------------------------------------
    # Statistics
    # --------------------------------------------------------------------------
    print("\n6) Memory statistics...")
    stats = mm.get_memory_statistics()
    print(f"  - Total memories: {stats.total_memories}")
    print(f"  - Short term: {stats.short_term_memories} ({stats.short_term_percentage:.1f}%)")
    print(f"  - Long term: {stats.long_term_memories} ({stats.long_term_percentage:.1f}%)")
    print(f"  - Critical + High: {stats.critical_memories} ({stats.critical_percentage:.1f}%)")
    print(f"  - Unique tags: {stats.unique_tags}")
    print(f"  - By type -> secret: {stats.secret_memories}, human: {stats.human_memories}, emotional: {stats.emotional_memories}, standard: {stats.standard_memories}")

    # --------------------------------------------------------------------------
    # Demonstrate robust parser
    # --------------------------------------------------------------------------
    print("\n7) Parsing a memory from text format...")
    memory_text = "<memory>standard; 2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long; client,development,iterative</memory>"
    parsed = parse_memory_format(memory_text, "user_123")
    print(f"  - Parsed moment: {parsed.moment}")
    print(f"  - Parsed meaning: {parsed.meaning}")
    print(f"  - Parsed reason: {parsed.reason}")
    print(f"  - Parsed importance: {parsed.importance}")
    print(f"  - Parsed term: {parsed.term}")
    print(f"  - Parsed memory_type: {parsed.memory_type}")
    print(f"  - Parsed tags: {parsed.tags}")

    # --------------------------------------------------------------------------
    # Error handling example
    # --------------------------------------------------------------------------
    print("\n8) Error handling (invalid emotion)...")
    try:
        invalid_emotion = EmotionalMemoryType("zest")  # not in allowed list
        print("This should not print.")
    except ValueError as ex:
        print(f"  - Caught expected ValueError: {ex}")

    print("\n" + "=" * 60)
    print("Example completed successfully.")


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nInterrupted.")
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()