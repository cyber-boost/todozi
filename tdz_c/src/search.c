#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>

// Forward declarations
typedef struct Task Task;
typedef struct Memory Memory;
typedef struct Idea Idea;
typedef struct Error Error;
typedef struct TrainingData TrainingData;
typedef struct Tag Tag;
typedef struct ChatContent ChatContent;

// Enums
typedef enum {
    TASKS,
    MEMORIES,
    IDEAS,
    ERRORS,
    TRAINING
} SearchDataType;

typedef enum {
    LOW,
    MEDIUM,
    HIGH
} Priority;

typedef enum {
    PENDING,
    IN_PROGRESS,
    COMPLETED
} Status;

typedef struct {
    bool filter_by_status;
    Status status;
    bool filter_by_priority;
    Priority priority;
    char* assignee;
    char* required_tag;
} TaskSearchCriteria;

typedef enum {
    LOW_IMPORTANCE,
    MEDIUM_IMPORTANCE,
    HIGH_IMPORTANCE
} MemoryImportance;

typedef enum {
    SHORT_TERM,
    LONG_TERM
} MemoryTerm;

typedef struct {
    bool filter_by_importance;
    MemoryImportance importance;
    bool filter_by_term;
    MemoryTerm term;
    char* required_tag;
} MemorySearchCriteria;

typedef struct {
    TaskSearchCriteria task_criteria;
    MemorySearchCriteria memory_criteria;
    // Other criteria would be defined here
} AdvancedSearchCriteria;

// Models
struct Task {
    char* action;
    Status status;
    Priority priority;
    char** tags;
    int tags_count;
    char* assignee;
    time_t created_at;
};

struct Memory {
    char* moment;
    char* meaning;
    char* reason;
    MemoryImportance importance;
    MemoryTerm term;
    char** tags;
    int tags_count;
    time_t created_at;
};

struct Idea {
    char* idea;
    char* context;
    char** tags;
    int tags_count;
    time_t created_at;
};

struct Error {
    char* title;
    char* description;
    char* source;
    char* context;
    char** tags;
    int tags_count;
    time_t created_at;
};

struct TrainingData {
    char* prompt;
    char* completion;
    char* source;
    char* context;
    char** tags;
    int tags_count;
    time_t created_at;
};

struct ChatContent {
    Task* tasks;
    int tasks_count;
    Memory* memories;
    int memories_count;
    Idea* ideas;
    int ideas_count;
    Error* errors;
    int errors_count;
    TrainingData* training_data;
    int training_data_count;
};

// Search structures
typedef struct {
    Task task;
    double score;
} TaskResult;

typedef struct {
    Memory memory;
    double score;
} MemoryResult;

typedef struct {
    Idea idea;
    double score;
} IdeaResult;

typedef struct {
    Error error;
    double score;
} ErrorResult;

typedef struct {
    TrainingData training_data;
    double score;
} TrainingResult;

typedef struct {
    TaskResult* task_results;
    int task_results_count;
    MemoryResult* memory_results;
    int memory_results_count;
    IdeaResult* idea_results;
    int idea_results_count;
    ErrorResult* error_results;
    int error_results_count;
    TrainingResult* training_results;
    int training_results_count;
} SearchResults;

typedef struct {
    SearchDataType* data_types;
    int data_types_count;
    time_t* since;
    time_t* until;
    int* limit;
} SearchOptions;

typedef struct {
    int total_indexed_items;
    int tasks_count;
    int memories_count;
    int ideas_count;
    int errors_count;
    int training_count;
} SearchAnalytics;

// Hash map implementation for suggestions
typedef struct {
    char* key;
    int value;
} KeyValuePair;

typedef struct {
    KeyValuePair* pairs;
    int count;
    int capacity;
} HashMap;

// SearchEngine structure
typedef struct {
    Task* tasks;
    int tasks_count;
    Memory* memories;
    int memories_count;
    Idea* ideas;
    int ideas_count;
    Error* errors;
    int errors_count;
    TrainingData* training_data;
    int training_data_count;
    Tag* tags;
    int tags_count;
} SearchEngine;

// Function prototypes
SearchEngine* search_engine_new();
void search_engine_free(SearchEngine* engine);
void search_engine_update_index(SearchEngine* engine, ChatContent* content);
SearchResults search_engine_search(SearchEngine* engine, const char* query, SearchOptions options);
bool matches_query(SearchEngine* engine, const char* query, const char* primary_text, const char* secondary_text, char** tags, int tags_count);
double calculate_relevance_score(SearchEngine* engine, const char* query, const char* text, char** tags, int tags_count);
SearchAnalytics search_engine_get_search_analytics(SearchEngine* engine);
char** search_engine_get_search_suggestions(SearchEngine* engine, const char* query, int limit, int* result_count);
void extract_keywords(SearchEngine* engine, const char* text, HashMap* keywords);
SearchResults search_engine_advanced_search(SearchEngine* engine, AdvancedSearchCriteria criteria);
bool matches_advanced_criteria(SearchEngine* engine, Task* task, TaskSearchCriteria* criteria);
bool matches_advanced_memory_criteria(SearchEngine* engine, Memory* memory, MemorySearchCriteria* criteria);

// Helper functions
static char* string_to_lowercase(const char* str);
bool string_contains(const char* haystack, const char* needle);
char** string_split(const char* str, int* count);
void string_split_free(char** words, int count);
void hashmap_init(HashMap* map);
static void hashmap_put(HashMap* map, const char* key, int value);
static int hashmap_get(HashMap* map, const char* key);
void hashmap_free(HashMap* map);
SearchResults search_results_new();
void search_results_free(SearchResults* results);
int search_results_total_results(SearchResults* results);
bool search_results_has_results(SearchResults* results);

// Implementation
SearchEngine* search_engine_new() {
    SearchEngine* engine = malloc(sizeof(SearchEngine));
    if (!engine) {
        return NULL;
    }
    engine->tasks = NULL;
    engine->tasks_count = 0;
    engine->memories = NULL;
    engine->memories_count = 0;
    engine->ideas = NULL;
    engine->ideas_count = 0;
    engine->errors = NULL;
    engine->errors_count = 0;
    engine->training_data = NULL;
    engine->training_data_count = 0;
    engine->tags = NULL;
    engine->tags_count = 0;
    return engine;
}

void search_engine_free(SearchEngine* engine) {
    if (!engine) {
        return;
    }
    // Note: This assumes the engine owns the data. If not, you need deep free logic.
    // For now, we only free the arrays, not the individual struct contents.
    free(engine->tasks);
    free(engine->memories);
    free(engine->ideas);
    free(engine->errors);
    free(engine->training_data);
    free(engine->tags);
    free(engine);
}

void search_engine_update_index(SearchEngine* engine, ChatContent* content) {
    if (!engine || !content) {
        return;
    }

    // Extend tasks
    if (content->tasks_count > 0 && content->tasks) {
        Task* new_tasks = realloc(engine->tasks, (engine->tasks_count + content->tasks_count) * sizeof(Task));
        if (new_tasks) {
            engine->tasks = new_tasks;
            for (int i = 0; i < content->tasks_count; i++) {
                engine->tasks[engine->tasks_count + i] = content->tasks[i];
            }
            engine->tasks_count += content->tasks_count;
        }
    }

    // Extend memories
    if (content->memories_count > 0 && content->memories) {
        Memory* new_memories = realloc(engine->memories, (engine->memories_count + content->memories_count) * sizeof(Memory));
        if (new_memories) {
            engine->memories = new_memories;
            for (int i = 0; i < content->memories_count; i++) {
                engine->memories[engine->memories_count + i] = content->memories[i];
            }
            engine->memories_count += content->memories_count;
        }
    }

    // Extend ideas
    if (content->ideas_count > 0 && content->ideas) {
        Idea* new_ideas = realloc(engine->ideas, (engine->ideas_count + content->ideas_count) * sizeof(Idea));
        if (new_ideas) {
            engine->ideas = new_ideas;
            for (int i = 0; i < content->ideas_count; i++) {
                engine->ideas[engine->ideas_count + i] = content->ideas[i];
            }
            engine->ideas_count += content->ideas_count;
        }
    }

    // Extend errors
    if (content->errors_count > 0 && content->errors) {
        Error* new_errors = realloc(engine->errors, (engine->errors_count + content->errors_count) * sizeof(Error));
        if (new_errors) {
            engine->errors = new_errors;
            for (int i = 0; i < content->errors_count; i++) {
                engine->errors[engine->errors_count + i] = content->errors[i];
            }
            engine->errors_count += content->errors_count;
        }
    }

    // Extend training_data
    if (content->training_data_count > 0 && content->training_data) {
        TrainingData* new_training_data = realloc(engine->training_data, (engine->training_data_count + content->training_data_count) * sizeof(TrainingData));
        if (new_training_data) {
            engine->training_data = new_training_data;
            for (int i = 0; i < content->training_data_count; i++) {
                engine->training_data[engine->training_data_count + i] = content->training_data[i];
            }
            engine->training_data_count += content->training_data_count;
        }
    }
}

SearchResults search_engine_search(SearchEngine* engine, const char* query, SearchOptions options) {
    (void)options; // Reserved for future use
    SearchResults results = search_results_new();
    if (!engine || !query) {
        return results;
    }
    char* query_lower = string_to_lowercase(query);
    if (!query_lower) {
        return results;
    }

    // Search tasks
    for (int i = 0; i < engine->tasks_count; i++) {
        Task* task = &engine->tasks[i];
        if (matches_query(engine, query_lower, task->action, NULL, task->tags, task->tags_count)) {
            double score = calculate_relevance_score(engine, query_lower, task->action, task->tags, task->tags_count);
            TaskResult* new_results = realloc(results.task_results, (results.task_results_count + 1) * sizeof(TaskResult));
            if (new_results) {
                results.task_results = new_results;
                results.task_results[results.task_results_count].task = *task;
                results.task_results[results.task_results_count].score = score;
                results.task_results_count++;
            }
        }
    }

    // Search memories
    for (int i = 0; i < engine->memories_count; i++) {
        Memory* memory = &engine->memories[i];
        if (matches_query(engine, query_lower, memory->moment, memory->meaning, memory->tags, memory->tags_count) ||
            matches_query(engine, query_lower, memory->reason, NULL, memory->tags, memory->tags_count)) {
            double score = calculate_relevance_score(engine, query_lower, memory->meaning, memory->tags, memory->tags_count);
            MemoryResult* new_results = realloc(results.memory_results, (results.memory_results_count + 1) * sizeof(MemoryResult));
            if (new_results) {
                results.memory_results = new_results;
                results.memory_results[results.memory_results_count].memory = *memory;
                results.memory_results[results.memory_results_count].score = score;
                results.memory_results_count++;
            }
        }
    }

    // Search ideas
    for (int i = 0; i < engine->ideas_count; i++) {
        Idea* idea = &engine->ideas[i];
        if (matches_query(engine, query_lower, idea->idea, idea->context, idea->tags, idea->tags_count)) {
            double score = calculate_relevance_score(engine, query_lower, idea->idea, idea->tags, idea->tags_count);
            IdeaResult* new_results = realloc(results.idea_results, (results.idea_results_count + 1) * sizeof(IdeaResult));
            if (new_results) {
                results.idea_results = new_results;
                results.idea_results[results.idea_results_count].idea = *idea;
                results.idea_results[results.idea_results_count].score = score;
                results.idea_results_count++;
            }
        }
    }

    // Search errors
    for (int i = 0; i < engine->errors_count; i++) {
        Error* error = &engine->errors[i];
        if (matches_query(engine, query_lower, error->title, error->description, error->tags, error->tags_count) ||
            matches_query(engine, query_lower, error->source, error->context, error->tags, error->tags_count)) {
            double score = calculate_relevance_score(engine, query_lower, error->title, error->tags, error->tags_count);
            ErrorResult* new_results = realloc(results.error_results, (results.error_results_count + 1) * sizeof(ErrorResult));
            if (new_results) {
                results.error_results = new_results;
                results.error_results[results.error_results_count].error = *error;
                results.error_results[results.error_results_count].score = score;
                results.error_results_count++;
            }
        }
    }

    // Search training data
    for (int i = 0; i < engine->training_data_count; i++) {
        TrainingData* training = &engine->training_data[i];
        if (matches_query(engine, query_lower, training->prompt, training->completion, training->tags, training->tags_count) ||
            matches_query(engine, query_lower, training->source, training->context, training->tags, training->tags_count)) {
            double score = calculate_relevance_score(engine, query_lower, training->prompt, training->tags, training->tags_count);
            TrainingResult* new_results = realloc(results.training_results, (results.training_results_count + 1) * sizeof(TrainingResult));
            if (new_results) {
                results.training_results = new_results;
                results.training_results[results.training_results_count].training_data = *training;
                results.training_results[results.training_results_count].score = score;
                results.training_results_count++;
            }
        }
    }

    free(query_lower);
    return results;
}

bool matches_query(SearchEngine* engine, const char* query, const char* primary_text, const char* secondary_text, char** tags, int tags_count) {
    (void)engine; // Reserved for future use
    if (!query || !primary_text) {
        return false;
    }

    char* primary_lower = string_to_lowercase(primary_text);
    if (!primary_lower) {
        return false;
    }
    if (string_contains(primary_lower, query)) {
        free(primary_lower);
        return true;
    }
    free(primary_lower);

    if (secondary_text) {
        char* secondary_lower = string_to_lowercase(secondary_text);
        if (secondary_lower) {
            if (string_contains(secondary_lower, query)) {
                free(secondary_lower);
                return true;
            }
            free(secondary_lower);
        }
    }

    if (tags) {
        for (int i = 0; i < tags_count; i++) {
            if (tags[i]) {
                char* tag_lower = string_to_lowercase(tags[i]);
                if (tag_lower) {
                    if (string_contains(tag_lower, query)) {
                        free(tag_lower);
                        return true;
                    }
                    free(tag_lower);
                }
            }
        }
    }

    return false;
}

double calculate_relevance_score(SearchEngine* engine, const char* query, const char* text, char** tags, int tags_count) {
    (void)engine; // Reserved for future use
    if (!query || !text) {
        return 0.0;
    }

    char* text_lower = string_to_lowercase(text);
    if (!text_lower) {
        return 0.0;
    }
    double score = 0.0;

    if (string_contains(text_lower, query)) {
        score += 1.0;
    }

    int words_count;
    char** words = string_split(query, &words_count);
    if (words) {
        for (int i = 0; i < words_count; i++) {
            if (words[i]) {
                char* word_with_spaces = malloc(strlen(words[i]) + 3);
                if (word_with_spaces) {
                    sprintf(word_with_spaces, " %s ", words[i]);
                    if (string_contains(text_lower, word_with_spaces)) {
                        score += 0.7;
                    }
                    free(word_with_spaces);
                }
            }
        }
        string_split_free(words, words_count);
    }

    if (tags) {
        for (int i = 0; i < tags_count; i++) {
            if (tags[i]) {
                char* tag_lower = string_to_lowercase(tags[i]);
                if (tag_lower) {
                    if (string_contains(tag_lower, query)) {
                        score += 0.5;
                    }
                    free(tag_lower);
                }
            }
        }
    }

    double length_penalty = 1.0 / ((strlen(text) / 100.0) > 1.0 ? (strlen(text) / 100.0) : 1.0);
    free(text_lower);
    return score * length_penalty;
}

SearchAnalytics search_engine_get_search_analytics(SearchEngine* engine) {
    SearchAnalytics analytics;
    if (!engine) {
        analytics.total_indexed_items = 0;
        analytics.tasks_count = 0;
        analytics.memories_count = 0;
        analytics.ideas_count = 0;
        analytics.errors_count = 0;
        analytics.training_count = 0;
        return analytics;
    }
    analytics.total_indexed_items = engine->tasks_count + engine->memories_count + engine->ideas_count + engine->errors_count + engine->training_data_count;
    analytics.tasks_count = engine->tasks_count;
    analytics.memories_count = engine->memories_count;
    analytics.ideas_count = engine->ideas_count;
    analytics.errors_count = engine->errors_count;
    analytics.training_count = engine->training_data_count;
    return analytics;
}

char** search_engine_get_search_suggestions(SearchEngine* engine, const char* query, int limit, int* result_count) {
    if (!engine || !query || !result_count || limit <= 0) {
        if (result_count) {
            *result_count = 0;
        }
        return NULL;
    }

    HashMap suggestions;
    hashmap_init(&suggestions);

    // Extract keywords from tasks
    for (int i = 0; i < engine->tasks_count; i++) {
        Task* task = &engine->tasks[i];
        if (task && task->action) {
            extract_keywords(engine, task->action, &suggestions);
        }
        if (task && task->tags) {
            for (int j = 0; j < task->tags_count; j++) {
                if (task->tags[j]) {
                    hashmap_put(&suggestions, task->tags[j], hashmap_get(&suggestions, task->tags[j]) + 1);
                }
            }
        }
    }

    // Extract keywords from memories
    for (int i = 0; i < engine->memories_count; i++) {
        Memory* memory = &engine->memories[i];
        if (memory && memory->meaning) {
            extract_keywords(engine, memory->meaning, &suggestions);
        }
        if (memory && memory->tags) {
            for (int j = 0; j < memory->tags_count; j++) {
                if (memory->tags[j]) {
                    hashmap_put(&suggestions, memory->tags[j], hashmap_get(&suggestions, memory->tags[j]) + 1);
                }
            }
        }
    }

    char* query_lower = string_to_lowercase(query);
    if (!query_lower) {
        hashmap_free(&suggestions);
        *result_count = 0;
        return NULL;
    }

    char** filtered = malloc(suggestions.count * sizeof(char*));
    if (!filtered) {
        free(query_lower);
        hashmap_free(&suggestions);
        *result_count = 0;
        return NULL;
    }
    int filtered_count = 0;

    for (int i = 0; i < suggestions.count; i++) {
        if (suggestions.pairs[i].key) {
            char* key_lower = string_to_lowercase(suggestions.pairs[i].key);
            if (key_lower) {
                if (string_contains(key_lower, query_lower)) {
                    filtered[filtered_count] = malloc(strlen(suggestions.pairs[i].key) + 1);
                    if (filtered[filtered_count]) {
                        strcpy(filtered[filtered_count], suggestions.pairs[i].key);
                        filtered_count++;
                    }
                }
                free(key_lower);
            }
        }
    }

    // Sort by frequency (simplified)
    for (int i = 0; i < filtered_count - 1; i++) {
        for (int j = 0; j < filtered_count - i - 1; j++) {
            // This is a simplified sort - in practice you'd sort by frequency
            if (filtered[j] && filtered[j + 1] && strcmp(filtered[j], filtered[j + 1]) > 0) {
                char* temp = filtered[j];
                filtered[j] = filtered[j + 1];
                filtered[j + 1] = temp;
            }
        }
    }

    int final_count = (filtered_count < limit) ? filtered_count : limit;
    char** result = malloc(final_count * sizeof(char*));
    if (!result) {
        // Free all filtered entries
        for (int i = 0; i < filtered_count; i++) {
            free(filtered[i]);
        }
        free(filtered);
        free(query_lower);
        hashmap_free(&suggestions);
        *result_count = 0;
        return NULL;
    }
    for (int i = 0; i < final_count; i++) {
        result[i] = filtered[i];
    }
    // Free remaining filtered entries that weren't copied
    for (int i = final_count; i < filtered_count; i++) {
        free(filtered[i]);
    }

    *result_count = final_count;
    free(query_lower);
    hashmap_free(&suggestions);
    free(filtered);
    return result;
}

void extract_keywords(SearchEngine* engine, const char* text, HashMap* keywords) {
    (void)engine; // Reserved for future use
    if (!text || !keywords) {
        return;
    }

    int words_count;
    char** words = string_split(text, &words_count);
    if (!words) {
        return;
    }

    for (int i = 0; i < words_count; i++) {
        if (words[i] && strlen(words[i]) > 3) {
            char* clean_word = malloc(strlen(words[i]) + 1);
            if (clean_word) {
                int clean_index = 0;
                for (int j = 0; words[i][j] != '\0'; j++) {
                    if ((words[i][j] >= 'a' && words[i][j] <= 'z') || (words[i][j] >= 'A' && words[i][j] <= 'Z') || (words[i][j] >= '0' && words[i][j] <= '9')) {
                        clean_word[clean_index++] = (words[i][j] >= 'A' && words[i][j] <= 'Z') ? words[i][j] + 32 : words[i][j];
                    }
                }
                clean_word[clean_index] = '\0';

                if (strlen(clean_word) > 3) {
                    hashmap_put(keywords, clean_word, hashmap_get(keywords, clean_word) + 1);
                }
                free(clean_word);
            }
        }
    }
    string_split_free(words, words_count);
}

SearchResults search_engine_advanced_search(SearchEngine* engine, AdvancedSearchCriteria criteria) {
    SearchResults results = search_results_new();
    if (!engine) {
        return results;
    }

    // Search tasks with advanced criteria
    for (int i = 0; i < engine->tasks_count; i++) {
        Task* task = &engine->tasks[i];
        if (matches_advanced_criteria(engine, task, &criteria.task_criteria)) {
            TaskResult* new_results = realloc(results.task_results, (results.task_results_count + 1) * sizeof(TaskResult));
            if (new_results) {
                results.task_results = new_results;
                results.task_results[results.task_results_count].task = *task;
                results.task_results[results.task_results_count].score = 1.0;
                results.task_results_count++;
            }
        }
    }

    // Search memories with advanced criteria
    for (int i = 0; i < engine->memories_count; i++) {
        Memory* memory = &engine->memories[i];
        if (matches_advanced_memory_criteria(engine, memory, &criteria.memory_criteria)) {
            MemoryResult* new_results = realloc(results.memory_results, (results.memory_results_count + 1) * sizeof(MemoryResult));
            if (new_results) {
                results.memory_results = new_results;
                results.memory_results[results.memory_results_count].memory = *memory;
                results.memory_results[results.memory_results_count].score = 1.0;
                results.memory_results_count++;
            }
        }
    }

    return results;
}

bool matches_advanced_criteria(SearchEngine* engine, Task* task, TaskSearchCriteria* criteria) {
    (void)engine; // Reserved for future use
    if (!task || !criteria) {
        return false;
    }

    // Check status if filtering is enabled
    if (criteria->filter_by_status && task->status != criteria->status) {
        return false;
    }
    
    // Check priority if filtering is enabled
    if (criteria->filter_by_priority && task->priority != criteria->priority) {
        return false;
    }
    
    // Check assignee
    if (criteria->assignee) {
        if (!task->assignee || strcmp(task->assignee, criteria->assignee) != 0) {
            return false;
        }
    }
    
    // Check required tag
    if (criteria->required_tag) {
        if (!task->tags) {
            return false;
        }
        bool found = false;
        for (int i = 0; i < task->tags_count; i++) {
            if (task->tags[i] && strcmp(task->tags[i], criteria->required_tag) == 0) {
                found = true;
                break;
            }
        }
        if (!found) {
            return false;
        }
    }
    
    return true;
}

bool matches_advanced_memory_criteria(SearchEngine* engine, Memory* memory, MemorySearchCriteria* criteria) {
    (void)engine; // Reserved for future use
    if (!memory || !criteria) {
        return false;
    }

    // Check importance if filtering is enabled
    if (criteria->filter_by_importance && memory->importance != criteria->importance) {
        return false;
    }
    
    // Check term if filtering is enabled
    if (criteria->filter_by_term && memory->term != criteria->term) {
        return false;
    }
    
    // Check required tag
    if (criteria->required_tag) {
        if (!memory->tags) {
            return false;
        }
        bool found = false;
        for (int i = 0; i < memory->tags_count; i++) {
            if (memory->tags[i] && strcmp(memory->tags[i], criteria->required_tag) == 0) {
                found = true;
                break;
            }
        }
        if (!found) {
            return false;
        }
    }
    
    return true;
}

// Helper function implementations
static char* string_to_lowercase(const char* str) {
    if (!str) {
        return NULL;
    }
    size_t len = strlen(str);
    char* result = malloc(len + 1);
    if (!result) {
        return NULL;
    }
    for (size_t i = 0; i < len; i++) {
        result[i] = (str[i] >= 'A' && str[i] <= 'Z') ? str[i] + 32 : str[i];
    }
    result[len] = '\0';
    return result;
}

bool string_contains(const char* haystack, const char* needle) {
    if (!haystack || !needle) {
        return false;
    }
    return strstr(haystack, needle) != NULL;
}

char** string_split(const char* str, int* count) {
    if (!str || !count) {
        if (count) {
            *count = 0;
        }
        return NULL;
    }

    size_t len = strlen(str);
    char* str_copy = malloc(len + 1);
    if (!str_copy) {
        *count = 0;
        return NULL;
    }
    strcpy(str_copy, str);

    // Start with reasonable capacity, grow as needed
    int capacity = 100;
    char** result = malloc(sizeof(char*) * capacity);
    if (!result) {
        free(str_copy);
        *count = 0;
        return NULL;
    }
    *count = 0;

    char* token = strtok(str_copy, " \t\n");
    while (token != NULL) {
        if (*count >= capacity) {
            capacity *= 2;
            char** new_result = realloc(result, sizeof(char*) * capacity);
            if (!new_result) {
                // Free what we've allocated so far
                for (int i = 0; i < *count; i++) {
                    free(result[i]);
                }
                free(result);
                free(str_copy);
                *count = 0;
                return NULL;
            }
            result = new_result;
        }
        result[*count] = malloc(strlen(token) + 1);
        if (!result[*count]) {
            // Free what we've allocated so far
            for (int i = 0; i < *count; i++) {
                free(result[i]);
            }
            free(result);
            free(str_copy);
            *count = 0;
            return NULL;
        }
        strcpy(result[*count], token);
        (*count)++;
        token = strtok(NULL, " \t\n");
    }

    free(str_copy);
    return result;
}

void string_split_free(char** words, int count) {
    if (!words) {
        return;
    }
    for (int i = 0; i < count; i++) {
        free(words[i]);
    }
    free(words);
}

void hashmap_init(HashMap* map) {
    if (!map) {
        return;
    }
    map->pairs = malloc(sizeof(KeyValuePair) * 100);
    if (!map->pairs) {
        map->count = 0;
        map->capacity = 0;
        return;
    }
    map->count = 0;
    map->capacity = 100;
}

static void hashmap_put(HashMap* map, const char* key, int value) {
    if (!map || !key) {
        return;
    }

    for (int i = 0; i < map->count; i++) {
        if (map->pairs[i].key && strcmp(map->pairs[i].key, key) == 0) {
            map->pairs[i].value = value;
            return;
        }
    }

    if (map->count >= map->capacity) {
        map->capacity *= 2;
        KeyValuePair* new_pairs = realloc(map->pairs, sizeof(KeyValuePair) * map->capacity);
        if (!new_pairs) {
            return; // Out of memory
        }
        map->pairs = new_pairs;
    }

    map->pairs[map->count].key = malloc(strlen(key) + 1);
    if (!map->pairs[map->count].key) {
        return; // Out of memory
    }
    strcpy(map->pairs[map->count].key, key);
    map->pairs[map->count].value = value;
    map->count++;
}

static int hashmap_get(HashMap* map, const char* key) {
    if (!map || !key) {
        return 0;
    }
    for (int i = 0; i < map->count; i++) {
        if (map->pairs[i].key && strcmp(map->pairs[i].key, key) == 0) {
            return map->pairs[i].value;
        }
    }
    return 0;
}

void hashmap_free(HashMap* map) {
    if (!map) {
        return;
    }
    if (map->pairs) {
        for (int i = 0; i < map->count; i++) {
            free(map->pairs[i].key);
        }
        free(map->pairs);
    }
    map->count = 0;
    map->capacity = 0;
}

SearchResults search_results_new() {
    SearchResults results;
    results.task_results = NULL;
    results.task_results_count = 0;
    results.memory_results = NULL;
    results.memory_results_count = 0;
    results.idea_results = NULL;
    results.idea_results_count = 0;
    results.error_results = NULL;
    results.error_results_count = 0;
    results.training_results = NULL;
    results.training_results_count = 0;
    return results;
}

void search_results_free(SearchResults* results) {
    if (!results) {
        return;
    }
    // Note: This only frees the arrays, not the individual struct contents
    // If the structs contain dynamically allocated strings, those need separate cleanup
    free(results->task_results);
    free(results->memory_results);
    free(results->idea_results);
    free(results->error_results);
    free(results->training_results);
    results->task_results = NULL;
    results->task_results_count = 0;
    results->memory_results = NULL;
    results->memory_results_count = 0;
    results->idea_results = NULL;
    results->idea_results_count = 0;
    results->error_results = NULL;
    results->error_results_count = 0;
    results->training_results = NULL;
    results->training_results_count = 0;
}

int search_results_total_results(SearchResults* results) {
    if (!results) {
        return 0;
    }
    return results->task_results_count + results->memory_results_count + results->idea_results_count + results->error_results_count + results->training_results_count;
}

bool search_results_has_results(SearchResults* results) {
    return search_results_total_results(results) > 0;
}