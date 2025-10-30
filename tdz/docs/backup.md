# `todozi backup`

Create a snapshot of the entire Todozi data directory (`~/.todozi/`) so you can restore it later in case of corruption, accidental deletion, or migration.

## Sub‑command

| Sub‑command | Description |
|-------------|-------------|
| **create** | Compress the current Todozi workspace into a timestamped `.tar.gz` archive placed under `~/.todozi/backups/`. |

## Usage

```sh
# Generate a new backup (the file name includes the current date and time)
todozi backup create
```

The command prints the path of the generated archive, e.g.:

```
✅ Backup created successfully!
📦 /home/youruser/.todozi/backups/todozi_backup_2025-10-27_14-23-11.tar.gz
```

## How it works

1. **Collect files** – All files and sub‑directories inside `~/.todozi/` (tasks, projects, agents, memories, ideas, embeddings, etc.) are gathered.
2. **Create archive** – A temporary directory is created, the data is copied into it, and `tar` (via the `tar` crate) compresses the content using gzip compression.
3. **Store archive** – The resulting `.tar.gz` file is written to `~/.todozi/backups/` with a filename that includes an ISO‑8601 timestamp for easy identification.
4. **Cleanup** – The temporary directory is removed automatically after the archive is written.

## Restoration

To restore a backup, use the `todozi restore` command and pass the archive name:

```sh
todozi restore --backup-name todozi_backup_2025-10-27_14-23-11.tar.gz
```

The restore process extracts the archive back into `~/.todozi/`, overwriting any existing files (after prompting for confirmation).

## When to use `backup`

- **Before major migrations** (e.g., after running `todozi migrate`).
- **Before bulk deletions** or refactoring of projects/tasks.
- **Periodically** (e.g., via a cron job) to keep recent snapshots.
- **Before upgrading** the Todozi binary to a new version.

## Best practices

- Keep at least **3** recent backups: daily, weekly, and monthly.
- Store copies of critical backups on an external drive or cloud storage.
- Verify the integrity of a backup occasionally with `tar -tzf <archive>`.

## Related commands

- `todozi restore` – Load a previously created backup.
- `todozi migrate` – After migrating, create a backup to capture the new structure.
- `todozi export-embeddings` – Export embeddings separately if you need them in a different format.

For a full overview of all Todozi commands, see **`docs/cmd.md`**.
