#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "search.c"  // Include the search engine implementation

int main() {
    // Initialize search engine
    SearchEngine* engine = search_engine_new();
    
    // Create sample data
    ChatContent content = {0};
    
    // Create sample tasks
    Task tasks[3];
    tasks[0].action = "Implement user authentication";
    tasks[0].status = PENDING;
    tasks[0].priority = HIGH;
    tasks[0].assignee = "john_doe";
    tasks[0].tags = malloc(2 * sizeof(char*));
    tasks[0].tags[0] = "security";
    tasks[0].tags[1] = "backend";
    tasks[0].tags_count = 2;
    tasks[0].created_at = time(NULL);
    
    tasks[1].action = "Design homepage UI";
    tasks[1].status = IN_PROGRESS;
    tasks[1].priority = MEDIUM;
    tasks[1].assignee = "jane_smith";
    tasks[1].tags = malloc(2 * sizeof(char*));
    tasks[1].tags[0] = "frontend";
    tasks[1].tags[1] = "design";
    tasks[1].tags_count = 2;
    tasks[1].created_at = time(NULL);
    
    tasks[2].action = "Fix database connection issue";
    tasks[2].status = COMPLETED;
    tasks[2].priority = HIGH;
    tasks[2].assignee = "john_doe";
    tasks[2].tags = malloc(2 * sizeof(char*));
    tasks[2].tags[0] = "database";
    tasks[2].tags[1] = "bug";
    tasks[2].tags_count = 2;
    tasks[2].created_at = time(NULL);
    
    content.tasks = tasks;
    content.tasks_count = 3;
    
    // Create sample memories
    Memory memories[2];
    memories[0].moment = "First successful deployment";
    memories[0].meaning = "Team achieved milestone";
    memories[0].reason = "Hard work and collaboration";
    memories[0].importance = HIGH_IMPORTANCE;
    memories[0].term = LONG_TERM;
    memories[0].tags = malloc(1 * sizeof(char*));
    memories[0].tags[0] = "milestone";
    memories[0].tags_count = 1;
    memories[0].created_at = time(NULL);
    
    memories[1].moment = "Debugging session with team";
    memories[1].meaning = "Learned new debugging techniques";
    memories[1].reason = "Complex issue required collaboration";
    memories[1].importance = MEDIUM_IMPORTANCE;
    memories[1].term = SHORT_TERM;
    memories[1].tags = malloc(1 * sizeof(char*));
    memories[1].tags[0] = "learning";
    memories[1].tags_count = 1;
    memories[1].created_at = time(NULL);
    
    content.memories = memories;
    content.memories_count = 2;
    
    // Update search engine index
    search_engine_update_index(engine, &content);
    
    // Perform advanced search for tasks
    printf("=== Advanced Task Search ===\n");
    AdvancedSearchCriteria criteria = {0};
    
    // Set task search criteria
    criteria.task_criteria.filter_by_status = true;
    criteria.task_criteria.status = PENDING;
    criteria.task_criteria.filter_by_priority = true;
    criteria.task_criteria.priority = HIGH;
    criteria.task_criteria.assignee = "john_doe";
    criteria.task_criteria.required_tag = "security";
    
    SearchResults results = search_engine_advanced_search(engine, criteria);
    
    printf("Found %d matching tasks:\n", results.task_results_count);
    for (int i = 0; i < results.task_results_count; i++) {
        printf("  - %s (Status: %d, Priority: %d)\n", 
               results.task_results[i].task.action,
               results.task_results[i].task.status,
               results.task_results[i].task.priority);
    }
    
    search_results_free(&results);
    
    // Perform advanced search for memories
    printf("\n=== Advanced Memory Search ===\n");
    criteria.task_criteria.filter_by_status = false;
    criteria.task_criteria.filter_by_priority = false;
    criteria.task_criteria.assignee = NULL;
    criteria.task_criteria.required_tag = NULL;
    
    criteria.memory_criteria.filter_by_importance = true;
    criteria.memory_criteria.importance = HIGH_IMPORTANCE;
    criteria.memory_criteria.filter_by_term = true;
    criteria.memory_criteria.term = LONG_TERM;
    criteria.memory_criteria.required_tag = "milestone";
    
    results = search_engine_advanced_search(engine, criteria);
    
    printf("Found %d matching memories:\n", results.memory_results_count);
    for (int i = 0; i < results.memory_results_count; i++) {
        printf("  - %s (Importance: %d, Term: %d)\n", 
               results.memory_results[i].memory.moment,
               results.memory_results[i].memory.importance,
               results.memory_results[i].memory.term);
    }
    
    search_results_free(&results);
    
    // Cleanup
    search_engine_free(engine);
    
    // Free allocated memory for tags
    for (int i = 0; i < 3; i++) {
        free(tasks[i].tags);
    }
    for (int i = 0; i < 2; i++) {
        free(memories[i].tags);
    }
    
    return 0;
}
