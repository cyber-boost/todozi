# Todozi Core Module Documentation

## Overview

The Todozi Core module is a comprehensive task management and data processing system that provides structured parsing, execution, and management of various types of content including tasks, memories, ideas, errors, and more. It uses a custom XML-like markup language to define structured content within text messages.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Todozi Core Module                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────────┐ │
│  │   Enums &       │    │   Helper        │    │   Content Parsers       │ │
│  │   Constants     │    │   Functions     │    │   (parse*, process*)    │ │
│  │                 │    │                 │    │                         │ │
│  │ Priority        │    │ transformShorthandTags │                     │ │
│  │ Status          │    │ parseEnum       │    │ parseTodoziFormat   │ │
│  │ Assignee        │    │                 │    │ parseMemoryFormat   │ │
│  │ MemoryImportance│    └─────────────────┘    │ parseIdeaFormat     │ │
│  │ MemoryTerm      │                           │ parseErrorFormat    │ │
│  │ MemoryType      │    ┌─────────────────┐    │ parseTrainingDataFormat│ │
│  │ ItemStatus      │    │   Execution     │    │ parseFeelingFormat  │ │
│  │ ShareLevel      │    │   Functions     │    │ processChatMessage  │ │
│  │ IdeaImportance  │    │                 │    │ processChatMessageExtended│ │
│  │ AssignmentStatus│    │ executeTask     │    │ processJsonExamples │ │
│  │ ErrorSeverity   │    │ executeAiTask   │    └─────────────────────────┘ │
│  │ ErrorCategory   │    │ executeHumanTask│                              │
│  │ TrainingDataType│    │ executeCollaborativeTask│                       │
│  └─────────────────┘    │ executeAgentTask│    ┌─────────────────────────┐ │
│                         │ processWorkflow │    │   Custom Error Class    │ │
│                         └─────────────────┘    │                         │ │
│                                                │ TodoziError             │ │
│                                                └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Custom Error Class

#### TodoziError
A custom error class that extends the native JavaScript Error class to provide typed error handling.

```javascript
class TodoziError extends Error {
    constructor(message, type = 'ValidationError') {
        super(message);
        this.name = 'TodoziError';
        this.type = type;
    }

    static validation(message) {
        return new TodoziError(message, 'ValidationError');
    }

    static storage(message) {
        return new TodoziError(message, 'StorageError');
    }
}
```

**Parameters:**
- `message` (string): Error message
- `type` (string, optional): Error type (default: 'ValidationError')

**Static Methods:**
- `validation(message)`: Creates a validation error
- `storage(message)`: Creates a storage error

### 2. Enums and Constants

#### Priority
Task priority levels:
- `Low`: 'low'
- `Medium`: 'medium'
- `High`: 'high'
- `Critical`: 'critical'

#### Status
Task status values:
- `Todo`: 'todo'
- `InProgress`: 'in_progress'
- `Blocked`: 'blocked'
- `Done`: 'done'
- `Deferred`: 'deferred'

#### Assignee
Task assignee types:
- `Ai`: 'ai'
- `Human`: 'human'
- `Collaborative`: 'collaborative'
- `Agent(name)`: Function returning agent object

#### MemoryImportance
Memory importance levels:
- `Low`: 'low'
- `Medium`: 'medium'
- `High`: 'high'
- `Critical`: 'critical'

#### MemoryTerm
Memory term duration:
- `Short`: 'short'
- `Long`: 'long'

#### MemoryType
Memory types:
- `Standard`: 'standard'
- `Secret`: 'secret'
- `Human`: 'human'
- `Short`: 'short'
- `Long`: 'long'
- `Emotional(emotion)`: Function returning emotional memory object

#### ItemStatus
General item status:
- `Active`: 'active'
- `Archived`: 'archived'
- `Deleted`: 'deleted'

#### ShareLevel
Content sharing levels:
- `Public`: 'public'
- `Private`: 'private'
- `Team`: 'team'

#### IdeaImportance
Idea importance levels:
- `Low`: 'low'
- `Medium`: 'medium'
- `High`: 'high'
- `Critical`: 'critical'

#### AssignmentStatus
Task assignment status:
- `Assigned`: 'assigned'
- `InProgress`: 'in_progress'
- `Completed`: 'completed'
- `Failed`: 'failed'

#### ErrorSeverity
Error severity levels:
- `Low`: 'low'
- `Medium`: 'medium'
- `High`: 'high'
- `Critical`: 'critical'

#### ErrorCategory
Error categories:
- `Network`: 'network'
- `Database`: 'database'
- `Authentication`: 'authentication'
- `Validation`: 'validation'
- `Logic`: 'logic'
- `UI`: 'ui'
- `API`: 'api'
- `System`: 'system'
- `Other`: 'other'

#### TrainingDataType
Training data types:
- `Instruction`: 'instruction'
- `Question`: 'question'
- `Conversation`: 'conversation'
- `Code`: 'code'
- `Documentation`: 'documentation'

### 3. Helper Functions

#### parseEnum(value, enumObj, fieldName)
Validates that a value exists in an enum object.

**Parameters:**
- `value` (string): Value to validate
- `enumObj` (object): Enum object to check against
- `fieldName` (string): Name of the field for error messages

**Returns:** Validated enum value

**Throws:** TodoziError if value is not found in enum

#### transformShorthandTags(message)
Transforms shorthand XML tags to full tags.

**Parameters:**
- `message` (string): Input message with shorthand tags

**Returns:** Transformed message with full tags

**Supported Shorthand Mappings:**
- `<tz>` → `<todozi>`
- `<mm>` → `<memory>`
- `<id>` → `<idea>`
- `<ch>` → `<chunk>`
- `<fe>` → `<feel>`
- `<tn>` → `<train>`
- `<er>` → `<error>`
- `<sm>` → `<summary>`
- `<rd>` → `<reminder>`
- `<tdz>` → `<tdz>`

### 4. Content Parsers

#### parseTodoziFormat(todoziText)
Parses todozi format text into a structured task object.

**Parameters:**
- `todoziText` (string): Text containing `<todozi>...</todozi>` tags

**Returns:** Task object with the following structure:
```javascript
{
    id: string,
    user_id: string,
    action: string,
    time: string,
    priority: string,
    parent_project: string,
    status: string,
    assignee: object|string|null,
    tags: array,
    dependencies: array,
    context_notes: string|null,
    progress: number|null,
    embedding_vector: null,
    created_at: string,
    updated_at: string
}
```

**Format:** `<todozi>action; time; priority; parent_project; status; [assignee]; [tags]; [dependencies]; [context_notes]; [progress]</todozi>`

#### parseMemoryFormat(memoryText, userId)
Parses memory format text into a structured memory object.

**Parameters:**
- `memoryText` (string): Text containing `<memory>...</memory>` tags
- `userId` (string): User ID for the memory

**Returns:** Memory object with the following structure:
```javascript
{
    id: string,
    user_id: string,
    project_id: null,
    status: string,
    moment: string,
    meaning: string,
    reason: string,
    importance: string,
    term: string,
    memory_type: object|string,
    tags: array,
    created_at: string,
    updated_at: string
}
```

**Format:** `<memory>type; moment; meaning; reason; importance; term; [tags]</memory>`

#### parseIdeaFormat(ideaText)
Parses idea format text into a structured idea object.

**Parameters:**
- `ideaText` (string): Text containing `<idea>...</idea>` tags

**Returns:** Idea object with the following structure:
```javascript
{
    id: string,
    idea: string,
    project_id: null,
    status: string,
    share: string,
    importance: string,
    tags: array,
    context: null,
    created_at: string,
    updated_at: string
}
```

**Format:** `<idea>idea; share; importance</idea>`

#### parseAgentAssignmentFormat(agentText)
Parses agent assignment format text into a structured assignment object.

**Parameters:**
- `agentText` (string): Text containing `<todozi_agent>...</todozi_agent>` tags

**Returns:** Agent assignment object with the following structure:
```javascript
{
    agent_id: string,
    task_id: string,
    project_id: string,
    assigned_at: string,
    status: string
}
```

**Format:** `<todozi_agent>agent_id; task_id; project_id</todozi_agent>`

#### parseErrorFormat(errorText)
Parses error format text into a structured error object.

**Parameters:**
- `errorText` (string): Text containing `<error>...</error>` tags

**Returns:** Error object with the following structure:
```javascript
{
    id: string,
    title: string,
    description: string,
    severity: string,
    category: string,
    source: string,
    context: string|null,
    tags: array,
    resolved: boolean,
    resolution: null,
    created_at: string,
    updated_at: string,
    resolved_at: null
}
```

**Format:** `<error>title; description; severity; category; source; [context]; [tags]</error>`

#### parseTrainingDataFormat(trainText)
Parses training data format text into a structured training data object.

**Parameters:**
- `trainText` (string): Text containing `<train>...</train>` tags

**Returns:** Training data object with the following structure:
```javascript
{
    id: string,
    data_type: string,
    prompt: string,
    completion: string,
    context: string|null,
    tags: array,
    quality_score: number|null,
    source: string,
    created_at: string,
    updated_at: string
}
```

**Format:** `<train>data_type; prompt; completion; context; [tags]; [quality_score]; [source]</train>`

#### parseFeelingFormat(feelText)
Parses feeling format text into a structured feeling object.

**Parameters:**
- `feelText` (string): Text containing `<feel>...</feel>` tags

**Returns:** Feeling object with the following structure:
```javascript
{
    id: string,
    emotion: string,
    intensity: number,
    description: string,
    context: string,
    tags: array,
    created_at: string,
    updated_at: string
}
```

**Format:** `<feel>emotion; intensity; description; [context]; [tags]</feel>`

### 5. Message Processors

#### processChatMessage(message)
Extracts and parses todozi tasks from a chat message.

**Parameters:**
- `message` (string): Chat message potentially containing todozi tasks

**Returns:** Array of parsed task objects

#### processChatMessageExtended(message, userId)
Extracts and parses all types of content from a chat message.

**Parameters:**
- `message` (string): Chat message potentially containing various content types
- `userId` (string): User ID for memory creation

**Returns:** Object containing arrays of parsed content:
```javascript
{
    tasks: array,
    memories: array,
    ideas: array,
    agent_assignments: array,
    code_chunks: array,
    errors: array,
    training_data: array,
    feelings: array,
    summaries: array,
    reminders: array
}
```

#### processJsonExamples(jsonData)
Parses todozi tasks from JSON examples.

**Parameters:**
- `jsonData` (string): JSON string containing examples with todozi_format

**Returns:** Array of parsed task objects

### 6. Task Execution Functions

#### executeTask(storage, task)
Executes a task based on its assignee.

**Parameters:**
- `storage` (object): Storage instance
- `task` (object): Task object to execute

**Returns:** Execution result message

#### executeAiTask(task)
Queues an AI task for processing.

**Parameters:**
- `task` (object): Task object

**Returns:** Success message with queue ID

#### executeHumanTask(task)
Queues a human task for the TUI.

**Parameters:**
- `task` (object): Task object

**Returns:** Success message with queue ID

#### executeCollaborativeTask(task)
Queues both AI and human portions of a collaborative task.

**Parameters:**
- `task` (object): Task object

**Returns:** Success message with both queue IDs

#### executeAgentTask(task, agentName)
Assigns a task to a specific agent.

**Parameters:**
- `task` (object): Task object
- `agentName` (string): Name of the agent

**Returns:** Success message with assignment details

#### processWorkflow(tasks)
Processes a workflow of tasks sequentially.

**Parameters:**
- `tasks` (array): Array of task objects

**Returns:** Array of execution results

## Usage Examples

### 1. Basic Task Parsing
```javascript
const { parseTodoziFormat } = require('./todozi');

const taskText = "<todozi>Complete documentation; tomorrow; high; project1; todo; ai; documentation,important; task123; Need detailed examples; 0</todozi>";
const task = parseTodoziFormat(taskText);
console.log(task);
```

### 2. Processing Chat Messages
```javascript
const { processChatMessageExtended } = require('./todozi');

const message = "Let's create a new task <tz>Write tests; today; medium; project2; todo</tz> and remember <mm>happy; coding session; productive work; motivation; high; long</mm>";
const content = processChatMessageExtended(message, "user123");
console.log(content.tasks);
console.log(content.memories);
```

### 3. Task Execution
```javascript
const { parseTodoziFormat, executeTask } = require('./todozi');
const storage = require('./storage');

const taskText = "<todozi>Generate report; next week; critical; analytics; todo; ai</todozi>";
const task = parseTodoziFormat(taskText);

executeTask(storage, task)
    .then(result => console.log(result))
    .catch(error => console.error(error));
```

### 4. Workflow Processing
```javascript
const { processChatMessage, processWorkflow } = require('./todozi');

const message = "Tasks: <todozi>Task 1; today; high; proj1; todo</todozi> <todozi>Task 2; tomorrow; medium; proj1; todo</todozi>";
const tasks = processChatMessage(message);

processWorkflow(tasks)
    .then(results => console.log(results))
    .catch(error => console.error(error));
```

### 5. Error Handling
```javascript
const { TodoziError, parseTodoziFormat } = require('./todozi');

try {
    const invalidTask = parseTodoziFormat("<todozi>Invalid task</todozi>");
} catch (error) {
    if (error instanceof TodoziError) {
        console.log(`Todozi Error (${error.type}): ${error.message}`);
    } else {
        console.log(`Unexpected error: ${error.message}`);
    }
}
```

## Design Patterns

### 1. Factory Pattern
The `Assignee.Agent(name)` function uses the factory pattern to create agent objects with consistent structure.

### 2. Strategy Pattern
The task execution functions (`executeAiTask`, `executeHumanTask`, etc.) implement different strategies for handling tasks based on their assignee.

### 3. Facade Pattern
The `processChatMessageExtended` function provides a simplified interface for parsing multiple content types from a single message.

### 4. Error Handling Pattern
Custom `TodoziError` class with static factory methods provides consistent error handling across the module.

## Performance Analysis

### Time Complexity
- **Parsing Functions**: O(n) where n is the length of the input text
- **Task Execution**: O(1) for simple execution, O(m) for collaborative tasks where m is the number of sub-tasks
- **Message Processing**: O(n + k) where n is message length and k is the number of content items

### Space Complexity
- **Parsing Functions**: O(1) additional space (not counting output objects)
- **Message Processing**: O(k) where k is the number of parsed content items

### Optimization Considerations
1. Regular expression compilation in `transformShorthandTags` could be cached
2. Enum validation in `parseEnum` could use a Set for O(1) lookup instead of O(n)
3. Message processing could be parallelized for better performance with large messages

## Security Considerations

### 1. Input Validation
- All parsed content is validated against predefined enums
- XML-like tags are properly escaped to prevent injection
- Numeric values are parsed and validated

### 2. Error Handling
- Custom error types prevent information leakage
- Storage errors are wrapped to hide implementation details

### 3. Data Sanitization
- User input is sanitized through parsing functions
- Tags and content are trimmed and validated

### 4. Access Control
- User IDs are required for memory creation
- Content sharing levels control visibility

## Testing Strategies

### Unit Tests
```javascript
// Test enum parsing
test('parseEnum validates correct values', () => {
    expect(parseEnum('high', Priority, 'priority')).toBe('high');
});

// Test todozi format parsing
test('parseTodoziFormat handles valid input', () => {
    const input = "<todozi>Test task; today; high; proj1; todo</todozi>";
    const result = parseTodoziFormat(input);
    expect(result.action).toBe('Test task');
    expect(result.priority).toBe('high');
});

// Test error handling
test('parseTodoziFormat throws on invalid input', () => {
    expect(() => parseTodoziFormat("invalid")).toThrow(TodoziError);
});
```

### Integration Tests
```javascript
// Test message processing
test('processChatMessageExtended handles multiple content types', () => {
    const message = "<todozi>Task; today; high; proj; todo</todozi> <memory>happy; moment; meaning; reason; high; long</memory>";
    const result = processChatMessageExtended(message, "user123");
    expect(result.tasks.length).toBe(1);
    expect(result.memories.length).toBe(1);
});
```

### Edge Case Tests
- Empty content tags
- Malformed XML structures
- Invalid enum values
- Missing required fields
- Excessive tag content

## Deployment Instructions

### 1. Dependencies
```bash
npm install uuid luxon
```

### 2. Required Modules
Ensure the following modules are available:
- `./storage` (for task storage operations)
- `./chunking` (for code chunk parsing)
- `./summary` (for summary parsing)
- `./reminder` (for reminder parsing)

### 3. Environment Setup
```javascript
// Set timezone for consistent date handling
process.env.TZ = 'UTC';
```

### 4. Integration
```javascript
const todozi = require('./todozi');

// Use in your application
const tasks = todozi.processChatMessage(userMessage);
```

## Troubleshooting Guide

### Common Issues

#### 1. "Missing start tag" Errors
**Problem:** Content parsing fails with missing tag errors
**Solution:** Ensure all content is properly wrapped in appropriate tags
```javascript
// Correct
const valid = "<todozi>Task; today; high; proj; todo</todozi>";

// Incorrect
const invalid = "Task; today; high; proj; todo";
```

#### 2. Enum Validation Failures
**Problem:** "Invalid [field]: [value]" errors
**Solution:** Check that values match predefined enum constants
```javascript
// Valid priorities: 'low', 'medium', 'high', 'critical'
const task = "<todozi>Task; today; HIGH; proj; todo</todozi>"; // Invalid - case sensitive
```

#### 3. Storage Errors
**Problem:** "Failed to queue task" errors
**Solution:** Verify storage module is properly configured and accessible

#### 4. Performance Issues with Large Messages
**Problem:** Slow processing of messages with many content items
**Solution:** Consider breaking large messages into smaller chunks or implementing streaming parsing

### Debugging Tips

1. **Enable verbose logging:**
```javascript
process.env.DEBUG = 'todozi:*';
```

2. **Check parsed content structure:**
```javascript
const content = processChatMessageExtended(message, userId);
console.log(JSON.stringify(content, null, 2));
```

3. **Validate individual components:**
```javascript
try {
    const task = parseTodoziFormat(taskText);
    console.log('Parsed successfully:', task);
} catch (error) {
    console.log('Parsing failed:', error.message);
}
```

## Version History

### v1.0.0
- Initial implementation
- Support for todozi, memory, idea, error, training data, and feeling formats
- Basic task execution functionality
- Message processing capabilities

### v1.1.0
- Added agent assignment support
- Enhanced error handling with custom error types
- Improved performance optimizations
- Extended testing coverage

## Contributing

### Code Style
- Follow JavaScript standard style
- Use descriptive variable and function names
- Include JSDoc comments for all public functions
- Maintain consistent error handling patterns

### Testing Requirements
- All new functions must include unit tests
- Integration tests required for parsing functions
- Edge cases must be covered
- Performance benchmarks for critical functions

### Documentation Updates
- Update this documentation with any new features
- Include usage examples for new functionality
- Maintain consistent formatting and structure