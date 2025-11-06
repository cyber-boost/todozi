"""Reminder management for Todozi."""

from typing import Dict, List, Optional
from datetime import datetime, timedelta
import uuid as uuid_lib

from .models import Reminder, ReminderPriority, ReminderStatus
from .error import ValidationError


class ReminderManager:
    """Manages reminders."""

    def __init__(self):
        self.reminders: Dict[str, Reminder] = {}
        self.reminder_tags: Dict[str, List[str]] = {}

    async def create_reminder(self, reminder: Reminder) -> str:
        """Create a new reminder."""
        reminder.id = str(uuid_lib.uuid4())
        reminder.created_at = datetime.utcnow()
        reminder.updated_at = datetime.utcnow()
        self.reminder_tags[reminder.id] = reminder.tags.copy()
        self.reminders[reminder.id] = reminder
        return reminder.id

    def get_reminder(self, reminder_id: str) -> Optional[Reminder]:
        """Get a reminder by ID."""
        return self.reminders.get(reminder_id)

    def get_all_reminders(self) -> List[Reminder]:
        """Get all reminders."""
        return list(self.reminders.values())

    async def update_reminder(self, reminder_id: str, updates: 'ReminderUpdate'):
        """Update a reminder."""
        reminder = self.reminders.get(reminder_id)
        if not reminder:
            raise ValidationError(f"Reminder {reminder_id} not found")

        if updates.content is not None:
            reminder.content = updates.content
        if updates.remind_at is not None:
            reminder.remind_at = updates.remind_at
        if updates.priority is not None:
            reminder.priority = updates.priority
        if updates.status is not None:
            reminder.status = updates.status
        if updates.tags is not None:
            reminder.tags = updates.tags.copy()
            self.reminder_tags[reminder_id] = updates.tags

        reminder.updated_at = datetime.utcnow()

    async def delete_reminder(self, reminder_id: str):
        """Delete a reminder."""
        if reminder_id not in self.reminders:
            raise ValidationError(f"Reminder {reminder_id} not found")
        del self.reminders[reminder_id]
        self.reminder_tags.pop(reminder_id, None)

    def search_reminders(self, query: str) -> List[Reminder]:
        """Search for reminders."""
        query_lower = query.lower()
        return [
            r for r in self.reminders.values()
            if query_lower in r.content.lower()
            or any(query_lower in tag.lower() for tag in r.tags)
        ]

    def get_reminders_by_priority(self, priority: ReminderPriority) -> List[Reminder]:
        """Get reminders by priority."""
        return [r for r in self.reminders.values() if r.priority == priority]

    def get_reminders_by_status(self, status: ReminderStatus) -> List[Reminder]:
        """Get reminders by status."""
        return [r for r in self.reminders.values() if r.status == status]

    def get_reminders_by_tag(self, tag: str) -> List[Reminder]:
        """Get reminders by tag."""
        tag_lower = tag.lower()
        return [
            r for r in self.reminders.values()
            if any(t.lower() == tag_lower for t in r.tags)
        ]

    def get_pending_reminders(self) -> List[Reminder]:
        """Get pending reminders."""
        return self.get_reminders_by_status(ReminderStatus.PENDING)

    def get_active_reminders(self) -> List[Reminder]:
        """Get active reminders."""
        return self.get_reminders_by_status(ReminderStatus.ACTIVE)

    def get_overdue_reminders(self) -> List[Reminder]:
        """Get overdue reminders."""
        now = datetime.utcnow()
        return [
            r for r in self.reminders.values()
            if r.remind_at < now
            and r.status in [ReminderStatus.PENDING, ReminderStatus.ACTIVE]
        ]

    def get_reminders_due_soon(self, duration: timedelta) -> List[Reminder]:
        """Get reminders due soon."""
        now = datetime.utcnow()
        due_time = now + duration
        return [
            r for r in self.reminders.values()
            if now < r.remind_at <= due_time
            and r.status in [ReminderStatus.PENDING, ReminderStatus.ACTIVE]
        ]

    def get_recent_reminders(self, limit: int) -> List[Reminder]:
        """Get recent reminders."""
        reminders = sorted(self.reminders.values(), key=lambda x: x.created_at, reverse=True)
        return reminders[:limit]

    def get_all_tags(self) -> List[str]:
        """Get all unique tags."""
        all_tags = set()
        for tags in self.reminder_tags.values():
            all_tags.update(tags)
        return list(all_tags)

    def get_tag_statistics(self) -> Dict[str, int]:
        """Get tag usage statistics."""
        stats = {}
        for tags in self.reminder_tags.values():
            for tag in tags:
                stats[tag] = stats.get(tag, 0) + 1
        return stats

    def get_reminder_statistics(self) -> 'ReminderStatistics':
        """Get reminder statistics."""
        return ReminderStatistics(
            total_reminders=len(self.reminders),
            pending_reminders=len(self.get_pending_reminders()),
            active_reminders=len(self.get_active_reminders()),
            overdue_reminders=len(self.get_overdue_reminders()),
            unique_tags=len(self.get_all_tags())
        )

    async def mark_reminder_completed(self, reminder_id: str):
        """Mark a reminder as completed."""
        reminder = self.reminders.get(reminder_id)
        if not reminder:
            raise ValidationError(f"Reminder {reminder_id} not found")
        reminder.mark_completed()

    async def mark_reminder_cancelled(self, reminder_id: str):
        """Mark a reminder as cancelled."""
        reminder = self.reminders.get(reminder_id)
        if not reminder:
            raise ValidationError(f"Reminder {reminder_id} not found")
        reminder.mark_cancelled()

    async def activate_reminder(self, reminder_id: str):
        """Activate a reminder."""
        reminder = self.reminders.get(reminder_id)
        if not reminder:
            raise ValidationError(f"Reminder {reminder_id} not found")
        reminder.activate()


class ReminderUpdate:
    """Reminder update structure."""
    def __init__(self):
        self.content: Optional[str] = None
        self.remind_at: Optional[datetime] = None
        self.priority: Optional[ReminderPriority] = None
        self.status: Optional[ReminderStatus] = None
        self.tags: Optional[List[str]] = None


class ReminderStatistics:
    """Reminder statistics."""
    def __init__(self, total_reminders: int, pending_reminders: int,
                 active_reminders: int, overdue_reminders: int, unique_tags: int):
        self.total_reminders = total_reminders
        self.pending_reminders = pending_reminders
        self.active_reminders = active_reminders
        self.overdue_reminders = overdue_reminders
        self.unique_tags = unique_tags

    def pending_percentage(self) -> float:
        """Get percentage of pending reminders."""
        return (self.pending_reminders / self.total_reminders * 100.0) if self.total_reminders > 0 else 0.0

    def active_percentage(self) -> float:
        """Get percentage of active reminders."""
        return (self.active_reminders / self.total_reminders * 100.0) if self.total_reminders > 0 else 0.0

    def overdue_percentage(self) -> float:
        """Get percentage of overdue reminders."""
        return (self.overdue_reminders / self.total_reminders * 100.0) if self.total_reminders > 0 else 0.0


def parse_reminder_format(reminder_text: str) -> Reminder:
    """Parse reminder from XML-like format."""
    start_tag = "<reminder>"
    end_tag = "</reminder>"

    start = reminder_text.find(start_tag)
    if start == -1:
        raise ValidationError("Missing <reminder> start tag")

    end = reminder_text.find(end_tag)
    if end == -1:
        raise ValidationError("Missing </reminder> end tag")

    content = reminder_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 3:
        raise ValidationError(
            "Invalid reminder format: need at least 3 parts (content; remind_at; priority)"
        )

    # Parse datetime
    try:
        remind_at = datetime.fromisoformat(parts[1].replace('Z', '+00:00'))
    except:
        raise ValidationError("Invalid reminder date format")

    # Parse priority
    priority = ReminderPriority(parts[2].lower())

    # Parse status
    status = ReminderStatus.PENDING
    if len(parts) > 3 and parts[3]:
        status = ReminderStatus(parts[3].lower())

    # Parse tags
    tags = []
    if len(parts) > 4 and parts[4]:
        tags = [s.strip() for s in parts[4].split(',')]

    return Reminder(
        id=str(uuid_lib.uuid4()),
        content=parts[0],
        remind_at=remind_at,
        priority=priority,
        status=status,
        tags=tags,
        created_at=datetime.utcnow(),
        updated_at=datetime.utcnow()
    )
