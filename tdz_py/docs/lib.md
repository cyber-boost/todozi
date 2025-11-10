# Todozi Technical Documentation

## 1. Overview

Todozi is a comprehensive task management and productivity system implemented in Python. It provides a robust framework for managing tasks, projects, ideas, memories, and AI-assisted workflows. The system is designed with production-readiness, async-first operations, and dependency injection patterns.

### Key Features
- **Task Management**: Complete CRUD operations with filtering and search capabilities
- **Project Organization**: Hierarchical task organization with project-based storage
- **AI Integration**: Embedding services for semantic search and task clustering
- **Multiple Content Types**: Support for tasks, ideas, memories, queue items, and reminders
- **Async Storage**: File-based storage with async I/O operations
- **Context Management**: Thread-safe context management using `contextvars`

## 2. Architecture

### 2.1 Core Components

#### TodoziContext
Central context object that manages storage and configuration dependencies.

```python
class TodoziContext:
    def __init__(self, storage: IndexedStorage, config: ValidatedConfig):
        self._storage = storage
        self._config = config
```

**Properties:**
- `storage`: IndexedStorage instance for data persistence
- `config`: ValidatedConfig instance for application settings

#### IndexedStorage
Async file-based storage system with task indexing for efficient operations.

```python
class IndexedStorage(CachedStorage):
    def __init__(self, base_dir: str):
        super().__init__(base_dir)
        self._task_index: Dict[str, str] = {}  # task_id -> project_path
        self._rebuild_index()
```

**Key Features:**
- Task indexing for O(1) task lookups
- Project-based file organization
- Async file operations with `aiofiles` fallback to stdlib
- Cached project listings with TTL invalidation

### 2.2 Data Models

#### Task Model
Primary task entity with comprehensive metadata:

```python
class Task:
    def __init__(
        self,
        id: str,
        user_id: str,
        action: str,
        time: str,
        priority: Priority,
        parent_project: str,
        status: Status,
        assignee: Optional[Assignee],
        tags: List[str],
        dependencies: List[str],
        context_notes: Optional[str],
        progress: Optional[int],
        created_at: datetime.datetime,
        updated_at: datetime.datetime,
        embedding_vector: Optional[List[float]],
    ):
```

**Supported Models:**
- `Idea`: Creative concepts and brainstorming items
- `Memory`: Long-term memory storage with semantic meaning
- `QueueItem`: Work queue management with status tracking
- `Reminder`: Time-based alert system
- `Tag`: Categorization and metadata system

### 2.3 Enum Types

Strictly typed enumerations for validation:

```python
class Status(str):
    Todo = "Todo"
    InProgress = "InProgress"
    Done = "Done"
    Blocked = "Blocked"
    
    @staticmethod
    def safe_parse(value: Optional[str]) -> Optional["Status"]:
        # Safe parsing with validation
```

**Available Enums:**
- `Priority`: Critical, Urgent, High, Medium, Low
- `AssigneeType`: Human, AI, Collaborative
- `ContentType`: Task, Idea, Memory, Note, Code
- Multiple specialized enums for different domains

## 3. Core Services

### 3.1 TodoziEmbeddingService

AI-powered embedding and similarity search service:

```python
class TodoziEmbeddingService:
    def __init__(self, config: TodoziEmbeddingConfig):
        self.config = config
        self.initialized = False

    async def generate_embedding(self, text: str) -> List[float]:
        # Simple heuristic-based embedding generation
        dim = self.config.dimension
        vec = [0.0] * dim
        for i, ch in enumerate(text):
            idx = (ord(ch) + i) % dim
            vec[idx] += 1.0
        norm = (sum(v * v for v in vec) ** 0.5) or 1.0
        return [v / norm for v in vec]
```

**Key Methods:**
- `semantic_search()`: Find similar content using semantic analysis
- `hybrid_search()`: Combine semantic and keyword search
- `cluster_content()`: Group related tasks using clustering algorithms
- `find_similar_tasks()`: Task-specific similarity matching

### 3.2 ServiceFactory

Dependency injection factory for service creation:

```python
class ServiceFactory:
    def __init__(self, context: TodoziContext):
        self._context = context

    def create_embedding_service(self, config: Optional[TodoziEmbeddingConfig] = None) -> TodoziEmbeddingService:
        cfg = config or TodoziEmbeddingConfig()
        return TodoziEmbeddingService(cfg)
```

## 4. Storage System

### 4.1 File Organization

```
~/.todozi/
├── tdz.hlx                    # Main configuration
├── registration.json          # User registration data
├── projects/                  # Project-based task storage
│   ├── project1/
│   │   ├── project.json       # Project metadata
│   │   ├── task1.json         # Individual task files
│   │   └── task2.json
│   └── project2/
├── data/                      # Additional data types
│   ├── ideas/
│   ├── memories/
│   ├── agents/
│   ├── chunks/
│   ├── errors/
│   ├── feelings/
│   └── queue_collection.json
└── backups/                   # Automated backups
```

### 4.2 AsyncFile Wrapper

Cross-platform async file operations with graceful fallback:

```python
class AsyncFile:
    def __init__(self, path: str, mode: str, encoding: str = "utf-8"):
        self.path = path
        self.mode = mode
        self.encoding = encoding
        self._file = None

    async def __aenter__(self):
        if aiofiles is not None:
            # Use aiofiles if available
            self._file = aiofiles.open(self.path, self.mode, encoding=self.encoding)
            return await self._file.__aenter__()
        else:
            # Fallback to threadpool executor
            loop = asyncio.get_event_loop()
            self._file = await loop.run_in_executor(None, open, self.path, self.mode, self.encoding)
            return self._file
```

## 5. API Surface

### 5.1 Public Interface

The `Done` class provides the main public API:

```python
class Done:
    @staticmethod
    async def create_task(
        action: str,
        priority: Optional[Priority] = None,
        project: Optional[str] = None,
        time: Optional[str] = None,
        context: Optional[str] = None,
    ) -> Task:
        # Creates and persists a new task
```

**Common Operations:**
- Task management: `create_task()`, `update_task()`, `delete_task()`
- Search: `find_tasks()`, `ai_find()`, `smart_search()`
- Project operations: `create_project()`, `tasks()`
- Analytics: `quick()`, `detailed()`, `stats()`

### 5.2 Builder Patterns

Fluent interface builders for complex object creation:

```python
class TaskBuilder:
    def __init__(self, storage: IndexedStorage, default_project: str):
        self._storage = storage
        self._action: Optional[str] = None
        self._priority: Optional[Priority] = None
        # ... other fields

    def with_action(self, action: str) -> "TaskBuilder":
        self._action = action
        return self

    def with_priority(self, priority: Priority) -> "TaskBuilder":
        self._priority = priority
        return self

    async def build(self) -> Task:
        # Validation and persistence
        if not self._action:
            raise TodoziError.validation("Task action is required")
        # ... build and save task
```

## 6. Configuration System

### 6.1 ValidatedConfig

Type-safe configuration management with validation:

```python
class ValidatedConfig:
    _ALLOWED_INTERVALS = {"daily", "weekly", "monthly"}
    _ALLOWED_TZ = {"UTC"}
    _ALLOWED_DATE_FORMAT = {"%Y-%m-%d %H:%M:%S"}

    def __init__(self):
        self._version = "1.2.0"
        self._default_project = "general"
        self._auto_backup = True
        # ... other defaults

    @property
    def backup_interval(self) -> str:
        return self._backup_interval

    @backup_interval.setter
    def backup_interval(self, value: str) -> None:
        if value not in self._ALLOWED_INTERVALS:
            raise ValueError(f"Invalid interval: {value}. Allowed: {self._ALLOWED_INTERVALS}")
        self._backup_interval = value
```

## 7. Error Handling

### 7.1 TodoziError Hierarchy

Structured error handling with context:

```python
class TodoziError(Exception):
    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None):
        super().__init__(message)
        self.details = details or {}

    @staticmethod
    def config(message: str) -> "TodoziError":
        return TodoziError(f"Config error: {message}")

    @staticmethod
    def validation(message: str) -> "TodoziError":
        return TodoziError(f"Validation error: {message}")

    @staticmethod
    def task_not_found(task_id: str) -> "TodoziError":
        return TodoziError(f"Task not found: {task_id}")
```

## 8. Performance Considerations

### 8.1 Caching Strategy

- **Project List Cache**: 30-second TTL with manual invalidation
- **Task Index**: Built at startup and updated on mutations
- **Embedding Cache**: Vector caching for repeated queries

### 8.2 Async Operations

- All file I/O operations are async with proper context management
- Embedding generation uses efficient heuristic algorithms
- Search operations optimized with indexing and batch processing

## 9. Dependencies and Requirements

### 9.1 Core Dependencies
- **Python 3.7+**: Async/await support required
- **aiofiles**: Optional for enhanced async file operations
- **Standard Library**: Comprehensive fallback implementation

### 9.2 Optional Dependencies
- External AI services for enhanced embedding capabilities
- Additional storage backends (database, cloud storage)

## 10. Usage Examples

### 10.1 Basic Task Management

```python
import asyncio
from todozi import Done, Priority, Status

async def main():
    # Initialize the system
    await Done.init()
    
    # Create a task
    task = await Done.create_task(
        action="Complete documentation",
        priority=Priority.High,
        project="documentation",
        context="Technical documentation for Todozi system"
    )
    
    # Update task status
    await Done.update_task_status(task.id, Status.InProgress)
    
    # Search for tasks
    tasks = await Done.find_tasks("documentation")
    
    # Complete the task
    await Done.complete_task(task.id)

asyncio.run(main())
```

### 10.2 Advanced Search Operations

```python
async def advanced_workflow():
    # Semantic search with AI
    similar_tasks = await Done.ai_find("refactor codebase")
    
    # Hybrid search combining keywords and semantics
    from todozi import ContentType
    results = await Done.hybrid_search(
        query="database optimization",
        keywords=["performance", "index"],
        content_types=[ContentType.Task, ContentType.Idea],
        semantic_weight=0.7,
        limit=10
    )
    
    # Cluster related content
    clusters = await Done.cluster_content()
```

### 10.3 Project Management

```python
async def project_workflow():
    # Create a new project
    await Done.create_project("mobile-app", "Mobile application development")
    
    # Add tasks to specific project
    task = await Done.create_task(
        action="Design mobile UI",
        project="mobile-app"
    )
    
    # Get all tasks in project
    project_tasks = await Done.tasks("mobile-app")
```

## 11. Technical Constraints

### 11.1 Limitations
- **File-based Storage**: Not designed for high-concurrency environments
- **Memory Usage**: Embedding vectors can consume significant memory
- **Scalability**: Best suited for single-user or small team usage
- **External Dependencies**: Some features require external AI services

### 11.2 Compatibility
- **Platform**: Cross-platform (Windows, macOS, Linux)
- **Python Versions**: 3.7+ required for async features
- **Storage**: Requires filesystem write permissions

## 12. Extension Points

### 12.1 Custom Storage Backends
Override `IndexedStorage` for alternative persistence:
```python
class DatabaseStorage(IndexedStorage):
    async def add_task_to_project(self, task: Task) -> None:
        # Custom database implementation
```

### 12.2 Custom Embedding Services
Implement alternative embedding algorithms:
```python
class CustomEmbeddingService(TodoziEmbeddingService):
    async def generate_embedding(self, text: str) -> List[float]:
        # Custom embedding logic
```

This documentation provides comprehensive coverage of the Todozi system architecture, API surface, and implementation details suitable for developers extending or integrating with the system.