# Todozi Executor Technical Documentation

## Overview

The Todozi Executor is a Python translation of a Rust-based task management system with enhanced error handling, configuration management, and API integration capabilities. This implementation provides a robust asynchronous interface for task management, memory storage, idea tracking, and AI-powered search functionality.

## Architecture and Design

### System Architecture

```
TodoziExecutor
├── Configuration Layer (TodoziConfig)
├── API Client Layer (TodoziAPI)
├── Storage Layer (Storage)
├── Singleton System (TodoziSystem)
├── Action Handlers (execute_* functions)
└── Public Interface (Done class)
```

### Design Decisions

1. **Singleton Pattern**: The `TodoziSystem` implements a thread-safe singleton to ensure single initialization
2. **Async-First Design**: All operations are asynchronous to support high-concurrency use cases
3. **Granular Exception Hierarchy**: Mirroring Rust error variants for precise error handling
4. **Dependency Injection**: API clients can be injected for testing and flexibility
5. **Efficient Data Structures**: Using `deque` for queue operations and `set` for O(1) lookups

## Core Components

### Exception Hierarchy

#### `ExecutorError` (Base Exception)
- **Purpose**: Root exception for all executor operations
- **Inheritance**: Direct subclass of `Exception`
- **Usage**: Catch-all for executor-related failures

#### `ExecutionError`
- **Purpose**: General execution failures
- **Scenario**: Task creation, storage operations

#### `BashToolError`
- **Purpose**: External tool and API call failures
- **Scenario**: HTTP request failures, JSON parsing errors

#### `MissingParameterError`
- **Purpose**: Required parameter validation
- **Scenario**: Missing action or content parameters

#### `UnknownActionError`
- **Purpose**: Invalid action type handling
- **Scenario**: Unsupported operation requests

### Configuration Management

#### `TodoziConfig` Dataclass
```python
@dataclass
class TodoziConfig:
    api_key: str = field(default_factory=lambda: os.getenv("TDZ_API_KEY", ""))
    base_url: str = field(default_factory=lambda: os.getenv("TDZ_BASE_URL", "https://todozi.com"))
```

**Parameters:**
- `api_key`: Todozi API authentication key (environment: `TDZ_API_KEY`)
- `base_url`: API endpoint base URL (environment: `TDZ_BASE_URL`)

**Methods:**
- `validate() -> None`: Ensures required configuration is present

**Usage Example:**
```python
config = TodoziConfig()
config.validate()
```

### Data Models

#### `SearchResult`
```python
@dataclass
class SearchResult:
    content_id: str
    text_content: str
```
- **Purpose**: Unified search result format
- **Fields**: ID for reference, text content for display

#### `ChatContent`
```python
@dataclass
class ChatContent:
    tasks: List[Dict[str, Any]] = field(default_factory=list)
    memories: List[Dict[str, Any]] = field(default_factory=list)
    ideas: List[Dict[str, Any]] = field(default_factory=list)
```
- **Purpose**: Structured chat message extraction
- **Fields**: Separate lists for different content types

#### `ExecutionResult`
```python
@dataclass
class ExecutionResult:
    success: bool
    output: str
    error: Optional[str]
    metadata: Dict[str, Any] = field(default_factory=dict)
    tool_used: str = "todozi_simple"
    execution_type: str = "simple_interface"
```
- **Purpose**: Standardized operation result format
- **Metadata**: Action-specific contextual information

### API Client

#### `TodoziAPI` Class
```python
class TodoziAPI:
    def __init__(self, base_url: str, api_key: str) -> None
    async def post(self, endpoint: str, payload: Dict[str, Any]) -> Dict[str, Any]
```

**Constructor Parameters:**
- `base_url`: API endpoint base URL
- `api_key`: Authentication token

**Methods:**
- `post()`: Async HTTP POST request with validation

**Error Handling:**
- HTTP status code validation (200 required)
- JSON response parsing validation
- Response format validation (dict expected)

**Usage Example:**
```python
api = TodoziAPI("https://api.todozi.com", "key123")
response = await api.post("/extract", {"content": "task description"})
```

### Storage System

#### `Storage` Class
**Purpose**: In-memory data storage with efficient operations

**Data Structures:**
- `_tasks`, `_ideas`, `_memories`: Dict-based storage with O(1) access
- `_priority_queue`: `deque` for efficient append/pop operations
- `_active_tasks`: `set` for O(1) membership testing

**Key Methods:**

##### Task Operations
```python
def create_task(self, content: str, priority: str, assignee: str) -> str
def complete_task(self, task_id: str) -> bool
def start_task(self, task_id: str) -> bool
def get_backlog(self) -> List[Dict[str, Any]]
def get_active(self) -> List[Dict[str, Any]]
```

##### Search Operations
- `search_fast()`: Simple keyword matching
- `search_ai()`: Simulated semantic search (returns all content)
- `search_smart()`: Hybrid scoring (keyword + length proximity)
- `search_find()`: Advanced scoring (word matching + position bonus)

##### Memory and Idea Operations
```python
def create_memory(self, content: str, extra: str, note: str, importance: str) -> str
def important_memory(self, content: str, extra: str, note: str) -> str
def create_idea(self, content: str, extra: Optional[str], note: Optional[str]) -> str
def breakthrough_idea(self, content: str) -> str
```

### Singleton System

#### `TodoziSystem` Class
**Purpose**: Global system coordination and resource management

**Initialization Pattern:**
```python
@classmethod
async def get_instance(cls) -> "TodoziSystem"
async def initialize(self) -> None
```

**Components:**
- `storage`: Main data storage instance
- `embedding_service`: Optional AI embedding service

**Access Patterns:**
```python
# Global access functions
async def ensure_todozi_system() -> None
def get_storage() -> Storage
def get_embedding_service() -> Optional[Any]
```

## Public Interface

### `Done` Class
**Purpose**: User-friendly async interface mirroring Rust patterns

**Task Creation Methods:**
```python
@staticmethod
async def task(content: str) -> str                    # Normal priority
async def urgent(content: str) -> str                 # Urgent priority  
async def high(content: str) -> str                   # High priority
async def low(content: str) -> str                    # Low priority
async def ai(content: str) -> str                     # AI-assigned
async def human(content: str) -> str                  # Human-assigned
async def collab(content: str) -> str                 # Collaborative
```

**Search Methods:**
```python
async def tdz_find(query: str) -> List[SearchResult]  # Smart search
async def deep(query: str) -> List[SearchResult]      # AI search
async def fast(query: str) -> List[SearchResult]      # Fast search
async def smart(query: str) -> List[SearchResult]     # Intent search
```

**Content Management:**
```python
async def create_memory(content: str, extra: str, note: str) -> Dict[str, Any]
async def important(content: str, extra: str, note: str) -> str
async def create_idea(content: str, extra: Optional[str]) -> Dict[str, Any]
async def breakthrough(content: str) -> str
```

**Task Management:**
```python
async def complete(task_id: str) -> bool
async def begin(task_id: str) -> bool
async def quick() -> str                              # Statistics
async def list_queue_items() -> List[Dict[str, Any]]
async def backlog() -> List[Dict[str, Any]]
async def active() -> List[Dict[str, Any]]
```

## Action Implementations

### Core Execution Function

```python
async def execute_todozi_tool_delegated(
    params: Dict[str, Any], 
    api_client: Optional[TodoziAPI] = None
) -> ExecutionResult
```

**Parameters:**
- `params`: Action-specific parameters dictionary
- `api_client`: Optional injected API client for testing

**Supported Actions:**
- `task`, `urgent`, `high`, `low`: Priority-based task creation
- `ai`, `human`, `collab`: Assignee-based task creation  
- `find`, `ai_search`, `fast_search`, `smart_search`: Search operations
- `remember`, `important_memory`: Memory management
- `idea`, `breakthrough_idea`: Idea management
- `complete`, `start`: Task lifecycle
- `stats`, `queue`: System status
- `chat`: Content extraction
- `extract`, `expand`, `plan`, `strategy`: API operations

### Parameter Validation

**Utility Functions:**
```python
def _req_str(params: Dict[str, Any], key: str) -> str
def _opt_str(params: Dict[str, Any], key: str) -> Optional[str]
```

**Validation Rules:**
- Required parameters raise `MissingParameterError` if missing
- Type validation ensures string parameters
- Optional parameters return `None` if not present

## API Integration

### Content Extraction
```python
async def extract_content(
    message: Optional[str],
    context: Optional[str],
    output_format: str,
    _: bool,
    api_client: Optional[TodoziAPI] = None
) -> str
```

**Output Formats:**
- `json`: Structured JSON response
- `csv`: Comma-separated values
- `markdown`: Human-readable markdown

### Strategy Generation
```python
async def strategy_content(
    message: Optional[str],
    context: Optional[str],
    output_format: str,
    _: bool,
    api_client: Optional[TodoziAPI] = None
) -> str
```

## Performance Considerations

### Memory Efficiency
- **Data Structures**: Optimized collections (`deque`, `set`) for specific operations
- **Lazy Initialization**: Embedding service initialized only when needed
- **In-Memory Storage**: Fast access but limited to process memory

### Search Algorithms
- **Fast Search**: O(n) keyword matching for small datasets
- **Smart Search**: O(n log n) scoring and sorting
- **AI Search**: Simulated semantic search (production would use embeddings)

### Concurrency
- **Async Operations**: Non-blocking HTTP requests and storage operations
- **Thread Safety**: Singleton initialization with async lock
- **Resource Management**: `aiohttp` context managers for connection pooling

## Error Handling and Edge Cases

### Common Failure Scenarios

1. **Configuration Errors**
   - Missing API key environment variable
   - Invalid API endpoint URL

2. **API Communication Errors**
   - Network timeouts and connection failures
   - HTTP status code errors (4xx, 5xx)
   - Malformed JSON responses

3. **Data Validation Errors**
   - Missing required parameters
   - Invalid parameter types
   - Non-existent task IDs for operations

4. **System Initialization Errors**
   - Embedding service dependency failures
   - Storage initialization problems

### Recovery Strategies
- **Graceful Degradation**: Operations fall back to simple implementations
- **Error Propagation**: Clear error messages with contextual metadata
- **Resource Cleanup**: Proper context manager usage for HTTP sessions

## Dependencies and Requirements

### Core Dependencies
```python
# Standard Library
import asyncio        # Async operations
import json           # JSON serialization  
import logging        # Structured logging
import os             # Environment variables
import time           # Timestamp generation

# Third-party
import aiohttp        # Async HTTP client
```

### Optional Dependencies
```python
# AI/Embedding Services (if available)
from todozi.emb import TodoziEmbeddingService, TodoziEmbeddingConfig
from todozi.extract import TodoziConfig, get_api_client, parse_extract_response
```

### Environment Variables
- `TDZ_API_KEY`: Required API authentication key
- `TDZ_BASE_URL`: Optional API base URL (default: https://todozi.com)

## Testing and Examples

### CLI Demonstration
```python
if __name__ == "__main__":
    async def _demo():
        res = await execute_todozi_tool_delegated(
            {"action": "task", "content": "Write documentation"}
        )
        print(res.output)
        print(res.metadata)

    asyncio.run(_demo())
```

### Testing Patterns
- **Dependency Injection**: Mock API clients for unit testing
- **Parameter Validation**: Test boundary conditions and error cases
- **Async Testing**: Use `pytest-asyncio` for async test support

## Limitations and Constraints

### Technical Limitations
1. **In-Memory Storage**: Data persistence limited to process lifetime
2. **Scalability**: Linear search algorithms unsuitable for large datasets
3. **AI Integration**: Embedding service requires external dependencies

### Design Constraints
1. **Rust Compatibility**: API designed to mirror Rust counterpart behavior
2. **Async-Only**: Synchronous operations not supported
3. **Singleton Pattern**: Global state management may complicate testing

### Performance Constraints
- Search operations scale linearly with data size
- No built-in persistence or database integration
- Embedding service optional and external

## Migration and Compatibility

### Rust to Python Translation
- Exception hierarchy mirrors Rust variants
- API signatures maintain compatibility
- Data structures optimized for Python ecosystem

### Backward Compatibility
- Environment variable configuration maintained
- API response formats consistent
- Error handling patterns preserved

This documentation provides comprehensive coverage of the Todozi Executor system architecture, implementation details, and operational characteristics. The system represents a robust translation of Rust functionality into Python with enhanced error handling, configuration management, and testing capabilities.