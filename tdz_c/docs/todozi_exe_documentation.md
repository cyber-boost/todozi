# Todozi C Executor - Comprehensive Documentation

## Overview

The Todozi C Executor is a comprehensive C library for interacting with the Todozi productivity and organization system. It provides a rich API for task management, memory storage, idea generation, and AI-assisted planning through HTTP API integration.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Data Structures](#data-structures)
3. [Core Functions](#core-functions)
4. [Usage Examples](#usage-examples)
5. [Design Patterns](#design-patterns)
6. [Performance Analysis](#performance-analysis)
7. [Security Considerations](#security-considerations)
8. [Testing Strategies](#testing-strategies)
9. [Deployment Instructions](#deployment-instructions)
10. [Troubleshooting Guide](#troubleshooting-guide)

---

## Architecture Overview

### System Architecture Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Application   │────│  Todozi Executor │────│  Todozi API     │
│    Layer        │    │    (C Library)   │    │   (HTTP/REST)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   JSON-C        │    │   libcurl        │    │   Environment   │
│  (Parsing)      │    │  (HTTP Client)   │    │   Variables     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Component Flow

```
User Request → Parameter Validation → Action Routing → API Call → Response Processing → Result Packaging
```

## Data Structures

### ExecutorErrorType Enum

**Purpose**: Defines error categories for the executor system

```c
typedef enum {
    EXECUTOR_ERROR_EXECUTION,        // General execution failure
    EXECUTOR_ERROR_BASH_TOOL,        // Bash tool execution error
    EXECUTOR_ERROR_MISSING_PARAMETER, // Required parameter missing
    EXECUTOR_ERROR_UNKNOWN_ACTION    // Unrecognized action type
} ExecutorErrorType;
```

### ExecutorError Structure

**Purpose**: Encapsulates error information with type and message

```c
typedef struct {
    ExecutorErrorType type;  // Error category
    char* message;           // Human-readable error description
} ExecutorError;
```

**Memory Management**: Requires explicit freeing using `free_executor_error()`

### ExecutionResult Structure

**Purpose**: Contains the outcome of executed operations

```c
typedef struct {
    bool success;           // Operation success status
    char* output;           // Primary result/output data
    char* error;            // Error message (if any)
    char* tool_used;        // Tool/interface identifier
    char* execution_type;   // Type of execution performed
} ExecutionResult;
```

**Memory Management**: Requires explicit freeing using `free_execution_result()`

## Core Functions

### Memory Management Functions

#### `free_execution_result(ExecutionResult* result)`

**Purpose**: Safely deallocates an ExecutionResult structure and all its string fields

**Parameters**:
- `result`: Pointer to ExecutionResult to free (nullable)

**Return**: void

**Usage**:
```c
ExecutionResult* res = execute_simple_task(params);
// ... use result ...
free_execution_result(res);  // Clean up
```

#### `create_executor_error(ExecutorErrorType type, const char* message)`

**Purpose**: Creates a new ExecutorError instance with specified type and message

**Parameters**:
- `type`: Error category from ExecutorErrorType enum
- `message`: Error description string

**Return**: Pointer to allocated ExecutorError, or NULL on allocation failure

**Memory**: Caller must free with `free_executor_error()`

#### `free_executor_error(ExecutorError* error)`

**Purpose**: Deallocates an ExecutorError structure

**Parameters**:
- `error`: Pointer to ExecutorError to free (nullable)

**Return**: void

### Helper Functions

#### `create_execution_result(void)`

**Purpose**: Creates and initializes a new ExecutionResult with safe defaults

**Return**: Pointer to zero-initialized ExecutionResult, or NULL on failure

**Internal Use**: Used internally by execution functions

#### `set_result_string(char** dest, const char* src)`

**Purpose**: Safely sets string fields in ExecutionResult with memory management

**Parameters**:
- `dest`: Pointer to destination string pointer
- `src`: Source string to copy (nullable)

**Return**: bool indicating success

**Features**:
- Handles NULL source strings
- Frees existing destination memory before assignment
- Returns false on allocation failure

### System Initialization Functions

#### `ensure_todozi_system()`

**Purpose**: Initializes the Todozi system and required dependencies

**Return**: ExecutorError* on failure, NULL on success

**Responsibilities**:
- Initializes libcurl globally (thread-safe)
- Sets system initialization flag
- Returns error if curl initialization fails

#### `get_todozi_api_key()`

**Purpose**: Retrieves API key from environment variable with caching

**Return**: const char* pointing to cached API key, or NULL if not set

**Environment Variable**: `TODOZI_API_KEY`

**Caching**: First call caches the key for subsequent use

### HTTP Communication Functions

#### `write_callback(void* contents, size_t size, size_t nmemb, struct http_response* response)`

**Purpose**: libcurl write callback for accumulating HTTP response data

**Parameters**:
- `contents`: Incoming data buffer
- `size`: Size of each data element
- `nmemb`: Number of elements
- `response`: http_response structure to accumulate data

**Return**: size_t of bytes processed

#### `make_todozi_request(const char* endpoint, json_object* payload)`

**Purpose**: Makes authenticated HTTP request to Todozi API

**Parameters**:
- `endpoint`: API endpoint path (e.g., "/api/todozi/extract")
- `payload`: JSON object containing request data

**Return**: json_object* containing parsed response, or NULL on error

**Features**:
- Automatic authentication header construction
- 30-second timeout, 10-second connect timeout
- JSON response parsing
- Error handling for various failure scenarios

### Parameter Extraction Function

#### `get_string_param(json_object* params, const char* key)`

**Purpose**: Extracts string parameter from JSON object

**Parameters**:
- `params`: JSON object containing parameters
- `key`: Parameter key to extract

**Return**: const char* to string value, or NULL if not found/invalid type

### Action Execution Functions

The library provides 20+ action-specific execution functions, each following this pattern:

#### Pattern Template
```c
ExecutionResult* execute_[action](json_object* params) {
    // 1. Parameter validation
    // 2. Business logic execution
    // 3. Result packaging
    // 4. Memory-safe return
}
```

#### Available Actions:
- **Task Management**: `execute_simple_task`, `execute_urgent_task`, `execute_high_task`, `execute_low_task`
- **Assignment Types**: `execute_ai_task`, `execute_human_task`, `execute_collab_task`
- **Search Operations**: `execute_find`, `execute_ai_search`, `execute_fast_search`, `execute_smart_search`
- **Memory Operations**: `execute_remember`, `execute_important_memory`
- **Idea Management**: `execute_idea`, `execute_breakthrough_idea`
- **Task Lifecycle**: `execute_complete`, `execute_start`
- **System Operations**: `execute_stats`, `execute_queue`, `execute_chat`
- **AI APIs**: `execute_extract_api`, `execute_expand_api`, `execute_plan_api`, `execute_strategy_api`

### Main Dispatcher Function

#### `execute_todozi_tool_delegated(json_object* params)`

**Purpose**: Main entry point that routes actions to appropriate execution functions

**Parameters**:
- `params`: JSON object containing "action" and other parameters

**Return**: ExecutionResult* with operation outcome

**Routing Logic**: Uses string comparison on "action" parameter to select handler

### Cleanup Function

#### `cleanup_todozi_executor()`

**Purpose**: Global cleanup of system resources

**Responsibilities**:
- Frees cached API key
- Cleans up libcurl global state
- Resets initialization flags

**Usage**: Should be called once at application shutdown

## Usage Examples

### Basic Example: Creating a Simple Task

```c
#include "todozi_executor.h"
#include <stdio.h>

int main() {
    // Create parameters JSON
    json_object* params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("task"));
    json_object_object_add(params, "content", json_object_new_string("Finish documentation"));
    
    // Execute the action
    ExecutionResult* result = execute_todozi_tool_delegated(params);
    
    if (result && result->success) {
        printf("Success: %s\n", result->output);
        printf("Tool: %s, Type: %s\n", result->tool_used, result->execution_type);
    } else {
        printf("Operation failed\n");
    }
    
    // Cleanup
    if (result) free_execution_result(result);
    json_object_put(params);
    cleanup_todozi_executor();
    
    return 0;
}
```

### Advanced Example: AI Task Extraction

```c
#include "todozi_executor.h"
#include <stdio.h>

int main() {
    // Set API key (should be in environment)
    setenv("TODOZI_API_KEY", "your-api-key-here", 1);
    
    // Create extraction parameters
    json_object* params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("extract"));
    json_object_object_add(params, "content", 
        json_object_new_string("I need to buy groceries and finish the report by Friday"));
    json_object_object_add(params, "extra", 
        json_object_new_string("Personal task management"));
    
    // Execute extraction
    ExecutionResult* result = execute_todozi_tool_delegated(params);
    
    if (result) {
        if (result->success) {
            printf("Extraction Results:\n%s\n", result->output);
        } else {
            printf("Error: %s\n", result->error ? result->error : "Unknown error");
        }
        free_execution_result(result);
    }
    
    json_object_put(params);
    cleanup_todozi_executor();
    return 0;
}
```

### Error Handling Example

```c
#include "todozi_executor.h"
#include <stdio.h>

void handle_executor_error(ExecutorError* error) {
    if (error) {
        switch (error->type) {
            case EXECUTOR_ERROR_MISSING_PARAMETER:
                printf("Parameter error: %s\n", error->message);
                break;
            case EXECUTOR_ERROR_UNKNOWN_ACTION:
                printf("Unknown action: %s\n", error->message);
                break;
            default:
                printf("Execution error: %s\n", error->message);
        }
        free_executor_error(error);
    }
}

int main() {
    // Test missing parameter
    json_object* params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("task"));
    // Missing "content" parameter
    
    ExecutionResult* result = execute_todozi_tool_delegated(params);
    
    if (!result) {
        printf("Operation returned NULL (likely parameter error)\n");
    }
    
    json_object_put(params);
    cleanup_todozi_executor();
    return 0;
}
```

### Batch Operations Example

```c
#include "todozi_executor.h"
#include <stdio.h>

void process_multiple_actions(const char* actions[], int count) {
    for (int i = 0; i < count; i++) {
        json_object* params = json_object_new_object();
        json_object_object_add(params, "action", json_object_new_string(actions[i]));
        json_object_object_add(params, "content", 
            json_object_new_string("Sample content for action"));
        
        ExecutionResult* result = execute_todozi_tool_delegated(params);
        
        if (result && result->success) {
            printf("Action %d (%s): SUCCESS\n", i+1, actions[i]);
        } else {
            printf("Action %d (%s): FAILED\n", i+1, actions[i]);
        }
        
        if (result) free_execution_result(result);
        json_object_put(params);
    }
}

int main() {
    const char* actions[] = {"task", "urgent", "remember", "idea"};
    process_multiple_actions(actions, 4);
    cleanup_todozi_executor();
    return 0;
}
```

## Design Patterns

### 1. Factory Pattern

**Implementation**: Action routing in `execute_todozi_tool_delegated()`
**Purpose**: Creates appropriate execution objects based on action type
**Benefits**: Easy extension, separation of concerns

### 2. Strategy Pattern

**Implementation**: Each action has its own execution function
**Purpose**: Encapsulates different algorithms for different actions
**Benefits**: Modularity, easier testing and maintenance

### 3. Resource Acquisition Is Initialization (RAII)

**Implementation**: Memory management helper functions
**Purpose**: Ensures proper cleanup of allocated resources
**Benefits**: Prevents memory leaks, consistent resource management

### 4. Facade Pattern

**Implementation**: `execute_todozi_tool_delegated()` as unified interface
**Purpose**: Simplifies complex subsystem interaction
**Benefits**: Easy-to-use API, hides implementation complexity

### 5. Singleton Pattern (Global State)

**Implementation**: Global flags (`tdz_system_initialized`, `curl_initialized`)
**Purpose**: Single instance management of system resources
**Benefits**: Resource optimization, consistent state

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Action Routing | O(n) | Linear search through action strings |
| Parameter Extraction | O(1) | HashMap lookup in JSON object |
| HTTP Request | O(1) | Constant time relative to data size |
| Memory Operations | O(n) | Linear with string length |

### Space Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| ExecutionResult | O(n) | Proportional to output size |
| HTTP Response | O(n) | Proportional to response data |
| JSON Parsing | O(n) | Proportional to JSON size |

### Memory Usage Patterns

- **Stack Usage**: Minimal (function calls, small locals)
- **Heap Usage**: Significant (strings, JSON objects, HTTP buffers)
- **Peak Memory**: During large API responses

### Optimization Opportunities

1. **String Pooling**: Reuse common strings (tool names, types)
2. **JSON Recycling**: Reuse JSON objects for similar requests
3. **Connection Pooling**: Reuse HTTP connections (currently not implemented)
4. **Buffer Reuse**: Reuse memory buffers for similar operations

## Security Considerations

### Authentication Security

**API Key Management**:
- Stored in environment variable `TODOZI_API_KEY`
- Cached in memory after first retrieval
- Zeroed on cleanup
- Never logged or exposed in error messages

**HTTP Security**:
- Bearer token authentication
- HTTPS mandatory (hardcoded)
- No sensitive data in URLs

### Input Validation

**JSON Parameter Validation**:
- Type checking for all parameters
- Null checks for required parameters
- Length limits on string parameters
- Malformed JSON handling

**Buffer Safety**:
- Bounded string operations using `snprintf()`
- Dynamic buffer allocation with size checks
- Overflow protection in all string operations

### Memory Security

**Allocation Safety**:
- Null checks after all malloc operations
- Proper cleanup on allocation failure
- No double-free vulnerabilities

**Sensitive Data Handling**:
- API key stored in secure memory
- No persistence of sensitive data
- Secure cleanup on shutdown

### Network Security

**TLS/SSL**: 
- libcurl handles TLS automatically
- Certificate verification enabled by default
- No custom certificate bypass

**Timeout Protection**:
- 30-second operation timeout
- 10-second connection timeout
- Prevents hanging requests

## Testing Strategies

### Unit Testing Framework

```c
// Example test structure
typedef struct {
    const char* test_name;
    bool (*test_function)(void);
    bool expected_result;
} TestCase;

bool test_simple_task_creation() {
    json_object* params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("task"));
    json_object_object_add(params, "content", json_object_new_string("test"));
    
    ExecutionResult* result = execute_simple_task(params);
    bool success = result != NULL && result->success;
    
    if (result) free_execution_result(result);
    json_object_put(params);
    return success;
}
```

### Test Categories

#### 1. Functional Tests
- Parameter validation tests
- Action routing tests
- Success/failure path tests

#### 2. Integration Tests
- HTTP API interaction tests
- JSON parsing tests
- End-to-end workflow tests

#### 3. Performance Tests
- Memory usage under load
- Response time measurements
- Concurrent operation tests

#### 4. Security Tests
- Input validation tests
- Authentication failure tests
- Memory safety tests

### Test Data Management

**Mock HTTP Responses**:
```c
// Mock curl easy perform for testing
CURLcode mock_curl_easy_perform(CURL* curl) {
    // Return mock response data
    return CURLE_OK;
}
```

**Test Fixtures**:
```c
typedef struct {
    json_object* valid_params;
    json_object* invalid_params;
    const char* expected_output;
} TestFixture;
```

## Deployment Instructions

### Prerequisites

#### Required Libraries
```bash
# Ubuntu/Debian
sudo apt-get install libcurl4-openssl-dev libjson-c-dev

# CentOS/RHEL
sudo yum install libcurl-devel json-c-devel

# macOS with Homebrew
brew install curl json-c
```

#### Development Tools
- GCC or Clang compiler
- Make or CMake
- Git for version control

### Build Instructions

#### Simple Makefile
```makefile
CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -O2
LIBS = -lcurl -ljson-c
SRC = todozi_executor.c
OBJ = $(SRC:.c=.o)
TARGET = libtodozi.a

$(TARGET): $(OBJ)
	ar rcs $(TARGET) $(OBJ)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: clean
```

#### CMake Build
```cmake
cmake_minimum_required(VERSION 3.10)
project(todozi_executor)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_FLAGS "-Wall -Wextra")

find_package(PkgConfig REQUIRED)
pkg_check_modules(CURL REQUIRED libcurl)
pkg_check_modules(JSON_C REQUIRED json-c)

add_library(todozi_executor STATIC todozi_executor.c)
target_link_libraries(todozi_executor ${CURL_LIBRARIES} ${JSON_C_LIBRARIES})
target_include_directories(todozi_executor PUBLIC ${CURL_INCLUDE_DIRS} ${JSON_C_INCLUDE_DIRS})
```

### Installation Steps

1. **Clone or Download Source**
```bash
git clone <repository-url>
cd todozi-executor
```

2. **Build Library**
```bash
make
```

3. **Install Headers and Library**
```bash
sudo cp todozi_executor.h /usr/local/include/
sudo cp libtodozi.a /usr/local/lib/
```

4. **Verify Installation**
```bash
# Create test program
echo '#include "todozi_executor.h"
int main() { return 0; }' > test.c
gcc test.c -ltodozi -lcurl -ljson-c -o test
./test
```

### Configuration

#### Environment Setup
```bash
# Set API key
export TODOZI_API_KEY="your-secret-api-key"

# Optional: Set custom API endpoint
export TODOZI_API_BASE="https://api.todozi.com"
```

#### Integration with Applications
```c
// In your application code
#include <todozi_executor.h>

// Link with: -ltodozi -lcurl -ljson-c
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. Compilation Errors

**Issue**: Missing library headers
```
error: curl/curl.h: No such file or directory
```
**Solution**: Install development packages
```bash
sudo apt-get install libcurl4-openssl-dev libjson-c-dev
```

**Issue**: Linker errors
```
undefined reference to `curl_global_init'
```
**Solution**: Ensure proper linking order
```bash
gcc program.c -ltodozi -lcurl -ljson-c -o program
```

#### 2. Runtime Errors

**Issue**: API key not found
```
Error: API key required but not set
```
**Solution**: Set environment variable
```bash
export TODOZI_API_KEY="your-key"
```

**Issue**: Network connectivity
```
Timeout or connection refused
```
**Solution**: Check network, verify API endpoint

#### 3. Memory Issues

**Issue**: Memory leaks
**Detection**: Use valgrind
```bash
valgrind --leak-check=full ./your_program
```
**Solution**: Ensure all allocated memory is properly freed

**Issue**: Segmentation faults
**Debugging**: Use gdb with debug symbols
```bash
gcc -g program.c -ltodozi -lcurl -ljson-c -o program
gdb ./program
```

### Debugging Techniques

#### Logging Support
```c
// Add debug logging
#define TODOZI_DEBUG 1

#if TODOZI_DEBUG
#define DEBUG_PRINT(fmt, ...) printf("DEBUG: " fmt, ##__VA_ARGS__)
#else
#define DEBUG_PRINT(fmt, ...)
#endif
```

#### Error Tracing
```c
void trace_execution(const char* function, json_object* params) {
    DEBUG_PRINT("Entering %s\n", function);
    if (params) {
        DEBUG_PRINT("Params: %s\n", json_object_to_json_string(params));
    }
}
```

### Performance Monitoring

#### Memory Profiling
```bash
# Monitor memory usage
valgrind --tool=massif ./your_program
ms_print massif.out.*
```

#### Execution Timing
```c
#include <time.h>

clock_t start = clock();
ExecutionResult* result = execute_todozi_tool_delegated(params);
clock_t end = clock();
double elapsed = ((double)(end - start)) / CLOCKS_PER_SEC;
printf("Execution time: %f seconds\n", elapsed);
```

### Recovery Strategies

#### Graceful Degradation
```c
ExecutionResult* execute_with_fallback(json_object* params) {
    ExecutionResult* result = execute_todozi_tool_delegated(params);
    if (!result || !result->success) {
        // Implement fallback logic
        return create_fallback_result(params);
    }
    return result;
}
```

#### Retry Mechanism
```c
ExecutionResult* execute_with_retry(json_object* params, int max_retries) {
    for (int i = 0; i < max_retries; i++) {
        ExecutionResult* result = execute_todozi_tool_delegated(params);
        if (result && result->success) {
            return result;
        }
        if (result) free_execution_result(result);
        sleep(1 << i); // Exponential backoff
    }
    return NULL; // All retries failed
}
```

This comprehensive documentation provides complete coverage of the Todozi C Executor library, including architecture, usage, security, testing, deployment, and troubleshooting. The library offers a robust foundation for integrating Todozi functionality into C applications with proper error handling, memory management, and security considerations.