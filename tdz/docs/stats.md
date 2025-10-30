# `todozi stats`

Show global statistics for the Todozi workspace.
The command aggregates information about tasks, projects, agents, memories, and other entities stored under `~/.todozi/`.

## Usage

```sh
todozi stats show
```

The sub‑command `show` is the only variant currently implemented.

## What is displayed

| Metric | Description |
|--------|-------------|
| **Total Tasks** | Number of tasks across all projects. |
| **Open Tasks** | Tasks whose status is not `done` or `archived`. |
| **Completed Tasks** | Tasks with status `done`. |
| **Projects** | Number of distinct projects (including archived). |
| **Active Projects** | Projects whose status is `active`. |
| **Agents** | Total number of registered agents. |
| **Memories** | Count of stored memories (short‑term & long‑term). |
| **Ideas** | Number of ideas recorded. |
| **Embeddings Cached** | Number of embedding vectors currently stored in `todozi_embeddings.hlx`. |
| **Storage Size** | Approximate size on disk of the `~/.todozi/` directory. |
| **Uptime** | How long the Todozi daemon (if running) has been up. |

The table is printed using the `tabled` crate for a clean, aligned layout.

## Example output

```text
╭───────────────────────┬─────────────────────╮
│ Metric                │ Value               │
├───────────────────────┼─────────────────────┤
│ Total Tasks           │ 342                 │
│ Open Tasks            │ 128                 │
│ Completed Tasks       │ 214                 │
│ Projects              │ 7                   │
│ Active Projects       │ 5                   │
│ Agents                │ 12                  │
│ Memories              │ 58                  │
│ Ideas                 │ 23                  │
│ Embeddings Cached     │ 342                 │
│ Storage Size          │ 12.4 MiB            │
│ Uptime                │ 3 days, 4 h, 12 m   │
╰───────────────────────┴─────────────────────╯
```

## How it works

1. **Data collection** – The handler iterates over the JSON files in the `tasks/`, `projects/`, `agents/`, `memories/`, and `ideas/` directories, counting entries and reading minimal metadata.
2. **Embedding cache** – The size of `todozi_embeddings.hlx` is obtained via file metadata; the number of cached vectors is derived from the HLX key count.
3. **Disk usage** – `std::fs::metadata` and `walkdir` are used to sum the size of all files under `~/.todozi/`.
4. **Formatting** – Results are assembled into a `StatsReport` struct and displayed with `tabled`.

## When to use `stats`

- **Capacity planning** – Understand how many tasks you have and whether you need to archive or prune.
- **Health checks** – Verify that the embedding cache is populated and the storage size is reasonable.
- **Reporting** – Quick snapshot for status meetings or personal retrospectives.

## Related commands

- `todozi backup` – Create a backup of all data shown in the statistics.
- `todozi migrate` – After migrating, you can compare pre‑ and post‑migration stats.
- `todozi server status` – Combine with server uptime for a full system view.

For a full list of commands, see **`docs/cmd.md`**.
