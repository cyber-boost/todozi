#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>  // For strcasecmp
#include <regex.h>
#include <time.h>
#include <uuid/uuid.h>


// Error codes
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_VALIDATION_ERROR,
    TODOZI_STORAGE_ERROR,
    TODOZI_PARSE_ERROR
} TodoziError;

// Priority levels
typedef enum {
    PRIORITY_LOW,
    PRIORITY_MEDIUM,
    PRIORITY_HIGH,
    PRIORITY_CRITICAL
} Priority;

// Status levels
typedef enum {
    STATUS_TODO,
    STATUS_IN_PROGRESS,
    STATUS_DONE,
    STATUS_BLOCKED,
    STATUS_DEFERRED
} Status;

// Assignee types
typedef enum {
    ASSIGNEE_AI,
    ASSIGNEE_HUMAN,
    ASSIGNEE_COLLABORATIVE,
    ASSIGNEE_AGENT
} AssigneeType;

// Memory importance
typedef enum {
    MEMORY_LOW,
    MEMORY_MEDIUM,
    MEMORY_HIGH
} MemoryImportance;

// Memory term
typedef enum {
    MEMORY_SHORT,
    MEMORY_LONG
} MemoryTerm;

// Memory type
typedef enum {
    MEMORY_STANDARD,
    MEMORY_SECRET,
    MEMORY_HUMAN,
    MEMORY_EMOTIONAL
} MemoryType;

// Share level
typedef enum {
    SHARE_PRIVATE,
    SHARE_TEAM,
    SHARE_PUBLIC
} ShareLevel;

// Idea importance
typedef enum {
    IDEA_LOW,
    IDEA_MEDIUM,
    IDEA_HIGH
} IdeaImportance;

// Error severity
typedef enum {
    ERROR_LOW,
    ERROR_MEDIUM,
    ERROR_HIGH,
    ERROR_CRITICAL
} ErrorSeverity;

// Error category
typedef enum {
    ERROR_GENERAL,
    ERROR_NETWORK,
    ERROR_DATABASE,
    ERROR_AUTHENTICATION
} ErrorCategory;

// Training data type
typedef enum {
    TRAINING_INSTRUCTION,
    TRAINING_EXAMPLE,
    TRAINING_QA
} TrainingDataType;

// Item status
typedef enum {
    ITEM_ACTIVE,
    ITEM_INACTIVE,
    ITEM_ARCHIVED
} ItemStatus;

// Assignment status
typedef enum {
    ASSIGNMENT_ASSIGNED,
    ASSIGNMENT_COMPLETED,
    ASSIGNMENT_CANCELLED
} AssignmentStatus;

// Forward declarations for structs (incomplete types)
typedef struct Task Task;
typedef struct Memory Memory;
typedef struct Idea Idea;
typedef struct AgentAssignment AgentAssignment;
typedef struct Error Error;
typedef struct TrainingData TrainingData;
typedef struct ChatContent ChatContent;
typedef struct Feeling Feeling;

// Forward declarations for free functions
void free_string_array(char** array, size_t count);
void free_task(Task* task);  // Defined in lib.c - frees the struct itself
void free_memory(Memory* memory);
void free_idea(Idea* idea);
void free_agent_assignment(AgentAssignment* assignment);
void free_error(Error* error);
void free_training_data(TrainingData* data);
static void free_chat_content(ChatContent* content);
void free_feeling(Feeling* feeling);

// Forward declaration for helper function
static void free_task_contents(Task* task);

// Task structure
typedef struct Task {
    char* id;
    char* user_id;
    char* action;
    char* time;
    Priority priority;
    char* parent_project;
    Status status;
    AssigneeType assignee_type;
    char* assignee_name;  // For agent assignments
    char** tags;
    size_t tags_count;
    char** dependencies;
    size_t dependencies_count;
    char* context_notes;
    int progress;
    int has_progress;
    double* embedding_vector;
    size_t embedding_size;
    time_t created_at;
    time_t updated_at;
} Task;

// Memory structure
typedef struct Memory {
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
    size_t tags_count;
    time_t created_at;
    time_t updated_at;
    char* emotion;  // For emotional memories
} Memory;

// Idea structure
typedef struct Idea {
    char* id;
    char* idea;
    char* project_id;
    ItemStatus status;
    ShareLevel share;
    IdeaImportance importance;
    char** tags;
    size_t tags_count;
    char* context;
    time_t created_at;
    time_t updated_at;
} Idea;

// Agent assignment structure
typedef struct AgentAssignment {
    char* agent_id;
    char* task_id;
    char* project_id;
    time_t assigned_at;
    AssignmentStatus status;
} AgentAssignment;

// Error structure
typedef struct Error {
    char* id;
    char* title;
    char* description;
    ErrorSeverity severity;
    ErrorCategory category;
    char* source;
    char* context;
    char** tags;
    size_t tags_count;
    int resolved;
    char* resolution;
    time_t created_at;
    time_t updated_at;
    time_t resolved_at;
} Error;

// Training data structure
typedef struct TrainingData {
    char* id;
    TrainingDataType data_type;
    char* prompt;
    char* completion;
    char* context;
    char** tags;
    size_t tags_count;
    double quality_score;
    int has_quality_score;
    char* source;
    time_t created_at;
    time_t updated_at;
} TrainingData;

// Feeling structure
typedef struct Feeling {
    char* id;
    char* emotion;
    int intensity;
    char* description;
    char* context;
    char** tags;
    size_t tags_count;
    time_t created_at;
    time_t updated_at;
} Feeling;

// Code chunk structure
typedef struct CodeChunk {
    char* id;
    char* name;
    char* language;
    char* description;
    char* code;
    char* purpose;
    time_t created_at;
    time_t updated_at;
} CodeChunk;

// Summary structure
typedef struct Summary {
    char* id;
    char* content;
    int length;
    char** tags;
    size_t tags_count;
    time_t created_at;
    time_t updated_at;
} Summary;

// Reminder structure
typedef struct Reminder {
    char* id;
    char* content;
    time_t remind_at;
    int priority;
    char* status;
    char* context;
    time_t created_at;
    time_t updated_at;
} Reminder;

// Chat content structure
typedef struct ChatContent {
    Task* tasks;
    size_t tasks_count;
    Memory* memories;
    size_t memories_count;
    Idea* ideas;
    size_t ideas_count;
    AgentAssignment* agent_assignments;
    size_t agent_assignments_count;
    CodeChunk* code_chunks;
    size_t code_chunks_count;
    Error* errors;
    size_t errors_count;
    TrainingData* training_data;
    size_t training_data_count;
    Feeling* feelings;
    size_t feelings_count;
    Summary* summaries;
    size_t summaries_count;
    Reminder* reminders;
    size_t reminders_count;
} ChatContent;

// Queue item structure
typedef struct QueueItem {
    char* id;
    char* title;
    char* description;
    Priority priority;
    char* project_id;
    time_t created_at;
} QueueItem;

// Helper function to create a copy of a string
char* string_copy(const char* source) {
    if (source == NULL) return NULL;
    size_t len = strlen(source);
    char* copy = malloc(len + 1);
    if (copy == NULL) return NULL;
    memcpy(copy, source, len + 1);
    return copy;
}

// Helper function to split string by delimiter
char** split_string(const char* str, const char* delimiter, size_t* count) {
    char* str_copy = string_copy(str);
    if (str_copy == NULL) {
        *count = 0;
        return NULL;
    }
    
    char** result = NULL;
    *count = 0;
    
    char* saveptr = NULL;
    char* token = strtok_r(str_copy, delimiter, &saveptr);
    while (token != NULL) {
        char** temp = realloc(result, (*count + 1) * sizeof(char*));
        if (temp == NULL) {
            free_string_array(result, *count);
            free(str_copy);
            *count = 0;
            return NULL;
        }
        result = temp;
        result[*count] = string_copy(token);
        if (result[*count] == NULL) {
            free_string_array(result, *count);
            free(str_copy);
            *count = 0;
            return NULL;
        }
        (*count)++;
        token = strtok_r(NULL, delimiter, &saveptr);
    }
    
    free(str_copy);
    return result;
}

// Helper function to trim whitespace
char* trim_whitespace(char* str) {
    if (str == NULL) return NULL;
    
    // Trim leading space
    while(*str == ' ' || *str == '\t') str++;
    
    if(*str == 0)  // All spaces?
        return str;
    
    // Trim trailing space
    char* end = str + strlen(str) - 1;
    while(end > str && (*end == ' ' || *end == '\t')) end--;
    
    // Write new null terminator
    *(end+1) = 0;
    
    return str;
}

// Helper function to free string array
void free_string_array(char** array, size_t count) {
    if (array == NULL) return;
    for (size_t i = 0; i < count; i++) {
        free(array[i]);
    }
    free(array);
}

// Helper to free Task contents (for stack-allocated Tasks)
// Must be defined after Task struct definition
static void free_task_contents(Task* task) {
    if (task == NULL) return;
    free(task->id);
    free(task->user_id);
    free(task->action);
    free(task->time);
    free(task->parent_project);
    free(task->assignee_name);
    free_string_array(task->tags, task->tags_count);
    free_string_array(task->dependencies, task->dependencies_count);
    free(task->context_notes);
    free(task->embedding_vector);
    memset(task, 0, sizeof(Task));
}

// Transform shorthand tags to full tags
char* transform_shorthand_tags(const char* message) {
    char* transformed = string_copy(message);
    if (transformed == NULL) return NULL;
    
    // Define mappings
    const char* mappings[][2] = {
        {"<tz>", "<todozi>"},
        {"</tz>", "</todozi>"},
        {"<mm>", "<memory>"},
        {"</mm>", "</memory>"},
        {"<id>", "<idea>"},
        {"</id>", "</idea>"},
        {"<ch>", "<chunk>"},
        {"</ch>", "</chunk>"},
        {"<fe>", "<feel>"},
        {"</fe>", "</feel>"},
        {"<tn>", "<train>"},
        {"</tn>", "</train>"},
        {"<er>", "<error>"},
        {"</er>", "</error>"},
        {"<sm>", "<summary>"},
        {"</sm>", "</summary>"},
        {"<rd>", "<reminder>"},
        {"</rd>", "</reminder>"},
        {"<tdz>", "<tdz>"},
        {"</tdz>", "</tdz>"}
    };
    
    size_t mappings_count = sizeof(mappings) / sizeof(mappings[0]);
    
    for (size_t i = 0; i < mappings_count; i++) {
        char* temp = transformed;
        size_t old_len = strlen(temp);
        const char* from = mappings[i][0];
        const char* to = mappings[i][1];
        size_t from_len = strlen(from);
        size_t to_len = strlen(to);
        
        // Count occurrences
        size_t occurrences = 0;
        char* pos = temp;
        while ((pos = strstr(pos, from)) != NULL) {
            occurrences++;
            pos += from_len;
        }
        
        if (occurrences == 0) {
            continue;
        }
        
        // Calculate new length
        size_t new_len = old_len + occurrences * (to_len - from_len);
        char* new_str = malloc(new_len + 1);
        if (new_str == NULL) {
            free(temp);
            return NULL;
        }
        
        // Perform replacements
        char* src = temp;
        char* dst = new_str;
        while (*src) {
            if (strncmp(src, from, from_len) == 0) {
                memcpy(dst, to, to_len);
                dst += to_len;
                src += from_len;
            } else {
                *dst++ = *src++;
            }
        }
        *dst = '\0';
        
        transformed = new_str;
        free(temp);
    }
    
    return transformed;
}

// Parse priority from string (case-insensitive)
Priority parse_priority(const char* str) {
    if (strcasecmp(str, "low") == 0) return PRIORITY_LOW;
    if (strcasecmp(str, "medium") == 0) return PRIORITY_MEDIUM;
    if (strcasecmp(str, "high") == 0) return PRIORITY_HIGH;
    if (strcasecmp(str, "critical") == 0) return PRIORITY_CRITICAL;
    return PRIORITY_MEDIUM; // default
}

// Parse status from string (case-insensitive)
Status parse_status(const char* str) {
    if (strcasecmp(str, "todo") == 0) return STATUS_TODO;
    if (strcasecmp(str, "in_progress") == 0) return STATUS_IN_PROGRESS;
    if (strcasecmp(str, "done") == 0) return STATUS_DONE;
    if (strcasecmp(str, "blocked") == 0) return STATUS_BLOCKED;
    if (strcasecmp(str, "deferred") == 0) return STATUS_DEFERRED;
    return STATUS_TODO; // default
}

// Parse assignee from string
AssigneeType parse_assignee(const char* str, char** assignee_name) {
    char* trimmed = string_copy(str);
    if (trimmed == NULL) return ASSIGNEE_HUMAN;
    trim_whitespace(trimmed);
    
    if (strncasecmp(trimmed, "assignee=", 9) == 0) {
        const char* assignee_part = trimmed + 9;
        if (strcasecmp(assignee_part, "ai") == 0) {
            free(trimmed);
            return ASSIGNEE_AI;
        }
        if (strcasecmp(assignee_part, "human") == 0) {
            free(trimmed);
            return ASSIGNEE_HUMAN;
        }
        if (strcasecmp(assignee_part, "collaborative") == 0) {
            free(trimmed);
            return ASSIGNEE_COLLABORATIVE;
        }
        if (strncasecmp(assignee_part, "agent=", 6) == 0) {
            *assignee_name = string_copy(assignee_part + 6);
            free(trimmed);
            return ASSIGNEE_AGENT;
        }
    } else {
        // Handle bare keywords
        if (strcasecmp(trimmed, "ai") == 0) {
            free(trimmed);
            return ASSIGNEE_AI;
        }
        if (strcasecmp(trimmed, "human") == 0) {
            free(trimmed);
            return ASSIGNEE_HUMAN;
        }
        if (strcasecmp(trimmed, "collaborative") == 0) {
            free(trimmed);
            return ASSIGNEE_COLLABORATIVE;
        }
    }
    
    free(trimmed);
    return ASSIGNEE_HUMAN; // default
}

// Parse memory importance (case-insensitive)
MemoryImportance parse_memory_importance(const char* str) {
    if (strcasecmp(str, "low") == 0) return MEMORY_LOW;
    if (strcasecmp(str, "medium") == 0) return MEMORY_MEDIUM;
    if (strcasecmp(str, "high") == 0) return MEMORY_HIGH;
    return MEMORY_MEDIUM; // default
}

// Parse memory term (case-insensitive)
MemoryTerm parse_memory_term(const char* str) {
    if (strcasecmp(str, "short") == 0) return MEMORY_SHORT;
    if (strcasecmp(str, "long") == 0) return MEMORY_LONG;
    return MEMORY_SHORT; // default
}

// Parse memory type
MemoryType parse_memory_type(const char* str) {
    const char* emotions[] = {
        "happy", "sad", "angry", "fearful", "surprised", "disgusted", "excited",
        "anxious", "confident", "frustrated", "motivated", "overwhelmed", "curious",
        "satisfied", "disappointed", "grateful", "proud", "ashamed", "hopeful",
        "resigned"
    };
    
    for (int i = 0; i < 20; i++) {
        if (strcasecmp(str, emotions[i]) == 0) {
            return MEMORY_EMOTIONAL;
        }
    }
    
    if (strcasecmp(str, "standard") == 0) return MEMORY_STANDARD;
    if (strcasecmp(str, "secret") == 0) return MEMORY_SECRET;
    if (strcasecmp(str, "human") == 0) return MEMORY_HUMAN;
    // Note: "short" and "long" are MemoryTerm values, not MemoryType
    // They should be handled by parse_memory_term(), not here
    
    return MEMORY_STANDARD; // default
}

// Parse share level (case-insensitive)
ShareLevel parse_share_level(const char* str) {
    if (strcasecmp(str, "share") == 0) return SHARE_PUBLIC;
    if (strcasecmp(str, "dont share") == 0 || 
        strcasecmp(str, "don't share") == 0 || 
        strcasecmp(str, "private") == 0) return SHARE_PRIVATE;
    if (strcasecmp(str, "team") == 0) return SHARE_TEAM;
    return SHARE_PRIVATE; // default
}

// Parse idea importance (case-insensitive)
IdeaImportance parse_idea_importance(const char* str) {
    if (strcasecmp(str, "low") == 0) return IDEA_LOW;
    if (strcasecmp(str, "medium") == 0) return IDEA_MEDIUM;
    if (strcasecmp(str, "high") == 0) return IDEA_HIGH;
    return IDEA_MEDIUM; // default
}

// Parse error severity (case-insensitive)
ErrorSeverity parse_error_severity(const char* str) {
    if (strcasecmp(str, "low") == 0) return ERROR_LOW;
    if (strcasecmp(str, "medium") == 0) return ERROR_MEDIUM;
    if (strcasecmp(str, "high") == 0) return ERROR_HIGH;
    if (strcasecmp(str, "critical") == 0) return ERROR_CRITICAL;
    return ERROR_MEDIUM; // default
}

// Parse error category (case-insensitive)
ErrorCategory parse_error_category(const char* str) {
    if (strcasecmp(str, "general") == 0) return ERROR_GENERAL;
    if (strcasecmp(str, "network") == 0) return ERROR_NETWORK;
    if (strcasecmp(str, "database") == 0) return ERROR_DATABASE;
    if (strcasecmp(str, "authentication") == 0) return ERROR_AUTHENTICATION;
    return ERROR_GENERAL; // default
}

// Parse training data type (case-insensitive)
TrainingDataType parse_training_data_type(const char* str) {
    if (strcasecmp(str, "instruction") == 0) return TRAINING_INSTRUCTION;
    if (strcasecmp(str, "example") == 0) return TRAINING_EXAMPLE;
    if (strcasecmp(str, "qa") == 0) return TRAINING_QA;
    return TRAINING_EXAMPLE; // default
}

// Parse todozi format
TodoziError parse_todozi_format(const char* todozi_text, Task* task) {
    const char* start_tag = "<todozi>";
    const char* end_tag = "</todozi>";
    
    char* start_pos = strstr(todozi_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(todozi_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 5) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize task
    memset(task, 0, sizeof(Task));
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    task->id = malloc(37); // UUID string length + null terminator
    if (task->id == NULL) {
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    uuid_unparse(uuid, task->id);
    
    task->user_id = string_copy("anonymous");
    task->action = string_copy(parts[0]);
    task->time = string_copy(parts[1]);
    task->priority = parse_priority(parts[2]);
    task->parent_project = string_copy(parts[3]);
    task->status = parse_status(parts[4]);
    
    // Check for allocation failures
    if (task->user_id == NULL || task->action == NULL || task->time == NULL || task->parent_project == NULL) {
        free_task_contents(task);
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    
    // Optional fields
    if (parts_count > 5 && strlen(parts[5]) > 0) {
        char* trimmed = string_copy(parts[5]);
        if (trimmed != NULL) {
            trim_whitespace(trimmed);
            task->assignee_type = parse_assignee(trimmed, &task->assignee_name);
            free(trimmed);
        } else {
            task->assignee_type = ASSIGNEE_HUMAN;
        }
    } else {
        task->assignee_type = ASSIGNEE_HUMAN;
    }
    
    if (parts_count > 6 && strlen(parts[6]) > 0) {
        task->tags = split_string(parts[6], ",", &task->tags_count);
        if (task->tags != NULL) {
            // Trim whitespace from tags
            for (size_t i = 0; i < task->tags_count; i++) {
                char* trimmed = string_copy(task->tags[i]);
                if (trimmed != NULL) {
                    trim_whitespace(trimmed);
                    free(task->tags[i]);
                    task->tags[i] = trimmed;
                }
            }
        }
    }
    
    if (parts_count > 7 && strlen(parts[7]) > 0) {
        task->dependencies = split_string(parts[7], ",", &task->dependencies_count);
        if (task->dependencies != NULL) {
            // Trim whitespace from dependencies
            for (size_t i = 0; i < task->dependencies_count; i++) {
                char* trimmed = string_copy(task->dependencies[i]);
                if (trimmed != NULL) {
                    trim_whitespace(trimmed);
                    free(task->dependencies[i]);
                    task->dependencies[i] = trimmed;
                }
            }
        }
    }
    
    if (parts_count > 8 && strlen(parts[8]) > 0) {
        task->context_notes = string_copy(parts[8]);
    }
    
    if (parts_count > 9 && strlen(parts[9]) > 0) {
        task->progress = atoi(parts[9]);
        task->has_progress = 1;
    }
    
    task->created_at = time(NULL);
    task->updated_at = time(NULL);
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Process chat message to extract tasks
TodoziError process_chat_message(const char* message, Task** tasks, size_t* tasks_count) {
    *tasks = NULL;
    *tasks_count = 0;
    
    regex_t regex;
    if (regcomp(&regex, "<todozi>.*?</todozi>", REG_EXTENDED) != 0) {
        return TODOZI_PARSE_ERROR;
    }
    
    const char* cursor = message;
    Task* result_tasks = NULL;
    size_t count = 0;
    
    regmatch_t matches[1];
    while (regexec(&regex, cursor, 1, matches, 0) == 0) {
        size_t match_start = matches[0].rm_so;
        size_t match_end = matches[0].rm_eo;
        size_t match_len = match_end - match_start;
        
        char* todozi_text = malloc(match_len + 1);
        if (todozi_text == NULL) {
            regfree(&regex);
            free(result_tasks);
            return TODOZI_STORAGE_ERROR;
        }
        memcpy(todozi_text, cursor + match_start, match_len);
        todozi_text[match_len] = '\0';
        
        Task task;
        TodoziError err = parse_todozi_format(todozi_text, &task);
        free(todozi_text);
        
        if (err == TODOZI_SUCCESS) {
            Task* temp = realloc(result_tasks, (count + 1) * sizeof(Task));
            if (temp == NULL) {
                free_task_contents(&task);
                regfree(&regex);
                for (size_t i = 0; i < count; i++) {
                    free_task_contents(&result_tasks[i]);
                }
                free(result_tasks);
                return TODOZI_STORAGE_ERROR;
            }
            result_tasks = temp;
            result_tasks[count] = task;
            count++;
        } else {
            // Clean up failed parse attempt
            free_task_contents(&task);
        }
        
        cursor += match_end;
    }
    
    regfree(&regex);
    *tasks = result_tasks;
    *tasks_count = count;
    
    return TODOZI_SUCCESS;
}

// Parse memory format
TodoziError parse_memory_format(const char* memory_text, const char* user_id, Memory* memory) {
    const char* start_tag = "<memory>";
    const char* end_tag = "</memory>";
    
    char* start_pos = strstr(memory_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(memory_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 6) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize memory
    memset(memory, 0, sizeof(Memory));
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    memory->id = malloc(37);
    if (memory->id == NULL) {
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    uuid_unparse(uuid, memory->id);
    
    memory->user_id = string_copy(user_id);
    memory->moment = string_copy(parts[1]);
    memory->meaning = string_copy(parts[2]);
    memory->reason = string_copy(parts[3]);
    memory->importance = parse_memory_importance(parts[4]);
    memory->term = parse_memory_term(parts[5]);
    memory->memory_type = parse_memory_type(parts[0]);
    
    if (memory->memory_type == MEMORY_EMOTIONAL) {
        memory->emotion = string_copy(parts[0]);
    }
    
    if (parts_count > 6 && strlen(parts[6]) > 0) {
        memory->tags = split_string(parts[6], ",", &memory->tags_count);
        if (memory->tags != NULL) {
            // Trim whitespace from tags
            for (size_t i = 0; i < memory->tags_count; i++) {
                char* trimmed = string_copy(memory->tags[i]);
                if (trimmed != NULL) {
                    trim_whitespace(trimmed);
                    free(memory->tags[i]);
                    memory->tags[i] = trimmed;
                }
            }
        }
    }
    
    memory->created_at = time(NULL);
    memory->updated_at = time(NULL);
    memory->status = ITEM_ACTIVE;
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Parse idea format
TodoziError parse_idea_format(const char* idea_text, Idea* idea) {
    const char* start_tag = "<idea>";
    const char* end_tag = "</idea>";
    
    char* start_pos = strstr(idea_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(idea_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 3) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize idea
    memset(idea, 0, sizeof(Idea));
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    idea->id = malloc(37);
    if (idea->id == NULL) {
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    uuid_unparse(uuid, idea->id);
    
    idea->idea = string_copy(parts[0]);
    idea->share = parse_share_level(parts[1]);
    idea->importance = parse_idea_importance(parts[2]);
    
    idea->created_at = time(NULL);
    idea->updated_at = time(NULL);
    idea->status = ITEM_ACTIVE;
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Parse agent assignment format - using implementation from agent.c
static TodoziError parse_agent_assignment_format(const char* agent_text, AgentAssignment* assignment) {
    const char* start_tag = "<todozi_agent>";
    const char* end_tag = "</todozi_agent>";
    
    char* start_pos = strstr(agent_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(agent_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 3) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize assignment
    memset(assignment, 0, sizeof(AgentAssignment));
    
    assignment->agent_id = string_copy(parts[0]);
    assignment->task_id = string_copy(parts[1]);
    assignment->project_id = string_copy(parts[2]);
    assignment->assigned_at = time(NULL);
    assignment->status = ASSIGNMENT_ASSIGNED;
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Parse error format
TodoziError parse_error_format(const char* error_text, Error* error) {
    const char* start_tag = "<error>";
    const char* end_tag = "</error>";
    
    char* start_pos = strstr(error_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(error_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 5) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize error
    memset(error, 0, sizeof(Error));
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    error->id = malloc(37);
    if (error->id == NULL) {
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    uuid_unparse(uuid, error->id);
    
    error->title = string_copy(parts[0]);
    error->description = string_copy(parts[1]);
    error->severity = parse_error_severity(parts[2]);
    error->category = parse_error_category(parts[3]);
    error->source = string_copy(parts[4]);
    
    if (parts_count > 5 && strlen(parts[5]) > 0) {
        error->context = string_copy(parts[5]);
    }
    
    if (parts_count > 6 && strlen(parts[6]) > 0) {
        error->tags = split_string(parts[6], ",", &error->tags_count);
        if (error->tags != NULL) {
            // Trim whitespace from tags
            for (size_t i = 0; i < error->tags_count; i++) {
                char* trimmed = string_copy(error->tags[i]);
                if (trimmed != NULL) {
                    trim_whitespace(trimmed);
                    free(error->tags[i]);
                    error->tags[i] = trimmed;
                }
            }
        }
    }
    
    error->created_at = time(NULL);
    error->updated_at = time(NULL);
    error->resolved = 0;
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Parse training data format
TodoziError parse_training_data_format(const char* train_text, TrainingData* training_data) {
    const char* start_tag = "<train>";
    const char* end_tag = "</train>";
    
    char* start_pos = strstr(train_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(train_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 4) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize training data
    memset(training_data, 0, sizeof(TrainingData));
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    training_data->id = malloc(37);
    if (training_data->id == NULL) {
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    uuid_unparse(uuid, training_data->id);
    
    training_data->data_type = parse_training_data_type(parts[0]);
    training_data->prompt = string_copy(parts[1]);
    training_data->completion = string_copy(parts[2]);
    
    if (parts_count > 3 && strlen(parts[3]) > 0) {
        training_data->context = string_copy(parts[3]);
    }
    
    if (parts_count > 4 && strlen(parts[4]) > 0) {
        training_data->tags = split_string(parts[4], ",", &training_data->tags_count);
        if (training_data->tags != NULL) {
            // Trim whitespace from tags
            for (size_t i = 0; i < training_data->tags_count; i++) {
                char* trimmed = string_copy(training_data->tags[i]);
                if (trimmed != NULL) {
                    trim_whitespace(trimmed);
                    free(training_data->tags[i]);
                    training_data->tags[i] = trimmed;
                }
            }
        }
    }
    
    if (parts_count > 5 && strlen(parts[5]) > 0) {
        training_data->quality_score = atof(parts[5]);
        training_data->has_quality_score = 1;
    }
    
    if (parts_count > 6) {
        training_data->source = string_copy(parts[6]);
    } else {
        training_data->source = string_copy("unknown");
    }
    
    training_data->created_at = time(NULL);
    training_data->updated_at = time(NULL);
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Parse feeling format
TodoziError parse_feeling_format(const char* feel_text, Feeling* feeling) {
    const char* start_tag = "<feel>";
    const char* end_tag = "</feel>";
    
    char* start_pos = strstr(feel_text, start_tag);
    if (start_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    char* end_pos = strstr(feel_text, end_tag);
    if (end_pos == NULL) {
        return TODOZI_VALIDATION_ERROR;
    }
    
    size_t content_len = end_pos - (start_pos + strlen(start_tag));
    char* content = malloc(content_len + 1);
    if (content == NULL) {
        return TODOZI_STORAGE_ERROR;
    }
    memcpy(content, start_pos + strlen(start_tag), content_len);
    content[content_len] = '\0';
    
    // Split by semicolon
    size_t parts_count;
    char** parts = split_string(content, ";", &parts_count);
    free(content);
    
    if (parts == NULL || parts_count < 3) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    int intensity = atoi(parts[1]);
    if (intensity < 1 || intensity > 10) {
        free_string_array(parts, parts_count);
        return TODOZI_VALIDATION_ERROR;
    }
    
    // Initialize feeling
    memset(feeling, 0, sizeof(Feeling));
    
    // Generate UUID
    uuid_t uuid;
    uuid_generate(uuid);
    feeling->id = malloc(37);
    if (feeling->id == NULL) {
        free_string_array(parts, parts_count);
        return TODOZI_STORAGE_ERROR;
    }
    uuid_unparse(uuid, feeling->id);
    
    feeling->emotion = string_copy(parts[0]);
    feeling->intensity = intensity;
    feeling->description = string_copy(parts[2]);
    
    if (parts_count > 3) {
        feeling->context = string_copy(parts[3]);
    } else {
        feeling->context = string_copy("general");
    }
    
    if (parts_count > 4 && strlen(parts[4]) > 0) {
        feeling->tags = split_string(parts[4], ",", &feeling->tags_count);
        if (feeling->tags != NULL) {
            // Trim whitespace from tags
            for (size_t i = 0; i < feeling->tags_count; i++) {
                char* trimmed = string_copy(feeling->tags[i]);
                if (trimmed != NULL) {
                    trim_whitespace(trimmed);
                    free(feeling->tags[i]);
                    feeling->tags[i] = trimmed;
                }
            }
        }
    }
    
    feeling->created_at = time(NULL);
    feeling->updated_at = time(NULL);
    
    free_string_array(parts, parts_count);
    return TODOZI_SUCCESS;
}

// Process chat message extended
TodoziError process_chat_message_extended(const char* message, const char* user_id, ChatContent* content) {
    char* transformed_message = transform_shorthand_tags(message);
    if (transformed_message == NULL) {
        return TODOZI_PARSE_ERROR;
    }
    
    // Initialize content
    memset(content, 0, sizeof(ChatContent));
    
    TodoziError err = TODOZI_SUCCESS;
    
    // Parse tasks
    err = process_chat_message(transformed_message, &content->tasks, &content->tasks_count);
    if (err != TODOZI_SUCCESS) {
        free(transformed_message);
        free_chat_content(content);
        return err;
    }
    
    // Parse memories
    regex_t memory_regex;
    if (regcomp(&memory_regex, "<memory>.*?</memory>", REG_EXTENDED) == 0) {
        const char* cursor = transformed_message;
        Memory* memories = NULL;
        size_t count = 0;
        
        regmatch_t memory_matches[1];
        while (regexec(&memory_regex, cursor, 1, memory_matches, 0) == 0) {
            size_t match_start = memory_matches[0].rm_so;
            size_t match_end = memory_matches[0].rm_eo;
            size_t match_len = match_end - match_start;
            
            char* memory_text = malloc(match_len + 1);
            if (memory_text == NULL) {
                regfree(&memory_regex);
                free(memories);
                free(transformed_message);
                free_chat_content(content);
                return TODOZI_STORAGE_ERROR;
            }
            memcpy(memory_text, cursor + match_start, match_len);
            memory_text[match_len] = '\0';
            
            Memory memory;
            TodoziError parse_err = parse_memory_format(memory_text, user_id, &memory);
            free(memory_text);
            
            if (parse_err == TODOZI_SUCCESS) {
                Memory* temp = realloc(memories, (count + 1) * sizeof(Memory));
                if (temp == NULL) {
                    free_memory(&memory);
                    regfree(&memory_regex);
                    for (size_t i = 0; i < count; i++) {
                        free_memory(&memories[i]);
                    }
                    free(memories);
                    free(transformed_message);
                    free_chat_content(content);
                    return TODOZI_STORAGE_ERROR;
                }
                memories = temp;
                memories[count] = memory;
                count++;
            }
            
            cursor += match_end;
        }
        
        content->memories = memories;
        content->memories_count = count;
        regfree(&memory_regex);
    }
    
    // Parse ideas
    regex_t idea_regex;
    if (regcomp(&idea_regex, "<idea>.*?</idea>", REG_EXTENDED) == 0) {
        const char* cursor = transformed_message;
        Idea* ideas = NULL;
        size_t count = 0;
        
        regmatch_t idea_matches[1];
        while (regexec(&idea_regex, cursor, 1, idea_matches, 0) == 0) {
            size_t match_start = idea_matches[0].rm_so;
            size_t match_end = idea_matches[0].rm_eo;
            size_t match_len = match_end - match_start;
            
            char* idea_text = malloc(match_len + 1);
            if (idea_text == NULL) {
                regfree(&idea_regex);
                free(ideas);
                free(transformed_message);
                free_chat_content(content);
                return TODOZI_STORAGE_ERROR;
            }
            memcpy(idea_text, cursor + match_start, match_len);
            idea_text[match_len] = '\0';
            
            Idea idea;
            TodoziError parse_err = parse_idea_format(idea_text, &idea);
            free(idea_text);
            
            if (parse_err == TODOZI_SUCCESS) {
                Idea* temp = realloc(ideas, (count + 1) * sizeof(Idea));
                if (temp == NULL) {
                    free_idea(&idea);
                    regfree(&idea_regex);
                    for (size_t i = 0; i < count; i++) {
                        free_idea(&ideas[i]);
                    }
                    free(ideas);
                    free(transformed_message);
                    free_chat_content(content);
                    return TODOZI_STORAGE_ERROR;
                }
                ideas = temp;
                ideas[count] = idea;
                count++;
            }
            
            cursor += match_end;
        }
        
        content->ideas = ideas;
        content->ideas_count = count;
        regfree(&idea_regex);
    }
    
    // Parse agent assignments
    regex_t agent_regex;
    if (regcomp(&agent_regex, "<todozi_agent>.*?</todozi_agent>", REG_EXTENDED) == 0) {
        const char* cursor = transformed_message;
        AgentAssignment* assignments = NULL;
        size_t count = 0;
        
        regmatch_t agent_matches[1];
        while (regexec(&agent_regex, cursor, 1, agent_matches, 0) == 0) {
            size_t match_start = agent_matches[0].rm_so;
            size_t match_end = agent_matches[0].rm_eo;
            size_t match_len = match_end - match_start;
            
            char* agent_text = malloc(match_len + 1);
            if (agent_text == NULL) {
                regfree(&agent_regex);
                free(assignments);
                free(transformed_message);
                free_chat_content(content);
                return TODOZI_STORAGE_ERROR;
            }
            memcpy(agent_text, cursor + match_start, match_len);
            agent_text[match_len] = '\0';
            
            AgentAssignment assignment;
            TodoziError parse_err = parse_agent_assignment_format(agent_text, &assignment);
            free(agent_text);
            
            if (parse_err == TODOZI_SUCCESS) {
                AgentAssignment* temp = realloc(assignments, (count + 1) * sizeof(AgentAssignment));
                if (temp == NULL) {
                    free_agent_assignment(&assignment);
                    regfree(&agent_regex);
                    for (size_t i = 0; i < count; i++) {
                        free_agent_assignment(&assignments[i]);
                    }
                    free(assignments);
                    free(transformed_message);
                    free_chat_content(content);
                    return TODOZI_STORAGE_ERROR;
                }
                assignments = temp;
                assignments[count] = assignment;
                count++;
            }
            
            cursor += match_end;
        }
        
        content->agent_assignments = assignments;
        content->agent_assignments_count = count;
        regfree(&agent_regex);
    }
    
    // Parse errors
    regex_t error_regex;
    if (regcomp(&error_regex, "<error>.*?</error>", REG_EXTENDED) == 0) {
        const char* cursor = transformed_message;
        Error* errors = NULL;
        size_t count = 0;
        
        regmatch_t error_matches[1];
        while (regexec(&error_regex, cursor, 1, error_matches, 0) == 0) {
            size_t match_start = error_matches[0].rm_so;
            size_t match_end = error_matches[0].rm_eo;
            size_t match_len = match_end - match_start;
            
            char* error_text = malloc(match_len + 1);
            if (error_text == NULL) {
                regfree(&error_regex);
                free(errors);
                free(transformed_message);
                free_chat_content(content);
                return TODOZI_STORAGE_ERROR;
            }
            memcpy(error_text, cursor + match_start, match_len);
            error_text[match_len] = '\0';
            
            Error error;
            TodoziError parse_err = parse_error_format(error_text, &error);
            free(error_text);
            
            if (parse_err == TODOZI_SUCCESS) {
                Error* temp = realloc(errors, (count + 1) * sizeof(Error));
                if (temp == NULL) {
                    free_error(&error);
                    regfree(&error_regex);
                    for (size_t i = 0; i < count; i++) {
                        free_error(&errors[i]);
                    }
                    free(errors);
                    free(transformed_message);
                    free_chat_content(content);
                    return TODOZI_STORAGE_ERROR;
                }
                errors = temp;
                errors[count] = error;
                count++;
            }
            
            cursor += match_end;
        }
        
        content->errors = errors;
        content->errors_count = count;
        regfree(&error_regex);
    }
    
    // Parse training data
    regex_t train_regex;
    if (regcomp(&train_regex, "<train>.*?</train>", REG_EXTENDED) == 0) {
        const char* cursor = transformed_message;
        TrainingData* training_data = NULL;
        size_t count = 0;
        
        regmatch_t train_matches[1];
        while (regexec(&train_regex, cursor, 1, train_matches, 0) == 0) {
            size_t match_start = train_matches[0].rm_so;
            size_t match_end = train_matches[0].rm_eo;
            size_t match_len = match_end - match_start;
            
            char* train_text = malloc(match_len + 1);
            if (train_text == NULL) {
                regfree(&train_regex);
                free(training_data);
                free(transformed_message);
                free_chat_content(content);
                return TODOZI_STORAGE_ERROR;
            }
            memcpy(train_text, cursor + match_start, match_len);
            train_text[match_len] = '\0';
            
            TrainingData data;
            TodoziError parse_err = parse_training_data_format(train_text, &data);
            free(train_text);
            
            if (parse_err == TODOZI_SUCCESS) {
                TrainingData* temp = realloc(training_data, (count + 1) * sizeof(TrainingData));
                if (temp == NULL) {
                    free_training_data(&data);
                    regfree(&train_regex);
                    for (size_t i = 0; i < count; i++) {
                        free_training_data(&training_data[i]);
                    }
                    free(training_data);
                    free(transformed_message);
                    free_chat_content(content);
                    return TODOZI_STORAGE_ERROR;
                }
                training_data = temp;
                training_data[count] = data;
                count++;
            }
            
            cursor += match_end;
        }
        
        content->training_data = training_data;
        content->training_data_count = count;
        regfree(&train_regex);
    }
    
    // Parse feelings
    regex_t feel_regex;
    if (regcomp(&feel_regex, "<feel>.*?</feel>", REG_EXTENDED) == 0) {
        const char* cursor = transformed_message;
        Feeling* feelings = NULL;
        size_t count = 0;
        
        regmatch_t feel_matches[1];
        while (regexec(&feel_regex, cursor, 1, feel_matches, 0) == 0) {
            size_t match_start = feel_matches[0].rm_so;
            size_t match_end = feel_matches[0].rm_eo;
            size_t match_len = match_end - match_start;
            
            char* feel_text = malloc(match_len + 1);
            if (feel_text == NULL) {
                regfree(&feel_regex);
                free(feelings);
                free(transformed_message);
                free_chat_content(content);
                return TODOZI_STORAGE_ERROR;
            }
            memcpy(feel_text, cursor + match_start, match_len);
            feel_text[match_len] = '\0';
            
            Feeling feeling;
            TodoziError parse_err = parse_feeling_format(feel_text, &feeling);
            free(feel_text);
            
            if (parse_err == TODOZI_SUCCESS) {
                Feeling* temp = realloc(feelings, (count + 1) * sizeof(Feeling));
                if (temp == NULL) {
                    free_feeling(&feeling);
                    regfree(&feel_regex);
                    for (size_t i = 0; i < count; i++) {
                        free_feeling(&feelings[i]);
                    }
                    free(feelings);
                    free(transformed_message);
                    free_chat_content(content);
                    return TODOZI_STORAGE_ERROR;
                }
                feelings = temp;
                feelings[count] = feeling;
                count++;
            }
            
            cursor += match_end;
        }
        
        content->feelings = feelings;
        content->feelings_count = count;
        regfree(&feel_regex);
    }
    
    free(transformed_message);
    return TODOZI_SUCCESS;
}

// Free task - implementation is in lib.c

// Free memory
void free_memory(Memory* memory) {
    if (memory == NULL) return;
    
    free(memory->id);
    free(memory->user_id);
    free(memory->project_id);
    free(memory->moment);
    free(memory->meaning);
    free(memory->reason);
    free(memory->emotion);
    free_string_array(memory->tags, memory->tags_count);
}

// Free idea
void free_idea(Idea* idea) {
    if (idea == NULL) return;
    
    free(idea->id);
    free(idea->idea);
    free(idea->project_id);
    free_string_array(idea->tags, idea->tags_count);
    free(idea->context);
}

// Free agent assignment
void free_agent_assignment(AgentAssignment* assignment) {
    if (assignment == NULL) return;
    
    free(assignment->agent_id);
    free(assignment->task_id);
    free(assignment->project_id);
}

// Free error
void free_error(Error* error) {
    if (error == NULL) return;
    
    free(error->id);
    free(error->title);
    free(error->description);
    free(error->source);
    free(error->context);
    free_string_array(error->tags, error->tags_count);
    free(error->resolution);
}

// Free training data
void free_training_data(TrainingData* data) {
    if (data == NULL) return;
    
    free(data->id);
    free(data->prompt);
    free(data->completion);
    free(data->context);
    free_string_array(data->tags, data->tags_count);
    free(data->source);
}

// Free feeling
void free_feeling(Feeling* feeling) {
    if (feeling == NULL) return;
    
    free(feeling->id);
    free(feeling->emotion);
    free(feeling->description);
    free(feeling->context);
    free_string_array(feeling->tags, feeling->tags_count);
}

// Free chat content
static void free_chat_content(ChatContent* content) {
    if (content == NULL) return;
    
    for (size_t i = 0; i < content->tasks_count; i++) {
        free_task_contents(&content->tasks[i]);
    }
    free(content->tasks);
    
    for (size_t i = 0; i < content->memories_count; i++) {
        free_memory(&content->memories[i]);
    }
    free(content->memories);
    
    for (size_t i = 0; i < content->ideas_count; i++) {
        free_idea(&content->ideas[i]);
    }
    free(content->ideas);
    
    for (size_t i = 0; i < content->agent_assignments_count; i++) {
        free_agent_assignment(&content->agent_assignments[i]);
    }
    free(content->agent_assignments);
    
    for (size_t i = 0; i < content->errors_count; i++) {
        free_error(&content->errors[i]);
    }
    free(content->errors);
    
    for (size_t i = 0; i < content->training_data_count; i++) {
        free_training_data(&content->training_data[i]);
    }
    free(content->training_data);
    
    for (size_t i = 0; i < content->feelings_count; i++) {
        free_feeling(&content->feelings[i]);
    }
    free(content->feelings);
}