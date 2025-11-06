"""Tag management for Todozi."""

from typing import List, Optional
from .models import Tag
import uuid as uuid_lib
from datetime import datetime


class TagManager:
    """Manages tags."""

    def __init__(self):
        self.tags: dict[str, Tag] = {}

    def create_tag(self, name: str, description: Optional[str] = None) -> Tag:
        """Create a new tag."""
        tag = Tag(
            id=str(uuid_lib.uuid4()),
            name=name,
            description=description,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow()
        )
        self.tags[tag.id] = tag
        return tag

    def get_tag(self, tag_id: str) -> Optional[Tag]:
        """Get a tag by ID."""
        return self.tags.get(tag_id)

    def list_tags(self) -> List[Tag]:
        """List all tags."""
        return list(self.tags.values())
