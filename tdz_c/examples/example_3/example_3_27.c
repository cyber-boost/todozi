// example3_todozi_usage.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"
#include "todozi_exe.h" // Assuming the main file is compiled as a library

int main() {
    // Initialize JSON objects for different actions
    json_object *params_task = json_object_new_object();
    json_object *params_urgent = json_object_new_object();
    json_object *params_extract = json_object_new_object();
    json_object *params_expand = json_object_new_object();
    
    // Example 1: Create a simple task
    json_object_object_add(params_task, "action", json_object_new_string("task"));
    json_object_object_add(params_task, "content", json_object_new_string("Complete project documentation"));
    
    // Example 2: Create an urgent task
    json_object_object_add(params_urgent, "action", json_object_new_string("urgent"));
    json_object_object_add(params_urgent, "content", json_object_new_string("Fix critical bug in production"));
    
    // Example 3: Extract tasks from message
    json_object_object_add(params_extract, "action", json_object_new_string("extract"));
    json_object_object_add(params_extract, "content", json_object_new_string("Meeting with team to discuss Q3 goals and deliverables"));
    json_object_object_add(params_extract, "extra", json_object_new_string("Product roadmap planning"));
    
    // Example 4: Expand a task into subtasks
    json_object_object_add(params_expand, "action", json_object_new_string("expand"));
    json_object_object_add(params_expand, "content", json_object_new_string("Implement user authentication system"));
    json_object_object_add(params_expand, "extra", json_object_new_string("Use OAuth 2.0 with Google and GitHub providers"));
    
    // Execute operations
    printf("=== Todozi Executor Examples ===\n\n");
    
    ExecutionResult *result1 = execute_todozi_tool_delegated(params_task);
    if (result1 && result1->success) {
        printf("✅ Simple Task Creation:\n%s\n\n", result1->output);
    } else {
        printf("❌ Failed to create simple task\n\n");
    }
    free_execution_result(result1);
    
    ExecutionResult *result2 = execute_todozi_tool_delegated(params_urgent);
    if (result2 && result2->success) {
        printf("🚨 Urgent Task Creation:\n%s\n\n", result2->output);
    } else {
        printf("❌ Failed to create urgent task\n\n");
    }
    free_execution_result(result2);
    
    ExecutionResult *result3 = execute_todozi_tool_delegated(params_extract);
    if (result3 && result3->success) {
        printf("🔍 Task Extraction:\n%s\n\n", result3->output);
    } else {
        printf("❌ Failed to extract tasks\n\n");
    }
    free_execution_result(result3);
    
    ExecutionResult *result4 = execute_todozi_tool_delegated(params_expand);
    if (result4 && result4->success) {
        printf("🚀 Task Expansion:\n%s\n\n", result4->output);
    } else {
        printf("❌ Failed to expand task\n\n");
    }
    free_execution_result(result4);
    
    // Cleanup JSON objects
    json_object_put(params_task);
    json_object_put(params_urgent);
    json_object_put(params_extract);
    json_object_put(params_expand);
    
    // Cleanup executor
    cleanup_todozi_executor();
    
    return 0;
}
