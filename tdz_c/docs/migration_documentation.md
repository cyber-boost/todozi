# Comprehensive Documentation: Todozi Task Migration System

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

The Todozi Task Migration System is a C library designed to migrate tasks from a legacy flat storage system to a project-based hierarchical storage system. It handles task grouping, validation, and cleanup operations with comprehensive error handling and reporting.

### Key Features
- **Dry Run Mode**: Preview migrations without making changes
- **Project-based Organization**: Group tasks by project
- **Verbose Logging**: Detailed progress reporting
- **Validation**: Post-migration integrity checks
- **Legacy Cleanup**: Automatic cleanup of migrated collections
- **Error Handling**: Comprehensive error tracking and reporting

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   Legacy Tasks  │    │  Task Migrator   │    │ Project Storage  │
│                 │    │                  │    │                  │
│ • active        │───▶│ • Grouping       │───▶│ • project1.json  │
│ • completed     │    │ • Migration      │    │ • project2.json  │
│ • archived      │    │ • Validation     │    │ • projectN.json  │
└─────────────────┘    └──────────────────┘    └──────────────────┘
         │                        │                        │
         └────────────────────────┼────────────────────────┘
                                  ▼
                       ┌──────────────────┐
                       │ Migration Report │
                       │ • Statistics     │
                       │ • Errors         │
                       └──────────────────┘
```

### Component Relationships
```
MigrationCli ───▶ TaskMigrator ───▶ MigrationReport
     │                  │                  │
     │                  │                  │
     ▼                  ▼                  ▼
   CLI Layer      Business Logic     Reporting Layer
                    │       │
                    ▼       ▼
             StorageOps   Validation
```

## Data Structures

### Core Structures

#### Task
```c
struct Task {
    char* id;                    // Unique task identifier
    char* parent_project;        // Parent project name
    char* status;                // Task status (active/completed/etc.)
    double* embedding_vector;    // AI embedding vector
    int vector_size;             // Size of embedding vector
    Task* next;                  // Linked list pointer
};
```

#### ProjectTaskContainer
```c
struct ProjectTaskContainer {
    char* project_name;          // Name of the project
    Task* tasks;                 // Linked list of tasks
    int task_count;              // Number of tasks in project
};
```

#### MigrationReport
```c
struct MigrationReport {
    int tasks_found;             // Total tasks discovered
    int tasks_migrated;          // Successfully migrated tasks
    int projects_migrated;       // Number of projects processed
    ProjectMigrationStats* project_stats; // Per-project statistics
    char** errors;               // Array of error messages
    int error_count;             // Number of errors encountered
};
```

#### HashMap
```c
typedef struct {
    char** keys;                 // Array of project names
    Task** task_lists;           // Array of task lists
    int* task_counts;            // Array of task counts
    int size;                    // Current number of entries
    int capacity;                // Maximum capacity
} HashMap;
```

### Error Handling

#### Result Type
```c
typedef struct {
    TodoziError error_type;      // Error code
    char* message;               // Human-readable error message
} Result;
```

#### Error Codes
```c
typedef enum {
    TODOZI_SUCCESS = 0,          // Operation completed successfully
    TODOZI_ERROR_STORAGE,        // Storage-related error
    TODOZI_ERROR_EMBEDDING,      // Embedding generation error
    TODOZI_ERROR_FILE            // File I/O error
} TodoziError;
```

## Core Components

### TaskMigrator

The main migration engine responsible for coordinating the entire migration process.

#### Responsibilities
- Load legacy tasks from collections
- Group tasks by project
- Migrate tasks to project containers
- Generate migration reports
- Validate migration integrity
- Clean up legacy collections

#### Configuration Flags
- `dry_run`: Preview mode (no changes made)
- `verbose`: Detailed logging output
- `force_overwrite`: Override existing projects

### MigrationCli

Command-line interface wrapper providing a fluent API for migration configuration.

#### Builder Pattern Methods
- `migration_cli_with_dry_run()`: Enable/disable dry run mode
- `migration_cli_with_verbose()`: Enable/disable verbose output
- `migration_cli_with_force()`: Enable/disable force overwrite

### HashMap Implementation

Simple hash map for grouping tasks by project name.

#### Characteristics
- Fixed capacity (no dynamic resizing)
- Linear search for key lookup
- Task list storage using linked lists
- Memory-efficient for small to medium datasets

## API Reference

### Core Migration Functions

#### `task_migrator_migrate()`
```c
Result task_migrator_migrate(TaskMigrator* self, MigrationReport* report);
```
**Description**: Main migration function that orchestrates the entire process.

**Parameters**:
- `self`: TaskMigrator instance
- `report`: MigrationReport structure to populate with results

**Returns**: Result indicating success or failure

**Process Flow**:
1. Load legacy tasks from all collections
2. Group tasks by project using HashMap
3. Migrate each project's tasks
4. Generate comprehensive report

#### `task_migrator_load_legacy_tasks()`
```c
Result task_migrator_load_legacy_tasks(TaskMigrator* self, 
                                      MigrationReport* report, 
                                      Task** all_tasks, 
                                      int* task_count);
```
**Description**: Loads tasks from legacy storage collections.

**Parameters**:
- `self`: TaskMigrator instance
- `report`: MigrationReport for statistics
- `all_tasks`: Output parameter for loaded tasks
- `task_count`: Output parameter for task count

**Collections Processed**:
- "active": Currently active tasks
- "completed": Completed tasks  
- "archived": Archived tasks

#### `task_migrator_group_tasks_by_project()`
```c
HashMap* task_migrator_group_tasks_by_project(TaskMigrator* self, 
                                             Task* tasks, 
                                             int task_count);
```
**Description**: Groups tasks by project name using a HashMap.

**Parameters**:
- `self`: TaskMigrator instance
- `tasks`: Linked list of tasks to group
- `task_count`: Number of tasks to process

**Returns**: HashMap containing project-grouped tasks

**Special Handling**: Tasks without projects are assigned to "general" project

### Project Migration Functions

#### `task_migrator_migrate_project_tasks()`
```c
Result task_migrator_migrate_project_tasks(TaskMigrator* self,
                                          const char* project_name,
                                          Task* tasks,
                                          int task_count,
                                          ProjectMigrationStats* stats);
```
**Description**: Migrates tasks for a specific project.

**Parameters**:
- `self`: TaskMigrator instance
- `project_name`: Name of the project to migrate
- `tasks`: Tasks belonging to the project
- `task_count`: Number of tasks to migrate
- `stats`: Statistics structure to populate

**Process**:
1. Check for existing project container
2. Handle force overwrite logic
3. Clone and migrate individual tasks
4. Update statistics

### Validation and Cleanup

#### `task_migrator_validate_migration()`
```c
Result task_migrator_validate_migration(TaskMigrator* self, bool* is_valid);
```
**Description**: Validates migration integrity by comparing legacy and new systems.

**Validation Logic**: 
```c
*is_valid = (legacy_tasks == 0) || (legacy_tasks > 0 && project_tasks >= legacy_tasks);
```

#### `task_migrator_cleanup_legacy()`
```c
Result task_migrator_cleanup_legacy(TaskMigrator* self);
```
**Description**: Removes empty legacy collections after successful migration.

**Safety Measures**:
- Only removes collections with 0 tasks
- Respects dry-run mode
- Comprehensive error reporting

### Helper Functions

#### Memory Management
```c
Task* task_clone(Task* task);                    // Deep copy of task
void task_free(Task* task);                      // Free task memory
void task_list_free(Task* tasks);                // Free linked list
void migration_report_free(MigrationReport* report); // Free report
```

#### HashMap Operations
```c
HashMap* hashmap_new(int capacity);              // Create new HashMap
void hashmap_free(HashMap* map);                 // Free HashMap memory
void hashmap_put(HashMap* map, const char* key, Task* task); // Add entry
int hashmap_size(HashMap* map);                  // Get number of entries
```

## Design Patterns

### 1. Builder Pattern
```c
// Fluent interface for configuration
MigrationCli* cli = migration_cli_new();
migration_cli_with_dry_run(cli, true)
    ->migration_cli_with_verbose(cli, true)
    ->migration_cli_with_force(cli, false);
```

### 2. Command Pattern
The `MigrationCli` acts as a command object that encapsulates the migration operation with its parameters.

### 3. Iterator Pattern
Task linked lists are traversed using iterative patterns:
```c
Task* current = tasks;
while (current) {
    // Process current task
    current = current->next;
}
```

### 4. Strategy Pattern
Different behaviors based on configuration flags (dry-run vs actual migration).

### 5. Factory Pattern
Helper functions like `task_clone()` and `project_task_container_new()` act as factories.

## Performance Analysis

### Time Complexity
| Operation | Complexity | Description |
|-----------|------------|-------------|
| Task loading | O(n) | Linear scan of collections |
| Grouping | O(n×m) | n tasks × m projects (worst case) |
| Migration | O(n) | Linear processing of tasks |
| Validation | O(p) | p projects to verify |

### Space Complexity
| Component | Complexity | Description |
|-----------|------------|-------------|
| Task storage | O(n) | n tasks in memory |
| HashMap | O(m) | m project groups |
| Migration report | O(p + e) | p projects + e errors |

### Memory Usage Optimizations
- **Linked lists**: Efficient for variable-sized collections
- **String pooling**: Duplicate project names are shared
- **Lazy loading**: Tasks loaded on-demand per collection

### Performance Tips
1. **Batch operations**: Process projects sequentially to minimize memory
2. **Early termination**: Stop on critical errors
3. **Memory reuse**: Clone only necessary task data

## Security Considerations

### Input Validation
```c
// All public functions validate parameters
if (!self || !project_name || !stats) {
    return result_err(TODOZI_ERROR_STORAGE, "Invalid parameters");
}
```

### Memory Safety
- **Bounds checking**: HashMap prevents buffer overflow
- **Null pointer checks**: Comprehensive validation
- **Resource cleanup**: All allocated memory properly freed

### File System Security
- **Path validation**: Sanitize file paths before operations
- **Permission checks**: Verify write permissions before saving
- **Safe defaults**: Use `/tmp` fallback for storage directory

### Data Integrity
- **Duplicate detection**: Prevent task duplication during migration
- **Validation checks**: Post-migration integrity verification
- **Error recovery**: Continue migration after non-critical errors

## Testing Strategy

### Unit Testing Framework
```c
void test_task_migrator_creation(void) {
    TaskMigrator* migrator = task_migrator_new();
    if (migrator && !migrator->dry_run && !migrator->verbose && !migrator->force_overwrite) {
        printf("✅ test_task_migrator_creation passed\n");
    } else {
        printf("❌ test_task_migrator_creation failed\n");
    }
    free(migrator);
}
```

### Test Categories

#### 1. Component Tests
- Task creation and cloning
- HashMap operations
- Memory management

#### 2. Integration Tests
- End-to-end migration scenarios
- Error handling paths
- Configuration combinations

#### 3. System Tests
- Large dataset performance
- File system operations
- Recovery scenarios

### Test Data Strategies
- **Empty collections**: Verify graceful handling
- **Large tasks**: Test memory management
- **Duplicate projects**: Test conflict resolution
- **Corrupted data**: Test error recovery

## Deployment Instructions

### Build Requirements
```bash
# Required libraries
gcc (C11 standard)
make
valgrind (for memory testing)
```

### Compilation
```bash
# Basic compilation
gcc -std=c11 -Wall -Wextra -pedantic -o todozi_migrator todozi_migration.c

# Debug build with sanitizers
gcc -std=c11 -g -fsanitize=address -fsanitize=undefined -o todozi_migrator_debug todozi_migration.c

# Release build with optimizations
gcc -std=c11 -O2 -DNDEBUG -o todozi_migrator_release todozi_migration.c
```

### Installation Steps

1. **Clone repository**
   ```bash
   git clone https://github.com/todozi/migration-tool.git
   cd migration-tool
   ```

2. **Build the tool**
   ```bash
   make all
   ```

3. **Run tests**
   ```bash
   make test
   ```

4. **Install system-wide**
   ```bash
   sudo make install
   ```

### Configuration

#### Environment Variables
```bash
export TODOZI_STORAGE_DIR="$HOME/.todozi"  # Custom storage directory
export TODOZI_VERBOSE=1                    # Enable verbose logging
```

#### Configuration File
Create `~/.todozi/config`:
```json
{
    "dry_run": false,
    "verbose": true,
    "force_overwrite": false,
    "backup_before_migration": true
}
```

## Troubleshooting Guide

### Common Issues

#### 1. Memory Allocation Failures
**Symptom**: Program crashes with segmentation fault
**Solution**: Check available memory, reduce batch size

#### 2. File Permission Errors  
**Symptom**: "Permission denied" errors during save operations
**Solution**: Verify write permissions in storage directory

#### 3. Migration Validation Failures
**Symptom**: Validation reports mismatch between legacy and new systems
**Solution**: Run with `--verbose` to identify specific issues

### Debugging Techniques

#### Verbose Mode
```c
MigrationCli* cli = migration_cli_new();
migration_cli_with_verbose(cli, true);
```

#### Dry Run Analysis
```c
// Preview migration without changes
migration_cli_with_dry_run(cli, true);
```

#### Memory Debugging
```bash
valgrind --leak-check=full ./todozi_migrator
```

### Error Recovery

#### Partial Migration Recovery
1. **Check migration report** for details
2. **Manual verification** of problematic projects
3. **Incremental re-migration** of failed tasks

#### Data Corruption Handling
1. **Backup creation** before migration
2. **Validation checks** at each step
3. **Rollback capability** for critical failures

## Usage Examples

### Basic Migration
```c
#include "todozi_migration.h"

int main() {
    MigrationCli* cli = migration_cli_new();
    Result result = migration_cli_run(cli);
    
    if (result.error_type == TODOZI_SUCCESS) {
        printf("Migration completed successfully!\n");
    } else {
        printf("Migration failed: %s\n", result.message);
    }
    
    result_free(&result);
    migration_cli_free(cli);
    return 0;
}
```

### Dry Run with Verbose Output
```c
MigrationCli* cli = migration_cli_new();
migration_cli_with_dry_run(cli, true);
migration_cli_with_verbose(cli, true);

Result result = migration_cli_run(cli);
// Preview migration without making changes
```

### Force Overwrite Existing Projects
```c
MigrationCli* cli = migration_cli_new();
migration_cli_with_force(cli, true);

Result result = migration_cli_run(cli);
// Overwrites existing project containers
```

### Custom Storage Directory
```c
// Set custom storage directory before migration
setenv("TODOZI_STORAGE_DIR", "/custom/path", 1);

MigrationCli* cli = migration_cli_new();
Result result = migration_cli_run(cli);
```

### Integration with Larger System
```c
void migrate_todozi_data() {
    MigrationReport report;
    TaskMigrator* migrator = task_migrator_new();
    
    // Configure migrator
    task_migrator_verbose(migrator, true);
    
    // Perform migration
    Result result = task_migrator_migrate(migrator, &report);
    
    if (result.error_type == TODOZI_SUCCESS) {
        // Process migration report
        printf("Migrated %d tasks across %d projects\n",
               report.tasks_migrated, report.projects_migrated);
        
        // Validate migration
        bool is_valid;
        task_migrator_validate_migration(migrator, &is_valid);
        
        if (is_valid) {
            // Clean up legacy data
            task_migrator_cleanup_legacy(migrator);
        }
    }
    
    // Cleanup
    migration_report_free(&report);
    free(migrator);
    result_free(&result);
}
```

This comprehensive documentation provides complete coverage of the Todozi Task Migration System, including architectural details, API references, security considerations, and practical usage examples. The system is designed for reliability, performance, and ease of integration into larger todo management applications.