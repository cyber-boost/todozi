# Todozi - A Comprehensive Task Management System in C

Todozi is a powerful, extensible task management system written in C, featuring agent management, semantic search, API endpoints, and more.

## Features

- **Agent Management**: AI agent coordination and task assignment
- **Semantic Search**: Find tasks using natural language and embeddings
- **REST API**: Full HTTP API for integration
- **CLI Interface**: Command-line tools for all operations
- **TUI Interface**: Terminal user interface
- **Memory Management**: Learning and memory systems
- **Error Tracking**: Comprehensive error handling and resolution
- **Migration Tools**: Data migration between formats
- **Extensible Architecture**: Plugin system with tool definitions

## Installation

### Using Conan (Recommended)

```bash
# Install Conan
pipx install conan

# Add Todozi to your project
conan install todozi/0.1.0@  # Add your user/channel

# For development
conan create . --build=missing

# Build with examples
conan create . --build=missing -o todozi:build_examples=True
```

### Manual Build

```bash
# Dependencies: jansson, libcurl, openssl, libuuid
make -C build/

# Build with examples
cmake -S . -B build -DBUILD_EXAMPLES=ON
cmake --build build
```

## Examples

Todozi includes 135 comprehensive examples organized by complexity level:

- **Example 1**: Basic usage of each component
- **Example 2**: Intermediate features and integrations
- **Example 3**: Advanced patterns and workflows
- **Example 4**: Real-world scenarios
- **Example 5**: Complex multi-component usage

### Building Examples

```bash
# With Conan
conan create . --build=missing -o todozi:build_examples=True

# With CMake
cmake -S . -B build -DBUILD_EXAMPLES=ON
cmake --build build

# Run examples using the helper script
./run_examples.sh example_1 1    # Run specific example
./run_examples.sh example_2     # List example_2 examples
./run_examples.sh all           # Run all examples (takes time)
```

### As a Library

```c
#include <todozi.h>

// Initialize
todozi_todozi_t* instance = todozi_new();

// Create a task
todozi_add_task(instance, "Implement user authentication", "high", "backend", "todo", "ai", NULL, 0, NULL, 0, NULL, 0);

// Search tasks
void* results = todozi_search_tasks(instance, "authentication", true, 10);

// Cleanup
todozi_free(instance);
```

### Command Line

```bash
# Add a task
todozi task add "Implement user authentication" --priority high --project backend

# List tasks
todozi task list

# Search tasks
todozi search "authentication"
```

## Dependencies

- **jansson**: JSON parsing
- **libcurl**: HTTP client
- **openssl**: Cryptography and SSL
- **libuuid**: UUID generation
- **pthread**: Threading support

## Architecture

Todozi is built with a modular architecture:

- **Core**: Base data structures and utilities
- **Agent**: AI agent management and coordination
- **API**: REST API server and client
- **CLI**: Command-line interface
- **Storage**: Data persistence layer
- **Search**: Semantic search with embeddings
- **Tools**: Extensible tool system

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Author

TonTon Bernie - A coding addict from Massachusetts working at an orphanage in Haiti.
