"""Agent management for Todozi.

This module provides functionality for managing AI agents, their assignments,
and their interactions with tasks.
"""

from typing import Dict, List, Optional
from datetime import datetime
import uuid as uuid_lib

from .models import Agent, AgentAssignment, AgentStatus, AssignmentStatus
from .error import ValidationError


class AgentManager:
    """Manages AI agents and their assignments."""

    def __init__(self):
        self.agents: Dict[str, Agent] = {}
        self.agent_assignments: List[AgentAssignment] = []

    async def load_agents(self):
        """Load agents from storage."""
        if not self.agents:
            # Create default agents if none exist
            from .storage import create_default_agents, list_agents
            create_default_agents()

        from .storage import list_agents
        agent_list = list_agents()
        for agent in agent_list:
            self.agents[agent.id] = agent

    async def create_agent(self, agent: Agent) -> str:
        """Create a new agent."""
        agent.id = str(uuid_lib.uuid4())
        agent.created_at = datetime.utcnow()
        agent.updated_at = datetime.utcnow()

        from .storage import save_agent
        save_agent(agent)

        self.agents[agent.id] = agent
        return agent.id

    async def update_agent(self, agent_id: str, updates: 'AgentUpdate'):
        """Update an existing agent."""
        if agent_id not in self.agents:
            raise ValidationError(f"Agent {agent_id} not found")

        agent = self.agents[agent_id]

        if updates.name is not None:
            agent.name = updates.name
        if updates.description is not None:
            agent.description = updates.description
        if updates.capabilities is not None:
            agent.capabilities = updates.capabilities
        if updates.specializations is not None:
            agent.specializations = updates.specializations
        if updates.status is not None:
            agent.metadata.status = updates.status

        agent.updated_at = datetime.utcnow()

        from .storage import save_agent
        save_agent(agent)

    async def delete_agent(self, agent_id: str):
        """Delete an agent."""
        if agent_id not in self.agents:
            raise ValidationError(f"Agent {agent_id} not found")

        del self.agents[agent_id]

    def get_agent(self, agent_id: str) -> Optional[Agent]:
        """Get an agent by ID."""
        return self.agents.get(agent_id)

    def get_all_agents(self) -> List[Agent]:
        """Get all agents."""
        return list(self.agents.values())

    def get_available_agents(self) -> List[Agent]:
        """Get all available agents."""
        return [a for a in self.agents.values() if a.metadata.status == AgentStatus.AVAILABLE]

    def get_agents_by_specialization(self, specialization: str) -> List[Agent]:
        """Get agents with a specific specialization."""
        return [a for a in self.agents.values() if specialization in a.specializations]

    def get_agents_by_capability(self, capability: str) -> List[Agent]:
        """Get agents with a specific capability."""
        return [a for a in self.agents.values() if capability in a.capabilities]

    async def assign_task_to_agent(self, task_id: str, agent_id: str, project_id: str) -> str:
        """Assign a task to an agent."""
        agent = self.get_agent(agent_id)
        if not agent:
            raise ValidationError(f"Agent {agent_id} not found")

        if agent.metadata.status != AgentStatus.AVAILABLE:
            raise ValidationError(
                f"Agent {agent_id} is not available (status: {agent.metadata.status})"
            )

        assignment = AgentAssignment(
            agent_id=agent_id,
            task_id=task_id,
            project_id=project_id,
            assigned_at=datetime.utcnow(),
            status=AssignmentStatus.ASSIGNED
        )

        # Update agent status
        agent.metadata.status = AgentStatus.BUSY
        agent.updated_at = datetime.utcnow()

        from .storage import save_agent
        save_agent(agent)

        self.agent_assignments.append(assignment)
        return assignment.task_id

    async def complete_agent_assignment(self, task_id: str):
        """Mark an agent assignment as completed."""
        assignment = None
        assignment_index = None

        for i, a in enumerate(self.agent_assignments):
            if a.task_id == task_id:
                assignment = a
                assignment_index = i
                break

        if not assignment:
            raise ValidationError(f"Assignment for task {task_id} not found")

        assignment.status = AssignmentStatus.COMPLETED

        # Update agent status back to available
        agent = self.agents.get(assignment.agent_id)
        if agent:
            agent.metadata.status = AgentStatus.AVAILABLE
            agent.updated_at = datetime.utcnow()

            from .storage import save_agent
            save_agent(agent)

    def get_agent_assignments(self, agent_id: str) -> List[AgentAssignment]:
        """Get all assignments for an agent."""
        return [a for a in self.agent_assignments if a.agent_id == agent_id]

    def get_task_assignments(self, task_id: str) -> List[AgentAssignment]:
        """Get all assignments for a task."""
        return [a for a in self.agent_assignments if a.task_id == task_id]

    def find_best_agent(self, required_specialization: str,
                        preferred_capability: Optional[str] = None) -> Optional[Agent]:
        """Find the best agent for a task."""
        candidates = [
            a for a in self.agents.values()
            if a.metadata.status == AgentStatus.AVAILABLE
            and required_specialization in a.specializations
        ]

        if not candidates:
            return None

        # Sort by preferred capability if specified
        if preferred_capability:
            candidates.sort(
                key=lambda a: preferred_capability in a.capabilities,
                reverse=True
            )

        return candidates[0] if candidates else None

    async def update_agent_status(self, agent_id: str, status: AgentStatus):
        """Update an agent's status."""
        agent = self.agents.get(agent_id)
        if not agent:
            raise ValidationError(f"Agent {agent_id} not found")

        agent.metadata.status = status
        agent.updated_at = datetime.utcnow()

        from .storage import save_agent
        save_agent(agent)

    def get_agent_statistics(self) -> 'AgentStatistics':
        """Get statistics about agents."""
        total_agents = len(self.agents)
        available_agents = len([a for a in self.agents.values()
                               if a.metadata.status == AgentStatus.AVAILABLE])
        busy_agents = len([a for a in self.agents.values()
                          if a.metadata.status == AgentStatus.BUSY])
        inactive_agents = len([a for a in self.agents.values()
                              if a.metadata.status == AgentStatus.INACTIVE])

        total_assignments = len(self.agent_assignments)
        completed_assignments = len([a for a in self.agent_assignments
                                     if a.status == AssignmentStatus.COMPLETED])

        return AgentStatistics(
            total_agents=total_agents,
            available_agents=available_agents,
            busy_agents=busy_agents,
            inactive_agents=inactive_agents,
            total_assignments=total_assignments,
            completed_assignments=completed_assignments
        )


class AgentUpdate:
    """Agent update structure."""

    def __init__(self):
        self.name: Optional[str] = None
        self.description: Optional[str] = None
        self.capabilities: Optional[List[str]] = None
        self.specializations: Optional[List[str]] = None
        self.status: Optional[AgentStatus] = None

    def with_name(self, name: str) -> 'AgentUpdate':
        """Set the name."""
        self.name = name
        return self

    def with_description(self, description: str) -> 'AgentUpdate':
        """Set the description."""
        self.description = description
        return self

    def with_capabilities(self, capabilities: List[str]) -> 'AgentUpdate':
        """Set the capabilities."""
        self.capabilities = capabilities
        return self

    def with_specializations(self, specializations: List[str]) -> 'AgentUpdate':
        """Set the specializations."""
        self.specializations = specializations
        return self

    def with_status(self, status: AgentStatus) -> 'AgentUpdate':
        """Set the status."""
        self.status = status
        return self


class AgentStatistics:
    """Agent statistics."""

    def __init__(self, total_agents: int, available_agents: int, busy_agents: int,
                 inactive_agents: int, total_assignments: int, completed_assignments: int):
        self.total_agents = total_agents
        self.available_agents = available_agents
        self.busy_agents = busy_agents
        self.inactive_agents = inactive_agents
        self.total_assignments = total_assignments
        self.completed_assignments = completed_assignments

    def completion_rate(self) -> float:
        """Calculate the completion rate."""
        if self.total_assignments == 0:
            return 0.0
        return (self.completed_assignments / self.total_assignments) * 100.0


def parse_agent_assignment_format(agent_text: str) -> AgentAssignment:
    """Parse agent assignment from XML-like format."""
    start_tag = "<todozi_agent>"
    end_tag = "</todozi_agent>"

    start = agent_text.find(start_tag)
    if start == -1:
        raise ValidationError("Missing <todozi_agent> start tag")

    end = agent_text.find(end_tag)
    if end == -1:
        raise ValidationError("Missing </todozi_agent> end tag")

    content = agent_text[start + len(start_tag):end]
    parts = [s.strip() for s in content.split(';')]

    if len(parts) < 3:
        raise ValidationError(
            "Invalid agent assignment format: need at least 3 parts (agent_id; task_id; project_id)"
        )

    return AgentAssignment(
        agent_id=parts[0],
        task_id=parts[1],
        project_id=parts[2],
        assigned_at=datetime.utcnow(),
        status=AssignmentStatus.ASSIGNED
    )
