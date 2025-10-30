# `server.rs` – Enhanced TCP Server

## Purpose
`server.rs` implements the **TCP‑based HTTP‑like server** that exposes Todozi’s functionality over the network.
It runs on port **8636** by default (configurable) and provides a set of REST‑style endpoints for tasks, embeddings, agents, memories, ideas, and more.

## Main Components

| Component | Description |
|-----------|-------------|
| `ServerConfig` | Holds host, port, TLS options, thread‑pool size, and rate‑limit settings. |
| `start_server(config: Option<ServerConfig>) -> Result<()>` | Boots the async Tokio server, registers request handlers, and starts listening. |
| `handle_request(req: Request) -> Response` | Dispatches an incoming request to the appropriate module (`tasks`, `embeddings`, `agents`, etc.). |
| `shutdown_signal()` | Gracefully stops the server on SIGINT/SIGTERM. |

## Endpoints (selected)

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Simple health‑check; returns `{ "status": "ok", "uptime_seconds": … }`. |
| `GET` | `/stats` | System statistics (task counts, agent activity, embedding cache size). |
| `GET` | `/tasks` | List all tasks (supports query parameters `project`, `status`, `priority`). |
| `POST` | `/tasks` | Create a new task – body contains a JSON representation of `Task`. |
| `GET` | `/tasks/{id}` | Retrieve a single task by its UUID. |
| `PUT` | `/tasks/{id}` | Update an existing task (partial fields allowed). |
| `DELETE` | `/tasks/{id}` | Delete a task. |
| `GET` | `/tasks/search?q={query}` | Keyword or semantic search (controlled by server config). |
| `GET` | `/agents` | List all registered agents. |
| `POST` | `/agents` | Create a new agent; body contains an `Agent` JSON. |
| `GET` | `/agents/{id}` | Get details of a specific agent. |
| `PUT` | `/agents/{id}` | Update an agent’s configuration. |
| `DELETE` | `/agents/{id}` | Remove an agent. |
| `GET` | `/memories` | List all memories (supports `type=secret|human|emotional`). |
| `POST` | `/memories` | Create a new memory entry. |
| `GET` | `/ideas` | List all ideas. |
| `POST` | `/ideas` | Create a new idea. |
| `GET` | `/chunks` | List all code chunks. |
| `POST` | `/chunks` | Create a new code chunk. |
| `GET` | `/chunks/ready` | Get chunks whose dependencies are satisfied and ready for generation. |
| `GET` | `/chunks/graph` | Return a JSON representation of the `CodeGenerationGraph`. |
| `POST` | `/chat/process` | Process a chat message, extract Todozi markup, and persist resulting objects. |
| `POST` | `/chat/agent/{id}` | Send a message to a specific agent and receive its response. |
| `GET` | `/analytics/tasks` | Task‑level analytics (completion rates, overdue tasks, etc.). |
| `GET` | `/analytics/agents` | Agent usage statistics. |
| `GET` | `/analytics/performance` | Server performance metrics (request latency, cache hit rate). |
| `POST` | `/time/start/{task_id}` | Start a time‑tracking session for a task. |
| `POST` | `/time/stop/{task_id}` | Stop a time‑tracking session. |
| `GET` | `/time/report` | Summarise time spent on tasks. |
| `GET` | `/projects` | List all projects. |
| `POST` | `/projects` | Create a new project. |
| `GET` | `/projects/{name}` | Get a project by name. |
| `PUT` | `/projects/{name}` | Update a project. |
| `DELETE` | `/projects/{name}` | Delete (or archive) a project. |

## TLS Support
If the `server` feature is built with TLS enabled, `tdz_tls.rs` supplies:

* `load_certificates(cert_path, key_path) -> Result<TlsAcceptor>`
* `configure_tls(config: &mut ServerConfig)` – attaches the TLS acceptor to the listener.
* Default certificate locations are `~/.todozi/tls/cert.pem` and `~/.todozi/tls/key.pem`.

## Typical Startup Flow (Mermaid)

```mermaid
flowchart TD
    A[Parse CLI args] --> B[Read ServerConfig (or defaults)]
    B --> C[load_certificates (optional TLS)]
    C --> D[start_server]
    D --> E[Accept incoming connections]
    E --> F[handle_request]
    F --> G[Dispatch to module handler]
    G --> H[Return JSON response]
    D --> I[Graceful shutdown on signal]
```

## Integration Points

| Module | Interaction |
|--------|-------------|
| `src/cli.rs` | `todozi server start` invokes the same `start_server` function. |
| `src/storage.rs` | All CRUD endpoints eventually call `Storage` methods to read/write JSON/HLX files. |
| `src/emb.rs` | Semantic search endpoints (`/tasks/search`, `/memories/search`) use `TodoziEmbeddingService`. |
| `src/agent.rs` | Agent‑related endpoints (`/agents/*`) forward to the `AgentManager`. |
| `src/tui.rs` | The optional terminal UI can also act as a client to these endpoints when run in “remote” mode. |
| `src/error.rs` | Errors from handlers are converted to HTTP status codes (e.g., 404 for “not found”, 400 for validation errors). |

## Example: Starting the Server

```sh
# Default host/port (127.0.0.1:8636)
todozi server start

# Custom host & port
todozi server start --host 0.0.0.0 --port 9000

# Enable TLS (requires cert/key files)
todozi server start --tls
```

The server logs startup details, available endpoints, and health‑check URL. Once running, you can interact with it via `curl`, Postman, or any HTTP client:

```sh
# List tasks
curl http://127.0.0.1:8636/tasks

# Create a new task (JSON body)
curl -X POST http://127.0.0.1:8636/tasks \
     -H "Content-Type: application/json" \
     -d '{"action":"Implement login","priority":"high","project":"auth"}'

# Search tasks semantically (requires embeddings)
curl http://127.0.0.1:8636/tasks/search?q=authentication
```

## Testing & Validation

* **Unit tests** (`src/tests.rs`) spin up the server on a random port, issue HTTP requests, and verify responses.
* **Integration tests** (`cargo test --features server`) exercise the full request‑handler pipeline.
* **Load testing** – the server can be benchmarked with `hey` or `wrk` to verify that the async Tokio runtime scales with concurrent connections.

## Future Enhancements

* **WebSocket support** – for real‑time push notifications (e.g., task status updates).
* **Rate limiting & API quotas** – per‑API‑key limits configurable in `ServerConfig`.
* **OpenAPI/Swagger generation** – automatic docs for all endpoints.
* **Clustered deployment** – multiple server instances behind a load balancer, sharing a common storage backend.

---

*Generated by GPT‑OSS – documentation for the `server.rs` module.*
