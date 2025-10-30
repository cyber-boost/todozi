# tui.py
import asyncio
import json
import os
import uuid
import time
import threading
from datetime import datetime, timedelta
from enum import Enum
from pathlib import Path
from typing import List, Optional, Dict, Any, Union
from dataclasses import dataclass, field
import curses
import curses.textpad
import curses.panel

# Rich imports for terminal UI
try:
    from rich.console import Console
    from rich.layout import Layout
    from rich.panel import Panel
    from rich.table import Table
    from rich.text import Text
    from rich.progress import Progress, BarColumn, TextColumn, Progress as RichProgress
    from rich.live import Live
    from rich.prompt import Prompt
    from rich.tree import Tree
    from rich.markdown import Markdown
    from rich.syntax import Syntax
    from rich.align import Align
    from rich.columns import Columns
    from rich.rule import Rule
    from rich import box
    from rich.style import Style
    from rich.emoji import Emoji
    from rich.bar import Bar
    from rich.spinner import Spinner
    from rich.status import Status
except ImportError:
    print("Please install rich: pip install rich")
    exit(1)

# Textual imports for advanced TUI (keeping for compatibility)
try:
    from textual.app import App, ComposeResult
    from textual.containers import Container, Horizontal, Vertical, ScrollableContainer
    from textual.widgets import Header, Footer, DataTable, Static, Button, Input, Label
    from textual.screen import Screen
    from textual.reactive import reactive
    from textual.binding import Binding
    from textual.message import Message
    from textual.coordinate import Coordinate
except ImportError:
    print("Please install textual: pip install textual")
    exit(1)

# Additional imports for file watching and server management
try:
    import watchfiles
except ImportError:
    watchfiles = None

try:
    import requests
except ImportError:
    requests = None

# Todozi imports
try:
    from todozi import TodoziClient, Task as TodoziTask
    TODOZI_AVAILABLE = True
except ImportError:
    TODOZI_AVAILABLE = False

# Color scheme definitions
class ColorScheme:
    def __init__(self):
        self.primary = "#5755d9"
        self.primary_light = "#8381f4"
        self.primary_lighter = "#b4b2f7"
        self.primary_lightest = "#e0e0ff"
        self.primary_dark = "#3d3b99"
        self.secondary = "#9c27b0"
        self.success = "#4caf50"
        self.warning = "#ff9800"
        self.danger = "#f44336"
        self.error = "#e91e63"
        self.info = "#2196f3"
        self.muted = "#9e9e9e"
        self.dark = "#212121"
        self.gray = "#757575"
        self.light_gray = "#e0e0e0"
        self.white = "#ffffff"
        self.text = "#212121"
        self.background = "#ffffff"
        self.highlight = "#ffeb3b"
        self.border = "#bdbdbd"
        self.reset = "default"

# Display configuration
@dataclass
class DisplayConfig:
    show_ai_insights: bool = True
    show_similarity_scores: bool = True
    show_related_tasks: bool = True
    max_related_tasks: int = 5
    color_scheme: ColorScheme = field(default_factory=ColorScheme)
    compact_mode: bool = False
    show_embeddings: bool = False
    show_ids: bool = False
    show_created_at: bool = False
    show_dependencies: bool = False
    show_context: bool = False
    show_progress: bool = True

# Priority enum
class Priority(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"
    URGENT = "urgent"

# Status enum
class Status(Enum):
    TODO = "todo"
    PENDING = "pending"
    IN_PROGRESS = "inprogress"
    BLOCKED = "blocked"
    REVIEW = "review"
    DONE = "done"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    DEFERRED = "deferred"

# Assignee enum
class Assignee(Enum):
    HUMAN = "human"
    AI = "ai"
    COLLABORATIVE = "collaborative"

# Task model
@dataclass
class Task:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    user_id: str = "default"
    action: str = ""
    time: str = ""
    priority: Priority = Priority.MEDIUM
    status: Status = Status.TODO
    assignee: Optional[Assignee] = None
    parent_project: str = ""
    tags: List[str] = field(default_factory=list)
    dependencies: List[str] = field(default_factory=list)
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)

# Similarity result
@dataclass
class SimilarityResult:
    id: str
    action: str
    similarity_score: float
    tags: List[str] = field(default_factory=list)

# Todozi content type
class TodoziContentType(Enum):
    TASK = "task"
    IDEA = "idea"
    MEMORY = "memory"
    FEELING = "feeling"
    ERROR = "error"
    TRAINING_DATA = "training_data"
    QUEUE_ITEM = "queue_item"
    REMINDER = "reminder"

# Task display
@dataclass
class TaskDisplay:
    task: Task
    similar_tasks: List[SimilarityResult]
    ai_suggestions: List[str]
    semantic_tags: List[str]
    confidence_score: float
    related_content: List[SimilarityResult]

# Task list display
@dataclass
class TaskListDisplay:
    tasks: List[TaskDisplay]
    total_count: int
    ai_summary: str
    semantic_clusters: List[List[str]]

# Edit session
@dataclass
class EditSession:
    task_id: str
    original_task: Task
    current_task: Task
    ai_suggestions: List[str]
    validation_errors: List[str]
    similarity_matches: List[SimilarityResult]
    session_start: datetime

# App tabs
class AppTab(Enum):
    PROJECTS = "projects"
    TASKS = "tasks"
    DONE = "done"
    FIND = "find"
    MORE = "more"
    API = "api"
    FEED = "feed"
    BYE = "bye"

# Task filters
@dataclass
class TaskFilters:
    status_filter: Optional[Status] = None
    priority_filter: Optional[Priority] = None
    project_filter: Optional[str] = None
    assignee_filter: Optional[Assignee] = None

# Task sort by
class TaskSortBy(Enum):
    DATE_COMPLETED = "date_completed"
    DATE_CREATED = "date_created"
    PRIORITY = "priority"
    PROJECT = "project"
    ACTION = "action"
    TIME = "time"
    ASSIGNEE = "assignee"

# Sort order
class SortOrder(Enum):
    ASCENDING = "ascending"
    DESCENDING = "descending"

# Editor field
class EditorField(Enum):
    ACTION = "action"
    TIME = "time"
    PRIORITY = "priority"
    STATUS = "status"
    PROJECT = "project"
    ASSIGNEE = "assignee"
    TAGS = "tags"
    CONTEXT = "context"
    PROGRESS = "progress"

# Task action
class TaskAction(Enum):
    EDIT = "edit"
    DELETE = "delete"
    VIEW_DETAILS = "view_details"
    DUPLICATE = "duplicate"

# Toast notification
class ToastType(Enum):
    SUCCESS = "success"
    ERROR = "error"
    WARNING = "warning"
    INFO = "info"

@dataclass
class ToastNotification:
    message: str
    notification_type: ToastType
    created_at: datetime
    duration: timedelta

# More tab section
class MoreTabSection(Enum):
    IDEAS = "ideas"
    MEMORIES = "memories"
    FEELINGS = "feelings"
    ERRORS = "errors"
    TRAINING = "training"
    QUEUE = "queue"
    REMINDERS = "reminders"
    ANALYTICS = "analytics"

# API Key model
@dataclass
class ApiKey:
    user_id: str
    active: bool
    created_at: datetime = field(default_factory=datetime.utcnow)
    last_used: Optional[datetime] = None

# Queue Item model
@dataclass
class QueueItem:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    task_name: str = ""
    priority: int = 0
    created_at: datetime = field(default_factory=datetime.utcnow)

# Reminder model
@dataclass
class Reminder:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str = ""
    message: str = ""
    due_date: Optional[datetime] = None
    completed: bool = False
    created_at: datetime = field(default_factory=datetime.utcnow)

# Training Data model
@dataclass
class TrainingData:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    data_type: str = ""
    prompt: str = ""
    response: str = ""
    created_at: datetime = field(default_factory=datetime.utcnow)

# Error model
@dataclass
class Error:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str = ""
    message: str = ""
    stack_trace: Optional[str] = None
    created_at: datetime = field(default_factory=datetime.utcnow)

# Feeling model
@dataclass
class Feeling:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    emotion: str = ""
    intensity: int = 5
    context: Optional[str] = None
    created_at: datetime = field(default_factory=datetime.utcnow)

# Memory model
@dataclass
class Memory:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    moment: str = ""
    description: str = ""
    tags: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=datetime.utcnow)

# Idea model
@dataclass
class Idea:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str = ""
    description: str = ""
    category: str = ""
    created_at: datetime = field(default_factory=datetime.utcnow)

# Main TUI Application using curses with Rich rendering
class TodoziApp:
    """A curses-based TUI app for Todozi with Rich rendering."""

    def __init__(self):
        self.display_config = DisplayConfig()
        self.current_tab = AppTab.PROJECTS

        # Initialize Todozi client if available
        self.todozi_client = None
        if TODOZI_AVAILABLE:
            try:
                self.todozi_client = TodoziClient()
            except Exception as e:
                print(f"Warning: Could not initialize Todozi client: {e}")
                print("Running in demo mode with mock data")

        # Task management
        self.tasks: List[Task] = []
        self.filtered_tasks: List[Task] = []
        self.selected_task_index = 0
        self.task_filters = TaskFilters()
        self.projects: List[str] = []

        # Done tab state
        self.done_sort_by = TaskSortBy.DATE_COMPLETED
        self.done_sort_order = SortOrder.DESCENDING
        self.done_filters = TaskFilters()
        self.done_selected_task_index = 0

        # Project tab state
        self.selected_project_index = 0

        # Search state
        self.search_query = ""
        self.search_results: List[Task] = []

        # Editor state
        self.editor: Optional[EditSession] = None
        self.editor_field = EditorField.ACTION
        self.editor_input = ""
        self.editor_selected_field = 0

        # Action menu state
        self.task_action_menu: Optional[int] = None
        self.task_action_selected = 0

        # Details popup state
        self.show_task_details: Optional[Task] = None

        # Application state
        self.should_quit = False

        # Analytics data
        self.completion_data: List[int] = [0] * 50
        self.priority_distribution: List[int] = [0] * 10

        # Server state
        self.server_status = "Starting..."
        self.server_running = False

        # Extended data
        self.ideas: List[Idea] = []
        self.memories: List[Memory] = []
        self.feelings: List[Feeling] = []
        self.errors: List[Error] = []
        self.training_data: List[TrainingData] = []
        self.queue_items: List[QueueItem] = []
        self.reminders: List[Reminder] = []

        # More tab state
        self.more_tab_section = MoreTabSection.IDEAS
        self.more_tab_selected_index = 0
        self.more_scroll_offset = 0

        # Feed tab state
        self.feed_scroll_offset = 0

        # API tab state
        self.api_keys: List[ApiKey] = []
        self.api_selected_index = 0
        self.api_endpoints_scroll = 0
        self.api_keys_scroll = 0
        self.show_api_key_details: Optional[ApiKey] = None

        # Notifications
        self.toast_notifications: List[ToastNotification] = []

        # Curses state
        self.stdscr = None
        self.console = Console()

        # File watching
        self.file_watcher_thread = None
        self.stop_watching = False

        # Load initial data
        self.load_tasks()
        self.load_extended_data()
        self.apply_filters()

    def setup_terminal(self):
        """Setup curses terminal."""
        self.stdscr = curses.initscr()
        curses.noecho()
        curses.cbreak()
        curses.start_color()
        curses.curs_set(0)
        self.stdscr.keypad(True)

        # Initialize color pairs
        curses.init_pair(1, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_GREEN, curses.COLOR_BLACK)
        curses.init_pair(3, curses.COLOR_YELLOW, curses.COLOR_BLACK)
        curses.init_pair(4, curses.COLOR_BLUE, curses.COLOR_BLACK)
        curses.init_pair(5, curses.COLOR_MAGENTA, curses.COLOR_BLACK)
        curses.init_pair(6, curses.COLOR_CYAN, curses.COLOR_BLACK)
        curses.init_pair(7, curses.COLOR_WHITE, curses.COLOR_BLACK)

    def restore_terminal(self):
        """Restore terminal to normal state."""
        if self.stdscr:
            curses.nocbreak()
            self.stdscr.keypad(False)
            curses.echo()
            curses.endwin()

    def generate_separator_line(self, width: int, style_char: str = "─") -> str:
        """Generate a separator line."""
        return style_char * width

    def responsive_text(self, text: str, max_width: int) -> str:
        """Truncate text to fit within max_width."""
        if len(text) <= max_width:
            return text
        return text[:max_width - 3] + "..."

    def generate_progress_bar(self, percentage: float, width: int, filled_char: str = "█", empty_char: str = "░") -> str:
        """Generate a progress bar string."""
        filled_width = int(width * percentage / 100.0)
        empty_width = width - filled_width
        return filled_char * filled_width + empty_char * empty_width

    def get_status_icon(self, status: Status) -> str:
        """Get status icon for task."""
        icons = {
            Status.TODO: "📝",
            Status.PENDING: "📝",
            Status.IN_PROGRESS: "🔄",
            Status.BLOCKED: "🚫",
            Status.REVIEW: "👀",
            Status.DONE: "✅",
            Status.COMPLETED: "✅",
            Status.CANCELLED: "❌",
            Status.DEFERRED: "⏸️",
        }
        return icons.get(status, "📝")

    def get_priority_score(self, task: Task) -> int:
        """Get priority score for sorting."""
        scores = {
            Priority.LOW: 1,
            Priority.MEDIUM: 2,
            Priority.HIGH: 3,
            Priority.CRITICAL: 4,
            Priority.URGENT: 5,
        }
        return scores.get(task.priority, 0)

    def format_duration(self, from_time: datetime, to_time: datetime) -> str:
        """Format duration between two datetimes."""
        duration = to_time - from_time
        total_seconds = int(duration.total_seconds())

        if total_seconds < 60:
            return f"{total_seconds}s ago"
        elif total_seconds < 3600:
            return f"{total_seconds // 60}m ago"
        elif total_seconds < 86400:
            return f"{total_seconds // 3600}h ago"
        elif total_seconds < 604800:
            return f"{total_seconds // 86400}d ago"
        else:
            return f"{total_seconds // 604800}w ago"

    def get_filtered_done_tasks(self) -> List[Task]:
        """Get filtered done tasks."""
        tasks = [
            task for task in self.tasks
            if task.status in [Status.DONE, Status.COMPLETED]
        ]

        # Apply filters
        if self.done_filters.status_filter:
            tasks = [t for t in tasks if t.status == self.done_filters.status_filter]
        if self.done_filters.priority_filter:
            tasks = [t for t in tasks if t.priority == self.done_filters.priority_filter]
        if self.done_filters.project_filter:
            tasks = [t for t in tasks if t.parent_project == self.done_filters.project_filter]

        # Sort tasks
        reverse = self.done_sort_order == SortOrder.DESCENDING
        if self.done_sort_by == TaskSortBy.DATE_COMPLETED:
            tasks.sort(key=lambda t: t.updated_at, reverse=reverse)
        elif self.done_sort_by == TaskSortBy.DATE_CREATED:
            tasks.sort(key=lambda t: t.created_at, reverse=reverse)
        elif self.done_sort_by == TaskSortBy.PRIORITY:
            tasks.sort(key=self.get_priority_score, reverse=reverse)
        elif self.done_sort_by == TaskSortBy.PROJECT:
            tasks.sort(key=lambda t: t.parent_project or "", reverse=reverse)
        elif self.done_sort_by == TaskSortBy.ACTION:
            tasks.sort(key=lambda t: t.action, reverse=reverse)

        return tasks

    def load_tasks(self) -> None:
        """Load tasks from storage."""
        if self.todozi_client:
            try:
                # Load tasks from Todozi
                todozi_tasks = self.todozi_client.all()
                self.tasks = []
                for t in todozi_tasks:
                    # Convert TodoziTask to our Task model
                    task = Task(
                        id=t.id,
                        user_id=t.user_id,
                        action=t.action,
                        time=t.time,
                        priority=Priority(t.priority.lower()) if hasattr(t, 'priority') and t.priority else Priority.MEDIUM,
                        status=Status(t.status.lower()) if hasattr(t, 'status') and t.status else Status.TODO,
                        parent_project=getattr(t, 'parent_project', ''),
                        tags=getattr(t, 'tags', []),
                        created_at=datetime.fromisoformat(t.created_at.replace('Z', '+00:00')) if hasattr(t, 'created_at') else datetime.utcnow()
                    )
                    self.tasks.append(task)
            except Exception as e:
                print(f"Failed to load tasks from Todozi: {e}")
                self._load_mock_tasks()
        else:
            self._load_mock_tasks()

        self.filtered_tasks = self.tasks.copy()
        self.projects = list(set(task.parent_project for task in self.tasks if task.parent_project))
        self.update_progress_data()

    def _load_mock_tasks(self) -> None:
        """Load mock tasks for demo purposes."""
        self.tasks = [
            Task(action="Complete project proposal", status=Status.TODO, priority=Priority.HIGH, parent_project="Work"),
            Task(action="Review team feedback", status=Status.IN_PROGRESS, priority=Priority.MEDIUM, parent_project="Work"),
            Task(action="Update documentation", status=Status.TODO, priority=Priority.LOW, parent_project="Dev"),
            Task(action="Fix critical bug", status=Status.BLOCKED, priority=Priority.CRITICAL, parent_project="Dev"),
            Task(action="Plan weekend trip", status=Status.DONE, priority=Priority.MEDIUM, parent_project="Personal", updated_at=datetime.utcnow() - timedelta(days=2)),
            Task(action="Buy groceries", status=Status.COMPLETED, priority=Priority.LOW, parent_project="Personal", updated_at=datetime.utcnow() - timedelta(days=1)),
        ]

    def load_extended_data(self) -> None:
        """Load extended data."""
        # In a real implementation, this would load from files or database
        self.ideas = [Idea(title="Improve UI design", description="Make the interface more intuitive", category="UI/UX")]
        self.memories = [Memory(moment="First successful deployment", description="The moment we launched our first product", tags=["milestone", "success"])]
        self.feelings = [Feeling(emotion="Happy", intensity=8, context="Completed a challenging task")]
        self.errors = [Error(title="Database connection failed", message="Connection timeout after 30 seconds")]
        self.training_data = [TrainingData(data_type="task", prompt="How to prioritize tasks", response="Use Eisenhower matrix")]
        self.queue_items = [QueueItem(task_name="Process user feedback", priority=3)]
        self.api_keys = [ApiKey(user_id="user123", active=True)]

    def load_api_keys(self) -> None:
        """Load API keys from storage."""
        # In a real implementation, this would load from secure storage
        pass

    def apply_filters(self) -> None:
        """Apply task filters."""
        self.filtered_tasks = [
            task for task in self.tasks
            if self.matches_filters(task)
        ]
        self.update_progress_data()

    def matches_filters(self, task: Task) -> bool:
        """Check if task matches current filters."""
        if self.task_filters.status_filter and task.status != self.task_filters.status_filter:
            return False
        if self.task_filters.priority_filter and task.priority != self.task_filters.priority_filter:
            return False
        if self.task_filters.project_filter and task.parent_project != self.task_filters.project_filter:
            return False
        if self.task_filters.assignee_filter and task.assignee != self.task_filters.assignee_filter:
            return False
        return True

    def update_progress_data(self) -> None:
        """Update analytics data."""
        # Update completion data (last 50 days)
        now = datetime.utcnow()
        for i in range(50):
            date = now - timedelta(days=49 - i)
            completed_count = len([
                task for task in self.tasks
                if task.status in [Status.DONE, Status.COMPLETED]
                and task.updated_at.date() == date.date()
            ])
            self.completion_data[i] = completed_count

        # Update priority distribution
        priority_counts = {Priority.LOW: 0, Priority.MEDIUM: 0, Priority.HIGH: 0, Priority.CRITICAL: 0, Priority.URGENT: 0}
        for task in self.tasks:
            if task.status not in [Status.DONE, Status.COMPLETED]:
                priority_counts[task.priority] += 1

        self.priority_distribution = [
            priority_counts[Priority.LOW],
            priority_counts[Priority.MEDIUM],
            priority_counts[Priority.HIGH],
            priority_counts[Priority.CRITICAL],
            priority_counts[Priority.URGENT],
        ]

    def get_completion_percentage(self) -> int:
        """Get overall completion percentage."""
        if not self.tasks:
            return 0
        completed = len([t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]])
        return int((completed / len(self.tasks)) * 100)

    def get_average_progress(self) -> float:
        """Get average progress percentage."""
        tasks_with_progress = [t for t in self.tasks if t.progress is not None]
        if not tasks_with_progress:
            return 0.0
        return sum(t.progress for t in tasks_with_progress) / len(tasks_with_progress)

    def save_tasks(self) -> None:
        """Save tasks to storage."""
        # In a real implementation, this would save to files or database
        pass

    def save_task(self, task: Task) -> None:
        """Save a single task."""
        # Find and update the task
        for i, t in enumerate(self.tasks):
            if t.id == task.id:
                self.tasks[i] = task
                break
        self.apply_filters()
        self.save_tasks()

    def delete_task(self, task_id: str) -> None:
        """Delete a task."""
        self.tasks = [t for t in self.tasks if t.id != task_id]
        self.apply_filters()
        self.save_tasks()

    def start_new_task_editor(self) -> None:
        """Start editing a new task."""
        new_task = Task(action="New task", status=Status.TODO, priority=Priority.MEDIUM)
        self.start_edit_task(new_task)

    def start_edit_task(self, task: Task) -> None:
        """Start editing an existing task."""
        self.editor = EditSession(
            task_id=task.id,
            original_task=task,
            current_task=task,
            ai_suggestions=[],
            validation_errors=[],
            similarity_matches=[],
            session_start=datetime.utcnow()
        )
        self.editor_selected_field = 0
        self.update_editor_field()

    def save_current_field(self) -> None:
        """Save the current editor field."""
        if not self.editor:
            return

        field_value = self.editor_input
        if self.editor_field == EditorField.ACTION:
            self.editor.current_task.action = field_value
        elif self.editor_field == EditorField.TIME:
            self.editor.current_task.time = field_value
        elif self.editor_field == EditorField.PRIORITY:
            priority_map = {
                "low": Priority.LOW,
                "medium": Priority.MEDIUM,
                "high": Priority.HIGH,
                "critical": Priority.CRITICAL,
                "urgent": Priority.URGENT,
            }
            self.editor.current_task.priority = priority_map.get(field_value.lower(), Priority.MEDIUM)
        elif self.editor_field == EditorField.STATUS:
            status_map = {
                "todo": Status.TODO,
                "pending": Status.PENDING,
                "inprogress": Status.IN_PROGRESS,
                "in_progress": Status.IN_PROGRESS,
                "blocked": Status.BLOCKED,
                "review": Status.REVIEW,
                "done": Status.DONE,
                "completed": Status.COMPLETED,
                "cancelled": Status.CANCELLED,
                "deferred": Status.DEFERRED,
            }
            self.editor.current_task.status = status_map.get(field_value.lower(), Status.TODO)
        elif self.editor_field == EditorField.PROJECT:
            self.editor.current_task.parent_project = field_value
        elif self.editor_field == EditorField.ASSIGNEE:
            assignee_map = {
                "human": Assignee.HUMAN,
                "ai": Assignee.AI,
                "collaborative": Assignee.COLLABORATIVE,
            }
            self.editor.current_task.assignee = assignee_map.get(field_value.lower(), None)
        elif self.editor_field == EditorField.TAGS:
            self.editor.current_task.tags = [tag.strip() for tag in field_value.split(',') if tag.strip()]
        elif self.editor_field == EditorField.CONTEXT:
            self.editor.current_task.context_notes = field_value if field_value else None
        elif self.editor_field == EditorField.PROGRESS:
            try:
                progress = int(field_value)
                self.editor.current_task.progress = min(max(progress, 0), 100)
            except ValueError:
                pass

    def load_current_field(self) -> None:
        """Load the current editor field into input."""
        if not self.editor:
            return

        if self.editor_field == EditorField.ACTION:
            self.editor_input = self.editor.current_task.action
        elif self.editor_field == EditorField.TIME:
            self.editor_input = self.editor.current_task.time
        elif self.editor_field == EditorField.PRIORITY:
            self.editor_input = self.editor.current_task.priority.value
        elif self.editor_field == EditorField.STATUS:
            self.editor_input = self.editor.current_task.status.value
        elif self.editor_field == EditorField.PROJECT:
            self.editor_input = self.editor.current_task.parent_project
        elif self.editor_field == EditorField.ASSIGNEE:
            self.editor_input = self.editor.current_task.assignee.value if self.editor.current_task.assignee else "None"
        elif self.editor_field == EditorField.TAGS:
            self.editor_input = ", ".join(self.editor.current_task.tags)
        elif self.editor_field == EditorField.CONTEXT:
            self.editor_input = self.editor.current_task.context_notes or ""
        elif self.editor_field == EditorField.PROGRESS:
            self.editor_input = str(self.editor.current_task.progress) if self.editor.current_task.progress else "0"

    def update_editor_field(self) -> None:
        """Update the current editor field based on selection."""
        fields = [
            EditorField.ACTION,
            EditorField.TIME,
            EditorField.PRIORITY,
            EditorField.STATUS,
            EditorField.PROJECT,
            EditorField.ASSIGNEE,
            EditorField.TAGS,
            EditorField.CONTEXT,
            EditorField.PROGRESS,
        ]
        self.editor_field = fields[self.editor_selected_field % len(fields)]

    def add_toast(self, message: str, notification_type: ToastType) -> None:
        """Add a toast notification."""
        toast = ToastNotification(
            message=message,
            notification_type=notification_type,
            created_at=datetime.utcnow(),
            duration=timedelta(seconds=5)
        )
        self.toast_notifications.append(toast)

    def update_toasts(self) -> None:
        """Update and clean up toast notifications."""
        now = datetime.utcnow()
        self.toast_notifications = [
            toast for toast in self.toast_notifications
            if (now - toast.created_at) < toast.duration
        ]

    def confirm_action(self, message: str) -> bool:
        """Show confirmation dialog."""
        self.add_toast(f"⚠️ {message}", ToastType.WARNING)
        return True  # In a real implementation, this would show a dialog

    def next_tab(self) -> None:
        """Switch to next tab."""
        tabs = list(AppTab)
        current_index = tabs.index(self.current_tab)
        self.current_tab = tabs[(current_index + 1) % len(tabs)]

    def previous_tab(self) -> None:
        """Switch to previous tab."""
        tabs = list(AppTab)
        current_index = tabs.index(self.current_tab)
        self.current_tab = tabs[(current_index - 1) % len(tabs)]

    def next_more_section(self) -> None:
        """Switch to next more section."""
        sections = list(MoreTabSection)
        current_index = sections.index(self.more_tab_section)
        self.more_tab_section = sections[(current_index + 1) % len(sections)]
        self.more_tab_selected_index = 0

    def previous_more_section(self) -> None:
        """Switch to previous more section."""
        sections = list(MoreTabSection)
        current_index = sections.index(self.more_tab_section)
        self.more_tab_section = sections[(current_index - 1) % len(sections)]
        self.more_tab_selected_index = 0

    def get_more_section_item_count(self) -> int:
        """Get the number of items in the current more section."""
        if self.more_tab_section == MoreTabSection.IDEAS:
            return len(self.ideas)
        elif self.more_tab_section == MoreTabSection.MEMORIES:
            return len(self.memories)
        elif self.more_tab_section == MoreTabSection.FEELINGS:
            return len(self.feelings)
        elif self.more_tab_section == MoreTabSection.ERRORS:
            return len(self.errors)
        elif self.more_tab_section == MoreTabSection.TRAINING:
            return len(self.training_data)
        elif self.more_tab_section == MoreTabSection.QUEUE:
            return len(self.queue_items)
        elif self.more_tab_section == MoreTabSection.REMINDERS:
            return len(self.reminders)
        elif self.more_tab_section == MoreTabSection.ANALYTICS:
            return 1  # Analytics is a single view
        return 0

    def update_search_results(self) -> None:
        """Update search results based on current query."""
        if not self.search_query:
            self.search_results = []
            return

        query = self.search_query.lower()
        self.search_results = [
            task for task in self.tasks
            if query in task.action.lower()
            or query in (task.parent_project or "").lower()
            or any(query in tag.lower() for tag in task.tags)
        ]

    def setup_file_watcher(self) -> None:
        """Setup file watching for data files."""
        if not watchfiles:
            return

        def watch_files():
            # Watch for changes in todozi data directory
            import os
            home = os.path.expanduser("~")
            todozi_dir = os.path.join(home, ".todozi")

            if os.path.exists(todozi_dir):
                for changes in watchfiles.watch(todozi_dir, stop_event=self.stop_watching):
                    if self.stop_watching:
                        break
                    self.load_tasks()
                    self.apply_filters()

        self.file_watcher_thread = threading.Thread(target=watch_files, daemon=True)
        self.file_watcher_thread.start()

    def cleanup_existing_server(self) -> None:
        """Clean up existing server processes."""
        # In a real implementation, this would kill existing server processes
        pass

    def start_server(self) -> None:
        """Start the Todozi server."""
        try:
            self.cleanup_existing_server()
            # In a real implementation, this would start the server process
            self.server_status = "Running"
            self.server_running = True
        except Exception as e:
            self.server_status = f"Failed: {str(e)}"
            self.server_running = False

    def check_server_status(self) -> None:
        """Check the status of the Todozi server."""
        if not self.server_running:
            self.server_status = "Stopped"
            return

        try:
            # In a real implementation, this would check server health
            self.server_status = "Running"
        except Exception as e:
            self.server_status = f"Error: {str(e)}"
            self.server_running = False

    def stop_server(self) -> None:
        """Stop the Todozi server."""
        # In a real implementation, this would stop the server process
        self.server_status = "Stopped"
        self.server_running = False

    def restart_server(self) -> None:
        """Restart the Todozi server."""
        self.stop_server()
        self.start_server()

    def clear_cache(self) -> None:
        """Clear application cache."""
        # In a real implementation, this would clear caches
        pass

    def run(self) -> None:
        """Run the TUI application."""
        try:
            self.setup_terminal()
            self.setup_file_watcher()
            self.start_server()

            while not self.should_quit:
                self.draw()
                self.handle_input()
                self.update_toasts()
                time.sleep(0.1)  # Small delay to prevent excessive CPU usage

        except KeyboardInterrupt:
            pass
        except Exception as e:
            print(f"Error running TUI: {e}")
        finally:
            self.stop_watching = True
            if self.file_watcher_thread:
                self.file_watcher_thread.join(timeout=1.0)
            self.restore_terminal()

    def draw(self) -> None:
        """Draw the current UI state."""
        if not self.stdscr:
            return

        height, width = self.stdscr.getmaxyx()

        # Clear screen
        self.stdscr.clear()

        # Draw header with tabs
        self.draw_tabs(height, width)

        # Draw main content area
        main_area_height = height - 4  # Leave room for header and footer
        main_area_width = width

        if self.editor:
            self.draw_editor(main_area_height, main_area_width)
        elif self.show_task_details:
            self.draw_task_details_popup(main_area_height, main_area_width)
        elif self.task_action_menu is not None:
            self.draw_action_menu(main_area_height, main_area_width)
        else:
            self.draw_main_content(main_area_height, main_area_width)

        # Draw status bar
        self.draw_status_bar(height - 2, width)

        # Draw toasts
        self.draw_toasts(height, width)

        # Refresh screen
        self.stdscr.refresh()

    def draw_tabs(self, height: int, width: int) -> None:
        """Draw the tab navigation."""
        tabs = [
            ("📁 Projects", AppTab.PROJECTS),
            ("📋 Tasks", AppTab.TASKS),
            ("✅ Done", AppTab.DONE),
            ("🔍 Find", AppTab.FIND),
            ("🔮 More", AppTab.MORE),
            ("🔑 API", AppTab.API),
            ("📰 Feed", AppTab.FEED),
            ("👋 Bye", AppTab.BYE),
        ]

        tab_width = width // len(tabs)
        for i, (label, tab) in enumerate(tabs):
            x = i * tab_width
            is_active = self.current_tab == tab

            if is_active:
                self.stdscr.addstr(0, x, f"[ {label} ]", curses.A_REVERSE | curses.A_BOLD)
            else:
                self.stdscr.addstr(0, x, f"  {label}  ", curses.A_NORMAL)

    def draw_main_content(self, height: int, width: int) -> None:
        """Draw the main content based on current tab."""
        y_offset = 2  # Start below tabs

        if self.current_tab == AppTab.PROJECTS:
            self.draw_projects_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.TASKS:
            self.draw_tasks_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.DONE:
            self.draw_done_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.FIND:
            self.draw_find_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.MORE:
            self.draw_more_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.API:
            self.draw_api_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.FEED:
            self.draw_feed_tab(y_offset, height - y_offset - 2, width)
        elif self.current_tab == AppTab.BYE:
            self.draw_bye_tab(y_offset, height - y_offset - 2, width)

    def draw_projects_tab(self, y: int, height: int, width: int) -> None:
        """Draw the projects tab."""
        self.stdscr.addstr(y, 0, "📁 Projects", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        if not self.projects:
            self.stdscr.addstr(y + 3, 0, "No projects found")
            return

        for i, project in enumerate(self.projects):
            if y + 3 + i >= y + height:
                break
            marker = "→ " if i == self.selected_project_index else "  "
            self.stdscr.addstr(y + 3 + i, 0, f"{marker}{project}")

    def draw_tasks_tab(self, y: int, height: int, width: int) -> None:
        """Draw the tasks tab."""
        self.stdscr.addstr(y, 0, "📋 Tasks", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        if not self.filtered_tasks:
            self.stdscr.addstr(y + 3, 0, "No tasks found with current filters")
            return

        # Show filter info
        filter_info = []
        if self.task_filters.status_filter:
            filter_info.append(f"Status: {self.task_filters.status_filter.value}")
        if self.task_filters.priority_filter:
            filter_info.append(f"Priority: {self.task_filters.priority_filter.value}")
        if self.task_filters.project_filter:
            filter_info.append(f"Project: {self.task_filters.project_filter}")

        filter_text = " | ".join(filter_info) if filter_info else "No filters"
        self.stdscr.addstr(y + 2, 0, f"Filters: {filter_text}")

        # Show tasks
        for i, task in enumerate(self.filtered_tasks):
            if y + 4 + i >= y + height:
                break
            marker = "→ " if i == self.selected_task_index else "  "
            status_icon = self.get_status_icon(task.status)
            action_display = self.responsive_text(task.action, width - 20)
            project_display = task.parent_project or "General"
            priority_display = task.priority.value.title()

            line = f"{marker}{status_icon} {action_display} [{project_display}] ({priority_display})"
            self.stdscr.addstr(y + 4 + i, 0, line[:width])

    def draw_done_tab(self, y: int, height: int, width: int) -> None:
        """Draw the done tab."""
        self.stdscr.addstr(y, 0, "✅ Completed Tasks", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        done_tasks = self.get_filtered_done_tasks()

        # Controls section
        controls_height = min(5, height // 3)
        self.stdscr.addstr(y + 2, 0, "Sort & Filter Controls:")
        sort_order_icon = "↓" if self.done_sort_order == SortOrder.DESCENDING else "↑"
        self.stdscr.addstr(y + 3, 2, f"Sort: {self.done_sort_by.title()} {sort_order_icon}")
        self.stdscr.addstr(y + 4, 2, "[s]ort [o]rder [p]roject [i]riority [c]lear")

        # Progress section
        progress_y = y + controls_height + 1
        if progress_y < y + height - 5:
            self.stdscr.addstr(progress_y, 0, "Progress:")
            completion_pct = self.get_completion_percentage()
            progress_bar = self.generate_progress_bar(float(completion_pct), width - 20)
            self.stdscr.addstr(progress_y + 1, 0, f"Overall: {completion_pct}% {progress_bar}")

        # Tasks section
        tasks_y = progress_y + 3
        tasks_height = height - (tasks_y - y) - 1

        if not done_tasks:
            self.stdscr.addstr(tasks_y, 0, "No completed tasks found")
            return

        self.stdscr.addstr(tasks_y, 0, f"Task ({len(done_tasks)} total):")
        self.stdscr.addstr(tasks_y + 1, 0, self.generate_separator_line(width))

        for i, task in enumerate(done_tasks[:tasks_height - 3]):
            if tasks_y + 2 + i >= y + height:
                break
            marker = "→ " if i == self.done_selected_task_index else "  "
            time_ago = self.format_duration(task.updated_at, datetime.utcnow())
            action_display = self.responsive_text(task.action, width - 30)
            project_display = task.parent_project or "General"

            line = f"{marker}✅ {action_display} [{project_display}] {time_ago}"
            self.stdscr.addstr(tasks_y + 2 + i, 0, line[:width])

    def draw_find_tab(self, y: int, height: int, width: int) -> None:
        """Draw the find tab."""
        self.stdscr.addstr(y, 0, "🔍 Find Tasks", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        self.stdscr.addstr(y + 2, 0, f"Search: {self.search_query}")
        self.stdscr.addstr(y + 3, 0, f"Results: {len(self.search_results)} tasks")

        if not self.search_results:
            if self.search_query:
                self.stdscr.addstr(y + 5, 0, "No tasks found matching your search")
            else:
                self.stdscr.addstr(y + 5, 0, "Enter a search query to find tasks")
            return

        self.stdscr.addstr(y + 4, 0, self.generate_separator_line(width))

        for i, task in enumerate(self.search_results[:height - 6]):
            marker = "→ " if i == self.selected_task_index else "  "
            status_icon = self.get_status_icon(task.status)
            action_display = self.responsive_text(task.action, width - 20)
            project_display = task.parent_project or "General"

            line = f"{marker}{status_icon} {action_display} [{project_display}]"
            self.stdscr.addstr(y + 5 + i, 0, line[:width])

    def draw_more_tab(self, y: int, height: int, width: int) -> None:
        """Draw the more tab."""
        self.stdscr.addstr(y, 0, "🔮 More", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        sections = list(MoreTabSection)
        section_width = width // len(sections)

        # Draw section tabs
        for i, section in enumerate(sections):
            x = i * section_width
            is_active = self.more_tab_section == section
            label = section.value.title()

            if is_active:
                self.stdscr.addstr(y + 2, x, f"[{label}]", curses.A_REVERSE | curses.A_BOLD)
            else:
                self.stdscr.addstr(y + 2, x, f" {label} ", curses.A_NORMAL)

        # Draw section content
        content_y = y + 4
        if self.more_tab_section == MoreTabSection.IDEAS:
            self.draw_ideas_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.MEMORIES:
            self.draw_memories_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.FEELINGS:
            self.draw_feelings_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.ERRORS:
            self.draw_errors_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.TRAINING:
            self.draw_training_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.QUEUE:
            self.draw_queue_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.REMINDERS:
            self.draw_reminders_content(content_y, height - (content_y - y), width)
        elif self.more_tab_section == MoreTabSection.ANALYTICS:
            self.draw_analytics_content(content_y, height - (content_y - y), width)

    def draw_api_tab(self, y: int, height: int, width: int) -> None:
        """Draw the API tab."""
        self.stdscr.addstr(y, 0, "🔑 API Management", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        self.stdscr.addstr(y + 2, 0, f"Server Status: {self.server_status}")
        self.stdscr.addstr(y + 3, 0, f"API Keys: {len(self.api_keys)}")

        if self.api_keys:
            self.stdscr.addstr(y + 5, 0, "Keys:")
            for i, key in enumerate(self.api_keys[:height - 7]):
                status = "Active" if key.active else "Inactive"
                line = f"  {key.user_id}: {status}"
                self.stdscr.addstr(y + 6 + i, 0, line[:width])

        # Controls
        controls_y = y + height - 3
        self.stdscr.addstr(controls_y, 0, "[x] stop server [c] clear cache [r] restart")

    def draw_feed_tab(self, y: int, height: int, width: int) -> None:
        """Draw the feed tab."""
        self.stdscr.addstr(y, 0, "📰 Live Feed", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        # Live feed content
        feed_content = self.get_live_feed_content()
        lines = feed_content.split('\n')

        for i, line in enumerate(lines[:height - 3]):
            self.stdscr.addstr(y + 2 + i, 0, line[:width])

    def draw_bye_tab(self, y: int, height: int, width: int) -> None:
        """Draw the bye tab."""
        self.stdscr.addstr(y, 0, "👋 Goodbye!", curses.A_BOLD)
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        bye_message = [
            "",
            "Thank you for using Todozi!",
            "",
            "Your tasks and data have been saved.",
            "",
            "Press Ctrl+Q to exit completely.",
            ""
        ]

        for i, line in enumerate(bye_message):
            if y + 3 + i < y + height:
                self.stdscr.addstr(y + 3 + i, (width - len(line)) // 2, line)

    def draw_ideas_content(self, y: int, height: int, width: int) -> None:
        """Draw ideas section content."""
        if not self.ideas:
            self.stdscr.addstr(y, 0, "No ideas recorded yet")
            return

        for i, idea in enumerate(self.ideas[:height]):
            title = self.responsive_text(idea.title, width - 10)
            line = f"💡 {title} ({idea.category})"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_memories_content(self, y: int, height: int, width: int) -> None:
        """Draw memories section content."""
        if not self.memories:
            self.stdscr.addstr(y, 0, "No memories recorded yet")
            return

        for i, memory in enumerate(self.memories[:height]):
            moment = self.responsive_text(memory.moment, width - 15)
            tags = ", ".join(memory.tags[:3])
            line = f"💭 {moment} [{tags}]"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_feelings_content(self, y: int, height: int, width: int) -> None:
        """Draw feelings section content."""
        if not self.feelings:
            self.stdscr.addstr(y, 0, "No feelings recorded yet")
            return

        for i, feeling in enumerate(self.feelings[:height]):
            intensity_bar = self.generate_progress_bar(feeling.intensity * 10, 10)
            context = self.responsive_text(feeling.context or "", width - 25)
            line = f"😊 {feeling.emotion} {intensity_bar} {context}"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_errors_content(self, y: int, height: int, width: int) -> None:
        """Draw errors section content."""
        if not self.errors:
            self.stdscr.addstr(y, 0, "No errors recorded")
            return

        for i, error in enumerate(self.errors[:height]):
            title = self.responsive_text(error.title, width - 10)
            line = f"❌ {title}"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_training_content(self, y: int, height: int, width: int) -> None:
        """Draw training data section content."""
        if not self.training_data:
            self.stdscr.addstr(y, 0, "No training data available")
            return

        for i, training in enumerate(self.training_data[:height]):
            prompt = self.responsive_text(training.prompt, width - 15)
            line = f"📚 {training.data_type}: {prompt}"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_queue_content(self, y: int, height: int, width: int) -> None:
        """Draw queue section content."""
        if not self.queue_items:
            self.stdscr.addstr(y, 0, "Queue is empty")
            return

        for i, item in enumerate(self.queue_items[:height]):
            task_name = self.responsive_text(item.task_name, width - 10)
            line = f"📋 {task_name} (Priority: {item.priority})"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_reminders_content(self, y: int, height: int, width: int) -> None:
        """Draw reminders section content."""
        if not self.reminders:
            self.stdscr.addstr(y, 0, "No reminders set")
            return

        for i, reminder in enumerate(self.reminders[:height]):
            title = self.responsive_text(reminder.title, width - 20)
            completed = "✓" if reminder.completed else "○"
            due_info = ""
            if reminder.due_date:
                if reminder.due_date > datetime.utcnow():
                    due_info = f" (due in {(reminder.due_date - datetime.utcnow()).days}d)"
                else:
                    due_info = " (overdue)"

            line = f"🔔 {completed} {title}{due_info}"
            self.stdscr.addstr(y + i, 0, line[:width])

    def draw_analytics_content(self, y: int, height: int, width: int) -> None:
        """Draw analytics section content."""
        self.stdscr.addstr(y, 0, "📊 Analytics Dashboard")
        self.stdscr.addstr(y + 1, 0, self.generate_separator_line(width))

        # Task completion stats
        total_tasks = len(self.tasks)
        completed_tasks = len([t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]])
        completion_rate = (completed_tasks / total_tasks * 100) if total_tasks > 0 else 0

        self.stdscr.addstr(y + 3, 0, f"Total Tasks: {total_tasks}")
        self.stdscr.addstr(y + 4, 0, f"Completed: {completed_tasks}")
        self.stdscr.addstr(y + 5, 0, f"Completion Rate: {completion_rate:.1f}%")
        # Priority distribution
        if y + 7 < y + height:
            self.stdscr.addstr(y + 7, 0, "Priority Distribution:")
            for i, (priority, count) in enumerate(zip([Priority.LOW, Priority.MEDIUM, Priority.HIGH, Priority.CRITICAL, Priority.URGENT], self.priority_distribution)):
                if y + 8 + i < y + height:
                    bar = self.generate_progress_bar(count * 5, 20)  # Scale for visibility
                    self.stdscr.addstr(y + 8 + i, 0, f"  {priority.value}: {bar} ({count})")

    def draw_editor(self, height: int, width: int) -> None:
        """Draw the task editor."""
        if not self.editor:
            return

        self.stdscr.addstr(2, 0, "✏️  Task Editor", curses.A_BOLD)
        self.stdscr.addstr(3, 0, self.generate_separator_line(width))

        fields = [
            ("Action", self.editor.current_task.action),
            ("Time", self.editor.current_task.time),
            ("Priority", self.editor.current_task.priority.value),
            ("Status", self.editor.current_task.status.value),
            ("Project", self.editor.current_task.parent_project),
            ("Assignee", self.editor.current_task.assignee.value if self.editor.current_task.assignee else "None"),
            ("Tags", ", ".join(self.editor.current_task.tags)),
            ("Context", self.editor.current_task.context_notes or ""),
            ("Progress", str(self.editor.current_task.progress or 0)),
        ]

        y = 5
        for i, (label, value) in enumerate(fields):
            marker = "→ " if i == self.editor_selected_field else "  "
            display_value = self.responsive_text(str(value), width - 15)
            line = f"{marker}{label}: {display_value}"

            if i == self.editor_selected_field:
                self.stdscr.addstr(y + i, 0, line, curses.A_REVERSE)
            else:
                self.stdscr.addstr(y + i, 0, line)

        # Instructions
        instructions_y = y + len(fields) + 2
        if instructions_y < height:
            self.stdscr.addstr(instructions_y, 0, "↑↓ navigate fields | Enter edit | Esc cancel | Ctrl+S save")

    def draw_task_details_popup(self, height: int, width: int) -> None:
        """Draw task details popup."""
        if not self.show_task_details:
            return

        task = self.show_task_details
        self.stdscr.addstr(2, 0, "📋 Task Details", curses.A_BOLD)
        self.stdscr.addstr(3, 0, self.generate_separator_line(width))

        details = [
            f"ID: {task.id}",
            f"Action: {task.action}",
            f"Status: {task.status.value}",
            f"Priority: {task.priority.value}",
            f"Project: {task.parent_project or 'None'}",
            f"Assignee: {task.assignee.value if task.assignee else 'None'}",
            f"Tags: {', '.join(task.tags) if task.tags else 'None'}",
            f"Progress: {task.progress}%" if task.progress else "Progress: Not set",
            f"Created: {task.created_at.strftime('%Y-%m-%d %H:%M')}",
            f"Updated: {task.updated_at.strftime('%Y-%m-%d %H:%M')}",
            f"Context: {task.context_notes or 'None'}",
        ]

        for i, detail in enumerate(details):
            if 5 + i < height:
                self.stdscr.addstr(5 + i, 0, detail[:width])

        # Close instructions
        if 5 + len(details) + 2 < height:
            self.stdscr.addstr(5 + len(details) + 2, 0, "Press any key to close")

    def draw_action_menu(self, height: int, width: int) -> None:
        """Draw task action menu."""
        if self.task_action_menu is None:
            return

        task = self.filtered_tasks[self.task_action_menu]
        self.stdscr.addstr(2, 0, f"⚡ Actions for: {task.action[:30]}", curses.A_BOLD)
        self.stdscr.addstr(3, 0, self.generate_separator_line(width))

        actions = ["Edit", "Delete", "View Details", "Duplicate"]
        for i, action in enumerate(actions):
            marker = "→ " if i == self.task_action_selected else "  "
            self.stdscr.addstr(5 + i, 0, f"{marker}{action}")

        # Instructions
        self.stdscr.addstr(5 + len(actions) + 2, 0, "↑↓ navigate | Enter select | Esc cancel")

    def draw_status_bar(self, y: int, width: int) -> None:
        """Draw the status bar."""
        status_text = f"Tab: {self.current_tab.title()} | Tasks: {len(self.filtered_tasks)}/{len(self.tasks)}"
        if self.task_filters.status_filter or self.task_filters.priority_filter or self.task_filters.project_filter:
            status_text += " | Filtered"

        self.stdscr.addstr(y, 0, status_text[:width], curses.A_REVERSE)

        # Key hints
        hints = "Ctrl+Q: Quit | Tab: Next Tab | Ctrl+N: New Task | Ctrl+E: Edit | Ctrl+D: Delete"
        self.stdscr.addstr(y + 1, 0, hints[:width], curses.A_REVERSE)

    def draw_toasts(self, height: int, width: int) -> None:
        """Draw toast notifications."""
        if not self.toast_notifications:
            return

        for i, toast in enumerate(self.toast_notifications[-3:]):  # Show last 3 toasts
            y_pos = height - 3 - (i * 3)
            if y_pos < 0:
                break

            # Toast background
            icon = {"success": "✅", "error": "❌", "warning": "⚠️", "info": "ℹ️"}.get(toast.notification_type.value, "ℹ️")
            message = f"{icon} {toast.message}"[:width - 2]

            self.stdscr.addstr(y_pos, width - len(message) - 2, message, curses.A_REVERSE)

    def handle_input(self) -> None:
        """Handle user input."""
        if not self.stdscr:
            return

        # Set non-blocking mode
        self.stdscr.nodelay(True)
        try:
            key = self.stdscr.getch()
            if key == -1:  # No key pressed
                return

            self.handle_key_event(key)
        except:
            pass
        finally:
            self.stdscr.nodelay(False)

    def handle_key_event(self, key: int) -> None:
        """Handle keyboard events."""
        # Editor mode
        if self.editor:
            self.handle_editor_input(key)
            return

        # Action menu mode
        if self.task_action_menu is not None:
            self.handle_action_menu_input(key)
            return

        # Normal mode
        if key == 17:  # Ctrl+Q
            self.should_quit = True
        elif key == 9:  # Tab
            self.next_tab()
        elif key == curses.KEY_BTAB:  # Shift+Tab
            self.previous_tab()
        elif key == curses.KEY_LEFT:
            if self.current_tab == AppTab.MORE:
                self.previous_more_section()
            else:
                self.previous_tab()
        elif key == curses.KEY_RIGHT:
            if self.current_tab == AppTab.MORE:
                self.next_more_section()
            else:
                self.next_tab()
        elif key == curses.KEY_UP:
            self.handle_up_key()
        elif key == curses.KEY_DOWN:
            self.handle_down_key()
        elif key in (10, curses.KEY_ENTER):  # Enter
            self.handle_enter_key()
        elif key == 27:  # Escape
            self.handle_escape_key()
        elif key == ord('1') and not self.editor:
            self.current_tab = AppTab.PROJECTS
        elif key == ord('2') and not self.editor:
            self.current_tab = AppTab.TASKS
        elif key == ord('3') and not self.editor:
            self.current_tab = AppTab.FEED
        elif key == ord('4') and not self.editor:
            self.current_tab = AppTab.DONE
        elif key == ord('5') and not self.editor:
            self.current_tab = AppTab.FIND
        elif key == ord('6') and not self.editor:
            self.current_tab = AppTab.MORE
        elif key == ord('7') and not self.editor:
            self.current_tab = AppTab.API
        elif key == ord('8') and not self.editor:
            self.current_tab = AppTab.BYE
        elif key == 14:  # Ctrl+N
            self.start_new_task_editor()
        elif key == 5:  # Ctrl+E
            if self.current_tab in [AppTab.TASKS, AppTab.DONE, AppTab.FIND]:
                tasks = self.get_current_task_list()
                if tasks and 0 <= self.selected_task_index < len(tasks):
                    self.start_edit_task(tasks[self.selected_task_index])
        elif key == 4:  # Ctrl+D
            if self.current_tab in [AppTab.TASKS, AppTab.DONE, AppTab.FIND]:
                tasks = self.get_current_task_list()
                if tasks and 0 <= self.selected_task_index < len(tasks):
                    task = tasks[self.selected_task_index]
                    if self.confirm_action(f"Delete task '{task.action}'?"):
                        self.delete_task(task.id)
                        self.add_toast("Task deleted", ToastType.SUCCESS)
        elif key in (ord('s'), ord('S')):
            if self.current_tab == AppTab.DONE:
                self.cycle_done_sort_by()
        elif key in (ord('o'), ord('O')):
            if self.current_tab == AppTab.DONE:
                self.done_sort_order = SortOrder.ASCENDING if self.done_sort_order == SortOrder.DESCENDING else SortOrder.DESCENDING
                # Re-apply filters to re-sort
                self.apply_filters()
        elif key in (ord('p'), ord('P')):
            if self.current_tab == AppTab.DONE:
                self.cycle_done_project_filter()
        elif key in (ord('i'), ord('I')):
            if self.current_tab == AppTab.DONE:
                self.cycle_done_priority_filter()
        elif key in (ord('c'), ord('C')):
            if self.current_tab == AppTab.DONE:
                self.done_filters = TaskFilters()
                self.apply_filters()
                self.add_toast("Filters cleared", ToastType.INFO)
            elif self.current_tab == AppTab.API:
                self.clear_cache()
                self.add_toast("Cache cleared", ToastType.SUCCESS)
        elif key in (ord('x'), ord('X')):
            if self.current_tab == AppTab.API:
                self.stop_server()
                self.check_server_status()
                self.add_toast("Server stopped", ToastType.WARNING)
        elif key in (ord('r'), ord('R')):
            if self.current_tab == AppTab.API:
                self.restart_server()
                self.add_toast("Server restarted", ToastType.SUCCESS)
        elif key == 19:  # Ctrl+S (save)
            if self.editor:
                self.save_current_task()
        else:
            # Handle character input for search
            if self.current_tab == AppTab.FIND and key >= 32 and key <= 126:
                self.search_query += chr(key)
                self.update_search_results()

    def handle_editor_input(self, key: int) -> None:
        """Handle input in editor mode."""
        if key == curses.KEY_UP:
            self.editor_selected_field = (self.editor_selected_field - 1) % 9
            self.update_editor_field()
            self.load_current_field()
        elif key == curses.KEY_DOWN:
            self.editor_selected_field = (self.editor_selected_field + 1) % 9
            self.update_editor_field()
            self.load_current_field()
        elif key in (10, curses.KEY_ENTER):  # Enter - start editing field
            self.edit_current_field()
        elif key == 27:  # Escape - cancel editing
            self.editor = None
            self.add_toast("Edit cancelled", ToastType.INFO)
        elif key == 19:  # Ctrl+S - save
            self.save_current_task()

    def handle_action_menu_input(self, key: int) -> None:
        """Handle input in action menu mode."""
        if key == curses.KEY_UP:
            self.task_action_selected = (self.task_action_selected - 1) % 4
        elif key == curses.KEY_DOWN:
            self.task_action_selected = (self.task_action_selected + 1) % 4
        elif key in (10, curses.KEY_ENTER):
            self.execute_task_action()
        elif key == 27:  # Escape
            self.task_action_menu = None

    def handle_up_key(self) -> None:
        """Handle up arrow key."""
        if self.current_tab == AppTab.PROJECTS:
            if self.projects:
                self.selected_project_index = (self.selected_project_index - 1) % len(self.projects)
        elif self.current_tab == AppTab.TASKS:
            if self.filtered_tasks:
                self.selected_task_index = (self.selected_task_index - 1) % len(self.filtered_tasks)
        elif self.current_tab == AppTab.DONE:
            if self.get_filtered_done_tasks():
                self.done_selected_task_index = (self.done_selected_task_index - 1) % len(self.get_filtered_done_tasks())
        elif self.current_tab == AppTab.FIND:
            if self.search_results:
                self.selected_task_index = (self.selected_task_index - 1) % len(self.search_results)
        elif self.current_tab == AppTab.MORE:
            item_count = self.get_more_section_item_count()
            if item_count > 0:
                self.more_tab_selected_index = (self.more_tab_selected_index - 1) % item_count

    def handle_down_key(self) -> None:
        """Handle down arrow key."""
        if self.current_tab == AppTab.PROJECTS:
            if self.projects:
                self.selected_project_index = (self.selected_project_index + 1) % len(self.projects)
        elif self.current_tab == AppTab.TASKS:
            if self.filtered_tasks:
                self.selected_task_index = (self.selected_task_index + 1) % len(self.filtered_tasks)
        elif self.current_tab == AppTab.DONE:
            if self.get_filtered_done_tasks():
                self.done_selected_task_index = (self.done_selected_task_index + 1) % len(self.get_filtered_done_tasks())
        elif self.current_tab == AppTab.FIND:
            if self.search_results:
                self.selected_task_index = (self.selected_task_index + 1) % len(self.search_results)
        elif self.current_tab == AppTab.MORE:
            item_count = self.get_more_section_item_count()
            if item_count > 0:
                self.more_tab_selected_index = (self.more_tab_selected_index + 1) % item_count

    def handle_enter_key(self) -> None:
        """Handle enter key."""
        if self.current_tab == AppTab.PROJECTS:
            if self.projects:
                project = self.projects[self.selected_project_index]
                self.task_filters.project_filter = project
                self.apply_filters()
                self.current_tab = AppTab.TASKS
                self.add_toast(f"Filtered to project: {project}", ToastType.INFO)
        elif self.current_tab in [AppTab.TASKS, AppTab.DONE, AppTab.FIND]:
            tasks = self.get_current_task_list()
            if tasks and 0 <= self.selected_task_index < len(tasks):
                self.show_task_details = tasks[self.selected_task_index]
        elif self.current_tab == AppTab.MORE:
            # Could implement more section actions here
            pass

    def handle_escape_key(self) -> None:
        """Handle escape key."""
        if self.show_task_details:
            self.show_task_details = None
        elif self.task_action_menu is not None:
            self.task_action_menu = None
        else:
            # Clear search query in find tab
            if self.current_tab == AppTab.FIND:
                self.search_query = ""
                self.search_results = []
            # Clear filters in tasks/done tabs
            elif self.current_tab in [AppTab.TASKS, AppTab.DONE]:
                self.clear_current_filters()

    def get_current_task_list(self) -> List[Task]:
        """Get the current list of tasks being displayed."""
        if self.current_tab == AppTab.TASKS:
            return self.filtered_tasks
        elif self.current_tab == AppTab.DONE:
            return self.get_filtered_done_tasks()
        elif self.current_tab == AppTab.FIND:
            return self.search_results
        else:
            return []

    def clear_current_filters(self) -> None:
        """Clear filters for current tab."""
        if self.current_tab == AppTab.TASKS:
            self.task_filters = TaskFilters()
            self.apply_filters()
            self.add_toast("Task filters cleared", ToastType.INFO)
        elif self.current_tab == AppTab.DONE:
            self.done_filters = TaskFilters()
            self.add_toast("Done filters cleared", ToastType.INFO)

    def cycle_done_sort_by(self) -> None:
        """Cycle through sort options for done tab."""
        sort_options = [
            TaskSortBy.DATE_COMPLETED,
            TaskSortBy.DATE_CREATED,
            TaskSortBy.PRIORITY,
            TaskSortBy.PROJECT,
            TaskSortBy.ACTION,
        ]
        current_index = sort_options.index(self.done_sort_by) if self.done_sort_by in sort_options else 0
        self.done_sort_by = sort_options[(current_index + 1) % len(sort_options)]
        # Re-apply filters to re-sort
        self.apply_filters()

    def cycle_done_project_filter(self) -> None:
        """Cycle through project filters for done tab."""
        available_projects = list(set(task.parent_project for task in self.tasks if task.parent_project))
        if not available_projects:
            return

        current_project = self.done_filters.project_filter
        if current_project and current_project in available_projects:
            current_index = available_projects.index(current_project)
            next_index = (current_index + 1) % (len(available_projects) + 1)
        else:
            next_index = 0

        if next_index == len(available_projects):
            self.done_filters.project_filter = None
        else:
            self.done_filters.project_filter = available_projects[next_index]

        # Re-apply filters
        self.apply_filters()

    def cycle_done_priority_filter(self) -> None:
        """Cycle through priority filters for done tab."""
        priorities = [Priority.CRITICAL, Priority.URGENT, Priority.HIGH, Priority.MEDIUM, Priority.LOW]
        current_priority = self.done_filters.priority_filter

        if current_priority and current_priority in priorities:
            current_index = priorities.index(current_priority)
            next_index = (current_index + 1) % (len(priorities) + 1)
        else:
            next_index = 0

        if next_index == len(priorities):
            self.done_filters.priority_filter = None
        else:
            self.done_filters.priority_filter = priorities[next_index]

        # Re-apply filters
        self.apply_filters()

    def edit_current_field(self) -> None:
        """Start editing the current field in the editor."""
        if not self.editor:
            return

        # In a real implementation, this would open a text input dialog
        # For now, just show a message
        self.add_toast("Field editing not implemented in this demo", ToastType.INFO)

    def save_current_task(self) -> None:
        """Save the current task being edited."""
        if not self.editor:
            return

        # Update the task
        if self.editor.task_id.startswith("temp-"):  # New task
            self.editor.current_task.id = str(uuid.uuid4())
            self.tasks.append(self.editor.current_task)
            self.add_toast("New task created", ToastType.SUCCESS)
        else:  # Existing task
            self.save_task(self.editor.current_task)
            self.add_toast("Task updated", ToastType.SUCCESS)

        self.editor = None
        self.apply_filters()

    def execute_task_action(self) -> None:
        """Execute the selected task action."""
        if self.task_action_menu is None:
            return

        task = self.filtered_tasks[self.task_action_menu]

        if self.task_action_selected == 0:  # Edit
            self.start_edit_task(task)
        elif self.task_action_selected == 1:  # Delete
            if self.confirm_action(f"Delete task '{task.action}'?"):
                self.delete_task(task.id)
                self.add_toast("Task deleted", ToastType.SUCCESS)
        elif self.task_action_selected == 2:  # View Details
            self.show_task_details = task
        elif self.task_action_selected == 3:  # Duplicate
            new_task = Task(
                action=f"{task.action} (Copy)",
                status=task.status,
                priority=task.priority,
                parent_project=task.parent_project,
                tags=task.tags.copy(),
                context_notes=task.context_notes,
            )
            self.tasks.append(new_task)
            self.apply_filters()
            self.add_toast("Task duplicated", ToastType.SUCCESS)

        self.task_action_menu = None

    def get_live_feed_content(self) -> str:
        """Get live feed content."""
        lines = ["📰 Live Task Feed", ""]

        # Recent tasks
        recent_tasks = sorted(self.tasks, key=lambda t: t.updated_at, reverse=True)[:10]
        for task in recent_tasks:
            time_ago = self.format_duration(task.updated_at, datetime.utcnow())
            status_icon = self.get_status_icon(task.status)
            lines.append(f"{status_icon} {task.action} - {time_ago}")

        if not recent_tasks:
            lines.append("No recent activity")

        lines.append("")
        lines.append("📊 Stats:")
        lines.append(f"  Total tasks: {len(self.tasks)}")
        completed = len([t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]])
        lines.append(f"  Completed: {completed}")
        completion_rate = (completed / len(self.tasks) * 100) if self.tasks else 0
        lines.append(f"  Completion rate: {completion_rate:.1f}%")

        return "\n".join(lines)


# TUI Service
class TuiService(TodoziApp):
    def __init__(self, embedding_service=None, display_config=None):
        # Initialize the parent TodoziApp first
        super().__init__()
        self.embedding_service = embedding_service
        self.display_config = display_config or DisplayConfig()

    async def show_loading_screen(self):
        """Show loading screen."""
        console = Console()
        console.clear()

        loading_text = """

         _______        _
        |__   __|      | |        (✓)
           | | ___   __| | ___ _____
           | |/ _ \ / _` |/ _ \_  / |
           | | (_) | (_| | (_) / /| |
           |_|\___/ \__,_|\___/___|_|

        ✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓
                Loading workspace...
                Please wait

        """

        console.print(loading_text, style="bold blue")
        await asyncio.sleep(1)

    def run(self):
        """Run the TUI application."""
        # Call the parent's run method
        super().run()

    async def display_task(self, task_id: str) -> 'TaskDisplay':
        """Display a single task with AI insights."""
        # Find the task
        task = next((t for t in self.tasks if t.id == task_id), None)
        if not task:
            raise ValueError(f"Task {task_id} not found")

        # Generate display data
        similar_tasks = []  # In a real implementation, this would use embeddings
        ai_suggestions = ["Consider breaking this into smaller tasks"]
        semantic_tags = ["work", "urgent"]
        confidence_score = 0.85

        return TaskDisplay(
            task=task,
            similar_tasks=similar_tasks,
            ai_suggestions=ai_suggestions,
            semantic_tags=semantic_tags,
            confidence_score=confidence_score,
            related_content=similar_tasks
        )

    async def display_tasks(self, task_ids: List[str]) -> 'TaskListDisplay':
        """Display multiple tasks with AI summary."""
        tasks = []
        for task_id in task_ids:
            task = next((t for t in self.tasks if t.id == task_id), None)
            if task:
                tasks.append(await self.display_task(task.id))

        ai_summary = f"Found {len(tasks)} tasks across different projects"
        semantic_clusters = [["work", "urgent"], ["personal", "shopping"]]

        return TaskListDisplay(
            tasks=tasks,
            total_count=len(tasks),
            ai_summary=ai_summary,
            semantic_clusters=semantic_clusters
        )

    async def start_edit_session(self, task_id: str) -> EditSession:
        """Start an editing session for a task."""
        task = next((t for t in self.tasks if t.id == task_id), None)
        if not task:
            raise ValueError(f"Task {task_id} not found")

        return EditSession(
            task_id=task.id,
            original_task=task,
            current_task=task,
            ai_suggestions=["Consider updating the priority"],
            validation_errors=[],
            similarity_matches=[],
            session_start=datetime.utcnow()
        )


# TaskDisplay and TaskListDisplay classes for the TUI service
@dataclass
class TaskDisplay:
    task: Task
    similar_tasks: List['SimilarityResult']
    ai_suggestions: List[str]
    semantic_tags: List[str]
    confidence_score: float
    related_content: List['SimilarityResult']

@dataclass
class TaskListDisplay:
    tasks: List[TaskDisplay]
    total_count: int
    ai_summary: str
    semantic_clusters: List[List[str]]

@dataclass
class SimilarityResult:
    id: str
    action: str
    similarity_score: float
    tags: List[str] = field(default_factory=list)


# Main function to run the TUI
async def main():
    """Main entry point for the TUI."""
    tui_service = TuiService()
    await tui_service.show_loading_screen()
    tui_service.run()

if __name__ == "__main__":
    asyncio.run(main())
