#!/usr/bin/env python3
"""
example3.py – Demo of Todozi’s MemoryManager

Run it with:
    $ python - <<'PY'
    ... (paste the whole file) ...
    PY

or simply:
    $ python example3.py
"""

import asyncio
from dataclasses import asdict
from typing import List

# ---------------------------------------------------------
# Import the core classes from the repository
# ---------------------------------------------------------
# The import lines assume that this script lives in the same
# directory (or a sub‑directory) as the Todozi source tree.
# If you run the script from elsewhere make sure the repo root
# is on PYTHONPATH, e.g.
#   export PYTHONPATH=/path/to/todozi_repo:$PYTHONPATH
# ---------------------------------------------------------
from memory import (          # <-- the file you provided
    Memory,
    MemoryImportance,
    MemoryTerm,
    MemoryType,
    MemoryManager,
)

# ---------------------------------------------------------
# Helper: pretty‑print a list of memories
# ---------------------------------------------------------
def show_memories(title: str, memories: List[Memory]) -> None:
    print(f"\n=== {title} ({len(memories)}) ===")
    for m in memories:
        # Convert the dataclass to a dict for a compact view
        data = asdict(m)
        # Remove the huge UUID if you don’t need it
        data.pop("id", None)
        # Show a few interesting fields only
        print(f"- moment: {data['moment']!r}")
        print(f"  meaning: {data['meaning']!r}")
        print(f"  importance: {data['importance'].name}")
        print(f"  term: {data['term'].name}")
        print(f"  type: {data['memory_type']!r}")
        if data.get("tags"):
            print(f"  tags: {', '.join(data['tags'])}")
        if data.get("emotion"):
            print(f"  emotion: {data['emotion']!r}")
        print()


# ---------------------------------------------------------
# Main async routine – everything that talks to MemoryManager
# ---------------------------------------------------------
async def main() -> None:
    # -----------------------------------------------------------------
    # 1️⃣  Initialise the manager (it creates the internal dicts for us)
    # -----------------------------------------------------------------
    manager = MemoryManager()

    # -----------------------------------------------------------------
    # 2️⃣  Create a few memories of different flavours
    # -----------------------------------------------------------------
    standard_mem = Memory(
        user_id="alice",
        project_id=None,
        status="Active",
        moment="2025‑04‑01 09:30",
        meaning="Had a great coffee meeting with the Design team.",
        reason="Kick‑off for the new UI project.",
        importance=MemoryImportance.High,
        term=MemoryTerm.Long,
        memory_type=MemoryType.Standard,
        tags=["coffee", "design", "kickoff"],
    )

    secret_mem = Memory(
        user_id="alice",
        project_id=None,
        status="Active",
        moment="2025‑04‑02 14:15",
        meaning="Discussed a potential merger that is not public yet.",
        reason="Strategic planning.",
        importance=MemoryImportance.Critical,
        term=MemoryTerm.Short,
        memory_type=MemoryType.Secret,
        tags=["confidential", "merger"],
    )

    emotional_mem = Memory(
        user_id="alice",
        project_id=None,
        status="Active",
        moment="2025‑04‑03 18:45",
        meaning="Feeling proud about the prototype demo.",
        reason="Team effort paid off.",
        importance=MemoryImportance.Medium,
        term=MemoryTerm.Short,
        memory_type=MemoryType.Emotional,
        # In the original model the emotion is stored in the `emotion` field
        # (the `memory_type` stays “Emotional”).  The parser in `todozi.py`
        # populates it automatically, but when we construct the object
        # manually we set it ourselves.
        emotion="proud",
        tags=["prototype", "proud"],
    )

    # -----------------------------------------------------------------
    # 3️⃣  Persist the memories (the API is async, so we await)
    # -----------------------------------------------------------------
    for mem in (standard_mem, secret_mem, emotional_mem):
        mem_id = await manager.create_memory(mem)
        print(f"🗃️  Stored memory – id={mem_id}")

    # -----------------------------------------------------------------
    # 4️⃣  Query the store in a few different ways
    # -----------------------------------------------------------------
    # 4a – All memories
    all_mem = manager.get_all_memories()
    show_memories("All memories", all_mem)

    # 4b – By concrete type
    secret_memories = manager.get_secret_memories()
    show_memories("Secret memories", secret_memories)

    emotional_memories = manager.get_emotional_memories("proud")
    show_memories("Emotional memories (proud)", emotional_memories)

    # 4c – By tag (searches all memories)
    coffee_memories = manager.get_memories_by_tag("coffee")
    show_memories("Memories that contain the tag “coffee”", coffee_memories)

    # 4d – Full‑text search (index built on moment, meaning, reason & tags)
    search_results = manager.search_memories("prototype")
    show_memories("Full‑text search for “prototype”", search_results)

    # -----------------------------------------------------------------
    # 5️⃣  Print aggregate statistics
    # -----------------------------------------------------------------
    stats = manager.get_memory_statistics()
    print("\n=== Memory statistics ===")
    print(f"Total memories       : {stats.total_memories}")
    print(f"Short‑term          : {stats.short_term_memories}")
    print(f"Long‑term           : {stats.long_term_memories}")
    print(f"Critical memories   : {stats.critical_memories}")
    print(f"Unique tags         : {stats.unique_tags}")
    print(f"Secret memories     : {stats.secret_memories}")
    print(f"Human‑visible       : {stats.human_memories}")
    print(f"Emotional memories  : {stats.emotional_memories}")
    print(f"Standard memories   : {stats.standard_memories}")
    # The convenience percentages are also available:
    print(f"Short‑term %        : {stats.short_term_percentage:.1f}%")
    print(f"Long‑term %         : {stats.long_term_percentage:.1f}%")
    print(f"Critical %          : {stats.critical_percentage:.1f}%\n")

    # -----------------------------------------------------------------
    # 6️⃣  Clean‑up (optional) – demonstrates the delete API
    # -----------------------------------------------------------------
    # Uncomment the following block if you want to remove the demo data
    # after the script finishes.
    #
    # for mem in all_mem:
    #     await manager.delete_memory(mem.id)
    # print("🗑️  Demo memories removed.")


# ---------------------------------------------------------
# Entry‑point – run the async main() function
# ---------------------------------------------------------
if __name__ == "__main__":
    # Using asyncio.run makes the script work the same on Python 3.7+
    asyncio.run(main())