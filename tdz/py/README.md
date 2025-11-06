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

✅ Completed:
- error.rs → error.py
- models.rs → models.py
- storage.rs → storage.py
- agent.rs → agent.py
- api.rs → api.py
- search.rs → search.py
- tags.rs → tags.py
- base.rs → base.py
- lib.rs → __init__.py

📋 Remaining (optional):
- emb.rs (embeddings)
- chunking.rs
- cli.rs / types.rs (CLI-specific)
- server.rs
- tui.rs
- And other utility files

## Development

The Python version focuses on core functionality. Some advanced features from the Rust version may require additional implementation.

## License

Same as the main Todozi project.
