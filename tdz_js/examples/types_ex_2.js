// example2.js - Todozi Task Management Example

import { TodoziEmbeddingService, TodoziEmbeddingConfig } from './emb.js';
import { Storage } from './storage.js';
import { Task, Priority, Status, Assignee } from './models.js';

async function runTodoziExample() {
    try {
        // Initialize storage and embedding service
        const storage = await Storage.new();
        const config = TodoziEmbeddingConfig.default();
        const embeddingService = await TodoziEmbeddingService.new(config);
        
        // Create sample tasks with different priorities and assignees
        const tasks = [
            Task.newFull(
                'user123',
                'Implement user authentication',
                '2 days',
                Priority.High,
                'web-app',
                Status.Todo,
                Assignee.Ai,
                ['auth', 'security'],
                [],
                'Create login/registration flow',
                null
            ),
            Task.newFull(
                'user123',
                'Design dashboard UI',
                '3 days',
                Priority.Medium,
                'web-app',
                Status.InProgress,
                Assignee.Human,
                ['ui', 'design'],
                ['auth-system'],
                'Focus on user experience',
                60
            ),
            Task.newFull(
                'user123',
                'Setup CI/CD pipeline',
                '1 day',
                Priority.Critical,
                'dev-ops',
                Status.Todo,
                Assignee.Collaborative,
                ['deployment', 'automation'],
                [],
                'Use GitHub Actions',
                null
            )
        ];

        // Add tasks to storage and generate embeddings
        console.log('Adding tasks...');
        for (const task of tasks) {
            await storage.addTaskToProject(task);
            if (task.embedding_vector) {
                await embeddingService.addTask(task);
            }
            console.log(`✓ Added: ${task.action} (${task.priority})`);
        }

        // Find similar tasks to "user login system"
        console.log('\nFinding similar tasks...');
        const similarTasks = await embeddingService.findSimilarTasks('user login system', 5);
        console.log(`Found ${similarTasks.length} similar tasks:`);
        similarTasks.forEach((result, index) => {
            console.log(`${index + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
        });

        // Semantic search for "security implementation"
        console.log('\nSearching for security-related tasks...');
        const searchResults = await embeddingService.semanticSearch('security implementation', null, 5);
        console.log(`Found ${searchResults.length} results:`);
        searchResults.forEach((result, index) => {
            console.log(`${index + 1}. ${result.text_content.split('\n')[0]}`);
        });

        // Update a task
        console.log('\nUpdating task progress...');
        const taskToUpdate = tasks[1]; // Dashboard UI task
        taskToUpdate.update({
            progress: 75,
            status: Status.Review
        });
        await storage.addTaskToProject(taskToUpdate);
        console.log(`✓ Updated: ${taskToUpdate.action} to ${taskToUpdate.progress}%`);

        // Cluster similar content
        console.log('\nClustering content...');
        const clusters = await embeddingService.clusterContent();
        console.log(`Found ${clusters.length} content clusters:`);
        clusters.forEach((cluster, index) => {
            console.log(`Cluster ${index + 1} (${cluster.cluster_size} items):`);
            cluster.content_items.slice(0, 2).forEach(item => {
                console.log(`  - ${item.text_content.split('\n')[0]}`);
            });
            if (cluster.content_items.length > 2) {
                console.log(`  ... and ${cluster.content_items.length - 2} more`);
            }
        });

        // Show statistics
        console.log('\nSystem Statistics:');
        const stats = await embeddingService.getStats();
        Object.entries(stats).forEach(([key, value]) => {
            console.log(`  ${key}: ${JSON.stringify(value)}`);
        });

    } catch (error) {
        console.error('Error in Todozi example:', error);
    }
}

// Run the example
runTodoziExample();

/*
Here's a practical example demonstrating how to use the Todozi system to create and manage tasks with structured data:

This example demonstrates:

1. **Task Creation**: Creating tasks with different priorities, assignees, and metadata
2. **Embedding Integration**: Generating embeddings for semantic search capabilities
3. **Similarity Search**: Finding tasks similar to a query
4. **Semantic Search**: Searching tasks by meaning rather than keywords
5. **Task Updates**: Modifying task progress and status
6. **Content Clustering**: Grouping similar tasks together
7. **Statistics**: Getting system usage metrics

Key features shown:
- Using structured task objects with proper typing
- Leveraging the embedding service for AI-powered features
- Working with different assignee types (AI, Human, Collaborative)
- Managing task relationships through dependencies
- Semantic search capabilities beyond traditional text matching

To run this example:
1. Ensure all required modules are available
2. Initialize the Todozi storage system
3. Execute the script to see task management in action

The output will show:
- Task creation confirmation
- Similar task matches with similarity scores
- Search results for security-related tasks
- Task update confirmation
- Content clusters grouping similar tasks
- System statistics summary

This demonstrates how Todozi combines traditional task management with AI-powered semantic capabilities for more intelligent organization and retrieval of tasks.
*/