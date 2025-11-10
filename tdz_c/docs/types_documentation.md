# TODOZI Types - Comprehensive Documentation

## Overview

This header file (`todoz_types.h`) defines the core type system for a sophisticated task management and AI agent system called TODOZI. It provides comprehensive type definitions, enumerations, and structures that form the foundation of the application's command system, data models, and API contracts.

## Table of Contents
1. [Header Protection and Includes](#header-protection)
2. [Forward Declarations](#forward-declarations)
3. [Type Definitions](#type-definitions)
4. [Optional Field Macros](#optional-macros)
5. [Enumeration Definitions](#enum-definitions)
6. [Structure Definitions](#structure-definitions)
7. [Command Hierarchy System](#command-hierarchy)
8. [Search Engine API](#search-engine-api)
9. [Architecture](#architecture)
10. [Design Patterns](#design-patterns)
11. [Performance Analysis](#performance-analysis)
12. [Security Considerations](#security-considerations)
13. [Testing Strategies](#testing-strategies)
14. [Deployment Instructions](#deployment-instructions)
15. [Troubleshooting Guide](#troubleshooting)

## Header Protection and Includes {#header-protection}

### Header Guard
```c
#ifndef TODOZI_TYPES_H
#define TODOZI_TYPES_H
// ... content ...
#endif // TODOZI_TYPES_H
```
- **Purpose**: Prevents multiple inclusion of the header file
- **Mechanism**: Standard C header guard pattern
- **Identifier**: `TODOZI_TYPES_H` (project-specific naming convention)

### Included Headers
```c
#include <stdio.h>      // Standard I/O functions
#include <stdlib.h>     // Memory allocation, process control
#include <string.h>     // String manipulation functions
#include <stdbool.h>    // Boolean type support
#include <time.h>       // Time and date functions
#include <stddef.h>     // Standard definitions (size_t, etc.)
```
- **Dependencies**: Standard C library headers only
- **Portability**: Highly portable across different C implementations
- **Memory Management**: Relies on standard library for memory operations

## Forward Declarations {#forward-declarations}

```c
typedef struct CodeChunk CodeChunk;
typedef struct AgentAssignment AgentAssignment;
typedef struct Error Error;
typedef struct Feeling Feeling;
typedef struct Idea Idea;
typedef struct Memory Memory;
typedef struct Task Task;
typedef struct TrainingData TrainingData;
```
- **Purpose**: Enable references to types defined in other files
- **Design Pattern**: Incomplete type declarations for modularity
- **File Organization**: Suggests a multi-file architecture with specialized modules

## Type Definitions {#type-definitions}

### DateTime Type
```c
typedef time_t DateTime;
```
- **Base Type**: `time_t` from `<time.h>`
- **Representation**: Seconds since Unix epoch (January 1, 1970)
- **Usage**: Timestamping for all temporal data in the system
- **Portability**: Standard POSIX time representation
- **Limitations**: Year 2038 problem on 32-bit systems

## Optional Field Macros {#optional-macros}

### OPTIONAL_PTR Macro
```c
#define OPTIONAL_PTR(type, name) \
    type name; \
    bool has_##name;
```
- **Purpose**: Create optional pointer fields with presence tracking
- **Usage**: `OPTIONAL_PTR(char*, tags)` creates `char* tags` and `bool has_tags`
- **Memory Layout**: Adds boolean flag for each optional field
- **Memory Overhead**: 1 byte per optional field + pointer size

### OPTIONAL_VALUE Macro
```c
#define OPTIONAL_VALUE(type, name) \
    struct { type value; bool present; } name;
```
- **Purpose**: Create optional value fields with presence tracking
- **Usage**: `OPTIONAL_VALUE(unsigned char, progress)` creates struct with `value` and `present`
- **Memory Layout**: Inline struct containing value and presence flag
- **Memory Overhead**: sizeof(type) + 1 byte padding

### Consistency Note
The code contains an inconsistency between `TaskUpdate` using `OPTIONAL_VALUE` for progress and `UpdateCommand` using a pointer. This should be unified for consistency.

## Enumeration Definitions {#enum-definitions}

### CommandType Enum (Primary Command Categories)
```c
typedef enum {
    CMD_INIT,                    // System initialization
    CMD_ADD,                     // Add new entities
    CMD_LIST,                    // List entities
    CMD_SHOW,                    // Show entity details
    CMD_UPDATE,                  // Update entities
    CMD_COMPLETE,                // Mark as complete
    CMD_FIX_CONSISTENCY,         // Data consistency operations
    CMD_CHECK_STRUCTURE,         // Structure validation
    CMD_ENSURE_STRUCTURE,        // Structure enforcement
    CMD_REGISTER,                // System registration
    CMD_REGISTRATION_STATUS,     // Registration status check
    CMD_CLEAR_REGISTRATION,      // Clear registration
    CMD_DELETE,                  // Delete entities
    CMD_PROJECT,                 // Project management
    CMD_SEARCH,                  // Search operations
    CMD_STATS,                   // Statistics display
    CMD_BACKUP,                  // Backup operations
    CMD_LIST_BACKUPS,            // List available backups
    CMD_RESTORE,                 // Restore from backup
    CMD_MEMORY,                  // Memory management
    CMD_IDEA,                    // Idea management
    CMD_AGENT,                   // AI agent management
    CMD_EMB,                     // Embedding operations
    CMD_ERROR_CMD,               // Error management
    CMD_TRAIN,                   // Training operations
    CMD_CHAT,                    // Chat functionality
    CMD_SEARCH_ALL,              // Comprehensive search
    CMD_MAESTRO,                 // Maestro system (orchestration)
    CMD_SERVER,                  // Server operations
    CMD_ML,                      // Machine learning operations
    CMD_IND_DEMO,                // Individual demonstration
    CMD_QUEUE,                   // Queue management
    CMD_API,                     // API management
    CMD_TDZ_CNT,                 // TODOZI content processing
    CMD_EXPORT_EMBEDDINGS,       // Embedding export
    CMD_MIGRATE,                 // Data migration
    CMD_TUI,                     // Text User Interface
    CMD_EXTRACT,                 // Data extraction
    CMD_STRATEGY,                // Strategy operations
    CMD_STEPS                    // Step management
} CommandType;
```

### Specialized Command Enums
The system uses a hierarchical enum structure where each primary command category has specialized sub-commands:

**Add Operations:**
```c
typedef enum {
    CMD_ADD_TASK
} AddCommandType;
```

**List Operations:**
```c
typedef enum {
    CMD_LIST_TASKS
} ListCommandType;
```

**Project Management:**
```c
typedef enum {
    CMD_PROJECT_CREATE,
    CMD_PROJECT_LIST,
    CMD_PROJECT_SHOW,
    CMD_PROJECT_ARCHIVE,
    CMD_PROJECT_DELETE,
    CMD_PROJECT_UPDATE
} ProjectCommandType;
```

**Queue Status:**
```c
typedef enum {
    QUEUE_STATUS_BACKLOG,
    QUEUE_STATUS_ACTIVE,
    QUEUE_STATUS_COMPLETE
} QueueStatus;
```

## Structure Definitions {#structure-definitions}

### Task Management Structures

#### TaskUpdate Structure
```c
typedef struct {
    char* id;                           // Required: Task identifier
    OPTIONAL_PTR(char*, action);        // Optional: Action description
    OPTIONAL_PTR(char*, time);          // Optional: Time specification
    OPTIONAL_PTR(char*, priority);      // Optional: Priority level
    OPTIONAL_PTR(char*, project);       // Optional: Project association
    OPTIONAL_PTR(char*, status);        // Optional: Status value
    OPTIONAL_PTR(char*, assignee);      // Optional: Assignee information
    OPTIONAL_PTR(char*, tags);          // Optional: Tags list
    OPTIONAL_PTR(char*, dependencies);  // Optional: Dependency list
    OPTIONAL_PTR(char*, context);       // Optional: Context information
    OPTIONAL_VALUE(unsigned char, progress); // Optional: Progress percentage (0-100)
} TaskUpdate;
```

**Field Specifications:**
- `id`: Unique identifier (required, non-null)
- All other fields: Optional with presence tracking
- `progress`: Unsigned char (0-255 range, typically 0-100 for percentages)

#### UpdateCommand Structure
```c
typedef struct {
    char* id;                    // Required: Task identifier
    char* action;                // Required: Action description  
    char* time;                  // Required: Time specification
    char* priority;              // Required: Priority level
    char* project;               // Required: Project association
    char* status;                // Required: Status value
    char* assignee;              // Required: Assignee information
    char* tags;                  // Required: Tags list
    char* dependencies;          // Required: Dependency list
    char* context;               // Required: Context information
    unsigned char* progress;     // Required: Progress pointer (inconsistent design)
} UpdateCommand;
```

**Design Issue**: Inconsistent use of pointer for progress vs TaskUpdate's value approach.

### AI Agent Management Structures

#### AgentCreateCommand Structure
```c
typedef struct {
    char* id;                                // Required: Agent identifier
    char* name;                              // Required: Agent name
    char* description;                       // Required: Agent description
    char* category;                          // Required: Category classification
    OPTIONAL_PTR(char*, capabilities);       // Optional: Capabilities list
    OPTIONAL_PTR(char*, specializations);    // Optional: Specializations
    char* model_provider;                    // Required: AI model provider
    char* model_name;                        // Required: Model name
    float temperature;                       // Required: Creativity parameter (0.0-1.0)
    unsigned int max_tokens;                 // Required: Maximum tokens per response
    OPTIONAL_PTR(char*, tags);               // Optional: Tags for categorization
    OPTIONAL_PTR(char*, system_prompt);      // Optional: System prompt template
    OPTIONAL_PTR(char*, prompt_template);    // Optional: User prompt template
    OPTIONAL_VALUE(bool, auto_format_code);  // Optional: Code formatting flag
    OPTIONAL_VALUE(bool, include_examples);  // Optional: Example inclusion flag
    OPTIONAL_VALUE(bool, explain_complexity);// Optional: Complexity explanation flag
    OPTIONAL_VALUE(bool, suggest_tests);     // Optional: Test suggestion flag
    OPTIONAL_PTR(char*, tools);              // Optional: Available tools
    OPTIONAL_VALUE(unsigned int, max_response_length); // Optional: Response length limit
    OPTIONAL_VALUE(unsigned int, timeout_seconds);     // Optional: Timeout in seconds
    OPTIONAL_VALUE(unsigned int, requests_per_minute); // Optional: Rate limiting
    OPTIONAL_VALUE(unsigned int, tokens_per_hour);     // Optional: Token budget
} AgentCreateCommand;
```

### Machine Learning Structures

#### MLProcessCommand Structure
```c
typedef struct {
    char* text;        // Required: Text to process
    bool use_ml;       // Required: Use ML processing flag
    char* model;       // Required: Model identifier
} MLProcessCommand;
```

#### MLAdvancedProcessCommand Structure
```c
typedef struct {
    char* text;        // Required: Text to process
    bool analytics;    // Required: Analytics flag
} MLAdvancedProcessCommand;
```

### Data Collection Structures

#### MaestroCollectConversationCommand Structure
```c
typedef struct {
    char* session_id;                  // Required: Session identifier
    char* conversation;                // Required: Conversation content
    size_t context_length;             // Required: Context length
    OPTIONAL_PTR(char*, tool_calls);   // Optional: Tool calls data
    char* response;                    // Required: AI response
    unsigned long long response_time_ms; // Required: Response time in milliseconds
} MaestroCollectConversationCommand;
```

## Command Hierarchy System {#command-hierarchy}

The system implements a comprehensive command hierarchy using union types:

### Hierarchical Command Structure

```c
// Base command structure with type and union
typedef struct {
    CommandType type;
    union {
        // Direct command structures
        UpdateCommand update;
        CompleteCommand complete;
        // ... other direct commands
        
        // Hierarchical command structures
        AddCommand add;
        ListCommand list;
        ShowCommand show;
        // ... other hierarchical commands
    } data;
} Command;
```

### Example: AddCommand Hierarchy
```c
typedef struct {
    AddCommandType type;
    union {
        AddTaskCommand task;
    } data;
} AddCommand;
```

### Example: ProjectCommand Hierarchy
```c
typedef struct {
    ProjectCommandType type;
    union {
        ProjectCreateCommand create;
        ProjectUpdateCommand update;
        char* name;  // For simple operations like show/delete
    } data;
} ProjectCommand;
```

## Search Engine API {#search-engine-api}

### SearchOptions Structure
```c
typedef struct {
    OPTIONAL_VALUE(size_t, limit);      // Optional: Result limit
    OPTIONAL_PTR(char*, data_types);    // Optional: Data types to search
    OPTIONAL_PTR(char*, since);         // Optional: Start datetime
    OPTIONAL_PTR(char*, until);         // Optional: End datetime
} SearchOptions;
```

### SearchResults Structure
```c
typedef struct {
    Task* task_results;
    Memory* memory_results;
    Idea* idea_results;
    Error* error_results;
    TrainingData* training_results;
    size_t task_results_count;
    size_t memory_results_count;
    size_t idea_results_count;
    size_t error_results_count;
    size_t training_results_count;
} SearchResults;
```

### ChatContent Structure
```c
typedef struct {
    Task* tasks;
    Memory* memories;
    Idea* ideas;
    AgentAssignment* agent_assignments;
    CodeChunk* code_chunks;
    Error* errors;
    TrainingData* training_data;
    Feeling* feelings;
    size_t tasks_count;
    size_t memories_count;
    size_t ideas_count;
    size_of agent_assignments_count;
    size_t code_chunks_count;
    size_t errors_count;
    size_t training_data_count;
    size_t feelings_count;
} ChatContent;
```

## Architecture {#architecture}

### System Architecture Diagram

```
TODOZI System Architecture
=================================

Application Layer
├── Command Parser
├── Command Dispatcher
├── User Interface (TUI/CLI)
└── API Server

Core System Layer
├── Task Management Engine
├── Project Management
├── AI Agent Orchestrator
├── Machine Learning Pipeline
├── Search Engine
├── Data Persistence
└── Backup/Restore System

Data Layer
├── Task Repository
├── Project Repository  
├── Agent Repository
├── Memory Store
├── Training Data Store
└── Embedding Database

External Integration Layer
├── ML Model Providers
├── Embedding Services
└── External APIs
```

### Module Interaction Flow

```
User Input → Command Parser → Command Dispatcher → Specific Handler → Data Repository → Response Generator → User Output
```

### Data Flow Architecture

```
Raw Data → Extraction → Processing → Storage → Search Index → Query Processing → Results
```

## Design Patterns {#design-patterns}

### 1. Command Pattern
**Implementation**: Hierarchical command structure with unions
**Purpose**: Encapsulate requests as objects, allowing parameterization and queuing
**Benefits**: 
- Easy extension of new command types
- Uniform command processing interface
- Command history and undo capability

### 2. Builder Pattern (implied)
**Implementation**: Command structures with optional fields
**Purpose**: Step-by-step construction of complex objects
**Benefits**:
- Flexible object creation
- Clear parameter specification
- Validation during construction

### 3. Strategy Pattern
**Implementation**: Different command handlers for each command type
**Purpose**: Define family of algorithms, make them interchangeable
**Benefits**:
- Easy addition of new functionality
- Clean separation of concerns
- Runtime algorithm selection

### 4. Composite Pattern
**Implementation**: Hierarchical command structure
**Purpose**: Treat individual and composite objects uniformly
**Benefits**:
- Unified interface for simple and complex commands
- Recursive command processing

### 5. Observer Pattern (implied)
**Implementation**: Event-based system for command execution
**Purpose**: Define one-to-many dependencies between objects
**Benefits**:
- Loose coupling between components
- Event-driven architecture

## Performance Analysis {#performance-analysis}

### Memory Usage Analysis

**Structure Sizes (64-bit system):**
- `TaskUpdate`: ~100 bytes + string lengths
- `AgentCreateCommand`: ~200 bytes + string lengths  
- `Command` union: Largest member determines size (~200 bytes)

**Memory Optimization Techniques:**
- Optional field macros minimize memory for unused fields
- Pointer-based string storage reduces structure size
- Union-based command hierarchy saves memory

### Time Complexity

**Command Processing:**
- Parsing: O(n) where n is command length
- Dispatch: O(1) via jump table/switch statement
- Execution: Varies by command type (O(1) to O(n))

**Search Operations:**
- Indexed search: O(log n) to O(1)
- Full-text search: O(n) without indexing

### Scalability Considerations

**Memory Scaling:**
- Linear growth with number of entities
- Pointer-based design reduces memory overhead

**Processing Scaling:**
- Command processing scales linearly with input size
- Search operations may require indexing for large datasets

## Security Considerations {#security-considerations}

### Input Validation

**String Handling:**
```c
// Safe string copying example
char* safe_strdup(const char* src) {
    if (!src) return NULL;
    size_t len = strlen(src);
    char* dest = malloc(len + 1);
    if (dest) {
        strncpy(dest, src, len);
        dest[len] = '\0';
    }
    return dest;
}
```

**Pointer Validation:**
- All pointer parameters should be validated before use
- Optional pointers require `has_field` checks before access

### API Security

**Authentication:**
- `ApiRegisterCommand` and `ApiCheckCommand` suggest API key management
- Public/private key pairs for API authentication

**Data Protection:**
- `MemoryCreateCommand` has `secret` type for sensitive data
- Proper encryption for stored sensitive information

### Security Best Practices

1. **Input Sanitization**: Validate all command parameters
2. **Memory Safety**: Use bounds-checked string functions
3. **Access Control**: Implement proper authentication/authorization
4. **Data Encryption**: Encrypt sensitive data at rest and in transit
5. **Audit Logging**: Track all command executions

## Testing Strategies {#testing-strategies}

### Unit Testing

**Test Structure Validation:**
```c
// Example test for TaskUpdate structure
void test_task_update_creation() {
    TaskUpdate update = {
        .id = strdup("task123"),
        .has_action = true,
        .action = strdup("Test action")
    };
    
    assert(update.id != NULL);
    assert(update.has_action == true);
    assert(update.action != NULL);
    
    // Test optional field absence
    assert(update.has_tags == false);
    assert(update.tags == NULL);
}
```

**Command Parsing Tests:**
- Test each command type parsing
- Validate optional field handling
- Test error conditions and edge cases

### Integration Testing

**Command Execution Flow:**
```c
// Test complete command flow
void test_command_execution() {
    Command cmd = parse_user_input("add task 'Test'");
    CommandResult result = execute_command(cmd);
    assert(result.success == true);
    assert(result.entity_id != NULL);
}
```

**Data Persistence Tests:**
- Test backup/restore functionality
- Verify data consistency after operations
- Test migration procedures

### Performance Testing

**Load Testing:**
- Test with large numbers of entities
- Measure memory usage growth
- Benchmark search performance

**Stress Testing:**
- Test under high concurrent load
- Verify system stability under stress
- Test error recovery mechanisms

## Deployment Instructions {#deployment-instructions}

### Build Requirements

**Compiler Requirements:**
- C99 compliant compiler (GCC, Clang, MSVC)
- Standard C library support
- POSIX compliance for time functions

**Build Configuration:**
```makefile
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -Werror
LDFLAGS = -lm
SRCS = main.c command.c search.c storage.c
OBJS = $(SRCS:.c=.o)
TARGET = todozi

$(TARGET): $(OBJS)
	$(CC) $(OBJS) -o $(TARGET) $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
```

### Installation Steps

1. **Clone Repository:**
   ```bash
   git clone https://github.com/todozi/todozi.git
   cd todozi
   ```

2. **Build System:**
   ```bash
   make release
   ```

3. **Install Binary:**
   ```bash
   sudo make install
   ```

4. **Configure Environment:**
   ```bash
   export TODOZI_HOME=/opt/todozi
   export PATH=$PATH:$TODOZI_HOME/bin
   ```

### Configuration Management

**Configuration File Example:**
```json
{
  "database": {
    "path": "/var/lib/todozi/data.db",
    "backup_interval": 3600
  },
  "ai": {
    "default_model": "gpt-4",
    "temperature": 0.7,
    "max_tokens": 2000
  },
  "security": {
    "api_key_rotation": 30,
    "encryption_enabled": true
  }
}
```

## Troubleshooting Guide {#troubleshooting}

### Common Issues and Solutions

**Memory Leaks:**
```c
// Proper cleanup function
void cleanup_command(Command* cmd) {
    if (!cmd) return;
    
    switch(cmd->type) {
        case CMD_ADD:
            if (cmd->data.add.data.task.id) free(cmd->data.add.data.task.id);
            // ... cleanup other fields
            break;
        // ... other command types
    }
}
```

**Command Parsing Errors:**
- Symptom: Invalid command execution
- Solution: Validate command structure before processing
- Debug: Use command validation function

**Performance Issues:**
- Symptom: Slow response times
- Solution: Implement search indexing
- Debug: Profile command execution times

### Debugging Techniques

**Logging Implementation:**
```c
typedef enum {
    LOG_DEBUG,
    LOG_INFO,
    LOG_WARNING,
    LOG_ERROR
} LogLevel;

void log_message(LogLevel level, const char* format, ...) {
    va_list args;
    va_start(args, format);
    vprintf(format, args);
    va_end(args);
}
```

**Memory Debugging:**
- Use valgrind or address sanitizer
- Implement memory allocation tracking
- Test with different allocation patterns

### Recovery Procedures

**Data Corruption:**
1. Identify corrupted data structures
2. Use `CMD_FIX_CONSISTENCY` command
3. Restore from backup if necessary

**System Crash:**
1. Check system logs for error details
2. Verify data file integrity
3. Use backup restoration procedure

**Performance Degradation:**
1. Analyze search query patterns
2. Rebuild search indexes
3. Optimize database queries

## Conclusion

This comprehensive type system forms the backbone of a sophisticated task management and AI orchestration platform. The design demonstrates careful consideration of extensibility, memory efficiency, and command processing efficiency. The hierarchical command structure allows for flexible system evolution while maintaining backward compatibility.

The system's architecture supports complex AI agent management, machine learning integration, and comprehensive search capabilities, making it suitable for advanced productivity and automation scenarios. Proper implementation of the security, testing, and deployment strategies outlined will ensure a robust and reliable system.

---
*Documentation generated for TODOZI Types System - Version 1.0*