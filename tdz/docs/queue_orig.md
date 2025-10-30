# Todozi Queue System

A powerful task queue management system designed for AI/Human collaboration with time tracking, session management, and priority-based workflow organization.

## 🎯 Overview

The Todozi Queue System provides a structured approach to task management with three distinct states:
- **Backlog**: Tasks waiting to be started
- **Active**: Tasks currently being worked on
- **Complete**: Finished tasks

## 🚀 Quick Start

### Initialize the System
```bash
todozi init
```

### Plan Your First Task
```bash
todozi queue plan --task-name "Implement user authentication" --task-description "Add OAuth2 login flow with JWT tokens" --priority high --project-id "web-app"
```

### Start Working
```bash
todozi queue start <queue_item_id>
```

### Finish Work
```bash
todozi queue end <session_id>
```

## 📋 CLI Commands

### Plan New Queue Items

Create a new task in the backlog:

```bash
# Basic task
todozi queue plan --task-name "Task Name" --task-description "Detailed description"

# With priority and project
todozi queue plan \
  --task-name "Database optimization" \
  --task-description "Optimize queries and add indexes" \
  --priority high \
  --project-id "backend"
```

**Parameters:**
- `--task-name` (required): Short, descriptive task name
- `--task-description` (required): Detailed task description
- `--priority` (optional): Priority level (low, medium, high, critical, urgent) - defaults to medium
- `--project-id` (optional): Associate with a project

### List Queue Items

View your queue in different ways:

```bash
# List all items
todozi queue list

# Filter by status
todozi queue list --status backlog
todozi queue list --status active
todozi queue list --status complete

# Quick status views
todozi queue backlog    # Show only backlog items
todozi queue active     # Show only active items
todozi queue complete   # Show only completed items
```

### Work Session Management

Start and end work sessions to track time:

```bash
# Start working on a backlog item
todozi queue start queue_abc123

# End the work session
todozi queue end session_xyz789
```

## 🤖 AI Integration

### For AI Assistants

The queue system is designed for seamless AI collaboration:

#### Planning Tasks
```bash
# AI can plan tasks with detailed descriptions
todozi queue plan \
  --task-name "Code review for PR #123" \
  --task-description "Review the authentication middleware changes, check for security vulnerabilities, verify error handling, and ensure proper logging" \
  --priority high \
  --project-id "security-audit"
```

#### Starting Work
```bash
# AI starts work on planned tasks
todozi queue start queue_abc123
# Returns: session_xyz789
```

#### Completing Work
```bash
# AI ends work session
todozi queue end session_xyz789
# Shows duration and completion details
```

### AI Workflow Examples

#### Code Development Cycle
```bash
# 1. Plan the task
todozi queue plan \
  --task-name "Implement user authentication" \
  --task-description "Create OAuth2 flow with JWT tokens, password hashing, and session management" \
  --priority high

# 2. Start development
todozi queue start queue_auth001

# 3. Complete and move to next task
todozi queue end session_dev001
```

#### Code Review Process
```bash
# 1. Plan review task
todozi queue plan \
  --task-name "Review authentication PR" \
  --task-description "Review PR #456 for security issues, code quality, and test coverage" \
  --priority medium

# 2. Start review
todozi queue start queue_review001

# 3. Complete review
todozi queue end session_review001
```

## 🌐 REST API

### Server Endpoints

Start the server:
```bash
todozi server start --host 127.0.0.1 --port 8636
```

### Queue API Routes

#### Plan New Queue Item
```http
POST /queue/plan
Content-Type: application/json

{
  "task_name": "Implement user authentication",
  "task_description": "Add OAuth2 login flow with JWT tokens",
  "priority": "high",
  "project_id": "web-app"
}
```

#### List Queue Items
```http
# All items
GET /queue/list

# By status
GET /queue/list/backlog
GET /queue/list/active
GET /queue/list/complete
```

#### Start Work Session
```http
POST /queue/start/queue_abc123
```

#### End Work Session
```http
POST /queue/end/session_xyz789
```

### API Response Examples

#### Successful Queue Item Creation
```json
{
  "message": "Queue item created successfully",
  "item": {
    "id": "queue_abc123",
    "task_name": "Implement user authentication",
    "task_description": "Add OAuth2 login flow with JWT tokens",
    "priority": "high",
    "project_id": "web-app",
    "status": "backlog",
    "created_at": "2025-01-13T10:30:00Z"
  }
}
```

#### Session Started
```json
{
  "message": "Queue session started successfully",
  "session_id": "session_xyz789",
  "queue_item_id": "queue_abc123",
  "started_at": "2025-01-13T10:30:00Z"
}
```

#### Session Ended
```json
{
  "message": "Queue session ended successfully",
  "session_id": "session_xyz789",
  "queue_item_id": "queue_abc123",
  "start_time": "2025-01-13T10:30:00Z",
  "end_time": "2025-01-13T12:45:00Z",
  "duration_seconds": 8100
}
```

## 📊 Data Models

### Queue Item Structure
```json
{
  "id": "queue_abc123",
  "task_name": "Task Name",
  "task_description": "Detailed description",
  "priority": "high",
  "project_id": "optional-project-id",
  "status": "backlog",
  "created_at": "2025-01-13T10:30:00Z",
  "updated_at": "2025-01-13T10:30:00Z"
}
```

### Session Structure
```json
{
  "id": "session_xyz789",
  "queue_item_id": "queue_abc123",
  "start_time": "2025-01-13T10:30:00Z",
  "end_time": "2025-01-13T12:45:00Z",
  "duration_seconds": 8100,
  "created_at": "2025-01-13T10:30:00Z",
  "updated_at": "2025-01-13T12:45:00Z"
}
```

## 🔄 Workflow States

### Backlog → Active → Complete

1. **Backlog**: Task is planned and waiting to be started
2. **Active**: Task is currently being worked on (has active session)
3. **Complete**: Task is finished (session ended)

### State Transitions

```
Backlog ──start──> Active ──end──> Complete
   ↑                    │
   └── (new task)       └── (can restart if needed)
```

## 💡 Best Practices

### For Humans

1. **Plan Before Starting**: Always plan tasks with clear descriptions
2. **Use Descriptive Names**: Make task names searchable and clear
3. **Set Appropriate Priorities**: Use priority levels to organize work
4. **Track Time Accurately**: Start and end sessions promptly
5. **Review Completed Work**: Use the complete list to review finished tasks

### For AI Assistants

1. **Detailed Descriptions**: Provide comprehensive task descriptions
2. **Context-Rich Planning**: Include relevant context and requirements
3. **Consistent Naming**: Use consistent naming conventions
4. **Priority Management**: Set priorities based on urgency and importance
5. **Session Management**: Always start and end sessions properly

### For Teams

1. **Project Organization**: Use project IDs to group related tasks
2. **Priority Coordination**: Coordinate priorities across team members
3. **Regular Reviews**: Review backlog and complete items regularly
4. **Time Tracking**: Use session data for time estimation and billing
5. **Status Communication**: Use status lists for team updates

## 📈 Analytics and Reporting

### Time Tracking
- Session duration tracking
- Total time per task
- Time distribution by priority
- Productivity metrics

### Queue Health
- Backlog size monitoring
- Active task count
- Completion rates
- Priority distribution

### Project Insights
- Tasks per project
- Time spent per project
- Project completion rates
- Priority distribution by project

## 🔧 Configuration

### Storage Location
Queue data is stored in `~/.todozi/queue/queue.json`

### File Structure
```
~/.todozi/queue/
└── queue.json          # Main queue collection
```

### Backup and Restore
```bash
# Create backup
todozi backup

# Restore from backup
todozi restore backup_name
```

## 🚨 Error Handling

### Common Errors

#### Item Not Found
```
❌ Queue item not found: queue_invalid123
```

#### Session Already Ended
```
❌ Session is already ended
```

#### Invalid Status Transition
```
❌ Item is not in backlog status
```

### Troubleshooting

1. **Check Item Status**: Use `todozi queue list` to see current status
2. **Verify IDs**: Ensure you're using correct queue item and session IDs
3. **Check Permissions**: Ensure you have write access to `~/.todozi/`
4. **Validate Priority**: Use valid priority levels (low, medium, high, critical, urgent)

## 🎯 Use Cases

### Development Workflow
```bash
# Plan development tasks
todozi queue plan --task-name "Feature: User Dashboard" --task-description "Create user dashboard with analytics, settings, and profile management" --priority high

# Start development
todozi queue start queue_dashboard001

# Complete and plan next
todozi queue end session_dev001
```

### Code Review Process
```bash
# Plan review tasks
todozi queue plan --task-name "Review PR #123" --task-description "Review authentication changes for security and code quality" --priority medium

# Start review
todozi queue start queue_review001

# Complete review
todozi queue end session_review001
```

### Bug Fixing
```bash
# Plan bug fixes
todozi queue plan --task-name "Fix login bug" --task-description "Fix issue where users can't login with special characters in password" --priority critical

# Start fixing
todozi queue start queue_bug001

# Complete fix
todozi queue end session_bug001
```

### Documentation
```bash
# Plan documentation tasks
todozi queue plan --task-name "API Documentation" --task-description "Write comprehensive API documentation with examples" --priority medium

# Start writing
todozi queue start queue_docs001

# Complete documentation
todozi queue end session_docs001
```

## 🔗 Integration Examples

### With CI/CD
```bash
# Plan deployment tasks
todozi queue plan --task-name "Deploy v1.2.0" --task-description "Deploy new version to production with database migrations" --priority high

# Start deployment
todozi queue start queue_deploy001

# Complete deployment
todozi queue end session_deploy001
```

### With Testing
```bash
# Plan testing tasks
todozi queue plan --task-name "Integration Tests" --task-description "Write integration tests for new authentication flow" --priority medium

# Start testing
todozi queue start queue_test001

# Complete testing
todozi queue end session_test001
```

## 📚 Advanced Features

### Batch Operations
```bash
# Plan multiple related tasks
todozi queue plan --task-name "Frontend Components" --task-description "Create reusable UI components" --priority medium
todozi queue plan --task-name "Backend API" --task-description "Implement REST API endpoints" --priority high
todozi queue plan --task-name "Database Schema" --task-description "Design and implement database schema" --priority high
```

### Project Organization
```bash
# Organize by project
todozi queue plan --task-name "User Auth" --task-description "Implement user authentication" --priority high --project-id "auth-system"
todozi queue plan --task-name "Payment Processing" --task-description "Integrate payment gateway" --priority high --project-id "payment-system"
```

### Priority Management
```bash
# Critical issues first
todozi queue plan --task-name "Security Fix" --task-description "Fix critical security vulnerability" --priority critical

# Then high priority features
todozi queue plan --task-name "New Feature" --task-description "Implement requested feature" --priority high

# Finally medium priority improvements
todozi queue plan --task-name "Code Refactor" --task-description "Refactor legacy code" --priority medium
```

## 🎉 Getting Started Checklist

- [ ] Initialize Todozi: `todozi init`
- [ ] Plan your first task: `todozi queue plan --task-name "..." --task-description "..."`
- [ ] Start working: `todozi queue start <item_id>`
- [ ] Complete work: `todozi queue end <session_id>`
- [ ] Review your queue: `todozi queue list`
- [ ] Check completed work: `todozi queue complete`

## 🆘 Support

For issues, questions, or feature requests:
- Check the main Todozi documentation
- Review error messages and troubleshooting section
- Use `todozi --help` for command reference
- Use `todozi queue --help` for queue-specific help

---

**Happy Queueing! 🚀**

The Todozi Queue System helps you stay organized, track time, and manage your work efficiently whether you're human, AI, or working together!
