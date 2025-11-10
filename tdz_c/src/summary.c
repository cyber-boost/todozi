#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Forward declarations
typedef struct Summary Summary;
typedef struct SummaryManager SummaryManager;
typedef struct SummaryUpdate SummaryUpdate;
typedef struct SummaryStatistics SummaryStatistics;

// Enums
typedef enum {
    SUMMARY_PRIORITY_LOW,
    SUMMARY_PRIORITY_MEDIUM,
    SUMMARY_PRIORITY_HIGH,
    SUMMARY_PRIORITY_CRITICAL
} SummaryPriority;

typedef enum {
    TODOZI_ERROR_VALIDATION
} TodoziErrorType;

// Error structure
typedef struct {
    TodoziErrorType type;
    char* message;
} TodoziError;

// String vector
typedef struct {
    char** data;
    size_t size;
    size_t capacity;
} StringVec;

// String-String map (simplified)
typedef struct {
    char** keys;
    char** values;
    size_t size;
    size_t capacity;
} StringStringMap;

// String-Pointer map for storing Summary* pointers
typedef struct {
    char** keys;
    void** values;  // Store pointers as void*
    size_t size;
    size_t capacity;
} StringPtrMap;

// String-StringVec map (simplified)
typedef struct {
    char** keys;
    StringVec* values;
    size_t size;
    size_t capacity;
} StringStringVecMap;

// Time structure
typedef struct {
    time_t seconds;
} DateTime;

// Summary structure
struct Summary {
    char* id;
    char* content;
    char* context;
    SummaryPriority priority;
    StringVec tags;
    DateTime created_at;
    DateTime updated_at;
};

// SummaryUpdate structure
struct SummaryUpdate {
    char* content;
    char* context;
    SummaryPriority* priority;
    StringVec* tags;
    int has_content;
    int has_context;
    int has_priority;
    int has_tags;
};

// SummaryStatistics structure
struct SummaryStatistics {
    size_t total_summaries;
    size_t high_priority_summaries;
    size_t unique_tags;
};

// SummaryManager structure
struct SummaryManager {
    StringPtrMap summaries;  // id -> Summary*
    StringStringVecMap summary_tags;  // id -> tags
};

// Function prototypes
SummaryManager* summary_manager_new();
void summary_manager_free(SummaryManager* manager);
char* summary_manager_create_summary(SummaryManager* manager, Summary* summary);
Summary* summary_manager_get_summary(SummaryManager* manager, const char* summary_id);
Summary** summary_manager_get_all_summaries(SummaryManager* manager, size_t* count);
int summary_manager_update_summary(SummaryManager* manager, const char* summary_id, SummaryUpdate* updates);
int summary_manager_delete_summary(SummaryManager* manager, const char* summary_id);
Summary** summary_manager_search_summaries(SummaryManager* manager, const char* query, size_t* count);
Summary** summary_manager_get_summaries_by_priority(SummaryManager* manager, SummaryPriority priority, size_t* count);
Summary** summary_manager_get_summaries_by_tag(SummaryManager* manager, const char* tag, size_t* count);
Summary** summary_manager_get_recent_summaries(SummaryManager* manager, size_t limit, size_t* count);
Summary** summary_manager_get_high_priority_summaries(SummaryManager* manager, size_t* count);
char** summary_manager_get_all_tags(SummaryManager* manager, size_t* count);
StringStringMap* summary_manager_get_tag_statistics(SummaryManager* manager);
SummaryStatistics* summary_manager_get_summary_statistics(SummaryManager* manager);

SummaryUpdate* summary_update_new();
void summary_update_free(SummaryUpdate* update);
SummaryUpdate* summary_update_content(SummaryUpdate* update, const char* content);
SummaryUpdate* summary_update_context(SummaryUpdate* update, const char* context);
SummaryUpdate* summary_update_priority(SummaryUpdate* update, SummaryPriority priority);
SummaryUpdate* summary_update_tags(SummaryUpdate* update, StringVec* tags);

SummaryStatistics* summary_statistics_new(size_t total, size_t high_priority, size_t unique_tags);
void summary_statistics_free(SummaryStatistics* stats);
double summary_statistics_high_priority_percentage(SummaryStatistics* stats);

Summary* summary_new();
void summary_free(Summary* summary);

StringVec* string_vec_new();
void string_vec_free(StringVec* vec);
void string_vec_push(StringVec* vec, const char* str);
StringVec* string_vec_clone(StringVec* vec);

StringStringMap* string_string_map_new();
void string_string_map_free(StringStringMap* map);
void string_string_map_insert(StringStringMap* map, const char* key, const char* value);
char* string_string_map_get(StringStringMap* map, const char* key);
int string_string_map_remove(StringStringMap* map, const char* key);

StringStringVecMap* string_string_vec_map_new();
void string_string_vec_map_free(StringStringVecMap* map);
void string_string_vec_map_insert(StringStringVecMap* map, const char* key, StringVec* value);
StringVec* string_string_vec_map_get(StringStringVecMap* map, const char* key);
int string_string_vec_map_remove(StringStringVecMap* map, const char* key);

StringPtrMap* string_ptr_map_new();
void string_ptr_map_free(StringPtrMap* map);
void string_ptr_map_insert(StringPtrMap* map, const char* key, void* value);
void* string_ptr_map_get(StringPtrMap* map, const char* key);
int string_ptr_map_remove(StringPtrMap* map, const char* key);

DateTime datetime_now();
int datetime_compare(DateTime* a, DateTime* b);

TodoziError* todozi_error_new(TodoziErrorType type, const char* message);
void todozi_error_free(TodoziError* error);

int parse_summary_format(const char* summary_text, Summary** result, TodoziError** error);

// Utility functions
char* string_clone(const char* str);
char* string_to_lower(const char* str);
static int string_contains(const char* haystack, const char* needle);
int string_equals_ignore_case(const char* a, const char* b);

// Implementation

SummaryManager* summary_manager_new() {
    SummaryManager* manager = malloc(sizeof(SummaryManager));
    if (!manager) return NULL;
    
    manager->summaries.keys = NULL;
    manager->summaries.values = NULL;
    manager->summaries.size = 0;
    manager->summaries.capacity = 0;
    
    manager->summary_tags.keys = NULL;
    manager->summary_tags.values = NULL;
    manager->summary_tags.size = 0;
    manager->summary_tags.capacity = 0;
    
    return manager;
}

void summary_manager_free(SummaryManager* manager) {
    if (!manager) return;
    
    // Free all Summary objects first
    for (size_t i = 0; i < manager->summaries.size; i++) {
        free(manager->summaries.keys[i]);
        Summary* summary = (Summary*)manager->summaries.values[i];
        if (summary) {
            summary_free(summary);
        }
    }
    free(manager->summaries.keys);
    free(manager->summaries.values);
    
    // Free summary_tags map
    for (size_t i = 0; i < manager->summary_tags.size; i++) {
        free(manager->summary_tags.keys[i]);
        string_vec_free(&manager->summary_tags.values[i]);
    }
    free(manager->summary_tags.keys);
    free(manager->summary_tags.values);
    
    free(manager);
}

char* summary_manager_create_summary(SummaryManager* manager, Summary* summary) {
    if (!manager || !summary) return NULL;
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    char* uuid_str = malloc(37); // UUID string length + null terminator
    if (!uuid_str) return NULL;
    uuid_unparse(uuid, uuid_str);
    
    // Store in maps - need to clone the summary to own it
    Summary* summary_copy = malloc(sizeof(Summary));
    if (!summary_copy) {
        free(uuid_str);
        return NULL;
    }
    
    // Initialize the copy
    summary_copy->id = uuid_str;
    summary_copy->content = summary->content ? string_clone(summary->content) : NULL;
    summary_copy->context = summary->context ? string_clone(summary->context) : NULL;
    summary_copy->priority = summary->priority;
    summary_copy->created_at = datetime_now();
    summary_copy->updated_at = datetime_now();
    
    // Clone tags
    StringVec* cloned_tags = string_vec_clone(&summary->tags);
    if (!cloned_tags) {
        free(summary_copy->id);
        if (summary_copy->content) free(summary_copy->content);
        if (summary_copy->context) free(summary_copy->context);
        free(summary_copy);
        return NULL;
    }
    summary_copy->tags = *cloned_tags;
    free(cloned_tags);
    
    string_ptr_map_insert(&manager->summaries, summary_copy->id, summary_copy);
    string_string_vec_map_insert(&manager->summary_tags, summary_copy->id, &summary_copy->tags);
    
    return string_clone(summary_copy->id);
}

Summary* summary_manager_get_summary(SummaryManager* manager, const char* summary_id) {
    if (!manager || !summary_id) return NULL;
    return (Summary*)string_ptr_map_get(&manager->summaries, summary_id);
}

Summary** summary_manager_get_all_summaries(SummaryManager* manager, size_t* count) {
    if (!manager || !count) return NULL;
    
    *count = manager->summaries.size;
    if (*count == 0) return NULL;
    
    Summary** result = malloc(*count * sizeof(Summary*));
    if (!result) return NULL;
    
    for (size_t i = 0; i < *count; i++) {
        result[i] = (Summary*)manager->summaries.values[i];
    }
    
    return result;
}

int summary_manager_update_summary(SummaryManager* manager, const char* summary_id, SummaryUpdate* updates) {
    if (!manager || !summary_id || !updates) return 0;
    
    Summary* summary = summary_manager_get_summary(manager, summary_id);
    if (!summary) {
        return 0; // Error case
    }
    
    if (updates->has_content) {
        free(summary->content);
        summary->content = string_clone(updates->content);
    }
    
    if (updates->has_context) {
        free(summary->context);
        summary->context = string_clone(updates->context);
    }
    
    if (updates->has_priority) {
        summary->priority = *updates->priority;
    }
    
    if (updates->has_tags) {
        string_vec_free(&summary->tags);
        summary->tags = *string_vec_clone(updates->tags);
        string_string_vec_map_insert(&manager->summary_tags, summary_id, &summary->tags);
    }
    
    summary->updated_at = datetime_now();
    return 1; // Success
}

int summary_manager_delete_summary(SummaryManager* manager, const char* summary_id) {
    if (!manager || !summary_id) return 0;
    
    // Get the summary first to free it
    Summary* summary = summary_manager_get_summary(manager, summary_id);
    if (!summary) return 0;
    
    // Remove from maps
    int removed = string_ptr_map_remove(&manager->summaries, summary_id);
    if (removed) {
        string_string_vec_map_remove(&manager->summary_tags, summary_id);
        // Free the summary object
        summary_free(summary);
        return 1; // Success
    }
    return 0; // Not found
}

Summary** summary_manager_search_summaries(SummaryManager* manager, const char* query, size_t* count) {
    if (!manager || !query || !count) return NULL;
    
    *count = 0;
    size_t matches_capacity = 10;
    Summary** matches = malloc(matches_capacity * sizeof(Summary*));
    if (!matches) return NULL;
    
    char* query_lower = string_to_lower(query);
    
    for (size_t i = 0; i < manager->summaries.size; i++) {
        Summary* summary = (Summary*)manager->summaries.values[i];
        if (!summary) continue;
        
        int match = 0;
        if (summary->content) {
            char* content_lower = string_to_lower(summary->content);
            match = string_contains(content_lower, query_lower);
            free(content_lower);
        }
        
        if (!match) {
            for (size_t j = 0; j < summary->tags.size; j++) {
                char* tag_lower = string_to_lower(summary->tags.data[j]);
                if (string_contains(tag_lower, query_lower)) {
                    match = 1;
                    free(tag_lower);
                    break;
                }
                free(tag_lower);
            }
        }
        
        if (!match && summary->context) {
            char* context_lower = string_to_lower(summary->context);
            match = string_contains(context_lower, query_lower);
            free(context_lower);
        }
        
        if (match) {
            if (*count >= matches_capacity) {
                matches_capacity *= 2;
                Summary** temp = realloc(matches, matches_capacity * sizeof(Summary*));
                if (!temp) {
                    free(matches);
                    free(query_lower);
                    *count = 0;
                    return NULL;
                }
                matches = temp;
            }
            matches[*count] = summary;
            (*count)++;
        }
    }
    
    free(query_lower);
    
    if (*count == 0) {
        free(matches);
        return NULL;
    }
    
    return matches;
}

Summary** summary_manager_get_summaries_by_priority(SummaryManager* manager, SummaryPriority priority, size_t* count) {
    if (!manager || !count) return NULL;
    
    *count = 0;
    size_t matches_capacity = 10;
    Summary** matches = malloc(matches_capacity * sizeof(Summary*));
    if (!matches) return NULL;
    
    for (size_t i = 0; i < manager->summaries.size; i++) {
        Summary* summary = (Summary*)manager->summaries.values[i];
        if (!summary) continue;
        
        if (summary->priority == priority) {
            if (*count >= matches_capacity) {
                matches_capacity *= 2;
                Summary** temp = realloc(matches, matches_capacity * sizeof(Summary*));
                if (!temp) {
                    free(matches);
                    *count = 0;
                    return NULL;
                }
                matches = temp;
            }
            matches[*count] = summary;
            (*count)++;
        }
    }
    
    if (*count == 0) {
        free(matches);
        return NULL;
    }
    
    return matches;
}

Summary** summary_manager_get_summaries_by_tag(SummaryManager* manager, const char* tag, size_t* count) {
    if (!manager || !tag || !count) return NULL;
    
    *count = 0;
    size_t matches_capacity = 10;
    Summary** matches = malloc(matches_capacity * sizeof(Summary*));
    if (!matches) return NULL;
    
    char* tag_lower = string_to_lower(tag);
    
    for (size_t i = 0; i < manager->summaries.size; i++) {
        Summary* summary = (Summary*)manager->summaries.values[i];
        if (!summary) continue;
        
        int found = 0;
        for (size_t j = 0; j < summary->tags.size; j++) {
            char* summary_tag_lower = string_to_lower(summary->tags.data[j]);
            if (string_equals_ignore_case(summary_tag_lower, tag_lower)) {
                found = 1;
                free(summary_tag_lower);
                break;
            }
            free(summary_tag_lower);
        }
        
        if (found) {
            if (*count >= matches_capacity) {
                matches_capacity *= 2;
                Summary** temp = realloc(matches, matches_capacity * sizeof(Summary*));
                if (!temp) {
                    free(matches);
                    free(tag_lower);
                    *count = 0;
                    return NULL;
                }
                matches = temp;
            }
            matches[*count] = summary;
            (*count)++;
        }
    }
    
    free(tag_lower);
    
    if (*count == 0) {
        free(matches);
        return NULL;
    }
    
    return matches;
}

Summary** summary_manager_get_recent_summaries(SummaryManager* manager, size_t limit, size_t* count) {
    if (!manager || !count) return NULL;
    
    Summary** all_summaries = summary_manager_get_all_summaries(manager, count);
    if (!all_summaries) return NULL;
    
    // Simple bubble sort by created_at (descending)
    for (size_t i = 0; i < *count - 1; i++) {
        for (size_t j = 0; j < *count - i - 1; j++) {
            if (datetime_compare(&((Summary*)all_summaries[j])->created_at, 
                                &((Summary*)all_summaries[j + 1])->created_at) < 0) {
                Summary* temp = all_summaries[j];
                all_summaries[j] = all_summaries[j + 1];
                all_summaries[j + 1] = temp;
            }
        }
    }
    
    size_t result_count = (*count < limit) ? *count : limit;
    Summary** result = malloc(result_count * sizeof(Summary*));
    if (!result) {
        free(all_summaries);
        *count = 0;
        return NULL;
    }
    
    for (size_t i = 0; i < result_count; i++) {
        result[i] = all_summaries[i];
    }
    
    free(all_summaries);
    *count = result_count;
    return result;
}

Summary** summary_manager_get_high_priority_summaries(SummaryManager* manager, size_t* count) {
    if (!manager || !count) return NULL;
    
    *count = 0;
    size_t matches_capacity = 10;
    Summary** matches = malloc(matches_capacity * sizeof(Summary*));
    if (!matches) return NULL;
    
    for (size_t i = 0; i < manager->summaries.size; i++) {
        Summary* summary = (Summary*)manager->summaries.values[i];
        if (!summary) continue;
        
        if (summary->priority == SUMMARY_PRIORITY_HIGH || summary->priority == SUMMARY_PRIORITY_CRITICAL) {
            if (*count >= matches_capacity) {
                matches_capacity *= 2;
                Summary** temp = realloc(matches, matches_capacity * sizeof(Summary*));
                if (!temp) {
                    free(matches);
                    *count = 0;
                    return NULL;
                }
                matches = temp;
            }
            matches[*count] = summary;
            (*count)++;
        }
    }
    
    if (*count == 0) {
        free(matches);
        return NULL;
    }
    
    return matches;
}

char** summary_manager_get_all_tags(SummaryManager* manager, size_t* count) {
    if (!manager || !count) return NULL;
    
    // Use a simple approach to collect unique tags
    size_t tags_capacity = 50;
    char** unique_tags = malloc(tags_capacity * sizeof(char*));
    if (!unique_tags) return NULL;
    
    *count = 0;
    
    for (size_t i = 0; i < manager->summary_tags.size; i++) {
        StringVec* tags = &manager->summary_tags.values[i];
        
        for (size_t j = 0; j < tags->size; j++) {
            // Check if tag already exists
            int exists = 0;
            for (size_t k = 0; k < *count; k++) {
                if (strcmp(unique_tags[k], tags->data[j]) == 0) {
                    exists = 1;
                    break;
                }
            }
            
            if (!exists) {
                if (*count >= tags_capacity) {
                    tags_capacity *= 2;
                    char** temp = realloc(unique_tags, tags_capacity * sizeof(char*));
                    if (!temp) {
                        // Free previously allocated tags
                        for (size_t k = 0; k < *count; k++) {
                            free(unique_tags[k]);
                        }
                        free(unique_tags);
                        *count = 0;
                        return NULL;
                    }
                    unique_tags = temp;
                }
                unique_tags[*count] = string_clone(tags->data[j]);
                (*count)++;
            }
        }
    }
    
    if (*count == 0) {
        free(unique_tags);
        return NULL;
    }
    
    return unique_tags;
}

StringStringMap* summary_manager_get_tag_statistics(SummaryManager* manager) {
    if (!manager) return NULL;
    
    StringStringMap* stats = string_string_map_new();
    if (!stats) return NULL;
    
    for (size_t i = 0; i < manager->summary_tags.size; i++) {
        StringVec* tags = &manager->summary_tags.values[i];
        
        for (size_t j = 0; j < tags->size; j++) {
            char* tag = tags->data[j];
            
            // Check if tag already exists in stats
            char* count_str = string_string_map_get(stats, tag);
            int count = 0;
            
            if (count_str) {
                count = atoi(count_str);
            }
            
            count++;
            
            // Convert count back to string
            char count_buffer[20];
            sprintf(count_buffer, "%d", count);
            
            string_string_map_insert(stats, tag, count_buffer);
        }
    }
    
    return stats;
}

SummaryStatistics* summary_manager_get_summary_statistics(SummaryManager* manager) {
    if (!manager) return NULL;
    
    size_t total_summaries = manager->summaries.size;
    
    size_t high_priority_count = 0;
    for (size_t i = 0; i < manager->summaries.size; i++) {
        Summary* summary = (Summary*)manager->summaries.values[i];
        if (!summary) continue;
        if (summary->priority == SUMMARY_PRIORITY_HIGH || summary->priority == SUMMARY_PRIORITY_CRITICAL) {
            high_priority_count++;
        }
    }
    
    size_t unique_tags_count = 0;
    char** all_tags = summary_manager_get_all_tags(manager, &unique_tags_count);
    if (all_tags) {
        for (size_t i = 0; i < unique_tags_count; i++) {
            free(all_tags[i]);
        }
        free(all_tags);
    }
    
    return summary_statistics_new(total_summaries, high_priority_count, unique_tags_count);
}

SummaryUpdate* summary_update_new() {
    SummaryUpdate* update = malloc(sizeof(SummaryUpdate));
    if (!update) return NULL;
    
    update->content = NULL;
    update->context = NULL;
    update->priority = NULL;
    update->tags = NULL;
    update->has_content = 0;
    update->has_context = 0;
    update->has_priority = 0;
    update->has_tags = 0;
    
    return update;
}

void summary_update_free(SummaryUpdate* update) {
    if (!update) return;
    
    if (update->content) free(update->content);
    if (update->context) free(update->context);
    if (update->priority) free(update->priority);
    if (update->tags) string_vec_free(update->tags);
    
    free(update);
}

SummaryUpdate* summary_update_content(SummaryUpdate* update, const char* content) {
    if (!update || !content) return update;
    
    if (update->content) free(update->content);
    update->content = string_clone(content);
    update->has_content = 1;
    
    return update;
}

SummaryUpdate* summary_update_context(SummaryUpdate* update, const char* context) {
    if (!update || !context) return update;
    
    if (update->context) free(update->context);
    update->context = string_clone(context);
    update->has_context = 1;
    
    return update;
}

SummaryUpdate* summary_update_priority(SummaryUpdate* update, SummaryPriority priority) {
    if (!update) return update;
    
    if (!update->priority) {
        update->priority = malloc(sizeof(SummaryPriority));
        if (!update->priority) return update;
    }
    
    *update->priority = priority;
    update->has_priority = 1;
    
    return update;
}

SummaryUpdate* summary_update_tags(SummaryUpdate* update, StringVec* tags) {
    if (!update || !tags) return update;
    
    if (update->tags) string_vec_free(update->tags);
    update->tags = string_vec_clone(tags);
    update->has_tags = 1;
    
    return update;
}

SummaryStatistics* summary_statistics_new(size_t total, size_t high_priority, size_t unique_tags) {
    SummaryStatistics* stats = malloc(sizeof(SummaryStatistics));
    if (!stats) return NULL;
    
    stats->total_summaries = total;
    stats->high_priority_summaries = high_priority;
    stats->unique_tags = unique_tags;
    
    return stats;
}

void summary_statistics_free(SummaryStatistics* stats) {
    if (!stats) return;
    free(stats);
}

double summary_statistics_high_priority_percentage(SummaryStatistics* stats) {
    if (!stats) return 0.0;
    
    if (stats->total_summaries == 0) {
        return 0.0;
    }
    
    return ((double)stats->high_priority_summaries / (double)stats->total_summaries) * 100.0;
}

Summary* summary_new() {
    Summary* summary = malloc(sizeof(Summary));
    if (!summary) return NULL;
    
    summary->id = NULL;
    summary->content = NULL;
    summary->context = NULL;
    summary->priority = SUMMARY_PRIORITY_MEDIUM;
    summary->tags.data = NULL;
    summary->tags.size = 0;
    summary->tags.capacity = 0;
    
    return summary;
}

void summary_free(Summary* summary) {
    if (!summary) return;
    
    if (summary->id) free(summary->id);
    if (summary->content) free(summary->content);
    if (summary->context) free(summary->context);
    string_vec_free(&summary->tags);
    
    free(summary);
}

StringVec* string_vec_new() {
    StringVec* vec = malloc(sizeof(StringVec));
    if (!vec) return NULL;
    
    vec->data = NULL;
    vec->size = 0;
    vec->capacity = 0;
    
    return vec;
}

void string_vec_free(StringVec* vec) {
    if (!vec) return;
    
    for (size_t i = 0; i < vec->size; i++) {
        free(vec->data[i]);
    }
    free(vec->data);
    vec->data = NULL;
    vec->size = 0;
    vec->capacity = 0;
}

void string_vec_push(StringVec* vec, const char* str) {
    if (!vec || !str) return;
    
    if (vec->size >= vec->capacity) {
        vec->capacity = (vec->capacity == 0) ? 10 : vec->capacity * 2;
        char** temp = realloc(vec->data, vec->capacity * sizeof(char*));
        if (!temp) return;
        vec->data = temp;
    }
    
    vec->data[vec->size] = string_clone(str);
    vec->size++;
}

StringVec* string_vec_clone(StringVec* vec) {
    if (!vec) return NULL;
    
    StringVec* clone = string_vec_new();
    if (!clone) return NULL;
    
    for (size_t i = 0; i < vec->size; i++) {
        string_vec_push(clone, vec->data[i]);
    }
    
    return clone;
}

StringStringMap* string_string_map_new() {
    StringStringMap* map = malloc(sizeof(StringStringMap));
    if (!map) return NULL;
    
    map->keys = NULL;
    map->values = NULL;
    map->size = 0;
    map->capacity = 0;
    
    return map;
}

void string_string_map_free(StringStringMap* map) {
    if (!map) return;
    
    for (size_t i = 0; i < map->size; i++) {
        free(map->keys[i]);
        free(map->values[i]);
    }
    free(map->keys);
    free(map->values);
    free(map);
}

void string_string_map_insert(StringStringMap* map, const char* key, const char* value) {
    if (!map || !key || !value) return;
    
    // Check if key already exists
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            free(map->values[i]);
            map->values[i] = string_clone(value);
            return;
        }
    }
    
    // Add new entry
    if (map->size >= map->capacity) {
        map->capacity = (map->capacity == 0) ? 10 : map->capacity * 2;
        char** temp_keys = realloc(map->keys, map->capacity * sizeof(char*));
        char** temp_values = realloc(map->values, map->capacity * sizeof(char*));
        
        if (!temp_keys || !temp_values) return;
        
        map->keys = temp_keys;
        map->values = temp_values;
    }
    
    map->keys[map->size] = string_clone(key);
    map->values[map->size] = string_clone(value);
    map->size++;
}

char* string_string_map_get(StringStringMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            return map->values[i];
        }
    }
    
    return NULL;
}

int string_string_map_remove(StringStringMap* map, const char* key) {
    if (!map || !key) return 0;
    
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            free(map->keys[i]);
            free(map->values[i]);
            
            // Shift remaining elements
            for (size_t j = i; j < map->size - 1; j++) {
                map->keys[j] = map->keys[j + 1];
                map->values[j] = map->values[j + 1];
            }
            
            map->size--;
            return 1; // Found and removed
        }
    }
    
    return 0; // Not found
}

StringStringVecMap* string_string_vec_map_new() {
    StringStringVecMap* map = malloc(sizeof(StringStringVecMap));
    if (!map) return NULL;
    
    map->keys = NULL;
    map->values = NULL;
    map->size = 0;
    map->capacity = 0;
    
    return map;
}

void string_string_vec_map_free(StringStringVecMap* map) {
    if (!map) return;
    
    for (size_t i = 0; i < map->size; i++) {
        free(map->keys[i]);
        string_vec_free(&map->values[i]);
    }
    free(map->keys);
    free(map->values);
    free(map);
}

void string_string_vec_map_insert(StringStringVecMap* map, const char* key, StringVec* value) {
    if (!map || !key || !value) return;
    
    // Check if key already exists
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            string_vec_free(&map->values[i]);
            StringVec* cloned = string_vec_clone(value);
            if (cloned) {
                map->values[i] = *cloned;
                free(cloned);
            }
            return;
        }
    }
    
    // Add new entry
    if (map->size >= map->capacity) {
        map->capacity = (map->capacity == 0) ? 10 : map->capacity * 2;
        char** temp_keys = realloc(map->keys, map->capacity * sizeof(char*));
        StringVec* temp_values = realloc(map->values, map->capacity * sizeof(StringVec));
        
        if (!temp_keys || !temp_values) return;
        
        map->keys = temp_keys;
        map->values = temp_values;
    }
    
    map->keys[map->size] = string_clone(key);
    StringVec* cloned = string_vec_clone(value);
    if (cloned) {
        map->values[map->size] = *cloned;
        free(cloned);
    }
    map->size++;
}

StringVec* string_string_vec_map_get(StringStringVecMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            return &map->values[i];
        }
    }
    
    return NULL;
}

int string_string_vec_map_remove(StringStringVecMap* map, const char* key) {
    if (!map || !key) return 0;
    
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            free(map->keys[i]);
            string_vec_free(&map->values[i]);
            
            // Shift remaining elements
            for (size_t j = i; j < map->size - 1; j++) {
                map->keys[j] = map->keys[j + 1];
                map->values[j] = map->values[j + 1];
            }
            
            map->size--;
            return 1; // Found and removed
        }
    }
    
    return 0; // Not found
}

DateTime datetime_now() {
    DateTime dt;
    dt.seconds = time(NULL);
    return dt;
}

int datetime_compare(DateTime* a, DateTime* b) {
    if (!a || !b) return 0;
    
    if (a->seconds < b->seconds) return -1;
    if (a->seconds > b->seconds) return 1;
    return 0;
}

TodoziError* todozi_error_new(TodoziErrorType type, const char* message) {
    TodoziError* error = malloc(sizeof(TodoziError));
    if (!error) return NULL;
    
    error->type = type;
    error->message = string_clone(message);
    
    return error;
}

void todozi_error_free(TodoziError* error) {
    if (!error) return;
    
    if (error->message) free(error->message);
    free(error);
}

int parse_summary_format(const char* summary_text, Summary** result, TodoziError** error) {
    if (!summary_text || !result) return 0;
    
    const char* start_tag = "<summary>";
    const char* end_tag = "</summary>";
    
    const char* start = strstr(summary_text, start_tag);
    if (!start) {
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, "Missing <summary> start tag");
        }
        return 0;
    }
    
    const char* end = strstr(summary_text, end_tag);
    if (!end) {
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, "Missing </summary> end tag");
        }
        return 0;
    }
    
    size_t start_len = strlen(start_tag);
    size_t content_len = end - (start + start_len);
    
    if (content_len == 0) {
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, "Empty summary content");
        }
        return 0;
    }
    
    char* content = malloc(content_len + 1);
    if (!content) {
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, "Memory allocation failed");
        }
        return 0;
    }
    
    strncpy(content, start + start_len, content_len);
    content[content_len] = '\0';
    
    // Split by ';'
    char* parts[10]; // Maximum expected parts
    int part_count = 0;
    char* token = strtok(content, ";");
    
    while (token && part_count < 10) {
        parts[part_count] = token;
        part_count++;
        token = strtok(NULL, ";");
    }
    
    if (part_count < 2) {
        free(content);
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, 
                                    "Invalid summary format: need at least 2 parts (content; priority)");
        }
        return 0;
    }
    
    // Parse priority
    SummaryPriority priority;
    if (strcmp(parts[1], "low") == 0) {
        priority = SUMMARY_PRIORITY_LOW;
    } else if (strcmp(parts[1], "medium") == 0) {
        priority = SUMMARY_PRIORITY_MEDIUM;
    } else if (strcmp(parts[1], "high") == 0) {
        priority = SUMMARY_PRIORITY_HIGH;
    } else if (strcmp(parts[1], "critical") == 0) {
        priority = SUMMARY_PRIORITY_CRITICAL;
    } else {
        free(content);
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, "Invalid summary priority");
        }
        return 0;
    }
    
    // Parse context
    char* context = NULL;
    if (part_count > 2 && strlen(parts[2]) > 0) {
        context = string_clone(parts[2]);
    }
    
    // Parse tags
    StringVec tags;
    tags.data = NULL;
    tags.size = 0;
    tags.capacity = 0;
    
    if (part_count > 3 && strlen(parts[3]) > 0) {
        char* tags_str = string_clone(parts[3]);
        char* tag_token = strtok(tags_str, ",");
        
        while (tag_token) {
            // Trim whitespace
            while (*tag_token == ' ') tag_token++;
            char* end = tag_token + strlen(tag_token) - 1;
            while (end > tag_token && *end == ' ') end--;
            *(end + 1) = '\0';
            
            if (strlen(tag_token) > 0) {
                string_vec_push(&tags, tag_token);
            }
            tag_token = strtok(NULL, ",");
        }
        free(tags_str);
    }
    
    // Create summary
    *result = summary_new();
    if (!*result) {
        free(content);
        if (context) free(context);
        string_vec_free(&tags);
        if (error) {
            *error = todozi_error_new(TODOZI_ERROR_VALIDATION, "Memory allocation failed");
        }
        return 0;
    }
    
    (*result)->content = string_clone(parts[0]);
    (*result)->context = context;
    (*result)->priority = priority;
    (*result)->tags = tags;
    (*result)->created_at = datetime_now();
    (*result)->updated_at = datetime_now();
    
    // Generate UUID for id
    uuid_t uuid;
    uuid_generate(uuid);
    (*result)->id = malloc(37);
    uuid_unparse(uuid, (*result)->id);
    
    free(content);
    return 1; // Success
}

// Utility functions implementation

char* string_clone(const char* str) {
    if (!str) return NULL;
    
    size_t len = strlen(str);
    char* clone = malloc(len + 1);
    if (!clone) return NULL;
    
    strcpy(clone, str);
    return clone;
}

char* string_to_lower(const char* str) {
    if (!str) return NULL;
    
    size_t len = strlen(str);
    char* lower = malloc(len + 1);
    if (!lower) return NULL;
    
    for (size_t i = 0; i < len; i++) {
        lower[i] = (str[i] >= 'A' && str[i] <= 'Z') ? str[i] + 32 : str[i];
    }
    lower[len] = '\0';
    
    return lower;
}

static int string_contains(const char* haystack, const char* needle) {
    if (!haystack || !needle) return 0;
    return strstr(haystack, needle) != NULL;
}

int string_equals_ignore_case(const char* a, const char* b) {
    if (!a && !b) return 1;
    if (!a || !b) return 0;
    
    size_t len_a = strlen(a);
    size_t len_b = strlen(b);
    
    if (len_a != len_b) return 0;
    
    for (size_t i = 0; i < len_a; i++) {
        char ca = (a[i] >= 'A' && a[i] <= 'Z') ? a[i] + 32 : a[i];
        char cb = (b[i] >= 'A' && b[i] <= 'Z') ? b[i] + 32 : b[i];
        
        if (ca != cb) return 0;
    }
    
    return 1;
}

// StringPtrMap implementation
StringPtrMap* string_ptr_map_new() {
    StringPtrMap* map = malloc(sizeof(StringPtrMap));
    if (!map) return NULL;
    
    map->keys = NULL;
    map->values = NULL;
    map->size = 0;
    map->capacity = 0;
    
    return map;
}

void string_ptr_map_free(StringPtrMap* map) {
    if (!map) return;
    
    for (size_t i = 0; i < map->size; i++) {
        free(map->keys[i]);
        // Note: values are pointers, caller is responsible for freeing them
    }
    free(map->keys);
    free(map->values);
    free(map);
}

void string_ptr_map_insert(StringPtrMap* map, const char* key, void* value) {
    if (!map || !key) return;
    
    // Check if key already exists
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            // Update existing value (caller must free old value if needed)
            map->values[i] = value;
            return;
        }
    }
    
    // Add new entry
    if (map->size >= map->capacity) {
        map->capacity = (map->capacity == 0) ? 10 : map->capacity * 2;
        char** temp_keys = realloc(map->keys, map->capacity * sizeof(char*));
        void** temp_values = realloc(map->values, map->capacity * sizeof(void*));
        
        if (!temp_keys || !temp_values) return;
        
        map->keys = temp_keys;
        map->values = temp_values;
    }
    
    map->keys[map->size] = string_clone(key);
    map->values[map->size] = value;
    map->size++;
}

void* string_ptr_map_get(StringPtrMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            return map->values[i];
        }
    }
    
    return NULL;
}

int string_ptr_map_remove(StringPtrMap* map, const char* key) {
    if (!map || !key) return 0;
    
    for (size_t i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            free(map->keys[i]);
            // Note: value pointer is not freed here, caller must handle it
            
            // Shift remaining elements
            for (size_t j = i; j < map->size - 1; j++) {
                map->keys[j] = map->keys[j + 1];
                map->values[j] = map->values[j + 1];
            }
            
            map->size--;
            return 1; // Found and removed
        }
    }
    
    return 0; // Not found
}
