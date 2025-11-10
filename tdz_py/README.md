# Todozi Python SDK

A comprehensive task management and AI-powered productivity system built with Python. This SDK provides structured parsing, execution, and management of various types of content including tasks, memories, ideas, errors, and more, with full type safety and robust error handling.

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
- [Type Safety](#type-safety)
- [Contributing](#contributing)

## Overview

Todozi Python SDK is a sophisticated task management platform that combines traditional task tracking with AI-powered features. Built with Python's type system and dataclasses, the system provides:

- **Task Management**: Create, update, and track tasks with rich metadata and type safety
- **AI Agent System**: Comprehensive agent management with capabilities and specializations
- **Semantic Search**: Find similar tasks and content using sentence-transformers embeddings
- **Memory Management**: Store and retrieve contextual memories with importance ratings
- **Idea Tracking**: Capture and organize ideas with sharing levels and importance
- **Queue System**: Prioritize and process tasks in structured workflows
- **API Management**: Secure API key generation with dual-key authentication
- **Content Extraction**: Extract structured data from unstructured text using AI
- **Result Type Pattern**: Rust-inspired error handling without exceptions

---

## Core Modules

### Agent Management

The Agent Management System provides a comprehensive framework for managing AI agents and their task assignments with JSON-backed storage and atomic operations.

**Key Features:**
- Agent lifecycle management (create, update, delete) with UUID-based identification
- Agent assignment to tasks with status tracking (Assigned, Completed)
- Agent availability management (Available, Busy, Inactive)
- Specialization and capability-based agent filtering
- Performance statistics tracking with completion rate calculations
- Atomic file operations for data consistency
- In-memory caching for performance optimization

**Architecture:**
The system uses a three-layer architecture: Data Models → Storage Layer → Manager Layer. Agents are stored as dataclasses with immutable metadata. The storage layer uses JSON files with atomic write operations via temporary files and atomic replacement. The `AgentManager` class provides a repository pattern interface for all agent operations.

**Data Models:**
- `Agent`: Core agent entity with id, name, description, capabilities, specializations, and metadata
- `AgentMetadata`: Tracks agent operational status (Available, Busy, Inactive)
- `AgentAssignment`: Links agents to tasks with project context and timestamps
- `AgentUpdate`: Builder pattern class for constructing agent updates
- `AgentStatistics`: Aggregated statistics with completion rate calculations

**Usage Example:**
```python
from todozi.agent import AgentManager, AgentStatus, ShareLevel

# Initialize manager
manager = AgentManager()

# Create agent
agent_id = manager.create_agent(
    name="Research Assistant",
    description="AI agent for research tasks",
    capabilities=["research", "analysis"],
    specializations=["academic", "scientific"]
)

# Find best agent
best_agent = manager.find_best_agent(
    required_specialization="academic",
    preferred_capability="research"
)

# Assign task
manager.assign_task_to_agent(
    task_id="task-123",
    agent_id=best_agent.id,
    project_id="project-456"
)

# Get statistics
stats = manager.get_agent_statistics()
print(f"Completion rate: {stats.completion_rate()}%")
```

**Performance:** O(1) for agent lookups using dictionaries, O(n) for filtering operations. Atomic file operations ensure data consistency. In-memory caching reduces file I/O overhead.

**Design Patterns:** Data Class pattern for immutable structures, Builder pattern for AgentUpdate, Context Manager pattern for atomic operations, Repository pattern for data access.

**Storage:** JSON-based file storage at `agents.json` with UTF-8 encoding. Atomic writes use temporary files and atomic replacement to prevent corruption.

---

### API Key Management

The Todozi API Key Management System provides secure generation, storage, and verification of API keys using a dual-key authentication approach with cryptographically secure random generation.

**Key Features:**
- Dual-key authentication (public/private key pairs)
- Cryptographically secure random generation using `secrets.token_urlsafe(32)`
- User-based API key association with UUID identifiers
- Key lifecycle management (activate/deactivate without deletion)
- Dual-indexed collections for O(1) lookups by user_id or public_key
- JSON-based persistent storage with atomic operations
- Admin-level authentication requiring both keys

**Architecture:**
The system uses `ApiKey` dataclasses with public/private key pairs. The `ApiKeyCollection` maintains dual indices: `_keys_by_user_id` and `_keys_by_public` for efficient lookups. Storage uses JSON files with atomic write operations.

**Data Models:**
- `ApiKey`: Contains user_id, public_key (32-byte URL-safe), private_key (32-byte URL-safe), and active status
- `ApiKeyCollection`: Manages collections with dual indexing for O(1) lookups

**Usage Example:**
```python
from todozi.api import ApiKey, ApiKeyCollection, save_api_key_collection, load_api_key_collection

# Generate new key
api_key = ApiKey.new()
print(f"Public: {api_key.public_key}")
print(f"Private: {api_key.private_key}")

# Create with user ID
user_key = ApiKey.with_user_id("user-123")

# Manage collection
collection = ApiKeyCollection()
collection.add_key(user_key)

# Lookup operations
found_key = collection.get_key("user-123")  # O(1)
found_by_public = collection.get_key_by_public(api_key.public_key)  # O(1)

# Authentication
if api_key.matches(public_key, private_key):
    print("Authentication successful")

# Admin check
if api_key.is_admin(public_key, private_key):
    print("Admin access granted")

# Persistence
save_api_key_collection(collection)
loaded = load_api_key_collection()
```

**Security:** Uses `secrets.token_urlsafe(32)` providing 256 bits of entropy. Private keys should never be logged. File permissions should be restricted. Admin operations require full key verification.

**Performance:** O(1) for all key operations using dual-indexed dictionaries. File I/O is performed atomically to prevent corruption.

**Storage:** Default location `~/.todozi/api/api_keys.json`. Uses atomic file operations with temporary files and atomic replacement.

---

### Base Tool Framework

A production-ready Python translation of a Rust tool library implementing a comprehensive tool management system with type safety, error resilience, and serialization capabilities.

**Key Features:**
- Type-safe tool parameter definitions with JSON-compatible types
- Resource locking system for concurrent operation safety
- Comprehensive error handling with detailed categorization
- Full JSON and Ollama API format serialization
- Tool registry for managing tool collections
- Validation system for tool definitions
- Configuration-driven behavior through ToolConfig

**Architecture:**
The framework consists of enums (ResourceLock, ErrorType), data structures (ToolParameter, ToolDefinition, ToolResult), error handling (ToolError, ErrorHandler), and core abstractions (Tool, ToolRegistry). Tools extend the base Tool class and implement execute methods.

**Core Components:**
- `ResourceLock`: Defines resource access restrictions (FILESYSTEM_WRITE, FILESYSTEM_READ, GIT, MEMORY, SHELL, NETWORK)
- `ToolParameter`: Defines parameters with name, type, description, required flag, and default value
- `ToolDefinition`: Complete tool specification with parameters, category, and resource locks
- `ToolResult`: Standardized result container with success status, output, error, execution time, and metadata
- `ToolRegistry`: Manages tool collections with registration and execution capabilities

**Usage Example:**
```python
from todozi.base import Tool, ToolParameter, ToolDefinition, ResourceLock, ToolRegistry

class FileReadTool(Tool):
    def definition(self) -> ToolDefinition:
        return ToolDefinition(
            name="file_read",
            description="Read file contents",
            parameters=[
                ToolParameter(
                    name="path",
                    type_="string",
                    description="Path to file",
                    required=True
                )
            ],
            category="File Operations",
            resource_locks=[ResourceLock.FILESYSTEM_READ]
        )
    
    def execute(self, **kwargs) -> ToolResult:
        path = kwargs.get("path")
        try:
            with open(path, 'r') as f:
                content = f.read()
            return ToolResult.success(content, execution_time_ms=50)
        except Exception as e:
            return ToolResult.error(str(e), execution_time_ms=10)

# Register and use
registry = ToolRegistry()
registry.register(FileReadTool())

# Serialize to Ollama format
ollama_format = registry.get_tool_definitions()[0].to_ollama_format()

# Execute tool
result = registry.execute_tool("file_read", path="/path/to/file.txt")
if result.success:
    print(result.output)
```

**Design Patterns:** Factory pattern for tool creation, Registry pattern for tool management, Template Method pattern for tool interface, Builder pattern for complex objects.

**Type Safety:** Full type validation for JSON-compatible types (string, number, boolean, array, object, integer, null). Parameter validation ensures type correctness.

**Serialization:** Tools can be serialized to Ollama LLM tool-call API format for integration with language models.

---

### Chunking System

The Todozi Code Chunking System manages code generation tasks through hierarchical chunking with dependency management and state tracking for complex software development workflows.

**Key Features:**
- Hierarchical code chunking with predefined levels (Project, Module, Class, Method, Block)
- Token limits for each chunking level (100, 500, 1000, 300, 100 tokens respectively)
- Dependency management between code chunks
- Project state tracking with line count and module completion
- Context window management for maintaining generation context
- Chunk status tracking through defined lifecycle (Pending, InProgress, Completed, Validated, Failed)
- Rust-inspired Result type for error handling

**Architecture:**
The system uses a `CodeGenerationGraph` that manages `CodeChunk` objects with dependencies. Each chunk has a `ChunkingLevel` with token limits and a `ChunkStatus` for lifecycle tracking. The system maintains `ProjectState` for overall project tracking and `ContextWindow` for generation context.

**Data Models:**
- `ChunkingLevel`: Enum defining hierarchical levels with token limits and descriptions
- `ChunkStatus`: Enum for chunk lifecycle states
- `ProjectState`: Manages total lines, max lines, current module, dependencies, completed/pending modules, global variables
- `ContextWindow`: Tracks previous/current class, next planned items, global vars in scope, imports, function signatures, error patterns
- `CodeChunk`: Individual code unit with chunk_id, status, dependencies, code, tests, validated flag, level, estimated tokens

**Usage Example:**
```python
from todozi.chunking import (
    CodeGenerationGraph, ChunkingLevel, ChunkStatus,
    ProjectState, ContextWindow
)

# Create graph with max lines
graph = CodeGenerationGraph(max_lines=10000)

# Add chunks with dependencies
graph.add_chunk("project-setup", ChunkingLevel.PROJECT, [])
graph.add_chunk("database-module", ChunkingLevel.MODULE, ["project-setup"])
graph.add_chunk("user-class", ChunkingLevel.CLASS, ["database-module"])
graph.add_chunk("user-create-method", ChunkingLevel.METHOD, ["user-class"])

# Get ready chunks (dependencies satisfied)
ready_chunks = graph.get_ready_chunks()

# Get next chunk to work on
next_chunk_id = graph.get_next_chunk_to_work_on()
if next_chunk_id:
    chunk = graph.get_chunk(next_chunk_id)
    
    # Update chunk code
    graph.update_chunk_code(next_chunk_id, generated_code)
    graph.update_chunk_tests(next_chunk_id, generated_tests)
    
    # Mark as completed
    graph.mark_chunk_completed(next_chunk_id)
    graph.mark_chunk_validated(next_chunk_id)

# Get project summary
summary = graph.get_project_summary()
print(summary)

# Get dependency chain
chain = graph.get_dependency_chain("user-create-method")
# Returns: ["project-setup", "database-module", "user-class", "user-create-method"]
```

**Performance:** O(1) for chunk creation, O(n×d) for ready chunks calculation where n is chunks and d is average dependencies, O(n+e) for dependency chain building where e is total dependencies.

**Design Patterns:** Factory pattern for chunk creation, Composite pattern for chunk relationships, State pattern for chunk lifecycle, Builder pattern for parsing, Observer pattern for state updates.

**Result Type:** Uses Rust-inspired `Result[Ok[T], Err[E]]` type for error handling without exceptions, making error paths predictable and composable.

---

### CLI Handler

The Todozi CLI provides a comprehensive command-line interface for the Todozi AI/Human task management system with unified access to all system features.

**Key Features:**
- Task management operations (create, update, delete, complete)
- API key management commands (register, list, check, activate/deactivate, remove)
- Queue management (plan, list, start, end sessions)
- Server management (start, status, endpoints)
- Unified search across all content types
- Chat message processing with tag extraction
- Error tracking and resolution
- Training data management
- Agent management operations
- Memory and idea management

**Architecture:**
The CLI uses a `TodoziHandler` class that processes commands through specialized handlers. Commands are structured as dataclasses with type safety. The system integrates seamlessly with the storage layer for data persistence.

**Command Structure:**
Commands are organized into categories:
- `ApiCommands`: Register, ListKeys, CheckKeys, DeactivateKey, ActivateKey, RemoveKey
- `QueueCommands`: PlanQueue, ListQueue, BacklogQueue, ActiveQueue, CompleteQueue, StartQueue, EndQueue
- `ServerCommands`: StartServer, ServerStatus, ServerEndpoints
- `SearchAll`: Unified search across tasks, memories, ideas, errors, training data
- `ChatCommands`: Process messages with tag extraction
- `ErrorCommands`: CreateError, ListErrors, ShowError, ResolveError, DeleteError
- `TrainingCommands`: CreateTraining, ListTraining, ShowTraining, DeleteTraining

**Usage Example:**
```python
from todozi.cli import TodoziHandler
from todozi.storage import Storage
from todozi.cli.commands import Register, PlanQueue, SearchAll

# Initialize
storage = Storage()
handler = TodoziHandler(storage)

# API key management
await handler.handle_api_command(Register(user_id="user-123"))

# Queue operations
await handler.handle_queue_command(
    PlanQueue(
        task_name="Code review",
        task_description="Review PR #123",
        priority="high",
        project_id="engineering"
    )
)

# Unified search
await handler.handle_search_all_command(
    SearchAll(query="documentation", types="tasks,ideas")
)

# Chat processing
content = handler.process_chat_message_extended(
    "Meeting notes: <todozi>Review budget; 1 week; high; finance; todo</todozi>",
    user_id="user-123"
)
```

**Chat Tag Support:**
The system supports XML-like tags for structured content:
- `<todozi>action|time|priority|project|status</todozi>` - Tasks
- `<memory>moment|meaning|reason|importance|term</memory>` - Memories
- `<idea>idea|share|importance</idea>` - Ideas
- `<error>title|description|severity|category</error>` - Errors
- `<train>prompt|completion|data_type</train>` - Training data

**Error Handling:** All commands use Result types for error handling. Invalid commands return descriptive error messages.

---

### Embedding Service

The Todozi Embedding Service provides comprehensive semantic embedding capabilities using sentence-transformers models for task management and content organization.

**Key Features:**
- Text embedding generation using pre-trained sentence-transformers models
- Semantic similarity search with cosine similarity
- Content clustering based on embedding similarity
- In-memory caching with TTL management
- Batch processing for multiple embeddings
- Multiple embedding model support
- Content-type agnostic unified embedding approach
- Hierarchical caching with LRU eviction

**Architecture:**
The service uses `TodoziEmbeddingService` as the main interface, wrapping `EmbeddingModel` which interfaces with sentence-transformers. Embeddings are cached in memory with TTL-based expiration. The system supports multiple content types (Task, Tag, Idea, Memory) with unified embedding generation.

**Dependencies:**
- `sentence-transformers >= 2.2.0`
- `torch >= 1.9.0`
- `transformers >= 4.20.0`
- `numpy >= 1.21.0`
- `pydantic >= 1.9.0`

**Configuration:**
- `model_name`: Pre-trained model identifier (default: "sentence-transformers/all-MiniLM-L6-v2")
- `dimensions`: Expected embedding dimensions (default: 384)
- `similarity_threshold`: Minimum similarity score (default: 0.7)
- `max_results`: Maximum results per query (default: 50)
- `cache_ttl_seconds`: Cache time-to-live (default: 86400)
- `enable_clustering`: Enable clustering (default: True)
- `clustering_threshold`: Similarity threshold for clustering (default: 0.8)

**Usage Example:**
```python
from todozi.emb import TodoziEmbeddingService, TodoziEmbeddingConfig

# Create configuration
config = TodoziEmbeddingConfig(
    model_name="sentence-transformers/all-MiniLM-L6-v2",
    similarity_threshold=0.8,
    max_results=100
)

# Initialize service
service = TodoziEmbeddingService(config)
service.initialize(device="cpu")

# Generate embedding
embedding = service.generate_embedding("Complete documentation")

# Batch processing
texts = ["Task 1", "Task 2", "Task 3"]
embeddings = service.generate_embeddings_batch(texts)

# Add task with embedding
service.add_task({
    "id": "task-001",
    "action": "Complete documentation",
    "context_notes": "Write comprehensive documentation"
})

# Find similar tasks
similar = service.find_similar_tasks(
    "Write technical documentation",
    limit=10
)

# Semantic search
results = service.semantic_search(
    query="machine learning concepts",
    content_types=["Task", "Memory", "Idea"],
    limit=20
)

# Clustering
clusters = service.cluster_content()
for cluster in clusters:
    print(f"Cluster {cluster.cluster_id}: {cluster.cluster_size} items")
```

**Performance:** O(n) for semantic search (linear scan), O(n²) for clustering (pairwise similarity), O(d) for cosine similarity where d is vector dimensions. Cache lookups are O(1).

**Caching:** Multi-level caching strategy with LRU eviction. Cache entries include vector, content_type, content_id, text_content, tags, created_at, and ttl_seconds.

**Model Support:** Default model is "sentence-transformers/all-MiniLM-L6-v2" (384 dimensions). System supports any sentence-transformers model. Lazy loading on first use.

---

### Error Management

The Todozi Error Management System provides structured error definitions, centralized error management, and robust error parsing capabilities with comprehensive error types.

**Key Features:**
- Custom error class extending Python Exception with structured metadata
- Centralized error registry with lifecycle management
- Error parsing from formatted text strings
- Multiple error types (Validation, Storage, Config, IO, JSON, UUID, etc.)
- Error resolution tracking
- Comprehensive error metadata (severity, category, source, context, tags)
- Result type pattern for error handling

**Architecture:**
The system uses a `TodoziError` exception class with structured error information. The `ErrorManager` class maintains a registry of errors with resolution tracking. Error parsing functions extract structured error information from formatted text.

**Error Types:**
- `ValidationError`: Input validation failures
- `StorageError`: Storage operation failures
- `ConfigError`: Configuration errors
- `IoError`: Input/output errors
- `JsonError`: JSON parsing errors
- `UuidError`: UUID generation/parsing errors
- `ChronoError`: Date/time parsing errors
- `DialoguerError`: User input errors
- `HlxError`: HLX format errors
- `ReqwestError`: HTTP request errors
- `DirError`: Directory operation errors
- `EmbeddingError`: Embedding operation errors
- `ApiError`: API operation errors
- `CandleError`: Candle library errors
- `NotImplementedError`: Feature not implemented
- `SerializationError`: Serialization failures

**Usage Example:**
```python
from todozi.error import TodoziError, ErrorManager, parse_error_format

# Create specific errors
raise TodoziError.invalid_priority("extreme")
raise TodoziError.task_not_found(123)
raise TodoziError.validation_error("Invalid input format")

# Error manager
error_manager = ErrorManager()

# Create and register error
error_id = await error_manager.create_error(
    title="Database Connection Failed",
    description="Could not connect to database",
    severity="high",
    category="storage",
    source="DatabaseManager.connect",
    context="Connection timeout after 30 seconds",
    tags=["database", "connection", "timeout"]
)

# Get unresolved errors
unresolved = error_manager.get_unresolved_errors()

# Resolve error
await error_manager.resolve_error(
    error_id=error_id,
    resolution="Connection restored after restarting database service"
)

# Parse error from formatted text
error_text = "<error>Network Timeout;Connection timed out;high;network;api_client;Request ID: 12345;timeout,connection</error>"
parsed_error = parse_error_format(error_text)
```

**Error Format:**
Errors can be parsed from XML-like format:
`<error>title;description;severity;category;source;context;tags</error>`

**Performance:** O(1) for error creation and registration, O(n) for retrieving unresolved errors, O(1) for error resolution using dictionary lookups.

**Design Patterns:** Factory pattern for error creation, Registry pattern for error management, Parser pattern for text parsing, Result type pattern for error handling.

---

### Content Extraction

The Todozi Content Extraction System extracts structured data from unstructured text content using AI-powered APIs with support for multiple output formats.

**Key Features:**
- Multi-format input (inline text or file paths)
- AI-powered extraction using Todozi API
- Structured output formats (JSON, CSV, Markdown)
- Auto-embedding of extracted content to project files
- Human-readable checklist generation
- History tracking with comprehensive logs
- Support for multiple content types (tasks, memories, ideas, errors, training data)

**Architecture:**
The system uses `extract_content` and `strategy_content` functions that call Todozi API endpoints. Extracted content is parsed into structured objects and formatted according to output preferences. The system supports both "plan" and "strategic" extraction endpoints.

**Extraction Types:**
- **Plan Extraction**: General content extraction for tasks, memories, ideas
- **Strategic Extraction**: Strategic planning content with importance ratings

**Output Formats:**
- **JSON**: Structured JSON with all extracted entities
- **CSV**: Comma-separated values for spreadsheet import
- **Markdown**: Human-readable markdown format
- **Human Checklist**: Formatted checklist for review

**Usage Example:**
```python
from todozi.extract import extract_content, strategy_content

# Extract from inline content
result = extract_content(
    content="Meeting notes: Review Q4 budget (High priority, due next week)",
    file_path=None,
    output_format="json",
    human=True
)

# Extract from file
file_result = extract_content(
    content=None,
    file_path="/path/to/meeting-notes.txt",
    output_format="md",
    human=False
)

# Strategic content analysis
strategic_result = strategy_content(
    content="Company Q4 Strategy: Expand into European markets (High importance)",
    file_path=None,
    output_format="json",
    human=True
)

# CSV output
csv_result = extract_content(
    content="Project kickoff: Design UI mockups (High priority, due Friday) - John",
    file_path=None,
    output_format="csv",
    human=False
)
```

**Extracted Content Types:**
- **Tasks**: Action, time, priority, project, status, assignee, tags
- **Memories**: Moment, meaning, reason, importance, term
- **Ideas**: Idea text, share level, importance
- **Errors**: Title, description, severity, category
- **Training Data**: Prompt, completion, data_type

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

**Performance:** O(1) network latency + O(n) content processing. File operations are O(1) for single files. Formatting is O(n) where n is number of extracted items.

**Security:** API key management through `get_tdz_api_key()`, content privacy through Todozi API, secure file system permissions, HTTPS communication. Input validation and sanitization are implemented.

**History Logging:** Extracted tasks are automatically logged to `~/.todozi/history/mega_log.jsonl` for audit and analysis.

---

### Idea Management

The Todozi Idea Management System provides comprehensive functionality for organizing, categorizing, and managing ideas within a collaborative environment with sharing controls and importance ratings.

**Key Features:**
- Idea CRUD operations with UUID-based identification
- Tag-based organization and filtering
- Sharing control (Public, Team, Private)
- Importance ranking (Breakthrough, High, Medium, Low)
- Full-text search across ideas, tags, and context
- Analytics and statistics (tag usage, importance distribution, sharing levels)
- Text-based idea format parsing
- Recent ideas retrieval with limit support

**Architecture:**
The `IdeaManager` class maintains a dictionary of ideas with UUID keys and a separate dictionary for tag indexing. Ideas have sharing levels, importance ratings, tags, and context. The system provides extensive filtering and search capabilities.

**Data Models:**
- `Idea`: Core idea entity with id, idea text, share level, importance, tags, context, timestamps
- `IdeaUpdate`: Builder pattern class for constructing idea updates
- `IdeaStatistics`: Aggregated statistics with percentage calculations
- `ShareLevel`: Enum for sharing levels (Public, Team, Private)
- `IdeaImportance`: Enum for importance levels (Breakthrough=5, High=4, Medium=3, Low=2, VeryLow=1)

**Usage Example:**
```python
from todozi.idea import IdeaManager, ShareLevel, IdeaImportance

# Initialize manager
idea_manager = IdeaManager()

# Create idea
idea_id = idea_manager.create_idea(
    idea="Implement microservices architecture",
    share=ShareLevel.TEAM,
    importance=IdeaImportance.HIGH,
    tags=["architecture", "scalability"],
    context="System performance review"
)

# Search ideas
results = idea_manager.search_ideas("microservices")

# Filter by importance
breakthrough_ideas = idea_manager.get_breakthrough_ideas()
high_importance = idea_manager.get_ideas_by_importance(IdeaImportance.HIGH)

# Filter by share level
public_ideas = idea_manager.get_public_ideas()
team_ideas = idea_manager.get_team_ideas()
private_ideas = idea_manager.get_private_ideas()

# Filter by tag
architecture_ideas = idea_manager.get_ideas_by_tag("architecture")

# Update using builder pattern
from todozi.idea import IdeaUpdate
idea_manager.update_idea(
    idea_id,
    IdeaUpdate.new()
        .with_importance(IdeaImportance.BREAKTHROUGH)
        .with_tags(["architecture", "scalability", "breakthrough"])
        .with_context("Critical system optimization")
)

# Statistics
stats = idea_manager.get_idea_statistics()
print(f"Total ideas: {stats.total_ideas}")
print(f"Public: {stats.public_percentage()}%")
print(f"Breakthrough: {stats.breakthrough_percentage()}%")

# Tag statistics
tag_stats = idea_manager.get_tag_statistics()
for tag, count in tag_stats.items():
    print(f"{tag}: {count} ideas")

# Recent ideas
recent = idea_manager.get_recent_ideas(limit=5)
```

**Idea Format Parsing:**
Ideas can be parsed from text format:
`<idea>idea text; share level; importance; tags; context</idea>`

**Performance:** O(1) for CRUD operations using dictionaries, O(n×m) for search where n is ideas and m is average text length, O(n) for filtering operations.

**Design Patterns:** Builder pattern for IdeaUpdate, Repository pattern for IdeaManager, Factory pattern for parsing, Strategy pattern for filtering.

**Storage:** Ideas are stored in memory. Persistence should be handled by the storage layer integration.

---

### Models & Data Structures

The Models module provides comprehensive data structures and enums for the entire Todozi system with full type safety and Rust-inspired Result type pattern.

**Key Features:**
- Enum-like classes with string values and case-insensitive parsing
- Core data models with dataclasses and type hints
- Builder patterns for updates
- Collection classes for managing groups
- Comprehensive metadata support
- Result type pattern (Ok[T], Err[E]) for error handling without exceptions
- LowercaseEnumMixin for consistent enum parsing with alias support

**Architecture:**
The module defines enum classes for all system constants, core entity classes with full metadata, builder classes for updates, and collection classes. All classes follow consistent patterns for validation and serialization. The Result type pattern provides explicit error handling.

**Core Enums:**
- `Priority`: Low, Medium, High, Critical, Urgent
- `Status`: Todo, Pending, InProgress, Blocked, Review, Done, Completed, Cancelled, Deferred
- `AssigneeType`: Ai, Human, Collaborative, Agent
- `MemoryImportance`: Low, Medium, High, Critical
- `MemoryTerm`: Short, Long
- `MemoryType`: Standard, Secret, Human, Emotional
- `ShareLevel`: Public, Private, Team
- `IdeaImportance`: VeryLow, Low, Medium, High, Breakthrough
- `ItemStatus`: Active, Archived, Deleted
- `ErrorSeverity`: Low, Medium, High, Critical
- `ErrorCategory`: Network, Database, Authentication, Validation, Logic, UI, API, System, Other
- `TrainingDataType`: Instruction, Question, Conversation, Code, Documentation
- `ProjectStatus`: Active, Archived, Completed
- `AgentStatus`: Available, Busy, Inactive
- `AssignmentStatus`: Assigned, InProgress, Completed, Failed

**Core Data Models:**
- `Task`: Core task entity with action, priority, status, assignee, tags, dependencies, context, progress, embedding_vector
- `TaskUpdate`: Builder for task updates
- `Project`: Project entity with name, description, status, tasks list
- `Agent`: AI agent with capabilities, specializations, behaviors, constraints, metadata
- `Memory`: Memory entity with moment, meaning, reason, importance, term, type
- `Idea`: Idea entity with idea text, share level, importance, tags, context
- `ApiKey`: API key with user_id, public_key, private_key, active status
- `Error`: Error entity with title, description, severity, category, source, context, tags, resolved flag
- `TrainingData`: Training data with prompt, completion, data_type
- `Tag`: Tag entity with name and metadata
- `Reminder`: Reminder with message, priority, status, due_date
- `Summary`: Summary with content, priority, tags

**Result Type Pattern:**
```python
from typing import Union, Generic, TypeVar

T = TypeVar('T')
E = TypeVar('E')

class Ok(Generic[T]):
    def __init__(self, value: T)
    def is_ok(self) -> bool
    def is_err(self) -> bool
    def unwrap(self) -> T

class Err(Generic[E]):
    def __init__(self, error: E)
    def is_ok(self) -> bool
    def is_err(self) -> bool
    def unwrap(self) -> Any

Result = Union[Ok[T], Err[E]]
```

**Usage Example:**
```python
from todozi.models import (
    Task, TaskUpdate, Priority, Status, Assignee,
    Result, Ok, Err, TodoziError
)

# Create task with Result type
result = Task.new_full(
    user_id="user-123",
    action="Complete documentation",
    time="4 hours",
    priority=Priority.HIGH,
    parent_project="project-123",
    status=Status.TODO,
    assignee=Assignee.human(),
    tags=["documentation", "writing"],
    dependencies=[],
    context_notes="Write comprehensive API documentation",
    progress=None
)

if isinstance(result, Ok):
    task = result.value
    print(f"Created task: {task.id}")
else:
    error = result.error
    print(f"Error: {error}")

# Update using builder
update = TaskUpdate.new()
    .with_status(Status.IN_PROGRESS)
    .with_progress(50)
    .with_context("Started writing introduction")

update_result = task.update(update)
if isinstance(update_result, Err):
    print(f"Update failed: {update_result.error}")

# Enum parsing with aliases
status_result = Status.from_str("in-progress")  # Uses alias
if isinstance(status_result, Ok):
    status = status_result.value  # Status.IN_PROGRESS
```

**Type Safety:** Full type hints throughout. Dataclasses provide runtime validation. Enum parsing includes alias support for user convenience.

**Design Patterns:** Enum pattern for constants, Builder pattern for updates, Factory pattern for creation, Repository pattern for collections, Result type pattern for error handling.

**Validation:** All models include validation logic. Progress must be 0-100. Status and priority validated through enum parsing. Automatic timestamp updates on modifications.

---

### Todozi Core

The Todozi Core module provides structured parsing, execution, and management of various content types using a custom XML-like markup language with comprehensive enum support.

**Key Features:**
- Custom XML-like markup parsing for structured content
- Content type parsing (tasks, memories, ideas, errors, training data, feelings, summaries, reminders)
- Task execution for different assignee types (AI, Human, Collaborative, Agent)
- Workflow processing with chat message extraction
- Enum definitions for all system constants
- Shorthand tag transformation for convenience
- Pattern cache system for regex optimization
- Multi-level import fallback mechanism

**Architecture:**
The module provides parsing functions for each content type that extract structured data from formatted text. Execution functions handle different task types. Helper functions transform shorthand tags and validate enums. A PatternCache singleton optimizes regex compilation.

**Content Formats:**
- **Tasks**: `<todozi>action; time; priority; project; status; assignee</todozi>`
- **Memories**: `<memory>moment; meaning; reason; importance; term</memory>`
- **Secret Memories**: `<memory_secret>moment; meaning; reason; importance; term</memory_secret>`
- **Ideas**: `<idea>idea text; share level; importance; tags; context</idea>`
- **Errors**: `<error>title; description; severity; category; source; context; tags</error>`
- **Training Data**: `<train>prompt; completion; data_type</train>`
- **Feelings**: `<feel>emotion; intensity; context</feel>`
- **Summaries**: `<summary>content; priority; tags</summary>`
- **Reminders**: `<reminder>message; priority; due_date</reminder>`

**Shorthand Tags:**
- `<tz>` → `<todozi>`
- `<mm>` → `<memory>`
- `<id>` → `<idea>`
- `<ch>` → `<chunk>`
- `<fe>` → `<feel>`
- `<tn>` → `<train>`
- `<er>` → `<error>`
- `<sm>` → `<summary>`
- `<rd>` → `<reminder>`

**Usage Example:**
```python
from todozi.todozi import (
    parse_todozi_format, process_chat_message,
    execute_task, execute_ai_task, execute_human_task,
    transform_shorthand_tags, Priority, Status
)

# Parse formatted content
task_text = "<todozi>Complete docs; 4 hours; high; project-123; todo; human</todozi>"
task_result = parse_todozi_format(task_text)
if isinstance(task_result, Ok):
    task = task_result.value
    print(f"Parsed task: {task.action}")

# Process chat message
message = """
Meeting notes:
<todozi>Review budget; 1 week; high; finance; todo</todozi>
<memory>Budget meeting; Discussed Q4 allocation; Planning session; high; long</memory>
<idea>Automate budget reporting; team; 4</idea>
"""
extracted = process_chat_message(message)
print(f"Extracted {len(extracted.tasks)} tasks, {len(extracted.memories)} memories")

# Transform shorthand
shorthand = "<tz>Task; 1h; high; proj; todo</tz>"
full_tags = transform_shorthand_tags(shorthand)

# Execute tasks
execute_result = execute_task(task)
if isinstance(execute_result, Ok):
    print("Task executed successfully")

# AI task execution
ai_result = execute_ai_task(task)

# Human task execution
human_result = execute_human_task(task)
```

**Pattern Cache:**
The `PatternCache` singleton caches compiled regex patterns to avoid recompilation overhead:
```python
from todozi.todozi import PatternCache

# First call compiles and caches
pattern1 = PatternCache.get(r"<todozi>(.*?)</todozi>")

# Subsequent calls use cached pattern (90% faster)
pattern2 = PatternCache.get(r"<todozi>(.*?)</todozi>")
```

**Import Fallback:**
The module includes a multi-level import fallback system:
1. Attempt relative import
2. Attempt absolute package import
3. Attempt direct file import
4. Raise ImportError if all fail

**Error Handling:** All parsing functions return `Result[Content, TodoziError]` types. Invalid formats return descriptive errors without raising exceptions.

**Performance:** Pattern caching reduces regex compilation overhead by ~90%. Parsing is O(n) where n is content length. Shorthand transformation is O(1) per tag.

---

### Server

The Todozi Enhanced Server provides a RESTful API for managing tasks, agents, training data, memories, and more with integrated AI capabilities and multiple protocol support.

**Key Features:**
- RESTful API endpoints for all system resources
- HTTP, HTTPS, gRPC, and WebSocket support
- AI agent system integration
- Training data management endpoints
- Semantic search and similarity detection
- Time tracking and analytics
- Memory and idea management
- Queue-based workflow processing
- API key authentication middleware
- Comprehensive error handling
- Request/response logging

**Architecture:**
The server uses a controller pattern with separate controllers for each resource type. Business logic is encapsulated in service classes. Data access is abstracted through storage interfaces. The system supports multiple protocols and includes comprehensive error handling and logging.

**Key Endpoints:**
- `GET/POST /api/tasks` - Task CRUD operations
- `GET/POST /api/projects` - Project management
- `GET/POST /api/agents` - Agent management
- `GET/POST /api/training` - Training data management
- `GET/POST /api/memory` - Memory operations
- `GET/POST /api/ideas` - Idea management
- `GET /api/search` - Semantic search
- `GET/POST /api/queue` - Queue management
- `GET /api/stats` - System statistics
- `GET /api/health` - Health check

**Server Configuration:**
```python
from todozi.server import TodoziServer, ServerConfig

config = ServerConfig(
    host="0.0.0.0",
    port=8636,
    max_connections=100,
    enable_https=True,
    ssl_cert_path="/path/to/cert.pem",
    ssl_key_path="/path/to/key.pem"
)

server = TodoziServer(config)
await server.start()
```

**Usage Example:**
```python
from todozi.server import TodoziServer
import aiohttp

# Start server
server = TodoziServer(config)
await server.start()

# API calls
async with aiohttp.ClientSession() as session:
    # Create task
    async with session.post(
        'http://localhost:8636/api/tasks',
        headers={
            'Authorization': f'Bearer {api_key}',
            'Content-Type': 'application/json'
        },
        json={
            'action': 'Complete documentation',
            'priority': 'high',
            'project': 'project-123',
            'status': 'todo'
        }
    ) as response:
        task = await response.json()
    
    # Search
    async with session.get(
        'http://localhost:8636/api/search',
        params={'query': 'documentation', 'types': 'tasks,ideas'},
        headers={'Authorization': f'Bearer {api_key}'}
    ) as response:
        results = await response.json()
    
    # Health check
    async with session.get('http://localhost:8636/api/health') as response:
        health = await response.json()
```

**Authentication:**
API key authentication via Bearer tokens in Authorization header. Keys are validated against the API key collection. Admin operations require full key verification.

**Error Handling:**
All endpoints return structured error responses:
```json
{
  "error": {
    "type": "ValidationError",
    "message": "Invalid priority value",
    "code": 400
  }
}
```

**Performance:**
- Efficient request routing with async/await
- Connection pooling for database operations
- Caching strategies for frequently accessed data
- Batch operations for bulk data processing

**Security:**
- API key authentication middleware
- Input validation on all endpoints
- Rate limiting support
- CORS configuration
- Secure headers (HSTS, CSP)
- HTTPS support with SSL/TLS

**Logging:**
Comprehensive request/response logging with configurable log levels. Structured logging for analysis and debugging.

---

## Additional Modules

### Memory Management
Comprehensive memory storage and retrieval system with importance ratings, term durations (Short/Long), and emotional context tracking. Supports standard, secret, human, and emotional memory types with full CRUD operations.

### Reminder System
Scheduling and notification system for task reminders with priority levels (Low, Medium, High, Critical) and status tracking (Pending, Active, Completed, Cancelled). Supports due date management and reminder queries.

### Search System
Advanced search capabilities including semantic search using embeddings, keyword search, and hybrid search across all content types (tasks, memories, ideas, errors, training data). Supports filtering and ranking.

### Storage System
Abstraction layer for data persistence supporting file-based JSON storage and database backends. Provides efficient querying, atomic operations, and transaction support. Handles serialization/deserialization automatically.

### Tag Management
Tagging system for organizing and categorizing tasks, memories, ideas, and other content types. Supports tag statistics, filtering by tags, and tag-based search. Provides tag metadata and relationships.

### TUI (Text User Interface)
Terminal-based user interface for interactive task management with keyboard navigation, real-time updates, and color-coded displays. Supports task editing, filtering, and batch operations.

### Migration System
Data migration tools for upgrading between system versions and importing/exporting data. Supports schema migrations, data transformations, and backup/restore operations.

### Summary System
Content summarization capabilities for generating concise summaries of tasks, projects, and other content. Supports priority-based summarization and tag-based organization.

### Test Framework
Comprehensive testing utilities and frameworks for unit, integration, and performance testing. Includes test fixtures, mock objects, and assertion helpers.

### Type Definitions
TypeScript-style type definitions and runtime type checking for all system components. Provides type validation and conversion utilities.

### TLS/SSL Support
Secure communication layer with TLS/SSL certificate management, key generation, and secure connection handling for server operations.

### DNE (Do Not Execute)
System for marking and tracking content that should not be executed, with filtering and exclusion capabilities.

---

## Installation

```bash
# Clone the repository
git clone https://github.com/todozi/tdz_py.git
cd tdz_py

# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Install in development mode
pip install -e .

# Run tests
pytest

# Run with coverage
pytest --cov=todozi --cov-report=html
```

## Quick Start

```python
from todozi import TodoziHandler
from todozi.storage import Storage
from todozi.models import Priority, Status

# Initialize
storage = Storage()
handler = TodoziHandler(storage)

# Create a task
task_result = handler.add_task(
    action="Learn Todozi SDK",
    time="2 hours",
    priority=Priority.HIGH,
    project="learning",
    status=Status.TODO
)

if isinstance(task_result, Ok):
    task = task_result.value
    print(f"Created task: {task.id}")

# List tasks
tasks_result = handler.list_tasks(project="learning")
if isinstance(tasks_result, Ok):
    for task in tasks_result.value:
        print(f"- {task.action} ({task.status})")
```

## Type Safety

The Todozi Python SDK is built with full type hints and uses Python's typing system extensively:

- All functions include type annotations
- Dataclasses provide runtime type validation
- Result types make error handling explicit
- Enum classes ensure type-safe constants
- Generic types support flexible data structures

Example:
```python
from typing import Result
from todozi.models import Task, TodoziError

def get_task(task_id: str) -> Result[Task, TodoziError]:
    # Function signature clearly indicates return type
    ...
```

## Contributing

Contributions are welcome! Please read the contributing guidelines and submit pull requests for any improvements.

---

**Built with ❤️ by the Todozi Team**
