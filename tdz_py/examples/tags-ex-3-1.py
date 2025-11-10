import asyncio
from pprint import pprint

# The TagManager and supporting classes live in `tags.py`.
# Adjust the import path if the file is not on the import search path.
# (If you run the demo from the repository root, the relative import works.)
from tags import (
    TagManager,
    TagUpdate,
    TagSearchEngine,
    TagSearchQuery,
    TagSortBy,
)

# ----------------------------------------------------------------------
# Helper that prints a short, readable representation of a Tag
# ----------------------------------------------------------------------
def tag_summary(tag):
    return {
        "id": tag.id,
        "name": tag.name,
        "category": tag.category,
        "usage": tag.usage_count,
        "created": tag.created_at.isoformat(timespec="seconds"),
    }


async def main() -> None:
    # --------------------------------------------------------------
    # 1️⃣  Initialise the manager (it holds all tags in memory)
    # --------------------------------------------------------------
    manager = TagManager()

    # --------------------------------------------------------------
    # 2️⃣  Create a few tags – one by one, one in bulk
    # --------------------------------------------------------------
    bug_id = await manager.create_tag(
        name="bug",
        description="A defect that needs fixing",
        color="#ff5555",
        category="development",
    )
    feat_id = await manager.create_tag(
        name="feature",
        description="A new piece of functionality",
        color="#55ff55",
        category="development",
    )

    # Bulk creation returns the IDs in the same order as the input list
    bulk_ids = await manager.bulk_create_tags(
        ["enhancement", "refactor", "documentation"], category="maintenance"
    )
    enh_id, ref_id, doc_id = bulk_ids

    print("\n🟢 Tags after creation")
    for t in manager.get_all_tags():
        pprint(tag_summary(t))

    # --------------------------------------------------------------
    # 3️⃣  Relate tags (e.g. a bug is often linked to a refactor)
    # --------------------------------------------------------------
    await manager.add_tag_relationship(bug_id, ref_id)          # bug → refactor
    await manager.add_tag_relationship(bug_id, doc_id)          # bug → documentation
    await manager.add_tag_relationship(feat_id, enh_id)         # feature → enhancement

    # --------------------------------------------------------------
    # 4️⃣  Update a tag using the fluent builder (`TagUpdate`)
    # --------------------------------------------------------------
    upd = (
        TagUpdate()
        .with_description("Critical software defect")
        .with_color("#ff0000")           # brighter red
        .with_category("critical‑issues")
    )
    await manager.update_tag(bug_id, upd)

    # --------------------------------------------------------------
    # 5️⃣  Simulate “using” the tags – every time a tag is attached to an
    #     item we bump its usage counter.
    # --------------------------------------------------------------
    await manager.increment_tag_usage("bug")          # used once
    await manager.increment_tag_usage("bug")          # used twice
    await manager.increment_tag_usage("feature")      # used once
    await manager.increment_tag_usage("enhancement") # used once

    print("\n🔵 Tags after usage increments")
    for t in manager.get_all_tags():
        pprint(tag_summary(t))

    # --------------------------------------------------------------
    # 6️⃣  Fuzzy search – find tags that are “close” to a miss‑typed name
    # --------------------------------------------------------------
    fuzzy_results = manager.search_tags("featur")   # typo on “feature”
    print("\n🔎 Fuzzy search result for 'featur'")
    for t in fuzzy_results:
        pprint(tag_summary(t))

    # --------------------------------------------------------------
    # 7️⃣  Advanced search using TagSearchEngine (filter, sort, limit)
    # --------------------------------------------------------------
    engine = TagSearchEngine(manager)
    query = TagSearchQuery(
        name_contains="bug",               # look for “bug” in the name
        min_usage=1,                       # only tags that have been used
        sort_by=TagSortBy.Usage,           # most used first
        limit=5,
    )
    advanced = engine.advanced_search(query)
    print("\n🧭 Advanced search (name contains 'bug', sorted by usage)")
    for t in advanced:
        pprint(tag_summary(t))

    # --------------------------------------------------------------
    # 8️⃣  Merge duplicate tags – suppose we discovered that “refactor”
    #     and “documentation” actually refer to the same concept.
    # --------------------------------------------------------------
    # The merge operation:
    #   * adds usage counts together
    #   * moves any relationships of the duplicates onto the primary tag
    #   * deletes the duplicate tags
    await manager.merge_tags(
        primary_tag_id=ref_id,                # keep “refactor”
        duplicate_tag_ids=[doc_id]            # merge “documentation” into it
    )

    print("\n✅ Tags after merging ‘documentation’ into ‘refactor’")
    for t in manager.get_all_tags():
        pprint(tag_summary(t))

    # --------------------------------------------------------------
    # 9️⃣  Show overall statistics (total tags, categories, relationships,
    #     average usage per tag)
    # --------------------------------------------------------------
    stats = manager.get_tag_statistics()
    print("\n📊 Tag statistics")
    print(f"  Total tags          : {stats.total_tags}")
    print(f"  Total categories    : {stats.total_categories}")
    print(f"  Total relationships : {stats.total_relationships}")
    print(f"  Average usage/tag  : {stats.average_usage:.2f}")

    # --------------------------------------------------------------
    # 10️⃣  Clean‑up – delete a tag we no longer need
    # --------------------------------------------------------------
    await manager.delete_tag(enh_id)          # delete “enhancement”
    print("\n❌ Tags after deleting ‘enhancement’")
    for t in manager.get_all_tags():
        pprint(tag_summary(t))


# --------------------------------------------------------------
# Run the demo
# --------------------------------------------------------------
if __name__ == "__main__":
    asyncio.run(main())