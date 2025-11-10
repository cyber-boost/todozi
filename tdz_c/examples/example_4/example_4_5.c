// example4_custom_workflow.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cli.c"  // Include the main CLI implementation

// Custom workflow function that creates a project and adds multiple tasks
TodoziResult create_development_workflow(TodoziHandler* handler) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("🚀 Starting Development Project Setup\n");
    printf("=====================================\n\n");
    
    // Create a new project
    TodoziResult result = handle_project_create(handler, "WebApp-Dev", "Develop new web application");
    if (result != TODOZI_SUCCESS) return result;
    
    // Add multiple tasks to the project
    const char* tasks[][5] = {
        {"Design UI mockups", "4h", "high", "WebApp-Dev", "todo"},
        {"Set up development environment", "2h", "medium", "WebApp-Dev", "todo"},
        {"Implement user authentication", "8h", "critical", "WebApp-Dev", "todo"},
        {"Create database schema", "3h", "high", "WebApp-Dev", "todo"},
        {"Write API documentation", "2h", "medium", "WebApp-Dev", "todo"}
    };
    
    for (int i = 0; i < 5; i++) {
        result = handle_add_task(
            handler,
            tasks[i][0],  // action
            tasks[i][1],  // time
            tasks[i][2],  // priority
            tasks[i][3],  // project
            tasks[i][4],  // status
            NULL,         // assignee
            NULL,         // tags
            NULL,         // dependencies
            NULL,         // context
            -1            // progress
        );
        if (result != TODOZI_SUCCESS) return result;
    }
    
    printf("\n✅ Development workflow created successfully!\n");
    printf("📋 Project: WebApp-Dev\n");
    printf("📌 5 tasks added\n");
    return TODOZI_SUCCESS;
}

// Function to demonstrate task completion workflow
TodoziResult complete_task_workflow(TodoziHandler* handler) {
    printf("\n🔧 Completing 'Set up development environment' task\n");
    printf("====================================================\n");
    
    // In a real implementation, we'd search for the task ID first
    // For this example, we'll use a placeholder ID
    TodoziResult result = handle_update_task(
        handler,
        "task-002",           // task ID (placeholder)
        NULL,                 // action (unchanged)
        NULL,                 // time (unchanged)
        NULL,                 // priority (unchanged)
        NULL,                 // project (unchanged)
        "done",               // status (updated)
        NULL,                 // assignee (unchanged)
        NULL,                 // tags (unchanged)
        NULL,                 // dependencies (unchanged)
        NULL,                 // context (unchanged)
        100                   // progress (100%)
    );
    
    if (result == TODOZI_SUCCESS) {
        printf("✅ Task marked as complete!\n");
    }
    return result;
}

// Function to show project statistics
TodoziResult show_project_stats(TodoziHandler* handler) {
    printf("\n📊 Project Statistics\n");
    printf("=====================\n");
    return handle_stats(handler);
}

int main() {
    // Initialize storage and handler
    Storage* storage = storage_new();
    if (!storage) {
        printf("❌ Failed to initialize storage\n");
        return 1;
    }
    
    TodoziHandler* handler = todozi_handler_new(storage);
    if (!handler) {
        printf("❌ Failed to create handler\n");
        storage_free(storage);
        return 1;
    }
    
    printf("🔧 Todozi Custom Workflow Example\n");
    printf("==================================\n\n");
    
    // Execute custom workflows
    TodoziResult result = create_development_workflow(handler);
    if (result != TODOZI_SUCCESS) {
        printf("❌ Workflow creation failed\n");
        todozi_handler_free(handler);
        storage_free(storage);
        return 1;
    }
    
    result = complete_task_workflow(handler);
    if (result != TODOZI_SUCCESS) {
        printf("❌ Task completion failed\n");
        todozi_handler_free(handler);
        storage_free(storage);
        return 1;
    }
    
    show_project_stats(handler);
    
    // Cleanup
    todozi_handler_free(handler);
    storage_free(storage);
    
    printf("\n🎉 Custom workflow example completed!\n");
    return 0;
}
