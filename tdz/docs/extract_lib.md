# `extract.rs` – Structured Content Extraction

## Purpose
Parses outgoing/incoming chat messages to extract Todozi‑specific markup (`<todozi>`, `<memory>`, `<idea>`, `<chunk>`). The extracted data is fed into the storage, embedding, and agent subsystems.

## Core Functions
| Function | Description |
|----------|-------------|
| `process_chat_message_extended` | Scans a chat string for all supported markup tags and returns a struct containing vectors of tasks, memories, ideas, and chunks. |
| `extract_tagged_content` | Low‑level helper that uses regexes to pull out a specific tag block. |

## Integration
* Called by the CLI (`cli.rs`) when the user runs `todozi chat process …`.
* Used by the server endpoint `/chat/process` to turn AI‑generated text into concrete Todozi objects.

---

*Documentation generated for the `extract.rs` module.*
