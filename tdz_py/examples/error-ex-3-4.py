# example_03_tag_manager.py
# --------------------------------------------------------------
# Demonstrates the public API of `TagManager` (tags.py).
# --------------------------------------------------------------

import asyncio
from tags import (
    TagManager,
    TagUpdate,
    TagStatistics,
    TagSearchQuery,
    TagSortBy,
)

# ----------------------------------------------------------------------
# Helper to pretty‑print a list of tags
# ----------------------------------------------------------------------
def show_tags(tags, title: str = "Tags"):
    print(f"\n=== {title} ({len(tags)}) ===")
    for t in tags:
        print(
            f"- {t.name:<12}  "
            f"category={t.category or '-':<10}  "
            f"usage={t.usage_count:<3}  "
            f"id={t.id[:8]}..."
        )
    print("-" * 50)


# ----------------------------------------------------------------------
# Main demo (async because TagManager uses asyncio.Lock)
# ----------------------------------------------------------------------
async def main():
    manager = TagManager()

    # ------------------------------------------------------------------
    # 1️⃣  Create a few tags – single and bulk
    # ------------------------------------------------------------------
    bug_id = await manager.create_tag(
        "bug",
        description="A defect that needs fixing",
        color="#ff0000",
        category="quality",
    )
    feature_id = await manager.create_tag(
        "feature",
        description="New functionality request",
        color="#00ff00",
        category="roadmap",
    )
    print(f"Created tags → bug={bug_id[:8]}, feature={feature_id[:8]}")

    # Bulk create – all will belong to the same category
    bulk_ids = await manager.bulk_create_tags(
        ["enhancement", "refactor", "documentation"],
        category="maintenance",
    )
    print(f"Bulk‑created tags IDs: {[i[:8] for i in bulk_ids]}")

    # ------------------------------------------------------------------
    # 2️⃣  Add relationships (e.g. bug ↔️ enhancement)
    # ------------------------------------------------------------------
    await manager.add_tag_relationship(bug_id, feature_id)          # bug → feature
    await manager.add_tag_relationship(bug_id, bulk_ids[0])        # bug → enhancement
    await manager.add_tag_relationship(bulk_ids[0], bulk_ids[1])   # enhancement → refactor

    # ------------------------------------------------------------------
    # 3️⃣  Increment usage counters – simulate tags being applied
    # ------------------------------------------------------------------
    await manager.increment_tag_usage("bug")
    await manager.increment_tag_usage("bug")
    await manager.increment_tag_usage("feature")
    await manager.increment_tag_usage("documentation")
    await manager.increment_tag_usage("documentation")
    await manager.increment_tag_usage("documentation")

    # ------------------------------------------------------------------
    # 4️⃣  Search examples
    # ------------------------------------------------------------------
    #   a) Simple name search (case‑insensitive)
    found = manager.search_tags("bug")
    show_tags(found, "Search result for 'bug'")

    #   b) Token‑based search using the built‑in index
    #      (searches name + description tokens)
    found = manager.search_tags("defect")
    show_tags(found, "Search result for token 'defect'")

    #   c) Advanced search with a full query object
    query = TagSearchQuery(
        name_contains="ref",
        category="maintenance",
        min_usage=0,
        sort_by=TagSortBy.Usage,   # most used first
        limit=5,
    )
    advanced = manager.search_tags(query.name_contains)  # fallback to simple search
    show_tags(advanced, "Advanced query (name contains 'ref')")

    # ------------------------------------------------------------------
    # 5️⃣  List tags by category
    # ------------------------------------------------------------------
    maint_tags = manager.get_tags_by_category("maintenance")
    show_tags(maint_tags, "Category → maintenance")

    # ------------------------------------------------------------------
    # 6️⃣  Get most‑used / most‑recent tags
    # ------------------------------------------------------------------
    most_used = manager.get_most_used_tags(limit=3)
    show_tags(most_used, "Top‑3 most used tags")

    recent = manager.get_recent_tags(limit=3)
    show_tags(recent, "3 most recently created tags")

    # ------------------------------------------------------------------
    # 7️⃣  Tag statistics
    # ------------------------------------------------------------------
    stats: TagStatistics = manager.get_tag_statistics()
    print("\n=== Tag statistics ===")
    print(f"Total tags          : {stats.total_tags}")
    print(f"Total categories    : {stats.total_categories}")
    print(f"Total relationships : {stats.total_relationships}")
    print(f"Average usage       : {stats.average_usage:.2f}")

    # ------------------------------------------------------------------
    # 8️⃣  Merge duplicate tags (e.g. merge 'documentation' into 'doc')
    # ------------------------------------------------------------------
    # First create a duplicate tag called "doc"
    doc_id = await manager.create_tag(
        "doc",
        description="Documentation files",
        color="#0000ff",
        category="maintenance",
    )
    # Simulate some usage on the duplicate
    await manager.increment_tag_usage("doc")
    await manager.increment_tag_usage("doc")

    print("\nBefore merge:")
    show_tags(manager.get_all_tags(), "All tags")

    # Merge the duplicate into the primary tag "documentation"
    await manager.merge_tags(
        primary_tag_id=bulk_ids[2],            # ID of the original 'documentation' tag
        duplicate_tag_ids=[doc_id],           # IDs to be merged into it
    )

    print("\nAfter merge (duplicate removed, usage accumulated):")
    show_tags(manager.get_all_tags(), "All tags")

    # ------------------------------------------------------------------
    # 9️⃣  Delete a tag (e.g. remove the temporary 'refactor' tag)
    # ------------------------------------------------------------------
    await manager.delete_tag(bulk_ids[1])   # bulk_ids[1] == 'refactor'
    print("\nAfter deleting 'refactor':")
    show_tags(manager.get_all_tags(), "All tags")

# ----------------------------------------------------------------------
# Run the demo
# ----------------------------------------------------------------------
if __name__ == "__main__":
    asyncio.run(main())