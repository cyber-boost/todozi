// example2.c - Advanced search example with filtering
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "search.c"  // Include the search engine implementation

// Helper function to create a task
Task create_task(const char* action, Status status, Priority priority, const char* assignee, const char** tags, int tags_count) {
    Task task;
    task.action = malloc(strlen(action) + 1);
    strcpy(task.action, action);
    task.status = status;
    task.priority = priority;
    task.assignee = malloc(strlen(assignee) + 1);
    strcpy(task.assignee, assignee);
    task.tags_count = tags_count;
    task.tags = malloc(sizeof(char*) * tags_count);
    for (int i = 0; i < tags_count; i++) {
        task.tags[i] = malloc(strlen(tags[i]) + 1);
        strcpy(task.tags[i], tags[i]);
    }
    task.created_at = time(NULL);
    return task;
}

// Helper function to create a memory
Memory create_memory(const char* moment, const char* meaning, MemoryImportance importance, MemoryTerm term, const char** tags, int tags_count) {
    Memory memory;
    memory.moment = malloc(strlen(moment) + 1);
    strcpy(memory.moment, moment);
    memory.meaning = malloc(strlen(meaning) + 1);
    strcpy(memory.meaning, meaning);
    memory.importance = importance;
    memory.term = term;
    memory.tags_count = tags_count;
    memory.tags = malloc(sizeof(char*) * tags_count);
    for (int i = 0; i < tags_count; i++) {
        memory.tags[i] = malloc(strlen(tags[i]) + 1);
        strcpy(memory.tags[i], tags[i]);
    }
    memory.created_at = time(NULL);
    return memory;
}

int main() {
    // Initialize search engine
    SearchEngine* engine = search_engine_new();
    if (!engine) {
        printf("Failed to create search engine\n");
        return 1;
    }

    // Create sample tasks
    const char* task1_tags[] = {"urgent", "development", "backend"};
    Task task1 = create_task("Fix database connection issue", COMPLETED, HIGH, "Alice", task1_tags, 3);
    
    const char* task2_tags[] = {"feature", "frontend", "ui"};
    Task task2 = create_task("Implement new user dashboard", IN_PROGRESS, MEDIUM, "Bob", task2_tags, 3);
    
    const char* task3_tags[] = {"maintenance", "backend", "low-priority"};
    Task task3 = create_task("Update documentation", PENDING, LOW, "Charlie", task3_tags, 3);

    // Create sample memories
    const char* memory1_tags[] = {"important", "learning"};
    Memory memory1 = create_memory("First successful deployment", "Team learned importance of testing", HIGH_IMPORTANCE, LONG_TERM, memory1_tags, 2);
    
    const char* memory2_tags[] = {"process", "improvement"};
    Memory memory2 = create_memory("Code review session", "Implemented better review practices", MEDIUM_IMPORTANCE, SHORT_TERM, memory2_tags, 2);

    // Create chat content to index
    ChatContent content;
    content.tasks = malloc(sizeof(Task) * 3);
    content.tasks[0] = task1;
    content.tasks[1] = task2;
    content.tasks[2] = task3;
    content.tasks_count = 3;
    
    content.memories = malloc(sizeof(Memory) * 2);
    content.memories[0] = memory1;
    content.memories[1] = memory2;
    content.memories_count = 2;
    
    // Initialize other content types (empty for this example)
    content.ideas = NULL;
    content.ideas_count = 0;
    content.errors = NULL;
    content.errors_count = 0;
    content.training_data = NULL;
    content.training_data_count = 0;

    // Update search index
    search_engine_update_index(engine, &content);
    printf("Indexed %d tasks and %d memories\n", content.tasks_count, content.memories_count);

    // Perform advanced search with criteria
    printf("\n=== Advanced Search Examples ===\n");
    
    // Example 1: Search for high priority tasks assigned to Alice
    AdvancedSearchCriteria criteria1;
    criteria1.task_criteria.filter_by_status = false;
    criteria1.task_criteria.filter_by_priority = true;
    criteria1.task_criteria.priority = HIGH;
    criteria1.task_criteria.assignee = "Alice";
    criteria1.task_criteria.required_tag = NULL;
    
    criteria1.memory_criteria.filter_by_importance = false;
    criteria1.memory_criteria.filter_by_term = false;
    criteria1.memory_criteria.required_tag = NULL;
    
    SearchResults results1 = search_engine_advanced_search(engine, criteria1);
    printf("High priority tasks assigned to Alice: %d results\n", results1.task_results_count);
    for (int i = 0; i < results1.task_results_count; i++) {
        printf("  - %s\n", results1.task_results[i].task.action);
    }
    search_results_free(&results1);

    // Example 2: Search for high importance long-term memories with "learning" tag
    AdvancedSearchCriteria criteria2;
    criteria2.task_criteria.filter_by_status = false;
    criteria2.task_criteria.filter_by_priority = false;
    criteria2.task_criteria.assignee = NULL;
    criteria2.task_criteria.required_tag = NULL;
    
    criteria2.memory_criteria.filter_by_importance = true;
    criteria2.memory_criteria.importance = HIGH_IMPORTANCE;
    criteria2.memory_criteria.filter_by_term = true;
    criteria2.memory_criteria.term = LONG_TERM;
    criteria2.memory_criteria.required_tag = "learning";
    
    SearchResults results2 = search_engine_advanced_search(engine, criteria2);
    printf("\nHigh importance long-term memories with 'learning' tag: %d results\n", results2.memory_results_count);
    for (int i = 0; i < results2.memory_results_count; i++) {
        printf("  - %s: %s\n", results2.memory_results[i].memory.moment, results2.memory_results[i].memory.meaning);
    }
    search_results_free(&results2);

    // Example 3: Get search analytics
    SearchAnalytics analytics = search_engine_get_search_analytics(engine);
    printf("\n=== Search Analytics ===\n");
    printf("Total indexed items: %d\n", analytics.total_indexed_items);
    printf("Tasks: %d\n", analytics.tasks_count);
    printf("Memories: %d\n", analytics.memories_count);
    printf("Ideas: %d\n", analytics.ideas_count);
    printf("Errors: %d\n", analytics.errors_count);
    printf("Training data: %d\n", analytics.training_count);

    // Example 4: Get search suggestions
    printf("\n=== Search Suggestions ===\n");
    int suggestion_count;
    char** suggestions = search_engine_get_search_suggestions(engine, "back", 5, &suggestion_count);
    printf("Suggestions for 'back': %d results\n", suggestion_count);
    for (int i = 0; i < suggestion_count; i++) {
        printf("  - %s\n", suggestions[i]);
        free(suggestions[i]);
    }
    free(suggestions);

    // Cleanup
    search_engine_free(engine);
    
    // Free allocated content
    for (int i = 0; i < content.tasks_count; i++) {
        free(content.tasks[i].action);
        free(content.tasks[i].assignee);
        for (int j = 0; j < content.tasks[i].tags_count; j++) {
            free(content.tasks[i].tags[j]);
        }
        free(content.tasks[i].tags);
    }
    free(content.tasks);
    
    for (int i = 0; i < content.memories_count; i++) {
        free(content.memories[i].moment);
        free(content.memories[i].meaning);
        for (int j = 0; j < content.memories[i].tags_count; j++) {
            free(content.memories[i].tags[j]);
        }
        free(content.memories[i].tags);
    }
    free(content.memories);

    return 0;
}
