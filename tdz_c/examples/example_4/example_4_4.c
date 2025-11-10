#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.c" // Include the provided library

int main() {
    // Initialize Todozi system
    TodoziErrorCode err = todozi_init();
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to initialize Todozi: %d\n", err);
        return 1;
    }

    // Create tasks with different priorities
    Task* critical_task = NULL;
    err = todozi_create_task(
        "Fix critical security vulnerability",
        TODOZI_PRIORITY_CRITICAL,
        "Security Project",
        "Immediately",
        "Patch CVE-2023-12345 in authentication module",
        &critical_task
    );
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to create critical task: %d\n", err);
        return 1;
    }

    Task* urgent_task = NULL;
    err = todozi_create_task(
        "Prepare quarterly report",
        TODOZI_PRIORITY_URGENT,
        "Finance",
        "End of week",
        "Include revenue and expense summaries",
        &urgent_task
    );
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to create urgent task: %d\n", err);
        return 1;
    }

    Task* regular_task = NULL;
    err = todozi_create_task(
        "Update documentation",
        TODOZI_PRIORITY_MEDIUM,
        "Documentation",
        "Next month",
        "Add new API endpoints to user guide",
        &regular_task
    );
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to create regular task: %d\n", err);
        return 1;
    }

    // Display created tasks
    printf("Created Tasks:\n");
    printf("1. [%s] %s\n", 
           critical_task->priority == TODOZI_PRIORITY_CRITICAL ? "CRITICAL" : "OTHER",
           critical_task->action);
    printf("2. [%s] %s\n", 
           urgent_task->priority == TODOZI_PRIORITY_URGENT ? "URGENT" : "OTHER",
           urgent_task->action);
    printf("3. [%s] %s\n", 
           regular_task->priority == TODOZI_PRIORITY_MEDIUM ? "MEDIUM" : "OTHER",
           regular_task->action);

    // Start working on the critical task
    err = todozi_start_task(critical_task->id);
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to start critical task: %d\n", err);
        return 1;
    }
    printf("\nStarted working on: %s\n", critical_task->action);

    // Complete the urgent task
    err = todozi_complete_task(urgent_task->id);
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to complete urgent task: %d\n", err);
        return 1;
    }
    printf("Completed task: %s\n", urgent_task->action);

    // List all tasks
    Task** all_tasks = NULL;
    size_t tasks_count = 0;
    err = todozi_all_tasks(&all_tasks, &tasks_count);
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to list tasks: %d\n", err);
        return 1;
    }

    printf("\nAll Tasks:\n");
    for (size_t i = 0; i < tasks_count; i++) {
        const char* status_str = "";
        switch (all_tasks[i]->status) {
            case TODOZI_STATUS_TODO: status_str = "TODO"; break;
            case TODOZI_STATUS_IN_PROGRESS: status_str = "IN PROGRESS"; break;
            case TODOZI_STATUS_DONE: status_str = "DONE"; break;
            case TODOZI_STATUS_BLOCKED: status_str = "BLOCKED"; break;
        }
        printf("- [%s] %s\n", status_str, all_tasks[i]->action);
    }

    // Cleanup
    free_task(critical_task);
    free_task(urgent_task);
    free_task(regular_task);
    free_task_array(all_tasks, tasks_count);

    return 0;
}
