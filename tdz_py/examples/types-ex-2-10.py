### Example 2: Advanced Task Management with Todozi

This example demonstrates a complete workflow of creating, managing, and completing a task with Todozi. It showcases more advanced features like tags, dependencies, progress tracking, and project management.

#### Step 1: Initialize Todozi Workspace
```bash
todozi init
```
Output:
```
Initialized.
```

#### Step 2: Create a New Task
Create a task with multiple attributes including tags, dependencies, and context notes.

```bash
todozi add task \
  "Implement user authentication system" \
  --time "4 hours" \
  --priority "high" \
  --project "webapp" \
  --status "todo" \
  --assignee "human" \
  --tags "auth,security,backend" \
  --dependencies "Design auth flow,Create user model" \
  --context "Must support OAuth2 and JWT" \
  --progress 0
```
Output:
```
✅ Task created: task_a1b2c3d4
   Action: Implement user authentication system
   Project: webapp
   Priority: high
   Status: todo
```

#### Step 3: List Tasks with Filters
List all tasks in the "webapp" project with high priority:

```bash
todozi list tasks --project "webapp" --priority "high"
```
Output:
```
Found 1 task(s):

[task_a1b2c3d4] Implement user authentication system
  Project: webapp | Priority: high | Status: todo
  Assignee: human | Tags: auth,security,backend
```

#### Step 4: Show Task Details
Display detailed information about the task:

```bash
todozi show task_a1b2c3d4
```
Output:
```
Task: task_a1b2c3d4
Action: Implement user authentication system
Time: 4 hours
Priority: high
Project: webapp
Status: todo
Assignee: human
Tags: auth,security,backend
Dependencies: Design auth flow,Create user model
Context: Must support OAuth2 and JWT
Progress: 0%
Created: 2023-11-15 14:30:00
Updated: 2023-11-15 14:30:00
```

#### Step 5: Update Task Progress
Update the task status to in-progress and set progress to 50%:

```bash
todozi update task_a1b2c3d4 \
  --status "in_progress" \
  --progress 50
```
Output:
```
✅ Task task_a1b2c3d4 updated successfully!
```

#### Step 6: Complete the Task
Mark the task as completed:

```bash
todozi complete task_a1b2c3d4
```
Output:
```
✅ Task task_a1b2c3d4 completed successfully!
```

#### Step 7: Verify Task Completion
Show the task details again to confirm its status:

```bash
todozi show task_a1b2c3d4
```
Output:
```
Task: task_a1b2c3d4
Action: Implement user authentication system
Time: 4 hours
Priority: high
Project: webapp
Status: done
Assignee: human
Tags: auth,security,backend
Dependencies: Design auth flow,Create user model
Context: Must support OAuth2 and JWT
Progress: 50%
Created: 2023-11-15 14:30:00
Updated: 2023-11-15 16:45:00
```

#### Step 8: List All Tasks in Project
List all tasks in the project to see the final state:

```bash
todozi list tasks --project "webapp"
```
Output:
```
Found 1 task(s):

[task_a1b2c3d4] Implement user authentication system
  Project: webapp | Priority: high | Status: done
  Assignee: human | Tags: auth,security,backend
  Progress: 50%
```

### Additional Features Demonstrated

#### Creating Multiple Tasks with Different Assignees
```bash
# Create an AI-assigned task
todozi add task \
  "Generate test cases for auth module" \
  --time "2 hours" \
  --priority "medium" \
  --project "webapp" \
  --assignee "ai" \
  --tags "testing,automation"

# Create a collaborative task
todozi add task \
  "Review authentication implementation" \
  --time "1 hour" \
  --priority "high" \
  --project "webapp" \
  --assignee "collaborative" \
  --tags "review,security"
```

#### Working with Projects
```bash
# Create a new project
todozi project create "mobile" --description "Mobile app tasks"

# List all projects
todozi project list

# Archive old project
todozi project archive "legacy"
```

#### Using Search
```bash
# Search for tasks containing "authentication"
todozi search tasks "authentication"

# Search with filters
todozi list tasks --search "security" --priority "high" --assignee "human"