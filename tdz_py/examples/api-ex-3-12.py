#!/usr/bin/env python3
"""
Demo: Tag creation, linking, searching and integration with TodoziHandler.

Run:
    $ python tag_demo.py
"""

from __future__ import annotations

import asyncio
import sys
from pathlib import Path

# ----------------------------------------------------------------------
# 1️⃣  Imports from the Todozi package
# ----------------------------------------------------------------------
# The repository uses a *flat* import layout, so we need to make sure the
# parent directory is on PYTHONPATH (the same trick used inside the
# package modules).
repo_root = Path(__file__).resolve().parent.parent
if str(repo_root) not in sys.path:
    sys.path.insert(0, str(repo_root))

from todozi.tags import (
    TagManager,
    TagUpdate,
    TagSearchEngine,
    TagSearchQuery,
    TagSortBy,
    levenshtein_distance,
)
from todozi.cli import TodoziHandler, CommandContext
from todozi.storage import Storage  # the high‑level storage wrapper
from todozi.types import ListTags, ShowTags  # tiny command structs (see types.py)

# ----------------------------------------------------------------------
# 2️⃣  Helper: run an async function from sync code
# ----------------------------------------------------------------------
def run(coro):
    """Simple wrapper that runs an async coroutine and returns its result."""
    return asyncio.get_event_loop().run_until_complete(coro)


# ----------------------------------------------------------------------
# 3️⃣  Tag creation & linking
# ----------------------------------------------------------------------
async def build_tags() -> TagManager:
    """Create a fresh TagManager, populate it and return the instance."""
    manager = TagManager()

    # ---- a) Bulk‑create a “workflow” vocabulary ---------------------------------
    workflow_tags = [
        "bug",
        "feature",
        "task",
        "refactor",
        "urgent",
        "blocked",
    ]
    await manager.bulk_create_tags(workflow_tags, category="workflow")
    print("✅ Created workflow tags")

    # ---- b) Single‑create a few coloured, described tags -----------------------
    await manager.create_tag(
        name="frontend",
        description="User‑facing UI code",
        color="#1f8ef1",
        category="component",
    )
    await manager.create_tag(
        name="backend",
        description="Server‑side logic & APIs",
        color="#e14eca",
        category="component",
    )
    await manager.create_tag(
        name="ui",
        description="UI‑specific pieces (CSS, HTML, React)",
        color="#00f2c3",
        category="subcomponent",
    )
    print("✅ Created component tags (frontend, backend, ui)")

    # ---- c) Link related tags -------------------------------------------------
    # “frontend” is strongly related to “ui”, “backend”
    fe_id = manager.get_tag_by_name("frontend").id
    ui_id = manager.get_tag_by_name("ui").id
    be_id = manager.get_tag_by_name("backend").id

    await manager.add_tag_relationship(fe_id, ui_id)   # frontend ↔ ui
    await manager.add_tag_relationship(fe_id, be_id)   # frontend ↔ backend
    await manager.add_tag_relationship(ui_id, be_id)   # ui ↔ backend (cross‑cut)

    print("🔗 Added tag relationships")

    return manager


# ----------------------------------------------------------------------
# 4️⃣  Searching & fuzzy matching
# ----------------------------------------------------------------------
def demo_search(manager: TagManager):
    # ---- a) Simple token search ------------------------------------------------
    results = manager.search_tags("front")
    print("\n🔎 Token search for 'front'")
    for t in results:
        print(f"  • {t.name}  ({t.category})")

    # ---- b) Advanced filtered search -----------------------------------------
    query = TagSearchQuery(
        name_contains="bug",
        category="workflow",
        min_usage=0,
        sort_by=TagSortBy.Name,
        limit=5,
    )
    engine = TagSearchEngine(manager)
    adv = engine.advanced_search(query)
    print("\n🧠 Advanced search (name contains ‘bug’, category workflow)")
    for t in adv:
        print(f"  • {t.name}")

    # ---- c) Fuzzy search (Levenshtein ≤ 2) ------------------------------------
    fuzzy = engine.fuzzy_search("urgen", max_distance=2)
    print("\n🤏 Fuzzy search for typo ‘urgen’")
    for tag, dist in fuzzy:
        print(f"  • {tag.name} (dist ={dist})")

    # ---- d) AI‑style suggestions (based on relationships) --------------------
    suggestions = engine.get_suggestions(["frontend"], limit=5)
    print("\n💡 Tag suggestions for ‘frontend’")
    for s in suggestions:
        print(f"  • {s}")


# ----------------------------------------------------------------------
# 5️⃣  Integrating the tags with the Todozi CLI (list‑most‑used)
# ----------------------------------------------------------------------
async def list_most_used_tags_via_cli():
    """Show how a normal Todozi command can reuse the TagManager."""
    # Build the context that the CLI expects
    storage = await Storage.new()
    handler = TodoziHandler(storage)

    # A tiny “command” – we reuse the same dataclass pattern that the CLI uses
    cmd = ListTags(limit=10)  # imported from todozi.types (you may rename)

    # ------------------------------------------------------------------
    # In the real CLI this would be routed via `handle_tag_command()`.
    # Here we perform the same logic manually for clarity.
    # ------------------------------------------------------------------
    manager = await build_tags()  # reuse the same tags we built earlier
    top_tags = manager.get_most_used_tags(limit=cmd.limit)

    print("\n📊 Most‑used tags (via a simulated Todozi command)")
    for t in top_tags:
        print(f"  • {t.name} – usage {t.usage_count}")

    # If you wanted to *actually* expose it as a sub‑command you could
    # add a small wrapper in `cli.py` that calls the same code.

# ----------------------------------------------------------------------
# 6️⃣  Main entry point
# ----------------------------------------------------------------------
def main():
    manager = run(build_tags())

    # Demonstrate search/fuzzy/tag‑suggestion capabilities
    demo_search(manager)

    # Show how the tag data could be consumed by a normal Todozi command
    run(list_most_used_tags_via_cli())


if __name__ == "__main__":
    main()