#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <time.h>
#include <errno.h>
#include <stdint.h>

// Maximum length for assignee strings (longest is "collaborative" = 13 chars)
#define MAX_ASSIGNEE_LEN 16
#define MAX_TASK_ID_LEN 64

// Forward declarations
typedef struct Task Task;
typedef struct Project Project;
typedef struct TaskCollection TaskCollection;
typedef struct Config Config;

// Enums
typedef enum {
    PRIORITY_LOW,
    PRIORITY_MEDIUM,
    PRIORITY_HIGH,
    PRIORITY_CRITICAL,
    PRIORITY_URGENT
} Priority;

typedef enum {
    STATUS_TODO,
    STATUS_IN_PROGRESS,
    STATUS_BLOCKED,
    STATUS_REVIEW,
    STATUS_DONE,
    STATUS_CANCELLED,
    STATUS_DEFERRED
} Status;

typedef enum {
    ASSIGNEE_AI,
    ASSIGNEE_HUMAN,
    ASSIGNEE_COLLABORATIVE
} Assignee;

typedef enum {
    PROJECT_STATUS_ACTIVE,
    PROJECT_STATUS_ARCHIVED,
    PROJECT_STATUS_COMPLETED
} ProjectStatus;

// Structures
struct Task {
    char* id;
    char* action;
    char* time;
    Priority priority;
    char* parent_project;
    Status status;
    char* assignee;  // NULL if none
    char** tags;     // NULL terminated array
    int tags_count;
    char** dependencies;  // NULL terminated array
    int dependencies_count;
    char* context_notes;  // NULL if none
    int* progress;        // NULL if none
};

struct Project {
    char* name;
    char* description;  // NULL if none
    ProjectStatus status;
    char** tasks;       // NULL terminated array
    int tasks_count;
};

struct TaskCollection {
    Task** tasks;       // NULL terminated array
    int tasks_count;
};

struct Config {
    char* version;
    char* default_project;
    int auto_backup;
    char* backup_interval;
    int ai_enabled;
    Assignee* default_assignee;  // NULL if none
    char* date_format;
    char* timezone;
};

// Error handling
typedef enum {
    ERROR_TASK_NOT_FOUND,
    ERROR_INVALID_PRIORITY,
    ERROR_INVALID_STATUS,
    ERROR_INVALID_ASSIGNEE,
    ERROR_INVALID_PROGRESS,
    ERROR_VALIDATION
} ErrorCode;

typedef struct {
    ErrorCode code;
    char* message;
    union {
        char* id;
        char* priority_str;
        char* status_str;
        char* assignee_str;
        int progress;
    } data;
} TodoziError;

// Helper functions
char* generate_task_id() {
    static uint64_t counter = 0;
    char* id = malloc(MAX_TASK_ID_LEN);
    if (id == NULL) {
        return NULL;
    }
    snprintf(id, MAX_TASK_ID_LEN, "task_%llu", (unsigned long long)++counter);
    return id;
}

char* string_clone(const char* str) {
    if (str == NULL) return NULL;
    size_t len = strlen(str);
    char* clone = malloc(len + 1);
    if (clone == NULL) {
        return NULL;
    }
    memcpy(clone, str, len + 1);
    return clone;
}

char** string_array_clone(char** array, int count) {
    if (array == NULL || count == 0) return NULL;
    char** clone = malloc((count + 1) * sizeof(char*));
    if (clone == NULL) {
        return NULL;
    }
    for (int i = 0; i < count; i++) {
        clone[i] = string_clone(array[i]);
        if (clone[i] == NULL) {
            // Free already allocated strings on failure
            for (int j = 0; j < i; j++) {
                free(clone[j]);
            }
            free(clone);
            return NULL;
        }
    }
    clone[count] = NULL;
    return clone;
}

void string_array_free(char** array) {
    if (array == NULL) return;
    for (int i = 0; array[i] != NULL; i++) {
        free(array[i]);
    }
    free(array);
}

// Task functions
Task* task_new(const char* user_id, const char* action, const char* time,
               Priority priority, const char* parent_project, Status status) {
    // Validate required parameters
    if (action == NULL || time == NULL || parent_project == NULL) {
        return NULL;
    }
    
    Task* task = malloc(sizeof(Task));
    if (task == NULL) {
        return NULL;
    }
    
    // Initialize all fields to safe defaults first
    memset(task, 0, sizeof(Task));
    
    task->id = generate_task_id();
    if (task->id == NULL) {
        free(task);
        return NULL;
    }
    
    task->action = string_clone(action);
    if (task->action == NULL) {
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->time = string_clone(time);
    if (task->time == NULL) {
        free(task->action);
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->parent_project = string_clone(parent_project);
    if (task->parent_project == NULL) {
        free(task->time);
        free(task->action);
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->priority = priority;
    task->status = status;
    task->assignee = NULL;
    task->tags = NULL;
    task->tags_count = 0;
    task->dependencies = NULL;
    task->dependencies_count = 0;
    task->context_notes = NULL;
    task->progress = NULL;
    
    // Note: user_id parameter is currently unused but kept for API compatibility
    (void)user_id;
    
    return task;
}

Task* task_new_full(const char* action, const char* time, Priority priority,
                    const char* parent_project, Status status, Assignee* assignee,
                    char** tags, int tags_count, char** dependencies, int dependencies_count,
                    const char* context_notes, int* progress) {
    // Validate required parameters
    if (action == NULL || time == NULL || parent_project == NULL) {
        return NULL;
    }
    
    // Validate progress
    if (progress != NULL && (*progress < 0 || *progress > 100)) {
        return NULL;
    }
    
    // Validate counts
    if (tags_count < 0 || dependencies_count < 0) {
        return NULL;
    }

    Task* task = malloc(sizeof(Task));
    if (task == NULL) {
        return NULL;
    }
    
    // Initialize all fields to safe defaults first
    memset(task, 0, sizeof(Task));
    
    task->id = generate_task_id();
    if (task->id == NULL) {
        free(task);
        return NULL;
    }
    
    task->action = string_clone(action);
    if (task->action == NULL) {
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->time = string_clone(time);
    if (task->time == NULL) {
        free(task->action);
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->parent_project = string_clone(parent_project);
    if (task->parent_project == NULL) {
        free(task->time);
        free(task->action);
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->priority = priority;
    task->status = status;
    
    if (assignee != NULL) {
        task->assignee = malloc(MAX_ASSIGNEE_LEN);
        if (task->assignee == NULL) {
            free(task->parent_project);
            free(task->time);
            free(task->action);
            free(task->id);
            free(task);
            return NULL;
        }
        switch (*assignee) {
            case ASSIGNEE_AI:
                strncpy(task->assignee, "ai", MAX_ASSIGNEE_LEN - 1);
                task->assignee[MAX_ASSIGNEE_LEN - 1] = '\0';
                break;
            case ASSIGNEE_HUMAN:
                strncpy(task->assignee, "human", MAX_ASSIGNEE_LEN - 1);
                task->assignee[MAX_ASSIGNEE_LEN - 1] = '\0';
                break;
            case ASSIGNEE_COLLABORATIVE:
                strncpy(task->assignee, "collaborative", MAX_ASSIGNEE_LEN - 1);
                task->assignee[MAX_ASSIGNEE_LEN - 1] = '\0';
                break;
            default:
                free(task->assignee);
                task->assignee = NULL;
                break;
        }
    } else {
        task->assignee = NULL;
    }
    
    task->tags = string_array_clone(tags, tags_count);
    task->tags_count = tags_count;
    if (tags_count > 0 && task->tags == NULL) {
        free(task->assignee);
        free(task->parent_project);
        free(task->time);
        free(task->action);
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->dependencies = string_array_clone(dependencies, dependencies_count);
    task->dependencies_count = dependencies_count;
    if (dependencies_count > 0 && task->dependencies == NULL) {
        string_array_free(task->tags);
        free(task->assignee);
        free(task->parent_project);
        free(task->time);
        free(task->action);
        free(task->id);
        free(task);
        return NULL;
    }
    
    task->context_notes = string_clone(context_notes);
    // context_notes can be NULL, so we don't fail if string_clone returns NULL for NULL input
    
    if (progress != NULL) {
        task->progress = malloc(sizeof(int));
        if (task->progress == NULL) {
            free(task->context_notes);
            string_array_free(task->dependencies);
            string_array_free(task->tags);
            free(task->assignee);
            free(task->parent_project);
            free(task->time);
            free(task->action);
            free(task->id);
            free(task);
            return NULL;
        }
        *task->progress = *progress;
    } else {
        task->progress = NULL;
    }
    
    return task;
}

void task_update(Task* task, const char* new_action, Priority* new_priority,
                 Status* new_status, int* new_progress) {
    if (task == NULL) {
        return;
    }
    
    if (new_action != NULL) {
        char* cloned = string_clone(new_action);
        if (cloned != NULL) {
            free(task->action);
            task->action = cloned;
        }
        // If clone fails, keep old action
    }
    
    if (new_priority != NULL) {
        task->priority = *new_priority;
    }
    
    if (new_status != NULL) {
        task->status = *new_status;
    }
    
    if (new_progress != NULL) {
        if (*new_progress >= 0 && *new_progress <= 100) {
            if (task->progress == NULL) {
                task->progress = malloc(sizeof(int));
                if (task->progress == NULL) {
                    return; // Can't allocate, keep old state
                }
            }
            *task->progress = *new_progress;
        }
    }
}

void task_complete(Task* task) {
    if (task == NULL) {
        return;
    }
    task->status = STATUS_DONE;
    if (task->progress == NULL) {
        task->progress = malloc(sizeof(int));
        if (task->progress == NULL) {
            return; // Can't allocate, but status is still updated
        }
    }
    *task->progress = 100;
}

int task_is_completed(Task* task) {
    if (task == NULL) {
        return 0;
    }
    return task->status == STATUS_DONE;
}

int task_is_active(Task* task) {
    if (task == NULL) {
        return 0;
    }
    return task->status != STATUS_DONE && task->status != STATUS_CANCELLED;
}

void task_free(Task* task) {
    if (task == NULL) return;
    free(task->id);
    free(task->action);
    free(task->time);
    free(task->parent_project);
    free(task->assignee);
    string_array_free(task->tags);
    string_array_free(task->dependencies);
    free(task->context_notes);
    free(task->progress);
    free(task);
}

// Project functions
Project* project_new(const char* name, const char* description) {
    if (name == NULL) {
        return NULL;
    }
    
    Project* project = malloc(sizeof(Project));
    if (project == NULL) {
        return NULL;
    }
    
    // Initialize all fields to safe defaults first
    memset(project, 0, sizeof(Project));
    
    project->name = string_clone(name);
    if (project->name == NULL) {
        free(project);
        return NULL;
    }
    
    project->description = string_clone(description);
    // description can be NULL, so we don't fail if string_clone returns NULL for NULL input
    
    project->status = PROJECT_STATUS_ACTIVE;
    project->tasks = NULL;
    project->tasks_count = 0;
    return project;
}

void project_add_task(Project* project, const char* task_id) {
    if (project == NULL || task_id == NULL) {
        return;
    }
    
    // Check if task already exists
    for (int i = 0; i < project->tasks_count; i++) {
        if (project->tasks[i] != NULL && strcmp(project->tasks[i], task_id) == 0) {
            return; // Already exists
        }
    }
    
    // Add new task
    char* cloned_id = string_clone(task_id);
    if (cloned_id == NULL) {
        return; // Can't clone, fail early
    }
    
    char** new_tasks = realloc(project->tasks, (project->tasks_count + 2) * sizeof(char*));
    if (new_tasks == NULL) {
        free(cloned_id); // Free the cloned string if realloc fails
        return;
    }
    project->tasks = new_tasks;
    
    project->tasks[project->tasks_count] = cloned_id;
    project->tasks[project->tasks_count + 1] = NULL;
    project->tasks_count++;
}

void project_remove_task(Project* project, const char* task_id) {
    if (project == NULL || task_id == NULL || project->tasks == NULL) {
        return;
    }
    
    for (int i = 0; i < project->tasks_count; i++) {
        if (project->tasks[i] != NULL && strcmp(project->tasks[i], task_id) == 0) {
            free(project->tasks[i]);
            // Shift remaining elements
            for (int j = i; j < project->tasks_count - 1; j++) {
                project->tasks[j] = project->tasks[j + 1];
            }
            project->tasks[project->tasks_count - 1] = NULL;
            project->tasks_count--;
            
            // Optionally shrink array if it becomes much smaller
            if (project->tasks_count == 0 && project->tasks != NULL) {
                free(project->tasks);
                project->tasks = NULL;
            }
            return;
        }
    }
}

void project_archive(Project* project) {
    if (project == NULL) {
        return;
    }
    project->status = PROJECT_STATUS_ARCHIVED;
}

void project_complete(Project* project) {
    if (project == NULL) {
        return;
    }
    project->status = PROJECT_STATUS_COMPLETED;
}

void project_free(Project* project) {
    if (project == NULL) return;
    free(project->name);
    free(project->description);
    string_array_free(project->tasks);
    free(project);
}

// TaskCollection functions
TaskCollection* task_collection_new() {
    TaskCollection* collection = malloc(sizeof(TaskCollection));
    if (collection == NULL) {
        return NULL;
    }
    collection->tasks = NULL;
    collection->tasks_count = 0;
    return collection;
}

void task_collection_add_task(TaskCollection* collection, Task* task) {
    if (collection == NULL || task == NULL) {
        return;
    }
    
    Task** new_tasks = realloc(collection->tasks, (collection->tasks_count + 2) * sizeof(Task*));
    if (new_tasks == NULL) {
        return; // realloc failed
    }
    collection->tasks = new_tasks;
    collection->tasks[collection->tasks_count] = task;
    collection->tasks[collection->tasks_count + 1] = NULL;
    collection->tasks_count++;
}

Task* task_collection_get_task(TaskCollection* collection, const char* task_id) {
    if (collection == NULL || task_id == NULL || collection->tasks == NULL) {
        return NULL;
    }
    
    for (int i = 0; i < collection->tasks_count; i++) {
        if (collection->tasks[i] != NULL && 
            collection->tasks[i]->id != NULL && 
            strcmp(collection->tasks[i]->id, task_id) == 0) {
            return collection->tasks[i];
        }
    }
    return NULL;
}

Task** task_collection_get_all_tasks(TaskCollection* collection, int* count) {
    if (collection == NULL || count == NULL) {
        return NULL;
    }
    *count = collection->tasks_count;
    return collection->tasks;
}

Task* task_collection_remove_task(TaskCollection* collection, const char* task_id) {
    if (collection == NULL || task_id == NULL || collection->tasks == NULL) {
        return NULL;
    }
    
    for (int i = 0; i < collection->tasks_count; i++) {
        if (collection->tasks[i] != NULL && 
            collection->tasks[i]->id != NULL && 
            strcmp(collection->tasks[i]->id, task_id) == 0) {
            Task* removed = collection->tasks[i];
            // Shift remaining elements
            for (int j = i; j < collection->tasks_count - 1; j++) {
                collection->tasks[j] = collection->tasks[j + 1];
            }
            collection->tasks[collection->tasks_count - 1] = NULL;
            collection->tasks_count--;
            return removed;
        }
    }
    return NULL;
}

Task** task_collection_get_filtered_tasks(TaskCollection* collection, 
                                          Priority* priority_filter,
                                          const char* project_filter,
                                          Status* status_filter,
                                          int* count) {
    if (collection == NULL || count == NULL) {
        return NULL;
    }
    
    Task** filtered = malloc((collection->tasks_count + 1) * sizeof(Task*));
    if (filtered == NULL) {
        *count = 0;
        return NULL;
    }
    
    int filtered_count = 0;
    
    if (collection->tasks != NULL) {
        for (int i = 0; i < collection->tasks_count; i++) {
            Task* task = collection->tasks[i];
            if (task == NULL) {
                continue;
            }
            
            int match = 1;
            
            if (priority_filter != NULL && task->priority != *priority_filter) {
                match = 0;
            }
            
            if (project_filter != NULL && 
                (task->parent_project == NULL || strcmp(task->parent_project, project_filter) != 0)) {
                match = 0;
            }
            
            if (status_filter != NULL && task->status != *status_filter) {
                match = 0;
            }
            
            if (match) {
                filtered[filtered_count++] = task;
            }
        }
    }
    
    filtered[filtered_count] = NULL;
    *count = filtered_count;
    return filtered;
}

void task_collection_free(TaskCollection* collection) {
    if (collection == NULL) return;
    // Note: Tasks are not freed here as they might be owned elsewhere
    free(collection->tasks);
    free(collection);
}

// Config functions
Config* config_default() {
    Config* config = malloc(sizeof(Config));
    if (config == NULL) {
        return NULL;
    }
    
    // Initialize all fields to safe defaults first
    memset(config, 0, sizeof(Config));
    
    config->version = string_clone("1.2.0");
    if (config->version == NULL) {
        free(config);
        return NULL;
    }
    
    config->default_project = string_clone("general");
    if (config->default_project == NULL) {
        free(config->version);
        free(config);
        return NULL;
    }
    
    config->auto_backup = 1;
    
    config->backup_interval = string_clone("daily");
    if (config->backup_interval == NULL) {
        free(config->default_project);
        free(config->version);
        free(config);
        return NULL;
    }
    
    config->ai_enabled = 1;
    
    config->default_assignee = malloc(sizeof(Assignee));
    if (config->default_assignee == NULL) {
        free(config->backup_interval);
        free(config->default_project);
        free(config->version);
        free(config);
        return NULL;
    }
    *config->default_assignee = ASSIGNEE_COLLABORATIVE;
    
    config->date_format = string_clone("%Y-%m-%d %H:%M:%S");
    if (config->date_format == NULL) {
        free(config->default_assignee);
        free(config->backup_interval);
        free(config->default_project);
        free(config->version);
        free(config);
        return NULL;
    }
    
    config->timezone = string_clone("UTC");
    if (config->timezone == NULL) {
        free(config->date_format);
        free(config->default_assignee);
        free(config->backup_interval);
        free(config->default_project);
        free(config->version);
        free(config);
        return NULL;
    }
    
    return config;
}

void config_free(Config* config) {
    if (config == NULL) return;
    free(config->version);
    free(config->default_project);
    free(config->backup_interval);
    free(config->default_assignee);
    free(config->date_format);
    free(config->timezone);
    free(config);
}

// Parsing functions
int parse_priority(const char* str, Priority* result) {
    if (str == NULL || result == NULL) {
        return 0;
    }
    
    if (strcmp(str, "low") == 0) {
        *result = PRIORITY_LOW;
        return 1;
    } else if (strcmp(str, "medium") == 0) {
        *result = PRIORITY_MEDIUM;
        return 1;
    } else if (strcmp(str, "high") == 0) {
        *result = PRIORITY_HIGH;
        return 1;
    } else if (strcmp(str, "critical") == 0) {
        *result = PRIORITY_CRITICAL;
        return 1;
    } else if (strcmp(str, "urgent") == 0) {
        *result = PRIORITY_URGENT;
        return 1;
    }
    return 0; // Error
}

int parse_status(const char* str, Status* result) {
    if (str == NULL || result == NULL) {
        return 0;
    }
    
    if (strcmp(str, "todo") == 0) {
        *result = STATUS_TODO;
        return 1;
    } else if (strcmp(str, "in_progress") == 0 || strcmp(str, "in-progress") == 0) {
        *result = STATUS_IN_PROGRESS;
        return 1;
    } else if (strcmp(str, "blocked") == 0) {
        *result = STATUS_BLOCKED;
        return 1;
    } else if (strcmp(str, "review") == 0) {
        *result = STATUS_REVIEW;
        return 1;
    } else if (strcmp(str, "done") == 0) {
        *result = STATUS_DONE;
        return 1;
    } else if (strcmp(str, "cancelled") == 0 || strcmp(str, "canceled") == 0) {
        *result = STATUS_CANCELLED;
        return 1;
    } else if (strcmp(str, "deferred") == 0) {
        *result = STATUS_DEFERRED;
        return 1;
    }
    return 0; // Error
}

int parse_assignee(const char* str, Assignee* result) {
    if (str == NULL || result == NULL) {
        return 0;
    }
    
    if (strcmp(str, "ai") == 0) {
        *result = ASSIGNEE_AI;
        return 1;
    } else if (strcmp(str, "human") == 0) {
        *result = ASSIGNEE_HUMAN;
        return 1;
    } else if (strcmp(str, "collaborative") == 0) {
        *result = ASSIGNEE_COLLABORATIVE;
        return 1;
    }
    return 0; // Error
}

// Test functions
void test_task_creation() {
    Task* task = task_new("user_123", "Test task", "1 hour", PRIORITY_MEDIUM, "test-project", STATUS_TODO);
    
    assert(strcmp(task->action, "Test task") == 0);
    assert(strcmp(task->time, "1 hour") == 0);
    assert(task->priority == PRIORITY_MEDIUM);
    assert(strcmp(task->parent_project, "test-project") == 0);
    assert(task->status == STATUS_TODO);
    assert(strncmp(task->id, "task_", 5) == 0);
    assert(task->assignee == NULL);
    assert(task->tags == NULL);
    assert(task->tags_count == 0);
    assert(task->dependencies == NULL);
    assert(task->dependencies_count == 0);
    assert(task->context_notes == NULL);
    assert(task->progress == NULL);
    
    task_free(task);
    printf("test_task_creation passed\n");
}

void test_task_creation_full() {
    char* tags[] = {"test", "example", NULL};
    char* dependencies[] = {"task_001", NULL};
    int progress = 50;
    Assignee assignee = ASSIGNEE_HUMAN;
    
    Task* task = task_new_full("Test task", "2 hours", PRIORITY_HIGH, "test-project", 
                              STATUS_IN_PROGRESS, &assignee, tags, 2, dependencies, 1,
                              "Test context", &progress);
    
    assert(task != NULL);
    assert(strcmp(task->action, "Test task") == 0);
    assert(strcmp(task->time, "2 hours") == 0);
    assert(task->priority == PRIORITY_HIGH);
    assert(strcmp(task->parent_project, "test-project") == 0);
    assert(task->status == STATUS_IN_PROGRESS);
    assert(strcmp(task->assignee, "human") == 0);
    assert(task->tags_count == 2);
    assert(strcmp(task->tags[0], "test") == 0);
    assert(strcmp(task->tags[1], "example") == 0);
    assert(task->dependencies_count == 1);
    assert(strcmp(task->dependencies[0], "task_001") == 0);
    assert(strcmp(task->context_notes, "Test context") == 0);
    assert(*task->progress == 50);
    
    task_free(task);
    printf("test_task_creation_full passed\n");
}

void test_task_creation_invalid_progress() {
    int progress = 150;
    Task* task = task_new_full("Test task", "1 hour", PRIORITY_MEDIUM, "test-project",
                              STATUS_TODO, NULL, NULL, 0, NULL, 0, NULL, &progress);
    
    assert(task == NULL);
    printf("test_task_creation_invalid_progress passed\n");
}

void test_task_update() {
    Task* task = task_new("user_123", "Original task", "1 hour", PRIORITY_LOW, "test-project", STATUS_TODO);
    
    char* new_action = "Updated task";
    Priority new_priority = PRIORITY_HIGH;
    Status new_status = STATUS_IN_PROGRESS;
    int new_progress = 75;
    
    task_update(task, new_action, &new_priority, &new_status, &new_progress);
    
    assert(strcmp(task->action, "Updated task") == 0);
    assert(task->priority == PRIORITY_HIGH);
    assert(task->status == STATUS_IN_PROGRESS);
    assert(*task->progress == 75);
    
    task_free(task);
    printf("test_task_update passed\n");
}

void test_task_complete() {
    Task* task = task_new("user_123", "Test task", "1 hour", PRIORITY_MEDIUM, "test-project", STATUS_TODO);
    
    task_complete(task);
    
    assert(task->status == STATUS_DONE);
    assert(*task->progress == 100);
    assert(task_is_completed(task));
    
    task_free(task);
    printf("test_task_complete passed\n");
}

void test_task_is_active() {
    Task* active_task = task_new("user_123", "Active task", "1 hour", PRIORITY_MEDIUM, "test-project", STATUS_TODO);
    Task* completed_task = task_new("user_123", "Completed task", "1 hour", PRIORITY_MEDIUM, "test-project", STATUS_TODO);
    task_complete(completed_task);
    Task* cancelled_task = task_new("user_123", "Cancelled task", "1 hour", PRIORITY_MEDIUM, "test-project", STATUS_CANCELLED);
    
    assert(task_is_active(active_task));
    assert(!task_is_active(completed_task));
    assert(!task_is_active(cancelled_task));
    
    task_free(active_task);
    task_free(completed_task);
    task_free(cancelled_task);
    printf("test_task_is_active passed\n");
}

void test_priority_parsing() {
    Priority priority;
    
    assert(parse_priority("low", &priority) && priority == PRIORITY_LOW);
    assert(parse_priority("medium", &priority) && priority == PRIORITY_MEDIUM);
    assert(parse_priority("high", &priority) && priority == PRIORITY_HIGH);
    assert(parse_priority("critical", &priority) && priority == PRIORITY_CRITICAL);
    assert(parse_priority("urgent", &priority) && priority == PRIORITY_URGENT);
    assert(!parse_priority("invalid", &priority));
    
    printf("test_priority_parsing passed\n");
}

void test_status_parsing() {
    Status status;
    
    assert(parse_status("todo", &status) && status == STATUS_TODO);
    assert(parse_status("in_progress", &status) && status == STATUS_IN_PROGRESS);
    assert(parse_status("in-progress", &status) && status == STATUS_IN_PROGRESS);
    assert(parse_status("blocked", &status) && status == STATUS_BLOCKED);
    assert(parse_status("review", &status) && status == STATUS_REVIEW);
    assert(parse_status("done", &status) && status == STATUS_DONE);
    assert(parse_status("cancelled", &status) && status == STATUS_CANCELLED);
    assert(parse_status("canceled", &status) && status == STATUS_CANCELLED);
    assert(parse_status("deferred", &status) && status == STATUS_DEFERRED);
    assert(!parse_status("invalid", &status));
    
    printf("test_status_parsing passed\n");
}

void test_assignee_parsing() {
    Assignee assignee;
    
    assert(parse_assignee("ai", &assignee) && assignee == ASSIGNEE_AI);
    assert(parse_assignee("human", &assignee) && assignee == ASSIGNEE_HUMAN);
    assert(parse_assignee("collaborative", &assignee) && assignee == ASSIGNEE_COLLABORATIVE);
    assert(!parse_assignee("invalid", &assignee));
    
    printf("test_assignee_parsing passed\n");
}

void test_project_creation() {
    Project* project = project_new("test-project", "Test project description");
    
    assert(strcmp(project->name, "test-project") == 0);
    assert(strcmp(project->description, "Test project description") == 0);
    assert(project->status == PROJECT_STATUS_ACTIVE);
    assert(project->tasks == NULL);
    assert(project->tasks_count == 0);
    
    project_free(project);
    printf("test_project_creation passed\n");
}

void test_project_add_task() {
    Project* project = project_new("test-project", NULL);
    
    project_add_task(project, "task_001");
    project_add_task(project, "task_002");
    project_add_task(project, "task_001"); // Duplicate
    
    assert(project->tasks_count == 2);
    assert(strcmp(project->tasks[0], "task_001") == 0);
    assert(strcmp(project->tasks[1], "task_002") == 0);
    
    project_free(project);
    printf("test_project_add_task passed\n");
}

void test_project_remove_task() {
    Project* project = project_new("test-project", NULL);
    
    project_add_task(project, "task_001");
    project_add_task(project, "task_002");
    project_remove_task(project, "task_001");
    
    assert(project->tasks_count == 1);
    assert(strcmp(project->tasks[0], "task_002") == 0);
    
    project_free(project);
    printf("test_project_remove_task passed\n");
}

void test_project_archive() {
    Project* project = project_new("test-project", NULL);
    
    project_archive(project);
    
    assert(project->status == PROJECT_STATUS_ARCHIVED);
    
    project_free(project);
    printf("test_project_archive passed\n");
}

void test_project_complete() {
    Project* project = project_new("test-project", NULL);
    
    project_complete(project);
    
    assert(project->status == PROJECT_STATUS_COMPLETED);
    
    project_free(project);
    printf("test_project_complete passed\n");
}

void test_task_collection() {
    TaskCollection* collection = task_collection_new();
    
    Task* task1 = task_new("user_123", "Task 1", "1 hour", PRIORITY_LOW, "project1", STATUS_TODO);
    Task* task2 = task_new("user_123", "Task 2", "2 hours", PRIORITY_HIGH, "project2", STATUS_IN_PROGRESS);
    
    task_collection_add_task(collection, task1);
    task_collection_add_task(collection, task2);
    
    assert(collection->tasks_count == 2);
    assert(task_collection_get_task(collection, task1->id) != NULL);
    assert(task_collection_get_task(collection, task2->id) != NULL);
    assert(task_collection_get_task(collection, "nonexistent") == NULL);
    
    int all_count;
    Task** all_tasks = task_collection_get_all_tasks(collection, &all_count);
    assert(all_count == 2);
    assert(all_tasks != NULL);
    (void)all_tasks; // Used in assertion above
    
    Task* removed_task = task_collection_remove_task(collection, task1->id);
    assert(removed_task != NULL);
    assert(collection->tasks_count == 1);
    
    task_free(task1);
    task_free(task2);
    task_collection_free(collection);
    printf("test_task_collection passed\n");
}

void test_task_collection_filtering() {
    TaskCollection* collection = task_collection_new();
    
    Task* task1 = task_new("user_123", "Low priority task", "1 hour", PRIORITY_LOW, "project1", STATUS_TODO);
    Task* task2 = task_new("user_123", "High priority task", "2 hours", PRIORITY_HIGH, "project2", STATUS_IN_PROGRESS);
    
    task_collection_add_task(collection, task1);
    task_collection_add_task(collection, task2);
    
    // Test priority filtering
    Priority high_priority = PRIORITY_HIGH;
    int high_priority_count;
    Task** high_priority_tasks = task_collection_get_filtered_tasks(collection, &high_priority, NULL, NULL, &high_priority_count);
    assert(high_priority_count == 1);
    assert(high_priority_tasks[0]->priority == PRIORITY_HIGH);
    free(high_priority_tasks);
    
    // Test project filtering
    int project1_count;
    Task** project1_tasks = task_collection_get_filtered_tasks(collection, NULL, "project1", NULL, &project1_count);
    assert(project1_count == 1);
    assert(strcmp(project1_tasks[0]->parent_project, "project1") == 0);
    free(project1_tasks);
    
    // Test status filtering
    Status todo_status = STATUS_TODO;
    int todo_count;
    Task** todo_tasks = task_collection_get_filtered_tasks(collection, NULL, NULL, &todo_status, &todo_count);
    assert(todo_count == 1);
    assert(todo_tasks[0]->status == STATUS_TODO);
    free(todo_tasks);
    
    task_free(task1);
    task_free(task2);
    task_collection_free(collection);
    printf("test_task_collection_filtering passed\n");
}

void test_config_default() {
    Config* config = config_default();
    
    assert(strcmp(config->version, "1.2.0") == 0);
    assert(strcmp(config->default_project, "general") == 0);
    assert(config->auto_backup == 1);
    assert(strcmp(config->backup_interval, "daily") == 0);
    assert(config->ai_enabled == 1);
    assert(*config->default_assignee == ASSIGNEE_COLLABORATIVE);
    assert(strcmp(config->date_format, "%Y-%m-%d %H:%M:%S") == 0);
    assert(strcmp(config->timezone, "UTC") == 0);
    
    config_free(config);
    printf("test_config_default passed\n");
}

void test_task_update_validation() {
    Task* task = task_new("user_123", "Test task", "1 hour", PRIORITY_MEDIUM, "test-project", STATUS_TODO);
    
    // Test invalid progress
    int invalid_progress = 150;
    task_update(task, NULL, NULL, NULL, &invalid_progress);
    // In C we can't easily return errors, so we just don't update
    assert(task->progress == NULL);
    
    // Test valid progress
    int valid_progress = 75;
    task_update(task, NULL, NULL, NULL, &valid_progress);
    assert(*task->progress == 75);
    
    task_free(task);
    printf("test_task_update_validation passed\n");
}

int main() {
    test_task_creation();
    test_task_creation_full();
    test_task_creation_invalid_progress();
    test_task_update();
    test_task_complete();
    test_task_is_active();
    test_priority_parsing();
    test_status_parsing();
    test_assignee_parsing();
    test_project_creation();
    test_project_add_task();
    test_project_remove_task();
    test_project_archive();
    test_project_complete();
    test_task_collection();
    test_task_collection_filtering();
    test_config_default();
    test_task_update_validation();
    
    printf("All tests passed!\n");
    return 0;
}