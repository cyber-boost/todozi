# example_summary_usage.py
from summary import (
    Summary, SummaryUpdate, SummaryPriority, SummaryManager, parse_summary_format
)

def main():
    # Create a summary manager
    manager = SummaryManager()

    # Example 1: Create a new summary
    summary1 = Summary(
        content="Project planning meeting notes",
        context="Q3 planning session",
        priority=SummaryPriority.High,
        tags=["planning", "Q3", "meeting"]
    )
    summary1_id = manager.create_summary(summary1)
    print(f"Created summary with ID: {summary1_id}")

    # Example 2: Create another summary using parse function
    summary_text = "<summary>Code review guidelines; medium; Development standards; review,coding</summary>"
    summary2 = parse_summary_format(summary_text)
    summary2_id = manager.create_summary(summary2)
    print(f"Parsed and created summary with ID: {summary2_id}")

    # Example 3: Retrieve summaries
    retrieved_summary = manager.get_summary(summary1_id)
    if retrieved_summary:
        print(f"\nRetrieved summary: {retrieved_summary.content}")
        print(f"Priority: {retrieved_summary.priority}")
        print(f"Tags: {retrieved_summary.tags}")

    # Example 4: Update a summary
    update = SummaryUpdate().with_content("Updated project planning notes").with_priority(SummaryPriority.Critical)
    manager.update_summary(summary1_id, update)
    updated_summary = manager.get_summary(summary1_id)
    if updated_summary:
        print(f"\nUpdated summary content: {updated_summary.content}")
        print(f"New priority: {updated_summary.priority}")

    # Example 5: Search summaries
    search_results = manager.search_summaries("planning")
    print(f"\nFound {len(search_results)} summaries matching 'planning':")
    for summary in search_results:
        print(f"- {summary.content}")

    # Example 6: Get summaries by priority
    high_priority_summaries = manager.get_summaries_by_priority(SummaryPriority.High)
    print(f"\nFound {len(high_priority_summaries)} high priority summaries")

    # Example 7: Get summaries by tag
    planning_summaries = manager.get_summaries_by_tag("planning")
    print(f"Found {len(planning_summaries)} summaries with 'planning' tag")

    # Example 8: Get statistics
    stats = manager.get_summary_statistics()
    print(f"\nSummary Statistics:")
    print(f"Total summaries: {stats.total_summaries}")
    print(f"High priority summaries: {stats.high_priority_summaries}")
    print(f"Unique tags: {stats.unique_tags}")
    print(f"High priority percentage: {stats.high_priority_percentage():.2f}%")

    # Example 9: Get all tags
    all_tags = manager.get_all_tags()
    print(f"\nAll tags: {all_tags}")

    # Example 10: Get tag statistics
    tag_stats = manager.get_tag_statistics()
    print(f"\nTag statistics: {tag_stats}")

if __name__ == "__main__":
    main()