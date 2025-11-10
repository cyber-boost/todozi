# Todozi Core Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

Todozi is a comprehensive task management and AI-assisted productivity system built in JavaScript. It provides a structured approach to managing tasks, projects, and AI agents with local storage capabilities.

### Key Features
- Project-based task organization
- AI agent integration
- Local storage with structured directory system
- Configuration management
- Registration and authentication support
- Task lifecycle management (active, completed, archived)

## Architecture

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    Todozi Application                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Storage   │  │   Models    │  │   Helpers   │         │
│  │             │  │             │  │             │         │
│  │  Storage    │  │  Config     │  │  Hlx        │         │
│  │  Task       │  │  Project    │  │  Utilities  │         │
│  │  Project    │  │  Task       │  │             │         │
│  │  Agent      │  │  Agent      │  │             │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                    File System Layer                        │
├─────────────────────────────────────────────────────────────┤
│                   Operating System                          │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow Architecture

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   User      │───▶│  Todozi     │───▶│  Storage    │
│  Actions    │    │  Core       │    │  System     │
└─────────────┘    └─────────────┘    └─────────────┘
                        │                    │
                        ▼                    ▼
                   ┌─────────────┐    ┌─────────────┐
                   │  Models     │    │  Files      │
                   │  (Task,     │    │  (.json,    │
                   │   Project,  │    │   .hlx)     │
                   │   Agent)    │    └─────────────┘
                   └─────────────┘
```

## Core Components

### 1. Storage System

The storage system manages all data persistence for Todozi, organizing data into structured directories.

#### Directory Structure
```
~/.todozi/
├── agents/           # AI agent configurations
├── api/              # API related data
├── assignments/      # Agent task assignments
├── backups/          # Backup files
├── chunks/           # Code chunks
├── embed/            # Embedding data
├── errors/           # Error logs
├── feelings/         # User feeling data
├── ideas/            # Idea storage
├── memories/         # Memory data
├── models/           # AI models
├── projects/         # Project definitions
├── queue/            # Task queue
├── responses/        # AI responses
├── tasks/            # Task collections
├── templates/        # Templates
├── training/         # Training data
└── tdz.hlx           # Main configuration
```

### 2. Model Classes

#### Config Class
Manages application configuration settings.

```javascript
class Config {
    constructor() {
        this.registration = null;
        this.version = "1.2.0";
        this.default_project = "general";
        this.auto_backup = true;
        this.backup_interval = "daily";
        this.ai_enabled = true;
        this.default_assignee = null;
        this.date_format = "%Y-%m-%d %H:%M:%S";
        this.timezone = "UTC";
    }
}
```

#### RegistrationInfo Class
Handles user registration and authentication information.

```javascript
class RegistrationInfo {
    constructor(serverUrl = "https://todozi.com") {
        this.user_name = "user";
        this.user_email = "user@example.com";
        this.api_key = "";
        this.user_id = null;
        this.fingerprint = null;
        this.registered_at = new Date();
        this.server_url = serverUrl;
    }
}
```

#### Project Class
Represents a project container for tasks.

```javascript
class Project {
    constructor(name, description) {
        this.name = name;
        this.description = description;
    }
}
```

#### Task Class
Represents an individual task with metadata.

```javascript
class Task {
    constructor() {
        this.id = uuidv4();
        this.action = "";
        this.parent_project = "";
        this.status = "todo";
        this.priority = "normal";
        this.context_notes = null;
        this.created_at = new Date();
        this.updated_at = new Date();
        this.embedding_vector = null;
    }
}
```

#### Agent Class
Represents an AI agent with specific capabilities.

```javascript
class Agent {
    constructor(id, name, description) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.system_prompt = "";
        this.prompt_template = null;
        this.capabilities = [];
        this.specializations = [];
        this.tools = [];
        this.metadata = {
            tags: [],
            category: "general",
            author: null,
            status: "available"
        };
        this.behaviors = {
            auto_format_code: false,
            include_examples: true,
            explain_complexity: true,
            suggest_tests: false
        };
        this.constraints = {
            max_response_length: null
        };
    }
}
```

## API Reference

### Storage Functions

#### `getStorageDir()`
Returns the path to the Todozi storage directory.

**Returns:** `Promise<string>` - Path to storage directory

**Example:**
```javascript
const storageDir = await getStorageDir();
console.log(storageDir); // ~/.todozi
```

#### `initStorage()`
Initializes the Todozi storage system, creating required directories and default files.

**Returns:** `Promise<void>`

**Example:**
```javascript
await initStorage();
console.log('Storage initialized successfully');
```

#### `checkFolderStructure()`
Verifies that the required folder structure exists.

**Returns:** `Promise<boolean>` - True if structure is complete

**Example:**
```javascript
const isValid = await checkFolderStructure();
if (isValid) {
    console.log('Folder structure is valid');
}
```

#### `ensureFolderStructure()`
Ensures the folder structure exists, creating it if necessary.

**Returns:** `Promise<boolean>` - True if structure is valid or was created successfully

**Example:**
```javascript
const result = await ensureFolderStructure();
console.log('Folder structure ready:', result);
```

### Configuration Functions

#### `saveConfig(config)`
Saves the configuration to the tdz.hlx file.

**Parameters:**
- `config` (Config): Configuration object to save

**Returns:** `Promise<void>`

**Example:**
```javascript
const config = Config.default();
config.default_project = 'work';
await saveConfig(config);
```

#### `loadConfig()`
Loads the configuration from the tdz.hlx file.

**Returns:** `Promise<Config>` - Loaded configuration object

**Example:**
```javascript
const config = await loadConfig();
console.log('Default project:', config.default_project);
```

### Project Functions

#### `saveProject(project)`
Saves a project to the projects directory.

**Parameters:**
- `project` (Project): Project object to save

**Returns:** `Promise<void>`

**Example:**
```javascript
const project = Project.new('work', 'Work-related tasks');
await saveProject(project);
```

#### `loadProject(projectName)`
Loads a project from the projects directory.

**Parameters:**
- `projectName` (string): Name of the project to load

**Returns:** `Promise<Project>` - Loaded project object

**Example:**
```javascript
const project = await loadProject('work');
console.log('Project description:', project.description);
```

#### `listProjects()`
Lists all available projects.

**Returns:** `Promise<Project[]>` - Array of project objects

**Example:**
```javascript
const projects = await listProjects();
projects.forEach(project => {
    console.log(`${project.name}: ${project.description}`);
});
```

### Task Functions

#### `saveTaskCollection(collectionName, collection)`
Saves a task collection to the tasks directory.

**Parameters:**
- `collectionName` (string): Name of the collection
- `collection` (TaskCollection): Task collection to save

**Returns:** `Promise<void>`

**Example:**
```javascript
const collection = TaskCollection.new();
await saveTaskCollection('active', collection);
```

#### `loadTaskCollection(collectionName)`
Loads a task collection from the tasks directory.

**Parameters:**
- `collectionName` (string): Name of the collection to load

**Returns:** `Promise<TaskCollection>` - Loaded task collection

**Example:**
```javascript
const collection = await loadTaskCollection('active');
console.log('Tasks count:', Object.keys(collection.tasks).length);
```

### Agent Functions

#### `createDefaultAgents()`
Creates default AI agents and saves them to the agents directory.

**Returns:** `Promise<void>`

**Example:**
```javascript
await createDefaultAgents();
console.log('Default agents created');
```

### Storage Class

#### `Storage.new()`
Creates a new Storage instance with loaded configuration.

**Returns:** `Promise<Storage>` - New Storage instance

**Example:**
```javascript
const storage = await Storage.new();
console.log('Storage initialized');
```

#### `storage.config()`
Returns the current configuration.

**Returns:** `Config` - Current configuration object

**Example:**
```javascript
const storage = await Storage.new();
const config = storage.config();
console.log('Version:', config.version);
```

#### `storage.updateConfig(config)`
Updates the configuration and saves it.

**Parameters:**
- `config` (Config): New configuration object

**Returns:** `Promise<void>`

**Example:**
```javascript
const storage = await Storage.new();
const config = storage.config();
config.default_project = 'personal';
await storage.updateConfig(config);
```

#### `storage.addTaskToProject(task)`
Adds a task to its parent project.

**Parameters:**
- `task` (Task): Task to add

**Returns:** `Promise<void>`

**Example:**
```javascript
const storage = await Storage.new();
const task = new Task();
task.action = 'Complete documentation';
task.parent_project = 'work';
await storage.addTaskToProject(task);
```

#### `storage.getTaskFromAnyProject(id)`
Retrieves a task from any project by ID.

**Parameters:**
- `id` (string): Task ID

**Returns:** `Promise<Task>` - Found task object

**Throws:** `TodoziError` if task not found

**Example:**
```javascript
const storage = await Storage.new();
try {
    const task = await storage.getTaskFromAnyProject('task-123');
    console.log('Found task:', task.action);
} catch (error) {
    console.error('Task not found:', error.message);
}
```

## Usage Examples

### Basic Initialization

```javascript
import { initStorage, Storage } from './todozi';

// Initialize the storage system
await initStorage();

// Create a storage instance
const storage = await Storage.new();

// Access configuration
const config = storage.config();
console.log('Todozi version:', config.version);
```

### Creating and Managing Projects

```javascript
import { Project, saveProject, loadProject, listProjects } from './todozi';

// Create a new project
const workProject = Project.new('work', 'Work-related tasks and projects');
await saveProject(workProject);

// Load an existing project
const loadedProject = await loadProject('work');
console.log('Project loaded:', loadedProject.name);

// List all projects
const projects = await listProjects();
console.log('Available projects:', projects.map(p => p.name));
```

### Task Management

```javascript
import { Task, Storage } from './todozi';

// Create a storage instance
const storage = await Storage.new();

// Create a new task
const task = new Task();
task.action = 'Write comprehensive documentation';
task.parent_project = 'work';
task.priority = 'high';

// Add task to project
await storage.addTaskToProject(task);
console.log('Task created with ID:', task.id);

// Retrieve task
const retrievedTask = await storage.getTaskFromAnyProject(task.id);
console.log('Retrieved task:', retrievedTask.action);
```

### Configuration Management

```javascript
import { loadConfig, saveConfig } from './todozi';

// Load current configuration
const config = await loadConfig();

// Modify configuration
config.default_project = 'personal';
config.auto_backup = true;
config.backup_interval = 'weekly';

// Save updated configuration
await saveConfig(config);
console.log('Configuration updated');
```

### Agent Management

```javascript
import { createDefaultAgents, Agent } from './todozi';

// Create default agents
await createDefaultAgents();
console.log('Default agents created');

// Create a custom agent
const customAgent = Agent.new('researcher', 'Research Assistant', 'Research and analysis specialist');
customAgent.system_prompt = 'You are an expert researcher...';
customAgent.capabilities = ['data_analysis', 'literature_review'];

// Save custom agent (implementation needed)
// await saveAgent(customAgent);
```

## Design Patterns

### 1. Singleton Pattern
The Storage class implements a singleton pattern to ensure a single point of access to the configuration and data management system.

### 2. Factory Pattern
Agent creation functions like `createPlannerAgent()`, `createTesterAgent()`, etc., implement the factory pattern for consistent agent instantiation.

### 3. Repository Pattern
The storage system implements a repository pattern with dedicated functions for each entity type (projects, tasks, agents) to abstract data persistence operations.

### 4. Builder Pattern
The Config and RegistrationInfo classes use the builder pattern through their constructor and static factory methods for flexible object creation.

### 5. Decorator Pattern
The Hlx class provides a decorator-like interface for configuration management, wrapping file I/O operations with a structured API.

## Performance Analysis

### Storage Performance
- **File I/O**: Uses `fs/promises` for asynchronous operations to prevent blocking
- **JSON Serialization**: Efficient for small to medium datasets
- **Directory Structure**: Organized for quick file access and minimal search paths

### Memory Usage
- **Configuration Loading**: Single configuration load at startup
- **Task Collections**: Loaded on demand, not kept in memory continuously
- **Project Containers**: Individual project data loaded when accessed

### Scalability Considerations
- **Current Limitations**: JSON-based storage may not scale well for large datasets
- **Potential Improvements**: 
  - Database integration for better query performance
  - Caching layer for frequently accessed data
  - Indexing system for task search operations

### Performance Recommendations
1. Use project-specific containers to limit data loading
2. Implement lazy loading for large collections
3. Consider batch operations for multiple task updates
4. Monitor file sizes and implement archiving for old data

## Security Considerations

### Data Protection
- **Configuration Files**: Contains sensitive API keys and user information
- **File Permissions**: Should restrict access to user-only read/write
- **Data Encryption**: Consider encrypting sensitive configuration data

### Input Validation
- **Path Traversal**: All file paths should be validated to prevent directory traversal attacks
- **User Input**: Validate and sanitize all user-provided data before storage

### Authentication Security
- **API Keys**: Should be stored securely and rotated regularly
- **Registration Data**: Fingerprinting for device identification
- **Server Communication**: HTTPS required for all server interactions

### Security Best Practices
1. Implement proper file permission controls
2. Encrypt sensitive configuration data
3. Use secure random number generation for IDs and keys
4. Validate all input data
5. Implement proper error handling without exposing sensitive information

## Testing Strategies

### Unit Testing
```javascript
// Example test structure
describe('Storage', () => {
    beforeEach(async () => {
        // Setup test environment
        await initStorage();
    });

    test('should load default configuration', async () => {
        const config = await loadConfig();
        expect(config.version).toBe('1.2.0');
        expect(config.default_project).toBe('general');
    });

    test('should save and load project', async () => {
        const project = Project.new('test', 'Test project');
        await saveProject(project);
        
        const loadedProject = await loadProject('test');
        expect(loadedProject.name).toBe('test');
        expect(loadedProject.description).toBe('Test project');
    });
});
```

### Integration Testing
```javascript
describe('Task Management', () => {
    test('should add and retrieve task from project', async () => {
        const storage = await Storage.new();
        const task = new Task();
        task.action = 'Test task';
        task.parent_project = 'general';
        
        await storage.addTaskToProject(task);
        const retrievedTask = await storage.getTaskFromAnyProject(task.id);
        
        expect(retrievedTask.action).toBe('Test task');
        expect(retrievedTask.id).toBe(task.id);
    });
});
```

### Mock Testing
```javascript
// Mock file system operations for testing
jest.mock('fs/promises', () => ({
    readFile: jest.fn(),
    writeFile: jest.fn(),
    mkdir: jest.fn(),
    access: jest.fn()
}));
```

### Test Coverage Areas
1. **Storage Operations**: File creation, reading, and writing
2. **Configuration Management**: Loading and saving configuration
3. **Project Management**: Project creation, loading, and listing
4. **Task Operations**: Task creation, retrieval, and status updates
5. **Agent Management**: Agent creation and configuration
6. **Error Handling**: Proper error handling and validation

## Deployment Instructions

### Prerequisites
- Node.js version 14 or higher
- npm or yarn package manager
- Write access to user home directory

### Installation Steps

1. **Clone Repository**
```bash
git clone https://github.com/your-org/todozi.git
cd todozi
```

2. **Install Dependencies**
```bash
npm install
# or
yarn install
```

3. **Initialize Storage**
```bash
node -e "require('./todozi').initStorage()"
```

4. **Verify Installation**
```bash
node -e "require('./todozi').checkFolderStructure()"
```

### Configuration
1. The system automatically creates `~/.todozi` directory
2. Default configuration is created in `~/.todozi/tdz.hlx`
3. Modify configuration through the API or direct file editing

### Production Deployment
1. **Environment Setup**
```bash
# Set environment variables
export TODOZI_STORAGE_DIR=/path/to/storage
export NODE_ENV=production
```

2. **Security Hardening**
- Restrict file permissions: `chmod 700 ~/.todozi`
- Use dedicated user account for running Todozi
- Implement backup strategies for critical data

3. **Monitoring**
- Log file monitoring for errors
- Storage space monitoring
- Performance monitoring for slow operations

### Update Process
1. Backup current configuration and data
2. Pull latest code changes
3. Run migration scripts if necessary
4. Verify functionality after update

## Troubleshooting Guide

### Common Issues

#### 1. Storage Directory Not Found
**Symptom:** `Could not find home directory` error
**Solution:** 
- Verify user home directory exists
- Check file permissions
- Manually specify storage directory with environment variable

#### 2. Configuration File Corruption
**Symptom:** Application fails to start or behaves unexpectedly
**Solution:**
```bash
# Backup current config
cp ~/.todozi/tdz.hlx ~/.todozi/tdz.hlx.backup

# Recreate default configuration
node -e "require('./todozi').initStorage()"
```

#### 3. Permission Errors
**Symptom:** `EACCES` errors when accessing files
**Solution:**
```bash
# Fix permissions
chmod -R 700 ~/.todozi/
chown -R $USER:$USER ~/.todozi/
```

#### 4. Project Not Found
**Symptom:** `ProjectNotFound` error when loading project
**Solution:**
- Verify project file exists in `~/.todozi/projects/`
- Check file permissions
- Recreate project if necessary

### Debugging Steps

1. **Enable Verbose Logging**
```javascript
// Add logging to configuration
const config = await loadConfig();
config.debug = true;
await saveConfig(config);
```

2. **Check File System**
```bash
# Verify storage directory structure
ls -la ~/.todozi/
ls -la ~/.todozi/projects/
ls -la ~/.todozi/tasks/
```

3. **Validate JSON Files**
```bash
# Check configuration file validity
node -e "JSON.parse(require('fs').readFileSync('~/.todozi/tdz.hlx'))"
```

### Recovery Procedures

#### Data Recovery
1. **From Backups**
```bash
# Restore from backup if available
cp ~/.todozi/backups/latest.tar.gz ~/.todozi/
tar -xzf ~/.todozi/latest.tar.gz -C ~/.todozi/
```

2. **Configuration Recovery**
```bash
# Recreate default configuration
rm ~/.todozi/tdz.hlx
node -e "require('./todozi').initStorage()"
```

#### System Reset
If all else fails:
```bash
# Complete reset (WARNING: Deletes all data)
rm -rf ~/.todozi/
node -e "require('./todozi').initStorage()"
```

### Performance Issues

#### Slow Startup
- Check for large JSON files in storage directory
- Verify disk space availability
- Consider archiving old projects

#### Memory Issues
- Monitor Node.js memory usage
- Implement task collection batching
- Consider database migration for large datasets

### Contact Support
For persistent issues:
1. Collect error logs and system information
2. Provide steps to reproduce the issue
3. Include Todozi version and Node.js version
4. Contact support team with detailed information

---

This documentation provides a comprehensive overview of the Todozi system, covering all major components, APIs, and operational procedures. For specific implementation details of individual functions, refer to the inline code comments and examples provided throughout this document.