# Todozi Tool Executor Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [Error Handling](#error-handling)
5. [Execution Flow](#execution-flow)
6. [API Reference](#api-reference)
7. [Usage Examples](#usage-examples)
8. [Design Patterns](#design-patterns)
9. [Performance Analysis](#performance-analysis)
10. [Security Considerations](#security-considerations)
11. [Testing Strategies](#testing-strategies)
12. [Deployment Instructions](#deployment-instructions)
13. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi Tool Executor is a comprehensive JavaScript module designed to provide a unified interface for interacting with Todozi task management system. It offers both simple command-based actions and advanced API integrations, supporting various task creation, search, and management operations.

### Key Features
- **Multi-action Support**: 20+ different action types for task management
- **Error Handling**: Comprehensive error types with detailed messaging
- **Thread Safety**: Mutex-based synchronization for concurrent operations
- **API Integration**: Secure communication with todozi.com services
- **Flexible Execution**: Support for both local and remote task operations

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────────┐
│                    Todozi Tool Executor                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌──────────────────────┐               │
│  │   Main Entry    │    │   Error Handling     │               │
│  │ executeTodozi-  │    │   ExecutorError      │               │
│  │ ToolDelegated   │    │   ExecutionResult    │               │
│  └─────────────────┘    └──────────────────────┘               │
│           │                            │                       │
│           ▼                            ▼                       │
│  ┌─────────────────┐    ┌──────────────────────┐               │
│  │   Action Router │    │   Result Container   │               │
│  │   actionMap     │    │   ExecutionResult    │               │
│  └─────────────────┘    └──────────────────────┘               │
│           │                                                     │
│           ▼                                                     │
│  ┌─────────────────┐    ┌──────────────────────┐               │
│  │   Action        │    │   System Management  │               │
│  │   Executors     │    │   ensureTodoziSystem │               │
│  │   (20+ types)   │    │   Mutex Synchronization│              │
│  └─────────────────┘    └──────────────────────┘               │
│           │                                                     │
│           ▼                                                     │
│  ┌─────────────────┐    ┌──────────────────────┐               │
│  │   API Layer     │    │   Todozi Core        │               │
│  │   makeTodozi-   │    │   (Assumed External) │               │
│  │   Request       │    │   Done.*, Storage    │               │
│  └─────────────────┘    └──────────────────────┘               │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow
1. **Input Processing**: Parameters are validated and routed to appropriate action handlers
2. **System Initialization**: Todozi system is initialized with thread-safe mutex protection
3. **Action Execution**: Specific action handler processes the request
4. **API Integration**: Remote operations use authenticated HTTP requests
5. **Result Packaging**: Execution results are wrapped in ExecutionResult objects
6. **Error Handling**: All errors are converted to ExecutorError instances

## Core Components

### ExecutorError Class
Custom error class for handling different types of execution failures.

**Constructor:**
```javascript
new ExecutorError(type, message)
```

**Parameters:**
- `type` (string): Error type identifier
- `message` (string): Human-readable error description

**Error Types:**
- `ExecutionError`: General execution failures
- `BashToolError`: API/tool communication errors
- `MissingParameter`: Required parameter not provided
- `UnknownAction`: Unsupported action type

### ExecutionResult Class
Container for execution results with metadata support.

**Constructor:**
```javascript
new ExecutionResult(options = {})
```

**Options Parameters:**
- `success` (boolean): Execution success status (default: false)
- `output` (string): Human-readable output (default: '')
- `error` (Error|null): Error object if any (default: null)
- `metadata` (Object): Additional execution metadata (default: {})
- `tool_used` (string): Tool identifier (default: '')
- `execution_type` (string): Type of execution performed (default: '')

### Global State Management
Thread-safe global state management using Mutex synchronization.

**Variables:**
- `TDZ_SYSTEM`: Todozi system instance
- `TDZ_EMBEDDING_SERVICE`: Embedding service reference
- `systemMutex`: Mutex for thread-safe initialization

## Error Handling

The system implements a comprehensive error handling strategy:

### Error Types Classification
1. **Validation Errors**: Missing parameters, invalid input
2. **System Errors**: Initialization failures, storage issues
3. **API Errors**: Network failures, authentication issues
4. **Execution Errors**: Task creation failures, processing errors

### Error Propagation
All errors are wrapped in `ExecutorError` instances with appropriate type classification, ensuring consistent error handling across the application.

## Execution Flow

### Main Execution Path
1. **Parameter Validation**: Check for required `action` parameter
2. **System Initialization**: Ensure Todozi system is properly initialized
3. **Action Routing**: Map action to appropriate executor function
4. **Execution**: Run the specific action handler
5. **Result Packaging**: Wrap results in ExecutionResult object
6. **Return**: Provide structured response to caller

### Action Execution Process
Each action follows this pattern:
1. Validate required parameters
2. Execute core operation (local or remote)
3. Handle success/failure cases
4. Package results with metadata
5. Return ExecutionResult

## API Reference

### Main Functions

#### executeTodoziToolDelegated(params)
Main entry point for executing Todozi operations.

**Parameters:**
- `params` (Object): Execution parameters
  - `action` (string): Required action type
  - `content` (string): Content for the action (varies by action type)
  - `extra` (string): Additional context information
  - Other action-specific parameters

**Returns:**
- `Promise<ExecutionResult>`: Execution result with success status and output

**Throws:**
- `ExecutorError`: For various error conditions

#### ensureTodoziSystem()
Initialize Todozi system with thread-safe mutex protection.

**Returns:**
- `Promise<void>`: Resolves when system is ready

#### makeTodoziRequest(endpoint, payload)
Make authenticated requests to todozi.com API.

**Parameters:**
- `endpoint` (string): API endpoint path
- `payload` (Object): Request payload

**Returns:**
- `Promise<Object>`: API response data

### Action Executors

#### Task Creation Actions
- `executeSimpleTask`: Create standard task
- `executeUrgentTask`: Create urgent priority task
- `executeHighTask`: Create high priority task
- `executeLowTask`: Create low priority task
- `executeAiTask`: Create AI-assigned task
- `executeHumanTask`: Create human-assigned task
- `executeCollabTask`: Create collaborative task

#### Search Actions
- `executeFind`: Smart search (AI + keyword)
- `executeAiSearch`: Semantic search only
- `executeFastSearch`: Keyword-only search
- `executeSmartSearch`: Intent-aware search

#### Memory Actions
- `executeRemember`: Save medium importance memory
- `executeImportantMemory`: Save high importance memory

#### Idea Actions
- `executeIdea`: Save standard idea
- `executeBreakthroughIdea`: Save breakthrough idea

#### Task Management Actions
- `executeComplete`: Mark task as completed
- `executeStart`: Start task execution
- `executeStats`: Get system statistics
- `executeQueue`: Get queue status

#### Communication Actions
- `executeChat`: Process chat content
- `executeExtractApi`: Extract structured content from messages
- `executeExpandApi`: Expand tasks with AI assistance
- `executePlanApi`: AI project planning
- `executeStrategyApi`: AI strategic planning

## Usage Examples

### Basic Task Creation
```javascript
const { executeTodoziToolDelegated } = require('./todozi-executor');

// Create a simple task
const result = await executeTodoziToolDelegated({
    action: 'task',
    content: 'Complete project documentation'
});
console.log(result.output); // "✅ Task created: task_12345"

// Create an urgent task
const urgentResult = await executeTodoziToolDelegated({
    action: 'urgent',
    content: 'Fix critical bug in production'
});
console.log(urgentResult.output); // "🚨 Urgent task created: task_67890"
```

### Search Operations
```javascript
// Smart search
const searchResult = await executeTodoziToolDelegated({
    action: 'find',
    content: 'project documentation'
});
console.log(searchResult.output);

// AI semantic search
const aiSearchResult = await executeTodoziToolDelegated({
    action: 'ai_search',
    content: 'machine learning algorithms'
});
console.log(aiSearchResult.output);
```

### Memory and Idea Management
```javascript
// Save a memory
const memoryResult = await executeTodoziToolDelegated({
    action: 'remember',
    content: 'Important meeting notes',
    extra: 'Client requirements discussion'
});

// Save a breakthrough idea
const ideaResult = await executeTodoziToolDelegated({
    action: 'breakthrough_idea',
    content: 'Revolutionary approach to data processing'
});
```

### API Integration Examples
```javascript
// Extract structured content from message
const extractResult = await executeTodoziToolDelegated({
    action: 'extract',
    content: 'Please create tasks for the marketing campaign and remember the budget constraints',
    extra: 'Marketing department Q4 campaign'
});

// AI task expansion
const expandResult = await executeTodoziToolDelegated({
    action: 'expand',
    content: 'Develop new feature',
    extra: 'Mobile application user authentication'
});
```

### System Management
```javascript
// Get system statistics
const statsResult = await executeTodoziToolDelegated({
    action: 'stats'
});
console.log(statsResult.output);

// Check queue status
const queueResult = await executeTodoziToolDelegated({
    action: 'queue'
});
console.log(queueResult.output);
```

## Design Patterns

### Strategy Pattern
The action routing system implements the Strategy pattern, where each action type is handled by a specific strategy function. This allows for easy extension and maintenance.

### Factory Pattern
The `ExecutionResult` class acts as a factory for creating standardized result objects with consistent structure.

### Singleton Pattern
Global system state (`TDZ_SYSTEM`, `TDZ_EMBEDDING_SERVICE`) follows the Singleton pattern with thread-safe initialization.

### Decorator Pattern
Error handling wraps all operations with consistent error conversion and reporting.

### Observer Pattern
The execution flow follows an observer-like pattern where actions are observed and results are reported through structured objects.

## Performance Analysis

### Time Complexity
- **Action Routing**: O(1) - Direct map lookup
- **System Initialization**: O(1) amortized - Once per application lifecycle
- **Task Creation**: O(1) - Direct API/storage call
- **Search Operations**: O(n) - Depends on dataset size
- **API Requests**: O(1) network + processing time

### Space Complexity
- **Global State**: O(1) - Fixed number of references
- **Execution Results**: O(n) - Where n is result data size
- **Error Objects**: O(1) - Fixed structure

### Performance Considerations
1. **Mutex Overhead**: System initialization uses mutex for thread safety
2. **Network Latency**: API calls introduce network dependency
3. **Memory Usage**: Result objects maintain metadata for debugging
4. **Caching**: System initialization is cached to avoid repeated setup

### Optimization Opportunities
1. **Connection Pooling**: Reuse HTTP connections for API requests
2. **Result Caching**: Cache frequent search results
3. **Batch Operations**: Group multiple operations for efficiency
4. **Asynchronous Loading**: Pre-load system components

## Security Considerations

### Authentication Security
- API keys are retrieved through secure channels
- Authorization headers use Bearer token pattern
- Keys are not exposed in logs or error messages

### Input Validation
- All parameters are validated before processing
- Content sanitization prevents injection attacks
- Size limits prevent resource exhaustion

### Error Handling Security
- Error messages don't expose internal system details
- Sensitive information is filtered from logs
- Consistent error responses prevent information leakage

### Network Security
- HTTPS enforced for all API communications
- Certificate validation ensures endpoint authenticity
- Timeout mechanisms prevent hanging connections

### Data Protection
- Metadata doesn't contain sensitive user information
- Execution results are properly scoped to calling context
- No persistent storage of sensitive operational data

## Testing Strategies

### Unit Testing
```javascript
// Test action routing
test('should route to correct executor', async () => {
    const params = { action: 'task', content: 'test task' };
    const result = await executeTodoziToolDelegated(params);
    expect(result.success).toBe(true);
    expect(result.tool_used).toBe('todozi_simple');
});

// Test error handling
test('should handle missing parameters', async () => {
    const params = { action: 'task' }; // missing content
    await expect(executeTodoziToolDelegated(params))
        .rejects.toThrow(ExecutorError);
});
```

### Integration Testing
```javascript
// Test system initialization
test('should initialize Todozi system', async () => {
    await ensureTodoziSystem();
    expect(getStorage()).toBeDefined();
});

// Test API integration
test('should make authenticated requests', async () => {
    const result = await makeTodoziRequest('/api/test', { test: true });
    expect(result).toBeDefined();
});
```

### Mock Testing
```javascript
// Mock external dependencies
jest.mock('axios');
jest.mock('./todozi-core'); // assumed external module

beforeEach(() => {
    axios.post.mockResolvedValue({ data: { success: true } });
});
```

### Performance Testing
```javascript
// Test execution time
test('should execute within time limits', async () => {
    const start = Date.now();
    await executeTodoziToolDelegated({ action: 'stats' });
    const duration = Date.now() - start;
    expect(duration).toBeLessThan(5000); // 5 second limit
});
```

### Security Testing
```javascript
// Test input sanitization
test('should handle malicious input', async () => {
    const maliciousInput = '<script>alert("xss")</script>';
    const result = await executeTodoziToolDelegated({
        action: 'task',
        content: maliciousInput
    });
    // Should not execute script, should sanitize input
    expect(result.success).toBe(true);
});
```

## Deployment Instructions

### Prerequisites
1. Node.js v14+ installed
2. npm or yarn package manager
3. Network access to todozi.com
4. Valid Todozi API credentials

### Installation
```bash
# Install dependencies
npm install async-mutex axios

# Install the module (assuming local development)
npm install ./path/to/todozi-executor
```

### Configuration
1. Ensure Todozi API key is available through expected channels
2. Verify network connectivity to todozi.com
3. Set up proper error logging and monitoring

### Environment Variables
```bash
# API configuration (if applicable)
TODOZI_API_KEY=your_api_key_here
TODOZI_API_URL=https://todozi.com
```

### Integration Example
```javascript
// In your application
const { executeTodoziToolDelegated } = require('todozi-executor');

async function integrateTodozi() {
    try {
        const result = await executeTodoziToolDelegated({
            action: 'task',
            content: 'Integrated task creation'
        });
        console.log('Task created:', result.output);
    } catch (error) {
        console.error('Integration failed:', error.toString());
    }
}
```

### Monitoring and Logging
1. Implement structured logging for all executions
2. Monitor API usage and rate limits
3. Track error rates and patterns
4. Set up alerts for system initialization failures

## Troubleshooting Guide

### Common Issues

#### System Initialization Failures
**Symptom**: `ExecutionError: Failed to initialize Todozi`
**Solutions**:
1. Check network connectivity to required services
2. Verify Todozi core module is properly installed
3. Ensure sufficient system resources
4. Check for conflicting global state

#### API Authentication Errors
**Symptom**: `BashToolError: API request failed with status: 401`
**Solutions**:
1. Verify API key is correctly configured
2. Check API key expiration
3. Ensure proper permissions for the key
4. Validate API endpoint URLs

#### Missing Parameter Errors
**Symptom**: `MissingParameter: content`
**Solutions**:
1. Check required parameters for the action type
2. Validate parameter structure and naming
3. Ensure content is not empty or null
4. Review action-specific requirements

#### Network Timeouts
**Symptom**: `BashToolError: Request failed: timeout`
**Solutions**:
1. Check network connectivity
2. Increase timeout thresholds if appropriate
3. Implement retry logic for transient failures
4. Monitor API service status

### Debugging Steps

1. **Enable Verbose Logging**
   ```javascript
   // Add detailed logging
   console.log('Debug: Executing action', params.action);
   ```

2. **Check System State**
   ```javascript
   // Verify system initialization
   console.log('System initialized:', !!TDZ_SYSTEM);
   ```

3. **Validate Parameters**
   ```javascript
   // Add parameter validation logging
   console.log('Parameters:', JSON.stringify(params, null, 2));
   ```

4. **Test Connectivity**
   ```javascript
   // Test API connectivity
   try {
       await makeTodoziRequest('/api/health', {});
       console.log('API connectivity: OK');
   } catch (error) {
       console.error('API connectivity: FAILED', error.message);
   }
   ```

### Recovery Procedures

#### System Reinitialization
```javascript
// Force system reinitialization
TDZ_SYSTEM = null;
await ensureTodoziSystem();
```

#### API Key Refresh
```javascript
// Clear cached API key and retry
// Implementation depends on key storage mechanism
```

#### Error State Reset
```javascript
// Clear error state and retry operation
// Handle specific error types appropriately
```

### Performance Monitoring

#### Key Metrics to Monitor
1. **Execution Time**: Average time per action type
2. **Error Rate**: Percentage of failed executions
3. **API Usage**: Request volume and response times
4. **System Health**: Initialization success rate

#### Alerting Thresholds
- Execution time > 5 seconds
- Error rate > 5%
- API failures > 10 consecutive requests
- System initialization failures > 3 in 5 minutes

This comprehensive documentation provides a complete understanding of the Todozi Tool Executor, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.