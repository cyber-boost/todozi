# Todozi Technical Documentation

## Overview

The Todozi system is a comprehensive task management and AI collaboration framework designed to support complex project workflows with integrated AI capabilities. This module provides core data models, enumerations, and utility functions that form the foundation of the Todozi ecosystem.

## Architecture and Design

### Result Type Pattern
The system implements a Rust-inspired `Result<T, E>` pattern for error handling:

```python
# Generic Result type mirroring Rust's approach
Result = Union[Ok[T], Err[E]]

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
```

**Design Decision**: This pattern provides explicit error handling without exceptions, making error paths more predictable and composable.

### Enum System with Aliasing
The `LowercaseEnumMixin` provides consistent string parsing with alias support:

```python
class LowercaseEnumMixin:
    ALIASES: Dict[str, Any] = {}
    
    @classmethod
    def from_str(cls: Type[Any], s: str) -> Result[Any, TodoziError]
    @classmethod
    def from_str_mapped(cls: Type[Any], s: str, mapping: Dict[str, Any]) -> Result[Any, TodoziError]
```

**Benefits**:
- Consistent case-insensitive parsing
- Support for multiple string representations (aliases)
- Type-safe error handling with specific error types

## Core Components

### Error Handling System

#### TodoziError Class
```python
class TodoziError(Exception):
    def __init__(self, kind: str, message: str, *, priority: Optional[str] = None, 
                 status: Optional[str] = None, progress: Optional[int] = None)
    
    # Static factory methods
    @staticmethod
    def invalid_priority(priority: str) -> "TodoziError"
    @staticmethod
    def invalid_status(status: str) -> "TodoziError"
    @staticmethod
    def invalid_progress(progress: int) -> "TodoziError"
    @staticmethod
    def validation_error(message: str) -> "TodoziError"
```

**Features**:
- Structured error information with kind categorization
- Contextual data for debugging
- Factory methods for common error types

### Enumeration Types

#### Priority Enum
```python
class Priority(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"
    URGENT = "urgent"
```

**Usage**:
```python
result = Priority.from_str("high")
if isinstance(result, Ok):
    priority = result.value
```

#### Status Enum with Aliases
```python
class Status(LowercaseEnumMixin, str, Enum):
    TODO = "todo"
    PENDING = "pending"
    IN_PROGRESS = "in_progress"
    # ... additional values
    
    ALIASES = {
        "pending": TODO,  # alias to TODO
        "in-progress": IN_PROGRESS,  # alias
        "completed": DONE,  # alias
    }
```

**Special Features**:
- Multiple string representations supported
- Comprehensive alias mapping for user convenience

### Core Data Models

#### Task Model
The central task management entity with comprehensive lifecycle support:

```python
class Task(BaseT):
    id: str
    user_id: str
    action: str
    time: str
    priority: Priority
    parent_project: str
    status: Status
    assignee: Optional[Assignee]
    tags: List[str]
    dependencies: List[str]
    context_notes: Optional[str]
    progress: Optional[int]
    embedding_vector: Optional[List[float]]
    created_at: datetime
    updated_at: datetime
```

**Key Methods**:
```python
@staticmethod
def new(user_id: str, action: str, time: str, priority: Priority, 
        parent_project: str, status: Status) -> "Task"

@staticmethod
def new_full(user_id: str, action: str, time: str, priority: Priority,
             parent_project: str, status: Status, assignee: Optional[Assignee],
             tags: List[str], dependencies: List[str], context_notes: Optional[str],
             progress: Optional[int]) -> Result["Task", TodoziError]

def update(self, updates: "TaskUpdate") -> Result[None, TodoziError]
def complete(self) -> None
def is_completed(self) -> bool
def is_active(self) -> bool
```

**Validation Rules**:
- Progress must be between 0-100 (inclusive)
- Status and priority validated through enum parsing
- Automatic timestamp updates on modifications

#### Assignee System
Flexible assignment system supporting AI, human, and collaborative work:

```python
@dataclass(frozen=True)
class Assignee:
    kind: str  # "ai", "human", "collaborative", "agent"
    name: Optional[str]  # required for "agent" kind
    
    @staticmethod
    def from_str(s: str) -> Result["Assignee", TodoziError]
```

**Supported Formats**:
- `"ai"` → AI assignment
- `"human"` → Human assignment  
- `"collaborative"` → Collaborative work
- `"agent:name"` → Specific agent assignment
- Plain string → Treated as agent name

#### Project Management
```python
class Project(BaseT):
    name: str
    description: Optional[str]
    status: ProjectStatus
    tasks: List[str]  # Task IDs
    
    def add_task(self, task_id: str) -> None
    def remove_task(self, task_id: str) -> None
    def archive(self) -> None
    def complete(self) -> None
```

### AI Agent System

#### Agent Model
Comprehensive AI agent configuration with tooling and constraints:

```python
class Agent(BaseT):
    id: str
    name: str
    description: str
    model: ModelConfig
    system_prompt: str
    capabilities: List[str]
    specializations: List[str]
    tools: List[AgentTool]
    behaviors: AgentBehaviors
    constraints: AgentConstraints
    metadata: AgentMetadata
```

**Specialized Agent Creation**:
```python
@staticmethod
def create_coder() -> "Agent"
```

**Agent Capabilities**:
- Tool management and validation
- Status tracking (active, inactive, busy, available)
- Prompt templating with variable substitution
- Capability and specialization checking

### Queue System
Advanced queue management with session tracking:

```python
class QueueCollection(BaseT):
    items: Dict[str, QueueItem]
    sessions: Dict[str, QueueSession]
    
    def start_session(self, queue_item_id: str) -> Result[str, TodoziError]
    def end_session(self, session_id: str) -> Result[None, TodoziError]
    def get_active_sessions(self) -> List[QueueSession]
```

**Session Features**:
- Automatic duration tracking
- Status validation
- Error handling for invalid transitions

### API Key Management
Secure API key generation and validation:

```python
@dataclass
class ApiKey:
    user_id: str
    public_key: str
    private_key: str
    active: bool
    
    @staticmethod
    def new() -> "ApiKey"
    def matches(self, public_key: str, private_key: Optional[str] = None) -> bool
```

**Security Features**:
- SHA-256/SHA-512 cryptographic hashing
- Active/inactive state management
- Secure key matching without exposing private keys

## Technical Constraints and Limitations

### Pydantic Dependency
The system uses Pydantic for data validation when available, with fallback to basic dataclasses:

```python
try:
    from pydantic import BaseModel, field_validator, model_validator
    BaseT = BaseModel
except Exception:
    BaseT = object  # Minimal fallback
```

**Impact**: Full validation features require Pydantic installation. Fallback mode provides basic functionality without validation.

### Enum Parsing Performance
The alias system in enum parsing adds O(n) lookup overhead. For high-performance scenarios, consider direct enum value assignment.

### Memory Usage
Large collections (TaskCollection, QueueCollection) store objects in memory. Consider pagination or database backing for large datasets.

## Performance Considerations

### UUID Generation
The system uses UUIDv4 with short prefixes for IDs:
```python
def short_uuid() -> str:
    return str(uuid.uuid4()).split("-")[0]
```

**Performance Impact**: UUID generation is cryptographically secure but may impact high-frequency operations.

### DateTime Operations
All timestamps use UTC with timezone awareness:
```python
def utc_now() -> datetime:
    return datetime.now(dt_timezone.utc)
```

**Benefit**: Consistent timezone handling across distributed systems.

## Error Handling Patterns

### Result Pattern Usage
All operations that can fail return `Result[T, TodoziError]`:

```python
# Correct usage pattern
result = Status.from_str("invalid_status")
if isinstance(result, Err):
    error = result.error
    # Handle error appropriately
else:
    status = result.value
```

### Validation Errors
Common validation points:
- Progress values (0-100 range)
- Enum string parsing
- Task status transitions
- Session state management

## Dependencies and Requirements

### Required Dependencies
- Python 3.8+ (for `from __future__ import annotations`)
- `uuid` module (standard library)
- `hashlib` module (standard library)
- `secrets` module (standard library)
- `datetime` module (standard library)

### Optional Dependencies
- `pydantic`: For enhanced validation and serialization
- Additional AI/ML libraries for MLEngine functionality

## Usage Examples

### Creating a Task
```python
# Basic task creation
task = Task.new(
    user_id="user_123",
    action="Implement authentication",
    time="2 hours",
    priority=Priority.HIGH,
    parent_project="auth_project",
    status=Status.TODO
)

# Full task creation with error handling
result = Task.new_full(
    user_id="user_123",
    action="Implement authentication",
    time="2 hours",
    priority=Priority.HIGH,
    parent_project="auth_project",
    status=Status.TODO,
    assignee=Assignee("ai"),
    tags=["auth", "security"],
    dependencies=["task_456"],
    context_notes="Use OAuth2 flow",
    progress=0
)

if isinstance(result, Err):
    print(f"Error: {result.error.message}")
else:
    task = result.value
```

### Agent Management
```python
# Create specialized agent
coder_agent = Agent.create_coder()

# Check capabilities
if coder_agent.has_capability("code_review"):
    print("Agent can review code")

# Format prompt with variables
prompt = coder_agent.get_formatted_prompt({
    "task": "Write a Python function",
    "language": "python",
    "context": "Data processing",
    "requirements": "Must handle edge cases"
})
```

### Queue Management
```python
# Create queue system
queue = QueueCollection()

# Add item and start session
item = QueueItem.new("Code review", "Review PR #123", Priority.HIGH)
queue.add_item(item)

session_result = queue.start_session(item.id)
if isinstance(session_result, Ok):
    session_id = session_result.value
    # Work on task...
    queue.end_session(session_id)
```

This documentation covers the core architecture and components of the Todozi system. The modular design supports extensibility while maintaining type safety and comprehensive error handling throughout the ecosystem.