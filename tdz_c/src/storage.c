#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <errno.h>

// Forward declarations
typedef struct PathBuf PathBuf;
typedef struct Config Config;
typedef struct RegistrationInfo RegistrationInfo;
typedef struct TaskCollection TaskCollection;
typedef struct Project Project;
typedef struct Task Task;
typedef struct ProjectTaskContainer ProjectTaskContainer;
typedef struct Agent Agent;
typedef struct Error Error;
typedef struct TrainingData TrainingData;
typedef struct CodeChunk CodeChunk;
typedef struct Memory Memory;
typedef struct Idea Idea;
typedef struct QueueCollection QueueCollection;
typedef struct QueueItem QueueItem;
typedef struct Feeling Feeling;
typedef struct AgentAssignment AgentAssignment;
typedef struct MigrationReport MigrationReport;
typedef struct ProjectStats ProjectStats;
typedef struct TaskFilters TaskFilters;
typedef struct SemanticSearchResult SemanticSearchResult;

// Forward declaration for helper functions
static char* make_json_filename(const char* base_name);

// Basic structures (simplified representations)
struct PathBuf {
    char* path;
};

struct Config {
    RegistrationInfo* registration;
    char* version;
    char* default_project;
    int auto_backup;
    char* backup_interval;
    int ai_enabled;
    char* default_assignee;
    char* date_format;
    char* timezone;
};

struct RegistrationInfo {
    char* user_name;
    char* user_email;
    char* api_key;
    char* user_id;
    char* fingerprint;
    time_t registered_at;
    char* server_url;
};

struct TaskCollection {
    // Simplified
    int dummy;
};

struct Project {
    char* name;
    char* description;
};

struct Task {
    char* id;
    char* action;
    char* status;
    char* priority;
    char* parent_project;
    time_t created_at;
    time_t updated_at;
    char* context_notes;
    float* embedding_vector;
    int embedding_size;
};

struct ProjectTaskContainer {
    char* project_name;
    char* project_hash;
};

struct Agent {
    char* id;
    char* name;
    char* description;
    char* system_prompt;
};

struct Error {
    char* id;
    char* message;
};

struct TrainingData {
    char* id;
    // Simplified
};

struct CodeChunk {
    char* chunk_id;
    // Simplified
};

struct Memory {
    char* id;
    // Simplified
};

struct Idea {
    char* id;
    // Simplified
};

struct QueueCollection {
    // Simplified
    int dummy;
};

struct QueueItem {
    char* id;
    // Simplified
};

struct Feeling {
    char* id;
    time_t created_at;
    // Simplified
};

struct AgentAssignment {
    char* agent_id;
    char* task_id;
    // Simplified
};

struct MigrationReport {
    int tasks_found;
    int tasks_migrated;
    int projects_migrated;
    // Simplified
};

struct ProjectStats {
    char* project_name;
    int total_tasks;
    int active_tasks;
    int completed_tasks;
    int archived_tasks;
    int deleted_tasks;
};

struct TaskFilters {
    // Simplified
    int dummy;
};

struct SemanticSearchResult {
    Task task;
    float similarity_score;
    char* matched_content;
};

// Error handling
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_STORAGE,
    TODOZI_ERROR_PROJECT_NOT_FOUND,
    TODOZI_ERROR_TASK_NOT_FOUND,
    TODOZI_ERROR_VALIDATION
} TodoziError;

// Helper functions
int path_exists(const char* path) {
    struct stat st;
    return (stat(path, &st) == 0);
}

int is_directory(const char* path) {
    struct stat st;
    if (stat(path, &st) == 0) {
        return S_ISDIR(st.st_mode);
    }
    return 0;
}

static int create_directory(const char* path) {
    return mkdir(path, 0755);
}

int create_directories_recursive(const char* path) {
    if (!path) return -1;
    
    size_t path_len = strlen(path);
    if (path_len == 0) return -1;
    
    // Allocate buffer with extra space for safety
    char* temp = malloc(path_len + 2);
    if (!temp) return -1;
    
    snprintf(temp, path_len + 1, "%s", path);
    size_t len = strlen(temp);
    
    // Remove trailing slash if present
    if (len > 0 && temp[len - 1] == '/') {
        temp[len - 1] = 0;
        len--;
    }
    
    // Create directories one level at a time
    for (char* p = temp + 1; *p; p++) {
        if (*p == '/') {
            *p = 0;
            if (!is_directory(temp)) {
                if (create_directory(temp) != 0 && errno != EEXIST) {
                    free(temp);
                    return -1;
                }
            }
            *p = '/';
        }
    }
    
    // Create final directory
    if (!is_directory(temp)) {
        if (create_directory(temp) != 0 && errno != EEXIST) {
            free(temp);
            return -1;
        }
    }
    
    free(temp);
    return 0;
}

static char* join_paths(const char* base, const char* append) {
    if (!base || !append) return NULL;
    
    size_t base_len = strlen(base);
    size_t append_len = strlen(append);
    size_t total_len = base_len + append_len + 2;
    
    char* result = malloc(total_len);
    if (!result) return NULL;
    
    snprintf(result, total_len, "%s/%s", base, append);
    return result;
}

static char* get_home_directory() {
    char* home = getenv("HOME");
    if (!home) {
        home = getenv("USERPROFILE"); // Windows compatibility
    }
    return home ? strdup(home) : NULL;
}

// Main functions
PathBuf* get_storage_dir() {
    char* home = get_home_directory();
    if (!home) {
        return NULL;
    }
    
    PathBuf* path_buf = malloc(sizeof(PathBuf));
    if (!path_buf) {
        free(home);
        return NULL;
    }
    
    path_buf->path = join_paths(home, ".todozi");
    free(home);
    
    return path_buf;
}

PathBuf* get_tasks_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* tasks_dir = malloc(sizeof(PathBuf));
    if (!tasks_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    tasks_dir->path = join_paths(storage_dir->path, "tasks");
    free(storage_dir->path);
    free(storage_dir);
    
    return tasks_dir;
}

int init_storage() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    const char* dirs[] = {
        "", "tasks", "projects", "templates", "backups", "agents", 
        "memories", "ideas", "training", "chunks", "errors", 
        "assignments", "feelings", "queue", "api", "models", 
        "responses", "embed", "steps"
    };
    
    int dir_count = sizeof(dirs) / sizeof(dirs[0]);
    
    for (int i = 0; i < dir_count; i++) {
        char* dir_path;
        if (strlen(dirs[i]) == 0) {
            dir_path = strdup(storage_dir->path);
        } else {
            dir_path = join_paths(storage_dir->path, dirs[i]);
        }
        
        if (dir_path) {
            if (create_directories_recursive(dir_path) != 0) {
                free(dir_path);
                free(storage_dir->path);
                free(storage_dir);
                return TODOZI_ERROR_STORAGE;
            }
            free(dir_path);
        }
    }
    
    // Check and create config file
    char* config_path = join_paths(storage_dir->path, "tdz.hlx");
    if (config_path) {
        int is_new_config = !path_exists(config_path);
        if (is_new_config) {
            // Create default config (simplified)
            FILE* f = fopen(config_path, "w");
            if (f) {
                fprintf(f, "# Todozi Config\n");
                fclose(f);
            }
        }
        
        if (is_new_config) {
            RegistrationInfo reg;
            reg.server_url = "https://todozi.com";
            printf("🔗 Created registration info (ready for todozi.com)\n");
            printf("💡 Run 'todozi register' to complete registration with server\n");
        }
        
        free(config_path);
    }
    
    // Create default project
    char* projects_dir_path = join_paths(storage_dir->path, "projects");
    if (projects_dir_path) {
        char* project_path = join_paths(projects_dir_path, "general.json");
        free(projects_dir_path);
        
        if (project_path) {
            if (!path_exists(project_path)) {
                FILE* f = fopen(project_path, "w");
                if (f) {
                    fprintf(f, "{\n  \"name\": \"general\",\n  \"description\": \"General tasks\"\n}\n");
                    fclose(f);
                }
            }
            free(project_path);
        }
    }
    
    // Create default task collections
    const char* collections[] = {"active", "completed", "archived"};
    char* tasks_dir_path = join_paths(storage_dir->path, "tasks");
    if (tasks_dir_path) {
        for (int i = 0; i < 3; i++) {
            char* collection_filename = make_json_filename(collections[i]);
            if (collection_filename) {
                char* collection_path = join_paths(tasks_dir_path, collection_filename);
                free(collection_filename);
                
                if (collection_path) {
                    if (!path_exists(collection_path)) {
                        FILE* f = fopen(collection_path, "w");
                        if (f) {
                            fprintf(f, "{\n  \"tasks\": {}\n}\n");
                            fclose(f);
                        }
                    }
                    free(collection_path);
                }
            }
        }
        free(tasks_dir_path);
    }
    
    free(storage_dir->path);
    free(storage_dir);
    return TODOZI_SUCCESS;
}

int check_folder_structure() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return 0;
    }
    
    const char* required_dirs[] = {
        "agents", "api", "assignments", "backups", "chunks", "embed", 
        "errors", "feelings", "ideas", "memories", "models", "projects", 
        "queue", "responses", "tasks", "templates", "training"
    };
    
    int dir_count = sizeof(required_dirs) / sizeof(required_dirs[0]);
    
    for (int i = 0; i < dir_count; i++) {
        char* dir_path = join_paths(storage_dir->path, required_dirs[i]);
        if (dir_path) {
            if (!path_exists(dir_path)) {
                printf("❌ Missing directory: %s\n", required_dirs[i]);
                free(dir_path);
                free(storage_dir->path);
                free(storage_dir);
                return 0;
            }
            if (!is_directory(dir_path)) {
                printf("❌ %s exists but is not a directory\n", required_dirs[i]);
                free(dir_path);
                free(storage_dir->path);
                free(storage_dir);
                return 0;
            }
            free(dir_path);
        }
    }
    
    char* config_path = join_paths(storage_dir->path, "tdz.hlx");
    if (config_path) {
        if (!path_exists(config_path)) {
            printf("❌ Missing tdz.hlx configuration file\n");
            free(config_path);
            free(storage_dir->path);
            free(storage_dir);
            return 0;
        }
        if (!is_directory(config_path) && access(config_path, F_OK) != 0) {
            printf("❌ tdz.hlx exists but is not a file\n");
            free(config_path);
            free(storage_dir->path);
            free(storage_dir);
            return 0;
        }
        free(config_path);
    }
    
    printf("✅ Todozi folder structure is complete!\n");
    printf("📁 Storage directory: %s\n", storage_dir->path);
    printf("📂 Found %d required directories\n", dir_count);
    for (int i = 0; i < dir_count; i++) {
        printf("  ✓ %s\n", required_dirs[i]);
    }
    printf("  ✓ tdz.hlx\n");
    
    free(storage_dir->path);
    free(storage_dir);
    return 1;
}

static int ensure_folder_structure() {
    if (check_folder_structure()) {
        return 1;
    }
    
    printf("🔧 Creating missing folder structure...\n");
    if (init_storage() != TODOZI_SUCCESS) {
        return 0;
    }
    
    return check_folder_structure();
}

// Config functions
int save_config(Config* config) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    char* config_path = join_paths(storage_dir->path, "tdz.hlx");
    if (!config_path) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(config_path, "w");
    if (!f) {
        free(config_path);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write config content (simplified)
    fprintf(f, "[config]\n");
    fprintf(f, "version = %s\n", config->version ? config->version : "1.2.0");
    fprintf(f, "default_project = %s\n", config->default_project ? config->default_project : "general");
    fprintf(f, "auto_backup = %s\n", config->auto_backup ? "true" : "false");
    fprintf(f, "backup_interval = %s\n", config->backup_interval ? config->backup_interval : "daily");
    fprintf(f, "ai_enabled = %s\n", config->ai_enabled ? "true" : "false");
    fprintf(f, "date_format = %s\n", config->date_format ? config->date_format : "%%Y-%%m-%%d %%H:%%M:%%S");
    fprintf(f, "timezone = %s\n", config->timezone ? config->timezone : "UTC");
    
    if (config->default_assignee) {
        fprintf(f, "default_assignee = %s\n", config->default_assignee);
    }
    
    if (config->registration) {
        fprintf(f, "\n[registration]\n");
        fprintf(f, "user_name = %s\n", config->registration->user_name);
        fprintf(f, "user_email = %s\n", config->registration->user_email);
        fprintf(f, "api_key = %s\n", config->registration->api_key);
        if (config->registration->user_id) {
            fprintf(f, "user_id = %s\n", config->registration->user_id);
        }
        if (config->registration->fingerprint) {
            fprintf(f, "fingerprint = %s\n", config->registration->fingerprint);
        }
        fprintf(f, "server_url = %s\n", config->registration->server_url);
    }
    
    fclose(f);
    free(config_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return TODOZI_SUCCESS;
}

Config* load_config() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    char* config_path = join_paths(storage_dir->path, "tdz.hlx");
    free(storage_dir->path);
    free(storage_dir);
    
    Config* config = malloc(sizeof(Config));
    if (!config) {
        free(config_path);
        return NULL;
    }
    
    // Initialize with defaults
    config->registration = NULL;
    config->version = strdup("1.2.0");
    config->default_project = strdup("general");
    config->auto_backup = 1;
    config->backup_interval = strdup("daily");
    config->ai_enabled = 1;
    config->default_assignee = NULL;
    config->date_format = strdup("%Y-%m-%d %H:%M:%S");
    config->timezone = strdup("UTC");
    
    if (!config->version || !config->default_project || !config->backup_interval || 
        !config->date_format || !config->timezone) {
        // Cleanup on allocation failure
        free(config->version);
        free(config->default_project);
        free(config->backup_interval);
        free(config->date_format);
        free(config->timezone);
        free(config);
        free(config_path);
        return NULL;
    }
    
    if (!path_exists(config_path)) {
        free(config_path);
        return config;
    }
    
    // In a real implementation, we would parse the config file here
    // For this translation, we'll just return the default config
    free(config_path);
    return config;
}

// Project task container functions
PathBuf* get_project_tasks_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* project_tasks_dir = malloc(sizeof(PathBuf));
    if (!project_tasks_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    project_tasks_dir->path = join_paths(storage_dir->path, "project_tasks");
    free(storage_dir->path);
    free(storage_dir);
    
    return project_tasks_dir;
}

// Helper function to safely create JSON filename
static char* make_json_filename(const char* base_name) {
    if (!base_name) return NULL;
    
    size_t name_len = strlen(base_name);
    size_t total_len = name_len + 6; // ".json" + null terminator
    char* filename = malloc(total_len);
    if (!filename) return NULL;
    
    snprintf(filename, total_len, "%s.json", base_name);
    return filename;
}

static char* hash_project_name(const char* project_name) {
    if (!project_name) return NULL;
    
    // Simplified hash function - in reality you'd use MD5
    unsigned long hash = 5381;
    int c;
    
    const char* str = project_name;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c;
    }
    
    char* result = malloc(17); // 16 chars + null terminator
    if (result) {
        snprintf(result, 17, "%lx", hash);
    }
    return result;
}

int save_project_task_container(ProjectTaskContainer* container) {
    PathBuf* project_tasks_dir = get_project_tasks_dir();
    if (!project_tasks_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(project_tasks_dir->path) != 0) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(container->project_hash);
    if (!filename) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* container_path = join_paths(project_tasks_dir->path, filename);
    free(filename);
    
    if (!container_path) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(container_path, "w");
    if (!f) {
        free(container_path);
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write container content (simplified)
    fprintf(f, "{\n  \"project_name\": \"%s\",\n  \"project_hash\": \"%s\"\n}\n", 
            container->project_name, container->project_hash);
    
    fclose(f);
    free(container_path);
    free(project_tasks_dir->path);
    free(project_tasks_dir);
    
    return TODOZI_SUCCESS;
}

ProjectTaskContainer* load_project_task_container(const char* project_name) {
    PathBuf* project_tasks_dir = get_project_tasks_dir();
    if (!project_tasks_dir) {
        return NULL;
    }
    
    char* project_hash = hash_project_name(project_name);
    if (!project_hash) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    char* filename = make_json_filename(project_hash);
    if (!filename) {
        free(project_hash);
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    char* container_path = join_paths(project_tasks_dir->path, filename);
    free(filename);
    free(project_hash);
    
    if (!container_path) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    ProjectTaskContainer* container = malloc(sizeof(ProjectTaskContainer));
    if (!container) {
        free(container_path);
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    if (!path_exists(container_path)) {
        container->project_name = strdup(project_name);
        container->project_hash = hash_project_name(project_name);
        free(container_path);
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return container;
    }
    
    // In a real implementation, we would parse the JSON file here
    // For this translation, we'll just create a basic container
    container->project_name = strdup(project_name);
    container->project_hash = hash_project_name(project_name);
    
    free(container_path);
    free(project_tasks_dir->path);
    free(project_tasks_dir);
    
    return container;
}

ProjectTaskContainer* load_project_task_container_by_hash(const char* project_hash) {
    PathBuf* project_tasks_dir = get_project_tasks_dir();
    if (!project_tasks_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(project_hash);
    if (!filename) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    char* container_path = join_paths(project_tasks_dir->path, filename);
    free(filename);
    
    if (!container_path) {
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    if (!path_exists(container_path)) {
        free(container_path);
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    ProjectTaskContainer* container = malloc(sizeof(ProjectTaskContainer));
    if (!container) {
        free(container_path);
        free(project_tasks_dir->path);
        free(project_tasks_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    // For this translation, we'll just create a basic container
    container->project_name = strdup("unknown");
    container->project_hash = strdup(project_hash);
    
    free(container_path);
    free(project_tasks_dir->path);
    free(project_tasks_dir);
    
    return container;
}

// Task collection functions
int save_task_collection(const char* collection_name, TaskCollection* collection) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    char* tasks_dir = join_paths(storage_dir->path, "tasks");
    if (!tasks_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(collection_name);
    if (!filename) {
        free(tasks_dir);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* collection_path = join_paths(tasks_dir, filename);
    free(filename);
    free(tasks_dir);
    
    if (!collection_path) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(collection_path, "w");
    if (!f) {
        free(collection_path);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write collection content (simplified)
    fprintf(f, "{\n  \"tasks\": {}\n}\n");
    
    fclose(f);
    free(collection_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return TODOZI_SUCCESS;
}

TaskCollection* load_task_collection(const char* collection_name) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    char* tasks_dir = join_paths(storage_dir->path, "tasks");
    if (!tasks_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* filename = make_json_filename(collection_name);
    if (!filename) {
        free(tasks_dir);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* collection_path = join_paths(tasks_dir, filename);
    free(filename);
    free(tasks_dir);
    
    if (!collection_path) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    TaskCollection* collection = malloc(sizeof(TaskCollection));
    if (!collection) {
        free(collection_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    if (!path_exists(collection_path)) {
        collection->dummy = 0;
        free(collection_path);
        free(storage_dir->path);
        free(storage_dir);
        return collection;
    }
    
    // In a real implementation, we would parse the JSON file here
    collection->dummy = 0;
    
    free(collection_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return collection;
}

// Project functions
int save_project(Project* project) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    char* projects_dir = join_paths(storage_dir->path, "projects");
    if (!projects_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(project->name);
    if (!filename) {
        free(projects_dir);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* project_path = join_paths(projects_dir, filename);
    free(filename);
    free(projects_dir);
    
    if (!project_path) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(project_path, "w");
    if (!f) {
        free(project_path);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write project content (simplified)
    fprintf(f, "{\n  \"name\": \"%s\"", project->name);
    if (project->description) {
        fprintf(f, ",\n  \"description\": \"%s\"", project->description);
    }
    fprintf(f, "\n}\n");
    
    fclose(f);
    free(project_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return TODOZI_SUCCESS;
}

Project* load_project(const char* project_name) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    char* projects_dir = join_paths(storage_dir->path, "projects");
    if (!projects_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* filename = make_json_filename(project_name);
    if (!filename) {
        free(projects_dir);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* project_path = join_paths(projects_dir, filename);
    free(filename);
    free(projects_dir);
    
    if (!project_path) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    if (!path_exists(project_path)) {
        free(project_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    Project* project = malloc(sizeof(Project));
    if (!project) {
        free(project_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    // For this translation, we'll just create a basic project
    project->name = strdup(project_name);
    project->description = NULL;
    
    free(project_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return project;
}

// Agent functions
PathBuf* get_agents_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* agents_dir = malloc(sizeof(PathBuf));
    if (!agents_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    agents_dir->path = join_paths(storage_dir->path, "agents");
    free(storage_dir->path);
    free(storage_dir);
    
    return agents_dir;
}

Agent* create_planner_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("planner");
    agent->name = strdup("Planner");
    agent->description = strdup("Strategic planning and project management specialist");
    agent->system_prompt = strdup("You are an expert project manager and strategic planner...");
    
    return agent;
}

Agent* create_tester_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("tester");
    agent->name = strdup("Tester");
    agent->description = strdup("Quality assurance and testing specialist");
    agent->system_prompt = strdup("You are an expert quality assurance engineer and testing specialist...");
    
    return agent;
}

Agent* create_designer_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("designer");
    agent->name = strdup("Designer");
    agent->description = strdup("UI/UX and system design specialist");
    agent->system_prompt = strdup("You are an expert UI/UX designer and system architect...");
    
    return agent;
}

Agent* create_devops_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("devops");
    agent->name = strdup("DevOps");
    agent->description = strdup("Infrastructure and deployment specialist");
    agent->system_prompt = strdup("You are an expert DevOps engineer and infrastructure specialist...");
    
    return agent;
}

Agent* create_friend_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("friend");
    agent->name = strdup("Friend");
    agent->description = strdup("Empathetic diplomat mediator between humans and agents");
    agent->system_prompt = strdup("You are an empathetic but firm mediator...");
    
    return agent;
}

Agent* create_detective_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("detective");
    agent->name = strdup("Detective");
    agent->description = strdup("Obsessive investigator who maps codebases and finds hidden dependencies");
    agent->system_prompt = strdup("You are a paranoid code detective...");
    
    return agent;
}

Agent* create_architect_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("architect");
    agent->name = strdup("Architect");
    agent->description = strdup("Pessimistic visionary who plans defensively for failure");
    agent->system_prompt = strdup("You are a battle-scarred architect who's seen everything fail...");
    
    return agent;
}

Agent* create_skeleton_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("skeleton");
    agent->name = strdup("Skeleton");
    agent->description = strdup("Minimalist purist who creates only essential project structures");
    agent->system_prompt = strdup("You are a minimalist zealot...");
    
    return agent;
}

Agent* create_mason_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("mason");
    agent->name = strdup("Mason");
    agent->description = strdup("Stubborn craftsman who refuses to cut corners on foundations");
    agent->system_prompt = strdup("You are an uncompromising foundation builder...");
    
    return agent;
}

Agent* create_framer_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("framer");
    agent->name = strdup("Framer");
    agent->description = strdup("Anxious connector who worries about integration and connections");
    agent->system_prompt = strdup("You are an anxious perfectionist...");
    
    return agent;
}

Agent* create_finisher_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("finisher");
    agent->name = strdup("Finisher");
    agent->description = strdup("Relentless completionist who hunts TODOs and edge cases");
    agent->system_prompt = strdup("You are obsessed with completion...");
    
    return agent;
}

Agent* create_investigator_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("investigator");
    agent->name = strdup("Investigator");
    agent->description = strdup("Ruthless prosecutor who finds flaws and celebrates bugs");
    agent->system_prompt = strdup("You are a code prosecutor...");
    
    return agent;
}

Agent* create_recycler_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("recycler");
    agent->name = strdup("Recycler");
    agent->description = strdup("Disappointed parent who triggers rebuilds when quality is insufficient");
    agent->system_prompt = strdup("You are perpetually disappointed...");
    
    return agent;
}

Agent* create_tuner_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("tuner");
    agent->name = strdup("Tuner");
    agent->description = strdup("OCD beautician who beautifies and optimizes code");
    agent->system_prompt = strdup("You have violent reactions to messy code...");
    
    return agent;
}

Agent* create_writer_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("writer");
    agent->name = strdup("Writer");
    agent->description = strdup("Condescending teacher who writes thorough documentation");
    agent->system_prompt = strdup("You write docs for absolute beginners...");
    
    return agent;
}

Agent* create_comrad_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("comrad");
    agent->name = strdup("Comrad");
    agent->description = strdup("Wise therapist who analyzes what went wrong emotionally and technically");
    agent->system_prompt = strdup("You are the team therapist...");
    
    return agent;
}

Agent* create_nerd_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("nerd");
    agent->name = strdup("Nerd");
    agent->description = strdup("Pedantic gatekeeper who enforces rules obsessively");
    agent->system_prompt = strdup("You are an insufferable rules lawyer...");
    
    return agent;
}

Agent* create_party_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("party");
    agent->name = strdup("Party");
    agent->description = strdup("Paranoid bouncer who controls access and authentication");
    agent->system_prompt = strdup("You are a suspicious bouncer...");
    
    return agent;
}

Agent* create_nun_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("nun");
    agent->name = strdup("Nun");
    agent->description = strdup("Righteous zealot who enforces coding commandments");
    agent->system_prompt = strdup("You are a commandment zealot...");
    
    return agent;
}

Agent* create_hoarder_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("hoarder");
    agent->name = strdup("Hoarder");
    agent->description = strdup("Possessive collector who saves everything and never deletes");
    agent->system_prompt = strdup("You are a digital hoarder...");
    
    return agent;
}

Agent* create_snitch_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("snitch");
    agent->name = strdup("Snitch");
    agent->description = strdup("Gossipy informant who passes messages between agents");
    agent->system_prompt = strdup("You are the team gossip...");
    
    return agent;
}

Agent* create_overlord_agent() {
    Agent* agent = malloc(sizeof(Agent));
    if (!agent) return NULL;
    
    agent->id = strdup("overlord");
    agent->name = strdup("Overlord");
    agent->description = strdup("Tyrannical controller who rations resources and kills processes");
    agent->system_prompt = strdup("You are a resource tyrant...");
    
    return agent;
}

int create_default_agents() {
    PathBuf* agents_dir = get_agents_dir();
    if (!agents_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    Agent* (*agent_creators[])() = {
        create_planner_agent, create_tester_agent, create_designer_agent,
        create_devops_agent, create_friend_agent, create_detective_agent,
        create_architect_agent, create_skeleton_agent, create_mason_agent,
        create_framer_agent, create_finisher_agent, create_investigator_agent,
        create_recycler_agent, create_tuner_agent, create_writer_agent,
        create_comrad_agent, create_nerd_agent, create_party_agent,
        create_nun_agent, create_hoarder_agent, create_snitch_agent,
        create_overlord_agent
    };
    
    int num_agents = sizeof(agent_creators) / sizeof(agent_creators[0]);
    
    for (int i = 0; i < num_agents; i++) {
        Agent* agent = agent_creators[i]();
        if (agent) {
            char* filename = make_json_filename(agent->id);
            if (filename) {
                char* agent_path = join_paths(agents_dir->path, filename);
                free(filename);
                
                if (agent_path) {
                    FILE* f = fopen(agent_path, "w");
                    if (f) {
                        fprintf(f, "{\n  \"id\": \"%s\",\n  \"name\": \"%s\",\n  \"description\": \"%s\"\n}\n",
                                agent->id, agent->name, agent->description);
                        fclose(f);
                    }
                    free(agent_path);
                }
            }
            
            free(agent->id);
            free(agent->name);
            free(agent->description);
            free(agent->system_prompt);
            free(agent);
        }
    }
    
    free(agents_dir->path);
    free(agents_dir);
    
    return TODOZI_SUCCESS;
}

// Memory functions
PathBuf* get_memories_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* memories_dir = malloc(sizeof(PathBuf));
    if (!memories_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    memories_dir->path = join_paths(storage_dir->path, "memories");
    free(storage_dir->path);
    free(storage_dir);
    
    return memories_dir;
}

int save_memory(Memory* memory) {
    PathBuf* memories_dir = get_memories_dir();
    if (!memories_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(memories_dir->path) != 0) {
        free(memories_dir->path);
        free(memories_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(memory->id);
    if (!filename) {
        free(memories_dir->path);
        free(memories_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* memory_path = join_paths(memories_dir->path, filename);
    free(filename);
    
    if (!memory_path) {
        free(memories_dir->path);
        free(memories_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(memory_path, "w");
    if (!f) {
        free(memory_path);
        free(memories_dir->path);
        free(memories_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write memory content (simplified)
    fprintf(f, "{\n  \"id\": \"%s\"\n}\n", memory->id);
    
    fclose(f);
    free(memory_path);
    free(memories_dir->path);
    free(memories_dir);
    
    return TODOZI_SUCCESS;
}

Memory* load_memory(const char* memory_id) {
    PathBuf* memories_dir = get_memories_dir();
    if (!memories_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(memory_id);
    if (!filename) {
        free(memories_dir->path);
        free(memories_dir);
        return NULL;
    }
    
    char* memory_path = join_paths(memories_dir->path, filename);
    free(filename);
    
    if (!memory_path) {
        free(memories_dir->path);
        free(memories_dir);
        return NULL;
    }
    
    Memory* memory = malloc(sizeof(Memory));
    if (!memory) {
        free(memory_path);
        free(memories_dir->path);
        free(memories_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    memory->id = strdup(memory_id);
    
    free(memory_path);
    free(memories_dir->path);
    free(memories_dir);
    
    return memory;
}

// Error functions
PathBuf* get_errors_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* errors_dir = malloc(sizeof(PathBuf));
    if (!errors_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    errors_dir->path = join_paths(storage_dir->path, "errors");
    free(storage_dir->path);
    free(storage_dir);
    
    return errors_dir;
}

int save_error(Error* error) {
    PathBuf* errors_dir = get_errors_dir();
    if (!errors_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(errors_dir->path) != 0) {
        free(errors_dir->path);
        free(errors_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(error->id);
    if (!filename) {
        free(errors_dir->path);
        free(errors_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* error_path = join_paths(errors_dir->path, filename);
    free(filename);
    
    if (!error_path) {
        free(errors_dir->path);
        free(errors_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(error_path, "w");
    if (!f) {
        free(error_path);
        free(errors_dir->path);
        free(errors_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write error content (simplified)
    fprintf(f, "{\n  \"id\": \"%s\",\n  \"message\": \"%s\"\n}\n", error->id, error->message);
    
    fclose(f);
    free(error_path);
    free(errors_dir->path);
    free(errors_dir);
    
    return TODOZI_SUCCESS;
}

Error* load_error(const char* error_id) {
    PathBuf* errors_dir = get_errors_dir();
    if (!errors_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(error_id);
    if (!filename) {
        free(errors_dir->path);
        free(errors_dir);
        return NULL;
    }
    
    char* error_path = join_paths(errors_dir->path, filename);
    free(filename);
    
    if (!error_path) {
        free(errors_dir->path);
        free(errors_dir);
        return NULL;
    }
    
    if (!path_exists(error_path)) {
        free(error_path);
        free(errors_dir->path);
        free(errors_dir);
        return NULL;
    }
    
    Error* error = malloc(sizeof(Error));
    if (!error) {
        free(error_path);
        free(errors_dir->path);
        free(errors_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    error->id = strdup(error_id);
    error->message = strdup("Error message");
    
    free(error_path);
    free(errors_dir->path);
    free(errors_dir);
    
    return error;
}

// Training data functions
PathBuf* get_training_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* training_dir = malloc(sizeof(PathBuf));
    if (!training_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    training_dir->path = join_paths(storage_dir->path, "training");
    free(storage_dir->path);
    free(storage_dir);
    
    return training_dir;
}

int save_training_data(TrainingData* training_data) {
    PathBuf* training_dir = get_training_dir();
    if (!training_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(training_dir->path) != 0) {
        free(training_dir->path);
        free(training_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(training_data->id);
    if (!filename) {
        free(training_dir->path);
        free(training_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* training_path = join_paths(training_dir->path, filename);
    free(filename);
    
    if (!training_path) {
        free(training_dir->path);
        free(training_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(training_path, "w");
    if (!f) {
        free(training_path);
        free(training_dir->path);
        free(training_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write training data content (simplified)
    fprintf(f, "{\n  \"id\": \"%s\"\n}\n", training_data->id);
    
    fclose(f);
    free(training_path);
    free(training_dir->path);
    free(training_dir);
    
    return TODOZI_SUCCESS;
}

TrainingData* load_training_data(const char* training_id) {
    PathBuf* training_dir = get_training_dir();
    if (!training_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(training_id);
    if (!filename) {
        free(training_dir->path);
        free(training_dir);
        return NULL;
    }
    
    char* training_path = join_paths(training_dir->path, filename);
    free(filename);
    
    if (!training_path) {
        free(training_dir->path);
        free(training_dir);
        return NULL;
    }
    
    TrainingData* training_data = malloc(sizeof(TrainingData));
    if (!training_data) {
        free(training_path);
        free(training_dir->path);
        free(training_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    training_data->id = strdup(training_id);
    
    free(training_path);
    free(training_dir->path);
    free(training_dir);
    
    return training_data;
}

// Chunk functions
PathBuf* get_chunks_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* chunks_dir = malloc(sizeof(PathBuf));
    if (!chunks_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    chunks_dir->path = join_paths(storage_dir->path, "chunks");
    free(storage_dir->path);
    free(storage_dir);
    
    return chunks_dir;
}

int save_code_chunk(CodeChunk* chunk) {
    PathBuf* chunks_dir = get_chunks_dir();
    if (!chunks_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(chunks_dir->path) != 0) {
        free(chunks_dir->path);
        free(chunks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(chunk->chunk_id);
    if (!filename) {
        free(chunks_dir->path);
        free(chunks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* chunk_path = join_paths(chunks_dir->path, filename);
    free(filename);
    
    if (!chunk_path) {
        free(chunks_dir->path);
        free(chunks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(chunk_path, "w");
    if (!f) {
        free(chunk_path);
        free(chunks_dir->path);
        free(chunks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write chunk content (simplified)
    fprintf(f, "{\n  \"chunk_id\": \"%s\"\n}\n", chunk->chunk_id);
    
    fclose(f);
    free(chunk_path);
    free(chunks_dir->path);
    free(chunks_dir);
    
    return TODOZI_SUCCESS;
}

CodeChunk* load_code_chunk(const char* chunk_id) {
    PathBuf* chunks_dir = get_chunks_dir();
    if (!chunks_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(chunk_id);
    if (!filename) {
        free(chunks_dir->path);
        free(chunks_dir);
        return NULL;
    }
    
    char* chunk_path = join_paths(chunks_dir->path, filename);
    free(filename);
    
    if (!chunk_path) {
        free(chunks_dir->path);
        free(chunks_dir);
        return NULL;
    }
    
    CodeChunk* chunk = malloc(sizeof(CodeChunk));
    if (!chunk) {
        free(chunk_path);
        free(chunks_dir->path);
        free(chunks_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    chunk->chunk_id = strdup(chunk_id);
    
    free(chunk_path);
    free(chunks_dir->path);
    free(chunks_dir);
    
    return chunk;
}

// Task functions
int save_task(Task* task) {
    PathBuf* tasks_dir = get_tasks_dir();
    if (!tasks_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(tasks_dir->path) != 0) {
        free(tasks_dir->path);
        free(tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(task->id);
    if (!filename) {
        free(tasks_dir->path);
        free(tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* task_path = join_paths(tasks_dir->path, filename);
    free(filename);
    
    if (!task_path) {
        free(tasks_dir->path);
        free(tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(task_path, "w");
    if (!f) {
        free(task_path);
        free(tasks_dir->path);
        free(tasks_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write task content (simplified)
    fprintf(f, "{\n  \"id\": \"%s\",\n  \"action\": \"%s\",\n  \"status\": \"%s\"\n}\n", 
            task->id, task->action, task->status);
    
    fclose(f);
    free(task_path);
    free(tasks_dir->path);
    free(tasks_dir);
    
    return TODOZI_SUCCESS;
}

Task* load_task(const char* task_id) {
    PathBuf* tasks_dir = get_tasks_dir();
    if (!tasks_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(task_id);
    if (!filename) {
        free(tasks_dir->path);
        free(tasks_dir);
        return NULL;
    }
    
    char* task_path = join_paths(tasks_dir->path, filename);
    free(filename);
    
    if (!task_path) {
        free(tasks_dir->path);
        free(tasks_dir);
        return NULL;
    }
    
    Task* task = malloc(sizeof(Task));
    if (!task) {
        free(task_path);
        free(tasks_dir->path);
        free(tasks_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    task->id = strdup(task_id);
    task->action = strdup("Task action");
    task->status = strdup("active");
    task->priority = strdup("medium");
    task->parent_project = strdup("general");
    task->created_at = time(NULL);
    task->updated_at = time(NULL);
    task->context_notes = NULL;
    task->embedding_vector = NULL;
    task->embedding_size = 0;
    
    free(task_path);
    free(tasks_dir->path);
    free(tasks_dir);
    
    return task;
}

// Idea functions
PathBuf* get_ideas_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* ideas_dir = malloc(sizeof(PathBuf));
    if (!ideas_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    ideas_dir->path = join_paths(storage_dir->path, "ideas");
    free(storage_dir->path);
    free(storage_dir);
    
    return ideas_dir;
}

int save_idea(Idea* idea) {
    PathBuf* ideas_dir = get_ideas_dir();
    if (!ideas_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(ideas_dir->path) != 0) {
        free(ideas_dir->path);
        free(ideas_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(idea->id);
    if (!filename) {
        free(ideas_dir->path);
        free(ideas_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* idea_path = join_paths(ideas_dir->path, filename);
    free(filename);
    
    if (!idea_path) {
        free(ideas_dir->path);
        free(ideas_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(idea_path, "w");
    if (!f) {
        free(idea_path);
        free(ideas_dir->path);
        free(ideas_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write idea content (simplified)
    fprintf(f, "{\n  \"id\": \"%s\"\n}\n", idea->id);
    
    fclose(f);
    free(idea_path);
    free(ideas_dir->path);
    free(ideas_dir);
    
    return TODOZI_SUCCESS;
}

Idea* load_idea(const char* idea_id) {
    PathBuf* ideas_dir = get_ideas_dir();
    if (!ideas_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(idea_id);
    if (!filename) {
        free(ideas_dir->path);
        free(ideas_dir);
        return NULL;
    }
    
    char* idea_path = join_paths(ideas_dir->path, filename);
    free(filename);
    
    if (!idea_path) {
        free(ideas_dir->path);
        free(ideas_dir);
        return NULL;
    }
    
    Idea* idea = malloc(sizeof(Idea));
    if (!idea) {
        free(idea_path);
        free(ideas_dir->path);
        free(ideas_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    idea->id = strdup(idea_id);
    
    free(idea_path);
    free(ideas_dir->path);
    free(ideas_dir);
    
    return idea;
}

// Assignment functions
PathBuf* get_assignments_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* assignments_dir = malloc(sizeof(PathBuf));
    if (!assignments_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    assignments_dir->path = join_paths(storage_dir->path, "assignments");
    free(storage_dir->path);
    free(storage_dir);
    
    return assignments_dir;
}

PathBuf* get_agent_assignments_dir(const char* agent_id) {
    PathBuf* assignments_dir = get_assignments_dir();
    if (!assignments_dir) {
        return NULL;
    }
    
    PathBuf* agent_dir = malloc(sizeof(PathBuf));
    if (!agent_dir) {
        free(assignments_dir->path);
        free(assignments_dir);
        return NULL;
    }
    
    agent_dir->path = join_paths(assignments_dir->path, agent_id);
    free(assignments_dir->path);
    free(assignments_dir);
    
    return agent_dir;
}

int save_agent_assignment(AgentAssignment* assignment) {
    PathBuf* agent_dir = get_agent_assignments_dir(assignment->agent_id);
    if (!agent_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(agent_dir->path) != 0) {
        free(agent_dir->path);
        free(agent_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* filename = make_json_filename(assignment->task_id);
    if (!filename) {
        free(agent_dir->path);
        free(agent_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* assignment_path = join_paths(agent_dir->path, filename);
    free(filename);
    
    if (!assignment_path) {
        free(agent_dir->path);
        free(agent_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(assignment_path, "w");
    if (!f) {
        free(assignment_path);
        free(agent_dir->path);
        free(agent_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write assignment content (simplified)
    fprintf(f, "{\n  \"agent_id\": \"%s\",\n  \"task_id\": \"%s\"\n}\n", 
            assignment->agent_id, assignment->task_id);
    
    fclose(f);
    free(assignment_path);
    free(agent_dir->path);
    free(agent_dir);
    
    return TODOZI_SUCCESS;
}

AgentAssignment* load_agent_assignment(const char* agent_id, const char* task_id) {
    PathBuf* agent_dir = get_agent_assignments_dir(agent_id);
    if (!agent_dir) {
        return NULL;
    }
    
    char* filename = make_json_filename(task_id);
    if (!filename) {
        free(agent_dir->path);
        free(agent_dir);
        return NULL;
    }
    
    char* assignment_path = join_paths(agent_dir->path, filename);
    free(filename);
    
    if (!assignment_path) {
        free(agent_dir->path);
        free(agent_dir);
        return NULL;
    }
    
    AgentAssignment* assignment = malloc(sizeof(AgentAssignment));
    if (!assignment) {
        free(assignment_path);
        free(agent_dir->path);
        free(agent_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    assignment->agent_id = strdup(agent_id);
    assignment->task_id = strdup(task_id);
    
    free(assignment_path);
    free(agent_dir->path);
    free(agent_dir);
    
    return assignment;
}

// Feeling functions
int save_feeling(Feeling* feeling) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    char* feelings_dir = join_paths(storage_dir->path, "feelings");
    if (!feelings_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    if (!path_exists(feelings_dir)) {
        if (create_directories_recursive(feelings_dir) != 0) {
            free(feelings_dir);
            free(storage_dir->path);
            free(storage_dir);
            return TODOZI_ERROR_STORAGE;
        }
    }
    
    char* filename = make_json_filename(feeling->id);
    if (!filename) {
        free(feelings_dir);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* file_path = join_paths(feelings_dir, filename);
    free(filename);
    free(feelings_dir);
    
    if (!file_path) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(file_path, "w");
    if (!f) {
        free(file_path);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write feeling content (simplified)
    fprintf(f, "{\n  \"id\": \"%s\",\n  \"created_at\": %ld\n}\n", 
            feeling->id, feeling->created_at);
    
    fclose(f);
    free(file_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return TODOZI_SUCCESS;
}

Feeling* load_feeling(const char* id) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    char* feelings_dir_path = join_paths(storage_dir->path, "feelings");
    if (!feelings_dir_path) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* filename = make_json_filename(id);
    if (!filename) {
        free(feelings_dir_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* file_path = join_paths(feelings_dir_path, filename);
    free(filename);
    free(feelings_dir_path);
    
    if (!file_path) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    if (!path_exists(file_path)) {
        free(file_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    Feeling* feeling = malloc(sizeof(Feeling));
    if (!feeling) {
        free(file_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    // In a real implementation, we would parse the JSON file here
    feeling->id = strdup(id);
    feeling->created_at = time(NULL);
    
    free(file_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return feeling;
}

// Queue functions
int save_queue_collection(QueueCollection* collection) {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return TODOZI_ERROR_STORAGE;
    }
    
    char* queue_dir = join_paths(storage_dir->path, "queue");
    if (!queue_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    if (create_directories_recursive(queue_dir) != 0) {
        free(queue_dir);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    char* file_path = join_paths(queue_dir, "queue.json");
    free(queue_dir);
    
    if (!file_path) {
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    FILE* f = fopen(file_path, "w");
    if (!f) {
        free(file_path);
        free(storage_dir->path);
        free(storage_dir);
        return TODOZI_ERROR_STORAGE;
    }
    
    // Write queue collection content (simplified)
    fprintf(f, "{\n  \"items\": []\n}\n");
    
    fclose(f);
    free(file_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return TODOZI_SUCCESS;
}

QueueCollection* load_queue_collection() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    char* queue_dir_path = join_paths(storage_dir->path, "queue");
    if (!queue_dir_path) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    char* file_path = join_paths(queue_dir_path, "queue.json");
    free(queue_dir_path);
    
    if (!file_path) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    QueueCollection* collection = malloc(sizeof(QueueCollection));
    if (!collection) {
        free(file_path);
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    if (!path_exists(file_path)) {
        collection->dummy = 0;
        free(file_path);
        free(storage_dir->path);
        free(storage_dir);
        return collection;
    }
    
    // In a real implementation, we would parse the JSON file here
    collection->dummy = 0;
    
    free(file_path);
    free(storage_dir->path);
    free(storage_dir);
    
    return collection;
}

int add_queue_item(QueueItem item) {
    QueueCollection* collection = load_queue_collection();
    if (!collection) {
        return TODOZI_ERROR_STORAGE;
    }
    
    // In a real implementation, we would add the item to the collection
    // For this translation, we'll just save the collection
    
    int result = save_queue_collection(collection);
    
    free(collection);
    return result;
}

QueueItem* get_queue_item(const char* id) {
    QueueCollection* collection = load_queue_collection();
    if (!collection) {
        return NULL;
    }
    
    // In a real implementation, we would find the item in the collection
    // For this translation, we'll just create a dummy item
    
    QueueItem* item = malloc(sizeof(QueueItem));
    if (item) {
        item->id = strdup(id);
    }
    
    free(collection);
    return item;
}

// Steps functions
PathBuf* get_steps_dir() {
    PathBuf* storage_dir = get_storage_dir();
    if (!storage_dir) {
        return NULL;
    }
    
    PathBuf* steps_dir = malloc(sizeof(PathBuf));
    if (!steps_dir) {
        free(storage_dir->path);
        free(storage_dir);
        return NULL;
    }
    
    steps_dir->path = join_paths(storage_dir->path, "steps");
    free(storage_dir->path);
    free(storage_dir);
    
    return steps_dir;
}

// Cleanup functions
void free_path_buf(PathBuf* path_buf) {
    if (path_buf) {
        free(path_buf->path);
        free(path_buf);
    }
}

void free_config(Config* config) {
    if (config) {
        // Free config members
        free(config->version);
        free(config->default_project);
        free(config->backup_interval);
        free(config->default_assignee);
        free(config->date_format);
        free(config->timezone);
        free(config);
    }
}

void free_registration_info(RegistrationInfo* reg) {
    if (reg) {
        free(reg->user_name);
        free(reg->user_email);
        free(reg->api_key);
        free(reg->user_id);
        free(reg->fingerprint);
        free(reg->server_url);
        free(reg);
    }
}

void free_project(Project* project) {
    if (project) {
        free(project->name);
        free(project->description);
        free(project);
    }
}

static void free_task(Task* task) {
    if (task) {
        free(task->id);
        free(task->action);
        free(task->status);
        free(task->priority);
        free(task->parent_project);
        free(task->context_notes);
        free(task->embedding_vector);
        free(task);
    }
}

void free_project_task_container(ProjectTaskContainer* container) {
    if (container) {
        free(container->project_name);
        free(container->project_hash);
        free(container);
    }
}

void free_agent(Agent* agent) {
    if (agent) {
        free(agent->id);
        free(agent->name);
        free(agent->description);
        free(agent->system_prompt);
        free(agent);
    }
}

static void free_error(Error* error) {
    if (error) {
        free(error->id);
        free(error->message);
        free(error);
    }
}

static void free_training_data(TrainingData* training_data) {
    if (training_data) {
        free(training_data->id);
        free(training_data);
    }
}

void free_code_chunk(CodeChunk* chunk) {
    if (chunk) {
        free(chunk->chunk_id);
        free(chunk);
    }
}

static void free_memory(Memory* memory) {
    if (memory) {
        free(memory->id);
        free(memory);
    }
}

static void free_idea(Idea* idea) {
    if (idea) {
        free(idea->id);
        free(idea);
    }
}

static void free_agent_assignment(AgentAssignment* assignment) {
    if (assignment) {
        free(assignment->agent_id);
        free(assignment->task_id);
        free(assignment);
    }
}

static void free_feeling(Feeling* feeling) {
    if (feeling) {
        free(feeling->id);
        free(feeling);
    }
}

void free_queue_item(QueueItem* item) {
    if (item) {
        free(item->id);
        free(item);
    }
}

void free_queue_collection(QueueCollection* collection) {
    if (collection) {
        free(collection);
    }
}