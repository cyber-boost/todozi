# TodoziSharp

A modern, high-performance task management library built in C# for .NET 8.0, providing comprehensive task tracking, semantic search, and API key management capabilities.

## Features

- **High-Performance Task Management**: Built-in C# implementation for optimal performance
- **Semantic Search**: Advanced search capabilities with natural language processing
- **API Key Management**: Secure key generation and user activation system
- **Cross-Platform**: Native .NET implementation supporting Windows, Linux, and macOS
- **Async/Await Support**: Modern asynchronous programming patterns
- **LINQ Integration**: Full LINQ support for querying and manipulating tasks
- **JSON Serialization**: Built-in JSON support for data persistence and API integration
- **Dependency Injection Ready**: Clean architecture with DI-friendly design

## Installation

### Using NuGet (Recommended)
```bash
dotnet add package TodoziSharp --version 0.1.0
```

The package is published on NuGet.org: https://www.nuget.org/packages/TodoziSharp

### Building from Source
```bash
git clone https://github.com/cyber-boost/todozi.git
cd todozi/tdz_sharp/TodoziSharp
dotnet build
```

## Quick Start

```csharp
using TodoziSharp;

// Create a task manager instance
var taskManager = new TaskManager();

// Create a new task
var task = new TaskItem
{
    Name = "Implement user authentication",
    Description = "High priority security feature requiring JWT tokens",
    Priority = TaskPriority.High,
    Status = TaskStatus.Todo
};

string taskId = await taskManager.AddTaskAsync(task);

// Search for tasks using semantic search
var searchResults = await taskManager.SearchTasksAsync("security features", useSemanticSearch: true);

// Update task status
await taskManager.UpdateTaskStatusAsync(taskId, TaskStatus.InProgress);

// Get API key for user
string apiKey = await taskManager.GenerateApiKeyAsync();
bool activated = await taskManager.ActivateApiKeyAsync(apiKey, "user123");
```

## Advanced Usage

### Task Filtering and Queries

```csharp
// Get all high-priority tasks
var highPriorityTasks = await taskManager.GetTasksAsync(
    filter: t => t.Priority >= TaskPriority.High);

// Get tasks created in the last 7 days
var recentTasks = await taskManager.GetTasksAsync(
    filter: t => t.CreatedAt >= DateTime.UtcNow.AddDays(-7));

// Search with custom scoring
var scoredResults = await taskManager.SearchWithScoringAsync(
    "authentication security",
    minScore: 0.7,
    maxResults: 20);
```

### Batch Operations

```csharp
// Bulk task creation
var newTasks = new[]
{
    new TaskItem { Name = "Setup CI/CD pipeline", Priority = TaskPriority.Medium },
    new TaskItem { Name = "Write unit tests", Priority = TaskPriority.High },
    new TaskItem { Name = "Update documentation", Priority = TaskPriority.Low }
};

var createdTaskIds = await taskManager.AddTasksAsync(newTasks);

// Bulk status updates
await taskManager.UpdateTaskStatusesAsync(createdTaskIds, TaskStatus.InProgress);
```

### Event Handling

```csharp
// Subscribe to task events
taskManager.TaskCreated += (sender, args) =>
    Console.WriteLine($"Task created: {args.Task.Name}");

taskManager.TaskUpdated += (sender, args) =>
    Console.WriteLine($"Task updated: {args.Task.Id} - {args.ChangeType}");

taskManager.ApiKeyActivated += (sender, args) =>
    Console.WriteLine($"API key activated for user: {args.UserId}");
```

## Architecture

TodoziSharp follows clean architecture principles:

- **Core**: Domain models and business logic
- **Infrastructure**: Data persistence and external services
- **API**: Public interfaces and DTOs
- **Extensions**: LINQ extensions and utility methods

## API Reference

### TaskManager Class

The main entry point for task management operations.

#### Key Methods

```csharp
// Task Operations
Task<string> AddTaskAsync(TaskItem task)
Task<IEnumerable<TaskItem>> GetTasksAsync(Func<TaskItem, bool>? filter = null)
Task<TaskItem?> GetTaskAsync(string taskId)
Task UpdateTaskAsync(string taskId, TaskItem updatedTask)
Task DeleteTaskAsync(string taskId)

// Search Operations
Task<IEnumerable<TaskItem>> SearchTasksAsync(string query, bool useSemanticSearch = false)
Task<IEnumerable<ScoredTaskResult>> SearchWithScoringAsync(string query, double minScore = 0.5)

// API Key Operations
Task<string> GenerateApiKeyAsync()
Task<bool> ActivateApiKeyAsync(string apiKey, string userId)
Task<bool> ValidateApiKeyAsync(string apiKey)
Task DeactivateApiKeyAsync(string apiKey)
```

### TaskItem Class

Represents a task with full metadata support.

```csharp
public class TaskItem
{
    public string Id { get; set; }
    public string Name { get; set; }
    public string Description { get; set; }
    public TaskPriority Priority { get; set; }
    public TaskStatus Status { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
    public string[] Tags { get; set; }
    public Dictionary<string, object> Metadata { get; set; }
}
```

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
- `Review`
- `Done`
- `Blocked`
- `Cancelled`

## Configuration

TodoziSharp supports configuration through `IConfiguration`:

```csharp
builder.Services.AddTodoziSharp(options =>
{
    options.EnableSemanticSearch = true;
    options.MaxSearchResults = 100;
    options.EnableCaching = true;
    options.CacheExpirationMinutes = 30;
});
```

## Performance

- **In-Memory Operations**: Sub-millisecond task operations
- **Semantic Search**: GPU-accelerated when available
- **Concurrent Access**: Thread-safe operations with optimistic locking
- **Memory Efficient**: Lazy loading and streaming for large datasets

## Testing

```bash
dotnet test TodoziSharp.Tests
```

Run performance benchmarks:
```bash
dotnet run --project TodoziSharp.Benchmarks
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Support

- 📖 [Documentation](https://docs.todozi.com)
- 🐛 [Issue Tracker](https://github.com/cyber-boost/todozi/issues)
- 💬 [Discussions](https://github.com/cyber-boost/todozi/discussions)
- 📧 [Email Support](mailto:support@todozi.com)

## Roadmap

- [ ] Mobile SDK (MAUI)
- [ ] Web API integration
- [ ] Advanced analytics dashboard
- [ ] Plugin architecture
- [ ] GraphQL API support
