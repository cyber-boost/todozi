# `todozi server`

Control the Todozi enhanced server, which exposes a REST‑like API for task, memory, idea, agent, and other operations. The server runs on TCP port **8636** by default (configurable) and provides a set of endpoints documented in the `endpoints` sub‑command.

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **start** `--host <HOST>` `--port <PORT>` | Launch the server. By default it binds to `127.0.0.1:8636`. |
| **status** | Check whether a Todozi server instance is running on any of the common ports (8636, 8637, 3000) and report its health. |
| **endpoints** | Print a concise list of all available HTTP endpoints, their methods, and a short description. |

## Usage examples

```sh
# Start the server on the default host/port
todozi server start

# Start the server on a custom address
todozi server start --host 0.0.0.0 --port 9000

# Check if the server is alive
todozi server status

# Show the API reference
todozi server endpoints
```

## What happens under the hood

1. **Configuration** – The command reads the server configuration from `~/.todozi/tdz.hlx` (host, port, TLS settings, rate limits). If a configuration entry is missing, sensible defaults are applied.
2. **Async runtime** – A Tokio runtime is created, and the `start_server` async function (found in `src/server.rs`) is invoked.
3. **Listener** – The server binds a TCP listener to the requested address and spawns a set of worker tasks to handle incoming connections concurrently.
4. **Routing** – Requests are dispatched through a router that maps URL paths (`/tasks`, `/agents`, `/memories`, etc.) to handler functions in the `src/api.rs` module.
5. **Authentication** – If the client supplies an API key (via the `Authorization: Bearer <key>` header), the request is validated against the stored API keys before any state‑changing operation is allowed.
6. **Graceful shutdown** – On Ctrl‑C or a termination signal, the server stops accepting new connections, finishes ongoing requests, and cleanly shuts down the runtime.

## Health‑check endpoint

When the server is running, `GET /health` returns a JSON payload:

```json
{
  "status": "ok",
  "uptime_seconds": 12345,
  "tasks_total": 278,
  "agents_active": 5
}
```

The `status` sub‑command essentially performs a short TCP connection test to this endpoint.

## Security notes

- The server does **not** enforce TLS by default; it is recommended to run it behind a reverse proxy (e.g., Nginx) or enable TLS in the future via a configuration flag.
- All write operations (`POST`, `PUT`, `DELETE`) require a valid API key with appropriate permissions. The `api` command can be used to generate, list, activate, deactivate, or remove API keys.

## Related commands

- **`todozi api`** – Manage API keys that the server expects.
- **`todozi register`** – Register the client with the central Todozi service (optional for server usage).
- **`todozi backup`** – Create a backup before performing large migrations that the server may later need to restore.
- **`todozi migrate`** – After a migration, you may need to restart the server to pick up new storage layouts.

## Further reading

- The source code for the server lives in `src/server.rs`.
- Detailed endpoint documentation is printed by `todozi server endpoints` and can also be inspected in the `README` under the *Server API* section.

---

*Documentation generated for the `server` command as part of the Todozi CLI.*
