
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from './todozi/emb.js';
import { Storage } from './todozi/storage.js';
import { TodoziHandler } from './todozi/cli/TodoziHandler.js';
import { Task, Priority, Status } from './todozi/models.js';

async function advancedTaskManagementDemo() {
  // Initialize storage and embedding service
  const storage = new Storage();
  await storage.init();
  
  const embeddingConfig = new TodoziEmbeddingConfig();
  embeddingConfig.model_name = "sentence-transformers/all-mpnet-base-v2";
  embeddingConfig.similarity_threshold = 0.75;
  
  const embeddingService = await TodoziEmbeddingService.new(embeddingConfig);
  const handler = new TodoziHandler(storage);
  
  console.log('🚀 Advanced Task Management Demo');
  console.log('=====================================');
  
  // Step 1: Create sample tasks with semantic content
  console.log('\n📝 Creating sample tasks...');
  
  const tasks = [
    {
      action: "Implement user authentication system with OAuth2",
      time: "2 days",
      priority: Priority.High,
      project: "webapp",
      status: Status.Todo,
      tags: ["auth", "security", "backend"],
      context: "Need to integrate Google and GitHub OAuth providers"
    },
    {
      action: "Design database schema for user management",
      time: "1 day",
      priority: Priority.High,
      project: "webapp",
      status: Status.Done,
      tags: ["database", "design", "backend"],
      context: " PostgreSQL with proper indexing"
    },
    {
      action: "Write unit tests for authentication endpoints",
      time: "3 hours",
      priority: Priority.Medium,
      project: "webapp",
      status: Status.Todo,
      tags: ["testing", "auth", "backend"],
      context: "Coverage should be above 90%"
    },
    {
      action: "Create login page UI component",
      time: "4 hours",
      priority: Priority.Medium,
      project: "webapp",
      status: Status.InProgress,
      tags: ["ui", "frontend", "auth"],
      context: "React component with form validation"
    }
  ];
  
  // Add tasks to storage
  for (const taskData of tasks) {
    const task = Task.newFull(
      'demo_user',
      taskData.action,
      taskData.time,
      taskData.priority,
      taskData.project,
      taskData.status,
      null,
      taskData.tags,
      [],
      taskData.context,
      null
    );
    
    await handler.handleAddCommand({
      type: 'Task',
      action: taskData.action,
      time: taskData.time,
      priority: taskData.priority,
      project: taskData.project,
      status: taskData.status,
      tags: taskData.tags.join(','),
      context: taskData.context
    });
  }
  
  console.log(`✅ Created ${tasks.length} tasks`);
  
  // Step 2: Perform semantic search
  console.log('\n🔍 Performing semantic search...');
  
  const searchQueries = [
    "user login security",
    "database user management",
    "testing authentication",
    "frontend login interface"
  ];
  
  for (const query of searchQueries) {
    console.log(`\n📋 Searching for: "${query}"`);
    const similarTasks = await embeddingService.findSimilarTasks(query, 3);
    
    if (similarTasks.length > 0) {
      console.log(`Found ${similarTasks.length} similar tasks:`);
      similarTasks.forEach((result, index) => {
        console.log(`  ${index + 1}. ${result.content_id} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
        console.log(`     ${result.text_content.split('\n')[0]}`);
      });
    } else {
      console.log('  No similar tasks found');
    }
  }
  
  // Step 3: Cluster related tasks
  console.log('\n🔗 Clustering related tasks...');
  
  const clusters = await embeddingService.clusterContent();
  console.log(`Found ${clusters.length} semantic clusters:`);
  
  clusters.forEach((cluster, index) => {
    console.log(`\nCluster ${index + 1}: ${cluster.cluster_size} tasks`);
    console.log(`Average similarity: ${(cluster.average_similarity * 100).toFixed(1)}%`);
    cluster.content_items.forEach(item => {
      console.log(`  - ${item.content_id}: ${item.text_content.split('\n')[0]}`);
    });
  });
  
  // Step 4: Multi-query search with aggregation
  console.log('\n🎯 Multi-query search with aggregation...');
  
  const queries = ["user authentication", "database design", "testing"];
  const aggregationTypes = ['Average', 'Max', 'Min'];
  
  for (const aggregationType of aggregationTypes) {
    console.log(`\n📊 Aggregation: ${aggregationType}`);
    const results = await embeddingService.multiQuerySearch(
      queries,
      { type: aggregationType },
      ['Task'],
      5
    );
    
    results.forEach((result, index) => {
      console.log(`  ${index + 1}. ${result.content_id} (${(result.similarity_score * 100).toFixed(1)}%)`);
      console.log(`     ${result.text_content.split('\n')[0]}`);
    });
  }
  
  // Step 5: Filtered semantic search
  console.log('\n🔽 Filtered semantic search...');
  
  const filters = {
    priority: [Priority.High],
    tags: ["auth"],
    status: [Status.Todo, Status.InProgress]
  };
  
  const filteredResults = await embeddingService.filteredSemanticSearch(
    "user login",
    filters,
    5
  );
  
  console.log(`\nFiltered results (High priority, auth tags, active):`);
  filteredResults.forEach((result, index) => {
    console.log(`  ${index + 1}. ${result.content_id} (${(result.similarity_score * 100).toFixed(1)}%)`);
    console.log(`     ${result.text_content.split('\n')[0]}`);
  });
  
  // Step 6: Get statistics and diagnostics
  console.log('\n📈 Embedding statistics...');
  
  const stats = await embeddingService.getStats();
  console.log(`Total embeddings: ${stats.total_embeddings}`);
  console.log('Type breakdown:');
  Object.entries(stats.type_counts).forEach(([type, count]) => {
    console.log(`  ${type}: ${count}`);
  });
  
  console.log('\n🔍 Diagnostic report...');
  const diagnostics = await embeddingService.exportDiagnostics();
  console.log(`Cache hit rate: ${(diagnostics.cache_hit_rate * 100).toFixed(1)}%`);
  console.log(`Average similarity: ${(diagnostics.avg_similarity_score * 100).toFixed(1)}%`);
  console.log(`Top similar pairs:`);
  diagnostics.top_similar_pairs.slice(0, 3).forEach(([id1, id2, sim]) => {
    console.log(`  ${id1} ↔ ${id2}: ${(sim * 100).toFixed(1)}%`);
  });
  
  // Step 7: Performance profiling
  console.log('\n⚡ Performance profiling...');
  
  const testQuery = "user authentication";
  const performanceMetrics = await embeddingService.profileSearchPerformance(testQuery, 5);
  
  console.log(`Query: "${testQuery}"`);
  console.log(`Average time: ${performanceMetrics.avg_time_ms.toFixed(2)}ms`);
  console.log(`Min time: ${performanceMetrics.min_time_ms}ms`);
  console.log(`Max time: ${performanceMetrics.max_time_ms}ms`);
  console.log(`Std dev: ${performanceMetrics.std_dev_ms.toFixed(2)}ms`);
  console.log(`Results per iteration: ${performanceMetrics.results_per_iteration}`);
  
  // Step 8: Suggest tags for existing content
  console.log('\n💡 Tag suggestions...');
  
  const allTasks = await storage.listTasksAcrossProjects({});
  if (allTasks.length > 0) {
    const sampleTask = allTasks[0];
    const suggestedTags = await embeddingService.suggestTags(sampleTask.id, 5);
    
    console.log(`Suggested tags for task "${sampleTask.action}":`);
    suggestedTags.forEach((tag, index) => {
      console.log(`  ${index + 1}. ${tag}`);
    });
  }
  
  // Step 9: Export embeddings for external use
  console.log('\n💾 Exporting embeddings...');
  
  const outputPath = './embeddings_export.json';
  await storage.exportEmbeddedTasksHlx(outputPath);
  console.log(`✅ Embeddings exported to: ${outputPath}`);
  
  console.log('\n🎉 Demo completed successfully!');
  console.log('\nKey features demonstrated:');
  console.log('  • Semantic task search with similarity scoring');
  console.log('  • Automatic task clustering based on content');
  console.log('  • Multi-query search with different aggregation methods');
  console.log('  • Filtered search combining semantic and metadata filters');
  console.log('  • Performance profiling and diagnostics');
  console.log('  • Intelligent tag suggestions');
  console.log('  • Embedding export for external ML processing');
}

// Helper function to demonstrate cross-content relationships
async function demonstrateCrossContentRelationships() {
  const embeddingService = await TodoziEmbeddingService.new();
  
  console.log('\n🔗 Cross-content relationships...');
  
  // Find relationships between tasks and other content types
  const relationships = await embeddingService.findCrossContentRelationships(
    'task_12345',
    'Task',
    0.6
  );
  
  Object.entries(relationships).forEach(([contentType, items]) => {
    console.log(`\n${contentType} relationships:`);
    items.forEach((item, index) => {
      console.log(`  ${index + 1}. ${item.content_id} (${(item.similarity_score * 100).toFixed(1)}%)`);
    });
  });
}

// Helper function to show embedding drift tracking
async function demonstrateEmbeddingDrift() {
  const embeddingService = await TodoziEmbeddingService.new();
  
  console.log('\n📊 Tracking embedding drift...');
  
  const taskId = 'task_12345';
  const originalText = "Implement user authentication";
  const updatedText = "Implement OAuth2 user authentication with multi-factor support";
  
  const driftReport = await embeddingService.trackEmbeddingDrift(taskId, updatedText);
  
  console.log(`Task: ${taskId}`);
  console.log(`Original similarity: ${(driftReport.current_similarity_to_original * 100).toFixed(1)}%`);
  console.log(`Drift percentage: ${driftReport.drift_percentage.toFixed(1)}%`);
  console.log(`Significant drift: ${driftReport.significant_drift ? 'Yes' : 'No'}`);
  
  if (driftReport.significant_drift) {
    console.log('⚠️  Task content has changed significantly - consider updating embeddings');
  }
}

// Run the demo
// Run if executed directly
  advancedTaskManagementDemo()
    .then(() => process.exit(0))
    .catch(error => {
      console.error('Demo failed:', error);
      process.exit(1);
    });
}

  advancedTaskManagementDemo,
  demonstrateCrossContentRelationships,
  demonstrateEmbeddingDrift
};

/*
# Example 3: Advanced Task Management with Semantic Search

This example demonstrates how to use Todozi's semantic search capabilities to find similar tasks, manage project workflows, and integrate with AI agents for intelligent task processing.

## Key Features Demonstrated:

1. **Semantic Search**: Find tasks based on meaning rather than exact keywords
2. **Task Clustering**: Automatically group related tasks using vector similarity
3. **Multi-query Search**: Combine multiple search queries with different aggregation methods
4. **Filtered Search**: Combine semantic search with metadata filters (priority, tags, status)
5. **Performance Profiling**: Measure search performance and optimize configurations
6. **Tag Suggestions**: AI-powered tag suggestions based on similar content
7. **Embedding Export**: Export embeddings for external ML processing
8. **Cross-content Relationships**: Find relationships between tasks and other content types
9. **Drift Tracking**: Monitor how task content changes over time affects embeddings

## Usage:

/ *
bash
# Run the demo
node advanced-task-demo.js

# Output will show:
# - Created tasks with embeddings
# - Semantic search results with similarity scores
# - Task clusters with groupings
# - Performance metrics
# - Tag suggestions
# - Export completion
*/