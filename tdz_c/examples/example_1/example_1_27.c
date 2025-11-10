#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// External function declarations (from todozi_exe.c)
extern ExecutionResult* execute_todozi_tool_delegated(json_object* params);
extern void free_execution_result(ExecutionResult* result);

int main() {
    // Initialize JSON parameters for creating a task
    json_object* params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("task"));
    json_object_object_add(params, "content", json_object_new_string("Complete project documentation"));
    
    // Execute the task creation
    ExecutionResult* result = execute_todozi_tool_delegated(params);
    
    // Check and display results
    if (result && result->success) {
        printf("Success: %s\n", result->output);
        printf("Tool used: %s\n", result->tool_used);
    } else {
        printf("Failed to create task\n");
        if (result && result->error) {
            printf("Error: %s\n", result->error);
        }
    }
    
    // Cleanup
    free_execution_result(result);
    json_object_put(params);
    
    // Example 2: Create an urgent task
    params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("urgent"));
    json_object_object_add(params, "content", json_object_new_string("Fix critical bug in production"));
    
    result = execute_todozi_tool_delegated(params);
    
    if (result && result->success) {
        printf("\nUrgent task created: %s\n", result->output);
    }
    
    // Cleanup
    free_execution_result(result);
    json_object_put(params);
    
    return 0;
}
