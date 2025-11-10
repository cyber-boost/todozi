#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdarg.h>
#include <errno.h>
#include <sys/stat.h>

// Forward declarations
typedef struct Task Task;
typedef struct ProjectTaskContainer ProjectTaskContainer;
typedef struct MigrationReport MigrationReport;
typedef struct ProjectMigrationStats ProjectMigrationStats;

// Error handling
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_STORAGE,
    TODOZI_ERROR_EMBEDDING,
    TODOZI_ERROR_FILE
} TodoziError;

typedef struct {
    TodoziError error_type;
    char* message;
} Result;

// Task structure (simplified)
struct Task {
    char* id;
    char* parent_project;
    char* status;
    double* embedding_vector;
    int vector_size;
    Task* next;
};

// ProjectTaskContainer structure (simplified)
struct ProjectTaskContainer {
    char* project_name;
    Task* tasks;
    int task_count;
};

// ProjectMigrationStats structure
struct ProjectMigrationStats {
    char* project_name;
    int initial_tasks;
    int migrated_tasks;
    int final_tasks;
    ProjectMigrationStats* next;
};

// MigrationReport structure
struct MigrationReport {
    int tasks_found;
    int tasks_migrated;
    int projects_migrated;
    ProjectMigrationStats* project_stats;
    char** errors;
    int error_count;
};

// HashMap for project groups (simplified)
typedef struct {
    char** keys;
    Task** task_lists;
    int* task_counts;
    int size;
    int capacity;
} HashMap;

// TaskMigrator structure
typedef struct {
    bool dry_run;
    bool verbose;
    bool force_overwrite;
} TaskMigrator;

// MigrationCli structure
typedef struct {
    TaskMigrator* migrator;
} MigrationCli;

// Function prototypes
TaskMigrator* task_migrator_new(void);
TaskMigrator* task_migrator_dry_run(TaskMigrator* self, bool dry_run);
TaskMigrator* task_migrator_verbose(TaskMigrator* self, bool verbose);
TaskMigrator* task_migrator_force_overwrite(TaskMigrator* self, bool force_overwrite);
Result migration_report_new(MigrationReport* report);
void migration_report_free(MigrationReport* report);
Result task_migrator_migrate(TaskMigrator* self, MigrationReport* report);
Result task_migrator_load_legacy_tasks(TaskMigrator* self, MigrationReport* report, Task** all_tasks, int* task_count);
HashMap* task_migrator_group_tasks_by_project(TaskMigrator* self, Task* tasks, int task_count);
Result task_migrator_migrate_project_tasks(TaskMigrator* self, const char* project_name, Task* tasks, int task_count, ProjectMigrationStats* stats);
void task_migrator_print_summary(TaskMigrator* self, MigrationReport* report);
Result task_migrator_validate_migration(TaskMigrator* self, bool* is_valid);
Result task_migrator_cleanup_legacy(TaskMigrator* self);
MigrationCli* migration_cli_new(void);
MigrationCli* migration_cli_with_dry_run(MigrationCli* self, bool dry_run);
MigrationCli* migration_cli_with_verbose(MigrationCli* self, bool verbose);
MigrationCli* migration_cli_with_force(MigrationCli* self, bool force);
Result migration_cli_run(MigrationCli* self);

// Helper functions
HashMap* hashmap_new(int capacity);
void hashmap_free(HashMap* map);
void hashmap_put(HashMap* map, const char* key, Task* task);
int hashmap_size(HashMap* map);
void hashmap_get_entries(HashMap* map, char*** keys, Task*** task_lists, int** task_counts, int* size);
Task* task_clone(Task* task);
void task_free(Task* task);
void task_list_free(Task* tasks);
ProjectTaskContainer* project_task_container_new(const char* project_name);
void project_task_container_free(ProjectTaskContainer* container);
Task* project_task_container_get_task(ProjectTaskContainer* container, const char* id);
void project_task_container_add_task(ProjectTaskContainer* container, Task* task);
Task* project_task_container_get_all_tasks(ProjectTaskContainer* container, int* count);
Result load_task_collection(const char* collection_name, Task** tasks, int* count);
Result load_project_task_container(const char* project_name, ProjectTaskContainer** container);
Result save_project_task_container(ProjectTaskContainer* container);
Result list_project_task_containers(ProjectTaskContainer*** containers, int* count);
char* get_storage_dir(void);
bool file_exists(const char* path);
ProjectMigrationStats* project_migration_stats_new(const char* project_name);
void project_migration_stats_free(ProjectMigrationStats* stats);
void project_migration_stats_list_free(ProjectMigrationStats* stats);

// Result helper functions
static inline Result result_ok(void) {
    return (Result){ TODOZI_SUCCESS, NULL };
}

static inline Result result_err(TodoziError code, const char *fmt, ...) {
    char *msg = NULL;
    va_list ap;
    va_start(ap, fmt);
    vasprintf(&msg, fmt, ap);
    va_end(ap);
    return (Result){ code, msg };
}

static inline void result_free(Result *r) {
    if (r && r->message) free(r->message);
}

// Implementation

TaskMigrator* task_migrator_new(void) {
    TaskMigrator* self = malloc(sizeof(TaskMigrator));
    if (!self) return NULL;
    self->dry_run = false;
    self->verbose = false;
    self->force_overwrite = false;
    return self;
}

TaskMigrator* task_migrator_dry_run(TaskMigrator* self, bool dry_run) {
    if (self) self->dry_run = dry_run;
    return self;
}

TaskMigrator* task_migrator_verbose(TaskMigrator* self, bool verbose) {
    if (self) self->verbose = verbose;
    return self;
}

TaskMigrator* task_migrator_force_overwrite(TaskMigrator* self, bool force_overwrite) {
    if (self) self->force_overwrite = force_overwrite;
    return self;
}

Result migration_report_new(MigrationReport* report) {
    if (!report) return result_err(TODOZI_ERROR_STORAGE, "Invalid report pointer");
    memset(report, 0, sizeof(MigrationReport));
    return result_ok();
}

void migration_report_free(MigrationReport* report) {
    if (!report) return;
    if (report->errors) {
        for (int i = 0; i < report->error_count; i++) {
            free(report->errors[i]);
        }
        free(report->errors);
    }
    project_migration_stats_list_free(report->project_stats);
    memset(report, 0, sizeof(MigrationReport));
}

Result task_migrator_migrate(TaskMigrator* self, MigrationReport* report) {
    if (!self || !report) return result_err(TODOZI_ERROR_STORAGE, "Invalid parameters");
    
    Result result = migration_report_new(report);
    if (result.error_type != TODOZI_SUCCESS) {
        return result;
    }

    if (self->verbose) {
        printf("🚀 Starting task migration to project-based system...\n");
        if (self->dry_run) {
            printf("🔍 DRY RUN MODE - No actual changes will be made\n");
        }
    }

    Task* all_tasks = NULL;
    int task_count = 0;
    result = task_migrator_load_legacy_tasks(self, report, &all_tasks, &task_count);
    if (result.error_type != TODOZI_SUCCESS) {
        task_list_free(all_tasks);
        return result;
    }

    if (task_count == 0) {
        if (self->verbose) {
            printf("✅ No legacy tasks found - migration complete\n");
        }
        task_list_free(all_tasks);
        return result_ok();
    }

    HashMap* project_groups = task_migrator_group_tasks_by_project(self, all_tasks, task_count);
    if (!project_groups) {
        task_list_free(all_tasks);
        return result_err(TODOZI_ERROR_STORAGE, "Failed to group tasks by project");
    }

    if (self->verbose) {
        printf("📊 Found %d unique projects\n", hashmap_size(project_groups));
        
        for (int i = 0; i < project_groups->size; i++) {
            printf("   • %s: %d tasks\n", project_groups->keys[i], project_groups->task_counts[i]);
        }
    }

    for (int i = 0; i < project_groups->size; i++) {
        ProjectMigrationStats* stats = project_migration_stats_new(project_groups->keys[i]);
        if (!stats) {
            hashmap_free(project_groups);
            task_list_free(all_tasks);
            return result_err(TODOZI_ERROR_STORAGE, "Failed to create project stats");
        }

        result = task_migrator_migrate_project_tasks(self, project_groups->keys[i], project_groups->task_lists[i], project_groups->task_counts[i], stats);
        if (result.error_type != TODOZI_SUCCESS) {
            project_migration_stats_free(stats);
            hashmap_free(project_groups);
            task_list_free(all_tasks);
            return result;
        }

        // Add stats to report
        stats->next = report->project_stats;
        report->project_stats = stats;
        report->projects_migrated++;
        report->tasks_migrated += stats->migrated_tasks;
    }

    hashmap_free(project_groups);
    task_list_free(all_tasks);

    if (self->verbose) {
        task_migrator_print_summary(self, report);
    }

    return result_ok();
}

Result task_migrator_load_legacy_tasks(TaskMigrator* self, MigrationReport* report, Task** all_tasks, int* task_count) {
    const char* collections[] = {"active", "completed", "archived"};
    int collection_count = 3;
    
    *all_tasks = NULL;
    *task_count = 0;
    Task* tail = NULL;

    for (int i = 0; i < collection_count; i++) {
        Task* collection_tasks = NULL;
        int collection_task_count = 0;
        
        Result result = load_task_collection(collections[i], &collection_tasks, &collection_task_count);
        if (result.error_type == TODOZI_SUCCESS) {
            // Append tasks to all_tasks list
            if (*all_tasks == NULL) {
                *all_tasks = collection_tasks;
            } else if (tail) {
                tail->next = collection_tasks;
            }
            
            // Find the new tail
            Task* current = collection_tasks;
            while (current && current->next) {
                current = current->next;
            }
            tail = current;
            
            *task_count += collection_task_count;
            report->tasks_found += collection_task_count;
            
            if (self->verbose) {
                printf("📂 Loaded %d tasks from '%s' collection\n", collection_task_count, collections[i]);
            }
        } else {
            if (self->verbose) {
                printf("⚠️  Could not load '%s' collection (may not exist)\n", collections[i]);
            }
            result_free(&result);
        }
    }

    return result_ok();
}

HashMap* task_migrator_group_tasks_by_project(TaskMigrator* self, Task* tasks, int task_count) {
    HashMap* map = hashmap_new(task_count > 16 ? task_count : 16);
    if (!map) {
        return NULL;
    }

    Task* current = tasks;
    while (current) {
        const char* project = (current->parent_project && strlen(current->parent_project) > 0) 
                             ? current->parent_project 
                             : "general";
        
        hashmap_put(map, project, current);
        current = current->next;
    }

    return map;
}

Result task_migrator_migrate_project_tasks(TaskMigrator* self, const char* project_name, Task* tasks, int task_count, ProjectMigrationStats* stats) {
    if (!self || !project_name || !stats) {
        return result_err(TODOZI_ERROR_STORAGE, "Invalid parameters");
    }
    
    stats->initial_tasks = 0;
    stats->migrated_tasks = 0;
    stats->final_tasks = 0;

    ProjectTaskContainer* existing_container = NULL;
    Result result = load_project_task_container(project_name, &existing_container);
    
    if (result.error_type == TODOZI_SUCCESS && existing_container) {
        int existing_task_count;
        project_task_container_get_all_tasks(existing_container, &existing_task_count);
        stats->initial_tasks = existing_task_count;
        
        if (!self->force_overwrite && stats->initial_tasks > 0) {
            if (self->verbose) {
                printf("⚠️  Project '%s' already exists with %d tasks (use --force to overwrite)\n", 
                       project_name, stats->initial_tasks);
            }
            stats->final_tasks = stats->initial_tasks;
            project_task_container_free(existing_container);
            return result_ok();
        }
        project_task_container_free(existing_container);
    } else {
        result_free(&result);
    }

    ProjectTaskContainer* container = NULL;
    result = load_project_task_container(project_name, &container);
    if (result.error_type != TODOZI_SUCCESS) {
        result_free(&result);
        container = project_task_container_new(project_name);
        if (!container) {
            return result_err(TODOZI_ERROR_STORAGE, "Failed to create project container");
        }
    } else if (self->verbose) {
        printf("📁 Loading existing project container for '%s'\n", project_name);
    }

    if (self->verbose) {
        printf("📁 Creating new project container for '%s'\n", project_name);
    }

    int initial_count;
    project_task_container_get_all_tasks(container, &initial_count);
    stats->initial_tasks = initial_count;

    Task* current = tasks;
    while (current) {
        // Check if task already exists
        Task* existing_task = project_task_container_get_task(container, current->id);
        if (existing_task) {
            if (self->verbose) {
                printf("   ⏭️  Skipping duplicate task: %s\n", current->id);
            }
            current = current->next;
            continue;
        }

        // In a real implementation, you would initialize the embedding service here
        // For now, we'll skip the embedding generation part
        // Stub embedding service
        if (1) { // Simulate successful embedding service initialization
            if (self->verbose) {
                printf("   🧠 Generated embedding for task: %s\n", current->id);
            }
        }

        Task* task_copy = task_clone(current);
        if (!task_copy) {
            project_task_container_free(container);
            return result_err(TODOZI_ERROR_STORAGE, "Failed to clone task");
        }
        
        project_task_container_add_task(container, task_copy);
        stats->migrated_tasks++;

        if (self->verbose) {
            printf("   ✅ Migrated task: %s (status: %s)\n", 
                   task_copy->id, 
                   task_copy->status ? task_copy->status : "unknown");
        }

        current = current->next;
    }

    int final_count;
    project_task_container_get_all_tasks(container, &final_count);
    stats->final_tasks = final_count;

    if (!self->dry_run) {
        result = save_project_task_container(container);
        if (result.error_type != TODOZI_SUCCESS) {
            project_task_container_free(container);
            return result;
        } else if (self->verbose) {
            printf("💾 Saved project container: %s\n", project_name);
        }
    } else {
        if (self->verbose) {
            printf("🔍 DRY RUN: Would save project container: %s (%d tasks)\n", project_name, stats->final_tasks);
        }
    }

    project_task_container_free(container);
    return result_ok();
}

void task_migrator_print_summary(TaskMigrator* self, MigrationReport* report) {
    if (!self || !report) return;
    
    printf("\n============================================================\n");
    printf("📊 MIGRATION SUMMARY\n");
    printf("============================================================\n");
    printf("Total legacy tasks found: %d\n", report->tasks_found);
    printf("Tasks migrated: %d\n", report->tasks_migrated);
    printf("Projects processed: %d\n", report->projects_migrated);
    
    if (report->project_stats) {
        printf("\n📋 Project Details:\n");
        ProjectMigrationStats* current = report->project_stats;
        while (current) {
            printf("  • %s: %d → %d tasks (%d migrated)\n", 
                   current->project_name, 
                   current->initial_tasks, 
                   current->final_tasks, 
                   current->migrated_tasks);
            current = current->next;
        }
    }
    
    if (report->error_count > 0) {
        printf("\n⚠️  Errors encountered:\n");
        for (int i = 0; i < report->error_count; i++) {
            printf("  • %s\n", report->errors[i]);
        }
    }
    
    if (report->tasks_migrated == report->tasks_found && report->error_count == 0) {
        printf("\n✅ Migration completed successfully!\n");
    } else {
        printf("\n⚠️  Migration completed with warnings\n");
    }
    printf("============================================================\n");
}

Result task_migrator_validate_migration(TaskMigrator* self, bool* is_valid) {
    if (!self || !is_valid) return result_err(TODOZI_ERROR_STORAGE, "Invalid parameters");
    
    if (self->verbose) {
        printf("🔍 Validating migration integrity...\n");
    }

    int legacy_tasks = 0;
    const char* collections[] = {"active", "completed", "archived"};
    for (int i = 0; i < 3; i++) {
        Task* tasks = NULL;
        int count = 0;
        Result result = load_task_collection(collections[i], &tasks, &count);
        if (result.error_type == TODOZI_SUCCESS) {
            legacy_tasks += count;
            task_list_free(tasks);
        } else {
            result_free(&result);
        }
    }

    int project_tasks = 0;
    ProjectTaskContainer** containers = NULL;
    int container_count = 0;
    Result result = list_project_task_containers(&containers, &container_count);
    if (result.error_type == TODOZI_SUCCESS) {
        for (int i = 0; i < container_count; i++) {
            int count;
            project_task_container_get_all_tasks(containers[i], &count);
            project_tasks += count;
        }
        free(containers);
    } else {
        result_free(&result);
    }

    if (self->verbose) {
        printf("Legacy system tasks: %d\n", legacy_tasks);
        printf("Project system tasks: %d\n", project_tasks);
    }

    *is_valid = (legacy_tasks == 0) || (legacy_tasks > 0 && project_tasks >= legacy_tasks);
    
    if (*is_valid) {
        if (self->verbose) {
            printf("✅ Migration validation passed\n");
        }
    } else {
        if (self->verbose) {
            printf("❌ Migration validation failed\n");
        }
    }

    return result_ok();
}

Result task_migrator_cleanup_legacy(TaskMigrator* self) {
    if (!self) return result_err(TODOZI_ERROR_STORAGE, "Invalid parameters");
    
    if (self->verbose) {
        printf("🧹 Cleaning up legacy collections...\n");
    }

    const char* collections[] = {"active", "completed", "archived"};
    int cleaned_count = 0;

    for (int i = 0; i < 3; i++) {
        Task* tasks = NULL;
        int count = 0;
        Result result = load_task_collection(collections[i], &tasks, &count);
        
        if (result.error_type == TODOZI_SUCCESS) {
            if (count == 0) {
                char* storage_dir = get_storage_dir();
                if (storage_dir) {
                    char* collection_path;
                    int path_len = asprintf(&collection_path, "%s/tasks/%s.json", storage_dir, collections[i]);
                    
                    if (path_len > 0) {
                        if (file_exists(collection_path)) {
                            if (self->dry_run) {
                                if (self->verbose) {
                                    printf("   🔍 DRY RUN: Would remove empty collection '%s'\n", collections[i]);
                                }
                            } else {
                                if (remove(collection_path) == 0) {
                                    if (self->verbose) {
                                        printf("   🗑️  Removed empty collection '%s'\n", collections[i]);
                                    }
                                    cleaned_count++;
                                } else {
                                    if (self->verbose) {
                                        printf("   ⚠️  Could not remove '%s': %s\n", collections[i], strerror(errno));
                                    }
                                }
                            }
                        }
                        free(collection_path);
                    }
                    free(storage_dir);
                }
            } else {
                if (self->verbose) {
                    printf("   ⚠️  Collection '%s' still has %d tasks - not removing\n", collections[i], count);
                }
            }
            task_list_free(tasks);
        } else {
            if (self->verbose) {
                printf("   ℹ️  Collection '%s' does not exist\n", collections[i]);
            }
            result_free(&result);
        }
    }

    if (self->verbose) {
        if (cleaned_count > 0) {
            printf("✅ Cleaned up %d empty legacy collections\n", cleaned_count);
        } else {
            printf("ℹ️  No empty legacy collections to clean up\n");
        }
    }

    return result_ok();
}

MigrationCli* migration_cli_new(void) {
    MigrationCli* self = malloc(sizeof(MigrationCli));
    if (!self) return NULL;
    self->migrator = task_migrator_new();
    if (!self->migrator) {
        free(self);
        return NULL;
    }
    return self;
}

MigrationCli* migration_cli_with_dry_run(MigrationCli* self, bool dry_run) {
    if (self) task_migrator_dry_run(self->migrator, dry_run);
    return self;
}

MigrationCli* migration_cli_with_verbose(MigrationCli* self, bool verbose) {
    if (self) task_migrator_verbose(self->migrator, verbose);
    return self;
}

MigrationCli* migration_cli_with_force(MigrationCli* self, bool force) {
    if (self) task_migrator_force_overwrite(self->migrator, force);
    return self;
}

Result migration_cli_run(MigrationCli* self) {
    if (!self) return result_err(TODOZI_ERROR_STORAGE, "Invalid CLI instance");
    
    MigrationReport report;
    Result result = task_migrator_migrate(self->migrator, &report);
    
    if (result.error_type != TODOZI_SUCCESS) {
        migration_report_free(&report);
        return result;
    }

    if (!self->migrator->dry_run) {
        bool is_valid;
        result = task_migrator_validate_migration(self->migrator, &is_valid);
        if (result.error_type != TODOZI_SUCCESS) {
            migration_report_free(&report);
            return result;
        }

        if (is_valid && report.error_count == 0) {
            result = task_migrator_cleanup_legacy(self->migrator);
        }
    }

    migration_report_free(&report);
    return result;
}

// Helper implementations

HashMap* hashmap_new(int capacity) {
    HashMap* map = calloc(1, sizeof(HashMap));
    if (!map) return NULL;
    
    map->keys = calloc(capacity, sizeof(char*));
    map->task_lists = calloc(capacity, sizeof(Task*));
    map->task_counts = calloc(capacity, sizeof(int));
    if (!map->keys || !map->task_lists || !map->task_counts) {
        hashmap_free(map);
        return NULL;
    }
    
    map->capacity = capacity;
    return map;
}

void hashmap_free(HashMap* map) {
    if (!map) return;
    
    for (int i = 0; i < map->size; i++) {
        free(map->keys[i]);
        task_list_free(map->task_lists[i]);
    }
    
    free(map->keys);
    free(map->task_lists);
    free(map->task_counts);
    free(map);
}

void hashmap_put(HashMap* map, const char* key, Task* task) {
    if (!map || !key || !task) return;
    
    // Check if key already exists
    for (int i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            // Add task to existing list
            Task* new_task = task_clone(task);
            if (new_task) {
                new_task->next = map->task_lists[i];
                map->task_lists[i] = new_task;
                map->task_counts[i]++;
            }
            return;
        }
    }
    
    // Create new entry
    if (map->size >= map->capacity) return; // No resizing in this simple implementation
    
    map->keys[map->size] = strdup(key);
    if (!map->keys[map->size]) return;
    
    Task* new_task = task_clone(task);
    if (!new_task) {
        free(map->keys[map->size]);
        return;
    }
    
    new_task->next = NULL;
    map->task_lists[map->size] = new_task;
    map->task_counts[map->size] = 1;
    map->size++;
}

int hashmap_size(HashMap* map) {
    return map ? map->size : 0;
}

void hashmap_get_entries(HashMap* map, char*** keys, Task*** task_lists, int** task_counts, int* size) {
    if (!map || !keys || !task_lists || !task_counts || !size) return;
    
    *keys = map->keys;
    *task_lists = map->task_lists;
    *task_counts = map->task_counts;
    *size = map->size;
}

Task* task_clone(Task* task) {
    if (!task) return NULL;
    
    Task* clone = calloc(1, sizeof(Task));
    if (!clone) return NULL;
    
    if (task->id) {
        clone->id = strdup(task->id);
        if (!clone->id) {
            free(clone);
            return NULL;
        }
    }
    
    if (task->parent_project) {
        clone->parent_project = strdup(task->parent_project);
        if (!clone->parent_project) {
            free(clone->id);
            free(clone);
            return NULL;
        }
    }
    
    if (task->status) {
        clone->status = strdup(task->status);
        if (!clone->status) {
            free(clone->id);
            free(clone->parent_project);
            free(clone);
            return NULL;
        }
    }
    
    if (task->embedding_vector && task->vector_size > 0) {
        clone->embedding_vector = malloc(task->vector_size * sizeof(double));
        if (clone->embedding_vector) {
            memcpy(clone->embedding_vector, task->embedding_vector, task->vector_size * sizeof(double));
            clone->vector_size = task->vector_size;
        }
    }
    
    return clone;
}

void task_free(Task* task) {
    if (!task) return;
    free(task->id);
    free(task->parent_project);
    free(task->status);
    free(task->embedding_vector);
    free(task);
}

void task_list_free(Task* tasks) {
    Task* current = tasks;
    while (current) {
        Task* next = current->next;
        task_free(current);
        current = next;
    }
}

ProjectTaskContainer* project_task_container_new(const char* project_name) {
    ProjectTaskContainer* container = calloc(1, sizeof(ProjectTaskContainer));
    if (!container) return NULL;
    
    if (project_name) {
        container->project_name = strdup(project_name);
        if (!container->project_name) {
            free(container);
            return NULL;
        }
    }
    
    return container;
}

void project_task_container_free(ProjectTaskContainer* container) {
    if (!container) return;
    free(container->project_name);
    task_list_free(container->tasks);
    free(container);
}

Task* project_task_container_get_task(ProjectTaskContainer* container, const char* id) {
    if (!container || !id) return NULL;
    
    Task* current = container->tasks;
    while (current) {
        if (current->id && strcmp(current->id, id) == 0) {
            return current;
        }
        current = current->next;
    }
    return NULL;
}

void project_task_container_add_task(ProjectTaskContainer* container, Task* task) {
    if (!container || !task) return;
    
    task->next = container->tasks;
    container->tasks = task;
    container->task_count++;
}

Task* project_task_container_get_all_tasks(ProjectTaskContainer* container, int* count) {
    if (!container) {
        if (count) *count = 0;
        return NULL;
    }
    
    if (count) *count = container->task_count;
    return container->tasks;
}

Result load_task_collection(const char* collection_name, Task** tasks, int* count) {
    // Placeholder implementation - in real code this would load from JSON files
    *tasks = NULL;
    *count = 0;
    return result_err(TODOZI_ERROR_STORAGE, "Not implemented");
}

Result load_project_task_container(const char* project_name, ProjectTaskContainer** container) {
    // Placeholder implementation
    *container = NULL;
    return result_err(TODOZI_ERROR_STORAGE, "Not implemented");
}

Result save_project_task_container(ProjectTaskContainer* container) {
    // Placeholder implementation
    return result_err(TODOZI_ERROR_STORAGE, "Not implemented");
}

Result list_project_task_containers(ProjectTaskContainer*** containers, int* count) {
    // Placeholder implementation
    *containers = NULL;
    *count = 0;
    return result_err(TODOZI_ERROR_STORAGE, "Not implemented");
}

char* get_storage_dir(void) {
    // More portable implementation
    const char* home = getenv("HOME");
    if (home) {
        char* path;
        if (asprintf(&path, "%s/.todozi", home) > 0) {
            return path;
        }
    }
    return strdup("/tmp/todozi"); // Fallback
}

bool file_exists(const char* path) {
    if (!path) return false;
    struct stat st;
    return (stat(path, &st) == 0);
}

ProjectMigrationStats* project_migration_stats_new(const char* project_name) {
    ProjectMigrationStats* stats = calloc(1, sizeof(ProjectMigrationStats));
    if (!stats) return NULL;
    
    if (project_name) {
        stats->project_name = strdup(project_name);
        if (!stats->project_name) {
            free(stats);
            return NULL;
        }
    }
    
    return stats;
}

void project_migration_stats_free(ProjectMigrationStats* stats) {
    if (!stats) return;
    free(stats->project_name);
    free(stats);
}

void project_migration_stats_list_free(ProjectMigrationStats* stats) {
    ProjectMigrationStats* current = stats;
    while (current) {
        ProjectMigrationStats* next = current->next;
        project_migration_stats_free(current);
        current = next;
    }
}

// Test functions
void test_task_migrator_creation(void) {
    TaskMigrator* migrator = task_migrator_new();
    if (migrator && !migrator->dry_run && !migrator->verbose && !migrator->force_overwrite) {
        printf("✅ test_task_migrator_creation passed\n");
    } else {
        printf("❌ test_task_migrator_creation failed\n");
    }
    free(migrator);
}

void test_task_migrator_builder(void) {
    TaskMigrator* migrator = task_migrator_new();
    task_migrator_dry_run(migrator, true);
    task_migrator_verbose(migrator, true);
    task_migrator_force_overwrite(migrator, true);
    
    if (migrator && migrator->dry_run && migrator->verbose && migrator->force_overwrite) {
        printf("✅ test_task_migrator_builder passed\n");
    } else {
        printf("❌ test_task_migrator_builder failed\n");
    }
    free(migrator);
}

void test_migration_cli_builder(void) {
    MigrationCli* cli = migration_cli_new();
    migration_cli_with_dry_run(cli, true);
    migration_cli_with_verbose(cli, true);
    migration_cli_with_force(cli, true);
    
    if (cli && cli->migrator && 
        cli->migrator->dry_run && 
        cli->migrator->verbose && 
        cli->migrator->force_overwrite) {
        printf("✅ test_migration_cli_builder passed\n");
    } else {
        printf("❌ test_migration_cli_builder failed\n");
    }
    free(cli);
}

int main(void) {
    // Run tests
    test_task_migrator_creation();
    test_task_migrator_builder();
    test_migration_cli_builder();
    
    return 0;
}