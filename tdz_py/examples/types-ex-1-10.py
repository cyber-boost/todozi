#!/usr/bin/env python3
"""
Example CLI for Todozi-style app.

This example demonstrates:
- Building the parser from types.py
- Using the main() dispatcher with custom argv
- Using the SearchEngine for unified search across tasks
- Keeping everything in-memory and immediately useful for testing/extending

Place this file next to types.py and run with Python 3.9+.
"""

import sys
import json
from pathlib import Path
from datetime import datetime, timezone
from typing import List

# Add current directory to path so "import types" works from the same folder
if str(Path(__file__).parent) not in sys.path:
    sys.path.insert(0, str(Path(__file__).parent))

# Import the parser and dispatcher from the provided types.py
from types import (
    build_parser,
    main as dispatch_main,
    ChatContent,
    SearchOptions,
    Task,
    Memory,
    Idea,
    AgentAssignment,
    CodeChunk,
    Error as TDZError,
    TrainingData,
    Feeling,
    SearchResults,
    QueueStatus,
    QueueItem,
    TaskUpdate,
    Commands,
    AddCommands,
    ListCommands,
    ShowCommands,
    SearchCommands,
    StatsCommands,
    ProjectCommands,
    MemoryCommands,
    IdeaCommands,
    AgentCommands,
    EmbCommands,
    ErrorCommands,
    TrainingCommands,
    MaestroCommands,
    ServerCommands,
    MLCommands,
    QueueCommands,
    ApiCommands,
    StepsCommands,
    SearchEngine,
)

# ---------------------------
# Minimal in-memory storage
# ---------------------------

class InMemoryStorage:
    def __init__(self) -> None:
        self.tasks: List[Task] = []
        self.memories: List[Memory] = []
        self.ideas: List[Idea] = []
        self.errors: List[TDZError] = []
        self.training_data: List[TrainingData] = []

    def add_task(self, task: Task) -> None:
        self.tasks.append(task)

    def list_tasks(self) -> List[Task]:
        return list(self.tasks)

    def get_task(self, task_id: str) -> Task:
        for t in self.tasks:
            if t.id == task_id:
                return t
        raise KeyError(f"Task not found: {task_id}")

    def update_task(self, task_id: str, updates: TaskUpdate) -> None:
        t = self.get_task(task_id)
        if updates.action is not None:
            t.action = updates.action
        if updates.time is not None:
            t.time = updates.time
        if updates.priority is not None:
            t.priority = updates.priority
        if updates.project is not None:
            t.project = updates.project
        if updates.status is not None:
            t.status = updates.status
        if updates.assignee is not None:
            t.assignee = updates.assignee
        if updates.tags is not None:
            t.tags = updates.tags
        if updates.dependencies is not None:
            t.dependencies = updates.dependencies
        if updates.context is not None:
            t.context = updates.context
        if updates.progress is not None:
            t.progress = updates.progress

    def complete_task(self, task_id: str) -> None:
        t = self.get_task(task_id)
        t.status = "done"

    def delete_task(self, task_id: str) -> None:
        t = self.get_task(task_id)
        self.tasks.remove(t)


# Global in-memory instance
STORAGE = InMemoryStorage()

# ---------------------------
# Utilities
# ---------------------------

def fake_task_id() -> str:
    return f"task_{datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S')}"


def now_rfc3339() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


# ---------------------------
# Demo handlers wired into the types.py dispatcher via monkey patching
# ---------------------------

# We’ll inject handlers by replacing the function names in the types module’s global namespace.
# This avoids duplicating the full argparse setup and keeps the example succinct.

import types as todozi_types

def handle_init(ns):
    print("Initialized example workspace (in-memory).")

def handle_add(ns):
    if ns.add_sub == AddCommands.TASK.value:
        task = Task(
            id=fake_task_id(),
            action=ns.action,
            time=ns.time,
            priority=ns.priority,
            project=ns.project,
            status=ns.status,
            assignee=ns.assignee,
            tags=ns.tags,
            dependencies=ns.dependencies,
            context=ns.context,
            progress=ns.progress,
        )
        STORAGE.add_task(task)
        print(f"✅ Task created: {task.id}")
        print(f"   Action: {task.action}")
        print(f"   Time: {task.time}")
        print(f"   Priority: {task.priority}")
        print(f"   Project: {task.project}")
        print(f"   Status: {task.status}")

def handle_list(ns):
    if ns.list_sub == ListCommands.TASKS.value:
        tasks = STORAGE.list_tasks()
        if not tasks:
            print("No tasks found.")
            return
        print(f"Found {len(tasks)} task(s):")
        for t in tasks:
            print(f"- [{t.id}] {t.action} (status: {t.status}, priority: {t.priority}, project: {t.project})")

def handle_show(ns):
    if ns.show_sub == ShowCommands.TASK.value:
        try:
            t = STORAGE.get_task(ns.id)
            print(f"Task: {t.id}")
            print(f"  Action: {t.action}")
            print(f"  Time: {t.time}")
            print(f"  Priority: {t.priority}")
            print(f"  Project: {t.project}")
            print(f"  Status: {t.status}")
            print(f"  Assignee: {t.assignee}")
            print(f"  Tags: {t.tags}")
            print(f"  Dependencies: {t.dependencies}")
            print(f"  Context: {t.context}")
            print(f"  Progress: {t.progress}")
        except KeyError as e:
            print(f"Error: {e}", file=sys.stderr)

def handle_update(ns):
    updates = TaskUpdate(id=ns.id)
    updates.action = ns.action
    updates.time = ns.time
    updates.priority = ns.priority
    updates.project = ns.project
    updates.status = ns.status
    updates.assignee = ns.assignee
    updates.tags = ns.tags
    updates.dependencies = ns.dependencies
    updates.context = ns.context
    updates.progress = ns.progress
    try:
        STORAGE.update_task(ns.id, updates)
        print(f"✅ Task {ns.id} updated successfully!")
    except KeyError:
        print(f"Error: Task {ns.id} not found", file=sys.stderr)

def handle_complete(ns):
    try:
        STORAGE.complete_task(ns.id)
        print(f"✅ Task {ns.id} completed!")
    except KeyError:
        print(f"Error: Task {ns.id} not found", file=sys.stderr)

def handle_delete(ns):
    try:
        STORAGE.delete_task(ns.id)
        print(f"✅ Task {ns.id} deleted!")
    except KeyError:
        print(f"Error: Task {ns.id} not found", file=sys.stderr)

def handle_search_all(ns):
    engine = SearchEngine()
    content = ChatContent(
        tasks=STORAGE.list_tasks(),
        memories=[],
        ideas=[],
        agent_assignments=[],
        code_chunks=[],
        errors=[],
        training_data=[],
        feelings=[],
    )
    engine.update_index(content)
    options = SearchOptions(limit=20, data_types=ns.types, since=None, until=None)
    results = engine.search(ns.query, options)

    print(f"Search results for '{ns.query}' (types={ns.types}):")
    if results.task_results:
        print("- Tasks:")
        for t in results.task_results:
            print(f"  [{t.id}] {t.action} (status: {t.status})")
    if results.memory_results:
        print("- Memories:")
        for m in results.memory_results:
            print(f"  {m.moment} -> {m.meaning}")
    if results.idea_results:
        print("- Ideas:")
        for i in results.idea_results:
            print(f"  {i.idea}")
    if results.error_results:
        print("- Errors:")
        for e in results.error_results:
            print(f"  {e.title} - {e.description}")
    if results.training_results:
        print("- Training data:")
        for tr in results.training_results:
            print(f"  {tr.prompt}")

def handle_project(ns):
    # This demo doesn't persist projects; just list known 'general'
    if ns.project_sub == ProjectCommands.LIST.value:
        print("Projects (example):")
        print("- general (built-in)")
    else:
        print("(project subcommand demo placeholder)")

def handle_search(ns):
    if ns.search_sub == SearchCommands.TASKS.value:
        # Simple substring search for demo purposes
        q = ns.query.lower()
        tasks = [t for t in STORAGE.list_tasks() if q in t.action.lower()]
        if not tasks:
            print(f"No tasks found matching '{ns.query}'")
        else:
            print(f"Found {len(tasks)} task(s) matching '{ns.query}':")
            for t in tasks:
                print(f"- [{t.id}] {t.action} (status: {t.status})")

def handle_stats(ns):
    print("📊 Example Statistics:")
    print(f"  Total tasks: {len(STORAGE.list_tasks())}")

# Monkey-patch handlers into the types module so the dispatcher can call them
todozi_types.handle_init = handle_init
todozi_types.handle_add = handle_add
todozi_types.handle_list = handle_list
todozi_types.handle_show = handle_show
todozi_types.handle_update = handle_update
todozi_types.handle_complete = handle_complete
todozi_types.handle_delete = handle_delete
todozi_types.handle_search_all = handle_search_all
todozi_types.handle_project = handle_project
todozi_types.handle_search = handle_search
todozi_types.handle_stats = handle_stats

# Minimal stubs for the rest (no-ops)
def noop_handler(ns):
    print(f"(No-op handler for command: {getattr(ns, 'command', '?')})")

for name in [
    "handle_fix_consistency",
    "handle_check_structure",
    "handle_ensure_structure",
    "handle_register",
    "handle_registration_status",
    "handle_clear_registration",
    "handle_backup",
    "handle_list_backups",
    "handle_restore",
    "handle_memory",
    "handle_idea",
    "handle_agent",
    "handle_emb",
    "handle_error",
    "handle_train",
    "handle_chat",
    "handle_maestro",
    "handle_server",
    "handle_ml",
    "handle_ind_demo",
    "handle_queue",
    "handle_api",
    "handle_tdzcnt",
    "handle_export_embeddings",
    "handle_migrate",
    "handle_tui",
    "handle_extract",
    "handle_strategy",
    "handle_steps",
]:
    setattr(todozi_types, name, noop_handler)


# ---------------------------
# Convenience function to run the CLI programmatically with custom args
# ---------------------------

def run_cli(argv: List[str]) -> int:
    """
    Run the CLI by dispatching through types.py's main().
    """
    return dispatch_main(argv)


# ---------------------------
# Example: end-to-end scripted usage
# ---------------------------

def main():
    # Example sequence:
    # 1) init
    # 2) add task
    # 3) list tasks
    # 4) search-all
    # 5) complete task
    # 6) show task (detail)
    # 7) search tasks (substring)
    # 8) stats

    print("=== Todozi-style Example CLI ===\n")

    run_cli(["example_cli", "init"])
    print()

    run_cli([
        "example_cli", "add", "task",
        "Write documentation",
        "--time", "1 hour",
        "--priority", "medium",
        "--project", "general",
        "--status", "todo",
        "--tags", "docs,writing",
        "--context", "User guide and README",
        "--progress", "0"
    ])
    print()

    run_cli(["example_cli", "add", "task", "Review PR #42", "--time", "30 minutes", "--priority", "high", "--project", "general"])
    print()

    run_cli(["example_cli", "list", "tasks"])
    print()

    run_cli(["example_cli", "search-all", "docs", "--types", "tasks"])
    print()

    # Show tasks and complete the first one
    tasks = STORAGE.list_tasks()
    if tasks:
        first_task_id = tasks[0].id
        run_cli(["example_cli", "show", "task", first_task_id])
        print()
        run_cli(["example_cli", "complete", first_task_id])
        print()

    run_cli(["example_cli", "search", "tasks", "Review"])
    print()

    run_cli(["example_cli", "stats"])


if __name__ == "__main__":
    # If you want to run the interactive demo sequence, call main():
    # main()

    # Otherwise, use the CLI as usual:
    raise SystemExit(run_cli(sys.argv))