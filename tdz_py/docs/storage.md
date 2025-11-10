# Todozi Storage Module Technical Documentation

## Overview

The `storage_fixed.py` module provides a comprehensive storage system for the Todozi task management application. This drop-in replacement for the original `storage.py` includes significant architectural improvements, particularly around project-task container consistency and storage-level caching.

## Architecture

### Core Design Principles

- **Exception-based Error Handling**: Pure exception model replacing result-like patterns
- **Status-based Task Organization**: Tasks stored in status buckets (`Todo`, `InProgress`, `Done`, `Completed`, `Cancelled`, `Archived`)
- **LRU Caching**: Storage-level caching for performance optimization
- **Synchronous Initialization**: Removed async from `Storage.new()` to prevent nested event loop issues
- **Context-managed File I/O**: Lightweight context manager for safe file operations

### Directory Structure

```
~/.todozi/
├── tdz.hlx (configuration)
├── tasks/ (legacy task collections)
├── project_tasks/ (project-based containers)
├── projects/ (project metadata)
├── agents/ (agent definitions)
├── backups/ (backup storage)
└── [15+ specialized directories]
```

## Core Components

### Exception Hierarchy

```python
class TodoziError(Exception):
    """Base exception for all storage-related errors"""
    
    @staticmethod
    def storage(message: str) -> "TodoziError"
    @staticmethod  
    def project_not_found(name: str) -> "TodoziError"
    @staticmethod
    def task_not_found(id: str) -> "TodoziError"
    # ... additional factory methods
```

### HLX File Format

The HLX (Hierarchical Local eXchange) format provides a simple JSON-based configuration and data storage system:

```python
class Hlx:
    """Lightweight hierarchical data storage using JSON"""
    
    def load(path: str) -> "Hlx"
    def get(section: str, key: str) -> Optional[HlxValue]
    def set(section: str, key: str, value: Any) -> None
    def save() -> None
```

### Domain Models

#### Task Management
```python
@dataclass
class Task:
    id: str = field(default_factory=lambda: new_id("task_"))
    action: str = ""
    status: Status = Status.Todo
    priority: Priority = Priority.Medium
    parent_project: str = ""
    # ... additional fields
    
    def update(self, updates: "TaskUpdate") -> "Task"
    def to_dict(self) -> Dict[str, Any]
    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "Task"
```

#### Project Task Container
The key improvement addressing inconsistent task storage:

```python
@dataclass
class ProjectTaskContainer:
    project_name: str
    project_hash: str
    _storage: Dict[Status, Dict[str, Task]]  # Status-based buckets
    
    @property
    def active_tasks(self) -> Dict[str, Task]
    @property  
    def completed_tasks(self) -> Dict[str, Task]
    
    def add_task(self, task: Task) -> None
    def update_task_status(self, task_id: str, new_status: Status) -> Optional[Task]
    def all_tasks_by_status(self) -> Dict[Status, Dict[str, Task]]
```

### Storage Context Manager

```python
@contextmanager
def storage_file(path: Path, mode: str = "r", *, mkdir: bool = True):
    """Lightweight context manager for safe file operations"""
    # Handles directory creation and UTF-8 encoding
```

## API Reference

### Storage Class

The main storage interface providing unified access to all storage operations:

```python
class Storage:
    @staticmethod
    async def new() -> "Storage"
    
    # Task Operations
    def add_task(self, task: Task) -> None
    def get_task(self, id: str) -> Task
    def update_task(self, id: str, updates: TaskUpdate) -> None
    def delete_task(self, id: str) -> None
    def list_tasks(self, filters: TaskFilters) -> List[Task]
    
    # Project-based Operations
    async def add_task_to_project(self, task: Task) -> None
    def get_task_from_project(self, project_name: str, task_id: str) -> Task
    def list_tasks_in_project(self, project_name: str, filters: TaskFilters) -> List[Task]
    
    # Migration and Maintenance
    async def migrate_to_project_based(self) -> MigrationReport
    def fix_completed_tasks_consistency(self) -> None
```

### Configuration Management

```python
async def save_config(config: Config) -> None
async def load_config() -> Config
async def is_registered() -> bool
async def get_registration_info() -> Optional[RegistrationInfo]
```

### Project Task Container Management

```python
def save_project_task_container(container: ProjectTaskContainer) -> None
def load_project_task_container(project_name: str) -> ProjectTaskContainer

@lru_cache(maxsize=100)
def list_project_task_containers() -> List[ProjectTaskContainer]
```

## Key Improvements

### 1. Consistent Task Storage
- Fixed inconsistent storage across status buckets
- Unified access through `all_tasks_by_status()` method
- Graceful handling of legacy container versions

### 2. Performance Optimizations
- LRU caching for project task containers (maxsize=100)
- Status-based bucket organization for efficient filtering
- Batch operations where appropriate

### 3. Enhanced Error Handling
- Pure exception model eliminating result-like patterns
- Specific exception types for different error scenarios
- Comprehensive error context in exception messages

### 4. Migration Support
```python
async def migrate_to_project_based(self) -> MigrationReport
```
- Migrates legacy task collections to project-based storage
- Provides detailed migration statistics
- Maintains data integrity during migration

## Usage Examples

### Basic Storage Operations
```python
# Initialize storage
storage = await Storage.new()

# Add a task
task = Task(action="Implement new feature", priority=Priority.High)
await storage.add_task_to_project(task)

# Query tasks
filters = TaskFilters(project="general", search="feature")
tasks = storage.list_tasks_in_project("general", filters)

# Update task status
storage.update_task(task.id, TaskUpdate().with_status(Status.InProgress))
```

### Project Management
```python
# Create project
storage.create_project("web-app", "New web application")

# Get project statistics
stats = storage.get_project_stats("web-app")
print(f"Active tasks: {stats.active_tasks}")

# Archive project  
storage.archive_project("completed-project")
```

### Backup and Recovery
```python
# Create backup
backup_name = storage.create_backup()

# List available backups
backups = storage.list_backups()

# Restore from backup
storage.restore_backup(backup_name)
```

## Agent System Integration

The module includes a comprehensive agent system with specialized agents for different roles:

```python
def create_default_agents() -> None
# Creates 24 specialized agents including:
# - Planner: Strategic planning
# - Coder: Programming specialist  
# - Detective: Code investigation
# - Architect: Defensive design
# - Mason: Foundation building
# - 19 additional specialized agents
```

## Technical Constraints and Limitations

### File System Dependencies
- Requires ~/.todozi directory structure
- UTF-8 encoding for all file operations
- JSON-based storage with 2-space indentation

### Performance Considerations
- LRU cache size limited to 100 project containers
- No distributed locking for concurrent access
- File I/O operations are synchronous

### Memory Usage
- Project task containers cached in memory
- Large task sets may impact performance
- Embedding vectors stored as JSON arrays

## Error Handling

### Common Error Scenarios

1. **File System Errors**
   - Missing directories automatically created
   - Permission errors raise `TodoziError.storage()`

2. **Data Consistency Errors**
   - Missing tasks: `TodoziError.task_not_found()`
   - Missing projects: `TodoziError.project_not_found()`
   - Validation errors: `TodoziError.validation_error()`

3. **Migration Errors**
   - Graceful fallback for legacy container formats
   - Detailed migration reporting

## Dependencies and Requirements

### Python Dependencies
- Python 3.7+
- `dataclasses` (standard library)
- `pathlib` (standard library)
- `asyncio` (standard library)
- `functools.lru_cache` (standard library)

### External Dependencies
- `todozi.emb` module for embedding services
- UUID generation for unique identifiers
- JSON serialization/deserialization

## Testing and Validation

The module includes self-validation capabilities:

```python
def check_folder_structure() -> bool
async def ensure_folder_structure() -> bool
```

## Migration Path

### From Legacy Storage
1. Call `migrate_to_project_based()` to transition from collection-based to project-based storage
2. Use `fix_completed_tasks_consistency()` to address status inconsistencies
3. Validate with `check_folder_structure()`

### Backup Strategy
- Automatic backup creation before major operations
- Manual backup via `create_backup()`
- Restoration via `restore_backup(backup_name)`

## Security Considerations

- No encryption of stored data
- File permissions follow system defaults
- API key storage in plain text (HLX format)
- Recommendation: Secure ~/.todozi directory permissions

This documentation provides comprehensive coverage of the storage module's architecture, APIs, and usage patterns. The improvements in consistency, performance, and error handling make this a robust foundation for the Todozi application's data persistence needs.