#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <stdarg.h>

// Forward declarations
typedef enum ChunkingLevel ChunkingLevel;
typedef enum ChunkStatus ChunkStatus;
typedef struct ProjectState ProjectState;
typedef struct ContextWindow ContextWindow;
typedef struct CodeChunk CodeChunk;
typedef struct CodeGenerationGraph CodeGenerationGraph;

// Enum definitions
enum ChunkingLevel {
    CHUNKING_LEVEL_PROJECT,
    CHUNKING_LEVEL_MODULE,
    CHUNKING_LEVEL_CLASS,
    CHUNKING_LEVEL_METHOD,
    CHUNKING_LEVEL_BLOCK,
    CHUNKING_LEVEL_INVALID
};

enum ChunkStatus {
    CHUNK_STATUS_PENDING,
    CHUNK_STATUS_IN_PROGRESS,
    CHUNK_STATUS_COMPLETED,
    CHUNK_STATUS_VALIDATED,
    CHUNK_STATUS_FAILED
};

// String array for ChunkingLevel names
const char* CHUNKING_LEVEL_NAMES[] = {
    "project", "module", "class", "method", "block"
};

const char* CHUNK_STATUS_NAMES[] = {
    "pending", "in_progress", "completed", "validated", "failed"
};

// Helper macros for memory allocation
#define CHECK_ALLOC(p) do { if ((p) == NULL) { fprintf(stderr, "Out of memory\n"); abort(); } } while (0)

// Helper function for string duplication
static char *dup_str(const char *s) {
    if (!s) return NULL;
    char *r = malloc(strlen(s) + 1);
    CHECK_ALLOC(r);
    strcpy(r, s);
    return r;
}

// Helper functions for ChunkingLevel
size_t chunking_level_max_tokens(ChunkingLevel level) {
    switch (level) {
        case CHUNKING_LEVEL_PROJECT: return 100;
        case CHUNKING_LEVEL_MODULE: return 500;
        case CHUNKING_LEVEL_CLASS: return 1000;
        case CHUNKING_LEVEL_METHOD: return 300;
        case CHUNKING_LEVEL_BLOCK: return 100;
        default: return 0;
    }
}

const char* chunking_level_description(ChunkingLevel level) {
    switch (level) {
        case CHUNKING_LEVEL_PROJECT: return "High-level project planning and architecture";
        case CHUNKING_LEVEL_MODULE: return "Major system components and interfaces";
        case CHUNKING_LEVEL_CLASS: return "Class definitions and major functions";
        case CHUNKING_LEVEL_METHOD: return "Individual methods and helper functions";
        case CHUNKING_LEVEL_BLOCK: return "Small code blocks and error handling";
        default: return "";
    }
}

const char* chunking_level_example(ChunkingLevel level) {
    switch (level) {
        case CHUNKING_LEVEL_PROJECT: return "Build web scraper with database storage";
        case CHUNKING_LEVEL_MODULE: return "Create database handler module";
        case CHUNKING_LEVEL_CLASS: return "Implement DatabaseConnection class";
        case CHUNKING_LEVEL_METHOD: return "Write insert_record method";
        case CHUNKING_LEVEL_BLOCK: return "Add error handling for connection timeout";
        default: return "";
    }
}

const char* chunking_level_to_string(ChunkingLevel level) {
    if (level >= 0 && level < CHUNKING_LEVEL_INVALID) {
        return CHUNKING_LEVEL_NAMES[level];
    }
    return "";
}

bool chunking_level_from_string(const char* str, ChunkingLevel *out_level) {
    if (!str || !out_level) return false;
    
    for (int i = 0; i < CHUNKING_LEVEL_INVALID; i++) {
        if (strcmp(str, CHUNKING_LEVEL_NAMES[i]) == 0) {
            *out_level = (ChunkingLevel)i;
            return true;
        }
    }
    return false;
}

const char* chunk_status_to_string(ChunkStatus status) {
    if (status >= 0 && status <= CHUNK_STATUS_FAILED) {
        return CHUNK_STATUS_NAMES[status];
    }
    return "";
}

// Simple dynamic string
typedef struct {
    char* data;
    size_t length;
    size_t capacity;
} String;

String* string_new() {
    String* str = malloc(sizeof(String));
    CHECK_ALLOC(str);
    str->data = malloc(16);
    CHECK_ALLOC(str->data);
    str->data[0] = '\0';
    str->length = 0;
    str->capacity = 16;
    return str;
}

String* string_new_with_capacity(size_t capacity) {
    String* str = malloc(sizeof(String));
    CHECK_ALLOC(str);
    str->data = malloc(capacity);
    CHECK_ALLOC(str->data);
    str->data[0] = '\0';
    str->length = 0;
    str->capacity = capacity;
    return str;
}

void string_free(String* str) {
    if (str) {
        free(str->data);
        free(str);
    }
}

bool string_append(String* str, const char* append_str) {
    if (!str || !append_str) return false;
    
    size_t append_len = strlen(append_str);
    if (str->length + append_len + 1 > str->capacity) {
        size_t new_capacity = (str->length + append_len + 1) * 2;
        char* new_data = realloc(str->data, new_capacity);
        if (!new_data) return false;
        str->data = new_data;
        str->capacity = new_capacity;
    }
    strcat(str->data, append_str);
    str->length += append_len;
    return true;
}

bool string_vappend(String* str, const char* format, va_list args) {
    if (!str || !format) return false;
    
    // First, calculate how much space we need
    va_list args_copy;
    va_copy(args_copy, args);
    int needed = vsnprintf(NULL, 0, format, args_copy);
    va_end(args_copy);
    
    if (needed < 0) return false;
    
    // Check if we need to resize
    if (str->length + needed + 1 > str->capacity) {
        size_t new_capacity = (str->length + needed + 1) * 2;
        char* new_data = realloc(str->data, new_capacity);
        if (!new_data) return false;
        str->data = new_data;
        str->capacity = new_capacity;
    }
    
    // Now do the actual formatting
    int written = vsnprintf(str->data + str->length, str->capacity - str->length, format, args);
    if (written < 0) return false;
    
    str->length += written;
    return true;
}

bool string_appendf(String* str, const char* format, ...) {
    if (!str || !format) return false;
    
    va_list args;
    va_start(args, format);
    bool result = string_vappend(str, format, args);
    va_end(args);
    return result;
}

// Simple dynamic array for strings
typedef struct {
    char** data;
    size_t length;
    size_t capacity;
} StringArray;

StringArray* string_array_new() {
    StringArray* arr = malloc(sizeof(StringArray));
    CHECK_ALLOC(arr);
    arr->data = malloc(sizeof(char*) * 8);
    CHECK_ALLOC(arr->data);
    arr->length = 0;
    arr->capacity = 8;
    return arr;
}

void string_array_free(StringArray* arr) {
    if (arr) {
        for (size_t i = 0; i < arr->length; i++) {
            free(arr->data[i]);
        }
        free(arr->data);
        free(arr);
    }
}

bool string_array_push(StringArray* arr, const char* str) {
    if (!arr || !str) return false;
    
    if (arr->length >= arr->capacity) {
        arr->capacity *= 2;
        char** new_data = realloc(arr->data, sizeof(char*) * arr->capacity);
        if (!new_data) return false;
        arr->data = new_data;
    }
    arr->data[arr->length] = dup_str(str);
    arr->length++;
    return true;
}

bool string_array_contains(StringArray* arr, const char* str) {
    if (!arr || !str) return false;
    
    for (size_t i = 0; i < arr->length; i++) {
        if (strcmp(arr->data[i], str) == 0) {
            return true;
        }
    }
    return false;
}

// Simple hash map for strings (using linear search for simplicity)
typedef struct {
    char* key;
    char* value;
} StringPair;

typedef struct {
    StringPair* data;
    size_t length;
    size_t capacity;
} StringMap;

StringMap* string_map_new() {
    StringMap* map = malloc(sizeof(StringMap));
    CHECK_ALLOC(map);
    map->data = malloc(sizeof(StringPair) * 8);
    CHECK_ALLOC(map->data);
    map->length = 0;
    map->capacity = 8;
    return map;
}

void string_map_free(StringMap* map) {
    if (map) {
        for (size_t i = 0; i < map->length; i++) {
            free(map->data[i].key);
            free(map->data[i].value);
        }
        free(map->data);
        free(map);
    }
}

bool string_map_insert(StringMap* map, const char* key, const char* value) {
    if (!map || !key || !value) return false;
    
    // Check if key already exists
    for (size_t i = 0; i < map->length; i++) {
        if (strcmp(map->data[i].key, key) == 0) {
            free(map->data[i].value);
            map->data[i].value = dup_str(value);
            return true;
        }
    }
    
    // Add new key-value pair
    if (map->length >= map->capacity) {
        map->capacity *= 2;
        StringPair* new_data = realloc(map->data, sizeof(StringPair) * map->capacity);
        if (!new_data) return false;
        map->data = new_data;
    }
    
    map->data[map->length].key = dup_str(key);
    map->data[map->length].value = dup_str(value);
    map->length++;
    return true;
}

const char* string_map_get(StringMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    for (size_t i = 0; i < map->length; i++) {
        if (strcmp(map->data[i].key, key) == 0) {
            return map->data[i].value;
        }
    }
    return NULL;
}

// ProjectState structure
struct ProjectState {
    size_t total_lines;
    size_t max_lines;
    char* current_module;
    StringArray* dependencies;
    StringArray* completed_modules;
    StringArray* pending_modules;
    StringMap* global_variables;
    time_t created_at;
    time_t updated_at;
};

ProjectState* project_state_new(size_t max_lines) {
    ProjectState* state = malloc(sizeof(ProjectState));
    CHECK_ALLOC(state);
    state->total_lines = 0;
    state->max_lines = max_lines;
    state->current_module = dup_str("");
    state->dependencies = string_array_new();
    state->completed_modules = string_array_new();
    state->pending_modules = string_array_new();
    state->global_variables = string_map_new();
    state->created_at = time(NULL);
    state->updated_at = state->created_at;
    return state;
}

void project_state_free(ProjectState* state) {
    if (state) {
        free(state->current_module);
        string_array_free(state->dependencies);
        string_array_free(state->completed_modules);
        string_array_free(state->pending_modules);
        string_map_free(state->global_variables);
        free(state);
    }
}

char* project_state_to_string(ProjectState* state) {
    if (!state) return NULL;
    
    String* str = string_new_with_capacity(512);
    if (!string_append(str, "<project_state>\n")) goto error;
    
    if (!string_appendf(str, "- Total lines written: %zu/%zu\n", state->total_lines, state->max_lines)) goto error;
    if (!string_appendf(str, "- Current module: %s\n", state->current_module)) goto error;
    
    if (!string_append(str, "- Dependencies: ")) goto error;
    for (size_t i = 0; i < state->dependencies->length; i++) {
        if (!string_append(str, state->dependencies->data[i])) goto error;
        if (i < state->dependencies->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Completed modules: ")) goto error;
    for (size_t i = 0; i < state->completed_modules->length; i++) {
        if (!string_append(str, state->completed_modules->data[i])) goto error;
        if (i < state->completed_modules->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Pending modules: ")) goto error;
    for (size_t i = 0; i < state->pending_modules->length; i++) {
        if (!string_append(str, state->pending_modules->data[i])) goto error;
        if (i < state->pending_modules->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Global variables: ")) goto error;
    for (size_t i = 0; i < state->global_variables->length; i++) {
        if (!string_appendf(str, "%s=%s", state->global_variables->data[i].key, state->global_variables->data[i].value)) goto error;
        if (i < state->global_variables->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    struct tm created_tm, updated_tm;
    localtime_r(&state->created_at, &created_tm);
    localtime_r(&state->updated_at, &updated_tm);
    
    char time_buffer[64];
    strftime(time_buffer, sizeof(time_buffer), "%Y-%m-%d %H:%M:%S", &created_tm);
    if (!string_appendf(str, "- Created: %s\n", time_buffer)) goto error;
    
    strftime(time_buffer, sizeof(time_buffer), "%Y-%m-%d %H:%M:%S", &updated_tm);
    if (!string_appendf(str, "- Updated: %s\n", time_buffer)) goto error;
    
    if (!string_append(str, "</project_state>")) goto error;
    
    char* result = dup_str(str->data);
    string_free(str);
    return result;

error:
    string_free(str);
    return NULL;
}

bool project_state_add_completed_module(ProjectState* state, const char* module) {
    if (!state || !module) return false;
    
    if (!string_array_contains(state->completed_modules, module)) {
        if (!string_array_push(state->completed_modules, module)) return false;
        state->updated_at = time(NULL);
    }
    return true;
}

bool project_state_add_pending_module(ProjectState* state, const char* module) {
    if (!state || !module) return false;
    
    if (!string_array_contains(state->pending_modules, module)) {
        if (!string_array_push(state->pending_modules, module)) return false;
        state->updated_at = time(NULL);
    }
    return true;
}

bool project_state_set_global_variable(ProjectState* state, const char* key, const char* value) {
    if (!state || !key || !value) return false;
    
    if (!string_map_insert(state->global_variables, key, value)) return false;
    state->updated_at = time(NULL);
    return true;
}

void project_state_increment_lines(ProjectState* state, size_t lines) {
    if (!state) return;
    
    state->total_lines += lines;
    state->updated_at = time(NULL);
}

// ContextWindow structure
struct ContextWindow {
    char* previous_class;
    char* current_class;
    char* next_planned;
    StringArray* global_vars_in_scope;
    StringArray* imports_used;
    StringMap* function_signatures;
    StringArray* error_patterns_seen;
    time_t created_at;
    time_t updated_at;
};

ContextWindow* context_window_new() {
    ContextWindow* window = malloc(sizeof(ContextWindow));
    CHECK_ALLOC(window);
    window->previous_class = dup_str("");
    window->current_class = dup_str("");
    window->next_planned = dup_str("");
    window->global_vars_in_scope = string_array_new();
    window->imports_used = string_array_new();
    window->function_signatures = string_map_new();
    window->error_patterns_seen = string_array_new();
    window->created_at = time(NULL);
    window->updated_at = window->created_at;
    return window;
}

void context_window_free(ContextWindow* window) {
    if (window) {
        free(window->previous_class);
        free(window->current_class);
        free(window->next_planned);
        string_array_free(window->global_vars_in_scope);
        string_array_free(window->imports_used);
        string_map_free(window->function_signatures);
        string_array_free(window->error_patterns_seen);
        free(window);
    }
}

char* context_window_to_string(ContextWindow* window) {
    if (!window) return NULL;
    
    String* str = string_new_with_capacity(512);
    if (!string_append(str, "<context_window>\n")) goto error;
    
    if (!string_appendf(str, "- Previous class: %s\n", window->previous_class)) goto error;
    if (!string_appendf(str, "- Current class: %s\n", window->current_class)) goto error;
    if (!string_appendf(str, "- Next planned: %s\n", window->next_planned)) goto error;
    
    if (!string_append(str, "- Global variables in scope: ")) goto error;
    for (size_t i = 0; i < window->global_vars_in_scope->length; i++) {
        if (!string_append(str, window->global_vars_in_scope->data[i])) goto error;
        if (i < window->global_vars_in_scope->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Imports used: ")) goto error;
    for (size_t i = 0; i < window->imports_used->length; i++) {
        if (!string_append(str, window->imports_used->data[i])) goto error;
        if (i < window->imports_used->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Function signatures: ")) goto error;
    for (size_t i = 0; i < window->function_signatures->length; i++) {
        if (!string_appendf(str, "%s: %s", window->function_signatures->data[i].key, window->function_signatures->data[i].value)) goto error;
        if (i < window->function_signatures->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Error patterns seen: ")) goto error;
    for (size_t i = 0; i < window->error_patterns_seen->length; i++) {
        if (!string_append(str, window->error_patterns_seen->data[i])) goto error;
        if (i < window->error_patterns_seen->length - 1) {
            if (!string_append(str, ", ")) goto error;
        }
    }
    if (!string_append(str, "\n")) goto error;
    
    struct tm created_tm, updated_tm;
    localtime_r(&window->created_at, &created_tm);
    localtime_r(&window->updated_at, &updated_tm);
    
    char time_buffer[64];
    strftime(time_buffer, sizeof(time_buffer), "%Y-%m-%d %H:%M:%S", &created_tm);
    if (!string_appendf(str, "- Created: %s\n", time_buffer)) goto error;
    
    strftime(time_buffer, sizeof(time_buffer), "%Y-%m-%d %H:%M:%S", &updated_tm);
    if (!string_appendf(str, "- Updated: %s\n", time_buffer)) goto error;
    
    if (!string_append(str, "</context_window>")) goto error;
    
    char* result = dup_str(str->data);
    string_free(str);
    return result;

error:
    string_free(str);
    return NULL;
}

bool context_window_add_import(ContextWindow* window, const char* import) {
    if (!window || !import) return false;
    
    if (!string_array_contains(window->imports_used, import)) {
        if (!string_array_push(window->imports_used, import)) return false;
        window->updated_at = time(NULL);
    }
    return true;
}

bool context_window_add_function_signature(ContextWindow* window, const char* name, const char* signature) {
    if (!window || !name || !signature) return false;
    
    if (!string_map_insert(window->function_signatures, name, signature)) return false;
    window->updated_at = time(NULL);
    return true;
}

bool context_window_add_error_pattern(ContextWindow* window, const char* pattern) {
    if (!window || !pattern) return false;
    
    if (!string_array_contains(window->error_patterns_seen, pattern)) {
        if (!string_array_push(window->error_patterns_seen, pattern)) return false;
        window->updated_at = time(NULL);
    }
    return true;
}

bool context_window_set_current_class(ContextWindow* window, const char* class) {
    if (!window || !class) return false;
    
    free(window->previous_class);
    window->previous_class = window->current_class;
    window->current_class = dup_str(class);
    window->updated_at = time(NULL);
    return true;
}

// CodeChunk structure
struct CodeChunk {
    char* chunk_id;
    ChunkStatus status;
    StringArray* dependencies;
    char* code;
    char* tests;
    bool validated;
    ChunkingLevel level;
    size_t estimated_tokens;
    time_t created_at;
    time_t updated_at;
};

CodeChunk* code_chunk_new(const char* chunk_id, ChunkingLevel level) {
    if (!chunk_id) return NULL;
    
    CodeChunk* chunk = malloc(sizeof(CodeChunk));
    CHECK_ALLOC(chunk);
    chunk->chunk_id = dup_str(chunk_id);
    chunk->status = CHUNK_STATUS_PENDING;
    chunk->dependencies = string_array_new();
    chunk->code = dup_str("");
    chunk->tests = dup_str("");
    chunk->validated = false;
    chunk->level = level;
    chunk->estimated_tokens = 0;
    chunk->created_at = time(NULL);
    chunk->updated_at = chunk->created_at;
    return chunk;
}

void code_chunk_free(CodeChunk* chunk) {
    if (chunk) {
        free(chunk->chunk_id);
        string_array_free(chunk->dependencies);
        free(chunk->code);
        free(chunk->tests);
        free(chunk);
    }
}

bool code_chunk_add_dependency(CodeChunk* chunk, const char* dep) {
    if (!chunk || !dep) return false;
    
    if (!string_array_contains(chunk->dependencies, dep)) {
        if (!string_array_push(chunk->dependencies, dep)) return false;
        chunk->updated_at = time(NULL);
    }
    return true;
}

void code_chunk_set_code(CodeChunk* chunk, const char* code) {
    if (!chunk || !code) return;
    
    free(chunk->code);
    chunk->code = dup_str(code);
    
    // Count tokens (words)
    chunk->estimated_tokens = 0;
    const char* ptr = code;
    bool in_word = false;
    while (*ptr) {
        if (*ptr == ' ' || *ptr == '\t' || *ptr == '\n' || *ptr == '\r') {
            in_word = false;
        } else {
            if (!in_word) {
                chunk->estimated_tokens++;
                in_word = true;
            }
        }
        ptr++;
    }
    
    chunk->updated_at = time(NULL);
}

void code_chunk_set_tests(CodeChunk* chunk, const char* tests) {
    if (!chunk || !tests) return;
    
    free(chunk->tests);
    chunk->tests = dup_str(tests);
    chunk->updated_at = time(NULL);
}

void code_chunk_mark_completed(CodeChunk* chunk) {
    if (!chunk) return;
    
    chunk->status = CHUNK_STATUS_COMPLETED;
    chunk->updated_at = time(NULL);
}

void code_chunk_mark_validated(CodeChunk* chunk) {
    if (!chunk) return;
    
    chunk->validated = true;
    chunk->status = CHUNK_STATUS_VALIDATED;
    chunk->updated_at = time(NULL);
}

void code_chunk_mark_failed(CodeChunk* chunk) {
    if (!chunk) return;
    
    chunk->status = CHUNK_STATUS_FAILED;
    chunk->updated_at = time(NULL);
}

void code_chunk_mark_in_progress(CodeChunk* chunk) {
    if (!chunk) return;
    
    chunk->status = CHUNK_STATUS_IN_PROGRESS;
    chunk->updated_at = time(NULL);
}

// Simple hash map for CodeChunk (using linear search for simplicity)
typedef struct {
    char* key;
    CodeChunk* value;
} ChunkPair;

typedef struct {
    ChunkPair* data;
    size_t length;
    size_t capacity;
} ChunkMap;

ChunkMap* chunk_map_new() {
    ChunkMap* map = malloc(sizeof(ChunkMap));
    CHECK_ALLOC(map);
    map->data = malloc(sizeof(ChunkPair) * 8);
    CHECK_ALLOC(map->data);
    map->length = 0;
    map->capacity = 8;
    return map;
}

void chunk_map_free(ChunkMap* map) {
    if (map) {
        for (size_t i = 0; i < map->length; i++) {
            free(map->data[i].key);
            code_chunk_free(map->data[i].value);
        }
        free(map->data);
        free(map);
    }
}

bool chunk_map_insert(ChunkMap* map, const char* key, CodeChunk* value) {
    if (!map || !key || !value) return false;
    
    // Check if key already exists
    for (size_t i = 0; i < map->length; i++) {
        if (strcmp(map->data[i].key, key) == 0) {
            code_chunk_free(map->data[i].value);
            map->data[i].value = value;
            return true;
        }
    }
    
    // Add new key-value pair
    if (map->length >= map->capacity) {
        map->capacity *= 2;
        ChunkPair* new_data = realloc(map->data, sizeof(ChunkPair) * map->capacity);
        if (!new_data) return false;
        map->data = new_data;
    }
    
    map->data[map->length].key = dup_str(key);
    map->data[map->length].value = value;
    map->length++;
    return true;
}

CodeChunk* chunk_map_get(ChunkMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    for (size_t i = 0; i < map->length; i++) {
        if (strcmp(map->data[i].key, key) == 0) {
            return map->data[i].value;
        }
    }
    return NULL;
}

// CodeGenerationGraph structure
struct CodeGenerationGraph {
    ChunkMap* chunks;
    ProjectState* project_state;
    ContextWindow* context_window;
};

CodeGenerationGraph* code_generation_graph_new(size_t max_lines) {
    CodeGenerationGraph* graph = malloc(sizeof(CodeGenerationGraph));
    CHECK_ALLOC(graph);
    graph->chunks = chunk_map_new();
    graph->project_state = project_state_new(max_lines);
    graph->context_window = context_window_new();
    return graph;
}

void code_generation_graph_free(CodeGenerationGraph* graph) {
    if (graph) {
        chunk_map_free(graph->chunks);
        project_state_free(graph->project_state);
        context_window_free(graph->context_window);
        free(graph);
    }
}

bool code_generation_graph_add_chunk(CodeGenerationGraph* graph, const char* chunk_id, ChunkingLevel level, StringArray* deps) {
    if (!graph || !chunk_id) return false;
    
    CodeChunk* chunk = code_chunk_new(chunk_id, level);
    if (!chunk) return false;
    
    if (deps) {
        for (size_t i = 0; i < deps->length; i++) {
            if (!code_chunk_add_dependency(chunk, deps->data[i])) {
                code_chunk_free(chunk);
                return false;
            }
        }
    }
    
    if (!chunk_map_insert(graph->chunks, chunk_id, chunk)) {
        code_chunk_free(chunk);
        return false;
    }
    return true;
}

StringArray* code_generation_graph_get_ready_chunks(CodeGenerationGraph* graph) {
    if (!graph) return NULL;
    
    StringArray* ready = string_array_new();
    if (!ready) return NULL;
    
    for (size_t i = 0; i < graph->chunks->length; i++) {
        CodeChunk* chunk = graph->chunks->data[i].value;
        if (chunk->status == CHUNK_STATUS_PENDING) {
            bool deps_satisfied = true;
            for (size_t j = 0; j < chunk->dependencies->length; j++) {
                CodeChunk* dep_chunk = chunk_map_get(graph->chunks, chunk->dependencies->data[j]);
                if (!dep_chunk || (dep_chunk->status != CHUNK_STATUS_COMPLETED && dep_chunk->status != CHUNK_STATUS_VALIDATED)) {
                    deps_satisfied = false;
                    break;
                }
            }
            if (deps_satisfied) {
                if (!string_array_push(ready, chunk->chunk_id)) {
                    string_array_free(ready);
                    return NULL;
                }
            }
        }
    }
    
    return ready;
}

CodeChunk* code_generation_graph_get_chunk(CodeGenerationGraph* graph, const char* chunk_id) {
    if (!graph || !chunk_id) return NULL;
    return chunk_map_get(graph->chunks, chunk_id);
}

bool code_generation_graph_update_chunk_code(CodeGenerationGraph* graph, const char* chunk_id, const char* code) {
    if (!graph || !chunk_id || !code) return false;
    
    CodeChunk* chunk = chunk_map_get(graph->chunks, chunk_id);
    if (chunk) {
        code_chunk_set_code(chunk, code);
        
        // Count lines in code
        size_t line_count = 0;
        const char* ptr = code;
        if (*ptr) line_count = 1;
        while (*ptr) {
            if (*ptr == '\n') {
                line_count++;
            }
            ptr++;
        }
        
        project_state_increment_lines(graph->project_state, line_count);
        return true;
    }
    return false;
}

bool code_generation_graph_update_chunk_tests(CodeGenerationGraph* graph, const char* chunk_id, const char* tests) {
    if (!graph || !chunk_id || !tests) return false;
    
    CodeChunk* chunk = chunk_map_get(graph->chunks, chunk_id);
    if (chunk) {
        code_chunk_set_tests(chunk, tests);
        return true;
    }
    return false;
}

bool code_generation_graph_mark_chunk_completed(CodeGenerationGraph* graph, const char* chunk_id) {
    if (!graph || !chunk_id) return false;
    
    CodeChunk* chunk = chunk_map_get(graph->chunks, chunk_id);
    if (chunk) {
        code_chunk_mark_completed(chunk);
        return project_state_add_completed_module(graph->project_state, chunk_id);
    }
    return false;
}

bool code_generation_graph_mark_chunk_validated(CodeGenerationGraph* graph, const char* chunk_id) {
    if (!graph || !chunk_id) return false;
    
    CodeChunk* chunk = chunk_map_get(graph->chunks, chunk_id);
    if (chunk) {
        code_chunk_mark_validated(chunk);
        return true;
    }
    return false;
}

bool code_generation_graph_mark_chunk_in_progress(CodeGenerationGraph* graph, const char* chunk_id) {
    if (!graph || !chunk_id) return false;
    
    CodeChunk* chunk = chunk_map_get(graph->chunks, chunk_id);
    if (chunk) {
        code_chunk_mark_in_progress(chunk);
        return true;
    }
    return false;
}

char* code_generation_graph_get_project_summary(CodeGenerationGraph* graph) {
    if (!graph) return NULL;
    
    size_t completed_count = 0;
    size_t pending_count = 0;
    size_t in_progress_count = 0;
    
    for (size_t i = 0; i < graph->chunks->length; i++) {
        CodeChunk* chunk = graph->chunks->data[i].value;
        if (chunk->status == CHUNK_STATUS_COMPLETED || chunk->status == CHUNK_STATUS_VALIDATED) {
            completed_count++;
        } else if (chunk->status == CHUNK_STATUS_PENDING) {
            pending_count++;
        } else if (chunk->status == CHUNK_STATUS_IN_PROGRESS) {
            in_progress_count++;
        }
    }
    
    size_t total_count = graph->chunks->length;
    
    char* state_str = project_state_to_string(graph->project_state);
    if (!state_str) return NULL;
    
    char* context_str = context_window_to_string(graph->context_window);
    if (!context_str) {
        free(state_str);
        return NULL;
    }
    
    String* str = string_new_with_capacity(1024);
    if (!str) {
        free(state_str);
        free(context_str);
        return NULL;
    }
    
    if (!string_append(str, "<project_summary>\n")) goto error;
    if (!string_appendf(str, "- Total chunks: %zu\n", total_count)) goto error;
    if (!string_appendf(str, "- Completed: %zu\n", completed_count)) goto error;
    if (!string_appendf(str, "- In progress: %zu\n", in_progress_count)) goto error;
    if (!string_appendf(str, "- Pending: %zu\n", pending_count)) goto error;
    
    if (!string_append(str, "- Project state: ")) goto error;
    if (!string_append(str, state_str)) goto error;
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "- Context window: ")) goto error;
    if (!string_append(str, context_str)) goto error;
    if (!string_append(str, "\n")) goto error;
    
    if (!string_append(str, "</project_summary>")) goto error;
    
    free(state_str);
    free(context_str);
    
    char* result = dup_str(str->data);
    string_free(str);
    return result;

error:
    free(state_str);
    free(context_str);
    string_free(str);
    return NULL;
}

char* code_generation_graph_get_next_chunk_to_work_on(CodeGenerationGraph* graph) {
    if (!graph) return NULL;
    
    StringArray* ready = code_generation_graph_get_ready_chunks(graph);
    if (!ready) return NULL;
    
    char* result = NULL;
    if (ready->length > 0) {
        result = dup_str(ready->data[0]);
    }
    string_array_free(ready);
    return result;
}

// Simple hash set for tracking visited nodes during dependency chain building
typedef struct {
    char** data;
    size_t length;
    size_t capacity;
} StringSet;

StringSet* string_set_new() {
    StringSet* set = malloc(sizeof(StringSet));
    CHECK_ALLOC(set);
    set->data = malloc(sizeof(char*) * 8);
    CHECK_ALLOC(set->data);
    set->length = 0;
    set->capacity = 8;
    return set;
}

void string_set_free(StringSet* set) {
    if (set) {
        for (size_t i = 0; i < set->length; i++) {
            free(set->data[i]);
        }
        free(set->data);
        free(set);
    }
}

bool string_set_contains(StringSet* set, const char* str) {
    if (!set || !str) return false;
    
    for (size_t i = 0; i < set->length; i++) {
        if (strcmp(set->data[i], str) == 0) {
            return true;
        }
    }
    return false;
}

bool string_set_add(StringSet* set, const char* str) {
    if (!set || !str) return false;
    
    // Check if already exists
    if (string_set_contains(set, str)) {
        return true;
    }
    
    // Add new string
    if (set->length >= set->capacity) {
        set->capacity *= 2;
        char** new_data = realloc(set->data, sizeof(char*) * set->capacity);
        if (!new_data) return false;
        set->data = new_data;
    }
    
    set->data[set->length] = dup_str(str);
    set->length++;
    return true;
}

// Build dependency chain with cycle detection
static bool code_generation_graph_build_dependency_chain(CodeGenerationGraph* graph, const char* chunk_id, StringArray* chain, StringSet* visited) {
    if (!graph || !chunk_id || !chain || !visited) return false;
    
    // Check for cycles
    if (string_set_contains(visited, chunk_id)) {
        return true; // Cycle detected, but we continue
    }
    
    if (!string_set_add(visited, chunk_id)) return false;
    
    CodeChunk* chunk = chunk_map_get(graph->chunks, chunk_id);
    if (chunk) {
        // Add dependencies first (depth-first)
        for (size_t i = 0; i < chunk->dependencies->length; i++) {
            if (!code_generation_graph_build_dependency_chain(graph, chunk->dependencies->data[i], chain, visited)) {
                return false;
            }
        }
        // Then add current chunk
        if (!string_array_push(chain, chunk_id)) return false;
    }
    return true;
}

StringArray* code_generation_graph_get_dependency_chain(CodeGenerationGraph* graph, const char* chunk_id) {
    if (!graph || !chunk_id) return NULL;
    
    StringArray* chain = string_array_new();
    if (!chain) return NULL;
    
    StringSet* visited = string_set_new();
    if (!visited) {
        string_array_free(chain);
        return NULL;
    }
    
    if (!code_generation_graph_build_dependency_chain(graph, chunk_id, chain, visited)) {
        string_set_free(visited);
        string_array_free(chain);
        return NULL;
    }
    
    string_set_free(visited);
    return chain;
}

// Simple parsing functions (without regex)
static bool find_substring(const char* str, const char* substr, size_t* start, size_t* end) {
    if (!str || !substr || !start || !end) return false;
    
    char* pos = strstr(str, substr);
    if (pos) {
        *start = pos - str;
        *end = *start + strlen(substr);
        return true;
    }
    return false;
}

CodeChunk* parse_chunking_format(const char* chunk_text) {
    if (!chunk_text) return NULL;
    
    size_t start, end;
    if (!find_substring(chunk_text, "<chunk>", &start, &end)) {
        return NULL;
    }
    
    size_t content_start = end;
    if (!find_substring(chunk_text, "</chunk>", &start, &end)) {
        return NULL;
    }
    
    size_t content_end = start;
    
    // Extract content
    if (content_end <= content_start) return NULL;
    
    size_t content_len = content_end - content_start;
    char* content = malloc(content_len + 1);
    if (!content) return NULL;
    
    strncpy(content, chunk_text + content_start, content_len);
    content[content_len] = '\0';
    
    // Split by ';'
    #define MAX_PARTS 10
    char* parts[MAX_PARTS];
    int part_count = 0;
    
    char* token = strtok(content, ";");
    while (token && part_count < MAX_PARTS) {
        // Trim leading whitespace
        while (*token == ' ' || *token == '\t') token++;
        
        // Trim trailing whitespace
        char* end = token + strlen(token) - 1;
        while (end > token && (*end == ' ' || *end == '\t')) end--;
        *(end + 1) = '\0';
        
        parts[part_count] = dup_str(token);
        if (!parts[part_count]) {
            // Cleanup on error
            for (int i = 0; i < part_count; i++) {
                free(parts[i]);
            }
            free(content);
            return NULL;
        }
        part_count++;
        token = strtok(NULL, ";");
    }
    
    if (part_count < 3) {
        // Cleanup
        for (int i = 0; i < part_count; i++) {
            free(parts[i]);
        }
        free(content);
        return NULL;
    }
    
    char* chunk_id = parts[0];
    ChunkingLevel level;
    if (!chunking_level_from_string(parts[1], &level)) {
        // Cleanup
        for (int i = 0; i < part_count; i++) {
            free(parts[i]);
        }
        free(content);
        return NULL;
    }
    
    CodeChunk* chunk = code_chunk_new(chunk_id, level);
    if (!chunk) {
        // Cleanup
        for (int i = 0; i < part_count; i++) {
            free(parts[i]);
        }
        free(content);
        return NULL;
    }
    
    if (part_count > 3) {
        // Parse dependencies
        char* deps_str = parts[3];
        char* dep_token = strtok(deps_str, ",");
        while (dep_token) {
            // Trim whitespace
            while (*dep_token == ' ' || *dep_token == '\t') dep_token++;
            
            char* dep_end = dep_token + strlen(dep_token) - 1;
            while (dep_end > dep_token && (*dep_end == ' ' || *dep_end == '\t')) dep_end--;
            *(dep_end + 1) = '\0';
            
            if (strlen(dep_token) > 0) {
                if (!code_chunk_add_dependency(chunk, dep_token)) {
                    // Cleanup
                    code_chunk_free(chunk);
                    for (int i = 0; i < part_count; i++) {
                        free(parts[i]);
                    }
                    free(content);
                    return NULL;
                }
            }
            dep_token = strtok(NULL, ",");
        }
    }
    
    if (part_count > 4) {
        code_chunk_set_code(chunk, parts[4]);
    }
    
    // Cleanup
    for (int i = 0; i < part_count; i++) {
        free(parts[i]);
    }
    free(content);
    
    return chunk;
}

// Example usage
int main() {
    // Create a graph
    CodeGenerationGraph* graph = code_generation_graph_new(1000);
    if (!graph) {
        fprintf(stderr, "Failed to create graph\n");
        return 1;
    }
    
    // Add some chunks
    StringArray* deps1 = string_array_new();
    if (!deps1) {
        code_generation_graph_free(graph);
        return 1;
    }
    
    if (!code_generation_graph_add_chunk(graph, "chunk1", CHUNKING_LEVEL_MODULE, deps1)) {
        string_array_free(deps1);
        code_generation_graph_free(graph);
        return 1;
    }
    
    StringArray* deps2 = string_array_new();
    if (!deps2) {
        string_array_free(deps1);
        code_generation_graph_free(graph);
        return 1;
    }
    
    if (!string_array_push(deps2, "chunk1")) {
        string_array_free(deps1);
        string_array_free(deps2);
        code_generation_graph_free(graph);
        return 1;
    }
    
    if (!code_generation_graph_add_chunk(graph, "chunk2", CHUNKING_LEVEL_CLASS, deps2)) {
        string_array_free(deps1);
        string_array_free(deps2);
        code_generation_graph_free(graph);
        return 1;
    }
    
    // Get ready chunks
    StringArray* ready = code_generation_graph_get_ready_chunks(graph);
    if (!ready) {
        string_array_free(deps1);
        string_array_free(deps2);
        code_generation_graph_free(graph);
        return 1;
    }
    
    printf("Ready chunks: ");
    for (size_t i = 0; i < ready->length; i++) {
        printf("%s ", ready->data[i]);
    }
    printf("\n");
    
    // Update chunk code
    if (!code_generation_graph_update_chunk_code(graph, "chunk1", "print('Hello World')")) {
        string_array_free(deps1);
        string_array_free(deps2);
        string_array_free(ready);
        code_generation_graph_free(graph);
        return 1;
    }
    
    // Mark chunk as completed
    if (!code_generation_graph_mark_chunk_completed(graph, "chunk1")) {
        string_array_free(deps1);
        string_array_free(deps2);
        string_array_free(ready);
        code_generation_graph_free(graph);
        return 1;
    }
    
    // Get ready chunks again
    string_array_free(ready);
    ready = code_generation_graph_get_ready_chunks(graph);
    if (!ready) {
        string_array_free(deps1);
        string_array_free(deps2);
        code_generation_graph_free(graph);
        return 1;
    }
    
    printf("Ready chunks after completion: ");
    for (size_t i = 0; i < ready->length; i++) {
        printf("%s ", ready->data[i]);
    }
    printf("\n");
    
    // Get project summary
    char* summary = code_generation_graph_get_project_summary(graph);
    if (summary) {
        printf("Project summary:\n%s\n", summary);
        free(summary);
    }
    
    // Cleanup
    string_array_free(deps1);
    string_array_free(deps2);
    string_array_free(ready);
    code_generation_graph_free(graph);
    
    return 0;
}