#!/usr/bin/env python3
"""
Example 3 –  Using Todozi’s SummaryManager

Shows:
  • parsing the <summary> mini‑language,
  • creating, reading, updating, searching & deleting summaries,
  • obtaining simple statistics.

Run:
    $ python example3_summary_manager.py
"""

# ----------------------------------------------------------------------
# Imports (all from the repository, no third‑party packages)
# ----------------------------------------------------------------------
from __future__ import annotations

import uuid
from typing import List

# The code you asked about lives in `summary.py`.
# We import the public API that we need.
from summary import (
    Summary,
    SummaryPriority,
    SummaryManager,
    SummaryUpdate,
    SummaryStatistics,
    parse_summary_format,
)

# ----------------------------------------------------------------------
# Helper – pretty‑print a summary (makes the demo output nicer)
# ----------------------------------------------------------------------
def fmt(summary: Summary) -> str:
    """Return a one‑line human readable description of a Summary."""
    tags = ", ".join(summary.tags) if summary.tags else "(no tags)"
    ctx = f" — {summary.context}" if summary.context else ""
    return (
        f"[{summary.id}] {summary.content!r} "
        f"(priority={summary.priority.value}, tags={tags}){ctx}"
    )


# ----------------------------------------------------------------------
# 1️⃣  Parse raw <summary> strings into Summary objects
# ----------------------------------------------------------------------
RAW_SUMMARIES = [
    # A high‑priority release‑note style summary
    "<summary>Release v2.1 – full‑text search added; high; Release notes; search,release</summary>",

    # A medium priority “idea” that we’ll later promote
    "<summary>Consider adding a Markdown renderer for notes; medium; UI/UX brainstorm; markdown,ui</summary>",

    # A low‑priority reminder (no tags, no context)
    "<summary>Refactor the storage layer to use async I/O; low</summary>",
]

parsed: List[Summary] = [parse_summary_format(txt) for txt in RAW_SUMMARIES]

print("\n📝 Parsed summaries (raw → dataclass):")
for s in parsed:
    print(f"   {fmt(s)}")

# ----------------------------------------------------------------------
# 2️⃣  Create a SummaryManager and persist the summaries
# ----------------------------------------------------------------------
manager = SummaryManager()

print("\n🚀 Adding summaries to the manager...")
ids = []  # keep the generated ids so we can look them up later
for s in parsed:
    sid = manager.create_summary(s)
    ids.append(sid)
    print(f"   created id={sid}")

# ----------------------------------------------------------------------
# 3️⃣  Retrieve a single summary by its id
# ----------------------------------------------------------------------
sample_id = ids[0]  # pick the first one
retrieved = manager.get_summary(sample_id)
print("\n🔍 Retrieved summary:")
print(f"   {fmt(retrieved)}")

# ----------------------------------------------------------------------
# 4️⃣  Update the *second* summary – change its priority and add a tag
# ----------------------------------------------------------------------
second_id = ids[1]
print("\n✏️  Updating second summary (medium → high, add tag ‘priority’)…")
update = SummaryUpdate().priority_set(SummaryPriority.High).tags_set(["markdown", "ui", "priority"])
manager.update_summary(second_id, update)

updated = manager.get_summary(second_id)
print("   after update:")
print(f"   {fmt(updated)}")

# ----------------------------------------------------------------------
# 5️⃣  Search for summaries containing the word “markdown”
# ----------------------------------------------------------------------
keyword = "markdown"
found = manager.search_summaries(keyword)
print(f"\n🔎 Search for keyword '{keyword}':")
for s in found:
    print(f"   {fmt(s)}")

# ----------------------------------------------------------------------
# 6️⃣  Gather statistics about the whole collection
# ----------------------------------------------------------------------
stats: SummaryStatistics = manager.get_summary_statistics()
print("\n📊 Summary statistics")
print(f"   total_summaries          = {stats.total_summaries}")
print(f"   high_priority_summaries  = {stats.high_priority_summaries}")
print(f"   unique_tags              = {stats.unique_tags}")
print(f"   high‑priority %           = {stats.high_priority_percentage():.1f}%")

# ----------------------------------------------------------------------
# 7️⃣  Delete the low‑priority entry and show the final list
# ----------------------------------------------------------------------
low_id = ids[2]
print(f"\n❌ Deleting low‑priority summary (id={low_id}) …")
manager.delete_summary(low_id)

print("\n📋 Remaining summaries after deletion:")
for s in manager.get_all_summaries():
    print(f"   {fmt(s)}")

# ----------------------------------------------------------------------
# End of example
# ----------------------------------------------------------------------
print("\n✅ Demo finished – you can now experiment with the manager in your own code!")