# Todozi Code Generation System - Comprehensive Documentation

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

The Todozi Code Generation System is a sophisticated framework designed to manage and orchestrate code generation tasks through a structured chunking approach. It provides mechanisms for organizing code into hierarchical levels, tracking dependencies, managing project state, and maintaining context during the generation process.

### Key Features
- Hierarchical code chunking with predefined levels
- Dependency management between code chunks
- Project state tracking with line count and module completion
- Context window management for maintaining generation context
- Chunk status tracking through a defined lifecycle
- XML-based chunk definition parsing

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────────────┐
│                        Todozi Code Generation System                │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────┐  ┌─────────────────────────────────────┐  │
│  │   Error Handling     │  │         Core Components             │  │
│  │                      │  │                                     │  │
│  │  ┌────────────────┐  │  │  ┌───────────────────────────────┐  │  │
│  │  │  TodoziError   │  │  │  │       ChunkingLevel           │  │  │
│  │  └────────────────┘  │  │  └───────────────────────────────┘  │  │
│  └──────────────────────┘  │                                     │  │
│                            │  ┌───────────────────────────────┐  │  │
│  ┌──────────────────────┐  │  │        ChunkStatus            │  │  │
│  │   Data Structures    │  │  └───────────────────────────────┘  │  │
│  │                      │  │                                     │  │
│  │  ┌────────────────┐  │  │  ┌───────────────────────────────┐  │  │
│  │  │  ProjectState  │  │  │  │       ContextWindow           │  │  │
│  │  └────────────────┘  │  │  └───────────────────────────────┘  │  │
│  │                      │  │                                     │  │
│  │  ┌────────────────┐  │  │  ┌───────────────────────────────┐  │  │
│  │  │   CodeChunk    │  │  │  │    CodeGenerationGraph        │  │  │
│  │  └────────────────┘  │  │  └───────────────────────────────┘  │  │
│  └──────────────────────┘  │                                     │  │
│                            └─────────────────────────────────────┘  │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                    Parsing & Processing                         ││
│  │                                                                 ││
│  │  ┌──────────────────────────┐  ┌─────────────────────────────┐  ││
│  │  │   parseChunkingFormat    │  │  processChunkingMessage     │  ││
│  │  └──────────────────────────┘  └─────────────────────────────┘  ││
│  └─────────────────────────────────────────────────────────────────┘│
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                          Testing                                ││
│  │                                                                 ││
│  │  ┌──────────────────────────┐                                  ││
│  │  │       runTests           │                                  ││
│  │  └──────────────────────────┘                                  ││
│  └─────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### Component Relationships
```
TodoziError
    ↑
    |
ChunkingLevel ←→ ChunkStatus
    ↑              ↑
    |              |
CodeChunk ← CodeGenerationGraph → ProjectState
    ↑              ↑
    |              |
ContextWindow ←────┘
    ↑
    |
parseChunkingFormat → processChunkingMessage
```

## Core Components

### 1. TodoziError
A custom error class that extends the native JavaScript Error class for specific error handling within the Todozi system.

### 2. ChunkingLevel
Defines hierarchical levels for code organization with token limits and descriptions.

### 3. ChunkStatus
Represents the lifecycle states of code chunks.

### 4. ProjectState
Manages the overall project state including line counts, modules, dependencies, and global variables.

### 5. ContextWindow
Maintains contextual information during code generation including class information, imports, and error patterns.

### 6. CodeChunk
Represents an individual unit of code with its dependencies, status, and content.

### 7. CodeGenerationGraph
The main orchestrator that manages all chunks, their relationships, and project state.

### 8. Parsing Functions
Utilities for parsing chunk definitions from text format.

## Detailed API Documentation

### TodoziError
```javascript
class TodoziError extends Error
```

**Description**: Custom error class for Todozi-specific errors.

**Constructor**:
```javascript
new TodoziError(message)
```
- `message` (string): Error message

**Properties**:
- `name` (string): Always set to 'TodoziError'

### ChunkingLevel
```javascript
class ChunkingLevel
```

**Description**: Defines hierarchical levels for code organization with associated token limits and examples.

**Static Properties**:
- `Project`: High-level project planning (100 tokens)
- `Module`: Major system components (500 tokens)
- `Class`: Class definitions (1000 tokens)
- `Method`: Individual methods (300 tokens)
- `Block`: Small code blocks (100 tokens)

**Constructor**:
```javascript
new ChunkingLevel(value, maxTokens, description, example)
```
- `value` (string): Level identifier
- `maxTokens` (number): Maximum token count for this level
- `description` (string): Description of the level
- `example` (string): Example of what this level represents

**Static Methods**:
#### fromString(s)
Converts a string to a ChunkingLevel instance.

**Parameters**:
- `s` (string): String representation of the level

**Returns**: ChunkingLevel instance

**Throws**: TodoziError if the string doesn't match any valid level

#### Instance Methods:
#### toString()
Returns the string representation of the level.

**Returns**: string

#### max_tokens()
Returns the maximum token count for this level.

**Returns**: number

#### description()
Returns the description of this level.

**Returns**: string

#### example()
Returns an example for this level.

**Returns**: string

### ChunkStatus
```javascript
class ChunkStatus
```

**Description**: Represents the lifecycle states of code chunks.

**Static Properties**:
- `Pending`: Chunk is waiting to be processed
- `InProgress`: Chunk is currently being worked on
- `Completed`: Chunk has been completed
- `Validated`: Chunk has been validated
- `Failed`: Chunk processing failed

**Constructor**:
```javascript
new ChunkStatus(value)
```
- `value` (string): Status identifier

**Methods**:
#### toString()
Returns the string representation of the status.

**Returns**: string

### ProjectState
```javascript
class ProjectState
```

**Description**: Manages the overall project state including line counts, modules, dependencies, and global variables.

**Constructor**:
```javascript
new ProjectState(maxLines)
```
- `maxLines` (number): Maximum number of lines for the project

**Properties**:
- `totalLines` (number): Total lines written
- `maxLines` (number): Maximum allowed lines
- `currentModule` (string): Currently active module
- `dependencies` (array): Project dependencies
- `completedModules` (array): List of completed modules
- `pendingModules` (array): List of pending modules
- `globalVariables` (Map): Map of global variables
- `createdAt` (Date): Creation timestamp
- `updatedAt` (Date): Last update timestamp

**Methods**:
#### toStateString()
Generates a formatted string representation of the project state.

**Returns**: string

#### formatDate(date)
Formats a date object into a standardized string.

**Parameters**:
- `date` (Date): Date to format

**Returns**: string

#### addCompletedModule(module)
Adds a module to the completed modules list.

**Parameters**:
- `module` (string): Module name

#### addPendingModule(module)
Adds a module to the pending modules list.

**Parameters**:
- `module` (string): Module name

#### setGlobalVariable(key, value)
Sets a global variable.

**Parameters**:
- `key` (string): Variable name
- `value` (any): Variable value

#### incrementLines(lines)
Increments the total line count.

**Parameters**:
- `lines` (number): Number of lines to add

### ContextWindow
```javascript
class ContextWindow
```

**Description**: Maintains contextual information during code generation.

**Constructor**:
```javascript
new ContextWindow()
```

**Properties**:
- `previousClass` (string): Previous class name
- `currentClass` (string): Current class name
- `nextPlanned` (string): Next planned item
- `globalVarsInScope` (array): Global variables in current scope
- `importsUsed` (array): Imports used
- `functionSignatures` (Map): Function signatures
- `errorPatternsSeen` (array): Error patterns encountered
- `createdAt` (Date): Creation timestamp
- `updatedAt` (Date): Last update timestamp

**Methods**:
#### toContextString()
Generates a formatted string representation of the context window.

**Returns**: string

#### formatDate(date)
Formats a date object into a standardized string.

**Parameters**:
- `date` (Date): Date to format

**Returns**: string

#### addImport(importName)
Adds an import to the imports list.

**Parameters**:
- `importName` (string): Import name

#### addFunctionSignature(name, signature)
Adds a function signature.

**Parameters**:
- `name` (string): Function name
- `signature` (string): Function signature

#### addErrorPattern(pattern)
Adds an error pattern to the seen patterns list.

**Parameters**:
- `pattern` (string): Error pattern

#### setCurrentClass(className)
Sets the current class and updates the previous class.

**Parameters**:
- `className` (string): Class name

### CodeChunk
```javascript
class CodeChunk
```

**Description**: Represents an individual unit of code with its dependencies, status, and content.

**Constructor**:
```javascript
new CodeChunk(chunkId, level)
```
- `chunkId` (string): Unique identifier for the chunk
- `level` (ChunkingLevel): Chunking level

**Properties**:
- `chunkId` (string): Chunk identifier
- `status` (ChunkStatus): Current status
- `dependencies` (array): Dependencies
- `code` (string): Code content
- `tests` (string): Test content
- `validated` (boolean): Validation status
- `level` (ChunkingLevel): Chunking level
- `estimatedTokens` (number): Estimated token count
- `createdAt` (Date): Creation timestamp
- `updatedAt` (Date): Last update timestamp

**Methods**:
#### addDependency(dep)
Adds a dependency to the chunk.

**Parameters**:
- `dep` (string): Dependency identifier

#### setCode(code)
Sets the code content and estimates tokens.

**Parameters**:
- `code` (string): Code content

#### setTests(tests)
Sets the test content.

**Parameters**:
- `tests` (string): Test content

#### markCompleted()
Marks the chunk as completed.

#### markValidated()
Marks the chunk as validated.

#### markFailed()
Marks the chunk as failed.

### CodeGenerationGraph
```javascript
class CodeGenerationGraph
```

**Description**: The main orchestrator that manages all chunks, their relationships, and project state.

**Constructor**:
```javascript
new CodeGenerationGraph(maxLines)
```
- `maxLines` (number): Maximum lines for the project

**Properties**:
- `chunks` (Map): Map of all chunks
- `projectState` (ProjectState): Project state manager
- `contextWindow` (ContextWindow): Context window manager

**Methods**:
#### addChunk(chunkId, level, deps)
Adds a new chunk to the graph.

**Parameters**:
- `chunkId` (string): Chunk identifier
- `level` (ChunkingLevel): Chunking level
- `deps` (array): Dependencies

#### getReadyChunks()
Gets chunks that are ready to be processed (pending with satisfied dependencies).

**Returns**: array of chunk IDs

#### getChunk(chunkId)
Retrieves a chunk by ID.

**Parameters**:
- `chunkId` (string): Chunk identifier

**Returns**: CodeChunk or undefined

#### getChunkMut(chunkId)
Alias for getChunk.

**Parameters**:
- `chunkId` (string): Chunk identifier

**Returns**: CodeChunk or undefined

#### updateChunkCode(chunkId, code)
Updates the code content of a chunk.

**Parameters**:
- `chunkId` (string): Chunk identifier
- `code` (string): New code content

**Returns**: null on success, error message on failure

#### updateChunkTests(chunkId, tests)
Updates the test content of a chunk.

**Parameters**:
- `chunkId` (string): Chunk identifier
- `tests` (string): New test content

**Returns**: null on success, error message on failure

#### markChunkCompleted(chunkId)
Marks a chunk as completed.

**Parameters**:
- `chunkId` (string): Chunk identifier

**Returns**: null on success, error message on failure

#### markChunkValidated(chunkId)
Marks a chunk as validated.

**Parameters**:
- `chunkId` (string): Chunk identifier

**Returns**: null on success, error message on failure

#### getProjectSummary()
Generates a comprehensive project summary.

**Returns**: string

#### getNextChunkToWorkOn()
Gets the next chunk that should be worked on.

**Returns**: chunk ID or null

#### getChunksByLevel(level)
Gets all chunks of a specific level.

**Parameters**:
- `level` (ChunkingLevel): Chunking level

**Returns**: array of CodeChunk

#### getDependencyChain(chunkId)
Gets the dependency chain for a chunk.

**Parameters**:
- `chunkId` (string): Chunk identifier

**Returns**: array of chunk IDs

#### _buildDependencyChain(chunkId, chain, visited)
Internal method to build dependency chain.

**Parameters**:
- `chunkId` (string): Chunk identifier
- `chain` (array): Chain being built
- `visited` (Set): Visited chunks

### Parsing Functions

#### parseChunkingFormat(chunkText)
Parses a chunk definition from text format.

**Parameters**:
- `chunkText` (string): Text containing chunk definition

**Returns**: CodeChunk

**Throws**: Error on parsing failure

#### processChunkingMessage(message)
Processes a message containing multiple chunk definitions.

**Parameters**:
- `message` (string): Message containing chunk definitions

**Returns**: array of CodeChunk

## Usage Examples

### Example 1: Basic Project Setup
```javascript
// Create a new code generation graph with 10000 line limit
const graph = new CodeGenerationGraph(10000);

// Add chunks with dependencies
graph.addChunk('project-setup', ChunkingLevel.Project, []);
graph.addChunk('database-module', ChunkingLevel.Module, ['project-setup']);
graph.addChunk('user-class', ChunkingLevel.Class, ['database-module']);
graph.addChunk('user-create-method', ChunkingLevel.Method, ['user-class']);

// Get ready chunks for processing
const readyChunks = graph.getReadyChunks();
console.log('Ready chunks:', readyChunks);

// Process the first ready chunk
const chunkId = graph.getNextChunkToWorkOn();
if (chunkId) {
    const chunk = graph.getChunk(chunkId);
    console.log(`Processing chunk: ${chunkId} at level ${chunk.level}`);
}
```

### Example 2: Project State Management
```javascript
const graph = new CodeGenerationGraph(5000);

// Update project state
graph.projectState.addCompletedModule('auth-module');
graph.projectState.addPendingModule('api-module');
graph.projectState.setGlobalVariable('DATABASE_URL', 'postgresql://localhost:5432/mydb');

// Update context window
graph.contextWindow.addImport('express');
graph.contextWindow.addFunctionSignature('createUser', 'function createUser(userData)');
graph.contextWindow.setCurrentClass('UserController');

// Get project summary
console.log(graph.getProjectSummary());
```

### Example 3: Chunk Processing Workflow
```javascript
const graph = new CodeGenerationGraph(2000);

// Add chunks
graph.addChunk('main-module', ChunkingLevel.Module, []);
graph.addChunk('config-class', ChunkingLevel.Class, ['main-module']);
graph.addChunk('init-method', ChunkingLevel.Method, ['config-class']);

// Process chunks in order
while (true) {
    const chunkId = graph.getNextChunkToWorkOn();
    if (!chunkId) break;
    
    const chunk = graph.getChunk(chunkId);
    
    // Simulate code generation
    const generatedCode = `// Generated code for ${chunkId}\nconsole.log('Hello from ${chunkId}');`;
    const generatedTests = `// Tests for ${chunkId}\ndescribe('${chunkId}', () => {\n  it('should work', () => {\n    expect(true).toBe(true);\n  });\n});`;
    
    // Update chunk
    graph.updateChunkCode(chunkId, generatedCode);
    graph.updateChunkTests(chunkId, generatedTests);
    graph.markChunkCompleted(chunkId);
    graph.markChunkValidated(chunkId);
    
    console.log(`Processed chunk: ${chunkId}`);
}

console.log('Project summary:');
console.log(graph.getProjectSummary());
```

### Example 4: Parsing Chunk Definitions
```javascript
// Parse individual chunk
const chunkText = '<chunk>user-model; class; User data model; database-module; class User {\n  constructor(name) {\n    this.name = name;\n  }\n}</chunk>';
try {
    const chunk = parseChunkingFormat(chunkText);
    console.log('Parsed chunk:', chunk.chunkId, chunk.level.toString());
} catch (error) {
    console.error('Failed to parse chunk:', error.message);
}

// Parse multiple chunks from message
const message = `
<chunk>auth-module; module; Authentication module; ; import express from 'express';</chunk>
<chunk>login-route; method; Login endpoint; auth-module; app.post('/login', (req, res) => {\n  res.send('Logged in');\n});</chunk>
`;

const chunks = processChunkingMessage(message);
console.log(`Parsed ${chunks.length} chunks`);
chunks.forEach(chunk => {
    console.log(`- ${chunk.chunkId} (${chunk.level})`);
});
```

### Example 5: Error Handling
```javascript
try {
    // This will throw an error
    const invalidLevel = ChunkingLevel.fromString('invalid-level');
} catch (error) {
    if (error instanceof TodoziError) {
        console.log('Caught TodoziError:', error.message);
    } else {
        console.log('Caught other error:', error.message);
    }
}

// Handle parsing errors gracefully
const invalidChunkText = '<chunk>missing-parts</chunk>';
try {
    const chunk = parseChunkingFormat(invalidChunkText);
} catch (error) {
    console.log('Parsing failed:', error.message);
}
```

## Design Patterns

### 1. Singleton Pattern
The static properties in `ChunkingLevel` and `ChunkStatus` implement a form of singleton pattern, ensuring consistent instances across the application.

### 2. Factory Pattern
The `fromString` methods in `ChunkingLevel` and `ChunkStatus` act as factory methods for creating instances from string representations.

### 3. Observer Pattern
The `updatedAt` property in various classes is automatically updated when relevant data changes, implementing a form of the observer pattern for tracking modifications.

### 4. Composite Pattern
The `CodeGenerationGraph` manages a collection of `CodeChunk` objects, creating a tree-like structure of dependencies.

### 5. State Pattern
The `ChunkStatus` enum and status management in `CodeChunk` implement a state pattern for managing chunk lifecycle.

### 6. Builder Pattern
The parsing functions (`parseChunkingFormat` and `processChunkingMessage`) implement a builder pattern for constructing `CodeChunk` objects from text representations.

## Performance Analysis

### Time Complexity
- **Chunk Creation**: O(1) - Constant time for creating individual chunks
- **Dependency Checking**: O(d) where d is the number of dependencies per chunk
- **Ready Chunks Calculation**: O(n × d) where n is the number of chunks and d is average dependencies
- **Dependency Chain Building**: O(n + e) where n is chunks and e is total dependencies (graph traversal)
- **Parsing**: O(s) where s is the size of the input string

### Space Complexity
- **Chunk Storage**: O(n) where n is the number of chunks
- **Project State**: O(m + v) where m is modules and v is global variables
- **Context Window**: O(i + f + e) where i is imports, f is functions, and e is error patterns
- **Dependency Chains**: O(d) where d is the depth of dependencies

### Performance Optimization Recommendations
1. **Caching Ready Chunks**: Cache the result of `getReadyChunks()` and invalidate only when chunk statuses change
2. **Index Dependencies**: Create an index of reverse dependencies for faster lookup
3. **Batch Updates**: Group multiple updates to reduce `updatedAt` timestamp updates
4. **Lazy Parsing**: Parse chunk content only when needed

### Memory Usage
- Each `CodeChunk`: ~200-500 bytes (depending on content)
- Each `ProjectState` entry: ~100-200 bytes
- Each `ContextWindow` entry: ~50-150 bytes
- For 1000 chunks: ~200KB-500KB memory footprint

## Security Considerations

### Input Validation
- All parsing functions include validation to prevent malformed input
- Error handling is implemented to gracefully handle invalid data
- String parsing is done with proper boundary checks

### Data Integrity
- Immutable date properties (`createdAt`) ensure audit trail integrity
- Status transitions follow a defined lifecycle to prevent invalid states
- Dependency relationships are validated to maintain graph consistency

### Potential Vulnerabilities
1. **XML External Entity (XXE)**: The parsing functions don't handle XML, but if extended to do so, XXE protection would be needed
2. **Denial of Service**: Large numbers of chunks or deeply nested dependencies could impact performance
3. **Memory Exhaustion**: Unbounded chunk creation could exhaust memory resources

### Security Best Practices
1. **Input Sanitization**: Always validate and sanitize external input before processing
2. **Resource Limits**: Implement limits on chunk count and dependency depth
3. **Audit Logging**: The built-in timestamping provides basic audit capabilities
4. **Error Handling**: Comprehensive error handling prevents information leakage

## Testing Strategies

### Unit Testing
The provided `runTests()` function demonstrates basic unit testing:

```javascript
function runTests() {
    // Test chunking levels
    console.assert(ChunkingLevel.Project.max_tokens() === 100);
    console.assert(ChunkingLevel.Module.max_tokens() === 500);
    // ... more assertions
    
    console.log('All tests passed!');
}
```

### Test Coverage Areas
1. **ChunkingLevel**: Test all level properties and fromString method
2. **ChunkStatus**: Test status transitions and string conversion
3. **ProjectState**: Test state updates and string formatting
4. **ContextWindow**: Test context updates and formatting
5. **CodeChunk**: Test lifecycle methods and dependency management
6. **CodeGenerationGraph**: Test chunk management and dependency resolution
7. **Parsing Functions**: Test valid and invalid input handling

### Integration Testing
```javascript
// Test complete workflow
function testCompleteWorkflow() {
    const graph = new CodeGenerationGraph(1000);
    
    // Add interdependent chunks
    graph.addChunk('A', ChunkingLevel.Module, []);
    graph.addChunk('B', ChunkingLevel.Class, ['A']);
    graph.addChunk('C', ChunkingLevel.Method, ['B']);
    
    // Verify dependency chain
    const chain = graph.getDependencyChain('C');
    console.assert(JSON.stringify(chain) === JSON.stringify(['A', 'B', 'C']));
    
    console.log('Workflow test passed!');
}
```

### Edge Case Testing
1. **Circular Dependencies**: Test behavior with circular dependencies
2. **Missing Dependencies**: Test chunks with non-existent dependencies
3. **Empty Content**: Test chunks with empty code/test content
4. **Large Token Counts**: Test behavior with chunks exceeding token limits
5. **Concurrent Updates**: Test thread safety (if used in multi-threaded environments)

### Performance Testing
```javascript
function performanceTest() {
    const startTime = Date.now();
    const graph = new CodeGenerationGraph(100000);
    
    // Create many chunks
    for (let i = 0; i < 1000; i++) {
        const deps = i > 0 ? [`chunk${i-1}`] : [];
        graph.addChunk(`chunk${i}`, ChunkingLevel.Method, deps);
    }
    
    const endTime = Date.now();
    console.log(`Created 1000 chunks in ${endTime - startTime}ms`);
    
    // Test ready chunks performance
    const startReady = Date.now();
    const ready = graph.getReadyChunks();
    const endReady = Date.now();
    console.log(`Found ${ready.length} ready chunks in ${endReady - startReady}ms`);
}
```

## Deployment Instructions

### Prerequisites
- Node.js version 12 or higher
- No external dependencies (pure JavaScript)

### Installation
1. Save the code to a file (e.g., `todozi.js`)
2. Import or require the module in your project:
   ```javascript
   // ES6 modules
   import { CodeGenerationGraph, ChunkingLevel } from './todozi.js';
   
   // CommonJS
   const { CodeGenerationGraph, ChunkingLevel } = require('./todozi.js');
   ```

### Configuration
The system requires minimal configuration:
```javascript
// Set maximum lines for project
const maxLines = 10000;
const graph = new CodeGenerationGraph(maxLines);

// Configure chunking levels if needed (they're predefined)
```

### Environment Variables
No environment variables are required, but you might want to set:
```bash
# For logging/debugging
export TODOZI_DEBUG=true
```

### Deployment Options

#### 1. Node.js Application
```javascript
const { CodeGenerationGraph, ChunkingLevel } = require('./todozi.js');

const app = express();
app.use(express.json());

app.post('/generate', (req, res) => {
    const { chunks, maxLines } = req.body;
    const graph = new CodeGenerationGraph(maxLines || 10000);
    
    // Process chunks...
    
    res.json({ summary: graph.getProjectSummary() });
});
```

#### 2. Browser Usage
```html
<script type="module">
    import { CodeGenerationGraph, ChunkingLevel } from './todozi.js';
    
    const graph = new CodeGenerationGraph(5000);
    // Use in browser...
</script>
```

#### 3. Library Integration
```javascript
// Package as npm module
export {
    TodoziError,
    ChunkingLevel,
    ChunkStatus,
    ProjectState,
    ContextWindow,
    CodeChunk,
    CodeGenerationGraph,
    parseChunkingFormat,
    processChunkingMessage
};
```

### Monitoring and Logging
```javascript
// Add logging to critical operations
class CodeGenerationGraph {
    addChunk(chunkId, level, deps) {
        console.log(`Adding chunk ${chunkId} at level ${level}`);
        // ... existing implementation
    }
    
    markChunkCompleted(chunkId) {
        console.log(`Marking chunk ${chunkId} as completed`);
        // ... existing implementation
    }
}
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. "Invalid chunking level" Error
**Problem**: `TodoziError: Invalid chunking level: xyz`
**Solution**: Check that the level string matches one of the predefined levels (project, module, class, method, block) and is correctly capitalized in the parsing context.

```javascript
// Correct usage
const level = ChunkingLevel.fromString('module'); // lowercase input is fine

// Incorrect usage that would cause issues in parsing
// Make sure your chunk definition uses valid level names
```

#### 2. Missing Dependencies
**Problem**: Chunks remain in pending state
**Solution**: Verify all dependencies exist and are correctly named.

```javascript
// Debug dependency issues
const graph = new CodeGenerationGraph(1000);
graph.addChunk('A', ChunkingLevel.Module, []);
graph.addChunk('B', ChunkingLevel.Class, ['A']);
graph.addChunk('C', ChunkingLevel.Method, ['B', 'nonexistent']); // This will cause issues

// Check for missing dependencies
const ready = graph.getReadyChunks();
console.log('Ready chunks:', ready); // 'C' won't be ready due to missing dependency
```

#### 3. Parsing Errors
**Problem**: `Error: Missing <chunk> start tag` or similar
**Solution**: Ensure chunk definitions follow the correct XML-like format.

```javascript
// Correct format
const correct = '<chunk>id; level; description; dependencies; code</chunk>';

// Common mistakes
const wrong1 = 'chunk>id; level</chunk>'; // Missing opening bracket
const wrong2 = '<chunk>id; level; description</chunk'; // Missing closing bracket
const wrong3 = '<chunk>id</chunk>'; // Too few parts
```

#### 4. Performance Issues with Large Projects
**Problem**: Slow operations with many chunks
**Solution**: Implement caching and optimize dependency checking.

```javascript
// Add caching for ready chunks
class OptimizedCodeGenerationGraph extends CodeGenerationGraph {
    constructor(maxLines) {
        super(maxLines);
        this._readyChunksCache = null;
        this._cacheInvalidated = true;
    }
    
    getReadyChunks() {
        if (this._cacheInvalidated || !this._readyChunksCache) {
            this._readyChunksCache = super.getReadyChunks();
            this._cacheInvalidated = false;
        }
        return this._readyChunksCache;
    }
    
    // Invalidate cache when chunks change
    markChunkCompleted(chunkId) {
        super.markChunkCompleted(chunkId);
        this._cacheInvalidated = true;
    }
}
```

### Debugging Techniques

#### 1. Enable Detailed Logging
```javascript
// Add verbose logging
const originalMethod = CodeChunk.prototype.markCompleted;
CodeChunk.prototype.markCompleted = function() {
    console.log(`Marking chunk ${this.chunkId} as completed`);
    return originalMethod.call(this);
};
```

#### 2. Inspect Internal State
```javascript
// Debug project state
console.log('Project State:', graph.projectState.toStateString());
console.log('Context Window:', graph.contextWindow.toContextString());

// Debug individual chunks
for (const [id, chunk] of graph.chunks) {
    console.log(`Chunk ${id}: Status=${chunk.status}, Dependencies=${chunk.dependencies}`);
}
```

#### 3. Validate Dependency Graph
```javascript
function validateDependencyGraph(graph) {
    const errors = [];
    
    for (const [chunkId, chunk] of graph.chunks) {
        // Check for circular dependencies
        if (hasCircularDependency(graph, chunkId)) {
            errors.push(`Circular dependency detected for chunk ${chunkId}`);
        }
        
        // Check for missing dependencies
        for (const dep of chunk.dependencies) {
            if (!graph.chunks.has(dep)) {
                errors.push(`Chunk ${chunkId} depends on non-existent chunk ${dep}`);
            }
        }
    }
    
    return errors;
}

function hasCircularDependency(graph, chunkId, visited = new Set(), path = []) {
    if (visited.has(chunkId)) return false;
    if (path.includes(chunkId)) return true;
    
    path.push(chunkId);
    const chunk = graph.chunks.get(chunkId);
    if (!chunk) return false;
    
    for (const dep of chunk.dependencies) {
        if (hasCircularDependency(graph, dep, visited, path)) {
            return true;
        }
    }
    
    path.pop();
    visited.add(chunkId);
    return false;
}
```

### Recovery Procedures

#### 1. Reset Chunk Status
```javascript
function resetFailedChunks(graph) {
    for (const [chunkId, chunk] of graph.chunks) {
        if (chunk.status === ChunkStatus.Failed) {
            chunk.status = ChunkStatus.Pending;
            console.log(`Reset failed chunk ${chunkId} to pending`);
        }
    }
}
```

#### 2. Rebuild Dependency Index
```javascript
function rebuildDependencyIndex(graph) {
    // This is automatically handled by the existing implementation
    // but you could add additional validation here
    console.log('Dependency index is automatically maintained');
}
```

#### 3. Export/Import State
```javascript
function exportGraphState(graph) {
    return {
        chunks: Array.from(graph.chunks.entries()).map(([id, chunk]) => ({
            id,
            status: chunk.status.toString(),
            dependencies: [...chunk.dependencies],
            code: chunk.code,
            tests: chunk.tests,
            level: chunk.level.toString(),
            createdAt: chunk.createdAt,
            updatedAt: chunk.updatedAt
        })),
        projectState: {
            totalLines: graph.projectState.totalLines,
            maxLines: graph.projectState.maxLines,
            completedModules: [...graph.projectState.completedModules],
            pendingModules: [...graph.projectState.pendingModules],
            globalVariables: Array.from(graph.projectState.globalVariables.entries())
        }
    };
}
```

This comprehensive documentation provides a complete understanding of the Todozi Code Generation System, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.