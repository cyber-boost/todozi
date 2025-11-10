# Todozi - Comprehensive C Library Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [API Reference](#api-reference)
5. [Error Handling](#error-handling)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Usage Examples](#usage-examples)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive C library for managing tasks and projects in a collaborative environment. It supports AI/human collaboration, task dependencies, progress tracking, and enterprise-grade configuration management.

### Key Features
- **Task Management**: Create, update, and track tasks with comprehensive metadata
- **Project Organization**: Group tasks into projects with status tracking
- **AI/Human Collaboration**: Support for AI, human, and mixed assignees
- **Error Handling**: Comprehensive error reporting system
- **Memory Safety**: Robust memory management with cleanup functions
- **Extensibility**: Modular design for easy extension

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Application   │◄──►│   Todozi API    │◄──►│  Data Storage   │
│     Layer       │    │     Layer       │    │     Layer       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                      ┌───────┴───────┐
                      │               │
               ┌─────────────┐  ┌─────────────┐
               │  Core Types │  │  Utilities  │
               │   (Task,    │  │ (UUID, Hash,│
               │   Project)  │  │   Config)   │
               └─────────────┘  └─────────────┘
```

### Module Relationships
```
Task Management ───┬──► Project Management
                   │
                   └──► Configuration
                   │
                   └──► Registration
                   │
                   └──► Utilities (UUID, Hashing)
```

## Data Structures

### Enumerations

#### TodoziResult
```c
typedef enum {
    TODOZI_OK = 0,           // Operation successful
    TODOZI_ERR_ALLOC,        // Memory allocation failure
    TODOZI_ERR_INVALID,      // Invalid parameters
    TODOZI_ERR_NOT_FOUND,    // Resource not found
    TODOZI_ERR_INTERNAL      // Internal library error
} TodoziResult;
```

#### Priority Levels
```c
typedef enum {
    PRIORITY_LOW,        // Low priority task
    PRIORITY_MEDIUM,     // Medium priority (default)
    PRIORITY_HIGH,       // High priority
    PRIORITY_CRITICAL,   // Critical priority
    PRIORITY_URGENT      // Urgent priority (highest)
} Priority;
```

#### Status Types
```c
typedef enum {
    STATUS_TODO,         // Task is pending/not started
    STATUS_PENDING,      // Synonym for TODO
    STATUS_IN_PROGRESS,  // Task is being worked on
    STATUS_BLOCKED,      // Task is blocked
    STATUS_REVIEW,       // Task is under review
    STATUS_DONE,         // Task is completed
    STATUS_COMPLETED,    // Synonym for DONE
    STATUS_CANCELLED,    // Task was cancelled
    STATUS_DEFERRED      // Task was deferred
} Status;
```

#### Assignee Types
```c
typedef enum {
    ASSIGNEE_AI,             // AI-based assignee
    ASSIGNEE_HUMAN,          // Human assignee
    ASSIGNEE_COLLABORATIVE,  // Mixed AI/human collaboration
    ASSIGNEE_AGENT           // Specific agent (AI or human)
} AssigneeType;
```

### Core Structures

#### Task Structure
```c
struct Task {
    char* id;                    // Unique task identifier
    char* user_id;               // User who created the task
    char* action;                // Task description/action
    char* time;                  // Time string (format flexible)
    Priority priority;           // Task priority level
    char* parent_project;        // Parent project identifier
    Status status;               // Current task status
    AssigneeType assignee_type;  // Type of assignee
    char* assignee_agent_name;   // Specific agent name (if any)
    char** tags;                 // Array of tag strings
    size_t tags_count;           // Number of tags
    char** dependencies;         // Array of dependent task IDs
    size_t dependencies_count;   // Number of dependencies
    char* context_notes;         // Additional context/notes
    uint8_t* progress;           // Progress percentage (0-100)
    float* embedding_vector;     // AI embedding vector (future use)
    size_t embedding_vector_size;// Size of embedding vector
    time_t created_at;           // Creation timestamp
    time_t updated_at;           // Last update timestamp
};
```

#### TaskUpdate Structure
```c
struct TaskUpdate {
    char* action;                // Updated action string
    char* time;                  // Updated time string
    Priority* priority;          // Updated priority (nullable)
    char* parent_project;        // Updated parent project
    Status* status;              // Updated status (nullable)
    AssigneeType* assignee_type; // Updated assignee type (nullable)
    char* assignee_agent_name;   // Updated agent name
    char** tags;                 // Updated tags array
    size_t tags_count;           // Number of updated tags
    char** dependencies;         // Updated dependencies array
    size_t dependencies_count;   // Number of updated dependencies
    char* context_notes;         // Updated context notes
    uint8_t* progress;           // Updated progress (nullable)
    float* embedding_vector;     // Updated embedding vector
    size_t embedding_vector_size;// Size of updated embedding vector
};
```

## API Reference

### Error Handling Functions

#### `todozi_priority_parse`
```c
/**
 * Parse a string representation of priority to enum value
 * 
 * @param s String to parse ("low", "medium", "high", "critical", "urgent")
 * @param out Output parameter for parsed priority
 * @return TODOZI_OK on success, TODOZI_ERR_INVALID on failure
 */
TodoziResult todozi_priority_parse(const char* s, Priority* out);
```

#### `todozi_priority_to_string`
```c
/**
 * Convert priority enum to string representation
 * 
 * @param p Priority enum value
 * @param buf Buffer to store string (can be NULL for size calculation)
 * @param buflen Buffer length
 * @return Length of string (excluding null terminator)
 */
size_t todozi_priority_to_string(Priority p, char* buf, size_t buflen);
```

### Task Management Functions

#### `todozi_task_new`
```c
/**
 * Create a new task with basic parameters
 * 
 * @param user_id User identifier (required)
 * @param action Task description (required)
 * @param time_str Time string (required)
 * @param priority Task priority
 * @param parent_project Parent project ID (required)
 * @param status Initial status
 * @param out Output parameter for new task
 * @param err Error information (can be NULL)
 * @return TODOZI_OK on success, error code on failure
 */
TodoziResult todozi_task_new(const char* user_id, const char* action, const char* time_str,
                            Priority priority, const char* parent_project, Status status,
                            Task** out, TodoziError* err);
```

#### `todozi_task_new_full`
```c
/**
 * Create a new task with all available parameters
 * 
 * @param user_id User identifier (required)
 * @param action Task description (required)
 * @param time_str Time string (required)
 * @param priority Task priority
 * @param parent_project Parent project ID (required)
 * @param status Initial status
 * @param assignee_type Type of assignee
 * @param assignee_agent_name Specific agent name (for ASSIGNEE_AGENT)
 * @param tags Array of tag strings
 * @param tags_count Number of tags
 * @param dependencies Array of dependent task IDs
 * @param dependencies_count Number of dependencies
 * @param context_notes Additional context information
 * @param progress Progress percentage (0-100, can be NULL)
 * @param out Output parameter for new task
 * @param err Error information (can be NULL)
 * @return TODOZI_OK on success, error code on failure
 */
TodoziResult todozi_task_new_full(const char* user_id, const char* action, const char* time_str,
                                 Priority priority, const char* parent_project, Status status,
                                 AssigneeType assignee_type, const char* assignee_agent_name,
                                 char** tags, size_t tags_count, char** dependencies, size_t dependencies_count,
                                 const char* context_notes, const uint8_t* progress,
                                 Task** out, TodoziError* err);
```

#### `todozi_task_update`
```c
/**
 * Update a task with the provided changes
 * 
 * @param task Task to update (required)
 * @param updates Update structure containing changes
 * @param err Error information (can be NULL)
 * @return TODOZI_OK on success, error code on failure
 */
TodoziResult todozi_task_update(Task* task, const TaskUpdate* updates, TodoziError* err);
```

### Task Accessor Functions

All accessor functions follow the pattern:
- Return the requested field value
- Return NULL/0/default value if task is NULL
- Are thread-safe for read operations

Example:
```c
const char* todozi_task_id(const Task* task);
const char* todozi_task_action(const Task* task);
Priority todozi_task_priority(const Task* task);
```

### Project Management Functions

#### `todozi_project_new`
```c
/**
 * Create a new project
 * 
 * @param name Project name (required)
 * @param description Project description (can be NULL)
 * @param out Output parameter for new project
 * @param err Error information (can be NULL)
 * @return TODOZI_OK on success, error code on failure
 */
TodoziResult todozi_project_new(const char* name, const char* description, Project** out, TodoziError* err);
```

#### `todozi_project_add_task`
```c
/**
 * Add a task to a project
 * 
 * @param project Project to modify
 * @param task_id Task ID to add (duplicates are ignored)
 */
void todozi_project_add_task(Project* project, const char* task_id);
```

### Utility Functions

#### `todozi_generate_short_uuid`
```c
/**
 * Generate a short UUID prefixed with "task_"
 * 
 * @return Newly allocated string with UUID, NULL on failure
 */
char* todozi_generate_short_uuid(void);
```

#### `todozi_generate_sha256`
```c
/**
 * Generate SHA256 hash of input string
 * 
 * @param input String to hash
 * @return Newly allocated hex string of hash, NULL on failure
 */
char* todozi_generate_sha256(const char* input);
```

## Error Handling

### Error Structure
```c
typedef struct {
    TodoziResult code;    // Error code
    char *msg;           // Human-readable error message
} TodoziError;
```

### Error Handling Best Practices

1. **Always check return values**
```c
TodoziError err = {0};
Task* task = NULL;
TodoziResult result = todozi_task_new("user123", "Complete documentation", 
                                     "2024-01-15", PRIORITY_HIGH, 
                                     "project1", STATUS_TODO, &task, &err);
if (result != TODOZI_OK) {
    printf("Error: %s\n", err.msg);
    // Handle error appropriately
}
```

2. **Clean up error messages**
```c
// After handling error, free the message
free(err.msg);
```

## Design Patterns

### 1. Opaque Pointer Pattern
- All main structures (Task, Project, etc.) are forward declared
- Internal implementation details are hidden
- Provides encapsulation and API stability

### 2. Builder Pattern
- `TaskUpdate` structure acts as a builder for task modifications
- Allows incremental construction of complex updates

### 3. Factory Pattern
- Functions like `todozi_task_new()` act as factories
- Centralized object creation with validation

### 4. RAII-inspired Pattern
- Every `_new` function has a corresponding `_free` function
- Clear ownership semantics

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Task Creation | O(1) | Constant time allocation |
| Task Update | O(n) | n = number of tags/dependencies |
| Project Task Addition | O(n) | n = current task count (duplicate check) |
| Priority/Status Parsing | O(1) | String comparison |
| UUID Generation | O(1) | System UUID call |

### Memory Usage
- **Task Structure**: ~200-500 bytes base + variable for strings/arrays
- **Project Structure**: ~100 bytes base + variable for task IDs
- **String Arrays**: Each string allocation + array overhead

### Optimization Opportunities
1. **String Interning**: Reduce duplicate string allocations
2. **Array Pre-allocation**: Use growth factors for dynamic arrays
3. **Memory Pooling**: For frequent task creation/destruction

## Security Considerations

### Input Validation
- All string inputs are validated for NULL pointers
- Progress values are validated (0-100 range)
- Array bounds checking for tags/dependencies

### Memory Safety
- Comprehensive error handling for allocation failures
- All allocated memory is properly freed
- Buffer size checking in string operations

### Cryptographic Security
- Uses OpenSSL for SHA256/SHA512 hashing
- UUID generation uses system cryptographic RNG
- No sensitive data storage in plain text

### Best Practices
1. **Validate all external inputs**
2. **Use the library's error reporting system**
3. **Regularly free allocated memory**
4. **Sanitize user-provided strings before use**

## Testing Strategies

### Unit Testing Framework
```c
// Example test structure
typedef struct {
    const char* name;
    TodoziResult (*test_func)(void);
} TestCase;

// Example test case
TodoziResult test_task_creation() {
    Task* task = NULL;
    TodoziError err = {0};
    TodoziResult result = todozi_task_new("test_user", "test action", 
                                         "now", PRIORITY_MEDIUM, 
                                         "test_project", STATUS_TODO, 
                                         &task, &err);
    
    if (result == TODOZI_OK) {
        todozi_task_free(task);
        return TODOZI_OK;
    } else {
        free(err.msg);
        return result;
    }
}
```

### Test Categories
1. **Unit Tests**: Individual function testing
2. **Integration Tests**: Multi-function workflow testing
3. **Memory Tests**: Allocation/failure testing
4. **Boundary Tests**: Edge case testing

### Test Coverage Goals
- 90%+ line coverage
- All error paths tested
- Memory leak detection
- Thread safety testing (where applicable)

## Deployment Instructions

### Build Dependencies
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev uuid-dev

# CentOS/RHEL
sudo yum install openssl-devel libuuid-devel

# macOS
brew install openssl ossp-uuid
```

### Compilation Flags
```makefile
CFLAGS = -Wall -Wextra -Werror -O2 -std=c99
LDFLAGS = -luuid -lcrypto
```

### Integration Steps
1. **Include the header**: `#include "todozi.h"`
2. **Link with dependencies**: `-luuid -lcrypto`
3. **Initialize structures**: Use provided factory functions
4. **Error handling**: Implement comprehensive error checking

### Cross-Platform Considerations
- Tested on Linux, macOS, Windows (with appropriate UUID library)
- OpenSSL dependency requires version 1.1.1 or newer
- UUID library compatibility verified

## Usage Examples

### Basic Task Creation
```c
#include "todozi.h"
#include <stdio.h>

int main() {
    TodoziError err = {0};
    Task* task = NULL;
    
    // Create a basic task
    TodoziResult result = todozi_task_new(
        "user123", 
        "Write project documentation", 
        "2024-01-15 10:00", 
        PRIORITY_HIGH, 
        "doc_project", 
        STATUS_TODO, 
        &task, 
        &err
    );
    
    if (result == TODOZI_OK) {
        printf("Task created: %s\n", todozi_task_id(task));
        printf("Action: %s\n", todozi_task_action(task));
        
        // Clean up
        todozi_task_free(task);
    } else {
        printf("Error: %s (code: %d)\n", err.msg, err.code);
        free(err.msg);
    }
    
    return 0;
}
```

### Advanced Task with Tags and Dependencies
```c
// Create tags array
char* tags[] = {"documentation", "priority", "urgent"};
size_t tags_count = 3;

// Create dependencies array  
char* deps[] = {"task_abc123", "task_def456"};
size_t deps_count = 2;

TodoziResult result = todozi_task_new_full(
    "user123",
    "Complete API documentation",
    "2024-01-20",
    PRIORITY_URGENT,
    "api_project", 
    STATUS_IN_PROGRESS,
    ASSIGNEE_COLLABORATIVE,
    "doc_team",
    tags, tags_count,
    deps, deps_count,
    "This is critical for API launch",
    NULL, // progress (NULL for default)
    &task,
    &err
);
```

### Task Update Example
```c
// Create update structure
TaskUpdate* update = NULL;
todozi_task_update_new(&update, &err);

// Set update fields
todozi_task_update_with_status(update, STATUS_IN_PROGRESS, &err);
todozi_task_update_with_progress(update, 50, &err);

// Apply update
todozi_task_update(task, update, &err);

// Clean up
todozi_task_update_free(update);
```

### Project Management
```c
// Create project
Project* project = NULL;
todozi_project_new("API Development", "Backend API implementation", &project, &err);

// Add tasks to project
todozi_project_add_task(project, "task_123456");
todozi_project_add_task(project, "task_789012");

// Archive project when done
todozi_project_archive(project);
```

## Troubleshooting Guide

### Common Issues

#### 1. Memory Allocation Failures
**Symptoms**: Functions return `TODOZI_ERR_ALLOC`
**Solution**: Check system memory, implement graceful degradation

#### 2. Invalid Parameter Errors
**Symptoms**: Functions return `TODOZI_ERR_INVALID`
**Solution**: Validate inputs before calling library functions

#### 3. UUID Generation Failures
**Symptoms**: `todozi_generate_short_uuid()` returns NULL
**Solution**: Ensure uuid-dev package is installed and accessible

#### 4. OpenSSL Issues
**Symptoms**: Linker errors or hash generation failures
**Solution**: Verify OpenSSL installation and library paths

### Debugging Techniques

#### Memory Leak Detection
```c
// Use valgrind or similar tools
valgrind --leak-check=full ./your_application
```

#### Error Tracing
```c
// Enable detailed error logging
TodoziError err = {0};
TodoziResult result = some_function(..., &err);
if (result != TODOZI_OK) {
    fprintf(stderr, "Error at %s:%d - %s\n", __FILE__, __LINE__, err.msg);
    free(err.msg);
}
```

### Performance Monitoring

#### Memory Usage Tracking
```c
// Implement custom memory tracking if needed
#ifdef DEBUG
#define todozi_malloc(sz) _debug_malloc(sz, __FILE__, __LINE__)
#else
#define todozi_malloc(sz) malloc(sz)
#endif
```

This comprehensive documentation covers all aspects of the Todozi C library, providing developers with everything needed to effectively use, extend, and maintain the codebase.