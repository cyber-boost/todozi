#!/usr/bin/env python3
# example_1_agent_usage.py
"""
A minimal, self-contained example that exercises the agent module (agent.py)
without depending on the larger CLI or server infrastructure.

Highlights:
- Create and persist agents
- Update agents (capabilities, specializations, status)
- Find best available agent for a task (by specialization/capability)
- Assign a task to an agent, then complete the assignment
- Compute and print statistics
"""

from __future__ import annotations

import asyncio
import json
import os
import sys
import uuid
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime, timezone
from enum import Enum
from typing import Any, Dict, List, Optional

# --- Bring in the agent types, manager, and storage helpers from agent.py ---
# This assumes agent.py is in the same directory. Adjust the import if needed.
try:
    from agent import (
        AgentManager,
        Agent,
        AgentUpdate,
        AgentMetadata,
        AgentStatus,
        AssignmentStatus,
        AgentStatistics,
        json_file_transaction,
        _AGENTS_JSON,
    )
except ImportError as e:
    print("ERROR: Could not import 'agent' module. Make sure agent.py is in the same directory.")
    raise


# -----------------------------
# Tiny in-memory storage for this example
# -----------------------------
_AGENTS: List[Dict[str, Any]] = []


def _save_agents(data: List[Dict[str, Any]]) -> None:
    with json_file_transaction(_AGENTS_JSON) as tmp_path:
        with open(tmp_path, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)


def _load_agents() -> List[Dict[str, Any]]:
    if not os.path.exists(_AGENTS_JSON):
        return []
    with open(_AGENTS_JSON, "r", encoding="utf-8") as f:
        data = json.load(f)
        if isinstance(data, list):
            return data
        return []


# -----------------------------
# Demo helpers
# -----------------------------
def print_sep(title: str) -> None:
    print("\n=== {} ===".format(title))


async def demo_agent_manager(mgr: AgentManager) -> None:
    # Clean slate for this example
    _save_agents([])

    print_sep("Create agents")
    alice = Agent.new("Alice", "General-purpose assistant")
    alice.capabilities = ["chat", "search", "planning"]
    alice.specializations = ["general"]

    bob = Agent.new("Bob", "Code expert")
    bob.capabilities = ["coding", "debugging", "refactoring"]
    bob.specializations = ["code"]

    carol = Agent.new("Carol", "Data analysis specialist")
    carol.capabilities = ["analysis", "visualization"]
    carol.specializations = ["data", "planning"]

    alice_id = await mgr.create_agent(alice)
    bob_id = await mgr.create_agent(bob)
    carol_id = await mgr.create_agent(carol)
    print(f"Created agents: {alice_id[:8]}..., {bob_id[:8]}..., {carol_id[:8]}...")

    print_sep("List all agents")
    for a in mgr.get_all_agents():
        print(f" - {a.name} ({a.id[:8]}...) capabilities={a.capabilities}, specializations={a.specializations}, status={a.metadata.status.value}")

    print_sep("Update Bob: add 'testing' capability and specialize in 'code'")
    await mgr.update_agent(bob_id, AgentUpdate().with_capabilities(["coding", "debugging", "refactoring", "testing"]).with_specializations(["code"]))
    updated_bob = mgr.get_agent(bob_id)
    print(f"Bob updated: capabilities={updated_bob.capabilities}, specializations={updated_bob.specializations}")

    print_sep("Find best available agent for 'code' specialization, preferred_capability='testing'")
    best = mgr.find_best_agent(required_specialization="code", preferred_capability="testing")
    if best:
        print(f"Best agent: {best.name} ({best.id[:8]}...), capabilities={best.capabilities}, specializations={best.specializations}")
    else:
        print("No suitable agent found.")

    print_sep("Assign a task to Bob and complete it")
    task_id = f"task-{uuid.uuid4().hex[:8]}"
    await mgr.assign_task_to_agent(task_id, bob_id, "demo-project")
    print(f"Assigned task {task_id} to Bob (status set to BUSY)")

    print_sep("Complete assignment for the task")
    await mgr.complete_agent_assignment(task_id)
    completed_bob = mgr.get_agent(bob_id)
    print(f"Bob status after completion: {completed_bob.metadata.status.value}")

    print_sep("Get assignments for Bob")
    assigns = mgr.get_agent_assignments(bob_id)
    for a in assigns:
        print(f" - task_id={a.task_id}, status={a.status.value}, project_id={a.project_id}")

    print_sep("Agent Statistics")
    stats: AgentStatistics = mgr.get_agent_statistics()
    print(f"Total agents: {stats.total_agents}")
    print(f"Available: {stats.available_agents} | Busy: {stats.busy_agents} | Inactive: {stats.inactive_agents}")
    print(f"Total assignments: {stats.total_assignments} | Completed: {stats.completed_assignments} | Completion rate: {stats.completion_rate():.1f}%")


# -----------------------------
# Example 1 entry
# -----------------------------
async def main() -> int:
    print("Example 1: AgentManager basics (no external storage required)")
    mgr = AgentManager()
    await mgr.load_agents()
    await demo_agent_manager(mgr)
    print("\nDone.")
    return 0


if __name__ == "__main__":
    try:
        exit_code = asyncio.run(main())
        sys.exit(exit_code)
    except KeyboardInterrupt:
        print("\nInterrupted.")
        sys.exit(130)
    except Exception as e:
        print(f"\nError: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)