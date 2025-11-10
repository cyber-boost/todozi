#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Forward declarations
typedef struct Task Task;
typedef struct TodoziApp TodoziApp;

// Enum definitions
typedef enum {
    PRIORITY_CRITICAL,
    PRIORITY_URGENT,
    PRIORITY_HIGH,
    PRIORITY_MEDIUM,
    PRIORITY_LOW
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
    ASSIGNEE_HUMAN,
    ASSIGNEE_AI,
    ASSIGNEE_COLLABORATIVE
} Assignee;

typedef enum {
    APP_TAB_PROJECTS,
    APP_TAB_TASKS,
    APP_TAB_DONE,
    APP_TAB_FIND,
    APP_TAB_MORE,
    APP_TAB_API,
    APP_TAB_FEED,
    APP_TAB_BYE
} AppTab;

typedef enum {
    SORT_ORDER_ASCENDING,
    SORT_ORDER_DESCENDING
} SortOrder;

typedef enum {
    TASK_SORT_BY_DATE_COMPLETED,
    TASK_SORT_BY_DATE_CREATED,
    TASK_SORT_BY_PRIORITY,
    TASK_SORT_BY_PROJECT,
    TASK_SORT_BY_ACTION,
    TASK_SORT_BY_TIME,
    TASK_SORT_BY_ASSIGNEE
} TaskSortBy;

typedef enum {
    EDITOR_FIELD_ACTION,
    EDITOR_FIELD_TIME,
    EDITOR_FIELD_PRIORITY,
    EDITOR_FIELD_STATUS,
    EDITOR_FIELD_PROJECT,
    EDITOR_FIELD_ASSIGNEE,
    EDITOR_FIELD_TAGS,
    EDITOR_FIELD_CONTEXT,
    EDITOR_FIELD_PROGRESS
} EditorField;

// Struct definitions
struct Task {
    char* id;
    char* user_id;
    char* action;
    char* time;
    Priority priority;
    Status status;
    Assignee* assignee;
    char* parent_project;
    char** tags;
    int tags_count;
    char** dependencies;
    int dependencies_count;
    char* context_notes;
    int* progress;
    double* embedding_vector;
    int embedding_vector_size;
    time_t created_at;
    time_t updated_at;
};

typedef struct {
    char* project_filter;
    Priority* priority_filter;
    Status* status_filter;
    Assignee* assignee_filter;
} TaskFilters;

typedef struct {
    Task task;
    char** similar_tasks;
    int similar_tasks_count;
    char** ai_suggestions;
    int ai_suggestions_count;
    char** semantic_tags;
    int semantic_tags_count;
    float confidence_score;
    char** related_content;
    int related_content_count;
} TaskDisplay;

typedef struct {
    Task* tasks;
    int tasks_count;
    int total_count;
    char* ai_summary;
    char*** semantic_clusters;
    int* semantic_clusters_sizes;
    int semantic_clusters_count;
} TaskListDisplay;

typedef struct {
    char* task_id;
    Task original_task;
    Task current_task;
    char** ai_suggestions;
    int ai_suggestions_count;
    char** validation_errors;
    int validation_errors_count;
    char** similarity_matches;
    int similarity_matches_count;
    time_t session_start;
} EditSession;

struct TodoziApp {
    AppTab current_tab;
    Task* tasks;
    int tasks_count;
    Task* filtered_tasks;
    int filtered_tasks_count;
    int selected_task_index;
    TaskFilters task_filters;
    char** projects;
    int projects_count;
    TaskSortBy done_sort_by;
    SortOrder done_sort_order;
    TaskFilters done_filters;
    int done_selected_task_index;
    int selected_project_index;
    char* search_query;
    Task* search_results;
    int search_results_count;
    EditSession* editor;
    EditorField editor_field;
    char* editor_input;
    int editor_selected_field;
    int* task_action_menu;
    int task_action_selected;
    Task* show_task_details;
    int should_quit;
    unsigned long* completion_data;
    int completion_data_size;
    unsigned long* priority_distribution;
    int priority_distribution_size;
    char* server_status;
    int server_running;
    // Extended data arrays would go here
    int ideas_count;
    int memories_count;
    int feelings_count;
    int errors_count;
    int training_data_count;
    int queue_items_count;
    int reminders_count;
    int more_tab_section;
    int more_tab_selected_index;
    int more_scroll_offset;
    int feed_scroll_offset;
    int api_keys_count;
    int api_selected_index;
    int api_endpoints_scroll;
    int api_keys_scroll;
    // Toast notifications would go here
    int toast_notifications_count;
};

// Function prototypes
TodoziApp* todozi_app_new();
void todozi_app_free(TodoziApp* app);
void todozi_app_load_tasks(TodoziApp* app);
void todozi_app_apply_filters(TodoziApp* app);
void todozi_app_render_done_tab(TodoziApp* app);
void todozi_app_render_done_controls(TodoziApp* app);
void todozi_app_render_done_tasks(TodoziApp* app);
void todozi_app_render_done_progress(TodoziApp* app);
Task** todozi_app_get_filtered_done_tasks(TodoziApp* app, int* count);
void todozi_app_sort_done_tasks(TodoziApp* app, Task** tasks, int count);
char* todozi_app_format_duration(time_t from, time_t to);
void todozi_app_handle_key_event(TodoziApp* app, int key_code);
void todozi_app_update_toasts(TodoziApp* app);
void todozi_app_save_current_field(TodoziApp* app);
void todozi_app_load_current_field(TodoziApp* app);
void todozi_app_update_editor_field(TodoziApp* app);
void todozi_app_handle_enter(TodoziApp* app);
void todozi_app_update_search_results(TodoziApp* app);
void todozi_app_draw(TodoziApp* app);
void todozi_app_draw_tabs(TodoziApp* app);
void todozi_app_draw_status_bar(TodoziApp* app);
void todozi_app_draw_tasks_tab(TodoziApp* app);
void todozi_app_draw_editor_tab(TodoziApp* app);
void todozi_app_draw_search_tab(TodoziApp* app);
void todozi_app_draw_api_tab(TodoziApp* app);
void todozi_app_draw_projects_tab(TodoziApp* app);
void todozi_app_draw_exit_tab(TodoziApp* app);
void todozi_app_next_tab(TodoziApp* app);
void todozi_app_previous_tab(TodoziApp* app);
void todozi_app_run(TodoziApp* app);

// Helper functions
char* string_clone(const char* str) {
    if (str == NULL) return NULL;
    char* clone = malloc(strlen(str) + 1);
    if (clone != NULL) {
        strcpy(clone, str);
    }
    return clone;
}

void string_array_free(char** array, int count) {
    if (array == NULL) return;
    for (int i = 0; i < count; i++) {
        free(array[i]);
    }
    free(array);
}

Task task_clone(Task* task) {
    if (task == NULL) {
        Task empty = {0};
        return empty;
    }
    
    Task clone = {0};
    clone.id = string_clone(task->id);
    clone.user_id = string_clone(task->user_id);
    clone.action = string_clone(task->action);
    clone.time = string_clone(task->time);
    clone.priority = task->priority;
    clone.status = task->status;
    clone.assignee = task->assignee ? malloc(sizeof(Assignee)) : NULL;
    if (clone.assignee) *clone.assignee = *task->assignee;
    clone.parent_project = string_clone(task->parent_project);
    
    clone.tags_count = task->tags_count;
    if (clone.tags_count > 0 && task->tags != NULL) {
        clone.tags = malloc(sizeof(char*) * clone.tags_count);
        if (clone.tags != NULL) {
            for (int i = 0; i < clone.tags_count; i++) {
                clone.tags[i] = string_clone(task->tags[i]);
            }
        } else {
            clone.tags_count = 0;
        }
    } else {
        clone.tags = NULL;
        clone.tags_count = 0;
    }
    
    clone.dependencies_count = task->dependencies_count;
    if (clone.dependencies_count > 0 && task->dependencies != NULL) {
        clone.dependencies = malloc(sizeof(char*) * clone.dependencies_count);
        if (clone.dependencies != NULL) {
            for (int i = 0; i < clone.dependencies_count; i++) {
                clone.dependencies[i] = string_clone(task->dependencies[i]);
            }
        } else {
            clone.dependencies_count = 0;
        }
    } else {
        clone.dependencies = NULL;
        clone.dependencies_count = 0;
    }
    
    clone.context_notes = string_clone(task->context_notes);
    clone.progress = task->progress ? malloc(sizeof(int)) : NULL;
    if (clone.progress) *clone.progress = *task->progress;
    
    clone.embedding_vector_size = task->embedding_vector_size;
    if (clone.embedding_vector_size > 0 && task->embedding_vector != NULL) {
        clone.embedding_vector = malloc(sizeof(double) * clone.embedding_vector_size);
        if (clone.embedding_vector != NULL) {
            for (int i = 0; i < clone.embedding_vector_size; i++) {
                clone.embedding_vector[i] = task->embedding_vector[i];
            }
        } else {
            clone.embedding_vector_size = 0;
        }
    } else {
        clone.embedding_vector = NULL;
        clone.embedding_vector_size = 0;
    }
    
    clone.created_at = task->created_at;
    clone.updated_at = task->updated_at;
    
    return clone;
}

void task_free(Task* task) {
    if (task == NULL) return;
    free(task->id);
    free(task->user_id);
    free(task->action);
    free(task->time);
    free(task->assignee);
    free(task->parent_project);
    string_array_free(task->tags, task->tags_count);
    string_array_free(task->dependencies, task->dependencies_count);
    free(task->context_notes);
    free(task->progress);
    free(task->embedding_vector);
}

// Main implementation
TodoziApp* todozi_app_new() {
    TodoziApp* app = calloc(1, sizeof(TodoziApp));
    if (app == NULL) return NULL;
    
    app->current_tab = APP_TAB_PROJECTS;
    app->done_sort_by = TASK_SORT_BY_DATE_COMPLETED;
    app->done_sort_order = SORT_ORDER_DESCENDING;
    app->search_query = string_clone("");
    app->editor_input = string_clone("");
    app->server_status = string_clone("Starting...");
    
    // Check for allocation failures
    if (app->search_query == NULL || app->editor_input == NULL || app->server_status == NULL) {
        free(app->search_query);
        free(app->editor_input);
        free(app->server_status);
        free(app);
        return NULL;
    }
    
    // Initialize arrays
    app->completion_data_size = 50;
    app->completion_data = calloc(app->completion_data_size, sizeof(unsigned long));
    if (app->completion_data == NULL) {
        todozi_app_free(app);
        return NULL;
    }
    
    app->priority_distribution_size = 10;
    app->priority_distribution = calloc(app->priority_distribution_size, sizeof(unsigned long));
    if (app->priority_distribution == NULL) {
        todozi_app_free(app);
        return NULL;
    }
    
    return app;
}

void todozi_app_free(TodoziApp* app) {
    if (app == NULL) return;
    
    // Free tasks
    if (app->tasks != NULL) {
        for (int i = 0; i < app->tasks_count; i++) {
            task_free(&app->tasks[i]);
        }
        free(app->tasks);
    }
    
    if (app->filtered_tasks != NULL) {
        for (int i = 0; i < app->filtered_tasks_count; i++) {
            task_free(&app->filtered_tasks[i]);
        }
        free(app->filtered_tasks);
    }
    
    if (app->search_results != NULL) {
        for (int i = 0; i < app->search_results_count; i++) {
            task_free(&app->search_results[i]);
        }
        free(app->search_results);
    }
    
    // Free projects
    if (app->projects != NULL) {
        string_array_free(app->projects, app->projects_count);
    }
    
    // Free strings
    free(app->search_query);
    free(app->editor_input);
    free(app->server_status);
    
    // Free arrays
    free(app->completion_data);
    free(app->priority_distribution);
    
    // Free task action menu
    free(app->task_action_menu);
    
    // Free editor if exists
    if (app->editor != NULL) {
        free(app->editor->task_id);
        task_free(&app->editor->original_task);
        task_free(&app->editor->current_task);
        string_array_free(app->editor->ai_suggestions, app->editor->ai_suggestions_count);
        string_array_free(app->editor->validation_errors, app->editor->validation_errors_count);
        string_array_free(app->editor->similarity_matches, app->editor->similarity_matches_count);
        free(app->editor);
    }
    
    // Free task details if exists
    if (app->show_task_details != NULL) {
        task_free(app->show_task_details);
        free(app->show_task_details);
    }
    
    // Free filter arrays
    free(app->task_filters.priority_filter);
    free(app->task_filters.status_filter);
    free(app->task_filters.assignee_filter);
    free(app->task_filters.project_filter);
    free(app->done_filters.priority_filter);
    free(app->done_filters.status_filter);
    free(app->done_filters.assignee_filter);
    free(app->done_filters.project_filter);
    
    free(app);
}

void todozi_app_load_tasks(TodoziApp* app) {
    if (app == NULL) return;
    
    // Placeholder implementation
    // In a real implementation, this would load from storage
    app->tasks_count = 0;
    app->tasks = NULL;
    todozi_app_apply_filters(app);
}

void todozi_app_apply_filters(TodoziApp* app) {
    if (app == NULL) return;
    
    // Free existing filtered tasks
    if (app->filtered_tasks != NULL) {
        for (int i = 0; i < app->filtered_tasks_count; i++) {
            task_free(&app->filtered_tasks[i]);
        }
        free(app->filtered_tasks);
    }
    app->filtered_tasks_count = 0;
    app->filtered_tasks = NULL;
    
    // Apply filters (simplified implementation)
    if (app->tasks == NULL || app->tasks_count == 0) {
        return;
    }
    
    app->filtered_tasks_count = app->tasks_count;
    app->filtered_tasks = malloc(sizeof(Task) * app->filtered_tasks_count);
    if (app->filtered_tasks == NULL) {
        app->filtered_tasks_count = 0;
        return;
    }
    
    for (int i = 0; i < app->filtered_tasks_count; i++) {
        app->filtered_tasks[i] = task_clone(&app->tasks[i]);
    }
}

void todozi_app_render_done_tab(TodoziApp* app) {
    if (app == NULL) return;
    printf("Rendering Done tab\n");
    todozi_app_render_done_controls(app);
    todozi_app_render_done_tasks(app);
    todozi_app_render_done_progress(app);
}

void todozi_app_render_done_controls(TodoziApp* app) {
    (void)app;  // Unused parameter
    printf("Rendering Done controls\n");
}

void todozi_app_render_done_tasks(TodoziApp* app) {
    (void)app;  // Unused parameter
    printf("Rendering Done tasks\n");
}

void todozi_app_render_done_progress(TodoziApp* app) {
    (void)app;  // Unused parameter
    printf("Rendering Done progress\n");
}

Task** todozi_app_get_filtered_done_tasks(TodoziApp* app, int* count) {
    if (app == NULL || count == NULL) {
        if (count != NULL) *count = 0;
        return NULL;
    }
    
    *count = 0;
    if (app->tasks == NULL || app->tasks_count == 0) {
        return NULL;
    }
    
    // First pass: count done tasks
    int done_count = 0;
    for (int i = 0; i < app->tasks_count; i++) {
        if (app->tasks[i].status == STATUS_DONE || app->tasks[i].status == STATUS_COMPLETED) {
            done_count++;
        }
    }
    
    if (done_count == 0) {
        return NULL;
    }
    
    // Second pass: allocate and populate
    Task** done_tasks = malloc(sizeof(Task*) * done_count);
    if (done_tasks == NULL) {
        *count = 0;
        return NULL;
    }
    
    for (int i = 0; i < app->tasks_count; i++) {
        if (app->tasks[i].status == STATUS_DONE || app->tasks[i].status == STATUS_COMPLETED) {
            done_tasks[*count] = &app->tasks[i];
            (*count)++;
        }
    }
    
    return done_tasks;
}

void todozi_app_sort_done_tasks(TodoziApp* app, Task** tasks, int count) {
    if (app == NULL || tasks == NULL || count <= 1) return;
    
    // Simplified bubble sort implementation
    for (int i = 0; i < count - 1; i++) {
        for (int j = 0; j < count - i - 1; j++) {
            if (tasks[j] == NULL || tasks[j + 1] == NULL) continue;
            
            int should_swap = 0;
            
            switch (app->done_sort_by) {
                case TASK_SORT_BY_DATE_COMPLETED:
                    if (app->done_sort_order == SORT_ORDER_ASCENDING) {
                        should_swap = tasks[j]->updated_at > tasks[j + 1]->updated_at;
                    } else {
                        should_swap = tasks[j]->updated_at < tasks[j + 1]->updated_at;
                    }
                    break;
                case TASK_SORT_BY_DATE_CREATED:
                    if (app->done_sort_order == SORT_ORDER_ASCENDING) {
                        should_swap = tasks[j]->created_at > tasks[j + 1]->created_at;
                    } else {
                        should_swap = tasks[j]->created_at < tasks[j + 1]->created_at;
                    }
                    break;
                case TASK_SORT_BY_PRIORITY:
                    if (app->done_sort_order == SORT_ORDER_ASCENDING) {
                        should_swap = tasks[j]->priority > tasks[j + 1]->priority;
                    } else {
                        should_swap = tasks[j]->priority < tasks[j + 1]->priority;
                    }
                    break;
                default:
                    break;
            }
            
            if (should_swap) {
                Task* temp = tasks[j];
                tasks[j] = tasks[j + 1];
                tasks[j + 1] = temp;
            }
        }
    }
}

char* todozi_app_format_duration(time_t from, time_t to) {
    double diff = difftime(to, from);
    char* result = malloc(64);  // Increased buffer size for safety
    if (result == NULL) {
        return NULL;
    }
    
    if (diff < 60) {
        snprintf(result, 64, "%.0fs ago", diff > 1 ? diff : 1);
    } else if (diff < 3600) {
        snprintf(result, 64, "%.0fm ago", diff / 60);
    } else if (diff < 86400) {
        snprintf(result, 64, "%.0fh ago", diff / 3600);
    } else if (diff < 604800) {
        snprintf(result, 64, "%.0fd ago", diff / 86400);
    } else {
        snprintf(result, 64, "%.0fw ago", diff / 604800);
    }
    
    return result;
}

void todozi_app_handle_key_event(TodoziApp* app, int key_code) {
    if (app == NULL) return;
    
    switch (key_code) {
        case 'q':
            app->should_quit = 1;
            break;
        case '\t': // Tab
            app->current_tab = (app->current_tab + 1) % 8;
            break;
        case 353: // BackTab (Shift+Tab)
            app->current_tab = (app->current_tab + 7) % 8;
            break;
        default:
            break;
    }
}

void todozi_app_update_toasts(TodoziApp* app) {
    (void)app;  // Unused parameter
    // Placeholder implementation
}

void todozi_app_save_current_field(TodoziApp* app) {
    if (app->editor == NULL) return;
    
    switch (app->editor_field) {
        case EDITOR_FIELD_ACTION:
            free(app->editor->current_task.action);
            app->editor->current_task.action = string_clone(app->editor_input);
            break;
        case EDITOR_FIELD_TIME:
            free(app->editor->current_task.time);
            app->editor->current_task.time = string_clone(app->editor_input);
            break;
        // Add other fields as needed
        default:
            break;
    }
}

void todozi_app_load_current_field(TodoziApp* app) {
    if (app->editor == NULL) return;
    
    free(app->editor_input);
    app->editor_input = string_clone("");
    
    switch (app->editor_field) {
        case EDITOR_FIELD_ACTION:
            app->editor_input = string_clone(app->editor->current_task.action);
            break;
        case EDITOR_FIELD_TIME:
            app->editor_input = string_clone(app->editor->current_task.time);
            break;
        // Add other fields as needed
        default:
            break;
    }
}

void todozi_app_update_editor_field(TodoziApp* app) {
    switch (app->editor_selected_field) {
        case 0: app->editor_field = EDITOR_FIELD_ACTION; break;
        case 1: app->editor_field = EDITOR_FIELD_TIME; break;
        case 2: app->editor_field = EDITOR_FIELD_PRIORITY; break;
        case 3: app->editor_field = EDITOR_FIELD_STATUS; break;
        case 4: app->editor_field = EDITOR_FIELD_PROJECT; break;
        case 5: app->editor_field = EDITOR_FIELD_ASSIGNEE; break;
        case 6: app->editor_field = EDITOR_FIELD_TAGS; break;
        case 7: app->editor_field = EDITOR_FIELD_CONTEXT; break;
        case 8: app->editor_field = EDITOR_FIELD_PROGRESS; break;
        default: app->editor_field = EDITOR_FIELD_ACTION; break;
    }
}

void todozi_app_handle_enter(TodoziApp* app) {
    if (app == NULL) return;
    
    switch (app->current_tab) {
        case APP_TAB_BYE:
            app->should_quit = 1;
            break;
        default:
            break;
    }
}

void todozi_app_update_search_results(TodoziApp* app) {
    if (app == NULL) return;
    
    // Free existing search results
    if (app->search_results != NULL) {
        for (int i = 0; i < app->search_results_count; i++) {
            task_free(&app->search_results[i]);
        }
        free(app->search_results);
    }
    app->search_results_count = 0;
    app->search_results = NULL;
    
    if (app->search_query == NULL || strlen(app->search_query) == 0) {
        return;
    }
    
    if (app->tasks == NULL || app->tasks_count == 0) {
        return;
    }
    
    // First pass: count matching tasks
    int match_count = 0;
    for (int i = 0; i < app->tasks_count; i++) {
        if (app->tasks[i].action != NULL && 
            strstr(app->tasks[i].action, app->search_query) != NULL) {
            match_count++;
        }
    }
    
    if (match_count == 0) {
        return;
    }
    
    // Second pass: allocate and populate
    app->search_results = malloc(sizeof(Task) * match_count);
    if (app->search_results == NULL) {
        app->search_results_count = 0;
        return;
    }
    
    app->search_results_count = 0;
    for (int i = 0; i < app->tasks_count; i++) {
        if (app->tasks[i].action != NULL && 
            strstr(app->tasks[i].action, app->search_query) != NULL) {
            app->search_results[app->search_results_count] = task_clone(&app->tasks[i]);
            app->search_results_count++;
        }
    }
}

void todozi_app_draw(TodoziApp* app) {
    if (app == NULL) return;
    
    printf("Drawing Todozi App\n");
    todozi_app_draw_tabs(app);
    
    switch (app->current_tab) {
        case APP_TAB_PROJECTS:
            todozi_app_draw_projects_tab(app);
            break;
        case APP_TAB_TASKS:
            todozi_app_draw_tasks_tab(app);
            break;
        case APP_TAB_DONE:
            todozi_app_render_done_tab(app);
            break;
        case APP_TAB_FIND:
            todozi_app_draw_search_tab(app);
            break;
        case APP_TAB_API:
            todozi_app_draw_api_tab(app);
            break;
        case APP_TAB_FEED:
            // Feed tab drawing
            break;
        case APP_TAB_BYE:
            todozi_app_draw_exit_tab(app);
            break;
        default:
            break;
    }
    
    todozi_app_draw_status_bar(app);
}

void todozi_app_draw_tabs(TodoziApp* app) {
    if (app == NULL) return;
    
    const char* tab_titles[] = {
        "📁 Projects", "📋 Tasks", "✅ Done", "🔍 Find",
        "🔮 More", "🔑 API", "📰 Feed", "👋 Bye"
    };
    
    printf("Todozi [✓] | ");
    for (int i = 0; i < 8; i++) {
        if (i == app->current_tab) {
            printf("[%s] ", tab_titles[i]);
        } else {
            printf("%s ", tab_titles[i]);
        }
    }
    printf("\n");
}

void todozi_app_draw_status_bar(TodoziApp* app) {
    if (app == NULL) return;
    
    switch (app->current_tab) {
        case APP_TAB_PROJECTS:
            printf("Projects: %d | ↑↓ Navigate | Enter Select | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit\n", 
                   app->projects_count);
            break;
        case APP_TAB_TASKS:
            printf("Tasks: %d | ↑↓ Navigate | Enter Edit | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit\n",
                   app->filtered_tasks_count);
            break;
        case APP_TAB_DONE:
            printf("Done: View statistics | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit\n");
            break;
        case APP_TAB_FIND: {
            const char* query = app->search_query ? app->search_query : "";
            printf("Find: '%s' | %d results | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit\n",
                   query, app->search_results_count);
            break;
        }
        case APP_TAB_BYE:
            printf("Bye: Press Enter to quit | 1-8: Direct tab | Tab Switch | Ctrl+Q Quit\n");
            break;
        default:
            printf("1-8: Direct tab | Tab Switch | Ctrl+Q Quit\n");
            break;
    }
}

void todozi_app_draw_tasks_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    printf("🔍 Filters\n");
    printf("📋 Tasks (%d)\n", app->filtered_tasks_count);
    
    // Show "Add New Task" option
    if (app->selected_task_index == 0) {
        printf("➤ ➕ Add New Task\n");
    } else {
        printf("  ➕ Add New Task\n");
    }
    
    // Show tasks
    if (app->filtered_tasks != NULL) {
        for (int i = 0; i < app->filtered_tasks_count; i++) {
            const char* status_icon;
            switch (app->filtered_tasks[i].status) {
                case STATUS_TODO: case STATUS_PENDING: status_icon = "📝"; break;
                case STATUS_IN_PROGRESS: status_icon = "🔄"; break;
                case STATUS_BLOCKED: status_icon = "🚫"; break;
                case STATUS_REVIEW: status_icon = "👀"; break;
                case STATUS_DONE: case STATUS_COMPLETED: status_icon = "✅"; break;
                case STATUS_CANCELLED: status_icon = "❌"; break;
                case STATUS_DEFERRED: status_icon = "⏸️"; break;
                default: status_icon = "📝"; break;
            }
            
            const char* action = app->filtered_tasks[i].action ? app->filtered_tasks[i].action : "(no action)";
            const char* project = app->filtered_tasks[i].parent_project ? app->filtered_tasks[i].parent_project : "(no project)";
            
            if (i + 1 == app->selected_task_index) {
                printf("➤ %s %s [%s]\n", status_icon, action, project);
            } else {
                printf("  %s %s [%s]\n", status_icon, action, project);
            }
        }
    }
}

void todozi_app_draw_editor_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    if (app->editor == NULL) {
        printf("No task selected for editing.\n");
        printf("Go to Tasks tab and press Enter on a task to edit it.\n");
        return;
    }
    
    const char* task_id = app->editor->task_id ? app->editor->task_id : "(no id)";
    printf("Editing: %s | ESC: Save & Close | ↑↓: Navigate Fields\n", task_id);
    printf("Task Fields\n");
    
    const char* field_names[] = {
        "Action", "Time", "Priority", "Status", "Project",
        "Assignee", "Tags", "Context/Notes", "Progress"
    };
    
    const char* action = app->editor->current_task.action ? app->editor->current_task.action : "";
    const char* time = app->editor->current_task.time ? app->editor->current_task.time : "";
    
    for (int i = 0; i < 9; i++) {
        const char* value = "";
        if (i == 0) value = action;
        else if (i == 1) value = time;
        
        if (i == app->editor_selected_field) {
            printf("▶ %d.%s: %s\n", i + 1, field_names[i], value);
        } else {
            printf("  %d.%s: %s\n", i + 1, field_names[i], value);
        }
    }
    
    printf("Type to edit current field | Enter: Next field | Backspace: Delete | ESC: Save & Close\n");
}

void todozi_app_draw_search_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    const char* query = app->search_query ? app->search_query : "";
    printf("🔍 Find: %s\n", query);
    printf("Results (%d)\n", app->search_results_count);
    
    if (app->search_results != NULL) {
        for (int i = 0; i < app->search_results_count; i++) {
            const char* action = app->search_results[i].action ? app->search_results[i].action : "(no action)";
            if (i == app->selected_task_index) {
                printf("➤ %s\n", action);
            } else {
                printf("  %s\n", action);
            }
        }
    }
}

void todozi_app_draw_api_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    printf("🖥️ Server Control\n");
    printf("Status: %s\n", app->server_running ? "🟢 Running" : "🔴 Stopped");
    printf("Controls: [S]tart [X]top [R]estart [C]lear Cache\n");
    
    printf("\n📋 API Endpoints\n");
    printf("GET /api/tasks - List all tasks\n");
    printf("POST /api/tasks - Create new task\n");
    printf("PUT /api/tasks/:id - Update existing task\n");
    printf("DELETE /api/tasks/:id - Delete task\n");
    
    printf("\n🔑 API Keys\n");
    printf("No API keys configured\n");
}

void todozi_app_draw_projects_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    printf("Actions: ↑↓ Navigate | Enter Select | + Add New | - Delete | F5 Refresh\n");
    printf("📁 Active (%d)\n", app->projects_count);
    
    // Show "Add New Project" option
    if (app->selected_project_index == 0) {
        printf("➤ ➕ Add New Project\n");
    } else {
        printf("  ➕ Add New Project\n");
    }
    
    // Show projects
    if (app->projects != NULL) {
        for (int i = 0; i < app->projects_count; i++) {
            int task_count = 0;
            // Count tasks for this project
            if (app->tasks != NULL && app->projects[i] != NULL) {
                for (int j = 0; j < app->tasks_count; j++) {
                    if (app->tasks[j].parent_project != NULL && 
                        strcmp(app->tasks[j].parent_project, app->projects[i]) == 0) {
                        task_count++;
                    }
                }
            }
            
            const char* project_name = app->projects[i] ? app->projects[i] : "(unnamed)";
            if (i + 1 == app->selected_project_index) {
                printf("➤ 📁 %s (%d tasks)\n", project_name, task_count);
            } else {
                printf("  📁 %s (%d tasks)\n", project_name, task_count);
            }
        }
    }
}

void todozi_app_draw_exit_tab(TodoziApp* app) {
    (void)app;  // Unused parameter
    printf(" _______        _            \n");
    printf("|__   __|      | |        (✓)\n");
    printf("   | | ___   __| | ___ _____\n");
    printf("   | |/ _ \\ / _` |/ _ \\_  / |\n");
    printf("   | | (_) | (_| | (_) / /| |\n");
    printf("   |_|\\___/ \\__,_|\\___/___|_|\n");
    printf("\n");
    printf("Are you sure you want to leave Todozi?\n");
    printf("Press Enter to confirm exit\n");
    printf("Thank you for using Todozi! 🎉\n");
}

void todozi_app_next_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    app->current_tab = (app->current_tab + 1) % 8;
    app->selected_task_index = 0;
    app->selected_project_index = 0;
}

void todozi_app_previous_tab(TodoziApp* app) {
    if (app == NULL) return;
    
    app->current_tab = (app->current_tab + 7) % 8;
    app->selected_task_index = 0;
    app->selected_project_index = 0;
}

void todozi_app_run(TodoziApp* app) {
    if (app == NULL) return;
    
    todozi_app_load_tasks(app);
    
    while (!app->should_quit) {
        todozi_app_draw(app);
        
        // In a real implementation, you would read input here
        // For this example, we'll just quit after one draw
        app->should_quit = 1;
    }
}

// Main function for testing
int main() {
    TodoziApp* app = todozi_app_new();
    if (app == NULL) {
        fprintf(stderr, "Failed to create TodoziApp\n");
        return 1;
    }
    
    todozi_app_run(app);
    todozi_app_free(app);
    
    return 0;
}