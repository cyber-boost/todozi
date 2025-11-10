#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include "json-c/json.h"
#include <uuid/uuid.h>
#include <time.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>

// Error handling
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_IO,
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_CONFIG,
    TODOZI_ERROR_API,
    TODOZI_ERROR_SERIALIZATION
} TodoziError;

// Priority enum
typedef enum {
    PRIORITY_LOW,
    PRIORITY_MEDIUM,
    PRIORITY_HIGH,
    PRIORITY_CRITICAL
} Priority;

// Status enum
typedef enum {
    STATUS_TODO,
    STATUS_IN_PROGRESS,
    STATUS_DONE,
    STATUS_CANCELLED
} Status;

// Memory importance enum
typedef enum {
    MEMORY_IMPORTANCE_LOW,
    MEMORY_IMPORTANCE_MEDIUM,
    MEMORY_IMPORTANCE_HIGH,
    MEMORY_IMPORTANCE_CRITICAL
} MemoryImportance;

// Memory term enum
typedef enum {
    MEMORY_TERM_SHORT,
    MEMORY_TERM_MEDIUM,
    MEMORY_TERM_LONG
} MemoryTerm;

// Share level enum
typedef enum {
    SHARE_LEVEL_PRIVATE,
    SHARE_LEVEL_TEAM,
    SHARE_LEVEL_PUBLIC
} ShareLevel;

// Idea importance enum
typedef enum {
    IDEA_IMPORTANCE_LOW,
    IDEA_IMPORTANCE_MEDIUM,
    IDEA_IMPORTANCE_HIGH,
    IDEA_IMPORTANCE_CRITICAL
} IdeaImportance;

// Item status enum
typedef enum {
    ITEM_STATUS_ACTIVE,
    ITEM_STATUS_ARCHIVED,
    ITEM_STATUS_DELETED
} ItemStatus;

// Memory type enum
typedef enum {
    MEMORY_TYPE_STANDARD,
    MEMORY_TYPE_LEARNING,
    MEMORY_TYPE_EXPERIENCE
} MemoryType;

// Extracted structures
typedef struct {
    char* action;
    char* time;
    char* priority;
    char* project;
    char* status;
    char* assignee;
    char** tags;
    int tags_count;
} ExtractedTask;

typedef struct {
    char* moment;
    char* meaning;
    char* reason;
    char* importance;
    char* term;
} ExtractedMemory;

typedef struct {
    char* idea;
    char* share;
    char* importance;
} ExtractedIdea;

typedef struct {
    char* title;
    char* description;
    char* severity;
    char* category;
} ExtractedError;

typedef struct {
    char* prompt;
    char* completion;
    char* data_type;
} ExtractedTrainingData;

typedef struct {
    ExtractedTask* tasks;
    int tasks_count;
    ExtractedMemory* memories;
    int memories_count;
    ExtractedIdea* ideas;
    int ideas_count;
    ExtractedError* errors;
    int errors_count;
    ExtractedTrainingData* training_data;
    int training_data_count;
    char** raw_tags;
    int raw_tags_count;
} ExtractResponse;

// Task structure
typedef struct {
    char* id;
    char* user_id;
    char* action;
    char* time;
    Priority priority;
    char* parent_project;
    Status status;
    char* assignee;
    char** tags;
    int tags_count;
    char** dependencies;
    int dependencies_count;
    char* context;
    int progress;
} Task;

// Memory structure
typedef struct {
    char* id;
    char* user_id;
    char* project_id;
    ItemStatus status;
    char* moment;
    char* meaning;
    char* reason;
    MemoryImportance importance;
    MemoryTerm term;
    MemoryType memory_type;
    char** tags;
    int tags_count;
    time_t created_at;
    time_t updated_at;
} Memory;

// Idea structure
typedef struct {
    char* id;
    char* idea;
    char* project_id;
    ItemStatus status;
    ShareLevel share;
    IdeaImportance importance;
    char** tags;
    int tags_count;
    char* context;
    time_t created_at;
    time_t updated_at;
} Idea;

// String buffer for curl response
struct string {
    char *ptr;
    size_t len;
};

// Forward declarations
TodoziError extract_content(const char* content, const char* file_path, const char* output_format, int human, char** result);
TodoziError strategy_content(const char* content, const char* file_path, const char* output_format, int human, char** result);
TodoziError extract_with_endpoint(const char* content, const char* file_path, const char* output_format, int human, const char* endpoint, char** result);
TodoziError format_as_csv(const ExtractResponse* response, char** result);
TodoziError format_as_markdown(const ExtractResponse* response, char** result);
TodoziError format_as_human_checklist(const ExtractResponse* response, char** result);
TodoziError log_to_history(const Task* task);
void init_string(struct string *s);
size_t writefunc(void *ptr, size_t size, size_t nmemb, struct string *s);
void free_extracted_task(ExtractedTask* task);
void free_extracted_memory(ExtractedMemory* memory);
void free_extracted_idea(ExtractedIdea* idea);
void free_extracted_error(ExtractedError* error);
void free_extracted_training_data(ExtractedTrainingData* data);
void free_extract_response(ExtractResponse* response);
char* read_file_to_string(const char* file_path, TodoziError* error);
static char* get_api_key(TodoziError* error);
static char* get_home_directory(TodoziError* error);
char* load_hlx_config(const char* config_path, TodoziError* error);
char* get_hlx_value(const char* hlx_content, const char* section, const char* key);
char* hash_project_name(const char* project_name);
Task* create_task(const char* user_id, const char* action, const char* time, Priority priority, const char* project_id, Status status, const char* assignee, char** tags, int tags_count, char** dependencies, int dependencies_count, const char* context, int progress, TodoziError* error);
Memory* create_memory(const char* user_id, const char* project_id, const char* moment, const char* meaning, const char* reason, MemoryImportance importance, MemoryTerm term, MemoryType memory_type, char** tags, int tags_count, TodoziError* error);
Idea* create_idea(const char* idea_text, const char* project_id, ShareLevel share, IdeaImportance importance, char** tags, int tags_count, const char* context, TodoziError* error);
static char* generate_uuid();
void* safe_malloc(size_t size);
char* safe_strdup(const char* str);
char** safe_strdup_array(char** src, int count);
int safe_strcmp(const char* a, const char* b);
char* safe_strcat(const char* a, const char* b);
char* get_current_timestamp();

// Implementation

void init_string(struct string *s) {
    s->len = 0;
    s->ptr = malloc(s->len+1);
    if (s->ptr == NULL) {
        fprintf(stderr, "malloc() failed\n");
        exit(EXIT_FAILURE);
    }
    s->ptr[0] = '\0';
}

size_t writefunc(void *ptr, size_t size, size_t nmemb, struct string *s) {
    size_t new_len = s->len + size*nmemb;
    s->ptr = realloc(s->ptr, new_len+1);
    if (s->ptr == NULL) {
        fprintf(stderr, "realloc() failed\n");
        exit(EXIT_FAILURE);
    }
    memcpy(s->ptr+s->len, ptr, size*nmemb);
    s->ptr[new_len] = '\0';
    s->len = new_len;

    return size*nmemb;
}

// Helper functions to free individual struct fields (not the struct itself)
static void free_extracted_task_fields(ExtractedTask* task) {
    if (task) {
        if (task->action) free(task->action);
        if (task->time) free(task->time);
        if (task->priority) free(task->priority);
        if (task->project) free(task->project);
        if (task->status) free(task->status);
        if (task->assignee) free(task->assignee);
        if (task->tags) {
            for (int i = 0; i < task->tags_count; i++) {
                if (task->tags[i]) free(task->tags[i]);
            }
            free(task->tags);
        }
    }
}

static void free_extracted_memory_fields(ExtractedMemory* memory) {
    if (memory) {
        if (memory->moment) free(memory->moment);
        if (memory->meaning) free(memory->meaning);
        if (memory->reason) free(memory->reason);
        if (memory->importance) free(memory->importance);
        if (memory->term) free(memory->term);
    }
}

static void free_extracted_idea_fields(ExtractedIdea* idea) {
    if (idea) {
        if (idea->idea) free(idea->idea);
        if (idea->share) free(idea->share);
        if (idea->importance) free(idea->importance);
    }
}

static void free_extracted_error_fields(ExtractedError* error) {
    if (error) {
        if (error->title) free(error->title);
        if (error->description) free(error->description);
        if (error->severity) free(error->severity);
        if (error->category) free(error->category);
    }
}

static void free_extracted_training_data_fields(ExtractedTrainingData* data) {
    if (data) {
        if (data->prompt) free(data->prompt);
        if (data->completion) free(data->completion);
        if (data->data_type) free(data->data_type);
    }
}

// Legacy functions for backward compatibility (free individual structs)
void free_extracted_task(ExtractedTask* task) {
    if (task) {
        free_extracted_task_fields(task);
        free(task);
    }
}

void free_extracted_memory(ExtractedMemory* memory) {
    if (memory) {
        free_extracted_memory_fields(memory);
        free(memory);
    }
}

void free_extracted_idea(ExtractedIdea* idea) {
    if (idea) {
        free_extracted_idea_fields(idea);
        free(idea);
    }
}

void free_extracted_error(ExtractedError* error) {
    if (error) {
        free_extracted_error_fields(error);
        free(error);
    }
}

void free_extracted_training_data(ExtractedTrainingData* data) {
    if (data) {
        free_extracted_training_data_fields(data);
        free(data);
    }
}

void free_extract_response(ExtractResponse* response) {
    if (response) {
        if (response->tasks) {
            for (int i = 0; i < response->tasks_count; i++) {
                free_extracted_task_fields(&response->tasks[i]);
            }
            free(response->tasks);
        }
        if (response->memories) {
            for (int i = 0; i < response->memories_count; i++) {
                free_extracted_memory_fields(&response->memories[i]);
            }
            free(response->memories);
        }
        if (response->ideas) {
            for (int i = 0; i < response->ideas_count; i++) {
                free_extracted_idea_fields(&response->ideas[i]);
            }
            free(response->ideas);
        }
        if (response->errors) {
            for (int i = 0; i < response->errors_count; i++) {
                free_extracted_error_fields(&response->errors[i]);
            }
            free(response->errors);
        }
        if (response->training_data) {
            for (int i = 0; i < response->training_data_count; i++) {
                free_extracted_training_data_fields(&response->training_data[i]);
            }
            free(response->training_data);
        }
        if (response->raw_tags) {
            for (int i = 0; i < response->raw_tags_count; i++) {
                if (response->raw_tags[i]) free(response->raw_tags[i]);
            }
            free(response->raw_tags);
        }
        free(response);
    }
}

char* read_file_to_string(const char* file_path, TodoziError* error) {
    FILE *file = fopen(file_path, "rb");
    if (!file) {
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    long length = ftell(file);
    if (length < 0) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    if (fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    char *buffer = (char*)malloc(length + 1);
    if (!buffer) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    size_t read_length = fread(buffer, 1, length, file);
    if (read_length != (size_t)length) {
        free(buffer);
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    buffer[length] = '\0';
    fclose(file);
    *error = TODOZI_SUCCESS;
    return buffer;
}

static char* get_api_key(TodoziError* error) {
    // This is a simplified implementation
    // In practice, you'd read from environment variables or config files
    const char* api_key_env = getenv("TODOZI_API_KEY");
    if (api_key_env) {
        *error = TODOZI_SUCCESS;
        return safe_strdup(api_key_env);
    }
    
    *error = TODOZI_ERROR_CONFIG;
    return NULL;
}

static char* get_home_directory(TodoziError* error) {
    const char* home = getenv("HOME");
    if (home) {
        *error = TODOZI_SUCCESS;
        return safe_strdup(home);
    }
    
    *error = TODOZI_ERROR_CONFIG;
    return NULL;
}

char* load_hlx_config(const char* config_path, TodoziError* error) {
    // Simplified implementation - in practice you'd parse the HLX format
    FILE *file = fopen(config_path, "rb");
    if (!file) {
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    long length = ftell(file);
    if (length < 0) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    if (fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    char *buffer = (char*)malloc(length + 1);
    if (!buffer) {
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    size_t read_length = fread(buffer, 1, length, file);
    if (read_length != (size_t)length) {
        free(buffer);
        fclose(file);
        *error = TODOZI_ERROR_IO;
        return NULL;
    }

    buffer[length] = '\0';
    fclose(file);
    *error = TODOZI_SUCCESS;
    return buffer;
}

char* get_hlx_value(const char* hlx_content, const char* section, const char* key) {
    // Simplified implementation - in practice you'd parse the HLX format properly
    // This is just a placeholder that returns a default value
    if (strcmp(section, "registration") == 0 && strcmp(key, "user_id") == 0) {
        return safe_strdup("default_user_id");
    }
    if (strcmp(section, "registration") == 0 && strcmp(key, "fingerprint") == 0) {
        return safe_strdup("default_fingerprint");
    }
    return safe_strdup("");
}

char* hash_project_name(const char* project_name) {
    // Simplified implementation - in practice you'd use a proper hash function
    // This is just a placeholder that returns the project name
    return safe_strdup(project_name);
}

static char* generate_uuid() {
    uuid_t uuid;
    char* uuid_str = (char*)malloc(37); // UUID string is 36 characters + null terminator
    if (!uuid_str) return NULL;
    
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    return uuid_str;
}

void* safe_malloc(size_t size) {
    void* ptr = malloc(size);
    if (!ptr) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(EXIT_FAILURE);
    }
    return ptr;
}

char* safe_strdup(const char* str) {
    if (!str) return NULL;
    char* new_str = (char*)malloc(strlen(str) + 1);
    if (!new_str) {
        fprintf(stderr, "Memory allocation failed\n");
        exit(EXIT_FAILURE);
    }
    strcpy(new_str, str);
    return new_str;
}

char** safe_strdup_array(char** src, int count) {
    if (!src || count <= 0) return NULL;
    
    char** new_array = (char**)safe_malloc(count * sizeof(char*));
    for (int i = 0; i < count; i++) {
        new_array[i] = safe_strdup(src[i]);
    }
    return new_array;
}

int safe_strcmp(const char* a, const char* b) {
    if (!a && !b) return 0;
    if (!a) return -1;
    if (!b) return 1;
    return strcmp(a, b);
}

char* safe_strcat(const char* a, const char* b) {
    if (!a || !b) return NULL;
    
    size_t len_a = strlen(a);
    size_t len_b = strlen(b);
    char* result = (char*)safe_malloc(len_a + len_b + 1);
    
    strcpy(result, a);
    strcat(result, b);
    
    return result;
}

char* get_current_timestamp() {
    time_t now = time(NULL);
    char* timestamp = (char*)safe_malloc(100);
    strftime(timestamp, 100, "%Y-%m-%d %H:%M:%S UTC", gmtime(&now));
    return timestamp;
}

Task* create_task(const char* user_id, const char* action, const char* time, Priority priority, const char* project_id, Status status, const char* assignee, char** tags, int tags_count, char** dependencies, int dependencies_count, const char* context, int progress, TodoziError* error) {
    Task* task = (Task*)safe_malloc(sizeof(Task));
    
    task->id = generate_uuid();
    task->user_id = safe_strdup(user_id);
    task->action = safe_strdup(action);
    task->time = safe_strdup(time);
    task->priority = priority;
    task->parent_project = safe_strdup(project_id);
    task->status = status;
    task->assignee = assignee ? safe_strdup(assignee) : NULL;
    task->tags = safe_strdup_array(tags, tags_count);
    task->tags_count = tags_count;
    task->dependencies = safe_strdup_array(dependencies, dependencies_count);
    task->dependencies_count = dependencies_count;
    task->context = context ? safe_strdup(context) : NULL;
    task->progress = progress;
    
    *error = TODOZI_SUCCESS;
    return task;
}

Memory* create_memory(const char* user_id, const char* project_id, const char* moment, const char* meaning, const char* reason, MemoryImportance importance, MemoryTerm term, MemoryType memory_type, char** tags, int tags_count, TodoziError* error) {
    Memory* memory = (Memory*)safe_malloc(sizeof(Memory));
    
    memory->id = generate_uuid();
    memory->user_id = safe_strdup(user_id);
    memory->project_id = project_id ? safe_strdup(project_id) : NULL;
    memory->status = ITEM_STATUS_ACTIVE;
    memory->moment = safe_strdup(moment);
    memory->meaning = safe_strdup(meaning);
    memory->reason = safe_strdup(reason);
    memory->importance = importance;
    memory->term = term;
    memory->memory_type = memory_type;
    memory->tags = safe_strdup_array(tags, tags_count);
    memory->tags_count = tags_count;
    memory->created_at = time(NULL);
    memory->updated_at = time(NULL);
    
    *error = TODOZI_SUCCESS;
    return memory;
}

Idea* create_idea(const char* idea_text, const char* project_id, ShareLevel share, IdeaImportance importance, char** tags, int tags_count, const char* context, TodoziError* error) {
    Idea* idea = (Idea*)safe_malloc(sizeof(Idea));
    
    idea->id = generate_uuid();
    idea->idea = safe_strdup(idea_text);
    idea->project_id = project_id ? safe_strdup(project_id) : NULL;
    idea->status = ITEM_STATUS_ACTIVE;
    idea->share = share;
    idea->importance = importance;
    idea->tags = safe_strdup_array(tags, tags_count);
    idea->tags_count = tags_count;
    idea->context = context ? safe_strdup(context) : NULL;
    idea->created_at = time(NULL);
    idea->updated_at = time(NULL);
    
    *error = TODOZI_SUCCESS;
    return idea;
}

TodoziError extract_content(const char* content, const char* file_path, const char* output_format, int human, char** result) {
    return extract_with_endpoint(content, file_path, output_format, human, "plan", result);
}

TodoziError strategy_content(const char* content, const char* file_path, const char* output_format, int human, char** result) {
    return extract_with_endpoint(content, file_path, output_format, human, "strategic", result);
}

TodoziError extract_with_endpoint(const char* content, const char* file_path, const char* output_format, int human, const char* endpoint, char** result) {
    TodoziError rc = TODOZI_SUCCESS;
    char* input_content = NULL;
    char* api_key = NULL;
    char* home_dir = NULL;
    char* config_path = NULL;
    char* hlx_content = NULL;
    char* user_id = NULL;
    char* fingerprint = NULL;
    char* url = NULL;
    char* payload = NULL;
    struct string response_data = {0};
    json_object *json_response = NULL;
    ExtractResponse* extract_response = NULL;
    char* primary_project = NULL;
    char* primary_project_id = NULL;
    char* checklist = NULL;
    
    // Get content from inline text or file
    if (content && !file_path) {
        input_content = safe_strdup(content);
    } else if (!content && file_path) {
        input_content = read_file_to_string(file_path, &rc);
        if (rc != TODOZI_SUCCESS) {
            goto cleanup;
        }
    } else if (!content && !file_path) {
        rc = TODOZI_ERROR_VALIDATION;
        goto cleanup;
    } else {
        rc = TODOZI_ERROR_VALIDATION;
        goto cleanup;
    }
    
    // Get API key
    api_key = get_api_key(&rc);
    if (rc != TODOZI_SUCCESS) {
        goto cleanup;
    }
    
    // Security: Don't print API key to stdout
    if (api_key && strlen(api_key) > 0) {
        printf("🔑 API Key: [REDACTED]\n");
    } else {
        printf("🔑 API Key: (empty)\n");
    }
    
    // Get user info from config
    home_dir = get_home_directory(&rc);
    if (rc != TODOZI_SUCCESS) {
        goto cleanup;
    }
    
    config_path = safe_strcat(home_dir, "/.todozi/tdz.hlx");
    hlx_content = load_hlx_config(config_path, &rc);
    if (rc != TODOZI_SUCCESS) {
        goto cleanup;
    }
    
    user_id = get_hlx_value(hlx_content, "registration", "user_id");
    fingerprint = get_hlx_value(hlx_content, "registration", "fingerprint");
    
    // Validate endpoint length to prevent buffer overflow
    if (!endpoint || strlen(endpoint) > 256) {
        rc = TODOZI_ERROR_VALIDATION;
        goto cleanup;
    }
    
    // Build URL with proper size calculation
    size_t url_len = strlen("https://todozi.com/api/tdz/") + strlen(endpoint) + 1;
    if (url_len > 1024) {
        rc = TODOZI_ERROR_VALIDATION;
        goto cleanup;
    }
    url = (char*)safe_malloc(url_len);
    snprintf(url, url_len, "https://todozi.com/api/tdz/%s", endpoint);
    
    // Build JSON payload using json-c for safety
    json_object *payload_obj = json_object_new_object();
    json_object_object_add(payload_obj, "content", json_object_new_string(input_content));
    json_object_object_add(payload_obj, "extract_all", json_object_new_boolean(1));
    json_object_object_add(payload_obj, "model", json_object_new_string("gpt-oss:120b"));
    json_object_object_add(payload_obj, "language", json_object_new_string("english"));
    json_object_object_add(payload_obj, "user_id", json_object_new_string(user_id));
    json_object_object_add(payload_obj, "fingerprint", json_object_new_string(fingerprint));
    payload = safe_strdup(json_object_to_json_string_ext(payload_obj, JSON_C_TO_STRING_PLAIN));
    json_object_put(payload_obj);
    
    printf("📤 Sending request to: %s\n", url);
    printf("📦 Payload: %s\n", payload);
    
    // Make HTTP request using libcurl
    CURL *curl = NULL;
    CURLcode res;
    struct curl_slist *headers = NULL;
    
    init_string(&response_data);
    
    curl = curl_easy_init();
    if (!curl) {
        fprintf(stderr, "curl_easy_init() failed\n");
        rc = TODOZI_ERROR_API;
        goto cleanup;
    }
    
    // Validate API key length for auth header
    if (!api_key || strlen(api_key) > 400) {
        rc = TODOZI_ERROR_VALIDATION;
        curl_easy_cleanup(curl);
        curl = NULL;
        goto cleanup;
    }
    
    char auth_header[512];
    int auth_header_len = snprintf(auth_header, sizeof(auth_header), "Authorization: Bearer %s", api_key);
    if (auth_header_len < 0 || auth_header_len >= (int)sizeof(auth_header)) {
        rc = TODOZI_ERROR_VALIDATION;
        curl_easy_cleanup(curl);
        curl = NULL;
        goto cleanup;
    }
    
    headers = curl_slist_append(headers, "Content-Type: application/json");
    if (!headers) {
        rc = TODOZI_ERROR_API;
        curl_easy_cleanup(curl);
        curl = NULL;
        goto cleanup;
    }
    
    headers = curl_slist_append(headers, auth_header);
    if (!headers) {
        curl_slist_free_all(headers);
        rc = TODOZI_ERROR_API;
        curl_easy_cleanup(curl);
        curl = NULL;
        goto cleanup;
    }
    
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, payload);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writefunc);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response_data);
    
    res = curl_easy_perform(curl);
    
    if(res != CURLE_OK) {
        fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
        rc = TODOZI_ERROR_API;
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        curl = NULL;
        goto cleanup;
    }
    
    long response_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);
    
    if (response_code != 200) {
        printf("API request failed with status code: %ld\n", response_code);
        rc = TODOZI_ERROR_API;
        curl_slist_free_all(headers);
        curl_easy_cleanup(curl);
        curl = NULL;
        goto cleanup;
    }
    
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    curl = NULL;
    
    // Validate response data before parsing
    if (!response_data.ptr) {
        rc = TODOZI_ERROR_API;
        goto cleanup;
    }
    
    printf("🔍 Raw API Response:\n%s\n", response_data.ptr);
    
    // Parse JSON response
    json_response = json_tokener_parse(response_data.ptr);
    if (!json_response) {
        rc = TODOZI_ERROR_API;
        goto cleanup;
    }
    
    // Create extract response structure
    extract_response = (ExtractResponse*)safe_malloc(sizeof(ExtractResponse));
    memset(extract_response, 0, sizeof(ExtractResponse));
    
    // Parse tasks
    json_object *tasks_array;
    if (json_object_object_get_ex(json_response, "tasks", &tasks_array) && json_object_is_type(tasks_array, json_type_array)) {
        int tasks_count = json_object_array_length(tasks_array);
        extract_response->tasks = (ExtractedTask*)safe_malloc(tasks_count * sizeof(ExtractedTask));
        extract_response->tasks_count = 0;
        
        for (int i = 0; i < tasks_count; i++) {
            json_object *task_obj = json_object_array_get_idx(tasks_array, i);
            if (task_obj) {
                json_object *action_obj, *time_obj, *priority_obj, *project_obj, *status_obj, *assignee_obj, *tags_obj;
                
                if (json_object_object_get_ex(task_obj, "action", &action_obj) &&
                    json_object_object_get_ex(task_obj, "time", &time_obj) &&
                    json_object_object_get_ex(task_obj, "priority", &priority_obj) &&
                    json_object_object_get_ex(task_obj, "project", &project_obj) &&
                    json_object_object_get_ex(task_obj, "status", &status_obj)) {
                    
                    ExtractedTask* task = &extract_response->tasks[extract_response->tasks_count];
                    task->action = safe_strdup(json_object_get_string(action_obj));
                    task->time = safe_strdup(json_object_get_string(time_obj));
                    task->priority = safe_strdup(json_object_get_string(priority_obj));
                    task->project = safe_strdup(json_object_get_string(project_obj));
                    task->status = safe_strdup(json_object_get_string(status_obj));
                    task->assignee = NULL;
                    task->tags = NULL;
                    task->tags_count = 0;
                    
                    if (json_object_object_get_ex(task_obj, "assignee", &assignee_obj) && !json_object_is_type(assignee_obj, json_type_null)) {
                        task->assignee = safe_strdup(json_object_get_string(assignee_obj));
                    }
                    
                    if (json_object_object_get_ex(task_obj, "tags", &tags_obj) && json_object_is_type(tags_obj, json_type_array)) {
                        int tags_count = json_object_array_length(tags_obj);
                        task->tags = (char**)safe_malloc(tags_count * sizeof(char*));
                        task->tags_count = 0;
                        
                        for (int j = 0; j < tags_count; j++) {
                            json_object *tag_obj = json_object_array_get_idx(tags_obj, j);
                            if (tag_obj) {
                                task->tags[task->tags_count] = safe_strdup(json_object_get_string(tag_obj));
                                task->tags_count++;
                            }
                        }
                    }
                    
                    extract_response->tasks_count++;
                }
            }
        }
    }
    
    // Parse memories
    json_object *memories_array;
    if (json_object_object_get_ex(json_response, "memories", &memories_array) && json_object_is_type(memories_array, json_type_array)) {
        int memories_count = json_object_array_length(memories_array);
        extract_response->memories = (ExtractedMemory*)safe_malloc(memories_count * sizeof(ExtractedMemory));
        extract_response->memories_count = 0;
        
        for (int i = 0; i < memories_count; i++) {
            json_object *memory_obj = json_object_array_get_idx(memories_array, i);
            if (memory_obj) {
                json_object *moment_obj, *meaning_obj, *reason_obj, *importance_obj, *term_obj;
                
                if (json_object_object_get_ex(memory_obj, "moment", &moment_obj) &&
                    json_object_object_get_ex(memory_obj, "meaning", &meaning_obj) &&
                    json_object_object_get_ex(memory_obj, "reason", &reason_obj) &&
                    json_object_object_get_ex(memory_obj, "importance", &importance_obj) &&
                    json_object_object_get_ex(memory_obj, "term", &term_obj)) {
                    
                    ExtractedMemory* memory = &extract_response->memories[extract_response->memories_count];
                    memory->moment = safe_strdup(json_object_get_string(moment_obj));
                    memory->meaning = safe_strdup(json_object_get_string(meaning_obj));
                    memory->reason = safe_strdup(json_object_get_string(reason_obj));
                    memory->importance = safe_strdup(json_object_get_string(importance_obj));
                    memory->term = safe_strdup(json_object_get_string(term_obj));
                    
                    extract_response->memories_count++;
                }
            }
        }
    }
    
    // Parse ideas
    json_object *ideas_array;
    if (json_object_object_get_ex(json_response, "ideas", &ideas_array) && json_object_is_type(ideas_array, json_type_array)) {
        int ideas_count = json_object_array_length(ideas_array);
        extract_response->ideas = (ExtractedIdea*)safe_malloc(ideas_count * sizeof(ExtractedIdea));
        extract_response->ideas_count = 0;
        
        for (int i = 0; i < ideas_count; i++) {
            json_object *idea_obj = json_object_array_get_idx(ideas_array, i);
            if (idea_obj) {
                json_object *idea_text_obj, *share_obj, *importance_obj;
                
                if (json_object_object_get_ex(idea_obj, "idea", &idea_text_obj) &&
                    json_object_object_get_ex(idea_obj, "share", &share_obj) &&
                    json_object_object_get_ex(idea_obj, "importance", &importance_obj)) {
                    
                    ExtractedIdea* idea = &extract_response->ideas[extract_response->ideas_count];
                    idea->idea = safe_strdup(json_object_get_string(idea_text_obj));
                    idea->share = safe_strdup(json_object_get_string(share_obj));
                    idea->importance = safe_strdup(json_object_get_string(importance_obj));
                    
                    extract_response->ideas_count++;
                }
            }
        }
    }
    
    // Parse errors
    json_object *errors_array;
    if (json_object_object_get_ex(json_response, "errors", &errors_array) && json_object_is_type(errors_array, json_type_array)) {
        int errors_count = json_object_array_length(errors_array);
        extract_response->errors = (ExtractedError*)safe_malloc(errors_count * sizeof(ExtractedError));
        extract_response->errors_count = 0;
        
        for (int i = 0; i < errors_count; i++) {
            json_object *error_obj = json_object_array_get_idx(errors_array, i);
            if (error_obj) {
                json_object *title_obj, *description_obj, *severity_obj, *category_obj;
                
                if (json_object_object_get_ex(error_obj, "title", &title_obj) &&
                    json_object_object_get_ex(error_obj, "description", &description_obj) &&
                    json_object_object_get_ex(error_obj, "severity", &severity_obj) &&
                    json_object_object_get_ex(error_obj, "category", &category_obj)) {
                    
                    ExtractedError* error_item = &extract_response->errors[extract_response->errors_count];
                    error_item->title = safe_strdup(json_object_get_string(title_obj));
                    error_item->description = safe_strdup(json_object_get_string(description_obj));
                    error_item->severity = safe_strdup(json_object_get_string(severity_obj));
                    error_item->category = safe_strdup(json_object_get_string(category_obj));
                    
                    extract_response->errors_count++;
                }
            }
        }
    }
    
    // Parse training data
    json_object *training_data_array;
    if (json_object_object_get_ex(json_response, "training_data", &training_data_array) && json_object_is_type(training_data_array, json_type_array)) {
        int training_data_count = json_object_array_length(training_data_array);
        extract_response->training_data = (ExtractedTrainingData*)safe_malloc(training_data_count * sizeof(ExtractedTrainingData));
        extract_response->training_data_count = 0;
        
        for (int i = 0; i < training_data_count; i++) {
            json_object *data_obj = json_object_array_get_idx(training_data_array, i);
            if (data_obj) {
                json_object *prompt_obj, *completion_obj, *data_type_obj;
                
                if (json_object_object_get_ex(data_obj, "prompt", &prompt_obj) &&
                    json_object_object_get_ex(data_obj, "completion", &completion_obj) &&
                    json_object_object_get_ex(data_obj, "data_type", &data_type_obj)) {
                    
                    ExtractedTrainingData* data = &extract_response->training_data[extract_response->training_data_count];
                    data->prompt = safe_strdup(json_object_get_string(prompt_obj));
                    data->completion = safe_strdup(json_object_get_string(completion_obj));
                    data->data_type = safe_strdup(json_object_get_string(data_type_obj));
                    
                    extract_response->training_data_count++;
                }
            }
        }
    }
    
    // Parse raw tags
    json_object *raw_tags_array;
    if (json_object_object_get_ex(json_response, "raw_tags", &raw_tags_array) && json_object_is_type(raw_tags_array, json_type_array)) {
        int raw_tags_count = json_object_array_length(raw_tags_array);
        extract_response->raw_tags = (char**)safe_malloc(raw_tags_count * sizeof(char*));
        extract_response->raw_tags_count = 0;
        
        for (int i = 0; i < raw_tags_count; i++) {
            json_object *tag_obj = json_object_array_get_idx(raw_tags_array, i);
            if (tag_obj && json_object_is_type(tag_obj, json_type_string)) {
                extract_response->raw_tags[extract_response->raw_tags_count] = safe_strdup(json_object_get_string(tag_obj));
                extract_response->raw_tags_count++;
            }
        }
    }
    
    // Auto-embed and save tasks to project files
    if (extract_response->tasks_count > 0 || extract_response->memories_count > 0 || extract_response->ideas_count > 0) {
        printf("🚀 Auto-embedding and saving extracted content...\n");
        
        // Determine the primary project from tasks
        if (extract_response->tasks_count > 0 && extract_response->tasks[0].project) {
            primary_project = extract_response->tasks[0].project; // Don't duplicate, just reference
            primary_project_id = hash_project_name(primary_project);
        } else {
            primary_project = safe_strdup("Default Project");
            primary_project_id = hash_project_name(primary_project);
        }
        
        // Save tasks
        if (extract_response->tasks_count > 0) {
            printf("📝 Saving %d extracted tasks...\n", extract_response->tasks_count);
            for (int i = 0; i < extract_response->tasks_count; i++) {
                ExtractedTask* extracted_task = &extract_response->tasks[i];
                
                // Generate consistent project ID by hashing the project name
                char* project_id = hash_project_name(extracted_task->project);
                
                // Convert priority string to enum
                Priority priority = PRIORITY_MEDIUM;
                if (strcmp(extracted_task->priority, "Low") == 0) {
                    priority = PRIORITY_LOW;
                } else if (strcmp(extracted_task->priority, "High") == 0) {
                    priority = PRIORITY_HIGH;
                } else if (strcmp(extracted_task->priority, "Critical") == 0) {
                    priority = PRIORITY_CRITICAL;
                }
                
                // Convert status string to enum
                Status status = STATUS_TODO;
                if (strcmp(extracted_task->status, "In Progress") == 0) {
                    status = STATUS_IN_PROGRESS;
                } else if (strcmp(extracted_task->status, "Done") == 0) {
                    status = STATUS_DONE;
                } else if (strcmp(extracted_task->status, "Cancelled") == 0) {
                    status = STATUS_CANCELLED;
                }
                
                TodoziError task_error;
                Task* task = create_task(
                    user_id,
                    extracted_task->action,
                    extracted_task->time,
                    priority,
                    project_id,
                    status,
                    extracted_task->assignee,
                    extracted_task->tags,
                    extracted_task->tags_count,
                    NULL, // dependencies
                    0,    // dependencies_count
                    NULL, // context
                    0,    // progress
                    &task_error
                );
                
                if (task_error == TODOZI_SUCCESS) {
                    printf("✅ Saved task: %s (ID: %s)\n", extracted_task->action, task->id);
                    log_to_history(task);
                    
                    // Clean up task
                    if (task->id) free(task->id);
                    if (task->user_id) free(task->user_id);
                    if (task->action) free(task->action);
                    if (task->time) free(task->time);
                    if (task->parent_project) free(task->parent_project);
                    if (task->assignee) free(task->assignee);
                    if (task->tags) {
                        for (int j = 0; j < task->tags_count; j++) {
                            if (task->tags[j]) free(task->tags[j]);
                        }
                        free(task->tags);
                    }
                    if (task->dependencies) {
                        for (int j = 0; j < task->dependencies_count; j++) {
                            if (task->dependencies[j]) free(task->dependencies[j]);
                        }
                        free(task->dependencies);
                    }
                    if (task->context) free(task->context);
                    free(task);
                }
                
                if (project_id) free(project_id);
            }
        }
        
        // Save memories
        if (extract_response->memories_count > 0) {
            printf("🧠 Saving %d extracted memories...\n", extract_response->memories_count);
            for (int i = 0; i < extract_response->memories_count; i++) {
                ExtractedMemory* extracted_memory = &extract_response->memories[i];
                
                // Convert importance string to enum
                MemoryImportance importance = MEMORY_IMPORTANCE_MEDIUM;
                if (strcmp(extracted_memory->importance, "Low") == 0) {
                    importance = MEMORY_IMPORTANCE_LOW;
                } else if (strcmp(extracted_memory->importance, "High") == 0) {
                    importance = MEMORY_IMPORTANCE_HIGH;
                } else if (strcmp(extracted_memory->importance, "Critical") == 0) {
                    importance = MEMORY_IMPORTANCE_CRITICAL;
                }
                
                // Convert term string to enum
                MemoryTerm term = MEMORY_TERM_SHORT;
                if (strcmp(extracted_memory->term, "Medium") == 0) {
                    term = MEMORY_TERM_MEDIUM;
                } else if (strcmp(extracted_memory->term, "Long") == 0) {
                    term = MEMORY_TERM_LONG;
                }
                
                TodoziError memory_error;
                Memory* memory = create_memory(
                    user_id,
                    primary_project_id,
                    extracted_memory->moment,
                    extracted_memory->meaning,
                    extracted_memory->reason,
                    importance,
                    term,
                    MEMORY_TYPE_STANDARD,
                    NULL, // tags
                    0,    // tags_count
                    &memory_error
                );
                
                if (memory_error == TODOZI_SUCCESS) {
                    printf("✅ Saved memory: %s (ID: %s)\n", extracted_memory->moment, memory->id);
                    
                    // Clean up memory
                    if (memory->id) free(memory->id);
                    if (memory->user_id) free(memory->user_id);
                    if (memory->project_id) free(memory->project_id);
                    if (memory->moment) free(memory->moment);
                    if (memory->meaning) free(memory->meaning);
                    if (memory->reason) free(memory->reason);
                    if (memory->tags) {
                        for (int j = 0; j < memory->tags_count; j++) {
                            if (memory->tags[j]) free(memory->tags[j]);
                        }
                        free(memory->tags);
                    }
                    free(memory);
                }
            }
        }
        
        // Save ideas
        if (extract_response->ideas_count > 0) {
            printf("💡 Saving %d extracted ideas...\n", extract_response->ideas_count);
            for (int i = 0; i < extract_response->ideas_count; i++) {
                ExtractedIdea* extracted_idea = &extract_response->ideas[i];
                
                // Convert share string to enum
                ShareLevel share = SHARE_LEVEL_PRIVATE;
                if (strcmp(extracted_idea->share, "Team") == 0) {
                    share = SHARE_LEVEL_TEAM;
                } else if (strcmp(extracted_idea->share, "Public") == 0) {
                    share = SHARE_LEVEL_PUBLIC;
                }
                
                // Convert importance string to enum
                IdeaImportance importance = IDEA_IMPORTANCE_MEDIUM;
                if (strcmp(extracted_idea->importance, "Low") == 0) {
                    importance = IDEA_IMPORTANCE_LOW;
                } else if (strcmp(extracted_idea->importance, "High") == 0) {
                    importance = IDEA_IMPORTANCE_HIGH;
                } else if (strcmp(extracted_idea->importance, "Critical") == 0) {
                    importance = IDEA_IMPORTANCE_CRITICAL;
                }
                
                TodoziError idea_error;
                Idea* idea = create_idea(
                    extracted_idea->idea,
                    primary_project_id,
                    share,
                    importance,
                    NULL, // tags
                    0,    // tags_count
                    NULL, // context
                    &idea_error
                );
                
                if (idea_error == TODOZI_SUCCESS) {
                    printf("✅ Saved idea: %s (ID: %s)\n", extracted_idea->idea, idea->id);
                    
                    // Clean up idea
                    if (idea->id) free(idea->id);
                    if (idea->idea) free(idea->idea);
                    if (idea->project_id) free(idea->project_id);
                    if (idea->tags) {
                        for (int j = 0; j < idea->tags_count; j++) {
                            if (idea->tags[j]) free(idea->tags[j]);
                        }
                        free(idea->tags);
                    }
                    if (idea->context) free(idea->context);
                    free(idea);
                }
            }
        }
    }
    
    // Format output based on requested format
    if (strcmp(output_format, "json") == 0) {
        *result = safe_strdup(json_object_to_json_string_ext(json_response, JSON_C_TO_STRING_PRETTY));
    } else if (strcmp(output_format, "csv") == 0) {
        rc = format_as_csv(extract_response, result);
    } else if (strcmp(output_format, "md") == 0 || strcmp(output_format, "markdown") == 0) {
        rc = format_as_markdown(extract_response, result);
    } else {
        rc = TODOZI_ERROR_VALIDATION;
    }
    
    // Generate human-readable checklist file if --human flag is set
    if (human && rc == TODOZI_SUCCESS) {
        rc = format_as_human_checklist(extract_response, &checklist);
        
        if (rc == TODOZI_SUCCESS) {
            time_t now = time(NULL);
            struct tm *tm_info = gmtime(&now);
            char timestamp[32];
            strftime(timestamp, sizeof(timestamp), "%Y%m%d_%H%M%S", tm_info);
            
            char* checklist_filename = (char*)safe_malloc(256);
            snprintf(checklist_filename, 256, "todozi_checklist_%s_%s.md", endpoint, timestamp);
            
            FILE *file = fopen(checklist_filename, "w");
            if (file) {
                fputs(checklist, file);
                fclose(file);
                printf("📋 Human checklist saved to: %s\n", checklist_filename);
            } else {
                rc = TODOZI_ERROR_IO;
            }
            
            free(checklist_filename);
        }
    }

cleanup:
    // Clean up all allocated resources
    if (input_content) free(input_content);
    if (api_key) free(api_key);
    if (home_dir) free(home_dir);
    if (config_path) free(config_path);
    if (hlx_content) free(hlx_content);
    if (user_id) free(user_id);
    if (fingerprint) free(fingerprint);
    if (url) free(url);
    if (payload) free(payload);
    if (response_data.ptr) free(response_data.ptr);
    if (json_response) json_object_put(json_response);
    if (extract_response) free_extract_response(extract_response);
    // Only free primary_project if we allocated it (when tasks_count == 0)
    if (primary_project && (!extract_response || extract_response->tasks_count == 0)) {
        free(primary_project);
    }
    if (primary_project_id) free(primary_project_id);
    if (checklist) free(checklist);
    
    return rc;
}

TodoziError format_as_csv(const ExtractResponse* response, char** result) {
    if (!response || !result) {
        return TODOZI_ERROR_VALIDATION;
    }
    
    // Calculate approximate buffer size needed
    size_t buffer_size = 4096;
    if (response->tasks) {
        for (int i = 0; i < response->tasks_count; i++) {
            const ExtractedTask* task = &response->tasks[i];
            if (task->action) buffer_size += strlen(task->action);
            if (task->time) buffer_size += strlen(task->time);
            if (task->priority) buffer_size += strlen(task->priority);
            if (task->project) buffer_size += strlen(task->project);
            if (task->status) buffer_size += strlen(task->status);
            buffer_size += 256; // overhead
            if (task->tags) {
                for (int j = 0; j < task->tags_count; j++) {
                    if (task->tags[j]) {
                        buffer_size += strlen(task->tags[j]) + 2;
                    }
                }
            }
        }
    }
    
    char* csv = (char*)safe_malloc(buffer_size);
    size_t offset = 0;
    
    // Tasks CSV
    if (response->tasks && response->tasks_count > 0) {
        offset += snprintf(csv + offset, buffer_size - offset, "Type,Action,Time,Priority,Project,Status,Assignee,Tags\n");
        for (int i = 0; i < response->tasks_count; i++) {
            const ExtractedTask* task = &response->tasks[i];
            if (!task) continue;
            
            // Escape quotes in action (with null check)
            const char* action_str = task->action ? task->action : "";
            size_t action_len = strlen(action_str);
            char* escaped_action = (char*)safe_malloc(action_len * 2 + 1);
            size_t esc_offset = 0;
            for (size_t j = 0; j < action_len; j++) {
                if (action_str[j] == '"') {
                    escaped_action[esc_offset++] = '"';
                    escaped_action[esc_offset++] = '"';
                } else {
                    escaped_action[esc_offset++] = action_str[j];
                }
            }
            escaped_action[esc_offset] = '\0';
            
            offset += snprintf(csv + offset, buffer_size - offset,
                "Task,\"%s\",\"%s\",%s,%s,%s,%s,\"",
                escaped_action,
                task->time ? task->time : "",
                task->priority ? task->priority : "",
                task->project ? task->project : "",
                task->status ? task->status : "",
                task->assignee ? task->assignee : "");
            
            free(escaped_action);
            
            // Add tags
            if (task->tags) {
                for (int j = 0; j < task->tags_count; j++) {
                    if (task->tags[j]) {
                        if (j > 0) {
                            offset += snprintf(csv + offset, buffer_size - offset, ", ");
                        }
                        offset += snprintf(csv + offset, buffer_size - offset, "%s", task->tags[j]);
                    }
                }
            }
            
            offset += snprintf(csv + offset, buffer_size - offset, "\"\n");
        }
    }
    
    *result = csv;
    return TODOZI_SUCCESS;
}

TodoziError format_as_markdown(const ExtractResponse* response, char** result) {
    if (!response || !result) {
        return TODOZI_ERROR_VALIDATION;
    }
    
    // Calculate approximate buffer size needed
    size_t buffer_size = 4096;
    if (response->tasks) {
        for (int i = 0; i < response->tasks_count; i++) {
            const ExtractedTask* task = &response->tasks[i];
            if (task->action) buffer_size += strlen(task->action);
            buffer_size += 256;
            if (task->tags) {
                for (int j = 0; j < task->tags_count; j++) {
                    if (task->tags[j]) {
                        buffer_size += strlen(task->tags[j]) + 16;
                    }
                }
            }
        }
    }
    if (response->memories) {
        for (int i = 0; i < response->memories_count; i++) {
            const ExtractedMemory* memory = &response->memories[i];
            if (memory->moment) buffer_size += strlen(memory->moment);
            if (memory->meaning) buffer_size += strlen(memory->meaning);
            if (memory->reason) buffer_size += strlen(memory->reason);
            if (memory->importance) buffer_size += strlen(memory->importance);
            if (memory->term) buffer_size += strlen(memory->term);
            buffer_size += 256;
        }
    }
    if (response->ideas) {
        for (int i = 0; i < response->ideas_count; i++) {
            const ExtractedIdea* idea = &response->ideas[i];
            if (idea->idea) buffer_size += strlen(idea->idea);
            if (idea->importance) buffer_size += strlen(idea->importance);
            if (idea->share) buffer_size += strlen(idea->share);
            buffer_size += 256;
        }
    }
    if (response->raw_tags) {
        for (int i = 0; i < response->raw_tags_count; i++) {
            if (response->raw_tags[i]) {
                buffer_size += strlen(response->raw_tags[i]) + 16;
            }
        }
    }
    
    char* md = (char*)safe_malloc(buffer_size);
    size_t offset = 0;
    
    offset += snprintf(md + offset, buffer_size - offset, "# Extracted Content\n\n");
    
    // Tasks
    if (response->tasks && response->tasks_count > 0) {
        offset += snprintf(md + offset, buffer_size - offset, "## Tasks\n\n");
        for (int i = 0; i < response->tasks_count; i++) {
            const ExtractedTask* task = &response->tasks[i];
            if (!task) continue;
            offset += snprintf(md + offset, buffer_size - offset, "%d. **%s**\n", i + 1, task->action ? task->action : "");
            offset += snprintf(md + offset, buffer_size - offset, "   - Time: %s\n", task->time ? task->time : "");
            offset += snprintf(md + offset, buffer_size - offset, "   - Priority: %s\n", task->priority ? task->priority : "");
            offset += snprintf(md + offset, buffer_size - offset, "   - Project: %s\n", task->project ? task->project : "");
            offset += snprintf(md + offset, buffer_size - offset, "   - Status: %s\n", task->status ? task->status : "");
            if (task->assignee) {
                offset += snprintf(md + offset, buffer_size - offset, "   - Assignee: %s\n", task->assignee);
            }
            if (task->tags && task->tags_count > 0) {
                offset += snprintf(md + offset, buffer_size - offset, "   - Tags: ");
                for (int j = 0; j < task->tags_count; j++) {
                    if (task->tags[j]) {
                        if (j > 0) {
                            offset += snprintf(md + offset, buffer_size - offset, ", ");
                        }
                        offset += snprintf(md + offset, buffer_size - offset, "%s", task->tags[j]);
                    }
                }
                offset += snprintf(md + offset, buffer_size - offset, "\n");
            }
            offset += snprintf(md + offset, buffer_size - offset, "\n");
        }
    }
    
    // Memories
    if (response->memories && response->memories_count > 0) {
        offset += snprintf(md + offset, buffer_size - offset, "## Memories\n\n");
        for (int i = 0; i < response->memories_count; i++) {
            const ExtractedMemory* memory = &response->memories[i];
            if (!memory) continue;
            offset += snprintf(md + offset, buffer_size - offset, "- **%s**: %s\n", 
                memory->moment ? memory->moment : "", memory->meaning ? memory->meaning : "");
            offset += snprintf(md + offset, buffer_size - offset, "  - Reason: %s\n", memory->reason ? memory->reason : "");
            offset += snprintf(md + offset, buffer_size - offset, "  - Importance: %s\n", memory->importance ? memory->importance : "");
            offset += snprintf(md + offset, buffer_size - offset, "  - Term: %s\n\n", memory->term ? memory->term : "");
        }
    }
    
    // Ideas
    if (response->ideas && response->ideas_count > 0) {
        offset += snprintf(md + offset, buffer_size - offset, "## Ideas\n\n");
        for (int i = 0; i < response->ideas_count; i++) {
            const ExtractedIdea* idea = &response->ideas[i];
            if (!idea) continue;
            offset += snprintf(md + offset, buffer_size - offset, "- **%s** (%s)\n", 
                idea->idea ? idea->idea : "", idea->importance ? idea->importance : "");
            offset += snprintf(md + offset, buffer_size - offset, "  - Share: %s\n\n", idea->share ? idea->share : "");
        }
    }
    
    // Raw tags
    if (response->raw_tags && response->raw_tags_count > 0) {
        offset += snprintf(md + offset, buffer_size - offset, "## Raw Tags\n\n```\n");
        for (int i = 0; i < response->raw_tags_count; i++) {
            if (response->raw_tags[i]) {
                offset += snprintf(md + offset, buffer_size - offset, "%s\n", response->raw_tags[i]);
            }
        }
        offset += snprintf(md + offset, buffer_size - offset, "```\n");
    }
    
    *result = md;
    return TODOZI_SUCCESS;
}

TodoziError format_as_human_checklist(const ExtractResponse* response, char** result) {
    if (!response || !result) {
        return TODOZI_ERROR_VALIDATION;
    }
    
    // Calculate approximate buffer size needed
    size_t buffer_size = 8192;
    if (response->tasks) {
        for (int i = 0; i < response->tasks_count; i++) {
            const ExtractedTask* task = &response->tasks[i];
            if (task->action) buffer_size += strlen(task->action);
            if (task->project) buffer_size += strlen(task->project);
            if (task->time) buffer_size += strlen(task->time);
            if (task->priority) buffer_size += strlen(task->priority);
            if (task->status) buffer_size += strlen(task->status);
            buffer_size += 256;
            if (task->assignee) {
                buffer_size += strlen(task->assignee) + 16;
            }
            if (task->tags) {
                for (int j = 0; j < task->tags_count; j++) {
                    if (task->tags[j]) {
                        buffer_size += strlen(task->tags[j]) + 16;
                    }
                }
            }
        }
    }
    if (response->memories) {
        for (int i = 0; i < response->memories_count; i++) {
            const ExtractedMemory* memory = &response->memories[i];
            if (memory->moment) buffer_size += strlen(memory->moment);
            if (memory->meaning) buffer_size += strlen(memory->meaning);
            if (memory->reason) buffer_size += strlen(memory->reason);
            if (memory->importance) buffer_size += strlen(memory->importance);
            if (memory->term) buffer_size += strlen(memory->term);
            buffer_size += 256;
        }
    }
    if (response->ideas) {
        for (int i = 0; i < response->ideas_count; i++) {
            const ExtractedIdea* idea = &response->ideas[i];
            if (idea->idea) buffer_size += strlen(idea->idea);
            if (idea->share) buffer_size += strlen(idea->share);
            if (idea->importance) buffer_size += strlen(idea->importance);
            buffer_size += 256;
        }
    }
    if (response->errors) {
        for (int i = 0; i < response->errors_count; i++) {
            const ExtractedError* error = &response->errors[i];
            if (error->title) buffer_size += strlen(error->title);
            if (error->description) buffer_size += strlen(error->description);
            if (error->severity) buffer_size += strlen(error->severity);
            if (error->category) buffer_size += strlen(error->category);
            buffer_size += 256;
        }
    }
    if (response->training_data) {
        for (int i = 0; i < response->training_data_count; i++) {
            const ExtractedTrainingData* data = &response->training_data[i];
            if (data->prompt) buffer_size += strlen(data->prompt);
            if (data->completion) buffer_size += strlen(data->completion);
            if (data->data_type) buffer_size += strlen(data->data_type);
            buffer_size += 256;
        }
    }
    
    char* checklist = (char*)safe_malloc(buffer_size);
    size_t offset = 0;
    
    offset += snprintf(checklist + offset, buffer_size - offset, "# 📋 Todozi Human Checklist\n\n");
    
    time_t now = time(NULL);
    struct tm *tm_info = gmtime(&now);
    char timestamp[64];
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S UTC", tm_info);
    offset += snprintf(checklist + offset, buffer_size - offset, "Generated: %s\n\n", timestamp);
    offset += snprintf(checklist + offset, buffer_size - offset, "---\n\n");
    
    // Tasks as checkboxes with metadata
    if (response->tasks && response->tasks_count > 0) {
        offset += snprintf(checklist + offset, buffer_size - offset, "## 📝 Tasks\n\n");
        for (int i = 0; i < response->tasks_count; i++) {
            const ExtractedTask* task = &response->tasks[i];
            if (!task) continue;
            offset += snprintf(checklist + offset, buffer_size - offset, "- [ ] **%s**\n", task->action ? task->action : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 📁 Project: `%s`\n", task->project ? task->project : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - ⏱️ Time: `%s`\n", task->time ? task->time : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 🎯 Priority: `%s`\n", task->priority ? task->priority : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 📊 Status: `%s`\n", task->status ? task->status : "");
            
            if (task->assignee) {
                offset += snprintf(checklist + offset, buffer_size - offset, "  - 👤 Assignee: `%s`\n", task->assignee);
            }
            
            if (task->tags && task->tags_count > 0) {
                offset += snprintf(checklist + offset, buffer_size - offset, "  - 🏷️ Tags: ");
                for (int j = 0; j < task->tags_count; j++) {
                    if (task->tags[j]) {
                        if (j > 0) {
                            offset += snprintf(checklist + offset, buffer_size - offset, ", ");
                        }
                        offset += snprintf(checklist + offset, buffer_size - offset, "`%s`", task->tags[j]);
                    }
                }
                offset += snprintf(checklist + offset, buffer_size - offset, "\n");
            }
            
            offset += snprintf(checklist + offset, buffer_size - offset, "\n");
        }
    }
    
    // Memories as checkboxes
    if (response->memories && response->memories_count > 0) {
        offset += snprintf(checklist + offset, buffer_size - offset, "## 🧠 Memories to Record\n\n");
        for (int i = 0; i < response->memories_count; i++) {
            const ExtractedMemory* memory = &response->memories[i];
            if (!memory) continue;
            offset += snprintf(checklist + offset, buffer_size - offset, "- [ ] **%s**\n", memory->moment ? memory->moment : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 💡 Meaning: %s\n", memory->meaning ? memory->meaning : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 🎯 Reason: %s\n", memory->reason ? memory->reason : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 📊 Importance: `%s`\n", memory->importance ? memory->importance : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - ⏰ Term: `%s`\n", memory->term ? memory->term : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "\n");
        }
    }
    
    // Ideas as checkboxes
    if (response->ideas && response->ideas_count > 0) {
        offset += snprintf(checklist + offset, buffer_size - offset, "## 💡 Ideas to Explore\n\n");
        for (int i = 0; i < response->ideas_count; i++) {
            const ExtractedIdea* idea = &response->ideas[i];
            if (!idea) continue;
            offset += snprintf(checklist + offset, buffer_size - offset, "- [ ] **%s**\n", idea->idea ? idea->idea : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 🔒 Share Level: `%s`\n", idea->share ? idea->share : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - ⭐ Importance: `%s`\n", idea->importance ? idea->importance : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "\n");
        }
    }
    
    // Errors as checkboxes
    if (response->errors && response->errors_count > 0) {
        offset += snprintf(checklist + offset, buffer_size - offset, "## ❌ Errors to Fix\n\n");
        for (int i = 0; i < response->errors_count; i++) {
            const ExtractedError* error = &response->errors[i];
            if (!error) continue;
            offset += snprintf(checklist + offset, buffer_size - offset, "- [ ] **%s**\n", error->title ? error->title : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 📝 Description: %s\n", error->description ? error->description : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 🔥 Severity: `%s`\n", error->severity ? error->severity : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 📂 Category: `%s`\n", error->category ? error->category : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "\n");
        }
    }
    
    // Training data as checkboxes
    if (response->training_data && response->training_data_count > 0) {
        offset += snprintf(checklist + offset, buffer_size - offset, "## 🎓 Training Data to Review\n\n");
        for (int i = 0; i < response->training_data_count; i++) {
            const ExtractedTrainingData* data = &response->training_data[i];
            if (!data) continue;
            offset += snprintf(checklist + offset, buffer_size - offset, "- [ ] **%s**\n", data->prompt ? data->prompt : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - 📦 Type: `%s`\n", data->data_type ? data->data_type : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "  - ✅ Completion: %s\n", data->completion ? data->completion : "");
            offset += snprintf(checklist + offset, buffer_size - offset, "\n");
        }
    }
    
    // Summary section
    offset += snprintf(checklist + offset, buffer_size - offset, "\n---\n\n");
    offset += snprintf(checklist + offset, buffer_size - offset, "## 📊 Summary\n\n");
    offset += snprintf(checklist + offset, buffer_size - offset, "- Total Tasks: **%d**\n", response->tasks_count);
    offset += snprintf(checklist + offset, buffer_size - offset, "- Total Memories: **%d**\n", response->memories_count);
    offset += snprintf(checklist + offset, buffer_size - offset, "- Total Ideas: **%d**\n", response->ideas_count);
    offset += snprintf(checklist + offset, buffer_size - offset, "- Total Errors: **%d**\n", response->errors_count);
    offset += snprintf(checklist + offset, buffer_size - offset, "- Total Training Items: **%d**\n", response->training_data_count);
    offset += snprintf(checklist + offset, buffer_size - offset, "\n**Grand Total:** %d items\n", 
        response->tasks_count + response->memories_count + response->ideas_count + 
        response->errors_count + response->training_data_count);
    
    *result = checklist;
    return TODOZI_SUCCESS;
}

// Log task to history mega file
TodoziError log_to_history(const Task* task) {
    if (!task) {
        return TODOZI_ERROR_VALIDATION;
    }
    
    const char* home = getenv("HOME");
    if (!home) {
        return TODOZI_ERROR_CONFIG;
    }
    
    // Validate home directory path length
    size_t home_len = strlen(home);
    if (home_len > 512) {
        return TODOZI_ERROR_CONFIG;
    }
    
    char* history_dir_path = (char*)safe_malloc(1024);
    int path_len = snprintf(history_dir_path, 1024, "%s/.todozi/history/core", home);
    if (path_len < 0 || path_len >= 1024) {
        free(history_dir_path);
        return TODOZI_ERROR_CONFIG;
    }
    
    // Ensure history directory exists
    struct stat st = {0};
    if (stat(history_dir_path, &st) == -1) {
        // Create directory structure
        char* dir_path = (char*)safe_malloc(1024);
        snprintf(dir_path, 1024, "%s/.todozi", home);
        mkdir(dir_path, 0755);
        snprintf(dir_path, 1024, "%s/.todozi/history", home);
        mkdir(dir_path, 0755);
        snprintf(dir_path, 1024, "%s/.todozi/history/core", home);
        mkdir(dir_path, 0755);
        free(dir_path);
    }
    
    // Append to mega file
    char* mega_file_path = (char*)safe_malloc(1024);
    int mega_path_len = snprintf(mega_file_path, 1024, "%s/mega", history_dir_path);
    if (mega_path_len < 0 || mega_path_len >= 1024) {
        free(history_dir_path);
        free(mega_file_path);
        return TODOZI_ERROR_CONFIG;
    }
    
    FILE *file = fopen(mega_file_path, "a");
    if (!file) {
        free(history_dir_path);
        free(mega_file_path);
        return TODOZI_ERROR_IO;
    }
    
    time_t now = time(NULL);
    struct tm *tm_info = gmtime(&now);
    if (!tm_info) {
        fclose(file);
        free(history_dir_path);
        free(mega_file_path);
        return TODOZI_ERROR_IO;
    }
    
    char timestamp[64];
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S UTC", tm_info);
    
    fprintf(file, "[%s] EXTRACTED_TASK: %s | Project: %s | Priority: %d | Status: %d | Tags: ",
            timestamp, 
            task->action ? task->action : "(null)",
            task->parent_project ? task->parent_project : "(null)",
            task->priority, 
            task->status);
    
    if (task->tags) {
        for (int i = 0; i < task->tags_count; i++) {
            if (task->tags[i]) {
                if (i > 0) {
                    fprintf(file, ", ");
                }
                fprintf(file, "%s", task->tags[i]);
            }
        }
    }
    fprintf(file, "\n");
    
    fclose(file);
    free(history_dir_path);
    free(mega_file_path);
    return TODOZI_SUCCESS;
}