#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "todozi_tool.c"  // Include the main implementation

int main() {
    // Initialize storage and shared Todozi system
    Storage storage = {0};
    SharedTodozi* todozi = shared_todozi_new(&storage);
    if (!todozi) {
        fprintf(stderr, "Failed to initialize Todozi system\n");
        return 1;
    }

    // Create task management tools
    Tool* create_task = create_task_tool_new(todozi);
    Tool* search_tasks = search_tasks_tool_new(todozi);
    Tool* update_task = update_task_tool_new(todozi);

    if (!create_task || !search_tasks || !update_task) {
        fprintf(stderr, "Failed to create tools\n");
        shared_todozi_free(todozi);
        return 1;
    }

    // Example 1: Create a new task
    printf("=== Creating a new task ===\n");
    HashMap* create_params = hashmap_new();
    hashmap_set(create_params, "action", "Implement user authentication system");
    hashmap_set(create_params, "time", "3 days");
    hashmap_set(create_params, "priority", "high");
    hashmap_set(create_params, "project", "Web Application");
    hashmap_set(create_params, "assignee", "ai");
    hashmap_set(create_params, "tags", "security,backend,authentication");

    ToolResult* create_result = create_task->execute(create_task, create_params);
    if (create_result) {
        printf("Create Task Result: %s\n", create_result->message);
        tool_result_free(create_result);
    }
    hashmap_free(create_params);

    // Example 2: Search for tasks
    printf("\n=== Searching for tasks ===\n");
    HashMap* search_params = hashmap_new();
    hashmap_set(search_params, "query", "authentication");
    hashmap_set(search_params, "semantic", "true");
    hashmap_set(search_params, "project", "Web Application");

    ToolResult* search_result = search_tasks->execute(search_tasks, search_params);
    if (search_result) {
        printf("Search Result: %s\n", search_result->message);
        tool_result_free(search_result);
    }
    hashmap_free(search_params);

    // Example 3: Update a task
    printf("\n=== Updating a task ===\n");
    HashMap* update_params = hashmap_new();
    hashmap_set(update_params, "task_id", "task-12345");
    hashmap_set(update_params, "status", "in_progress");
    hashmap_set(update_params, "progress", "50");
    hashmap_set(update_params, "context", "Completed database schema design");

    ToolResult* update_result = update_task->execute(update_task, update_params);
    if (update_result) {
        printf("Update Result: %s\n", update_result->message);
        tool_result_free(update_result);
    }
    hashmap_free(update_params);

    // Cleanup
    create_task->destroy(create_task);
    search_tasks->destroy(search_tasks);
    update_task->destroy(update_task);
    shared_todozi_free(todozi);

    return 0;
}
