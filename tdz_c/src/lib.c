#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>
#include <uuid/uuid.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <sys/stat.h>
#include <unistd.h>

// Forward declarations
typedef struct Task Task;
typedef struct Storage Storage;
typedef struct TodoziEmbeddingService TodoziEmbeddingService;
typedef struct TodoziEmbeddingConfig TodoziEmbeddingConfig;
typedef struct TaskFilters TaskFilters;
typedef struct TaskUpdate TaskUpdate;
typedef struct ChatContent ChatContent;
typedef struct Agent Agent;
typedef struct RegistrationInfo RegistrationInfo;
typedef struct Config Config;
typedef struct Project Project;
typedef struct Memory Memory;
typedef struct Idea Idea;
typedef struct QueueItem QueueItem;
typedef struct SimilarityResult SimilarityResult;
typedef struct ClusteringResult ClusteringResult;
typedef struct Tag Tag;
typedef struct Error Error;
typedef struct Feeling Feeling;
typedef struct TrainingData TrainingData;
typedef struct ProjectTaskContainer ProjectTaskContainer;
typedef struct AgentAssignment AgentAssignment;
typedef struct ApiKeyCollection ApiKeyCollection;
typedef struct ApiKey ApiKey;
typedef struct QueueCollection QueueCollection;
typedef struct TaskCollection TaskCollection;
typedef struct MigrationReport MigrationReport;
typedef struct ProjectStats ProjectStats;
typedef struct SearchAnalytics SearchAnalytics;
typedef struct SearchResults SearchResults;
typedef struct HierarchicalCluster HierarchicalCluster;
typedef struct IdeaStatistics IdeaStatistics;
typedef struct MemoryStatistics MemoryStatistics;
typedef struct SummaryStatistics SummaryStatistics;
typedef struct TagStatistics TagStatistics;
typedef struct DriftReport DriftReport;
typedef struct ValidationReport ValidationReport;
typedef struct ModelComparisonResult ModelComparisonResult;
typedef struct SimilarityGraph SimilarityGraph;
typedef struct LabeledCluster LabeledCluster;
typedef struct PerformanceMetrics PerformanceMetrics;
typedef struct ToolResult ToolResult;
typedef struct ToolParameter ToolParameter;
typedef struct ToolDefinition ToolDefinition;
typedef struct ResourceLock ResourceLock;
typedef struct ToolRegistry ToolRegistry;
typedef struct CodeChunk CodeChunk;
typedef struct ProjectState ProjectState;
typedef struct ContextWindow ContextWindow;
typedef struct ErrorManager ErrorManager;
typedef struct ReminderManager ReminderManager;
typedef struct Reminder Reminder;
typedef struct TagManager TagManager;
typedef struct TagSearchEngine TagSearchEngine;
typedef struct TagSearchQuery TagSearchQuery;
typedef struct DisplayConfig DisplayConfig;
typedef struct TodoziApp TodoziApp;
typedef struct TaskEditor TaskEditor;
typedef struct ProcessedAction ProcessedAction;
typedef struct ChecklistItem ChecklistItem;
typedef struct CodeGenerationGraph CodeGenerationGraph;
typedef struct SharedTodoziState SharedTodoziState;
typedef struct TodoziProcessorState TodoziProcessorState;
typedef struct TdzCommand TdzCommand;
typedef struct Helix Helix;
typedef struct DnaValue DnaValue;
typedef struct ItemStatus ItemStatus;
typedef struct ShareLevel ShareLevel;
typedef struct MemoryImportance MemoryImportance;
typedef struct MemoryTerm MemoryTerm;
typedef struct MemoryType MemoryType;
typedef struct IdeaImportance IdeaImportance;
typedef struct QueueStatus QueueStatus;
typedef struct Priority Priority;
typedef struct Status Status;
typedef struct Assignee Assignee;
typedef struct ErrorType ErrorType;
typedef struct SummaryPriority SummaryPriority;
typedef struct ChunkingLevel ChunkingLevel;
typedef struct AgentUpdate AgentUpdate;
typedef struct IdeaUpdate IdeaUpdate;
typedef struct MemoryUpdate MemoryUpdate;
typedef struct AggregationType AggregationType;
typedef struct TodoziContentType TodoziContentType;
typedef struct SearchFilters SearchFilters;
typedef struct TagSortBy TagSortBy;
typedef struct Commands Commands;
typedef struct MemoryCommands MemoryCommands;
typedef struct ProjectCommands ProjectCommands;
typedef struct QueueCommands QueueCommands;
typedef struct SearchCommands SearchCommands;
typedef struct ServerCommands ServerCommands;
typedef struct ShowCommands ShowCommands;
typedef struct StatsCommands StatsCommands;
typedef struct TrainingCommands TrainingCommands;

// Function forward declarations
void free_task(Task* task);

// Enum definitions
typedef enum {
    TODOZI_PRIORITY_CRITICAL,
    TODOZI_PRIORITY_URGENT,
    TODOZI_PRIORITY_HIGH,
    TODOZI_PRIORITY_MEDIUM,
    TODOZI_PRIORITY_LOW
} TodoziPriority;

typedef enum {
    TODOZI_STATUS_TODO,
    TODOZI_STATUS_IN_PROGRESS,
    TODOZI_STATUS_DONE,
    TODOZI_STATUS_BLOCKED
} TodoziStatus;

typedef enum {
    TODOZI_ASSIGNEE_HUMAN,
    TODOZI_ASSIGNEE_AI,
    TODOZI_ASSIGNEE_COLLABORATIVE
} TodoziAssignee;

typedef enum {
    TODOZI_QUEUE_STATUS_PENDING,
    TODOZI_QUEUE_STATUS_ACTIVE,
    TODOZI_QUEUE_STATUS_COMPLETED,
    TODOZI_QUEUE_STATUS_BACKLOG
} TodoziQueueStatus;

typedef enum {
    TODOZI_ITEM_STATUS_ACTIVE,
    TODOZI_ITEM_STATUS_ARCHIVED,
    TODOZI_ITEM_STATUS_DELETED
} TodoziItemStatus;

typedef enum {
    TODOZI_SHARE_LEVEL_PRIVATE,
    TODOZI_SHARE_LEVEL_TEAM,
    TODOZI_SHARE_LEVEL_PUBLIC
} TodoziShareLevel;

typedef enum {
    TODOZI_MEMORY_IMPORTANCE_LOW,
    TODOZI_MEMORY_IMPORTANCE_MEDIUM,
    TODOZI_MEMORY_IMPORTANCE_HIGH
} TodoziMemoryImportance;

typedef enum {
    TODOZI_MEMORY_TERM_SHORT,
    TODOZI_MEMORY_TERM_LONG
} TodoziMemoryTerm;

typedef enum {
    TODOZI_MEMORY_TYPE_STANDARD,
    TODOZI_MEMORY_TYPE_EXPERIENCE,
    TODOZI_MEMORY_TYPE_LEARNING
} TodoziMemoryType;

typedef enum {
    TODOZI_IDEA_IMPORTANCE_NORMAL,
    TODOZI_IDEA_IMPORTANCE_BREAKTHROUGH
} TodoziIdeaImportance;

typedef enum {
    TODOZI_CHUNKING_LEVEL_FILE,
    TODOZI_CHUNKING_LEVEL_CLASS,
    TODOZI_CHUNKING_LEVEL_METHOD
} TodoziChunkingLevel;

typedef enum {
    TODOZI_ERROR_TYPE_VALIDATION,
    TODOZI_ERROR_TYPE_SYSTEM,
    TODOZI_ERROR_TYPE_NETWORK
} TodoziErrorType;

typedef enum {
    TODOZI_OK = 0,
    TODOZI_ERR_NOT_IMPL = 1,
    TODOZI_ERR_IO = 2,
    TODOZI_ERR_INVALID_ARG = 3,
    TODOZI_ERR_MEMORY = 4
} TodoziErrorCode;

// Struct definitions
struct Task {
    char* id;
    char* user_id;
    char* action;
    char* time;
    TodoziPriority priority;
    char* parent_project;
    TodoziStatus status;
    TodoziAssignee assignee;
    char** tags;
    size_t tags_count;
    char** dependencies;
    size_t dependencies_count;
    char* context_notes;
    int* progress;
    time_t created_at;
    time_t updated_at;
    float* embedding_vector;
    size_t embedding_size;
};

struct Storage {
    // Storage implementation details
};

struct TodoziEmbeddingService {
    TodoziEmbeddingConfig* config;
    // Service implementation details
};

struct TodoziEmbeddingConfig {
    // Config implementation details
};

struct TaskFilters {
    char* project;
    TodoziStatus* status;
    TodoziPriority* priority;
    TodoziAssignee* assignee;
    char** tags;
    size_t tags_count;
    char* search;
};

struct TaskUpdate {
    char* action;
    TodoziPriority* priority;
    TodoziStatus* status;
    char* parent_project;
    TodoziAssignee* assignee;
    char** tags;
    size_t tags_count;
    char** dependencies;
    size_t dependencies_count;
    char* context_notes;
    int* progress;
};

struct ChatContent {
    char* response;
    Task** tasks;
    size_t tasks_count;
};

struct Agent {
    char* id;
    char* name;
    char* description;
    char** capabilities;
    size_t capabilities_count;
    char** specializations;
    size_t specializations_count;
};

struct RegistrationInfo {
    char* api_key;
    char* user_id;
    char* fingerprint;
};

struct Config {
    char* version;
    char* default_project;
    bool auto_backup;
    char* backup_interval;
    bool ai_enabled;
    char* default_assignee;
    char* date_format;
    char* timezone;
};

struct Project {
    char* name;
    char* description;
    time_t created_at;
    time_t updated_at;
};

struct Memory {
    char* id;
    char* user_id;
    char* project_id;
    TodoziItemStatus status;
    char* moment;
    char* meaning;
    char* reason;
    TodoziMemoryImportance importance;
    TodoziMemoryTerm term;
    TodoziMemoryType memory_type;
    char** tags;
    size_t tags_count;
    time_t created_at;
    time_t updated_at;
};

struct Idea {
    char* id;
    char* idea;
    char* project_id;
    TodoziItemStatus status;
    TodoziShareLevel share;
    TodoziIdeaImportance importance;
    char** tags;
    size_t tags_count;
    char* context;
    time_t created_at;
    time_t updated_at;
};

struct QueueItem {
    char* id;
    char* task_name;
    char* task_description;
    TodoziPriority priority;
    char* project;
    TodoziQueueStatus status;
    time_t created_at;
    time_t updated_at;
};

struct SimilarityResult {
    char* text_content;
    float similarity_score;
    char* content_type;
};

struct ClusteringResult {
    int cluster_id;
    char** items;
    size_t items_count;
    float* centroid;
    size_t centroid_size;
};

struct Tag {
    char* id;
    char* name;
    char* description;
    char* color;
    char* category;
    int usage_count;
    time_t created_at;
    time_t updated_at;
};

struct Error {
    char* id;
    char* message;
    TodoziErrorType error_type;
    char* context;
    time_t timestamp;
    bool resolved;
    char* resolution;
};

struct Feeling {
    char* id;
    char* user_id;
    char* emotion;
    char* description;
    int intensity;
    time_t timestamp;
    char** tags;
    size_t tags_count;
};

struct TrainingData {
    char* id;
    char* content;
    char* category;
    char** labels;
    size_t labels_count;
    time_t created_at;
    time_t updated_at;
};

struct ProjectTaskContainer {
    char* project_name;
    char* project_hash;
    Task** tasks;
    size_t tasks_count;
    time_t created_at;
    time_t updated_at;
};

struct AgentAssignment {
    char* id;
    char* agent_id;
    char* task_id;
    char* status;
    time_t assigned_at;
    time_t updated_at;
};

struct ApiKeyCollection {
    // Collection implementation details
};

struct ApiKey {
    char* public_key;
    char* private_key;
    bool is_admin;
    time_t created_at;
    time_t expires_at;
};

struct QueueCollection {
    QueueItem** items;
    size_t items_count;
};

struct TaskCollection {
    char* name;
    Task** tasks;
    size_t tasks_count;
};

struct MigrationReport {
    char* project_name;
    int tasks_migrated;
    int errors;
    char** warnings;
    size_t warnings_count;
    time_t completed_at;
};

struct ProjectStats {
    char* project_name;
    int total_tasks;
    int completed_tasks;
    int pending_tasks;
    int overdue_tasks;
    float completion_rate;
};

struct SearchAnalytics {
    // Analytics implementation details
};

struct SearchResults {
    char* query;
    void** results;
    size_t results_count;
    time_t search_time;
};

struct HierarchicalCluster {
    int depth;
    ClusteringResult** clusters;
    size_t clusters_count;
};

struct IdeaStatistics {
    int total_ideas;
    int breakthrough_ideas;
    int team_shared;
    int private_ideas;
};

struct MemoryStatistics {
    int total_memories;
    int important_memories;
    int long_term_memories;
    int recent_memories;
};

struct SummaryStatistics {
    // Summary statistics implementation details
};

struct TagStatistics {
    int total_tags;
    int most_used_tag_count;
    char* most_used_tag;
    int average_usage_per_tag;
};

struct DriftReport {
    char* content_id;
    float drift_score;
    char** changed_fields;
    size_t changed_fields_count;
    time_t last_updated;
};

struct ValidationReport {
    int total_items;
    int valid_items;
    int invalid_items;
    char** errors;
    size_t errors_count;
};

struct ModelComparisonResult {
    char* model_name;
    float accuracy;
    float speed;
    int memory_usage;
};

struct SimilarityGraph {
    // Graph implementation details
};

struct LabeledCluster {
    int cluster_id;
    char* label;
    char** items;
    size_t items_count;
};

struct PerformanceMetrics {
    double search_time_ms;
    int results_count;
    float accuracy_score;
};

struct ToolResult {
    bool success;
    char* output;
    char* error;
    unsigned long execution_time_ms;
    void* metadata;
    void* recovery_context;
};

struct ToolParameter {
    char* name;
    char* type;
    char* description;
    bool required;
};

struct ToolDefinition {
    char* name;
    char* description;
    char* category;
    ToolParameter** parameters;
    size_t parameters_count;
    ResourceLock** locks;
    size_t locks_count;
};

struct ResourceLock {
    char* resource_id;
    char* lock_type;
    time_t acquired_at;
};

struct ToolRegistry {
    // Registry implementation details
};

struct CodeChunk {
    char* id;
    char* file_path;
    char* content;
    TodoziChunkingLevel level;
    char** dependencies;
    size_t dependencies_count;
    bool validated;
    bool completed;
    time_t created_at;
    time_t updated_at;
};

struct ProjectState {
    char** completed_modules;
    size_t completed_modules_count;
    char** pending_modules;
    size_t pending_modules_count;
    int max_lines;
};

struct ContextWindow {
    char** imports;
    size_t imports_count;
    char** function_signatures;
    size_t function_signatures_count;
    char** error_patterns;
    size_t error_patterns_count;
};

struct ErrorManager {
    // Error manager implementation details
};

struct ReminderManager {
    // Reminder manager implementation details
};

struct Reminder {
    char* id;
    char* title;
    time_t remind_at;
    char* priority;
    bool completed;
    time_t created_at;
};

struct TagManager {
    // Tag manager implementation details
};

struct TagSearchEngine {
    TagManager* tag_manager;
};

struct TagSearchQuery {
    char* name_contains;
    char* description_contains;
    char* category;
    char* color;
    int* min_usage;
    int* max_usage;
    TagSortBy* sort_by;
    int* limit;
};

struct DisplayConfig {
    bool show_priority;
    bool show_project;
    bool show_tags;
    int max_width;
    char* color_scheme;
};

struct TodoziApp {
    TodoziEmbeddingService* embedding_service;
    DisplayConfig* config;
};

struct TaskEditor {
    // Task editor implementation details
};

struct ProcessedAction {
    char* id;
    char* action_type;
    char* description;
    time_t timestamp;
    bool success;
    char* result;
};

struct ChecklistItem {
    char* id;
    char* content;
    char* priority;
    bool completed;
    time_t created_at;
    char* source;
};

struct CodeGenerationGraph {
    // Graph implementation details
};

struct SharedTodoziState {
    // Shared state implementation details
};

struct TodoziProcessorState {
    ProcessedAction** recent_actions;
    size_t recent_actions_count;
    ChecklistItem** checklist_items;
    size_t checklist_items_count;
};

struct TdzCommand {
    char* command;
    char** args;
    size_t args_count;
};

struct Helix {
    char* file_path;
    // Helix implementation details
};

struct DnaValue {
    int type; // 0 = String, 1 = Boolean, 2 = Integer
    union {
        char* string_value;
        bool bool_value;
        int int_value;
    } value;
};

// Function declarations
TodoziErrorCode todozi_init();
TodoziErrorCode todozi_init_with_auto_registration();
TodoziErrorCode todozi_tdzfp(bool* result);
TodoziErrorCode todozi_begin();
TodoziErrorCode todozi_get_tdz_api_key(char** api_key);
TodoziErrorCode todozi_ensure_todozi_initialized();
TodoziErrorCode todozi_find_tdz(const char* str, char** result);
TodoziErrorCode todozi_create_task(const char* action, TodoziPriority priority, const char* project, const char* time_str, const char* context, Task** task);
TodoziErrorCode todozi_search_tasks(const char* query, bool semantic, size_t limit, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_update_task_status(const char* task_id, TodoziStatus status);
TodoziErrorCode todozi_extract_tasks(const char* content, const char* context, char*** task_actions, size_t* actions_count);
TodoziErrorCode todozi_plan_tasks(const char* goal, const char* complexity, const char* timeline, const char* context, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_list_tasks(Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_get_task(const char* task_id, Task** task);
TodoziErrorCode todozi_delete_task(const char* task_id);
TodoziErrorCode todozi_create_memory(const char* moment, const char* meaning, const char* reason, Task** task);
TodoziErrorCode todozi_create_idea(const char* idea, const char* context, Task** task);
TodoziErrorCode todozi_process_chat(const char* message, const char* user_id, ChatContent** content);
TodoziErrorCode todozi_storage_get(Storage** storage);
TodoziErrorCode todozi_embedding_service_get(TodoziEmbeddingService** service);
TodoziErrorCode todozi_search_with_filters(TaskFilters* filters, size_t limit, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_update_task_full(const char* task_id, TaskUpdate* updates);
Task* todozi_sample_task();
TaskFilters* todozi_default_filters();
TaskUpdate* todozi_default_update();
TodoziEmbeddingConfig* todozi_embedding_config();
TodoziErrorCode todozi_create_storage(Storage** storage);
TodoziErrorCode todozi_create_embedding_service(TodoziEmbeddingService** service);
TaskFilters* todozi_create_filters();
TaskUpdate* todozi_create_update();
TodoziErrorCode todozi_extract_task_actions(const char* content, char*** actions, size_t* actions_count);
TodoziErrorCode todozi_plan_task_actions(const char* goal, char*** actions, size_t* actions_count);
TodoziErrorCode todozi_quick_task(const char* action, Task** task);
TodoziErrorCode todozi_find_tasks(const char* query, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_find_tasks_ai(const char* query, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_all_tasks(Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_complete_task(const char* task_id);
TodoziErrorCode todozi_start_task(const char* task_id);
TodoziErrorCode todozi_chat(const char* message, ChatContent** content);
TodoziErrorCode todozi_remember(const char* moment, const char* meaning, Task** task);
TodoziErrorCode todozi_ideate(const char* idea, Task** task);
TodoziErrorCode todozi_create_task_filters(const char* project, const char* status, const char* priority, const char* assignee, const char* tags, const char* search, TaskFilters** filters);
TodoziErrorCode todozi_create_task_update(const char* action, const char* priority, const char* status, const char* project, TaskUpdate** update);
TodoziErrorCode todozi_complete_task_in_project(const char* task_id);
TodoziErrorCode todozi_add(const char* action);
TodoziErrorCode todozi_analyze_code_quality(float* features, size_t features_count, float* result);
TodoziErrorCode todozi_api(const char* message);
const char* todozi_as_str();
TodoziErrorCode todozi_auto_label_clusters(ClusteringResult** clusters, size_t clusters_count, LabeledCluster*** labeled_clusters, size_t* labeled_count);
TodoziErrorCode todozi_backup_embeddings(const char* backup_path, char** result_path);
TodoziErrorCode todozi_breakthrough_percentage(double* percentage);
TodoziErrorCode todozi_build_similarity_graph(float threshold, SimilarityGraph** graph);
TodoziErrorCode todozi_calculate_diversity(char** content_ids, size_t content_ids_count, float* diversity);
TodoziErrorCode todozi_capabilities(char** capabilities, size_t capabilities_count);
TodoziErrorCode todozi_category(const char* category);
TodoziErrorCode todozi_check_folder_structure(bool* result);
TodoziErrorCode todozi_cleanup_expired(size_t* count);
TodoziErrorCode todozi_cleanup_legacy();
TodoziErrorCode todozi_cli_fix_consistency();
TodoziErrorCode todozi_color(const char* color);
TodoziErrorCode todozi_compare_models(const char* text, char** model_aliases, size_t model_aliases_count, ModelComparisonResult** result);
TodoziErrorCode todozi_completion_rate(double* rate);
TodoziErrorCode todozi_config_get(Config** config);
TodoziErrorCode todozi_content(const char* content);
TodoziErrorCode todozi_context(const char* context);
TodoziErrorCode todozi_craft_embedding(float* features, size_t features_count, float** result, size_t* result_size);
TodoziErrorCode todozi_create(const char* name, const char* description, char** result);
TodoziErrorCode todozi_create_advanced_todozi_tools(void* todozi, char*** tool_names, size_t* tools_count);
TodoziErrorCode todozi_create_architect_agent(Agent** agent);
TodoziErrorCode todozi_create_backup(char** backup_path);
TodoziErrorCode todozi_create_coder(Agent** agent);
TodoziErrorCode todozi_create_comrad_agent(Agent** agent);
TodoziErrorCode todozi_create_custom_agent(const char* id, const char* name, const char* description, char** capabilities, size_t capabilities_count, char** specializations, size_t specializations_count, const char* category, const char* author, char** agent_id);
TodoziErrorCode todozi_create_default_agents();
TodoziErrorCode todozi_create_designer_agent(Agent** agent);
TodoziErrorCode todozi_create_detective_agent(Agent** agent);
TodoziErrorCode todozi_create_devops_agent(Agent** agent);
TodoziErrorCode todozi_create_embedding_version(const char* content_id, const char* version_label, char** version_id);
TodoziErrorCode todozi_create_error(Error* error, char** error_id);
TodoziErrorCode todozi_create_error_result(const char* error_msg, unsigned long execution_time_ms, TodoziErrorType error_type, void* metadata, ToolResult** result);
TodoziErrorCode todozi_create_finisher_agent(Agent** agent);
TodoziErrorCode todozi_create_framer_agent(Agent** agent);
TodoziErrorCode todozi_create_friend_agent(Agent** agent);
TodoziErrorCode todozi_create_grok_level_todozi_tools(void* todozi, char*** tool_names, size_t* tools_count);
TodoziErrorCode todozi_create_hoarder_agent(Agent** agent);
TodoziErrorCode todozi_create_investigator_agent(Agent** agent);
TodoziErrorCode todozi_create_mason_agent(Agent** agent);
TodoziErrorCode todozi_create_nerd_agent(Agent** agent);
TodoziErrorCode todozi_create_nun_agent(Agent** agent);
TodoziErrorCode todozi_create_overlord_agent(Agent** agent);
TodoziErrorCode todozi_create_party_agent(Agent** agent);
TodoziErrorCode todozi_create_planner_agent(Agent** agent);
TodoziErrorCode todozi_create_recycler_agent(Agent** agent);
TodoziErrorCode todozi_create_skeleton_agent(Agent** agent);
TodoziErrorCode todozi_create_snitch_agent(Agent** agent);
TodoziErrorCode todozi_create_success_result(const char* output, unsigned long execution_time_ms, void* metadata, ToolResult** result);
TodoziErrorCode todozi_create_tdz_content_processor_tool(SharedTodoziState* state, void** tool);
TodoziErrorCode todozi_create_tester_agent(Agent** agent);
TodoziErrorCode todozi_create_todozi_tools(void* todozi, char*** tool_names, size_t* tools_count);
TodoziErrorCode todozi_create_todozi_tools_with_embedding(void* todozi, TodoziEmbeddingService* embedding_service, char*** tool_names, size_t* tools_count);
TodoziErrorCode todozi_create_tool_definition_with_locks(const char* name, const char* description, const char* category, ToolParameter** parameters, size_t parameters_count, ResourceLock** locks, size_t locks_count, ToolDefinition** definition);
TodoziErrorCode todozi_create_tuner_agent(Agent** agent);
TodoziErrorCode todozi_create_writer_agent(Agent** agent);
TodoziErrorCode todozi_critical_percentage(double* percentage);
TodoziErrorCode todozi_deactivate_key(const char* key_id);
TodoziErrorCode todozi_delete_agent_assignment(const char* assignment_id);
TodoziErrorCode todozi_delete_code_chunk(const char* chunk_id);
TodoziErrorCode todozi_delete_error(const char* error_id);
TodoziErrorCode todozi_delete_feeling(const char* feeling_id);
TodoziErrorCode todozi_delete_idea(const char* idea_id);
TodoziErrorCode todozi_delete_memory(const char* memory_id);
TodoziErrorCode todozi_delete_project_task_container(const char* container_id);
TodoziErrorCode todozi_delete_task_from_project(const char* task_id);
TodoziErrorCode todozi_delete_training_data(const char* training_id);
TodoziErrorCode todozi_description(const char* description);
TodoziErrorCode todozi_display_task(const char* task_id, char** result);
TodoziErrorCode todozi_display_tasks(char** result);
TodoziErrorCode todozi_dry_run(const char* command, char** result);
TodoziErrorCode todozi_embed_idea(const char* idea_id, float** embedding, size_t* size);
TodoziErrorCode todozi_embed_memory(const char* memory_id, float** embedding, size_t* size);
TodoziErrorCode todozi_embed_tag(const char* tag_id, float** embedding, size_t* size);
TodoziErrorCode todozi_encode(const char* text, char** encoded);
TodoziErrorCode todozi_end_queue_session(const char* session_id);
TodoziErrorCode todozi_end_session(const char* session_id);
TodoziErrorCode todozi_ensure_folder_structure(bool* result);
TodoziErrorCode todozi_error_set(const char* error);
const char* todozi_example();
TodoziErrorCode todozi_example_usage(char** result);
TodoziErrorCode todozi_execute_task(const char* task_id, char** result);
TodoziErrorCode todozi_execute_tdz_command(const char* command, char** result);
TodoziErrorCode todozi_execute_todozi_tool_delegated(const char* params, void** result);
TodoziErrorCode todozi_explain_search_result(const char* result_id, char** explanation);
TodoziErrorCode todozi_export_diagnostics(const char* path, char** result);
TodoziErrorCode todozi_export_embedded_tasks_hlx(const char* path, char** result);
TodoziErrorCode todozi_export_for_fine_tuning(const char* path, char** result);
TodoziErrorCode todozi_filtered_semantic_search(const char* query, SearchFilters* filters, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_find_best_agent(const char* task_description, Agent** agent);
TodoziErrorCode todozi_find_cross_content_relationships(char** content_ids, size_t content_ids_count, char*** relationships, size_t* relationships_count);
TodoziErrorCode todozi_find_outliers(char** content_ids, size_t content_ids_count, char*** outliers, size_t* outliers_count);
TodoziErrorCode todozi_find_similar_tags(const char* tag_id, char*** similar_tags, size_t* tags_count);
TodoziErrorCode todozi_get_agent(const char* agent_id, Agent** agent);
TodoziErrorCode todozi_get_all_agents(Agent*** agents, size_t* agents_count);
TodoziErrorCode todozi_get_available_agents(Agent*** agents, size_t* agents_count);
TodoziErrorCode todozi_get_code_chunk(const char* chunk_id, CodeChunk** chunk);
TodoziErrorCode todozi_get_error(const char* error_id, Error** error);
TodoziErrorCode todozi_get_feeling(const char* feeling_id, Feeling** feeling);
TodoziErrorCode todozi_get_idea(const char* idea_id, Idea** idea);
TodoziErrorCode todozi_get_memory(const char* memory_id, Memory** memory);
TodoziErrorCode todozi_get_project_task_container(const char* container_id, ProjectTaskContainer** container);
TodoziErrorCode todozi_get_training_data(const char* training_id, TrainingData** training_data);
TodoziErrorCode todozi_hierarchical_cluster(char** content_ids, size_t content_ids_count, size_t depth, HierarchicalCluster** cluster);
TodoziErrorCode todozi_idea_statistics(IdeaStatistics** stats);
TodoziErrorCode todozi_import_embeddings(const char* path, size_t* imported_count);
TodoziErrorCode todozi_import_project(const char* path, char** project_name);
TodoziErrorCode todozi_initialize_embedding_service();
TodoziErrorCode todozi_is_registered(bool* registered);
TodoziErrorCode todozi_list_agent_assignments(AgentAssignment*** assignments, size_t* assignments_count);
TodoziErrorCode todozi_list_code_chunks(CodeChunk*** chunks, size_t* chunks_count);
TodoziErrorCode todozi_list_errors(Error*** errors, size_t* errors_count);
TodoziErrorCode todozi_list_feelings(Feeling*** feelings, size_t* feelings_count);
TodoziErrorCode todozi_list_ideas(Idea*** ideas, size_t* ideas_count);
TodoziErrorCode todozi_list_memories(Memory*** memories, size_t* memories_count);
TodoziErrorCode todozi_list_project_task_containers(ProjectTaskContainer*** containers, size_t* containers_count);
TodoziErrorCode todozi_list_projects(char*** project_names, size_t* projects_count);
TodoziErrorCode todozi_list_training_data(TrainingData*** training_data, size_t* training_data_count);
TodoziErrorCode todozi_list_all_agent_assignments(AgentAssignment*** assignments, size_t* assignments_count);
TodoziErrorCode todozi_list_backlog_items(QueueItem*** items, size_t* items_count);
TodoziErrorCode todozi_list_backups(char*** backups, size_t* backups_count);
TodoziErrorCode todozi_list_complete_items(QueueItem*** items, size_t* items_count);
TodoziErrorCode todozi_list_tasks_across_projects(TaskFilters* filters, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_list_tasks_in_project(const char* project_name, TaskFilters* filters, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_load(const char* model_name, const char* device);
TodoziErrorCode todozi_load_additional_model(const char* model_name, const char* model_alias);
TodoziErrorCode todozi_load_agent(const char* agent_id, Agent** agent);
TodoziErrorCode todozi_load_agent_assignment(const char* agent_id, const char* task_id, AgentAssignment** assignment);
TodoziErrorCode todozi_load_agents();
TodoziErrorCode todozi_load_api_key_collection(ApiKeyCollection** collection);
TodoziErrorCode todozi_load_api_keys();
TodoziErrorCode todozi_load_code_chunk(const char* chunk_id, CodeChunk** chunk);
TodoziErrorCode todozi_load_config(Config** config);
TodoziErrorCode todozi_load_error(const char* error_id, Error** error);
TodoziErrorCode todozi_load_extended_data();
TodoziErrorCode todozi_load_feeling(const char* id, Feeling** feeling);
TodoziErrorCode todozi_load_idea(const char* idea_id, Idea** idea);
TodoziErrorCode todozi_load_memory(const char* memory_id, Memory** memory);
TodoziErrorCode todozi_load_project_task_container(const char* project_name, ProjectTaskContainer** container);
TodoziErrorCode todozi_load_project_task_container_by_hash(const char* project_hash, ProjectTaskContainer** container);
TodoziErrorCode todozi_load_queue_collection(QueueCollection** collection);
TodoziErrorCode todozi_load_task_collection(const char* collection_name, TaskCollection** collection);
TodoziErrorCode todozi_memory_statistics(MemoryStatistics** stats);
TodoziErrorCode todozi_migrate_project(const char* project_name, MigrationReport** report);
TodoziErrorCode todozi_project_statistics(const char* project_name, ProjectStats** stats);
TodoziErrorCode todozi_register_with_server(const char* server_url, RegistrationInfo** registration);
TodoziErrorCode todozi_registration_status(RegistrationInfo** registration);
TodoziErrorCode todozi_search_analytics(SearchAnalytics** analytics);
TodoziErrorCode todozi_search_results(const char* query, SearchResults** results);
TodoziErrorCode todozi_semantic_search(const char* query, size_t* limit, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_start_queue_session(const char* description, char** session_id);
TodoziErrorCode todozi_summary_statistics(SummaryStatistics** stats);
TodoziErrorCode todozi_tag_statistics(TagStatistics** stats);
TodoziErrorCode todozi_update_agent(const char* agent_id, AgentUpdate* updates);
TodoziErrorCode todozi_update_idea(const char* idea_id, IdeaUpdate* updates);
TodoziErrorCode todozi_update_memory(const char* memory_id, MemoryUpdate* updates);
TodoziErrorCode todozi_validate_commands(TdzCommand** commands, size_t commands_count, char*** errors, size_t* errors_count);
TodoziErrorCode todozi_validate_project(const char* project_name, ValidationReport** report);
TodoziErrorCode todozi_tool_count(size_t* count);
TodoziErrorCode todozi_total_results(size_t* count);
TodoziErrorCode todozi_to_ollama_format(char** json_result);
TodoziErrorCode todozi_to_state_string(char** state_string);
const char* todozi_title();
TodoziErrorCode todozi_to_context_string(char** context_string);
TodoziErrorCode todozi_update_task_in_project(const char* task_id, TaskUpdate* updates);
TodoziErrorCode todozi_track_embedding_drift(const char* content_id, const char* current_text, DriftReport** report);
TodoziErrorCode todozi_transform_shorthand_tags(const char* message, char** result);
const char* todozi_types();
TodoziErrorCode todozi_unregister(const char* name, bool* result);
TodoziErrorCode todozi_update(TaskUpdate* updates);
TodoziErrorCode todozi_update_agent_assignment_status(const char* agent_id, const char* task_id, const char* status);
TodoziErrorCode todozi_update_chunk_code(const char* chunk_id, const char* code);
TodoziErrorCode todozi_update_chunk_tests(const char* chunk_id, const char* tests);
TodoziErrorCode todozi_update_config(Config* config);
TodoziErrorCode todozi_update_config_with_registration(RegistrationInfo* registration);
TodoziErrorCode todozi_update_feeling(Feeling* feeling);
TodoziErrorCode todozi_update_project(Project* project);
TodoziErrorCode todozi_update_registration_api_key(const char* api_key);
TodoziErrorCode todozi_update_registration_keys(const char* api_key, const char* user_id, const char* fingerprint);
TodoziErrorCode todozi_update_task(const char* id, TaskUpdate* updates);
TodoziErrorCode todozi_validate_embeddings(ValidationReport** report);
TodoziErrorCode todozi_validate_migration(bool* result);
TodoziErrorCode todozi_validate_required_params(void* kwargs, char** required_params, size_t required_params_count, ToolResult** result);
TodoziErrorCode todozi_validate_string_param(void* value, const char* param_name, size_t min_length, size_t max_length, const char* pattern, ToolResult** result);
TodoziErrorCode todozi_validate_task_input(const char* action, const char* time, const char* priority, const char* project, const char* status, const char* assignee, const unsigned char* progress);
TodoziErrorCode todozi_validation(const char* message, void** result);
TodoziErrorCode todozi_verbose(bool verbose, void** result);
TodoziErrorCode todozi_with_action(const char* action, void** result);
TodoziErrorCode todozi_with_assignee(TodoziAssignee assignee, void** result);
TodoziErrorCode todozi_with_context(const char* context, void** result);
TodoziErrorCode todozi_with_context_notes(const char* context_notes, void** result);
TodoziErrorCode todozi_with_dependencies(char** dependencies, size_t dependencies_count, void** result);
TodoziErrorCode todozi_with_dry_run(bool dry_run, void** result);
TodoziErrorCode todozi_with_embedding_service(TodoziEmbeddingService* service, void** result);
TodoziErrorCode todozi_with_embedding_service_option(TodoziEmbeddingService* service, void** result);
TodoziErrorCode todozi_with_force(bool force, void** result);
TodoziErrorCode todozi_with_max_tokens(unsigned int max_tokens, void** result);
TodoziErrorCode todozi_with_parent_project(const char* parent_project, void** result);
TodoziErrorCode todozi_with_priority(TodoziPriority priority, void** result);
TodoziErrorCode todozi_with_progress(unsigned char progress, void** result);
TodoziErrorCode todozi_with_shared_components(void* config, void* cache, void* embedding_model, void* embedding_models, void* tag_manager, void* storage, void** result);
TodoziErrorCode todozi_with_status(TodoziStatus status, void** result);
TodoziErrorCode todozi_with_tags(char** tags, size_t tags_count, void** result);
TodoziErrorCode todozi_with_time(const char* time, void** result);
void todozi_with_temperature(float temperature, void* result);
TodoziErrorCode todozi_with_user_id(const char* user_id, void** result);
TodoziErrorCode todozi_load_tasks();
TodoziErrorCode todozi_load_training_data(const char* training_data_id, TrainingData** training_data);
TodoziErrorCode todozi_long_term_percentage(double* percentage);
TodoziErrorCode todozi_mark_chunk_completed(const char* chunk_id);
TodoziErrorCode todozi_mark_chunk_validated(const char* chunk_id);
TodoziErrorCode todozi_matches(const char* public_key, const char* private_key, bool* result);
TodoziErrorCode todozi_max_tokens(size_t* tokens);
TodoziErrorCode todozi_meaning(const char* meaning, void** result);
TodoziErrorCode todozi_migrate(MigrationReport** report);
TodoziErrorCode todozi_migrate_to_project_based(MigrationReport** report);
TodoziErrorCode todozi_moment(const char* moment, void** result);
TodoziErrorCode todozi_move_task(const char* id, const char* from_collection, const char* to_collection);
TodoziErrorCode todozi_multi_query_search(char** queries, size_t queries_count, AggregationType* aggregation, TodoziContentType** content_types, size_t content_types_count, size_t limit, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_name(const char* name, void** result);
TodoziErrorCode todozi_new_full(const char* user_id, const char* action, const char* time, TodoziPriority priority, const char* parent_project, TodoziStatus status, TodoziAssignee* assignee, char** tags, size_t tags_count, char** dependencies, size_t dependencies_count, const char* context_notes, const unsigned char* progress, void** result);
TodoziErrorCode todozi_new_idea(Idea* idea, char** idea_id);
TodoziErrorCode todozi_new_memory(Memory* memory, char** memory_id);
TodoziErrorCode todozi_new_with_hashes(const char* server_url, void** result);
TodoziErrorCode todozi_ok(const char* body, void** result);
TodoziErrorCode todozi_overdue_percentage(double* percentage);
TodoziErrorCode todozi_parse_agent_assignment_format(const char* agent_text, AgentAssignment** assignment);
TodoziErrorCode todozi_parse_chunking_format(const char* chunk_text, CodeChunk** chunk);
TodoziErrorCode todozi_parse_dependencies(const char* deps_str, char*** dependencies, size_t* dependencies_count);
TodoziErrorCode todozi_parse_error_format(const char* error_text, Error** error);
TodoziErrorCode todozi_parse_feeling_format(const char* feel_text, Feeling** feeling);
TodoziErrorCode todozi_parse_idea_format(const char* idea_text, Idea** idea);
TodoziErrorCode todozi_parse_memory_format(const char* memory_text, const char* user_id, Memory** memory);
TodoziErrorCode todozi_parse_reminder_format(const char* reminder_text, Reminder** reminder);
TodoziErrorCode todozi_parse_summary_format(const char* summary_text, void* summary);
TodoziErrorCode todozi_parse_tags(const char* tags_str, char*** tags, size_t* tags_count);
TodoziErrorCode todozi_parse_tdz_command(const char* text, TdzCommand*** commands, size_t* commands_count);
TodoziErrorCode todozi_parse_todozi_format(const char* todozi_text, Task** task);
TodoziErrorCode todozi_parse_training_data_format(const char* train_text, TrainingData** training_data);
TodoziErrorCode todozi_pending_percentage(double* percentage);
TodoziErrorCode todozi_predict_relevance(float* features, size_t features_count, float* result);
TodoziErrorCode todozi_preload_related_embeddings(const char* content_id, size_t depth);
TodoziErrorCode todozi_prepare_task_content(Task* task, char** content);
TodoziErrorCode todozi_priority_set(SummaryPriority* priority, void** result);
TodoziErrorCode todozi_private_percentage(double* percentage);
TodoziErrorCode todozi_process_chat_message(const char* message, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_process_chat_message_extended(const char* message, const char* user_id, ChatContent** content);
TodoziErrorCode todozi_process_chunking_message(const char* message, CodeChunk*** chunks, size_t* chunks_count);
TodoziErrorCode todozi_process_json_examples(const char* json_data, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_process_tdz_commands(const char* text, const char* base_url, const char* api_key, char*** results, size_t* results_count);
TodoziErrorCode todozi_process_workflow(Task** tasks, size_t tasks_count, char*** results, size_t* results_count);
TodoziErrorCode todozi_profile_search_performance(const char* query, size_t iterations, PerformanceMetrics** metrics);
TodoziErrorCode todozi_public_percentage(double* percentage);
TodoziErrorCode todozi_reason(const char* reason, void** result);
TodoziErrorCode todozi_relationships_per_tag(double* relationships);
TodoziErrorCode todozi_recommend_similar(char** based_on, size_t based_on_count, char** exclude, size_t exclude_count, size_t limit, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_remove_item(const char* id, QueueItem** item);
TodoziErrorCode todozi_remove_key(const char* user_id, ApiKey** key);
TodoziErrorCode todozi_remove_task(const char* id, Task** task);
TodoziErrorCode todozi_render(DisplayConfig* config, char** output);
TodoziErrorCode todozi_render_compact(DisplayConfig* config, char** output);
TodoziErrorCode todozi_render_detailed(DisplayConfig* config, char** output);
TodoziErrorCode todozi_run();
TodoziErrorCode todozi_run_interactive(char** result);
TodoziErrorCode todozi_sample_task_async(Task** task);
TodoziErrorCode todozi_save_agent(Agent* agent);
TodoziErrorCode todozi_save_agent_assignment(AgentAssignment* assignment);
TodoziErrorCode todozi_save_api_key_collection(ApiKeyCollection* collection);
TodoziErrorCode todozi_save_as_default(const char* model_name);
TodoziErrorCode todozi_save_code_chunk(CodeChunk* chunk);
TodoziErrorCode todozi_save_config(Config* config);
TodoziErrorCode todozi_save_error(Error* error);
TodoziErrorCode todozi_save_feeling(Feeling* feeling);
TodoziErrorCode todozi_save_idea(Idea* idea);
TodoziErrorCode todozi_remind_at(const char* task_id, time_t when);
TodoziErrorCode todozi_task_create(const char* action, char** task_id);
TodoziErrorCode todozi_urgent(const char* action, char** task_id);
TodoziErrorCode todozi_high(const char* action, char** task_id);
TodoziErrorCode todozi_low(const char* action, char** task_id);
TodoziErrorCode todozi_find(const char* query, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_ai_find(const char* query, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_done(const char* task_id);
TodoziErrorCode todozi_start_work(const char* task_id);
TodoziErrorCode todozi_all_get(Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_idea_create(const char* idea, Task** task);
TodoziErrorCode todozi_ai_create(const char* action, char** task_id);
TodoziErrorCode todozi_human_create(const char* action, char** task_id);
TodoziErrorCode todozi_collab_create(const char* action, char** task_id);
TodoziErrorCode todozi_complete_task_op(const char* task_id);
TodoziErrorCode todozi_begin_task(const char* task_id);
TodoziErrorCode todozi_delete_task_op(const char* task_id);
TodoziErrorCode todozi_get_task_op(const char* task_id, Task** task);
TodoziErrorCode todozi_list_tasks_op(Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_add_recent(const char* description);
TodoziErrorCode todozi_create_tag(const char* name, const char* description, char** tag_id);
TodoziErrorCode todozi_find_tag(const char* tag_name, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_add_to_task(const char* task_id, const char* tag);
TodoziErrorCode todozi_remove_from_task(const char* task_id, const char* tag);
TodoziErrorCode todozi_advanced_search(const char* query, Tag*** tags, size_t* tags_count);
TodoziErrorCode todozi_create_project(const char* name, const char* description);
TodoziErrorCode todozi_tasks_in_project(const char* project_name, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_delete_project_op(const char* project_name);
TodoziErrorCode todozi_important(const char* moment, const char* meaning, const char* reason, char** memory_id);
TodoziErrorCode todozi_find_memory(const char* query, Memory*** memories, size_t* memories_count);
TodoziErrorCode todozi_breakthrough(const char* idea, char** idea_id);
TodoziErrorCode todozi_find_idea(const char* query, Idea*** ideas, size_t* ideas_count);
TodoziErrorCode todozi_add_to_queue(const char* task_name, const char* description, char** item_id);
TodoziErrorCode todozi_list_queue_items(QueueItem*** items, size_t* items_count);
TodoziErrorCode todozi_list_queue_items_by_status(TodoziQueueStatus status, QueueItem*** items, size_t* items_count);
TodoziErrorCode todozi_backlog(QueueItem*** items, size_t* items_count);
TodoziErrorCode todozi_active(QueueItem*** items, size_t* items_count);
TodoziErrorCode todozi_tdz_find(const char* query, char** result);
TodoziErrorCode todozi_ai_search(const char* query, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_keyword_search(const char* query, char** result);
TodoziErrorCode todozi_ai_tasks(const char* query, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_keyword_tasks(const char* query, Task*** tasks, size_t* tasks_count);
TodoziErrorCode todozi_similar_tasks(const char* task_id, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_smart(const char* query, char** result);
TodoziErrorCode todozi_fast(const char* query, char** result);
TodoziErrorCode todozi_deep(const char* query, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_embed_text(const char* text, float** embedding, size_t* size);
TodoziErrorCode todozi_find_similar(const char* query, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_find_ai_tasks(const char* query, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_cluster_content(ClusteringResult*** results, size_t* results_count);
TodoziErrorCode todozi_stats(char** result);
TodoziErrorCode todozi_embed_task(const char* task_id, char** result);
TodoziErrorCode todozi_quick_stats(char** result);
TodoziErrorCode todozi_detailed_stats(char** result);
TodoziErrorCode todozi_add_task_to_project(Task* task);
TodoziErrorCode todozi_add_item(const char* content);
TodoziErrorCode todozi_add_chunk(char* chunk_id, char* level, char** deps, size_t deps_count);
TodoziErrorCode todozi_add_completed_module(char* module);
TodoziErrorCode todozi_add_dependency(char* dep);
TodoziErrorCode todozi_add_error_pattern(char* pattern);
TodoziErrorCode todozi_add_function_signature(char* name, char* signature);
TodoziErrorCode todozi_add_import(char* import_stmt);
TodoziErrorCode todozi_add_pending_module(char* module);
TodoziErrorCode todozi_do_it(const char* what, const char* action, TodoziPriority* priority, const char* project, const char* time, const char* context, char** task_id);
TodoziErrorCode todozi_find_task(const char* what, char** result);
TodoziErrorCode todozi_see_all(char** result);
TodoziErrorCode todozi_has_results(bool* result);
TodoziErrorCode todozi_has_specialization(const char* specialization, bool* result);
TodoziErrorCode todozi_has_tool(const char* tool_name, bool* result);
TodoziErrorCode todozi_hash_project_name(const char* project_name, char** hash);
TodoziErrorCode todozi_hierarchical_clustering(TodoziContentType* content_types, size_t content_types_count, size_t max_depth, HierarchicalCluster*** clusters, size_t* clusters_count);
TodoziErrorCode todozi_high_priority_percentage(double* percentage);
TodoziErrorCode todozi_hybrid_search(const char* query, char** keywords, size_t keywords_count, TodoziContentType** content_types, size_t content_types_count, float semantic_weight, size_t limit, SimilarityResult*** results, size_t* results_count);
TodoziErrorCode todozi_importance(TodoziIdeaImportance importance);
TodoziErrorCode todozi_initialize_grok_level_todozi_system(void** result);
TodoziErrorCode todozi_initialize_grok_level_todozi_system_with_embedding(bool enable_embeddings, void** result);
TodoziErrorCode todozi_initialize_tdz_content_processor(SharedTodoziState** state);
TodoziErrorCode todozi_interactive_create_task(Task** task);
TodoziErrorCode todozi_io(char* message);
TodoziErrorCode todozi_is_active(bool* result);
TodoziErrorCode todozi_is_admin(const char* public_key, const char* private_key, bool* result);
TodoziErrorCode todozi_is_available(bool* result);
TodoziErrorCode todozi_is_backlog(bool* result);
TodoziErrorCode todozi_is_complete(bool* result);
TodoziErrorCode todozi_is_completed(bool* result);
TodoziErrorCode todozi_is_empty(bool* result);
TodoziErrorCode todozi_is_overdue(bool* result);
TodoziErrorCode todozi_smart_search(const char* query, char** result);
TodoziErrorCode todozi_handle_memory_command(MemoryCommands* command);
TodoziErrorCode todozi_handle_project_command(ProjectCommands* command);
TodoziErrorCode todozi_handle_queue_command(QueueCommands* command);
TodoziErrorCode todozi_handle_search_all_command(Commands* command);
TodoziErrorCode todozi_handle_search_command(SearchCommands* command);
TodoziErrorCode todozi_handle_server_command(ServerCommands* command);
TodoziErrorCode todozi_handle_show_command(ShowCommands* command);
TodoziErrorCode todozi_handle_stats_command(StatsCommands* command);
TodoziErrorCode todozi_handle_strategy_command(const char* content, const char* file, const char* output_format, bool human);
TodoziErrorCode todozi_handle_train_command(TrainingCommands* command);
TodoziErrorCode todozi_handle_update_command(const char* id, const char* action, const char* time, const char* priority, const char* project, const char* status, const char* assignee, const char* tags, const char* dependencies, const char* context, const unsigned char* progress);
TodoziErrorCode todozi_has_capability(const char* capability, bool* result);

// Implementation stubs
TodoziErrorCode todozi_init() {
    // Implementation would go here
    return TODOZI_OK; // Success
}

TodoziErrorCode todozi_init_with_auto_registration() {
    // Implementation would go here
    return TODOZI_OK; // Success
}

TodoziErrorCode todozi_tdzfp(bool* result) {
    // Implementation would go here
    *result = true;
    return TODOZI_OK; // Success
}

TodoziErrorCode todozi_begin() {
    // Implementation would go here
    return TODOZI_OK; // Success
}

TodoziErrorCode todozi_get_tdz_api_key(char** api_key) {
    if (!api_key) {
        return TODOZI_ERR_INVALID_ARG;
    }

    // TODO: Implement UUID generation when linking issues are resolved
    *api_key = strdup("temp-api-key-no-uuid-support-yet");
    if (!*api_key) {
        return TODOZI_ERR_MEMORY;
    }

    return TODOZI_OK;
}

TodoziErrorCode todozi_ensure_todozi_initialized() {
    // Implementation would go here
    return TODOZI_OK; // Success
}

TodoziErrorCode todozi_find_tdz(const char* str, char** result) {
    if (!result) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    const char* home = getenv("HOME");
    if (!home) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    size_t home_len = strlen(home);
    size_t base_len = strlen("/.todozi");
    size_t total_len;
    
    if (str) {
        size_t str_len = strlen(str);
        // Check for integer overflow
        if (home_len > SIZE_MAX - base_len - 1 - str_len - 1) {
            return TODOZI_ERR_INVALID_ARG;
        }
        total_len = home_len + base_len + 1 + str_len + 1;
    } else {
        if (home_len > SIZE_MAX - base_len - 1) {
            return TODOZI_ERR_INVALID_ARG;
        }
        total_len = home_len + base_len + 1;
    }
    
    *result = malloc(total_len);
    if (!*result) {
        return TODOZI_ERR_MEMORY;
    }
    
    if (str) {
        int written = snprintf(*result, total_len, "%s/.todozi/%s", home, str);
        if (written < 0 || (size_t)written >= total_len) {
            free(*result);
            *result = NULL;
            return TODOZI_ERR_INVALID_ARG;
        }
    } else {
        int written = snprintf(*result, total_len, "%s/.todozi", home);
        if (written < 0 || (size_t)written >= total_len) {
            free(*result);
            *result = NULL;
            return TODOZI_ERR_INVALID_ARG;
        }
    }
    
    return TODOZI_OK;
}

TodoziErrorCode todozi_create_task(const char* action, TodoziPriority priority, const char* project, const char* time_str, const char* context, Task** task) {
    if (!task) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    *task = calloc(1, sizeof(Task));
    if (!*task) {
        return TODOZI_ERR_MEMORY;
    }
    
    // Get current time
    time_t now = time(NULL);
    
    // Initialize task fields - TODO: Use UUID when linking issues are resolved
    char uuid_str[37];
    sprintf(uuid_str, "temp-task-id-%ld", (long)now);

    // Allocate and copy strings with error handling
    (*task)->id = strdup(uuid_str);
    if (!(*task)->id) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    
    (*task)->user_id = strdup("external_app");
    if (!(*task)->user_id) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    
    (*task)->action = strdup(action ? action : "");
    if (!(*task)->action) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    
    (*task)->time = strdup(time_str ? time_str : "ASAP");
    if (!(*task)->time) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    
    (*task)->priority = priority;
    
    (*task)->parent_project = strdup(project ? project : "external_apps");
    if (!(*task)->parent_project) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    
    (*task)->status = TODOZI_STATUS_TODO;
    (*task)->assignee = TODOZI_ASSIGNEE_HUMAN;
    
    (*task)->tags = malloc(sizeof(char*) * 1);
    if (!(*task)->tags) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    (*task)->tags[0] = strdup("external");
    if (!(*task)->tags[0]) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    (*task)->tags_count = 1;
    
    (*task)->dependencies = NULL;
    (*task)->dependencies_count = 0;
    
    if (context) {
        (*task)->context_notes = strdup(context);
        if (!(*task)->context_notes) {
            free_task(*task);
            *task = NULL;
            return TODOZI_ERR_MEMORY;
        }
    } else {
        (*task)->context_notes = NULL;
    }
    
    (*task)->progress = malloc(sizeof(int));
    if (!(*task)->progress) {
        free_task(*task);
        *task = NULL;
        return TODOZI_ERR_MEMORY;
    }
    *(*task)->progress = 0;
    
    (*task)->created_at = now;
    (*task)->updated_at = now;
    (*task)->embedding_vector = NULL;
    (*task)->embedding_size = 0;
    
    return TODOZI_OK;
}

// Additional function implementations would follow the same pattern
// For brevity, only a few key functions are fully implemented

TodoziErrorCode todozi_search_tasks(const char* query, bool semantic, size_t limit, Task*** tasks, size_t* tasks_count) {
    // Implementation would go here
    *tasks_count = 0;
    *tasks = NULL;
    return TODOZI_OK;
}

TodoziErrorCode todozi_update_task_status(const char* task_id, TodoziStatus status) {
    // Implementation would go here
    return TODOZI_OK;
}

TodoziErrorCode todozi_extract_tasks(const char* content, const char* context, char*** task_actions, size_t* actions_count) {
    // Implementation would go here
    *actions_count = 0;
    *task_actions = NULL;
    return TODOZI_OK;
}

TodoziErrorCode todozi_plan_tasks(const char* goal, const char* complexity, const char* timeline, const char* context, Task*** tasks, size_t* tasks_count) {
    // Implementation would go here
    *tasks_count = 0;
    *tasks = NULL;
    return TODOZI_OK;
}

TodoziErrorCode todozi_list_tasks(Task*** tasks, size_t* tasks_count) {
    // Implementation would go here
    *tasks_count = 0;
    *tasks = NULL;
    return TODOZI_OK;
}

TodoziErrorCode todozi_get_task(const char* task_id, Task** task) {
    // Implementation would go here
    *task = NULL;
    return TODOZI_OK;
}

TodoziErrorCode todozi_delete_task(const char* task_id) {
    // Implementation would go here
    return TODOZI_OK;
}

TodoziErrorCode todozi_create_memory(const char* moment, const char* meaning, const char* reason, Task** task) {
    if (!moment || !meaning || !task) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    size_t action_len = strlen("Memory: ") + strlen(moment) + strlen(" - ") + strlen(meaning) + 1;
    // Check for integer overflow
    if (action_len < strlen("Memory: ") || action_len < strlen(moment) || action_len < strlen(meaning)) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    char* action = malloc(action_len);
    if (!action) {
        return TODOZI_ERR_MEMORY;
    }
    
    int written = snprintf(action, action_len, "Memory: %s - %s", moment, meaning);
    if (written < 0 || (size_t)written >= action_len) {
        free(action);
        return TODOZI_ERR_INVALID_ARG;
    }
    
    TodoziErrorCode result = todozi_create_task(action, TODOZI_PRIORITY_LOW, NULL, "Long-term", reason, task);
    free(action);
    
    if (result == TODOZI_OK && *task) {
        (*task)->status = TODOZI_STATUS_DONE;
        if ((*task)->progress) {
            *(*task)->progress = 100;
        }
    }
    
    return result;
}

TodoziErrorCode todozi_create_idea(const char* idea, const char* context, Task** task) {
    if (!idea || !task) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    size_t action_len = strlen("Idea: ") + strlen(idea) + 1;
    // Check for integer overflow
    if (action_len < strlen("Idea: ") || action_len < strlen(idea)) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    char* action = malloc(action_len);
    if (!action) {
        return TODOZI_ERR_MEMORY;
    }
    
    int written = snprintf(action, action_len, "Idea: %s", idea);
    if (written < 0 || (size_t)written >= action_len) {
        free(action);
        return TODOZI_ERR_INVALID_ARG;
    }
    
    TodoziErrorCode result = todozi_create_task(action, TODOZI_PRIORITY_LOW, NULL, "Future consideration", context, task);
    free(action);
    
    return result;
}

TodoziErrorCode todozi_process_chat(const char* message, const char* user_id, ChatContent** content) {
    if (!message || !content) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    *content = calloc(1, sizeof(ChatContent));
    if (!*content) {
        return TODOZI_ERR_MEMORY;
    }
    
    (*content)->response = strdup("Processed chat message");
    if (!(*content)->response) {
        free(*content);
        *content = NULL;
        return TODOZI_ERR_MEMORY;
    }
    
    (*content)->tasks = NULL;
    (*content)->tasks_count = 0;
    
    return TODOZI_OK;
}

TodoziErrorCode todozi_storage_get(Storage** storage) {
    if (!storage) {
        return TODOZI_ERR_INVALID_ARG;
    }
    *storage = calloc(1, sizeof(Storage));
    return *storage ? TODOZI_OK : TODOZI_ERR_MEMORY;
}

TodoziErrorCode todozi_embedding_service_get(TodoziEmbeddingService** service) {
    if (!service) {
        return TODOZI_ERR_INVALID_ARG;
    }
    *service = calloc(1, sizeof(TodoziEmbeddingService));
    return *service ? TODOZI_OK : TODOZI_ERR_MEMORY;
}

TodoziErrorCode todozi_search_with_filters(TaskFilters* filters, size_t limit, Task*** tasks, size_t* tasks_count) {
    // Implementation would go here
    *tasks_count = 0;
    *tasks = NULL;
    return TODOZI_OK;
}

TodoziErrorCode todozi_update_task_full(const char* task_id, TaskUpdate* updates) {
    // Implementation would go here
    return TODOZI_OK;
}

Task* todozi_sample_task() {
    Task* task = calloc(1, sizeof(Task));
    if (!task) {
        return NULL;
    }
    
    task->id = strdup("sample_id");
    if (!task->id) {
        free_task(task);
        return NULL;
    }
    
    task->user_id = strdup("sample_user");
    if (!task->user_id) {
        free_task(task);
        return NULL;
    }
    
    task->action = strdup("Sample task action");
    if (!task->action) {
        free_task(task);
        return NULL;
    }
    
    task->time = strdup("ASAP");
    if (!task->time) {
        free_task(task);
        return NULL;
    }
    
    task->priority = TODOZI_PRIORITY_MEDIUM;
    task->parent_project = strdup("external_apps_samples");
    if (!task->parent_project) {
        free_task(task);
        return NULL;
    }
    
    task->status = TODOZI_STATUS_TODO;
    task->assignee = TODOZI_ASSIGNEE_HUMAN;
    
    task->tags = malloc(sizeof(char*) * 1);
    if (!task->tags) {
        free_task(task);
        return NULL;
    }
    task->tags[0] = strdup("sample");
    if (!task->tags[0]) {
        free_task(task);
        return NULL;
    }
    task->tags_count = 1;
    
    task->dependencies = NULL;
    task->dependencies_count = 0;
    
    task->context_notes = strdup("Sample context");
    if (!task->context_notes) {
        free_task(task);
        return NULL;
    }
    
    task->progress = malloc(sizeof(int));
    if (!task->progress) {
        free_task(task);
        return NULL;
    }
    *task->progress = 0;
    
    task->created_at = time(NULL);
    task->updated_at = time(NULL);
    task->embedding_vector = NULL;
    task->embedding_size = 0;
    
    return task;
}

TaskFilters* todozi_default_filters() {
    TaskFilters* filters = calloc(1, sizeof(TaskFilters));
    return filters;
}

TaskUpdate* todozi_default_update() {
    TaskUpdate* update = calloc(1, sizeof(TaskUpdate));
    return update;
}

TodoziEmbeddingConfig* todozi_embedding_config() {
    TodoziEmbeddingConfig* config = calloc(1, sizeof(TodoziEmbeddingConfig));
    return config;
}

TodoziErrorCode todozi_create_storage(Storage** storage) {
    return todozi_storage_get(storage);
}

TodoziErrorCode todozi_create_embedding_service(TodoziEmbeddingService** service) {
    return todozi_embedding_service_get(service);
}

TaskFilters* todozi_create_filters() {
    return todozi_default_filters();
}

TaskUpdate* todozi_create_update() {
    return todozi_default_update();
}

TodoziErrorCode todozi_extract_task_actions(const char* content, char*** actions, size_t* actions_count) {
    return todozi_extract_tasks(content, NULL, actions, actions_count);
}

TodoziErrorCode todozi_plan_task_actions(const char* goal, char*** actions, size_t* actions_count) {
    if (!goal || !actions || !actions_count) {
        return TODOZI_ERR_INVALID_ARG;
    }
    
    Task** tasks = NULL;
    size_t tasks_count = 0;
    
    TodoziErrorCode result = todozi_plan_tasks(goal, NULL, NULL, NULL, &tasks, &tasks_count);
    if (result != TODOZI_OK) {
        return result;
    }
    
    if (tasks_count == 0) {
        *actions = NULL;
        *actions_count = 0;
        if (tasks) {
            free(tasks);
        }
        return TODOZI_OK;
    }
    
    *actions = calloc(tasks_count, sizeof(char*));
    if (!*actions) {
        // Free tasks if allocated
        if (tasks) {
            for (size_t i = 0; i < tasks_count; i++) {
                if (tasks[i]) {
                    free_task(tasks[i]);
                }
            }
            free(tasks);
        }
        return TODOZI_ERR_MEMORY;
    }
    
    *actions_count = tasks_count;
    for (size_t i = 0; i < tasks_count; i++) {
        if (tasks[i] && tasks[i]->action) {
            (*actions)[i] = strdup(tasks[i]->action);
            if (!(*actions)[i]) {
                // Cleanup on error
                for (size_t j = 0; j < i; j++) {
                    free((*actions)[j]);
                }
                free(*actions);
                *actions = NULL;
                *actions_count = 0;
                // Free tasks
                if (tasks) {
                    for (size_t j = 0; j < tasks_count; j++) {
                        if (tasks[j]) {
                            free_task(tasks[j]);
                        }
                    }
                    free(tasks);
                }
                return TODOZI_ERR_MEMORY;
            }
        } else {
            (*actions)[i] = NULL;
        }
    }
    
    // Free tasks array and tasks
    if (tasks) {
        for (size_t i = 0; i < tasks_count; i++) {
            if (tasks[i]) {
                free_task(tasks[i]);
            }
        }
        free(tasks);
    }
    
    return TODOZI_OK;
}

TodoziErrorCode todozi_quick_task(const char* action, Task** task) {
    return todozi_create_task(action, TODOZI_PRIORITY_MEDIUM, NULL, NULL, NULL, task);
}

TodoziErrorCode todozi_find_tasks(const char* query, Task*** tasks, size_t* tasks_count) {
    return todozi_search_tasks(query, false, 0, tasks, tasks_count);
}

TodoziErrorCode todozi_find_tasks_ai(const char* query, Task*** tasks, size_t* tasks_count) {
    return todozi_search_tasks(query, true, 0, tasks, tasks_count);
}

TodoziErrorCode todozi_all_tasks(Task*** tasks, size_t* tasks_count) {
    return todozi_list_tasks(tasks, tasks_count);
}

TodoziErrorCode todozi_complete_task(const char* task_id) {
    return todozi_update_task_status(task_id, TODOZI_STATUS_DONE);
}

TodoziErrorCode todozi_start_task(const char* task_id) {
    return todozi_update_task_status(task_id, TODOZI_STATUS_IN_PROGRESS);
}

TodoziErrorCode todozi_chat(const char* message, ChatContent** content) {
    return todozi_process_chat(message, "external_user", content);
}

TodoziErrorCode todozi_remember(const char* moment, const char* meaning, Task** task) {
    return todozi_create_memory(moment, meaning, "Created via external API", task);
}

TodoziErrorCode todozi_ideate(const char* idea, Task** task) {
    return todozi_create_idea(idea, NULL, task);
}

TodoziErrorCode todozi_create_task_filters(const char* project, const char* status, const char* priority, const char* assignee, const char* tags, const char* search, TaskFilters** filters) {
    *filters = malloc(sizeof(TaskFilters));
    if (!*filters) {
        return TODOZI_ERR_MEMORY;
    }
    
    (*filters)->project = project ? strdup(project) : NULL;
    (*filters)->status = status ? malloc(sizeof(TodoziStatus)) : NULL;
    if ((*filters)->status) {
        // Parse status string to enum
        if (strcmp(status, "todo") == 0) {
            *(*filters)->status = TODOZI_STATUS_TODO;
        } else if (strcmp(status, "in_progress") == 0) {
            *(*filters)->status = TODOZI_STATUS_IN_PROGRESS;
        } else if (strcmp(status, "done") == 0) {
            *(*filters)->status = TODOZI_STATUS_DONE;
        } else if (strcmp(status, "blocked") == 0) {
            *(*filters)->status = TODOZI_STATUS_BLOCKED;
        } else {
            free((*filters)->status);
            (*filters)->status = NULL;
        }
    }
    
    (*filters)->priority = priority ? malloc(sizeof(TodoziPriority)) : NULL;
    if ((*filters)->priority) {
        // Parse priority string to enum
        if (strcmp(priority, "critical") == 0) {
            *(*filters)->priority = TODOZI_PRIORITY_CRITICAL;
        } else if (strcmp(priority, "urgent") == 0) {
            *(*filters)->priority = TODOZI_PRIORITY_URGENT;
        } else if (strcmp(priority, "high") == 0) {
            *(*filters)->priority = TODOZI_PRIORITY_HIGH;
        } else if (strcmp(priority, "medium") == 0) {
            *(*filters)->priority = TODOZI_PRIORITY_MEDIUM;
        } else if (strcmp(priority, "low") == 0) {
            *(*filters)->priority = TODOZI_PRIORITY_LOW;
        } else {
            free((*filters)->priority);
            (*filters)->priority = NULL;
        }
    }
    
    (*filters)->assignee = assignee ? malloc(sizeof(TodoziAssignee)) : NULL;
    if ((*filters)->assignee) {
        // Parse assignee string to enum
        if (strcmp(assignee, "human") == 0) {
            *(*filters)->assignee = TODOZI_ASSIGNEE_HUMAN;
        } else if (strcmp(assignee, "ai") == 0) {
            *(*filters)->assignee = TODOZI_ASSIGNEE_AI;
        } else if (strcmp(assignee, "collaborative") == 0) {
            *(*filters)->assignee = TODOZI_ASSIGNEE_COLLABORATIVE;
        } else {
            free((*filters)->assignee);
            (*filters)->assignee = NULL;
        }
    }
    
    if (tags) {
        // Parse comma-separated tags
        char* tags_copy = strdup(tags);
        if (!tags_copy) {
            // Cleanup on error
            free((*filters)->project);
            free((*filters)->status);
            free((*filters)->priority);
            free((*filters)->assignee);
            free(*filters);
            *filters = NULL;
            return TODOZI_ERR_MEMORY;
        }
        
        // Count tokens first
        size_t count = 0;
        char* temp = strdup(tags);
        if (temp) {
            char* temp_token = strtok(temp, ",");
            while (temp_token) {
                count++;
                temp_token = strtok(NULL, ",");
            }
            free(temp);
        }
        
        if (count > 0) {
            (*filters)->tags = calloc(count, sizeof(char*));
            if (!(*filters)->tags) {
                free(tags_copy);
                free((*filters)->project);
                free((*filters)->status);
                free((*filters)->priority);
                free((*filters)->assignee);
                free(*filters);
                *filters = NULL;
                return TODOZI_ERR_MEMORY;
            }
            (*filters)->tags_count = count;
            
            // Parse tags
            char* token = strtok(tags_copy, ",");
            size_t idx = 0;
            while (token && idx < count) {
                // Trim leading whitespace
                while (*token == ' ') token++;
                // Trim trailing whitespace
                char* end = token + strlen(token) - 1;
                while (end > token && *end == ' ') end--;
                *(end + 1) = '\0';
                
                if (*token != '\0') {
                    (*filters)->tags[idx] = strdup(token);
                    if (!(*filters)->tags[idx]) {
                        // Cleanup on error
                        for (size_t i = 0; i < idx; i++) {
                            free((*filters)->tags[i]);
                        }
                        free((*filters)->tags);
                        free(tags_copy);
                        free((*filters)->project);
                        free((*filters)->status);
                        free((*filters)->priority);
                        free((*filters)->assignee);
                        free(*filters);
                        *filters = NULL;
                        return TODOZI_ERR_MEMORY;
                    }
                    idx++;
                }
                token = strtok(NULL, ",");
            }
            (*filters)->tags_count = idx; // Update to actual count
        } else {
            (*filters)->tags = NULL;
            (*filters)->tags_count = 0;
        }
        free(tags_copy);
    } else {
        (*filters)->tags = NULL;
        (*filters)->tags_count = 0;
    }
    
    (*filters)->search = search ? strdup(search) : NULL;
    
    return TODOZI_OK;
}

TodoziErrorCode todozi_create_task_update(const char* action, const char* priority, const char* status, const char* project, TaskUpdate** update) {
    *update = malloc(sizeof(TaskUpdate));
    if (!*update) {
        return TODOZI_ERR_MEMORY;
    }
    
    (*update)->action = action ? strdup(action) : NULL;
    (*update)->priority = priority ? malloc(sizeof(TodoziPriority)) : NULL;
    if ((*update)->priority) {
        // Parse priority string to enum
        if (strcmp(priority, "critical") == 0) {
            *(*update)->priority = TODOZI_PRIORITY_CRITICAL;
        } else if (strcmp(priority, "urgent") == 0) {
            *(*update)->priority = TODOZI_PRIORITY_URGENT;
        } else if (strcmp(priority, "high") == 0) {
            *(*update)->priority = TODOZI_PRIORITY_HIGH;
        } else if (strcmp(priority, "medium") == 0) {
            *(*update)->priority = TODOZI_PRIORITY_MEDIUM;
        } else if (strcmp(priority, "low") == 0) {
            *(*update)->priority = TODOZI_PRIORITY_LOW;
        } else {
            free((*update)->priority);
            (*update)->priority = NULL;
        }
    }
    
    (*update)->status = status ? malloc(sizeof(TodoziStatus)) : NULL;
    if ((*update)->status) {
        // Parse status string to enum
        if (strcmp(status, "todo") == 0) {
            *(*update)->status = TODOZI_STATUS_TODO;
        } else if (strcmp(status, "in_progress") == 0) {
            *(*update)->status = TODOZI_STATUS_IN_PROGRESS;
        } else if (strcmp(status, "done") == 0) {
            *(*update)->status = TODOZI_STATUS_DONE;
        } else if (strcmp(status, "blocked") == 0) {
            *(*update)->status = TODOZI_STATUS_BLOCKED;
        } else {
            free((*update)->status);
            (*update)->status = NULL;
        }
    }
    
    (*update)->parent_project = project ? strdup(project) : NULL;
    (*update)->assignee = NULL;
    (*update)->tags = NULL;
    (*update)->tags_count = 0;
    (*update)->dependencies = NULL;
    (*update)->dependencies_count = 0;
    (*update)->context_notes = NULL;
    (*update)->progress = NULL;
    
    return TODOZI_OK;
}

TodoziErrorCode todozi_complete_task_in_project(const char* task_id) {
    return todozi_update_task_status(task_id, TODOZI_STATUS_DONE);
}

TodoziErrorCode todozi_add(const char* action) {
    // This function should return an error as in the Rust version
    return TODOZI_ERR_NOT_IMPL;
}

TodoziErrorCode todozi_analyze_code_quality(float* features, size_t features_count, float* result) {
    // This function should return an error as in the Rust version
    return TODOZI_ERR_NOT_IMPL;
}

TodoziErrorCode todozi_api(const char* message) {
    // This function should return an error as in the Rust version
    return TODOZI_ERR_NOT_IMPL;
}

const char* todozi_as_str() {
    return "Done";
}

// Additional implementations would continue in the same pattern...

// Utility functions for memory management
void free_task(Task* task) {
    if (task) {
        if (task->id) free(task->id);
        if (task->user_id) free(task->user_id);
        if (task->action) free(task->action);
        if (task->time) free(task->time);
        if (task->parent_project) free(task->parent_project);
        
        if (task->tags) {
            for (size_t i = 0; i < task->tags_count; i++) {
                if (task->tags[i]) {
                    free(task->tags[i]);
                }
            }
            free(task->tags);
        }
        
        if (task->dependencies) {
            for (size_t i = 0; i < task->dependencies_count; i++) {
                if (task->dependencies[i]) {
                    free(task->dependencies[i]);
                }
            }
            free(task->dependencies);
        }
        
        if (task->context_notes) free(task->context_notes);
        if (task->progress) free(task->progress);
        if (task->embedding_vector) free(task->embedding_vector);
        free(task);
    }
}

void free_task_array(Task** tasks, size_t tasks_count) {
    if (tasks) {
        for (size_t i = 0; i < tasks_count; i++) {
            free_task(tasks[i]);
        }
        free(tasks);
    }
}

void free_task_filters(TaskFilters* filters) {
    if (filters) {
        if (filters->project) free(filters->project);
        if (filters->status) free(filters->status);
        if (filters->priority) free(filters->priority);
        if (filters->assignee) free(filters->assignee);
        if (filters->search) free(filters->search);
        
        if (filters->tags) {
            for (size_t i = 0; i < filters->tags_count; i++) {
                if (filters->tags[i]) {
                    free(filters->tags[i]);
                }
            }
            free(filters->tags);
        }
        
        free(filters);
    }
}

void free_task_update(TaskUpdate* update) {
    if (update) {
        if (update->action) free(update->action);
        if (update->priority) free(update->priority);
        if (update->status) free(update->status);
        if (update->parent_project) free(update->parent_project);
        if (update->assignee) free(update->assignee);
        
        if (update->tags) {
            for (size_t i = 0; i < update->tags_count; i++) {
                if (update->tags[i]) {
                    free(update->tags[i]);
                }
            }
            free(update->tags);
        }
        
        if (update->dependencies) {
            for (size_t i = 0; i < update->dependencies_count; i++) {
                if (update->dependencies[i]) {
                    free(update->dependencies[i]);
                }
            }
            free(update->dependencies);
        }
        
        if (update->context_notes) free(update->context_notes);
        if (update->progress) free(update->progress);
        free(update);
    }
}

static void free_chat_content(ChatContent* content) {
    if (content) {
        if (content->response) free(content->response);
        if (content->tasks) {
            free_task_array(content->tasks, content->tasks_count);
        }
        free(content);
    }
}