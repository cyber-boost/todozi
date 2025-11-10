// example4.c - Advanced Search with Filtering Criteria
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "search.c"  // Include the search engine implementation

int main() {
    // Initialize search engine
    SearchEngine* engine = search_engine_new();
    if (!engine) {
        printf("Failed to create search engine\n");
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
            .tags = (char*[]){"security", "backend"},
            .tags_count = 2,
            .created_at = time(NULL)
        },
        {
            .action = "Design homepage UI",
            .status = PENDING,
            .priority = MEDIUM,
            .assignee = "bob",
            .tags = (char*[]){"frontend", "design"},
            .tags_count = 2,
            .created_at = time(NULL)
        },
        {
            .action = "Fix database connection issue",
            .status = COMPLETED,
            .priority = HIGH,
            .assignee = "alice",
            .tags = (char*[]){"backend", "bug"},
            .tags_count = 2,
            .created_at = time(NULL)
        }
    };
    content.tasks = tasks;
    content.tasks_count = 3;

    // Sample memories
    Memory memories[] = {
        {
            .moment = "User login failure",
            .meaning = "Authentication system failed under load",
            .importance = HIGH_IMPORTANCE,
            .term = SHORT_TERM,
            .tags = (char*[]){"security", "performance"},
            .tags_count = 2,
            .created_at = time(NULL)
        },
        {
            .moment = "Team meeting discussion",
            .meaning = "Decided to use React for frontend",
            .importance = MEDIUM_IMPORTANCE,
            .term = LONG_TERM,
            .tags = (char*[]){"frontend", "decision"},
            .tags_count = 2,
            .created_at = time(NULL)
        }
    };
    content.memories = memories;
    content.memories_count = 2;

    // Update search index with sample data
    search_engine_update_index(engine, &content);

    // Example 1: Advanced search for high priority tasks assigned to Alice
    printf("=== Advanced Task Search ===\n");
    AdvancedSearchCriteria task_criteria = {0};
    task_criteria.task_criteria.filter_by_priority = true;
    task_criteria.task_criteria.priority = HIGH;
    task_criteria.task_criteria.assignee = "alice";
    
    SearchResults results1 = search_engine_advanced_search(engine, task_criteria);
    printf("Found %d high priority tasks assigned to Alice:\n", results1.task_results_count);
    for (int i = 0; i < results1.task_results_count; i++) {
        printf("  - %s (Status: %d)\n", results1.task_results[i].task.action, results1.task_results[i].task.status);
    }
    search_results_free(&results1);

    // Example 2: Advanced search for high importance short-term memories with security tag
    printf("\n=== Advanced Memory Search ===\n");
    AdvancedSearchCriteria memory_criteria = {0};
    memory_criteria.memory_criteria.filter_by_importance = true;
    memory_criteria.memory_criteria.importance = HIGH_IMPORTANCE;
    memory_criteria.memory_criteria.filter_by_term = true;
    memory_criteria.memory_criteria.term = SHORT_TERM;
    memory_criteria.memory_criteria.required_tag = "security";
    
    SearchResults results2 = search_engine_advanced_search(engine, memory_criteria);
    printf("Found %d high importance short-term memories with 'security' tag:\n", results2.memory_results_count);
    for (int i = 0; i < results2.memory_results_count; i++) {
        printf("  - %s: %s\n", results2.memory_results[i].memory.moment, results2.memory_results[i].memory.meaning);
    }
    search_results_free(&results2);

    // Example 3: Combined criteria search
    printf("\n=== Combined Advanced Search ===\n");
    AdvancedSearchCriteria combined_criteria = {0};
    // Task criteria: Medium priority, pending status
    combined_criteria.task_criteria.filter_by_priority = true;
    combined_criteria.task_criteria.priority = MEDIUM;
    combined_criteria.task_criteria.filter_by_status = true;
    combined_criteria.task_criteria.status = PENDING;
    // Memory criteria: Long term memories
    combined_criteria.memory_criteria.filter_by_term = true;
    combined_criteria.memory_criteria.term = LONG_TERM;
    
    SearchResults results3 = search_engine_advanced_search(engine, combined_criteria);
    printf("Found %d tasks and %d memories matching combined criteria:\n", 
           results3.task_results_count, results3.memory_results_count);
    
    printf("Tasks:\n");
    for (int i = 0; i < results3.task_results_count; i++) {
        printf("  - %s (Assignee: %s)\n", 
               results3.task_results[i].task.action, 
               results3.task_results[i].task.assignee);
    }
    
    printf("Memories:\n");
    for (int i = 0; i < results3.memory_results_count; i++) {
        printf("  - %s: %s\n", 
               results3.memory_results[i].memory.moment, 
               results3.memory_results[i].memory.meaning);
    }
    search_results_free(&results3);

    // Cleanup
    search_engine_free(engine);
    return 0;
}
