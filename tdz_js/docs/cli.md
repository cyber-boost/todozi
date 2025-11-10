# Todozi Handler Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [API Documentation](#api-documentation)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi Handler is a comprehensive task management system built in JavaScript that provides CLI-based functionality for managing tasks, projects, agents, and AI-powered features. It integrates with storage systems, supports API key management, queue processing, and semantic search capabilities.

### Key Features
- Task and project management
- API key generation and authentication
- Queue-based task processing
- AI agent management
- Semantic search across all data types
- Memory and idea management
- Training data collection
- Error tracking and resolution
- Embedding model management

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                    Todozi Handler CLI                       │
├─────────────────────────────────────────────────────────────┤
│  Command Handlers  │  Storage Interface  │  AI Services     │
├─────────────────────────────────────────────────────────────┤
│    Task Handler    │   Project Storage   │  Embedding       │
│   Project Handler  │   Task Storage      │  Search Engine   │
│   Agent Handler    │   API Key Storage   │  TUI Service     │
│   Queue Handler    │   Error Storage     │  Task Editor     │
│   Memory Handler   │   Training Storage  │                  │
│   Idea Handler     │   Agent Storage     │                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────────┐
                    │   File System       │
                    │   (JSON Storage)    │
                    └─────────────────────┘
```

### Data Flow Architecture
```
CLI Input → Command Parser → TodoziHandler → Storage/Service Layer → Output/Storage
```

## Core Components

### 1. TodoziHandler Class

The main handler class that processes all commands and manages the application state.

#### Constructor
```javascript
constructor(storage)
```

**Parameters:**
- `storage`: Storage interface for data persistence

#### Static Methods

##### `new(storage)`
Creates a new instance of TodoziHandler

**Parameters:**
- `storage`: Storage interface

**Returns:** Promise resolving to TodoziHandler instance

#### Instance Methods

##### `completeTask(id)`
Completes a task by ID

**Parameters:**
- `id`: Task identifier

**Returns:** Promise resolving to completion result

##### `fixTaskConsistency()`
Fixes task data consistency issues

**Returns:** Promise

##### `deleteTask(id)`
Deletes a task by ID

**Parameters:**
- `id`: Task identifier

**Returns:** Promise

##### `restoreBackup(backupName)`
Restores system from a backup

**Parameters:**
- `backupName`: Name of backup to restore

**Returns:** Promise

##### `handleApiCommand(command)`
Handles API key management commands

**Parameters:**
- `command`: API command object with type and parameters

**Returns:** Promise

##### `handleQueueCommand(command)`
Handles queue item management commands

**Parameters:**
- `command`: Queue command object

**Returns:** Promise

##### `handleServerCommand(command)`
Handles server management commands

**Parameters:**
- `command`: Server command object

**Returns:** Promise

##### `handleSearchAllCommand(command)`
Performs unified search across all data types

**Parameters:**
- `command`: Search command object

**Returns:** Promise

##### `handleChatCommand(command)`
Processes chat messages and extracts structured content

**Parameters:**
- `command`: Chat command object

**Returns:** Promise

##### `processChatMessageExtended(message, userId)`
Extended chat message processing

**Parameters:**
- `message`: Chat message content
- `userId`: User identifier

**Returns:** Promise resolving to extracted content object

##### `handleErrorCommand(command)`
Handles error management commands

**Parameters:**
- `command`: Error command object

**Returns:** Promise

##### `handleTrainCommand(command)`
Handles training data management commands

**Parameters:**
- `command`: Training command object

**Returns:** Promise

##### `handleAgentCommand(command)`
Handles agent management commands

**Parameters:**
- `command`: Agent command object

**Returns:** Promise

##### `handleEmbCommand(command)`
Handles embedding model management commands

**Parameters:**
- `command`: Embedding command object

**Returns:** Promise

##### `handleIdeaCommand(command)`
Handles idea management commands

**Parameters:**
- `command`: Idea command object

**Returns:** Promise

##### `handleMemoryCommand(command)`
Handles memory management commands

**Parameters:**
- `command`: Memory command object

**Returns:** Promise

##### `handleListBackupsCommand()`
Lists available backups

**Returns:** Promise

##### `handleStatsCommand(command)`
Displays system statistics

**Parameters:**
- `command`: Stats command object

**Returns:** Promise

##### `handleSearchCommand(command)`
Handles search commands

**Parameters:**
- `command`: Search command object

**Returns:** Promise

##### `handleProjectCommand(command)`
Handles project management commands

**Parameters:**
- `command`: Project command object

**Returns:** Promise

##### `handleUpdateCommand(id, options)`
Updates a task with specified options

**Parameters:**
- `id`: Task identifier
- `options`: Update options object

**Returns:** Promise

##### `handleShowCommand(command)`
Shows detailed information about an entity

**Parameters:**
- `command`: Show command object

**Returns:** Promise

##### `handleListCommand(command)`
Lists entities based on filters

**Parameters:**
- `command`: List command object

**Returns:** Promise

##### `handleAddCommand(command)`
Adds a new entity

**Parameters:**
- `command`: Add command object

**Returns:** Promise

##### `launchGui()`
Launches the GUI interface

**Returns:** Promise

##### `handleAiCommands(command, args)`
Handles AI-powered commands

**Parameters:**
- `command`: AI command type
- `args`: Command arguments

**Returns:** Promise

##### `handleExtractCommand(content, file, outputFormat, human)`
Handles content extraction

**Parameters:**
- `content`: Content to extract
- `file`: File path
- `outputFormat`: Output format
- `human`: Human-readable flag

**Returns:** Promise

##### `handleStrategyCommand(content, file, outputFormat, human)`
Handles strategy content processing

**Parameters:**
- `content`: Content to process
- `file`: File path
- `outputFormat`: Output format
- `human`: Human-readable flag

**Returns:** Promise

##### `handleStepsCommand(command)`
Handles task steps management

**Parameters:**
- `command`: Steps command object

**Returns:** Promise

### 2. Constants

#### Priority Enum
```javascript
const Priority = {
  LOW: 'low',
  MEDIUM: 'medium',
  HIGH: 'high',
  CRITICAL: 'critical',
  URGENT: 'urgent'
};
```

#### Status Enum
```javascript
const Status = {
  TODO: 'todo',
  PENDING: 'pending',
  IN_PROGRESS: 'in_progress',
  BLOCKED: 'blocked',
  REVIEW: 'review',
  DONE: 'done',
  COMPLETED: 'completed',
  CANCELLED: 'cancelled',
  DEFERRED: 'deferred'
};
```

#### Assignee Enum
```javascript
const Assignee = {
  AI: 'ai',
  HUMAN: 'human',
  COLLABORATIVE: 'collaborative',
  AGENT: (id) => `agent:${id}`
};
```

### 3. Helper Classes

#### TaskUpdate Class
Utility class for building task updates incrementally.

##### Methods:
- `new()`: Creates new TaskUpdate instance
- `withAction(action)`: Sets action
- `withTime(time)`: Sets time estimate
- `withPriority(priority)`: Sets priority
- `withParentProject(project)`: Sets parent project
- `withStatus(status)`: Sets status

## API Documentation

### Command Structure

All commands follow a consistent structure:
```javascript
{
  type: 'CommandType',
  ...commandSpecificParameters
}
```

### API Key Management

#### Register Command
```javascript
{
  type: 'Register',
  userId: 'optional_user_id'
}
```

#### List Command
```javascript
{
  type: 'List',
  activeOnly: boolean
}
```

#### Check Command
```javascript
{
  type: 'Check',
  publicKey: 'public_key',
  privateKey: 'private_key'
}
```

#### Deactivate Command
```javascript
{
  type: 'Deactivate',
  userId: 'user_id'
}
```

#### Activate Command
```javascript
{
  type: 'Activate',
  userId: 'user_id'
}
```

#### Remove Command
```javascript
{
  type: 'Remove',
  userId: 'user_id'
}
```

### Queue Management

#### Plan Command
```javascript
{
  type: 'Plan',
  taskName: 'Task name',
  taskDescription: 'Task description',
  priority: 'Priority level',
  projectId: 'Project ID'
}
```

#### List Command
```javascript
{
  type: 'List',
  status: 'Status filter'
}
```

#### Start Command
```javascript
{
  type: 'Start',
  queueItemId: 'Queue item ID'
}
```

#### End Command
```javascript
{
  type: 'End',
  sessionId: 'Session ID'
}
```

### Server Management

#### Start Command
```javascript
{
  type: 'Start',
  host: 'Host address',
  port: 'Port number'
}
```

#### Status Command
```javascript
{
  type: 'Status'
}
```

#### Endpoints Command
```javascript
{
  type: 'Endpoints'
}
```

## Usage Examples

### 1. Task Management

#### Creating a Task
```javascript
const handler = await TodoziHandler.new(storage);
await handler.handleAddCommand({
  type: 'Task',
  action: 'Implement user authentication',
  time: '4 hours',
  priority: 'HIGH',
  project: 'auth-system',
  status: 'TODO',
  assignee: 'HUMAN',
  tags: 'security,authentication',
  context: 'Need to implement JWT-based authentication'
});
```

#### Listing Tasks
```javascript
await handler.handleListCommand({
  type: 'Tasks',
  project: 'auth-system',
  status: 'TODO',
  priority: 'HIGH'
});
```

#### Updating a Task
```javascript
await handler.handleUpdateCommand('task-123', {
  status: 'IN_PROGRESS',
  progress: 50,
  context: 'Started implementation, working on JWT token generation'
});
```

### 2. Project Management

#### Creating a Project
```javascript
await handler.handleProjectCommand({
  type: 'Create',
  name: 'auth-system',
  description: 'User authentication system'
});
```

#### Listing Projects
```javascript
await handler.handleProjectCommand({
  type: 'List'
});
```

### 3. API Key Management

#### Registering a New API Key
```javascript
await handler.handleApiCommand({
  type: 'Register',
  userId: 'user-123'
});
```

#### Checking API Key Authentication
```javascript
await handler.handleApiCommand({
  type: 'Check',
  publicKey: 'public-key-here',
  privateKey: 'private-key-here'
});
```

### 4. Queue Management

#### Planning a Queue Item
```javascript
await handler.handleQueueCommand({
  type: 'Plan',
  taskName: 'Database migration',
  taskDescription: 'Migrate user data to new schema',
  priority: 'HIGH',
  projectId: 'data-migration'
});
```

#### Starting a Queue Session
```javascript
await handler.handleQueueCommand({
  type: 'Start',
  queueItemId: 'queue-item-456'
});
```

### 5. AI-Powered Features

#### Finding Similar Tasks
```javascript
await handler.handleAiCommands('similar', ['user authentication implementation']);
```

#### Getting AI Insights
```javascript
await handler.handleAiCommands('insights', []);
```

## Design Patterns

### 1. Command Pattern
The system extensively uses the Command pattern to encapsulate all operations as objects. This provides:
- Decoupling of command execution from command definition
- Easy addition of new commands
- Support for undo/redo operations
- Command queuing and logging

### 2. Factory Pattern
Used in creating various entities like tasks, agents, and API keys through static factory methods.

### 3. Builder Pattern
The `TaskUpdate` class implements the Builder pattern for constructing complex task update operations incrementally.

### 4. Strategy Pattern
Different storage strategies and AI models can be swapped without changing the core logic.

### 5. Singleton Pattern
Storage and configuration objects are typically implemented as singletons to ensure consistent state.

### 6. Observer Pattern
Used in the TUI service to update displays when underlying data changes.

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Task Creation | O(1) | Direct storage write |
| Task Search | O(n) | Linear search through tasks |
| Task Update | O(1) | Direct storage update |
| Project Creation | O(1) | Direct storage write |
| Project Listing | O(n) | Linear read of projects |
| API Key Validation | O(1) | Hash lookup |

### Space Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Task Storage | O(n) | Where n is number of tasks |
| Project Storage | O(m) | Where m is number of projects |
| API Key Storage | O(k) | Where k is number of API keys |
| Search Index | O(t) | Where t is total text content |

### Optimization Strategies

1. **Caching**: Frequently accessed data is cached in memory
2. **Indexing**: Search operations use indexed data structures
3. **Batch Operations**: Multiple operations are batched when possible
4. **Lazy Loading**: Data is loaded only when needed
5. **Asynchronous Processing**: I/O operations are non-blocking

### Performance Recommendations

1. **Database Indexing**: Ensure proper indexing on frequently queried fields
2. **Memory Management**: Monitor memory usage for large datasets
3. **Connection Pooling**: Use connection pooling for database operations
4. **Caching Strategy**: Implement appropriate caching for read-heavy operations
5. **Pagination**: Use pagination for large result sets

## Security Considerations

### 1. API Key Security

#### Private Key Protection
- Private keys should never be logged or stored in plain text
- Use environment variables or secure vaults for key storage
- Implement key rotation policies
- Monitor key usage and detect anomalies

#### Authentication Flow
```javascript
// Secure key validation example
async function validateApiKey(publicKey, privateKey) {
  const storedKey = await storage.getApiKey(publicKey);
  if (!storedKey || !storedKey.active) {
    throw new Error('Invalid or inactive API key');
  }
  
  const isValid = await bcrypt.compare(privateKey, storedKey.hashedPrivateKey);
  if (!isValid) {
    throw new Error('Invalid private key');
  }
  
  return { userId: storedKey.userId, isAdmin: storedKey.isAdmin };
}
```

### 2. Data Encryption

#### At-Rest Encryption
- Sensitive data should be encrypted before storage
- Use industry-standard encryption algorithms (AES-256)
- Implement proper key management

#### In-Transit Encryption
- All API communications should use HTTPS/TLS
- Implement certificate pinning for additional security
- Use secure headers (HSTS, CSP, etc.)

### 3. Input Validation

#### Sanitization
```javascript
function sanitizeInput(input) {
  // Remove potentially dangerous characters
  return input.replace(/[<>]/g, '');
}

function validateTaskInput(action, time, priority, project, status) {
  if (action.trim().length === 0) {
    throw new Error('Action cannot be empty');
  }
  
  if (action.length > 500) {
    throw new Error('Action must be less than 500 characters');
  }
  
  // Validate enum values
  if (!Object.values(Priority).includes(priority.toLowerCase())) {
    throw new Error('Invalid priority');
  }
}
```

### 4. Rate Limiting

Implement rate limiting to prevent abuse:
```javascript
const rateLimiter = {
  requests: new Map(),
  
  async checkLimit(userId, maxRequests = 100, windowMs = 3600000) {
    const now = Date.now();
    const userRequests = this.requests.get(userId) || [];
    
    // Remove old requests
    const recentRequests = userRequests.filter(time => now - time < windowMs);
    
    if (recentRequests.length >= maxRequests) {
      throw new Error('Rate limit exceeded');
    }
    
    recentRequests.push(now);
    this.requests.set(userId, recentRequests);
  }
};
```

### 5. Access Control

Implement role-based access control:
```javascript
function checkPermission(userRole, requiredPermission) {
  const permissions = {
    'admin': ['read', 'write', 'delete', 'admin'],
    'user': ['read', 'write'],
    'guest': ['read']
  };
  
  return permissions[userRole]?.includes(requiredPermission) || false;
}
```

## Testing Strategies

### 1. Unit Testing

#### Test Structure
```javascript
describe('TodoziHandler', () => {
  let handler;
  let mockStorage;
  
  beforeEach(() => {
    mockStorage = {
      addTaskToProject: jest.fn(),
      getTaskFromProject: jest.fn(),
      // ... other mock methods
    };
    handler = new TodoziHandler(mockStorage);
  });
  
  describe('handleAddCommand', () => {
    it('should create a task with valid input', async () => {
      const command = {
        type: 'Task',
        action: 'Test task',
        time: '1 hour',
        priority: 'MEDIUM',
        project: 'test-project',
        status: 'TODO'
      };
      
      await handler.handleAddCommand(command);
      
      expect(mockStorage.addTaskToProject).toHaveBeenCalled();
    });
    
    it('should throw error for invalid input', async () => {
      const command = {
        type: 'Task',
        action: '', // Invalid: empty action
        time: '1 hour',
        priority: 'MEDIUM',
        project: 'test-project',
        status: 'TODO'
      };
      
      await expect(handler.handleAddCommand(command))
        .rejects.toThrow('Action cannot be empty');
    });
  });
});
```

### 2. Integration Testing

#### Storage Integration
```javascript
describe('Storage Integration', () => {
  let handler;
  let storage;
  
  beforeAll(async () => {
    storage = await initializeTestStorage();
    handler = new TodoziHandler(storage);
  });
  
  it('should persist tasks across sessions', async () => {
    const taskCommand = {
      type: 'Task',
      action: 'Integration test task',
      time: '30 minutes',
      priority: 'HIGH',
      project: 'integration-tests',
      status: 'TODO'
    };
    
    await handler.handleAddCommand(taskCommand);
    
    const listCommand = {
      type: 'Tasks',
      project: 'integration-tests'
    };
    
    const result = await handler.handleListCommand(listCommand);
    expect(result).toContain('Integration test task');
  });
});
```

### 3. End-to-End Testing

#### CLI Command Testing
```javascript
describe('CLI Commands', () => {
  it('should handle complete workflow', async () => {
    // Create project
    await execTodoziCommand('project create test-project "Test Project"');
    
    // Add task
    await execTodoziCommand('add task "Complete integration test" --project test-project --priority HIGH');
    
    // List tasks
    const output = await execTodoziCommand('list tasks --project test-project');
    expect(output).toContain('Complete integration test');
    
    // Update task
    await execTodoziCommand('update task-123 --status IN_PROGRESS --progress 50');
    
    // Complete task
    await execTodoziCommand('complete task-123');
  });
});
```

### 4. Performance Testing

#### Load Testing
```javascript
describe('Performance', () => {
  it('should handle concurrent operations', async () => {
    const startTime = Date.now();
    const promises = [];
    
    // Create 100 concurrent task creations
    for (let i = 0; i < 100; i++) {
      promises.push(handler.handleAddCommand({
        type: 'Task',
        action: `Task ${i}`,
        time: '1 hour',
        priority: 'MEDIUM',
        project: 'performance-test',
        status: 'TODO'
      }));
    }
    
    await Promise.all(promises);
    const endTime = Date.now();
    
    // Should complete within 5 seconds
    expect(endTime - startTime).toBeLessThan(5000);
  });
});
```

### 5. Security Testing

#### Penetration Testing
```javascript
describe('Security', () => {
  it('should reject invalid API keys', async () => {
    const invalidCommand = {
      type: 'Check',
      publicKey: 'invalid-key',
      privateKey: 'invalid-private-key'
    };
    
    await expect(handler.handleApiCommand(invalidCommand))
      .rejects.toThrow('Invalid API key');
  });
  
  it('should sanitize input to prevent injection', async () => {
    const maliciousCommand = {
      type: 'Task',
      action: '<script>alert("xss")</script>Malicious task',
      time: '1 hour',
      priority: 'MEDIUM',
      project: 'test-project',
      status: 'TODO'
    };
    
    await handler.handleAddCommand(maliciousCommand);
    
    // Verify that malicious content was sanitized
    const tasks = await storage.listTasksAcrossProjects({ project: 'test-project' });
    const task = tasks.find(t => t.action.includes('Malicious task'));
    expect(task.action).not.toContain('<script>');
  });
});
```

## Deployment Instructions

### 1. Prerequisites

- Node.js v14 or higher
- npm or yarn package manager
- Git (for version control)
- System with at least 2GB RAM

### 2. Installation

#### Using npm
```bash
# Clone the repository
git clone https://github.com/your-org/todozi.git
cd todozi

# Install dependencies
npm install

# Build the application
npm run build

# Install globally
npm install -g .
```

#### Using yarn
```bash
# Clone the repository
git clone https://github.com/your-org/todozi.git
cd todozi

# Install dependencies
yarn install

# Build the application
yarn build

# Install globally
yarn global add .
```

### 3. Configuration

#### Environment Variables
Create a `.env` file in the project root:
```env
TODOZI_STORAGE_PATH=/var/lib/todozi
TODOZI_LOG_LEVEL=info
TODOZI_API_PORT=8636
TODOZI_MAX_CONCURRENT_TASKS=100
TODOZI_EMBEDDING_MODEL=sentence-transformers/all-MiniLM-L6-v2
```

#### Configuration File
Create `config/todozi.json`:
```json
{
  "storage": {
    "type": "file",
    "path": "/var/lib/todozi/data"
  },
  "server": {
    "host": "0.0.0.0",
    "port": 8636,
    "cors": true
  },
  "ai": {
    "embeddingModel": "sentence-transformers/all-MiniLM-L6-v2",
    "maxTokens": 4096,
    "temperature": 0.7
  },
  "security": {
    "apiKeyTTL": 86400,
    "rateLimit": {
      "requestsPerMinute": 100,
      "burstLimit": 200
    }
  }
}
```

### 4. Docker Deployment

#### Dockerfile
```dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 8636

VOLUME ["/var/lib/todozi"]

CMD ["node", "dist/index.js", "server", "start"]
```

#### Docker Compose
```yaml
version: '3.8'

services:
  todozi:
    build: .
    ports:
      - "8636:8636"
    volumes:
      - todozi_data:/var/lib/todozi
    environment:
      - TODOZI_STORAGE_PATH=/var/lib/todozi
      - TODOZI_LOG_LEVEL=info

volumes:
  todozi_data:
```

### 5. Kubernetes Deployment

#### Deployment YAML
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: todozi
spec:
  replicas: 3
  selector:
    matchLabels:
      app: todozi
  template:
    metadata:
      labels:
        app: todozi
    spec:
      containers:
      - name: todozi
        image: your-registry/todozi:latest
        ports:
        - containerPort: 8636
        env:
        - name: TODOZI_STORAGE_PATH
          value: /var/lib/todozi
        volumeMounts:
        - name: todozi-storage
          mountPath: /var/lib/todozi
      volumes:
      - name: todozi-storage
        persistentVolumeClaim:
          claimName: todozi-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: todozi-service
spec:
  selector:
    app: todozi
  ports:
  - port: 8636
    targetPort: 8636
  type: LoadBalancer
```

### 6. Monitoring and Logging

#### Prometheus Metrics
```javascript
const client = require('prom-client');

const taskCreationCounter = new client.Counter({
  name: 'todozi_task_creations_total',
  help: 'Total number of task creations'
});

const activeTasksGauge = new client.Gauge({
  name: 'todozi_active_tasks',
  help: 'Number of active tasks'
});
```

#### Health Checks
```bash
# Health check endpoint
curl http://localhost:8636/health

# Stats endpoint
curl http://localhost:8636/stats
```

## Troubleshooting Guide

### 1. Common Issues and Solutions

#### Issue: "Module not found" errors
**Solution:**
```bash
# Clear npm cache
npm cache clean --force

# Remove node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

#### Issue: Storage path permissions
**Solution:**
```bash
# Check permissions
ls -la /var/lib/todozi

# Fix permissions
sudo chown -R $USER:$USER /var/lib/todozi
sudo chmod -R 755 /var/lib/todozi
```

#### Issue: Port already in use
**Solution:**
```bash
# Check which process is using the port
lsof -i :8636

# Kill the process
kill -9 <PID>

# Or start on different port
todozi server start --port 8637
```

### 2. Performance Issues

#### Slow Task Creation
**Diagnosis:**
```bash
# Check system resources
top
df -h
free -m

# Check logs for errors
tail -f /var/log/todozi.log
```

**Solutions:**
1. Increase system resources
2. Optimize storage configuration
3. Enable caching
4. Check for database indexing issues

#### High Memory Usage
**Diagnosis:**
```bash
# Monitor memory usage
ps aux | grep todozi
```

**Solutions:**
1. Implement memory limits in configuration
2. Optimize data structures
3. Enable garbage collection tuning
4. Use streaming for large data operations

### 3. Security Issues

#### API Key Compromise
**Solution:**
```bash
# Deactivate compromised key
todozi api deactivate --user-id <user-id>

# Generate new key
todozi api register --user-id <user-id>

# Audit key usage
todozi api list --active-only
```

#### Unauthorized Access Attempts
**Solution:**
1. Check logs for suspicious activity
2. Implement IP whitelisting
3. Enable rate limiting
4. Review API key permissions

### 4. Data Integrity Issues

#### Corrupted Task Data
**Solution:**
```bash
# Run consistency check
todozi fix-task-consistency

# Restore from backup if needed
todozi restore-backup <backup-name>

# Validate data integrity
todozi stats
```

#### Missing Dependencies
**Solution:**
```bash
# Check for missing dependencies
npm audit

# Update dependencies
npm update

# Reinstall dependencies
npm install
```

### 5. Logging and Debugging

#### Enable Debug Mode
```bash
# Set environment variable
export TODOZI_LOG_LEVEL=debug

# Or use command flag
todozi --debug <command>
```

#### Log Analysis
```bash
# View recent logs
tail -f /var/log/todozi.log

# Search for specific errors
grep "ERROR" /var/log/todozi.log

# Analyze performance logs
grep "PERFORMANCE" /var/log/todozi.log
```

### 6. Backup and Recovery

#### Creating Backups
```bash
# Manual backup
todozi backup create

# Automated backup script
#!/bin/bash
todozi backup create --name "backup-$(date +%Y%m%d-%H%M%S)"
```

#### Recovery Process
```bash
# List available backups
todozi backup list

# Restore from backup
todozi restore-backup <backup-name>
```

This comprehensive documentation provides a complete overview of the Todozi Handler system, covering all aspects from architecture and API usage to deployment and troubleshooting. The system is designed to be robust, scalable, and secure while providing powerful task management and AI-powered features.