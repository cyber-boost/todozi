#!/usr/bin/env python3
"""
Example 3 – Tag subsystem & search integration

Shows:
* creating / updating / deleting tags,
* building relationships,
* token‑based and fuzzy search,
* getting tag suggestions for a Todozi task,
* adding a new “tag‑suggest” sub‑command to the Todozi CLI.
"""

# ----------------------------------------------------------------------
# Imports – everything lives in the repository, no external packages needed
# ----------------------------------------------------------------------
import asyncio
import sys
from pathlib import Path

# Ensure the repository root is on sys.path (makes relative imports work)
REPO_ROOT = Path(__file__).resolve().parents[0]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

# Core Todozi pieces
from todozi.todozi import Assignee, Priority, Status
from todozi.models import Task, TaskUpdate, Ok
from todozi.storage import Storage

# Tag subsystem
from tags import TagManager, TagSearchEngine, TagSearchQuery

# ----------------------------------------------------------------------
# Helper – pretty‑print a list of tags
# ----------------------------------------------------------------------
def fmt_tags(tag_list):
    lines = []
    for t in tag_list:
        lines.append(
            f"• {t.name}  (id={t.id})   "
            f"desc={t.description or '-'}   "
            f"cat={t.category or '-'}   "
            f"usage={t.usage_count}"
        )
    return "\n".join(lines)


# ----------------------------------------------------------------------
# Async demo – everything is async because Todozi storage is async
# ----------------------------------------------------------------------
async def demo_tag_workflow():
    print("\n=== 1️⃣  Initialise TagManager & SearchEngine ===")
    tm = TagManager()
    se = TagSearchEngine(tm)

    # ------------------------------------------------------------------
    # 2️⃣  Create a few tags
    # ------------------------------------------------------------------
    print("\n=== 2️⃣  Creating tags ===")
    bug_id = await tm.create_tag(
        name="bug",
        description="A defect that needs fixing",
        color="#ff5555",
        category="dev",
    )
    frontend_id = await tm.create_tag(
        name="frontend",
        description="UI / client‑side code",
        color="#5faf5f",
        category="dev",
    )
    ui_id = await tm.create_tag(
        name="ui",
        description="User‑interface components",
        color="#5fafff",
        category="dev",
    )
    print("Created tags:\n" + fmt_tags([await tm.get_tag(bug_id),
                                      await tm.get_tag(frontend_id),
                                      await tm.get_tag(ui_id)]))

    # ------------------------------------------------------------------
    # 3️⃣  Relate tags (frontend ↔ ui)
    # ------------------------------------------------------------------
    print("\n=== 3️⃣  Adding relationships ===")
    await tm.add_tag_relationship(frontend_id, ui_id)
    await tm.add_tag_relationship(ui_id, frontend_id)   # bi‑directional for demo
    related_to_frontend = tm.get_related_tags(frontend_id)
    print("Tags related to **frontend**:\n" + fmt_tags(related_to_frontend))

    # ------------------------------------------------------------------
    # 4️⃣  Search tags
    # ------------------------------------------------------------------
    print("\n=== 4️⃣  Token‑based search (\"ui\") ===")
    result = tm.search_tags("ui")
    print(fmt_tags(result))

    print("\n=== 4️⃣  Fuzzy search (\"frntend\" – distance ≤ 2) ===")
    fuzzy = se.fuzzy_search("frntend", max_distance=2)
    for tag, dist in fuzzy:
        print(f"  {tag.name} (distance={dist})")

    # ------------------------------------------------------------------
    # 5️⃣  Get suggestions for a new task based on existing tags
    # ------------------------------------------------------------------
    print("\n=== 5️⃣  Tag suggestions for a task ===")
    # Imagine we are about to create a task about fixing a UI bug
    current_tags = ["bug", "ui"]
    suggestions = se.get_suggestions(current_tags, limit=5)
    print("Suggested additional tags:", ", ".join(suggestions))

    # ------------------------------------------------------------------
    # 6️⃣  Persist a task that uses the tags we just created
    # ------------------------------------------------------------------
    print("\n=== 6️⃣  Creating a Todozi task with tags ===")
    # Resolve tag objects → we just need the *names* for the Task model
    task_tags = ["bug", "ui"]
    task = Task.new_full(
        user_id="demo_user",
        action="Fix UI bug where button stays disabled",
        time="2 h",
        priority=Priority.High,
        parent_project="frontend",
        status=Status.Todo,
        assignee=Assignee.human(),
        tags=task_tags,
        dependencies=[],
        context_notes="The button never becomes clickable after form validation.",
        progress=0,
    )
    # Store the task (uses the async storage layer)
    storage = await Storage.new()
    await storage.add_task_to_project(task)
    print(f"✅  Task stored – id={task.id}")

    # ------------------------------------------------------------------
    # 7️⃣  BONUS – extend TodoziHandler with a **tag‑suggest** command
    # ------------------------------------------------------------------
    print("\n=== 7️⃣  Registering a custom CLI sub‑command ===")
    # The real Todozi CLI lives in `todozi.py`.  We monkey‑patch its
    # `CommandRegistry` after it has been built.
    from types import SimpleNamespace
    from todozi import main as todozi_main, build_registry, CommandContext, TodoziHandler

    # Our handler method – it will be called by the registry.
    async def handle_tag_suggest(args: SimpleNamespace, ctx: CommandContext):
        """CLI:  todozi tag-suggest --tags bug,ui --limit 3"""
        # Parse user‑provided comma‑separated tags
        user_tags = [t.strip() for t in args.tags.split(",") if t.strip()]
        limit = int(args.limit) if args.limit else 5
        suggestions = se.get_suggestions(user_tags, limit=limit)
        print("\nTag suggestions based on:", ", ".join(user_tags))
        print(" →", ", ".join(suggestions) if suggestions else "none")
        return None

    # ------------------------------------------------------------------
    #   a) Build the original registry (the function lives in todozi.py)
    # ------------------------------------------------------------------
    # We need a running storage + handler for the context
    handler = TodoziHandler(storage)
    registry = build_registry(handler, storage, Path.cwd())

    # ------------------------------------------------------------------
    #   b) Add our new sub‑command “tag-suggest”
    # ------------------------------------------------------------------
    import argparse
    tag_parser = argparse.ArgumentParser(
        prog="todozi",
        description="Todozi CLI (extended with tag‑suggest)",
    )
    tag_parser.add_argument(
        "--tags", required=True, help="Comma‑separated list of existing tags"
    )
    tag_parser.add_argument(
        "--limit", type=int, default=5, help="Maximum number of suggestions"
    )
    # Register under a new top‑level name “tag‑suggest”
    registry.register(
        name="tag-suggest",
        parser=tag_parser,
        handler=handle_tag_suggest,
    )
    print("✅  Custom command `tag-suggest` registered.\n")

    # ------------------------------------------------------------------
    #   c) Run a tiny demo of the new CLI command **without exiting the script**
    # ------------------------------------------------------------------
    # Simulate `todozi tag-suggest --tags bug,ui --limit 3`
    demo_argv = ["tag-suggest", "--tags", "bug,ui", "--limit", "3"]
    print("Running simulated CLI call:", " ".join(demo_argv))
    # We have to rebuild the ArgumentParser used by Todozi’s `main`.
    # The helper `todozi_main` builds its own parser, but we can
    # reuse the registry we just patched.
    sys.argv = ["todozi"] + demo_argv
    # Directly call Todozi’s entry‑point (it will use our patched registry)
    exit_code = todozi_main()
    print("\nCLI exited with code:", exit_code)


# ----------------------------------------------------------------------
# Entrypoint – run the demo
# ----------------------------------------------------------------------
if __name__ == "__main__":
    try:
        asyncio.run(demo_tag_workflow())
    except KeyboardInterrupt:
        print("\nDemo cancelled by user.", file=sys.stderr)
        sys.exit(130)