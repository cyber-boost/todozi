# Todozi Memory Management System - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Core Components](#core-components)
5. [API Reference](#api-reference)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategy](#testing-strategy)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)
12. [Usage Examples](#usage-examples)

## Overview

The Todozi Memory Management System is a C library designed for managing various types of memories with different classifications, importance levels, and metadata. It provides a comprehensive suite of functions for creating, updating, searching, and analyzing memories with support for tags, statistics, and advanced querying capabilities.

### Key Features
- Multi-type memory classification (Standard, Secret, Human, Emotional, Short, Long)
- Importance-based prioritization (Low, Medium, High, Critical)
- Term-based categorization (Short-term, Long-term)
- Tag-based organization and search
- Statistical analysis and reporting
- Text-based memory parsing
- Error handling and validation

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   MemoryManager │◄──►│     HashMap     │◄──►│    Memory       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ MemoryStatistics│    │  MemoryUpdate   │    │      Vector     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### System Flow
```
Memory Creation → Memory Storage → Memory Query → Statistics Generation
       ↑                ↑                ↑                ↑
   Text Parsing     Hash Mapping     Search Logic    Data Analysis
```

## Data Structures

### Memory
```c
struct Memory {
    char* id;                    // UUID identifier
    char* user_id;               // Owner identifier
    char* project_id;            // Optional project association
    ItemStatus status;           // ACTIVE, COMPLETED, ARCHIVED
    char* moment;                // Memory content/description
    char* meaning;               // Significance/interpretation
    char* reason;                // Purpose/justification
    MemoryImportance importance; // LOW, MEDIUM, HIGH, CRITICAL
    MemoryTerm term;             // SHORT, LONG term
    MemoryType memory_type;      // STANDARD, SECRET, HUMAN, etc.
    char* emotion;               // For emotional memories
    Vector* tags;                // String vector of tags
    time_t created_at;           // Creation timestamp
    time_t updated_at;           // Last update timestamp
};
```

### MemoryManager
```c
struct MemoryManager {
    HashMap* memories;      // Memory storage (id → Memory*)
    HashMap* memory_tags;   // Tag index (memory_id → Vector* of tags)
};
```

### MemoryUpdate
```c
struct MemoryUpdate {
    char* moment;               // Optional update
    char* meaning;              // Optional update  
    char* reason;               // Optional update
    MemoryImportance* importance; // Optional pointer update
    MemoryTerm* term;           // Optional pointer update
    Vector* tags;               // Optional tag replacement
};
```

## Core Components

### 1. HashMap Implementation
**Purpose**: Provides key-value storage for efficient memory lookup

**Functions**:
- `hashmap_create(size_t capacity)`: Creates a new hashmap
- `hashmap_destroy(HashMap*, void (*free_key)(void*), void (*free_value)(void*))`: Cleans up hashmap
- `hashmap_put(HashMap*, const char* key, void* value)`: Stores key-value pair
- `hashmap_get(HashMap*, const char* key)`: Retrieves value by key
- `hashmap_remove(HashMap*, const char* key)`: Removes key-value pair
- `hashmap_size(HashMap*)`: Returns number of entries

**Hash Function**: Uses DJB2 algorithm for string hashing

### 2. Vector Implementation
**Purpose**: Dynamic array implementation for flexible collections

**Functions**:
- `vector_create()`: Creates empty vector
- `vector_destroy(Vector*, void (*free_element)(void*))`: Cleans up vector
- `vector_push(Vector*, void* element)`: Adds element to end
- `vector_get(Vector*, size_t index)`: Retrieves element by index
- `vector_size(Vector*)`: Returns number of elements

### 3. Memory Management Core
**Purpose**: Main API for memory operations

**Key Functions**:
- `memory_manager_create()`: Initializes memory manager
- `memory_manager_create_memory()`: Adds new memory
- `memory_manager_update_memory()`: Modifies existing memory
- `memory_manager_delete_memory()`: Removes memory
- `memory_manager_search_memories()`: Text-based search
- `memory_manager_get_memory_statistics()`: Generates analytics

## API Reference

### Memory Creation and Management

#### `Memory* memory_create(void)`
**Purpose**: Creates a new empty memory structure
**Returns**: Pointer to allocated Memory structure, NULL on failure
**Memory Allocation**: Allocates memory for structure and internal vector

#### `TodoziError* memory_manager_create_memory(MemoryManager* manager, Memory* memory)`
**Parameters**:
- `manager`: MemoryManager instance
- `memory`: Pre-configured Memory structure

**Returns**: TodoziError* on failure, NULL on success
**Behavior**: Generates UUID, sets timestamps, stores in hashmap

#### `TodoziError* memory_manager_update_memory(MemoryManager* manager, const char* memory_id, MemoryUpdate* updates)`
**Parameters**:
- `manager`: MemoryManager instance  
- `memory_id`: UUID of memory to update
- `updates`: MemoryUpdate structure with modifications

**Returns**: TodoziError* on failure, NULL on success
**Validation**: Checks memory existence, applies non-NULL updates

### Query Operations

#### `Vector* memory_manager_search_memories(MemoryManager* manager, const char* query)`
**Purpose**: Case-insensitive text search across moments, meanings, reasons, and tags
**Algorithm**: Linear scan with string duplication for case conversion
**Performance**: O(n) where n is number of memories

#### `Vector* memory_manager_get_memories_by_importance(MemoryManager* manager, MemoryImportance importance)`
**Purpose**: Filters memories by importance level
**Performance**: O(n) linear scan

#### `Vector* memory_manager_get_recent_memories(MemoryManager* manager, size_t limit)`
**Purpose**: Returns most recently created memories
**Algorithm**: Quicksort on creation timestamp, then limit selection
**Performance**: O(n log n) for sorting

### Statistical Functions

#### `MemoryStatistics* memory_manager_get_memory_statistics(MemoryManager* manager)`
**Purpose**: Generates comprehensive memory analytics
**Metrics Tracked**:
- Total memories count
- Short/long term distribution  
- Critical memory count
- Unique tags count
- Type-based classifications
- Percentage calculations

## Design Patterns

### 1. Builder Pattern
**Implementation**: `MemoryUpdate` structure with chained setters
```c
MemoryUpdate* update = memory_update_create();
memory_update_moment(update, "New moment")
          ->memory_update_importance(update, MEMORY_IMPORTANCE_HIGH);
```

### 2. Factory Pattern  
**Implementation**: `parse_memory_format()` function creates Memory from text
```c
TodoziError* error = parse_memory_format(memory_text, "user_123", &memory);
```

### 3. Repository Pattern
**Implementation**: `MemoryManager` acts as data access layer
```c
Memory* memory = memory_manager_get_memory(manager, memory_id);
```

### 4. Strategy Pattern
**Implementation**: Different search strategies via specialized functions
```c
Vector* by_importance = memory_manager_get_memories_by_importance(manager, HIGH);
Vector* by_tag = memory_manager_get_memories_by_tag(manager, "work");
```

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Memory Creation | O(1) | HashMap insertion with good distribution |
| Memory Retrieval | O(1) average | HashMap lookup |
| Memory Update | O(1) average | HashMap lookup + field update |
| Memory Deletion | O(1) average | HashMap removal |
| Text Search | O(n) | Linear scan with string operations |
| Filter by Property | O(n) | Linear scan through all memories |
| Statistics Generation | O(n) | Multiple linear scans for different metrics |

### Space Complexity
- Memory Storage: O(n) where n is number of memories
- Tag Index: O(n × t) where t is average tags per memory  
- Search Results: O(k) where k is number of matching memories

### Optimization Opportunities
1. **Indexing**: Add secondary indexes for common queries (importance, term, type)
2. **Caching**: Cache statistics and frequent query results
3. **Pagination**: Implement limit/offset for large result sets
4. **Batch Operations**: Add batch creation/deletion operations

## Security Considerations

### 1. Input Validation
**Current Implementation**: Basic null checks and memory existence validation
**Improvements Needed**:
- Input length limits to prevent buffer overflows
- Sanitization of user-provided strings
- Validation of enum values ranges

### 2. Memory Safety
**Strengths**:
- Consistent memory allocation/deallocation patterns
- Ownership transfer clarity in API design
- Error handling for allocation failures

**Concerns**:
- Potential memory leaks if error paths not properly handled
- No bounds checking on string operations in search functions

### 3. Data Privacy
**Considerations**:
- Secret memory type indicates sensitive data handling needed
- Emotion and human memory types may contain personal information
- Recommendation: Implement encryption for sensitive memory fields

### 4. API Security
**Recommendations**:
- Add user authentication/authorization checks
- Validate user_id ownership before operations
- Implement rate limiting for API calls

## Testing Strategy

### Unit Testing Framework
```c
// Example test structure
void test_memory_lifecycle() {
    MemoryManager* manager = memory_manager_create();
    Memory* memory = memory_create();
    
    // Test creation
    TodoziError* error = memory_manager_create_memory(manager, memory);
    assert(error == NULL);
    
    // Test retrieval
    Memory* retrieved = memory_manager_get_memory(manager, memory->id);
    assert(retrieved == memory);
    
    // Test deletion
    error = memory_manager_delete_memory(manager, memory->id);
    assert(error == NULL);
    
    memory_manager_destroy(manager);
}
```

### Test Categories

#### 1. Functional Tests
- Memory creation, retrieval, update, deletion
- Search functionality with various queries
- Statistical calculations accuracy
- Error handling scenarios

#### 2. Performance Tests
- Memory scalability (thousands of memories)
- Search performance with large datasets
- Memory allocation/deallocation patterns

#### 3. Integration Tests
- End-to-end memory lifecycle
- Parser integration with manager
- Statistical reporting accuracy

#### 4. Boundary Tests
- Empty memory manager operations
- Maximum capacity testing
- Invalid input handling

### Test Data Generation
```c
// Helper function for test memory creation
Memory* create_test_memory(const char* moment, MemoryImportance importance) {
    Memory* memory = memory_create();
    memory->moment = strdup(moment);
    memory->importance = importance;
    memory->term = MEMORY_TERM_SHORT;
    // ... set other fields
    return memory;
}
```

## Deployment Instructions

### Prerequisites
- C compiler (GCC recommended)
- UUID development library (`libuuid-devel` on Linux)
- Standard C library

### Build Process
```bash
# Compile with necessary flags
gcc -o todozi_memory.o -c todozi_memory.c -luuid

# Link with application
gcc -o my_app my_app.c todozi_memory.o -luuid
```

### Platform-Specific Notes

#### Linux
```bash
# Install UUID library
sudo apt-get install uuid-dev  # Debian/Ubuntu
sudo yum install libuuid-devel # CentOS/RHEL
```

#### macOS
```bash
# UUID library included in system
gcc -o todozi_memory todozi_memory.c
```

#### Windows
- Use Microsoft's GUID functions instead of libuuid
- Modify UUID generation code accordingly

### Integration with Applications
```c
#include "todozi_memory.h"

int main() {
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        fprintf(stderr, "Failed to initialize memory manager\n");
        return 1;
    }
    
    // Use memory manager functions...
    memory_manager_destroy(manager);
    return 0;
}
```

## Troubleshooting Guide

### Common Issues

#### 1. Memory Leaks
**Symptoms**: Increasing memory usage over time
**Diagnosis**: Use valgrind or similar memory debugger
```bash
valgrind --leak-check=full ./my_app
```
**Solutions**: Ensure all allocated memory is properly freed in error paths

#### 2. UUID Generation Failures
**Symptoms**: Memory creation fails with allocation errors
**Solution**: Check UUID library installation and linking

#### 3. Search Performance Issues
**Symptoms**: Slow response with large memory sets
**Solutions**:
- Implement query result caching
- Add search indexes for common fields
- Use pagination for large result sets

#### 4. Tag Management Problems
**Symptoms**: Tag statistics inaccurate or tags missing
**Diagnosis**: Check tag vector handling in update operations
**Solution**: Verify tag copying logic in update functions

### Error Codes and Handling

#### TodoziError Types
- `TODOZI_ERROR_VALIDATION`: Input validation failures
- `TODOZI_ERROR_OTHER`: General system errors

#### Error Recovery Strategy
```c
TodoziError* error = memory_manager_create_memory(manager, memory);
if (error) {
    fprintf(stderr, "Error %d: %s\n", error->type, error->message);
    todozi_error_free(error);
    // Recovery logic...
}
```

### Debugging Techniques

#### 1. Memory State Inspection
```c
void debug_memory_manager(MemoryManager* manager) {
    printf("Total memories: %zu\n", hashmap_size(manager->memories));
    printf("Memory tags entries: %zu\n", hashmap_size(manager->memory_tags));
    
    Vector* all_memories = memory_manager_get_all_memories(manager);
    printf("Vector size: %zu\n", vector_size(all_memories));
    vector_destroy(all_memories, NULL);
}
```

#### 2. Performance Profiling
```c
#include <time.h>

clock_t start = clock();
Vector* results = memory_manager_search_memories(manager, query);
clock_t end = clock();
printf("Search took %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);
```

## Usage Examples

### Basic Memory Management
```c
#include "todozi_memory.h"

int main() {
    // Initialize memory manager
    MemoryManager* manager = memory_manager_create();
    
    // Create a memory
    Memory* memory = memory_create();
    memory->moment = strdup("Meeting with client");
    memory->meaning = strdup("Important project discussion");
    memory->reason = strdup("Quarterly review preparation");
    memory->importance = MEMORY_IMPORTANCE_HIGH;
    memory->term = MEMORY_TERM_LONG;
    memory->memory_type = MEMORY_TYPE_STANDARD;
    memory->user_id = strdup("user_123");
    
    // Add tags
    string_vector_push(memory->tags, "meeting");
    string_vector_push(memory->tags, "client");
    string_vector_push(memory->tags, "important");
    
    // Store memory
    TodoziError* error = memory_manager_create_memory(manager, memory);
    if (error) {
        printf("Error: %s\n", error->message);
        todozi_error_free(error);
        memory_destroy(memory);
    } else {
        printf("Memory created with ID: %s\n", memory->id);
    }
    
    // Search for memories
    Vector* results = memory_manager_search_memories(manager, "client");
    printf("Found %zu memories matching 'client'\n", vector_size(results));
    
    // Get statistics
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    printf("Total memories: %zu\n", stats->total_memories);
    printf("Short term percentage: %.2f%%\n", 
           memory_statistics_short_term_percentage(stats));
    
    memory_statistics_destroy(stats);
    vector_destroy(results, NULL);
    memory_manager_destroy(manager);
    
    return 0;
}
```

### Advanced Usage: Memory Updates
```c
// Update an existing memory
MemoryUpdate* update = memory_update_create();
memory_update_moment(update, "Updated meeting details")
          ->memory_update_importance(update, MEMORY_IMPORTANCE_CRITICAL);

// Create new tags vector
Vector* new_tags = string_vector_create();
string_vector_push(new_tags, "urgent");
string_vector_push(new_tags, "followup");
memory_update_tags(update, new_tags);

TodoziError* update_error = memory_manager_update_memory(manager, memory_id, update);
if (update_error) {
    // Handle error
    memory_update_destroy(update); // Clean up on failure
} else {
    // update object is consumed by update function
}
```

### Text Parsing Example
```c
const char* memory_text = 
    "<memory>emotional;happy; Birthday celebration; Family gathering; "
    "Made me feel loved and appreciated; high; long; family,birthday,celebration</memory>";

Memory* parsed_memory;
TodoziError* parse_error = parse_memory_format(memory_text, "user_456", &parsed_memory);

if (!parse_error) {
    printf("Parsed emotional memory: %s\n", parsed_memory->emotion);
    memory_manager_create_memory(manager, parsed_memory);
} else {
    printf("Parse error: %s\n", parse_error->message);
    todozi_error_free(parse_error);
}
```

### Statistical Reporting
```c
void print_memory_report(MemoryManager* manager) {
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    
    printf("=== Memory Statistics Report ===\n");
    printf("Total Memories: %zu\n", stats->total_memories);
    printf("Short Term: %zu (%.1f%%)\n", stats->short_term_memories,
           memory_statistics_short_term_percentage(stats));
    printf("Long Term: %zu (%.1f%%)\n", stats->long_term_memories,
           memory_statistics_long_term_percentage(stats));
    printf("Critical Memories: %zu (%.1f%%)\n", stats->critical_memories,
           memory_statistics_critical_percentage(stats));
    printf("Unique Tags: %zu\n", stats->unique_tags);
    printf("Secret Memories: %zu\n", stats->secret_memories);
    printf("Human Memories: %zu\n", stats->human_memories);
    printf("Emotional Memories: %zu\n", stats->emotional_memories);
    printf("Standard Memories: %zu\n", stats->standard_memories);
    
    memory_statistics_destroy(stats);
    
    // Tag popularity analysis
    HashMap* tag_stats = memory_manager_get_tag_statistics(manager);
    // Iterate through tag statistics...
}
```

This comprehensive documentation provides complete coverage of the Todozi Memory Management System, including detailed API references, architectural insights, performance considerations, security guidelines, testing strategies, deployment instructions, and practical usage examples. The system demonstrates robust memory management capabilities with extensible architecture for various memory classification and analysis needs.