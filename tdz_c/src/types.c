#ifndef TODOZI_TYPES_H
#define TODOZI_TYPES_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>
#include <stddef.h>

// Forward declarations for structs that would be defined in other files
typedef struct CodeChunk CodeChunk;
typedef struct AgentAssignment AgentAssignment;
typedef struct Error Error;
typedef struct Feeling Feeling;
typedef struct Idea Idea;
typedef struct Memory Memory;
typedef struct Task Task;
typedef struct TrainingData TrainingData;

// Date/Time representation using time_t (seconds since epoch)
typedef time_t DateTime;

// Optional field macros for consistency
#define OPTIONAL_PTR(type, name) \
    type name; \
    bool has_##name;

#define OPTIONAL_VALUE(type, name) \
    struct { type value; bool present; } name;

// Enum definitions
typedef enum {
    CMD_INIT,
    CMD_ADD,
    CMD_LIST,
    CMD_SHOW,
    CMD_UPDATE,
    CMD_COMPLETE,
    CMD_FIX_CONSISTENCY,
    CMD_CHECK_STRUCTURE,
    CMD_ENSURE_STRUCTURE,
    CMD_REGISTER,
    CMD_REGISTRATION_STATUS,
    CMD_CLEAR_REGISTRATION,
    CMD_DELETE,
    CMD_PROJECT,
    CMD_SEARCH,
    CMD_STATS,
    CMD_BACKUP,
    CMD_LIST_BACKUPS,
    CMD_RESTORE,
    CMD_MEMORY,
    CMD_IDEA,
    CMD_AGENT,
    CMD_EMB,
    CMD_ERROR_CMD,
    CMD_TRAIN,
    CMD_CHAT,
    CMD_SEARCH_ALL,
    CMD_MAESTRO,
    CMD_SERVER,
    CMD_ML,
    CMD_IND_DEMO,
    CMD_QUEUE,
    CMD_API,
    CMD_TDZ_CNT,
    CMD_EXPORT_EMBEDDINGS,
    CMD_MIGRATE,
    CMD_TUI,
    CMD_EXTRACT,
    CMD_STRATEGY,
    CMD_STEPS
} CommandType;

typedef enum {
    CMD_ADD_TASK
} AddCommandType;

typedef enum {
    CMD_LIST_TASKS
} ListCommandType;

typedef enum {
    CMD_SHOW_TASK
} ShowCommandType;

typedef enum {
    CMD_SEARCH_TASKS
} SearchCommandType;

typedef enum {
    CMD_STATS_SHOW
} StatsCommandType;

typedef enum {
    CMD_BACKUP_CREATE
} BackupCommandType;

typedef enum {
    CMD_PROJECT_CREATE,
    CMD_PROJECT_LIST,
    CMD_PROJECT_SHOW,
    CMD_PROJECT_ARCHIVE,
    CMD_PROJECT_DELETE,
    CMD_PROJECT_UPDATE
} ProjectCommandType;

typedef enum {
    CMD_STEPS_SHOW,
    CMD_STEPS_ADD,
    CMD_STEPS_UPDATE,
    CMD_STEPS_DONE,
    CMD_STEPS_ARCHIVE
} StepsCommandType;

typedef enum {
    CMD_MEMORY_CREATE,
    CMD_MEMORY_CREATE_SECRET,
    CMD_MEMORY_CREATE_HUMAN,
    CMD_MEMORY_CREATE_EMOTIONAL,
    CMD_MEMORY_LIST,
    CMD_MEMORY_SHOW,
    CMD_MEMORY_TYPES
} MemoryCommandType;

typedef enum {
    CMD_IDEA_CREATE,
    CMD_IDEA_LIST,
    CMD_IDEA_SHOW
} IdeaCommandType;

typedef enum {
    CMD_AGENT_LIST,
    CMD_AGENT_SHOW,
    CMD_AGENT_CREATE,
    CMD_AGENT_ASSIGN,
    CMD_AGENT_UPDATE,
    CMD_AGENT_DELETE
} AgentCommandType;

typedef enum {
    CMD_EMB_SET_MODEL,
    CMD_EMB_SHOW_MODEL,
    CMD_EMB_LIST_MODELS
} EmbCommandType;

typedef enum {
    CMD_ERROR_CREATE,
    CMD_ERROR_LIST,
    CMD_ERROR_SHOW,
    CMD_ERROR_RESOLVE,
    CMD_ERROR_DELETE
} ErrorCommandType;

typedef enum {
    CMD_TRAINING_CREATE,
    CMD_TRAINING_LIST,
    CMD_TRAINING_SHOW,
    CMD_TRAINING_STATS,
    CMD_TRAINING_EXPORT,
    CMD_TRAINING_COLLECT,
    CMD_TRAINING_UPDATE,
    CMD_TRAINING_DELETE
} TrainingCommandType;

typedef enum {
    CMD_MAESTRO_INIT,
    CMD_MAESTRO_COLLECT_CONVERSATION,
    CMD_MAESTRO_COLLECT_TOOL,
    CMD_MAESTRO_LIST,
    CMD_MAESTRO_STATS,
    CMD_MAESTRO_EXPORT,
    CMD_MAESTRO_INTEGRATE
} MaestroCommandType;

typedef enum {
    CMD_SERVER_START,
    CMD_SERVER_STATUS,
    CMD_SERVER_ENDPOINTS
} ServerCommandType;

typedef enum {
    CMD_ML_PROCESS,
    CMD_ML_TRAIN,
    CMD_ML_LIST,
    CMD_ML_SHOW,
    CMD_ML_LOAD,
    CMD_ML_SAVE,
    CMD_ML_TEST,
    CMD_ML_GENERATE_TRAINING_DATA,
    CMD_ML_ADVANCED_PROCESS,
    CMD_ML_ADVANCED_TRAIN,
    CMD_ML_ADVANCED_INFER
} MLCommandType;

typedef enum {
    CMD_QUEUE_PLAN,
    CMD_QUEUE_LIST,
    CMD_QUEUE_BACKLOG,
    CMD_QUEUE_ACTIVE,
    CMD_QUEUE_COMPLETE,
    CMD_QUEUE_START,
    CMD_QUEUE_END
} QueueCommandType;

typedef enum {
    CMD_API_REGISTER,
    CMD_API_LIST,
    CMD_API_CHECK,
    CMD_API_DEACTIVATE,
    CMD_API_ACTIVATE,
    CMD_API_REMOVE
} ApiCommandType;

typedef enum {
    QUEUE_STATUS_BACKLOG,
    QUEUE_STATUS_ACTIVE,
    QUEUE_STATUS_COMPLETE
} QueueStatus;

// Struct definitions
typedef struct {
    char* id;
    OPTIONAL_PTR(char*, action);
    OPTIONAL_PTR(char*, time);
    OPTIONAL_PTR(char*, priority);
    OPTIONAL_PTR(char*, project);
    OPTIONAL_PTR(char*, status);
    OPTIONAL_PTR(char*, assignee);
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_PTR(char*, dependencies);
    OPTIONAL_PTR(char*, context);
    OPTIONAL_VALUE(unsigned char, progress);
} TaskUpdate;

// NOTE: UpdateCommand uses pointer for progress, while TaskUpdate uses OPTIONAL_VALUE
// This inconsistency should be reviewed and potentially unified
typedef struct {
    char* id;
    char* action;
    char* time;
    char* priority;
    char* project;
    char* status;
    char* assignee;
    char* tags;
    char* dependencies;
    char* context;
    unsigned char* progress;  // Inconsistent with TaskUpdate::progress (value vs pointer)
} UpdateCommand;

typedef struct {
    char* id;
} CompleteCommand;

typedef struct {
    char* server_url;
} RegisterCommand;

typedef struct {
    char* backup_name;
} RestoreCommand;

typedef struct {
    char* id;
} DeleteCommand;

typedef struct {
    char* task_id;
    char* step;
} StepsAddCommand;

typedef struct {
    char* task_id;
    size_t step_index;
    char* new_step;
} StepsUpdateCommand;

typedef struct {
    char* task_id;
} StepsTaskCommand;

typedef struct {
    char* moment;
    char* meaning;
    char* reason;
    char* importance;
    char* term;
    char* memory_type;
    OPTIONAL_PTR(char*, tags);
} MemoryCreateCommand;

typedef struct {
    char* idea;
    char* share;
    char* importance;
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_PTR(char*, context);
} IdeaCreateCommand;

typedef struct {
    char* id;
    char* name;
    char* description;
    char* category;
    OPTIONAL_PTR(char*, capabilities);
    OPTIONAL_PTR(char*, specializations);
    char* model_provider;
    char* model_name;
    float temperature;
    unsigned int max_tokens;
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_PTR(char*, system_prompt);
    OPTIONAL_PTR(char*, prompt_template);
    OPTIONAL_VALUE(bool, auto_format_code);
    OPTIONAL_VALUE(bool, include_examples);
    OPTIONAL_VALUE(bool, explain_complexity);
    OPTIONAL_VALUE(bool, suggest_tests);
    OPTIONAL_PTR(char*, tools);
    OPTIONAL_VALUE(unsigned int, max_response_length);
    OPTIONAL_VALUE(unsigned int, timeout_seconds);
    OPTIONAL_VALUE(unsigned int, requests_per_minute);
    OPTIONAL_VALUE(unsigned int, tokens_per_hour);
} AgentCreateCommand;

typedef struct {
    char* agent_id;
    char* task_id;
    char* project_id;
} AgentAssignCommand;

typedef struct {
    char* id;
    OPTIONAL_PTR(char*, name);
    OPTIONAL_PTR(char*, description);
    OPTIONAL_PTR(char*, category);
    OPTIONAL_PTR(char*, capabilities);
    OPTIONAL_PTR(char*, specializations);
    OPTIONAL_PTR(char*, system_prompt);
    OPTIONAL_PTR(char*, prompt_template);
    OPTIONAL_PTR(char*, model_provider);
    OPTIONAL_PTR(char*, model_name);
    OPTIONAL_VALUE(float, temperature);
    OPTIONAL_VALUE(unsigned int, max_tokens);
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_VALUE(bool, auto_format_code);
    OPTIONAL_VALUE(bool, include_examples);
    OPTIONAL_VALUE(bool, explain_complexity);
    OPTIONAL_VALUE(bool, suggest_tests);
    OPTIONAL_PTR(char*, tools);
    OPTIONAL_VALUE(unsigned int, max_response_length);
    OPTIONAL_VALUE(unsigned int, timeout_seconds);
    OPTIONAL_VALUE(unsigned int, requests_per_minute);
    OPTIONAL_VALUE(unsigned int, tokens_per_hour);
} AgentUpdateCommand;

typedef struct {
    char* model_name;
} EmbSetModelCommand;

typedef struct {
    char* title;
    char* description;
    char* severity;
    char* category;
    char* source;
    OPTIONAL_PTR(char*, context);
    OPTIONAL_PTR(char*, tags);
} ErrorCreateCommand;

typedef struct {
    OPTIONAL_PTR(char*, severity);
    OPTIONAL_PTR(char*, category);
    bool unresolved_only;
} ErrorListCommand;

typedef struct {
    char* id;
    OPTIONAL_PTR(char*, resolution);
} ErrorResolveCommand;

typedef struct {
    char* data_type;
    char* prompt;
    char* completion;
    OPTIONAL_PTR(char*, context);
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_VALUE(float, quality);
    char* source;
} TrainingCreateCommand;

typedef struct {
    OPTIONAL_PTR(char*, data_type);
    OPTIONAL_VALUE(float, min_quality);
} TrainingListCommand;

typedef struct {
    char* format;
    OPTIONAL_PTR(char*, data_type);
    OPTIONAL_VALUE(float, min_quality);
    OPTIONAL_PTR(char*, output_file);
} TrainingExportCommand;

typedef struct {
    char* message;
} TrainingCollectCommand;

typedef struct {
    char* id;
    OPTIONAL_PTR(char*, data_type);
    OPTIONAL_PTR(char*, prompt);
    OPTIONAL_PTR(char*, completion);
    OPTIONAL_PTR(char*, context);
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_VALUE(unsigned char, quality);
    OPTIONAL_PTR(char*, source);
} TrainingUpdateCommand;

typedef struct {
    char* name;
    OPTIONAL_PTR(char*, description);
} ProjectCreateCommand;

typedef struct {
    char* name;
    OPTIONAL_PTR(char*, new_name);
    OPTIONAL_PTR(char*, description);
    OPTIONAL_PTR(char*, status);
} ProjectUpdateCommand;

typedef struct {
    char* session_id;
    char* conversation;
    size_t context_length;
    OPTIONAL_PTR(char*, tool_calls);
    char* response;
    unsigned long long response_time_ms;
} MaestroCollectConversationCommand;

typedef struct {
    char* session_id;
    char* tool_name;
    char* tool_call;
    unsigned long long execution_time_ms;
    bool success;
    char* result_summary;
} MaestroCollectToolCommand;

typedef struct {
    OPTIONAL_PTR(char*, session_id);
    OPTIONAL_PTR(char*, interaction_type);
    size_t limit;
} MaestroListCommand;

typedef struct {
    char* output;
} MaestroExportCommand;

typedef struct {
    char* host;
    unsigned short port;
} ServerStartCommand;

typedef struct {
    char* text;
    bool use_ml;
    char* model;
} MLProcessCommand;

typedef struct {
    char* data;
    char* model_name;
    unsigned int epochs;
} MLTrainCommand;

typedef struct {
    char* model_name;
    char* path;
} MLLoadCommand;

typedef struct {
    char* model_name;
    char* output;
} MLSaveCommand;

typedef struct {
    char* test_data;
    char* model_name;
} MLTestCommand;

typedef struct {
    char* output;
    size_t samples;
} MLGenerateTrainingDataCommand;

typedef struct {
    char* text;
    bool analytics;
} MLAdvancedProcessCommand;

typedef struct {
    char* data;
    unsigned int epochs;
} MLAdvancedTrainCommand;

typedef struct {
    char* text;
    bool detailed;
} MLAdvancedInferCommand;

typedef struct {
    char* task_name;
    char* task_description;
    char* priority;
    OPTIONAL_PTR(char*, project_id);
} QueuePlanCommand;

typedef struct {
    OPTIONAL_PTR(char*, status);
} QueueListCommand;

typedef struct {
    char* queue_item_id;
} QueueStartCommand;

typedef struct {
    char* session_id;
} QueueEndCommand;

typedef struct {
    OPTIONAL_PTR(char*, user_id);
} ApiRegisterCommand;

typedef struct {
    bool active_only;
} ApiListCommand;

typedef struct {
    char* public_key;
    OPTIONAL_PTR(char*, private_key);
} ApiCheckCommand;

typedef struct {
    char* user_id;
} ApiUserCommand;

typedef struct {
    char* action;
    char* time;
    char* priority;
    char* project;
    char* status;
    OPTIONAL_PTR(char*, assignee);
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_PTR(char*, dependencies);
    OPTIONAL_PTR(char*, context);
    OPTIONAL_VALUE(unsigned char, progress);
} AddTaskCommand;

typedef struct {
    OPTIONAL_PTR(char*, project);
    OPTIONAL_PTR(char*, status);
    OPTIONAL_PTR(char*, priority);
    OPTIONAL_PTR(char*, assignee);
    OPTIONAL_PTR(char*, tags);
    OPTIONAL_PTR(char*, search);
} ListTasksCommand;

typedef struct {
    char* id;
} ShowTaskCommand;

typedef struct {
    char* query;
} SearchTasksCommand;

typedef struct {
    char* query;
    char* types;
} SearchAllCommand;

typedef struct {
    char* content;
    OPTIONAL_PTR(char*, session_id);
    bool no_checklist;
    bool no_session;
} TdzCntCommand;

typedef struct {
    char* output;
} ExportEmbeddingsCommand;

typedef struct {
    bool dry_run;
    bool verbose;
    bool force;
    bool cleanup;
} MigrateCommand;

typedef struct {
    OPTIONAL_PTR(char*, content);
    OPTIONAL_PTR(char*, file);
    char* output_format;
    bool human;
} ExtractCommand;

typedef struct {
    OPTIONAL_PTR(char*, content);
    OPTIONAL_PTR(char*, file);
    char* output_format;
    bool human;
} StrategyCommand;

typedef struct {
    OPTIONAL_VALUE(size_t, limit);
    OPTIONAL_PTR(char*, data_types);
    OPTIONAL_PTR(char*, since);
    OPTIONAL_PTR(char*, until);
} SearchOptions;

typedef struct {
    char* id;
    char* task_name;
    char* task_description;
    char* priority;
    OPTIONAL_PTR(char*, project_id);
    QueueStatus status;
    DateTime created_at;
    DateTime updated_at;
} QueueItem;

typedef struct {
    Task* tasks;
    Memory* memories;
    Idea* ideas;
    AgentAssignment* agent_assignments;
    CodeChunk* code_chunks;
    Error* errors;
    TrainingData* training_data;
    Feeling* feelings;
    size_t tasks_count;
    size_t memories_count;
    size_t ideas_count;
    size_t agent_assignments_count;
    size_t code_chunks_count;
    size_t errors_count;
    size_t training_data_count;
    size_t feelings_count;
} ChatContent;

typedef struct {
    Task* task_results;
    Memory* memory_results;
    Idea* idea_results;
    Error* error_results;
    TrainingData* training_results;
    size_t task_results_count;
    size_t memory_results_count;
    size_t idea_results_count;
    size_t error_results_count;
    size_t training_results_count;
} SearchResults;

// Command structures with proper hierarchy
typedef struct {
    AddCommandType type;
    union {
        AddTaskCommand task;
    } data;
} AddCommand;

typedef struct {
    ListCommandType type;
    union {
        ListTasksCommand tasks;
    } data;
} ListCommand;

typedef struct {
    ShowCommandType type;
    union {
        ShowTaskCommand task;
    } data;
} ShowCommand;

typedef struct {
    SearchCommandType type;
    union {
        SearchTasksCommand tasks;
    } data;
} SearchCommand;

typedef struct {
    StatsCommandType type;
} StatsCommand;

typedef struct {
    BackupCommandType type;
} BackupCommand;

typedef struct {
    ProjectCommandType type;
    union {
        ProjectCreateCommand create;
        ProjectUpdateCommand update;
        char* name;
    } data;
} ProjectCommand;

typedef struct {
    StepsCommandType type;
    union {
        StepsTaskCommand show;
        StepsAddCommand add;
        StepsUpdateCommand update;
        StepsTaskCommand done;
        StepsTaskCommand archive;
    } data;
} StepsCommand;

typedef struct {
    MemoryCommandType type;
    union {
        MemoryCreateCommand create;
        char* id;
        struct {
            OPTIONAL_PTR(char*, importance);
            OPTIONAL_PTR(char*, term);
            OPTIONAL_PTR(char*, memory_type);
        } list;
    } data;
} MemoryCommand;

typedef struct {
    IdeaCommandType type;
    union {
        IdeaCreateCommand create;
        char* id;
        struct {
            OPTIONAL_PTR(char*, share);
            OPTIONAL_PTR(char*, importance);
        } list;
    } data;
} IdeaCommand;

typedef struct {
    AgentCommandType type;
    union {
        AgentCreateCommand create;
        AgentAssignCommand assign;
        AgentUpdateCommand update;
        char* id;
    } data;
} AgentCommand;

typedef struct {
    EmbCommandType type;
    union {
        EmbSetModelCommand set_model;
    } data;
} EmbCommand;

typedef struct {
    ErrorCommandType type;
    union {
        ErrorCreateCommand create;
        ErrorListCommand list;
        ErrorResolveCommand resolve;
        char* id;
    } data;
} ErrorCommand;

typedef struct {
    TrainingCommandType type;
    union {
        TrainingCreateCommand create;
        TrainingListCommand list;
        TrainingExportCommand export;
        TrainingCollectCommand collect;
        TrainingUpdateCommand update;
        char* id;
    } data;
} TrainingCommand;

typedef struct {
    MaestroCommandType type;
    union {
        MaestroCollectConversationCommand collect_conversation;
        MaestroCollectToolCommand collect_tool;
        MaestroListCommand list;
        MaestroExportCommand export;
    } data;
} MaestroCommand;

typedef struct {
    ServerCommandType type;
    union {
        ServerStartCommand start;
    } data;
} ServerCommand;

typedef struct {
    MLCommandType type;
    union {
        MLProcessCommand process;
        MLTrainCommand train;
        MLLoadCommand load;
        MLSaveCommand save;
        MLTestCommand test;
        MLGenerateTrainingDataCommand generate_training_data;
        MLAdvancedProcessCommand advanced_process;
        MLAdvancedTrainCommand advanced_train;
        MLAdvancedInferCommand advanced_infer;
        char* model_name;
    } data;
} MLCommand;

typedef struct {
    QueueCommandType type;
    union {
        QueuePlanCommand plan;
        QueueListCommand list;
        QueueStartCommand start;
        QueueEndCommand end;
    } data;
} QueueCommand;

typedef struct {
    ApiCommandType type;
    union {
        ApiRegisterCommand reg;
        ApiListCommand list;
        ApiCheckCommand check;
        ApiUserCommand user;
    } data;
} ApiCommand;

// Main command structure
typedef struct {
    CommandType type;
    union {
        UpdateCommand update;
        CompleteCommand complete;
        RegisterCommand reg;
        RestoreCommand restore;
        DeleteCommand delete_cmd;
        AddCommand add;
        ListCommand list;
        ShowCommand show;
        SearchCommand search;
        StatsCommand stats;
        BackupCommand backup;
        ProjectCommand project;
        StepsCommand steps;
        MemoryCommand memory;
        IdeaCommand idea;
        AgentCommand agent;
        EmbCommand emb;
        ErrorCommand error_cmd;
        TrainingCommand train;
        MaestroCommand maestro;
        ServerCommand server;
        MLCommand ml;
        QueueCommand queue;
        ApiCommand api;
        char* message;
        SearchAllCommand search_all;
        TdzCntCommand tdz_cnt;
        ExportEmbeddingsCommand export_embeddings;
        MigrateCommand migrate;
        ExtractCommand extract;
        StrategyCommand strategy;
    } data;
} Command;

// Search Engine API
// NOTE: SearchEngine implementation is in search.c, not here.
// This file contains only type definitions and should ideally be converted to types.h
// Forward declaration for SearchEngine (actual definition in search.c)
typedef struct SearchEngine SearchEngine;

#endif // TODOZI_TYPES_H