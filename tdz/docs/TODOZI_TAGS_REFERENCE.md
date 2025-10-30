# Todozi Tags Reference

This document provides a formal list of all `<todozi>` tags and their required parameters that models would use to create tasks, memories, ideas, etc. Technically you could also in a tdzcnt active chat, but the cli and gui/tui are quicker. 

## Examples

### Tasks
<todozi>Action; Time; Priority; Project; Status; assignee=human; tags=tag1,tag2</todozi>
<todozi>Fix login bug; 2 hours; high; authentication; in_progress; assignee=ai; tags=security,bug</todozi>
<todozi>Write API documentation; 6 hours; medium; backend; todo; assignee=human; tags=documentation,api</todozi>
<todozi>Design database schema; 8 hours; high; infrastructure; blocked; assignee=collaborative; tags=database,design</todozi>
<todozi>Implement user notifications; 4 hours; medium; frontend; todo; assignee=ai; tags=ux,notifications</todozi>

### Agent Assignments
<todozi_agent>task123; agent456; review_code; important</todozi_agent>
<todozi_agent>task789; ai_assistant; debug_error; urgent</todozi_agent>
<todozi_agent>task456; code_reviewer; quality_check; standard</todozi_agent>
<todozi_agent>task101; test_runner; validate_deployment; critical</todozi_agent>
<todozi_agent>task202; design_expert; ui_review; medium</todozi_agent>

### Memories
<memory>standard; What happened; What it means; Why it matters; high; long; tags</memory>
<memory>happy; Completed project milestone; Achievement feels great; Motivation for future work; high; long; success,productivity</memory>
<memory>secret; Private conversation details; Confidential information; Keep secure; critical; long; confidential</memory>
<memory>short; Quick reminder note; Temporary information; Will forget soon; low; short; temp</memory>
<memory>frustrated; API integration failed; Error handling is crucial; Always implement proper error handling; high; long; debugging,api</memory>

### Ideas
<idea>Idea content; share; importance; context; tags</idea>
<idea>Voice-controlled interface; team; high; Could revolutionize mobile UX; innovation,accessibility</idea>
<idea>AI-powered code reviews; public; breakthrough; Transform development workflow; ai,productivity</idea>
<idea>Real-time collaboration tools; team; medium; Improve team communication; collaboration,tools</idea>
<idea>Automated testing framework; private; high; Reduce manual testing time; development,qa</idea>

### Code Chunks
<chunk>id; level; description; dependencies; code</chunk>
<chunk>auth_module; module; User authentication system; user_model; class AuthManager</chunk>
<chunk>api_routes; class; REST API endpoints; auth_module; @app.route('/api/users')</chunk>
<chunk>db_connection; method; Database connection handler; config; def get_db_connection()</chunk>
<chunk>error_handler; block; Global error handling; logging; try: ... except Exception</chunk>

### Errors
<error>title; description; severity; category; source; context; tags</error>
<error>Database connection failed; Connection timeout after 30 seconds; critical; database; db_service; Check connection pool settings; database,timeout</error>
<error>API authentication error; Invalid JWT token format; high; security; auth_middleware; Implement token validation; security,jwt</error>
<error>Memory leak detected; Application memory usage growing; medium; performance; memory_manager; Add garbage collection; performance,memory</error>
<error>File upload failed; File size exceeds limit; low; validation; upload_handler; Increase file size limit; upload,validation</error>

### Training Data
<train>data_type; prompt; completion; context; tags; quality_score; source</train>
<train>instruction; Write a hello world function; def hello_world(): return "Hello, World!"; Basic Python syntax; python,basics; 0.95; tutorial</train>
<train>code_example; Implement binary search; def binary_search(arr, target): ...; Algorithm implementation; algorithms,search; 0.9; leetcode</train>
<train>conversation; How to handle errors; Use try-except blocks and log errors; Error handling best practices; python,errors; 0.85; documentation</train>
<train>completion; Complete the SQL query; SELECT * FROM users WHERE; SELECT * FROM users WHERE active = 1; Database queries; sql,database; 0.8; examples</train>

### Feelings
<feel>emotion; intensity; description; context; tags</feel>
<feel>excited; 9; Just launched the new feature successfully!; product launch; achievement,success</feel>
<feel>frustrated; 7; Bug took 3 hours to find and fix; debugging session; debugging,patience</feel>
<feel>confident; 8; Code review went perfectly; team collaboration; leadership,communication</feel>
<feel>overwhelmed; 6; Too many tasks, need to prioritize; project planning; organization,time-management</feel>
<feel>proud; 9; Team delivered ahead of schedule; project completion; teamwork,achievement</feel>

### Summaries
<summary>content; priority; context; tags</summary>
<summary>Successfully deployed v2.0 with zero downtime; critical; Major milestone achieved; deployment,success,reliability</summary>
<summary>Completed user testing phase; high; Gathered valuable feedback; testing,user-research,ux</summary>
<summary>Security audit passed with no issues; high; System security validated; security,compliance</summary>
<summary>Team productivity increased 25%; medium; Process improvements working; productivity,metrics</summary>

### Reminders
<reminder>content; remind_at; priority; status; tags</reminder>
<reminder>Team standup meeting; 2025-01-17T09:00:00Z; high; pending; meeting,daily,team</reminder>
<reminder>Submit quarterly report; 2025-01-31T17:00:00Z; medium; pending; reporting,deadline</reminder>
<reminder>Code review for pull request #123; 2025-01-18T14:00:00Z; high; pending; code-review,development</reminder>
<reminder>Client presentation preparation; 2025-01-20T10:00:00Z; critical; pending; presentation,client</reminder>

### System Responses
<tdz_sys>content</tdz_sys>
<tdz_sys>Great job! I've processed the following items: • Task: Implement user login • Memory: First successful deployment</tdz_sys>
<tdz_sys>System maintenance scheduled for tonight at 2 AM UTC</tdz_sys>
<tdz_sys>Backup completed successfully - 1.2GB data archived</tdz_sys>
<tdz_sys>Security scan found 3 vulnerabilities requiring attention</tdz_sys>

### General Tdz Tags
<tdz>content</tdz>
<tdz>Process this message and extract all relevant todozi tags</tdz>
<tdz>Analyze project requirements and create implementation plan</tdz>
<tdz>Review code quality and suggest improvements</tdz>
<tdz>Generate documentation for the new API endpoints</tdz>

### Shorthand Tags
<tz>Fix critical bug in authentication; 2h; urgent; security; in_progress</tz>
<mm>frustrated; Debug session took 4 hours; Need better error logging; medium; long; debugging,productivity</mm>
<id>Implement dark mode toggle; team; medium; Improve user experience; ux,accessibility</id>
<ch>auth_service; module; User authentication and authorization; user_model,database; class AuthService</ch>
<fe>excited; 8; Just got approval for the new feature release!; product meeting; success,teamwork</fe>
<tn>instruction; Write a REST API endpoint; @app.route('/api/users', methods=['GET']); API development; python,flask; 0.9; tutorial</tn>
<er>Database timeout; Connection pool exhausted; critical; database; db_handler; Increase pool size; performance,database</er>
<sm>Weekly sprint completed ahead of schedule; high; Team velocity increased 15%; agile,success</sm>
<rd>Team retrospective meeting; 2025-01-19T15:00:00Z; medium; pending; retrospective,team</rd>

## Task Tags

### `<todozi>` - Task Definition
**Required Parameters (minimum 5):**
- `action` (string) - The task description/action to perform
- `time` (string) - Time estimate or deadline
- `priority` (enum: low, medium, high, critical) - Task priority level
- `parent_project` (string) - Project this task belongs to
- `status` (enum: todo, in_progress, done, blocked, deferred) - Current task status

**Optional Parameters:**
- `assignee` (enum: human, ai, collaborative, agent:{name}) - Who should execute this task
- `tags` (comma-separated strings) - Task tags for categorization
- `dependencies` (comma-separated strings) - Task IDs this task depends on
- `context_notes` (string) - Additional context or notes
- `progress` (integer 0-100) - Completion percentage

**Example:**
```xml
<todozi>Implement OAuth2 login flow; 6 hours; high; python-web-framework; todo; assignee=human; tags=auth,backend; dependencies=Design API; context_notes=Ensure security; progress=0%</todozi>
```

### `<todozi_agent>` - Agent Assignment
**Required Parameters (minimum 3):**
- `agent_id` (string) - Identifier of the agent to assign
- `task_id` (string) - ID of the task to assign
- `project_id` (string) - ID of the project

**Example:**
```xml
<todozi_agent>task123; agent456; review_code; important</todozi_agent>
```

## Memory Tags

### `<memory>` - Unified Memory (All Types)
**Required Parameters (minimum 6):**
- `type` (string/enum: standard, secret, human, short, long, or emotion name) - Memory type
- `moment` (string) - What happened
- `meaning` (string) - What it means
- `reason` (string) - Why it matters
- `importance` (enum: low, medium, high, critical) - Importance level
- `term` (enum: short, long) - Memory retention term

**Optional Parameters:**
- `tags` (comma-separated strings) - Memory tags

**Memory Types:**
- `standard` - General memory
- `secret` - Sensitive/private memory
- `human` - Human-related memory
- `short` - Short-term memory (term automatically set to short)
- `long` - Long-term memory (term automatically set to long)
- Emotion names: `happy`, `sad`, `angry`, `fearful`, `surprised`, `disgusted`, `excited`, `anxious`, `confident`, `frustrated`, `motivated`, `overwhelmed`, `curious`, `satisfied`, `disappointed`, `grateful`, `proud`, `ashamed`, `hopeful`, `resigned`

**Examples:**
```xml
<memory>standard; First insight; This is an important insight; High value information; high; long; insight,valuable</memory>
<memory>secret; Private conversation; Confidential information; Need to keep secure; high; long; confidential</memory>
<memory>happy; Completed project milestone; Feels great to accomplish goals; Motivation boost; high; long; achievement,success</memory>
<memory>short; Temporary note; Quick reminder; Will forget soon; low; short; temp</memory>
```

## Idea Tags

### `<idea>` - Idea Capture
**Required Parameters (minimum 3):**
- `idea` (string) - The idea content
- `share` (enum: share/dont share/don't share/private/team) - Sharing level
- `importance` (enum: low, medium, high, critical) - Idea importance

**Example:**
```xml
<idea>Revolutionary approach; high; This could change everything</idea>
```

## Code Chunking Tags

### `<chunk>` - Code Chunk Definition
**Required Parameters (minimum 3):**
- `id` (string) - Unique chunk identifier
- `level` (enum: project, module, class, method, block) - Chunking level
- `description` (string) - Description of the chunk

**Optional Parameters:**
- `dependencies` (comma-separated strings) - Chunk dependencies
- `code` (string) - Actual code content

**Example:**
```xml
<chunk>chunk1; module; Create database handler; chunk0; import sqlite3</chunk>
```

## Error Tags

### `<error>` - Error Logging
**Required Parameters (minimum 5):**
- `title` (string) - Error title
- `description` (string) - Error description
- `severity` (enum: low, medium, high, critical) - Error severity
- `category` (enum: network, database, security, performance, logic, configuration, external_service) - Error category
- `source` (string) - Error source/component

**Optional Parameters:**
- `context` (string) - Additional context
- `tags` (comma-separated strings) - Error tags

**Example:**
```xml
<error>Database connection failed; Unable to connect to PostgreSQL database; critical; network; database-service; Connection timeout after 30 seconds; database,postgres,connection</error>
```

## Training Data Tags

### `<train>` - Training Data
**Required Parameters (minimum 4):**
- `data_type` (enum: instruction, completion, conversation, code_example) - Type of training data
- `prompt` (string) - The prompt/input
- `completion` (string) - The completion/output
- `context` (string) - Context information

**Optional Parameters:**
- `tags` (comma-separated strings) - Training data tags
- `quality_score` (float 0.0-1.0) - Quality score
- `source` (string) - Data source

**Example:**
```xml
<train>instruction; Write a function to calculate fibonacci numbers; def fibonacci(n):\n    if n <= 1:\n        return n\n    return fibonacci(n-1) + fibonacci(n-2); Python programming example; python,algorithm,recursion; 0.9; code-examples</train>
```

## Feeling Tags

### `<feel>` - Emotional State
**Required Parameters (minimum 3):**
- `emotion` (string) - Emotion name
- `intensity` (integer 1-10) - Emotion intensity
- `description` (string) - Description of the feeling

**Optional Parameters:**
- `context` (string) - Context where feeling occurred
- `tags` (comma-separated strings) - Feeling tags

**Example:**
```xml
<feel>excited; 9; Making great progress on this project!; coding session; productive,happy</feel>
```

## Summary Tags

### `<summary>` - Content Summary
**Required Parameters (minimum 2):**
- `content` (string) - Summary content
- `priority` (enum: low, medium, high, critical) - Summary priority

**Optional Parameters:**
- `context` (string) - Additional context
- `tags` (comma-separated strings) - Summary tags

**Example:**
```xml
<summary>Project completed successfully; high; Final project delivery; project,completion,success</summary>
```

## Reminder Tags

### `<reminder>` - Reminder Setting
**Required Parameters (minimum 3):**
- `content` (string) - Reminder content
- `remind_at` (ISO 8601 datetime) - When to remind
- `priority` (enum: low, medium, high, critical) - Reminder priority

**Optional Parameters:**
- `status` (enum: pending, active, completed, cancelled) - Reminder status (default: pending)
- `tags` (comma-separated strings) - Reminder tags

**Example:**
```xml
<reminder>Team meeting at 3 PM; 2025-01-17T15:00:00Z; high; pending; meeting,team</reminder>
```

## Shorthand Tags

The following shorthand tags are automatically transformed to their full forms:

- `<tz>` → `<todozi>`
- `<mm>` → `<memory>`
- `<id>` → `<idea>`
- `<ch>` → `<chunk>`
- `<fe>` → `<feel>`
- `<tn>` → `<train>`
- `<er>` → `<error>`
- `<sm>` → `<summary>`
- `<rd>` → `<reminder>`
- `<tdz>` → `<tdz>` (appears to be a duplicate)

## Parameter Format

All tags use semicolon (`;`) as parameter separators. Parameters should be trimmed of whitespace. Optional parameters that are empty or missing will use default values or be set to `None`.

## Validation

Each tag has minimum parameter requirements. If these are not met, parsing will fail with a validation error indicating what's missing.



#JSON 

{
  "type": "function",
  "function": {
    "name": "create_task",
    "description": "Create a new task in the Todozi system with automatic AI assignment and queue management. Use this when users mention tasks, todos, or things they need to do.",
    "parameters": {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "description": "Task description/action to perform. Be specific and actionable.",
          "examples": [
            "Implement user authentication system",
            "Write API documentation",
            "Fix login bug in mobile app",
            "Design database schema for e-commerce platform"
          ]
        },
        "time": {
          "type": "string",
          "description": "Time estimate (e.g., '2 hours', '1 day', '1 week'). Optional but helpful for planning.",
          "examples": ["2 hours", "1 day", "3 days", "1 week"]
        },
        "priority": {
          "type": "string",
          "enum": ["low", "medium", "high", "critical", "urgent"],
          "description": "Priority level. Use 'urgent' or 'critical' for time-sensitive issues.",
          "examples": ["low", "medium", "high", "urgent", "critical"]
        },
        "project": {
          "type": "string",
          "description": "Project name for organization. Use existing projects or create new ones.",
          "examples": ["website_redesign", "mobile_app", "api_development", "infrastructure"]
        },
        "assignee": {
          "type": "string",
          "enum": ["ai", "human", "collaborative"],
          "description": "Who should handle this task. 'ai' for AI processing, 'human' for manual work, 'collaborative' for both.",
          "examples": ["ai", "human", "collaborative"]
        },
        "tags": {
          "type": "string",
          "description": "Comma-separated tags for categorization and search.",
          "examples": ["frontend,ui,design", "backend,api,database", "testing,q&a", "documentation"]
        },
        "context": {
          "type": "string",
          "description": "Additional context, requirements, or background information.",
          "examples": [
            "This task requires knowledge of React and TypeScript",
            "Must follow our existing design system",
            "Coordinate with the backend team for API changes"
          ]
        }
      },
      "required": ["action"],
      "examples": [
        {
          "description": "Create a high-priority task for AI processing",
          "example": {
            "action": "Implement user authentication system",
            "time": "4 hours",
            "priority": "high",
            "project": "security_upgrade",
            "assignee": "ai",
            "tags": "security,authentication,backend",
            "context": "Must integrate with existing user management system"
          }
        },
        {
          "description": "Create a collaborative task for human-AI work",
          "example": {
            "action": "Design new landing page",
            "time": "2 days",
            "priority": "medium",
            "project": "marketing_site",
            "assignee": "collaborative",
            "tags": "design,frontend,marketing",
            "context": "Should match our brand guidelines and be mobile-responsive"
          }
        },
        {
          "description": "Simple task creation",
          "example": {
            "action": "Fix typo in README",
            "priority": "low",
            "assignee": "human",
            "tags": "documentation"
          }
        }
      ]
    }
  }
},
{
  "type": "function",
  "function": {
    "name": "create_idea",
    "description": "Create a new creative idea or concept. Use this to capture innovative thoughts, potential features, or creative solutions that might be valuable in the future.",
    "parameters": {
      "type": "object",
      "properties": {
        "idea": {
          "type": "string",
          "description": "The idea content or concept description. Be creative and descriptive.",
          "examples": [
            "Implement voice-controlled task management",
            "Create a gamified learning platform for developers",
            "Build an AI-powered code review assistant",
            "Design a collaborative workspace with real-time mind mapping"
          ]
        },
        "share": {
          "type": "string",
          "enum": ["private", "team", "public"],
          "description": "Who should be able to see this idea. Private for personal ideas, team for collaboration, public for broader sharing.",
          "examples": ["private", "team", "public"]
        },
        "importance": {
          "type": "string",
          "enum": ["low", "medium", "high", "breakthrough"],
          "description": "How significant is this idea. Breakthrough for potentially transformative concepts.",
          "examples": ["low", "medium", "high", "breakthrough"]
        },
        "tags": {
          "type": "string",
          "description": "Comma-separated tags for categorization and discovery.",
          "examples": [
            "innovation,product,ai",
            "ux,design,mobile",
            "development,tools,productivity",
            "business,monetization,strategy"
          ]
        },
        "context": {
          "type": "string",
          "description": "Additional context, inspiration, or background for the idea.",
          "examples": [
            "Inspired by seeing similar features in competitor products",
            "Came up during brainstorming about user engagement",
            "Based on customer feedback about current limitations",
            "Technical feasibility confirmed through recent research"
          ]
        }
      },
      "required": ["idea"],
      "examples": [
        {
          "description": "Create a breakthrough idea for public sharing",
          "example": {
            "idea": "AI-powered code review assistant that learns from team patterns and suggests improvements",
            "share": "public",
            "importance": "breakthrough",
            "tags": "ai,development,productivity,collaboration",
            "context": "Could revolutionize how development teams work together"
          }
        },
        {
          "description": "Create a team-shared idea for collaboration",
          "example": {
            "idea": "Implement real-time collaborative mind mapping for project planning",
            "share": "team",
            "importance": "high",
            "tags": "collaboration,planning,ux",
            "context": "Team expressed frustration with current planning tools"
          }
        },
        {
          "description": "Simple private idea capture",
          "example": {
            "idea": "Add dark mode toggle to all applications",
            "share": "private",
            "importance": "medium",
            "tags": "ux,accessibility,design"
          }
        }
      ]
    }
  }
},
{
  "type": "function",
  "function": {
    "name": "create_memory",
    "description": "Create a new memory for learning and context. Use this to capture important lessons, experiences, or knowledge that should be remembered for future reference.",
    "parameters": {
      "type": "object",
      "properties": {
        "moment": {
          "type": "string",
          "description": "What happened or what was learned (the key moment or insight).",
          "examples": [
            "Discovered that async operations need proper error handling",
            "Learned that database indexes improve query performance significantly",
            "Found that mobile users prefer swipe gestures over buttons"
          ]
        },
        "meaning": {
          "type": "string",
          "description": "What it means or why it's important. The deeper significance or implications.",
          "examples": [
            "This pattern prevents data corruption in concurrent systems",
            "This optimization could save hours of processing time",
            "This insight will improve user experience across our platform"
          ]
        },
        "reason": {
          "type": "string",
          "description": "Why this should be remembered. The reason for capturing this memory.",
          "examples": [
            "Will apply this to all future API designs",
            "Important for performance reviews and architecture decisions",
            "Should influence our mobile design guidelines"
          ]
        },
        "importance": {
          "type": "string",
          "enum": ["low", "medium", "high", "critical"],
          "description": "How important is this memory for future reference.",
          "examples": ["low", "medium", "high", "critical"]
        },
        "term": {
          "type": "string",
          "enum": ["short", "long"],
          "description": "How long to remember this. Short-term for immediate projects, long-term for lasting knowledge.",
          "examples": ["short", "long"]
        },
        "tags": {
          "type": "string",
          "description": "Comma-separated tags for categorization and search.",
          "examples": [
            "architecture,performance,scalability",
            "ux,design,mobile,user-research",
            "development,debugging,problem-solving"
          ]
        }
      },
      "required": ["moment", "meaning", "reason"],
      "examples": [
        {
          "description": "Create an important long-term memory about a technical lesson",
          "example": {
            "moment": "Found that database connection pooling prevents timeout errors",
            "meaning": "Proper resource management is crucial for system stability",
            "reason": "Will apply this to all future database implementations",
            "importance": "high",
            "term": "long",
            "tags": "database,performance,architecture"
          }
        },
        {
          "description": "Create a critical short-term memory for immediate project needs",
          "example": {
            "moment": "Client specifically requested dark mode support",
            "meaning": "User preferences should drive feature prioritization",
            "reason": "Important for current project requirements",
            "importance": "critical",
            "term": "short",
            "tags": "ux,requirements,client-feedback"
          }
        },
        {
          "description": "Simple memory creation",
          "example": {
            "moment": "React hooks must be called at the top level",
            "meaning": "Following the rules of hooks prevents bugs",
            "reason": "Common mistake that causes runtime errors",
            "importance": "medium",
            "tags": "react,frontend,best-practices"
          }
        }
      ]
    }
  }
},
{
  "type": "function",
  "function": {
    "name": "search_tasks",
    "description": "Search for tasks in the Todozi system with semantic AI capabilities. Use this when users want to find existing tasks, check status, or review work.",
    "parameters": {
      "type": "object",
      "properties": {
        "query": {
          "type": "string",
          "description": "Search query to match against task content. Can be keywords or natural language.",
          "examples": [
            "authentication bugs",
            "API documentation",
            "mobile app features",
            "database optimization"
          ]
        },
        "semantic": {
          "type": "boolean",
          "description": "Use AI semantic search instead of keyword matching. Better for natural language queries.",
          "default": false,
          "examples": [true, false]
        },
        "project": {
          "type": "string",
          "description": "Filter by project name to narrow search scope.",
          "examples": ["website_redesign", "mobile_app", "api_development"]
        },
        "status": {
          "type": "string",
          "enum": ["todo", "in_progress", "blocked", "review", "done"],
          "description": "Filter by task status.",
          "examples": ["todo", "in_progress", "done", "blocked"]
        },
        "assignee": {
          "type": "string",
          "enum": ["ai", "human", "collaborative"],
          "description": "Filter by assignee type.",
          "examples": ["ai", "human", "collaborative"]
        },
        "limit": {
          "type": "number",
          "description": "Maximum number of results to return. Default is 10.",
          "default": 10,
          "examples": [5, 10, 20, 50]
        }
      },
      "required": ["query"],
      "examples": [
        {
          "description": "Semantic search for similar tasks",
          "example": {
            "query": "user login problems",
            "semantic": true,
            "limit": 5
          }
        },
        {
          "description": "Find all completed tasks in a project",
          "example": {
            "query": "security",
            "project": "security_upgrade",
            "status": "done",
            "limit": 20
          }
        },
        {
          "description": "Quick keyword search",
          "example": {
            "query": "API",
            "assignee": "ai",
            "limit": 10
          }
        },
        {
          "description": "Find blocked tasks",
          "example": {
            "query": "",
            "status": "blocked",
            "limit": 15
          }
        }
      ]
    }
  }
},
{
  "type": "function",
  "function": {
    "name": "simple_todozi",
    "description": "Ultra-simple Todozi interface with automatic AI/human coordination and smart search. The easiest way to interact with Todozi - just specify what you want to do.",
    "parameters": {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "enum": [
            "task", "urgent", "high", "low", "ai", "human", "collab",
            "find", "ai_search", "fast_search", "smart_search",
            "remember", "important_memory", "idea", "breakthrough_idea",
            "complete", "start", "stats", "queue", "chat",
            "extract", "expand", "plan", "strategy"
          ],
          "description": "🚀 SIMPLE ACTIONS: task=create task, urgent=urgent task, find=search everything, remember=save memory, idea=save idea, complete=finish task, start=begin task, stats=get overview, ai=AI task, human=human task, collab=collaborative task, extract=AI extract tasks from text, expand=AI expand task into subtasks, plan=AI plan complex projects, strategy=AI strategic planning & enhancement"
        },
        "content": {
          "type": "string",
          "description": "📝 WHAT TO DO: The main content - task description, search query, memory text, idea text, or task ID to complete/start"
        },
        "extra": {
          "type": "string",
          "description": "💡 OPTIONAL EXTRAS: Additional context, meaning for memories, project name, or any extra details"
        }
      },
      "required": ["action", "content"],
      "examples": [
        {
          "description": "Create a simple task",
          "example": {"action": "task", "content": "Fix the login bug"}
        },
        {
          "description": "Create urgent task",
          "example": {"action": "urgent", "content": "Server is down - fix immediately"}
        },
        {
          "description": "Search everything with AI + keywords",
          "example": {"action": "find", "content": "authentication issues"}
        },
        {
          "description": "AI-only semantic search",
          "example": {"action": "ai_search", "content": "similar to user management"}
        },
        {
          "description": "Remember something important",
          "example": {"action": "remember", "content": "User prefers dark mode", "extra": "UI design preference"}
        },
        {
          "description": "Save breakthrough idea",
          "example": {"action": "breakthrough_idea", "content": "Voice-controlled task manager"}
        },
        {
          "description": "Complete a task",
          "example": {"action": "complete", "content": "task_12345"}
        },
        {
          "description": "Get quick overview",
          "example": {"action": "stats", "content": ""}
        },
        {
          "description": "Create AI task (queued for AI systems)",
          "example": {"action": "ai", "content": "Analyze code performance bottlenecks"}
        },
        {
          "description": "Create human task (appears in TUI)",
          "example": {"action": "human", "content": "Review pull request #123"}
        },
        {
          "description": "Process chat with Todozi tags",
          "example": {"action": "chat", "content": "I need to <todozi>fix bug; 2h; high; myproject; todo</todozi> and remember this"}
        },
        {
          "description": "Extract tasks from text using todozi.com AI",
          "example": {"action": "extract", "content": "I need to build a web app with authentication, payments, and email notifications"}
        },
        {
          "description": "Expand task into subtasks using todozi.com AI",
          "example": {"action": "expand", "content": "Build user authentication system", "extra": "for a Rust web application"}
        },
        {
          "description": "AI project planning with comprehensive task breakdown",
          "example": {"action": "plan", "content": "Build a complete e-commerce platform", "extra": "with payment integration and inventory management"}
        },
        {
          "description": "AI strategic planning with enhanced analysis",
          "example": {"action": "strategy", "content": "Optimize our development workflow", "extra": "for a team of 5 developers using agile methodology"}
        }
      ]
    }
  }
},
{
  "type": "function",
  "function": {
    "name": "update_task",
    "description": "Update an existing task in the Todozi system. Use this to change status, progress, priority, or other task properties.",
    "parameters": {
      "type": "object",
      "properties": {
        "task_id": {
          "type": "string",
          "description": "ID of the task to update. Required - you must know the specific task ID.",
          "examples": ["task_12345", "abc-123-def-456"]
        },
        "status": {
          "type": "string",
          "enum": ["todo", "in_progress", "blocked", "review", "done"],
          "description": "New status for the task. Use 'done' to complete, 'in_progress' to start working.",
          "examples": ["todo", "in_progress", "blocked", "review", "done"]
        },
        "progress": {
          "type": "number",
          "description": "Progress percentage (0-100). Use this to track completion progress.",
          "minimum": 0,
          "maximum": 100,
          "examples": [25, 50, 75, 100]
        },
        "priority": {
          "type": "string",
          "enum": ["low", "medium", "high", "critical", "urgent"],
          "description": "New priority level for the task.",
          "examples": ["low", "medium", "high", "urgent", "critical"]
        },
        "assignee": {
          "type": "string",
          "enum": ["ai", "human", "collaborative"],
          "description": "Change who is assigned to handle this task.",
          "examples": ["ai", "human", "collaborative"]
        },
        "context": {
          "type": "string",
          "description": "Additional context, notes, or updated requirements for the task.",
          "examples": [
            "Actually, this needs to be done by Friday",
            "Found additional requirements during investigation",
            "Blocked by dependency on user management system"
          ]
        }
      },
      "required": ["task_id"],
      "examples": [
        {
          "description": "Mark a task as completed",
          "example": {
            "task_id": "task_12345",
            "status": "done"
          }
        },
        {
          "description": "Start working on a task",
          "example": {
            "task_id": "abc-123-def-456",
            "status": "in_progress",
            "progress": 25
          }
        },
        {
          "description": "Update task priority and add context",
          "example": {
            "task_id": "task_67890",
            "priority": "urgent",
            "context": "Client meeting moved up to tomorrow morning"
          }
        },
        {
          "description": "Mark task as blocked with explanation",
          "example": {
            "task_id": "xyz-789-abc-123",
            "status": "blocked",
            "context": "Waiting for API documentation from external vendor"
          }
        }
      ]
    }
  }
}