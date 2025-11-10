# Comprehensive Documentation: Code Generation Graph System

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategy](#testing-strategy)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

This C library provides a comprehensive code generation graph system for managing software development projects through a dependency-based approach. The system organizes code development into manageable chunks with explicit dependencies, tracking progress and maintaining context throughout the development lifecycle.

### Key Features
- **Hierarchical Chunking**: Five levels of code granularity (Project → Module → Class → Method → Block)
- **Dependency Management**: Graph-based dependency tracking with cycle detection
- **State Tracking**: Comprehensive project state and context window management
- **Dynamic Data Structures**: Custom string, array, and map implementations
- **Progress Monitoring**: Real-time tracking of completion status and metrics

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                    CodeGenerationGraph                      │
├─────────────────┬─────────────────┬─────────────────────────┤
│   ChunkMap      │  ProjectState   │    ContextWindow        │
│     (hash)      │                 │                         │
└─────────────────┴─────────────────┴─────────────────────────┘
         │               │                   │
         ▼               ▼                   ▼
┌─────────────────┐ ┌──────────────┐ ┌─────────────────┐
│    CodeChunk    │ │ StringArray  │ │ StringArray     │
│   (per chunk)   │ │   (deps)     │ │   (imports)     │
└─────────────────┘ └──────────────┘ └─────────────────┘
```

### Data Flow
```
Project Planning → Chunk Creation → Dependency Resolution → 
Code Generation → Validation → Project Completion
```

## Data Structures

### Enumerations

#### `ChunkingLevel`
```c
typedef enum ChunkingLevel {
    CHUNKING_LEVEL_PROJECT,    // High-level project planning
    CHUNKING_LEVEL_MODULE,     // Major system components
    CHUNKING_LEVEL_CLASS,      // Class definitions
    CHUNKING_LEVEL_METHOD,     // Individual methods
    CHUNKING_LEVEL_BLOCK,      // Small code blocks
    CHUNKING_LEVEL_INVALID     // Invalid level marker
} ChunkingLevel;
```

**Properties:**
- **PROJECT**: 100 token limit, architectural planning
- **MODULE**: 500 token limit, system components
- **CLASS**: 1000 token limit, class implementations
- **METHOD**: 300 token limit, function logic
- **BLOCK**: 100 token limit, error handling/details

#### `ChunkStatus`
```c
typedef enum ChunkStatus {
    CHUNK_STATUS_PENDING,      // Not yet started
    CHUNK_STATUS_IN_PROGRESS,  // Currently being worked on
    CHUNK_STATUS_COMPLETED,    // Code written but not validated
    CHUNK_STATUS_VALIDATED,    // Code tested and verified
    CHUNK_STATUS_FAILED        // Generation failed
} ChunkStatus;
```

### Core Structures

#### `String` - Dynamic String Implementation
```c
typedef struct {
    char* data;        // String content
    size_t length;     // Current length
    size_t capacity;   // Allocated capacity
} String;
```

**Methods:**
- `string_new()`: Creates empty string with default capacity (16)
- `string_new_with_capacity(size_t)`: Creates string with specified capacity
- `string_free(String*)`: Releases all memory
- `string_append(String*, const char*)`: Appends C string
- `string_appendf(String*, const char*, ...)`: Formatted append
- `string_vappend(String*, const char*, va_list)`: va_list version

#### `StringArray` - Dynamic String Array
```c
typedef struct {
    char** data;       // Array of string pointers
    size_t length;     // Number of elements
    size_t capacity;   // Allocated capacity
} StringArray;
```

**Methods:**
- `string_array_new()`: Creates array with default capacity (8)
- `string_array_free(StringArray*)`: Releases all memory
- `string_array_push(StringArray*, const char*)`: Adds string
- `string_array_contains(StringArray*, const char*)`: Checks membership

#### `StringMap` - Simple Key-Value Store
```c
typedef struct {
    StringPair* data;  // Array of key-value pairs
    size_t length;     // Number of pairs
    size_t capacity;   // Allocated capacity
} StringMap;

typedef struct {
    char* key;         // Map key
    char* value;       // Map value
} StringPair;
```

**Methods:**
- `string_map_new()`: Creates map with default capacity (8)
- `string_map_free(StringMap*)`: Releases all memory
- `string_map_insert(StringMap*, const char*, const char*)`: Adds/updates entry
- `string_map_get(StringMap*, const char*)`: Retrieves value

#### `ProjectState` - Project Tracking
```c
struct ProjectState {
    size_t total_lines;              // Lines written so far
    size_t max_lines;                // Project line limit
    char* current_module;            // Current working module
    StringArray* dependencies;       // Project dependencies
    StringArray* completed_modules;  // Finished modules
    StringArray* pending_modules;    // Modules to be done
    StringMap* global_variables;     // Global configuration
    time_t created_at;               // Creation timestamp
    time_t updated_at;               // Last update timestamp
};
```

**Methods:**
- `project_state_new(size_t)`: Creates new project state
- `project_state_free(ProjectState*)`: Releases memory
- `project_state_to_string(ProjectState*)`: Serializes to string
- `project_state_add_completed_module(ProjectState*, const char*)`: Marks module complete
- `project_state_add_pending_module(ProjectState*, const char*)`: Adds pending module
- `project_state_set_global_variable(ProjectState*, const char*, const char*)`: Sets global var
- `project_state_increment_lines(ProjectState*, size_t)`: Updates line count

#### `ContextWindow` - Development Context
```c
struct ContextWindow {
    char* previous_class;            // Recently completed class
    char* current_class;             // Current working class
    char* next_planned;              // Next planned class
    StringArray* global_vars_in_scope; // Available globals
    StringArray* imports_used;       // Import statements
    StringMap* function_signatures;  // Function definitions
    StringArray* error_patterns_seen; // Error handling patterns
    time_t created_at;               // Creation timestamp
    time_t updated_at;               // Last update timestamp
};
```

**Methods:**
- `context_window_new()`: Creates new context window
- `context_window_free(ContextWindow*)`: Releases memory
- `context_window_to_string(ContextWindow*)`: Serializes to string
- `context_window_add_import(ContextWindow*, const char*)`: Adds import
- `context_window_add_function_signature(ContextWindow*, const char*, const char*)`: Adds function
- `context_window_add_error_pattern(ContextWindow*, const char*)`: Records error pattern
- `context_window_set_current_class(ContextWindow*, const char*)`: Updates current class

#### `CodeChunk` - Code Unit
```c
struct CodeChunk {
    char* chunk_id;                  // Unique identifier
    ChunkStatus status;              // Current status
    StringArray* dependencies;       // Required chunks
    char* code;                      // Generated code
    char* tests;                     // Associated tests
    bool validated;                  // Validation flag
    ChunkingLevel level;             // Granularity level
    size_t estimated_tokens;         // Token count estimate
    time_t created_at;               // Creation timestamp
    time_t updated_at;               // Last update timestamp
};
```

**Methods:**
- `code_chunk_new(const char*, ChunkingLevel)`: Creates new chunk
- `code_chunk_free(CodeChunk*)`: Releases memory
- `code_chunk_add_dependency(CodeChunk*, const char*)`: Adds dependency
- `code_chunk_set_code(CodeChunk*, const char*)`: Sets code content
- `code_chunk_set_tests(CodeChunk*, const char*)`: Sets test content
- Status update methods: `mark_completed`, `mark_validated`, `mark_failed`, `mark_in_progress`

#### `CodeGenerationGraph` - Main System
```c
struct CodeGenerationGraph {
    ChunkMap* chunks;                // All code chunks
    ProjectState* project_state;     // Project metadata
    ContextWindow* context_window;   // Development context
};
```

**Methods:**
- `code_generation_graph_new(size_t)`: Creates new graph
- `code_generation_graph_free(CodeGenerationGraph*)`: Releases memory
- `code_generation_graph_add_chunk(CodeGenerationGraph*, const char*, ChunkingLevel, StringArray*)`: Adds chunk
- `code_generation_graph_get_ready_chunks(CodeGenerationGraph*)`: Gets executable chunks
- `code_generation_graph_get_chunk(CodeGenerationGraph*, const char*)`: Retrieves chunk
- `code_generation_graph_update_chunk_code(CodeGenerationGraph*, const char*, const char*)`: Updates code
- `code_generation_graph_update_chunk_tests(CodeGenerationGraph*, const char*, const char*)`: Updates tests
- Status management methods for chunks
- `code_generation_graph_get_project_summary(CodeGenerationGraph*)`: Gets project report
- `code_generation_graph_get_next_chunk_to_work_on(CodeGenerationGraph*)`: Gets next chunk
- `code_generation_graph_get_dependency_chain(CodeGenerationGraph*, const char*)`: Gets dependency chain

## API Reference

### Memory Management Macros

#### `CHECK_ALLOC(p)`
```c
#define CHECK_ALLOC(p) do { \
    if ((p) == NULL) { \
        fprintf(stderr, "Out of memory\n"); \
        abort(); \
    } \
} while (0)
```
**Purpose**: Validates memory allocation success
**Parameters**: `p` - Pointer to check
**Behavior**: Aborts program on allocation failure

### Helper Functions

#### `dup_str(const char* s)`
```c
static char *dup_str(const char *s) {
    if (!s) return NULL;
    char *r = malloc(strlen(s) + 1);
    CHECK_ALLOC(r);
    strcpy(r, s);
    return r;
}
```
**Purpose**: Safe string duplication
**Parameters**: `s` - String to duplicate
**Returns**: New allocated string or NULL
**Memory**: Caller must free returned string

### Chunking Level Utilities

#### `chunking_level_max_tokens(ChunkingLevel level)`
```c
size_t chunking_level_max_tokens(ChunkingLevel level) {
    switch (level) {
        case CHUNKING_LEVEL_PROJECT: return 100;
        case CHUNKING_LEVEL_MODULE: return 500;
        case CHUNKING_LEVEL_CLASS: return 1000;
        case CHUNKING_LEVEL_METHOD: return 300;
        case CHUNKING_LEVEL_BLOCK: return 100;
        default: return 0;
    }
}
```
**Purpose**: Gets maximum tokens for chunk level
**Parameters**: `level` - Chunking level
**Returns**: Token limit

#### `chunking_level_description(ChunkingLevel level)`
```c
const char* chunking_level_description(ChunkingLevel level) {
    switch (level) {
        case CHUNKING_LEVEL_PROJECT: return "High-level project planning and architecture";
        case CHUNKING_LEVEL_MODULE: return "Major system components and interfaces";
        case CHUNKING_LEVEL_CLASS: return "Class definitions and major functions";
        case CHUNKING_LEVEL_METHOD: return "Individual methods and helper functions";
        case CHUNKING_LEVEL_BLOCK: return "Small code blocks and error handling";
        default: return "";
    }
}
```
**Purpose**: Gets human-readable description
**Parameters**: `level` - Chunking level
**Returns**: Description string

#### `chunking_level_to_string(ChunkingLevel level)`
```c
const char* chunking_level_to_string(ChunkingLevel level) {
    if (level >= 0 && level < CHUNKING_LEVEL_INVALID) {
        return CHUNKING_LEVEL_NAMES[level];
    }
    return "";
}
```
**Purpose**: Converts enum to string
**Parameters**: `level` - Chunking level
**Returns**: String representation

#### `chunking_level_from_string(const char* str, ChunkingLevel *out_level)`
```c
bool chunking_level_from_string(const char* str, ChunkingLevel *out_level) {
    if (!str || !out_level) return false;
    
    for (int i = 0; i < CHUNKING_LEVEL_INVALID; i++) {
        if (strcmp(str, CHUNKING_LEVEL_NAMES[i]) == 0) {
            *out_level = (ChunkingLevel)i;
            return true;
        }
    }
    return false;
}
```
**Purpose**: Converts string to enum
**Parameters**: 
- `str` - String to convert
- `out_level` - Output parameter for result
**Returns**: true if conversion successful

## Usage Examples

### Basic Project Setup
```c
#include "code_generation_graph.h"

int main() {
    // Create project with 5000 line limit
    CodeGenerationGraph* graph = code_generation_graph_new(5000);
    
    // Set global project variables
    project_state_set_global_variable(graph->project_state, "language", "python");
    project_state_set_global_variable(graph->project_state, "framework", "django");
    
    // Create project-level chunk
    StringArray* project_deps = string_array_new();
    code_generation_graph_add_chunk(graph, "project_plan", CHUNKING_LEVEL_PROJECT, project_deps);
    string_array_free(project_deps);
    
    // Create module chunks with dependencies
    StringArray* module_deps = string_array_new();
    string_array_push(module_deps, "project_plan");
    
    code_generation_graph_add_chunk(graph, "database_module", CHUNKING_LEVEL_MODULE, module_deps);
    code_generation_graph_add_chunk(graph, "web_module", CHUNKING_LEVEL_MODULE, module_deps);
    
    string_array_free(module_deps);
    
    // Cleanup
    code_generation_graph_free(graph);
    return 0;
}
```

### Complete Development Workflow
```c
void development_workflow() {
    CodeGenerationGraph* graph = code_generation_graph_new(10000);
    
    // Add chunks with proper dependencies
    StringArray* no_deps = string_array_new();
    StringArray* db_deps = string_array_new();
    string_array_push(db_deps, "project_setup");
    
    code_generation_graph_add_chunk(graph, "project_setup", CHUNKING_LEVEL_PROJECT, no_deps);
    code_generation_graph_add_chunk(graph, "database_connector", CHUNKING_LEVEL_MODULE, db_deps);
    
    // Work on ready chunks
    StringArray* ready_chunks = code_generation_graph_get_ready_chunks(graph);
    for (size_t i = 0; i < ready_chunks->length; i++) {
        const char* chunk_id = ready_chunks->data[i];
        
        // Mark as in progress
        code_generation_graph_mark_chunk_in_progress(graph, chunk_id);
        
        // Generate code
        code_generation_graph_update_chunk_code(graph, chunk_id, 
            "def connect_db():\n    return Database()");
        
        // Mark completed
        code_generation_graph_mark_chunk_completed(graph, chunk_id);
    }
    
    // Get project summary
    char* summary = code_generation_graph_get_project_summary(graph);
    printf("Project Status:\n%s\n", summary);
    free(summary);
    
    string_array_free(no_deps);
    string_array_free(db_deps);
    string_array_free(ready_chunks);
    code_generation_graph_free(graph);
}
```

### Parsing Chunk Definitions
```c
void parse_chunk_example() {
    const char* chunk_def = 
        "<chunk>"
        "user_auth;class;User authentication system;login_service,user_model"
        "class UserAuth:\n    def authenticate(self, user, pass):\n        return True"
        "</chunk>";
    
    CodeChunk* chunk = parse_chunking_format(chunk_def);
    if (chunk) {
        printf("Parsed chunk: %s\n", chunk->chunk_id);
        printf("Level: %s\n", chunking_level_to_string(chunk->level));
        code_chunk_free(chunk);
    }
}
```

## Design Patterns

### 1. Composite Pattern
The chunk hierarchy (Project → Module → Class → Method → Block) follows the composite pattern, allowing uniform treatment of code units at different granularity levels.

### 2. Observer Pattern
The timestamp updates (`created_at`, `updated_at`) provide a simple observer mechanism for tracking state changes.

### 3. Builder Pattern
The progressive chunk building with dependency resolution implements a builder pattern for complex object creation.

### 4. Strategy Pattern
Different chunking levels represent different strategies for code organization and token limits.

### 5. Factory Pattern
The various `_new()` functions act as factories for creating different data structures.

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| String append | O(n) amortized | Doubling strategy |
| Array push | O(1) amortized | Doubling strategy |
| Map insert | O(n) | Linear search (simplified) |
| Map lookup | O(n) | Linear search (simplified) |
| Dependency resolution | O(V+E) | Graph traversal |
| Ready chunks check | O(n²) | Checks all dependencies |

### Space Complexity
| Structure | Space | Notes |
|-----------|-------|-------|
| String | O(n) | Dynamic allocation |
| StringArray | O(n) | Pointer array |
| StringMap | O(n) | Pair array |
| CodeGenerationGraph | O(V+E) | Graph structure |

### Optimization Opportunities
1. **Replace linear search maps** with hash tables for O(1) operations
2. **Implement caching** for frequently accessed chunks
3. **Add batch operations** for bulk updates
4. **Use more efficient** string concatenation algorithms

## Security Considerations

### Memory Safety
- **All allocations checked** with `CHECK_ALLOC` macro
- **String operations** use bounded copying
- **Pointer validation** before dereferencing
- **Memory cleanup** in all free functions

### Input Validation
- **Parameter null checks** in all public functions
- **Bounds checking** for array operations
- **String length validation** during parsing

### Data Integrity
- **Immutable chunk IDs** once created
- **Timestamp tracking** for audit trail
- **Dependency cycle detection** prevents deadlocks

### Recommendations
1. **Add input sanitization** for external chunk definitions
2. **Implement size limits** to prevent resource exhaustion
3. **Add encryption** for sensitive project data
4. **Include audit logging** for security events

## Testing Strategy

### Unit Tests

#### Memory Management Tests
```c
void test_memory_management() {
    // Test allocation failure handling
    // Test proper cleanup in error paths
    // Test memory leak detection
}
```

#### Data Structure Tests
```c
void test_string_operations() {
    String* str = string_new();
    assert(string_append(str, "test"));
    assert(str->length == 4);
    string_free(str);
}

void test_dependency_chain() {
    // Test cycle detection
    // Test dependency resolution
    // Test ready chunk calculation
}
```

#### Integration Tests
```c
void test_complete_workflow() {
    // Test full project lifecycle
    // Test error recovery
    // Test performance under load
}
```

### Test Categories
1. **Unit Tests**: Individual function testing
2. **Integration Tests**: Multi-component testing
3. **Performance Tests**: Load and stress testing
4. **Security Tests**: Input validation and boundary testing

### Testing Framework Recommendations
- Use **Google Test** for C++ wrapper
- Implement **valgrind** for memory leak detection
- Use **gcov** for code coverage analysis
- Implement **fuzz testing** for input validation

## Deployment Instructions

### Build Requirements
- **C Compiler**: GCC 4.8+ or Clang 3.4+
- **Standard Library**: C99 compatible
- **Build System**: Make or CMake

### Compilation
```bash
# Basic compilation
gcc -std=c99 -Wall -Wextra -pedantic code_generation_graph.c -o code_graph

# With debugging
gcc -std=c99 -g -O0 code_generation_graph.c -o code_graph_debug

# With optimizations
gcc -std=c99 -O2 -DNDEBUG code_generation_graph.c -o code_graph_release
```

### CMake Configuration
```cmake
cmake_minimum_required(VERSION 3.10)
project(CodeGenerationGraph)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_FLAGS "-Wall -Wextra -pedantic")

add_library(code_generation_graph STATIC code_generation_graph.c)
add_executable(demo main.c)
target_link_libraries(demo code_generation_graph)
```

### Integration Steps
1. **Include header** in your project
2. **Link against library** or include source
3. **Initialize graph** at project start
4. **Add chunks** according to project structure
5. **Process chunks** in dependency order
6. **Clean up** resources when done

## Troubleshooting Guide

### Common Issues

#### Memory Leaks
**Symptoms**: Increasing memory usage, eventual crash
**Solution**: 
```c
// Ensure all allocations are freed
CodeGenerationGraph* graph = code_generation_graph_new(1000);
// ... use graph ...
code_generation_graph_free(graph); // Don't forget this!
```

#### Dependency Deadlocks
**Symptoms**: No ready chunks, circular dependencies
**Solution**:
```c
// Check for cycles during chunk addition
StringArray* deps = string_array_new();
string_array_push(deps, "chunk_b");
code_generation_graph_add_chunk(graph, "chunk_a", CHUNKING_LEVEL_MODULE, deps);

// Later: don't create circular dependency
string_array_push(deps, "chunk_a"); // This creates a cycle!
```

#### Performance Issues
**Symptoms**: Slow dependency resolution with many chunks
**Solution**:
- Use more efficient data structures
- Implement caching for ready chunks
- Batch operations when possible

### Error Codes and Messages

| Error | Cause | Solution |
|-------|-------|----------|
| "Out of memory" | Allocation failure | Reduce chunk size, check memory limits |
| NULL returns | Invalid parameters | Add parameter validation |
| Infinite loops | Circular dependencies | Use dependency chain validation |

### Debugging Techniques

#### Memory Debugging
```c
// Use valgrind for memory checking
valgrind --leak-check=full ./code_graph

// Add debug logging
#ifdef DEBUG
#define DBG_PRINT(...) printf(__VA_ARGS__)
#else
#define DBG_PRINT(...)
#endif
```

#### Dependency Visualization
```c
// Print dependency graph for debugging
void print_dependency_graph(CodeGenerationGraph* graph) {
    for (size_t i = 0; i < graph->chunks->length; i++) {
        CodeChunk* chunk = graph->chunks->data[i].value;
        printf("%s depends on: ", chunk->chunk_id);
        for (size_t j = 0; j < chunk->dependencies->length; j++) {
            printf("%s ", chunk->dependencies->data[j]);
        }
        printf("\n");
    }
}
```

### Recovery Strategies

#### Graceful Failure
```c
CodeGenerationGraph* graph = code_generation_graph_new(1000);
if (!graph) {
    // Log error and exit gracefully
    fprintf(stderr, "Failed to initialize project graph\n");
    exit(EXIT_FAILURE);
}
```

#### Data Persistence
```c
// Save project state periodically
void save_project_state(CodeGenerationGraph* graph, const char* filename) {
    char* summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        FILE* f = fopen(filename, "w");
        if (f) {
            fputs(summary, f);
            fclose(f);
        }
        free(summary);
    }
}
```

This documentation provides comprehensive coverage of the code generation graph system. The implementation offers a robust foundation for managing complex software projects through dependency-based code generation with proper memory management and error handling.