"""Summary management for Todozi."""

from typing import Dict, List, Optional
from datetime import datetime
import uuid as uuid_lib

from .models import Summary, SummaryPriority
from .error import ValidationError


class SummaryManager:
    """Manages summaries."""

    def __init__(self):
        self.summaries: Dict[str, Summary] = {}
        self.summary_tags: Dict[str, List[str]] = {}

    async def create_summary(self, summary: Summary) -> str:
        """Create a new summary."""
        summary.id = str(uuid_lib.uuid4())
        summary.created_at = datetime.utcnow()
        summary.updated_at = datetime.utcnow()
        self.summary_tags[summary.id] = summary.tags.copy()
        self.summaries[summary.id] = summary
        return summary.id

    def get_summary(self, summary_id: str) -> Optional[Summary]:
        """Get a summary by ID."""
        return self.summaries.get(summary_id)

    def get_all_summaries(self) -> List[Summary]:
        """Get all summaries."""
        return list(self.summaries.values())

    async def update_summary(self, summary_id: str, updates: 'SummaryUpdate'):
        """Update a summary."""
        summary = self.summaries.get(summary_id)
        if not summary:
            raise ValidationError(f"Summary {summary_id} not found")

        if updates.content is not None:
            summary.content = updates.content
        if updates.context is not None:
            summary.context = updates.context
        if updates.priority is not None:
            summary.priority = updates.priority
        if updates.tags is not None:
            summary.tags = updates.tags.copy()
            self.summary_tags[summary_id] = updates.tags

        summary.updated_at = datetime.utcnow()

    async def delete_summary(self, summary_id: str):
        """Delete a summary."""
        if summary_id not in self.summaries:
            raise ValidationError(f"Summary {summary_id} not found")
        del self.summaries[summary_id]
        self.summary_tags.pop(summary_id, None)

    def search_summaries(self, query: str) -> List[Summary]:
        """Search for summaries."""
        query_lower = query.lower()
        return [
            s for s in self.summaries.values()
            if query_lower in s.content.lower()
            or any(query_lower in tag.lower() for tag in s.tags)
            or (s.context and query_lower in s.context.lower())
        ]

    def get_summaries_by_priority(self, priority: SummaryPriority) -> List[Summary]:
        """Get summaries by priority."""
        return [s for s in self.summaries.values() if s.priority == priority]

    def get_summaries_by_tag(self, tag: str) -> List[Summary]:
        """Get summaries by tag."""
        tag_lower = tag.lower()
        return [
            s for s in self.summaries.values()
            if any(t.lower() == tag_lower for t in s.tags)
        ]

    def get_recent_summaries(self, limit: int) -> List[Summary]:
        """Get recent summaries."""
        summaries = sorted(self.summaries.values(), key=lambda x: x.created_at, reverse=True)
        return summaries[:limit]

    def get_high_priority_summaries(self) -> List[Summary]:
        """Get high priority summaries."""
        return [
            s for s in self.summaries.values()
            if s.priority in [SummaryPriority.HIGH, SummaryPriority.CRITICAL]
        ]

    def get_all_tags(self) -> List[str]:
        """Get all unique tags."""
        all_tags = set()
        for tags in self.summary_tags.values():
            all_tags.update(tags)
        return list(all_tags)

    def get_tag_statistics(self) -> Dict[str, int]:
        """Get tag usage statistics."""
        stats = {}
        for tags in self.summary_tags.values():
            for tag in tags:
                stats[tag] = stats.get(tag, 0) + 1
        return stats

    def get_summary_statistics(self) -> 'SummaryStatistics':
        """Get summary statistics."""
        return SummaryStatistics(
            total_summaries=len(self.summaries),
            high_priority_summaries=len(self.get_high_priority_summaries()),
            unique_tags=len(self.get_all_tags())
        )


class SummaryUpdate:
    """Summary update structure."""
    def __init__(self):
        self.content: Optional[str] = None
        self.context: Optional[str] = None
        self.priority: Optional[SummaryPriority] = None
        self.tags: Optional[List[str]] = None


class SummaryStatistics:
    """Summary statistics."""
    def __init__(self, total_summaries: int, high_priority_summaries: int, unique_tags: int):
        self.total_summaries = total_summaries
        self.high_priority_summaries = high_priority_summaries
        self.unique_tags = unique_tags

    def high_priority_percentage(self) -> float:
        """Get percentage of high priority summaries."""
        return (self.high_priority_summaries / self.total_summaries * 100.0) if self.total_summaries > 0 else 0.0


def parse_summary_format(summary_text: str) -> Summary:
    """Parse summary from XML-like format."""
    start_tag = "<summary>"
    end_tag = "</summary>"

    start = summary_text.find(start_tag)
    if start == -1:
        raise ValidationError("Missing <summary> start tag")

    end = summary_text.find(end_tag)
    if end == -1:
        raise ValidationError("Missing </summary> end tag")

    content = summary_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 2:
        raise ValidationError("Invalid summary format: need at least 2 parts (content; priority)")

    # Parse priority
    priority = SummaryPriority(parts[1].lower())

    # Parse context
    context = parts[2] if len(parts) > 2 and parts[2] else None

    # Parse tags
    tags = []
    if len(parts) > 3 and parts[3]:
        tags = [s.strip() for s in parts[3].split(',')]

    return Summary(
        id=str(uuid_lib.uuid4()),
        content=parts[0],
        context=context,
        priority=priority,
        tags=tags,
        created_at=datetime.utcnow(),
        updated_at=datetime.utcnow()
    )
