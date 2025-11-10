# Comprehensive Documentation: Tool Framework System

## Overview

This C-based tool framework provides a comprehensive system for defining, registering, and executing various tools with parameter validation, error handling, and JSON serialization capabilities. The system is designed to be extensible and supports multiple resource locking mechanisms for safe concurrent operations.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Data Structures](#data-structures)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Design Patterns](#design-patterns)
6. [Usage Examples](#usage-examples)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Architecture Overview

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Tool Framework System                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │ ToolRegistry│    │    Tool     │    │   HashMap   │     │
│  │             │    │             │    │             │     │
│  │ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │     │
│  │ │ tools   │◄├────┼─│ data    │ │    │ │ buckets │ │     │
│  │ │ HashMap │ │    │ │ void*   │ │    │ │ Entry*  │ │     │
│  │ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │     │
│  └─────────────┘    │ ┌─────────┐ │    │             │     │
│                     │ │ def_fn  │ │    │             │     │
│  ┌─────────────┐    │ │ exec_fn │ │    └─────────────┘     │
│  │ToolDefinition│   │ │ ...     │ │                        │
│  │             │   │ └─────────┘ │    ┌─────────────┐     │
│  │ ┌─────────┐ │   └─────────────┘    │  JsonValue  │     │
│  │ │ params  │ │                      │             │     │
│  │ │ToolParam│ │                      │ ┌─────────┐ │     │
│  │ │ array   │ │                      │ │ type    │ │     │
│  │ └─────────┘ │                      │ │ union   │ │     │
│  └─────────────┘                      │ │ refcount│ │     │
│                                       │ └─────────┘ │     │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships

```
ToolRegistry (1) ──contains─── (0..*) Tool
Tool (1) ──has─── (1) ToolDefinition
ToolDefinition (1) ──contains─── (0..*) ToolParameter
Tool (1) ──produces─── (0..*) ToolResult
ToolResult (1) ──may contain─── (0..1) ToolError
HashMap (1) ──stores─── (0..*) Key-Value Pairs
JsonValue (1) ──represents─── (1) JSON Data Type
```

## Data Structures

### Enum Definitions

#### ResourceLock Enum
```c
typedef enum {
    RESOURCE_LOCK_FILESYSTEM_WRITE,
    RESOURCE_LOCK_FILESYSTEM_READ,
    RESOURCE_LOCK_GIT,
    RESOURCE_LOCK_MEMORY,
    RESOURCE_LOCK_SHELL,
    RESOURCE_LOCK_NETWORK
} ResourceLock;
```

**Purpose**: Defines system resources that tools may need to lock for exclusive access.

**Values**:
- `RESOURCE_LOCK_FILESYSTEM_WRITE`: Write operations on filesystem
- `RESOURCE_LOCK_FILESYSTEM_READ`: Read operations on filesystem  
- `RESOURCE_LOCK_GIT`: Git repository operations
- `RESOURCE_LOCK_MEMORY`: Memory-intensive operations
- `RESOURCE_LOCK_SHELL`: Shell command execution
- `RESOURCE_LOCK_NETWORK`: Network operations

#### ErrorType Enum
```c
typedef enum {
    ERROR_TYPE_VALIDATION_ERROR,
    ERROR_TYPE_PERMISSION_ERROR,
    ERROR_TYPE_FILE_NOT_FOUND,
    ERROR_TYPE_TIMEOUT_ERROR,
    ERROR_TYPE_RESOURCE_ERROR,
    ERROR_TYPE_NETWORK_ERROR,
    ERROR_TYPE_SECURITY_ERROR,
    ERROR_TYPE_INTERNAL_ERROR
} ErrorType;
```

**Purpose**: Categorizes different types of errors that can occur during tool execution.

### Core Structures

#### HashMapEntry
```c
typedef struct HashMapEntry {
    char* key;                      // String key
    void* value;                    // Associated value
    struct HashMapEntry* next;      // Next entry in bucket (for collision resolution)
    void (*destroy_value)(void*);   // Value destructor function pointer
} HashMapEntry;
```

**Fields**:
- `key`: Null-terminated string used as lookup key
- `value`: Pointer to stored data
- `next`: Pointer to next entry in collision chain
- `destroy_value`: Optional destructor function for custom cleanup

#### HashMap
```c
typedef struct {
    HashMapEntry** buckets;     // Array of bucket pointers
    size_t size;               // Number of entries in map
    size_t capacity;           // Total number of buckets
} HashMap;
```

**Fields**:
- `buckets`: Array of linked list heads for hash buckets
- `size`: Current number of key-value pairs
- `capacity`: Total capacity of the hash table

#### JsonValue
```c
typedef enum {
    JSON_NULL,
    JSON_BOOL,
    JSON_NUMBER,
    JSON_STRING,
    JSON_ARRAY,
    JSON_OBJECT
} JsonType;

typedef struct JsonValue {
    JsonType type;              // Type of JSON value
    int refcount;              // Reference counting for memory management
    union {
        bool bool_value;       // Boolean value
        double number_value;   // Numeric value
        char* string_value;    // String value
        struct {
            struct JsonValue** values;  // Array elements
            size_t count;               // Array length
        } array_value;
        HashMap* object_value;  // Object properties
    };
} JsonValue;
```

**Purpose**: Represents JSON data types with reference counting for memory management.

#### ToolParameter
```c
struct ToolParameter {
    char* name;            // Parameter name
    char* type_;           // Expected data type
    char* description;     // Human-readable description
    bool required;         // Whether parameter is required
    JsonValue* default_value; // Default value if not provided
    regex_t* pattern;      // Compiled regex pattern for validation
};
```

**Purpose**: Defines a parameter that a tool accepts.

#### ToolDefinition
```c
struct ToolDefinition {
    char* name;                    // Tool name
    char* description;             // Tool description
    ToolParameter* parameters;     // Array of parameters
    size_t parameters_count;       // Number of parameters
    char* category;                // Tool category
    ResourceLock* resource_locks;  // Required resource locks
    size_t resource_locks_count;   // Number of resource locks
};
```

**Purpose**: Complete definition of a tool's interface and requirements.

#### ToolResult
```c
struct ToolResult {
    bool success;               // Execution success status
    char* output;              // Output data (if successful)
    char* error;               // Error message (if failed)
    uint64_t execution_time_ms; // Execution duration
    HashMap* metadata;         // Additional metadata
    HashMap* recovery_context; // Context for error recovery
};
```

**Purpose**: Contains the result of tool execution.

#### ToolError
```c
struct ToolError {
    char* message;      // Error message
    ErrorType error_type; // Error category
    HashMap* details;   // Additional error details
};
```

**Purpose**: Detailed error information.

#### Tool (Interface)
```c
struct Tool {
    ToolDefinitionFn definition_fn;           // Returns tool definition
    ToolExecuteFn execute_fn;                 // Executes the tool
    ToolNameFn name_fn;                       // Returns tool name
    ToolValidateParametersFn validate_parameters_fn; // Validates parameters
    void* data;                               // Implementation-specific data
};
```

**Purpose**: Interface that all tools must implement.

#### ToolRegistry
```c
struct ToolRegistry {
    HashMap* tools;  // Map of tool names to Tool instances
};
```

**Purpose**: Central registry for managing all available tools.

## Core Components

### HashMap Implementation

#### hashmap_create()
```c
HashMap* hashmap_create(size_t capacity);
```
**Purpose**: Creates a new hash map with specified capacity.

**Parameters**:
- `capacity`: Initial number of buckets (should be prime for better distribution)

**Returns**: Pointer to newly allocated HashMap, or NULL on failure.

**Time Complexity**: O(1)

**Memory Usage**: O(capacity)

#### hashmap_destroy()
```c
void hashmap_destroy(HashMap* map);
```
**Purpose**: Completely destroys a hash map and all its entries.

**Parameters**:
- `map`: HashMap to destroy

**Time Complexity**: O(n) where n is number of entries

**Memory Cleanup**: Frees all keys, values (using destructor if provided), and internal structures

#### hashmap_put()
```c
void hashmap_put(HashMap* map, const char* key, void* value);
void hashmap_put_with_destructor(HashMap* map, const char* key, void* value, 
                                void (*destroy_value)(void*));
```

**Purpose**: Inserts or updates a key-value pair in the hash map.

**Parameters**:
- `map`: Target hash map
- `key`: String key (duplicated internally)
- `value`: Pointer to value
- `destroy_value`: Optional destructor function for value cleanup

**Time Complexity**: 
- Average: O(1)
- Worst-case: O(n) with many collisions

#### hashmap_get()
```c
void* hashmap_get(HashMap* map, const char* key);
```
**Purpose**: Retrieves a value by key.

**Parameters**:
- `map`: Source hash map
- `key`: Lookup key

**Returns**: Value pointer or NULL if not found

**Time Complexity**: 
- Average: O(1)
- Worst-case: O(n) with many collisions

### JSON Value Implementation

#### json_value_create_*()
```c
JsonValue* json_value_create_null();
JsonValue* json_value_create_bool(bool value);
JsonValue* json_value_create_number(double value);
JsonValue* json_value_create_string(const char* value);
JsonValue* json_value_create_array(JsonValue** values, size_t count);
JsonValue* json_value_create_object(HashMap* object);
```

**Purpose**: Factory functions for creating different JSON value types.

**Memory Management**: Uses reference counting for automatic cleanup.

#### json_value_destroy()
```c
void json_value_destroy(JsonValue* value);
```
**Purpose**: Decrements reference count and frees memory when count reaches zero.

**Reference Counting**: Prevents double-free and enables sharing.

#### json_value_to_string()
```c
char* json_value_to_string(JsonValue* value);
```
**Purpose**: Serializes JSON value to string representation.

**Returns**: Newly allocated string that must be freed by caller.

### Tool Management System

#### ToolRegistry Functions

##### tool_registry_new()
```c
ToolRegistry* tool_registry_new();
```
**Purpose**: Creates a new tool registry instance.

**Returns**: New ToolRegistry instance or NULL on failure.

##### tool_registry_register()
```c
void tool_registry_register(ToolRegistry* registry, Tool* tool);
```
**Purpose**: Registers a tool in the registry.

**Parameters**:
- `registry`: Target registry
- `tool`: Tool to register

**Side Effects**: Tool name must be unique; existing tool with same name will be replaced.

##### tool_registry_execute_tool()
```c
ToolResult* tool_registry_execute_tool(ToolRegistry* registry, const char* tool_name, HashMap* kwargs);
```
**Purpose**: Executes a registered tool with provided parameters.

**Parameters**:
- `registry`: Source registry
- `tool_name`: Name of tool to execute
- `kwargs`: Key-value pairs of parameters

**Returns**: ToolResult containing execution outcome

**Error Handling**:
- Returns error result if tool not found
- Validates parameters before execution

## API Reference

### Complete Function List

#### HashMap API
```c
HashMap* hashmap_create(size_t capacity);
void hashmap_destroy(HashMap* map);
void hashmap_put(HashMap* map, const char* key, void* value);
void hashmap_put_with_destructor(HashMap* map, const char* key, void* value, 
                                void (*destroy_value)(void*));
void* hashmap_get(HashMap* map, const char* key);
bool hashmap_contains(HashMap* map, const char* key);
void hashmap_remove(HashMap* map, const char* key);
size_t hashmap_size(HashMap* map);
```

#### JSON API
```c
JsonValue* json_value_create_null();
JsonValue* json_value_create_bool(bool value);
JsonValue* json_value_create_number(double value);
JsonValue* json_value_create_string(const char* value);
JsonValue* json_value_create_array(JsonValue** values, size_t count);
JsonValue* json_value_create_object(HashMap* object);
JsonValue* json_value_clone(JsonValue* value);
void json_value_destroy(JsonValue* value);
char* json_value_to_string(JsonValue* value);
```

#### Tool Parameter API
```c
ToolParameter* tool_parameter_new(const char* name, const char* type_, 
                                 const char* description, bool required, 
                                 JsonValue* default_value);
void tool_parameter_destroy(ToolParameter* param);
```

#### Tool Definition API
```c
ToolDefinition* tool_definition_new(const char* name, const char* description,
                                   ToolParameter* parameters, size_t parameters_count,
                                   const char* category, ResourceLock* resource_locks,
                                   size_t resource_locks_count);
void tool_definition_destroy(ToolDefinition* def);
JsonValue* tool_definition_to_ollama_format(ToolDefinition* def);
```

#### Tool Result API
```c
ToolResult* tool_result_new(bool success, const char* output, const char* error,
                           uint64_t execution_time_ms, HashMap* metadata,
                           HashMap* recovery_context);
ToolResult* tool_result_success(const char* output, uint64_t execution_time_ms);
ToolResult* tool_result_error(const char* error, uint64_t execution_time_ms);
void tool_result_destroy(ToolResult* result);
char* tool_result_to_string(ToolResult* result);
```

#### Error Handling API
```c
ToolError* tool_error_new(const char* message, ErrorType error_type, HashMap* details);
void tool_error_destroy(ToolError* error);
char* tool_error_to_string(ToolError* error);
ToolResult* handle_error(const char* context, int err_code, const char* err_msg);
ToolResult* validate_required_params(HashMap* kwargs, char** required_params, 
                                   size_t required_count);
ToolResult* validate_string_param(JsonValue* value, const char* param_name, 
                                 size_t min_length, size_t max_length, const char* pattern);
```

#### Tool Registry API
```c
ToolRegistry* tool_registry_new();
void tool_registry_destroy(ToolRegistry* registry);
void tool_registry_register(ToolRegistry* registry, Tool* tool);
void tool_registry_register_core_tools(ToolRegistry* registry);
Tool* tool_registry_get_tool(ToolRegistry* registry, const char* name);
Tool** tool_registry_get_all_tools(ToolRegistry* registry, size_t* count);
JsonValue** tool_registry_get_tool_definitions(ToolRegistry* registry, size_t* count);
ToolResult* tool_registry_execute_tool(ToolRegistry* registry, const char* tool_name, 
                                      HashMap* kwargs);
size_t tool_registry_tool_count(ToolRegistry* registry);
bool tool_registry_has_tool(ToolRegistry* registry, const char* name);
bool tool_registry_unregister(ToolRegistry* registry, const char* name);
void tool_registry_clear(ToolRegistry* registry);
```

## Design Patterns

### 1. Strategy Pattern
**Implementation**: Tool interface with function pointers
```c
struct Tool {
    ToolDefinitionFn definition_fn;
    ToolExecuteFn execute_fn;
    // ... other function pointers
};
```
**Purpose**: Allows different tool implementations to be interchangeable.

### 2. Factory Pattern
**Implementation**: Various `create_*` functions (`tool_parameter_new`, `json_value_create_*`, etc.)
**Purpose**: Centralized object creation with proper initialization.

### 3. Composite Pattern
**Implementation**: JSON value system with nested arrays and objects
**Purpose**: Represents complex hierarchical data structures.

### 4. Flyweight Pattern
**Implementation**: Reference counting in JsonValue
**Purpose**: Reduces memory usage by sharing common values.

### 5. Registry Pattern
**Implementation**: ToolRegistry managing tool instances
**Purpose**: Centralized management and lookup of tools.

## Usage Examples

### Basic Example: Creating and Using a Simple Tool

```c
#include "tool_framework.h"
#include <time.h>

// Simple tool implementation
static ToolDefinition* file_read_definition(Tool* self) {
    ToolParameter params[] = {
        *create_tool_parameter("filename", "string", "File to read", true),
        *create_tool_parameter("encoding", "string", "File encoding", false)
    };
    
    ResourceLock locks[] = {RESOURCE_LOCK_FILESYSTEM_READ};
    
    return create_tool_definition_with_locks(
        "file_read", 
        "Reads contents of a file",
        "file_operations",
        params, 2,
        locks, 1
    );
}

static char* file_read_name(Tool* self) {
    return string_duplicate("file_read");
}

static bool file_read_validate(Tool* self, HashMap* kwargs) {
    return tool_validate_parameters(self, kwargs);
}

static ToolResult* file_read_execute(Tool* self, HashMap* kwargs) {
    uint64_t start_time = get_current_time_ms();
    
    // Parameter extraction
    JsonValue* filename_val = hashmap_get(kwargs, "filename");
    if (!filename_val || filename_val->type != JSON_STRING) {
        return create_error_result("Filename parameter required and must be string", 
                                 0, ERROR_TYPE_VALIDATION_ERROR, NULL);
    }
    
    // Simulate file reading
    char* content = simulate_file_read(filename_val->string_value);
    uint64_t end_time = get_current_time_ms();
    
    if (content) {
        ToolResult* result = tool_result_success(content, end_time - start_time);
        free(content);
        return result;
    } else {
        return create_error_result("File not found or cannot be read", 
                                 end_time - start_time, 
                                 ERROR_TYPE_FILE_NOT_FOUND, NULL);
    }
}

// Tool instance creation
Tool* create_file_read_tool() {
    Tool* tool = malloc(sizeof(Tool));
    tool->definition_fn = file_read_definition;
    tool->execute_fn = file_read_execute;
    tool->name_fn = file_read_name;
    tool->validate_parameters_fn = file_read_validate;
    tool->data = NULL; // No extra data needed
    
    return tool;
}

// Usage example
int main() {
    // Create registry
    ToolRegistry* registry = tool_registry_new();
    
    // Register tool
    Tool* file_tool = create_file_read_tool();
    tool_registry_register(registry, file_tool);
    
    // Prepare parameters
    HashMap* params = hashmap_create(8);
    JsonValue* filename = json_value_create_string("example.txt");
    hashmap_put(params, "filename", filename);
    
    // Execute tool
    ToolResult* result = tool_registry_execute_tool(registry, "file_read", params);
    
    // Process result
    if (result->success) {
        printf("File content: %s\n", result->output);
    } else {
        printf("Error: %s\n", result->error);
    }
    
    // Cleanup
    tool_result_destroy(result);
    hashmap_destroy(params);
    tool_registry_destroy(registry);
    
    return 0;
}
```

### Advanced Example: Tool with Complex Validation

```c
// Tool with regex validation and default values
ToolParameter* create_email_parameter() {
    JsonValue* default_email = json_value_create_string("user@example.com");
    ToolParameter* param = create_tool_parameter_with_default(
        "email", "string", "Email address", false, default_email
    );
    
    // Compile email regex pattern
    param->pattern = malloc(sizeof(regex_t));
    regcomp(param->pattern, "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", 
           REG_EXTENDED);
    
    return param;
}

// Validation function with custom rules
ToolResult* validate_user_input(Tool* tool, HashMap* kwargs) {
    ToolDefinition* def = tool->definition_fn(tool);
    
    // Check required parameters
    char* required[] = {"username", "email"};
    ToolResult* validation_result = validate_required_params(kwargs, required, 2);
    if (validation_result) return validation_result;
    
    // Validate email format
    JsonValue* email_val = hashmap_get(kwargs, "email");
    validation_result = validate_string_param(email_val, "email", 5, 254, 
                                            "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$");
    if (validation_result) return validation_result;
    
    // Validate username length
    JsonValue* username_val = hashmap_get(kwargs, "username");
    validation_result = validate_string_param(username_val, "username", 3, 50, NULL);
    
    return validation_result; // NULL if validation passed
}
```

### JSON Serialization Example

```c
// Creating complex JSON structures
void create_nested_json_example() {
    // Create inner object
    HashMap* address = hashmap_create(4);
    JsonValue* street = json_value_create_string("123 Main St");
    JsonValue* city = json_value_create_string("Springfield");
    
    hashmap_put(address, "street", street);
    hashmap_put(address, "city", city);
    
    JsonValue* address_obj = json_value_create_object(address);
    
    // Create array
    JsonValue* hobbies[] = {
        json_value_create_string("reading"),
        json_value_create_string("coding"),
        json_value_create_string("gaming")
    };
    JsonValue* hobbies_array = json_value_create_array(hobbies, 3);
    
    // Create main object
    HashMap* person = hashmap_create(8);
    JsonValue* name = json_value_create_string("John Doe");
    JsonValue* age = json_value_create_number(30);
    
    hashmap_put(person, "name", name);
    hashmap_put(person, "age", age);
    hashmap_put(person, "address", address_obj);
    hashmap_put(person, "hobbies", hobbies_array);
    
    JsonValue* person_obj = json_value_create_object(person);
    
    // Serialize to string
    char* json_str = json_value_to_string(person_obj);
    printf("JSON: %s\n", json_str);
    
    // Cleanup
    free(json_str);
    json_value_destroy(person_obj);
}
```

## Performance Analysis

### Time Complexity Analysis

| Operation | Average Case | Worst Case | Notes |
|-----------|--------------|------------|-------|
| HashMap Put/Get | O(1) | O(n) | Depends on hash distribution |
| Tool Registration | O(1) | O(1) | Constant time |
| Tool Execution | O(p) | O(p) | p = number of parameters |
| JSON Serialization | O(n) | O(n) | n = size of JSON tree |
| Parameter Validation | O(p) | O(p) | p = number of parameters |

### Memory Usage Analysis

| Component | Memory Footprint | Notes |
|-----------|------------------|-------|
| HashMap Entry | ~32-64 bytes | Depends on key length |
| JsonValue | 16-48 bytes | Varies by type |
| ToolDefinition | ~100-500 bytes | Depends on parameter count |
| ToolRegistry | ~1-10 KB | Scales with tool count |

### Optimization Recommendations

1. **HashMap Sizing**: Use prime numbers for capacity to reduce collisions
2. **JSON Sharing**: Use `json_value_clone()` for shared values
3. **String Management**: Pool common strings to reduce duplication
4. **Tool Caching**: Cache tool definitions for frequently used tools

## Security Considerations

### Input Validation
- **Parameter Type Checking**: All parameters are validated against expected types
- **String Length Limits**: Prevents buffer overflow attacks
- **Regex Validation**: Validates input patterns where applicable
- **Resource Locking**: Prevents concurrent access to sensitive resources

### Memory Safety
- **Bounds Checking**: Array operations include bounds verification
- **Null Pointer Checks**: All functions handle NULL inputs gracefully
- **Memory Leak Prevention**: Comprehensive cleanup functions provided

### Access Control
- **Resource Lock Enum**: Defines access levels for different operations
- **Permission Error Handling**: Specific error type for permission issues
- **Input Sanitization**: Parameters are validated before processing

### Security Best Practices

1. **Always validate tool parameters** before execution
2. **Use resource locks** for operations involving external resources
3. **Implement proper error handling** to avoid information leakage
4. **Regularly audit tool definitions** for security implications

## Testing Strategies

### Unit Testing Framework

```c
// Example test structure
typedef struct TestCase {
    const char* name;
    bool (*test_function)();
    const char* description;
} TestCase;

bool test_hashmap_operations() {
    HashMap* map = hashmap_create(16);
    
    // Test basic operations
    hashmap_put(map, "key1", "value1");
    void* value = hashmap_get(map, "key1");
    bool success = (value != NULL && strcmp(value, "value1") == 0);
    
    hashmap_destroy(map);
    return success;
}

bool test_tool_validation() {
    Tool* tool = create_test_tool();
    HashMap* params = hashmap_create(8);
    
    // Test missing required parameter
    ToolResult* result = tool->validate_parameters_fn(tool, params);
    bool success = (result != NULL && !result->success);
    
    if (result) tool_result_destroy(result);
    hashmap_destroy(params);
    // Cleanup tool...
    
    return success;
}
```

### Integration Testing

```c
// End-to-end tool execution test
bool test_tool_execution_flow() {
    ToolRegistry* registry = tool_registry_new();
    tool_registry_register_core_tools(registry);
    
    HashMap* params = hashmap_create(8);
    // Setup parameters...
    
    ToolResult* result = tool_registry_execute_tool(registry, "file_operation", params);
    
    bool success = (result != NULL && 
                   ((result->success && result->output != NULL) ||
                    (!result->success && result->error != NULL)));
    
    // Cleanup...
    return success;
}
```

### Performance Testing

```c
// Benchmark tool execution
void benchmark_tool_execution() {
    const int iterations = 1000;
    uint64_t total_time = 0;
    
    for (int i = 0; i < iterations; i++) {
        uint64_t start = get_current_time_ms();
        
        // Execute tool...
        
        uint64_t end = get_current_time_ms();
        total_time += (end - start);
    }
    
    printf("Average execution time: %lu ms\n", total_time / iterations);
}
```

## Deployment Instructions

### Build Configuration

```makefile
# Makefile example
CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -D_POSIX_C_SOURCE=200809L
LDFLAGS = -lregex

SOURCES = tool_framework.c
HEADERS = tool_framework.h
OBJECTS = $(SOURCES:.c=.o)

TARGET = libtoolframework.a

$(TARGET): $(OBJECTS)
	$(AR) rcs $@ $(OBJECTS)

%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(TARGET)

.PHONY: clean
```

### Platform Requirements

- **Compiler**: C99 compliant (GCC, Clang, MSVC)
- **Libraries**: POSIX regex support
- **Platform**: Linux, macOS, Windows (with POSIX compatibility)
- **Memory**: Minimum 1MB RAM, recommended 4MB+

### Integration Steps

1. **Include the header file** in your project
2. **Link against the library** during compilation
3. **Initialize tool registry** at application startup
4. **Register tools** according to your requirements
5. **Execute tools** through the registry interface

### Configuration Options

```c
// Compile-time configuration
#define TOOL_FRAMEWORK_MAX_PARAMETERS 64
#define TOOL_FRAMEWORK_MAX_TOOLS 256
#define TOOL_FRAMEWORK_HASHMAP_PRIME_SIZE 101

// Runtime configuration
typedef struct FrameworkConfig {
    size_t max_concurrent_tools;
    bool enable_logging;
    const char* log_file_path;
} FrameworkConfig;
```

## Troubleshooting Guide

### Common Issues and Solutions

#### Issue 1: Memory Leaks
**Symptoms**: Increasing memory usage over time
**Solution**: 
```c
// Enable memory debugging
#define DEBUG_MEMORY 1

#ifdef DEBUG_MEMORY
void* debug_malloc(size_t size, const char* file, int line) {
    void* ptr = malloc(size);
    printf("Allocated %zu bytes at %s:%d\n", size, file, line);
    return ptr;
}
#define malloc(size) debug_malloc(size, __FILE__, __LINE__)
#endif
```

#### Issue 2: Tool Registration Failures
**Symptoms**: Tools not found during execution
**Solution**: Check tool name uniqueness and registration order

#### Issue 3: Parameter Validation Failures
**Symptoms**: Validation errors even with correct parameters
**Solution**: Verify parameter types and required flags in tool definition

### Debugging Techniques

#### Logging Framework Integration
```c
typedef enum {
    LOG_LEVEL_ERROR,
    LOG_LEVEL_WARNING,
    LOG_LEVEL_INFO,
    LOG_LEVEL_DEBUG
} LogLevel;

void tool_framework_log(LogLevel level, const char* format, ...) {
    va_list args;
    va_start(args, format);
    
    const char* level_str[] = {"ERROR", "WARNING", "INFO", "DEBUG"};
    printf("[%s] ", level_str[level]);
    vprintf(format, args);
    printf("\n");
    
    va_end(args);
}
```

#### Error Recovery Strategies
```c
ToolResult* execute_tool_with_recovery(ToolRegistry* registry, 
                                      const char* tool_name, 
                                      HashMap* kwargs,
                                      int max_retries) {
    for (int attempt = 0; attempt < max_retries; attempt++) {
        ToolResult* result = tool_registry_execute_tool(registry, tool_name, kwargs);
        
        if (result->success) {
            return result;
        }
        
        // Analyze error and decide if retry is possible
        if (can_retry_based_on_error(result)) {
            tool_framework_log(LOG_LEVEL_WARNING, 
                             "Tool execution failed, retrying... (attempt %d/%d)",
                             attempt + 1, max_retries);
            tool_result_destroy(result);
            sleep(1 << attempt); // Exponential backoff
            continue;
        }
        
        return result; // Unrecoverable error
    }
    
    return create_error_result("Max retries exceeded", 0, 
                             ERROR_TYPE_RESOURCE_ERROR, NULL);
}
```

### Performance Tuning

#### Memory Pool Optimization
```c
typedef struct MemoryPool {
    void** blocks;
    size_t block_size;
    size_t capacity;
    size_t used;
} MemoryPool;

MemoryPool* create_memory_pool(size_t block_size, size_t capacity) {
    MemoryPool* pool = malloc(sizeof(MemoryPool));
    pool->blocks = malloc(capacity * sizeof(void*));
    pool->block_size = block_size;
    pool->capacity = capacity;
    pool->used = 0;
    
    for (size_t i = 0; i < capacity; i++) {
        pool->blocks[i] = malloc(block_size);
    }
    
    return pool;
}

void* pool_allocate(MemoryPool* pool) {
    if (pool->used >= pool->capacity) {
        return NULL; // Pool exhausted
    }
    
    return pool->blocks[pool->used++];
}
```

This comprehensive documentation covers all aspects of the tool framework system, providing developers with complete information for understanding, using, and extending the framework. The documentation follows best practices for clarity, completeness, and practical utility.