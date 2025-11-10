#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>  // For strcasestr on POSIX systems
#include <time.h>
#include <ctype.h>
#include <uuid/uuid.h>
#include "json-c/json.h"
#include <regex.h>
#include <errno.h>
#include <unistd.h>
#include <sys/wait.h>
#include <stdarg.h>
#include <assert.h>

// Forward declarations
typedef struct ChecklistItem ChecklistItem;
typedef struct ExtractedAction ExtractedAction;
typedef struct ProcessedContent ProcessedContent;
typedef struct ProcessingStats ProcessingStats;
typedef struct ProcessedAction ProcessedAction;
typedef struct ConversationSession ConversationSession;
typedef struct TodoziProcessorState TodoziProcessorState;
typedef struct ParsedContent ParsedContent;
typedef struct ExtractionResult ExtractionResult;
typedef struct ProcessingResult ProcessingResult;
typedef struct TdzContentProcessorTool TdzContentProcessorTool;

// Error handling
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_STORAGE,
    TODOZI_ERROR_PARSE
} TodoziError;

// Basic structures
struct ChecklistItem {
    char* id;
    char* content;
    char* priority;
    int completed;
    time_t created_at;
    char* source;
};

struct ExtractedAction {
    char* id;
    char* action_type;
    json_object* parameters;
    double confidence;
};

struct ProcessingStats {
    size_t content_length;
    size_t tool_calls_found;
    size_t tags_extracted;
    size_t checklists_generated;
    long processing_time_ms;
};

struct ProcessedContent {
    char* id;
    char* session_id;
    char* raw_content;
    char* cleaned_content;
    time_t timestamp;
    ExtractedAction* extracted_items;
    size_t extracted_items_count;
    ChecklistItem* checklist_items;
    size_t checklist_items_count;
    ProcessedAction* tool_calls;
    size_t tool_calls_count;
    ProcessingStats processing_stats;
};

struct ProcessedAction {
    char* id;
    char* action_type;
    char* description;
    time_t timestamp;
    int success;
    char* result;
};

struct ConversationSession {
    char* id;
    time_t start_time;
    time_t last_activity;
    char* topic;
    unsigned int participant_count;
    unsigned int message_count;
};

// Hash map implementation (simplified)
typedef struct HashMapEntry {
    char* key;
    void* value;
    struct HashMapEntry* next;
} HashMapEntry;

typedef struct {
    HashMapEntry** buckets;
    size_t size;
} HashMap;

HashMap* hashmap_create(size_t size) {
    if (size == 0) size = 1;  // Prevent division by zero
    HashMap* map = malloc(sizeof(HashMap));
    if (!map) return NULL;
    
    map->buckets = calloc(size, sizeof(HashMapEntry*));
    if (!map->buckets) {
        free(map);
        return NULL;
    }
    map->size = size;
    return map;
}

void hashmap_destroy(HashMap* map) {
    if (!map) return;
    
    for (size_t i = 0; i < map->size; i++) {
        HashMapEntry* entry = map->buckets[i];
        while (entry) {
            HashMapEntry* next = entry->next;
            free(entry->key);
            // Note: value cleanup is caller's responsibility
            free(entry);
            entry = next;
        }
    }
    free(map->buckets);
    free(map);
}

int hashmap_put(HashMap* map, const char* key, void* value) {
    if (!map || !key) return -1;
    
    unsigned int hash = 0;
    for (int i = 0; key[i]; i++) {
        hash = hash * 31 + key[i];
    }
    hash = hash % map->size;
    
    // Check if key already exists and update it
    HashMapEntry* entry = map->buckets[hash];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            entry->value = value;
            return 0;
        }
        entry = entry->next;
    }
    
    // Create new entry
    entry = malloc(sizeof(HashMapEntry));
    if (!entry) return -1;
    
    entry->key = strdup(key);
    if (!entry->key) {
        free(entry);
        return -1;
    }
    entry->value = value;
    entry->next = map->buckets[hash];
    map->buckets[hash] = entry;
    return 0;
}

void* hashmap_get(HashMap* map, const char* key) {
    unsigned int hash = 0;
    for (int i = 0; key[i]; i++) {
        hash = hash * 31 + key[i];
    }
    hash = hash % map->size;
    
    HashMapEntry* entry = map->buckets[hash];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            return entry->value;
        }
        entry = entry->next;
    }
    return NULL;
}

struct TodoziProcessorState {
    HashMap* active_sessions;
    ProcessedAction* recent_actions;
    size_t recent_actions_count;
    size_t recent_actions_capacity;
    ChecklistItem* checklist_items;
    size_t checklist_items_count;
    size_t checklist_items_capacity;
    ProcessedContent* processed_contents;
    size_t processed_contents_count;
    size_t processed_contents_capacity;
};

struct ParsedContent {
    char* text_content;
    json_object* json_content;
    json_object* tool_calls;
};

struct ExtractionResult {
    char** extracted_tags;
    size_t extracted_tags_count;
    json_object* tool_calls;
    char** natural_patterns;
    size_t natural_patterns_count;
};

struct ProcessingResult {
    ProcessedAction* actions;
    size_t actions_count;
};

struct TdzContentProcessorTool {
    TodoziProcessorState* state;
    char** natural_language_patterns;
    size_t patterns_count;
};

// Function prototypes
TodoziProcessorState* todozi_processor_state_new(void);
void todozi_processor_state_free(TodoziProcessorState* state);
int todozi_processor_state_add_checklist_item(TodoziProcessorState* state, ChecklistItem* item);
int todozi_processor_state_add_recent_action(TodoziProcessorState* state, ProcessedAction* action);
TodoziError todozi_processor_state_save_processed_content(TodoziProcessorState* state, const char* raw, const char* cleaned, const char* session_id);
TdzContentProcessorTool* tdz_content_processor_tool_new(TodoziProcessorState* state);
void tdz_content_processor_tool_free(TdzContentProcessorTool* tool);
void tdz_content_processor_tool_initialize_patterns(TdzContentProcessorTool* tool);
ParsedContent* parse_raw_content(const char* content);
ParsedContent* parse_json_content(json_object* json);
void parsed_content_free(ParsedContent* parsed);
ParsedContent* parse_text_content(const char* content);
ExtractionResult* extract_todozi_data(TdzContentProcessorTool* tool, ParsedContent* parsed);
void extraction_result_free(ExtractionResult* result);
ProcessingResult* process_tool_calls(TdzContentProcessorTool* tool, json_object* tool_calls);
void processing_result_free(ProcessingResult* result);
char* clean_content(const char* original, char** extracted_tags, size_t tags_count);
char** extract_natural_language_patterns(const char* text, size_t* count);
ChecklistItem* extract_checklist_items(const char* text, size_t* count);
void checklist_items_free(ChecklistItem* items, size_t count);
void ensure_session_exists(TodoziProcessorState* state, const char* session_id, ParsedContent* parsed);
char* infer_topic(const char* text);
char* generate_response(const char* cleaned_content, TodoziProcessorState* state, ProcessingResult* processing, ProcessingStats stats);
ProcessedAction* process_create_task_call(json_object* tool_call);
ProcessedAction* process_search_call(json_object* tool_call);
ProcessedAction* process_update_call(json_object* tool_call);
ProcessedAction* process_memory_call(json_object* tool_call);
ProcessedAction* process_idea_call(json_object* tool_call);
void processed_action_free(ProcessedAction* action);
char* execute_binary_command(const char* command, char* args[], size_t args_count, int* success);
char* tdz_cnt(const char* content, const char* session_id);
TodoziProcessorState* initialize_tdz_content_processor(void);

// Implementation
TodoziProcessorState* todozi_processor_state_new(void) {
    TodoziProcessorState* state = malloc(sizeof(TodoziProcessorState));
    if (!state) return NULL;
    
    state->active_sessions = hashmap_create(100);
    if (!state->active_sessions) {
        free(state);
        return NULL;
    }
    
    state->recent_actions = NULL;
    state->recent_actions_count = 0;
    state->recent_actions_capacity = 0;
    state->checklist_items = NULL;
    state->checklist_items_count = 0;
    state->checklist_items_capacity = 0;
    state->processed_contents = NULL;
    state->processed_contents_count = 0;
    state->processed_contents_capacity = 0;
    
    return state;
}

void todozi_processor_state_free(TodoziProcessorState* state) {
    if (!state) return;
    
    // Free hashmap and sessions
    if (state->active_sessions) {
        // Free all session objects
        for (size_t i = 0; i < state->active_sessions->size; i++) {
            HashMapEntry* entry = state->active_sessions->buckets[i];
            while (entry) {
                ConversationSession* session = (ConversationSession*)entry->value;
                if (session) {
                    free(session->id);
                    free(session->topic);
                    free(session);
                }
                entry = entry->next;
            }
        }
        hashmap_destroy(state->active_sessions);
    }
    
    // Free recent actions
    if (state->recent_actions) {
        for (size_t i = 0; i < state->recent_actions_count; i++) {
            processed_action_free(&state->recent_actions[i]);
        }
        free(state->recent_actions);
    }
    
    // Free checklist items
    if (state->checklist_items) {
        checklist_items_free(state->checklist_items, state->checklist_items_count);
        free(state->checklist_items);
    }
    
    // Free processed contents
    if (state->processed_contents) {
        for (size_t i = 0; i < state->processed_contents_count; i++) {
            ProcessedContent* pc = &state->processed_contents[i];
            free(pc->id);
            free(pc->session_id);
            free(pc->raw_content);
            free(pc->cleaned_content);
            // Free extracted items
            if (pc->extracted_items) {
                for (size_t j = 0; j < pc->extracted_items_count; j++) {
                    free(pc->extracted_items[j].id);
                    free(pc->extracted_items[j].action_type);
                    if (pc->extracted_items[j].parameters) {
                        json_object_put(pc->extracted_items[j].parameters);
                    }
                }
                free(pc->extracted_items);
            }
            // Free checklist items
            if (pc->checklist_items) {
                checklist_items_free(pc->checklist_items, pc->checklist_items_count);
                free(pc->checklist_items);
            }
            // Free tool calls
            if (pc->tool_calls) {
                for (size_t j = 0; j < pc->tool_calls_count; j++) {
                    processed_action_free(&pc->tool_calls[j]);
                }
                free(pc->tool_calls);
            }
        }
        free(state->processed_contents);
    }
    
    free(state);
}

int todozi_processor_state_add_checklist_item(TodoziProcessorState* state, ChecklistItem* item) {
    if (!state || !item) return -1;
    
    if (state->checklist_items_count >= state->checklist_items_capacity) {
        size_t new_capacity = state->checklist_items_capacity == 0 ? 10 : state->checklist_items_capacity * 2;
        ChecklistItem* new_items = realloc(state->checklist_items, new_capacity * sizeof(ChecklistItem));
        if (!new_items) return -1;
        state->checklist_items = new_items;
        state->checklist_items_capacity = new_capacity;
    }
    state->checklist_items[state->checklist_items_count++] = *item;
    return 0;
}

int todozi_processor_state_add_recent_action(TodoziProcessorState* state, ProcessedAction* action) {
    if (!state || !action) return -1;
    
    if (state->recent_actions_count >= state->recent_actions_capacity) {
        size_t new_capacity = state->recent_actions_capacity == 0 ? 10 : state->recent_actions_capacity * 2;
        ProcessedAction* new_actions = realloc(state->recent_actions, new_capacity * sizeof(ProcessedAction));
        if (!new_actions) return -1;
        state->recent_actions = new_actions;
        state->recent_actions_capacity = new_capacity;
    }
    
    // Deep copy the action
    ProcessedAction* dest = &state->recent_actions[state->recent_actions_count];
    dest->id = action->id ? strdup(action->id) : NULL;
    dest->action_type = action->action_type ? strdup(action->action_type) : NULL;
    dest->description = action->description ? strdup(action->description) : NULL;
    dest->result = action->result ? strdup(action->result) : NULL;
    dest->timestamp = action->timestamp;
    dest->success = action->success;
    
    state->recent_actions_count++;
    
    if (state->recent_actions_count > 100) {
        // Remove oldest actions
        for (size_t i = 0; i < state->recent_actions_count - 100; i++) {
            processed_action_free(&state->recent_actions[i]);
        }
        memmove(state->recent_actions, state->recent_actions + (state->recent_actions_count - 100), 
                100 * sizeof(ProcessedAction));
        state->recent_actions_count = 100;
    }
    return 0;
}

TodoziError todozi_processor_state_save_processed_content(TodoziProcessorState* state, const char* raw, const char* cleaned, const char* session_id) {
    if (!state || !raw || !cleaned || !session_id) {
        return TODOZI_ERROR_VALIDATION;
    }
    
    if (state->processed_contents_count >= state->processed_contents_capacity) {
        size_t new_capacity = state->processed_contents_capacity == 0 ? 10 : state->processed_contents_capacity * 2;
        ProcessedContent* new_contents = realloc(state->processed_contents, new_capacity * sizeof(ProcessedContent));
        if (!new_contents) return TODOZI_ERROR_STORAGE;
        state->processed_contents = new_contents;
        state->processed_contents_capacity = new_capacity;
    }
    
    ProcessedContent* processed = &state->processed_contents[state->processed_contents_count++];
    
    uuid_t uuid;
    char uuid_str[37];
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    processed->id = strdup(uuid_str);
    processed->session_id = strdup(session_id);
    processed->raw_content = strdup(raw);
    processed->cleaned_content = strdup(cleaned);
    
    if (!processed->id || !processed->session_id || !processed->raw_content || !processed->cleaned_content) {
        free(processed->id);
        free(processed->session_id);
        free(processed->raw_content);
        free(processed->cleaned_content);
        state->processed_contents_count--;
        return TODOZI_ERROR_STORAGE;
    }
    
    processed->timestamp = time(NULL);
    processed->extracted_items = NULL;
    processed->extracted_items_count = 0;
    processed->checklist_items = NULL;
    processed->checklist_items_count = 0;
    processed->tool_calls = NULL;
    processed->tool_calls_count = 0;
    processed->processing_stats.content_length = strlen(raw);
    processed->processing_stats.tool_calls_found = 0;
    processed->processing_stats.tags_extracted = 0;
    processed->processing_stats.checklists_generated = 0;
    processed->processing_stats.processing_time_ms = 0;
    
    return TODOZI_SUCCESS;
}

TdzContentProcessorTool* tdz_content_processor_tool_new(TodoziProcessorState* state) {
    if (!state) return NULL;
    
    TdzContentProcessorTool* tool = malloc(sizeof(TdzContentProcessorTool));
    if (!tool) return NULL;
    
    tool->state = state;
    tool->natural_language_patterns = NULL;
    tool->patterns_count = 0;
    
    tdz_content_processor_tool_initialize_patterns(tool);
    return tool;
}

void tdz_content_processor_tool_free(TdzContentProcessorTool* tool) {
    if (!tool) return;
    
    if (tool->natural_language_patterns) {
        for (size_t i = 0; i < tool->patterns_count; i++) {
            free(tool->natural_language_patterns[i]);
        }
        free(tool->natural_language_patterns);
    }
    
    free(tool);
}

void tdz_content_processor_tool_initialize_patterns(TdzContentProcessorTool* tool) {
    if (!tool) return;
    
    const char* patterns[] = {
        "we should", "I need to", "let's", "we need to", "don't forget",
        "remember to", "make sure", "important:", "note:", "todo:",
        "add to checklist", "checklist item", "action item", "next step"
    };
    
    tool->patterns_count = sizeof(patterns) / sizeof(patterns[0]);
    tool->natural_language_patterns = malloc(tool->patterns_count * sizeof(char*));
    if (!tool->natural_language_patterns) {
        tool->patterns_count = 0;
        return;
    }
    
    for (size_t i = 0; i < tool->patterns_count; i++) {
        tool->natural_language_patterns[i] = strdup(patterns[i]);
        if (!tool->natural_language_patterns[i]) {
            // Cleanup on failure
            for (size_t j = 0; j < i; j++) {
                free(tool->natural_language_patterns[j]);
            }
            free(tool->natural_language_patterns);
            tool->natural_language_patterns = NULL;
            tool->patterns_count = 0;
            return;
        }
    }
}

ParsedContent* parse_raw_content(const char* content) {
    json_object* json = json_tokener_parse(content);
    if (json != NULL) {
        ParsedContent* result = parse_json_content(json);
        // Don't free json here - parsed_content_free will handle it
        return result;
    } else {
        return parse_text_content(content);
    }
}

ParsedContent* parse_json_content(json_object* json) {
    if (!json) return NULL;
    
    ParsedContent* parsed = malloc(sizeof(ParsedContent));
    if (!parsed) return NULL;
    
    parsed->text_content = malloc(1);
    if (!parsed->text_content) {
        free(parsed);
        return NULL;
    }
    parsed->text_content[0] = '\0';
    parsed->json_content = json;  // Parent manages reference
    parsed->tool_calls = NULL;
    
    // Extract text content
    json_object* content_obj = json_object_object_get(json, "content");
    if (content_obj && json_object_is_type(content_obj, json_type_string)) {
        const char* content_str = json_object_get_string(content_obj);
        size_t current_len = strlen(parsed->text_content);
        size_t new_len = current_len + strlen(content_str) + 1;
        char* new_text = realloc(parsed->text_content, new_len);
        if (!new_text) {
            free(parsed->text_content);
            if (parsed->json_content) json_object_put(parsed->json_content);
            free(parsed);
            return NULL;
        }
        parsed->text_content = new_text;
        strcpy(parsed->text_content + current_len, content_str);
    }
    
    json_object* message_obj = json_object_object_get(json, "message");
    if (message_obj && json_object_is_type(message_obj, json_type_string)) {
        const char* message_str = json_object_get_string(message_obj);
        size_t current_len = strlen(parsed->text_content);
        size_t new_len = current_len + strlen(message_str) + 1;
        char* new_text = realloc(parsed->text_content, new_len);
        if (!new_text) {
            free(parsed->text_content);
            if (parsed->json_content) json_object_put(parsed->json_content);
            free(parsed);
            return NULL;
        }
        parsed->text_content = new_text;
        strcpy(parsed->text_content + current_len, message_str);
    }
    
    // Extract tool calls
    json_object* tool_calls_obj = json_object_object_get(json, "tool_calls");
    if (tool_calls_obj && json_object_is_type(tool_calls_obj, json_type_array)) {
        parsed->tool_calls = tool_calls_obj;  // Parent manages reference
    }
    
    return parsed;
}

void parsed_content_free(ParsedContent* parsed) {
    if (!parsed) return;
    
    free(parsed->text_content);
    if (parsed->json_content) json_object_put(parsed->json_content);
    if (parsed->tool_calls) json_object_put(parsed->tool_calls);
    free(parsed);
}

ParsedContent* parse_text_content(const char* content) {
    if (!content) return NULL;
    
    ParsedContent* parsed = malloc(sizeof(ParsedContent));
    if (!parsed) return NULL;
    
    parsed->text_content = strdup(content);
    if (!parsed->text_content) {
        free(parsed);
        return NULL;
    }
    parsed->json_content = NULL;
    parsed->tool_calls = NULL;
    return parsed;
}

ExtractionResult* extract_todozi_data(TdzContentProcessorTool* tool, ParsedContent* parsed) {
    ExtractionResult* result = malloc(sizeof(ExtractionResult));
    result->extracted_tags = NULL;
    result->extracted_tags_count = 0;
    result->tool_calls = NULL;
    result->natural_patterns = NULL;
    result->natural_patterns_count = 0;
    
    const char* tag_patterns[] = {
        "<todozi>.*?</todozi>", "<memory>.*?</memory>", "<idea>.*?</idea>",
        "<todozi_agent>.*?</todozi_agent>", "<chunk>.*?</chunk>", "<tdz>.*?</tdz>"
    };
    
    // Extract tags using regex (simplified implementation)
    for (size_t i = 0; i < sizeof(tag_patterns) / sizeof(tag_patterns[0]); i++) {
        regex_t regex;
        if (regcomp(&regex, tag_patterns[i], REG_EXTENDED) == 0) {
            regmatch_t matches[1];
            const char* text = parsed->text_content;
            while (regexec(&regex, text, 1, matches, 0) == 0) {
                result->extracted_tags = realloc(result->extracted_tags, (result->extracted_tags_count + 1) * sizeof(char*));
                int start = matches[0].rm_so;
                int end = matches[0].rm_eo;
                int len = end - start;
                result->extracted_tags[result->extracted_tags_count] = malloc(len + 1);
                strncpy(result->extracted_tags[result->extracted_tags_count], text + start, len);
                result->extracted_tags[result->extracted_tags_count][len] = '\0';
                result->extracted_tags_count++;
                text += end;
            }
            regfree(&regex);
        }
    }
    
    // Extract tool calls
    if (parsed->tool_calls) {
        result->tool_calls = parsed->tool_calls;  // Parent manages reference
    }
    
    // Extract natural language patterns
    result->natural_patterns = extract_natural_language_patterns(parsed->text_content, &result->natural_patterns_count);
    
    return result;
}

void extraction_result_free(ExtractionResult* result) {
    if (!result) return;
    
    if (result->extracted_tags) {
        for (size_t i = 0; i < result->extracted_tags_count; i++) {
            free(result->extracted_tags[i]);
        }
        free(result->extracted_tags);
    }
    
    // Note: tool_calls is managed by parent json object, don't free here
    // The parent (parsed->json_content) will free it when parsed_content_free is called
    
    if (result->natural_patterns) {
        for (size_t i = 0; i < result->natural_patterns_count; i++) {
            free(result->natural_patterns[i]);
        }
        free(result->natural_patterns);
    }
    
    free(result);
}

char** extract_natural_language_patterns(const char* text, size_t* count) {
    *count = 0;
    char** patterns = NULL;
    
    const char* action_patterns[] = {
        "we should", "I need to", "let's", "we need to", "don't forget",
        "remember to", "make sure", "important:", "note:", "todo:"
    };
    
    for (size_t i = 0; i < sizeof(action_patterns) / sizeof(action_patterns[0]); i++) {
        const char* pos = strcasestr(text, action_patterns[i]);
        while (pos != NULL) {
            const char* start = pos;
            const char* end = strchr(start, '.');
            if (!end) end = strchr(start, '\n');
            if (!end) end = start + strlen(start);
            
            int len = end - start;
            if (len > 10 && len < 200) {
                patterns = realloc(patterns, (*count + 1) * sizeof(char*));
                patterns[*count] = malloc(len + 1);
                strncpy(patterns[*count], start, len);
                patterns[*count][len] = '\0';
                (*count)++;
            }
            
            pos = strcasestr(pos + 1, action_patterns[i]);
        }
    }
    
    return patterns;
}

void checklist_items_free(ChecklistItem* items, size_t count) {
    if (!items) return;
    
    for (size_t i = 0; i < count; i++) {
        free(items[i].id);
        free(items[i].content);
        free(items[i].priority);
        free(items[i].source);
    }
}

ChecklistItem* extract_checklist_items(const char* text, size_t* count) {
    *count = 0;
    ChecklistItem* items = NULL;
    
    const char* patterns[] = {
        "add to (?:checklist|list|todo)", "we need to", "should (?:have|do)",
        "don't forget to", "remember to", "make sure to", "need to", "have to", "must"
    };
    
    for (size_t i = 0; i < sizeof(patterns) / sizeof(patterns[0]); i++) {
        regex_t regex;
        char pattern[256];
        snprintf(pattern, sizeof(pattern), "(?i)%s", patterns[i]);
        
        if (regcomp(&regex, pattern, REG_EXTENDED | REG_ICASE) == 0) {
            regmatch_t matches[1];
            const char* search_text = text;
            
            while (regexec(&regex, search_text, 1, matches, 0) == 0) {
                int start = matches[0].rm_so;
                const char* text_after = search_text + start;
                const char* end = strchr(text_after, '.');
                if (!end) end = strchr(text_after, '!');
                if (!end) end = strchr(text_after, '?');
                if (!end) end = text_after + strlen(text_after);
                
                int len = end - text_after;
                if (len > 0 && len < 200) {
                    items = realloc(items, (*count + 1) * sizeof(ChecklistItem));
                    
                    uuid_t uuid;
                    char uuid_str[37];
                    uuid_generate(uuid);
                    uuid_unparse(uuid, uuid_str);
                    
                    items[*count].id = strdup(uuid_str);
                    items[*count].content = malloc(len + 1);
                    strncpy(items[*count].content, text_after, len);
                    items[*count].content[len] = '\0';
                    items[*count].priority = strdup("medium");
                    items[*count].created_at = time(NULL);
                    items[*count].completed = 0;
                    items[*count].source = strdup("natural_language");
                    (*count)++;
                }
                
                search_text = text_after + len;
            }
            regfree(&regex);
        }
    }
    
    return items;
}

char* clean_content(const char* original, char** extracted_tags, size_t tags_count) {
    char* cleaned = strdup(original);
    
    for (size_t i = 0; i < tags_count; i++) {
        char* pos = strstr(cleaned, extracted_tags[i]);
        if (pos) {
            size_t tag_len = strlen(extracted_tags[i]);
            memmove(pos, pos + tag_len, strlen(pos + tag_len) + 1);
        }
    }
    
    // Remove extra whitespace
    char* result = malloc(strlen(cleaned) + 1);
    char* dest = result;
    const char* src = cleaned;
    int space = 0;
    
    while (*src) {
        if (*src == ' ' || *src == '\n' || *src == '\t') {
            if (!space) {
                *dest++ = ' ';
                space = 1;
            }
        } else {
            *dest++ = *src;
            space = 0;
        }
        src++;
    }
    *dest = '\0';
    
    free(cleaned);
    return result;
}

char* infer_topic(const char* text) {
    char* lower_text = malloc(strlen(text) + 1);
    for (size_t i = 0; i < strlen(text); i++) {
        lower_text[i] = tolower(text[i]);
    }
    lower_text[strlen(text)] = '\0';
    
    if (strstr(lower_text, "bug") || strstr(lower_text, "error") || strstr(lower_text, "fix")) {
        free(lower_text);
        return strdup("Bug Fixing & Debugging");
    } else if (strstr(lower_text, "feature") || strstr(lower_text, "implement")) {
        free(lower_text);
        return strdup("Feature Development");
    } else if (strstr(lower_text, "design") || strstr(lower_text, "architecture")) {
        free(lower_text);
        return strdup("System Design & Architecture");
    } else if (strstr(lower_text, "test") || strstr(lower_text, "testing")) {
        free(lower_text);
        return strdup("Testing & Quality Assurance");
    } else if (strstr(lower_text, "deploy") || strstr(lower_text, "production")) {
        free(lower_text);
        return strdup("Deployment & Operations");
    } else {
        free(lower_text);
        return strdup("General Discussion");
    }
}

void ensure_session_exists(TodoziProcessorState* state, const char* session_id, ParsedContent* parsed) {
    if (!hashmap_get(state->active_sessions, session_id)) {
        char* topic = infer_topic(parsed->text_content);
        ConversationSession* session = malloc(sizeof(ConversationSession));
        session->id = strdup(session_id);
        session->start_time = time(NULL);
        session->last_activity = time(NULL);
        session->topic = topic;
        session->participant_count = 1;
        session->message_count = 1;
        hashmap_put(state->active_sessions, session_id, session);
    } else {
        ConversationSession* session = (ConversationSession*)hashmap_get(state->active_sessions, session_id);
        if (session) {
            session->last_activity = time(NULL);
            session->message_count++;
        }
    }
}

ProcessingResult* process_tool_calls(TdzContentProcessorTool* tool, json_object* tool_calls) {
    ProcessingResult* result = malloc(sizeof(ProcessingResult));
    result->actions = NULL;
    result->actions_count = 0;
    
    if (!tool_calls || !json_object_is_type(tool_calls, json_type_array)) {
        return result;
    }
    
    size_t array_len = json_object_array_length(tool_calls);
    for (size_t i = 0; i < array_len; i++) {
        json_object* tool_call = json_object_array_get_idx(tool_calls, i);
        json_object* function_obj = json_object_object_get(tool_call, "function");
        if (!function_obj) continue;
        
        json_object* name_obj = json_object_object_get(function_obj, "name");
        if (!name_obj || !json_object_is_type(name_obj, json_type_string)) continue;
        
        const char* function_name = json_object_get_string(name_obj);
        
        ProcessedAction* action = NULL;
        if (strstr(function_name, "create_task") || strstr(function_name, "add_task")) {
            action = process_create_task_call(function_obj);
        } else if (strstr(function_name, "search") || strstr(function_name, "list")) {
            action = process_search_call(function_obj);
        } else if (strstr(function_name, "update") || strstr(function_name, "complete")) {
            action = process_update_call(function_obj);
        } else if (strstr(function_name, "memory")) {
            action = process_memory_call(function_obj);
        } else if (strstr(function_name, "idea")) {
            action = process_idea_call(function_obj);
        } else {
            action = malloc(sizeof(ProcessedAction));
            uuid_t uuid;
            char uuid_str[37];
            uuid_generate(uuid);
            uuid_unparse(uuid, uuid_str);
            
            action->id = strdup(uuid_str);
            action->action_type = strdup("unknown_tool_call");
            char desc[256];
            snprintf(desc, sizeof(desc), "Unknown tool call: %s", function_name);
            action->description = strdup(desc);
            action->timestamp = time(NULL);
            action->success = 0;
            action->result = strdup("Tool call not recognized");
        }
        
        if (action) {
            result->actions = realloc(result->actions, (result->actions_count + 1) * sizeof(ProcessedAction));
            result->actions[result->actions_count++] = *action;
            free(action);
        }
    }
    
    return result;
}

ProcessedAction* process_create_task_call(json_object* tool_call) {
    if (!tool_call) return NULL;
    
    ProcessedAction* action = malloc(sizeof(ProcessedAction));
    if (!action) return NULL;
    
    uuid_t uuid;
    char uuid_str[37];
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    action->id = strdup(uuid_str);
    action->action_type = strdup("create_task");
    action->description = strdup("Created task via tool call");
    action->timestamp = time(NULL);
    action->success = 1;
    action->result = strdup("Task created");
    
    if (!action->id || !action->action_type || !action->description || !action->result) {
        processed_action_free(action);
        free(action);
        return NULL;
    }
    
    return action;
}

ProcessedAction* process_search_call(json_object* tool_call) {
    if (!tool_call) return NULL;
    
    ProcessedAction* action = malloc(sizeof(ProcessedAction));
    if (!action) return NULL;
    
    uuid_t uuid;
    char uuid_str[37];
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    action->id = strdup(uuid_str);
    action->action_type = strdup("search_tasks");
    action->description = strdup("Searched tasks via tool call");
    action->timestamp = time(NULL);
    action->success = 1;
    action->result = strdup("Tasks searched");
    
    if (!action->id || !action->action_type || !action->description || !action->result) {
        processed_action_free(action);
        free(action);
        return NULL;
    }
    
    return action;
}

ProcessedAction* process_update_call(json_object* tool_call) {
    if (!tool_call) return NULL;
    
    ProcessedAction* action = malloc(sizeof(ProcessedAction));
    if (!action) return NULL;
    
    uuid_t uuid;
    char uuid_str[37];
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    action->id = strdup(uuid_str);
    action->action_type = strdup("update_task");
    action->description = strdup("Updated task via tool call");
    action->timestamp = time(NULL);
    action->success = 1;
    action->result = strdup("Task update processed");
    
    if (!action->id || !action->action_type || !action->description || !action->result) {
        processed_action_free(action);
        free(action);
        return NULL;
    }
    
    return action;
}

ProcessedAction* process_memory_call(json_object* tool_call) {
    if (!tool_call) return NULL;
    
    ProcessedAction* action = malloc(sizeof(ProcessedAction));
    if (!action) return NULL;
    
    uuid_t uuid;
    char uuid_str[37];
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    action->id = strdup(uuid_str);
    action->action_type = strdup("create_memory");
    action->description = strdup("Created memory via tool call");
    action->timestamp = time(NULL);
    action->success = 1;
    action->result = strdup("Memory created");
    
    if (!action->id || !action->action_type || !action->description || !action->result) {
        processed_action_free(action);
        free(action);
        return NULL;
    }
    
    return action;
}

ProcessedAction* process_idea_call(json_object* tool_call) {
    if (!tool_call) return NULL;
    
    ProcessedAction* action = malloc(sizeof(ProcessedAction));
    if (!action) return NULL;
    
    uuid_t uuid;
    char uuid_str[37];
    uuid_generate(uuid);
    uuid_unparse(uuid, uuid_str);
    
    action->id = strdup(uuid_str);
    action->action_type = strdup("create_idea");
    action->description = strdup("Created idea via tool call");
    action->timestamp = time(NULL);
    action->success = 1;
    action->result = strdup("Idea created");
    
    if (!action->id || !action->action_type || !action->description || !action->result) {
        processed_action_free(action);
        free(action);
        return NULL;
    }
    
    return action;
}

char* generate_response(const char* cleaned_content, TodoziProcessorState* state, ProcessingResult* processing, ProcessingStats stats) {
    if (!cleaned_content) cleaned_content = "";
    if (!processing) return strdup(cleaned_content);
    
    size_t response_size = strlen(cleaned_content) + 4096;
    char* response = malloc(response_size);
    if (!response) return NULL;
    
    strncpy(response, cleaned_content, response_size - 1);
    response[response_size - 1] = '\0';
    size_t pos = strlen(response);
    
    if (processing->actions_count > 0 || stats.checklists_generated > 0) {
        const char* summary = "\n\n--- TDZ PROCESSING SUMMARY ---\n";
        size_t len = strlen(summary);
        if (pos + len < response_size) {
            strncpy(response + pos, summary, response_size - pos - 1);
            pos += len;
        }
        
        if (stats.checklists_generated > 0) {
            char buffer[128];
            int n = snprintf(buffer, sizeof(buffer), "📋 Generated %zu checklist items\n", stats.checklists_generated);
            if (pos + n < response_size) {
                strncpy(response + pos, buffer, response_size - pos - 1);
                pos += n;
            }
        }
        if (processing->actions_count > 0) {
            char buffer[128];
            int n = snprintf(buffer, sizeof(buffer), "✅ Processed %zu actions\n", processing->actions_count);
            if (pos + n < response_size) {
                strncpy(response + pos, buffer, response_size - pos - 1);
                pos += n;
            }
            
            size_t successful = 0;
            for (size_t i = 0; i < processing->actions_count; i++) {
                if (processing->actions[i].success) successful++;
            }
            if (successful > 0) {
                n = snprintf(buffer, sizeof(buffer), "✅ %zu successful actions\n", successful);
                if (pos + n < response_size) {
                    strncpy(response + pos, buffer, response_size - pos - 1);
                    pos += n;
                }
            }
        }
        char buffer[128];
        int n = snprintf(buffer, sizeof(buffer), "⏱️ Processing time: %ldms\n", stats.processing_time_ms);
        if (pos + n < response_size) {
            strncpy(response + pos, buffer, response_size - pos - 1);
            pos += n;
        }
    }
    
    // Add recent actions
    if (state && state->recent_actions_count > 0) {
        const char* header = "\n--- RECENT ACTIONS ---\n";
        size_t len = strlen(header);
        if (pos + len < response_size) {
            strncpy(response + pos, header, response_size - pos - 1);
            pos += len;
        }
        
        size_t start = state->recent_actions_count > 3 ? state->recent_actions_count - 3 : 0;
        for (size_t i = start; i < state->recent_actions_count; i++) {
            char buffer[512];
            const char* status = state->recent_actions[i].success ? "✅" : "❌";
            int n = snprintf(buffer, sizeof(buffer), "%s %s: %s\n", status, 
                     state->recent_actions[i].action_type ? state->recent_actions[i].action_type : "unknown", 
                     state->recent_actions[i].description ? state->recent_actions[i].description : "");
            if (pos + n < response_size) {
                strncpy(response + pos, buffer, response_size - pos - 1);
                pos += n;
            }
        }
    }
    
    // Add active checklist
    if (state && state->checklist_items) {
        size_t active_count = 0;
        for (size_t i = 0; i < state->checklist_items_count && active_count < 3; i++) {
            if (!state->checklist_items[i].completed) {
                if (active_count == 0) {
                    const char* header = "\n--- ACTIVE CHECKLIST ---\n";
                    size_t len = strlen(header);
                    if (pos + len < response_size) {
                        strncpy(response + pos, header, response_size - pos - 1);
                        pos += len;
                    }
                }
                char buffer[512];
                int n = snprintf(buffer, sizeof(buffer), "☐ %s\n", 
                        state->checklist_items[i].content ? state->checklist_items[i].content : "");
                if (pos + n < response_size) {
                    strncpy(response + pos, buffer, response_size - pos - 1);
                    pos += n;
                }
                active_count++;
            }
        }
    }
    
    const char* footer = "\n💡 Run `todozi stats` or `todozi list` to see all recent activity\n";
    size_t len = strlen(footer);
    if (pos + len < response_size) {
        strncpy(response + pos, footer, response_size - pos - 1);
        pos += len;
    }
    response[pos] = '\0';
    
    return response;
}

void processed_action_free(ProcessedAction* action) {
    if (!action) return;
    free(action->id);
    free(action->action_type);
    free(action->description);
    free(action->result);
}

char* execute_binary_command(const char* command, char* args[], size_t args_count, int* success) {
    if (!command || !success) {
        if (success) *success = 0;
        return NULL;
    }
    
    *success = 0;
    
    // Create pipe for capturing output
    int pipefd[2];
    if (pipe(pipefd) == -1) {
        return NULL;
    }
    
    pid_t pid = fork();
    if (pid == -1) {
        close(pipefd[0]);
        close(pipefd[1]);
        return NULL;
    }
    
    if (pid == 0) {
        // Child process
        close(pipefd[0]);
        dup2(pipefd[1], STDOUT_FILENO);
        dup2(pipefd[1], STDERR_FILENO);
        close(pipefd[1]);
        
        // Build argument array
        char** exec_args = malloc((args_count + 2) * sizeof(char*));
        if (!exec_args) {
            exit(1);
        }
        
        exec_args[0] = (char*)command;
        for (size_t i = 0; i < args_count; i++) {
            exec_args[i + 1] = args[i];
        }
        exec_args[args_count + 1] = NULL;
        
        execvp(command, exec_args);
        free(exec_args);
        exit(1);
    } else {
        // Parent process
        close(pipefd[1]);
        
        // Read output
        size_t buffer_size = 4096;
        size_t total_size = 0;
        char* output = malloc(buffer_size);
        if (!output) {
            close(pipefd[0]);
            waitpid(pid, NULL, 0);
            return NULL;
        }
        
        ssize_t bytes_read;
        while ((bytes_read = read(pipefd[0], output + total_size, buffer_size - total_size - 1)) > 0) {
            total_size += bytes_read;
            if (total_size >= buffer_size - 1) {
                buffer_size *= 2;
                char* new_output = realloc(output, buffer_size);
                if (!new_output) {
                    free(output);
                    close(pipefd[0]);
                    waitpid(pid, NULL, 0);
                    return NULL;
                }
                output = new_output;
            }
        }
        output[total_size] = '\0';
        
        close(pipefd[0]);
        
        int status;
        waitpid(pid, &status, 0);
        
        if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
            *success = 1;
        }
        
        return output;
    }
}

void processing_result_free(ProcessingResult* result) {
    if (!result) return;
    
    if (result->actions) {
        for (size_t i = 0; i < result->actions_count; i++) {
            processed_action_free(&result->actions[i]);
        }
        free(result->actions);
    }
    free(result);
}

char* tdz_cnt(const char* content, const char* session_id) {
    if (!content) return strdup("");
    if (!session_id) session_id = "default";
    
    // Parse content
    ParsedContent* parsed = parse_raw_content(content);
    if (!parsed) return strdup(content);
    
    // Extract data
    TodoziProcessorState* state = todozi_processor_state_new();
    if (!state) {
        parsed_content_free(parsed);
        return strdup(content);
    }
    
    TdzContentProcessorTool* tool = tdz_content_processor_tool_new(state);
    if (!tool) {
        todozi_processor_state_free(state);
        parsed_content_free(parsed);
        return strdup(content);
    }
    
    ExtractionResult* extraction = extract_todozi_data(tool, parsed);
    if (!extraction) {
        tdz_content_processor_tool_free(tool);
        todozi_processor_state_free(state);
        parsed_content_free(parsed);
        return strdup(content);
    }
    
    // Process tool calls
    ProcessingResult* processing = process_tool_calls(tool, extraction->tool_calls);
    if (!processing) {
        extraction_result_free(extraction);
        tdz_content_processor_tool_free(tool);
        todozi_processor_state_free(state);
        parsed_content_free(parsed);
        return strdup(content);
    }
    
    // Clean content
    char* cleaned = clean_content(content, extraction->extracted_tags, extraction->extracted_tags_count);
    if (!cleaned) {
        processing_result_free(processing);
        extraction_result_free(extraction);
        tdz_content_processor_tool_free(tool);
        todozi_processor_state_free(state);
        parsed_content_free(parsed);
        return strdup(content);
    }
    
    // Create response
    ProcessingStats stats = {0};
    stats.content_length = strlen(content);
    stats.tool_calls_found = extraction->tool_calls ? json_object_array_length(extraction->tool_calls) : 0;
    stats.tags_extracted = extraction->extracted_tags_count;
    
    char* response = generate_response(cleaned, tool->state, processing, stats);
    if (!response) {
        response = strdup(cleaned);
    }
    
    // Cleanup
    parsed_content_free(parsed);
    extraction_result_free(extraction);
    processing_result_free(processing);
    free(cleaned);
    tdz_content_processor_tool_free(tool);
    todozi_processor_state_free(state);
    
    return response;
}

TodoziProcessorState* initialize_tdz_content_processor(void) {
    return todozi_processor_state_new();
}

// Main function for testing
int main() {
    const char* test_content = "Hello world, <todozi>add task; test task</todozi>";
    char* result = tdz_cnt(test_content, NULL);
    printf("Result: %s\n", result);
    free(result);
    return 0;
}