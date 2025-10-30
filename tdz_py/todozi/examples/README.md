# Todozi Python Examples

This directory contains comprehensive examples demonstrating all the functionality of the Todozi Python bindings.

## Getting Started

First, make sure you have Todozi installed:

```bash
pip install todozi
```

## Examples Overview

### 01_basic_task_creation.py
**Basic Task Creation**
- Create tasks with different priority levels (urgent, high, medium, low)
- Demonstrates the `task()`, `urgent()`, `high()`, and `low()` methods

### 02_ai_assignments.py
**AI Assignment Features**
- Assign tasks to AI, humans, or collaborative teams
- Shows `ai_task()`, `human_task()`, and `collab_task()` methods

### 03_task_management.py
**Task Management Operations**
- Find, start, and complete tasks
- Demonstrates `find()`, `start()`, `done()`, and `all()` methods

### 04_projects.py
**Project Management**
- Create and manage projects
- Set project context for task organization
- Shows `create_project()`, `list_projects()`, `project_tasks()`, and `set_project()`

### 05_memories_and_ideas.py
**Memories and Ideas**
- Capture learning experiences and creative ideas
- Demonstrates `remember()`, `idea()`, `create_memory()`, `create_idea()`, `list_memories()`, and `list_ideas()`

### 06_search_functionality.py
**Search Functionality**
- Keyword-based search with `find()`
- AI-powered semantic search with `ai_find()`
- Shows the difference between exact matching and intelligent search

### 07_statistics.py
**Statistics and Analytics**
- Get basic and detailed task statistics
- Analyze task status and priority distributions
- Shows `stats()` and `detailed_stats()` methods

### 08_task_lifecycle.py
**Complete Task Lifecycle**
- Full workflow from creation to completion
- Demonstrates realistic task management scenarios
- Shows integration of multiple Todozi features

### 09_batch_operations.py
**Batch Operations**
- Create multiple tasks at once
- Bulk priority assignment and status updates
- Project-based batch operations

### 10_advanced_features.py
**Advanced Features and Integrations**
- Complex project breakdown
- Knowledge management integration
- Workflow automation concepts
- Data export/import patterns

## Running the Examples

Each example can be run individually:

```bash
python examples/01_basic_task_creation.py
python examples/02_ai_assignments.py
# ... etc
```

## TodoziClient API Reference

### Core Methods
- `task(action)` - Create a regular priority task
- `urgent(action)` - Create an urgent priority task
- `high(action)` - Create a high priority task
- `low(action)` - Create a low priority task

### Task Management
- `find(query)` - Search tasks by keywords
- `ai_find(query)` - AI-powered semantic search
- `start(task_id)` - Start working on a task
- `done(task_id)` - Mark task as completed
- `all()` - Get all tasks

### AI Assignments
- `ai_task(action)` - Create AI-assigned task
- `human_task(action)` - Create human-assigned task
- `collab_task(action)` - Create collaborative task

### Projects
- `create_project(name, description)` - Create a new project
- `list_projects()` - List all projects
- `project_tasks(project_name)` - Get tasks for a project
- `set_project(project_name)` - Set current project context

### Knowledge Management
- `remember(moment, meaning)` - Capture a learning experience
- `idea(content)` - Capture a creative idea
- `create_memory(moment, meaning, reason)` - Create structured memory
- `create_idea(content)` - Create structured idea
- `list_memories()` - Get all memories
- `list_ideas()` - Get all ideas

### Analytics
- `stats()` - Get basic statistics
- `detailed_stats()` - Get detailed analytics

## Data Persistence

All data is automatically persisted to the local file system. Todozi uses a sophisticated file-based storage system that maintains data integrity and supports concurrent access.

## Integration Patterns

These examples demonstrate common integration patterns:
- **Batch processing** for handling multiple tasks
- **Project-based organization** for complex workflows
- **Knowledge capture** during development processes
- **AI-assisted workflows** for enhanced productivity
- **Analytics-driven insights** for process improvement

## Contributing

Feel free to add more examples or improve existing ones! The examples should demonstrate real-world usage patterns and best practices for Todozi integration.
