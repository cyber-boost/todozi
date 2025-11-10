// example5_todozi_usage.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"
#include "todozi_exe.c"  // Include the main implementation

void demonstrate_task_operations() {
    printf("=== Todozi Task Management Examples ===\n\n");
    
    // Example 1: Create a simple task
    printf("1. Creating a simple task:\n");
    json_object* simple_params = json_object_new_object();
    json_object_object_add(simple_params, "content", json_object_new_string("Review project proposal"));
    ExecutionResult* simple_result = execute_simple_task(simple_params);
    if (simple_result && simple_result->success) {
        printf("   Output: %s\n", simple_result->output);
    }
    free_execution_result(simple_result);
    json_object_put(simple_params);
    
    // Example 2: Create an urgent task
    printf("\n2. Creating an urgent task:\n");
    json_object* urgent_params = json_object_new_object();
    json_object_object_add(urgent_params, "content", json_object_new_string("Fix critical bug in production"));
    ExecutionResult* urgent_result = execute_urgent_task(urgent_params);
    if (urgent_result && urgent_result->success) {
        printf("   Output: %s\n", urgent_result->output);
    }
    free_execution_result(urgent_result);
    json_object_put(urgent_params);
    
    // Example 3: Create a collaborative task
    printf("\n3. Creating a collaborative task:\n");
    json_object* collab_params = json_object_new_object();
    json_object_object_add(collab_params, "content", json_object_new_string("Design new user interface"));
    ExecutionResult* collab_result = execute_collab_task(collab_params);
    if (collab_result && collab_result->success) {
        printf("   Output: %s\n", collab_result->output);
    }
    free_execution_result(collab_result);
    json_object_put(collab_params);
    
    // Example 4: Complete a task
    printf("\n4. Completing a task:\n");
    json_object* complete_params = json_object_new_object();
    json_object_object_add(complete_params, "content", json_object_new_string("task_12345"));
    ExecutionResult* complete_result = execute_complete(complete_params);
    if (complete_result && complete_result->success) {
        printf("   Output: %s\n", complete_result->output);
    }
    free_execution_result(complete_result);
    json_object_put(complete_params);
    
    // Example 5: Get system stats
    printf("\n5. Retrieving system stats:\n");
    json_object* stats_params = json_object_new_object();
    ExecutionResult* stats_result = execute_stats(stats_params);
    if (stats_result && stats_result->success) {
        printf("   Output:\n%s\n", stats_result->output);
    }
    free_execution_result(stats_result);
    json_object_put(stats_params);
}

void demonstrate_search_operations() {
    printf("\n=== Todozi Search Examples ===\n\n");
    
    // Example 1: Smart search
    printf("1. Performing smart search:\n");
    json_object* search_params = json_object_new_object();
    json_object_object_add(search_params, "content", json_object_new_string("project deadlines"));
    ExecutionResult* search_result = execute_find(search_params);
    if (search_result && search_result->success) {
        printf("   Output: %s\n", search_result->output);
    }
    free_execution_result(search_result);
    json_object_put(search_params);
    
    // Example 2: AI semantic search
    printf("\n2. Performing AI semantic search:\n");
    json_object* ai_search_params = json_object_new_object();
    json_object_object_add(ai_search_params, "content", json_object_new_string("machine learning frameworks"));
    ExecutionResult* ai_search_result = execute_ai_search(ai_search_params);
    if (ai_search_result && ai_search_result->success) {
        printf("   Output:\n%s\n", ai_search_result->output);
    }
    free_execution_result(ai_search_result);
    json_object_put(ai_search_params);
}

void demonstrate_memory_operations() {
    printf("\n=== Todozi Memory & Idea Examples ===\n\n");
    
    // Example 1: Save a memory
    printf("1. Saving a memory:\n");
    json_object* memory_params = json_object_new_object();
    json_object_object_add(memory_params, "content", json_object_new_string("Client meeting notes"));
    json_object_object_add(memory_params, "extra", json_object_new_string("Discussed timeline changes"));
    ExecutionResult* memory_result = execute_remember(memory_params);
    if (memory_result && memory_result->success) {
        printf("   Output: %s\n", memory_result->output);
    }
    free_execution_result(memory_result);
    json_object_put(memory_params);
    
    // Example 2: Save an important memory
    printf("\n2. Saving an important memory:\n");
    json_object* important_params = json_object_new_object();
    json_object_object_add(important_params, "content", json_object_new_string("Security vulnerability details"));
    ExecutionResult* important_result = execute_important_memory(important_params);
    if (important_result && important_result->success) {
        printf("   Output: %s\n", important_result->output);
    }
    free_execution_result(important_result);
    json_object_put(important_params);
    
    // Example 3: Save an idea
    printf("\n3. Saving an idea:\n");
    json_object* idea_params = json_object_new_object();
    json_object_object_add(idea_params, "content", json_object_new_string("Implement dark mode toggle"));
    ExecutionResult* idea_result = execute_idea(idea_params);
    if (idea_result && idea_result->success) {
        printf("   Output: %s\n", idea_result->output);
    }
    free_execution_result(idea_result);
    json_object_put(idea_params);
}

int main() {
    // Initialize system
    ExecutorError* init_error = ensure_todozi_system();
    if (init_error) {
        printf("Failed to initialize Todozi system: %s\n", init_error->message);
        free_executor_error(init_error);
        return 1;
    }
    
    printf("Todozi Executor Example 5\n");
    printf("========================\n\n");
    
    demonstrate_task_operations();
    demonstrate_search_operations();
    demonstrate_memory_operations();
    
    // Cleanup
    cleanup_todozi_executor();
    
    return 0;
}
