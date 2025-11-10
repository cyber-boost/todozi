#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <curl/curl.h>
#include "jansson.h"

// Error handling structure
typedef enum {
    TDZ_ERROR_NONE,
    TDZ_ERROR_VALIDATION,
    TDZ_ERROR_NETWORK,
    TDZ_ERROR_JSON,
    TDZ_ERROR_OOM
} TdzErrorType;

typedef struct {
    TdzErrorType type;
    char* message;
} TdzError;

// TdzCommand structure
typedef struct {
    char* command;
    char* target;
    char** parameters;
    size_t param_count;
    json_t* options; // Using jansson for key-value pairs
} TdzCommand;

// Vector for commands
typedef struct {
    TdzCommand* commands;
    size_t count;
    size_t capacity;
} TdzCommandVector;

// Memory response structure for curl
struct MemoryStruct {
    char* memory;
    size_t size;
};

// Helper function prototypes
char* find_todozi(const char* str);
TdzCommandVector* parse_tdz_command(const char* text, TdzError** error);
TdzError* execute_tdz_command(TdzCommand* command, const char* base_url, const char* api_key, json_t** result);
char* get_endpoint_path(TdzCommand* command);
json_t* build_request_body(TdzCommand* command);
json_t* build_run_body(TdzCommand* command);
TdzError* process_tdz_commands(const char* text, const char* base_url, const char* api_key, json_t** results);

// Curl write callback
static size_t WriteMemoryCallback(void* contents, size_t size, size_t nmemb, struct MemoryStruct* mem) {
    size_t realsize = size * nmemb;
    
    // Check for overflow
    if (realsize / size != nmemb) {
        return 0;
    }
    
    char* ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

// Helper function to duplicate string with error checking
static char* string_duplicate(const char* str) {
    if (!str) return NULL;
    
    size_t len = strlen(str);
    char* new_str = malloc(len + 1);
    if (!new_str) return NULL;
    
    // Use memcpy for safety (we know the exact length)
    memcpy(new_str, str, len);
    new_str[len] = '\0';
    return new_str;
}

// Helper function to lowercase string
static void string_to_lowercase(char* str) {
    if (!str) return;
    for (size_t i = 0; str[i]; i++) {
        str[i] = tolower((unsigned char)str[i]);
    }
}

// Create new TdzCommand
TdzCommand* tdz_command_new() {
    TdzCommand* cmd = calloc(1, sizeof(TdzCommand));
    if (!cmd) return NULL;
    cmd->options = json_object();
    if (!cmd->options) {
        free(cmd);
        return NULL;
    }
    return cmd;
}

// Free TdzCommand
void tdz_command_free(TdzCommand* cmd) {
    if (!cmd) return;
    if (cmd->command) free(cmd->command);
    if (cmd->target) free(cmd->target);
    if (cmd->parameters) {
        for (size_t i = 0; i < cmd->param_count; i++) {
            if (cmd->parameters[i]) free(cmd->parameters[i]);
        }
        free(cmd->parameters);
    }
    if (cmd->options) json_decref(cmd->options);
    free(cmd);
}

// Create new TdzCommandVector
TdzCommandVector* tdz_command_vector_new() {
    TdzCommandVector* vec = calloc(1, sizeof(TdzCommandVector));
    return vec;
}

// Free TdzCommandVector
void tdz_command_vector_free(TdzCommandVector* vec) {
    if (!vec) return;
    if (vec->commands) {
        for (size_t i = 0; i < vec->count; i++) {
            // Free individual command fields since commands are stored as values, not pointers
            if (vec->commands[i].command) free(vec->commands[i].command);
            if (vec->commands[i].target) free(vec->commands[i].target);
            if (vec->commands[i].parameters) {
                for (size_t j = 0; j < vec->commands[i].param_count; j++) {
                    if (vec->commands[i].parameters[j]) free(vec->commands[i].parameters[j]);
                }
                free(vec->commands[i].parameters);
            }
            if (vec->commands[i].options) json_decref(vec->commands[i].options);
        }
        free(vec->commands);
    }
    free(vec);
}

// Add command to vector with capacity checking
TdzError* tdz_command_vector_add(TdzCommandVector* vec, TdzCommand* cmd) {
    if (!vec || !cmd) return NULL;
    
    if (vec->count >= vec->capacity) {
        size_t new_capacity = vec->capacity == 0 ? 8 : vec->capacity * 2;
        TdzCommand* new_commands = realloc(vec->commands, new_capacity * sizeof(TdzCommand));
        if (!new_commands) {
            return (TdzError*)-1; // Special marker for OOM
        }
        vec->commands = new_commands;
        vec->capacity = new_capacity;
    }
    
    vec->commands[vec->count++] = *cmd;
    return NULL;
}

// Create new TdzError
TdzError* tdz_error_new(TdzErrorType type, const char* message) {
    TdzError* error = malloc(sizeof(TdzError));
    if (!error) return NULL;
    error->type = type;
    error->message = string_duplicate(message);
    if (!error->message) {
        free(error);
        return NULL;
    }
    return error;
}

// Free TdzError
void tdz_error_free(TdzError* error) {
    if (!error) return;
    if (error->message) free(error->message);
    free(error);
}

// Find todozi path
char* find_todozi(const char* str) {
    char* home = getenv("HOME");
    if (!home) return NULL;
    
    size_t base_len = strlen(home) + 9; // +9 for "/.todozi" + null terminator
    char* base = malloc(base_len);
    if (!base) return NULL;
    
    int written = snprintf(base, base_len, "%s/.todozi", home);
    if (written < 0 || (size_t)written >= base_len) {
        free(base);
        return NULL;
    }
    
    if (str) {
        size_t result_len = strlen(base) + strlen(str) + 2; // +1 for '/', +1 for null terminator
        char* result = malloc(result_len);
        if (!result) {
            free(base);
            return NULL;
        }
        
        written = snprintf(result, result_len, "%s/%s", base, str);
        free(base);
        if (written < 0 || (size_t)written >= result_len) {
            free(result);
            return NULL;
        }
        return result;
    }
    
    return base;
}

// Parse parameters and options from parts
TdzError* parse_parameters_and_options(TdzCommand* cmd, char** parts, size_t part_count) {
    // Parse parameters and options from parts[2] onwards
    for (size_t i = 2; i < part_count; i++) {
        char* part = parts[i];
        char* equals = strchr(part, '=');
        if (equals) {
            // This is an option - create copies to avoid modifying original
            size_t key_len = equals - part;
            char* key = malloc(key_len + 1);
            if (!key) return tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            
            strncpy(key, part, key_len);
            key[key_len] = '\0';
            string_to_lowercase(key);
            
            char* value = string_duplicate(equals + 1);
            if (!value) {
                free(key);
                return tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            }
            
            // json_object_set_new takes ownership of the JSON value
            // json_string creates a copy, so we can free value after
            json_object_set_new(cmd->options, key, json_string(value));
            free(key);
            free(value); // Safe to free - json_string made a copy
        } else {
            // This is a parameter
            char* param = string_duplicate(part);
            if (!param) return tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            
            char** new_params = realloc(cmd->parameters, (cmd->param_count + 1) * sizeof(char*));
            if (!new_params) {
                free(param);
                return tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            }
            
            cmd->parameters = new_params;
            cmd->parameters[cmd->param_count++] = param;
        }
    }
    return NULL;
}

// Parse tdz command
TdzCommandVector* parse_tdz_command(const char* text, TdzError** error) {
    if (error) *error = NULL;
    
    TdzCommandVector* commands = tdz_command_vector_new();
    if (!commands) {
        if (error) *error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
        return NULL;
    }
    
    // Simple parsing - in real implementation you'd use regex
    // This is a simplified version that looks for <tdz>...</tdz> patterns
    const char* start_tag = "<tdz>";
    const char* end_tag = "</tdz>";
    
    const char* pos = text;
    while ((pos = strstr(pos, start_tag)) != NULL) {
        pos += 5; // Skip <tdz>
        const char* end_pos = strstr(pos, end_tag);
        if (!end_pos) break;
        
        // Extract content
        size_t content_len = end_pos - pos;
        char* content = malloc(content_len + 1);
        if (!content) {
            if (error) *error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            tdz_command_vector_free(commands);
            return NULL;
        }
        strncpy(content, pos, content_len);
        content[content_len] = '\0';
        
        // Trim whitespace
        char* trimmed = content;
        while (*trimmed == ' ' || *trimmed == '\t' || *trimmed == '\r' || *trimmed == '\n') {
            trimmed++;
        }
        char* trimmed_end = trimmed + strlen(trimmed) - 1;
        while (trimmed_end > trimmed && 
               (*trimmed_end == ' ' || *trimmed_end == '\t' || *trimmed_end == '\r' || *trimmed_end == '\n')) {
            *trimmed_end = '\0';
            trimmed_end--;
        }
        
        // Split by ;
        char* content_copy = string_duplicate(trimmed);
        if (!content_copy) {
            free(content);
            continue;
        }
        
        // Count parts
        size_t part_count = 1;
        for (size_t i = 0; content_copy[i]; i++) {
            if (content_copy[i] == ';') part_count++;
        }
        
        // Allocate parts array
        char** parts = malloc(part_count * sizeof(char*));
        if (!parts) {
            free(content_copy);
            free(content);
            continue;
        }
        
        // Split content using strtok_r for thread safety
        char* saveptr;
        char* token = strtok_r(content_copy, ";", &saveptr);
        size_t i = 0;
        while (token && i < part_count) {
            // Trim token
            while (*token == ' ' || *token == '\t' || *token == '\r' || *token == '\n') {
                token++;
            }
            char* token_end = token + strlen(token) - 1;
            while (token_end > token && 
                   (*token_end == ' ' || *token_end == '\t' || *token_end == '\r' || *token_end == '\n')) {
                *token_end = '\0';
                token_end--;
            }
            
            parts[i] = string_duplicate(token);
            if (!parts[i]) {
                // Clean up previously allocated parts
                for (size_t j = 0; j < i; j++) {
                    free(parts[j]);
                }
                free(parts);
                free(content_copy);
                free(content);
                continue;
            }
            i++;
            token = strtok_r(NULL, ";", &saveptr);
        }
        
        if (i > 0) {
            TdzCommand* cmd = tdz_command_new();
            if (cmd) {
                cmd->command = string_duplicate(parts[0]);
                if (cmd->command) {
                    string_to_lowercase(cmd->command);
                    
                    if (i > 1) {
                        cmd->target = string_duplicate(parts[1]);
                        if (cmd->target) {
                            string_to_lowercase(cmd->target);
                        }
                    } else {
                        cmd->target = string_duplicate("");
                    }
                    
                    if (cmd->target) {
                        TdzError* parse_error = parse_parameters_and_options(cmd, parts, i);
                        if (parse_error) {
                            tdz_command_free(cmd);
                            // Clean up parts
                            for (size_t j = 0; j < i; j++) {
                                free(parts[j]);
                            }
                            free(parts);
                            free(content_copy);
                            free(content);
                            if (error) *error = parse_error;
                            tdz_command_vector_free(commands);
                            return NULL;
                        }
                        
                        TdzError* add_error = tdz_command_vector_add(commands, cmd);
                        if (add_error) {
                            tdz_command_free(cmd);
                            // Clean up parts
                            for (size_t j = 0; j < i; j++) {
                                free(parts[j]);
                            }
                            free(parts);
                            free(content_copy);
                            free(content);
                            if (add_error == (TdzError*)-1) {
                                if (error) *error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
                                tdz_command_vector_free(commands);
                                return NULL;
                            }
                            continue;
                        }
                    } else {
                        tdz_command_free(cmd);
                    }
                } else {
                    tdz_command_free(cmd);
                }
            }
        }
        
        // Cleanup
        for (size_t j = 0; j < i; j++) {
            if (parts[j]) free(parts[j]);
        }
        free(parts);
        free(content_copy);
        free(content);
        
        pos = end_pos + 6; // Skip </tdz>
    }
    
    return commands;
}

// Helper to get first parameter or default
static const char* get_first_param(TdzCommand* command) {
    if (command->param_count > 0 && command->parameters[0]) {
        return command->parameters[0];
    }
    return "";
}

// Get endpoint path with lookup table
char* get_endpoint_path(TdzCommand* command) {
    // Endpoint mapping table
    static const struct {
        const char* cmd;
        const char* target;
        const char* format;
        bool needs_param;
    } endpoint_map[] = {
        {"list", "health", "/health", false},
        {"get", "health", "/health", false},
        {"list", "stats", "/stats", false},
        {"get", "stats", "/stats", false},
        {"run", "init", "/init", false},
        {"list", "tasks", "/tasks", false},
        {"get", "task", "/tasks/%s", true},
        {"create", "task", "/tasks", false},
        {"update", "task", "/tasks/%s", true},
        {"delete", "task", "/tasks/%s", true},
        {"search", "tasks", "/tasks/search?q=%s", true},
        {"list", "memories", "/memories", false},
        {"list", "memories_secret", "/memories/secret", false},
        {"list", "memories_human", "/memories/human", false},
        {"list", "memories_short", "/memories/short", false},
        {"list", "memories_long", "/memories/long", false},
        {"create", "memory", "/memories", false},
        {"list", "ideas", "/ideas", false},
        {"create", "idea", "/ideas", false},
        {"list", "agents", "/agents", false},
        {"list", "agents_available", "/agents/available", false},
        {"get", "agent", "/agents/%s", true},
        {"get", "agent_status", "/agents/%s/status", true},
        {"create", "agent", "/agents", false},
        {"update", "agent", "/agents/%s", true},
        {"delete", "agent", "/agents/%s", true},
        {"run", "agent", "/chat/agent/%s", true},
        {"list", "training", "/training", false},
        {"get", "training", "/training/%s", true},
        {"create", "training", "/training", false},
        {"update", "training", "/training/%s", true},
        {"delete", "training", "/training/%s", true},
        {"run", "training_export", "/training/export", false},
        {"list", "training_stats", "/training/stats", false},
        {"run", "chat", "/chat/process", false},
        {"list", "chat_history", "/chat/history", false},
        {"list", "analytics_tasks", "/analytics/tasks", false},
        {"list", "analytics_agents", "/analytics/agents", false},
        {"list", "analytics_performance", "/analytics/performance", false},
        {"run", "time_start", "/time/start/%s", true},
        {"run", "time_stop", "/time/stop/%s", true},
        {"list", "time_report", "/time/report", false},
        {"list", "chunks", "/chunks", false},
        {"list", "chunks_ready", "/chunks/ready", false},
        {"list", "chunks_graph", "/chunks/graph", false},
        {"create", "chunk", "/chunks", false},
        {"list", "projects", "/projects", false},
        {"create", "project", "/projects", false},
        {"list", "feelings", "/feelings", false},
        {"get", "feeling", "/feelings/%s", true},
        {"create", "feeling", "/feelings", false},
        {"update", "feeling", "/feelings/%s/%s", true}, // Special case handled separately
        {"delete", "feeling", "/feelings/%s", true},
        {"list", "errors", "/errors", false},
        {"get", "error", "/errors/%s", true},
        {"create", "error", "/errors", false},
        {"update", "error", "/errors/%s", true},
        {"delete", "error", "/errors/%s", true},
        {"search", "errors", "/errors/search?q=%s", true},
        {"create", "queue_item", "/queue/plan", false},
        {"list", "queue", "/queue/list", false},
        {"list", "queue_backlog", "/queue/list/backlog", false},
        {"list", "queue_active", "/queue/list/active", false},
        {"list", "queue_complete", "/queue/list/complete", false},
        {"run", "queue_start", "/queue/start/%s", true},
        {"run", "queue_end", "/queue/end/%s", true},
        {"run", "api_register", "/api/register", false},
        {"run", "api_check", "/api/check", false},
        {NULL, NULL, NULL, false}
    };
    
    // Special case for update feeling (requires 2 parameters)
    if (strcmp(command->command, "update") == 0 && strcmp(command->target, "feeling") == 0) {
        const char* param0 = command->param_count > 0 ? command->parameters[0] : "";
        const char* param1 = command->param_count > 1 ? command->parameters[1] : "";
        size_t len = strlen("/feelings/") + strlen(param0) + 1 + strlen(param1) + 1; // +1 for '/', +1 for null terminator
        char* result = malloc(len);
        if (result) {
            int written = snprintf(result, len, "/feelings/%s/%s", param0, param1);
            if (written < 0 || (size_t)written >= len) {
                free(result);
                return NULL;
            }
        }
        return result;
    }
    
    // Search through the mapping table
    for (size_t i = 0; endpoint_map[i].cmd; i++) {
        if (strcmp(command->command, endpoint_map[i].cmd) == 0 && 
            strcmp(command->target, endpoint_map[i].target) == 0) {
            
            if (endpoint_map[i].needs_param) {
                const char* param = get_first_param(command);
                // Calculate size: format length - 2 (for %s) + param length + 1 (null terminator)
                size_t len = strlen(endpoint_map[i].format) - 2 + strlen(param) + 1;
                char* result = malloc(len);
                if (result) {
                    int written = snprintf(result, len, endpoint_map[i].format, param);
                    if (written < 0 || (size_t)written >= len) {
                        free(result);
                        return NULL;
                    }
                }
                return result;
            } else {
                return string_duplicate(endpoint_map[i].format);
            }
        }
    }
    
    // Default case
    if (command->target) {
        size_t len = strlen("/") + strlen(command->target) + 1; // +1 for null terminator
        char* result = malloc(len);
        if (result) {
            int written = snprintf(result, len, "/%s", command->target);
            if (written < 0 || (size_t)written >= len) {
                free(result);
                return NULL;
            }
        }
        return result;
    }
    return string_duplicate("/");
}

// Helper to convert comma-separated values to JSON array
json_t* comma_separated_to_array(const char* csv) {
    if (!csv) return json_array();
    
    json_t* array = json_array();
    if (!array) return NULL;
    
    char* csv_copy = string_duplicate(csv);
    if (!csv_copy) {
        json_decref(array);
        return NULL;
    }
    
    char* saveptr;
    char* token = strtok_r(csv_copy, ",", &saveptr);
    while (token) {
        // Trim token
        while (*token == ' ' || *token == '\t' || *token == '\r' || *token == '\n') {
            token++;
        }
        char* token_end = token + strlen(token) - 1;
        while (token_end > token && 
               (*token_end == ' ' || *token_end == '\t' || *token_end == '\r' || *token_end == '\n')) {
            *token_end = '\0';
            token_end--;
        }
        
        json_array_append_new(array, json_string(token));
        token = strtok_r(NULL, ",", &saveptr);
    }
    
    free(csv_copy);
    return array;
}

// Helper to get option value or default
static const char* get_option(TdzCommand* command, const char* key, const char* default_value) {
    json_t* value = json_object_get(command->options, key);
    if (value && json_is_string(value)) {
        return json_string_value(value);
    }
    return default_value ? default_value : "";
}

// Helper to get optional option value
static const char* get_optional_option(TdzCommand* command, const char* key) {
    json_t* value = json_object_get(command->options, key);
    if (value && json_is_string(value)) {
        return json_string_value(value);
    }
    return NULL;
}

// Build request body for different targets
json_t* build_request_body(TdzCommand* command) {
    if (strcmp(command->target, "task") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "action", json_string(get_option(command, "action", "")));
        json_object_set_new(obj, "time", json_string(get_option(command, "time", "")));
        json_object_set_new(obj, "priority", json_string(get_option(command, "priority", "")));
        json_object_set_new(obj, "project", json_string(get_option(command, "project", "")));
        json_object_set_new(obj, "status", json_string(get_option(command, "status", "")));
        
        const char* assignee = get_optional_option(command, "assignee");
        if (assignee) {
            json_object_set_new(obj, "assignee", json_string(assignee));
        }
        
        const char* tags = get_option(command, "tags", NULL);
        json_t* tags_array = tags ? comma_separated_to_array(tags) : json_array();
        if (tags_array) {
            json_object_set_new(obj, "tags", tags_array);
        }
        
        return obj;
    } else if (strcmp(command->target, "memory") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "moment", json_string(get_option(command, "moment", "")));
        json_object_set_new(obj, "meaning", json_string(get_option(command, "meaning", "")));
        json_object_set_new(obj, "reason", json_string(get_option(command, "reason", "")));
        json_object_set_new(obj, "importance", json_string(get_option(command, "importance", "")));
        json_object_set_new(obj, "term", json_string(get_option(command, "term", "")));
        json_object_set_new(obj, "memory_type", json_string(get_option(command, "memory_type", "")));
        
        const char* emotion = get_optional_option(command, "emotion");
        if (emotion) {
            json_object_set_new(obj, "emotion", json_string(emotion));
        }
        
        return obj;
    } else if (strcmp(command->target, "idea") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "idea", json_string(get_option(command, "idea", "")));
        json_object_set_new(obj, "share", json_string(get_option(command, "share", "")));
        json_object_set_new(obj, "importance", json_string(get_option(command, "importance", "")));
        return obj;
    } else if (strcmp(command->target, "agent") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "name", json_string(get_option(command, "name", "")));
        json_object_set_new(obj, "description", json_string(get_option(command, "description", "")));
        
        const char* capabilities = get_option(command, "capabilities", NULL);
        json_t* capabilities_array = capabilities ? comma_separated_to_array(capabilities) : json_array();
        if (capabilities_array) {
            json_object_set_new(obj, "capabilities", capabilities_array);
        }
        
        return obj;
    } else if (strcmp(command->target, "chunk") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "id", json_string(get_option(command, "id", "")));
        json_object_set_new(obj, "level", json_string(get_option(command, "level", "")));
        json_object_set_new(obj, "description", json_string(get_option(command, "description", "")));
        
        const char* dependencies = get_option(command, "dependencies", NULL);
        json_t* dependencies_array = dependencies ? comma_separated_to_array(dependencies) : json_array();
        if (dependencies_array) {
            json_object_set_new(obj, "dependencies", dependencies_array);
        }
        
        json_object_set_new(obj, "code", json_string(get_option(command, "code", "")));
        return obj;
    } else if (strcmp(command->target, "project") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "name", json_string(get_option(command, "name", "")));
        json_object_set_new(obj, "description", json_string(get_option(command, "description", "")));
        json_object_set_new(obj, "status", json_string(get_option(command, "status", "")));
        return obj;
    } else if (strcmp(command->target, "feeling") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "emotion", json_string(get_option(command, "emotion", "")));
        
        const char* intensity_str = get_option(command, "intensity", "5");
        int intensity = 5;
        if (intensity_str) {
            char* endptr;
            long val = strtol(intensity_str, &endptr, 10);
            if (*endptr == '\0') { // Successfully parsed
                intensity = (int)val;
                if (intensity < 0) intensity = 0;
                if (intensity > 10) intensity = 10;
            }
        }
        json_object_set_new(obj, "intensity", json_integer(intensity));
        
        json_object_set_new(obj, "description", json_string(get_option(command, "description", "")));
        
        const char* context = get_optional_option(command, "context");
        if (context) {
            json_object_set_new(obj, "context", json_string(context));
        }
        
        const char* tags = get_option(command, "tags", NULL);
        json_t* tags_array = tags ? comma_separated_to_array(tags) : json_array();
        if (tags_array) {
            json_object_set_new(obj, "tags", tags_array);
        }
        
        return obj;
    } else if (strcmp(command->target, "training") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "data_type", json_string(get_option(command, "data_type", "")));
        json_object_set_new(obj, "prompt", json_string(get_option(command, "prompt", "")));
        json_object_set_new(obj, "completion", json_string(get_option(command, "completion", "")));
        json_object_set_new(obj, "source", json_string(get_option(command, "source", "")));
        
        const char* context = get_optional_option(command, "context");
        if (context) {
            json_object_set_new(obj, "context", json_string(context));
        }
        
        const char* tags = get_option(command, "tags", NULL);
        json_t* tags_array = tags ? comma_separated_to_array(tags) : json_array();
        if (tags_array) {
            json_object_set_new(obj, "tags", tags_array);
        }
        
        const char* quality_score_str = get_option(command, "quality_score", NULL);
        if (quality_score_str) {
            char* endptr;
            float quality_score = strtof(quality_score_str, &endptr);
            if (*endptr == '\0') { // Successfully parsed
                json_object_set_new(obj, "quality_score", json_real(quality_score));
            }
        }
        
        return obj;
    }
    
    return json_null();
}

// Build run body
json_t* build_run_body(TdzCommand* command) {
    if (strcmp(command->target, "agent") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "message", json_string(get_option(command, "message", "")));
        
        const char* context = get_optional_option(command, "context");
        if (context) {
            json_object_set_new(obj, "context", json_string(context));
        }
        
        return obj;
    } else if (strcmp(command->target, "chat") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "message", json_string(get_option(command, "message", "")));
        
        const char* context = get_optional_option(command, "context");
        if (context) {
            json_object_set_new(obj, "context", json_string(context));
        }
        
        return obj;
    } else if (strcmp(command->target, "queue_start") == 0 || strcmp(command->target, "queue_end") == 0) {
        return json_null();
    } else if (strcmp(command->target, "api_check") == 0) {
        json_t* obj = json_object();
        if (!obj) return NULL;
        
        json_object_set_new(obj, "public_key", json_string(get_option(command, "public_key", "")));
        
        const char* private_key = get_optional_option(command, "private_key");
        if (private_key) {
            json_object_set_new(obj, "private_key", json_string(private_key));
        }
        
        return obj;
    }
    
    return json_null();
}

// Execute tdz command
TdzError* execute_tdz_command(TdzCommand* command, const char* base_url, const char* api_key, json_t** result) {
    if (result) *result = NULL;
    
    CURL* curl = NULL;
    struct MemoryStruct chunk = {0};
    struct curl_slist* headers = NULL;
    json_t* body = NULL;
    char* body_str = NULL;
    char* url = NULL;
    TdzError* error = NULL;
    
    chunk.memory = malloc(1);
    if (!chunk.memory) {
        return tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
    }
    chunk.size = 0;
    
    curl = curl_easy_init();
    if (!curl) {
        error = tdz_error_new(TDZ_ERROR_NETWORK, "Failed to initialize curl");
        goto cleanup;
    }
    
    // Validate base_url
    if (!base_url || strlen(base_url) == 0) {
        error = tdz_error_new(TDZ_ERROR_VALIDATION, "Base URL is required");
        goto cleanup;
    }
    
    // Build URL
    char* base_url_trimmed = string_duplicate(base_url);
    if (!base_url_trimmed) {
        error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
        goto cleanup;
    }
    
    size_t base_len = strlen(base_url_trimmed);
    if (base_len > 0 && base_url_trimmed[base_len - 1] == '/') {
        base_url_trimmed[base_len - 1] = '\0';
    }
    
    char* endpoint = get_endpoint_path(command);
    if (!endpoint) {
        free(base_url_trimmed);
        error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
        goto cleanup;
    }
    
    size_t url_len = strlen(base_url_trimmed) + strlen(endpoint) + 1;
    // Check for overflow
    if (url_len < strlen(base_url_trimmed)) {
        free(base_url_trimmed);
        free(endpoint);
        error = tdz_error_new(TDZ_ERROR_VALIDATION, "URL too long");
        goto cleanup;
    }
    
    url = malloc(url_len);
    if (!url) {
        free(base_url_trimmed);
        free(endpoint);
        error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
        goto cleanup;
    }
    
    int written = snprintf(url, url_len, "%s%s", base_url_trimmed, endpoint);
    free(base_url_trimmed);
    free(endpoint);
    
    if (written < 0 || (size_t)written >= url_len) {
        error = tdz_error_new(TDZ_ERROR_VALIDATION, "Failed to build URL");
        goto cleanup;
    }
    
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteMemoryCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void*)&chunk);
    
    // Set headers
    headers = curl_slist_append(headers, "Content-Type: application/json");
    if (api_key) {
        size_t header_len = strlen("X-API-Key: ") + strlen(api_key) + 1;
        char* header = malloc(header_len);
        if (!header) {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            goto cleanup;
        }
        
        written = snprintf(header, header_len, "X-API-Key: %s", api_key);
        if (written < 0 || (size_t)written >= header_len) {
            free(header);
            error = tdz_error_new(TDZ_ERROR_VALIDATION, "API key header too long");
            goto cleanup;
        }
        
        headers = curl_slist_append(headers, header);
        free(header);
        if (!headers) {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            goto cleanup;
        }
    }
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    
    // Set method and body based on command
    char* cmd = command->command;
    bool is_get = (strcmp(cmd, "list") == 0 || strcmp(cmd, "get") == 0 || strcmp(cmd, "search") == 0);
    bool is_post = (strcmp(cmd, "create") == 0 || strcmp(cmd, "run") == 0 || strcmp(cmd, "execute") == 0);
    bool is_put = (strcmp(cmd, "update") == 0);
    bool is_delete = (strcmp(cmd, "delete") == 0);
    
    if (is_get) {
        curl_easy_setopt(curl, CURLOPT_HTTPGET, 1L);
    } else if (is_post) {
        curl_easy_setopt(curl, CURLOPT_POST, 1L);
    } else if (is_put) {
        curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
    } else if (is_delete) {
        curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");
    } else {
        error = tdz_error_new(TDZ_ERROR_VALIDATION, "Unknown command");
        goto cleanup;
    }
    
    // Add body for create/update/run/execute
    if (strcmp(cmd, "create") == 0 || strcmp(cmd, "update") == 0) {
        body = build_request_body(command);
        if (!body) {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            goto cleanup;
        }
        body_str = json_dumps(body, JSON_COMPACT);
        if (!body_str) {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            goto cleanup;
        }
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body_str);
    } else if (strcmp(cmd, "run") == 0 || strcmp(cmd, "execute") == 0) {
        body = build_run_body(command);
        if (!body) {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            goto cleanup;
        }
        body_str = json_dumps(body, JSON_COMPACT);
        if (!body_str) {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
            goto cleanup;
        }
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body_str);
    }
    
    // Perform request
    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        error = tdz_error_new(TDZ_ERROR_NETWORK, curl_easy_strerror(res));
        goto cleanup;
    }
    
    // Check response code
    long response_code;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
    if (response_code < 200 || response_code >= 300) {
        // Limit error message size to prevent excessive memory usage
        size_t max_error_size = 512;
        size_t response_preview_size = (chunk.size > max_error_size) ? max_error_size : chunk.size;
        size_t error_msg_len = 64 + response_preview_size + 1;
        char* error_msg = malloc(error_msg_len);
        if (error_msg) {
            if (chunk.memory && chunk.size > 0) {
                // Truncate response if too long
                char* preview = malloc(response_preview_size + 1);
                if (preview) {
                    memcpy(preview, chunk.memory, response_preview_size);
                    preview[response_preview_size] = '\0';
                    snprintf(error_msg, error_msg_len, "HTTP error %ld: %s", response_code, preview);
                    free(preview);
                } else {
                    snprintf(error_msg, error_msg_len, "HTTP error %ld", response_code);
                }
            } else {
                snprintf(error_msg, error_msg_len, "HTTP error %ld", response_code);
            }
            error = tdz_error_new(TDZ_ERROR_VALIDATION, error_msg);
            free(error_msg);
        } else {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
        }
        goto cleanup;
    }
    
    // Parse JSON response
    json_error_t json_error;
    json_t* parsed_result = NULL;
    
    // Check if we have data to parse
    if (!chunk.memory || chunk.size == 0) {
        error = tdz_error_new(TDZ_ERROR_JSON, "Empty response from server");
        goto cleanup;
    }
    
    parsed_result = json_loads(chunk.memory, 0, &json_error);
    if (!parsed_result) {
        size_t error_msg_len = 64 + strlen(json_error.text);
        char* error_msg = malloc(error_msg_len);
        if (error_msg) {
            snprintf(error_msg, error_msg_len, "JSON parse error: %s", json_error.text);
            error = tdz_error_new(TDZ_ERROR_JSON, error_msg);
            free(error_msg);
        } else {
            error = tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
        }
        goto cleanup;
    }
    
    // Only set result if we successfully parsed and result pointer is provided
    if (result) {
        *result = parsed_result;
    } else {
        // Caller didn't want the result, so clean it up
        json_decref(parsed_result);
    }
    
cleanup:
    // Clean up resources
    if (body) json_decref(body);
    if (body_str) free(body_str);
    if (headers) curl_slist_free_all(headers);
    if (curl) curl_easy_cleanup(curl);
    if (url) free(url);
    if (chunk.memory) free(chunk.memory);
    
    return error;
}

// Process tdz commands
TdzError* process_tdz_commands(const char* text, const char* base_url, const char* api_key, json_t** results) {
    if (results) *results = NULL;
    
    TdzError* error = NULL;
    
    // Parse commands
    TdzCommandVector* commands = parse_tdz_command(text, &error);
    if (!commands) {
        return error;
    }
    
    // Create results array
    *results = json_array();
    if (!*results) {
        tdz_command_vector_free(commands);
        return tdz_error_new(TDZ_ERROR_OOM, "Memory allocation failed");
    }
    
    // Execute each command
    for (size_t i = 0; i < commands->count; i++) {
        json_t* result = NULL;
        error = execute_tdz_command(&commands->commands[i], base_url, api_key, &result);
        if (error) {
            tdz_command_vector_free(commands);
            json_decref(*results);
            *results = NULL;
            return error;
        }
        json_array_append_new(*results, result);
    }
    
    tdz_command_vector_free(commands);
    return NULL;
}

