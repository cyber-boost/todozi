#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// ... (include all the existing code from tui.c above) ...

// New function to create a sample task for demonstration
Task create_sample_task(const char* id, const char* action, Priority priority, Status status) {
    Task task = {0};
    task.id = string_clone(id);
    task.action = string_clone(action);
    task.priority = priority;
    task.status = status;
    task.created_at = time(NULL);
    task.updated_at = task.created_at;
    return task;
}

// Extended version of todozi_app_load_tasks with sample data
void todozi_app_load_tasks_extended(TodoziApp* app) {
    if (app == NULL) return;
    
    // Free existing tasks
    if (app->tasks != NULL) {
        for (int i = 0; i < app->tasks_count; i++) {
            task_free(&app->tasks[i]);
        }
        free(app->tasks);
    }
    
    // Create sample tasks
    app->tasks_count = 5;
    app->tasks = malloc(sizeof(Task) * app->tasks_count);
    
    if (app->tasks == NULL) {
        app->tasks_count = 0;
        return;
    }
    
    app->tasks[0] = create_sample_task("1", "Complete project proposal", PRIORITY_HIGH, STATUS_IN_PROGRESS);
    app->tasks[1] = create_sample_task("2", "Review team feedback", PRIORITY_MEDIUM, STATUS_TODO);
    app->tasks[2] = create_sample_task("3", "Update documentation", PRIORITY_LOW, STATUS_PENDING);
    app->tasks[3] = create_sample_task("4", "Fix critical bug", PRIORITY_CRITICAL, STATUS_BLOCKED);
    app->tasks[4] = create_sample_task("5", "Prepare presentation", PRIORITY_URGENT, STATUS_REVIEW);
    
    // Apply filters to populate filtered_tasks
    todozi_app_apply_filters(app);
}

// Enhanced draw function to show task details
void todozi_app_draw_tasks_tab_enhanced(TodoziApp* app) {
    if (app == NULL) return;
    
    printf("🔍 Filters\n");
    printf("📋 Tasks (%d)\n", app->filtered_tasks_count);
    
    // Show "Add New Task" option
    if (app->selected_task_index == 0) {
        printf("➤ ➕ Add New Task\n");
    } else {
        printf("  ➕ Add New Task\n");
    }
    
    // Show tasks with enhanced formatting
    if (app->filtered_tasks != NULL) {
        for (int i = 0; i < app->filtered_tasks_count; i++) {
            const char* status_icon;
            const char* priority_color;
            
            // Status icons
            switch (app->filtered_tasks[i].status) {
                case STATUS_TODO: case STATUS_PENDING: status_icon = "📝"; break;
                case STATUS_IN_PROGRESS: status_icon = "🔄"; break;
                case STATUS_BLOCKED: status_icon = "🚫"; break;
                case STATUS_REVIEW: status_icon = "👀"; break;
                case STATUS_DONE: case STATUS_COMPLETED: status_icon = "✅"; break;
                case STATUS_CANCELLED: status_icon = "❌"; break;
                case STATUS_DEFERRED: status_icon = "⏸️"; break;
                default: status_icon = "📝"; break;
            }
            
            // Priority colors (represented as text prefixes)
            switch (app->filtered_tasks[i].priority) {
                case PRIORITY_CRITICAL: priority_color = "[CRITICAL] "; break;
                case PRIORITY_URGENT: priority_color = "[URGENT] "; break;
                case PRIORITY_HIGH: priority_color = "[HIGH] "; break;
                case PRIORITY_MEDIUM: priority_color = "[MEDIUM] "; break;
                case PRIORITY_LOW: priority_color = "[LOW] "; break;
                default: priority_color = ""; break;
            }
            
            const char* action = app->filtered_tasks[i].action ? app->filtered_tasks[i].action : "(no action)";
            const char* project = app->filtered_tasks[i].parent_project ? app->filtered_tasks[i].parent_project : "(no project)";
            
            if (i + 1 == app->selected_task_index) {
                printf("➤ %s%s %s [%s]\n", status_icon, priority_color, action, project);
            } else {
                printf("  %s%s %s [%s]\n", status_icon, priority_color, action, project);
            }
        }
    }
    
    // Show task details when one is selected
    if (app->selected_task_index > 0 && app->selected_task_index <= app->filtered_tasks_count) {
        Task* selected = &app->filtered_tasks[app->selected_task_index - 1];
        printf("\n--- Task Details ---\n");
        printf("ID: %s\n", selected->id ? selected->id : "N/A");
        printf("Action: %s\n", selected->action ? selected->action : "N/A");
        printf("Status: %d\n", selected->status);
        printf("Priority: %d\n", selected->priority);
        printf("Created: %s", ctime(&selected->created_at));
    }
}

// Modified run function to use enhanced features
void todozi_app_run_extended(TodoziApp* app) {
    if (app == NULL) return;
    
    // Load sample data
    todozi_app_load_tasks_extended(app);
    
    // For demo purposes, we'll just draw once and exit
    todozi_app_draw_tabs(app);
    todozi_app_draw_tasks_tab_enhanced(app);
    todozi_app_draw_status_bar(app);
}

// Example usage in main
int main() {
    TodoziApp* app = todozi_app_new();
    if (app == NULL) {
        fprintf(stderr, "Failed to create TodoziApp\n");
        return 1;
    }
    
    printf("=== Todozi Extended Example ===\n");
    todozi_app_run_extended(app);
    printf("\n=== End of Example ===\n");
    
    todozi_app_free(app);
    return 0;
}
