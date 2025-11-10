#!/usr/bin/env python3
"""
Todozi TUI - A comprehensive, feature-rich terminal UI
Enhanced with AI insights, analytics, and full Todozi integration
"""

from __future__ import annotations

import sys
from pathlib import Path

# Add parent directory to path so we can import todozi modules
# This allows running: python3 todozi/tui.py or python3 -m todozi.tui
# Get the directory containing this file (todozi/)
_current_file = Path(__file__).resolve()
_current_dir = _current_file.parent
# Get the parent directory (tdz_py/)
_parent_dir = _current_dir.parent
# Add it to sys.path if not already there
if str(_parent_dir) not in sys.path:
    sys.path.insert(0, str(_parent_dir))

import argparse
import asyncio
import json
import logging
import threading
import time
import uuid

# Suppress warnings before imports
import warnings
from collections import deque
from dataclasses import dataclass, field
from datetime import datetime, timedelta, timezone
from enum import Enum, auto
from pathlib import Path
from typing import Any, Callable, Deque, Dict, List, Optional, Union

warnings.filterwarnings("ignore")

try:
    import urllib3

    urllib3.disable_warnings(urllib3.exceptions.NotOpenSSLWarning)
except ImportError:
    pass

# Optional imports for enhanced features
try:
    import watchfiles

    WATCHDOG_AVAILABLE = True
except ImportError:
    WATCHDOG_AVAILABLE = False

try:
    import requests

    REQUESTS_AVAILABLE = True
except ImportError:
    REQUESTS_AVAILABLE = False

# Enhancements are now integrated into the main TUI
ENHANCEMENTS_AVAILABLE = True

# Module-level logger
logger = logging.getLogger(__name__)

# Rich imports for enhanced UI
from rich import box
from rich.align import Align
from rich.columns import Columns
from rich.console import Console
from rich.panel import Panel
from rich.progress import BarColumn, Progress, TextColumn
from rich.style import Style
from rich.table import Table
from rich.text import Text

# Textual imports
from textual import on, work
from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import (
    Container,
    Grid,
    Horizontal,
    ScrollableContainer,
    Vertical,
)
from textual.css.query import NoMatches
from textual.message import Message
from textual.reactive import reactive
from textual.screen import ModalScreen, Screen
from textual.widgets import (
    Button,
    Checkbox,
    Collapsible,
    DataTable,
    Footer,
    Header,
    Input,
    Label,
    ListItem,
    ListView,
    LoadingIndicator,
    Log,
    OptionList,
    Placeholder,
    ProgressBar,
    RichLog,
    Select,
    SelectionList,
    Sparkline,
    Static,
    TabbedContent,
    TabPane,
    Tabs,
    TextArea,
)

from todozi.agent import AgentManager
from todozi.api import create_api_key, list_api_keys
from todozi.error import ErrorManager, TodoziError
from todozi.idea import IdeaManager
from todozi.memory import MemoryManager
from todozi.storage import (
    Assignee,
    Error,
    Idea,
    Memory,
    Priority,
    Project,
    Status,
    Storage,
    Task,
    TaskFilters,
    TaskUpdate,
)

# For compatibility with Result type if needed
try:
    from todozi.models import Ok
except ImportError:

    class Ok:
        def __init__(self, value):
            self.value = value


# Add uppercase aliases to storage enums for backward compatibility
# This allows old code using Status.TODO to work with storage.py's Status.Todo
if not hasattr(Status, "TODO"):
    Status.TODO = Status.Todo
    Status.PENDING = Status.Todo  # Map pending to todo
    Status.IN_PROGRESS = Status.InProgress
    Status.DONE = Status.Done
    Status.COMPLETED = Status.Completed
    Status.CANCELLED = Status.Cancelled
    Status.ARCHIVED = Status.Archived

if not hasattr(Priority, "LOW"):
    Priority.LOW = Priority.Low
    Priority.MEDIUM = Priority.Medium
    Priority.HIGH = Priority.High
    Priority.CRITICAL = Priority.Critical
    Priority.URGENT = Priority.Urgent

if not hasattr(Assignee, "HUMAN"):
    Assignee.HUMAN = Assignee.Human
    Assignee.AI = Assignee.Ai
    Assignee.COLLABORATIVE = Assignee.Collaborative


class FullTaskEditorScreen(Container):
    """Full field-by-field task editor screen matching Rust TUI - can be used as overlay"""

    def __init__(self, task: Task, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.task = task
        self.original_task = task
        self.field_index = 0
        self.editor_input = ""
        self.fields = [
            ("action", "Action", str),
            ("time", "Time", str),
            ("priority", "Priority", Priority),
            ("status", "Status", Status),
            ("parent_project", "Project", str),
            ("assignee", "Assignee", Optional[Assignee]),
            ("tags", "Tags", List[str]),
            ("context_notes", "Context", Optional[str]),
            ("progress", "Progress", Optional[int]),
        ]
        self._dismiss_event = None

    def wait_for_dismiss(self):
        """Wait for editor to be dismissed and return result"""
        from asyncio import Future

        self._dismiss_future = Future()
        return self._dismiss_future

    def dismiss(self, result):
        """Dismiss editor with result"""
        if (
            hasattr(self, "_dismiss_future")
            and self._dismiss_future
            and not self._dismiss_future.done()
        ):
            self._dismiss_future.set_result(result)

    def compose(self):
        with Container(id="editor-container", classes="editor-fullscreen"):
            yield Label(f"✏️ Task Editor - {self.task.id[:8]}", id="editor-header")
            yield Static(
                "ESC: Save & Close | ↑↓: Navigate Fields | Enter: Next Field",
                id="editor-help",
            )

            with Vertical(id="fields-container"):
                for i, (field_name, field_label, _) in enumerate(self.fields):
                    yield Static(
                        f"{'▶' if i == self.field_index else ' '} {i + 1}. {field_label}: {self._get_field_value(field_name)}",
                        id=f"field-{i}",
                        classes="field-line",
                    )

            with Container(id="input-container"):
                yield Label("", id="current-field-label")
                yield Input(value="", id="field-input", placeholder="Type to edit...")

            yield Static(
                "Type to edit current field | Enter: Next field | Backspace: Delete | ESC: Save & Close",
                id="editor-actions",
            )

    def on_mount(self):
        self.update_field_display()
        try:
            self.query_one("#field-input", Input).focus()
        except Exception:
            pass

    def _get_field_value(self, field_name: str) -> str:
        """Get the current value of a field"""
        value = getattr(self.task, field_name, None)
        if field_name == "priority" and hasattr(value, "value"):
            return value.value
        elif field_name == "status" and hasattr(value, "value"):
            return value.value
        elif field_name == "assignee":
            if value is None:
                return "None"
            return str(value)
        elif field_name == "tags" and isinstance(value, list):
            return ", ".join(value)
        elif field_name == "progress" and value is not None:
            return f"{value}%"
        return str(value) if value else ""

    def update_field_display(self):
        """Update all field displays"""
        for i, (field_name, field_label, _) in enumerate(self.fields):
            try:
                field_widget = self.query_one(f"#field-{i}", Static)
                marker = "▶" if i == self.field_index else " "
                value = self._get_field_value(field_name)
                field_widget.update(f"{marker} {i + 1}. {field_label}: {value}")
            except Exception:
                pass

        # Update input
        field_name, field_label, field_type = self.fields[self.field_index]
        try:
            label_widget = self.query_one("#current-field-label", Label)
            input_widget = self.query_one("#field-input", Input)

            label_widget.update(f"✏️ {field_label}")
            input_widget.value = self._get_field_value(field_name)

            # Add hints based on field type
            if field_name == "priority":
                input_widget.placeholder = "low/medium/high/critical/urgent"
            elif field_name == "status":
                input_widget.placeholder = (
                    "todo/pending/inprogress/blocked/review/done/cancelled/deferred"
                )
            elif field_name == "assignee":
                input_widget.placeholder = "human/ai/collaborative"
            elif field_name == "tags":
                input_widget.placeholder = "comma-separated tags"
            elif field_name == "progress":
                input_widget.placeholder = "0-100"
            else:
                input_widget.placeholder = "Type to edit..."
        except Exception:
            pass

    @on(Input.Submitted, "#field-input")
    def on_input_submitted(self):
        self.save_current_field()
        self.action_next_field()

    def save_current_field(self):
        """Save the current field value"""
        field_name, _, field_type = self.fields[self.field_index]
        input_widget = self.query_one("#field-input", Input)
        value = input_widget.value

        if field_name == "priority":
            priority_map = {
                "low": Priority.LOW,
                "medium": Priority.MEDIUM,
                "high": Priority.HIGH,
                "critical": Priority.CRITICAL,
                "urgent": Priority.URGENT,
            }
            setattr(
                self.task, field_name, priority_map.get(value.lower(), Priority.MEDIUM)
            )
        elif field_name == "status":
            status_map = {
                "todo": Status.TODO,
                "pending": Status.PENDING,
                "in_progress": Status.IN_PROGRESS,
                "inprogress": Status.IN_PROGRESS,
                "blocked": Status.BLOCKED,
                "review": Status.REVIEW,
                "done": Status.DONE,
                "completed": Status.COMPLETED,
                "cancelled": Status.CANCELLED,
                "canceled": Status.CANCELLED,
                "deferred": Status.DEFERRED,
            }
            setattr(self.task, field_name, status_map.get(value.lower(), Status.TODO))
        elif field_name == "assignee":
            assignee_map = {
                "human": Assignee.HUMAN,
                "ai": Assignee.AI,
                "collaborative": Assignee.COLLABORATIVE,
            }
            setattr(self.task, field_name, assignee_map.get(value.lower(), None))
        elif field_name == "tags":
            setattr(
                self.task,
                field_name,
                [tag.strip() for tag in value.split(",") if tag.strip()],
            )
        elif field_name == "progress":
            try:
                progress = max(0, min(100, int(value)))
                setattr(self.task, field_name, progress)
            except ValueError:
                pass
        else:
            setattr(self.task, field_name, value)

        self.update_field_display()

    def action_previous_field(self):
        """Move to previous field"""
        self.save_current_field()
        self.field_index = (self.field_index - 1) % len(self.fields)
        self.update_field_display()

    def action_next_field(self):
        """Move to next field"""
        self.save_current_field()
        self.field_index = (self.field_index + 1) % len(self.fields)
        self.update_field_display()

    def action_save_and_close(self):
        """Save and close editor"""
        self.save_current_field()
        if hasattr(self, "dismiss"):
            self.dismiss(self.task)
        elif hasattr(self, "parent") and self.parent:
            # Remove from parent
            try:
                self.remove()
            except Exception:
                pass


class TaskDetailsWithStepsModal(ModalScreen):
    """Task details modal with steps support matching Rust TUI"""

    def __init__(self, task: Task):
        super().__init__()
        self.task = task

    def compose(self):
        with Container(id="task-details-modal"):
            yield Label(f"📋 Task Details: {self.task.action[:50]}", id="details-title")
            with ScrollableContainer(id="details-content"):
                yield Static(self._format_task_details(), id="details-text")
            yield Button("Close", id="close-btn", variant="primary")

    def _format_task_details(self) -> str:
        """Format task details with steps support"""
        details = f"""
[bold]Task ID:[/bold] {self.task.id}
[bold]Action:[/bold] {self.task.action}
[bold]Status:[/bold] {self.task.status.value if hasattr(self.task.status, "value") else str(self.task.status)}
[bold]Priority:[/bold] {self.task.priority.value if hasattr(self.task.priority, "value") else str(self.task.priority)}
[bold]Project:[/bold] {getattr(self.task, "parent_project", "N/A")}
[bold]Assignee:[/bold] {getattr(self.task, "assignee", "Unassigned")}
[bold]Time Estimate:[/bold] {getattr(self.task, "time", "N/A")}
[bold]Progress:[/bold] {getattr(self.task, "progress", 0)}%
[bold]Tags:[/bold] {", ".join(getattr(self.task, "tags", []) or [])}
[bold]Created:[/bold] {getattr(self.task, "created_at", datetime.now()).strftime("%Y-%m-%d %H:%M")}
[bold]Updated:[/bold] {getattr(self.task, "updated_at", datetime.now()).strftime("%Y-%m-%d %H:%M")}
"""

        context = getattr(self.task, "context_notes", None)
        if context:
            details += f"\n[bold]Context:[/bold]\n{context}\n"

        # Load and display steps
        try:
            from todozi.storage import load_task_steps

            steps_data = load_task_steps(self.task.id)
            if steps_data:
                if isinstance(steps_data, dict):
                    # Handle steps data
                    if "summary" in steps_data and steps_data["summary"]:
                        details += (
                            f"\n[bold]📝 Summary:[/bold]\n{steps_data['summary']}\n"
                        )

                    if "steps" in steps_data and isinstance(steps_data["steps"], list):
                        details += "\n[bold]📌 Steps:[/bold]\n"
                        for i, step in enumerate(steps_data["steps"], 1):
                            step_str = step if isinstance(step, str) else str(step)
                            details += f"  {i}. {step_str}\n"

                    if "status" in steps_data:
                        details += (
                            f"\n[bold]Steps Status:[/bold] {steps_data['status']}\n"
                        )
        except Exception as e:
            pass  # Steps not available

        return details

    @on(Button.Pressed, "#close-btn")
    def on_close(self):
        self.dismiss()


class ChartWidget(Static):
    """Simple bar chart widget for Textual"""

    def __init__(self, title: str, data: List[tuple], *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.title = title
        self.data = data
        self.update_chart()

    def update_chart(self):
        """Update the chart display"""
        if not self.data:
            self.update(f"[bold]{self.title}[/bold]\nNo data")
            return

        max_value = max(v for _, v in self.data) if self.data else 1
        max_value = max(max_value, 1)
        max_bar_width = 30

        lines = [f"[bold]{self.title}[/bold]", ""]
        for label, value in self.data:
            bar_width = int((value / max_value) * max_bar_width) if max_value > 0 else 0
            bar = "█" * bar_width
            lines.append(f"{label:12} {bar} {value}")

        self.update("\n".join(lines))


class FeedChartsWidget(Container):
    """Widget container for feed tab charts"""

    def __init__(
        self,
        tasks: List[Task],
        ideas: List,
        memories: List,
        queue_items: List,
        training_data: List,
        *args,
        **kwargs,
    ):
        super().__init__(*args, **kwargs)
        self.tasks = tasks
        self.ideas = ideas
        self.memories = memories
        self.queue_items = queue_items
        self.training_data = training_data

    def compose(self):
        with Horizontal():
            # Task Status Chart
            status_data = self._get_task_status_data()
            yield ChartWidget("📊 Task Status", status_data, id="status-chart")

            # Content Stats Chart
            content_data = self._get_content_stats_data()
            yield ChartWidget("💡 Content", content_data, id="content-chart")

            # Priority Distribution Chart
            priority_data = self._get_priority_data()
            yield ChartWidget("🎯 Priority", priority_data, id="priority-chart")

    def _get_task_status_data(self) -> List[tuple]:
        """Get task status distribution"""
        status_counts = {"Todo": 0, "Progress": 0, "Blocked": 0, "Review": 0, "Done": 0}
        for task in self.tasks:
            if task.status == Status.TODO or task.status == Status.PENDING:
                status_counts["Todo"] += 1
            elif task.status == Status.IN_PROGRESS:
                status_counts["Progress"] += 1
            elif task.status == Status.BLOCKED:
                status_counts["Blocked"] += 1
            elif task.status == Status.REVIEW:
                status_counts["Review"] += 1
            elif task.status in [Status.DONE, Status.COMPLETED]:
                status_counts["Done"] += 1
        return list(status_counts.items())

    def _get_content_stats_data(self) -> List[tuple]:
        """Get content statistics"""
        return [
            ("Ideas", len(self.ideas)),
            ("Memory", len(self.memories)),
            ("Queue", len(self.queue_items)),
            ("Train", len(self.training_data)),
        ]

    def _get_priority_data(self) -> List[tuple]:
        """Get priority distribution"""
        priority_counts = {"Crit": 0, "Urg": 0, "High": 0, "Med": 0, "Low": 0}
        for task in self.tasks:
            if task.priority == Priority.CRITICAL:
                priority_counts["Crit"] += 1
            elif task.priority == Priority.URGENT:
                priority_counts["Urg"] += 1
            elif task.priority == Priority.HIGH:
                priority_counts["High"] += 1
            elif task.priority == Priority.MEDIUM:
                priority_counts["Med"] += 1
            elif task.priority == Priority.LOW:
                priority_counts["Low"] += 1
        return list(priority_counts.items())


def load_task_steps(task_id: str) -> Optional[Dict[str, Any]]:
    """Load task steps from storage"""
    try:
        from todozi.storage import get_storage_dir

        storage_dir = get_storage_dir()
        if not storage_dir:
            return None

        steps_file = storage_dir / "tasks" / "steps" / f"{task_id}.json"
        if steps_file.exists():
            with open(steps_file, "r") as f:
                return json.load(f)
    except Exception:
        pass
    return None


# Enhanced models for comprehensive TUI
@dataclass
class SimilarityResult:
    """Result from semantic similarity search."""

    id: str
    action: str
    similarity_score: float
    tags: List[str] = field(default_factory=list)


@dataclass
class TaskDisplay:
    """Enhanced task display with AI insights."""

    task: Task
    similar_tasks: List[SimilarityResult] = field(default_factory=list)
    ai_suggestions: List[str] = field(default_factory=list)
    semantic_tags: List[str] = field(default_factory=list)
    confidence_score: float = 0.0
    related_content: List[SimilarityResult] = field(default_factory=list)


@dataclass
class TaskListDisplay:
    """Enhanced task list display with analytics."""

    tasks: List[TaskDisplay]
    total_count: int
    ai_summary: str = ""
    semantic_clusters: List[List[str]] = field(default_factory=list)


@dataclass
class ApiKey:
    """API key model for management."""

    user_id: str
    active: bool = True
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    last_used: Optional[datetime] = None


@dataclass
class QueueItem:
    """Queue item for task processing."""

    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    task_name: str = ""
    priority: int = 0
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


@dataclass
class Reminder:
    """Reminder model."""

    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str = ""
    message: str = ""
    due_date: Optional[datetime] = None
    completed: bool = False
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


@dataclass
class TrainingData:
    """Training data model."""

    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    data_type: str = ""
    prompt: str = ""
    response: str = ""
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


@dataclass
class Feeling:
    """Feeling/emotion tracking model."""

    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    emotion: str = ""
    intensity: int = 5
    context: Optional[str] = None
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


@dataclass
class ActivityEntry:
    """Activity feed entry."""

    timestamp: datetime
    message: str
    level: str = "info"  # info, success, error, warning


# Optional file watching
try:
    from watchdog.events import (
        FileCreatedEvent,
        FileDeletedEvent,
        FileModifiedEvent,
        FileSystemEventHandler,
    )
    from watchdog.observers import Observer

    WATCHDOG_AVAILABLE = True
except Exception:
    WATCHDOG_AVAILABLE = False

    # Create stub classes if watchdog is not available
    class FileSystemEventHandler:
        pass

    class Observer:
        def __init__(self):
            pass

        def schedule(self, *args, **kwargs):
            pass

        def start(self):
            pass

        def stop(self):
            pass

        def join(self, *args, **kwargs):
            pass

        def is_alive(self):
            return False

    class FileCreatedEvent:
        pass

    class FileDeletedEvent:
        pass

    class FileModifiedEvent:
        pass


UTC = timezone.utc


def utcnow() -> datetime:
    return datetime.now(UTC)


# Enhanced enums for comprehensive TUI
class AppTab(Enum):
    Projects = "📁 Projects"
    Tasks = "📋 Tasks"
    Done = "✅ Done"
    Find = "🔍 Find"
    More = "🔮 More"
    Api = "🔑 API"
    Feed = "📰 Feed"
    Bye = "👋 Bye"

    @staticmethod
    def all() -> List["AppTab"]:
        return [
            AppTab.Projects,
            AppTab.Tasks,
            AppTab.Done,
            AppTab.Find,
            AppTab.More,
            AppTab.Api,
            AppTab.Feed,
            AppTab.Bye,
        ]


class MoreTabSection(Enum):
    Ideas = "💡 Ideas"
    Memories = "🧠 Memories"
    Feelings = "😊 Feelings"
    Errors = "❌ Errors"
    Training = "🎓 Training"
    Queue = "📋 Queue"
    Reminders = "🔔 Reminders"
    Analytics = "📊 Analytics"

    @staticmethod
    def all() -> List["MoreTabSection"]:
        return [
            MoreTabSection.Ideas,
            MoreTabSection.Memories,
            MoreTabSection.Feelings,
            MoreTabSection.Errors,
            MoreTabSection.Training,
            MoreTabSection.Queue,
            MoreTabSection.Reminders,
            MoreTabSection.Analytics,
        ]


class TaskAction(Enum):
    Edit = "✏️ Edit"
    Delete = "🗑️ Delete"
    ViewDetails = "👀 View Details"
    Duplicate = "📋 Duplicate"
    Complete = "✅ Complete"
    MoveToProject = "📁 Move to Project"


class TaskSortBy(Enum):
    DateCompleted = auto()
    DateCreated = auto()
    Priority = auto()
    Project = auto()
    Action = auto()
    Time = auto()
    Assignee = auto()

    def title(self) -> str:
        return {
            TaskSortBy.DateCompleted: "Date Completed",
            TaskSortBy.DateCreated: "Date Created",
            TaskSortBy.Priority: "Priority",
            TaskSortBy.Project: "Project",
            TaskSortBy.Action: "Task",
            TaskSortBy.Time: "Time",
            TaskSortBy.Assignee: "Assignee",
        }[self]

    @staticmethod
    def all() -> List["TaskSortBy"]:
        return list(TaskSortBy)


class SortOrder(Enum):
    Ascending = auto()
    Descending = auto()


class EditorField(Enum):
    Action = auto()
    Time = auto()
    Priority = auto()
    Status = auto()
    Project = auto()
    Assignee = auto()
    Tags = auto()
    Context = auto()
    Progress = auto()


class ToastType(Enum):
    Success = auto()
    Error = auto()
    Warning = auto()
    Info = auto()


@dataclass
class ToastNotification:
    message: str
    notification_type: ToastType
    created_at: float = field(default_factory=time.time)
    duration: float = 5.0

    def is_expired(self, now: float | None = None) -> bool:
        """Check if the toast has expired."""
        now = now if now is not None else time.time()
        return now - self.created_at > self.duration


class ToastType(Enum):
    Success = "success"
    Error = "error"
    Warning = "warning"
    Info = "info"


# Enhanced display configuration
@dataclass
class DisplayConfig:
    show_ai_insights: bool = True
    show_similarity_scores: bool = True
    show_related_tasks: bool = True
    max_related_tasks: int = 5
    compact_mode: bool = False
    show_embeddings: bool = False
    show_ids: bool = False
    show_created_at: bool = False
    show_dependencies: bool = False
    show_context: bool = False
    show_progress: bool = True


@dataclass
class ActivityEntry:
    """An entry in the activity feed."""

    timestamp: datetime
    message: str
    level: str = "info"  # info, success, error, warning


@dataclass
class EditSession:
    task_id: str
    original_task: Task
    current_task: Task
    ai_suggestions: List[str] = field(default_factory=list)
    validation_errors: List[str] = field(default_factory=list)
    session_start: datetime = field(default_factory=utcnow)


@dataclass
class TaskFilters:
    status_filter: Optional[Status] = None
    priority_filter: Optional[Priority] = None
    project_filter: Optional[str] = None
    assignee_filter: Optional[Assignee] = None


def format_duration(from_dt: datetime, to_dt: datetime) -> str:
    delta = to_dt - from_dt
    seconds = int(delta.total_seconds())
    if seconds < 60:
        return f"{max(1, seconds)}s ago"
    minutes = seconds // 60
    if minutes < 60:
        return f"{minutes}m ago"
    hours = minutes // 60
    if hours < 24:
        return f"{hours}h ago"
    days = hours // 24
    if days < 7:
        return f"{days}d ago"
    weeks = days // 7
    return f"{weeks}w ago"


class ToastNotifier:
    """Centralized toast notification management."""

    def __init__(self, parent_app: "TodoziTUI"):
        self.parent = parent_app

    def _make(self, message: str, toast_type: ToastType) -> None:
        """Create and schedule a toast notification."""
        toast = ToastNotification(message=message, notification_type=toast_type)
        self.parent.toast_notifications.append(toast)
        # Use set_timer for better resource management
        self.parent.set_timer(toast.duration, lambda: self._remove_toast(toast))

    def _remove_toast(self, toast: ToastNotification) -> None:
        """Remove a toast notification safely."""
        if toast in self.parent.toast_notifications:
            self.parent.toast_notifications.remove(toast)

    def success(self, message: str) -> None:
        """Show success toast."""
        self._make(message, ToastType.Success)

    def info(self, message: str) -> None:
        """Show info toast."""
        self._make(message, ToastType.Info)

    def warning(self, message: str) -> None:
        """Show warning toast."""
        self._make(message, ToastType.Warning)

    def error(self, message: str) -> None:
        """Show error toast."""
        self._make(message, ToastType.Error)


# Enhanced widgets for comprehensive TUI
class RichLogWidget(RichLog):
    """Enhanced RichLog widget with better styling."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.max_lines = 1000


class TaskActionMenu(ModalScreen):
    """Modal screen for task actions."""

    def __init__(self, task: Task, actions: List[TaskAction]):
        super().__init__()
        self.task = task
        self.actions = actions
        self.selected_index = 0

    def compose(self) -> ComposeResult:
        with Container(id="action-menu"):
            yield Label(f"Actions for: {self.task.action[:50]}", id="menu-title")
            with SelectionList(id="action-list") as selection_list:
                for action in self.actions:
                    selection_list.add_option((action.value, action))

    @on(SelectionList.SelectionHighlighted)
    def on_selection_highlighted(
        self, event: SelectionList.SelectionHighlighted
    ) -> None:
        self.selected_index = event.selection_list.highlighted

    @on(SelectionList.OptionSelected)
    def on_option_selected(self, event: SelectionList.OptionSelected) -> None:
        # Get the selected option index
        selected = event.selection_list.highlighted
        if selected is not None and 0 <= selected < len(self.actions):
            self.dismiss((self.actions[selected], self.task))
        else:
            self.dismiss(None)

    def key_escape(self) -> None:
        self.dismiss(None)


class TaskDetailsModal(ModalScreen):
    """Modal screen for detailed task view."""

    def __init__(self, task: Task):
        super().__init__()
        self.task = task

    def compose(self) -> ComposeResult:
        with Container(id="task-details"):
            yield Label(f"📋 {self.task.action}", id="details-title")
            yield Static(self._format_task_details(), id="details-content")
            yield Button("Close", id="close-btn", variant="primary")

    def _format_task_details(self) -> str:
        """Format task details for display."""
        details = f"""
[bold]ID:[/bold] {self.task.id}
[bold]Status:[/bold] {self.task.status.value if hasattr(self.task.status, "value") else str(self.task.status)}
[bold]Priority:[/bold] {self.task.priority.value if hasattr(self.task.priority, "value") else str(self.task.priority)}
[bold]Project:[/bold] {getattr(self.task, "parent_project", "N/A")}
[bold]Assignee:[/bold] {getattr(self.task, "assignee", "Unassigned")}
[bold]Time Estimate:[/bold] {getattr(self.task, "time", "N/A")}
[bold]Progress:[/bold] {getattr(self.task, "progress", "N/A")}%
[bold]Tags:[/bold] {", ".join(getattr(self.task, "tags", []))}
[bold]Created:[/bold] {getattr(self.task, "created_at", datetime.now()).strftime("%Y-%m-%d %H:%M")}
[bold]Updated:[/bold] {getattr(self.task, "updated_at", datetime.now()).strftime("%Y-%m-%d %H:%M")}
"""
        context = getattr(self.task, "context_notes", None)
        if context:
            details += f"\n[bold]Context:[/bold]\n{context}\n"

        return details

    @on(Button.Pressed, "#close-btn")
    def on_close(self) -> None:
        self.dismiss()


class EditTaskModal(ModalScreen):
    """Modal screen for editing tasks."""

    def __init__(self, task: Task):
        super().__init__()
        self.original_task = task
        self.current_task = task
        self.field_index = 0
        self.fields = [
            ("action", "Action"),
            ("time", "Time Estimate"),
            ("priority", "Priority"),
            ("status", "Status"),
            ("parent_project", "Project"),
            ("assignee", "Assignee"),
            ("tags", "Tags"),
            ("context_notes", "Context"),
            ("progress", "Progress"),
        ]

    def compose(self) -> ComposeResult:
        with Container(id="edit-modal"):
            yield Label(f"✏️ Edit Task", id="edit-title")
            yield Input(placeholder="Field value", id="field-input")
            yield Label("", id="field-label")
            with Horizontal():
                yield Button("Previous", id="prev-btn", variant="secondary")
                yield Button("Next", id="next-btn", variant="secondary")
                yield Button("Save", id="save-btn", variant="primary")
                yield Button("Cancel", id="cancel-btn", variant="secondary")

    def on_mount(self) -> None:
        self.update_field_display()

    def update_field_display(self) -> None:
        """Update the current field display."""
        field_name, field_label = self.fields[self.field_index]
        input_widget = self.query_one("#field-input", Input)
        label_widget = self.query_one("#field-label", Label)

        # Get current value
        value = getattr(self.current_task, field_name, "")
        if field_name == "priority" and hasattr(value, "value"):
            value = value.value
        elif field_name == "status" and hasattr(value, "value"):
            value = value.value
        elif field_name == "assignee" and hasattr(value, "value"):
            value = value.value
        elif field_name == "tags" and isinstance(value, list):
            value = ", ".join(value)

        input_widget.value = str(value)
        label_widget.update(f"[bold]{field_label}:[/bold]")

    @on(Button.Pressed, "#prev-btn")
    def on_previous(self) -> None:
        self.field_index = (self.field_index - 1) % len(self.fields)
        self.update_field_display()

    @on(Button.Pressed, "#next-btn")
    def on_next(self) -> None:
        self.field_index = (self.field_index + 1) % len(self.fields)
        self.update_field_display()

    @on(Button.Pressed, "#save-btn")
    def on_save(self) -> None:
        # Update the current field
        field_name, _ = self.fields[self.field_index]
        input_widget = self.query_one("#field-input", Input)
        value = input_widget.value

        # Convert value based on field type
        if field_name == "priority":
            priority_map = {
                "low": Priority.LOW,
                "medium": Priority.MEDIUM,
                "high": Priority.HIGH,
                "critical": Priority.CRITICAL,
                "urgent": Priority.URGENT,
            }
            setattr(
                self.current_task,
                field_name,
                priority_map.get(value.lower(), Priority.MEDIUM),
            )
        elif field_name == "status":
            status_map = {
                "todo": Status.TODO,
                "pending": Status.PENDING,
                "in_progress": Status.IN_PROGRESS,
                "blocked": Status.BLOCKED,
                "review": Status.REVIEW,
                "done": Status.DONE,
                "completed": Status.COMPLETED,
                "cancelled": Status.CANCELLED,
                "deferred": Status.DEFERRED,
            }
            setattr(
                self.current_task,
                field_name,
                status_map.get(value.lower(), Status.TODO),
            )
        elif field_name == "assignee":
            assignee_map = {
                "human": Assignee.HUMAN,
                "ai": Assignee.AI,
                "collaborative": Assignee.COLLABORATIVE,
            }
            setattr(
                self.current_task, field_name, assignee_map.get(value.lower(), None)
            )
        elif field_name == "tags":
            setattr(
                self.current_task,
                field_name,
                [tag.strip() for tag in value.split(",") if tag.strip()],
            )
        elif field_name == "progress":
            try:
                setattr(self.current_task, field_name, min(max(int(value), 0), 100))
            except ValueError:
                pass
        else:
            setattr(self.current_task, field_name, value)

        self.dismiss(self.current_task)

    @on(Button.Pressed, "#cancel-btn")
    def on_cancel(self) -> None:
        self.dismiss(None)


class TasksDirWatcher(FileSystemEventHandler):
    """File system event handler for task directory changes."""

    def __init__(self, callback: Callable[[], None]) -> None:
        super().__init__()
        self.callback = callback

    def on_any_event(self, event) -> None:
        """Handle file system events."""
        if isinstance(event, (FileModifiedEvent, FileCreatedEvent, FileDeletedEvent)):
            self.callback()


class EnhancedTaskListWidget(ListView):
    """Enhanced task list widget with rich display and actions"""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.tasks: List[Task] = []
        self.display_config: DisplayConfig = DisplayConfig()

    def update_tasks(
        self,
        tasks: List[Task],
        selected_index: int = 0,
        display_config: Optional[DisplayConfig] = None,
    ) -> None:
        """Update the task list with enhanced display"""
        self.tasks = tasks
        if display_config:
            self.display_config = display_config

        self.clear()

        if not tasks:
            self.append(ListItem(Label("+ Add New Task", classes="empty-state")))
            return

        for i, task in enumerate(tasks):
            display = self._format_task_display(task, i == selected_index)
            self.append(ListItem(Label(display)))

    def _format_task_display(self, task: Task, is_selected: bool = False) -> str:
        """Format a task for display with enhanced information."""
        # Handle status from storage.py (Status.Todo, Status.InProgress, etc.)
        status = task.status
        status_str = (
            str(status.name).lower()
            if hasattr(status, "name")
            else str(status).lower().replace("status.", "")
        )

        # Map storage.py status names to emojis
        status_emoji_map = {
            "todo": "📝",
            "pending": "⏳",
            "inprogress": "🔄",
            "in_progress": "🔄",
            "blocked": "🚫",
            "review": "👀",
            "done": "✅",
            "completed": "✅",
            "cancelled": "❌",
            "canceled": "❌",
            "deferred": "⏸️",
            "archived": "📦",
        }
        status_emoji = status_emoji_map.get(status_str, "❓")

        # Handle priority from storage.py (Priority.Low, Priority.Medium, etc.)
        priority = task.priority
        priority_str = (
            str(priority.name).lower()
            if hasattr(priority, "name")
            else str(priority).lower().replace("priority.", "")
        )

        priority_emoji_map = {
            "low": "🟢",
            "medium": "🟡",
            "high": "🟠",
            "critical": "🔴",
            "urgent": "🚨",
        }
        priority_emoji = priority_emoji_map.get(priority_str, "⚪")

        # Handle assignee
        assignee_emoji = "❓"
        if task.assignee:
            assignee_str = (
                str(task.assignee.name).lower()
                if hasattr(task.assignee, "name")
                else str(task.assignee).lower().replace("assignee.", "")
            )
            assignee_emoji_map = {
                "human": "👤",
                "ai": "🤖",
                "collaborative": "🤝",
            }
            assignee_emoji = assignee_emoji_map.get(assignee_str, "❓")

        # Base display
        action = task.action[:55] + "..." if len(task.action) > 55 else task.action
        project = f"[{task.parent_project}]" if task.parent_project else ""
        progress = f" {task.progress}%" if task.progress is not None else ""

        display = f"{status_emoji} {priority_emoji} {assignee_emoji} {action} {project}{progress}"

        # Add additional info based on display config
        if self.display_config.show_created_at and hasattr(task, "created_at"):
            time_ago = format_duration(task.created_at, utcnow())
            display += f" ({time_ago})"

        if self.display_config.show_tags and hasattr(task, "tags") and task.tags:
            tags_display = f" #{', #'.join(task.tags[:3])}"
            if len(task.tags) > 3:
                tags_display += "..."
            display += tags_display

        return display


class AnalyticsWidget(Static):
    """Widget for displaying analytics and statistics."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.completion_data: List[int] = []
        self.priority_distribution: List[int] = []

    def update_analytics(
        self,
        tasks: List[Task],
        completion_data: List[int],
        priority_distribution: List[int],
    ) -> None:
        """Update analytics display."""
        self.completion_data = completion_data
        self.priority_distribution = priority_distribution

        # Create analytics display
        total_tasks = len(tasks)
        completed = len(
            [t for t in tasks if t.status in [Status.DONE, Status.COMPLETED]]
        )
        completion_rate = (completed / total_tasks * 100) if total_tasks > 0 else 0

        content = f"""
[bold #4361ee]📊 Analytics Dashboard[/bold #4361ee]

[bold]Task Overview:[/bold]
• Total Tasks: {total_tasks}
• Completed: {completed}
• Completion Rate: {completion_rate:.1f}%

[bold]Priority Distribution:[/bold]
• Low: {priority_distribution[0] if len(priority_distribution) > 0 else 0}
• Medium: {priority_distribution[1] if len(priority_distribution) > 1 else 0}
• High: {priority_distribution[2] if len(priority_distribution) > 2 else 0}
• Critical: {priority_distribution[3] if len(priority_distribution) > 3 else 0}
• Urgent: {priority_distribution[4] if len(priority_distribution) > 4 else 0}

[bold]Recent Activity:[/bold]
{self._format_recent_activity(tasks)}
"""
        self.update(content)

    def _format_recent_activity(self, tasks: List[Task]) -> str:
        """Format recent activity summary."""
        recent_tasks = sorted(
            tasks, key=lambda t: getattr(t, "updated_at", datetime.min), reverse=True
        )[:5]

        if not recent_tasks:
            return "No recent activity"

        activity_lines = []
        for task in recent_tasks:
            time_ago = format_duration(getattr(task, "updated_at", utcnow()), utcnow())
            status_icon = (
                "✅" if task.status in [Status.DONE, Status.COMPLETED] else "📝"
            )
            activity_lines.append(f"{status_icon} {task.action[:30]}... ({time_ago})")

        return "\n".join(activity_lines)


class ActivityFeedWidget(RichLogWidget):
    """Widget for displaying live activity feed."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.entries: Deque[ActivityEntry] = deque(maxlen=200)

    def add_entry(self, level: str, message: str) -> None:
        """Add an activity entry."""
        entry = ActivityEntry(timestamp=utcnow(), level=level, message=message)
        self.entries.append(entry)
        self.write(f"[{entry.timestamp.strftime('%H:%M:%S')}] {message}", level=level)

    def get_recent_entries(self, limit: int = 10) -> List[ActivityEntry]:
        """Get recent activity entries."""
        return list(self.entries)[-limit:]


class ToastWidget(Container):
    """Widget for displaying toast notifications - enhanced to show multiple toasts like Rust TUI"""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.toasts: List[ToastNotification] = []

    def update_toasts(self, toasts: List[ToastNotification]) -> None:
        """Update toast display - show up to 3 toasts stacked"""
        self.toasts = [t for t in toasts if not t.is_expired()][-3:]  # Show last 3

        # Remove all existing toast children
        self.remove_children()

        if not self.toasts:
            return

        # Create toast widgets for each toast (bottom to top)
        for i, toast in enumerate(reversed(self.toasts)):
            icon_map = {
                ToastType.Success: "✅",
                ToastType.Error: "❌",
                ToastType.Warning: "⚠️",
                ToastType.Info: "ℹ️",
            }
            icon = icon_map.get(toast.notification_type, "ℹ️")

            style_map = {
                ToastType.Success: "bold green on dark_green",
                ToastType.Error: "bold white on red",
                ToastType.Warning: "bold black on yellow",
                ToastType.Info: "bold white on blue",
            }
            style = style_map.get(toast.notification_type, "bold white on blue")

            toast_text = f"[{style}]{icon} {toast.message}[/]"
            toast_widget = Static(toast_text, classes="toast-item")
            self.mount(toast_widget)


class StatusBar(Static):
    """A reusable status bar widget that formats messages consistently."""

    def update_status(self, **kwargs) -> None:
        """Update the status bar with formatted message."""
        parts = []
        for key, value in kwargs.items():
            if value is not None:
                parts.append(f"{key.title()}: {value}")
        self.update(" | ".join(parts) if parts else "")


class TaskDetailWidget(Static):
    """Widget showing detailed task information with improved formatting."""

    def update_task(self, task: Optional[Task]) -> None:
        """Update the displayed task with safe attribute access."""
        if not task:
            self.update("[dim]Select a task to view details[/dim]")
            return

        def safe_getattr(obj, attr, default="N/A"):
            """Safely get attribute with fallback."""
            return str(getattr(obj, attr, default))

        def safe_getattr_value(obj, attr, default="N/A"):
            """Safely get attribute value (handles enums)."""
            attr_val = getattr(obj, attr, default)
            return getattr(attr_val, "value", str(attr_val))

        status_str = safe_getattr_value(task, "status")
        priority_str = safe_getattr_value(task, "priority")
        assignee_str = safe_getattr(task, "assignee", "Unassigned")
        tags_str = ", ".join(getattr(task, "tags", []) or [])
        progress_str = (
            f"{task.progress}%"
            if getattr(task, "progress", None) is not None
            else "N/A"
        )
        ctx = getattr(task, "context_notes", None) or ""
        created = getattr(task, "created_at", datetime.now(UTC))
        updated = getattr(task, "updated_at", datetime.now(UTC))

        details = f"""
[bold #4361ee]Task Details[/bold #4361ee]

[bold]Action:[/bold] {task.action}
[bold]Status:[/bold] {status_str}
[bold]Priority:[/bold] {priority_str}
[bold]Project:[/bold] {safe_getattr(task, "parent_project")}
[bold]Assignee:[/bold] {assignee_str}
[bold]Time Estimate:[/bold] {safe_getattr(task, "time")}
[bold]Progress:[/bold] {progress_str}
[bold]Tags:[/bold] {tags_str if tags_str else "None"}
[bold]Created:[/bold] {created.strftime("%Y-%m-%d %H:%M")}
[bold]Updated:[/bold] {updated.strftime("%Y-%m-%d %H:%M")}
"""
        if ctx:
            details += f"\n[bold]Context:[/bold]\n{ctx}\n"

        self.update(details)


class TodoziTUI(App):
    """Enhanced Todozi TUI Application with comprehensive features"""

    CSS = """
    /* Beautiful Todozi Light Purple Theme - Matching Rust TUI */
    Screen {
        background: #e8e0f5;  /* Light lavender background */
    }

    Header {
        background: #6b4c93;  /* Dark purple header */
        color: #ffffff;
        text-style: bold;
        border-bottom: heavy #4a2c6b;
        height: 3;
    }

    Footer {
        background: #6b4c93;  /* Dark purple footer */
        color: #ffffff;
        border-top: heavy #4a2c6b;
        height: 3;
    }

    #main-container {
        layout: horizontal;
        height: 1fr;
        background: #e8e0f5;
    }

    #left-panel {
        width: 60%;
        border: dashed #6b4c93;
        background: #e8e0f5;
        padding: 1;
    }

    #right-panel {
        width: 40%;
        border: dashed #6b4c93;
        background: #e8e0f5;
        padding: 1;
    }

    .status-bar {
        height: 3;
        background: #e8e0f5;
        border: dashed #6b4c93;
        color: #4a2c6b;
        text-style: bold;
    }

    .filter-bar {
        height: 3;
        background: #e8e0f5;
        border: dashed #6b4c93;
        color: #4a2c6b;
        text-style: bold;
    }

    TabbedContent {
        background: #e8e0f5;
    }

    TabbedContent > Tab {
        background: #6b4c93;
        color: #ffffff;
        text-style: bold;
        padding: 1;
    }

    TabbedContent > Tab.--selected {
        background: #8b6fb3;
        color: #ffffff;
        text-style: bold;
    }

    TabbedContent > Tab:hover {
        background: #7a5fa3;
    }

    ListView {
        background: #e8e0f5;
        color: #4a2c6b;
        border: dashed #6b4c93;
    }

    ListView > ListItem {
        background: #e8e0f5;
        color: #4a2c6b;
    }

    ListView > ListItem.--highlight {
        background: #b7c9e2;
        color: #2d1b4e;
        text-style: bold;
    }

    ListView > ListItem:hover {
        background: #d0d5e8;
    }

    Static {
        background: #e8e0f5;
        color: #4a2c6b;
    }

    Label {
        color: #4a2c6b;
        text-style: bold;
    }

    Input {
        background: #ffffff;
        color: #2d1b4e;
        border: solid #6b4c93;
        padding: 1;
    }

    Input:focus {
        border: solid #8b6fb3;
        background: #f5f0ff;
    }

    Button {
        background: #6b4c93;
        color: #ffffff;
        text-style: bold;
        border: solid #4a2c6b;
    }

    Button:hover {
        background: #8b6fb3;
    }

    Button.--active {
        background: #4a2c6b;
    }

    /* Section headers */
    .section-header {
        background: #6b4c93;
        color: #ffffff;
        text-style: bold;
        padding: 1;
        border: solid #4a2c6b;
    }

    .section-content {
        background: #e8e0f5;
        color: #4a2c6b;
        border: dashed #6b4c93;
        padding: 1;
    }

    /* Empty state styling */
    .empty-state {
        color: #6b4c93;
        text-style: italic;
        padding: 2;
        text-align: center;
    }

    /* Success/Warning/Danger colors */
    .success {
        background: #06d6a0;
        color: #0d0d1a;
    }

    .warning {
        background: #ffd166;
        color: #0d0d1a;
    }

    .error {
        background: #ef476f;
        color: #ffffff;
    }

    .info {
        background: #4361ee;
        color: #ffffff;
    }

    /* Enhanced styling for components */
    .toast-widget {
        height: 2;
        background: #e8e0f5;
        border: dashed #6b4c93;
        color: #4a2c6b;
    }

    .analytics-widget {
        background: #e8e0f5;
        color: #4a2c6b;
        border: dashed #6b4c93;
        padding: 1;
    }

    .activity-feed {
        background: #e8e0f5;
        color: #4a2c6b;
        border: dashed #6b4c93;
        padding: 1;
    }

    /* Modal styling */
    ModalScreen {
        background: rgba(107, 76, 147, 0.8);
    }

    .modal-content {
        background: #e8e0f5;
        border: solid #6b4c93;
        color: #4a2c6b;
        padding: 2;
    }

    /* Panel styling */
    .panel {
        background: #e8e0f5;
        border: dashed #6b4c93;
        padding: 1;
    }

    /* Action buttons */
    .action-button {
        background: #6b4c93;
        color: #ffffff;
        border: solid #4a2c6b;
    }

    .action-button:hover {
        background: #8b6fb3;
    }

    /* Scrollable containers */
    ScrollableContainer {
        background: #e8e0f5;
        border: dashed #6b4c93;
    }

    /* Data tables */
    DataTable {
        background: #e8e0f5;
        color: #4a2c6b;
        border: dashed #6b4c93;
    }

    DataTable > .datatable--cursor {
        background: #b7c9e2;
        color: #2d1b4e;
    }

    DataTable > .datatable--header {
        background: #6b4c93;
        color: #ffffff;
        text-style: bold;
    }
    """

    BINDINGS = [
        Binding("q", "quit", "Quit", priority=True),
        Binding("ctrl+q", "quit", "Quit", priority=True),
        Binding("escape", "escape", "Escape/Close", priority=True),
        Binding("tab", "next_tab", "Next Tab"),
        Binding("shift+tab", "prev_tab", "Prev Tab"),
        Binding("1", "tab_1", "Projects"),
        Binding("2", "tab_2", "Tasks"),
        Binding("3", "tab_3", "Feed"),
        Binding("4", "tab_4", "Done"),
        Binding("5", "tab_5", "Find"),
        Binding("6", "tab_6", "More"),
        Binding("7", "tab_7", "API"),
        Binding("8", "tab_8", "Bye"),
        Binding("a", "add_task", "Add Task"),
        Binding("shift+a", "add_project", "Add Project"),
        Binding("d", "delete_task", "Delete Task"),
        Binding("c", "complete_task", "Complete Task"),
        Binding("e", "edit_task", "Edit Task"),
        Binding("r", "refresh", "Refresh"),
        Binding("f", "filter", "Filter"),
        Binding("p", "projects", "Projects"),
        Binding("s", "stats", "Stats"),
        Binding("/", "focus_search", "Focus Search"),
        Binding("?", "help", "Help"),
        # Enhanced bindings
        Binding("ctrl+a", "add_task_quick", "Quick Add"),
        Binding("ctrl+e", "edit_task_modal", "Edit Modal"),
        Binding("ctrl+d", "delete_task_confirm", "Delete with Confirm"),
        Binding("space", "task_actions", "Task Actions"),
        Binding("enter", "task_enter", "Enter/Select"),
        Binding("ctrl+r", "refresh_all", "Full Refresh"),
        Binding("ctrl+s", "save_all", "Save All"),
        Binding("ctrl+f", "toggle_filters", "Toggle Filters"),
        Binding("ctrl+l", "clear_filters", "Clear Filters"),
        Binding("ctrl+t", "toggle_display", "Toggle Display"),
        # Filter shortcuts (F1-F4) matching Rust TUI
        Binding("f1", "filter_status", "Filter Status", priority=True),
        Binding("f2", "filter_priority", "Filter Priority", priority=True),
        Binding("f3", "filter_project", "Filter Project", priority=True),
        Binding("f4", "clear_filters", "Clear Filters", priority=True),
        Binding("f5", "refresh_all", "Refresh All", priority=True),
        # Done tab sorting (matching Rust TUI)
        Binding("s", "sort_done", "Sort Done", priority=True),
        Binding("o", "sort_order", "Sort Order", priority=True),
        Binding("shift+r", "reset_done_filters", "Reset Done Filters", priority=True),
        # Navigation enhancements
        Binding("up", "navigate_up", "Navigate Up", priority=True),
        Binding("down", "navigate_down", "Navigate Down", priority=True),
        Binding("left", "navigate_left", "Navigate Left", priority=True),
        Binding("right", "navigate_right", "Navigate Right", priority=True),
        # Task actions (matching Rust TUI)
        Binding("x", "stop_server", "Stop Server", priority=True),
        Binding("shift+s", "start_server", "Start Server", priority=True),
        Binding("shift+r", "restart_server", "Restart Server", priority=True),
        Binding("shift+c", "clear_cache", "Clear Cache", priority=True),
        # Done tab specific
        Binding("shift+p", "filter_done_project", "Filter Done Project", priority=True),
        Binding(
            "shift+i", "filter_done_priority", "Filter Done Priority", priority=True
        ),
    ]

    TITLE = "Todozi [✓]"
    SUB_TITLE = "AI/Human Task Management System"

    # Reactive state
    selected_task_index: reactive[int] = reactive(0)
    selected_project_index: reactive[int] = reactive(0)
    current_project: reactive[Optional[str]] = reactive(None)
    filter_status: reactive[Optional[Status]] = reactive(None)
    current_tab: reactive[AppTab] = reactive(AppTab.Projects)

    def __init__(self):
        super().__init__()
        self.storage: Optional[Storage] = None
        self.tasks: List[Task] = []
        self.filtered_tasks: List[Task] = []
        self.projects: List[Project] = []
        self.task_filters = TaskFilters()
        self.done_filters = TaskFilters()
        self.done_sort_by = TaskSortBy.DateCompleted
        self.done_sort_order = SortOrder.Descending
        self.done_selected_task_index = 0
        self.search_query = ""
        self.search_results: List[Task] = []
        self.editor: Optional[EditSession] = None
        self.editor_field = EditorField.Action
        self.editor_input = ""
        self.editor_selected_field = 0
        self.more_tab_section = MoreTabSection.Ideas
        self.more_scroll_offset = 0
        self.api_keys: List[ApiKey] = []
        self.api_selected_index = 0
        self.toast_notifications: List[ToastNotification] = []
        self.toast_notifier = ToastNotifier(self)
        self.ideas: List[Idea] = []
        self.memories: List[Memory] = []
        self.errors: List[Error] = []
        self.feelings: List[Feeling] = []
        self.training_data: List[TrainingData] = []
        self.queue_items: List[QueueItem] = []
        self.reminders: List[Reminder] = []
        self.server_running = False
        self.server_status = "🔴 Not running"
        self.observer: Optional[Observer] = None
        self._watch_callback_scheduled = False
        self._activity_feed: Deque[ActivityEntry] = deque(maxlen=200)

        # Enhanced state
        self.display_config = DisplayConfig()
        self.completion_data: List[int] = [0] * 50  # Last 50 days
        self.priority_distribution: List[int] = [
            0
        ] * 5  # Low, Medium, High, Critical, Urgent
        self.selected_project_index = 0
        self.task_action_menu: Optional[int] = None
        self.task_action_selected = 0
        self.show_task_details: Optional[Task] = None

        # File watching
        self.file_watcher_thread: Optional[threading.Thread] = None
        self.stop_watching = False

    async def on_mount(self) -> None:
        """Initialize the app"""
        await self.load_storage()
        await self.refresh_data()

        # Use call_next to ensure widgets are mounted before updating
        self.call_next(self.update_all_ui)
        # Also explicitly update the initial Projects tab after a brief delay
        self.set_timer(0.15, self._update_projects_tab_widgets)

        self.start_file_watcher()
        self.set_interval(1.0, self.update_toasts)  # More frequent toast updates
        self.set_interval(5.0, self.auto_refresh)
        self.log_activity("info", "Enhanced Todozi TUI started")

        # Update analytics (with error handling)
        try:
            self.update_analytics_data()
        except Exception as e:
            logger.debug(f"Failed to update analytics: {e}")
            # Don't fail the whole app if analytics fails

    def update_all_ui(self) -> None:
        """Update all UI components"""
        self.update_task_list()
        self.update_projects_list()
        self.update_done_list()
        self.update_search_results()
        self.update_api_keys_list()
        self.update_feed_stats()
        self.update_more_content()

        # Update activity feed
        try:
            feed_widget = self.query_one("#feed-content", ActivityFeedWidget)
            for entry in list(self._activity_feed)[-20:]:  # Last 20 entries
                feed_widget.add_entry(entry.level, entry.message)
        except Exception:
            pass

    async def on_unmount(self) -> None:
        """Clean up resources when app is closed"""
        self.stop_watching = True

        if self.file_watcher_thread and self.file_watcher_thread.is_alive():
            try:
                self.file_watcher_thread.join(timeout=2.0)
                logger.info("File watcher thread stopped cleanly")
            except Exception as exc:
                logger.exception("Error while stopping watcher thread: %s", exc)

        if (
            self.observer
            and hasattr(self.observer, "is_alive")
            and self.observer.is_alive()
        ):
            try:
                self.observer.stop()
                self.observer.join(timeout=2.0)
                logger.info("File watcher stopped cleanly")
            except Exception as exc:
                logger.exception("Error while stopping watcher: %s", exc)

    def update_analytics_data(self) -> None:
        """Update analytics data based on current tasks."""
        if not self.tasks:
            return

        try:
            # Update completion data (last 50 days)
            now = utcnow()
            for i in range(50):
                date = now - timedelta(days=49 - i)
                completed_count = len(
                    [
                        task
                        for task in self.tasks
                        if task.status in [Status.DONE, Status.COMPLETED]
                        and hasattr(task, "updated_at")
                        and task.updated_at
                        and task.updated_at.date() == date.date()
                    ]
                )
                self.completion_data[i] = completed_count

            # Update priority distribution
            priority_counts = {
                Priority.LOW: 0,
                Priority.MEDIUM: 0,
                Priority.HIGH: 0,
                Priority.CRITICAL: 0,
                Priority.URGENT: 0,
            }
            for task in self.tasks:
                if task.status not in [Status.DONE, Status.COMPLETED]:
                    # Handle both enum and string priorities
                    priority = task.priority
                    if isinstance(priority, str):
                        # Convert string to enum
                        priority_str = priority.lower()
                        if priority_str == "low":
                            priority = Priority.LOW
                        elif priority_str == "medium":
                            priority = Priority.MEDIUM
                        elif priority_str == "high":
                            priority = Priority.HIGH
                        elif priority_str == "critical":
                            priority = Priority.CRITICAL
                        elif priority_str == "urgent":
                            priority = Priority.URGENT
                        else:
                            priority = Priority.MEDIUM  # default fallback
                    elif hasattr(priority, "value"):
                        # If it's an enum with a value attribute, use it as-is
                        pass
                    else:
                        # Unknown type, default to MEDIUM
                        priority = Priority.MEDIUM

                    # Ensure priority is in our counts dict
                    if priority in priority_counts:
                        priority_counts[priority] += 1
                    else:
                        priority_counts[Priority.MEDIUM] += 1  # fallback

            self.priority_distribution = [
                priority_counts[Priority.LOW],
                priority_counts[Priority.MEDIUM],
                priority_counts[Priority.HIGH],
                priority_counts[Priority.CRITICAL],
                priority_counts[Priority.URGENT],
            ]
        except Exception as e:
            logger.debug(f"Error updating analytics data: {e}")
            # Reset to defaults on error
            self.completion_data = [0] * 50
            self.priority_distribution = [0] * 5

    def log_activity(self, level: str, message: str) -> None:
        """Log an activity to the feed and standard logging."""
        entry = ActivityEntry(timestamp=datetime.now(UTC), level=level, message=message)
        self._activity_feed.append(entry)
        getattr(logger, level.lower(), logger.info)(message)

    def _handle_error(self, context: str, exception: Exception) -> None:
        """Centralized error handling for consistent logging and notifications."""
        error_msg = f"{context}: {str(exception)}"
        self.log_activity("error", error_msg)
        self.toast_notifier.error(f"Error {context.lower()}: {exception}")

    def _ensure_ready(self) -> bool:
        """Return True if storage and tasks are available."""
        ready = bool(self.storage) and bool(self.filtered_tasks)
        if not ready:
            self.toast_notifier.warning("No tasks or storage unavailable")
        return ready

    def _current_task(self) -> Optional[Task]:
        """Return the currently selected task or None."""
        idx = self.selected_task_index
        if 0 <= idx < len(self.filtered_tasks):
            return self.filtered_tasks[idx]
        self.toast_notifier.warning("No task selected")
        return None

    async def load_storage(self) -> None:
        """Load storage asynchronously with better error handling"""
        try:
            self.storage = await Storage.new()
            self.log_activity("info", "Storage loaded successfully")
        except Exception as e:
            self._handle_error("Loading storage", e)
            self.storage = None

    async def refresh_data(self) -> None:
        """Refresh all data from storage with better error handling"""
        if not self.storage:
            self.log_activity("warning", "Storage not available")
            return

        try:
            # Load tasks
            try:
                filters = TaskFilters()
                if self.filter_status:
                    filters.status_filter = self.filter_status
                if self.current_project:
                    filters.project_filter = self.current_project

                # Try to load tasks
                if hasattr(self.storage, "list_tasks_across_projects"):
                    self.tasks = self.storage.list_tasks_across_projects(
                        self.task_filters
                    )
                else:
                    # Fallback to basic list if method doesn't exist
                    self.tasks = self.storage.list_tasks_across_projects(TaskFilters())

                # Ensure we have a list
                if not isinstance(self.tasks, list):
                    self.tasks = []

                self.filtered_tasks = self.tasks.copy() if self.tasks else []
                self.log_activity("info", f"Loaded {len(self.tasks)} tasks")
            except Exception as e:
                logger.error(f"Error loading tasks: {e}", exc_info=True)
                self.tasks = []
                self.filtered_tasks = []
                self.log_activity("error", f"Failed to load tasks: {e}")

            # Load projects
            try:
                self.projects = self.storage.list_projects()
                if not isinstance(self.projects, list):
                    self.projects = []
                self.log_activity("info", f"Loaded {len(self.projects)} projects")
            except Exception as e:
                logger.error(f"Error loading projects: {e}", exc_info=True)
                self.projects = []
                self.log_activity("error", f"Failed to load projects: {e}")

            # Load extended data
            try:
                idea_manager = IdeaManager()
                await idea_manager.load_ideas()
                self.ideas = idea_manager.get_all_ideas()
            except Exception as e:
                logger.debug("Failed to load ideas: %s", e)
                self.ideas = []

            try:
                memory_manager = MemoryManager()
                await memory_manager.load_memories()
                self.memories = memory_manager.get_all_memories()
            except Exception as e:
                logger.debug("Failed to load memories: %s", e)
                self.memories = []

            try:
                error_manager = ErrorManager()
                self.errors = error_manager.list_errors()
            except Exception as e:
                logger.debug("Failed to load errors: %s", e)
                self.errors = []

            # Load additional extended data
            try:
                # Load feelings (mock data for now)
                self.feelings = [
                    Feeling(
                        emotion="Happy",
                        intensity=8,
                        context="Completed a challenging task",
                    ),
                    Feeling(
                        emotion="Focused",
                        intensity=7,
                        context="Working on important project",
                    ),
                ]
            except Exception as e:
                logger.debug("Failed to load feelings: %s", e)
                self.feelings = []

            try:
                # Load training data (mock data for now)
                self.training_data = [
                    TrainingData(
                        data_type="task",
                        prompt="How to prioritize tasks",
                        response="Use Eisenhower matrix",
                    ),
                    TrainingData(
                        data_type="project",
                        prompt="How to manage team projects",
                        response="Use agile methodology",
                    ),
                ]
            except Exception as e:
                logger.debug("Failed to load training data: %s", e)
                self.training_data = []

            try:
                # Load queue items (mock data for now)
                self.queue_items = [
                    QueueItem(task_name="Process user feedback", priority=3),
                    QueueItem(task_name="Update documentation", priority=1),
                ]
            except Exception as e:
                logger.debug("Failed to load queue items: %s", e)
                self.queue_items = []

            try:
                # Load reminders (mock data for now)
                self.reminders = [
                    Reminder(
                        title="Team meeting",
                        message="Weekly standup at 10 AM",
                        due_date=utcnow() + timedelta(hours=2),
                    ),
                    Reminder(
                        title="Project deadline",
                        message="Submit final report",
                        due_date=utcnow() + timedelta(days=3),
                    ),
                ]
            except Exception as e:
                logger.debug("Failed to load reminders: %s", e)
                self.reminders = []

            # Load API keys
            try:
                self.api_keys = list_api_keys()
            except Exception as e:
                logger.debug("Failed to load API keys: %s", e)
                self.api_keys = []

            # Update all UI components after data refresh
            self.update_all_ui()

            # Update API tab components
            try:
                self.update_api_keys_list()
                self.update_api_endpoints_list()
                self.update_server_status()
            except Exception as e:
                logger.debug(f"Error updating API components: {e}")

        except Exception as e:
            # Log the full error with traceback
            logger.error(f"Error in refresh_data: {e}", exc_info=True)
            self._handle_error("Refreshing data", e)
            # Still try to update UI with whatever data we have
            try:
                self.update_all_ui()
            except Exception as ui_error:
                logger.error(f"Error updating UI: {ui_error}", exc_info=True)

    def apply_filters(self) -> None:
        """Apply current filters to tasks"""
        if not self.tasks:
            self.filtered_tasks = []
            return

        filtered = self.tasks

        # Apply status filter
        if self.filter_status:
            filtered = [t for t in filtered if t.status == self.filter_status]

        # Apply project filter
        if self.current_project:
            filtered = [
                t
                for t in filtered
                if getattr(t, "parent_project", None) == self.current_project
            ]

        # Apply task filters
        if self.task_filters.status_filter:
            filtered = [
                t for t in filtered if t.status == self.task_filters.status_filter
            ]
        if self.task_filters.priority_filter:
            filtered = [
                t for t in filtered if t.priority == self.task_filters.priority_filter
            ]
        if self.task_filters.project_filter:
            filtered = [
                t
                for t in filtered
                if getattr(t, "parent_project", None)
                == self.task_filters.project_filter
            ]
        if self.task_filters.assignee_filter:
            filtered = [
                t for t in filtered if t.assignee == self.task_filters.assignee_filter
            ]

        self.filtered_tasks = filtered
        self.selected_task_index = (
            min(self.selected_task_index, len(self.filtered_tasks) - 1)
            if self.filtered_tasks
            else 0
        )

    def compose(self) -> ComposeResult:
        """Create the enhanced UI layout"""
        yield Header(show_clock=True)

        # Main content area
        with Container(id="main-content"):
            with TabbedContent(initial="projects"):
                with TabPane("📁 Projects", id="projects"):
                    yield from self._compose_projects_tab()

                with TabPane("📋 Tasks", id="tasks"):
                    yield from self._compose_tasks_tab()

                with TabPane("✅ Done", id="done"):
                    yield from self._compose_done_tab()

                with TabPane("🔍 Find", id="find"):
                    yield from self._compose_find_tab()

                with TabPane("🔮 More", id="more"):
                    yield from self._compose_more_tab()

                with TabPane("🔑 API", id="api"):
                    yield from self._compose_api_tab()

                with TabPane("📰 Feed", id="feed"):
                    yield from self._compose_feed_tab()

                with TabPane("👋 Bye", id="bye"):
                    yield from self._compose_bye_tab()

        yield Footer()

        # Toast notification widget (overlay on top)
        yield ToastWidget(classes="toast-widget", id="toast-widget")

        # Editor overlay (full screen when editor is active)
        if hasattr(self, "editor") and self.editor:
            with Container(id="editor-overlay", classes="editor-overlay"):
                # Editor will be rendered here when active
                pass

    def _compose_projects_tab(self) -> ComposeResult:
        with Vertical():
            yield Static(
                "Actions", classes="section-header", id="projects-actions-header"
            )
            with Container(classes="section-content"):
                yield Static(
                    "Projects: ↑↓ Navigate\nEnter Select\nShift+A Add New\nD Delete\nF5 Refresh",
                    id="projects-actions",
                )
                yield Button("Add Project", id="add-project", classes="action-button")
            yield Static(
                "Active Projects", classes="section-header", id="projects-list-header"
            )
            with Container(classes="section-content"):
                yield ListView(id="projects-list")
            yield Static("", classes="status-bar", id="projects-status")

    def _compose_tasks_tab(self) -> ComposeResult:
        with Vertical():
            yield Static("Filters", classes="section-header", id="filters-header")
            with Container(classes="section-content"):
                yield Static(
                    "| Filters: None | Status: All | Priority: All | Project: All | F1-F4: Set filter",
                    classes="filter-bar",
                    id="filter-status",
                )
            yield Static("Tasks", classes="section-header", id="task-list-header")
            with Container(classes="section-content"):
                yield EnhancedTaskListWidget(id="task-list")
                # Task action menu container (will be shown/hidden dynamically)
                with Container(id="task-action-menu-container", classes="hidden"):
                    yield Static("", id="task-action-menu")
            yield Static("", classes="status-bar", id="task-status")

    def _compose_done_tab(self) -> ComposeResult:
        with Vertical():
            # Sort & Filter controls (matching Rust TUI)
            yield Static(
                "Sort & Filter", classes="section-header", id="done-filters-header"
            )
            with Container(classes="section-content"):
                yield Static(
                    f"Sort: {self.done_sort_by.title()} {'↓' if self.done_sort_order == SortOrder.Descending else '↑'} | Filters: None | Stats: 0/0",
                    classes="filter-bar",
                    id="done-filter-bar",
                )

            # Completed tasks list
            yield Static(
                "✅ Completed Tasks", classes="section-header", id="done-header"
            )
            with Container(classes="section-content"):
                yield ListView(id="done-list")

            # Progress section (matching Rust TUI)
            yield Static(
                "Progress & Stats", classes="section-header", id="done-progress-header"
            )
            with Container(classes="section-content"):
                yield Static("", id="done-progress-content")

            yield Static("", classes="status-bar", id="done-status")

    def _compose_find_tab(self) -> ComposeResult:
        with Vertical():
            yield Static("Find", classes="section-header", id="find-header")
            with Container(classes="section-content"):
                yield Input(placeholder="🔍 Find:", id="search-input")
            yield Static("Results", classes="section-header", id="results-header")
            with Container(classes="section-content"):
                yield ListView(id="search-results")
            yield Static("", classes="status-bar", id="find-status")

    def _compose_more_tab(self) -> ComposeResult:
        with Vertical():
            yield Label("[bold]Extended Data[/bold]", id="more-header")
            yield Tabs(
                *[section.value for section in MoreTabSection.all()], id="more-tabs"
            )
            yield ScrollableContainer(id="more-content")
            yield Static("", classes="status-bar")

    def _compose_api_tab(self) -> ComposeResult:
        with Container(id="main-container"):
            with Vertical(id="left-panel"):
                yield Static(
                    "🖥️ Server Control", classes="section-header", id="api-server-header"
                )
                with Container(classes="section-content"):
                    yield Static("", id="server-status")
                    yield Button(
                        "Start Server", id="start-server", classes="action-button"
                    )
                    yield Button(
                        "Stop Server", id="stop-server", classes="action-button"
                    )
                    yield Button(
                        "Restart Server", id="restart-server", classes="action-button"
                    )
                    yield Button(
                        "Clear Cache", id="clear-cache", classes="action-button"
                    )

                yield Static(
                    "📋 API Endpoints",
                    classes="section-header",
                    id="api-endpoints-header",
                )
                with Container(classes="section-content"):
                    yield ScrollableContainer(id="api-endpoints-list")

            with Vertical(id="right-panel"):
                yield Static(
                    "🔑 API Keys", classes="section-header", id="api-keys-header"
                )
                with Container(classes="section-content"):
                    yield ScrollableContainer(id="api-keys-list")
                    yield Button(
                        "Create New API Key",
                        id="create-api-key",
                        classes="action-button",
                    )

    def _compose_feed_tab(self) -> ComposeResult:
        with Container(id="main-container"):
            with Vertical(id="left-panel"):
                yield Static(
                    "📰 Live Activity Stream",
                    classes="section-header",
                    id="feed-header",
                )
                with Container(classes="section-content"):
                    yield ActivityFeedWidget(id="feed-content", classes="activity-feed")

            with Vertical(id="right-panel"):
                yield Static(
                    "📊 Feed Stats & Controls",
                    classes="section-header",
                    id="feed-stats-header",
                )
                with Container(classes="section-content"):
                    yield Static("", id="feed-stats")

                # Add charts if enhancements available
                if ENHANCEMENTS_AVAILABLE:
                    yield Static(
                        "📊 Charts", classes="section-header", id="feed-charts-header"
                    )
                    with Container(
                        classes="section-content", id="feed-charts-container"
                    ):
                        # Charts will be added dynamically
                        pass

    def _compose_bye_tab(self) -> ComposeResult:
        with Vertical():
            yield Static(
                """
[bold] _______        _            [/bold]
[bold]|__   __|      | |        (✓)[/bold]
[bold]   | | ___   __| | ___ _____[/bold]
[bold]   | |/ _ \\ / _` |/ _ \\_  / |[/bold]
[bold]   | | (_) | (_| | (_) / /| |[/bold]
[bold]   |_|\\___/ \\__,_|\\___/___|_|[/bold]

Are you sure you want to leave Todozi?
Press Enter to confirm exit

Thank you for using Todozi! 🎉
""",
                id="bye-message",
            )

    def _search_tasks(self, query: str) -> List[Task]:
        """Search tasks by action, tags, or parent project."""
        query = query.lower()
        return [
            t
            for t in self.tasks
            if query in (getattr(t, "action", "") or "").lower()
            or query in " ".join(getattr(t, "tags", []) or []).lower()
            or query in (getattr(t, "parent_project", "") or "").lower()
        ]

    def update_task_list(self) -> None:
        """Update the task list widget with enhanced display and analytics"""
        try:
            task_list = self.query_one("#task-list", EnhancedTaskListWidget)
            task_list.update_tasks(
                self.filtered_tasks, self.selected_task_index, self.display_config
            )
            logger.debug(f"Updated task list with {len(self.filtered_tasks)} tasks")
        except NoMatches:
            # Widget doesn't exist yet (tab not active), skip update
            return
        except Exception as e:
            logger.debug(f"Error updating task list widget: {e}")
            # Widget might not be mounted yet, try again later
            return

        # Update detail view (optional, might not exist)
        try:
            if self.filtered_tasks and 0 <= self.selected_task_index < len(
                self.filtered_tasks
            ):
                task = self.filtered_tasks[self.selected_task_index]
                detail = self.query_one("#task-detail", TaskDetailWidget)
                detail.update_task(task)
            else:
                detail = self.query_one("#task-detail", TaskDetailWidget)
                detail.update_task(None)
        except Exception:
            pass  # Detail widget might not exist

        # Update analytics (optional, might not exist)
        try:
            analytics = self.query_one("#task-analytics", AnalyticsWidget)
            analytics.update_analytics(
                self.tasks, self.completion_data, self.priority_distribution
            )
        except Exception:
            pass  # Analytics widget might not exist

        # Update status bars
        try:
            filter_status = self.query_one("#filter-status", Static)
            filters_active = []
            if self.task_filters.status_filter:
                status_name = (
                    self.task_filters.status_filter.value
                    if hasattr(self.task_filters.status_filter, "value")
                    else str(self.task_filters.status_filter)
                )
                filters_active.append(f"Status={status_name}")
            if self.task_filters.priority_filter:
                priority_name = (
                    self.task_filters.priority_filter.value
                    if hasattr(self.task_filters.priority_filter, "value")
                    else str(self.task_filters.priority_filter)
                )
                filters_active.append(f"Priority={priority_name}")
            if self.task_filters.project_filter:
                filters_active.append(f"Project={self.task_filters.project_filter}")

            filter_text = f"| Filters: {', '.join(filters_active) if filters_active else 'None'} | "
            filter_text += f"Status: {self.task_filters.status_filter.value if self.task_filters.status_filter and hasattr(self.task_filters.status_filter, 'value') else 'All'} | "
            filter_text += f"Priority: {self.task_filters.priority_filter.value if self.task_filters.priority_filter and hasattr(self.task_filters.priority_filter, 'value') else 'All'} | "
            filter_text += f"Project: {self.task_filters.project_filter or 'All'} | F1-F4: Set filter"
            filter_status.update(filter_text)
        except Exception as e:
            logger.debug(f"Error updating filter status: {e}")

        try:
            task_status = self.query_one("#task-status", Static)
            status_text = f"Tasks: {len(self.filtered_tasks)} | "
            status_text += f"Project: {self.current_project or 'All'} | "
            status_text += "↑↓ Navigate | Enter Edit | Space Actions | F1-F4 Filters"
            task_status.update(status_text)
        except Exception as e:
            logger.debug(f"Error updating task status: {e}")

        # Update action menu if visible
        if self.task_action_menu is not None:
            try:
                self._render_task_action_menu()
            except Exception as e:
                logger.debug(f"Error rendering task action menu: {e}")

    def update_projects_list(self) -> None:
        """Update the projects list with proper empty state"""
        try:
            projects_list = self.query_one("#projects-list", ListView)
            projects_list.clear()

            # Ensure we have projects data
            if not hasattr(self, "projects") or self.projects is None:
                self.projects = []

            if not self.projects:
                projects_list.append(
                    ListItem(Label("+ Add New Project", classes="empty-state"))
                )
                logger.debug("Projects list is empty, showing add new project message")
            else:
                logger.debug(
                    f"Updating projects list with {len(self.projects)} projects"
                )
                for project in self.projects:
                    project_name = getattr(project, "name", str(project))
                    projects_list.append(ListItem(Label(f"📁 {project_name}")))
                logger.debug(
                    f"Successfully added {len(self.projects)} projects to list"
                )

            # Update status
            try:
                status = self.query_one("#projects-status", Static)
                count = len(self.projects)
                status.update(f"Active Projects: {count}")
            except NoMatches:
                pass  # Widget doesn't exist yet (tab not active)
            except Exception as e:
                logger.debug(f"Error updating projects status: {e}")
        except NoMatches:
            # Widget doesn't exist yet (tab not active), skip update
            logger.debug("Projects list widget not found (tab not active)")
            pass
        except Exception as e:
            logger.error(f"Error updating projects list: {e}", exc_info=True)
            # Widget might not be mounted yet

    def update_done_list(self) -> None:
        """Update the completed tasks list with filtering and sorting (matching Rust TUI)"""
        try:
            done_list = self.query_one("#done-list", ListView)
            done_list.clear()

            # Filter completed tasks
            completed_tasks = [
                t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]
            ]

            # Apply filters
            if self.done_filters.project_filter:
                completed_tasks = [
                    t
                    for t in completed_tasks
                    if getattr(t, "parent_project", "")
                    == self.done_filters.project_filter
                ]

            if self.done_filters.priority_filter:
                completed_tasks = [
                    t
                    for t in completed_tasks
                    if t.priority == self.done_filters.priority_filter
                ]

            # Sort tasks (matching Rust TUI)
            def sort_key(task: Task):
                if self.done_sort_by == TaskSortBy.DateCompleted:
                    return getattr(task, "updated_at", datetime.min)
                elif self.done_sort_by == TaskSortBy.DateCreated:
                    return getattr(task, "created_at", datetime.min)
                elif self.done_sort_by == TaskSortBy.Priority:
                    priority_order = {
                        Priority.LOW: 0,
                        Priority.MEDIUM: 1,
                        Priority.HIGH: 2,
                        Priority.CRITICAL: 3,
                        Priority.URGENT: 4,
                    }
                    return priority_order.get(task.priority, 0)
                elif self.done_sort_by == TaskSortBy.Project:
                    return getattr(task, "parent_project", "")
                elif self.done_sort_by == TaskSortBy.Action:
                    return task.action.lower()
                elif self.done_sort_by == TaskSortBy.Time:
                    return getattr(task, "time", "")
                elif self.done_sort_by == TaskSortBy.Assignee:
                    assignee_str = str(getattr(task, "assignee", ""))
                    return assignee_str
                return datetime.min

            completed_tasks.sort(
                key=sort_key, reverse=(self.done_sort_order == SortOrder.Descending)
            )

            if not completed_tasks:
                done_list.append(
                    ListItem(
                        Label(
                            "No completed tasks found with current filters",
                            classes="empty-state",
                        )
                    )
                )
            else:
                for i, task in enumerate(completed_tasks):
                    # Format time ago
                    updated_at = getattr(task, "updated_at", utcnow())
                    time_ago = format_duration(updated_at, utcnow())

                    # Priority indicator
                    priority_emoji = {
                        Priority.LOW: "🟢",
                        Priority.MEDIUM: "🟡",
                        Priority.HIGH: "🟠",
                        Priority.CRITICAL: "🔴",
                        Priority.URGENT: "🚨",
                    }.get(task.priority, "⚪")

                    display = f"✅ {priority_emoji} {task.action[:40]}"
                    if task.parent_project:
                        display += f" [{task.parent_project[:15]}]"
                    display += f" • {time_ago}"

                    # Highlight selected
                    if i == self.done_selected_task_index:
                        display = f"[bold]{display}[/bold]"

                    done_list.append(ListItem(Label(display)))

            # Update filter bar and status
            try:
                filter_bar = self.query_one("#done-filter-bar", Static)
                sort_icon = "↓" if self.done_sort_order == SortOrder.Descending else "↑"
                filter_text = f"Sort: {self.done_sort_by.title()} {sort_icon} | "
                filter_text += f"Filters: "
                filters_active = []
                if self.done_filters.project_filter:
                    filters_active.append(f"Project={self.done_filters.project_filter}")
                if self.done_filters.priority_filter:
                    priority_name = (
                        self.done_filters.priority_filter.value
                        if hasattr(self.done_filters.priority_filter, "value")
                        else str(self.done_filters.priority_filter)
                    )
                    filters_active.append(f"Priority={priority_name}")
                filter_text += ", ".join(filters_active) if filters_active else "None"
                filter_text += f" | Stats: {len(completed_tasks)}/{len(self.tasks)}"
                filter_bar.update(filter_text)
            except Exception:
                pass

            try:
                status = self.query_one("#done-status", Static)
                status.update(
                    f"Completed Tasks: {len(completed_tasks)} | ↑↓ Navigate | Enter Details | s=Sort o=Order r=Reset"
                )
            except Exception:
                pass

            # Update progress section (matching Rust TUI)
            try:
                progress_content = self.query_one("#done-progress-content", Static)
                total_tasks = len(self.tasks)
                completed_count = len(
                    [
                        t
                        for t in self.tasks
                        if t.status in [Status.DONE, Status.COMPLETED]
                    ]
                )

                # Calculate weekly stats
                now = utcnow()
                week_ago = now - timedelta(weeks=1)
                this_week_completed = len(
                    [
                        t
                        for t in self.tasks
                        if t.status in [Status.DONE, Status.COMPLETED]
                        and hasattr(t, "updated_at")
                        and t.updated_at
                        and t.updated_at > week_ago
                    ]
                )
                weekly_goal = 15
                weekly_progress_pct = (
                    (this_week_completed / weekly_goal * 100) if weekly_goal > 0 else 0
                )

                # Generate progress bar
                max_bar_width = 30
                filled = int((weekly_progress_pct / 100) * max_bar_width)
                progress_bar = "█" * filled + "░" * (max_bar_width - filled)

                completion_rate = (
                    (completed_count / total_tasks * 100) if total_tasks > 0 else 0
                )

                progress_text = f"""
[bold]🎯 Weekly Goal[/bold]
{progress_bar} {weekly_progress_pct:.1f}%
  {this_week_completed}/{weekly_goal} completed this week

[bold]📊 Statistics[/bold]
{completion_rate:.1f}% Complete
{completed_count}/{total_tasks} Total
"""
                progress_content.update(progress_text)
            except Exception:
                pass
        except Exception:
            pass

    def update_search_results(self) -> None:
        """Update search results list"""
        try:
            results_list = self.query_one("#search-results", ListView)
            results_list.clear()

            if not self.search_results:
                results_list.append(
                    ListItem(Label("No results found", classes="empty-state"))
                )
            else:
                for task in self.search_results:
                    display = f"{task.action[:50]}"
                    if task.parent_project:
                        display += f" [{task.parent_project}]"
                    results_list.append(ListItem(Label(display)))

            # Update status
            try:
                status = self.query_one("#find-status", Static)
                status.update(f"Search Results: {len(self.search_results)}")
            except Exception:
                pass
        except Exception:
            pass

    def update_more_content(self) -> None:
        """Update the content in the More tab based on selected section - enhanced rendering"""
        try:
            content_container = self.query_one("#more-content", ScrollableContainer)
            content_container.remove_children()

            if self.more_tab_section == MoreTabSection.Ideas:
                header = Static(
                    "[bold]💡 Creative Ideas & Concepts[/bold]\n\nYour space for brainstorming and capturing innovative thoughts.\n",
                    classes="section-header",
                )
                content_container.mount(header)
                if self.ideas:
                    for i, idea in enumerate(self.ideas, 1):
                        idea_text = getattr(idea, "idea", "Unknown idea")
                        content_container.mount(
                            Static(f"[bold]{i}.[/bold] {idea_text[:100]}")
                        )
                else:
                    content_container.mount(
                        Static("[dim]No ideas yet. Start brainstorming![/dim]")
                    )

            elif self.more_tab_section == MoreTabSection.Memories:
                header = Static(
                    "[bold]🧠 Personal Memories & Moments[/bold]\n\nCapture and cherish your special moments.\n",
                    classes="section-header",
                )
                content_container.mount(header)
                if self.memories:
                    for i, memory in enumerate(self.memories, 1):
                        moment = getattr(memory, "moment", "Unknown moment")
                        meaning = getattr(memory, "meaning", "")
                        content_container.mount(
                            Static(
                                f"[bold]{i}.[/bold] [italic]{moment[:50]}[/italic]: {meaning[:100]}"
                            )
                        )
                else:
                    content_container.mount(
                        Static("[dim]No memories recorded yet.[/dim]")
                    )

            elif self.more_tab_section == MoreTabSection.Feelings:
                header = Static(
                    "[bold]😊 Emotional Tracking[/bold]\n\nTrack your emotional journey and well-being.\n",
                    classes="section-header",
                )
                content_container.mount(header)
                if self.feelings:
                    for i, feeling in enumerate(self.feelings, 1):
                        emotion = getattr(feeling, "emotion", "Unknown")
                        intensity = getattr(feeling, "intensity", 5)
                        intensity_bar = "█" * intensity + "░" * (10 - intensity)
                        content_container.mount(
                            Static(
                                f"[bold]{i}.[/bold] {emotion} [{intensity_bar}] {intensity}/10"
                            )
                        )
                else:
                    content_container.mount(
                        Static("[dim]No feelings logged yet.[/dim]")
                    )

            elif self.more_tab_section == MoreTabSection.Errors:
                header = Static(
                    "[bold]❌ Error Logs & Debugging[/bold]\n\nSystem errors and debugging information.\n",
                    classes="section-header",
                )
                content_container.mount(header)
                if self.errors:
                    for i, error in enumerate(self.errors, 1):
                        timestamp = getattr(error, "timestamp", "Unknown time")
                        message = getattr(error, "message", "Unknown error")
                        content_container.mount(
                            Static(
                                f"[bold red]{i}.[/bold red] [{timestamp}] {message[:100]}"
                            )
                        )
                else:
                    content_container.mount(Static("[dim]No errors logged.[/dim]"))

            elif self.more_tab_section == MoreTabSection.Training:
                header = Static(
                    "[bold]🎓 AI Training Data[/bold]\n\nData used for AI model training and improvement.\n",
                    classes="section-header",
                )
                content_container.mount(header)
                if self.training_data:
                    for i, data in enumerate(self.training_data, 1):
                        category = getattr(data, "data_type", "Unknown")
                        prompt = getattr(data, "prompt", "No prompt")[:50]
                        response = getattr(data, "response", "")[:50]
                        content_container.mount(
                            Static(
                                f"[bold]{i}.[/bold] [{category}] {prompt}... → {response}..."
                            )
                        )
                else:
                    content_container.mount(
                        Static("[dim]No training data available.[/dim]")
                    )

            elif self.more_tab_section == MoreTabSection.Queue:
                header = Static(
                    "[bold]📋 Processing Queue[/bold]\n\nTasks waiting to be processed.\n",
                    classes="section-header",
                )
                content_container.mount(header)
                if self.queue_items:
                    for i, item in enumerate(self.queue_items, 1):
                        task_name = getattr(item, "task_name", "Unknown task")
                        status = getattr(item, "status", "Unknown status")
                        content_container.mount(Static(f"• {task_name}: {status}"))
                else:
                    content_container.mount(Static("Queue is empty."))

            elif self.more_tab_section == MoreTabSection.Reminders:
                content_container.mount(
                    Static(
                        "🔔 Scheduled Reminders\n\nYour upcoming reminders and notifications."
                    )
                )
                if self.reminders:
                    for reminder in self.reminders:
                        title = getattr(reminder, "title", "Untitled")
                        message = getattr(reminder, "message", "")
                        content_container.mount(Static(f"• {title}: {message[:100]}"))
                else:
                    content_container.mount(Static("No reminders set."))

            elif self.more_tab_section == MoreTabSection.Analytics:
                content_container.mount(
                    Static(
                        "📊 Analytics & Insights\n\nDetailed statistics and performance metrics."
                    )
                )
                content_container.mount(Static(f"Total Tasks: {len(self.tasks)}"))
                content_container.mount(
                    Static(
                        f"Active Tasks: {len([t for t in self.tasks if t.status not in [Status.DONE, Status.COMPLETED]])}"
                    )
                )
                content_container.mount(
                    Static(
                        f"Completed Tasks: {len([t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]])}"
                    )
                )
                content_container.mount(Static(f"Projects: {len(self.projects)}"))

        except Exception as e:
            logger.debug(f"Failed to update more content: {e}")

    def update_api_keys_list(self) -> None:
        """Update API keys list"""
        try:
            api_keys_container = self.query_one("#api-keys-list", ScrollableContainer)
            api_keys_container.remove_children()

            if not self.api_keys:
                api_keys_container.mount(
                    Static("No API keys configured", classes="empty-state")
                )
            else:
                for key in self.api_keys:
                    key_name = getattr(
                        key, "name", f"Key {getattr(key, 'user_id', 'unknown')}"
                    )
                    active = "✓" if getattr(key, "active", True) else "✗"
                    api_keys_container.mount(Static(f"{active} {key_name}"))
        except NoMatches:
            # Widget doesn't exist yet (tab not active), skip update
            pass
        except Exception as e:
            logger.debug(f"Error updating API keys list: {e}")

    def update_api_endpoints_list(self) -> None:
        """Update API endpoints list"""
        try:
            endpoints_container = self.query_one(
                "#api-endpoints-list", ScrollableContainer
            )
            endpoints_container.remove_children()

            endpoints = [
                ("GET", "/api/tasks", "List all tasks with optional filters"),
                ("POST", "/api/tasks", "Create new task"),
                ("PUT", "/api/tasks/:id", "Update existing task"),
                ("DELETE", "/api/tasks/:id", "Delete task"),
                ("POST", "/api/tasks/:id/complete", "Mark task as complete"),
                ("GET", "/api/projects", "List all projects"),
                (
                    "GET",
                    "/api/embeddings/search",
                    "Search tasks using semantic similarity",
                ),
                ("POST", "/api/embeddings/generate", "Generate embeddings for content"),
            ]

            for method, path, description in endpoints:
                method_color = {
                    "GET": "green",
                    "POST": "yellow",
                    "PUT": "blue",
                    "DELETE": "red",
                }.get(method, "white")

                endpoint_text = (
                    f"[bold {method_color}]{method}[/] {path}\n  {description}"
                )
                endpoints_container.mount(Static(endpoint_text))
        except NoMatches:
            # Widget doesn't exist yet (tab not active), skip update
            pass
        except Exception as e:
            logger.debug(f"Error updating API endpoints list: {e}")

    def update_server_status(self) -> None:
        """Update server status display"""
        try:
            server_status_widget = self.query_one("#server-status", Static)
            status_icon = "🟢" if self.server_running else "🔴"
            status_text = f"{status_icon} {self.server_status}"
            server_status_widget.update(status_text)
        except NoMatches:
            # Widget doesn't exist yet (tab not active), skip update
            pass
        except Exception as e:
            logger.debug(f"Error updating server status: {e}")

    def update_feed_stats(self) -> None:
        """Update feed statistics and charts"""
        try:
            feed_stats = self.query_one("#feed-stats", Static)
            stats_text = f"""
📊 Quick Stats
• Total Tasks: {len(self.tasks)}
• Active: {len([t for t in self.tasks if t.status not in [Status.DONE, Status.COMPLETED]])}
• Completed: {len([t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]])}
• Ideas: {len(self.ideas)}
• Memories: {len(self.memories)}

🎯 Navigation
• ↑↓: Scroll Activity
• u/d: Scroll Activity
• R: Refresh data
• F5: Force update
• Tab: Switch tabs

💡 Live Updates
• Auto-refresh: ON
• Real-time: ON
"""
            feed_stats.update(stats_text)

            # Update charts if enhancements available
            if ENHANCEMENTS_AVAILABLE:
                try:
                    charts_container = self.query_one(
                        "#feed-charts-container", Container
                    )
                    charts_container.remove_children()

                    # Create and add charts
                    charts_widget = FeedChartsWidget(
                        self.tasks,
                        self.ideas,
                        self.memories,
                        self.queue_items,
                        self.training_data,
                        id="feed-charts",
                    )
                    charts_container.mount(charts_widget)
                except Exception:
                    pass  # Charts container might not exist yet
        except Exception:
            pass

    def update_toasts(self) -> None:
        """Update toast notifications and render them."""
        now = time.time()
        self.toast_notifications[:] = [
            t for t in self.toast_notifications if not t.is_expired(now)
        ]

        # Update toast widget
        try:
            toast_widget = self.query_one("#toast-widget", ToastWidget)
            toast_widget.update_toasts(self.toast_notifications)
        except Exception:
            pass

        # Update status bar with context-aware messages
        try:
            footer = self.query_one("Footer", Footer)
            if self.current_tab == AppTab.Tasks:
                footer.update(
                    f"Tasks: {len(self.filtered_tasks)} | Project: {self.current_project or 'All'} | ↑↓ Navigate | Enter Edit | Space Actions"
                )
            elif self.current_tab == AppTab.Projects:
                footer.update(
                    f"Projects: {len(self.projects)} | ↑↓ Navigate | Enter Select | Shift+A Add"
                )
            elif self.current_tab == AppTab.Done:
                completed = len(
                    [
                        t
                        for t in self.tasks
                        if t.status in [Status.DONE, Status.COMPLETED]
                    ]
                )
                footer.update(
                    f"Completed: {completed} | ↑↓ Navigate | Enter Details | s=Sort o=Order"
                )
            elif self.current_tab == AppTab.Feed:
                footer.update(f"Feed | R=Refresh | F5=Force Update | Tab=Switch")
            elif self.current_tab == AppTab.Api:
                status = "🟢 Running" if self.server_running else "🔴 Stopped"
                footer.update(
                    f"API Server: {status} | S=Start X=Stop R=Restart C=Clear"
                )
            else:
                footer.update("Todozi TUI | Tab=Switch | ?=Help | q=Quit")
        except Exception:
            pass

    @on(ListView.Selected)
    def on_task_selected(self, event: ListView.Selected) -> None:
        """Handle task selection"""
        if isinstance(event.item, ListItem):
            index = event.list_view.highlighted
            if 0 <= index < len(self.filtered_tasks):
                self.selected_task_index = index
                task = self.filtered_tasks[index]
                try:
                    detail = self.query_one("#task-detail", TaskDetailWidget)
                    detail.update_task(task)
                except Exception:
                    pass

    async def action_add_project(self) -> None:
        """Add a new project with improved error handling"""
        try:
            from todozi.models import Err, Ok, Project

            # Simple project creation - in a real app you'd have a modal
            project_name = f"New Project {len(self.projects) + 1}"
            project_result = Project.new(
                name=project_name,
                description="New project description",
                user_id="tui_user",
            )

            if isinstance(project_result, Err):
                self._handle_error(
                    "Creating project", Exception(str(project_result.error))
                )
                return

            project = (
                project_result.value
                if isinstance(project_result, Ok)
                else project_result
            )
            await self.storage.add_project(project)
            self.log_activity("success", f"Project added: {project.name}")
            await self.refresh_data()
            self.update_projects_list()
            self.toast_notifier.success(f"Project '{project.name}' added!")
        except Exception as e:
            self._handle_error("Adding project", e)

    async def action_add_task(self) -> None:
        """Add a new task with improved error handling"""
        if not self._ensure_ready():
            return

        try:
            # Create a simple task using storage.py Task class
            task = Task(
                action="New task - edit me",
                priority=Priority.MEDIUM,
                parent_project=self.current_project or "general",
                status=Status.TODO,
                assignee=None,
                context_notes="Add details here",
            )

            await self.storage.add_task_to_project(task)
            self.log_activity("success", f"Task added: {task.action}")
            await self.refresh_data()
            self.update_task_list()
            self.toast_notifier.success("Task added! Press 'e' to edit")
        except Exception as e:
            self._handle_error("Adding task", e)

    async def action_delete_task(self) -> None:
        """Delete the selected task with improved error handling"""
        if not self._ensure_ready():
            return

        task = self._current_task()
        if not task:
            return

        try:
            self.storage.delete_task_from_project(task.id)
            self.log_activity("warning", f"Task deleted: {task.action}")
            await self.refresh_data()
            self.selected_task_index = min(
                self.selected_task_index, len(self.filtered_tasks) - 1
            )
            self.update_task_list()
            self.toast_notifier.success("Task deleted")
        except Exception as e:
            self._handle_error("Deleting task", e)

    async def action_complete_task(self) -> None:
        """Mark the selected task as complete with improved error handling"""
        if not self._ensure_ready():
            return

        task = self._current_task()
        if not task:
            return

        try:
            self.storage.complete_task_in_project(task.id)
            self.log_activity("success", f"Task completed: {task.action}")
            await self.refresh_data()
            self.update_task_list()
            self.toast_notifier.success("Task completed!")
        except Exception as e:
            self._handle_error("Completing task", e)

    async def action_edit_task(self) -> None:
        """Edit the selected task - use full editor screen overlay if available"""
        if not self.filtered_tasks:
            return

        if 0 <= self.selected_task_index < len(self.filtered_tasks):
            task = self.filtered_tasks[self.selected_task_index]

            # Use full editor screen overlay if enhancements available
            if ENHANCEMENTS_AVAILABLE:
                # Show editor as full screen overlay
                try:
                    editor_overlay = self.query_one("#editor-overlay", Container)
                    editor_overlay.remove_children()

                    # Create editor screen content
                    editor_screen = FullTaskEditorScreen(task)
                    # Mount editor content to overlay
                    editor_overlay.mount(editor_screen)
                    editor_overlay.remove_class("hidden")

                    # Wait for editor to complete
                    result = await editor_screen.wait_for_dismiss()
                    if result:
                        # Save the edited task
                        try:
                            await self.storage.update_task_in_project(result)
                            await self.refresh_data()
                            self.update_task_list()
                            self.toast_notifier.success("Task updated!")
                        except Exception as e:
                            self._handle_error("Updating task", e)

                    # Hide overlay
                    editor_overlay.add_class("hidden")
                    editor_overlay.remove_children()
                except Exception:
                    # Fallback to modal
                    editor_screen = FullTaskEditorScreen(task)
                    result = await self.push_screen_wait(editor_screen)
                    if result:
                        try:
                            await self.storage.update_task_in_project(result)
                            await self.refresh_data()
                            self.update_task_list()
                            self.toast_notifier.success("Task updated!")
                        except Exception as e:
                            self._handle_error("Updating task", e)
            else:
                # Fallback to modal editor
                self.editor = EditSession(
                    task_id=task.id,
                    original_task=task,
                    current_task=task,
                    ai_suggestions=[],
                    validation_errors=[],
                    session_start=utcnow(),
                )
                self.editor_selected_field = 0
                self.editor_input = ""
                self.push_screen("editor")

    async def action_edit_task_modal(self) -> None:
        """Edit task using modal interface"""
        if not self.filtered_tasks or not (
            0 <= self.selected_task_index < len(self.filtered_tasks)
        ):
            return

        task = self.filtered_tasks[self.selected_task_index]
        modal = EditTaskModal(task)
        result = await self.push_screen_wait(modal)

        if result:
            # Update the task
            self.save_task(result)
            self.apply_filters()
            self.update_task_list()
            self.toast_notifier.success("Task updated")

    async def action_task_actions(self) -> None:
        """Show task action menu - inline or modal"""
        if not self.filtered_tasks or not (
            0 <= self.selected_task_index < len(self.filtered_tasks)
        ):
            return

        task = self.filtered_tasks[self.selected_task_index]
        actions = [
            TaskAction.Edit,
            TaskAction.Complete,
            TaskAction.ViewDetails,
            TaskAction.Duplicate,
        ]

        # Show inline action menu if on tasks tab
        if self.current_tab == AppTab.Tasks:
            self.task_action_menu = self.selected_task_index
            self.task_action_selected = 0
            self._render_task_action_menu()
        else:
            # Use modal for other tabs
            modal = TaskActionMenu(task, actions)
            result = await self.push_screen_wait(modal)
            if result:
                action, task = result
                await self._execute_task_action(action, task)

    def _render_task_action_menu(self) -> None:
        """Render inline task action menu matching Rust TUI"""
        try:
            menu_container = self.query_one("#task-action-menu-container", Container)
            menu_widget = self.query_one("#task-action-menu", Static)

            if self.task_action_menu is not None and 0 <= self.task_action_menu < len(
                self.filtered_tasks
            ):
                task = self.filtered_tasks[self.task_action_menu]
                menu_items = [
                    "✏️ Edit Task",
                    "✅ Complete Task",
                    "👀 View Details",
                    "📋 Edit Steps",
                ]

                menu_text = "[bold]Actions[/bold]\n"
                for i, item in enumerate(menu_items):
                    marker = "▶" if i == self.task_action_selected else " "
                    menu_text += f"{marker} {item}\n"

                menu_widget.update(menu_text)
                menu_container.remove_class("hidden")
            else:
                menu_container.add_class("hidden")
        except Exception:
            pass

    async def action_task_enter(self) -> None:
        """Handle Enter key - context aware (matching Rust TUI)"""
        # Handle action menu if open
        if self.task_action_menu is not None:
            await self._execute_inline_action_menu()
            return

        # Context-aware Enter handling
        if self.current_tab == AppTab.Projects:
            # Projects tab: Create new or select project
            if self.selected_project_index == 0:
                await self.action_add_project()
            elif 0 < self.selected_project_index <= len(self.projects):
                project_index = self.selected_project_index - 1
                if project_index < len(self.projects):
                    project = self.projects[project_index]
                    project_name = (
                        project.name if hasattr(project, "name") else str(project)
                    )
                    self.current_project = project_name
                    self.task_filters.project_filter = project_name
                    self.apply_filters()
                    self.current_tab = AppTab.Tasks
                    self.selected_task_index = 0
                    self.switch_tab(AppTab.Tasks)
                    self.toast_notifier.info(f"Switched to project: {project_name}")

        elif self.current_tab == AppTab.Tasks:
            # Tasks tab: Create new task or open action menu
            if self.selected_task_index == 0:
                await self.action_add_task()
            else:
                # Open action menu
                await self.action_task_actions()

        elif self.current_tab == AppTab.Done:
            # Done tab: Show task details
            await self.action_task_details()

        elif self.current_tab == AppTab.Find:
            # Find tab: Open editor for selected task
            if self.search_results and 0 <= self.selected_task_index < len(
                self.search_results
            ):
                task = self.search_results[self.selected_task_index]
                if ENHANCEMENTS_AVAILABLE:
                    editor_screen = FullTaskEditorScreen(task)
                    result = await self.push_screen_wait(editor_screen)
                    if result:
                        try:
                            await self.storage.update_task_in_project(result)
                            await self.refresh_data()
                            self.update_search_results()
                            self.toast_notifier.success("Task updated!")
                        except Exception as e:
                            self._handle_error("Updating task", e)

        elif self.current_tab == AppTab.Api:
            # API tab: Create new API key or show details
            if self.api_selected_index == 0:
                await self.on_create_api_key(Button.Pressed())
            elif 0 < self.api_selected_index <= len(self.api_keys):
                api_index = self.api_selected_index - 1
                if api_index < len(self.api_keys):
                    self.show_api_key_details = self.api_keys[api_index]
                    # Show details modal would go here

        elif self.current_tab == AppTab.Bye:
            # Bye tab: Quit
            self.exit()

    async def action_task_details(self) -> None:
        """Show task details modal with steps support"""
        if not self.filtered_tasks or not (
            0 <= self.selected_task_index < len(self.filtered_tasks)
        ):
            return

        task = self.filtered_tasks[self.selected_task_index]

        # Use enhanced modal with steps if available
        if ENHANCEMENTS_AVAILABLE:
            modal = TaskDetailsWithStepsModal(task)
        else:
            modal = TaskDetailsModal(task)
        await self.push_screen_wait(modal)

    async def action_delete_task_confirm(self) -> None:
        """Delete task with confirmation"""
        if not self._ensure_ready():
            return

        task = self._current_task()
        if not task:
            return

        # Show confirmation modal
        confirmed = await self._show_confirmation_dialog(
            f"Delete task '{task.action}'?"
        )
        if confirmed:
            await self.action_delete_task()

    async def action_refresh_all(self) -> None:
        """Full refresh of all data"""
        try:
            await self.refresh_data()
            self.update_task_list()
            self.toast_notifier.success("Full refresh completed")
            self.log_activity("info", "Full data refresh completed")
        except Exception as e:
            self._handle_error("Full refresh", e)

    async def action_save_all(self) -> None:
        """Save all data"""
        try:
            self.save_tasks()
            self.toast_notifier.success("All data saved")
            self.log_activity("info", "All data saved")
        except Exception as e:
            self._handle_error("Saving data", e)

    async def action_toggle_filters(self) -> None:
        """Toggle filter visibility"""
        # In a real implementation, this would toggle filter UI visibility
        self.toast_notifier.info("Filter toggle not implemented")

    async def action_clear_filters(self) -> None:
        """Clear all filters"""
        self.task_filters = TaskFilters()
        self.done_filters = TaskFilters()
        self.current_project = None
        self.filter_status = None
        self.apply_filters()
        self.update_task_list()
        self.toast_notifier.info("All filters cleared")

    async def action_toggle_display(self) -> None:
        """Toggle display options"""
        self.display_config.compact_mode = not self.display_config.compact_mode
        self.update_task_list()
        mode = "compact" if self.display_config.compact_mode else "normal"
        self.toast_notifier.info(f"Display mode: {mode}")

    async def _execute_task_action(self, action: TaskAction, task: Task) -> None:
        """Execute a task action"""
        if action == TaskAction.Edit:
            await self.action_edit_task()
        elif action == TaskAction.ViewDetails:
            await self.action_task_details()
        elif action == TaskAction.Complete:
            await self.action_complete_task()
        elif action == TaskAction.Duplicate:
            await self._duplicate_task(task)
        elif action == TaskAction.Delete:
            confirmed = await self._show_confirmation_dialog(
                f"Delete task '{task.action}'?"
            )
            if confirmed:
                await self.action_delete_task()
        elif action == TaskAction.MoveToProject:
            await self._move_task_to_project(task)

    async def _execute_inline_action_menu(self) -> None:
        """Execute action from inline action menu"""
        if self.task_action_menu is None or not (
            0 <= self.task_action_menu < len(self.filtered_tasks)
        ):
            return

        task = self.filtered_tasks[self.task_action_menu]
        menu_items = [
            TaskAction.Edit,
            TaskAction.Complete,
            TaskAction.ViewDetails,
            TaskAction.Duplicate,
        ]

        if 0 <= self.task_action_selected < len(menu_items):
            action = menu_items[self.task_action_selected]
            self.task_action_menu = None  # Close menu
            self._render_task_action_menu()
            await self._execute_task_action(action, task)

    async def _duplicate_task(self, task: Task) -> None:
        """Duplicate a task"""
        new_task = Task(
            action=f"{task.action} (Copy)",
            status=task.status,
            priority=task.priority,
            parent_project=task.parent_project,
            tags=task.tags.copy() if task.tags else [],
            context_notes=task.context_notes,
        )

        if self.storage:
            await self.storage.add_task_to_project(new_task)

        self.tasks.append(new_task)
        self.apply_filters()
        self.update_task_list()
        self.toast_notifier.success("Task duplicated")

    async def _move_task_to_project(self, task: Task) -> None:
        """Move task to different project"""
        # In a real implementation, this would show a project selection dialog
        self.toast_notifier.info("Move to project not implemented")

    async def _show_confirmation_dialog(self, message: str) -> bool:
        """Show a confirmation dialog"""
        # For now, just return True. In a real implementation, this would show a modal
        return True

    async def action_refresh(self) -> None:
        """Refresh data from storage"""
        try:
            await self.refresh_data()
            self.update_task_list()
            self.toast_notifier.success("Refreshed")
            self.log_activity("info", "Data refreshed")
        except Exception as e:
            self._handle_error("Refreshing", e)

    async def action_filter(self) -> None:
        """Toggle status filter"""
        try:
            statuses = [None, Status.TODO, Status.IN_PROGRESS, Status.DONE]
            current_idx = (
                statuses.index(self.filter_status)
                if self.filter_status in statuses
                else 0
            )
            next_idx = (current_idx + 1) % len(statuses)
            self.filter_status = statuses[next_idx]
            await self.refresh_data()
            self.update_task_list()
            status_name = self.filter_status.value if self.filter_status else "All"
            self.toast_notifier.info(f"Filter: {status_name}")
        except Exception as e:
            self._handle_error("Applying filter", e)

    def action_next_tab(self) -> None:
        """Switch to next tab"""
        tabs = AppTab.all()
        idx = tabs.index(self.current_tab)
        next_tab = tabs[(idx + 1) % len(tabs)]
        self.switch_tab(next_tab)

    def action_prev_tab(self) -> None:
        """Switch to previous tab"""
        tabs = AppTab.all()
        idx = tabs.index(self.current_tab)
        prev_tab = tabs[(idx - 1) % len(tabs)]
        self.switch_tab(prev_tab)

    def action_tab_1(self) -> None:
        self.current_tab = AppTab.Projects
        self.switch_tab(AppTab.Projects)

    def action_tab_2(self) -> None:
        self.current_tab = AppTab.Tasks
        self.switch_tab(AppTab.Tasks)

    def action_tab_3(self) -> None:
        self.current_tab = AppTab.Feed
        self.switch_tab(AppTab.Feed)

    def action_tab_4(self) -> None:
        self.current_tab = AppTab.Done
        self.switch_tab(AppTab.Done)

    def action_tab_5(self) -> None:
        self.current_tab = AppTab.Find
        self.switch_tab(AppTab.Find)

    def action_tab_6(self) -> None:
        self.current_tab = AppTab.More
        self.switch_tab(AppTab.More)

    def action_tab_7(self) -> None:
        self.current_tab = AppTab.Api
        self.switch_tab(AppTab.Api)

    def action_tab_8(self) -> None:
        self.current_tab = AppTab.Bye
        self.switch_tab(AppTab.Bye)

    def switch_tab(self, tab: AppTab) -> None:
        """Switch to a specific tab"""
        self.current_tab = tab
        tab_map = {
            AppTab.Projects: "projects",
            AppTab.Tasks: "tasks",
            AppTab.Done: "done",
            AppTab.Find: "find",
            AppTab.More: "more",
            AppTab.Api: "api",
            AppTab.Feed: "feed",
            AppTab.Bye: "bye",
        }
        try:
            tabbed_content = self.query_one(TabbedContent)
            tabbed_content.active = tab_map[tab]
            # Update UI for the new tab - use a small delay to ensure widgets are mounted
            self.call_next(self.update_all_ui)
            # Explicitly update tab-specific widgets after a brief delay to ensure they're mounted
            if tab == AppTab.Api:
                self.set_timer(0.1, self._update_api_tab_widgets)
            elif tab == AppTab.Projects:
                self.set_timer(0.1, self._update_projects_tab_widgets)
            elif tab == AppTab.Tasks:
                self.set_timer(0.1, self._update_tasks_tab_widgets)
        except Exception as e:
            logger.debug(f"Error switching tab: {e}")

    def _update_api_tab_widgets(self) -> None:
        """Update API tab widgets after tab is mounted"""
        self.update_api_keys_list()
        self.update_api_endpoints_list()
        self.update_server_status()

    def _update_projects_tab_widgets(self) -> None:
        """Update Projects tab widgets after tab is mounted"""
        self.update_projects_list()

    def _update_tasks_tab_widgets(self) -> None:
        """Update Tasks tab widgets after tab is mounted"""
        self.update_task_list()

    def add_toast(self, message: str, toast_type: ToastType) -> None:
        """Add a toast notification (delegates to ToastNotifier)"""
        self.toast_notifier._make(message, toast_type)

    def update_toasts(self) -> None:
        """Update toast notifications by removing expired ones"""
        now = time.time()
        # More efficient in-place filtering
        self.toast_notifications[:] = [
            t for t in self.toast_notifications if not t.is_expired(now)
        ]

    async def auto_refresh(self) -> None:
        """Auto-refresh data periodically"""
        try:
            await self.refresh_data()
            self.update_task_list()
        except Exception as e:
            logger.debug("Auto-refresh failed: %s", e)

    def start_file_watcher(self) -> None:
        """Start watching the Todozi storage directory for changes"""
        if not WATCHDOG_AVAILABLE or self.file_watcher_thread is not None:
            return

        # Resolve a sensible directory to watch
        watch_dir: Path = Path(__file__).resolve().parent
        try:
            from todozi.storage import get_storage_dir

            p = get_storage_dir()
            if p:
                watch_dir = Path(p) if isinstance(p, str) else p
        except Exception as exc:
            logger.debug("Failed to obtain storage dir: %s", exc)

        if not watch_dir.exists():
            logger.debug("Watch directory does not exist: %s", watch_dir)
            return

        def watch_files():
            try:
                self.observer = Observer()
                self.observer.daemon = True
                handler = TasksDirWatcher(self._on_files_changed)
                self.observer.schedule(handler, str(watch_dir), recursive=True)
                self.observer.start()
                logger.info("Started file watcher on %s", watch_dir)
                # Don't call UI methods from background thread - use call_from_thread instead
                try:
                    self.call_from_thread(
                        self.log_activity,
                        "info",
                        f"File watcher started on {watch_dir}",
                    )
                    self.call_from_thread(
                        self.toast_notifier.info,
                        "File watcher started - auto-refresh enabled",
                    )
                except Exception:
                    # If call_from_thread fails, just log it
                    logger.debug("Could not notify UI from file watcher thread")

                # Keep thread alive
                while not self.stop_watching:
                    time.sleep(1)

            except Exception as exc:
                logger.exception("File watcher error: %s", exc)
            finally:
                if self.observer:
                    try:
                        self.observer.stop()
                        self.observer.join(timeout=2.0)
                    except Exception as exc:
                        logger.exception("Error stopping watcher: %s", exc)

        self.file_watcher_thread = threading.Thread(target=watch_files, daemon=True)
        self.file_watcher_thread.start()

    def _on_files_changed(self) -> None:
        """Debounced entry point for the watchdog thread"""
        if self._watch_callback_scheduled:
            return
        self._watch_callback_scheduled = True
        # Schedule the async handler to run on the UI event loop
        self.call_later(self._handle_files_changed)

    async def _handle_files_changed(self) -> None:
        """Refresh UI after a change on disk"""
        self._watch_callback_scheduled = False
        self.log_activity("info", "Detected file changes; refreshing")
        try:
            await self.refresh_data()
            self.update_task_list()
        except Exception as e:
            logger.debug("File change handler failed: %s", e)

    def action_focus_search(self) -> None:
        """Focus the search input"""
        try:
            search_input = self.query_one("#search-input", Input)
            self.set_focus(search_input)
        except Exception as e:
            logger.debug("Search focus failed: %s", e)
            self.toast_notifier.warning("Search box not available")

    @on(Input.Submitted, "#search-input")
    def on_search_submitted(self, event: Input.Submitted) -> None:
        """Execute search when Enter is pressed"""
        self.search_query = (event.value or "").strip()
        if self.search_query:
            self.search_results = self._search_tasks(self.search_query)
            self.log_activity(
                "info",
                f"Search: '{self.search_query}' ({len(self.search_results)} results)",
            )
        else:
            self.search_results = []
        self.update_search_results()

    @on(Input.Changed, "#search-input")
    def on_search_changed(self, event: Input.Changed) -> None:
        """Execute search as user types"""
        self.search_query = (event.value or "").strip()
        if self.search_query:
            self.search_results = self._search_tasks(self.search_query)
        else:
            self.search_results = []
        self.update_search_results()

    @on(Tabs.TabActivated, "#more-tabs")
    def on_more_tab_changed(self, event: Tabs.TabActivated) -> None:
        """Handle tab changes in the More section"""
        try:
            # Get the active tab from the event
            active_tab = event.tab
            if active_tab:
                tab_label = active_tab.label
                all_sections = MoreTabSection.all()
                for section in all_sections:
                    if section.value == tab_label:
                        self.more_tab_section = section
                        self.update_more_content()
                        self.log_activity(
                            "info", f"Switched to {section.value} section"
                        )
                        break
        except Exception as e:
            logger.debug(f"Failed to handle more tab change: {e}")

    @on(Button.Pressed, "#start-server")
    def on_start_server(self, event: Button.Pressed) -> None:
        """Handle server start button"""
        event.stop()
        if not self.server_running:
            self.server_running = True
            started_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            try:
                widget = self.query_one("#server-status", Static)
                widget.update(
                    f"Status: 🟢 Running\nStarted: {started_time}\nEnvironment: Development\nBase URL: http://localhost:8636/api\n\nControls:\n[S]tart [X]top [R]estart [C]lear Cache"
                )
            except Exception:
                pass
            self.log_activity("info", "Server started")
            self.toast_notifier.success("Server started")
        else:
            self.toast_notifier.info("Server is already running")

    @on(Button.Pressed, "#stop-server")
    def on_stop_server(self, event: Button.Pressed) -> None:
        """Handle server stop button"""
        event.stop()
        if self.server_running:
            self.server_running = False
            try:
                widget = self.query_one("#server-status", Static)
                widget.update(
                    "Status: 🔴 Stopped\nStarted: -\nEnvironment: Development\nBase URL: http://localhost:8636/api\n\nControls:\n[S]tart [X]top [R]estart [C]lear Cache"
                )
            except Exception:
                pass
            self.log_activity("info", "Server stopped")
            self.toast_notifier.warning("Server stopped")
        else:
            self.toast_notifier.info("Server is already stopped")

    @on(Button.Pressed, "#restart-server")
    def on_restart_server(self, event: Button.Pressed) -> None:
        """Handle server restart button"""
        event.stop()
        if self.server_running:
            self.server_running = False
            self.log_activity("info", "Server restarting")
            self.toast_notifier.info("Restarting server...")
            # Simulate restart delay
            import asyncio

            asyncio.create_task(self._delayed_server_start())
        else:
            self.toast_notifier.warning("Server is not running")

    async def _delayed_server_start(self) -> None:
        """Delayed server start simulation"""
        await asyncio.sleep(1)  # Simulate restart time
        self.server_running = True
        started_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        try:
            widget = self.query_one("#server-status", Static)
            widget.update(
                f"Status: 🟢 Running\nStarted: {started_time}\nEnvironment: Development\nBase URL: http://localhost:8636/api\n\nControls:\n[S]tart [X]top [R]estart [C]lear Cache"
            )
        except Exception:
            pass
        self.log_activity("info", "Server restarted")
        self.toast_notifier.success("Server restarted")

    @on(Button.Pressed, "#clear-cache")
    def on_clear_cache(self, event: Button.Pressed) -> None:
        """Handle clear cache button"""
        event.stop()
        self.log_activity("info", "Cache cleared")
        self.toast_notifier.success("Cache cleared")

    @on(Button.Pressed, "#create-api-key")
    def on_create_api_key(self, event: Button.Pressed) -> None:
        """Handle create API key button"""
        event.stop()
        # For now, just simulate creating an API key
        # In a real implementation, this would create a proper API key
        self.log_activity("info", "API key created")
        self.toast_notifier.success("New API key created")
        # Refresh the API keys list
        self.update_api_keys_list()

    @on(Button.Pressed, "#add-project")
    def on_add_project(self, event: Button.Pressed) -> None:
        """Handle add project button"""
        event.stop()
        import asyncio

        asyncio.create_task(self.action_add_project())

    @on(ListView.Selected, "#projects-list")
    def on_project_selected(self, event: ListView.Selected) -> None:
        """Handle project selection"""
        if isinstance(event.item, ListItem):
            index = event.list_view.highlighted
            if 0 <= index < len(self.projects):
                project = self.projects[index]
                self.current_project = project.name
                self.log_activity("info", f"Selected project: {project.name}")
                self.toast_notifier.info(f"Switched to project: {project.name}")
                # Refresh task list to show only tasks from this project
                self.apply_filters()
                self.update_task_list()

    # Filter actions (F1-F4) matching Rust TUI
    async def action_filter_status(self) -> None:
        """Cycle through status filters (F1)"""
        if self.current_tab != AppTab.Tasks:
            return
        statuses = [
            None,
            Status.TODO,
            Status.IN_PROGRESS,
            Status.BLOCKED,
            Status.REVIEW,
            Status.DONE,
        ]
        current_idx = (
            statuses.index(self.task_filters.status_filter)
            if self.task_filters.status_filter in statuses
            else 0
        )
        next_idx = (current_idx + 1) % len(statuses)
        self.task_filters.status_filter = statuses[next_idx]
        self.filter_status = statuses[next_idx]
        self.apply_filters()
        self.update_task_list()
        status_name = (
            self.task_filters.status_filter.value
            if self.task_filters.status_filter
            else "All"
        )
        self.toast_notifier.info(f"Status filter: {status_name}")

    async def action_filter_priority(self) -> None:
        """Cycle through priority filters (F2)"""
        if self.current_tab != AppTab.Tasks:
            return
        priorities = [
            None,
            Priority.CRITICAL,
            Priority.URGENT,
            Priority.HIGH,
            Priority.MEDIUM,
            Priority.LOW,
        ]
        current_idx = (
            priorities.index(self.task_filters.priority_filter)
            if self.task_filters.priority_filter in priorities
            else 0
        )
        next_idx = (current_idx + 1) % len(priorities)
        self.task_filters.priority_filter = priorities[next_idx]
        self.apply_filters()
        self.update_task_list()
        priority_name = (
            self.task_filters.priority_filter.value
            if self.task_filters.priority_filter
            else "All"
        )
        self.toast_notifier.info(f"Priority filter: {priority_name}")

    async def action_filter_project(self) -> None:
        """Cycle through project filters (F3)"""
        if self.current_tab != AppTab.Tasks or not self.projects:
            return
        projects = [None] + [
            p.name if hasattr(p, "name") else str(p) for p in self.projects
        ]
        current_project = self.task_filters.project_filter
        current_idx = (
            projects.index(current_project) if current_project in projects else 0
        )
        next_idx = (current_idx + 1) % len(projects)
        self.task_filters.project_filter = projects[next_idx]
        self.current_project = projects[next_idx]
        self.apply_filters()
        self.update_task_list()
        project_name = self.task_filters.project_filter or "All"
        self.toast_notifier.info(f"Project filter: {project_name}")

    # Done tab sorting (matching Rust TUI)
    async def action_sort_done(self) -> None:
        """Cycle through sort options for Done tab (s)"""
        if self.current_tab != AppTab.Done:
            return
        sort_options = TaskSortBy.all()
        current_idx = (
            sort_options.index(self.done_sort_by)
            if self.done_sort_by in sort_options
            else 0
        )
        next_idx = (current_idx + 1) % len(sort_options)
        self.done_sort_by = sort_options[next_idx]
        self.update_done_list()
        self.toast_notifier.info(f"Sorting by: {self.done_sort_by.title()}")

    async def action_sort_order(self) -> None:
        """Toggle sort order for Done tab (o)"""
        if self.current_tab != AppTab.Done:
            return
        self.done_sort_order = (
            SortOrder.Ascending
            if self.done_sort_order == SortOrder.Descending
            else SortOrder.Descending
        )
        self.update_done_list()
        order_text = (
            "ascending" if self.done_sort_order == SortOrder.Ascending else "descending"
        )
        self.toast_notifier.info(f"Sort order: {order_text}")

    async def action_reset_done_filters(self) -> None:
        """Reset Done tab filters and sorting (Shift+R)"""
        if self.current_tab != AppTab.Done:
            return
        self.done_sort_by = TaskSortBy.DateCompleted
        self.done_sort_order = SortOrder.Descending
        self.done_filters = TaskFilters()
        self.update_done_list()
        self.toast_notifier.info("Sort and filters reset to defaults")

    # Navigation actions
    async def action_navigate_up(self) -> None:
        """Navigate up"""
        # Handle action menu navigation
        if self.task_action_menu is not None and self.current_tab == AppTab.Tasks:
            if self.task_action_selected > 0:
                self.task_action_selected -= 1
                self._render_task_action_menu()
            return

        if self.current_tab == AppTab.Tasks:
            if self.selected_task_index > 0:
                self.selected_task_index -= 1
                self.update_task_list()
        elif self.current_tab == AppTab.Projects:
            if self.selected_project_index > 0:
                self.selected_project_index -= 1
                self.update_projects_list()
        elif self.current_tab == AppTab.Done:
            if self.done_selected_task_index > 0:
                self.done_selected_task_index -= 1
                self.update_done_list()

    async def action_navigate_down(self) -> None:
        """Navigate down"""
        # Handle action menu navigation
        if self.task_action_menu is not None and self.current_tab == AppTab.Tasks:
            if self.task_action_selected < 3:  # 4 menu items (0-3)
                self.task_action_selected += 1
                self._render_task_action_menu()
            return

        if self.current_tab == AppTab.Tasks:
            if self.selected_task_index < len(self.filtered_tasks) - 1:
                self.selected_task_index += 1
                self.update_task_list()
        elif self.current_tab == AppTab.Projects:
            if self.selected_project_index < len(self.projects) - 1:
                self.selected_project_index += 1
                self.update_projects_list()
        elif self.current_tab == AppTab.Done:
            completed = [
                t for t in self.tasks if t.status in [Status.DONE, Status.COMPLETED]
            ]
            if self.done_selected_task_index < len(completed) - 1:
                self.done_selected_task_index += 1
                self.update_done_list()

    async def action_navigate_left(self) -> None:
        """Navigate left"""
        if self.current_tab == AppTab.More:
            sections = MoreTabSection.all()
            current_idx = sections.index(self.more_tab_section)
            self.more_tab_section = sections[(current_idx - 1) % len(sections)]
            self.update_more_content()
        else:
            self.action_prev_tab()

    async def action_navigate_right(self) -> None:
        """Navigate right"""
        if self.current_tab == AppTab.More:
            sections = MoreTabSection.all()
            current_idx = sections.index(self.more_tab_section)
            self.more_tab_section = sections[(current_idx + 1) % len(sections)]
            self.update_more_content()
        else:
            self.action_next_tab()

    # Done tab filter actions
    async def action_filter_done_project(self) -> None:
        """Filter Done tab by project (Shift+P)"""
        if self.current_tab != AppTab.Done or not self.projects:
            return
        projects = [None] + [
            p.name if hasattr(p, "name") else str(p) for p in self.projects
        ]
        current_project = self.done_filters.project_filter
        current_idx = (
            projects.index(current_project) if current_project in projects else 0
        )
        next_idx = (current_idx + 1) % len(projects)
        self.done_filters.project_filter = projects[next_idx]
        self.update_done_list()
        project_name = self.done_filters.project_filter or "None"
        self.toast_notifier.info(f"Project filter: {project_name}")

    async def action_filter_done_priority(self) -> None:
        """Filter Done tab by priority (Shift+I)"""
        if self.current_tab != AppTab.Done:
            return
        priorities = [
            None,
            Priority.CRITICAL,
            Priority.URGENT,
            Priority.HIGH,
            Priority.MEDIUM,
            Priority.LOW,
        ]
        current_priority = self.done_filters.priority_filter
        current_idx = (
            priorities.index(current_priority) if current_priority in priorities else 0
        )
        next_idx = (current_idx + 1) % len(priorities)
        self.done_filters.priority_filter = priorities[next_idx]
        self.update_done_list()
        priority_name = (
            self.done_filters.priority_filter.value
            if self.done_filters.priority_filter
            else "None"
        )
        self.toast_notifier.info(f"Priority filter: {priority_name}")

    async def action_escape(self) -> None:
        """Handle Escape key - close menus/modals"""
        # Close action menu if open
        if self.task_action_menu is not None:
            self.task_action_menu = None
            self._render_task_action_menu()
            return

        # Close task details if showing
        if self.show_task_details is not None:
            self.show_task_details = None
            self.update_task_list()
            return

    async def action_help(self) -> None:
        """Show help"""
        help_text = """
[bold]🚀 Enhanced Todozi TUI Help[/bold]

[bold]Navigation:[/bold]
  Tab/Shift+Tab  - Switch tabs
  1-8            - Direct tab access
  ↑/↓/←/→       - Navigate items
  Enter          - Select/Open details
  Space          - Task actions menu
  Esc            - Cancel/Back/Clear

[bold]Task Management:[/bold]
  a / Ctrl+A     - Add new task
  d / Ctrl+D     - Delete task (with confirm)
  c              - Complete selected task
  e / Ctrl+E     - Edit task (modal)
  r / Ctrl+R     - Full refresh
  f / Ctrl+F     - Toggle filters
  Ctrl+L         - Clear all filters
  Ctrl+T         - Toggle display mode

[bold]Filter Shortcuts (Tasks Tab):[/bold]
  F1             - Filter by Status
  F2             - Filter by Priority
  F3             - Filter by Project
  F4             - Clear all filters
  F5             - Refresh all data

[bold]Done Tab Controls:[/bold]
  s              - Change sort field
  o              - Toggle sort order
  Shift+R        - Reset filters & sort
  Shift+P        - Filter by project
  Shift+I        - Filter by priority

[bold]Quick Actions:[/bold]
  Space          - Task action menu
  Enter          - Task details modal
  /              - Focus search
  ?              - Show this help
  Ctrl+S         - Save all data

[bold]Tabs:[/bold]
  1 📁 Projects  - Manage projects
  2 📋 Tasks     - View and manage tasks
  3 📰 Feed      - Live activity feed
  4 ✅ Done      - Completed tasks
  5 🔍 Find      - Search tasks
  6 🔮 More      - Extended data (Ideas, Memories, etc.)
  7 🔑 API       - API management & server control
  8 👋 Bye       - Exit application

[bold]More Tab Sections:[/bold]
  💡 Ideas       - Creative ideas and concepts
  🧠 Memories    - Personal memories and moments
  😊 Feelings    - Emotional tracking
  ❌ Errors      - Error logs and debugging
  🎓 Training    - AI training data
  📋 Queue       - Task processing queue
  🔔 Reminders   - Scheduled reminders
  📊 Analytics   - Statistics and insights

[bold]Enhanced Features:[/bold]
  • Real-time file watching
  • Toast notifications
  • Modal dialogs for editing
  • Analytics dashboard
  • Activity feed
  • Server management
  • API key management
  • Priority distribution charts
  • Completion tracking

[bold]Tips:[/bold]
  • Use Space on any task for quick actions menu
  • Ctrl+E for modal editing with field navigation
  • All changes are auto-saved
  • Analytics update in real-time
  • Activity feed shows all operations
"""
        self.notify(help_text, severity="information", timeout=15)


def main() -> None:
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Todozi TUI - AI/Human Task Management System"
    )
    parser.add_argument(
        "-d",
        "--debug",
        action="store_true",
        help="Enable debug logging (shows all errors and debug messages)",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="Enable verbose logging (shows info, warnings, and errors)",
    )
    parser.add_argument(
        "--log-file",
        type=str,
        help="Write logs to a file (e.g., --log-file todozi.log)",
    )
    parser.add_argument(
        "--error-file",
        type=str,
        help="Write errors to a file (e.g., --error-file errors.log)",
    )

    args = parser.parse_args()

    # Configure logging
    log_level = logging.WARNING  # Default: only warnings and errors
    if args.debug:
        log_level = logging.DEBUG
    elif args.verbose:
        log_level = logging.INFO

    # Set up logging format
    log_format = "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
    date_format = "%Y-%m-%d %H:%M:%S"

    # Configure handlers
    handlers = []

    # Console handler (always show errors and warnings)
    console_handler = logging.StreamHandler(sys.stderr)
    console_handler.setLevel(
        logging.WARNING
    )  # Always show warnings and errors on console
    console_handler.setFormatter(logging.Formatter(log_format, date_format))
    handlers.append(console_handler)

    # File handler if specified
    if args.log_file:
        file_handler = logging.FileHandler(args.log_file)
        file_handler.setLevel(log_level)
        file_handler.setFormatter(logging.Formatter(log_format, date_format))
        handlers.append(file_handler)

    # Error file handler if specified
    if args.error_file:
        error_handler = logging.FileHandler(args.error_file)
        error_handler.setLevel(logging.ERROR)  # Only errors
        error_handler.setFormatter(logging.Formatter(log_format, date_format))
        handlers.append(error_handler)

    # Configure root logger
    logging.basicConfig(
        level=log_level,
        format=log_format,
        datefmt=date_format,
        handlers=handlers,
        force=True,  # Override any existing configuration
    )

    # Also configure the module logger
    logger.setLevel(log_level)

    # Print startup info
    if args.debug or args.verbose:
        print(f"Logging level: {logging.getLevelName(log_level)}", file=sys.stderr)
        if args.log_file:
            print(f"Logging to: {args.log_file}", file=sys.stderr)
        if args.error_file:
            print(f"Errors logging to: {args.error_file}", file=sys.stderr)

    try:
        app = TodoziTUI()
        app.run()
    except KeyboardInterrupt:
        print("\nInterrupted by user", file=sys.stderr)
    except Exception as e:
        logger.exception("Fatal error in TUI")
        print(f"\nFatal error: {e}", file=sys.stderr)
        if args.debug:
            import traceback

    try:
        app = TodoziTUI()
        app.run()
    except KeyboardInterrupt:
        print("\nInterrupted by user", file=sys.stderr)
    except Exception as e:
        logger.exception("Fatal error in TUI")
        print(f"\nFatal error: {e}", file=sys.stderr)
        if args.debug:
            import traceback

            traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
