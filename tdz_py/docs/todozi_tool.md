# Todozi Tools Module Technical Documentation

## Overview

The Todozi Tools module is a comprehensive Python implementation of a Rust-based task management system, providing asynchronous, validated tools for AI-agent task orchestration, memory management, and code organization. This module implements a concurrency-safe, dependency-injected architecture with robust error handling and resource management.

## Module Structure

### Core Enumerations

#### Priority Levels
```python
class Priority(Enum):
    Low = "low"          # Standard priority tasks
    Medium = "medium"    # Default priority level
    High = "high"        # Elevated importance
    Critical = "critical" # System-critical operations
    Urgent = "urgent"    # Time-sensitive tasks
```

#### Task Status
```python
class Status(Enum):
    Todo = "todo"           # Not yet started
    InProgress = "in_progress" # Currently being worked on
    Blocked = "blocked"     # Waiting on dependencies
    Review = "review"       # Under review/QA
    Done = "done"           # Completed
```

#### Memory Management
```python
class MemoryImportance(Enum):
    Low = "low"        # Casual memory
    Medium = "medium"  # Standard importance
    High = "high"      # Important to remember
    Critical = "critical" # Critical system knowledge
```

### Data Models

#### Task Model
```python
@dataclass
class Task:
    id: str                    # UUID identifier
    user_id: str              # Owner identifier
    action: str               # Task description (1-500 chars)
    time: str                 # Time estimate ("ASAP", "2 hours")
    priority: Priority        # Priority level
    parent_project: str       # Project association
    status: Status           # Current state
    assignee: Optional[Priority] # Assignment type (AI/Human/Collaborative)
    tags: List[str]           # Categorization tags
    dependencies: List[str]   # Task dependencies
    context_notes: Optional[str] # Additional context
    progress: Optional[int]   # Completion percentage (0-100)
    embedding_vector: Optional[List[float]] # AI vector for semantic search
    created_at: datetime      # Creation timestamp
    updated_at: datetime      # Last modification timestamp
```

#### Memory Model
```python
@dataclass
class Memory:
    id: str
    user_id: str
    project_id: Optional[str]
    status: ItemStatus
    moment: str              # What happened
    meaning: str             # Significance/meaning
    reason: str              # Why it's remembered
    importance: MemoryImportance
    term: MemoryTerm         # Short/Long term
    memory_type: MemoryType  # Standard/Emotional
    tags: List[str]
    created_at: datetime
    updated_at: datetime
```

## Core Architecture Components

### Storage System

#### In-Memory Storage
```python
@dataclass
class Storage:
    tasks: Dict[str, Task] = field(default_factory=dict)
    memories: Dict[str, Memory] = field(default_factory=dict)
    ideas: Dict[str, Idea] = field(default_factory=dict)
    errors: Dict[str, Error] = field(default_factory=dict)
    code_chunks: Dict[str, CodeChunk] = field(default_factory=dict)
```

**Key Methods:**
- `add_task_to_project()`: Atomic task creation
- `list_tasks_across_projects()`: Filtered task retrieval
- `update_task()`: Partial task updates with automatic timestamp

#### Concurrency Safety with StorageProxy
```python
class StorageProxy:
    def __init__(self, storage: Storage):
        self._storage = storage
        self._lock = asyncio.Lock()  # Ensures thread-safe operations

    async def execute(self, operation: Callable[[Storage], Awaitable[Any]]) -> Any:
        async with self._lock:  # Exclusive access to storage
            return await operation(self._storage)
```

### Resource Management

#### ResourceLock Enumeration
```python
class ResourceLock(Enum):
    FilesystemRead = "FilesystemRead"
    FilesystemWrite = "FilesystemWrite"
    Network = "Network"
    Memory = "Memory"
```

#### ResourceManager Implementation
```python
class ResourceManager:
    def __init__(self) -> None:
        self._locks: Dict[ResourceLock, asyncio.Lock] = {}

    async def acquire(self, resource_locks: List[ResourceLock]) -> None:
        for lock_type in resource_locks:
            if lock_type not in self._locks:
                self._locks[lock_type] = asyncio.Lock()
            await self._locks[lock_type].acquire()

    def release(self, resource_locks: List[ResourceLock]) -> None:
        for lock_type in resource_locks:
            if lock_type in self._locks:
                self._locks[lock_type].release()
```

## Tool Framework

### Tool Definition Pattern

```python
@dataclass
class ToolDefinition:
    name: str                    # Tool identifier
    description: str            # Human-readable description
    parameters: List[ToolParameter] # Input specifications
    category: str               # Functional grouping
    resource_locks: List[ResourceLock] # Required resources
```

### ToolResult Standardization

```python
class ToolResult:
    def __init__(self, success: bool, message: str, code: int, 
                 data: Optional[Dict[str, Any]] = None):
        self.success = success    # Operation status
        self.message = str        # Human-readable result
        self.code = int          # Status code (100=success, 400=error)
        self.data = data or {}   # Additional payload

    @staticmethod
    def success(message: str, code: int, data: Optional[Dict[str, Any]] = None):
        return ToolResult(True, message, code, data)

    @staticmethod
    def error(message: str, code: int):
        return ToolResult(False, message, code, {})
```

### Parameter Validation Decorator

```python
def validate_params(*validations: Tuple[str, type, bool]):
    def decorator(method: Callable[..., Awaitable[ToolResult]]):
        async def wrapper(self: "BaseTool", kwargs: Dict[str, Any]) -> ToolResult:
            for param_name, param_type, required in validations:
                value = kwargs.get(param_name)
                if required and (value is None or not isinstance(value, param_type)):
                    return ToolResult.error(f"Missing or invalid '{param_name}' parameter", 400)
            return await method(self, kwargs)
        return wrapper
    return decorator
```

## Tool Implementations

### CreateTaskTool

**Purpose:** Create new tasks with automatic AI/human assignment

```python
class CreateTaskTool(BaseTool):
    def definition(self) -> ToolDefinition:
        return ToolDefinition(
            name="create_task",
            description="Create a new task in the Todozi system with automatic AI assignment and queue management",
            parameters=[
                create_tool_parameter("action", str, "Task description/action to perform", True),
                create_tool_parameter("time", str, "Time estimate (e.g., '2 hours', '1 day')", False),
                create_tool_parameter("priority", str, "Priority level", False),
                # ... additional parameters
            ],
            category="Task Management",
            resource_locks=[ResourceLock.FilesystemWrite],
        )

    @validate_params(("action", str, True))
    async def execute(self, kwargs: Dict[str, Any]) -> ToolResult:
        # Implementation with resource locking and validation
        return await self._with_resources(run)
```

**Usage Example:**
```python
tool = CreateTaskTool(storage_proxy)
result = await tool.execute({
    "action": "Write API documentation",
    "priority": "high", 
    "assignee": "human",
    "tags": "documentation,api"
})
```

### Dictionary Dispatch Pattern

Mimics Rust match patterns for strategy selection:

```python
def get_task_creator(done: Done, assignee: str, priority: str) -> Callable[[str], Awaitable[str]]:
    strategies = {
        ("ai", "_"): done.ai,                    # AI-assigned tasks
        ("human", "_"): done.human,              # Human-assigned tasks
        ("collaborative", "_"): done.collab,     # Collaborative tasks
        ("_", "urgent"): done.urgent,           # Urgent priority
        ("_", "critical"): done.urgent,         # Critical = urgent
        ("_", "high"): done.high,               # High priority
        ("_", "low"): done.low,                 # Low priority
    }
    # Pattern matching logic
    for (a, p), creator in strategies.items():
        if (a == assignee or a == "_") and (p == priority or p == "_"):
            return creator
    return lambda action: done.create_task(action)  # Default strategy
```

## Search Capabilities

### Semantic Search Integration

```python
class TodoziEmbeddingService:
    def __init__(self, config: TodoziEmbeddingConfig):
        self.config = config
        self._cache: Dict[str, List[Dict[str, Any]]] = {}  # Result caching

    async def search(self, query: str, limit: int = 10) -> List[Dict[str, Any]]:
        # Returns semantic matches with similarity scores
        return [
            {
                "content_id": str(uuid.uuid4()),
                "text_content": f"Embedding match for '{query}'",
                "similarity_score": 0.91
            }
        ][:limit]
```

### Multi-Modal Search

**UnifiedSearchTool** provides cross-data-type searching:
- Tasks, memories, ideas, errors
- Keyword and semantic search modes
- Configurable data type filtering

## Chat Processing Engine

### Intelligent Message Parsing

```python
async def chat(self, message: str) -> Dict[str, Any]:
    # Pattern-based content extraction
    patterns = {
        "TODO:": self._create_task,      # Task extraction
        "REMEMBER:": self._create_memory, # Memory creation  
        "IDEA:": self._create_idea,      # Idea capture
        "ERROR:": self._create_error     # Error tracking
    }
    
    return {
        "tasks": extracted_tasks,
        "memories": extracted_memories,
        "ideas": extracted_ideas,
        "errors": extracted_errors
    }
```

## Error Handling System

### Comprehensive Error Model

```python
@dataclass
class Error:
    id: str
    title: str                    # Error summary
    description: str              # Detailed explanation
    severity: ErrorSeverity       # Impact level
    category: ErrorCategory       # Error type
    source: str                   # Origin component
    context: Optional[str]        # Additional context
    tags: List[str]               # Categorization
    resolved: bool = False        # Resolution status
    resolution: Optional[str]     # Solution description
    created_at: datetime          # Occurrence timestamp
    updated_at: datetime          # Last update
    resolved_at: Optional[datetime] # Resolution timestamp
```

## Code Chunking System

### Hierarchical Code Organization

```python
@dataclass
class CodeChunk:
    chunk_id: str                 # Unique identifier
    status: ChunkStatus           # Development state
    dependencies: List[str]       # Required chunks
    code: str = ""                # Implementation code
    tests: str = ""               # Associated tests
    validated: bool = False       # Quality assurance status
    level: ChunkingLevel = ChunkingLevel.Block # Granularity level
    estimated_tokens: int = 0     # AI context size estimate
    created_at: datetime          # Creation timestamp
    updated_at: datetime          # Modification timestamp
```

**Chunking Levels:**
- Project → Module → Class → Method → Block

## Performance Considerations

### Concurrency Design
- **StorageProxy** ensures thread-safe operations with async locks
- **ResourceManager** prevents resource contention
- **Async/await** pattern for non-blocking I/O operations

### Memory Management
- In-memory storage for development/testing
- Configurable caching strategies in embedding service
- Lazy loading of large data sets

### Search Optimization
- Keyword indexing for fast text search
- Semantic caching with TTL configuration
- Configurable result limits and thresholds

## Error Handling and Edge Cases

### Validation Coverage
- Parameter type and length validation
- Required field enforcement
- Enum value validation with graceful fallbacks

### Exception Scenarios
- Missing API keys with clear error messages
- Network timeout handling (120s default)
- Storage operation failures with rollback semantics
- Invalid state transitions with descriptive errors

## Dependencies and Requirements

### Core Dependencies
```python
# Required packages:
- asyncio: Async programming
- uuid: Unique identifier generation  
- datetime: Timestamp management
- pathlib: Filesystem operations
- aiohttp: Async HTTP client (for API calls)
- re: Regular expression parsing
```

### Environment Variables
```bash
TDZ_API_KEY="your_api_key"          # Authentication
TDZ_BASE_URL="https://todozi.com"   # API endpoint
```

## Usage Examples

### System Initialization
```python
async def initialize_system():
    storage_proxy, embedding = await initialize_grok_level_todozi_system(
        enable_embeddings=True
    )
    tools = create_todozi_tools_with_embedding(storage_proxy, embedding)
    return tools
```

### Tool Execution Pattern
```python
# Find and execute specific tool
search_tool = next(t for t in tools if isinstance(t, SearchTasksTool))
result = await search_tool.execute({
    "query": "documentation", 
    "semantic": True,
    "limit": 5
})

if result.success:
    print(f"Found {len(result.data)} results")
else:
    print(f"Error: {result.message}")
```

## Limitations and Constraints

### Current Limitations
- In-memory storage (not persistent)
- Simplified error resolution workflow
- Basic AI integration (stubbed embeddings)
- Limited distributed locking capabilities

### Design Constraints
- Maximum field lengths enforced for data integrity
- Async-first architecture requires event loop
- Python 3.8+ compatibility required
- No built-in persistence layer (filesystem/DB)

## Future Enhancements

### Planned Improvements
- Persistent storage backends (SQLite, PostgreSQL)
- Advanced AI integration with real embeddings
- Distributed locking for multi-process environments
- Comprehensive testing suite with mock services
- Performance monitoring and metrics collection

This documentation provides comprehensive coverage of the Todozi Tools module architecture, implementation details, and usage patterns suitable for developers integrating with or extending the system.