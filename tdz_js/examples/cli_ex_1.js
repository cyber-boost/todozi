# Example: Creating and Managing Tasks in Todozi CLI

This example demonstrates how to use various Todozi CLI commands to create and manage tasks effectively.

## Basic Task Creation

/*
bash
# Create a basic task
todozi add task "Implement user authentication" --time "4 hours" --priority high --project "web-app" --status todo

# Expected output:
# Task created: task_abc123
# Action: Implement user authentication
# Project: web-app
# Priority: high
# Status: todo

/ *
## Advanced Task with Tags and Dependencies

# Create a task with tags, dependencies, and context
todozi add task "Setup database migrations" \
  --time "2 hours" \
  --priority medium \
  --project "backend" \
  --status todo \
  --tags "database,setup,migrations" \
  --dependencies "task_xyz789" \
  --context "Use PostgreSQL with proper connection pooling"

# Expected output:
# Task created: task_def456
# Action: Setup database migrations
# Project: backend
# Priority: medium
# Status: todo
# Tags: database, setup, migrations
# Dependencies: task_xyz789

## Using Chat Mode for Natural Language Task Creation

# Create tasks using natural language chat formatting
todozi chat "I need to <todozi>Review authentication code;2 hours;medium;security-review;review</todozi> and also <todozi>Optimize database queries;3 hours;high;backend;in_progress</todozi>."

# Expected output:
# ✅ Chat processed successfully!
# 📊 Content extracted:
#   📋 Tasks: 2
#   🧠 Memories: 0
#   💡 Ideas: 0
#   🤖 Agent Assignments: 0
#   🧩 Code Chunks: 0
#   ❌ Errors: 0
#   🎓 Training Data: 0
# 
# ✅ Successfully created/processed:
#   📋 Task: Review authentication code
#   📋 Task: Optimize database queries
# 
# 🎉 Total items processed: 2

## Managing Task Status and Progress

# List all active tasks in a project
todozi list tasks --project "backend" --status todo

# Expected output:
# Tasks:
#   task_abc123: Review authentication code (backend, medium, todo)
#   task_def456: Setup database migrations (backend, medium, todo)

# Update task status and progress
todozi update task_def456 --status in_progress --progress 25

# Expected output:
# Task task_def456 updated successfully!

# Complete a task
todozi complete task_abc123

# Expected output:
# ✅ Task completed and saved: Review authentication code

## Using AI Features for Task Suggestions

# Get AI suggestions for similar tasks
todozi ai similar "database performance"

# Expected output:
# Similar tasks for 'database performance':
# ══════════════════════════════════════
# 1. Optimize database queries (82.3% similar)
# 2. Add database indexing (75.1% similar)
# 3. Setup database monitoring (68.9% similar)

# Get AI insights about your tasks
todozi ai insights

# Expected output:
# 🧠 AI Insights & Statistics:
# ═════════════════════════════════════
# total_embeddings: 15
# type_counts: {"Task": 15}
# 
# 🔗 Semantic Clusters:
#   1. 4 items (avg similarity: 76.2%)

## Project Management

# Create a new project
todozi project create "mobile-app" --description "New iOS and Android application"

# Expected output:
# Project 'mobile-app' created successfully!

# Show project details
todozi project show mobile-app

# Expected output:
# Project: mobile-app
# Description: New iOS and Android application
# Status: active
# Tasks: 0
# Created: 2023-11-15T10:30:00Z
# Updated: 2023-11-15T10:30:00Z

# List all projects
todozi project list

# Expected output:
# Projects:
#   mobile-app - New iOS and Android application (0 tasks)
#   backend - Backend API services (5 tasks)
#   web-app - Frontend web application (3 tasks)

## Error Tracking Integration

# Create an error record when tasks encounter issues
todozi error create \
  --title "Database connection timeout" \
  --description "Database queries timing out after 30 seconds" \
  --severity high \
  --category database \
  --source "backend project" \
  --tags "database,performance,timeout"

# Expected output:
# ✅ Error record created with ID: err_789xyz

## Practical Use Case: Development Workflow
*/