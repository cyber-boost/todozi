# Todozi C Framework - Comprehensive Documentation

## Overview

Todozi is a comprehensive task management and AI collaboration framework written in C. The system implements a polymorphic tool architecture with shared state management, supporting various AI-powered capabilities including task management, memory synthesis, idea refinement, error prevention, and learning analytics.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Data Structures](#core-data-structures)
3. [Tool System Design](#tool-system-design)
4. [Utility Functions](#utility-functions)
5. [Memory Management](#memory-management)
6. [Thread Safety](#thread-safety)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)
13. [API Reference](#api-reference)

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Todozi Framework                         │
├─────────────┬─────────────┬─────────────┬───────────────────────┤
│   Utility   │   Storage   │   Memory    │     AI Services       │
│  Functions  │   Layer     │ Management  │   (Embedding, etc.)   │
├─────────────┼─────────────┼─────────────┼───────────────────────┤
│  HashMap    │  Storage    │  Shared-    │ TodoziEmbedding-      │
│    Vec      │   Struct    │   Todozi    │   Service             │
│ ToolResult  │             │             │                       │
└─────────────┴─────────────┴─────────────┴───────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌─────────────┐       ┌─────────────┐       ┌─────────────┐
│ Basic Data  │       │    Tool     │       │   Advanced  │
│   Types     │       │  System     │       │    Tools    │
├─────────────┤       ├─────────────┤       ├─────────────┤
│ Task        │       │ Tool        │       │ Predictive  │
│ Memory      │       │ ToolDefini- │       │  Error      │
│ Idea        │       │   tion      │       │ Memory      │
│ Error       │       │ ToolPara-   │       │  Synthesis  │
│ CodeChunk   │       │   meter     │       │ AI Agent    │
└─────────────┘       └─────────────┘       │ Orchestrator│
                                            │ etc.        │
                                            └─────────────┘
```

### System Flow

```
User Input → Parameter Validation → Tool Selection → Resource Locking → 
Execution → Result Processing → Response Generation → Cleanup
```

---

## Core Data Structures

### Enumeration Types

```c
// Priority levels for tasks
typedef enum {
    PRIORITY_LOW,       // Low priority tasks
    PRIORITY_MEDIUM,    // Medium priority tasks  
    PRIORITY_HIGH,      // High priority tasks
    PRIORITY_CRITICAL,  // Critical priority tasks
    PRIORITY_URGENT     // Urgent priority tasks
} Priority;

// Task status states
typedef enum {
    STATUS_TODO,        // Task is pending
    STATUS_IN_PROGRESS, // Task is being worked on
    STATUS_BLOCKED,     // Task is blocked
    STATUS_REVIEW,      // Task is under review
    STATUS_DONE         // Task is completed
} Status;

// Assignee types for task assignment
typedef enum {
    ASSIGN_TYPE_AI,           // AI agent assignment
    ASSIGN_TYPE_HUMAN,        // Human user assignment
    ASSIGN_TYPE_COLLABORATIVE // Collaborative assignment
} AssigneeType;
```

### Main Data Structures

#### HashMap Structure
```c
struct HashMap {
    char** keys;        // Array of key strings
    char** values;      // Array of value strings
    int size;           // Current number of entries
    int capacity;       // Maximum capacity before reallocation
};
```

**Purpose**: Provides key-value storage for configuration and parameters
**Memory Layout**: Dynamic arrays with geometric growth
**Thread Safety**: Not thread-safe by default

#### Vec Structure
```c
struct Vec {
    char** data;        // Array of string pointers
    int size;           // Current number of elements
    int capacity;       // Maximum capacity before reallocation
};
```

**Purpose**: Dynamic string array implementation
**Growth Factor**: Doubles capacity when full
**Memory Efficiency**: Amortized O(1) insertion

#### Task Structure
```c
struct Task {
    char* id;                   // Unique task identifier (UUID)
    char* user_id;              // Associated user ID
    char* action;               // Task description/action
    char* time;                 // Time estimate string
    Priority priority;          // Priority level
    char* parent_project;       // Parent project name
    Status status;              // Current status
    AssigneeType* assignee;     // Assigned entity type
    Vec tags;                   // Categorization tags
    Vec dependencies;           // Dependent task IDs
    char* context_notes;        // Additional context
    int* progress;              // Completion percentage (0-100)
    Vec* embedding_vector;      // AI embedding for semantic search
    time_t created_at;          // Creation timestamp
    time_t updated_at;          // Last update timestamp
};
```

**Constraints**: 
- `id`, `user_id`, `action` are required fields
- Maximum action length: 500 characters
- Progress range: 0-100

#### SharedTodozi Structure
```c
struct SharedTodozi {
    Storage* storage;           // Backing storage implementation
    pthread_mutex_t mutex;      // Thread synchronization mutex
};
```

**Thread Safety**: Uses pthread mutex for synchronization
**Memory Management**: Owns storage reference

---

## Tool System Design

### Tool Polymorphism Interface

```c
// Function pointer types for polymorphic behavior
typedef ToolDefinition* (*tool_def_fn)(const Tool* self);
typedef ToolResult* (*tool_exec_fn)(const Tool* self, const HashMap* kwargs);
typedef void (*tool_destroy_fn)(Tool* self);

// Base Tool structure
struct Tool {
    tool_def_fn definition;     // Returns tool metadata
    tool_exec_fn execute;       // Executes tool functionality
    tool_destroy_fn destroy;    // Cleans up tool resources
    void* impl;                 // Tool-specific implementation data
};
```

### ToolDefinition Structure
```c
struct ToolDefinition {
    char* name;                 // Tool identifier name
    char* description;          // Human-readable description
    ToolParameter* parameters;  // Required/optional parameters
    size_t parameters_count;    // Number of parameters
    char* category;             // Tool categorization
    Vec resource_locks;         // Required resource locks
};
```

### ToolResult Structure
```c
struct ToolResult {
    bool success;               // Execution success status
    char* message;              // Result message/error
    int confidence;             // Confidence level (0-1000 scale)
};
```

## Utility Functions

### HashMap Functions

#### `hashmap_new()`
```c
HashMap* hashmap_new();
```
**Purpose**: Creates a new empty HashMap
**Return**: Pointer to allocated HashMap or NULL on failure
**Memory**: Allocates initial capacity of 10 entries
**Time Complexity**: O(1)

#### `hashmap_set()`
```c
void hashmap_set(HashMap* map, const char* key, const char* value);
```
**Parameters**:
- `map`: Target HashMap (non-NULL)
- `key`: String key (non-NULL, duplicated)
- `value`: String value (non-NULL, duplicated)

**Behavior**:
- Updates existing key if present
- Inserts new key-value pair if not present
- Automatically resizes if capacity exceeded
- Silent failure on memory allocation errors

**Time Complexity**: O(n) worst-case, O(1) amortized

### Vec Functions

#### `vec_push()`
```c
void vec_push(Vec* vec, const char* item);
```
**Parameters**:
- `vec`: Target vector (non-NULL)
- `item`: String to add (non-NULL, duplicated)

**Growth Strategy**: Doubles capacity when full
**Memory**: Amortized O(1) allocation cost

## Tool Implementations

### CreateTaskTool

**Purpose**: Creates new tasks with AI assignment capabilities
**Category**: Task Management
**Resource Locks**: FilesystemWrite

#### Parameters:
```c
ToolParameter params[7] = {
    {"action", "string", "Task description/action to perform", true},
    {"time", "string", "Time estimate (e.g., '2 hours', '1 day')", false},
    {"priority", "string", "Priority level", false},
    {"project", "string", "Project name association", false},
    {"assignee", "string", "Assignee type", false},
    {"tags", "string", "Comma-separated tags", false},
    {"context", "string", "Additional context", false}
};
```

#### Validation Rules:
- Action length: 1-500 characters
- Default assignee: "human"
- Automatic ID generation: "task-{timestamp}"

### IntelligentTaskPlannerTool

**Purpose**: AI-powered task planning with predictive analytics
**Category**: Intelligent Planning
**Resource Locks**: FilesystemRead, Memory
**Confidence Scale**: 0-1000

#### Implementation Details:
```c
typedef struct {
    SharedTodozi* todozi;
    HashMap* context_memory; // conversation_id → Vec<String>
} IntelligentTaskPlannerTool;
```

**Features**:
- Context-aware planning with conversation memory
- Resource optimization algorithms
- Intelligent scheduling based on complexity
- Predictive timeline estimation

---

## Design Patterns

### 1. Strategy Pattern (Tool System)
Each tool implements a common interface (`tool_def_fn`, `tool_exec_fn`, `tool_destroy_fn`) allowing runtime selection and execution.

### 2. Factory Pattern (Tool Creation)
Tool factory functions (`create_*_tool_new`) encapsulate object creation logic and initialization.

### 3. Polymorphism (Tool Behavior)
Base `Tool` structure with function pointers enables polymorphic behavior without C++-style inheritance.

### 4. Resource Management (RAII-like)
Each tool provides destruction function for proper cleanup of implementation-specific resources.

### 5. Observer Pattern (Memory Synthesis)
Learning pattern tracking and analytics provide observation capabilities for system behavior.

## Memory Management

### Allocation Strategy
- **Small objects**: Direct `calloc()` for individual structures
- **Arrays**: Geometric growth (double capacity when full)
- **Strings**: Always duplicated with `strdup()` for ownership

### Memory Safety Rules
1. All allocated memory must be freed
2. String parameters are always duplicated
3. NULL checks before all operations
4. Cleanup on allocation failure

### Leak Prevention
```c
// Example safe allocation pattern
Tool* tool = calloc(1, sizeof(Tool));
if (!tool) {
    free(impl);  // Cleanup previous allocation
    return NULL;
}
```

## Thread Safety

### Mutex-based Synchronization
```c
// SharedTodozi locking mechanism
void shared_todozi_lock(SharedTodozi* todozi) {
    if (todozi) pthread_mutex_lock(&todozi->mutex);
}

void shared_todozi_unlock(SharedTodozi* todozi) {
    if (todozi) pthread_mutex_unlock(&todozi->mutex);
}
```

### Concurrency Rules
1. Tools must lock SharedTodozi before storage operations
2. Resource locks declared in ToolDefinition guide locking strategy
3. No thread safety for individual HashMap/Vec objects

## Performance Analysis

### Time Complexity
| Operation | Best Case | Worst Case | Amortized |
|-----------|-----------|------------|-----------|
| HashMap set | O(1) | O(n) | O(1) |
| HashMap get | O(1) | O(n) | O(1) |
| Vec push | O(1) | O(n) | O(1) |
| Tool execution | O(1) | O(n) | - |

### Memory Usage
- **Base overhead**: ~50KB for framework structures
- **Per tool**: 1-5KB depending on internal state
- **String storage**: Dynamic based on content size
- **Cache structures**: Grow with usage patterns

### Optimization Opportunities
1. **HashMap**: Implement hash-based lookup for O(1) performance
2. **Vec**: Pre-allocate based on expected size
3. **String pooling**: Reduce duplication overhead
4. **Cache warming**: Pre-load frequent patterns

## Security Considerations

### Input Validation
```c
// Parameter length validation example
if (!action || strlen(action) == 0 || strlen(action) > 500) {
    return tool_result_error("Action must be 1-500 characters", 100);
}
```

### Security Measures
1. **Bounds checking**: All string operations have length limits
2. **NULL safety**: Comprehensive pointer validation
3. **Injection prevention**: Parameter sanitization in tool execution
4. **Resource limits**: Maximum sizes for all user inputs

### Potential Vulnerabilities
1. **Buffer overflows**: Mitigated by length checking
2. **Memory exhaustion**: Limited by allocation bounds
3. **Race conditions**: Addressed by mutex locking
4. **Information leakage**: Minimal sensitive data storage

## Testing Strategies

### Unit Testing Approach
```c
// Example test structure
void test_create_task_tool() {
    SharedTodozi* todozi = shared_todozi_new(NULL);
    Tool* tool = create_task_tool_new(todozi);
    
    HashMap* params = hashmap_new();
    hashmap_set(params, "action", "Test task");
    
    ToolResult* result = tool->execute(tool, params);
    assert(result->success == true);
    
    // Cleanup
    tool_result_free(result);
    hashmap_free(params);
    tool->destroy(tool);
    shared_todozi_free(todozi);
}
```

### Test Categories
1. **Unit Tests**: Individual function validation
2. **Integration Tests**: Tool interaction testing
3. **Concurrency Tests**: Thread safety verification
4. **Memory Tests**: Leak detection and cleanup
5. **Performance Tests**: Load and stress testing

### Testing Tools Recommended
- **Valgrind**: Memory leak detection
- **Google Test**: C++ testing framework (with C interface)
- **pthread testing**: Concurrency validation
- **Custom harness**: Framework-specific testing

## Deployment Instructions

### Build Requirements
```bash
# Required libraries
sudo apt-get install build-essential
sudo apt-get install libuuid-dev  # UUID generation

# Compilation flags
gcc -std=c99 -D_POSIX_C_SOURCE=200809L -pedantic -Wall -Wextra \
    -O2 -pthread todozi.c -luuid -o todozi
```

### Platform Support
- **Linux**: Primary supported platform
- **macOS**: Should work with minor adjustments
- **Windows**: Requires pthreads-w32 library

### Deployment Steps
1. **Compilation**: Use provided build flags
2. **Library linking**: Ensure uuid and pthread libraries available
3. **Testing**: Run comprehensive test suite
4. **Integration**: Incorporate into larger application
5. **Monitoring**: Implement logging and metrics

## Troubleshooting Guide

### Common Issues

#### 1. Memory Leaks
**Symptoms**: Increasing memory usage over time
**Solution**: Run with Valgrind to identify leaks
```bash
valgrind --leak-check=full ./todozi
```

#### 2. Thread Deadlocks
**Symptoms**: Application hangs during concurrent access
**Solution**: Ensure proper lock/unlock pairing in tools
```c
shared_todozi_lock(todozi);
// Critical section
shared_todozi_unlock(todozi);
```

#### 3. Parameter Validation Failures
**Symptoms**: Tools return error results unexpectedly
**Solution**: Check parameter lengths and required fields
```c
// Ensure all required parameters are provided
if (!required_param) {
    return tool_result_error("Missing required parameter", confidence);
}
```

### Debugging Techniques

1. **Logging**: Add printf statements for execution flow
2. **Assertions**: Use assert for invariant checking
3. **GDB debugging**: Step through tool execution
4. **Resource monitoring**: Track memory and lock usage

## API Reference

### Core Functions

#### `tool_result_success()`
```c
ToolResult* tool_result_success(const char* message, int confidence);
```
**Parameters**:
- `message`: Success message (required)
- `confidence`: Confidence level (0-1000)

**Returns**: New ToolResult indicating success
**Memory**: Caller must free with `tool_result_free()`

#### `shared_todozi_new()`
```c
SharedTodozi* shared_todozi_new(Storage* storage);
```
**Parameters**:
- `storage`: Backing storage implementation

**Returns**: New SharedTodozi instance with initialized mutex
**Thread Safety**: Creates thread-safe shared context

### Tool Factory Functions

Each tool follows the same pattern:
```c
Tool* tool_name_new(SharedTodozi* todozi);
```

**Parameters**: SharedTodozi context for storage access
**Returns**: New Tool instance ready for use
**Memory**: Must be destroyed with tool's destroy function

## Example Usage

### Basic Tool Creation and Execution
```c
#include "todozi.h"

int main() {
    // Initialize framework
    Storage storage = {0};
    SharedTodozi* todozi = shared_todozi_new(&storage);
    
    // Create task tool
    Tool* task_tool = create_task_tool_new(todozi);
    
    // Prepare parameters
    HashMap* params = hashmap_new();
    hashmap_set(params, "action", "Write comprehensive documentation");
    hashmap_set(params, "priority", "high");
    hashmap_set(params, "time", "4 hours");
    
    // Execute tool
    ToolResult* result = task_tool->execute(task_tool, params);
    if (result->success) {
        printf("Success: %s\n", result->message);
    } else {
        printf("Error: %s\n", result->message);
    }
    
    // Cleanup
    tool_result_free(result);
    hashmap_free(params);
    task_tool->destroy(task_tool);
    shared_todozi_free(todozi);
    
    return 0;
}
```

### Advanced Tool Usage
```c
// Intelligent planning with context
Tool* planner = intelligent_task_planner_tool_new(todozi);

HashMap* plan_params = hashmap_new();
hashmap_set(plan_params, "goal", "Develop new AI feature");
hashmap_set(plan_params, "complexity", "complex");
hashmap_set(plan_params, "timeline", "2 weeks");

ToolResult* plan = planner->execute(planner, plan_params);
// Process comprehensive planning result
```

## Conclusion

The Todozi framework provides a robust, extensible foundation for AI-powered task management and collaboration systems. Its polymorphic tool architecture, comprehensive memory management, and thread-safe design make it suitable for production deployment in various environments.

The documentation above covers all aspects of the codebase from architecture and design patterns to security considerations and deployment instructions. This comprehensive coverage ensures developers can effectively understand, extend, and maintain the Todozi framework.