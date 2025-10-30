# TDZ Content Processor (tdz_cnt)

A powerful tool for processing raw JSON content from AI model responses, extracting structured Todozi data, and returning clean conversational content.

## Overview

The `tdz_cnt` function is designed to be the ultimate helper for AI-human collaboration by:

1. **Processing Raw Content**: Handles JSON responses from AI models containing dialogue, tool calls, and Todozi tags
2. **Intelligent Extraction**: Automatically extracts tasks, memories, ideas, and checklist items from natural language
3. **Dual Processing**: Supports both `<todozi>` tag format and JSON tool call format
4. **Smart Execution**: Chooses between binary calls (preferred for performance) and library calls
5. **Content Cleaning**: Returns conversation-ready content with all Todozi elements removed
6. **Persistent Storage**: Saves both raw and cleaned content to `$HOME/.todozi/wash/cleaned.json`
7. **Auto-Organization**: Creates checklists, sessions, and tracks actions automatically

## Quick Start

```rust
use todozi::tdz_cnt;

// Process raw model response
let result = tdz_cnt(r#"{
    "content": "I think we should <todozi>add user authentication; high priority</todozi> and don't forget to add error handling",
    "tool_calls": [...]
}"#, Some("session_123")).await?;

println!("{}", result); // Cleaned content + status summary
```

## Features

### Natural Language Processing
Automatically extracts checklist items from phrases like:
- "add to checklist"
- "we need to"
- "should have"
- "don't forget to"
- "make sure to"

### Session Management
- Auto-creates conversation sessions
- Tracks participant count and message history
- Infers topics from content (bug fixing, feature development, etc.)

### Action Tracking
- Records all processed actions
- Maintains history of recent activity
- Provides status summaries in responses

## File Structure

```
tdz_cnt/
├── README.md              # This file
├── examples/              # Usage examples
│   ├── basic_usage.rs
│   ├── json_processing.rs
│   └── natural_language.rs
├── tests/                 # Test cases
│   ├── checklist_tests.rs
│   └── processing_tests.rs
└── docs/                  # Documentation
    ├── api_reference.md
    └── integration_guide.md
```

## Storage

All processed content is saved to:
- **Raw & Cleaned Content**: `$HOME/.todozi/wash/cleaned.json`
- **Binary**: Uses the `todozi` binary for optimal performance
- **History**: Maintains rolling history (last 1000 entries)

## Response Format

Each `tdz_cnt` response includes:

1. **Cleaned Content**: Original text with Todozi elements removed
2. **Processing Summary**: Count of processed actions
3. **Recent Actions**: Last 5 processed actions
4. **Active Checklist**: Current incomplete checklist items
5. **Active Sessions**: Current conversation sessions
6. **Reminder**: How to view all activity with `todozi stats`

## Integration

The tool integrates seamlessly with:
- **AI Models**: Process responses from Claude, GPT, etc.
- **Existing Todozi**: All standard Todozi operations supported
- **Tool Calling**: Handles both JSON and tag-based tool calls
- **Session Continuity**: Maintains context across interactions
