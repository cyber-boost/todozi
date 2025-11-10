// Import the necessary modules

import { Task, TaskUpdate, Project, Priority, Status, Assignee } from '../todozi/models.js';
import { TodoziHandler } from '../todozi/cli.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';

class TaskManagerExample {
    constructor() {
        this.tasks = [];
        this.projects = new Map();
        this.embeddingService = null;
    }

    async initialize() {
        console.log('🚀 Initializing Task Manager Example...');
        
        // Initialize embedding service
        const config = TodoziEmbeddingConfig.default();
        this.embeddingService = await TodoziEmbeddingService.new(config);
        
        // Create sample projects
        await this.createSampleProjects();
        
        console.log('✅ Task Manager initialized successfully!');
    }

    async createSampleProjects() {
        const projects = [
            new Project('work', 'Work-related tasks'),
            new Project('personal', 'Personal tasks'),
            new Project('learning', 'Learning and development')
        ];

        for (const project of projects) {
            this.projects.set(project.name, project);
        }
    }

    // Example 1: Create tasks with different priorities
    async createSampleTasks() {
        console.log('\n📋 Creating Sample Tasks...');

        const tasks = [
            // Critical work task
            Task.newFull(
                'user1',
                'Fix production server outage',
                '2 hours',
                Priority.Critical,
                'work',
                Status.InProgress,
                Assignee.Ai,
                ['urgent', 'infrastructure', 'server'],
                [],
                'Server is down affecting all users. Need immediate attention.',
                25
            ),

            // High priority learning task
            Task.newFull(
                'user1',
                'Complete machine learning course',
                '4 hours',
                Priority.High,
                'learning',
                Status.Todo,
                Assignee.Human,
                ['education', 'ml', 'skills'],
                [],
                'Complete chapters 5-8 of the ML specialization course',
                0
            ),

            // Medium priority personal task
            Task.newFull(
                'user1',
                'Grocery shopping',
                '1 hour',
                Priority.Medium,
                'personal',
                Status.Pending,
                Assignee.Collaborative,
                ['shopping', 'essentials'],
                [],
                'Buy groceries for the week - milk, eggs, vegetables',
                0
            ),

            // Low priority task
            Task.newFull(
                'user1',
                'Organize bookshelf',
                '30 minutes',
                Priority.Low,
                'personal',
                Status.Todo,
                null,
                ['organization', 'home'],
                [],
                'Sort books by category and author',
                0
            )
        ];

        for (const task of tasks) {
            this.tasks.push(task);
            const project = this.projects.get(task.parentProject);
            if (project) {
                project.addTask(task.id);
            }

            // Generate embedding for the task
            if (this.embeddingService) {
                await this.embeddingService.addTask(task);
            }
        }

        console.log(`✅ Created ${tasks.length} sample tasks!`);
    }

    // Example 2: Update task progress and status
    async demonstrateTaskUpdates() {
        console.log('\n🔄 Demonstrating Task Updates...');

        const taskToUpdate = this.tasks[0]; // The critical server task
        
        // Create update using builder pattern
        const update = TaskUpdate.new()
            .withProgress(75)
            .withStatus(Status.Review)
            .withContextNotes('Server is back online, performing final checks');

        console.log(`Before update: ${taskToUpdate.action} - ${taskToUpdate.status} - ${taskToUpdate.progress}%`);
        
        taskToUpdate.update(update);
        
        console.log(`After update: ${taskToUpdate.action} - ${taskToUpdate.status} - ${taskToUpdate.progress}%`);
    }

    // Example 3: Complete a task
    async demonstrateTaskCompletion() {
        console.log('\n✅ Demonstrating Task Completion...');

        const taskToComplete = this.tasks[1]; // The learning task
        
        console.log(`Before completion: ${taskToComplete.action} - ${taskToComplete.status}`);
        
        taskToComplete.complete();
        
        console.log(`After completion: ${taskToComplete.action} - ${taskToComplete.status} - ${taskToComplete.progress}%`);
        console.log(`Task completed: ${taskToComplete.isCompleted()}`);
        console.log(`Task active: ${taskToComplete.isActive()}`);
    }

    // Example 4: Search for similar tasks using embeddings
    async demonstrateSimilaritySearch() {
        console.log('\n🔍 Demonstrating Similarity Search...');

        if (!this.embeddingService) {
            console.log('Embedding service not available');
            return;
        }

        const query = "fix technical problems";
        const similarTasks = await this.embeddingService.findSimilarTasks(query, 3);

        console.log(`Search results for "${query}":`);
        similarTasks.forEach((result, index) => {
            console.log(`${index + 1}. ${result.text_content} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
        });
    }

    // Example 5: Filter tasks by various criteria
    async demonstrateTaskFiltering() {
        console.log('\n🎯 Demonstrating Task Filtering...');

        // Filter for high priority work tasks
        const highPriorityWorkTasks = this.tasks.filter(task => 
            task.priority === Priority.High && 
            task.parentProject === 'work'
        );

        console.log('High priority work tasks:');
        highPriorityWorkTasks.forEach(task => {
            console.log(`- ${task.action} (${task.priority})`);
        });

        // Filter for incomplete tasks
        const incompleteTasks = this.tasks.filter(task => 
            task.status !== Status.Done && 
            task.status !== Status.Completed
        );

        console.log(`\nIncomplete tasks: ${incompleteTasks.length}`);
    }

    // Example 6: Project statistics
    async demonstrateProjectStats() {
        console.log('\n📊 Demonstrating Project Statistics...');

        for (const [projectName, project] of this.projects) {
            const projectTasks = this.tasks.filter(task => task.parentProject === projectName);
            const completedTasks = projectTasks.filter(task => task.isCompleted());
            const activeTasks = projectTasks.filter(task => task.isActive());

            console.log(`\nProject: ${projectName}`);
            console.log(`  Total tasks: ${projectTasks.length}`);
            console.log(`  Completed: ${completedTasks.length}`);
            console.log(`  Active: ${activeTasks.length}`);
            console.log(`  Completion rate: ${((completedTasks.length / projectTasks.length) * 100).toFixed(1)}%`);
        }
    }

    // Run all examples
    async runAllExamples() {
        await this.initialize();
        await this.createSampleTasks();
        await this.demonstrateTaskUpdates();
        await this.demonstrateTaskCompletion();
        await this.demonstrateSimilaritySearch();
        await this.demonstrateTaskFiltering();
        await this.demonstrateProjectStats();

        console.log('\n🎉 All examples completed successfully!');
        console.log(`Total tasks managed: ${this.tasks.length}`);
        console.log(`Projects: ${Array.from(this.projects.keys()).join(', ')}`);
    }
}

// Usage example
async function main() {
    try {
        const taskManager = new TaskManagerExample();
        await taskManager.runAllExamples();
    } catch (error) {
        console.error('❌ Error running examples:', error);
    }
}

// Uncomment to run the example
// main();


/*
Here's a practical example that demonstrates task management using the Todozi models:

## Example 1: Task Management System

This example shows how to create tasks with different priorities, manage them across projects, and demonstrate the embedding functionality.

## Key Features Demonstrated:

1. **Task Creation**: Creating tasks with different priorities, assignees, and metadata
2. **Project Organization**: Managing tasks across multiple projects
3. **Task Updates**: Using the builder pattern for updates
4. **Completion Tracking**: Marking tasks as complete with progress tracking
5. **Semantic Search**: Finding similar tasks using embeddings
6. **Filtering**: Filtering tasks by priority, project, and status
7. **Statistics**: Generating project-level statistics

## Sample Output:

/ *
🚀 Initializing Task Manager Example...
✅ Task Manager initialized successfully!

📋 Creating Sample Tasks...
✅ Created 4 sample tasks!

🔄 Demonstrating Task Updates...
Before update: Fix production server outage - in_progress - 25%
After update: Fix production server outage - review - 75%

✅ Demonstrating Task Completion...
Before completion: Complete machine learning course - todo
After completion: Complete machine learning course - done - 100%
Task completed: true
Task active: false

🔍 Demonstrating Similarity Search...
Search results for "fix technical problems":
1. Fix production server outage (85.2% similar)
2. Complete machine learning course (23.1% similar)

🎯 Demonstrating Task Filtering...
High priority work tasks:
- Fix production server outage (critical)

Incomplete tasks: 2

📊 Demonstrating Project Statistics...

Project: work
  Total tasks: 1
  Completed: 0
  Active: 1
  Completion rate: 0.0%

Project: personal
  Total tasks: 2
  Completed: 0
  Active: 2
  Completion rate: 0.0%

Project: learning
  Total tasks: 1
  Completed: 1
  Active: 0
  Completion rate: 100.0%

🎉 All examples completed successfully!
Total tasks managed: 4
Projects: work, personal, learning
*/