// Example 4: Real-world Task Management Scenario
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Assuming all the structures and functions from tests.c are available

void example_real_world_scenario() {
    printf("\n=== Example 4: Real-world Task Management Scenario ===\n");
    
    // 1. Create configuration
    Config* config = config_default();
    printf("Using configuration version: %s\n", config->version);
    printf("Default project: %s\n", config->default_project);
    printf("AI enabled: %s\n", config->ai_enabled ? "yes" : "no");
    
    // 2. Create projects
    Project* webProject = project_new("website-redesign", "Complete overhaul of company website");
    Project* marketingProject = project_new("q4-campaign", "Quarterly marketing campaign");
    
    printf("\nCreated projects:\n");
    printf("- %s (%s)\n", webProject->name, webProject->description);
    printf("- %s (%s)\n", marketingProject->name, marketingProject->description);
    
    // 3. Create task collection
    TaskCollection* allTasks = task_collection_new();
    
    // 4. Create various tasks with different properties
    char* designTags[] = {"design", "ui", "frontend", NULL};
    char* devTags[] = {"development", "backend", "api", NULL};
    char* marketingTags[] = {"content", "social-media", NULL};
    
    // Web project tasks
    Assignee aiAssignee = ASSIGNEE_AI;
    Assignee humanAssignee = ASSIGNEE_HUMAN;
    Assignee collabAssignee = ASSIGNEE_COLLABORATIVE;
    
    Task* designTask = task_new_full(
        "Create new homepage design", 
        "8 hours", 
        PRIORITY_HIGH, 
        "website-redesign", 
        STATUS_TODO, 
        &collabAssignee, 
        designTags, 3, 
        NULL, 0, 
        "Need to match brand guidelines", 
        NULL
    );
    
    Task* apiTask = task_new_full(
        "Implement user authentication API", 
        "12 hours", 
        PRIORITY_CRITICAL, 
        "website-redesign", 
        STATUS_IN_PROGRESS, 
        &humanAssignee, 
        devTags, 3, 
        NULL, 0, 
        "Follow OAuth 2.0 standards", 
        NULL
    );
    
    // Marketing project tasks
    Task* contentTask = task_new_full(
        "Write social media posts", 
        "4 hours", 
        PRIORITY_MEDIUM, 
        "q4-campaign", 
        STATUS_TODO, 
        &humanAssignee, 
        marketingTags, 2, 
        NULL, 0, 
        "Focus on holiday themes", 
        NULL
    );
    
    Task* analyticsTask = task_new_full(
        "Set up campaign analytics", 
        "3 hours", 
        PRIORITY_HIGH, 
        "q4-campaign", 
        STATUS_BLOCKED, 
        &aiAssignee, 
        NULL, 0, 
        NULL, 0, 
        "Waiting for access credentials", 
        NULL
    );
    
    // 5. Add tasks to collection and projects
    task_collection_add_task(allTasks, designTask);
    task_collection_add_task(allTasks, apiTask);
    task_collection_add_task(allTasks, contentTask);
    task_collection_add_task(allTasks, analyticsTask);
    
    project_add_task(webProject, designTask->id);
    project_add_task(webProject, apiTask->id);
    project_add_task(marketingProject, contentTask->id);
    project_add_task(marketingProject, analyticsTask->id);
    
    printf("\nCreated %d tasks:\n", allTasks->tasks_count);
    for (int i = 0; i < allTasks->tasks_count; i++) {
        Task* task = allTasks->tasks[i];
        printf("- [%s] %s (Project: %s, Assignee: %s)\n", 
               task->id, task->action, task->parent_project, 
               task->assignee ? task->assignee : "unassigned");
    }
    
    // 6. Demonstrate task filtering
    printf("\nFiltering tasks:\n");
    
    // High priority tasks
    Priority highPriority = PRIORITY_HIGH;
    int highCount;
    Task** highTasks = task_collection_get_filtered_tasks(allTasks, &highPriority, NULL, NULL, &highCount);
    printf("High priority tasks (%d):\n", highCount);
    for (int i = 0; i < highCount; i++) {
        printf("  - %s\n", highTasks[i]->action);
    }
    free(highTasks);
    
    // Web project tasks
    int webCount;
    Task** webTasks = task_collection_get_filtered_tasks(allTasks, NULL, "website-redesign", NULL, &webCount);
    printf("Website project tasks (%d):\n", webCount);
    for (int i = 0; i < webCount; i++) {
        printf("  - %s\n", webTasks[i]->action);
    }
    free(webTasks);
    
    // 7. Update task progress
    printf("\nUpdating task progress:\n");
    int apiProgress = 60;
    task_update(apiTask, NULL, NULL, NULL, &apiProgress);
    printf("API task progress: %d%%\n", *apiTask->progress);
    
    int designProgress = 30;
    task_update(designTask, NULL, NULL, NULL, &designProgress);
    printf("Design task progress: %d%%\n", *designTask->progress);
    
    // 8. Complete a task
    printf("\nCompleting content task:\n");
    printf("Before: Status=%d, Progress=%s\n", 
           contentTask->status, 
           contentTask->progress ? "set" : "not set");
    
    task_complete(contentTask);
    
    printf("After: Status=%d, Progress=%d%%\n", 
           contentTask->status, 
           contentTask->progress ? *contentTask->progress : 0);
    
    // 9. Check active tasks
    printf("\nActive tasks:\n");
    for (int i = 0; i < allTasks->tasks_count; i++) {
        Task* task = allTasks->tasks[i];
        if (task_is_active(task)) {
            printf("- %s (%s)\n", task->action, task->parent_project);
        }
    }
    
    // 10. Clean up
    task_collection_free(allTasks);
    project_free(webProject);
    project_free(marketingProject);
    config_free(config);
    task_free(designTask);
    task_free(apiTask);
    task_free(contentTask);
    task_free(analyticsTask);
    
    printf("\nExample 4 completed successfully!\n");
}

int main() {
    example_real_world_scenario();
    return 0;
}
