# Todozi JavaScript SDK

A comprehensive task management and AI-powered productivity system built with Node.js. This SDK provides structured parsing, execution, and management of various types of content including tasks, memories, ideas, errors, and more.

## Table of Contents

- [Overview](#overview)
- [Core Modules](#core-modules)
  - [Agent Management](#agent-management)
  - [API Key Management](#api-key-management)
  - [Base Tool Framework](#base-tool-framework)
  - [Chunking System](#chunking-system)
  - [CLI Handler](#cli-handler)
  - [Embedding Service](#embedding-service)
  - [Error Management](#error-management)
  - [Content Extraction](#content-extraction)
  - [Idea Management](#idea-management)
  - [Models & Data Structures](#models--data-structures)
  - [Todozi Core](#todozi-core)
  - [Server](#server)
  - [Additional Modules](#additional-modules)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Contributing](#contributing)

## Overview

Todozi is a sophisticated task management platform that combines traditional task tracking with AI-powered features. The system provides:

- **Task Management**: Create, update, and track tasks with rich metadata
- **AI Agent System**: 26 pre-configured agents for automated task handling
- **Semantic Search**: Find similar tasks and content using embeddings
- **Memory Management**: Store and retrieve contextual memories
- **Idea Tracking**: Capture and organize ideas with importance ratings
- **Queue System**: Prioritize and process tasks in workflows
- **API Management**: Secure API key generation and authentication
- **Content Extraction**: Extract structured data from unstructured text

---

## Core Modules

### Agent Management

The Agent Management System provides comprehensive functionality for managing AI agents, their assignments, and lifecycle within the task management platform. This system enables intelligent task assignment by matching agent capabilities and specializations with task requirements.

**Key Features:**
- Agent lifecycle management (create, update, delete) with UUID-based identification
- Agent assignment to tasks with status tracking (Assigned, InProgress, Completed)
- Agent availability management (Available, Busy, Inactive)
- Specialization and capability-based agent filtering
- Performance statistics tracking with completion rate calculations
- Agent assignment parsing from formatted text
- Best agent selection algorithm based on requirements
- Agent status updates and lifecycle management
- Assignment history tracking

**Architecture:**
The system uses a singleton `AgentManager` class that maintains a Map of agents (keyed by agent ID) and an array of assignments. Agents can be filtered by specialization, capability, and availability status. The system supports finding the best agent for a given task based on required specializations and preferred capabilities using a scoring algorithm that considers availability, specialization match, and capability match.

**Data Models:**
- `Agent`: Core agent entity with id, name, description, capabilities, specializations, metadata (status), timestamps
- `AgentUpdate`: Builder pattern class for constructing agent updates with fluent interface
- `AgentStatistics`: Aggregated statistics with completion rate, total assignments, and status distribution
- `AgentAssignment`: Assignment entity linking agents to tasks with project context, timestamps, and status
- `AgentStatus`: Enum for agent status (Available, Busy, Inactive)
- `AssignmentStatus`: Enum for assignment status (Assigned, InProgress, Completed, Failed)

**Usage Example:**
```javascript
const agentManager = new AgentManager();
await agentManager.loadAgents();

// Create a new agent
const agentId = await agentManager.createAgent({
    name: "Research Assistant",
    description: "AI agent specialized in research tasks",
    capabilities: ["research", "analysis", "data-collection"],
    specializations: ["academic", "scientific", "technical"],
    metadata: {
        status: "available"
    }
});

// Update agent using builder pattern
const update = AgentUpdate.new()
    .name("Advanced Research Assistant")
    .description("Enhanced research capabilities")
    .capabilities(["research", "analysis", "data-collection", "writing"])
    .specializations(["academic", "scientific", "technical", "medical"])
    .status(AgentStatus.Available);

await agentManager.updateAgent(agentId, update);

// Find best agent for a task
const bestAgent = agentManager.findBestAgent("academic", "research");
if (bestAgent) {
    await agentManager.assignTaskToAgent(taskId, bestAgent.id, projectId);
    console.log(`Task assigned to ${bestAgent.name}`);
}

// Get agent assignments
const assignments = agentManager.getAgentAssignments(agentId);
console.log(`Agent has ${assignments.length} assignments`);

// Complete assignment
await agentManager.completeAgentAssignment(taskId);

// Get statistics
const stats = agentManager.getAgentStatistics();
console.log(`Total agents: ${stats.total_agents}`);
console.log(`Available: ${stats.available_agents}`);
console.log(`Completion rate: ${stats.completionRate()}%`);
```

**Agent Selection Algorithm:**
The `findBestAgent` method uses a scoring algorithm:
1. Filters agents by required specialization
2. Scores agents based on capability match (preferred capability gets bonus)
3. Prioritizes available agents over busy ones
4. Returns the highest-scoring agent or null if none found

**Performance:** Operations are O(1) for most agent lookups using hash maps, with O(n) for filtering operations. Agent selection is O(n log n) due to sorting. The system is optimized for in-memory operations with efficient data structures.

**Design Patterns:** Singleton pattern for AgentManager, Builder pattern for AgentUpdate, Repository pattern for data access, Factory pattern for agent creation, Strategy pattern for agent selection.

**Storage Integration:** Agents are loaded from storage on initialization. The system supports default agent creation if no agents exist. All agent operations can be persisted through the storage layer.

**Error Handling:** The system throws `TodoziError` for invalid operations such as assigning to unavailable agents, updating non-existent agents, or completing non-existent assignments.

---

### API Key Management

The Todozi API Key Management System handles secure API key generation, authentication, and lifecycle management with a dual-key authentication approach for enhanced security.

**Key Features:**
- API key generation with cryptographically secure random identifiers (32-byte URL-safe tokens)
- User-based API key association with UUID identifiers
- Public/private key authentication mechanism for layered security
- Key activation/deactivation management without deletion
- Persistent storage using JSON files with atomic operations
- Error handling with custom error types
- Admin-level authentication requiring both keys
- Dual-indexed collections for O(1) lookups by user_id or public_key
- Key cloning to prevent external modification

**Architecture:**
The system uses an `ApiKeyManager` class that manages an `ApiKeyCollection`. Each API key has a public key (32-byte identifier) and private key (32-byte authentication token). Keys can be associated with users and have admin privileges. The system supports activation/deactivation without deletion, allowing temporary key suspension. The collection maintains dual indices for efficient lookups.

**Data Models:**
- `ApiKey`: Core API key entity with user_id, public_key, private_key, active status
- `ApiKeyCollection`: Collection manager with dual indexing (_keys_by_user_id, _keys_by_public)
- `ApiKeyManager`: Main manager class (currently minimal, functions are exported directly)

**Usage Example:**
```javascript
// Create a new API key without user association
const newKey = createApiKey();
console.log('Public Key:', newKey.publicKey);
console.log('Private Key:', newKey.privateKey); // Store securely!

// Create user-specific key
const userKey = createApiKeyWithUserId('user123');
console.log('User ID:', userKey.userId);

// Retrieve API key by user ID
const retrievedKey = getApiKey('user123');
console.log('Retrieved Public Key:', retrievedKey.publicKey);

// Retrieve by public key
const keyByPublic = getApiKeyByPublic(userKey.publicKey);

// List all keys
const allKeys = listApiKeys();
console.log(`Total keys: ${allKeys.length}`);

// List only active keys
const activeKeys = listActiveApiKeys();
console.log(`Active keys: ${activeKeys.length}`);

// Authenticate using key pair
const [userId, isAdmin] = checkApiKeyAuth(
    userKey.publicKey, 
    userKey.privateKey
);
console.log(`Authenticated user: ${userId}, Admin: ${isAdmin}`);

// Deactivate key (temporary suspension)
deactivateApiKey('user123');

// Reactivate key
activateApiKey('user123');

// Remove key permanently
const removedKey = removeApiKey('user123');
```

**Security Considerations:**
- Keys use cryptographically secure random generation via `crypto.randomBytes()` or equivalent
- 32-byte keys provide 256 bits of entropy (recommended for security)
- Private keys should NEVER be exposed in logs, error messages, or API responses
- Keys are cloned before returning to prevent external modification
- File permissions should be restricted (chmod 600) for API key storage files
- Keys stored in JSON files - ensure proper file system security
- Consider key rotation policies for long-term security

**Performance:** O(1) for key creation and lookups using dual-indexed dictionaries, O(n) for listing operations. File I/O operations are performed on each key management action. The system uses efficient hash map lookups for both user_id and public_key indices.

**Storage:** Default storage location is `{storageDir}/api/api_keys.json`. The system uses atomic file operations to prevent corruption. Keys are serialized as JSON arrays with all key properties.

**Error Handling:** The system throws `TodoziError.ValidationError` for operations on non-existent keys or invalid key pairs. All errors include descriptive messages for debugging.

**Design Patterns:** Factory pattern for key creation, Repository pattern for collection management, Decorator pattern for key cloning, Singleton pattern for storage access.

---

### Base Tool Framework

A comprehensive framework for defining, managing, and executing tools in AI applications, particularly with language models like Ollama. This framework provides a structured approach to tool creation with validation, resource management, and standardized result handling.

**Key Features:**
- Tool parameter definitions with type validation and required/optional flags
- Resource locking system for concurrent operation safety (FilesystemWrite, FilesystemRead, Git, Memory, Shell, Network)
- Tool definitions with categories, descriptions, and metadata
- Tool result handling with success/error states, execution time tracking, and metadata
- Comprehensive error types (ValidationError, PermissionError, FileNotFound, TimeoutError, ResourceError, NetworkError, SecurityError, InternalError)
- Tool registry for managing tool collections with registration and execution
- Ollama format conversion for function calling API compatibility
- Parameter validation with type checking and constraint validation
- Error handler utilities for consistent error processing

**Architecture:**
The framework consists of several core classes: `ToolParameter` for parameter definitions with type and validation metadata, `ToolDefinition` for complete tool specifications including parameters and resource locks, `ToolResult` for standardized execution results, `ToolError` for tool-specific errors with detailed metadata, `ErrorHandler` for centralized error processing, and `ToolRegistry` for managing collections of tools. Tools extend a base `Tool` abstract class and must implement `definition()` and `execute()` methods.

**Data Models:**
- `ToolParameter`: Parameter definition with name, type, description, required flag, and optional default value
- `ToolDefinition`: Complete tool specification with name, description, parameters array, category, and resource_locks array
- `ToolResult`: Execution result with success flag, output string, error string, execution_time_ms, optional metadata, and optional recovery_context
- `ToolError`: Custom error with message, error_type (ErrorType enum), and optional details object
- `ResourceLock`: Enum for resource types (FilesystemWrite, FilesystemRead, Git, Memory, Shell, Network)
- `ErrorType`: Enum for error categories
- `Tool`: Abstract base class for all tools
- `ToolRegistry`: Registry for managing and executing tools

**Usage Example:**
```javascript
// Define a custom tool
class FileReadTool extends Tool {
    definition() {
        return new ToolDefinition(
            'file_read',
            'Read the contents of a file',
            [
                new ToolParameter('path', 'string', 'Path to the file to read', true),
                new ToolParameter('encoding', 'string', 'File encoding', false, 'utf8')
            ],
            'File Operations',
            [ResourceLock.FilesystemRead]
        );
    }
    
    async execute(kwargs) {
        const startTime = Date.now();
        
        try {
            // Validate required parameters
            const validationError = ErrorHandler.validateRequiredParams(
                kwargs, 
                ['path']
            );
            if (validationError) return validationError;
            
            // Validate string parameter
            const pathValidation = ErrorHandler.validateStringParam(
                kwargs.path, 
                'path', 
                1, 
                1000
            );
            if (pathValidation) return pathValidation;
            
            // Simulate file reading
            const fs = require('fs');
            const content = fs.readFileSync(kwargs.path, kwargs.encoding || 'utf8');
            
            return ErrorHandler.createSuccessResult(
                content,
                Date.now() - startTime,
                { fileSize: content.length }
            );
        } catch (error) {
            return ErrorHandler.handleError(error, 'FileReadTool.execute');
        }
    }
}

// Register and use tools
const registry = new ToolRegistry();
registry.register(new FileReadTool());

// Get tool definitions in Ollama format
const ollamaDefs = registry.getToolDefinitions();
console.log(JSON.stringify(ollamaDefs, null, 2));

// Execute tool
const result = await registry.executeTool('file_read', { 
    path: '/path/to/file.txt',
    encoding: 'utf8'
});

if (result.success) {
    console.log('File content:', result.output);
    console.log(`Execution time: ${result.execution_time_ms}ms`);
} else {
    console.error('Error:', result.error);
}

// Check if tool exists
if (registry.hasTool('file_read')) {
    console.log('Tool is registered');
}

// Get all tools
const allTools = registry.getAllTools();
console.log(`Registered tools: ${allTools.length}`);
```

**Ollama Format Conversion:**
Tools can be converted to Ollama function calling format:
```javascript
const ollamaFormat = toolDefinition.toOllamaFormat();
// Returns:
// {
//     type: "function",
//     function: {
//         name: "tool_name",
//         description: "tool_description",
//         parameters: {
//             type: "object",
//             properties: { ... },
//             required: [ ... ]
//         }
//     }
// }
```

**Error Handling:**
The framework provides comprehensive error handling through `ErrorHandler` class:
- `handleError(error, context)`: Converts errors to ToolResult
- `validateRequiredParams(kwargs, required_params)`: Validates required parameters
- `validateStringParam(value, param_name, min_length, max_length, pattern)`: Validates string parameters
- `createSuccessResult(output, execution_time_ms, metadata)`: Creates success result
- `createErrorResult(error_msg, execution_time_ms, error_type, metadata)`: Creates error result

**Performance:** O(1) for tool registration and retrieval using Maps. Parameter validation is O(n) where n is the number of parameters. Tool execution time depends on implementation. The registry uses efficient hash map lookups.

**Design Patterns:** Factory pattern for tool creation (static `new()` methods), Registry pattern for tool management, Template Method pattern for tool interface (abstract Tool class), Builder pattern for complex objects (helper functions), Comprehensive error handling strategy with ErrorHandler.

**Resource Locking:** The resource lock system helps prevent race conditions and unauthorized access. Tools declare required locks, and the system can enforce these locks during execution to ensure safe concurrent operations.

**Helper Functions:**
- `createToolParameter(name, type, description, required)`: Creates parameter without default
- `createToolParameterWithDefault(name, type, description, required, defaultValue)`: Creates parameter with default
- `createToolDefinition(name, description, category, parameters)`: Creates definition without locks
- `createToolDefinitionWithLocks(name, description, category, parameters, resource_locks)`: Creates definition with locks

---

### Chunking System

The Todozi Code Generation System manages and orchestrates code generation tasks through a structured chunking approach with hierarchical levels, dependency management, and comprehensive state tracking for complex software development workflows.

**Key Features:**
- Hierarchical code chunking with predefined levels (Project: 100 tokens, Module: 500 tokens, Class: 1000 tokens, Method: 300 tokens, Block: 100 tokens)
- Dependency management between code chunks with graph-based relationships
- Project state tracking with line count, module completion, and global variables
- Context window management for maintaining generation context (previous/current class, imports, function signatures, error patterns)
- Chunk status tracking through defined lifecycle (Pending, InProgress, Completed, Validated, Failed)
- XML-based chunk definition parsing from formatted text
- Token estimation for chunks based on code content
- Dependency chain resolution for ordered processing
- Ready chunks identification (dependencies satisfied)

**Architecture:**
The system uses a `CodeGenerationGraph` that manages `CodeChunk` objects with dependencies stored as a directed graph. Each chunk has a `ChunkingLevel` (with token limits and descriptions) and `ChunkStatus` (lifecycle state). The system maintains `ProjectState` for overall project tracking (total lines, max lines, current module, dependencies, completed/pending modules, global variables) and `ContextWindow` for generation context (previous/current class, next planned items, global vars in scope, imports, function signatures, error patterns).

**Data Models:**
- `ChunkingLevel`: Enum defining hierarchical levels with token limits, descriptions, and examples
- `ChunkStatus`: Enum for chunk lifecycle states (Pending, InProgress, Completed, Validated, Failed)
- `CodeChunk`: Individual code unit with chunkId, status, dependencies array, code string, tests string, validated flag, level, estimatedTokens, timestamps
- `ProjectState`: Project state manager with totalLines, maxLines, currentModule, dependencies array, completedModules set, pendingModules set, globalVariables Map
- `ContextWindow`: Context manager with previousClass, currentClass, nextPlanned, globalVarsInScope array, importsUsed array, functionSignatures Map, errorPatternsSeen array
- `CodeGenerationGraph`: Main orchestrator managing chunks Map, projectState, and contextWindow

**Usage Example:**
```javascript
const graph = new CodeGenerationGraph(10000);

// Add chunks with dependencies
graph.addChunk('project-setup', ChunkingLevel.Project, []);
graph.addChunk('database-module', ChunkingLevel.Module, ['project-setup']);
graph.addChunk('user-class', ChunkingLevel.Class, ['database-module']);
graph.addChunk('user-create-method', ChunkingLevel.Method, ['user-class']);

// Get ready chunks (dependencies satisfied)
const readyChunks = graph.getReadyChunks();
console.log('Ready chunks:', readyChunks);

// Get next chunk to work on
const nextChunkId = graph.getNextChunkToWorkOn();
if (nextChunkId) {
    const chunk = graph.getChunk(nextChunkId);
    console.log(`Processing: ${chunk.chunkId} at level ${chunk.level.toString()}`);
    
    // Update chunk code
    const generatedCode = `class User {\n  constructor(name) {\n    this.name = name;\n  }\n}`;
    graph.updateChunkCode(nextChunkId, generatedCode);
    
    // Update tests
    const generatedTests = `describe('User', () => {\n  it('should create user', () => {\n    // test\n  });\n});`;
    graph.updateChunkTests(nextChunkId, generatedTests);
    
    // Mark as completed
    graph.markChunkCompleted(nextChunkId);
    
    // Mark as validated
    graph.markChunkValidated(nextChunkId);
}

// Get dependency chain
const chain = graph.getDependencyChain('user-create-method');
// Returns: ['project-setup', 'database-module', 'user-class', 'user-create-method']

// Get chunks by level
const moduleChunks = graph.getChunksByLevel(ChunkingLevel.Module);

// Get project summary
const summary = graph.getProjectSummary();
console.log(summary);

// Update project state
graph.projectState.addCompletedModule('database-module');
graph.projectState.addPendingModule('api-module');
graph.projectState.setGlobalVariable('DATABASE_URL', 'postgresql://localhost:5432/mydb');
graph.projectState.incrementLines(150);

// Update context window
graph.contextWindow.addImport('express');
graph.contextWindow.addFunctionSignature('createUser', 'function createUser(userData)');
graph.contextWindow.setCurrentClass('UserController');
graph.contextWindow.addErrorPattern('TypeError: Cannot read property');
```

**Chunk Format Parsing:**
Chunks can be parsed from XML-like format:
`<chunk>chunk_id; level; description; dependencies; code</chunk>`

**Performance:** O(1) for chunk creation, O(n×d) for ready chunks calculation where n is chunks and d is average dependencies, O(n+e) for dependency chain building where e is total dependencies (graph traversal). The system uses efficient graph traversal algorithms with visited set tracking to prevent cycles.

**Design Patterns:** Factory pattern for chunk creation (`ChunkingLevel.fromString()`), Composite pattern for chunk relationships (graph structure), State pattern for chunk lifecycle (`ChunkStatus`), Builder pattern for parsing (`parseChunkingFormat`), Observer pattern for state updates (automatic `updatedAt` timestamps).

**Optimization Recommendations:**
1. Cache ready chunks result and invalidate only when chunk statuses change
2. Create reverse dependency index for faster lookup
3. Batch updates to reduce timestamp updates
4. Lazy parse chunk content only when needed

**Error Handling:** The system throws `TodoziError` for invalid chunking levels, missing dependencies, and parsing failures. All operations validate inputs and provide descriptive error messages.

---

### CLI Handler

The Todozi Handler provides comprehensive CLI-based functionality for managing tasks, projects, agents, and AI-powered features through a unified command interface with extensive command support and integration capabilities.

**Key Features:**
- Task and project management with full CRUD operations
- API key generation, authentication, and lifecycle management
- Queue-based task processing with session management
- AI agent management and assignment
- Semantic search across all data types (tasks, memories, ideas, errors, training data)
- Memory and idea management with full feature set
- Training data collection and management
- Error tracking, resolution, and analytics
- Embedding model management and configuration
- Chat message processing with structured content extraction
- Backup and restore functionality
- Statistics and analytics reporting
- GUI/TUI launch support

**Architecture:**
The `TodoziHandler` class processes all commands and manages application state. It integrates with storage systems, supports API key management, queue processing, semantic search, and all other system features. The system uses a command pattern for all operations, providing a consistent interface. Each command type has a dedicated handler method that processes the command and returns results.

**Command Categories:**
- **Task Commands**: Add, List, Show, Update, Complete, Delete tasks
- **Project Commands**: Create, List, Show, Update projects
- **API Commands**: Register, List, Check, Activate, Deactivate, Remove API keys
- **Queue Commands**: Plan, List, Start, End queue sessions
- **Server Commands**: Start, Status, Endpoints server operations
- **Search Commands**: Unified search across all content types
- **Chat Commands**: Process messages and extract structured content
- **Error Commands**: Create, List, Show, Resolve, Delete errors
- **Training Commands**: Create, List, Show, Delete training data
- **Agent Commands**: Create, List, Show, Update, Delete agents
- **Memory Commands**: Create, List, Search memories
- **Idea Commands**: Create, List, Search ideas
- **Embedding Commands**: Configure and manage embedding models

**Usage Example:**
```javascript
const handler = await TodoziHandler.new(storage);

// Create task
await handler.handleAddCommand({
    type: 'Task',
    action: 'Implement authentication',
    time: '4 hours',
    priority: 'HIGH',
    project: 'auth-system',
    status: 'TODO',
    assignee: 'HUMAN',
    tags: 'security,authentication',
    context: 'Need to implement JWT-based authentication'
});

// List tasks with filters
await handler.handleListCommand({
    type: 'Tasks',
    project: 'auth-system',
    status: 'TODO',
    priority: 'HIGH'
});

// Update task
await handler.handleUpdateCommand('task-123', {
    status: 'IN_PROGRESS',
    progress: 50,
    context: 'Started implementation, working on JWT token generation'
});

// Complete task
await handler.completeTask('task-123');

// API key management
await handler.handleApiCommand({
    type: 'Register',
    userId: 'user-123'
});

await handler.handleApiCommand({
    type: 'List',
    activeOnly: true
});

// Queue management
await handler.handleQueueCommand({
    type: 'Plan',
    taskName: 'Database migration',
    taskDescription: 'Migrate user data to new schema',
    priority: 'HIGH',
    projectId: 'data-migration'
});

await handler.handleQueueCommand({
    type: 'Start',
    queueItemId: 'queue-item-456'
});

// Unified search
await handler.handleSearchAllCommand({
    type: 'SearchAll',
    query: 'authentication',
    types: 'tasks,memories,ideas'
});

// Chat processing
const content = await handler.processChatMessageExtended(
    "Meeting notes: <todozi>Review budget; 1 week; high; finance; todo</todozi>",
    'user-123'
);
console.log('Extracted tasks:', content.tasks);

// Error management
await handler.handleErrorCommand({
    type: 'CreateError',
    title: 'Database Connection Failed',
    description: 'Could not connect to database',
    severity: 'high',
    category: 'storage'
});

// Statistics
await handler.handleStatsCommand({
    type: 'Stats'
});

// Launch GUI
await handler.launchGui();
```

**Command Processing Flow:**
1. Command received and validated
2. Routed to appropriate handler method
3. Handler processes command with storage/service integration
4. Results formatted and returned
5. Errors handled with custom error types

**Performance:** O(1) for task creation, O(n) for searches and listings. The system uses efficient data structures and caching for frequently accessed data. Command routing is O(1) using command type mapping.

**Design Patterns:** Command pattern for operations (each command is an object), Factory pattern for entity creation, Builder pattern for complex updates (TaskUpdate), Strategy pattern for different storage/AI models, Singleton pattern for configuration, Facade pattern for unified interface.

**Integration Points:**
- Storage layer for data persistence
- Embedding service for semantic search
- API key management for authentication
- Agent management for task assignment
- Error management for error tracking
- Queue system for workflow processing

---

### Embedding Service

The Todozi Embedding Service provides comprehensive semantic search and embedding management for task management and knowledge organization using sentence-transformers models and advanced similarity algorithms.

**Key Features:**
- Semantic search across all content types (Task, Tag, Memory, Idea, Chunk, Feel, Train, Error, Summary, Reminder, Tdz)
- Content clustering and similarity detection with configurable thresholds
- Embedding caching with TTL management and LRU eviction
- Hierarchical clustering capabilities with depth control
- Drift tracking for content evolution over time
- Performance profiling and diagnostics with timing metrics
- Multi-model embedding support with model comparison
- Batch processing for multiple embeddings
- Hybrid search combining semantic and keyword search
- Multi-query search with aggregation strategies (Average, Weighted, Max)
- Content-type agnostic unified embedding approach

**Architecture:**
The `TodoziEmbeddingService` manages embeddings using an LRU cache with TTL-based expiration. It supports multiple embedding models through the `EmbeddingModel` wrapper and provides similarity search using cosine similarity. The system can cluster content hierarchically and track embedding drift over time. The service integrates with tag management and storage systems for persistence.

**Data Models:**
- `TodoziEmbeddingService`: Main service class managing embeddings and operations
- `TodoziEmbeddingConfig`: Configuration with model_name, dimensions, similarity_threshold, max_results, cache_ttl_seconds, enable_clustering, clustering_threshold
- `TodoziEmbeddingCache`: Cache entry with vector, content_type, content_id, text_content, tags, created_at, ttl_seconds
- `SimilarityResult`: Search result with content_id, content_type, similarity_score, text_content, tags, metadata
- `ClusteringResult`: Cluster result with cluster_id, content_items array, cluster_center vector, cluster_size, average_similarity
- `LRUEmbeddingCache`: LRU cache implementation with memory limits
- `EmbeddingModel`: Model wrapper for sentence-transformers

**Usage Example:**
```javascript
const service = await TodoziEmbeddingService.new(config);

// Initialize with device selection
await service.initialize(device="cpu"); // or "cuda"

// Generate single embedding
const embedding = service.generate_embedding("Complete documentation");

// Batch processing
const texts = ["Task 1", "Task 2", "Task 3"];
const embeddings = service.generate_embeddings_batch(texts);

// Add task with embedding
await service.addTask({
    id: 'task-001',
    action: 'Complete documentation',
    context_notes: 'Write comprehensive documentation',
    priority: 'High',
    status: 'InProgress',
    tags: ['documentation', 'writing']
});

// Find similar tasks
const similar = await service.findSimilarTasks(
    'Write technical documentation',
    10
);
similar.forEach(result => {
    console.log(`${result.content_id}: ${result.similarity_score}`);
});

// Semantic search across content types
const results = await service.semanticSearch(
    'machine learning concepts',
    ['Task', 'Memory', 'Idea'],
    20
);

// Clustering
const clusters = await service.clusterContent();
clusters.forEach(cluster => {
    console.log(`Cluster ${cluster.cluster_id}: ${cluster.cluster_size} items`);
    console.log(`Average similarity: ${cluster.average_similarity}`);
});

// Hierarchical clustering
const hierarchical = await service.hierarchicalClustering(
    ['Task', 'Idea'],
    3 // max depth
);

// Hybrid search (semantic + keyword)
const hybridResults = await service.hybridSearch(
    'project planning',
    ['schedule', 'timeline', 'milestone'],
    ['Task', 'Memory'],
    0.7, // 70% semantic weight
    15
);

// Multi-query search
const multiResults = await service.multiQuerySearch(
    ['software development', 'coding practices', 'programming techniques'],
    'Average', // aggregation strategy
    ['Task', 'Idea'],
    10
);

// Drift tracking
const driftReport = await service.trackEmbeddingDrift(
    'content-001',
    'Updated content text here...'
);
console.log(`Drift: ${driftReport.drift_percentage}%`);

// Performance profiling
const metrics = await service.profileSearchPerformance(
    'machine learning',
    20 // iterations
);
console.log(`Average time: ${metrics.avg_time_ms}ms`);

// Statistics
const stats = await service.getStats();
console.log(`Total embeddings: ${stats.total_embeddings}`);
console.log(`Cache size: ${stats.cache_size}`);
```

**Configuration Options:**
- `model_name`: Pre-trained model identifier (default: "sentence-transformers/all-MiniLM-L6-v2")
- `dimensions`: Expected embedding dimensions (default: 384)
- `similarity_threshold`: Minimum similarity score (default: 0.7)
- `max_results`: Maximum search results (default: 50)
- `cache_ttl_seconds`: Cache time-to-live (default: 86400)
- `enable_clustering`: Enable clustering (default: true)
- `clustering_threshold`: Clustering similarity threshold (default: 0.8)

**Performance:** O(n) for semantic search (linear scan of cache), O(n²) for clustering (pairwise similarity), O(d) for cosine similarity where d is vector dimensions (typically 384). Cache lookups are O(1). Batch processing reduces overhead for multiple embeddings.

**Security:** Embeddings contain semantic information that could potentially be reverse-engineered. Sensitive content should have shorter TTL values. The system implements proper access control and input validation. Cache entries include original text content which should be protected.

**Caching Strategy:**
- LRU cache with memory limits
- TTL-based expiration
- Automatic cleanup of expired entries
- Memory size estimation for cache entries
- Cache hit/miss tracking

**Design Patterns:** Service pattern for centralized operations, Factory pattern for service initialization, Strategy pattern for aggregation methods, Cache pattern for LRU management, Builder pattern for result construction.

---

### Error Management

The Todozi Error Management System provides structured error definitions, centralized error management, and robust error parsing capabilities with comprehensive error lifecycle tracking and resolution management.

**Key Features:**
- Custom error class extending JavaScript Error with structured metadata
- Centralized error registry with full lifecycle management
- Error parsing from formatted text strings with validation
- Multiple error types (Validation, Storage, Config, IO, JSON, UUID, Chrono, Dialoguer, HLX, Reqwest, Dir, Embedding, API, Candle, NotImplemented, Serialization)
- Error resolution tracking with resolution notes and timestamps
- Comprehensive error metadata (severity, category, source, context, tags)
- Error severity parsing and validation (low, medium, high, critical)
- Error category parsing and validation (network, database, authentication, validation, logic, UI, API, system, other)
- UUID generation for error identification

**Architecture:**
The system uses a `TodoziError` class with static factory methods for different error types. The `ErrorManager` class maintains a registry of errors (Map-based) with resolution tracking. Error parsing functions can extract structured error information from formatted text. The system supports error severity and category validation with normalization.

**Data Models:**
- `TodoziError`: Custom error class extending Error with type and details properties
- `ErrorManager`: Error registry manager with create, resolve, and query methods
- Error object structure: id (UUID), title, description, severity, category, source, context, tags, resolved (boolean), resolved_at (Date), created_at (Date)

**Usage Example:**
```javascript
// Create specific errors using factory methods
throw TodoziError.taskNotFound(123);
throw TodoziError.projectNotFound('my-project');
throw TodoziError.feelingNotFound(456);
throw TodoziError.invalidPriority('extreme');
throw TodoziError.invalidStatus('invalid-status');
throw TodoziError.invalidAssignee('invalid-assignee');
throw TodoziError.invalidProgress(150); // Must be 0-100
throw TodoziError.validation('Invalid input format');
throw TodoziError.storage('Storage operation failed');
throw TodoziError.config('Configuration error');
throw TodoziError.io('I/O operation failed');
throw TodoziError.json(new Error('JSON parse error'));
throw TodoziError.uuid(new Error('UUID generation failed'));
throw TodoziError.chrono(new Error('Date parsing failed'));
throw TodoziError.dialoguer(new Error('User input error'));
throw TodoziError.hlx(new Error('HLX format error'));
throw TodoziError.reqwest(new Error('HTTP request failed'));
throw TodoziError.dir('Directory operation failed');
throw TodoziError.embedding('Embedding operation failed');
throw TodoziError.api('API operation failed');
throw TodoziError.candle('Candle library error');
throw TodoziError.notImplemented('Feature not implemented');
throw TodoziError.serialization('Serialization failed');

// Error manager
const errorManager = new ErrorManager();

// Create and register error
const errorId = await errorManager.createError({
    title: 'Database Connection Failed',
    description: 'Could not connect to the main database',
    severity: 'high',
    category: 'storage',
    source: 'DatabaseManager.connect',
    context: 'Connection timeout after 30 seconds',
    tags: ['database', 'connection', 'timeout']
});

// Get unresolved errors
const unresolved = errorManager.getUnresolvedErrors();
console.log(`Unresolved errors: ${unresolved.length}`);

// Resolve error with resolution note
await errorManager.resolveError(
    errorId, 
    'Database connection restored after restarting database service'
);

// Parse error from formatted text
const errorText = "<error>Network Timeout;Connection to server timed out;high;network;api_client;Request ID: 12345;timeout,connection</error>";
const parsedError = parseErrorFormat(errorText);
console.log('Parsed error:', parsedError);

// Parse and validate severity
const severity = parseErrorSeverity('high'); // Returns normalized severity

// Parse and validate category
const category = parseErrorCategory('network'); // Returns normalized category
```

**Error Format:**
Errors can be parsed from XML-like format:
`<error>title; description; severity; category; source; context; tags</error>`

**Error Types:**
- `TaskNotFound`: Task with given ID not found
- `ProjectNotFound`: Project with given name not found
- `FeelingNotFound`: Feeling with given ID not found
- `InvalidPriority`: Invalid priority value
- `InvalidStatus`: Invalid status value
- `InvalidAssignee`: Invalid assignee value
- `InvalidProgress`: Invalid progress value (must be 0-100)
- `ValidationError`: General validation error
- `StorageError`: Storage operation error
- `ConfigError`: Configuration error
- `IoError`: Input/output error
- `JsonError`: JSON parsing error
- `UuidError`: UUID generation/parsing error
- `ChronoError`: Date/time parsing error
- `DialoguerError`: User input error
- `HlxError`: HLX format error
- `ReqwestError`: HTTP request error
- `DirError`: Directory operation error
- `EmbeddingError`: Embedding operation error
- `ApiError`: API operation error
- `CandleError`: Candle library error
- `NotImplemented`: Feature not implemented error
- `SerializationError`: Serialization error

**Performance:** O(1) for error creation and registration using Map storage, O(n) for retrieving unresolved errors (filtering), O(1) for error resolution (Map lookup and update). UUID generation is O(1).

**Design Patterns:** Factory pattern for error creation (static factory methods), Registry pattern for error management (ErrorManager), Custom Error Extension pattern (extends Error), Parser pattern for text parsing (parseErrorFormat).

**Error Lifecycle:**
1. Error created and registered with UUID
2. Error tracked in registry with unresolved status
3. Error can be queried and filtered
4. Error resolved with resolution note and timestamp
5. Resolved errors remain in registry for audit trail

**Security:** Error messages should not expose sensitive information. The system provides structured error information that can be filtered before exposing to end users. Error details should be logged securely for debugging.

---

### Content Extraction

The Todozi Content Extraction System extracts structured data from unstructured text content using AI-powered APIs with support for multiple content types and output formats.

**Key Features:**
- Multi-format input (inline text or file paths)
- AI-powered extraction using Todozi API endpoints ("plan" and "strategic")
- Structured output formats (JSON, CSV, Markdown)
- Auto-embedding of extracted content to project files
- Human-readable checklist generation for review
- History tracking with comprehensive logs to mega log file
- Support for extracting tasks, memories, ideas, errors, and training data
- Configuration loading from `.todozi/tdz.hlx` file
- API key management through `getTdzApiKey()` function

**Architecture:**
The system uses `extractContent` and `strategyContent` functions that call Todozi API endpoints. Extracted content is parsed into structured objects (`ExtractResponse` containing arrays of tasks, memories, ideas, errors, training_data) and formatted according to output preferences. The system supports both "plan" endpoint for general extraction and "strategic" endpoint for strategic planning content.

**Data Models:**
- `ExtractResponse`: Container for all extracted content types
- `ExtractedTask`: Task entity with action, time, priority, project, status, assignee, tags
- `ExtractedMemory`: Memory entity with moment, meaning, reason, importance, term
- `ExtractedIdea`: Idea entity with idea text, share level, importance
- `ExtractedError`: Error entity with title, description, severity, category
- `ExtractedTrainingData`: Training data entity with prompt, completion, data_type

**Usage Example:**
```javascript
// Extract from inline content
const result = await extractContent(
    "Meeting notes: Review Q4 budget (High priority, due next week)",
    null,
    'json',
    true // Generate human-readable checklist
);

// Extract from file
const fileResult = await extractContent(
    null,
    '/path/to/meeting-notes.txt',
    'md',
    false
);

// Strategic content analysis
const strategic = await strategyContent(
    "Company Q4 Strategy: Expand into European markets (High importance, 18-month timeline)",
    null,
    'json',
    true
);

// CSV output
const csvResult = await extractContent(
    "Project kickoff: Design UI mockups (High priority, due Friday) - John",
    null,
    'csv',
    false
);

// The result contains structured data
// JSON format:
// {
//     tasks: [...],
//     memories: [...],
//     ideas: [...],
//     errors: [...],
//     training_data: [...],
//     raw_tags: [...]
// }
```

**Output Formats:**
- **JSON**: Structured JSON with all extracted entities as arrays
- **CSV**: Comma-separated values format for spreadsheet import
- **Markdown**: Human-readable markdown format
- **Human Checklist**: Formatted checklist for review (when `human=true`)

**Configuration:**
The system requires a configuration file at `~/.todozi/tdz.hlx`:
```json
{
  "registration": {
    "user_id": "user-123",
    "fingerprint": "device-fingerprint-456"
  }
}
```

**API Integration:**
- Uses `getTdzApiKey()` for API key retrieval
- Makes HTTP POST requests to Todozi API endpoints
- Handles authentication via Bearer token
- Processes API responses and extracts structured content

**Performance:** O(1) network latency + O(n) content processing where n is content size. File operations are O(1) for single files. Formatting is O(n) where n is number of extracted items. The system includes retry logic for transient failures.

**Security:** API key management through secure key retrieval, content privacy through Todozi API, secure file system permissions, HTTPS communication for all API requests. Input validation and sanitization are implemented. File paths are validated to prevent directory traversal.

**History Logging:** Extracted tasks are automatically logged to `~/.todozi/history/mega_log.jsonl` for audit and analysis. The system maintains comprehensive logs of all extracted content.

**Error Handling:** The system handles API request failures, configuration loading errors, file system errors, and parsing errors with descriptive error messages. All errors are wrapped in `TodoziError` for consistent error handling.

**Design Patterns:** Facade pattern for simplified API, Factory pattern for response creation, Strategy pattern for different endpoints, Singleton pattern for configuration access.

---

### Idea Management

The Todozi Idea Management System provides comprehensive functionality for organizing, categorizing, and managing ideas within a collaborative environment with sharing controls, importance ratings, and advanced analytics.

**Key Features:**
- Idea CRUD operations with UUID-based identification
- Tag-based organization and filtering with case-insensitive matching
- Sharing control (Public, Team, Private) for collaboration
- Importance ranking (Breakthrough=5, High=4, Medium=3, Low=2, VeryLow=1)
- Full-text search across ideas, tags, and context fields
- Analytics and statistics with percentage calculations
- Text-based idea format parsing from XML-like markup
- Recent ideas retrieval with limit support
- Tag statistics and usage tracking
- Filtering by importance, share level, and tags

**Architecture:**
The `IdeaManager` class maintains a Map of ideas with UUID keys and a separate Map for tag indexing (`idea_tags`). Ideas have sharing levels, importance ratings, tags, and context. The system provides extensive filtering and search capabilities with case-insensitive matching. Statistics are calculated on-demand and include percentage distributions.

**Data Models:**
- `Idea`: Core idea entity with id (UUID), idea (text), share (ShareLevel), importance (IdeaImportance), tags (array), context (string), created_at, updated_at
- `IdeaUpdate`: Builder pattern class for constructing idea updates
- `IdeaStatistics`: Aggregated statistics with percentage methods
- `ShareLevel`: Enum for sharing levels (Public, Team, Private)
- `IdeaImportance`: Enum for importance levels (VeryLow=1, Low=2, Medium=3, High=4, Breakthrough=5)

**Usage Example:**
```javascript
const ideaManager = new IdeaManager();

// Create idea
const ideaId = await ideaManager.createIdea({
    idea: "Implement microservices architecture",
    share: ShareLevel.TEAM,
    importance: IdeaImportance.HIGH,
    tags: ["architecture", "scalability"],
    context: "System performance review"
});

// Update using builder pattern
const update = IdeaUpdate.new()
    .withIdea("Advanced microservices architecture with service mesh")
    .withImportance(IdeaImportance.BREAKTHROUGH)
    .withTags(["architecture", "scalability", "microservices", "service-mesh"])
    .withContext("Critical system redesign for scalability");

await ideaManager.updateIdea(ideaId, update);

// Search ideas
const results = ideaManager.searchIdeas("microservices");

// Filter by importance
const breakthrough = ideaManager.getBreakthroughIdeas();
const highImportance = ideaManager.getIdeasByImportance(IdeaImportance.HIGH);

// Filter by share level
const publicIdeas = ideaManager.getPublicIdeas();
const teamIdeas = ideaManager.getTeamIdeas();
const privateIdeas = ideaManager.getPrivateIdeas();

// Filter by tag
const architectureIdeas = ideaManager.getIdeasByTag("architecture");

// Get recent ideas
const recent = ideaManager.getRecentIdeas(10);

// Get all tags
const allTags = ideaManager.getAllTags();

// Get tag statistics
const tagStats = ideaManager.getTagStatistics();
for (const [tag, count] of tagStats) {
    console.log(`${tag}: ${count} ideas`);
}

// Statistics
const stats = ideaManager.getIdeaStatistics();
console.log(`Total ideas: ${stats.totalIdeas}`);
console.log(`Public: ${stats.publicPercentage()}%`);
console.log(`Team: ${stats.teamPercentage()}%`);
console.log(`Private: ${stats.privatePercentage()}%`);
console.log(`Breakthrough: ${stats.breakthroughPercentage()}%`);
```

**Idea Format Parsing:**
Ideas can be parsed from text format:
`<idea>idea text; share level; importance; tags; context</idea>`

**Performance:** O(1) for CRUD operations using hash maps, O(n×m) for search where n is ideas and m is average text length, O(n) for filtering operations, O(n log n) for recent ideas (sorting required), O(n×t) for tag statistics where t is average tags per idea.

**Design Patterns:** Builder pattern for IdeaUpdate (fluent interface), Repository pattern for IdeaManager (data access abstraction), Factory pattern for parsing (parseIdeaFormat), Strategy pattern for filtering (different filter methods).

**Storage:** Ideas are stored in memory. Persistence should be handled by the storage layer integration. The system maintains tag indexes for efficient tag-based queries.

---

### Models & Data Structures

The Models module provides comprehensive data structures and enums for the entire Todozi system with full type safety, validation, and consistent patterns across all entities.

**Key Features:**
- Enum-like classes for type safety (Priority, Status, Assignee, MemoryImportance, MemoryTerm, MemoryType, ShareLevel, IdeaImportance, ItemStatus, ErrorSeverity, ErrorCategory, TrainingDataType, ProjectStatus, AgentStatus, AssignmentStatus, QueueStatus, SummaryPriority, ReminderPriority, ReminderStatus)
- Core data models with comprehensive metadata (Task, Project, Agent, Memory, Idea, ApiKey, Error, TrainingData, Tag, Reminder, Summary)
- Builder patterns for updates with fluent interfaces
- Collection classes for managing groups of entities
- Comprehensive metadata support with timestamps and tracking
- Validation logic for all entity types
- Serialization support for persistence

**Architecture:**
The module defines enum classes for all system constants with string values and parsing methods, core entity classes with full metadata and validation, builder classes for updates with method chaining, and collection classes for managing groups of entities. All classes follow consistent patterns for validation, serialization, and lifecycle management.

**Key Classes:**
- **Task**: Core task entity with id, userId, action, time, priority, parentProject, status, assignee, tags, dependencies, contextNotes, progress, embeddingVector, timestamps
- **TaskUpdate**: Builder for task updates with fluent interface
- **TaskFilters**: Filtering criteria for task queries
- **TaskCollection**: Container for managing task collections
- **Project**: Project entity with name, description, status, tasks list, and statistics
- **ProjectTaskContainer**: Specialized container for project-based task management
- **Agent**: AI agent with id, name, description, capabilities, specializations, behaviors, constraints, metadata, timestamps
- **Memory**: Memory entity with moment, meaning, reason, importance, term, type, tags, timestamps
- **Idea**: Idea entity with idea text, share level, importance, tags, context, timestamps
- **ApiKey**: API key with user_id, public_key, private_key, active status
- **ApiKeyCollection**: Collection manager for API keys
- **Error**: Error entity with title, description, severity, category, source, context, tags, resolved flag, timestamps
- **TrainingData**: Training data with prompt, completion, data_type
- **Tag**: Tag entity with name, description, color, category, usage_count
- **Reminder**: Reminder with content, remind_at, priority, status, tags
- **Summary**: Summary with content, priority, context, tags

**Usage Example:**
```javascript
// Create task using factory method
const task = Task.new(
    userId,
    "Complete documentation",
    "4 hours",
    Priority.HIGH,
    "project-123",
    Status.TODO
);

// Create full task with all fields
const fullTask = Task.newFull(
    userId,
    "Complete documentation",
    "4 hours",
    Priority.HIGH,
    "project-123",
    Status.TODO,
    Assignee.HUMAN,
    ["documentation", "writing"],
    [],
    "Write comprehensive API documentation",
    0
);

// Update using builder
const update = TaskUpdate.new()
    .withAction("Complete API documentation")
    .withStatus(Status.IN_PROGRESS)
    .withProgress(50)
    .withContext("Started writing introduction section")
    .withTags(["documentation", "writing", "api"]);

task.update(update);

// Check task status
if (task.isCompleted()) {
    console.log("Task is completed");
}

if (task.isActive()) {
    console.log("Task is active");
}

// Complete task
task.complete();

// Enum parsing
const priority = Priority.fromStr("high"); // Returns Priority.HIGH
const status = Status.fromStr("in_progress"); // Returns Status.IN_PROGRESS
const assignee = Assignee.AI; // or Assignee.HUMAN, Assignee.COLLABORATIVE, Assignee.AGENT("agent-id")
```

**Enum Classes:**
- `Priority`: Low, Medium, High, Critical, Urgent
- `Status`: Todo, Pending, InProgress, Blocked, Review, Done, Completed, Cancelled, Deferred
- `Assignee`: Ai, Human, Collaborative, Agent(id)
- `MemoryImportance`: Low, Medium, High, Critical
- `MemoryTerm`: Short, Long
- `MemoryType`: Standard, Secret, Human, Emotional(emotion)
- `ShareLevel`: Public, Private, Team
- `IdeaImportance`: VeryLow, Low, Medium, High, Breakthrough
- `ItemStatus`: Active, Archived, Deleted
- `ErrorSeverity`: Low, Medium, High, Critical
- `ErrorCategory`: Network, Database, Authentication, Validation, Logic, UI, API, System, Other
- `TrainingDataType`: Instruction, Question, Conversation, Code, Documentation
- `ProjectStatus`: Active, Archived, Completed
- `AgentStatus`: Available, Busy, Inactive
- `AssignmentStatus`: Assigned, InProgress, Completed, Failed
- `QueueStatus`: Pending, Active, Completed, Cancelled
- `SummaryPriority`: Low, Medium, High, Critical
- `ReminderPriority`: Low, Medium, High, Critical
- `ReminderStatus`: Pending, Active, Completed, Cancelled

**Validation:**
- Progress must be between 0-100 (inclusive)
- Status and priority validated through enum parsing
- Automatic timestamp updates on modifications
- Required field validation on creation
- Type validation for all fields

**Performance:** O(1) for entity creation and updates, O(n) for collection operations. Enum parsing is O(1) with string mapping.

**Design Patterns:** Enum pattern for constants (string-based enums), Builder pattern for updates (fluent interfaces), Factory pattern for creation (static `new()` methods), Repository pattern for collections (data access abstraction), Validation pattern for input validation.

**Serialization:** All models support JSON serialization for persistence. The system handles serialization/deserialization automatically through the storage layer.

---

### Todozi Core

The Todozi Core module provides structured parsing, execution, and management of various content types using a custom XML-like markup language with comprehensive support for all system entities and execution workflows.

**Key Features:**
- Custom XML-like markup parsing with validation
- Content type parsing (tasks, memories, ideas, errors, training data, feelings)
- Task execution for different assignee types (AI, Human, Collaborative, Agent)
- Workflow processing with dependency resolution
- Enum definitions for all system constants with parsing support
- Shorthand tag transformation for simplified syntax
- Extended chat message processing with multiple content types
- JSON example processing for structured data extraction
- Helper functions for enum parsing and validation
- Tag transformation utilities

**Architecture:**
The module provides parsing functions for each content type that extract structured data from formatted text. Execution functions handle different task types with appropriate workflows. Helper functions transform shorthand tags, validate enums, and provide utility operations. The system supports both single content extraction and batch processing from chat messages.

**Content Format Parsing:**
- **Tasks**: `<todozi>action; time; priority; project; status; assignee</todozi>`
- **Memories**: `<memory>moment; meaning; reason; importance; term; tags; context</memory>`
- **Ideas**: `<idea>idea text; share level; importance; tags; context</idea>`
- **Errors**: `<error>title; description; severity; category; source; context; tags</error>`
- **Training Data**: `<train>prompt; completion; data_type</train>`
- **Feelings**: `<feel>feeling; intensity; context; tags</feel>`

**Usage Example:**
```javascript
// Parse formatted task
const taskText = "<todozi>Complete docs; 4 hours; high; project-123; todo; human</todozi>";
const task = parseTodoziFormat(taskText);
console.log(task.action); // "Complete docs"
console.log(task.priority); // Priority.HIGH

// Parse memory
const memoryText = "<memory>First day; New beginning; Career start; high; long; career,new; Starting new job</memory>";
const memory = parseMemoryFormat(memoryText);

// Parse idea
const ideaText = "<idea>Microservices architecture; team; high; architecture,scalability; System redesign</idea>";
const idea = parseIdeaFormat(ideaText);

// Parse error
const errorText = "<error>Connection Failed; Database timeout; high; network; db_client; Request ID: 123; timeout,connection</error>";
const error = parseErrorFormat(errorText);

// Parse training data
const trainText = "<train>What is a task?; A task is a unit of work; instruction</train>";
const training = parseTrainingDataFormat(trainText);

// Parse feeling
const feelText = "<feel>excited; 8; Starting new project; work,new</feel>";
const feeling = parseFeelingFormat(feelText);

// Process chat message (extracts all content types)
const message = `
Meeting notes:
<todozi>Review budget; 1 week; high; finance; todo</todozi>
<memory>Budget meeting; Important discussion; Q4 planning; high; short; meeting,budget</memory>
<idea>Automated budget tracking; team; medium; automation,finance; Budget management</idea>
`;
const extracted = processChatMessage(message);
console.log('Tasks:', extracted.tasks);
console.log('Memories:', extracted.memories);
console.log('Ideas:', extracted.ideas);

// Extended processing with additional metadata
const extended = processChatMessageExtended(message, 'user-123');
console.log('Extended extraction:', extended);

// Process JSON examples
const jsonExamples = [
    { action: "Task 1", priority: "high" },
    { action: "Task 2", priority: "medium" }
];
const processed = processJsonExamples(jsonExamples);

// Execute tasks
await executeTask(task); // Generic execution
await executeAiTask(task); // AI-specific execution
await executeHumanTask(task); // Human-specific execution
await executeCollaborativeTask(task); // Collaborative execution
await executeAgentTask(task, 'agent-123'); // Agent-specific execution

// Process workflow
const workflow = [
    { action: "Setup", dependencies: [] },
    { action: "Implement", dependencies: ["Setup"] },
    { action: "Test", dependencies: ["Implement"] }
];
await processWorkflow(workflow);

// Transform shorthand tags
const shorthand = "Complete docs [high] [project-123]";
const transformed = transformShorthandTags(shorthand);
// Converts to: "<todozi>Complete docs; ASAP; high; project-123; todo; human</todozi>"

// Parse enums
const priority = parseEnum("high", Priority); // Returns Priority.HIGH
const status = parseEnum("in_progress", Status); // Returns Status.IN_PROGRESS
```

**Helper Functions:**
- `transformShorthandTags(text)`: Converts shorthand syntax to full format
- `parseEnum(value, enumClass)`: Parses enum values with validation
- `processChatMessage(message)`: Extracts all content types from message
- `processChatMessageExtended(message, userId)`: Extended processing with user context
- `processJsonExamples(examples)`: Processes structured JSON examples
- `processWorkflow(workflow)`: Executes workflow with dependency resolution

**Execution Functions:**
- `executeTask(task)`: Generic task execution
- `executeAiTask(task)`: AI agent execution
- `executeHumanTask(task)`: Human execution workflow
- `executeCollaborativeTask(task)`: Collaborative execution
- `executeAgentTask(task, agentId)`: Specific agent execution

**Parsing Functions:**
- `parseTodoziFormat(text)`: Parse task format
- `parseMemoryFormat(text)`: Parse memory format
- `parseIdeaFormat(text)`: Parse idea format
- `parseErrorFormat(text)`: Parse error format
- `parseTrainingDataFormat(text)`: Parse training data format
- `parseFeelingFormat(text)`: Parse feeling format

**Performance:** O(n) for parsing where n is text length, O(1) for enum parsing with string mapping, O(n×d) for workflow processing where d is dependencies.

**Design Patterns:** Parser pattern for content extraction (dedicated parsing functions), Strategy pattern for execution (different execution methods), Factory pattern for object creation (parsing returns objects), Template Method pattern for workflow processing.

**Error Handling:** All parsing functions validate input and throw `TodoziError` for invalid formats. Enum parsing includes validation and normalization. Execution functions handle errors gracefully with proper error reporting.

---

### Server

The Todozi Enhanced Server provides a comprehensive RESTful API for managing tasks, agents, training data, memories, and more with integrated AI capabilities, supporting multiple protocols and advanced features.

**Key Features:**
- RESTful API endpoints for all system resources with full CRUD operations
- Multiple protocol support (HTTP, HTTPS, gRPC, WebSocket)
- AI agent system with 26 pre-configured agents for automated task handling
- Training data management for AI model improvement
- Semantic search and similarity detection across all content types
- Time tracking and analytics with detailed metrics
- Memory and idea management with full feature set
- Queue-based workflow processing with session management
- API key authentication with role-based access control
- Real-time updates via WebSocket connections
- Comprehensive error handling and logging
- Request validation and sanitization
- Rate limiting and throttling
- CORS support for cross-origin requests
- Health check and status endpoints
- Metrics and monitoring endpoints

**Architecture:**
The server uses a controller pattern with separate controllers for each resource type. Business logic is encapsulated in service classes. Data access is abstracted through storage interfaces. The system supports multiple protocols (HTTP/HTTPS for REST, gRPC for high-performance RPC, WebSocket for real-time) and includes comprehensive error handling, middleware for authentication and validation, and request/response transformation.

**Key Endpoints:**
- **Tasks**: `/api/tasks` - GET (list), POST (create), GET `/:id` (get), PUT `/:id` (update), DELETE `/:id` (delete)
- **Projects**: `/api/projects` - GET (list), POST (create), GET `/:name` (get), PUT `/:name` (update)
- **Agents**: `/api/agents` - GET (list), POST (create), GET `/:id` (get), PUT `/:id` (update), DELETE `/:id` (delete)
- **Training Data**: `/api/training` - GET (list), POST (create), GET `/:id` (get), DELETE `/:id` (delete)
- **Memory**: `/api/memory` - GET (list), POST (create), GET `/:id` (get), PUT `/:id` (update), DELETE `/:id` (delete)
- **Ideas**: `/api/ideas` - GET (list), POST (create), GET `/:id` (get), PUT `/:id` (update), DELETE `/:id` (delete)
- **Search**: `/api/search` - POST (semantic search across all types)
- **Queue**: `/api/queue` - GET (list), POST (plan), POST `/:id/start` (start), POST `/:id/end` (end)
- **Health**: `/api/health` - GET (health check)
- **Status**: `/api/status` - GET (server status and metrics)
- **Endpoints**: `/api/endpoints` - GET (list all available endpoints)

**Usage Example:**
```javascript
// Start server
const server = new TodoziServer({
    port: 8636,
    host: '0.0.0.0',
    enableHttps: false,
    enableGrpc: true,
    enableWebSocket: true,
    corsEnabled: true,
    rateLimitEnabled: true
});
await server.start();

// Create task via API
const response = await fetch('http://localhost:8636/api/tasks', {
    method: 'POST',
    headers: {
        'Authorization': `Bearer ${apiKey}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        action: 'Complete documentation',
        priority: 'high',
        project: 'project-123',
        status: 'todo',
        assignee: 'human',
        time: '4 hours'
    })
});
const task = await response.json();

// List tasks with filters
const listResponse = await fetch('http://localhost:8636/api/tasks?project=project-123&status=todo', {
    headers: {
        'Authorization': `Bearer ${apiKey}`
    }
});
const tasks = await listResponse.json();

// Update task
await fetch(`http://localhost:8636/api/tasks/${task.id}`, {
    method: 'PUT',
    headers: {
        'Authorization': `Bearer ${apiKey}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        status: 'in_progress',
        progress: 50
    })
});

// Semantic search
const searchResponse = await fetch('http://localhost:8636/api/search', {
    method: 'POST',
    headers: {
        'Authorization': `Bearer ${apiKey}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        query: 'documentation tasks',
        types: ['Task', 'Memory', 'Idea'],
        limit: 20
    })
});
const searchResults = await searchResponse.json();

// WebSocket connection for real-time updates
const ws = new WebSocket('ws://localhost:8636/ws');
ws.onmessage = (event) => {
    const update = JSON.parse(event.data);
    console.log('Real-time update:', update);
};

// Health check
const healthResponse = await fetch('http://localhost:8636/api/health');
const health = await healthResponse.json();
console.log('Server health:', health);

// Get server status
const statusResponse = await fetch('http://localhost:8636/api/status');
const status = await statusResponse.json();
console.log('Server status:', status);
```

**Authentication:**
- API key authentication via Bearer token in Authorization header
- Key validation against API key registry
- Role-based access control (admin vs regular users)
- Token expiration and refresh support
- Rate limiting per API key

**Security Features:**
- API key authentication with secure key validation
- Input validation and sanitization for all endpoints
- Rate limiting and throttling to prevent abuse
- CORS support with configurable origins
- Secure headers (X-Content-Type-Options, X-Frame-Options, etc.)
- HTTPS support with TLS/SSL configuration
- Request size limits to prevent DoS
- SQL injection prevention (if using SQL storage)
- XSS protection through input sanitization

**Performance Optimizations:**
- Efficient request routing with minimal overhead
- Connection pooling for database connections
- Caching strategies for frequently accessed data
- Async processing for long-running operations
- Response compression (gzip/brotli)
- Pagination for large result sets
- Query optimization for database operations
- Load balancing support

**Error Handling:**
- Comprehensive error types with appropriate HTTP status codes
- Detailed error messages for debugging (in development)
- Sanitized error messages for production
- Error logging and monitoring
- Graceful error recovery
- Request validation errors with field-level details

**Monitoring and Metrics:**
- Request/response metrics
- Error rate tracking
- Response time monitoring
- Active connection tracking
- Resource usage metrics
- Custom metrics endpoints

**Design Patterns:** Controller pattern for request handling, Service pattern for business logic, Repository pattern for data access, Middleware pattern for cross-cutting concerns, Factory pattern for service creation, Strategy pattern for different protocols.

**Deployment:** The server can be deployed as a standalone service or integrated into existing applications. Supports containerization (Docker), process managers (PM2), and cloud platforms. Configuration can be provided via environment variables or configuration files.

---

## Additional Modules

### Memory Management

The Memory Management System provides comprehensive functionality for storing, retrieving, and organizing memories with rich metadata including importance ratings, term durations, and emotional context tracking.

**Key Features:**
- Memory CRUD operations with UUID-based identification
- Importance-based categorization (Low, Medium, High, Critical)
- Term duration management (Short-term, Long-term)
- Memory type support (Standard, Secret, Human, Emotional)
- Tag-based organization and filtering
- Full-text search across moment, meaning, reason, and context
- Recent memories retrieval with limit support
- Critical memories filtering for high-priority items
- Statistics and analytics for memory distribution
- Text-based memory format parsing

**Architecture:**
The `MemoryManager` class maintains a Map of memories with UUID keys and a separate Map for tag indexing. Memories include moment descriptions, meaning, reason, importance level, term duration, memory type, and tags. The system provides extensive querying capabilities including search, filtering by importance/term/type/tag, and statistical analysis.

**Data Models:**
- `Memory`: Core memory entity with moment, meaning, reason, importance, term, memoryType, tags, timestamps
- `MemoryUpdate`: Builder pattern class for constructing memory updates
- `MemoryStatistics`: Aggregated statistics with percentage calculations
- `MemoryImportance`: Enum for importance levels (Low, Medium, High, Critical)
- `MemoryTerm`: Enum for term durations (Short, Long)
- `MemoryType`: Enum for memory types (Standard, Secret, Human, Emotional)

**Usage Example:**
```javascript
const memoryManager = new MemoryManager();

// Create memory
const memoryId = await memoryManager.createMemory({
    moment: "First day at new job",
    meaning: "Beginning of career journey",
    reason: "Started new position",
    importance: MemoryImportance.High,
    term: MemoryTerm.Long,
    memoryType: { type: "Standard" },
    tags: ["career", "new beginnings"]
});

// Search memories
const results = memoryManager.searchMemories("career");

// Filter by importance
const critical = memoryManager.getCriticalMemories();
const highImportance = memoryManager.getMemoriesByImportance(MemoryImportance.High);

// Filter by term
const longTerm = memoryManager.getMemoriesByTerm(MemoryTerm.Long);
const shortTerm = memoryManager.getMemoriesByTerm(MemoryTerm.Short);

// Filter by tag
const careerMemories = memoryManager.getMemoriesByTag("career");

// Recent memories
const recent = memoryManager.getRecentMemories(10);

// Statistics
const stats = memoryManager.getMemoryStatistics();
console.log(`Total memories: ${stats.totalMemories}`);
console.log(`Short-term: ${stats.shortTermPercentage()}%`);
console.log(`Critical: ${stats.criticalPercentage()}%`);
```

**Memory Format Parsing:**
Memories can be parsed from text format:
`<memory>moment; meaning; reason; importance; term; tags; context</memory>`

**Performance:** O(1) for CRUD operations using hash maps, O(n×m) for search where n is memories and m is average text length, O(n) for filtering operations.

**Design Patterns:** Builder pattern for MemoryUpdate, Repository pattern for MemoryManager, Factory pattern for parsing, Strategy pattern for filtering.

**Storage:** Memories are stored in memory. Persistence should be handled by the storage layer integration.

---

### Reminder System

The Reminder Manager System provides comprehensive functionality for managing reminders with features including creation, updating, deletion, searching, and categorization with priority levels and status tracking.

**Key Features:**
- Reminder CRUD operations with UUID-based identification
- Priority-based categorization (Low, Medium, High, Critical)
- Status management (Pending, Active, Completed, Cancelled)
- Time-based filtering (overdue, due soon)
- Tag-based organization and filtering
- Full-text search across reminder content and tags
- Statistics and analytics for reminder distribution
- Text-based reminder format parsing
- Automatic overdue detection

**Architecture:**
The `ReminderManager` class maintains a Map of reminders with UUID keys and a separate Map for tag indexing. Reminders include content, remind_at timestamp, priority, status, and tags. The system provides time-based queries for overdue reminders and reminders due soon within a specified duration.

**Data Models:**
- `Reminder`: Core reminder entity with content, remind_at, priority, status, tags, timestamps
- `ReminderUpdate`: Builder pattern class for constructing reminder updates
- `ReminderStatistics`: Aggregated statistics with percentage calculations
- `ReminderPriority`: Enum for priority levels (Low, Medium, High, Critical)
- `ReminderStatus`: Enum for status values (Pending, Active, Completed, Cancelled)

**Usage Example:**
```javascript
const reminderManager = new ReminderManager();

// Create reminder
const reminderId = await reminderManager.createReminder({
    content: "Meeting with team",
    remind_at: new Date("2024-12-31T10:00:00Z"),
    priority: ReminderPriority.High,
    status: ReminderStatus.Pending,
    tags: ["meeting", "work"]
});

// Get overdue reminders
const overdue = reminderManager.getOverdueReminders();

// Get reminders due soon (within 24 hours)
const dueSoon = reminderManager.getRemindersDueSoon(24 * 60 * 60 * 1000);

// Filter by priority
const highPriority = reminderManager.getRemindersByPriority(ReminderPriority.High);

// Filter by status
const pending = reminderManager.getPendingReminders();
const active = reminderManager.getActiveReminders();

// Search reminders
const results = reminderManager.searchReminders("meeting");

// Statistics
const stats = reminderManager.getReminderStatistics();
console.log(`Total reminders: ${stats.total_reminders}`);
console.log(`Pending: ${stats.pendingPercentage()}%`);
console.log(`Overdue: ${stats.overduePercentage()}%`);
```

**Reminder Format Parsing:**
Reminders can be parsed from text format:
`<reminder>content; remind_at; priority; status; tags</reminder>`

**Performance:** O(1) for CRUD operations, O(n) for time-based filtering, O(n×m) for search operations.

**Design Patterns:** Builder pattern for ReminderUpdate, Repository pattern for ReminderManager, Factory pattern for parsing.

---

### Search System

The SearchEngine provides advanced search capabilities including semantic search, keyword search, and hybrid search across all content types with relevance scoring and analytics.

**Key Features:**
- Multi-type content indexing (tasks, memories, ideas, errors, training data)
- Text-based search with relevance scoring
- Tag-based search capabilities
- Advanced filtering with custom criteria
- Search analytics and suggestions
- Configurable search options (date ranges, limits, data type filtering)
- Relevance score calculation based on text matching and tag relevance
- Search result aggregation across content types

**Architecture:**
The `SearchEngine` class maintains separate collections for each content type. The system uses a relevance scoring algorithm that considers full text matches, word matches, tag matches, and text length penalties. Search results are sorted by relevance score and can be filtered by date ranges and content types.

**Data Models:**
- `SearchEngine`: Main search engine class with content collections
- `SearchResults`: Container for search results across content types
- `SearchOptions`: Configuration options for search operations
- `SearchDataType`: Enum for content types (Tasks, Memories, Ideas, Errors, Training)

**Usage Example:**
```javascript
const engine = new SearchEngine();

// Update index with content
engine.updateIndex({
    tasks: [{ action: "Complete project", tags: ["work", "urgent"] }],
    memories: [{ moment: "Meeting", meaning: "Important discussion", tags: ["meeting"] }],
    ideas: [{ idea: "New feature", tags: ["innovation"] }]
});

// Basic search
const results = engine.search("project", {
    dataTypes: [SearchDataType.Tasks],
    limit: 10
});

// Advanced search with date filtering
const advancedResults = engine.advancedSearch({
    query: "important",
    dataTypes: [SearchDataType.Tasks, SearchDataType.Memories],
    since: new Date("2024-01-01"),
    until: new Date("2024-12-31"),
    limit: 20
});

// Get search analytics
const analytics = engine.getSearchAnalytics();
console.log(`Total indexed items: ${analytics.totalIndexedItems}`);
console.log(`Tasks: ${analytics.tasksCount}`);

// Get search suggestions
const suggestions = engine.getSearchSuggestions("proj", 5);
```

**Relevance Scoring Algorithm:**
1. Full text match: +1.0
2. Word matches: +0.7 per word
3. Tag matches: +0.5 per tag
4. Length penalty: Score multiplied by 1.0 / max(text.length/100, 1.0)

**Performance:** O(n×m) for search where n is indexed items and m is average text length. Relevance calculation is O(m) per item.

**Design Patterns:** Strategy pattern for different search types, Factory pattern for result creation.

---

### Storage System

The Storage System provides an abstraction layer for data persistence supporting file-based JSON storage with efficient querying and atomic operations.

**Key Features:**
- Project-based task organization
- Structured directory system for all data types
- Atomic file operations for data consistency
- Configuration management
- Registration and authentication support
- Task lifecycle management (active, completed, archived)
- Backup and restore functionality
- Cross-platform file system support

**Architecture:**
The `Storage` class manages data persistence using a structured directory system under `~/.todozi/`. Each data type has its own directory with JSON files for storage. The system supports atomic write operations to prevent data corruption and provides methods for creating, reading, updating, and deleting data.

**Directory Structure:**
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

**Usage Example:**
```javascript
const storage = new Storage();

// Create project
const project = storage.createProject("my-project", "Project description");

// Add task to project
const task = storage.addTaskToProject("my-project", {
    action: "Complete task",
    priority: "high",
    status: "todo"
});

// Get task from project
const retrievedTask = storage.getTaskFromProject("my-project", task.id);

// List tasks across projects
const allTasks = storage.listTasksAcrossProjects({
    status: "todo",
    priority: "high"
});

// Update task
storage.updateTaskInProject("my-project", task.id, {
    status: "in_progress",
    progress: 50
});

// Delete task
storage.deleteTaskFromProject("my-project", task.id);
```

**Data Models:**
- `Storage`: Main storage class with project and task management
- `Config`: Configuration management with registration info
- `Project`: Project entity with name and description
- `Task`: Task entity with full metadata

**Performance:** O(1) for direct lookups, O(n) for listing operations. File I/O operations are atomic to prevent corruption.

**Design Patterns:** Repository pattern for data access, Factory pattern for entity creation, Singleton pattern for configuration.

---

### Tag Management

The Tag Management System provides comprehensive functionality for managing tags, categories, and relationships in a tagging system with advanced search and analytics.

**Key Features:**
- Tag creation, update, and deletion
- Category management for tag organization
- Tag relationships (parent/child, related tags)
- Advanced search capabilities with fuzzy matching
- Tag suggestions based on usage patterns
- Usage tracking and statistics
- Bulk operations for tag management
- Color coding for visual organization

**Architecture:**
The `TagManager` class maintains a Map of tags with UUID keys. Tags can have categories, colors, descriptions, and relationships. The `TagSearchEngine` provides advanced search capabilities including fuzzy search and suggestions. The system tracks tag usage counts and provides statistical analysis.

**Data Models:**
- `Tag`: Core tag entity with id, name, description, color, category, usage_count, timestamps
- `TagUpdate`: Builder pattern class for constructing tag updates
- `TagStatistics`: Aggregated statistics with relationship analysis
- `TagSearchQuery`: Advanced search query builder
- `TagSortBy`: Enum for sorting options (Name, Usage, Created, Updated)

**Usage Example:**
```javascript
const tagManager = new TagManager();

// Create tag
const tagId = tagManager.createTag({
    name: "documentation",
    description: "Tasks related to documentation",
    color: "#3498db",
    category: "work"
});

// Update tag
tagManager.updateTag(tagId, TagUpdate.new()
    .withDescription("All documentation-related tasks")
    .withColor("#2980b9")
);

// Create tag relationship
tagManager.addTagRelationship(tagId, "related-tag-id", "related");

// Advanced search
const searchEngine = new TagSearchEngine(tagManager);
const results = searchEngine.advancedSearch({
    name_contains: "doc",
    category: "work",
    min_usage: 5
});

// Fuzzy search
const fuzzyResults = searchEngine.fuzzySearch("documntation", 5);

// Get suggestions
const suggestions = searchEngine.getSuggestions("doc", 10);

// Statistics
const stats = tagManager.getTagStatistics();
console.log(`Total tags: ${stats.total_tags}`);
console.log(`Total categories: ${stats.total_categories}`);
console.log(`Average usage: ${stats.average_usage}`);
```

**Performance:** O(1) for tag lookups, O(n) for search operations, O(n×m) for fuzzy search where m is query length.

**Design Patterns:** Builder pattern for TagUpdate, Repository pattern for TagManager, Strategy pattern for search algorithms.

---

### TUI (Text User Interface)

The Terminal User Interface provides an interactive, color-coded terminal interface for task management with keyboard navigation and real-time updates.

**Key Features:**
- Color-coded terminal interface with customizable color schemes
- Keyboard navigation for task browsing and editing
- Real-time task updates and status changes
- AI-powered suggestions and semantic analysis
- Task filtering and sorting capabilities
- Project organization and filtering
- Data visualization and analytics
- True color and ANSI fallback support
- Task editing with inline modifications
- Similar task detection and recommendations

**Architecture:**
The `TodoziApp` class serves as the main application controller with state management. The `TuiService` handles terminal rendering and display logic. The system integrates with `EmbeddingService` for AI-powered features. Color schemes support both true color and ANSI fallback for compatibility.

**Data Models:**
- `TodoziApp`: Main application controller
- `TuiService`: Terminal UI service for rendering
- `TaskDisplay`: Task display with AI insights
- `ColorScheme`: Color configuration management
- `DisplayConfig`: Display configuration options
- `TaskFilters`: Filtering criteria for tasks
- `EditSession`: Task editing session management

**Usage Example:**
```javascript
const app = new TodoziApp();
const tuiService = new TuiService();

// Launch GUI
await app.launchGui();

// Configure color scheme
const colorScheme = ColorScheme.indexedColorScheme();
colorScheme.primary = "#385898";

// Display task with AI insights
const taskDisplay = new TaskDisplay({
    task: task,
    similarTasks: similarTasks,
    aiSuggestions: suggestions,
    semanticTags: tags
});

// Render task
const rendered = taskDisplay.render(displayConfig);
console.log(rendered);

// Filter tasks
const filters = new TaskFilters({
    project: "my-project",
    status: Status.Todo,
    priority: Priority.High
});

const filteredTasks = app.getTasks(filters);
```

**Color Support:**
- True color support for modern terminals
- ANSI fallback for older terminals
- Indexed color scheme for compatibility
- Customizable color palette

**Performance:** O(n) for rendering where n is number of tasks. Real-time updates use efficient diff algorithms.

**Design Patterns:** MVC pattern for UI separation, Observer pattern for state updates, Strategy pattern for rendering.

---

### Migration System

The Task Migration System provides comprehensive migration capabilities for upgrading legacy task data to project-based organizational structure with dry-run mode and verbose logging.

**Key Features:**
- Legacy task data migration to project-based structure
- Dry-run mode for safe migration testing
- Verbose logging for detailed migration tracking
- Force overwrite protection
- Automatic cleanup of legacy data
- Migration validation and reporting
- Grouping tasks by project automatically
- Migration summary generation

**Architecture:**
The `TaskMigrator` class handles the migration process with configuration options for dry-run, verbose logging, and force overwrite. The system loads legacy tasks, groups them by project, and migrates them to project containers. A `MigrationCli` provides command-line interface for migration operations.

**Data Models:**
- `TaskMigrator`: Main migration class with configuration
- `MigrationCli`: CLI interface for migration
- `MigrationReport`: Migration results and statistics

**Usage Example:**
```javascript
const migrator = TaskMigrator.new()
    .withDryRun(true)
    .withVerbose(true)
    .withForceOverwrite(false);

// Execute migration
const report = await migrator.migrate();

// Print summary
migrator.printSummary(report);

// Validate migration
const isValid = migrator.validateMigration(report);

// Cleanup legacy data (after validation)
if (isValid && !migrator.dry_run) {
    await migrator.cleanupLegacy();
}
```

**Migration Process:**
1. Load legacy tasks from old collections
2. Group tasks by project (based on parent_project field)
3. Create project containers if they don't exist
4. Migrate tasks to project containers
5. Validate migration results
6. Generate migration report
7. Optionally cleanup legacy data

**Performance:** O(n) where n is number of tasks. Migration is performed in batches for efficiency.

**Design Patterns:** Builder pattern for configuration, Strategy pattern for migration strategies.

---

### Summary System

The Summary Manager provides comprehensive functionality for managing, storing, and manipulating text summaries with associated metadata such as tags, priority levels, and context.

**Key Features:**
- Summary CRUD operations with UUID-based identification
- Priority-based categorization (Low, Medium, High, Critical)
- Tag-based organization and filtering
- Full-text search across summary content, tags, and context
- Statistics and analytics for summary distribution
- Text-based summary format parsing
- Recent summaries retrieval with limit support

**Architecture:**
The `SummaryManager` class maintains a Map of summaries with UUID keys and a separate Map for tag indexing. Summaries include content, priority, context, and tags. The system provides extensive querying capabilities including search and filtering by priority/tag.

**Data Models:**
- `Summary`: Core summary entity with content, priority, context, tags, timestamps
- `SummaryUpdate`: Builder pattern class for constructing summary updates
- `SummaryStatistics`: Aggregated statistics with percentage calculations
- `SummaryPriority`: Enum for priority levels (Low, Medium, High, Critical)

**Usage Example:**
```javascript
const summaryManager = new SummaryManager();

// Create summary
const summaryId = await summaryManager.createSummary({
    content: "Project completed successfully",
    priority: SummaryPriority.High,
    context: "Final project delivery",
    tags: ["project", "completion", "success"]
});

// Search summaries
const results = summaryManager.searchSummaries("project");

// Filter by priority
const highPriority = summaryManager.getSummariesByPriority(SummaryPriority.High);

// Filter by tag
const projectSummaries = summaryManager.getSummariesByTag("project");

// Statistics
const stats = summaryManager.getSummaryStatistics();
console.log(`Total summaries: ${stats.totalSummaries}`);
```

**Summary Format Parsing:**
Summaries can be parsed from text format:
`<summary>content; priority; context; tags</summary>`

**Performance:** O(1) for CRUD operations, O(n×m) for search operations.

**Design Patterns:** Builder pattern for SummaryUpdate, Repository pattern for SummaryManager.

---

### Test Framework

The Testing Framework provides comprehensive testing utilities and frameworks for unit, integration, and performance testing with test fixtures and assertion helpers.

**Key Features:**
- Unit test utilities for all system components
- Integration test support for cross-module testing
- Performance testing utilities
- Mock objects and test fixtures
- Assertion helpers for common validations
- Test runner with reporting
- Coverage analysis support
- Test data generation utilities

**Architecture:**
The test framework provides utilities for testing all Todozi components including models, storage, handlers, and services. Test fixtures provide pre-configured test data. Mock objects allow isolated unit testing. Performance tests measure operation timing and resource usage.

**Usage Example:**
```javascript
// Unit test example
describe('TaskManager', () => {
    it('should create task with unique ID', async () => {
        const manager = new TaskManager();
        const task = await manager.createTask({
            action: "Test task",
            priority: Priority.High
        });
        expect(task.id).toBeDefined();
        expect(typeof task.id).toBe('string');
    });
});

// Integration test example
describe('Storage Integration', () => {
    it('should persist tasks across sessions', async () => {
        const storage = await Storage.new();
        const task = await storage.addTaskToProject("test-project", taskData);
        const retrieved = await storage.getTaskFromProject("test-project", task.id);
        expect(retrieved).toEqual(task);
    });
});

// Performance test example
describe('Performance', () => {
    it('should handle 1000 tasks efficiently', async () => {
        const startTime = Date.now();
        for (let i = 0; i < 1000; i++) {
            await manager.createTask({ action: `Task ${i}` });
        }
        const duration = Date.now() - startTime;
        expect(duration).toBeLessThan(5000);
    });
});
```

**Test Types:**
- Unit tests for individual components
- Integration tests for component interactions
- Performance tests for operation timing
- Security tests for input validation
- End-to-end tests for complete workflows

**Design Patterns:** Factory pattern for test data, Builder pattern for test setup.

---

### Type Definitions

The Type Definitions module provides TypeScript type definitions and runtime type checking for all system components with comprehensive type safety.

**Key Features:**
- TypeScript type definitions for all entities
- Runtime type validation
- Type conversion utilities
- Generic type support
- Union and intersection types
- Type guards for safe type checking
- Type inference helpers

**Architecture:**
The types module exports TypeScript interfaces and types for all Todozi components. Runtime validation functions ensure type safety at runtime. Type guards provide safe type narrowing. Conversion utilities handle type transformations.

**Usage Example:**
```typescript
import { Task, Priority, Status, TaskUpdate } from './types';

// Type-safe task creation
const task: Task = {
    id: "task-123",
    action: "Complete documentation",
    priority: Priority.High,
    status: Status.Todo,
    // TypeScript will catch type errors
};

// Type guard
function isTask(obj: any): obj is Task {
    return obj && typeof obj.id === 'string' && typeof obj.action === 'string';
}

// Safe type checking
if (isTask(someObject)) {
    // TypeScript knows someObject is Task here
    console.log(someObject.action);
}
```

**Type Coverage:**
- All data models (Task, Project, Agent, Memory, Idea, etc.)
- All enums (Priority, Status, Assignee, etc.)
- All configuration types
- All API request/response types
- All error types

**Design Patterns:** Type guard pattern for runtime type checking, Factory pattern for type creation.

---

## Installation

### Prerequisites

Before installing the Todozi JavaScript SDK, ensure you have the following prerequisites:

- **Node.js**: Version 14.0 or higher (LTS version recommended)
- **npm**: Version 6.0 or higher (comes with Node.js)
- **Git**: For cloning the repository (optional, can download as ZIP)
- **Operating System**: Windows, macOS, or Linux

### Installation Methods

#### Method 1: Clone from Repository

```bash
# Clone the repository
git clone https://github.com/todozi/tdz_js.git
cd tdz_js

# Install dependencies
npm install

# Build the project (if TypeScript or build step required)
npm run build

# Run tests to verify installation
npm test
```

#### Method 2: Install via npm (if published)

```bash
# Install as a dependency in your project
npm install @todozi/tdz_js

# Or install globally
npm install -g @todozi/tdz_js
```

#### Method 3: Development Installation

```bash
# Clone repository
git clone https://github.com/todozi/tdz_js.git
cd tdz_js

# Install dependencies including dev dependencies
npm install

# Link for local development
npm link

# In your project, link to local development version
npm link @todozi/tdz_js
```

### Post-Installation Setup

After installation, you may need to configure Todozi:

```bash
# Initialize Todozi configuration
todozi init

# Or programmatically
node -e "const { Storage } = require('@todozi/tdz_js'); Storage.new().then(s => s.initialize())"
```

### Verification

Verify your installation:

```bash
# Check version
node -e "const pkg = require('@todozi/tdz_js/package.json'); console.log(pkg.version)"

# Run basic test
node -e "const { TodoziHandler } = require('@todozi/tdz_js'); console.log('Todozi SDK loaded successfully')"
```

### Troubleshooting Installation

**Issue: Module not found**
- Ensure Node.js version is 14.0 or higher: `node --version`
- Clear npm cache: `npm cache clean --force`
- Delete `node_modules` and reinstall: `rm -rf node_modules && npm install`

**Issue: Build errors**
- Ensure all dependencies are installed: `npm install`
- Check Node.js version compatibility
- Review build logs for specific error messages

**Issue: Permission errors**
- Use `sudo` for global installation (Linux/macOS): `sudo npm install -g @todozi/tdz_js`
- Or configure npm to use a different directory: `npm config set prefix ~/.npm-global`

## Quick Start

### Basic Usage

Get started with Todozi in just a few lines of code:

```javascript
const { TodoziHandler } = require('@todozi/tdz_js');
const { Storage } = require('@todozi/tdz_js');

async function main() {
    // Initialize storage
    const storage = await Storage.new();
    
    // Initialize handler
    const handler = await TodoziHandler.new(storage);
    
    // Create a task
    await handler.handleAddCommand({
        type: 'Task',
        action: 'Learn Todozi SDK',
        time: '2 hours',
        priority: 'HIGH',
        project: 'learning',
        status: 'TODO'
    });
    
    // List tasks
    const tasks = await handler.handleListCommand({
        type: 'Tasks',
        project: 'learning'
    });
    
    console.log('Tasks:', tasks);
}

main().catch(console.error);
```

### Advanced Usage

Here's a more comprehensive example showing multiple features:

```javascript
const { TodoziHandler, Storage, Priority, Status, Assignee } = require('@todozi/tdz_js');

async function advancedExample() {
    // Initialize
    const storage = await Storage.new();
    const handler = await TodoziHandler.new(storage);
    
    // Create a project
    await handler.handleProjectCommand({
        type: 'CreateProject',
        name: 'my-project',
        description: 'My first Todozi project'
    });
    
    // Create multiple tasks
    const tasks = [
        {
            type: 'Task',
            action: 'Set up development environment',
            time: '1 hour',
            priority: Priority.HIGH,
            project: 'my-project',
            status: Status.TODO,
            assignee: Assignee.HUMAN
        },
        {
            type: 'Task',
            action: 'Write documentation',
            time: '3 hours',
            priority: Priority.MEDIUM,
            project: 'my-project',
            status: Status.TODO,
            assignee: Assignee.AI
        }
    ];
    
    for (const taskData of tasks) {
        await handler.handleAddCommand(taskData);
    }
    
    // Search for tasks
    const searchResults = await handler.handleSearchAllCommand({
        type: 'SearchAll',
        query: 'documentation',
        types: 'tasks'
    });
    
    console.log('Search results:', searchResults);
    
    // Create a memory
    await handler.handleMemoryCommand({
        type: 'CreateMemory',
        moment: 'Started using Todozi',
        meaning: 'Beginning of productivity journey',
        reason: 'Want to organize tasks better',
        importance: 'high',
        term: 'long'
    });
    
    // Create an idea
    await handler.handleIdeaCommand({
        type: 'CreateIdea',
        idea: 'Build a Todozi mobile app',
        share: 'team',
        importance: 'high',
        tags: 'mobile,app,productivity',
        context: 'Expand Todozi to mobile platforms'
    });
    
    // Get statistics
    const stats = await handler.handleStatsCommand({
        type: 'Stats'
    });
    
    console.log('Statistics:', stats);
}

advancedExample().catch(console.error);
```

### Using the Core Module Directly

You can also use the core module for more direct control:

```javascript
const { Done } = require('@todozi/tdz_js/lib');

async function coreExample() {
    // Initialize Todozi
    const todozi = new Done();
    await todozi.init();
    
    // Create a task
    const task = await todozi.createTask(
        'Complete project documentation',
        'high',
        'my-project',
        '4 hours',
        'Write comprehensive API documentation'
    );
    
    console.log('Created task:', task);
    
    // Search tasks
    const results = await todozi.searchTasks('documentation', true, 10);
    console.log('Search results:', results);
    
    // Create a memory
    const memory = await todozi.createMemory(
        'Project milestone reached',
        'Successfully completed first phase',
        'Team effort and good planning'
    );
    
    console.log('Created memory:', memory);
    
    // Create an idea
    const idea = await todozi.createIdea(
        'Implement automated testing',
        'Improve code quality and reliability'
    );
    
    console.log('Created idea:', idea);
}

coreExample().catch(console.error);
```

### Using the Embedding Service

For semantic search capabilities:

```javascript
const { TodoziEmbeddingService } = require('@todozi/tdz_js/emb');

async function embeddingExample() {
    const config = {
        model_name: 'sentence-transformers/all-MiniLM-L6-v2',
        similarity_threshold: 0.7,
        max_results: 20
    };
    
    const service = await TodoziEmbeddingService.new(config);
    await service.initialize('cpu');
    
    // Add tasks with embeddings
    await service.addTask({
        id: 'task-1',
        action: 'Complete API documentation',
        context_notes: 'Write comprehensive REST API documentation'
    });
    
    // Find similar tasks
    const similar = await service.findSimilarTasks(
        'Write technical documentation',
        10
    );
    
    console.log('Similar tasks:', similar);
    
    // Semantic search
    const results = await service.semanticSearch(
        'documentation tasks',
        ['Task', 'Memory'],
        20
    );
    
    console.log('Semantic search results:', results);
}

embeddingExample().catch(console.error);
```

### Error Handling

Always handle errors appropriately:

```javascript
const { TodoziHandler, TodoziError } = require('@todozi/tdz_js');

async function errorHandlingExample() {
    try {
        const storage = await Storage.new();
        const handler = await TodoziHandler.new(storage);
        
        // Operations that might throw errors
        await handler.handleAddCommand({
            type: 'Task',
            action: 'Test task',
            priority: 'INVALID_PRIORITY' // This will throw an error
        });
    } catch (error) {
        if (error instanceof TodoziError) {
            console.error('Todozi Error:', error.type, error.message);
        } else {
            console.error('Unexpected error:', error);
        }
    }
}

errorHandlingExample();
```

### Next Steps

After getting started, explore:

1. **CLI Usage**: Use the command-line interface for quick operations
2. **API Integration**: Integrate Todozi into your applications
3. **Advanced Features**: Explore semantic search, AI agents, and workflows
4. **Customization**: Configure Todozi to match your workflow
5. **Contributing**: Contribute to the Todozi project

For more examples and detailed documentation, see the [examples directory](./examples) and [full documentation](./docs).

---

## Additional Module Details

### Todozi Command System (tdz)

The Todozi Command System provides functionality for parsing and executing custom `<tdz>` commands that allow interaction with the Todozi API through simple markup syntax. The system supports various operations including listing, creating, updating, deleting, and running different types of resources.

**Key Features:**
- Custom `<tdz>` command parsing
- Support for multiple resource types (tasks, agents, memories, ideas, etc.)
- Command execution with API integration
- Request body building for different operations
- Endpoint path resolution
- Error handling for invalid commands

**Usage:**
```javascript
const { processTdzCommands, executeTdzCommand } = require('@todozi/tdz_js/tdz');

// Process multiple commands
const commands = "<tdz>list tasks</tdz><tdz>create task: Complete docs</tdz>";
const results = await processTdzCommands(commands, apiKey);

// Execute single command
const result = await executeTdzCommand('list', 'tasks', [], {}, apiKey);
```

---

### Todozi Content Processor (tdz_tls)

The Todozi Content Processor Tool extracts actionable items, checklist entries, and metadata from AI-generated content. It handles both structured JSON input and natural language processing to identify tasks, ideas, and memory markers.

**Key Features:**
- Content parsing from JSON and plain text
- Custom XML-style tag extraction
- Natural language action item recognition
- Session management and topic tracking
- Automatic checklist generation
- Tool call processing
- Statistics tracking

**Usage:**
```javascript
const { processContent } = require('@todozi/tdz_js/tdz_tls');

const content = "Meeting notes: We need to complete the documentation (high priority)";
const result = await processContent(content);
console.log('Extracted items:', result.checklistItems);
```

---

### Todozi Tool Executor (todozi_exe)

The Todozi Tool Executor provides a unified interface for interacting with the Todozi task management system. It offers both simple command-based actions and advanced API integrations, supporting various task creation, search, and management operations.

**Key Features:**
- 20+ different action types for task management
- Comprehensive error types with detailed messaging
- Thread-safe mutex-based synchronization
- Secure API communication
- Support for both local and remote operations

**Usage:**
```javascript
const { executeTodoziToolDelegated } = require('@todozi/tdz_js/todozi_exe');

const result = await executeTodoziToolDelegated({
    action: 'create_task',
    action_input: {
        action: 'Complete documentation',
        priority: 'high',
        project: 'my-project'
    }
});
```

---

### Todozi System (todozi_tool)

The Todozi System is a comprehensive task and project management framework designed for AI agents and human collaboration. It provides a rich set of tools for creating, managing, searching, and organizing tasks, memories, ideas, errors, and code chunks.

**Key Features:**
- Comprehensive tool system with 20+ tools
- Semantic search capabilities
- Automatic AI/human coordination
- Structured data extraction from natural language
- Multi-format data support

**Usage:**
```javascript
const { CreateTaskTool, SearchTasksTool } = require('@todozi/tdz_js/todozi_tool');

const createTool = new CreateTaskTool();
const result = await createTool.execute({
    action: 'Complete documentation',
    priority: 'high'
});
```

---

### Type Definitions (types)

The Type Definitions module provides comprehensive TypeScript type definitions and runtime type checking for all system components. It includes all data structures, command references, and search engine types.

**Key Features:**
- Complete type definitions for all entities
- Command reference types
- Search engine type definitions
- Runtime type validation
- Type guards for safe type checking

**Usage:**
```typescript
import { Task, Priority, Status, SearchOptions, SearchResults } from '@todozi/tdz_js/types';

const task: Task = {
    id: 'task-123',
    action: 'Complete docs',
    priority: Priority.HIGH,
    status: Status.TODO
};
```

---

### Test Framework (tests)

The Test Framework provides comprehensive testing utilities and frameworks for unit, integration, and performance testing. It includes test fixtures, assertion helpers, and test data generation utilities.

**Key Features:**
- Unit test utilities for all components
- Integration test support
- Performance testing utilities
- Mock objects and test fixtures
- Assertion helpers
- Test runner with reporting

**Usage:**
```javascript
const { testTaskCreation, testStorageIntegration } = require('@todozi/tdz_js/tests');

describe('Task Management', () => {
    it('should create tasks', testTaskCreation);
    it('should persist tasks', testStorageIntegration);
});
```

---

### Main CLI (main)

The Main CLI module provides the command-line interface entry point for the Todozi application. It handles command parsing, routing, and execution with support for both interactive and non-interactive modes.

**Key Features:**
- Command-line argument parsing
- Interactive mode support
- Command routing and execution
- Help and version information
- Configuration management
- Error handling and reporting

**Usage:**
```bash
# Command-line usage
todozi add "Complete documentation" --priority high --project my-project
todozi list --project my-project
todozi search "documentation"
```

---

## Contributing

We welcome contributions to the Todozi JavaScript SDK! Here's how you can help:

### Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a branch** for your changes
4. **Make your changes** and test thoroughly
5. **Submit a pull request** with a clear description

### Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/tdz_js.git
cd tdz_js

# Install dependencies
npm install

# Run tests
npm test

# Run linter
npm run lint

# Build the project
npm run build
```

### Code Style

- Follow the existing code style and conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Write tests for new features
- Update documentation as needed

### Testing

- Write unit tests for new features
- Ensure all tests pass before submitting
- Add integration tests for complex features
- Test error cases and edge conditions

### Documentation

- Update README.md for user-facing changes
- Add JSDoc comments for new functions/classes
- Update examples if API changes
- Keep documentation in sync with code

### Pull Request Process

1. Ensure your code follows the style guide
2. Write or update tests as needed
3. Update documentation
4. Ensure all tests pass
5. Submit PR with clear description
6. Respond to review feedback

### Reporting Issues

When reporting issues, please include:

- Description of the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (Node.js version, OS, etc.)
- Error messages or logs

### Feature Requests

For feature requests, please:

- Describe the feature clearly
- Explain the use case
- Discuss potential implementation approaches
- Consider backward compatibility

### Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different viewpoints and experiences

Thank you for contributing to Todozi!

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Support

For support, please:

- Check the [documentation](./docs)
- Search [existing issues](https://github.com/todozi/tdz_js/issues)
- Create a [new issue](https://github.com/todozi/tdz_js/issues/new) if needed
- Join our [community discussions](https://github.com/todozi/tdz_js/discussions)

---

## Acknowledgments

- Thanks to all contributors who have helped improve Todozi
- Special thanks to the open-source community
- Built with ❤️ for productivity and organization

---

*Last updated: 2024*

