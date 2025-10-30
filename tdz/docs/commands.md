# Todozi Commands Reference

This document provides a comprehensive reference for all Todozi CLI commands and their usage.

## 📋 Basic Operations

### Initialize System
```bash
todozi init
```
Initializes the Todozi system by creating the `~/.todozi/` directory structure and default configuration.

### Add Task
```bash
todozi add task <action> [options]
```

**Required Arguments:**
- `<action>` - Task description

**Options:**
- `-t, --time <time>` - Time estimate (e.g., "2 hours", "1 day")
- `--priority <priority>` - Priority level (low/medium/high/critical/urgent)
- `--project <project>` - Project name to associate with task
- `-s, --status <status>` - Status (todo/in_progress/completed/cancelled) [default: todo]
- `-u, --assignee <assignee>` - Assignee (ai/human/collaborative or specific agent)
- `--tags <tags>` - Comma-separated tags for the task
- `--dependencies <dependencies>` - Comma-separated task IDs this task depends on
- `-c, --context <context>` - Additional context or notes
- `-p, --progress <progress>` - Progress percentage (0-100)

**Example:**
```bash
todozi add task "Fix the login bug" --time "2 hours" --priority high --project webapp
```

### List Tasks
```bash
todozi list tasks [options]
```

**Options:**
- `-j, --project <project>` - Filter by project name
- `--status <status>` - Filter by status (todo/in_progress/completed/cancelled)
- `--priority <priority>` - Filter by priority (low/medium/high/critical/urgent)
- `-u, --assignee <assignee>` - Filter by assignee
- `-g, --tags <tags>` - Filter by tags (comma-separated)
- `-s, --search <search>` - Search in task descriptions

**Example:**
```bash
todozi list tasks --project webapp --status todo --priority high
```

### Show Task Details
```bash
todozi show task <id>
```

**Required Arguments:**
- `<id>` - Task ID to display

### Update Task
```bash
todozi update [options] <id>
```

**Required Arguments:**
- `<id>` - Task ID to update

**Options:**
- `-a, --action <action>` - Updated task description
- `-t, --time <time>` - Updated time estimate
- `-r, --priority <priority>` - Updated priority level
- `-j, --project <project>` - Updated project name
- `-s, --status <status>` - Updated status
- `-u, --assignee <assignee>` - Updated assignee
- `-g, --tags <tags>` - Updated tags (comma-separated)
- `-d, --dependencies <dependencies>` - Updated dependencies (comma-separated task IDs)
- `-c, --context <context>` - Updated context or notes
- `-p, --progress <progress>` - Updated progress percentage (0-100)

**Example:**
```bash
todozi update task_12345 --status in_progress --progress 50
```

### Complete Task
```bash
todozi complete <id>
```

**Required Arguments:**
- `<id>` - Task ID to mark as completed

### Delete Task
```bash
todozi delete <id>
```

**Required Arguments:**
- `<id>` - Task ID to delete

## 📁 Project Management

### Create Project
```bash
todozi project create <name> [options]
```

**Required Arguments:**
- `<name>` - Project name

**Options:**
- `-d, --description <description>` - Project description

### List Projects
```bash
todozi project list
```

### Show Project Details
```bash
todozi project show <name>
```

**Required Arguments:**
- `<name>` - Project name

### Archive Project
```bash
todozi project archive <name>
```

**Required Arguments:**
- `<name>` - Project name

### Update Project
```bash
todozi project update <name> [options]
```

**Required Arguments:**
- `<name>` - Project name

**Options:**
- `-n, --new-name <new-name>` - New project name
- `-d, --description <description>` - Updated description
- `-s, --status <status>` - Updated status

### Delete Project
```bash
todozi project delete <name>
```

**Required Arguments:**
- `<name>` - Project name

## 🤖 Agent Management

### List Agents
```bash
todozi agent list
```

### Show Agent Details
```bash
todozi agent show <id>
```

**Required Arguments:**
- `<id>` - Agent ID

### Create Enhanced Agent
```bash
todozi agent create [options] <id> <name> <description>
```

**Required Arguments:**
- `<id>` - Agent ID
- `<name>` - Agent name
- `<description>` - Agent description

**Options:**
- `-c, --category <category>` - Agent category (technical/creative/management/general) [default: general]
- `--capabilities <capabilities>` - Comma-separated list of capabilities
- `-s, --specializations <specializations>` - Comma-separated list of specializations
- `-p, --model-provider <model-provider>` - Model provider [default: todozi]
- `--model-name <model-name>` - Model name [default: baton]
- `--temperature <temperature>` - Model temperature [default: 0.2]
- `--max-tokens <max-tokens>` - Maximum tokens [default: 4096]
- `--tags <tags>` - Comma-separated tags
- `--system-prompt <system-prompt>` - Custom system prompt
- `--prompt-template <prompt-template>` - Custom prompt template
- `--auto-format-code <auto-format-code>` - Auto-format code (true/false)
- `--include-examples <include-examples>` - Include examples (true/false)
- `--explain-complexity <explain-complexity>` - Explain complexity (true/false)
- `--suggest-tests <suggest-tests>` - Suggest tests (true/false)
- `--tools <tools>` - Comma-separated list of tools
- `--max-response-length <max-response-length>` - Maximum response length
- `--timeout-seconds <timeout-seconds>` - Timeout in seconds
- `--requests-per-minute <requests-per-minute>` - Requests per minute
- `--tokens-per-hour <tokens-per-hour>` - Tokens per hour

### Update Agent
```bash
todozi agent update [options] <id>
```

**Required Arguments:**
- `<id>` - Agent ID

**Options:**
- `-n, --name <name>` - Updated agent name
- `-d, --description <description>` - Updated description
- `-c, --category <category>` - Updated category
- `--capabilities <capabilities>` - Updated capabilities
- `-s, --specializations <specializations>` - Updated specializations
- `--system-prompt <system-prompt>` - Updated system prompt
- `--prompt-template <prompt-template>` - Updated prompt template
- `-p, --model-provider <model-provider>` - Updated model provider
- `--model-name <model-name>` - Updated model name
- `--temperature <temperature>` - Updated temperature
- `--max-tokens <max-tokens>` - Updated maximum tokens
- `--tags <tags>` - Updated tags
- `--auto-format-code <auto-format-code>` - Updated auto-format code setting
- `--include-examples <include-examples>` - Updated include examples setting
- `--explain-complexity <explain-complexity>` - Updated explain complexity setting
- `--suggest-tests <suggest-tests>` - Updated suggest tests setting
- `--tools <tools>` - Updated tools
- `--max-response-length <max-response-length>` - Updated maximum response length
- `--timeout-seconds <timeout-seconds>` - Updated timeout
- `--requests-per-minute <requests-per-minute>` - Updated requests per minute
- `--tokens-per-hour <tokens-per-hour>` - Updated tokens per hour

### Delete Agent
```bash
todozi agent delete <id>
```

**Required Arguments:**
- `<id>` - Agent ID

### Assign Task to Agent
```bash
todozi agent assign <agent_id> <task_id> <project_id>
```

**Required Arguments:**
- `<agent_id>` - Agent ID
- `<task_id>` - Task ID
- `<project_id>` - Project ID

## 🧠 Memory Management

### List Memories
```bash
todozi memory list [options]
```

**Options:**
- `-r, --importance <importance>` - Filter by importance (low/medium/high)
- `-t, --term <term>` - Filter by term (short/medium/long)
- `-T, --memory-type <memory-type>` - Filter by memory type

### Create Memory
```bash
todozi memory create [options] <moment> <meaning> <reason>
```

**Required Arguments:**
- `<moment>` - What happened
- `<meaning>` - What it means
- `<reason>` - Why it matters

**Options:**
- `-r, --importance <importance>` - Importance level (low/medium/high) [default: medium]
- `-t, --term <term>` - Term (short/medium/long) [default: short]
- `-T, --memory-type <memory-type>` - Memory type (standard/secret/human/emotional) [default: standard]
- `-g, --tags <tags>` - Comma-separated tags
- `--emotion <emotion>` - Emotion (for emotional memories)

**Examples:**
```bash
# Standard memory
todozi memory create "User prefers dark mode" "UI customization preference" "Important for user experience" --importance high --term long

# Emotional memory
todozi memory create "User was frustrated with login" "Negative experience with authentication" "Need to improve UX" --importance high --term short --emotion frustration
```

### Show Memory Details
```bash
todozi memory show <id>
```

**Required Arguments:**
- `<id>` - Memory ID

### Create Secret Memory
```bash
todozi memory create-secret [options] <moment> <meaning> <reason>
```

**Required Arguments:**
- `<moment>` - What happened
- `<meaning>` - What it means
- `<reason>` - Why it matters

**Options:**
- `-r, --importance <importance>` - Importance level (low/medium/high) [default: medium]
- `-t, --term <term>` - Term (short/medium/long) [default: short]
- `-g, --tags <tags>` - Comma-separated tags

### Create Human Memory
```bash
todozi memory create-human [options] <moment> <meaning> <reason>
```

**Required Arguments:**
- `<moment>` - What happened
- `<meaning>` - What it means
- `<reason>` - Why it matters

**Options:**
- `-r, --importance <importance>` - Importance level (low/medium/high) [default: high]
- `-t, --term <term>` - Term (short/medium/long) [default: long]
- `-g, --tags <tags>` - Comma-separated tags

### Create Emotional Memory
```bash
todozi memory create-emotional [options] <moment> <meaning> <reason> <emotion>
```

**Required Arguments:**
- `<moment>` - What happened
- `<meaning>` - What it means
- `<reason>` - Why it matters
- `<emotion>` - Emotion type

**Options:**
- `-r, --importance <importance>` - Importance level (low/medium/high) [default: medium]
- `-t, --term <term>` - Term (short/medium/long) [default: short]
- `-g, --tags <tags>` - Comma-separated tags

## 💡 Idea Management

### List Ideas
```bash
todozi idea list [options]
```

**Options:**
- `-s, --share <share>` - Filter by share setting (private/public)
- `-r, --importance <importance>` - Filter by importance (low/medium/high)

### Create Idea
```bash
todozi idea create [options] <idea>
```

**Required Arguments:**
- `<idea>` - Idea description

**Options:**
- `-s, --share <share>` - Share setting (private/public) [default: private]
- `-r, --importance <importance>` - Importance level (low/medium/high) [default: medium]
- `-g, --tags <tags>` - Comma-separated tags
- `-c, --context <context>` - Additional context

### Show Idea Details
```bash
todozi idea show <id>
```

**Required Arguments:**
- `<id>` - Idea ID

## 🔍 Search and Filtering

### Search Tasks
```bash
todozi search tasks <query>
```

**Required Arguments:**
- `<query>` - Search query

### Search All Content
```bash
todozi search-all [options] <query>
```

**Required Arguments:**
- `<query>` - Search query

**Options:**
- `-t, --types <types>` - Data types to search (tasks,memories,ideas,errors) [default: tasks,memories,ideas,errors]

### Show Statistics
```bash
todozi stats show
```

## 💾 Backup and Restore

### Create Backup
```bash
todozi backup create
```

### List Backups
```bash
todozi list-backups
```

### Restore from Backup
```bash
todozi restore <backup_name>
```

**Required Arguments:**
- `<backup_name>` - Name of backup to restore

## 🧩 Code Chunking Management

### List Code Chunks
```bash
todozi chunk list [options]
```

**Options:**
- `-p, --project <project>` - Filter by project
- `-s, --status <status>` - Filter by status
- `-l, --level <level>` - Filter by chunking level

### Create Code Chunk
```bash
todozi chunk create [options] <file_path>
```

**Required Arguments:**
- `<file_path>` - Path to source file

**Options:**
- `-p, --project <project>` - Project name
- `-l, --level <level>` - Chunking level (file/function/class/block)
- `-c, --context <context>` - Additional context

### Show Chunk Details
```bash
todozi chunk show <id>
```

**Required Arguments:**
- `<id>` - Chunk ID

### Show Dependency Graph
```bash
todozi chunk graph [options]
```

**Options:**
- `-p, --project <project>` - Filter by project
- `-f, --format <format>` - Output format (text/dot/json)

### Show Ready Chunks
```bash
todozi chunk ready [options]
```

**Options:**
- `-p, --project <project>` - Filter by project

## 🤖 AI Enhancement Commands

### Extract Tasks from Content
```bash
todozi extract [options] [content]
```

**Arguments:**
- `[content]` - Inline text content to extract tasks from

**Options:**
- `-f, --file <file>` - File path to extract content from
- `-o, --output <output>` - Output format (json/csv/md) [default: json]
- `--human` - Generate human-readable markdown checklist file

### Strategic Planning
```bash
todozi strategy [options] [content]
```

**Arguments:**
- `[content]` - Inline text content to strategize from

**Options:**
- `-f, --file <file>` - File path to strategize content from
- `-o, --output <output>` - Output format (json/csv/md) [default: json]
- `--human` - Generate human-readable markdown checklist file

### Chat with Todozi
```bash
todozi chat <message>
```

**Required Arguments:**
- `<message>` - Chat message

### Process Error
```bash
todozi error create [options] <title> <description> <source>
```

**Required Arguments:**
- `<title>` - Error title
- `<description>` - Error description
- `<source>` - Error source

**Options:**
- `-s, --severity <severity>` - Severity level (low/medium/high/critical) [default: medium]
- `-c, --category <category>` - Error category (runtime/logic/config/network) [default: runtime]
- `-c, --context <context>` - Additional context
- `-g, --tags <tags>` - Comma-separated tags

### List Errors
```bash
todozi error list [options]
```

**Options:**
- `-s, --severity <severity>` - Filter by severity
- `-c, --category <category>` - Filter by category
- `-u, --unresolved-only` - Show only unresolved errors

### Show Error Details
```bash
todozi error show <id>
```

**Required Arguments:**
- `<id>` - Error ID

### Resolve Error
```bash
todozi error resolve <id> [resolution]
```

**Required Arguments:**
- `<id>` - Error ID

**Arguments:**
- `[resolution]` - Resolution description

### Delete Error
```bash
todozi error delete <id>
```

**Required Arguments:**
- `<id>` - Error ID

## 🚀 Queue Management

### Plan Task Queue
```bash
todozi queue plan [options]
```

**Options:**
- `-t, --task-name <task-name>` - Task name
- `-d, --task-description <task-description>` - Task description
- `-p, --priority <priority>` - Priority level (low/medium/high) [default: medium]
- `-j, --project-id <project-id>` - Project ID

### List Queue Items
```bash
todozi queue list [options]
```

**Options:**
- `-s, --status <status>` - Filter by status

### Show Backlog
```bash
todozi queue backlog
```

### Show Active Queue
```bash
todozi queue active
```

### Complete Queue
```bash
todozi queue complete
```

### Start Queue Item
```bash
todozi queue start <queue_item_id>
```

**Required Arguments:**
- `<queue_item_id>` - Queue item ID

### End Session
```bash
todozi queue end <session_id>
```

**Required Arguments:**
- `<session_id>` - Session ID

## 🧠 Training Data Management

### Create Training Data
```bash
todozi train create [options] <prompt> <completion>
```

**Required Arguments:**
- `<prompt>` - Prompt text
- `<completion>` - Completion text

**Options:**
- `-t, --data-type <data-type>` - Data type (instruction/chat/preference) [default: instruction]
- `-c, --context <context>` - Additional context
- `-g, --tags <tags>` - Comma-separated tags
- `-q, --quality <quality>` - Quality score (0.0-1.0)
- `-s, --source <source>` - Data source (manual/automatic) [default: manual]

### List Training Data
```bash
todozi train list [options]
```

**Options:**
- `-t, --data-type <data-type>` - Filter by data type
- `-q, --min-quality <min-quality>` - Minimum quality score

### Show Training Data
```bash
todozi train show <id>
```

**Required Arguments:**
- `<id>` - Training data ID

### Training Statistics
```bash
todozi train stats
```

### Export Training Data
```bash
todozi train export [options]
```

**Options:**
- `-f, --format <format>` - Export format (json/csv) [default: json]
- `-t, --data-type <data-type>` - Filter by data type
- `-q, --min-quality <min-quality>` - Minimum quality score
- `-o, --output-file <output-file>` - Output file path

### Collect Training Data
```bash
todozi train collect <message>
```

**Required Arguments:**
- `<message>` - Message to collect

### Update Training Data
```bash
todozi train update [options] <id>
```

**Required Arguments:**
- `<id>` - Training data ID

**Options:**
- `-t, --data-type <data-type>` - Updated data type
- `-p, --prompt <prompt>` - Updated prompt
- `-c, --completion <completion>` - Updated completion
- `--context <context>` - Updated context
- `--tags <tags>` - Updated tags
- `--quality <quality>` - Updated quality score

## 🔧 System Commands

### Check Structure
```bash
todozi check-structure
```

### Ensure Structure
```bash
todozi ensure-structure
```

### Fix Consistency
```bash
todozi fix-consistency
```

### Register with Server
```bash
todozi register [options]
```

**Options:**
- `-s, --server-url <server-url>` - Server URL [default: https://todozi.com]

### Check Registration Status
```bash
todozi registration-status
```

### Clear Registration
```bash
todozi clear-registration
```

### Launch TUI
```bash
todozi tui
```

### Launch Web UI
```bash
todozi web
```

## 🌐 API Management

### Register API User
```bash
todozi api register [options]
```

**Options:**
- `-u, --user-id <user-id>` - User ID

### List API Users
```bash
todozi api list [options]
```

**Options:**
- `-a, --active-only` - Show only active users

### Check API Key
```bash
todozi api check [options] <public_key>
```

**Required Arguments:**
- `<public_key>` - Public key

**Options:**
- `-p, --private-key <private-key>` - Private key

### Deactivate API User
```bash
todozi api deactivate <user_id>
```

**Required Arguments:**
- `<user_id>` - User ID

### Activate API User
```bash
todozi api activate <user_id>
```

**Required Arguments:**
- `<user_id>` - User ID

### Remove API User
```bash
todozi api remove <user_id>
```

**Required Arguments:**
- `<user_id>` - User ID

## 🧬 Embedding Management

### Set Embedding Model
```bash
todozi emb set-model <model_name>
```

**Required Arguments:**
- `<model_name>` - Model name from HuggingFace

### Show Current Embedding Model
```bash
todozi emb show-model
```

### List Popular Embedding Models
```bash
todozi emb list-models
```

## 📤 Data Export

### Export Embeddings
```bash
todozi export-embeddings [options]
```

**Options:**
- `-o, --output <output>` - Output file path [default: todozi_embeddings.hlx]

## 🔄 Migration

### Migrate Data
```bash
todozi migrate [options]
```

**Options:**
- `--dry-run` - Perform a dry run without making changes
- `-v, --verbose` - Enable verbose output
- `--force` - Force migration even if checks fail
- `--cleanup` - Clean up old collections after migration

## 🎭 Maestro Commands

### Initialize Maestro
```bash
todozi maestro init
```

### Collect Conversation
```bash
todozi maestro collect-conversation [options]
```

**Options:**
- `-s, --session-id <session-id>` - Session ID
- `-c, --conversation <conversation>` - Conversation data
- `-l, --context-length <context-length>` - Context length [default: 0]
- `-t, --tool-calls <tool-calls>` - Tool calls
- `-r, --response <response>` - Response data
- `-T, --response-time-ms <response-time-ms>` - Response time in milliseconds [default: 1000]

### Collect Tool Usage
```bash
todozi maestro collect-tool [options]
```

**Options:**
- `-s, --session-id <session-id>` - Session ID
- `-t, --tool-name <tool-name>` - Tool name
- `-c, --tool-call <tool-call>` - Tool call data
- `-T, --execution-time-ms <execution-time-ms>` - Execution time in milliseconds [default: 500]
- `-s, --success <success>` - Success status
- `-r, --result-summary <result-summary>` - Result summary

## 🤖 ML Commands

### Train Model
```bash
todozi ml train [options]
```

**Options:**
- `-d, --dataset <dataset>` - Dataset path
- `-m, --model <model>` - Model type
- `-e, --epochs <epochs>` - Number of epochs
- `-b, --batch-size <batch-size>` - Batch size

### Evaluate Model
```bash
todozi ml evaluate [options] <model_path>
```

**Required Arguments:**
- `<model_path>` - Path to model

**Options:**
- `-d, --dataset <dataset>` - Dataset path
- `-m, --metrics <metrics>` - Metrics to evaluate

### Deploy Model
```bash
todozi ml deploy [options] <model_path>
```

**Required Arguments:**
- `<model_path>` - Path to model

**Options:**
- `-e, --endpoint <endpoint>` - Deployment endpoint
- `-v, --version <version>` - Model version

## 🚨 Error Handling

### Task Not Found
When a task ID is not found, Todozi will display:
```
❌ Task not found: <task_id>
```

### Project Not Found
When a project name is not found, Todozi will display:
```
❌ Project not found: <project_name>
```

### Agent Not Found
When an agent ID is not found, Todozi will display:
```
❌ Agent not found: <agent_id>
```

### Invalid Priority/Status
When an invalid priority or status is provided, Todozi will display:
```
❌ Invalid priority/status: <value>
Valid options are: <list_of_valid_options>
```

### Invalid Assignee
When an invalid assignee is provided, Todozi will display:
```
❌ Invalid assignee: <value>
Valid options are: ai, human, collaborative, or a specific agent ID
```

## 💡 Tips and Best Practices

1. **Use Projects**: Organize tasks into projects for better management
2. **Set Priorities**: Always set appropriate priorities for tasks
3. **Add Context**: Provide context for complex tasks
4. **Use Tags**: Tag tasks for easier filtering and searching
5. **Regular Backups**: Create backups regularly to prevent data loss
6. **AI Collaboration**: Use AI agents for tasks that benefit from automation
7. **Memory Management**: Save important insights as memories for future reference
8. **Idea Capture**: Capture ideas immediately to prevent forgetting them
9. **Error Tracking**: Log errors to improve system reliability
10. **Training Data**: Contribute to model improvement by collecting training data
