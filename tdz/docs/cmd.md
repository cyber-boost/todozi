### Top‑level commands (`Commands` enum)

| Command | Description / Sub‑commands |
|---------|----------------------------|
| **Init** | Initialise a Todozi folder structure |
| **Add** | Add resources – currently only `task` (see sub‑command) |
| **List** | List resources – currently only `tasks` |
| **Show** | Show a single resource – currently only `task` |
| **Update** | Update a task (many optional flags) |
| **Complete** | Mark a task as completed |
| **FixConsistency** | Repair task‑data consistency |
| **CheckStructure** | Verify the Todozi folder structure |
| **EnsureStructure** | Create missing folder components |
| **Register** | Register the client with the Todozi server (`--server-url`) |
| **RegistrationStatus** | Show current registration state |
| **ClearRegistration** | Remove stored registration data |
| **Delete** | Delete a task (`id`) |
| **Project** | Project‑related actions (see sub‑commands) |
| **Search** | Search resources (currently only `tasks`) |
| **Stats** | Show statistics (currently only `show`) |
| **Backup** | Create a backup (`create`) |
| **ListBackups** | List existing backups |
| **Restore** | Restore a named backup |
| **Memory** | Memory management (see sub‑commands) |
| **Idea** | Idea management (see sub‑commands) |
| **Agent** | Agent management (see sub‑commands) |
| **Emb** | Embedding‑service management (see sub‑commands) |
| **Error** | Error tracking (see sub‑commands) |
| **Train** | Training‑data handling (see sub‑commands) |
| **Chat** | Process a chat message |
| **SearchAll** | Unified search across all content types |
| **Maestro** | Maestro‑type orchestration (see sub‑commands) |
| **Server** | Server control (`start`, `status`, `endpoints`) |
| **ML** | Machine‑learning utilities (see sub‑commands) |
| **IndDemo** | Run the “individual demo” (placeholder) |
| **Queue** | Queue workflow (see sub‑commands) |
| **Api** | API‑key management (see sub‑commands) |
| **TdzCnt** | Process Todozi‑formatted content (`tdz_cnt`) |
| **ExportEmbeddings** | Export task embeddings to an HLX file |
| **Migrate** | Run the migration tool (`dry_run`, `verbose`, `force`, `cleanup`) |
| **Tui** | Launch the terminal UI |
| **Extract** | Extract tasks/memories/ideas from text or a file |
| **Strategy** | Run a strategic analysis on supplied content |

---

### Sub‑commands (selected groups)

| Parent command | Sub‑commands |
|----------------|--------------|
| **Add** | `task` – create a new task with fields `action`, `time`, `priority`, `project`, `status`, `assignee`, `tags`, `dependencies`, `context`, `progress`. |
| **List** | `tasks` – filter by `project`, `status`, `priority`, `assignee`, `tags`, `search`. |
| **Show** | `task {id}` – display a single task. |
| **Project** | `create`, `list`, `show`, `archive`, `delete`, `update`. |
| **Search** | `tasks {query}` – keyword search for tasks. |
| **Stats** | `show` – display overall statistics. |
| **Backup** | `create` – make a new backup file. |
| **Memory** | `create`, `create-secret`, `create-human`, `create-emotional`, `list`, `show`, `types`. |
| **Idea** | `create`, `list`, `show`. |
| **Agent** | `list`, `show`, `create`, `assign`, `update`, `delete`. |
| **Emb** | `set-model`, `show-model`, `list-models`. |
| **Error** | `create`, `list`, `show`, `resolve`, `delete`. |
| **Train** | `create`, `list`, `show`, `stats`, `export`, `collect`, `update`, `delete`. |
| **Maestro** | `init`, `collect-conversation`, `collect-tool`, `list`, `stats`, `export`, `integrate`. |
| **Server** | `start`, `status`, `endpoints`. |
| **ML** | `process`, `train`, `list`, `show`, `load`, `save`, `test`, `generate-training-data`, `advanced-process`, `advanced-train`, `advanced-infer`. |
| **Queue** | `plan`, `list`, `backlog`, `active`, `complete`, `start`, `end`. |
| **Api** | `register`, `list`, `check`, `deactivate`, `activate`, `remove`. |

---

### Quick usage example

```sh
# Initialise a new Todozi workspace
todozi init

# Add a task
todozi add task "Write documentation" --time "2h" --priority "high" --project "docs" --status "todo"

# List all tasks in the “docs” project
todo zi list tasks --project docs

# Show a specific task
todozi show task <task-id>

# Export embeddings
todozi export-embeddings --output my_embeddings.hlx