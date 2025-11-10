#!/usr/bin/env python3
"""
Demo: 1️⃣  Run a Todozi “content‑to‑JSON” conversion with `tdz_cnt`.
     2️⃣  Store the extracted items in a simple Tag‑based index.
"""

import asyncio
import json
import sys
from pathlib import Path

# ----------------------------------------------------------------------
# 1️⃣  Load the high‑level processor (tdz_cnt) from the library
# ----------------------------------------------------------------------
# The function lives in todozi/tdz_tls.py – we import it via the package.
from todozi.tdz_tls import tdz_cnt  # ← the async helper you already have

# ----------------------------------------------------------------------
# 2️⃣  Load the TagManager (fast in‑memory tag DB)
# ----------------------------------------------------------------------
from todozi.tags import TagManager, TagUpdate, TagSearchQuery, TagSortBy

# ----------------------------------------------------------------------
# Helper: pretty‑print JSON with colours (optional, just for the demo)
# ----------------------------------------------------------------------
def _pretty(json_str: str) -> str:
    try:
        # Use the builtin json module with indentation
        obj = json.loads(json_str)
        return json.dumps(obj, indent=2, ensure_ascii=False)
    except Exception:                     # pragma: no cover
        return json_str

# ----------------------------------------------------------------------
# 3️⃣  Main async entry point
# ----------------------------------------------------------------------
async def main():
    # --------------------------------------------------------------
    # a) Sample message that contains many Todozi tags
    # --------------------------------------------------------------
    raw_message = """
    Hey team, we need to finish the Q3 report.

    <todozi>Write Q3 report; 2h; high; marketing; todo; assignee=human; tags=report,marketing; dependencies=Gather data; context_notes=Use latest figures</todozi>

    <memory>standard; 2024‑03‑15; Discussed KPI targets; Need to align with finance; medium; short; kpi,finance</memory>

    <idea>Launch new referral program; team; high; We could give 10 % credit for each referred signup</idea>

    <todozi_agent>task-001; agent‑planner; marketing</todozi_agent>

    <chunk>def calculate_roi(cost, revenue):\n    return (revenue - cost) / cost</chunk>

    <error>Report generation failed; Could not find template; high; runtime; reporting</error>

    <train>instruction; Generate a short email confirming a meeting; Dear {name},\n\nYour meeting is confirmed for {date} at {time}.\n\nBest,\nTeam</train>

    <feel>excited; 9; Got the green light for the new campaign!; marketing; motivated</feel>
    """

    # --------------------------------------------------------------
    # b) Run the high‑level processor – it returns a JSON string
    # --------------------------------------------------------------
    print("\n=== 🟦 Running tdz_cnt (high‑level processor) ===")
    json_result = await tdz_cnt(raw_message, session_id="demo‑session")
    print(_pretty(json_result))

    # --------------------------------------------------------------
    # c) Parse the JSON back to a Python dict so we can work with it
    # --------------------------------------------------------------
    data = json.loads(json_result)

    # --------------------------------------------------------------
    # d) Initialise a TagManager (in‑memory, async‑safe)
    # --------------------------------------------------------------
    tag_mgr = TagManager()

    # --------------------------------------------------------------
    # e) Helper: create a tag for every distinct entity we just parsed.
    #    For a real‑world app you would probably have richer rules,
    #    but this keeps the demo simple.
    # --------------------------------------------------------------
    async def _create_tag(name: str, description: str = "", category: str = "demo"):
        """Create a tag and return its id (or reuse if it already exists)."""
        # The manager guarantees unique names – we just ignore the exception
        try:
            tag_id = await tag_mgr.create_tag(name,
                                              description=description,
                                              category=category)
        except Exception:  # pragma: no cover – name already exists
            tag = tag_mgr.get_tag_by_name(name)
            tag_id = tag.id if tag else None
        return tag_id

    # ------------------------------------------------------------------
    # f) Create tags for the extracted objects
    # ------------------------------------------------------------------
    #   – one tag per task, memory, idea, agent‑assignment, code‑chunk,
    #     error and feeling.  The tag name is the human‑readable title.
    # ------------------------------------------------------------------
    tag_ids = []   # keep the ids so we can later relate them

    # 1️⃣  Tasks
    for task in data.get("clean_with_response", {}):
        # The `clean_with_response` field contains the processed text;
        # the JSON object also has a list `processed_items`.
        # The easier way is to look at `processed_items` which contains the
        # textual representation of each extracted item.
        # We'll just create a tag for each item in `processed_items`.
        pass  # No‑op – we already have a richer list below

    # The detailed lists are in the top‑level keys:
    for t in data.get("processed_items", []):
        # Example item: "Task: Write Q3 report"
        if t.lower().startswith("task:"):
            title = t.split(":", 1)[1].strip()
            tid = await _create_tag(title, description="Task from tdz_cnt", category="task")
            tag_ids.append(tid)

    # 2️⃣  Memories
    for mem in data.get("clean_with_response", {}):  # placeholder – real data in `memories`
        pass

    # Actually the JSON already contains a separate `processed_items` count,
    # but the full extracted objects live under the keys that match the
    # original tags: `tasks`, `memories`, `ideas`, `agent_assignments`,
    # `code_chunks`, `errors`, `training_data`, `feelings`.
    # We will iterate over each collection and create a tag for it.

    # ----- Tasks --------------------------------------------------------
    for task in data.get("tasks", []):
        # Each entry is a dict with keys `action`, `priority`, … (see tdz_cnt)
        title = task.get("action", "Unnamed Task")
        tid = await _create_tag(title,
                                 description=f"Priority: {task.get('priority')}",
                                 category="task")
        tag_ids.append(tid)

    # ----- Memories -----------------------------------------------------
    for mem in data.get("memories", []):
        title = mem.get("moment", "Unnamed Memory")
        tid = await _create_tag(title,
                                 description=mem.get("meaning", ""),
                                 category="memory")
        tag_ids.append(tid)

    # ----- Ideas --------------------------------------------------------
    for idea in data.get("ideas", []):
        title = idea.get("idea", "Unnamed Idea")
        tid = await _create_tag(title,
                                 description=f"Share: {idea.get('share')}",
                                 category="idea")
        tag_ids.append(tid)

    # ----- Agent assignments --------------------------------------------
    for assign in data.get("agent_assignments", []):
        aid = assign.get("agent_id", "unknown")
        t_id = assign.get("task_id", "unknown")
        title = f"Assign {t_id} → {aid}"
        tid = await _create_tag(title,
                                 description="Agent assignment",
                                 category="agent")
        tag_ids.append(tid)

    # ----- Code chunks ---------------------------------------------------
    for chunk in data.get("code_chunks", []):
        # We just store the language as the tag name
        lang = chunk.get("lang", "unknown")
        title = f"Code chunk ({lang})"
        tid = await _create_tag(title,
                                 description=chunk.get("code", "")[:30] + "...",
                                 category="code")
        tag_ids.append(tid)

    # ----- Errors --------------------------------------------------------
    for err in data.get("errors", []):
        title = err.get("title", "Unnamed Error")
        tid = await _create_tag(title,
                                 description=err.get("detail", ""),
                                 category="error")
        tag_ids.append(tid)

    # ----- Feelings ------------------------------------------------------
    for feel in data.get("feelings", []):
        title = f"{feel.get('emotion', 'unknown')} ({feel.get('intensity')})"
        tid = await _create_tag(title,
                                 description=feel.get("detail", ""),
                                 category="feeling")
        tag_ids.append(tid)

    # ------------------------------------------------------------------
    # g) Show the created tags
    # ------------------------------------------------------------------
    print("\n=== 🏷️  Tags created in the TagManager ===")
    all_tags = tag_mgr.get_all_tags()
    for t in all_tags:
        print(f"- {t.name}  (id={t.id})  category={t.category}")

    # ------------------------------------------------------------------
    # h) Demonstrate a *search* – find everything that mentions “report”
    # ------------------------------------------------------------------
    print("\n=== 🔎 Search for the word 'report' ===")
    results = tag_mgr.search_tags("report")
    for t in results:
        print(f"* {t.name}  →  {t.description}")

    # ------------------------------------------------------------------
    # i) Demonstrate a *related‑tag* query – we will relate the first
    #    “task” tag to the first “memory” tag, then ask the engine for
    #    suggestions based on that relationship.
    # ------------------------------------------------------------------
    if len(tag_ids) >= 2:
        task_tag_id = tag_ids[0]          # first tag we created (most likely the task)
        memory_tag_id = tag_ids[1]        # second tag (a memory)
        await tag_mgr.add_tag_relationship(task_tag_id, memory_tag_id)
        related = tag_mgr.get_related_tags(task_tag_id)
        print("\n=== 🔗 Tags related to the first tag ===")
        for r in related:
            print(f"- {r.name} (category={r.category})")

        # Use the *search engine* `TagSearchEngine` to get suggestions
        from todozi.tags import TagSearchEngine
        engine = TagSearchEngine(tag_mgr)
        suggestions = engine.get_suggestions([tag_mgr.get_tag(task_tag_id).name], limit=5)
        print("\n=== 💡 Tag suggestions based on the first tag ===")
        for s in suggestions:
            print(f"* {s}")

    # ------------------------------------------------------------------
    # j) Show some statistics about the tag store
    # ------------------------------------------------------------------
    stats = tag_mgr.get_tag_statistics()
    print("\n=== 📊 Tag statistics ===")
    print(f"Total tags          : {stats.total_tags}")
    print(f"Total categories    : {stats.total_categories}")
    print(f"Total relationships : {stats.total_relationships}")
    print(f"Average usage       : {stats.average_usage:.2f}")

# ----------------------------------------------------------------------
# Run the async main() when the script is executed directly
# ----------------------------------------------------------------------
if __name__ == "__main__":
    # Because the demo uses async/await we have to run the event‑loop.
    # `asyncio.run()` is the modern, simplest way.
    try:
        asyncio.run(main())
    except KeyboardInterrupt:                     # pragma: no cover
        sys.exit(130)