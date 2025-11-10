#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdarg.h>
#include <regex.h>

// Forward declarations
typedef struct ToolParameter ToolParameter;
typedef struct ToolDefinition ToolDefinition;
typedef struct ToolResult ToolResult;
typedef struct ToolError ToolError;
typedef struct Tool Tool;
typedef struct ToolRegistry ToolRegistry;

// Enum definitions
typedef enum {
    RESOURCE_LOCK_FILESYSTEM_WRITE,
    RESOURCE_LOCK_FILESYSTEM_READ,
    RESOURCE_LOCK_GIT,
    RESOURCE_LOCK_MEMORY,
    RESOURCE_LOCK_SHELL,
    RESOURCE_LOCK_NETWORK
} ResourceLock;

typedef enum {
    ERROR_TYPE_VALIDATION_ERROR,
    ERROR_TYPE_PERMISSION_ERROR,
    ERROR_TYPE_FILE_NOT_FOUND,
    ERROR_TYPE_TIMEOUT_ERROR,
    ERROR_TYPE_RESOURCE_ERROR,
    ERROR_TYPE_NETWORK_ERROR,
    ERROR_TYPE_SECURITY_ERROR,
    ERROR_TYPE_INTERNAL_ERROR
} ErrorType;

// HashMap implementation for string keys and void* values
typedef struct HashMapEntry {
    char* key;
    void* value;
    struct HashMapEntry* next;
    void (*destroy_value)(void*); // Destructor for value
} HashMapEntry;

typedef struct {
    HashMapEntry** buckets;
    size_t size;
    size_t capacity;
} HashMap;

// JSON value representation (simplified)
typedef enum {
    JSON_NULL,
    JSON_BOOL,
    JSON_NUMBER,
    JSON_STRING,
    JSON_ARRAY,
    JSON_OBJECT
} JsonType;

typedef struct JsonValue {
    JsonType type;
    int refcount; // Reference counting
    union {
        bool bool_value;
        double number_value;
        char* string_value;
        struct {
            struct JsonValue** values;
            size_t count;
        } array_value;
        HashMap* object_value;
    };
} JsonValue;

// ToolParameter structure
struct ToolParameter {
    char* name;
    char* type_;
    char* description;
    bool required;
    JsonValue* default_value;
    regex_t* pattern; // Compiled regex pattern
};

// ToolDefinition structure
struct ToolDefinition {
    char* name;
    char* description;
    ToolParameter* parameters;
    size_t parameters_count;
    char* category;
    ResourceLock* resource_locks;
    size_t resource_locks_count;
};

// ToolResult structure
struct ToolResult {
    bool success;
    char* output;
    char* error;
    uint64_t execution_time_ms;
    HashMap* metadata;
    HashMap* recovery_context;
};

// ToolError structure
struct ToolError {
    char* message;
    ErrorType error_type;
    HashMap* details;
};

// Function pointer types for Tool interface
typedef ToolDefinition* (*ToolDefinitionFn)(Tool* self);
typedef ToolResult* (*ToolExecuteFn)(Tool* self, HashMap* kwargs);
typedef char* (*ToolNameFn)(Tool* self);
typedef bool (*ToolValidateParametersFn)(Tool* self, HashMap* kwargs);

// Tool interface
struct Tool {
    ToolDefinitionFn definition_fn;
    ToolExecuteFn execute_fn;
    ToolNameFn name_fn;
    ToolValidateParametersFn validate_parameters_fn;
    void* data; // For implementation-specific data
};

// ToolRegistry structure
struct ToolRegistry {
    HashMap* tools;
};

// Function prototypes
HashMap* hashmap_create(size_t capacity);
void hashmap_destroy(HashMap* map);
void hashmap_put(HashMap* map, const char* key, void* value);
void hashmap_put_with_destructor(HashMap* map, const char* key, void* value, void (*destroy_value)(void*));
void* hashmap_get(HashMap* map, const char* key);
bool hashmap_contains(HashMap* map, const char* key);
void hashmap_remove(HashMap* map, const char* key);
size_t hashmap_size(HashMap* map);

JsonValue* json_value_create_null();
JsonValue* json_value_create_bool(bool value);
JsonValue* json_value_create_number(double value);
JsonValue* json_value_create_string(const char* value);
JsonValue* json_value_create_array(JsonValue** values, size_t count);
JsonValue* json_value_create_object(HashMap* object);
JsonValue* json_value_clone(JsonValue* value);
void json_value_destroy(JsonValue* value);
char* json_value_to_string(JsonValue* value);

ToolParameter* tool_parameter_new(const char* name, const char* type_, 
                                 const char* description, bool required, 
                                 JsonValue* default_value);
void tool_parameter_destroy(ToolParameter* param);

ToolDefinition* tool_definition_new(const char* name, const char* description,
                                   ToolParameter* parameters, size_t parameters_count,
                                   const char* category, ResourceLock* resource_locks,
                                   size_t resource_locks_count);
void tool_definition_destroy(ToolDefinition* def);
JsonValue* tool_definition_to_ollama_format(ToolDefinition* def);

ToolResult* tool_result_new(bool success, const char* output, const char* error,
                           uint64_t execution_time_ms, HashMap* metadata,
                           HashMap* recovery_context);
ToolResult* tool_result_success(const char* output, uint64_t execution_time_ms);
ToolResult* tool_result_error(const char* error, uint64_t execution_time_ms);
void tool_result_destroy(ToolResult* result);
char* tool_result_to_string(ToolResult* result);

ToolError* tool_error_new(const char* message, ErrorType error_type, HashMap* details);
void tool_error_destroy(ToolError* error);
char* tool_error_to_string(ToolError* error);

ToolResult* handle_error(const char* context, int err_code, const char* err_msg);
ToolResult* validate_required_params(HashMap* kwargs, char** required_params, size_t required_count);
ToolResult* validate_string_param(JsonValue* value, const char* param_name, 
                                 size_t min_length, size_t max_length, const char* pattern);
ToolResult* create_success_result(const char* output, uint64_t execution_time_ms, HashMap* metadata);
ToolResult* create_error_result(const char* error_msg, uint64_t execution_time_ms, 
                               ErrorType error_type, HashMap* metadata);

ToolRegistry* tool_registry_new();
void tool_registry_destroy(ToolRegistry* registry);
void tool_registry_register(ToolRegistry* registry, Tool* tool);
void tool_registry_register_core_tools(ToolRegistry* registry);
Tool* tool_registry_get_tool(ToolRegistry* registry, const char* name);
Tool** tool_registry_get_all_tools(ToolRegistry* registry, size_t* count);
JsonValue** tool_registry_get_tool_definitions(ToolRegistry* registry, size_t* count);
ToolResult* tool_registry_execute_tool(ToolRegistry* registry, const char* tool_name, HashMap* kwargs);
size_t tool_registry_tool_count(ToolRegistry* registry);
bool tool_registry_has_tool(ToolRegistry* registry, const char* name);
bool tool_registry_unregister(ToolRegistry* registry, const char* name);
void tool_registry_clear(ToolRegistry* registry);

ToolParameter* create_tool_parameter(const char* name, const char* type_,
                                    const char* description, bool required);
ToolParameter* create_tool_parameter_with_default(const char* name, const char* type_,
                                                 const char* description, bool required,
                                                 JsonValue* default_value);
ToolDefinition* create_tool_definition(const char* name, const char* description,
                                      const char* category, ToolParameter* parameters,
                                      size_t parameters_count);
ToolDefinition* create_tool_definition_with_locks(const char* name, const char* description,
                                                 const char* category, ToolParameter* parameters,
                                                 size_t parameters_count, ResourceLock* resource_locks,
                                                 size_t resource_locks_count);

// Helper functions
char* string_duplicate(const char* str);
char* resource_lock_to_string(ResourceLock lock);
char* error_type_to_string(ErrorType error_type);
bool tool_validate_parameters(Tool* tool, HashMap* kwargs);
bool json_type_matches(JsonValue* value, const char* expected_type);

// Implementation

// String duplication helper
char* string_duplicate(const char* str) {
    if (!str) return NULL;
    size_t len = strlen(str);
    char* dup = malloc(len + 1);
    if (dup) {
        strcpy(dup, str);
    }
    return dup;
}

// HashMap implementation
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

void hashmap_destroy(HashMap* map) {
    if (!map) return;
    
    for (size_t i = 0; i < map->capacity; i++) {
        HashMapEntry* entry = map->buckets[i];
        while (entry) {
            HashMapEntry* next = entry->next;
            free(entry->key);
            if (entry->destroy_value && entry->value) {
                entry->destroy_value(entry->value);
            }
            free(entry);
            entry = next;
        }
    }
    
    free(map->buckets);
    free(map);
}

void hashmap_put(HashMap* map, const char* key, void* value) {
    hashmap_put_with_destructor(map, key, value, NULL);
}

void hashmap_put_with_destructor(HashMap* map, const char* key, void* value, void (*destroy_value)(void*)) {
    if (!map || !key) return;
    
    size_t index = 0; // Simple hash function (would need better implementation)
    for (const char* p = key; *p; p++) {
        index = (index * 31 + *p) % map->capacity;
    }
    
    HashMapEntry* entry = map->buckets[index];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            if (entry->destroy_value && entry->value) {
                entry->destroy_value(entry->value);
            }
            entry->value = value;
            entry->destroy_value = destroy_value;
            return;
        }
        entry = entry->next;
    }
    
    entry = malloc(sizeof(HashMapEntry));
    if (!entry) return;
    
    entry->key = string_duplicate(key);
    entry->value = value;
    entry->destroy_value = destroy_value;
    entry->next = map->buckets[index];
    map->buckets[index] = entry;
    map->size++;
}

void* hashmap_get(HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    size_t index = 0;
    for (const char* p = key; *p; p++) {
        index = (index * 31 + *p) % map->capacity;
    }
    
    HashMapEntry* entry = map->buckets[index];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            return entry->value;
        }
        entry = entry->next;
    }
    
    return NULL;
}

bool hashmap_contains(HashMap* map, const char* key) {
    return hashmap_get(map, key) != NULL;
}

void hashmap_remove(HashMap* map, const char* key) {
    if (!map || !key) return;
    
    size_t index = 0;
    for (const char* p = key; *p; p++) {
        index = (index * 31 + *p) % map->capacity;
    }
    
    HashMapEntry** entry_ptr = &map->buckets[index];
    while (*entry_ptr) {
        HashMapEntry* entry = *entry_ptr;
        if (strcmp(entry->key, key) == 0) {
            *entry_ptr = entry->next;
            free(entry->key);
            if (entry->destroy_value && entry->value) {
                entry->destroy_value(entry->value);
            }
            free(entry);
            map->size--;
            return;
        }
        entry_ptr = &entry->next;
    }
}

size_t hashmap_size(HashMap* map) {
    return map ? map->size : 0;
}

// JSON value implementation
JsonValue* json_value_create_internal(JsonType type) {
    JsonValue* value = malloc(sizeof(JsonValue));
    if (value) {
        value->type = type;
        value->refcount = 1;
    }
    return value;
}

JsonValue* json_value_create_null() {
    return json_value_create_internal(JSON_NULL);
}

JsonValue* json_value_create_bool(bool value) {
    JsonValue* json_value = json_value_create_internal(JSON_BOOL);
    if (json_value) {
        json_value->bool_value = value;
    }
    return json_value;
}

JsonValue* json_value_create_number(double value) {
    JsonValue* json_value = json_value_create_internal(JSON_NUMBER);
    if (json_value) {
        json_value->number_value = value;
    }
    return json_value;
}

JsonValue* json_value_create_string(const char* value) {
    JsonValue* json_value = json_value_create_internal(JSON_STRING);
    if (json_value) {
        json_value->string_value = string_duplicate(value);
    }
    return json_value;
}

JsonValue* json_value_create_array(JsonValue** values, size_t count) {
    JsonValue* json_value = json_value_create_internal(JSON_ARRAY);
    if (json_value) {
        json_value->array_value.values = values;
        json_value->array_value.count = count;
    }
    return json_value;
}

JsonValue* json_value_create_object(HashMap* object) {
    JsonValue* json_value = json_value_create_internal(JSON_OBJECT);
    if (json_value) {
        json_value->object_value = object;
    }
    return json_value;
}

JsonValue* json_value_clone(JsonValue* value) {
    if (!value) return NULL;
    
    value->refcount++;
    return value;
}

void json_value_destroy(JsonValue* value) {
    if (!value) return;
    
    value->refcount--;
    if (value->refcount > 0) {
        return;
    }
    
    if (value->type == JSON_STRING) {
        free(value->string_value);
    } else if (value->type == JSON_ARRAY) {
        for (size_t i = 0; i < value->array_value.count; i++) {
            json_value_destroy(value->array_value.values[i]);
        }
        free(value->array_value.values);
    } else if (value->type == JSON_OBJECT) {
        hashmap_destroy(value->object_value);
    }
    
    free(value);
}

char* json_value_to_string(JsonValue* value) {
    if (!value) return string_duplicate("null");
    
    switch (value->type) {
        case JSON_NULL:
            return string_duplicate("null");
        case JSON_BOOL:
            return string_duplicate(value->bool_value ? "true" : "false");
        case JSON_NUMBER: {
            char buffer[32];
            snprintf(buffer, sizeof(buffer), "%.17g", value->number_value);
            return string_duplicate(buffer);
        }
        case JSON_STRING: {
            size_t len = strlen(value->string_value);
            char* result = malloc(len + 3);
            if (result) {
                result[0] = '"';
                strcpy(result + 1, value->string_value);
                result[len + 1] = '"';
                result[len + 2] = '\0';
            }
            return result;
        }
        case JSON_ARRAY: {
            char* result = string_duplicate("[");
            for (size_t i = 0; i < value->array_value.count; i++) {
                char* item_str = json_value_to_string(value->array_value.values[i]);
                if (item_str) {
                    char* new_result = malloc(strlen(result) + strlen(item_str) + 3);
                    if (new_result) {
                        strcpy(new_result, result);
                        strcat(new_result, item_str);
                        if (i < value->array_value.count - 1) {
                            strcat(new_result, ",");
                        }
                    }
                    free(result);
                    result = new_result;
                    free(item_str);
                }
            }
            char* new_result = malloc(strlen(result) + 2);
            if (new_result) {
                strcpy(new_result, result);
                strcat(new_result, "]");
            }
            free(result);
            return new_result;
        }
        case JSON_OBJECT: {
            char* result = string_duplicate("{");
            for (size_t i = 0; i < value->object_value->capacity; i++) {
                HashMapEntry* entry = value->object_value->buckets[i];
                while (entry) {
                    char* key_str = malloc(strlen(entry->key) + 3);
                    if (key_str) {
                        key_str[0] = '"';
                        strcpy(key_str + 1, entry->key);
                        strcat(key_str, "\"");
                    }
                    
                    char* value_str = json_value_to_string((JsonValue*)entry->value);
                    if (key_str && value_str) {
                        char* new_result = malloc(strlen(result) + strlen(key_str) + strlen(value_str) + 4);
                        if (new_result) {
                            strcpy(new_result, result);
                            strcat(new_result, key_str);
                            strcat(new_result, ":");
                            strcat(new_result, value_str);
                            // Add comma if not last item
                            HashMapEntry* temp = entry->next;
                            bool has_more = temp != NULL;
                            if (!has_more) {
                                for (size_t j = i + 1; j < value->object_value->capacity; j++) {
                                    if (value->object_value->buckets[j]) {
                                        has_more = true;
                                        break;
                                    }
                                }
                            }
                            if (has_more) {
                                strcat(new_result, ",");
                            }
                        }
                        free(result);
                        result = new_result;
                    }
                    free(key_str);
                    free(value_str);
                    entry = entry->next;
                }
            }
            char* new_result = malloc(strlen(result) + 2);
            if (new_result) {
                strcpy(new_result, result);
                strcat(new_result, "}");
            }
            free(result);
            return new_result;
        }
    }
    return string_duplicate("null");
}

// ToolParameter implementation
ToolParameter* tool_parameter_new(const char* name, const char* type_,
                                 const char* description, bool required,
                                 JsonValue* default_value) {
    ToolParameter* param = malloc(sizeof(ToolParameter));
    if (!param) return NULL;
    
    param->name = string_duplicate(name);
    param->type_ = string_duplicate(type_);
    param->description = string_duplicate(description);
    param->required = required;
    param->default_value = default_value;
    param->pattern = NULL;
    
    return param;
}

void tool_parameter_destroy(ToolParameter* param) {
    if (!param) return;
    
    free(param->name);
    free(param->type_);
    free(param->description);
    json_value_destroy(param->default_value);
    if (param->pattern) {
        regfree(param->pattern);
        free(param->pattern);
    }
    free(param);
}

// ResourceLock implementation
char* resource_lock_to_string(ResourceLock lock) {
    switch (lock) {
        case RESOURCE_LOCK_FILESYSTEM_WRITE:
            return string_duplicate("filesystem_write");
        case RESOURCE_LOCK_FILESYSTEM_READ:
            return string_duplicate("filesystem_read");
        case RESOURCE_LOCK_GIT:
            return string_duplicate("git");
        case RESOURCE_LOCK_MEMORY:
            return string_duplicate("memory");
        case RESOURCE_LOCK_SHELL:
            return string_duplicate("shell");
        case RESOURCE_LOCK_NETWORK:
            return string_duplicate("network");
        default:
            return string_duplicate("unknown");
    }
}

// ErrorType implementation
char* error_type_to_string(ErrorType error_type) {
    switch (error_type) {
        case ERROR_TYPE_VALIDATION_ERROR:
            return string_duplicate("validation_error");
        case ERROR_TYPE_PERMISSION_ERROR:
            return string_duplicate("permission_error");
        case ERROR_TYPE_FILE_NOT_FOUND:
            return string_duplicate("file_not_found");
        case ERROR_TYPE_TIMEOUT_ERROR:
            return string_duplicate("timeout_error");
        case ERROR_TYPE_RESOURCE_ERROR:
            return string_duplicate("resource_error");
        case ERROR_TYPE_NETWORK_ERROR:
            return string_duplicate("network_error");
        case ERROR_TYPE_SECURITY_ERROR:
            return string_duplicate("security_error");
        case ERROR_TYPE_INTERNAL_ERROR:
            return string_duplicate("internal_error");
        default:
            return string_duplicate("unknown_error");
    }
}

// ToolDefinition implementation
ToolDefinition* tool_definition_new(const char* name, const char* description,
                                   ToolParameter* parameters, size_t parameters_count,
                                   const char* category, ResourceLock* resource_locks,
                                   size_t resource_locks_count) {
    ToolDefinition* def = malloc(sizeof(ToolDefinition));
    if (!def) return NULL;
    
    def->name = string_duplicate(name);
    def->description = string_duplicate(description);
    def->parameters = parameters;
    def->parameters_count = parameters_count;
    def->category = string_duplicate(category);
    def->resource_locks = resource_locks;
    def->resource_locks_count = resource_locks_count;
    
    return def;
}

void tool_definition_destroy(ToolDefinition* def) {
    if (!def) return;
    
    free(def->name);
    free(def->description);
    free(def->category);
    
    for (size_t i = 0; i < def->parameters_count; i++) {
        tool_parameter_destroy(&def->parameters[i]);
    }
    free(def->parameters);
    
    free(def->resource_locks);
    free(def);
}

JsonValue* tool_definition_to_ollama_format(ToolDefinition* def) {
    if (!def) return NULL;
    
    HashMap* properties = hashmap_create(16);
    HashMap* required_params = hashmap_create(16);
    
    for (size_t i = 0; i < def->parameters_count; i++) {
        ToolParameter* param = &def->parameters[i];
        HashMap* prop_value = hashmap_create(8);
        
        JsonValue* type_value = json_value_create_string(param->type_);
        hashmap_put_with_destructor(prop_value, "type", type_value, (void (*)(void*))json_value_destroy);
        
        JsonValue* desc_value = json_value_create_string(param->description);
        hashmap_put_with_destructor(prop_value, "description", desc_value, (void (*)(void*))json_value_destroy);
        
        if (param->default_value) {
            JsonValue* default_clone = json_value_clone(param->default_value);
            hashmap_put_with_destructor(prop_value, "default", default_clone, (void (*)(void*))json_value_destroy);
        }
        
        if (param->required) {
            hashmap_put(required_params, param->name, param);
        }
        
        JsonValue* prop_json = json_value_create_object(prop_value);
        hashmap_put_with_destructor(properties, param->name, prop_json, (void (*)(void*))json_value_destroy);
    }
    
    HashMap* parameters_obj = hashmap_create(8);
    JsonValue* type_obj = json_value_create_string("object");
    hashmap_put_with_destructor(parameters_obj, "type", type_obj, (void (*)(void*))json_value_destroy);
    
    JsonValue* properties_obj = json_value_create_object(properties);
    hashmap_put_with_destructor(parameters_obj, "properties", properties_obj, (void (*)(void*))json_value_destroy);
    
    // Convert required_params to array of strings
    size_t required_count = hashmap_size(required_params);
    JsonValue** required_items = NULL;
    if (required_count > 0) {
        required_items = malloc(required_count * sizeof(JsonValue*));
        size_t idx = 0;
        for (size_t i = 0; i < required_params->capacity; i++) {
            HashMapEntry* entry = required_params->buckets[i];
            while (entry && idx < required_count) {
                required_items[idx] = json_value_create_string(entry->key);
                idx++;
                entry = entry->next;
            }
        }
    }
    
    JsonValue* required_array = json_value_create_array(required_items, required_count);
    hashmap_put_with_destructor(parameters_obj, "required", required_array, (void (*)(void*))json_value_destroy);
    
    HashMap* function_obj = hashmap_create(8);
    JsonValue* name_value = json_value_create_string(def->name);
    hashmap_put_with_destructor(function_obj, "name", name_value, (void (*)(void*))json_value_destroy);
    
    JsonValue* desc_value = json_value_create_string(def->description);
    hashmap_put_with_destructor(function_obj, "description", desc_value, (void (*)(void*))json_value_destroy);
    
    JsonValue* params_value = json_value_create_object(parameters_obj);
    hashmap_put_with_destructor(function_obj, "parameters", params_value, (void (*)(void*))json_value_destroy);
    
    HashMap* result_obj = hashmap_create(4);
    JsonValue* type_result = json_value_create_string("function");
    hashmap_put_with_destructor(result_obj, "type", type_result, (void (*)(void*))json_value_destroy);
    
    JsonValue* function_result = json_value_create_object(function_obj);
    hashmap_put_with_destructor(result_obj, "function", function_result, (void (*)(void*))json_value_destroy);
    
    hashmap_destroy(required_params);
    
    return json_value_create_object(result_obj);
}

// ToolResult implementation
ToolResult* tool_result_new(bool success, const char* output, const char* error,
                           uint64_t execution_time_ms, HashMap* metadata,
                           HashMap* recovery_context) {
    ToolResult* result = malloc(sizeof(ToolResult));
    if (!result) return NULL;
    
    result->success = success;
    result->output = string_duplicate(output ? output : "");
    result->error = string_duplicate(error ? error : "");
    result->execution_time_ms = execution_time_ms;
    result->metadata = metadata;
    result->recovery_context = recovery_context;
    
    return result;
}

ToolResult* tool_result_success(const char* output, uint64_t execution_time_ms) {
    return tool_result_new(true, output, NULL, execution_time_ms, NULL, NULL);
}

ToolResult* tool_result_error(const char* error, uint64_t execution_time_ms) {
    return tool_result_new(false, "", error, execution_time_ms, NULL, NULL);
}

void tool_result_destroy(ToolResult* result) {
    if (!result) return;
    
    free(result->output);
    free(result->error);
    hashmap_destroy(result->metadata);
    hashmap_destroy(result->recovery_context);
    free(result);
}

char* tool_result_to_string(ToolResult* result) {
    if (!result) return string_duplicate("");
    
    if (result->success) {
        return string_duplicate(result->output ? result->output : "");
    } else {
        char* error_msg = result->error ? result->error : "Unknown error";
        size_t len = strlen("Error: ") + strlen(error_msg) + 1;
        char* str = malloc(len);
        if (str) {
            snprintf(str, len, "Error: %s", error_msg);
        }
        return str;
    }
}

// ToolError implementation
ToolError* tool_error_new(const char* message, ErrorType error_type, HashMap* details) {
    ToolError* error = malloc(sizeof(ToolError));
    if (!error) return NULL;
    
    error->message = string_duplicate(message);
    error->error_type = error_type;
    error->details = details;
    
    return error;
}

void tool_error_destroy(ToolError* error) {
    if (!error) return;
    
    free(error->message);
    hashmap_destroy(error->details);
    free(error);
}

char* tool_error_to_string(ToolError* error) {
    if (!error) return string_duplicate("");
    return string_duplicate(error->message ? error->message : "");
}

// Error handling functions
ToolResult* handle_error(const char* context, int err_code, const char* err_msg) {
    HashMap* metadata = hashmap_create(8);
    if (metadata) {
        JsonValue* context_value = json_value_create_string(context);
        hashmap_put_with_destructor(metadata, "context", context_value, (void (*)(void*))json_value_destroy);
        
        ErrorType error_type = ERROR_TYPE_INTERNAL_ERROR;
        JsonValue* error_type_value = json_value_create_string("internal_error");
        
        if (err_msg && strstr(err_msg, "I/O")) {
            error_type = ERROR_TYPE_RESOURCE_ERROR;
            error_type_value = json_value_create_string("resource_error");
        }
        
        hashmap_put_with_destructor(metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
    }
    
    return tool_result_new(false, "", err_msg, 0, metadata, NULL);
}

ToolResult* validate_required_params(HashMap* kwargs, char** required_params, size_t required_count) {
    if (!kwargs || !required_params) return NULL;
    
    char** missing_params = malloc(required_count * sizeof(char*));
    if (!missing_params) return NULL;
    
    size_t missing_count = 0;
    
    for (size_t i = 0; i < required_count; i++) {
        if (!hashmap_contains(kwargs, required_params[i])) {
            missing_params[missing_count] = string_duplicate(required_params[i]);
            missing_count++;
        }
    }
    
    if (missing_count > 0) {
        // Build error message
        size_t msg_len = strlen("Missing required parameters: ") + 1;
        for (size_t i = 0; i < missing_count; i++) {
            msg_len += strlen(missing_params[i]);
            if (i < missing_count - 1) {
                msg_len += 2; // for ", "
            }
        }
        
        char* error_msg = malloc(msg_len);
        if (error_msg) {
            strcpy(error_msg, "Missing required parameters: ");
            for (size_t i = 0; i < missing_count; i++) {
                strcat(error_msg, missing_params[i]);
                if (i < missing_count - 1) {
                    strcat(error_msg, ", ");
                }
            }
        }
        
        // Create metadata
        HashMap* metadata = hashmap_create(8);
        if (metadata) {
            JsonValue* error_type_value = json_value_create_string("validation_error");
            hashmap_put_with_destructor(metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
            
            // Create array of missing params
            JsonValue** missing_items = malloc(missing_count * sizeof(JsonValue*));
            if (missing_items) {
                for (size_t i = 0; i < missing_count; i++) {
                    missing_items[i] = json_value_create_string(missing_params[i]);
                }
                JsonValue* missing_array = json_value_create_array(missing_items, missing_count);
                hashmap_put_with_destructor(metadata, "missing_params", missing_array, (void (*)(void*))json_value_destroy);
            }
        }
        
        // Clean up
        for (size_t i = 0; i < missing_count; i++) {
            free(missing_params[i]);
        }
        free(missing_params);
        free(error_msg);
        
        return tool_result_new(false, "", error_msg, 0, metadata, NULL);
    }
    
    // Clean up
    for (size_t i = 0; i < missing_count; i++) {
        free(missing_params[i]);
    }
    free(missing_params);
    
    return NULL; // No validation errors
}

ToolResult* validate_string_param(JsonValue* value, const char* param_name, 
                                 size_t min_length, size_t max_length, const char* pattern) {
    if (!value || !param_name) return NULL;
    
    if (value->type != JSON_STRING) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "Parameter '%s' must be a string", param_name);
        
        HashMap* metadata = hashmap_create(8);
        if (metadata) {
            JsonValue* error_type_value = json_value_create_string("validation_error");
            hashmap_put_with_destructor(metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* param_name_value = json_value_create_string(param_name);
            hashmap_put_with_destructor(metadata, "param_name", param_name_value, (void (*)(void*))json_value_destroy);
        }
        
        return tool_result_new(false, "", error_msg, 0, metadata, NULL);
    }
    
    size_t actual_length = strlen(value->string_value);
    
    if (actual_length < min_length) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), 
                "Parameter '%s' must be at least %zu characters, got %zu", 
                param_name, min_length, actual_length);
        
        HashMap* metadata = hashmap_create(8);
        if (metadata) {
            JsonValue* error_type_value = json_value_create_string("validation_error");
            hashmap_put_with_destructor(metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* param_name_value = json_value_create_string(param_name);
            hashmap_put_with_destructor(metadata, "param_name", param_name_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* actual_length_value = json_value_create_number(actual_length);
            hashmap_put_with_destructor(metadata, "actual_length", actual_length_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* min_length_value = json_value_create_number(min_length);
            hashmap_put_with_destructor(metadata, "min_length", min_length_value, (void (*)(void*))json_value_destroy);
        }
        
        return tool_result_new(false, "", error_msg, 0, metadata, NULL);
    }
    
    if (actual_length > max_length) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), 
                "Parameter '%s' must be at most %zu characters, got %zu", 
                param_name, max_length, actual_length);
        
        HashMap* metadata = hashmap_create(8);
        if (metadata) {
            JsonValue* error_type_value = json_value_create_string("validation_error");
            hashmap_put_with_destructor(metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* param_name_value = json_value_create_string(param_name);
            hashmap_put_with_destructor(metadata, "param_name", param_name_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* actual_length_value = json_value_create_number(actual_length);
            hashmap_put_with_destructor(metadata, "actual_length", actual_length_value, (void (*)(void*))json_value_destroy);
            
            JsonValue* max_length_value = json_value_create_number(max_length);
            hashmap_put_with_destructor(metadata, "max_length", max_length_value, (void (*)(void*))json_value_destroy);
        }
        
        return tool_result_new(false, "", error_msg, 0, metadata, NULL);
    }
    
    if (pattern) {
        regex_t regex;
        if (regcomp(&regex, pattern, REG_EXTENDED) == 0) {
            if (regexec(&regex, value->string_value, 0, NULL, 0) != 0) {
                char error_msg[256];
                snprintf(error_msg, sizeof(error_msg), 
                        "Parameter '%s' does not match required pattern", param_name);
                
                HashMap* metadata = hashmap_create(8);
                if (metadata) {
                    JsonValue* error_type_value = json_value_create_string("validation_error");
                    hashmap_put_with_destructor(metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
                    
                    JsonValue* param_name_value = json_value_create_string(param_name);
                    hashmap_put_with_destructor(metadata, "param_name", param_name_value, (void (*)(void*))json_value_destroy);
                    
                    JsonValue* pattern_value = json_value_create_string(pattern);
                    hashmap_put_with_destructor(metadata, "pattern", pattern_value, (void (*)(void*))json_value_destroy);
                }
                
                regfree(&regex);
                return tool_result_new(false, "", error_msg, 0, metadata, NULL);
            }
            regfree(&regex);
        }
    }
    
    return NULL; // No validation errors
}

ToolResult* create_success_result(const char* output, uint64_t execution_time_ms, HashMap* metadata) {
    return tool_result_new(true, output, NULL, execution_time_ms, metadata, NULL);
}

ToolResult* create_error_result(const char* error_msg, uint64_t execution_time_ms, 
                               ErrorType error_type, HashMap* metadata) {
    HashMap* result_metadata = hashmap_create(8);
    if (result_metadata) {
        char* error_type_str = error_type_to_string(error_type);
        if (error_type_str) {
            JsonValue* error_type_value = json_value_create_string(error_type_str);
            hashmap_put_with_destructor(result_metadata, "error_type", error_type_value, (void (*)(void*))json_value_destroy);
            free(error_type_str);
        }
        
        if (metadata) {
            for (size_t i = 0; i < metadata->capacity; i++) {
                HashMapEntry* entry = metadata->buckets[i];
                while (entry) {
                    hashmap_put_with_destructor(result_metadata, entry->key, entry->value, entry->destroy_value);
                    entry = entry->next;
                }
            }
        }
    }
    
    return tool_result_new(false, "", error_msg, execution_time_ms, result_metadata, NULL);
}

// ToolRegistry implementation
ToolRegistry* tool_registry_new() {
    ToolRegistry* registry = malloc(sizeof(ToolRegistry));
    if (!registry) return NULL;
    
    registry->tools = hashmap_create(32);
    if (!registry->tools) {
        free(registry);
        return NULL;
    }
    
    return registry;
}

void tool_registry_destroy(ToolRegistry* registry) {
    if (!registry) return;
    
    // Free each tool before destroying the hashmap
    for (size_t i = 0; i < registry->tools->capacity; i++) {
        HashMapEntry* entry = registry->tools->buckets[i];
        while (entry) {
            Tool* tool = (Tool*)entry->value;
            if (tool && tool->data) {
                // Assuming tool->data points to the actual tool struct
                // In a real implementation, you'd have a destroy function for each tool type
                free(tool->data);
                free(tool);
            }
            entry = entry->next;
        }
    }
    
    hashmap_destroy(registry->tools);
    free(registry);
}

void tool_registry_register(ToolRegistry* registry, Tool* tool) {
    if (!registry || !tool) return;
    
    char* tool_name = tool->name_fn(tool);
    if (tool_name) {
        hashmap_put(registry->tools, tool_name, tool);
        free(tool_name);
    }
}

void tool_registry_register_core_tools(ToolRegistry* registry) {
    if (!registry) return;
    // Implementation would register all core tools
    printf("Core tools registration structure prepared\n");
}

Tool* tool_registry_get_tool(ToolRegistry* registry, const char* name) {
    if (!registry || !name) return NULL;
    return (Tool*)hashmap_get(registry->tools, name);
}

Tool** tool_registry_get_all_tools(ToolRegistry* registry, size_t* count) {
    if (!registry || !count) return NULL;
    *count = hashmap_size(registry->tools);
    // Implementation would return array of tools
    return NULL;
}

JsonValue** tool_registry_get_tool_definitions(ToolRegistry* registry, size_t* count) {
    if (!registry || !count) return NULL;
    *count = hashmap_size(registry->tools);
    // Implementation would return array of JSON definitions
    return NULL;
}

ToolResult* tool_registry_execute_tool(ToolRegistry* registry, const char* tool_name, HashMap* kwargs) {
    if (!registry || !tool_name) {
        return tool_result_error("Invalid registry or tool name", 0);
    }
    
    Tool* tool = tool_registry_get_tool(registry, tool_name);
    if (!tool) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "Tool '%s' not found", tool_name);
        return tool_result_error(error_msg, 0);
    }
    
    if (!tool_validate_parameters(tool, kwargs)) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "Invalid parameters for tool '%s'", tool_name);
        return tool_result_error(error_msg, 0);
    }
    
    return tool->execute_fn(tool, kwargs);
}

size_t tool_registry_tool_count(ToolRegistry* registry) {
    return registry ? hashmap_size(registry->tools) : 0;
}

bool tool_registry_has_tool(ToolRegistry* registry, const char* name) {
    return registry && name ? hashmap_contains(registry->tools, name) : false;
}

bool tool_registry_unregister(ToolRegistry* registry, const char* name) {
    if (!registry || !name) return false;
    
    bool exists = hashmap_contains(registry->tools, name);
    if (exists) {
        hashmap_remove(registry->tools, name);
    }
    return exists;
}

void tool_registry_clear(ToolRegistry* registry) {
    if (!registry) return;
    
    // Free each tool before clearing the hashmap
    for (size_t i = 0; i < registry->tools->capacity; i++) {
        HashMapEntry* entry = registry->tools->buckets[i];
        while (entry) {
            Tool* tool = (Tool*)entry->value;
            if (tool && tool->data) {
                // Assuming tool->data points to the actual tool struct
                // In a real implementation, you'd have a destroy function for each tool type
                free(tool->data);
                free(tool);
            }
            entry = entry->next;
        }
    }
    
    hashmap_destroy(registry->tools);
    registry->tools = hashmap_create(32);
}

// Helper functions for creating parameters and definitions
ToolParameter* create_tool_parameter(const char* name, const char* type_,
                                    const char* description, bool required) {
    return tool_parameter_new(name, type_, description, required, NULL);
}

ToolParameter* create_tool_parameter_with_default(const char* name, const char* type_,
                                                 const char* description, bool required,
                                                 JsonValue* default_value) {
    return tool_parameter_new(name, type_, description, required, default_value);
}

ToolDefinition* create_tool_definition(const char* name, const char* description,
                                      const char* category, ToolParameter* parameters,
                                      size_t parameters_count) {
    return tool_definition_new(name, description, parameters, parameters_count, 
                              category, NULL, 0);
}

ToolDefinition* create_tool_definition_with_locks(const char* name, const char* description,
                                                 const char* category, ToolParameter* parameters,
                                                 size_t parameters_count, ResourceLock* resource_locks,
                                                 size_t resource_locks_count) {
    return tool_definition_new(name, description, parameters, parameters_count,
                              category, resource_locks, resource_locks_count);
}

// Tool parameter validation
bool json_type_matches(JsonValue* value, const char* expected_type) {
    if (!value || !expected_type) return false;
    
    if (strcmp(expected_type, "string") == 0) return value->type == JSON_STRING;
    if (strcmp(expected_type, "number") == 0) return value->type == JSON_NUMBER;
    if (strcmp(expected_type, "boolean") == 0) return value->type == JSON_BOOL;
    if (strcmp(expected_type, "array") == 0) return value->type == JSON_ARRAY;
    if (strcmp(expected_type, "object") == 0) return value->type == JSON_OBJECT;
    
    return false;
}

bool tool_validate_parameters(Tool* tool, HashMap* kwargs) {
    if (!tool || !kwargs) return false;
    
    ToolDefinition* definition = tool->definition_fn(tool);
    if (!definition) return false;
    
    // Check required parameters
    for (size_t i = 0; i < definition->parameters_count; i++) {
        ToolParameter* param = &definition->parameters[i];
        if (param->required && !hashmap_contains(kwargs, param->name)) {
            return false;
        }
    }
    
    // Validate parameter types
    for (size_t i = 0; i < kwargs->capacity; i++) {
        HashMapEntry* entry = kwargs->buckets[i];
        while (entry) {
            const char* param_name = entry->key;
            JsonValue* value = (JsonValue*)entry->value;
            
            // Find parameter definition
            ToolParameter* param_def = NULL;
            for (size_t j = 0; j < definition->parameters_count; j++) {
                if (strcmp(definition->parameters[j].name, param_name) == 0) {
                    param_def = &definition->parameters[j];
                    break;
                }
            }
            
            if (param_def) {
                // Special case for "value" parameter with JSON-serializable description
                if (strcmp(param_name, "value") == 0 && 
                    strstr(param_def->description, "JSON-serializable")) {
                    // Skip validation for this special case
                } else if (!json_type_matches(value, param_def->type_)) {
                    return false;
                }
            }
            
            entry = entry->next;
        }
    }
    
    return true;
}

