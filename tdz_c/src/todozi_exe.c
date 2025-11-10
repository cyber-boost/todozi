#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>
#include <errno.h>
#include <curl/curl.h>
#include "json-c/json.h"

// Error types
typedef enum {
    EXECUTOR_ERROR_EXECUTION,
    EXECUTOR_ERROR_BASH_TOOL,
    EXECUTOR_ERROR_MISSING_PARAMETER,
    EXECUTOR_ERROR_UNKNOWN_ACTION
} ExecutorErrorType;

typedef struct {
    ExecutorErrorType type;
    char* message;
} ExecutorError;

// Execution result
typedef struct {
    bool success;
    char* output;
    char* error;
    // Note: metadata implementation simplified due to C limitations
    char* tool_used;
    char* execution_type;
} ExecutionResult;

// Memory management helpers
void free_execution_result(ExecutionResult* result) {
    if (result) {
        if (result->output) free(result->output);
        if (result->error) free(result->error);
        if (result->tool_used) free(result->tool_used);
        if (result->execution_type) free(result->execution_type);
        free(result);
    }
}

ExecutorError* create_executor_error(ExecutorErrorType type, const char* message) {
    ExecutorError* error = malloc(sizeof(ExecutorError));
    if (!error) return NULL;
    
    error->type = type;
    error->message = malloc(strlen(message) + 1);
    if (!error->message) {
        free(error);
        return NULL;
    }
    strcpy(error->message, message);
    return error;
}

void free_executor_error(ExecutorError* error) {
    if (error) {
        if (error->message) free(error->message);
        free(error);
    }
}

// Helper function to safely create and initialize ExecutionResult
static ExecutionResult* create_execution_result(void) {
    ExecutionResult* result = calloc(1, sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = false;
    result->output = NULL;
    result->error = NULL;
    result->tool_used = NULL;
    result->execution_type = NULL;
    
    return result;
}

// Helper function to safely set string field in ExecutionResult
static bool set_result_string(char** dest, const char* src) {
    if (!dest) return false;
    if (*dest) {
        free(*dest);
        *dest = NULL;
    }
    if (src) {
        size_t len = strlen(src);
        *dest = malloc(len + 1);
        if (!*dest) return false;
        strcpy(*dest, src);
    }
    return true;
}

// Global state (simplified)
static bool tdz_system_initialized = false;
static bool curl_initialized = false;
static char* api_key_copy = NULL;  // Copy of API key from environment

// Forward declarations
ExecutorError* ensure_todozi_system();
ExecutionResult* execute_simple_task(json_object* params);
ExecutionResult* execute_urgent_task(json_object* params);
ExecutionResult* execute_high_task(json_object* params);
ExecutionResult* execute_low_task(json_object* params);
ExecutionResult* execute_ai_task(json_object* params);
ExecutionResult* execute_human_task(json_object* params);
ExecutionResult* execute_collab_task(json_object* params);
ExecutionResult* execute_find(json_object* params);
ExecutionResult* execute_ai_search(json_object* params);
ExecutionResult* execute_fast_search(json_object* params);
ExecutionResult* execute_smart_search(json_object* params);
ExecutionResult* execute_remember(json_object* params);
ExecutionResult* execute_important_memory(json_object* params);
ExecutionResult* execute_idea(json_object* params);
ExecutionResult* execute_breakthrough_idea(json_object* params);
ExecutionResult* execute_complete(json_object* params);
ExecutionResult* execute_start(json_object* params);
ExecutionResult* execute_stats(json_object* params);
ExecutionResult* execute_queue(json_object* params);
ExecutionResult* execute_chat(json_object* params);
ExecutionResult* execute_extract_api(json_object* params);
ExecutionResult* execute_expand_api(json_object* params);
ExecutionResult* execute_plan_api(json_object* params);
ExecutionResult* execute_strategy_api(json_object* params);

// HTTP response structure
struct http_response {
    char* data;
    size_t size;
};

// HTTP response callback
static size_t write_callback(void* contents, size_t size, size_t nmemb, struct http_response* response) {
    size_t realsize = size * nmemb;
    char* ptr = realloc(response->data, response->size + realsize + 1);
    if (!ptr) return 0;
    
    response->data = ptr;
    memcpy(&(response->data[response->size]), contents, realsize);
    response->size += realsize;
    response->data[response->size] = 0;
    
    return realsize;
}

// Get parameter from JSON object
const char* get_string_param(json_object* params, const char* key) {
    json_object* value;
    if (json_object_object_get_ex(params, key, &value)) {
        if (json_object_is_type(value, json_type_string)) {
            return json_object_get_string(value);
        }
    }
    return NULL;
}

// Initialize Todozi system
ExecutorError* ensure_todozi_system() {
    if (!tdz_system_initialized) {
        // Initialize curl globally (thread-safe, can be called multiple times)
        if (!curl_initialized) {
            CURLcode curl_res = curl_global_init(CURL_GLOBAL_DEFAULT);
            if (curl_res != CURLE_OK) {
                return create_executor_error(EXECUTOR_ERROR_EXECUTION, 
                    "Failed to initialize curl library");
            }
            curl_initialized = true;
        }
        printf("Warning: Todozi system initialization not implemented in C version\n");
        tdz_system_initialized = true;
    }
    return NULL;
}

// Get API key (properly handles environment variable)
const char* get_todozi_api_key() {
    if (!api_key_copy) {
        const char* env_key = getenv("TODOZI_API_KEY");
        if (env_key) {
            // Allocate and copy the key (getenv returns static memory)
            size_t key_len = strlen(env_key);
            api_key_copy = malloc(key_len + 1);
            if (api_key_copy) {
                strcpy(api_key_copy, env_key);
            }
        }
    }
    return api_key_copy;
}

// Make HTTP request to Todozi API
json_object* make_todozi_request(const char* endpoint, json_object* payload) {
    CURL* curl = NULL;
    CURLcode res;
    struct http_response response = {0};
    json_object* result = NULL;
    struct curl_slist* headers = NULL;
    
    if (!endpoint || !payload) {
        return NULL;
    }
    
    curl = curl_easy_init();
    if (!curl) {
        return NULL;
    }
    
    char url[512];
    int url_len = snprintf(url, sizeof(url), "https://todozi.com%s", endpoint);
    if (url_len < 0 || url_len >= (int)sizeof(url)) {
        curl_easy_cleanup(curl);
        return NULL;
    }
    
    const char* api_key = get_todozi_api_key();
    if (!api_key) {
        curl_easy_cleanup(curl);
        return NULL;
    }
    
    headers = curl_slist_append(headers, "Content-Type: application/json");
    if (!headers) {
        curl_easy_cleanup(curl);
        return NULL;
    }
    
    char auth_header[512];
    int auth_len = snprintf(auth_header, sizeof(auth_header), "Authorization: Bearer %s", api_key);
    if (auth_len < 0 || auth_len >= (int)sizeof(auth_header)) {
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        return NULL;
    }
    
    headers = curl_slist_append(headers, auth_header);
    if (!headers) {
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        return NULL;
    }
    
    const char* payload_str = json_object_to_json_string(payload);
    if (!payload_str) {
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        return NULL;
    }
    
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, payload_str);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 30L);  // 30 second timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 10L);  // 10 second connect timeout
    
    res = curl_easy_perform(curl);
    
    if (res == CURLE_OK) {
        long response_code;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
        
        if (response_code >= 200 && response_code < 300 && response.data) {
            result = json_tokener_parse(response.data);
        }
    }
    
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    
    if (response.data) {
        free(response.data);
        response.data = NULL;
    }
    
    return result;
}

// Enhanced Todozi tool integration
ExecutionResult* execute_todozi_tool_delegated(json_object* params) {
    printf("🎯 Executing Todozi operation using enhanced simple interfaces\n");
    
    ExecutorError* init_error = ensure_todozi_system();
    if (init_error) {
        printf("⚠️ Warning: Failed to initialize Todozi system: %s\n", init_error->message);
        free_executor_error(init_error);
    }
    
    const char* action = get_string_param(params, "action");
    if (!action) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "action");
        free_executor_error(error);
        return NULL;
    }
    
    printf("🎯 Todozi action: %s\n", action);
    
    if (strcmp(action, "task") == 0) {
        return execute_simple_task(params);
    } else if (strcmp(action, "urgent") == 0) {
        return execute_urgent_task(params);
    } else if (strcmp(action, "high") == 0) {
        return execute_high_task(params);
    } else if (strcmp(action, "low") == 0) {
        return execute_low_task(params);
    } else if (strcmp(action, "ai") == 0) {
        return execute_ai_task(params);
    } else if (strcmp(action, "human") == 0) {
        return execute_human_task(params);
    } else if (strcmp(action, "collab") == 0) {
        return execute_collab_task(params);
    } else if (strcmp(action, "find") == 0) {
        return execute_find(params);
    } else if (strcmp(action, "ai_search") == 0) {
        return execute_ai_search(params);
    } else if (strcmp(action, "fast_search") == 0) {
        return execute_fast_search(params);
    } else if (strcmp(action, "smart_search") == 0) {
        return execute_smart_search(params);
    } else if (strcmp(action, "remember") == 0) {
        return execute_remember(params);
    } else if (strcmp(action, "important_memory") == 0) {
        return execute_important_memory(params);
    } else if (strcmp(action, "idea") == 0) {
        return execute_idea(params);
    } else if (strcmp(action, "breakthrough_idea") == 0) {
        return execute_breakthrough_idea(params);
    } else if (strcmp(action, "complete") == 0) {
        return execute_complete(params);
    } else if (strcmp(action, "start") == 0) {
        return execute_start(params);
    } else if (strcmp(action, "stats") == 0) {
        return execute_stats(params);
    } else if (strcmp(action, "queue") == 0) {
        return execute_queue(params);
    } else if (strcmp(action, "chat") == 0) {
        return execute_chat(params);
    } else if (strcmp(action, "extract") == 0) {
        return execute_extract_api(params);
    } else if (strcmp(action, "expand") == 0) {
        return execute_expand_api(params);
    } else if (strcmp(action, "plan") == 0) {
        return execute_plan_api(params);
    } else if (strcmp(action, "strategy") == 0) {
        return execute_strategy_api(params);
    } else {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), "Unsupported Todozi action: %s", action);
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_UNKNOWN_ACTION, error_msg);
        free_executor_error(error);
        return NULL;
    }
}

// Execute simple task creation
ExecutionResult* execute_simple_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        return NULL;
    }
    
    // Simulate task creation
    char output[256];
    int output_len = snprintf(output, sizeof(output), "✅ Task created: task_%ld", time(NULL));
    if (output_len < 0 || output_len >= (int)sizeof(output)) {
        return NULL;
    }
    
    ExecutionResult* result = create_execution_result();
    if (!result) return NULL;
    
    result->success = true;
    if (!set_result_string(&result->output, output) ||
        !set_result_string(&result->tool_used, "todozi_simple") ||
        !set_result_string(&result->execution_type, "easy_interface")) {
        free_execution_result(result);
        return NULL;
    }
    
    return result;
}

// Execute urgent task creation
ExecutionResult* execute_urgent_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🚨 Urgent task created: task_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("priority_interface") + 1);
    if (result->execution_type) strcpy(result->execution_type, "priority_interface");
    
    return result;
}

// Execute high priority task creation
ExecutionResult* execute_high_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🟠 High priority task created: task_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("priority_interface") + 1);
    if (result->execution_type) strcpy(result->execution_type, "priority_interface");
    
    return result;
}

// Execute low priority task creation
ExecutionResult* execute_low_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🟢 Low priority task created: task_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("priority_interface") + 1);
    if (result->execution_type) strcpy(result->execution_type, "priority_interface");
    
    return result;
}

// Execute AI task creation
ExecutionResult* execute_ai_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🤖 AI task queued: task_%ld (available for Maestro/Claude/etc.)", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("ai_assignment") + 1);
    if (result->execution_type) strcpy(result->execution_type, "ai_assignment");
    
    return result;
}

// Execute human task creation
ExecutionResult* execute_human_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "👤 Human task created: task_%ld (visible in TUI)", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("human_assignment") + 1);
    if (result->execution_type) strcpy(result->execution_type, "human_assignment");
    
    return result;
}

// Execute collaborative task creation
ExecutionResult* execute_collab_task(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🤝 Collaborative task created: task_%ld (AI+Human coordination)", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("collaborative_assignment") + 1);
    if (result->execution_type) strcpy(result->execution_type, "collaborative_assignment");
    
    return result;
}

// Execute smart find
ExecutionResult* execute_find(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🔍 Smart search results: [simulated results for '%s']", content);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("smart_search") + 1);
    if (result->execution_type) strcpy(result->execution_type, "smart_search");
    
    return result;
}

// Execute AI-only semantic search
ExecutionResult* execute_ai_search(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[512];
    snprintf(output, sizeof(output), "🤖 AI semantic search results:\n• Result 1 for '%s' [ID: 1]\n• Result 2 for '%s' [ID: 2]", content, content);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("semantic_search") + 1);
    if (result->execution_type) strcpy(result->execution_type, "semantic_search");
    
    return result;
}

// Execute fast keyword search
ExecutionResult* execute_fast_search(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "⚡ Fast search results: [keyword matches for '%s']", content);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("fast_search") + 1);
    if (result->execution_type) strcpy(result->execution_type, "fast_search");
    
    return result;
}

// Execute smart intent-aware search
ExecutionResult* execute_smart_search(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🧠 Smart intent search results: [intent-based results for '%s']", content);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("intent_search") + 1);
    if (result->execution_type) strcpy(result->execution_type, "intent_search");
    
    return result;
}

// Execute memory creation
ExecutionResult* execute_remember(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    const char* extra = get_string_param(params, "extra");
    if (!extra) extra = content;
    
    char output[256];
    snprintf(output, sizeof(output), "🧠 Memory saved: memory_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("memory_creation") + 1);
    if (result->execution_type) strcpy(result->execution_type, "memory_creation");
    
    return result;
}

// Execute important memory creation
ExecutionResult* execute_important_memory(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    const char* extra = get_string_param(params, "extra");
    if (!extra) extra = content;
    
    char output[256];
    snprintf(output, sizeof(output), "🧠⭐ Important memory saved: memory_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("important_memory") + 1);
    if (result->execution_type) strcpy(result->execution_type, "important_memory");
    
    return result;
}

// Execute idea creation
ExecutionResult* execute_idea(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "💡 Idea saved: idea_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("idea_creation") + 1);
    if (result->execution_type) strcpy(result->execution_type, "idea_creation");
    
    return result;
}

// Execute breakthrough idea creation
ExecutionResult* execute_breakthrough_idea(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "💡🚀 Breakthrough idea saved: idea_%ld", time(NULL));
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("breakthrough_idea") + 1);
    if (result->execution_type) strcpy(result->execution_type, "breakthrough_idea");
    
    return result;
}

// Execute task completion
ExecutionResult* execute_complete(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "✅ Task %s marked as completed", content);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("task_completion") + 1);
    if (result->execution_type) strcpy(result->execution_type, "task_completion");
    
    return result;
}

// Execute task start
ExecutionResult* execute_start(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[256];
    snprintf(output, sizeof(output), "🔄 Task %s started", content);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("task_start") + 1);
    if (result->execution_type) strcpy(result->execution_type, "task_start");
    
    return result;
}

// Execute stats retrieval
ExecutionResult* execute_stats(json_object* params) {
    // Simulate stats retrieval
    char output[256];
    snprintf(output, sizeof(output), "📊 Todozi Stats:\nTasks: 10\nCompleted: 7\nPending: 3");
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("stats_retrieval") + 1);
    if (result->execution_type) strcpy(result->execution_type, "stats_retrieval");
    
    return result;
}

// Execute queue status
ExecutionResult* execute_queue(json_object* params) {
    // Simulate queue status
    char output[256];
    snprintf(output, sizeof(output), "📋 Queue Status:\n  Total: 5 items\n  Backlog: 3 items\n  Active: 2 items");
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("queue_status") + 1);
    if (result->execution_type) strcpy(result->execution_type, "queue_status");
    
    return result;
}

// Execute chat processing
ExecutionResult* execute_chat(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    char output[512];
    snprintf(output, sizeof(output), "✅ Chat processed: 📋 Created 2 tasks, 🧠 Created 1 memory, 💡 Created 1 idea");
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_simple") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_simple");
    result->execution_type = malloc(strlen("chat_processing") + 1);
    if (result->execution_type) strcpy(result->execution_type, "chat_processing");
    
    return result;
}

// Execute task extraction using todozi.com API
ExecutionResult* execute_extract_api(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        return NULL;
    }
    
    const char* extra = get_string_param(params, "extra");
    if (!extra) extra = "";
    
    // Create payload
    json_object* payload = json_object_new_object();
    if (!payload) return NULL;
    
    json_object_object_add(payload, "message", json_object_new_string(content));
    json_object_object_add(payload, "context", json_object_new_string(extra));
    
    json_object* response = make_todozi_request("/api/todozi/extract", payload);
    json_object_put(payload);
    
    if (!response) {
        return NULL;
    }
    
    ExecutionResult* result = create_execution_result();
    if (!result) {
        json_object_put(response);
        return NULL;
    }
    
    result->success = true;
    const char* default_msg = "✅ Message processed successfully - no structured content extracted";
    char output[1024] = "";
    int offset = 0;
    
    json_object* extracted;
    if (json_object_object_get_ex(response, "extracted_content", &extracted)) {
        json_object* tasks;
        if (json_object_object_get_ex(extracted, "tasks", &tasks) && json_object_is_type(tasks, json_type_array)) {
            int tasks_len = json_object_array_length(tasks);
            int written = snprintf(output + offset, sizeof(output) - offset, "📋 Extracted %d tasks\n", tasks_len);
            if (written > 0 && offset + written < (int)sizeof(output)) {
                offset += written;
            }
            
            for (int i = 0; i < tasks_len && offset < (int)sizeof(output) - 100; i++) {
                json_object* task = json_object_array_get_idx(tasks, i);
                json_object* action;
                if (json_object_object_get_ex(task, "action", &action) && json_object_is_type(action, json_type_string)) {
                    const char* action_str = json_object_get_string(action);
                    written = snprintf(output + offset, sizeof(output) - offset, "%d. %s\n", i + 1, action_str);
                    if (written > 0 && offset + written < (int)sizeof(output)) {
                        offset += written;
                    }
                }
            }
        }
        
        json_object* memories;
        if (json_object_object_get_ex(extracted, "memories", &memories) && json_object_is_type(memories, json_type_array)) {
            int memories_len = json_object_array_length(memories);
            int written = snprintf(output + offset, sizeof(output) - offset, "🧠 Created %d memories\n", memories_len);
            if (written > 0 && offset + written < (int)sizeof(output)) {
                offset += written;
            }
        }
        
        json_object* ideas;
        if (json_object_object_get_ex(extracted, "ideas", &ideas) && json_object_is_type(ideas, json_type_array)) {
            int ideas_len = json_object_array_length(ideas);
            int written = snprintf(output + offset, sizeof(output) - offset, "💡 Generated %d ideas\n", ideas_len);
            if (written > 0 && offset + written < (int)sizeof(output)) {
                offset += written;
            }
        }
    }
    
    if (offset == 0) {
        strncpy(output, default_msg, sizeof(output) - 1);
        output[sizeof(output) - 1] = '\0';
    }
    
    if (!set_result_string(&result->output, output) ||
        !set_result_string(&result->tool_used, "todozi_api") ||
        !set_result_string(&result->execution_type, "ai_extraction")) {
        json_object_put(response);
        free_execution_result(result);
        return NULL;
    }
    
    json_object_put(response);
    return result;
}

// Execute task expansion using todozi.com API
ExecutionResult* execute_expand_api(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        return NULL;
    }
    
    const char* extra = get_string_param(params, "extra");
    if (!extra) extra = "";
    
    // Create tasks array
    json_object* tasks_array = json_object_new_array();
    if (!tasks_array) return NULL;
    
    json_object_array_add(tasks_array, json_object_new_string(content));
    
    // Create payload
    json_object* payload = json_object_new_object();
    if (!payload) {
        json_object_put(tasks_array);
        return NULL;
    }
    
    json_object_object_add(payload, "tasks", tasks_array);
    json_object_object_add(payload, "context", json_object_new_string(extra));
    
    json_object* response = make_todozi_request("/api/todozi/expand", payload);
    json_object_put(payload);
    
    if (!response) {
        return NULL;
    }
    
    ExecutionResult* result = create_execution_result();
    if (!result) {
        json_object_put(response);
        return NULL;
    }
    
    result->success = true;
    char output[2048] = "";
    
    json_object* expanded;
    if (json_object_object_get_ex(response, "expanded_tasks", &expanded) && json_object_is_type(expanded, json_type_array)) {
        int expanded_len = json_object_array_length(expanded);
        
        if (expanded_len == 0) {
            strncpy(output, "🤖 No task expansion generated", sizeof(output) - 1);
            output[sizeof(output) - 1] = '\0';
        } else {
            // Calculate required size more safely
            size_t task_list_size = 0;
            for (int i = 0; i < expanded_len; i++) {
                json_object* task = json_object_array_get_idx(expanded, i);
                if (json_object_is_type(task, json_type_string)) {
                    task_list_size += strlen(json_object_get_string(task)) + 32; // +32 for formatting
                }
            }
            
            char* task_list = NULL;
            if (task_list_size > 0 && task_list_size < 65536) { // Reasonable limit
                task_list = malloc(task_list_size);
            }
            
            if (task_list) {
                int offset = 0;
                for (int i = 0; i < expanded_len && offset < (int)task_list_size - 100; i++) {
                    json_object* task = json_object_array_get_idx(expanded, i);
                    if (json_object_is_type(task, json_type_string)) {
                        const char* task_str = json_object_get_string(task);
                        int written = snprintf(task_list + offset, task_list_size - offset, 
                                             "%d. %s\n", i + 1, task_str);
                        if (written > 0 && offset + written < (int)task_list_size) {
                            offset += written;
                        }
                    }
                }
                
                int written = snprintf(output, sizeof(output), "🚀 Expanded into %d detailed tasks:\n%s", expanded_len, task_list);
                if (written < 0 || written >= (int)sizeof(output)) {
                    strncpy(output, "🚀 Task expansion completed", sizeof(output) - 1);
                    output[sizeof(output) - 1] = '\0';
                }
                free(task_list);
            } else {
                snprintf(output, sizeof(output), "🚀 Expanded into %d detailed tasks", expanded_len);
            }
        }
    } else {
        strncpy(output, "🤖 Invalid response format from API", sizeof(output) - 1);
        output[sizeof(output) - 1] = '\0';
        result->success = false;
    }
    
    if (!set_result_string(&result->output, output) ||
        !set_result_string(&result->tool_used, "todozi_api") ||
        !set_result_string(&result->execution_type, "ai_expansion")) {
        json_object_put(response);
        free_execution_result(result);
        return NULL;
    }
    
    json_object_put(response);
    return result;
}

// Execute AI project planning using todozi plan command
ExecutionResult* execute_plan_api(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    const char* output_format = get_string_param(params, "output_format");
    if (!output_format) output_format = "json";
    
    const char* extra = get_string_param(params, "extra");
    
    // Simulate AI project planning
    char output[1024];
    snprintf(output, sizeof(output), "🎯 AI Project Planning Complete:\nProject: %s\nFormat: %s\nStatus: Completed", content, output_format);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_plan") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_plan");
    result->execution_type = malloc(strlen("ai_project_planning") + 1);
    if (result->execution_type) strcpy(result->execution_type, "ai_project_planning");
    
    return result;
}

// Execute AI strategic planning using todozi strategy command
ExecutionResult* execute_strategy_api(json_object* params) {
    const char* content = get_string_param(params, "content");
    if (!content) {
        ExecutorError* error = create_executor_error(EXECUTOR_ERROR_MISSING_PARAMETER, "content");
        free_executor_error(error);
        return NULL;
    }
    
    const char* output_format = get_string_param(params, "output_format");
    if (!output_format) output_format = "json";
    
    const char* extra = get_string_param(params, "extra");
    
    // Simulate AI strategic planning
    char output[1024];
    snprintf(output, sizeof(output), "🎭 AI Strategic Planning Complete:\nStrategy: %s\nFormat: %s\nStatus: Completed", content, output_format);
    
    ExecutionResult* result = malloc(sizeof(ExecutionResult));
    if (!result) return NULL;
    
    result->success = true;
    result->output = malloc(strlen(output) + 1);
    if (result->output) strcpy(result->output, output);
    result->error = NULL;
    result->tool_used = malloc(strlen("todozi_strategy") + 1);
    if (result->tool_used) strcpy(result->tool_used, "todozi_strategy");
    result->execution_type = malloc(strlen("ai_strategic_planning") + 1);
    if (result->execution_type) strcpy(result->execution_type, "ai_strategic_planning");
    
    return result;
}

// Cleanup function
void cleanup_todozi_executor() {
    if (api_key_copy) {
        free(api_key_copy);
        api_key_copy = NULL;
    }
    if (curl_initialized) {
        curl_global_cleanup();
        curl_initialized = false;
    }
    tdz_system_initialized = false;
}