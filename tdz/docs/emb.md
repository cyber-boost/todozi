# `todozi emb`

Manage the embedding service that powers Todozi’s semantic search.

The **emb** command is a thin wrapper around the `TodoziEmbeddingService`. It lets you view, set, and enumerate the embedding models used when generating vector representations for tasks, memories, ideas, code chunks, etc.

---

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **set-model** `model_name` | Change the default embedding model. The model name must be a valid HuggingFace identifier (e.g., `sentence-transformers/all-MiniLM-L6-v2`). |
| **show-model** | Print the currently selected model name. |
| **list-models** | Show a curated list of popular embedding models that Todozi supports out‑of‑the‑box. |

---

## Usage examples

```sh
# Show which model is currently active
todozi emb show-model
# → sentence-transformers/all-MiniLM-L6-v2

# Switch to a different model
todozi emb set-model sentence-transformers/paraphrase-MiniLM-L3-v2
# The new model will be downloaded (if not cached) and used for subsequent embeddings.

# List the models that are known to work well with Todozi
todozi emb list-models
# → sentence-transformers/all-MiniLM-L6-v2
# → sentence-transformers/paraphrase-MiniLM-L3-v2
# → nlp‑ai/bert‑base‑uncased‑embedding
```

---

## What happens under the hood

1. **Configuration** – The selected model name is persisted in `~/.todozi/tdz.hlx` under the `embedding` namespace, so the choice survives restarts.
2. **Model loading** – When the `TodoziEmbeddingService` is instantiated, it reads the configuration and loads the model via the `candle` crate (or the appropriate backend). If the model is not yet cached, it is downloaded from HuggingFace.
3. **Cache handling** – For each piece of content, an embedding vector is generated and stored in the `todozi_embeddings.hlx` file. Subsequent requests for the same text will retrieve the cached vector, dramatically speeding up repeated searches.
4. **Thread‑safety** – The service uses an async‑aware LRU cache, ensuring that concurrent CLI invocations share the same model instance without re‑loading it.

---

## When to use `emb`

- **Switching models** – If you need more accurate embeddings (e.g., for domain‑specific vocabulary), pick a larger model with `set-model`.
- **Diagnosing performance** – `show-model` helps you confirm which model your instance is actually using.
- **Exploring options** – `list-models` gives you a quick overview of alternatives without leaving the terminal.

---

## Related commands

- `todozi list tasks` – uses the current embedding model for semantic search.
- `todozi export-embeddings` – writes the stored vectors to an HLX file; the model used for generation is the one set via `emb`.
- `todozi migrate` – may need to re‑generate embeddings if you change the model after migration.

For a complete overview of all top‑level commands, see `docs/cmd.md`.
