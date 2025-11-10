#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>
#include <errno.h>
#include <time.h>
#include <uuid/uuid.h>

// Forward declarations (removed typedef conflicts)
struct ApiKey;
struct ApiKeyCollection;
struct Result;

// Error types
typedef enum {
    TODOZI_ERROR_NONE,
    TODOZI_ERROR_IO,
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_JSON
} TodoziErrorType;

typedef struct {
    TodoziErrorType type;
    char* message;
} TodoziError;

// struct Result type
struct Result {
    void* data;
    TodoziError* error;
};

// Vector for storing API keys
typedef struct {
    struct ApiKey** keys;
    size_t size;
    size_t capacity;
} ApiKeyVector;

// API Key structure
struct ApiKey {
    char* user_id;
    char* public_key;
    char* private_key;
    bool is_active;
    bool admin;
};

// API Key Collection structure
struct ApiKeyCollection {
    ApiKeyVector keys;
};

// Function declarations
static struct Result* get_storage_dir(void);
struct Result* save_api_key_collection(const struct ApiKeyCollection* collection);
struct Result* load_api_key_collection(void);
struct Result* create_api_key(void);
struct Result* create_api_key_with_user_id(const char* user_id);
struct Result* get_api_key(const char* user_id);
struct Result* get_api_key_by_public(const char* public_key);
struct Result* list_api_keys(void);
struct Result* list_active_api_keys(void);
struct Result* check_api_key_auth(const char* public_key, const char* private_key);
struct Result* deactivate_api_key(const char* user_id);
struct Result* activate_api_key(const char* user_id);
struct Result* remove_api_key(const char* user_id);

// Helper functions
static char* generate_uuid(void);
char* generate_random_string(size_t length);
bool file_exists(const char* path);
static char* join_paths(const char* path1, const char* path2);
char* read_file(const char* path);
bool write_file(const char* path, const char* content);
static bool create_directory(const char* path);

// Vector functions
ApiKeyVector* api_key_vector_new(void);
void api_key_vector_free(ApiKeyVector* vector);
void api_key_vector_push(ApiKeyVector* vector, struct ApiKey* key);
struct ApiKey* api_key_vector_get(ApiKeyVector* vector, size_t index);
size_t api_key_vector_size(const ApiKeyVector* vector);

// API Key functions
struct ApiKey* api_key_new(void);
struct ApiKey* api_key_with_user_id(const char* user_id);
void api_key_free(struct ApiKey* key);
bool api_key_is_admin(const struct ApiKey* key, const char* public_key, const char* private_key);
bool api_key_matches(const struct ApiKey* key, const char* public_key, const char* private_key);
struct ApiKey* api_key_clone(const struct ApiKey* key);

// API Key Collection functions
struct ApiKeyCollection* api_key_collection_new(void);
void api_key_collection_free(struct ApiKeyCollection* collection);
void api_key_collection_add_key(struct ApiKeyCollection* collection, struct ApiKey* key);
struct ApiKey* api_key_collection_get_key(const struct ApiKeyCollection* collection, const char* user_id);
struct ApiKey* api_key_collection_get_key_by_public(const struct ApiKeyCollection* collection, const char* public_key);
ApiKeyVector* api_key_collection_get_all_keys(const struct ApiKeyCollection* collection);
ApiKeyVector* api_key_collection_get_active_keys(const struct ApiKeyCollection* collection);
bool api_key_collection_deactivate_key(struct ApiKeyCollection* collection, const char* user_id);
bool api_key_collection_activate_key(struct ApiKeyCollection* collection, const char* user_id);
struct ApiKey* api_key_collection_remove_key(struct ApiKeyCollection* collection, const char* user_id);

// struct Result functions
struct Result* result_ok(void* data);
struct Result* result_error(TodoziErrorType type, const char* message);
void result_free(struct Result* result);
bool result_is_ok(const struct Result* result);

// Implementation

// Helper function to create a new result with data
struct Result* result_ok(void* data) {
    struct Result* result = malloc(sizeof(struct Result));
    if (!result) return NULL;
    result->data = data;
    result->error = NULL;
    return result;
}

// Helper function to create a new error result
struct Result* result_error(TodoziErrorType type, const char* message) {
    struct Result* result = malloc(sizeof(struct Result));
    if (!result) return NULL;
    result->data = NULL;
    result->error = malloc(sizeof(TodoziError));
    if (!result->error) {
        free(result);
        return NULL;
    }
    result->error->type = type;
    result->error->message = message ? strdup(message) : NULL;
    return result;
}

// Check if result is OK
bool result_is_ok(const struct Result* result) {
    return result && result->error == NULL;
}

// Free result
void result_free(struct Result* result) {
    if (!result) return;
    if (result->error) {
        free(result->error->message);
        free(result->error);
    }
    free(result);
}

// Create a new API key vector
ApiKeyVector* api_key_vector_new(void) {
    ApiKeyVector* vector = malloc(sizeof(ApiKeyVector));
    if (!vector) return NULL;
    vector->keys = NULL;
    vector->size = 0;
    vector->capacity = 0;
    return vector;
}

// Free API key vector
void api_key_vector_free(ApiKeyVector* vector) {
    if (!vector) return;
    for (size_t i = 0; i < vector->size; i++) {
        api_key_free(vector->keys[i]);
    }
    free(vector->keys);
    free(vector);
}

// Add key to vector
void api_key_vector_push(ApiKeyVector* vector, struct ApiKey* key) {
    if (!vector || !key) return;
    if (vector->size >= vector->capacity) {
        size_t new_capacity = vector->capacity == 0 ? 8 : vector->capacity * 2;
        // Check for overflow
        if (new_capacity < vector->capacity) {
            return; // Overflow detected
        }
        struct ApiKey** new_keys = realloc(vector->keys, new_capacity * sizeof(struct ApiKey*));
        if (!new_keys) return;
        vector->keys = new_keys;
        vector->capacity = new_capacity;
    }
    vector->keys[vector->size++] = key;
}

// Get key from vector by index
struct ApiKey* api_key_vector_get(ApiKeyVector* vector, size_t index) {
    if (!vector || index >= vector->size) return NULL;
    return vector->keys[index];
}

// Get vector size
size_t api_key_vector_size(const ApiKeyVector* vector) {
    return vector ? vector->size : 0;
}

// Create a new API key
struct ApiKey* api_key_new(void) {
    struct ApiKey* key = malloc(sizeof(struct ApiKey));
    if (!key) return NULL;
    
    key->user_id = generate_uuid();
    key->public_key = generate_random_string(32);
    key->private_key = generate_random_string(64);
    
    // Check if any allocation failed
    if (!key->user_id || !key->public_key || !key->private_key) {
        api_key_free(key);
        return NULL;
    }
    
    key->is_active = true;
    key->admin = false;
    return key;
}

// Create API key with specific user ID
struct ApiKey* api_key_with_user_id(const char* user_id) {
    if (!user_id) return NULL;
    struct ApiKey* key = malloc(sizeof(struct ApiKey));
    if (!key) return NULL;
    
    key->user_id = strdup(user_id);
    key->public_key = generate_random_string(32);
    key->private_key = generate_random_string(64);
    
    // Check if any allocation failed
    if (!key->user_id || !key->public_key || !key->private_key) {
        api_key_free(key);
        return NULL;
    }
    
    key->is_active = true;
    key->admin = false;
    return key;
}

// Free API key
void api_key_free(struct ApiKey* key) {
    if (!key) return;
    free(key->user_id);
    free(key->public_key);
    free(key->private_key);
    free(key);
}

// Check if API key is admin
bool api_key_is_admin(const struct ApiKey* key, const char* public_key, const char* private_key) {
    if (!key || !public_key || !private_key) return false;
    return key->admin && 
           strcmp(key->public_key, public_key) == 0 && 
           strcmp(key->private_key, private_key) == 0;
}

// Check if API key matches
bool api_key_matches(const struct ApiKey* key, const char* public_key, const char* private_key) {
    if (!key || !public_key) return false;
    if (private_key) {
        return strcmp(key->public_key, public_key) == 0 && 
               strcmp(key->private_key, private_key) == 0;
    }
    return strcmp(key->public_key, public_key) == 0;
}

// Clone API key
struct ApiKey* api_key_clone(const struct ApiKey* key) {
    if (!key) return NULL;
    struct ApiKey* clone = malloc(sizeof(struct ApiKey));
    if (!clone) return NULL;
    clone->user_id = strdup(key->user_id);
    clone->public_key = strdup(key->public_key);
    clone->private_key = strdup(key->private_key);
    clone->is_active = key->is_active;
    clone->admin = key->admin;
    return clone;
}

// Create new API key collection
struct ApiKeyCollection* api_key_collection_new(void) {
    struct ApiKeyCollection* collection = malloc(sizeof(struct ApiKeyCollection));
    if (!collection) return NULL;
    ApiKeyVector* vector = api_key_vector_new();
    if (!vector) {
        free(collection);
        return NULL;
    }
    collection->keys = *vector;
    free(vector); // Free the container, but keep the struct data
    return collection;
}

// Free API key collection
void api_key_collection_free(struct ApiKeyCollection* collection) {
    if (!collection) return;
    // Create a temporary pointer to the vector for proper cleanup
    ApiKeyVector* vector = &collection->keys;
    for (size_t i = 0; i < vector->size; i++) {
        api_key_free(vector->keys[i]);
    }
    free(vector->keys);
    free(collection);
}

// Add key to collection
void api_key_collection_add_key(struct ApiKeyCollection* collection, struct ApiKey* key) {
    if (!collection || !key) return;
    api_key_vector_push(&collection->keys, key);
}

// Get key by user ID
struct ApiKey* api_key_collection_get_key(const struct ApiKeyCollection* collection, const char* user_id) {
    if (!collection || !user_id) return NULL;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get((ApiKeyVector*)&collection->keys, i);
        if (key && strcmp(key->user_id, user_id) == 0) {
            return key;
        }
    }
    return NULL;
}

// Get key by public key
struct ApiKey* api_key_collection_get_key_by_public(const struct ApiKeyCollection* collection, const char* public_key) {
    if (!collection || !public_key) return NULL;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get((ApiKeyVector*)&collection->keys, i);
        if (key && strcmp(key->public_key, public_key) == 0) {
            return key;
        }
    }
    return NULL;
}

// Get all keys
ApiKeyVector* api_key_collection_get_all_keys(const struct ApiKeyCollection* collection) {
    if (!collection) return NULL;
    ApiKeyVector* result = api_key_vector_new();
    if (!result) return NULL;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get((ApiKeyVector*)&collection->keys, i);
        if (key) {
            api_key_vector_push(result, api_key_clone(key));
        }
    }
    return result;
}

// Get active keys
ApiKeyVector* api_key_collection_get_active_keys(const struct ApiKeyCollection* collection) {
    if (!collection) return NULL;
    ApiKeyVector* result = api_key_vector_new();
    if (!result) return NULL;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get((ApiKeyVector*)&collection->keys, i);
        if (key && key->is_active) {
            api_key_vector_push(result, api_key_clone(key));
        }
    }
    return result;
}

// Deactivate key
bool api_key_collection_deactivate_key(struct ApiKeyCollection* collection, const char* user_id) {
    if (!collection || !user_id) return false;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get(&collection->keys, i);
        if (key && strcmp(key->user_id, user_id) == 0) {
            key->is_active = false;
            return true;
        }
    }
    return false;
}

// Activate key
bool api_key_collection_activate_key(struct ApiKeyCollection* collection, const char* user_id) {
    if (!collection || !user_id) return false;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get(&collection->keys, i);
        if (key && strcmp(key->user_id, user_id) == 0) {
            key->is_active = true;
            return true;
        }
    }
    return false;
}

// Remove key
struct ApiKey* api_key_collection_remove_key(struct ApiKeyCollection* collection, const char* user_id) {
    if (!collection || !user_id) return NULL;
    for (size_t i = 0; i < collection->keys.size; i++) {
        struct ApiKey* key = api_key_vector_get(&collection->keys, i);
        if (key && strcmp(key->user_id, user_id) == 0) {
            // Shift elements
            for (size_t j = i; j < collection->keys.size - 1; j++) {
                collection->keys.keys[j] = collection->keys.keys[j + 1];
            }
            collection->keys.size--;
            return key;
        }
    }
    return NULL;
}

// Get storage directory
static struct Result* get_storage_dir(void) {
    char* home = getenv("HOME");
    if (!home) {
        return result_error(TODOZI_ERROR_IO, "HOME environment variable not set");
    }
    size_t path_len = strlen(home) + strlen("/.todozi") + 1;
    char* storage_dir = malloc(path_len);
    if (!storage_dir) {
        return result_error(TODOZI_ERROR_IO, "Memory allocation failed");
    }
    snprintf(storage_dir, path_len, "%s/.todozi", home);
    return result_ok(storage_dir);
}

// Join two paths
static char* join_paths(const char* path1, const char* path2) {
    if (!path1 || !path2) return NULL;
    
    size_t len1 = strlen(path1);
    size_t len2 = strlen(path2);
    bool path1_ends_slash = (len1 > 0 && path1[len1 - 1] == '/');
    bool path2_starts_slash = (len2 > 0 && path2[0] == '/');
    
    size_t total_len = len1 + len2 + 2; // +2 for separator and null terminator
    if (path1_ends_slash || path2_starts_slash) {
        total_len--; // One less separator needed
    }
    
    char* result = malloc(total_len);
    if (!result) return NULL;
    
    if (path1_ends_slash || path2_starts_slash) {
        snprintf(result, total_len, "%s%s", path1, path2);
    } else {
        snprintf(result, total_len, "%s/%s", path1, path2);
    }
    
    return result;
}

// Check if file exists
bool file_exists(const char* path) {
    if (!path) return false;
    return access(path, F_OK) == 0;
}

// Read file content
char* read_file(const char* path) {
    if (!path) return NULL;
    FILE* file = fopen(path, "rb");
    if (!file) return NULL;
    
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    
    long length = ftell(file);
    if (length < 0) {
        fclose(file);
        return NULL;
    }
    
    if (fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    
    // Handle empty files
    if (length == 0) {
        char* content = malloc(1);
        if (content) {
            content[0] = '\0';
        }
        fclose(file);
        return content;
    }
    
    char* content = malloc((size_t)length + 1);
    if (!content) {
        fclose(file);
        return NULL;
    }
    
    size_t read = fread(content, 1, (size_t)length, file);
    if (read != (size_t)length && ferror(file)) {
        free(content);
        fclose(file);
        return NULL;
    }
    
    content[read] = '\0';
    fclose(file);
    return content;
}

// Write content to file
bool write_file(const char* path, const char* content) {
    if (!path || !content) return false;
    FILE* file = fopen(path, "wb");
    if (!file) return false;
    
    size_t len = strlen(content);
    size_t written = fwrite(content, 1, len, file);
    bool success = (written == len && ferror(file) == 0);
    
    if (success) {
        success = (fflush(file) == 0);
    }
    
    fclose(file);
    return success;
}

// Create directory
static bool create_directory(const char* path) {
    if (!path) return false;
    return mkdir(path, 0755) == 0 || errno == EEXIST;
}

// Save API key collection
struct Result* save_api_key_collection(const struct ApiKeyCollection* collection) {
    if (!collection) {
        return result_error(TODOZI_ERROR_VALIDATION, "Collection is NULL");
    }
    
    struct Result* storage_result = get_storage_dir();
    if (!result_is_ok(storage_result)) {
        return storage_result;
    }
    
    char* storage_dir = (char*)storage_result->data;
    char* api_dir = join_paths(storage_dir, "api");
    if (!api_dir) {
        result_free(storage_result);
        free(storage_dir);
        return result_error(TODOZI_ERROR_IO, "Failed to create API directory path");
    }
    
    if (!create_directory(api_dir)) {
        result_free(storage_result);
        free(storage_dir);
        free(api_dir);
        return result_error(TODOZI_ERROR_IO, "Failed to create API directory");
    }
    
    char* file_path = join_paths(api_dir, "api_keys.json");
    if (!file_path) {
        result_free(storage_result);
        free(storage_dir);
        free(api_dir);
        return result_error(TODOZI_ERROR_IO, "Failed to create file path");
    }
    
    // In a real implementation, you would serialize the collection to JSON here
    // For this example, we'll just write a placeholder
    const char* content = "{\n  \"keys\": []\n}";
    
    if (!write_file(file_path, content)) {
        result_free(storage_result);
        free(storage_dir);
        free(api_dir);
        free(file_path);
        return result_error(TODOZI_ERROR_IO, "Failed to write API keys file");
    }
    
    free(storage_dir);
    free(api_dir);
    free(file_path);
    result_free(storage_result);
    return result_ok(NULL);
}

// Load API key collection
struct Result* load_api_key_collection(void) {
    struct Result* storage_result = get_storage_dir();
    if (!result_is_ok(storage_result)) {
        return storage_result;
    }
    
    char* storage_dir = (char*)storage_result->data;
    char* api_dir = join_paths(storage_dir, "api");
    if (!api_dir) {
        result_free(storage_result);
        free(storage_dir);
        return result_error(TODOZI_ERROR_IO, "Failed to create API directory path");
    }
    
    char* file_path = join_paths(api_dir, "api_keys.json");
    if (!file_path) {
        result_free(storage_result);
        free(storage_dir);
        free(api_dir);
        return result_error(TODOZI_ERROR_IO, "Failed to create file path");
    }
    
    if (!file_exists(file_path)) {
        free(storage_dir);
        free(api_dir);
        free(file_path);
        result_free(storage_result);
        return result_ok(api_key_collection_new());
    }
    
    char* content = read_file(file_path);
    if (!content) {
        result_free(storage_result);
        free(storage_dir);
        free(api_dir);
        free(file_path);
        return result_error(TODOZI_ERROR_IO, "Failed to read API keys file");
    }
    
    // In a real implementation, you would deserialize the JSON content here
    // For this example, we'll just create an empty collection
    struct ApiKeyCollection* collection = api_key_collection_new();
    
    free(content);
    free(storage_dir);
    free(api_dir);
    free(file_path);
    result_free(storage_result);
    return result_ok(collection);
}

// Create API key
struct Result* create_api_key(void) {
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    struct ApiKey* api_key = api_key_new();
    if (!api_key) {
        api_key_collection_free(collection);
        result_free(load_result);
        return result_error(TODOZI_ERROR_IO, "Failed to create API key");
    }
    
    struct ApiKey* cloned_key = api_key_clone(api_key);
    if (!cloned_key) {
        api_key_collection_free(collection);
        result_free(load_result);
        api_key_free(api_key);
        return result_error(TODOZI_ERROR_IO, "Failed to clone API key");
    }
    
    api_key_collection_add_key(collection, cloned_key);
    
    struct Result* save_result = save_api_key_collection(collection);
    if (!result_is_ok(save_result)) {
        api_key_collection_free(collection);
        result_free(load_result);
        result_free(save_result);
        api_key_free(api_key);
        return save_result;
    }
    
    api_key_collection_free(collection);
    result_free(load_result);
    result_free(save_result);
    return result_ok(api_key);
}

// Create API key with user ID
struct Result* create_api_key_with_user_id(const char* user_id) {
    if (!user_id) {
        return result_error(TODOZI_ERROR_VALIDATION, "User ID is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    struct ApiKey* api_key = api_key_with_user_id(user_id);
    if (!api_key) {
        api_key_collection_free(collection);
        result_free(load_result);
        return result_error(TODOZI_ERROR_IO, "Failed to create API key");
    }
    
    struct ApiKey* cloned_key = api_key_clone(api_key);
    if (!cloned_key) {
        api_key_collection_free(collection);
        result_free(load_result);
        api_key_free(api_key);
        return result_error(TODOZI_ERROR_IO, "Failed to clone API key");
    }
    
    api_key_collection_add_key(collection, cloned_key);
    
    struct Result* save_result = save_api_key_collection(collection);
    if (!result_is_ok(save_result)) {
        api_key_collection_free(collection);
        result_free(load_result);
        result_free(save_result);
        api_key_free(api_key);
        return save_result;
    }
    
    api_key_collection_free(collection);
    result_free(load_result);
    result_free(save_result);
    return result_ok(api_key);
}

// Get API key by user ID
struct Result* get_api_key(const char* user_id) {
    if (!user_id) {
        return result_error(TODOZI_ERROR_VALIDATION, "User ID is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    struct ApiKey* key = api_key_collection_get_key(collection, user_id);
    
    if (!key) {
        api_key_collection_free(collection);
        result_free(load_result);
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "API key not found: %s", user_id);
        return result_error(TODOZI_ERROR_VALIDATION, error_msg);
    }
    
    struct ApiKey* cloned_key = api_key_clone(key);
    api_key_collection_free(collection);
    result_free(load_result);
    return result_ok(cloned_key);
}

// Get API key by public key
struct Result* get_api_key_by_public(const char* public_key) {
    if (!public_key) {
        return result_error(TODOZI_ERROR_VALIDATION, "Public key is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    struct ApiKey* key = api_key_collection_get_key_by_public(collection, public_key);
    
    if (!key) {
        api_key_collection_free(collection);
        result_free(load_result);
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "API key not found for public key: %s", public_key);
        return result_error(TODOZI_ERROR_VALIDATION, error_msg);
    }
    
    struct ApiKey* cloned_key = api_key_clone(key);
    api_key_collection_free(collection);
    result_free(load_result);
    return result_ok(cloned_key);
}

// List all API keys
struct Result* list_api_keys(void) {
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    ApiKeyVector* keys = api_key_collection_get_all_keys(collection);
    
    api_key_collection_free(collection);
    result_free(load_result);
    return result_ok(keys);
}

// List active API keys
struct Result* list_active_api_keys(void) {
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    ApiKeyVector* keys = api_key_collection_get_active_keys(collection);
    
    api_key_collection_free(collection);
    result_free(load_result);
    return result_ok(keys);
}

// Check API key authentication
struct Result* check_api_key_auth(const char* public_key, const char* private_key) {
    if (!public_key) {
        return result_error(TODOZI_ERROR_VALIDATION, "Public key is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    struct ApiKey* api_key = api_key_collection_get_key_by_public(collection, public_key);
    
    if (!api_key) {
        api_key_collection_free(collection);
        result_free(load_result);
        return result_error(TODOZI_ERROR_VALIDATION, "Invalid API key");
    }
    
    // Check if key is active
    if (!api_key->is_active) {
        api_key_collection_free(collection);
        result_free(load_result);
        return result_error(TODOZI_ERROR_VALIDATION, "API key is not active");
    }
    
    bool is_admin = false;
    if (private_key) {
        is_admin = api_key_is_admin(api_key, public_key, private_key);
        // Also verify the key matches
        if (!api_key_matches(api_key, public_key, private_key)) {
            api_key_collection_free(collection);
            result_free(load_result);
            return result_error(TODOZI_ERROR_VALIDATION, "Invalid API key credentials");
        }
    } else {
        // Public key only - not admin
        if (!api_key_matches(api_key, public_key, NULL)) {
            api_key_collection_free(collection);
            result_free(load_result);
            return result_error(TODOZI_ERROR_VALIDATION, "Invalid API key");
        }
    }
    
    char* user_id = strdup(api_key->user_id);
    api_key_collection_free(collection);
    result_free(load_result);
    
    if (!user_id) {
        return result_error(TODOZI_ERROR_IO, "Memory allocation failed");
    }
    
    // Create a pair (user_id, is_admin) - in C we'll use a simple struct
    typedef struct {
        char* user_id;
        bool is_admin;
    } AuthResult;
    
    AuthResult* auth_result = malloc(sizeof(AuthResult));
    if (!auth_result) {
        free(user_id);
        return result_error(TODOZI_ERROR_IO, "Memory allocation failed");
    }
    
    auth_result->user_id = user_id;
    auth_result->is_admin = is_admin;
    
    return result_ok(auth_result);
}

// Deactivate API key
struct Result* deactivate_api_key(const char* user_id) {
    if (!user_id) {
        return result_error(TODOZI_ERROR_VALIDATION, "User ID is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    if (!api_key_collection_deactivate_key(collection, user_id)) {
        api_key_collection_free(collection);
        result_free(load_result);
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "API key not found: %s", user_id);
        return result_error(TODOZI_ERROR_VALIDATION, error_msg);
    }
    
    struct Result* save_result = save_api_key_collection(collection);
    if (!result_is_ok(save_result)) {
        api_key_collection_free(collection);
        result_free(load_result);
        // Create a new error result with the same error message
        struct Result* error_result = result_error(save_result->error->type, save_result->error->message);
        result_free(save_result);
        return error_result;
    }
    
    api_key_collection_free(collection);
    result_free(load_result);
    result_free(save_result);
    return result_ok(NULL);
}

// Activate API key
struct Result* activate_api_key(const char* user_id) {
    if (!user_id) {
        return result_error(TODOZI_ERROR_VALIDATION, "User ID is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    if (!api_key_collection_activate_key(collection, user_id)) {
        api_key_collection_free(collection);
        result_free(load_result);
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "API key not found: %s", user_id);
        return result_error(TODOZI_ERROR_VALIDATION, error_msg);
    }
    
    struct Result* save_result = save_api_key_collection(collection);
    if (!result_is_ok(save_result)) {
        api_key_collection_free(collection);
        result_free(load_result);
        // Create a new error result with the same error message
        struct Result* error_result = result_error(save_result->error->type, save_result->error->message);
        result_free(save_result);
        return error_result;
    }
    
    api_key_collection_free(collection);
    result_free(load_result);
    result_free(save_result);
    return result_ok(NULL);
}

// Remove API key
struct Result* remove_api_key(const char* user_id) {
    if (!user_id) {
        return result_error(TODOZI_ERROR_VALIDATION, "User ID is NULL");
    }
    
    struct Result* load_result = load_api_key_collection();
    if (!result_is_ok(load_result)) {
        return load_result;
    }
    
    struct ApiKeyCollection* collection = (struct ApiKeyCollection*)load_result->data;
    struct ApiKey* key = api_key_collection_remove_key(collection, user_id);
    
    if (!key) {
        api_key_collection_free(collection);
        result_free(load_result);
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "API key not found: %s", user_id);
        return result_error(TODOZI_ERROR_VALIDATION, error_msg);
    }
    
    struct Result* save_result = save_api_key_collection(collection);
    if (!result_is_ok(save_result)) {
        api_key_free(key);
        api_key_collection_free(collection);
        result_free(load_result);
        // Create a new error result with the same error message
        struct Result* error_result = result_error(save_result->error->type, save_result->error->message);
        result_free(save_result);
        return error_result;
    }
    
    api_key_collection_free(collection);
    result_free(load_result);
    result_free(save_result);
    return result_ok(key);
}

// Helper function to generate UUID
static char* generate_uuid(void) {
    // TODO: Use UUID when linking issues are resolved
    char* uuid_str = malloc(37); // UUID string length + null terminator
    if (!uuid_str) return NULL;

    sprintf(uuid_str, "temp-uuid-%ld", (long)time(NULL));

    return uuid_str;
}

// Helper function to generate random string
char* generate_random_string(size_t length) {
    if (length == 0) {
        char* str = malloc(1);
        if (str) str[0] = '\0';
        return str;
    }
    
    char* str = malloc(length + 1);
    if (!str) return NULL;
    
    const char charset[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const size_t charset_size = sizeof(charset) - 1; // Exclude null terminator
    
    // Use /dev/urandom for cryptographically secure random bytes
    FILE* urandom = fopen("/dev/urandom", "rb");
    if (urandom) {
        unsigned char* random_bytes = malloc(length);
        if (random_bytes) {
            if (fread(random_bytes, 1, length, urandom) == length) {
                for (size_t i = 0; i < length; i++) {
                    str[i] = charset[random_bytes[i] % charset_size];
                }
                free(random_bytes);
                fclose(urandom);
                str[length] = '\0';
                return str;
            }
            free(random_bytes);
        }
        fclose(urandom);
    }
    
    // Fallback to time-based seeding if /dev/urandom fails
    static bool seeded = false;
    if (!seeded) {
        srand((unsigned int)time(NULL) ^ (unsigned int)getpid());
        seeded = true;
    }
    
    for (size_t i = 0; i < length; i++) {
        str[i] = charset[rand() % charset_size];
    }
    str[length] = '\0';
    return str;
}
