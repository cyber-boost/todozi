### Example 2: Creating and Managing Tasks with Priority, Tags, and Search

This example demonstrates creating tasks with priorities and tags, filtering tasks, searching, updating tasks, and completing them. We'll use the `todozi` CLI to manage a small project.

#### Step 1: Create two tasks with different priorities and tags

```bash
# Create a high-priority task for the "webapp" project with tags
todozi add task "Implement user authentication" --time "4 hours" --priority high --project webapp --status todo --tags auth,security

# Create a medium-priority task for the same project with different tags
todozi add task "Design login page UI" --time "2 hours" --priority medium --project webapp --status todo --tags ui,design
```

**Expected Output:**

```
✅ Task created: abc12345-6789-0abc-def0-1234567890ab
   Action: Implement user authentication
   Project: webapp
   Priority: high
   Status: todo

✅ Task created: def12345-6789-0abc-def0-1234567890ab
   Action: Design login page UI
   Project: webapp
   Priority: medium
   Status: todo
```

#### Step 2: List all tasks and filter by priority

```bash
# List all tasks in the "webapp" project
todozi list tasks --project webapp

# List only high-priority tasks
todozi list tasks --priority high
```

**Expected Output for `todozi list tasks --project webapp`:**

```
Found 2 task(s):

[abc12345-6789-0abc-def0-1234567890ab] Implement user authentication
  Project: webapp | Priority: high | Status: todo
  Assignee: unassigned | Tags: auth,security

[def12345-6789-0abc-def0-1234567890ab] Design login page UI
  Project: webapp | Priority: medium | Status: todo
  Assignee: unassigned | Tags: ui,design
```

**Expected Output for `todozi list tasks --priority high`:**

```
Found 1 task(s):

[abc12345-6789-0abc-def0-1234567890ab] Implement user authentication
  Project: webapp | Priority: high | Status: todo
  Assignee: unassigned | Tags: auth,security
```

#### Step 3: Search tasks by keyword and update a task

```bash
# Search for tasks containing the word "authentication"
todozi search tasks "authentication"

# Update the high-priority task: add context notes and change priority to critical
todozi update abc12345-6789-0abc-def0-1234567890ab --priority critical --context "Use OAuth 2.0 and integrate with Google and GitHub"
```

**Expected Output for `todozi search tasks "authentication"`:**

```
Found 1 task(s) matching 'authentication':

[abc12345-6789-0abc-def0-1234567890ab] Implement user authentication
  Project: webapp | Priority: high | Status: todo
  Assignee: unassigned | Tags: auth,security
```

**Expected Output for the update command:**

```
✅ Task abc12345-6789-0abc-def0-1234567890ab updated successfully!
```

#### Step 4: Show the updated task and mark it as completed

```bash
# Show the updated task details
todozi show task abc12345-6789-0abc-def0-1234567890ab

# Mark the task as completed
todozi complete abc12345-6789-0abc-def0-1234567890ab
```

**Expected Output for `todozi show task abc12345-6789-0abc-def0-1234567890ab`:**

```
Task: abc12345-6789-0abc-def0-1234567890ab
Action: Implement user authentication
Time: 4 hours
Priority: critical
Project: webapp
Status: todo
Assignee: unassigned
Tags: auth,security
Context: Use OAuth 2.0 and integrate with Google and GitHub
Created: 2023-04-01 12:00:00+00:00
Updated: 2023-04-01 12:05:00+00:00
```

**Expected Output for `todozi complete abc12345-6789-0abc-def0-1234567890ab`:**

```
✅ Task abc12345-6789-0abc-def0-1234567890ab completed successfully!
```

#### Step 5: Delete the completed task (optional)

```bash
# Delete the completed task
todozi delete abc12345-6789-0abc-def0-1234567890ab
```

**Expected Output:**

```
✅ Task abc12345-6789-0abc-def0-1234567890ab deleted successfully!