import { Storage, loadConfig, saveConfig, initStorage } from '../todozi/storage.js';
import { Task, Project, Priority, Status, Assignee } from '../todozi/models.js';
import { 
    TodoziEmbeddingService, 
    TodoziEmbeddingConfig 
} from '../todozi/emb.js';
import { 
    processChatMessageExtended 
} from '../todozi/todozi.js';

async function setupDevelopmentProject() {
    console.log('🚀 Setting up Todozi for Development Project Management\n');
    
    // Initialize storage
    await initStorage();
    const storage = await Storage.new();
    
    // Create a development project
    const devProject = new Project(
        'webapp-dev',
        'Web Application Development Project'
    );
    await saveProject(devProject);
    console.log('✅ Created project: webapp-dev');
    
    // Initialize embedding service for semantic search
    const embeddingConfig = TodoziEmbeddingConfig.default();
    const embeddingService = await TodoziEmbeddingService.new(embeddingConfig);
    
    return { storage, embeddingService, devProject };
}

async function addDevelopmentTasks(storage, embeddingService) {
    console.log('\n📋 Adding development tasks...\n');
    
    const tasks = [
        // Backend tasks
        new Task({
            userId: 'dev-lead',
            action: 'Design REST API endpoints for user authentication',
            time: '4 hours',
            priority: Priority.High,
            parentProject: 'webapp-dev',
            status: Status.Todo,
            assignee: Assignee.Human,
            tags: ['backend', 'api', 'auth'],
            contextNotes: 'Need to design endpoints for login, register, and password reset'
        }),
        
        new Task({
            userId: 'dev-lead',
            action: 'Implement JWT token validation middleware',
            time: '3 hours',
            priority: Priority.High,
            parentProject: 'webapp-dev',
            status: Status.InProgress,
            assignee: Assignee.Agent('coder'),
            tags: ['backend', 'security', 'jwt'],
            dependencies: ['task-1']
        }),
        
        // Frontend tasks
        new Task({
            userId: 'frontend-dev',
            action: 'Create responsive login page component',
            time: '5 hours',
            priority: Priority.Medium,
            parentProject: 'webapp-dev',
            status: Status.Todo,
            assignee: Assignee.Human,
            tags: ['frontend', 'ui', 'auth']
        }),
        
        new Task({
            userId: 'frontend-dev',
            action: 'Implement form validation with error handling',
            time: '3 hours',
            priority: Priority.Medium,
            parentProject: 'webapp-dev',
            status: Status.Todo,
            assignee: Assignee.Ai,
            tags: ['frontend', 'validation', 'ux']
        }),
        
        // Testing tasks
        new Task({
            userId: 'qa-engineer',
            action: 'Write unit tests for authentication service',
            time: '6 hours',
            priority: Priority.High,
            parentProject: 'webapp-dev',
            status: Status.Todo,
            assignee: Assignee.Agent('tester'),
            tags: ['testing', 'backend', 'auth'],
            contextNotes: 'Test coverage should be at least 90%'
        })
    ];
    
    // Add tasks to storage and generate embeddings
    for (const task of tasks) {
        await storage.addTaskToProject(task);
        
        // Generate embedding for semantic search
        const taskContent = embeddingService.prepareTaskContent(task);
        const embedding = await embeddingService.generateEmbedding(taskContent);
        task.embedding_vector = embedding;
        
        console.log(`✅ Added: ${task.action} (${task.priority})`);
    }
    
    return tasks;
}

async function demonstrateSemanticSearch(embeddingService, storage) {
    console.log('\n🔍 Demonstrating Semantic Search...\n');
    
    // Search for similar tasks
    const query = 'implement user login functionality';
    console.log(`Query: "${query}"\n`);
    
    const similarTasks = await embeddingService.findSimilarTasks(query, 3);
    
    if (similarTasks.length > 0) {
        console.log('Found similar tasks:');
        similarTasks.forEach((result, index) => {
            const task = await storage.getTaskFromAnyProject(result.content_id);
            if (task) {
                console.log(`\n${index + 1}. ${task.action}`);
                console.log(`   Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
                console.log(`   Priority: ${task.priority} | Status: ${task.status}`);
                console.log(`   Tags: ${task.tags.join(', ')}`);
            }
        });
    } else {
        console.log('No similar tasks found.');
    }
    
    // Another search example
    console.log('\n' + '='.repeat(50) + '\n');
    const query2 = 'testing authentication';
    console.log(`Query: "${query2}"\n`);
    
    const similarTasks2 = await embeddingService.findSimilarTasks(query2, 2);
    
    if (similarTasks2.length > 0) {
        console.log('Found similar tasks:');
        similarTasks2.forEach((result, index) => {
            const task = await storage.getTaskFromAnyProject(result.content_id);
            if (task) {
                console.log(`\n${index + 1}. ${task.action}`);
                console.log(`   Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
                console.log(`   Assignee: ${task.assignee}`);
            }
        });
    }
}

async function processNaturalLanguageInput(embeddingService) {
    console.log('\n' + '='.repeat(50));
    console.log('💬 Processing Natural Language Input...\n');
    
    // Example chat message with multiple task definitions
    const chatMessage = `
        We need to work on the new feature. Here are the tasks:
        
        <todozi>Set up CI/CD pipeline with GitHub Actions;2 days;high;webapp-dev;todo;human;devops,automation</todozi>
        
        <todozi>Create database migration scripts;4 hours;medium;webapp-dev;todo;agent=coder;backend,database</todozi>
        
        <memory>team adopted agile methodology;improved sprint velocity by 30%;better project management;high;long;agile,scrum</memory>
        
        <error>Database connection pool exhausted;Failed to acquire connection after 30 seconds;high;database;production;connection,timeout</error>
        
        <train>How to optimize database queries;Use proper indexing and query optimization techniques;database,performance;code-review;0.8</train>
    `;
    
    console.log('Processing message...');
    const content = processChatMessageExtended(chatMessage, 'project-manager');
    
    console.log('\n📊 Extracted Content:');
    console.log(`  Tasks: ${content.tasks.length}`);
    console.log(`  Memories: ${content.memories.length}`);
    console.log(`  Errors: ${content.errors.length}`);
    console.log(`  Training Data: ${content.training_data.length}`);
    
    // Display extracted tasks
    content.tasks.forEach((task, index) => {
        console.log(`\n${index + 1}. Task: ${task.action}`);
        console.log(`   Time: ${task.time} | Priority: ${task.priority}`);
        console.log(`   Assignee: ${typeof task.assignee === 'string' ? task.assignee : task.assignee?.name || 'Unspecified'}`);
        console.log(`   Tags: ${task.tags.join(', ')}`);
    });
    
    return content;
}

async function demonstrateTaskClustering(embeddingService) {
    console.log('\n' + '='.repeat(50));
    console.log('🔗 Demonstrating Task Clustering...\n');
    
    // Find semantic clusters in tasks
    const clusters = await embeddingService.clusterContent();
    
    if (clusters.length > 0) {
        console.log(`Found ${clusters.length} semantic clusters:\n`);
        
        clusters.forEach((cluster, index) => {
            console.log(`Cluster ${index + 1}:`);
            console.log(`  Size: ${cluster.cluster_size} tasks`);
            console.log(`  Average Similarity: ${(cluster.average_similarity * 100).toFixed(1)}%`);
            
            // Display first item in each cluster
            if (cluster.content_items.length > 0) {
                const firstItem = cluster.content_items[0];
                const preview = firstItem.text_content.split('\n')[0].substring(0, 60);
                console.log(`  Sample: "${preview}..."`);
            }
            console.log();
        });
    } else {
        console.log('No semantic clusters found. Need more tasks for clustering.');
    }
}

async function updateTaskProgress(storage) {
    console.log('📈 Updating Task Progress...\n');
    
    // Get all tasks from the project
    const containers = await listProjectTaskContainers();
    const webappContainer = containers.find(c => c.project_name === 'webapp-dev');
    
    if (webappContainer) {
        const tasks = webappContainer.get_all_tasks();
        
        // Update progress for some tasks
        for (const task of tasks) {
            if (task.status === Status.InProgress) {
                task.progress = 65;
                task.updated_at = new Date();
                console.log(`Updated progress for: ${task.action} (${task.progress}%)`);
            }
        }
        
        await saveProjectTaskContainer(webappContainer);
    }
}

async function generateProjectSummary(storage) {
    console.log('\n📊 Generating Project Summary...\n');
    
    const containers = await listProjectTaskContainers();
    const webappContainer = containers.find(c => c.project_name === 'webapp-dev');
    
    if (webappContainer) {
        const tasks = webappContainer.get_all_tasks();
        
        // Calculate statistics
        const totalTasks = tasks.length;
        const completedTasks = tasks.filter(t => t.status === Status.Done).length;
        const inProgressTasks = tasks.filter(t => t.status === Status.InProgress).length;
        const todoTasks = tasks.filter(t => t.status === Status.Todo).length;
        
        // Priority breakdown
        const priorityCount = {
            high: tasks.filter(t => t.priority === Priority.High).length,
            medium: tasks.filter(t => t.priority === Priority.Medium).length,
            critical: tasks.filter(t => t.priority === Priority.Critical).length
        };
        
        console.log('Project Summary: webapp-dev');
        console.log('='.repeat(40));
        console.log(`Total Tasks: ${totalTasks}`);
        console.log(`Completed: ${completedTasks} (${((completedTasks/totalTasks)*100).toFixed(1)}%)`);
        console.log(`In Progress: ${inProgressTasks}`);
        console.log(`To Do: ${todoTasks}`);
        console.log('\nPriority Distribution:');
        console.log(`  High: ${priorityCount.high}`);
        console.log(`  Medium: ${priorityCount.medium}`);
        console.log(`  Critical: ${priorityCount.critical}`);
        
        // Calculate completion rate
        const completionRate = (completedTasks / totalTasks) * 100;
        console.log(`\nOverall Completion Rate: ${completionRate.toFixed(1)}%`);
    }
}

// Main execution function
async function runDevelopmentProjectDemo() {
    try {
        // 1. Setup the project
        const { storage, embeddingService, devProject } = await setupDevelopmentProject();
        
        // 2. Add development tasks
        await addDevelopmentTasks(storage, embeddingService);
        
        // 3. Demonstrate semantic search
        await demonstrateSemanticSearch(embeddingService, storage);
        
        // 4. Process natural language input
        await processNaturalLanguageInput(embeddingService);
        
        // 5. Show task clustering
        await demonstrateTaskClustering(embeddingService);
        
        // 6. Update task progress
        await updateTaskProgress(storage);
        
        // 7. Generate project summary
        await generateProjectSummary(storage);
        
        console.log('\n✨ Demo completed successfully!');
        
    } catch (error) {
        console.error('❌ Error during demo:', error.message);
    }
}

// Run the demo
// Run if executed directly
    runDevelopmentProjectDemo();
}

    setupDevelopmentProject,
    addDevelopmentTasks,
    demonstrateSemanticSearch,
    processNaturalLanguageInput,
    demonstrateTaskClustering,
    updateTaskProgress,
    generateProjectSummary,
    runDevelopmentProjectDemo
};

/*
## Example 3: Managing a Software Development Project with Todozi

This example demonstrates how to use Todozi to manage a software development project, including task management, semantic search, and agent collaboration.

## Key Features Demonstrated

1. **Project Initialization**: Creating a development project and initializing the Todozi system

2. **Task Management**: Adding tasks with different priorities, assignees (human, AI, agents), and tags

3. **Semantic Search**: Finding similar tasks using natural language queries

4. **Natural Language Processing**: Parsing chat messages to extract tasks, memories, errors, and training data

5. **Task Clustering**: Automatically grouping related tasks based on semantic similarity

6. **Progress Tracking**: Updating task progress and generating project summaries

7. **Multi-Agent Collaboration**: Assigning tasks to different types of agents (coder, tester, AI)

## Usage

To run this example:

/ *
bash
node example3-software-dev-project.js
*/