// Example 1: Creating various types of tasks using the SimpleTodoziTool

// Initialize the tool
import { SimpleTodoziTool, Priority, Status } from '../todozi/todozi_tool.js';
import { CreateTaskTool, initializeTodoziSystem } from '../todozi/todozi_tool.js';

const todoziTool = new SimpleTodoziTool();

async function demonstrateTodoziUsage() {
    console.log('🎯 Todozi Task Management Demo\n');
    
    // Example 1: Create a regular task
    console.log('1. Creating a Regular Task:');
    const regularTask = await todoziTool.execute({
        action: 'task',
        content: 'Review the quarterly project report'
    });
    console.log(regularTask.content);
    console.log();
    
    // Example 2: Create an urgent task
    console.log('2. Creating an Urgent Task:');
    const urgentTask = await todoziTool.execute({
        action: 'urgent',
        content: 'Fix production server outage immediately'
    });
    console.log(urgentTask.content);
    console.log();
    
    // Example 3: Create an AI-assigned task
    console.log('3. Creating an AI-Assigned Task:');
    const aiTask = await todoziTool.execute({
        action: 'ai',
        content: 'Analyze user engagement metrics and generate insights'
    });
    console.log(aiTask.content);
    console.log();
    
    // Example 4: Create a collaborative task
    console.log('4. Creating a Collaborative Task:');
    const collabTask = await todoziTool.execute({
        action: 'collab',
        content: 'Design new user onboarding flow'
    });
    console.log(collabTask.content);
    console.log();
    
    // Example 5: Search for tasks
    console.log('5. Searching for Tasks:');
    const searchResult = await todoziTool.execute({
        action: 'find',
        content: 'report'
    });
    console.log(searchResult.content);
    console.log();
    
    // Example 6: Get system statistics
    console.log('6. Getting System Stats:');
    const stats = await todoziTool.execute({
        action: 'stats',
        content: '' // content not needed for stats
    });
    console.log(stats.content);
    console.log();
}

// Example of using the more detailed CreateTaskTool
async function demonstrateDetailedTaskCreation() {
    console.log('\n📋 Detailed Task Creation Example:\n');
    
    
    // Initialize the system
    const { todozi } = await initializeTodoziSystem();
    const createTaskTool = new CreateTaskTool(todozi);
    
    // Create a task with full parameters
    const detailedTask = await createTaskTool.execute({
        action: 'Implement user authentication system',
        time: '3 days',
        priority: 'high',
        project: 'Website Redesign',
        assignee: 'ai',
        tags: 'auth,security,backend',
        context: 'Need to implement OAuth2.0 with Google and GitHub providers'
    });
    
    console.log('Detailed Task Creation Result:');
    console.log(detailedTask.content);
}

// Run the examples
demonstrateTodoziUsage()
    .then(() => demonstrateDetailedTaskCreation())
    .catch(error => console.error('Error:', error));

/*
# Todozi Task Management: Simple Usage Example

Here's a practical example showing how to use Todozi's simple task creation tool:

**Output Example:**

/ *
🎯 Todozi Task Management Demo

1. Creating a Regular Task:
✅ Task created: 1a2b3c4d-5e6f-7g8h-9i0j

2. Creating an Urgent Task:
🚨 Urgent task created: 2b3c4d5e-6f7g-8h9i-0j1k

3. Creating an AI-Assigned Task:
🤖 AI task queued: 3c4d5e6f-7g8h-9i0j-1k2l

4. Creating a Collaborative Task:
🤝 Collaborative task created: 4d5e6f7g-8h9i-0j1k-2l3m

5. Searching for Tasks:
🔍 Smart search results: []

6. Getting System Stats:
📊 Quick stats:
Total tasks: 0
Completed: 0
In Progress: 0

📋 Detailed Task Creation Example:

Detailed Task Creation Result:
✅ Created task 'Implement user authentication system' with ID: 5e6f7g8h-9i0j-1k2l-3m4n (queued for ai)
*/