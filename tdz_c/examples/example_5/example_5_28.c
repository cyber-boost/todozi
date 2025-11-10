// Example 5: Complete Migration Workflow with Error Handling
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Mock implementations for demonstration
static bool mock_storage_setup = false;
static int mock_task_count = 5;
static char* mock_project_names[] = {"web-app", "mobile-app", "general"};

// Mock task data structure
typedef struct {
    char* id;
    char* parent_project;
    char* status;
} MockTask;

static MockTask mock_tasks[] = {
    {"task-001", "web-app", "active"},
    {"task-002", "web-app", "completed"},
    {"task-003", "mobile-app", "active"},
    {"task-004", NULL, "active"},        // No project specified
    {"task-005", "general", "archived"}
};

// Mock implementations of required functions
Result load_task_collection(const char* collection_name, Task** tasks, int* count) {
    if (!mock_storage_setup) {
        return result_err(TODOZI_ERROR_STORAGE, "Storage not initialized");
    }
    
    *tasks = NULL;
    *count = 0;
    
    // Simulate loading tasks from a collection
    if (strcmp(collection_name, "active") == 0) {
        Task* head = NULL;
        Task* tail = NULL;
        
        for (int i = 0; i < mock_task_count; i++) {
            if (strcmp(mock_tasks[i].status, "active") == 0) {
                Task* new_task = calloc(1, sizeof(Task));
                new_task->id = strdup(mock_tasks[i].id);
                new_task->parent_project = mock_tasks[i].parent_project ? 
                                          strdup(mock_tasks[i].parent_project) : NULL;
                new_task->status = strdup(mock_tasks[i].status);
                
                if (!head) {
                    head = new_task;
                } else {
                    tail->next = new_task;
                }
                tail = new_task;
                (*count)++;
            }
        }
        *tasks = head;
    }
    
    return result_ok();
}

Result load_project_task_container(const char* project_name, ProjectTaskContainer** container) {
    *container = project_task_container_new(project_name);
    return *container ? result_ok() : result_err(TODOZI_ERROR_STORAGE, "Failed to create container");
}

Result save_project_task_container(ProjectTaskContainer* container) {
    printf("   💾 Saved container for project: %s (%d tasks)\n", 
           container->project_name, container->task_count);
    return result_ok();
}

Result list_project_task_containers(ProjectTaskContainer*** containers, int* count) {
    *containers = calloc(3, sizeof(ProjectTaskContainer*));
    for (int i = 0; i < 3; i++) {
        (*containers)[i] = project_task_container_new(mock_project_names[i]);
    }
    *count = 3;
    return result_ok();
}

char* get_storage_dir(void) {
    return strdup("/tmp/todozi");
}

// Initialize mock storage with sample data
void setup_mock_storage(void) {
    mock_storage_setup = true;
    printf("📦 Mock storage initialized with %d sample tasks\n", mock_task_count);
}

// Example 5: Complete Migration Workflow
int main_example5(void) {
    printf("=== Example 5: Complete Migration Workflow ===\n\n");
    
    // Setup mock environment
    setup_mock_storage();
    
    // Create migration CLI with all options enabled
    MigrationCli* cli = migration_cli_new();
    if (!cli) {
        printf("❌ Failed to create migration CLI\n");
        return 1;
    }
    
    // Configure CLI for verbose dry-run first
    printf("📋 Performing dry-run migration...\n");
    migration_cli_with_dry_run(cli, true);
    migration_cli_with_verbose(cli, true);
    migration_cli_with_force(cli, false);
    
    Result result = migration_cli_run(cli);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Dry-run failed: %s\n", result.message);
        result_free(&result);
        free(cli);
        return 1;
    }
    printf("✅ Dry-run completed successfully\n\n");
    
    // Now perform actual migration
    printf("🚀 Performing actual migration...\n");
    migration_cli_with_dry_run(cli, false);  // Disable dry-run
    
    result = migration_cli_run(cli);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Migration failed: %s\n", result.message);
        result_free(&result);
        free(cli);
        return 1;
    }
    printf("✅ Migration completed successfully\n\n");
    
    // Validate migration results
    printf("🔍 Validating migration...\n");
    bool is_valid;
    result = task_migrator_validate_migration(cli->migrator, &is_valid);
    if (result.error_type != TODOZI_SUCCESS) {
        printf("❌ Validation failed: %s\n", result.message);
        result_free(&result);
        free(cli);
        return 1;
    }
    
    if (is_valid) {
        printf("✅ Migration validation passed\n");
    } else {
        printf("⚠️  Migration validation has warnings\n");
    }
    
    // Cleanup
    free(cli);
    printf("\n✨ Example 5 completed successfully\n");
    return 0;
}

// Enhanced main function to run example
int main(void) {
    // Run original tests
    test_task_migrator_creation();
    test_task_migrator_builder();
    test_migration_cli_builder();
    printf("\n");
    
    // Run example 5
    return main_example5();
}
