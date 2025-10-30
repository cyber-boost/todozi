# `todozi queue`

The **queue** subsystem lets you stage work items that are not yet formal tasks, track their priority, and move them through a lightweight workflow (backlog → active → complete). It is especially handy for brainstorming ideas, triaging incoming requests, or managing short‑term to‑do items without creating full task objects.

---

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **plan** `--task-name <NAME>` `--task-description <DESC>` `--priority <PRIORITY>` `[--project-id <PROJECT>]` | Add a new item to the queue. The priority can be `low`, `medium`, or `high`. |
| **list** `[--status <STATUS>]` | Show all queue items, optionally filtered by status (`backlog`, `active`, `complete`). |
| **backlog** | List only items that are waiting to be started. |
| **active** | List items that are currently being worked on. |
| **complete** | List items that have been finished. |
| **start** `--queue-item-id <ID>` | Promote a backlog item to the active state and start a tracking session. |
| **end** `--session-id <ID>` | End an active session, marking the associated queue item as complete. |

---

## Usage examples

```sh
# Plan a new item (high priority) for the "research" project
todozi queue plan \
    --task-name "Read latest AI paper" \
    --task-description "Summarize findings and add to knowledge base" \
    --priority high \
    --project-id research

# Show everything that is still waiting
todozi queue backlog

# Begin work on a specific queue item
todozi queue start --queue-item-id 73a1f2c4

# When you finish, close the session
todozi queue end --session-id 5f2e9b1a
```

---

## How it works

1. **Storage** – Queue items are persisted in `~/.todozi/queue/` as JSON files. Each entry contains:
   - `id` – a UUID.
   - `task_name` & `task_description`.
   - `priority`.
   - Optional `project_id`.
   - `status` – one of `Backlog`, `Active`, `Complete`.
   - Timestamps for creation, start, and completion.

2. **Session tracking** – When you run `queue start`, a **session** record is created (`queue_sessions/`). The session captures:
   - The associated queue‑item ID.
   - Start time.
   - End time (filled on `queue end`).
   - Duration (seconds) computed automatically.

3. **Progress reporting** – The `list` sub‑command prints a table with the current status, priority, and elapsed time (for active items). The output uses the `tabled` crate for nice alignment.

4. **Integration with tasks** – After a queue item is completed you may want to turn it into a full‑blown task. The ID can be copied into `todozi add task` or you can write a custom script that reads from the queue JSON files and creates tasks programmatically.

---

## When to use the queue

- **Ad‑hoc ideas** that haven’t been fully scoped yet.
- **Incoming requests** (e.g., support tickets) that need triage before becoming tasks.
- **Small, repeatable actions** where you want a lightweight checklist without full task metadata.
- **Time tracking**: start/end commands give you a quick way to log time spent on an item.

---

## Related commands

- `todozi add task` – Convert a queued item into a formal task.
- `todozi list tasks` – View tasks after they have been created.
- `todozi complete` – Mark a full task as done (queue items have their own completion flow).
- `todozi backup` – Preserve the queue state before performing bulk migrations.

---

*Documentation for the `queue` command, generated for the Todozi CLI.*
