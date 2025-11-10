// project_setup.js - Setting up a web app development project using Todozi

import { executeTodoziToolDelegated } from '../todozi/todozi_exe.js';

async function setupWebAppProject() {
    console.log('🚀 Setting up Todozi for Web App Development Project\n');
    
    // 1. Create project tasks with different priorities
    const projectTasks = [
        {
            action: 'task',
            content: 'Research and select frontend framework (React/Vue)'
        },
        {
            action: 'urgent', 
            content: 'Fix authentication security vulnerability'
        },
        {
            action: 'high',
            content: 'Implement user dashboard functionality'
        },
        {
            action: 'ai',
            content: 'Analyze performance metrics and suggest optimizations'
        },
        {
            action: 'human',
            content: 'Design database schema for user profiles'
        },
        {
            action: 'collab',
            content: 'Plan API integration with third-party services'
        }
    ];
    
    // 2. Execute all tasks
    for (const task of projectTasks) {
        try {
            const result = await executeTodoziToolDelegated(task);
            console.log(result.output);
        } catch (error) {
            console.error(`❌ Failed to create task: ${error.message}`);
        }
    }
    
    // 3. Search for specific tasks
    console.log('\n🔍 Searching for security-related tasks...');
    const searchResult = await executeTodoziToolDelegated({
        action: 'find',
        content: 'security authentication'
    });
    console.log(searchResult.output);
    
    // 4. Track important project memories
    console.log('\n🧠 Saving project decisions...');
    const memoryResult = await executeTodoziToolDelegated({
        action: 'important_memory',
        content: 'Chose React over Vue due to team experience',
        extra: 'Framework selection impacts timeline and training needs'
    });
    console.log(memoryResult.output);
    
    // 5. Get project statistics
    console.log('\n📊 Getting project overview...');
    const statsResult = await executeTodoziToolDelegated({
        action: 'stats'
    });
    console.log(statsResult.output);
}

async function processDevelopmentMeeting() {
    console.log('\n💬 Processing development meeting notes...');
    
    const meetingNotes = `
    <todozi>Fix login page CSS issues;2 hours;medium;web-app;in_progress;human</todozi>
    <todozi>Implement payment integration;8 hours;high;web-app;todo;collab</todozi>
    <todozi>Write unit tests for user service;4 hours;medium;web-app;todo;ai</todozi>
    <memory>technical;API rate limiting implemented;Prevent abuse;high;long</memory>
    <idea>Add dark mode toggle;share;medium</idea>
    `;
    
    const chatResult = await executeTodoziToolDelegated({
        action: 'chat',
        content: meetingNotes
    });
    console.log(chatResult.output);
}

async function expandComplexTask() {
    console.log('\n🚀 Breaking down complex task...');
    
    const expansionResult = await executeTodoziToolDelegated({
        action: 'expand',
        content: 'Implement user analytics dashboard',
        extra: 'Include charts for user activity, retention, and feature usage'
    });
    console.log(expansionResult.output);
}

// Main execution
async function main() {
    try {
        await setupWebAppProject();
        await processDevelopmentMeeting();
        await expandComplexTask();
        
        console.log('\n✅ Project setup complete! Use todozi commands to manage your development workflow.');
        console.log('💡 Try these next:');
        console.log('  • todozi queue - Check task status');
        console.log('  • todozi search "database" - Find related tasks');
        console.log('  • todozi complete <task_id> - Mark tasks as done');
        
    } catch (error) {
        console.error('❌ Project setup failed:', error.message);
    }
}

// Run the example
main().catch(console.error);

/*
Here's a practical example that demonstrates using the Todozi system to manage a software development project:

## Example: Todozi Project Management for Web App Development

## Expected Output:

/ *
🚀 Setting up Todozi for Web App Development Project

✅ Task created: task_abc123
🚨 Urgent task created: task_def456  
🟠 High priority task created: task_ghi789
🤖 AI task queued: task_jkl012 (available for Maestro/Claude/etc.)
👤 Human task created: task_mno345 (visible in TUI)
🤝 Collaborative task created: task_pqr678 (AI+Human coordination)

🔍 Searching for security-related tasks...
🔍 Smart search results: [{"task_id": "task_def456", "action": "Fix authentication security vulnerability", ...}]

🧠 Saving project decisions...
🧠⭐ Important memory saved: mem_stu901

📊 Getting project overview...
📊 Todozi Stats: Total tasks: 6, Active: 5, Completed: 0

💬 Processing development meeting notes...
✅ Chat processed: 📋 Created 3 tasks, 🧠 Created 1 memory, 💡 Created 1 idea

🚀 Breaking down complex task...
🚀 Expanded into 5 detailed tasks:
1. Design analytics dashboard layout
2. Implement user activity tracking
3. Create retention metrics charts
4. Add feature usage statistics
5. Set up data export functionality

✅ Project setup complete! Use todozi commands to manage your development workflow.
*/