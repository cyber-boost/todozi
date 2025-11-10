// example3.js
import { Task, Priority, Status, Assignee, Agent, Project, QueueItem, QueueStatus } from '../todozi/models.js';

import { Storage } from '../todozi/storage.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { processChatMessageExtended } from '../todozi/todozi.js';

async function main() {
  console.log('🚀 Initializing Todozi Task Management System...\n');

  // Initialize storage and embedding service
  const storage = await Storage.new();
  const embConfig = TodoziEmbeddingConfig.default();
  const embeddingService = await TodoziEmbeddingService.new(embConfig);

  // Create a new project
  const project = new Project('webapp', 'E-commerce Web Application');
  await storage.saveProject(project);
  console.log(`📁 Created project: ${project.name}`);

  // Create specialized agents for the project
  const coderAgent = Agent.create_coder();
  coderAgent.id = 'webapp-coder';
  coderAgent.name = 'WebApp Coder';
  coderAgent.description = 'Specialized in React and Node.js development';
  await storage.saveAgent(coderAgent);

  const testerAgent = Agent.new('webapp-tester', 'QA Tester', 'Web application testing specialist');
  testerAgent.systemPrompt = "You are an expert QA tester focused on web applications. Test for functionality, performance, and security.";
  await storage.saveAgent(testerAgent);

  console.log(`🤖 Created agents: ${coderAgent.name}, ${testerAgent.name}\n`);

  // Example 1: Adding tasks through direct API
  console.log('--- Adding Tasks Directly ---');
  
  const tasks = [
    Task.new('dev-user', 'Implement user authentication system', '3 days', Priority.High, 'webapp', Status.InProgress),
    Task.new('dev-user', 'Design database schema for products', '1 day', Priority.Medium, 'webapp', Status.Todo),
    Task.new('dev-user', 'Create responsive homepage layout', '2 days', Priority.Critical, 'webapp', Status.Todo),
    Task.new('dev-user', 'Set up CI/CD pipeline', '4 hours', Priority.Low, 'webapp', Status.Blocked),
  ];

  for (const task of tasks) {
    await storage.addTaskToProject(task);
    console.log(`✓ Added task: ${task.action} (${task.priority} priority)`);
  }

  // Example 2: Processing tasks from chat message
  console.log('\n--- Processing Tasks from Chat Message ---');
  
  const chatMessage = `
    I need to complete these tasks for the webapp:
    <todozi>Add payment gateway integration;5 hours;high;webapp;todo;agent:webapp-coder</todozi>
    <todozi>Write API documentation;3 hours;medium;webapp;todo;human</todozi>
    <todozi>Optimize database queries;1 day;critical;webapp;in_progress;collaborative</todozi>
  `;

  const content = processChatMessageExtended(chatMessage, 'project-manager');
  
  for (const task of content.tasks) {
    await storage.addTaskToProject(task);
    console.log(`✓ Processed task: ${task.action} (assigned to ${JSON.stringify(task.assignee)})`);
  }

  // Example 3: Semantic search for similar tasks
  console.log('\n--- Semantic Search Examples ---');
  
  // Search for authentication-related tasks
  const authTasks = await embeddingService.findSimilarTasks('user login security', 5);
  console.log('🔍 Tasks similar to "user login security":');
  authTasks.forEach((result, i) => {
    console.log(`  ${i + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
  });

  // Search for UI-related tasks
  const uiTasks = await embeddingService.findSimilarTasks('interface design responsive', 5);
  console.log('\n🔍 Tasks similar to "interface design responsive":');
  uiTasks.forEach((result, i) => {
    console.log(`  ${i + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
  });

  // Example 4: Queue management
  console.log('\n--- Managing Task Queue ---');
  
  const queueItems = [
    QueueItem.new({
      taskName: 'Deploy to production',
      taskDescription: 'Deploy latest changes to production server',
      priority: 'high',
      projectId: 'webapp'
    }),
    QueueItem.new({
      taskName: 'Fix bug in checkout flow',
      taskDescription: 'Customers report checkout failures',
      priority: 'critical',
      projectId: 'webapp'
    })
  ];

  const queueCollection = QueueCollection.new();
  for (const item of queueItems) {
    queueCollection.addItem(item);
    console.log(`✓ Queued: ${item.taskName} (${item.priority})`);
  }

  // Start a work session
  const sessionId = queueCollection.startSession(queueCollection.getAllItems()[0].id);
  console.log(`🚀 Started session: ${sessionId}`);

  // Example 5: Task clustering and insights
  console.log('\n--- Task Clustering and Insights ---');
  
  const clusters = await embeddingService.clusterContent();
  console.log(`📊 Found ${clusters.length} task clusters:`);
  clusters.forEach((cluster, i) => {
    console.log(`  Cluster ${i + 1}: ${cluster.cluster_size} tasks (avg similarity: ${(cluster.average_similarity * 100).toFixed(1)}%)`);
  });

  // Example 6: Listing and filtering tasks
  console.log('\n--- Task Filtering and Listing ---');
  
  // Get all tasks for the webapp project
  const projectTasks = await storage.listTasksAcrossProjects({ project: 'webapp' });
  console.log(`📋 Total tasks in webapp project: ${projectTasks.length}`);

  // Get high priority tasks
  const highPriorityTasks = await storage.listTasksAcrossProjects({ 
    project: 'webapp', 
    priority: 'high' 
  });
  console.log(`🔥 High priority tasks: ${highPriorityTasks.length}`);

  // Get in-progress tasks
  const inProgressTasks = await storage.listTasksAcrossProjects({ 
    project: 'webapp', 
    status: 'in_progress' 
  });
  console.log(`🔄 In-progress tasks: ${inProgressTasks.length}`);

  // Example 7: Agent assignment workflow
  console.log('\n--- Agent Assignment Workflow ---');
  
  // Find tasks suitable for the coder agent
  const codingTasks = projectTasks.filter(task => 
    task.action.toLowerCase().includes('implement') || 
    task.action.toLowerCase().includes('create') ||
    task.action.toLowerCase().includes('set up')
  );

  console.log(`💻 Found ${codingTasks.length} tasks suitable for coder agent:`);
  codingTasks.forEach(task => {
    console.log(`  - ${task.action}`);
  });

  // Example 8: Task completion workflow
  console.log('\n--- Completing Tasks ---');
  
  // Complete a task and log the completion
  const taskToComplete = projectTasks.find(t => t.action.includes('database schema'));
  if (taskToComplete) {
    await storage.completeTaskInProject(taskToComplete.id);
    console.log(`✅ Completed task: ${taskToComplete.action}`);
    
    // Generate embedding for completion message
    const completionText = `Completed: ${taskToComplete.action}. Database schema designed with MongoDB collections for users, products, and orders.`;
    await embeddingService.generateEmbedding(completionText);
  }

  // Example 9: Statistics and reporting
  console.log('\n--- Project Statistics ---');
  
  const stats = {
    total: projectTasks.length,
    completed: projectTasks.filter(t => t.status === 'done').length,
    inProgress: projectTasks.filter(t => t.status === 'in_progress').length,
    blocked: projectTasks.filter(t => t.status === 'blocked').length,
    todo: projectTasks.filter(t => t.status === 'todo').length
  };

  console.log(`📊 Project Statistics for ${project.name}:`);
  console.log(`  Total tasks: ${stats.total}`);
  console.log(`  Completed: ${stats.completed} (${((stats.completed / stats.total) * 100).toFixed(1)}%)`);
  console.log(`  In Progress: ${stats.inProgress}`);
  console.log(`  Blocked: ${stats.blocked}`);
  console.log(`  To Do: ${stats.todo}`);

  // Example 10: Hybrid search (semantic + keyword)
  console.log('\n--- Hybrid Search Example ---');
  
  const hybridResults = await embeddingService.hybridSearch(
    'user interface design',
    ['responsive', 'mobile', 'layout'],
    ['Task'],
    0.7, // semantic weight
    3
  );
  
  console.log('🔍 Hybrid search results for "user interface design" with keywords:');
  hybridResults.forEach((result, i) => {
    console.log(`  ${i + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% match)`);
  });

  console.log('\n🎉 Todozi example completed successfully!');
}

// Error handling helper
function handleError(error) {
  console.error('❌ Error:', error.message);
  if (error.type) {
    console.error(`   Type: ${error.type}`);
  }
  process.exit(1);
}

// Run the example
// Run if executed directly
  main().catch(handleError);
}


/*
## Example 3: Comprehensive Todozi Task Management with Semantic Search

This example demonstrates a complete workflow for managing a software development project using Todozi, including task creation, semantic search, and agent assignments.

### Key Features Demonstrated:

1. **Project Setup**: Creating and managing projects with the Storage API
2. **Task Creation**: Adding tasks directly and processing from chat messages
3. **Semantic Search**: Finding similar tasks using embedding vectors
4. **Queue Management**: Organizing work sessions and task queues
5. **Task Clustering**: Automatically grouping similar tasks
6. **Advanced Filtering**: Listing tasks by various criteria (priority, status, project)
7. **Agent Integration**: Assigning tasks to specialized AI agents
8. **Task Completion**: Workflow for marking tasks as done
9. **Statistics**: Generating project progress reports
10. **Hybrid Search**: Combining semantic similarity with keyword matching

### To Run This Example:

/ *
bash
# Install dependencies (if not already installed)
npm install

# Run the example
node example3.js
*/