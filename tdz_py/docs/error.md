# Todozi Error Handling Framework - Technical Documentation

## Overview

The Todozi Error Handling Framework provides a comprehensive error management system for Python applications, featuring structured error types, error tracking, and configurable error lifecycle management.

## Architecture and Design Decisions

### Core Design Principles
- **Hierarchical Error Types**: Base exception class with specialized subclasses for different error categories
- **Immutable Error Data**: Error instances contain timestamped, contextual information
- **Thread-Safe Error Management**: Lock-based synchronization for concurrent access
- **Enum-Based Classification**: Strongly typed severity and category classification
- **Serialization Support**: JSON-compatible error representation for logging and persistence

### Module Structure
```
error_framework/
├── enums/           # ErrorSeverity, ErrorCategory
├── exceptions/      # TodoziError and specialized subclasses
├── entities/        # Error dataclass
├── management/      # ErrorManager with configuration
└── parsing/         # Structured error format parsing
```

## Dependencies and Requirements

### Required Dependencies
```python
# Core Python modules
import json
import logging
import re
import uuid
from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum, auto
from threading import Lock
from typing import Any, Dict, List, Optional, Pattern, Tuple, TypedDict, Union
```

### Python Version Compatibility
- Requires Python 3.7+ (for `from __future__ import annotations`)
- Uses modern typing features (TypedDict, Union, Optional)
- Compatible with dataclasses (Python 3.7+)

## Core Components

### 1. ErrorSeverity Enum

#### Description
Defines the severity levels for errors with case-insensitive lookup support.

#### Values
- `LOW`: Non-critical issues
- `MEDIUM`: Issues requiring attention
- `HIGH`: Significant problems affecting functionality  
- `CRITICAL`: System-threatening issues
- `URGENT`: Immediate attention required

#### Methods
```python
@classmethod
def _missing_(cls, value: object) -> Optional["ErrorSeverity"]
```
**Parameters:**
- `value`: String or object to convert to enum

**Returns:** Corresponding ErrorSeverity enum or None if not found

**Example:**
```python
severity = ErrorSeverity("HIGH")  # Returns ErrorSeverity.HIGH
severity = ErrorSeverity("high")  # Case-insensitive match
```

### 2. ErrorCategory Enum

#### Description
Categorizes errors by functional area with case-insensitive lookup.

#### Values
- `NETWORK`: Network connectivity issues
- `VALIDATION`: Data validation failures
- `STORAGE`: Persistence layer errors
- `CONFIGURATION`: Configuration problems
- `API`: External API integration errors
- `EMBEDDING`: Embedding/model-related errors
- `SYSTEM`: System-level errors
- `BUSINESS_LOGIC`: Domain logic violations

### 3. Typed Context Dictionaries

#### Design Rationale
Type-safe context dictionaries provide structured error context while maintaining flexibility through `total=False`.

#### Available Context Types
- `ValidationContext`: Field validation failures
- `StorageContext`: Storage operation details
- `ConfigContext`: Configuration issues
- `ApiContext`: API call metadata
- Domain-specific contexts for business logic errors

### 4. TodoziError Base Exception

#### Class Signature
```python
class TodoziError(Exception):
    def __init__(
        self,
        message: str,
        error_code: str = "UNKNOWN_ERROR",
        context: Optional[Dict[str, Any]] = None,
        cause: Optional[Exception] = None,
    )
```

#### Attributes
- `message`: Human-readable error description
- `error_code`: Machine-readable error identifier
- `context`: Structured error context dictionary
- `timestamp`: UTC timestamp of error creation
- `__cause__`: Original exception for chaining

#### Factory Methods

**Validation Error Factory**
```python
@classmethod
def validation(cls, message: Union[str, ValidationContext]) -> "TodoziError"
```
**Usage:**
```python
# String message
error = TodoziError.validation("Invalid email format")

# Structured context
error = TodoziError.validation({
    "field": "email", 
    "value": "invalid@",
    "constraint": "RFC 5322 format"
})
```

**Other Factory Methods**
- `storage()`: Storage layer errors
- `config()`: Configuration issues  
- `api()`: API integration failures
- `io()`: I/O operations with exception chaining
- `serialization()`: Data serialization/deserialization errors

#### Serialization
```python
def to_dict(self) -> Dict[str, Any]
```
**Returns:** JSON-serializable dictionary representation

**Example Output:**
```json
{
    "error_code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "context": {"field": "email", "value": "invalid@"},
    "timestamp": "2023-01-01T12:00:00Z",
    "type": "TodoziError"
}
```

### 5. Specialized Error Classes

#### Domain-Specific Errors
- `TaskNotFoundError`, `ProjectNotFoundError`, `FeelingNotFoundError`: Entity not found
- `InvalidPriorityError`, `InvalidStatusError`: Enum validation failures
- `InvalidAssigneeError`, `InvalidProgressError`: Business rule violations
- `EmbeddingError`: AI/ML model failures
- `NotImplementedError_`: Unimplemented features

#### Wrapper Errors
Wrap third-party library exceptions into TodoziError hierarchy:
- `IoError`, `JsonError`, `UuidError`
- `ChronoError`, `DialoguerError`, `HlxError`
- `ReqwestError`, `CandleError`

### 6. Error Entity (Dataclass)

#### Structure
```python
@dataclass
class Error:
    id: str                    # UUID identifier
    title: str                 # Brief error title
    description: str          # Detailed description
    severity: ErrorSeverity    # Error severity level
    category: ErrorCategory    # Functional category
    source: str               # Error origin/source
    context: Optional[str]     # Additional context
    tags: List[str]           # Searchable tags
    resolved: bool            # Resolution status
    resolution: Optional[str]  # Resolution description
    created_at: datetime      # Creation timestamp
    updated_at: datetime      # Last update timestamp
    resolved_at: Optional[datetime]  # Resolution timestamp
```

#### Serialization Methods
```python
def to_dict(self) -> Dict[str, Any]
```
Converts Error instance to dictionary with ISO format timestamps.

```python
@classmethod
def from_dict(cls, data: Dict[str, Any]) -> "Error"
```
Robust deserialization with enum parsing and datetime handling.

### 7. ErrorManager Configuration

#### Configuration Options
```python
@dataclass
class ErrorManagerConfig:
    max_errors: int = 10000              # Maximum errors to store
    auto_cleanup_resolved: bool = True   # Automatic cleanup enabled
    cleanup_interval_hours: int = 24     # Cleanup frequency
```

### 8. ErrorManager Class

#### Initialization
```python
def __init__(self, config: Optional[ErrorManagerConfig] = None, 
             logger: Optional[logging.Logger] = None)
```

#### Key Methods

**Error Creation**
```python
def create_error(self, error: Error) -> str
```
**Parameters:**
- `error`: Error instance to store

**Returns:** Error ID (UUID string)

**Behavior:**
- Applies automatic cleanup if needed
- Enforces maximum error limit with LRU eviction
- Generates UUID if not provided
- Sets timestamps automatically
- Logs error via configured logger

**Error Resolution**
```python
def resolve_error(self, error_id: str, resolution: str) -> None
```
**Parameters:**
- `error_id`: UUID of error to resolve
- `resolution`: Resolution description

**Throws:** `TodoziError` if error not found

**Query Methods**
- `get_unresolved_errors() -> List[Error]`: All unresolved errors
- `get_errors_needing_attention() -> List[Error]`: Critical/urgent unresolved errors
- `stats() -> Dict[str, int]`: Error counts by category

**Export Functionality**
```python
def export_errors_json(self, include_resolved: bool = True) -> str
```
Returns JSON array of error dictionaries.

### 9. Error Format Parser

#### Structured Error Format
```xml
<error>
Title; Description; Severity; Category; Source; Context; Tags
</error>
```

**Format Specification:**
- Fields separated by semicolons
- Minimum 5 required fields
- Context and tags optional
- Tags comma-separated

#### Parser Function
```python
def parse_error_format(error_text: str) -> Error
```
**Parameters:**
- `error_text`: String containing `<error>...</error>` tags

**Returns:** Parsed Error instance

**Throws:** `TodoziError` on format violations

## Technical Constraints and Limitations

### Performance Considerations

**Memory Usage**
- ErrorManager stores errors in memory with configurable limit (default: 10,000)
- Each Error instance ~1-2KB depending on context size
- Consider external storage for high-volume error scenarios

**Thread Safety**
- Lock-based synchronization ensures thread safety
- Lock contention may occur with high concurrency
- Consider partitioning for high-throughput systems

**Parsing Performance**
- Regex-based parser suitable for moderate volumes
- For high-frequency parsing, consider compiled regex reuse

### Error Handling Edge Cases

**Enum Parsing Robustness**
- Case-insensitive enum matching
- Graceful handling of invalid enum values
- Default fallback values for missing data

**DateTime Handling**
- Timezone-aware timestamp storage
- Fallback parsing for naive datetime strings
- Robust handling of malformed timestamp formats

**Memory Management**
- Automatic cleanup of resolved errors
- LRU eviction when capacity exceeded
- Configurable cleanup intervals

## Usage Patterns and Examples

### Basic Error Creation
```python
# Create domain-specific error
try:
    if not valid_email(email):
        raise TodoziError.validation({
            "field": "email",
            "value": email,
            "constraint": "valid format"
        })
except TodoziError as e:
    print(f"Error: {e}")
    print(f"Details: {e.to_dict()}")
```

### Error Management Integration
```python
# Initialize error manager
error_mgr = ErrorManager(
    config=ErrorManagerConfig(max_errors=5000),
    logger=logging.getLogger("app.errors")
)

# Create and track error
error = Error(
    title="Database Connection Failed",
    description="Timeout connecting to PostgreSQL",
    severity=ErrorSeverity.CRITICAL,
    category=ErrorCategory.NETWORK,
    source="db-service",
    tags=["database", "postgres", "connection"]
)

error_id = error_mgr.create_error(error)

# Later, resolve the error
error_mgr.resolve_error(error_id, "Network configuration fixed")
```

### Structured Error Parsing
```python
error_text = """
<error>
File Write Failed; 
Cannot write to /var/data/file.txt; 
high; 
storage; 
file-service; 
Permission denied for user 'app'; 
file,permission,storage
</error>
"""

try:
    parsed_error = parse_error_format(error_text)
    error_mgr.create_error(parsed_error)
except TodoziError as e:
    print(f"Parse error: {e}")
```

## Error Recovery Strategies

### Graceful Degradation
```python
def safe_operation():
    try:
        return perform_risky_operation()
    except TodoziError as e:
        # Log error but continue operation
        error_mgr.create_error(Error.from_exception(e))
        return fallback_operation()
```

### Circuit Breaker Pattern
```python
class CircuitBreaker:
    def __init__(self, error_mgr: ErrorManager, threshold: int = 5):
        self.error_mgr = error_mgr
        self.threshold = threshold
        
    def should_open(self, category: ErrorCategory) -> bool:
        stats = self.error_mgr.stats()
        return stats.get(category.value, 0) > self.threshold
```

## Testing and Validation

### Unit Testing Strategy
The framework includes comprehensive tests covering:
- Error parsing and validation
- Serialization/deserialization round-trip
- Thread safety under concurrent access
- Edge cases and error conditions

### Test Coverage Areas
- `test_parse_error_format()`: Format parsing validation
- `test_error_serde()`: Serialization integrity
- `test_manager_stats_and_resolve()`: Manager functionality
- `test_edge_parsing()`: Error condition handling
- `test_thread_safety()`: Concurrency validation

## Integration Guidelines

### Logging Integration
```python
import logging

# Configure structured logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

error_mgr = ErrorManager(logger=logging.getLogger("todozi.errors"))
```

### Monitoring Integration
```python
def get_error_metrics(error_mgr: ErrorManager) -> Dict[str, Any]:
    return {
        "total_errors": len(error_mgr.errors),
        "unresolved_errors": len(error_mgr.get_unresolved_errors()),
        "critical_errors": len(error_mgr.get_errors_needing_attention()),
        "category_breakdown": error_mgr.stats()
    }
```

## Security Considerations

### Error Information Exposure
- Error messages should avoid sensitive data exposure
- Consider context sanitization before logging
- Implement error message filtering for production

### Resource Exhaustion
- Configurable limits prevent memory exhaustion
- Regular cleanup prevents unbounded growth
- Monitor error volume for abnormal patterns

## Future Enhancements

### Potential Extensions
- Database persistence for error storage
- Error aggregation and deduplication
- Error correlation across services
- Real-time error alerting integration
- Error analytics and trend analysis

This framework provides a robust foundation for error handling in Python applications, with extensible architecture suitable for both small-scale and enterprise-level deployments.