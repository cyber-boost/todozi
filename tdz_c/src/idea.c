#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>
#include <ctype.h>
#include <assert.h>

// Forward declarations
typedef struct Idea Idea;
typedef struct IdeaUpdate IdeaUpdate;
typedef struct IdeaManager IdeaManager;
typedef struct IdeaStatistics IdeaStatistics;

// Enum definitions
typedef enum {
    SHARE_LEVEL_PUBLIC,
    SHARE_LEVEL_TEAM,
    SHARE_LEVEL_PRIVATE
} ShareLevel;

typedef enum {
    IDEA_IMPORTANCE_LOW,
    IDEA_IMPORTANCE_MEDIUM,
    IDEA_IMPORTANCE_HIGH,
    IDEA_IMPORTANCE_BREAKTHROUGH
} IdeaImportance;

typedef enum {
    ITEM_STATUS_ACTIVE,
    ITEM_STATUS_COMPLETED,
    ITEM_STATUS_ARCHIVED
} ItemStatus;

// Error handling
typedef enum {
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_NOT_FOUND
} TodoziErrorType;

typedef struct {
    TodoziErrorType type;
    char* message;
} TodoziError;

// Simple hash map implementation for strings
typedef struct StringHashMapNode {
    char* key;
    void* value;
    struct StringHashMapNode* next;
} StringHashMapNode;

typedef struct {
    StringHashMapNode** buckets;
    size_t bucket_count;
    size_t size;
    void (*value_free)(void*); // Function pointer to free values
} StringHashMap;

// Vector implementation for strings
typedef struct {
    char** data;
    size_t size;
    size_t capacity;
} StringVector;

// Vector implementation for ideas
typedef struct {
    Idea** data;
    size_t size;
    size_t capacity;
} IdeaVector;

// Hash set implementation for strings
typedef struct {
    StringHashMap* map;
} StringHashSet;

// Idea structure
struct Idea {
    char* id;
    char* idea;
    char* project_id;
    ItemStatus status;
    ShareLevel share;
    IdeaImportance importance;
    StringVector* tags;
    char* context;
    time_t created_at;
    time_t updated_at;
};

// IdeaUpdate structure
struct IdeaUpdate {
    char* idea;
    ShareLevel* share;
    IdeaImportance* importance;
    StringVector* tags;
    char* context;
};

// IdeaStatistics structure
struct IdeaStatistics {
    size_t total_ideas;
    size_t public_ideas;
    size_t team_ideas;
    size_t private_ideas;
    size_t breakthrough_ideas;
    size_t unique_tags;
};

// IdeaManager structure
struct IdeaManager {
    StringHashMap* ideas;
    StringHashMap* idea_tags;
};

// String vector functions
StringVector* string_vector_new() {
    StringVector* vec = malloc(sizeof(StringVector));
    if (!vec) return NULL;
    vec->data = malloc(sizeof(char*) * 4);
    if (!vec->data) {
        free(vec);
        return NULL;
    }
    vec->size = 0;
    vec->capacity = 4;
    return vec;
}

void string_vector_push(StringVector* vec, char* str) {
    if (!vec || !str) return;
    if (vec->size >= vec->capacity) {
        vec->capacity *= 2;
        char** new_data = realloc(vec->data, sizeof(char*) * vec->capacity);
        if (!new_data) return;
        vec->data = new_data;
    }
    vec->data[vec->size] = str;
    vec->size++;
}

char* string_vector_get(StringVector* vec, size_t index) {
    if (!vec || index >= vec->size) return NULL;
    return vec->data[index];
}

size_t string_vector_size(StringVector* vec) {
    return vec ? vec->size : 0;
}

void string_vector_free(StringVector* vec) {
    if (vec) {
        for (size_t i = 0; i < vec->size; i++) {
            free(vec->data[i]);
        }
        free(vec->data);
        free(vec);
    }
}

// Idea vector functions
IdeaVector* idea_vector_new() {
    IdeaVector* vec = malloc(sizeof(IdeaVector));
    if (!vec) return NULL;
    vec->data = malloc(sizeof(Idea*) * 4);
    if (!vec->data) {
        free(vec);
        return NULL;
    }
    vec->size = 0;
    vec->capacity = 4;
    return vec;
}

void idea_vector_push(IdeaVector* vec, Idea* idea) {
    if (!vec || !idea) return;
    if (vec->size >= vec->capacity) {
        vec->capacity *= 2;
        Idea** new_data = realloc(vec->data, sizeof(Idea*) * vec->capacity);
        if (!new_data) return;
        vec->data = new_data;
    }
    vec->data[vec->size] = idea;
    vec->size++;
}

Idea* idea_vector_get(IdeaVector* vec, size_t index) {
    if (!vec || index >= vec->size) return NULL;
    return vec->data[index];
}

size_t idea_vector_size(IdeaVector* vec) {
    return vec ? vec->size : 0;
}

void idea_vector_free(IdeaVector* vec) {
    if (vec) {
        free(vec->data);
        free(vec);
    }
}

// String hash map functions
StringHashMap* string_hashmap_new(void (*value_free)(void*)) {
    StringHashMap* map = malloc(sizeof(StringHashMap));
    if (!map) return NULL;
    map->bucket_count = 16;
    map->buckets = calloc(map->bucket_count, sizeof(StringHashMapNode*));
    if (!map->buckets) {
        free(map);
        return NULL;
    }
    map->size = 0;
    map->value_free = value_free;
    return map;
}

unsigned int string_hash(const char* str) {
    unsigned int hash = 5381;
    int c;
    while ((c = *str++))
        hash = ((hash << 5) + hash) + c;
    return hash;
}

void string_hashmap_put(StringHashMap* map, const char* key, void* value) {
    if (!map || !key) return;
    unsigned int hash = string_hash(key);
    unsigned int index = hash % map->bucket_count;
    
    StringHashMapNode* node = map->buckets[index];
    while (node) {
        if (strcmp(node->key, key) == 0) {
            if (map->value_free && node->value != value) {
                map->value_free(node->value);
            }
            node->value = value;
            return;
        }
        node = node->next;
    }
    
    StringHashMapNode* new_node = malloc(sizeof(StringHashMapNode));
    if (!new_node) return;
    new_node->key = strdup(key);
    if (!new_node->key) {
        free(new_node);
        return;
    }
    new_node->value = value;
    new_node->next = map->buckets[index];
    map->buckets[index] = new_node;
    map->size++;
}

void* string_hashmap_get(StringHashMap* map, const char* key) {
    if (!map || !key) return NULL;
    unsigned int hash = string_hash(key);
    unsigned int index = hash % map->bucket_count;
    
    StringHashMapNode* node = map->buckets[index];
    while (node) {
        if (strcmp(node->key, key) == 0) {
            return node->value;
        }
        node = node->next;
    }
    return NULL;
}

int string_hashmap_remove(StringHashMap* map, const char* key) {
    if (!map || !key) return 0;
    unsigned int hash = string_hash(key);
    unsigned int index = hash % map->bucket_count;
    
    StringHashMapNode* node = map->buckets[index];
    StringHashMapNode* prev = NULL;
    
    while (node) {
        if (strcmp(node->key, key) == 0) {
            if (prev) {
                prev->next = node->next;
            } else {
                map->buckets[index] = node->next;
            }
            if (map->value_free) {
                map->value_free(node->value);
            }
            free(node->key);
            free(node);
            map->size--;
            return 1;
        }
        prev = node;
        node = node->next;
    }
    return 0;
}

IdeaVector* string_hashmap_values(StringHashMap* map) {
    if (!map) return NULL;
    IdeaVector* vec = idea_vector_new();
    if (!vec) return NULL;
    
    for (size_t i = 0; i < map->bucket_count; i++) {
        StringHashMapNode* node = map->buckets[i];
        while (node) {
            idea_vector_push(vec, (Idea*)node->value);
            node = node->next;
        }
    }
    return vec;
}

void string_hashmap_free(StringHashMap* map) {
    if (map) {
        for (size_t i = 0; i < map->bucket_count; i++) {
            StringHashMapNode* node = map->buckets[i];
            while (node) {
                StringHashMapNode* next = node->next;
                if (map->value_free) {
                    map->value_free(node->value);
                }
                free(node->key);
                free(node);
                node = next;
            }
        }
        free(map->buckets);
        free(map);
    }
}

// String hash set functions
StringHashSet* string_hashset_new() {
    StringHashSet* set = malloc(sizeof(StringHashSet));
    if (!set) return NULL;
    set->map = string_hashmap_new(NULL);
    if (!set->map) {
        free(set);
        return NULL;
    }
    return set;
}

void string_hashset_add(StringHashSet* set, const char* str) {
    if (!set || !str) return;
    string_hashmap_put(set->map, str, (void*)1);
}

int string_hashset_contains(StringHashSet* set, const char* str) {
    if (!set || !str) return 0;
    return string_hashmap_get(set->map, str) != NULL;
}

StringVector* string_hashset_to_vector(StringHashSet* set) {
    if (!set) return NULL;
    StringVector* vec = string_vector_new();
    if (!vec) return NULL;
    
    for (size_t i = 0; i < set->map->bucket_count; i++) {
        StringHashMapNode* node = set->map->buckets[i];
        while (node) {
            string_vector_push(vec, strdup(node->key));
            node = node->next;
        }
    }
    return vec;
}

void string_hashset_free(StringHashSet* set) {
    if (set) {
        string_hashmap_free(set->map);
        free(set);
    }
}

// Helper functions
char* generate_uuid() {
    uuid_t uuid;
    char* uuid_str = malloc(37);
    if (!uuid_str) return NULL;
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    return uuid_str;
}

time_t get_current_time() {
    return time(NULL);
}

// Case-insensitive substring search
int strcasestr_custom(const char* haystack, const char* needle) {
    if (!haystack || !needle) return 0;
    
    size_t haystack_len = strlen(haystack);
    size_t needle_len = strlen(needle);
    
    if (needle_len == 0) return 1;
    if (haystack_len < needle_len) return 0;
    
    for (size_t i = 0; i <= haystack_len - needle_len; i++) {
        int match = 1;
        for (size_t j = 0; j < needle_len; j++) {
            if (tolower(haystack[i + j]) != tolower(needle[j])) {
                match = 0;
                break;
            }
        }
        if (match) return 1;
    }
    return 0;
}

// Convert string to lowercase
char* str_tolower(const char* str) {
    if (!str) return NULL;
    size_t len = strlen(str);
    char* lower = malloc(len + 1);
    if (!lower) return NULL;
    
    for (size_t i = 0; i < len; i++) {
        lower[i] = tolower(str[i]);
    }
    lower[len] = '\0';
    return lower;
}

// Trim whitespace from string
char* str_trim(const char* str) {
    if (!str) return NULL;
    
    // Find start of non-whitespace
    const char* start = str;
    while (*start && isspace(*start)) {
        start++;
    }
    
    // Find end of non-whitespace
    const char* end = str + strlen(str) - 1;
    while (end > start && isspace(*end)) {
        end--;
    }
    
    // Calculate length and copy
    size_t len = end - start + 1;
    char* result = malloc(len + 1);
    if (!result) return NULL;
    
    strncpy(result, start, len);
    result[len] = '\0';
    return result;
}

// Idea functions
Idea* idea_new() {
    Idea* idea = malloc(sizeof(Idea));
    if (!idea) return NULL;
    idea->id = NULL;
    idea->idea = NULL;
    idea->project_id = NULL;
    idea->status = ITEM_STATUS_ACTIVE;
    idea->share = SHARE_LEVEL_PRIVATE;
    idea->importance = IDEA_IMPORTANCE_LOW;
    idea->tags = string_vector_new();
    if (!idea->tags) {
        free(idea);
        return NULL;
    }
    idea->context = NULL;
    idea->created_at = 0;
    idea->updated_at = 0;
    return idea;
}

void idea_free(Idea* idea) {
    if (idea) {
        free(idea->id);
        free(idea->idea);
        free(idea->project_id);
        string_vector_free(idea->tags);
        free(idea->context);
        free(idea);
    }
}

// IdeaUpdate functions
IdeaUpdate* idea_update_new() {
    IdeaUpdate* update = malloc(sizeof(IdeaUpdate));
    if (!update) return NULL;
    update->idea = NULL;
    update->share = NULL;
    update->importance = NULL;
    update->tags = NULL;
    update->context = NULL;
    return update;
}

IdeaUpdate* idea_update_idea(IdeaUpdate* update, const char* idea) {
    if (!update || !idea) return update;
    free(update->idea);
    update->idea = strdup(idea);
    return update;
}

IdeaUpdate* idea_update_share(IdeaUpdate* update, ShareLevel share) {
    if (!update) return update;
    if (!update->share) {
        update->share = malloc(sizeof(ShareLevel));
        if (!update->share) return update;
    }
    *update->share = share;
    return update;
}

IdeaUpdate* idea_update_importance(IdeaUpdate* update, IdeaImportance importance) {
    if (!update) return update;
    if (!update->importance) {
        update->importance = malloc(sizeof(IdeaImportance));
        if (!update->importance) return update;
    }
    *update->importance = importance;
    return update;
}

IdeaUpdate* idea_update_tags(IdeaUpdate* update, StringVector* tags) {
    if (!update) return update;
    update->tags = tags;
    return update;
}

IdeaUpdate* idea_update_context(IdeaUpdate* update, const char* context) {
    if (!update || !context) return update;
    free(update->context);
    update->context = strdup(context);
    return update;
}

void idea_update_free(IdeaUpdate* update) {
    if (update) {
        free(update->idea);
        free(update->share);
        free(update->importance);
        free(update->context);
        free(update);
    }
}

// IdeaStatistics functions
IdeaStatistics* idea_statistics_new() {
    IdeaStatistics* stats = malloc(sizeof(IdeaStatistics));
    if (!stats) return NULL;
    stats->total_ideas = 0;
    stats->public_ideas = 0;
    stats->team_ideas = 0;
    stats->private_ideas = 0;
    stats->breakthrough_ideas = 0;
    stats->unique_tags = 0;
    return stats;
}

double idea_statistics_public_percentage(IdeaStatistics* stats) {
    if (!stats || stats->total_ideas == 0) return 0.0;
    return ((double)stats->public_ideas / (double)stats->total_ideas) * 100.0;
}

double idea_statistics_team_percentage(IdeaStatistics* stats) {
    if (!stats || stats->total_ideas == 0) return 0.0;
    return ((double)stats->team_ideas / (double)stats->total_ideas) * 100.0;
}

double idea_statistics_private_percentage(IdeaStatistics* stats) {
    if (!stats || stats->total_ideas == 0) return 0.0;
    return ((double)stats->private_ideas / (double)stats->total_ideas) * 100.0;
}

double idea_statistics_breakthrough_percentage(IdeaStatistics* stats) {
    if (!stats || stats->total_ideas == 0) return 0.0;
    return ((double)stats->breakthrough_ideas / (double)stats->total_ideas) * 100.0;
}

void idea_statistics_free(IdeaStatistics* stats) {
    free(stats);
}

// IdeaManager functions
IdeaManager* idea_manager_new() {
    IdeaManager* manager = malloc(sizeof(IdeaManager));
    if (!manager) return NULL;
    manager->ideas = string_hashmap_new((void (*)(void*))idea_free);
    if (!manager->ideas) {
        free(manager);
        return NULL;
    }
    manager->idea_tags = string_hashmap_new((void (*)(void*))string_vector_free);
    if (!manager->idea_tags) {
        string_hashmap_free(manager->ideas);
        free(manager);
        return NULL;
    }
    return manager;
}

char* idea_manager_create_idea(IdeaManager* manager, Idea* idea) {
    if (!manager || !idea) return NULL;
    
    idea->id = generate_uuid();
    if (!idea->id) return NULL;
    
    idea->created_at = get_current_time();
    idea->updated_at = get_current_time();
    
    // Clone the tags vector for storage in idea_tags
    StringVector* tags_clone = string_vector_new();
    if (!tags_clone) {
        free(idea->id);
        idea->id = NULL;
        return NULL;
    }
    
    for (size_t i = 0; i < string_vector_size(idea->tags); i++) {
        char* tag = string_vector_get(idea->tags, i);
        char* tag_clone = strdup(tag);
        if (!tag_clone) {
            string_vector_free(tags_clone);
            free(idea->id);
            idea->id = NULL;
            return NULL;
        }
        string_vector_push(tags_clone, tag_clone);
    }
    
    string_hashmap_put(manager->idea_tags, idea->id, tags_clone);
    string_hashmap_put(manager->ideas, idea->id, idea);
    
    return strdup(idea->id);
}

Idea* idea_manager_get_idea(IdeaManager* manager, const char* idea_id) {
    if (!manager || !idea_id) return NULL;
    return (Idea*)string_hashmap_get(manager->ideas, idea_id);
}

IdeaVector* idea_manager_get_all_ideas(IdeaManager* manager) {
    if (!manager) return NULL;
    return string_hashmap_values(manager->ideas);
}

TodoziError* idea_manager_update_idea(IdeaManager* manager, const char* idea_id, IdeaUpdate* updates) {
    if (!manager || !idea_id || !updates) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Invalid parameters");
        }
        return error;
    }
    
    Idea* idea = (Idea*)string_hashmap_get(manager->ideas, idea_id);
    if (!idea) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            char msg[256];
            snprintf(msg, sizeof(msg), "Idea %s not found", idea_id);
            error->message = strdup(msg);
        }
        return error;
    }
    
    if (updates->idea) {
        free(idea->idea);
        idea->idea = strdup(updates->idea);
        if (!idea->idea) {
            TodoziError* error = malloc(sizeof(TodoziError));
            if (error) {
                error->type = TODOZI_ERROR_VALIDATION;
                error->message = strdup("Memory allocation failed");
            }
            return error;
        }
    }
    
    if (updates->share) {
        idea->share = *updates->share;
    }
    
    if (updates->importance) {
        idea->importance = *updates->importance;
    }
    
    if (updates->tags) {
        string_hashmap_remove(manager->idea_tags, idea_id);
        
        // Clone the tags vector for storage in idea_tags
        StringVector* tags_clone = string_vector_new();
        if (!tags_clone) {
            TodoziError* error = malloc(sizeof(TodoziError));
            if (error) {
                error->type = TODOZI_ERROR_VALIDATION;
                error->message = strdup("Memory allocation failed");
            }
            return error;
        }
        
        for (size_t i = 0; i < string_vector_size(updates->tags); i++) {
            char* tag = string_vector_get(updates->tags, i);
            char* tag_clone = strdup(tag);
            if (!tag_clone) {
                string_vector_free(tags_clone);
                TodoziError* error = malloc(sizeof(TodoziError));
                if (error) {
                    error->type = TODOZI_ERROR_VALIDATION;
                    error->message = strdup("Memory allocation failed");
                }
                return error;
            }
            string_vector_push(tags_clone, tag_clone);
        }
        
        string_vector_free(idea->tags);
        idea->tags = updates->tags;
        string_hashmap_put(manager->idea_tags, idea_id, tags_clone);
    }
    
    if (updates->context) {
        free(idea->context);
        idea->context = strdup(updates->context);
        if (!idea->context) {
            TodoziError* error = malloc(sizeof(TodoziError));
            if (error) {
                error->type = TODOZI_ERROR_VALIDATION;
                error->message = strdup("Memory allocation failed");
            }
            return error;
        }
    }
    
    idea->updated_at = get_current_time();
    
    return NULL; // Success
}

TodoziError* idea_manager_delete_idea(IdeaManager* manager, const char* idea_id) {
    if (!manager || !idea_id) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Invalid parameters");
        }
        return error;
    }
    
    if (string_hashmap_remove(manager->ideas, idea_id)) {
        string_hashmap_remove(manager->idea_tags, idea_id);
        return NULL; // Success
    } else {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            char msg[256];
            snprintf(msg, sizeof(msg), "Idea %s not found", idea_id);
            error->message = strdup(msg);
        }
        return error;
    }
}

IdeaVector* idea_manager_search_ideas(IdeaManager* manager, const char* query) {
    if (!manager || !query) return NULL;
    
    IdeaVector* result = idea_vector_new();
    if (!result) return NULL;
    
    char* query_lower = str_tolower(query);
    if (!query_lower) {
        idea_vector_free(result);
        return NULL;
    }
    
    IdeaVector* all_ideas = string_hashmap_values(manager->ideas);
    if (!all_ideas) {
        free(query_lower);
        idea_vector_free(result);
        return NULL;
    }
    
    for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
        Idea* idea = idea_vector_get(all_ideas, i);
        int match = 0;
        
        // Check idea text
        char* idea_lower = str_tolower(idea->idea);
        if (idea_lower) {
            if (strcasestr_custom(idea_lower, query_lower)) {
                match = 1;
            }
            free(idea_lower);
        }
        
        // Check tags
        if (!match) {
            for (size_t j = 0; j < string_vector_size(idea->tags); j++) {
                char* tag = string_vector_get(idea->tags, j);
                char* tag_lower = str_tolower(tag);
                if (tag_lower) {
                    if (strcasestr_custom(tag_lower, query_lower)) {
                        match = 1;
                        free(tag_lower);
                        break;
                    }
                    free(tag_lower);
                }
            }
        }
        
        // Check context
        if (!match && idea->context) {
            char* context_lower = str_tolower(idea->context);
            if (context_lower) {
                if (strcasestr_custom(context_lower, query_lower)) {
                    match = 1;
                }
                free(context_lower);
            }
        }
        
        if (match) {
            idea_vector_push(result, idea);
        }
    }
    
    free(query_lower);
    idea_vector_free(all_ideas);
    return result;
}

IdeaVector* idea_manager_get_ideas_by_importance(IdeaManager* manager, IdeaImportance importance) {
    if (!manager) return NULL;
    
    IdeaVector* result = idea_vector_new();
    if (!result) return NULL;
    
    IdeaVector* all_ideas = string_hashmap_values(manager->ideas);
    if (!all_ideas) {
        idea_vector_free(result);
        return NULL;
    }
    
    for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
        Idea* idea = idea_vector_get(all_ideas, i);
        if (idea->importance == importance) {
            idea_vector_push(result, idea);
        }
    }
    
    idea_vector_free(all_ideas);
    return result;
}

IdeaVector* idea_manager_get_ideas_by_share_level(IdeaManager* manager, ShareLevel share_level) {
    if (!manager) return NULL;
    
    IdeaVector* result = idea_vector_new();
    if (!result) return NULL;
    
    IdeaVector* all_ideas = string_hashmap_values(manager->ideas);
    if (!all_ideas) {
        idea_vector_free(result);
        return NULL;
    }
    
    for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
        Idea* idea = idea_vector_get(all_ideas, i);
        if (idea->share == share_level) {
            idea_vector_push(result, idea);
        }
    }
    
    idea_vector_free(all_ideas);
    return result;
}

IdeaVector* idea_manager_get_ideas_by_tag(IdeaManager* manager, const char* tag) {
    if (!manager || !tag) return NULL;
    
    IdeaVector* result = idea_vector_new();
    if (!result) return NULL;
    
    char* tag_lower = str_tolower(tag);
    if (!tag_lower) {
        idea_vector_free(result);
        return NULL;
    }
    
    IdeaVector* all_ideas = string_hashmap_values(manager->ideas);
    if (!all_ideas) {
        free(tag_lower);
        idea_vector_free(result);
        return NULL;
    }
    
    for (size_t i = 0; i < idea_vector_size(all_ideas); i++) {
        Idea* idea = idea_vector_get(all_ideas, i);
        int found = 0;
        
        for (size_t j = 0; j < string_vector_size(idea->tags); j++) {
            char* idea_tag = string_vector_get(idea->tags, j);
            char* idea_tag_lower = str_tolower(idea_tag);
            if (idea_tag_lower) {
                if (strcmp(idea_tag_lower, tag_lower) == 0) {
                    found = 1;
                    free(idea_tag_lower);
                    break;
                }
                free(idea_tag_lower);
            }
        }
        
        if (found) {
            idea_vector_push(result, idea);
        }
    }
    
    free(tag_lower);
    idea_vector_free(all_ideas);
    return result;
}

IdeaVector* idea_manager_get_public_ideas(IdeaManager* manager) {
    return idea_manager_get_ideas_by_share_level(manager, SHARE_LEVEL_PUBLIC);
}

IdeaVector* idea_manager_get_team_ideas(IdeaManager* manager) {
    return idea_manager_get_ideas_by_share_level(manager, SHARE_LEVEL_TEAM);
}

IdeaVector* idea_manager_get_private_ideas(IdeaManager* manager) {
    return idea_manager_get_ideas_by_share_level(manager, SHARE_LEVEL_PRIVATE);
}

IdeaVector* idea_manager_get_breakthrough_ideas(IdeaManager* manager) {
    return idea_manager_get_ideas_by_importance(manager, IDEA_IMPORTANCE_BREAKTHROUGH);
}

// Comparison function for sorting ideas by creation time (descending)
int idea_compare_by_created_at_desc(const void* a, const void* b) {
    Idea* idea_a = *(Idea**)a;
    Idea* idea_b = *(Idea**)b;
    if (idea_a->created_at > idea_b->created_at) return -1;
    if (idea_a->created_at < idea_b->created_at) return 1;
    return 0;
}

IdeaVector* idea_manager_get_recent_ideas(IdeaManager* manager, size_t limit) {
    if (!manager) return NULL;
    
    IdeaVector* all_ideas = string_hashmap_values(manager->ideas);
    if (!all_ideas) return NULL;
    
    // Sort by creation time (descending)
    qsort(all_ideas->data, all_ideas->size, sizeof(Idea*), idea_compare_by_created_at_desc);
    
    // Create result vector with limited size
    IdeaVector* result = idea_vector_new();
    if (!result) {
        idea_vector_free(all_ideas);
        return NULL;
    }
    
    size_t count = (limit < all_ideas->size) ? limit : all_ideas->size;
    for (size_t i = 0; i < count; i++) {
        idea_vector_push(result, idea_vector_get(all_ideas, i));
    }
    
    idea_vector_free(all_ideas);
    return result;
}

StringVector* idea_manager_get_all_tags(IdeaManager* manager) {
    if (!manager) return NULL;
    
    StringHashSet* all_tags = string_hashset_new();
    if (!all_tags) return NULL;
    
    for (size_t i = 0; i < manager->idea_tags->bucket_count; i++) {
        StringHashMapNode* node = manager->idea_tags->buckets[i];
        while (node) {
            StringVector* tags = (StringVector*)node->value;
            for (size_t j = 0; j < string_vector_size(tags); j++) {
                char* tag = string_vector_get(tags, j);
                string_hashset_add(all_tags, tag);
            }
            node = node->next;
        }
    }
    
    StringVector* result = string_hashset_to_vector(all_tags);
    string_hashset_free(all_tags);
    return result;
}

StringHashMap* idea_manager_get_tag_statistics(IdeaManager* manager) {
    if (!manager) return NULL;
    
    StringHashMap* stats = string_hashmap_new(free);
    if (!stats) return NULL;
    
    for (size_t i = 0; i < manager->idea_tags->bucket_count; i++) {
        StringHashMapNode* node = manager->idea_tags->buckets[i];
        while (node) {
            StringVector* tags = (StringVector*)node->value;
            for (size_t j = 0; j < string_vector_size(tags); j++) {
                char* tag = string_vector_get(tags, j);
                size_t* count = (size_t*)string_hashmap_get(stats, tag);
                if (count) {
                    (*count)++;
                } else {
                    size_t* new_count = malloc(sizeof(size_t));
                    if (new_count) {
                        *new_count = 1;
                        string_hashmap_put(stats, tag, new_count);
                    }
                }
            }
            node = node->next;
        }
    }
    
    return stats;
}

IdeaStatistics* idea_manager_get_idea_statistics(IdeaManager* manager) {
    if (!manager) return NULL;
    
    IdeaStatistics* stats = idea_statistics_new();
    if (!stats) return NULL;
    
    stats->total_ideas = manager->ideas->size;
    
    IdeaVector* public_ideas = idea_manager_get_public_ideas(manager);
    if (public_ideas) {
        stats->public_ideas = idea_vector_size(public_ideas);
        idea_vector_free(public_ideas);
    }
    
    IdeaVector* team_ideas = idea_manager_get_team_ideas(manager);
    if (team_ideas) {
        stats->team_ideas = idea_vector_size(team_ideas);
        idea_vector_free(team_ideas);
    }
    
    IdeaVector* private_ideas = idea_manager_get_private_ideas(manager);
    if (private_ideas) {
        stats->private_ideas = idea_vector_size(private_ideas);
        idea_vector_free(private_ideas);
    }
    
    IdeaVector* breakthrough_ideas = idea_manager_get_breakthrough_ideas(manager);
    if (breakthrough_ideas) {
        stats->breakthrough_ideas = idea_vector_size(breakthrough_ideas);
        idea_vector_free(breakthrough_ideas);
    }
    
    StringVector* all_tags = idea_manager_get_all_tags(manager);
    if (all_tags) {
        stats->unique_tags = string_vector_size(all_tags);
        string_vector_free(all_tags);
    }
    
    return stats;
}

void idea_manager_free(IdeaManager* manager) {
    if (manager) {
        string_hashmap_free(manager->ideas);
        string_hashmap_free(manager->idea_tags);
        free(manager);
    }
}

// Parse importance from string
int parse_importance(const char* str, IdeaImportance* out_importance) {
    if (!str || !out_importance) return 0;
    
    char* lower = str_tolower(str);
    if (!lower) return 0;
    
    int result = 1;
    if (strcmp(lower, "low") == 0) {
        *out_importance = IDEA_IMPORTANCE_LOW;
    } else if (strcmp(lower, "medium") == 0) {
        *out_importance = IDEA_IMPORTANCE_MEDIUM;
    } else if (strcmp(lower, "high") == 0) {
        *out_importance = IDEA_IMPORTANCE_HIGH;
    } else if (strcmp(lower, "breakthrough") == 0) {
        *out_importance = IDEA_IMPORTANCE_BREAKTHROUGH;
    } else {
        result = 0;
    }
    
    free(lower);
    return result;
}

// Parse share level from string
ShareLevel parse_share_level(const char* str) {
    if (!str) return SHARE_LEVEL_PRIVATE;
    
    char* lower = str_tolower(str);
    if (!lower) return SHARE_LEVEL_PRIVATE;
    
    ShareLevel result = SHARE_LEVEL_PRIVATE;
    if (strcmp(lower, "share") == 0) {
        result = SHARE_LEVEL_PUBLIC;
    } else if (strcmp(lower, "dont share") == 0 || 
               strcmp(lower, "don't share") == 0 || 
               strcmp(lower, "private") == 0) {
        result = SHARE_LEVEL_PRIVATE;
    } else if (strcmp(lower, "team") == 0) {
        result = SHARE_LEVEL_TEAM;
    }
    
    free(lower);
    return result;
}

// Parse idea format function
TodoziError* parse_idea_format(const char* idea_text, Idea** out_idea) {
    if (!idea_text || !out_idea) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Invalid parameters");
        }
        return error;
    }
    
    const char* start_tag = "<idea>";
    const char* end_tag = "</idea>";
    
    const char* start = strstr(idea_text, start_tag);
    if (!start) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Missing <idea> start tag");
        }
        return error;
    }
    
    const char* end = strstr(idea_text, end_tag);
    if (!end) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Missing </idea> end tag");
        }
        return error;
    }
    
    if (end <= start) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Invalid idea format");
        }
        return error;
    }
    
    size_t content_len = end - (start + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (!content) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Memory allocation failed");
        }
        return error;
    }
    
    strncpy(content, start + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split content by ';'
    StringVector* parts = string_vector_new();
    if (!parts) {
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Memory allocation failed");
        }
        return error;
    }
    
    char* token = strtok(content, ";");
    while (token) {
        char* trimmed = str_trim(token);
        if (trimmed) {
            string_vector_push(parts, trimmed);
        }
        token = strtok(NULL, ";");
    }
    
    if (string_vector_size(parts) < 3) {
        string_vector_free(parts);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Invalid idea format: need at least 3 parts (idea; share; importance)");
        }
        return error;
    }
    
    // Create idea
    Idea* idea = idea_new();
    if (!idea) {
        string_vector_free(parts);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Memory allocation failed");
        }
        return error;
    }
    
    // Set idea text
    char* idea_text_part = string_vector_get(parts, 0);
    idea->idea = strdup(idea_text_part ? idea_text_part : "");
    if (!idea->idea) {
        idea_free(idea);
        string_vector_free(parts);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Memory allocation failed");
        }
        return error;
    }
    
    // Set share level
    char* share_part = string_vector_get(parts, 1);
    idea->share = parse_share_level(share_part);
    
    // Set importance
    char* importance_part = string_vector_get(parts, 2);
    if (!parse_importance(importance_part, &idea->importance)) {
        idea_free(idea);
        string_vector_free(parts);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (error) {
            error->type = TODOZI_ERROR_VALIDATION;
            error->message = strdup("Invalid idea importance");
        }
        return error;
    }
    
    // Set tags (if present)
    if (string_vector_size(parts) > 3) {
        char* tags_part = string_vector_get(parts, 3);
        if (tags_part && strlen(tags_part) > 0) {
            char* tags_copy = strdup(tags_part);
            if (tags_copy) {
                char* tag_token = strtok(tags_copy, ",");
                while (tag_token) {
                    char* trimmed_tag = str_trim(tag_token);
                    if (trimmed_tag) {
                        string_vector_push(idea->tags, trimmed_tag);
                    }
                    tag_token = strtok(NULL, ",");
                }
                free(tags_copy);
            }
        }
    }
    
    // Set context (if present)
    if (string_vector_size(parts) > 4) {
        char* context_part = string_vector_get(parts, 4);
        if (context_part && strlen(context_part) > 0) {
            idea->context = strdup(context_part);
            if (!idea->context) {
                idea_free(idea);
                string_vector_free(parts);
                free(content);
                TodoziError* error = malloc(sizeof(TodoziError));
                if (error) {
                    error->type = TODOZI_ERROR_VALIDATION;
                    error->message = strdup("Memory allocation failed");
                }
                return error;
            }
        }
    }
    
    // Set timestamps
    idea->created_at = get_current_time();
    idea->updated_at = get_current_time();
    
    // Clean up
    string_vector_free(parts);
    free(content);
    
    *out_idea = idea;
    return NULL; // Success
}

void todozi_error_free(TodoziError* error) {
    if (error) {
        free(error->message);
        free(error);
    }
}

// Test functions
void test_idea_manager_creation() {
    IdeaManager* manager = idea_manager_new();
    assert(manager != NULL);
    assert(manager->ideas->size == 0);
    assert(manager->idea_tags->size == 0);
    idea_manager_free(manager);
    printf("test_idea_manager_creation: PASSED\n");
}

void test_idea_update_builder() {
    IdeaUpdate* update = idea_update_new();
    assert(update != NULL);
    
    update = idea_update_idea(update, "New idea");
    update = idea_update_share(update, SHARE_LEVEL_PUBLIC);
    update = idea_update_importance(update, IDEA_IMPORTANCE_HIGH);
    
    assert(update->idea != NULL);
    assert(strcmp(update->idea, "New idea") == 0);
    assert(update->share != NULL);
    assert(*update->share == SHARE_LEVEL_PUBLIC);
    assert(update->importance != NULL);
    assert(*update->importance == IDEA_IMPORTANCE_HIGH);
    
    idea_update_free(update);
    printf("test_idea_update_builder: PASSED\n");
}

void test_idea_statistics_percentages() {
    IdeaStatistics stats = {
        .total_ideas = 10,
        .public_ideas = 4,
        .team_ideas = 3,
        .private_ideas = 3,
        .breakthrough_ideas = 2,
        .unique_tags = 8
    };
    
    assert(idea_statistics_public_percentage(&stats) == 40.0);
    assert(idea_statistics_team_percentage(&stats) == 30.0);
    assert(idea_statistics_private_percentage(&stats) == 30.0);
    assert(idea_statistics_breakthrough_percentage(&stats) == 20.0);
    
    IdeaStatistics empty_stats = {
        .total_ideas = 0,
        .public_ideas = 0,
        .team_ideas = 0,
        .private_ideas = 0,
        .breakthrough_ideas = 0,
        .unique_tags = 0
    };
    
    assert(idea_statistics_public_percentage(&empty_stats) == 0.0);
    assert(idea_statistics_team_percentage(&empty_stats) == 0.0);
    assert(idea_statistics_private_percentage(&empty_stats) == 0.0);
    assert(idea_statistics_breakthrough_percentage(&empty_stats) == 0.0);
    
    printf("test_idea_statistics_percentages: PASSED\n");
}

void test_parse_idea_format() {
    const char* idea_text = "<idea>Use microservices for better scalability; share; high; architecture,microservices,scalability; This will improve deployment speed</idea>";
    Idea* idea = NULL;
    TodoziError* error = parse_idea_format(idea_text, &idea);
    
    assert(error == NULL);
    assert(idea != NULL);
    assert(strcmp(idea->idea, "Use microservices for better scalability") == 0);
    assert(idea->share == SHARE_LEVEL_PUBLIC);
    assert(idea->importance == IDEA_IMPORTANCE_HIGH);
    assert(string_vector_size(idea->tags) == 3);
    assert(strcmp(string_vector_get(idea->tags, 0), "architecture") == 0);
    assert(strcmp(string_vector_get(idea->tags, 1), "microservices") == 0);
    assert(strcmp(string_vector_get(idea->tags, 2), "scalability") == 0);
    assert(idea->context != NULL);
    assert(strcmp(idea->context, "This will improve deployment speed") == 0);
    
    idea_free(idea);
    printf("test_parse_idea_format: PASSED\n");
}

void test_parse_idea_format_minimal() {
    const char* idea_text = "<idea>Simple idea; private; low</idea>";
    Idea* idea = NULL;
    TodoziError* error = parse_idea_format(idea_text, &idea);
    
    assert(error == NULL);
    assert(idea != NULL);
    assert(strcmp(idea->idea, "Simple idea") == 0);
    assert(idea->share == SHARE_LEVEL_PRIVATE);
    assert(idea->importance == IDEA_IMPORTANCE_LOW);
    assert(string_vector_size(idea->tags) == 0);
    assert(idea->context == NULL);
    
    idea_free(idea);
    printf("test_parse_idea_format_minimal: PASSED\n");
}

int main() {
    test_idea_manager_creation();
    test_idea_update_builder();
    test_idea_statistics_percentages();
    test_parse_idea_format();
    test_parse_idea_format_minimal();
    return 0;
}