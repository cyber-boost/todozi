import asyncio
from tags import TagManager, TagSearchEngine, TagSearchQuery, TagSortBy

async def main():
    # Initialize tag manager and search engine
    manager = TagManager()
    engine = TagSearchEngine(manager)
    
    # Create some tags
    bug_tag_id = await manager.create_tag(
        "bug", 
        description="Software defect or issue", 
        color="red", 
        category="issue-type"
    )
    
    feature_tag_id = await manager.create_tag(
        "feature", 
        description="New functionality", 
        color="blue", 
        category="issue-type"
    )
    
    urgent_tag_id = await manager.create_tag(
        "urgent", 
        description="Requires immediate attention", 
        color="orange", 
        category="priority"
    )
    
    # Create relationships between tags
    await manager.add_tag_relationship(bug_tag_id, urgent_tag_id)
    
    # Increment usage count
    await manager.increment_tag_usage("bug")
    await manager.increment_tag_usage("bug")
    
    # Search for tags
    print("All tags:")
    for tag in manager.get_all_tags():
        print(f"  {tag.name} ({tag.usage_count} uses)")
    
    print("\nTags in 'issue-type' category:")
    for tag in manager.get_tags_by_category("issue-type"):
        print(f"  {tag.name}")
    
    print("\nAdvanced search for tags with 'func' in name:")
    query = TagSearchQuery(
        name_contains="func",
        sort_by=TagSortBy.Name
    )
    results = engine.advanced_search(query)
    for tag in results:
        print(f"  {tag.name}: {tag.description}")
    
    print("\nFuzzy search for 'bog' (distance <= 2):")
    fuzzy_results = engine.fuzzy_search("bog", 2)
    for tag, distance in fuzzy_results:
        print(f"  {tag.name} (distance: {distance})")
    
    print("\nTag suggestions based on 'bug':")
    suggestions = engine.get_suggestions(["bug"], 5)
    for suggestion in suggestions:
        print(f"  {suggestion}")

if __name__ == "__main__":
    asyncio.run(main())