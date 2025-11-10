// Example 1: Chat message processing with Todozi integration
import { tdzCnt } from '../todozi/tdz_tls.js';
import { Storage } from '../todozi/storage.js';

async function processChatWithTodozi() {
    // Example chat message containing Todozi tags
    const chatMessage = `
Hey team, here are some action items from our meeting:

<todozi>Refactor user authentication module;2 days;high;backend;todo;ai;security,refactor;auth-001;Need to improve security and performance;20</todozi>

<todozi>Update API documentation;4 hours;medium;documentation;todo;human;docs,api;;Reference new endpoints added last week</todozi>

<memory>standard;User reported login issue;Fixed authentication timeout;Resolved customer issue;medium;short;support,customer</memory>

<idea>Add SSO integration;share;high;Authentication,Feature</idea>

Also, we should create unit tests for the new payment module and don't forget to update the deployment script.
    `;

    try {
        // Process the chat message
        const result = await tdzCnt(chatMessage, 'meeting-123');
        
        // Parse the JSON response
        const processed = JSON.parse(result);
        
        console.log('🎯 CHAT PROCESSING RESULTS');
        console.log('═════════════════════════');
        
        console.log('\n📊 Processing Summary:');
        console.log(`✅ Process: ${processed.process}`);
        console.log(`📝 Cleaned Content: ${processed.clean.substring(0, 100)}...`);
        console.log(`🔧 Processed Items: ${processed.processed_items}`);
        
        console.log('\n🚀 Extracted Tasks:');
        if (processed.traditional_processing) {
            const traditional = processed.traditional_processing;
            console.log('Traditional processing executed successfully');
            
            // The traditional processing would have created tasks in storage
            const storage = await Storage.new();
            const tasks = await storage.listTasksAcrossProjects({});
            
            console.log(`📋 Total tasks in system: ${tasks.length}`);
            tasks.forEach(task => {
                console.log(`   • ${task.action} (${task.status}, ${task.priority})`);
            });
        }
        
        console.log('\n💡 Next Steps:');
        console.log('1. Review extracted tasks above');
        console.log('2. Check todozi list for all active tasks');
        console.log('3. Use todozi show <task_id> for details');
        
    } catch (error) {
        console.error('❌ Error processing chat:', error.message);
    }
}

// Example 2: Interactive Todozi tag creation
function createTodoziTask() {
    const taskData = {
        action: 'Implement rate limiting',
        time: '3 days',
        priority: 'high',
        project: 'security',
        status: 'todo',
        assignee: 'ai',
        tags: ['security', 'api', 'performance'],
        dependencies: ['auth-001'],
        context: 'Prevent API abuse and ensure fair usage',
        progress: 0
    };
    
    // Convert to Todozi tag format
    const todoziTag = `
<todozi>
${taskData.action};
${taskData.time};
${taskData.priority};
${taskData.project};
${taskData.status};
${taskData.assignee};
${taskData.tags.join(',')};
${taskData.dependencies.join(',')};
${taskData.context};
${taskData.progress}
</todozi>
    `.trim();
    
    console.log('📋 Generated Todozi Tag:');
    console.log(todoziTag);
    
    return todoziTag;
}

// Example 3: Batch processing multiple messages
async function processConversationBatch() {
    const conversation = [
        "Let's start with <todozi>Set up CI/CD pipeline;1 week;high;infrastructure;todo;collaborative;devops,ci-cd;;Need automated testing and deployment</todozi>",
        "<memory>short;Team prefers Docker;Containerization decision;Consensus on technology;medium;short;technology,decision</memory>",
        "Also <todozi>Write integration tests;2 days;medium;testing;todo;ai;testing,quality;;Cover main user flows</todozi>"
    ];
    
    const sessionId = 'project-setup-001';
    
    for (const [index, message] of conversation.entries()) {
        console.log(`\n💬 Processing message ${index + 1}:`);
        console.log(message.substring(0, 80) + '...');
        
        try {
            const result = await tdzCnt(message, sessionId);
            const processed = JSON.parse(result);
            
            console.log(`✅ Processed ${processed.processed_items} items`);
            console.log(`📝 Cleaned: ${processed.clean.substring(0, 60)}...`);
        } catch (error) {
            console.error(`❌ Failed to process message ${index + 1}:`, error.message);
        }
    }
}

// Run examples
(async () => {
    console.log('🚀 EXAMPLE 1: Single Message Processing');
    await processChatWithTodozi();
    
    console.log('\n\n🚀 EXAMPLE 2: Todozi Tag Generation');
    createTodoziTask();
    
    console.log('\n\n🚀 EXAMPLE 3: Conversation Batch Processing');
    await processConversationBatch();
})();

/*
Here's a practical example demonstrating how to use the Todozi content processor for processing chat messages with task extraction:

## Example: Process Chat Message with Todozi Tags

## Expected Output:

/ *
🎯 CHAT PROCESSING RESULTS
═════════════════════════

📊 Processing Summary:
✅ Process: success
📝 Cleaned Content: Hey team, here are some action items from our meeting: Also, we should create unit tests for the new payment module and don't forget to update the deployment script....
🔧 Processed Items: 4

🚀 Extracted Tasks:
Traditional processing executed successfully
📋 Total tasks in system: 2
   • Refactor user authentication module (todo, high)
   • Update API documentation (todo, medium)

💡 Next Steps:
1. Review extracted tasks above
2. Check todozi list for all active tasks
3. Use todozi show <task_id> for details
*/