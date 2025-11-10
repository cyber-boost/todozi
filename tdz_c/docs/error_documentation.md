# Todozi Error Management System - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [API Reference](#api-reference)
5. [Design Patterns](#design-patterns)
6. [Implementation Details](#implementation-details)
7. [Usage Examples](#usage-examples)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)
13. [Future Enhancements](#future-enhancements)

## Overview

Todozi is a comprehensive error management system designed for C applications. It provides a structured approach to error tracking, resolution, and analysis with a focus on extensibility and reliability.

### Key Features
- Structured error representation with severity and categorization
- Error lifecycle management (creation, resolution, tracking)
- Hash-based storage for efficient error lookup
- Type-safe result types using tagged unions
- Text-based error parsing for flexible error input
- Comprehensive error metadata including timestamps and tags

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Application   │───▶│ ErrorManager    │───▶│    HashMap      │
│     Layer       │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       ▼
         │                       │             ┌─────────────────┐
         │                       └─────────────│      Error      │
         │                                     │   Structures    │
         │                                     └─────────────────┘
         │                                             │
         └─────────────────────────────────────────────┘
                         Error Reporting
```

### Component Relationships
```text
Application → ErrorManager → HashMap → Error
     ↓              ↓            ↓        ↓
  Creates      Manages       Stores    Contains
  Errors      Lifecycle     Errors     Metadata
```

## Data Structures

### ErrorSeverity Enum
```c
typedef enum {
    ERROR_SEVERITY_LOW = 0,
    ERROR_SEVERITY_MEDIUM = 1,
    ERROR_SEVERITY_HIGH = 2,
    ERROR_SEVERITY_CRITICAL = 3
} ErrorSeverity;
```
**Purpose**: Defines the impact level of errors
- **LOW**: Minor issues, non-breaking
- **MEDIUM**: Moderate impact, may affect functionality
- **HIGH**: Serious issues affecting core functionality
- **CRITICAL**: System-breaking errors requiring immediate attention

### ErrorCategory Enum
```c
typedef enum {
    ERROR_CATEGORY_NETWORK = 0,
    ERROR_CATEGORY_DATABASE = 1,
    ERROR_CATEGORY_APPLICATION = 2,
    ERROR_CATEGORY_SYSTEM = 3
} ErrorCategory;
```
**Purpose**: Categorizes errors by system component
- **NETWORK**: Communication and connectivity issues
- **DATABASE**: Data storage and retrieval problems
- **APPLICATION**: Business logic and processing errors
- **SYSTEM**: Infrastructure and platform issues

### Error Structure
```c
struct Error {
    char* id;                // Unique identifier (UUID)
    char* title;             // Brief error description
    char* description;       // Detailed error explanation
    ErrorSeverity severity;  // Error impact level
    ErrorCategory category;  // Error classification
    char* source;            // Originating component
    char* context;           // Additional context information
    char** tags;             // Categorization tags array
    int tags_count;          // Number of tags
    bool resolved;           // Resolution status
    char* resolution;        // Resolution description
    time_t created_at;       // Creation timestamp
    time_t updated_at;       // Last update timestamp
    time_t resolved_at;      // Resolution timestamp
};
```
**Memory Management**: All string fields are dynamically allocated and must be freed

### HashMap Structure
```c
struct HashMap {
    char** keys;      // Array of string keys
    Error** values;   // Array of Error pointers
    int size;         // Current number of entries
    int capacity;     // Maximum capacity before resize
};
```
**Design**: Simple linear-probing hash map with dynamic resizing
- Initial capacity: 16 entries
- Growth factor: 2x when full
- Linear search for key lookup

### Result Types (Tagged Unions)
```c
// String result type
typedef struct {
    bool is_ok;
    union {
        char* ok_value;  // Success case: string result
        struct {
            TodoziErrorType error_type;
            char* message;
        } err_value;     // Error case: error information
    } data;
} TodoziResultString;

// Error pointer result type
typedef struct {
    bool is_ok;
    union {
        Error* ok_value;  // Success case: Error pointer
        struct {
            TodoziErrorType error_type;
            char* message;
        } err_value;      // Error case: error information
    } data;
} TodoziResultErrorPtr;

// Void result type
typedef struct {
    bool is_ok;
    union {
        void* ok_value;   // Success case: NULL (void)
        struct {
            TodoziErrorType error_type;
            char* message;
        } err_value;      // Error case: error information
    } data;
} TodoziResultVoid;
```

## API Reference

### Core Management Functions

#### error_manager_new
```c
ErrorManager* error_manager_new(void);
```
**Purpose**: Creates a new ErrorManager instance
- **Returns**: Pointer to initialized ErrorManager, NULL on failure
- **Memory**: Allocates memory for manager and internal hashmap
- **Error Handling**: Returns NULL if allocation fails

#### error_manager_free
```c
void error_manager_free(ErrorManager* manager);
```
**Purpose**: Releases all resources associated with ErrorManager
- **Parameters**: 
  - `manager`: ErrorManager instance to free
- **Memory**: Frees manager, hashmap, and all contained errors
- **Safety**: Handles NULL input gracefully

#### error_manager_create_error
```c
TodoziResultString error_manager_create_error(ErrorManager* manager, Error* error);
```
**Purpose**: Adds a new error to the management system
- **Parameters**:
  - `manager`: ErrorManager instance
  - `error`: Pre-configured Error structure (ID will be generated)
- **Returns**: TodoziResultString containing error ID or error information
- **Side Effects**: Generates UUID, sets timestamps, inserts into hashmap
- **Error Conditions**: Invalid parameters, UUID generation failure

#### error_manager_resolve_error
```c
TodoziResultVoid error_manager_resolve_error(ErrorManager* manager, const char* error_id, const char* resolution);
```
**Purpose**: Marks an error as resolved with explanation
- **Parameters**:
  - `manager`: ErrorManager instance
  - `error_id`: UUID of error to resolve
  - `resolution`: Description of resolution (optional)
- **Returns**: TodoziResultVoid indicating success or failure
- **Side Effects**: Updates error status, sets resolution text and timestamps
- **Error Conditions**: Invalid parameters, error not found

### Error Parsing Functions

#### parse_error_format
```c
TodoziResultErrorPtr parse_error_format(const char* error_text);
```
**Purpose**: Parses error information from structured text format
- **Parameters**: `error_text`: Formatted error string
- **Format**: `<error>title;description;severity;category;source;context;tags</error>`
- **Returns**: TodoziResultErrorPtr containing parsed Error or error information
- **Validation**: Checks format integrity and field validity

### Utility Functions

#### error_new / error_free
```c
Error* error_new(void);
void error_free(Error* error);
```
**Purpose**: Creates and destroys Error structures
- **Memory**: Allocates/initializes or frees all nested resources

#### Hash Map Operations
```c
HashMap* hashmap_new(void);
void hashmap_free(HashMap* map);
void hashmap_insert(HashMap* map, const char* key, Error* value);
Error* hashmap_get(HashMap* map, const char* key);
```

### Helper Macros
```c
// Success macros
#define TODOZI_OK_STRING(res, value)
#define TODOZI_OK_ERROR_PTR(res, value)  
#define TODOZI_OK_VOID(res)

// Error macros  
#define TODOZI_ERR_STRING(res, kind, msg)
#define TODOZI_ERR_ERROR_PTR(res, kind, msg)
#define TODOZI_ERR_VOID(res, kind, msg)
```
**Purpose**: Standardized result initialization
- **Usage**: Ensures consistent result structure population

## Design Patterns

### 1. Result Type Pattern
**Implementation**: Tagged unions with success/error discrimination
**Benefits**: Type-safe error handling without exceptions
**Usage**: All API functions return result types

### 2. Resource Acquisition Is Initialization (RAII)
**Implementation**: _new/_free function pairs
**Benefits**: Clear ownership and memory management
**Usage**: ErrorManager, Error, HashMap structures

### 3. Factory Pattern
**Implementation**: `parse_error_format` function
**Benefits**: Flexible object creation from different sources
**Usage**: Text-to-Error conversion

### 4. Manager Pattern
**Implementation**: ErrorManager centralizes error operations
**Benefits**: Single responsibility, simplified interface
**Usage**: Core error lifecycle management

## Implementation Details

### Memory Management Strategy
```c
// Ownership rules:
// - ErrorManager owns HashMap and contained Errors
// - HashMap owns key strings but shares Error ownership with manager
// - Result types own their string/error data until freed
// - Caller must free result data using todozi_result_*_free functions
```

### UUID Generation
```c
char* generate_uuid(void) {
    // Note: This is a simplified implementation
    // Production code should use proper UUID library
    char* uuid = malloc(37);
    snprintf(uuid, 37, "%08x-%04x-%04x-%04x-%012llx",
            rand(), rand() & 0xFFFF, rand() & 0xFFFF,
            rand() & 0xFFFF, ((long long)rand() << 32) | rand());
    return uuid;
}
```
**Limitation**: Not cryptographically secure
**Improvement**: Use system UUID library in production

### Error Text Parsing Logic
```text
Parsing Flow:
1. Extract content between <error> tags
2. Split by ';' delimiter into parts
3. Validate minimum required parts (5)
4. Parse severity and category from strings
5. Handle optional context and tags
6. Create Error structure with parsed data
```

## Usage Examples

### Basic Error Creation and Management
```c
#include "todozi.h"

int main() {
    // Initialize error manager
    ErrorManager* manager = error_manager_new();
    if (!manager) {
        fprintf(stderr, "Failed to create error manager\n");
        return 1;
    }

    // Create a new error
    Error* error = error_new();
    error->title = strdup("Database Connection Failed");
    error->description = strdup("Unable to connect to PostgreSQL");
    error->severity = ERROR_SEVERITY_HIGH;
    error->category = ERROR_CATEGORY_DATABASE;
    error->source = strdup("database-service");

    // Add error to manager
    TodoziResultString result = error_manager_create_error(manager, error);
    if (!result.is_ok) {
        fprintf(stderr, "Error: %s\n", result.data.err_value.message);
        todozi_result_string_free(&result);
        error_free(error);
        error_manager_free(manager);
        return 1;
    }

    printf("Created error with ID: %s\n", result.data.ok_value);
    
    // Resolve the error
    TodoziResultVoid resolve_result = error_manager_resolve_error(
        manager, result.data.ok_value, "Restarted database service");
    
    if (!resolve_result.is_ok) {
        fprintf(stderr, "Failed to resolve error: %s\n", 
                resolve_result.data.err_value.message);
    } else {
        printf("Error resolved successfully\n");
    }

    // Cleanup
    todozi_result_string_free(&result);
    todozi_result_void_free(&resolve_result);
    error_manager_free(manager);
    return 0;
}
```

### Error Parsing from Text
```c
void parse_error_example() {
    const char* error_text = 
        "<error>File Not Found;Unable to open configuration file;high;application;config-loader;Path: /etc/app/config.json;file,config,io</error>";
    
    TodoziResultErrorPtr result = parse_error_format(error_text);
    
    if (result.is_ok) {
        Error* error = result.data.ok_value;
        printf("Parsed Error:\n");
        printf("  Title: %s\n", error->title);
        printf("  Severity: %s\n", error_severity_to_str(error->severity));
        printf("  Tags: ");
        for (int i = 0; i < error->tags_count; i++) {
            printf("%s ", error->tags[i]);
        }
        printf("\n");
        
        todozi_result_error_ptr_free(&result);
    } else {
        fprintf(stderr, "Parse failed: %s\n", result.data.err_value.message);
        todozi_result_error_ptr_free(&result);
    }
}
```

### Batch Error Processing
```c
void process_unresolved_errors(ErrorManager* manager) {
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    
    if (unresolved) {
        printf("Found %d unresolved errors:\n", count);
        
        for (int i = 0; i < count; i++) {
            Error* error = unresolved[i];
            printf("  %s: %s (Severity: %s)\n", 
                   error->id, error->title, 
                   error_severity_to_str(error->severity));
        }
        
        error_manager_free_unresolved_errors(unresolved);
    }
}
```

## Performance Analysis

### Time Complexity
| Operation | Best Case | Average Case | Worst Case |
|-----------|-----------|--------------|------------|
| error_manager_create_error | O(1) | O(n) | O(n) |
| error_manager_resolve_error | O(1) | O(n) | O(n) |
| hashmap_get | O(1) | O(n) | O(n) |
| hashmap_insert | O(1) | O(n) | O(n) |
| parse_error_format | O(n) | O(n) | O(n) |

**Note**: HashMap operations are O(n) due to linear probing implementation

### Space Complexity
- **Error Structure**: O(1) per error, but variable string storage
- **HashMap**: O(capacity) storage overhead
- **Overall**: Linear in number of errors managed

### Memory Usage Patterns
```text
Memory Allocation Breakdown:
- Error structure: ~200-500 bytes base
- String fields: Variable (typically 50-500 bytes each)
- HashMap overhead: 16 * (pointer size) per capacity unit
- Tag array: 4-8 bytes per tag plus string storage
```

### Optimization Opportunities
1. **HashMap Improvement**: Implement proper hashing for O(1) operations
2. **String Interning**: Reduce duplicate string storage
3. **Memory Pool**: Batch allocation for Error structures
4. **Lazy Resolution**: Deferred timestamp updates

## Security Considerations

### Input Validation
```c
// Current validation in parse_error_format:
- Checks for NULL input
- Validates tag structure presence
- Verifies minimum required fields
- Validates enum values from strings
```

### Memory Safety
- **Risk**: Manual memory management can lead to leaks/double-free
- **Mitigation**: Consistent _free function usage, NULL checks
- **Improvement**: Add memory tracking in debug builds

### UUID Security
- **Current**: Pseudo-random generation not cryptographically secure
- **Recommendation**: Use system UUID library (libuuid on Linux)

### Buffer Handling
- **Risk**: Fixed buffers in parsing logic
- **Mitigation**: Dynamic allocation with size checks
- **Example**: `content_len` calculation prevents overflow

### Recommended Security Enhancements
1. **Bounds Checking**: Add length validation for all string operations
2. **Input Sanitization**: Validate error text format more rigorously
3. **Memory Canaries**: Add guard bytes around allocations
4. **Audit Logging**: Track error management operations

## Testing Strategies

### Unit Testing Framework
```c
// Test macros for comprehensive testing
#define ASSERT(condition, message) \
    if (!(condition)) { \
        printf("TEST FAILED: %s\n", message); \
        return 1; \
    }

#define TEST_ERROR_PARSING  // Compile flag for test main
```

### Test Categories

#### 1. Functional Tests
```c
void test_error_creation() {
    ErrorManager* manager = error_manager_new();
    ASSERT(manager != NULL, "Manager creation");
    
    Error* error = error_new();
    error->title = strdup("Test Error");
    // ... configure error
    
    TodoziResultString result = error_manager_create_error(manager, error);
    ASSERT(result.is_ok, "Error creation");
    ASSERT(strlen(result.data.ok_value) > 0, "UUID generation");
    
    // Cleanup
    todozi_result_string_free(&result);
    error_manager_free(manager);
}
```

#### 2. Error Condition Tests
```c
void test_error_conditions() {
    // Test NULL parameters
    TodoziResultString result = error_manager_create_error(NULL, NULL);
    ASSERT(!result.is_ok, "NULL parameter handling");
    ASSERT(result.data.err_value.error_type == TODOZI_ERROR_VALIDATION_ERROR,
           "Correct error type");
    
    todozi_result_string_free(&result);
}
```

#### 3. Boundary Tests
```c
void test_boundary_conditions() {
    // Test hashmap resizing
    ErrorManager* manager = error_manager_new();
    
    for (int i = 0; i < 20; i++) {
        Error* error = error_new();
        // ... create unique error
        error_manager_create_error(manager, error);
    }
    
    // Verify no crashes during resize
    ASSERT(manager->errors->capacity >= 32, "HashMap resized correctly");
    
    error_manager_free(manager);
}
```

#### 4. Memory Leak Tests
```c
void test_memory_management() {
    // Use valgrind or similar tool to verify no leaks
    ErrorManager* manager = error_manager_new();
    
    // Perform various operations
    // ...
    
    error_manager_free(manager);
    // Should show no memory leaks
}
```

### Integration Testing
```c
// Test error lifecycle completely
void test_full_lifecycle() {
    ErrorManager* manager = error_manager_new();
    
    // Create error
    Error* error = create_test_error();
    TodoziResultString create_result = error_manager_create_error(manager, error);
    
    // Resolve error
    TodoziResultVoid resolve_result = error_manager_resolve_error(
        manager, create_result.data.ok_value, "Test resolution");
    
    // Verify resolution
    Error* resolved_error = hashmap_get(manager->errors, create_result.data.ok_value);
    ASSERT(resolved_error->resolved, "Error marked as resolved");
    ASSERT(strcmp(resolved_error->resolution, "Test resolution") == 0, 
           "Resolution text set");
    
    // Cleanup
    todozi_result_string_free(&create_result);
    todozi_result_void_free(&resolve_result);
    error_manager_free(manager);
}
```

## Deployment Instructions

### Compilation Requirements
```bash
# Required headers
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>

# Compilation command
gcc -std=c99 -Wall -Wextra -O2 -DTEST_ERROR_PARSING todozi.c -o todozi_test
```

### Platform-Specific Considerations

#### Linux
```bash
# Additional libraries for enhanced UUID generation
sudo apt-get install uuid-dev
gcc -luuid todozi.c -o todozi_app
```

#### Windows
```cmd
// Use Windows Crypto API for better UUID generation
// Link with advapi32.lib
cl todozi.c advapi32.lib
```

#### Embedded Systems
```makefile
# Minimal configuration for resource-constrained environments
CFLAGS += -DTODOZI_MINIMAL -DNO_ERROR_PARSING
```

### Integration Steps

1. **Header Inclusion**: Add `#include "todozi.h"` to your source files
2. **Initialization**: Create ErrorManager at application startup
3. **Error Reporting**: Use error creation functions throughout code
4. **Cleanup**: Ensure proper manager freeing at application exit
5. **Error Handling**: Check result types for all operations

### Build System Integration

#### CMake
```cmake
cmake_minimum_required(VERSION 3.10)
project(MyAppWithTodozi)

add_library(todozi STATIC todozi.c)
target_include_directories(todozi PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(myapp main.c)
target_link_libraries(myapp todozi)
```

#### Makefile
```makefile
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -O2
SRCS = todozi.c main.c
OBJS = $(SRCS:.c=.o)
TARGET = myapp

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(TARGET)
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. Memory Leaks
**Symptom**: Increasing memory usage over time
**Solution**: 
```c
// Ensure all allocated resources are freed
ErrorManager* manager = error_manager_new();
// ... use manager ...
error_manager_free(manager);  // Must be called

// For result types:
TodoziResultString result = some_function();
// ... use result ...
todozi_result_string_free(&result);  // Cleanup result data
```

#### 2. UUID Collisions
**Symptom**: Duplicate error IDs
**Solution**: Use proper UUID library instead of `rand()`
```c
// Linux: use libuuid
#include <uuid/uuid.h>
char* generate_uuid_secure(void) {
    uuid_t uuid;
    char* str = malloc(37);
    uuid_generate(uuid);
    uuid_unparse(uuid, str);
    return str;
}
```

#### 3. HashMap Performance Degradation
**Symptom**: Slower operations with many errors
**Solution**: Improve hash function and collision resolution
```c
// Better hash function example
unsigned int hash_function(const char* key) {
    unsigned int hash = 5381;
    int c;
    while ((c = *key++))
        hash = ((hash << 5) + hash) + c;
    return hash;
}
```

#### 4. Parsing Failures
**Symptom**: `parse_error_format` returns errors for valid input
**Solution**: Check input format strictly
```text
Required format: <error>title;desc;severity;category;source[;context][;tags]</error>
Example: <error>DB Error;Connection failed;high;database;db-service;Timeout;db,network</error>
```

### Debugging Techniques

#### 1. Memory Debugging
```c
// Add debug allocations
#ifdef DEBUG
#define malloc(size) debug_malloc(size, __FILE__, __LINE__)
#define free(ptr) debug_free(ptr, __FILE__, __LINE__)
#endif
```

#### 2. Error Tracking
```c
// Add error logging
void error_manager_create_error_with_log(ErrorManager* manager, Error* error, const char* context) {
    printf("Creating error from: %s\n", context);
    TodoziResultString result = error_manager_create_error(manager, error);
    // ... handle result
}
```

#### 3. Performance Profiling
```c
// Add timing for performance analysis
#include <sys/time.h>
long get_time_us() {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return tv.tv_sec * 1000000 + tv.tv_usec;
}

long start = get_time_us();
// ... operation ...
long end = get_time_us();
printf("Operation took: %ld microseconds\n", end - start);
```

### Error Code Reference

| Error Type | Description | Common Causes |
|------------|-------------|---------------|
| `TODOZI_ERROR_VALIDATION_ERROR` | Invalid input parameters | NULL pointers, malformed data |
| `TODOZI_ERROR_STORAGE_ERROR` | Memory allocation failure | System out of memory |
| `TODOZI_ERROR_UUID_ERROR` | UUID generation failure | Random number generator issue |
| `TODOZI_ERROR_JSON_ERROR` | JSON parsing failure | Malformed JSON input |

## Future Enhancements

### Planned Features

#### 1. Enhanced Storage Backends
```c
// Abstract storage interface
typedef struct {
    int (*save_error)(Error* error);
    Error* (*load_error)(const char* id);
    int (*delete_error)(const char* id);
} ErrorStorageBackend;

// Implementations for:
// - File-based storage
// - Database storage (SQLite, PostgreSQL)
// - Network storage (REST API)
```

#### 2. Advanced Querying
```c
// Error query interface
typedef struct {
    ErrorSeverity min_severity;
    ErrorCategory* categories;
    int category_count;
    time_t start_time;
    time_t end_time;
    char** tags;
    int tag_count;
} ErrorQuery;

Error** error_manager_query_errors(ErrorManager* manager, ErrorQuery* query, int* count);
```

#### 3. Statistical Analysis
```c
// Error statistics
typedef struct {
    int total_errors;
    int unresolved_errors;
    int errors_by_severity[4];  // Indexed by ErrorSeverity
    int errors_by_category[4];  // Indexed by ErrorCategory
    double average_resolution_time;
} ErrorStatistics;

ErrorStatistics error_manager_get_statistics(ErrorManager* manager);
```

#### 4. Export Capabilities
```c
// Multiple export formats
typedef enum {
    EXPORT_FORMAT_JSON = 0,
    EXPORT_FORMAT_XML = 1,
    EXPORT_FORMAT_CSV = 2
} ExportFormat;

char* error_manager_export_errors(ErrorManager* manager, ExportFormat format);
```

### Performance Improvements

1. **Caching Layer**: Add LRU cache for frequently accessed errors
2. **Bulk Operations**: Batch error creation and resolution
3. **Asynchronous Processing**: Non-blocking error management operations
4. **Compression**: Reduce memory usage for error storage

### Security Enhancements

1. **Authentication**: Secure error access controls
2. **Encryption**: Encrypt sensitive error data
3. **Audit Trail**: Complete operation logging
4. **Rate Limiting**: Prevent error reporting abuse

This comprehensive documentation provides complete coverage of the Todozi error management system, including architectural overview, detailed API references, implementation guidance, and operational procedures. The system is designed for extensibility and can be adapted to various application needs while maintaining robustness and reliability.