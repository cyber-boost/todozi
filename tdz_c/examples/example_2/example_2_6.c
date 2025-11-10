// example2.c - Practical example extending tui.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the main TUI implementation
#include "tui.c"

// Helper function to create a new task
Task* create_task(const char* id, const char* action, Priority priority, Status status, const char* project) {
    Task* task = calloc(1, sizeof(Task));
    if (!task) return NULL;
    
    task->id = string_clone(id);
    task->action = string_clone(action);
    task->priority = priority;
    task->status = status;
    task->parent_project = string_clone(project);
    task->created_at = time(NULL);
    task->updated_at = task->created_at;
    
    // Set assignee to human by default
    task->assignee = malloc(sizeof(Assignee));
    if (task->assignee) {
        *task->assignee = ASSIGNEE_HUMAN;
    }
    
    return task;
}

// Initialize sample data for demonstration
void init_sample_data(TodoziApp* app) {
    // Create sample projects
    app->projects_count = 3;
    app->projects = calloc(app->projects_count, sizeof(char*));
    app->projects[0] = string_clone("Work");
    app->projects[1] = string_clone("Personal");
    app->projects[2] = string_clone("Learning");
    
    // Create sample tasks
    app->tasks_count = 5;
    app->tasks = calloc(app->tasks_count, sizeof(Task));
    
    // Add tasks to projects
    app->tasks[0] = *create_task("001", "Complete project proposal", PRIORITY_HIGH, STATUS_IN_PROGRESS, "Work");
    app->tasks[1] = *create_task("002", "Buy groceries", PRIORITY_MEDIUM, STATUS_TODO, "Personal");
    app->tasks[2] = *create_task("003", "Read 30 pages of book", PRIORITY_LOW, STATUS_DONE, "Personal");
    app->tasks[3] = *create_task("004", "Learn C programming", PRIORITY_URGENT, STATUS_IN_PROGRESS, "Learning");
    app->tasks[4] = *create_task("005", "Fix critical bug", PRIORITY_CRITICAL, STATUS_BLOCKED, "Work");
    
    // Apply filters to populate filtered_tasks
    todozi_app_apply_filters(app);
}

// Enhanced key event handler for demonstration
void handle_demo_key_event(TodoziApp* app, int key_code) {
    switch (key_code) {
        case 'q':
        case 'Q':
            app->should_quit = 1;
            break;
        case '\t': // Tab
            todozi_app_next_tab(app);
            break;
        case 353: // Shift+Tab
            todozi_app_previous_tab(app);
            break;
        case '1': app->current_tab = APP_TAB_PROJECTS; break;
        case '2': app->current_tab = APP_TAB_TASKS; break;
        case '3': app->current_tab = APP_TAB_DONE; break;
        case '4': app->current_tab = APP_TAB_FIND; break;
        case '8': app->current_tab = APP_TAB_BYE; break;
        case '\n': // Enter
            todozi_app_handle_enter(app);
            break;
        default:
            break;
    }
}

// Simple input simulation for demonstration
void simulate_user_input(TodoziApp* app) {
    printf("\n=== Simulating User Interaction ===\n");
    
    // Navigate through tabs
    printf("Switching to Tasks tab...\n");
    handle_demo_key_event(app, '2');
    todozi_app_draw(app);
    
    printf("\nSwitching to Done tab...\n");
    handle_demo_key_event(app, '3');
    todozi_app_draw(app);
    
    printf("\nSwitching to Find tab...\n");
    handle_demo_key_event(app, '4');
    todozi_app_draw(app);
    
    printf("\nPerforming search for 'project'...\n");
    free(app->search_query);
    app->search_query = string_clone("project");
    todozi_app_update_search_results(app);
    todozi_app_draw(app);
    
    printf("\nSwitching to Exit tab...\n");
    handle_demo_key_event(app, '8');
    todozi_app_draw(app);
    
    printf("\nConfirming exit...\n");
    handle_demo_key_event(app, '\n');
}

int main() {
    printf("=== Todozi TUI Example 2 ===\n");
    printf("Demonstrating task management with sample data\n\n");
    
    // Create and initialize the app
    TodoziApp* app = todozi_app_new();
    if (!app) {
        fprintf(stderr, "Failed to create TodoziApp\n");
        return 1;
    }
    
    // Load sample data
    init_sample_data(app);
    
    // Run the demonstration
    simulate_user_input(app);
    
    // Clean up
    todozi_app_free(app);
    
    printf("\n=== Example Completed ===\n");
    return 0;
}
