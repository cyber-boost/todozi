import asyncio
from tags import TagManager, TagSearchEngine, TagSearchQuery, TagSortBy
from datetime import datetime, timezone

async def setup_development_tags():
    """Initialize tags for a software development project"""
    manager = TagManager()
    
    # Create category-based tags
    frontend_id = await manager.create_tag(
        "frontend",
        description="Frontend development tasks",
        color="#4CAF50",
        category="development"
    )
    
    backend_id = await manager.create_tag(
        "backend",
        description="Backend and API development",
        color="#2196F3", 
        category="development"
    )
    
    database_id = await manager.create_tag(
        "database",
        description="Database design and optimization",
        color="#FF9800",
        category="development"
    )
    
    # Create priority-based tags
    critical_id = await manager.create_tag(
        "critical",
        description="High-priority, time-sensitive tasks",
        color="#F44336",
        category="priority"
    )
    
    # Create technology tags
    react_id = await manager.create_tag(
        "react",
        description="React.js frontend framework",
        color="#61DAFB",
        category="technology"
    )
    
    nodejs_id = await manager.create_tag(
        "nodejs",
        description="Node.js backend runtime",
        color="#339933",
        category="technology"
    )
    
    postgres_id = await manager.create_tag(
        "postgresql",
        description="PostgreSQL database system",
        color="#336791",
        category="technology"
    )
    
    # Create methodology tags
    testing_id = await manager.create_tag(
        "testing",
        description="Unit and integration testing",
        color="#9C27B0",
        category="process"
    )
    
    documentation_id = await manager.create_tag(
        "documentation",
        description="Technical documentation",
        color="#607D8B",
        category="process"
    )
    
    return manager

async def establish_tag_relationships(manager: TagManager):
    """Create meaningful relationships between tags"""
    
    # Frontend relationships
    await manager.add_tag_relationship("frontend", "react")
    await manager.add_tag_relationship("frontend", "testing")
    await manager.add_tag_relationship("frontend", "documentation")
    
    # Backend relationships
    await manager.add_tag_relationship("backend", "nodejs")
    await manager.add_tag_relationship("backend", "database")
    await manager.add_tag_relationship("backend", "testing")
    await manager.add_tag_relationship("backend", "documentation")
    
    # Database relationships
    await manager.add_tag_relationship("database", "postgresql")
    await manager.add_tag_relationship("database", "testing")
    
    # Technology cross-relationships
    await manager.add_tag_relationship("react", "testing")
    await manager.add_tag_relationship("nodejs", "testing")
    await manager.add_tag_relationship("postgresql", "testing")

async def demonstrate_tag_operations():
    """Demonstrate various tag management operations"""
    
    print("🏷️  Todozi Tag Management Example")
    print("=" * 50)
    
    # Initialize tag manager
    manager = await setup_development_tags()
    
    # Establish relationships
    await establish_tag_relationships(manager)
    
    # Display all tags
    print("\n📋 All Created Tags:")
    all_tags = manager.get_all_tags()
    for tag in all_tags:
        print(f"  • {tag.name} ({tag.category}) - {tag.description}")
    
    # Display categories
    print(f"\n📂 Categories: {manager.get_all_categories()}")
    
    # Display tag statistics
    stats = manager.get_tag_statistics()
    print(f"\n📊 Tag Statistics:")
    print(f"  Total tags: {stats.total_tags}")
    print(f"  Categories: {stats.total_categories}")
    print(f"  Relationships: {stats.total_relationships}")
    print(f"  Avg usage: {stats.average_usage:.2f}")
    
    # Get tags by category
    print("\n🔷 Development Tags:")
    dev_tags = manager.get_tags_by_category("development")
    for tag in dev_tags:
        print(f"  • {tag.name}: {tag.description}")
    
    # Show tag relationships
    frontend_tag = manager.get_tag_by_name("frontend")
    if frontend_tag:
        print(f"\n🔗 Tags related to 'frontend':")
        related_tags = manager.get_related_tags(frontend_tag.id)
        for tag in related_tags:
            print(f"  • {tag.name}")
    
    # Update a tag
    print("\n✏️  Updating frontend tag description...")
    await manager.update_tag(
        frontend_tag.id,
        TagUpdate().with_description("Frontend development with modern frameworks")
    )
    updated = manager.get_tag(frontend_tag.id)
    print(f"  New description: {updated.description}")
    
    # Increment tag usage
    print("\n📈 Incrementing tag usage...")
    await manager.increment_tag_usage("frontend")
    await manager.increment_tag_usage("frontend")
    await manager.increment_tag_usage("react")
    
    # Show most used tags
    print("\n🏆 Most Used Tags:")
    most_used = manager.get_most_used_tags(5)
    for tag in most_used:
        print(f"  • {tag.name}: {tag.usage_count} uses")
    
    return manager

async def demonstrate_search_engine(manager: TagManager):
    """Demonstrate advanced tag search capabilities"""
    
    print("\n🔍 Tag Search Engine Demo")
    print("-" * 30)
    
    engine = TagSearchEngine(manager)
    
    # Basic search
    print("\n🔎 Basic search for 'frontend':")
    results = manager.search_tags("frontend")
    for tag in results:
        print(f"  • {tag.name} - {tag.description}")
    
    # Advanced search with multiple filters
    print("\n🔎 Advanced search (category='development', sort by usage):")
    query = TagSearchQuery(
        category="development",
        sort_by=TagSortBy.Usage,
        limit=10
    )
    results = engine.advanced_search(query)
    for tag in results:
        print(f"  • {tag.name} (usage: {tag.usage_count})")
    
    # Search by description
    print("\n🔎 Search by description contains 'testing':")
    query = TagSearchQuery(
        description_contains="testing",
        sort_by=TagSortBy.Name
    )
    results = engine.advanced_search(query)
    for tag in results:
        print(f"  • {tag.name}: {tag.description}")
    
    # Search with usage range
    print("\n🔎 Search with minimum usage (>=2):")
    query = TagSearchQuery(
        min_usage=2,
        sort_by=TagSortBy.Usage
    )
    results = engine.advanced_search(query)
    for tag in results:
        print(f"  • {tag.name}: {tag.usage_count} uses")
    
    # Fuzzy search
    print("\n🔎 Fuzzy search for 'front' (max distance 2):")
    fuzzy_results = engine.fuzzy_search("front", 2)
    for tag, distance in fuzzy_results[:5]:
        print(f"  • {tag.name} (distance: {distance})")
    
    # Get tag suggestions
    print("\n💡 Tag suggestions based on 'frontend':")
    suggestions = engine.get_suggestions(["frontend"], 5)
    for tag_name in suggestions:
        print(f"  • {tag_name}")

async def demonstrate_bulk_operations(manager: TagManager):
    """Demonstrate bulk tag operations"""
    
    print("\n📦 Bulk Operations Demo")
    print("-" * 30)
    
    # Bulk create tags
    print("\n➕ Creating multiple issue tags:")
    issue_tags = ["bug", "enhancement", "feature-request", "documentation", "performance"]
    created_ids = await manager.bulk_create_tags(
        issue_tags,
        category="issue-type"
    )
    print(f"  Created {len(created_ids)} tags")
    
    # Show all tags after bulk creation
    all_tags = manager.get_all_tags()
    print(f"\n📊 Total tags after bulk creation: {len(all_tags)}")
    
    # Demonstrate tag merging
    print("\n🔄 Merging tags...")
    
    # Create a duplicate tag
    await manager.create_tag(
        "frontend-dev",
        description="Duplicate frontend tag",
        category="development"
    )
    
    # Merge the duplicate into frontend
    frontend = manager.get_tag_by_name("frontend")
    frontend_dev = manager.get_tag_by_name("frontend-dev")
    
    if frontend and frontend_dev:
        print(f"  Merging '{frontend_dev.name}' into '{frontend.name}'")
        await manager.merge_tags(frontend.id, [frontend_dev.id])
        
        # Check merged tag
        merged = manager.get_tag(frontend.id)
        print(f"  Merged tag usage count: {merged.usage_count}")
        
        # Verify duplicate is gone
        duplicate_check = manager.get_tag_by_name("frontend-dev")
        print(f"  Duplicate tag exists: {duplicate_check is not None}")

async def demonstrate_tag_workflow():
    """Complete workflow demonstrating practical tag usage"""
    
    print("\n🚀 Complete Tag Workflow Example")
    print("=" * 50)
    
    # Setup
    manager = await demonstrate_tag_operations()
    await demonstrate_search_engine(manager)
    await demonstrate_bulk_operations(manager)
    
    # Final statistics
    print("\n📈 Final Statistics:")
    stats = manager.get_tag_statistics()
    print(f"  Total tags: {stats.total_tags}")
    print(f"  Total categories: {len(manager.get_all_categories())}")
    print(f"  Total relationships: {stats.total_relationships}")
    print(f"  Average usage per tag: {stats.average_usage:.2f}")
    print(f"  Relationships per tag: {stats.relationships_per_tag():.2f}")
    
    # Show recent tags
    print("\n🕒 Recently Created Tags:")
    recent = manager.get_recent_tags(5)
    for tag in recent:
        print(f"  • {tag.name} (created: {tag.created_at.strftime('%H:%M')})")
    
    print("\n✅ Tag management example completed successfully!")

if __name__ == "__main__":
    asyncio.run(demonstrate_tag_workflow())