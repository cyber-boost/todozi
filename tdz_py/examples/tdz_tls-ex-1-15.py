import asyncio
import json
from typing import Any, Dict

# Import the public API from the tdz_tls module
from tdz_tls import (
    TdzContentProcessorTool,
    TodoziProcessorState,
    parse_chat_message_extended,
)


async def run_example() -> None:
    # 1) Initialize shared state used by the content processor
    state = TodoziProcessorState.new()

    # 2) Create the content processor bound to the state
    processor = TdzContentProcessorTool(state)

    # 3) Example content with tags, a memory, and natural language lines
    tagged_content = """
Hello team! Let's tackle the following:

<todozi>add task; prepare release notes for v1.0</todozi>

<memory>last sprint retro; learnings captured; we should improve code review time; high; long</memory>

We need to fix the bug, don't forget to test it, and make sure to deploy.
"""

    # 4) Process content with checklist extraction and automatic session management
    result = await processor.execute(
        {
            "content": tagged_content,
            "extract_checklist": True,
            "auto_session": True,
            "session_id": "demo-session-1",
        }
    )

    # 5) Print the processor response (summary + state)
    print("--- Processor Response ---")
    print(result.output)

    # 6) Also demonstrate parsing a JSON-style "chat model" output
    json_content = {
        "choices": [
            {
                "message": "We should schedule a follow-up meeting to review actions.\n"
                            "Remember to add checklist item: update roadmap."
            }
        ],
        "tool_calls": [
            {
                "function": {
                    "name": "create_task",
                    "arguments": json.dumps(
                        {
                            "action": "prepare release notes for v1.0",
                            "priority": "medium",
                            "parent_project": "release",
                            "time": "1 hour",
                            "context_notes": "Draft release notes for v1.0",
                        }
                    ),
                }
            },
            {
                "function": {
                    "name": "search_tasks",
                    "arguments": json.dumps({"query": "release"}),
                }
            },
        ],
    }

    print("\n--- JSON Content Processing via tdz_cnt ---")
    # tdz_cnt will parse JSON, extract tasks/memories/ideas, and run traditional processing as well
    tdz_response = await tdz_cnt(json.dumps(json_content), session_id="demo-session-2")
    parsed = json.loads(tdz_response)
    print(json.dumps(parsed, indent=2))

    # 7) Show checklist items detected from raw text (natural language)
    print("\n--- Checklist Items Detected from Natural Language ---")
    checklist = processor.extract_checklist_items(
        "We need to fix the bug, don't forget to test it, and make sure to deploy, "
        "remember to add to checklist: update roadmap."
    )
    for item in checklist:
        print(f"- {item.content} (source: {item.source})")

    # 8) Demonstrate parse_chat_message_extended utility
    print("\n--- Direct Parsing of Chat Message with Tags ---")
    parsed_chat = parse_chat_message_extended(
        "Next steps:\n"
        "<todozi>update roadmap; 30 minutes; low; planning</todozi>\n"
        "<idea>automate release notes; share; high</idea>\n"
        "<error>connection error; Failed to connect to DB; high; runtime; api</error>",
        "demo",
    )
    print(f"Tasks: {len(parsed_chat.tasks)}")
    for t in parsed_chat.tasks:
        print(f" - {t.action} (priority: {t.priority}, project: {t.parent_project})")
    print(f"Memories: {len(parsed_chat.memories)}")
    print(f"Ideas: {len(parsed_chat.ideas)}")
    print(f"Errors: {len(parsed_chat.errors)}")
    for e in parsed_chat.errors:
        print(f" - {e.title}: {e.detail}")

    print("\n--- Shared State Snapshot (last 3 actions) ---")
    recent_actions = list(state.recent_actions)[-3:]
    for a in recent_actions:
        print(f"{'✅' if a.success else '❌'} {a.action_type}: {a.description}")

    # 9) Optional: Show active sessions tracked in state
    print("\n--- Active Sessions (last 24h) ---")
    from datetime import datetime, timezone, timedelta
    now = datetime.now(timezone.utc)
    for sid, sess in state.active_sessions.items():
        if now - sess.last_activity < timedelta(hours=24):
            print(f"{sess.id}: topic='{sess.topic}', messages={sess.message_count}, last_activity={sess.last_activity}")


async def tdz_cnt(content: str, session_id: str | None = None) -> str:
    """
    High-level content processing function from tdz_tls.
    Accepts either a raw string or a JSON string representing "chat model" output.
    """
    from tdz_tls import tdz_cnt as _tdz_cnt
    return await _tdz_cnt(content, session_id)


if __name__ == "__main__":
    try:
        asyncio.run(run_example())
    except KeyboardInterrupt:
        print("\nInterrupted.")
    except FileNotFoundError as e:
        # If the external 'todozi' CLI is not installed, the tool will fail on tool calls.
        # Show a helpful message and continue with parsing/extract parts.
        print(f"Note: External 'todozi' CLI not found ({e}). "
              f"Tool-call actions will be skipped, but parsing and extraction still work.")
    except Exception as e:
        print(f"Error: {e}")