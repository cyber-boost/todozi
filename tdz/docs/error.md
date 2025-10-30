# `todozi error`

Manage **errors** – structured records of runtime or validation problems that occur within Todozi or its agents. Errors are stored in `~/.todozi/errors/` as JSON files and can be listed, inspected, resolved, or deleted. The error subsystem is used by the CLI, agents, and the server to surface problems to the user and to provide a persistent audit trail.

---

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **create** `--title <TITLE>` `--description <DESCRIPTION>` `--severity <SEVERITY>` `--category <CATEGORY>` `--source <SOURCE>` `[--context <CONTEXT>]` `[--tags <TAGS>]` | Create a new error record. |
| **list** `[--severity <SEVERITY>]` `[--category <CATEGORY>]` `[--unresolved-only]` | List errors, optionally filtered by severity, category, or unresolved status. |
| **show** `--id <ID>` | Display the full JSON representation of a specific error. |
| **resolve** `--id <ID>` `[--resolution <RESOLUTION>]` | Mark an error as resolved and optionally add a human‑readable resolution note. |
| **delete** `--id <ID>` | Permanently delete an error record (asks for confirmation). |

---

## Usage examples

```sh
# Register a new runtime error
todozi error create \
    --title "Failed to parse task file" \
    --description "JSON parsing failed for tasks/backend/12345.json" \
    --severity high \
    --category runtime \
    --source "todozi::storage::load_task" \
    --context "While loading tasks for the backend project" \
    --tags "json,parse"

# List all high‑severity errors
todozi error list --severity high

# Show a particular error (replace <ID> with the actual UUID)
todozi error show --id 7c3e9f12-4b5a-4d9e-9a6f-1e2b3c4d5e6f

# Resolve an error with a remediation note
todozi error resolve \
    --id 7c3e9f12-4b5a-4d9e-9a6f-1e2b3c4d5e6f \
    --resolution "Fixed malformed JSON by adding missing commas."

# Delete an obsolete error record
todozi error delete --id 7c3e9f12-4b5a-4d9e-9a6f-1e2b3c4d5e6f
```

---

## How it works

1. **Storage** – When `create` is invoked, a UUID is generated and a JSON file `<id>.json` is written under `~/.todozi/errors/`. The file contains:
   ```json
   {
     "id": "7c3e9f12-4b5a-4d9e-9a6f-1e2b3c4d5e6f",
     "title": "Failed to parse task file",
     "description": "JSON parsing failed for tasks/backend/12345.json",
     "severity": "high",
     "category": "runtime",
     "source": "todozi::storage::load_task",
     "context": "While loading tasks for the backend project",
     "tags": ["json","parse"],
     "resolved": false,
     "resolution": null,
     "created_at": "2025-10-27T15:12:34Z",
     "updated_at": "2025-10-27T15:12:34Z"
   }
   ```

2. **Listing** – The `list` sub‑command reads all files in the errors directory, deserialises them into `Error` structs, applies the requested filters, and prints a compact table via the `tabled` crate. Unresolved errors have `resolved: false`.

3. **Resolution** – `resolve` updates the `resolved` flag to `true`, stores the optional `resolution` note, and updates `updated_at`. This allows you to keep a historical record of how an issue was addressed.

4. **Deletion** – `delete` removes the corresponding JSON file after a safety prompt, ensuring that accidental deletions are avoided.

5. **Embedding (optional)** – Errors can be indexed by the embedding service for semantic search using `todozi search-all`. This is useful for discovering similar failures across runs.

---

## When to use `error`

- **Debugging** – Capture unexpected panics, file‑I/O failures, or validation problems.
- **Auditing** – Maintain a persistent log of issues for compliance or post‑mortem analysis.
- **Automation** – Agents can report errors back to the system, which you can later query and resolve.
- **Monitoring** – Periodically list unresolved high‑severity errors to drive operational alerts.

---

## Related commands

- `todozi server` – The server exposes `/errors` endpoints that respect the same CRUD operations.
- `todozi backup` – Errors are included in workspace backups, ensuring you retain the full error history.
- `todozi migrate` – Migration tools will preserve error records and update their schema if needed.
- `todozi emb` – Switch embedding models to improve semantic search across error messages.

For a full overview of all top‑level commands, see **`docs/cmd.md`**.
