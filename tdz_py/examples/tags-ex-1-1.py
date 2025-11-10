# example1_tags_usage.py
# Demonstrates practical usage of the tags module (TagManager, TagSearchEngine).

import asyncio
from pathlib import Path
import sys

# Ensure the package root is on the path so `from tags import ...` works.
_tags_path = Path(__file__).with_name("tags.py")
if _tags_path.exists():
    sys.path.insert(0, str(Path(__file__).parent))

from tags import (
    TagManager,
    TagSearchEngine,
    TagUpdate,
    TagSearchQuery,
    TagSortBy,
    ValidationError,
)


async def main() -> None:
    manager = TagManager()
    engine = TagSearchEngine(manager)

    print("=== Example 1: TagManager and TagSearchEngine ===\n")

    # 1) Create tags
    id_bug = await manager.create_tag(
        "bug",
        description="A software bug",
        color="#FF3B30",
        category="quality",
    )
    id_feature = await manager.create_tag(
        "feature",
        description="A new product feature",
        color="#34C759",
        category="product",
    )
    id_urgent = await manager.create_tag(
        "urgent",
        description="Urgent priority",
        color="#FF9500",
        category="priority",
    )
    print(f"Created tags: bug={id_bug}, feature={id_feature}, urgent={id_urgent}")

    # 2) Relate tags
    await manager.add_tag_relationship(id_bug, id_feature)
    await manager.add_tag_relationship(id_bug, id_urgent)
    print("Created relationships: bug <-> feature, bug <-> urgent\n")

    # 3) Update tag using fluent builder
    await manager.update_tag(
        id_bug,
        TagUpdate().with_description("Software bug report").with_color("#E53935"),
    )
    print("Updated bug tag description and color")

    # 4) Usage tracking
    await manager.increment_tag_usage("feature")
    await manager.increment_tag_usage("feature")
    await manager.increment_tag_usage("feature")
    print("Incremented usage for 'feature' 3 times\n")

    # 5) Statistics
    stats = manager.get_tag_statistics()
    print("TagStatistics:")
    print(f"  total_tags: {stats.total_tags}")
    print(f"  total_categories: {stats.total_categories}")
    print(f"  total_relationships: {stats.total_relationships}")
    print(f"  average_usage: {stats.average_usage:.2f}")
    print(f"  relationships_per_tag: {stats.relationships_per_tag():.2f}\n")

    # 6) Most used / recent
    most_used = manager.get_most_used_tags(limit=2)
    print("Most used tags:")
    for t in most_used:
        print(f"  {t.name} (usage={t.usage_count})")

    recent = manager.get_recent_tags(limit=3)
    print("Recent tags:")
    for t in recent:
        print(f"  {t.name} (created={t.created_at.isoformat()})")
    print()

    # 7) Search (token + substring)
    results = manager.search_tags(query="bug")
    print("Search 'bug' results:")
    for t in results:
        print(f"  - {t.name}: {t.description} (category={t.category})")
    print()

    # 8) Advanced search with filters/sorting
    query = TagSearchQuery(
        name_contains="e",
        min_usage=1,
        sort_by=TagSortBy.Usage,
        limit=5,
    )
    advanced = engine.advanced_search(query)
    print("Advanced search: name_contains='e', min_usage>=1, sort_by usage:")
    for t in advanced:
        print(f"  - {t.name}: usage={t.usage_count}, category={t.category}")
    print()

    # 9) Fuzzy search (edit distance)
    fuzzy = engine.fuzzy_search(query="featuer", max_distance=2)  # typo "featuer" -> "feature"
    print("Fuzzy search 'featuer' (max_distance=2):")
    for t, dist in fuzzy:
        print(f"  - {t.name} (distance={dist})")
    print()

    # 10) Suggestions from relationships
    suggestions = engine.get_suggestions(current_tags=["bug"], limit=5)
    print("Suggestions for 'bug' (via related tags):", suggestions)
    print()

    # 11) Bulk create
    created_ids = await manager.bulk_create_tags(
        ["regression", "hotfix", "refactor"],
        category="engineering",
    )
    print(f"Bulk created tags: {created_ids}\n")

    # 12) Merge duplicates
    id_regression = manager.get_tag_by_name("regression").id
    id_hotfix = manager.get_tag_by_name("hotfix").id
    # Accumulate usage for merge demo
    await manager.increment_tag_usage("regression")
    await manager.increment_tag_usage("hotfix")
    await manager.merge_tags(primary_tag_id=id_bug, duplicate_tag_ids=[id_regression, id_hotfix])
    print("Merged 'regression' and 'hotfix' into 'bug'")
    print(f"Primary tag usage now: {manager.get_tag(id_bug).usage_count}")
    print()

    # 13) Get related tags for 'bug'
    related = manager.get_related_tags(id_bug)
    print("Related tags for 'bug':")
    for t in related:
        print(f"  - {t.name}")
    print()

    # 14) Error handling (duplicate name)
    try:
        await manager.create_tag("feature", description="Duplicate!")
    except ValidationError as e:
        print("Caught expected ValidationError on duplicate name:")
        print(f"  {e.message}")
    print()

    # 15) Final state dump (names only)
    all_tags = manager.get_all_tags()
    print("All tags (name | category | usage):")
    for t in all_tags:
        print(f"  - {t.name} | {t.category} | {t.usage_count}")


if __name__ == "__main__":
    asyncio.run(main())