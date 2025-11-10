import { TodoziEmbeddingService, TodoziEmbeddingConfig, SimilarityResult, ClusteringResult } from '../todozi/emb.js';
import { Priority, Status, Task, Memory, Idea } from '../todozi/models.js';

async function semanticSearchExample() {
    console.log('🔍 Todozi Semantic Search & Clustering Example');
    console.log('==========================================\n');

    // 1. Initialize the embedding service
    const config = new TodoziEmbeddingConfig();
    config.model_name = "sentence-transformers/all-MiniLM-L6-v2";
    config.dimensions = 384;
    config.similarity_threshold = 0.6;
    config.enable_clustering = true;
    config.clustering_threshold = 0.7;
    
    const embeddingService = await TodoziEmbeddingService.new(config);
    console.log('✅ Embedding service initialized');

    // 2. Create sample tasks
    const tasks = [
        new Task({
            userId: 'user1',
            action: 'Implement user authentication system',
            time: '2 days',
            priority: Priority.High,
            parentProject: 'web-app',
            status: Status.InProgress,
            tags: ['security', 'backend', 'auth'],
            contextNotes: 'Need to implement JWT-based authentication with refresh tokens'
        }),
        new Task({
            userId: 'user1',
            action: 'Design database schema for user profiles',
            time: '3 hours',
            priority: Priority.Medium,
            parentProject: 'web-app',
            status: Status.Todo,
            tags: ['database', 'design', 'users'],
            contextNotes: 'Create tables for users, roles, and permissions'
        }),
        new Task({
            userId: 'user1',
            action: 'Fix authentication bug in production',
            time: '1 hour',
            priority: Priority.Critical,
            parentProject: 'web-app',
            status: Status.Todo,
            tags: ['security', 'bug', 'urgent'],
            contextNotes: 'Users cannot log in due to token validation issue'
        }),
        new Task({
            userId: 'user1',
            action: 'Create API documentation',
            time: '4 hours',
            priority: Priority.Low,
            parentProject: 'documentation',
            status: Status.Todo,
            tags: ['docs', 'api', 'writing'],
            contextNotes: 'Document all REST endpoints with examples'
        }),
        new Task({
            userId: 'user1',
            action: 'Implement password reset feature',
            time: '3 hours',
            priority: Priority.Medium,
            parentProject: 'web-app',
            status: Status.Todo,
            tags: ['security', 'auth', 'feature'],
            contextNotes: 'Add forgot password functionality with email verification'
        })
    ];

    // 3. Embed tasks
    console.log('\n📝 Embedding tasks...');
    for (const task of tasks) {
        await embeddingService.addTask(task);
        console.log(`✅ Embedded: ${task.action}`);
    }

    // 4. Create sample ideas
    const ideas = [
        {
            id: 'idea1',
            idea: 'Use biometric authentication for better security',
            projectId: 'web-app',
            status: 'active',
            share: 'team',
            importance: 'high',
            tags: ['security', 'innovation', 'auth'],
            context: 'Explore fingerprint or face recognition options'
        },
        {
            id: 'idea2',
            idea: 'Implement real-time collaboration features',
            projectId: 'web-app',
            status: 'active',
            share: 'public',
            importance: 'medium',
            tags: ['feature', 'real-time', 'collaboration'],
            context: 'Allow multiple users to work on documents simultaneously'
        },
        {
            id: 'idea3',
            idea: 'Add AI-powered task recommendations',
            projectId: 'ai-integration',
            status: 'active',
            share: 'team',
            importance: 'high',
            tags: ['ai', 'automation', 'features'],
            context: 'Use ML to suggest relevant tasks based on user behavior'
        }
    ];

    console.log('\n💡 Embedding ideas...');
    for (const idea of ideas) {
        await embeddingService.newIdea(idea);
        console.log(`✅ Embedded: ${idea.idea}`);
    }

    // 5. Create sample memories
    const memories = [
        {
            id: 'mem1',
            userId: 'user1',
            projectId: 'web-app',
            status: 'active',
            moment: 'Last security audit revealed token expiration issues',
            meaning: 'JWT tokens need proper refresh mechanism',
            reason: 'Security vulnerability found',
            importance: 'critical',
            term: 'long',
            memoryType: 'standard',
            tags: ['security', 'audit', 'jwt'],
            createdAt: new Date(),
            updatedAt: new Date()
        },
        {
            id: 'mem2',
            userId: 'user1',
            projectId: 'web-app',
            status: 'active',
            moment: 'User feedback showed password complexity requirements too strict',
            meaning: 'Balance security with user experience',
            reason: 'UX improvement needed',
            importance: 'medium',
            term: 'short',
            memoryType: 'standard',
            tags: ['ux', 'security', 'feedback'],
            createdAt: new Date(),
            updatedAt: new Date()
        }
    ];

    console.log('\n🧠 Embedding memories...');
    for (const memory of memories) {
        await embeddingService.newMemory(memory);
        console.log(`✅ Embedded: ${memory.moment}`);
    }

    // 6. Perform semantic search
    console.log('\n🔍 Performing semantic searches...\n');

    // Search for authentication-related content
    console.log('📋 Search: "authentication and security"');
    const authResults = await embeddingService.semanticSearch(
        'authentication and security',
        ['Task', 'Idea', 'Memory'],
        5
    );
    
    console.log(`Found ${authResults.length} results:`);
    authResults.forEach((result, i) => {
        console.log(`  ${i + 1}. [${result.content_type}] ${result.text_content.substring(0, 60)}... (${(result.similarity_score * 100).toFixed(1)}%)`);
    });

    // Search for database-related content
    console.log('\n🗄️  Search: "database design"');
    const dbResults = await embeddingService.semanticSearch(
        'database design',
        ['Task'],
        3
    );
    
    console.log(`Found ${dbResults.length} results:`);
    dbResults.forEach((result, i) => {
        console.log(`  ${i + 1}. ${result.text_content.substring(0, 60)}... (${(result.similarity_score * 100).toFixed(1)}%)`);
    });

    // 7. Find similar tasks
    console.log('\n🔄 Finding similar tasks for: "Implement user authentication system"');
    const similarTasks = await embeddingService.findSimilarTasks(
        'Implement user authentication system',
        3
    );
    
    console.log(`Found ${similarTasks.length} similar tasks:`);
    similarTasks.forEach((result, i) => {
        console.log(`  ${i + 1}. ${result.text_content.substring(0, 60)}... (${(result.similarity_score * 100).toFixed(1)}%)`);
    });

    // 8. Generate clusters
    console.log('\n🔗 Generating content clusters...');
    const clusters = await embeddingService.clusterContent();
    console.log(`Generated ${clusters.length} semantic clusters:\n`);
    
    clusters.forEach((cluster, i) => {
        console.log(`📦 Cluster ${i + 1} (${cluster.cluster_size} items, ${(cluster.average_similarity * 100).toFixed(1)}% avg similarity):`);
        cluster.content_items.slice(0, 3).forEach((item, j) => {
            console.log(`    ${j + 1}. [${item.content_type}] ${item.text_content.substring(0, 50)}...`);
        });
        if (cluster.content_items.length > 3) {
            console.log(`    ...and ${cluster.content_items.length - 3} more`);
        }
        console.log('');
    });

    // 9. Hybrid search (semantic + keyword)
    console.log('🔍 Hybrid search: "security token" with keywords ["bug", "urgent"]');
    const hybridResults = await embeddingService.hybridSearch(
        'security token',
        ['bug', 'urgent'],
        ['Task'],
        0.6, // semantic weight
        3
    );
    
    console.log(`Found ${hybridResults.length} hybrid results:`);
    hybridResults.forEach((result, i) => {
        const meta = result.metadata;
        console.log(`  ${i + 1}. ${result.text_content.substring(0, 60)}...`);
        console.log(`     Semantic: ${(meta.semantic_score * 100).toFixed(1)}%, Keyword: ${(meta.keyword_score * 100).toFixed(1)}%, Combined: ${(meta.combined_score * 100).toFixed(1)}%`);
    });

    // 10. Multi-query search
    console.log('\n🔍 Multi-query search: ["authentication", "security", "user"]');
    const multiResults = await embeddingService.multiQuerySearch(
        ['authentication', 'security', 'user'],
        'Average', // aggregation type
        ['Task'],
        3
    );
    
    console.log(`Found ${multiResults.length} results matching all queries:`);
    multiResults.forEach((result, i) => {
        console.log(`  ${i + 1}. ${result.text_content.substring(0, 60)}... (${(result.similarity_score * 100).toFixed(1)}%)`);
    });

    // 11. Get statistics
    console.log('\n📊 Embedding Statistics:');
    const stats = await embeddingService.getStats();
    console.log(`  Total embeddings: ${stats.total_embeddings}`);
    console.log(`  Type breakdown:`);
    Object.entries(stats.type_counts).forEach(([type, count]) => {
        console.log(`    ${type}: ${count}`);
    });

    // 12. Suggest tags for content
    console.log('\n🏷️  Tag suggestions for first task...');
    const suggestedTags = await embeddingService.suggestTags(tasks[0].id, 5);
    if (suggestedTags.length > 0) {
        console.log('  Suggested tags:');
        suggestedTags.forEach((tag, i) => {
            console.log(`    ${i + 1}. ${tag}`);
        });
    } else {
        console.log('  No tag suggestions available');
    }

    // 13. Performance profiling
    console.log('\n⚡ Performance profiling for search query...');
    const perfMetrics = await embeddingService.profileSearchPerformance(
        'authentication security',
        5 // iterations
    );
    
    console.log('  Performance metrics:');
    console.log(`    Average time: ${perfMetrics.avg_time_ms.toFixed(2)}ms`);
    console.log(`    Min time: ${perfMetrics.min_time_ms}ms`);
    console.log(`    Max time: ${perfMetrics.max_time_ms}ms`);
    console.log(`    Std deviation: ${perfMetrics.std_dev_ms.toFixed(2)}ms`);
    console.log(`    Results per search: ${perfMetrics.results_per_iteration}`);

    // 14. Diagnostics
    console.log('\n🔬 Embedding diagnostics...');
    const diagnostics = await embeddingService.exportDiagnostics();
    console.log(`  Cache hit rate: ${diagnostics.cache_hit_rate || 'N/A'}`);
    console.log(`  Average similarity: ${(diagnostics.avg_similarity_score * 100).toFixed(2)}%`);
    console.log(`  Content type breakdown:`);
    Object.entries(diagnostics.content_type_breakdown).forEach(([type, count]) => {
        console.log(`    ${type}: ${count}`);
    });

    console.log('\n🎉 Example completed successfully!');
}

// Run the example
semanticSearchExample().catch(console.error);

/*
# Example: Todozi Semantic Search and Clustering

This example demonstrates how to use the Todozi embedding service to perform semantic search and clustering on various types of content.

## What This Example Demonstrates

This example showcases the key capabilities of the Todozi embedding service:

1. **Service Initialization**: Setting up the embedding service with custom configuration

2. **Content Embedding**: Embedding different types of content (tasks, ideas, memories)

3. **Semantic Search**: Finding content based on semantic similarity rather than just keywords

4. **Similarity Matching**: Finding items similar to a specific piece of content

5. **Clustering**: Automatically grouping related content together

6. **Hybrid Search**: Combining semantic and keyword search for better results

7. **Multi-Query Search**: Searching across multiple queries simultaneously

8. **Tag Suggestions**: Recommending tags based on similar content

9. **Performance Monitoring**: Measuring search performance

10. **Diagnostics**: Getting insights into the embedding system's health

## Key Features Highlighted

- **Semantic understanding**: The system understands that "authentication" and "security" are related concepts
- **Cross-type search**: Can search across tasks, ideas, and memories simultaneously
- **Flexible configuration**: Thresholds and parameters can be adjusted based on needs
- **Performance optimization**: Caching and efficient similarity calculations
- **Rich metadata**: Search results include detailed similarity metrics

This example can be run as a standalone script to test the embedding functionality or as a reference for implementing similar features in your Todozi application.
*/