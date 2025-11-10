#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "todozi_tool.c" // Include the main implementation

// Helper function to demonstrate tool usage
void demonstrate_tool_usage(Tool* tool, const char* tool_name) {
    printf("\n=== Demonstrating %s ===\n", tool_name);
    
    ToolDefinition* def = tool->definition(tool);
    if (!def) {
        printf("Failed to get tool definition\n");
        return;
    }
    
    printf("Tool: %s\n", def->name);
    printf("Description: %s\n", def->description);
    printf("Category: %s\n", def->category);
    printf("Parameters (%zu):\n", def->parameters_count);
    
    for (size_t i = 0; i < def->parameters_count; i++) {
        printf("  - %s (%s): %s [%s]\n",
               def->parameters[i].name,
               def->parameters[i].type,
               def->parameters[i].description,
               def->parameters[i].required ? "required" : "optional");
    }
    
    tool_definition_free(def);
}

// Helper function to execute a tool with parameters
ToolResult* execute_tool_with_params(Tool* tool, ...) {
    HashMap* kwargs = hashmap_new();
    if (!kwargs) return NULL;
    
    va_list args;
    va_start(args, tool);
    
    char* key;
    while ((key = va_arg(args, char*)) != NULL) {
        char* value = va_arg(args, char*);
        hashmap_set(kwargs, key, value);
    }
    
    va_end(args);
    
    ToolResult* result = tool->execute(tool, kwargs);
    hashmap_free(kwargs);
    return result;
}

int main() {
    // Initialize system
    Storage storage = {0};
    SharedTodozi* todozi = shared_todozi_new(&storage);
    if (!todozi) {
        printf("Failed to initialize Todozi system\n");
        return 1;
    }
    
    // Create tools
    Tool* create_task = create_task_tool_new(todozi);
    Tool* search_tasks = search_tasks_tool_new(todozi);
    Tool* update_task = update_task_tool_new(todozi);
    
    if (!create_task || !search_tasks || !update_task) {
        printf("Failed to create tools\n");
        shared_todozi_free(todozi);
        return 1;
    }
    
    // Demonstrate tool definitions
    demonstrate_tool_usage(create_task, "Create Task Tool");
    demonstrate_tool_usage(search_tasks, "Search Tasks Tool");
    demonstrate_tool_usage(update_task, "Update Task Tool");
    
    // Create sample tasks
    printf("\n=== Creating Sample Tasks ===\n");
    
    ToolResult* result1 = execute_tool_with_params(
        create_task,
        "action", "Implement user authentication",
        "priority", "high",
        "assignee", "human",
        "project", "WebApp Development",
        "tags", "security,backend,authentication",
        NULL
    );
    
    if (result1) {
        printf("Result: %s\n", result1->message);
        tool_result_free(result1);
    }
    
    ToolResult* result2 = execute_tool_with_params(
        create_task,
        "action", "Design UI mockups",
        "priority", "medium",
        "assignee", "ai",
        "project", "WebApp Development",
        "tags", "frontend,design,ui",
        NULL
    );
    
    if (result2) {
        printf("Result: %s\n", result2->message);
        tool_result_free(result2);
    }
    
    ToolResult* result3 = execute_tool_with_params(
        create_task,
        "action", "Setup CI/CD pipeline",
        "priority", "critical",
        "assignee", "collaborative",
        "project", "DevOps",
        "tags", "automation,deployment,ci/cd",
        NULL
    );
    
    if (result3) {
        printf("Result: %s\n", result3->message);
        tool_result_free(result3);
    }
    
    // Search tasks
    printf("\n=== Searching Tasks ===\n");
    
    ToolResult* search_result = execute_tool_with_params(
        search_tasks,
        "query", "authentication",
        "semantic", "false",
        "project", "WebApp Development",
        NULL
    );
    
    if (search_result) {
        printf("Search Result: %s\n", search_result->message);
        tool_result_free(search_result);
    }
    
    // Update a task
    printf("\n=== Updating Task ===\n");
    
    ToolResult* update_result = execute_tool_with_params(
        update_task,
        "task_id", "task-12345",
        "status", "in_progress",
        "progress", "50",
        NULL
    );
    
    if (update_result) {
        printf("Update Result: %s\n", update_result->message);
        tool_result_free(update_result);
    }
    
    // Cleanup
    create_task->destroy(create_task);
    search_tasks->destroy(search_tasks);
    update_task->destroy(update_task);
    shared_todozi_free(todozi);
    
    printf("\n=== Demo Completed Successfully ===\n");
    return 0;
}
