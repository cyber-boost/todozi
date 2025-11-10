# Comprehensive Documentation for Todozi (TDZ) API Client Library

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Code Analysis](#code-analysis)
4. [Data Structures](#data-structures)
5. [Functions](#functions)
6. [Usage Examples](#usage-examples)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategy](#testing-strategy)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi (TDZ) API Client Library is a comprehensive C library for interacting with the Todozi AI assistant API. It provides functionality to parse TDZ commands embedded in text, execute them against the API, and process responses. The library handles command parsing, request building, HTTP communication, and JSON response processing.

### Key Features
- **Command Parsing**: Extracts TDZ commands from text using `<tdz>...</tdz>` tags
- **RESTful API Integration**: Supports GET, POST, PUT, DELETE operations
- **JSON Handling**: Comprehensive JSON request/response processing using jansson
- **Error Handling**: Robust error management with detailed error types
- **Memory Management**: Safe memory allocation and cleanup
- **Thread Safety**: Uses thread-safe string operations

## Architecture

### System Architecture Diagram
```
Text Input → Parser → Command Vector → Executor → HTTP Client → API Server
    ↓           ↓           ↓           ↓           ↓           ↓
Error Handling ← Results Processing ← JSON Parsing ← Response ← Server Response
```

### Component Relationships
```
+----------------+     +----------------+     +-----------------+
|   Text Input   | --> |   Parser       | --> | Command Vector  |
+----------------+     +----------------+     +-----------------+
                                                  |
                                                  v
+----------------+     +----------------+     +-----------------+
|   API Server   | <-- | HTTP Client    | <-- |   Executor      |
+----------------+     +----------------+     +-----------------+
```

### Data Flow
1. **Input**: Text containing `<tdz>command;target;params</tdz>` tags
2. **Parsing**: Extract and validate commands into structured format
3. **Execution**: Convert commands to HTTP requests with proper headers/body
4. **Communication**: Send requests to Todozi API server
5. **Processing**: Parse JSON responses and handle errors

## Code Analysis

### Dependencies and Requirements

#### Required Libraries
- **libcurl**: HTTP client functionality
- **jansson**: JSON parsing and generation
- **Standard C Library**: Memory management, string operations

#### Compilation Flags
```bash
gcc -std=c99 -Wall -Wextra -pedantic -D_POSIX_C_SOURCE=200809L \
    -I/usr/local/include -L/usr/local/lib -lcurl -ljansson \
    tdz_client.c -o tdz_client
```

### Memory Management Strategy
The library implements careful memory management with:
- **Ownership Semantics**: Clear ownership of allocated memory
- **RAII-like Patterns**: Automatic cleanup functions
- **Error Recovery**: Graceful handling of allocation failures
- **Buffer Overflow Protection**: Safe string operations with bounds checking

## Data Structures

### TdzErrorType Enum
```c
typedef enum {
    TDZ_ERROR_NONE,        // No error
    TDZ_ERROR_VALIDATION,  // Input validation error
    TDZ_ERROR_NETWORK,     // Network communication error
    TDZ_ERROR_JSON,        // JSON parsing/generation error
    TDZ_ERROR_OOM          // Out of memory error
} TdzErrorType;
```

### TdzError Structure
```c
typedef struct {
    TdzErrorType type;     // Error category
    char* message;         // Human-readable error message (owned)
} TdzError;
```

**Memory Ownership**: The `message` field is owned by the structure and must be freed with `tdz_error_free()`.

### TdzCommand Structure
```c
typedef struct {
    char* command;         // Action (create, list, get, update, delete, run)
    char* target;          // Resource type (task, memory, agent, etc.)
    char** parameters;     // Array of parameter strings
    size_t param_count;    // Number of parameters
    json_t* options;       // Key-value options (jansson object)
} TdzCommand;
```

**Memory Ownership**:
- `command`, `target`: Owned, freed by `tdz_command_free()`
- `parameters`: Array and elements owned, freed by `tdz_command_free()`
- `options`: jansson object, reference counted

### TdzCommandVector Structure
```c
typedef struct {
    TdzCommand* commands;  // Dynamic array of commands
    size_t count;          // Current number of commands
    size_t capacity;       // Current capacity of the array
} TdzCommandVector;
```

**Growth Strategy**: Capacity doubles when full (initial capacity: 8)

### MemoryStruct Structure (CURL)
```c
struct MemoryStruct {
    char* memory;          // Dynamically allocated response buffer
    size_t size;           // Current size of buffer
};
```

## Functions

### Core API Functions

#### `parse_tdz_command`
```c
TdzCommandVector* parse_tdz_command(const char* text, TdzError** error);
```

**Purpose**: Parse TDZ commands from input text

**Parameters**:
- `text`: Input text containing `<tdz>...</tdz>` tags
- `error`: Double pointer to receive error details (optional)

**Returns**: 
- `TdzCommandVector*`: Parsed commands (caller must free)
- `NULL`: On error, with error details populated

**Error Conditions**:
- Memory allocation failure
- Malformed command syntax
- Invalid tag structure

#### `execute_tdz_command`
```c
TdzError* execute_tdz_command(TdzCommand* command, const char* base_url, 
                             const char* api_key, json_t** result);
```

**Purpose**: Execute a single TDZ command against the API

**Parameters**:
- `command`: Command to execute
- `base_url`: Base URL of Todozi API (e.g., "https://api.todozi.com")
- `api_key`: API key for authentication (optional)
- `result`: Pointer to receive JSON result (optional)

**Returns**:
- `NULL`: Success
- `TdzError*`: Error details on failure

**HTTP Methods Supported**:
- GET: list, get, search
- POST: create, run, execute  
- PUT: update
- DELETE: delete

#### `process_tdz_commands`
```c
TdzError* process_tdz_commands(const char* text, const char* base_url,
                              const char* api_key, json_t** results);
```

**Purpose**: High-level function to parse and execute all commands in text

**Parameters**:
- `text`: Input text containing TDZ commands
- `base_url`: API base URL
- `api_key`: Authentication key
- `results`: Array of JSON results from all commands

**Returns**:
- `NULL`: All commands executed successfully
- `TdzError*`: First error encountered

### Helper Functions

#### Memory Management
```c
TdzCommand* tdz_command_new(void);
void tdz_command_free(TdzCommand* cmd);

TdzCommandVector* tdz_command_vector_new(void);
void tdz_command_vector_free(TdzCommandVector* vec);

TdzError* tdz_error_new(TdzErrorType type, const char* message);
void tdz_error_free(TdzError* error);

TdzError* tdz_command_vector_add(TdzCommandVector* vec, TdzCommand* cmd);
```

#### String Utilities
```c
static char* string_duplicate(const char* str);
static void string_to_lowercase(char* str);
```

#### Path Resolution
```c
char* find_todozi(const char* str);
```

### Internal Helper Functions

#### `get_endpoint_path`
```c
char* get_endpoint_path(TdzCommand* command);
```

**Purpose**: Map command/target to API endpoint path

**Endpoint Mapping Examples**:
- `list tasks` → `/tasks`
- `get task` → `/tasks/{id}`
- `create memory` → `/memories`

#### Request Body Builders
```c
json_t* build_request_body(TdzCommand* command);
json_t* build_run_body(TdzCommand* command);
```

**Supported Targets**:
- task, memory, idea, agent, chunk, project, feeling, training

#### CURL Callback
```c
static size_t WriteMemoryCallback(void* contents, size_t size, 
                                 size_t nmemb, struct MemoryStruct* mem);
```

**Purpose**: Accumulate HTTP response data in memory buffer

## Usage Examples

### Basic Example: Single Command
```c
#include "tdz_client.h"

int main() {
    const char* text = "<tdz>list;tasks</tdz>";
    const char* base_url = "https://api.todozi.com";
    const char* api_key = "your-api-key";
    
    json_t* results = NULL;
    TdzError* error = process_tdz_commands(text, base_url, api_key, &results);
    
    if (error) {
        printf("Error: %s (type: %d)\n", error->message, error->type);
        tdz_error_free(error);
        return 1;
    }
    
    // Process results
    if (results && json_is_array(results)) {
        size_t index;
        json_t* value;
        json_array_foreach(results, index, value) {
            char* result_str = json_dumps(value, JSON_INDENT(2));
            printf("Result %zu: %s\n", index, result_str);
            free(result_str);
        }
    }
    
    if (results) json_decref(results);
    return 0;
}
```

### Advanced Example: Multiple Commands with Error Handling
```c
#include "tdz_client.h"

void process_complex_commands() {
    const char* text = 
        "Here's my todo list:\n"
        "<tdz>create;task;action=Buy groceries;priority=high</tdz>\n"
        "<tdz>list;tasks</tdz>\n"
        "<tdz>create;memory;moment=Lunch with team;importance=medium</tdz>";
    
    TdzCommandVector* commands = parse_tdz_command(text, NULL);
    if (!commands) {
        printf("Failed to parse commands\n");
        return;
    }
    
    for (size_t i = 0; i < commands->count; i++) {
        TdzCommand* cmd = &commands->commands[i];
        json_t* result = NULL;
        TdzError* error = execute_tdz_command(cmd, "https://api.todozi.com", 
                                            "api-key-123", &result);
        
        if (error) {
            printf("Command %zu failed: %s\n", i, error->message);
            tdz_error_free(error);
        } else {
            printf("Command %zu succeeded\n", i);
            if (result) {
                // Process result
                json_decref(result);
            }
        }
    }
    
    tdz_command_vector_free(commands);
}
```

### Command Syntax Examples

#### Task Management
```c
// Create task
"<tdz>create;task;action=Complete project;priority=high;project=work</tdz>"

// List tasks  
"<tdz>list;tasks</tdz>"

// Update task
"<tdz>update;task;123;status=completed</tdz>"

// Search tasks
"<tdz>search;tasks;urgent</tdz>"
```

#### Memory Management
```c
// Create memory
"<tdz>create;memory;moment=Team meeting;importance=high;memory_type=short</tdz>"

// List memories by type
"<tdz>list;memories_short</tdz>"
```

#### Agent Operations
```c
// Create agent
"<tdz>create;agent;name=Research Bot;capabilities=search,analyze</tdz>"

// Run agent
"<tdz>run;agent;agent123;message=Research AI trends</tdz>"
```

## Design Patterns

### 1. Resource Acquisition Is Initialization (RAII)
```c
// Pattern: Automatic cleanup with dedicated free functions
TdzCommand* cmd = tdz_command_new();
if (!cmd) { /* handle error */ }
// ... use command ...
tdz_command_free(cmd);  // Automatic cleanup
```

### 2. Factory Pattern
```c
// Pattern: Factory functions for object creation
TdzError* error = tdz_error_new(TDZ_ERROR_NETWORK, "Connection failed");
TdzCommandVector* vec = tdz_command_vector_new();
```

### 3. Strategy Pattern
```c
// Pattern: Different strategies for different command types
if (strcmp(cmd->command, "create") == 0) {
    body = build_request_body(cmd);  // Strategy for create
} else if (strcmp(cmd->command, "run") == 0) {
    body = build_run_body(cmd);      // Strategy for run
}
```

### 4. Command Pattern
```c
// Pattern: Encapsulating operations as command objects
TdzCommand cmd = {
    .command = "create",
    .target = "task",
    .parameters = /* ... */,
    .options = /* ... */
};
execute_tdz_command(&cmd, base_url, api_key, &result);
```

### 5. Iterator Pattern
```c
// Pattern: Iterating through command vector
for (size_t i = 0; i < commands->count; i++) {
    TdzCommand* cmd = &commands->commands[i];
    process_command(cmd);
}
```

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Command Parsing | O(n) | Linear to input text size |
| Command Vector Add | O(1) amortized | Dynamic array doubling |
| Endpoint Lookup | O(1) | Constant-time hash lookup ideal |
| JSON Processing | O(n) | Linear to JSON size |

### Memory Usage
| Component | Memory Footprint | Notes |
|-----------|------------------|-------|
| Command Structure | ~100-500 bytes | Depends on parameters |
| Command Vector | 8 + n*sizeof(TdzCommand) | Grows dynamically |
| HTTP Response | Variable | Depends on API response size |

### Optimization Opportunities
1. **String Interning**: Reduce duplicate string allocations
2. **Object Pooling**: Reuse command structures
3. **Response Streaming**: Process large responses incrementally
4. **Connection Pooling**: Reuse HTTP connections

### Memory Safety Features
- Bounds checking in string operations
- Overflow protection in arithmetic
- Null pointer validation
- Resource cleanup guarantees

## Security Considerations

### Input Validation
```c
// Example: Safe string duplication with validation
static char* string_duplicate(const char* str) {
    if (!str) return NULL;  // Null check
    size_t len = strlen(str);
    char* new_str = malloc(len + 1);
    if (!new_str) return NULL;
    memcpy(new_str, str, len);  // Safe copy with known length
    new_str[len] = '\0';
    return new_str;
}
```

### API Security
1. **Authentication**: API key validation and header injection
2. **HTTPS Enforcement**: Recommend TLS 1.2+ for production
3. **Input Sanitization**: Command parameter validation
4. **Error Message Security**: Avoid leaking sensitive information

### Memory Safety
- Buffer overflow protection
- Integer overflow checks
- Double-free prevention
- Use-after-free mitigation

### Security Best Practices
```c
// Safe string operations
int written = snprintf(buffer, buffer_size, format, args);
if (written < 0 || (size_t)written >= buffer_size) {
    // Handle truncation or error
}

// Safe memory allocation with overflow check
if (realsize / size != nmemb) {
    return 0;  // Overflow detected
}
```

## Testing Strategy

### Unit Testing Framework
Recommended using CUnit or Check framework:

```c
// Example test case for command parsing
void test_parse_basic_command(void) {
    const char* text = "<tdz>list;tasks</tdz>";
    TdzError* error = NULL;
    TdzCommandVector* commands = parse_tdz_command(text, &error);
    
    CU_ASSERT_PTR_NOT_NULL(commands);
    CU_ASSERT(commands->count == 1);
    CU_ASSERT_STRING_EQUAL(commands->commands[0].command, "list");
    CU_ASSERT_STRING_EQUAL(commands->commands[0].target, "tasks");
    
    tdz_command_vector_free(commands);
    if (error) tdz_error_free(error);
}
```

### Test Categories

#### 1. Parser Tests
- Valid command extraction
- Malformed tag handling
- Multiple command parsing
- Parameter/option parsing

#### 2. Endpoint Mapping Tests
- All command/target combinations
- Parameter substitution
- Special cases (update feeling)

#### 3. HTTP Integration Tests
- Successful requests
- Error responses
- Authentication failures
- Network timeouts

#### 4. Memory Safety Tests
- Allocation failure recovery
- Buffer overflow scenarios
- Cleanup verification

### Mock Testing Strategy
```c
// Mock CURL for unit testing
typedef CURL* (*curl_easy_init_mock)(void);
typedef CURLcode (*curl_easy_perform_mock)(CURL*);

// Inject mock functions during testing
void set_curl_mocks(curl_easy_init_mock init_mock, 
                   curl_easy_perform_mock perform_mock);
```

## Deployment Instructions

### Build Requirements
```bash
# Ubuntu/Debian
sudo apt-get install libcurl4-openssl-dev libjansson-dev

# CentOS/RHEL
sudo yum install libcurl-devel jansson-devel

# macOS with Homebrew
brew install curl jansson
```

### Compilation
```makefile
# Makefile example
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -pedantic -O2
LIBS = -lcurl -ljansson
INCLUDES = -I/usr/local/include

tdz_client: tdz_client.c
	$(CC) $(CFLAGS) $(INCLUDES) tdz_client.c -o tdz_client $(LIBS)

clean:
	rm -f tdz_client
```

### Integration Example
```c
// main.c - Example integration
#include "tdz_client.h"
#include <stdio.h>

int main(int argc, char* argv[]) {
    if (argc < 3) {
        fprintf(stderr, "Usage: %s <base_url> <api_key> [input_file]\n", argv[0]);
        return 1;
    }
    
    const char* base_url = argv[1];
    const char* api_key = argv[2];
    const char* input_file = argc > 3 ? argv[3] : NULL;
    
    char* text = NULL;
    if (input_file) {
        // Read from file
        FILE* f = fopen(input_file, "r");
        if (!f) {
            perror("Failed to open input file");
            return 1;
        }
        fseek(f, 0, SEEK_END);
        long size = ftell(f);
        fseek(f, 0, SEEK_SET);
        text = malloc(size + 1);
        fread(text, 1, size, f);
        text[size] = '\0';
        fclose(f);
    } else {
        // Read from stdin
        text = read_stdin();
    }
    
    json_t* results = NULL;
    TdzError* error = process_tdz_commands(text, base_url, api_key, &results);
    
    if (error) {
        fprintf(stderr, "Error: %s\n", error->message);
        tdz_error_free(error);
        free(text);
        return 1;
    }
    
    // Output results as JSON
    char* output = json_dumps(results, JSON_INDENT(2));
    printf("%s\n", output);
    
    free(output);
    if (results) json_decref(results);
    free(text);
    return 0;
}
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. Compilation Errors
**Problem**: Missing libraries
```bash
# Error: curl/curl.h: No such file or directory
sudo apt-get install libcurl4-openssl-dev  # Ubuntu
sudo yum install libcurl-devel            # CentOS
```

**Problem**: Linker errors
```bash
# Error: undefined reference to `curl_easy_init'
# Ensure libraries are linked in correct order
gcc program.c -lcurl -ljansson  # Correct order
```

#### 2. Runtime Errors

**Memory Allocation Failures**
```c
// Check system memory limits
// Use valgrind for memory leak detection
valgrind --leak-check=full ./tdz_client
```

**Network Connection Issues**
```c
// Verify base_url format
// Check API key validity
// Test network connectivity
curl -I https://api.todozi.com/health
```

**JSON Parsing Errors**
```c
// Enable jansson error reporting
json_error_t error;
json_t* root = json_loads(response, 0, &error);
if (!root) {
    fprintf(stderr, "JSON error: %s\n", error.text);
}
```

#### 3. Performance Issues

**High Memory Usage**
- Monitor with tools like `top` or `htop`
- Check for memory leaks with valgrind
- Consider response size limits

**Slow Execution**
- Enable CURL verbose mode for debugging
- Check network latency
- Profile with `gprof` or `perf`

### Debugging Techniques

#### Verbose Logging
```c
// Enable CURL verbose output
curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);

// Add custom debug logging
#ifdef DEBUG
#define DBG printf
#else
#define DBG(...)
#endif
```

#### Error Recovery
```c
// Comprehensive error handling example
TdzError* error = execute_tdz_command(cmd, base_url, api_key, &result);
switch (error ? error->type : TDZ_ERROR_NONE) {
    case TDZ_ERROR_NONE:
        // Success
        break;
    case TDZ_ERROR_NETWORK:
        // Retry logic
        break;
    case TDZ_ERROR_VALIDATION:
        // User input error
        break;
    default:
        // Generic error handling
        break;
}
```

### Monitoring and Logging

#### Integration with Logging Systems
```c
// Example syslog integration
#include <syslog.h>

void log_tdz_error(TdzError* error) {
    syslog(LOG_ERR, "TDZ Error [%d]: %s", error->type, error->message);
}

// Example file logging
void log_to_file(const char* message) {
    FILE* logfile = fopen("/var/log/tdz_client.log", "a");
    if (logfile) {
        fprintf(logfile, "%s\n", message);
        fclose(logfile);
    }
}
```

This comprehensive documentation provides complete coverage of the TDZ API client library, including architectural overview, detailed function specifications, usage examples, security considerations, and troubleshooting guidance. The library demonstrates robust C programming practices with careful attention to memory safety, error handling, and API integration.