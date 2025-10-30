# Todozi Python Library

Todozi is an AI-human collaborative task management system that combines the power of artificial intelligence with human intuition to help you organize, prioritize, and accomplish tasks more effectively.

This Python library provides bindings to the high-performance Rust implementation of Todozi, offering a professional Python interface to all core functionality.

## 🚀 Key Features

- **AI-Powered Task Management**: Intelligent task creation, prioritization, and organization
- **Semantic Search**: Find tasks using natural language and contextual understanding
- **Project Organization**: Group related tasks into logical project structures
- **Memory & Idea Capture**: Record important insights and creative thoughts
- **Collaborative Workflow**: Support for both human and AI task execution
- **High Performance**: Built on Rust for speed and reliability

## 📦 Installation

```bash
pip install todozi
```

Or for development installation:

```bash
pip install -e .
```

## 🛠️ Quick Start

```python
from todozi import TodoziClient

# Initialize the Todozi client
tdz = TodoziClient()

# Create tasks with different priority levels
task_id = tdz.task("Review the quarterly budget report")
urgent_id = tdz.urgent("Fix critical security vulnerability")
high_id = tdz.high("Prepare presentation for board meeting")
low_id = tdz.low("Update README documentation")

# Find tasks using keywords
api_tasks = tdz.find("API")
print(f"Found {len(api_tasks)} tasks containing 'API'")

# Start and complete tasks
tdz.start(task_id)
tdz.done(task_id)

# List all tasks
all_tasks = tdz.all()
for task in all_tasks:
    print(f"{task.action} - Status: {task.status}")
```

## 📚 Core Functionality

### Task Management

Create tasks with different priority levels:

```python
# Basic task creation
task_id = tdz.task("Write unit tests")

# Priority-specific task creation
urgent_task = tdz.urgent("Fix critical bug")
high_task = tdz.high("Implement new feature")
low_task = tdz.low("Update documentation")
```

Manage task lifecycle:

```python
# Start working on a task
tdz.start(task_id)

# Mark task as completed
tdz.done(task_id)

# Find tasks by keyword
matching_tasks = tdz.find("bug")

# Get all tasks
all_tasks = tdz.all()
```

### Project Management

Organize tasks into projects:

```python
# Create a project
project_id = tdz.create_project("Mobile App", "Building the next-gen mobile app")

# Set current project context
tdz.set_project("Mobile App")

# Create tasks within the current project
task_id = tdz.task("Design user authentication flow")

# Get tasks for a specific project
project_tasks = tdz.project_tasks("Mobile App")

# List all projects
projects = tdz.list_projects()
```

### Memory & Idea Capture

Record important information:

```python
# Capture memories (learning experiences)
memory_id = tdz.remember(
    "First time deploying to production",
    "Learned that environment variables must be set before app startup"
)

# Create structured memories
memory_id = tdz.create_memory(
    "Code review feedback",
    "Always check for null pointer exceptions",
    "Improves code reliability"
)

# Capture ideas
idea_id = tdz.idea("Implement dark mode toggle")

# Create structured ideas
idea_id = tdz.create_idea("Create a mobile app companion")

# List memories and ideas
memories = tdz.list_memories()
ideas = tdz.list_ideas()
```

### Advanced Search

Find information using various search methods:

```python
# Keyword search
tasks = tdz.find("authentication")

# AI-powered semantic search
similar_tasks = tdz.ai_find("login related issues")

# Get all tasks
all_tasks = tdz.all()
```

### AI Integration

Leverage AI capabilities:

```python
# Chat with Todozi AI
response = tdz.chat("Help me organize my tasks by priority")

# AI-powered task breakdown
complex_task = tdz.task("Build complete e-commerce platform")
# AI would help break this down into subtasks
```

## 🏗️ Data Classes

The library provides Python classes for working with Todozi data:

- `Task`: Task information (id, action, priority, status, etc.)
- `Memory`: Memory information (id, moment, meaning, importance, etc.)
- `Idea`: Idea information (id, content, importance, etc.)
- `Project`: Project information (name, description, status, etc.)

## 📖 Examples

Check the `examples/` directory for comprehensive usage examples:

1. `01_basic_task_creation.py` - Creating tasks with different priorities
2. `02_ai_assignments.py` - Working with AI-assigned tasks
3. `03_task_management.py` - Finding, starting, and completing tasks
4. `04_projects.py` - Project organization and management
5. `05_memories_and_ideas.py` - Capturing memories and ideas
6. `06_search_functionality.py` - Using search features
7. `07_statistics.py` - Getting system statistics
8. `08_task_lifecycle.py` - Complete task workflow
9. `09_batch_operations.py` - Working with multiple tasks
10. `10_advanced_features.py` - Advanced capabilities
11. `11_error_handling.py` - Error management
12. `12_dev_team_workflow.py` - Team collaboration patterns

## ⚙️ Requirements

- Python 3.7+
- Rust toolchain (for building from source)

## 🧪 Running Examples

```bash
cd examples
python 01_basic_task_creation.py
```

## 📄 License

Todozi is licensed under the MIT License. See the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please see the CONTRIBUTING.md file for guidelines.

## 🆘 Support

For issues, questions, or feedback, please open a GitHub issue or contact the maintainers.
