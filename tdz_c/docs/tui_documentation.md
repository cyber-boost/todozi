# TodoziApp - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Function Reference](#function-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

TodoziApp is a comprehensive task management application written in C. It provides a modular architecture for managing tasks with multiple statuses, priorities, and organization features. The application features a tab-based interface with filtering, sorting, and search capabilities.

### Key Features
- Multi-tab interface (Projects, Tasks, Done, Find, API, Feed, Bye)
- Advanced task filtering and sorting
- Real-time search functionality
- Task editing with validation
- Project-based organization
- API endpoint management
- Progress tracking and analytics

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Main Module   │───▶│   TodoziApp     │───▶│    Task Model   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  UI Rendering   │◄──▶│  Event Handler  │◄──▶│ Data Management │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Tab System     │    │ Filter Engine   │    │ Storage Layer   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Component Relationships
- **TodoziApp**: Main application controller
- **Task**: Core data structure for task management
- **TaskFilters**: Filter configuration for task views
- **EditSession**: Manages task editing operations
- **Renderer Modules**: Handle UI display for different tabs

## Data Structures

### Enum Definitions

#### Priority
```c
typedef enum {
    PRIORITY_CRITICAL,    // Highest priority
    PRIORITY_URGENT,      // Urgent tasks
    PRIORITY_HIGH,        // High importance
    PRIORITY_MEDIUM,      // Medium importance
    PRIORITY_LOW          // Low importance
} Priority;
```

#### Status
```c
typedef enum {
    STATUS_TODO,          // Task to be done
    STATUS_PENDING,       // Awaiting action
    STATUS_IN_PROGRESS,   // Currently working on
    STATUS_BLOCKED,       // Blocked by dependencies
    STATUS_REVIEW,        // Needs review
    STATUS_DONE,          // Completed
    STATUS_COMPLETED,     // Fully completed
    STATUS_CANCELLED,     // Cancelled task
    STATUS_DEFERRED       // Postponed to later
} Status;
```

#### Assignee
```c
typedef enum {
    ASSIGNEE_HUMAN,       // Assigned to human
    ASSIGNEE_AI,          // AI-assisted task
    ASSIGNEE_COLLABORATIVE // Collaborative effort
} Assignee;
```

### Struct Definitions

#### Task Structure
```c
struct Task {
    char* id;                    // Unique identifier
    char* user_id;               // User owner
    char* action;                // Task description
    char* time;                  // Time estimation
    Priority priority;           // Priority level
    Status status;               // Current status
    Assignee* assignee;          // Assignee type
    char* parent_project;        // Project association
    char** tags;                 // Categorization tags
    int tags_count;              // Number of tags
    char** dependencies;         // Task dependencies
    int dependencies_count;      // Dependency count
    char* context_notes;         // Additional context
    int* progress;               // Progress percentage
    double* embedding_vector;    // AI embedding for similarity
    int embedding_vector_size;   // Vector dimension
    time_t created_at;           // Creation timestamp
    time_t updated_at;           // Last update timestamp
};
```

#### TodoziApp Structure
```c
struct TodoziApp {
    // Navigation and State
    AppTab current_tab;
    int selected_task_index;
    int selected_project_index;
    int should_quit;
    
    // Data Storage
    Task* tasks;
    int tasks_count;
    Task* filtered_tasks;
    int filtered_tasks_count;
    Task* search_results;
    int search_results_count;
    char** projects;
    int projects_count;
    
    // Configuration
    TaskFilters task_filters;
    TaskSortBy done_sort_by;
    SortOrder done_sort_order;
    TaskFilters done_filters;
    
    // Editor System
    EditSession* editor;
    EditorField editor_field;
    char* editor_input;
    int editor_selected_field;
    
    // Analytics
    unsigned long* completion_data;
    int completion_data_size;
    unsigned long* priority_distribution;
    int priority_distribution_size;
    
    // System Status
    char* server_status;
    int server_running;
    
    // Extended counters
    int ideas_count, memories_count, feelings_count;
    int errors_count, training_data_count, queue_items_count;
    int reminders_count;
    
    // UI State
    int more_tab_section, more_tab_selected_index;
    int more_scroll_offset, feed_scroll_offset;
    int api_keys_count, api_selected_index;
    int api_endpoints_scroll, api_keys_scroll;
    int toast_notifications_count;
};
```

## Function Reference

### Core Application Functions

#### `todozi_app_new()`
**Purpose**: Initialize a new TodoziApp instance
**Parameters**: None
**Returns**: `TodoziApp*` - New application instance or NULL on failure
**Complexity**: O(1)
```c
TodoziApp* app = todozi_app_new();
if (app == NULL) {
    // Handle allocation failure
}
```

#### `todozi_app_free()`
**Purpose**: Clean up application resources
**Parameters**: `TodoziApp* app` - Application instance to free
**Returns**: void
**Complexity**: O(n) where n is number of tasks
```c
todozi_app_free(app);
```

#### `todozi_app_run()`
**Purpose**: Main application loop
**Parameters**: `TodoziApp* app` - Application instance
**Returns**: void
**Complexity**: O(n) per iteration
```c
todozi_app_run(app);
```

### Task Management Functions

#### `task_clone()`
**Purpose**: Create a deep copy of a task
**Parameters**: `Task* task` - Source task to clone
**Returns**: `Task` - Cloned task
**Complexity**: O(n + m) where n=tags, m=dependencies
```c
Task original = {0};
Task copy = task_clone(&original);
```

#### `task_free()`
**Purpose**: Free task resources
**Parameters**: `Task* task` - Task to free
**Returns**: void
**Complexity**: O(n + m) where n=tags, m=dependencies
```c
task_free(&task);
```

### Filtering and Sorting Functions

#### `todozi_app_apply_filters()`
**Purpose**: Apply current filters to task list
**Parameters**: `TodoziApp* app` - Application instance
**Returns**: void
**Complexity**: O(n) where n is number of tasks
```c
todozi_app_apply_filters(app);
```

#### `todozi_app_sort_done_tasks()`
**Purpose**: Sort completed tasks based on current configuration
**Parameters**: 
- `TodoziApp* app` - Application instance
- `Task** tasks` - Array of task pointers to sort
- `int count` - Number of tasks
**Returns**: void
**Complexity**: O(n²) - uses bubble sort
```c
Task** done_tasks;
int count;
todozi_app_sort_done_tasks(app, done_tasks, count);
```

### UI Rendering Functions

#### `todozi_app_draw()`
**Purpose**: Render complete application UI
**Parameters**: `TodoziApp* app` - Application instance
**Returns**: void
**Complexity**: O(n) where n is visible items
```c
todozi_app_draw(app);
```

#### `todozi_app_draw_tabs()`
**Purpose**: Render tab navigation header
**Parameters**: `TodoziApp* app` - Application instance
**Returns**: void
**Complexity**: O(1)
```c
todozi_app_draw_tabs(app);
```

### Helper Functions

#### `string_clone()`
**Purpose**: Create a duplicate string
**Parameters**: `const char* str` - String to clone
**Returns**: `char*` - New string or NULL on failure
**Complexity**: O(n) where n is string length
```c
char* copy = string_clone("original");
```

#### `string_array_free()`
**Purpose**: Free array of strings
**Parameters**: 
- `char** array` - String array to free
- `int count` - Number of strings
**Returns**: void
**Complexity**: O(n) where n is array size
```c
string_array_free(strings, count);
```

## Usage Examples

### Basic Application Setup
```c
#include "todozi.h"

int main() {
    // Initialize application
    TodoziApp* app = todozi_app_new();
    if (!app) {
        fprintf(stderr, "Failed to initialize TodoziApp\n");
        return 1;
    }
    
    // Run main loop
    todozi_app_run(app);
    
    // Cleanup
    todozi_app_free(app);
    return 0;
}
```

### Task Creation and Management
```c
// Create a sample task
Task sample_task = {0};
sample_task.id = string_clone("task_001");
sample_task.action = string_clone("Implement new feature");
sample_task.priority = PRIORITY_HIGH;
sample_task.status = STATUS_TODO;
sample_task.created_at = time(NULL);

// Clone the task
Task cloned_task = task_clone(&sample_task);

// Free resources when done
task_free(&sample_task);
task_free(&cloned_task);
```

### Filter Configuration
```c
// Configure task filters
app->task_filters.priority_filter = malloc(sizeof(Priority) * 2);
app->task_filters.priority_filter[0] = PRIORITY_HIGH;
app->task_filters.priority_filter[1] = PRIORITY_CRITICAL;

app->task_filters.status_filter = malloc(sizeof(Status) * 3);
app->task_filters.status_filter[0] = STATUS_TODO;
app->task_filters.status_filter[1] = STATUS_IN_PROGRESS;
app->task_filters.status_filter[2] = STATUS_REVIEW;

// Apply filters
todozi_app_apply_filters(app);
```

### Custom Rendering Example
```c
void custom_render_function(TodoziApp* app) {
    printf("Custom Task View\n");
    printf("================\n");
    
    for (int i = 0; i < app->filtered_tasks_count; i++) {
        Task* task = &app->filtered_tasks[i];
        printf("%d. %s [Priority: %d, Status: %d]\n", 
               i + 1, task->action, task->priority, task->status);
    }
}
```

## Design Patterns

### 1. Model-View-Controller (MVC)
- **Model**: `Task` structure and data management functions
- **View**: `todozi_app_draw_*` functions for UI rendering
- **Controller**: `TodoziApp` structure and event handling functions

### 2. Singleton Pattern
- Single `TodoziApp` instance manages entire application state
- Centralized resource management and cleanup

### 3. Observer Pattern
- Filter system observes task changes and updates views accordingly
- Search results update dynamically based on query changes

### 4. Strategy Pattern
- Multiple sorting strategies (`TASK_SORT_BY_*`) for different views
- Configurable filter strategies for task organization

### 5. Factory Pattern
- `todozi_app_new()` acts as factory for application instances
- `task_clone()` provides controlled object creation

## Performance Analysis

### Time Complexity
| Operation | Best Case | Average Case | Worst Case |
|-----------|-----------|--------------|------------|
| App Initialization | O(1) | O(1) | O(1) |
| Task Filtering | O(n) | O(n) | O(n) |
| Task Sorting | O(n log n) | O(n²)* | O(n²) |
| Search | O(n) | O(n) | O(n) |
| UI Rendering | O(k) | O(k) | O(k) |

*Note: Current implementation uses bubble sort O(n²), could be optimized

### Space Complexity
| Component | Memory Usage |
|-----------|--------------|
| TodoziApp Base | O(1) |
| Task Storage | O(n × m) where n=tasks, m=avg fields |
| Filter Results | O(k) where k=filtered tasks |
| UI State | O(1) |

### Optimization Recommendations
1. **Replace bubble sort** with quicksort or mergesort (O(n log n))
2. **Implement pagination** for large task lists
3. **Add caching** for frequent filter operations
4. **Use more efficient string handling** (string interning)
5. **Implement lazy loading** for task details

## Security Considerations

### Memory Safety
```c
// Always check malloc results
char* clone = malloc(strlen(str) + 1);
if (clone == NULL) {
    // Handle allocation failure
    return NULL;
}

// Proper string copying with bounds checking
strncpy(clone, str, strlen(str) + 1);
```

### Input Validation
```c
// Validate task field inputs
int validate_task_input(const Task* task) {
    if (task == NULL) return 0;
    if (task->action == NULL || strlen(task->action) == 0) return 0;
    if (task->priority < PRIORITY_CRITICAL || task->priority > PRIORITY_LOW) return 0;
    // Additional validation checks...
    return 1;
}
```

### Resource Management
- Always free allocated memory using corresponding free functions
- Use `calloc` for sensitive data to avoid information leakage
- Implement proper error handling for allocation failures

### API Security
- Validate all API inputs before processing
- Implement rate limiting for search operations
- Sanitize search queries to prevent injection attacks

## Testing Strategies

### Unit Testing Framework
```c
// Example test case for task cloning
void test_task_clone() {
    Task original = {0};
    original.id = string_clone("test_id");
    original.action = string_clone("Test action");
    original.priority = PRIORITY_MEDIUM;
    
    Task cloned = task_clone(&original);
    
    // Assertions
    assert(strcmp(original.id, cloned.id) == 0);
    assert(strcmp(original.action, cloned.action) == 0);
    assert(original.priority == cloned.priority);
    assert(original.id != cloned.id); // Should be different pointers
    
    // Cleanup
    task_free(&original);
    task_free(&cloned);
}
```

### Integration Testing
```c
void test_app_workflow() {
    TodoziApp* app = todozi_app_new();
    assert(app != NULL);
    
    // Test tab navigation
    AppTab initial_tab = app->current_tab;
    todozi_app_next_tab(app);
    assert(app->current_tab != initial_tab);
    
    // Test filter application
    todozi_app_apply_filters(app);
    assert(app->filtered_tasks_count >= 0);
    
    todozi_app_free(app);
}
```

### Test Categories
1. **Unit Tests**: Individual function testing
2. **Integration Tests**: Component interaction testing
3. **Performance Tests**: Load and stress testing
4. **Security Tests**: Input validation and boundary testing
5. **UI Tests**: Rendering and interaction testing

### Test Coverage Goals
- 90%+ line coverage for core functions
- 100% error path testing
- Memory leak detection in all tests
- Performance regression testing

## Deployment Instructions

### Build Configuration
```makefile
# Makefile example
CC = gcc
CFLAGS = -Wall -Wextra -Werror -std=c99 -O2
SOURCES = todozi.c
TARGET = todozi_app

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES)

clean:
	rm -f $(TARGET)

.PHONY: clean
```

### Compilation Options
```bash
# Debug build with sanitizers
gcc -g -fsanitize=address -fsanitize=undefined -o todozi_debug todozi.c

# Release build
gcc -O3 -DNDEBUG -o todozi_release todozi.c

# Static analysis
scan-build gcc -o todozi todozi.c
```

### Platform-Specific Considerations

#### Linux
```bash
# Install dependencies (if any)
sudo apt-get install build-essential

# Build and run
make
./todozi_app
```

#### Windows (MinGW)
```cmd
# Using MinGW
gcc -o todozi_app.exe todozi.c
todozi_app.exe
```

#### macOS
```bash
# Install Xcode command line tools
xcode-select --install

# Build and run
make
./todozi_app
```

### Docker Deployment
```dockerfile
FROM alpine:latest

# Install build dependencies
RUN apk add --no-cache gcc musl-dev

# Copy source code
COPY todozi.c /app/
WORKDIR /app

# Compile application
RUN gcc -static -O3 -o todozi_app todozi.c

# Run application
CMD ["./todozi_app"]
```

## Troubleshooting Guide

### Common Issues and Solutions

#### Memory Leaks
**Problem**: Application memory usage grows over time
**Solution**: Use valgrind to detect leaks
```bash
valgrind --leak-check=full ./todozi_app
```

#### Segmentation Faults
**Problem**: Application crashes with segfault
**Solution**: Enable core dumps and debug symbols
```bash
ulimit -c unlimited
gcc -g -o todozi_app todozi.c
gdb todozi_app core
```

#### Performance Issues
**Problem**: Slow response with large task lists
**Solution**: Implement pagination and optimize sorting
```c
// Replace bubble sort with more efficient algorithm
void optimized_sort(Task** tasks, int count, TaskSortBy sort_by) {
    // Implement quicksort or mergesort
}
```

### Debugging Techniques

#### Logging System
```c
#define DEBUG 1

void debug_log(const char* format, ...) {
    #ifdef DEBUG
    va_list args;
    va_start(args, format);
    vprintf(format, args);
    va_end(args);
    #endif
}
```

#### Runtime Assertions
```c
#include <assert.h>

void todozi_app_apply_filters(TodoziApp* app) {
    assert(app != NULL);
    assert(app->tasks_count >= 0);
    // ... implementation
}
```

### Error Recovery Strategies

#### Graceful Degradation
```c
TodoziApp* todozi_app_new() {
    TodoziApp* app = calloc(1, sizeof(TodoziApp));
    if (app == NULL) {
        fprintf(stderr, "Critical: Failed to allocate application memory\n");
        return NULL;
    }
    
    // Initialize with safe defaults if allocations fail
    app->search_query = string_clone("");
    if (app->search_query == NULL) {
        app->search_query = string_clone("default");
    }
    
    return app;
}
```

#### Resource Cleanup on Failure
```c
void initialize_component(TodoziApp* app) {
    char* resource1 = malloc(100);
    if (resource1 == NULL) goto cleanup;
    
    char* resource2 = malloc(100);
    if (resource2 == NULL) goto cleanup;
    
    // ... more initialization
    
    return;
    
cleanup:
    free(resource1);
    free(resource2);
    // Handle error
}
```

### Monitoring and Diagnostics

#### Performance Metrics
```c
typedef struct {
    time_t startup_time;
    unsigned long tasks_processed;
    unsigned long memory_used;
    double average_render_time;
} AppMetrics;

void collect_metrics(TodoziApp* app, AppMetrics* metrics) {
    // Collect runtime metrics for monitoring
}
```

This documentation provides a comprehensive reference for understanding, using, and maintaining the TodoziApp codebase. The modular architecture and clear separation of concerns make it suitable for extension and customization while maintaining stability and performance.