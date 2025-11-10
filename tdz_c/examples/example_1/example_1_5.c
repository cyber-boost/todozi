#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Assuming the structures and functions from cli.c are available
// This example shows how to create and manage tasks programmatically

int main() {
    // Initialize storage and handler
    Storage* storage = storage_new();
    if (!storage) {
        printf("Failed to create storage\n");
        return 1;
    }

    TodoziHandler* handler = todozi_handler_new(storage);
    if (!handler) {
        printf("Failed to create handler\n");
        storage_free(storage);
        return 1;
    }

    // Example 1: Create a new task
    printf("=== Creating a New Task ===\n");
    handle_add_task(
        handler,
        "Implement user authentication",  // action
        "4 hours",                        // time estimate
        "high",                           // priority
        "web-app",                        // project
        "todo",                           // status
        "human",                          // assignee
        "auth,security",                  // tags
        NULL,                             // dependencies
        "Use OAuth 2.0 protocol",         // context
        -1                                // progress
    );

    // Example 2: List all tasks
    printf("\n=== Listing All Tasks ===\n");
    handle_list_tasks(handler, NULL, NULL, NULL, NULL, NULL, NULL);

    // Example 3: Show specific task
    printf("\n=== Showing Task Details ===\n");
    handle_show_task(handler, "task-id-placeholder");

    // Example 4: Update task progress
    printf("\n=== Updating Task ===\n");
    handle_update_task(
        handler,
        "task-id-placeholder",            // task ID
        NULL,                             // action (unchanged)
        NULL,                             // time (unchanged)
        NULL,                             // priority (unchanged)
        NULL,                             // project (unchanged)
        "in-progress",                    // new status
        NULL,                             // assignee (unchanged)
        NULL,                             // tags (unchanged)
        NULL,                             // dependencies (unchanged)
        NULL,                             // context (unchanged)
        25                                // progress: 25%
    );

    // Example 5: Complete task
    printf("\n=== Completing Task ===\n");
    todozi_handler_complete_task(handler, "task-id-placeholder");

    // Example 6: Check statistics
    printf("\n=== System Statistics ===\n");
    handle_stats(handler);

    // Example 7: Create a project
    printf("\n=== Creating Project ===\n");
    handle_project_create(handler, "mobile-app", "Mobile application development");

    // Example 8: List projects
    printf("\n=== Listing Projects ===\n");
    handle_project_list(handler);

    // Example 9: Add task to project
    printf("\n=== Adding Task to Project ===\n");
    handle_add_task(
        handler,
        "Design login screen",           // action
        "2 hours",                       // time estimate
        "medium",                        // priority
        "mobile-app",                    // project
        "todo",                          // status
        "human",                         // assignee
        "ui,design",                     // tags
        NULL,                            // dependencies
        "Follow Material Design guidelines", // context
        -1                               // progress
    );

    // Example 10: Search tasks
    printf("\n=== Searching Tasks ===\n");
    handle_search_tasks(handler, "login");

    // Cleanup
    todozi_handler_free(handler);
    storage_free(storage);

    return 0;
}
