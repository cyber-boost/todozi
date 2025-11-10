#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Example usage of the storage system
int main() {
    printf(" Todozi Storage System Example\n");
    printf("==============================\n");
    
    // Initialize storage structure
    if (init_storage() != TODOZI_SUCCESS) {
        printf("❌ Failed to initialize storage\n");
        return 1;
    }
    
    printf("✅ Storage initialized successfully\n\n");
    
    // Create a new project
    Project* my_project = malloc(sizeof(Project));
    my_project->name = strdup("website-redesign");
    my_project->description = strdup("Complete redesign of company website");
    
    if (save_project(my_project) == TODOZI_SUCCESS) {
        printf("✅ Created project: %s\n", my_project->name);
    } else {
        printf("❌ Failed to create project\n");
        free_project(my_project);
        return 1;
    }
    
    // Create tasks for the project
    Task* task1 = malloc(sizeof(Task));
    task1->id = strdup("task-001");
    task1->action = strdup("Create wireframes for homepage");
    task1->status = strdup("active");
    task1->priority = strdup("high");
    task1->parent_project = strdup("website-redesign");
    task1->created_at = time(NULL);
    task1->updated_at = time(NULL);
    task1->context_notes = NULL;
    task1->embedding_vector = NULL;
    task1->embedding_size = 0;
    
    Task* task2 = malloc(sizeof(Task));
    task2->id = strdup("task-002");
    task2->action = strdup("Write copy for product pages");
    task2->status = strdup("pending");
    task2->priority = strdup("medium");
    task2->parent_project = strdup("website-redesign");
    task2->created_at = time(NULL);
    task2->updated_at = time(NULL);
    task2->context_notes = NULL;
    task2->embedding_vector = NULL;
    task2->embedding_size = 0;
    
    // Save tasks
    if (save_task(task1) == TODOZI_SUCCESS) {
        printf("✅ Created task: %s\n", task1->action);
    }
    
    if (save_task(task2) == TODOZI_SUCCESS) {
        printf("✅ Created task: %s\n", task2->action);
    }
    
    // Load and display a task
    Task* loaded_task = load_task("task-001");
    if (loaded_task) {
        printf("\n📋 Loaded Task Details:\n");
        printf("   ID: %s\n", loaded_task->id);
        printf("   Action: %s\n", loaded_task->action);
        printf("   Status: %s\n", loaded_task->status);
        printf("   Priority: %s\n", loaded_task->priority);
        printf("   Project: %s\n", loaded_task->parent_project);
        free_task(loaded_task);
    }
    
    // Create and save a feeling entry
    Feeling* feeling = malloc(sizeof(Feeling));
    feeling->id = strdup("feeling-001");
    feeling->created_at = time(NULL);
    
    if (save_feeling(feeling) == TODOZI_SUCCESS) {
        printf("\n✅ Recorded feeling entry\n");
    }
    
    // Create and save an agent assignment
    AgentAssignment* assignment = malloc(sizeof(AgentAssignment));
    assignment->agent_id = strdup("designer");
    assignment->task_id = strdup("task-001");
    
    if (save_agent_assignment(assignment) == TODOZI_SUCCESS) {
        printf("✅ Assigned designer agent to task-001\n");
    }
    
    // Verify folder structure
    printf("\n");
    check_folder_structure();
    
    // Cleanup
    free_task(task1);
    free_task(task2);
    free_project(my_project);
    free_feeling(feeling);
    free_agent_assignment(assignment);
    
    printf("\n✅ Example completed successfully!\n");
    return 0;
}
