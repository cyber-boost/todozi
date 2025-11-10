# Todozi - Technical Documentation

## Overview

Todozi is a Python-based reminder management system designed for handling time-sensitive reminders with tagging, prioritization, and status tracking capabilities. The system provides both in-memory and persistent storage options, along with advanced features for searching, categorization, and batch operations.

## Table of Contents

1. [Architecture and Design](#architecture-and-design)
2. [Core Components](#core-components)
3. [API Reference](#api-reference)
4. [Usage Patterns](#usage-patterns)
5. [Error Handling](#error-handling)
6. [Performance Considerations](#performance-considerations)
7. [Technical Constraints](#technical-constraints)
8. [Dependencies](#dependencies)

## Architecture and Design

### Design Philosophy

- **Type Safety**: Extensive use of Python type hints for robust development
- **Immutability**: Data classes provide clean data structures with controlled mutation
- **Error Prevention**: Validation functions prevent invalid state creation
- **Extensibility**: Modular design allows for feature enhancement through inheritance
- **Observability**: Event listener system for monitoring state changes

### Core Patterns

- **Builder Pattern**: `ReminderUpdate` class for ergonomic updates
- **Repository Pattern**: `ReminderManager` as the central data access layer
- **Observer Pattern**: Listener system for state change notifications
- **Strategy Pattern**: Multiple manager implementations for different storage needs

## Core Components

### 1. Error Handling

#### `TodoziError`
```python
class TodoziError(Exception):
    def __init__(self, message: str) -> None
```
- **Purpose**: Base exception for all Todozi-specific errors
- **Parameters**: `message` - Detailed error description
- **Usage**: Raised for validation failures and operational errors

#### Validation Functions

##### `_ensure_utc(dt: datetime) -> datetime`
- **Purpose**: Validates and converts datetime objects to UTC timezone
- **Parameters**: `dt` - Input datetime object
- **Returns**: UTC timezone-aware datetime
- **Throws**: `TodoziError` for naive (timezone-unaware) datetimes
- **Usage**: Ensures all timestamps are properly timezone-normalized

##### `_require_non_empty_content(content: str) -> None`
- **Purpose**: Validates reminder content is non-empty
- **Parameters**: `content` - Reminder text content
- **Throws**: `TodoziError` for empty or whitespace-only content
- **Usage**: Prevents creation of reminders with no meaningful content

### 2. Data Models

#### `ReminderPriority` Enum
```python
class ReminderPriority(Enum):
    Low = "low"
    Medium = "medium"
    High = "high"
```
- **Purpose**: Defines reminder priority levels
- **Values**: Three-tier priority system for task management

#### `ReminderStatus` Enum
```python
class ReminderStatus(Enum):
    Pending = "pending"
    Active = "active"
    Completed = "completed"
    Cancelled = "cancelled"
```
- **Purpose**: Tracks reminder lifecycle states
- **State Transitions**: Pending → Active → Completed/Cancelled

#### `Reminder` Data Class
```python
@dataclass
class Reminder:
    id: str
    content: str
    remind_at: datetime
    priority: ReminderPriority
    status: ReminderStatus
    tags: List[str]
    created_at: datetime
    updated_at: datetime
```

**Fields:**
- `id`: Unique identifier (UUID4)
- `content`: Descriptive text (validated as non-empty)
- `remind_at`: UTC datetime for reminder activation
- `priority`: Importance level
- `status`: Current state in lifecycle
- `tags`: Categorization labels
- `created_at`: UTC creation timestamp
- `updated_at`: UTC last modification timestamp

**Methods:**

##### `mark_completed() -> None`
- **Purpose**: Transitions reminder to completed state
- **Effects**: Updates status and modification timestamp
- **Idempotent**: No effect if already completed

##### `mark_cancelled() -> None`
- **Purpose**: Transitions reminder to cancelled state
- **Effects**: Updates status and modification timestamp
- **Idempotent**: No effect if already cancelled

##### `activate() -> None`
- **Purpose**: Activates a pending reminder
- **Effects**: Updates status and modification timestamp
- **Precondition**: Must be in Pending state
- **Idempotent**: No effect if already active

#### `ReminderStatistics` Data Class
```python
@dataclass
class ReminderStatistics:
    total_reminders: int
    pending_reminders: int
    active_reminders: int
    overdue_reminders: int
    unique_tags: int
```

**Methods:**

##### `pending_percentage() -> float`
- **Returns**: Percentage of reminders in pending state (0-100)

##### `active_percentage() -> float`
- **Returns**: Percentage of reminders in active state (0-100)

##### `overdue_percentage() -> float`
- **Returns**: Percentage of overdue reminders (0-100)

#### `ReminderUpdate` Builder Class
```python
@dataclass
class ReminderUpdate:
    content: Optional[str] = None
    remind_at: Optional[datetime] = None
    priority: Optional[ReminderPriority] = None
    status: Optional[ReminderStatus] = None
    tags: Optional[List[str]] = None
```

**Builder Methods:**

##### `new() -> ReminderUpdate`
- **Purpose**: Factory method for creating update instances

##### `with_content(content: str) -> ReminderUpdate`
- **Purpose**: Sets content update with validation

##### `with_remind_at(remind_at: datetime) -> ReminderUpdate`
- **Purpose**: Sets reminder time with UTC validation

##### `with_priority(priority: ReminderPriority) -> ReminderUpdate`
- **Purpose**: Sets priority level

##### `with_status(status: ReminderStatus) -> ReminderUpdate`
- **Purpose**: Sets status with validation

##### `with_tags(tags: List[str]) -> ReminderUpdate`
- **Purpose**: Sets tags with whitespace normalization

### 3. Core Manager

#### `ReminderManager` Class
```python
class ReminderManager:
    def __init__(self) -> None
```

**Internal State:**
- `reminders: Dict[str, Reminder]` - Primary storage by ID
- `reminder_tags: Dict[str, List[str]]` - Tag index for efficient searching

#### Core CRUD Operations

##### `create_reminder(reminder: Reminder) -> str`
- **Purpose**: Creates a new reminder with validation
- **Parameters**: `reminder` - Pre-configured Reminder instance
- **Returns**: Generated reminder ID
- **Validation**:
  - Non-empty content
  - UTC timezone requirement
  - Future reminder time requirement
- **Side Effects**: Auto-generates ID and timestamps

##### `get_reminder(reminder_id: str) -> Reminder`
- **Purpose**: Retrieves reminder by ID
- **Parameters**: `reminder_id` - Unique identifier
- **Returns**: Reminder instance
- **Throws**: `TodoziError` if reminder not found

##### `get_all_reminders() -> List[Reminder]`
- **Purpose**: Retrieves all reminders
- **Returns**: List of reminder copies (API stability)

##### `update_reminder(reminder_id: str, updates: ReminderUpdate) -> None`
- **Purpose**: Applies partial updates to reminder
- **Parameters**:
  - `reminder_id` - Target reminder ID
  - `updates` - Builder-configured changes
- **Validation**: Same as creation validations for updated fields
- **Side Effects**: Updates modification timestamp

##### `delete_reminder(reminder_id: str) -> None`
- **Purpose**: Removes reminder from system
- **Parameters**: `reminder_id` - Target reminder ID
- **Throws**: `TodoziError` if reminder not found

#### Query Operations

##### `search_reminders(query: str) -> List[Reminder]`
- **Purpose**: Case-insensitive search in content and tags
- **Parameters**: `query` - Search term
- **Returns**: Matching reminders

##### `get_reminders_by_priority(priority: ReminderPriority) -> List[Reminder]`
- **Purpose**: Filters reminders by priority level

##### `get_reminders_by_status(status: ReminderStatus) -> List[Reminder]`
- **Purpose**: Filters reminders by status

##### `get_reminders_by_tag(tag: str) -> List[Reminder]`
- **Purpose**: Case-insensitive tag filtering

##### `get_pending_reminders() -> List[Reminder]`
- **Purpose**: Shortcut for pending status filter

##### `get_active_reminders() -> List[Reminder]`
- **Purpose**: Shortcut for active status filter

##### `get_overdue_reminders() -> List[Reminder]`
- **Purpose**: Finds reminders past due time with active status
- **Criteria**: `remind_at < now` AND status in (Pending, Active)

##### `get_reminders_due_soon(duration: timedelta) -> List[Reminder]`
- **Purpose**: Finds reminders due within specified duration
- **Parameters**: `duration` - Time window from current time
- **Criteria**: `now < remind_at <= now + duration` AND active status

##### `get_recent_reminders(limit: int) -> List[Reminder]`
- **Purpose**: Retrieves most recently created reminders
- **Parameters**: `limit` - Maximum number to return
- **Sorting**: Descending by creation time

#### Statistical Operations

##### `get_all_tags() -> List[str]`
- **Purpose**: Returns unique sorted list of all tags

##### `get_tag_statistics() -> Dict[str, int]`
- **Purpose**: Returns tag usage frequency counts

##### `get_reminder_statistics() -> ReminderStatistics`
- **Purpose**: Returns comprehensive system statistics

#### Status Management

##### `mark_reminder_completed(reminder_id: str) -> None`
- **Purpose**: Completes a specific reminder

##### `mark_reminder_cancelled(reminder_id: str) -> None`
- **Purpose**: Cancels a specific reminder

##### `activate_reminder(reminder_id: str) -> None`
- **Purpose**: Activates a pending reminder

#### Observer System

##### `add_listener(callback: Callable[[str, str], None]) -> None`
- **Purpose**: Registers event listener
- **Parameters**: `callback(event, reminder_id)` - Event handler
- **Events**: create|update|delete|status_change|activate|complete|cancel

### 4. Persistent Storage

#### `PersistentReminderManager` Class
```python
class PersistentReminderManager(ReminderManager):
    def __init__(self, storage_path: Path) -> None
```

**Features:**
- Automatic loading on initialization
- Atomic file writes with temporary file swapping
- JSON-based serialization with UTF-8 encoding
- Graceful handling of malformed data

**Methods:**

##### `save() -> None`
- **Purpose**: Persists current state to storage path
- **Format**: JSON with ISO 8601 datetime formatting
- **Atomicity**: Uses temporary file to prevent corruption

##### `_load() -> None`
- **Purpose**: Loads state from storage path
- **Robustness**: Continues loading despite individual entry failures

### 5. Enhanced Features

#### `EnhancedReminderManager` Class
```python
class EnhancedReminderManager(ReminderManager):
```

**Additional Methods:**

##### `get_reminders_by_time_range(start_time: datetime, end_time: datetime) -> List[Reminder]`
- **Purpose**: Finds reminders within time range
- **Parameters**: UTC start and end times

##### `get_reminders_by_multiple_tags(tags: List[str]) -> List[Reminder]`
- **Purpose**: Filters reminders matching any of multiple tags

##### `get_reminder_categories() -> Dict[str, List[Reminder]]`
- **Purpose**: Categorizes reminders by first tag or "Uncategorized"

##### `bulk_update_status(reminder_ids: List[str], new_status: ReminderStatus) -> Dict[str, bool]`
- **Purpose**: Batch status updates with individual success tracking
- **Returns**: Mapping of reminder ID to success status

##### `export_reminders_to_json() -> List[Dict[str, Any]]`
- **Purpose**: Exports all reminders to JSON-serializable format

##### `import_reminders_from_json(reminders_data: List[Dict[str, Any]]) -> List[str]`
- **Purpose**: Imports reminders from JSON data
- **Returns**: List of successfully imported reminder IDs
- **Robustness**: Continues importing despite individual failures

### 6. Utility Functions

#### `parse_reminder_format(reminder_text: str) -> Reminder`
- **Purpose**: Parses reminders from custom text format
- **Format**: `<reminder>content;datetime;priority[;status[;tags]]</reminder>`
- **Validation**: Comprehensive format and content validation

#### `_patch_notifications(manager: ReminderManager) -> None`
- **Purpose**: Monkey-patches manager methods with notification support
- **Usage**: Applied to enable observer pattern functionality

#### `batch_update(manager: ReminderManager)`
- **Purpose**: Context manager for batch operations
- **Future Use**: Placeholder for optimization features

## API Reference

### Method Signatures and Return Types

| Method | Parameters | Return Type | Async | Throws |
|--------|------------|-------------|-------|--------|
| `create_reminder` | `Reminder` | `str` | ✓ | `TodoziError` |
| `get_reminder` | `str` | `Reminder` | ✗ | `TodoziError` |
| `update_reminder` | `str, ReminderUpdate` | `None` | ✓ | `TodoziError` |
| `delete_reminder` | `str` | `None` | ✓ | `TodoziError` |
| `search_reminders` | `str` | `List[Reminder]` | ✗ | - |
| `get_reminder_statistics` | - | `ReminderStatistics` | ✗ | - |

## Usage Patterns

### Basic Usage Example
```python
import asyncio
from datetime import datetime, timedelta, timezone
from todozi import Reminder, ReminderPriority, ReminderStatus, EnhancedReminderManager, _patch_notifications

async def main():
    # Initialize manager
    manager = EnhancedReminderManager()
    _patch_notifications(manager)  # Enable notifications
    
    # Create reminder
    reminder = Reminder(
        id="",  # Auto-generated
        content="Team meeting preparation",
        remind_at=datetime.now(timezone.utc) + timedelta(hours=24),
        priority=ReminderPriority.High,
        status=ReminderStatus.Pending,
        tags=["meeting", "work", "important"],
        created_at=datetime.now(timezone.utc),
        updated_at=datetime.now(timezone.utc),
    )
    
    reminder_id = await manager.create_reminder(reminder)
    print(f"Created reminder: {reminder_id}")
    
    # Update using builder pattern
    from todozi import ReminderUpdate
    await manager.update_reminder(
        reminder_id,
        ReminderUpdate.new()
            .with_content("Team meeting preparation - URGENT")
            .with_priority(ReminderPriority.High)
    )
    
    # Query operations
    overdue = manager.get_overdue_reminders()
    high_priority = manager.get_reminders_by_priority(ReminderPriority.High)
    
    # Statistics
    stats = manager.get_reminder_statistics()
    print(f"Pending: {stats.pending_percentage():.1f}%")

asyncio.run(main())
```

### Event Listening Example
```python
def event_handler(event: str, reminder_id: str):
    print(f"Event: {event} for reminder {reminder_id}")
    
manager.add_listener(event_handler)
```

### Batch Operations Example
```python
from todozi import batch_update

with batch_update(manager):
    # Multiple operations performed together
    results = manager.bulk_update_status(reminder_ids, ReminderStatus.Completed)
    print(f"Successful updates: {sum(results.values())}")
```

### Persistent Storage Example
```python
from pathlib import Path
from todozi import PersistentReminderManager

# Auto-loads from file if exists
manager = PersistentReminderManager(Path("reminders.json"))

# ... perform operations ...

manager.save()  # Explicit save (auto-save could be added)
```

## Error Handling

### Common Error Scenarios

1. **Validation Errors**:
   - Empty reminder content
   - Naive datetime objects (missing timezone)
   - Past reminder times
   - Invalid enum values

2. **Operational Errors**:
   - Non-existent reminder IDs
   - Malformed data during import/parsing
   - File I/O errors in persistent storage

3. **Listener Errors**:
   - Failures in observer callbacks (gracefully handled)

### Error Recovery Strategies

- **Data Integrity**: Atomic file operations prevent partial writes
- **Robust Loading**: Continues despite individual malformed entries
- **Graceful Degradation**: Listener failures don't break core functionality

## Performance Considerations

### Time Complexity Analysis

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Create/Update/Delete | O(1) | Dictionary-based operations |
| Get by ID | O(1) | Direct dictionary access |
| Search operations | O(n) | Linear scans through reminders |
| Tag-based queries | O(n) | No inverted index optimization |
| Statistical queries | O(n) | Requires full dataset scanning |

### Memory Usage

- **Primary Storage**: `O(n)` where n is number of reminders
- **Tag Index**: Additional `O(n)` storage for efficient tag queries
- **JSON Serialization**: Temporary memory spike during save/load operations

### Optimization Opportunities

1. **Indexing**: Add inverted indexes for frequent queries (tags, status)
2. **Pagination**: Implement limit/offset for large datasets
3. **Caching**: Cache statistical data with invalidation on mutations
4. **Lazy Loading**: For persistent storage with large datasets

## Technical Constraints

### Platform Requirements

- **Python Version**: 3.7+ (due to `from __future__ import annotations`)
- **Timezone Handling**: Requires timezone-aware datetime objects
- **Storage**: File-based persistence requires writable filesystem

### Design Limitations

1. **Single-threaded**: Not designed for concurrent access
2. **In-memory Focus**: Persistent storage is optional enhancement
3. **No Built-in Scheduling**: Reminder triggering must be implemented externally
4. **Linear Search**: No advanced indexing for complex queries

### Scalability Considerations

- **Suitable For**: Small to medium-sized datasets (thousands of reminders)
- **Not Recommended For**: High-frequency real-time systems
- **Extension Points**: Modular design allows for optimized implementations

## Dependencies

### Standard Library Dependencies
```python
import asyncio
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timedelta, timezone
from enum import Enum
from pathlib import Path
from typing import (Any, Callable, Dict, List, Optional, Set, Tuple, 
                   ValuesView, Union, cast)
from uuid import uuid4
import json
from contextlib import contextmanager
```

### External Dependencies
- **None**: Pure Python implementation with no external dependencies

### Compatibility Notes

- **Type Hinting**: Comprehensive type hints for IDE support and mypy validation
- **Async/Await**: Limited async usage (primarily for notification extensibility)
- **Serialization**: JSON-based with ISO 8601 datetime formatting

## Future Enhancements

### Planned Features
1. **Database Backend**: SQLite/PostgreSQL integration
2. **Advanced Indexing**: Optimized query performance
3. **Reminder Scheduling**: Built-in notification system
4. **Web Interface**: REST API and web UI
5. **Advanced Search**: Full-text search with stemming
6. **Batch Operations**: Optimized bulk processing

### Extension Points
- Custom manager implementations for different storage backends
- Additional reminder metadata and custom fields
- Plugin system for advanced features
- Integration with calendar systems and notification services

---

*This documentation covers version 1.0 of the Todozi reminder management system. For specific implementation questions or bug reports, refer to the source code annotations and test suites.*