### Example 2: Managing a Software Project with Todozi CLI and Server

This example demonstrates how to use Todozi to manage a software development project. We'll:
1. Initialize the workspace
2. Create a project and add tasks
3. Record important decisions and ideas
4. Start the server to access tasks via API
5. Update and complete tasks

---

#### Step 1: Initialize Todozi Workspace
```bash
# Initialize your local workspace
$ todozi init
✅ Todozi initialized at /home/user/.todozi
```

#### Step 2: Create a New Project
```bash
# Create a project for our web application
$ todozi project create "WebApp" --description "Customer portal web application"
✅ Project 'WebApp' created successfully!

# List projects to verify
$ todozi project list
📁 Available projects:
  WebApp: Customer portal web application
```

#### Step 3: Add Tasks to the Project
```bash
# Add backend tasks
$ todozi add task \
  "Design user authentication API" \
  --time "4 hours" \
  --priority "high" \
  --project "WebApp" \
  --status "todo" \
  --tags "api,auth,backend"

✅ Task created: task_12345678
   Action: Design user authentication API
   Project: WebApp
   Priority: high
   Status: todo

# Add frontend tasks
$ todozi add task \
  "Create login page UI" \
  --time "2 hours" \
  --priority "medium" \
  --project "WebApp" \
  --assignee "human" \
  --dependencies "task_12345678"

✅ Task created: task_87654321
   Action: Create login page UI
   Project: WebApp
   Priority: medium
   Status: todo
```

#### Step 4: Record Important Decisions and Ideas
```bash
# Record a design decision
$ todozi memory create \
  "Authentication Design" \
  "We decided to use JWT tokens for authentication" \
  "After evaluating OAuth vs JWT" \
  --importance "high" \
  --tags "auth,jwt,decision"

✅ Memory created with ID: mem_abcdef12

# Add an innovative idea
$ todozi idea create \
  "Implement biometric login" \
  --share "team" \
  --importance "low" \
  --tags "auth,innovation"

✅ Idea created with ID: idea_bcdef34
```

#### Step 5: Start the Server for API Access
```bash
# Start the Todozi server in background
$ todozi server start --host 0.0.0.0 --port 8636 &
🚀 Todozi Enhanced Server starting on 0.0.0.0:8636

# Check server status
$ todozi server status
✅ Server is running on port 8636
🌐 API available at: http://0.0.0.0:8636
```

#### Step 6: Interact with Tasks via HTTP API
```bash
# Get all tasks in JSON format
$ curl http://localhost:8636/tasks | jq '.'
{
  "tasks": [
    {
      "id": "task_12345678",
      "action": "Design user authentication API",
      "priority": "high",
      "status": "todo",
      "parent_project": "WebApp",
      "created_at": "2023-07-15T10:00:00Z"
    },
    {
      "id": "task_87654321",
      "action": "Create login page UI",
      "priority": "medium",
      "status": "todo",
      "parent_project": "WebApp",
      "dependencies": ["task_12345678"]
    }
  ]
}

# Update a task status
$ curl -X PUT http://localhost:8636/tasks/task_12345678 \
  -H "Content-Type: application/json" \
  -d '{"status":"in_progress"}' | jq '.'

{
  "message": "Task updated successfully",
  "task": {
    "id": "task_12345678",
    "status": "in_progress",
    "updated_at": "2023-07-15T11:30:00Z"
  }
}
```

#### Step 7: Continue Managing Tasks via CLI
```bash
# List all tasks in the project
$ todozi list tasks --project "WebApp"
📋 Tasks in project 'WebApp':
  [task_12345678] Design user authentication API
    Priority: high | Status: in_progress
  [task_87654321] Create login page UI
    Priority: medium | Status: todo

# Complete the API design task
$ todozi complete task_12345678
✅ Task task_12345678 completed successfully!

# Add a new dependent task
$ todozi add task \
  "Implement JWT token service" \
  --time "6 hours" \
  --priority "high" \
  --project "WebApp" \
  --dependencies "task_12345678"

✅ Task created: task_56781234
```

#### Step 8: Review Project Statistics
```bash
# Get project overview
$ todozi stats show
📊 Todozi Statistics:
   Total tasks: 3
   Active tasks: 2
   Completed tasks: 1
   Projects: 1

# Check your recorded memories
$ todozi memory list --importance "high"
🧠 Important memories:
  [mem_abcdef12] Authentication Design
    Moment: Authentication Design
    Meaning: We decided to use JWT tokens for authentication
    Importance: high
```

#### Step 9: Backup Your Work
```bash
# Create a backup
$ todozi backup create
✅ Backup created: todozi_backup_20230715_120000

# List available backups
$ todozi list-backups
📦 Available backups:
  todozi_backup_20230715_120000