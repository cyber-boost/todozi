# Todozi Tag Management System Technical Documentation

## Overview

Todozi is a comprehensive tag management system designed for high-performance applications requiring robust tagging capabilities. The system provides thread-safe operations, advanced search functionality, and comprehensive tag relationship management.

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Data Models](#data-models)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Performance Considerations](#performance-considerations)
6. [Error Handling](#error-handling)
7. [Dependencies and Requirements](#dependencies-and-requirements)
8. [Limitations and Constraints](#limitations-and-constraints)

## Architecture Overview

### System Design
The system follows a layered architecture with clear separation of concerns:
- **Data Layer**: Immutable data models using frozen dataclasses
- **Business Logic Layer**: Thread-safe `TagManager` with indexing capabilities
- **Search Layer**: Advanced search engine with fuzzy matching capabilities
- **Indexing Layer**: Multiple indexes for fast lookup operations

### Key Design Decisions
1. **Immutable Data Models**: Tags are implemented as frozen dataclasses to prevent unintended mutations
2. **Thread Safety**: Uses `asyncio.Lock` for all mutable operations
3. **Index Strategy**: Maintains multiple indexes (name, category, search tokens) for performance
4. **Copy-on-Return**: All public API methods return copies to prevent external mutation
5. **Token-based Search**: Implements inverted index for fast text search

## Data Models

### Tag
```python
@dataclass(frozen=True)
class Tag:
    id: str
    name: str
    description: Optional[str] = None
    color: Optional[str] = None
    category: Optional[str] = None
    usage_count: int = 0
    created_at: datetime = field(default_factory=lambda: datetime.now(dt_timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(dt_timezone.utc))
```

**Properties:**
- `id`: Unique identifier (UUID)
- `name`: Required tag name (unique within system)
- `description`: Optional descriptive text
- `color`: Optional color representation
- `category`: Optional categorization
- `usage_count`: Counter for tag usage
- `created_at`: Creation timestamp (UTC)
- `updated_at`: Last modification timestamp (UTC)

**Methods:**
- `to_dict() -> Dict[str, Any]`: Serializes tag to dictionary
- `from_dict(data: Dict[str, Any]) -> Tag`: Deserializes from dictionary

### TagUpdate
```python
@dataclass
class TagUpdate:
    name: Optional[str] = None
    description: Optional[str] = None
    color: Optional[str] = None
    category: Optional[str] = None
```

**Fluent Builder Methods:**
- `with_name(name: str) -> TagUpdate`
- `with_description(description: str) -> TagUpdate`
- `with_color(color: str) -> TagUpdate`
- `with_category(category: str) -> TagUpdate`

### TagStatistics
```python
@dataclass
class TagStatistics:
    total_tags: int
    total_categories: int
    total_relationships: int
    average_usage: float
```

**Methods:**
- `relationships_per_tag() -> float`: Calculates average relationships per tag

### Enumerations

#### TagSortBy
```python
class TagSortBy(Enum):
    Name = auto()    # Sort by tag name
    Usage = auto()   # Sort by usage count
    Created = auto() # Sort by creation date
    Updated = auto() # Sort by last update date
```

## Core Components

### TagManager

#### Constructor
```python
def __init__(self) -> None
```
Initializes a new TagManager instance with empty storage and indexes.

**Internal State:**
- `tags: Dict[str, Tag]`: Primary tag storage
- `tag_relationships: Dict[str, List[str]]`: Tag relationship mappings
- `category_tags: Dict[str, List[str]]`: Category to tag mappings
- `_name_index: Dict[str, str]`: Lowercase name to tag ID mapping
- `_search_index: Dict[str, Set[str]]`: Token to tag ID set mapping

#### Public API Methods

##### create_tag
```python
async def create_tag(
    self,
    name: str,
    *,
    description: Optional[str] = None,
    color: Optional[str] = None,
    category: Optional[str] = None,
) -> str
```
Creates a new tag with the specified attributes.

**Parameters:**
- `name` (str): Required tag name
- `description` (Optional[str]): Tag description
- `color` (Optional[str]): Color representation
- `category` (Optional[str]): Category assignment

**Returns:**
- `str`: Generated tag ID

**Raises:**
- `ValidationError`: If name is empty or already exists

**Example:**
```python
tag_id = await manager.create_tag(
    name="bug",
    description="Software defect",
    color="#FF0000",
    category="development"
)
```

##### get_tag
```python
def get_tag(self, tag_id: str) -> Optional[Tag]
```
Retrieves a tag by ID.

**Parameters:**
- `tag_id` (str): Tag identifier

**Returns:**
- `Optional[Tag]`: Tag instance or None if not found

**Example:**
```python
tag = manager.get_tag("123e4567-e89b-12d3-a456-426614174000")
```

##### update_tag
```python
async def update_tag(self, tag_id: str, updates: TagUpdate) -> None
```
Updates an existing tag with the specified changes.

**Parameters:**
- `tag_id` (str): Tag identifier
- `updates` (TagUpdate): Update specifications

**Raises:**
- `TagNotFoundError`: If tag does not exist
- `ValidationError`: If name validation fails

**Example:**
```python
await manager.update_tag(
    tag_id,
    TagUpdate().with_name("enhancement").with_category("feature")
)
```

##### delete_tag
```python
async def delete_tag(self, tag_id: str) -> None
```
Deletes a tag and all its relationships.

**Parameters:**
- `tag_id` (str): Tag identifier

**Raises:**
- `TagNotFoundError`: If tag does not exist

##### search_tags
```python
def search_tags(self, query: str) -> List[Tag]
```
Searches tags by name and description using token-based indexing.

**Parameters:**
- `query` (str): Search query string

**Returns:**
- `List[Tag]`: Matching tags

**Search Behavior:**
- Uses token-based inverted index for performance
- Falls back to linear scan for non-token queries
- Case-insensitive substring matching

##### bulk_create_tags
```python
async def bulk_create_tags(
    self,
    tag_names: Union[List[str], Tuple[str, ...]],
    category: Optional[str] = None,
) -> List[str]
```
Creates multiple tags in a single atomic operation.

**Parameters:**
- `tag_names`: List or tuple of tag names
- `category`: Optional category for all tags

**Returns:**
- `List[str]`: List of created tag IDs

##### merge_tags
```python
async def merge_tags(self, primary_tag_id: str, duplicate_tag_ids: List[str]) -> None
```
Merges duplicate tags into a primary tag.

**Parameters:**
- `primary_tag_id` (str): Target tag for merging
- `duplicate_tag_ids` (List[str]): Tags to merge into primary

**Behavior:**
- Accumulates usage counts from duplicates
- Migrates relationships to primary tag
- Removes duplicate tags from system

### TagSearchEngine

#### Constructor
```python
def __init__(self, tag_manager: TagManager) -> None
```
Initializes search engine with reference to tag manager.

#### Public Methods

##### advanced_search
```python
def advanced_search(self, query: TagSearchQuery) -> List[Tag]
```
Performs complex tag searches with multiple filtering options.

**TagSearchQuery Parameters:**
- `name_contains`: Substring match in tag name
- `description_contains`: Substring match in description
- `category`: Exact category match
- `color`: Exact color match
- `min_usage`: Minimum usage count
- `max_usage`: Maximum usage count
- `sort_by`: Sorting criteria
- `limit`: Maximum results to return

##### fuzzy_search
```python
def fuzzy_search(self, query: str, max_distance: int) -> List[Tuple[Tag, int]]
```
Performs fuzzy string matching using Levenshtein distance.

**Parameters:**
- `query` (str): Search string
- `max_distance` (int): Maximum allowed edit distance

**Returns:**
- `List[Tuple[Tag, int]]`: Tags with their edit distances

##### get_suggestions
```python
def get_suggestions(self, current_tags: List[str], limit: int) -> List[str]
```
Suggests related tags based on relationship frequency.

**Parameters:**
- `current_tags`: List of existing tag names
- `limit`: Maximum suggestions to return

## Performance Considerations

### Indexing Strategy
The system maintains multiple indexes for optimal performance:

1. **Name Index**: O(1) lookup by name
2. **Category Index**: O(1) category-based grouping
3. **Search Index**: Token-based inverted index for text search
4. **Relationship Index**: Efficient relationship traversal

### Memory Usage
- **Primary Storage**: O(n) where n is number of tags
- **Indexes**: Additional O(n) memory for indexes
- **Token Index**: Memory usage depends on token diversity

### Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Tag Creation | O(k) | k = number of tokens in name/description |
| Tag Lookup | O(1) | Using hash-based indexes |
| Text Search | O(m) | m = number of matching tags (indexed) |
| Bulk Operations | O(n) | Optimized with single lock acquisition |

### Concurrency
- All mutable operations are protected by `asyncio.Lock`
- Read operations are lock-free for better performance
- Bulk operations minimize lock acquisition overhead

## Error Handling

### Exception Hierarchy
```
TodoziError
├── TagNotFoundError
└── ValidationError
```

### Common Error Scenarios

#### Validation Errors
- Empty tag names
- Duplicate tag names
- Self-referential relationships
- Invalid update parameters

#### Resource Errors
- Non-existent tag references
- Missing related tags
- Invalid tag IDs

### Error Recovery
- Atomic operations prevent partial state updates
- Index consistency maintained through careful update sequences
- Relationship integrity preserved during deletions

## Dependencies and Requirements

### Python Version
- **Minimum**: Python 3.7+ (for `from __future__ import annotations`)
- **Recommended**: Python 3.9+ (for better type hint support)

### Core Dependencies
```python
import asyncio    # Asynchronous operations
import re         # Regular expressions for tokenization
import uuid       # Unique identifier generation
from dataclasses import dataclass, field, replace  # Data modeling
from datetime import datetime, timezone as dt_timezone  # Timestamp handling
from enum import Enum, auto  # Enumerations
from typing import Any, Dict, List, Optional, Set, Tuple, Union  # Type hints
```

### Optional Dependencies
- None required; the system is self-contained

## Limitations and Constraints

### Functional Limitations
1. **Maximum Tag Count**: Limited by available memory
2. **Token Length**: Tokenization uses `[A-Za-z0-9_]` pattern only
3. **Search Precision**: Fuzzy search uses basic Levenshtein distance
4. **Category Depth**: Single-level categorization only

### Technical Constraints
1. **Memory Usage**: All data stored in memory (no persistence layer)
2. **Concurrency**: Limited to single-process asyncio operations
3. **Scalability**: Designed for moderate-sized tag collections (<100K tags)
4. **Persistence**: Requires external serialization for data persistence

### Performance Boundaries
- **Optimal Range**: 1,000 - 10,000 tags
- **Token Index**: Becomes memory-intensive with highly diverse text
- **Search Performance**: Linear scan fallback for non-token queries

## Usage Examples

### Basic Tag Management
```python
import asyncio
from todazi import TagManager, TagUpdate

async def main():
    manager = TagManager()
    
    # Create tags
    bug_id = await manager.create_tag("bug", category="issues")
    feature_id = await manager.create_tag("feature", category="enhancements")
    
    # Establish relationships
    await manager.add_tag_relationship(bug_id, feature_id)
    
    # Update tag
    await manager.update_tag(bug_id, TagUpdate().with_color("#FF0000"))
    
    # Search tags
    results = manager.search_tags("bug")
    print(f"Found {len(results)} matching tags")

asyncio.run(main())
```

### Advanced Search
```python
from todazi import TagSearchEngine, TagSearchQuery, TagSortBy

# Initialize search engine
engine = TagSearchEngine(manager)

# Complex search query
query = TagSearchQuery(
    name_contains="email",
    category="infra",
    min_usage=5,
    sort_by=TagSortBy.Usage,
    limit=10
)

results = engine.advanced_search(query)
```

### Bulk Operations
```python
# Bulk tag creation
tag_names = ["urgent", "critical", "blocked"]
tag_ids = await manager.bulk_create_tags(tag_names, category="priority")

# Tag merging
await manager.merge_tags(primary_tag_id, ["duplicate1", "duplicate2"])
```

This documentation provides comprehensive coverage of the Todozi tag management system. For implementation-specific details or advanced usage patterns, refer to the inline code comments and test examples provided in the source code.