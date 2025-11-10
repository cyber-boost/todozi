#!/usr/bin/env python3
"""
Example 1: Summary Management with SummaryManager

This example demonstrates:
- Creating summaries with different priorities and tags
- Updating summaries using SummaryUpdate (fluent and non-fluent APIs)
- Searching summaries by keyword
- Filtering by priority and tag
- Getting recent summaries
- Computing and printing statistics
- Error handling (not found, validation)

To run:
  python example1_summary_usage.py
"""

from __future__ import annotations

import sys
from datetime import datetime, timezone
from typing import List

# Adjust the import path if you place summary.py in a different directory
# from your_package.path.to.summary import SummaryManager, Summary, SummaryUpdate, SummaryPriority
from summary import SummaryManager, Summary, SummaryUpdate, SummaryPriority


def pretty_summary(s: Summary) -> str:
    return (
        f"[{s.id[:8]}] {s.content[:60]}{'...' if len(s.content) > 60 else ''}\n"
        f"  priority={s.priority.value} | tags={', '.join(s.tags) if s.tags else '(none)'} | "
        f"created={s.created_at.date()} | updated={s.updated_at.date()}\n"
    )


def print_list(title: str, items: List[Summary]) -> None:
    print(f"\n{title} ({len(items)} total):")
    if not items:
        print("  (none)")
        return
    for s in items:
        print(pretty_summary(s), end="")


def main() -> int:
    print("Example 1: Summary Management\n" + "=" * 60)

    # 1) Create manager
    manager = SummaryManager()

    # 2) Seed with some summaries
    s1 = Summary(
        content="Design the project kickoff meeting agenda and share with stakeholders.",
        context="Q4 Planning",
        priority=SummaryPriority.High,
        tags=["planning", "meeting"],
    )
    s2 = Summary(
        content="Refactor the authentication module to use a unified error strategy.",
        context="Security improvements",
        priority=SummaryPriority.Critical,
        tags=["backend", "refactor", "security"],
    )
    s3 = Summary(
        content="Write user onboarding documentation for the new dashboard.",
        context="Docs for release 1.2",
        priority=SummaryPriority.Medium,
        tags=["docs", "onboarding"],
    )
    s4 = Summary(
        content="Set up smoke tests for the payment service.",
        context="Stability",
        priority=SummaryPriority.Low,
        tags=["testing", "payments"],
    )
    s5 = Summary(
        content="Prepare Q3 retrospective notes and action items.",
        context="Team retrospective",
        priority=SummaryPriority.Medium,
        tags=["retrospective", "team"],
    )

    ids = [
        manager.create_summary(s1),
        manager.create_summary(s2),
        manager.create_summary(s3),
        manager.create_summary(s4),
        manager.create_summary(s5),
    ]
    print(f"Created {len(ids)} summaries.")

    # 3) Demonstrate SummaryUpdate (fluent API)
    manager.update_summary(
        ids[0],
        SummaryUpdate()
        .with_content("Finalize the project kickoff meeting agenda and share with stakeholders.")
        .with_tags(["planning", "meeting", "communication"]),
    )
    print("Updated s1 content and tags (fluent).")

    # 4) Demonstrate SummaryUpdate (non-fluent API)
    upd = SummaryUpdate()
    upd.priority = SummaryPriority.High
    upd.context = "Q4 Planning - Final"
    manager.update_summary(ids[0], upd)
    print("Updated s1 context and priority (non-fluent).")

    # 5) Search summaries by text
    q = "payment"
    by_search = manager.search_summaries(q)
    print_list(f'Summaries matching "{q}"', by_search)

    # 6) Get summaries by tag
    by_tag = manager.get_summaries_by_tag("security")
    print_list('Summaries with tag "security"', by_tag)

    # 7) Get summaries by priority
    by_prio = manager.get_summaries_by_priority(SummaryPriority.Medium)
    print_list('Summaries with priority "Medium"', by_prio)

    # 8) Get recent summaries (limit 3)
    recent = manager.get_recent_summaries(3)
    print_list("Recent summaries (limit 3)", recent)

    # 9) High priority summaries (High + Critical)
    high = manager.get_high_priority_summaries()
    print_list("High priority summaries (High + Critical)", high)

    # 10) All tags and tag statistics
    tags = manager.get_all_tags()
    print(f"\nAll unique tags ({len(tags)}): {', '.join(tags)}")
    tag_stats = manager.get_tag_statistics()
    print("Tag usage statistics:")
    for tag, count in sorted(tag_stats.items(), key=lambda kv: (-kv[1], kv[0])):
        print(f"  {tag}: {count}")

    # 11) Summary statistics
    stats = manager.get_summary_statistics()
    hp_pct = stats.high_priority_percentage()
    print(
        f"\nSummary statistics:\n"
        f"  total_summaries={stats.total_summaries}\n"
        f"  high_priority_summaries={stats.high_priority_summaries} "
        f"({hp_pct:.1f}%)\n"
        f"  unique_tags={stats.unique_tags}"
    )

    # 12) Error handling
    try:
        manager.update_summary("non-existent-id", SummaryUpdate().with_content("Should fail"))
    except Exception as e:
        print(f"\nHandled expected error: {type(e).__name__}: {e}")

    try:
        manager.delete_summary(ids[0])  # Delete s1
    except Exception as e:
        print(f"Unexpected delete error: {e}")

    # 13) Demonstrate re-search after changes
    recent_after = manager.get_recent_summaries(10)
    print_list("All summaries after updates/deletions", recent_after)

    # 14) Parse a summary from a text format
    # Format: <summary>content; priority; context; tag1,tag2</summary>
    text = "<summary>Prepare monthly report; High; Finance; report,monthly,finance</summary>"
    from summary import parse_summary_format
    parsed = parse_summary_format(text)
    print(
        f"\nParsed summary from format:\n"
        f"  content={parsed.content}\n"
        f"  priority={parsed.priority.value}\n"
        f"  context={parsed.context}\n"
        f"  tags={parsed.tags}"
    )
    new_id = manager.create_summary(parsed)
    print(f"Created summary from parsed text with id={new_id[:8]}")

    print("\n" + "=" * 60)
    print("Example completed successfully.")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\nInterrupted.", file=sys.stderr)
        sys.exit(130)
    except Exception as e:
        print(f"Fatal error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)