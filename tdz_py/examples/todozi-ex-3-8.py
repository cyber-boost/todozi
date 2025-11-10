#!/usr/bin/env python3
"""
demo_todozi.py  –  Minimal end‑to‑end demo of the Todozi library
"""

# ----------------------------------------------------------------------
# 1️⃣  Imports – only the public symbols we need
# ----------------------------------------------------------------------
import asyncio
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

# The `todozi` package is the one you already have in the repo.
# The `storage` and `todozi` modules expose the high‑level API.
from todozi.storage import Storage, ensure_folder_structure, get_storage_dir
from todozi.todozi import (
    Task,
    Priority,
    Status,
    Assignee,
    parse_todozi_format,
    process_chat_message_extended,
)

# ----------------------------------------------------------------------
# 2️⃣  Helper utilities (pretty printing)
# ----------------------------------------------------------------------
def _to_table(headers, rows):
    """Very small table printer – no external deps."""
    col_widths = [len(h) for h in headers]
    for r in rows:
        for i, cell in enumerate(r):
            col_widths[i] = max(col_widths[i], len(str(cell)))
    sep = "+".join("-" * (w + 2) for w in col_widths)
    sep = f"+{sep}+"

    def fmt_row(row):
        return "|" + "|".join(f" {str(c).ljust(col_widths[i])} " for i, c in enumerate(row)) + "|"

    out = [sep, fmt_row(headers), sep]
    out.extend(fmt_row(r) for r in rows)
    out.append(sep)
    return "\n".join(out)


# ----------------------------------------------------------------------
# 3️⃣  Core demo logic – creates a project + a few tasks
# ----------------------------------------------------------------------
async def _demo_core():
    # 3️⃣①  Make sure the storage folder exists (creates ~/.todozi if missing)
    await ensure_folder_structure()

    # 3️⃣②  Access the high‑level Storage singleton
    storage = await Storage.new()

    # 3️⃣③  ---- create a project -------------------------------------------------
    project_name = "demo-project"
    try:
        storage.create_project(project_name, description="Demo project for the example")
        print(f"✅ Project '{project_name}' created")
    except Exception as e:
        # Project may already exist – that's fine for a demo.
        print(f"⚠️  Project creation failed (maybe it already exists): {e}")

    # 3️⃣④  ---- add a few tasks -------------------------------------------------
    # We can build tasks manually or reuse the parser that handles the <todozi> tag.
    raw_task = "<todozi>Write demo script; 30m; high; demo-project; todo; assignee=human; tags=demo,python; dependencies=none; context_notes=Use the library; progress=0%</todozi>"
    task = parse_todozi_format(raw_task)

    # Store the task via the async storage helper (adds embedding if available)
    await storage.add_task_to_project(task)
    print(f"✅ Task added: {task.id!s[:8]} – {task.action}")

    # Add two more tasks the "hard way" (direct dataclass construction)
    more_tasks = [
        Task(
            action="Review demo output",
            time="15m",
            priority=Priority.Medium,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.human(),
            tags=["review"],
        ),
        Task(
            action="Cleanup temporary files",
            time="5m",
            priority=Priority.Low,
            parent_project=project_name,
            status=Status.Todo,
            assignee=Assignee.ai(),
            tags=["cleanup", "automation"],
        ),
    ]

    for t in more_tasks:
        await storage.add_task_to_project(t)
        print(f"✅ Task added: {t.id[:8]} – {t.action}")

    # 3️⃣⑤  ---- list all tasks for the project ---------------------------------
    filters = storage.storage.TaskFilters()
    filters.project = project_name
    tasks = storage.list_tasks_across_projects(filters)

    # Pretty‑print a small table
    rows = [
        [
            t.id[:8],
            t.action,
            t.time,
            t.priority.name,
            t.status.name,
            t.assignee.kind.name if t.assignee else "none",
        ]
        for t in tasks
    ]
    print("\n📋  Tasks in project:", project_name)
    print(_to_table(["ID", "Action", "Time", "Prio", "Status", "Assignee"], rows))

    # 3️⃣⑥  ---- demo of the “chat” processor (optional) -----------------------
    # The chat parser can pull out any `<todozi>` blocks from a free‑form string.
    chat_msg = """
    Hey bot, can you create these tasks?
    <todozi>Fix typo in README; 2m; low; demo-project; todo</todozi>
    <todozi>Run unit tests; 10m; medium; demo-project; todo; assignee=ai</todozi>
    """
    chat_content = process_chat_message_extended(chat_msg, user_id="demo_user")
    for t in chat_content.tasks:
        await storage.add_task_to_project(t)

    print("\n🤖  Parsed %d tasks from a chat‑style message." % len(chat_content.tasks))

# ----------------------------------------------------------------------
# 4️⃣  EXTENDING THE CLI – a tiny `note` sub‑command
# ----------------------------------------------------------------------
def _handle_note(ns):
    """
    `todozi note "my free‑form note"` stores the note in ~/.todozi/notes/
    The note file name is a timestamp, e.g. 2025-01-01_12-00-00_note.json
    """
    notes_dir = Path.home() / ".todozi" / "notes"
    notes_dir.mkdir(parents=True, exist_ok=True)

    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%d_%H-%M-%S")
    note_path = notes_dir / f"{timestamp}_note.json"

    payload = {
        "timestamp": timestamp,
        "note": ns.note,
        "user": ns.user or "anonymous",
    }
    note_path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    print(f"✅ Note saved to {note_path}")


def _register_note_subcommand(parser):
    """
    Integrate the new `note` command into the existing argparse tree.
    All existing commands keep working unchanged.
    """
    note_parser = parser.add_parser("note", help="Store a quick free‑form note")
    note_parser.add_argument("note", help="The note text")
    note_parser.add_argument("--user", help="Optional user name (defaults to anonymous)")
    note_parser.set_defaults(func=_handle_note)


# ----------------------------------------------------------------------
# 5️⃣  Main entry point – glue everything together
# ----------------------------------------------------------------------
def main(argv: list | None = None) -> int:
    """
    * Build the original Todozi parser (the huge `build_parser()` from `types.py`).
    * Add our extra `note` sub‑command.
    * Parse arguments.
    * If the user selected a built‑in command → run the original dispatcher.
    * If the user selected our `note` command → run our handler.
    """
    # Import the `build_parser` function from the big `types.py` file.
    from types import build_parser, main as todozi_main

    # ------- 1️⃣ Build the original parser ----------
    parser = build_parser()

    # ------- 2️⃣ Register our extra sub‑command -------
    _register_note_subcommand(parser)

    # ------- 3️⃣ Parse the command line ----------
    try:
        ns = parser.parse_args(argv)
    except SystemExit as e:
        # argparse already printed its own error message
        return e.code

    # ------- 4️⃣ Did we hit our custom sub‑command? ----------
    # All built‑in commands use `command` as the top‑level dest.
    # Our custom one uses a dedicated `func` attribute.
    if hasattr(ns, "func"):
        try:
            ns.func(ns)          # ← our custom `note` handler
            return 0
        except Exception as exc:
            print(f"Error while handling note: {exc}", file=sys.stderr)
            return 1

    # ------- 5️⃣ Otherwise run the original Todozi dispatcher (unchanged) ----------
    # The original `todozi.main()` returns an exit code.
    return todozi_main(argv)


# ----------------------------------------------------------------------
# 6️⃣  Run the demo when executed as a script
# ----------------------------------------------------------------------
if __name__ == "__main__":
    # Run the storage‑heavy demo *once* before handing control to the CLI.
    # If you only want the CLI (including the new `note` command) you can comment
    # out the call to `_demo_core()`.
    try:
        asyncio.run(_demo_core())
    except Exception as e:
        print(f"Demo setup failed: {e}", file=sys.stderr)
        sys.exit(1)

    # Hand over to the combined parser/dispatcher.
    sys.exit(main())