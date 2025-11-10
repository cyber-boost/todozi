from __future__ import annotations

import hashlib
import secrets
import uuid
from dataclasses import dataclass, field
from datetime import datetime, timezone as dt_timezone
from enum import Enum
from typing import Any, Dict, Generic, List, Optional, Type, TypeVar, Union

# Result type mirroring Rust's Result<T, E>
T = TypeVar("T")
E = TypeVar("E", bound=Exception)


class Ok(Generic[T]):
    def __init__(self, value: T):
        self.value: T = value

    def is_ok(self) -> bool:
        return True

    def is_err(self) -> bool:
        return False

    def unwrap(self) -> T:
        return self.value


class Err(Generic[E]):
    def __init__(self, error: E):
        self.error: E = error

    def is_ok(self) -> bool:
        return False

    def is_err(self) -> bool:
        return True

    def unwrap(self) -> Any:
        raise self.error


Result = Union[Ok[T], Err[E]]


# Error type
class TodoziError(Exception):
    def __init__(
        self,
        kind: str,
        message: str,
        *,
        priority: Optional[str] = None,
        status: Optional[str] = None,
        progress: Optional[int] = None,
    ):
        super().__init__(message)
        self.kind = kind
        self.message = message
        self.priority = priority
        self.status = status
        self.progress = progress

    @staticmethod
    def invalid_priority(priority: str) -> "TodoziError":
        return TodoziError("InvalidPriority", f"Invalid priority: {priority}", priority=priority)

    @staticmethod
    def invalid_status(status: str) -> "TodoziError":
        return TodoziError("InvalidStatus", f"Invalid status: {status}", status=status)

    @staticmethod
    def invalid_progress(progress: int) -> "TodoziError":
        return TodoziError("InvalidProgress", f"Progress must be in 0..100, got {progress}", progress=progress)

    @staticmethod
    def validation_error(message: str) -> "TodoziError":
        return TodoziError("ValidationError", message)


# Utilities
def utc_now() -> datetime:
    return datetime.now(dt_timezone.utc)


def short_uuid() -> str:
    return str(uuid.uuid4()).split("-")[0]


# Enum base with parsing and aliases
class LowercaseEnumMixin:
    # Override in subclasses: ALIASES: Dict[str, 'LowercaseEnum'] = {...}
    ALIASES: Dict[str, Any] = {}

    def __str__(self) -> str:
        return self.value

    @classmethod
    def from_str(cls: Type[Any], s: str) -> Result[Any, TodoziError]:
        lower = s.lower()
        if lower in cls.ALIASES:
            return Ok(cls.ALIASES[lower])
        if lower in {m.value for m in cls}:
            return Ok(cls(lower))
        # Default error message for enums that require specific error
        if cls.__name__ == "Priority":
            return Err(TodoziError.invalid_priority(s))
        if cls.__name__ == "Status":
            return Err(TodoziError.invalid_status(s))
        return Err(TodoziError.validation_error(f"Invalid {cls.__name__}: {s}"))

    @classmethod
    def from_str_mapped(cls: Type[Any], s: str, mapping: Dict[str, Any]) -> Result[Any, TodoziError]:
        lower = s.lower()
        if lower in mapping:
            return Ok(mapping[lower])
        # Try enum direct
        try:
            return Ok(cls(lower))
        except ValueError:
            # Use the enum name for default error kind mapping
            if cls.__name__ == "Priority":
                return Err(TodoziError.invalid_priority(s))
            if cls.__name__ == "Status":
                return Err(TodoziError.invalid_status(s))
            return Err(TodoziError.validation_error(f"Invalid {cls.__name__}: {s}"))

    @classmethod
    def from_str_exhaustive(cls: Type[Any], s: str) -> Result[Any, TodoziError]:
        lower = s.lower()
        if lower in cls.ALIASES:
            return Ok(cls.ALIASES[lower])
        if lower in {m.value for m in cls}:
            return Ok(cls(lower))
        return Err(TodoziError.validation_error(f"Invalid {cls.__name__}: {s}"))


# Enums
class Priority(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"
    URGENT = "urgent"

    # No aliases, but keep mapping API uniform
    @classmethod
    def from_str(cls, s: str) -> Result["Priority", TodoziError]:
        return super().from_str_mapped(s, cls.ALIASES if cls.ALIASES else {m.value: m for m in cls})


class Status(LowercaseEnumMixin, str, Enum):
    TODO = "todo"
    PENDING = "pending"
    IN_PROGRESS = "in_progress"
    BLOCKED = "blocked"
    REVIEW = "review"
    DONE = "done"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    DEFERRED = "deferred"

    @classmethod
    def _get_aliases(cls):
        return {
            "pending": "todo",  # alias to TODO
            "in-progress": "in_progress",  # alias
            "completed": "done",  # alias
            "canceled": "cancelled",  # alias
        }

    @classmethod
    def from_str(cls, s: str) -> Result["Status", TodoziError]:
        # Use exhaustive mapping for statuses to honor all aliases
        mapping = {m.value: m for m in cls}
        aliases = cls._get_aliases()
        for alias, target_value in aliases.items():
            # Find the enum member with the target value
            for member in cls:
                if member.value == target_value:
                    mapping[alias] = member
                    break
        return super().from_str_mapped(s, mapping)


class MemoryImportance(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class MemoryTerm(LowercaseEnumMixin, str, Enum):
    SHORT = "short"
    LONG = "long"


class CoreEmotion(LowercaseEnumMixin, str, Enum):
    HAPPY = "happy"
    SAD = "sad"
    ANGRY = "angry"
    FEARFUL = "fearful"
    SURPRISED = "surprised"
    DISGUSTED = "disgusted"
    EXCITED = "excited"
    ANXIOUS = "anxious"
    CONFIDENT = "confident"
    FRUSTRATED = "frustrated"
    MOTIVATED = "motivated"
    OVERWHELMED = "overwhelmed"
    CURIOUS = "curious"
    SATISFIED = "satisfied"
    DISAPPOINTED = "disappointed"
    GRATEFUL = "grateful"
    PROUD = "proud"
    ASHAMED = "ashamed"
    HOPEFUL = "hopeful"
    RESIGNED = "resigned"


class MemoryType(LowercaseEnumMixin, str, Enum):
    STANDARD = "standard"
    SECRET = "secret"
    HUMAN = "human"
    SHORT = "short"
    LONG = "long"
    EMOTIONAL = "emotional"

    @classmethod
    def from_str(cls, s: str) -> Result["MemoryType", TodoziError]:
        lower = s.lower()
        direct = {m.value: m for m in cls}
        if lower in direct:
            return Ok(direct[lower])
        # Emotional if parses as CoreEmotion
        emo = CoreEmotion.from_str(lower)
        if isinstance(emo, Ok):
            # Return EMOTIONAL but keep the raw string in display
            return Ok(cls.EMOTIONAL)
        return Err(TodoziError.validation_error(f"Invalid memory type: {s}"))

    def __str__(self) -> str:
        if self == MemoryType.EMOTIONAL:
            # Caller should set a __post_init__ to store original string if needed.
            return "emotional"
        return self.value


class ShareLevel(LowercaseEnumMixin, str, Enum):
    PRIVATE = "private"
    TEAM = "team"
    PUBLIC = "public"


class IdeaImportance(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    BREAKTHROUGH = "breakthrough"


class ItemStatus(LowercaseEnumMixin, str, Enum):
    ACTIVE = "active"
    ARCHIVED = "archived"
    DELETED = "deleted"


class ErrorSeverity(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class ErrorCategory(LowercaseEnumMixin, str, Enum):
    NETWORK = "network"
    DATABASE = "database"
    AUTHENTICATION = "authentication"
    AUTHORIZATION = "authorization"
    VALIDATION = "validation"
    PERFORMANCE = "performance"
    SECURITY = "security"
    INTEGRATION = "integration"
    CONFIGURATION = "configuration"
    RUNTIME = "runtime"
    COMPILATION = "compilation"
    DEPENDENCY = "dependency"
    USERERROR = "user_error"
    SYSTEMERROR = "system_error"

    ALIASES = {
        "usererror": USERERROR,
        "user_error": USERERROR,
        "systemerror": SYSTEMERROR,
        "system_error": SYSTEMERROR,
    }


class TrainingDataType(LowercaseEnumMixin, str, Enum):
    INSTRUCTION = "instruction"
    COMPLETION = "completion"
    CONVERSATION = "conversation"
    CODE = "code"
    ANALYSIS = "analysis"
    PLANNING = "planning"
    REVIEW = "review"
    DOCUMENTATION = "documentation"
    EXAMPLE = "example"
    TEST = "test"
    VALIDATION = "validation"


class ProjectStatus(LowercaseEnumMixin, str, Enum):
    ACTIVE = "active"
    ARCHIVED = "archived"
    COMPLETED = "completed"


class AgentStatus(LowercaseEnumMixin, str, Enum):
    ACTIVE = "active"
    INACTIVE = "inactive"
    BUSY = "busy"
    AVAILABLE = "available"


class AssignmentStatus(LowercaseEnumMixin, str, Enum):
    ASSIGNED = "assigned"
    ACCEPTED = "accepted"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    REJECTED = "rejected"


class QueueStatus(LowercaseEnumMixin, str, Enum):
    BACKLOG = "backlog"
    ACTIVE = "active"
    COMPLETE = "complete"


class SummaryPriority(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class ReminderPriority(LowercaseEnumMixin, str, Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class ReminderStatus(LowercaseEnumMixin, str, Enum):
    PENDING = "pending"
    ACTIVE = "active"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    OVERDUE = "overdue"

    ALIASES = {
        "cancelled": CANCELLED,
        "canceled": CANCELLED,
    }


ACCEPTED = "accepted"
ACTIVE = "active"
ALIASES = "aliases"
ANALYSIS = "analysis"
ANGRY = "angry"
ANXIOUS = "anxious"
ARCHIVED = "archived"
ASHAMED = "ashamed"
ASSIGNED = "assigned"
AUTHENTICATION = "authentication"
AUTHORIZATION = "authorization"
AVAILABLE = "available"
BACKLOG = "backlog"
BLOCKED = "blocked"
BREAKTHROUGH = "breakthrough"
BUSY = "busy"
CANCELLED = "cancelled"
CODE = "code"
COMPILATION = "compilation"
COMPLETE = "complete"
COMPLETED = "completed"
COMPLETION = "completion"
CONFIDENT = "confident"
CONFIGURATION = "configuration"
CONVERSATION = "conversation"
CRITICAL = "critical"
CURIOUS = "curious"
DATABASE = "database"
DEFERRED = "deferred"
DELETED = "deleted"
DEPENDENCY = "dependency"
DISAPPOINTED = "disappointed"
DISGUSTED = "disgusted"
DOCUMENTATION = "documentation"
DONE = "done"
EMOTIONAL = "emotional"
EXAMPLE = "example"
EXCITED = "excited"
FEARFUL = "fearful"
FRUSTRATED = "frustrated"
GRATEFUL = "grateful"
HAPPY = "happy"
HIGH = "high"
HOPEFUL = "hopeful"
HUMAN = "human"
INACTIVE = "inactive"
IN_PROGRESS = "in_progress"
INSTRUCTION = "instruction"
INTEGRATION = "integration"
LOW = "low"
LONG = "long"
MEDIUM = "medium"
MOTIVATED = "motivated"
NETWORK = "network"
OVERDUE = "overdue"
OVERWHELMED = "overwhelmed"
PENDING = "pending"
PERFORMANCE = "performance"
PLANNING = "planning"
PRIVATE = "private"
PROUD = "proud"
PUBLIC = "public"
REJECTED = "rejected"
RESIGNED = "resigned"
REVIEW = "review"
RUNTIME = "runtime"
SAD = "sad"
SATISFIED = "satisfied"
SECRET = "secret"
SECURITY = "security"
SHORT = "short"
STANDARD = "standard"
SURPRISED = "surprised"
SYSTEMERROR = "system_error"
TEAM = "team"
TEST = "test"
TODO = "todo"
URGENT = "urgent"
USERERROR = "user_error"
VALIDATION = "validation"


# Assignee
@dataclass(frozen=True)
class Assignee:
    """
    Mimics Rust Assignee:
    - Ai
    - Human
    - Collaborative
    - Agent(name)
    """

    kind: str  # one of "ai", "human", "collaborative", "agent"
    name: Optional[str] = None  # set when kind == "agent"

    def __str__(self) -> str:
        if self.kind == "agent":
            return f"agent:{self.name or ''}"
        return self.kind

    @staticmethod
    def default() -> "Assignee":
        return Assignee("human")

    @staticmethod
    def from_str(s: str) -> Result["Assignee", TodoziError]:
        lower = s.lower()
        if lower == "ai":
            return Ok(Assignee("ai"))
        if lower == "human":
            return Ok(Assignee("human"))
        if lower == "collaborative":
            return Ok(Assignee("collaborative"))
        if lower.startswith("agent:"):
            name = lower.split("agent:", 1)[1]
            return Ok(Assignee("agent", name))
        # If plain string provided, treat as agent name
        return Ok(Assignee("agent", s))


# Pydantic models (if available); fallback to dataclasses otherwise
try:
    from pydantic import BaseModel, field_validator, model_validator
    from pydantic.config import ConfigDict

    BaseModel.model_config = ConfigDict(extra="forbid", use_enum_values=True, validate_assignment=True)
    BaseT = BaseModel
except Exception:  # pragma: no cover
    # Minimal fallback if pydantic is not available
    BaseT = object


# Data models
class Task(BaseT):
    id: str
    user_id: str
    action: str
    time: str
    priority: Priority
    parent_project: str
    status: Status
    assignee: Optional[Assignee] = None
    tags: List[str] = []
    dependencies: List[str] = []
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @field_validator("progress")
    @classmethod
    def validate_progress(cls, v: Optional[int]) -> Optional[int]:
        if v is not None and not (0 <= v <= 100):
            raise ValueError("Progress must be between 0 and 100")
        return v

    @field_validator("assignee")
    @classmethod
    def validate_assignee(cls, v: Any) -> Any:
        # Allow Pydantic to coerce from string (e.g., "ai", "agent:foo")
        if isinstance(v, str):
            res = Assignee.from_str(v)
            if isinstance(res, Err):
                raise ValueError(str(res.error))
            return res.value  # type: ignore[attr-defined]
        return v

    @field_validator("priority")
    @classmethod
    def validate_priority(cls, v: Any) -> Any:
        if isinstance(v, str):
            res = Priority.from_str(v)
            if isinstance(res, Err):
                raise ValueError(str(res.error))
            return res.value  # type: ignore[attr-defined]
        return v

    @field_validator("status")
    @classmethod
    def validate_status(cls, v: Any) -> Any:
        if isinstance(v, str):
            res = Status.from_str(v)
            if isinstance(res, Err):
                raise ValueError(str(res.error))
            return res.value  # type: ignore[attr-defined]
        return v

    @staticmethod
    def new(
        user_id: str,
        action: str,
        time: str,
        priority: Priority,
        parent_project: str,
        status: Status,
    ) -> "Task":
        now = utc_now()
        return Task(
            id=f"task_{short_uuid()}",
            user_id=user_id,
            action=action,
            time=time,
            priority=priority,
            parent_project=parent_project,
            status=status,
            assignee=None,
            tags=[],
            dependencies=[],
            context_notes=None,
            progress=None,
            embedding_vector=None,
            created_at=now,
            updated_at=now,
        )

    @staticmethod
    def new_full(
        user_id: str,
        action: str,
        time: str,
        priority: Priority,
        parent_project: str,
        status: Status,
        assignee: Optional[Assignee],
        tags: List[str],
        dependencies: List[str],
        context_notes: Optional[str],
        progress: Optional[int],
    ) -> Result["Task", TodoziError]:
        if progress is not None and not (0 <= progress <= 100):
            return Err(TodoziError.invalid_progress(progress))
        now = utc_now()
        return Ok(
            Task(
                id=f"task_{short_uuid()}",
                user_id=user_id,
                action=action,
                time=time,
                priority=priority,
                parent_project=parent_project,
                status=status,
                assignee=assignee,
                tags=tags,
                dependencies=dependencies,
                context_notes=context_notes,
                progress=progress,
                embedding_vector=None,
                created_at=now,
                updated_at=now,
            )
        )

    def update(self, updates: "TaskUpdate") -> Result[None, TodoziError]:
        if updates.action is not None:
            self.action = updates.action  # type: ignore[attr-defined]
        if updates.time is not None:
            self.time = updates.time  # type: ignore[attr-defined]
        if updates.priority is not None:
            self.priority = updates.priority  # type: ignore[attr-defined]
        if updates.parent_project is not None:
            self.parent_project = updates.parent_project  # type: ignore[attr-defined]
        if updates.status is not None:
            self.status = updates.status  # type: ignore[attr-defined]
        if updates.assignee is not None:
            self.assignee = updates.assignee  # type: ignore[attr-defined]
        if updates.tags is not None:
            self.tags = updates.tags  # type: ignore[attr-defined]
        if updates.dependencies is not None:
            self.dependencies = updates.dependencies  # type: ignore[attr-defined]
        if updates.context_notes is not None:
            self.context_notes = updates.context_notes  # type: ignore[attr-defined]
        if updates.progress is not None:
            if not (0 <= updates.progress <= 100):  # type: ignore[attr-defined]
                return Err(TodoziError.invalid_progress(updates.progress))  # type: ignore[attr-defined]
            self.progress = updates.progress  # type: ignore[attr-defined]
        if updates.embedding_vector is not None:
            self.embedding_vector = updates.embedding_vector  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]
        return Ok(None)

    def complete(self) -> None:
        self.status = Status.DONE  # type: ignore[attr-defined]
        self.progress = 100  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def is_completed(self) -> bool:
        return self.status == Status.DONE  # type: ignore[attr-defined]

    def is_active(self) -> bool:
        return self.status not in (Status.DONE, Status.CANCELLED)  # type: ignore[attr-defined]


class TaskUpdate(BaseT):
    action: Optional[str] = None
    time: Optional[str] = None
    priority: Optional[Priority] = None
    parent_project: Optional[str] = None
    status: Optional[Status] = None
    assignee: Optional[Union[str, Assignee]] = None
    tags: Optional[List[str]] = None
    dependencies: Optional[List[str]] = None
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None

    @field_validator("assignee")
    @classmethod
    def validate_assignee(cls, v: Any) -> Any:
        if v is None or isinstance(v, Assignee):
            return v
        if isinstance(v, str):
            res = Assignee.from_str(v)
            if isinstance(res, Err):
                raise ValueError(str(res.error))
            return res.value  # type: ignore[attr-defined]
        return v

    def with_action(self, action: str) -> "TaskUpdate":
        self.action = action  # type: ignore[attr-defined]
        return self

    def with_time(self, time: str) -> "TaskUpdate":
        self.time = time  # type: ignore[attr-defined]
        return self

    def with_priority(self, priority: Priority) -> "TaskUpdate":
        self.priority = priority  # type: ignore[attr-defined]
        return self

    def with_parent_project(self, parent_project: str) -> "TaskUpdate":
        self.parent_project = parent_project  # type: ignore[attr-defined]
        return self

    def with_status(self, status: Status) -> "TaskUpdate":
        self.status = status  # type: ignore[attr-defined]
        return self

    def with_assignee(self, assignee: Union[Assignee, str]) -> "TaskUpdate":
        if isinstance(assignee, str):
            res = Assignee.from_str(assignee)
            if isinstance(res, Err):
                raise ValueError(str(res.error))
            self.assignee = res.value  # type: ignore[attr-defined]
        else:
            self.assignee = assignee  # type: ignore[attr-defined]
        return self

    def with_tags(self, tags: List[str]) -> "TaskUpdate":
        self.tags = tags  # type: ignore[attr-defined]
        return self

    def with_dependencies(self, dependencies: List[str]) -> "TaskUpdate":
        self.dependencies = dependencies  # type: ignore[attr-defined]
        return self

    def with_context_notes(self, context_notes: str) -> "TaskUpdate":
        self.context_notes = context_notes  # type: ignore[attr-defined]
        return self

    def with_progress(self, progress: int) -> "TaskUpdate":
        if not (0 <= progress <= 100):
            raise ValueError("Progress must be between 0 and 100")
        self.progress = progress  # type: ignore[attr-defined]
        return self


class TaskFilters(BaseT):
    project: Optional[str] = None
    status: Optional[Status] = None
    priority: Optional[Priority] = None
    assignee: Optional[Assignee] = None
    tags: Optional[List[str]] = None
    search: Optional[str] = None


class Project(BaseT):
    name: str
    description: Optional[str]
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()
    status: ProjectStatus = ProjectStatus.ACTIVE
    tasks: List[str] = []

    @staticmethod
    def new(name: str, description: Optional[str] = None) -> "Project":
        now = utc_now()
        return Project(
            name=name,
            description=description,
            created_at=now,
            updated_at=now,
            status=ProjectStatus.ACTIVE,
        )

    def add_task(self, task_id: str) -> None:
        if task_id not in self.tasks:
            self.tasks.append(task_id)  # type: ignore[attr-defined]
            self.updated_at = utc_now()  # type: ignore[attr-defined]

    def remove_task(self, task_id: str) -> None:
        if task_id in self.tasks:
            self.tasks.remove(task_id)  # type: ignore[attr-defined]
            self.updated_at = utc_now()  # type: ignore[attr-defined]

    def archive(self) -> None:
        self.status = ProjectStatus.ARCHIVED  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def complete(self) -> None:
        self.status = ProjectStatus.COMPLETED  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]


class Config(BaseT):
    registration: Optional["RegistrationInfo"] = None
    version: str = "1.2.0"
    default_project: str = "general"
    auto_backup: bool = True
    backup_interval: str = "daily"
    ai_enabled: bool = True
    default_assignee: Optional[Assignee] = Assignee("collaborative")
    date_format: str = "%Y-%m-%d %H:%M:%S"
    timezone: str = "UTC"


class RegistrationInfo(BaseT):
    user_name: str
    user_email: str
    api_key: str
    user_id: Optional[str] = None
    fingerprint: Optional[str] = None
    registered_at: datetime = utc_now()
    server_url: str

    @staticmethod
    def new(user_name: str, user_email: str, api_key: str, server_url: str) -> "RegistrationInfo":
        return RegistrationInfo(
            user_name=user_name,
            user_email=user_email,
            api_key=api_key,
            user_id=None,
            fingerprint=None,
            registered_at=utc_now(),
            server_url=server_url,
        )

    @staticmethod
    def new_with_hashes(server_url: str) -> "RegistrationInfo":
        user_id = f"user_{short_uuid()}"
        email_hash = f"hash_{short_uuid()}@example.com"
        return RegistrationInfo(
            user_name=user_id,
            user_email=email_hash,
            api_key="",
            user_id=None,
            fingerprint=None,
            registered_at=utc_now(),
            server_url=server_url,
        )


class TaskCollection(BaseT):
    version: str = "1.2.0"
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()
    tasks: Dict[str, Task] = {}

    def add_task(self, task: Task) -> None:
        self.tasks[task.id] = task  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def get_task(self, id: str) -> Optional[Task]:
        return self.tasks.get(id)  # type: ignore[attr-defined]

    def get_task_mut(self, id: str) -> Optional[Task]:
        # Python's mutability; return the actual object
        return self.tasks.get(id)  # type: ignore[attr-defined]

    def remove_task(self, id: str) -> Optional[Task]:
        task = self.tasks.pop(id, None)  # type: ignore[attr-defined]
        if task is not None:
            self.updated_at = utc_now()  # type: ignore[attr-defined]
        return task

    def get_all_tasks(self) -> List[Task]:
        return list(self.tasks.values())  # type: ignore[attr-defined]

    def get_filtered_tasks(self, filters: TaskFilters) -> List[Task]:
        def match(task: Task) -> bool:  # type: ignore[valid-type]
            if filters.project is not None and task.parent_project != filters.project:  # type: ignore[attr-defined]
                return False
            if filters.status is not None and task.status != filters.status:  # type: ignore[attr-defined]
                return False
            if filters.priority is not None and task.priority != filters.priority:  # type: ignore[attr-defined]
                return False
            if filters.assignee is not None and task.assignee != filters.assignee:  # type: ignore[attr-defined]
                return False
            if filters.tags is not None:
                if not any(tag in task.tags for tag in filters.tags):  # type: ignore[attr-defined]
                    return False
            if filters.search is not None:
                if filters.search.lower() not in task.action.lower():  # type: ignore[attr-defined]
                    return False
            return True

        return [t for t in self.tasks.values() if match(t)]  # type: ignore[attr-defined]


class Memory(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    user_id: str
    project_id: Optional[str]
    status: ItemStatus
    moment: str
    meaning: str
    reason: str
    importance: MemoryImportance
    term: MemoryTerm
    memory_type: MemoryType
    tags: List[str] = []
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()


class ModelConfig(BaseT):
    provider: str
    name: str
    temperature: float
    max_tokens: int


class AgentTool(BaseT):
    name: str
    enabled: bool
    config: Optional[Dict[str, Any]]


class AgentBehaviors(BaseT):
    auto_format_code: bool = True
    include_examples: bool = True
    explain_complexity: bool = True
    suggest_tests: bool = True


class RateLimit(BaseT):
    requests_per_minute: Optional[int] = None
    tokens_per_hour: Optional[int] = None


class AgentConstraints(BaseT):
    max_response_length: Optional[int] = 10_000
    timeout_seconds: Optional[int] = 300
    rate_limit: Optional[RateLimit] = None


class AgentMetadata(BaseT):
    author: str = "system"
    tags: List[str] = field(default_factory=lambda: ["ai", "assistant"])
    category: str = "general"
    status: AgentStatus = AgentStatus.AVAILABLE


class Agent(BaseT):
    id: str
    name: str
    description: str
    version: str = "1.0.0"
    model: ModelConfig = field(default_factory=lambda: ModelConfig(provider="anthropic", name="claude-3-opus-20240229", temperature=0.2, max_tokens=4096))
    system_prompt: str = ""
    prompt_template: Optional[str] = None
    capabilities: List[str] = field(default_factory=list)
    specializations: List[str] = field(default_factory=list)
    tools: List[AgentTool] = field(default_factory=list)
    behaviors: AgentBehaviors = field(default_factory=AgentBehaviors)
    constraints: AgentConstraints = field(default_factory=lambda: AgentConstraints(max_response_length=10_000, timeout_seconds=300, rate_limit=RateLimit(requests_per_minute=10, tokens_per_hour=100_000)))
    metadata: AgentMetadata = field(default_factory=AgentMetadata)
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @staticmethod
    def new(id: str, name: str, description: str) -> "Agent":
        now = utc_now()
        system_prompt = f"You are {id}, an AI assistant specialized in {description}."
        return Agent(
            id=id,
            name=name,
            description=description,
            system_prompt=system_prompt,
            created_at=now,
            updated_at=now,
        )

    @staticmethod
    def create_coder() -> "Agent":
        agent = Agent.new("coder", "Coder", "Software development and programming specialist")
        agent.system_prompt = (
            "You are an expert software developer with deep knowledge of multiple programming "
            "languages and best practices. Your role is to:\n"
            "- Write clean, efficient, and well-documented code\n"
            "- Follow language-specific conventions and idioms\n"
            "- Consider security, performance, and maintainability\n"
            "- Provide clear explanations of your code and decisions\n"
            "- Suggest improvements and alternatives when appropriate"
        )
        agent.prompt_template = (
            "Task: {task}\nLanguage: {language}\nContext: {context}\n\n"
            "Requirements:\n{requirements}\n\nPlease provide a solution with explanations."
        )
        agent.capabilities = [
            "code_development",
            "code_review",
            "debugging",
            "refactoring",
            "testing",
            "documentation",
            "architecture_design",
        ]
        agent.specializations = [
            "rust",
            "python",
            "javascript",
            "typescript",
            "go",
            "sql",
            "docker",
        ]
        agent.tools = [
            AgentTool(name="code_executor", enabled=True, config=None),
            AgentTool(name="linter", enabled=True, config=None),
            AgentTool(name="test_runner", enabled=True, config=None),
        ]
        agent.metadata.tags = ["development", "programming", "technical"]  # type: ignore[attr-defined]
        agent.metadata.category = "technical"  # type: ignore[attr-defined]
        return agent

    def has_capability(self, capability: str) -> bool:
        return capability in self.capabilities  # type: ignore[attr-defined]

    def has_specialization(self, specialization: str) -> bool:
        return specialization in self.specializations  # type: ignore[attr-defined]

    def has_tool(self, tool_name: str) -> bool:
        return any(t.name == tool_name and t.enabled for t in self.tools)  # type: ignore[attr-defined]

    def get_enabled_tools(self) -> List[AgentTool]:
        return [t for t in self.tools if t.enabled]  # type: ignore[attr-defined]

    def set_status(self, status: AgentStatus) -> None:
        self.metadata.status = status  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def is_available(self) -> bool:
        return self.metadata.status == AgentStatus.AVAILABLE  # type: ignore[attr-defined]

    def get_formatted_prompt(self, variables: Dict[str, str]) -> str:
        prompt = self.system_prompt  # type: ignore[attr-defined]
        if self.prompt_template is not None:  # type: ignore[attr-defined]
            formatted = self.prompt_template  # type: ignore[attr-defined]
            for k, v in variables.items():
                placeholder = f"{{{k}}}"
                formatted = formatted.replace(placeholder, v)
            prompt = f"{prompt}\n\n{formatted}"
        return prompt


class Idea(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    idea: str
    project_id: Optional[str]
    status: ItemStatus
    share: ShareLevel
    importance: IdeaImportance
    tags: List[str] = []
    context: Optional[str] = None
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()


class AgentAssignment(BaseT):
    agent_id: str
    task_id: str
    project_id: str
    assigned_at: datetime = utc_now()
    status: AssignmentStatus


class Error(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    title: str
    description: str
    severity: ErrorSeverity = ErrorSeverity.MEDIUM
    category: ErrorCategory = ErrorCategory.RUNTIME
    source: str
    context: Optional[str] = None
    tags: List[str] = []
    resolved: bool = False
    resolution: Optional[str] = None
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()
    resolved_at: Optional[datetime] = None

    @staticmethod
    def new(title: str, description: str, source: str) -> "Error":
        return Error(
            title=title,
            description=description,
            source=source,
        )


class TrainingData(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    data_type: TrainingDataType = TrainingDataType.INSTRUCTION
    prompt: str
    completion: str
    context: Optional[str] = None
    tags: List[str] = []
    quality_score: Optional[float] = None
    source: str
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @staticmethod
    def new(data_type: str, prompt: str, completion: str, source: str) -> "TrainingData":
        parsed = TrainingDataType.from_str(data_type)
        dtype = parsed.value if isinstance(parsed, Ok) else TrainingDataType.INSTRUCTION  # type: ignore[attr-defined]
        return TrainingData(
            data_type=dtype,
            prompt=prompt,
            completion=completion,
            source=source,
        )


class Feeling(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    emotion: str
    intensity: int
    description: str
    context: str
    tags: List[str] = []
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()


class Tag(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str
    description: Optional[str] = None
    color: Optional[str] = None
    category: Optional[str] = None
    usage_count: int = 0
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()


class QueueItem(BaseT):
    id: str = field(default_factory=lambda: f"queue_{short_uuid()}")
    task_name: str
    task_description: str
    priority: Priority = Priority.MEDIUM
    project_id: Optional[str] = None
    status: QueueStatus = QueueStatus.BACKLOG
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @staticmethod
    def new(task_name: str, task_description: str, priority: Priority, project_id: Optional[str] = None) -> "QueueItem":
        return QueueItem(
            task_name=task_name,
            task_description=task_description,
            priority=priority,
            project_id=project_id,
        )

    def start(self) -> None:
        self.status = QueueStatus.ACTIVE  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def complete(self) -> None:
        self.status = QueueStatus.COMPLETE  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def is_backlog(self) -> bool:
        return self.status == QueueStatus.BACKLOG  # type: ignore[attr-defined]

    def is_active(self) -> bool:
        return self.status == QueueStatus.ACTIVE  # type: ignore[attr-defined]

    def is_complete(self) -> bool:
        return self.status == QueueStatus.COMPLETE  # type: ignore[attr-defined]


class QueueSession(BaseT):
    id: str = field(default_factory=lambda: f"session_{short_uuid()}")
    queue_item_id: str
    start_time: datetime = utc_now()
    end_time: Optional[datetime] = None
    duration_seconds: Optional[int] = None
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @staticmethod
    def new(queue_item_id: str) -> "QueueSession":
        return QueueSession(queue_item_id=queue_item_id)

    def end(self) -> None:
        end_time = utc_now()
        self.end_time = end_time  # type: ignore[attr-defined]
        self.duration_seconds = int((end_time - self.start_time).total_seconds())  # type: ignore[attr-defined]
        self.updated_at = end_time  # type: ignore[attr-defined]

    def is_active(self) -> bool:
        return self.end_time is None  # type: ignore[attr-defined]

    def get_current_duration(self) -> int:
        if self.is_active():  # type: ignore[attr-defined]
            return int((utc_now() - self.start_time).total_seconds())  # type: ignore[attr-defined]
        return self.duration_seconds or 0  # type: ignore[attr-defined]


class QueueCollection(BaseT):
    version: str = "1.0.0"
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()
    items: Dict[str, QueueItem] = {}
    sessions: Dict[str, QueueSession] = {}

    def add_item(self, item: QueueItem) -> None:
        self.items[item.id] = item  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def get_item(self, id: str) -> Optional[QueueItem]:
        return self.items.get(id)  # type: ignore[attr-defined]

    def get_item_mut(self, id: str) -> Optional[QueueItem]:
        return self.items.get(id)  # type: ignore[attr-defined]

    def remove_item(self, id: str) -> Optional[QueueItem]:
        item = self.items.pop(id, None)  # type: ignore[attr-defined]
        if item is not None:
            self.updated_at = utc_now()  # type: ignore[attr-defined]
        return item

    def get_all_items(self) -> List[QueueItem]:
        return list(self.items.values())  # type: ignore[attr-defined]

    def get_items_by_status(self, status: QueueStatus) -> List[QueueItem]:
        return [i for i in self.items.values() if i.status == status]  # type: ignore[attr-defined]

    def get_backlog_items(self) -> List[QueueItem]:
        return self.get_items_by_status(QueueStatus.BACKLOG)  # type: ignore[attr-defined]

    def get_active_items(self) -> List[QueueItem]:
        return self.get_items_by_status(QueueStatus.ACTIVE)  # type: ignore[attr-defined]

    def get_complete_items(self) -> List[QueueItem]:
        return self.get_items_by_status(QueueStatus.COMPLETE)  # type: ignore[attr-defined]

    def start_session(self, queue_item_id: str) -> Result[str, TodoziError]:
        item = self.items.get(queue_item_id)  # type: ignore[attr-defined]
        if item is None:
            return Err(TodoziError.validation_error("Queue item not found"))
        if not item.is_backlog():  # type: ignore[attr-defined]
            return Err(TodoziError.validation_error("Item is not in backlog status"))
        session = QueueSession.new(queue_item_id)
        self.sessions[session.id] = session  # type: ignore[attr-defined]
        item.start()  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]
        return Ok(session.id)

    def end_session(self, session_id: str) -> Result[None, TodoziError]:
        session = self.sessions.get(session_id)  # type: ignore[attr-defined]
        if session is None:
            return Err(TodoziError.validation_error("Session not found"))
        if not session.is_active():  # type: ignore[attr-defined]
            return Err(TodoziError.validation_error("Session is already ended"))
        session.end()  # type: ignore[attr-defined]
        item = self.items.get(session.queue_item_id)  # type: ignore[attr-defined]
        if item is not None:
            item.complete()  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]
        return Ok(None)

    def get_active_sessions(self) -> List[QueueSession]:
        return [s for s in self.sessions.values() if s.is_active()]  # type: ignore[attr-defined]

    def get_session(self, id: str) -> Optional[QueueSession]:
        return self.sessions.get(id)  # type: ignore[attr-defined]


@dataclass
class ApiKey:
    user_id: str
    public_key: str
    private_key: str
    active: bool = True
    created_at: datetime = field(default_factory=utc_now)
    updated_at: datetime = field(default_factory=utc_now)

    @staticmethod
    def new() -> "ApiKey":
        now = utc_now()
        user_id = f"user_{short_uuid()}"
        time_str = str(int(now.timestamp()))
        rand_str1 = str(secrets.randbits(64))
        rand_str2 = str(secrets.randbits(64))
        input_str = f"{time_str}{rand_str1}{rand_str2}"
        public_key = hashlib.sha256(input_str.encode("utf-8")).hexdigest()
        private_key = hashlib.sha512(public_key.encode("utf-8")).hexdigest()
        return ApiKey(
            user_id=user_id,
            public_key=public_key,
            private_key=private_key,
            active=True,
            created_at=now,
            updated_at=now,
        )

    @staticmethod
    def with_user_id(user_id: str) -> "ApiKey":
        now = utc_now()
        time_str = str(int(now.timestamp()))
        rand_str1 = str(secrets.randbits(64))
        rand_str2 = str(secrets.randbits(64))
        input_str = f"{time_str}{rand_str1}{rand_str2}"
        public_key = hashlib.sha256(input_str.encode("utf-8")).hexdigest()
        private_key = hashlib.sha512(public_key.encode("utf-8")).hexdigest()
        return ApiKey(
            user_id=user_id,
            public_key=public_key,
            private_key=private_key,
            active=True,
            created_at=now,
            updated_at=now,
        )

    def deactivate(self) -> None:
        self.active = False
        self.updated_at = utc_now()

    def activate(self) -> None:
        self.active = True
        self.updated_at = utc_now()

    def is_active(self) -> bool:
        return self.active

    def matches(self, public_key: str, private_key: Optional[str] = None) -> bool:
        if not self.is_active():
            return False
        if self.public_key != public_key:
            return False
        if private_key is not None:
            return self.private_key == private_key
        return True

    def is_admin(self, public_key: str, private_key: str) -> bool:
        return self.matches(public_key, private_key)


@dataclass
class ApiKeyCollection:
    version: str = "1.0.0"
    created_at: datetime = field(default_factory=utc_now)
    updated_at: datetime = field(default_factory=utc_now)
    keys: Dict[str, ApiKey] = field(default_factory=dict)

    def add_key(self, key: ApiKey) -> None:
        self.keys[key.user_id] = key
        self.updated_at = utc_now()

    def get_key(self, user_id: str) -> Optional[ApiKey]:
        return self.keys.get(user_id)

    def get_key_by_public(self, public_key: str) -> Optional[ApiKey]:
        for k in self.keys.values():
            if k.public_key == public_key:
                return k
        return None

    def get_all_keys(self) -> List[ApiKey]:
        return list(self.keys.values())

    def get_active_keys(self) -> List[ApiKey]:
        return [k for k in self.keys.values() if k.is_active()]

    def remove_key(self, user_id: str) -> Optional[ApiKey]:
        key = self.keys.pop(user_id, None)
        if key is not None:
            self.updated_at = utc_now()
        return key

    def deactivate_key(self, user_id: str) -> bool:
        key = self.keys.get(user_id)
        if key is None:
            return False
        key.deactivate()
        self.updated_at = utc_now()
        return True

    def activate_key(self, user_id: str) -> bool:
        key = self.keys.get(user_id)
        if key is None:
            return False
        key.activate()
        self.updated_at = utc_now()
        return True


class Summary(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    content: str
    context: Optional[str] = None
    priority: SummaryPriority = SummaryPriority.MEDIUM
    tags: List[str] = []
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @staticmethod
    def new(content: str, priority: SummaryPriority) -> "Summary":
        return Summary(content=content, priority=priority)

    def with_context(self, context: str) -> "Summary":
        self.context = context  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]
        return self

    def with_tags(self, tags: List[str]) -> "Summary":
        self.tags = tags  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]
        return self


class Reminder(BaseT):
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    content: str
    remind_at: datetime = utc_now()
    priority: ReminderPriority = ReminderPriority.MEDIUM
    status: ReminderStatus = ReminderStatus.PENDING
    tags: List[str] = []
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()

    @staticmethod
    def new(content: str, remind_at: datetime, priority: ReminderPriority) -> "Reminder":
        return Reminder(content=content, remind_at=remind_at, priority=priority)

    def with_tags(self, tags: List[str]) -> "Reminder":
        self.tags = tags  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]
        return self

    def is_overdue(self) -> bool:
        return self.remind_at < utc_now() and self.status == ReminderStatus.PENDING  # type: ignore[attr-defined]

    def mark_completed(self) -> None:
        self.status = ReminderStatus.COMPLETED  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def mark_cancelled(self) -> None:
        self.status = ReminderStatus.CANCELLED  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def activate(self) -> None:
        self.status = ReminderStatus.ACTIVE  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]


class MLEngine:
    def __init__(self, model_name: str):
        self.model_name = model_name
        self.temperature = 0.7
        self.max_tokens = 4096

    def with_temperature(self, temperature: float) -> "MLEngine":
        self.temperature = temperature
        return self

    def with_max_tokens(self, max_tokens: int) -> "MLEngine":
        self.max_tokens = max_tokens
        return self

    async def predict_relevance(self, features: List[float]) -> float:
        return 0.5

    async def craft_embedding(self, features: List[float]) -> List[float]:
        return [0.1] * 384

    async def strike_tags(self, features: List[float]) -> List[float]:
        return [0.1] * 10

    async def strike_cluster(self, embedding: List[float]) -> int:
        return 0

    async def analyze_code_quality(self, features: List[float]) -> float:
        return 0.7


class ProjectStats(BaseT):
    project_name: str
    total_tasks: int
    active_tasks: int
    completed_tasks: int
    archived_tasks: int
    deleted_tasks: int


class SemanticSearchResult(BaseT):
    task: Task
    similarity_score: float
    matched_content: str


class ProjectMigrationStats(BaseT):
    project_name: str
    initial_tasks: int = 0
    migrated_tasks: int = 0
    final_tasks: int = 0


class MigrationReport(BaseT):
    tasks_found: int = 0
    tasks_migrated: int = 0
    projects_migrated: int = 0
    project_stats: List[ProjectMigrationStats] = []
    errors: List[str] = []


class ProjectTaskContainer(BaseT):
    project_name: str
    project_hash: str
    created_at: datetime = utc_now()
    updated_at: datetime = utc_now()
    active_tasks: Dict[str, Task] = {}
    completed_tasks: Dict[str, Task] = {}
    archived_tasks: Dict[str, Task] = {}
    deleted_tasks: Dict[str, Task] = {}

    @staticmethod
    def new(project_name: str) -> "ProjectTaskContainer":
        return ProjectTaskContainer(
            project_name=project_name,
            project_hash=hash_project_name(project_name),
        )

    def add_task(self, task: Task) -> None:
        task_id = task.id  # type: ignore[attr-defined]
        if task.status in (Status.TODO, Status.PENDING, Status.IN_PROGRESS, Status.BLOCKED, Status.REVIEW):  # type: ignore[attr-defined]
            self.active_tasks[task_id] = task  # type: ignore[attr-defined]
        elif task.status in (Status.DONE, Status.COMPLETED):  # type: ignore[attr-defined]
            self.completed_tasks[task_id] = task  # type: ignore[attr-defined]
        elif task.status in (Status.CANCELLED, Status.DEFERRED):  # type: ignore[attr-defined]
            self.archived_tasks[task_id] = task  # type: ignore[attr-defined]
        else:
            # Default to archived for unknown statuses
            self.archived_tasks[task_id] = task  # type: ignore[attr-defined]
        self.updated_at = utc_now()  # type: ignore[attr-defined]

    def get_task(self, task_id: str) -> Optional[Task]:
        return (self.active_tasks.get(task_id) or self.completed_tasks.get(task_id) or  # type: ignore[attr-defined]
                self.archived_tasks.get(task_id) or self.deleted_tasks.get(task_id))  # type: ignore[attr-defined]

    def get_task_mut(self, task_id: str) -> Optional[Task]:
        if task_id in self.active_tasks:  # type: ignore[attr-defined]
            return self.active_tasks[task_id]  # type: ignore[attr-defined]
        if task_id in self.completed_tasks:  # type: ignore[attr-defined]
            return self.completed_tasks[task_id]  # type: ignore[attr-defined]
        if task_id in self.archived_tasks:  # type: ignore[attr-defined]
            return self.archived_tasks[task_id]  # type: ignore[attr-defined]
        return self.deleted_tasks.get(task_id)  # type: ignore[attr-defined]

    def remove_task(self, task_id: str) -> Optional[Task]:
        return (self.active_tasks.pop(task_id, None) or  # type: ignore[attr-defined]
                self.completed_tasks.pop(task_id, None) or  # type: ignore[attr-defined]
                self.archived_tasks.pop(task_id, None) or  # type: ignore[attr-defined]
                self.deleted_tasks.pop(task_id, None))  # type: ignore[attr-defined]

    def update_task_status(self, task_id: str, new_status: Status) -> Optional[None]:
        task = self.remove_task(task_id)
        if task is None:
            return None
        task.status = new_status  # type: ignore[attr-defined]
        task.updated_at = utc_now()  # type: ignore[attr-defined]
        self.add_task(task)
        return None

    def get_all_tasks(self) -> List[Task]:
        all_tasks: List[Task] = []
        all_tasks.extend(self.active_tasks.values())  # type: ignore[attr-defined]
        all_tasks.extend(self.completed_tasks.values())  # type: ignore[attr-defined]
        all_tasks.extend(self.archived_tasks.values())  # type: ignore[attr-defined]
        all_tasks.extend(self.deleted_tasks.values())  # type: ignore[attr-defined]
        return all_tasks

    def get_filtered_tasks(self, filters: TaskFilters) -> List[Task]:
        def match(task: Task) -> bool:  # type: ignore[valid-type]
            if filters.project is not None and task.parent_project != filters.project:  # type: ignore[attr-defined]
                return False
            # Rust behavior: match status if provided; status field used as-is
            if filters.status is not None and task.status != filters.status:  # type: ignore[attr-defined]
                return False
            if filters.priority is not None and task.priority != filters.priority:  # type: ignore[attr-defined]
                return False
            if filters.assignee is not None and task.assignee != filters.assignee:  # type: ignore[attr-defined]
                return False
            if filters.tags is not None:
                if not any(tag in task.tags for tag in filters.tags):  # type: ignore[attr-defined]
                    return False
            if filters.search is not None:
                search_lower = filters.search.lower()  # type: ignore[attr-defined]
                if search_lower not in task.action.lower():  # type: ignore[attr-defined]
                    ctx = task.context_notes or ""  # type: ignore[attr-defined]
                    if search_lower not in ctx.lower():  # type: ignore[attr-defined]
                        return False
            return True

        return [t for t in self.get_all_tasks() if match(t)]  # type: ignore[attr-defined]


def hash_project_name(project_name: str) -> str:
    return hashlib.md5(project_name.encode("utf-8")).hexdigest()
