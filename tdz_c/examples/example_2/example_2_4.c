// example2.c - Advanced Task Management with Todozi
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.c"  // Include the main library

int main() {
    // Initialize the Todozi system
    TodoziErrorCode result = todozi_init();
    if (result != TODOZI_OK) {
        printf("Failed to initialize Todozi: %d\n", result);
        return 1;
    }

    // Create a new task
    Task* task = NULL;
    result = todozi_create_task(
        "Implement user authentication",  // action
        TODOZI_PRIORITY_HIGH,             // priority
        "Web Application",                // project
        "2023-12-31",                     // time
        "Need to support OAuth 2.0",      // context
        &task
    );
    
    if (result != TODOZI_OK || !task) {
        printf("Failed to create task: %d\n", result);
        return 1;
    }
    
    printf("Created task: %s\n", task->action);
    printf("Task ID: %s\n", task->id);
    printf("Priority: %d\n", task->priority);
    
    // Start working on the task
    result = todozi_start_task(task->id);
    if (result == TODOZI_OK) {
        printf("Task started successfully\n");
    }
    
    // Update task progress
    TaskUpdate* update = todozi_create_update();
    if (update) {
        int progress = 50;
        update->progress = &progress;
        result = todozi_update_task_full(task->id, update);
        if (result == TODOZI_OK) {
            printf("Task progress updated to %d%%\n", progress);
        }
        free_task_update(update);
    }
    
    // Create a filter to find high priority tasks
    TaskFilters* filters = todozi_create_filters();
    if (filters) {
        TodoziPriority high_priority = TODOZI_PRIORITY_HIGH;
        filters->priority = &high_priority;
        filters->project = strdup("Web Application");
        
        Task** filtered_tasks = NULL;
        size_t filtered_count = 0;
        result = todozi_search_with_filters(filters, 10, &filtered_tasks, &filtered_count);
        if (result == TODOZI_OK) {
            printf("Found %zu high priority tasks\n", filtered_count);
            for (size_t i = 0; i < filtered_count; i++) {
                printf("  - %s\n", filtered_tasks[i]->action);
            }
            free_task_array(filtered_tasks, filtered_count);
        }
        free_task_filters(filters);
    }
    
    // Complete the task
    result = todozi_complete_task(task->id);
    if (result == TODOZI_OK) {
        printf("Task completed successfully\n");
    }
    
    // Create a memory from the completed task
    Task* memory_task = NULL;
    result = todozi_create_memory(
        "User authentication implementation",
        "Used OAuth 2.0 with Google and GitHub providers",
        "Security requirements specified in RFC 6749",
        &memory_task
    );
    
    if (result == TODOZI_OK && memory_task) {
        printf("Created memory: %s\n", memory_task->action);
        free_task(memory_task);
    }
    
    // Create an idea for future improvement
    Task* idea_task = NULL;
    result = todozi_create_idea(
        "Add biometric authentication",
        "Consider fingerprint and face recognition for mobile users",
        &idea_task
    );
    
    if (result == TODOZI_OK && idea_task) {
        printf("Created idea: %s\n", idea_task->action);
        free_task(idea_task);
    }
    
    // Clean up
    free_task(task);
    
    printf("Example completed successfully\n");
    return 0;
}
