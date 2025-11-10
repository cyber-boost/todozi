// Example 2: Creating and Managing Tasks with the TUI Service

import { 
  TuiService, 
  Task, 
  Priority, 
  Status, 
  Assignee, 
  DisplayConfig,
  ColorScheme
} from './tui.js';

// Mock embedding service for demonstration
class MockEmbeddingService {
  async getTask(taskId) {
    // Simulate fetching a task with embedding
    return new Task({
      id: taskId,
      action: "Implement user authentication",
      time: "2 hours",
      priority: Priority.High,
      status: Status.Todo,
      assignee: Assignee.Human,
      parentProject: "Web App Development",
      tags: ["security", "frontend", "backend"],
      contextNotes: "Need to integrate with existing OAuth system",
      progress: 0,
      embeddingVector: [0.1, 0.2, 0.3] // Mock embedding vector
    });
  }

  async findSimilarTasks(query, limit) {
    // Simulate finding similar tasks
    return [
      {
        taskId: "task-002",
        similarity: 0.85,
        tags: ["security", "authentication"]
      },
      {
        taskId: "task-003",
        similarity: 0.72,
        tags: ["frontend", "login"]
      }
    ];
  }

  async semanticSearch(query, contentTypes, limit) {
    // Simulate semantic search results
    return [
      {
        id: "mem-001",
        content: "User authentication best practices",
        type: "Memory"
      }
    ];
  }
}

// Example usage
async function demonstrateTaskManagement() {
  console.log("=== Todozi TUI Task Management Example ===\n");

  // 1. Initialize services
  const embeddingService = new MockEmbeddingService();
  const displayConfig = new DisplayConfig();
  displayConfig.colorScheme = ColorScheme.ansiFallbackScheme(); // For compatibility
  const tuiService = new TuiService(embeddingService, displayConfig);

  // 2. Create a new task
  const newTask = new Task({
    action: "Design database schema for user profiles",
    time: "3 hours",
    priority: Priority.Medium,
    status: Status.InProgress,
    assignee: Assignee.Collaborative,
    parentProject: "Web App Development",
    tags: ["database", "design", "schema"],
    contextNotes: "Follow company data modeling standards",
    progress: 50
  });

  console.log("1. Created New Task:");
  console.log(`   Action: ${newTask.action}`);
  console.log(`   Project: ${newTask.parentProject}`);
  console.log(`   Priority: ${newTask.priority}`);
  console.log(`   Status: ${newTask.status}`);
  console.log(`   Assignee: ${newTask.assignee}`);
  console.log(`   Progress: ${newTask.progress}%\n`);

  // 3. Display task with AI enhancements (using mock service)
  try {
    const taskDisplay = await tuiService.displayTask("task-001");
    
    console.log("2. Task Display with AI Enhancements:");
    console.log("   " + taskDisplay.render(displayConfig));
    console.log("\n   Compact View:");
    console.log("   " + taskDisplay.renderCompact(displayConfig));
    console.log("\n   Detailed View:");
    console.log("   " + taskDisplay.renderDetailed(displayConfig));
    
    console.log("\n   AI Suggestions:");
    taskDisplay.aiSuggestions.forEach((suggestion, index) => {
      console.log(`     ${index + 1}. ${suggestion}`);
    });
    
    console.log("\n   Semantic Tags:");
    console.log("     " + taskDisplay.semanticTags.join(", "));
    
    console.log(`\n   Confidence Score: ${(taskDisplay.confidenceScore * 100).toFixed(1)}%`);
    
    console.log("\n   Related Content:");
    taskDisplay.relatedContent.forEach((content, index) => {
      console.log(`     ${index + 1}. ${content.content} (${content.type})`);
    });
    
  } catch (error) {
    console.error("Error displaying task:", error.message);
  }

  // 4. Display multiple tasks
  try {
    const taskIds = ["task-001", "task-002", "task-003"];
    const taskListDisplay = await tuiService.displayTasks(taskIds);
    
    console.log("\n3. Task List Display:");
    console.log("   " + taskListDisplay.render(displayConfig));
    console.log("\n   Compact View:");
    console.log("   " + taskListDisplay.renderCompact(displayConfig));
    console.log("\n   Detailed View:");
    console.log("   " + taskListDisplay.renderDetailed(displayConfig));
    
    console.log("\n   AI Summary:");
    console.log("   " + taskListDisplay.aiSummary);
    
    console.log("\n   Semantic Clusters:");
    taskListDisplay.semanticClusters.forEach((cluster, index) => {
      console.log(`     Cluster ${index + 1}: ${cluster.length} related tasks`);
    });
    
  } catch (error) {
    console.error("Error displaying task list:", error.message);
  }

  // 5. Start an edit session
  try {
    const editSession = await tuiService.startEditSession("task-001");
    
    console.log("\n4. Edit Session Started:");
    console.log(`   Task ID: ${editSession.taskId}`);
    console.log(`   Original Action: ${editSession.originalTask.action}`);
    console.log(`   Current Action: ${editSession.currentTask.action}`);
    console.log(`   Session Start: ${editSession.sessionStart}`);
    
    console.log("\n   Similarity Matches:");
    editSession.similarityMatches.forEach((match, index) => {
      console.log(`     ${index + 1}. Task ${match.taskId} (${(match.similarity * 100).toFixed(1)}% similar)`);
    });
    
    console.log("\n   AI Suggestions:");
    editSession.aiSuggestions.forEach((suggestion, index) => {
      console.log(`     ${index + 1}. ${suggestion}`);
    });
    
  } catch (error) {
    console.error("Error starting edit session:", error.message);
  }

  // 6. Demonstrate task filtering and sorting
  console.log("\n5. Task Filtering Example:");
  const filters = {
    statusFilter: Status.InProgress,
    priorityFilter: Priority.High,
    projectFilter: "Web App Development"
  };
  
  console.log("   Applied Filters:");
  Object.entries(filters).forEach(([key, value]) => {
    if (value) console.log(`     ${key}: ${value}`);
  });

  // 7. Show color scheme capabilities
  console.log("\n6. Color Scheme Example:");
  const colors = displayConfig.colorScheme;
  console.log(colors.primary("Primary Text"));
  console.log(colors.success("Success Message"));
  console.log(colors.warning("Warning Message"));
  console.log(colors.danger("Error Message"));
  console.log(colors.info("Info Message"));

  console.log("\n=== End of Example ===");
}

// Run the demonstration
demonstrateTaskManagement().catch(console.error);