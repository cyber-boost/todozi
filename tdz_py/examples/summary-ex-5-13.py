# summary_example.py
from summary import Summary, SummaryManager, SummaryPriority, parse_summary_format

def main():
    # Initialize the summary manager
    manager = SummaryManager()
    
    # Create summaries with various tags
    summaries = [
        Summary(
            content="Complete project documentation",
            context="Final project phase",
            priority=SummaryPriority.High,
            tags=["documentation", "project", "urgent"]
        ),
        Summary(
            content="Fix authentication bug",
            context="User login issues",
            priority=SummaryPriority.Critical,
            tags=["bug", "security", "authentication"]
        ),
        Summary(
            content="Design new API endpoints",
            context="API expansion project",
            priority=SummaryPriority.Medium,
            tags=["api", "design", "backend"]
        ),
        Summary(
            content="Update user interface",
            context="UI/UX improvements",
            priority=SummaryPriority.Medium,
            tags=["ui", "frontend", "design"]
        ),
        Summary(
            content="Set up deployment pipeline",
            context="CI/CD implementation",
            priority=SummaryPriority.High,
            tags=["devops", "deployment", "automation"]
        )
    ]
    
    # Add summaries to manager
    summary_ids = []
    for summary in summaries:
        summary_id = manager.create_summary(summary)
        summary_ids.append(summary_id)
        print(f"✅ Created summary: {summary.content} (ID: {summary_id})")
    
    print("\n" + "="*50)
    
    # Search summaries by specific tags
    print("🔍 Searching summaries by tag 'design':")
    design_summaries = manager.get_summaries_by_tag("design")
    for summary in design_summaries:
        print(f"  - {summary.content} (Priority: {summary.priority.value})")
    
    print("\n🔍 Searching summaries by tag 'security':")
    security_summaries = manager.get_summaries_by_tag("security")
    for summary in security_summaries:
        print(f"  - {summary.content} (Priority: {summary.priority.value})")
    
    print("\n" + "="*50)
    
    # Get summaries by priority
    print("📊 High priority summaries:")
    high_priority = manager.get_summaries_by_priority(SummaryPriority.High)
    for summary in high_priority:
        print(f"  - {summary.content}")
    
    print(f"\n📊 Critical priority summaries:")
    critical_priority = manager.get_summaries_by_priority(SummaryPriority.Critical)
    for summary in critical_priority:
        print(f"  - {summary.content}")
    
    print("\n" + "="*50)
    
    # Get all tags
    all_tags = manager.get_all_tags()
    print("🏷️ All unique tags across summaries:")
    for tag in all_tags:
        print(f"  - {tag}")
    
    # Get tag statistics
    tag_stats = manager.get_tag_statistics()
    print(f"\n📈 Tag usage statistics:")
    for tag, count in tag_stats.items():
        print(f"  - {tag}: used in {count} summaries")
    
    print("\n" + "="*50)
    
    # Get comprehensive summary statistics
    stats = manager.get_summary_statistics()
    print("📊 Summary Manager Statistics:")
    print(f"  Total summaries: {stats.total_summaries}")
    print(f"  High priority summaries: {stats.high_priority_summaries}")
    print(f"  High priority percentage: {stats.high_priority_percentage():.1f}%")
    print(f"  Unique tags: {stats.unique_tags}")
    
    print("\n" + "="*50)
    
    # Search summaries by keyword
    print("🔍 Searching summaries containing 'project':")
    search_results = manager.search_summaries("project")
    for summary in search_results:
        print(f"  - {summary.content} (Tags: {', '.join(summary.tags)})")
    
    print("\n" + "="*50)
    
    # Demonstrate text parsing functionality
    print("📝 Testing summary format parsing:")
    summary_text = "<summary>Database optimization; high; Performance improvements; database,performance,optimization</summary>"
    try:
        parsed_summary = parse_summary_format(summary_text)
        print(f"✅ Parsed successfully:")
        print(f"  Content: {parsed_summary.content}")
        print(f"  Priority: {parsed_summary.priority.value}")
        print(f"  Context: {parsed_summary.context}")
        print(f"  Tags: {parsed_summary.tags}")
    except Exception as e:
        print(f"❌ Parsing failed: {e}")
    
    print("\n" + "="*50)
    
    # Update a summary
    print("🔄 Updating a summary:")
    if summary_ids:
        update_id = summary_ids[0]
        from summary import SummaryUpdate
        
        # Create update with new content and tags
        updates = SummaryUpdate()
        updates.content_set("Complete project documentation with examples")
        updates.tags_set(["documentation", "project", "examples", "complete"])
        
        manager.update_summary(update_id, updates)
        updated_summary = manager.get_summary(update_id)
        if updated_summary:
            print(f"✅ Updated summary:")
            print(f"  New content: {updated_summary.content}")
            print(f"  New tags: {', '.join(updated_summary.tags)}")

if __name__ == "__main__":
    main()