#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.c" // Include the provided library

int main() {
    // Initialize the Todozi system
    TodoziErrorCode err = todozi_init();
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to initialize Todozi: %d\n", err);
        return 1;
    }

    // Create tasks with different priorities
    Task* task1 = NULL;
    err = todozi_create_task(
        "Complete project proposal", 
        TODOZI_PRIORITY_CRITICAL, 
        "Work", 
        "Tomorrow", 
        "Client deadline approaching", 
        &task1
    );
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to create task 1: %d\n", err);
        return 1;
    }

    Task* task2 = NULL;
    err = todozi_create_task(
        "Buy groceries", 
        TODOZI_PRIORITY_MEDIUM, 
        "Personal", 
        "This weekend", 
        "Milk and eggs needed", 
        &task2
    );
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to create task 2: %d\n", err);
        return 1;
    }

    Task* task3 = NULL;
    err = todozi_create_task(
        "Schedule dentist appointment", 
        TODOZI_PRIORITY_LOW, 
        "Health", 
        "Next month", 
        "Regular checkup", 
        &task3
    );
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to create task 3: %d\n", err);
        return 1;
    }

    // Display created tasks
    printf("Created Tasks:\n");
    printf("1. %s (Priority: Critical)\n", task1->action);
    printf("2. %s (Priority: Medium)\n", task2->action);
    printf("3. %s (Priority: Low)\n", task3->action);

    // Search for tasks containing "project"
    Task** search_results = NULL;
    size_t results_count = 0;
    err = todozi_search_tasks("project", false, 10, &search_results, &results_count);
    if (err != TODOZI_OK) {
        fprintf(stderr, "Search failed: %d\n", err);
        return 1;
    }

    printf("\nSearch Results for 'project':\n");
    for (size_t i = 0; i < results_count; i++) {
        printf("- %s\n", search_results[i]->action);
    }
    free_task_array(search_results, results_count);

    // Mark task as complete
    err = todozi_complete_task(task1->id);
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to complete task: %d\n", err);
        return 1;
    }
    printf("\nMarked '%s' as complete\n", task1->action);

    // List all tasks
    Task** all_tasks = NULL;
    size_t all_tasks_count = 0;
    err = todozi_list_tasks(&all_tasks, &all_tasks_count);
    if (err != TODOZI_OK) {
        fprintf(stderr, "Failed to list tasks: %d\n", err);
        return 1;
    }

    printf("\nAll Tasks:\n");
    for (size_t i = 0; i < all_tasks_count; i++) {
        const char* status_str = "Unknown";
        switch (all_tasks[i]->status) {
            case TODOZI_STATUS_TODO: status_str = "To Do"; break;
            case TODOZI_STATUS_IN_PROGRESS: status_str = "In Progress"; break;
            case TODOZI_STATUS_DONE: status_str = "Done"; break;
            case TODOZI_STATUS_BLOCKED: status_str = "Blocked"; break;
        }
        printf("- %s [%s]\n", all_tasks[i]->action, status_str);
    }
    free_task_array(all_tasks, all_tasks_count);

    // Clean up individual tasks
    free_task(task1);
    free_task(task2);
    free_task(task3);

    return 0;
}
