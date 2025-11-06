"""Memory management for Todozi."""

from typing import Dict, List, Optional
from datetime import datetime
import uuid as uuid_lib

from .models import Memory, MemoryImportance, MemoryTerm, MemoryType, ItemStatus
from .error import ValidationError


class MemoryManager:
    """Manages memories."""

    def __init__(self):
        self.memories: Dict[str, Memory] = {}
        self.memory_tags: Dict[str, List[str]] = {}

    async def create_memory(self, memory: Memory) -> str:
        """Create a new memory."""
        memory.id = str(uuid_lib.uuid4())
        memory.created_at = datetime.utcnow()
        memory.updated_at = datetime.utcnow()
        self.memory_tags[memory.id] = memory.tags.copy()
        self.memories[memory.id] = memory
        return memory.id

    def get_memory(self, memory_id: str) -> Optional[Memory]:
        """Get a memory by ID."""
        return self.memories.get(memory_id)

    def get_all_memories(self) -> List[Memory]:
        """Get all memories."""
        return list(self.memories.values())

    async def update_memory(self, memory_id: str, updates: 'MemoryUpdate'):
        """Update a memory."""
        memory = self.memories.get(memory_id)
        if not memory:
            raise ValidationError(f"Memory {memory_id} not found")

        if updates.moment is not None:
            memory.moment = updates.moment
        if updates.meaning is not None:
            memory.meaning = updates.meaning
        if updates.reason is not None:
            memory.reason = updates.reason
        if updates.importance is not None:
            memory.importance = updates.importance
        if updates.term is not None:
            memory.term = updates.term
        if updates.tags is not None:
            memory.tags = updates.tags.copy()
            self.memory_tags[memory_id] = updates.tags

        memory.updated_at = datetime.utcnow()

    async def delete_memory(self, memory_id: str):
        """Delete a memory."""
        if memory_id not in self.memories:
            raise ValidationError(f"Memory {memory_id} not found")
        del self.memories[memory_id]
        self.memory_tags.pop(memory_id, None)

    def search_memories(self, query: str) -> List[Memory]:
        """Search for memories."""
        query_lower = query.lower()
        return [
            m for m in self.memories.values()
            if query_lower in m.moment.lower()
            or query_lower in m.meaning.lower()
            or query_lower in m.reason.lower()
            or any(query_lower in tag.lower() for tag in m.tags)
        ]

    def get_memories_by_importance(self, importance: MemoryImportance) -> List[Memory]:
        """Get memories by importance level."""
        return [m for m in self.memories.values() if m.importance == importance]

    def get_memories_by_term(self, term: MemoryTerm) -> List[Memory]:
        """Get memories by term."""
        return [m for m in self.memories.values() if m.term == term]

    def get_memories_by_tag(self, tag: str) -> List[Memory]:
        """Get memories by tag."""
        tag_lower = tag.lower()
        return [
            m for m in self.memories.values()
            if any(t.lower() == tag_lower for t in m.tags)
        ]

    def get_recent_memories(self, limit: int) -> List[Memory]:
        """Get recent memories."""
        memories = sorted(self.memories.values(), key=lambda x: x.created_at, reverse=True)
        return memories[:limit]

    def get_critical_memories(self) -> List[Memory]:
        """Get critical memories."""
        return [
            m for m in self.memories.values()
            if m.importance in [MemoryImportance.HIGH, MemoryImportance.CRITICAL]
        ]

    def get_short_term_memories(self) -> List[Memory]:
        """Get short-term memories."""
        return [m for m in self.memories.values() if m.term == MemoryTerm.SHORT]

    def get_long_term_memories(self) -> List[Memory]:
        """Get long-term memories."""
        return [m for m in self.memories.values() if m.term == MemoryTerm.LONG]

    def get_memories_by_type(self, memory_type: MemoryType) -> List[Memory]:
        """Get memories by type."""
        return [m for m in self.memories.values() if m.memory_type == memory_type]

    def get_secret_memories(self) -> List[Memory]:
        """Get secret memories."""
        return self.get_memories_by_type(MemoryType.SECRET)

    def get_human_memories(self) -> List[Memory]:
        """Get human memories."""
        return self.get_memories_by_type(MemoryType.HUMAN)

    def get_all_tags(self) -> List[str]:
        """Get all unique tags."""
        all_tags = set()
        for tags in self.memory_tags.values():
            all_tags.update(tags)
        return list(all_tags)

    def get_tag_statistics(self) -> Dict[str, int]:
        """Get tag usage statistics."""
        stats = {}
        for tags in self.memory_tags.values():
            for tag in tags:
                stats[tag] = stats.get(tag, 0) + 1
        return stats

    def get_memory_statistics(self) -> 'MemoryStatistics':
        """Get memory statistics."""
        return MemoryStatistics(
            total_memories=len(self.memories),
            short_term_memories=len(self.get_short_term_memories()),
            long_term_memories=len(self.get_long_term_memories()),
            critical_memories=len(self.get_critical_memories()),
            unique_tags=len(self.get_all_tags()),
            secret_memories=len(self.get_secret_memories()),
            human_memories=len(self.get_human_memories()),
            emotional_memories=0,  # Count emotional memories
            standard_memories=len(self.get_memories_by_type(MemoryType.STANDARD))
        )


class MemoryUpdate:
    """Memory update structure."""
    def __init__(self):
        self.moment: Optional[str] = None
        self.meaning: Optional[str] = None
        self.reason: Optional[str] = None
        self.importance: Optional[MemoryImportance] = None
        self.term: Optional[MemoryTerm] = None
        self.tags: Optional[List[str]] = None


class MemoryStatistics:
    """Memory statistics."""
    def __init__(self, total_memories: int, short_term_memories: int,
                 long_term_memories: int, critical_memories: int,
                 unique_tags: int, secret_memories: int, human_memories: int,
                 emotional_memories: int, standard_memories: int):
        self.total_memories = total_memories
        self.short_term_memories = short_term_memories
        self.long_term_memories = long_term_memories
        self.critical_memories = critical_memories
        self.unique_tags = unique_tags
        self.secret_memories = secret_memories
        self.human_memories = human_memories
        self.emotional_memories = emotional_memories
        self.standard_memories = standard_memories

    def short_term_percentage(self) -> float:
        """Get percentage of short-term memories."""
        return (self.short_term_memories / self.total_memories * 100.0) if self.total_memories > 0 else 0.0

    def long_term_percentage(self) -> float:
        """Get percentage of long-term memories."""
        return (self.long_term_memories / self.total_memories * 100.0) if self.total_memories > 0 else 0.0

    def critical_percentage(self) -> float:
        """Get percentage of critical memories."""
        return (self.critical_memories / self.total_memories * 100.0) if self.total_memories > 0 else 0.0


def parse_memory_format(memory_text: str, user_id: str) -> Memory:
    """Parse memory from XML-like format."""
    start_tag = "<memory>"
    end_tag = "</memory>"

    start = memory_text.find(start_tag)
    if start == -1:
        raise ValidationError("Missing <memory> start tag")

    end = memory_text.find(end_tag)
    if end == -1:
        raise ValidationError("Missing </memory> end tag")

    content = memory_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 6:
        raise ValidationError(
            "Invalid memory format: need at least 6 parts (type; moment; meaning; reason; importance; term)"
        )

    # Parse memory type
    memory_type_str = parts[0]
    if memory_type_str in ["standard", "secret", "human", "short", "long"]:
        memory_type = MemoryType(memory_type_str)
    else:
        # Assume it's an emotional type
        memory_type = MemoryType.emotional(memory_type_str)

    # Parse tags
    tags = []
    if len(parts) > 6 and parts[6]:
        tags = [s.strip() for s in parts[6].split(',')]

    return Memory(
        id=str(uuid_lib.uuid4()),
        user_id=user_id,
        project_id=None,
        status=ItemStatus.ACTIVE,
        moment=parts[1],
        meaning=parts[2],
        reason=parts[3],
        importance=MemoryImportance(parts[4].lower()),
        term=MemoryTerm(parts[5].lower()),
        memory_type=memory_type,
        tags=tags,
        created_at=datetime.utcnow(),
        updated_at=datetime.utcnow()
    )
