#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tests.c"  // Include the main implementation

void example_project_workflow() {
    printf("\n=== Example 2: Project Workflow with Dependencies ===\n");
    
    // Create a software development project
    Project* project = project_new("web-app", "Build a new web application");
    printf("Created project: %s\n", project->name);
    
    // Create configuration
    Config* config = config_default();
    printf("Using default assignee: ");
    switch(*config->default_assignee) {
        case ASSIGNEE_AI: printf("AI\n"); break;
        case ASSIGNEE_HUMAN: printf("Human\n"); break;
        case ASSIGNEE_COLLABORATIVE: printf("Collaborative\n"); break;
    }
    
    // Create task collection
    TaskCollection* tasks = task_collection_new();
    
    // Create tasks with dependencies
    char* design_tags[] = {"frontend", "ui", NULL};
    int design_progress = 0;
    Assignee human = ASSIGNEE_HUMAN;
    Task* design_task = task_new_full(
        "Create UI wireframes", 
        "8 hours", 
        PRIORITY_HIGH, 
        "web-app", 
        STATUS_TODO, 
        &human, 
        design_tags, 
        2, 
        NULL,  // No dependencies
        0, 
        "Need to align with marketing team", 
        &design_progress
    );
    
    char* backend_tags[] = {"backend", "api", NULL};
    char* backend_deps[] = {design_task->id, NULL};  // Depends on design
    int backend_progress = 0;
    Assignee collaborative = ASSIGNEE_COLLABORATIVE;
    Task* backend_task = task_new_full(
        "Implement REST API", 
        "16 hours", 
        PRIORITY_CRITICAL, 
        "web-app", 
        STATUS_TODO, 
        &collaborative, 
        backend_tags, 
        2, 
        backend_deps, 
        1, 
        "Follow OpenAPI specification", 
        &backend_progress
    );
    
    char* frontend_tags[] = {"frontend", "react", NULL};
    char* frontend_deps[] = {design_task->id, NULL};  // Depends on design
    int frontend_progress = 0;
    Task* frontend_task = task_new_full(
        "Develop frontend components", 
        "20 hours", 
        PRIORITY_HIGH, 
        "web-app", 
        STATUS_TODO, 
        &human, 
        frontend_tags, 
        2, 
        frontend_deps, 
        1, 
        "Use React with TypeScript", 
        &frontend_progress
    );
    
    char* testing_tags[] = {"testing", "qa", NULL};
    char* testing_deps[] = {backend_task->id, frontend_task->id, NULL};  // Depends on both
    int testing_progress = 0;
    Assignee ai = ASSIGNEE_AI;
    Task* testing_task = task_new_full(
        "Automated testing", 
        "6 hours", 
        PRIORITY_MEDIUM, 
        "web-app", 
        STATUS_TODO, 
        &ai, 
        testing_tags, 
        2, 
        testing_deps, 
        2, 
        "Run unit and integration tests", 
        &testing_progress
    );
    
    // Add tasks to collection and project
    task_collection_add_task(tasks, design_task);
    task_collection_add_task(tasks, backend_task);
    task_collection_add_task(tasks, frontend_task);
    task_collection_add_task(tasks, testing_task);
    
    project_add_task(project, design_task->id);
    project_add_task(project, backend_task->id);
    project_add_task(project, frontend_task->id);
    project_add_task(project, testing_task->id);
    
    printf("Created %d tasks:\n", tasks->tasks_count);
    for (int i = 0; i < tasks->tasks_count; i++) {
        Task* t = tasks->tasks[i];
        printf("  - %s (%s) [Assignee: %s]\n", 
               t->action, 
               t->id, 
               t->assignee ? t->assignee : "none");
    }
    
    // Simulate work progress
    printf("\nSimulating work progress...\n");
    
    // Design task 50% complete
    int progress_50 = 50;
    task_update(design_task, NULL, NULL, NULL, &progress_50);
    printf("Design task progress: %d%%\n", *design_task->progress);
    
    // Backend task can start now (design is in progress)
    Status in_progress = STATUS_IN_PROGRESS;
    task_update(backend_task, NULL, NULL, &in_progress, NULL);
    printf("Backend task status: in progress\n");
    
    // Frontend task can also start
    task_update(frontend_task, NULL, NULL, &in_progress, NULL);
    printf("Frontend task status: in progress\n");
    
    // Complete design task
    task_complete(design_task);
    printf("Design task completed: %s\n", task_is_completed(design_task) ? "Yes" : "No");
    
    // Update backend progress to 75%
    int progress_75 = 75;
    task_update(backend_task, NULL, NULL, NULL, &progress_75);
    printf("Backend task progress: %d%%\n", *backend_task->progress);
    
    // Complete frontend task
    task_complete(frontend_task);
    printf("Frontend task completed: %s\n", task_is_completed(frontend_task) ? "Yes" : "No");
    
    // Complete backend task
    task_complete(backend_task);
    printf("Backend task completed: %s\n", task_is_completed(backend_task) ? "Yes" : "No");
    
    // Testing can now start
    task_update(testing_task, NULL, NULL, &in_progress, NULL);
    printf("Testing task status: in progress\n");
    
    // Complete testing
    task_complete(testing_task);
    printf("Testing task completed: %s\n", task_is_completed(testing_task) ? "Yes" : "No");
    
    // Show final project status
    printf("\nFinal project status:\n");
    printf("Project: %s\n", project->name);
    printf("Tasks completed: %d/%d\n", 
           task_is_completed(design_task) + 
           task_is_completed(backend_task) + 
           task_is_completed(frontend_task) + 
           task_is_completed(testing_task),
           4);
    
    // Filter tasks by assignee
    printf("\nTasks assigned to humans:\n");
    for (int i = 0; i < tasks->tasks_count; i++) {
        Task* t = tasks->tasks[i];
        if (t->assignee && strcmp(t->assignee, "human") == 0) {
            printf("  - %s (%s)\n", t->action, t->id);
        }
    }
    
    // Cleanup
    task_collection_free(tasks);
    project_free(project);
    config_free(config);
    
    printf("\nExample 2 completed successfully!\n");
}

int main() {
    example_project_workflow();
    return 0;
}
