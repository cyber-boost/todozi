# Todozi Agent Management System - Technical Documentation

## 1. Overview

The Todozi Agent Management System provides a comprehensive framework for managing AI agents and their task assignments. The system supports agent creation, status tracking, task assignment, and performance monitoring through a JSON-backed storage layer with atomic operations.

## 2. Architecture and Design Decisions

### 2.1 Core Architecture
- **Three-Layer Architecture**: Data Models → Storage Layer → Manager Layer
- **JSON-Based Persistence**: Simple file-based storage for ease of deployment
- **Atomic Operations**: Transactional file operations to prevent data corruption
- **In-Memory Caching**: Agent data cached in memory for performance

### 2.2 Design Patterns
- **Data Class Pattern**: Immutable data structures for agent metadata
- **Builder Pattern**: Fluent interface for agent updates via `AgentUpdate`
- **Context Manager**: Atomic file operations for data consistency
- **Repository Pattern**: Separation between storage and business logic

## 3. Data Models

### 3.1 Enums

#### `AgentStatus`
```python
class AgentStatus(Enum):
    AVAILABLE = "Available"    # Ready for task assignment
    BUSY = "Busy"             # Currently assigned to a task
    INACTIVE = "Inactive"     # Not available for assignments
```

#### `AssignmentStatus`
```python
class AssignmentStatus(Enum):
    ASSIGNED = "Assigned"     # Task assigned but not completed
    COMPLETED = "Completed"   # Task successfully completed
```

### 3.2 Core Data Classes

#### `AgentMetadata`
```python
@dataclass
class AgentMetadata:
    status: AgentStatus = AgentStatus.AVAILABLE
```
**Purpose**: Tracks agent operational status.

#### `Agent`
```python
@dataclass
class Agent:
    id: str                    # Unique identifier (UUID)
    name: str                 # Human-readable name
    description: str          # Agent description
    capabilities: List[str]   # General capabilities
    specializations: List[str] # Domain specializations
    metadata: AgentMetadata   # Operational metadata
    created_at: datetime      # Creation timestamp
    updated_at: datetime      # Last update timestamp
```

**Factory Method**:
```python
@staticmethod
def new(name: str, description: str = "") -> "Agent":
    """
    Create a new agent instance with default values.
    
    Args:
        name: Agent name (required)
        description: Agent description (optional)
        
    Returns:
        Pre-configured Agent instance with UTC timestamps
    """
```

#### `AgentAssignment`
```python
@dataclass
class AgentAssignment:
    agent_id: str             # Reference to agent
    task_id: str             # Reference to task
    project_id: str          # Project context
    assigned_at: datetime    # Assignment timestamp
    status: AssignmentStatus # Current assignment status
```

#### `AgentUpdate`
```python
@dataclass
class AgentUpdate:
    name: Optional[str] = None
    description: Optional[str] = None
    capabilities: Optional[List[str]] = None
    specializations: Optional[List[str]] = None
    status: Optional[AgentStatus] = None
```

**Builder Methods**: Fluent interface for constructing updates:
- `with_name(name: str) → AgentUpdate`
- `with_description(description: str) → AgentUpdate`
- `with_capabilities(capabilities: List[str]) → AgentUpdate`
- `with_specializations(specializations: List[str]) → AgentUpdate`
- `with_status(status: AgentStatus) → AgentUpdate`

#### `AgentStatistics`
```python
@dataclass
class AgentStatistics:
    total_agents: int
    available_agents: int
    busy_agents: int
    inactive_agents: int
    total_assignments: int
    completed_assignments: int
```

**Computation Method**:
```python
def completion_rate(self) -> float:
    """
    Calculate task completion rate as percentage.
    
    Returns:
        Completion percentage (0.0-100.0) or 0.0 if no assignments
    """
```

## 4. Storage Layer

### 4.1 File-Based JSON Storage

**Configuration**:
- File path: `agents.json`
- Encoding: UTF-8
- Format: JSON array of agent objects
- Atomic writes: Temporary file + atomic replace

### 4.2 Core Storage Functions

#### `json_file_transaction(contextmanager)`
```python
@contextmanager
def json_file_transaction(path: str):
    """
    Context manager for atomic JSON file operations.
    
    Strategy:
    1. Create temporary file (.tmp extension)
    2. Yield temporary file path for writing
    3. Atomically replace original file
    4. Clean up temporary file on failure
    
    Args:
        path: Target file path
        
    Raises:
        Exception: Propagates any file operation errors
    """
```

#### `_load_json_list(path: str) → List[Dict[str, Any]]`
```python
def _load_json_list(path: str) -> List[Dict[str, Any]]:
    """
    Load JSON data from file, returning empty list if file doesn't exist.
    
    Args:
        path: JSON file path
        
    Returns:
        List of dictionaries or empty list
        
    Raises:
        JSONDecodeError: If file contains invalid JSON
    """
```

#### `_save_json_list(path: str, data: List[Dict[str, Any]]) → None`
```python
def _save_json_list(path: str, data: List[Dict[str, Any]]) -> None:
    """
    Save data to JSON file with atomic transaction.
    
    Args:
        path: Target file path
        data: Data to serialize as JSON
        
    Raises:
        IOError: If file operations fail
    """
```

### 4.3 Agent-Specific Storage Functions

#### `create_default_agents() → None`
```python
def create_default_agents() -> None:
    """
    Initialize empty agents file if it doesn't exist.
    Safe to call multiple times.
    """
```

#### `list_agents() → List[Agent]`
```python
def list_agents() -> List[Agent]:
    """
    Load all agents from persistent storage.
    
    Returns:
        List of Agent objects
        Empty list if no agents exist
        
    Error Handling:
        - Handles missing status values gracefully
        - Defaults to AVAILABLE status on parsing errors
    """
```

#### `save_agent(agent: Agent) → None`
```python
def save_agent(agent: Agent) -> None:
    """
    Save agent to storage (create or update).
    
    Args:
        agent: Agent instance to persist
        
    Behavior:
        - Updates existing agent if ID matches
        - Appends new agent if ID not found
        - Performs atomic file operation
    """
```

## 5. AgentManager Class

### 5.1 Initialization

```python
class AgentManager:
    def __init__(self) -> None:
        self.agents: Dict[str, Agent] = {}           # In-memory agent cache
        self.agent_assignments: List[AgentAssignment] = []  # Assignment tracking
```

### 5.2 Core Methods

#### `async load_agents() → None`
```python
async def load_agents(self) -> None:
    """
    Initialize manager by loading agents from storage.
    
    Behavior:
        - Creates default storage file if needed
        - Populates in-memory cache
        - Safe to call multiple times (checks cache first)
    """
```

#### `async create_agent(agent: Agent) → str`
```python
async def create_agent(self, agent: Agent) -> str:
    """
    Create new agent with auto-generated UUID.
    
    Args:
        agent: Agent template (ID will be overwritten)
        
    Returns:
        Generated agent ID
        
    Side Effects:
        - Updates in-memory cache
        - Persists to storage
        - Sets creation/update timestamps to current UTC
    """
```

#### `async update_agent(agent_id: str, updates: AgentUpdate) → None`
```python
async def update_agent(self, agent_id: str, updates: AgentUpdate) -> None:
    """
    Update agent properties using partial update pattern.
    
    Args:
        agent_id: Target agent identifier
        updates: Partial update specification
        
    Raises:
        TodoziError: If agent not found
        
    Behavior:
        - Only updates specified fields (None values ignored)
        - Updates timestamp to current UTC
        - Persists changes to storage
    """
```

#### `async delete_agent(agent_id: str) → None`
```python
async def delete_agent(self, agent_id: str) -> None:
    """
    Remove agent from system.
    
    Args:
        agent_id: Agent to remove
        
    Raises:
        TodoziError: If agent not found
        
    Note: Does not remove agent assignments (historical tracking)
    """
```

### 5.3 Query Methods

#### `get_agent(agent_id: str) → Optional[Agent]`
```python
def get_agent(self, agent_id: str) -> Optional[Agent]:
    """Retrieve agent by ID, returns None if not found."""
```

#### `get_all_agents() → List[Agent]`
```python
def get_all_agents(self) -> List[Agent]:
    """Get all agents regardless of status."""
```

#### `get_available_agents() → List[Agent]`
```python
def get_available_agents(self) -> List[Agent]:
    """Get agents with AVAILABLE status for task assignment."""
```

#### `get_agents_by_specialization(specialization: str) → List[Agent]`
```python
def get_agents_by_specialization(self, specialization: str) -> List[Agent]:
    """Filter agents by specialization (exact match)."""
```

#### `get_agents_by_capability(capability: str) → List[Agent]`
```python
def get_agents_by_capability(self, capability: str) -> List[Agent]:
    """Filter agents by capability (exact match)."""
```

### 5.4 Assignment Management

#### `async assign_task_to_agent(task_id: str, agent_id: str, project_id: str) → str`
```python
async def assign_task_to_agent(self, task_id: str, agent_id: str, project_id: str) -> str:
    """
    Assign task to available agent.
    
    Args:
        task_id: Unique task identifier
        agent_id: Target agent ID
        project_id: Project context
        
    Returns:
        Assigned task_id
        
    Raises:
        TodoziError: If agent not found or not available
        
    Side Effects:
        - Updates agent status to BUSY
        - Creates assignment record
        - Updates agent timestamp
    """
```

#### `async complete_agent_assignment(task_id: str) → None`
```python
async def complete_agent_assignment(self, task_id: str) -> None:
    """
    Mark assignment as completed and free agent.
    
    Args:
        task_id: Task to complete
        
    Raises:
        TodoziError: If assignment not found
        
    Behavior:
        - Updates assignment status to COMPLETED
        - Sets agent status to AVAILABLE
        - Updates agent timestamp
    """
```

### 5.5 Advanced Features

#### `find_best_agent(required_specialization: str, preferred_capability: Optional[str]) → Optional[Agent]`
```python
def find_best_agent(self, required_specialization: str, preferred_capability: Optional[str] = None) -> Optional[Agent]:
    """
    Intelligent agent selection based on requirements.
    
    Selection Criteria:
    1. Must have required specialization
    2. Must be AVAILABLE
    3. Priority given to agents with preferred capability
    4. Secondary: agents with more capabilities
    5. Tertiary: consistent ordering by ID
    
    Returns:
        Best matching agent or None if no candidates
    """
```

#### `get_agent_statistics() → AgentStatistics`
```python
def get_agent_statistics(self) -> AgentStatistics:
    """
    Generate comprehensive system statistics.
    
    Metrics:
        - Agent counts by status
        - Assignment counts by completion status
        - Supports completion rate calculation
    """
```

## 6. Parser Module

### `parse_agent_assignment_format(agent_text: str) → AgentAssignment`
```python
def parse_agent_assignment_format(agent_text: str) -> AgentAssignment:
    """
    Parse XML-like format for agent assignments.
    
    Expected Format:
        <todozi_agent>agent_id; task_id; project_id</todozi_agent>
    
    Args:
        agent_text: Text containing assignment markup
        
    Returns:
        Parsed AgentAssignment with current UTC timestamp
        
    Raises:
        TodoziError: On format violations or missing tags
    """
```

## 7. Error Handling

### 7.1 Custom Exception

#### `TodoziError(Exception)`
```python
class TodoziError(Exception):
    def __init__(self, message: str):
        self.message = message
        super().__init__(self.message)
```

### 7.2 Error Scenarios

| Scenario | Error Type | Handling Strategy |
|----------|------------|------------------|
| Agent not found | `TodoziError` | Validate existence before operations |
| Invalid assignment format | `TodoziError` | Comprehensive format validation |
| File I/O errors | Built-in exceptions | Atomic transaction rollback |
| JSON parsing errors | `JSONDecodeError` | Graceful fallback to empty data |

## 8. Performance Considerations

### 8.1 Optimizations
- **In-Memory Caching**: Agents cached after initial load
- **Lazy Loading**: Agents loaded only when needed
- **Efficient Queries**: List comprehensions for filtering
- **Atomic File Operations**: Prevents partial writes

### 8.2 Limitations
- **Memory Usage**: All agents loaded into memory
- **File-Based Storage**: Not suitable for high-concurrency scenarios
- **Linear Search**: O(n) complexity for assignment lookups

## 9. Dependencies and Requirements

### 9.1 Python Dependencies
```python
# Standard Library Dependencies
import copy
import json
import os
import uuid
import sys
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime, timezone
from enum import Enum
from typing import Dict, List, Optional, Tuple, Any, Iterable
```

### 9.2 System Requirements
- **Python Version**: 3.7+ (for `from __future__ import annotations`)
- **File System**: Write permissions for current directory
- **Encoding Support**: UTF-8 file handling

## 10. Testing

### 10.1 Test Coverage
```python
def test_agent_manager_creation():
    """Verify manager initialization creates empty state."""

def test_parse_agent_assignment_format():
    """Validate XML-like assignment parsing."""

def test_agent_update_builder():
    """Test fluent interface for agent updates."""

def test_agent_statistics_completion_rate():
    """Verify completion rate calculation handles edge cases."""
```

### 10.2 Test Execution
```bash
python todozi_agents.py  # Runs all validation tests
```

## 11. Usage Examples

### 11.1 Basic Agent Management
```python
manager = AgentManager()
await manager.load_agents()

# Create new agent
agent = Agent.new("Research Assistant", "Handles research tasks")
agent_id = await manager.create_agent(agent)

# Update agent capabilities
update = AgentUpdate().with_capabilities(["research", "analysis"])
await manager.update_agent(agent_id, update)
```

### 11.2 Task Assignment Workflow
```python
# Find best agent for research task
best_agent = manager.find_best_agent("research", "analysis")
if best_agent:
    task_id = await manager.assign_task_to_agent("task_123", best_agent.id, "project_x")
    
    # Later: mark as completed
    await manager.complete_agent_assignment(task_id)
```

### 11.3 Monitoring and Statistics
```python
stats = manager.get_agent_statistics()
print(f"Completion Rate: {stats.completion_rate():.1f}%")
print(f"Available Agents: {stats.available_agents}/{stats.total_agents}")
```

## 12. Technical Constraints

### 12.1 Scalability Limits
- Maximum agents: Limited by available memory
- Concurrent access: Not thread-safe (single-writer pattern)
- File size: JSON file grows linearly with agent count

### 12.2 Data Consistency
- **Single Writer**: File-based storage requires serialized access
- **No Transactions**: Individual agent operations are atomic, but bulk operations aren't
- **Eventual Consistency**: In-memory cache may lag behind file changes

### 12.3 Extension Points
- **Storage Backend**: Replaceable JSON implementation
- **Assignment Algorithms**: Customizable agent selection logic
- **Notification System**: Hook for assignment lifecycle events

This documentation provides comprehensive coverage of the Todozi Agent Management System architecture, implementation details, and usage patterns suitable for developers and technical stakeholders.