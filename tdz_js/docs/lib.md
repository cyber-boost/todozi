# Todozi Core Module Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Module Structure](#module-structure)
4. [Core Classes](#core-classes)
5. [Functions](#functions)
6. [Usage Examples](#usage-examples)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi core module is the central hub of the Todozi task management system. It provides a comprehensive API for managing tasks, ideas, memories, and other productivity elements. The module is designed to work with both local storage and cloud-based services, offering semantic search capabilities through AI embeddings.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                                    Todozi Core                                      │
├─────────────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │   Storage    │  │   Models     │  │   API        │  │   Embedding  │            │
│  │              │  │              │  │              │  │   Service    │            │
│  │ - Local/Hybrid│  │ - Task       │  │ - REST       │  │              │            │
│  │ - Cloud Sync  │  │ - Priority   │  │ - Auth       │  │ - Semantic   │            │
│  │ - Projects    │  │ - Status     │  │ - Keys       │  │   Search     │            │
│  └──────────────┘  │ - Assignee   │  └──────────────┘  │ - Clustering │            │
│                    │ - Filters    │                   │ - Similarity │            │
│                    └──────────────┘                   └──────────────┘            │
│                                                                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │   Agents     │  │   Search     │  │   Memory     │  │   Idea       │            │
│  │              │  │              │  │              │  │              │            │
│  │ - AI Agents  │  │ - Keyword    │  │ - Memories   │  │ - Ideas      │            │
│  │ - Planning   │  │ - Semantic   │  │ - Important  │  │ - Breakthrough│            │
│  │ - Execution  │  │ - Filters    │  │ - Long-term  │  │ - Innovation │            │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘            │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

## Module Structure

The Todozi core module exports several submodules:

- `agent`: AI agent management
- `api`: API communication layer
- `base`: Base classes and utilities
- `chunking`: Content chunking utilities
- `cli`: Command-line interface
- `emb`: Embedding services
- `error`: Error handling
- `extract`: Content extraction
- `idea`: Idea management
- `memory`: Memory management
- `migration`: Data migration tools
- `models`: Data models
- `nodejs`: Node.js specific utilities
- `php`: PHP integration
- `python`: Python integration
- `reminder`: Reminder system
- `ruby`: Ruby integration
- `search`: Search functionality
- `server`: Server management
- `storage`: Data storage
- `summary`: Content summarization
- `tags`: Tag management
- `tdz`: Todozi core utilities
- `tdz_tls`: Thread-local storage
- `todozi`: Main Todozi functionality
- `toolbox`: Utility tools
- `tui`: Terminal user interface
- `types`: Type definitions

## Core Classes

### Done Class

The `Done` class is the primary interface for interacting with the Todozi system.

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| `projectName` | string | The current project name (default: 'external_apps') |

#### Methods

##### Initialization Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `init()` | None | Promise<void> | Initializes the Todozi system |
| `initWithAutoRegistration()` | None | Promise<void> | Initializes with automatic registration |
| `todoziBegin()` | None | Promise<void> | Begins Todozi initialization |
| `ensureTodoziInitialized()` | None | Promise<void> | Ensures Todozi is initialized |
| `findTdz(str)` | str: string | Promise<string> | Finds Todozi directory |

##### Task Management Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `createTask(action, priority, project, time, context)` | action: string, priority: Priority, project: string, time: string, context: string | Promise<Task> | Creates a new task |
| `searchTasks(query, semantic, limit)` | query: string, semantic: boolean, limit: number | Promise<Array<Task>> | Searches for tasks |
| `updateTaskStatus(taskId, status)` | taskId: string, status: Status | Promise<void> | Updates task status |
| `listTasks()` | None | Promise<Array<Task>> | Lists all tasks |
| `getTask(taskId)` | taskId: string | Promise<Task> | Gets a specific task |
| `deleteTask(taskId)` | taskId: string | Promise<void> | Deletes a task |
| `quickTask(action)` | action: string | Promise<Task> | Creates a quick task |

##### AI Integration Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `extractTasks(content, context)` | content: string, context: string | Promise<Array<string>> | Extracts tasks from content |
| `planTasks(goal, complexity, timeline, context)` | goal: string, complexity: string, timeline: string, context: string | Promise<Array<Task>> | Plans tasks for a goal |
| `processChat(message, userId)` | message: string, userId: string | Promise<any> | Processes chat message |

##### Memory and Idea Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `createMemory(moment, meaning, reason)` | moment: string, meaning: string, reason: string | Promise<Memory> | Creates a memory |
| `createIdea(idea, context)` | idea: string, context: string | Promise<Idea> | Creates an idea |
| `remember(moment, meaning)` | moment: string, meaning: string | Promise<Memory> | Creates a memory |
| `ideate(idea)` | idea: string | Promise<Idea> | Creates an idea |

##### Utility Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `apiKey()` | None | Promise<string> | Gets API key |
| `storage()` | None | Promise<Storage> | Gets storage instance |
| `embeddingService()` | None | Promise<EmbeddingService> | Gets embedding service |
| `searchWithFilters(filters, limit)` | filters: TaskFilters, limit: number | Promise<Array<Task>> | Searches with filters |
| `updateTaskFull(taskId, updates)` | taskId: string, updates: TaskUpdate | Promise<void> | Updates task fully |

### Ready Class

The `Ready` class handles initialization and readiness checks.

#### Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `init()` | None | Promise<void> | Initializes Todozi |

## Functions

### Core Functions

#### `init()`
Initializes the storage system.

**Parameters:** None
**Returns:** Promise<void>
**Throws:** Any storage initialization errors

#### `initWithAutoRegistration()`
Initializes the system with automatic registration.

**Parameters:** None
**Returns:** Promise<void>
**Throws:** Registration errors

#### `tdzfp()`
Checks folder structure.

**Parameters:** None
**Returns:** boolean
**Throws:** File system errors

#### `todoziBegin()`
Begins Todozi initialization process.

**Parameters:** None
**Returns:** Promise<void>
**Throws:** Initialization errors

#### `getTdzApiKey()`
Retrieves the Todozi API key.

**Parameters:** None
**Returns:** Promise<string>
**Throws:** Configuration errors

#### `ensureTodoziInitialized()`
Ensures Todozi is properly initialized.

**Parameters:** None
**Returns:** Promise<void>
**Throws:** Initialization errors

#### `findTdz(str)`
Finds the Todozi directory.

**Parameters:** 
- `str` (string): Directory path component
**Returns:** Promise<string>
**Throws:** File system errors

## Usage Examples

### Basic Task Management

```javascript
// Initialize Todozi
await Done.init();

// Create a simple task
const taskId = await Done.quickTask("Complete project documentation");

// Create a task with specific properties
const task = await Done.createTask(
    "Review code changes",
    Done.models.Priority.High,
    "development",
    "2023-12-31",
    "Code review for sprint completion"
);

// Search for tasks
const tasks = await Done.searchTasks("documentation", false, 10);

// Update task status
await Done.updateTaskStatus(taskId, Done.models.Status.Done);

// List all tasks
const allTasks = await Done.listTasks();
```

### AI-Powered Features

```javascript
// Extract tasks from content
const extractedTasks = await Done.extractTasks(
    "We need to review the documentation, update the API, and test the new features",
    "Project planning meeting notes"
);

// Plan tasks for a goal
const plannedTasks = await Done.planTasks(
    "Launch new product feature",
    "medium",
    "2 weeks",
    "Product roadmap Q1 2024"
);

// Semantic search
const similarTasks = await Done.searchTasks("user authentication", true, 5);

// Generate embeddings
const embedding = await Done.embed("Implement user login functionality");
```

### Memory and Idea Management

```javascript
// Create a memory
const memory = await Done.createMemory(
    "Team meeting discussion",
    "Decided to use microservices architecture",
    "Architecture planning session"
);

// Create an idea
const idea = await Done.createIdea(
    "Implement voice commands for task management",
    "Explore voice recognition APIs for hands-free task entry"
);

// Quick memory creation
await Done.remember("Important client feedback", "Client prefers dark mode interface");

// Quick idea creation
await Done.ideate("Add gamification elements to task completion");
```

### Project Management

```javascript
// Create a new project
await Done.createProject("Q1-2024-Initiatives", "First quarter projects");

// List tasks in a project
const projectTasks = await Done.tasks("Q1-2024-Initiatives");

// Delete a project
await Done.deleteProject("Old-Project-Archive");
```

### Advanced Search and Analysis

```javascript
// Smart search
const smartResults = await Done.smart("find all authentication related tasks");

// Fast keyword search
const fastResults = await Done.fast("bug fix");

// Deep semantic search
const deepResults = await Done.deep("security vulnerability");

// Find similar tasks
const similar = await Done.similarTasks(taskId);

// Get system statistics
const stats = await Done.quick();
```

## Design Patterns

### Singleton Pattern
The `Done` class acts as a singleton providing global access to Todozi functionality.

### Factory Pattern
Task creation methods like `createTask`, `quickTask`, and specialized creators (`ai`, `human`, `collab`) follow the factory pattern.

### Observer Pattern
The system uses event-driven architecture for task updates and notifications.

### Strategy Pattern
Different search strategies (keyword, semantic, smart) are implemented through separate methods.

### Facade Pattern
The `Done` class provides a simplified interface to complex subsystems.

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Task Creation | O(1) | Basic operation |
| Task Search (Keyword) | O(n) | Linear search through tasks |
| Task Search (Semantic) | O(n log n) | Embedding comparison and sorting |
| Task Update | O(1) | Direct update |
| Task Deletion | O(1) | Direct deletion |

### Space Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Task Storage | O(n) | Where n is number of tasks |
| Embedding Cache | O(m) | Where m is cached embeddings |
| Search Index | O(k) | Where k is indexed content |

### Optimization Strategies

1. **Caching**: Frequently accessed tasks and embeddings are cached
2. **Indexing**: Search indexes are maintained for faster lookups
3. **Batch Operations**: Multiple operations can be batched for efficiency
4. **Lazy Loading**: Components are loaded only when needed

### Performance Recommendations

1. Use `quickTask` for simple task creation
2. Limit search results with the `limit` parameter
3. Use semantic search judiciously as it's more resource-intensive
4. Batch operations when possible
5. Regularly clean up completed tasks to maintain performance

## Security Considerations

### Data Protection

1. **Encryption at Rest**: Sensitive data should be encrypted in storage
2. **Secure API Keys**: API keys are stored securely and rotated regularly
3. **Access Control**: Role-based access control for different operations
4. **Audit Logging**: All operations are logged for security auditing

### Authentication and Authorization

1. **API Key Authentication**: All API calls require valid API keys
2. **Token Expiration**: Tokens have expiration times
3. **Rate Limiting**: API calls are rate-limited to prevent abuse
4. **Secure Registration**: Registration process validates user identity

### Input Validation

1. **Sanitization**: All inputs are sanitized to prevent injection attacks
2. **Validation**: Input validation prevents malformed data
3. **Size Limits**: Input size limits prevent buffer overflow attacks
4. **Type Checking**: Strong type checking prevents type confusion

### Network Security

1. **HTTPS**: All communications use HTTPS
2. **Certificate Pinning**: Certificate pinning prevents man-in-the-middle attacks
3. **Secure Headers**: Security headers are set for API responses
4. **CORS Protection**: Cross-origin resource sharing is properly configured

## Testing Strategies

### Unit Testing

```javascript
// Test task creation
test('createTask should create a valid task', async () => {
    const task = await Done.createTask('Test task', null, null, null, null);
    expect(task.action).toBe('Test task');
    expect(task.id).toBeDefined();
});

// Test search functionality
test('searchTasks should return matching tasks', async () => {
    await Done.createTask('Find this task', null, null, null, null);
    const results = await Done.searchTasks('Find this', false, 10);
    expect(results.length).toBeGreaterThan(0);
});
```

### Integration Testing

```javascript
// Test full workflow
test('full task workflow', async () => {
    // Create task
    const task = await Done.createTask('Integration test task', null, null, null, null);
    
    // Verify creation
    const retrieved = await Done.getTask(task.id);
    expect(retrieved.action).toBe('Integration test task');
    
    // Update status
    await Done.updateTaskStatus(task.id, Done.models.Status.Done);
    
    // Verify update
    const updated = await Done.getTask(task.id);
    expect(updated.status).toBe(Done.models.Status.Done);
    
    // Delete task
    await Done.deleteTask(task.id);
    
    // Verify deletion
    const deleted = await Done.getTask(task.id);
    expect(deleted).toBeUndefined();
});
```

### Performance Testing

```javascript
// Test search performance
test('search performance', async () => {
    // Create test data
    for (let i = 0; i < 1000; i++) {
        await Done.createTask(`Test task ${i}`, null, null, null, null);
    }
    
    // Measure search time
    const start = Date.now();
    const results = await Done.searchTasks('Test', false, 10);
    const end = Date.now();
    
    expect(end - start).toBeLessThan(1000); // Should complete in under 1 second
    expect(results.length).toBe(10);
});
```

### Security Testing

```javascript
// Test input validation
test('input validation', async () => {
    // Test SQL injection
    try {
        await Done.createTask("'; DROP TABLE tasks; --", null, null, null, null);
        // Should not create malicious task
        const tasks = await Done.searchTasks("DROP TABLE", false, 10);
        expect(tasks.length).toBe(0);
    } catch (error) {
        // Expected to throw validation error
        expect(error).toBeDefined();
    }
});
```

## Deployment Instructions

### Prerequisites

1. Node.js v16+ installed
2. npm or yarn package manager
3. Access to Todozi server (optional for cloud features)

### Installation

```bash
# Clone the repository
git clone https://github.com/todozi/todozi-core.git
cd todozi-core

# Install dependencies
npm install

# Build the project
npm run build
```

### Configuration

1. Create a `.todozi` directory in the user's home directory
2. Configure `tdz.hlx` with registration information
3. Set up API keys for cloud services if needed

### Environment Variables

```bash
TODOZI_API_KEY=your_api_key_here
TODOZI_SERVER_URL=https://todozi.com
TODOZI_STORAGE_PATH=/path/to/storage
```

### Deployment Options

#### Local Deployment

```bash
# Start the local server
npm start

# Run in development mode
npm run dev
```

#### Cloud Deployment

1. Deploy to cloud provider (AWS, GCP, Azure)
2. Configure environment variables
3. Set up database connections
4. Configure load balancing if needed

#### Docker Deployment

```dockerfile
FROM node:16-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 3000
CMD ["npm", "start"]
```

### Monitoring and Maintenance

1. Set up logging and monitoring
2. Configure automated backups
3. Regular security updates
4. Performance monitoring

## Troubleshooting Guide

### Common Issues

#### Initialization Errors

**Problem**: `ensureTodoziInitialized` fails
**Solution**: 
1. Check `.todozi` directory permissions
2. Verify `tdz.hlx` configuration file exists
3. Ensure network connectivity for registration

#### API Key Issues

**Problem**: API calls fail with authentication errors
**Solution**:
1. Verify API key in `tdz.hlx`
2. Check API key expiration
3. Regenerate API key if necessary

#### Storage Issues

**Problem**: Task operations fail
**Solution**:
1. Check disk space availability
2. Verify file permissions
3. Restart storage service

#### Search Performance

**Problem**: Semantic search is slow
**Solution**:
1. Check embedding service availability
2. Verify network connectivity
3. Consider reducing search scope

### Error Codes

| Code | Description | Solution |
|------|-------------|----------|
| ValidationError | Input validation failed | Check input parameters |
| TaskNotFound | Task ID not found | Verify task exists |
| StorageError | Storage operation failed | Check storage configuration |
| NetworkError | Network connectivity issue | Check network connection |
| AuthError | Authentication failed | Verify API key |

### Debugging Steps

1. **Check Logs**: Examine system logs for error messages
2. **Verify Configuration**: Ensure all configuration files are correct
3. **Test Connectivity**: Verify network connectivity to required services
4. **Check Permissions**: Ensure proper file and directory permissions
5. **Validate Inputs**: Confirm all inputs are properly formatted

### Recovery Procedures

#### Database Corruption

1. Restore from latest backup
2. Run consistency checks
3. Re-index search data
4. Validate task integrity

#### Service Outages

1. Check service status
2. Restart affected services
3. Failover to backup systems
4. Notify users of downtime

#### Data Loss

1. Restore from backups
2. Recover from cloud storage
3. Reconstruct from logs
4. Implement additional backup strategies

This comprehensive documentation provides a complete overview of the Todozi core module, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.