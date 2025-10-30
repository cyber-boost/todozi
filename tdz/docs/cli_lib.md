# `cli.rs` – Todozi CLI Command Handling Overview

## Purpose
`cli.rs` implements the **command‑line interface** for Todozi.
It bridges user‑issued sub‑commands to the core library services:

- Task CRUD operations (`add`, `list`, `complete`, `delete`, …)
- Queue management (`plan`, `list`, `start`, `end`)
- API key administration (`api register`, `api list`, …)
- Server control (`server start`, `server status`, `server endpoints`)
- Optional TUI integration (when the `tui` feature is enabled)

The file defines the central `TodoziHandler` struct that holds a `Storage` instance and provides both **synchronous** helper methods and **asynchronous** command dispatchers.

## Core Types

| Type | Category | Description |
|------|----------|-------------|
| `TodoziHandler` | Struct | Holds a `Storage` instance and groups CLI‑related actions. |
| `ApiCommands` | Enum (in `types.rs`) | Variants for all API‑key related sub‑commands (`Register`, `List`, `Check`, `Deactivate`, `Activate`, `Remove`). |
| `QueueCommands` | Enum (in `types.rs`) | Variants for queue operations (`Plan`, `List`, `Backlog`, `Active`, `Complete`, `Start`, `End`). |
| `ServerCommands` | Enum (in `types.rs`) | Variants for server management (`Start`, `Status`, `Endpoints`). |
| `Result<T>` / `TodoziError` | Types | Unified error handling used throughout the CLI. |
| `TodoziEmbeddingService` | Service (used in the async handlers) | Provides embedding generation for AI‑enhanced commands (e.g., `search`). |
| `TuiService` (feature‑gated) | Service | Handles the optional terminal UI (`tui` feature). |

## Key Methods

### Synchronous Helpers
| Method | Signature | Action |
|--------|-----------|--------|
| `new(storage: Storage) -> Self` | `fn(Storage) -> Self` | Construct a new handler. |
| `complete_task(&mut self, id: &str) -> Result<()>` | `fn(&mut self, &str) -> Result<()>` | Mark a task as completed. |
| `fix_task_consistency(&mut self) -> Result<()>` | `fn(&mut self) -> Result<()>` | Repair any corrupted task state. |
| `delete_task(&mut self, id: &str) -> Result<()>` | `fn(&mut self, &str) -> Result<()>` | Remove a task from storage. |
| `restore_backup(&mut self, backup_name: &str) -> Result<()>` | `fn(&mut self, &str) -> Result<()>` | Restore a previous backup file. |

### Asynchronous Command Dispatchers
| Method | Signature | Description |
|--------|-----------|-------------|
| `handle_api_command(&self, command: ApiCommands) -> Result<()>` | `async fn(&self, ApiCommands) -> Result<()>` | Executes any `api` sub‑command, printing human‑readable output. |
| `handle_queue_command(&self, command: QueueCommands) -> Result<()>` | `async fn(&self, QueueCommands) -> Result<()>` | Performs queue‑related actions (plan, list, start, end). |
| `handle_server_command(&self, command: ServerCommands) -> Result<()>` | `async fn(&self, ServerCommands) -> Result<()>` | Starts the server, checks status, or prints endpoint documentation. |

All async handlers return `Result<()>` and internally call the lower‑level library functions (e.g., `crate::api::create_api_key`, `crate::server::start_server`).

## Typical Execution Flow (Mermaid Diagram)

```mermaid
flowchart TD
    A[Parse CLI args] --> B[Instantiate TodoziHandler]
    B --> C{Command Group}
    C -->|api| D[handle_api_command]
    C -->|queue| E[handle_queue_command]
    C -->|server| F[handle_server_command]
    C -->|task| G[Sync helpers (complete, delete, …)]
    D --> H[Print human‑readable result]
    E --> H
    F --> H
    G --> H
    H --> I[Exit with status code]
```

## Integration Points

| Module | Interaction |
|--------|-------------|
| `storage.rs` | Provides the persistent file‑based backend used by `TodoziHandler`. |
| `api.rs` | Supplies all API‑key management functions invoked by `handle_api_command`. |
| `queue.rs` (via `crate::queue::*`) | Implements storage and querying of `QueueItem`s used by queue commands. |
| `server.rs` | Contains the async TCP server started by `ServerCommands::Start`. |
| `emb.rs` | May be used by future AI‑enhanced CLI commands (e.g., semantic search). |
| `tui.rs` (feature‑gated) | When compiled with `tui`, the CLI can launch an interactive terminal UI. |

## Example Usage

```sh
# Initialise storage (first run)
todozi init

# Create a new task
todozi add "Write documentation for CLI" -p high -t docs

# Mark a task as done
todozi complete 123e4567-e89b-12d3-a456-426614174000

# List all active queue items
todozi queue list --status active

# Register a new API key for a user
todozi api register --user-id alice

# Start the enhanced server on a custom port
todozi server start --host 0.0.0.0 --port 8636
```

The CLI parses the arguments, creates a `TodoziHandler` (which internally builds a `Storage` pointing at `~/.todozi/`), and then dispatches to the appropriate async handler.

## Extending the CLI

1. **Add a new command group** – Define a new enum variant in `types.rs` (e.g., `BackupCommands`).
2. **Implement a handler** – Add an `async fn handle_backup_command(&self, command: BackupCommands) -> Result<()>` method following the pattern of the existing handlers.
3. **Wire it up** – Extend the match block in the main `run` function (or `clap` configuration) to route the new sub‑command to the handler.
4. **Document** – Add a new `*_lib.md` file in `docs/` mirroring the style used here.

## Testing

The CLI is exercised by integration tests located in `src/tests.rs`. They invoke the handler methods directly, avoiding OS‑level process spawning, which keeps the test suite fast and deterministic.

## See Also

- `src/lib.rs` – Public API façade used by external Rust projects.
- `src/models.rs` – Data structures for tasks, queues, agents, etc.
- `src/types.rs` – Command enums (`ApiCommands`, `QueueCommands`, `ServerCommands`).
- `src/docs/agent_lib.md` – Documentation of the agent system that consumes CLI‑initiated operations.

---

