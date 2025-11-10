#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include or assume the structures and functions from tests.c are available
// (In practice, you'd include a header file like "todozi.h")

void example_task_workflow() {
    printf("=== Example 3: Task Management Workflow ===\n");
    
    // 1. Create a task collection to manage our tasks
    TaskCollection* collection = task_collection_new();
    if (!collection) {
        printf("Failed to create task collection\n");
        return;
    }
    
    // 2. Create different types of tasks
    // Simple task
    Task* designTask = task_new(
        "user001", 
        "Create UI wireframes", 
        "4 hours", 
        PRIORITY_HIGH, 
        "Website Redesign", 
        STATUS_TODO
    );
    
    // Full-featured task with assignee and tags
    char* backendTags[] = {"api", "database", "security", NULL};
    char* dependencies[] = {designTask->id, NULL};
    int progress = 30;
    Assignee assignee = ASSIGNEE_COLLABORATIVE;
    
    Task* backendTask = task_new_full(
        "Implement user authentication API",
        "2 days",
        PRIORITY_CRITICAL,
        "Website Redesign",
        STATUS_IN_PROGRESS,
        &assignee,
        backendTags,
        3,
        dependencies,
        1,
        "Follow OAuth 2.0 standards",
        &progress
    );
    
    // Quick maintenance task
    Task* fixTask = task_new(
        "user002",
        "Fix broken links in footer",
        "30 minutes",
        PRIORITY_MEDIUM,
        "Website Redesign",
        STATUS_TODO
    );
    
    // 3. Add tasks to collection
    task_collection_add_task(collection, designTask);
    task_collection_add_task(collection, backendTask);
    task_collection_add_task(collection, fixTask);
    
    // 4. Demonstrate filtering capabilities
    printf("\n--- High Priority Tasks ---\n");
    Priority highPriority = PRIORITY_HIGH;
    int highCount;
    Task** highTasks = task_collection_get_filtered_tasks(
        collection, &highPriority, NULL, NULL, &highCount
    );
    for (int i = 0; i < highCount; i++) {
        printf("- %s (%s)\n", highTasks[i]->action, highTasks[i]->time);
    }
    free(highTasks);
    
    printf("\n--- Tasks in 'Website Redesign' Project ---\n");
    int projectCount;
    Task** projectTasks = task_collection_get_filtered_tasks(
        collection, NULL, "Website Redesign", NULL, &projectCount
    );
    for (int i = 0; i < projectCount; i++) {
        printf("- %s [Status: %d]\n", projectTasks[i]->action, projectTasks[i]->status);
    }
    free(projectTasks);
    
    printf("\n--- In-Progress Tasks ---\n");
    Status inProgress = STATUS_IN_PROGRESS;
    int progressCount;
    Task** progressTasks = task_collection_get_filtered_tasks(
        collection, NULL, NULL, &inProgress, &progressCount
    );
    for (int i = 0; i < progressCount; i++) {
        printf("- %s (%d%% complete)\n", 
               progressTasks[i]->action, 
               progressTasks[i]->progress ? *progressTasks[i]->progress : 0);
    }
    free(progressTasks);
    
    // 5. Update a task
    printf("\n--- Updating Task Progress ---\n");
    int newProgress = 65;
    task_update(fixTask, "Fix broken links and update copyright", NULL, NULL, &newProgress);
    printf("Updated: %s (%d%%)\n", fixTask->action, *fixTask->progress);
    
    // 6. Complete a task
    printf("\n--- Completing Task ---\n");
    task_complete(designTask);
    printf("Completed: %s\n", designTask->action);
    printf("Status: %d, Progress: %d%%\n", 
           designTask->status, 
           designTask->progress ? *designTask->progress : 0);
    
    // 7. Clean up
    task_free(designTask);
    task_free(backendTask);
    task_free(fixTask);
    task_collection_free(collection);
    
    printf("\nWorkflow example completed successfully!\n");
}

int main() {
    example_task_workflow();
    return 0;
}
