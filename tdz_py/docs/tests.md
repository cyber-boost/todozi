# Todozi Technical Documentation

## Overview

Todozi is a Python task management system providing structured task and project management capabilities. The system includes data models, storage management, and comprehensive error handling designed for reliability and extensibility.

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Data Models](#data-models)
3. [Storage System](#storage-system)
4. [Error Handling](#error-handling)
5. [Serialization](#serialization)
6. [Testing Framework](#testing-framework)
7. [Dependencies and Requirements](#dependencies-and-requirements)
8. [Performance Considerations](#performance-considerations)
9. [Technical Constraints](#technical-constraints)

## Architecture Overview

Todozi follows a layered architecture with clear separation between data models, business logic, and storage concerns. The system is designed to be extensible while maintaining type safety and validation.

### Core Components
- **Data Models**: Task, Project, TaskCollection, and configuration objects
- **Enums and Value Objects**: Type-safe enumerations for status, priority, and assignments
- **Storage Layer**: File-based persistence with JSON serialization
- **Error Handling**: Comprehensive exception hierarchy
- **Testing**: Built-in test suite with data model validation

## Data Models

### Task Model

#### `@dataclass Task`
Central data model representing individual tasks with comprehensive metadata.

**Fields:**
- `id: str` - Auto-generated unique identifier (format: `task_[uuid8]`)
- `assignee: Optional[Union[Assignee, str]]` - Task assignee with enum or string flexibility
- `action: str` - Description of the task action
- `time: str` - Time estimation/duration
- `priority: Priority` - Task priority level
- `parent_project: str` - Associated project name
- `status: Status` - Current task status
- `tags: List[str]` - Categorization labels
- `dependencies: List[str]` - Task IDs this task depends on
- `context_notes: Optional[str]` - Additional context information
- `progress: Optional[int]` - Completion percentage (0-100)

**Methods:**

```python
@staticmethod
def new(
    assignee: str,
    action: str,
    time: str,
    priority: Priority,
    parent_project: str,
    status: Status,
) -> "Task"
```
Creates a new task with minimal required fields. Generates auto ID and defaults optional fields.

**Parameters:**
- `assignee: str` - Assignee identifier
- `action: str` - Task description
- `time: str` - Time estimate
- `priority: Priority` - Priority level
- `parent_project: str` - Project name
- `status: Status` - Initial status

**Returns:** New Task instance

```python
@staticmethod
def new_full(
    action: str,
    time: str,
    priority: Priority,
    parent_project: str,
    status: Status,
    assignee: Optional[Union[Assignee, str]],
    tags: List[str],
    dependencies: List[str],
    context_notes: Optional[str],
    progress: Optional[int],
) -> "Task"
```
Creates a task with all fields specified. Includes validation for progress range.

**Validation:** Progress must be between 0-100 inclusive

```python
def update(self, updates: "TaskUpdate") -> None
```
Applies updates to task fields. Validates progress ranges.

```python
def complete(self) -> None
```
Marks task as completed (status=Done, progress=100)

```python
def is_completed(self) -> bool
```
Returns True if task status is Done

```python
def is_active(self) -> bool
```
Returns True if task is not completed and not cancelled

### TaskUpdate Model

#### `@dataclass TaskUpdate`
Immutable update specification for task modification.

**Fields:**
- `action: Optional[str]` - Updated task description
- `priority: Optional[Priority]` - Updated priority
- `status: Optional[Status]` - Updated status
- `progress: Optional[int]` - Updated progress (validated 0-100)

**Builder Pattern Methods:**
```python
def with_action(self, action: str) -> "TaskUpdate"
def with_priority(self, priority: Priority) -> "TaskUpdate" 
def with_status(self, status: Status) -> "TaskUpdate"
def with_progress(self, progress: int) -> "TaskUpdate"
```

**Usage Example:**
```python
update = TaskUpdate.new() \
    .with_action("Updated task") \
    .with_priority(Priority.High) \
    .with_progress(75)
task.update(update)
```

### Project Model

#### `@dataclass Project`
Represents a collection of related tasks.

**Fields:**
- `name: str` - Unique project identifier
- `description: Optional[str]` - Project description
- `status: ProjectStatus` - Current project status
- `tasks: Set[str]` - Set of task IDs belonging to this project

**Methods:**
```python
def add_task(self, task_id: str) -> None
def remove_task(self, task_id: str) -> None
def archive(self) -> None  # Sets status to Archived
def complete(self) -> None  # Sets status to Completed
```

### TaskCollection Model

#### `@dataclass TaskCollection`
Container for managing multiple tasks with filtering capabilities.

**Fields:**
- `tasks: Dict[str, Task]` - Dictionary mapping task IDs to Task objects

**Methods:**
```python
def add_task(self, task: Task) -> None
def get_task(self, task_id: str) -> Optional[Task]
def remove_task(self, task_id: str) -> Optional[Task]
def get_all_tasks(self) -> List[Task]
def get_filtered_tasks(self, f: TaskFilters) -> List[Task]
```

### TaskFilters Model

#### `@dataclass TaskFilters`
Criteria for filtering tasks in a TaskCollection.

**Fields:**
- `priority: Optional[Priority]` - Filter by priority
- `project: Optional[str]` - Filter by project name
- `status: Optional[Status]` - Filter by status

## Enums and Value Objects

### Priority Enum
```python
class Priority(Enum):
    Low = auto()
    Medium = auto() 
    High = auto()
    Critical = auto()
    Urgent = auto()
```

**Parsing:** Case-insensitive with whitespace/underscore normalization
**Serialization:** Lowercase names

### Status Enum
```python
class Status(Enum):
    Todo = auto()
    InProgress = auto()
    Blocked = auto() 
    Review = auto()
    Done = auto()
    Cancelled = auto()
    Deferred = auto()
```

**Special Handling:** Accepts both "cancelled" and "canceled" spelling
**Serialization:** Snake_case format for consistency

### Assignee Enum
```python
class Assignee(Enum):
    Ai = auto()
    Human = auto()
    Collaborative = auto()
```

### ProjectStatus Enum
```python
class ProjectStatus(Enum):
    Active = auto()
    Archived = auto()
    Completed = auto()
```

## Storage System

### Storage Class
File-based persistence layer with organized directory structure.

**Directory Structure:**
```
root/
├── config.json
├── tasks/
│   ├── active.json
│   ├── completed.json
│   └── archived.json
├── projects/
│   └── {project_name}.json
├── templates/
└── backups/
```

**Initialization:**
```python
def __init__(self, root: Path, config: Optional[Config] = None)
```
- Creates directory structure if missing
- Writes default config if config.json doesn't exist

**Key Methods:**
```python
def load_config(self) -> Config
def save_config(self, config: Config) -> None
def load_collection(self, name: str) -> TaskCollection
def save_collection(self, collection: TaskCollection, name: str) -> None
def load_project(self, name: str) -> Project
def save_project(self, project: Project) -> None
def list_project_names(self) -> List[str]
```

## Error Handling

### TodoziError Exception Hierarchy
Comprehensive error types with contextual information.

**Error Types:**
- `TaskNotFound` - Referenced task ID doesn't exist
- `ProjectNotFound` - Referenced project doesn't exist  
- `InvalidProgress` - Progress value outside 0-100 range
- `InvalidPriority` - Unrecognized priority value
- `InvalidStatus` - Unrecognized status value
- `InvalidAssignee` - Unrecognized assignee value
- `ValidationError` - General validation failure

**Error Context:** All errors include contextual information for debugging

**Usage:**
```python
try:
    task = Task.new_full(progress=150, ...)
except TodoziError as e:
    print(f"{e.kind}: {e}")  # "InvalidProgress: Progress must be between 0 and 100, got 150"
```

## Serialization

### JSON Serialization Protocol
All models implement `to_json()` and `from_json()` methods for consistent serialization.

**Task JSON Format:**
```json
{
    "id": "task_abc123",
    "assignee": "human",
    "action": "Test task",
    "time": "1 hour",
    "priority": "high",
    "parent_project": "general",
    "status": "in_progress",
    "tags": ["test", "example"],
    "dependencies": ["task_def456"],
    "context_notes": "Additional context",
    "progress": 50
}
```

**Enum Serialization:**
- Priority, Assignee: lowercase names
- Status: snake_case names
- ProjectStatus: lowercase names

## Testing Framework

### TaskModelTests Class
Comprehensive test suite validating data model behavior and edge cases.

**Test Coverage:**
- Task creation and validation
- Task updates and completion
- Enum parsing and serialization
- Project management operations
- Task collection filtering
- Error handling scenarios
- Configuration management

**Test Execution:**
```python
if __name__ == "__main__":
    # Automated test runner
    # Runs all test_* methods in TaskModelTests
```

## Dependencies and Requirements

### Core Dependencies
- Python 3.7+ (for `from __future__ import annotations`)
- Standard Library Only:
  - `json` - Serialization
  - `re` - String normalization
  - `uuid` - Unique ID generation
  - `tempfile` - Test infrastructure
  - `pathlib` - Path handling
  - `dataclasses` - Data model definitions
  - `enum` - Type-safe enumerations

### Type Hints
Comprehensive type annotations using:
- `Optional`, `Union`, `List`, `Dict`, `Set`, `Tuple`
- Forward references with `from __future__ import annotations`

## Performance Considerations

### Memory Efficiency
- Task IDs use compact UUID format (8 characters)
- Project tasks stored as sets for O(1) membership testing
- Lazy loading of collections from storage

### Computational Complexity
- Task filtering: O(n) where n is number of tasks
- Task lookup: O(1) with dictionary-based storage
- Project task management: O(1) for add/remove operations

### Storage Optimization
- JSON serialization with minimal whitespace in production
- Organized file structure prevents single large files
- Backup management prevents storage bloat

## Technical Constraints

### Validation Constraints
- Progress values must be 0-100 inclusive
- All enum values must be parseable from normalized strings
- Task IDs must follow `task_[a-f0-9]{8}` pattern
- Project names must be unique within storage

### Storage Constraints
- Filesystem-based storage requires write permissions
- JSON file size limited by memory during deserialization
- Backup interval configuration affects storage usage

### Design Constraints
- Maintains compatibility with Rust implementation patterns
- Immutable update patterns for data consistency
- Comprehensive error handling for reliability

## Usage Examples

### Basic Task Management
```python
# Create a new task
task = Task.new(
    assignee="user123",
    action="Write documentation",
    time="2 hours",
    priority=Priority.High,
    parent_project="documentation",
    status=Status.Todo
)

# Update task progress
update = TaskUpdate.new().with_progress(50).with_status(Status.InProgress)
task.update(update)

# Complete task
task.complete()
```

### Project Management
```python
# Create project and add tasks
project = Project(name="new_feature", description="Implement new feature")
project.add_task(task.id)

# Archive completed project  
project.archive()
```

### Storage Operations
```python
# Initialize storage
storage = Storage(Path("/path/to/data"))

# Save and load tasks
collection = TaskCollection()
collection.add_task(task)
storage.save_collection(collection, "active")

loaded = storage.load_collection("active")
```

This documentation provides comprehensive coverage of the Todozi system architecture, data models, and operational characteristics for developers and technical stakeholders.