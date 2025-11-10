# Todozi CLI Technical Documentation

## Overview

The Todozi CLI is a comprehensive command-line interface for the Todozi AI/Human task management system. It provides access to task management, AI agents, memory systems, project tracking, and server operations through a unified command-line interface.

## Architecture

### Module Structure
```
todozi/
├── cli.py (this file)          # Main CLI entry point
├── storage.py                  # Data persistence layer
├── models.py                   # Core data structures
├── agent.py                    # AI agent management
├── memory.py                   # Memory system
├── idea.py                     # Idea management
├── error.py                    # Error handling
├── search.py                   # Search functionality
├── api.py                      # API key management
├── server.py                   # Web server
├── extract.py                  # Content extraction
├── emb.py                      # Embedding services
└── tui.py                      # Text-based user interface
```

### Core Components

#### 1. **CLI Infrastructure**
- **CommandRegistry**: Dynamic command registration and dispatch
- **CommandContext**: Execution context with storage and handler references
- **HandlerFunc**: Async command handler function signature

#### 2. **TodoziHandler**
Main command processor with specialized handlers for different command categories.

#### 3. **Storage Integration**
Seamless integration with the Todozi storage system for data persistence.

## Core Classes and Methods

### TodoziHandler Class

#### Constructor
```python
def __init__(self, storage: Storage)
```
**Parameters:**
- `storage: Storage` - Pre-initialized storage instance

**Description:**
Initializes the CLI handler with a storage backend for data operations.

### Command Handlers

#### 1. Basic Task Operations

**Complete Task**
```python
def complete_task(self, id: str) -> None
```
**Parameters:**
- `id: str` - Task identifier

**Usage:**
```python
handler.complete_task("task_12345")
```

**Delete Task**
```python
def delete_task(self, id: str) -> None
```
**Parameters:**
- `id: str` - Task identifier

#### 2. API Key Management

**Register New API Key**
```python
async def handle_api_command(self, command: ApiCommands) -> None
```
**Supported Commands:**
- `Register`: Create new API key with optional user ID
- `ListKeys`: List all API keys with filtering options
- `CheckKeys`: Validate API key authentication
- `DeactivateKey`: Disable API key
- `ActivateKey`: Enable API key
- `RemoveKey`: Permanently delete API key

**Example:**
```python
await handler.handle_api_command(Register(user_id="user_123"))
```

#### 3. Queue Management

**Queue Operations**
```python
async def handle_queue_command(self, command: QueueCommands) -> None
```
**Supported Commands:**
- `PlanQueue`: Add item to work queue
- `ListQueue`: List queue items with status filtering
- `BacklogQueue`: Show backlog items only
- `ActiveQueue`: Show active items only
- `CompleteQueue`: Show completed items only
- `StartQueue`: Begin queue session
- `EndQueue`: End queue session

**Example:**
```python
await handler.handle_queue_command(
    PlanQueue(
        task_name="Code review",
        task_description="Review PR #123",
        priority="high",
        project_id="engineering"
    )
)
```

#### 4. Server Management

**Server Operations**
```python
async def handle_server_command(self, command: ServerCommands) -> None
```
**Supported Commands:**
- `StartServer`: Launch Todozi web server
- `ServerStatus`: Check server health and status
- `ServerEndpoints`: Display API endpoint documentation

**Example:**
```python
await handler.handle_server_command(
    StartServer(host="0.0.0.0", port=8636)
)
```

#### 5. Search Operations

**Unified Search**
```python
async def handle_search_all_command(self, command: Commands) -> None
```
**Parameters:**
- `command: SearchAll` - Search configuration

**Search Types:**
- `tasks`: Task content and metadata
- `memories`: Memory system entries
- `ideas`: Idea repository
- `errors`: Error tracking system
- `training`: Training data

**Example:**
```python
await handler.handle_search_all_command(
    SearchAll(query="documentation", types="tasks,ideas")
)
```

#### 6. Chat Processing

**Enhanced Chat Processing**
```python
async def handle_chat_command(self, command: Commands) -> None
def process_chat_message_extended(self, message: str, user_id: str) -> ChatContent
```

**Supported Tags:**
```xml
<todozi>action|time|priority|project|status</todozi>
<memory>moment|meaning|reason|importance|term</memory>
<memory_secret>moment|meaning|reason|importance|term</memory_secret>
<idea>idea|share|importance</idea>
<error>title|description|severity|category</error>
<train>prompt|completion|data_type</train>
```

#### 7. Error Management

**Error Tracking**
```python
async def handle_error_command(self, command: Commands) -> None
```
**Supported Operations:**
- `CreateError`: Record new error
- `ListErrors`: Filter and display errors
- `ShowError`: Show error details
- `ResolveError`: Mark error as resolved
- `DeleteError`: Remove error record

#### 8. Training Data Management

**Training System**
```python
async def handle_train_command(self, command: TrainingCommands) -> None
```
**Data Types:**
- `instruction`: Instruction-following data
- `conversation`: Dialogue training data
- `completion`: Text completion examples

#### 9. Agent Management

**AI Agent Operations**
```python
async def handle_agent_command(self, command: Commands) -> None
```
**Agent Categories:**
- `CreateAgent`: Initialize new AI agent
- `ListAgents`: Display available agents
- `ShowAgent`: Show agent configuration
- `AssignAgent`: Assign task to agent
- `UpdateAgent`: Modify agent settings
- `DeleteAgent`: Remove agent

#### 10. Embedding Services

**Vector Embedding Operations**
```python
async def handle_emb_command(self, command: Commands) -> None
```
**Supported Models:**
- `sentence-transformers/all-MiniLM-L6-v2` (384 dimensions)
- `sentence-transformers/all-mpnet-base-v2` (768 dimensions)
- `sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2` (multilingual)

## Utility Functions

### Date/Time Utilities
```python
def _now_str() -> str
```
Returns current UTC timestamp in "YYYY-MM-DD HH:MM:SS" format.

### ID Generation
```python
def _id(prefix: str) -> str
```
Generates unique identifiers with specified prefix.

### Tag Parsing
```python
def _parse_tags(s: str) -> List[str]
```
Parses comma-separated tag strings into list format.

### Port Checking
```python
def _is_port_open(host: str, port: int) -> bool
```
Validates network port availability.

## Command Line Interface

### Argument Parsing System

**Command Registry Pattern:**
```python
registry = CommandRegistry()
registry.register("add", build_add_parser(), wrap_add)
```

**Supported Commands:**
```
todozi init                    # Initialize Todozi directory
todozi add <action> <time> <priority> <project> [options]
todozi list [--project] [--status] [--priority] [--search]
todozi show <id>               # Show task details
todozi update <id> [options]   # Update task properties
todozi complete <id>           # Mark task as complete
todozi delete <id>             # Delete task
todozi search <query>          # Search tasks
todozi stats                   # System statistics
todozi project <subcommand>    # Project management
```

### Subcommand Structure

**Project Management:**
```bash
todozi project list            # List all projects
todozi project create <name>   # Create new project
todozi project show <name>     # Show project details
```

## Data Models

### Core Data Structures

**Task Model:**
```python
@dataclass
class Task:
    id: str
    user_id: str
    action: str
    time: str
    priority: Priority
    parent_project: str
    status: Status
    assignee: Optional[Assignee]
    tags: List[str]
    dependencies: List[str]
    context_notes: Optional[str]
    progress: Optional[int]
```

**Memory Model:**
```python
@dataclass
class Memory:
    user_id: str
    moment: str
    meaning: str
    reason: str
    importance: MemoryImportance
    term: MemoryTerm
    memory_type: MemoryType
```

## Error Handling

### Exception Hierarchy
- `TodoziError`: Base exception for all Todozi errors
- `TaskNotFoundError`: Specific error for missing tasks
- `ValidationError`: Input validation failures

### Error Recovery
- Automatic backup restoration capabilities
- Data consistency validation and repair
- Graceful degradation for partial failures

## Performance Considerations

### Memory Management
- Lazy loading of large datasets
- Efficient search indexing with incremental updates
- Streaming processing for large content extraction

### Async Operations
- All I/O operations use async/await pattern
- Non-blocking command execution
- Concurrent task processing where applicable

## Dependencies and Requirements

### Core Dependencies
```python
# System
import sys
import os
import asyncio
import argparse
from pathlib import Path

# Data Processing
import json
import re
from datetime import datetime, timezone
from dataclasses import dataclass, field

# Networking
import socket
import warnings
```

### External Dependencies
- `sentence-transformers`: For embedding services
- `aiohttp`: For async HTTP operations (server)
- `uvicorn`: For ASGI server implementation

## Configuration

### Storage Configuration
```python
def get_storage_dir() -> Path
```
Returns platform-appropriate storage directory.

### Warning Suppression
```python
# Suppress specific warnings for cleaner output
warnings.filterwarnings("ignore", category=UserWarning, module="urllib3")
```

## Security Considerations

### API Key Security
- Private keys stored securely
- Public/private key authentication system
- Key rotation and revocation support

### Data Protection
- Memory type segregation (standard/secret/human)
- Access control through API key permissions
- Secure storage of sensitive data

## Limitations and Constraints

### Technical Limitations
- Maximum task action length: 500 characters
- Maximum concurrent connections: System-dependent
- File size limits for content extraction: System memory dependent

### Platform Constraints
- Requires Python 3.8+
- Unix-style path conventions preferred
- Async I/O requires compatible event loop

## Example Usage Patterns

### Basic Task Management
```python
storage = await Storage.new()
handler = TodoziHandler(storage)

# Create task
await handler.handle_add_command(AddTask(
    action="Write documentation",
    time="2 hours",
    priority="high",
    project="engineering"
))

# Complete task
handler.complete_task("task_abc123")
```

### Advanced AI Integration
```python
# Create AI agent
await handler.handle_agent_command(CreateAgent(
    id="doc_writer",
    name="Documentation Writer",
    description="AI agent for documentation tasks",
    model_provider="openai",
    model_name="gpt-4"
))

# Process chat with AI extraction
await handler.handle_chat_command(Chat(
    message="<todozi>Write API docs|4 hours|high|engineering</todozi>"
))
```

## Troubleshooting

### Common Issues

**Import Path Problems:**
```python
# Ensure parent directory is in Python path
_parent_dir = _cli_file.parent.parent
if str(_parent_dir) not in sys.path:
    sys.path.insert(0, str(_parent_dir))
```

**Storage Initialization:**
```python
# Proper async initialization
storage = await Storage.new()
```

**Command Parsing Errors:**
- Use `--help` flag for command-specific usage
- Validate enum values (priority, status, etc.)
- Check required parameter presence

This documentation provides comprehensive coverage of the Todozi CLI system architecture, capabilities, and usage patterns for developers and technical stakeholders.