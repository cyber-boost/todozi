# `todozi memory`

Manage *memories* – structured notes that the AI system can recall, associate with tasks, and use for semantic reasoning. Memories are stored in `~/.todozi/memories/` as JSON files and indexed in the HLX embedding cache for fast retrieval.

---

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **create** `--moment <MOMENT>` `--meaning <MEANING>` `--reason <REASON>` `--importance <IMPORTANCE>` `--term <TERM>` `[--memory-type <TYPE>]` `[--tags <TAGS>]` | Create a new **standard** memory. |
| **create-secret** `--moment <MOMENT>` `--meaning <MEANING>` `--reason <REASON>` `--importance <IMPORTANCE>` `--term <TERM>` `[--tags <TAGS>]` | Create a *secret* memory (visible only to AI, never shown to humans). |
| **create-human** `--moment <MOMENT>` `--meaning <MEANING>` `--reason <REASON>` `--importance <IMPORTANCE>` `--term <TERM>` `[--tags <TAGS>]` | Create a *human* memory (visible to the user in UI/CLI). |
| **create-emotional** `--moment <MOMENT>` `--meaning <MEANING>` `--reason <REASON>` `--emotion <EMOTION>` `--importance <IMPORTANCE>` `--term <TERM>` `[--tags <TAGS>]` | Create an *emotional* memory that includes an `emotion` field for affect‑driven reasoning. |
| **list** `[--importance <IMPORTANCE>]` `[--term <TERM>]` `[--memory-type <TYPE>]` | List memories, optionally filtered by importance, term length, or type (`standard`, `secret`, `human`, `emotional`). |
| **show** `--id <ID>` | Display the full JSON representation of a specific memory. |
| **types** | Print the set of supported memory types. |

---

## Usage examples

```sh
# Create a regular memory
todozi memory create \
    --moment "Discussed API versioning" \
    --meaning "We decided to bump minor version for compatible changes" \
    --reason "Maintain backward compatibility" \
    --importance high \
    --term short \
    --tags "api,versioning"

# Create a secret memory (AI‑only)
todozi memory create-secret \
    --moment "User mentioned a private API key" \
    --meaning "Store it securely for future AI‑driven actions" \
    --reason "Avoid exposing the key in logs" \
    --importance medium \
    --term short

# List all high‑importance memories
todozi memory list --importance high

# Show a specific memory
todozi memory show --id 3f5e2a9c-1b4d-4e6a-9f7c-2d8b3a5e6f1a
```

---

## How it works

1. **Storage** – Each memory is saved as a JSON file under `~/.todozi/memories/`. The filename is the UUID of the memory (`<id>.json`).
2. **Embedding** – When a memory is created, its textual content (`moment`, `meaning`, `reason`, and optional `emotion`) is passed to `TodoziEmbeddingService`. The resulting vector is stored in `todozi_embeddings.hlx` under the `memory` namespace.
3. **Visibility** –
   - *Standard* and *human* memories can be listed and shown via CLI.
   - *Secret* memories are omitted from any human‑facing output; they are only available to the AI during semantic searches.
   - *Emotional* memories include an `emotion` field that the AI can weight during affect‑aware reasoning.
4. **Filtering** – The `list` command reads all memory files, deserialises them into `Memory` structs, and applies the requested filters before printing a table via the `tabled` crate.
5. **Deletion / Update** – Currently not exposed as a CLI sub‑command; memory lifecycle is managed via the API or programmatically using the library.

---

## When to use `memory`

- **Knowledge retention** – Capture insights, decisions, or contextual facts that the AI should remember across sessions.
- **Sensitive information** – Store private data (e.g., API keys, passwords) as *secret* memories so they never appear in user‑visible output.
- **Emotional context** – Record affective states (`emotion` field) to enable the AI to respond with empathy or to prioritize tasks that alleviate negative emotions.
- **Long‑term planning** – Use the `term` field (`short` vs. `long`) to indicate how long the memory should stay relevant; the system can prune old short‑term memories automatically.

---

## Related commands

- `todozi add task` – Tasks can reference a memory ID to link a concrete action with a stored insight.
- `todozi search` – Keyword search does **not** look into secret memories; semantic search (`todozi search-all`) does.
- `todozi emb` – Change the embedding model to improve memory similarity matching.
- `todozi backup` – Back up the entire `~/.todozi/` directory, including all memories.
- `todozi server` – The server exposes a `/memories` endpoint that respects the visibility rules defined above.

---

For a complete list of top‑level commands, see **`docs/cmd.md`**.
