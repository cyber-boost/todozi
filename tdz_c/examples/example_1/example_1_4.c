#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.c"  // Include the library

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
        "Complete project documentation",  // Action
        TODOZI_PRIORITY_HIGH,              // Priority
        "Work Projects",                   // Project
        "2023-12-31",                      // Due date
        "Documentation for client deliverable", // Context
        &task
    );

    if (result != TODOZI_OK || task == NULL) {
        printf("Failed to create task: %d\n", result);
        return 1;
    }

    printf("Created task with ID: %s\n", task->id);
    printf("Task action: %s\n", task->action);
    printf("Task priority: %d\n", task->priority);
    printf("Task project: %s\n", task->parent_project);
    printf("Task status: %d\n", task->status);

    // Create a memory (completed task)
    Task* memory_task = NULL;
    result = todozi_create_memory(
        "Project kickoff meeting",         // Moment
        "Established project requirements", // Meaning
        "Meeting with client on Jan 15",   // Reason
        &memory_task
    );

    if (result == TODOZI_OK && memory_task != NULL) {
        printf("\nCreated memory: %s\n", memory_task->action);
        free_task(memory_task);
    }

    // Create an idea
    Task* idea_task = NULL;
    result = todozi_create_idea(
        "Implement machine learning for data analysis", // Idea
        "Could improve processing speed",               // Context
        &idea_task
    );

    if (result == TODOZI_OK && idea_task != NULL) {
        printf("\nCreated idea: %s\n", idea_task->action);
        free_task(idea_task);
    }

    // Search for tasks
    Task** found_tasks = NULL;
    size_t tasks_count = 0;
    result = todozi_search_tasks("documentation", false, 10, &found_tasks, &tasks_count);
    
    if (result == TODOZI_OK) {
        printf("\nFound %zu tasks matching 'documentation'\n", tasks_count);
        for (size_t i = 0; i < tasks_count; i++) {
            printf("- %s (ID: %s)\n", found_tasks[i]->action, found_tasks[i]->id);
        }
        free_task_array(found_tasks, tasks_count);
    }

    // Update task status
    result = todozi_update_task_status(task->id, TODOZI_STATUS_IN_PROGRESS);
    if (result == TODOZI_OK) {
        printf("\nTask status updated to in progress\n");
    }

    // List all tasks
    Task** all_tasks = NULL;
    size_t all_tasks_count = 0;
    result = todozi_list_tasks(&all_tasks, &all_tasks_count);
    
    if (result == TODOZI_OK) {
        printf("\nAll tasks (%zu total):\n", all_tasks_count);
        for (size_t i = 0; i < all_tasks_count; i++) {
            printf("- %s [%s]\n", all_tasks[i]->action, 
                   all_tasks[i]->status == TODOZI_STATUS_TODO ? "TODO" :
                   all_tasks[i]->status == TODOZI_STATUS_IN_PROGRESS ? "IN PROGRESS" :
                   all_tasks[i]->status == TODOZI_STATUS_DONE ? "DONE" : "BLOCKED");
        }
        free_task_array(all_tasks, all_tasks_count);
    }

    // Clean up
    free_task(task);
    
    return 0;
}
