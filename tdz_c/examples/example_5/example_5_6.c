#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the provided tui.c code here (omitted for brevity)
// The full tui.c implementation should be compiled with this example

// Helper function to create a sample task
Task create_sample_task(const char* id, const char* action, Priority priority, Status status, const char* project) {
    Task task = {0};
    task.id = string_clone(id);
    task.action = string_clone(action);
    task.priority = priority;
    task.status = status;
    task.parent_project = string_clone(project);
    task.created_at = time(NULL);
    task.updated_at = task.created_at;
    return task;
}

// Function to populate the app with sample data
void populate_sample_data(TodoziApp* app) {
    // Create sample projects
    app->projects_count = 3;
    app->projects = malloc(sizeof(char*) * app->projects_count);
    app->projects[0] = string_clone("Work");
    app->projects[1] = string_clone("Personal");
    app->projects[2] = string_clone("Learning");

    // Create sample tasks
    app->tasks_count = 5;
    app->tasks = malloc(sizeof(Task) * app->tasks_count);
    
    app->tasks[0] = create_sample_task("001", "Complete project proposal", PRIORITY_HIGH, STATUS_IN_PROGRESS, "Work");
    app->tasks[1] = create_sample_task("002", "Buy groceries", PRIORITY_MEDIUM, STATUS_TODO, "Personal");
    app->tasks[2] = create_sample_task("003", "Learn C programming", PRIORITY_LOW, STATUS_PENDING, "Learning");
    app->tasks[3] = create_sample_task("004", "Team meeting", PRIORITY_URGENT, STATUS_COMPLETED, "Work");
    app->tasks[4] = create_sample_task("005", "Gym workout", PRIORITY_LOW, STATUS_DONE, "Personal");

    // Apply filters to populate filtered_tasks
    todozi_app_apply_filters(app);
}

int main() {
    TodoziApp* app = todozi_app_new();
    if (app == NULL) {
        fprintf(stderr, "Failed to create TodoziApp\n");
        return 1;
    }

    // Populate with sample data
    populate_sample_data(app);

    // Run the application
    todozi_app_run(app);

    // Cleanup
    todozi_app_free(app);
    return 0;
}
