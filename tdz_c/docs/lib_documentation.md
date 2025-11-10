# TODOZI C API - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Functions Reference](#functions-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)
12. [Error Handling](#error-handling)

## Overview

TODOZI is a comprehensive task management and AI-powered productivity system written in C. The API provides extensive functionality for task management, memory tracking, idea generation, and intelligent task processing with semantic search capabilities.

### Key Features
- **Task Management**: Create, update, search, and manage tasks with priorities and dependencies
- **AI Integration**: Semantic search and intelligent task planning
- **Memory System**: Long-term memory storage and retrieval
- **Idea Tracking**: Capture and organize creative ideas
- **Project Organization**: Multi-project task management
- **Embedding Service**: Vector-based similarity search
- **Agent System**: AI agent assignment and management

## Architecture

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│  Task Management  │  Memory System   │   Idea System       │
│       API         │       API        │       API           │
├─────────────────────────────────────────────────────────────┤
│              Core Service Layer                             │
│  Embedding Service  │  Storage Service  │  Search Engine   │
├─────────────────────────────────────────────────────────────┤
│                    Data Access Layer                        │
│      File Storage      │     Metadata     │   Embeddings    │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships

```
User Input → Task Editor → Task Storage
                            ↓
                    Embedding Service
                            ↓
                    Semantic Search Engine
                            ↓
                    AI Processing Pipeline
                            ↓
                    Result Display
```

## Data Structures

### Core Enumerations

#### TodoziPriority
```c
typedef enum {
    TODOZI_PRIORITY_CRITICAL,  // Highest priority - immediate attention
    TODOZI_PRIORITY_URGENT,    // Time-sensitive tasks
    TODOZI_PRIORITY_HIGH,      // Important tasks
    TODOZI_PRIORITY_MEDIUM,    // Normal priority (default)
    TODOZI_PRIORITY_LOW        // Low priority - can be deferred
} TodoziPriority;
```

#### TodoziStatus
```c
typedef enum {
    TODOZI_STATUS_TODO,        // Task not started
    TODOZI_STATUS_IN_PROGRESS, // Task actively being worked on
    TODOZI_STATUS_DONE,        // Task completed
    TODOZI_STATUS_BLOCKED      // Task cannot proceed due to dependencies
} TodoziStatus;
```

### Main Data Structures

#### Task Structure
```c
struct Task {
    char* id;                    // UUID identifier
    char* user_id;               // User identifier
    char* action;                // Task description
    char* time;                  // Time specification (e.g., "ASAP", "Tomorrow")
    TodoziPriority priority;     // Priority level
    char* parent_project;        // Project name
    TodoziStatus status;         // Current status
    TodoziAssignee assignee;     // Assigned entity
    char** tags;                 // Array of tags
    size_t tags_count;           // Number of tags
    char** dependencies;         // Task dependencies
    size_t dependencies_count;   // Number of dependencies
    char* context_notes;         // Additional context
    int* progress;               // Progress percentage (0-100)
    time_t created_at;           // Creation timestamp
    time_t updated_at;           // Last update timestamp
    float* embedding_vector;     // Semantic embedding vector
    size_t embedding_size;       // Embedding vector size
};
```

#### TaskFilters Structure
```c
struct TaskFilters {
    char* project;              // Filter by project
    TodoziStatus* status;       // Filter by status
    TodoziPriority* priority;   // Filter by priority
    TodoziAssignee* assignee;   // Filter by assignee
    char** tags;                // Filter by tags
    size_t tags_count;          // Number of tags to filter
    char* search;               // Text search query
};
```

## Functions Reference

### Initialization Functions

#### `todozi_init()`
**Purpose**: Initialize the TODOZI system
**Parameters**: None
**Returns**: `TodoziErrorCode` - Success or error code
**Complexity**: O(1)
```c
TodoziErrorCode todozi_init() {
    // Implementation would go here
    return TODOZI_OK;
}
```

#### `todozi_init_with_auto_registration()`
**Purpose**: Initialize with automatic API key registration
**Parameters**: None
**Returns**: `TodoziErrorCode`
**Complexity**: O(1)

### Task Management Functions

#### `todozi_create_task()`
**Purpose**: Create a new task with full specifications
**Parameters**:
- `const char* action`: Task description
- `TodoziPriority priority`: Priority level
- `const char* project`: Project name
- `const char* time_str`: Time specification
- `const char* context`: Additional context
- `Task** task`: Output parameter for created task

**Returns**: `TodoziErrorCode`
**Complexity**: O(n) where n is string lengths

```c
TodoziErrorCode todozi_create_task(const char* action, TodoziPriority priority, 
                                  const char* project, const char* time_str, 
                                  const char* context, Task** task) {
    // Full implementation with memory allocation and validation
}
```

#### `todozi_search_tasks()`
**Purpose**: Search tasks with optional semantic search
**Parameters**:
- `const char* query`: Search query
- `bool semantic`: Use semantic search if true
- `size_t limit`: Maximum results to return
- `Task*** tasks`: Output array of tasks
- `size_t* tasks_count`: Output count of tasks found

**Returns**: `TodoziErrorCode`
**Complexity**: O(n log n) for semantic search

### Memory Management Functions

#### `free_task()`
**Purpose**: Safely deallocate a Task structure and all its components
**Parameters**: `Task* task` - Task to free
**Complexity**: O(n) where n is number of tags/dependencies

```c
void free_task(Task* task) {
    if (task) {
        // Comprehensive memory cleanup
        if (task->id) free(task->id);
        if (task->tags) {
            for (size_t i = 0; i < task->tags_count; i++) {
                free(task->tags[i]);
            }
            free(task->tags);
        }
        // ... additional cleanup
        free(task);
    }
}
```

## Usage Examples

### Basic Task Creation
```c
#include "todozi.h"

int main() {
    TodoziErrorCode result;
    Task* task = NULL;
    
    // Initialize system
    result = todozi_init();
    if (result != TODOZI_OK) {
        printf("Initialization failed: %d\n", result);
        return 1;
    }
    
    // Create a task
    result = todozi_create_task(
        "Implement new feature", 
        TODOZI_PRIORITY_HIGH,
        "project-alpha",
        "ASAP",
        "This is critical for next release",
        &task
    );
    
    if (result == TODOZI_OK && task) {
        printf("Task created: %s\n", task->action);
        free_task(task);
    }
    
    return 0;
}
```

### Advanced Task Search with Filters
```c
void search_high_priority_tasks() {
    TaskFilters* filters = NULL;
    Task** tasks = NULL;
    size_t tasks_count = 0;
    
    // Create filters for high priority tasks in progress
    todozi_create_task_filters(
        "project-alpha",      // project
        "in_progress",        // status
        "high",               // priority
        NULL,                 // assignee (any)
        "urgent,critical",    // tags
        "feature",            // search text
        &filters
    );
    
    TodoziErrorCode result = todozi_search_with_filters(
        filters, 10, &tasks, &tasks_count
    );
    
    if (result == TODOZI_OK) {
        for (size_t i = 0; i < tasks_count; i++) {
            printf("Found task: %s\n", tasks[i]->action);
        }
        free_task_array(tasks, tasks_count);
    }
    
    free_task_filters(filters);
}
```

### Memory and Idea Management
```c
void manage_memories_and_ideas() {
    Task* memory_task = NULL;
    Task* idea_task = NULL;
    
    // Create a memory
    todozi_remember(
        "Team meeting discussion",
        "Important decision about architecture",
        &memory_task
    );
    
    // Create an idea
    todozi_ideate(
        "New algorithm optimization approach",
        &idea_task
    );
    
    // Cleanup
    if (memory_task) free_task(memory_task);
    if (idea_task) free_task(idea_task);
}
```

## Design Patterns

### 1. Factory Pattern
**Usage**: Task creation functions (`todozi_create_task`, `todozi_quick_task`)
**Benefits**: Centralized object creation with validation

### 2. Builder Pattern
**Usage**: Filter creation (`todozi_create_task_filters`)
**Benefits**: Flexible object construction with optional parameters

### 3. Repository Pattern
**Usage**: Data access through storage service
**Benefits**: Separation of data access from business logic

### 4. Strategy Pattern
**Usage**: Search algorithms (semantic vs keyword)
**Benefits**: Interchangeable algorithms at runtime

### 5. Observer Pattern
**Usage**: Task status updates and notifications
**Benefits**: Loose coupling between components

## Performance Analysis

### Time Complexity
| Operation | Best Case | Average Case | Worst Case |
|-----------|-----------|--------------|------------|
| Task Creation | O(1) | O(n) | O(n) |
| Task Search (Keyword) | O(1) | O(log n) | O(n) |
| Task Search (Semantic) | O(n) | O(n log n) | O(n²) |
| Memory Allocation | O(1) | O(1) | O(1) |

### Space Complexity
- **Task Structure**: O(n + m) where n=tags, m=dependencies
- **Search Operations**: O(k) where k=results count
- **Embedding Storage**: O(d) where d=embedding dimensions

### Memory Management Strategy
- **Allocation**: Dynamic allocation with error checking
- **Deallocation**: Comprehensive cleanup functions
- **Leak Prevention**: Reference counting for shared resources

## Security Considerations

### Input Validation
```c
TodoziErrorCode validate_task_input(const char* action, const char* time, 
                                   const char* priority, const char* project, 
                                   const char* status, const char* assignee, 
                                   const unsigned char* progress) {
    // Comprehensive input validation
    if (!action || strlen(action) == 0) {
        return TODOZI_ERR_INVALID_ARG;
    }
    // Additional validation checks...
}
```

### API Key Security
- UUID-based key generation
- Secure storage in user home directory
- Key expiration mechanisms

### Data Protection
- Sensitive data encryption
- Secure file permissions (600)
- Input sanitization for file paths

## Testing Strategies

### Unit Testing Framework
```c
// Example test case for task creation
void test_task_creation() {
    Task* task = NULL;
    TodoziErrorCode result;
    
    // Test valid input
    result = todozi_create_task("Test task", TODOZI_PRIORITY_MEDIUM, 
                               "test-project", "ASAP", NULL, &task);
    assert(result == TODOZI_OK);
    assert(task != NULL);
    assert(strcmp(task->action, "Test task") == 0);
    
    free_task(task);
    
    // Test invalid input
    result = todozi_create_task(NULL, TODOZI_PRIORITY_MEDIUM, 
                               NULL, NULL, NULL, &task);
    assert(result == TODOZI_ERR_INVALID_ARG);
}
```

### Integration Testing
- End-to-end workflow testing
- Cross-component interaction tests
- Performance benchmarking

### Security Testing
- Input validation tests
- Boundary condition tests
- Memory leak detection

## Deployment Instructions

### Build Requirements
```bash
# Dependencies
sudo apt-get install uuid-dev  # UUID library
gcc -std=c99 -Wall -Wextra -O2 todozi.c -o todozi -luuid
```

### Installation Steps
1. **Clone and Build**
   ```bash
   git clone https://github.com/todozi/todozi-c.git
   cd todozi-c
   make && sudo make install
   ```

2. **Configuration**
   ```bash
   mkdir -p ~/.todozi
   chmod 700 ~/.todozi
   ```

3. **API Key Setup**
   ```c
   char* api_key = NULL;
   todozi_get_tdz_api_key(&api_key);
   printf("Your API key: %s\n", api_key);
   free(api_key);
   ```

### System Integration
```c
// CMakeLists.txt example
cmake_minimum_required(VERSION 3.10)
project(MyTodoziApp)

find_library(UUID_LIB uuid)
add_executable(myapp main.c)
target_link_libraries(myapp ${UUID_LIB})
```

## Troubleshooting Guide

### Common Issues

#### Memory Leaks
**Symptoms**: Increasing memory usage over time
**Solution**: Use Valgrind for detection
```bash
valgrind --leak-check=full ./my_todozi_app
```

#### API Key Issues
**Symptoms**: Registration failures
**Solution**: Regenerate API key
```c
todozi_init_with_auto_registration();
```

#### Search Performance
**Symptoms**: Slow semantic searches
**Solution**: Limit results and use filters
```c
todozi_search_with_filters(filters, 50, &tasks, &tasks_count); // Limit to 50 results
```

### Error Codes Reference

| Error Code | Description | Resolution |
|------------|-------------|------------|
| `TODOZI_OK` | Success | No action needed |
| `TODOZI_ERR_NOT_IMPL` | Function not implemented | Check API version |
| `TODOZI_ERR_IO` | I/O operation failed | Check file permissions |
| `TODOZI_ERR_INVALID_ARG` | Invalid parameter | Validate input parameters |
| `TODOZI_ERR_MEMORY` | Memory allocation failed | Check system memory |

### Debugging Techniques
```c
// Enable verbose logging
TodoziErrorCode todozi_verbose(bool verbose, void** result) {
    // Implementation for debug output
}
```

## Error Handling

### Comprehensive Error Management
```c
TodoziErrorCode safe_task_operation(const char* action) {
    Task* task = NULL;
    TodoziErrorCode result;
    
    result = todozi_create_task(action, TODOZI_PRIORITY_MEDIUM, 
                               NULL, NULL, NULL, &task);
    
    if (result != TODOZI_OK) {
        // Handle specific error cases
        switch (result) {
            case TODOZI_ERR_MEMORY:
                fprintf(stderr, "Memory allocation failed\n");
                break;
            case TODOZI_ERR_INVALID_ARG:
                fprintf(stderr, "Invalid task parameters\n");
                break;
            default:
                fprintf(stderr, "Unknown error: %d\n", result);
        }
        return result;
    }
    
    // Process task...
    free_task(task);
    return TODOZI_OK;
}
```

### Recovery Strategies
- **Graceful degradation**: Fallback to simpler operations
- **Resource cleanup**: Automatic memory management
- **Error propagation**: Clear error codes and messages

This documentation provides a comprehensive reference for the TODOZI C API. The system is designed for robustness, scalability, and ease of use with thorough error handling and memory management practices.