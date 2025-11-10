// example2.js - Semantic Task Search and Similarity Example
import { TodoziEmbeddingService, TodoziEmbeddingConfig, Task } from './emb.js';

async function runSemanticTaskExample() {
  try {
    // 1. Initialize embedding service with custom configuration
    const config = new TodoziEmbeddingConfig();
    config.similarity_threshold = 0.6; // Lower threshold for more results
    config.max_results = 20;
    
    const embeddingService = await TodoziEmbeddingService.new(config);
    
    // 2. Create sample tasks with rich content
    const tasks = [
      {
        id: 'task-001',
        action: 'Implement user authentication system',
        context_notes: 'Need OAuth2 integration with Google and GitHub',
        priority: 'High',
        status: 'InProgress',
        parent_project: 'WebApp',
        tags: ['backend', 'security', 'authentication'],
        assignee: 'ai',
        progress: 75
      },
      {
        id: 'task-002',
        action: 'Design responsive UI components',
        context_notes: 'Focus on mobile-first approach using Tailwind CSS',
        priority: 'Medium',
        status: 'Todo',
        parent_project: 'WebApp',
        tags: ['frontend', 'ui', 'design'],
        assignee: 'human'
      },
      {
        id: 'task-003',
        action: 'Setup CI/CD pipeline',
        context_notes: 'Configure automated testing and deployment to AWS',
        priority: 'High',
        status: 'Todo',
        parent_project: 'DevOps',
        tags: ['devops', 'ci/cd', 'aws'],
        assignee: 'ai'
      },
      {
        id: 'task-004',
        action: 'Optimize database queries',
        context_notes: 'Identify and fix N+1 query issues in user profiles',
        priority: 'Critical',
        status: 'Todo',
        parent_project: 'Backend',
        tags: ['database', 'performance', 'optimization'],
        assignee: 'ai'
      }
    ];

    // 3. Add tasks to embedding service (generates embeddings automatically)
    console.log('📝 Adding tasks to embedding service...');
    for (const taskData of tasks) {
      const task = Object.assign(new Task(), taskData);
      await embeddingService.addTask(task);
    }

    // 4. Perform semantic search
    console.log('\n🔍 Performing semantic search for "user login"');
    const searchResults = await embeddingService.semanticSearch(
      'user login', 
      ['Task'], 
      5
    );
    
    console.log(`Found ${searchResults.length} relevant tasks:`);
    searchResults.forEach((result, index) => {
      console.log(`${index + 1}. ${result.text_content.split('\n')[0]}`);
      console.log(`   Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
      console.log(`   Tags: ${result.tags.join(', ')}`);
    });

    // 5. Find similar tasks
    console.log('\n🔗 Finding tasks similar to "authentication system"');
    const similarTasks = await embeddingService.findSimilarTasks(
      'authentication system',
      3
    );
    
    console.log(`Found ${similarTasks.length} similar tasks:`);
    similarTasks.forEach((result, index) => {
      console.log(`${index + 1}. ${result.text_content.split('\n')[0]}`);
      console.log(`   Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
    });

    // 6. Create content clusters
    console.log('\n📊 Generating content clusters...');
    const clusters = await embeddingService.clusterContent();
    console.log(`Created ${clusters.length} clusters:`);
    clusters.forEach((cluster, index) => {
      console.log(`\nCluster ${index + 1} (${cluster.cluster_size} items):`);
      cluster.content_items.forEach((item, itemIndex) => {
        console.log(`  ${itemIndex + 1}. ${item.text_content.split('\n')[0]}`);
      });
    });

    // 7. Hybrid search (semantic + keyword)
    console.log('\n🔍 Performing hybrid search for "security"');
    const hybridResults = await embeddingService.hybridSearch(
      'security',
      ['authentication', 'encryption'],
      ['Task'],
      0.7, // 70% semantic weight
      5
    );
    
    console.log(`Found ${hybridResults.length} hybrid results:`);
    hybridResults.forEach((result, index) => {
      console.log(`${index + 1}. ${result.text_content.split('\n')[0]}`);
      console.log(`   Combined Score: ${(result.similarity_score * 100).toFixed(1)}%`);
      console.log(`   Semantic: ${(result.metadata.semantic_score * 100).toFixed(1)}%`);
      console.log(`   Keywords: ${(result.metadata.keyword_score * 100).toFixed(1)}%`);
    });

    // 8. Get system statistics
    console.log('\n📈 Embedding service statistics:');
    const stats = await embeddingService.getStats();
    console.log(JSON.stringify(stats, null, 2));

  } catch (error) {
    console.error('❌ Error running example:', error.message);
  }
}

// Run the example
runSemanticTaskExample();

/*
Here's a practical example demonstrating how to use the embedding service for semantic search and task similarity:

Key features demonstrated in this example:

1. **Custom Configuration**: Shows how to adjust similarity thresholds and result limits
2. **Task Embedding**: Automatically generates embeddings when adding tasks
3. **Semantic Search**: Finds tasks based on meaning rather than keywords
4. **Similarity Matching**: Identifies related tasks using vector comparisons
5. **Content Clustering**: Groups similar content automatically
6. **Hybrid Search**: Combines semantic and keyword-based search
7. **Statistics**: Provides insights into the embedding system

To run this example:
1. Save as `example2.js`
2. Ensure `emb.js` is in the same directory
3. Run with `node example2.js`

The example will:
- Create embeddings for sample tasks
- Perform various search operations
- Show how similarity scores work
- Demonstrate clustering of related content
- Output formatted results to the console

This showcases the core functionality of the Todozi embedding system for intelligent task management and content discovery.
*/