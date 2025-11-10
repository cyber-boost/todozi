#include <stdio.h>
#include <stdlib.h>
#include "json-c/json.h"
#include "todozi_exe.c"  // Include the main implementation

int main() {
    // Initialize JSON objects for different task types
    json_object* task_params = json_object_new_object();
    json_object* urgent_params = json_object_new_object();
    json_object* ai_params = json_object_new_object();
    json_object* collab_params = json_object_new_object();
    
    // Create a regular task
    json_object_object_add(task_params, "action", json_object_new_string("task"));
    json_object_object_add(task_params, "content", json_object_new_string("Complete project documentation"));
    
    // Create an urgent priority task
    json_object_object_add(urgent_params, "action", json_object_new_string("urgent"));
    json_object_object_add(urgent_params, "content", json_object_new_string("Fix critical bug in production"));
    
    // Create an AI-assigned task
    json_object_object_add(ai_params, "action", json_object_new_string("ai"));
    json_object_object_add(ai_params, "content", json_object_new_string("Analyze user engagement metrics"));
    
    // Create a collaborative task
    json_object_object_add(collab_params, "action", json_object_new_string("collab"));
    json_object_object_add(collab_params, "content", json_object_new_string("Review design specifications with team"));
    
    // Execute different task creation operations
    ExecutionResult* task_result = execute_todozi_tool_delegated(task_params);
    ExecutionResult* urgent_result = execute_todozi_tool_delegated(urgent_params);
    ExecutionResult* ai_result = execute_todozi_tool_delegated(ai_params);
    ExecutionResult* collab_result = execute_todozi_tool_delegated(collab_params);
    
    // Print results
    printf("=== Task Creation Results ===\n");
    if (task_result && task_result->success) {
        printf("Regular Task: %s\n", task_result->output);
    }
    
    if (urgent_result && urgent_result->success) {
        printf("Urgent Task: %s\n", urgent_result->output);
    }
    
    if (ai_result && ai_result->success) {
        printf("AI Task: %s\n", ai_result->output);
    }
    
    if (collab_result && collab_result->success) {
        printf("Collab Task: %s\n", collab_result->output);
    }
    
    // Clean up resources
    free_execution_result(task_result);
    free_execution_result(urgent_result);
    free_execution_result(ai_result);
    free_execution_result(collab_result);
    
    json_object_put(task_params);
    json_object_put(urgent_params);
    json_object_put(ai_params);
    json_object_put(collab_params);
    
    cleanup_todozi_executor();
    return 0;
}
