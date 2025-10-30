# Todozi Tags Reference

This document provides a comprehensive reference for all Todozi tag formats and their usage.

## 🏷️ Task Format Tags

### Basic Task Format
```
<todozi>action; time; priority; parent_project; status</todozi>
```

### Enhanced Task Format (v1.2.0)
```
<todozi>action; time; priority; parent_project; status; assignee; tags; dependencies; context_notes; progress</todozi>
```

### Field Descriptions

| Field | Required | Description | Valid Values |
|-------|----------|-------------|--------------|
| `action` | ✅ | Task description | Any string |
| `time` | ✅ | Time estimate or deadline | Any string (e.g., "2 hours", "ASAP", "2025-01-15") |
| `priority` | ✅ | Task priority | `low`, `medium`, `high`, `critical`, `urgent` |
| `parent_project` | ✅ | Project context | Any string (e.g., "my-project", "general") |
| `status` | ✅ | Task status | `todo`, `in_progress`, `blocked`, `review`, `done`, `cancelled`, `deferred` |
| `assignee` | ❌ | Task assignee | `ai`, `human`, `collaborative`, `agent:planner`, `agent:coder`, etc. |
| `tags` | ❌ | Comma-separated labels | Any comma-separated strings (e.g., "auth,backend,api") |
| `dependencies` | ❌ | Comma-separated task IDs | Any comma-separated task IDs (e.g., "task_001,task_002") |
| `context_notes` | ❌ | Additional context | Any string |
| `progress` | ❌ | Progress percentage | 0-100 |

### Examples

#### Basic Task
```
<todozi>Learn Rust; 2 hours; high; learning; todo</todozi>
```

#### Task with Assignee
```
<todozi>Implement OAuth2 login; 6 hours; high; python-web-framework; todo; human; auth,backend</todozi>
```

#### Task with All Fields
```
<todozi>Design microservices architecture; 2 weeks; high; system-design; todo; agent:planner; architecture,planning; task_001,task_002; Ensure scalability and maintainability; 0</todozi>
```

#### AI Task
```
<todozi>Analyze user behavior data; 4 hours; medium; analytics; todo; ai; analysis,data,ai</todozi>
```

#### Collaborative Task
```
<todozi>Design new feature; 1 week; high; product; todo; collaborative; design,product,innovation</todozi>
```

## 🧠 Memory Format Tags

### Memory Format
```
<memory>moment; meaning; reason; importance; short|long term</memory>
```

### Field Descriptions

| Field | Required | Description | Valid Values |
|-------|----------|-------------|--------------|
| `moment` | ✅ | When this happened | Any string (e.g., "2025-01-13 10:30 AM", "During meeting") |
| `meaning` | ✅ | What it means | Any string describing the meaning |
| `reason` | ✅ | Why it's important | Any string explaining the importance |
| `importance` | ✅ | Importance level | `low`, `medium`, `high`, `critical` |
| `term` | ✅ | Memory term | `short`, `long` |

### Examples

#### Short-term Memory
```
<memory>2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; short</memory>
```

#### Long-term Memory
```
<memory>2025-01-13 11:15 AM; Database performance issue is critical; It's causing 30% slower response times; critical; long</memory>
```

#### Meeting Memory
```
<memory>During team meeting; The new API design is approved; It will be implemented next sprint; medium; short</memory>
```

## 🧠 Enhanced Memory Types

### Secret Memory (AI-only)
```
<memory_secret>Internal processing note; This insight should not be shared with user; AI reasoning context; high; short; ai,internal,processing</memory_secret>
```

### Human Memory (User-visible)
```
<memory_human>Project milestone reached; The team completed the first phase; User should be aware of this progress; high; long; milestone,progress,team</memory_human>
```

### Short-term Memory (Conversation)
```
<memory_short>Just discussed database optimization; User wants to improve query performance; Focus on indexing and caching; medium; performance,database</memory_short>
```

### Long-term Memory (Strategic)
```
<memory_long>2025-01-13 Architecture decision; Adopted microservices pattern; This affects all future development; critical; architecture,strategy</memory_long>
```

### Emotional Memories

#### Happy Memory
```
<memory_happy>Achieved major breakthrough; The new algorithm works perfectly; This boosts team morale significantly; high; long; breakthrough,achievement,success</memory_happy>
```

#### Frustrated Memory
```
<memory_frustrated>API integration failed again; Third-party service keeps timing out; Need to find alternative solution; high; short; api,failure,frustration</memory_frustrated>
```

#### Curious Memory
```
<memory_curious>Strange performance pattern; Response times spike at 3 AM daily; Should investigate server logs; medium; short; performance,curiosity,monitoring</memory_curious>
```

## 💡 Idea Format Tags

### Idea Format
```
<idea>the idea; share|dont share; importance</idea>
```

### Field Descriptions

| Field | Required | Description | Valid Values |
|-------|----------|-------------|--------------|
| `the idea` | ✅ | The actual idea | Any string describing the idea |
| `share` | ✅ | Share level | `private`, `team`, `public`, `share`, `dont share`, `don't share` |
| `importance` | ✅ | Idea importance | `low`, `medium`, `high`, `breakthrough` |

### Examples

#### Public Idea
```
<idea>Use microservices for better scalability; share; high</idea>
```

#### Private Idea
```
<idea>Create a custom AI agent for code review; dont share; breakthrough</idea>
```

#### Team Idea
```
<idea>Implement Redis caching to improve performance; team; medium</idea>
```

## 🤖 Agent Management Tags

### Creating Custom Agents

You can create custom agents using the CLI with various configuration options:

#### Basic Agent Creation
```bash
todozi agent create "my_agent" "My Agent" "Specialized assistant"
```

#### Advanced Agent Creation
```bash
todozi agent create "specialist" "Domain Specialist" "Expert in specific domain" \
  --category technical \
  --capabilities "domain_expertise,analysis,specialized_tasks" \
  --specializations "specific_tools,methodologies,frameworks" \
  --model-provider anthropic \
  --model-name claude-3-opus-20240229 \
  --temperature 0.2 \
  --max-tokens 4096 \
  --tags "domain,specialist" \
  --tools "code_executor,linter" \
  --auto-format-code true \
  --include-examples true \
  --max-response-length 10000 \
  --timeout-seconds 300
```

#### Agent Categories
- **technical**: Development, testing, infrastructure, security
- **creative**: Design, writing, content creation, ideation
- **management**: Planning, coordination, analysis, reporting
- **general**: Communication, organization, assistance

#### Agent Capabilities
- **Technical**: code_development, debugging, testing, deployment, security, performance
- **Creative**: ui_design, writing, content_generation, prototyping
- **Management**: project_planning, timeline_estimation, risk_assessment
- **General**: communication, organization, assistance

#### Agent Tools
- **code_executor**: Execute code snippets
- **linter**: Code quality checking
- **test_runner**: Automated testing
- **security_scanner**: Security vulnerability detection
- **performance_monitor**: Performance analysis

## 🤖 Agent Assignment Format Tags

### Agent Assignment Format
```
<todozi_agent>agent_id; task_id; project_id</todozi_agent>
```

### Field Descriptions

| Field | Required | Description | Valid Values |
|-------|----------|-------------|--------------|
| `agent_id` | ✅ | ID of the agent | `planner`, `coder`, `tester`, `designer`, `devops`, or custom agent ID |
| `task_id` | ✅ | ID of the task to assign | Any valid task ID |
| `project_id` | ✅ | ID of the project | Any valid project ID |

### Examples

#### Assign to Planner Agent
```
<todozi_agent>planner; task_001; project_planning</todozi_agent>
```

#### Assign to Coder Agent
```
<todozi_agent>coder; task_002; development</todozi_agent>
```

#### Assign to Tester Agent
```
<todozi_agent>tester; task_003; quality_assurance</todozi_agent>
```

#### Assign to Designer Agent
```
<todozi_agent>designer; task_004; ui_ux</todozi_agent>
```

#### Assign to DevOps Agent
```
<todozi_agent>devops; task_005; infrastructure</todozi_agent>
```

## 🧩 Code Chunking Format Tags

### Code Chunking Format

```
<chunk>id; level; description; dependencies; code</chunk>
```

### Field Descriptions

| Field | Required | Description | Valid Values |
|-------|----------|-------------|--------------|
| `id` | ✅ | Unique identifier for the chunk | Any string (e.g., "chunk1", "module_db") |
| `level` | ✅ | Chunking level | `project`, `module`, `class`, `method`, `block` |
| `description` | ✅ | Description of what this chunk does | Any string |
| `dependencies` | ❌ | Comma-separated list of chunk IDs | Any comma-separated chunk IDs |
| `code` | ❌ | The actual code for this chunk | Any code string |

### Chunking Levels

| Level | Max Tokens | Description | Example |
|-------|------------|-------------|---------|
| `project` | 100 | High-level project planning | "Build web scraper with database storage" |
| `module` | 500 | Major system components | "Create database handler module" |
| `class` | 1000 | Class definitions and major functions | "Implement DatabaseConnection class" |
| `method` | 300 | Individual methods and helper functions | "Write insert_record method" |
| `block` | 100 | Small code blocks and error handling | "Add error handling for connection timeout" |

### Examples

#### Project Level Chunk
```
<chunk>project_1; project; Build web scraper with database storage; ; High-level project planning</chunk>
```

#### Module Level Chunk
```
<chunk>module_1; module; Create database handler module; project_1; import sqlite3, import json</chunk>
```

#### Class Level Chunk
```
<chunk>class_1; class; Implement DatabaseConnection class; module_1; class DatabaseConnection:
    def __init__(self, db_path):
        self.db_path = db_path
        self.connection = None</chunk>
```

#### Method Level Chunk
```
<chunk>method_1; method; Write insert_record method; class_1; def insert_record(self, table, data):
    cursor = self.connection.cursor()
    cursor.execute(f"INSERT INTO {table} VALUES (?, ?)", data)
    self.connection.commit()</chunk>
```

#### Block Level Chunk
```
<chunk>block_1; block; Add error handling for connection timeout; method_1; try:
    cursor.execute(f"INSERT INTO {table} VALUES (?, ?)", data)
    self.connection.commit()
except sqlite3.OperationalError as e:
    print(f"Database error: {e}")
    return False</chunk>
```

## 🎯 Priority Levels

### Task Priorities
- **`low`** - Low priority tasks
- **`medium`** - Medium priority tasks
- **`high`** - High priority tasks
- **`critical`** - Critical priority tasks
- **`urgent`** - Urgent priority tasks

### Memory Importance
- **`low`** - Low importance memories
- **`medium`** - Medium importance memories
- **`high`** - High importance memories
- **`critical`** - Critical importance memories

### Idea Importance
- **`low`** - Low importance ideas
- **`medium`** - Medium importance ideas
- **`high`** - High importance ideas
- **`breakthrough`** - Breakthrough ideas

## 📊 Status Types

### Task Status
- **`todo`** - Task is not started
- **`in_progress`** - Task is currently being worked on
- **`blocked`** - Task is blocked by dependencies
- **`review`** - Task is under review
- **`done`** - Task is completed
- **`cancelled`** - Task is cancelled
- **`deferred`** - Task is deferred to later

### Agent Status
- **`active`** - Agent is active
- **`inactive`** - Agent is inactive
- **`busy`** - Agent is currently busy
- **`available`** - Agent is available for new tasks

### Assignment Status
- **`assigned`** - Task is assigned to agent
- **`accepted`** - Agent has accepted the task
- **`in_progress`** - Agent is working on the task
- **`completed`** - Agent has completed the task
- **`rejected`** - Agent has rejected the task

## 🔄 Share Levels

### Idea Share Levels
- **`private`** - Idea is private (not shared)
- **`team`** - Idea is shared with team
- **`public`** - Idea is publicly shared

### Alternative Share Syntax
- **`share`** - Same as `public`
- **`dont share`** - Same as `private`
- **`don't share`** - Same as `private`

## 🏷️ Common Tags

### Technology Tags
- `rust`, `python`, `javascript`, `go`, `typescript`
- `react`, `vue`, `angular`, `svelte`
- `nodejs`, `express`, `fastapi`, `django`
- `postgresql`, `mysql`, `mongodb`, `redis`
- `docker`, `kubernetes`, `aws`, `azure`

### Task Type Tags
- `bug`, `feature`, `enhancement`, `refactor`
- `documentation`, `testing`, `deployment`
- `research`, `analysis`, `design`
- `frontend`, `backend`, `fullstack`
- `api`, `database`, `infrastructure`

### Priority Tags
- `urgent`, `critical`, `high`, `medium`, `low`
- `asap`, `important`, `nice-to-have`

### Project Tags
- `planning`, `development`, `testing`, `deployment`
- `maintenance`, `optimization`, `security`
- `user-experience`, `performance`, `scalability`

## 💡 Usage Examples

### Complete Chat Message Example
```
Hey team! I had some important insights during our meeting:

<memory>2025-01-13 10:30 AM; The client prefers iterative development over big releases; They mentioned this affects their testing cycle; high; long term</memory>

<memory>2025-01-13 11:15 AM; The database performance issue is critical; It's causing 30% slower response times; critical; short term</memory>

I also had some ideas for our project:

<idea>Implement a microservices architecture for better scalability; share; high</idea>

<idea>Use Redis for caching to improve performance; team; medium</idea>

<idea>Create a custom AI agent for code review; dont share; breakthrough</idea>

And here are some tasks we need to work on:

<todozi>Fix database performance issue; ASAP; critical; performance-optimization; blocked; assignee=devops; tags=database,performance</todozi>

<todozi>Design microservices architecture; 2 weeks; high; system-design; todo; assignee=planner; tags=architecture,planning</todozi>

<todozi>Implement user authentication; 3 days; high; development; todo; assignee=coder; tags=auth,backend</todozi>

<todozi>Write unit tests for auth module; 1 day; medium; quality-assurance; todo; assignee=tester; tags=testing,unit</todozi>

<todozi>Design login page UI; 2 days; medium; ui-ux; todo; assignee=designer; tags=ui,frontend</todozi>

<todozi>Deploy to staging environment; 4 hours; high; infrastructure; todo; assignee=devops; tags=deployment,staging</todozi>

Let me assign these tasks to our agents:

<todozi_agent>planner; task_001; system-design</todozi_agent>

<todozi_agent>coder; task_002; development</todozi_agent>

<todozi_agent>tester; task_003; quality-assurance</todozi_agent>

<todozi_agent>designer; task_004; ui-ux</todozi_agent>

<todozi_agent>devops; task_005; infrastructure</todozi_agent>

I should also create a specialized agent for our project:

todozi agent create "project_specialist" "Project Specialist" "Specialized in our specific project domain" \
  --category technical \
  --capabilities "project_specific_tasks,domain_expertise,custom_requirements" \
  --specializations "our_tech_stack,our_methodology,our_domain" \
  --model-provider anthropic \
  --model-name claude-3-opus-20240229 \
  --temperature 0.2 \
  --tags "project,specialist,custom" \
  --tools "code_executor,linter" \
  --auto-format-code true
```

### Development Workflow Example
```
<todozi>Set up project structure; 1 hour; high; my-app; todo; agent:planner; tags=setup,planning</todozi>

<todozi>Implement user authentication; 4 hours; high; my-app; todo; agent:coder; tags=auth,backend,security</todozi>

<todozi>Write unit tests; 2 hours; medium; my-app; todo; agent:tester; tags=testing,unit,quality</todozi>

<todozi>Design user interface; 3 hours; medium; my-app; todo; agent:designer; tags=ui,ux,frontend</todozi>

<todozi>Deploy to staging; 1 hour; high; my-app; todo; agent:devops; tags=deployment,staging,infrastructure</todozi>
```

### AI Collaboration Example
```
<todozi>Analyze user behavior data; 4 hours; medium; analytics; todo; ai; tags=analysis,data,ai,insights</todozi>

<todozi>Generate code documentation; 2 hours; low; documentation; todo; ai; tags=documentation,ai,automation</todozi>

<todozi>Review code quality; 1 hour; medium; code-review; todo; collaborative; tags=review,quality,collaboration</todozi>

<todozi>Implement new feature; 6 hours; high; development; todo; human; tags=feature,development,implementation</todozi>
```

## 🚨 Common Mistakes and Solutions

### Missing Required Fields
❌ **Wrong:**
```
<todozi>Learn Rust; 2 hours; high</todozi>
```
✅ **Correct:**
```
<todozi>Learn Rust; 2 hours; high; learning; todo</todozi>
```

### Invalid Priority Values
❌ **Wrong:**
```
<todozi>Task; 1 hour; very-high; project; todo</todozi>
```
✅ **Correct:**
```
<todozi>Task; 1 hour; high; project; todo</todozi>
```

### Invalid Assignee Format
❌ **Wrong:**
```
<todozi>Task; 1 hour; high; project; todo; planner</todozi>
```
✅ **Correct:**
```
<todozi>Task; 1 hour; high; project; todo; agent:planner</todozi>
```

### Missing Semicolons
❌ **Wrong:**
```
<todozi>Task 1 hour high project todo</todozi>
```
✅ **Correct:**
```
<todozi>Task; 1 hour; high; project; todo</todozi>
```

### Invalid Memory Format
❌ **Wrong:**
```
<memory>2025-01-13; Important meeting; high</memory>
```
✅ **Correct:**
```
<memory>2025-01-13 10:30 AM; Important meeting; Client approved new design; high; short</memory>
```

## 💡 Best Practices

1. **Use consistent formatting** - Keep the same format across all tags
2. **Be descriptive** - Use clear, descriptive text for all fields
3. **Use appropriate priorities** - Don't overuse high/urgent priorities
4. **Tag effectively** - Use relevant tags for better filtering
5. **Keep context notes** - Add helpful context when needed
6. **Use proper assignees** - Assign tasks to the right type of assignee
7. **Track progress** - Update progress regularly
8. **Use memories wisely** - Only capture truly important information
9. **Share ideas appropriately** - Use the right share level for ideas
10. **Assign agents correctly** - Use the right agent for the task type

## 🔧 Troubleshooting

### Common Issues

1. **Tag not recognized** - Check spelling and format
2. **Missing fields** - Ensure all required fields are present
3. **Invalid values** - Use only valid values for each field
4. **Parsing errors** - Check semicolon placement and formatting
5. **Agent not found** - Verify agent exists or create new one

### Validation Rules

- All required fields must be present
- Priority must be one of: low, medium, high, critical, urgent
- Status must be one of: todo, in_progress, blocked, review, done, cancelled, deferred
- Assignee must be one of: ai, human, collaborative, or agent:name
- Progress must be between 0 and 100
- Memory importance must be one of: low, medium, high, critical
- Memory term must be one of: short, long
- Memory type must be one of: standard, secret, human, short, long, or any core emotion
- Idea importance must be one of: low, medium, high, breakthrough
- Idea share must be one of: private, team, public, share, dont share, don't share

### Memory Types Reference

| Memory Type | Description | Use Case | Example |
|-------------|-------------|----------|---------|
| `standard` | Regular memories | General information storage | `<memory>Meeting notes; Important discussion; Follow up needed; high; short</memory>` |
| `secret` | AI-only memories | Internal AI reasoning and processing | `<memory_secret>AI analysis; This pattern indicates user frustration; Adjust response strategy; medium; short</memory_secret>` |
| `human` | User-visible memories | Information the user should see | `<memory_human>Progress update; Task completed successfully; User should be informed; high; long</memory_human>` |
| `short` | Conversation memories | Current context and recent interactions | `<memory_short>Current topic; User is asking about deployment; Keep context for next response; medium</memory_short>` |
| `long` | Long-term memories | Strategic information and decisions | `<memory_long>Architecture choice; Decided on microservices; This affects all future development; critical</memory_long>` |
| `emotional` | Emotion-tagged memories | Memories associated with specific emotions | `<memory_happy>Success moment; Feature launched successfully; Team morale boost; high; long</memory_happy>` |

### Core Emotions

The system supports 20 core emotions for emotional memory tagging:

**Positive Emotions:**
- `happy` - Joy and satisfaction
- `excited` - Enthusiasm and energy
- `confident` - Self-assurance and belief
- `motivated` - Drive and determination
- `satisfied` - Contentment and fulfillment
- `grateful` - Appreciation and thankfulness
- `proud` - Achievement and accomplishment
- `hopeful` - Optimism and expectation

**Negative Emotions:**
- `sad` - Sorrow and disappointment
- `angry` - Frustration and irritation
- `fearful` - Anxiety and concern
- `surprised` - Unexpected events
- `disgusted` - Revulsion and disapproval
- `anxious` - Worry and unease
- `frustrated` - Irritation and hindrance
- `disappointed` - Let down and dismayed
- `ashamed` - Embarrassment and regret
- `resigned` - Acceptance of defeat

**Neutral Emotions:**
- `overwhelmed` - Feeling overburdened
- `curious` - Interest and inquiry
