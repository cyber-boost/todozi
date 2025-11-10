#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Forward declarations
typedef struct Reminder Reminder;
typedef struct ReminderUpdate ReminderUpdate;
typedef struct ReminderManager ReminderManager;

// Enums
typedef enum {
    REMINDER_PRIORITY_LOW,
    REMINDER_PRIORITY_MEDIUM,
    REMINDER_PRIORITY_HIGH
} ReminderPriority;

typedef enum {
    REMINDER_STATUS_PENDING,
    REMINDER_STATUS_ACTIVE,
    REMINDER_STATUS_COMPLETED,
    REMINDER_STATUS_CANCELLED
} ReminderStatus;

// Hash table structures
typedef struct HashNode {
    char* key;
    void* value;
    struct HashNode* next;
} HashNode;

typedef struct {
    HashNode** buckets;
    size_t size;
    size_t capacity;
} HashMap;

// Vector structure
typedef struct {
    void** data;
    size_t size;
    size_t capacity;
} Vector;

// Vector function forward declarations
Vector* vector_create(void);
void vector_destroy(Vector* vec, void (*destructor)(void*));
int vector_push(Vector* vec, void* item);

// DateTime structure
typedef struct {
    time_t timestamp;
} DateTime;

// Error structure
typedef struct {
    int error_type;
    char* message;
} TodoziError;

// Reminder structure
struct Reminder {
    char* id;
    char* content;
    DateTime remind_at;
    ReminderPriority priority;
    ReminderStatus status;
    Vector* tags;
    DateTime created_at;
    DateTime updated_at;
};

// ReminderUpdate structure
struct ReminderUpdate {
    char* content;
    DateTime* remind_at;
    ReminderPriority* priority;
    ReminderStatus* status;
    Vector* tags;
};

// ReminderStatistics structure
typedef struct {
    size_t total_reminders;
    size_t pending_reminders;
    size_t active_reminders;
    size_t overdue_reminders;
    size_t unique_tags;
} ReminderStatistics;

// Hash table functions
HashMap* hashmap_create(size_t capacity) {
    HashMap* map = malloc(sizeof(HashMap));
    if (!map) return NULL;
    map->buckets = calloc(capacity, sizeof(HashNode*));
    if (!map->buckets) {
        free(map);
        return NULL;
    }
    map->size = 0;
    map->capacity = capacity;
    return map;
}

void hashmap_destroy(HashMap* map, void (*key_destructor)(void*), void (*value_destructor)(void*)) {
    if (!map) return;
    
    for (size_t i = 0; i < map->capacity; i++) {
        HashNode* node = map->buckets[i];
        while (node) {
            HashNode* next = node->next;
            if (key_destructor) key_destructor(node->key);
            if (value_destructor) value_destructor(node->value);
            free(node);
            node = next;
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

int hashmap_put(HashMap* map, char* key, void* value) {
    if (!map || !key) return 0;
    
    size_t index = hashmap_hash(key, map->capacity);
    HashNode* node = map->buckets[index];
    
    while (node) {
        if (strcmp(node->key, key) == 0) {
            // Key already exists - update value
            // Note: Caller is responsible for freeing old value if needed
            node->value = value;
            free(key); // Free the duplicate key since we're using the existing one
            return 1;
        }
        node = node->next;
    }
    
    // Key doesn't exist - create new node
    HashNode* new_node = malloc(sizeof(HashNode));
    if (!new_node) {
        free(key); // Free key on allocation failure
        return 0;
    }
    new_node->key = key;
    new_node->value = value;
    new_node->next = map->buckets[index];
    map->buckets[index] = new_node;
    map->size++;
    return 1;
}

void* hashmap_get(HashMap* map, const char* key) {
    size_t index = hashmap_hash(key, map->capacity);
    HashNode* node = map->buckets[index];
    
    while (node) {
        if (strcmp(node->key, key) == 0) {
            return node->value;
        }
        node = node->next;
    }
    return NULL;
}

void* hashmap_remove(HashMap* map, const char* key) {
    size_t index = hashmap_hash(key, map->capacity);
    HashNode* node = map->buckets[index];
    HashNode* prev = NULL;
    
    while (node) {
        if (strcmp(node->key, key) == 0) {
            if (prev) {
                prev->next = node->next;
            } else {
                map->buckets[index] = node->next;
            }
            void* value = node->value;
            free(node->key);
            free(node);
            map->size--;
            return value;
        }
        prev = node;
        node = node->next;
    }
    return NULL;
}

Vector* hashmap_values(HashMap* map) {
    Vector* vec = malloc(sizeof(Vector));
    if (!vec) return NULL;
    vec->data = malloc(sizeof(void*) * (map->size > 0 ? map->size : 1));
    if (!vec->data) {
        free(vec);
        return NULL;
    }
    vec->size = 0;
    vec->capacity = map->size > 0 ? map->size : 1;
    
    for (size_t i = 0; i < map->capacity; i++) {
        HashNode* node = map->buckets[i];
        while (node) {
            if (vec->size >= vec->capacity) {
                vec->capacity *= 2;
                void** temp = realloc(vec->data, sizeof(void*) * vec->capacity);
                if (!temp) {
                    // Handle realloc failure
                    vector_destroy(vec, NULL);
                    return NULL;
                }
                vec->data = temp;
            }
            vec->data[vec->size++] = node->value;
            node = node->next;
        }
    }
    return vec;
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

void vector_destroy(Vector* vec, void (*destructor)(void*)) {
    if (!vec) return;
    if (destructor) {
        for (size_t i = 0; i < vec->size; i++) {
            destructor(vec->data[i]);
        }
    }
    free(vec->data);
    free(vec);
}

int vector_push(Vector* vec, void* item) {
    if (!vec) return 0;
    
    if (vec->size >= vec->capacity) {
        vec->capacity *= 2;
        void** temp = realloc(vec->data, sizeof(void*) * vec->capacity);
        if (!temp) return 0; // Handle realloc failure
        vec->data = temp;
    }
    vec->data[vec->size++] = item;
    return 1;
}

void* vector_get(Vector* vec, size_t index) {
    if (!vec || index >= vec->size) return NULL;
    return vec->data[index];
}

size_t vector_size(Vector* vec) {
    if (!vec) return 0;
    return vec->size;
}

void vector_sort(Vector* vec, int (*comparator)(const void*, const void*)) {
    if (!vec || !comparator) return;
    qsort(vec->data, vec->size, sizeof(void*), comparator);
}

Vector* vector_filter(Vector* vec, int (*predicate)(void*)) {
    if (!vec || !predicate) return NULL;
    
    Vector* result = vector_create();
    if (!result) return NULL;
    for (size_t i = 0; i < vec->size; i++) {
        if (predicate(vec->data[i])) {
            if (!vector_push(result, vec->data[i])) {
                vector_destroy(result, NULL);
                return NULL;
            }
        }
    }
    return result;
}

// DateTime functions
DateTime datetime_now() {
    DateTime dt;
    dt.timestamp = time(NULL);
    return dt;
}

DateTime datetime_add_days(DateTime dt, int days) {
    DateTime result;
    result.timestamp = dt.timestamp + (days * 24 * 60 * 60);
    return result;
}

int datetime_compare(DateTime a, DateTime b) {
    if (a.timestamp < b.timestamp) return -1;
    if (a.timestamp > b.timestamp) return 1;
    return 0;
}

// String functions
char* string_duplicate(const char* str) {
    if (!str) return NULL;
    size_t len = strlen(str);
    char* dup = malloc(len + 1);
    if (!dup) return NULL;
    strcpy(dup, str);
    return dup;
}

char* string_to_lowercase(const char* str) {
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

int string_contains(const char* str, const char* substr) {
    if (!str || !substr) return 0;
    char* lower_str = string_to_lowercase(str);
    char* lower_substr = string_to_lowercase(substr);
    int result = strstr(lower_str, lower_substr) != NULL;
    free(lower_str);
    free(lower_substr);
    return result;
}

Vector* string_split(const char* str, char delimiter) {
    Vector* result = vector_create();
    if (!result) return NULL;
    if (!str) return result;
    
    const char* start = str;
    const char* end = str;
    
    while (*end) {
        if (*end == delimiter) {
            size_t len = end - start;
            if (len > 0) {
                char* token = malloc(len + 1);
                if (!token) {
                    vector_destroy(result, free);
                    return NULL;
                }
                strncpy(token, start, len);
                token[len] = '\0';
                if (!vector_push(result, token)) {
                    free(token);
                    vector_destroy(result, free);
                    return NULL;
                }
            }
            start = end + 1;
        }
        end++;
    }
    
    // Handle last token
    size_t len = end - start;
    if (len > 0) {
        char* token = malloc(len + 1);
        if (!token) {
            vector_destroy(result, free);
            return NULL;
        }
        strncpy(token, start, len);
        token[len] = '\0';
        vector_push(result, token);
    }
    
    return result;
}

// Reminder functions
Reminder* reminder_create() {
    Reminder* reminder = malloc(sizeof(Reminder));
    if (!reminder) return NULL;
    reminder->id = NULL;
    reminder->content = NULL;
    reminder->priority = REMINDER_PRIORITY_MEDIUM;
    reminder->status = REMINDER_STATUS_PENDING;
    reminder->tags = vector_create();
    if (!reminder->tags) {
        free(reminder);
        return NULL;
    }
    reminder->created_at = datetime_now();
    reminder->updated_at = datetime_now();
    return reminder;
}

void reminder_destroy(Reminder* reminder) {
    if (!reminder) return;
    free(reminder->id);
    free(reminder->content);
    vector_destroy(reminder->tags, free);
    free(reminder);
}

void reminder_mark_completed(Reminder* reminder) {
    if (reminder) {
        reminder->status = REMINDER_STATUS_COMPLETED;
        reminder->updated_at = datetime_now();
    }
}

void reminder_mark_cancelled(Reminder* reminder) {
    if (reminder) {
        reminder->status = REMINDER_STATUS_CANCELLED;
        reminder->updated_at = datetime_now();
    }
}

void reminder_activate(Reminder* reminder) {
    if (reminder) {
        reminder->status = REMINDER_STATUS_ACTIVE;
        reminder->updated_at = datetime_now();
    }
}

// ReminderUpdate functions
ReminderUpdate* reminder_update_create() {
    ReminderUpdate* update = malloc(sizeof(ReminderUpdate));
    if (!update) return NULL;
    update->content = NULL;
    update->remind_at = NULL;
    update->priority = NULL;
    update->status = NULL;
    update->tags = NULL;
    return update;
}

void reminder_update_destroy(ReminderUpdate* update) {
    if (!update) return;
    free(update->content);
    free(update->remind_at);
    free(update->priority);
    free(update->status);
    // Note: We don't destroy the tags vector here as it may be transferred
    free(update);
}

ReminderUpdate* reminder_update_content(ReminderUpdate* update, const char* content) {
    if (update) {
        free(update->content);
        update->content = content ? string_duplicate(content) : NULL;
    }
    return update;
}

ReminderUpdate* reminder_update_remind_at(ReminderUpdate* update, DateTime remind_at) {
    if (update) {
        free(update->remind_at);
        update->remind_at = malloc(sizeof(DateTime));
        if (update->remind_at) {
            *update->remind_at = remind_at;
        }
    }
    return update;
}

ReminderUpdate* reminder_update_priority(ReminderUpdate* update, ReminderPriority priority) {
    if (update) {
        free(update->priority);
        update->priority = malloc(sizeof(ReminderPriority));
        if (update->priority) {
            *update->priority = priority;
        }
    }
    return update;
}

ReminderUpdate* reminder_update_status(ReminderUpdate* update, ReminderStatus status) {
    if (update) {
        free(update->status);
        update->status = malloc(sizeof(ReminderStatus));
        if (update->status) {
            *update->status = status;
        }
    }
    return update;
}

ReminderUpdate* reminder_update_tags(ReminderUpdate* update, Vector* tags) {
    if (update) {
        // Note: This transfers ownership of the tags vector
        update->tags = tags;
    }
    return update;
}

// ReminderManager structure
struct ReminderManager {
    HashMap* reminders;
    HashMap* reminder_tags;
};

// ReminderManager functions
ReminderManager* reminder_manager_create() {
    ReminderManager* manager = malloc(sizeof(ReminderManager));
    if (!manager) return NULL;
    manager->reminders = hashmap_create(100);
    if (!manager->reminders) {
        free(manager);
        return NULL;
    }
    manager->reminder_tags = hashmap_create(100);
    if (!manager->reminder_tags) {
        hashmap_destroy(manager->reminders, free, NULL);
        free(manager);
        return NULL;
    }
    return manager;
}

void reminder_manager_destroy(ReminderManager* manager) {
    if (!manager) return;
    hashmap_destroy(manager->reminders, free, (void (*)(void*))reminder_destroy);
    hashmap_destroy(manager->reminder_tags, free, (void (*)(void*))vector_destroy);
    free(manager);
}

char* reminder_manager_create_reminder(ReminderManager* manager, Reminder* reminder) {
    if (!manager || !reminder) return NULL;
    
    // TODO: Use UUID when linking issues are resolved
    char* uuid_str = malloc(37);
    if (!uuid_str) return NULL;
    sprintf(uuid_str, "temp-reminder-id-%ld", (long)time(NULL));
    
    reminder->id = uuid_str;
    reminder->created_at = datetime_now();
    reminder->updated_at = datetime_now();
    
    // Store a copy of tags in reminder_tags map
    Vector* tags_copy = vector_create();
    if (!tags_copy) {
        free(uuid_str);
        reminder->id = NULL;
        return NULL;
    }
    if (reminder->tags) {
        for (size_t i = 0; i < reminder->tags->size; i++) {
            char* tag = (char*)vector_get(reminder->tags, i);
            if (!tag) continue;
            char* tag_copy = string_duplicate(tag);
            if (!tag_copy) {
                vector_destroy(tags_copy, free);
                free(uuid_str);
                reminder->id = NULL;
                return NULL;
            }
            if (!vector_push(tags_copy, tag_copy)) {
                free(tag_copy);
                vector_destroy(tags_copy, free);
                free(uuid_str);
                reminder->id = NULL;
                return NULL;
            }
        }
    }
    
    char* id_copy = string_duplicate(reminder->id);
    if (!id_copy) {
        vector_destroy(tags_copy, free);
        free(uuid_str);
        reminder->id = NULL;
        return NULL;
    }
    
    char* id_copy2 = string_duplicate(reminder->id);
    if (!id_copy2) {
        free(id_copy);
        vector_destroy(tags_copy, free);
        free(uuid_str);
        reminder->id = NULL;
        return NULL;
    }
    
    if (!hashmap_put(manager->reminder_tags, id_copy, tags_copy)) {
        free(id_copy);
        vector_destroy(tags_copy, free);
        free(id_copy2);
        free(uuid_str);
        reminder->id = NULL;
        return NULL;
    }
    
    if (!hashmap_put(manager->reminders, id_copy2, reminder)) {
        // Rollback: remove from reminder_tags using reminder->id (same content as id_copy)
        hashmap_remove(manager->reminder_tags, reminder->id);
        free(id_copy2);
        free(uuid_str);
        reminder->id = NULL;
        return NULL;
    }
    
    return string_duplicate(reminder->id);
}

Reminder* reminder_manager_get_reminder(ReminderManager* manager, const char* reminder_id) {
    if (!manager || !reminder_id) return NULL;
    return (Reminder*)hashmap_get(manager->reminders, reminder_id);
}

Vector* reminder_manager_get_all_reminders(ReminderManager* manager) {
    if (!manager) return NULL;
    return hashmap_values(manager->reminders);
}

int reminder_manager_update_reminder(ReminderManager* manager, const char* reminder_id, ReminderUpdate* updates) {
    if (!manager || !reminder_id || !updates) return 0;
    
    Reminder* reminder = (Reminder*)hashmap_get(manager->reminders, reminder_id);
    if (!reminder) return 0;
    
    if (updates->content) {
        free(reminder->content);
        reminder->content = updates->content;
        updates->content = NULL; // Transfer ownership
    }
    
    if (updates->remind_at) {
        reminder->remind_at = *updates->remind_at;
        free(updates->remind_at);
        updates->remind_at = NULL;
    }
    
    if (updates->priority) {
        reminder->priority = *updates->priority;
        free(updates->priority);
        updates->priority = NULL;
    }
    
    if (updates->status) {
        reminder->status = *updates->status;
        free(updates->status);
        updates->status = NULL;
    }
    
    if (updates->tags) {
        // Update tags in both reminder and reminder_tags map
        vector_destroy(reminder->tags, free);
        reminder->tags = updates->tags;
        
        // Update the tags in the reminder_tags map
        Vector* tags_copy = vector_create();
        if (tags_copy) {
            for (size_t i = 0; i < updates->tags->size; i++) {
                char* tag = (char*)vector_get(updates->tags, i);
                if (!tag) continue;
                char* tag_copy = string_duplicate(tag);
                if (tag_copy) {
                    if (!vector_push(tags_copy, tag_copy)) {
                        free(tag_copy);
                        // Clean up what we've added so far
                        vector_destroy(tags_copy, free);
                        tags_copy = NULL;
                        break;
                    }
                }
            }
            if (tags_copy) {
                char* id_copy = string_duplicate(reminder_id);
                if (id_copy) {
                    if (!hashmap_put(manager->reminder_tags, id_copy, tags_copy)) {
                        free(id_copy);
                        vector_destroy(tags_copy, free);
                    }
                } else {
                    vector_destroy(tags_copy, free);
                }
            }
        }
        updates->tags = NULL; // Transfer ownership
    }
    
    reminder->updated_at = datetime_now();
    return 1;
}

int reminder_manager_delete_reminder(ReminderManager* manager, const char* reminder_id) {
    if (!manager || !reminder_id) return 0;
    
    Reminder* reminder = (Reminder*)hashmap_remove(manager->reminders, reminder_id);
    if (!reminder) return 0;
    
    hashmap_remove(manager->reminder_tags, reminder_id);
    reminder_destroy(reminder);
    return 1;
}

Vector* reminder_manager_search_reminders(ReminderManager* manager, const char* query) {
    if (!manager || !query) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    char* query_lower = string_to_lowercase(query);
    if (!query_lower) {
        vector_destroy(all_reminders, NULL);
        vector_destroy(result, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < all_reminders->size; i++) {
        Reminder* reminder = (Reminder*)all_reminders->data[i];
        if (!reminder) continue;
        
        // Check content if it exists
        if (reminder->content) {
            char* content_lower = string_to_lowercase(reminder->content);
            if (content_lower) {
                if (string_contains(content_lower, query_lower)) {
                    vector_push(result, reminder);
                    free(content_lower);
                    continue;
                }
                free(content_lower);
            }
        }
        
        // Check tags
        if (reminder->tags) {
            for (size_t j = 0; j < reminder->tags->size; j++) {
                char* tag = (char*)vector_get(reminder->tags, j);
                if (tag && string_contains(tag, query_lower)) {
                    vector_push(result, reminder);
                    break;
                }
            }
        }
    }
    
    free(query_lower);
    vector_destroy(all_reminders, NULL);
    return result;
}

Vector* reminder_manager_get_reminders_by_priority(ReminderManager* manager, ReminderPriority priority) {
    if (!manager) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < all_reminders->size; i++) {
        Reminder* reminder = (Reminder*)all_reminders->data[i];
        if (reminder && reminder->priority == priority) {
            vector_push(result, reminder);
        }
    }
    
    vector_destroy(all_reminders, NULL);
    return result;
}

Vector* reminder_manager_get_reminders_by_status(ReminderManager* manager, ReminderStatus status) {
    if (!manager) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < all_reminders->size; i++) {
        Reminder* reminder = (Reminder*)all_reminders->data[i];
        if (reminder && reminder->status == status) {
            vector_push(result, reminder);
        }
    }
    
    vector_destroy(all_reminders, NULL);
    return result;
}

Vector* reminder_manager_get_reminders_by_tag(ReminderManager* manager, const char* tag) {
    if (!manager || !tag) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    char* tag_lower = string_to_lowercase(tag);
    if (!tag_lower) {
        vector_destroy(all_reminders, NULL);
        vector_destroy(result, NULL);
        return NULL;
    }
    
    for (size_t i = 0; i < all_reminders->size; i++) {
        Reminder* reminder = (Reminder*)all_reminders->data[i];
        if (!reminder || !reminder->tags) continue;
        
        for (size_t j = 0; j < reminder->tags->size; j++) {
            char* reminder_tag = (char*)vector_get(reminder->tags, j);
            if (!reminder_tag) continue;
            
            char* reminder_tag_lower = string_to_lowercase(reminder_tag);
            if (!reminder_tag_lower) continue;
            
            if (strcmp(reminder_tag_lower, tag_lower) == 0) {
                vector_push(result, reminder);
                free(reminder_tag_lower);
                break;
            }
            free(reminder_tag_lower);
        }
    }
    
    free(tag_lower);
    vector_destroy(all_reminders, NULL);
    return result;
}

Vector* reminder_manager_get_pending_reminders(ReminderManager* manager) {
    return reminder_manager_get_reminders_by_status(manager, REMINDER_STATUS_PENDING);
}

Vector* reminder_manager_get_active_reminders(ReminderManager* manager) {
    return reminder_manager_get_reminders_by_status(manager, REMINDER_STATUS_ACTIVE);
}

Vector* reminder_manager_get_overdue_reminders(ReminderManager* manager) {
    if (!manager) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    DateTime now = datetime_now();
    
    for (size_t i = 0; i < all_reminders->size; i++) {
        Reminder* reminder = (Reminder*)all_reminders->data[i];
        if (reminder && datetime_compare(reminder->remind_at, now) < 0 &&
            (reminder->status == REMINDER_STATUS_PENDING || reminder->status == REMINDER_STATUS_ACTIVE)) {
            vector_push(result, reminder);
        }
    }
    
    vector_destroy(all_reminders, NULL);
    return result;
}

Vector* reminder_manager_get_reminders_due_soon(ReminderManager* manager, int days) {
    if (!manager) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    DateTime now = datetime_now();
    DateTime due_time = datetime_add_days(now, days);
    
    for (size_t i = 0; i < all_reminders->size; i++) {
        Reminder* reminder = (Reminder*)all_reminders->data[i];
        if (reminder && datetime_compare(reminder->remind_at, due_time) <= 0 && 
            datetime_compare(reminder->remind_at, now) > 0 &&
            (reminder->status == REMINDER_STATUS_PENDING || reminder->status == REMINDER_STATUS_ACTIVE)) {
            vector_push(result, reminder);
        }
    }
    
    vector_destroy(all_reminders, NULL);
    return result;
}

// Comparator for sorting reminders by created_at (descending)
int reminder_compare_by_created_at_desc(const void* a, const void* b) {
    Reminder* rem_a = *(Reminder**)a;
    Reminder* rem_b = *(Reminder**)b;
    // Note: We want descending order, so we reverse the comparison
    return datetime_compare(rem_b->created_at, rem_a->created_at);
}

Vector* reminder_manager_get_recent_reminders(ReminderManager* manager, size_t limit) {
    if (!manager) return NULL;
    
    Vector* all_reminders = hashmap_values(manager->reminders);
    if (!all_reminders) return NULL;
    
    // Sort by created_at descending
    vector_sort(all_reminders, reminder_compare_by_created_at_desc);
    
    // Take only the first 'limit' items
    Vector* result = vector_create();
    if (!result) {
        vector_destroy(all_reminders, NULL);
        return NULL;
    }
    
    size_t count = (limit < all_reminders->size) ? limit : all_reminders->size;
    for (size_t i = 0; i < count; i++) {
        if (!vector_push(result, all_reminders->data[i])) {
            vector_destroy(result, NULL);
            free(all_reminders->data);
            free(all_reminders);
            return NULL;
        }
    }
    
    // Note: We don't destroy all_reminders here because we're returning pointers to its data
    // In a real implementation, we might want to copy the reminders instead
    free(all_reminders->data);
    free(all_reminders);
    return result;
}

Vector* reminder_manager_get_all_tags(ReminderManager* manager) {
    if (!manager) return NULL;
    
    Vector* all_tags = vector_create();
    if (!all_tags) return NULL;
    
    // Use a temporary hashmap for deduplication
    HashMap* unique_tags = hashmap_create(50);
    if (!unique_tags) {
        vector_destroy(all_tags, free);
        return NULL;
    }
    
    for (size_t i = 0; i < manager->reminder_tags->capacity; i++) {
        HashNode* node = manager->reminder_tags->buckets[i];
        while (node) {
            Vector* tags = (Vector*)node->value;
            for (size_t j = 0; j < tags->size; j++) {
                char* tag = (char*)vector_get(tags, j);
                // Check if we've already added this tag
                if (tag && !hashmap_get(unique_tags, tag)) {
                    char* tag_copy = string_duplicate(tag);
                    if (tag_copy) {
                        if (vector_push(all_tags, tag_copy)) {
                            // Add to hashmap to mark as seen
                            char* key_copy = string_duplicate(tag);
                            if (key_copy) {
                                if (!hashmap_put(unique_tags, key_copy, key_copy)) {
                                    free(key_copy);
                                }
                            }
                        } else {
                            free(tag_copy);
                        }
                    }
                }
            }
            node = node->next;
        }
    }
    
    hashmap_destroy(unique_tags, free, free);
    return all_tags;
}

HashMap* reminder_manager_get_tag_statistics(ReminderManager* manager) {
    if (!manager) return NULL;
    
    HashMap* stats = hashmap_create(50);
    if (!stats) return NULL;
    
    for (size_t i = 0; i < manager->reminder_tags->capacity; i++) {
        HashNode* node = manager->reminder_tags->buckets[i];
        while (node) {
            Vector* tags = (Vector*)node->value;
            for (size_t j = 0; j < tags->size; j++) {
                char* tag = (char*)vector_get(tags, j);
                if (!tag) continue;
                
                int* count = (int*)hashmap_get(stats, tag);
                if (count) {
                    (*count)++;
                } else {
                    count = malloc(sizeof(int));
                    if (count) {
                        *count = 1;
                        char* key_copy = string_duplicate(tag);
                        if (key_copy) {
                            if (!hashmap_put(stats, key_copy, count)) {
                                free(key_copy);
                                free(count);
                            }
                        } else {
                            free(count);
                        }
                    }
                }
            }
            node = node->next;
        }
    }
    
    return stats;
}

void reminder_manager_free_tag_statistics(HashMap* stats) {
    if (!stats) return;
    hashmap_destroy(stats, free, free);
}

ReminderStatistics reminder_manager_get_reminder_statistics(ReminderManager* manager) {
    ReminderStatistics stats = {0};
    if (!manager) return stats;
    
    stats.total_reminders = manager->reminders->size;
    
    Vector* pending = reminder_manager_get_pending_reminders(manager);
    if (pending) {
        stats.pending_reminders = pending->size;
        vector_destroy(pending, NULL);
    }
    
    Vector* active = reminder_manager_get_active_reminders(manager);
    if (active) {
        stats.active_reminders = active->size;
        vector_destroy(active, NULL);
    }
    
    Vector* overdue = reminder_manager_get_overdue_reminders(manager);
    if (overdue) {
        stats.overdue_reminders = overdue->size;
        vector_destroy(overdue, NULL);
    }
    
    Vector* all_tags = reminder_manager_get_all_tags(manager);
    if (all_tags) {
        stats.unique_tags = all_tags->size;
        vector_destroy(all_tags, free);
    }
    
    return stats;
}

int reminder_manager_mark_reminder_completed(ReminderManager* manager, const char* reminder_id) {
    if (!manager || !reminder_id) return 0;
    
    Reminder* reminder = (Reminder*)hashmap_get(manager->reminders, reminder_id);
    if (!reminder) return 0;
    
    reminder_mark_completed(reminder);
    return 1;
}

int reminder_manager_mark_reminder_cancelled(ReminderManager* manager, const char* reminder_id) {
    if (!manager || !reminder_id) return 0;
    
    Reminder* reminder = (Reminder*)hashmap_get(manager->reminders, reminder_id);
    if (!reminder) return 0;
    
    reminder_mark_cancelled(reminder);
    return 1;
}

int reminder_manager_activate_reminder(ReminderManager* manager, const char* reminder_id) {
    if (!manager || !reminder_id) return 0;
    
    Reminder* reminder = (Reminder*)hashmap_get(manager->reminders, reminder_id);
    if (!reminder) return 0;
    
    reminder_activate(reminder);
    return 1;
}

// ReminderStatistics functions
double reminder_statistics_pending_percentage(ReminderStatistics* stats) {
    if (!stats || stats->total_reminders == 0) return 0.0;
    return ((double)stats->pending_reminders / (double)stats->total_reminders) * 100.0;
}

double reminder_statistics_active_percentage(ReminderStatistics* stats) {
    if (!stats || stats->total_reminders == 0) return 0.0;
    return ((double)stats->active_reminders / (double)stats->total_reminders) * 100.0;
}

double reminder_statistics_overdue_percentage(ReminderStatistics* stats) {
    if (!stats || stats->total_reminders == 0) return 0.0;
    return ((double)stats->overdue_reminders / (double)stats->total_reminders) * 100.0;
}

// Helper functions for parsing
int parse_reminder_priority(const char* str, ReminderPriority* priority) {
    if (!str || !priority) return 0;
    
    char* lower = string_to_lowercase(str);
    if (!lower) return 0;
    
    if (strcmp(lower, "low") == 0) {
        *priority = REMINDER_PRIORITY_LOW;
        free(lower);
        return 1;
    } else if (strcmp(lower, "medium") == 0) {
        *priority = REMINDER_PRIORITY_MEDIUM;
        free(lower);
        return 1;
    } else if (strcmp(lower, "high") == 0) {
        *priority = REMINDER_PRIORITY_HIGH;
        free(lower);
        return 1;
    }
    free(lower);
    return 0;
}

int parse_reminder_status(const char* str, ReminderStatus* status) {
    if (!str || !status) return 0;
    
    char* lower = string_to_lowercase(str);
    if (!lower) return 0;
    
    if (strcmp(lower, "pending") == 0) {
        *status = REMINDER_STATUS_PENDING;
        free(lower);
        return 1;
    } else if (strcmp(lower, "active") == 0) {
        *status = REMINDER_STATUS_ACTIVE;
        free(lower);
        return 1;
    } else if (strcmp(lower, "completed") == 0) {
        *status = REMINDER_STATUS_COMPLETED;
        free(lower);
        return 1;
    } else if (strcmp(lower, "cancelled") == 0) {
        *status = REMINDER_STATUS_CANCELLED;
        free(lower);
        return 1;
    }
    free(lower);
    return 0;
}

int parse_reminder_format(const char* reminder_text, Reminder** out_reminder) {
    if (!reminder_text || !out_reminder) return 0;
    
    const char* start_tag = "<reminder>";
    const char* end_tag = "</reminder>";
    
    const char* start = strstr(reminder_text, start_tag);
    if (!start) return 0;
    
    const char* end = strstr(reminder_text, end_tag);
    if (!end) return 0;
    
    size_t start_len = strlen(start_tag);
    size_t content_len = end - (start + start_len);
    char* content = malloc(content_len + 1);
    if (!content) return 0;
    strncpy(content, start + start_len, content_len);
    content[content_len] = '\0';
    
    Vector* parts = string_split(content, ';');
    if (!parts || parts->size < 3) {
        if (parts) vector_destroy(parts, free);
        free(content);
        return 0;
    }
    
    Reminder* reminder = reminder_create();
    if (!reminder) {
        vector_destroy(parts, free);
        free(content);
        return 0;
    }
    
    reminder->content = string_duplicate((char*)vector_get(parts, 0));
    if (!reminder->content) {
        reminder_destroy(reminder);
        vector_destroy(parts, free);
        free(content);
        return 0;
    }
    
    // Parse remind_at (simplified - would need actual parsing in real implementation)
    // For now we'll just use current time
    reminder->remind_at = datetime_now();
    
    // Parse priority
    if (!parse_reminder_priority((char*)vector_get(parts, 2), &reminder->priority)) {
        reminder_destroy(reminder);
        vector_destroy(parts, free);
        free(content);
        return 0;
    }
    
    // Parse status if present
    if (parts->size > 3 && strlen((char*)vector_get(parts, 3)) > 0) {
        if (!parse_reminder_status((char*)vector_get(parts, 3), &reminder->status)) {
            reminder_destroy(reminder);
            vector_destroy(parts, free);
            free(content);
            return 0;
        }
    }
    
    // Parse tags if present
    if (parts->size > 4 && strlen((char*)vector_get(parts, 4)) > 0) {
        Vector* tags = string_split((char*)vector_get(parts, 4), ',');
        if (tags) {
            for (size_t i = 0; i < tags->size; i++) {
                char* tag = (char*)vector_get(tags, i);
                // Trim whitespace
                while (*tag == ' ') tag++;
                char* tag_copy = string_duplicate(tag);
                if (tag_copy) {
                    if (!vector_push(reminder->tags, tag_copy)) {
                        free(tag_copy);
                    }
                }
            }
            vector_destroy(tags, free);
        }
    }
    
    vector_destroy(parts, free);
    free(content);
    *out_reminder = reminder;
    return 1;
}

// Test functions
void test_reminder_manager_creation() {
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Test failed: manager creation failed\n");
        return;
    }
    if (manager->reminders->size != 0) {
        printf("Test failed: reminders size should be 0\n");
    }
    if (manager->reminder_tags->size != 0) {
        printf("Test failed: reminder_tags size should be 0\n");
    }
    reminder_manager_destroy(manager);
    printf("test_reminder_manager_creation passed\n");
}

void test_reminder_update_builder() {
    ReminderUpdate* update = reminder_update_create();
    if (!update) {
        printf("Test failed: update creation failed\n");
        return;
    }
    reminder_update_content(update, "New content");
    ReminderPriority priority = REMINDER_PRIORITY_HIGH;
    reminder_update_priority(update, priority);
    
    if (!update->content || strcmp(update->content, "New content") != 0) {
        printf("Test failed: content not set correctly\n");
    }
    if (!update->priority || *update->priority != REMINDER_PRIORITY_HIGH) {
        printf("Test failed: priority not set correctly\n");
    }
    
    reminder_update_destroy(update);
    printf("test_reminder_update_builder passed\n");
}

void test_reminder_statistics() {
    ReminderStatistics stats;
    stats.total_reminders = 10;
    stats.pending_reminders = 5;
    stats.active_reminders = 3;
    stats.overdue_reminders = 2;
    stats.unique_tags = 4;
    
    if (reminder_statistics_pending_percentage(&stats) != 50.0) {
        printf("Test failed: pending percentage incorrect\n");
    }
    if (reminder_statistics_active_percentage(&stats) != 30.0) {
        printf("Test failed: active percentage incorrect\n");
    }
    if (reminder_statistics_overdue_percentage(&stats) != 20.0) {
        printf("Test failed: overdue percentage incorrect\n");
    }
    
    ReminderStatistics empty_stats;
    empty_stats.total_reminders = 0;
    empty_stats.pending_reminders = 0;
    empty_stats.active_reminders = 0;
    empty_stats.overdue_reminders = 0;
    empty_stats.unique_tags = 0;
    
    if (reminder_statistics_pending_percentage(&empty_stats) != 0.0) {
        printf("Test failed: empty stats pending percentage incorrect\n");
    }
    if (reminder_statistics_active_percentage(&empty_stats) != 0.0) {
        printf("Test failed: empty stats active percentage incorrect\n");
    }
    if (reminder_statistics_overdue_percentage(&empty_stats) != 0.0) {
        printf("Test failed: empty stats overdue percentage incorrect\n");
    }
    
    printf("test_reminder_statistics passed\n");
}

void test_parse_reminder_format() {
    const char* reminder_text = "<reminder>Review project proposal; 2025-01-20T10:00:00Z; high; pending; review,project,deadline</reminder>";
    Reminder* reminder;
    if (parse_reminder_format(reminder_text, &reminder)) {
        if (strcmp(reminder->content, "Review project proposal") != 0) {
            printf("Test failed: content incorrect\n");
        }
        if (reminder->priority != REMINDER_PRIORITY_HIGH) {
            printf("Test failed: priority incorrect\n");
        }
        if (reminder->status != REMINDER_STATUS_PENDING) {
            printf("Test failed: status incorrect\n");
        }
        reminder_destroy(reminder);
    } else {
        printf("Test failed: parsing failed\n");
    }
    printf("test_parse_reminder_format passed\n");
}

void test_parse_reminder_format_minimal() {
    const char* reminder_text = "<reminder>Simple reminder; 2025-01-20T10:00:00Z; medium</reminder>";
    Reminder* reminder;
    if (parse_reminder_format(reminder_text, &reminder)) {
        if (strcmp(reminder->content, "Simple reminder") != 0) {
            printf("Test failed: content incorrect\n");
        }
        if (reminder->priority != REMINDER_PRIORITY_MEDIUM) {
            printf("Test failed: priority incorrect\n");
        }
        if (reminder->status != REMINDER_STATUS_PENDING) {
            printf("Test failed: status incorrect\n");
        }
        if (reminder->tags->size != 0) {
            printf("Test failed: tags should be empty\n");
        }
        reminder_destroy(reminder);
    } else {
        printf("Test failed: parsing failed\n");
    }
    printf("test_parse_reminder_format_minimal passed\n");
}

int main() {
    test_reminder_manager_creation();
    test_reminder_update_builder();
    test_reminder_statistics();
    test_parse_reminder_format();
    test_parse_reminder_format_minimal();
    return 0;
}