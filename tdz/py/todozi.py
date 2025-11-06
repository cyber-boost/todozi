"""Main Todozi interface.

High-level Todozi interface combining all functionality.
"""

from .tdz import TDZ
from .agent import AgentManager
from .memory import MemoryManager
from .idea import IdeaManager
from .reminder import ReminderManager
from .summary import SummaryManager


class TodoziApp:
    """Main Todozi application interface."""

    def __init__(self):
        self.tdz = TDZ()
        self.agents = AgentManager()
        self.memories = MemoryManager()
        self.ideas = IdeaManager()
        self.reminders = ReminderManager()
        self.summaries = SummaryManager()

    async def initialize(self):
        """Initialize all components."""
        await self.tdz.initialize()
        await self.agents.load_agents()

    async def shutdown(self):
        """Shutdown and cleanup."""
        pass
