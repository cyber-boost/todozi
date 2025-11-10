// example5.c - Advanced Task Management with Filters and Updates
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.c" // Include the main library

int main() {
    // Initialize Todozi system
    TodoziErrorCode err = todozi_init();
    if (err != TODOZI_OK) {
        printf("Failed to initialize Todozi: %d\n", err);
        return 1;
    }

    // Create sample tasks
    Task* task1;
    Task* task2;
    Task* task3;
    
    todozi_create_task(
        "Implement user authentication", 
        TODOZI_PRIORITY_HIGH, 
        "WebApp Project", 
        "2023-12-01", 
        "Use OAuth 2.0", 
        &task1
    );
    
    todozi_create_task(
        "Design database schema", 
        TODOZI_PRIORITY_CRITICAL, 
        "WebApp Project", 
        "2023-11-15", 
        "ERD required", 
        &task2
    );
    
    todozi_create_task(
        "Write API documentation", 
        TODOZI_PRIORITY_MEDIUM, 
        "WebApp Project", 
        "2023-12-10", 
        "Use Swagger", 
        &task3
    );

    printf("Created tasks:\n");
    printf("1. %s (ID: %s)\n", task1->action, task1->id);
    printf("2. %s (ID: %s)\n", task2->action, task2->id);
    printf("3. %s (ID: %s)\n", task3->action, task3->id);

    // Create filters for high-priority tasks in WebApp Project
    TaskFilters* filters;
    err = todozi_create_task_filters(
        "WebApp Project",  // project
        NULL,              // status
        "high",            // priority
        NULL,              // assignee
        NULL,              // tags
        NULL,              // search
        &filters
    );
    
    if (err != TODOZI_OK) {
        printf("Failed to create filters: %d\n", err);
        return 1;
    }

    // Search with filters
    Task** filtered_tasks;
    size_t filtered_count;
    err = todozi_search_with_filters(filters, 10, &filtered_tasks, &filtered_count);
    
    if (err == TODOZI_OK) {
        printf("\nHigh-priority tasks in WebApp Project:\n");
        for (size_t i = 0; i < filtered_count; i++) {
            printf("- %s\n", filtered_tasks[i]->action);
        }
        free_task_array(filtered_tasks, filtered_count);
    } else {
        printf("Search failed: %d\n", err);
    }

    // Update task status
    printf("\nUpdating task status...\n");
    err = todozi_update_task_status(task1->id, TODOZI_STATUS_IN_PROGRESS);
    if (err == TODOZI_OK) {
        printf("Task '%s' status updated to IN_PROGRESS\n", task1->action);
    } else {
        printf("Failed to update task status: %d\n", err);
    }

    // Create update object for task details
    TaskUpdate* update;
    err = todozi_create_task_update(
        "Implement user authentication with JWT", // new action
        "critical",                               // new priority
        "in_progress",                            // new status
        "WebApp Project v2",                      // new project
        &update
    );
    
    if (err == TODOZI_OK) {
        err = todozi_update_task_full(task2->id, update);
        if (err == TODOZI_OK) {
            printf("Task '%s' updated successfully\n", task2->action);
        } else {
            printf("Failed to update task: %d\n", err);
        }
        free_task_update(update);
    } else {
        printf("Failed to create update object: %d\n", err);
    }

    // List all tasks after updates
    Task** all_tasks;
    size_t all_count;
    err = todozi_all_tasks(&all_tasks, &all_count);
    
    if (err == TODOZI_OK) {
        printf("\nAll tasks after updates:\n");
        for (size_t i = 0; i < all_count; i++) {
            printf("- %s (Priority: %d, Status: %d, Project: %s)\n",
                   all_tasks[i]->action,
                   all_tasks[i]->priority,
                   all_tasks[i]->status,
                   all_tasks[i]->parent_project);
        }
        free_task_array(all_tasks, all_count);
    } else {
        printf("Failed to list tasks: %d\n", err);
    }

    // Cleanup
    free_task(task1);
    free_task(task2);
    free_task(task3);
    free_task_filters(filters);

    return 0;
}
