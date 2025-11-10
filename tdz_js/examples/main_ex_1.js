# Todozi Task Management System - Example 1: Basic Project and Task Management

Here's a practical example demonstrating how to use Todozi for basic project and task management:

## Example: Managing a "Website Redesign" Project

### 1. Initialize Todozi

/*
bash
# First-time setup
todozi init

/ *
### 2. Create a Project
# Create a project for website redesign
todozi project create --name "website-redesign" --description "Complete website overhaul with modern design"

### 3. Add Tasks to the Project
# Add tasks with different priorities and assignees
todozi add --action "Create wireframes" --time "4 hours" --priority "high" --project "website-redesign" --status "todo"
todozi add --action "Set up development environment" --time "2 hours" --priority "medium" --project "website-redesign" --status "todo"
todozi add --action "Design color scheme" --time "3 hours" --priority "medium" --project "website-redesign" --status "todo" --assignee "ai"
todozi add --action "Write content for homepage" --time "5 hours" --priority "high" --project "website-redesign" --status "todo"

### 4. List and Manage Tasks
# List all tasks in the project
todozi list --project "website-redesign"

# Search for specific tasks
todozi search --query "wireframes"

# Update task status when work begins
todozi update --id "task_abc123" --status "in_progress"

# Record progress on a task
todozi update --id "task_def456" --progress 75 --context "Color scheme approved by client"

# Add tags to organize tasks
todozi update --id "task_ghi789" --tags "frontend,design,urgent"

### 5. Complete Tasks
# Mark a task as completed
todozi complete --id "task_abc123"

# Show task details
todozi show --id "task_def456"

### 6. Project Statistics and Backups
# Check project statistics
todozi stats

# Create a backup of all data
todozi backup create

# List available backups
todozi list-backups

### 7. Using the Chat Feature for Task Creation
# Create multiple tasks using chat syntax
todozi chat --message "
We need to complete the website redesign project. Here are the next steps:
<todozi>Implement responsive design;6 hours;high;website-redesign;todo;ai;css,frontend</todozi>
<todozi>Test cross-browser compatibility;3 hours;medium;website-redesign;todo;tester</todozi>
<todozi>Optimize images for web;2 hours;low;website-redesign;todo</todozi>
"

### 8. Agent Assignment (Advanced)
# Create specialized agents for the project
todozi agent create --id "designer" --name "Design Specialist" --description "UI/UX design expert" --category "design"

# Assign tasks to agents
todozi agent assign --agent-id "designer" --task-id "task_xyz123" --project-id "website-redesign"

### 9. Migration and Structure Management
# Check if your todozi structure is complete
todozi check-structure

# Migrate to new project-based system (if needed)
todozi migrate --dry-run --verbose

## Expected Output Examples

When you run `todozi list --project "website-redesign"`, you might see:
📋 Tasks for website-redesign:
🟡 🤖 [task_abc123] Create wireframes (high, in_progress, 50% progress)
🟡 👤 [task_def456] Set up development environment (medium, todo)
🟠 🤖 [task_ghi789] Design color scheme (medium, todo, AI assigned)
🔴 👤 [task_jkl012] Write content for homepage (high, todo)

When you check stats with `todozi stats`:
*/