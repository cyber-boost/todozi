#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "models.c"  // Include the main models file

int main() {
    // Error handling structure
    TodoziError error = {0};
    
    // Example 1: Create a simple task
    printf("=== Example 1: Creating a Simple Task ===\n");
    Task* task1 = NULL;
    TodoziResult result = todozi_task_new(
        "user123",                    // User ID
        "Complete project proposal",  // Action
        "2023-12-15 17:00",          // Due time
        PRIORITY_HIGH,               // Priority
        "Work Projects",             // Parent project
        STATUS_TODO,                 // Initial status
        &task1,                      // Output task
        &error                       // Error handling
    );
    
    if (result != TODOZI_OK) {
        printf("Error creating task: %s\n", error.msg);
        free(error.msg);
        return 1;
    }
    
    printf("Created task: %s\n", todozi_task_action(task1));
    printf("Task ID: %s\n", todozi_task_id(task1));
    printf("Priority: %d\n", todozi_task_priority(task1));
    
    // Example 2: Create a complex task with all features
    printf("\n=== Example 2: Creating a Complex Task ===\n");
    char* tags[] = {"urgent", "client-A", "documentation"};
    char* deps[] = {"task_dep1", "task_dep2"};
    
    Task* task2 = NULL;
    result = todozi_task_new_full(
        "user456",                          // User ID
        "Review client feedback",           // Action
        "2023-12-20 10:00",                // Due time
        PRIORITY_CRITICAL,                 // Priority
        "Client Management",               // Parent project
        STATUS_IN_PROGRESS,                // Status
        ASSIGNEE_AGENT,                    // Assignee type
        "SupportBot-v2",                   // Agent name
        tags, 3,                           // Tags
        deps, 2,                           // Dependencies
        "Needs to be completed before quarterly review", // Context notes
        (uint8_t[]){45},                   // Progress (45%)
        &task2,                            // Output task
        &error                             // Error handling
    );
    
    if (result != TODOZI_OK) {
        printf("Error creating complex task: %s\n", error.msg);
        free(error.msg);
        todozi_task_free(task1);
        return 1;
    }
    
    printf("Created complex task: %s\n", todozi_task_action(task2));
    printf("Assignee: ");
    char assignee_buf[128];
    todozi_assignee_to_string(
        todozi_task_assignee_type(task2),
        todozi_task_assignee_agent_name(task2),
        assignee_buf,
        sizeof(assignee_buf)
    );
    printf("%s\n", assignee_buf);
    
    // Show tags
    size_t tag_count;
    const char* const* task_tags = todozi_task_tags(task2, &tag_count);
    printf("Tags: ");
    for (size_t i = 0; i < tag_count; i++) {
        printf("%s ", task_tags[i]);
    }
    printf("\n");
    
    // Example 3: Update a task
    printf("\n=== Example 3: Updating a Task ===\n");
    TaskUpdate* update = NULL;
    result = todozi_task_update_new(&update, &error);
    if (result != TODOZI_OK) {
        printf("Error creating update: %s\n", error.msg);
        free(error.msg);
        todozi_task_free(task1);
        todozi_task_free(task2);
        return 1;
    }
    
    // Update various fields
    todozi_task_update_with_action(update, "Complete project proposal and send to client", &error);
    todozi_task_update_with_status(update, STATUS_IN_PROGRESS, &error);
    todozi_task_update_with_progress(update, 30, &error);  // 30% complete
    
    // Apply the update
    result = todozi_task_update(task1, update, &error);
    if (result != TODOZI_OK) {
        printf("Error updating task: %s\n", error.msg);
        free(error.msg);
    } else {
        printf("Updated task: %s\n", todozi_task_action(task1));
        printf("New status: %d\n", todozi_task_status(task1));
        if (todozi_task_progress(task1)) {
            printf("Progress: %d%%\n", *todozi_task_progress(task1));
        }
    }
    
    // Example 4: Complete a task
    printf("\n=== Example 4: Completing a Task ===\n");
    printf("Task status before completion: %d\n", todozi_task_status(task1));
    todozi_task_complete(task1);
    printf("Task status after completion: %d\n", todozi_task_status(task1));
    if (todozi_task_progress(task1)) {
        printf("Progress after completion: %d%%\n", *todozi_task_progress(task1));
    }
    
    // Example 5: Create and manage a project
    printf("\n=== Example 5: Project Management ===\n");
    Project* project = NULL;
    result = todozi_project_new("Website Redesign", "Redesign company website", &project, &error);
    if (result != TODOZI_OK) {
        printf("Error creating project: %s\n", error.msg);
        free(error.msg);
        todozi_task_free(task1);
        todozi_task_free(task2);
        todozi_task_update_free(update);
        return 1;
    }
    
    printf("Created project: %s\n", project->name);
    todozi_project_add_task(project, todozi_task_id(task1));
    todozi_project_add_task(project, todozi_task_id(task2));
    printf("Added %zu tasks to project\n", project->tasks_count);
    
    // Example 6: Configuration
    printf("\n=== Example 6: Configuration ===\n");
    Config* config = NULL;
    result = todozi_config_default(&config, &error);
    if (result != TODOZI_OK) {
        printf("Error creating config: %s\n", error.msg);
        free(error.msg);
    } else {
        printf("Default config version: %s\n", config->version);
        printf("Auto backup enabled: %s\n", config->auto_backup ? "yes" : "no");
        printf("Default assignee type: %d\n", *config->default_assignee_type);
    }
    
    // Example 7: Registration info
    printf("\n=== Example 7: Registration ===\n");
    RegistrationInfo* reg = NULL;
    result = todozi_registration_info_new_with_hashes(
        "https://api.todozi.com", 
        &reg, 
        &error
    );
    if (result != TODOZI_OK) {
        printf("Error creating registration: %s\n", error.msg);
        free(error.msg);
    } else {
        printf("Generated user: %s\n", reg->user_name);
        printf("Generated email: %s\n", reg->user_email);
    }
    
    // Cleanup
    todozi_task_free(task1);
    todozi_task_free(task2);
    todozi_task_update_free(update);
    todozi_project_free(project);
    todozi_config_free(config);
    todozi_registration_info_free(reg);
    
    printf("\n=== All examples completed successfully ===\n");
    return 0;
}
