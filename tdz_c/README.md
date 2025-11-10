# Todozi C Library

A comprehensive task management system written in pure C, providing a complete solution for task management with AI assistance, semantic search, and cross-platform compatibility.

## Features

- **Complete Task Management**: Create, update, delete, and organize tasks
- **AI-Powered Features**: Task planning, extraction from natural language, and intelligent suggestions
- **Semantic Search**: Full-text search with semantic understanding
- **REST API**: Built-in HTTP server for remote access
- **CLI & TUI**: Command-line interface and terminal user interface
- **Cross-Platform**: Linux, macOS, and Windows support
- **135 Examples**: Comprehensive examples covering all functionality
- **Memory Management**: Efficient memory handling with automatic cleanup
- **Error Handling**: Robust error handling with detailed error messages

## Quick Start

### Installation

#### Via Conan (Recommended)
```bash
conan install todozi/0.1.0@ --build=missing
```

#### Build from Source
```bash
git clone https://github.com/cyber-boost/todozi.git
cd todozi/tdz_c
mkdir build && cd build
cmake ..
make
```

### Basic Usage

```c
#include "todozi.h"
#include <stdio.h>

int main() {
    // Initialize Todozi
    todozi_todozi_t* instance = todozi_new();
    if (!instance) {
        printf("Failed to initialize Todozi\n");
        return 1;
    }

    // Create a task
    const char* task_id = todozi_add(instance, "Implement user authentication", "High priority security feature", NULL);
    if (task_id) {
        printf("Created task: %s\n", task_id);
    }

    // Search tasks
    todozi_array_t* tasks = NULL;
    size_t count = 0;
    if (todozi_search_tasks(instance, "authentication", true, 10, &tasks, &count, NULL) == TODOZI_OK) {
        printf("Found %zu tasks\n", count);
        todozi_array_free(tasks);
    }

    // Cleanup
    todozi_free(instance);
    return 0;
}
```

## API Overview

### Core Functions

- `todozi_new()` / `todozi_free()` - Initialize and cleanup
- `todozi_add()` - Create new tasks
- `todozi_search_tasks()` - Search with semantic capabilities
- `todozi_update_task_status()` - Update task status
- `todozi_delete_task()` - Remove tasks

### Advanced Features

- `todozi_plan_tasks()` - AI-powered task planning
- `todozi_extract_tasks()` - Extract tasks from natural language
- `todozi_process_chat()` - Chat-based task management
- `todozi_api()` - REST API server

### Memory Management

All functions follow consistent memory management patterns:
- Output parameters are allocated by the library
- Use corresponding `_free()` functions for cleanup
- No memory leaks with proper usage

## Examples

The library includes 135 comprehensive examples covering:

- Basic task operations
- Advanced search functionality
- AI-powered features
- REST API usage
- Memory management
- Error handling
- Cross-platform development

### Running Examples

```bash
# Build examples
cmake -DBUILD_EXAMPLES=ON ..
make

# Run specific example
./examples/example_1_1_agent

# Run all examples
./run_examples.sh all
```

## Dependencies

- **jansson** (>= 2.14): JSON parsing and generation
- **libcurl** (>= 8.4.0): HTTP client functionality
- **OpenSSL** (>= 3.2.0): Cryptographic operations

## Building

### Prerequisites

- CMake 3.15+
- C compiler (GCC/Clang/MSVC)
- Conan package manager (recommended)

### Build Commands

```bash
# Using Conan (recommended)
conan install . --build=missing
conan build .

# Using system packages
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
```

## Testing

```bash
# Run unit tests
ctest

# Run specific test
ctest -R test_task_creation
```

## Architecture

```
tdz_c/
├── include/          # Public headers
├── src/             # Source files
│   ├── agent.c      # AI agent functionality
│   ├── api.c        # REST API implementation
│   ├── base.c       # Core data structures
│   ├── chunking.c   # Text chunking utilities
│   ├── cli.c        # Command-line interface
│   ├── emb.c        # Embeddings processing
│   ├── error.c      # Error handling
│   ├── extract.c    # Task extraction
│   ├── idea.c       # Idea management
│   ├── lib.c        # Main library interface
│   ├── memory.c     # Memory management
│   ├── migration.c  # Data migration
│   ├── models.c     # Data models
│   ├── reminder.c   # Reminder system
│   ├── search.c     # Search functionality
│   ├── server.c     # HTTP server
│   ├── storage.c    # Data persistence
│   ├── summary.c    # Summary generation
│   ├── tags.c       # Tag management
│   ├── tdz.c        # Core logic
│   ├── tests.c      # Unit tests
│   └── types.c      # Type definitions
└── examples/        # 135 example programs
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Links

- [Website](https://todozi.com) - Official Todozi website
- [GitHub](https://github.com/cyber-boost/todozi) - Source code repository
- [ConanCenter](https://conan.io/center/todozi) - Package repository
- [NuGet](https://www.nuget.org/packages/TodoziSharp) - .NET wrapper package
- [Documentation](https://github.com/cyber-boost/todozi/tree/main/tdz_c/docs) - API documentation