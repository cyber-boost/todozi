# `todozi search`

Perform a keyword‑based search for tasks.
The command scans the textual content of every stored task and returns those that match the given query string.

## Usage

```sh
todozi search tasks <QUERY>
```

- `<QUERY>` – Any string you want to look for (e.g., `"authentication"`, `"refactor"`).

## What you get

A table with the matching tasks, showing at least the following columns:

| ID | Action | Project | Tags | Status | Priority |
|----|--------|---------|------|--------|----------|

If no tasks match, the command prints:

```
📭 No tasks found for query "<QUERY>"
```

## How it works

1. **Load tasks** – All task files under `~/.todozi/tasks/…` are read into memory.
2. **Simple filter** – The query string is lower‑cased and compared against the concatenated fields `action`, `context`, `tags`, and `project`.
3. **Result formatting** – Matching tasks are sorted by creation date (newest first) and displayed using the `tabled` crate.

## When to use `search`

- Quickly locate a task when you only remember a fragment of its description.
- Find all tasks that reference a particular library, endpoint, or code concept.
- Combine with other CLI tools (e.g., pipe the IDs to `todozi show`).

## Related commands

- `todozi list tasks` – List tasks with structured filters (project, priority, etc.).
- `todozi search-all` – Run a semantic (embedding‑based) search across tasks, memories, ideas, and more.
- `todozi find` – Unified search that mixes keyword and AI‑driven semantic results.

For a full overview of all commands, see **`docs/cmd.md`**.
