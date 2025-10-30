# `todozi show`

Display a single resource (currently only **tasks**) in a human‑readable format.
The command is primarily useful when you have the task identifier and want to
inspect all of its fields, including automatically generated data such as the
embedding vector summary, creation timestamps, and any attached tags.

## Usage

```sh
todozi show task <TASK_ID>
```

* `<TASK_ID>` – The UUID of the task you wish to view (as printed by `list`,
  `add`, or `search`).

## Output

The command prints a formatted table that includes:

| Field               | Description                                                                 |
|---------------------|-----------------------------------------------------------------------------|
| **ID**              | Unique identifier (UUID).                                                   |
| **Action**          | Human‑readable description of the work to be performed.                     |
| **Time**            | Estimated effort or duration (e.g., `2h`, `30m`).                           |
| **Priority**        | `low` · `medium` · `high`.                                                 |
| **Project**         | Name of the project the task belongs to.                                    |
| **Status**          | Current state (`todo`, `in‑progress`, `done`, …).                            |
| **Assignee**        | Human or agent responsible for the task.                                    |
| **Tags**            | Comma‑separated list of user‑defined tags.                                  |
| **Dependencies**    | IDs of tasks this one depends on.                                           |
| **Context**         | Optional free‑form notes.                                                  |
| **Progress**        | Percentage (0‑100) indicating how far along the task is.                     |
| **Created At**      | Timestamp of task creation (UTC).                                           |
| **Updated At**      | Timestamp of the latest update (UTC).                                       |
| **Embedding Summary**| Length of the embedding vector and a short checksum (useful for debugging). |

If the task does not exist, the command exits with an error message:

```
❌ No task found with ID <TASK_ID>
```

## How it works

1. **Lookup** – The CLI reads the task file from `~/.todozi/tasks/<project>/<id>.json`.
2. **Deserialization** – The JSON is parsed into a `Task` struct.
3. **Formatting** – `tabled` renders the fields into a compact table.
4. **Embedding display** – A short digest of the embedding vector (first three values and a checksum) is shown to confirm that the semantic representation exists.

## Related commands

- `todozi list tasks` – Locate the ID you need before calling `show`.
- `todozi update` – Modify any of the fields displayed by `show`.
- `todozi complete` – Mark the task as finished after you have inspected it.

For a complete list of all top‑level commands, see **`docs/cmd.md`**.
