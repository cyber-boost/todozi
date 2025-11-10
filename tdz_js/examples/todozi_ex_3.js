import { processChatMessageExtended, processWorkflow, transformShorthandTags, Priority, Status, Assignee } from '../todozi/todozi.js';

console.log('🚀 Starting workflow execution...\n');

// Note: In a real implementation, you would import and mock storage like this:
// import * as storageModule from '../todozi/storage.js';
// storageModule.Storage = class { ... };
// For this example, the mocks are defined in the commented section below.

// Execute the workflow
processWorkflow(content.tasks).then(results => {
    console.log('\n✅ Workflow Results:');
    results.forEach((result, index) => {
        console.log(`${index + 1}. ${result}`);
    });
});

/*
console.log('🔧 Task Analysis:\n');

content.tasks.forEach((task, index) => {
    console.log(`${index + 1}. Task: ${task.action}`);
    console.log(`   Priority: ${task.priority}`);
    console.log(`   Status: ${task.status}`);
    console.log(`   Assignee: ${JSON.stringify(task.assignee)}`);
    console.log(`   Tags: ${task.tags.join(', ')}`);
    console.log(`   Dependencies: ${task.dependencies.join(', ') || 'None'}\n`);
});

/ *
const complexMessage = `
I need to complete several tasks for the Q1 launch:

<tz>Design REST API endpoints; 2 days; critical; q1-launch; todo; ai; api,design; endpoint-design</tz>

<tz>Review security implementation; 1 day; high; q1-launch; in_progress; human; security,review; auth-review</tz>

<tz>Create deployment pipeline; 3 days; high; infrastructure; todo; collaborative; devops,ci-cd</tz>

<tz>Assign performance testing; 2 days; medium; testing; todo; agent=perf_tester; testing</tz>

<mm>Feeling anxious about the deadline; Recognizing deadline pressure; Need to manage stress; medium; short; anxiety,deadline</mm>

<id>Implement real-time notifications using WebSockets; share; high; feature,websocket</id>

<er>Authentication token expiration bug; JWT tokens expiring too quickly; critical; security; auth-service; token-expiry</er>

<train>How to implement rate limiting in Node.js; Use Redis with sliding window algorithm; code; redis,rate-limiting; 0.9</train>
`;

console.log('🔍 Processing complex chat message...\n');

// Transform shorthand tags first
const transformedMessage = transformShorthandTags(complexMessage);
console.log('📝 Transformed message (shorthand → longhand):');
console.log(transformedMessage.substring(0, 200) + '...\n');

// Parse all content types
const content = processChatMessageExtended(transformedMessage, 'user_123');

console.log('📊 Extracted content summary:');
console.log(`  📋 Tasks: ${content.tasks.length}`);
console.log(`  🧠 Memories: ${content.memories.length}`);
console.log(`  💡 Ideas: ${content.ideas.length}`);
console.log(`  ❌ Errors: ${content.errors.length}`);
console.log(`  🎓 Training Data: ${content.training_data.length}\n`);

/ *
// Mock storage for demonstration
const mockStorage = {
    searchTasksSemantic: async (query, limit) => {
        // Simulate finding similar tasks
        return [
            { task: { assignee: Assignee.Human, action: "Previous API design task" } },
            { task: { assignee: Assignee.Human, action: "Database schema review" } }
        ];
    },
    addTaskToProject: async (task) => {
        console.log(`💾 Saved task to project "${task.parent_project}": ${task.action}`);
        return task;
    },
    updateTaskInProject: async (taskId, updates) => {
        console.log(`📝 Updated task ${taskId} status to ${updates.status}`);
    }
};

// Mock queue storage
const mockQueueStorage = {
    addQueueItem: async (item) => {
        console.log(`📋 Added to queue: ${item.title} (Priority: ${item.priority})`);
    },
    saveAgentAssignment: async (assignment) => {
        console.log(`🤖 Agent assignment saved: ${assignment.agent_id} → ${assignment.task_id}`);
    }
};

/ *
# Example 3: Complex Workflow Processing with Multiple Content Types

This example demonstrates how Todozi processes a complex chat message containing multiple content types and executes tasks with different assignees.

## Setup

## Processing Complex Chat Message

## Detailed Task Analysis

## Workflow Execution

## Expected Output

🔍 Processing complex chat message...

📝 Transformed message (shorthand → longhand):
I need to complete several tasks for the Q1 launch:

<todozi>Design REST API endpoints; 2 days; critical; q1-launch; todo; ai; api,design; endpoint-design</todozi>

<todozi>Review security implementation; 1 day; high; q1-launch; in_progress; human; security,review; auth-review</todozi>...

📊 Extracted content summary:
  📋 Tasks: 4
  🧠 Memories: 1
  💡 Ideas: 1
  ❌ Errors: 1
  🎓 Training Data: 1

🔧 Task Analysis:

1. Task: Design REST API endpoints
   Priority: critical
   Status: todo
   Assignee: "ai"
   Tags: api, design, endpoint-design
   Dependencies: None

2. Task: Review security implementation
   Priority: high
   Status: in_progress
   Assignee: "human"
   Tags: security, review, auth-review
   Dependencies: None

3. Task: Create deployment pipeline
   Priority: high
   Status: todo
   Assignee: "collaborative"
   Tags: devops, ci-cd
   Dependencies: None

4. Task: Assign performance testing
   Priority: medium
   Status: todo
   Assignee: {"type":"agent","name":"perf_tester"}
   Tags: testing
   Dependencies: None

🚀 Starting workflow execution...

📋 Added to queue: AI: Design REST API endpoints (Priority: critical)
💾 Saved task to project "q1-launch": Design REST API endpoints
✅ Task completed and saved: Design REST API endpoints

📋 Added to queue: Human: Review security implementation (Priority: high)
💾 Saved task to project "q1-launch": Review security implementation
✅ Task completed and saved: Review security implementation

📋 Added to queue: AI Collab: Create deployment pipeline (Priority: high)
📋 Added to queue: Human Collab: Create deployment pipeline (Priority: high)
💾 Saved task to project "infrastructure": Create deployment pipeline
✅ Task completed and saved: Create deployment pipeline

🤖 Agent assignment saved: perf_tester → [generated-task-id]
📋 Added to queue: perf_tester Agent: Assign performance testing (Priority: medium)
💾 Saved task to project "testing": Assign performance testing
✅ Task completed and saved: Assign performance testing

✅ Workflow Results:
1. Task queued for AI processing: Design REST API endpoints (Queue ID: [queue-id-1])
2. Task available in TUI queue: Review security implementation (Queue ID: [queue-id-2])
3. Collaborative task queued: Create deployment pipeline (AI Queue: [queue-id-3], Human Queue: [queue-id-4])
4. Task assigned to perf_tester agent: Assign performance testing (Assignment saved, Queue ID: [queue-id-5])

## Advanced Features Demonstrated

### 1. Content Type Parsing
- **Tasks**: Four tasks with different assignees (AI, human, collaborative, agent)
- **Memories**: Emotional memory about deadline anxiety
- **Ideas**: Real-time notifications feature
- **Errors**: Critical authentication bug report
- **Training Data**: Rate limiting implementation example

### 2. Assignee-Specific Processing
*/