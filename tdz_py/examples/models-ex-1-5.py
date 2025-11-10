# example1_models_usage.py
#
# A self-contained, executable example showing how to use the data models
# from models.py (Result<T, E> style, task creation, validation, and filtering).
#
# Run:
#   python example1_models_usage.py
#
# No external dependencies are required. This script only uses the standard library
# and the code from models.py.

from __future__ import annotations

import json
import random
from datetime import datetime, timezone
from typing import List, Optional, Dict, Any

# Import everything we need from models.py
from models import (
    Ok,
    Err,
    Result,
    TodoziError,
    Priority,
    Status,
    Assignee,
    Task,
    TaskUpdate,
    TaskFilters,
    ItemStatus,
    Project,
    TaskCollection,
    utc_now,
    short_uuid,
    ProjectTaskContainer,
    hash_project_name,
)


def utc_now_str() -> str:
    return utc_now().strftime("%Y-%m-%d %H:%M:%S %Z")


def banner(msg: str) -> None:
    print("\n" + "=" * 70)
    print(f"  {msg}")
    print("=" * 70)


def hr() -> None:
    print("-" * 70)


def demo_enums() -> None:
    banner("Enum parsing and validation")

    # Priority parsing (case-insensitive, supports common aliases)
    for p in ["LOW", "Medium", "HIGH", "critical", "urgent"]:
        res: Result[Priority, TodoziError] = Priority.from_str(p)
        if isinstance(res, Ok):
            print(f"✅ Parsed priority '{p}' -> {res.value}")
        else:
            print(f"❌ Failed to parse priority '{p}': {res.error.message}")

    hr()

    # Status parsing (case-insensitive, honors aliases like "pending" -> "todo")
    for s in ["todo", "PENDING", "in_progress", "in-progress", "done", "completed", "cancelled", "canceled", "deferred"]:
        res: Result[Status, TodoziError] = Status.from_str(s)
        if isinstance(res, Ok):
            print(f"✅ Parsed status '{s}' -> {res.value}")
        else:
            print(f"❌ Failed to parse status '{s}': {res.error.message}")

    hr()

    # Assignee parsing (supports "ai", "human", "collaborative", "agent:NAME")
    for a in ["ai", "human", "collaborative", "agent:clippy", "agent:copilot", "someone"]:
        res: Result[Assignee, TodoziError] = Assignee.from_str(a)
        if isinstance(res, Ok):
            print(f"✅ Parsed assignee '{a}' -> {res.value}")
        else:
            print(f"❌ Failed to parse assignee '{a}': {res.error.message}")


def demo_result() -> None:
    banner("Result<T, E> (Ok/Err) usage patterns")

    def parse_priority(priority: str) -> Result[Priority, TodoziError]:
        return Priority.from_str(priority)

    def try_parse(priority: str) -> None:
        match parse_priority(priority):
            case Ok(value):
                print(f"  → Ok: {value}")
            case Err(e):
                print(f"  → Err: {e.message}")

    for p in ["low", "medium", "very_high", "high"]:
        print(f"Parsing '{p}':")
        try_parse(p)


def demo_create_tasks() -> tuple[Task, Task, Task, Task]:
    banner("Creating tasks (valid and invalid)")

    # Helper to print task creation
    def print_task_result(label: str, res: Result[Task, TodoziError]) -> Optional[Task]:
        match res:
            case Ok(task):
                print(f"  ✅ {label}: {task.id} | {task.action} | project={task.parent_project} priority={task.priority} status={task.status}")
                return task
            case Err(e):
                print(f"  ❌ {label}: {e.message}")
                return None

    # Valid tasks
    t1 = print_task_result(
        "t1 (new_full)",
        Task.new_full(
            user_id="user_123",
            action="Write documentation for models.py",
            time="1 day",
            priority=Priority.HIGH,
            parent_project="docs",
            status=Status.TODO,
            assignee=Assignee.from_str("ai").value,
            tags=["docs", "models"],
            dependencies=[],
            context_notes="Cover Result<T, E>, validators, and enums",
            progress=None,
        ),
    )
    t2 = print_task_result(
        "t2 (new_full)",
        Task.new_full(
            user_id="user_456",
            action="Prepare demo script",
            time="2 hours",
            priority=Priority.MEDIUM,
            parent_project="examples",
            status=Status.IN_PROGRESS,
            assignee=Assignee.from_str("agent:assistant").value,
            tags=["example", "demo"],
            dependencies=[],
            context_notes=None,
            progress=25,
        ),
    )

    # Invalid task (progress out of range) demonstrates Err handling
    t3 = print_task_result(
        "t3 (invalid progress)",
        Task.new_full(
            user_id="user_789",
            action="Refactor parser",
            time="3 hours",
            priority=Priority.CRITICAL,
            parent_project="core",
            status=Status.BLOCKED,
            assignee=None,
            tags=["refactor"],
            dependencies=[],
            context_notes=None,
            progress=150,  # invalid
        ),
    )

    # Another invalid case (bad priority handled earlier)
    t4 = print_task_result(
        "t4 (new)",
        Task.new(
            user_id="user_999",
            action="Ship the example",
            time="ASAP",
            priority=Priority.URGENT,
            parent_project="release",
            status=Status.REVIEW,
        ),
    )

    hr()
    return (t1, t2, t3, t4)  # type: ignore


def demo_update_tasks(t1: Task, t2: Task) -> None:
    banner("Updating tasks with TaskUpdate")

    def attempt_update(task: Task, label: str, updates: TaskUpdate) -> None:
        res: Result[None, TodoziError] = task.update(updates)
        match res:
            case Ok(_):
                print(f"  ✅ {label}: updated {task.id} -> progress={task.progress}, status={task.status}, priority={task.priority}")
            case Err(e):
                print(f"  ❌ {label}: update failed -> {e.message}")

    # Valid update
    attempt_update(
        t1,
        "t1",
        TaskUpdate()
            .with_action("Write comprehensive docs for models.py")
            .with_progress(60)
            .with_status(Status.IN_PROGRESS)
            .with_priority(Priority.HIGH),
    )

    # Invalid update (progress out of range)
    attempt_update(
        t2,
        "t2 (invalid progress)",
        TaskUpdate().with_progress(999).with_status(Status.DONE),
    )

    hr()


def demo_collection(t1: Task, t2: Task) -> None:
    banner("TaskCollection (in-memory store) and filtering")

    collection = TaskCollection()

    # Add tasks to the collection
    for t in [t1, t2]:
        if t is not None:
            collection.add_task(t)
            print(f"  ➕ Added {t.id} to collection")

    hr()
    print("  All tasks in collection:")
    for t in collection.get_all_tasks():
        print(f"    - {t.id} | {t.action} | proj={t.parent_project} | priority={t.priority} | status={t.status}")

    hr()
    print("  Filter by project='docs':")
    for t in collection.get_filtered_tasks(TaskFilters(project="docs")):
        print(f"    - {t.id} | {t.action}")

    hr()
    print("  Filter by search='demo':")
    for t in collection.get_filtered_tasks(TaskFilters(search="demo")):
        print(f"    - {t.id} | {t.action}")


def demo_project_container(t1: Task, t2: Task) -> None:
    banner("ProjectTaskContainer (by-status buckets)")

    # Create a container for project "examples"
    container = ProjectTaskContainer.new("examples")

    # Add tasks
    for t in [t1, t2]:
        if t is not None:
            t.parent_project = "examples"
            container.add_task(t)
            print(f"  ➕ Added {t.id} to project container 'examples'")

    hr()
    print("  Active tasks:", list(container.active_tasks.keys()))
    print("  Completed tasks:", list(container.completed_tasks.keys()))

    hr()
    # Move a task to DONE and see it shift buckets
    if t1:
        container.update_task_status(t1.id, Status.DONE)
        print(f"  🔁 Moved {t1.id} to DONE")
        print("  Completed tasks:", list(container.completed_tasks.keys()))


def demo_serialization(t1: Task, t2: Task) -> None:
    banner("Serialization helpers (to_dict/from_dict)")

    def to_dict(task: Task) -> Dict[str, Any]:
        return {
            "id": task.id,
            "user_id": task.user_id,
            "action": task.action,
            "time": task.time,
            "priority": task.priority.value if hasattr(task.priority, "value") else str(task.priority),
            "parent_project": task.parent_project,
            "status": task.status.value if hasattr(task.status, "value") else str(task.status),
            "assignee": str(task.assignee) if task.assignee else None,
            "tags": task.tags,
            "dependencies": task.dependencies,
            "context_notes": task.context_notes,
            "progress": task.progress,
            "created_at": task.created_at.isoformat(),
            "updated_at": task.updated_at.isoformat(),
        }

    def from_dict(d: Dict[str, Any]) -> Task:
        # Basic reconstruction; in real code you'd map all fields
        priority = Priority.from_str(d.get("priority", "medium")).value if isinstance(Priority.from_str(d.get("priority", "medium")), Ok) else Priority.MEDIUM
        status = Status.from_str(d.get("status", "todo")).value if isinstance(Status.from_str(d.get("status", "todo")), Ok) else Status.TODO
        assignee = Assignee.from_str(d["assignee"]).value if d.get("assignee") and isinstance(Assignee.from_str(d["assignee"]), Ok) else None
        return Task(
            id=d["id"],
            user_id=d.get("user_id", "system"),
            action=d["action"],
            time=d.get("time", "1 hour"),
            priority=priority,
            parent_project=d.get("parent_project", "general"),
            status=status,
            assignee=assignee,
            tags=d.get("tags", []),
            dependencies=d.get("dependencies", []),
            context_notes=d.get("context_notes"),
            progress=d.get("progress"),
            created_at=datetime.fromisoformat(d["created_at"]),
            updated_at=datetime.fromisoformat(d["updated_at"]),
        )

    # Dump a couple of tasks
    dumped = [to_dict(t) for t in [t1, t2] if t]
    print("  JSON:")
    print(json.dumps(dumped, indent=2))

    hr()
    print("  Loaded from JSON:")
    for obj in dumped:
        t = from_dict(obj)
        print(f"    - {t.id} | {t.action} | status={t.status} | priority={t.priority}")


def demo_filters() -> None:
    banner("TaskFilters (across projects via a collection)")

    collection = TaskCollection()

    # Create several tasks
    samples: List[Task] = []
    for i in range(6):
        priority = random.choice([Priority.LOW, Priority.MEDIUM, Priority.HIGH, Priority.CRITICAL])
        status = random.choice([Status.TODO, Status.IN_PROGRESS, Status.DONE, Status.REVIEW])
        assignee = random.choice([None, Assignee.from_str("ai").value, Assignee.from_str("human").value])
        tags = random.choice([[], ["backend"], ["frontend", "urgent"], ["docs"]])
        t = Task.new(
            user_id="demo_user",
            action=f"Sample task #{i+1}",
            time="1d",
            priority=priority,
            parent_project=random.choice(["web", "core", "infra"]),
            status=status,
        )
        t.assignee = assignee
        t.tags = tags
        collection.add_task(t)
        samples.append(t)

    # Different filters
    filters: List[TaskFilters] = [
        TaskFilters(project="web"),
        TaskFilters(status=Status.DONE),
        TaskFilters(priority=Priority.CRITICAL),
        TaskFilters(search="Sample"),
        TaskFilters(tags=["urgent"]),
    ]

    for f in filters:
        print(f"\n  Filter: {f}")
        for t in collection.get_filtered_tasks(f):
            print(f"    - {t.id} | {t.action} | proj={t.parent_project} | priority={t.priority} | status={t.status}")


def demo_compute_stats(tasks: List[Task]) -> None:
    banner("Compute basic project stats")

    def compute_stats(task_list: List[Task]) -> Dict[str, Any]:
        total = len(task_list)
        by_status: Dict[str, int] = {}
        by_priority: Dict[str, int] = {}
        for t in task_list:
            s = t.status.value if hasattr(t.status, "value") else str(t.status)
            p = t.priority.value if hasattr(t.priority, "value") else str(t.priority)
            by_status[s] = by_status.get(s, 0) + 1
            by_priority[p] = by_priority.get(p, 0) + 1
        completed = by_status.get("done", 0) + by_status.get("completed", 0)
        completion_rate = (completed / total) if total > 0 else 0.0
        return {
            "total": total,
            "by_status": by_status,
            "by_priority": by_priority,
            "completion_rate": completion_rate,
        }

    stats = compute_stats([t for t in tasks if t is not None])
    print(f"  Total tasks: {stats['total']}")
    print(f"  By status: {stats['by_status']}")
    print(f"  By priority: {stats['by_priority']}")
    print(f"  Completion rate: {stats['completion_rate']:.1%}")


def main() -> None:
    print("\n" + "🛠️  Todozi Models Example 1" + "\n")
    print(f"⏰ Start time: {utc_now_str()}")

    # 1) Enums and parsing
    demo_enums()

    # 2) Result<T, E> style
    demo_result()

    # 3) Task creation (Ok/Err)
    t1, t2, t3, t4 = demo_create_tasks()

    # 4) Task updates
    if t1 and t2:
        demo_update_tasks(t1, t2)

    # 5) In-memory collection and filtering
    if t1 and t2:
        demo_collection(t1, t2)

    # 6) Project-based container (status buckets)
    if t1 and t2:
        demo_project_container(t1, t2)

    # 7) Serialization helpers
    if t1 and t2:
        demo_serialization(t1, t2)

    # 8) More filtering examples
    demo_filters()

    # 9) Compute stats
    demo_compute_stats([t1, t2, t3, t4])

    # 10) A complete "workflow" example
    banner("Mini workflow: create → update → complete")
    wf_task_res = Task.new_full(
        user_id="demo_user",
        action="Implement example1 script",
        time="2 hours",
        priority=Priority.HIGH,
        parent_project="examples",
        status=Status.TODO,
        assignee=Assignee.from_str("agent:assistant").value,
        tags=["example", "documentation"],
        dependencies=[],
        context_notes="No blockers",
        progress=None,
    )
    if isinstance(wf_task_res, Ok):
        wf = wf_task_res.value
        print(f"  ➕ Created: {wf.id} | {wf.action}")
        # Update progress
        wf.update(TaskUpdate().with_progress(50))
        print(f"  ⬆️  Progress: {wf.progress}%")
        # Complete
        wf.complete()
        print(f"  ✅ Completed: {wf.id} | status={wf.status} | progress={wf.progress}%")
    else:
        print(f"  ❌ Failed to create task: {wf_task_res.error.message}")

    banner("Done")
    print(f"⏰ End time: {utc_now_str()}\n")


if __name__ == "__main__":
    main()