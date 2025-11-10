#!/usr/bin/env python3
# example1.py
# Practical usage example for todozi.py pipeline.

import asyncio
import re
import uuid
import json
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional, Tuple, Union

# Import core parsing/workflow from todozi.py
from todozi import (
    process_chat_message_extended,
    process_json_examples,
    process_workflow,
    TodoziError,
    ValidationError,
    Task,
    Status,
    Priority,
    Assignee,
    Memory,
    Idea,
    Error,
    TrainingData,
    Feeling,
    CodeChunk,
)

# -----------------------------
# Minimal in-memory storage (fallback)
# -----------------------------
class SimpleStorage:
    def __init__(self) -> None:
        self.queue: List[Dict[str, Any]] = []
        self.assignments: List[Dict[str, Any]] = []
        self.tasks: Dict[str, Task] = {}

    async def add_queue_item(self, item: "QueueItem") -> None:
        self.queue.append({
            "id": item.id,
            "title": item.title,
            "description": item.description,
            "priority": item.priority,
            "project_id": item.project_id,
        })

    async def save_agent_assignment(self, assignment: "AgentAssignment") -> None:
        self.assignments.append({
            "agent_id": assignment.agent_id,
            "task_id": assignment.task_id,
            "project_id": assignment.project_id,
            "assigned_at": assignment.assigned_at.isoformat(),
            "status": assignment.status,
        })

    async def update_task_in_project(self, task_id: str, update: "TaskUpdate") -> None:
        t = self.tasks.get(task_id)
        if not t:
            # If unknown, create it so demo can show persistence
            t = Task(
                id=task_id,
                user_id="demo",
                action=update.action or "(no action)",
                time="1 hour",
                priority=Priority.Medium,
                parent_project="demo",
                status=Status.Todo,
            )
            self.tasks[task_id] = t

        if update.action is not None:
            t.action = update.action
        if update.status is not None:
            t.status = update.status
        t.updated_at = datetime.now(timezone.utc)

    # For the demo we also expose a setter to inject tasks before processing
    def add_task(self, task: Task) -> None:
        self.tasks[task.id] = task


# These are used by the workflow, so they must be importable from todozi modules
# (code uses from todozi.storage import ...)
import sys
from pathlib import Path

_file = Path(__file__)
if _file.exists():
    parent = _file.parent
    if str(parent) not in sys.path:
        sys.path.insert(0, str(parent))

class QueueItem:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str = ""
    description: str = ""
    priority: Priority = Priority.Medium
    project_id: Optional[str] = None

class AgentAssignment:
    agent_id: str = ""
    task_id: str = ""
    project_id: str = ""
    assigned_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    status: Any = None

class TaskUpdate:
    action: Optional[str] = None
    status: Optional[Status] = None

# Patch storage import to use our SimpleStorage
class FakeStorage:
    def __init__(self) -> None:
        self._impl = SimpleStorage()

    @staticmethod
    async def new():
        return FakeStorage()._impl

# Inject storage access into the workflow by monkey-patching the module
import todozi
todozi.Storage.get_instance = lambda self=None: FakeStorage()._impl

# -----------------------------
# Demo
# -----------------------------

MESSAGE_WITH_MIXED_CONTENT = """
Let's plan the day.

<todozi>Generate weekly report; 2 hours; medium; ops; inprogress; assignee=ai</todozi>

<memory>emotional; Feeling overwhelmed with too many tasks; Need to balance workload; Because of tight deadlines; high; short; overwhelmed,deadline</memory>

<idea>Automate test pipelines; team; high; Use GitHub Actions for CI; automation,ci</idea>

<todozi>Fix critical bug in checkout; ASAP; critical; payments; blocked; assignee=agent=plumber; dependencies=Identify root cause</todozi>

<error>Payment timeout; Payments API times out randomly; high; network; payments-api; Retries enabled; payments,api,timeout</error>

<train>instruction; Summarize key points from meeting notes; Provide a 5-bullet summary; Keep it concise; meetings,notes; 0.9; ai-studio</train>

<feel>frustrated; 7; The API keeps failing; on-call; monitoring,ops</feel>

<chunk>def fibonacci(n): return n if n <= 1 else fibonacci(n-1) + fibonacci(n-2); python; Simple recursive function; example/fib.py; sample,python</chunk>

<reminder>Prepare Q3 roadmap; 2025-10-01T09:00:00Z; high; pending; planning,roadmap</reminder>

<todozi>Refactor auth module; 1 day; high; security; todo; assignee=collaborative; tags=auth,security; dependencies=Design threat model; context_notes=Minimize breaking changes; progress=10%</todozi>
"""

JSON_EXAMPLES_PAYLOAD = """
{
  "tool_definition": {
    "name": "todozi.add_task",
    "description": "Add a todozi task",
    "parameters": {
      "type": "object",
      "properties": {
        "action": {"type": "string"},
        "time": {"type": "string"},
        "priority": {"type": "string"},
        "parent_project": {"type": "string"},
        "status": {"type": "string"},
        "assignee": {"type": "string"},
        "tags": {"type": "array", "items": {"type": "string"}},
        "dependencies": {"type": "array", "items": {"type": "string"}},
        "context_notes": {"type": "string"},
        "progress": {"type": "number"}
      },
      "required": ["action", "time", "priority", "parent_project", "status"]
    },
    "examples": [
      {
        "todozi_format": "Implement OAuth2 login flow; 6 hours; high; python-web-framework; todo; assignee=human; tags=auth,backend; dependencies=Design API; context_notes=Ensure security; progress=0%"
      },
      {
        "todozi_format": "Draft security checklist; 1 hour; medium; compliance; deferred; assignee=ai; tags=security,checklist"
      }
    ]
  }
}
"""

def print_content_summary(content: "ChatContent") -> None:
    def safe(v: Optional[List[Any]]) -> int:
        return len(v) if v else 0

    print("Extracted content summary:")
    print(f"  - Tasks: {len(content.tasks)}")
    print(f"  - Memories: {len(content.memories)}")
    print(f"  - Ideas: {len(content.ideas)}")
    print(f"  - Agent Assignments: {len(content.agent_assignments)}")
    print(f"  - Code Chunks: {len(content.code_chunks)}")
    print(f"  - Errors: {len(content.errors)}")
    print(f"  - Training Data: {len(content.training_data)}")
    print(f"  - Feelings: {len(content.feelings)}")
    print(f"  - Summaries: {len(content.summaries)}")
    print(f"  - Reminders: {len(content.reminders)}")
    print()

    for i, t in enumerate(content.tasks, 1):
        print(f"Task {i}: {t.action} | {t.priority.name} | {t.status.name} | {t.parent_project} | assignee={t.assignee}")
    print()

    for i, m in enumerate(content.memories, 1):
        mem_type = m.memory_type.name if hasattr(m.memory_type, "name") else str(m.memory_type)
        print(f"Memory {i}: {m.moment} | {mem_type} | {m.importance.name if hasattr(m.importance, 'name') else str(m.importance)}")
    print()

    for i, idea in enumerate(content.ideas, 1):
        print(f"Idea {i}: {idea.idea} | {idea.share.name if hasattr(idea.share, 'name') else str(idea.share)}")
    print()

    for i, err in enumerate(content.errors, 1):
        print(f"Error {i}: {err.title} | {err.severity.name if hasattr(err.severity, 'name') else str(err.severity)} | {err.category.name if hasattr(err.category, 'name') else str(err.category)}")
    print()

    for i, tr in enumerate(content.training_data, 1):
        print(f"Training {i}: {tr.data_type.name if hasattr(tr.data_type, 'name') else str(tr.data_type)} | prompt={tr.prompt[:50]}...")
    print()

    for i, feel in enumerate(content.feelings, 1):
        print(f"Feeling {i}: {feel.emotion} (intensity {feel.intensity}) | {feel.description[:60]}")
    print()

    for i, chunk in enumerate(content.code_chunks, 1):
        print(f"Code Chunk {i}: {chunk.language} | {chunk.summary or '(no summary)'} | {chunk.source_file or '(no file)'}")
    print()

    for i, r in enumerate(content.reminders, 1):
        due = r.due_at.isoformat() if r.due_at else "(no due date)"
        print(f"Reminder {i}: {r.content} | due={due} | {r.priority.name if hasattr(r.priority, 'name') else str(r.priority)}")
    print()

async def run_workflow(tasks: List[Task], storage: SimpleStorage) -> List[str]:
    # Add tasks to storage so update_task_in_project can find them
    for t in tasks:
        storage.add_task(t)

    results = await process_workflow(tasks)
    return results

async def main():
    print("=" * 60)
    print("Example 1: Parse, Queue, Execute, and Persist Tasks")
    print("=" * 60)
    print()

    # 1) Extract structured content from a free-form message
    print("Step 1: Parsing message with <todozi>, <memory>, <idea>, <error>, <train>, <feel>, <chunk>, <reminder>")
    content = process_chat_message_extended(MESSAGE_WITH_MIXED_CONTENT, user_id="example-user")
    print_content_summary(content)

    # 2) Execute the workflow for tasks (AI/Human/Collab/Agent assignments, queueing, status updates)
    print("Step 2: Executing tasks and queuing work...")
    storage = SimpleStorage()
    execution_results = await run_workflow(content.tasks, storage)
    for r in execution_results:
        print(" -", r)
    print()

    # 3) Show persisted status (fetch updated tasks back)
    print("Step 3: Persisted status (in-memory store):")
    for t in content.tasks:
        updated = storage.tasks.get(t.id)
        if updated:
            print(f" - {updated.id} | {updated.status.name} | {updated.action}")
    print()

    # 4) Show queued items
    print("Step 4: Queue items (in-memory store):")
    for q in storage.queue:
        print(f" - [{q['id']}] {q['title']} | project={q['project_id']} | priority={q['priority'].name if hasattr(q['priority'], 'name') else str(q['priority'])}")
    print()

    # 5) Show agent assignments
    print("Step 5: Agent assignments:")
    for a in storage.assignments:
        print(f" - agent={a['agent_id']} -> task={a['task_id']} in project={a['project_id']}")
    print()

    # 6) Parse JSON examples to tasks
    print("Step 6: Parsing JSON examples...")
    json_tasks = process_json_examples(JSON_EXAMPLES_PAYLOAD)
    for i, t in enumerate(json_tasks, 1):
        print(f" - JSON Task {i}: {t.action} | {t.priority.name} | {t.status.name} | {t.parent_project}")
    print()

    print("=" * 60)
    print("Done. This example shows how to parse messages into structured")
    print("objects, route work to AI/humans/agents, and persist status via Storage.")
    print("=" * 60)

if __name__ == "__main__":
    # Ensure Windows proactor event loop if available (optional)
    try:
        asyncio.set_event_loop_policy(asyncio.WindowsProactorEventLoopPolicy())  # type: ignore
    except Exception:
        pass
    asyncio.run(main())