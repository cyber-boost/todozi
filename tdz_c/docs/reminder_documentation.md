# Todozi Reminder System - Complete Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Core Components](#core-components)
5. [API Reference](#api-reference)
6. [Usage Examples](#usage-examples)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategy](#testing-strategy)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive reminder management system written in C that provides robust functionality for creating, managing, and tracking reminders with advanced features like tagging, searching, and statistical analysis.

### Key Features
- **UUID-based identification** for unique reminder tracking
- **Priority and status management** with multiple levels
- **Tag-based organization** and search capabilities
- **Statistical analysis** of reminder data
- **Advanced search functionality** across content and tags
- **Date/time management** with overdue and due-soon detection
- **Builder pattern** for complex updates

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│  ReminderManager  │  ReminderStatistics  │  Parser Utils    │
├─────────────────────────────────────────────────────────────┤
│                    Business Logic Layer                     │
├─────────────────────────────────────────────────────────────┤
│   Reminder        │  ReminderUpdate      │  DateTime        │
├─────────────────────────────────────────────────────────────┤
│                    Data Structure Layer                     │
├─────────────────────────────────────────────────────────────┤
│    HashMap        │       Vector         │   String Utils   │
├─────────────────────────────────────────────────────────────┤
│                    System Layer                             │
├─────────────────────────────────────────────────────────────┤
│   Standard C Library        │        UUID Library           │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships
```
ReminderManager ──┬── HashMap<reminder_id, Reminder>
                 └── HashMap<reminder_id, Vector<tags>>
                      │
                      ├── Reminder ──┬── DateTime (created_at, updated_at, remind_at)
                      │              ├── Vector<tags>
                      │              ├── Priority (enum)
                      │              └── Status (enum)
                      │
                      └── ReminderUpdate (Builder Pattern)
```

## Data Structures

### Enumerations

#### ReminderPriority
```c
typedef enum {
    REMINDER_PRIORITY_LOW,      // Low priority reminder
    REMINDER_PRIORITY_MEDIUM,   // Medium priority (default)
    REMINDER_PRIORITY_HIGH      // High priority reminder
} ReminderPriority;
```

#### ReminderStatus
```c
typedef enum {
    REMINDER_STATUS_PENDING,    // Created but not yet active
    REMINDER_STATUS_ACTIVE,     // Currently active reminder
    REMINDER_STATUS_COMPLETED,  // Successfully completed
    REMINDER_STATUS_CANCELLED   // Manually cancelled
} ReminderStatus;
```

### Core Structures

#### HashNode
```c
typedef struct HashNode {
    char* key;              // String key for the hash entry
    void* value;            // Pointer to the stored value
    struct HashNode* next;  // Next node in collision chain
} HashNode;
```

#### HashMap
```c
typedef struct {
    HashNode** buckets;     // Array of bucket pointers
    size_t size;            // Current number of entries
    size_t capacity;        // Total capacity of the hash table
} HashMap;
```

#### Vector
```c
typedef struct {
    void** data;            // Array of void pointers
    size_t size;            // Current number of elements
    size_t capacity;        // Total capacity of the vector
} Vector;
```

#### DateTime
```c
typedef struct {
    time_t timestamp;       // UNIX timestamp representation
} DateTime;
```

#### Reminder
```c
typedef struct Reminder {
    char* id;               // UUID string identifier
    char* content;          // Reminder text content
    DateTime remind_at;     // Scheduled reminder time
    ReminderPriority priority; // Priority level
    ReminderStatus status;  // Current status
    Vector* tags;           // Vector of tag strings
    DateTime created_at;    // Creation timestamp
    DateTime updated_at;    // Last update timestamp
} Reminder;
```

#### ReminderUpdate (Builder Pattern)
```c
typedef struct ReminderUpdate {
    char* content;          // New content (optional)
    DateTime* remind_at;    // New reminder time (optional)
    ReminderPriority* priority; // New priority (optional)
    ReminderStatus* status; // New status (optional)
    Vector* tags;           // New tags vector (optional)
} ReminderUpdate;
```

#### ReminderStatistics
```c
typedef struct {
    size_t total_reminders;     // Total number of reminders
    size_t pending_reminders;   // Reminders in pending state
    size_t active_reminders;    // Currently active reminders
    size_t overdue_reminders;   // Overdue reminders
    size_t unique_tags;         // Number of unique tags used
} ReminderStatistics;
```

## Core Components

### HashMap Implementation

#### hashmap_create()
```c
HashMap* hashmap_create(size_t capacity);
```
**Parameters:**
- `capacity`: Initial capacity of the hash table

**Returns:**
- Pointer to newly allocated HashMap, or NULL on failure

**Complexity:** O(1)

#### hashmap_destroy()
```c
void hashmap_destroy(HashMap* map, 
                    void (*key_destructor)(void*), 
                    void (*value_destructor)(void*));
```
**Parameters:**
- `map`: HashMap to destroy
- `key_destructor`: Function to destroy key objects
- `value_destructor`: Function to destroy value objects

**Complexity:** O(n)

#### hashmap_put()
```c
int hashmap_put(HashMap* map, char* key, void* value);
```
**Parameters:**
- `map`: Target HashMap
- `key`: Key string (ownership transferred)
- `value`: Value to store

**Returns:**
- 1 on success, 0 on failure

**Complexity:** O(1) average, O(n) worst-case

#### hashmap_get()
```c
void* hashmap_get(HashMap* map, const char* key);
```
**Parameters:**
- `map`: Source HashMap
- `key`: Key to search for

**Returns:**
- Pointer to value, or NULL if not found

**Complexity:** O(1) average, O(n) worst-case

### Vector Implementation

#### vector_create()
```c
Vector* vector_create(void);
```
**Returns:**
- New Vector with initial capacity of 10

**Complexity:** O(1)

#### vector_push()
```c
int vector_push(Vector* vec, void* item);
```
**Parameters:**
- `vec`: Target Vector
- `item`: Item to add

**Returns:**
- 1 on success, 0 on failure

**Complexity:** O(1) amortized

#### vector_sort()
```c
void vector_sort(Vector* vec, int (*comparator)(const void*, const void*));
```
**Parameters:**
- `vec`: Vector to sort
- `comparator`: Comparison function

**Complexity:** O(n log n)

### DateTime Functions

#### datetime_now()
```c
DateTime datetime_now(void);
```
**Returns:**
- DateTime structure with current timestamp

**Complexity:** O(1)

#### datetime_add_days()
```c
DateTime datetime_add_days(DateTime dt, int days);
```
**Parameters:**
- `dt`: Base DateTime
- `days`: Number of days to add

**Returns:**
- New DateTime with added days

**Complexity:** O(1)

### Reminder Management

#### reminder_create()
```c
Reminder* reminder_create(void);
```
**Returns:**
- New Reminder with default values:
  - Priority: MEDIUM
  - Status: PENDING
  - Created/Updated: Current time

**Complexity:** O(1)

#### reminder_manager_create()
```c
ReminderManager* reminder_manager_create(void);
```
**Returns:**
- New ReminderManager with initialized data structures

**Complexity:** O(1)

#### reminder_manager_create_reminder()
```c
char* reminder_manager_create_reminder(ReminderManager* manager, Reminder* reminder);
```
**Parameters:**
- `manager`: ReminderManager instance
- `reminder`: Reminder to add (ownership transferred)

**Returns:**
- Copy of the generated UUID, or NULL on failure

**Complexity:** O(n) where n is number of tags

## API Reference

### Complete Function Documentation

#### HashMap Functions

**hashmap_hash()**
```c
size_t hashmap_hash(const char* key, size_t capacity);
```
*Computes DJB2 hash for the given key*

**Parameters:**
- `key`: String to hash
- `capacity`: Hash table capacity for modulo operation

**Returns:**
- Hash index between 0 and capacity-1

**hashmap_remove()**
```c
void* hashmap_remove(HashMap* map, const char* key);
```
*Removes and returns value for given key*

**Parameters:**
- `map`: HashMap to remove from
- `key`: Key to remove

**Returns:**
- Removed value, or NULL if not found

**hashmap_values()**
```c
Vector* hashmap_values(HashMap* map);
```
*Returns all values in the hashmap as a Vector*

**Parameters:**
- `map`: Source HashMap

**Returns:**
- Vector containing all values

#### Vector Functions

**vector_destroy()**
```c
void vector_destroy(Vector* vec, void (*destructor)(void*));
```
*Destroys vector and optionally its elements*

**Parameters:**
- `vec`: Vector to destroy
- `destructor`: Function to destroy each element

**vector_get()**
```c
void* vector_get(Vector* vec, size_t index);
```
*Returns element at specified index*

**Parameters:**
- `vec`: Source Vector
- `index`: Zero-based index

**Returns:**
- Element pointer, or NULL if index out of bounds

**vector_filter()**
```c
Vector* vector_filter(Vector* vec, int (*predicate)(void*));
```
*Creates new vector with elements satisfying predicate*

**Parameters:**
- `vec`: Source Vector
- `predicate`: Filter function returning non-zero for inclusion

**Returns:**
- New Vector with filtered elements

#### String Utilities

**string_duplicate()**
```c
char* string_duplicate(const char* str);
```
*Creates heap-allocated copy of string*

**Parameters:**
- `str`: Source string

**Returns:**
- New string copy, or NULL on failure

**string_to_lowercase()**
```c
char* string_to_lowercase(const char* str);
```
*Creates lowercase version of string*

**Parameters:**
- `str`: Source string

**Returns:**
- Lowercase string copy

**string_split()**
```c
Vector* string_split(const char* str, char delimiter);
```
*Splits string by delimiter into vector of strings*

**Parameters:**
- `str`: String to split
- `delimiter`: Character delimiter

**Returns:**
- Vector of string tokens

#### Reminder Operations

**reminder_mark_completed()**
```c
void reminder_mark_completed(Reminder* reminder);
```
*Marks reminder as completed and updates timestamp*

**Parameters:**
- `reminder`: Reminder to update

**reminder_manager_search_reminders()**
```c
Vector* reminder_manager_search_reminders(ReminderManager* manager, const char* query);
```
*Searches reminders by content and tags (case-insensitive)*

**Parameters:**
- `manager`: ReminderManager instance
- `query`: Search query string

**Returns:**
- Vector of matching reminders

**reminder_manager_get_overdue_reminders()**
```c
Vector* reminder_manager_get_overdue_reminders(ReminderManager* manager);
```
*Returns reminders that are overdue (past their remind_at time)*

**Parameters:**
- `manager`: ReminderManager instance

**Returns:**
- Vector of overdue reminders

#### Statistical Functions

**reminder_manager_get_reminder_statistics()**
```c
ReminderStatistics reminder_manager_get_reminder_statistics(ReminderManager* manager);
```
*Computes comprehensive statistics about reminders*

**Parameters:**
- `manager`: ReminderManager instance

**Returns:**
- ReminderStatistics structure

**reminder_manager_get_tag_statistics()**
```c
HashMap* reminder_manager_get_tag_statistics(ReminderManager* manager);
```
*Returns hashmap of tag usage counts*

**Parameters:**
- `manager`: ReminderManager instance

**Returns:**
- HashMap<tag_string, count_int*>

## Usage Examples

### Basic Usage

```c
#include "todozi.h"

int main() {
    // Create reminder manager
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Failed to create manager\n");
        return 1;
    }
    
    // Create a reminder
    Reminder* reminder = reminder_create();
    if (!reminder) {
        printf("Failed to create reminder\n");
        reminder_manager_destroy(manager);
        return 1;
    }
    
    reminder->content = string_duplicate("Buy groceries");
    reminder->priority = REMINDER_PRIORITY_HIGH;
    
    // Add tags
    vector_push(reminder->tags, string_duplicate("shopping"));
    vector_push(reminder->tags, string_duplicate("urgent"));
    
    // Add reminder to manager
    char* reminder_id = reminder_manager_create_reminder(manager, reminder);
    if (!reminder_id) {
        printf("Failed to add reminder\n");
        reminder_destroy(reminder);
        reminder_manager_destroy(manager);
        return 1;
    }
    
    printf("Created reminder with ID: %s\n", reminder_id);
    free(reminder_id);
    
    // Cleanup
    reminder_manager_destroy(manager);
    return 0;
}
```

### Advanced Usage with Updates

```c
// Update a reminder using builder pattern
ReminderUpdate* update = reminder_update_create();
reminder_update_content(update, "Buy groceries and cook dinner");
reminder_update_priority(update, REMINDER_PRIORITY_MEDIUM);

Vector* new_tags = vector_create();
vector_push(new_tags, string_duplicate("shopping"));
vector_push(new_tags, string_duplicate("cooking"));
vector_push(new_tags, string_duplicate("weekend"));
reminder_update_tags(update, new_tags);

// Apply update
if (reminder_manager_update_reminder(manager, reminder_id, update)) {
    printf("Reminder updated successfully\n");
} else {
    printf("Failed to update reminder\n");
}

reminder_update_destroy(update); // Only destroys the update struct, not transferred data
```

### Search and Statistics

```c
// Search for reminders
Vector* results = reminder_manager_search_reminders(manager, "grocery");
if (results) {
    printf("Found %zu reminders\n", results->size);
    for (size_t i = 0; i < results->size; i++) {
        Reminder* rem = (Reminder*)vector_get(results, i);
        printf(" - %s (Priority: %d)\n", rem->content, rem->priority);
    }
    vector_destroy(results, NULL);
}

// Get statistics
ReminderStatistics stats = reminder_manager_get_reminder_statistics(manager);
printf("Total reminders: %zu\n", stats.total_reminders);
printf("Pending: %zu (%.1f%%)\n", stats.pending_reminders, 
       reminder_statistics_pending_percentage(&stats));
printf("Overdue: %zu\n", stats.overdue_reminders);

// Get tag statistics
HashMap* tag_stats = reminder_manager_get_tag_statistics(manager);
if (tag_stats) {
    Vector* all_tags = hashmap_values(tag_stats);
    for (size_t i = 0; i < all_tags->size; i++) {
        // Process tag statistics...
    }
    reminder_manager_free_tag_statistics(tag_stats);
}
```

## Design Patterns

### Builder Pattern
The `ReminderUpdate` structure implements the Builder pattern, allowing flexible construction of update operations:

```c
// Fluent interface for building updates
ReminderUpdate* update = reminder_update_create();
reminder_update_content(update, "New content")
    ->reminder_update_priority(update, REMINDER_PRIORITY_HIGH)
    ->reminder_update_status(update, REMINDER_STATUS_ACTIVE);
```

### Factory Pattern
`reminder_create()` and `reminder_manager_create()` act as factory functions, encapsulating object creation logic.

### Observer Pattern (Implicit)
The statistics functions observe the state of reminders without modifying them, providing insights into the system.

### Strategy Pattern
The `vector_sort()` function uses a comparator strategy, allowing different sorting algorithms to be applied.

## Performance Analysis

### Time Complexity

| Operation | Average Case | Worst Case | Notes |
|-----------|-------------|------------|--------|
| Reminder Creation | O(n) | O(n) | n = number of tags |
| Reminder Lookup | O(1) | O(n) | HashMap lookup |
| Reminder Update | O(1) | O(n) | Depends on tag count |
| Reminder Deletion | O(1) | O(n) | HashMap operations |
| Search (by content) | O(n) | O(n) | Linear scan through all reminders |
| Statistics Generation | O(n) | O(n) | Processes all reminders |

### Space Complexity

| Component | Space Usage | Notes |
|-----------|-------------|--------|
| Reminder | O(m) | m = content length + tag count |
| HashMap | O(n) | n = number of reminders |
| Vector | O(k) | k = number of elements |

### Memory Management
- **Allocations**: Extensive use of `malloc()` for dynamic structures
- **Ownership**: Clear ownership transfer in API functions
- **Cleanup**: Comprehensive destructor functions provided

### Optimization Opportunities
1. **Caching**: Implement LRU cache for frequently accessed reminders
2. **Indexing**: Add secondary indexes for common search patterns
3. **Pagination**: Implement pagination for large result sets
4. **Batching**: Add batch operations for bulk updates

## Security Considerations

### Input Validation
```c
// Always validate inputs before processing
if (!manager || !reminder_id) return 0;
if (strlen(reminder_id) != 36) return 0; // UUID validation
```

### Memory Safety
- **Bounds Checking**: Vector operations include bounds validation
- **Null Checks**: Extensive null pointer checking throughout
- **Ownership Management**: Clear documentation of ownership transfer

### UUID Security
- Uses cryptographically secure `uuid_generate_random()`
- UUIDs are unique across system instances
- No sensitive information encoded in IDs

### String Handling
- **Buffer Overflow Protection**: Proper string length calculations
- **Case Conversion**: Safe lowercase conversion without locale dependencies
- **Tokenization**: Robust string splitting with memory safety

## Testing Strategy

### Unit Testing Framework
The code includes built-in test functions demonstrating comprehensive testing:

### Test Categories

#### 1. Data Structure Tests
```c
void test_hashmap_operations() {
    HashMap* map = hashmap_create(10);
    // Test put, get, remove, size operations
    // Verify collision handling
    // Test destruction with various destructors
}
```

#### 2. Reminder Lifecycle Tests
```c
void test_reminder_lifecycle() {
    // Test creation with various parameters
    // Test status transitions
    // Test update operations
    // Test destruction and memory cleanup
}
```

#### 3. Manager Integration Tests
```c
void test_manager_integration() {
    // Test reminder creation and retrieval
    // Test search functionality
    // Test statistical calculations
    // Test error conditions
}
```

#### 4. Parser Tests
```c
void test_parser_edge_cases() {
    // Test malformed input
    // Test boundary conditions
    // Test encoding issues
    // Test memory allocation failures
}
```

### Test Execution
```bash
# Compile with test flags
gcc -std=c99 -D_POSIX_C_SOURCE=200809L -D_GNU_SOURCE todozi.c -luuid -o todozi_test

# Run tests
./todozi_test
```

### Coverage Goals
- **Line Coverage**: >90% for core functions
- **Branch Coverage**: >85% for decision points
- **Error Paths**: All error conditions tested
- **Memory Tests**: Valgrind checks for leaks

## Deployment Instructions

### Prerequisites
```bash
# Ubuntu/Debian
sudo apt-get install libuuid1 libuuid-dev build-essential

# CentOS/RHEL
sudo yum install libuuid libuuid-devel gcc

# macOS
brew install ossp-uuid
```

### Compilation Options
```makefile
# Basic compilation
CC = gcc
CFLAGS = -std=c99 -Wall -Wextra -D_POSIX_C_SOURCE=200809L
LIBS = -luuid

todozi: todozi.c
	$(CC) $(CFLAGS) -o todozi todozi.c $(LIBS)

# Debug build
todozi_debug: todozi.c
	$(CC) $(CFLAGS) -g -DDEBUG -o todozi_debug todozi.c $(LIBS)

# Release build
todozi_release: todozi.c
	$(CC) $(CFLAGS) -O2 -DNDEBUG -o todozi_release todozi.c $(LIBS)
```

### Integration with Applications

#### As a Library
```c
// todozi.h
#ifndef TODOZI_H
#define TODOZI_H

// Include all public headers
#include "reminder.h"
#include "manager.h"
#include "statistics.h"

#endif
```

#### As a Standalone Service
```c
// server.c - Example HTTP server integration
#include "todozi.h"

void handle_create_reminder(Request* req, Response* res) {
    Reminder* reminder = parse_reminder_from_json(req->body);
    char* id = reminder_manager_create_reminder(global_manager, reminder);
    send_json_response(res, 201, {"id": id});
}
```

### Deployment Checklist
1. [ ] Verify UUID library installation
2. [ ] Compile with appropriate flags
3. [ ] Run unit tests
4. [ ] Perform memory leak testing
5. [ ] Validate on target platform
6. [ ] Document API for consumers

## Troubleshooting Guide

### Common Issues

#### 1. Compilation Errors
**Problem**: "uuid/uuid.h: No such file or directory"
**Solution**: Install UUID development package
```bash
sudo apt-get install libuuid-dev  # Ubuntu
sudo yum install libuuid-devel    # CentOS
```

#### 2. Memory Leaks
**Problem**: Application memory usage grows over time
**Solution**: Use Valgrind to identify leaks
```bash
valgrind --leak-check=full ./todozi_app
```

#### 3. Performance Issues
**Problem**: Slow search operations with large datasets
**Solution**: Implement indexing or use smaller hashmap capacities
```c
// Use smaller initial capacity for better memory usage
HashMap* map = hashmap_create(50); // Instead of 100
```

### Debugging Techniques

#### Memory Debugging
```c
#ifdef DEBUG
#define DEBUG_PRINT(...) printf(__VA_ARGS__)
#else
#define DEBUG_PRINT(...)
#endif

void* debug_malloc(size_t size, const char* file, int line) {
    void* ptr = malloc(size);
    DEBUG_PRINT("Allocated %zu bytes at %s:%d\n", size, file, line);
    return ptr;
}
```

#### Error Tracking
```c
typedef struct {
    int error_code;
    char* message;
    const char* function;
    int line;
} TodoziError;

#define RETURN_ERROR(code, msg) \
    do { \
        TodoziError err = {code, msg, __func__, __LINE__}; \
        log_error(&err); \
        return code; \
    } while(0)
```

### Recovery Strategies

#### 1. Graceful Degradation
```c
Vector* reminder_manager_search_reminders(ReminderManager* manager, const char* query) {
    if (!manager || !query) {
        // Return empty vector instead of NULL for graceful degradation
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    // ... normal processing
}
```

#### 2. Data Corruption Recovery
```c
int validate_reminder_integrity(Reminder* reminder) {
    if (!reminder) return 0;
    if (!reminder->id || strlen(reminder->id) != 36) return 0;
    if (!reminder->content) return 0;
    // Additional validation checks
    return 1;
}
```

### Performance Monitoring

#### Benchmarking Setup
```c
#include <time.h>

void benchmark_operation() {
    clock_t start = clock();
    // Operation to benchmark
    clock_t end = clock();
    double time_spent = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Operation took: %f seconds\n", time_spent);
}
```

This comprehensive documentation provides complete coverage of the Todozi reminder system, including architectural insights, detailed API references, practical examples, and operational guidance for successful deployment and maintenance.