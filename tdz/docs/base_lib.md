# `base.rs` – Core Tool Definition & Utility Overview

## Purpose
`base.rs` defines the foundational building blocks for **Todozi’s extensible tool system**.
It provides:

* A **schema** for describing tools (parameters, categories, resource locks).
* A **runtime representation** (`ToolResult`) for execution outcomes.
* The **`Tool` trait** that all concrete tool implementations must adhere to.
* Helper constructors for common definitions.

These abstractions enable agents to discover, invoke, and manage a wide variety of capabilities (e.g., code execution, file manipulation, network calls) in a safe, declarative manner.

---

## Core Types

| Type | Kind | Description |
|------|------|-------------|
| `ToolParameter` | Struct | Describes a single input argument for a tool (name, type, description, required flag). |
| `ResourceLock` | Enum | Enumerates exclusive resources a tool may need (`FilesystemWrite`, `FilesystemRead`, `Memory`, `Network`). |
| `ToolDefinition` | Struct | Full metadata for a tool: name, description, parameters, category, and required `ResourceLock`s. |
| `ToolResult` | Struct | Standardised result of tool execution, containing success flag, result string, and optional cost. |
| `Tool` | Trait (`async_trait`) | The contract every tool must implement: `definition()` → `ToolDefinition` and async `execute(...)` → `ToolResult`. |

---

## Important Functions & Implementations

| Function | Signature | Summary |
|----------|-----------|---------|
| `ToolParameter::new` | `fn(name: String, type_: String, description: String, required: bool) -> Self` | Convenience constructor for a tool parameter. |
| `ToolDefinition::new` | `fn(name: String, description: String, parameters: Vec<ToolParameter>, category: String, resource_locks: Vec<ResourceLock>) -> Self` | Creates a full tool description. |
| `ToolResult::success` | `fn(result: String, cost: u32) -> Self` | Quick builder for successful executions. |
| `ToolResult::error` | `fn(error: String, cost: u32) -> Self` | Quick builder for failed executions. |
| `create_tool_parameter` | `fn(name: &str, type_: &str, description: &str, required: bool) -> ToolParameter` | Helper function (mirrors `ToolParameter::new`) for ergonomic usage throughout the codebase. |

---

## Typical Usage Flow

```mermaid
flowchart TD
    A[Define Tool Parameters] --> B[Create ToolDefinition]
    B --> C[Implement Tool Trait]
    C --> D[Register Tool in AgentManager]
    D --> E[Agent invokes Tool.execute()]
    E --> F[Receive ToolResult]
```

1. **Define parameters** using `create_tool_parameter` or `ToolParameter::new`.
2. **Build a `ToolDefinition`** with name, description, category, and required `ResourceLock`s.
3. **Implement `Tool`** for the concrete functionality, providing async `execute`.
4. **Register** the tool with the agent system (`AgentManager`).
5. **Agents** can now discover the tool’s metadata and safely invoke it, receiving a `ToolResult`.

---

## Integration Points

* **`agent.rs`** – Loads tool definitions, resolves resource locks, and dispatches execution.
* **`cli.rs`** – May expose certain tools as sub‑commands (e.g., a `code_executor` tool).
* **`server.rs`** – Allows remote clients to request tool execution via the API, respecting locks.

---

## Error Handling & Concurrency

* The **`ResourceLock`** enum allows the runtime to enforce exclusive access, preventing race conditions (e.g., two tools writing to the same file).
* `ToolResult` provides a uniform way to surface errors back to agents or callers, with an optional `cost` metric for billing or throttling.

---

## See Also

* **`agent.rs`** – Agent orchestration and tool registration.
* **`emb.rs`** – Example of a tool that utilizes embeddings for semantic search.
* **`error.rs`** – Central `TodoziError` used throughout the project.

---

