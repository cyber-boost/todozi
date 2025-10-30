# `todozi agent`

Handle the **agent system**, which provides AI‑powered assistants with distinct personalities, toolsets, and capabilities. Agents can be created, listed, inspected, updated, assigned to tasks, and deleted. They are stored under `~/.todozi/agents/` as JSON files and integrated with the embedding service for semantic matching.

---

## Sub‑commands

| Sub‑command | Description |
|-------------|-------------|
| **list** | Show a table of all registered agents with ID, name, category, model, and status. |
| **show** `--id <AGENT_ID>` | Display the full JSON representation of a specific agent, including its configuration and capabilities. |
| **create** `--id <ID>` `--name <NAME>` `--description <DESC>` `--category <CATEGORY>` `[options]` | Register a new agent. Options let you configure model provider, model name, temperature, token limits, tool list, and various behavioural flags. |
| **assign** `--agent-id <AGENT_ID>` `--task-id <TASK_ID>` `--project-id <PROJECT_ID>` | Assign an existing agent to a task, updating the task’s `assignee` field to reference the agent. |
| **update** `--id <AGENT_ID>` `[options]` | Modify any of the mutable fields of an existing agent (name, description, model settings, tools, etc.). |
| **delete** `--id <AGENT_ID>` | Permanently remove an agent from the system (asks for confirmation). |

---

## Usage examples

```sh
# List all agents
todozi agent list

# Show details for a specific agent
todozi agent show --id planner_01

# Create a new coding assistant
todozi agent create \
    --id coder_01 \
    --name "CodeGen" \
    --description "Generates Rust code snippets on demand" \
    --category coder \
    --model-provider anthropic \
    --model-name claude-3-5-sonnet-20240620 \
    --temperature 0.2 \
    --max-tokens 4096 \
    --tools "code_executor,linter,test_runner" \
    --auto-format-code true \
    --include-examples true \
    --explain-complexity false \
    --suggest-tests true

# Assign the new agent to a task
todozi agent assign \
    --agent-id coder_01 \
    --task-id 7f3e2a9c-1b4d-4e6a-9f7c-2d8b3a5e6f1a \
    --project-id backend

# Update the agent’s temperature and add a new tool
todozi agent update \
    --id coder_01 \
    --temperature 0.3 \
    --tools "code_executor,linter,test_runner,security_scanner"

# Delete an obsolete agent
todozi agent delete --id old_debugger
```

---

## How it works

1. **Storage** – Each agent is saved as `<id>.json` in `~/.todozi/agents/`. The file contains all configuration fields defined in `src/models.rs`, e.g.:

   ```json
   {
     "id": "coder_01",
     "name": "CodeGen",
     "description": "Generates Rust code snippets on demand",
     "category": "coder",
     "capabilities": ["code_generation", "linting", "testing"],
     "specializations": null,
     "model_provider": "anthropic",
     "model_name": "claude-3-5-sonnet-20240620",
     "temperature": 0.2,
     "max_tokens": 4096,
     "tags": ["rust","code"],
     "system_prompt": null,
     "prompt_template": null,
     "auto_format_code": true,
     "include_examples": true,
     "explain_complexity": false,
     "suggest_tests": true,
     "tools": ["code_executor","linter","test_runner"],
     "max_response_length": null,
     "timeout_seconds": null,
     "requests_per_minute": null,
     "tokens_per_hour": null,
     "created_at": "2025-10-27T14:45:00Z",
     "updated_at": "2025-10-27T14:45:00Z"
   }
   ```

2. **Embedding** – When an agent is created or updated, its description and capabilities are fed to `TodoziEmbeddingService`. The resulting vector is stored in `todozi_embeddings.hlx` under the `agent` namespace. This enables semantic lookup of agents (e.g., `todozi find agents --query "security scanner"`).

3. **Assignment** – The `assign` sub‑command updates the target task’s `assignee` field to `Assignee::Agent("<AGENT_ID>")`. The task is then stored back to disk, and the embedding cache is refreshed.

4. **Tool integration** – Agents declare a list of tools (e.g., `code_executor`, `linter`). The CLI passes this list to the underlying agent runtime, which loads the corresponding implementations from `src/agent.rs`.

5. **Safety** – Deleting an agent checks that no active tasks are currently assigned to it, warning the user before proceeding.

---

## When to use `agent`

- **Automated coding** – Deploy a coder agent to generate scaffolding or fix bugs.
- **Design assistance** – Use a designer agent for UI mock‑ups or style suggestions.
- **Testing** – Assign a tester agent to automatically generate test suites.
- **Planning** – A planner agent can break down high‑level goals into actionable tasks.
- **Specialised tooling** – Agents can be equipped with custom tools (e.g., security scanners, database migrations) to handle domain‑specific work.

---

## Related commands

- `todozi add task` – After an agent is assigned, the task can be processed automatically.
- `todozi emb` – Switch the embedding model to improve agent similarity matching.
- `todozi server` – The server exposes `/agents` endpoints for CRUD operations.
- `todozi backup` – Preserve agent definitions in backups.
- `todozi migrate` – Ensure agents survive structural migrations.

For a complete list of all top‑level commands, see **`docs/cmd.md`**.
