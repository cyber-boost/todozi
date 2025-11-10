#!/usr/bin/env python3
"""
Example 3 – Tag management playground

Demonstrates:
* creating, updating and deleting tags
* building relationships between tags
* fast token‑based search + fuzzy search
* suggestions based on co‑occurrence
* merging duplicate tags
* optional JSON persistence
"""

# ----------------------------------------------------------------------
# Standard library
# ----------------------------------------------------------------------
import asyncio
import json
import pathlib
from pprint import pprint

# ----------------------------------------------------------------------
# Todozi tag subsystem (import from the repository)
# ----------------------------------------------------------------------
# Adjust the import path if you are running the script from outside the repo:
#   import sys, pathlib; sys.path.append(str(pathlib.Path(__file__).parents[1]))
from tags import (                     # <-- the file you posted
    TagManager,
    TagSearchEngine,
    Tag,
    TagUpdate,
    TagStatistics,
    TagSearchQuery,
)

# ----------------------------------------------------------------------
# Helper: a tiny persistence layer (optional)
# ----------------------------------------------------------------------
DATA_DIR = pathlib.Path("./tag_demo_data")
DATA_DIR.mkdir(exist_ok=True)
JSON_FILE = DATA_DIR / "tags_snapshot.json"


def dump_tags_to_json(manager: TagManager) -> None:
    """Write the current tag collection to a JSON file."""
    payload = [tag.to_dict() for tag in manager.get_all_tags()]
    JSON_FILE.write_text(json.dumps(payload, indent=2))
    print(f"💾 Snapshot saved to {JSON_FILE}")


def load_tags_from_json(manager: TagManager) -> None:
    """Read tags from the snapshot and re‑populate the manager."""
    if not JSON_FILE.exists():
        print("⚠️  No snapshot found – starting with an empty manager.")
        return

    raw = json.loads(JSON_FILE.read_text())
    # The manager only knows how to *create* tags, so we use the low‑level
    # `Tag` constructor directly and then feed the objects into the internal dict.
    for data in raw:
        tag = Tag.from_dict(data)
        # Directly insert – we bypass the public API because we already have
        # a fully‑formed Tag instance.
        manager.tags[tag.id] = tag
        # Re‑build the auxiliary indexes (name, category, search tokens)
        manager._add_name_index(tag.name, tag.id)
        if tag.category:
            manager._add_category_index(tag.category, tag.id)
        manager._rebuild_search_index_for_tag(tag)

    print(f"🔄 Restored {len(manager.tags)} tags from {JSON_FILE}")


# ----------------------------------------------------------------------
# Main demo logic – all async because the manager uses an asyncio.Lock
# ----------------------------------------------------------------------
async def demo():
    # --------------------------------------------------------------
    # 1️⃣  Initialise a fresh manager (or restore a previous one)
    # --------------------------------------------------------------
    manager = TagManager()
    load_tags_from_json(manager)           # <-- optional, safe even if file is missing

    # --------------------------------------------------------------
    # 2️⃣  Create a few tags (category = "tech", color = hex code)
    # --------------------------------------------------------------
    print("\n🚀 Creating tags …")
    tag_ids = {}
    tag_ids["frontend"] = await manager.create_tag(
        "frontend",
        description="Anything the user sees – UI, CSS, JS",
        color="#ff5733",
        category="tech",
    )
    tag_ids["backend"] = await manager.create_tag(
        "backend",
        description="Server‑side logic, databases, APIs",
        color="#33aaff",
        category="tech",
    )
    tag_ids["ui"] = await manager.create_tag(
        "ui",
        description="User‑interface components",
        color="#ffb733",
        category="tech",
    )
    tag_ids["api"] = await manager.create_tag(
        "api",
        description="Public endpoints, REST/GraphQL",
        color="#8e44ad",
        category="tech",
    )
    tag_ids["devops"] = await manager.create_tag(
        "devops",
        description="CI/CD, infra, monitoring",
        color="#27ae60",
        category="ops",
    )
    print("✅ Tags created:", list(tag_ids.values()))

    # --------------------------------------------------------------
    # 3️⃣  Build relationships (e.g. frontend ↔ ui, backend ↔ api)
    # --------------------------------------------------------------
    print("\n🔗 Adding relationships …")
    await manager.add_tag_relationship(tag_ids["frontend"], tag_ids["ui"])
    await manager.add_tag_relationship(tag_ids["backend"], tag_ids["api"])
    await manager.add_tag_relationship(tag_ids["frontend"], tag_ids["api"])
    await manager.add_tag_relationship(tag_ids["devops"], tag_ids["backend"])
    print("✅ Relationships added")

    # --------------------------------------------------------------
    # 4️⃣  Increment usage counters (simulating real‑world hits)
    # --------------------------------------------------------------
    print("\n📈 Simulating tag usage …")
    for _ in range(5):
        await manager.increment_tag_usage("frontend")
    for _ in range(3):
        await manager.increment_tag_usage("backend")
    for _ in range(2):
        await manager.increment_tag_usage("ui")
    await manager.increment_tag_usage("api")
    await manager.increment_tag_usage("devops")
    print("✅ Usage counters updated")

    # --------------------------------------------------------------
    # 5️⃣  Token‑based search (fast, uses the internal index)
    # --------------------------------------------------------------
    print("\n🔍 Token‑based search for 'ui' …")
    results = manager.search_tags("ui")
    pprint([r.name for r in results])     # → ['ui']

    print("\n🔍 Token‑based search for 'server' …")
    results = manager.search_tags("server")
    pprint([r.name for r in results])     # → ['backend']

    # --------------------------------------------------------------
    # 6️⃣  Fuzzy search (Levenshtein distance ≤ 2)
    # --------------------------------------------------------------
    engine = TagSearchEngine(manager)
    print("\n🤏 Fuzzy search for 'fronend' (typo) …")
    fuzzy = engine.fuzzy_search("fronend", max_distance=2)
    for tag, dist in fuzzy:
        print(f"  • {tag.name!r} (distance {dist})")
    # Expected output: 'frontend' with distance 1

    # --------------------------------------------------------------
    # 7️⃣  Suggestions based on relationships
    # --------------------------------------------------------------
    print("\n💡 Get suggestions for ['frontend'] …")
    suggestions = engine.get_suggestions(["frontend"], limit=5)
    pprint(suggestions)   # → ['ui', 'api']

    # --------------------------------------------------------------
    # 8️⃣  Advanced filtered search (via TagSearchQuery)
    # --------------------------------------------------------------
    query = TagSearchQuery(
        name_contains="dev",
        category="ops",
        min_usage=1,
        sort_by="Usage",          # enum values are strings in the dataclass
        limit=10,
    )
    advanced = engine.advanced_search(query)
    print("\n🧾 Advanced search results:")
    for t in advanced:
        print(f"  • {t.name} (usage={t.usage_count}, category={t.category})")

    # --------------------------------------------------------------
    # 9️⃣  Merge duplicate tags – we create a duplicate of “api”
    # --------------------------------------------------------------
    print("\n🧹 Creating a duplicate tag ‘api_v2’ …")
    dup_id = await manager.create_tag(
        "api_v2",
        description="Legacy API tag (to be merged)",
        color="#8e44ad",
        category="tech",
    )
    # Simulate some usage on the duplicate
    for _ in range(4):
        await manager.increment_tag_usage("api_v2")

    # Show usage before merge
    print("\n📊 Usage before merge:")
    primary = manager.get_tag_by_name("api")
    duplicate = manager.get_tag_by_name("api_v2")
    print(f"  • api       → {primary.usage_count}")
    print(f"  • api_v2    → {duplicate.usage_count}")

    # Merge – keep the original “api”, absorb the duplicate
    print("\n🔀 Merging duplicate into primary …")
    await manager.merge_tags(primary.id, [duplicate.id])

    # Verify merge result
    merged = manager.get_tag_by_name("api")
    print("\n📊 Usage after merge:")
    print(f"  • api → {merged.usage_count}")      # should be 1 (original) + 4 (dup) = 5
    print(f"  • api_v2 exists? {manager.get_tag_by_name('api_v2') is not None}")

    # --------------------------------------------------------------
    # 10️⃣  Statistics overview
    # --------------------------------------------------------------
    stats: TagStatistics = manager.get_tag_statistics()
    print("\n📊 Tag statistics")
    print(f"  Total tags          : {stats.total_tags}")
    print(f"  Total categories    : {stats.total_categories}")
    print(f"  Total relationships : {stats.total_relationships}")
    print(f"  Avg usage per tag  : {stats.average_usage:.2f}")
    print(f"  Relationships/tag   : {stats.relationships_per_tag():.2f}")

    # --------------------------------------------------------------
    # 11️⃣  Persist the current state (optional)
    # --------------------------------------------------------------
    dump_tags_to_json(manager)


# ----------------------------------------------------------------------
# Run the demo
# ----------------------------------------------------------------------
if __name__ == "__main__":
    # Using uvloop (optional) gives a nicer event‑loop on Linux/macOS.
    try:
        import uvloop
        asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())
    except Exception:
        pass

    asyncio.run(demo())