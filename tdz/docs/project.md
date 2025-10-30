# `todozi project`

The **project** command is used to manage collections of tasks under a common namespace. Projects give you a way to organise your work, apply bulk operations, and isolate task views.

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **create** `--name <NAME>` `[--description <DESC>]` | Initialise a new project. A directory `~/.todozi/projects/<NAME>/` is created and a `project.json` descriptor is written. |
| **list** | Show all existing projects with their descriptions, number of tasks, and creation date. |
| **show** `--name <NAME>` | Display detailed information about a single project, including its task count, tags, and any custom metadata stored in `project.json`. |
| **archive** `--name <NAME>` | Move a project to the `archive/` sub‑folder, effectively de‑activating it while keeping its data for later reference. |
| **delete** `--name <NAME>` | Permanently remove a project and all of its tasks. The command asks for confirmation before deleting. |
| **update** `--name <NAME>` `[--new-name <NEW_NAME>]` `[--description <DESC>]` `[--status <STATUS>]` | Rename a project, modify its description, or change its status (e.g., `active`, `archived`). |

## Usage examples

```sh
# Create a new project called "backend"
todozi project create --name backend --description "Server‑side services and APIs"

# List all projects
todozi project list

# Show details about the "frontend" project
todozi project show --name frontend

# Rename a project
todozi project update --name old_name --new-name new_name

# Archive a completed project
todozi project archive --name finished_sprint

# Delete a project that is no longer needed
todozi project delete --name obsolete
```

## How it works

1. **Filesystem layout** – Each project lives under `~/.todozi/projects/<project_name>/`. Task files for that project are stored in `tasks/` inside the project directory.
2. **Metadata file** – `project.json` stores a small JSON object:
   ```json
   {
     "name": "backend",
     "description": "Server‑side services and APIs",
     "created_at": "2025-10-27T12:34:56Z",
     "status": "active"
   }
   ```
   The CLI reads and updates this file for `show`, `update`, and `archive`.
3. **Task association** – When a task is created with `--project <NAME>`, the task file is placed in the corresponding project folder and its metadata (`project` field) is set to the project name.
4. **Archiving** – Archiving simply moves the whole project directory under `~/.todozi/projects/archive/<NAME>/`. The `list` command hides archived projects unless `--include-archived` is added (future feature).
5. **Safety** – Deleting a project triggers a confirmation prompt (`dialoguer`) and uses atomic file system operations to avoid partial deletions.

## When to use `project`

- **Large codebases** where grouping related tasks (e.g., “frontend”, “backend”, “infrastructure”) makes navigation easier.
- **Sprints or milestones** – create a project per sprint, then archive it when the sprint ends.
- **Domain separation** – keep personal tasks distinct from work‑related tasks.

## Related commands

- `todozi add task` – Assign a task to a project with `--project <NAME>`.
- `todozi list tasks` – Filter by project using `--project <NAME>`.
- `todozi backup` – Include all projects in a backup snapshot.
- `todozi migrate` – Migrates tasks across projects; useful when reorganising project structures.

For a complete overview of all top‑level commands, see **`docs/cmd.md`**.
