# Todozi Python SDK

This directory contains the Python conversion of the Todozi Rust codebase.

## Structure

- `models.py` - Core data models and enums
- `error.py` - Error types and exceptions
- `storage.py` - Storage operations (file-based)
- `agent.py` - AI agent management
- `api.py` - API key management
- `search.py` - Search functionality
- `tags.py` - Tag management
- `base.py` - Base utilities
- `__init__.py` - Package initialization and simple API

## Installation

```bash
pip install -e .
```

## Usage

### Simple API

```python
from tdz.py import Todozi, Priority

# Initialize
await Todozi.init()

# Create a task
task = await Todozi.create_task(
    "Implement new feature",
    priority=Priority.HIGH,
    project="my_project"
)

# Search tasks
results = await Todozi.search_tasks("feature")

# List all tasks
tasks = await Todozi.list_tasks()
```

### Advanced API

```python
from tdz.py import Storage, AgentManager, Task, Priority, Status

# Initialize storage
storage = await Storage.new()

# Create a task
task = Task.new(
    user_id="user123",
    action="Build Python SDK",
    time="This week",
    priority=Priority.HIGH,
    parent_project="todozi",
    status=Status.IN_PROGRESS
)

await storage.add_task_to_project(task)

# Work with agents
agent_manager = AgentManager()
await agent_manager.load_agents()

agents = agent_manager.get_available_agents()
coder = agent_manager.find_best_agent("rust")
```

## Key Differences from Rust Version

1. **Async/Await**: Python's async/await is used instead of Tokio
2. **Error Handling**: Python exceptions instead of Result<T>
3. **Type Hints**: Python type hints instead of Rust's static types
4. **JSON Storage**: Direct JSON instead of Helix (for now)
5. **Dataclasses**: Python dataclasses instead of Rust structs

## Converted Files

✅ **ALL 28 Rust files converted!**

### Core Files (Full Implementation)
- error.rs → error.py (5.4KB) - Error types and exceptions
- models.rs → models.py (26.2KB) - All data models and enums
- storage.rs → storage.py (21.1KB) - File-based storage operations
- agent.rs → agent.py (10.4KB) - Agent management
- api.rs → api.py (5.3KB) - API key management
- search.rs → search.py (1.3KB) - Search functionality
- tags.rs → tags.py (0.9KB) - Tag management
- base.rs → base.py (0.4KB) - Base utilities
- lib.rs → __init__.py (2.2KB) - Package interface

### Manager Files (Full Implementation)
- idea.rs → idea.py (5.8KB) - Idea management
- memory.rs → memory.py (6.2KB) - Memory management
- reminder.rs → reminder.py (5.6KB) - Reminder management
- summary.rs → summary.py (4.1KB) - Summary management

### Advanced Features (Full Implementation)
- chunking.rs → chunking.py (9.8KB) - Code generation chunking
- migration.rs → migration.py (4.5KB) - Task migration utilities
- extract.rs → extract.py (2.9KB) - Content extraction
- emb.rs → emb.py (6.7KB) - Embedding/ML service

### Infrastructure & Utilities (Stubs/Placeholders)
- cli.rs → cli.py (1.6KB) - CLI interface
- main.rs → main.py (0.4KB) - Main entry point
- server.rs → server.py (0.7KB) - Server functionality
- tdz.rs → tdz.py (1.3KB) - Core TDZ operations
- todozi.rs → todozi.py (1.0KB) - Main app interface
- tdz_tls.rs → tdz_tls.py (0.8KB) - Security utilities
- types.rs → types.py (1.2KB) - Type definitions
- tests.rs → tests.py (2.4KB) - Test suite
- tui.rs → tui.py (0.9KB) - Terminal UI (stub)
- validate_commands_docs.rs → validate_commands_docs.py (0.6KB) - Validation
- python.rs → python_bindings.py (0.4KB) - Python bindings
- nodejs.rs → nodejs_bindings.py (0.6KB) - Node.js bindings info

**Total: 28 files, ~120KB of Python code**

## Development

The Python version focuses on core functionality. Some advanced features from the Rust version may require additional implementation.

## License

Same as the main Todozi project.
