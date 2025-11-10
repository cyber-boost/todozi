# Todozi Technical Documentation

## Overview

Todozi is a Python-based summary management system that provides structured storage and retrieval of text summaries with priority classification and tagging capabilities. The system is designed with Rust-inspired error handling and a fluent API for updates.

## Architecture and Design Decisions

### Core Design Principles
- **Rust-inspired Error Handling**: Custom exception hierarchy with dedicated validation errors
- **Immutable Data Patterns**: Defensive copying to prevent external mutation
- **Fluent Interface**: Builder pattern for update operations
- **Enum-based Priority System**: Type-safe priority classification
- **Dataclass Usage**: Leveraging Python dataclasses for clean data models

### System Components
- **Summary**: Core data model for storing summary information
- **SummaryManager**: Central manager for CRUD operations and search
- **SummaryUpdate**: Fluent builder for partial updates
- **SummaryStatistics**: Analytics and metrics component
- **Parser Utility**: Text format parsing for external data ingestion

## Detailed Class and Method Documentation

### Exception Hierarchy

#### `TodoziError`
**Base Class**: `Exception`
**Description**: Root exception class for all Todozi-specific errors

**Properties**:
- `message: str` - Human-readable error description

**Usage**:
```python
try:
    # Todozi operation
except TodoziError as e:
    print(f"Todozi error: {e.message}")
```

#### `ValidationError`
**Base Class**: `TodoziError`
**Description**: Specific error for validation failures and invalid operations

**Usage**:
```python
try:
    manager.update_summary("invalid_id", update)
except ValidationError as e:
    print(f"Validation failed: {e.message}")
```

### Priority Enumeration

#### `SummaryPriority`
**Base Class**: `Enum`
**Description**: Defines priority levels for summaries

**Values**:
- `Low` - Low priority items
- `Medium` - Default priority level
- `High` - High priority items
- `Critical` - Critical priority items

**Methods**:

##### `from_string(cls, s: str) -> SummaryPriority`
**Description**: Converts string representation to enum value

**Parameters**:
- `s: str` - Case-insensitive string representation

**Returns**: `SummaryPriority` enum value

**Raises**: `ValueError` if string doesn't match any enum value

**Usage**:
```python
priority = SummaryPriority.from_string("high")  # Returns SummaryPriority.High
```

### Core Data Models

#### `Summary`
**Decorator**: `@dataclass`
**Description**: Primary data structure representing a text summary

**Properties**:
- `id: str` - UUID identifier (auto-generated)
- `content: str` - Main summary text content
- `context: Optional[str]` - Optional context information
- `priority: SummaryPriority` - Priority classification (default: Medium)
- `tags: List[str]` - List of categorization tags
- `created_at: datetime` - Creation timestamp (UTC)
- `updated_at: datetime` - Last modification timestamp (UTC)

**Generated Methods**:
- `copy() -> Summary` - Creates a defensive copy with list duplication

**Usage**:
```python
summary = Summary(
    content="Project completed successfully",
    context="Final project delivery",
    priority=SummaryPriority.High,
    tags=["project", "completion"]
)
```

#### `SummaryUpdate`
**Decorator**: `@dataclass`
**Description**: Fluent builder for partial summary updates

**Properties**:
- `content: Optional[str]` - New content (optional)
- `context: Optional[str]` - New context (optional)
- `priority: Optional[SummaryPriority]` - New priority (optional)
- `tags: Optional[List[str]]` - New tag list (optional)

**Fluent Methods**:

##### `content_set(content: str) -> SummaryUpdate`
Sets the content field and returns self for chaining

##### `context_set(context: str) -> SummaryUpdate`
Sets the context field and returns self for chaining

##### `priority_set(priority: SummaryPriority) -> SummaryUpdate`
Sets the priority field and returns self for chaining

##### `tags_set(tags: List[str]) -> SummaryUpdate`
Sets the tags field and returns self for chaining

**Alternative Fluent API**:
- `with_content(content: str) -> SummaryUpdate`
- `with_context(context: str) -> SummaryUpdate`
- `with_priority(priority: SummaryPriority) -> SummaryUpdate`
- `with_tags(tags: List[str]) -> SummaryUpdate`

**Usage**:
```python
update = SummaryUpdate()\
    .content_set("Updated content")\
    .priority_set(SummaryPriority.Critical)\
    .tags_set(["updated", "urgent"])
```

#### `SummaryStatistics`
**Decorator**: `@dataclass`
**Description**: Statistical information about summaries

**Properties**:
- `total_summaries: int` - Total number of summaries
- `high_priority_summaries: int` - Count of high and critical priority summaries
- `unique_tags: int` - Number of distinct tags

**Methods**:

##### `high_priority_percentage() -> float`
**Description**: Calculates percentage of high-priority summaries

**Returns**: `float` between 0.0 and 100.0

**Edge Cases**: Returns 0.0 when total_summaries is 0

### Manager Class

#### `SummaryManager`
**Description**: Central management class for summary operations

**Properties**:
- `summaries: Dict[str, Summary]` - Primary storage dictionary
- `summary_tags: Dict[str, List[str]]` - Tag index for efficient searching

#### Core CRUD Operations

##### `create_summary(summary: Summary) -> str`
**Description**: Creates a new summary with auto-generated ID and timestamps

**Parameters**:
- `summary: Summary` - Summary object to create

**Returns**: `str` - Generated UUID identifier

**Side Effects**:
- Generates new UUID for summary.id
- Sets created_at and updated_at to current UTC time
- Updates internal storage and tag index

**Usage**:
```python
manager = SummaryManager()
summary = Summary(content="New summary")
summary_id = manager.create_summary(summary)
```

##### `get_summary(summary_id: str) -> Optional[Summary]`
**Description**: Retrieves a summary by ID

**Parameters**:
- `summary_id: str` - UUID identifier

**Returns**: `Optional[Summary]` - Summary object or None if not found

##### `get_all_summaries() -> List[Summary]`
**Description**: Returns all summaries as defensive copies

**Returns**: `List[Summary]` - List of summary copies

**Performance**: O(n) where n is number of summaries

##### `update_summary(summary_id: str, updates: SummaryUpdate) -> None`
**Description**: Applies partial updates to a summary

**Parameters**:
- `summary_id: str` - UUID identifier
- `updates: SummaryUpdate` - Update instructions

**Raises**: `ValidationError` if summary not found

**Side Effects**:
- Updates modified fields
- Sets updated_at to current UTC time
- Updates tag index if tags are modified

##### `delete_summary(summary_id: str) -> None`
**Description**: Removes a summary from storage

**Parameters**:
- `summary_id: str` - UUID identifier

**Raises**: `ValidationError` if summary not found

**Side Effects**: Removes from both summaries and summary_tags dictionaries

#### Search and Filter Operations

##### `search_summaries(query: str) -> List[Summary]`
**Description**: Case-insensitive search across content, tags, and context

**Parameters**:
- `query: str` - Search string

**Returns**: `List[Summary]` - Matching summaries as defensive copies

**Search Scope**: Content, tags, and optional context field

**Performance**: O(n) linear scan

##### `get_summaries_by_priority(priority: SummaryPriority) -> List[Summary]`
**Description**: Filters summaries by priority level

**Parameters**:
- `priority: SummaryPriority` - Target priority level

**Returns**: `List[Summary]` - Matching summaries

##### `get_summaries_by_tag(tag: str) -> List[Summary]`
**Description**: Case-insensitive tag filtering

**Parameters**:
- `tag: str` - Tag to search for

**Returns**: `List[Summary]` - Summaries containing the tag

##### `get_recent_summaries(limit: int) -> List[Summary]`
**Description**: Returns most recently created summaries

**Parameters**:
- `limit: int` - Maximum number of results

**Returns**: `List[Summary]` - Sorted by created_at descending

##### `get_high_priority_summaries() -> List[Summary]`
**Description**: Returns summaries with High or Critical priority

**Returns**: `List[Summary]` - High priority summaries

#### Analytics Operations

##### `get_all_tags() -> List[str]`
**Description**: Returns all unique tags across all summaries

**Returns**: `List[str]` - Sorted list of unique tags

##### `get_tag_statistics() -> Dict[str, int]`
**Description**: Returns tag usage frequency statistics

**Returns**: `Dict[str, int]` - Tag to count mapping

##### `get_summary_statistics() -> SummaryStatistics`
**Description**: Returns comprehensive summary statistics

**Returns**: `SummaryStatistics` object

### Utility Functions

#### `parse_summary_format(summary_text: str) -> Summary`
**Description**: Parses summary from custom text format

**Format**: `<summary>content; priority; context; tag1,tag2,tag3</summary>`

**Parameters**:
- `summary_text: str` - Formatted text input

**Returns**: `Summary` - Parsed summary object

**Raises**: `ValidationError` for format violations

**Minimum Requirements**: At least content and priority

**Usage**:
```python
text = "<summary>Project done; high; Delivery; project,success</summary>"
summary = parse_summary_format(text)
```

## Technical Constraints and Limitations

### Storage Limitations
- **In-Memory Storage**: Data is not persisted across sessions
- **Memory Usage**: Linear growth with number of summaries
- **No Database**: Lacks persistence layer for long-term storage

### Performance Considerations
- **Search Performance**: O(n) linear scans for search operations
- **Tag Indexing**: Basic tag indexing but no advanced search optimization
- **Copy Operations**: Defensive copying impacts performance for large datasets

### Scalability Constraints
- **Single Instance**: Designed for single-process usage
- **No Concurrency**: Not thread-safe for concurrent access
- **Memory Bound**: Limited by available system memory

## Error Handling and Edge Cases

### Common Error Scenarios
1. **Invalid ID Operations**: `ValidationError` for non-existent IDs
2. **Format Parsing**: Structured validation for text format parsing
3. **Priority Conversion**: `ValueError` for invalid priority strings

### Defensive Programming
- **Null Safety**: Extensive use of `Optional` types
- **Immutable Returns**: Defensive copying prevents external mutation
- **Input Validation**: Comprehensive validation throughout API

## Dependencies and Requirements

### Python Requirements
- **Python Version**: 3.7+ (for `from __future__ import annotations`)
- **Standard Library**: `uuid`, `datetime`, `enum`, `typing`, `dataclasses`

### Type Hints
- Comprehensive type annotations for static analysis
- Compatibility with mypy and other type checkers

## Testing and Validation

### Test Coverage
The module includes comprehensive unit tests covering:
- Manager creation and initialization
- CRUD operations validation
- Search and filter functionality
- Statistical calculations
- Format parsing edge cases

### Test Execution
```python
python todozi.py  # Runs test suite automatically
```

## Usage Examples

### Basic Workflow
```python
# Initialize manager
manager = SummaryManager()

# Create summary
summary = Summary(
    content="Quarterly report completed",
    priority=SummaryPriority.High,
    tags=["report", "quarterly", "finance"]
)
summary_id = manager.create_summary(summary)

# Search and update
results = manager.search_summaries("quarterly")
update = SummaryUpdate().with_content("Updated report content")
manager.update_summary(summary_id, update)

# Analytics
stats = manager.get_summary_statistics()
print(f"High priority percentage: {stats.high_priority_percentage():.1f}%")
```

### Advanced Usage
```python
# Batch operations
summaries = [
    Summary(content=f"Task {i}", priority=SummaryPriority.Medium)
    for i in range(100)
]

for summary in summaries:
    manager.create_summary(summary)

# Complex filtering
high_priority_recent = [
    s for s in manager.get_recent_summaries(10)
    if s.priority in (SummaryPriority.High, SummaryPriority.Critical)
]
```

## Extension Points

### Potential Enhancements
1. **Persistence Layer**: Add database or file storage
2. **Advanced Search**: Implement full-text search capabilities
3. **Concurrency**: Add thread-safe operations
4. **Serialization**: JSON/XML import/export functionality
5. **Validation Framework**: Enhanced input validation rules

This documentation provides comprehensive coverage of the Todozi system's architecture, API, and implementation details suitable for developers and technical stakeholders.