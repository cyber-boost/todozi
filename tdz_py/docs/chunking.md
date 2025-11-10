# Todozi Code Chunking System Technical Documentation

## Overview

The Todozi Code Chunking System is a Python framework designed for managing code generation tasks through hierarchical chunking. The system provides structured organization of code components, dependency management, and state tracking for complex software development workflows.

## Architecture

### Core Components
- **ChunkingLevel**: Defines granularity levels for code organization
- **ProjectState**: Manages overall project progress and metadata
- **ContextWindow**: Tracks current development context and dependencies
- **CodeChunk**: Represents individual code units with metadata
- **CodeGenerationGraph**: Manages relationships between chunks and project state

## Class Documentation

### Result Type System

#### `Ok(Generic[T])`
Wrapper for successful operation results.

**Parameters:**
- `value: T` - The successful result value

**Methods:**
- `__init__(value: T)`: Initializes with successful value

#### `Err(Generic[E])`
Wrapper for error operation results.

**Parameters:**
- `error: E` - The error value

**Methods:**
- `__init__(error: E)`: Initializes with error value

**Type Alias:**
- `Result = Union[Ok[T], Err[E]]`: Rust-style Result type for error handling

### ChunkingLevel Enum

Defines hierarchical levels of code organization with token limits and descriptions.

**Enum Members:**
- `PROJECT`: High-level project planning (100 tokens max)
- `MODULE`: Major system components (500 tokens max)
- `CLASS`: Class definitions and major functions (1000 tokens max)
- `METHOD`: Individual methods (300 tokens max)
- `BLOCK`: Small code blocks (100 tokens max)

**Methods:**

#### `max_tokens() -> int`
Returns maximum token count for the chunking level.

**Returns:**
- `int`: Token limit for the level

**Example:**
```python
level = ChunkingLevel.MODULE
print(level.max_tokens())  # Output: 500
```

#### `description() -> str`
Returns descriptive text for the chunking level.

**Returns:**
- `str`: Human-readable description

#### `example() -> str`
Returns example use case for the level.

**Returns:**
- `str`: Example scenario

#### `from_string(s: str) -> ChunkingLevel`
Converts string to ChunkingLevel enum member.

**Parameters:**
- `s: str` - String representation of level

**Returns:**
- `ChunkingLevel`: Corresponding enum member

**Raises:**
- `ValueError`: If string doesn't match any level

**Example:**
```python
level = ChunkingLevel.from_string("module")
print(level)  # Output: ChunkingLevel.MODULE
```

### ChunkStatus Enum

Tracks the lifecycle state of code chunks.

**Enum Members:**
- `PENDING`: Chunk awaiting processing
- `IN_PROGRESS`: Currently being developed
- `COMPLETED`: Development finished
- `VALIDATED`: Tests and validation passed
- `FAILED`: Development or validation failed

### TodoziError Exception

Base exception for Todozi project-specific errors.

**Parameters:**
- `message: str` - Error description

### ProjectState Class

Manages global project metadata and progress tracking.

**Attributes:**
- `total_lines: int` - Lines of code written (default: 0)
- `max_lines: int` - Maximum allowed lines (default: 0)
- `current_module: str` - Currently active module (default: "")
- `dependencies: List[str]` - Project dependencies (default: [])
- `completed_modules: Set[str]` - Successfully completed modules (default: set())
- `pending_modules: Set[str]` - Modules awaiting completion (default: set())
- `global_variables: Dict[str, str]` - Project-wide variables (default: {})
- `created_at: datetime` - Creation timestamp (UTC)
- `updated_at: datetime` - Last modification timestamp (UTC)

**Methods:**

#### `to_state_string() -> str`
Generates formatted string representation of project state.

**Returns:**
- `str`: XML-like formatted state summary

**Example Output:**
```xml
<project_state>
- Total lines written: 150/1000
- Current module: database_handler
- Dependencies: requests, sqlite3
- Completed modules: utils, config
- Pending modules: api_handler, cli
- Global variables: API_KEY=secret123, DB_PATH=/data/db.sqlite
- Created: 2024-01-15 10:30:00
- Updated: 2024-01-15 14:45:00
</project_state>
```

#### `add_completed_module(module: str) -> None`
Marks module as completed and updates timestamp.

**Parameters:**
- `module: str` - Module identifier

#### `add_pending_module(module: str) -> None`
Adds module to pending set and updates timestamp.

**Parameters:**
- `module: str` - Module identifier

#### `set_global_variable(key: str, value: str) -> None`
Sets global variable value.

**Parameters:**
- `key: str` - Variable name
- `value: str` - Variable value

#### `increment_lines(lines: int) -> None`
Increments line count and updates timestamp.

**Parameters:**
- `lines: int` - Number of lines to add

### ContextWindow Class

Manages current development context and scope information.

**Attributes:**
- `previous_class: str` - Previously worked-on class (default: "")
- `current_class: str` - Current class focus (default: "")
- `next_planned: str` - Next planned class/module (default: "")
- `global_vars_in_scope: List[str]` - Accessible global variables (default: [])
- `imports_used: List[str]` - Import statements in use (default: [])
- `function_signatures: Dict[str, str]` - Available function signatures (default: {})
- `error_patterns_seen: List[str]` - Previously encountered error patterns (default: [])
- `created_at: datetime` - Creation timestamp (UTC)
- `updated_at: datetime` - Last modification timestamp (UTC)

**Methods:**

#### `to_context_string() -> str`
Generates formatted context summary.

**Returns:**
- `str`: XML-like formatted context information

#### `add_import(import_stmt: str) -> None`
Adds import statement to context.

**Parameters:**
- `import_stmt: str` - Import statement

#### `add_function_signature(name: str, signature: str) -> None`
Records function signature in context.

**Parameters:**
- `name: str` - Function name
- `signature: str` - Function signature

#### `add_error_pattern(pattern: str) -> None`
Adds error pattern to tracking list.

**Parameters:**
- `pattern: str` - Error pattern description

#### `set_current_class(class_name: str) -> None`
Updates current class and maintains previous class reference.

**Parameters:**
- `class_name: str` - New current class name

### CodeChunk Class

Represents individual code units with comprehensive metadata.

**Attributes:**
- `chunk_id: str` - Unique identifier (required)
- `status: ChunkStatus` - Current state (default: PENDING)
- `dependencies: Set[str]` - Required chunk IDs (default: set())
- `code: str` - Actual code content (default: "")
- `tests: str` - Associated test code (default: "")
- `validated: bool` - Validation status (default: False)
- `level: ChunkingLevel` - Granularity level (default: BLOCK)
- `estimated_tokens: int` - Token count estimate (default: 0)
- `created_at: datetime` - Creation timestamp (UTC)
- `updated_at: datetime` - Last modification timestamp (UTC)

**Methods:**

#### `add_dependency(dep: str) -> None`
Adds dependency to chunk.

**Parameters:**
- `dep: str` - Dependency chunk ID

#### `set_code(code: str) -> None`
Sets code content and estimates token count.

**Parameters:**
- `code: str` - Code content
- **Token Estimation:** Uses `len(code.split())` for simple word count

#### `set_tests(tests: str) -> None`
Sets test code content.

**Parameters:**
- `tests: str` - Test code

#### `mark_completed() -> None`
Transitions chunk to COMPLETED status.

#### `mark_validated() -> None`
Marks chunk as validated (status: VALIDATED).

#### `mark_failed() -> None`
Marks chunk as failed (status: FAILED).

### CodeGenerationGraph Class

Central manager for code chunk relationships and project coordination.

**Attributes:**
- `project_state: ProjectState` - Global project state
- `context_window: ContextWindow` - Current development context
- `chunks: Dict[str, CodeChunk]` - All chunks in the project

**Constructor:**
- `__init__(max_lines: int = 0)` - Initializes with maximum line limit

**Methods:**

#### `add_chunk(chunk_id: str, level: ChunkingLevel, deps: List[str]) -> None`
Adds new chunk to the graph.

**Parameters:**
- `chunk_id: str` - Unique chunk identifier
- `level: ChunkingLevel` - Chunk granularity level
- `deps: List[str]` - List of dependency chunk IDs

#### `get_ready_chunks() -> List[str]`
Returns chunks ready for processing.

**Criteria for Readiness:**
- Status is PENDING
- All dependencies are COMPLETED or VALIDATED

**Returns:**
- `List[str]`: List of ready chunk IDs

#### `get_chunk(chunk_id: str) -> Optional[CodeChunk]`
Retrieves chunk by ID (immutable).

**Parameters:**
- `chunk_id: str` - Chunk identifier

**Returns:**
- `Optional[CodeChunk]`: Chunk object or None if not found

#### `get_chunk_mut(chunk_id: str) -> Optional[CodeChunk]`
Retrieves mutable chunk reference.

**Parameters:**
- `chunk_id: str` - Chunk identifier

**Returns:**
- `Optional[CodeChunk]`: Mutable chunk reference or None

#### `update_chunk_code(chunk_id: str, code: str) -> Result[None, str]`
Updates chunk code and increments project line count.

**Parameters:**
- `chunk_id: str` - Target chunk identifier
- `code: str` - New code content

**Returns:**
- `Result[None, str]`: Ok(None) on success, Err message on failure

**Line Counting:** Uses `len(code.splitlines())` for accurate line count

#### `update_chunk_tests(chunk_id: str, tests: str) -> Result[None, str]`
Updates chunk test code.

**Parameters:**
- `chunk_id: str` - Target chunk identifier
- `tests: str` - New test code

**Returns:**
- `Result[None, str]`: Ok(None) on success, Err message on failure

#### `mark_chunk_completed(chunk_id: str) -> Result[None, str]`
Marks chunk as completed and updates project state.

**Parameters:**
- `chunk_id: str` - Target chunk identifier

**Returns:**
- `Result[None, str]`: Ok(None) on success, Err message on failure

#### `mark_chunk_validated(chunk_id: str) -> Result[None, str]`
Marks chunk as validated.

**Parameters:**
- `chunk_id: str` - Target chunk identifier

**Returns:**
- `Result[None, str]`: Ok(None) on success, Err message on failure

#### `get_project_summary() -> str`
Generates comprehensive project summary.

**Returns:**
- `str`: Formatted summary including counts and state information

#### `get_next_chunk_to_work_on() -> Optional[str]`
Returns highest-priority ready chunk.

**Returns:**
- `Optional[str]`: Next chunk ID or None if none ready

#### `get_chunks_by_level(level: ChunkingLevel) -> List[CodeChunk]`
Filters chunks by granularity level.

**Parameters:**
- `level: ChunkingLevel` - Target level

**Returns:**
- `List[CodeChunk]`: Chunks at specified level

#### `get_dependency_chain(chunk_id: str) -> List[str]`
Returns ordered dependency chain for chunk.

**Parameters:**
- `chunk_id: str` - Target chunk identifier

**Returns:**
- `List[str]`: Ordered list of dependency chunk IDs

## Utility Functions

### `parse_chunking_format(chunk_text: str) -> Result[CodeChunk, str]`
Parses chunk from formatted text string.

**Format Specification:**
```xml
<chunk>id; level; description; dependencies; code</chunk>
```

**Parameters:**
- `chunk_text: str` - Formatted chunk text

**Returns:**
- `Result[CodeChunk, str]`: Parsed chunk or error message

**Example:**
```python
text = "<chunk>db_handler; module; Database interface; utils; import sqlite3</chunk>"
result = parse_chunking_format(text)
```

### `process_chunking_message(message: str) -> Result[List[CodeChunk], str]`
Extracts multiple chunks from message text.

**Parameters:**
- `message: str` - Text containing chunk definitions

**Returns:**
- `Result[List[CodeChunk], str]`: List of parsed chunks or error

## Technical Constraints and Limitations

### Performance Considerations
- **Chunk Dependency Resolution:** O(n) complexity for each `get_ready_chunks()` call
- **Dependency Chain Building:** Recursive traversal may be inefficient for deep chains
- **Token Estimation:** Simple word counting may not match actual LLM tokenization

### Memory Usage
- All chunks stored in memory; not suitable for extremely large projects
- String representations of state/context can become large

### Error Handling
- Extensive use of Result type for Rust-style error handling
- Missing chunk errors handled gracefully with Optional returns
- Parsing errors provide descriptive messages

### Dependencies and Requirements
- **Python Version:** 3.7+ (due to dataclasses and typing enhancements)
- **Required Packages:** Standard library only (datetime, re, enum, typing)
- **Timezone Handling:** All timestamps stored as UTC

## Testing

The module includes comprehensive test functions:

- `test_chunking_levels()`: Validates token limits and enum functionality
- `test_project_state()`: Tests state management operations
- `test_code_generation_graph()`: Verifies graph operations and dependency resolution
- `test_parse_chunking_format()`: Ensures chunk parsing correctness

**Run Tests:**
```bash
python todozi_chunking.py
```

## Design Decisions

### Rust-Inspired Architecture
- **Result Type:** Adopted for explicit error handling vs exceptions
- **Immutable/Mutable Access:** Separate getters mirror Rust's ownership system
- **UTC Timezone:** Consistent timestamp handling across systems

### XML-Like Formatting
- Human-readable state representations
- Easy parsing by external tools
- Consistent structure for machine processing

### Hierarchical Chunking
- Progressive refinement from high-level to detailed implementation
- Natural mapping to software architecture patterns
- Flexible granularity for different development phases

## Usage Example

```python
# Initialize project
graph = CodeGenerationGraph(max_lines=5000)

# Add chunks with dependencies
graph.add_chunk("database", ChunkingLevel.MODULE, [])
graph.add_chunk("user_model", ChunkingLevel.CLASS, ["database"])
graph.add_chunk("auth_service", ChunkingLevel.MODULE, ["database"])

# Process ready chunks
ready = graph.get_ready_chunks()
if ready:
    next_chunk = graph.get_next_chunk_to_work_on()
    # Generate and set code
    graph.update_chunk_code(next_chunk, "class User:\n    def __init__(self):\n        pass")
    graph.mark_chunk_completed(next_chunk)

# Get project summary
print(graph.get_project_summary())
```

This documentation provides comprehensive coverage of the Todozi Code Chunking System's architecture, API, and implementation details for developers and technical stakeholders.