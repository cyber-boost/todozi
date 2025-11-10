#ifndef TODOZI_H
#define TODOZI_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct Error Error;
typedef struct HashMap HashMap;

// Error severity enum
typedef enum {
    ERROR_SEVERITY_LOW = 0,
    ERROR_SEVERITY_MEDIUM = 1,
    ERROR_SEVERITY_HIGH = 2,
    ERROR_SEVERITY_CRITICAL = 3
} ErrorSeverity;

// Error category enum
typedef enum {
    ERROR_CATEGORY_NETWORK = 0,
    ERROR_CATEGORY_DATABASE = 1,
    ERROR_CATEGORY_APPLICATION = 2,
    ERROR_CATEGORY_SYSTEM = 3
} ErrorCategory;

// Error structure
struct Error {
    char* id;
    char* title;
    char* description;
    ErrorSeverity severity;
    ErrorCategory category;
    char* source;
    char* context;
    char** tags;
    int tags_count;
    bool resolved;
    char* resolution;
    time_t created_at;
    time_t updated_at;
    time_t resolved_at;
};

// HashMap structure
struct HashMap {
    char** keys;
    Error** values;
    int size;
    int capacity;
};

// TodoziError enum
typedef enum {
    TODOZI_ERROR_TASK_NOT_FOUND = 0,
    TODOZI_ERROR_PROJECT_NOT_FOUND = 1,
    TODOZI_ERROR_FEELING_NOT_FOUND = 2,
    TODOZI_ERROR_INVALID_PRIORITY = 3,
    TODOZI_ERROR_INVALID_STATUS = 4,
    TODOZI_ERROR_INVALID_ASSIGNEE = 5,
    TODOZI_ERROR_INVALID_PROGRESS = 6,
    TODOZI_ERROR_VALIDATION_ERROR = 7,
    TODOZI_ERROR_STORAGE_ERROR = 8,
    TODOZI_ERROR_CONFIG_ERROR = 9,
    TODOZI_ERROR_IO_ERROR = 10,
    TODOZI_ERROR_JSON_ERROR = 11,
    TODOZI_ERROR_UUID_ERROR = 12,
    TODOZI_ERROR_CHRONO_ERROR = 13,
    TODOZI_ERROR_DIALOGUER_ERROR = 14,
    TODOZI_ERROR_HLX_ERROR = 15,
    TODOZI_ERROR_REQWEST_ERROR = 16,
    TODOZI_ERROR_DIR_ERROR = 17,
    TODOZI_ERROR_EMBEDDING_ERROR = 18,
    TODOZI_ERROR_API_ERROR = 19,
    TODOZI_ERROR_CANDLE_ERROR = 20,
    TODOZI_ERROR_NOT_IMPLEMENTED = 21
} TodoziErrorType;

// Result types
typedef struct {
    bool is_ok;
    union {
        char* ok_value;  // For string results
        struct {
            TodoziErrorType error_type;
            char* message;
        } err_value;
    } data;
} TodoziResultString;

typedef struct {
    bool is_ok;
    union {
        Error* ok_value;  // For Error* results
        struct {
            TodoziErrorType error_type;
            char* message;
        } err_value;
    } data;
} TodoziResultErrorPtr;

typedef struct {
    bool is_ok;
    union {
        void* ok_value;  // For void results (())
        struct {
            TodoziErrorType error_type;
            char* message;
        } err_value;
    } data;
} TodoziResultVoid;

// ErrorManager structure
typedef struct {
    HashMap* errors;
} ErrorManager;

// Function prototypes
ErrorManager* error_manager_new(void);
void error_manager_free(ErrorManager* manager);
TodoziResultString error_manager_create_error(ErrorManager* manager, Error* error);
TodoziResultVoid error_manager_resolve_error(ErrorManager* manager, const char* error_id, const char* resolution);
TodoziResultErrorPtr parse_error_format(const char* error_text);
Error* error_new(void);
void error_free(Error* error);
Error** error_manager_get_unresolved_errors(ErrorManager* manager, int* count);
void error_manager_free_unresolved_errors(Error** errors);

// Helper functions
HashMap* hashmap_new(void);
void hashmap_free(HashMap* map);
void hashmap_clear(HashMap* map, void (*free_val)(void*));
void hashmap_insert(HashMap* map, const char* key, Error* value);
Error* hashmap_get(HashMap* map, const char* key);
Error* hashmap_get_mut(HashMap* map, const char* key);
char* generate_uuid(void);
time_t get_current_time(void);
const char* error_severity_to_str(ErrorSeverity severity);
ErrorSeverity error_severity_from_str(const char* str);
const char* error_category_to_str(ErrorCategory category);
ErrorCategory error_category_from_str(const char* str);
const char* todozi_error_str(TodoziErrorType err);
void todozi_result_string_free(TodoziResultString* result);
void todozi_result_error_ptr_free(TodoziResultErrorPtr* result);
void todozi_result_void_free(TodoziResultVoid* result);

// Helper macros
#define TODOZI_OK_STRING(res, value) do { \
    (res).is_ok = true; \
    (res).data.ok_value = strdup(value); \
} while (0)

#define TODOZI_OK_ERROR_PTR(res, value) do { \
    (res).is_ok = true; \
    (res).data.ok_value = (value); \
} while (0)

#define TODOZI_OK_VOID(res) do { \
    (res).is_ok = true; \
    (res).data.ok_value = NULL; \
} while (0)

#define TODOZI_ERR_STRING(res, kind, msg) do { \
    (res).is_ok = false; \
    (res).data.err_value.error_type = (kind); \
    (res).data.err_value.message = strdup(msg); \
} while (0)

#define TODOZI_ERR_ERROR_PTR(res, kind, msg) do { \
    (res).is_ok = false; \
    (res).data.err_value.error_type = (kind); \
    (res).data.err_value.message = strdup(msg); \
} while (0)

#define TODOZI_ERR_VOID(res, kind, msg) do { \
    (res).is_ok = false; \
    (res).data.err_value.error_type = (kind); \
    (res).data.err_value.message = strdup(msg); \
} while (0)

#ifdef __cplusplus
}
#endif

#endif // TODOZI_H

// Implementation (todozi.h content is already above)
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>

// Implementation

ErrorManager* error_manager_new(void) {
    ErrorManager* manager = calloc(1, sizeof(ErrorManager));
    if (manager) {
        manager->errors = hashmap_new();
        if (!manager->errors) {
            free(manager);
            return NULL;
        }
    }
    return manager;
}

void error_manager_free(ErrorManager* manager) {
    if (!manager) return;
    hashmap_clear(manager->errors, (void (*)(void*))error_free);
    hashmap_free(manager->errors);
    free(manager);
}

TodoziResultString error_manager_create_error(ErrorManager* manager, Error* error) {
    TodoziResultString result = {0};
    
    if (!manager || !error) {
        TODOZI_ERR_STRING(result, TODOZI_ERROR_VALIDATION_ERROR, "Invalid parameters");
        return result;
    }
    
    // Generate UUID
    free(error->id);
    error->id = generate_uuid();
    if (!error->id) {
        TODOZI_ERR_STRING(result, TODOZI_ERROR_UUID_ERROR, "Failed to generate UUID");
        return result;
    }
    
    // Set timestamps
    error->created_at = get_current_time();
    error->updated_at = error->created_at;
    
    // Insert into hashmap
    hashmap_insert(manager->errors, error->id, error);
    
    TODOZI_OK_STRING(result, error->id);
    return result;
}

TodoziResultVoid error_manager_resolve_error(ErrorManager* manager, const char* error_id, const char* resolution) {
    TodoziResultVoid result = {0};
    
    if (!manager || !error_id) {
        TODOZI_ERR_VOID(result, TODOZI_ERROR_VALIDATION_ERROR, "Invalid parameters");
        return result;
    }
    
    Error* error = hashmap_get_mut(manager->errors, error_id);
    if (!error) {
        char msg[256];
        snprintf(msg, sizeof(msg), "Error %s not found", error_id);
        TODOZI_ERR_VOID(result, TODOZI_ERROR_VALIDATION_ERROR, msg);
        return result;
    }
    
    error->resolved = true;
    free(error->resolution);
    error->resolution = resolution ? strdup(resolution) : NULL;
    error->resolved_at = get_current_time();
    error->updated_at = error->resolved_at;
    
    TODOZI_OK_VOID(result);
    return result;
}

TodoziResultErrorPtr parse_error_format(const char* error_text) {
    TodoziResultErrorPtr result = {0};
    
    if (!error_text) {
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_VALIDATION_ERROR, "Invalid parameters");
        return result;
    }
    
    const char* start_tag = "<error>";
    const char* end_tag = "</error>";
    
    const char* start = strstr(error_text, start_tag);
    if (!start) {
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_VALIDATION_ERROR, "Missing <error> start tag");
        return result;
    }
    
    const char* end = strstr(error_text, end_tag);
    if (!end) {
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_VALIDATION_ERROR, "Missing </error> end tag");
        return result;
    }
    
    size_t content_len = end - (start + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (!content) {
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_STORAGE_ERROR, "Memory allocation failed");
        return result;
    }
    
    strncpy(content, start + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by ';'
    char** parts = malloc(sizeof(char*) * 10); // Assuming max 10 parts
    if (!parts) {
        free(content);
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_STORAGE_ERROR, "Memory allocation failed");
        return result;
    }
    
    int part_count = 0;
    char* saveptr1 = NULL;
    char* token = strtok_r(content, ";", &saveptr1);
    while (token && part_count < 10) {
        parts[part_count] = token;
        part_count++;
        token = strtok_r(NULL, ";", &saveptr1);
    }
    
    if (part_count < 5) {
        free(parts);
        free(content);
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_VALIDATION_ERROR, "Invalid error format: need at least 5 parts (title; description; severity; category; source)");
        return result;
    }
    
    // Create error object
    Error* error = error_new();
    if (!error) {
        free(parts);
        free(content);
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_STORAGE_ERROR, "Memory allocation failed");
        return result;
    }
    
    // Parse parts
    error->title = strdup(parts[0] ? parts[0] : "");
    error->description = strdup(parts[1] ? parts[1] : "");
    error->source = strdup(parts[4] ? parts[4] : "");
    
    // Parse severity
    ErrorSeverity severity = error_severity_from_str(parts[2]);
    if (severity == -1) {
        error_free(error);
        free(parts);
        free(content);
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_VALIDATION_ERROR, "Invalid error severity");
        return result;
    }
    error->severity = severity;
    
    // Parse category
    ErrorCategory category = error_category_from_str(parts[3]);
    if (category == -1) {
        error_free(error);
        free(parts);
        free(content);
        TODOZI_ERR_ERROR_PTR(result, TODOZI_ERROR_VALIDATION_ERROR, "Invalid error category");
        return result;
    }
    error->category = category;
    
    // Parse context
    if (part_count > 5 && parts[5] && strlen(parts[5]) > 0) {
        error->context = strdup(parts[5]);
    } else {
        error->context = NULL;
    }
    
    // Parse tags
    error->tags_count = 0;
    error->tags = NULL;
    if (part_count > 6 && parts[6] && strlen(parts[6]) > 0) {
        // Make a copy of the tags string to avoid strtok_r corrupting the original
        char* tags_str = strdup(parts[6]);
        if (tags_str) {
            // Count commas to determine number of tags
            int tag_count = 1;
            for (int i = 0; tags_str[i]; i++) {
                if (tags_str[i] == ',') tag_count++;
            }
            
            error->tags = malloc(sizeof(char*) * tag_count);
            if (error->tags) {
                error->tags_count = tag_count;
                
                char* saveptr2 = NULL;
                char* tag_token = strtok_r(tags_str, ",", &saveptr2);
                int tag_index = 0;
                while (tag_token && tag_index < tag_count) {
                    // Trim whitespace
                    while (*tag_token == ' ') tag_token++;
                    char* end = tag_token + strlen(tag_token) - 1;
                    while (end > tag_token && *end == ' ') end--;
                    *(end + 1) = '\0';
                    
                    error->tags[tag_index] = strdup(tag_token);
                    tag_index++;
                    tag_token = strtok_r(NULL, ",", &saveptr2);
                }
            }
            free(tags_str);
        }
    }
    
    error->resolved = false;
    error->resolution = NULL;
    error->created_at = get_current_time();
    error->updated_at = error->created_at;
    error->resolved_at = 0;
    
    free(parts);
    free(content);
    
    TODOZI_OK_ERROR_PTR(result, error);
    return result;
}

Error* error_new(void) {
    Error* error = calloc(1, sizeof(Error));
    return error;
}

void error_free(Error* error) {
    if (!error) return;
    
    free(error->id);
    free(error->title);
    free(error->description);
    free(error->source);
    free(error->context);
    free(error->resolution);
    
    for (int i = 0; i < error->tags_count; i++) {
        free(error->tags[i]);
    }
    free(error->tags);
    
    free(error);
}

Error** error_manager_get_unresolved_errors(ErrorManager* manager, int* count) {
    if (!manager || !count) {
        if (count) *count = 0;
        return NULL;
    }
    
    // Count unresolved errors first
    int unresolved_count = 0;
    for (int i = 0; i < manager->errors->size; i++) {
        if (manager->errors->values[i] && !manager->errors->values[i]->resolved) {
            unresolved_count++;
        }
    }
    
    if (unresolved_count == 0) {
        *count = 0;
        return NULL;
    }
    
    // Allocate array for unresolved errors
    Error** unresolved_errors = malloc(sizeof(Error*) * unresolved_count);
    if (!unresolved_errors) {
        *count = 0;
        return NULL;
    }
    
    // Populate array
    int index = 0;
    for (int i = 0; i < manager->errors->size && index < unresolved_count; i++) {
        if (manager->errors->values[i] && !manager->errors->values[i]->resolved) {
            unresolved_errors[index] = manager->errors->values[i];
            index++;
        }
    }
    
    *count = unresolved_count;
    return unresolved_errors;
}

void error_manager_free_unresolved_errors(Error** errors) {
    // Note: This function doesn't free the Error objects themselves,
    // just the array that holds the pointers
    free(errors);
}

HashMap* hashmap_new(void) {
    HashMap* map = calloc(1, sizeof(HashMap));
    if (map) {
        map->capacity = 16;
        map->keys = calloc(map->capacity, sizeof(char*));
        map->values = calloc(map->capacity, sizeof(Error*));
        if (!map->keys || !map->values) {
            free(map->keys);
            free(map->values);
            free(map);
            return NULL;
        }
    }
    return map;
}

void hashmap_free(HashMap* map) {
    if (!map) return;
    
    for (int i = 0; i < map->size; i++) {
        free(map->keys[i]);
    }
    
    free(map->keys);
    free(map->values);
    free(map);
}

void hashmap_clear(HashMap* map, void (*free_val)(void*)) {
    if (!map) return;
    
    for (int i = 0; i < map->size; i++) {
        free(map->keys[i]);
        if (free_val && map->values[i]) {
            free_val(map->values[i]);
        }
    }
    
    map->size = 0;
}

void hashmap_insert(HashMap* map, const char* key, Error* value) {
    if (!map || !key || !value) return;
    
    // Simple linear search for existing key
    for (int i = 0; i < map->size; i++) {
        if (strcmp(map->keys[i], key) == 0) {
            // Free the old value before overwriting
            error_free(map->values[i]);
            map->values[i] = value;
            return;
        }
    }
    
    // Add new key-value pair
    if (map->size >= map->capacity) {
        // Resize arrays
        int new_capacity = map->capacity * 2;
        char** new_keys = realloc(map->keys, sizeof(char*) * new_capacity);
        Error** new_values = realloc(map->values, sizeof(Error*) * new_capacity);
        
        if (!new_keys || !new_values) {
            // Handle realloc failure - clean up and return
            free(new_keys);
            free(new_values);
            return;
        }
        
        map->keys = new_keys;
        map->values = new_values;
        map->capacity = new_capacity;
        
        // Initialize new slots to NULL
        for (int i = map->size; i < map->capacity; i++) {
            map->keys[i] = NULL;
            map->values[i] = NULL;
        }
    }
    
    map->keys[map->size] = strdup(key);
    map->values[map->size] = value;
    map->size++;
}

Error* hashmap_get(HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    for (int i = 0; i < map->size; i++) {
        if (map->keys[i] && strcmp(map->keys[i], key) == 0) {
            return map->values[i];
        }
    }
    
    return NULL;
}

Error* hashmap_get_mut(HashMap* map, const char* key) {
    return hashmap_get(map, key);
}

char* generate_uuid(void) {
    // Simplified UUID generation - in practice, you'd use a proper UUID library
    char* uuid = malloc(37); // UUID string length + null terminator
    if (uuid) {
        // Generate a pseudo-UUID (not cryptographically secure)
        snprintf(uuid, 37, "%08x-%04x-%04x-%04x-%012llx",
                rand(), rand() & 0xFFFF, rand() & 0xFFFF,
                rand() & 0xFFFF, ((long long)rand() << 32) | rand());
    }
    return uuid;
}

time_t get_current_time(void) {
    return time(NULL);
}

const char* error_severity_to_str(ErrorSeverity severity) {
    switch (severity) {
        case ERROR_SEVERITY_LOW: return "low";
        case ERROR_SEVERITY_MEDIUM: return "medium";
        case ERROR_SEVERITY_HIGH: return "high";
        case ERROR_SEVERITY_CRITICAL: return "critical";
        default: return "unknown";
    }
}

ErrorSeverity error_severity_from_str(const char* str) {
    if (strcmp(str, "low") == 0) return ERROR_SEVERITY_LOW;
    if (strcmp(str, "medium") == 0) return ERROR_SEVERITY_MEDIUM;
    if (strcmp(str, "high") == 0) return ERROR_SEVERITY_HIGH;
    if (strcmp(str, "critical") == 0) return ERROR_SEVERITY_CRITICAL;
    return -1; // Invalid
}

const char* error_category_to_str(ErrorCategory category) {
    switch (category) {
        case ERROR_CATEGORY_NETWORK: return "network";
        case ERROR_CATEGORY_DATABASE: return "database";
        case ERROR_CATEGORY_APPLICATION: return "application";
        case ERROR_CATEGORY_SYSTEM: return "system";
        default: return "unknown";
    }
}

ErrorCategory error_category_from_str(const char* str) {
    if (strcmp(str, "network") == 0) return ERROR_CATEGORY_NETWORK;
    if (strcmp(str, "database") == 0) return ERROR_CATEGORY_DATABASE;
    if (strcmp(str, "application") == 0) return ERROR_CATEGORY_APPLICATION;
    if (strcmp(str, "system") == 0) return ERROR_CATEGORY_SYSTEM;
    return -1; // Invalid
}

const char* todozi_error_str(TodoziErrorType err) {
    switch (err) {
        case TODOZI_ERROR_TASK_NOT_FOUND: return "Task not found";
        case TODOZI_ERROR_PROJECT_NOT_FOUND: return "Project not found";
        case TODOZI_ERROR_FEELING_NOT_FOUND: return "Feeling not found";
        case TODOZI_ERROR_INVALID_PRIORITY: return "Invalid priority";
        case TODOZI_ERROR_INVALID_STATUS: return "Invalid status";
        case TODOZI_ERROR_INVALID_ASSIGNEE: return "Invalid assignee";
        case TODOZI_ERROR_INVALID_PROGRESS: return "Invalid progress";
        case TODOZI_ERROR_VALIDATION_ERROR: return "Validation error";
        case TODOZI_ERROR_STORAGE_ERROR: return "Storage error";
        case TODOZI_ERROR_CONFIG_ERROR: return "Configuration error";
        case TODOZI_ERROR_IO_ERROR: return "IO error";
        case TODOZI_ERROR_JSON_ERROR: return "JSON error";
        case TODOZI_ERROR_UUID_ERROR: return "UUID error";
        case TODOZI_ERROR_CHRONO_ERROR: return "Chrono error";
        case TODOZI_ERROR_DIALOGUER_ERROR: return "Dialoguer error";
        case TODOZI_ERROR_HLX_ERROR: return "HLX error";
        case TODOZI_ERROR_REQWEST_ERROR: return "Reqwest error";
        case TODOZI_ERROR_DIR_ERROR: return "Directory error";
        case TODOZI_ERROR_EMBEDDING_ERROR: return "Embedding error";
        case TODOZI_ERROR_API_ERROR: return "API error";
        case TODOZI_ERROR_CANDLE_ERROR: return "Candle error";
        case TODOZI_ERROR_NOT_IMPLEMENTED: return "Feature not implemented";
        default: return "Unknown error";
    }
}

void todozi_result_string_free(TodoziResultString* result) {
    if (!result) return;
    if (!result->is_ok && result->data.err_value.message) {
        free(result->data.err_value.message);
    } else if (result->is_ok && result->data.ok_value) {
        free(result->data.ok_value);
    }
}

void todozi_result_error_ptr_free(TodoziResultErrorPtr* result) {
    if (!result) return;
    if (!result->is_ok && result->data.err_value.message) {
        free(result->data.err_value.message);
    } else if (result->is_ok && result->data.ok_value) {
        error_free(result->data.ok_value);
    }
}

void todozi_result_void_free(TodoziResultVoid* result) {
    if (!result) return;
    if (!result->is_ok && result->data.err_value.message) {
        free(result->data.err_value.message);
    }
}

#ifdef TEST_ERROR_PARSING
#include <assert.h>

int main(void) {
    const char* error_text = "<error>Database connection failed; Unable to connect to PostgreSQL database; critical; network; database-service; Connection timeout after 30 seconds; database,postgres,connection</error>";
    
    TodoziResultErrorPtr result = parse_error_format(error_text);
    
    if (result.is_ok) {
        Error* error = result.data.ok_value;
        printf("Title: %s\n", error->title);
        printf("Description: %s\n", error->description);
        printf("Severity: %s\n", error_severity_to_str(error->severity));
        printf("Category: %s\n", error_category_to_str(error->category));
        printf("Source: %s\n", error->source);
        printf("Context: %s\n", error->context ? error->context : "NULL");
        printf("Tags count: %d\n", error->tags_count);
        
        for (int i = 0; i < error->tags_count; i++) {
            printf("Tag %d: %s\n", i, error->tags[i]);
        }
        
        printf("Test passed!\n");
        todozi_result_error_ptr_free(&result);
    } else {
        printf("Test failed: %s\n", result.data.err_value.message);
        todozi_result_error_ptr_free(&result);
        return 1;
    }
    
    // Test ErrorManager
    ErrorManager* manager = error_manager_new();
    if (!manager) {
        printf("Failed to create ErrorManager\n");
        return 1;
    }
    
    Error* test_error = error_new();
    if (!test_error) {
        printf("Failed to create test error\n");
        error_manager_free(manager);
        return 1;
    }
    
    test_error->title = strdup("Test Error");
    test_error->description = strdup("This is a test error");
    test_error->severity = ERROR_SEVERITY_HIGH;
    test_error->category = ERROR_CATEGORY_APPLICATION;
    test_error->source = strdup("test");
    
    TodoziResultString create_result = error_manager_create_error(manager, test_error);
    if (!create_result.is_ok) {
        printf("Failed to create error in manager: %s\n", create_result.data.err_value.message);
        todozi_result_string_free(&create_result);
        error_manager_free(manager);
        return 1;
    }
    
    printf("Created error with ID: %s\n", create_result.data.ok_value);
    
    // Test getting unresolved errors
    int count;
    Error** unresolved = error_manager_get_unresolved_errors(manager, &count);
    printf("Unresolved errors count: %d\n", count);
    error_manager_free_unresolved_errors(unresolved);
    
    // Test resolving error
    TodoziResultVoid resolve_result = error_manager_resolve_error(manager, create_result.data.ok_value, "Fixed");
    if (!resolve_result.is_ok) {
        printf("Failed to resolve error: %s\n", resolve_result.data.err_value.message);
        todozi_result_string_free(&create_result);
        todozi_result_void_free(&resolve_result);
        error_manager_free(manager);
        return 1;
    }
    
    printf("Error resolved successfully\n");
    
    // Cleanup
    todozi_result_string_free(&create_result);
    todozi_result_void_free(&resolve_result);
    error_manager_free(manager);
    
    return 0;
}
#endif