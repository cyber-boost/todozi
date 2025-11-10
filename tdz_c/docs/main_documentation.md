# Todozi CLI - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Functions](#functions)
5. [Usage Examples](#usage-examples)
6. [Error Handling](#error-handling)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive task management system with a command-line interface that provides extensive functionality for task management, project organization, AI integration, and data persistence. The system supports multiple commands ranging from basic task operations to advanced features like embeddings export, migration, and AI agent management.

### Key Features
- **Task Management**: Add, list, show, update, complete, and delete tasks
- **Project Organization**: Project-based task grouping and management
- **Backup System**: Automated backup and restore functionality
- **Registration System**: Optional server registration for cloud synchronization
- **AI Integration**: Embeddings, chat functionality, and ML capabilities
- **Migration Tools**: Data migration between system versions
- **Multiple Interfaces**: CLI, TUI, and GUI support

## Architecture

### System Architecture Diagram
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Parser    │ →  │ Command Handler │ →  │    Storage      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                      │                      │
         ↓                      ↓                      ↓
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Error Handler  │    │   Todozi API    │    │   HLX Format    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Component Interactions
```
User Input → CLI Parser → Command Router → Specific Handler → Storage → HLX File
                                 ↓
                           Error Handler ←── Exception/Error
```

### Data Flow
1. **Input**: Command line arguments
2. **Parsing**: `parse_cli()` function extracts command and parameters
3. **Validation**: Command validation and parameter checking
4. **Execution**: Command-specific handler executes the operation
5. **Storage**: Data persistence through Storage layer to HLX files
6. **Output**: Result display or error reporting

## Data Structures

### Core Structures

#### TodoziError
```c
typedef struct {
    int code;        // Error code (system errno or custom)
    char* message;   // Human-readable error message
} TodoziError;
```
**Purpose**: Unified error handling structure across the system
**Fields**:
- `code`: Error code matching system errno or custom error codes
- `message`: Dynamically allocated error message string

#### Cli (Command Line Interface)
```c
typedef struct {
    CommandType command_type;  // Type of command to execute
    void* command_data;        // Command-specific parameters
    bool has_command;          // Whether a valid command was parsed
} Cli;
```
**Purpose**: Represents parsed command-line arguments
**Fields**:
- `command_type`: Enumeration of possible command types
- `command_data`: Pointer to command-specific data structure
- `has_command`: Flag indicating successful command parsing

### Command-Specific Structures

#### AddCmd - Add Command
```c
typedef struct {
    char* title;        // Task title (required)
    char* description;  // Task description (optional)
} AddCmd;
```

#### ListCmd - List Command
```c
typedef struct {
    char* filter;       // Filter string for task listing
} ListCmd;
```

#### UpdateCmd - Update Command
```c
typedef struct {
    int id;             // Task ID to update
    char* action;       // Action description
    char* time;         // Time-related updates
    char* priority;     // Priority level
    char* project;      // Project assignment
    char* status;       // Status update
    char* assignee;     // Assignee information
    char* tags;         // Tag updates
    char* dependencies; // Dependency information
    char* context;      // Context information
    int progress;       // Progress percentage
} UpdateCmd;
```

#### Storage and Handler Structures
```c
struct Storage {
    void* data;         // Opaque storage implementation data
};

struct TodoziHandler {
    Storage* storage;   // Associated storage instance
};

struct Hlx {
    void* data;         // HLX format data structure
};

struct PathBuf {
    char* path;         // File system path
};
```

## Functions

### Core Functions

#### Main Entry Point
```c
int main(int argc, char* argv[])
```
**Purpose**: Program entry point and main control flow
**Parameters**:
- `argc`: Argument count
- `argv`: Argument vector
**Returns**: Exit status code (0 for success, non-zero for error)
**Flow**:
1. Install error handling
2. Parse CLI arguments
3. Handle early exits (help, errors)
4. Initialize storage and handlers
5. Load HLX data
6. Execute command
7. Cleanup resources

#### CLI Parsing
```c
Cli parse_cli(int argc, char* argv[])
```
**Purpose**: Parse command-line arguments into structured command data
**Parameters**:
- `argc`: Argument count
- `argv`: Argument vector
**Returns**: `Cli` structure containing parsed command information
**Complexity**: O(n) where n is number of arguments

#### Memory Management
```c
void free_cli_data(Cli* cli)
```
**Purpose**: Safely free all dynamically allocated command data
**Parameters**:
- `cli`: Pointer to Cli structure to cleanup
**Features**: Idempotent (safe to call multiple times)

### Error Handling Functions

#### Error Cleanup
```c
static void todozi_error_cleanup(TodoziError* err)
```
**Purpose**: Safely free error message memory
**Parameters**:
- `err`: Error structure to cleanup
**Safety**: Checks for NULL pointers before freeing

#### Error Handling Macros
```c
#define CHECK(expr)     // Execute expression and exit on error
#define HANDLE_ERROR(err_var, msg_prefix)  // Handle error with message prefix
```

### Storage Functions

#### Storage Management
```c
TodoziError storage_new(Storage** storage)
```
**Purpose**: Create new storage instance
**Parameters**:
- `storage`: Double pointer to receive new storage instance
**Returns**: Error structure indicating success/failure
**Error Codes**: `EINVAL` (invalid pointer), `ENOMEM` (allocation failure)

```c
static void storage_free(Storage* storage)
```
**Purpose**: Free storage instance
**Parameters**:
- `storage`: Storage instance to free

### Handler Functions

#### Handler Management
```c
TodoziError todozi_handler_new(Storage* storage, TodoziHandler** handler)
```
**Purpose**: Create new todozi handler with associated storage
**Parameters**:
- `storage`: Storage instance to associate
- `handler`: Double pointer to receive new handler
**Returns**: Error structure indicating success/failure

```c
static void todozi_handler_free(TodoziHandler* handler)
```
**Purpose**: Free handler instance
**Parameters**:
- `handler`: Handler instance to free

### File System Functions

#### Directory Discovery
```c
static char* find_todozi(void* none)
```
**Purpose**: Find todozi directory using multiple fallback strategies
**Parameters**:
- `none`: Unused parameter (for API compatibility)
**Returns**: Dynamically allocated path string
**Search Order**:
1. `TODOZI_HOME` environment variable
2. `~/.todozi` (user home directory)
3. `./todozi` (current directory)
4. Fallback to `./todozi`

#### HLX File Management
```c
TodoziError hlx_load(const char* path, Hlx** hlx)
```
**Purpose**: Load HLX format file
**Parameters**:
- `path`: File path to load
- `hlx`: Double pointer to receive HLX instance
**Returns**: Error structure indicating success/failure

```c
void hlx_free(Hlx* hlx)
```
**Purpose**: Free HLX instance
**Parameters**:
- `hlx`: HLX instance to free

## Usage Examples

### Basic Task Management

#### Initialize Todozi
```bash
todozi init
```
**Purpose**: Initialize todozi directory structure
**Files Created**: Configuration files, storage directories

#### Add a Task
```bash
todozi add "Complete project documentation" "Write comprehensive docs for Todozi CLI"
```
**Purpose**: Add new task with title and description
**Output**: Task ID and confirmation message

#### List Tasks
```bash
todozi list
todozi list "important"  # Filtered listing
```
**Purpose**: Display all tasks or filtered subset
**Output**: Formatted task list with IDs, titles, and status

#### Show Task Details
```bash
todozi show 1
```
**Purpose**: Display detailed information for specific task
**Output**: Complete task details including metadata

#### Update Task
```bash
todozi update 1 --priority high --progress 50
```
**Purpose**: Modify task properties
**Parameters**: Various update options supported

#### Complete Task
```bash
todozi complete 1
```
**Purpose**: Mark task as completed
**Output**: Confirmation message

### Advanced Operations

#### Backup and Restore
```bash
todozi backup
todozi list-backups
todozi restore backup_20231201
```
**Purpose**: Manage data backups
**Features**: Automated backup creation and selective restore

#### Registration Management
```bash
todozi register https://api.todozi.com
todozi registration-status
todozi clear-registration
```
**Purpose**: Manage server registration for cloud sync
**Note**: Registration is optional for local operation

#### Export Embeddings
```bash
todozi export-embeddings ./ai_training_data.hlx
```
**Purpose**: Export task data in HLX format for AI/ML training
**Use Case**: Machine learning model training data preparation

#### Migration
```bash
todozi migrate --dry-run  # Test migration
todozi migrate --force    # Execute migration
```
**Purpose**: Migrate data between system versions
**Safety**: Dry-run option for testing migrations

### Content Management

#### Content Extraction
```bash
todozi extract "Meeting notes" --file notes.txt --format json
```
**Purpose**: Extract and process content from various sources
**Formats**: Multiple output format support

#### AI Integration
```bash
todozi chat "What tasks are due today?"
todozi agent "Process incoming emails"
```
**Purpose**: AI-powered task management assistance
**Features**: Natural language processing and automation

## Error Handling

### Error Codes System

The system uses a unified error handling approach with the following error categories:

#### System Errors (errno-based)
- `EINVAL`: Invalid parameters
- `ENOMEM`: Memory allocation failure
- `ENOENT`: File or directory not found
- `EACCES`: Permission denied

#### Custom Error Categories
- **Configuration Errors**: Invalid configuration or missing files
- **Storage Errors**: Data persistence issues
- **Network Errors**: Server communication failures
- **Validation Errors**: Invalid input data

### Error Handling Strategy

#### Defensive Programming
```c
// All functions return TodoziError for consistent error handling
TodoziError storage_new(Storage** storage) {
    if (!storage) {
        TodoziError err = {EINVAL, strdup("Invalid storage pointer")};
        return err;
    }
    // ... implementation
}
```

#### Resource Cleanup
```c
// Macros ensure proper cleanup on error
#define CHECK(expr) do { \
    TodoziError err = (expr); \
    if (err.code != 0) { \
        fprintf(stderr, "Error: %s\n", err.message); \
        todozi_error_cleanup(&err); \
        exit(err.code); \
    } \
} while(0)
```

#### Graceful Degradation
```c
// Fallback behavior when primary methods fail
static char* find_todozi(void* none) {
    // Try multiple strategies in priority order
    // Final fallback to default location
    return strdup("./todozi");
}
```

## Design Patterns

### 1. Command Pattern
**Implementation**: Each command type corresponds to a specific handler function
```c
typedef enum {
    CMD_ADD,
    CMD_LIST,
    CMD_SHOW,
    // ... other commands
} CommandType;

// Command routing in main()
switch (cli.command_type) {
    case CMD_ADD:
        err = todozi_handler_handle_add_command(handler, add_cmd);
        break;
    case CMD_LIST:
        err = todozi_handler_handle_list_command(handler, list_cmd);
        break;
    // ... other cases
}
```

### 2. Factory Pattern
**Implementation**: Creation functions for major components
```c
TodoziError storage_new(Storage** storage);
TodoziError todozi_handler_new(Storage* storage, TodoziHandler** handler);
```

### 3. Strategy Pattern
**Implementation**: Different storage strategies and command implementations
```c
// Storage abstraction allows different implementations
struct Storage {
    void* data;  // Opaque implementation data
};
```

### 4. RAII (Resource Acquisition Is Initialization)
**Implementation**: Automatic cleanup through structured programming
```c
// Resources acquired in main() and cleaned up at the end
Storage* storage = NULL;
TodoziHandler* handler = NULL;
Hlx* todozi_hlx = NULL;

// ... acquisition code

cleanup:
    if (todozi_hlx) hlx_free(todozi_hlx);
    if (handler) todozi_handler_free(handler);
    if (storage) storage_free(storage);
```

### 5. Facade Pattern
**Implementation**: Simplified interface to complex subsystem
```c
// TodoziHandler provides simplified interface to storage+logic
TodoziError todozi_handler_handle_add_command(TodoziHandler* handler, AddCmd* add_cmd);
```

## Performance Analysis

### Time Complexity

#### Command Parsing: O(n)
- Linear time relative to number of arguments
- Efficient string comparisons

#### File Operations: O(1) to O(n)
- Constant time for existence checks
- Linear time for file loading based on size

#### Memory Operations: O(1)
- Constant time allocation and deallocation
- Efficient pointer management

### Space Complexity

#### Memory Usage: O(m + n)
- `m`: Size of loaded HLX data
- `n`: Size of command parameters and temporary buffers

#### Disk Usage
- Primary storage: HLX files with efficient binary format
- Backups: Compressed or differential backup strategies

### Optimization Strategies

#### Efficient Memory Management
```c
// Reuse of buffers and careful allocation
void free_cli_data(Cli* cli) {
    // Comprehensive cleanup of all allocated resources
}
```

#### Lazy Loading
```c
// HLX files loaded only when needed
if (cli.command_type != CMD_EXPORT_EMBEDDINGS) {
    TodoziError hlx_err = hlx_load(hlx_path, &todozi_hlx);
}
```

#### Batch Operations
```c
// Commands designed for batch processing where appropriate
TodoziError todozi_handler_handle_list_command(TodoziHandler* handler, ListCmd* list_cmd);
```

## Security Considerations

### Input Validation

#### Command Injection Prevention
```c
// All user input treated as data, not executable code
char* command = argv[1];
if (!command) {
    cli.command_type = CMD_HELP;
    return cli;
}
```

#### Path Traversal Protection
```c
// Secure path construction with bounds checking
char hlx_path[1024];
int snprintf_result = snprintf(hlx_path, sizeof(hlx_path), "%s/tdz.hlx", todozi_dir_str);
if (snprintf_result < 0 || snprintf_result >= (int)sizeof(hlx_path)) {
    fprintf(stderr, "Path too long for HLX file\n");
    goto cleanup_error;
}
```

### Data Protection

#### Secure Storage
- Local data stored in user-controlled directories
- Optional encryption for sensitive data
- Backup integrity verification

#### Network Security
- TLS/SSL for server communications
- Authentication token management
- Secure credential storage

### Privacy Considerations

#### Data Minimization
- Only essential data collected and stored
- Optional features clearly documented
- User control over data sharing

#### Access Controls
- File permission management
- User-based access restrictions
- Audit logging capabilities

## Testing Strategies

### Unit Testing

#### Test Categories

1. **Command Parsing Tests**
```c
// Test parse_cli with various inputs
void test_parse_cli() {
    char* args[] = {"todozi", "add", "test task"};
    Cli cli = parse_cli(3, args);
    assert(cli.command_type == CMD_ADD);
    assert(cli.has_command == true);
    free_cli_data(&cli);
}
```

2. **Error Handling Tests**
```c
// Test error propagation and cleanup
void test_error_handling() {
    TodoziError err = {EINVAL, strdup("Test error")};
    assert(err.code == EINVAL);
    assert(strcmp(err.message, "Test error") == 0);
    todozi_error_cleanup(&err);
    assert(err.message == NULL);
}
```

3. **File System Tests**
```c
// Test directory discovery and file operations
void test_file_operations() {
    char* path = find_todozi(NULL);
    assert(path != NULL);
    // Test file loading and validation
    free(path);
}
```

### Integration Testing

#### Test Scenarios

1. **End-to-End Workflow**
```bash
# Test complete task lifecycle
todozi init
todozi add "Test task"
todozi list
todozi show 1
todozi complete 1
todozi list
```

2. **Error Recovery Testing**
- Test system behavior under various error conditions
- Validate cleanup and recovery mechanisms

3. **Performance Testing**
- Load testing with large task datasets
- Memory usage profiling
- Response time measurements

### Automated Testing Framework

#### Continuous Integration
- Automated build and test execution
- Code coverage analysis
- Memory leak detection
- Static code analysis

## Deployment Instructions

### Build Requirements

#### Prerequisites
- C compiler (GCC, Clang, or MSVC)
- Standard C library
- POSIX-compliant system (for some features)
- Optional: Todozi API library for enhanced functionality

#### Build Process
```bash
# Basic compilation
gcc -o todozi todozi.c -std=c99 -Wall -Wextra -pedantic

# With optimizations
gcc -o todozi todozi.c -O2 -std=c99 -Wall -Wextra -pedantic

# Debug build
gcc -o todozi todozi.c -g -std=c99 -Wall -Wextra -pedantic
```

### Installation Methods

#### System-wide Installation
```bash
# Copy to system bin directory
sudo cp todozi /usr/local/bin/

# Set execute permissions
sudo chmod +x /usr/local/bin/todozi
```

#### User-local Installation
```bash
# Install in user bin directory
cp todozi ~/.local/bin/

# Add to PATH if not already
export PATH="$HOME/.local/bin:$PATH"
```

#### Package Management
```bash
# Create package (distribution-specific)
# For Debian/Ubuntu: dpkg-buildpackage
# For RedHat: rpmbuild
# For macOS: brew create formula
```

### Configuration

#### Environment Variables
```bash
# Custom todozi home directory
export TODOZI_HOME="/path/to/custom/todozi"

# Add to shell profile for persistence
echo 'export TODOZI_HOME="/path/to/custom/todozi"' >> ~/.bashrc
```

#### Initial Setup
```bash
# Initialize todozi system
todozi init

# Verify installation
todozi check-structure
```

## Troubleshooting Guide

### Common Issues

#### Installation Problems

**Issue**: "Command not found" after installation
**Solution**: Verify PATH environment variable includes installation directory
```bash
echo $PATH
which todozi
```

**Issue**: Permission denied errors
**Solution**: Check file permissions and ownership
```bash
chmod +x /path/to/todozi
ls -la /path/to/todozi
```

#### Runtime Errors

**Issue**: "HLX file not found" error
**Solution**: Initialize todozi directory structure
```bash
todozi init
todozi ensure-structure
```

**Issue**: Memory allocation failures
**Solution**: Check system memory and ulimits
```bash
ulimit -a
free -h
```

**Issue**: Command parsing errors
**Solution**: Verify command syntax and parameters
```bash
todozi help
todozi --help
```

### Debugging Techniques

#### Verbose Logging
```c
// Enable debug output by modifying source
#define DEBUG 1
#ifdef DEBUG
    fprintf(stderr, "Debug: %s\n", debug_message);
#endif
```

#### Error Investigation
```bash
# Run with strace for system call tracing
strace todozi list

# Use valgrind for memory debugging
valgrind --leak-check=full todozi list
```

#### Diagnostic Commands
```bash
# Check system status
todozi check-structure
todozi registration-status
todozi stats
```

### Recovery Procedures

#### Data Recovery
```bash
# List available backups
todozi list-backups

# Restore from backup
todozi restore backup_name

# Fix data consistency
todozi fix-consistency
```

#### Configuration Reset
```bash
# Clear registration data
todozi clear-registration

# Reinitialize system
rm -rf ~/.todozi
todozi init
```

### Performance Troubleshooting

#### Memory Usage
```bash
# Monitor memory usage
top -p $(pgrep todozi)

# Check for memory leaks
valgrind --tool=memcheck todozi list
```

#### Disk I/O
```bash
# Monitor file operations
iotop -p $(pgrep todozi)

# Check disk space
df -h ~/.todozi
```

This comprehensive documentation provides complete coverage of the Todozi CLI system, including architecture, implementation details, usage patterns, and operational guidance. The system demonstrates robust error handling, extensible design, and comprehensive functionality for modern task management needs.