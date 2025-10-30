# Todozi: AI/Human Task Management System

## Overview

Todozi is a simple, file-based task management system designed for AI/Human collaboration. It uses JSON files stored in `~/.todozi/` for persistence and provides both a CLI binary and a Rust library for integration into other projects.

## Core Design Principles

1. **Simplicity**: File-based storage using JSON, no database dependencies
2. **Portability**: Works standalone and as a library in Cargo projects
3. **AI-Friendly**: Structured format optimized for AI model training and inference
4. **Human-Friendly**: Simple CLI interface for direct interaction
5. **Extensible**: Support for both v1.0.0 (5 fields) and v1.2.0 (10 fields) formats

## Data Model

### Task Structure (v1.2.0)
```rust
pub struct Task {
    pub id: String,                    // UUID or timestamp-based ID
    pub action: String,                // Required: Task description
    pub time: String,                  // Required: Time estimate/deadline
    pub priority: Priority,            // Required: low|medium|high|critical|urgent
    pub parent_project: String,        // Required: Project context
    pub status: Status,                // Required: todo|in_progress|blocked|review|done|cancelled|deferred
    pub assignee: Option<String>,      // Optional: AI|human|collaborative
    pub tags: Vec<String>,             // Optional: Comma-separated labels
    pub dependencies: Vec<String>,     // Optional: Task IDs this depends on
    pub context_notes: Option<String>, // Optional: Additional context
    pub progress: Option<u8>,          // Optional: Progress percentage (0-100)
    pub created_at: DateTime<Utc>,     // Auto-generated
    pub updated_at: DateTime<Utc>,     // Auto-updated
}
```

### Enums
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
    Urgent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Blocked,
    Review,
    Done,
    Cancelled,
    Deferred,
}
```

## File Structure

```
~/.todozi/
├── config.json              # Global configuration
├── tasks/
│   ├── active.json          # Active tasks
│   ├── completed.json       # Completed tasks
│   └── archived.json        # Archived/cancelled tasks
├── projects/
│   ├── {project_name}.json  # Project-specific task files
│   └── ...
└── templates/
    ├── v1.0.0.json          # Legacy format template
    └── v1.2.0.json          # Current format template
```

## CLI Commands

### Basic Operations
- `todozi add "action" --time "2 hours" --priority high --project "my-project"` - Add new task
- `todozi list [--project PROJECT] [--status STATUS] [--priority PRIORITY]` - List tasks
- `todozi show <id>` - Show task details
- `todozi update <id> --status in_progress` - Update task
- `todozi delete <id>` - Delete task
- `todozi complete <id>` - Mark task as done

### Project Management
- `todozi project create <name>` - Create new project
- `todozi project list` - List all projects
- `todozi project show <name>` - Show project details
- `todozi project archive <name>` - Archive project

### AI/Human Collaboration
- `todozi assign <id> --assignee ai` - Assign task to AI
- `todozi assign <id> --assignee human` - Assign task to human
- `todozi assign <id> --assignee collaborative` - Mark for collaboration
- `todozi suggest` - AI suggests next tasks based on context

### Data Management
- `todozi export [--format json|csv]` - Export tasks
- `todozi import <file>` - Import tasks
- `todozi backup` - Create backup
- `todozi restore <backup>` - Restore from backup

### Advanced Features
- `todozi dependencies <id>` - Show task dependencies
- `todozi timeline` - Show task timeline
- `todozi stats` - Show statistics
- `todozi search <query>` - Search tasks

## Library API

### Core Functions
```rust
// Task management
pub fn add_task(task: Task) -> Result<String, TodoziError>
pub fn get_task(id: &str) -> Result<Task, TodoziError>
pub fn update_task(id: &str, updates: TaskUpdate) -> Result<(), TodoziError>
pub fn delete_task(id: &str) -> Result<(), TodoziError>
pub fn list_tasks(filters: TaskFilters) -> Result<Vec<Task>, TodoziError>

// Project management
pub fn create_project(name: &str) -> Result<(), TodoziError>
pub fn get_project_tasks(project: &str) -> Result<Vec<Task>, TodoziError>
pub fn archive_project(name: &str) -> Result<(), TodoziError>

// AI collaboration
pub fn suggest_next_tasks(context: &str) -> Result<Vec<Task>, TodoziError>
pub fn get_ai_tasks() -> Result<Vec<Task>, TodoziError>
pub fn get_human_tasks() -> Result<Vec<Task>, TodoziError>

// Data operations
pub fn export_tasks(format: ExportFormat) -> Result<String, TodoziError>
pub fn import_tasks(data: &str, format: ImportFormat) -> Result<(), TodoziError>
pub fn backup_data() -> Result<String, TodoziError>
```

## Implementation Plan

### Phase 1: Core Library (Week 1)
1. **Data Models**: Define Task, Priority, Status enums and structs
2. **File Operations**: JSON serialization/deserialization
3. **Storage Layer**: File-based CRUD operations
4. **Error Handling**: Comprehensive error types
5. **Validation**: Input validation based on JSON schema

### Phase 2: CLI Binary (Week 2)
1. **CLI Framework**: Using clap for argument parsing
2. **Basic Commands**: add, list, show, update, delete
3. **Project Commands**: create, list, show, archive
4. **Configuration**: Global config management
5. **Interactive Mode**: TUI for better UX

### Phase 3: Advanced Features (Week 3)
1. **AI Integration**: Task suggestion algorithms
2. **Dependencies**: Task dependency tracking
3. **Search & Filtering**: Advanced query capabilities
4. **Import/Export**: Multiple format support
5. **Backup/Restore**: Data management tools

### Phase 4: Integration & Testing (Week 4)
1. **Library Integration**: Cargo workspace setup
2. **Unit Tests**: Comprehensive test coverage
3. **Integration Tests**: End-to-end testing
4. **Documentation**: API docs and examples
5. **Performance**: Optimization and benchmarking

## File Format Examples

### Task File (active.json)
```json
{
  "version": "1.2.0",
  "created_at": "2025-01-13T10:00:00Z",
  "updated_at": "2025-01-13T10:00:00Z",
  "tasks": [
    {
      "id": "task_001",
      "action": "Implement OAuth2 login flow",
      "time": "6 hours",
      "priority": "high",
      "parent_project": "python-web-framework",
      "status": "todo",
      "assignee": "human",
      "tags": ["auth", "backend"],
      "dependencies": [],
      "context_notes": "Ensure coverage for edge cases like declined cards",
      "progress": 0,
      "created_at": "2025-01-13T10:00:00Z",
      "updated_at": "2025-01-13T10:00:00Z"
    }
  ]
}
```

### Project File (python-web-framework.json)
```json
{
  "name": "python-web-framework",
  "description": "Modern Python web framework",
  "created_at": "2025-01-13T10:00:00Z",
  "updated_at": "2025-01-13T10:00:00Z",
  "status": "active",
  "tasks": ["task_001", "task_002"]
}
```

### Configuration File (config.json)
```json
{
  "version": "1.2.0",
  "default_project": "general",
  "auto_backup": true,
  "backup_interval": "daily",
  "ai_enabled": true,
  "default_assignee": "collaborative",
  "date_format": "%Y-%m-%d %H:%M:%S",
  "timezone": "UTC"
}
```

## Dependencies

### Core Dependencies
- `serde` + `serde_json`: Serialization
- `uuid`: Task ID generation
- `chrono`: Date/time handling
- `thiserror`: Error handling
- `anyhow`: Error context

### CLI Dependencies
- `clap`: Argument parsing
- `tabled`: Table formatting
- `dialoguer`: Interactive prompts
- `indicatif`: Progress bars

### Optional Dependencies
- `tokio`: Async runtime (for future async features)
- `reqwest`: HTTP client (for AI API integration)
- `sqlx`: Database support (for future SQLite option)

## Error Handling

```rust
#[derive(thiserror::Error, Debug)]
pub enum TodoziError {
    #[error("Task not found: {id}")]
    TaskNotFound { id: String },
    
    #[error("Project not found: {name}")]
    ProjectNotFound { name: String },
    
    #[error("Invalid priority: {priority}")]
    InvalidPriority { priority: String },
    
    #[error("Invalid status: {status}")]
    InvalidStatus { status: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Validation error: {message}")]
    ValidationError { message: String },
}
```

## Future Enhancements

1. **Database Support**: Optional SQLite backend
2. **Web Interface**: Simple web UI for remote access
3. **AI Integration**: Direct AI model integration
4. **Team Collaboration**: Multi-user support
5. **Plugin System**: Extensible architecture
6. **Mobile App**: Companion mobile application
7. **API Server**: REST API for external integration

## Getting Started

1. **Install**: `cargo install todozi`
2. **Initialize**: `todozi init`
3. **Add Task**: `todozi add "Learn Rust" --time "2 hours" --priority high --project "learning"`
4. **List Tasks**: `todozi list`
5. **Complete**: `todozi complete <task-id>`

This design provides a solid foundation for a simple yet powerful task management system that can grow with your needs while maintaining simplicity and ease of use.
