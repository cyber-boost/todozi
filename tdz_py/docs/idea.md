# Todozi Technical Documentation

## Overview

Todozi is a Python library for managing ideas with rich metadata, search capabilities, and statistical analysis. The system provides thread-safe operations, comprehensive validation, and flexible data models for idea management applications.

## Architecture and Design Decisions

### Core Design Principles
- **Immutability**: Data classes are frozen to prevent accidental mutations
- **Thread Safety**: All manager operations use reentrant locks for concurrent access
- **Type Safety**: Extensive type hints using Python's typing system
- **Validation**: Comprehensive input validation with descriptive error messages
- **Memory Efficiency**: Generator-based search for large datasets

### Data Flow Architecture
```
Input → Validation → Processing → Storage → Output
    ↓
Error Handling ← Monitoring ← Statistics
```

## Dependencies and Requirements

### Runtime Dependencies
- Python 3.7+ (for `from __future__ import annotations`)
- Standard library modules:
  - `threading` (thread safety)
  - `uuid` (unique identifier generation)
  - `dataclasses` (data modeling)
  - `datetime` (timestamp management)
  - `enum` (type-safe enumerations)

### Type Checking Dependencies
- `typing` module for static type analysis
- TYPE_CHECKING guard for runtime optimization

## Data Models

### Enumerations

#### `ShareLevel`
Defines visibility levels for ideas:
- `Public`: Visible to all users
- `Team`: Visible to team members only  
- `Private`: Visible only to the creator

#### `IdeaImportance`
Prioritization levels for ideas:
- `Low`: Low priority ideas
- `Medium`: Standard priority (default)
- `High`: High priority ideas
- `Critical`: Time-sensitive critical ideas
- `Breakthrough`: Game-changing breakthrough ideas

#### `ItemStatus`
Lifecycle states for ideas:
- `Active`: Currently active ideas
- `Archived`: Archived/inactive ideas

### Core Data Classes

#### `Idea` (frozen dataclass)
The primary data model representing an idea with comprehensive metadata.

**Fields:**
- `id` (str): Unique identifier (UUID format)
- `idea` (str): The idea content/text (required)
- `project_id` (Optional[str]): Associated project identifier
- `status` (ItemStatus): Current status (default: Active)
- `share` (ShareLevel): Visibility level (default: Private)
- `importance` (IdeaImportance): Priority level (default: Medium)
- `tags` (FrozenSet[str]): Immutable set of categorization tags
- `context` (Optional[str]): Additional context or notes
- `created_at` (datetime): Creation timestamp (auto-generated)
- `updated_at` (datetime): Last modification timestamp (auto-generated)

**Immutability Considerations:**
- Frozen dataclass prevents accidental mutations
- All modifications require creating new instances via `replace()`
- Thread-safe for read operations without locking

#### `IdeaUpdate` (mutable dataclass)
Builder pattern for partial idea updates with fluid interface.

**Fields (all optional):**
- `idea` (Optional[str]): Updated idea text
- `share` (Optional[ShareLevel]): Updated visibility level
- `importance` (Optional[IdeaImportance]): Updated priority
- `tags` (Optional[List[str]]): Updated tag list (normalized)
- `context` (Optional[str]): Updated context

**Builder Methods:**
- `with_idea(idea: str) → IdeaUpdate`: Set idea text
- `with_share(share: ShareLevel) → IdeaUpdate`: Set share level
- `with_importance(importance: IdeaImportance) → IdeaUpdate`: Set importance
- `with_tags(tags: List[str]) → IdeaUpdate`: Set tags (with normalization)
- `with_context(context: str) → IdeaUpdate`: Set context

**Usage Pattern:**
```python
update = (IdeaUpdate()
          .with_idea("Updated idea")
          .with_share(ShareLevel.Public)
          .with_tags(["new", "tags"]))
```

#### `IdeaStatistics` (frozen dataclass)
Analytical data structure for idea collection metrics.

**Fields:**
- `total_ideas` (int): Total number of ideas
- `public_ideas` (int): Count of public ideas
- `team_ideas` (int): Count of team-shared ideas
- `private_ideas` (int): Count of private ideas
- `breakthrough_ideas` (int): Count of breakthrough ideas
- `unique_tags` (int): Count of distinct tags

**Analytical Methods:**
- `public_percentage() → float`: Percentage of public ideas
- `team_percentage() → float`: Percentage of team ideas
- `private_percentage() → float`: Percentage of private ideas
- `breakthrough_percentage() → float`: Percentage of breakthrough ideas

**Edge Case Handling:** Returns 0.0 when total_ideas is 0 to avoid division by zero.

## Core Manager Class

### `IdeaManager`
Thread-safe manager for idea lifecycle operations with comprehensive search and analytics.

#### Initialization
```python
def __init__(self) → None
```
Creates empty idea storage with thread synchronization.

**Internal State:**
- `_lock` (threading.RLock): Reentrant lock for thread safety
- `ideas` (Dict[str, Idea]): Primary idea storage by ID
- `idea_tags` (Dict[str, List[str]]): Tag index for efficient searching

#### Core Operations

##### `create_idea(idea: Idea) → str`
Creates a new idea with auto-generated metadata.

**Parameters:**
- `idea` (Idea): Idea instance (ID field ignored)

**Returns:**
- `str`: Generated UUID for the new idea

**Validation:**
- Raises `TypeError` if input is not an Idea instance
- Normalizes tags (strips whitespace, filters empty strings)

**Internal Processing:**
- Generates new UUID and current timestamp
- Creates immutable copy with normalized data
- Updates both primary storage and tag index

##### `get_idea(idea_id: str) → Optional[Idea]`
Retrieves an idea by ID.

**Parameters:**
- `idea_id` (str): Unique identifier

**Returns:**
- `Optional[Idea]`: Copy of the idea or None if not found

**Thread Safety:** Uses read lock for concurrent access.

##### `update_idea(idea_id: str, updates: IdeaUpdate) → None`
Applies partial updates to an existing idea.

**Parameters:**
- `idea_id` (str): Target idea identifier
- `updates` (IdeaUpdate): Changes to apply

**Validation:**
- Raises `TypeError` if updates is not IdeaUpdate instance
- Raises `ValidationError` if idea_id doesn't exist

**Update Logic:**
- Applies each non-None field from updates
- Updates timestamp automatically
- Maintains tag index consistency

##### `delete_idea(idea_id: str) → None`
Removes an idea from storage.

**Parameters:**
- `idea_id` (str): Identifier of idea to delete

**Validation:**
- Raises `ValidationError` if idea_id doesn't exist

**Cleanup:** Removes from both primary storage and tag index.

#### Search and Filtering Operations

##### `search_ideas(query: str) → Iterator[Idea]`
Case-insensitive substring search across idea content.

**Parameters:**
- `query` (str): Search term

**Returns:**
- `Iterator[Idea]`: Generator yielding matching ideas

**Search Scope:**
- Idea text content
- Tags (individual tag matching)
- Context field (if present)

**Performance:** Generator-based for memory efficiency with large datasets.

##### Filter Methods
- `get_ideas_by_importance(importance: IdeaImportance) → List[Idea]`
- `get_ideas_by_share_level(share_level: ShareLevel) → List[Idea]`
- `get_ideas_by_tag(tag: str) → List[Idea]` (case-insensitive)
- `get_public_ideas() → List[Idea]` (convenience wrapper)
- `get_team_ideas() → List[Idea]`
- `get_private_ideas() → List[Idea]`
- `get_breakthrough_ideas() → List[Idea]`

##### `get_recent_ideas(limit: int) → List[Idea]`
Retrieves most recently created ideas.

**Parameters:**
- `limit` (int): Maximum number of ideas to return

**Edge Case:** Returns empty list if limit ≤ 0

#### Analytics Operations

##### `get_all_tags() → List[str]`
Returns sorted list of all unique tags across all ideas.

##### `get_tag_statistics() → Dict[str, int]`
Returns mapping of tag names to usage counts.

##### `get_idea_statistics() → IdeaStatistics`
Comprehensive analytics for the entire idea collection.

## Parser Module

### `parse_idea_format(idea_text: str) → Idea`
Parses ideas from custom string format.

**Format Specification:**
```
<idea>idea text; share; importance; tags; context</idea>
```

**Field Definitions:**
- `idea text`: The idea content (required)
- `share`: Visibility level (see mapping below)
- `importance`: Importance level (must match IdeaImportance)
- `tags`: Comma-separated list (optional)
- `context`: Additional context (optional)

**Share Level Mapping:**
- "share" → `ShareLevel.Public`
- "dont share"/"don't share"/"private" → `ShareLevel.Private`
- "team" → `ShareLevel.Team`

**Validation:**
- Raises `ValidationError` for missing tags or malformed content
- Validates importance against enum values

**Auto-generation:**
- Creates UUID for ID field
- Sets current UTC timestamp for created_at/updated_at
- Defaults project_id to None, status to Active

## Error Handling

### Exception Hierarchy
```
TodoziError (base)
    └── ValidationError (data validation failures)
```

### `TodoziError`
Base exception class for module-specific errors.

### `ValidationError`
Raised for data validation failures with descriptive messages.

**Common Causes:**
- Missing required format tags
- Invalid importance levels
- Non-existent idea IDs in operations
- Malformed input data

## Performance Considerations

### Memory Management
- **Generator-based Search**: `search_ideas()` uses generators to avoid loading all matches into memory
- **Frozen Data**: Immutable objects enable safe caching and sharing
- **Tag Index**: Separate tag storage enables efficient tag-based queries

### Thread Safety
- **Reentrant Locks**: `RLock` allows same-thread reentry for nested operations
- **Copy-on-Read**: All retrieval methods return copies to prevent external mutations
- **Atomic Operations**: Critical sections are properly locked for consistency

### Time Complexity
- **Create/Update/Delete**: O(1) with dictionary operations
- **Search Operations**: O(n) linear scans (consider indexing for large datasets)
- **Statistical Operations**: O(n) for comprehensive metrics

## Technical Constraints and Limitations

### Storage Limitations
- **In-Memory Only**: No persistent storage implementation
- **Scalability**: Linear search complexity may impact large datasets
- **Memory Usage**: Entire dataset must fit in memory

### Functional Limitations
- **No Persistence**: Data lost on application restart
- **No User Management**: Share levels are metadata only
- **No Project Hierarchy**: Flat project_id association only
- **No Versioning**: Updates overwrite previous state

### API Limitations
- **Batch Operations**: No support for bulk create/update/delete
- **Pagination**: Large result sets returned in full
- **Advanced Queries**: Limited to basic substring and field filtering

## Usage Examples

### Basic Idea Management
```python
manager = IdeaManager()

# Create an idea
idea = Idea(
    id="",  # Ignored, auto-generated
    idea="Implement feature X",
    share=ShareLevel.Team,
    importance=IdeaImportance.High,
    tags=["feature", "backend"]
)
idea_id = manager.create_idea(idea)

# Update context
update = IdeaUpdate().with_context("Required for Q3 launch")
manager.update_idea(idea_id, update)

# Search and display
results = list(manager.search_ideas("feature"))
stats = manager.get_idea_statistics()
```

### Parser Integration
```python
# Parse from custom format
idea_text = """<idea>Optimize database queries; team; high; performance,database; 
               This will improve page load times</idea>"""
parsed_idea = parse_idea_format(idea_text)

# Add to manager
manager.create_idea(parsed_idea)
```

### Statistical Analysis
```python
stats = manager.get_idea_statistics()
print(f"Public ideas: {stats.public_ideas} ({stats.public_percentage():.1f}%)")
print(f"Breakthrough ideas: {stats.breakthrough_ideas}")

tag_stats = manager.get_tag_statistics()
popular_tags = sorted(tag_stats.items(), key=lambda x: x[1], reverse=True)[:5]
```

## Testing Strategy

The module includes comprehensive unit tests covering:
- Manager initialization and basic operations
- Builder pattern functionality
- Statistical calculations (including edge cases)
- Parser functionality with various input formats
- Error handling and validation

**Test Coverage Areas:**
- Normal operation paths
- Edge cases (empty collections, zero divisions)
- Error conditions (invalid inputs, missing data)
- Thread safety (concurrent access patterns)

## Future Enhancements

### Recommended Improvements
1. **Persistence Layer**: Add database or file-based storage
2. **Advanced Search**: Implement full-text search with indexing
3. **Batch Operations**: Add bulk create/update/delete methods
4. **Pagination**: Implement limit/offset for large result sets
5. **Export Formats**: Add JSON, CSV, and other export options
6. **Event System**: Add hooks for idea lifecycle events

### Performance Optimizations
- Implement search indexes for large datasets
- Add caching for frequent statistical queries
- Consider async operations for I/O-bound scenarios

This documentation provides comprehensive coverage of the Todozi module's architecture, API, and implementation details for developers and technical stakeholders.