// example_usage.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "migration.c"  // Include the main migration code

// Mock implementation of required functions for demonstration
char* get_storage_dir(void) {
    return strdup("/tmp/todozi");
}

bool file_exists(const char* path) {
    struct stat st;
    return (stat(path, &st) == 0);
}

// Simple mock task creation for demonstration
Task* create_mock_task(const char* id, const char* project, const char* status) {
    Task* task = calloc(1, sizeof(Task));
    if (!task) return NULL;
    
    task->id = strdup(id);
    task->parent_project = strdup(project);
    task->status = strdup(status);
    task->vector_size = 3;
    task->embedding_vector = malloc(3 * sizeof(double));
    task->embedding_vector[0] = 0.1;
    task->embedding_vector[1] = 0.5;
    task->embedding_vector[2] = 0.9;
    
    return task;
}

// Mock implementation of storage functions
Result load_task_collection(const char* collection_name, Task** tasks, int* count) {
    *tasks = NULL;
    *count = 0;
    
    // Simulate loading tasks from legacy collections
    if (strcmp(collection_name, "active") == 0) {
        Task* task1 = create_mock_task("task1", "project-a", "active");
        Task* task2 = create_mock_task("task2", "project-b", "active");
        task2->next = task1;
        *tasks = task2;
        *count = 2;
        return result_ok();
    }
    
    if (strcmp(collection_name, "completed") == 0) {
        Task* task3 = create_mock_task("task3", "project-a", "completed");
        *tasks = task3;
        *count = 1;
        return result_ok();
    }
    
    return result_err(TODOZI_ERROR_STORAGE, "Collection not found");
}

Result load_project_task_container(const char* project_name, ProjectTaskContainer** container) {
    *container = NULL;
    return result_err(TODOZI_ERROR_STORAGE, "Project container not found");
}

Result save_project_task_container(ProjectTaskContainer* container) {
    printf("Saved project container: %s with %d tasks\n", 
           container->project_name, container->task_count);
    return result_ok();
}

Result list_project_task_containers(ProjectTaskContainer*** containers, int* count) {
    *containers = NULL;
    *count = 0;
    return result_ok();
}

// Main example function
int main(int argc, char* argv[]) {
    printf("=== Todozi Migration Tool Example ===\n\n");
    
    // Create migration CLI
    MigrationCli* cli = migration_cli_new();
    if (!cli) {
        printf("Failed to create migration CLI\n");
        return 1;
    }
    
    // Parse command line arguments
    bool dry_run = false;
    bool verbose = false;
    bool force = false;
    
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--dry-run") == 0) {
            dry_run = true;
        } else if (strcmp(argv[i], "--verbose") == 0) {
            verbose = true;
        } else if (strcmp(argv[i], "--force") == 0) {
            force = true;
        }
    }
    
    // Configure CLI
    migration_cli_with_dry_run(cli, dry_run);
    migration_cli_with_verbose(cli, verbose);
    migration_cli_with_force(cli, force);
    
    // Print configuration
    printf("Configuration:\n");
    printf("  Dry run: %s\n", dry_run ? "enabled" : "disabled");
    printf("  Verbose: %s\n", verbose ? "enabled" : "disabled");
    printf("  Force overwrite: %s\n\n", force ? "enabled" : "disabled");
    
    // Run migration
    printf("Running migration...\n");
    Result result = migration_cli_run(cli);
    
    if (result.error_type == TODOZI_SUCCESS) {
        printf("\nMigration completed successfully!\n");
    } else {
        printf("\nMigration failed: %s\n", result.message);
        result_free(&result);
        free(cli);
        return 1;
    }
    
    // Cleanup
    free(cli);
    return 0;
}
