"""Tests for Todozi Python SDK.

Run with: python -m pytest tdz/py/tests.py
"""

import pytest
from datetime import datetime


class TestModels:
    """Test data models."""

    def test_priority_enum(self):
        """Test Priority enum."""
        from .models import Priority
        assert Priority.LOW == "low"
        assert Priority.HIGH == "high"

    def test_status_enum(self):
        """Test Status enum."""
        from .models import Status
        assert Status.TODO == "todo"
        assert Status.DONE == "done"

    def test_task_creation(self):
        """Test Task creation."""
        from .models import Task, Priority, Status
        task = Task.new(
            user_id="test_user",
            action="Test task",
            time="ASAP",
            priority=Priority.MEDIUM,
            parent_project="test",
            status=Status.TODO
        )
        assert task.action == "Test task"
        assert task.priority == Priority.MEDIUM


class TestStorage:
    """Test storage operations."""

    def test_get_storage_dir(self):
        """Test getting storage directory."""
        from .storage import get_storage_dir
        storage_dir = get_storage_dir()
        assert storage_dir.name == ".todozi"


class TestManagers:
    """Test manager classes."""

    @pytest.mark.asyncio
    async def test_idea_manager(self):
        """Test IdeaManager."""
        from .idea import IdeaManager
        from .models import Idea, ShareLevel, IdeaImportance, ItemStatus

        manager = IdeaManager()
        idea = Idea(
            id="test",
            idea="Test idea",
            share=ShareLevel.PRIVATE,
            importance=IdeaImportance.MEDIUM,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow()
        )
        idea_id = await manager.create_idea(idea)
        assert idea_id is not None


if __name__ == "__main__":
    pytest.main([__file__])
