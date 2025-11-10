// example_migration_usage.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "migration.c"  // Include the main migration code

// Mock implementations for demonstration purposes
// In a real application, these would interact with actual storage

static Task* mock_active_tasks = NULL;
static Task* mock_completed_tasks = NULL;
static Task* mock_archived_tasks = NULL;

// Create a sample task for testing
Task* create_sample_task(const char* id, const char* project, const char* status) {
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

// Mock implementation of load_task_collection
Result load_task_collection(const char* collection_name, Task** tasks, int* count) {
    *tasks = NULL;
    *count = 0;
    
    if (strcmp(collection_name, "active") == 0) {
        *tasks = mock_active_tasks;
        Task* current = mock_active_tasks;
        while (current) {
            (*count)++;
            current = current->next;
        }
        return result_ok();
    } else if (strcmp(collection_name, "completed") == 0) {
        *tasks = mock_completed_tasks;
        Task* current = mock_completed_tasks;
        while (current) {
            (*count)++;
            current = current->next;
        }
        return result_ok();
    } else if (strcmp(collection_name, "archived") == 0) {
        *tasks = mock_archived_tasks;
        Task* current = mock_archived_tasks;
        while (current) {
            (*count)++;
            current = current->next;
        }
        return result_ok();
    }
    
    return result_err(TODOZI_ERROR_STORAGE, "Collection not found");
}

// Mock implementation of load_project_task_container
Result load_project_task_container(const char* project_name, ProjectTaskContainer** container) {
    *container = NULL;
    return result_err(TODOZI_ERROR_STORAGE, "Project container not found"); // Simulate not found
}

// Mock implementation of save_project_task_container
Result save_project_task_container(ProjectTaskContainer* container) {
    printf("💾 Saved project '%s' with %d tasks\n", container->project_name, container->task_count);
    return result_ok();
}

// Mock implementation of list_project_task_containers
Result list_project_task_containers(ProjectTaskContainer*** containers, int* count) {
    *containers = NULL;
    *count = 0;
    return result_ok();
}

// Mock implementation of get_storage_dir
char* get_storage_dir(void) {
    return strdup("/tmp/todozi");
}

// Set up mock data for demonstration
void setup_mock_data(void) {
    // Create sample active tasks
    Task* task1 = create_sample_task("task-001", "ProjectA", "active");
    Task* task2 = create_sample_task("task-002", "ProjectB", "active");
    Task* task3 = create_sample_task("task-003", "", "active"); // No project specified
    
    task1->next = task2;
    task2->next = task3;
    mock_active_tasks = task1;
    
    // Create sample completed tasks
    Task* task4 = create_sample_task("task-004", "ProjectA", "completed");
    Task* task5 = create_sample_task("task-005", "ProjectC", "completed");
    
    task4->next = task5;
    mock_completed_tasks = task4;
    
    // No archived tasks for this example
    mock_archived_tasks = NULL;
}

// Clean up mock data
void cleanup_mock_data(void) {
    task_list_free(mock_active_tasks);
    task_list_free(mock_completed_tasks);
    task_list_free(mock_archived_tasks);
}

int main(void) {
    printf("🔧 Setting up mock data...\n");
    setup_mock_data();
    
    printf("\n🚀 Running migration with verbose output...\n");
    
    // Create migration CLI with verbose output
    MigrationCli* cli = migration_cli_new();
    if (!cli) {
        printf("❌ Failed to create migration CLI\n");
        cleanup_mock_data();
        return 1;
    }
    
    // Configure the CLI
    migration_cli_with_verbose(cli, true);     // Show detailed output
    migration_cli_with_dry_run(cli, false);    // Actually perform migration
    migration_cli_with_force(cli, false);      // Don't overwrite existing projects
    
    // Run the migration
    Result result = migration_cli_run(cli);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Migration failed: %s\n", result.message);
        result_free(&result);
        free(cli);
        cleanup_mock_data();
        return 1;
    }
    
    printf("\n✅ Migration completed successfully!\n");
    
    // Clean up
    free(cli);
    cleanup_mock_data();
    
    return 0;
}
