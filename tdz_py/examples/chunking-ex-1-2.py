#!/usr/bin/env python3
"""
example1.py

Demonstrates how to use chunking.py to:
- Create a code generation graph
- Add chunks with different levels and dependencies
- Update chunk code/tests, mark chunks completed/validated
- Query ready chunks and dependency chains
- Parse chunk definitions from text
- Print project summary

Run:
  python example1.py
"""

import sys
from pathlib import Path

# Add the project root to the path so we can import chunking.py as a module
project_root = Path(__file__).parent
if str(project_root) not in sys.path:
    sys.path.insert(0, str(project_root))

from chunking import (
    CodeGenerationGraph,
    ChunkingLevel,
    process_chunking_message,
    Ok,
    Err,
)

def main():
    print("=" * 80)
    print("Example 1: Using chunking.py to build and process a code generation graph")
    print("=" * 80)
    print()

    # 1) Create a new graph with a maximum line budget
    print("1) Create a code generation graph with max_lines=1000")
    graph = CodeGenerationGraph(max_lines=1000)
    print(f"   - Graph created with max_lines={graph.project_state.max_lines}")
    print()

    # 2) Add some chunks with dependencies
    print("2) Add chunks with different granularity levels and dependencies")
    graph.add_chunk("chunk_project_plan", ChunkingLevel.PROJECT, deps=[])
    graph.add_chunk("chunk_db_module", ChunkingLevel.MODULE, deps=["chunk_project_plan"])
    graph.add_chunk("chunk_db_connection_class", ChunkingLevel.CLASS, deps=["chunk_db_module"])
    graph.add_chunk("chunk_connect_method", ChunkingLevel.METHOD, deps=["chunk_db_connection_class"])
    graph.add_chunk("chunk_timeout_handler", ChunkingLevel.BLOCK, deps=["chunk_connect_method"])
    print("   - Chunks added: project_plan -> db_module -> db_connection_class -> connect_method -> timeout_handler")
    print()

    # 3) Show ready chunks (those with all dependencies satisfied)
    print("3) Show chunks ready to work on (all dependencies satisfied)")
    ready = graph.get_ready_chunks()
    print(f"   - Ready chunks: {ready}")
    print()

    # 4) Update chunk code and tests
    print("4) Update 'chunk_project_plan' with code and tests")
    plan_code = """# Project Plan
We will build a small database handler module.
- Define a connection class
- Implement a connect() method
- Handle connection timeouts
"""
    plan_tests = """# Tests for Project Plan
assert True, "Project plan exists"
"""
    result = graph.update_chunk_code("chunk_project_plan", plan_code)
    if isinstance(result, Ok):
        print("   - Code updated successfully")
    else:
        print(f"   - Error updating code: {result.error}")

    result = graph.update_chunk_tests("chunk_project_plan", plan_tests)
    if isinstance(result, Ok):
        print("   - Tests updated successfully")
    else:
        print(f"   - Error updating tests: {result.error}")
    print()

    # 5) Mark the chunk as completed
    print("5) Mark 'chunk_project_plan' as completed")
    result = graph.mark_chunk_completed("chunk_project_plan")
    if isinstance(result, Ok):
        print("   - Marked as completed")
    else:
        print(f"   - Error: {result.error}")
    print()

    # 6) Check which chunks are ready now
    print("6) Re-evaluate ready chunks after completion")
    ready = graph.get_ready_chunks()
    print(f"   - Ready chunks: {ready}")
    print()

    # 7) Simulate working through a few more chunks
    print("7) Simulate work on 'chunk_db_module'")
    db_code = """# db_module.py
import sqlite3

class DatabaseConnection:
    def __init__(self, path: str):
        self.path = path
        self.conn = None
"""
    result = graph.update_chunk_code("chunk_db_module", db_code)
    if isinstance(result, Ok):
        print("   - Code updated")
    result = graph.mark_chunk_completed("chunk_db_module")
    if isinstance(result, Ok):
        print("   - Marked as completed")
    print()

    print("8) Work on 'chunk_db_connection_class'")
    conn_code = """class DatabaseConnection:
    def __init__(self, path: str):
        self.path = path
        self.conn = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc, tb):
        if self.conn:
            self.conn.close()
"""
    result = graph.update_chunk_code("chunk_db_connection_class", conn_code)
    if isinstance(result, Ok):
        print("   - Code updated")
    result = graph.mark_chunk_completed("chunk_db_connection_class")
    if isinstance(result, Ok):
        print("   - Marked as completed")
    print()

    print("9) Work on 'chunk_connect_method'")
    connect_code = """    def connect(self, timeout: int = 5):
        self.conn = sqlite3.connect(self.path, timeout=timeout)
"""
    result = graph.update_chunk_code("chunk_connect_method", connect_code)
    if isinstance(result, Ok):
        print("   - Code updated")
    result = graph.mark_chunk_validated("chunk_connect_method")
    if isinstance(result, Ok):
        print("   - Marked as validated")
    print()

    print("10) Work on 'chunk_timeout_handler'")
    timeout_code = """    def connect(self, timeout: int = 5):
        try:
            self.conn = sqlite3.connect(self.path, timeout=timeout)
        except sqlite3.OperationalError as e:
            if "database is locked" in str(e).lower():
                raise TimeoutError("Connection timed out and database is locked") from e
        raise
"""
    result = graph.update_chunk_code("chunk_timeout_handler", timeout_code)
    if isinstance(result, Ok):
        print("   - Code updated")
    result = graph.mark_chunk_completed("chunk_timeout_handler")
    if isinstance(result, Ok):
        print("   - Marked as completed")
    print()

    # 8) Show project summary
    print("11) Project summary")
    print(graph.get_project_summary())
    print()

    # 9) Show dependency chains
    print("12) Dependency chains")
    for chunk_id in ["chunk_db_module", "chunk_connect_method", "chunk_timeout_handler"]:
        chain = graph.get_dependency_chain(chunk_id)
        print(f"   - Dependency chain for {chunk_id}: {chain}")
    print()

    # 13) Parse chunks from a formatted text message
    print("13) Parse chunk definitions from a formatted message")
    message = """
Here is the plan:
<chunk>plan; project; Create project plan; ; # Project plan stub</chunk>
<chunk>config; module; Database configuration loader; plan; import os; import json</chunk>
"""

    parse_result = process_chunking_message(message)
    if isinstance(parse_result, Ok):
        print(f"   - Parsed {len(parse_result.value)} chunk(s) from message:")
        for c in parse_result.value:
            print(f"      - id={c.chunk_id}, level={c.level}, dependencies={sorted(c.dependencies)}")
            if c.code:
                print(f"        code={c.code[:50]}...")
    else:
        print(f"   - Error parsing chunks: {parse_result.error}")
    print()

    # 14) Final ready chunks
    print("14) Final ready chunks (should be empty for this demo)")
    ready = graph.get_ready_chunks()
    print(f"   - Ready chunks: {ready}")
    print()

    print("=" * 80)
    print("Example complete.")
    print("=" * 80)

if __name__ == "__main__":
    main()