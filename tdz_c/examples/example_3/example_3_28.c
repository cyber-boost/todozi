// example3.c - Complete migration example with storage implementation
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include "migration.c"  // Include the main migration code

// Simple in-memory storage for demonstration
static Task* active_tasks = NULL;
static Task* completed_tasks = NULL;
static Task* archived_tasks = NULL;
static ProjectTaskContainer** project_containers = NULL;
static int project_container_count = 0;

// Helper function to create a task
Task* create_task(const char* id, const char* project, const char* status) {
    Task* task = calloc(1, sizeof(Task));
    if (!task) return NULL;
    
    task->id = strdup(id);
    task->parent_project = strdup(project);
    task->status = strdup(status);
    task->vector_size = 3;
    task->embedding_vector = malloc(3 * sizeof(double));
    if (task->embedding_vector) {
        task->embedding_vector[0] = 0.1;
        task->embedding_vector[1] = 0.2;
        task->embedding_vector[2] = 0.3;
    }
    return task;
}

// Implementation of storage functions
Result load_task_collection(const char* collection_name, Task** tasks, int* count) {
    *tasks = NULL;
    *count = 0;
    
    if (strcmp(collection_name, "active") == 0) {
        *tasks = active_tasks;
    } else if (strcmp(collection_name, "completed") == 0) {
        *tasks = completed_tasks;
    } else if (strcmp(collection_name, "archived") == 0) {
        *tasks = archived_tasks;
    } else {
        return result_err(TODOZI_ERROR_STORAGE, "Unknown collection: %s", collection_name);
    }
    
    // Count tasks
    Task* current = *tasks;
    while (current) {
        (*count)++;
        current = current->next;
    }
    
    return result_ok();
}

Result load_project_task_container(const char* project_name, ProjectTaskContainer** container) {
    for (int i = 0; i < project_container_count; i++) {
        if (strcmp(project_containers[i]->project_name, project_name) == 0) {
            *container = project_containers[i];
            return result_ok();
        }
    }
    *container = NULL;
    return result_err(TODOZI_ERROR_STORAGE, "Project not found: %s", project_name);
}

Result save_project_task_container(ProjectTaskContainer* container) {
    // In a real implementation, this would save to disk
    printf("Saving project '%s' with %d tasks\n", 
           container->project_name, container->task_count);
    return result_ok();
}

Result list_project_task_containers(ProjectTaskContainer*** containers, int* count) {
    *containers = project_containers;
    *count = project_container_count;
    return result_ok();
}

// Setup test data
void setup_test_data(void) {
    // Create legacy tasks
    Task* task1 = create_task("task1", "ProjectA", "active");
    Task* task2 = create_task("task2", "ProjectA", "completed");
    Task* task3 = create_task("task3", "ProjectB", "active");
    Task* task4 = create_task("task4", "", "active");  // No project
    Task* task5 = create_task("task5", "ProjectC", "archived");
    
    active_tasks = task1;
    task1->next = task3;
    task3->next = task4;
    
    completed_tasks = task2;
    
    archived_tasks = task5;
    
    // Create project containers
    project_containers = calloc(2, sizeof(ProjectTaskContainer*));
    project_containers[0] = project_task_container_new("ProjectA");
    project_containers[1] = project_task_container_new("general");
    project_container_count = 2;
}

// Cleanup test data
void cleanup_test_data(void) {
    task_list_free(active_tasks);
    task_list_free(completed_tasks);
    task_list_free(archived_tasks);
    
    for (int i = 0; i < project_container_count; i++) {
        project_task_container_free(project_containers[i]);
    }
    free(project_containers);
}

int main(void) {
    printf("=== Example 3: Complete Migration Workflow ===\n\n");
    
    // Setup test data
    setup_test_data();
    
    // Create migration CLI with verbose output
    MigrationCli* cli = migration_cli_new();
    migration_cli_with_verbose(cli, true);
    migration_cli_with_dry_run(cli, false);  // Actually perform migration
    
    // Run migration
    printf("Running migration...\n");
    Result result = migration_cli_run(cli);
    
    if (result.error_type != TODOZI_SUCCESS) {
        printf("Migration failed: %s\n", result.message);
        result_free(&result);
        cleanup_test_data();
        free(cli);
        return 1;
    }
    
    printf("\nMigration completed successfully!\n");
    
    // Cleanup
    cleanup_test_data();
    free(cli);
    
    return 0;
}
