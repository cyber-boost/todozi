// example5.c - Advanced Search with Custom Criteria
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "search.c" // Include the main search implementation

void print_task_result(TaskResult* result) {
    const char* status_str[] = {"PENDING", "IN_PROGRESS", "COMPLETED"};
    const char* priority_str[] = {"LOW", "MEDIUM", "HIGH"};
    
    printf("Task: %s\n", result->task.action);
    printf("  Status: %s | Priority: %s\n", 
           status_str[result->task.status], 
           priority_str[result->task.priority]);
    if (result->task.assignee) {
        printf("  Assignee: %s\n", result->task.assignee);
    }
    printf("  Tags: ");
    for (int i = 0; i < result->task.tags_count; i++) {
        printf("%s ", result->task.tags[i]);
    }
    printf("\n");
}

void print_memory_result(MemoryResult* result) {
    const char* importance_str[] = {"LOW", "MEDIUM", "HIGH"};
    const char* term_str[] = {"SHORT_TERM", "LONG_TERM"};
    
    printf("Memory: %s\n", result->memory.moment);
    printf("  Meaning: %s\n", result->memory.meaning);
    printf("  Importance: %s | Term: %s\n", 
           importance_str[result->memory.importance], 
           term_str[result->memory.term]);
    printf("  Tags: ");
    for (int i = 0; i < result->memory.tags_count; i++) {
        printf("%s ", result->memory.tags[i]);
    }
    printf("\n");
}

int main() {
    // Initialize search engine
    SearchEngine* engine = search_engine_new();
    if (!engine) {
        fprintf(stderr, "Failed to create search engine\n");
        return 1;
    }

    // Create sample data
    ChatContent content = {0};
    
    // Sample tasks
    Task tasks[] = {
        {
            .action = "Implement user authentication",
            .status = IN_PROGRESS,
            .priority = HIGH,
            .assignee = "alice",
            .tags = (char*[]){"auth", "security", "backend"},
            .tags_count = 3,
            .created_at = time(NULL)
        },
        {
            .action = "Design database schema",
            .status = COMPLETED,
            .priority = MEDIUM,
            .assignee = "bob",
            .tags = (char*[]){"database", "design", "backend"},
            .tags_count = 3,
            .created_at = time(NULL)
        },
        {
            .action = "Write API documentation",
            .status = PENDING,
            .priority = LOW,
            .assignee = "charlie",
            .tags = (char*[]){"docs", "api", "frontend"},
            .tags_count = 3,
            .created_at = time(NULL)
        }
    };
    content.tasks = tasks;
    content.tasks_count = 3;

    // Sample memories
    Memory memories[] = {
        {
            .moment = "First successful login",
            .meaning = "User authentication is working",
            .reason = "Implemented JWT tokens",
            .importance = HIGH_IMPORTANCE,
            .term = LONG_TERM,
            .tags = (char*[]){"auth", "success", "milestone"},
            .tags_count = 3,
            .created_at = time(NULL)
        },
        {
            .moment = "Database connection timeout",
            .meaning = "Need to optimize queries",
            .reason = "High load during testing",
            .importance = MEDIUM_IMPORTANCE,
            .term = SHORT_TERM,
            .tags = (char*[]){"database", "performance", "issue"},
            .tags_count = 3,
            .created_at = time(NULL)
        }
    };
    content.memories = memories;
    content.memories_count = 2;

    // Update search index
    search_engine_update_index(engine, &content);

    // Create advanced search criteria
    AdvancedSearchCriteria criteria = {0};
    
    // Task search criteria: High priority, in progress tasks assigned to alice with 'auth' tag
    criteria.task_criteria.filter_by_status = true;
    criteria.task_criteria.status = IN_PROGRESS;
    criteria.task_criteria.filter_by_priority = true;
    criteria.task_criteria.priority = HIGH;
    criteria.task_criteria.assignee = "alice";
    criteria.task_criteria.required_tag = "auth";
    
    // Memory search criteria: High importance long-term memories with 'auth' tag
    criteria.memory_criteria.filter_by_importance = true;
    criteria.memory_criteria.importance = HIGH_IMPORTANCE;
    criteria.memory_criteria.filter_by_term = true;
    criteria.memory_criteria.term = LONG_TERM;
    criteria.memory_criteria.required_tag = "auth";

    // Perform advanced search
    printf("=== Advanced Search Results ===\n");
    SearchResults results = search_engine_advanced_search(engine, criteria);
    
    printf("Found %d tasks and %d memories matching criteria:\n\n", 
           results.task_results_count, results.memory_results_count);
    
    // Print task results
    for (int i = 0; i < results.task_results_count; i++) {
        printf("Task Result %d:\n", i+1);
        print_task_result(&results.task_results[i]);
        printf("\n");
    }
    
    // Print memory results
    for (int i = 0; i < results.memory_results_count; i++) {
        printf("Memory Result %d:\n", i+1);
        print_memory_result(&results.memory_results[i]);
        printf("\n");
    }

    // Clean up
    search_results_free(&results);
    search_engine_free(engine);
    
    return 0;
}
