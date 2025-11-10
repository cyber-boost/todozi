#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "models.h"

int main() {
    // Initialize error handling
    TodoziError err = {0};
    
    // Create a new task with dependencies
    Task* task1 = NULL;
    char* tags1[] = {"urgent", "frontend"};
    char* deps1[] = {"task_a1b2c3d4"}; // Reference to another task
    
    TodoziResult result = todozi_task_new_full(
        "user123",                           // user_id
        "Implement login page",             // action
        "2023-12-01T10:00:00Z",            // time_str
        PRIORITY_HIGH,                      // priority
        "Web Application",                  // parent_project
        STATUS_TODO,                        // status
        ASSIGNEE_HUMAN,                     // assignee_type
        NULL,                               // assignee_agent_name
        tags1, 2,                           // tags
        deps1, 1,                           // dependencies
        "Needs to integrate with backend auth service", // context_notes
        NULL,                               // progress (initially NULL)
        &task1,                             // output task
        &err                                // error info
    );
    
    if (result != TODOZI_OK) {
        printf("Error creating task: %s\n", err.msg);
        free(err.msg);
        return 1;
    }
    
    printf("Created task: %s\n", todozi_task_action(task1));
    printf("Task ID: %s\n", todozi_task_id(task1));
    
    // Show initial dependencies
    size_t dep_count;
    const char* const* dependencies = todozi_task_dependencies(task1, &dep_count);
    printf("Dependencies (%zu): ", dep_count);
    for (size_t i = 0; i < dep_count; i++) {
        printf("%s ", dependencies[i]);
    }
    printf("\n");
    
    // Update task progress
    TaskUpdate* update = NULL;
    result = todozi_task_update_new(&update, &err);
    if (result != TODOZI_OK) {
        printf("Error creating update: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task1);
        return 1;
    }
    
    // Mark 50% progress
    result = todozi_task_update_with_progress(update, 50, &err);
    if (result != TODOZI_OK) {
        printf("Error setting progress: %s\n", err.msg);
        free(err.msg);
        todozi_task_update_free(update);
        todozi_task_free(task1);
        return 1;
    }
    
    // Add more context
    result = todozi_task_update_with_context_notes(
        update, 
        "Halfway done - UI completed, starting integration", 
        &err
    );
    if (result != TODOZI_OK) {
        printf("Error updating context: %s\n", err.msg);
        free(err.msg);
        todozi_task_update_free(update);
        todozi_task_free(task1);
        return 1;
    }
    
    // Apply updates
    result = todozi_task_update(task1, update, &err);
    if (result != TODOZI_OK) {
        printf("Error applying updates: %s\n", err.msg);
        free(err.msg);
        todozi_task_update_free(update);
        todozi_task_free(task1);
        return 1;
    }
    
    printf("Updated task progress to %d%%\n", *todozi_task_progress(task1));
    printf("New context: %s\n", todozi_task_context_notes(task1));
    
    // Complete the task
    todozi_task_complete(task1);
    printf("Task completed: %s\n", todozi_task_is_completed(task1) ? "Yes" : "No");
    printf("Final progress: %d%%\n", *todozi_task_progress(task1));
    
    // Cleanup
    todozi_task_update_free(update);
    todozi_task_free(task1);
    
    return 0;
}
