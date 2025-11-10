// Example 5: Real-world Task Management Workflow
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Assuming all the structures and functions from tests.c are available

void example_task_workflow() {
    printf("\n=== Example 5: Real-world Task Management Workflow ===\n");
    
    // 1. Create a configuration
    Config* config = config_default();
    printf("Using configuration version: %s\n", config->version);
    printf("Default project: %s\n", config->default_project);
    printf("AI enabled: %s\n", config->ai_enabled ? "yes" : "no");
    
    // 2. Create projects
    Project* webProject = project_new("website-redesign", "Redesign company website");
    Project* mobileProject = project_new("mobile-app", "Develop new mobile application");
    
    printf("\nCreated projects:\n");
    printf("- %s (%s)\n", webProject->name, webProject->description);
    printf("- %s (%s)\n", mobileProject->name, mobileProject->description);
    
    // 3. Create tasks with various properties
    char* designTags[] = {"ui", "ux", "creative", NULL};
    char* devTags[] = {"backend", "api", "database", NULL};
    char* mobileDeps[] = {"task_001", NULL}; // Depends on design task
    
    Assignee aiAssignee = ASSIGNEE_AI;
    Assignee humanAssignee = ASSIGNEE_HUMAN;
    Assignee collabAssignee = ASSIGNEE_COLLABORATIVE;
    
    int progress0 = 0;
    int progress50 = 50;
    
    Task* designTask = task_new_full(
        "Create wireframes", "3 days", PRIORITY_HIGH, "website-redesign",
        STATUS_TODO, &collabAssignee, designTags, 3, NULL, 0,
        "Need to consider mobile responsiveness", &progress0
    );
    
    Task* backendTask = task_new_full(
        "Implement REST API", "5 days", PRIORITY_CRITICAL, "website-redesign",
        STATUS_IN_PROGRESS, &humanAssignee, devTags, 3, NULL, 0,
        "Follow OpenAPI specification", &progress50
    );
    
    Task* mobileTask = task_new_full(
        "Develop login screen", "2 days", PRIORITY_MEDIUM, "mobile-app",
        STATUS_TODO, &humanAssignee, NULL, 0, mobileDeps, 1,
        "Use company design system", NULL
    );
    
    printf("\nCreated tasks:\n");
    printf("1. %s (ID: %s) - %s\n", designTask->action, designTask->id, designTask->assignee);
    printf("2. %s (ID: %s) - %s (%d%% complete)\n", backendTask->action, backendTask->id, backendTask->assignee, *backendTask->progress);
    printf("3. %s (ID: %s) - %s (depends on %s)\n", mobileTask->action, mobileTask->id, mobileTask->assignee, mobileTask->dependencies[0]);
    
    // 4. Add tasks to projects
    project_add_task(webProject, designTask->id);
    project_add_task(webProject, backendTask->id);
    project_add_task(mobileProject, mobileTask->id);
    
    printf("\nProject task assignments:\n");
    printf("- %s has %d tasks\n", webProject->name, webProject->tasks_count);
    printf("- %s has %d tasks\n", mobileProject->name, mobileProject->tasks_count);
    
    // 5. Create task collection and add tasks
    TaskCollection* allTasks = task_collection_new();
    task_collection_add_task(allTasks, designTask);
    task_collection_add_task(allTasks, backendTask);
    task_collection_add_task(allTasks, mobileTask);
    
    // 6. Filter tasks by priority
    Priority highPriority = PRIORITY_HIGH;
    int highCount;
    Task** highPriorityTasks = task_collection_get_filtered_tasks(allTasks, &highPriority, NULL, NULL, &highCount);
    printf("\nHigh priority tasks (%d found):\n", highCount);
    for (int i = 0; i < highCount; i++) {
        printf("- %s\n", highPriorityTasks[i]->action);
    }
    free(highPriorityTasks);
    
    // 7. Filter tasks by project
    int webProjectCount;
    Task** webProjectTasks = task_collection_get_filtered_tasks(allTasks, NULL, "website-redesign", NULL, &webProjectCount);
    printf("\nTasks in website-redesign project (%d found):\n", webProjectCount);
    for (int i = 0; i < webProjectCount; i++) {
        printf("- %s\n", webProjectTasks[i]->action);
    }
    free(webProjectTasks);
    
    // 8. Update task progress
    int newProgress = 75;
    task_update(backendTask, NULL, NULL, NULL, &newProgress);
    printf("\nUpdated backend task progress to %d%%\n", *backendTask->progress);
    
    // 9. Complete a task
    task_complete(designTask);
    printf("Completed task: %s (Status: %s, Progress: %d%%)\n", 
           designTask->action, 
           designTask->status == STATUS_DONE ? "DONE" : "OTHER",
           *designTask->progress);
    
    // 10. Check active tasks
    printf("\nActive tasks:\n");
    int totalCount;
    Task** allTasksArray = task_collection_get_all_tasks(allTasks, &totalCount);
    for (int i = 0; i < totalCount; i++) {
        if (task_is_active(allTasksArray[i])) {
            printf("- %s\n", allTasksArray[i]->action);
        }
    }
    
    // 11. Archive completed project
    project_complete(webProject);
    printf("\nProject '%s' status: %s\n", 
           webProject->name,
           webProject->status == PROJECT_STATUS_COMPLETED ? "COMPLETED" : "ACTIVE");
    
    // Cleanup
    task_collection_free(allTasks);
    project_free(webProject);
    project_free(mobileProject);
    task_free(designTask);
    task_free(backendTask);
    task_free(mobileTask);
    config_free(config);
    
    printf("\nExample 5 completed successfully!\n");
}

// To run this example, you would call example_task_workflow() from main()
