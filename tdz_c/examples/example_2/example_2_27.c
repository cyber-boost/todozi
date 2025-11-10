#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "json-c/json.h"

// Assuming todozi_exe.c functions are available
extern ExecutionResult* execute_todozi_tool_delegated(json_object* params);
extern void free_execution_result(ExecutionResult* result);

int main() {
    // Initialize JSON parameters for creating a task
    json_object* params = json_object_new_object();
    json_object_object_add(params, "action", json_object_new_string("task"));
    json_object_object_add(params, "content", json_object_new_string("Complete project documentation"));
    
    // Execute the task creation
    ExecutionResult* result = execute_todozi_tool_delegated(params);
    
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
    
    // Example 2: Create an urgent task with additional context
    json_object* urgent_params = json_object_new_object();
    json_object_object_add(urgent_params, "action", json_object_new_string("urgent"));
    json_object_object_add(urgent_params, "content", json_object_new_string("Fix critical bug in production"));
    json_object_object_add(urgent_params, "extra", json_object_new_string("Affects user login functionality"));
    
    ExecutionResult* urgent_result = execute_todozi_tool_delegated(urgent_params);
    
    if (urgent_result && urgent_result->success) {
        printf("\nUrgent task created:\n%s\n", urgent_result->output);
    }
    
    // Cleanup
    free_execution_result(urgent_result);
    json_object_put(urgent_params);
    
    return 0;
}
