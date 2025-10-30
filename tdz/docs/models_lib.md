# `models.rs` – Core Data Structures

## Purpose
`models.rs` defines **all domain models** used throughout Todozi: tasks, agents, queues, tags, priorities, statuses, and various supporting primitives. These structs and enums are the foundation for storage, CLI, API, and AI‑agent interactions.

## Highlights

| Struct / Enum | Category | Key Fields |
|---------------|----------|------------|
| **Task** | Task | `id`, `action`, `time`, `priority`, `status`, `assignee`, `tags`, `project`, `dependencies`, `context`, `progress`, `created_at`, `updated_at` |
| **Priority** | Enum | `Low`, `Medium`, `High`, `Critical` |
| **Status** | Enum | `Todo`, `InProgress`, `Done`, `Blocked` |
| **Assignee** | Enum | `Human(String)`, `Agent(String)`, `None` |
| **Agent** | Agent | `id`, `name`, `description`, `category`, `capabilities`, `specializations`, `model_provider`, `model_name`, `temperature`, `max_tokens`, `tags`, `system_prompt`, `prompt_template`, `auto_format_code`, `include_examples`, `explain_complexity`, `suggest_tests`, `tools`, `max_response_length`, `timeout_seconds`, `requests_per_minute`, `tokens_per_hour`, `created_at`, `updated_at` |
| **QueueItem** | Queue | `id`, `task_name`, `task_description`, `priority`, `project_id`, `status`, `created_at` |
| **Tag** | Tag | Simple `String` wrapper with optional metadata |
| **Idea** | Idea | `id`, `text`, `share_level`, `importance`, `created_at`, `updated_at` |
| **Memory** | Memory | `id`, `moment`, `meaning`, `reason`, `importance`, `term`, `created_at`, `updated_at` |
| **Reminder** | Reminder | `id`, `message`, `trigger_at`, `repeat_interval`, `created_at` |
| **TodoziContentType** | Enum | `Task`, `Tag`, `Memory`, `Idea`, `Chunk`, `Feel`, `Train`, `Error`, `Summary`, `Reminder`, `Tdz` |
| **Project** | Project | `name`, `description`, `created_at`, `updated_at`, `status` |
| **EmbeddingCache** | Cache entry | `vector`, `content_type`, `content_id`, `text_content`, `tags`, `created_at`, `ttl_seconds` |

## Helper Types

| Type | Description |
|------|-------------|
| **TaskFilters** | Builder for query‑time filtering (by project, status, priority, assignee, tags, progress, date ranges). |
| **TaskUpdate** | Partial update struct used by the `update` CLI command (fields are `Option<T>` allowing selective patching). |
| **AgentAssignment** | Links an agent to a task and project, with status (`Assigned`, `Completed`) and timestamps. |
| **AgentStatistics** | Aggregates counts of agents by status, total assignments, and completion rate. |
| **ShareLevel** | Enum used by `Idea` (`Private`, `Team`, `Public`). |
| **MemoryTerm** | Enum for memory lifespan (`Short`, `Long`). |
| **ReminderTerm** | Enum for reminder recurrence (`OneShot`, `Recurring`). |
| **VersionInfo** | Holds version strings for the binary, schema, and embedding model. |

## Serialization

All structs derive `Serialize` / `Deserialize` via **serde**, enabling:

* JSON persistence in `~/.todozi/` (tasks, projects, agents, etc.).
* HLX storage for embeddings (`todozi_embeddings.hlx`).
* Easy conversion to/from the **Todozi** structured text format (`<todozi>…</todozi>`).

## Integration Points

| Module | Interaction |
|--------|-------------|
| **storage.rs** | Implements CRUD operations (`add_task_to_project`, `list_tasks_across_projects`, `save_agent`, `load_memory`). |
| **cli.rs** | Parses user input and calls the corresponding methods on the models (e.g., `Task::new`, `Agent::new`). |
| **emb.rs** | Generates embeddings for `Task`, `Memory`, `Idea`, `Chunk` using the `TodoziContentType` enum. |
| **agent.rs** | Uses `Agent`, `AgentAssignment`, and `AgentStatistics` to schedule AI agents. |
| **server.rs** | Exposes REST endpoints that accept/return the JSON representation of these models. |
| **lib.rs** | Re‑exports the key structs (`Task`, `Agent`, `Memory`, `Idea`, `QueueItem`) for external crates. |
| **todozi.rs** | Parses structured markup (`<todozi>`, `<memory>`, `<idea>`, `<chunk>`) and constructs the corresponding model instances. |

## Example: Creating a Task

```rust
use todozi::models::{Task, Priority, Status, Assignee};
use uuid::Uuid;

fn main() -> todozi::error::Result<()> {
    let task = Task::new(
        Uuid::new_v4().to_string(),
        "Implement OAuth login".to_string(),
        Some("2h".to_string()),
        Some(Priority::High),
        Status::Todo,
        Assignee::Human("alice".to_string()),
        vec!["auth".to_string(), "backend".to_string()],
        Some("security".to_string()),
        vec![], // no dependencies
        Some("Login flow for user authentication".to_string()),
        0,
    );
    // Persist the task via Storage
    // storage.add_task_to_project(task, "backend")?;
    Ok(())
}
```

## When to Extend `models.rs`

* Add a new domain entity (e.g., **FeatureFlag**, **Metric**) – create a struct with `Serialize`/`Deserialize` and update `Storage` accordingly.
* Introduce new enum variants (e.g., additional `Priority` levels) – update related UI/CLI handling and migration logic.
* Refactor for performance – consider using `Arc<Mutex<>>` for shared mutable state (e.g., `Agent` status).

---

*Generated by GPT‑OSS – documentation for the `models.rs` module.*
