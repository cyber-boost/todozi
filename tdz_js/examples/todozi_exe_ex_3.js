
import { AdvancedSearchEngine } from './advanced_search.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { Storage } from '../todozi/storage.js';
import { Priority, Status, Assignee } from '../todozi/models.js';

const engine = new AdvancedSearchEngine();
await engine.initialize();

// Search for high-priority backend tasks
const results = await engine.searchTasks({
  query: 'implement REST API endpoints',
  filters: {
    priority: 'high',
    tags: ['backend', 'api'],
    status: 'todo'
  },
  limit: 10,
  includeContext: true
});

// Find tasks similar to an existing one
const similar = await engine.findSimilarTasks('task_123', {
  limit: 5,
  excludeSelf: true
});

// Get recommendations based on completed tasks
const recommendations = await engine.getRecommendations(
  ['task_123', 'task_456'],
  { limit: 10 }
);

/*
// advanced_search.js

class AdvancedSearchEngine {
  constructor() {
    this.embeddingService = null;
    this.storage = null;
  }

  async initialize() {
    // Initialize storage
    this.storage = await Storage.new();
    
    // Initialize embedding service with custom configuration
    const config = new TodoziEmbeddingConfig();
    config.model_name = "sentence-transformers/all-mpnet-base-v2";
    config.similarity_threshold = 0.65; // Lower threshold for more results
    config.max_results = 20;
    
    this.embeddingService = await TodoziEmbeddingService.new(config);
    await this.embeddingService.initialize();
    
    console.log('✅ Advanced Search Engine initialized');
  }

  
   * Search tasks using semantic similarity with optional filters
   * @param {Object} options - Search options
   * @returns {Promise<Array>} Array of similar tasks with metadata
   */
  async searchTasks(options = {}) {
    const {
      query,
      limit = 10,
      semanticWeight = 0.7,
      keywordWeight = 0.3,
      filters = {},
      includeContext = false
    } = options;

    if (!query) {
      throw new Error('Query parameter is required');
    }

    // Extract keywords from the query
    const keywords = this.extractKeywords(query);
    
    // Define content types to search
    const contentTypes = filters.contentTypes || ['Task'];
    
    // Perform hybrid search combining semantic and keyword matching
    const results = await this.embeddingService.hybridSearch(
      query,
      keywords,
      contentTypes,
      semanticWeight,
      limit * 2 // Get more results initially for filtering
    );

    // Apply additional filters
    const filteredResults = await this.applyFilters(results, filters);
    
    // Enrich results with task details if requested
    if (includeContext) {
      for (const result of filteredResults) {
        try {
          const task = await this.storage.getTaskFromAnyProject(result.content_id);
          if (task) {
            result.taskDetails = {
              project: task.parentProject,
              status: task.status,
              priority: task.priority,
              assignee: task.assignee,
              tags: task.tags,
              progress: task.progress,
              createdAt: task.createdAt
            };
          }
        } catch (e) {
          console.warn(`Failed to load task ${result.content_id}:`, e.message);
        }
      }
    }

    // Sort by combined score and return limited results
    return filteredResults
      .sort((a, b) => b.similarity_score - a.similarity_score)
      .slice(0, limit);
  }

  
   * Find similar tasks based on an existing task ID
   * @param {string} taskId - The reference task ID
   * @param {Object} options - Search options
   * @returns {Promise<Array>} Array of similar tasks
   */
  async findSimilarTasks(taskId, options = {}) {
    const { limit = 10, excludeSelf = true } = options;

    // Get the reference task
    const referenceTask = await this.storage.getTaskFromAnyProject(taskId);
    if (!referenceTask) {
      throw new Error(`Task not found: ${taskId}`);
    }

    // Prepare content for embedding
    const taskContent = this.embeddingService.prepareTaskContent(referenceTask);
    
    // Find similar tasks
    const similarTasks = await this.embeddingService.findSimilarTasks(taskContent, limit + 1);
    
    // Exclude the reference task if requested
    const results = excludeSelf 
      ? similarTasks.filter(r => r.content_id !== taskId)
      : similarTasks;

    // Enrich with task details
    for (const result of results) {
      try {
        const task = await this.storage.getTaskFromAnyProject(result.content_id);
        if (task) {
          result.taskDetails = {
            project: task.parentProject,
            status: task.status,
            priority: task.priority,
            assignee: task.assignee,
            tags: task.tags,
            progress: task.progress,
            createdAt: task.createdAt
          };
        }
      } catch (e) {
        console.warn(`Failed to load task ${result.content_id}:`, e.message);
      }
    }

    return results;
  }

  
   * Search across multiple content types (tasks, memories, ideas)
   * @param {string} query - Search query
   * @param {Object} options - Search options
   * @returns {Promise<Object>} Results grouped by content type
   */
  async searchAllContent(query, options = {}) {
    const { limit = 10, contentTypes = ['Task', 'Memory', 'Idea'] } = options;
    
    const results = {
      tasks: [],
      memories: [],
      ideas: [],
      total: 0
    };

    // Search each content type separately
    for (const contentType of contentTypes) {
      const contentResults = await this.embeddingService.semanticSearch(
        query,
        [contentType],
        limit
      );

      // Convert to consistent format
      const formattedResults = contentResults.map(result => ({
        id: result.content_id,
        type: result.content_type,
        score: result.similarity_score,
        content: result.text_content,
        tags: result.tags
      }));

      // Group results by type
      switch (contentType) {
        case 'Task':
          results.tasks = formattedResults;
          break;
        case 'Memory':
          results.memories = formattedResults;
          break;
        case 'Idea':
          results.ideas = formattedResults;
          break;
      }
    }

    results.total = results.tasks.length + results.memories.length + results.ideas.length;
    return results;
  }

  
   * Get recommendations based on a set of tasks
   * @param {Array} taskIds - Array of task IDs to base recommendations on
   * @param {Object} options - Recommendation options
   * @returns {Promise<Array>} Recommended tasks
   */
  async getRecommendations(taskIds, options = {}) {
    const { limit = 10, exclude = [] } = options;

    if (taskIds.length === 0) {
      throw new Error('At least one task ID is required');
    }

    // Get recommendations from embedding service
    const recommendations = await this.embeddingService.recommendSimilar(
      taskIds,
      [...exclude, ...taskIds], // Exclude provided tasks
      limit
    );

    // Enrich with task details
    for (const result of recommendations) {
      try {
        const task = await this.storage.getTaskFromAnyProject(result.content_id);
        if (task) {
          result.taskDetails = {
            project: task.parentProject,
            status: task.status,
            priority: task.priority,
            assignee: task.assignee,
            tags: task.tags,
            progress: task.progress,
            createdAt: task.createdAt
          };
        }
      } catch (e) {
        console.warn(`Failed to load task ${result.content_id}:`, e.message);
      }
    }

    return recommendations;
  }

  
   * Apply additional filters to search results
   * @param {Array} results - Initial search results
   * @param {Object} filters - Filter criteria
   * @returns {Promise<Array>} Filtered results
   */
  async applyFilters(results, filters) {
    if (Object.keys(filters).length === 0) {
      return results;
    }

    const filtered = [];
    
    for (const result of results) {
      let include = true;
      
      // Get task details for filtering
      let taskDetails = null;
      try {
        const task = await this.storage.getTaskFromAnyProject(result.content_id);
        if (task) {
          taskDetails = {
            project: task.parentProject,
            status: task.status,
            priority: task.priority,
            assignee: task.assignee,
            tags: task.tags,
            progress: task.progress,
            createdAt: task.createdAt
          };
        }
      } catch (e) {
        // If we can't load task details, skip filtering for this result
        continue;
      }

      // Apply filters
      if (filters.project && taskDetails.project !== filters.project) {
        include = false;
      }
      
      if (filters.status && taskDetails.status !== filters.status) {
        include = false;
      }
      
      if (filters.priority && taskDetails.priority !== filters.priority) {
        include = false;
      }
      
      if (filters.assignee && JSON.stringify(taskDetails.assignee) !== JSON.stringify(filters.assignee)) {
        include = false;
      }
      
      if (filters.tags && !filters.tags.some(tag => taskDetails.tags.includes(tag))) {
        include = false;
      }
      
      if (filters.minProgress !== undefined && 
          (taskDetails.progress === null || taskDetails.progress < filters.minProgress)) {
        include = false;
      }
      
      if (filters.maxProgress !== undefined && 
          (taskDetails.progress === null || taskDetails.progress > filters.maxProgress)) {
        include = false;
      }
      
      if (filters.dateFrom && new Date(taskDetails.createdAt) < new Date(filters.dateFrom)) {
        include = false;
      }
      
      if (filters.dateTo && new Date(taskDetails.createdAt) > new Date(filters.dateTo)) {
        include = false;
      }

      if (include) {
        result.taskDetails = taskDetails;
        filtered.push(result);
      }
    }

    return filtered;
  }

  
   * Extract keywords from a query string
   * @param {string} query - The search query
   * @returns {Array} Array of keywords
   */
  extractKeywords(query) {
    // Simple keyword extraction - remove stop words and split
    const stopWords = new Set(['the', 'a', 'an', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for', 'of', 'with', 'by']);
    
    return query
      .toLowerCase()
      .replace(/[^\w\s]/g, ' ')
      .split(/\s+/)
      .filter(word => word.length > 2 && !stopWords.has(word));
  }

  
   * Get search statistics and diagnostics
   * @returns {Promise<Object>} Search engine statistics
   */
  async getStats() {
    const embeddingStats = await this.embeddingService.getStats();
    const diagnosticReport = await this.embeddingService.exportDiagnostics();
    
    return {
      embeddings: embeddingStats,
      diagnostics: diagnosticReport,
      timestamp: new Date().toISOString()
    };
  }
}

// Example usage
async function demonstrateAdvancedSearch() {
  const searchEngine = new AdvancedSearchEngine();
  await searchEngine.initialize();

  console.log('\n🔍 Advanced Search Examples\n');

  // Example 1: Semantic search with filters
  console.log('1. Semantic search with filters:');
  const semanticResults = await searchEngine.searchTasks({
    query: 'implement user authentication system',
    limit: 5,
    filters: {
      status: 'todo',
      priority: 'high',
      tags: ['backend', 'security']
    },
    includeContext: true
  });
  
  console.log(`Found ${semanticResults.length} similar tasks:`);
  semanticResults.forEach((result, i) => {
    console.log(`  ${i + 1}. [${result.taskDetails.project}] ${result.content} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
  });

  // Example 2: Find similar tasks
  console.log('\n2. Find similar tasks based on existing task:');
  const similarResults = await searchEngine.findSimilarTasks('task_abc123', {
    limit: 3,
    excludeSelf: true
  });
  
  console.log(`Found ${similarResults.length} similar tasks:`);
  similarResults.forEach((result, i) => {
    console.log(`  ${i + 1}. [${result.taskDetails.project}] ${result.content} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
  });

  // Example 3: Search across all content types
  console.log('\n3. Search across all content types:');
  const allContentResults = await searchEngine.searchAllContent('database optimization');
  
  console.log(`Found ${allContentResults.total} total items:`);
  console.log(`  Tasks: ${allContentResults.tasks.length}`);
  console.log(`  Memories: ${allContentResults.memories.length}`);
  console.log(`  Ideas: ${allContentResults.ideas.length}`);

  // Example 4: Get recommendations
  console.log('\n4. Get recommendations based on multiple tasks:');
  const recommendations = await searchEngine.getRecommendations(
    ['task_abc123', 'task_def456'],
    { limit: 3 }
  );
  
  console.log(`Found ${recommendations.length} recommended tasks:`);
  recommendations.forEach((result, i) => {
    console.log(`  ${i + 1}. [${result.taskDetails.project}] ${result.content} (${(result.similarity_score * 100).toFixed(1)}% relevant)`);
  });

  // Example 5: Get search statistics
  console.log('\n5. Search engine statistics:');
  const stats = await searchEngine.getStats();
  console.log(`  Total embeddings: ${stats.embeddings.total_embeddings}`);
  console.log(`  Content types: ${Object.keys(stats.embeddings.type_counts).join(', ')}`);
  console.log(`  Average similarity: ${(stats.diagnostics.avg_similarity_score * 100).toFixed(2)}%`);
}

// Export for use
  AdvancedSearchEngine,
  demonstrateAdvancedSearch
};

// Run demonstration if this file is executed directly
// Run if executed directly
  demonstrateAdvancedSearch().catch(console.error);
}

/ *
# Example 3: Advanced Task Search and Filtering with Embeddings

This example demonstrates how to use Todozi's embedding service to perform advanced semantic search across tasks, with filtering capabilities and result ranking.

## Key Features Demonstrated:

1. **Hybrid Search**: Combines semantic similarity with keyword matching for more relevant results
2. **Advanced Filtering**: Filter results by project, status, priority, assignee, tags, progress, and date ranges
3. **Multi-type Search**: Search across tasks, memories, and ideas simultaneously
4. **Similar Task Discovery**: Find tasks similar to a reference task using embeddings
5. **Recommendations**: Get task recommendations based on a set of completed tasks
6. **Context Enrichment**: Automatically enrich search results with task details
7. **Statistics & Diagnostics**: Monitor search engine performance and health

## Usage Example:

This example showcases how Todozi's embedding service can be leveraged to create powerful search capabilities that understand the semantic meaning of tasks, not just keyword matching. The integration with storage allows for rich filtering and context enrichment, making it easy to find exactly what you're looking for in your task management system.