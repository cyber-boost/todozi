# Todozi Content Extraction Library - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive content extraction and management system that processes unstructured text to extract tasks, memories, ideas, errors, and training data. The library integrates with external APIs and provides multiple output formats for the extracted content.

### Key Features
- **Multi-format Content Extraction**: Extracts tasks, memories, ideas, errors, and training data
- **API Integration**: Communicates with Todozi's external API service
- **Multiple Output Formats**: JSON, CSV, Markdown, and human-readable checklists
- **Automatic Persistence**: Saves extracted content to project files
- **Comprehensive Error Handling**: Robust error management system
- **Memory Safety**: Safe memory allocation and deallocation practices

## Architecture

### System Architecture Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Input Source  │───▶│  Content Parser  │───▶│   API Client    │
│   (File/Text)   │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────┬───────┘
                                                         │
┌─────────────────┐    ┌──────────────────┐    ┌─────────▼───────┐
│  Output Format  │◀───│ Response Parser  │◀───│  API Response   │
│   Generator     │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │
         ▼
┌─────────────────┐    ┌──────────────────┐
│   File System   │◀───│  Data Persister  │
│   (Projects)    │    │                  │
└─────────────────┘    └──────────────────┘
```

### Component Relationships

```
Main Functions
├── extract_content()          # Primary extraction function
├── strategy_content()         # Strategic content extraction
├── extract_with_endpoint()    # Generic API caller
└── Format Functions
    ├── format_as_csv()
    ├── format_as_markdown()
    └── format_as_human_checklist()

Data Structures
├── ExtractResponse            # Container for all extracted data
├── Task/Memory/Idea           # Domain objects
└── Extracted* structures      # Parsed API responses

Support Functions
├── Memory Management
├── Configuration Handling
├── UUID Generation
└── File I/O Operations
```

## Data Structures

### Enumeration Types

#### TodoziError
```c
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_IO,           // File/IO operations failed
    TODOZI_ERROR_VALIDATION,   // Input validation failed
    TODOZI_ERROR_CONFIG,       // Configuration errors
    TODOZI_ERROR_API,          // API communication errors
    TODOZI_ERROR_SERIALIZATION // JSON serialization/parsing errors
} TodoziError;
```

#### Priority Levels
```c
typedef enum {
    PRIORITY_LOW,      // Low priority tasks
    PRIORITY_MEDIUM,   // Medium priority (default)
    PRIORITY_HIGH,     // High priority tasks
    PRIORITY_CRITICAL  // Critical priority tasks
} Priority;
```

#### Status Types
```c
typedef enum {
    STATUS_TODO,        // Task not started
    STATUS_IN_PROGRESS, // Task in progress
    STATUS_DONE,        // Task completed
    STATUS_CANCELLED    // Task cancelled
} Status;
```

#### Memory Importance
```c
typedef enum {
    MEMORY_IMPORTANCE_LOW,
    MEMORY_IMPORTANCE_MEDIUM,
    MEMORY_IMPORTANCE_HIGH,
    MEMORY_IMPORTANCE_CRITICAL
} MemoryImportance;
```

### Core Data Structures

#### ExtractResponse
```c
typedef struct {
    ExtractedTask* tasks;           // Array of extracted tasks
    int tasks_count;                // Number of tasks
    
    ExtractedMemory* memories;      // Array of extracted memories
    int memories_count;             // Number of memories
    
    ExtractedIdea* ideas;           // Array of extracted ideas
    int ideas_count;                // Number of ideas
    
    ExtractedError* errors;         // Array of extracted errors
    int errors_count;               // Number of errors
    
    ExtractedTrainingData* training_data; // Training data items
    int training_data_count;        // Number of training items
    
    char** raw_tags;                // Raw tag strings
    int raw_tags_count;             // Number of raw tags
} ExtractResponse;
```

#### Task Structure
```c
typedef struct {
    char* id;                   // UUID identifier
    char* user_id;              // User identifier
    char* action;               // Task description
    char* time;                 // Time estimate/requirement
    Priority priority;          // Priority level
    char* parent_project;       // Parent project ID
    Status status;              // Current status
    char* assignee;             // Assigned person
    char** tags;                // Array of tags
    int tags_count;             // Number of tags
    char** dependencies;        // Task dependencies
    int dependencies_count;     // Number of dependencies
    char* context;              // Context information
    int progress;               // Progress percentage (0-100)
} Task;
```

#### Memory Structure
```c
typedef struct {
    char* id;                   // UUID identifier
    char* user_id;              // User identifier
    char* project_id;           // Associated project
    ItemStatus status;          // Active/Archived/Deleted
    char* moment;               // Memory moment description
    char* meaning;              // Meaning/interpretation
    char* reason;               // Reason for importance
    MemoryImportance importance;// Importance level
    MemoryTerm term;            // Time term (short/medium/long)
    MemoryType memory_type;     // Type of memory
    char** tags;                // Associated tags
    int tags_count;             // Number of tags
    time_t created_at;          // Creation timestamp
    time_t updated_at;          // Last update timestamp
} Memory;
```

## API Reference

### Main Extraction Functions

#### `extract_content()`
```c
TodoziError extract_content(
    const char* content,        // Input text content (NULL if using file)
    const char* file_path,      // Path to input file (NULL if using content)
    const char* output_format,  // "json", "csv", "md", "markdown"
    int human,                  // 1 to generate human checklist, 0 otherwise
    char** result               // Output buffer (caller must free)
);
```

**Parameters:**
- `content`: Raw text content to extract from
- `file_path`: Alternative file path to read content from
- `output_format`: Desired output format
- `human`: Flag for human-readable checklist generation
- `result`: Pointer to output string (allocated by function)

**Returns:** `TodoziError` indicating success or failure

**Usage:**
```c
char* result = NULL;
TodoziError error = extract_content(
    "Complete project documentation by Friday",
    NULL,  // No file path
    "json", // JSON output
    1,     // Generate human checklist
    &result
);

if (error == TODOZI_SUCCESS) {
    printf("Extraction successful: %s\n", result);
    free(result);
}
```

#### `strategy_content()`
```c
TodoziError strategy_content(
    const char* content,
    const char* file_path,
    const char* output_format,
    int human,
    char** result
);
```

Calls the "strategic" API endpoint for strategic planning content extraction.

### Core Internal Function

#### `extract_with_endpoint()`
```c
TodoziError extract_with_endpoint(
    const char* content,
    const char* file_path,
    const char* output_format,
    int human,
    const char* endpoint,      // API endpoint ("plan" or "strategic")
    char** result
);
```

**Implementation Flow:**
1. **Input Validation**: Validate input parameters and security constraints
2. **Content Acquisition**: Read from file or use provided content
3. **Configuration Loading**: Load user configuration and API key
4. **API Request**: Build and send HTTP request to Todozi API
5. **Response Handling**: Parse JSON response and extract structured data
6. **Data Persistence**: Save extracted content to project files
7. **Format Generation**: Convert to requested output format
8. **Checklist Generation**: Create human-readable checklist if requested

### Memory Management Functions

#### `safe_malloc()`
```c
void* safe_malloc(size_t size);
```
**Description:** Wrapper around malloc that exits on failure
**Parameters:** `size` - Number of bytes to allocate
**Returns:** Pointer to allocated memory, never NULL

#### `safe_strdup()`
```c
char* safe_strdup(const char* str);
```
**Description:** Safe string duplication with error handling
**Parameters:** `str` - String to duplicate
**Returns:** New string copy, NULL if input is NULL

#### Memory Freeing Functions
```c
void free_extract_response(ExtractResponse* response);
void free_extracted_task(ExtractedTask* task);
void free_extracted_memory(ExtractedMemory* memory);
// ... and other free functions
```

### Factory Functions

#### `create_task()`
```c
Task* create_task(
    const char* user_id,
    const char* action,
    const char* time,
    Priority priority,
    const char* project_id,
    Status status,
    const char* assignee,
    char** tags,
    int tags_count,
    char** dependencies,
    int dependencies_count,
    const char* context,
    int progress,
    TodoziError* error
);
```

**Returns:** Newly allocated Task structure with generated UUID

## Usage Examples

### Basic Content Extraction
```c
#include "todozi.h"

int main() {
    char* result = NULL;
    TodoziError error;
    
    // Extract from inline text
    error = extract_content(
        "Meeting with team at 2 PM. High priority. Assign to John.",
        NULL,
        "json",
        0,
        &result
    );
    
    if (error == TODOZI_SUCCESS) {
        printf("Extracted: %s\n", result);
        free(result);
    }
    
    return 0;
}
```

### File-based Extraction with Checklist
```c
int main() {
    char* result = NULL;
    TodoziError error;
    
    // Extract from file and generate human checklist
    error = extract_content(
        NULL,
        "/path/to/project_notes.txt",
        "markdown",
        1,  // Generate human checklist
        &result
    );
    
    if (error == TODOZI_SUCCESS) {
        // Save markdown result to file
        FILE* out = fopen("extracted.md", "w");
        fputs(result, out);
        fclose(out);
        free(result);
    }
    
    return 0;
}
```

### Strategic Content Processing
```c
int main() {
    char* result = NULL;
    
    // Process strategic content
    TodoziError error = strategy_content(
        "Q4 goals: Increase revenue by 20%, launch new product line",
        NULL,
        "csv",
        1,
        &result
    );
    
    if (error == TODOZI_SUCCESS) {
        // Process CSV result
        printf("Strategic analysis: %s\n", result);
        free(result);
    }
    
    return 0;
}
```

### Error Handling Example
```c
int main() {
    char* result = NULL;
    TodoziError error = extract_content("test", NULL, "json", 0, &result);
    
    switch (error) {
        case TODOZI_SUCCESS:
            printf("Success: %s\n", result);
            free(result);
            break;
        case TODOZI_ERROR_IO:
            printf("I/O error occurred\n");
            break;
        case TODOZI_ERROR_API:
            printf("API communication failed\n");
            break;
        case TODOZI_ERROR_CONFIG:
            printf("Configuration error - check API key\n");
            break;
        default:
            printf("Unknown error: %d\n", error);
    }
    
    return 0;
}
```

## Design Patterns

### 1. Factory Pattern
**Implementation:** `create_task()`, `create_memory()`, `create_idea()`
**Purpose:** Centralized object creation with consistent initialization
**Benefits:** 
- Encapsulates complex initialization logic
- Ensures consistent object state
- Simplifies object creation throughout codebase

### 2. Builder Pattern
**Implementation:** API request building in `extract_with_endpoint()`
**Purpose:** Step-by-step construction of complex API requests
**Benefits:**
- Flexible request configuration
- Validation at each step
- Easy modification of request parameters

### 3. Strategy Pattern
**Implementation:** Output format generators (`format_as_csv()`, `format_as_markdown()`)
**Purpose:** Interchangeable output formatting algorithms
**Benefits:**
- Easy addition of new output formats
- Clean separation of formatting logic
- Runtime format selection

### 4. Resource Acquisition Is Initialization (RAII)
**Implementation:** Comprehensive cleanup in `extract_with_endpoint()`
**Purpose:** Automatic resource management through structured cleanup
**Benefits:**
- Prevents memory leaks
- Consistent error handling
- Simplified resource management

### 5. Adapter Pattern
**Implementation:** JSON parsing adapts external API responses to internal structures
**Purpose:** Bridge between external API format and internal data structures
**Benefits:**
- Isolation of external dependencies
- Easy API version migration
- Clean data transformation

## Performance Analysis

### Time Complexity
| Operation | Complexity | Description |
|-----------|------------|-------------|
| JSON Parsing | O(n) | Linear to response size |
| Memory Allocation | O(1) | Constant time per allocation |
| String Operations | O(n) | Linear to string length |
| API Request | O(1) | Network-bound, constant |

### Space Complexity
| Component | Complexity | Description |
|-----------|------------|-------------|
| ExtractResponse | O(n) | Proportional to extracted items |
| JSON Parsing | O(n) | Linear to JSON depth and size |
| String Buffers | O(n) | Proportional to content size |

### Memory Usage Optimization
- **String Reuse**: Reference counting for common strings
- **Buffer Pooling**: Reusable buffers for formatting
- **Lazy Allocation**: Memory allocated only when needed
- **Early Freeing**: Immediate freeing of temporary objects

### Performance Tips
1. **Batch Processing**: Process multiple items together when possible
2. **Stream Processing**: Use streaming JSON parsing for large responses
3. **Memory Pools**: Implement object pools for frequent allocations
4. **Caching**: Cache configuration and API responses when appropriate

## Security Considerations

### Input Validation
```c
// Buffer size validation
if (strlen(endpoint) > 256) {
    rc = TODOZI_ERROR_VALIDATION;
    goto cleanup;
}

// API key length validation
if (!api_key || strlen(api_key) > 400) {
    rc = TODOZI_ERROR_VALIDATION;
    // Handle error
}
```

### Security Measures Implemented

#### 1. Input Sanitization
- **Path Validation**: Check file paths for directory traversal
- **Size Limits**: Enforce maximum sizes for all inputs
- **Type Checking**: Validate JSON types before processing

#### 2. API Security
- **Key Redaction**: API keys never printed to stdout
- **HTTPS Enforcement**: All API calls use secure HTTPS
- **Header Validation**: Validate HTTP headers before processing

#### 3. Memory Safety
- **Bounds Checking**: All buffer operations include bounds checks
- **Null Termination**: Ensure all strings are properly terminated
- **Safe String Functions**: Use length-limited string functions

#### 4. File System Security
- **Path Length Limits**: Prevent buffer overflow in path operations
- **Directory Creation**: Safe directory creation with proper permissions
- **File Access Control**: Validate file operations succeed

### Security Best Practices
1. **Never log sensitive data** (API keys, user identifiers)
2. **Validate all external inputs** before processing
3. **Use secure memory allocation** practices
4. **Implement proper error handling** to avoid information leakage
5. **Regular security audits** of the codebase

## Testing Strategies

### Unit Testing Framework

#### Test Structure Example
```c
// test_todozi.c
#include "todozi.h"
#include <assert.h>

void test_extract_content_basic() {
    char* result = NULL;
    TodoziError error = extract_content(
        "Test task with high priority",
        NULL,
        "json",
        0,
        &result
    );
    
    assert(error == TODOZI_SUCCESS);
    assert(result != NULL);
    // Validate JSON structure
    free(result);
}

void test_memory_management() {
    ExtractResponse* response = create_test_response();
    free_extract_response(response);
    // Use valgrind to check for leaks
}

void test_error_handling() {
    char* result = NULL;
    TodoziError error = extract_content(NULL, NULL, "json", 0, &result);
    assert(error == TODOZI_ERROR_VALIDATION);
}
```

### Integration Testing

#### API Integration Tests
```c
void test_api_integration() {
    // Mock API server required
    // Test successful API calls
    // Test error responses
    // Test timeout scenarios
}
```

#### File System Tests
```c
void test_file_operations() {
    // Test file reading
    // Test directory creation
    // Test permission handling
    // Test error conditions
}
```

### Performance Testing

#### Benchmark Suite
```c
void benchmark_large_content() {
    // Test with large input files
    // Measure memory usage
    // Measure processing time
    // Identify bottlenecks
}
```

### Test Coverage Goals
- **Function Coverage**: 100% of functions tested
- **Branch Coverage**: 90%+ of conditional branches
- **Path Coverage**: Critical paths thoroughly tested
- **Memory Testing**: Valgrind for leak detection

## Deployment Instructions

### Build Requirements

#### Dependencies
```bash
# Required libraries
libcurl-dev    # HTTP client functionality
json-c-dev     # JSON parsing and generation
libuuid-dev    # UUID generation
```

#### Build Configuration
```makefile
# Makefile example
CC = gcc
CFLAGS = -Wall -Wextra -Werror -O2
LDFLAGS = -lcurl -ljson-c -luuid

todozi.o: todozi.c todozi.h
	$(CC) $(CFLAGS) -c todozi.c -o todozi.o

libtodozi.a: todozi.o
	ar rcs libtodozi.a todozi.o
```

### Installation Steps

#### Step 1: Install Dependencies
```bash
# Ubuntu/Debian
sudo apt-get install libcurl4-openssl-dev libjson-c-dev uuid-dev

# CentOS/RHEL
sudo yum install libcurl-devel json-c-devel libuuid-devel
```

#### Step 2: Build Library
```bash
git clone https://github.com/todozi/library.git
cd library
make
sudo make install
```

#### Step 3: Configuration
```bash
# Set API key environment variable
export TODOZI_API_KEY="your_api_key_here"

# Create configuration directory
mkdir -p ~/.todozi
```

#### Step 4: Verification
```c
// test_installation.c
#include <todozi.h>
#include <stdio.h>

int main() {
    printf("Todozi library version: %s\n", TODOZI_VERSION);
    return 0;
}
```

### Docker Deployment
```dockerfile
FROM ubuntu:20.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    libcurl4-openssl-dev \
    libjson-c-dev \
    uuid-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . /app
WORKDIR /app

# Build and install
RUN make && make install

# Set environment
ENV TODOZI_API_KEY="your_key_here"
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. API Connection Issues
**Symptoms:** `TODOZI_ERROR_API` returned, curl errors in logs
**Solutions:**
```bash
# Check network connectivity
ping todozi.com

# Verify API key
echo $TODOZI_API_KEY

# Test curl independently
curl -H "Authorization: Bearer $TODOZI_API_KEY" https://todozi.com/api/tdz/plan
```

#### 2. Memory Allocation Errors
**Symptoms:** Program crashes with allocation errors
**Solutions:**
```c
// Check system memory
#include <sys/sysinfo.h>
struct sysinfo info;
sysinfo(&info);
printf("Free memory: %lu MB\n", info.freeram / 1024 / 1024);

// Use valgrind for leak detection
valgrind --leak-check=full ./your_program
```

#### 3. File Permission Errors
**Symptoms:** `TODOZI_ERROR_IO` when accessing files
**Solutions:**
```bash
# Check file permissions
ls -la ~/.todozi/

# Fix permissions if needed
chmod 755 ~/.todozi/
chmod 600 ~/.todozi/tdz.hlx
```

#### 4. JSON Parsing Errors
**Symptoms:** `TODOZI_ERROR_SERIALIZATION` returned
**Solutions:**
```c
// Enable verbose JSON parsing
json_object* obj = json_tokener_parse_verbose(response, &error);
if (!obj) {
    printf("JSON error: %s\n", json_tokener_error_desc(error));
}
```

### Debugging Techniques

#### Verbose Logging
```c
// Add debug prints throughout the code
#ifdef DEBUG
#define DBG_PRINT(fmt, ...) printf("DEBUG: " fmt, ##__VA_ARGS__)
#else
#define DBG_PRINT(fmt, ...)
#endif

// Usage
DBG_PRINT("Processing task %d: %s\n", i, task->action);
```

#### Memory Debugging
```bash
# Use valgrind for detailed memory analysis
valgrind --tool=memcheck --leak-check=yes ./program

# Use address sanitizer for build
gcc -fsanitize=address -g todozi.c -o todozi
```

### Performance Troubleshooting

#### Identify Bottlenecks
```c
#include <time.h>

// Time specific operations
clock_t start = clock();
// Operation to time
clock_t end = clock();
double elapsed = (double)(end - start) / CLOCKS_PER_SEC;
printf("Operation took: %f seconds\n", elapsed);
```

#### Memory Profiling
```bash
# Use massif for heap profiling
valgrind --tool=massif ./program
ms_print massif.out.*
```

### Recovery Procedures

#### API Failure Recovery
```c
// Implement retry logic with exponential backoff
int max_retries = 3;
int retry_delay = 1; // seconds

for (int i = 0; i < max_retries; i++) {
    error = extract_with_endpoint(...);
    if (error != TODOZI_ERROR_API) break;
    sleep(retry_delay * (1 << i)); // Exponential backoff
}
```

#### Data Corruption Recovery
```c
// Implement data validation before processing
int validate_extract_response(const ExtractResponse* response) {
    if (!response) return 0;
    if (response->tasks_count < 0) return 0;
    // Additional validation checks
    return 1;
}
```

This documentation provides comprehensive coverage of the Todozi content extraction library. The system is designed for robustness, security, and performance while maintaining flexibility for various use cases.