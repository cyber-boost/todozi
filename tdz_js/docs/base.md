# Tool Framework Documentation

## Overview

This documentation provides a comprehensive analysis of a JavaScript framework for defining, managing, and executing tools. The framework provides a structured approach to creating tools that can be used in AI applications, particularly with language models like Ollama.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Tool Framework                               │
├─────────────────────────────────────────────────────────────────────┤
│  ToolParameter     ToolDefinition     ToolResult     ToolError      │
│        │                 │              │              │           │
│        └─────────────────┼──────────────┼──────────────┘           │
│                          │              │                          │
│                   ToolRegistry     ErrorHandler                    │
│                          │              │                          │
│                          └──────────────┘                          │
│                                 │                                  │
│                              Tool                                  │
│                                 │                                  │
│                         Tool Subclasses                            │
└─────────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. ToolParameter Class

#### Description
Represents a parameter for a tool, defining its properties such as name, type, description, and whether it's required.

#### Constructor
```javascript
new ToolParameter(name, type_, description, required, default = null)
```

**Parameters:**
- `name` (string): The parameter name
- `type_` (string): The parameter type (e.g., 'string', 'number', 'boolean')
- `description` (string): Description of the parameter
- `required` (boolean): Whether the parameter is required
- `default` (any, optional): Default value for the parameter

#### Methods

##### static new()
Creates a new ToolParameter instance.

**Parameters:**
- `name` (string): The parameter name
- `type_` (string): The parameter type
- `description` (string): Description of the parameter
- `required` (boolean): Whether the parameter is required
- `default` (any, optional): Default value for the parameter

**Returns:** ToolParameter instance

#### Example Usage
```javascript
const param = new ToolParameter('filename', 'string', 'Name of the file to process', true);
const optionalParam = new ToolParameter('timeout', 'number', 'Timeout in seconds', false, 30);
```

### 2. ResourceLock Object

#### Description
An enumeration of resource types that tools might need to lock when executing to prevent conflicts.

#### Constants
- `FilesystemWrite`: Lock for writing to the filesystem
- `FilesystemRead`: Lock for reading from the filesystem
- `Git`: Lock for Git operations
- `Memory`: Lock for memory operations
- `Shell`: Lock for shell command execution
- `Network`: Lock for network operations

#### Methods

##### asStr()
Converts a ResourceLock constant to its string representation.

**Parameters:**
- `lock` (ResourceLock): The lock constant to convert

**Returns:** string representation of the lock

**Throws:** Error if the lock is unknown

#### Example Usage
```javascript
const lockStr = ResourceLock.asStr(ResourceLock.FilesystemWrite); // 'filesystem_write'
```

### 3. ToolDefinition Class

#### Description
Defines the structure and metadata of a tool, including its parameters, category, and required resource locks.

#### Constructor
```javascript
new ToolDefinition(name, description, parameters, category, resource_locks)
```

**Parameters:**
- `name` (string): Name of the tool
- `description` (string): Description of what the tool does
- `parameters` (Array<ToolParameter>): Array of parameter definitions
- `category` (string): Category the tool belongs to
- `resource_locks` (Array<ResourceLock>): Array of required resource locks

#### Methods

##### static new()
Creates a new ToolDefinition instance.

**Parameters:**
- `name` (string): Name of the tool
- `description` (string): Description of what the tool does
- `parameters` (Array<ToolParameter>): Array of parameter definitions
- `category` (string): Category the tool belongs to
- `resource_locks` (Array<ResourceLock>): Array of required resource locks

**Returns:** ToolDefinition instance

##### toOllamaFormat()
Converts the tool definition to a format compatible with Ollama function calling.

**Returns:** Object in Ollama function format

#### Example Usage
```javascript
const readFileParam = new ToolParameter('path', 'string', 'Path to file', true);
const definition = new ToolDefinition(
    'file_read',
    'Read file contents',
    [readFileParam],
    'File Operations',
    [ResourceLock.FilesystemRead]
);

const ollamaFormat = definition.toOllamaFormat();
```

### 4. ToolResult Class

#### Description
Represents the result of executing a tool, including success status, output, errors, and metadata.

#### Constructor
```javascript
new ToolResult(success, output, error, execution_time_ms, metadata = null, recovery_context = null)
```

**Parameters:**
- `success` (boolean): Whether the tool execution was successful
- `output` (string): Output from the tool execution
- `error` (string): Error message if execution failed
- `execution_time_ms` (number): Time taken to execute in milliseconds
- `metadata` (Object, optional): Additional metadata about the execution
- `recovery_context` (Object, optional): Context for recovery if needed

#### Methods

##### static new()
Creates a new ToolResult instance.

**Parameters:**
- `success` (boolean): Whether the tool execution was successful
- `output` (string): Output from the tool execution
- `error` (string): Error message if execution failed
- `execution_time_ms` (number): Time taken to execute in milliseconds
- `metadata` (Object, optional): Additional metadata about the execution
- `recovery_context` (Object, optional): Context for recovery if needed

**Returns:** ToolResult instance

##### static success()
Creates a successful ToolResult.

**Parameters:**
- `output` (string): Output from the tool execution
- `execution_time_ms` (number): Time taken to execute in milliseconds

**Returns:** ToolResult instance

##### static error()
Creates an error ToolResult.

**Parameters:**
- `error` (string): Error message
- `execution_time_ms` (number): Time taken to execute in milliseconds

**Returns:** ToolResult instance

##### toString()
Converts the result to a string representation.

**Returns:** string representation of the result

#### Example Usage
```javascript
const successResult = ToolResult.success('File contents here', 150);
const errorResult = ToolResult.error('File not found', 50);
```

### 5. ErrorType Object

#### Description
An enumeration of error types that can occur during tool execution.

#### Constants
- `ValidationError`: Error in parameter validation
- `PermissionError`: Permission denied error
- `FileNotFound`: File not found error
- `TimeoutError`: Operation timed out
- `ResourceError`: Resource-related error
- `NetworkError`: Network communication error
- `SecurityError`: Security-related error
- `InternalError`: Internal system error

#### Methods

##### toString()
Converts an ErrorType constant to its string representation.

**Parameters:**
- `errorType` (ErrorType): The error type to convert

**Returns:** string representation of the error type

**Throws:** Error if the error type is unknown

#### Example Usage
```javascript
const errorStr = ErrorType.toString(ErrorType.ValidationError); // 'validation_error'
```

### 6. ToolError Class

#### Description
Custom error class for tool-specific errors with additional metadata.

#### Constructor
```javascript
new ToolError(message, error_type, details = {})
```

**Parameters:**
- `message` (string): Error message
- `error_type` (ErrorType): Type of error
- `details` (Object, optional): Additional error details

#### Methods

##### static new()
Creates a new ToolError instance.

**Parameters:**
- `message` (string): Error message
- `error_type` (ErrorType): Type of error
- `details` (Object, optional): Additional error details

**Returns:** ToolError instance

##### toString()
Returns the error message.

**Returns:** string error message

#### Example Usage
```javascript
const error = new ToolError('Invalid file path', ErrorType.ValidationError, { path: '/invalid/path' });
```

### 7. ErrorHandler Class

#### Description
Provides utility methods for handling errors and validating parameters in tool execution.

#### Methods

##### static handleError()
Handles an error and creates an appropriate ToolResult.

**Parameters:**
- `error` (Error): The error to handle
- `context` (string): Context where the error occurred

**Returns:** ToolResult representing the error

##### static validateRequiredParams()
Validates that all required parameters are present.

**Parameters:**
- `kwargs` (Object): Parameter values to validate
- `required_params` (Array<string>): Names of required parameters

**Returns:** ToolResult if validation fails, null if validation passes

##### static validateStringParam()
Validates a string parameter against constraints.

**Parameters:**
- `value` (any): Value to validate
- `param_name` (string): Name of the parameter
- `min_length` (number): Minimum length requirement
- `max_length` (number): Maximum length requirement
- `pattern` (string, optional): Regular expression pattern to match

**Returns:** ToolResult if validation fails, null if validation passes

##### static createSuccessResult()
Creates a successful ToolResult.

**Parameters:**
- `output` (string): Output from the tool execution
- `execution_time_ms` (number): Time taken to execute in milliseconds
- `metadata` (Object, optional): Additional metadata

**Returns:** ToolResult instance

##### static createErrorResult()
Creates an error ToolResult with specific error type.

**Parameters:**
- `error_msg` (string): Error message
- `execution_time_ms` (number): Time taken to execute in milliseconds
- `error_type` (ErrorType): Type of error
- `metadata` (Object, optional): Additional metadata

**Returns:** ToolResult instance

#### Example Usage
```javascript
const validationError = ErrorHandler.validateRequiredParams(
    { param1: 'value1' },
    ['param1', 'param2']
);

if (validationError) {
    // Handle missing param2
}
```

### 8. Tool Class

#### Description
Abstract base class for all tools. Defines the interface that all tools must implement.

#### Methods

##### async execute()
Executes the tool with the given parameters.

**Parameters:**
- `kwargs` (Object): Parameters for tool execution

**Returns:** Promise<ToolResult>

**Throws:** Error if not implemented by subclass

##### definition()
Returns the tool's definition.

**Returns:** ToolDefinition

**Throws:** Error if not implemented by subclass

##### name()
Returns the tool's name.

**Returns:** string tool name

##### validateParameters()
Validates the parameters against the tool's definition.

**Parameters:**
- `kwargs` (Object): Parameters to validate

**Returns:** boolean indicating if parameters are valid

#### Example Usage
```javascript
class FileReadTool extends Tool {
    definition() {
        return new ToolDefinition(
            'file_read',
            'Read file contents',
            [new ToolParameter('path', 'string', 'Path to file', true)],
            'File Operations',
            [ResourceLock.FilesystemRead]
        );
    }
    
    async execute(kwargs) {
        // Implementation here
    }
}
```

### 9. ToolRegistry Class

#### Description
Manages a collection of tools, providing registration, retrieval, and execution capabilities.

#### Constructor
```javascript
new ToolRegistry()
```

#### Methods

##### register()
Registers a tool in the registry.

**Parameters:**
- `tool` (Tool): Tool to register

##### registerCoreTools()
Registers core tools (placeholder method).

##### getTool()
Retrieves a tool by name.

**Parameters:**
- `name` (string): Name of the tool

**Returns:** Tool or undefined

##### getAllTools()
Retrieves all registered tools.

**Returns:** Array<Tool>

##### getToolDefinitions()
Retrieves all tool definitions in Ollama format.

**Returns:** Array<Object>

##### async executeTool()
Executes a tool by name with the given parameters.

**Parameters:**
- `tool_name` (string): Name of the tool to execute
- `kwargs` (Object): Parameters for execution

**Returns:** Promise<ToolResult>

##### toolCount()
Returns the number of registered tools.

**Returns:** number

##### hasTool()
Checks if a tool is registered.

**Parameters:**
- `name` (string): Name of the tool

**Returns:** boolean

##### unregister()
Removes a tool from the registry.

**Parameters:**
- `name` (string): Name of the tool to remove

**Returns:** boolean indicating success

##### clear()
Removes all tools from the registry.

#### Example Usage
```javascript
const registry = new ToolRegistry();
const tool = new FileReadTool();
registry.register(tool);

const result = await registry.executeTool('file_read', { path: '/path/to/file' });
```

## Helper Functions

### createToolParameter()
Creates a new ToolParameter without a default value.

**Parameters:**
- `name` (string): Parameter name
- `type_` (string): Parameter type
- `description` (string): Parameter description
- `required` (boolean): Whether parameter is required

**Returns:** ToolParameter

### createToolParameterWithDefault()
Creates a new ToolParameter with a default value.

**Parameters:**
- `name` (string): Parameter name
- `type_` (string): Parameter type
- `description` (string): Parameter description
- `required` (boolean): Whether parameter is required
- `defaultValue` (any): Default value

**Returns:** ToolParameter

### createToolDefinition()
Creates a new ToolDefinition without resource locks.

**Parameters:**
- `name` (string): Tool name
- `description` (string): Tool description
- `category` (string): Tool category
- `parameters` (Array<ToolParameter>): Tool parameters

**Returns:** ToolDefinition

### createToolDefinitionWithLocks()
Creates a new ToolDefinition with resource locks.

**Parameters:**
- `name` (string): Tool name
- `description` (string): Tool description
- `category` (string): Tool category
- `parameters` (Array<ToolParameter>): Tool parameters
- `resource_locks` (Array<ResourceLock>): Required resource locks

**Returns:** ToolDefinition

## Design Patterns

### 1. Factory Pattern
The framework uses factory methods (`new()` static methods) to create instances of classes, providing a consistent interface for object creation.

### 2. Registry Pattern
The `ToolRegistry` class implements the registry pattern to manage and provide access to registered tools.

### 3. Template Method Pattern
The `Tool` class defines the interface that all concrete tool implementations must follow, with abstract methods that subclasses must implement.

### 4. Builder Pattern
Helper functions provide a simplified interface for creating complex objects like `ToolParameter` and `ToolDefinition`.

### 5. Error Handling Strategy
The framework implements a comprehensive error handling strategy with custom error types and centralized error processing.

## Performance Analysis

### Time Complexity
- Tool registration: O(1)
- Tool retrieval: O(1)
- Parameter validation: O(n) where n is the number of parameters
- Tool execution: Depends on the specific tool implementation

### Space Complexity
- Tool storage: O(n) where n is the number of registered tools
- Parameter storage: O(m) where m is the total number of parameters across all tools

### Optimization Considerations
1. The registry uses a Map for O(1) tool lookups
2. Parameter validation is performed efficiently with early exits
3. Tool definitions are converted to Ollama format on-demand rather than pre-computing

## Security Considerations

### 1. Input Validation
The framework provides robust parameter validation to prevent injection attacks and malformed inputs.

### 2. Resource Locking
Resource locks help prevent race conditions and unauthorized access to system resources.

### 3. Error Handling
Careful error handling prevents information leakage through error messages.

### 4. Type Safety
Strong typing helps prevent type confusion vulnerabilities.

## Testing Strategies

### Unit Testing
The `TestRunner` class provides basic unit tests for core functionality:

1. ToolParameter creation
2. ToolDefinition Ollama format conversion
3. ToolRegistry operations
4. ErrorHandler validation
5. ToolResult display

### Integration Testing
Integration tests should verify:
1. Tool registration and retrieval
2. Parameter validation across different tool types
3. Error handling in various scenarios
4. Tool execution with different parameter combinations

### Performance Testing
Performance tests should measure:
1. Tool registration and lookup times
2. Parameter validation performance
3. Tool execution overhead

## Deployment Instructions

### Installation
```javascript
// For Node.js environments
const {
    ToolParameter,
    ResourceLock,
    ToolDefinition,
    ToolResult,
    ErrorType,
    ToolError,
    ErrorHandler,
    Tool,
    ToolRegistry,
    createToolParameter,
    createToolParameterWithDefault,
    createToolDefinition,
    createToolDefinitionWithLocks,
    TestRunner
} = require('./tool-framework');

// For browser environments (if bundled)
import {
    ToolParameter,
    ResourceLock,
    ToolDefinition,
    ToolResult,
    ErrorType,
    ToolError,
    ErrorHandler,
    Tool,
    ToolRegistry,
    createToolParameter,
    createToolParameterWithDefault,
    createToolDefinition,
    createToolDefinitionWithLocks,
    TestRunner
} from './tool-framework';
```

### Configuration
No special configuration is required. The framework is self-contained and ready to use.

### Dependencies
The framework has no external dependencies and can run in any modern JavaScript environment.

## Troubleshooting Guide

### Common Issues

#### 1. "execute must be implemented by subclass"
**Cause:** A Tool subclass doesn't implement the execute method
**Solution:** Implement the execute method in your Tool subclass

#### 2. "definition must be implemented by subclass"
**Cause:** A Tool subclass doesn't implement the definition method
**Solution:** Implement the definition method in your Tool subclass

#### 3. "Tool 'X' not found"
**Cause:** Attempting to execute an unregistered tool
**Solution:** Register the tool with ToolRegistry before executing

#### 4. "Invalid parameters for tool 'X'"
**Cause:** Parameter validation failed
**Solution:** Check that all required parameters are provided and have correct types

### Debugging Tips

1. Use `console.log` to inspect ToolResult objects for detailed error information
2. Check the metadata field in ToolResult for additional context
3. Use the TestRunner to verify basic functionality
4. Implement custom logging in tool execute methods for detailed execution traces

### Performance Issues

1. If tool registration is slow, check for unnecessary tool definitions
2. If parameter validation is slow, review the number and complexity of parameters
3. For execution performance issues, profile individual tool implementations

## Example Implementations

### File Read Tool
```javascript
class FileReadTool extends Tool {
    definition() {
        return new ToolDefinition(
            'file_read',
            'Read the contents of a file',
            [
                new ToolParameter('path', 'string', 'Path to the file to read', true)
            ],
            'File Operations',
            [ResourceLock.FilesystemRead]
        );
    }

    async execute(kwargs) {
        const startTime = Date.now();
        
        try {
            // Validate parameters
            const validationError = ErrorHandler.validateRequiredParams(kwargs, ['path']);
            if (validationError) return validationError;
            
            const pathValidation = ErrorHandler.validateStringParam(kwargs.path, 'path', 1, 1000);
            if (pathValidation) return pathValidation;
            
            // Simulate file reading
            const content = `Contents of file: ${kwargs.path}`;
            
            return ErrorHandler.createSuccessResult(
                content,
                Date.now() - startTime
            );
        } catch (error) {
            return ErrorHandler.handleError(error, 'FileReadTool.execute');
        }
    }
}
```

### HTTP Request Tool
```javascript
class HttpRequestTool extends Tool {
    definition() {
        return new ToolDefinition(
            'http_request',
            'Make an HTTP request to a URL',
            [
                new ToolParameter('url', 'string', 'URL to request', true),
                new ToolParameter('method', 'string', 'HTTP method (GET, POST, etc.)', false, 'GET'),
                new ToolParameter('headers', 'object', 'HTTP headers', false, {}),
                new ToolParameter('body', 'string', 'Request body', false, '')
            ],
            'Network Operations',
            [ResourceLock.Network]
        );
    }

    async execute(kwargs) {
        const startTime = Date.now();
        
        try {
            // Validate parameters
            const validationError = ErrorHandler.validateRequiredParams(kwargs, ['url']);
            if (validationError) return validationError;
            
            const urlValidation = ErrorHandler.validateStringParam(kwargs.url, 'url', 1, 2000);
            if (urlValidation) return urlValidation;
            
            // Simulate HTTP request
            const response = {
                status: 200,
                body: `Response from ${kwargs.url}`,
                headers: kwargs.headers || {}
            };
            
            return ErrorHandler.createSuccessResult(
                JSON.stringify(response),
                Date.now() - startTime
            );
        } catch (error) {
            return ErrorHandler.handleError(error, 'HttpRequestTool.execute');
        }
    }
}
```

### Usage Example
```javascript
// Create registry and register tools
const registry = new ToolRegistry();
registry.register(new FileReadTool());
registry.register(new HttpRequestTool());

// Execute tools
async function example() {
    // File read
    const fileResult = await registry.executeTool('file_read', {
        path: '/path/to/file.txt'
    });
    
    console.log('File read result:', fileResult.toString());
    
    // HTTP request
    const httpResult = await registry.executeTool('http_request', {
        url: 'https://api.example.com/data',
        method: 'GET'
    });
    
    console.log('HTTP result:', httpResult.toString());
}

example();
```

This comprehensive documentation covers all aspects of the tool framework, providing developers with the information needed to understand, use, and extend the system effectively.