# Todozi C Library Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [File Structure](#file-structure)
5. [Error Handling](#error-handling)
6. [API Reference](#api-reference)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Usage Examples](#usage-examples)
13. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive task management system written in C that provides sophisticated organization, AI integration, and multi-agent task assignment capabilities. The system manages projects, tasks, agents, and various data types through a structured file-based storage system.

### Key Features
- Multi-project task management
- AI agent system with specialized roles
- Configurable storage and backup
- Semantic search capabilities
- Agent-task assignment system
- Memory and idea tracking
- Error and feeling logging

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                      TODOZI SYSTEM                          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   STORAGE   │  │   CONFIG    │  │    AGENTS   │         │
│  │   MANAGER   │  │   MANAGER   │  │    SYSTEM   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   TASK      │  │   PROJECT   │  │   QUEUE     │         │
│  │   MANAGER   │  │   MANAGER   │  │   MANAGER   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┤
│  │                   FILE SYSTEM LAYER                    │
│  └─────────────────────────────────────────────────────────┤
└─────────────────────────────────────────────────────────────┘
```

### Data Flow
```
User/Agent → API Layer → Business Logic → Storage Layer → File System
```

## Data Structures

### Core Structures

#### PathBuf
```c
struct PathBuf {
    char* path;  // Dynamically allocated path string
};
```
**Purpose**: Wrapper for file system paths with automatic memory management.

#### Config
```c
struct Config {
    RegistrationInfo* registration;  // User registration details
    char* version;                  // Application version
    char* default_project;          // Default project name
    int auto_backup;                // Auto-backup enabled flag
    char* backup_interval;          // Backup frequency
    int ai_enabled;                 // AI integration flag
    char* default_assignee;         // Default task assignee
    char* date_format;              // Date formatting string
    char* timezone;                 // Timezone setting
};
```

#### RegistrationInfo
```c
struct RegistrationInfo {
    char* user_name;      // User's display name
    char* user_email;     // User's email address
    char* api_key;        // API authentication key
    char* user_id;        // Unique user identifier
    char* fingerprint;    // System fingerprint
    time_t registered_at; // Registration timestamp
    char* server_url;     // Server endpoint URL
};
```

#### Task
```c
struct Task {
    char* id;               // Unique task identifier
    char* action;           // Task description/action
    char* status;           // Current status (active/completed/etc.)
    char* priority;         // Priority level
    char* parent_project;   // Associated project
    time_t created_at;      // Creation timestamp
    time_t updated_at;      // Last update timestamp
    char* context_notes;    // Additional context information
    float* embedding_vector; // Semantic embedding vector
    int embedding_size;     // Size of embedding vector
};
```

#### Agent
```c
struct Agent {
    char* id;             // Unique agent identifier
    char* name;           // Display name
    char* description;    // Agent description
    char* system_prompt;  // AI system prompt
};
```

### Specialized Structures

#### ProjectTaskContainer
Manages project-task relationships with hashed project names for efficient lookup.

#### SemanticSearchResult
Contains task information with similarity score for semantic search operations.

#### AgentAssignment
Links agents to specific tasks for assignment tracking.

## File Structure

### Directory Hierarchy
```
~/.todozi/
├── tdz.hlx                     # Main configuration file
├── tasks/                      # Task storage
│   ├── active.json
│   ├── completed.json
│   └── archived.json
├── projects/                   # Project definitions
├── agents/                     # Agent configurations
├── memories/                   # System memories
├── ideas/                      # Idea storage
├── training/                   # Training data
├── chunks/                     # Code chunks
├── errors/                     # Error logs
├── assignments/                # Agent-task assignments
├── feelings/                   # System feeling logs
├── queue/                      # Processing queue
├── templates/                  # Template storage
├── backups/                    # Backup files
├── api/                        # API-related data
├── models/                     # AI model data
├── responses/                  # AI response storage
├── embed/                      # Embedding data
└── steps/                      # Step-by-step process data
```

## Error Handling

### Error Codes
```c
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_STORAGE,
    TODOZI_ERROR_PROJECT_NOT_FOUND,
    TODOZI_ERROR_TASK_NOT_FOUND,
    TODOZI_ERROR_VALIDATION
} TodoziError;
```

### Error Structure
```c
struct Error {
    char* id;        // Unique error identifier
    char* message;   // Human-readable error message
};
```

## API Reference

### Storage Management Functions

#### `init_storage()`
**Purpose**: Initialize the complete Todozi storage structure.
**Returns**: `TodoziError` status code.
**Complexity**: O(n) where n is number of directories to create.

```c
int init_storage();
```

#### `check_folder_structure()`
**Purpose**: Validate that all required directories exist.
**Returns**: 1 if valid, 0 if invalid.
**Complexity**: O(n) directory checks.

```c
int check_folder_structure();
```

### Path Management Functions

#### `get_storage_dir()`
**Purpose**: Get the base storage directory path.
**Returns**: `PathBuf*` containing the storage path.
**Memory**: Caller must free with `free_path_buf()`.

```c
PathBuf* get_storage_dir();
```

#### `join_paths()`
**Purpose**: Safely join two path components.
**Parameters**: 
- `const char* base`: Base path
- `const char* append`: Path component to append
**Returns**: Newly allocated path string.
**Memory**: Caller must free returned string.

```c
static char* join_paths(const char* base, const char* append);
```

### Configuration Functions

#### `save_config()`
**Purpose**: Save configuration to file.
**Parameters**: `Config* config`: Configuration to save.
**Returns**: `TodoziError` status.

```c
int save_config(Config* config);
```

#### `load_config()`
**Purpose**: Load configuration from file.
**Returns**: `Config*` or NULL on error.
**Memory**: Caller must free with `free_config()`.

```c
Config* load_config();
```

### Project Management

#### `save_project()`
**Purpose**: Save project definition.
**Parameters**: `Project* project`: Project to save.
**Returns**: `TodoziError` status.

```c
int save_project(Project* project);
```

#### `load_project()`
**Purpose**: Load project by name.
**Parameters**: `const char* project_name`: Project identifier.
**Returns**: `Project*` or NULL if not found.

```c
Project* load_project(const char* project_name);
```

### Task Management

#### `save_task()`
**Purpose**: Save individual task.
**Parameters**: `Task* task`: Task to save.
**Returns**: `TodoziError` status.

```c
int save_task(Task* task);
```

#### `load_task()`
**Purpose**: Load task by ID.
**Parameters**: `const char* task_id`: Task identifier.
**Returns**: `Task*` or NULL if not found.

```c
Task* load_task(const char* task_id);
```

### Agent System

#### Agent Creation Functions
Specialized functions for creating different agent types:
- `create_planner_agent()`
- `create_tester_agent()`
- `create_designer_agent()`
- `create_devops_agent()`
- `create_friend_agent()`
- `create_detective_agent()`
- `create_architect_agent()`
- `create_skeleton_agent()`
- `create_mason_agent()`
- `create_framer_agent()`
- `create_finisher_agent()`
- `create_investigator_agent()`
- `create_recycler_agent()`
- `create_tuner_agent()`
- `create_writer_agent()`
- `create_comrad_agent()`
- `create_nerd_agent()`
- `create_party_agent()`
- `create_nun_agent()`
- `create_hoarder_agent()`
- `create_snitch_agent()`
- `create_overlord_agent()`

#### `create_default_agents()`
**Purpose**: Create all default agent configurations.
**Returns**: `TodoziError` status.

```c
int create_default_agents();
```

### Memory Management Functions

Each data structure has corresponding free function:
- `free_path_buf()`
- `free_config()`
- `free_project()`
- `free_task()`
- `free_agent()`
- etc.

## Design Patterns

### 1. Factory Pattern
**Implementation**: Agent creation functions act as factories for different agent types.

### 2. Repository Pattern
**Implementation**: Each data type has dedicated save/load functions that abstract storage details.

### 3. Singleton Pattern
**Implementation**: Configuration is loaded once and shared across the system.

### 4. Strategy Pattern
**Implementation**: Different agents implement different strategies for task handling.

### 5. Observer Pattern
**Implementation**: Queue system observes task changes and triggers agent assignments.

## Performance Analysis

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Directory creation | O(n) | n = number of directories |
| File operations | O(1) | Constant time for individual files |
| Path joining | O(1) | String concatenation |
| Configuration load | O(1) | Single file read |
| Task search | O(n) | Linear search through task files |

### Memory Usage
- Each structure uses dynamic memory allocation
- Path strings are duplicated for safety
- Embedded vectors can consume significant memory
- File handles are properly closed after operations

### Optimization Opportunities
1. Implement caching for frequently accessed data
2. Add indexing for faster task searches
3. Use memory pooling for frequent allocations
4. Implement lazy loading for large datasets

## Security Considerations

### Data Security
1. **File Permissions**: All directories created with 0755 permissions
2. **Path Sanitization**: Input validation for all file operations
3. **Memory Safety**: Proper allocation and freeing of all resources
4. **Error Handling**: Comprehensive error checking throughout

### Authentication Security
1. **API Key Storage**: Secure storage of authentication tokens
2. **User Identification**: Fingerprint-based system identification
3. **Configuration Protection**: Sensitive data in configuration files

### Best Practices
- Always validate paths before file operations
- Use secure memory allocation patterns
- Implement proper error recovery
- Regular backup of critical data

## Testing Strategies

### Unit Testing
```c
// Example test structure
void test_storage_initialization() {
    // Test directory creation
    // Test file existence
    // Test permission settings
}

void test_config_management() {
    // Test config save/load
    // Test default values
    // Test error conditions
}
```

### Integration Testing
1. **End-to-end workflow testing**
2. **File system interaction testing**
3. **Multi-agent coordination testing**
4. **Error recovery testing**

### Performance Testing
1. **Load testing with large datasets**
2. **Memory leak detection**
3. **File I/O performance benchmarking**
4. **Concurrent access testing**

## Deployment Instructions

### Prerequisites
- C compiler (GCC recommended)
- Standard C library
- POSIX-compliant system
- Sufficient disk space for storage

### Build Process
```bash
# Compile the library
gcc -c todozi.c -o todozi.o
gcc -shared todozi.o -o libtodozi.so

# Or compile statically
gcc -c todozi.c -o todozi.o
ar rcs libtodozi.a todozi.o
```

### Installation
```bash
# Copy library to system location
sudo cp libtodozi.so /usr/local/lib/
sudo cp todozi.h /usr/local/include/

# Update library cache
sudo ldconfig
```

### Configuration
1. Run initialization: `init_storage()`
2. Set up user registration
3. Configure AI settings if needed
4. Verify folder structure with `check_folder_structure()`

## Usage Examples

### Basic Initialization
```c
#include "todozi.h"

int main() {
    // Initialize storage
    if (init_storage() != TODOZI_SUCCESS) {
        printf("Failed to initialize storage\n");
        return 1;
    }
    
    // Check structure
    if (!check_folder_structure()) {
        printf("Folder structure invalid\n");
        return 1;
    }
    
    // Load configuration
    Config* config = load_config();
    if (!config) {
        printf("Failed to load config\n");
        return 1;
    }
    
    // Use the system...
    free_config(config);
    return 0;
}
```

### Task Management
```c
// Create and save a task
Task* task = malloc(sizeof(Task));
task->id = strdup("task_001");
task->action = strdup("Implement feature X");
task->status = strdup("active");
task->priority = strdup("high");

if (save_task(task) != TODOZI_SUCCESS) {
    printf("Failed to save task\n");
}

free_task(task);
```

### Agent System
```c
// Create specialized agents
Agent* planner = create_planner_agent();
Agent* tester = create_tester_agent();

// Save agents
char* planner_file = make_json_filename(planner->id);
// ... save logic

free_agent(planner);
free_agent(tester);
```

### Project Management
```c
// Create project
Project* project = malloc(sizeof(Project));
project->name = strdup("New Project");
project->description = strdup("Project description");

if (save_project(project) != TODOZI_SUCCESS) {
    printf("Failed to save project\n");
}

free_project(project);
```

## Troubleshooting Guide

### Common Issues

#### 1. Storage Initialization Failure
**Symptoms**: `init_storage()` returns error
**Solutions**:
- Check disk space availability
- Verify write permissions in home directory
- Check for existing .todozi directory conflicts

#### 2. Configuration Load Failure
**Symptoms**: `load_config()` returns NULL
**Solutions**:
- Verify tdz.hlx file exists and is readable
- Check file permissions
- Validate configuration file syntax

#### 3. Memory Allocation Errors
**Symptoms**: Segmentation faults or NULL returns
**Solutions**:
- Check system memory availability
- Verify proper error handling in calling code
- Use memory debugging tools

#### 4. File Permission Issues
**Symptoms**: Save operations failing
**Solutions**:
- Verify directory permissions
- Check for locked files
- Ensure proper user permissions

### Debugging Techniques

#### 1. Enable Verbose Logging
```c
// Add debug prints to track execution flow
printf("DEBUG: Entering function %s\n", __FUNCTION__);
```

#### 2. Memory Debugging
Use tools like Valgrind to detect memory leaks:
```bash
valgrind --leak-check=full ./your_program
```

#### 3. File System Inspection
Manually check the storage structure:
```bash
ls -la ~/.todozi/
find ~/.todozi/ -name "*.json" -exec cat {} \;
```

### Recovery Procedures

#### 1. Data Corruption
- Use backup files from backups/ directory
- Run integrity checks on data files
- Reinitialize storage if necessary

#### 2. Configuration Loss
- Restore from backup configuration
- Use default configuration template
- Re-run registration process

#### 3. Agent System Failure
- Recreate default agents with `create_default_agents()`
- Verify agent configuration files
- Check agent assignment integrity

This documentation provides comprehensive coverage of the Todozi C library. The system is designed for robustness, extensibility, and performance, with careful attention to memory management and error handling.