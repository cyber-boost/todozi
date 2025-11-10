// example3_task_update.c
#include "types.c"  // Include the header with all type definitions
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Helper function to safely duplicate strings
char* safe_strdup(const char* str) {
    if (!str) return NULL;
    char* dup = malloc(strlen(str) + 1);
    if (dup) strcpy(dup, str);
    return dup;
}

// Function to initialize a TaskUpdate with optional fields
void init_task_update(TaskUpdate* update, const char* id) {
    update->id = safe_strdup(id);
    update->action = NULL; update->has_action = false;
    update->time = NULL; update->has_time = false;
    update->priority = NULL; update->has_priority = false;
    update->project = NULL; update->has_project = false;
    update->status = NULL; update->has_status = false;
    update->assignee = NULL; update->has_assignee = false;
    update->tags = NULL; update->has_tags = false;
    update->dependencies = NULL; update->has_dependencies = false;
    update->context = NULL; update->has_context = false;
    update->progress.value = 0; update->progress.present = false;
}

// Function to set optional string field
void set_optional_string(char** field, bool* has_field, const char* value) {
    if (value) {
        *field = safe_strdup(value);
        *has_field = true;
    } else {
        *field = NULL;
        *has_field = false;
    }
}

// Function to set optional progress value
void set_optional_progress(TaskUpdate* update, unsigned char value) {
    update->progress.value = value;
    update->progress.present = true;
}

// Function to simulate applying update to a task (printing for demo)
void apply_task_update(const TaskUpdate* update) {
    printf("Applying update to task ID: %s\n", update->id);
    if (update->has_action) printf("  Action: %s\n", update->action);
    if (update->has_time) printf("  Time: %s\n", update->time);
    if (update->has_priority) printf("  Priority: %s\n", update->priority);
    if (update->has_project) printf("  Project: %s\n", update->project);
    if (update->has_status) printf("  Status: %s\n", update->status);
    if (update->has_assignee) printf("  Assignee: %s\n", update->assignee);
    if (update->has_tags) printf("  Tags: %s\n", update->tags);
    if (update->has_dependencies) printf("  Dependencies: %s\n", update->dependencies);
    if (update->has_context) printf("  Context: %s\n", update->context);
    if (update->progress.present) printf("  Progress: %u%%\n", update->progress.value);
}

// Function to free TaskUpdate resources
void free_task_update(TaskUpdate* update) {
    free(update->id);
    if (update->has_action) free(update->action);
    if (update->has_time) free(update->time);
    if (update->has_priority) free(update->priority);
    if (update->has_project) free(update->project);
    if (update->has_status) free(update->status);
    if (update->has_assignee) free(update->assignee);
    if (update->has_tags) free(update->tags);
    if (update->has_dependencies) free(update->dependencies);
    if (update->has_context) free(update->context);
    memset(update, 0, sizeof(TaskUpdate));
}

int main() {
    TaskUpdate update;
    
    // Initialize update for task "task-123"
    init_task_update(&update, "task-123");
    
    // Set some optional fields
    set_optional_string(&update.action, &update.has_action, "Review documentation");
    set_optional_string(&update.priority, &update.has_priority, "High");
    set_optional_string(&update.status, &update.has_status, "In Progress");
    set_optional_progress(&update, 75);
    
    // Apply the update (in real app, this would modify actual task)
    apply_task_update(&update);
    
    // Clean up
    free_task_update(&update);
    
    return 0;
}
