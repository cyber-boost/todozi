# Todozi Task Management System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [Detailed API Documentation](#detailed-api-documentation)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive task management system built with Node.js that provides a rich set of features for organizing, tracking, and managing tasks, projects, and workflows. The system includes advanced features such as AI agent integration, memory management, queue systems, and API key management.

### Key Features

- Task and project management with rich metadata
- AI agent system with customizable behaviors
- Memory and idea management
- Queue system for task prioritization
- API key management for secure access
- Comprehensive error handling and logging
- Semantic search capabilities
- Data migration support

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                            Todozi System                            │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   Task Mgmt │  │ Project Mgmt│  │   Agent Mgmt│  │  Memory Mgmt│ │
│  │             │  │             │  │             │  │             │ │
│  │  Task       │  │  Project    │  │  Agent      │  │  Memory     │ │
│  │  TaskUpdate │  │  Config     │  │  AgentTool  │  │  Idea       │ │
│  │  TaskFilters│  │  ProjectStats│  │  AgentBehav.│  │  Feeling    │ │
│  │  TaskColl.  │  │             │  │  AgentConst.│  │             │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
│                                                                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │  Queue Mgmt │  │   API Mgmt  │  │  Security   │  │   Utilities │ │
│  │             │  │             │  │             │  │             │ │
│  │  QueueItem  │  │  ApiKey     │  │  ApiKeyColl.│  │  MLEngine   │ │
│  │  QueueSess. │  │             │  │             │  │  Summary    │ │
│  │  QueueColl. │  │             │  │             │  │  Reminder   │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                        Error Handling                          │ │
│  │  TodoziError      ValidationError    InvalidPriority           │ │
│  │  InvalidStatus    InvalidProgress                              │ │
│  └─────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Enum-like Classes

The system uses enum-like classes for type safety and consistency:

- **Priority**: Task priority levels (Low, Medium, High, Critical, Urgent)
- **Status**: Task status states (Todo, Pending, InProgress, etc.)
- **Assignee**: Task assignee types (AI, Human, Collaborative, Agent)
- **MemoryImportance**: Memory importance levels
- **MemoryTerm**: Memory term durations (Short, Long)
- **MemoryType**: Memory types (Standard, Secret, Human, Emotional)
- **CoreEmotion**: Core emotional states
- **ShareLevel**: Data sharing levels (Private, Team, Public)
- **IdeaImportance**: Idea importance levels
- **ItemStatus**: General item status (Active, Archived, Deleted)
- **ErrorSeverity**: Error severity levels
- **ErrorCategory**: Error category classifications
- **TrainingDataType**: Training data types
- **ProjectStatus**: Project status states
- **AgentStatus**: Agent status states
- **AssignmentStatus**: Assignment status states
- **QueueStatus**: Queue item status states
- **SummaryPriority**: Summary priority levels
- **ReminderPriority**: Reminder priority levels
- **ReminderStatus**: Reminder status states

### 2. Core Data Models

#### Task Management
- **Task**: Core task entity with comprehensive metadata
- **TaskUpdate**: Builder pattern for task updates
- **TaskFilters**: Filtering criteria for task queries
- **TaskCollection**: Container for managing collections of tasks
- **Project**: Project entity with task associations
- **ProjectTaskContainer**: Specialized container for project-based task management

#### Agent System
- **Agent**: AI agent with configurable capabilities
- **ModelConfig**: AI model configuration
- **AgentTool**: Agent tool definitions
- **AgentBehaviors**: Agent behavioral settings
- **AgentConstraints**: Agent operational constraints
- **AgentMetadata**: Agent descriptive metadata
- **AgentAssignment**: Agent-task assignment tracking

#### Queue System
- **QueueItem**: Queueable task item
- **QueueSession**: Work session tracking
- **QueueCollection**: Queue item management

#### Memory & Ideas
- **Memory**: Memory entity with rich context
- **Idea**: Idea capture and management
- **Feeling**: Emotional state tracking

#### API & Security
- **ApiKey**: API key management
- **ApiKeyCollection**: Collection of API keys
- **RegistrationInfo**: User registration information

#### Utilities
- **Config**: System configuration
- **Summary**: Content summarization
- **Reminder**: Reminder scheduling
- **MLEngine**: Machine learning engine stub
- **Tag**: Tagging system
- **Error**: Error tracking and management
- **TrainingData**: Training data management

## Detailed API Documentation

### Enum Classes

#### Priority
```javascript
class Priority {
  static Low = 'low';
  static Medium = 'medium';
  static High = 'high';
  static Critical = 'critical';
  static Urgent = 'urgent';

  static fromStr(s); // Convert string to Priority enum
}
```

**Methods:**
- `fromStr(s: string): Priority` - Converts a string to a Priority enum value

#### Status
```javascript
class Status {
  static Todo = 'todo';
  static Pending = 'pending';
  static InProgress = 'in_progress';
  static Blocked = 'blocked';
  static Review = 'review';
  static Done = 'done';
  static Completed = 'completed';
  static Cancelled = 'cancelled';
  static Deferred = 'deferred';

  static fromStr(s); // Convert string to Status enum
}
```

**Methods:**
- `fromStr(s: string): Status` - Converts a string to a Status enum value

### Task Management Classes

#### Task
```javascript
class Task {
  constructor({
    userId,
    action,
    time,
    priority,
    parentProject,
    status,
    assignee = null,
    tags = [],
    dependencies = [],
    contextNotes = null,
    progress = null,
    embeddingVector = null,
  });

  static new(userId, action, time, priority, parentProject, status);
  static newFull(userId, action, time, priority, parentProject, status, assignee, tags, dependencies, contextNotes, progress);
  update(updates);
  complete();
  isCompleted();
  isActive();
}
```

**Constructor Parameters:**
- `userId` (string): User identifier
- `action` (string): Task description
- `time` (string): Task time/due date
- `priority` (Priority): Task priority level
- `parentProject` (string): Parent project identifier
- `status` (Status): Task status
- `assignee` (any, optional): Task assignee
- `tags` (Array, optional): Task tags
- `dependencies` (Array, optional): Task dependencies
- `contextNotes` (string, optional): Additional context
- `progress` (number, optional): Task progress percentage
- `embeddingVector` (Array, optional): Vector embedding for ML

**Static Methods:**
- `new(userId, action, time, priority, parentProject, status): Task` - Creates a basic task
- `newFull(...): Task` - Creates a fully featured task

**Instance Methods:**
- `update(updates: Object): void` - Updates task properties
- `complete(): void` - Marks task as completed
- `isCompleted(): boolean` - Checks if task is completed
- `isActive(): boolean` - Checks if task is active

#### TaskUpdate
```javascript
class TaskUpdate {
  constructor();
  static new();
  withAction(action);
  withTime(time);
  withPriority(priority);
  withParentProject(parentProject);
  withStatus(status);
  withAssignee(assignee);
  withTags(tags);
  withDependencies(dependencies);
  withContextNotes(contextNotes);
  withProgress(progress);
}
```

**Methods:**
- `withAction(action: string): TaskUpdate` - Sets action
- `withTime(time: string): TaskUpdate` - Sets time
- `withPriority(priority: Priority): TaskUpdate` - Sets priority
- `withParentProject(project: string): TaskUpdate` - Sets parent project
- `withStatus(status: Status): TaskUpdate` - Sets status
- `withAssignee(assignee: any): TaskUpdate` - Sets assignee
- `withTags(tags: Array): TaskUpdate` - Sets tags
- `withDependencies(deps: Array): TaskUpdate` - Sets dependencies
- `withContextNotes(notes: string): TaskUpdate` - Sets context notes
- `withProgress(progress: number): TaskUpdate` - Sets progress

#### TaskFilters
```javascript
class TaskFilters {
  constructor({
    project = null,
    status = null,
    priority = null,
    assignee = null,
    tags = null,
    search = null,
  } = {});
}
```

**Constructor Parameters:**
- `project` (string, optional): Filter by project
- `status` (Status, optional): Filter by status
- `priority` (Priority, optional): Filter by priority
- `assignee` (any, optional): Filter by assignee
- `tags` (Array, optional): Filter by tags
- `search` (string, optional): Search text

#### TaskCollection
```javascript
class TaskCollection {
  constructor();
  static new();
  addTask(task);
  getTask(id);
  getTaskMut(id);
  removeTask(id);
  getAllTasks();
  getFilteredTasks(filters);
  static default();
}
```

**Methods:**
- `addTask(task: Task): void` - Adds a task to collection
- `getTask(id: string): Task` - Gets a task by ID
- `getTaskMut(id: string): Task` - Gets mutable task reference
- `removeTask(id: string): Task` - Removes and returns task
- `getAllTasks(): Array<Task>` - Gets all tasks
- `getFilteredTasks(filters: TaskFilters): Array<Task>` - Gets filtered tasks

### Project Management

#### Project
```javascript
class Project {
  constructor(name, description = null);
  addTask(taskId);
  removeTask(taskId);
  archive();
  complete();
}
```

**Constructor Parameters:**
- `name` (string): Project name
- `description` (string, optional): Project description

**Methods:**
- `addTask(taskId: string): void` - Adds task to project
- `removeTask(taskId: string): void` - Removes task from project
- `archive(): void` - Archives the project
- `complete(): void` - Marks project as completed

#### ProjectTaskContainer
```javascript
class ProjectTaskContainer {
  constructor(projectName);
  static new(projectName);
  addTask(task);
  getTask(taskId);
  getTaskMut(taskId);
  removeTask(taskId);
  updateTaskStatus(taskId, newStatus);
  getAllTasks();
  getFilteredTasks(filters);
  static default();
}
```

**Constructor Parameters:**
- `projectName` (string): Name of the project

**Methods:**
- `addTask(task: Task): void` - Adds task to appropriate status container
- `getTask(taskId: string): Task` - Gets task by ID
- `getTaskMut(taskId: string): Task` - Gets mutable task reference
- `removeTask(taskId: string): Task` - Removes and returns task
- `updateTaskStatus(taskId: string, newStatus: Status): Task` - Updates task status
- `getAllTasks(): Array<Task>` - Gets all tasks in container
- `getFilteredTasks(filters: TaskFilters): Array<Task>` - Gets filtered tasks

### Agent System

#### Agent
```javascript
class Agent {
  constructor(id, name, description);
  static new(id, name, description);
  static createCoder();
  hasCapability(capability);
  hasSpecialization(specialization);
  hasTool(toolName);
  getEnabledTools();
  setStatus(status);
  isAvailable();
  getFormattedPrompt(variables);
}
```

**Constructor Parameters:**
- `id` (string): Agent identifier
- `name` (string): Agent name
- `description` (string): Agent description

**Static Methods:**
- `new(id, name, description): Agent` - Creates new agent
- `createCoder(): Agent` - Creates specialized coder agent

**Methods:**
- `hasCapability(capability: string): boolean` - Checks agent capabilities
- `hasSpecialization(specialization: string): boolean` - Checks specializations
- `hasTool(toolName: string): boolean` - Checks if tool is enabled
- `getEnabledTools(): Array<AgentTool>` - Gets enabled tools
- `setStatus(status: AgentStatus): void` - Sets agent status
- `isAvailable(): boolean` - Checks if agent is available
- `getFormattedPrompt(variables: Object): string` - Formats system prompt

### Queue System

#### QueueItem
```javascript
class QueueItem {
  constructor({
    taskName,
    taskDescription,
    priority,
    projectId = null,
  });
  start();
  complete();
  isBacklog();
  isActive();
  isComplete();
}
```

**Constructor Parameters:**
- `taskName` (string): Queue item name
- `taskDescription` (string): Queue item description
- `priority` (Priority): Queue item priority
- `projectId` (string, optional): Associated project ID

**Methods:**
- `start(): void` - Marks item as active
- `complete(): void` - Marks item as complete
- `isBacklog(): boolean` - Checks if item is in backlog
- `isActive(): boolean` - Checks if item is active
- `isComplete(): boolean` - Checks if item is complete

#### QueueSession
```javascript
class QueueSession {
  constructor(queueItemId);
  end();
  isActive();
  getCurrentDuration();
}
```

**Constructor Parameters:**
- `queueItemId` (string): Associated queue item ID

**Methods:**
- `end(): void` - Ends the session
- `isActive(): boolean` - Checks if session is active
- `getCurrentDuration(): number` - Gets current session duration in seconds

#### QueueCollection
```javascript
class QueueCollection {
  constructor();
  static new();
  addItem(item);
  getItem(id);
  getItemMut(id);
  removeItem(id);
  getAllItems();
  getItemsByStatus(status);
  getBacklogItems();
  getActiveItems();
  getCompleteItems();
  startSession(queueItemId);
  endSession(sessionId);
  getActiveSessions();
  getSession(id);
  static default();
}
```

**Methods:**
- `addItem(item: QueueItem): void` - Adds item to collection
- `getItem(id: string): QueueItem` - Gets item by ID
- `getItemMut(id: string): QueueItem` - Gets mutable item reference
- `removeItem(id: string): QueueItem` - Removes and returns item
- `getAllItems(): Array<QueueItem>` - Gets all items
- `getItemsByStatus(status: QueueStatus): Array<QueueItem>` - Gets items by status
- `getBacklogItems(): Array<QueueItem>` - Gets backlog items
- `getActiveItems(): Array<QueueItem>` - Gets active items
- `getCompleteItems(): Array<QueueItem>` - Gets complete items
- `startSession(queueItemId: string): string` - Starts new session
- `endSession(sessionId: string): void` - Ends session
- `getActiveSessions(): Array<QueueSession>` - Gets active sessions
- `getSession(id: string): QueueSession` - Gets session by ID

### API & Security

#### ApiKey
```javascript
class ApiKey {
  constructor({
    userId,
    publicKey,
    privateKey,
    active = true,
    createdAt,
    updatedAt,
  });
  static new();
  static withUserId(userId);
  deactivate();
  activate();
  isActive();
  matches(publicKey, privateKey = null);
  isAdmin(publicKey, privateKey);
}
```

**Constructor Parameters:**
- `userId` (string): User identifier
- `publicKey` (string): Public key
- `privateKey` (string): Private key
- `active` (boolean, optional): Key active status
- `createdAt` (string, optional): Creation timestamp
- `updatedAt` (string, optional): Last update timestamp

**Static Methods:**
- `new(): ApiKey` - Generates new API key pair
- `withUserId(userId: string): ApiKey` - Generates key pair for user

**Methods:**
- `deactivate(): void` - Deactivates the key
- `activate(): void` - Activates the key
- `isActive(): boolean` - Checks if key is active
- `matches(publicKey: string, privateKey: string): boolean` - Validates key pair
- `isAdmin(publicKey: string, privateKey: string): boolean` - Checks admin status

#### ApiKeyCollection
```javascript
class ApiKeyCollection {
  constructor();
  static new();
  addKey(key);
  getKey(userId);
  getKeyByPublic(publicKey);
  getAllKeys();
  getActiveKeys();
  removeKey(userId);
  deactivateKey(userId);
  activateKey(userId);
  static default();
}
```

**Methods:**
- `addKey(key: ApiKey): void` - Adds key to collection
- `getKey(userId: string): ApiKey` - Gets key by user ID
- `getKeyByPublic(publicKey: string): ApiKey` - Gets key by public key
- `getAllKeys(): Array<ApiKey>` - Gets all keys
- `getActiveKeys(): Array<ApiKey>` - Gets active keys
- `removeKey(userId: string): ApiKey` - Removes and returns key
- `deactivateKey(userId: string): boolean` - Deactivates key
- `activateKey(userId: string): boolean` - Activates key

### Error Handling

#### TodoziError
```javascript
class TodoziError extends Error {
  constructor(type, details);
}
```

**Constructor Parameters:**
- `type` (string): Error type
- `details` (Object): Error details

#### ValidationError
```javascript
class ValidationError extends TodoziError {
  constructor(message);
}
```

**Constructor Parameters:**
- `message` (string): Validation error message

#### InvalidPriority
```javascript
class InvalidPriority extends TodoziError {
  constructor(priority);
}
```

**Constructor Parameters:**
- `priority` (any): Invalid priority value

#### InvalidStatus
```javascript
class InvalidStatus extends TodoziError {
  constructor(status);
}
```

**Constructor Parameters:**
- `status` (any): Invalid status value

#### InvalidProgress
```javascript
class InvalidProgress extends TodoziError {
  constructor(progress);
}
```

**Constructor Parameters:**
- `progress` (number): Invalid progress value

## Usage Examples

### 1. Basic Task Management

```javascript
// Create a new task
const task = Task.new(
  'user123',
  'Complete project documentation',
  '2023-12-31T23:59:59Z',
  Priority.High,
  'project-docs',
  Status.Todo
);

// Update task progress
const update = TaskUpdate.new()
  .withProgress(50)
  .withStatus(Status.InProgress);

task.update(update);

// Complete the task
task.complete();

console.log(`Task ${task.id} is completed: ${task.isCompleted()}`);
```

### 2. Project Management

```javascript
// Create a new project
const project = new Project('Website Redesign', 'Redesign company website');

// Add tasks to project
const task1 = Task.new('user123', 'Design mockups', '2023-12-15T00:00:00Z', Priority.High, 'Website Redesign', Status.Todo);
const task2 = Task.new('user123', 'Implement frontend', '2023-12-30T00:00:00Z', Priority.Critical, 'Website Redesign', Status.Todo);

project.addTask(task1.id);
project.addTask(task2.id);

console.log(`Project has ${project.tasks.length} tasks`);
```

### 3. Agent System

```javascript
// Create a specialized coder agent
const coderAgent = Agent.createCoder();

// Check agent capabilities
if (coderAgent.hasCapability('code_development')) {
  console.log('Agent can develop code');
}

// Check specializations
if (coderAgent.hasSpecialization('javascript')) {
  console.log('Agent specializes in JavaScript');
}

// Format a prompt for the agent
const prompt = coderAgent.getFormattedPrompt({
  task: 'Create a REST API',
  language: 'Node.js',
  context: 'E-commerce platform',
  requirements: 'Use Express.js and MongoDB'
});

console.log(prompt);
```

### 4. Queue System

```javascript
// Create queue collection
const queue = QueueCollection.new();

// Add items to queue
const item1 = new QueueItem({
  taskName: 'Review code',
  taskDescription: 'Review pull request #123',
  priority: Priority.Medium,
  projectId: 'project1'
});

const item2 = new QueueItem({
  taskName: 'Fix bug',
  taskDescription: 'Fix login authentication bug',
  priority: Priority.Critical,
  projectId: 'project1'
});

queue.addItem(item1);
queue.addItem(item2);

// Start working on an item
const sessionId = queue.startSession(item1.id);

// Get active sessions
const activeSessions = queue.getActiveSessions();
console.log(`Active sessions: ${activeSessions.length}`);

// End session
queue.endSession(sessionId);
```

### 5. API Key Management

```javascript
// Create API key collection
const keyCollection = ApiKeyCollection.new();

// Generate new API key
const apiKey = ApiKey.new();
keyCollection.addKey(apiKey);

// Check if key is valid
const isValid = apiKey.matches(apiKey.publicKey, apiKey.privateKey);
console.log(`API key is valid: ${isValid}`);

// Deactivate key
apiKey.deactivate();
console.log(`API key is active: ${apiKey.isActive()}`);
```

### 6. Memory Management

```javascript
// Create a memory entry
const memory = new Memory({
  id: `mem_${uuidv4().substring(0, 8)}`,
  userId: 'user123',
  projectId: 'project1',
  moment: '2023-12-01T10:00:00Z',
  meaning: 'Important client meeting',
  reason: 'Discussed project requirements',
  importance: MemoryImportance.High,
  term: MemoryTerm.Long,
  memoryType: MemoryType.Human,
  tags: ['client', 'requirements']
});

console.log(`Memory created: ${memory.id}`);
```

### 7. Filtering Tasks

```javascript
// Create task collection
const taskCollection = TaskCollection.new();

// Add some tasks
const task1 = Task.new('user123', 'Write documentation', '2023-12-15T00:00:00Z', Priority.High, 'docs', Status.Todo);
const task2 = Task.new('user123', 'Fix bug', '2023-12-10T00:00:00Z', Priority.Critical, 'bug-fix', Status.InProgress);
const task3 = Task.new('user123', 'Review code', '2023-12-20T00:00:00Z', Priority.Medium, 'code-review', Status.Done);

taskCollection.addTask(task1);
taskCollection.addTask(task2);
taskCollection.addTask(task3);

// Filter tasks
const filters = new TaskFilters({
  priority: Priority.High,
  status: Status.Todo
});

const filteredTasks = taskCollection.getFilteredTasks(filters);
console.log(`Found ${filteredTasks.length} high priority todo tasks`);
```

## Design Patterns

### 1. Builder Pattern
The `TaskUpdate` class implements the Builder pattern to provide a fluent interface for updating task properties:

```javascript
const update = TaskUpdate.new()
  .withAction('Updated task description')
  .withPriority(Priority.Critical)
  .withStatus(Status.InProgress);
```

### 2. Factory Pattern
Several classes use factory methods for object creation:
- `Task.new()` and `Task.newFull()`
- `Agent.new()` and `Agent.createCoder()`
- `ApiKey.new()` and `ApiKey.withUserId()`

### 3. Singleton Pattern
The `Config` class can be used as a singleton with `Config.default()`.

### 4. Observer Pattern
The queue system uses an observer-like pattern where sessions observe queue item status changes.

### 5. Strategy Pattern
The agent system allows different behaviors to be configured through the `AgentBehaviors` class.

### 6. Decorator Pattern
The `ProjectTaskContainer` decorates the basic task management functionality with project-specific organization.

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Task Creation | O(1) | Simple object instantiation |
| Task Update | O(1) | Direct property assignment |
| Task Retrieval | O(1) | Map lookup |
| Task Filtering | O(n) | Linear scan through collection |
| Project Task Management | O(1) | Map operations |
| Queue Operations | O(1) | Map operations |
| API Key Validation | O(n) | Linear search through keys |

### Memory Usage

- **Task Objects**: ~1KB per task (varies with metadata)
- **Project Objects**: ~500B per project
- **Agent Objects**: ~2KB per agent (with tools and configurations)
- **Queue Items**: ~500B per item
- **API Keys**: ~300B per key

### Optimization Recommendations

1. **Caching**: Implement caching for frequently accessed tasks and projects
2. **Indexing**: Create indexes for common filter fields (priority, status, assignee)
3. **Pagination**: Implement pagination for large task collections
4. **Lazy Loading**: Load task details only when needed
5. **Batch Operations**: Support batch task updates and deletions

## Security Considerations

### 1. API Key Security

- **Key Generation**: Uses cryptographically secure random generation
- **Storage**: Keys should be stored securely with proper access controls
- **Transmission**: Keys should only be transmitted over HTTPS
- **Rotation**: Implement key rotation policies

### 2. Data Validation

- **Input Sanitization**: All inputs are validated and sanitized
- **Enum Validation**: Strict enum validation prevents injection attacks
- **Progress Validation**: Prevents invalid progress values (>100%)

### 3. Access Control

- **User Isolation**: Tasks are associated with specific users
- **Project Permissions**: Project-based access control
- **Key-based Authentication**: API key-based authentication system

### 4. Data Integrity

- **Immutable Timestamps**: Creation timestamps cannot be modified
- **Audit Trails**: Update timestamps track all changes
- **Status Validation**: Strict status transitions prevent invalid states

### 5. Rate Limiting

- **Agent Constraints**: Built-in rate limiting for agents
- **API Key Limits**: Configurable request limits per key

## Testing Strategies

### Unit Testing

```javascript
// Example unit test for Task class
describe('Task', () => {
  test('should create task with valid parameters', () => {
    const task = Task.new('user123', 'Test task', '2023-12-31T23:59:59Z', Priority.High, 'test-project', Status.Todo);
    expect(task.userId).toBe('user123');
    expect(task.action).toBe('Test task');
    expect(task.priority).toBe(Priority.High);
  });

  test('should throw error for invalid progress', () => {
    expect(() => {
      Task.newFull('user123', 'Test task', '2023-12-31T23:59:59Z', Priority.High, 'test-project', Status.Todo, null, [], [], null, 150);
    }).toThrow(InvalidProgress);
  });
});
```

### Integration Testing

```javascript
// Example integration test for TaskCollection
describe('TaskCollection', () => {
  test('should manage tasks correctly', () => {
    const collection = TaskCollection.new();
    const task = Task.new('user123', 'Test task', '2023-12-31T23:59:59Z', Priority.High, 'test-project', Status.Todo);
    
    collection.addTask(task);
    expect(collection.getAllTasks()).toHaveLength(1);
    
    const retrievedTask = collection.getTask(task.id);
    expect(retrievedTask).toEqual(task);
    
    collection.removeTask(task.id);
    expect(collection.getAllTasks()).toHaveLength(0);
  });
});
```

### Mock Testing

```javascript
// Example mock test for Agent system
describe('Agent', () => {
  test('should format prompt correctly', () => {
    const agent = Agent.new('test-agent', 'Test Agent', 'Testing agent');
    const variables = {
      task: 'Complete testing',
      language: 'JavaScript'
    };
    
    const prompt = agent.getFormattedPrompt(variables);
    expect(prompt).toContain('Complete testing');
    expect(prompt).toContain('JavaScript');
  });
});
```

## Deployment Instructions

### Prerequisites

1. Node.js v16+ installed
2. npm or yarn package manager
3. MongoDB or compatible database (if persistence is needed)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd todozi

# Install dependencies
npm install

# Install required packages
npm install uuid crypto
```

### Configuration

Create a configuration file:

```javascript
// config.js
const { Config } = require('./todozi');

const config = Config.default();
config.version = "1.2.0";
config.defaultProject = "general";
config.aiEnabled = true;

module.exports = config;
```

### Running the Application

```bash
# Start the application
node app.js

# Run with specific environment
NODE_ENV=production node app.js
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

EXPOSE 3000

CMD ["node", "app.js"]
```

```bash
# Build and run Docker container
docker build -t todozi .
docker run -p 3000:3000 todozi
```

### Environment Variables

```bash
# Required environment variables
NODE_ENV=production
PORT=3000
MONGODB_URI=mongodb://localhost:27017/todozi
JWT_SECRET=your-jwt-secret
```

## Troubleshooting Guide

### Common Issues

#### 1. Invalid Priority Error
**Symptom**: `InvalidPriority` error when creating tasks
**Solution**: Ensure priority values are from the `Priority` enum:
```javascript
// Correct
const task = Task.new(userId, action, time, Priority.High, project, status);

// Incorrect
const task = Task.new(userId, action, time, 'high-priority', project, status);
```

#### 2. Invalid Status Error
**Symptom**: `InvalidStatus` error when updating tasks
**Solution**: Use valid status values from the `Status` enum:
```javascript
// Correct
task.update({ status: Status.InProgress });

// Incorrect
task.update({ status: 'working-on-it' });
```

#### 3. Progress Validation Error
**Symptom**: `InvalidProgress` error when setting progress
**Solution**: Ensure progress values are between 0-100:
```javascript
// Correct
task.update({ progress: 75 });

// Incorrect
task.update({ progress: 150 });
```

#### 4. API Key Validation Issues
**Symptom**: Authentication failures with API keys
**Solution**: 
1. Verify key format and length
2. Check if key is active
3. Ensure proper transmission over HTTPS
4. Validate both public and private key components

#### 5. Queue Session Management
**Symptom**: Session not starting or ending properly
**Solution**:
1. Ensure queue item exists and is in backlog status
2. Verify session ID format
3. Check for concurrent session conflicts

### Performance Issues

#### Slow Task Filtering
**Solution**: 
1. Implement indexing on filter fields
2. Use pagination for large collections
3. Cache frequently accessed filter results

#### Memory Leaks
**Solution**:
1. Monitor memory usage with Node.js profiling tools
2. Implement proper cleanup for completed sessions
3. Use weak references where appropriate

### Logging and Monitoring

Implement comprehensive logging:

```javascript
// Example logging setup
const winston = require('winston');

const logger = winston.createLogger({
  level: 'info',
  format: winston.format.json(),
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});

// Log task operations
logger.info('Task created', { taskId: task.id, userId: task.userId });
logger.error('Task update failed', { taskId: task.id, error: error.message });
```

This comprehensive documentation provides a complete overview of the Todozi task management system, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.