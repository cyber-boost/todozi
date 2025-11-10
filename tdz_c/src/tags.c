#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>
#include <ctype.h>

// Forward declarations
typedef struct Tag Tag;
typedef struct TagManager TagManager;
typedef struct TagUpdate TagUpdate;
typedef struct TagStatistics TagStatistics;
typedef struct TagSearchEngine TagSearchEngine;
typedef struct TagSearchQuery TagSearchQuery;

// Error handling
typedef enum {
    TODOZI_SUCCESS,
    TODOZI_VALIDATION_ERROR,
    TODOZI_OUT_OF_MEMORY
} TodoziError;

// Hash map structures (simplified)
typedef struct HashMapEntry {
    char* key;
    void* value;
    struct HashMapEntry* next;
} HashMapEntry;

typedef struct {
    HashMapEntry** buckets;
    size_t size;
    size_t capacity;
} HashMap;

// Vector structure
typedef struct {
    void** data;
    size_t size;
    size_t capacity;
} Vector;

// DateTime structure
typedef struct {
    time_t timestamp;
} DateTime;

// Tag structure
struct Tag {
    char* id;
    char* name;
    char* description;
    char* color;
    char* category;
    unsigned int usage_count;
    DateTime created_at;
    DateTime updated_at;
};

// TagUpdate structure
struct TagUpdate {
    char* name;
    char* description;
    char* color;
    char* category;
};

// TagStatistics structure
struct TagStatistics {
    size_t total_tags;
    size_t total_categories;
    size_t total_relationships;
    double average_usage;
};

// TagSortBy enum
typedef enum {
    TAG_SORT_BY_NAME,
    TAG_SORT_BY_USAGE,
    TAG_SORT_BY_CREATED,
    TAG_SORT_BY_UPDATED
} TagSortBy;

// TagSearchQuery structure
struct TagSearchQuery {
    char* name_contains;
    char* description_contains;
    char* category;
    char* color;
    unsigned int* min_usage;
    unsigned int* max_usage;
    TagSortBy sort_by;
    size_t* limit;
};

// TagManager structure
struct TagManager {
    HashMap* tags;
    HashMap* tag_relationships;
    HashMap* category_tags;
};

// TagSearchEngine structure
struct TagSearchEngine {
    TagManager* tag_manager;
};

// Vector functions
Vector* vector_create() {
    Vector* vec = malloc(sizeof(Vector));
    if (!vec) return NULL;
    vec->data = malloc(sizeof(void*) * 8);
    if (!vec->data) {
        free(vec);
        return NULL;
    }
    vec->size = 0;
    vec->capacity = 8;
    return vec;
}

void vector_push(Vector* vec, void* item) {
    if (!vec) return;
    if (vec->size >= vec->capacity) {
        size_t new_capacity = vec->capacity * 2;
        void** new_data = realloc(vec->data, sizeof(void*) * new_capacity);
        if (!new_data) return;
        vec->data = new_data;
        vec->capacity = new_capacity;
    }
    vec->data[vec->size++] = item;
}

// Helper function to remove an element from vector by value comparison
// Returns 1 if found and removed, 0 otherwise
int vector_remove_by_value(Vector* vec, void* value, int (*compare)(const void*, const void*), void (*free_func)(void*)) {
    if (!vec || !value || !compare) return 0;
    for (size_t i = 0; i < vec->size; i++) {
        if (compare(vec->data[i], value) == 0) {
            if (free_func) {
                free_func(vec->data[i]);
            }
            // Shift elements left
            for (size_t j = i; j < vec->size - 1; j++) {
                vec->data[j] = vec->data[j + 1];
            }
            vec->size--;
            return 1;
        }
    }
    return 0;
}

void* vector_get(Vector* vec, size_t index) {
    if (!vec || index >= vec->size) return NULL;
    return vec->data[index];
}

void vector_free(Vector* vec, void (*free_func)(void*)) {
    if (!vec) return;
    if (free_func) {
        for (size_t i = 0; i < vec->size; i++) {
            free_func(vec->data[i]);
        }
    }
    free(vec->data);
    free(vec);
}

size_t vector_size(Vector* vec) {
    return vec ? vec->size : 0;
}

void vector_clear(Vector* vec, void (*free_func)(void*)) {
    if (!vec) return;
    if (free_func) {
        for (size_t i = 0; i < vec->size; i++) {
            free_func(vec->data[i]);
        }
    }
    vec->size = 0;
}

void vector_truncate(Vector* vec, size_t new_size, void (*free_func)(void*)) {
    if (!vec || new_size >= vec->size) return;
    if (free_func) {
        for (size_t i = new_size; i < vec->size; i++) {
            free_func(vec->data[i]);
        }
    }
    vec->size = new_size;
}

// HashMap functions
HashMap* hashmap_create() {
    HashMap* map = malloc(sizeof(HashMap));
    if (!map) return NULL;
    map->capacity = 16;
    map->size = 0;
    map->buckets = calloc(map->capacity, sizeof(HashMapEntry*));
    if (!map->buckets) {
        free(map);
        return NULL;
    }
    return map;
}

void hashmap_free(HashMap* map, void (*free_key)(void*), void (*free_value)(void*)) {
    if (!map) return;
    for (size_t i = 0; i < map->capacity; i++) {
        HashMapEntry* entry = map->buckets[i];
        while (entry) {
            HashMapEntry* next = entry->next;
            if (free_key) free_key(entry->key);
            if (free_value) free_value(entry->value);
            free(entry);
            entry = next;
        }
    }
    free(map->buckets);
    free(map);
}

size_t hashmap_hash(const char* key, size_t capacity) {
    size_t hash = 5381;
    int c;
    while ((c = *key++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash % capacity;
}

TodoziError hashmap_put(HashMap* map, const char* key, void* value) {
    if (!map || !key) return TODOZI_VALIDATION_ERROR;
    size_t index = hashmap_hash(key, map->capacity);
    HashMapEntry* entry = map->buckets[index];
    
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            entry->value = value;
            return TODOZI_SUCCESS;
        }
        entry = entry->next;
    }
    
    HashMapEntry* new_entry = malloc(sizeof(HashMapEntry));
    if (!new_entry) return TODOZI_OUT_OF_MEMORY;
    new_entry->key = strdup(key);
    if (!new_entry->key) {
        free(new_entry);
        return TODOZI_OUT_OF_MEMORY;
    }
    new_entry->value = value;
    new_entry->next = map->buckets[index];
    map->buckets[index] = new_entry;
    map->size++;
    return TODOZI_SUCCESS;
}

void* hashmap_get(HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    size_t index = hashmap_hash(key, map->capacity);
    HashMapEntry* entry = map->buckets[index];
    
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            return entry->value;
        }
        entry = entry->next;
    }
    return NULL;
}

void* hashmap_remove(HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    size_t index = hashmap_hash(key, map->capacity);
    HashMapEntry* entry = map->buckets[index];
    HashMapEntry* prev = NULL;
    
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            if (prev) {
                prev->next = entry->next;
            } else {
                map->buckets[index] = entry->next;
            }
            void* value = entry->value;
            free(entry->key);
            free(entry);
            map->size--;
            return value;
        }
        prev = entry;
        entry = entry->next;
    }
    return NULL;
}

Vector* hashmap_keys(HashMap* map) {
    if (!map) return NULL;
    Vector* keys = vector_create();
    if (!keys) return NULL;
    for (size_t i = 0; i < map->capacity; i++) {
        HashMapEntry* entry = map->buckets[i];
        while (entry) {
            char* key_copy = strdup(entry->key);
            if (!key_copy) {
                vector_free(keys, free);
                return NULL;
            }
            vector_push(keys, key_copy);
            entry = entry->next;
        }
    }
    return keys;
}

Vector* hashmap_values(HashMap* map) {
    if (!map) return NULL;
    Vector* values = vector_create();
    if (!values) return NULL;
    for (size_t i = 0; i < map->capacity; i++) {
        HashMapEntry* entry = map->buckets[i];
        while (entry) {
            vector_push(values, entry->value);
            entry = entry->next;
        }
    }
    return values;
}

size_t hashmap_size(HashMap* map) {
    return map ? map->size : 0;
}

// DateTime functions
DateTime datetime_now() {
    DateTime dt;
    dt.timestamp = time(NULL);
    return dt;
}

// Utility functions
char* generate_uuid() {
    uuid_t uuid;
    char* uuid_str = malloc(37);
    if (!uuid_str) return NULL;
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    return uuid_str;
}

char* string_clone(const char* str) {
    if (!str) return NULL;
    char* clone = malloc(strlen(str) + 1);
    if (clone) strcpy(clone, str);
    return clone;
}

void string_to_lowercase(char* str) {
    for (int i = 0; str[i]; i++) {
        str[i] = tolower(str[i]);
    }
}

char* string_to_lowercase_clone(const char* str) {
    if (!str) return NULL;
    char* clone = string_clone(str);
    if (clone) string_to_lowercase(clone);
    return clone;
}

int string_contains(const char* str, const char* substr) {
    if (!str || !substr) return 0;
    char* str_lower = string_to_lowercase_clone(str);
    if (!str_lower) return 0;
    char* substr_lower = string_to_lowercase_clone(substr);
    if (!substr_lower) {
        free(str_lower);
        return 0;
    }
    int result = (strstr(str_lower, substr_lower) != NULL);
    free(str_lower);
    free(substr_lower);
    return result;
}

// Tag functions
Tag* tag_create() {
    Tag* tag = malloc(sizeof(Tag));
    if (!tag) return NULL;
    tag->id = NULL;
    tag->name = NULL;
    tag->description = NULL;
    tag->color = NULL;
    tag->category = NULL;
    tag->usage_count = 0;
    tag->created_at = datetime_now();
    tag->updated_at = datetime_now();
    return tag;
}

void tag_free(Tag* tag) {
    if (tag) {
        free(tag->id);
        free(tag->name);
        free(tag->description);
        free(tag->color);
        free(tag->category);
        free(tag);
    }
}

Tag* tag_clone(Tag* tag) {
    if (!tag) return NULL;
    Tag* clone = tag_create();
    if (!clone) return NULL;
    clone->id = string_clone(tag->id);
    clone->name = string_clone(tag->name);
    clone->description = string_clone(tag->description);
    clone->color = string_clone(tag->color);
    clone->category = string_clone(tag->category);
    clone->usage_count = tag->usage_count;
    clone->created_at = tag->created_at;
    clone->updated_at = tag->updated_at;
    return clone;
}

// TagUpdate functions
TagUpdate* tag_update_create() {
    TagUpdate* update = malloc(sizeof(TagUpdate));
    if (!update) return NULL;
    update->name = NULL;
    update->description = NULL;
    update->color = NULL;
    update->category = NULL;
    return update;
}

void tag_update_free(TagUpdate* update) {
    if (update) {
        free(update->name);
        free(update->description);
        free(update->color);
        free(update->category);
        free(update);
    }
}

TagUpdate* tag_update_new() {
    return tag_update_create();
}

TagUpdate* tag_update_name(TagUpdate* update, const char* name) {
    if (update) {
        free(update->name);
        update->name = string_clone(name);
    }
    return update;
}

TagUpdate* tag_update_description(TagUpdate* update, const char* description) {
    if (update) {
        free(update->description);
        update->description = string_clone(description);
    }
    return update;
}

TagUpdate* tag_update_color(TagUpdate* update, const char* color) {
    if (update) {
        free(update->color);
        update->color = string_clone(color);
    }
    return update;
}

TagUpdate* tag_update_category(TagUpdate* update, const char* category) {
    if (update) {
        free(update->category);
        update->category = string_clone(category);
    }
    return update;
}

// TagStatistics functions
TagStatistics* tag_statistics_create() {
    TagStatistics* stats = malloc(sizeof(TagStatistics));
    if (!stats) return NULL;
    stats->total_tags = 0;
    stats->total_categories = 0;
    stats->total_relationships = 0;
    stats->average_usage = 0.0;
    return stats;
}

void tag_statistics_free(TagStatistics* stats) {
    free(stats);
}

double tag_statistics_relationships_per_tag(TagStatistics* stats) {
    if (!stats || stats->total_tags == 0) {
        return 0.0;
    }
    return (double)stats->total_relationships / (double)stats->total_tags;
}

// TagSearchQuery functions
TagSearchQuery* tag_search_query_create() {
    TagSearchQuery* query = malloc(sizeof(TagSearchQuery));
    if (!query) return NULL;
    query->name_contains = NULL;
    query->description_contains = NULL;
    query->category = NULL;
    query->color = NULL;
    query->min_usage = NULL;
    query->max_usage = NULL;
    query->sort_by = TAG_SORT_BY_NAME;
    query->limit = NULL;
    return query;
}

void tag_search_query_free(TagSearchQuery* query) {
    if (query) {
        free(query->name_contains);
        free(query->description_contains);
        free(query->category);
        free(query->color);
        free(query->min_usage);
        free(query->max_usage);
        free(query->limit);
        free(query);
    }
}

// TagManager functions
TagManager* tag_manager_create() {
    TagManager* manager = malloc(sizeof(TagManager));
    if (!manager) return NULL;
    manager->tags = hashmap_create();
    if (!manager->tags) {
        free(manager);
        return NULL;
    }
    manager->tag_relationships = hashmap_create();
    if (!manager->tag_relationships) {
        hashmap_free(manager->tags, free, NULL);
        free(manager);
        return NULL;
    }
    manager->category_tags = hashmap_create();
    if (!manager->category_tags) {
        hashmap_free(manager->tags, free, NULL);
        hashmap_free(manager->tag_relationships, free, NULL);
        free(manager);
        return NULL;
    }
    return manager;
}

void tag_manager_free(TagManager* manager) {
    if (manager) {
        hashmap_free(manager->tags, free, (void (*)(void*))tag_free);
        hashmap_free(manager->tag_relationships, free, (void (*)(void*))vector_free);
        hashmap_free(manager->category_tags, free, (void (*)(void*))vector_free);
        free(manager);
    }
}

TodoziError tag_manager_create_tag(TagManager* manager, Tag* tag, char** out_id) {
    if (!manager || !tag) return TODOZI_VALIDATION_ERROR;
    
    char* new_id = generate_uuid();
    if (!new_id) return TODOZI_OUT_OF_MEMORY;
    
    free(tag->id);
    tag->id = new_id;
    tag->created_at = datetime_now();
    tag->updated_at = datetime_now();
    
    if (tag->category) {
        Vector* tag_ids = hashmap_get(manager->category_tags, tag->category);
        if (!tag_ids) {
            tag_ids = vector_create();
            if (!tag_ids) {
                return TODOZI_OUT_OF_MEMORY;
            }
            TodoziError err = hashmap_put(manager->category_tags, tag->category, tag_ids);
            if (err != TODOZI_SUCCESS) {
                vector_free(tag_ids, NULL);
                return err;
            }
        }
        char* id_copy = string_clone(tag->id);
        if (!id_copy) return TODOZI_OUT_OF_MEMORY;
        vector_push(tag_ids, id_copy);
    }
    
    Tag* cloned_tag = tag_clone(tag);
    if (!cloned_tag) return TODOZI_OUT_OF_MEMORY;
    
    TodoziError err = hashmap_put(manager->tags, tag->id, cloned_tag);
    if (err != TODOZI_SUCCESS) {
        tag_free(cloned_tag);
        return err;
    }
    
    if (out_id) {
        *out_id = string_clone(tag->id);
        if (!*out_id) return TODOZI_OUT_OF_MEMORY;
    }
    
    return TODOZI_SUCCESS;
}

Tag* tag_manager_get_tag(TagManager* manager, const char* tag_id) {
    if (!manager || !tag_id) return NULL;
    return hashmap_get(manager->tags, tag_id);
}

Tag* tag_manager_get_tag_by_name(TagManager* manager, const char* name) {
    if (!manager || !name) return NULL;
    
    Vector* tags = hashmap_values(manager->tags);
    if (!tags) return NULL;
    
    Tag* result = NULL;
    
    for (size_t i = 0; i < vector_size(tags); i++) {
        Tag* tag = (Tag*)vector_get(tags, i);
        if (tag && tag->name && strcmp(tag->name, name) == 0) {
            result = tag;
            break;
        }
    }
    
    vector_free(tags, NULL);
    return result;
}

Vector* tag_manager_get_all_tags(TagManager* manager) {
    if (!manager) return NULL;
    return hashmap_values(manager->tags);
}

TodoziError tag_manager_update_tag(TagManager* manager, const char* tag_id, TagUpdate* updates) {
    if (!manager || !tag_id || !updates) return TODOZI_VALIDATION_ERROR;
    
    Tag* tag = hashmap_get(manager->tags, tag_id);
    if (!tag) return TODOZI_VALIDATION_ERROR;
    
    char* old_category = tag->category ? string_clone(tag->category) : NULL;
    
    if (updates->name) {
        free(tag->name);
        tag->name = string_clone(updates->name);
        if (updates->name && !tag->name) {
            free(old_category);
            return TODOZI_OUT_OF_MEMORY;
        }
    }
    
    if (updates->description) {
        free(tag->description);
        tag->description = string_clone(updates->description);
        if (updates->description && !tag->description) {
            free(old_category);
            return TODOZI_OUT_OF_MEMORY;
        }
    }
    
    if (updates->color) {
        free(tag->color);
        tag->color = string_clone(updates->color);
        if (updates->color && !tag->color) {
            free(old_category);
            return TODOZI_OUT_OF_MEMORY;
        }
    }
    
    if (updates->category) {
        if (old_category) {
            Vector* tag_ids = hashmap_get(manager->category_tags, old_category);
            if (tag_ids) {
                // Remove tag_id from old category
                for (size_t i = 0; i < vector_size(tag_ids); i++) {
                    char* id = (char*)vector_get(tag_ids, i);
                    if (id && strcmp(id, tag_id) == 0) {
                        free(id);
                        // Remove by shifting elements
                        for (size_t j = i; j < vector_size(tag_ids) - 1; j++) {
                            tag_ids->data[j] = tag_ids->data[j + 1];
                        }
                        tag_ids->size--;
                        // Clear the last element pointer to avoid confusion
                        if (tag_ids->size < tag_ids->capacity) {
                            tag_ids->data[tag_ids->size] = NULL;
                        }
                        break;
                    }
                }
            }
        }
        
        free(tag->category);
        tag->category = string_clone(updates->category);
        if (updates->category && !tag->category) {
            free(old_category);
            return TODOZI_OUT_OF_MEMORY;
        }
        
        if (tag->category) {
            Vector* tag_ids = hashmap_get(manager->category_tags, tag->category);
            if (!tag_ids) {
                tag_ids = vector_create();
                if (!tag_ids) {
                    free(old_category);
                    return TODOZI_OUT_OF_MEMORY;
                }
                TodoziError err = hashmap_put(manager->category_tags, tag->category, tag_ids);
                if (err != TODOZI_SUCCESS) {
                    vector_free(tag_ids, NULL);
                    free(old_category);
                    return err;
                }
            }
            char* id_copy = string_clone(tag_id);
            if (!id_copy) {
                free(old_category);
                return TODOZI_OUT_OF_MEMORY;
            }
            vector_push(tag_ids, id_copy);
        }
    }
    
    tag->updated_at = datetime_now();
    free(old_category);
    return TODOZI_SUCCESS;
}

TodoziError tag_manager_delete_tag(TagManager* manager, const char* tag_id) {
    if (!manager || !tag_id) return TODOZI_VALIDATION_ERROR;
    
    Tag* tag = hashmap_remove(manager->tags, tag_id);
    if (!tag) return TODOZI_VALIDATION_ERROR;
    
    if (tag->category) {
        Vector* tag_ids = hashmap_get(manager->category_tags, tag->category);
        if (tag_ids) {
            // Remove tag_id from category
            for (size_t i = 0; i < vector_size(tag_ids); i++) {
                char* id = (char*)vector_get(tag_ids, i);
                if (id && strcmp(id, tag_id) == 0) {
                    free(id);
                    // Remove by shifting elements
                    for (size_t j = i; j < vector_size(tag_ids) - 1; j++) {
                        tag_ids->data[j] = tag_ids->data[j + 1];
                    }
                    tag_ids->size--;
                    // Clear the last element pointer to avoid confusion
                    if (tag_ids->size < tag_ids->capacity) {
                        tag_ids->data[tag_ids->size] = NULL;
                    }
                    break;
                }
            }
        }
    }
    
    hashmap_remove(manager->tag_relationships, tag_id);
    
    Vector* all_relationships = hashmap_values(manager->tag_relationships);
    if (all_relationships) {
        for (size_t i = 0; i < vector_size(all_relationships); i++) {
            Vector* relationships = (Vector*)vector_get(all_relationships, i);
            if (relationships) {
                for (size_t j = 0; j < vector_size(relationships); j++) {
                    char* id = (char*)vector_get(relationships, j);
                    if (id && strcmp(id, tag_id) == 0) {
                        free(id);
                        // Remove by shifting elements
                        for (size_t k = j; k < vector_size(relationships) - 1; k++) {
                            relationships->data[k] = relationships->data[k + 1];
                        }
                        relationships->size--;
                        // Clear the last element pointer to avoid confusion
                        if (relationships->size < relationships->capacity) {
                            relationships->data[relationships->size] = NULL;
                        }
                        break;
                    }
                }
            }
        }
        vector_free(all_relationships, NULL);
    }
    
    tag_free(tag);
    return TODOZI_SUCCESS;
}

TodoziError tag_manager_add_tag_relationship(TagManager* manager, const char* tag_id, const char* related_tag_id) {
    if (!manager || !tag_id || !related_tag_id) return TODOZI_VALIDATION_ERROR;
    
    // Prevent self-relationships
    if (strcmp(tag_id, related_tag_id) == 0) return TODOZI_VALIDATION_ERROR;
    
    if (!hashmap_get(manager->tags, tag_id)) return TODOZI_VALIDATION_ERROR;
    if (!hashmap_get(manager->tags, related_tag_id)) return TODOZI_VALIDATION_ERROR;
    
    Vector* relationships = hashmap_get(manager->tag_relationships, tag_id);
    if (!relationships) {
        relationships = vector_create();
        if (!relationships) return TODOZI_OUT_OF_MEMORY;
        TodoziError err = hashmap_put(manager->tag_relationships, tag_id, relationships);
        if (err != TODOZI_SUCCESS) {
            vector_free(relationships, NULL);
            return err;
        }
    }
    
    // Check for duplicate relationships
    for (size_t i = 0; i < vector_size(relationships); i++) {
        char* existing_id = (char*)vector_get(relationships, i);
        if (existing_id && strcmp(existing_id, related_tag_id) == 0) {
            // Relationship already exists
            return TODOZI_SUCCESS;
        }
    }
    
    char* related_id_copy = string_clone(related_tag_id);
    if (!related_id_copy) return TODOZI_OUT_OF_MEMORY;
    vector_push(relationships, related_id_copy);
    return TODOZI_SUCCESS;
}

Vector* tag_manager_get_related_tags(TagManager* manager, const char* tag_id) {
    if (!manager || !tag_id) return NULL;
    
    Vector* related_ids = hashmap_get(manager->tag_relationships, tag_id);
    if (!related_ids) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* related_tags = vector_create();
    if (!related_tags) return NULL;
    
    for (size_t i = 0; i < vector_size(related_ids); i++) {
        char* related_id = (char*)vector_get(related_ids, i);
        if (related_id) {
            Tag* tag = hashmap_get(manager->tags, related_id);
            if (tag) {
                vector_push(related_tags, tag);
            }
        }
    }
    
    return related_tags;
}

Vector* tag_manager_search_tags(TagManager* manager, const char* query) {
    if (!manager || !query) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    char* query_lower = string_to_lowercase_clone(query);
    if (!query_lower) return NULL;
    
    Vector* tags = hashmap_values(manager->tags);
    if (!tags) {
        free(query_lower);
        return NULL;
    }
    
    Vector* results = vector_create();
    if (!results) {
        free(query_lower);
        vector_free(tags, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < vector_size(tags); i++) {
        Tag* tag = (Tag*)vector_get(tags, i);
        if (tag && tag->name) {
            char* name_lower = string_to_lowercase_clone(tag->name);
            if (!name_lower) {
                free(query_lower);
                vector_free(tags, NULL);
                vector_free(results, NULL);
                return NULL;
            }
            int match = (strstr(name_lower, query_lower) != NULL);
            free(name_lower);
            
            if (match) {
                vector_push(results, tag);
                continue;
            }
            
            if (tag->description) {
                char* desc_lower = string_to_lowercase_clone(tag->description);
                if (!desc_lower) {
                    free(query_lower);
                    vector_free(tags, NULL);
                    vector_free(results, NULL);
                    return NULL;
                }
                match = (strstr(desc_lower, query_lower) != NULL);
                free(desc_lower);
                
                if (match) {
                    vector_push(results, tag);
                }
            }
        }
    }
    
    free(query_lower);
    vector_free(tags, NULL);
    return results;
}

Vector* tag_manager_get_tags_by_category(TagManager* manager, const char* category) {
    if (!manager || !category) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* tag_ids = hashmap_get(manager->category_tags, category);
    if (!tag_ids) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* tags = vector_create();
    if (!tags) return NULL;
    
    for (size_t i = 0; i < vector_size(tag_ids); i++) {
        char* tag_id = (char*)vector_get(tag_ids, i);
        if (tag_id) {
            Tag* tag = hashmap_get(manager->tags, tag_id);
            if (tag) {
                vector_push(tags, tag);
            }
        }
    }
    
    return tags;
}

Vector* tag_manager_get_all_categories(TagManager* manager) {
    if (!manager) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    return hashmap_keys(manager->category_tags);
}

TodoziError tag_manager_increment_tag_usage(TagManager* manager, const char* tag_name) {
    if (!manager || !tag_name) return TODOZI_VALIDATION_ERROR;
    
    Vector* tags = hashmap_values(manager->tags);
    if (!tags) return TODOZI_OUT_OF_MEMORY;
    
    for (size_t i = 0; i < vector_size(tags); i++) {
        Tag* tag = (Tag*)vector_get(tags, i);
        if (tag && tag->name && strcmp(tag->name, tag_name) == 0) {
            tag->usage_count++;
            tag->updated_at = datetime_now();
            break;
        }
    }
    vector_free(tags, NULL);
    
    return TODOZI_SUCCESS;
}

// Comparison functions for sorting
static int compare_tags_by_name_asc(const void* a, const void* b) {
    Tag* tag_a = *(Tag**)a;
    Tag* tag_b = *(Tag**)b;
    if (!tag_a || !tag_a->name) return 1;
    if (!tag_b || !tag_b->name) return -1;
    return strcmp(tag_a->name, tag_b->name);
}

static int compare_tags_by_usage_desc(const void* a, const void* b) {
    Tag* tag_a = *(Tag**)a;
    Tag* tag_b = *(Tag**)b;
    if (!tag_a || !tag_b) return 0;
    if (tag_a->usage_count < tag_b->usage_count) return 1;
    if (tag_a->usage_count > tag_b->usage_count) return -1;
    return 0;
}

static int compare_tags_by_created_desc(const void* a, const void* b) {
    Tag* tag_a = *(Tag**)a;
    Tag* tag_b = *(Tag**)b;
    if (!tag_a || !tag_b) return 0;
    if (tag_a->created_at.timestamp < tag_b->created_at.timestamp) return 1;
    if (tag_a->created_at.timestamp > tag_b->created_at.timestamp) return -1;
    return 0;
}

static int compare_tags_by_updated_desc(const void* a, const void* b) {
    Tag* tag_a = *(Tag**)a;
    Tag* tag_b = *(Tag**)b;
    if (!tag_a || !tag_b) return 0;
    if (tag_a->updated_at.timestamp < tag_b->updated_at.timestamp) return 1;
    if (tag_a->updated_at.timestamp > tag_b->updated_at.timestamp) return -1;
    return 0;
}

// Comparison functions for fuzzy search and suggestions
static int compare_fuzzy_pairs(const void* a, const void* b) {
    void** pair_a = *(void***)a;
    void** pair_b = *(void***)b;
    if (!pair_a || !pair_b) return 0;
    size_t dist_a = (size_t)pair_a[1];
    size_t dist_b = (size_t)pair_b[1];
    if (dist_a < dist_b) return -1;
    if (dist_a > dist_b) return 1;
    return 0;
}

static int compare_suggestion_pairs(const void* a, const void* b) {
    void** pair_a = *(void***)a;
    void** pair_b = *(void***)b;
    if (!pair_a || !pair_b) return 0;
    size_t count_a = (size_t)pair_a[1];
    size_t count_b = (size_t)pair_b[1];
    if (count_a < count_b) return 1;
    if (count_a > count_b) return -1;
    return 0;
}

Vector* tag_manager_get_most_used_tags(TagManager* manager, size_t limit) {
    if (!manager) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* tags = hashmap_values(manager->tags);
    if (!tags) return NULL;
    
    // Sort using qsort for better performance
    qsort(tags->data, vector_size(tags), sizeof(Tag*), compare_tags_by_usage_desc);
    
    Vector* result = vector_create();
    if (!result) {
        vector_free(tags, NULL);
        return NULL;
    }
    
    size_t count = (limit < vector_size(tags)) ? limit : vector_size(tags);
    for (size_t i = 0; i < count; i++) {
        vector_push(result, vector_get(tags, i));
    }
    
    vector_free(tags, NULL);
    return result;
}

Vector* tag_manager_get_recent_tags(TagManager* manager, size_t limit) {
    if (!manager) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* tags = hashmap_values(manager->tags);
    if (!tags) return NULL;
    
    // Sort using qsort for better performance
    qsort(tags->data, vector_size(tags), sizeof(Tag*), compare_tags_by_created_desc);
    
    Vector* result = vector_create();
    if (!result) {
        vector_free(tags, NULL);
        return NULL;
    }
    
    size_t count = (limit < vector_size(tags)) ? limit : vector_size(tags);
    for (size_t i = 0; i < count; i++) {
        vector_push(result, vector_get(tags, i));
    }
    
    vector_free(tags, NULL);
    return result;
}

TagStatistics* tag_manager_get_tag_statistics(TagManager* manager) {
    if (!manager) return NULL;
    
    TagStatistics* stats = tag_statistics_create();
    if (!stats) return NULL;
    
    stats->total_tags = hashmap_size(manager->tags);
    stats->total_categories = hashmap_size(manager->category_tags);
    
    Vector* relationships_values = hashmap_values(manager->tag_relationships);
    if (relationships_values) {
        for (size_t i = 0; i < vector_size(relationships_values); i++) {
            Vector* rels = (Vector*)vector_get(relationships_values, i);
            if (rels) {
                stats->total_relationships += vector_size(rels);
            }
        }
        vector_free(relationships_values, NULL);
    }
    
    if (stats->total_tags > 0) {
        Vector* tags = hashmap_values(manager->tags);
        if (tags) {
            unsigned int total_usage = 0;
            for (size_t i = 0; i < vector_size(tags); i++) {
                Tag* tag = (Tag*)vector_get(tags, i);
                if (tag) {
                    total_usage += tag->usage_count;
                }
            }
            stats->average_usage = (double)total_usage / (double)stats->total_tags;
            vector_free(tags, NULL);
        }
    }
    
    return stats;
}

TodoziError tag_manager_bulk_create_tags(TagManager* manager, Vector* tag_names, const char* category, Vector** out_ids) {
    if (!manager || !tag_names) return TODOZI_VALIDATION_ERROR;
    
    Vector* created_ids = vector_create();
    if (!created_ids) return TODOZI_OUT_OF_MEMORY;
    
    for (size_t i = 0; i < vector_size(tag_names); i++) {
        char* name = (char*)vector_get(tag_names, i);
        if (name) {
            Tag* tag = tag_create();
            if (!tag) {
                vector_free(created_ids, free);
                return TODOZI_OUT_OF_MEMORY;
            }
            tag->name = string_clone(name);
            if (!tag->name) {
                tag_free(tag);
                vector_free(created_ids, free);
                return TODOZI_OUT_OF_MEMORY;
            }
            if (category) {
                tag->category = string_clone(category);
                if (!tag->category) {
                    tag_free(tag);
                    vector_free(created_ids, free);
                    return TODOZI_OUT_OF_MEMORY;
                }
            }
            
            char* id = NULL;
            TodoziError err = tag_manager_create_tag(manager, tag, &id);
            if (err != TODOZI_SUCCESS) {
                tag_free(tag);
                vector_free(created_ids, free);
                return err;
            }
            
            vector_push(created_ids, id);
            tag_free(tag);
        }
    }
    
    if (out_ids) {
        *out_ids = created_ids;
    } else {
        vector_free(created_ids, free);
    }
    
    return TODOZI_SUCCESS;
}

TodoziError tag_manager_merge_tags(TagManager* manager, const char* primary_tag_id, Vector* duplicate_tag_ids) {
    if (!manager || !primary_tag_id || !duplicate_tag_ids) return TODOZI_VALIDATION_ERROR;
    
    Tag* primary_tag = hashmap_get(manager->tags, primary_tag_id);
    if (!primary_tag) return TODOZI_VALIDATION_ERROR;
    
    for (size_t i = 0; i < vector_size(duplicate_tag_ids); i++) {
        char* duplicate_id = (char*)vector_get(duplicate_tag_ids, i);
        if (duplicate_id) {
            Tag* duplicate_tag = hashmap_remove(manager->tags, duplicate_id);
            if (duplicate_tag) {
                primary_tag->usage_count += duplicate_tag->usage_count;
                primary_tag->updated_at = datetime_now();
                
                Vector* relationships = hashmap_remove(manager->tag_relationships, duplicate_id);
                if (relationships) {
                    Vector* primary_relationships = hashmap_get(manager->tag_relationships, primary_tag_id);
                    if (!primary_relationships) {
                        primary_relationships = vector_create();
                        if (!primary_relationships) {
                            vector_free(relationships, free);
                            tag_free(duplicate_tag);
                            return TODOZI_OUT_OF_MEMORY;
                        }
                        TodoziError err = hashmap_put(manager->tag_relationships, primary_tag_id, primary_relationships);
                        if (err != TODOZI_SUCCESS) {
                            vector_free(primary_relationships, NULL);
                            vector_free(relationships, free);
                            tag_free(duplicate_tag);
                            return err;
                        }
                    }
                    
                    for (size_t j = 0; j < vector_size(relationships); j++) {
                        char* rel_id = (char*)vector_get(relationships, j);
                        if (rel_id) {
                            char* rel_id_copy = string_clone(rel_id);
                            if (!rel_id_copy) {
                                vector_free(relationships, free);
                                tag_free(duplicate_tag);
                                return TODOZI_OUT_OF_MEMORY;
                            }
                            vector_push(primary_relationships, rel_id_copy);
                        }
                    }
                    vector_free(relationships, free);
                }
                tag_free(duplicate_tag);
            }
        }
    }
    
    return TODOZI_SUCCESS;
}

// TagSearchEngine functions
TagSearchEngine* tag_search_engine_create(TagManager* tag_manager) {
    if (!tag_manager) return NULL;
    
    TagSearchEngine* engine = malloc(sizeof(TagSearchEngine));
    if (!engine) return NULL;
    engine->tag_manager = tag_manager;
    return engine;
}

void tag_search_engine_free(TagSearchEngine* engine) {
    free(engine);
}

Vector* tag_search_engine_advanced_search(TagSearchEngine* engine, TagSearchQuery* query) {
    if (!engine || !query) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* results = hashmap_values(engine->tag_manager->tags);
    if (!results) return NULL;
    
    if (query->name_contains) {
        char* name_lower = string_to_lowercase_clone(query->name_contains);
        if (!name_lower) {
            vector_free(results, NULL);
            return NULL;
        }
        Vector* filtered = vector_create();
        if (!filtered) {
            free(name_lower);
            vector_free(results, NULL);
            return NULL;
        }
        
        for (size_t i = 0; i < vector_size(results); i++) {
            Tag* tag = (Tag*)vector_get(results, i);
            if (tag && tag->name) {
                char* tag_name_lower = string_to_lowercase_clone(tag->name);
                if (!tag_name_lower) {
                    free(name_lower);
                    vector_free(results, NULL);
                    vector_free(filtered, NULL);
                    return NULL;
                }
                if (strstr(tag_name_lower, name_lower)) {
                    vector_push(filtered, tag);
                }
                free(tag_name_lower);
            }
        }
        
        vector_free(results, NULL);
        results = filtered;
        free(name_lower);
    }
    
    if (query->description_contains) {
        char* desc_lower = string_to_lowercase_clone(query->description_contains);
        if (!desc_lower) {
            vector_free(results, NULL);
            return NULL;
        }
        Vector* filtered = vector_create();
        if (!filtered) {
            free(desc_lower);
            vector_free(results, NULL);
            return NULL;
        }
        
        for (size_t i = 0; i < vector_size(results); i++) {
            Tag* tag = (Tag*)vector_get(results, i);
            if (tag && tag->description) {
                char* tag_desc_lower = string_to_lowercase_clone(tag->description);
                if (!tag_desc_lower) {
                    free(desc_lower);
                    vector_free(results, NULL);
                    vector_free(filtered, NULL);
                    return NULL;
                }
                if (strstr(tag_desc_lower, desc_lower)) {
                    vector_push(filtered, tag);
                }
                free(tag_desc_lower);
            }
        }
        
        vector_free(results, NULL);
        results = filtered;
        free(desc_lower);
    }
    
    if (query->category) {
        Vector* filtered = vector_create();
        if (!filtered) {
            vector_free(results, NULL);
            return NULL;
        }
        
        for (size_t i = 0; i < vector_size(results); i++) {
            Tag* tag = (Tag*)vector_get(results, i);
            if (tag && tag->category && strcmp(tag->category, query->category) == 0) {
                vector_push(filtered, tag);
            }
        }
        
        vector_free(results, NULL);
        results = filtered;
    }
    
    if (query->min_usage) {
        Vector* filtered = vector_create();
        if (!filtered) {
            vector_free(results, NULL);
            return NULL;
        }
        
        for (size_t i = 0; i < vector_size(results); i++) {
            Tag* tag = (Tag*)vector_get(results, i);
            if (tag && tag->usage_count >= *query->min_usage) {
                vector_push(filtered, tag);
            }
        }
        
        vector_free(results, NULL);
        results = filtered;
    }
    
    if (query->max_usage) {
        Vector* filtered = vector_create();
        if (!filtered) {
            vector_free(results, NULL);
            return NULL;
        }
        
        for (size_t i = 0; i < vector_size(results); i++) {
            Tag* tag = (Tag*)vector_get(results, i);
            if (tag && tag->usage_count <= *query->max_usage) {
                vector_push(filtered, tag);
            }
        }
        
        vector_free(results, NULL);
        results = filtered;
    }
    
    if (query->color) {
        Vector* filtered = vector_create();
        if (!filtered) {
            vector_free(results, NULL);
            return NULL;
        }
        
        for (size_t i = 0; i < vector_size(results); i++) {
            Tag* tag = (Tag*)vector_get(results, i);
            if (tag && tag->color && strcmp(tag->color, query->color) == 0) {
                vector_push(filtered, tag);
            }
        }
        
        vector_free(results, NULL);
        results = filtered;
    }
    
    // Sort results
    if (vector_size(results) > 0) {
        switch (query->sort_by) {
            case TAG_SORT_BY_NAME:
                qsort(results->data, vector_size(results), sizeof(Tag*), compare_tags_by_name_asc);
                break;
                
            case TAG_SORT_BY_USAGE:
                qsort(results->data, vector_size(results), sizeof(Tag*), compare_tags_by_usage_desc);
                break;
                
            case TAG_SORT_BY_CREATED:
                qsort(results->data, vector_size(results), sizeof(Tag*), compare_tags_by_created_desc);
                break;
                
            case TAG_SORT_BY_UPDATED:
                qsort(results->data, vector_size(results), sizeof(Tag*), compare_tags_by_updated_desc);
                break;
        }
    }
    
    if (query->limit && *query->limit < vector_size(results)) {
        Vector* limited = vector_create();
        if (!limited) {
            vector_free(results, NULL);
            return NULL;
        }
        for (size_t i = 0; i < *query->limit; i++) {
            vector_push(limited, vector_get(results, i));
        }
        vector_free(results, NULL);
        results = limited;
    }
    
    return results;
}

size_t levenshtein_distance(const char* s1, const char* s2) {
    size_t len1 = strlen(s1);
    size_t len2 = strlen(s2);
    
    // Use two-row optimization to reduce memory usage
    size_t* prev_row = calloc(len2 + 1, sizeof(size_t));
    if (!prev_row) return (size_t)-1;
    size_t* curr_row = calloc(len2 + 1, sizeof(size_t));
    if (!curr_row) {
        free(prev_row);
        return (size_t)-1;
    }
    
    for (size_t j = 0; j <= len2; j++) {
        prev_row[j] = j;
    }
    
    for (size_t i = 1; i <= len1; i++) {
        curr_row[0] = i;
        for (size_t j = 1; j <= len2; j++) {
            size_t cost = (s1[i - 1] == s2[j - 1]) ? 0 : 1;
            size_t del = prev_row[j] + 1;
            size_t ins = curr_row[j - 1] + 1;
            size_t sub = prev_row[j - 1] + cost;
            
            size_t min = del < ins ? del : ins;
            min = min < sub ? min : sub;
            curr_row[j] = min;
        }
        size_t* temp = prev_row;
        prev_row = curr_row;
        curr_row = temp;
    }
    
    size_t result = prev_row[len2];
    free(prev_row);
    free(curr_row);
    return result;
}

Vector* tag_search_engine_fuzzy_search(TagSearchEngine* engine, const char* query, size_t max_distance) {
    if (!engine || !query) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    Vector* results = vector_create();
    if (!results) return NULL;
    
    char* query_lower = string_to_lowercase_clone(query);
    if (!query_lower) {
        vector_free(results, NULL);
        return NULL;
    }
    
    Vector* tags = hashmap_values(engine->tag_manager->tags);
    if (!tags) {
        free(query_lower);
        vector_free(results, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < vector_size(tags); i++) {
        Tag* tag = (Tag*)vector_get(tags, i);
        if (tag && tag->name) {
            char* name_lower = string_to_lowercase_clone(tag->name);
            if (!name_lower) {
                free(query_lower);
                vector_free(tags, NULL);
                vector_free(results, NULL);
                return NULL;
            }
            size_t distance = levenshtein_distance(query_lower, name_lower);
            
            if (distance <= max_distance) {
                // Store tag and distance as a pair
                void** pair = malloc(sizeof(void*) * 2);
                if (!pair) {
                    free(name_lower);
                    free(query_lower);
                    vector_free(tags, NULL);
                    vector_free(results, NULL);
                    return NULL;
                }
                pair[0] = tag;
                pair[1] = (void*)distance;
                vector_push(results, pair);
            }
            
            free(name_lower);
        }
    }
    
    vector_free(tags, NULL);
    free(query_lower);
    
    // Sort by distance using qsort for better performance
    if (vector_size(results) > 1) {
        qsort(results->data, vector_size(results), sizeof(void*), compare_fuzzy_pairs);
    }
    
    return results;
}

void vector_free_fuzzy_results(Vector* results) {
    if (!results) return;
    for (size_t i = 0; i < vector_size(results); i++) {
        void** pair = (void**)vector_get(results, i);
        free(pair);
    }
    vector_free(results, NULL);
}

Vector* tag_search_engine_get_suggestions(TagSearchEngine* engine, Vector* current_tags, size_t limit) {
    if (!engine || !current_tags) {
        Vector* empty = vector_create();
        return empty ? empty : NULL;
    }
    
    HashMap* suggestions = hashmap_create();
    if (!suggestions) return NULL;
    
    for (size_t i = 0; i < vector_size(current_tags); i++) {
        char* tag_name = (char*)vector_get(current_tags, i);
        if (tag_name) {
            Tag* current_tag = tag_manager_get_tag_by_name(engine->tag_manager, tag_name);
            if (current_tag) {
                Vector* related_tags = tag_manager_get_related_tags(engine->tag_manager, current_tag->id);
                if (related_tags) {
                    for (size_t j = 0; j < vector_size(related_tags); j++) {
                        Tag* related_tag = (Tag*)vector_get(related_tags, j);
                        if (related_tag) {
                            size_t* count = hashmap_get(suggestions, related_tag->name);
                            if (count) {
                                (*count)++;
                            } else {
                                count = malloc(sizeof(size_t));
                                if (!count) {
                                    vector_free(related_tags, NULL);
                                    hashmap_free(suggestions, free, free);
                                    return NULL;
                                }
                                *count = 1;
                                char* name_copy = string_clone(related_tag->name);
                                if (!name_copy) {
                                    free(count);
                                    vector_free(related_tags, NULL);
                                    hashmap_free(suggestions, free, free);
                                    return NULL;
                                }
                                hashmap_put(suggestions, name_copy, count);
                            }
                        }
                    }
                    vector_free(related_tags, NULL);
                }
            }
        }
    }
    
    Vector* suggestion_list = vector_create();
    if (!suggestion_list) {
        hashmap_free(suggestions, free, free);
        return NULL;
    }
    
    Vector* keys = hashmap_keys(suggestions);
    if (!keys) {
        hashmap_free(suggestions, free, free);
        vector_free(suggestion_list, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < vector_size(keys); i++) {
        char* key = (char*)vector_get(keys, i);
        size_t* count = hashmap_get(suggestions, key);
        if (key && count) {
            void** pair = malloc(sizeof(void*) * 2);
            if (!pair) {
                vector_free(keys, free);
                hashmap_free(suggestions, free, free);
                vector_free(suggestion_list, NULL);
                return NULL;
            }
            pair[0] = strdup(key);
            if (!pair[0]) {
                free(pair);
                vector_free(keys, free);
                hashmap_free(suggestions, free, free);
                vector_free(suggestion_list, NULL);
                return NULL;
            }
            pair[1] = (void*)*count;
            vector_push(suggestion_list, pair);
        }
    }
    vector_free(keys, free);
    
    hashmap_free(suggestions, free, free);
    
    // Sort by count (descending) using qsort for better performance
    if (vector_size(suggestion_list) > 1) {
        qsort(suggestion_list->data, vector_size(suggestion_list), sizeof(void*), compare_suggestion_pairs);
    }
    
    Vector* result = vector_create();
    if (!result) {
        // Free all pairs
        for (size_t i = 0; i < vector_size(suggestion_list); i++) {
            void** pair = (void**)vector_get(suggestion_list, i);
            free(pair[0]);
            free(pair);
        }
        vector_free(suggestion_list, NULL);
        return NULL;
    }
    
    size_t count = (limit < vector_size(suggestion_list)) ? limit : vector_size(suggestion_list);
    for (size_t i = 0; i < count; i++) {
        void** pair = (void**)vector_get(suggestion_list, i);
        vector_push(result, pair[0]); // Just the name
    }
    
    // Free remaining pairs
    for (size_t i = count; i < vector_size(suggestion_list); i++) {
        void** pair = (void**)vector_get(suggestion_list, i);
        free(pair[0]);
        free(pair);
    }
    
    // Free the pairs we're keeping in result
    for (size_t i = 0; i < count; i++) {
        void** pair = (void**)vector_get(suggestion_list, i);
        free(pair);
    }
    
    vector_free(suggestion_list, NULL);
    return result;
}

// Test functions
void test_tag_manager_creation() {
    TagManager* manager = tag_manager_create();
    if (!manager) {
        printf("Failed to create TagManager\n");
        return;
    }
    printf("TagManager created\n");
    printf("Tags count: %zu\n", hashmap_size(manager->tags));
    printf("Category tags count: %zu\n", hashmap_size(manager->category_tags));
    tag_manager_free(manager);
}

void test_tag_update_builder() {
    TagUpdate* update = tag_update_new();
    if (!update) {
        printf("Failed to create TagUpdate\n");
        return;
    }
    tag_update_name(update, "New Name");
    tag_update_description(update, "New Description");
    tag_update_color(update, "#FF0000");
    
    printf("TagUpdate created\n");
    printf("Name: %s\n", update->name ? update->name : "NULL");
    printf("Description: %s\n", update->description ? update->description : "NULL");
    printf("Color: %s\n", update->color ? update->color : "NULL");
    
    tag_update_free(update);
}

void test_tag_statistics() {
    TagStatistics* stats = tag_statistics_create();
    if (!stats) {
        printf("Failed to create TagStatistics\n");
        return;
    }
    stats->total_tags = 10;
    stats->total_categories = 3;
    stats->total_relationships = 15;
    stats->average_usage = 5.5;
    
    printf("TagStatistics test\n");
    printf("Relationships per tag: %f\n", tag_statistics_relationships_per_tag(stats));
    
    stats->total_tags = 0;
    printf("Empty stats relationships per tag: %f\n", tag_statistics_relationships_per_tag(stats));
    
    tag_statistics_free(stats);
}

void test_tag_search_query() {
    TagSearchQuery* query = tag_search_query_create();
    if (!query) {
        printf("Failed to create TagSearchQuery\n");
        return;
    }
    query->name_contains = string_clone("test");
    query->category = string_clone("development");
    query->min_usage = malloc(sizeof(unsigned int));
    if (query->min_usage) *query->min_usage = 5;
    query->sort_by = TAG_SORT_BY_USAGE;
    query->limit = malloc(sizeof(size_t));
    if (query->limit) *query->limit = 10;
    
    printf("TagSearchQuery test\n");
    printf("Name contains: %s\n", query->name_contains ? query->name_contains : "NULL");
    printf("Category: %s\n", query->category ? query->category : "NULL");
    printf("Min usage: %u\n", query->min_usage ? *query->min_usage : 0);
    printf("Limit: %zu\n", query->limit ? *query->limit : 0);
    
    tag_search_query_free(query);
}

void test_levenshtein_distance() {
    printf("Levenshtein distance tests:\n");
    printf("kitten -> kitten: %zu\n", levenshtein_distance("kitten", "kitten"));
    printf("kitten -> kittens: %zu\n", levenshtein_distance("kitten", "kittens"));
    printf("kitten -> sitting: %zu\n", levenshtein_distance("kitten", "sitting"));
    printf("'' -> abc: %zu\n", levenshtein_distance("", "abc"));
    printf("abc -> '': %zu\n", levenshtein_distance("abc", ""));
}

int main() {
    printf("Running tests...\n\n");
    
    test_tag_manager_creation();
    printf("\n");
    
    test_tag_update_builder();
    printf("\n");
    
    test_tag_statistics();
    printf("\n");
    
    test_tag_search_query();
    printf("\n");
    
    test_levenshtein_distance();
    printf("\n");
    
    printf("All tests completed.\n");
    return 0;
}