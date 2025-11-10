# Todozi Memory Management System Technical Documentation

## Overview

The Todozi Memory Management System provides a comprehensive framework for managing and organizing memory objects with advanced search capabilities, statistical analysis, and robust error handling. The system is designed to handle various types of memories with different characteristics and importance levels.

## Architecture and Design Decisions

### Core Design Principles
- **Immutable Data Models**: Memory objects are implemented as frozen dataclasses to ensure thread safety
- **Separation of Concerns**: Clear separation between data models, business logic, and parsing functionality
- **Async-First Design**: Core operations use asynchronous locking for concurrent access
- **Optimized Search**: Efficient search implementation using inverted index pattern
- **Type Safety**: Comprehensive type hints and validation throughout the system

### System Components
1. **Data Models**: Memory, MemoryUpdate, MemoryStatistics
2. **Manager Layer**: MemoryManager with thread-safe operations
3. **Parser Layer**: Text-based memory parsing with validation
4. **Enumeration System**: MemoryImportance, MemoryTerm, EmotionalMemoryType

## Data Models

### MemoryImportance Enumeration
Defines the importance levels for memory prioritization.

```python
class MemoryImportance(Enum):
    Low = auto()      # Low priority memories
    Medium = auto()   # Default importance level
    High = auto()     # Important memories requiring attention
    Critical = auto() # Critical memories with highest priority
```

### MemoryTerm Enumeration
Specifies the temporal duration of memories.

```python
class MemoryTerm(Enum):
    Short = auto()    # Short-term, temporary memories
    Long = auto()     # Long-term, persistent memories
```

### EmotionalMemoryType Class
Represents emotionally-tagged memories with validation.

**Attributes:**
- `emotion` (str): Validated emotional state from predefined list

**Validation:**
- Only accepts emotions from the predefined EMOTIONS list
- Immutable implementation using `@dataclass(frozen=True)`

```python
# Usage Example
emotional_memory = EmotionalMemoryType(emotion="happy")
```

### Memory Data Class
Core memory data structure with comprehensive metadata.

**Attributes:**
- `id` (str): UUID-based unique identifier (auto-generated)
- `user_id` (str): Owner identifier
- `project_id` (Optional[str]): Optional project association
- `status` (str): Memory status ("Active" by default)
- `moment` (str): Description of the memory event
- `meaning` (str): Significance or interpretation
- `reason` (str): Rationale or cause
- `importance` (MemoryImportance): Priority level
- `term` (MemoryTerm): Temporal duration
- `memory_type` (MemoryTypeUnion): Classification type
- `tags` (List[str]): Organizational labels
- `created_at` (datetime): Creation timestamp (UTC)
- `updated_at` (datetime): Last modification timestamp (UTC)

**Default Values:**
- `importance`: MemoryImportance.Medium
- `term`: MemoryTerm.Short
- `memory_type`: "Standard"
- Timestamps: Current UTC time

### MemoryUpdate Data Class
Builder pattern for partial memory updates.

```python
# Usage Example
update = MemoryUpdate.builder()
update.moment = "Updated moment description"
update.tags = ["updated", "priority"]
```

### MemoryStatistics Data Class
Comprehensive memory collection analytics.

**Properties:**
- `short_term_percentage`: Percentage of short-term memories
- `long_term_percentage`: Percentage of long-term memories  
- `critical_percentage`: Percentage of critical importance memories

## MemoryManager Class

### Initialization
```python
manager = MemoryManager()
```

### Core Storage Structures
- `memories`: Dictionary mapping memory IDs to Memory objects
- `memory_tags`: Dictionary tracking tags per memory
- `search_index`: Inverted index for efficient text search

### Thread Safety Implementation
- **Read Operations**: Use `threading.Lock()` (`_lock`)
- **Write Operations**: Use `asyncio.Lock()` for async compatibility

### Method Specifications

#### `create_memory(memory: Memory) -> str`
Creates a new memory with automatic ID generation and indexing.

**Parameters:**
- `memory` (Memory): Memory object to create

**Returns:**
- `str`: Generated memory ID

**Async Context:** Requires asynchronous execution

**Error Handling:** Thread-safe creation with automatic indexing

```python
# Usage Example
memory = Memory(
    user_id="user123",
    moment="Project milestone achieved",
    importance=MemoryImportance.High
)
memory_id = await manager.create_memory(memory)
```

#### `get_memory(memory_id: str) -> Optional[Memory]`
Retrieves a memory by ID.

**Parameters:**
- `memory_id` (str): Memory identifier

**Returns:**
- `Optional[Memory]`: Memory object or None if not found

**Thread Safety:** Uses threading lock for read consistency

#### `update_memory(memory_id: str, updates: MemoryUpdate) -> Memory`
Partial memory update with automatic timestamp and index updates.

**Parameters:**
- `memory_id` (str): Target memory identifier
- `updates` (MemoryUpdate): Partial update specifications

**Returns:**
- `Memory`: Updated memory object

**Async Context:** Requires asynchronous execution

**Error Handling:** Raises TodoziError if memory not found

```python
# Usage Example
update = MemoryUpdate(
    moment="Revised milestone description",
    importance=MemoryImportance.Critical
)
updated_memory = await manager.update_memory(memory_id, update)
```

#### `delete_memory(memory_id: str) -> None`
Removes a memory and cleans up associated indexes.

**Parameters:**
- `memory_id` (str): Memory identifier to delete

**Async Context:** Requires asynchronous execution

**Error Handling:** Raises TodoziError if memory not found

### Search and Query Methods

#### `search_memories(query: str) -> List[Memory]`
Efficient text search using inverted index.

**Algorithm:**
1. Tokenizes query into words
2. Performs set intersection on indexed memory IDs
3. Returns matching memories

**Performance:** O(k) where k is number of query words

```python
# Usage Example
results = manager.search_memories("project milestone")
```

#### Filter Methods
- `get_memories_by_importance(importance)`: Filter by importance level
- `get_memories_by_term(term)`: Filter by temporal term
- `get_memories_by_tag(tag)`: Case-insensitive tag filtering
- `get_recent_memories(limit)`: Most recently created memories
- `get_memories_by_type(memory_type)`: Type-specific filtering

### Statistical Methods

#### `get_memory_statistics() -> MemoryStatistics`
Comprehensive memory collection analytics.

**Metrics Collected:**
- Total memory count
- Short/long term distribution
- Importance level distribution
- Type classification counts
- Unique tag count

```python
# Usage Example
stats = manager.get_memory_statistics()
print(f"Critical memories: {stats.critical_percentage:.1f}%")
```

#### `get_tag_statistics() -> Dict[str, int]`
Tag usage frequency analysis.

## Parser Module

### `parse_memory_format(memory_text: str, user_id: str) -> Memory`
Parses structured text format into Memory objects.

**Input Format:**
```xml
<memory>type; moment; meaning; reason; importance; term; tags</memory>
```

**Parsing Logic:**
1. Validates XML-like tag structure
2. Splits content by semicolon delimiters
3. Maps string values to appropriate enum types
4. Handles emotional vs standard memory types
5. Validates all required fields

**Error Handling:**
- Missing start/end tags
- Insufficient field count
- Invalid enum values
- Emotional type validation

```python
# Usage Example
text = "<memory>standard; Meeting; Product discussion; Weekly sync; high; short; work,meeting</memory>"
memory = parse_memory_format(text, "user123")
```

## Error Handling

### TodoziError Exception
Custom exception for memory management errors.

**Usage:**
```python
try:
    memory = parse_memory_format(invalid_text, user_id)
except TodoziError as e:
    print(f"Parsing error: {e.message}")
```

### Validation Points
- EmotionalMemoryType emotion validation
- Memory format parsing validation
- Memory existence checks in update/delete operations
- Enum value mapping validation

## Performance Considerations

### Memory Efficiency
- **Inverted Index**: O(1) word lookup for search operations
- **Lazy Evaluation**: Statistics computed on-demand
- **Batch Operations**: Efficient filtering with list comprehensions

### Concurrency Model
- **Read-Heavy**: Threading lock for read operations
- **Write Operations**: Async locks for concurrent modifications
- **Index Updates**: Atomic index maintenance during CRUD operations

## Dependencies and Requirements

### Required Packages
- `asyncio`: Asynchronous operation support
- `uuid`: Unique identifier generation
- `threading`: Thread safety for synchronous operations
- `dataclasses`: Immutable data model implementation
- `typing_extensions`: TypeAlias support for complex type definitions

### Python Version Compatibility
- Requires Python 3.7+ (due to `from __future__ import annotations`)
- Async/await syntax compatibility
- Dataclass support

## Technical Constraints and Limitations

### Memory Constraints
- **In-Memory Storage**: Not suitable for large-scale persistence
- **Search Index Memory**: Additional memory overhead for search optimization

### Scalability Limits
- Single-process implementation
- No distributed locking mechanism
- Memory-intensive operations with large datasets

### Type System Limitations
- Complex union types may impact static analysis
- Emotional memory type validation at runtime only

## Testing Framework

### Test Coverage Areas
1. **Manager Operations**: CRUD functionality and thread safety
2. **Parser Validation**: Format parsing and error handling
3. **Statistical Calculations**: Percentage computations and edge cases
4. **Async Integration**: Concurrent operation testing

### Test Execution
```python
# Run comprehensive tests
if __name__ == "__main__":
    _run_tests()
```

## Usage Patterns

### Basic Memory Lifecycle
```python
# Create memory
memory = Memory(user_id="user1", moment="Important event")
memory_id = await manager.create_memory(memory)

# Query and update
memories = manager.search_memories("important")
await manager.update_memory(memory_id, MemoryUpdate(importance=MemoryImportance.High))

# Analyze statistics
stats = manager.get_memory_statistics()
```

### Bulk Operations Pattern
```python
# Batch memory creation
async def create_multiple_memories(memories_list):
    for memory in memories_list:
        await manager.create_memory(memory)
```

This documentation provides comprehensive coverage of the Todozi Memory Management System's architecture, implementation details, and usage patterns for developers and technical stakeholders.