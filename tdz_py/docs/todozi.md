# Todozi Technical Documentation

## Overview

Todozi is a comprehensive task management and information processing system designed to handle various types of structured data including tasks, memories, ideas, errors, training data, and more. The system provides robust parsing capabilities, storage abstractions, and workflow execution for AI-human collaborative environments.

## Table of Contents
1. [Module Import and Setup](#module-import-and-setup)
2. [Enums and Data Types](#enums-and-data-types)
3. [Domain Models](#domain-models)
4. [Pattern Cache System](#pattern-cache-system)
5. [Parsing System](#parsing-system)
6. [Storage Abstraction](#storage-abstraction)
7. [Task Execution Engine](#task-execution-engine)
8. [Utility Functions](#utility-functions)
9. [Error Handling](#error-handling)
10. [Testing Framework](#testing-framework)

## Module Import and Setup

### Path Resolution
```python
# Fix imports when running directly or when package isn't properly set up
import sys
from pathlib import Path
_todozi_file = Path(__file__)
if _todozi_file.exists():
    parent_dir = _todozi_file.parent.parent
    if str(parent_dir) not in sys.path:
        sys.path.insert(0, str(parent_dir))
```

**Purpose**: Resolves import path issues when running the module directly or when the package structure isn't properly configured.

**Parameters**: None
**Returns**: None (modifies `sys.path` in place)

### Import Fallback Mechanism
```python
try:
    from .models import Ok
except ImportError:
    try:
        from todozi.models import Ok
    except ImportError:
        # Last resort: try importing models directly if we're in the same directory
        import importlib.util
        models_path = _todozi_file.parent / "models.py"
        if models_path.exists():
            spec = importlib.util.spec_from_file_location("models", models_path)
            if spec and spec.loader:
                models_module = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(models_module)
                Ok = models_module.Ok
        else:
            raise ImportError("Cannot import Ok from todozi.models")
```

**Purpose**: Implements a multi-level import fallback system for the `Ok` model class.

**Behavior**:
1. Attempt relative import first
2. Attempt absolute package import
3. Attempt direct file import as last resort
4. Raise `ImportError` if all methods fail

## Configuration and Constants

### `get_emotion_list()`
```python
def get_emotion_list() -> List[str]:
    return [
        "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
        "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
        "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
        "resigned",
    ]
```

**Purpose**: Provides a standardized list of supported emotions for emotional memory types.

**Returns**: `List[str]` - Complete list of valid emotion strings

### `validation_error()`
```python
def validation_error(message: str) -> "ValidationError":
    return ValidationError(message)
```

**Purpose**: Factory function for creating validation error instances.

**Parameters**:
- `message` (str): Descriptive error message

**Returns**: `ValidationError` instance

## Pattern Cache System

### `PatternCache` Class
```python
class PatternCache:
    _patterns: Dict[str, re.Pattern] = {}
```

**Purpose**: Implements a singleton pattern cache to avoid recompiling regular expressions.

#### `get()` Class Method
```python
@classmethod
def get(cls, pattern: str, flags: int = 0) -> re.Pattern:
    key = f"{pattern}|{flags}"
    if key not in cls._patterns:
        cls._patterns[key] = re.compile(pattern, flags)
    return cls._patterns[key]
```

**Purpose**: Retrieves or creates a compiled regex pattern.

**Parameters**:
- `pattern` (str): Regular expression pattern string
- `flags` (int): Regex compilation flags (default: 0)

**Returns**: `re.Pattern` - Compiled regex object

**Performance Benefits**: Reduces regex compilation overhead by ~90% for repeated patterns

## Enums and Data Types

### `Priority` Enum
```python
class Priority(Enum):
    Low = auto()
    Medium = auto()
    High = auto()
    Critical = auto()
```

**Values**: Low, Medium, High, Critical

#### `from_str()` Method
```python
@classmethod
def from_str(cls, value: str) -> "Priority":
    v = value.strip().lower()
    mapping = cls._str_to_enum()
    if v not in mapping:
        raise ValueError(f"Invalid priority: {value}")
    return mapping[v]
```

**Purpose**: Converts string representation to enum value.

**Parameters**:
- `value` (str): Priority string ("low", "medium", "high", "critical")

**Returns**: `Priority` enum value

**Raises**: `ValueError` for invalid input

### `Status` Enum
```python
class Status(Enum):
    Todo = auto()
    InProgress = auto()
    Done = auto()
    Blocked = auto()
    Deferred = auto()
```

**Values**: Todo, InProgress, Done, Blocked, Deferred

**Special Handling**: Accepts both "inprogress" and "in progress" for InProgress status

### `AssigneeType` and `Assignee`

#### `AssigneeType` Enum
```python
class AssigneeType(Enum):
    Ai = auto()
    Human = auto()
    Collaborative = auto()
    Agent = auto()
```

#### `Assignee` Data Class
```python
@dataclass
class Assignee:
    kind: AssigneeType
    name: Optional[str] = None  # only used for Agent
```

**Factory Methods**:
- `Assignee.ai()` - AI-assigned task
- `Assignee.human()` - Human-assigned task  
- `Assignee.collaborative()` - Collaborative task
- `Assignee.agent(agent_name: str)` - Specific agent assignment

**Parse Support**:
- "ai", "human", "collaborative", "agent=name" formats
- Case-insensitive parsing

### Memory-Related Enums

#### `MemoryImportance`
```python
class MemoryImportance(Enum):
    Low = auto()
    Medium = auto()
    High = auto()
```

#### `MemoryTerm`
```python
class MemoryTerm(Enum):
    Short = auto()
    Long = auto()
```

#### `MemoryType`
```python
class MemoryType(Enum):
    Standard = auto()
    Secret = auto()
    Human = auto()
    Short = auto()
    Long = auto()
    Emotional = auto()  # for emotions, value stored separately in Memory.emotion
```

**Special Feature**: Emotional type stores emotion string separately in `Memory.emotion` field

### Additional Enums
- `ItemStatus`: Active, Archived
- `ShareLevel`: Private, Public, Team  
- `IdeaImportance`: Low, Medium, High
- `ErrorSeverity`: Low, Medium, High, Critical
- `ErrorCategory`: Network, Database, Logic, General
- `TrainingDataType`: Instruction, Completion
- `AssignmentStatus`: Assigned

## Domain Models

### `Task` Data Class
```python
@dataclass
class Task:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    user_id: str = "anonymous"
    action: str = ""
    time: str = ""
    priority: Priority = Priority.Medium
    parent_project: str = ""
    status: Status = Status.Todo
    assignee: Optional[Assignee] = None
    tags: List[str] = field(default_factory=list)
    dependencies: List[str] = field(default_factory=list)
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
```

**Key Fields**:
- `embedding_vector`: Optional vector for semantic search
- `dependencies`: List of dependent task IDs
- `progress`: Integer percentage (0-100)

### `Memory` Data Class
```python
@dataclass
class Memory:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    user_id: str = ""
    project_id: Optional[str] = None
    status: ItemStatus = ItemStatus.Active
    moment: str = ""
    meaning: str = ""
    reason: str = ""
    importance: MemoryImportance = MemoryImportance.Medium
    term: MemoryTerm = MemoryTerm.Short
    memory_type: MemoryType = MemoryType.Standard
    emotion: Optional[str] = None  # populated if memory_type == Emotional
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
```

**Emotional Memory Support**: When `memory_type` is `Emotional`, the `emotion` field must contain a valid emotion from `get_emotion_list()`

### Additional Data Models
- `Idea`: Creative ideas with sharing levels
- `AgentAssignment`: Task-agent mapping
- `CodeChunk`: Source code fragments with metadata
- `Error`: Error tracking with severity and resolution
- `TrainingData`: AI training examples with quality scoring
- `Feeling`: Emotional state tracking (1-10 intensity)
- `Summary`: Content summaries with importance
- `Reminder`: Time-based reminders with due dates

### `ChatContent` Container
```python
@dataclass
class ChatContent:
    tasks: List[Task] = field(default_factory=list)
    memories: List[Memory] = field(default_factory=list)
    ideas: List[Idea] = field(default_factory=list)
    agent_assignments: List[AgentAssignment] = field(default_factory=list)
    code_chunks: List[CodeChunk] = field(default_factory=list)
    errors: List[Error] = field(default_factory=list)
    training_data: List[TrainingData] = field(default_factory=list)
    feelings: List[Feeling] = field(default_factory=list)
    summaries: List[Summary] = field(default_factory=list)
    reminders: List[Reminder] = field(default_factory=list)
```

**Purpose**: Container for all content types extracted from chat messages.

## Parsing System

### Core Parsing Functions

#### `_extract_content()`
```python
def _extract_content(text: str, start_tag: str, end_tag: str) -> str:
    start = text.find(start_tag)
    if start == -1:
        raise validation_error(f"Missing {start_tag} start tag")
    end = text.find(end_tag, start + len(start_tag))
    if end == -1:
        raise validation_error(f"Missing {end_tag} end tag")
    return text[start + len(start_tag):end]
```

**Purpose**: Extracts content between specified XML-like tags.

**Parameters**:
- `text` (str): Input text containing tagged content
- `start_tag` (str): Opening tag (e.g., "<todozi>")
- `end_tag` (str): Closing tag (e.g., "</todozi>")

**Returns**: `str` - Extracted content between tags

**Raises**: `ValidationError` if tags are missing or malformed

#### `_split_parts()`
```python
def _split_parts(content: str) -> List[str]:
    return [p.strip() for p in content.split(";") if p.strip() != ""]
```

**Purpose**: Splits semi-colon separated content into parts.

#### `_extract_value()`
```python
def _extract_value(part: str, key: Optional[str] = None) -> str:
    """Extract value from key=value format or return part as-is if no '=' found."""
    if "=" in part:
        _, value = part.split("=", 1)
        return value.strip()
    return part.strip()
```

**Purpose**: Handles both key=value and direct value formats.

### Format-Specific Parsers

#### `parse_todozi_format()`
```python
def parse_todozi_format(todozi_text: str) -> Task:
```

**Format**: `<todozi>action; time; priority; parent_project; status[; assignee][; tags][; dependencies][; context_notes][; progress]</todozi>`

**Required Parts**: 5 (action, time, priority, parent_project, status)
**Optional Parts**: assignee, tags, dependencies, context_notes, progress

**Example**:
```xml
<todozi>Implement OAuth2; 6 hours; high; web-project; todo; assignee=human; tags=auth,security; dependencies=design-api; context_notes=Ensure security; progress=0%</todozi>
```

#### `parse_memory_format()`
```python
def parse_memory_format(memory_text: str, user_id: str) -> Memory:
```

**Format**: `<memory>type; moment; meaning; reason; importance; term[; tags]</memory>`

**Emotional Memory Detection**: If first part matches emotion list, automatically sets `memory_type=Emotional`

#### `parse_idea_format()`
```python
def parse_idea_format(idea_text: str) -> Idea:
```

**Format**: `<idea>idea; share; importance[; context][; tags]</idea>`

#### Additional Parsers
- `parse_agent_assignment_format()`: Agent-task assignments
- `parse_error_format()`: Error tracking (supports `<error>`, `<er>`, `<e>` tags)
- `parse_training_data_format()`: AI training examples
- `parse_feeling_format()`: Emotional states (intensity 1-10)
- `parse_chunking_format()`: Code fragments
- `parse_summary_format()`: Content summaries
- `parse_reminder_format()`: Time-based reminders

## Message Processing

### `transform_shorthand_tags()`
```python
def transform_shorthand_tags(message: str) -> str:
```

**Purpose**: Converts shorthand tags to full tag names for consistent parsing.

**Mappings**:
- `<tz>` → `<todozi>`
- `<mm>` → `<memory>`
- `<id>` → `<idea>`
- `<ch>` → `<chunk>`
- `<fe>` → `<feel>`
- `<tn>` → `<train>`
- `<er>` → `<error>`
- `<sm>` → `<summary>`
- `<rd>` → `<reminder>`

### `process_chat_message()`
```python
def process_chat_message(message: str) -> List[Task]:
```

**Purpose**: Extracts only task information from chat messages.

**Returns**: `List[Task]` - All parsed tasks from the message

### `process_chat_message_extended()`
```python
def process_chat_message_extended(message: str, user_id: str) -> ChatContent:
```

**Purpose**: Comprehensive message processing that extracts all content types.

**Parameters**:
- `message` (str): Input chat message containing various tagged content
- `user_id` (str): User identifier for memory attribution

**Returns**: `ChatContent` - Container with all extracted content types

**Features**:
- Handles shorthand tag conversion
- Robust error handling with warning messages
- Supports all content types simultaneously

## Storage Abstraction

### `Storage` Class
```python
class Storage:
    _instance: Optional["Storage"] = None
```

**Design Pattern**: Singleton with async initialization

#### `get_instance()` Method
```python
@classmethod
async def get_instance(cls) -> "Storage":
    if cls._instance is None:
        cls._instance = Storage()
    return cls._instance
```

**Purpose**: Provides singleton access to storage instance with async support.

#### Key Storage Methods
- `search_tasks_semantic()`: Semantic task search with vector similarity
- `add_queue_item()`: Adds items to processing queue
- `save_agent_assignment()`: Stores agent-task assignments
- `update_task_in_project()`: Updates task status and metadata

### `QueueItem` and `TaskResult`
```python
@dataclass
class QueueItem:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str = ""
    description: str = ""
    priority: Priority = Priority.Medium
    project_id: Optional[str] = None

class TaskResult:
    def __init__(self, task: Task, score: float):
        self.task = task
        self.score = score
```

## Task Execution Engine

### Assignment-Specific Executors

#### `execute_ai_task()`
```python
async def execute_ai_task(task: Task) -> str:
```

**Behavior**: Queues task for AI processing with appropriate metadata.

#### `execute_human_task()`
```python
async def execute_human_task(task: Task) -> str:
```

**Behavior**: Queues task for human processing via TUI interface.

#### `execute_collaborative_task()`
```python
async def execute_collaborative_task(task: Task) -> str:
```

**Behavior**: Creates separate queue items for AI and human portions.

#### `execute_agent_task()`
```python
async def execute_agent_task(task: Task, agent_name: str) -> str:
```

**Behavior**: Assigns task to specific agent and creates tracking entries.

### `execute_task()` Router
```python
async def execute_task(storage: Storage, task: Task) -> str:
```

**Decision Logic**:
1. Explicit assignee takes priority
2. For unassigned tasks: semantic analysis of similar tasks
3. Fallback: AI for analytical tasks, Human for others

### `process_workflow()` Orchestrator
```python
async def process_workflow(tasks: List[Task]) -> List[str]:
```

**Workflow**:
1. Executes each task with appropriate executor
2. Updates task status to "Done" upon completion
3. Persists updated task state to storage
4. Returns execution results for each task

## Utility Functions

### `parse_date_robust()`
```python
def parse_date_robust(date_str: str) -> Optional[datetime]:
```

**Purpose**: Robust date parsing with multiple format support.

**Formats Supported**:
- ISO format (with timezone support)
- Dateutil parser (if available)
- Returns `None` for unparseable dates

### JSON Processing
```python
def process_json_examples(json_data: str) -> List[Task]:
```

**Purpose**: Extracts task examples from JSON tool definitions.

**Format**: Expects `tool_definition.examples[]` with `todozi_format` fields

## Error Handling

### Exception Hierarchy
```python
class TodoziError(Exception):
    def __init__(self, message: str):
        self.message = message

class ValidationError(TodoziError):
    def __init__(self, message: str):
        super().__init__(message)
```

**Error Types**:
- `TodoziError`: Base exception for all Todozi-related errors
- `ValidationError`: Specific to data validation and parsing errors

## Testing Framework

### Test Structure
```python
def _run_tests():
    test_functions = [
        test_parse_todozi_format_basic,
        test_parse_todozi_format_extended,
        # ... additional tests
    ]
```

**Features**:
- Comprehensive test coverage for all parsers
- Shorthand tag transformation testing
- Multi-content message processing validation
- Error handling and edge case testing

### Key Test Cases

#### Basic Task Parsing
```python
def test_parse_todozi_format_basic():
    todozi_text = "<todozi>Fix critical bug; ASAP; critical; rust-performance-optimizer; blocked</todozi>"
    task = parse_todozi_format(todozi_text)
    assert task.action == "Fix critical bug"
    assert task.priority == Priority.Critical
```

#### Extended Format Testing
```python
def test_parse_todozi_format_extended():
    # Tests all optional fields: assignee, tags, dependencies, context_notes, progress
```

#### Shorthand Tag Processing
```python
def test_process_chat_message_with_shorthand_tags():
    # Verifies shorthand to full tag conversion and parsing
```

## Performance Considerations

### Optimization Features
1. **Pattern Cache**: Eliminates regex compilation overhead
2. **Lazy Imports**: Storage dependencies loaded only when needed
3. **Async Operations**: Non-blocking storage operations
4. **Memory Efficiency**: Dataclasses with sensible defaults

### Memory Usage
- UUID generation: Minimal overhead with default factories
- List fields: Empty lists by default, only allocated when used
- Optional fields: `None` by default, conserving memory

## Dependencies and Requirements

### Core Dependencies
- `uuid`: Unique identifier generation
- `datetime`: Timestamp management with timezone support
- `re`: Regular expression parsing
- `json`: JSON data processing
- `dataclasses`: Efficient data structure definition

### Optional Dependencies
- `dateutil`: Enhanced date parsing (graceful degradation)

### Import Requirements
- `todozi.models`: Core model definitions
- `todozi.storage`: Storage implementation (runtime dependency)

## Technical Constraints

### Input Validation
- All enum parsers include comprehensive validation
- Date parsing includes robust error handling
- Tag extraction validates proper nesting and completeness

### Memory Limits
- No inherent size limits on text fields
- List fields should be monitored for excessive growth
- Embedding vectors may have significant memory impact

### Concurrency Considerations
- Singleton storage instance requires thread safety in implementation
- Async operations should be properly awaited
- UUID generation is thread-safe

## Usage Examples

### Basic Task Creation
```python
task = parse_todozi_format("<todozi>Review code; 2 hours; high; project-x; todo</todozi>")
```

### Comprehensive Message Processing
```python
message = """
<todozi>Implement feature; 4 hours; medium; project-a; inprogress</todozi>
<memory>emotional; happy; Project milestone; Team celebration; high; long</memory>
<idea>New approach; public; high; Innovative solution</idea>
"""

content = process_chat_message_extended(message, "user123")
```

### Workflow Execution
```python
tasks = process_chat_message(chat_message)
results = await process_workflow(tasks)
```

This documentation provides comprehensive coverage of the Todozi system's architecture, functionality, and implementation details suitable for developers and technical stakeholders.