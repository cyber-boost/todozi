# TODOZI Idea Management System - Comprehensive Documentation

## Table of Contents
1. [System Overview](#system-overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Core Functions](#core-functions)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategy](#testing-strategy)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## System Overview

TODOZI is a C-based idea management system that provides comprehensive functionality for storing, organizing, and analyzing ideas. The system supports tagging, categorization by importance and sharing levels, search capabilities, and statistical analysis.

### Key Features
- **Idea Management**: Create, read, update, and delete ideas
- **Tagging System**: Flexible tagging with automatic deduplication
- **Search Functionality**: Case-insensitive search across idea content, tags, and context
- **Sharing Levels**: Public, team, and private sharing controls
- **Importance Classification**: Low, medium, high, and breakthrough levels
- **Statistics Generation**: Comprehensive analytics and reporting
- **Parsing Support**: Structured idea format parsing

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   IdeaManager   │◄───│   IdeaVector    │◄───│     Idea        │
│                 │    │                 │    │                 │
│ - ideas HashMap │    │ - Idea* array   │    │ - id, idea text │
│ - tags HashMap  │    │ - size/capacity │    │ - metadata      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ StringHashMap   │    │  StringVector   │    │  IdeaUpdate     │
│                 │    │                 │    │                 │
│ - buckets array │    │ - string array  │    │ - update fields │
│ - hash function │    │ - size/capacity │    │ - builder API   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Component Relationships
- **IdeaManager**: Central controller managing all ideas and tags
- **Idea**: Core data structure representing individual ideas
- **IdeaUpdate**: Builder pattern for partial idea updates
- **StringHashMap**: Generic hash map implementation for key-value storage
- **StringVector**: Dynamic array implementation for string collections

## Data Structures

### Enum Definitions

#### ShareLevel
```c
typedef enum {
    SHARE_LEVEL_PUBLIC,    // Idea is publicly visible
    SHARE_LEVEL_TEAM,      // Idea visible to team members only
    SHARE_LEVEL_PRIVATE    // Idea private to creator only
} ShareLevel;
```

#### IdeaImportance
```c
typedef enum {
    IDEA_IMPORTANCE_LOW,         // Low priority idea
    IDEA_IMPORTANCE_MEDIUM,      // Medium priority idea
    IDEA_IMPORTANCE_HIGH,        // High priority idea
    IDEA_IMPORTANCE_BREAKTHROUGH // Breakthrough/strategic idea
} IdeaImportance;
```

#### ItemStatus
```c
typedef enum {
    ITEM_STATUS_ACTIVE,    // Idea is active and visible
    ITEM_STATUS_COMPLETED, // Idea has been implemented
    ITEM_STATUS_ARCHIVED   // Idea is archived/hidden
} ItemStatus;
```

### Core Structures

#### Idea Structure
```c
struct Idea {
    char* id;               // UUID identifier (36 bytes + null)
    char* idea;             // Main idea text content
    char* project_id;       // Optional project association
    ItemStatus status;      // Current status (active/completed/archived)
    ShareLevel share;       // Sharing visibility level
    IdeaImportance importance; // Priority classification
    StringVector* tags;     // Dynamic array of tags
    char* context;          // Additional context/notes
    time_t created_at;      // Creation timestamp
    time_t updated_at;      // Last modification timestamp
};
```

#### IdeaManager Structure
```c
struct IdeaManager {
    StringHashMap* ideas;    // Main ideas storage (key: UUID, value: Idea*)
    StringHashMap* idea_tags; // Tag associations (key: UUID, value: StringVector*)
};
```

#### Error Handling
```c
typedef enum {
    TODOZI_ERROR_VALIDATION, // Input validation errors
    TODOZI_ERROR_NOT_FOUND   // Resource not found errors
} TodoziErrorType;

typedef struct {
    TodoziErrorType type;   // Error category
    char* message;          // Human-readable error message
} TodoziError;
```

## Core Functions

### Memory Management Functions

#### string_vector_new()
```c
/**
 * Creates a new StringVector with initial capacity
 * 
 * @return Pointer to allocated StringVector, NULL on allocation failure
 * @memory Allocates 4 * sizeof(char*) + sizeof(StringVector) bytes initially
 * @complexity O(1)
 */
StringVector* string_vector_new();
```

#### string_vector_push()
```c
/**
 * Appends a string to the vector with automatic resizing
 * 
 * @param vec StringVector to modify
 * @param str String to append (will be stored as-is, no copying)
 * @memory May reallocate if capacity is exceeded (doubling strategy)
 * @complexity Amortized O(1), worst-case O(n) during resize
 */
void string_vector_push(StringVector* vec, char* str);
```

#### string_hashmap_new()
```c
/**
 * Creates a new StringHashMap with specified value destructor
 * 
 * @param value_free Function pointer for value cleanup, NULL for no cleanup
 * @return Pointer to allocated StringHashMap, NULL on failure
 * @memory Allocates 16 buckets + structure overhead
 * @complexity O(1)
 */
StringHashMap* string_hashmap_new(void (*value_free)(void*));
```

### Idea Management Functions

#### idea_manager_new()
```c
/**
 * Creates a new IdeaManager instance with initialized storage
 * 
 * @return Pointer to allocated IdeaManager, NULL on allocation failure
 * @memory Allocates two hash maps with 16 buckets each
 * @complexity O(1)
 * @error Returns NULL on memory allocation failure
 */
IdeaManager* idea_manager_new();
```

#### idea_manager_create_idea()
```c
/**
 * Adds a new idea to the manager with automatic UUID generation
 * 
 * @param manager IdeaManager instance
 * @param idea Pre-configured Idea structure
 * @return Duplicate of generated UUID string, NULL on failure
 * @memory Allocates UUID string and tag vector clone
 * @complexity O(n) where n is number of tags
 * @error Returns NULL on UUID generation or memory allocation failure
 */
char* idea_manager_create_idea(IdeaManager* manager, Idea* idea);
```

#### idea_manager_search_ideas()
```c
/**
 * Searches ideas by content, tags, or context (case-insensitive)
 * 
 * @param manager IdeaManager instance
 * @param query Search query string
 * @return Vector of matching Idea pointers, NULL on error
 * @memory Allocates result vector and temporary lowercase strings
 * @complexity O(n*m) where n is ideas count, m is query length
 * @error Returns NULL on memory allocation failure
 */
IdeaVector* idea_manager_search_ideas(IdeaManager* manager, const char* query);
```

### Statistical Functions

#### idea_manager_get_idea_statistics()
```c
/**
 * Generates comprehensive statistics about stored ideas
 * 
 * @param manager IdeaManager instance
 * @return Pointer to IdeaStatistics structure, NULL on error
 * @memory Allocates statistics structure and temporary vectors
 * @complexity O(n) where n is number of ideas
 * @error Returns NULL on memory allocation failure
 */
IdeaStatistics* idea_manager_get_idea_statistics(IdeaManager* manager);
```

## Usage Examples

### Basic Idea Management
```c
#include "todozi.h"

int main() {
    // Initialize manager
    IdeaManager* manager = idea_manager_new();
    if (!manager) {
        fprintf(stderr, "Failed to initialize idea manager\n");
        return 1;
    }

    // Create a new idea
    Idea* idea = idea_new();
    idea->idea = strdup("Implement AI-powered search");
    idea->share = SHARE_LEVEL_TEAM;
    idea->importance = IDEA_IMPORTANCE_HIGH;
    
    // Add tags
    string_vector_push(idea->tags, strdup("ai"));
    string_vector_push(idea->tags, strdup("search"));
    string_vector_push(idea->tags, strdup("enhancement"));

    // Store the idea
    char* idea_id = idea_manager_create_idea(manager, idea);
    if (!idea_id) {
        fprintf(stderr, "Failed to create idea\n");
        idea_free(idea);
        idea_manager_free(manager);
        return 1;
    }

    printf("Created idea with ID: %s\n", idea_id);

    // Search for ideas
    IdeaVector* results = idea_manager_search_ideas(manager, "AI");
    if (results) {
        printf("Found %zu matching ideas\n", idea_vector_size(results));
        idea_vector_free(results);
    }

    // Cleanup
    free(idea_id);
    idea_manager_free(manager);
    return 0;
}
```

### Advanced Usage with Updates
```c
// Update an existing idea
IdeaUpdate* update = idea_update_new();
idea_update_idea(update, "Enhanced AI search with ML");
idea_update_importance(update, IDEA_IMPORTANCE_BREAKTHROUGH);

TodoziError* error = idea_manager_update_idea(manager, idea_id, update);
if (error) {
    fprintf(stderr, "Update failed: %s\n", error->message);
    todozi_error_free(error);
}

idea_update_free(update);
```

### Statistics Generation
```c
// Get comprehensive statistics
IdeaStatistics* stats = idea_manager_get_idea_statistics(manager);
if (stats) {
    printf("Total ideas: %zu\n", stats->total_ideas);
    printf("Public ideas: %.1f%%\n", idea_statistics_public_percentage(stats));
    printf("Breakthrough ideas: %.1f%%\n", idea_statistics_breakthrough_percentage(stats));
    idea_statistics_free(stats);
}
```

## Design Patterns

### 1. Builder Pattern
**Implementation**: `IdeaUpdate` structure with fluent interface
```c
// Fluent builder API
IdeaUpdate* update = idea_update_new()
    ->idea_update_idea(update, "New content")
    ->idea_update_share(update, SHARE_LEVEL_PUBLIC)
    ->idea_update_importance(update, IDEA_IMPORTANCE_HIGH);
```

### 2. Factory Pattern
**Implementation**: `idea_new()`, `idea_manager_new()` functions
- Creates complex objects with proper initialization
- Ensures consistent object state

### 3. Strategy Pattern
**Implementation**: Custom comparison functions for sorting
```c
// Configurable sorting strategy
int idea_compare_by_created_at_desc(const void* a, const void* b);
```

### 4. Iterator Pattern
**Implementation**: Vector and hash map traversal functions
```c
// Iterate through all ideas
IdeaVector* all_ideas = idea_manager_get_all_ideas(manager);
for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
    Idea* idea = idea_vector_get(all_ideas, i);
    // Process idea
}
```

## Performance Analysis

### Time Complexity Analysis

| Operation | Average Case | Worst Case | Notes |
|-----------|--------------|------------|-------|
| Idea Creation | O(n) | O(n) | n = number of tags |
| Idea Retrieval | O(1) | O(k) | k = bucket chain length |
| Idea Search | O(n*m) | O(n*m) | n = ideas, m = query length |
| Tag Statistics | O(n*t) | O(n*t) | n = ideas, t = tags per idea |
| Recent Ideas | O(n log n) | O(n log n) | Due to sorting |

### Space Complexity
- **Idea Storage**: O(n) where n is number of ideas
- **Tag Index**: O(n*t) where t is average tags per idea
- **Search Operations**: O(k) temporary storage for results

### Memory Usage Patterns
```c
// Memory allocation sizes:
- Idea structure: ~64 bytes + string contents
- StringVector: 16 bytes + 4*sizeof(char*) initially
- StringHashMap: 16 buckets * 8 bytes + structure overhead
```

### Optimization Strategies
1. **Hash Map Sizing**: Initial 16 buckets, consider tuning for expected load
2. **Vector Resizing**: Doubling strategy provides amortized O(1) push operations
3. **String Handling**: Avoid unnecessary copies, use reference counting if needed

## Security Considerations

### Input Validation
```c
// All public functions include null checks
if (!manager || !idea_id) {
    return error_response(TODOZI_ERROR_VALIDATION, "Invalid parameters");
}
```

### Memory Safety
- **String Handling**: Always use `strdup()` for user input
- **Buffer Management**: No fixed-size buffers vulnerable to overflow
- **Resource Cleanup**: Comprehensive free functions prevent leaks

### UUID Security
- Uses standard `uuid_generate()` for unique identifiers
- No predictable sequences for idea IDs

### Access Control
- Share level enforcement through filtering functions
- No authentication/authorization built-in (application-level concern)

## Testing Strategy

### Unit Testing Framework
```c
// Test structure example
void test_functionality() {
    // Setup
    IdeaManager* manager = idea_manager_new();
    assert(manager != NULL);
    
    // Exercise
    Idea* idea = create_test_idea();
    char* id = idea_manager_create_idea(manager, idea);
    
    // Verify
    assert(id != NULL);
    Idea* retrieved = idea_manager_get_idea(manager, id);
    assert(retrieved != NULL);
    assert(strcmp(retrieved->idea, idea->idea) == 0);
    
    // Teardown
    free(id);
    idea_manager_free(manager);
}
```

### Test Categories

#### 1. Core Functionality Tests
- Idea creation, retrieval, update, deletion
- Search functionality with various queries
- Tag management and statistics

#### 2. Boundary Tests
- Empty manager operations
- Maximum capacity testing (stress tests)
- Invalid input handling

#### 3. Integration Tests
- End-to-end workflow validation
- Memory leak detection
- Performance benchmarking

### Test Coverage Goals
- 100% function coverage
- 90% branch coverage
- Memory leak detection in all tests

## Deployment Instructions

### Build Requirements
```bash
# Required libraries
sudo apt-get install libuuid1 libuuid-dev  # UUID support
gcc -std=c99 -D_POSIX_C_SOURCE=200809L     # Compiler flags
```

### Compilation Instructions
```makefile
# Makefile example
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -D_POSIX_C_SOURCE=200809L
LIBS = -luuid

todozi: todozi.c
	$(CC) $(CFLAGS) -o todozi todozi.c $(LIBS)

test: todozi
	./todozi

clean:
	rm -f todozi
```

### Platform Support
- **Linux**: Fully supported (primary platform)
- **macOS**: Supported with Homebrew uuid installation
- **Windows**: Requires MinGW-w64 and external UUID library

### Integration Guidelines
```c
// Example integration with existing C application
#include "todozi.h"

// Initialize on application start
IdeaManager* idea_mgr = idea_manager_new();

// Use throughout application lifecycle
// ...

// Cleanup on application exit
idea_manager_free(idea_mgr);
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. Memory Allocation Failures
**Symptom**: Functions return NULL unexpectedly
**Solution**: Check system memory, implement graceful degradation
```c
IdeaManager* manager = idea_manager_new();
if (!manager) {
    // Implement fallback strategy or graceful shutdown
    log_error("Failed to allocate idea manager");
    return ERROR_MEMORY;
}
```

#### 2. UUID Generation Failures
**Symptom**: `generate_uuid()` returns NULL
**Solution**: Check uuid library installation
```bash
# Verify UUID library
ldconfig -p | grep libuuid
# Reinstall if missing
sudo apt-get install --reinstall libuuid1 libuuid-dev
```

#### 3. Search Performance Issues
**Symptom**: Slow search operations with large datasets
**Solution**: Implement indexing or limit search scope
```c
// Add search limits for large datasets
IdeaVector* results = idea_manager_search_ideas(manager, query);
if (idea_vector_size(results) > 1000) {
    // Implement pagination or results limiting
}
```

### Debugging Techniques

#### Memory Leak Detection
```bash
# Using valgrind
valgrind --leak-check=full ./todozi

# Compile with debug symbols
gcc -g -o todozi_debug todozi.c -luuid
```

#### Performance Profiling
```bash
# Using gprof
gcc -pg -o todozi_prof todozi.c -luuid
./todozi_prof
gprof todozi_prof gmon.out > analysis.txt
```

### Error Code Reference

| Error Type | Common Causes | Resolution |
|------------|---------------|------------|
| TODOZI_ERROR_VALIDATION | Null parameters, invalid format | Check input validation |
| TODOZI_ERROR_NOT_FOUND | Invalid UUID, deleted idea | Verify idea existence |
| Memory Allocation | System out of memory | Reduce load, check for leaks |

### Recovery Procedures

#### Application Crash Recovery
```c
// Implement periodic backup system
void backup_ideas(IdeaManager* manager) {
    // Serialize ideas to file periodically
    // Implement recovery from backup on startup
}
```

#### Data Corruption Handling
```c
// Add integrity checks
int verify_idea_integrity(Idea* idea) {
    return idea && idea->id && idea->idea; // Basic checks
}
```

This comprehensive documentation provides complete coverage of the TODOZI system. The implementation demonstrates robust C programming practices with attention to memory management, error handling, and extensible architecture.