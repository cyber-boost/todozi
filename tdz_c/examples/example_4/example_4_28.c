// example4.c - Complete Migration Workflow Example
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "migration.c"  // Include the main migration system

// Mock implementation of required functions for demonstration
static Task* create_sample_task(const char* id, const char* project, const char* status) {
    Task* task = calloc(1, sizeof(Task));
    if (!task) return NULL;
    
    task->id = strdup(id);
    task->parent_project = strdup(project);
    task->status = strdup(status);
    task->vector_size = 3;
    task->embedding_vector = malloc(3 * sizeof(double));
    if (task->embedding_vector) {
        task->embedding_vector[0] = 0.1;
        task->embedding_vector[1] = 0.5;
        task->embedding_vector[2] = 0.9;
    }
    return task;
}

// Mock implementation of storage functions
static Task* legacy_active_tasks = NULL;
static Task* legacy_completed_tasks = NULL;
static Task* legacy_archived_tasks = NULL;

Result load_task_collection(const char* collection_name, Task** tasks, int* count) {
    *tasks = NULL;
    *count = 0;
    
    if (strcmp(collection_name, "active") == 0) {
        *tasks = legacy_active_tasks;
        Task* current = legacy_active_tasks;
        while (current) {
            (*count)++;
            current = current->next;
        }
    } else if (strcmp(collection_name, "completed") == 0) {
        *tasks = legacy_completed_tasks;
        Task* current = legacy_completed_tasks;
        while (current) {
            (*count)++;
            current = current->next;
        }
    } else if (strcmp(collection_name, "archived") == 0) {
        *tasks = legacy_archived_tasks;
        Task* current = legacy_archived_tasks;
        while (current) {
            (*count)++;
            current = current->next;
        }
    }
    
    return result_ok();
}

Result load_project_task_container(const char* project_name, ProjectTaskContainer** container) {
    *container = project_task_container_new(project_name);
    return *container ? result_ok() : result_err(TODOZI_ERROR_STORAGE, "Failed to create container");
}

Result save_project_task_container(ProjectTaskContainer* container) {
    printf("💾 Saved project '%s' with %d tasks\n", container->project_name, container->task_count);
    return result_ok();
}

Result list_project_task_containers(ProjectTaskContainer*** containers, int* count) {
    *containers = NULL;
    *count = 0;
    return result_ok();
}

// Setup sample data
void setup_sample_data(void) {
    // Create sample legacy tasks
    Task* task1 = create_sample_task("task-001", "web-app", "active");
    Task* task2 = create_sample_task("task-002", "web-app", "completed");
    Task* task3 = create_sample_task("task-003", "mobile-app", "active");
    Task* task4 = create_sample_task("task-004", NULL, "active"); // No project
    Task* task5 = create_sample_task("task-005", "web-app", "archived");
    
    // Add to legacy collections
    task1->next = task2;
    legacy_active_tasks = task1;
    
    legacy_completed_tasks = task3;
    
    task4->next = task5;
    legacy_archived_tasks = task4;
}

// Cleanup sample data
void cleanup_sample_data(void) {
    task_list_free(legacy_active_tasks);
    task_list_free(legacy_completed_tasks);
    task_list_free(legacy_archived_tasks);
    legacy_active_tasks = NULL;
    legacy_completed_tasks = NULL;
    legacy_archived_tasks = NULL;
}

int main(void) {
    printf("🚀 Todozi Migration System Example\n");
    printf("==================================\n\n");
    
    // Setup sample data
    setup_sample_data();
    
    // Example 1: Basic migration with verbose output
    printf("📋 Example 1: Basic Migration\n");
    printf("-----------------------------\n");
    
    MigrationCli* cli = migration_cli_new();
    if (!cli) {
        printf("❌ Failed to create migration CLI\n");
        cleanup_sample_data();
        return 1;
    }
    
    // Configure the CLI
    migration_cli_with_verbose(cli, true);
    migration_cli_with_dry_run(cli, false);
    migration_cli_with_force(cli, true);
    
    // Run the migration
    Result result = migration_cli_run(cli);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Migration failed: %s\n", result.message ? result.message : "Unknown error");
        result_free(&result);
        free(cli);
        cleanup_sample_data();
        return 1;
    }
    
    printf("✅ Basic migration completed successfully!\n\n");
    
    // Example 2: Dry run to preview changes
    printf("📋 Example 2: Dry Run Mode\n");
    printf("--------------------------\n");
    
    migration_cli_with_dry_run(cli, true);  // Enable dry run
    migration_cli_with_verbose(cli, true);
    
    result = migration_cli_run(cli);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Dry run failed: %s\n", result.message ? result.message : "Unknown error");
        result_free(&result);
    } else {
        printf("✅ Dry run completed successfully!\n\n");
    }
    
    // Example 3: Migration without verbose output
    printf("📋 Example 3: Silent Migration\n");
    printf("-----------------------------\n");
    
    migration_cli_with_dry_run(cli, false);   // Disable dry run
    migration_cli_with_verbose(cli, false);   // Disable verbose
    
    result = migration_cli_run(cli);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Silent migration failed: %s\n", result.message ? result.message : "Unknown error");
        result_free(&result);
    } else {
        printf("✅ Silent migration completed successfully!\n\n");
    }
    
    // Cleanup
    free(cli);
    cleanup_sample_data();
    
    printf("🎉 All examples completed!\n");
    return 0;
}
