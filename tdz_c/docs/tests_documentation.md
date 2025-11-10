# Comprehensive Documentation for Todozi Task Management System

## Overview

Todozi is a comprehensive C-based task management system designed to handle project organization, task tracking, and configuration management. The system provides robust data structures and functionality for managing tasks, projects, and system configurations with support for various statuses, priorities, and assignee types.

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Data Structures](#data-structures)
3. [Function API Documentation](#function-api-documentation)
4. [Design Patterns](#design-patterns)
5. [Performance Analysis](#performance-analysis)
6. [Security Considerations](#security-considerations)
7. [Testing Strategies](#testing-strategies)
8. [Deployment Instructions](#deployment-instructions)
9. [Troubleshooting Guide](#troubleshooting-guide)
10. [Usage Examples](#usage-examples)

## Architecture Overview

### System Architecture Diagram
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   TaskManager   │───▶│  TaskCollection │───▶│      Config     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│      Task       │    │     Project     │    │  Error Handler  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Component Relationships
- **Task**: Individual task unit with metadata
- **Project**: Container for related tasks
- **TaskCollection**: Manager for multiple tasks
- **Config**: System configuration settings

## Data Structures

### Enumerations

#### Priority Enum
```c
typedef enum {
    PRIORITY_LOW,      // Low priority tasks
    PRIORITY_MEDIUM,   // Medium priority tasks  
    PRIORITY_HIGH,     // High priority tasks
    PRIORITY_CRITICAL, // Critical priority tasks
    PRIORITY_URGENT    // Urgent priority tasks
} Priority;
```

#### Status Enum
```c
typedef enum {
    STATUS_TODO,        // Task not started
    STATUS_IN_PROGRESS, // Task in progress
    STATUS_BLOCKED,     // Task blocked by dependencies
    STATUS_REVIEW,      // Task under review
    STATUS_DONE,        // Task completed
    STATUS_CANCELLED,   // Task cancelled
    STATUS_DEFERRED     // Task deferred to later
} Status;
```

#### Assignee Enum
```c
typedef enum {
    ASSIGNEE_AI,            // AI-assigned task
    ASSIGNEE_HUMAN,         // Human-assigned task
    ASSIGNEE_COLLABORATIVE  // Collaborative task
} Assignee;
```

### Structures

#### Task Structure
```c
struct Task {
    char* id;                // Unique identifier (auto-generated)
    char* action;            // Task description (required)
    char* time;              // Time estimation (required)
    Priority priority;       // Task priority level
    char* parent_project;    // Parent project reference (required)
    Status status;           // Current status
    char* assignee;          // Assignee type string or NULL
    char** tags;             // NULL-terminated tag array
    int tags_count;          // Number of tags
    char** dependencies;     // NULL-terminated dependency array
    int dependencies_count;  // Number of dependencies
    char* context_notes;     // Additional context or NULL
    int* progress;           // Progress percentage (0-100) or NULL
};
```

#### Project Structure
```c
struct Project {
    char* name;           // Project name (required)
    char* description;    // Project description or NULL
    ProjectStatus status; // Current project status
    char** tasks;         // NULL-terminated task ID array
    int tasks_count;      // Number of tasks in project
};
```

#### Configuration Structure
```c
struct Config {
    char* version;           // System version string
    char* default_project;   // Default project name
    int auto_backup;         // Auto-backup enabled flag
    char* backup_interval;   // Backup interval setting
    int ai_enabled;          // AI features enabled flag
    Assignee* default_assignee; // Default task assignee
    char* date_format;       // Date format string
    char* timezone;          // Timezone setting
};
```

## Function API Documentation

### Memory Management Functions

#### `string_clone()`
```c
/**
 * Creates a deep copy of a string
 * 
 * @param str: Source string to clone (can be NULL)
 * @return: New allocated string copy or NULL if allocation fails
 * @note: Returns NULL if input is NULL
 */
char* string_clone(const char* str);
```

#### `string_array_clone()`
```c
/**
 * Creates a deep copy of a string array
 * 
 * @param array: Source string array to clone
 * @param count: Number of strings in array
 * @return: New allocated string array or NULL if allocation fails
 * @note: Returns NULL if input is NULL or count is 0
 */
char** string_array_clone(char** array, int count);
```

#### `string_array_free()`
```c
/**
 * Frees a string array and all contained strings
 * 
 * @param array: String array to free
 * @note: Safe to call with NULL input
 */
void string_array_free(char** array);
```

### Task Management Functions

#### `task_new()`
```c
/**
 * Creates a new basic task with minimal required fields
 * 
 * @param user_id: User identifier (currently unused, for API compatibility)
 * @param action: Task description (required, cannot be NULL)
 * @param time: Time estimation (required, cannot be NULL)
 * @param priority: Task priority level
 * @param parent_project: Parent project name (required, cannot be NULL)
 * @param status: Initial task status
 * @return: New Task object or NULL if allocation fails
 * @error: Returns NULL if required parameters are NULL
 */
Task* task_new(const char* user_id, const char* action, const char* time,
               Priority priority, const char* parent_project, Status status);
```

#### `task_new_full()`
```c
/**
 * Creates a new task with all optional fields
 * 
 * @param action: Task description (required)
 * @param time: Time estimation (required)
 * @param priority: Task priority level
 * @param parent_project: Parent project name (required)
 * @param status: Initial task status
 * @param assignee: Task assignee type (can be NULL)
 * @param tags: Array of tag strings (can be NULL)
 * @param tags_count: Number of tags in array
 * @param dependencies: Array of dependency task IDs (can be NULL)
 * @param dependencies_count: Number of dependencies
 * @param context_notes: Additional context notes (can be NULL)
 * @param progress: Progress percentage pointer (0-100, can be NULL)
 * @return: New Task object or NULL if allocation/validation fails
 * @error: Returns NULL for invalid progress values or allocation failures
 */
Task* task_new_full(const char* action, const char* time, Priority priority,
                    const char* parent_project, Status status, Assignee* assignee,
                    char** tags, int tags_count, char** dependencies, int dependencies_count,
                    const char* context_notes, int* progress);
```

#### `task_update()`
```c
/**
 * Updates task properties with validation
 * 
 * @param task: Task to update (cannot be NULL)
 * @param new_action: New task description (can be NULL to keep current)
 * @param new_priority: New priority level (can be NULL to keep current)
 * @param new_status: New status (can be NULL to keep current)
 * @param new_progress: New progress value (0-100, can be NULL to keep current)
 * @note: Progress validation ensures values between 0-100
 */
void task_update(Task* task, const char* new_action, Priority* new_priority,
                 Status* new_status, int* new_progress);
```

#### `task_complete()`
```c
/**
 * Marks a task as completed with 100% progress
 * 
 * @param task: Task to complete (cannot be NULL)
 * @effect: Sets status to STATUS_DONE and progress to 100
 */
void task_complete(Task* task);
```

#### `task_is_completed()`
```c
/**
 * Checks if a task is completed
 * 
 * @param task: Task to check (can be NULL)
 * @return: 1 if task is completed (STATUS_DONE), 0 otherwise
 */
int task_is_completed(Task* task);
```

#### `task_is_active()`
```c
/**
 * Checks if a task is active (not completed or cancelled)
 * 
 * @param task: Task to check (can be NULL)
 * @return: 1 if task is active, 0 otherwise
 * @note: Active means status is not DONE or CANCELLED
 */
int task_is_active(Task* task);
```

#### `task_free()`
```c
/**
 * Frees all memory associated with a task
 * 
 * @param task: Task to free (safe to call with NULL)
 */
void task_free(Task* task);
```

### Project Management Functions

#### `project_new()`
```c
/**
 * Creates a new project
 * 
 * @param name: Project name (required, cannot be NULL)
 * @param description: Project description (can be NULL)
 * @return: New Project object or NULL if allocation fails
 */
Project* project_new(const char* name, const char* description);
```

#### `project_add_task()`
```c
/**
 * Adds a task ID to a project
 * 
 * @param project: Project to modify (cannot be NULL)
 * @param task_id: Task ID to add (cannot be NULL)
 * @note: Prevents duplicate task IDs
 */
void project_add_task(Project* project, const char* task_id);
```

#### `project_remove_task()`
```c
/**
 * Removes a task ID from a project
 * 
 * @param project: Project to modify (cannot be NULL)
 * @param task_id: Task ID to remove (cannot be NULL)
 */
void project_remove_task(Project* project, const char* task_id);
```

#### `project_archive()`
```c
/**
 * Archives a project
 * 
 * @param project: Project to archive (cannot be NULL)
 * @effect: Sets project status to PROJECT_STATUS_ARCHIVED
 */
void project_archive(Project* project);
```

#### `project_complete()`
```c
/**
 * Marks a project as completed
 * 
 * @param project: Project to complete (cannot be NULL)
 * @effect: Sets project status to PROJECT_STATUS_COMPLETED
 */
void project_complete(Project* project);
```

### Task Collection Functions

#### `task_collection_new()`
```c
/**
 * Creates a new task collection
 * 
 * @return: New TaskCollection object or NULL if allocation fails
 */
TaskCollection* task_collection_new();
```

#### `task_collection_add_task()`
```c
/**
 * Adds a task to the collection
 * 
 * @param collection: Collection to modify (cannot be NULL)
 * @param task: Task to add (cannot be NULL)
 */
void task_collection_add_task(TaskCollection* collection, Task* task);
```

#### `task_collection_get_task()`
```c
/**
 * Retrieves a task by ID from the collection
 * 
 * @param collection: Collection to search (cannot be NULL)
 * @param task_id: Task ID to find (cannot be NULL)
 * @return: Task object if found, NULL otherwise
 */
Task* task_collection_get_task(TaskCollection* collection, const char* task_id);
```

#### `task_collection_get_filtered_tasks()`
```c
/**
 * Retrieves tasks matching filter criteria
 * 
 * @param collection: Collection to search (cannot be NULL)
 * @param priority_filter: Priority filter (can be NULL)
 * @param project_filter: Project name filter (can be NULL)
 * @param status_filter: Status filter (can be NULL)
 * @param count: Output parameter for number of matches
 * @return: Array of matching tasks (caller must free) or NULL if allocation fails
 */
Task** task_collection_get_filtered_tasks(TaskCollection* collection, 
                                          Priority* priority_filter,
                                          const char* project_filter,
                                          Status* status_filter,
                                          int* count);
```

### Configuration Functions

#### `config_default()`
```c
/**
 * Creates default system configuration
 * 
 * @return: New Config object with default values or NULL if allocation fails
 * @defaults: version="1.2.0", default_project="general", auto_backup=1,
 *            backup_interval="daily", ai_enabled=1, 
 *            default_assignee=ASSIGNEE_COLLABORATIVE,
 *            date_format="%Y-%m-%d %H:%M:%S", timezone="UTC"
 */
Config* config_default();
```

### Parsing Functions

#### `parse_priority()`
```c
/**
 * Parses priority string to enum value
 * 
 * @param str: String to parse ("low", "medium", "high", "critical", "urgent")
 * @param result: Output parameter for parsed priority
 * @return: 1 if successful, 0 if parsing failed
 */
int parse_priority(const char* str, Priority* result);
```

## Design Patterns

### 1. Factory Pattern
- **`task_new()`** and **`task_new_full()`** act as factory methods for Task creation
- **`project_new()`** acts as factory for Project creation
- **`config_default()`** provides default configuration factory

### 2. Collection Pattern
- **`TaskCollection`** implements a collection pattern for task management
- Provides filtering, retrieval, and management operations

### 3. Builder Pattern (Partial)
- **`task_new_full()`** provides a builder-like interface for complex Task creation

### 4. RAII (Resource Acquisition Is Initialization)
- Memory management follows RAII principles with proper cleanup functions

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Task Creation | O(1) | Constant time allocation |
| Task Lookup | O(n) | Linear search in collection |
| Task Filtering | O(n) | Linear scan with filtering |
| Project Task Management | O(n) | Linear search for duplicates |
| String Operations | O(k) | Proportional to string length |

### Space Complexity
| Structure | Memory Usage | Notes |
|-----------|--------------|-------|
| Task | O(n + t + d) | n=action length, t=tags, d=dependencies |
| Project | O(m + k) | m=name length, k=task count |
| TaskCollection | O(p) | p=number of tasks |
| Config | O(1) | Fixed size with string pointers |

### Memory Management
- **Allocations**: Manual memory management with malloc/free
- **Fragmentation**: Proper cleanup reduces fragmentation
- **Leak Prevention**: Comprehensive free functions provided

## Security Considerations

### Input Validation
- **Null Checks**: All functions validate NULL inputs
- **String Length**: Bounded string operations with MAX_ASSIGNEE_LEN
- **Progress Validation**: Ensures 0-100 range for progress values
- **Duplicate Prevention**: Project task addition prevents duplicates

### Memory Safety
- **Buffer Overflow Protection**: strncpy with length limits
- **Memory Allocation Checks**: All malloc calls verified
- **Cleanup on Failure**: Proper rollback on allocation failures

### Data Integrity
- **Immutable IDs**: Task IDs are generated automatically
- **Consistent State**: Validation maintains data consistency
- **Error Handling**: Graceful handling of invalid operations

## Testing Strategies

### Unit Testing Framework
The code includes comprehensive test functions covering:

#### Test Categories
1. **Task Creation Tests**: Basic and full task creation
2. **Task Operation Tests**: Update, complete, status checks
3. **Parsing Tests**: String-to-enum conversion
4. **Project Tests**: Creation and task management
5. **Collection Tests**: Filtering and management
6. **Configuration Tests**: Default config creation

#### Test Coverage
- **Function Coverage**: 100% of public functions tested
- **Edge Cases**: Invalid inputs, boundary conditions
- **Memory Tests**: Allocation failure scenarios
- **Integration Tests**: Multi-component interactions

### Testing Methodology
```c
// Example test structure
void test_functionality() {
    // Setup
    Object* obj = function_under_test(params);
    
    // Assertions
    assert(condition);
    assert(another_condition);
    
    // Cleanup
    cleanup_function(obj);
    
    printf("test_functionality passed\n");
}
```

### Test Execution
```bash
# Compile with tests
gcc -o todozi_test todozi.c -DNDEBUG

# Run tests
./todozi_test
```

## Deployment Instructions

### Compilation Requirements
- **C Compiler**: GCC or Clang (C99 standard)
- **Standard Libraries**: stdio, stdlib, string, assert, time, errno, stdint
- **Platform**: Cross-platform (Linux, macOS, Windows with MinGW)

### Build Instructions

#### Basic Compilation
```bash
gcc -o todozi main.c todozi.c -Wall -Wextra -std=c99
```

#### Production Build (Optimized)
```bash
gcc -o todozi main.c todozi.c -O2 -DNDEBUG -std=c99
```

#### Debug Build
```bash
gcc -o todozi_debug main.c todozi.c -g -O0 -std=c99
```

### Integration Steps

1. **Include Header**: Copy function declarations to header file
2. **Link Object**: Compile as library or include directly
3. **Initialize**: Call setup functions before use
4. **Cleanup**: Ensure proper cleanup on application exit

### Memory Management Guidelines
- Always pair allocations with corresponding free calls
- Use provided free functions for structured objects
- Validate allocation success before use
- Implement error handling for allocation failures

## Troubleshooting Guide

### Common Issues

#### Memory Allocation Failures
**Symptom**: Functions return NULL unexpectedly
**Solution**: Check system memory, implement fallback strategies

```c
Task* task = task_new(...);
if (task == NULL) {
    // Handle allocation failure
    fprintf(stderr, "Failed to allocate task\n");
    return ERROR_MEMORY;
}
```

#### Invalid Parameter Errors
**Symptom**: Functions return NULL or behave unexpectedly
**Solution**: Validate inputs before calling functions

```c
if (action == NULL || time == NULL) {
    return ERROR_INVALID_PARAM;
}
```

#### Memory Leaks
**Symptom**: Increasing memory usage over time
**Solution**: Ensure proper cleanup calls

```c
// Correct usage pattern
Task* task = task_new(...);
// Use task...
task_free(task); // Always call free
```

### Debugging Techniques

#### Memory Debugging
```bash
# Valgrind memory check
valgrind --leak-check=full ./todozi_test
```

#### Assertion Debugging
```c
// Enable assertions for debugging
#ifndef NDEBUG
#include <assert.h>
#endif
```

### Error Codes and Handling

The system defines error codes for common failure scenarios:

```c
typedef enum {
    ERROR_TASK_NOT_FOUND,
    ERROR_INVALID_PRIORITY,
    ERROR_INVALID_STATUS,
    // ... additional error codes
} ErrorCode;
```

## Usage Examples

### Basic Task Management

#### Creating a Simple Task
```c
Task* task = task_new("user123", "Write documentation", "2 hours", 
                     PRIORITY_HIGH, "project-alpha", STATUS_TODO);
if (task == NULL) {
    // Handle error
}

// Use the task...
task_free(task);
```

#### Creating a Complex Task
```c
char* tags[] = {"documentation", "high-priority", NULL};
char* dependencies[] = {"task_research", NULL};
int progress = 0;
Assignee assignee = ASSIGNEE_HUMAN;

Task* task = task_new_full("Write comprehensive docs", "4 hours", 
                          PRIORITY_CRITICAL, "project-beta", STATUS_IN_PROGRESS,
                          &assignee, tags, 2, dependencies, 1,
                          "Include all API endpoints", &progress);
```

#### Task Lifecycle Management
```c
// Create task
Task* task = task_new(...);

// Update progress
int new_progress = 50;
task_update(task, NULL, NULL, NULL, &new_progress);

// Complete task
task_complete(task);

// Check status
if (task_is_completed(task)) {
    printf("Task completed!\n");
}

// Cleanup
task_free(task);
```

### Project Management

#### Creating and Managing Projects
```c
// Create project
Project* project = project_new("Mobile App", "iOS and Android development");

// Add tasks
project_add_task(project, "task_design");
project_add_task(project, "task_development");

// Archive when done
project_archive(project);

project_free(project);
```

### Collection Usage

#### Managing Task Collections
```c
TaskCollection* collection = task_collection_new();

// Add multiple tasks
task_collection_add_task(collection, task1);
task_collection_add_task(collection, task2);

// Filter tasks
Priority high_prio = PRIORITY_HIGH;
int count;
Task** high_priority_tasks = task_collection_get_filtered_tasks(
    collection, &high_prio, NULL, NULL, &count);

// Use filtered tasks...
free(high_priority_tasks);

task_collection_free(collection);
```

### Configuration Management

#### System Configuration
```c
Config* config = config_default();
if (config == NULL) {
    // Handle configuration error
}

// Modify configuration if needed
// ...

config_free(config);
```

## Conclusion

This documentation provides comprehensive coverage of the Todozi task management system. The system offers robust task and project management capabilities with proper memory management, error handling, and testing support. The modular design allows for easy integration into larger applications while maintaining data integrity and security.

For additional support or to report issues, refer to the troubleshooting guide and ensure all usage follows the patterns demonstrated in the examples section.