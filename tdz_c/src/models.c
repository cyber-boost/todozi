#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <stdint.h>
#include <uuid/uuid.h>
#include "openssl/sha.h"
#include "openssl/rand.h"

// Error handling
typedef enum {
    TODOZI_OK = 0,
    TODOZI_ERR_ALLOC,
    TODOZI_ERR_INVALID,
    TODOZI_ERR_NOT_FOUND,
    TODOZI_ERR_INTERNAL
} TodoziResult;

typedef struct {
    TodoziResult code;
    char *msg;
} TodoziError;

// Forward declarations for opaque types
typedef struct Task Task;
typedef struct TaskUpdate TaskUpdate;
typedef struct Project Project;
typedef struct Config Config;
typedef struct RegistrationInfo RegistrationInfo;

// Enum definitions
typedef enum {
    PRIORITY_LOW,
    PRIORITY_MEDIUM,
    PRIORITY_HIGH,
    PRIORITY_CRITICAL,
    PRIORITY_URGENT
} Priority;

typedef enum {
    STATUS_TODO,
    STATUS_PENDING,
    STATUS_IN_PROGRESS,
    STATUS_BLOCKED,
    STATUS_REVIEW,
    STATUS_DONE,
    STATUS_COMPLETED,
    STATUS_CANCELLED,
    STATUS_DEFERRED
} Status;

typedef enum {
    ASSIGNEE_AI,
    ASSIGNEE_HUMAN,
    ASSIGNEE_COLLABORATIVE,
    ASSIGNEE_AGENT
} AssigneeType;

typedef enum {
    PROJECT_STATUS_ACTIVE,
    PROJECT_STATUS_ARCHIVED,
    PROJECT_STATUS_COMPLETED
} ProjectStatus;

// Function prototypes
// Priority functions
TodoziResult todozi_priority_parse(const char* s, Priority* out);
size_t todozi_priority_to_string(Priority p, char* buf, size_t buflen);

// Status functions
TodoziResult todozi_status_parse(const char* s, Status* out);
size_t todozi_status_to_string(Status s, char* buf, size_t buflen);

// Assignee functions
TodoziResult todozi_assignee_parse(const char* s, AssigneeType* out, char** agent_name);
size_t todozi_assignee_to_string(AssigneeType a, const char* agent_name, char* buf, size_t buflen);

// ProjectStatus functions
TodoziResult todozi_project_status_parse(const char* s, ProjectStatus* out);
size_t todozi_project_status_to_string(ProjectStatus p, char* buf, size_t buflen);

// Task functions
TodoziResult todozi_task_new(const char* user_id, const char* action, const char* time_str,
                            Priority priority, const char* parent_project, Status status,
                            Task** out, TodoziError* err);
TodoziResult todozi_task_new_full(const char* user_id, const char* action, const char* time_str,
                                 Priority priority, const char* parent_project, Status status,
                                 AssigneeType assignee_type, const char* assignee_agent_name,
                                 char** tags, size_t tags_count, char** dependencies, size_t dependencies_count,
                                 const char* context_notes, const uint8_t* progress,
                                 Task** out, TodoziError* err);
void todozi_task_free(Task* task);
TodoziResult todozi_task_update(Task* task, const TaskUpdate* updates, TodoziError* err);
void todozi_task_complete(Task* task);
bool todozi_task_is_completed(const Task* task);
bool todozi_task_is_active(const Task* task);

// Task accessors
const char* todozi_task_id(const Task* task);
const char* todozi_task_user_id(const Task* task);
const char* todozi_task_action(const Task* task);
const char* todozi_task_time(const Task* task);
Priority todozi_task_priority(const Task* task);
const char* todozi_task_parent_project(const Task* task);
Status todozi_task_status(const Task* task);
AssigneeType todozi_task_assignee_type(const Task* task);
const char* todozi_task_assignee_agent_name(const Task* task);
const char* const* todozi_task_tags(const Task* task, size_t* count);
const char* const* todozi_task_dependencies(const Task* task, size_t* count);
const char* todozi_task_context_notes(const Task* task);
const uint8_t* todozi_task_progress(const Task* task);
time_t todozi_task_created_at(const Task* task);
time_t todozi_task_updated_at(const Task* task);

// TaskUpdate functions
TodoziResult todozi_task_update_new(TaskUpdate** out, TodoziError* err);
void todozi_task_update_free(TaskUpdate* update);
TodoziResult todozi_task_update_with_action(TaskUpdate* update, const char* action, TodoziError* err);
TodoziResult todozi_task_update_with_time(TaskUpdate* update, const char* time, TodoziError* err);
TodoziResult todozi_task_update_with_priority(TaskUpdate* update, Priority priority, TodoziError* err);
TodoziResult todozi_task_update_with_parent_project(TaskUpdate* update, const char* parent_project, TodoziError* err);
TodoziResult todozi_task_update_with_status(TaskUpdate* update, Status status, TodoziError* err);
TodoziResult todozi_task_update_with_assignee(TaskUpdate* update, AssigneeType assignee_type, const char* agent_name, TodoziError* err);
TodoziResult todozi_task_update_with_tags(TaskUpdate* update, char** tags, size_t tags_count, TodoziError* err);
TodoziResult todozi_task_update_with_dependencies(TaskUpdate* update, char** dependencies, size_t dependencies_count, TodoziError* err);
TodoziResult todozi_task_update_with_context_notes(TaskUpdate* update, const char* context_notes, TodoziError* err);
TodoziResult todozi_task_update_with_progress(TaskUpdate* update, uint8_t progress, TodoziError* err);

// Project functions
TodoziResult todozi_project_new(const char* name, const char* description, Project** out, TodoziError* err);
void todozi_project_free(Project* project);
void todozi_project_add_task(Project* project, const char* task_id);
void todozi_project_remove_task(Project* project, const char* task_id);
void todozi_project_archive(Project* project);
void todozi_project_complete(Project* project);

// Config functions
TodoziResult todozi_config_default(Config** out, TodoziError* err);
void todozi_config_free(Config* config);

// RegistrationInfo functions
TodoziResult todozi_registration_info_new(const char* user_name, const char* user_email,
                                         const char* api_key, const char* server_url,
                                         RegistrationInfo** out, TodoziError* err);
TodoziResult todozi_registration_info_new_with_hashes(const char* server_url,
                                                     RegistrationInfo** out, TodoziError* err);
void todozi_registration_info_free(RegistrationInfo* reg);

// Utility functions
char* todozi_strdup_or_null(const char* src);
void todozi_free_string_array(char** arr, size_t n);
TodoziResult todozi_alloc_array(size_t count, size_t elem_sz, void** out, TodoziError* err);
char* todozi_generate_short_uuid(void);
char* todozi_generate_sha256(const char* input);
char* todozi_generate_sha512(const char* input);

// Helper functions
static TodoziResult alloc_and_copy_string(const char* src, char** dst, TodoziError* err) {
    if (!dst) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid destination pointer");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (!src) {
        *dst = NULL;
        return TODOZI_OK;
    }
    
    *dst = todozi_strdup_or_null(src);
    if (!*dst) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate string");
        }
        return TODOZI_ERR_ALLOC;
    }
    return TODOZI_OK;
}

static TodoziResult copy_string_array(const char** src, size_t n, char*** dst, size_t* dst_count, TodoziError* err) {
    if (!dst || !dst_count) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid destination pointers");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (n == 0) {
        *dst = NULL;
        *dst_count = 0;
        return TODOZI_OK;
    }
    
    if (!src) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid source array");
        }
        return TODOZI_ERR_INVALID;
    }
    
    *dst = malloc(n * sizeof(char*));
    if (!*dst) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate string array");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    for (size_t i = 0; i < n; ++i) {
        (*dst)[i] = todozi_strdup_or_null(src[i]);
        if (!(*dst)[i]) {
            // Roll back allocations
            while (i--) {
                free((*dst)[i]);
            }
            free(*dst);
            *dst = NULL;
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to copy string array elements");
            }
            return TODOZI_ERR_ALLOC;
        }
    }
    
    *dst_count = n;
    return TODOZI_OK;
}

// Priority implementation
TodoziResult todozi_priority_parse(const char* s, Priority* out) {
    if (!s || !out) return TODOZI_ERR_INVALID;
    
    if (strcmp(s, "low") == 0) {
        *out = PRIORITY_LOW;
        return TODOZI_OK;
    }
    if (strcmp(s, "medium") == 0) {
        *out = PRIORITY_MEDIUM;
        return TODOZI_OK;
    }
    if (strcmp(s, "high") == 0) {
        *out = PRIORITY_HIGH;
        return TODOZI_OK;
    }
    if (strcmp(s, "critical") == 0) {
        *out = PRIORITY_CRITICAL;
        return TODOZI_OK;
    }
    if (strcmp(s, "urgent") == 0) {
        *out = PRIORITY_URGENT;
        return TODOZI_OK;
    }
    
    return TODOZI_ERR_INVALID;
}

size_t todozi_priority_to_string(Priority p, char* buf, size_t buflen) {
    const char* str;
    switch (p) {
        case PRIORITY_LOW: str = "low"; break;
        case PRIORITY_MEDIUM: str = "medium"; break;
        case PRIORITY_HIGH: str = "high"; break;
        case PRIORITY_CRITICAL: str = "critical"; break;
        case PRIORITY_URGENT: str = "urgent"; break;
        default: str = "medium"; break;
    }
    
    if (buf && buflen > 0) {
        strncpy(buf, str, buflen - 1);
        buf[buflen - 1] = '\0';
    }
    return strlen(str);
}

// Status implementation
TodoziResult todozi_status_parse(const char* s, Status* out) {
    if (!s || !out) return TODOZI_ERR_INVALID;
    
    if (strcmp(s, "todo") == 0 || strcmp(s, "pending") == 0) {
        *out = STATUS_TODO;
        return TODOZI_OK;
    }
    if (strcmp(s, "in_progress") == 0 || strcmp(s, "in-progress") == 0) {
        *out = STATUS_IN_PROGRESS;
        return TODOZI_OK;
    }
    if (strcmp(s, "blocked") == 0) {
        *out = STATUS_BLOCKED;
        return TODOZI_OK;
    }
    if (strcmp(s, "review") == 0) {
        *out = STATUS_REVIEW;
        return TODOZI_OK;
    }
    if (strcmp(s, "done") == 0 || strcmp(s, "completed") == 0) {
        *out = STATUS_DONE;
        return TODOZI_OK;
    }
    if (strcmp(s, "cancelled") == 0 || strcmp(s, "canceled") == 0) {
        *out = STATUS_CANCELLED;
        return TODOZI_OK;
    }
    if (strcmp(s, "deferred") == 0) {
        *out = STATUS_DEFERRED;
        return TODOZI_OK;
    }
    
    return TODOZI_ERR_INVALID;
}

size_t todozi_status_to_string(Status s, char* buf, size_t buflen) {
    const char* str;
    switch (s) {
        case STATUS_TODO:
        case STATUS_PENDING:
            str = "todo";
            break;
        case STATUS_IN_PROGRESS:
            str = "in_progress";
            break;
        case STATUS_BLOCKED:
            str = "blocked";
            break;
        case STATUS_REVIEW:
            str = "review";
            break;
        case STATUS_DONE:
        case STATUS_COMPLETED:
            str = "done";
            break;
        case STATUS_CANCELLED:
            str = "cancelled";
            break;
        case STATUS_DEFERRED:
            str = "deferred";
            break;
        default:
            str = "todo";
            break;
    }
    
    if (buf && buflen > 0) {
        strncpy(buf, str, buflen - 1);
        buf[buflen - 1] = '\0';
    }
    return strlen(str);
}

// Assignee implementation
TodoziResult todozi_assignee_parse(const char* s, AssigneeType* out, char** agent_name) {
    if (!s || !out || !agent_name) return TODOZI_ERR_INVALID;
    
    if (strcmp(s, "ai") == 0) {
        *out = ASSIGNEE_AI;
        *agent_name = NULL;
        return TODOZI_OK;
    }
    if (strcmp(s, "human") == 0) {
        *out = ASSIGNEE_HUMAN;
        *agent_name = NULL;
        return TODOZI_OK;
    }
    if (strcmp(s, "collaborative") == 0) {
        *out = ASSIGNEE_COLLABORATIVE;
        *agent_name = NULL;
        return TODOZI_OK;
    }
    
    if (strncmp(s, "agent:", 6) == 0) {
        *out = ASSIGNEE_AGENT;
        *agent_name = todozi_strdup_or_null(s + 6);
        if (!*agent_name) return TODOZI_ERR_ALLOC;
        return TODOZI_OK;
    }
    
    *out = ASSIGNEE_AGENT;
    *agent_name = todozi_strdup_or_null(s);
    if (!*agent_name) return TODOZI_ERR_ALLOC;
    return TODOZI_OK;
}

size_t todozi_assignee_to_string(AssigneeType a, const char* agent_name, char* buf, size_t buflen) {
    char temp_buf[512];  // Increased buffer size for safety
    const char* str;
    size_t str_len;
    
    switch (a) {
        case ASSIGNEE_AI: 
            str = "ai"; 
            str_len = 2;
            break;
        case ASSIGNEE_HUMAN: 
            str = "human"; 
            str_len = 5;
            break;
        case ASSIGNEE_COLLABORATIVE: 
            str = "collaborative"; 
            str_len = 13;
            break;
        case ASSIGNEE_AGENT:
            if (agent_name) {
                int written = snprintf(temp_buf, sizeof(temp_buf), "agent:%s", agent_name);
                if (written < 0 || (size_t)written >= sizeof(temp_buf)) {
                    // Truncation occurred, ensure null termination
                    temp_buf[sizeof(temp_buf) - 1] = '\0';
                    str_len = sizeof(temp_buf) - 1;
                } else {
                    str_len = (size_t)written;
                }
                str = temp_buf;
            } else {
                str = "agent:";
                str_len = 6;
            }
            break;
        default: 
            str = "human"; 
            str_len = 5;
            break;
    }
    
    if (buf && buflen > 0) {
        strncpy(buf, str, buflen - 1);
        buf[buflen - 1] = '\0';
    }
    return str_len;
}

// ProjectStatus implementation
TodoziResult todozi_project_status_parse(const char* s, ProjectStatus* out) {
    if (!s || !out) return TODOZI_ERR_INVALID;
    
    if (strcmp(s, "active") == 0) {
        *out = PROJECT_STATUS_ACTIVE;
        return TODOZI_OK;
    }
    if (strcmp(s, "archived") == 0) {
        *out = PROJECT_STATUS_ARCHIVED;
        return TODOZI_OK;
    }
    if (strcmp(s, "completed") == 0) {
        *out = PROJECT_STATUS_COMPLETED;
        return TODOZI_OK;
    }
    
    return TODOZI_ERR_INVALID;
}

size_t todozi_project_status_to_string(ProjectStatus p, char* buf, size_t buflen) {
    const char* str;
    switch (p) {
        case PROJECT_STATUS_ACTIVE: str = "active"; break;
        case PROJECT_STATUS_ARCHIVED: str = "archived"; break;
        case PROJECT_STATUS_COMPLETED: str = "completed"; break;
        default: str = "active"; break;
    }
    
    if (buf && buflen > 0) {
        strncpy(buf, str, buflen - 1);
        buf[buflen - 1] = '\0';
    }
    return strlen(str);
}

// Task implementation
struct Task {
    char* id;
    char* user_id;
    char* action;
    char* time;
    Priority priority;
    char* parent_project;
    Status status;
    AssigneeType assignee_type;
    char* assignee_agent_name;
    char** tags;
    size_t tags_count;
    char** dependencies;
    size_t dependencies_count;
    char* context_notes;
    uint8_t* progress;
    float* embedding_vector;
    size_t embedding_vector_size;
    time_t created_at;
    time_t updated_at;
};

struct TaskUpdate {
    char* action;
    char* time;
    Priority* priority;
    char* parent_project;
    Status* status;
    AssigneeType* assignee_type;
    char* assignee_agent_name;
    char** tags;
    size_t tags_count;
    char** dependencies;
    size_t dependencies_count;
    char* context_notes;
    uint8_t* progress;
    float* embedding_vector;
    size_t embedding_vector_size;
};

TodoziResult todozi_task_new(const char* user_id, const char* action, const char* time_str,
                            Priority priority, const char* parent_project, Status status,
                            Task** out, TodoziError* err) {
    // Get current time
    time_t now = time(NULL);
    
    if (!user_id || !action || !time_str || !parent_project || !out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    Task* task = calloc(1, sizeof(Task));
    if (!task) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate task");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    task->id = todozi_generate_short_uuid();
    if (!task->id) goto oom;
    
    if (alloc_and_copy_string(user_id, &task->user_id, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(action, &task->action, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(time_str, &task->time, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(parent_project, &task->parent_project, NULL) != TODOZI_OK) goto oom;
    
    task->priority = priority;
    task->status = status;
    task->assignee_type = ASSIGNEE_HUMAN; // Default
    task->created_at = now;
    task->updated_at = now;
    
    *out = task;
    return TODOZI_OK;
    
oom:
    todozi_task_free(task);
    if (err) {
        err->code = TODOZI_ERR_ALLOC;
        err->msg = todozi_strdup_or_null("Failed to allocate task fields");
    }
    return TODOZI_ERR_ALLOC;
}

TodoziResult todozi_task_new_full(const char* user_id, const char* action, const char* time_str,
                                 Priority priority, const char* parent_project, Status status,
                                 AssigneeType assignee_type, const char* assignee_agent_name,
                                 char** tags, size_t tags_count, char** dependencies, size_t dependencies_count,
                                 const char* context_notes, const uint8_t* progress,
                                 Task** out, TodoziError* err) {
    // Get current time
    time_t now = time(NULL);
    
    if (!user_id || !action || !time_str || !parent_project || !out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (progress && *progress > 100) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Progress must be between 0 and 100");
        }
        return TODOZI_ERR_INVALID;
    }
    
    Task* task = calloc(1, sizeof(Task));
    if (!task) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate task");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    task->id = todozi_generate_short_uuid();
    if (!task->id) goto oom;
    
    if (alloc_and_copy_string(user_id, &task->user_id, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(action, &task->action, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(time_str, &task->time, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(parent_project, &task->parent_project, NULL) != TODOZI_OK) goto oom;
    
    task->priority = priority;
    task->status = status;
    task->assignee_type = assignee_type;
    
    if (assignee_agent_name) {
        if (alloc_and_copy_string(assignee_agent_name, &task->assignee_agent_name, NULL) != TODOZI_OK) goto oom;
    }
    
    if (copy_string_array((const char**)tags, tags_count, &task->tags, &task->tags_count, NULL) != TODOZI_OK) goto oom;
    if (copy_string_array((const char**)dependencies, dependencies_count, &task->dependencies, &task->dependencies_count, NULL) != TODOZI_OK) goto oom;
    
    if (context_notes) {
        if (alloc_and_copy_string(context_notes, &task->context_notes, NULL) != TODOZI_OK) goto oom;
    }
    
    if (progress) {
        task->progress = malloc(sizeof(uint8_t));
        if (!task->progress) goto oom;
        *task->progress = *progress;
    }
    
    task->created_at = now;
    task->updated_at = now;
    
    *out = task;
    return TODOZI_OK;
    
oom:
    todozi_task_free(task);
    if (err) {
        err->code = TODOZI_ERR_ALLOC;
        err->msg = todozi_strdup_or_null("Failed to allocate task fields");
    }
    return TODOZI_ERR_ALLOC;
}

void todozi_task_free(Task* task) {
    if (!task) return;
    
    free(task->id);
    free(task->user_id);
    free(task->action);
    free(task->time);
    free(task->parent_project);
    free(task->assignee_agent_name);
    todozi_free_string_array(task->tags, task->tags_count);
    todozi_free_string_array(task->dependencies, task->dependencies_count);
    free(task->context_notes);
    free(task->progress);
    free(task->embedding_vector);
    free(task);
}

TodoziResult todozi_task_update(Task* task, const TaskUpdate* updates, TodoziError* err) {
    if (!task || !updates) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    // Update action
    if (updates->action) {
        char* temp = todozi_strdup_or_null(updates->action);
        if (!temp) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to update action");
            }
            return TODOZI_ERR_ALLOC;
        }
        free(task->action);
        task->action = temp;
    }
    
    // Update time
    if (updates->time) {
        char* temp = todozi_strdup_or_null(updates->time);
        if (!temp) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to update time");
            }
            return TODOZI_ERR_ALLOC;
        }
        free(task->time);
        task->time = temp;
    }
    
    // Update priority
    if (updates->priority) {
        task->priority = *updates->priority;
    }
    
    // Update parent project
    if (updates->parent_project) {
        char* temp = todozi_strdup_or_null(updates->parent_project);
        if (!temp) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to update parent project");
            }
            return TODOZI_ERR_ALLOC;
        }
        free(task->parent_project);
        task->parent_project = temp;
    }
    
    // Update status
    if (updates->status) {
        task->status = *updates->status;
    }
    
    // Update assignee
    if (updates->assignee_type) {
        task->assignee_type = *updates->assignee_type;
        if (updates->assignee_agent_name) {
            char* temp = todozi_strdup_or_null(updates->assignee_agent_name);
            if (!temp) {
                if (err) {
                    err->code = TODOZI_ERR_ALLOC;
                    err->msg = todozi_strdup_or_null("Failed to update assignee agent name");
                }
                return TODOZI_ERR_ALLOC;
            }
            free(task->assignee_agent_name);
            task->assignee_agent_name = temp;
        }
    }
    
    // Update tags
    if (updates->tags) {
        todozi_free_string_array(task->tags, task->tags_count);
        if (copy_string_array((const char**)updates->tags, updates->tags_count, &task->tags, &task->tags_count, err) != TODOZI_OK) {
            return TODOZI_ERR_ALLOC;
        }
    }
    
    // Update dependencies
    if (updates->dependencies) {
        todozi_free_string_array(task->dependencies, task->dependencies_count);
        if (copy_string_array((const char**)updates->dependencies, updates->dependencies_count, &task->dependencies, &task->dependencies_count, err) != TODOZI_OK) {
            return TODOZI_ERR_ALLOC;
        }
    }
    
    // Update context notes
    if (updates->context_notes) {
        char* temp = todozi_strdup_or_null(updates->context_notes);
        if (!temp) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to update context notes");
            }
            return TODOZI_ERR_ALLOC;
        }
        free(task->context_notes);
        task->context_notes = temp;
    }
    
    // Update progress
    if (updates->progress) {
        if (*updates->progress > 100) {
            if (err) {
                err->code = TODOZI_ERR_INVALID;
                err->msg = todozi_strdup_or_null("Progress must be between 0 and 100");
            }
            return TODOZI_ERR_INVALID;
        }
        
        if (!task->progress) {
            task->progress = malloc(sizeof(uint8_t));
            if (!task->progress) {
                if (err) {
                    err->code = TODOZI_ERR_ALLOC;
                    err->msg = todozi_strdup_or_null("Failed to allocate progress");
                }
                return TODOZI_ERR_ALLOC;
            }
        }
        *task->progress = *updates->progress;
    }
    
    task->updated_at = time(NULL);
    return TODOZI_OK;
}

void todozi_task_complete(Task* task) {
    if (!task) return;
    task->status = STATUS_DONE;
    if (!task->progress) {
        task->progress = malloc(sizeof(uint8_t));
        if (!task->progress) {
            // Allocation failed, but we can still update status
            task->updated_at = time(NULL);
            return;
        }
    }
    *task->progress = 100;
    task->updated_at = time(NULL);
}

bool todozi_task_is_completed(const Task* task) {
    return task && task->status == STATUS_DONE;
}

bool todozi_task_is_active(const Task* task) {
    return task && !(task->status == STATUS_DONE || task->status == STATUS_CANCELLED);
}

// Task accessors
const char* todozi_task_id(const Task* task) { return task ? task->id : NULL; }
const char* todozi_task_user_id(const Task* task) { return task ? task->user_id : NULL; }
const char* todozi_task_action(const Task* task) { return task ? task->action : NULL; }
const char* todozi_task_time(const Task* task) { return task ? task->time : NULL; }
Priority todozi_task_priority(const Task* task) { return task ? task->priority : PRIORITY_MEDIUM; }
const char* todozi_task_parent_project(const Task* task) { return task ? task->parent_project : NULL; }
Status todozi_task_status(const Task* task) { return task ? task->status : STATUS_TODO; }
AssigneeType todozi_task_assignee_type(const Task* task) { return task ? task->assignee_type : ASSIGNEE_HUMAN; }
const char* todozi_task_assignee_agent_name(const Task* task) { return task ? task->assignee_agent_name : NULL; }
const char* const* todozi_task_tags(const Task* task, size_t* count) {
    if (!task) {
        if (count) *count = 0;
        return NULL;
    }
    if (count) *count = task->tags_count;
    return (const char* const*)task->tags;
}
const char* const* todozi_task_dependencies(const Task* task, size_t* count) {
    if (!task) {
        if (count) *count = 0;
        return NULL;
    }
    if (count) *count = task->dependencies_count;
    return (const char* const*)task->dependencies;
}
const char* todozi_task_context_notes(const Task* task) { return task ? task->context_notes : NULL; }
const uint8_t* todozi_task_progress(const Task* task) { return task ? task->progress : NULL; }
time_t todozi_task_created_at(const Task* task) { return task ? task->created_at : 0; }
time_t todozi_task_updated_at(const Task* task) { return task ? task->updated_at : 0; }

// TaskUpdate implementation
TodoziResult todozi_task_update_new(TaskUpdate** out, TodoziError* err) {
    if (!out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    TaskUpdate* update = calloc(1, sizeof(TaskUpdate));
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate task update");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    *out = update;
    return TODOZI_OK;
}

void todozi_task_update_free(TaskUpdate* update) {
    if (!update) return;
    
    free(update->action);
    free(update->time);
    free(update->priority);
    free(update->parent_project);
    free(update->status);
    free(update->assignee_type);
    free(update->assignee_agent_name);
    todozi_free_string_array(update->tags, update->tags_count);
    todozi_free_string_array(update->dependencies, update->dependencies_count);
    free(update->context_notes);
    free(update->progress);
    free(update->embedding_vector);
    free(update);
}

TodoziResult todozi_task_update_with_action(TaskUpdate* update, const char* action, TodoziError* err) {
    if (!update || !action) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    char* temp = todozi_strdup_or_null(action);
    if (!temp) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to set action");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    free(update->action);
    update->action = temp;
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_time(TaskUpdate* update, const char* time, TodoziError* err) {
    if (!update || !time) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    char* temp = todozi_strdup_or_null(time);
    if (!temp) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to set time");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    free(update->time);
    update->time = temp;
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_priority(TaskUpdate* update, Priority priority, TodoziError* err) {
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (!update->priority) {
        update->priority = malloc(sizeof(Priority));
        if (!update->priority) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to allocate priority");
            }
            return TODOZI_ERR_ALLOC;
        }
    }
    
    *update->priority = priority;
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_parent_project(TaskUpdate* update, const char* parent_project, TodoziError* err) {
    if (!update || !parent_project) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    char* temp = todozi_strdup_or_null(parent_project);
    if (!temp) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to set parent project");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    free(update->parent_project);
    update->parent_project = temp;
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_status(TaskUpdate* update, Status status, TodoziError* err) {
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (!update->status) {
        update->status = malloc(sizeof(Status));
        if (!update->status) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to allocate status");
            }
            return TODOZI_ERR_ALLOC;
        }
    }
    
    *update->status = status;
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_assignee(TaskUpdate* update, AssigneeType assignee_type, const char* agent_name, TodoziError* err) {
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (!update->assignee_type) {
        update->assignee_type = malloc(sizeof(AssigneeType));
        if (!update->assignee_type) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to allocate assignee type");
            }
            return TODOZI_ERR_ALLOC;
        }
    }
    
    *update->assignee_type = assignee_type;
    
    if (agent_name) {
        char* temp = todozi_strdup_or_null(agent_name);
        if (!temp) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to set assignee agent name");
            }
            return TODOZI_ERR_ALLOC;
        }
        free(update->assignee_agent_name);
        update->assignee_agent_name = temp;
    }
    
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_tags(TaskUpdate* update, char** tags, size_t tags_count, TodoziError* err) {
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    todozi_free_string_array(update->tags, update->tags_count);
    
    if (copy_string_array((const char**)tags, tags_count, &update->tags, &update->tags_count, err) != TODOZI_OK) {
        return TODOZI_ERR_ALLOC;
    }
    
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_dependencies(TaskUpdate* update, char** dependencies, size_t dependencies_count, TodoziError* err) {
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    todozi_free_string_array(update->dependencies, update->dependencies_count);
    
    if (copy_string_array((const char**)dependencies, dependencies_count, &update->dependencies, &update->dependencies_count, err) != TODOZI_OK) {
        return TODOZI_ERR_ALLOC;
    }
    
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_context_notes(TaskUpdate* update, const char* context_notes, TodoziError* err) {
    if (!update || !context_notes) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    char* temp = todozi_strdup_or_null(context_notes);
    if (!temp) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to set context notes");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    free(update->context_notes);
    update->context_notes = temp;
    return TODOZI_OK;
}

TodoziResult todozi_task_update_with_progress(TaskUpdate* update, uint8_t progress, TodoziError* err) {
    if (!update) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (progress > 100) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Progress must be between 0 and 100");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (!update->progress) {
        update->progress = malloc(sizeof(uint8_t));
        if (!update->progress) {
            if (err) {
                err->code = TODOZI_ERR_ALLOC;
                err->msg = todozi_strdup_or_null("Failed to allocate progress");
            }
            return TODOZI_ERR_ALLOC;
        }
    }
    
    *update->progress = progress;
    return TODOZI_OK;
}

// Project implementation
struct Project {
    char* name;
    char* description;
    time_t created_at;
    time_t updated_at;
    ProjectStatus status;
    char** tasks;
    size_t tasks_count;
};

TodoziResult todozi_project_new(const char* name, const char* description, Project** out, TodoziError* err) {
    if (!name || !out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    Project* project = calloc(1, sizeof(Project));
    if (!project) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate project");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    if (alloc_and_copy_string(name, &project->name, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(description, &project->description, NULL) != TODOZI_OK) goto oom;
    
    project->created_at = time(NULL);
    project->updated_at = project->created_at;
    project->status = PROJECT_STATUS_ACTIVE;
    
    *out = project;
    return TODOZI_OK;
    
oom:
    todozi_project_free(project);
    if (err) {
        err->code = TODOZI_ERR_ALLOC;
        err->msg = todozi_strdup_or_null("Failed to allocate project fields");
    }
    return TODOZI_ERR_ALLOC;
}

void todozi_project_free(Project* project) {
    if (!project) return;
    
    free(project->name);
    free(project->description);
    todozi_free_string_array(project->tasks, project->tasks_count);
    free(project);
}

void todozi_project_add_task(Project* project, const char* task_id) {
    if (!project || !task_id) return;
    
    // Check if task already exists
    for (size_t i = 0; i < project->tasks_count; i++) {
        if (strcmp(project->tasks[i], task_id) == 0) {
            return; // Task already exists
        }
    }
    
    // Add new task
    char** new_tasks = realloc(project->tasks, sizeof(char*) * (project->tasks_count + 1));
    if (!new_tasks) return; // Allocation failed
    
    project->tasks = new_tasks;
    project->tasks[project->tasks_count] = todozi_strdup_or_null(task_id);
    if (project->tasks[project->tasks_count]) {
        project->tasks_count++;
        project->updated_at = time(NULL);
    }
}

void todozi_project_remove_task(Project* project, const char* task_id) {
    if (!project || !task_id) return;
    
    for (size_t i = 0; i < project->tasks_count; i++) {
        if (strcmp(project->tasks[i], task_id) == 0) {
            free(project->tasks[i]);
            // Shift remaining tasks
            for (size_t j = i; j < project->tasks_count - 1; j++) {
                project->tasks[j] = project->tasks[j + 1];
            }
            project->tasks_count--;
            
            // Reallocate array to free unused memory (optional optimization)
            if (project->tasks_count > 0) {
                char** new_tasks = realloc(project->tasks, sizeof(char*) * project->tasks_count);
                if (new_tasks) {
                    project->tasks = new_tasks;
                }
                // If realloc fails, we keep the old pointer - not critical
            } else {
                // No tasks left, free the array
                free(project->tasks);
                project->tasks = NULL;
            }
            
            project->updated_at = time(NULL);
            return;
        }
    }
}

void todozi_project_archive(Project* project) {
    if (project) {
        project->status = PROJECT_STATUS_ARCHIVED;
        project->updated_at = time(NULL);
    }
}

void todozi_project_complete(Project* project) {
    if (project) {
        project->status = PROJECT_STATUS_COMPLETED;
        project->updated_at = time(NULL);
    }
}

// Config implementation
struct Config {
    struct RegistrationInfo* registration;
    char* version;
    char* default_project;
    bool auto_backup;
    char* backup_interval;
    bool ai_enabled;
    AssigneeType* default_assignee_type;
    char* default_assignee_agent_name;
    char* date_format;
    char* timezone;
};

TodoziResult todozi_config_default(Config** out, TodoziError* err) {
    if (!out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    Config* config = calloc(1, sizeof(Config));
    if (!config) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate config");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    if (alloc_and_copy_string("1.2.0", &config->version, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string("general", &config->default_project, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string("daily", &config->backup_interval, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string("%Y-%m-%d %H:%M:%S", &config->date_format, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string("UTC", &config->timezone, NULL) != TODOZI_OK) goto oom;
    
    config->auto_backup = true;
    config->ai_enabled = true;
    
    config->default_assignee_type = malloc(sizeof(AssigneeType));
    if (!config->default_assignee_type) goto oom;
    *config->default_assignee_type = ASSIGNEE_COLLABORATIVE;
    
    *out = config;
    return TODOZI_OK;
    
oom:
    todozi_config_free(config);
    if (err) {
        err->code = TODOZI_ERR_ALLOC;
        err->msg = todozi_strdup_or_null("Failed to allocate config fields");
    }
    return TODOZI_ERR_ALLOC;
}

void todozi_config_free(Config* config) {
    if (!config) return;
    
    todozi_registration_info_free(config->registration);
    free(config->version);
    free(config->default_project);
    free(config->backup_interval);
    free(config->default_assignee_type);
    free(config->default_assignee_agent_name);
    free(config->date_format);
    free(config->timezone);
    free(config);
}

// RegistrationInfo implementation
struct RegistrationInfo {
    char* user_name;
    char* user_email;
    char* api_key;
    char* user_id;
    char* fingerprint;
    time_t registered_at;
    char* server_url;
};

TodoziResult todozi_registration_info_new(const char* user_name, const char* user_email,
                                         const char* api_key, const char* server_url,
                                         RegistrationInfo** out, TodoziError* err) {
    if (!user_name || !user_email || !api_key || !server_url || !out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    RegistrationInfo* reg = calloc(1, sizeof(RegistrationInfo));
    if (!reg) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate registration info");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    if (alloc_and_copy_string(user_name, &reg->user_name, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(user_email, &reg->user_email, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(api_key, &reg->api_key, NULL) != TODOZI_OK) goto oom;
    if (alloc_and_copy_string(server_url, &reg->server_url, NULL) != TODOZI_OK) goto oom;
    
    reg->registered_at = time(NULL);
    
    *out = reg;
    return TODOZI_OK;
    
oom:
    todozi_registration_info_free(reg);
    if (err) {
        err->code = TODOZI_ERR_ALLOC;
        err->msg = todozi_strdup_or_null("Failed to allocate registration info fields");
    }
    return TODOZI_ERR_ALLOC;
}

TodoziResult todozi_registration_info_new_with_hashes(const char* server_url,
                                                     RegistrationInfo** out, TodoziError* err) {
    if (!server_url || !out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    RegistrationInfo* reg = calloc(1, sizeof(RegistrationInfo));
    if (!reg) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate registration info");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    char* user_id = todozi_generate_short_uuid();
    if (!user_id) goto oom;
    
    char* email_hash = todozi_generate_short_uuid();
    if (!email_hash) {
        free(user_id);
        goto oom;
    }
    
    char* email_buffer = malloc(256);
    if (!email_buffer) {
        free(user_id);
        free(email_hash);
        goto oom;
    }
    
    snprintf(email_buffer, 256, "hash_%s@example.com", email_hash);
    free(email_hash);
    
    reg->user_name = user_id;
    reg->user_email = email_buffer;
    reg->api_key = todozi_strdup_or_null("");
    if (!reg->api_key) goto oom;
    
    if (alloc_and_copy_string(server_url, &reg->server_url, NULL) != TODOZI_OK) goto oom;
    
    reg->registered_at = time(NULL);
    
    *out = reg;
    return TODOZI_OK;
    
oom:
    todozi_registration_info_free(reg);
    if (err) {
        err->code = TODOZI_ERR_ALLOC;
        err->msg = todozi_strdup_or_null("Failed to allocate registration info fields");
    }
    return TODOZI_ERR_ALLOC;
}

void todozi_registration_info_free(RegistrationInfo* reg) {
    if (!reg) return;
    
    free(reg->user_name);
    free(reg->user_email);
    free(reg->api_key);
    free(reg->user_id);
    free(reg->fingerprint);
    free(reg->server_url);
    free(reg);
}

// Utility functions
char* todozi_strdup_or_null(const char* src) {
    if (!src) return NULL;
    char* dst = malloc(strlen(src) + 1);
    if (dst) strcpy(dst, src);
    return dst;
}

void todozi_free_string_array(char** arr, size_t n) {
    if (!arr) return;
    for (size_t i = 0; i < n; i++) {
        free(arr[i]);
    }
    free(arr);
}

TodoziResult todozi_alloc_array(size_t count, size_t elem_sz, void** out, TodoziError* err) {
    if (!out) {
        if (err) {
            err->code = TODOZI_ERR_INVALID;
            err->msg = todozi_strdup_or_null("Invalid parameters");
        }
        return TODOZI_ERR_INVALID;
    }
    
    if (count == 0) {
        *out = NULL;
        return TODOZI_OK;
    }
    
    *out = calloc(count, elem_sz);
    if (!*out) {
        if (err) {
            err->code = TODOZI_ERR_ALLOC;
            err->msg = todozi_strdup_or_null("Failed to allocate array");
        }
        return TODOZI_ERR_ALLOC;
    }
    
    return TODOZI_OK;
}

char* todozi_generate_short_uuid(void) {
    uuid_t uuid;
    char uuid_str[37];
    
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    // "task_" (5) + 8 chars from UUID + null terminator = 14 bytes
    char* short_uuid = malloc(14);
    if (!short_uuid) return NULL;
    
    snprintf(short_uuid, 14, "task_%.8s", uuid_str);
    return short_uuid;
}

char* todozi_generate_sha256(const char* input) {
    if (!input) return NULL;
    
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, input, strlen(input));
    SHA256_Final(hash, &sha256);
    
    char* output = malloc(SHA256_DIGEST_LENGTH * 2 + 1);
    if (!output) return NULL;
    
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        snprintf(output + (i * 2), 3, "%02x", hash[i]);
    }
    output[SHA256_DIGEST_LENGTH * 2] = '\0';
    
    return output;
}

char* todozi_generate_sha512(const char* input) {
    if (!input) return NULL;
    
    unsigned char hash[SHA512_DIGEST_LENGTH];
    SHA512_CTX sha512;
    SHA512_Init(&sha512);
    SHA512_Update(&sha512, input, strlen(input));
    SHA512_Final(hash, &sha512);
    
    char* output = malloc(SHA512_DIGEST_LENGTH * 2 + 1);
    if (!output) return NULL;
    
    for (int i = 0; i < SHA512_DIGEST_LENGTH; i++) {
        snprintf(output + (i * 2), 3, "%02x", hash[i]);
    }
    output[SHA512_DIGEST_LENGTH * 2] = '\0';
    
    return output;
}