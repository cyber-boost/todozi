// example_usage.js
import { v4 as uuidv4 } from 'uuid';
import fs from 'fs/promises';
import path from 'path';

// Import necessary classes and enums
import {
    Task, TaskUpdate, Project, Priority, Status, Assignee,
    TaskCollection, ProjectTaskContainer
} from '../todozi/models.js';
import { Storage } from '../todozi/storage.js';
import { TagManager } from '../todozi/tags.js';

// Example 1: Creating and managing tasks
async function taskManagementExample() {
    console.log("=== Task Management Example ===");
    
    // Create a new task
    const task1 = Task.newFull(
        'user_123',
        'Implement user authentication',
        '3 hours',
        Priority.High,
        'web-app',
        Status.Todo,
        Assignee.Human,
        ['backend', 'security'],
        [],
        'Need to implement JWT-based authentication',
        0
    );
    
    console.log(`Created task: ${task1.action} (${task1.priority})`);
    
    // Update task progress
    const updates = TaskUpdate.new().withProgress(50).withStatus(Status.InProgress);
    task1.update(updates);
    console.log(`Updated task progress: ${task1.progress}%`);
    
    // Complete the task
    task1.complete();
    console.log(`Task completed: ${task1.isCompleted()}`);
}

// Example 2: Project organization
async function projectExample() {
    console.log("\n=== Project Organization Example ===");
    
    // Create a project
    const project = new Project('web-app', 'Web application development');
    console.log(`Created project: ${project.name}`);
    
    // Add tasks to project
    const task1 = Task.new('user_123', 'Design UI', '2 hours', Priority.Medium, 'web-app', Status.Todo);
    const task2 = Task.new('user_123', 'Setup database', '4 hours', Priority.High, 'web-app', Status.Todo);
    
    // Using project task container
    const container = ProjectTaskContainer.new('web-app');
    container.add_task(task1);
    container.add_task(task2);
    
    console.log(`Project has ${container.get_all_tasks().length} tasks`);
    
    // Archive completed tasks
    task1.complete();
    container.update_task_status(task1.id, Status.Done);
    console.log(`Active tasks: ${Object.keys(container.active_tasks).length}`);
    console.log(`Completed tasks: ${Object.keys(container.completed_tasks).length}`);
}

// Example 3: Task filtering and searching
async function filteringExample() {
    console.log("\n=== Filtering and Search Example ===");
    
    // Create a task collection
    const collection = TaskCollection.new();
    
    // Add various tasks
    const tasks = [
        Task.newFull('user_123', 'Fix login bug', '1 hour', Priority.Critical, 'web-app', Status.Todo, Assignee.Human, ['bug'], [], null, null),
        Task.newFull('user_123', 'Write documentation', '2 hours', Priority.Medium, 'web-app', Status.InProgress, Assignee.Collaborative, ['docs'], [], null, 30),
        Task.newFull('user_123', 'Refactor API', '5 hours', Priority.High, 'backend', Status.Todo, Assignee.Ai, ['refactor'], [], null, 0)
    ];
    
    tasks.forEach(task => collection.add_task(task));
    
    // Filter by priority
    const highPriorityTasks = collection.get_filtered_tasks({ priority: Priority.High });
    console.log(`High priority tasks: ${highPriorityTasks.length}`);
    
    // Filter by project
    const webAppTasks = collection.get_filtered_tasks({ project: 'web-app' });
    console.log(`Web app tasks: ${webAppTasks.length}`);
    
    // Filter by status
    const todoTasks = collection.get_filtered_tasks({ status: Status.Todo });
    console.log(`Todo tasks: ${todoTasks.length}`);
    
    // Combined filters
    const criticalWebTasks = collection.get_filtered_tasks({
        priority: Priority.Critical,
        project: 'web-app'
    });
    console.log(`Critical web tasks: ${criticalWebTasks.length}`);
}

// Example 4: Tag management
async function taggingExample() {
    console.log("\n=== Tag Management Example ===");
    
    const tagManager = TagManager.new();
    
    // Create tags
    const tagIds = await tagManager.bulkCreateTags(
        ['urgent', 'frontend', 'backend', 'documentation', 'testing'],
        'development'
    );
    console.log(`Created ${tagIds.length} tags`);
    
    // Create tasks with tags
    const task = Task.newFull(
        'user_123',
        'Update API documentation',
        '2 hours',
        Priority.Medium,
        'docs-project',
        Status.Todo,
        null,
        ['documentation', 'api'],
        [],
        null,
        null
    );
    
    // Increment tag usage
    await tagManager.incrementTagUsage('documentation');
    await tagManager.incrementTagUsage('api');
    
    // Search tags
    const docTags = tagManager.searchTags('doc');
    console.log(`Found ${docTags.length} tags matching 'doc'`);
    
    // Get tag statistics
    const stats = tagManager.getTagStatistics();
    console.log(`Total tags: ${stats.total_tags}, Average usage: ${stats.average_usage}`);
}

// Example 5: Storage operations
async function storageExample() {
    console.log("\n=== Storage Operations Example ===");
    
    // Initialize storage
    const storage = await Storage.new();
    console.log("Storage initialized");
    
    // Create and save a task
    const task = Task.new(
        'user_123',
        'Setup project structure',
        '1 hour',
        Priority.High,
        'new-project',
        Status.Todo
    );
    
    // Add task to storage (in a real app, this would save to disk)
    await storage.addTaskToProject(task);
    console.log(`Task added to project: ${task.parentProject}`);
    
    // Retrieve task
    try {
        const retrievedTask = await storage.getTaskFromAnyProject(task.id);
        console.log(`Retrieved task: ${retrievedTask.action}`);
    } catch (error) {
        console.log("Task not found in storage");
    }
}

// Example 6: Error handling
async function errorHandlingExample() {
    console.log("\n=== Error Handling Example ===");
    
    try {
        // This will throw an InvalidProgress error
        Task.newFull(
            'user_123',
            'Invalid task',
            '1 hour',
            Priority.Medium,
            'test-project',
            Status.Todo,
            null,
            [],
            [],
            null,
            150 // Invalid progress (> 100)
        );
    } catch (error) {
        console.log(`Caught error: ${error.type} - ${error.details.progress}`);
    }
    
    try {
        // Parse invalid priority
        Priority.fromStr('invalid');
    } catch (error) {
        console.log(`Caught error: ${error.type} - ${error.details.priority}`);
    }
}

// Run all examples
async function runExamples() {
    try {
        await taskManagementExample();
        await projectExample();
        await filteringExample();
        await taggingExample();
        await storageExample();
        await errorHandlingExample();
        console.log("\n✅ All examples completed successfully!");
    } catch (error) {
        console.error("❌ Error running examples:", error);
    }
}

// Execute the examples
runExamples();

/*
Here's a practical example demonstrating how to use the Todozi task management system with various features:

This example demonstrates:

1. **Task Creation and Management**:
   - Creating tasks with full parameters
   - Updating task progress and status
   - Completing tasks

2. **Project Organization**:
   - Creating projects
   - Adding tasks to projects
   - Managing task status within projects

3. **Filtering and Search**:
   - Using TaskCollection to manage tasks
   - Filtering by priority, project, and status
   - Combining multiple filters

4. **Tag Management**:
   - Creating tags in bulk
   - Associating tags with tasks
   - Searching and analyzing tags

5. **Storage Operations**:
   - Initializing storage
   - Adding and retrieving tasks
   - Error handling for missing tasks

6. **Error Handling**:
   - Handling validation errors
   - Catching specific error types
   - Working with error details

Key features shown:
- Proper use of enums (Priority, Status, Assignee)
- Task lifecycle management
- Project-based organization
- Filtering and search capabilities
- Tag management system
- Error handling patterns
- Storage integration

To run this example:
1. Save as `example_usage.js`
2. Ensure all required modules are available
3. Run with `node example_usage.js`

The example showcases practical usage patterns while demonstrating the core functionality of the Todozi system.
*/