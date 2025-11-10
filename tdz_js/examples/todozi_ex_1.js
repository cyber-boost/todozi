// Example: Creating and executing tasks with various assignees
import { parseTodoziFormat, processWorkflow, executeTask, processChatMessage } from '../todozi/todozi.js';

// Example 1: AI-assigned task
const aiTaskText = `<todozi>
Analyze user engagement metrics; 2 hours; high; analytics_project; todo; ai
</todozi>`;

// Example 2: Human-assigned task
const humanTaskText = `<todozi>
Review design mockups; 1 hour; medium; design_project; todo; human; review,design
</todozi>`;

// Example 3: Collaborative task
const collaborativeTaskText = `<todozi>
Develop new feature; 8 hours; critical; development_project; todo; collaborative; development,collaboration
</todozi>`;

// Example 4: Agent-assigned task
const agentTaskText = `<todozi>
Fix database performance issues; 4 hours; high; ops_project; todo; agent=database_specialist; database,performance
</todozi>`;

// Example 5: Default (no assignee specified)
const defaultTaskText = `<todozi>
Write documentation; 3 hours; medium; docs_project; todo; documentation,writing
</todozi>`;

// Parse all tasks
try {
    const aiTask = parseTodoziFormat(aiTaskText);
    const humanTask = parseTodoziFormat(humanTaskText);
    const collaborativeTask = parseTodoziFormat(collaborativeTaskText);
    const agentTask = parseTodoziFormat(agentTaskText);
    const defaultTask = parseTodoziFormat(defaultTaskText);
    
    console.log('✅ Successfully parsed all tasks:');
    console.log(`🤖 AI Task: ${aiTask.action} (Priority: ${aiTask.priority})`);
    console.log(`👤 Human Task: ${humanTask.action} (Tags: ${humanTask.tags.join(', ')})`);
    console.log(`🤝 Collaborative Task: ${collaborativeTask.action}`);
    console.log(`⚙️ Agent Task: ${agentTask.assignee.name} -> ${agentTask.action}`);
    console.log(`❓ Default Task: ${defaultTask.action} (Auto-assignment based on task type)`);
    
    // Process tasks in workflow
    const tasks = [aiTask, humanTask, collaborativeTask, agentTask, defaultTask];
    
    console.log('\n🚀 Processing workflow...');
    processWorkflow(tasks).then(results => {
        console.log('\n📊 Workflow Results:');
        results.forEach((result, index) => {
            console.log(`${index + 1}. ${result}`);
        });
    });
    
} catch (error) {
    console.error('❌ Error parsing tasks:', error.message);
}

// Example of processing a chat message containing multiple todozi tasks
const sampleChatMessage = `
Hey team, here are our tasks for today:

<tz>Analyze user metrics;2h;high;analytics;todo;ai</tz>
Let the AI handle the data analysis.

<mm>Improve login flow;1h;medium;auth;todo;human;ui,ux</mm>
John, please review this.

<t dz>Fix API bug;4h;critical;backend;todo;agent=bug_fixer</tdz>
Assign to our specialist.
`;

console.log('\n💬 Processing chat message with shorthand tags:');
const processedChat = processChatMessage(sampleChatMessage);
console.log(`Found ${processedChat.length} tasks in chat message`);
processedChat.forEach(task => {
    console.log(`- ${task.action} (${task.assignee ? task.assignee : 'auto-assign'})`);
});

/*
## Example 1: Creating and Parsing Todozi Tasks with Different Assignees

This example demonstrates how to use the Todozi system to create tasks with different assignees and process them through the workflow.

**Key Features Demonstrated:**

1. **Different Assignee Types:**
   - `ai`: AI-only tasks
   - `human`: Human-only tasks  
   - `collaborative`: AI+Human collaboration
   - `agent=name`: Specific agent assignment
   - Default: Automatic assignment based on task content

2. **Task Format:**
   - Action description
   - Time estimate
   - Priority level
   - Parent project
   - Status
   - Optional: assignee, tags, dependencies, context notes, progress

3. **Shorthand Support:**
   - `<tz>` instead of `<todozi>`
   - Automatic expansion of shorthand tags

**Expected Output:**

/ *
✅ Successfully parsed all tasks:
🤖 AI Task: Analyze user engagement metrics (Priority: high)
👤 Human Task: Review design mockups (Tags: review,design)
🤝 Collaborative Task: Develop new feature
⚙️ Agent Task: database_specialist -> Fix database performance issues
❓ Default Task: Write documentation (Auto-assignment based on task type)

🚀 Processing workflow...

📊 Workflow Results:
1. Task queued for AI processing: Analyze user engagement metrics (Queue ID: ...)
2. Task available in TUI queue: Review design mockups (Queue ID: ...)
3. Collaborative task queued: Develop new feature (AI Queue: ..., Human Queue: ...)
4. Task assigned to database_specialist agent: Fix database performance issues...
5. Task queued for AI processing: Write documentation (Queue ID: ...)

💬 Processing chat message with shorthand tags:
Found 3 tasks in chat message
- Analyze user metrics (ai)
- Improve login flow (human)
- Fix API bug (agent=bug_fixer)
*/