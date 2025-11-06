"""Code generation chunking for Todozi.

This module handles breaking down large code generation tasks into manageable chunks
with dependency tracking and validation.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Dict, List, Optional, Set
from datetime import datetime
import re


class ChunkingLevel(str, Enum):
    """Levels of code chunking."""
    PROJECT = "project"
    MODULE = "module"
    CLASS = "class"
    METHOD = "method"
    BLOCK = "block"

    def max_tokens(self) -> int:
        """Get maximum tokens for this level."""
        return {
            ChunkingLevel.PROJECT: 100,
            ChunkingLevel.MODULE: 500,
            ChunkingLevel.CLASS: 1000,
            ChunkingLevel.METHOD: 300,
            ChunkingLevel.BLOCK: 100,
        }[self]

    def description(self) -> str:
        """Get description for this level."""
        return {
            ChunkingLevel.PROJECT: "High-level project planning and architecture",
            ChunkingLevel.MODULE: "Major system components and interfaces",
            ChunkingLevel.CLASS: "Class definitions and major functions",
            ChunkingLevel.METHOD: "Individual methods and helper functions",
            ChunkingLevel.BLOCK: "Small code blocks and error handling",
        }[self]

    def example(self) -> str:
        """Get example for this level."""
        return {
            ChunkingLevel.PROJECT: "Build web scraper with database storage",
            ChunkingLevel.MODULE: "Create database handler module",
            ChunkingLevel.CLASS: "Implement DatabaseConnection class",
            ChunkingLevel.METHOD: "Write insert_record method",
            ChunkingLevel.BLOCK: "Add error handling for connection timeout",
        }[self]


class ChunkStatus(str, Enum):
    """Status of a code chunk."""
    PENDING = "pending"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    VALIDATED = "validated"
    FAILED = "failed"


@dataclass
class ProjectState:
    """State of the overall project generation."""
    total_lines: int = 0
    max_lines: int = 1000
    current_module: str = ""
    dependencies: List[str] = field(default_factory=list)
    completed_modules: List[str] = field(default_factory=list)
    pending_modules: List[str] = field(default_factory=list)
    global_variables: Dict[str, str] = field(default_factory=dict)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    def to_state_string(self) -> str:
        """Convert to formatted state string."""
        return f"""<project_state>
- Total lines written: {self.total_lines}/{self.max_lines}
- Current module: {self.current_module}
- Dependencies: {', '.join(self.dependencies)}
- Completed modules: {', '.join(self.completed_modules)}
- Pending modules: {', '.join(self.pending_modules)}
- Global variables: {', '.join(f'{k}={v}' for k, v in self.global_variables.items())}
- Created: {self.created_at.strftime('%Y-%m-%d %H:%M:%S')}
- Updated: {self.updated_at.strftime('%Y-%m-%d %H:%M:%S')}
</project_state>"""

    def add_completed_module(self, module: str):
        """Add a completed module."""
        if module not in self.completed_modules:
            self.completed_modules.append(module)
            self.updated_at = datetime.utcnow()

    def add_pending_module(self, module: str):
        """Add a pending module."""
        if module not in self.pending_modules:
            self.pending_modules.append(module)
            self.updated_at = datetime.utcnow()

    def set_global_variable(self, key: str, value: str):
        """Set a global variable."""
        self.global_variables[key] = value
        self.updated_at = datetime.utcnow()

    def increment_lines(self, lines: int):
        """Increment line count."""
        self.total_lines += lines
        self.updated_at = datetime.utcnow()


@dataclass
class ContextWindow:
    """Context window for tracking code generation state."""
    previous_class: str = ""
    current_class: str = ""
    next_planned: str = ""
    global_vars_in_scope: List[str] = field(default_factory=list)
    imports_used: List[str] = field(default_factory=list)
    function_signatures: Dict[str, str] = field(default_factory=dict)
    error_patterns_seen: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    def to_context_string(self) -> str:
        """Convert to formatted context string."""
        return f"""<context_window>
- Previous class: {self.previous_class}
- Current class: {self.current_class}
- Next planned: {self.next_planned}
- Global variables in scope: {', '.join(self.global_vars_in_scope)}
- Imports used: {', '.join(self.imports_used)}
- Function signatures: {', '.join(f'{k}: {v}' for k, v in self.function_signatures.items())}
- Error patterns seen: {', '.join(self.error_patterns_seen)}
- Created: {self.created_at.strftime('%Y-%m-%d %H:%M:%S')}
- Updated: {self.updated_at.strftime('%Y-%m-%d %H:%M:%S')}
</context_window>"""

    def add_import(self, import_stmt: str):
        """Add an import."""
        if import_stmt not in self.imports_used:
            self.imports_used.append(import_stmt)
            self.updated_at = datetime.utcnow()

    def add_function_signature(self, name: str, signature: str):
        """Add a function signature."""
        self.function_signatures[name] = signature
        self.updated_at = datetime.utcnow()

    def add_error_pattern(self, pattern: str):
        """Add an error pattern."""
        if pattern not in self.error_patterns_seen:
            self.error_patterns_seen.append(pattern)
            self.updated_at = datetime.utcnow()

    def set_current_class(self, class_name: str):
        """Set the current class being worked on."""
        self.previous_class = self.current_class
        self.current_class = class_name
        self.updated_at = datetime.utcnow()


@dataclass
class CodeChunk:
    """A chunk of code to be generated."""
    chunk_id: str
    status: ChunkStatus = ChunkStatus.PENDING
    dependencies: List[str] = field(default_factory=list)
    code: str = ""
    tests: str = ""
    validated: bool = False
    level: ChunkingLevel = ChunkingLevel.METHOD
    estimated_tokens: int = 0
    created_at: datetime = field(default_factory=lambda: datetime.utcnow())
    updated_at: datetime = field(default_factory=lambda: datetime.utcnow())

    @staticmethod
    def new(chunk_id: str, level: ChunkingLevel) -> 'CodeChunk':
        """Create a new code chunk."""
        return CodeChunk(chunk_id=chunk_id, level=level)

    def add_dependency(self, dep: str):
        """Add a dependency."""
        if dep not in self.dependencies:
            self.dependencies.append(dep)
            self.updated_at = datetime.utcnow()

    def set_code(self, code: str):
        """Set the code for this chunk."""
        self.estimated_tokens = len(code.split())
        self.code = code
        self.updated_at = datetime.utcnow()

    def set_tests(self, tests: str):
        """Set the tests for this chunk."""
        self.tests = tests
        self.updated_at = datetime.utcnow()

    def mark_completed(self):
        """Mark the chunk as completed."""
        self.status = ChunkStatus.COMPLETED
        self.updated_at = datetime.utcnow()

    def mark_validated(self):
        """Mark the chunk as validated."""
        self.validated = True
        self.status = ChunkStatus.VALIDATED
        self.updated_at = datetime.utcnow()

    def mark_failed(self):
        """Mark the chunk as failed."""
        self.status = ChunkStatus.FAILED
        self.updated_at = datetime.utcnow()


class CodeGenerationGraph:
    """Graph managing code generation chunks and dependencies."""

    def __init__(self, max_lines: int):
        self.chunks: Dict[str, CodeChunk] = {}
        self.project_state = ProjectState(max_lines=max_lines)
        self.context_window = ContextWindow()

    def add_chunk(self, chunk_id: str, level: ChunkingLevel, deps: List[str]):
        """Add a new chunk to the graph."""
        chunk = CodeChunk.new(chunk_id, level)
        for dep in deps:
            chunk.add_dependency(dep)
        self.chunks[chunk_id] = chunk

    def get_ready_chunks(self) -> List[str]:
        """Get chunks that are ready to be worked on."""
        ready = []
        for chunk_id, chunk in self.chunks.items():
            if chunk.status == ChunkStatus.PENDING:
                deps_satisfied = all(
                    self.chunks.get(dep) and
                    self.chunks[dep].status in [ChunkStatus.COMPLETED, ChunkStatus.VALIDATED]
                    for dep in chunk.dependencies
                )
                if deps_satisfied:
                    ready.append(chunk_id)
        return ready

    def get_chunk(self, chunk_id: str) -> Optional[CodeChunk]:
        """Get a chunk by ID."""
        return self.chunks.get(chunk_id)

    def get_chunk_mut(self, chunk_id: str) -> Optional[CodeChunk]:
        """Get a mutable reference to a chunk."""
        return self.chunks.get(chunk_id)

    def update_chunk_code(self, chunk_id: str, code: str):
        """Update the code for a chunk."""
        chunk = self.chunks.get(chunk_id)
        if not chunk:
            raise ValueError(f"Chunk {chunk_id} not found")

        chunk.set_code(code)
        self.project_state.increment_lines(len(code.splitlines()))

    def update_chunk_tests(self, chunk_id: str, tests: str):
        """Update the tests for a chunk."""
        chunk = self.chunks.get(chunk_id)
        if not chunk:
            raise ValueError(f"Chunk {chunk_id} not found")

        chunk.set_tests(tests)

    def mark_chunk_completed(self, chunk_id: str):
        """Mark a chunk as completed."""
        chunk = self.chunks.get(chunk_id)
        if not chunk:
            raise ValueError(f"Chunk {chunk_id} not found")

        chunk.mark_completed()
        self.project_state.add_completed_module(chunk_id)

    def mark_chunk_validated(self, chunk_id: str):
        """Mark a chunk as validated."""
        chunk = self.chunks.get(chunk_id)
        if not chunk:
            raise ValueError(f"Chunk {chunk_id} not found")

        chunk.mark_validated()

    def get_project_summary(self) -> str:
        """Get a summary of the project state."""
        completed_count = sum(
            1 for c in self.chunks.values()
            if c.status in [ChunkStatus.COMPLETED, ChunkStatus.VALIDATED]
        )
        total_count = len(self.chunks)
        pending_count = sum(
            1 for c in self.chunks.values()
            if c.status == ChunkStatus.PENDING
        )
        in_progress_count = sum(
            1 for c in self.chunks.values()
            if c.status == ChunkStatus.IN_PROGRESS
        )

        return f"""<project_summary>
- Total chunks: {total_count}
- Completed: {completed_count}
- In progress: {in_progress_count}
- Pending: {pending_count}
- Project state: {self.project_state.to_state_string()}
- Context window: {self.context_window.to_context_string()}
</project_summary>"""

    def get_next_chunk_to_work_on(self) -> Optional[str]:
        """Get the next chunk to work on."""
        ready = self.get_ready_chunks()
        return ready[0] if ready else None

    def get_chunks_by_level(self, level: ChunkingLevel) -> List[CodeChunk]:
        """Get all chunks at a specific level."""
        return [c for c in self.chunks.values() if c.level == level]

    def get_dependency_chain(self, chunk_id: str) -> List[str]:
        """Get the full dependency chain for a chunk."""
        chain = []
        visited: Set[str] = set()
        self._build_dependency_chain(chunk_id, chain, visited)
        return chain

    def _build_dependency_chain(self, chunk_id: str, chain: List[str], visited: Set[str]):
        """Recursively build dependency chain."""
        if chunk_id in visited:
            return

        visited.add(chunk_id)
        chunk = self.chunks.get(chunk_id)
        if chunk:
            for dep in chunk.dependencies:
                self._build_dependency_chain(dep, chain, visited)
            chain.append(chunk_id)


def parse_chunking_format(chunk_text: str) -> CodeChunk:
    """Parse a chunk from XML-like format."""
    start_tag = "<chunk>"
    end_tag = "</chunk>"

    start = chunk_text.find(start_tag)
    if start == -1:
        raise ValueError("Missing <chunk> start tag")

    end = chunk_text.find(end_tag)
    if end == -1:
        raise ValueError("Missing </chunk> end tag")

    content = chunk_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 3:
        raise ValueError("Invalid chunk format: need at least 3 parts (id; level; description)")

    chunk_id = parts[0]

    # Parse level
    level_str = parts[1].lower()
    try:
        level = ChunkingLevel(level_str)
    except ValueError:
        raise ValueError(f"Invalid chunking level: {parts[1]}")

    chunk = CodeChunk.new(chunk_id, level)

    # Parse dependencies
    if len(parts) > 3:
        dependencies = [s.strip() for s in parts[3].split(',') if s.strip()]
        for dep in dependencies:
            chunk.add_dependency(dep)

    # Parse code
    if len(parts) > 4:
        chunk.set_code(parts[4])

    return chunk


def process_chunking_message(message: str) -> List[CodeChunk]:
    """Process a message and extract all chunks."""
    chunks = []
    chunk_pattern = r"<chunk>.*?</chunk>"

    for match in re.finditer(chunk_pattern, message, re.DOTALL):
        chunk_text = match.group(0)
        try:
            chunk = parse_chunking_format(chunk_text)
            chunks.append(chunk)
        except ValueError as e:
            print(f"Warning: Failed to parse chunk: {e}")

    return chunks
