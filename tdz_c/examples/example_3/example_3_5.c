#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cli.c" // Include the provided CLI implementation

int main() {
    // Initialize storage and handler
    Storage* storage = storage_new();
    if (!storage) {
        fprintf(stderr, "Failed to initialize storage\n");
        return 1;
    }

    TodoziHandler* handler = todozi_handler_new(storage);
    if (!handler) {
        fprintf(stderr, "Failed to create handler\n");
        storage_free(storage);
        return 1;
    }

    printf("=== Todozi Task Management Demo ===\n\n");

    // 1. Create a new project
    printf("1. Creating a new project...\n");
    handle_project_create(handler, "ai-research", "AI research and development");

    // 2. Add tasks to the project
    printf("\n2. Adding tasks...\n");
    handle_add_task(handler, 
                    "Implement neural network model", 
                    "8 hours", 
                    "high", 
                    "ai-research", 
                    "todo", 
                    "human", 
                    "ml,research", 
                    NULL, 
                    "Focus on transformer architecture", 
                    -1);

    handle_add_task(handler, 
                    "Review literature on attention mechanisms", 
                    "4 hours", 
                    "medium", 
                    "ai-research", 
                    "in-progress", 
                    "ai", 
                    "research,literature", 
                    "task-1", 
                    "Check recent papers from 2023", 
                    50);

    // 3. List all tasks
    printf("\n3. Listing all tasks...\n");
    handle_list_tasks(handler, NULL, NULL, NULL, NULL, NULL, NULL);

    // 4. Show a specific task
    printf("\n4. Showing task details...\n");
    handle_show_task(handler, "task-2");

    // 5. Update a task
    printf("\n5. Updating task progress...\n");
    handle_update_task(handler, 
                       "task-2", 
                       NULL, 
                       NULL, 
                       NULL, 
                       NULL, 
                       NULL, 
                       NULL, 
                       NULL, 
                       NULL, 
                       NULL, 
                       75); // Update progress to 75%

    // 6. Complete a task
    printf("\n6. Completing a task...\n");
    todozi_handler_complete_task(handler, "task-1");

    // 7. Check final task list
    printf("\n7. Final task list...\n");
    handle_list_tasks(handler, "ai-research", NULL, NULL, NULL, NULL, NULL);

    // Cleanup
    todozi_handler_free(handler);
    storage_free(storage);

    printf("\n=== Demo completed successfully ===\n");
    return 0;
}
