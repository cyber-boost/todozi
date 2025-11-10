// example_task_management.js

import { Task, Priority, Status, Assignee } from '../todozi/models.js';
import { TodoziHandler } from '../todozi/cli.js';
import { Storage } from '../todozi/storage.js';

async function exampleTaskManagement() {
    try {
        // Initialize storage
        const storage = await Storage.new();
        
        // Create a new task using the models
        const task = Task.newFull(
            'user123',
            'Implement user authentication',
            '2 hours',
            Priority.High,
            'web-app',
            Status.Todo,
            Assignee.Human,
            ['backend', 'security'],
            [],
            'Need to implement JWT-based authentication',
            0
        );
        
        // Add task to storage
        await storage.addTaskToProject(task);
        console.log(`✅ Created task: ${task.id} - ${task.action}`);
        
        // Update task progress
        const updates = {
            progress: 50,
            status: Status.InProgress,
            contextNotes: 'Implemented login endpoint, working on refresh tokens'
        };
        task.update(updates);
        await storage.addTaskToProject(task); // Save updated task
        console.log(`🔄 Updated task: ${task.id} - Progress: ${task.progress}%`);
        
        // Complete the task
        task.complete();
        await storage.addTaskToProject(task);
        console.log(`✅ Completed task: ${task.id}`);
        
        // List all tasks in the project
        const projectTasks = await storage.getProjectTasks('web-app');
        console.log('\n📋 Tasks in web-app project:');
        projectTasks.forEach(t => {
            console.log(`  ${t.id}: ${t.action} (${t.status})`);
        });
        
        // Search for tasks
        const searchResults = await storage.searchTasks('authentication');
        console.log('\n🔍 Search results for "authentication":');
        searchResults.forEach(result => {
            console.log(`  ${result.id}: ${result.action}`);
        });
        
    } catch (error) {
        console.error('❌ Error in task management:', error.message);
    }
}

// Run the example
exampleTaskManagement();

/*
Here's a practical example demonstrating how to use the models and CLI handler for task management:

This example demonstrates:

1. **Creating Tasks**: Using `Task.newFull()` to create a new task with all properties
2. **Task Management**: Updating task progress and status
3. **Storage Integration**: Saving tasks to storage and retrieving them
4. **Task Completion**: Using the built-in `complete()` method
5. **Project Organization**: Listing tasks by project
6. **Search Functionality**: Finding tasks by keyword

Key features shown:
- Proper use of enums (`Priority`, `Status`, `Assignee`)
- Task lifecycle management (create → update → complete)
- Storage operations (save, retrieve, search)
- Error handling with try/catch

To run this example:
1. Save as `example_task_management.js`
2. Ensure you have the required modules (`models.js`, `cli.js`, `storage.js`)
3. Execute with: `node example_task_management.js`

The output will show task creation, updates, and retrieval operations. This demonstrates how the Todozi models and storage system work together to manage tasks effectively.
*/