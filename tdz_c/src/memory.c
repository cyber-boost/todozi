#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>
#include <ctype.h>

// Forward declarations
typedef struct Memory Memory;
typedef struct MemoryManager MemoryManager;
typedef struct MemoryUpdate MemoryUpdate;
typedef struct MemoryStatistics MemoryStatistics;

// Enums
typedef enum {
    MEMORY_IMPORTANCE_LOW,
    MEMORY_IMPORTANCE_MEDIUM,
    MEMORY_IMPORTANCE_HIGH,
    MEMORY_IMPORTANCE_CRITICAL
} MemoryImportance;

typedef enum {
    MEMORY_TERM_SHORT,
    MEMORY_TERM_LONG
} MemoryTerm;

typedef enum {
    MEMORY_TYPE_STANDARD,
    MEMORY_TYPE_SECRET,
    MEMORY_TYPE_HUMAN,
    MEMORY_TYPE_EMOTIONAL,
    MEMORY_TYPE_SHORT,
    MEMORY_TYPE_LONG
} MemoryType;

typedef enum {
    ITEM_STATUS_ACTIVE,
    ITEM_STATUS_COMPLETED,
    ITEM_STATUS_ARCHIVED
} ItemStatus;

// HashMap implementation (simplified)
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

// Vector implementation (simplified)
typedef struct {
    void** data;
    size_t size;
    size_t capacity;
} Vector;

// Memory structure
struct Memory {
    char* id;
    char* user_id;
    char* project_id;  // NULL if not set
    ItemStatus status;
    char* moment;
    char* meaning;
    char* reason;
    MemoryImportance importance;
    MemoryTerm term;
    MemoryType memory_type;
    char* emotion;      // For emotional memories
    Vector* tags;       // Vector of char*
    time_t created_at;
    time_t updated_at;
};

// MemoryUpdate structure
struct MemoryUpdate {
    char* moment;       // NULL if not set
    char* meaning;      // NULL if not set
    char* reason;       // NULL if not set
    MemoryImportance* importance;  // NULL if not set
    MemoryTerm* term;   // NULL if not set
    Vector* tags;       // NULL if not set
};

// MemoryStatistics structure
struct MemoryStatistics {
    size_t total_memories;
    size_t short_term_memories;
    size_t long_term_memories;
    size_t critical_memories;
    size_t unique_tags;
    size_t secret_memories;
    size_t human_memories;
    size_t emotional_memories;
    size_t standard_memories;
};

// Error handling
typedef enum {
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_OTHER
} TodoziErrorType;

typedef struct {
    TodoziErrorType type;
    char* message;
} TodoziError;

// Error helper function
void todozi_error_free(TodoziError* error) {
    if (!error) return;
    free(error->message);
    free(error);
}

// HashMap functions
HashMap* hashmap_create(size_t capacity) {
    HashMap* map = malloc(sizeof(HashMap));
    if (!map) return NULL;
    map->buckets = calloc(capacity, sizeof(HashMapEntry*));
    if (!map->buckets) {
        free(map);
        return NULL;
    }
    map->size = 0;
    map->capacity = capacity;
    return map;
}

void hashmap_destroy(HashMap* map, void (*free_key)(void*), void (*free_value)(void*)) {
    if (!map) return;
    for (size_t i = 0; i < map->capacity; i++) {
        HashMapEntry* entry = map->buckets[i];
        while (entry) {
            HashMapEntry* next = entry->next;
            if (free_key) free_key(entry->key);
            else free(entry->key);
            if (free_value) free_value(entry->value);
            free(entry);
            entry = next;
        }
    }
    free(map->buckets);
    free(map);
}

// Simple hash function
unsigned int hash(const char* str, size_t capacity) {
    unsigned int hash = 5381;
    int c;
    while ((c = *str++))
        hash = ((hash << 5) + hash) + c;
    return hash % capacity;
}

void hashmap_put(HashMap* map, const char* key, void* value) {
    unsigned int index = hash(key, map->capacity);
    HashMapEntry* entry = map->buckets[index];
    
    // Check if key already exists
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            // Free old value before overwriting
            if (entry->value) {
                free(entry->value);
            }
            entry->value = value;
            return;
        }
        entry = entry->next;
    }
    
    // Create new entry
    HashMapEntry* new_entry = malloc(sizeof(HashMapEntry));
    if (!new_entry) return;
    new_entry->key = strdup(key);
    new_entry->value = value;
    new_entry->next = map->buckets[index];
    map->buckets[index] = new_entry;
    map->size++;
}

void* hashmap_get(HashMap* map, const char* key) {
    unsigned int index = hash(key, map->capacity);
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
    unsigned int index = hash(key, map->capacity);
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

size_t hashmap_size(HashMap* map) {
    return map ? map->size : 0;
}

// Vector functions
Vector* vector_create() {
    Vector* vec = malloc(sizeof(Vector));
    if (!vec) return NULL;
    vec->data = malloc(sizeof(void*) * 10);
    if (!vec->data) {
        free(vec);
        return NULL;
    }
    vec->size = 0;
    vec->capacity = 10;
    return vec;
}

void vector_destroy(Vector* vec, void (*free_element)(void*)) {
    if (!vec) return;
    if (free_element) {
        for (size_t i = 0; i < vec->size; i++) {
            free_element(vec->data[i]);
        }
    }
    free(vec->data);
    free(vec);
}

void vector_push(Vector* vec, void* element) {
    if (!vec) return;
    if (vec->size >= vec->capacity) {
        vec->capacity *= 2;
        vec->data = realloc(vec->data, sizeof(void*) * vec->capacity);
    }
    vec->data[vec->size++] = element;
}

void* vector_get(Vector* vec, size_t index) {
    if (!vec || index >= vec->size) return NULL;
    return vec->data[index];
}

size_t vector_size(Vector* vec) {
    return vec ? vec->size : 0;
}

// String vector specific functions
Vector* string_vector_create() {
    return vector_create();
}

void string_vector_destroy(Vector* vec) {
    vector_destroy(vec, free);
}

void string_vector_push(Vector* vec, const char* str) {
    if (str) vector_push(vec, strdup(str));
}

char* string_vector_get(Vector* vec, size_t index) {
    return (char*)vector_get(vec, index);
}

// Memory functions
Memory* memory_create() {
    Memory* memory = malloc(sizeof(Memory));
    if (!memory) return NULL;
    memory->id = NULL;
    memory->user_id = NULL;
    memory->project_id = NULL;
    memory->moment = NULL;
    memory->meaning = NULL;
    memory->reason = NULL;
    memory->emotion = NULL;
    memory->tags = string_vector_create();
    if (!memory->tags) {
        free(memory);
        return NULL;
    }
    return memory;
}

void memory_destroy(Memory* memory) {
    if (!memory) return;
    if (memory->id) free(memory->id);
    if (memory->user_id) free(memory->user_id);
    if (memory->project_id) free(memory->project_id);
    if (memory->moment) free(memory->moment);
    if (memory->meaning) free(memory->meaning);
    if (memory->reason) free(memory->reason);
    if (memory->emotion) free(memory->emotion);
    if (memory->tags) string_vector_destroy(memory->tags);
    free(memory);
}

// MemoryUpdate functions
MemoryUpdate* memory_update_create() {
    MemoryUpdate* update = malloc(sizeof(MemoryUpdate));
    if (!update) return NULL;
    update->moment = NULL;
    update->meaning = NULL;
    update->reason = NULL;
    update->importance = NULL;
    update->term = NULL;
    update->tags = NULL;
    return update;
}

void memory_update_destroy(MemoryUpdate* update) {
    if (!update) return;
    if (update->moment) free(update->moment);
    if (update->meaning) free(update->meaning);
    if (update->reason) free(update->reason);
    if (update->importance) free(update->importance);
    if (update->term) free(update->term);
    if (update->tags) string_vector_destroy(update->tags);
    free(update);
}

MemoryUpdate* memory_update_moment(MemoryUpdate* update, const char* moment) {
    if (!update || !moment) return update;
    if (update->moment) free(update->moment);
    update->moment = strdup(moment);
    return update;
}

MemoryUpdate* memory_update_meaning(MemoryUpdate* update, const char* meaning) {
    if (!update || !meaning) return update;
    if (update->meaning) free(update->meaning);
    update->meaning = strdup(meaning);
    return update;
}

MemoryUpdate* memory_update_reason(MemoryUpdate* update, const char* reason) {
    if (!update || !reason) return update;
    if (update->reason) free(update->reason);
    update->reason = strdup(reason);
    return update;
}

MemoryUpdate* memory_update_importance(MemoryUpdate* update, MemoryImportance importance) {
    if (!update) return update;
    if (!update->importance) update->importance = malloc(sizeof(MemoryImportance));
    if (update->importance) *update->importance = importance;
    return update;
}

MemoryUpdate* memory_update_term(MemoryUpdate* update, MemoryTerm term) {
    if (!update) return update;
    if (!update->term) update->term = malloc(sizeof(MemoryTerm));
    if (update->term) *update->term = term;
    return update;
}

MemoryUpdate* memory_update_tags(MemoryUpdate* update, Vector* tags) {
    if (!update) return update;
    if (update->tags) string_vector_destroy(update->tags);
    update->tags = tags;
    return update;
}

// MemoryStatistics functions
MemoryStatistics* memory_statistics_create() {
    MemoryStatistics* stats = malloc(sizeof(MemoryStatistics));
    if (!stats) return NULL;
    stats->total_memories = 0;
    stats->short_term_memories = 0;
    stats->long_term_memories = 0;
    stats->critical_memories = 0;
    stats->unique_tags = 0;
    stats->secret_memories = 0;
    stats->human_memories = 0;
    stats->emotional_memories = 0;
    stats->standard_memories = 0;
    return stats;
}

void memory_statistics_destroy(MemoryStatistics* stats) {
    free(stats);
}

double memory_statistics_short_term_percentage(MemoryStatistics* stats) {
    if (!stats || stats->total_memories == 0) return 0.0;
    return ((double)stats->short_term_memories / (double)stats->total_memories) * 100.0;
}

double memory_statistics_long_term_percentage(MemoryStatistics* stats) {
    if (!stats || stats->total_memories == 0) return 0.0;
    return ((double)stats->long_term_memories / (double)stats->total_memories) * 100.0;
}

double memory_statistics_critical_percentage(MemoryStatistics* stats) {
    if (!stats || stats->total_memories == 0) return 0.0;
    return ((double)stats->critical_memories / (double)stats->total_memories) * 100.0;
}

// MemoryManager structure
struct MemoryManager {
    HashMap* memories;      // HashMap<String, Memory*>
    HashMap* memory_tags;   // HashMap<String, Vector* of char*>
};

// MemoryManager functions
MemoryManager* memory_manager_create() {
    MemoryManager* manager = malloc(sizeof(MemoryManager));
    if (!manager) return NULL;
    manager->memories = hashmap_create(100);
    if (!manager->memories) {
        free(manager);
        return NULL;
    }
    manager->memory_tags = hashmap_create(100);
    if (!manager->memory_tags) {
        hashmap_destroy(manager->memories, free, (void(*)(void*))memory_destroy);
        free(manager);
        return NULL;
    }
    return manager;
}

void memory_manager_destroy(MemoryManager* manager) {
    if (!manager) return;
    // Free memories
    hashmap_destroy(manager->memories, free, (void(*)(void*))memory_destroy);
    // Free memory_tags
    hashmap_destroy(manager->memory_tags, free, (void(*)(void*))string_vector_destroy);
    free(manager);
}

TodoziError* memory_manager_create_memory(MemoryManager* manager, Memory* memory) {
    if (!manager || !memory) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid parameters");
        return error;
    }
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    char* uuid_str = malloc(37);
    if (!uuid_str) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_OTHER;
        error->message = strdup("Memory allocation failed");
        return error;
    }
    uuid_unparse(uuid, uuid_str);
    
    memory->id = uuid_str;
    memory->created_at = time(NULL);
    memory->updated_at = time(NULL);
    
    // Store in memories map
    Memory* memory_copy = memory; // We're transferring ownership
    hashmap_put(manager->memories, memory->id, memory_copy);
    
    // Store tags - create a copy for the tags map
    Vector* tags_copy = vector_create();
    if (tags_copy) {
        for (size_t i = 0; i < vector_size(memory->tags); i++) {
            char* tag = string_vector_get(memory->tags, i);
            if (tag) {
                string_vector_push(tags_copy, tag);
            }
        }
        hashmap_put(manager->memory_tags, memory->id, tags_copy);
    }
    
    return NULL; // Success
}

Memory* memory_manager_get_memory(MemoryManager* manager, const char* memory_id) {
    if (!manager || !memory_id) return NULL;
    return (Memory*)hashmap_get(manager->memories, memory_id);
}

Vector* memory_manager_get_all_memories(MemoryManager* manager) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            vector_push(result, entry->value);
            entry = entry->next;
        }
    }
    return result;
}

TodoziError* memory_manager_update_memory(MemoryManager* manager, const char* memory_id, MemoryUpdate* updates) {
    if (!manager || !memory_id || !updates) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid parameters");
        return error;
    }
    
    Memory* memory = (Memory*)hashmap_get(manager->memories, memory_id);
    if (!memory) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        char msg[256];
        snprintf(msg, sizeof(msg), "Memory %s not found", memory_id);
        error->message = strdup(msg);
        return error;
    }
    
    if (updates->moment) {
        if (memory->moment) free(memory->moment);
        memory->moment = strdup(updates->moment);
    }
    
    if (updates->meaning) {
        if (memory->meaning) free(memory->meaning);
        memory->meaning = strdup(updates->meaning);
    }
    
    if (updates->reason) {
        if (memory->reason) free(memory->reason);
        memory->reason = strdup(updates->reason);
    }
    
    if (updates->importance) {
        memory->importance = *updates->importance;
    }
    
    if (updates->term) {
        memory->term = *updates->term;
    }
    
    memory->updated_at = time(NULL);
    
    if (updates->tags) {
        // Replace tags in memory
        string_vector_destroy(memory->tags);
        memory->tags = updates->tags;
        
        // Update tags in memory_tags map
        Vector* tags_copy = vector_create();
        if (tags_copy) {
            for (size_t i = 0; i < vector_size(updates->tags); i++) {
                char* tag = string_vector_get(updates->tags, i);
                if (tag) {
                    string_vector_push(tags_copy, tag);
                }
            }
            hashmap_put(manager->memory_tags, memory_id, tags_copy);
        }
    }
    
    return NULL; // Success
}

TodoziError* memory_manager_delete_memory(MemoryManager* manager, const char* memory_id) {
    if (!manager || !memory_id) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid parameters");
        return error;
    }
    
    Memory* removed = (Memory*)hashmap_remove(manager->memories, memory_id);
    if (!removed) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        char msg[256];
        snprintf(msg, sizeof(msg), "Memory %s not found", memory_id);
        error->message = strdup(msg);
        return error;
    }
    
    // Remove from memory_tags as well
    Vector* tags = (Vector*)hashmap_remove(manager->memory_tags, memory_id);
    if (tags) {
        string_vector_destroy(tags);
    }
    
    memory_destroy(removed);
    return NULL; // Success
}

Vector* memory_manager_search_memories(MemoryManager* manager, const char* query) {
    if (!manager || !query) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    char* query_lower = strdup(query);
    if (!query_lower) {
        vector_destroy(result, NULL);
        return NULL;
    }
    for (char* p = query_lower; *p; p++) *p = tolower(*p);
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            int match = 0;
            
            // Check moment
            if (memory->moment) {
                char* moment_lower = strdup(memory->moment);
                if (moment_lower) {
                    for (char* p = moment_lower; *p; p++) *p = tolower(*p);
                    if (strstr(moment_lower, query_lower)) match = 1;
                    free(moment_lower);
                }
            }
            
            if (!match && memory->meaning) {
                // Check meaning
                char* meaning_lower = strdup(memory->meaning);
                if (meaning_lower) {
                    for (char* p = meaning_lower; *p; p++) *p = tolower(*p);
                    if (strstr(meaning_lower, query_lower)) match = 1;
                    free(meaning_lower);
                }
            }
            
            if (!match && memory->reason) {
                // Check reason
                char* reason_lower = strdup(memory->reason);
                if (reason_lower) {
                    for (char* p = reason_lower; *p; p++) *p = tolower(*p);
                    if (strstr(reason_lower, query_lower)) match = 1;
                    free(reason_lower);
                }
            }
            
            if (!match) {
                // Check tags
                for (size_t j = 0; j < vector_size(memory->tags); j++) {
                    char* tag = string_vector_get(memory->tags, j);
                    if (tag) {
                        char* tag_lower = strdup(tag);
                        if (tag_lower) {
                            for (char* p = tag_lower; *p; p++) *p = tolower(*p);
                            if (strstr(tag_lower, query_lower)) {
                                match = 1;
                                free(tag_lower);
                                break;
                            }
                            free(tag_lower);
                        }
                    }
                }
            }
            
            if (match) {
                vector_push(result, memory);
            }
            
            entry = entry->next;
        }
    }
    
    free(query_lower);
    return result;
}

Vector* memory_manager_get_memories_by_importance(MemoryManager* manager, MemoryImportance importance) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->importance == importance) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_memories_by_term(MemoryManager* manager, MemoryTerm term) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->term == term) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_memories_by_tag(MemoryManager* manager, const char* tag) {
    if (!manager || !tag) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    char* tag_lower = strdup(tag);
    if (!tag_lower) {
        vector_destroy(result, NULL);
        return NULL;
    }
    for (char* p = tag_lower; *p; p++) *p = tolower(*p);
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            int found = 0;
            for (size_t j = 0; j < vector_size(memory->tags); j++) {
                char* mem_tag = string_vector_get(memory->tags, j);
                if (mem_tag) {
                    char* mem_tag_lower = strdup(mem_tag);
                    if (mem_tag_lower) {
                        for (char* p = mem_tag_lower; *p; p++) *p = tolower(*p);
                        if (strcmp(mem_tag_lower, tag_lower) == 0) {
                            found = 1;
                            free(mem_tag_lower);
                            break;
                        }
                        free(mem_tag_lower);
                    }
                }
            }
            if (found) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    
    free(tag_lower);
    return result;
}

// Comparison function for qsort
static int cmp_mem_created(const void *a, const void *b) {
    const Memory *ma = *(Memory* const*)a;
    const Memory *mb = *(Memory* const*)b;
    if (ma->created_at < mb->created_at) return 1;
    if (ma->created_at > mb->created_at) return -1;
    return 0;
}

Vector* memory_manager_get_recent_memories(MemoryManager* manager, size_t limit) {
    if (!manager) return NULL;
    Vector* memories = memory_manager_get_all_memories(manager);
    if (!memories) return NULL;
    
    // Sort using qsort
    qsort(memories->data, vector_size(memories), sizeof(void*), cmp_mem_created);
    
    // Take first 'limit' elements
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(memories, NULL);
        return NULL;
    }
    
    size_t count = (limit < vector_size(memories)) ? limit : vector_size(memories);
    for (size_t i = 0; i < count; i++) {
        vector_push(result, vector_get(memories, i));
    }
    
    vector_destroy(memories, NULL);
    return result;
}

Vector* memory_manager_get_critical_memories(MemoryManager* manager) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->importance == MEMORY_IMPORTANCE_HIGH || 
                memory->importance == MEMORY_IMPORTANCE_CRITICAL) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_short_term_memories(MemoryManager* manager) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->term == MEMORY_TERM_SHORT) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_long_term_memories(MemoryManager* manager) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->term == MEMORY_TERM_LONG) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_memories_by_type(MemoryManager* manager, MemoryType memory_type) {
    if (!manager) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->memory_type == memory_type) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_secret_memories(MemoryManager* manager) {
    return memory_manager_get_memories_by_type(manager, MEMORY_TYPE_SECRET);
}

Vector* memory_manager_get_human_memories(MemoryManager* manager) {
    return memory_manager_get_memories_by_type(manager, MEMORY_TYPE_HUMAN);
}

Vector* memory_manager_get_emotional_memories(MemoryManager* manager, const char* emotion) {
    if (!manager || !emotion) return NULL;
    Vector* result = vector_create();
    if (!result) return NULL;
    
    for (size_t i = 0; i < manager->memories->capacity; i++) {
        HashMapEntry* entry = manager->memories->buckets[i];
        while (entry) {
            Memory* memory = (Memory*)entry->value;
            if (memory->memory_type == MEMORY_TYPE_EMOTIONAL && 
                memory->emotion && 
                strcmp(memory->emotion, emotion) == 0) {
                vector_push(result, memory);
            }
            entry = entry->next;
        }
    }
    return result;
}

Vector* memory_manager_get_all_tags(MemoryManager* manager) {
    if (!manager) return NULL;
    // Using a simple approach with a fixed-size array for unique tags
    char** unique_tags = malloc(sizeof(char*) * 1000);
    if (!unique_tags) return NULL;
    size_t tag_count = 0;
    
    for (size_t i = 0; i < manager->memory_tags->capacity; i++) {
        HashMapEntry* entry = manager->memory_tags->buckets[i];
        while (entry) {
            Vector* tags = (Vector*)entry->value;
            for (size_t j = 0; j < vector_size(tags); j++) {
                char* tag = string_vector_get(tags, j);
                if (tag) {
                    // Check if tag already exists
                    int found = 0;
                    for (size_t k = 0; k < tag_count; k++) {
                        if (strcmp(unique_tags[k], tag) == 0) {
                            found = 1;
                            break;
                        }
                    }
                    
                    if (!found && tag_count < 1000) {
                        unique_tags[tag_count++] = strdup(tag);
                    }
                }
            }
            entry = entry->next;
        }
    }
    
    Vector* result = vector_create();
    if (!result) {
        for (size_t i = 0; i < tag_count; i++) {
            free(unique_tags[i]);
        }
        free(unique_tags);
        return NULL;
    }
    
    for (size_t i = 0; i < tag_count; i++) {
        vector_push(result, unique_tags[i]); // Transfer ownership
    }
    
    free(unique_tags);
    return result;
}

HashMap* memory_manager_get_tag_statistics(MemoryManager* manager) {
    if (!manager) return NULL;
    HashMap* stats = hashmap_create(100);
    if (!stats) return NULL;
    
    for (size_t i = 0; i < manager->memory_tags->capacity; i++) {
        HashMapEntry* entry = manager->memory_tags->buckets[i];
        while (entry) {
            Vector* tags = (Vector*)entry->value;
            for (size_t j = 0; j < vector_size(tags); j++) {
                char* tag = string_vector_get(tags, j);
                if (tag) {
                    int* count = (int*)hashmap_get(stats, tag);
                    if (count) {
                        (*count)++;
                    } else {
                        count = malloc(sizeof(int));
                        if (count) {
                            *count = 1;
                            hashmap_put(stats, strdup(tag), count);
                        }
                    }
                }
            }
            entry = entry->next;
        }
    }
    
    return stats;
}

MemoryStatistics* memory_manager_get_memory_statistics(MemoryManager* manager) {
    if (!manager) return NULL;
    MemoryStatistics* stats = memory_statistics_create();
    if (!stats) return NULL;
    
    stats->total_memories = hashmap_size(manager->memories);
    stats->short_term_memories = vector_size(memory_manager_get_short_term_memories(manager));
    stats->long_term_memories = vector_size(memory_manager_get_long_term_memories(manager));
    stats->critical_memories = vector_size(memory_manager_get_critical_memories(manager));
    stats->unique_tags = vector_size(memory_manager_get_all_tags(manager));
    stats->secret_memories = vector_size(memory_manager_get_secret_memories(manager));
    stats->human_memories = vector_size(memory_manager_get_human_memories(manager));
    
    // Count emotional memories
    const char* emotions[] = {
        "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
        "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
        "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
        "resigned"
    };
    size_t emotion_count = sizeof(emotions) / sizeof(emotions[0]);
    
    for (size_t i = 0; i < emotion_count; i++) {
        Vector* emotional_mems = memory_manager_get_emotional_memories(manager, emotions[i]);
        if (emotional_mems) {
            stats->emotional_memories += vector_size(emotional_mems);
            vector_destroy(emotional_mems, NULL);
        }
    }
    
    stats->standard_memories = vector_size(memory_manager_get_memories_by_type(manager, MEMORY_TYPE_STANDARD));
    
    return stats;
}

// Helper function to parse memory importance from string
int parse_memory_importance(const char* str, MemoryImportance* result) {
    if (!str || !result) return 0;
    if (strcmp(str, "low") == 0) {
        *result = MEMORY_IMPORTANCE_LOW;
        return 1;
    } else if (strcmp(str, "medium") == 0) {
        *result = MEMORY_IMPORTANCE_MEDIUM;
        return 1;
    } else if (strcmp(str, "high") == 0) {
        *result = MEMORY_IMPORTANCE_HIGH;
        return 1;
    } else if (strcmp(str, "critical") == 0) {
        *result = MEMORY_IMPORTANCE_CRITICAL;
        return 1;
    }
    return 0; // Failed to parse
}

// Helper function to parse memory term from string
int parse_memory_term(const char* str, MemoryTerm* result) {
    if (!str || !result) return 0;
    if (strcmp(str, "short") == 0) {
        *result = MEMORY_TERM_SHORT;
        return 1;
    } else if (strcmp(str, "long") == 0) {
        *result = MEMORY_TERM_LONG;
        return 1;
    }
    return 0; // Failed to parse
}

// Parse memory format function
TodoziError* parse_memory_format(const char* memory_text, const char* user_id, Memory** result) {
    if (!memory_text || !user_id || !result) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid parameters");
        return error;
    }
    
    const char* start_tag = "<memory>";
    const char* end_tag = "</memory>";
    
    const char* start = strstr(memory_text, start_tag);
    if (!start) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Missing <memory> start tag");
        return error;
    }
    
    const char* end = strstr(memory_text, end_tag);
    if (!end) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Missing </memory> end tag");
        return error;
    }
    
    start += strlen(start_tag);
    size_t content_len = end - start;
    char* content = strndup(start, content_len);
    if (!content) {
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_OTHER;
        error->message = strdup("Memory allocation failed");
        return error;
    }
    
    // Split by ';'
    Vector* parts = vector_create();
    if (!parts) {
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_OTHER;
        error->message = strdup("Memory allocation failed");
        return error;
    }
    
    char* content_copy = strdup(content);
    if (!content_copy) {
        vector_destroy(parts, NULL);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_OTHER;
        error->message = strdup("Memory allocation failed");
        return error;
    }
    
    char* token = strtok(content_copy, ";");
    while (token) {
        // Trim whitespace
        while (*token == ' ' || *token == '\t') token++;
        char* end_ptr = token + strlen(token) - 1;
        while (end_ptr > token && (*end_ptr == ' ' || *end_ptr == '\t')) end_ptr--;
        *(end_ptr + 1) = '\0';
        if (*token) string_vector_push(parts, token);
        token = strtok(NULL, ";");
    }
    
    if (vector_size(parts) < 6) {
        free(content_copy);
        vector_destroy(parts, NULL);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid memory format: need at least 6 parts (type; moment; meaning; reason; importance; term)");
        return error;
    }
    
    Memory* memory = memory_create();
    if (!memory) {
        free(content_copy);
        vector_destroy(parts, NULL);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_OTHER;
        error->message = strdup("Memory allocation failed");
        return error;
    }
    
    memory->user_id = strdup(user_id);
    memory->project_id = NULL;
    memory->status = ITEM_STATUS_ACTIVE;
    
    char* memory_type_str = string_vector_get(parts, 0);
    const char* emotion_list[] = {
        "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
        "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
        "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
        "resigned"
    };
    size_t emotion_count = sizeof(emotion_list) / sizeof(emotion_list[0]);
    
    int is_emotion = 0;
    for (size_t i = 0; i < emotion_count; i++) {
        if (strcmp(memory_type_str, emotion_list[i]) == 0) {
            is_emotion = 1;
            memory->memory_type = MEMORY_TYPE_EMOTIONAL;
            memory->emotion = strdup(memory_type_str);
            break;
        }
    }
    
    if (!is_emotion) {
        if (strcmp(memory_type_str, "standard") == 0) {
            memory->memory_type = MEMORY_TYPE_STANDARD;
        } else if (strcmp(memory_type_str, "secret") == 0) {
            memory->memory_type = MEMORY_TYPE_SECRET;
        } else if (strcmp(memory_type_str, "human") == 0) {
            memory->memory_type = MEMORY_TYPE_HUMAN;
        } else if (strcmp(memory_type_str, "short") == 0) {
            memory->memory_type = MEMORY_TYPE_SHORT;
        } else if (strcmp(memory_type_str, "long") == 0) {
            memory->memory_type = MEMORY_TYPE_LONG;
        } else {
            memory->memory_type = MEMORY_TYPE_STANDARD;
        }
        memory->emotion = NULL;
    }
    
    memory->moment = strdup(string_vector_get(parts, 1));
    memory->meaning = strdup(string_vector_get(parts, 2));
    memory->reason = strdup(string_vector_get(parts, 3));
    
    if (!parse_memory_importance(string_vector_get(parts, 4), &memory->importance)) {
        memory_destroy(memory);
        free(content_copy);
        vector_destroy(parts, NULL);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid memory importance");
        return error;
    }
    
    if (!parse_memory_term(string_vector_get(parts, 5), &memory->term)) {
        memory_destroy(memory);
        free(content_copy);
        vector_destroy(parts, NULL);
        free(content);
        TodoziError* error = malloc(sizeof(TodoziError));
        if (!error) return NULL;
        error->type = TODOZI_ERROR_VALIDATION;
        error->message = strdup("Invalid memory term");
        return error;
    }
    
    // Parse tags if present
    if (vector_size(parts) > 6) {
        char* tags_str = string_vector_get(parts, 6);
        if (tags_str && strlen(tags_str) > 0) {
            char* tags_copy = strdup(tags_str);
            if (tags_copy) {
                char* tag_token = strtok(tags_copy, ",");
                while (tag_token) {
                    // Trim whitespace
                    while (*tag_token == ' ' || *tag_token == '\t') tag_token++;
                    char* tag_end = tag_token + strlen(tag_token) - 1;
                    while (tag_end > tag_token && (*tag_end == ' ' || *tag_end == '\t')) tag_end--;
                    *(tag_end + 1) = '\0';
                    if (*tag_token) string_vector_push(memory->tags, tag_token);
                    tag_token = strtok(NULL, ",");
                }
                free(tags_copy);
            }
        }
    }
    
    memory->created_at = time(NULL);
    memory->updated_at = time(NULL);
    
    free(content_copy);
    vector_destroy(parts, NULL);
    free(content);
    
    *result = memory;
    return NULL; // Success
}

// Test functions
void test_memory_manager_creation() {
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        printf("Failed to create MemoryManager\n");
        return;
    }
    printf("MemoryManager created\n");
    printf("Memories count: %zu\n", hashmap_size(manager->memories));
    printf("Memory tags count: %zu\n", hashmap_size(manager->memory_tags));
    memory_manager_destroy(manager);
}

void test_memory_update_builder() {
    MemoryUpdate* update = memory_update_create();
    if (!update) {
        printf("Failed to create MemoryUpdate\n");
        return;
    }
    memory_update_moment(update, "New moment");
    memory_update_meaning(update, "New meaning");
    memory_update_importance(update, MEMORY_IMPORTANCE_HIGH);
    
    printf("MemoryUpdate created\n");
    printf("Moment: %s\n", update->moment ? update->moment : "NULL");
    printf("Meaning: %s\n", update->meaning ? update->meaning : "NULL");
    if (update->importance) {
        printf("Importance: %d\n", *update->importance);
    }
    
    memory_update_destroy(update);
}

void test_memory_statistics_percentages() {
    MemoryStatistics stats;
    stats.total_memories = 10;
    stats.short_term_memories = 6;
    stats.long_term_memories = 4;
    stats.critical_memories = 2;
    stats.unique_tags = 8;
    stats.secret_memories = 1;
    stats.human_memories = 2;
    stats.emotional_memories = 3;
    stats.standard_memories = 4;
    
    printf("Short term percentage: %.2f\n", memory_statistics_short_term_percentage(&stats));
    printf("Long term percentage: %.2f\n", memory_statistics_long_term_percentage(&stats));
    printf("Critical percentage: %.2f\n", memory_statistics_critical_percentage(&stats));
    
    MemoryStatistics empty_stats;
    empty_stats.total_memories = 0;
    empty_stats.short_term_memories = 0;
    empty_stats.long_term_memories = 0;
    empty_stats.critical_memories = 0;
    empty_stats.unique_tags = 0;
    empty_stats.secret_memories = 0;
    empty_stats.human_memories = 0;
    empty_stats.emotional_memories = 0;
    empty_stats.standard_memories = 0;
    
    printf("Empty short term percentage: %.2f\n", memory_statistics_short_term_percentage(&empty_stats));
    printf("Empty long term percentage: %.2f\n", memory_statistics_long_term_percentage(&empty_stats));
    printf("Empty critical percentage: %.2f\n", memory_statistics_critical_percentage(&empty_stats));
}

void test_parse_memory_format() {
    const char* memory_text = "<memory>standard; 2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long; client,development,iterative</memory>";
    Memory* memory;
    TodoziError* error = parse_memory_format(memory_text, "user_123", &memory);
    
    if (error) {
        printf("Error: %s\n", error->message);
        todozi_error_free(error);
        return;
    }
    
    printf("Memory parsed successfully\n");
    printf("Moment: %s\n", memory->moment ? memory->moment : "NULL");
    printf("Meaning: %s\n", memory->meaning ? memory->meaning : "NULL");
    printf("Reason: %s\n", memory->reason ? memory->reason : "NULL");
    printf("Importance: %d\n", memory->importance);
    printf("Term: %d\n", memory->term);
    printf("Memory type: %d\n", memory->memory_type);
    
    printf("Tags: ");
    for (size_t i = 0; i < vector_size(memory->tags); i++) {
        char* tag = string_vector_get(memory->tags, i);
        printf("%s ", tag ? tag : "NULL");
    }
    printf("\n");
    
    memory_destroy(memory);
}

int main() {
    printf("Running tests...\n");
    test_memory_manager_creation();
    test_memory_update_builder();
    test_memory_statistics_percentages();
    test_parse_memory_format();
    printf("Tests completed.\n");
    return 0;
}
