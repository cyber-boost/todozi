# Todozi Task Migration System - Technical Documentation

## Overview

The Todozi Task Migration System is a comprehensive Python-based framework designed to migrate task data from a legacy collection-based storage system to a modern project-based architecture. The system includes robust error handling, embedding generation capabilities, and a CLI interface for operational use.

## Architecture and Design Decisions

### Core Design Patterns
- **Result Pattern**: Generic `Result[T]` type for functional error handling
- **Repository Pattern**: Storage abstractions for task collections and project containers
- **Builder Pattern**: Fluent interface for configuring migrator and CLI instances
- **Async/Await**: Asynchronous embedding generation for performance

### System Architecture
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Legacy System │    │  Migration Engine │    │   New System    │
│   Collections   │───▶│   TaskMigrator    │───▶│ Project Contain │
│ (active, etc.)  │    │                   │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                      ┌───────┴───────┐
                      │ Embedding     │
                      │ Service       │
                      └───────────────┘
```

## Core Components

### 1. Error Handling System

#### `TodoziError` Class
Base exception class for all Todozi system errors.

**Properties:**
- `message: str` - Primary error description
- `details: Optional[str]` - Additional error context

**Static Methods:**
```python
@staticmethod
def storage(message: str) -> "TodoziError"
# Creates a storage-specific error instance

@staticmethod  
def config(message: str) -> "TodoziError"
# Creates a configuration-specific error instance
```

#### Specialized Error Classes
```python
class MigrationError(TodoziError):
    """Specialized error for migration operations"""

class StorageError(TodoziError):
    """Specialized error for storage operations"""
```

### 2. Result Type System

#### `Result(Generic[T])` Class
Generic container for operations that may succeed or fail.

**Methods:**
```python
def is_ok(self) -> bool
# Returns True if operation succeeded

def is_err(self) -> bool  
# Returns True if operation failed

def unwrap(self) -> T
# Returns value if ok, raises error if err

def expect(self, msg: str) -> T
# Returns value if ok, raises MigrationError with custom message if err
```

**Helper Functions:**
```python
def ok(value: T) -> Result[T]
# Creates successful result

def err(e: Union[TodoziError, str]) -> Result[T]
# Creates failed result
```

### 3. Data Models

#### `Task` Model
```python
@dataclass
class Task:
    id: str                    # Unique task identifier
    title: str = ""           # Task title
    description: str = ""     # Task description  
    status: str = "active"    # Task status
    parent_project: str = ""  # Associated project name
    embedding_vector: Optional[List[float]] = None  # Vector representation
```

#### `ProjectTaskContainer` Model
Container for managing tasks within a specific project.

**Methods:**
```python
@staticmethod
def new(project_name: str) -> "ProjectTaskContainer"
# Creates new container instance

def get_all_tasks(self) -> List[Task]
# Returns all tasks in container

def get_task(self, task_id: str) -> Optional[Task]  
# Retrieves specific task by ID

def add_task(self, task: Task) -> None
# Adds task to container
```

### 4. Storage Module

#### Storage Layout
```
~/.todozi/
├── tasks/          # Legacy collections
│   ├── active.json
│   ├── completed.json
│   └── archived.json
└── projects/       # New project containers
    ├── project1.json
    └── project2.json
```

#### Key Storage Functions

**Directory Management:**
```python
def get_storage_dir() -> Result[Path]
# Returns and ensures existence of storage root directory

def _ensure_dir(path: Path) -> Result[None]
# Ensures directory exists, creates if necessary
```

**JSON Operations:**
```python
def _load_json(path: Path) -> Result[Any]
# Loads JSON data from file with error handling

def _save_json(path: Path, data: Any) -> Result[None]
# Saves data as JSON with proper formatting
```

**Collection Operations:**
```python
def load_task_collection(collection_name: str) -> Result[Collection]
# Loads legacy task collection

def load_project_task_container(project_name: str) -> Result[ProjectTaskContainer]
# Loads project container

def save_project_task_container(container: ProjectTaskContainer) -> Result[None]
# Saves project container to disk

def list_project_task_containers() -> Result[List[ProjectTaskContainer]]
# Lists all available project containers
```

### 5. Embedding Service

#### `TodoziEmbeddingService` Class
Generates vector embeddings for task content.

**Configuration:**
```python
@dataclass
class TodoziEmbeddingConfig:
    model_name: str = "mini"    # Embedding model identifier
    dimension: int = 64        # Vector dimension size
```

**Methods:**
```python
async def initialize(self) -> Result[None]
# Initializes embedding service

def prepare_task_content(self, task: Task) -> str
# Prepares text content from task data for embedding

async def generate_embedding(self, text: str) -> Result[List[float]]
# Generates embedding vector from text content
```

**Embedding Generation Algorithm:**
1. Create deterministic pseudo-random vector from text hash
2. Apply normalization to ensure unit vector
3. Return as list of floats

### 6. Migration Engine

#### `MigrationConfig` Class
```python
@dataclass(frozen=True)
class MigrationConfig:
    dry_run: bool = False          # Preview mode without actual changes
    verbose: bool = False          # Detailed logging output  
    force_overwrite: bool = False  # Overwrite existing project containers
    batch_size: int = 100          # Processing batch size
```

#### `TaskMigrator` Class
Core migration logic implementation.

**Builder Methods:**
```python
def with_dry_run(self, dry_run: bool) -> "TaskMigrator"
def with_verbose(self, verbose: bool) -> "TaskMigrator"  
def with_force_overwrite(self, force_overwrite: bool) -> "TaskMigrator"
```

**Primary Migration Flow:**
```python
async def migrate(self) -> Result[MigrationReport]
# Executes complete migration process

def validate_migration(self) -> Result[bool]
# Validates migration integrity

def cleanup_legacy(self) -> Result[None]
# Cleans up empty legacy collections
```

**Migration Steps:**
1. Load legacy tasks from collections
2. Group tasks by project association
3. Generate embeddings for each task
4. Save to project containers
5. Validate migration integrity
6. Clean up legacy data

### 7. CLI Interface

#### `MigrationCli` Class
Command-line interface for migration operations.

**Usage Example:**
```python
cli = (
    MigrationCli()
    .with_verbose(True)
    .with_dry_run(False) 
    .with_force(False)
)
result = await cli.run()
```

## Performance Considerations

### Memory Management
- Tasks processed in batches (`batch_size` configuration)
- Streaming file operations for large collections
- Lazy loading of project containers

### Async Operations
- Embedding generation uses async/await for I/O efficiency
- Parallel processing capabilities for multiple projects

### Storage Efficiency
- JSON serialization with compression considerations
- Incremental saving to prevent data loss

## Error Handling and Edge Cases

### Common Error Scenarios
- **Missing files**: Graceful handling of non-existent collections
- **Corrupted data**: JSON parsing errors with detailed reporting
- **Permission issues**: Storage directory access problems
- **Duplicate tasks**: Skip or merge logic based on configuration

### Recovery Strategies
- Partial migration with error reporting
- Dry-run mode for safe testing
- Validation step for integrity checking

## Dependencies and Requirements

### Python Requirements
```python
# Core dependencies
import asyncio    # Async operations
import json       # JSON serialization  
import logging    # Logging framework
import math       # Mathematical operations
from pathlib import Path  # Path operations
from dataclasses import dataclass  # Data classes
from typing import Any, Dict, Iterable, List, Optional, Tuple, Type, TypeVar, Generic, Union  # Type hints

# Testing dependencies (optional)
import pytest
import pytest_asyncio
```

### System Requirements
- Python 3.7+
- Read/write access to user home directory
- Sufficient disk space for task storage

## Testing Strategy

### Unit Tests
```python
def test_task_migrator_creation()
def test_task_migrator_builder() 
def test_migration_cli_builder()
```

### Async Integration Tests
```python
@pytest.mark.asyncio
async def test_migration_happy_path()
```

### Test Coverage
- Configuration validation
- Builder pattern functionality
- Migration flow integration
- Error handling scenarios

## Usage Examples

### Basic Migration
```python
async def migrate_tasks():
    migrator = TaskMigrator(MigrationConfig(verbose=True))
    result = await migrator.migrate()
    
    if result.is_ok():
        print("Migration successful!")
    else:
        print(f"Migration failed: {result.error}")
```

### Advanced Configuration
```python
config = MigrationConfig(
    dry_run=True,          # Preview mode
    verbose=True,          # Detailed logging
    force_overwrite=False, # Safe migration
    batch_size=50          # Smaller batches
)

migrator = TaskMigrator(config)
```

### CLI Integration
```python
if __name__ == "__main__":
    asyncio.run(main())  # Uses default CLI configuration
```

## Limitations and Constraints

### Technical Limitations
- Embedding service uses deterministic pseudo-random vectors (not ML-based)
- Single-threaded async processing (no true parallelism)
- JSON storage format (no database backend)

### Operational Constraints
- Migration requires consistent project naming conventions
- No automatic conflict resolution for duplicate tasks
- Legacy cleanup only removes empty collections

## Future Enhancements

### Planned Improvements
- True machine learning embeddings integration
- Database backend support (SQLite, PostgreSQL)
- Parallel processing for large datasets
- Real-time migration progress tracking
- Rollback capabilities for failed migrations

This documentation provides comprehensive coverage of the Todozi Task Migration System's architecture, components, and operational characteristics. The system represents a robust foundation for task data migration with extensible design patterns and comprehensive error handling.