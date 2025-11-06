"""Error types and error management for Todozi."""

from typing import Optional, Dict
from datetime import datetime
import uuid as uuid_lib


class TodoziError(Exception):
    """Base exception for all Todozi errors."""
    pass


class TaskNotFound(TodoziError):
    """Task not found error."""
    def __init__(self, task_id: str):
        self.id = task_id
        super().__init__(f"Task not found: {task_id}")


class ProjectNotFound(TodoziError):
    """Project not found error."""
    def __init__(self, name: str):
        self.name = name
        super().__init__(f"Project not found: {name}")


class FeelingNotFound(TodoziError):
    """Feeling not found error."""
    def __init__(self, feeling_id: str):
        self.id = feeling_id
        super().__init__(f"Feeling not found: {feeling_id}")


class InvalidPriority(TodoziError):
    """Invalid priority error."""
    def __init__(self, priority: str):
        self.priority = priority
        super().__init__(
            f"Invalid priority: {priority}. Must be one of: low, medium, high, critical, urgent"
        )


class InvalidStatus(TodoziError):
    """Invalid status error."""
    def __init__(self, status: str):
        self.status = status
        super().__init__(
            f"Invalid status: {status}. Must be one of: todo, in_progress, blocked, review, done, cancelled, deferred"
        )


class InvalidAssignee(TodoziError):
    """Invalid assignee error."""
    def __init__(self, assignee: str):
        self.assignee = assignee
        super().__init__(
            f"Invalid assignee: {assignee}. Must be one of: ai, human, collaborative"
        )


class InvalidProgress(TodoziError):
    """Invalid progress error."""
    def __init__(self, progress: int):
        self.progress = progress
        super().__init__(
            f"Invalid progress: {progress}. Must be between 0 and 100"
        )


class ValidationError(TodoziError):
    """Validation error."""
    def __init__(self, message: str):
        self.message = message
        super().__init__(f"Validation error: {message}")


class StorageError(TodoziError):
    """Storage error."""
    def __init__(self, message: str):
        self.message = message
        super().__init__(f"Storage error: {message}")


class ConfigError(TodoziError):
    """Configuration error."""
    def __init__(self, message: str):
        self.message = message
        super().__init__(f"Configuration error: {message}")


class EmbeddingError(TodoziError):
    """Embedding error."""
    def __init__(self, message: str):
        self.message = message
        super().__init__(f"Embedding error: {message}")


class ApiError(TodoziError):
    """API error."""
    def __init__(self, message: str):
        self.message = message
        super().__init__(f"API error: {message}")


class CandleError(TodoziError):
    """Candle (ML framework) error."""
    def __init__(self, message: str):
        self.message = message
        super().__init__(f"Candle error: {message}")


class ErrorManager:
    """Manages error instances in the system."""

    def __init__(self):
        self.errors: Dict[str, 'Error'] = {}

    async def create_error(self, error: 'Error') -> str:
        """Create a new error entry."""
        error.id = str(uuid_lib.uuid4())
        error.created_at = datetime.utcnow()
        error.updated_at = datetime.utcnow()
        self.errors[error.id] = error
        return error.id

    def get_unresolved_errors(self) -> list:
        """Get all unresolved errors."""
        return [e for e in self.errors.values() if not e.resolved]

    async def resolve_error(self, error_id: str, resolution: str):
        """Resolve an error with a resolution message."""
        if error_id not in self.errors:
            raise ValidationError(f"Error {error_id} not found")

        error = self.errors[error_id]
        error.resolved = True
        error.resolution = resolution
        error.resolved_at = datetime.utcnow()
        error.updated_at = datetime.utcnow()


def parse_error_format(error_text: str) -> 'Error':
    """Parse error from XML-like format."""
    from .models import Error, ErrorSeverity, ErrorCategory

    start_tag = "<error>"
    end_tag = "</error>"

    start = error_text.find(start_tag)
    if start == -1:
        raise ValidationError("Missing <error> start tag")

    end = error_text.find(end_tag)
    if end == -1:
        raise ValidationError("Missing </error> end tag")

    content = error_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 5:
        raise ValidationError(
            "Invalid error format: need at least 5 parts (title; description; severity; category; source)"
        )

    tags = []
    if len(parts) > 6 and parts[6]:
        tags = [s.strip() for s in parts[6].split(',')]

    context = None
    if len(parts) > 5 and parts[5]:
        context = parts[5]

    return Error(
        id=str(uuid_lib.uuid4()),
        title=parts[0],
        description=parts[1],
        severity=ErrorSeverity[parts[2].upper()],
        category=ErrorCategory[parts[3].upper()],
        source=parts[4],
        context=context,
        tags=tags,
        resolved=False,
        resolution=None,
        created_at=datetime.utcnow(),
        updated_at=datetime.utcnow(),
        resolved_at=None
    )
