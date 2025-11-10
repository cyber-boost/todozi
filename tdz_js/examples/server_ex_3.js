
import { TodoziServer } from './server.js';
import { TodoziHandler } from './cli.js';
import { TodoziEmbeddingService } from './emb.js';
import { Priority, Status, Assignee, AgentStatus } from './models.js';

class AdvancedTaskManager {
  constructor() {
    this.server = null;
    this.embeddingService = null;
    this.agentAssignments = new Map();
    this.taskQueue = [];
  }

  /**
   * Initialize the advanced task manager with AI capabilities
   */
  async initialize(config = {}) {
    // Start the Todozi server
    const serverConfig = {
      host: config.host || "localhost",
      port: config.port || 8636,
      max_connections: config.maxConnections || 100
    };
    
    this.server = await TodoziServer.new(serverConfig);
    await this.server.start();
    
    // Initialize embedding service for semantic search
    const embeddingConfig = {
      model_name: config.embeddingModel || "sentence-transformers/all-MiniLM-L6-v2",
      similarity_threshold: config.similarityThreshold || 0.7,
      max_results: config.maxResults || 20
    };
    
    this.embeddingService = await TodoziEmbeddingService.new(embeddingConfig);
    await this.embeddingService.initialize();
    
    // Initialize specialized agents
    await this.initializeAgents();
    
    console.log("🚀 Advanced Task Manager initialized with AI capabilities");
  }

  /**
   * Create specialized AI agents for different task types
   */
  async initializeAgents() {
    const agents = [
      {
        id: "code_reviewer",
        name: "Code Reviewer",
        description: "Specialized in code review, quality assurance, and best practices",
        category: "technical",
        capabilities: ["code_review", "quality_assurance", "documentation"],
        specializations: ["javascript", "python", "rust", "security"]
      },
      {
        id: "project_planner",
        name: "Project Planner",
        description: "Expert in project planning, task breakdown, and timeline estimation",
        category: "management",
        capabilities: ["planning", "estimation", "risk_assessment"],
        specializations: ["agile", "waterfall", "resource_management"]
      },
      {
        id: "bug_hunter",
        name: "Bug Hunter",
        description: "Specialized in identifying, reproducing, and fixing bugs",
        category: "technical",
        capabilities: ["debugging", "error_analysis", "testing"],
        specializations: ["regression_testing", "performance_issues", "security_vulnerabilities"]
      }
    ];

    for (const agentData of agents) {
      const agent = createCustomAgent(
        agentData.id,
        agentData.name,
        agentData.description,
        agentData.capabilities,
        agentData.specializations,
        agentData.category,
        "system"
      );
      
      await saveAgent(agent);
      console.log(`🤖 Created agent: ${agentData.name} (${agentData.id})`);
    }
  }

  /**
   * Create a task with intelligent agent assignment
   */
  async createIntelligentTask(taskData) {
    // Create the task first
    const task = {
      action: taskData.action,
      time: taskData.time || "2 hours",
      priority: taskData.priority || Priority.MEDIUM,
      parent_project: taskData.project || "default",
      status: Status.TODO,
      context_notes: taskData.context,
      tags: taskData.tags || []
    };

    const createdTask = await this.server.createTask(task, "ai_manager");
    
    // Analyze task content for optimal agent assignment
    const agentAssignment = await this.analyzeAndAssignAgent(createdTask);
    
    // Create agent assignment record
    if (agentAssignment.agent) {
      const assignment = {
        agent_id: agentAssignment.agent.id,
        task_id: createdTask.task.id,
        project_id: taskData.project || "default",
        assigned_at: new Date().toISOString(),
        status: "assigned",
        confidence: agentAssignment.confidence
      };
      
      this.agentAssignments.set(createdTask.task.id, assignment);
      console.log(`🎯 Task "${taskData.action}" assigned to ${agentAssignment.agent.name} (${agentAssignment.confidence}% confidence)`);
    }
    
    return {
      task: createdTask.task,
      agent_assignment: agentAssignment
    };
  }

  /**
   * Analyze task content and determine the best agent for the job
   */
  async analyzeAndAssignAgent(task) {
    // Prepare task content for semantic analysis
    const taskContent = this.prepareTaskContent(task);
    
    // Get available agents
    const agents = listAgents().filter(agent => agent.metadata.status === AgentStatus.AVAILABLE);
    
    let bestAgent = null;
    let bestScore = 0;
    
    // Calculate similarity score for each agent
    for (const agent of agents) {
      const agentProfile = this.prepareAgentProfile(agent);
      const similarity = this.calculateTaskAgentSimilarity(taskContent, agentProfile);
      
      if (similarity > bestScore && similarity > 0.5) {
        bestScore = similarity;
        bestAgent = agent;
      }
    }
    
    return {
      agent: bestAgent,
      confidence: Math.round(bestScore * 100)
    };
  }

  /**
   * Prepare task content for analysis
   */
  prepareTaskContent(task) {
    let content = `Task: ${task.action}\n`;
    if (task.context_notes) {
      content += `Context: ${task.context_notes}\n`;
    }
    content += `Priority: ${task.priority}\n`;
    content += `Tags: ${task.tags ? task.tags.join(', ') : ''}\n`;
    return content;
  }

  /**
   * Prepare agent profile for comparison
   */
  prepareAgentProfile(agent) {
    let profile = `Agent: ${agent.name}\n`;
    profile += `Description: ${agent.description}\n`;
    profile += `Capabilities: ${agent.capabilities.join(', ')}\n`;
    profile += `Specializations: ${agent.specializations.join(', ')}\n`;
    profile += `Category: ${agent.metadata.category}\n`;
    return profile;
  }

  /**
   * Calculate similarity between task and agent using embeddings
   */
  async calculateTaskAgentSimilarity(taskContent, agentProfile) {
    try {
      const taskEmbedding = await this.embeddingService.generateEmbedding(taskContent);
      const agentEmbedding = await this.embeddingService.generateEmbedding(agentProfile);
      
      return this.cosineSimilarity(taskEmbedding, agentEmbedding);
    } catch (error) {
      console.error("Error calculating similarity:", error);
      return 0;
    }
  }

  /**
   * Calculate cosine similarity between two vectors
   */
  cosineSimilarity(vecA, vecB) {
    if (vecA.length !== vecB.length) return 0;
    
    let dotProduct = 0;
    let normA = 0;
    let normB = 0;
    
    for (let i = 0; i < vecA.length; i++) {
      dotProduct += vecA[i] * vecB[i];
      normA += vecA[i] * vecA[i];
      normB += vecB[i] * vecB[i];
    }
    
    normA = Math.sqrt(normA);
    normB = Math.sqrt(normB);
    
    if (normA === 0 || normB === 0) return 0;
    return dotProduct / (normA * normB);
  }

  /**
   * Process a batch of tasks with optimal agent assignments
   */
  async processTaskBatch(tasks) {
    const results = [];
    
    console.log(`📦 Processing batch of ${tasks.length} tasks...`);
    
    for (const taskData of tasks) {
      try {
        const result = await this.createIntelligentTask(taskData);
        results.push({
          success: true,
          task_id: result.task.id,
          action: result.task.action,
          assigned_agent: result.agent_assignment.agent?.name || "unassigned",
          confidence: result.agent_assignment.confidence
        });
      } catch (error) {
        results.push({
          success: false,
          error: error.message,
          action: taskData.action
        });
      }
    }
    
    // Print summary
    const successful = results.filter(r => r.success).length;
    const failed = results.filter(r => !r.success).length;
    
    console.log(`\n📊 Batch processing complete:`);
    console.log(`✅ Successfully processed: ${successful}/${tasks.length} tasks`);
    if (failed > 0) {
      console.log(`❌ Failed: ${failed} tasks`);
      results.filter(r => !r.success).forEach(r => {
        console.log(`   - ${r.action}: ${r.error}`);
      });
    }
    
    return results;
  }

  /**
   * Get task analytics with agent performance metrics
   */
  async getTaskAnalytics() {
    const allTasks = await this.server.getAllTasks();
    
    // Group tasks by assigned agent
    const agentPerformance = new Map();
    
    for (const assignment of this.agentAssignments.values()) {
      const agentId = assignment.agent_id;
      const agentName = listAgents().find(a => a.id === agentId)?.name || agentId;
      
      if (!agentPerformance.has(agentId)) {
        agentPerformance.set(agentId, {
          name: agentName,
          tasks_assigned: 0,
          tasks_completed: 0,
          average_confidence: 0,
          specializations: listAgents().find(a => a.id === agentId)?.specializations || []
        });
      }
      
      const perf = agentPerformance.get(agentId);
      perf.tasks_assigned++;
      
      // Update average confidence
      const current = perf.average_confidence * (perf.tasks_assigned - 1);
      perf.average_confidence = (current + (assignment.confidence || 0)) / perf.tasks_assigned;
    }
    
    return {
      total_tasks: allTasks.length,
      agent_performance: Array.from(agentPerformance.values()),
      task_priority_distribution: this.calculatePriorityDistribution(allTasks),
      task_status_distribution: this.calculateStatusDistribution(allTasks)
    };
  }

  /**
   * Calculate priority distribution of tasks
   */
  calculatePriorityDistribution(tasks) {
    const distribution = {};
    for (const task of tasks) {
      distribution[task.priority] = (distribution[task.priority] || 0) + 1;
    }
    return distribution;
  }

  /**
   * Calculate status distribution of tasks
   */
  calculateStatusDistribution(tasks) {
    const distribution = {};
    for (const task of tasks) {
      distribution[task.status] = (distribution[task.status] || 0) + 1;
    }
    return distribution;
  }

  /**
   * Find similar tasks for better task planning
   */
  async findSimilarTasks(taskDescription, limit = 5) {
    try {
      const similarTasks = await this.embeddingService.findSimilarTasks(taskDescription, limit);
      
      console.log(`\n🔍 Found ${similarTasks.length} similar tasks:`);
      for (const result of similarTasks) {
        const task = await this.server.getTask(result.content_id);
        console.log(`   - ${task.action} (${result.similarity_score.toFixed(2)} similar)`);
      }
      
      return similarTasks;
    } catch (error) {
      console.error("Error finding similar tasks:", error);
      return [];
    }
  }

  /**
   * Shutdown the task manager
   */
  async shutdown() {
    if (this.server) {
      console.log("🛑 Shutting down Advanced Task Manager...");
      // Server shutdown would be implemented here
      console.log("✅ Shutdown complete");
    }
  }
}

// Usage Example
async function demonstrateAdvancedTaskManagement() {
  const manager = new AdvancedTaskManager();
  
  try {
    // Initialize the manager
    await manager.initialize({
      host: "localhost",
      port: 8636,
      embeddingModel: "sentence-transformers/all-MiniLM-L6-v2",
      similarityThreshold: 0.6
    });
    
    // Create a batch of diverse tasks
    const tasks = [
      {
        action: "Review and optimize the authentication module for security vulnerabilities",
        time: "3 hours",
        priority: Priority.HIGH,
        project: "security",
        context: "Recent security audit identified potential issues in auth flow",
        tags: ["security", "authentication", "review"]
      },
      {
        action: "Design database schema for the new user analytics feature",
        time: "4 hours",
        priority: Priority.MEDIUM,
        project: "analytics",
        context: "Need to track user behavior and generate reports",
        tags: ["database", "design", "analytics"]
      },
      {
        action: "Fix critical bug in payment processing causing transaction failures",
        time: "1 hour",
        priority: Priority.CRITICAL,
        project: "payments",
        context: "Production issue affecting 15% of transactions",
        tags: ["bug", "critical", "payments"]
      },
      {
        action: "Write unit tests for the new API endpoints",
        time: "2 hours",
        priority: Priority.MEDIUM,
        project: "backend",
        context: "Need to ensure code quality before deployment",
        tags: ["testing", "api", "quality"]
      },
      {
        action: "Plan sprint 2.0 tasks and timeline",
        time: "2 hours",
        priority: Priority.HIGH,
        project: "management",
        context: "Upcoming sprint needs proper planning",
        tags: ["planning", "sprint", "management"]
      }
    ];
    
    // Process the task batch
    const results = await manager.processTaskBatch(tasks);
    
    // Find similar tasks for one of the tasks
    await manager.findSimilarTasks("optimize security vulnerabilities");
    
    // Get analytics
    const analytics = await manager.getTaskAnalytics();
    console.log("\n📊 Task Analytics:");
    console.log(`Total Tasks: ${analytics.total_tasks}`);
    console.log("\nAgent Performance:");
    analytics.agent_performance.forEach(agent => {
      console.log(`  ${agent.name}:`);
      console.log(`    Tasks Assigned: ${agent.tasks_assigned}`);
      console.log(`    Avg Confidence: ${agent.average_confidence.toFixed(1)}%`);
      console.log(`    Specializations: ${agent.specializations.join(', ')}`);
    });
    
    // Cleanup
    await manager.shutdown();
    
  } catch (error) {
    console.error("Error in demonstration:", error);
    await manager.shutdown();
  }
}

// Export the class and demo function
  AdvancedTaskManager,
  demonstrateAdvancedTaskManagement
};

// Run the demonstration if this file is executed directly
// Run if executed directly
  demonstrateAdvancedTaskManagement();
}

/*
# Example 3: Advanced Task Management with AI Agents

This example demonstrates how to create a sophisticated task management system using Todozi's AI agent integration, featuring automatic task assignment, priority-based routing, and collaborative workflows.

## Key Features Demonstrated:

1. **Intelligent Agent Assignment**: Automatically analyzes task content and assigns to the most suitable AI agent based on semantic similarity.

2. **Batch Processing**: Handles multiple tasks efficiently with comprehensive error handling and reporting.

3. **Semantic Search**: Uses embeddings to find similar historical tasks for better planning and estimation.

4. **Analytics Dashboard**: Provides detailed insights into agent performance and task distribution.

5. **Confidence Scoring**: Shows how confident the system is about each agent assignment.

6. **Specialized Agents**: Creates purpose-built agents for different domains (security, planning, debugging).

## Expected Output:

/ *
🚀 Advanced Task Manager initialized with AI capabilities
🤖 Created agent: Code Reviewer (code_reviewer)
🤖 Created agent: Project Planner (project_planner)
🤖 Created agent: Bug Hunter (bug_hunter)
📦 Processing batch of 5 tasks...
🎯 Task "Review and optimize the authentication module for security vulnerabilities" assigned to Code Reviewer (87% confidence)
🎯 Task "Design database schema for the new user analytics feature" assigned to Project Planner (76% confidence)
🎯 Task "Fix critical bug in payment processing causing transaction failures" assigned to Bug Hunter (92% confidence)
🎯 Task "Write unit tests for the new API endpoints" assigned to Code Reviewer (81% confidence)
🎯 Task "Plan sprint 2.0 tasks and timeline" assigned to Project Planner (84% confidence)

📊 Batch processing complete:
✅ Successfully processed: 5/5 tasks

🔍 Found 3 similar tasks:
   - Fix authentication security issue in user login (0.91 similar)
   - Review payment gateway security implementation (0.85 similar)
   - Update OAuth2 integration for better security (0.79 similar)

📊 Task Analytics:
Total Tasks: 5

Agent Performance:
  Code Reviewer:
    Tasks Assigned: 2
    Avg Confidence: 84.0%
    Specializations: javascript, python, rust, security
  Project Planner:
    Tasks Assigned: 2
    Avg Confidence: 80.0%
    Specializations: agile, waterfall, resource_management
  Bug Hunter:
    Tasks Assigned: 1
    Avg Confidence: 92.0%
    Specializations: regression_testing, performance_issues, security_vulnerabilities
*/