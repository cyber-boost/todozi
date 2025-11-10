#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "models.c"  // Include the provided models implementation

void print_task_details(const Task* task) {
    char priority_str[32], status_str[32], assignee_str[64];
    size_t tags_count, deps_count;
    
    todozi_priority_to_string(todozi_task_priority(task), priority_str, sizeof(priority_str));
    todozi_status_to_string(todozi_task_status(task), status_str, sizeof(status_str));
    todozi_assignee_to_string(
        todozi_task_assignee_type(task),
        todozi_task_assignee_agent_name(task),
        assignee_str, sizeof(assignee_str)
    );
    
    printf("Task ID: %s\n", todozi_task_id(task));
    printf("Action: %s\n", todozi_task_action(task));
    printf("Priority: %s\n", priority_str);
    printf("Status: %s\n", status_str);
    printf("Assignee: %s\n", assignee_str);
    
    const char* const* tags = todozi_task_tags(task, &tags_count);
    if (tags_count > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < tags_count; i++) {
            printf("%s%s", tags[i], (i < tags_count - 1) ? ", " : "\n");
        }
    }
    
    const char* const* deps = todozi_task_dependencies(task, &deps_count);
    if (deps_count > 0) {
        printf("Dependencies: ");
        for (size_t i = 0; i < deps_count; i++) {
            printf("%s%s", deps[i], (i < deps_count - 1) ? ", " : "\n");
        }
    }
    
    const uint8_t* progress = todozi_task_progress(task);
    if (progress) {
        printf("Progress: %d%%\n", *progress);
    }
    printf("\n");
}

int main() {
    TodoziError err = {0};
    Task* task = NULL;
    Project* project = NULL;
    TaskUpdate* update = NULL;
    
    // Create a new project
    if (todozi_project_new("Website Redesign", "Complete overhaul of company website", &project, &err) != TODOZI_OK) {
        printf("Error creating project: %s\n", err.msg);
        goto cleanup;
    }
    printf("Created project: %s\n\n", project->name);
    
    // Create a new task with basic information
    if (todozi_task_new(
        "user_12345", 
        "Design homepage mockup", 
        "2023-12-01T10:00:00Z", 
        PRIORITY_HIGH, 
        todozi_project_new, 
        STATUS_TODO, 
        &task, 
        &err
    ) != TODOZI_OK) {
        printf("Error creating task: %s\n", err.msg);
        goto cleanup;
    }
    
    printf("Created basic task:\n");
    print_task_details(task);
    
    // Add task to project
    todozi_project_add_task(project, todozi_task_id(task));
    
    // Create a more complex task with all fields
    char* tags[] = {"design", "frontend", "urgent"};
    char* deps[] = {todozi_task_id(task)};  // Depends on previous task
    
    Task* task2 = NULL;
    if (todozi_task_new_full(
        "user_12345",
        "Implement homepage UI",
        "2023-12-05T09:00:00Z",
        PRIORITY_CRITICAL,
        todozi_project_new,
        STATUS_PENDING,
        ASSIGNEE_AGENT,
        "frontend-bot-v2",
        tags, 3,
        deps, 1,
        "Use new design system components",
        NULL,  // No progress yet
        &task2,
        &err
    ) != TODOZI_OK) {
        printf("Error creating full task: %s\n", err.msg);
        goto cleanup;
    }
    
    printf("Created complex task:\n");
    print_task_details(task2);
    
    // Add second task to project
    todozi_project_add_task(project, todozi_task_id(task2));
    
    // Update the first task
    if (todozi_task_update_new(&update, &err) != TODOZI_OK) {
        printf("Error creating update object: %s\n", err.msg);
        goto cleanup;
    }
    
    // Update status and add progress
    Status new_status = STATUS_IN_PROGRESS;
    uint8_t progress = 45;
    todozi_task_update_with_status(update, new_status, NULL);
    todozi_task_update_with_progress(update, progress, NULL);
    todozi_task_update_with_action(update, "Design homepage mockup (revised)", NULL);
    
    if (todozi_task_update(task, update, &err) != TODOZI_OK) {
        printf("Error updating task: %s\n", err.msg);
        goto cleanup;
    }
    
    printf("Updated first task:\n");
    print_task_details(task);
    
    // Complete the task
    todozi_task_complete(task);
    printf("After completing task:\n");
    print_task_details(task);
    
    // Show project details
    printf("Project '%s' contains %zu tasks\n", project->name, project->tasks_count);
    printf("Project status: ");
    char project_status_str[32];
    todozi_project_status_to_string(project->status, project_status_str, sizeof(project_status_str));
    printf("%s\n", project_status_str);
    
cleanup:
    // Clean up resources
    todozi_task_free(task);
    todozi_task_free(task2);
    todozi_project_free(project);
    todozi_task_update_free(update);
    free(err.msg);
    
    return 0;
}
