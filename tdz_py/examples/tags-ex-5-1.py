import asyncio
from typing import List

async def setup_project_tag_system() -> None:
    """Example of creating a hierarchical tag system for software development projects."""
    
    # Initialize tag manager
    manager = TagManager()
    
    # Create main categories
    category_tags = {
        "frontend": ["javascript", "react", "vue", "html", "css"],
        "backend": ["python", "django", "flask", "nodejs", "api"],
        "devops": ["docker", "kubernetes", "ci-cd", "deployment"],
        "database": ["postgresql", "mongodb", "redis", "sql"],
    }
    
    # Bulk create category tags
    for category, tech_tags in category_tags.items():
        # Create the category tag
        category_id = await manager.create_tag(
            name=category,
            description=f"{category.title()} development tasks",
            color="#4A90E2",
            category="technology"
        )
        
        # Create technology tags under this category
        for tech in tech_tags:
            tech_id = await manager.create_tag(
                name=tech,
                description=f"{tech.title()} related tasks",
                color="#7ED321",
                category=category
            )
            
            # Establish parent-child relationship
            await manager.add_tag_relationship(category_id, tech_id)
    
    # Create priority tags
    priority_tags = ["urgent", "high-priority", "medium-priority", "low-priority"]
    for priority in priority_tags:
        await manager.create_tag(
            name=priority,
            description=f"{priority.replace('-', ' ').title()} tasks",
            color="#D0021B" if "urgent" in priority else "#F5A623",
            category="priority"
        )
    
    # Create status tags
    status_tags = ["bug", "feature-request", "enhancement", "documentation"]
    for status in status_tags:
        await manager.create_tag(
            name=status,
            description=f"{status.replace('-', ' ').title()} items",
            color="#BD10E0" if "bug" in status else "#50E3C2",
            category="type"
        )

async def search_and_filter_tasks() -> None:
    """Example of using tag search for project organization."""
    
    manager = TagManager()
    
    # Search for backend-related tags
    backend_tags = manager.search_tags("backend")
    print("Backend-related tags:")
    for tag in backend_tags:
        print(f"  - {tag.name}: {tag.description}")
    
    # Find all database technologies
    db_tags = manager.search_tags("sql")
    print("\nDatabase technologies:")
    for tag in db_tags:
        print(f"  - {tag.name}")
    
    # Get tags by category
    tech_tags = manager.get_tags_by_category("technology")
    print(f"\nTechnology categories: {len(tech_tags)}")
    
    # Find related tags for a specific technology
    python_tag = manager.get_tag_by_name("python")
    if python_tag:
        related_tags = manager.get_related_tags(python_tag.id)
        print(f"\nTags related to Python: {[t.name for t in related_tags]}")

async def tag_statistics_and_usage() -> None:
    """Example of tracking tag usage statistics."""
    
    manager = TagManager()
    
    # Simulate tag usage (increment when tasks use these tags)
    frequently_used = ["javascript", "python", "bug", "urgent"]
    for tag_name in frequently_used:
        await manager.increment_tag_usage(tag_name)
        await manager.increment_tag_usage(tag_name)  # Use twice
    
    # Get statistics
    stats = manager.get_tag_statistics()
    print("Tag System Statistics:")
    print(f"  Total tags: {stats.total_tags}")
    print(f"  Categories: {stats.total_categories}")
    print(f"  Relationships: {stats.total_relationships}")
    print(f"  Average usage: {stats.average_usage:.1f}")
    print(f"  Relationships per tag: {stats.relationships_per_tag():.2f}")
    
    # Most used tags
    popular_tags = manager.get_most_used_tags(3)
    print("\nMost popular tags:")
    for tag in popular_tags:
        print(f"  - {tag.name}: {tag.usage_count} uses")

async def advanced_tag_search() -> None:
    """Example using TagSearchEngine for complex queries."""
    
    manager = TagManager()
    search_engine = TagSearchEngine(manager)
    
    # Advanced search query
    query = TagSearchQuery(
        name_contains="api",
        category="backend",
        min_usage=1,
        sort_by=TagSortBy.Usage,
        limit=5
    )
    
    results = search_engine.advanced_search(query)
    print("Advanced search results for API in backend:")
    for tag in results:
        print(f"  - {tag.name} (Category: {tag.category}, Uses: {tag.usage_count})")
    
    # Fuzzy search for misspelled tags
    fuzzy_results = search_engine.fuzzy_search("javascrit", max_distance=3)
    print("\nFuzzy search for 'javascrit':")
    for tag, distance in fuzzy_results:
        print(f"  - {tag.name} (Distance: {distance})")
    
    # Tag suggestions based on current tags
    current_tags = ["javascript", "react"]
    suggestions = search_engine.get_suggestions(current_tags, 5)
    print(f"\nSuggestions for {current_tags}: {suggestions}")

async def tag_merging_and_cleanup() -> None:
    """Example of cleaning up duplicate tags."""
    
    manager = TagManager()
    
    # Create some duplicate tags (simulating migration scenario)
    await manager.create_tag(name="js", description="JavaScript", category="frontend")
    await manager.create_tag(name="javascript", description="JavaScript language", category="frontend")
    await manager.create_tag(name="ecmascript", description="ECMAScript", category="frontend")
    
    # Merge duplicates into primary tag
    primary_id = manager.get_tag_by_name("javascript").id
    duplicate_ids = [
        tag.id for tag in [
            manager.get_tag_by_name("js"),
            manager.get_tag_by_name("ecmascript")
        ] if tag
    ]
    
    await manager.merge_tags(primary_id, duplicate_ids)
    
    # Verify merge
    js_tag = manager.get_tag_by_name("js")
    javascript_tag = manager.get_tag_by_name("javascript")
    
    print("After merging duplicates:")
    print(f"  'js' tag exists: {js_tag is None}")
    print(f"  'javascript' tag usage: {javascript_tag.usage_count if javascript_tag else 0}")

async def main():
    """Run all tag management examples."""
    print("=== Project Tag Management System ===\n")
    
    await setup_project_tag_system()
    await search_and_filter_tasks()
    await tag_statistics_and_usage()
    await advanced_tag_search()
    await tag_merging_and_cleanup()

if __name__ == "__main__":
    asyncio.run(main())