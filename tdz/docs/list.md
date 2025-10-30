# `todozi list`

List existing resources stored in your Todozi workspace.
At the moment the only supported sub‑command is **tasks**, which outputs a table of tasks with optional filters.

## Usage

```sh
todozi list tasks [OPTIONS]
```

### Options

| Flag | Description |
|------|-------------|
| `-p, --project <PROJECT>` | Filter tasks belonging to a specific project. |
| `-s, --status <STATUS>`   | Filter by task status (`todo`, `in‑progress`, `done`, etc.). |
| `-r, --priority <PRIORITY>` | Filter by priority (`low`, `medium`, `high`). |
| `-a, --assignee <ASSIGNEE>` | Show only tasks assigned to a particular user or agent. |
| `-g, --tags <TAGS>` | Show tasks that contain **all** of the supplied comma‑separated tags. |
| `-e, --search <TEXT>` | Full‑text search across task titles, context, and tags. |

## Example

```sh
# List all high‑priority tasks in the "backend" project that are still open
todozi list tasks --project backend --priority high --status todo
```

```sh
# Search for any tasks mentioning "authentication"
todozi list tasks --search authentication
```

## What the command does

1. **Loads** the task files from `~/.todozi/tasks/` (or the folder configured via `TDZ_HOME`).
2. **Applies** any supplied filters, performing a quick in‑memory scan.
3. **Prints** a nicely formatted table (using the `tabled` crate) that includes:
   - ID
   - Action
   - Project
   - Status
   - Priority
   - Assignee
   - Tags
   - Progress
4. If no tasks match the filter set, a friendly *“No tasks found”* message is shown.

## Related commands

- `todozi add task` – create new tasks that will appear in `list`.
- `todozi update` – modify a task’s fields, after which `list` will reflect the changes.
- `todozi complete` – mark a task as done; it will still appear unless filtered out.

Check **`cmd.md`** for a full overview of all available top‑level commands.
