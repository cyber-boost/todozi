# `search.rs` – Unified Search Facade

## Purpose
`search.rs` provides a **single entry point** for both keyword‑based and AI‑semantic search across all Todozi content types (tasks, ideas, memories, chunks, etc.). It decides which backend to use (simple text search or embedding‑based semantic search) and returns a uniform `SearchResult` structure that the CLI, server, and agents can consume.

## Core Types

| Type | Category | Description |
|------|----------|-------------|
| `SearchResult` | Struct | Represents a single result with fields: `id`, `content_type`, `title` (or snippet), `summary`, `score` (for semantic results), `tags`, `metadata`. |
| `KeywordResult` | Alias | `SearchResult` when the result comes from a plain keyword match. |
| `SemanticResult` | Alias | `SearchResult` when the result comes from an embedding similarity lookup. |
| `SearchFilters` | Struct | Optional filters (tags, priority, status, assignee, date range, progress) that can be applied on top of any result set. |
| `SearchMode` | Enum | `Keyword`, `Semantic`, `Hybrid` – indicates which algorithm to run. |

## Public Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `search_keyword(query: &str, limit: usize) -> Result<Vec<KeywordResult>>` | `fn(&str, usize) -> Result<Vec<SearchResult>>` | Simple case‑insensitive substring search over task titles, contexts, tags, ideas, and memories. |
| `search_semantic(query: &str, types: &[TodoziContentType], limit: usize) -> Result<Vec<SemanticResult>>` | `fn(&str, &[TodoziContentType], usize) -> Result<Vec<SearchResult>>` | Generates an embedding for `query` and performs a cosine‑similarity search via `TodoziEmbeddingService`. |
| `search_hybrid(query: &str, keywords: Vec<String>, weight: f32, limit: usize) -> Result<Vec<SearchResult>>` | `fn(&str, Vec<String>, f32, usize) -> Result<Vec<SearchResult>>` | Combines keyword matches with semantic similarity; `weight` controls the contribution of the semantic component (0 = keyword‑only, 1 = semantic‑only). |
| `filter_search(results: Vec<SearchResult>, filters: SearchFilters) -> Vec<SearchResult>` | `fn(Vec<SearchResult>, SearchFilters) -> Vec<SearchResult>` | Applies tag, priority, status, assignee, date‑range, and progress filters to an existing result set. |
| `search_all(query: &str, mode: SearchMode, limit: usize) -> Result<Vec<SearchResult>>` | `fn(&str, SearchMode, usize) -> Result<Vec<SearchResult>>` | Convenience wrapper that dispatches to the appropriate backend based on `mode`. |

## Typical Flow (Mermaid)

```mermaid
flowchart TD
    Q[User Query] -->|Keyword| K[search_keyword]
    Q -->|Semantic| S[search_semantic]
    Q -->|Hybrid| H[search_hybrid]
    K --> R[Collect KeywordResult]
    S --> R
    H --> R
    R --> F[filter_search (optional)]
    F --> O[Return Vec<SearchResult>]
```

1. The CLI (`todozi search`) or the server receives a query string.
2. Depending on the requested mode (`--mode keyword|semantic|hybrid`) the corresponding function is called.
3. For semantic searches the `TodoziEmbeddingService` generates an embedding and looks up the nearest neighbours in the HLX cache.
4. Optional filters (e.g., `--tags security,backend`) are applied via `filter_search`.
5. Results are returned to the caller, which formats them for display or API response.

## Integration Points

| Module | Interaction |
|--------|-------------|
| `cli.rs` | Parses `todozi search` sub‑command, forwards the query and options to `search_all`. |
| `server.rs` | Exposes `/search` endpoint that accepts `q`, `mode`, and filter parameters, then calls `search_all`. |
| `emb.rs` | Provides the embedding generation and similarity index used by `search_semantic` and `search_hybrid`. |
| `storage.rs` | Supplies the raw text data for keyword search (task titles, contexts, memory texts, idea descriptions). |
| `agent.rs` | Agents can invoke `search_all` to retrieve relevant knowledge while planning or debugging. |

## Example Usage

### CLI

```sh
# Keyword search (fast, exact matches)
todozi search tasks "authentication"

# Semantic search (AI‑driven)
todozi search all --mode semantic "how to store passwords securely"

# Hybrid search with tag filter
todozi search all --mode hybrid --tags "security,backend" "password storage"
```

### Rust API

```rust
use todozi::search::{search_all, SearchMode};

#[tokio::main]
async fn main() -> todozi::error::Result<()> {
    // Find the most relevant tasks and ideas about OAuth
    let results = search_all(
        "OAuth token refresh strategy",
        SearchMode::Hybrid,
        10,
    ).await?;

    for r in results {
        println!(
            "[{}] {} (score: {:.2})",
            r.content_type, r.title, r.score.unwrap_or(0.0)
        );
    }

    Ok(())
}
```

## Performance Notes

* **Keyword search** runs in O(N) over the number of stored items and is virtually instantaneous for typical Todozi workloads (< 10 k items).
* **Semantic search** incurs the cost of embedding generation (+ ~10 ms per query) and a similarity lookup (vector distance computation). The embedding cache dramatically speeds up repeated queries.
* **Hybrid search** adds a small merging step; the overall latency is still dominated by the semantic component.

## Extending the Search System

1. **Add new content types** – Extend `TodoziContentType` (e.g., `Commit`, `PullRequest`) and ensure they are indexed in the embedding cache.
2. **Custom ranking** – Implement a `rank_results` function that re‑orders `SearchResult`s based on domain‑specific heuristics (e.g., recentness, task priority).
3. **Faceted filters** – Add more fields to `SearchFilters` (e.g., `project`, `author`) and update `filter_search` accordingly.

## Testing

`src/tests.rs` contains integration tests that:

* Populate a few tasks, ideas, and memories.
* Run `search_keyword` and assert that expected substrings appear.
* Run `search_semantic` after generating embeddings and verify that semantically related items are returned (e.g., “login” matches “authentication”).

## See Also

* `src/emb.rs` – Embedding service used by semantic search.
* `src/models.rs` – Data structures (`Task`, `Idea`, `Memory`) that populate the search index.
* `src/storage.rs` – Low‑level file access for keyword lookup.
* `docs/cmd.md` – Overview of the `todozi search` CLI command.

---

*Generated by GPT‑OSS – documentation for the `search.rs` module.*
