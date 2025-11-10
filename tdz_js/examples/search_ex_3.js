// Example 3: Using the SearchEngine to search across tasks, memories, and ideas

// Initialize the search engine
const engine = new SearchEngine();

// Sample content to index
const content = {
  tasks: [
    {
      id: "task_001",
      action: "Implement user authentication system",
      status: "in_progress",
      priority: "high",
      tags: ["security", "backend", "auth"],
      createdAt: "2024-01-15T10:00:00Z"
    },
    {
      id: "task_002", 
      action: "Design responsive dashboard layout",
      status: "todo",
      priority: "medium",
      tags: ["frontend", "ui", "design"],
      createdAt: "2024-01-16T14:30:00Z"
    },
    {
      id: "task_003",
      action: "Optimize database queries",
      status: "done",
      priority: "critical",
      tags: ["database", "performance", "backend"],
      createdAt: "2024-01-14T09:15:00Z"
    }
  ],
  memories: [
    {
      id: "mem_001",
      moment: "User feedback session",
      meaning: "Users want faster load times for the dashboard",
      reason: "Performance issues detected during testing",
      tags: ["performance", "user-feedback", "optimization"],
      createdAt: "2024-01-15T16:00:00Z"
    },
    {
      id: "mem_002",
      moment: "Security audit completed",
      meaning: "Authentication system needs multi-factor support",
      reason: "Security recommendations from audit report",
      tags: ["security", "audit", "authentication"],
      createdAt: "2024-01-16T11:00:00Z"
    }
  ],
  ideas: [
    {
      id: "idea_001",
      idea: "Implement real-time notifications using WebSockets",
      context: "Would improve user engagement significantly",
      tags: ["notifications", "websockets", "real-time"],
      createdAt: "2024-01-17T10:30:00Z"
    },
    {
      id: "idea_002",
      idea: "Add dark mode theme support",
      context: "Many users requested this feature",
      tags: ["ui", "theme", "accessibility"],
      createdAt: "2024-01-17T13:45:00Z"
    }
  ],
  errors: [
    {
      id: "err_001",
      title: "Database connection timeout",
      description: "Connection to primary database fails after 30 seconds",
      severity: "high",
      source: "backend-api",
      tags: ["database", "timeout", "connection"],
      createdAt: "2024-01-15T18:20:00Z"
    },
    {
      id: "err_002", 
      title: "CORS policy blocking requests",
      description: "Frontend requests to API blocked by CORS configuration",
      severity: "medium",
      source: "frontend",
      tags: ["cors", "frontend", "api"],
      createdAt: "2024-01-16T12:10:00Z"
    }
  ],
  training_data: [
    {
      id: "train_001",
      prompt: "How to implement JWT authentication in Node.js",
      completion: "Use jsonwebtoken library to create and verify tokens",
      tags: ["jwt", "nodejs", "authentication"],
      createdAt: "2024-01-14T15:00:00Z"
    }
  ]
};

// Update the search index with all content
engine.updateIndex(content);

console.log("=== Search Engine Example ===\n");

// Example 1: Basic search across all content types
console.log("1. Basic search for 'authentication':");
const basicResults = engine.search("authentication");
console.log(`Found ${basicResults.totalResults()} total results`);

// Display task results
if (basicResults.taskResults.length > 0) {
  console.log("\nTasks:");
  basicResults.taskResults.forEach(result => {
    console.log(`  - ${result.task.action} (Score: ${result.score.toFixed(2)})`);
  });
}

// Example 2: Search with specific data types filter
console.log("\n\n2. Search for 'database' in tasks and memories only:");
const filteredOptions = {
  dataTypes: [SearchDataType.Tasks, SearchDataType.Memories]
};
const filteredResults = engine.search("database", filteredOptions);

console.log(`Task results: ${filteredResults.taskResults.length}`);
console.log(`Memory results: ${filteredResults.memoryResults.length}`);

filteredResults.taskResults.forEach(result => {
  console.log(`  Task: ${result.task.action} (${result.task.status})`);
});

filteredResults.memoryResults.forEach(result => {
  console.log(`  Memory: ${result.memory.moment} - ${result.memory.meaning}`);
});

// Example 3: Search with date range filter
console.log("\n\n3. Search for 'security' in items created after 2024-01-15:");
const dateOptions = {
  since: "2024-01-15T00:00:00Z"
};
const dateResults = engine.search("security", dateOptions);

console.log(`Results found after specified date: ${dateResults.totalResults()}`);

// Example 4: Search with limit
console.log("\n\n4. Search for 'ui' with limit of 3 results:");
const limitOptions = {
  limit: 3
};
const limitedResults = engine.search("ui", limitOptions);
console.log(`Limited to ${limitedResults.totalResults()} results`);

// Example 5: Get search suggestions
console.log("\n\n5. Get search suggestions for 'auth':");
const suggestions = engine.getSearchSuggestions("auth", 5);
console.log("Suggestions:", suggestions);

// Example 6: Get search analytics
console.log("\n\n6. Search Analytics:");
const analytics = engine.getSearchAnalytics();
console.log(`Total indexed items: ${analytics.totalIndexedItems}`);
console.log(`Tasks: ${analytics.tasksCount}`);
console.log(`Memories: ${analytics.memoriesCount}`);
console.log(`Ideas: ${analytics.ideasCount}`);
console.log(`Errors: ${analytics.errorsCount}`);
console.log(`Training data: ${analytics.trainingCount}`);

// Example 7: Advanced search for tasks with specific criteria
console.log("\n\n7. Advanced search for high priority tasks:");
const taskCriteria = {
  taskCriteria: {
    status: "todo",
    priority: "high"
  }
};
const advancedResults = engine.advancedSearch(taskCriteria);

if (advancedResults.taskResults.length > 0) {
  console.log("Found matching tasks:");
  advancedResults.taskResults.forEach(result => {
    console.log(`  - ${result.task.action} (Priority: ${result.task.priority})`);
  });
}

// Example 8: Search with multiple keywords
console.log("\n\n8. Search for multiple keywords ('backend performance'):");
const keywordResults = engine.search("backend performance");
console.log(`Found ${keywordResults.totalResults()} results containing keywords`);

keywordResults.taskResults.forEach(result => {
  console.log(`  Task: ${result.task.action} (Score: ${result.score.toFixed(2)})`);
});

// Example 9: Search by tags
console.log("\n\n9. Search by tag 'frontend':");
const tagResults = engine.search("frontend");
console.log(`Found ${tagResults.totalResults()} items with 'frontend' tag`);

// Example 10: Get relevance score breakdown
console.log("\n\n10. Relevance score analysis for 'API':");
const apiResults = engine.search("API");

if (apiResults.taskResults.length > 0) {
  console.log("Task results with scores:");
  apiResults.taskResults.forEach(result => {
    console.log(`  - "${result.task.action}"`);
    console.log(`    Score: ${result.score.toFixed(3)}`);
    console.log(`    Tags: ${result.task.tags.join(', ')}`);
    console.log(`    Created: ${result.task.createdAt}`);
    console.log();
  });
}

console.log("=== Search Engine Example Complete ===");

/*
I'll create a practical example that demonstrates how to use the SearchEngine class to index and search through different types of content.

This example demonstrates the full capabilities of the SearchEngine class:

1. **Indexing Content**: Shows how to update the index with multiple content types (tasks, memories, ideas, errors, training data)

2. **Basic Search**: Simple text search across all content types with relevance scoring

3. **Filtered Search**: Searching only specific data types using the `dataTypes` option

4. **Date Filtering**: Searching content within a specific date range using `since` parameter

5. **Result Limiting**: Limiting the number of results returned using the `limit` option

6. **Search Suggestions**: Getting keyword suggestions based on existing content

7. **Analytics**: Retrieving statistics about the indexed content

8. **Advanced Search**: Using criteria-based filtering for specific content types

9. **Multi-keyword Search**: Searching with multiple words in the query

10. **Tag-based Search**: Finding content that matches specific tags

11. **Relevance Scoring**: Understanding how search results are scored and ranked

The example uses realistic data that might be found in a project management system, demonstrating how the search engine can help users quickly find relevant information across different types of content. The search results are sorted by relevance score, making the most relevant items appear first.
*/