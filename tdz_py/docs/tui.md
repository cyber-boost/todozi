# Todozi TUI Technical Documentation

## Overview

Todozi TUI is a sophisticated terminal user interface implementation of the Todozi task management system, matching the functionality and design principles of its Rust counterpart. This Python application provides a comprehensive interface for managing tasks, projects, and extended data types through an intuitive text-based interface.

## Architecture

### Core Components

#### Application Structure
- **Main Application Class**: `TodoziTUI` extends `textual.app.App`
- **Widget System**: Custom widgets built on Textual framework
- **Data Layer**: Integration with `todozi` package models and storage
- **Event System**: Asynchronous event handling with proper error management

#### Key Architectural Decisions
- **Model-View Separation**: Clear separation between data models and UI presentation
- **Reactive Programming**: Extensive use of Textual's reactive system for state management
- **Async-First Design**: Built around asyncio for non-blocking operations
- **Graceful Degradation**: Robust error handling and fallback mechanisms

## Dependencies and Requirements

### Core Dependencies
```python
# Textual Framework
textual >= 0.34.0
# Todozi Core Package
todozi >= 1.0.0
# Standard Library
asyncio, dataclasses, enum, typing, pathlib
```

### Optional Dependencies
- **watchdog**: For file system monitoring (graceful degradation if unavailable)
- **urllib3**: HTTP client utilities

### System Requirements
- Python 3.8+
- Terminal supporting ANSI colors and Unicode
- Sufficient terminal dimensions (minimum 80x24 recommended)

## Class Documentation

### TodoziTUI Class

**Purpose**: Main application class orchestrating the entire TUI interface.

```python
class TodoziTUI(App):
    """Main Todozi TUI Application - matching Rust implementation"""
```

#### Key Attributes

**Reactive State**
- `selected_task_index: reactive[int]`: Currently selected task index
- `current_tab: reactive[AppTab]`: Active application tab
- `filter_status: reactive[Optional[Status]]`: Current status filter

**Data Storage**
- `storage: Optional[Storage]`: Todozi storage backend
- `tasks: List[Task]`: Complete task list
- `filtered_tasks: List[Task]`: Filtered task view

#### Methods

**Core Lifecycle Methods**

```python
async def on_mount(self) -> None:
    """Initialize the app after mounting"""
    # Loads storage, starts file watcher, sets up intervals
```

```python
async def on_unmount(self) -> None:
    """Clean up resources when app is closed"""
    # Stops file watcher, cleans up resources
```

**Data Management Methods**

```python
async def load_storage(self) -> None:
    """Load storage asynchronously with error handling"""
    # Attempts to initialize Storage with proper error recovery
```

```python
async def refresh_data(self) -> None:
    """Refresh all data from storage"""
    # Loads tasks, projects, ideas, memories, errors, API keys
```

**UI Management Methods**

```python
def compose(self) -> ComposeResult:
    """Create the UI layout"""
    # Builds tabbed interface with all application sections
```

```python
def update_task_list(self) -> None:
    """Update the task list widget and associated status bars"""
    # Synchronizes UI with current data state
```

### ToastNotifier Class

**Purpose**: Centralized notification system for user feedback.

```python
class ToastNotifier:
    """Centralized toast notification management"""
```

#### Methods

```python
def success(self, message: str) -> None:
    """Show success toast with green styling"""
```

```python
def error(self, message: str) -> None:
    """Show error toast with red styling"""
```

### TaskListWidget Class

**Purpose**: Enhanced task list display with rich formatting.

```python
class TaskListWidget(ListView):
    """Enhanced task list widget"""
```

#### Methods

```python
def update_tasks(self, tasks: List[Task], selected_index: int = 0) -> None:
    """Update the task list with formatted display"""
    # Uses emoji indicators for status, priority, assignee
```

### TaskDetailWidget Class

**Purpose**: Detailed task information display.

```python
class TaskDetailWidget(Static):
    """Widget showing detailed task information"""
```

#### Methods

```python
def update_task(self, task: Optional[Task]) -> None:
    """Update displayed task with safe attribute access"""
    # Handles missing attributes gracefully
```

## Enum Definitions

### AppTab Enum
```python
class AppTab(Enum):
    """Application tab identifiers"""
    Projects = "📁 Projects"    # Project management
    Tasks = "📋 Tasks"          # Task management  
    Done = "✅ Done"           # Completed tasks
    Find = "🔍 Find"           # Search functionality
    More = "🔮 More"           # Extended data
    Api = "🔑 API"             # API management
    Feed = "📰 Feed"           # Activity feed
    Bye = "👋 Bye"             # Exit interface
```

### TaskSortBy Enum
```python
class TaskSortBy(Enum):
    """Task sorting criteria"""
    DateCompleted = auto()   # Completion date
    DateCreated = auto()     # Creation date  
    Priority = auto()        # Priority level
    Project = auto()         # Project association
    Action = auto()          # Task description
    Time = auto()            # Time estimates
    Assignee = auto()        # Assignment status
```

## Data Models

### ToastNotification
```python
@dataclass
class ToastNotification:
    """Toast notification data structure"""
    message: str                    # Notification content
    notification_type: ToastType    # Message severity
    created_at: float               # Creation timestamp
    duration: float = 5.0           # Display duration
```

### ActivityEntry
```python
@dataclass  
class ActivityEntry:
    """Activity feed entry"""
    timestamp: datetime  # Event time
    message: str         # Activity description
    level: str = "info"  # Severity level
```

### EditSession
```python
@dataclass
class EditSession:
    """Task editing session state"""
    task_id: str              # Task identifier
    original_task: Task       # Original task state
    current_task: Task        # Current edits
    ai_suggestions: List[str] # AI-generated suggestions
    validation_errors: List[str] # Current validation issues
    session_start: datetime   # Session start time
```

## Key Algorithms and Patterns

### File Watching Mechanism
```python
def start_file_watcher(self) -> None:
    """Start watching Todozi storage directory for changes"""
    # Uses watchdog with debounced change detection
    # Gracefully degrades if watchdog unavailable
```

**Technical Constraints**:
- Requires `watchdog` package for full functionality
- Limited to single directory monitoring
- Change detection has ~1-second debounce

### Search Implementation
```python
def _search_tasks(self, query: str) -> List[Task]:
    """Search tasks by action, tags, or parent project"""
    # Case-insensitive substring matching
    # Multi-field search (action, tags, project)
```

**Performance Considerations**:
- Linear search through all tasks
- Suitable for typical task counts (<10,000)
- No indexing for large datasets

### Error Handling Pattern
```python
def _handle_error(self, context: str, exception: Exception) -> None:
    """Centralized error handling"""
    # Logs to activity feed
    # Shows user-friendly toast notification
    # Maintains application stability
```

## UI Components and Layout

### CSS Styling System
The application uses a custom CSS system with Todozi brand colors:

```css
/* Color Scheme */
--dark: #1a1a2e          /* Main background */
--darker: #0d0d1a        /* Panel backgrounds */  
--sidebar-bg: #161625    /* Header/footer */
--primary: #4361ee       /* Primary accent */
--secondary: #7209b7     /* Secondary accent */
--light: #f8f9fa         /* Text color */
```

### Tab System Architecture
- **8 primary tabs** covering all functionality areas
- **Keyboard shortcuts** (1-8) for direct tab access
- **TabbedContent widget** for efficient space usage
- **Reactive tab switching** with state preservation

## Performance Considerations

### Memory Management
- **Activity feed**: Limited to 200 entries (deque with maxlen)
- **Task lists**: Loaded on-demand with filtering
- **Toast notifications**: Auto-expiration with timer cleanup

### Refresh Optimization
- **Auto-refresh interval**: 5 seconds (configurable)
- **Debounced file changes**: Prevents excessive refreshes
- **Selective data loading**: Only reloads necessary components

### Async Operations
- **Non-blocking storage operations**: Uses async/await pattern
- **Timer-based updates**: set_interval for periodic tasks
- **Event-driven architecture**: Responsive to user input

## Error Handling and Edge Cases

### Storage Availability
```python
def _ensure_ready(self) -> bool:
    """Check if storage and tasks are available"""
    # Returns False if storage unavailable
    # Shows warning toast to user
```

### Task Selection Safety
```python
def _current_task(self) -> Optional[Task]:
    """Safely get currently selected task"""
    # Validates index bounds
    # Handles empty task lists
```

### Widget Lifecycle Management
- **Graceful widget access**: Try/except around UI operations
- **Mount state awareness**: Checks if widgets are available
- **Progressive enhancement**: Basic functionality without advanced features

## Configuration and Customization

### Key Bindings System
```python
BINDINGS = [
    Binding("q", "quit", "Quit", priority=True),
    Binding("a", "add_task", "Add Task"),
    Binding("c", "complete_task", "Complete Task"),
    # ... 15+ keyboard shortcuts
]
```

### Customization Points
- **CSS variables**: Color scheme customization
- **Key bindings**: Remappable keyboard shortcuts
- **Refresh intervals**: Configurable timing
- **Toast durations**: Adjustable notification timing

## Integration Points

### Todozi Package Integration
- **Storage backend**: Direct integration with todozi.storage
- **Model compatibility**: Uses todozi.models directly
- **Error handling**: Compatible with todozi.error system
- **Extended data**: Ideas, memories, errors from respective managers

### File System Integration
- **Storage directory detection**: Automatic path resolution
- **Cross-platform compatibility**: Pathlib for OS-agnostic paths
- **Permission handling**: Graceful failure on access issues

## Limitations and Constraints

### Technical Limitations
- **Terminal dependency**: Requires capable terminal emulator
- **Python version**: Requires Python 3.8+ features
- **Package availability**: Depends on todozi package installation
- **Screen size**: Minimum 80x24 terminal dimensions

### Functional Constraints
- **No offline mode**: Requires todozi storage availability
- **Limited undo**: Single-level task operations
- **Export limitations**: Basic data export capabilities
- **Concurrent access**: No locking for multiple instances

## Usage Examples

### Basic Task Management
```python
# Start the application
app = TodoziTUI()
app.run()

# Typical workflow:
# 1. Navigate to Tasks tab (key 2)
# 2. Add task with 'a' key
# 3. Edit task with 'e' key  
# 4. Complete task with 'c' key
# 5. Filter tasks with 'f' key
```

### API Management
```python
# API tab provides:
# - Server start/stop controls
# - API key management
# - Server status monitoring
```

### Search Operations
```python
# Find tab enables:
# - Full-text search across tasks
# - Multi-field search (action, tags, project)
# - Real-time search results
```

## Troubleshooting and Debugging

### Common Issues
1. **Storage not loading**: Check todozi package installation
2. **Widget errors**: Verify terminal compatibility
3. **Performance issues**: Reduce auto-refresh interval
4. **Missing features**: Check optional dependency installation

### Debug Mode
```python
# Enable debug logging
import logging
logging.basicConfig(level=logging.DEBUG)
```

### Recovery Procedures
- **Storage failure**: Automatic reconnection attempts
- **UI errors**: Graceful component degradation
- **File watcher issues**: Automatic fallback to manual refresh

This documentation provides comprehensive coverage of the Todozi TUI implementation, including architectural decisions, technical specifications, and usage patterns suitable for developers and technical stakeholders.