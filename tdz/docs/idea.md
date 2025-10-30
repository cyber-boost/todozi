# `todozi idea`

Manage **ideas** – high‑level concepts, proposals, or brainstorming items that may later become tasks, projects, or research items. Ideas are stored as JSON files in `~/.todozi/ideas/` and can be enriched with tags, importance levels, and optional context.

---

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **create** `--idea <IDEA>` `[--share <LEVEL>]` `[--importance <IMPORTANCE>]` `[--tags <TAGS>]` `[--context <CONTEXT>]` | Create a new idea. |
| **list** `[--share <LEVEL>]` `[--importance <IMPORTANCE>]` | List existing ideas, optionally filtered by share level or importance. |
| **show** `--id <ID>` | Display the full JSON representation of a specific idea. |

---

## Usage examples

```sh
# Create a personal idea with medium importance
todozi idea create \
    --idea "Add a dark mode to the UI" \
    --share private \
    --importance medium \
    --tags "ui,theme" \
    --context "User requests have increased for a dark mode."

# List all public ideas
todozi idea list --share public

# Show a particular idea (replace with the actual UUID)
todozi idea show --id 7c3e9f12-4b5a-4d9e-9a6f-1e2b3c4d5e6f
```

---

## How it works

1. **Storage** – Upon creation, a UUID is generated and a JSON file `<id>.json` is written under `~/.todozi/ideas/`. The file contains:
   ```json
   {
     "id": "7c3e9f12-4b5a-4d9e-9a6f-1e2b3c4d5e6f",
     "idea": "Add a dark mode to the UI",
     "share": "private",
     "importance": "medium",
     "tags": ["ui","theme"],
     "context": "User requests have increased for a dark mode.",
     "created_at": "2025-10-27T14:35:22Z"
   }
   ```

2. **Embedding (optional)** – When the embedding service is active, the idea text is sent to `TodoziEmbeddingService` to generate a vector stored in `todozi_embeddings.hlx`. This enables semantic search across ideas.

3. **Filtering** – The `list` command reads all JSON files, deserialises them into `Idea` structs, and applies any supplied filters (`share`, `importance`). Results are displayed in a table using the `tabled` crate.

4. **Visibility** –
   - `private` ideas are shown only to the user via CLI.
   - `public` ideas are also exposed through the server's `/ideas` endpoint for collaborative environments.

---

## When to use `idea`

- **Brainstorming sessions** – Capture raw thoughts without committing to a concrete task.
- **Feature proposals** – Store high‑level feature concepts that may later be broken down into tasks or projects.
- **Research tracking** – Record topics you want to explore later, complete with tags and context for easy retrieval.
- **Collaborative planning** – Share ideas across a team (`--share public`) and let agents suggest implementations.

---

## Related commands

- `todozi add task` – Turn an idea into an actionable task (`todozi add task "Implement dark mode"`).
- `todozi search-all` – Perform a semantic search that includes ideas alongside tasks, memories, and errors.
- `todozi emb` – Change the embedding model to improve idea similarity matching.
- `todozi backup` – Ensure ideas are preserved in backups.
- `todozi server` – Access ideas via the `/ideas` REST endpoint.

For a full overview of all commands, see **`docs/cmd.md`**.
