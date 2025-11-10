# Comprehensive Documentation: Todozi Library

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Functions](#functions)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)
12. [Error Handling](#error-handling)

## Overview

The Todozi library is a comprehensive C library designed for parsing and processing structured content from chat messages. It supports multiple data types including tasks, memories, ideas, agent assignments, errors, training data, and feelings. The system uses XML-like tags to identify and parse different content types within unstructured text.

### Key Features
- Multi-format content parsing (tasks, memories, ideas, errors, etc.)
- Shorthand tag transformation system
- Comprehensive error handling
- Memory-safe resource management
- UUID generation for all entities
- Flexible tag and dependency processing

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Input Message │───▶│ Tag Transformer  │───▶│ Parser Engine   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                            │
                                                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Content Extractors                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────────────┐  │
│  │ Task     │  │ Memory   │  │ Idea     │  │ Error Parser    │  │
│  │ Parser   │  │ Parser   │  │ Parser   │  │                 │  │
│  └──────────┘  └──────────┘  └──────────┘  └─────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                                            │
                                                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                 Structured Content Output                       │
└─────────────────────────────────────────────────────────────────┘
```

### Component Relationships
```
ChatContent (Container)
├── Tasks[]
├── Memories[]
├── Ideas[]
├── AgentAssignments[]
├── Errors[]
├── TrainingData[]
└── Feelings[]
```

## Data Structures

### Enum Types

#### TodoziError
```c
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_VALIDATION_ERROR,   // Format validation failed
    TODOZI_STORAGE_ERROR,      // Memory allocation error
    TODOZI_PARSE_ERROR         // General parsing error
} TodoziError;
```

#### Priority Levels
```c
typedef enum {
    PRIORITY_LOW,      // Low priority tasks
    PRIORITY_MEDIUM,   // Medium priority (default)
    PRIORITY_HIGH,     // High priority
    PRIORITY_CRITICAL  // Critical priority
} Priority;
```

#### Status Levels
```c
typedef enum {
    STATUS_TODO,        // Task not started
    STATUS_IN_PROGRESS, // Task in progress
    STATUS_DONE,        // Task completed
    STATUS_BLOCKED,     // Task blocked
    STATUS_DEFERRED     // Task deferred
} Status;
```

### Core Structures

#### Task Structure
```c
typedef struct Task {
    char* id;                     // UUID identifier
    char* user_id;                // User identifier
    char* action;                 // Task description
    char* time;                   // Time specification
    Priority priority;            // Priority level
    char* parent_project;         // Parent project ID
    Status status;                // Current status
    AssigneeType assignee_type;   // Assignment type
    char* assignee_name;          // Specific agent name
    char** tags;                  // Tag array
    size_t tags_count;           // Number of tags
    char** dependencies;          // Dependency IDs
    size_t dependencies_count;   // Number of dependencies
    char* context_notes;          // Context information
    int progress;                 // Progress percentage (0-100)
    int has_progress;            // Progress flag
    double* embedding_vector;     // Vector embedding
    size_t embedding_size;       // Embedding dimensions
    time_t created_at;           // Creation timestamp
    time_t updated_at;           // Last update timestamp
} Task;
```

#### Memory Structure
```c
typedef struct Memory {
    char* id;                    // UUID identifier
    char* user_id;               // User identifier
    char* project_id;            // Associated project
    ItemStatus status;           // Active/Inactive/Archived
    char* moment;                // Memory context
    char* meaning;               // Memory significance
    char* reason;                // Reason for memory
    MemoryImportance importance; // Importance level
    MemoryTerm term;             // Short/Long term
    MemoryType memory_type;      // Memory classification
    char** tags;                 // Tag array
    size_t tags_count;          // Number of tags
    time_t created_at;          // Creation timestamp
    time_t updated_at;          // Last update timestamp
    char* emotion;               // Emotional context
} Memory;
```

#### ChatContent Container
```c
typedef struct ChatContent {
    Task* tasks;                        // Extracted tasks
    size_t tasks_count;                // Task count
    Memory* memories;                  // Extracted memories
    size_t memories_count;            // Memory count
    Idea* ideas;                       // Extracted ideas
    size_t ideas_count;               // Idea count
    AgentAssignment* agent_assignments; // Agent assignments
    size_t agent_assignments_count;   // Assignment count
    CodeChunk* code_chunks;            // Code chunks
    size_t code_chunks_count;         // Code chunk count
    Error* errors;                     // Error records
    size_t errors_count;              // Error count
    TrainingData* training_data;       // Training data
    size_t training_data_count;       // Training data count
    Feeling* feelings;                 // Emotional feelings
    size_t feelings_count;            // Feeling count
    Summary* summaries;                // Summaries
    size_t summaries_count;           // Summary count
    Reminder* reminders;               // Reminders
    size_t reminders_count;           // Reminder count
} ChatContent;
```

## Functions

### Memory Management Functions

#### `string_copy`
```c
/**
 * Creates a deep copy of a string
 * 
 * @param source: Source string to copy
 * @return: Newly allocated string copy, or NULL on failure
 * @warning: Caller must free the returned string
 */
char* string_copy(const char* source);
```

#### `free_string_array`
```c
/**
 * Frees an array of strings and the array itself
 * 
 * @param array: Array of strings to free
 * @param count: Number of elements in the array
 */
void free_string_array(char** array, size_t count);
```

#### `free_task_contents`
```c
/**
 * Frees all internal resources of a Task structure
 * 
 * @param task: Task structure to clean up
 * @note: Does not free the Task structure itself
 */
static void free_task_contents(Task* task);
```

### Parsing Functions

#### `transform_shorthand_tags`
```c
/**
 * Transforms shorthand tags to full XML tags
 * 
 * @param message: Input message with shorthand tags
 * @return: New string with transformed tags, or NULL on error
 * 
 * Supported transformations:
 * <tz> → <todozi>, </tz> → </todozi>
 * <mm> → <memory>, </mm> → </memory>
 * <id> → <idea>, </id> → </idea>
 * <ch> → <chunk>, </ch> → </chunk>
 * <fe> → <feel>, </fe> → </feel>
 * <tn> → <train>, </tn> → </train>
 * <er> → <error>, </er> → </error>
 * <sm> → <summary>, </sm> → </summary>
 * <rd> → <reminder>, </rd> → </reminder>
 */
char* transform_shorthand_tags(const char* message);
```

#### `parse_todozi_format`
```c
/**
 * Parses a todozi formatted string into a Task structure
 * 
 * @param todozi_text: Text between <todozi> tags
 * @param task: Output Task structure (must be allocated)
 * @return: TodoziError code indicating success/failure
 * 
 * Format: action;time;priority;project;status[;assignee;tags;dependencies;context;progress]
 */
TodoziError parse_todozi_format(const char* todozi_text, Task* task);
```

#### `process_chat_message`
```c
/**
 * Processes a chat message to extract Task elements
 * 
 * @param message: Input chat message
 * @param tasks: Output array of Tasks (allocated by function)
 * @param tasks_count: Output number of tasks found
 * @return: TodoziError code indicating success/failure
 * 
 * @note: Caller must free the tasks array using free_task_contents
 */
TodoziError process_chat_message(const char* message, Task** tasks, size_t* tasks_count);
```

#### `process_chat_message_extended`
```c
/**
 * Extended message processing for all content types
 * 
 * @param message: Input chat message
 * @param user_id: User identifier for content attribution
 * @param content: Output ChatContent structure (must be allocated)
 * @return: TodoziError code indicating success/failure
 * 
 * @note: Caller must free content using free_chat_content
 */
TodoziError process_chat_message_extended(const char* message, const char* user_id, ChatContent* content);
```

### Type Parsing Functions

#### `parse_priority`
```c
/**
 * Parses priority string to enum value (case-insensitive)
 * 
 * @param str: Priority string ("low", "medium", "high", "critical")
 * @return: Corresponding Priority enum value
 * @default: PRIORITY_MEDIUM for unrecognized values
 */
Priority parse_priority(const char* str);
```

#### `parse_status`
```c
/**
 * Parses status string to enum value (case-insensitive)
 * 
 * @param str: Status string ("todo", "in_progress", "done", etc.)
 * @return: Corresponding Status enum value
 * @default: STATUS_TODO for unrecognized values
 */
Status parse_status(const char* str);
```

#### `parse_assignee`
```c
/**
 * Parses assignee specification string
 * 
 * @param str: Assignee string ("ai", "human", "collaborative", "agent=name")
 * @param assignee_name: Output agent name for ASSIGNEE_AGENT type
 * @return: Corresponding AssigneeType enum value
 * @default: ASSIGNEE_HUMAN for unrecognized values
 */
AssigneeType parse_assignee(const char* str, char** assignee_name);
```

## Usage Examples

### Basic Task Extraction
```c
#include "todozi.h"
#include <stdio.h>

int main() {
    const char* message = "I need to <todozi>Finish report;tomorrow;high;project_x;todo;ai;urgent,important;task123;Finish by EOD;50</todozi>";
    
    Task* tasks = NULL;
    size_t tasks_count = 0;
    
    TodoziError err = process_chat_message(message, &tasks, &tasks_count);
    
    if (err == TODOZI_SUCCESS && tasks_count > 0) {
        printf("Extracted %zu tasks\n", tasks_count);
        printf("Task ID: %s\n", tasks[0].id);
        printf("Action: %s\n", tasks[0].action);
        printf("Priority: %d\n", tasks[0].priority);
        
        // Clean up
        for (size_t i = 0; i < tasks_count; i++) {
            free_task_contents(&tasks[i]);
        }
        free(tasks);
    }
    
    return 0;
}
```

### Full Content Processing
```c
#include "todozi.h"
#include <stdio.h>

int main() {
    const char* message = 
        "User: <tz>Buy groceries;today;medium;personal;todo</tz> "
        "<mm>standard;Morning breakfast;Important meal;Health reason;high;short;nutrition</mm> "
        "<id>New feature idea;share;high;Implement dark mode</id>";
    
    ChatContent content;
    const char* user_id = "user123";
    
    TodoziError err = process_chat_message_extended(message, user_id, &content);
    
    if (err == TODOZI_SUCCESS) {
        printf("Tasks: %zu\n", content.tasks_count);
        printf("Memories: %zu\n", content.memories_count);
        printf("Ideas: %zu\n", content.ideas_count);
        
        // Process extracted content...
        
        // Clean up
        free_chat_content(&content);
    }
    
    return 0;
}
```

### Shorthand Tag Usage
```c
// Shorthand tags are automatically expanded
const char* message = 
    "<tz>Quick task;now;low;test;todo</tz> "
    "<mm>emotional;happy;Good moment;Success;medium;long;celebration</mm>";

// Equivalent to:
// "<todozi>Quick task;now;low;test;todo</todozi> "
// "<memory>emotional;happy;Good moment;Success;medium;long;celebration</memory>"
```

## Design Patterns

### 1. Parser Factory Pattern
Each content type has its own parser function following a consistent interface:
- `parse_[type]_format()` functions
- Unified error handling
- Consistent memory management

### 2. Resource Acquisition Is Initialization (RAII)
- All allocated resources are properly freed
- Cleanup functions for each structure type
- Consistent memory management patterns

### 3. Strategy Pattern
Different parsing strategies for different content types:
- Regular expression-based extraction
- Semantic parsing for different formats
- Flexible tag processing

### 4. Composite Pattern
`ChatContent` acts as a composite container for various content types:
- Unified interface for content extraction
- Consistent processing pipeline
- Modular content handling

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Tag transformation | O(n*m) | n = message length, m = tag mappings |
| Regex extraction | O(n) | n = message length |
| String splitting | O(n) | n = string length |
| Content parsing | O(k) | k = number of content elements |

### Memory Usage
| Component | Memory Footprint | Notes |
|-----------|------------------|-------|
| Task structure | ~200-500 bytes | Depends on string lengths |
| Memory structure | ~150-400 bytes | Emotional memories larger |
| ChatContent | Variable | Depends on extracted content |
| String arrays | O(n) | n = number of elements |

### Optimization Strategies
1. **Regex Pre-compilation**: Regular expressions are compiled once per content type
2. **Memory Pooling**: Consider implementing memory pools for frequent allocations
3. **Lazy Parsing**: Parse only required content types on demand
4. **String Interning**: Reduce duplicate string allocations

## Security Considerations

### Input Validation
```c
// All input strings are validated:
- Null pointer checks
- Empty string handling
- Buffer length validation
- Regular expression safety
```

### Memory Safety
```c
// Comprehensive memory management:
- All allocations checked for success
- Consistent freeing patterns
- Zeroing of structures after free
- Protection against double-free
```

### Data Integrity
- UUID generation for all entities
- Timestamp validation
- Enum value range checking
- String length limitations

### Security Best Practices
1. **Bounds Checking**: All array operations include bounds checks
2. **Input Sanitization**: Tag content is properly trimmed and validated
3. **Resource Limits**: Consider implementing max content limits
4. **Error Containment**: Failures are contained and don't leak information

## Testing Strategies

### Unit Test Framework
```c
// Example test structure
typedef struct {
    const char* test_name;
    const char* input;
    TodoziError expected_error;
    size_t expected_elements;
} TestCase;

// Test cases for different scenarios
TestCase test_cases[] = {
    {"Valid Task", "<todozi>Test;now;medium;project;todo</todozi>", TODOZI_SUCCESS, 1},
    {"Invalid Format", "<todozi>Incomplete</todozi>", TODOZI_VALIDATION_ERROR, 0},
    {"Memory Error", NULL, TODOZI_STORAGE_ERROR, 0},
    // ... more test cases
};
```

### Test Categories
1. **Functional Tests**: Valid input parsing
2. **Boundary Tests**: Edge cases and limits
3. **Error Tests**: Invalid input handling
4. **Memory Tests**: Allocation and cleanup
5. **Performance Tests**: Large input processing

### Test Coverage Goals
- 100% function coverage
- 90% branch coverage
- All error paths tested
- Memory leak detection

## Deployment Instructions

### Build Requirements
```bash
# Required libraries
sudo apt-get install libuuid-dev  # UUID support

# Compilation flags
gcc -o todozi_app main.c todozi.c -luuid -lregex
```

### CMake Configuration
```cmake
cmake_minimum_required(VERSION 3.10)
project(Todozi)

find_package(PkgConfig REQUIRED)
pkg_check_modules(UUID REQUIRED uuid)

add_library(todozi STATIC todozi.c)
target_link_libraries(todozi ${UUID_LIBRARIES})
target_include_directories(todozi PUBLIC ${UUID_INCLUDE_DIRS})

add_executable(todozi_app main.c)
target_link_libraries(todozi_app todozi)
```

### Docker Deployment
```dockerfile
FROM gcc:latest

RUN apt-get update && apt-get install -y libuuid-dev

COPY . /app
WORKDIR /app

RUN gcc -o todozi_app main.c todozi.c -luuid -lregex

CMD ["./todozi_app"]
```

## Troubleshooting Guide

### Common Issues

#### Memory Leaks
**Symptoms**: Increasing memory usage, application slowdown
**Solution**: Use Valgrind to detect leaks:
```bash
valgrind --leak-check=full ./todozi_app
```

#### Parsing Failures
**Symptoms**: Tasks not extracted, error codes returned
**Debugging Steps**:
1. Check input format compliance
2. Verify tag nesting and closure
3. Test with minimal valid input
4. Check regular expression patterns

#### Compilation Errors
**Issue**: Missing UUID library
**Solution**: Install development package:
```bash
# Ubuntu/Debian
sudo apt-get install libuuid-dev

# CentOS/RHEL
sudo yum install libuuid-devel
```

### Debug Mode
Compile with debugging symbols:
```bash
gcc -g -DDEBUG -o todozi_app main.c todozi.c -luuid -lregex
```

### Logging Integration
Add debug logging:
```c
#ifdef DEBUG
#define DEBUG_LOG(fmt, ...) fprintf(stderr, "DEBUG: " fmt "\n", ##__VA_ARGS__)
#else
#define DEBUG_LOG(fmt, ...)
#endif
```

## Error Handling

### Error Recovery Strategies
1. **Graceful Degradation**: Continue processing after partial failures
2. **Resource Cleanup**: Ensure proper cleanup on error paths
3. **Error Reporting**: Provide detailed error information
4. **Retry Mechanisms**: For transient failures

### Error Code Meanings
| Error Code | Meaning | Recovery Action |
|------------|---------|-----------------|
| `TODOZI_SUCCESS` | Operation completed successfully | Continue normal processing |
| `TODOZI_VALIDATION_ERROR` | Input format invalid | Check input format, retry with corrected input |
| `TODOZI_STORAGE_ERROR` | Memory allocation failed | Reduce input size, check system memory |
| `TODOZI_PARSE_ERROR` | General parsing error | Validate input, check for corruption |

### Best Practices
- Always check return codes
- Implement comprehensive cleanup
- Provide meaningful error messages
- Consider error logging for production use

---

This documentation provides comprehensive coverage of the Todozi library. For additional support, refer to the source code comments and test suite implementations.