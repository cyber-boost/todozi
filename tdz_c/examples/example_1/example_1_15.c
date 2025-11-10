// example1.c - Basic search engine usage example

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the search engine header (assuming it's in search.h)
// For this example, we'll include the necessary declarations directly
extern SearchEngine* search_engine_new();
extern void search_engine_free(SearchEngine* engine);
extern void search_engine_update_index(SearchEngine* engine, ChatContent* content);
extern SearchResults search_engine_search(SearchEngine* engine, const char* query, SearchOptions options);
extern SearchAnalytics search_engine_get_search_analytics(SearchEngine* engine);
extern void search_results_free(SearchResults* results);
extern int search_results_total_results(SearchResults* results);

// Helper function to create a task
Task create_task(const char* action, Status status, Priority priority, const char* assignee, const char* tags_str) {
    Task task;
    task.action = strdup(action);
    task.status = status;
    task.priority = priority;
    task.assignee = strdup(assignee);
    task.created_at = time(NULL);
    
    // Parse tags
    if (tags_str) {
        char* tags_copy = strdup(tags_str);
        char* token = strtok(tags_copy, ",");
        task.tags_count = 0;
        char** temp_tags = malloc(sizeof(char*) * 10); // Max 10 tags
        
        while (token && task.tags_count < 10) {
            temp_tags[task.tags_count] = strdup(token);
            task.tags_count++;
            token = strtok(NULL, ",");
        }
        
        task.tags = malloc(sizeof(char*) * task.tags_count);
        for (int i = 0; i < task.tags_count; i++) {
            task.tags[i] = temp_tags[i];
        }
        free(temp_tags);
        free(tags_copy);
    } else {
        task.tags = NULL;
        task.tags_count = 0;
    }
    
    return task;
}

// Helper function to create a memory
Memory create_memory(const char* moment, const char* meaning, MemoryImportance importance, const char* tags_str) {
    Memory memory;
    memory.moment = strdup(moment);
    memory.meaning = strdup(meaning);
    memory.reason = strdup("Important learning experience");
    memory.importance = importance;
    memory.term = LONG_TERM;
    memory.created_at = time(NULL);
    
    // Parse tags
    if (tags_str) {
        char* tags_copy = strdup(tags_str);
        char* token = strtok(tags_copy, ",");
        memory.tags_count = 0;
        char** temp_tags = malloc(sizeof(char*) * 10);
        
        while (token && memory.tags_count < 10) {
            temp_tags[memory.tags_count] = strdup(token);
            memory.tags_count++;
            token = strtok(NULL, ",");
        }
        
        memory.tags = malloc(sizeof(char*) * memory.tags_count);
        for (int i = 0; i < memory.tags_count; i++) {
            memory.tags[i] = temp_tags[i];
        }
        free(temp_tags);
        free(tags_copy);
    } else {
        memory.tags = NULL;
        memory.tags_count = 0;
    }
    
    return memory;
}

int main() {
    // Create search engine
    SearchEngine* engine = search_engine_new();
    if (!engine) {
        printf("Failed to create search engine\n");
        return 1;
    }
    
    // Create sample content
    ChatContent content = {0};
    
    // Create sample tasks
    Task tasks[3];
    tasks[0] = create_task("Implement user authentication system", COMPLETED, HIGH, "Alice", "security,backend,authentication");
    tasks[1] = create_task("Design database schema for chat messages", IN_PROGRESS, MEDIUM, "Bob", "database,design,messaging");
    tasks[2] = create_task("Fix memory leak in search module", PENDING, HIGH, "Charlie", "bug,performance,search");
    
    content.tasks = tasks;
    content.tasks_count = 3;
    
    // Create sample memories
    Memory memories[2];
    memories[0] = create_memory("User reported slow search performance", "Database indexing was missing", HIGH_IMPORTANCE, "performance,optimization,database");
    memories[1] = create_memory("Team meeting about project timeline", "Need to prioritize critical features", MEDIUM_IMPORTANCE, "planning,team,project");
    
    content.memories = memories;
    content.memories_count = 2;
    
    // Update search engine index
    search_engine_update_index(engine, &content);
    
    // Get analytics
    SearchAnalytics analytics = search_engine_get_search_analytics(engine);
    printf("Search Engine Analytics:\n");
    printf("  Total items indexed: %d\n", analytics.total_indexed_items);
    printf("  Tasks: %d\n", analytics.tasks_count);
    printf("  Memories: %d\n", analytics.memories_count);
    printf("\n");
    
    // Perform searches
    SearchOptions options = {0}; // Default options
    
    printf("Searching for 'database':\n");
    SearchResults results1 = search_engine_search(engine, "database", options);
    printf("  Found %d results\n", search_results_total_results(&results1));
    search_results_free(&results1);
    printf("\n");
    
    printf("Searching for 'authentication':\n");
    SearchResults results2 = search_engine_search(engine, "authentication", options);
    printf("  Found %d results\n", search_results_total_results(&results2));
    search_results_free(&results2);
    printf("\n");
    
    printf("Searching for 'performance':\n");
    SearchResults results3 = search_engine_search(engine, "performance", options);
    printf("  Found %d results\n", search_results_total_results(&results3));
    search_results_free(&results3);
    printf("\n");
    
    // Clean up
    search_engine_free(engine);
    
    // Free allocated memory for tasks
    for (int i = 0; i < 3; i++) {
        free(tasks[i].action);
        free(tasks[i].assignee);
        for (int j = 0; j < tasks[i].tags_count; j++) {
            free(tasks[i].tags[j]);
        }
        free(tasks[i].tags);
    }
    
    // Free allocated memory for memories
    for (int i = 0; i < 2; i++) {
        free(memories[i].moment);
        free(memories[i].meaning);
        free(memories[i].reason);
        for (int j = 0; j < memories[i].tags_count; j++) {
            free(memories[i].tags[j]);
        }
        free(memories[i].tags);
    }
    
    printf("Example completed successfully!\n");
    return 0;
}
