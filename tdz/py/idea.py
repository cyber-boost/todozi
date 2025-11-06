"""Idea management for Todozi."""

from typing import Dict, List, Optional
from datetime import datetime
import uuid as uuid_lib

from .models import Idea, IdeaImportance, ShareLevel, ItemStatus
from .error import ValidationError


class IdeaManager:
    """Manages ideas."""

    def __init__(self):
        self.ideas: Dict[str, Idea] = {}
        self.idea_tags: Dict[str, List[str]] = {}

    async def create_idea(self, idea: Idea) -> str:
        """Create a new idea."""
        idea.id = str(uuid_lib.uuid4())
        idea.created_at = datetime.utcnow()
        idea.updated_at = datetime.utcnow()
        self.idea_tags[idea.id] = idea.tags.copy()
        self.ideas[idea.id] = idea
        return idea.id

    def get_idea(self, idea_id: str) -> Optional[Idea]:
        """Get an idea by ID."""
        return self.ideas.get(idea_id)

    def get_all_ideas(self) -> List[Idea]:
        """Get all ideas."""
        return list(self.ideas.values())

    async def update_idea(self, idea_id: str, updates: 'IdeaUpdate'):
        """Update an idea."""
        idea = self.ideas.get(idea_id)
        if not idea:
            raise ValidationError(f"Idea {idea_id} not found")

        if updates.idea is not None:
            idea.idea = updates.idea
        if updates.share is not None:
            idea.share = updates.share
        if updates.importance is not None:
            idea.importance = updates.importance
        if updates.tags is not None:
            idea.tags = updates.tags.copy()
            self.idea_tags[idea_id] = updates.tags
        if updates.context is not None:
            idea.context = updates.context

        idea.updated_at = datetime.utcnow()

    async def delete_idea(self, idea_id: str):
        """Delete an idea."""
        if idea_id not in self.ideas:
            raise ValidationError(f"Idea {idea_id} not found")
        del self.ideas[idea_id]
        self.idea_tags.pop(idea_id, None)

    def search_ideas(self, query: str) -> List[Idea]:
        """Search for ideas."""
        query_lower = query.lower()
        return [
            idea for idea in self.ideas.values()
            if query_lower in idea.idea.lower()
            or any(query_lower in tag.lower() for tag in idea.tags)
            or (idea.context and query_lower in idea.context.lower())
        ]

    def get_ideas_by_importance(self, importance: IdeaImportance) -> List[Idea]:
        """Get ideas by importance level."""
        return [i for i in self.ideas.values() if i.importance == importance]

    def get_ideas_by_share_level(self, share_level: ShareLevel) -> List[Idea]:
        """Get ideas by share level."""
        return [i for i in self.ideas.values() if i.share == share_level]

    def get_ideas_by_tag(self, tag: str) -> List[Idea]:
        """Get ideas by tag."""
        tag_lower = tag.lower()
        return [
            i for i in self.ideas.values()
            if any(t.lower() == tag_lower for t in i.tags)
        ]

    def get_public_ideas(self) -> List[Idea]:
        """Get public ideas."""
        return self.get_ideas_by_share_level(ShareLevel.PUBLIC)

    def get_team_ideas(self) -> List[Idea]:
        """Get team ideas."""
        return self.get_ideas_by_share_level(ShareLevel.TEAM)

    def get_private_ideas(self) -> List[Idea]:
        """Get private ideas."""
        return self.get_ideas_by_share_level(ShareLevel.PRIVATE)

    def get_breakthrough_ideas(self) -> List[Idea]:
        """Get breakthrough ideas."""
        return self.get_ideas_by_importance(IdeaImportance.BREAKTHROUGH)

    def get_recent_ideas(self, limit: int) -> List[Idea]:
        """Get recent ideas."""
        ideas = sorted(self.ideas.values(), key=lambda x: x.created_at, reverse=True)
        return ideas[:limit]

    def get_all_tags(self) -> List[str]:
        """Get all unique tags."""
        all_tags = set()
        for tags in self.idea_tags.values():
            all_tags.update(tags)
        return list(all_tags)

    def get_tag_statistics(self) -> Dict[str, int]:
        """Get tag usage statistics."""
        stats = {}
        for tags in self.idea_tags.values():
            for tag in tags:
                stats[tag] = stats.get(tag, 0) + 1
        return stats

    def get_idea_statistics(self) -> 'IdeaStatistics':
        """Get idea statistics."""
        return IdeaStatistics(
            total_ideas=len(self.ideas),
            public_ideas=len(self.get_public_ideas()),
            team_ideas=len(self.get_team_ideas()),
            private_ideas=len(self.get_private_ideas()),
            breakthrough_ideas=len(self.get_breakthrough_ideas()),
            unique_tags=len(self.get_all_tags())
        )


class IdeaUpdate:
    """Idea update structure."""
    def __init__(self):
        self.idea: Optional[str] = None
        self.share: Optional[ShareLevel] = None
        self.importance: Optional[IdeaImportance] = None
        self.tags: Optional[List[str]] = None
        self.context: Optional[str] = None


class IdeaStatistics:
    """Idea statistics."""
    def __init__(self, total_ideas: int, public_ideas: int, team_ideas: int,
                 private_ideas: int, breakthrough_ideas: int, unique_tags: int):
        self.total_ideas = total_ideas
        self.public_ideas = public_ideas
        self.team_ideas = team_ideas
        self.private_ideas = private_ideas
        self.breakthrough_ideas = breakthrough_ideas
        self.unique_tags = unique_tags

    def public_percentage(self) -> float:
        """Get percentage of public ideas."""
        return (self.public_ideas / self.total_ideas * 100.0) if self.total_ideas > 0 else 0.0

    def team_percentage(self) -> float:
        """Get percentage of team ideas."""
        return (self.team_ideas / self.total_ideas * 100.0) if self.total_ideas > 0 else 0.0

    def private_percentage(self) -> float:
        """Get percentage of private ideas."""
        return (self.private_ideas / self.total_ideas * 100.0) if self.total_ideas > 0 else 0.0

    def breakthrough_percentage(self) -> float:
        """Get percentage of breakthrough ideas."""
        return (self.breakthrough_ideas / self.total_ideas * 100.0) if self.total_ideas > 0 else 0.0


def parse_idea_format(idea_text: str) -> Idea:
    """Parse idea from XML-like format."""
    start_tag = "<idea>"
    end_tag = "</idea>"

    start = idea_text.find(start_tag)
    if start == -1:
        raise ValidationError("Missing <idea> start tag")

    end = idea_text.find(end_tag)
    if end == -1:
        raise ValidationError("Missing </idea> end tag")

    content = idea_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 3:
        raise ValidationError("Invalid idea format: need at least 3 parts (idea; share; importance)")

    # Parse share level
    share_str = parts[1].lower()
    if share_str in ["share", "public"]:
        share = ShareLevel.PUBLIC
    elif share_str in ["dont share", "don't share", "private"]:
        share = ShareLevel.PRIVATE
    elif share_str == "team":
        share = ShareLevel.TEAM
    else:
        share = ShareLevel.PRIVATE

    # Parse tags
    tags = []
    if len(parts) > 3 and parts[3]:
        tags = [s.strip() for s in parts[3].split(',')]

    # Parse context
    context = parts[4] if len(parts) > 4 and parts[4] else None

    return Idea(
        id=str(uuid_lib.uuid4()),
        idea=parts[0],
        project_id=None,
        status=ItemStatus.ACTIVE,
        share=share,
        importance=IdeaImportance(parts[2].lower()),
        tags=tags,
        context=context,
        created_at=datetime.utcnow(),
        updated_at=datetime.utcnow()
    )
