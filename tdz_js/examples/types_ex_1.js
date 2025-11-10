// Example: Creating and Searching Tasks in Todozi System
import { SearchEngine } from './types.js';

class TodoziTaskManager {
  constructor() {
    this.searchEngine = new SearchEngine();
    this.currentContent = {
      tasks: [],
      memories: [],
      ideas: [],
      agentAssignments: [],
      codeChunks: [],
      errors: [],
      trainingData: [],
      feelings: []
    };
  }

  /**
   * Create a new task with comprehensive metadata
   * @param {Object} taskData - Task creation parameters
   */
  async createTask(taskData) {
    const task = {
      id: `task_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      action: taskData.action,
      time: taskData.time || '1 hour',
      priority: taskData.priority || 'medium',
      project: taskData.project || 'general',
      status: taskData.status || 'todo',
      assignee: taskData.assignee,
      tags: taskData.tags || [],
      dependencies: taskData.dependencies || [],
      context: taskData.context,
      progress: taskData.progress || 0,
      createdAt: new Date(),
      updatedAt: new Date()
    };

    this.currentContent.tasks.push(task);
    this.searchEngine.updateIndex(this.currentContent);
    
    console.log(`✅ Task created: ${task.action}`);
    console.log(`   ID: ${task.id}, Project: ${task.project}, Priority: ${task.priority}`);
    
    return task;
  }

  /**
   * Update an existing task
   * @param {TaskUpdate} updateData - Task update parameters
   */
  async updateTask(updateData) {
    const taskIndex = this.currentContent.tasks.findIndex(t => t.id === updateData.id);
    
    if (taskIndex === -1) {
      throw new Error(`Task not found: ${updateData.id}`);
    }

    const task = this.currentContent.tasks[taskIndex];
    
    // Apply updates
    if (updateData.action) task.action = updateData.action;
    if (updateData.time) task.time = updateData.time;
    if (updateData.priority) task.priority = updateData.priority;
    if (updateData.project) task.project = updateData.project;
    if (updateData.status) task.status = updateData.status;
    if (updateData.assignee) task.assignee = updateData.assignee;
    if (updateData.tags) task.tags = updateData.tags.split(',').map(tag => tag.trim());
    if (updateData.dependencies) task.dependencies = updateData.dependencies.split(',').map(dep => dep.trim());
    if (updateData.context) task.context = updateData.context;
    if (updateData.progress !== undefined) task.progress = updateData.progress;
    
    task.updatedAt = new Date();
    
    this.searchEngine.updateIndex(this.currentContent);
    console.log(`✅ Task updated: ${task.action}`);
  }

  /**
   * Search across all content types
   * @param {string} query - Search query
   * @param {SearchOptions} options - Search options
   */
  async searchAll(query, options = {}) {
    console.log(`🔍 Searching for: "${query}"`);
    
    const results = this.searchEngine.search(query, options);
    
    // Display results in a user-friendly format
    this.displaySearchResults(results, query);
    
    return results;
  }

  /**
   * Display search results in a formatted way
   */
  displaySearchResults(results, query) {
    const totalResults = results.taskResults.length + 
                        results.memoryResults.length + 
                        results.ideaResults.length;
    
    console.log(`\n📊 Search Results for "${query}"`);
    console.log('══════════════════════════════════════');
    console.log(`Total matches: ${totalResults}`);
    
    if (results.taskResults.length > 0) {
      console.log(`\n📋 Tasks (${results.taskResults.length}):`);
      results.taskResults.slice(0, 3).forEach((task, index) => {
        console.log(`  ${index + 1}. ${task.action} [${task.status}] - ${task.project}`);
      });
      if (results.taskResults.length > 3) {
        console.log(`  ... and ${results.taskResults.length - 3} more`);
      }
    }
    
    if (results.ideaResults.length > 0) {
      console.log(`\n💡 Ideas (${results.ideaResults.length}):`);
      results.ideaResults.slice(0, 2).forEach((idea, index) => {
        console.log(`  ${index + 1}. ${idea.idea.substring(0, 50)}...`);
      });
    }
  }

  /**
   * List all tasks with filtering
   */
  listTasks(filters = {}) {
    let tasks = this.currentContent.tasks;
    
    if (filters.project) {
      tasks = tasks.filter(task => task.project === filters.project);
    }
    
    if (filters.status) {
      tasks = tasks.filter(task => task.status === filters.status);
    }
    
    if (filters.priority) {
      tasks = tasks.filter(task => task.priority === filters.priority);
    }
    
    console.log(`\n📋 Task List (${tasks.length} tasks)`);
    console.log('══════════════════════════════════════');
    
    tasks.forEach((task, index) => {
      const statusEmoji = this.getStatusEmoji(task.status);
      const priorityEmoji = this.getPriorityEmoji(task.priority);
      
      console.log(`${index + 1}. ${statusEmoji} ${priorityEmoji} ${task.action}`);
      console.log(`   📁 ${task.project} | ⏱️ ${task.time} | 📊 ${task.progress}%`);
      if (task.tags.length > 0) {
        console.log(`   🏷️  ${task.tags.join(', ')}`);
      }
      console.log('');
    });
  }

  // Helper methods for emoji display
  getStatusEmoji(status) {
    const emojis = {
      'todo': '📝',
      'in_progress': '🔄',
      'done': '✅',
      'blocked': '🚫'
    };
    return emojis[status] || '❓';
  }

  getPriorityEmoji(priority) {
    const emojis = {
      'low': '🟢',
      'medium': '🟡',
      'high': '🟠',
      'critical': '🔴'
    };
    return emojis[priority] || '⚪';
  }
}

// Usage Example
async function demonstrateTodoziUsage() {
  const todozi = new TodoziTaskManager();

  // Create some sample tasks
  await todozi.createTask({
    action: "Implement user authentication system",
    time: "3 days",
    priority: "high",
    project: "web-app",
    status: "todo",
    tags: ["backend", "security"],
    context: "Need to implement JWT-based authentication"
  });

  await todozi.createTask({
    action: "Design responsive homepage layout",
    time: "2 days",
    priority: "medium",
    project: "web-app",
    status: "in_progress",
    tags: ["frontend", "design"],
    progress: 30
  });

  await todozi.createTask({
    action: "Write API documentation",
    time: "1 day",
    priority: "low",
    project: "documentation",
    status: "todo",
    tags: ["docs", "api"]
  });

  // Update a task
  await todozi.updateTask({
    id: todozi.currentContent.tasks[1].id,
    progress: 75,
    status: "in_progress"
  });

  // List all tasks
  todozi.listTasks();

  // Filter tasks by project
  console.log('\n🔍 Tasks in "web-app" project:');
  todozi.listTasks({ project: "web-app" });

  // Search across content
  await todozi.searchAll("authentication", { limit: 5 });
  await todozi.searchAll("design", { limit: 3 });
}

// Run the demonstration
demonstrateTodoziUsage().catch(console.error);

/*
Here's a practical example showing how to use the Todozi system's task management and search functionality:

**Example Output:**

/ *
✅ Task created: Implement user authentication system
   ID: task_1701234567890_abc123, Project: web-app, Priority: high
✅ Task created: Design responsive homepage layout
   ID: task_1701234567891_def456, Project: web-app, Priority: medium
✅ Task created: Write API documentation
   ID: task_1701234567892_ghi789, Project: documentation, Priority: low
✅ Task updated: Design responsive homepage layout

📋 Task List (3 tasks)
══════════════════════════════════════
1. 📝 🔴 Implement user authentication system
   📁 web-app | ⏱️ 3 days | 📊 0%
   🏷️  backend, security

2. 🔄 🟡 Design responsive homepage layout
   📁 web-app | ⏱️ 2 days | 📊 75%

3. 📝 ⚪ Write API documentation
   📁 documentation | ⏱️ 1 day | 📊 0%
   🏷️  docs, api

🔍 Tasks in "web-app" project:
📋 Task List (2 tasks)
══════════════════════════════════════
1. 📝 🔴 Implement user authentication system
   📁 web-app | ⏱️ 3 days | 📊 0%
   🏷️  backend, security

2. 🔄 🟡 Design responsive homepage layout
   📁 web-app | ⏱️ 2 days | 📊 75%

🔍 Searching for: "authentication"
📊 Search Results for "authentication"
══════════════════════════════════════
Total matches: 1

📋 Tasks (1):
  1. Implement user authentication system [todo] - web-app

🔍 Searching for: "design"
📊 Search Results for "design"
══════════════════════════════════════
Total matches: 1

📋 Tasks (1):
  1. Design responsive homepage layout [in_progress] - web-app
*/