# Todozi Embedding Service - Comprehensive Documentation

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

The Todozi Embedding Service is a C library for managing text embeddings, supporting semantic search, similarity matching, and content clustering. It provides a comprehensive suite of utilities including string handling, vector operations, hash maps, and LRU caching.

### Key Features
- Text embedding generation and management
- Semantic similarity computation (cosine similarity, Euclidean distance)
- Content clustering capabilities
- Configurable caching with TTL support
- Multiple embedding model support
- Memory-efficient data structures

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
├─────────────────────────────────────────────────────────────┤
│  TodoziEmbeddingTool ─── TodoziEmbeddingService             │
├─────────────────────────────────────────────────────────────┤
│                   Service Layer                              │
├─────────────────────────────────────────────────────────────┤
│  CacheManager ─── EmbeddingModel ─── SimilarityEngine       │
├─────────────────────────────────────────────────────────────┤
│                  Utility Layer                               │
├─────────────────────────────────────────────────────────────┤
│  String ─── Vector ─── HashMap ─── LRUCache                 │
├─────────────────────────────────────────────────────────────┤
│                 Memory Management                            │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships
```
main() → TodoziEmbeddingTool → TodoziEmbeddingService
                                ↓
            EmbeddingModel ↔ CacheManager ↔ SimilarityEngine
                                ↓
                    String/Vector/HashMap Utilities
```

## Data Structures

### String Structure
```c
struct String {
    char* data;        // String data
    size_t len;        // Current length
    size_t capacity;   // Allocated capacity
};
```

### Vector Structure
```c
struct Vec {
    void** data;       // Array of pointers
    size_t size;       // Number of elements
    size_t capacity;   // Allocated capacity
};
```

### HashMap Structure
```c
struct HashMap {
    size_t size;                  // Number of entries
    size_t capacity;              // Number of buckets
    struct HashMapEntry** buckets; // Array of linked lists
};
```

### Configuration Structure
```c
struct TodoziEmbeddingConfig {
    struct String model_name;           // Model identifier
    size_t dimensions;                  // Embedding dimensions
    float similarity_threshold;         // Similarity cutoff
    size_t max_results;                 // Maximum search results
    size_t cache_ttl_seconds;          // Cache time-to-live
    bool enable_clustering;             // Clustering flag
    float clustering_threshold;         // Clustering similarity
};
```

## API Reference

### Utility Functions - String

#### `string_new`
```c
struct String string_new(const char* str)
```
Creates a new String from a C string.

**Parameters:**
- `str`: Source C string (can be NULL)

**Returns:**
- `struct String`: New string structure

**Error Handling:**
- Returns empty string if allocation fails
- Handles NULL input gracefully

#### `string_append`
```c
int string_append(struct String* s, const char* str)
```
Appends a string to an existing String.

**Parameters:**
- `s`: Target String pointer
- `str`: Source C string to append

**Returns:**
- `int`: EMB_SUCCESS or error code

**Error Codes:**
- `EMB_ERROR_NULL_POINTER`: Invalid input
- `EMB_ERROR_MEMORY`: Allocation failure

### Utility Functions - Vector

#### `vec_new`
```c
struct Vec vec_new(void)
```
Creates a new vector with default capacity.

**Returns:**
- `struct Vec`: New vector structure

#### `vec_push`
```c
void vec_push(struct Vec* v, void* item)
```
Adds an item to the vector.

**Parameters:**
- `v`: Vector pointer
- `item`: Item to add

**Memory Management:**
- Automatically resizes when capacity exceeded
- Uses doubling strategy for growth

### HashMap Functions

#### `hashmap_new`
```c
struct HashMap* hashmap_new(void)
```
Creates a new hash map with default capacity.

**Returns:**
- `struct HashMap*`: New hash map pointer

**Collision Resolution:**
- Uses separate chaining with linked lists

#### `hashmap_insert`
```c
int hashmap_insert(struct HashMap* map, const char* key, void* value)
```
Inserts a key-value pair into the hash map.

**Parameters:**
- `map`: Target hash map
- `key`: String key
- `value`: Associated value

**Returns:**
- `int`: EMB_SUCCESS or error code

**Hash Function:**
- Uses DJB2 algorithm: `hash * 33 + c`

### Service Functions

#### `service_new`
```c
struct TodoziEmbeddingService* service_new(struct TodoziEmbeddingConfig config)
```
Creates a new embedding service instance.

**Parameters:**
- `config`: Service configuration

**Returns:**
- `struct TodoziEmbeddingService*`: New service instance

**Memory Allocation:**
- Allocates service structure and all subcomponents
- Returns NULL on allocation failure

#### `service_find_similar_tasks`
```c
struct Vec service_find_similar_tasks(struct TodoziEmbeddingService* service, const char* task_description, size_t limit)
```
Finds tasks similar to the given description.

**Parameters:**
- `service`: Service instance
- `task_description`: Query text
- `limit`: Maximum results

**Returns:**
- `struct Vec`: Vector of similar tasks

## Usage Examples

### Basic Setup and Configuration
```c
#include "emb.h"

int main() {
    // Create default configuration
    struct TodoziEmbeddingConfig config = config_default();
    
    // Customize configuration
    config.similarity_threshold = 0.8f;
    config.max_results = 100;
    
    // Create tool instance
    struct TodoziEmbeddingTool* tool = tool_new(config);
    
    if (tool_initialize(tool) == EMB_SUCCESS) {
        printf("Service initialized successfully\n");
    }
    
    // Cleanup
    tool_free(tool);
    config_free(&config);
    return 0;
}
```

### String Operations
```c
// Create strings
struct String str1 = string_new("Hello");
struct String str2 = string_new_with_capacity(50);

// Append strings
string_append(&str1, " World!");
string_append(&str2, "Dynamic string");

// Cleanup
string_free(&str1);
string_free(&str2);
```

### HashMap Usage
```c
struct HashMap* map = hashmap_new();

// Insert values
int value1 = 42;
char* value2 = "test";
hashmap_insert(map, "number", &value1);
hashmap_insert(map, "text", value2);

// Retrieve values
int* retrieved = (int*)hashmap_get(map, "number");
printf("Value: %d\n", *retrieved);

// Cleanup
hashmap_free(map);
```

### Similarity Computation
```c
float vec1[] = {1.0f, 2.0f, 3.0f};
float vec2[] = {1.0f, 2.0f, 3.0f};
float vec3[] = {4.0f, 5.0f, 6.0f};

float sim1 = cosine_similarity(vec1, vec2, 3); // 1.0
float sim2 = cosine_similarity(vec1, vec3, 3); // ~0.974
float dist = euclidean_distance(vec1, vec3, 3); // ~5.196
```

## Design Patterns

### 1. Facade Pattern
- `TodoziEmbeddingTool` provides a simplified interface to complex subsystem
- Hides implementation details of service, cache, and model management

### 2. Builder Pattern
- `config_default()` provides sensible defaults
- Configuration can be customized before service creation

### 3. Strategy Pattern
- Multiple similarity functions (cosine, Euclidean, dot product)
- Different clustering algorithms can be implemented

### 4. Observer Pattern
- Cache expiration checking
- Statistics collection

### 5. Factory Pattern
- `embedding_model_load()` creates appropriate model instances
- `service_new()` creates complete service hierarchy

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| String append | O(n) amortized | Doubling strategy |
| Vector push | O(1) amortized | Resize on capacity |
| HashMap insert | O(1) average | Hash collision handling |
| HashMap get | O(1) average | Direct bucket access |
| Cosine similarity | O(n) | Vector dimension dependent |

### Memory Usage
- **String**: Grows exponentially (doubling strategy)
- **Vector**: Pre-allocates capacity, grows as needed
- **HashMap**: Fixed bucket count, dynamic entry allocation
- **Cache**: LRU eviction with memory limits

### Optimization Strategies
1. **Memory Pooling**: Reuse allocated blocks for frequent operations
2. **Cache Warming**: Pre-load frequently used embeddings
3. **Batch Processing**: Process multiple texts simultaneously
4. **Vectorization**: Use SIMD instructions for similarity computations

## Security Considerations

### Input Validation
```c
// All public functions validate input parameters
if (!service || !task_description) {
    return EMB_ERROR_NULL_POINTER;
}
```

### Memory Safety
- Bounds checking on all array accesses
- Null pointer validation
- Memory allocation failure handling

### Data Integrity
- Hash map key duplication prevention
- Cache TTL enforcement
- Configuration validation

### Potential Vulnerabilities
1. **Buffer Overflows**: Mitigated by capacity tracking
2. **Memory Leaks**: Comprehensive cleanup functions
3. **Integer Overflows**: Size_t used for memory sizes
4. **API Misuse**: Extensive error checking and return codes

## Testing Strategy

### Unit Testing Framework
```c
void test_string_operations() {
    struct String str = string_new("test");
    assert(str.len == 4);
    assert(strcmp(str.data, "test") == 0);
    string_free(&str);
}

void test_hashmap_functionality() {
    struct HashMap* map = hashmap_new();
    int value = 42;
    hashmap_insert(map, "key", &value);
    assert(*(int*)hashmap_get(map, "key") == 42);
    hashmap_free(map);
}
```

### Test Categories
1. **Unit Tests**: Individual function testing
2. **Integration Tests**: Component interaction testing
3. **Performance Tests**: Benchmarking critical operations
4. **Memory Tests**: Leak detection and allocation patterns

### Test Coverage Goals
- 90%+ function coverage
- 85%+ branch coverage
- All error paths tested
- Memory allocation/deallocation balanced

## Deployment Instructions

### Build Requirements
```bash
# Dependencies
gcc >= 9.0
make >= 4.0
cmake >= 3.10 (optional)

# Build commands
gcc -c emb.c -o emb.o
gcc -c main.c -o main.o
gcc emb.o main.o -lm -o todozi_embedding
```

### Platform Support
- **Linux**: Full support
- **macOS**: Full support
- **Windows**: Requires MinGW or WSL

### Integration Steps
1. Include header file: `#include "emb.h"`
2. Link with library: `-lemb`
3. Call initialization: `tool_initialize()`
4. Use service functions as needed

### Configuration Management
```c
// Environment-based configuration
const char* model_env = getenv("EMBEDDING_MODEL");
if (model_env) {
    config.model_name = string_new(model_env);
}
```

## Troubleshooting Guide

### Common Issues

#### Memory Leaks
**Symptoms**: Increasing memory usage over time
**Solution**: Ensure all allocated resources are freed
```c
// Proper cleanup sequence
tool_free(tool);
config_free(&config);
```

#### Performance Degradation
**Symptoms**: Slow similarity computations
**Solution**: Check cache hit rates, consider batch processing
```c
// Enable caching and monitor stats
struct HashMap* stats = service_get_stats(service);
```

#### Model Loading Failures
**Symptoms**: `service_initialize` returns error
**Solution**: Verify model name and dependencies
```c
// Check model availability
if (embedding_model_validate(model) != EMB_SUCCESS) {
    // Handle validation error
}
```

### Error Codes Reference
| Code | Meaning | Recovery Action |
|------|---------|-----------------|
| `EMB_SUCCESS` | Operation successful | Continue normal flow |
| `EMB_ERROR_NULL_POINTER` | Invalid null input | Check parameter validity |
| `EMB_ERROR_MEMORY` | Allocation failure | Reduce memory usage or check system |
| `EMB_ERROR_INVALID_INPUT` | Invalid parameter value | Validate input ranges |
| `EMB_ERROR_NOT_FOUND` | Resource not found | Check existence before access |

### Debugging Techniques
1. **Memory Profiling**: Use valgrind or address sanitizer
2. **Performance Profiling**: Use gprof or perf tools
3. **Logging**: Add debug prints for complex operations
4. **Assertions**: Use runtime checks for invariants

### Recovery Strategies
1. **Graceful Degradation**: Fall back to simpler algorithms
2. **Resource Cleanup**: Comprehensive cleanup on failure
3. **Error Propagation**: Clear error codes for caller handling
4. **State Validation**: Check service state before operations

This documentation provides comprehensive coverage of the Todozi Embedding Service implementation. The code demonstrates robust memory management, efficient data structures, and scalable architecture suitable for production use in embedding-based applications.