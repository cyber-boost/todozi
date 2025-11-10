# Todozi CLI Technical Documentation

## Overview

Todozi is a comprehensive task management and AI-assisted productivity system translated from Rust to Python. This CLI application provides a complete interface for managing tasks, projects, agents, training data, and various AI-powered features through a structured command-line interface.

## Architecture Design

### Core Architecture Principles

The system follows a modular architecture with clear separation of concerns:

1. **Domain Models**: Data classes representing core entities (Task, Project, Agent, etc.)
2. **Command Parser**: argparse-based command structure mirroring the Rust implementation
3. **Command Handlers**: Business logic implementations for each command
4. **Search Engine**: In-memory search capabilities across all data types
5. **Storage Layer**: Abstracted data persistence (implied by imports)

### Module Structure

```
todozi/
├── cli.py              # Main CLI entry point and parser
├── models.py           # Domain model definitions
├── storage.py          # Data persistence layer
├── error.py           # Custom exception types
├── emb.py             # Embedding services
├── tui.py             # Text User Interface
└── __init__.py
```

## Domain Models

### Core Data Structures

#### Task Management
```python
@dataclass
class Task:
    id: str = ""
    action: str = ""
    time: str = ""
    priority: str = ""
    project: str = ""
    status: str = "todo"
    assignee: Optional[str] = None
    tags: Optional[str] = None
    dependencies: Optional[str] = None
    context: Optional[str] = None
    progress: Optional[int] = None
```

#### Project Management
```python
@dataclass
class Project:
    name: str
    description: Optional[str] = None
    created_at: datetime
    updated_at: datetime
```

#### AI Agent System
```python
@dataclass
class Agent:
    id: str
    name: str
    description: str
    category: str
    model_provider: str = "openai"
    model_name: str = "gpt-4o-mini"
    temperature: float = 0.7
    max_tokens: int = 1024
    # ... additional configuration fields
```

### Enumeration Types

The system uses Python Enums to mirror Rust enum functionality:

```python
class Commands(Enum):
    INIT = "init"
    ADD = "add"
    LIST = "list"
    SHOW = "show"
    UPDATE = "update"
    # ... 40+ additional commands
```

## Command Parser System

### Parser Architecture

The argparse parser is built hierarchically with three levels:

1. **Top-level commands** (`dest="command"`)
2. **Sub-commands** (e.g., `dest="add_sub"`, `dest="project_sub"`)
3. **Command-specific arguments**

### Argument Groups

The parser uses mutually exclusive groups for certain operations:

```python
# For extract/strategy commands
extract_group = extract.add_mutually_exclusive_group(required=True)
extract_group.add_argument("content", nargs="?", help="Inline text content")
extract_group.add_argument("--file", "-f", help="File path")
```

### Boolean Flag Handling

Boolean flags use `action="store_true"` for proper argparse behavior:

```python
parser.add_argument("--auto-format-code", action="store_true", help="Auto format code")
parser.add_argument("--include-examples", action="store_true", help="Include examples")
```

## Search Engine Implementation

### SearchEngine Class

```python
class SearchEngine:
    """
    A simple in-memory search engine across tasks, memories, ideas, errors, and training data.
    """
    
    def __init__(self) -> None:
        self.tasks: List[Task] = []
        self.memories: List[Memory] = []
        # ... other data type collections

    def update_index(self, content: ChatContent) -> None:
        """Merge a ChatContent payload into the search index."""
        
    def search(self, query: str, options: SearchOptions) -> SearchResults:
        """Perform keyword-based search across all indexed content types."""
```

### Search Algorithm

The search uses case-insensitive keyword matching across multiple fields:

1. **Primary text fields** (task action, memory moment, etc.)
2. **Secondary text fields** (descriptions, context)
3. **Tag matching** (comma-separated tag lists)

### Search Options

```python
@dataclass
class SearchOptions:
    limit: Optional[int] = None
    data_types: Optional[str] = None
    since: Optional[str] = None
    until: Optional[str] = None
```

## Command Handlers

### Handler Pattern

Each command follows a consistent handler pattern:

```python
def handle_command(ns: argparse.Namespace) -> None:
    # 1. Validate inputs
    # 2. Call storage layer
    # 3. Handle errors
    # 4. Output results
```

### Error Handling

Handlers implement robust error handling:

```python
def handle_show(ns: argparse.Namespace) -> None:
    try:
        task = storage.get_task_from_any_project(ns.id)
        # Output task details
    except TaskNotFoundError as e:
        print(f"Error: Task not found: {ns.id}", file=sys.stderr)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
```

### Async Operations

Certain operations use asyncio for async storage operations:

```python
async def show_model():
    config = TodoziEmbeddingConfig()
    service = TodoziEmbeddingService(config)
    await service.initialize()
    # ... output model info
```

## Storage Layer Integration

### Implied Storage Interface

The handlers reference a storage layer with the following interface:

```python
class Storage:
    def new() -> Storage
    async def add_task_to_project(task: Task) -> None
    def list_tasks_across_projects(filters: TaskFilters) -> List[Task]
    def get_task_from_any_project(task_id: str) -> Task
    async def update_task_in_project(task_id: str, updates: TaskUpdate) -> None
    # ... additional methods for projects, agents, training data
```

### Task Filtering System

```python
@dataclass
class TaskFilters:
    project: Optional[str] = None
    status: Optional[Status] = None
    priority: Optional[Priority] = None
    assignee: Optional[Assignee] = None
    tags: Optional[List[str]] = None
    search: Optional[str] = None
```

## Feature Categories

### 1. Task Management
- **Add/List/Show/Update/Delete** tasks with full metadata
- **Project-based organization** with archiving capabilities
- **Progress tracking** with percentage-based completion
- **Dependency management** between tasks

### 2. AI Agent System
- **Agent creation and configuration** with model settings
- **Task assignment** to specific agents
- **Capability-based filtering** and specialization
- **Performance metrics** monitoring

### 3. Search and Analytics
- **Cross-type search** (tasks, memories, ideas, errors, training data)
- **Statistical reporting** with project and status breakdowns
- **Backup and restore** functionality
- **Data export** in multiple formats

### 4. ML and Embeddings
- **Embedding model management** with popular pre-trained models
- **Training data collection** and quality scoring
- **Model testing** and performance evaluation
- **Advanced processing** pipelines

### 5. Server and API
- **Local server** with REST API endpoints
- **API key management** with activation/deactivation
- **Health monitoring** and status reporting
- **Web interface** integration points

## Technical Constraints and Limitations

### Performance Considerations

1. **In-Memory Search**: The search engine loads all data into memory, limiting scalability
2. **Async Operations**: Mixed sync/async patterns may cause blocking in some scenarios
3. **Data Persistence**: Storage implementation details not specified in provided code

### Memory Usage

- Search indices maintain copies of all domain objects
- Large datasets may exceed memory limits
- No pagination or streaming for large result sets

### Error Handling Constraints

- Limited validation of input data types
- Basic exception handling without detailed error categorization
- No retry mechanisms for failed operations

## Dependencies and Requirements

### Core Dependencies
```python
# Required standard library modules
import argparse
import asyncio
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum
from typing import List, Optional, Dict, Any
```

### External Dependencies (Implied)
- **Sentence-transformers** for embedding models
- **Optional TUI framework** for text user interface
- **Network libraries** for server functionality
- **Storage backend** (SQLite, file-based, or database)

## Usage Patterns and Examples

### Basic Task Management
```bash
# Add a task
todozi add task "Write documentation" --time "2h" --priority high --project docs

# List tasks
todozi list tasks --project docs --status todo

# Update task progress
todozi update task-123 --progress 50

# Complete a task
todozi complete task-123
```

### Agent Operations
```bash
# Create an AI agent
todozi agent create doc-agent "Documentation Assistant" "Helps with technical writing" \
    --category writing --model-name gpt-4 --temperature 0.3

# Assign agent to task
todozi agent assign doc-agent task-123 docs
```

### Search Operations
```bash
# Search across all data types
todozi search-all "documentation" --types tasks,ideas

# Export search results
todozi train export --format json --min-quality 0.8
```

### Server Management
```bash
# Start local server
todozi server start --port 8636

# Check server status
todozi server status

# List API endpoints
todozi server endpoints
```

## Error Handling and Edge Cases

### Common Error Scenarios

1. **Task Not Found**: Handled with specific exception type
2. **Invalid Enum Values**: Conversion functions return Err type for validation
3. **Storage Errors**: Generic exception catching with user-friendly messages
4. **Network Issues**: Timeout handling for server operations

### Input Validation

- **Enum conversion**: String-to-enum conversion with error checking
- **Required fields**: argparse handles missing required arguments
- **Type validation**: Type converters for numeric and boolean values

## Extension Points

### Adding New Commands

1. **Define command enum** in appropriate Commands class
2. **Add parser configuration** in `build_parser()`
3. **Implement handler function** following established patterns
4. **Add dispatch logic** in main function

### Custom Storage Backends

The storage layer can be replaced by implementing the implied Storage interface with different persistence mechanisms.

### Plugin System

The modular architecture allows for additional feature modules to be added without modifying core functionality.

## Migration and Compatibility

The code maintains compatibility with the original Rust implementation through:

1. **Command structure mirroring** identical command names and options
2. **Data model equivalence** matching Rust struct definitions
3. **Behavioral consistency** similar output formats and error handling

This documentation provides a comprehensive technical reference for developers working with or extending the Todozi CLI system. The modular design and clear separation of concerns make the system maintainable and extensible for future enhancements.