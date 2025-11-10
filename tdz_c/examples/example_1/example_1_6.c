#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the provided code here (tui.c content)
// ... (all the provided code from tui.c)

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
    
    return task;
}

// Function to add sample tasks to the app
void add_sample_tasks(TodoziApp* app) {
    // Create sample projects
    app->projects_count = 3;
    app->projects = malloc(sizeof(char*) * app->projects_count);
    app->projects[0] = string_clone("Work");
    app->projects[1] = string_clone("Personal");
    app->projects[2] = string_clone("Learning");
    
    // Create sample tasks
    app->tasks_count = 5;
    app->tasks = malloc(sizeof(Task) * app->tasks_count);
    
    app->tasks[0] = *create_task("1", "Complete project proposal", PRIORITY_HIGH, STATUS_IN_PROGRESS, "Work");
    app->tasks[1] = *create_task("2", "Buy groceries", PRIORITY_MEDIUM, STATUS_TODO, "Personal");
    app->tasks[2] = *create_task("3", "Learn C programming", PRIORITY_LOW, STATUS_PENDING, "Learning");
    app->tasks[3] = *create_task("4", "Prepare presentation", PRIORITY_URGENT, STATUS_DONE, "Work");
    app->tasks[4] = *create_task("5", "Call dentist", PRIORITY_CRITICAL, STATUS_BLOCKED, "Personal");
    
    // Apply filters to populate filtered_tasks
    todozi_app_apply_filters(app);
}

// Enhanced run function with basic interaction simulation
void enhanced_todozi_run(TodoziApp* app) {
    if (app == NULL) return;
    
    add_sample_tasks(app);
    
    int steps = 0;
    while (!app->should_quit && steps < 10) {  // Limit steps for demo
        printf("\033[2J\033[H"); // Clear screen (Unix/Linux/Mac)
        todozi_app_draw(app);
        
        // Simulate user interactions
        switch(steps) {
            case 2:
                printf("\n\nSwitching to Tasks tab...\n");
                app->current_tab = APP_TAB_TASKS;
                break;
            case 4:
                printf("\n\nSwitching to Done tab...\n");
                app->current_tab = APP_TAB_DONE;
                break;
            case 6:
                printf("\n\nSwitching to Find tab...\n");
                app->current_tab = APP_TAB_FIND;
                free(app->search_query);
                app->search_query = string_clone("project");
                todozi_app_update_search_results(app);
                break;
            case 8:
                printf("\n\nSwitching to Exit tab...\n");
                app->current_tab = APP_TAB_BYE;
                break;
        }
        
        steps++;
        #ifdef _WIN32
            Sleep(2000); // Windows
        #else
            sleep(2); // Unix/Linux/Mac
        #endif
    }
}

int main() {
    TodoziApp* app = todozi_app_new();
    if (app == NULL) {
        fprintf(stderr, "Failed to create TodoziApp\n");
        return 1;
    }
    
    printf("Todozi TUI Demo\n");
    printf("==============\n");
    enhanced_todozi_run(app);
    todozi_app_free(app);
    
    printf("\nDemo completed. Thank you for using Todozi!\n");
    return 0;
}
