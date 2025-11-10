// Example 3: Priority Filter Extension
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Add this function to implement priority filtering
void todozi_app_apply_priority_filter(TodoziApp* app, Priority priority) {
    if (app == NULL) return;
    
    // Free existing filtered tasks
    if (app->filtered_tasks != NULL) {
        for (int i = 0; i < app->filtered_tasks_count; i++) {
            task_free(&app->filtered_tasks[i]);
        }
        free(app->filtered_tasks);
    }
    app->filtered_tasks_count = 0;
    app->filtered_tasks = NULL;
    
    if (app->tasks == NULL || app->tasks_count == 0) {
        return;
    }
    
    // Count matching tasks
    int match_count = 0;
    for (int i = 0; i < app->tasks_count; i++) {
        if (app->tasks[i].priority == priority) {
            match_count++;
        }
    }
    
    if (match_count == 0) {
        return;
    }
    
    // Allocate and populate filtered tasks
    app->filtered_tasks = malloc(sizeof(Task) * match_count);
    if (app->filtered_tasks == NULL) {
        app->filtered_tasks_count = 0;
        return;
    }
    
    app->filtered_tasks_count = 0;
    for (int i = 0; i < app->tasks_count; i++) {
        if (app->tasks[i].priority == priority) {
            app->filtered_tasks[app->filtered_tasks_count] = task_clone(&app->tasks[i]);
            app->filtered_tasks_count++;
        }
    }
}

// Enhanced key handler for priority filtering
void todozi_app_handle_priority_filter_key(TodoziApp* app, int key_code) {
    if (app == NULL || app->current_tab != APP_TAB_TASKS) return;
    
    switch (key_code) {
        case '1': // Critical priority filter
            todozi_app_apply_priority_filter(app, PRIORITY_CRITICAL);
            break;
        case '2': // Urgent priority filter
            todozi_app_apply_priority_filter(app, PRIORITY_URGENT);
            break;
        case '3': // High priority filter
            todozi_app_apply_priority_filter(app, PRIORITY_HIGH);
            break;
        case '4': // Medium priority filter
            todozi_app_apply_priority_filter(app, PRIORITY_MEDIUM);
            break;
        case '5': // Low priority filter
            todozi_app_apply_priority_filter(app, PRIORITY_LOW);
            break;
        case '0': // Show all tasks
            todozi_app_apply_filters(app);
            break;
    }
}

// Enhanced status bar for Tasks tab
void todozi_app_draw_tasks_status_bar(TodoziApp* app) {
    printf("Tasks: %d | 1-5: Priority Filter | 0: Show All | ↑↓ Navigate | Enter Edit\n", 
           app->filtered_tasks_count);
}

// Example usage in main application loop
void enhanced_todozi_app_run(TodoziApp* app) {
    if (app == NULL) return;
    
    todozi_app_load_tasks(app);
    
    while (!app->should_quit) {
        todozi_app_draw(app);
        
        // Simulate key input handling
        int key = getchar(); // In real app, use proper input handling
        todozi_app_handle_key_event(app, key);
        todozi_app_handle_priority_filter_key(app, key);
        
        // Exit after one iteration for demo
        app->should_quit = 1;
    }
}

// Sample task creation helper
Task create_sample_task(const char* action, Priority priority, Status status) {
    Task task = {0};
    task.id = string_clone("1");
    task.action = string_clone(action);
    task.priority = priority;
    task.status = status;
    task.created_at = time(NULL);
    task.updated_at = time(NULL);
    return task;
}

// Enhanced main for demonstration
int main() {
    TodoziApp* app = todozi_app_new();
    if (app == NULL) {
        fprintf(stderr, "Failed to create TodoziApp\n");
        return 1;
    }
    
    // Create sample tasks
    app->tasks_count = 3;
    app->tasks = malloc(sizeof(Task) * app->tasks_count);
    app->tasks[0] = create_sample_task("Fix critical bug", PRIORITY_CRITICAL, STATUS_TODO);
    app->tasks[1] = create_sample_task("Update documentation", PRIORITY_MEDIUM, STATUS_IN_PROGRESS);
    app->tasks[2] = create_sample_task("Plan team outing", PRIORITY_LOW, STATUS_PENDING);
    
    // Apply initial filtering
    todozi_app_apply_filters(app);
    
    // Run enhanced application
    enhanced_todozi_app_run(app);
    
    todozi_app_free(app);
    return 0;
}
