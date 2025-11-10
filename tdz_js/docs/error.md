# Todozi Error Management System - Comprehensive Documentation

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

The Todozi Error Management System is a comprehensive error handling framework designed for the Todozi task management application. It provides structured error definitions, centralized error management, and robust error parsing capabilities. The system ensures consistent error handling across the application while providing detailed error information for debugging and user feedback.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Todozi Error System                          │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────┐  │
│  │   TodoziError       │  │   ErrorManager      │  │ ErrorParser │  │
│  │   (Custom Error)    │  │   (Error Registry)  │  │ (Format)    │  │
│  └─────────────────────┘  └─────────────────────┘  └─────────────┘  │
│           │                         │                     │         │
│           │                         │                     │         │
│  ┌─────────────────────┐  ┌─────────────────────┐        │         │
│  │ Error Factories     │  │ Error CRUD Ops      │        │         │
│  │ (Static Methods)    │  │ (Create/Resolve)    │        │         │
│  └─────────────────────┘  └─────────────────────┘        │         │
│           │                         │                     │         │
│           └─────────────────────────┼─────────────────────┘         │
│                                     │                               │
│                        ┌─────────────────────┐                     │
│                        │   UUID Generator    │                     │
│                        └─────────────────────┘                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. TodoziError Class
A custom error class that extends JavaScript's native Error class with additional metadata for better error categorization and handling.

### 2. ErrorManager Class
A centralized error registry that manages error lifecycle including creation, storage, and resolution.

### 3. Error Parser Functions
Utility functions for parsing error information from formatted text strings.

## Detailed API Documentation

### TodoziError Class

#### Constructor
```javascript
new TodoziError(message, type, details)
```

**Parameters:**
- `message` (string): The error message
- `type` (string, optional): Error type/category. Defaults to 'General'
- `details` (object, optional): Additional error details. Defaults to null

**Properties:**
- `name` (string): Always 'TodoziError'
- `message` (string): The error message
- `type` (string): Error type/category
- `details` (object): Additional error details

#### Static Factory Methods

##### taskNotFound(id)
Creates a TaskNotFound error.

**Parameters:**
- `id` (string|number): The ID of the task that was not found

**Returns:** TodoziError instance

##### projectNotFound(name)
Creates a ProjectNotFound error.

**Parameters:**
- `name` (string): The name of the project that was not found

**Returns:** TodoziError instance

##### feelingNotFound(id)
Creates a FeelingNotFound error.

**Parameters:**
- `id` (string|number): The ID of the feeling that was not found

**Returns:** TodoziError instance

##### invalidPriority(priority)
Creates an InvalidPriority error.

**Parameters:**
- `priority` (string): The invalid priority value

**Returns:** TodoziError instance

##### invalidStatus(status)
Creates an InvalidStatus error.

**Parameters:**
- `status` (string): The invalid status value

**Returns:** TodoziError instance

##### invalidAssignee(assignee)
Creates an InvalidAssignee error.

**Parameters:**
- `assignee` (string): The invalid assignee value

**Returns:** TodoziError instance

##### invalidProgress(progress)
Creates an InvalidProgress error.

**Parameters:**
- `progress` (number): The invalid progress value

**Returns:** TodoziError instance

##### validation(message)
Creates a ValidationError.

**Parameters:**
- `message` (string): Validation error message

**Returns:** TodoziError instance

##### storage(message)
Creates a StorageError.

**Parameters:**
- `message` (string): Storage error message

**Returns:** TodoziError instance

##### config(message)
Creates a ConfigError.

**Parameters:**
- `message` (string): Configuration error message

**Returns:** TodoziError instance

##### io(message)
Creates an IoError.

**Parameters:**
- `message` (string): IO error message

**Returns:** TodoziError instance

##### json(error)
Creates a JsonError.

**Parameters:**
- `error` (Error): Original JSON error

**Returns:** TodoziError instance

##### uuid(error)
Creates a UuidError.

**Parameters:**
- `error` (Error): Original UUID error

**Returns:** TodoziError instance

##### chrono(error)
Creates a ChronoError.

**Parameters:**
- `error` (Error): Original Chrono error

**Returns:** TodoziError instance

##### dialoguer(error)
Creates a DialoguerError.

**Parameters:**
- `error` (Error): Original Dialoguer error

**Returns:** TodoziError instance

##### hlx(error)
Creates an HlxError.

**Parameters:**
- `error` (Error): Original HLX error

**Returns:** TodoziError instance

##### reqwest(error)
Creates a ReqwestError.

**Parameters:**
- `error` (Error): Original Reqwest error

**Returns:** TodoziError instance

##### dir(message)
Creates a DirError.

**Parameters:**
- `message` (string): Directory error message

**Returns:** TodoziError instance

##### embedding(message)
Creates an EmbeddingError.

**Parameters:**
- `message` (string): Embedding error message

**Returns:** TodoziError instance

##### api(message)
Creates an ApiError.

**Parameters:**
- `message` (string): API error message

**Returns:** TodoziError instance

##### candle(message)
Creates a CandleError.

**Parameters:**
- `message` (string): Candle error message

**Returns:** TodoziError instance

##### notImplemented(feature)
Creates a NotImplemented error.

**Parameters:**
- `feature` (string): The feature that is not implemented

**Returns:** TodoziError instance

##### serialization(message)
Creates a SerializationError.

**Parameters:**
- `message` (string): Serialization error message

**Returns:** TodoziError instance

### ErrorManager Class

#### Constructor
```javascript
new ErrorManager()
```

Initializes a new ErrorManager instance with an empty error registry.

#### Methods

##### createError(error)
Creates and registers a new error.

**Parameters:**
- `error` (object): Error object to register

**Returns:** Promise<string> - The UUID of the created error

##### getUnresolvedErrors()
Retrieves all unresolved errors.

**Returns:** Array<object> - Array of unresolved error objects

##### resolveError(errorId, resolution)
Marks an error as resolved with a resolution note.

**Parameters:**
- `errorId` (string): The UUID of the error to resolve
- `resolution` (string): Resolution note

**Returns:** Promise<void>

**Throws:** TodoziError if error with given ID is not found

##### generateUUID()
Generates a UUID.

**Returns:** string - Generated UUID

### Error Parser Functions

#### parseErrorFormat(errorText)
Parses error information from a formatted text string.

**Parameters:**
- `errorText` (string): Formatted error text with `<error>` and `</error>` tags

**Returns:** object - Parsed error object

**Throws:** TodoziError if format is invalid

#### parseErrorSeverity(severity)
Parses and validates error severity.

**Parameters:**
- `severity` (string): Severity string to parse

**Returns:** string - Normalized severity

**Throws:** TodoziError if severity is invalid

#### parseErrorCategory(category)
Parses and validates error category.

**Parameters:**
- `category` (string): Category string to parse

**Returns:** string - Normalized category

**Throws:** TodoziError if category is invalid

#### generateUUID()
Generates a UUID.

**Returns:** string - Generated UUID

## Usage Examples

### Basic Error Creation and Handling

```javascript
// Creating specific errors
try {
    throw TodoziError.taskNotFound(123);
} catch (error) {
    console.error(`${error.type}: ${error.message}`);
    // Output: TaskNotFound: Task not found: 123
}

// Creating validation errors
try {
    throw TodoziError.invalidPriority('extreme');
} catch (error) {
    console.error(`${error.type}: ${error.message}`);
    console.error('Details:', error.details);
    // Output: 
    // InvalidPriority: Invalid priority: extreme. Must be one of: low, medium, high, critical, urgent
    // Details: { priority: 'extreme' }
}
```

### Error Manager Usage

```javascript
// Initialize error manager
const errorManager = new ErrorManager();

// Create and register an error
const error = {
    title: 'Database Connection Failed',
    description: 'Could not connect to the main database',
    severity: 'high',
    category: 'storage'
};

errorManager.createError(error).then(errorId => {
    console.log(`Error registered with ID: ${errorId}`);
    
    // Get unresolved errors
    const unresolved = errorManager.getUnresolvedErrors();
    console.log(`Unresolved errors: ${unresolved.length}`);
    
    // Resolve the error
    errorManager.resolveError(errorId, 'Database connection restored')
        .then(() => console.log('Error resolved'))
        .catch(err => console.error('Failed to resolve error:', err));
});
```

### Error Parsing

```javascript
// Parse formatted error text
const errorText = "<error>Network Timeout;Connection to server timed out;high;network;api_client;Request ID: 12345;timeout,connection</error>";

try {
    const parsedError = parseErrorFormat(errorText);
    console.log('Parsed Error:', parsedError);
    // Output will include all parsed fields with proper structure
} catch (error) {
    console.error('Failed to parse error:', error.message);
}
```

### Custom Error Handling Integration

```javascript
// Example integration with application logic
class TaskManager {
    constructor() {
        this.errorManager = new ErrorManager();
    }
    
    async getTask(id) {
        try {
            // Simulate task lookup
            const task = this.findTask(id);
            if (!task) {
                const error = TodoziError.taskNotFound(id);
                await this.errorManager.createError({
                    title: 'Task Not Found',
                    description: error.message,
                    severity: 'medium',
                    category: 'validation',
                    source: 'TaskManager.getTask'
                });
                throw error;
            }
            return task;
        } catch (error) {
            if (error instanceof TodoziError) {
                throw error;
            }
            // Handle unexpected errors
            const genericError = TodoziError.storage('Unexpected error while retrieving task');
            await this.errorManager.createError({
                title: 'Unexpected Error',
                description: genericError.message,
                severity: 'high',
                category: 'general',
                source: 'TaskManager.getTask'
            });
            throw genericError;
        }
    }
}
```

## Design Patterns

### 1. Factory Pattern
The `TodoziError` class uses static factory methods to create specific error types, providing a clean API for error creation.

### 2. Registry Pattern
The `ErrorManager` class implements a registry pattern to centralize error management and tracking.

### 3. Custom Error Extension
Extends the native JavaScript Error class to provide additional context and structure.

### 4. Parser Pattern
The error parsing functions implement a parser pattern for converting formatted text into structured error objects.

## Performance Analysis

### Time Complexity
- **Error Creation**: O(1) - Constant time for creating new error instances
- **Error Registration**: O(1) - Map-based storage provides constant time insertion
- **Error Retrieval**: O(n) - Linear time for filtering unresolved errors where n is total errors
- **Error Resolution**: O(1) - Map lookup and update operations

### Space Complexity
- **Error Storage**: O(n) - Where n is the number of registered errors
- **UUID Generation**: O(1) - Constant space for UUID generation

### Optimization Considerations
1. **Memory Management**: Consider implementing error expiration or archiving for long-running applications
2. **Bulk Operations**: For high-volume error handling, consider batch processing capabilities
3. **Indexing**: For large error sets, consider indexing by category, severity, or date for faster queries

## Security Considerations

### 1. Error Information Exposure
- Avoid including sensitive information in error messages
- Sanitize error details before exposing to end users
- Implement different error detail levels for different user roles

### 2. Input Validation
- All error parsing functions include validation to prevent injection attacks
- UUID generation uses cryptographically secure random number generation (note: current implementation uses Math.random which is not cryptographically secure)

### 3. Error Handling Best Practices
- Never expose stack traces to end users
- Log errors securely for debugging purposes
- Implement rate limiting for error creation to prevent abuse

## Testing Strategies

### Unit Tests

```javascript
// Test error creation
describe('TodoziError', () => {
    test('creates error with correct properties', () => {
        const error = new TodoziError('Test message', 'TestType', { detail: 'value' });
        expect(error.message).toBe('Test message');
        expect(error.type).toBe('TestType');
        expect(error.details).toEqual({ detail: 'value' });
    });
    
    test('creates specific errors using factory methods', () => {
        const error = TodoziError.taskNotFound(123);
        expect(error.type).toBe('TaskNotFound');
        expect(error.message).toBe('Task not found: 123');
    });
});

// Test error manager
describe('ErrorManager', () => {
    test('creates and retrieves errors', async () => {
        const manager = new ErrorManager();
        const errorId = await manager.createError({ title: 'Test Error' });
        const unresolved = manager.getUnresolvedErrors();
        expect(unresolved).toHaveLength(1);
        expect(unresolved[0].id).toBe(errorId);
    });
});
```

### Integration Tests

```javascript
// Test error parsing integration
describe('Error Parsing', () => {
    test('parses valid error format', () => {
        const errorText = "<error>Test;Description;high;network;source;context;tag1,tag2</error>";
        const parsed = parseErrorFormat(errorText);
        expect(parsed.title).toBe('Test');
        expect(parsed.severity).toBe('high');
        expect(parsed.tags).toEqual(['tag1', 'tag2']);
    });
    
    test('throws on invalid format', () => {
        const invalidText = "invalid format";
        expect(() => parseErrorFormat(invalidText)).toThrow(TodoziError);
    });
});
```

## Deployment Instructions

### Prerequisites
- Node.js v12 or higher
- npm or yarn package manager

### Installation
```bash
# Install as dependency
npm install todozi-error-system

# Or include directly in your project
# Copy the source files to your project directory
```

### Configuration
No special configuration required. The system is self-contained and ready to use.

### Integration
```javascript
// ES6 modules
import { TodoziError, ErrorManager, parseErrorFormat } from './todozi-error-system';

// CommonJS
const { TodoziError, ErrorManager, parseErrorFormat } = require('./todozi-error-system');
```

## Troubleshooting Guide

### Common Issues

#### 1. "TodoziError is not a constructor"
**Cause:** Incorrect import or instantiation
**Solution:** Ensure proper import and use `new` keyword:
```javascript
const error = new TodoziError('Message'); // Correct
```

#### 2. "Error not found" when resolving
**Cause:** Invalid error ID or error already resolved
**Solution:** Verify error ID exists and is unresolved:
```javascript
const unresolved = errorManager.getUnresolvedErrors();
console.log(unresolved.map(e => e.id)); // Check available IDs
```

#### 3. "Invalid error format" parsing errors
**Cause:** Malformed error text format
**Solution:** Ensure proper format with required fields:
```javascript
// Correct format:
"<error>title;description;severity;category;source;context;tag1,tag2</error>"
```

### Debugging Tips

1. **Enable detailed logging** to capture error creation and resolution events
2. **Check error types** using `instanceof` to ensure proper error handling
3. **Validate inputs** before calling error parsing functions
4. **Monitor error volume** to identify potential system issues

### Performance Monitoring

1. **Track error creation rate** to identify system health
2. **Monitor resolution time** for SLA compliance
3. **Analyze error categories** to identify systemic issues
4. **Implement error aging** to identify stale unresolved errors

This comprehensive documentation provides everything needed to understand, implement, and maintain the Todozi Error Management System effectively.