// example2.js - Task Management with AI Features

import { TodoziHandler } from './todozi/cli/TodoziHandler.js';
import { Storage } from '../todozi/todozi.js';
import { TodoziEmbeddingConfig, TodoziEmbeddingService } from './todozi/emb.js';

async function runTaskManagementExample() {
    try {
        // Initialize storage and handler
        const storage = new Storage();
        await storage.init();
        const handler = new TodoziHandler(storage);

        // Initialize embedding service for AI features
        const embeddingConfig = new TodoziEmbeddingConfig();
        const embeddingService = new TodoziEmbeddingService(embeddingConfig);
        await embeddingService.init();

        console.log("=== Todozi Task Management Example ===\n");

        // 1. Create a new project
        console.log("1. Creating project 'Website Redesign'");
        await handler.handleProjectCommand({
            type: 'Create',
            name: 'Website Redesign',
            description: 'Complete overhaul of company website'
        });

        // 2. Add tasks with different assignees
        console.log("\n2. Adding tasks");
        await handler.handleAddCommand({
            type: 'Task',
            action: 'Design new homepage layout',
            time: '4 hours',
            priority: 'high',
            project: 'Website Redesign',
            status: 'todo',
            assignee: 'ai',
            tags: 'design,ui,homepage',
            context: 'Focus on mobile-first design principles'
        });

        await handler.handleAddCommand({
            type: 'Task',
            action: 'Write product descriptions',
            time: '3 hours',
            priority: 'medium',
            project: 'Website Redesign',
            status: 'todo',
            assignee: 'human',
            tags: 'content,writing,products',
            context: 'Use SEO best practices'
        });

        // 3. List all tasks in the project
        console.log("\n3. Current tasks in 'Website Redesign':");
        await handler.handleListCommand({
            type: 'Tasks',
            project: 'Website Redesign'
        });

        // 4. Use AI to find similar tasks
        console.log("\n4. Finding similar tasks to 'web page design':");
        const similarTasks = await embeddingService.findSimilarTasks('web page design', 5);
        similarTasks.forEach((result, index) => {
            console.log(`  ${index + 1}. ${result.textContent.split('\n')[0]} (${(result.similarityScore * 100).toFixed(1)}% similar)`);
        });

        // 5. Update a task
        console.log("\n5. Updating task progress");
        const tasks = await storage.listTasksAcrossProjects({ project: 'Website Redesign' });
        const taskId = tasks[0].id;
        
        await handler.handleUpdateCommand(taskId, {
            progress: 50,
            status: 'in_progress'
        });

        // 6. Show updated task
        console.log("\n6. Updated task details:");
        await handler.handleShowCommand({
            type: 'Task',
            id: taskId
        });

        // 7. Complete a task
        console.log("\n7. Completing task");
        await handler.completeTask(taskId);

        // 8. Show final project stats
        console.log("\n8. Final project statistics:");
        await handler.handleStatsCommand({});

        console.log("\n=== Example completed successfully ===");

    } catch (error) {
        console.error("Error in example:", error.message);
    }
}

// Run the example
runTaskManagementExample();

/*
Here's a practical example showing how to use the Todozi system to manage tasks with AI integration:

This example demonstrates:

1. **Project Management**:
   - Creating a new project
   - Adding tasks with different assignees (AI vs human)
   - Listing project tasks

2. **AI Integration**:
   - Initializing the embedding service
   - Finding similar tasks using semantic search
   - Updating task progress

3. **Task Lifecycle**:
   - Creating tasks with detailed metadata
   - Updating task status and progress
   - Completing tasks
   - Viewing project statistics

Key features shown:
- Task assignment to AI/human agents
- Semantic search for similar tasks
- Progress tracking
- Project-based organization
- Rich metadata (tags, context, priority)

To run this example:
1. Save as `example2.js`
2. Ensure Todozi is initialized (`todozi init`)
3. Execute with: `node example2.js`

The example will create a "Website Redesign" project with sample tasks, demonstrate AI-powered task similarity search, and show the complete task lifecycle from creation to completion.
*/