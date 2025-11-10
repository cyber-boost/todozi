on
{
  "action": "Create API documentation for user management",
  "priority": "high",
  "status": "todo",
  "project": "backend",
  "tags": ["api", "documentation", "backend"]
}

/*
// Process meeting notes with embedded TDZ commands
const meetingNotes = `
Action items from today's meeting:
- Fix authentication bug <tdz>create;task;action=Fix authentication bug;priority=critical;project=security</tdz>
- Update documentation <tdz>create;task;action=Update API documentation;priority=medium;project=docs</tdz>
`;

/ *
// In a chat application
const userMessage = "I just finished testing the login feature <tdz>update;task;login-test-456;status=done;progress=100</tdz>";
const results = await processTdzCommands(userMessage, API_URL, API_KEY);

/ *
// Example: Using TDZ commands to create, list, and manage tasks
import { processTdzCommands } from './tdz.js';

// Configuration
const API_BASE_URL = 'http://localhost:8636';
const API_KEY = 'your-api-key-here';

// Sample text containing TDZ commands
const taskManagementText = `
I need to organize my work for today:

<t dz>create;task;action=Create API documentation for user management;priority=high;status=todo;project=backend;tags=api,documentation,backend</tdz>

Also, I should review the existing tasks:

<t dz>list;tasks</tdz>

And check on that high-priority bug:

<t dz>get;tasks;bug-fix-123</tdz>

After completing the documentation:

<t dz>update;task;bug-fix-123;status=done;progress=100</tdz>
`;

// Process the TDZ commands
async function manageTasks() {
    try {
        console.log('🔄 Processing TDZ commands...');
        
        const results = await processTdzCommands(
            taskManagementText, 
            API_BASE_URL, 
            API_KEY
        );
        
        console.log('✅ Command processing completed!');
        console.log('📊 Results:', results);
        
    } catch (error) {
        console.error('❌ Error processing commands:', error.message);
    }
}

// Execute the example
manageTasks();

/ *
# Example 1: Using Todozi TDZ Commands for Task Management

This example demonstrates how to use the Todozi TDZ command system to manage tasks through structured API calls.

## Code Example

## Expected API Calls Generated

The above code will generate the following HTTP requests:

1. **POST /tasks** - Create new task

2. **GET /tasks** - List all tasks

3. **GET /tasks/bug-fix-123** - Get specific task

4. **PUT /tasks/bug-fix-123** - Update task status
{
  "status": "done",
  "progress": 100
}

## Key Features Demonstrated

### 1. **Command Parsing**
- Extracts TDZ commands from text using regex pattern
- Splits commands into components: command, target, parameters, options
- Automatically converts to lowercase for consistency

### 2. **Endpoint Mapping**
- Maps natural language commands to specific API endpoints
- `create;task` → `POST /tasks`
- `list;tasks` → `GET /tasks`
- `get;tasks;{id}` → `GET /tasks/{id}`

### 3. **Request Body Building**
- Converts command options into proper JSON request bodies
- Handles parameter types (arrays for tags, numbers for progress)
- Sets appropriate HTTP methods and headers

### 4. **Error Handling**
- Comprehensive error handling for HTTP errors and JSON parsing
- Clear error messages with status codes and response text

## Practical Usage Scenarios

### 1. **Chat Integration**

### 2. **Document Processing**

### 3. **CLI Integration**
*/