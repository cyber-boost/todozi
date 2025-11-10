import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { TodoziError, ErrorSeverity, ErrorCategory } from '../todozi/error.js';
import { TagManager, TagSearchEngine } from '../todozi/tags.js';
import { Task, Priority, Status, Assignee } from '../todozi/models.js';
import { parseTodoziFormat, processChatMessageExtended } from '../todozi/todozi.js';

class SmartTaskManager {
  constructor() {
    this.embeddingService = null;
    this.tagManager = TagManager.new();
    this.searchEngine = new TagSearchEngine(this.tagManager);
    this.tasks = new Map();
    this.initialized = false;
  }

  async initialize() {
    try {
      // Initialize embedding service with default configuration
      const config = TodoziEmbeddingConfig.default();
      config.similarity_threshold = 0.6;
      config.max_results = 10;
      
      this.embeddingService = await TodoziEmbeddingService.new(config);
      await this.initializeDefaultTags();
      this.initialized = true;
      
      console.log('✅ Smart Task Manager initialized successfully');
    } catch (error) {
      throw new TodoziError(
        `Failed to initialize Smart Task Manager: ${error.message}`,
        'InitializationError'
      );
    }
  }

  async initializeDefaultTags() {
    const defaultTags = [
      { name: 'development', category: 'technical', description: 'Software development tasks' },
      { name: 'bugfix', category: 'technical', description: 'Bug fixing and debugging' },
      { name: 'documentation', category: 'technical', description: 'Documentation and writing' },
      { name: 'meeting', category: 'business', description: 'Meetings and discussions' },
      { name: 'research', category: 'learning', description: 'Research and investigation' },
      { name: 'urgent', category: 'priority', description: 'High priority urgent tasks' }
    ];

    for (const tag of defaultTags) {
      await this.tagManager.createTag(tag);
      await this.embeddingService.embedTag(tag);
    }
  }

  async createTaskFromText(taskText, userId = 'user') {
    if (!this.initialized) {
      await this.initialize();
    }

    try {
      // Parse the task from todozi format if it contains tags
      let task;
      if (taskText.includes('<todozi>')) {
        const parsedTasks = parseTodoziFormat(taskText);
        task = new Task({
          userId,
          action: parsedTasks.action,
          time: parsedTasks.time || '2 hours',
          priority: parsedTasks.priority || Priority.Medium,
          parentProject: parsedTasks.parent_project || 'default',
          status: parsedTasks.status || Status.Todo,
          assignee: parsedTasks.assignee || Assignee.Human,
          tags: parsedTasks.tags || [],
          contextNotes: parsedTasks.context_notes
        });
      } else {
        // Create task from plain text with AI-suggested tags
        task = new Task({
          userId,
          action: taskText,
          time: '2 hours',
          priority: Priority.Medium,
          parentProject: 'default',
          status: Status.Todo,
          assignee: Assignee.Human,
          tags: []
        });
      }

      // Generate embedding for the task
      const taskContent = this.prepareTaskContent(task);
      const embedding = await this.embeddingService.generateEmbedding(taskContent);
      
      // Suggest tags based on semantic similarity
      const suggestedTags = await this.suggestTagsForTask(taskContent);
      task.tags = [...new Set([...task.tags, ...suggestedTags])];
      
      // Update tag usage counts
      for (const tagName of task.tags) {
        await this.tagManager.incrementTagUsage(tagName);
      }
      
      // Store task with embedding
      task.embeddingVector = embedding;
      this.tasks.set(task.id, task);
      
      // Add to embedding service index
      await this.embeddingService.addTask(task);
      
      console.log(`✅ Created task: ${task.action}`);
      console.log(`📁 Project: ${task.parentProject}`);
      console.log(`🏷️  Tags: ${task.tags.join(', ')}`);
      
      return task;
    } catch (error) {
      throw new TodoziError(
        `Failed to create task: ${error.message}`,
        'TaskCreationError'
      );
    }
  }

  async suggestTagsForTask(taskContent) {
    try {
      // Get all tags from the tag manager
      const allTags = this.tagManager.getAllTags();
      const suggestions = [];
      
      // Find semantically similar tags
      const similarTags = await this.embeddingService.findSimilarTags(taskContent, 5);
      
      for (const result of similarTags) {
        const tag = this.tagManager.getTag(result.content_id);
        if (tag && result.similarity_score > 0.7) {
          suggestions.push(tag.name);
        }
      }
      
      return suggestions.slice(0, 3); // Return top 3 suggestions
    } catch (error) {
      console.warn(`Failed to suggest tags: ${error.message}`);
      return [];
    }
  }

  async searchTasks(query, options = {}) {
    if (!this.initialized) {
      await this.initialize();
    }

    try {
      // Perform semantic search
      const searchResults = await this.embeddingService.findSimilarTasks(query, options.limit || 10);
      
      // Enrich results with task objects
      const enrichedResults = searchResults.map(result => {
        const task = this.tasks.get(result.content_id);
        if (task) {
          return {
            ...task,
            similarityScore: result.similarity_score,
            matchedContent: result.text_content
          };
        }
        return null;
      }).filter(Boolean);
      
      console.log(`🔍 Found ${enrichedResults.length} similar tasks for: "${query}"`);
      
      return enrichedResults;
    } catch (error) {
      throw new TodoziError(
        `Search failed: ${error.message}`,
        'SearchError'
      );
    }
  }

  async processChatInput(message, userId = 'user') {
    if (!this.initialized) {
      await this.initialize();
    }

    try {
      // Process the chat message to extract structured content
      const content = processChatMessageExtended(message, userId);
      
      const results = {
        tasksCreated: [],
        errors: [],
        summary: {}
      };
      
      // Process extracted tasks
      for (const taskData of content.tasks) {
        try {
          const task = new Task({
            userId,
            action: taskData.action,
            time: taskData.time || '2 hours',
            priority: taskData.priority || Priority.Medium,
            parentProject: taskData.parent_project || 'default',
            status: taskData.status || Status.Todo,
            assignee: taskData.assignee || Assignee.Human,
            tags: taskData.tags || [],
            contextNotes: taskData.context_notes
          });
          
          // Suggest additional tags based on content
          const taskContent = this.prepareTaskContent(task);
          const suggestedTags = await this.suggestTagsForTask(taskContent);
          task.tags = [...new Set([...task.tags, ...suggestedTags])];
          
          // Store and embed the task
          const embedding = await this.embeddingService.generateEmbedding(taskContent);
          task.embeddingVector = embedding;
          this.tasks.set(task.id, task);
          await this.embeddingService.addTask(task);
          
          results.tasksCreated.push(task);
          
          // Update tag usage
          for (const tagName of task.tags) {
            await this.tagManager.incrementTagUsage(tagName);
          }
          
        } catch (error) {
          results.errors.push({
            type: 'TaskCreationError',
            message: error.message,
            data: taskData
          });
        }
      }
      
      // Process any errors in the message
      for (const errorData of content.errors) {
        try {
          const errorRecord = {
            id: errorData.id,
            title: errorData.title,
            description: errorData.description,
            severity: errorData.severity,
            category: errorData.category,
            source: 'chat_input',
            tags: errorData.tags,
            resolved: false,
            createdAt: new Date().toISOString()
          };
          
          console.log(`⚠️  Error recorded: ${errorRecord.title} (${errorRecord.severity})`);
        } catch (err) {
          results.errors.push({
            type: 'ErrorProcessingError',
            message: err.message,
            data: errorData
          });
        }
      }
      
      // Generate summary
      results.summary = {
        totalProcessed: content.tasks.length + content.errors.length,
        tasksCreated: results.tasksCreated.length,
        errorsRecorded: content.errors.length,
        processingErrors: results.errors.length
      };
      
      console.log(`📊 Processing complete: ${results.summary.tasksCreated} tasks created`);
      
      return results;
    } catch (error) {
      throw new TodoziError(
        `Failed to process chat input: ${error.message}`,
        'ChatProcessingError'
      );
    }
  }

  async findRelatedTasks(taskId, limit = 5) {
    if (!this.initialized) {
      await this.initialize();
    }

    try {
      const task = this.tasks.get(taskId);
      if (!task) {
        throw new TodoziError(`Task not found: ${taskId}`, 'TaskNotFoundError');
      }
      
      // Use embedding service to find similar tasks
      const taskContent = this.prepareTaskContent(task);
      const similarTasks = await this.embeddingService.findSimilarTasks(taskContent, limit);
      
      return similarTasks.map(result => {
        const relatedTask = this.tasks.get(result.content_id);
        return relatedTask ? {
          ...relatedTask,
          similarity: result.similarity_score
        } : null;
      }).filter(Boolean);
    } catch (error) {
      throw new TodoziError(
        `Failed to find related tasks: ${error.message}`,
        'RelationError'
      );
    }
  }

  async getTaskAnalytics() {
    if (!this.initialized) {
      await this.initialize();
    }

    const analytics = {
      totalTasks: this.tasks.size,
      tagStats: this.tagManager.getTagStatistics(),
      priorityBreakdown: {},
      statusBreakdown: {},
      projectBreakdown: {},
      topTags: this.tagManager.getMostUsedTags(5)
    };
    
    // Calculate breakdowns
    for (const task of this.tasks.values()) {
      // Priority breakdown
      analytics.priorityBreakdown[task.priority] = 
        (analytics.priorityBreakdown[task.priority] || 0) + 1;
      
      // Status breakdown
      analytics.statusBreakdown[task.status] = 
        (analytics.statusBreakdown[task.status] || 0) + 1;
      
      // Project breakdown
      analytics.projectBreakdown[task.parentProject] = 
        (analytics.projectBreakdown[task.parentProject] || 0) + 1;
    }
    
    return analytics;
  }

  async clusterTasks() {
    if (!this.initialized) {
      await this.initialize();
    }

    try {
      const clusters = await this.embeddingService.clusterContent();
      
      console.log(`🔗 Found ${clusters.length} semantic clusters:`);
      
      clusters.forEach((cluster, index) => {
        console.log(`\nCluster ${index + 1} (${cluster.cluster_size} tasks, ${cluster.average_similarity.toFixed(2)} avg similarity):`);
        cluster.content_items.slice(0, 3).forEach(item => {
          const task = this.tasks.get(item.content_id);
          if (task) {
            console.log(`  • ${task.action.substring(0, 50)}... (${item.similarity_score.toFixed(2)})`);
          }
        });
      });
      
      return clusters;
    } catch (error) {
      throw new TodoziError(
        `Failed to cluster tasks: ${error.message}`,
        'ClusteringError'
      );
    }
  }

  prepareTaskContent(task) {
    let content = `Task: ${task.action}\n`;
    if (task.contextNotes) {
      content += `Description: ${task.contextNotes}\n`;
    }
    content += `Priority: ${task.priority}\n`;
    content += `Status: ${task.status}\n`;
    if (task.tags.length > 0) {
      content += `Tags: ${task.tags.join(', ')}\n`;
    }
    content += `Project: ${task.parentProject}\n`;
    return content;
  }
}

// Usage Example
async function demonstrateSmartTaskManager() {
  const manager = new SmartTaskManager();
  
  try {
    // Initialize the manager
    await manager.initialize();
    
    // Example 1: Create tasks from natural language
    console.log('\n=== Creating Tasks ===');
    await manager.createTaskFromText('Fix the authentication bug in the login service');
    await manager.createTaskFromText('Write documentation for the new API endpoints');
    await manager.createTaskFromText('Research machine learning frameworks for our project');
    
    // Example 2: Process chat input with structured content
    console.log('\n=== Processing Chat Input ===');
    const chatMessage = `
      I need to work on several things:
      <todozi>Implement user authentication;4 hours;high;backend;todo;ai</todozi>
      <todozi>Design database schema;3 hours;medium;backend;todo;collaborative</todozi>
      Also, there's an issue:
      <error>Database Connection Failed;Can't connect to PostgreSQL;high;database;production</error>
    `;
    
    const processResult = await manager.processChatInput(chatMessage);
    console.log(`Processed ${processResult.summary.tasksCreated} tasks`);
    
    // Example 3: Semantic search
    console.log('\n=== Semantic Search ===');
    const searchResults = await manager.searchTasks('database');
    searchResults.forEach(result => {
      console.log(`📋 ${result.action} (Similarity: ${(result.similarityScore * 100).toFixed(1)}%)`);
    });
    
    // Example 4: Find related tasks
    console.log('\n=== Finding Related Tasks ===');
    const tasks = Array.from(manager.tasks.values());
    if (tasks.length > 0) {
      const relatedTasks = await manager.findRelatedTasks(tasks[0].id);
      console.log(`Tasks related to "${tasks[0].action}":`);
      relatedTasks.forEach(task => {
        console.log(`  🔗 ${task.action} (${(task.similarity * 100).toFixed(1)}%)`);
      });
    }
    
    // Example 5: Analytics
    console.log('\n=== Task Analytics ===');
    const analytics = await manager.getTaskAnalytics();
    console.log(`Total tasks: ${analytics.totalTasks}`);
    console.log(`Top tags: ${analytics.topTags.map(t => t.name).join(', ')}`);
    console.log(`Status breakdown:`, analytics.statusBreakdown);
    
    // Example 6: Clustering
    console.log('\n=== Task Clustering ===');
    await manager.clusterTasks();
    
  } catch (error) {
    console.error('❌ Error in demonstration:', error.message);
    if (error.type === 'TodoziError') {
      console.error(`Error type: ${error.type}`);
    }
  }
}

// Run the demonstration
demonstrateSmartTaskManager().catch(console.error);


/*
# Example 3: Smart Task Management with Semantic Search

This example demonstrates how to use Todozi's embedding service to create a smart task management system that can semantically search tasks, suggest relevant tags, and process natural language input.

## Key Features Demonstrated:

1. **Semantic Search**: Tasks are embedded as vectors, allowing for semantic similarity search beyond simple keyword matching

2. **Smart Tag Suggestions**: The system suggests relevant tags based on semantic similarity to existing tagged content

3. **Natural Language Processing**: Processes structured chat input with todozi format tags and extracts tasks, errors, and other entities

4. **Task Clustering**: Groups similar tasks together based on semantic similarity for better organization

5. **Analytics**: Provides insights into task distribution by priority, status, project, and tags

6. **Error Handling**: Comprehensive error handling with custom error types throughout the system

7. **Embedding Integration**: Seamlessly integrates with the embedding service for intelligent task management

This example shows how the various components of the Todozi system work together to create a powerful, AI-enhanced task management solution that understands the semantic meaning of tasks and can provide intelligent suggestions and search capabilities.
*/