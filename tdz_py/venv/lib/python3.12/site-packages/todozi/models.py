import uuid
import hashlib
import random
from datetime import datetime, timezone
from enum import Enum
from typing import Optional, List, Dict, Any, Union
from dataclasses import dataclass, field
import json

class TodoziError(Exception):
    pass

class Priority(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"
    URGENT = "urgent"

    @classmethod
    def from_str(cls, s: str) -> 'Priority':
        s = s.lower()
        if s == "low":
            return cls.LOW
        elif s == "medium":
            return cls.MEDIUM
        elif s == "high":
            return cls.HIGH
        elif s == "critical":
            return cls.CRITICAL
        elif s == "urgent":
            return cls.URGENT
        else:
            raise TodoziError(f"Invalid priority: {s}")

    def __str__(self) -> str:
        return self.value

class Status(Enum):
    TODO = "todo"
    PENDING = "todo"
    IN_PROGRESS = "in_progress"
    BLOCKED = "blocked"
    REVIEW = "review"
    DONE = "done"
    COMPLETED = "done"
    CANCELLED = "cancelled"
    DEFERRED = "deferred"

    @classmethod
    def from_str(cls, s: str) -> 'Status':
        s = s.lower()
        if s in ["todo", "pending"]:
            return cls.TODO
        elif s in ["in_progress", "in-progress"]:
            return cls.IN_PROGRESS
        elif s == "blocked":
            return cls.BLOCKED
        elif s == "review":
            return cls.REVIEW
        elif s in ["done", "completed"]:
            return cls.DONE
        elif s in ["cancelled", "canceled"]:
            return cls.CANCELLED
        elif s == "deferred":
            return cls.DEFERRED
        else:
            raise TodoziError(f"Invalid status: {s}")

    def __str__(self) -> str:
        if self in [Status.TODO, Status.PENDING]:
            return "todo"
        elif self == Status.IN_PROGRESS:
            return "in_progress"
        elif self == Status.BLOCKED:
            return "blocked"
        elif self == Status.REVIEW:
            return "review"
        elif self in [Status.DONE, Status.COMPLETED]:
            return "done"
        elif self == Status.CANCELLED:
            return "cancelled"
        elif self == Status.DEFERRED:
            return "deferred"

class Assignee:
    def __init__(self, assignee_type: str, name: Optional[str] = None):
        self.assignee_type = assignee_type
        self.name = name

    @classmethod
    def from_str(cls, s: str) -> 'Assignee':
        s = s.lower()
        if s == "ai":
            return cls("ai")
        elif s == "human":
            return cls("human")
        elif s == "collaborative":
            return cls("collaborative")
        elif s.startswith("agent:"):
            return cls("agent", s[6:])
        else:
            return cls("agent", s)

    def __str__(self) -> str:
        if self.assignee_type == "agent" and self.name:
            return f"agent:{self.name}"
        return self.assignee_type

    def __eq__(self, other) -> bool:
        if not isinstance(other, Assignee):
            return False
        return self.assignee_type == other.assignee_type and self.name == other.name

    def __hash__(self) -> int:
        return hash((self.assignee_type, self.name))

class MemoryImportance(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

    @classmethod
    def from_str(cls, s: str) -> 'MemoryImportance':
        s = s.lower()
        if s == "low":
            return cls.LOW
        elif s == "medium":
            return cls.MEDIUM
        elif s == "high":
            return cls.HIGH
        elif s == "critical":
            return cls.CRITICAL
        else:
            raise TodoziError(f"Invalid memory importance: {s}")

    def __str__(self) -> str:
        return self.value

class MemoryTerm(Enum):
    SHORT = "short"
    LONG = "long"

    @classmethod
    def from_str(cls, s: str) -> 'MemoryTerm':
        s = s.lower()
        if s == "short":
            return cls.SHORT
        elif s == "long":
            return cls.LONG
        else:
            raise TodoziError(f"Invalid memory term: {s}")

    def __str__(self) -> str:
        return self.value

class MemoryType:
    def __init__(self, memory_type: str, emotion: Optional[str] = None):
        self.memory_type = memory_type
        self.emotion = emotion

    @classmethod
    def from_str(cls, s: str) -> 'MemoryType':
        s = s.lower()
        if s == "standard":
            return cls("standard")
        elif s == "secret":
            return cls("secret")
        elif s == "human":
            return cls("human")
        elif s == "short":
            return cls("short")
        elif s == "long":
            return cls("long")
        else:
            try:
                CoreEmotion.from_str(s)
                return cls("emotional", s)
            except TodoziError:
                raise TodoziError(f"Invalid memory type: {s}")

    def __str__(self) -> str:
        if self.memory_type == "emotional" and self.emotion:
            return self.emotion
        return self.memory_type

    def __eq__(self, other) -> bool:
        if not isinstance(other, MemoryType):
            return False
        return self.memory_type == other.memory_type and self.emotion == other.emotion

    def __hash__(self) -> int:
        return hash((self.memory_type, self.emotion))

class CoreEmotion(Enum):
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

    @classmethod
    def from_str(cls, s: str) -> 'CoreEmotion':
        s = s.lower()
        if s == "happy":
            return cls.HAPPY
        elif s == "sad":
            return cls.SAD
        elif s == "angry":
            return cls.ANGRY
        elif s == "fearful":
            return cls.FEARFUL
        elif s == "surprised":
            return cls.SURPRISED
        elif s == "disgusted":
            return cls.DISGUSTED
        elif s == "excited":
            return cls.EXCITED
        elif s == "anxious":
            return cls.ANXIOUS
        elif s == "confident":
            return cls.CONFIDENT
        elif s == "frustrated":
            return cls.FRUSTRATED
        elif s == "motivated":
            return cls.MOTIVATED
        elif s == "overwhelmed":
            return cls.OVERWHELMED
        elif s == "curious":
            return cls.CURIOUS
        elif s == "satisfied":
            return cls.SATISFIED
        elif s == "disappointed":
            return cls.DISAPPOINTED
        elif s == "grateful":
            return cls.GRATEFUL
        elif s == "proud":
            return cls.PROUD
        elif s == "ashamed":
            return cls.ASHAMED
        elif s == "hopeful":
            return cls.HOPEFUL
        elif s == "resigned":
            return cls.RESIGNED
        else:
            raise TodoziError(f"Invalid core emotion: {s}")

    def __str__(self) -> str:
        return self.value

class ShareLevel(Enum):
    PRIVATE = "private"
    TEAM = "team"
    PUBLIC = "public"

    @classmethod
    def from_str(cls, s: str) -> 'ShareLevel':
        s = s.lower()
        if s == "private":
            return cls.PRIVATE
        elif s == "team":
            return cls.TEAM
        elif s == "public":
            return cls.PUBLIC
        else:
            raise TodoziError(f"Invalid share level: {s}")

class IdeaImportance(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    BREAKTHROUGH = "breakthrough"

    @classmethod
    def from_str(cls, s: str) -> 'IdeaImportance':
        s = s.lower()
        if s == "low":
            return cls.LOW
        elif s == "medium":
            return cls.MEDIUM
        elif s == "high":
            return cls.HIGH
        elif s == "breakthrough":
            return cls.BREAKTHROUGH
        else:
            raise TodoziError(f"Invalid idea importance: {s}")

    def __str__(self) -> str:
        return self.value

class ItemStatus(Enum):
    ACTIVE = "active"
    ARCHIVED = "archived"
    DELETED = "deleted"

class ErrorSeverity(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

    @classmethod
    def from_str(cls, s: str) -> 'ErrorSeverity':
        s = s.lower()
        if s == "low":
            return cls.LOW
        elif s == "medium":
            return cls.MEDIUM
        elif s == "high":
            return cls.HIGH
        elif s == "critical":
            return cls.CRITICAL
        else:
            raise TodoziError(f"Invalid error severity: {s}")

    def __str__(self) -> str:
        return self.value

class ErrorCategory(Enum):
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
    USER_ERROR = "user_error"
    SYSTEM_ERROR = "system_error"

    @classmethod
    def from_str(cls, s: str) -> 'ErrorCategory':
        s = s.lower()
        if s == "network":
            return cls.NETWORK
        elif s == "database":
            return cls.DATABASE
        elif s == "authentication":
            return cls.AUTHENTICATION
        elif s == "authorization":
            return cls.AUTHORIZATION
        elif s == "validation":
            return cls.VALIDATION
        elif s == "performance":
            return cls.PERFORMANCE
        elif s == "security":
            return cls.SECURITY
        elif s == "integration":
            return cls.INTEGRATION
        elif s == "configuration":
            return cls.CONFIGURATION
        elif s == "runtime":
            return cls.RUNTIME
        elif s == "compilation":
            return cls.COMPILATION
        elif s == "dependency":
            return cls.DEPENDENCY
        elif s in ["usererror", "user_error"]:
            return cls.USER_ERROR
        elif s in ["systemerror", "system_error"]:
            return cls.SYSTEM_ERROR
        else:
            raise TodoziError(f"Invalid error category: {s}")

    def __str__(self) -> str:
        return self.value

class TrainingDataType(Enum):
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

    @classmethod
    def from_str(cls, s: str) -> 'TrainingDataType':
        s = s.lower()
        if s == "instruction":
            return cls.INSTRUCTION
        elif s == "completion":
            return cls.COMPLETION
        elif s == "conversation":
            return cls.CONVERSATION
        elif s == "code":
            return cls.CODE
        elif s == "analysis":
            return cls.ANALYSIS
        elif s == "planning":
            return cls.PLANNING
        elif s == "review":
            return cls.REVIEW
        elif s == "documentation":
            return cls.DOCUMENTATION
        elif s == "example":
            return cls.EXAMPLE
        elif s == "test":
            return cls.TEST
        elif s == "validation":
            return cls.VALIDATION
        else:
            raise TodoziError(f"Invalid training data type: {s}")

    def __str__(self) -> str:
        return self.value

@dataclass
class Task:
    id: str
    user_id: str
    action: str
    time: str
    priority: Priority
    parent_project: str
    status: Status
    assignee: Optional[Assignee]
    tags: List[str]
    dependencies: List[str]
    context_notes: Optional[str]
    progress: Optional[int]
    embedding_vector: Optional[List[float]]
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls, user_id: str, action: str, time: str, priority: Priority, parent_project: str, status: Status) -> 'Task':
        now = datetime.now(timezone.utc)
        task_id = f"task_{str(uuid.uuid4())[:8]}"
        return cls(
            id=task_id,
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
            updated_at=now
        )

    @classmethod
    def new_full(cls, user_id: str, action: str, time: str, priority: Priority, parent_project: str, status: Status,
                 assignee: Optional[Assignee], tags: List[str], dependencies: List[str],
                 context_notes: Optional[str], progress: Optional[int]) -> 'Task':
        if progress is not None and progress > 100:
            raise TodoziError(f"Invalid progress: {progress}")
        now = datetime.now(timezone.utc)
        task_id = f"task_{str(uuid.uuid4())[:8]}"
        return cls(
            id=task_id,
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
            updated_at=now
        )

    def update(self, updates: 'TaskUpdate') -> None:
        if updates.action is not None:
            self.action = updates.action
        if updates.time is not None:
            self.time = updates.time
        if updates.priority is not None:
            self.priority = updates.priority
        if updates.parent_project is not None:
            self.parent_project = updates.parent_project
        if updates.status is not None:
            self.status = updates.status
        if updates.assignee is not None:
            self.assignee = updates.assignee
        if updates.tags is not None:
            self.tags = updates.tags
        if updates.dependencies is not None:
            self.dependencies = updates.dependencies
        if updates.context_notes is not None:
            self.context_notes = updates.context_notes
        if updates.progress is not None:
            if updates.progress > 100:
                raise TodoziError(f"Invalid progress: {updates.progress}")
            self.progress = updates.progress
        if updates.embedding_vector is not None:
            self.embedding_vector = updates.embedding_vector
        self.updated_at = datetime.now(timezone.utc)

    def complete(self) -> None:
        self.status = Status.DONE
        self.progress = 100
        self.updated_at = datetime.now(timezone.utc)

    def is_completed(self) -> bool:
        return self.status == Status.DONE

    def is_active(self) -> bool:
        return self.status not in [Status.DONE, Status.CANCELLED]

@dataclass
class TaskUpdate:
    action: Optional[str] = None
    time: Optional[str] = None
    priority: Optional[Priority] = None
    parent_project: Optional[str] = None
    status: Optional[Status] = None
    assignee: Optional[Assignee] = None
    tags: Optional[List[str]] = None
    dependencies: Optional[List[str]] = None
    context_notes: Optional[str] = None
    progress: Optional[int] = None
    embedding_vector: Optional[List[float]] = None

    @classmethod
    def new(cls) -> 'TaskUpdate':
        return cls()

    def with_action(self, action: str) -> 'TaskUpdate':
        self.action = action
        return self

    def with_time(self, time: str) -> 'TaskUpdate':
        self.time = time
        return self

    def with_priority(self, priority: Priority) -> 'TaskUpdate':
        self.priority = priority
        return self

    def with_parent_project(self, parent_project: str) -> 'TaskUpdate':
        self.parent_project = parent_project
        return self

    def with_status(self, status: Status) -> 'TaskUpdate':
        self.status = status
        return self

    def with_assignee(self, assignee: Assignee) -> 'TaskUpdate':
        self.assignee = assignee
        return self

    def with_tags(self, tags: List[str]) -> 'TaskUpdate':
        self.tags = tags
        return self

    def with_dependencies(self, dependencies: List[str]) -> 'TaskUpdate':
        self.dependencies = dependencies
        return self

    def with_context_notes(self, context_notes: str) -> 'TaskUpdate':
        self.context_notes = context_notes
        return self

    def with_progress(self, progress: int) -> 'TaskUpdate':
        self.progress = progress
        return self

@dataclass
class TaskFilters:
    project: Optional[str] = None
    status: Optional[Status] = None
    priority: Optional[Priority] = None
    assignee: Optional[Assignee] = None
    tags: Optional[List[str]] = None
    search: Optional[str] = None

@dataclass
class Project:
    name: str
    description: Optional[str]
    created_at: datetime
    updated_at: datetime
    status: 'ProjectStatus'
    tasks: List[str]

    @classmethod
    def new(cls, name: str, description: Optional[str]) -> 'Project':
        now = datetime.now(timezone.utc)
        return cls(
            name=name,
            description=description,
            created_at=now,
            updated_at=now,
            status=ProjectStatus.ACTIVE,
            tasks=[]
        )

    def add_task(self, task_id: str) -> None:
        if task_id not in self.tasks:
            self.tasks.append(task_id)
            self.updated_at = datetime.now(timezone.utc)

    def remove_task(self, task_id: str) -> None:
        self.tasks = [id for id in self.tasks if id != task_id]
        self.updated_at = datetime.now(timezone.utc)

    def archive(self) -> None:
        self.status = ProjectStatus.ARCHIVED
        self.updated_at = datetime.now(timezone.utc)

    def complete(self) -> None:
        self.status = ProjectStatus.COMPLETED
        self.updated_at = datetime.now(timezone.utc)

class ProjectStatus(Enum):
    ACTIVE = "active"
    ARCHIVED = "archived"
    COMPLETED = "completed"

    @classmethod
    def from_str(cls, s: str) -> 'ProjectStatus':
        s = s.lower()
        if s == "active":
            return cls.ACTIVE
        elif s == "archived":
            return cls.ARCHIVED
        elif s == "completed":
            return cls.COMPLETED
        else:
            raise TodoziError(f"Invalid project status: {s}")

    def __str__(self) -> str:
        return self.value

@dataclass
class RegistrationInfo:
    user_name: str
    user_email: str
    api_key: str
    user_id: Optional[str]
    fingerprint: Optional[str]
    registered_at: datetime
    server_url: str

    @classmethod
    def new(cls, user_name: str, user_email: str, api_key: str, server_url: str) -> 'RegistrationInfo':
        return cls(
            user_name=user_name,
            user_email=user_email,
            api_key=api_key,
            user_id=None,
            fingerprint=None,
            registered_at=datetime.now(timezone.utc),
            server_url=server_url
        )

    @classmethod
    def new_with_hashes(cls, server_url: str) -> 'RegistrationInfo':
        user_id = f"user_{str(uuid.uuid4())[:8]}"
        email_hash = f"hash_{str(uuid.uuid4())[:8]}@example.com"
        return cls(
            user_name=user_id,
            user_email=email_hash,
            api_key="",
            user_id=None,
            fingerprint=None,
            registered_at=datetime.now(timezone.utc),
            server_url=server_url
        )

@dataclass
class Config:
    version: str = "1.2.0"
    default_project: str = "general"
    auto_backup: bool = True
    backup_interval: str = "daily"
    ai_enabled: bool = True
    default_assignee: Optional[Assignee] = field(default_factory=lambda: Assignee("collaborative"))
    date_format: str = "%Y-%m-%d %H:%M:%S"
    timezone: str = "UTC"
    registration: Optional[RegistrationInfo] = None

@dataclass
class TaskCollection:
    version: str = "1.2.0"
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    tasks: Dict[str, Task] = field(default_factory=dict)

    @classmethod
    def new(cls) -> 'TaskCollection':
        now = datetime.now(timezone.utc)
        return cls(
            version="1.2.0",
            created_at=now,
            updated_at=now,
            tasks={}
        )

    def add_task(self, task: Task) -> None:
        self.tasks[task.id] = task
        self.updated_at = datetime.now(timezone.utc)

    def get_task(self, id: str) -> Optional[Task]:
        return self.tasks.get(id)

    def get_task_mut(self, id: str) -> Optional[Task]:
        return self.tasks.get(id)

    def remove_task(self, id: str) -> Optional[Task]:
        task = self.tasks.pop(id, None)
        if task is not None:
            self.updated_at = datetime.now(timezone.utc)
        return task

    def get_all_tasks(self) -> List[Task]:
        return list(self.tasks.values())

    def get_filtered_tasks(self, filters: TaskFilters) -> List[Task]:
        result = []
        for task in self.tasks.values():
            if filters.project is not None and task.parent_project != filters.project:
                continue
            if filters.status is not None and task.status != filters.status:
                continue
            if filters.priority is not None and task.priority != filters.priority:
                continue
            if filters.assignee is not None and task.assignee != filters.assignee:
                continue
            if filters.tags is not None and not any(tag in task.tags for tag in filters.tags):
                continue
            if filters.search is not None and filters.search.lower() not in task.action.lower():
                continue
            result.append(task)
        return result

@dataclass
class Memory:
    id: str
    user_id: str
    project_id: Optional[str]
    status: ItemStatus
    moment: str
    meaning: str
    reason: str
    importance: MemoryImportance
    term: MemoryTerm
    memory_type: MemoryType
    tags: List[str]
    created_at: datetime
    updated_at: datetime

@dataclass
class ModelConfig:
    provider: str
    name: str
    temperature: float
    max_tokens: int

@dataclass
class AgentTool:
    name: str
    enabled: bool
    config: Optional[Any]

@dataclass
class AgentBehaviors:
    auto_format_code: bool
    include_examples: bool
    explain_complexity: bool
    suggest_tests: bool

@dataclass
class AgentConstraints:
    max_response_length: Optional[int]
    timeout_seconds: Optional[int]
    rate_limit: Optional['RateLimit']

@dataclass
class RateLimit:
    requests_per_minute: Optional[int]
    tokens_per_hour: Optional[int]

@dataclass
class AgentMetadata:
    author: str
    tags: List[str]
    category: str
    status: 'AgentStatus'

class AgentStatus(Enum):
    ACTIVE = "active"
    INACTIVE = "inactive"
    BUSY = "busy"
    AVAILABLE = "available"

    def __str__(self) -> str:
        return self.value

@dataclass
class Agent:
    id: str
    name: str
    description: str
    version: str
    model: ModelConfig
    system_prompt: str
    prompt_template: Optional[str]
    capabilities: List[str]
    specializations: List[str]
    tools: List[AgentTool]
    behaviors: AgentBehaviors
    constraints: AgentConstraints
    metadata: AgentMetadata
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls, id: str, name: str, description: str) -> 'Agent':
        now = datetime.now(timezone.utc)
        return cls(
            id=id,
            name=name,
            description=description,
            version="1.0.0",
            model=ModelConfig(
                provider="anthropic",
                name="claude-3-opus-20240229",
                temperature=0.2,
                max_tokens=4096
            ),
            system_prompt=f"You are {id}, an AI assistant specialized in {description}.",
            prompt_template=None,
            capabilities=[],
            specializations=[],
            tools=[],
            behaviors=AgentBehaviors(
                auto_format_code=True,
                include_examples=True,
                explain_complexity=True,
                suggest_tests=True
            ),
            constraints=AgentConstraints(
                max_response_length=10000,
                timeout_seconds=300,
                rate_limit=RateLimit(
                    requests_per_minute=10,
                    tokens_per_hour=100000
                )
            ),
            metadata=AgentMetadata(
                author="system",
                tags=["ai", "assistant"],
                category="general",
                status=AgentStatus.AVAILABLE
            ),
            created_at=now,
            updated_at=now
        )

    @classmethod
    def create_coder(cls) -> 'Agent':
        agent = cls.new("coder", "Coder", "Software development and programming specialist")
        agent.system_prompt = "You are an expert software developer with deep knowledge of multiple programming languages and best practices. Your role is to:\n- Write clean, efficient, and well-documented code\n- Follow language-specific conventions and idioms\n- Consider security, performance, and maintainability\n- Provide clear explanations of your code and decisions\n- Suggest improvements and alternatives when appropriate"
        agent.prompt_template = "Task: {task}\nLanguage: {language}\nContext: {context}\n\nRequirements:\n{requirements}\n\nPlease provide a solution with explanations."
        agent.capabilities = [
            "code_development", "code_review", "debugging",
            "refactoring", "testing", "documentation", "architecture_design"
        ]
        agent.specializations = [
            "rust", "python", "javascript",
            "typescript", "go", "sql", "docker"
        ]
        agent.tools = [
            AgentTool(name="code_executor", enabled=True, config=None),
            AgentTool(name="linter", enabled=True, config=None),
            AgentTool(name="test_runner", enabled=True, config=None)
        ]
        agent.metadata.tags = ["development", "programming", "technical"]
        agent.metadata.category = "technical"
        return agent

    def has_capability(self, capability: str) -> bool:
        return capability in self.capabilities

    def has_specialization(self, specialization: str) -> bool:
        return specialization in self.specializations

    def has_tool(self, tool_name: str) -> bool:
        return any(tool.name == tool_name and tool.enabled for tool in self.tools)

    def get_enabled_tools(self) -> List[AgentTool]:
        return [tool for tool in self.tools if tool.enabled]

    def set_status(self, status: AgentStatus) -> None:
        self.metadata.status = status
        self.updated_at = datetime.now(timezone.utc)

    def is_available(self) -> bool:
        return self.metadata.status == AgentStatus.AVAILABLE

    def get_formatted_prompt(self, variables: Dict[str, str]) -> str:
        prompt = self.system_prompt
        if self.prompt_template:
            formatted_template = self.prompt_template
            for key, value in variables.items():
                placeholder = f"{{{key}}}"
                formatted_template = formatted_template.replace(placeholder, value)
            prompt += "\n\n" + formatted_template
        return prompt

@dataclass
class Idea:
    id: str
    idea: str
    project_id: Optional[str]
    status: ItemStatus
    share: ShareLevel
    importance: IdeaImportance
    tags: List[str]
    context: Optional[str]
    created_at: datetime
    updated_at: datetime

class AssignmentStatus(Enum):
    ASSIGNED = "assigned"
    ACCEPTED = "accepted"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    REJECTED = "rejected"

    def __str__(self) -> str:
        if self == AssignmentStatus.IN_PROGRESS:
            return "in_progress"
        return self.value

@dataclass
class AgentAssignment:
    agent_id: str
    task_id: str
    project_id: str
    assigned_at: datetime
    status: AssignmentStatus

@dataclass
class Error:
    id: str
    title: str
    description: str
    severity: ErrorSeverity
    category: ErrorCategory
    source: str
    context: Optional[str]
    tags: List[str]
    resolved: bool
    resolution: Optional[str]
    created_at: datetime
    updated_at: datetime
    resolved_at: Optional[datetime]

    @classmethod
    def new(cls, title: str, description: str, source: str) -> 'Error':
        now = datetime.now(timezone.utc)
        return cls(
            id=str(uuid.uuid4()),
            title=title,
            description=description,
            severity=ErrorSeverity.MEDIUM,
            category=ErrorCategory.RUNTIME,
            source=source,
            context=None,
            tags=[],
            resolved=False,
            resolution=None,
            created_at=now,
            updated_at=now,
            resolved_at=None
        )

@dataclass
class TrainingData:
    id: str
    data_type: TrainingDataType
    prompt: str
    completion: str
    context: Optional[str]
    tags: List[str]
    quality_score: Optional[float]
    source: str
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls, data_type: str, prompt: str, completion: str, source: str) -> 'TrainingData':
        now = datetime.now(timezone.utc)
        try:
            data_type_enum = TrainingDataType.from_str(data_type)
        except TodoziError:
            data_type_enum = TrainingDataType.INSTRUCTION
        return cls(
            id=str(uuid.uuid4()),
            data_type=data_type_enum,
            prompt=prompt,
            completion=completion,
            context=None,
            tags=[],
            quality_score=None,
            source=source,
            created_at=now,
            updated_at=now
        )

class QueueStatus(Enum):
    BACKLOG = "backlog"
    ACTIVE = "active"
    COMPLETE = "complete"

    @classmethod
    def from_str(cls, s: str) -> 'QueueStatus':
        s = s.lower()
        if s == "backlog":
            return cls.BACKLOG
        elif s == "active":
            return cls.ACTIVE
        elif s == "complete":
            return cls.COMPLETE
        else:
            raise TodoziError(f"Invalid queue status: {s}")

    def __str__(self) -> str:
        return self.value

@dataclass
class QueueItem:
    id: str
    task_name: str
    task_description: str
    priority: Priority
    project_id: Optional[str]
    status: QueueStatus
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls, task_name: str, task_description: str, priority: Priority, project_id: Optional[str]) -> 'QueueItem':
        now = datetime.now(timezone.utc)
        item_id = f"queue_{str(uuid.uuid4())[:8]}"
        return cls(
            id=item_id,
            task_name=task_name,
            task_description=task_description,
            priority=priority,
            project_id=project_id,
            status=QueueStatus.BACKLOG,
            created_at=now,
            updated_at=now
        )

    def start(self) -> None:
        self.status = QueueStatus.ACTIVE
        self.updated_at = datetime.now(timezone.utc)

    def complete(self) -> None:
        self.status = QueueStatus.COMPLETE
        self.updated_at = datetime.now(timezone.utc)

    def is_backlog(self) -> bool:
        return self.status == QueueStatus.BACKLOG

    def is_active(self) -> bool:
        return self.status == QueueStatus.ACTIVE

    def is_complete(self) -> bool:
        return self.status == QueueStatus.COMPLETE

@dataclass
class QueueSession:
    id: str
    queue_item_id: str
    start_time: datetime
    end_time: Optional[datetime]
    duration_seconds: Optional[int]
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls, queue_item_id: str) -> 'QueueSession':
        now = datetime.now(timezone.utc)
        session_id = f"session_{str(uuid.uuid4())[:8]}"
        return cls(
            id=session_id,
            queue_item_id=queue_item_id,
            start_time=now,
            end_time=None,
            duration_seconds=None,
            created_at=now,
            updated_at=now
        )

    def end(self) -> None:
        end_time = datetime.now(timezone.utc)
        self.end_time = end_time
        self.duration_seconds = int((end_time - self.start_time).total_seconds())
        self.updated_at = end_time

    def is_active(self) -> bool:
        return self.end_time is None

    def get_current_duration(self) -> int:
        if self.is_active():
            return int((datetime.now(timezone.utc) - self.start_time).total_seconds())
        return self.duration_seconds or 0

@dataclass
class QueueCollection:
    version: str = "1.0.0"
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    items: Dict[str, QueueItem] = field(default_factory=dict)
    sessions: Dict[str, QueueSession] = field(default_factory=dict)

    @classmethod
    def new(cls) -> 'QueueCollection':
        now = datetime.now(timezone.utc)
        return cls(
            version="1.0.0",
            created_at=now,
            updated_at=now,
            items={},
            sessions={}
        )

    def add_item(self, item: QueueItem) -> None:
        self.items[item.id] = item
        self.updated_at = datetime.now(timezone.utc)

    def get_item(self, id: str) -> Optional[QueueItem]:
        return self.items.get(id)

    def get_item_mut(self, id: str) -> Optional[QueueItem]:
        return self.items.get(id)

    def remove_item(self, id: str) -> Optional[QueueItem]:
        item = self.items.pop(id, None)
        if item is not None:
            self.updated_at = datetime.now(timezone.utc)
        return item

    def get_all_items(self) -> List[QueueItem]:
        return list(self.items.values())

    def get_items_by_status(self, status: QueueStatus) -> List[QueueItem]:
        return [item for item in self.items.values() if item.status == status]

    def get_backlog_items(self) -> List[QueueItem]:
        return self.get_items_by_status(QueueStatus.BACKLOG)

    def get_active_items(self) -> List[QueueItem]:
        return self.get_items_by_status(QueueStatus.ACTIVE)

    def get_complete_items(self) -> List[QueueItem]:
        return self.get_items_by_status(QueueStatus.COMPLETE)

    def start_session(self, queue_item_id: str) -> str:
        item = self.items.get(queue_item_id)
        if item is None:
            raise TodoziError("Queue item not found")
        if not item.is_backlog():
            raise TodoziError("Item is not in backlog status")
        session = QueueSession.new(queue_item_id)
        session_id = session.id
        self.sessions[session_id] = session
        item.start()
        self.updated_at = datetime.now(timezone.utc)
        return session_id

    def end_session(self, session_id: str) -> None:
        session = self.sessions.get(session_id)
        if session is None:
            raise TodoziError("Session not found")
        if not session.is_active():
            raise TodoziError("Session is already ended")
        session.end()
        item = self.items.get(session.queue_item_id)
        if item is not None:
            item.complete()
        self.updated_at = datetime.now(timezone.utc)

    def get_active_sessions(self) -> List[QueueSession]:
        return [session for session in self.sessions.values() if session.is_active()]

    def get_session(self, id: str) -> Optional[QueueSession]:
        return self.sessions.get(id)

@dataclass
class ApiKey:
    user_id: str
    public_key: str
    private_key: str
    active: bool
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls) -> 'ApiKey':
        now = datetime.now(timezone.utc)
        user_id = f"user_{str(uuid.uuid4())[:8]}"
        time_str = str(int(now.timestamp()))
        mt_rand = str(random.randint(0, 2**64))
        rand_str = str(random.randint(0, 2**64))
        input_str = time_str + mt_rand + rand_str
        public_key = hashlib.sha256(input_str.encode()).hexdigest()
        private_key = hashlib.sha512(public_key.encode()).hexdigest()
        return cls(
            user_id=user_id,
            public_key=public_key,
            private_key=private_key,
            active=True,
            created_at=now,
            updated_at=now
        )

    @classmethod
    def with_user_id(cls, user_id: str) -> 'ApiKey':
        now = datetime.now(timezone.utc)
        time_str = str(int(now.timestamp()))
        mt_rand = str(random.randint(0, 2**64))
        rand_str = str(random.randint(0, 2**64))
        input_str = time_str + mt_rand + rand_str
        public_key = hashlib.sha256(input_str.encode()).hexdigest()
        private_key = hashlib.sha512(public_key.encode()).hexdigest()
        return cls(
            user_id=user_id,
            public_key=public_key,
            private_key=private_key,
            active=True,
            created_at=now,
            updated_at=now
        )

    def deactivate(self) -> None:
        self.active = False
        self.updated_at = datetime.now(timezone.utc)

    def activate(self) -> None:
        self.active = True
        self.updated_at = datetime.now(timezone.utc)

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
    created_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    keys: Dict[str, ApiKey] = field(default_factory=dict)

    @classmethod
    def new(cls) -> 'ApiKeyCollection':
        now = datetime.now(timezone.utc)
        return cls(
            version="1.0.0",
            created_at=now,
            updated_at=now,
            keys={}
        )

    def add_key(self, key: ApiKey) -> None:
        self.keys[key.user_id] = key
        self.updated_at = datetime.now(timezone.utc)

    def get_key(self, user_id: str) -> Optional[ApiKey]:
        return self.keys.get(user_id)

    def get_key_by_public(self, public_key: str) -> Optional[ApiKey]:
        for key in self.keys.values():
            if key.public_key == public_key:
                return key
        return None

    def get_all_keys(self) -> List[ApiKey]:
        return list(self.keys.values())

    def get_active_keys(self) -> List[ApiKey]:
        return [key for key in self.keys.values() if key.is_active()]

    def remove_key(self, user_id: str) -> Optional[ApiKey]:
        key = self.keys.pop(user_id, None)
        if key is not None:
            self.updated_at = datetime.now(timezone.utc)
        return key

    def deactivate_key(self, user_id: str) -> bool:
        key = self.keys.get(user_id)
        if key is not None:
            key.deactivate()
            self.updated_at = datetime.now(timezone.utc)
            return True
        return False

    def activate_key(self, user_id: str) -> bool:
        key = self.keys.get(user_id)
        if key is not None:
            key.activate()
            self.updated_at = datetime.now(timezone.utc)
            return True
        return False

class SummaryPriority(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

    @classmethod
    def from_str(cls, s: str) -> 'SummaryPriority':
        s = s.lower()
        if s == "low":
            return cls.LOW
        elif s == "medium":
            return cls.MEDIUM
        elif s == "high":
            return cls.HIGH
        elif s == "critical":
            return cls.CRITICAL
        else:
            raise TodoziError(f"Invalid summary priority: {s}")

    def __str__(self) -> str:
        return self.value

@dataclass
class Summary:
    id: str
    content: str
    context: Optional[str]
    priority: SummaryPriority
    tags: List[str]
    created_at: datetime
    updated_at: datetime

    @classmethod
    def new(cls, content: str, priority: SummaryPriority) -> 'Summary':
        now = datetime.now(timezone.utc)
        return cls(
            id=str(uuid.uuid4()),
            content=content,
            context=None,
            priority=priority,
            tags=[],
            created_at=now,
            updated_at=now
        )

    def with_context(self, context: str) -> 'Summary':
        self.context = context
        self.updated_at = datetime.now(timezone.utc)
        return self

    def with_tags(self, tags: List[str]) -> 'Summary':
        self.tags = tags
        self.updated_at = datetime.now(timezone.utc)
        return self

class ReminderPriority(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

    @classmethod
    def from_str(cls, s: str) -> 'ReminderPriority':
        s = s.lower()
        if s == "low":
            return cls.LOW
        elif s == "medium":
            return cls.MEDIUM
        elif s == "high":
            return cls.HIGH
        elif s == "critical":
            return cls.CRITICAL
        else:
            raise TodoziError(f"Invalid reminder priority: {s}")

    def __str__(self) -> str:
        return self.value

class ReminderStatus(Enum):
    PENDING = "pending"
    ACTIVE = "active"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    OVERDUE = "overdue"

    @classmethod
    def from_str(cls, s: str) -> 'ReminderStatus':
        s = s.lower()
        if s == "pending":
            return cls.PENDING
        elif s == "active":
            return cls.ACTIVE
        elif s == "completed":
            return cls.COMPLETED
        elif s in ["cancelled", "canceled"]:
            return cls.CANCELLED
        elif s == "overdue":
            return cls.OVERDUE
        else:
            raise TodoziError(f"Invalid reminder status: {s}")

    def __str__(self) -> str:
        return self.value

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

    @classmethod
    def new(cls, content: str, remind_at: datetime, priority: ReminderPriority) -> 'Reminder':
        now = datetime.now(timezone.utc)
        return cls(
            id=str(uuid.uuid4()),
            content=content,
            remind_at=remind_at,
            priority=priority,
            status=ReminderStatus.PENDING,
            tags=[],
            created_at=now,
            updated_at=now
        )

    def with_tags(self, tags: List[str]) -> 'Reminder':
        self.tags = tags
        self.updated_at = datetime.now(timezone.utc)
        return self

    def is_overdue(self) -> bool:
        return self.remind_at < datetime.now(timezone.utc) and self.status == ReminderStatus.PENDING

    def mark_completed(self) -> None:
        self.status = ReminderStatus.COMPLETED
        self.updated_at = datetime.now(timezone.utc)

    def mark_cancelled(self) -> None:
        self.status = ReminderStatus.CANCELLED
        self.updated_at = datetime.now(timezone.utc)

    def activate(self) -> None:
        self.status = ReminderStatus.ACTIVE
        self.updated_at = datetime.now(timezone.utc)

@dataclass
class MLEngine:
    model_name: str
    temperature: float = 0.7
    max_tokens: int = 4096

    @classmethod
    def new(cls, model_name: str) -> 'MLEngine':
        return cls(model_name=model_name)

    def with_temperature(self, temperature: float) -> 'MLEngine':
        self.temperature = temperature
        return self

    def with_max_tokens(self, max_tokens: int) -> 'MLEngine':
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

@dataclass
class ProjectStats:
    project_name: str
    total_tasks: int
    active_tasks: int
    completed_tasks: int
    archived_tasks: int
    deleted_tasks: int

@dataclass
class SemanticSearchResult:
    task: Task
    similarity_score: float
    matched_content: str

@dataclass
class MigrationReport:
    tasks_found: int = 0
    tasks_migrated: int = 0
    projects_migrated: int = 0
    project_stats: List['ProjectMigrationStats'] = field(default_factory=list)
    errors: List[str] = field(default_factory=list)

@dataclass
class ProjectMigrationStats:
    project_name: str
    initial_tasks: int
    migrated_tasks: int
    final_tasks: int

@dataclass
class ProjectTaskContainer:
    project_name: str
    project_hash: str
    created_at: datetime
    updated_at: datetime
    active_tasks: Dict[str, Task] = field(default_factory=dict)
    completed_tasks: Dict[str, Task] = field(default_factory=dict)
    archived_tasks: Dict[str, Task] = field(default_factory=dict)
    deleted_tasks: Dict[str, Task] = field(default_factory=dict)

    @classmethod
    def new(cls, project_name: str) -> 'ProjectTaskContainer':
        now = datetime.now(timezone.utc)
        project_hash = cls.hash_project_name(project_name)
        return cls(
            project_name=project_name,
            project_hash=project_hash,
            created_at=now,
            updated_at=now,
            active_tasks={},
            completed_tasks={},
            archived_tasks={},
            deleted_tasks={}
        )

    @staticmethod
    def hash_project_name(project_name: str) -> str:
        return hashlib.md5(project_name.encode()).hexdigest()

    def add_task(self, task: Task) -> None:
        task_id = task.id
        if task.status in [Status.TODO, Status.PENDING, Status.IN_PROGRESS, Status.BLOCKED, Status.REVIEW]:
            self.active_tasks[task_id] = task
        elif task.status in [Status.DONE, Status.COMPLETED]:
            self.completed_tasks[task_id] = task
        elif task.status in [Status.CANCELLED, Status.DEFERRED]:
            self.archived_tasks[task_id] = task
        self.updated_at = datetime.now(timezone.utc)

    def get_task(self, task_id: str) -> Optional[Task]:
        return (
            self.active_tasks.get(task_id) or
            self.completed_tasks.get(task_id) or
            self.archived_tasks.get(task_id) or
            self.deleted_tasks.get(task_id)
        )

    def get_task_mut(self, task_id: str) -> Optional[Task]:
        if task_id in self.active_tasks:
            return self.active_tasks[task_id]
        elif task_id in self.completed_tasks:
            return self.completed_tasks[task_id]
        elif task_id in self.archived_tasks:
            return self.archived_tasks[task_id]
        elif task_id in self.deleted_tasks:
            return self.deleted_tasks[task_id]
        return None

    def remove_task(self, task_id: str) -> Optional[Task]:
        return (
            self.active_tasks.pop(task_id, None) or
            self.completed_tasks.pop(task_id, None) or
            self.archived_tasks.pop(task_id, None) or
            self.deleted_tasks.pop(task_id, None)
        )

    def update_task_status(self, task_id: str, new_status: Status) -> None:
        task = self.remove_task(task_id)
        if task is not None:
            task.status = new_status
            task.updated_at = datetime.now(timezone.utc)
            self.add_task(task)

    def get_all_tasks(self) -> List[Task]:
        all_tasks = []
        all_tasks.extend(self.active_tasks.values())
        all_tasks.extend(self.completed_tasks.values())
        all_tasks.extend(self.archived_tasks.values())
        all_tasks.extend(self.deleted_tasks.values())
        return all_tasks

    def get_filtered_tasks(self, filters: TaskFilters) -> List[Task]:
        all_tasks = self.get_all_tasks()
        result = []
        for task in all_tasks:
            if filters.project is not None and task.parent_project != filters.project:
                continue
            if filters.status is not None and task.status != filters.status:
                continue
            if filters.priority is not None and task.priority != filters.priority:
                continue
            if filters.assignee is not None and task.assignee != filters.assignee:
                continue
            if filters.tags is not None and not any(tag in task.tags for tag in filters.tags):
                continue
            if filters.search is not None:
                search_lower = filters.search.lower()
                if (search_lower not in task.action.lower() and
                    (task.context_notes is None or search_lower not in task.context_notes.lower())):
                    continue
            result.append(task)
        return result