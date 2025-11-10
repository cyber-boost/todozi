# TodoziSharp - .NET Wrapper for Todozi

A .NET 8.0 wrapper for the Todozi C library, providing comprehensive task management capabilities with a modern .NET API.

## Features

- **Full P/Invoke Integration**: Direct binding to the native Todozi C library
- **Memory Management**: Automatic memory management with IDisposable pattern
- **Error Handling**: Proper .NET exception handling for native errors
- **Task Management**: Create, search, and manage tasks with semantic search
- **API Key Management**: Built-in API key generation and activation
- **Cross-Platform**: Works on Windows, Linux, and macOS

## Installation

### Using NuGet (Coming Soon)
```bash
dotnet add package TodoziSharp
```

### Building from Source
1. Ensure you have the Todozi C library built and available
2. Clone this repository
3. Build the project:

```bash
cd todozi-sharp/TodoziSharp
dotnet build
```

## Usage

```csharp
using TodoziSharp;

// Create a new Todozi instance
using var todozi = new Todozi();

// Add a task
string? taskId = todozi.AddTask("Implement user authentication", "High priority security feature");

// Search for tasks
var tasks = todozi.SearchTasks("authentication", semantic: true);

// Get an API key
string? apiKey = todozi.GetApiKey();

// Activate an API key for a user
bool activated = todozi.ActivateKey("user123");
```

## Dependencies

This wrapper depends on the native Todozi C library. You can obtain it via:

### Conan
```bash
conan install todozi/0.1.0@
```

### Manual Build
Build the C library from the `tdz_c` directory and ensure the shared library is in your system's library path.

## Platform Support

- **Windows**: x64
- **Linux**: x64
- **macOS**: x64 (Intel/Apple Silicon)

## API Reference

### Todozi Class

#### Constructor
```csharp
public Todozi()
```

Creates a new Todozi instance. Throws `InvalidOperationException` if initialization fails.

#### Task Management
```csharp
public string? AddTask(string taskName, string? description = null)
```
Adds a new task and returns its ID.

```csharp
public IEnumerable<Task> SearchTasks(string query, bool semantic = false, int limit = 10)
```
Searches for tasks using text or semantic search.

```csharp
public IEnumerable<Task> GetTasks()
```
Retrieves all tasks.

#### API Key Management
```csharp
public string? GetApiKey()
```
Generates and returns an API key.

```csharp
public bool ActivateKey(string userId)
```
Activates an API key for the specified user.

### Task Class

Represents a task with the following properties:
- `string? Id`: Unique task identifier
- `string? Name`: Task name
- `string? Description`: Task description
- `TaskPriority Priority`: Task priority level
- `TaskStatus Status`: Current task status
- `DateTime? CreatedAt`: Creation timestamp
- `DateTime? UpdatedAt`: Last update timestamp

### Enums

#### TaskPriority
- `Low`
- `Medium`
- `High`
- `Urgent`
- `Critical`

#### TaskStatus
- `Todo`
- `InProgress`
- `Done`
- `Blocked`

### Exceptions

#### TodoziException
Thrown when native Todozi operations fail.
- `int ErrorCode`: Native error code
- `string Message`: Error message

## Building

### Prerequisites
- .NET 8.0 SDK
- Native Todozi C library
- Conan (recommended for dependency management)

### Build Steps
```bash
# Install dependencies
conan install ../tdz_c --build=missing

# Build the wrapper
dotnet build

# Run tests
dotnet test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Links

- [Website](https://todozi.com) - Official Todozi website
- [GitHub](https://github.com/cyber-boost/todozi) - Source code repository
- [Todozi C Library](https://github.com/cyber-boost/todozi/tree/main/tdz_c) - The native C implementation
- [Todozi Conan Package](https://conan.io/center/todozi) - C library package
