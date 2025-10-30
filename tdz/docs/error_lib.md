# `error.rs` – Central Error Type

## Purpose
Defines `TodoziError`, the unified error enum used throughout the project, and the `Result<T>` alias.

## Key Types
| Type | Description |
|------|-------------|
| `TodoziError` | Enum with variants such as `IOError`, `ValidationError`, `EmbeddingError`, `AgentError`, etc. |
| `Result<T>` | Alias for `std::result::Result<T, TodoziError>`. |

## Usage
All fallible functions in Todozi return `Result<T>`. Callers typically use `?` to propagate errors, and the CLI/server prints human‑readable messages based on the variant.

---
