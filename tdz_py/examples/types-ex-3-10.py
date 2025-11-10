from __future__ import annotations

import asyncio
import sys
from pathlib import Path

# ----------------------------------------------------------------------
# 1️⃣  Import the public API from the repository
# ----------------------------------------------------------------------
#   The repository is organised as a package named `todozi`.  The `types.py`
#   file lives at the top level, so we can import directly from it.
# ----------------------------------------------------------------------
sys.path.append(str(Path(__file__).resolve().parent))   # make the repo importable

from types import (                # domain models & the parser builder
    Task,
    Memory,
    Idea,
    CodeChunk,
    Error,
    TrainingData,
    Feeling,
    ChatContent,
    SearchEngine,
    SearchOptions,
    SearchResults,
    build_parser,
)

from cli import TodoziHandler, Storage   # high‑level command handler & storage


# ----------------------------------------------------------------------
# 2️⃣  Helper: create a few sample objects
# ----------------------------------------------------------------------
def make_sample_payload() -> ChatContent:
    """
    Return a ChatContent instance that contains a mixture of objects.
    This mimics what the extended chat parser (`process_chat_message_extended`)
    would normally produce.
    """
    # A simple task – note the different assignee styles.
    task = Task(
        id="task_001",
        action="Write a README for the repo",
        time="30m",
        priority="medium",
        project="doc",
        status="todo",
        assignee="human",
        tags="docs,readme",
        dependencies="",
        context="Make sure to include installation instructions",
        progress=None,
    )

    # A memory entry (standard)
    mem = Memory(
        id="mem_001",
        moment="2025‑03‑01",
        meaning="Discovered that pathlib.Path exists for OS‑independent paths",
        reason="While cleaning up imports",
        importance="high",
        term="short",
        memory_type="standard",
        tags="python,pathlib",
    )

    # An idea
    idea = Idea(
        id="idea_001",
        idea="Add a built‑in `todozi export` command that dumps all data as JSON",
        share="team",
        importance="high",
        tags="cli,export,json",
    )

    # A tiny code chunk
    chunk = CodeChunk(
        id="chunk_001",
        content="def hello():\n    print('hello')",
        language="python",
        tags="example,hello",
    )

    # An error (just for fun)
    err = Error(
        id="err_001",
        title="FileNotFoundError",
        description="Could not locate `todozi.cfg` in the working directory",
        severity="high",
        category="filesystem",
        source="cli",
    )

    # A tiny training record
    train = TrainingData(
        id="train_001",
        data_type="instruction",
        prompt="Explain the difference between `list.append` and `list.extend`.",
        completion="`append` adds a single element; `extend` concatenates an iterable.",
        tags="python,list",
    )

    # A feeling (just to show the type works)
    feel = Feeling(label="motivated")

    return ChatContent(
        tasks=[task],
        memories=[mem],
        ideas=[idea],
        code_chunks=[chunk],
        errors=[err],
        training_data=[train],
        feelings=[feel],
    )


# ----------------------------------------------------------------------
# 3️⃣  Index the payload and run a keyword search
# ----------------------------------------------------------------------
def demo_search_engine(payload: ChatContent, query: str) -> SearchResults:
    """
    Feed the payload into the in‑memory SearchEngine, then query it.
    The function prints the results in a human‑readable way and returns the
    raw `SearchResults` object.
    """
    engine = SearchEngine()
    engine.update_index(payload)

    # No fancy filtering – just a plain search.
    opts = SearchOptions(limit=10)        # return at most 10 hits per type
    results: SearchResults = engine.search(query, opts)

    print("\n🔎  Search results for:", query)
    if results.task_results:
        print("\n🗒️  Tasks:")
        for t in results.task_results:
            print(f" • {t.id}: {t.action} [{t.project}]")
    if results.memory_results:
        print("\n💾 Memories:")
        for m in results.memory_results:
            print(f" • {m.id}: {m.moment} – {m.meaning}")
    if results.idea_results:
        print("\n💡 Ideas:")
        for i in results.idea_results:
            print(f" • {i.id}: {i.idea}")
    if results.error_results:
        print("\n❗ Errors:")
        for e in results.error_results:
            print(f" • {e.id}: {e.title}")
    if results.training_results:
        print("\n📚 Training records:")
        for tr in results.training_results:
            print(f" • {tr.id}: {tr.prompt}")

    return results


# ----------------------------------------------------------------------
# 4️⃣  Store the tasks in the real storage using the high‑level handler
# ----------------------------------------------------------------------
async def demo_handler_workflow(task: Task) -> None:
    """
    1️⃣  Initialise the storage (creates the `~/.todozi` hierarchy if missing).  
    2️⃣  Put the task into the project‑based store (`general` is the default).  
    3️⃣  List all tasks, print them, complete the task and list again.
    """
    # Initialise the on‑disk storage (async because the original code uses asyncio)
    storage = await Storage.new()
    handler = TodoziHandler(storage)

    # ------------------------------------------------------------------
    # a) add the task
    # ------------------------------------------------------------------
    print("\n🚀 Adding a task via TodoziHandler …")
    await handler.handle_add_command(
        # The `AddTask` dataclass lives in `cli.py`; we reuse the same fields.
        # Import lazily to avoid circular imports at module load time.
        __import__("cli").AddTask(
            action=task.action,
            time=task.time,
            priority=task.priority,
            project=task.project,
            status=task.status,
            assignee=task.assignee,
            tags=task.tags,
            dependencies=task.dependencies,
            context=task.context,
            progress=task.progress,
        )
    )

    # ------------------------------------------------------------------
    # b) list tasks
    # ------------------------------------------------------------------
    print("\n📋 Listing tasks after the add …")
    await handler.handle_list_command(
        __import__("cli").ListTasks()   # empty filter → list everything
    )

    # ------------------------------------------------------------------
    # c) complete the task (use the id printed by the previous step)
    # ------------------------------------------------------------------
    # In a real REPL you would capture the id, here we already know it.
    print("\n✅ Marking the task as completed …")
    await handler.handle_complete(task.id)

    # ------------------------------------------------------------------
    # d) final list – the task should now be under “Done”
    # ------------------------------------------------------------------
    print("\n📋 Final task list (should show status=Done) …")
    await handler.handle_list_command(__import__("cli").ListTasks())


# ----------------------------------------------------------------------
# 5️⃣  Tiny CLI that re‑uses the full argument parser from `types.py`
# ----------------------------------------------------------------------
def cli_entrypoint(argv: list[str] | None = None) -> int:
    """
    The repository already ships a fully‑featured CLI (the `main()` function in
    `types.py`).  Here we wrap it so that we can invoke the demo *and* the original
    CLI from the same script:

        $ python mini_workflow.py demo          # run the demo
        $ python mini_workflow.py add task …    # any other Todozi command
    """
    parser = build_parser()
    subparsers = parser._subparsers._group_actions[0].choices  # internal but stable

    # Insert a tiny “demo” sub‑command that runs the workflow above.
    demo_parser = subparsers["demo"] = parser.add_parser("demo", help="Run the mini‑workflow demo")
    demo_parser.set_defaults(_run_demo=True)

    # Parse arguments (the original parser will raise SystemExit on error,
    # which we forward as exit‑code 2 – the same behaviour as the upstream CLI).
    try:
        ns = parser.parse_args(argv)
    except SystemExit as e:
        return e.code

    # If the hidden flag `_run_demo` is present we launch the demo instead of the
    # regular command‑dispatch loop.
    if getattr(ns, "_run_demo", False):
        # ------------------------------------------------------------------
        # a) Build sample payload
        # ------------------------------------------------------------------
        payload = make_sample_payload()

        # b) Run the in‑memory search engine demo
        _ = demo_search_engine(payload, query="readme")

        # c) Store the single task from the payload via the high‑level handler
        #    (the payload contains exactly one task, so we pick it.)
        asyncio.run(demo_handler_workflow(payload.tasks[0]))

        # The demo finishes successfully.
        return 0

    # --------------------------------------------------------------
    # Otherwise we fall back to the original `types.main` dispatcher.
    # --------------------------------------------------------------
    from types import main as original_main
    return original_main(argv)


# ----------------------------------------------------------------------
# 6️⃣  Script entry point
# ----------------------------------------------------------------------
if __name__ == "__main__":
    sys.exit(cli_entrypoint())