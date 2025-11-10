#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "models.c"  // Include the provided models implementation

int main() {
    TodoziError err = {0};
    Task* task = NULL;
    TaskUpdate* update = NULL;
    Project* project = NULL;
    
    // Example 1: Create a basic task
    printf("=== Creating Basic Task ===\n");
    TodoziResult result = todozi_task_new(
        "user123",                           // user_id
        "Complete project documentation",    // action
        "2023-12-15 17:00:00",              // time
        PRIORITY_HIGH,                       // priority
        "project_alpha",                     // parent_project
        STATUS_TODO,                         // status
        &task,                               // output task
        &err                                 // error info
    );
    
    if (result != TODOZI_OK) {
        printf("Error creating task: %s\n", err.msg);
        free(err.msg);
        return 1;
    }
    
    printf("Created task: %s\n", todozi_task_action(task));
    printf("Task ID: %s\n", todozi_task_id(task));
    printf("Priority: %d\n", todozi_task_priority(task));
    
    // Example 2: Create a complex task with all fields
    printf("\n=== Creating Complex Task ===\n");
    char* tags[] = {"documentation", "urgent", "writing"};
    char* deps[] = {"task_12345678", "task_87654321"};
    uint8_t progress = 25;
    
    Task* complex_task = NULL;
    result = todozi_task_new_full(
        "user456",                           // user_id
        "Write API documentation",           // action
        "2023-12-20 10:00:00",              // time
        PRIORITY_CRITICAL,                   // priority
        "project_beta",                      // parent_project
        STATUS_IN_PROGRESS,                  // status
        ASSIGNEE_AGENT,                      // assignee_type
        "technical_writer_v1",               // agent_name
        tags, 3,                             // tags
        deps, 2,                             // dependencies
        "See RFC-123 for API specifications", // context_notes
        &progress,                           // progress
        &complex_task,                       // output task
        &err                                 // error info
    );
    
    if (result != TODOZI_OK) {
        printf("Error creating complex task: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task);
        return 1;
    }
    
    printf("Created complex task: %s\n", todozi_task_action(complex_task));
    printf("Assignee: agent:%s\n", todozi_task_assignee_agent_name(complex_task));
    printf("Progress: %d%%\n", *todozi_task_progress(complex_task));
    
    size_t tag_count;
    const char* const* task_tags = todozi_task_tags(complex_task, &tag_count);
    printf("Tags (%zu): ", tag_count);
    for (size_t i = 0; i < tag_count; i++) {
        printf("%s ", task_tags[i]);
    }
    printf("\n");
    
    // Example 3: Update a task
    printf("\n=== Updating Task ===\n");
    result = todozi_task_update_new(&update, &err);
    if (result != TODOZI_OK) {
        printf("Error creating update: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task);
        todozi_task_free(complex_task);
        return 1;
    }
    
    // Update multiple fields
    todozi_task_update_with_action(update, "Complete project documentation and submit", &err);
    todozi_task_update_with_status(update, STATUS_IN_PROGRESS, &err);
    todozi_task_update_with_progress(update, 50, &err);
    
    result = todozi_task_update(task, update, &err);
    if (result != TODOZI_OK) {
        printf("Error updating task: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task);
        todozi_task_free(complex_task);
        todozi_task_update_free(update);
        return 1;
    }
    
    printf("Updated task: %s\n", todozi_task_action(task));
    printf("New status: %d\n", todozi_task_status(task));
    printf("New progress: %d%%\n", *todozi_task_progress(task));
    
    // Example 4: Create and manage a project
    printf("\n=== Creating Project ===\n");
    result = todozi_project_new(
        "Website Redesign", 
        "Complete overhaul of company website", 
        &project, 
        &err
    );
    
    if (result != TODOZI_OK) {
        printf("Error creating project: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task);
        todozi_task_free(complex_task);
        todozi_task_update_free(update);
        return 1;
    }
    
    printf("Created project: %s\n", project->name);
    
    // Add tasks to project
    todozi_project_add_task(project, todozi_task_id(task));
    todozi_project_add_task(project, todozi_task_id(complex_task));
    
    printf("Added %zu tasks to project\n", project->tasks_count);
    
    // Example 5: Complete a task
    printf("\n=== Completing Task ===\n");
    printf("Task status before: %d\n", todozi_task_status(task));
    todozi_task_complete(task);
    printf("Task status after: %d\n", todozi_task_status(task));
    printf("Task progress: %d%%\n", *todozi_task_progress(task));
    
    // Cleanup
    todozi_task_free(task);
    todozi_task_free(complex_task);
    todozi_task_update_free(update);
    todozi_project_free(project);
    
    printf("\n=== All operations completed successfully ===\n");
    return 0;
}
