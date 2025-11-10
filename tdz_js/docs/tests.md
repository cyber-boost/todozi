# Todozi - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [Enums](#enums)
5. [Error Handling](#error-handling)
6. [Model Classes](#model-classes)
7. [Storage System](#storage-system)
8. [Configuration](#configuration)
9. [Testing Framework](#testing-framework)
10. [Usage Examples](#usage-examples)
11. [Performance Analysis](#performance-analysis)
12. [Security Considerations](#security-considerations)
13. [Deployment Instructions](#deployment-instructions)
14. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a task management system implemented in JavaScript that provides a robust framework for creating, organizing, and tracking tasks and projects. The system is designed with a focus on flexibility, extensibility, and data integrity.

### Key Features

- Task creation with detailed metadata
- Project organization and management
- Priority and status tracking
- Assignee management (AI, Human, Collaborative)
- Filtering and querying capabilities
- Comprehensive error handling
- Built-in testing framework
- Configurable storage system

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Todozi System                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Models    │  │   Storage   │  │  Config     │  │   Errors    │        │
│  │             │  │             │  │             │  │             │        │
│  │  Task       │  │  Storage    │  │  Config     │  │ TodoziError │        │
│  │  Project    │  │             │  │             │  │             │        │
│  │  Collection │  │             │  │             │  │             │        │
│  │  TaskUpdate │  │             │  │             │  │             │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Enums     │  │   Filters   │  │   Testing   │  │   Utilities │        │
│  │             │  │             │  │             │  │             │        │
│  │  Priority   │  │ TaskFilters │  │  Test Fns   │  │  Helpers    │        │
│  │  Status     │  │             │  │             │  │             │        │
│  │  Assignee   │  │             │  │             │  │             │        │
│  │  ProjectSt  │  │             │  │             │  │             │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Design Patterns Used

1. **Builder Pattern**: `TaskUpdate` class uses a fluent interface for building updates
2. **Factory Pattern**: `Task.new_full()` method for creating fully initialized tasks
3. **Singleton Pattern**: `Config.default()` provides a single source of default configuration
4. **Repository Pattern**: `TaskCollection` manages task persistence and retrieval
5. **Strategy Pattern**: Different parsing functions for enums
6. **Error Handling Pattern**: Custom error types with detailed metadata

## Core Components

### Enums

#### Priority Enum

```javascript
const Priority = {
    Low: 'low',
    Medium: 'medium',
    High: 'high',
    Critical: 'critical',
    Urgent: 'urgent'
};
```

Represents the priority levels for tasks, with Urgent being the highest priority.

#### Status Enum

```javascript
const Status = {
    Todo: 'todo',
    InProgress: 'in_progress',
    Blocked: 'blocked',
    Review: 'review',
    Done: 'done',
    Cancelled: 'cancelled',
    Deferred: 'deferred'
};
```

Represents the current state of a task in its lifecycle.

#### Assignee Enum

```javascript
const Assignee = {
    Ai: 'ai',
    Human: 'human',
    Collaborative: 'collaborative'
};
```

Defines who is responsible for completing a task.

#### ProjectStatus Enum

```javascript
const ProjectStatus = {
    Active: 'active',
    Archived: 'archived',
    Completed: 'completed'
};
```

Represents the current state of a project.

## Error Handling

### TodoziError Class

The `TodoziError` class extends the built-in JavaScript `Error` class and provides structured error handling with additional metadata.

#### Constructor

```javascript
new TodoziError(message, type, details = {})
```

**Parameters:**
- `message` (string): Human-readable error message
- `type` (string): Error type identifier
- `details` (Object): Additional error context (default: `{}`)

**Properties:**
- `name`: Always 'TodoziError'
- `message`: The error message
- `type`: Error type identifier
- `details`: Additional context about the error

#### Static Methods

##### taskNotFound(id)
Creates an error for when a task cannot be found.

**Parameters:**
- `id` (string): The ID of the task that was not found

**Returns:** TodoziError instance

##### invalidPriority(priority)
Creates an error for invalid priority values.

**Parameters:**
- `priority` (string): The invalid priority value

**Returns:** TodoziError instance

##### validation(message)
Creates a validation error.

**Parameters:**
- `message` (string): Validation error message

**Returns:** TodoziError instance

##### invalidProgress(progress)
Creates an error for invalid progress values.

**Parameters:**
- `progress` (number): The invalid progress value

**Returns:** TodoziError instance

## Model Classes

### Task Class

Represents a single task in the system.

#### Constructor

```javascript
new Task(userId, action, time, priority, parentProject, status)
```

**Parameters:**
- `userId` (string): ID of the user who created the task
- `action` (string): Description of what needs to be done
- `time` (string): Estimated time to complete the task
- `priority` (Priority): Priority level of the task
- `parentProject` (string): ID of the project this task belongs to
- `status` (Status): Current status of the task

**Properties:**
- `id` (string): Unique identifier for the task
- `userId` (string): ID of the user who created the task
- `action` (string): Description of what needs to be done
- `time` (string): Estimated time to complete the task
- `priority` (Priority): Priority level of the task
- `parentProject` (string): ID of the project this task belongs to
- `status` (Status): Current status of the task
- `assignee` (Assignee|null): Who is responsible for the task
- `tags` (Array): Tags associated with the task
- `dependencies` (Array): IDs of tasks this task depends on
- `contextNotes` (string|null): Additional context information
- `progress` (number|null): Completion percentage (0-100)

#### Static Methods

##### new_full(action, time, priority, parentProject, status, assignee, tags, dependencies, contextNotes, progress)

Creates a fully initialized task with all optional parameters.

**Parameters:**
- `action` (string): Description of what needs to be done
- `time` (string): Estimated time to complete the task
- `priority` (Priority): Priority level of the task
- `parentProject` (string): ID of the project this task belongs to
- `status` (Status): Current status of the task
- `assignee` (Assignee): Who is responsible for the task
- `tags` (Array): Tags associated with the task
- `dependencies` (Array): IDs of tasks this task depends on
- `contextNotes` (string): Additional context information
- `progress` (number): Completion percentage (0-100)

**Returns:** Task instance

**Throws:** TodoziError if progress is invalid

#### Instance Methods

##### update(updates)

Updates the task with new values.

**Parameters:**
- `updates` (TaskUpdate): Object containing the updates to apply

**Throws:** TodoziError if progress is invalid

##### complete()

Marks the task as completed.

**Side Effects:**
- Sets status to `Status.Done`
- Sets progress to 100

##### is_completed()

Checks if the task is completed.

**Returns:** boolean

##### is_active()

Checks if the task is active (not completed or cancelled).

**Returns:** boolean

### TaskUpdate Class

Builder class for creating task updates using a fluent interface.

#### Constructor

```javascript
new TaskUpdate()
```

Initializes a new TaskUpdate with all properties set to undefined.

**Properties:**
- `action` (string|undefined)
- `priority` (Priority|undefined)
- `status` (Status|undefined)
- `progress` (number|undefined)

#### Instance Methods

##### with_action(action)

Sets the action property.

**Parameters:**
- `action` (string): New action description

**Returns:** this (for chaining)

##### with_priority(priority)

Sets the priority property.

**Parameters:**
- `priority` (Priority): New priority level

**Returns:** this (for chaining)

##### with_status(status)

Sets the status property.

**Parameters:**
- `status` (Status): New status

**Returns:** this (for chaining)

##### with_progress(progress)

Sets the progress property.

**Parameters:**
- `progress` (number): New progress percentage (0-100)

**Returns:** this (for chaining)

##### new()

Creates a new TaskUpdate instance.

**Returns:** TaskUpdate

### Project Class

Represents a project that contains multiple tasks.

#### Constructor

```javascript
new Project(name, description)
```

**Parameters:**
- `name` (string): Name of the project
- `description` (string|null): Description of the project (optional)

**Properties:**
- `name` (string): Name of the project
- `description` (string|null): Description of the project
- `status` (ProjectStatus): Current status of the project
- `tasks` (Array): IDs of tasks in this project

#### Instance Methods

##### add_task(taskId)

Adds a task to the project.

**Parameters:**
- `taskId` (string): ID of the task to add

**Behavior:** Prevents duplicate task IDs

##### remove_task(taskId)

Removes a task from the project.

**Parameters:**
- `taskId` (string): ID of the task to remove

##### archive()

Archives the project.

**Side Effects:** Sets status to `ProjectStatus.Archived`

##### complete()

Marks the project as completed.

**Side Effects:** Sets status to `ProjectStatus.Completed`

### TaskCollection Class

Manages a collection of tasks with filtering capabilities.

#### Constructor

```javascript
new TaskCollection()
```

Initializes an empty task collection.

**Properties:**
- `tasks` (Map): Map of task IDs to Task objects

#### Instance Methods

##### add_task(task)

Adds a task to the collection.

**Parameters:**
- `task` (Task): Task to add

##### get_task(taskId)

Retrieves a task by ID.

**Parameters:**
- `taskId` (string): ID of the task to retrieve

**Returns:** Task|null

##### get_all_tasks()

Retrieves all tasks in the collection.

**Returns:** Array of Task objects

##### remove_task(taskId)

Removes a task from the collection.

**Parameters:**
- `taskId` (string): ID of the task to remove

**Returns:** boolean (true if task was removed, false if not found)

##### get_filtered_tasks(filters)

Retrieves tasks that match the given filters.

**Parameters:**
- `filters` (TaskFilters): Object containing filter criteria

**Returns:** Array of Task objects that match the filters

### TaskFilters Class

Container for task filtering criteria.

#### Constructor

```javascript
new TaskFilters()
```

Initializes a new TaskFilters with all properties set to undefined.

**Properties:**
- `priority` (Priority|undefined)
- `project` (string|undefined)
- `status` (Status|undefined)

### Config Class

Provides default configuration values for the system.

#### Static Methods

##### default()

Returns the default configuration.

**Returns:** Object with configuration properties:
- `version` (string): System version
- `default_project` (string): Default project name
- `auto_backup` (boolean): Whether to enable automatic backups
- `backup_interval` (string): Backup frequency
- `ai_enabled` (boolean): Whether AI features are enabled
- `default_assignee` (Assignee): Default assignee for new tasks
- `date_format` (string): Date format string
- `timezone` (string): Timezone for date operations

### Storage Class

Abstract storage interface (currently a mock implementation).

#### Static Methods

##### new()

Creates a new storage instance.

**Returns:** Promise that resolves to a storage object

## Storage System

The storage system is designed to be modular and extensible. Currently, it's implemented as a mock for testing purposes, but it can be extended to support various storage backends.

### Storage Structure

```
.todozi/
├── config.json
├── tasks/
│   ├── active.json
│   ├── completed.json
│   └── archived.json
├── projects/
│   └── general.json
├── templates/
└── backups/
```

### Data Persistence

Tasks are organized by their status:
- Active tasks: Currently in progress or pending
- Completed tasks: Finished tasks
- Archived tasks: Old or inactive tasks

Projects are stored separately with references to their tasks.

## Configuration

The system uses a configuration object to control behavior. The default configuration includes:

```javascript
{
    version: "1.2.0",
    default_project: "general",
    auto_backup: true,
    backup_interval: "daily",
    ai_enabled: true,
    default_assignee: Assignee.Collaborative,
    date_format: "%Y-%m-%d %H:%M:%S",
    timezone: "UTC"
}
```

## Testing Framework

The system includes a comprehensive testing framework with unit tests for all core functionality.

### Test Utilities

#### create_test_storage()

Creates a temporary storage environment for testing.

**Returns:** Promise that resolves to an object with:
- `tempDir` (string): Path to temporary directory
- `storage` (Object): Storage instance

### Test Functions

1. `test_task_creation()` - Tests basic task creation
2. `test_task_creation_full()` - Tests full task creation with all parameters
3. `test_task_creation_invalid_progress()` - Tests invalid progress validation
4. `test_task_update()` - Tests task updating functionality
5. `test_task_complete()` - Tests task completion
6. `test_task_is_active()` - Tests active task detection
7. `test_priority_parsing()` - Tests priority parsing
8. `test_status_parsing()` - Tests status parsing
9. `test_assignee_parsing()` - Tests assignee parsing
10. `test_project_creation()` - Tests project creation
11. `test_project_add_task()` - Tests adding tasks to projects
12. `test_project_remove_task()` - Tests removing tasks from projects
13. `test_project_archive()` - Tests project archiving
14. `test_project_complete()` - Tests project completion
15. `test_task_collection()` - Tests task collection management
16. `test_task_collection_filtering()` - Tests task filtering
17. `test_config_default()` - Tests default configuration
18. `test_task_update_validation()` - Tests task update validation
19. `test_error_types()` - Tests error type creation

### runTests()

Runs all test functions and reports results.

## Usage Examples

### Creating a Basic Task

```javascript
const task = new Task(
    "user_123",
    "Complete project documentation",
    "2 hours",
    Priority.High,
    "documentation-project",
    Status.Todo
);

console.log(task.id); // task_xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
console.log(task.action); // "Complete project documentation"
```

### Creating a Fully Configured Task

```javascript
const task = Task.new_full(
    "Review code changes",
    "30 minutes",
    Priority.Critical,
    "development",
    Status.InProgress,
    Assignee.Human,
    ["code-review", "urgent"],
    ["task_001"],
    "Review the latest pull request",
    25
);

console.log(task.assignee); // "human"
console.log(task.tags); // ["code-review", "urgent"]
console.log(task.progress); // 25
```

### Updating a Task

```javascript
const task = new Task(
    "user_123",
    "Initial task",
    "1 hour",
    Priority.Medium,
    "project1",
    Status.Todo
);

const updates = new TaskUpdate()
    .with_action("Updated task description")
    .with_priority(Priority.High)
    .with_status(Status.InProgress)
    .with_progress(50);

task.update(updates);

console.log(task.action); // "Updated task description"
console.log(task.priority); // "high"
console.log(task.status); // "in_progress"
console.log(task.progress); // 50
```

### Creating and Managing Projects

```javascript
const project = new Project(
    "Website Redesign",
    "Complete overhaul of company website"
);

const task1 = new Task(
    "user_123",
    "Design homepage mockup",
    "4 hours",
    Priority.High,
    project.name,
    Status.Todo
);

const task2 = new Task(
    "user_123",
    "Implement responsive design",
    "8 hours",
    Priority.Medium,
    project.name,
    Status.Todo
);

project.add_task(task1.id);
project.add_task(task2.id);

console.log(project.tasks.length); // 2
console.log(project.status); // "active"

project.complete();
console.log(project.status); // "completed"
```

### Filtering Tasks

```javascript
const collection = new TaskCollection();

// Add some tasks
const task1 = new Task("user_123", "Low priority", "1h", Priority.Low, "proj1", Status.Todo);
const task2 = new Task("user_123", "High priority", "2h", Priority.High, "proj2", Status.InProgress);

collection.add_task(task1);
collection.add_task(task2);

// Filter by priority
const filters = new TaskFilters();
filters.priority = Priority.High;

const highPriorityTasks = collection.get_filtered_tasks(filters);
console.log(highPriorityTasks.length); // 1
console.log(highPriorityTasks[0].action); // "High priority"
```

### Error Handling

```javascript
try {
    const task = Task.new_full(
        "Invalid task",
        "1 hour",
        Priority.Medium,
        "test-project",
        Status.Todo,
        null,
        [],
        [],
        null,
        150 // Invalid progress
    );
} catch (error) {
    if (error instanceof TodoziError) {
        console.log(`Error type: ${error.type}`);
        console.log(`Error message: ${error.message}`);
        console.log(`Error details:`, error.details);
    }
}
```

## Performance Analysis

### Time Complexity

- Task creation: O(1)
- Task retrieval: O(1) (Map lookup)
- Task filtering: O(n) where n is the number of tasks
- Project task management: O(n) for add/remove operations
- Task updates: O(1)

### Memory Usage

- Each task: ~200-300 bytes (depending on content)
- Task collection: O(n) where n is the number of tasks
- Projects: O(m) where m is the number of tasks in the project

### Scalability Considerations

1. **Task Storage**: For large numbers of tasks, consider implementing pagination or lazy loading
2. **Filtering**: Complex filtering operations may benefit from indexing
3. **Persistence**: The current JSON-based storage may not scale well for very large datasets
4. **Memory Management**: For long-running applications, consider implementing task eviction policies

## Security Considerations

### Data Validation

The system implements comprehensive input validation:
- Progress values are constrained to 0-100
- Priority, status, and assignee values are validated against enums
- Task IDs are generated using UUIDs to prevent collisions

### Error Handling

Custom error types provide structured error information without exposing internal implementation details.

### Storage Security

The current mock storage implementation should be replaced with a secure storage solution in production:
- File permissions should be properly set
- Sensitive data should be encrypted
- Access controls should be implemented

## Deployment Instructions

### Prerequisites

1. Node.js (version 12 or higher)
2. npm (Node Package Manager)
3. File system access permissions

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd todozi

# Install dependencies
npm install
```

### Configuration

1. Create a `.todozi` directory in the user's home directory
2. Copy the default configuration or create a custom one
3. Set appropriate file permissions

### Running Tests

```bash
# Run all tests
npm test

# Run specific tests (if test runner supports it)
npm test -- --filter="task_creation"
```

### Production Deployment

1. Replace the mock storage with a production-ready storage solution
2. Configure appropriate backup strategies
3. Set up monitoring and logging
4. Implement proper access controls
5. Configure environment-specific settings

## Troubleshooting Guide

### Common Issues

#### 1. Task Creation Fails with Invalid Progress

**Symptom:** `TodoziError: Invalid progress: 150`

**Cause:** Progress value must be between 0 and 100

**Solution:** Ensure progress values are within valid range:
```javascript
// Correct
const task = Task.new_full(..., 75);

// Incorrect
const task = Task.new_full(..., 150);
```

#### 2. Task Not Found Errors

**Symptom:** `TodoziError: Task not found: task_xyz`

**Cause:** Attempting to access a task that doesn't exist or has been deleted

**Solution:** 
- Verify the task ID is correct
- Check if the task was properly created
- Ensure the task hasn't been removed from the collection

#### 3. Invalid Enum Values

**Symptom:** `TodoziError: Invalid priority: super_high`

**Cause:** Using an unrecognized enum value

**Solution:** Use valid enum values:
```javascript
// Correct
const task = new Task(..., Priority.High, ...);

// Incorrect
const task = new Task(..., "super_high", ...);
```

#### 4. Storage Initialization Failures

**Symptom:** Errors during storage setup

**Cause:** Insufficient file permissions or disk space

**Solution:**
- Check file system permissions
- Verify available disk space
- Ensure the `.todozi` directory can be created

### Debugging Tips

1. **Enable verbose logging** to see detailed operation information
2. **Use the test framework** to isolate issues
3. **Check error details** in TodoziError instances for specific information
4. **Verify data integrity** by examining stored JSON files directly

### Performance Issues

#### Slow Task Filtering

**Symptoms:** Long delays when filtering large task collections

**Solutions:**
1. Implement indexing for frequently filtered properties
2. Use pagination for large result sets
3. Consider caching frequently accessed filters

#### Memory Leaks

**Symptoms:** Gradually increasing memory usage over time

**Solutions:**
1. Implement proper cleanup of unused task references
2. Use weak references where appropriate
3. Monitor memory usage and implement garbage collection strategies

This comprehensive documentation provides a complete overview of the Todozi system, covering all aspects from basic usage to advanced deployment considerations. The modular design and comprehensive testing framework make it suitable for both small personal projects and larger team-based task management scenarios.