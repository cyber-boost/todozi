#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Assuming storage.h contains the declarations from storage.c
// #include "storage.h"

// Example usage of the storage system
int main() {
    printf("Initializing Todozi storage system...\n");
    
    // Initialize the storage structure
    if (init_storage() != TODOZI_SUCCESS) {
        printf("Failed to initialize storage\n");
        return 1;
    }
    
    printf("Storage initialized successfully!\n");
    
    // Create and save a new task
    Task* task = malloc(sizeof(Task));
    if (!task) {
        printf("Failed to allocate task\n");
        return 1;
    }
    
    task->id = strdup("task-001");
    task->action = strdup("Implement user authentication");
    task->status = strdup("active");
    task->priority = strdup("high");
    task->parent_project = strdup("general");
    task->created_at = time(NULL);
    task->updated_at = time(NULL);
    task->context_notes = strdup("Need to integrate with OAuth providers");
    task->embedding_vector = NULL;
    task->embedding_size = 0;
    
    if (save_task(task) != TODOZI_SUCCESS) {
        printf("Failed to save task\n");
        free_task(task);
        return 1;
    }
    
    printf("Task saved successfully!\n");
    
    // Load the task back
    Task* loaded_task = load_task("task-001");
    if (!loaded_task) {
        printf("Failed to load task\n");
        free_task(task);
        return 1;
    }
    
    printf("Loaded task: %s - %s (Status: %s)\n", 
           loaded_task->id, 
           loaded_task->action, 
           loaded_task->status);
    
    // Create a project
    Project* project = malloc(sizeof(Project));
    if (!project) {
        printf("Failed to allocate project\n");
        free_task(task);
        free_task(loaded_task);
        return 1;
    }
    
    project->name = strdup("web-app");
    project->description = strdup("Main web application project");
    
    if (save_project(project) != TODOZI_SUCCESS) {
        printf("Failed to save project\n");
        free_task(task);
        free_task(loaded_task);
        free_project(project);
        return 1;
    }
    
    printf("Project saved successfully!\n");
    
    // Create an assignment
    AgentAssignment* assignment = malloc(sizeof(AgentAssignment));
    if (!assignment) {
        printf("Failed to allocate assignment\n");
        free_task(task);
        free_task(loaded_task);
        free_project(project);
        return 1;
    }
    
    assignment->agent_id = strdup("planner");
    assignment->task_id = strdup("task-001");
    
    if (save_agent_assignment(assignment) != TODOZI_SUCCESS) {
        printf("Failed to save assignment\n");
        free_task(task);
        free_task(loaded_task);
        free_project(project);
        free_agent_assignment(assignment);
        return 1;
    }
    
    printf("Assignment saved successfully!\n");
    
    // Create a feeling entry
    Feeling* feeling = malloc(sizeof(Feeling));
    if (!feeling) {
        printf("Failed to allocate feeling\n");
        free_task(task);
        free_task(loaded_task);
        free_project(project);
        free_agent_assignment(assignment);
        return 1;
    }
    
    feeling->id = strdup("feeling-001");
    feeling->created_at = time(NULL);
    
    if (save_feeling(feeling) != TODOZI_SUCCESS) {
        printf("Failed to save feeling\n");
        free_task(task);
        free_task(loaded_task);
        free_project(project);
        free_agent_assignment(assignment);
        free_feeling(feeling);
        return 1;
    }
    
    printf("Feeling saved successfully!\n");
    
    // Verify folder structure
    if (check_folder_structure()) {
        printf("Storage structure is valid!\n");
    } else {
        printf("Storage structure validation failed\n");
    }
    
    // Cleanup
    free_task(task);
    free_task(loaded_task);
    free_project(project);
    free_agent_assignment(assignment);
    free_feeling(feeling);
    
    printf("Example completed successfully!\n");
    return 0;
}
