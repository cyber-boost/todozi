# Comprehensive Documentation for Todozi Tag Management System

## Table of Contents
1. [System Overview](#system-overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [API Reference](#api-reference)
5. [Design Patterns](#design-patterns)
6. [Performance Analysis](#performance-analysis)
7. [Security Considerations](#security-considerations)
8. [Testing Strategies](#testing-strategies)
9. [Deployment Instructions](#deployment-instructions)
10. [Troubleshooting Guide](#troubleshooting-guide)

## System Overview

Todozi is a comprehensive tag management system written in C that provides robust functionality for creating, updating, searching, and managing tags with relationships, categories, and statistics tracking. The system supports advanced search capabilities including fuzzy matching and tag suggestions.

### Key Features
- **Tag Management**: Create, read, update, delete operations
- **Tag Relationships**: Establish and manage relationships between tags
- **Category Support**: Organize tags into categories
- **Advanced Search**: Multi-criteria search with sorting
- **Fuzzy Search**: Levenshtein distance-based approximate matching
- **Statistics**: Usage tracking and analytics
- **Bulk Operations**: Batch tag creation and merging

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌──────────────────┐    ┌────────────────────┐
│   TagManager    │◇──│  TagSearchEngine │◇──│   TagSearchQuery   │
└─────────────────┘    └──────────────────┘    └────────────────────┘
         │                        │
         │                        │
┌─────────┼─────────┐    ┌─────────┼─────────┐
│         │         │    │         │         │
▼         ▼         ▼    ▼         ▼         ▼
┌─────┐  ┌─────┐  ┌─────┐  ┌─────────────┐  ┌─────────────┐
│ Tag │  │HashMap│ │Vector│ │TagStatistics│ │TagUpdate    │
└─────┘  └─────┘  └─────┘  └─────────────┘  └─────────────┘
```

### Component Relationships
- **TagManager**: Core management component storing tags in hash maps
- **TagSearchEngine**: Search functionality built on TagManager
- **HashMap**: Custom hash map implementation for data storage
- **Vector**: Dynamic array implementation for collections
- **Tag**: Individual tag entity with metadata
- **TagUpdate**: Builder pattern for tag modifications

## Data Structures

### Core Structures

#### Tag Structure
```c
struct Tag {
    char* id;                   // UUID identifier
    char* name;                 // Tag name
    char* description;          // Tag description
    char* color;                // Color representation
    char* category;             // Category classification
    unsigned int usage_count;   // Usage counter
    DateTime created_at;        // Creation timestamp
    DateTime updated_at;        // Last update timestamp
};
```

#### TagManager Structure
```c
struct TagManager {
    HashMap* tags;              // Primary tag storage (id → Tag)
    HashMap* tag_relationships; // Tag relationships (id → Vector<related_ids>)
    HashMap* category_tags;     // Category organization (category → Vector<tag_ids>)
};
```

#### Vector Structure
```c
typedef struct {
    void** data;                // Array of pointers
    size_t size;                // Current element count
    size_t capacity;            // Current allocation size
} Vector;
```

#### HashMap Structure
```c
typedef struct {
    HashMapEntry** buckets;     // Array of linked list heads
    size_t size;                // Number of entries
    size_t capacity;            // Number of buckets
} HashMap;
```

### Error Handling Enum
```c
typedef enum {
    TODOZI_SUCCESS,            // Operation successful
    TODOZI_VALIDATION_ERROR,   // Invalid input parameters
    TODOZI_OUT_OF_MEMORY       // Memory allocation failure
} TodoziError;
```

## API Reference

### Vector Operations

#### `vector_create()`
Creates a new vector with initial capacity.

**Parameters**: None  
**Returns**: `Vector*` - New vector instance or NULL on failure  
**Time Complexity**: O(1)  
**Memory Complexity**: O(1)

**Example**:
```c
Vector* vec = vector_create();
if (!vec) {
    // Handle allocation failure
}
```

#### `vector_push(Vector* vec, void* item)`
Adds an element to the end of the vector.

**Parameters**:
- `vec`: Vector to modify
- `item`: Pointer to element to add

**Returns**: void  
**Time Complexity**: O(1) amortized  
**Memory Complexity**: O(n)

**Example**:
```c
Vector* vec = vector_create();
vector_push(vec, some_data);
```

#### `vector_get(Vector* vec, size_t index)`
Retrieves an element at the specified index.

**Parameters**:
- `vec`: Vector to access
- `index`: Zero-based index

**Returns**: `void*` - Element pointer or NULL if invalid  
**Time Complexity**: O(1)  
**Memory Complexity**: O(1)

**Example**:
```c
void* data = vector_get(vec, 0);
if (data) {
    // Use the data
}
```

### HashMap Operations

#### `hashmap_create()`
Creates a new hash map with default capacity.

**Parameters**: None  
**Returns**: `HashMap*` - New hash map instance  
**Time Complexity**: O(1)  
**Memory Complexity**: O(1)

**Example**:
```c
HashMap* map = hashmap_create();
```

#### `hashmap_put(HashMap* map, const char* key, void* value)`
Inserts or updates a key-value pair.

**Parameters**:
- `map`: Hash map to modify
- `key`: String key
- `value`: Associated value

**Returns**: `TodoziError` - Operation status  
**Time Complexity**: O(1) average, O(n) worst-case  
**Memory Complexity**: O(1)

**Example**:
```c
TodoziError err = hashmap_put(map, "key1", data);
if (err != TODOZI_SUCCESS) {
    // Handle error
}
```

#### `hashmap_get(HashMap* map, const char* key)`
Retrieves a value by key.

**Parameters**:
- `map`: Hash map to search
- `key`: Key to lookup

**Returns**: `void*` - Associated value or NULL  
**Time Complexity**: O(1) average, O(n) worst-case  
**Memory Complexity**: O(1)

**Example**:
```c
void* value = hashmap_get(map, "key1");
```

### Tag Management Operations

#### `tag_manager_create()`
Creates a new tag manager instance.

**Parameters**: None  
**Returns**: `TagManager*` - New tag manager  
**Time Complexity**: O(1)  
**Memory Complexity**: O(1)

**Example**:
```c
TagManager* manager = tag_manager_create();
if (!manager) {
    // Handle creation failure
}
```

#### `tag_manager_create_tag(TagManager* manager, Tag* tag, char** out_id)`
Creates a new tag in the system.

**Parameters**:
- `manager`: Tag manager instance
- `tag`: Tag to create (id will be generated)
- `out_id`: Optional output parameter for created tag ID

**Returns**: `TodoziError` - Operation status  
**Time Complexity**: O(1) average  
**Memory Complexity**: O(1)

**Example**:
```c
Tag* new_tag = tag_create();
new_tag->name = string_clone("Important");
char* tag_id = NULL;
TodoziError err = tag_manager_create_tag(manager, new_tag, &tag_id);
```

#### `tag_manager_search_tags(TagManager* manager, const char* query)`
Searches tags by name or description.

**Parameters**:
- `manager`: Tag manager instance
- `query`: Search query string

**Returns**: `Vector*` - Vector of matching tags  
**Time Complexity**: O(n)  
**Memory Complexity**: O(k) where k is number of matches

**Example**:
```c
Vector* results = tag_manager_search_tags(manager, "important");
```

### Advanced Search Operations

#### `tag_search_engine_advanced_search(TagSearchEngine* engine, TagSearchQuery* query)`
Performs multi-criteria tag search.

**Parameters**:
- `engine`: Search engine instance
- `query`: Search criteria object

**Returns**: `Vector*` - Sorted and filtered results  
**Time Complexity**: O(n log n) for sorting  
**Memory Complexity**: O(n)

**Example**:
```c
TagSearchQuery* query = tag_search_query_create();
query->name_contains = string_clone("bug");
query->min_usage = malloc(sizeof(unsigned int));
*query->min_usage = 5;
Vector* results = tag_search_engine_advanced_search(engine, query);
```

#### `tag_search_engine_fuzzy_search(TagSearchEngine* engine, const char* query, size_t max_distance)`
Performs fuzzy string matching search.

**Parameters**:
- `engine`: Search engine instance
- `query`: Search string
- `max_distance`: Maximum Levenshtein distance

**Returns**: `Vector*` - Results sorted by similarity  
**Time Complexity**: O(n * m) where m is query length  
**Memory Complexity**: O(n)

**Example**:
```c
Vector* fuzzy_results = tag_search_engine_fuzzy_search(engine, "important", 2);
```

## Design Patterns

### Builder Pattern
**TagUpdate** structure uses builder pattern for incremental tag modification:
```c
TagUpdate* update = tag_update_new();
tag_update_name(update, "New Name");
tag_update_description(update, "New Description");
tag_manager_update_tag(manager, tag_id, update);
```

### Factory Pattern
**Tag** creation uses factory functions:
```c
Tag* tag = tag_create();  // Factory function
```

### Repository Pattern
**TagManager** acts as a repository for tag entities, providing data access abstraction.

### Strategy Pattern
Sorting algorithms are implemented as strategy functions passed to `qsort`.

## Performance Analysis

### Time Complexity Analysis

| Operation | Average Case | Worst Case | Notes |
|-----------|-------------|------------|--------|
| Tag creation | O(1) | O(n) | Hash map insertion |
| Tag lookup by ID | O(1) | O(n) | Hash map lookup |
| Tag search | O(n) | O(n) | Linear scan |
| Fuzzy search | O(n*m) | O(n*m) | Levenshtein per tag |
| Bulk operations | O(k) | O(k) | k = number of tags |
| Sorting | O(n log n) | O(n²) | Uses qsort |

### Memory Usage Analysis

| Component | Base Memory | Per Element |
|-----------|-------------|-------------|
| TagManager | ~200 bytes | ~100 bytes/tag |
| Vector | 24 bytes | 8 bytes/element |
| HashMap | 24 bytes | ~40 bytes/entry |
| Tag | 48 bytes | Variable string data |

### Optimization Strategies
1. **HashMap resizing**: Dynamic resizing with load factor management
2. **Vector doubling**: Amortized constant time appends
3. **String interning**: Consider for duplicate tag names
4. **Caching**: Query result caching for repeated searches

## Security Considerations

### Input Validation
- All public API functions validate NULL parameters
- String length checks prevent buffer overflows
- UUID validation for tag identifiers

### Memory Safety
- Comprehensive error handling for allocation failures
- Proper cleanup functions for all resources
- Defensive programming against memory leaks

### Data Integrity
- Atomic operations where possible
- Consistent state maintenance
- Transaction-like behavior for complex operations

### Security Best Practices
1. **Validation**: Always validate external inputs
2. **Sanitization**: Escape special characters in search queries
3. **Access Control**: Implement proper authorization (extension point)
4. **Audit Logging**: Track tag modifications (extension point)

## Testing Strategies

### Unit Testing Framework
```c
// Example test function
void test_tag_creation() {
    TagManager* manager = tag_manager_create();
    assert(manager != NULL);
    
    Tag* tag = tag_create();
    tag->name = string_clone("Test Tag");
    
    TodoziError err = tag_manager_create_tag(manager, tag, NULL);
    assert(err == TODOZI_SUCCESS);
    
    tag_manager_free(manager);
    tag_free(tag);
}
```

### Test Categories

#### Functional Tests
- Tag lifecycle management
- Search functionality validation
- Relationship management
- Error handling verification

#### Performance Tests
- Large dataset handling
- Memory usage profiling
- Concurrent access patterns

#### Integration Tests
- End-to-end workflow validation
- Data persistence integration
- API interface testing

### Test Data Generation
```c
// Helper function for test data
Vector* generate_test_tags(size_t count) {
    Vector* tags = vector_create();
    for (size_t i = 0; i < count; i++) {
        Tag* tag = tag_create();
        tag->name = malloc(32);
        sprintf(tag->name, "TestTag%zu", i);
        vector_push(tags, tag);
    }
    return tags;
}
```

## Deployment Instructions

### Prerequisites
- C99 compatible compiler (GCC, Clang)
- UUID development library (`libuuid`)
- Standard C library

### Build Instructions
```bash
# Compile with GCC
gcc -std=c99 -Wall -Wextra -pedantic -luuid todozi.c -o todozi

# Compile with Clang
clang -std=c99 -Wall -Wextra -pedantic -luuid todozi.c -o todozi
```

### Library Installation
```bash
# Ubuntu/Debian
sudo apt-get install libuuid1 libuuid-dev

# CentOS/RHEL
sudo yum install libuuid libuuid-devel

# macOS (Homebrew)
brew install ossp-uuid
```

### Integration with Applications
```c
// Example integration
#include "todozi.h"

int main() {
    TagManager* manager = tag_manager_create();
    if (!manager) return 1;
    
    // Use tag management functionality
    Tag* tag = tag_create();
    tag->name = string_clone("Integration Test");
    tag_manager_create_tag(manager, tag, NULL);
    
    tag_manager_free(manager);
    return 0;
}
```

### Production Deployment Checklist
1. [ ] Memory leak testing completed
2. [ ] Performance benchmarking passed
3. [ ] Security review conducted
4. [ ] Integration testing validated
5. [ ] Documentation reviewed and updated

## Troubleshooting Guide

### Common Issues and Solutions

#### Memory Leaks
**Symptoms**: Increasing memory usage over time
**Solutions**:
- Use valgrind for leak detection: `valgrind --leak-check=full ./todozi`
- Ensure all allocated memory is properly freed
- Check vector and hashmap cleanup functions

```bash
valgrind --leak-check=full --show-leak-kinds=all ./todozi
```

#### Performance Degradation
**Symptoms**: Slow search operations with large datasets
**Solutions**:
- Implement result caching
- Use more efficient data structures for large datasets
- Profile with `gprof` to identify bottlenecks

```bash
gcc -pg -std=c99 todozi.c -luuid -o todozi
./todozi
gprof todozi gmon.out > analysis.txt
```

#### UUID Generation Failures
**Symptoms**: Tag creation fails with NULL IDs
**Solutions**:
- Check libuuid installation
- Verify library linking
- Implement fallback UUID generation

#### Search Functionality Issues
**Symptoms**: Incorrect or missing search results
**Solutions**:
- Verify string comparison logic
- Check case sensitivity settings
- Test with known data sets

### Debugging Techniques

#### Logging Implementation
```c
#ifdef DEBUG
#define TODOZI_LOG(fmt, ...) printf("TODOZI: " fmt "\n", ##__VA_ARGS__)
#else
#define TODOZI_LOG(fmt, ...)
#endif
```

#### Assertion Checking
```c
#include <assert.h>

TodoziError tag_manager_create_tag(TagManager* manager, Tag* tag, char** out_id) {
    assert(manager != NULL);
    assert(tag != NULL);
    // ... rest of implementation
}
```

### Error Handling Patterns

#### Comprehensive Error Reporting
```c
const char* todozi_error_string(TodoziError error) {
    switch (error) {
        case TODOZI_SUCCESS: return "Success";
        case TODOZI_VALIDATION_ERROR: return "Validation error";
        case TODOZI_OUT_OF_MEMORY: return "Out of memory";
        default: return "Unknown error";
    }
}
```

#### Resource Cleanup on Failure
```c
TodoziError complex_operation() {
    Resource* res1 = acquire_resource();
    if (!res1) return TODOZI_OUT_OF_MEMORY;
    
    Resource* res2 = acquire_resource();
    if (!res2) {
        release_resource(res1);  // Cleanup on failure
        return TODOZI_OUT_OF_MEMORY;
    }
    
    // ... operation logic
    
    release_resource(res1);
    release_resource(res2);
    return TODOZI_SUCCESS;
}
```

This documentation provides a comprehensive reference for the Todozi tag management system, covering all aspects from architecture to troubleshooting. The system demonstrates robust C programming practices with attention to memory management, error handling, and performance considerations.