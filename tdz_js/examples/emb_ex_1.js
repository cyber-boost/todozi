import { TodoziEmbeddingService, TodoziEmbeddingConfig, TodoziEmbeddingCache } from '../todozi/emb.js';

// Mock task data for demonstration
const mockTasks = [
    {
        id: 'task_1',
        action: "Implement user authentication system",
        priority: "High",
        status: "Done",
        tags: ["backend", "security"],
        assignee: "ai",
        progress: 100,
        context_notes: "Need to implement JWT-based authentication with refresh tokens",
        embedding_vector: null
    },
    {
        id: 'task_2',
        action: "Build REST API for user management",
        priority: "Medium",
        status: "InProgress",
        tags: ["api", "backend"],
        assignee: "human",
        progress: 60,
        context_notes: "CRUD operations for users with proper validation",
        embedding_vector: null
    },
    {
        id: 'task_3',
        action: "Create database schema for authentication",
        priority: "High",
        status: "Done",
        tags: ["database", "security"],
        assignee: "ai",
        progress: 100,
        context_notes: "Design tables for users, roles, and permissions",
        embedding_vector: null
    }
];

async function demonstrateSemanticSearch() {
    try {
        console.log("🔧 Initializing Todozi Embedding Service...");
        
        // Create embedding service with default configuration
        const config = TodoziEmbeddingConfig.default();
        config.similarity_threshold = 0.6; // Lower threshold for broader results
        config.max_results = 5;
        
        const service = new TodoziEmbeddingService(config);
        await service.initialize();
        
        console.log("✅ Service initialized successfully!");
        console.log(`📊 Model: ${config.model_name}`);
        console.log(`📏 Dimensions: ${config.dimensions}`);
        console.log(`🎯 Similarity threshold: ${config.similarity_threshold}`);
        
        // Simulate embedding generation and caching for mock tasks
        console.log("\n🔍 Generating embeddings for existing tasks...");
        for (const task of mockTasks) {
            const taskContent = service.prepareTaskContent(task);
            const embedding = await service.generateEmbedding(taskContent);
            task.embedding_vector = embedding;
            
            // Cache the embedding
            const cacheKey = `task_${task.id}`;
            const cacheEntry = new TodoziEmbeddingCache(
                embedding,
                'Task',
                task.id,
                taskContent,
                task.tags,
                new Date(),
                config.cache_ttl_seconds
            );
            service.cache.set(cacheKey, cacheEntry);
        }
        
        console.log(`📚 Cached ${mockTasks.length} task embeddings`);
        
        // Perform semantic search
        console.log("\n🔍 Searching for similar tasks...");
        
        // Test queries
        const queries = [
            "security implementation",
            "API development",
            "database design"
        ];
        
        for (const query of queries) {
            console.log(`\n📋 Query: "${query}"`);
            console.log("─".repeat(50));
            
            const results = await service.semanticSearch(query, ['Task'], 3);
            
            if (results.length === 0) {
                console.log("❌ No similar tasks found");
            } else {
                results.forEach((result, index) => {
                    console.log(`${index + 1}. ${result.text_content.split('\n')[0]}`);
                    console.log(`   Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
                    console.log(`   Tags: ${result.tags.join(', ')}`);
                    console.log();
                });
            }
        }
        
        // Demonstrate clustering
        console.log("\n🔗 Clustering related content...");
        const clusters = await service.clusterContent();
        
        if (clusters.length > 0) {
            console.log(`📊 Found ${clusters.length} clusters:`);
            clusters.forEach((cluster, index) => {
                console.log(`\nCluster ${index + 1}:`);
                console.log(`   Size: ${cluster.cluster_size} items`);
                console.log(`   Average similarity: ${(cluster.average_similarity * 100).toFixed(1)}%`);
                cluster.content_items.slice(0, 3).forEach(item => {
                    console.log(`   • ${item.text_content.split('\n')[0]}`);
                });
            });
        } else {
            console.log("❌ No significant clusters found");
        }
        
        // Get statistics
        console.log("\n📈 Getting embedding statistics...");
        const stats = await service.getStats();
        console.log(`Total embeddings: ${stats.total_embeddings}`);
        console.log(`Content type breakdown:`, stats.type_counts);
        
        // Cleanup expired entries (handy for cache management)
        console.log("\n🧹 Cleaning up expired entries...");
        const cleanedCount = await service.cleanupExpired();
        console.log(`Cleaned ${cleanedCount} expired entries`);
        
    } catch (error) {
        console.error("❌ Error:", error.message);
    }
}

// Additional utility function for finding similar tasks from new descriptions
async function findSimilarTasks(service, taskDescription, limit = 5) {
    try {
        console.log(`\n🎯 Searching tasks similar to: "${taskDescription}"`);
        
        const results = await service.findSimilarTasks(taskDescription, limit);
        
        if (results.length === 0) {
            console.log("❌ No similar tasks found");
            return [];
        }
        
        console.log(`📊 Found ${results.length} similar tasks:`);
        results.forEach((result, index) => {
            console.log(`${index + 1}. Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
            console.log(`   Action: ${result.text_content.split('\n')[0].replace('Task: ', '')}`);
            console.log(`   Type: ${result.content_type}`);
            console.log(`   Tags: ${result.tags.join(', ')}`);
            console.log();
        });
        
        return results;
    } catch (error) {
        console.error("❌ Error finding similar tasks:", error.message);
        return [];
    }
}

// Run the demonstration
demonstrateSemanticSearch().then(() => {
    console.log("\n✅ Semantic search demonstration completed!");
});

    demonstrateSemanticSearch,
    findSimilarTasks
};

/*
## Example 1: Semantic Task Search with Embeddings

This example demonstrates how to use the TodoziEmbeddingService to find similar tasks using semantic similarity.

## Example Usage Output:

/ *
🔧 Initializing Todozi Embedding Service...
✅ Service initialized successfully!
📊 Model: sentence-transformers/all-MiniLM-L6-v2
📏 Dimensions: 384
🎯 Similarity threshold: 0.6

🔍 Generating embeddings for existing tasks...
📚 Cached 3 task embeddings

🔍 Searching for similar tasks...

📋 Query: "security implementation"
──────────────────────────────────────────────────
1. Task: Implement user authentication system
   Similarity: 85.2%
   Tags: backend, security

2. Task: Create database schema for authentication
   Similarity: 78.6%
   Tags: database, security

📋 Query: "API development"
──────────────────────────────────────────────────
1. Task: Build REST API for user management
   Similarity: 92.3%
   Tags: api, backend

📋 Query: "database design"
──────────────────────────────────────────────────
1. Task: Create database schema for authentication
   Similarity: 89.7%
   Tags: database, security

🔗 Clustering related content...
📊 Found 1 clusters:

Cluster 1:
   Size: 2 items
   Average similarity: 82.1%
   • Task: Implement user authentication system
   • Task: Create database schema for authentication

📈 Getting embedding statistics...
Total embeddings: 3
Content type breakdown: { Task: 3 }

🧹 Cleaning up expired entries...
Cleaned 0 expired entries

✅ Semantic search demonstration completed!
*/