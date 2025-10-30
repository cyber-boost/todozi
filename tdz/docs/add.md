# `todozi add`

Add new resources to your Todozi workspace.
At the moment the only supported sub‑command is **task**, which creates a new task entry.

## Usage

```sh
todozi add task <ACTION> --time <TIME> --priority <PRIORITY> --project <PROJECT> [OPTIONS]
```

### Arguments

| Flag | Description | Required |
|------|-------------|----------|
| `<ACTION>` | The textual description of what the task does. | Yes |
| `--time <TIME>` | Expected effort or duration (e.g., `2h`, `30m`). | Yes |
| `--priority <PRIORITY>` | Task priority (`low`, `medium`, `high`). | Yes |
| `--project <PROJECT>` | Project name the task belongs to. | Yes |

### Optional flags

| Flag | Description |
|------|-------------|
| `-s, --status <STATUS>` | Initial status, defaults to `todo`. |
| `-u, --assignee <ASSIGNEE>` | Human or agent that will own the task. |
| `--tags <TAGS>` | Comma‑separated list of tags. |
| `--dependencies <DEPS>` | Comma‑separated list of task IDs this task depends on. |
| `-c, --context <CONTEXT>` | Additional free‑form context information. |
| `-p, --progress <PROGRESS>` | Initial progress percentage (0‑100). |

## Example

```sh
todozi add task "Write project documentation" \
    --time "3h" \
    --priority "high" \
    --project "docs" \
    --status "todo" \
    --tags "writing,internal" \
    --context "Include API usage examples"
```

This command creates a new task, generates an embedding for it, stores it under `~/.todozi/tasks/docs/` and updates the HLX index.

## What happens under the hood

1. **Parsing** – CLI arguments are parsed into a `Task` struct.
2. **Embedding** – The task description (action, tags, context, etc.) is turned into an embedding vector by `TodoziEmbeddingService`.
3. **Storage** – The task (with its embedding) is written to a JSON file in the appropriate project folder.
4. **Index update** – The `todozi.hlx` index is refreshed so the task is immediately searchable.

## Related commands

- `todozi list tasks` – view tasks (you can filter by project, priority, etc.).
- `todozi update` – modify an existing task.
- `todozi complete` – mark a task as finished.

Refer to **`cmd.md`** for a complete overview of all top‑level commands.
