import { processChatMessageExtended, processWorkflow, executeTask, Status, Priority, Assignee } from '../todozi/todozi.js';

import { SummaryManager } from '../todozi/summary.js';
import { TodoziEmbeddingService } from '../todozi/emb.js';
import { Storage } from '../todozi/storage.js';

class ChatProcessor {
  constructor() {
    this.summaryManager = new SummaryManager();
    this.embeddingService = null;
    this.storage = null;
  }

  async initialize() {
    // Initialize storage
    this.storage = await Storage.new();
    
    // Initialize embedding service for semantic search
    const embeddingConfig = { modelName: "sentence-transformers/all-MiniLM-L6-v2" };
    this.embeddingService = await TodoziEmbeddingService.new(embeddingConfig);
    await this.embeddingService.initialize();
    
    console.log("✅ ChatProcessor initialized successfully");
  }

  async processComplexMessage(message, userId = "user_001") {
    console.log(`📨 Processing message from user: ${userId}`);
    console.log(`💬 Message length: ${message.length} characters`);
    
    // Transform shorthand tags to full format
    const transformedMessage = this.transformShorthandTags(message);
    console.log("🔄 Shorthand tags transformed");
    
    // Parse all content types from the message
    const content = processChatMessageExtended(transformedMessage, userId);
    
    // Display summary of parsed content
    this.displayContentSummary(content);
    
    // Process and save each content type
    const results = await this.saveContent(content, userId);
    
    // Generate embeddings for semantic search
    await this.generateEmbeddings(content);
    
    // Execute any tasks that were created
    const taskResults = await this.executeTasks(content.tasks);
    
    return {
      summary: this.generateProcessingSummary(content, results, taskResults),
      details: results,
      taskExecutions: taskResults
    };
  }

  transformShorthandTags(message) {
    let transformed = message;
    const mappings = [
      ["<tz>", "<todozi>"], ["</tz>", "</todozi>"],
      ["<mm>", "<memory>"], ["</mm>", "</memory>"],
      ["<id>", "<idea>"], ["</id>", "</idea>"],
      ["<er>", "<error>"], ["</er>", "</error>"],
      ["<tn>", "<train>"], ["</tn>", "</train>"]
    ];
    
    for (const [shorthand, longhand] of mappings) {
      transformed = transformed.replace(
        new RegExp(shorthand.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g'), 
        longhand
      );
    }
    return transformed;
  }

  displayContentSummary(content) {
    console.log("\n📊 Content Summary:");
    console.log("═══════════════════════════════════════");
    console.log(`📋 Tasks: ${content.tasks.length}`);
    console.log(`🧠 Memories: ${content.memories.length}`);
    console.log(`💡 Ideas: ${content.ideas.length}`);
    console.log(`🤖 Agent Assignments: ${content.agent_assignments.length}`);
    console.log(`🧩 Code Chunks: ${content.code_chunks.length}`);
    console.log(`❌ Errors: ${content.errors.length}`);
    console.log(`🎓 Training Data: ${content.training_data.length}`);
    console.log(`😊 Feelings: ${content.feelings.length}`);
    console.log(`📝 Summaries: ${content.summaries.length}`);
  }

  async saveContent(content, userId) {
    const results = {
      tasks: [],
      memories: [],
      ideas: [],
      errors: [],
      trainingData: [],
      feelings: [],
      summaries: []
    };

    // Save tasks
    for (const task of content.tasks) {
      try {
        await this.storage.addTaskToProject(task);
        results.tasks.push({
          id: task.id,
          action: task.action,
          status: "saved",
          project: task.parent_project
        });
        console.log(`✅ Task saved: ${task.action} (${task.id})`);
      } catch (e) {
        console.error(`❌ Failed to save task: ${e.message}`);
        results.tasks.push({
          id: task.id,
          action: task.action,
          status: "failed",
          error: e.message
        });
      }
    }

    // Save memories
    for (const memory of content.memories) {
      try {
        await this.storage.saveMemory(memory);
        results.memories.push({
          id: memory.id,
          moment: memory.moment.substring(0, 50) + "...",
          status: "saved",
          type: memory.memory_type
        });
        console.log(`✅ Memory saved: ${memory.moment} (${memory.id})`);
      } catch (e) {
        console.error(`❌ Failed to save memory: ${e.message}`);
        results.memories.push({
          id: memory.id,
          moment: memory.moment.substring(0, 50) + "...",
          status: "failed",
          error: e.message
        });
      }
    }

    // Save ideas
    for (const idea of content.ideas) {
      try {
        await this.storage.saveIdea(idea);
        results.ideas.push({
          id: idea.id,
          idea: idea.idea.substring(0, 50) + "...",
          status: "saved",
          importance: idea.importance
        });
        console.log(`✅ Idea saved: ${idea.idea} (${idea.id})`);
      } catch (e) {
        console.error(`❌ Failed to save idea: ${e.message}`);
        results.ideas.push({
          id: idea.id,
          idea: idea.idea.substring(0, 50) + "...",
          status: "failed",
          error: e.message
        });
      }
    }

    // Save errors
    for (const error of content.errors) {
      try {
        await this.storage.saveError(error);
        results.errors.push({
          id: error.id,
          title: error.title,
          status: "saved",
          severity: error.severity
        });
        console.log(`✅ Error saved: ${error.title} (${error.id})`);
      } catch (e) {
        console.error(`❌ Failed to save error: ${e.message}`);
        results.errors.push({
          id: error.id,
          title: error.title,
          status: "failed",
          error: e.message
        });
      }
    }

    // Save training data
    for (const training of content.training_data) {
      try {
        await this.storage.saveTrainingData(training);
        results.trainingData.push({
          id: training.id,
          prompt: training.prompt.substring(0, 50) + "...",
          status: "saved",
          dataType: training.data_type
        });
        console.log(`✅ Training data saved: ${training.data_type} (${training.id})`);
      } catch (e) {
        console.error(`❌ Failed to save training data: ${e.message}`);
        results.trainingData.push({
          id: training.id,
          prompt: training.prompt.substring(0, 50) + "...",
          status: "failed",
          error: e.message
        });
      }
    }

    // Save feelings
    for (const feeling of content.feelings) {
      try {
        await this.storage.saveFeeling(feeling);
        results.feelings.push({
          id: feeling.id,
          emotion: feeling.emotion,
          intensity: feeling.intensity,
          status: "saved"
        });
        console.log(`✅ Feeling saved: ${feeling.emotion} (${feeling.id})`);
      } catch (e) {
        console.error(`❌ Failed to save feeling: ${e.message}`);
        results.feelings.push({
          id: feeling.id,
          emotion: feeling.emotion,
          status: "failed",
          error: e.message
        });
      }
    }

    // Save summaries
    for (const summary of content.summaries) {
      try {
        const summaryId = await this.summaryManager.createSummary({
          content: summary.content,
          priority: summary.priority,
          context: summary.context,
          tags: summary.tags
        });
        results.summaries.push({
          id: summaryId,
          content: summary.content.substring(0, 50) + "...",
          status: "saved",
          priority: summary.priority
        });
        console.log(`✅ Summary saved: ${summary.content.substring(0, 50)}... (${summaryId})`);
      } catch (e) {
        console.error(`❌ Failed to save summary: ${e.message}`);
        results.summaries.push({
          content: summary.content.substring(0, 50) + "...",
          status: "failed",
          error: e.message
        });
      }
    }

    return results;
  }

  async generateEmbeddings(content) {
    console.log("\n🧠 Generating embeddings for semantic search...");
    
    // Generate embeddings for tasks
    for (const task of content.tasks) {
      try {
        const taskContent = this.prepareTaskContent(task);
        const embedding = await this.embeddingService.generateEmbedding(taskContent);
        task.embedding_vector = embedding;
        
        // Update task with embedding
        await this.storage.updateTaskInProject(task.id, {
          embedding_vector: embedding
        });
      } catch (e) {
        console.error(`Failed to generate embedding for task ${task.id}: ${e.message}`);
      }
    }
    
    // Generate embeddings for memories
    for (const memory of content.memories) {
      try {
        const memoryContent = `Memory: ${memory.moment}\nMeaning: ${memory.meaning}\nReason: ${memory.reason}`;
        await this.embeddingService.embedMemory(memory);
      } catch (e) {
        console.error(`Failed to generate embedding for memory ${memory.id}: ${e.message}`);
      }
    }
    
    // Generate embeddings for ideas
    for (const idea of content.ideas) {
      try {
        await this.embeddingService.embedIdea(idea);
      } catch (e) {
        console.error(`Failed to generate embedding for idea ${idea.id}: ${e.message}`);
      }
    }
    
    console.log("✅ Embeddings generated successfully");
  }

  prepareTaskContent(task) {
    let content = `Task: ${task.action}\n`;
    if (task.context_notes) {
      content += `Description: ${task.context_notes}\n`;
    }
    content += `Priority: ${task.priority}\n`;
    content += `Status: ${task.status}\n`;
    if (task.tags && task.tags.length > 0) {
      content += `Tags: ${task.tags.join(', ')}\n`;
    }
    if (task.assignee) {
      content += `Assignee: ${JSON.stringify(task.assignee)}\n`;
    }
    return content;
  }

  async executeTasks(tasks) {
    console.log("\n⚡ Executing tasks...");
    const results = [];
    
    for (const task of tasks) {
      try {
        const result = await executeTask(this.storage, task);
        results.push({
          taskId: task.id,
          action: task.action,
          result: result,
          status: "executed"
        });
        console.log(`✅ Task executed: ${task.action} -> ${result}`);
      } catch (e) {
        console.error(`❌ Task execution failed: ${task.action} -> ${e.message}`);
        results.push({
          taskId: task.id,
          action: task.action,
          error: e.message,
          status: "failed"
        });
      }
    }
    
    return results;
  }

  generateProcessingSummary(content, results, taskExecutions) {
    const totalItems = Object.values(content).reduce((sum, arr) => sum + arr.length, 0);
    const savedItems = Object.values(results).reduce((sum, arr) => 
      sum + arr.filter(item => item.status === "saved").length, 0);
    const failedItems = totalItems - savedItems;
    
    const summary = {
      totalProcessed: totalItems,
      successfullySaved: savedItems,
      failed: failedItems,
      successRate: totalItems > 0 ? (savedItems / totalItems * 100).toFixed(2) + "%" : "0%",
      breakdown: {
        tasks: { total: content.tasks.length, saved: results.tasks.filter(t => t.status === "saved").length },
        memories: { total: content.memories.length, saved: results.memories.filter(m => m.status === "saved").length },
        ideas: { total: content.ideas.length, saved: results.ideas.filter(i => i.status === "saved").length },
        errors: { total: content.errors.length, saved: results.errors.filter(e => e.status === "saved").length },
        trainingData: { total: content.training_data.length, saved: results.trainingData.filter(t => t.status === "saved").length },
        feelings: { total: content.feelings.length, saved: results.feelings.filter(f => f.status === "saved").length },
        summaries: { total: content.summaries.length, saved: results.summaries.filter(s => s.status === "saved").length }
      },
      tasksExecuted: taskExecutions.filter(t => t.status === "executed").length,
      tasksFailed: taskExecutions.filter(t => t.status === "failed").length
    };
    
    return summary;
  }
}

// Example usage
async function demonstrateChatProcessing() {
  console.log("🚀 Starting Chat Processing Example");
  console.log("=====================================\n");
  
  const processor = new ChatProcessor();
  await processor.initialize();
  
  // Example complex message with multiple content types
  const complexMessage = `
Working on the new authentication system today. Need to:

<tz>Implement JWT authentication middleware; 4 hours; high; auth-project; in_progress; ai; security,jwt,middleware</tz>

<mm>Authentication is critical for security; JWT provides secure token-based authentication; Must implement today; high; short; security</mm>

<id>Create a microservice architecture for auth; share; high; architecture,microservice,auth</id>

<er>JWT token validation failing; Getting 401 errors even with valid tokens; high; security; auth-service; jwt,validation,401</er>

<tn>instruction;How to implement JWT middleware in Node.js; import jwt from 'jsonwebtoken.js'; function authMiddleware(req, res, next) { const token = req.header('Authorization')?.replace('Bearer ', ''); if (!token) return res.status(401).json({ error: 'Access denied' }); try { const decoded = jwt.verify(token, process.env.JWT_SECRET); req.user = decoded; next(); } catch (error) { res.status(401).json({ error: 'Invalid token' }); } }; code</tn>

<feel>confident; 7; Making good progress on authentication; development</feel>

<summary>JWT authentication implementation; high; Auth system security</summary>
  `;
  
  console.log("📨 Processing complex message with multiple content types...\n");
  
  const result = await processor.processComplexMessage(complexMessage, "developer_001");
  
  console.log("\n📊 Processing Results:");
  console.log("======================");
  console.log(`Total items processed: ${result.summary.totalProcessed}`);
  console.log(`Successfully saved: ${result.summary.successfullySaved}`);
  console.log(`Failed: ${result.summary.failed}`);
  console.log(`Success rate: ${result.summary.successRate}`);
  console.log(`Tasks executed: ${result.summary.tasksExecuted}`);
  console.log(`Tasks failed: ${result.summary.tasksFailed}`);
  
  console.log("\n📈 Breakdown by type:");
  Object.entries(result.summary.breakdown).forEach(([type, stats]) => {
    if (stats.total > 0) {
      console.log(`  ${type}: ${stats.saved}/${stats.total} saved`);
    }
  });
  
  // Demonstrate semantic search
  console.log("\n🔍 Testing semantic search...");
  try {
    const similarTasks = await processor.embeddingService.findSimilarTasks("authentication security", 3);
    if (similarTasks.length > 0) {
      console.log(`Found ${similarTasks.length} similar tasks:`);
      similarTasks.forEach((result, index) => {
        console.log(`  ${index + 1}. Similarity: ${(result.similarity_score * 100).toFixed(1)}% - ${result.content_id}`);
      });
    }
  } catch (e) {
    console.log(`Semantic search test: ${e.message}`);
  }
  
  console.log("\n✅ Chat processing example completed!");
}

// Run the example
// Run if executed directly
  demonstrateChatProcessing().catch(console.error);
}


/*
# Example 3: Processing Complex Chat Messages with Multiple Content Types

This example demonstrates how to process a chat message containing various content types (tasks, memories, ideas, errors, etc.) and save them to storage using the Todozi system.

## Key Features Demonstrated:

1. **Multi-format Processing**: Handles tasks, memories, ideas, errors, training data, feelings, and summaries in a single message

2. **Shorthand Tag Transformation**: Converts short tags like `<tz>` to full format `<todozi>`

3. **Comprehensive Storage**: Saves all content types to appropriate storage mechanisms

4. **Semantic Search Integration**: Generates embeddings for all content to enable intelligent search

5. **Task Execution**: Automatically executes parsed tasks based on their assignee (AI, human, collaborative, or agent)

6. **Error Handling**: Gracefully handles failures and provides detailed feedback

7. **Progress Tracking**: Shows detailed statistics about what was processed, saved, and executed

8. **Embedding Generation**: Creates semantic embeddings for tasks, memories, and ideas for enhanced search capabilities

This example shows how the Todozi system can handle complex, multi-type content from natural language input, parse it intelligently, store it appropriately, and even execute tasks automatically based on their configuration.
*/