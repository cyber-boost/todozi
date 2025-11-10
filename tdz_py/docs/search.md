# Search Engine Technical Documentation

## Overview

This document provides comprehensive technical documentation for the Search Engine module, an in-memory search system designed to index and search across multiple domain entities including tasks, memories, ideas, errors, and training data.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Domain Models](#domain-models)
3. [Search Protocol and Options](#search-protocol-and-options)
4. [Search Engine Implementation](#search-engine-implementation)
5. [Performance Considerations](#performance-considerations)
6. [Error Handling and Edge Cases](#error-handling-and-edge-cases)
7. [Dependencies and Requirements](#dependencies-and-requirements)

## Architecture Overview

### System Design

The search engine follows a domain-driven design approach with clear separation between:
- **Domain Entities**: Core data models representing business objects
- **Search Protocol**: Interface definition for searchable content
- **Search Engine**: Implementation of search and indexing functionality
- **Result Types**: Structured output containers for search results

### Design Decisions

1. **In-Memory Storage**: Chosen for simplicity and performance in moderate-scale applications
2. **Protocol-Based Design**: `ChatContent` protocol allows flexible integration with different data sources
3. **Comprehensive Enum System**: Type-safe categorization across all domain entities
4. **Scoring System**: Configurable relevance scoring with length normalization
5. **Flexible Filtering**: Support for time-based, type-based, and advanced structured filtering

## Domain Models

### Enum Definitions

#### Status Enum
```python
class Status(Enum):
    TODO = "TODO"
    IN_PROGRESS = "IN_PROGRESS"
    DONE = "DONE"
```
**Purpose**: Represents task lifecycle states
**Usage**: Task status tracking and filtering

#### Priority Enum
```python
class Priority(Enum):
    LOW = "LOW"
    MEDIUM = "MEDIUM"
    HIGH = "HIGH"
    CRITICAL = "CRITICAL"
```
**Purpose**: Task urgency classification
**Usage**: Prioritization and advanced search filtering

#### Domain Entity Enums
- `MemoryImportance`: Importance classification for memories
- `MemoryTerm`: Temporal classification (SHORT_TERM/LONG_TERM)
- `IdeaImportance`: Significance level for ideas
- `ShareLevel`: Visibility scope (PRIVATE/TEAM/PUBLIC)
- `ErrorSeverity`: Impact level for errors
- `ErrorCategory`: Error type classification

### Data Classes

#### Task Model
```python
@dataclass
class Task:
    action: str
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    status: Optional[Status] = None
    priority: Optional[Priority] = None
    assignee: Optional[Assignee] = None
    id: str = ""
```

**Parameters**:
- `action`: Required string describing the task
- `tags`: Optional categorization labels
- `created_at`: Automatic UTC timestamp
- `status`: Current progress state
- `priority`: Urgency classification
- `assignee`: Responsible party identifier
- `id`: Unique identifier (empty by default)

#### Memory Model
```python
@dataclass
class Memory:
    moment: str
    meaning: str
    reason: str
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    importance: Optional[MemoryImportance] = None
    term: Optional[MemoryTerm] = None
    id: str = ""
```

**Key Fields**:
- `moment`: Descriptive context of the memory
- `meaning`: Significance interpretation
- `reason`: Rationale for retention

#### Other Domain Models
- `Idea`: Creative concepts with sharing controls
- `Error`: Problem tracking with resolution status
- `TrainingData`: Machine learning training examples

## Search Protocol and Options

### ChatContent Protocol
```python
class ChatContent(Protocol):
    tasks: Sequence[Task]
    memories: Sequence[Memory]
    ideas: Sequence[Idea]
    errors: Sequence[Error]
    training_data: Sequence[TrainingData]
```

**Purpose**: Defines the interface for searchable content sources
**Implementation**: Any class providing sequences of domain entities can be indexed

### SearchOptions
```python
@dataclass
class SearchOptions:
    data_types: Optional[List[SearchDataType]] = None
    since: Optional[datetime] = None
    until: Optional[datetime] = None
    limit: Optional[int] = None
    page: Optional[int] = None
    page_size: Optional[int] = None
```

**Default Configuration**:
```python
@staticmethod
def default() -> SearchOptions:
    return SearchOptions(limit=50)
```

**Usage Examples**:
```python
# Search only tasks and errors with time constraints
options = SearchOptions(
    data_types=[SearchDataType.TASKS, SearchDataType.ERRORS],
    since=datetime(2024, 1, 1),
    limit=20
)
```

### Search Results Structure

#### Individual Result Types
```python
@dataclass
class TaskResult:
    task: Task
    score: float
```

**Scoring**: Relevance score between 0.0 and theoretical maximum (~2.2 with current weights)

#### Composite Results
```python
@dataclass
class SearchResults:
    task_results: List[TaskResult] = field(default_factory=list)
    memory_results: List[MemoryResult] = field(default_factory=list)
    # ... other result types
    
    def total_results(self) -> int:
        return sum(len(getattr(self, attr)) for attr in [
            'task_results', 'memory_results', 'idea_results', 
            'error_results', 'training_results'
        ])
```

## Search Engine Implementation

### Core Search Engine Class

#### Initialization
```python
class SearchEngine:
    # Configuration constants
    SCORE_EXACT_MATCH: float = 1.0
    SCORE_WORD_MATCH: float = 0.7
    SCORE_TAG_MATCH: float = 0.5
    MIN_KEYWORD_LENGTH: int = 3
    LENGTH_PENALTY_BASE: float = 100.0
    
    def __init__(self) -> None:
        self.tasks: List[Task] = []
        self.memories: List[Memory] = []
        # ... other collections
```

#### Index Update Method
```python
def update_index(self, content: ChatContent) -> None:
    """
    Merge a ChatContent payload into the search index.
    
    Args:
        content: An object implementing the ChatContent protocol
        
    Raises:
        TypeError: If content does not match the ChatContent protocol
                   or its fields are not sequences as expected.
    """
```

**Usage Example**:
```python
engine = SearchEngine()
content = SimpleChatContent(tasks=[task1, task2], memories=[memory1])
engine.update_index(content)
```

#### Main Search Method
```python
def search(self, query: str, options: SearchOptions) -> SearchResults:
    """
    Perform a keyword-based search across all indexed content types.
    
    Args:
        query: Search query string
        options: SearchOptions including filters, limit, and pagination
        
    Returns:
        SearchResults object containing matching items sorted by relevance
    """
```

**Search Process**:
1. Query normalization to lowercase
2. Parallel matching across all content types
3. Relevance scoring calculation
4. Type-based filtering
5. Time-range filtering
6. Sorting by relevance score
7. Pagination application

### Relevance Scoring Algorithm

#### Scoring Logic
```python
def _calculate_relevance_score(self, query_lower: str, text: Optional[str], tags: List[str]) -> float:
    """
    Compute relevance score using weighted matching strategy.
    
    Scoring Components:
    - Exact match: +1.0
    - Word-level match: +0.7 per matching word
    - Tag match: +0.5 per matching tag
    - Length penalty: Normalize for text length
    """
```

**Example Calculation**:
- Query: "urgent task"
- Text: "Complete urgent task by tomorrow"
- Tags: ["urgent", "work"]
- Score: 1.0 (exact match) + 0.7 ("urgent") + 0.7 ("task") + 0.5 ("urgent" tag) = 2.9
- Length penalty: 2.9 * (100 / 35) ≈ 0.83 (normalized)

### Advanced Search Capabilities

#### Structured Search Criteria
```python
@dataclass
class AdvancedSearchCriteria:
    task_criteria: TaskSearchCriteria = field(default_factory=TaskSearchCriteria)
    memory_criteria: MemorySearchCriteria = field(default_factory=MemorySearchCriteria)
    # ... other criteria types
```

**Advanced Search Example**:
```python
criteria = AdvancedSearchCriteria(
    task_criteria=TaskSearchCriteria(
        status=Status.IN_PROGRESS,
        priority=Priority.HIGH,
        required_tag="urgent"
    )
)
results = engine.advanced_search(criteria)
```

## Performance Considerations

### Time Complexity
- **Index Update**: O(n) where n is total items across all content types
- **Basic Search**: O(m × k) where m is total indexed items and k is query complexity
- **Advanced Search**: O(p + q) where p is target entity count and q is criteria complexity

### Memory Usage
- **Storage**: All domain entities stored in memory
- **Scaling**: Suitable for thousands of items; consider external indexing for larger datasets
- **Optimizations**: In-place filtering to minimize temporary allocations

### Search Optimization Techniques

#### Query Matching Optimization
```python
def _matches_query(self, query_lower: str, primary_text: Optional[str], 
                  secondary_text: Optional[str], tags: List[str]) -> bool:
    """
    Optimized matching with single lower() call and early termination
    """
```

#### Time Filtering Efficiency
```python
def _apply_time_filters(self, results: SearchResults, since: Optional[datetime], 
                       until: Optional[datetime]) -> None:
    """
    In-place filtering to avoid list recreation
    """
```

## Error Handling and Edge Cases

### Input Validation
```python
def update_index(self, content: ChatContent) -> None:
    if not hasattr(content, "tasks") or not isinstance(content.tasks, Sequence):
        raise TypeError("ChatContent.tasks must be a Sequence[Task]")
    # ... similar validation for other fields
```

### Edge Case Handling

#### Timezone Handling
```python
def _apply_time_filters(self, results: SearchResults, since: Optional[datetime], 
                       until: Optional[datetime]) -> None:
    # Ensure timezone-awareness for comparison
    if ts.tzinfo is None:
        ts = ts.replace(tzinfo=timezone.utc)
```

#### Empty Query Handling
- Empty queries return empty results
- Short queries (< MIN_KEYWORD_LENGTH) are processed but may have limited matches

#### Pagination Edge Cases
```python
def _apply_pagination(self, results: SearchResults, page: int, page_size: int) -> None:
    start = (max(page, 1) - 1) * page_size  # Handle page < 1
    end = start + page_size
```

## Dependencies and Requirements

### Python Requirements
- **Python Version**: 3.7+ (for `from __future__ import annotations`)
- **Standard Library Dependencies**:
  - `re`: Regular expressions for keyword extraction
  - `collections.defaultdict`: Efficient frequency counting
  - `dataclasses`: Data model definitions
  - `datetime`: Timestamp handling
  - `enum`: Type-safe enumerations
  - `typing`: Type hints and protocols

### Type Safety
- Comprehensive type hints throughout
- Protocol-based interface definitions
- Enum-based domain constraints

## Usage Examples

### Basic Search Usage
```python
# Initialize engine and index content
engine = SearchEngine()
content = SimpleChatContent(tasks=[Task(action="Fix critical bug")])
engine.update_index(content)

# Perform search
results = engine.search("critical", SearchOptions.default())
print(f"Found {results.total_results()} matches")
```

### Advanced Search Usage
```python
# Structured search for high-priority in-progress tasks
criteria = AdvancedSearchCriteria(
    task_criteria=TaskSearchCriteria(
        status=Status.IN_PROGRESS,
        priority=Priority.HIGH
    )
)
results = engine.advanced_search(criteria)
```

### Analytics and Suggestions
```python
# Get index statistics
analytics = engine.get_search_analytics()
print(f"Total indexed items: {analytics.total_indexed_items}")

# Get search suggestions
suggestions = engine.get_search_suggestions("bug", 5)
print(f"Suggestions: {suggestions}")
```

## Testing and Validation

### Test Coverage
The module includes comprehensive tests covering:
- Engine initialization and basic functionality
- Search result structure validation
- Time filtering accuracy
- Pagination correctness
- Keyword extraction reliability

### Running Tests
```python
if __name__ == "__main__":
    # Execute all test functions
    test_search_engine_creation()
    test_search_results()
    # ... other tests
    print("All tests passed.")
```

## Limitations and Future Enhancements

### Current Limitations
1. **Memory-Based Storage**: Not suitable for very large datasets
2. **Simple Scoring**: Basic TF-like scoring without advanced NLP
3. **Limited Advanced Search**: Currently supports only tasks and memories
4. **No Persistence**: Index lost on application restart

### Enhancement Opportunities
1. **External Indexing**: Integration with Elasticsearch or similar
2. **Advanced NLP**: Synonyms, stemming, fuzzy matching
3. **Persistence Layer**: Database-backed storage
4. **Real-time Updates**: Incremental index updates
5. **Performance Monitoring**: Search analytics and performance tracking

This documentation provides comprehensive coverage of the search engine module's architecture, implementation, and usage patterns. The system is designed for flexibility and extensibility while maintaining performance for moderate-scale applications.