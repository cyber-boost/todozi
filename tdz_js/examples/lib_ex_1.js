// Import the library (assuming it's available in your project)

import { Done } from '../todozi/todozi.js';

class PersonalTaskManager {
    constructor(projectName = 'personal_tasks') {
        // Set custom project name for personal tasks
        Done.setProject(projectName);
    }

    // Quick task creation methods
    async addQuickTask(action) {
        return await Done.task(action);
    }

    async addUrgentTask(action) {
        return await Done.urgent(action);
    }

    async addTaskWithPriority(action, priority) {
        return await Done.createTask(action, priority, null, null, null);
    }

    // Task management
    async listAllTasks() {
        return await Done.all();
    }

    async completeTask(taskId) {
        return await Done.done(taskId);
    }

    async startTask(taskId) {
        return await Done.start(taskId);
    }

    async deleteTask(taskId) {
        return await Done.delete(taskId);
    }

    // Search functionality
    async searchTasks(query) {
        return await Done.find(query);
    }

    async searchWithAI(query) {
        return await Done.aiFind(query);
    }

    // Ideas and memories
    async saveIdea(idea, context = null) {
        return await Done.idea(idea);
    }

    async saveImportantMemory(moment, meaning, reason = 'Important personal memory') {
        return await Done.important(moment, meaning, reason);
    }

    // Statistics and insights
    async getTaskStats() {
        return await Done.quick();
    }

    async getCompletionRate() {
        return await Done.completionRate();
    }

    // Project management
    async createCustomProject(name, description) {
        return await Done.createProject(name, description);
    }

    async listProjects() {
        return await Done.listProjects();
    }

    // Example workflow
    async dailyPlanning() {
        console.log('📋 Starting daily planning...');
        
        // Add morning tasks
        const breakfastTask = await this.addQuickTask('Prepare and eat breakfast');
        const exerciseTask = await this.addTaskWithPriority('Morning exercise', 'high');
        
        // Add work tasks
        const workTask1 = await this.addQuickTask('Review emails');
        const workTask2 = await this.addTaskWithPriority('Complete project milestone', 'critical');
        
        // Add urgent personal task
        const urgentTask = await this.addUrgentTask('Pay electricity bill');
        
        console.log('✅ Daily tasks added successfully!');
        
        // Display stats
        const stats = await this.getTaskStats();
        console.log(stats);
        
        return {
            breakfastTask,
            exerciseTask,
            workTask1,
            workTask2,
            urgentTask
        };
    }

    async completeMorningRoutine() {
        console.log('🌅 Completing morning routine...');
        
        // Search for morning tasks
        const morningTasks = await this.searchTasks('breakfast exercise');
        
        for (const task of morningTasks) {
            if (task.action.toLowerCase().includes('breakfast') || 
                task.action.toLowerCase().includes('exercise')) {
                await this.completeTask(task.id);
                console.log(`✅ Completed: ${task.action}`);
            }
        }
        
        console.log('🎉 Morning routine completed!');
    }
}

// Usage example
async function runExample() {
    const taskManager = new PersonalTaskManager('my_personal_tasks');
    
    try {
        // Initialize Todozi system
        await Done.todoziBegin();
        
        // Create daily plan
        const tasks = await taskManager.dailyPlanning();
        
        // List all current tasks
        const allTasks = await taskManager.listAllTasks();
        console.log('\n📊 Current Tasks:');
        allTasks.forEach((task, index) => {
            console.log(`${index + 1}. ${task.action} [${task.status}] [${task.priority}]`);
        });
        
        // Complete some tasks
        await taskManager.completeMorningRoutine();
        
        // Save an idea that came up during planning
        await taskManager.saveIdea('Create automated meal planner using AI');
        
        // Get updated statistics
        const updatedStats = await taskManager.getTaskStats();
        console.log('\n' + updatedStats);
        
        // Search for specific tasks
        const billTasks = await taskManager.searchTasks('bill');
        console.log('\n💰 Bill-related tasks:');
        billTasks.forEach(task => {
            console.log(`- ${task.action}`);
        });
        
    } catch (error) {
        console.error('❌ Error:', error.message);
    }
}

// Advanced example: AI-powered task recommendations
async function advancedExample() {
    const taskManager = new PersonalTaskManager('smart_tasks');
    
    await Done.todoziBegin();
    
    // Use AI to find similar tasks
    const similarTasks = await taskManager.searchWithAI('coding project');
    console.log('🤖 AI-powered task recommendations:');
    similarTasks.forEach((task, index) => {
        console.log(`${index + 1}. ${task.action} (similarity: ${(task.similarityScore * 100).toFixed(1)}%)`);
    });
    
    // Create a memory about successful project completion
    await taskManager.saveImportantMemory(
        'Completed web application',
        'Successfully deployed Node.js application to production',
        'Important learning experience for future projects'
    );
}

// Run examples
(async () => {
    console.log('🚀 Starting Personal Task Manager Examples\n');
    
    await runExample();
    
    console.log('\n--- Advanced Example ---');
    await advancedExample();
    
    console.log('\n🎯 Examples completed successfully!');
})();

/*
## Example 1: Using the Todozi Done Class for Personal Task Management

This example demonstrates how to use the Todozi library's `Done` class to create a simple personal task management system.

### Code Example

### Key Features Demonstrated

1. **Quick Task Creation**: Simple methods like `task()`, `urgent()`, and `createTask()` for adding tasks
2. **Task Management**: Complete, start, delete, and search tasks
3. **Advanced Search**: Both keyword-based and AI-powered semantic search
4. **Ideas & Memories**: Store ideas and important memories alongside tasks
5. **Statistics**: Get completion rates and task statistics
6. **Project Organization**: Create and manage multiple project contexts

### Expected Output

/ *
🚀 Starting Personal Task Manager Examples

📋 Starting daily planning...
✅ Daily tasks added successfully!
📊 EMBEDDING STATS: 45 tasks, 12% complete

📊 Current Tasks:
1. Prepare and eat breakfast [todo] [medium]
2. Morning exercise [todo] [high]
3. Review emails [todo] [medium]
4. Complete project milestone [todo] [critical]
5. Pay electricity bill [todo] [urgent]

🌅 Completing morning routine...
✅ Completed: Prepare and eat breakfast
✅ Completed: Morning exercise
🎉 Morning routine completed!

📊 TODOZI STATS
📋 Tasks: 45 total
  ✅ Done: 15
  🔄 In Progress: 5
  🚫 Blocked: 2

💰 Bill-related tasks:
- Pay electricity bill

--- Advanced Example ---
🤖 AI-powered task recommendations:
1. Complete coding project documentation (similarity: 87.2%)
2. Review pull request for web app (similarity: 76.5%)

🎯 Examples completed successfully!
*/