from __future__ import annotations

import asyncio
import sys
from pathlib import Path

# --------------------------------------------------------------
# Make sure the repository root (the parent of the `todozi` package)
# is on sys.path so imports work when the script lives in
# `examples/` or any other sub‑directory.
# --------------------------------------------------------------
REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

# ----------------------------------------------------------------------
# Imports from the Todozi code‑base
# ----------------------------------------------------------------------
from todozi.tags import (
    TagManager,
    TagUpdate,
    TagSearchEngine,
    levenshtein_distance,
)
from todozi.storage import Storage, init_storage

# ----------------------------------------------------------------------
# Helper: pretty print a list of tags
# ----------------------------------------------------------------------
def show_tags(label: str, tags) -> None:
    print(f"\n=== {label} ({len(tags)}) ===")
    for t in tags:
        cat = f" [{t.category}]" if t.category else ""
        col = f" ({t.color})" if t.color else ""
        print(
            f" • {t.name}{cat}{col} – {t.description or '<no description>'}"
            f"   usage: {t.usage_count}"
        )


# ----------------------------------------------------------------------
# Main async demo function
# ----------------------------------------------------------------------
async def main() -> None:
    # --------------------------------------------------------------
    # 0️⃣  Initialise the on‑disk storage layout once (creates
    #   ~/.todozi, default project, etc.).  It is safe to call
    #   repeatedly – existing folders are left untouched.
    # --------------------------------------------------------------
    await init_storage()

    # --------------------------------------------------------------
    # 1️⃣  Create a TagManager instance and add a few tags.
    # --------------------------------------------------------------
    tm = TagManager()

    # Create a handful of tags (we await each call because TagManager
    # uses an asyncio.Lock internally).
    tag_ids = {}
    tag_ids["frontend"] = await tm.create_tag(
        name="frontend",
        description="Anything related to the UI / client side",
        color="#1f8ef1",
        category="area",
    )
    tag_ids["backend"] = await tm.create_tag(
        name="backend",
        description="Server‑side code, APIs, databases",
        color="#ff9f43",
        category="area",
    )
    tag_ids["ui"] = await tm.create_tag(
        name="ui",
        description="User‑interface components",
        color="#00d8ff",
        category="concept",
    )
    tag_ids["api"] = await tm.create_tag(
        name="api",
        description="Public / internal APIs",
        color="#f368e0",
        category="concept",
    )
    tag_ids["performance"] = await tm.create_tag(
        name="performance",
        description="Speed, latency, throughput concerns",
        color="#ff5e57",
        category="quality",
    )
    tag_ids["bug"] = await tm.create_tag(
        name="bug",
        description="Defects that need fixing",
        color="#ff3333",
        category="issue",
    )
    tag_ids["enhancement"] = await tm.create_tag(
        name="enhancement",
        description="Feature improvements",
        color="#28c76f",
        category="issue",
    )
    print("\n✅ Created tags – IDs stored in `tag_ids` dict.")

    # --------------------------------------------------------------
    # 2️⃣  Add relationships between logical tags.
    # --------------------------------------------------------------
    await tm.add_tag_relationship(tag_ids["frontend"], tag_ids["ui"])
    await tm.add_tag_relationship(tag_ids["frontend"], tag_ids["performance"])
    await tm.add_tag_relationship(tag_ids["backend"], tag_ids["api"])
    await tm.add_tag_relationship(tag_ids["backend"], tag_ids["performance"])
    await tm.add_tag_relationship(tag_ids["bug"], tag_ids["backend"])
    await tm.add_tag_relationship(tag_ids["enhancement"], tag_ids["frontend"])

    # --------------------------------------------------------------
    # 3️⃣  Increment usage counters – this mimics a user actually
    #     selecting a tag while creating a Todozi task.
    # --------------------------------------------------------------
    for _ in range(4):
        await tm.increment_tag_usage("frontend")
    for _ in range(2):
        await tm.increment_tag_usage("backend")
    for _ in range(5):
        await tm.increment_tag_usage("performance")
    await tm.increment_tag_usage("bug")
    await tm.increment_tag_usage("enhancement")
    await tm.increment_tag_usage("enhancement")  # a second use → total 2

    # --------------------------------------------------------------
    # 4️⃣  Fetch and display *all* tags.
    # --------------------------------------------------------------
    all_tags = tm.get_all_tags()
    show_tags("All tags in the system", all_tags)

    # --------------------------------------------------------------
    # 5️⃣  Token‑based search (fast, uses the pre‑built index).
    # --------------------------------------------------------------
    search_term = "perf"
    perf_tags = tm.search_tags(search_term)
    show_tags(f"Search for token '{search_term}'", perf_tags)

    # --------------------------------------------------------------
    # 6️⃣  Fuzzy search – useful when the user mistypes a name.
    # --------------------------------------------------------------
    fuzzy_term = "fronend"   # miss‑spelled "frontend"
    fuzzy_results = TagSearchEngine(tm).fuzzy_search(fuzzy_term, max_distance=2)
    print(f"\n=== Fuzzy search for '{fuzzy_term}' (max distance = 2) ===")
    for tag, distance in fuzzy_results:
        print(f" • {tag.name} (distance = {distance})")

    # --------------------------------------------------------------
    # 7️⃣  Tag statistics – a quick way to surface analytics.
    # --------------------------------------------------------------
    stats = tm.get_tag_statistics()
    print("\n=== Tag statistics ===")
    print(f" • total tags           : {stats.total_tags}")
    print(f" • total categories     : {stats.total_categories}")
    print(f" • total relationships  : {stats.total_relationships}")
    print(f" • average usage per tag: {stats.average_usage:.2f}")

    # --------------------------------------------------------------
    # 8️⃣  Merge duplicate tags.
    #     Imagine we have a duplicate “frontend” tag called “front‑end”.
    # --------------------------------------------------------------
    duplicate_id = await tm.create_tag(
        name="front‑end",
        description="Duplicate of frontend (to be merged)",
        color="#1f8ef1",
        category="area",
    )
    # Give the duplicate a couple of uses so we can see the merge effect.
    await tm.increment_tag_usage("front‑end")
    await tm.increment_tag_usage("front‑end")

    print("\n🧹 Before merging duplicates:")
    show_tags("All tags", tm.get_all_tags())

    # Merge the duplicate into the canonical “frontend” tag.
    await tm.merge_tags(primary_tag_id=tag_ids["frontend"], duplicate_tag_ids=[duplicate_id])

    print("\n✅ After merging duplicate into 'frontend':")
    show_tags("All tags (post‑merge)", tm.get_all_tags())

    # --------------------------------------------------------------
    # 9️⃣  Demonstrate fetching *related* tags for “frontend”.
    # --------------------------------------------------------------
    related = tm.get_related_tags(tag_ids["frontend"])
    show_tags("Tags related to 'frontend'", related)

    # --------------------------------------------------------------
    #  🔚  End of demo – optionally clean up the storage folder.
    # --------------------------------------------------------------
    # Uncomment the line below if you want to remove the ~/.todozi
    # directory after the demo (useful for CI runs).
    # from todozi.storage import get_storage_dir, copy_dir_recursive, shutil; shutil.rmtree(get_storage_dir())
    print("\n🎉 Demo finished – all TagManager operations succeeded!")


# ----------------------------------------------------------------------
# Run the demo
# ----------------------------------------------------------------------
if __name__ == "__main__":
    # Run the async main() entry point.
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n🛑 Demo cancelled by user.")
        sys.exit(130)