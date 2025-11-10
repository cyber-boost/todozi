import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { Storage } from '../todozi/storage.js';
import { Priority, Status } from '../todozi/models.js';

class TaskSimilaritySearch {
  constructor() {
    this.embeddingService = null;
    this.storage = null;
  }

  async initialize() {
    // Initialize the embedding service with default configuration
    const config = TodoziEmbeddingConfig.default();
    this.embeddingService = await TodoziEmbeddingService.new(config);
    
    // Initialize storage
    this.storage = await Storage.new();
    
    console.log('✅ Task similarity search system initialized');
  }

  /**
   * Find tasks similar to a given query using semantic search
   */
  async findSimilarTasks(query, options = {}) {
    const {
      limit = 10,
      projectId = null,
      minSimilarity = 0.6,
      includeCompleted = false
    } = options;

    try {
      // Perform semantic search across all task embeddings
      const results = await this.embeddingService.semanticSearch(
        query,
        ['Task'], // Only search tasks
        limit
      );

      // Filter and enrich results
      const similarTasks = [];
      for (const result of results) {
        if (result.similarity_score < minSimilarity) continue;

        // Retrieve the actual task from storage
        const task = await this.storage.getTaskFromAnyProject(result.content_id);
        if (!task) continue;

        // Apply additional filters
        if (projectId && task.parentProject !== projectId) continue;
        if (!includeCompleted && task.status === Status.Done) continue;

        similarTasks.push({
          ...task,
          similarityScore: result.similarity_score,
          matchedContent: result.text_content
        });
      }

      // Sort by similarity score (highest first)
      similarTasks.sort((a, b) => b.similarityScore - a.similarityScore);

      return similarTasks.slice(0, limit);
    } catch (error) {
      console.error('Error finding similar tasks:', error.message);
      return [];
    }
  }

  /**
   * Get task recommendations based on a set of reference tasks
   */
  async getRecommendations(basedOnTaskIds, options = {}) {
    const { limit = 5, excludeProject = null } = options;

    try {
      // Collect embeddings for reference tasks
      const referenceTasks = [];
      for (const taskId of basedOnTaskIds) {
        const task = await this.storage.getTaskFromAnyProject(taskId);
        if (task) referenceTasks.push(task);
      }

      if (referenceTasks.length === 0) {
        console.log('⚠️ No valid reference tasks found');
        return [];
      }

      // Get recommendations using the embedding service
      const recommendations = await this.embeddingService.recommendSimilar(
        basedOnTaskIds,
        [], // No explicit exclusions
        limit
      );

      // Enrich recommendations with task details
      const enrichedRecommendations = [];
      for (const rec of recommendations) {
        const task = await this.storage.getTaskFromAnyProject(rec.content_id);
        if (!task) continue;

        // Skip tasks from excluded project
        if (excludeProject && task.parentProject === excludeProject) continue;

        enrichedRecommendations.push({
          ...task,
          recommendationScore: rec.similarity_score,
          reason: 'Similar to your completed tasks'
        });
      }

      return enrichedRecommendations;
    } catch (error) {
      console.error('Error getting recommendations:', error.message);
      return [];
    }
  }

  /**
   * Analyze task patterns and suggest tags
   */
  async suggestTagsForTask(taskId) {
    try {
      // Get tag suggestions from embedding service
      const suggestions = await this.embeddingService.suggestTags(taskId, 10);
      
      // Retrieve the task to provide context
      const task = await this.storage.getTaskFromAnyProject(taskId);
      if (!task) return [];

      return {
        taskId,
        taskTitle: task.action,
        suggestedTags: suggestions,
        currentTags: task.tags || []
      };
    } catch (error) {
      console.error('Error suggesting tags:', error.message);
      return null;
    }
  }

  /**
   * Find duplicate or very similar tasks
   */
  async findPotentialDuplicates(projectId = null, threshold = 0.85) {
    try {
      const tasks = await this.storage.listTasksAcrossProjects({
        project: projectId,
        status: Status.Todo // Only check active tasks
      });

      const duplicates = [];
      const processed = new Set();

      for (const task of tasks) {
        if (processed.has(task.id)) continue;

        // Find similar tasks to the current task
        const similar = await this.embeddingService.semanticSearch(
          task.action,
          ['Task'],
          5
        );

        for (const result of similar) {
          if (result.content_id === task.id) continue;
          if (result.similarity_score < threshold) continue;
          if (processed.has(result.content_id)) continue;

          const similarTask = await this.storage.getTaskFromAnyProject(result.content_id);
          if (!similarTask) continue;

          duplicates.push({
            task1: task,
            task2: similarTask,
            similarity: result.similarity_score,
            reason: 'Very similar task descriptions'
          });

          processed.add(task.id);
          processed.add(result.content_id);
        }
      }

      return duplicates;
    } catch (error) {
      console.error('Error finding duplicates:', error.message);
      return [];
    }
  }

  /**
   * Get semantic clustering of tasks
   */
  async getTaskClusters(projectId = null) {
    try {
      // Filter tasks by project if specified
      let taskIds = [];
      const tasks = await this.storage.listTasksAcrossProjects({
        project: projectId
      });
      
      taskIds = tasks.map(t => t.id);

      // Perform hierarchical clustering
      const clusters = await this.embeddingService.hierarchicalClustering(
        ['Task'],
        3 // Max depth of 3 levels
      );

      // Enrich clusters with task details
      const enrichedClusters = [];
      for (const cluster of clusters) {
        const clusterTasks = [];
        for (const item of cluster.content_items) {
          const task = await this.storage.getTaskFromAnyProject(item.content_id);
          if (task) clusterTasks.push(task);
        }

        enrichedClusters.push({
          clusterId: cluster.cluster_id,
          level: cluster.level,
          tasks: clusterTasks,
          averageSimilarity: cluster.average_similarity,
          description: `Cluster of ${clusterTasks.length} similar tasks`
        });
      }

      return enrichedClusters;
    } catch (error) {
      console.error('Error getting task clusters:', error.message);
      return [];
    }
  }
}

// Usage example
async function demonstrateSimilaritySearch() {
  const search = new TaskSimilaritySearch();
  await search.initialize();

  console.log('\n🔍 Task Similarity Search Demo');
  console.log('═══════════════════════════════════');

  // Example 1: Find tasks similar to a query
  console.log('\n1. Finding tasks similar to "database optimization":');
  const similarTasks = await search.findSimilarTasks('database optimization', {
    limit: 5,
    minSimilarity: 0.5
  });

  if (similarTasks.length > 0) {
    console.log('Found similar tasks:');
    similarTasks.forEach(task => {
      console.log(`  • ${task.action} (${(task.similarityScore * 100).toFixed(1)}% similar)`);
    });
  } else {
    console.log('No similar tasks found');
  }

  // Example 2: Get recommendations based on completed tasks
  console.log('\n2. Getting task recommendations:');
  const recommendations = await search.getRecommendations(['task_123', 'task_456'], {
    limit: 3
  });

  if (recommendations.length > 0) {
    console.log('Recommended tasks:');
    recommendations.forEach(task => {
      console.log(`  • ${task.action} (${(task.recommendationScore * 100).toFixed(1)}% match)`);
    });
  }

  // Example 3: Suggest tags for a task
  console.log('\n3. Suggesting tags for a task:');
  const tagSuggestions = await search.suggestTagsForTask('task_789');
  if (tagSuggestions) {
    console.log(`Task: ${tagSuggestions.taskTitle}`);
    console.log('Suggested tags:', tagSuggestions.suggestedTags.join(', '));
    console.log('Current tags:', tagSuggestions.currentTags.join(', '));
  }

  // Example 4: Find potential duplicates
  console.log('\n4. Checking for duplicate tasks:');
  const duplicates = await search.findPotentialDuplicates('web-project', 0.8);
  if (duplicates.length > 0) {
    console.log(`Found ${duplicates.length} potential duplicates:`);
    duplicates.forEach(dup => {
      console.log(`  • "${dup.task1.action}" and "${dup.task2.action}"`);
      console.log(`    Similarity: ${(dup.similarity * 100).toFixed(1)}%`);
    });
  } else {
    console.log('No potential duplicates found');
  }

  // Example 5: Get task clusters
  console.log('\n5. Task clustering analysis:');
  const clusters = await search.getTaskClusters('web-project');
  if (clusters.length > 0) {
    console.log(`Found ${clusters.length} task clusters:`);
    clusters.forEach(cluster => {
      console.log(`  • Cluster ${cluster.clusterId}: ${cluster.tasks.length} tasks`);
      console.log(`    Avg similarity: ${(cluster.averageSimilarity * 100).toFixed(1)}%`);
    });
  } else {
    console.log('No task clusters found');
  }
}

// Command-line interface
async function main() {
  const command = process.argv[2];
  const search = new TaskSimilaritySearch();
  await search.initialize();

  switch (command) {
    case 'search':
      const query = process.argv[3];
      if (!query) {
        console.error('Usage: node task_similarity_search.js search "<query>"');
        process.exit(1);
      }
      const results = await search.findSimilarTasks(query);
      results.forEach(task => {
        console.log(`[${task.id}] ${task.action}`);
        console.log(`  Project: ${task.parentProject}`);
        console.log(`  Priority: ${task.priority}`);
        console.log(`  Similarity: ${(task.similarityScore * 100).toFixed(1)}%`);
        console.log();
      });
      break;

    case 'duplicates':
      const dupResults = await search.findPotentialDuplicates();
      if (dupResults.length > 0) {
        console.log('Potential duplicates found:');
        dupResults.forEach(dup => {
          console.log(`"${dup.task1.action}" ↔ "${dup.task2.action}"`);
          console.log(`  Similarity: ${(dup.similarity * 100).toFixed(1)}%`);
        });
      } else {
        console.log('No duplicates found');
      }
      break;

    case 'clusters':
      const clusterResults = await search.getTaskClusters();
      clusterResults.forEach(cluster => {
        console.log(`\nCluster ${cluster.clusterId} (Level ${cluster.level}):`);
        console.log(`  Tasks: ${cluster.tasks.length}`);
        console.log(`  Avg Similarity: ${(cluster.averageSimilarity * 100).toFixed(1)}%`);
        cluster.tasks.slice(0, 3).forEach(task => {
          console.log(`    - ${task.action}`);
        });
      });
      break;

    case 'demo':
      await demonstrateSimilaritySearch();
      break;

    default:
      console.log('Usage: node task_similarity_search.js <command>');
      console.log('Commands: search, duplicates, clusters, demo');
  }
}

main().catch(console.error);

/*
# Example 3: Embedding-Based Task Similarity Search

This example demonstrates how to use the embedding system to find similar tasks and provide intelligent task recommendations.

## File: task_similarity_search.js

## Key Features Demonstrated:

1. **Semantic Task Search**: Find tasks similar to a natural language query
2. **Task Recommendations**: Get suggestions based on completed tasks
3. **Tag Suggestions**: AI-powered tag recommendations for tasks
4. **Duplicate Detection**: Find potential duplicate tasks to avoid redundancy
5. **Task Clustering**: Group similar tasks together for better organization

## Usage Examples:

/*
bash
# Find tasks similar to "database optimization"
node task_similarity_search.js search "database optimization"

# Check for duplicate tasks
node task_similarity_search.js duplicates

# View task clusters
node task_similarity_search.js clusters

# Run full demonstration
node task_similarity_search.js demo
*/