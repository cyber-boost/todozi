import { Task, TaskUpdate, Priority, Status, Assignee, Storage, QueueItem, QueueStatus } from '../todozi/models.js';

import { TodoziEmbeddingService } from '../todozi/emb.js';

// Initialize storage and embedding service
async function initializeServices() {
    const storage = await Storage.new();
    const embeddingService = await TodoziEmbeddingService.new();
    return { storage, embeddingService };
}

// Create a complex project with dependent tasks
async function createProjectWithDependencies() {
    const { storage, embeddingService } = await initializeServices();
    
    // Create main project task
    const mainTask = Task.newFull(
        'project_manager',
        'Build E-commerce Platform',
        '6 months',
        Priority.High,
        'ecommerce_platform',
        Status.Todo,
        Assignee.Collaborative,
        ['backend', 'frontend', 'database'],
        [],
        'Complete e-commerce solution with payment processing'
    );
    
    // Add main task to storage
    await storage.addTaskToProject(mainTask);
    console.log(`✅ Created main task: ${mainTask.id}`);
    
    // Create database design task (dependency)
    const dbTask = Task.newFull(
        'database_architect',
        'Design Database Schema',
        '2 weeks',
        Priority.Critical,
        'ecommerce_platform',
        Status.Todo,
        Assignee.Human,
        ['database', 'design'],
        [],
        'Design normalized schema for users, products, and orders'
    );
    
    // Create API development task (depends on database)
    const apiTask = Task.newFull(
        'backend_developer',
        'Develop REST API',
        '4 weeks',
        Priority.High,
        'ecommerce_platform',
        Status.Todo,
        Assignee.Agent('coder'),
        ['backend', 'api'],
        [dbTask.id], // Depends on database design
        'Implement CRUD operations for all entities'
    );
    
    // Create frontend task (depends on API)
    const frontendTask = Task.newFull(
        'frontend_developer',
        'Build React Frontend',
        '6 weeks',
        Priority.High,
        'ecommerce_platform',
        Status.Todo,
        Assignee.Agent('coder'),
        ['frontend', 'react'],
        [apiTask.id], // Depends on API
        'Create responsive UI with shopping cart functionality'
    );
    
    // Create payment integration task
    const paymentTask = Task.newFull(
        'payment_specialist',
        'Integrate Payment Gateway',
        '2 weeks',
        Priority.Critical,
        'ecommerce_platform',
        Status.Todo,
        Assignee.Agent('devops'),
        ['payment', 'integration'],
        [apiTask.id], // Depends on API
        'Integrate Stripe payment processing'
    );
    
    // Add all tasks to storage
    for (const task of [dbTask, apiTask, frontendTask, paymentTask]) {
        await storage.addTaskToProject(task);
        console.log(`✅ Created task: ${task.id} with ${task.dependencies.length} dependencies`);
    }
    
    return { mainTask, dbTask, apiTask, frontendTask, paymentTask };
}

// Track task progress and update dependencies
async function updateTaskProgress(taskId, progressPercentage, statusUpdate = null) {
    const { storage } = await initializeServices();
    
    // Get current task
    const task = await storage.getTaskFromAnyProject(taskId);
    if (!task) {
        throw new Error(`Task ${taskId} not found`);
    }
    
    // Create update with progress
    const update = TaskUpdate.new()
        .withProgress(progressPercentage);
    
    if (statusUpdate) {
        update.withStatus(statusUpdate);
    }
    
    // Apply update
    await storage.updateTaskInProject(taskId, update);
    
    // Check for dependent tasks that might be ready to start
    const dependentTasks = await findDependentTasks(taskId);
    console.log(`📊 Task ${taskId} updated to ${progressPercentage}% complete`);
    console.log(`🔗 Found ${dependentTasks.length} dependent tasks`);
    
    for (const dependent of dependentTasks) {
        if (await areAllDependenciesComplete(dependent.id)) {
            console.log(`⚡ Task ${dependent.id} is now ready to start!`);
            // Optionally auto-update dependent task status
            await storage.updateTaskInProject(dependent.id, 
                TaskUpdate.new().withStatus(Status.InProgress)
            );
        }
    }
    
    return task;
}

// Find tasks that depend on a given task
async function findDependentTasks(taskId) {
    const { storage } = await initializeServices();
    const allTasks = await storage.listTasksAcrossProjects({});
    
    return allTasks.filter(task => 
        task.dependencies.includes(taskId)
    );
}

// Check if all dependencies for a task are complete
async function areAllDependenciesComplete(taskId) {
    const { storage } = await initializeServices();
    const task = await storage.getTaskFromAnyProject(taskId);
    
    if (!task || task.dependencies.length === 0) {
        return true;
    }
    
    for (const depId of task.dependencies) {
        const depTask = await storage.getTaskFromAnyProject(depId);
        if (!depTask || depTask.status !== Status.Done) {
            return false;
        }
    }
    
    return true;
}

// Create a task queue for workflow management
async function setupTaskQueue() {
    const { storage } = await initializeServices();
    
    // Create queue items for tasks that are ready to start
    const tasks = await storage.listTasksAcrossProjects({ status: Status.Todo });
    
    for (const task of tasks) {
        if (await areAllDependenciesComplete(task.id)) {
            const queueItem = new QueueItem({
                taskName: task.action,
                taskDescription: task.contextNotes || `Task: ${task.action}`,
                priority: task.priority,
                projectId: task.parentProject
            });
            
            // Add to queue (simplified - in real implementation would use queue storage)
            console.log(`📋 Queued task: ${task.id} - ${task.action}`);
        }
    }
}

// Generate progress report for a project
async function generateProgressReport(projectName) {
    const { storage, embeddingService } = await initializeServices();
    
    const projectTasks = await storage.listTasksAcrossProjects({ project: projectName });
    
    const report = {
        projectName,
        totalTasks: projectTasks.length,
        completedTasks: 0,
        inProgressTasks: 0,
        blockedTasks: 0,
        averageProgress: 0,
        criticalPath: []
    };
    
    let totalProgress = 0;
    
    for (const task of projectTasks) {
        // Count by status
        if (task.status === Status.Done) {
            report.completedTasks++;
        } else if (task.status === Status.InProgress) {
            report.inProgressTasks++;
        } else if (task.status === Status.Blocked) {
            report.blockedTasks++;
        }
        
        // Sum progress
        if (task.progress !== null) {
            totalProgress += task.progress;
        }
        
        // Identify critical path tasks (high priority with dependencies)
        if (task.priority === Priority.Critical && task.dependencies.length > 0) {
            report.criticalPath.push({
                id: task.id,
                action: task.action,
                progress: task.progress,
                dependencies: task.dependencies.length
            });
        }
    }
    
    // Calculate average progress
    report.averageProgress = projectTasks.length > 0 ? 
        Math.round(totalProgress / projectTasks.length) : 0;
    
    // Find similar tasks for recommendations
    if (projectTasks.length > 0) {
        const sampleTask = projectTasks[0];
        const similarTasks = await embeddingService.findSimilarTasks(
            sampleTask.action, 
            5
        );
        report.recommendations = similarTasks.map(t => ({
            action: t.textContent.split('\n')[0],
            similarity: Math.round(t.similarityScore * 100)
        }));
    }
    
    return report;
}

// Main execution
async function main() {
    try {
        console.log('🚀 Starting Advanced Task Management Example\n');
        
        // Create project with dependencies
        const tasks = await createProjectWithDependencies();
        
        // Setup task queue
        await setupTaskQueue();
        
        // Simulate task progress updates
        console.log('\n📈 Updating task progress...');
        
        // Complete database design (first task)
        await updateTaskProgress(tasks.dbTask.id, 100, Status.Done);
        
        // Update API progress
        await updateTaskProgress(tasks.apiTask.id, 50);
        
        // Generate progress report
        console.log('\n📊 Generating progress report...');
        const report = await generateProgressReport('ecommerce_platform');
        
        console.log('\n═════════════════════════════════════════');
        console.log('         PROJECT PROGRESS REPORT');
        console.log('═════════════════════════════════════════');
        console.log(`Project: ${report.projectName}`);
        console.log(`Total Tasks: ${report.totalTasks}`);
        console.log(`Completed: ${report.completedTasks}`);
        console.log(`In Progress: ${report.inProgressTasks}`);
        console.log(`Blocked: ${report.blockedTasks}`);
        console.log(`Average Progress: ${report.averageProgress}%`);
        
        if (report.criticalPath.length > 0) {
            console.log('\n🔥 Critical Path:');
            report.criticalPath.forEach(task => {
                console.log(`  • ${task.action} (${task.progress}% complete)`);
            });
        }
        
        if (report.recommendations.length > 0) {
            console.log('\n💡 Similar Tasks for Reference:');
            report.recommendations.forEach(rec => {
                console.log(`  • ${rec.action} (${rec.similarity}% similar)`);
            });
        }
        
        console.log('\n✅ Advanced task management example completed!');
        
    } catch (error) {
        console.error('❌ Error:', error.message);
        console.error(error.stack);
    }
}

// Execute if run directly
// Run if executed directly
    main();
}

    createProjectWithDependencies,
    updateTaskProgress,
    findDependentTasks,
    areAllDependenciesComplete,
    setupTaskQueue,
    generateProgressReport
};

/*
# Example 3: Advanced Task Management with Dependencies and Progress Tracking

This example demonstrates how to create complex tasks with dependencies, track progress, and manage task relationships using the Todozi system.

## Key Features Demonstrated:

1. **Task Dependencies**: Creating tasks that depend on other tasks being completed first
2. **Progress Tracking**: Updating task progress percentage and status
3. **Dependency Resolution**: Automatically checking when dependent tasks can start
4. **Task Queue Management**: Queueing tasks that are ready to start
5. **Progress Reporting**: Generating comprehensive project status reports
6. **Semantic Search**: Finding similar tasks using embeddings for recommendations
7. **Critical Path Identification**: Highlighting high-priority tasks with dependencies

## How to Use:

/ *
bash
# Run the example
node example3-advanced-task-management.js

# The output will show:
# - Task creation with dependencies
# - Progress updates
# - Automatic dependency resolution
# - Detailed progress report with critical path
# - Similar task recommendations
*/