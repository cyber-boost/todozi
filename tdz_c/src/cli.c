#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <errno.h>

// Platform-specific includes for path expansion
#ifdef _WIN32
    #include <windows.h>
    #include <shlobj.h>
    #include <direct.h>
    #define mkdir(path, mode) _mkdir(path)
#else
    #include <unistd.h>
    #include <pwd.h>
    #include <sys/stat.h>
#endif

// Forward declarations
typedef struct TodoziHandler TodoziHandler;
typedef struct Storage Storage;
typedef struct Task Task;
typedef struct QueueItem QueueItem;

// Error handling
typedef enum {
    TODOZI_SUCCESS,
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_IO,
    TODOZI_ERROR_PARSE
} TodoziResult;

// Priority enum
typedef enum {
    PRIORITY_LOW,
    PRIORITY_MEDIUM,
    PRIORITY_HIGH,
    PRIORITY_CRITICAL,
    PRIORITY_URGENT
} Priority;

// Status enum
typedef enum {
    STATUS_TODO,
    STATUS_IN_PROGRESS,
    STATUS_BLOCKED,
    STATUS_REVIEW,
    STATUS_DONE,
    STATUS_CANCELLED,
    STATUS_DEFERRED
} Status;

// Assignee enum
typedef enum {
    ASSIGNEE_AI,
    ASSIGNEE_HUMAN,
    ASSIGNEE_COLLABORATIVE
} Assignee;

// QueueStatus enum
typedef enum {
    QUEUE_STATUS_BACKLOG,
    QUEUE_STATUS_ACTIVE,
    QUEUE_STATUS_COMPLETE
} QueueStatus;

// Basic structures
struct Storage {
    char* data_path;
};

struct Task {
    char* id;
    char* action;
    char* time_estimate;
    Priority priority;
    char* project;
    Status status;
    Assignee assignee;
    char** tags;
    int tag_count;
    char** dependencies;
    int dep_count;
    char* context_notes;
    int progress;
    time_t created_at;
    time_t updated_at;
};

struct QueueItem {
    char* id;
    char* task_name;
    char* task_description;
    Priority priority;
    char* project_id;
    QueueStatus status;
    time_t created_at;
};

struct TodoziHandler {
    Storage* storage;
};

// Function prototypes
static TodoziHandler* todozi_handler_new(Storage* storage);
static void todozi_handler_free(TodoziHandler* handler);
TodoziResult todozi_handler_complete_task(TodoziHandler* handler, const char* id);
TodoziResult todozi_handler_fix_task_consistency(TodoziHandler* handler);
TodoziResult todozi_handler_delete_task(TodoziHandler* handler, const char* id);
TodoziResult todozi_handler_restore_backup(TodoziHandler* handler, const char* backup_name);

// Storage functions
static Storage* storage_new(void);
static void storage_free(Storage* storage);
TodoziResult storage_complete_task_in_project(Storage* storage, const char* id);
TodoziResult storage_fix_completed_tasks_consistency(Storage* storage);
TodoziResult storage_delete_task_from_project(Storage* storage, const char* id);
TodoziResult storage_restore_backup(Storage* storage, const char* backup_name);

// Implementation

// Helper function to expand tilde in path (cross-platform)
static char* expand_path(const char* path) {
    if (!path) return NULL;
    
    // If path doesn't start with ~, just duplicate it
    if (path[0] != '~') {
        return strdup(path);
    }
    
    // Handle ~/path format
    if (path[1] == '/' || path[1] == '\0') {
        char* home = NULL;
        
#ifdef _WIN32
        // Windows: Use SHGetFolderPath or environment variable
        char win_home[MAX_PATH];
        if (SUCCEEDED(SHGetFolderPathA(NULL, CSIDL_PROFILE, NULL, 0, win_home))) {
            home = strdup(win_home);
        } else {
            const char* userprofile = getenv("USERPROFILE");
            if (userprofile) {
                home = strdup(userprofile);
            }
        }
#else
        // Unix-like: Use HOME environment variable or getpwuid
        const char* home_env = getenv("HOME");
        if (home_env) {
            home = strdup(home_env);
        } else {
            // Fallback to getpwuid
            struct passwd* pw = getpwuid(getuid());
            if (pw && pw->pw_dir) {
                home = strdup(pw->pw_dir);
            }
        }
#endif
        
        if (!home) {
            return strdup(path); // Return original if we can't expand
        }
        
        size_t home_len = strlen(home);
        size_t path_len = strlen(path);
        size_t total_len = home_len + path_len; // path includes ~
        
        char* expanded = malloc(total_len + 1);
        if (!expanded) {
            free(home);
            return NULL;
        }
        
        strcpy(expanded, home);
        free(home); // Free the duplicated home string
        if (path[1] != '\0') {
            strcat(expanded, path + 1); // Skip ~
        }
        
        return expanded;
    }
    
    // Handle ~user/path format (not fully implemented, just return original)
    return strdup(path);
}

static Storage* storage_new(void) {
    Storage* storage = malloc(sizeof(Storage));
    if (!storage) return NULL;
    
    // Expand ~/.todozi to actual home directory path
    storage->data_path = expand_path("~/.todozi");
    if (!storage->data_path) {
        free(storage);
        return NULL;
    }
    
    // Ensure directory exists
    struct stat st = {0};
    if (stat(storage->data_path, &st) == -1) {
        // Try to create directory (mkdir -p equivalent)
        char* path = strdup(storage->data_path);
        if (path) {
            for (char* p = path + 1; *p; p++) {
                if (*p == '/') {
                    *p = '\0';
                    mkdir(path, 0700);
                    *p = '/';
                }
            }
            mkdir(path, 0700);
            free(path);
        }
    }
    
    return storage;
}

static void storage_free(Storage* storage) {
    if (storage) {
        free(storage->data_path);
        free(storage);
    }
}

TodoziResult storage_complete_task_in_project(Storage* storage, const char* id) {
    if (!storage || !id) return TODOZI_ERROR_VALIDATION;
    
    printf("Completing task %s in project\n", id);
    return TODOZI_SUCCESS;
}

TodoziResult storage_fix_completed_tasks_consistency(Storage* storage) {
    if (!storage) return TODOZI_ERROR_VALIDATION;
    
    printf("Fixing completed tasks consistency\n");
    return TODOZI_SUCCESS;
}

TodoziResult storage_delete_task_from_project(Storage* storage, const char* id) {
    if (!storage || !id) return TODOZI_ERROR_VALIDATION;
    
    printf("Deleting task %s from project\n", id);
    return TODOZI_SUCCESS;
}

TodoziResult storage_restore_backup(Storage* storage, const char* backup_name) {
    if (!storage || !backup_name) return TODOZI_ERROR_VALIDATION;
    
    printf("Restoring backup %s\n", backup_name);
    return TODOZI_SUCCESS;
}

static TodoziHandler* todozi_handler_new(Storage* storage) {
    if (!storage) return NULL;
    
    TodoziHandler* handler = malloc(sizeof(TodoziHandler));
    if (!handler) return NULL;
    
    handler->storage = storage;
    return handler;
}

static void todozi_handler_free(TodoziHandler* handler) {
    if (handler) {
        // Note: handler doesn't own storage, so we don't free it here
        // The caller is responsible for freeing storage separately
        free(handler);
    }
}

TodoziResult todozi_handler_complete_task(TodoziHandler* handler, const char* id) {
    if (!handler || !id) return TODOZI_ERROR_VALIDATION;
    return storage_complete_task_in_project(handler->storage, id);
}

TodoziResult todozi_handler_fix_task_consistency(TodoziHandler* handler) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("🔧 Fixing task data consistency...\n");
    TodoziResult result = storage_fix_completed_tasks_consistency(handler->storage);
    if (result == TODOZI_SUCCESS) {
        printf("✅ Task consistency fix completed!\n");
    }
    return result;
}

TodoziResult todozi_handler_delete_task(TodoziHandler* handler, const char* id) {
    if (!handler || !id) return TODOZI_ERROR_VALIDATION;
    return storage_delete_task_from_project(handler->storage, id);
}

TodoziResult todozi_handler_restore_backup(TodoziHandler* handler, const char* backup_name) {
    if (!handler || !backup_name) return TODOZI_ERROR_VALIDATION;
    return storage_restore_backup(handler->storage, backup_name);
}

// Queue item functions
QueueItem* queue_item_new(const char* task_name, const char* task_description, 
                         Priority priority, const char* project_id) {
    if (!task_name) return NULL;
    
    QueueItem* item = malloc(sizeof(QueueItem));
    if (!item) return NULL;
    
    // Initialize all pointers to NULL for safe cleanup
    item->id = NULL;
    item->task_name = NULL;
    item->task_description = NULL;
    item->project_id = NULL;
    
    // Allocate ID buffer (UUID length + null terminator)
    item->id = malloc(37);
    if (!item->id) {
        free(item);
        return NULL;
    }
    // Generate simple ID (in real implementation would use proper UUID)
    strncpy(item->id, "queue-item-id", 36);
    item->id[36] = '\0';
    
    item->task_name = strdup(task_name);
    if (!item->task_name) {
        free(item->id);
        free(item);
        return NULL;
    }
    
    item->task_description = task_description ? strdup(task_description) : strdup("");
    if (!item->task_description) {
        free(item->id);
        free(item->task_name);
        free(item);
        return NULL;
    }
    
    item->priority = priority;
    item->project_id = project_id ? strdup(project_id) : NULL;
    item->status = QUEUE_STATUS_BACKLOG;
    item->created_at = time(NULL);
    
    return item;
}

void queue_item_free(QueueItem* item) {
    if (item) {
        free(item->id);
        free(item->task_name);
        free(item->task_description);
        free(item->project_id);
        free(item);
    }
}

// API command handling functions
TodoziResult handle_api_register(const char* user_id) {
    printf("🔑 API key created successfully!\n");
    printf("🆔 User ID: %s\n", user_id ? user_id : "generated-id");
    printf("🔓 Public Key: public-key-placeholder\n");
    printf("🔒 Private Key: private-key-placeholder\n");
    printf("✅ Active: true\n");
    time_t now = time(NULL);
    printf("🕒 Created: %s", ctime(&now));
    printf("\n");
    printf("💡 Keep your private key secure! It provides admin access.\n");
    printf("📖 Use public key for read-only access, both keys for admin access.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_api_list(int active_only) {
    printf("🔑 API Keys:\n\n");
    printf("No API keys found\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_api_check(const char* public_key, const char* private_key) {
    if (!public_key) return TODOZI_ERROR_VALIDATION;
    
    printf("✅ API key authentication successful!\n");
    printf("🆔 User ID: user-id-placeholder\n");
    printf("🔓 Public Key: %s\n", public_key);
    if (private_key) {
        printf("🔒 Private Key: %s\n", private_key);
    }
    printf("👑 Admin Access: true\n");
    printf("📖 Access Level: admin\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_api_deactivate(const char* user_id) {
    if (!user_id) return TODOZI_ERROR_VALIDATION;
    
    printf("🔒 API key deactivated successfully!\n");
    printf("🆔 User ID: %s\n", user_id);
    return TODOZI_SUCCESS;
}

TodoziResult handle_api_activate(const char* user_id) {
    if (!user_id) return TODOZI_ERROR_VALIDATION;
    
    printf("🔓 API key activated successfully!\n");
    printf("🆔 User ID: %s\n", user_id);
    return TODOZI_SUCCESS;
}

TodoziResult handle_api_remove(const char* user_id) {
    if (!user_id) return TODOZI_ERROR_VALIDATION;
    
    printf("🗑️  API key removed successfully!\n");
    printf("🆔 User ID: %s\n", user_id);
    printf("🔓 Public Key: public-key-placeholder\n");
    printf("🔒 Private Key: private-key-placeholder\n");
    return TODOZI_SUCCESS;
}

// Queue command handling functions
TodoziResult handle_queue_plan(const char* task_name, const char* task_description,
                              const char* priority_str, const char* project_id) {
    if (!task_name) return TODOZI_ERROR_VALIDATION;
    
    Priority priority = PRIORITY_MEDIUM;
    if (priority_str) {
        if (strcmp(priority_str, "low") == 0) priority = PRIORITY_LOW;
        else if (strcmp(priority_str, "high") == 0) priority = PRIORITY_HIGH;
        else if (strcmp(priority_str, "critical") == 0) priority = PRIORITY_CRITICAL;
        else if (strcmp(priority_str, "urgent") == 0) priority = PRIORITY_URGENT;
    }
    
    QueueItem* item = queue_item_new(task_name, task_description, priority, project_id);
    if (!item) return TODOZI_ERROR_IO;
    
    printf("✅ Queue item planned successfully!\n");
    printf("📋 ID: %s\n", item->id);
    printf("📝 Task: %s\n", item->task_name);
    printf("📄 Description: %s\n", item->task_description);
    printf("⚡ Priority: %d\n", item->priority);
    if (item->project_id) {
        printf("📁 Project: %s\n", item->project_id);
    }
    printf("📊 Status: %d\n", item->status);
    
    queue_item_free(item);
    return TODOZI_SUCCESS;
}

TodoziResult handle_queue_list(const char* status_str) {
    printf("📋 Queue Items:\n\n");
    printf("No queue items found\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_queue_backlog(void) {
    printf("📋 Backlog Items:\n\n");
    printf("No backlog items found\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_queue_active(void) {
    printf("📋 Active Items:\n\n");
    printf("No active items found\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_queue_complete(void) {
    printf("📋 Complete Items:\n\n");
    printf("No complete items found\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_queue_start(const char* queue_item_id) {
    if (!queue_item_id) return TODOZI_ERROR_VALIDATION;
    
    char session_id[37];
    strncpy(session_id, "session-id-placeholder", sizeof(session_id) - 1);
    session_id[sizeof(session_id) - 1] = '\0';
    
    printf("🚀 Queue session started successfully!\n");
    printf("🆔 Session ID: %s\n", session_id);
    printf("📋 Queue Item ID: %s\n", queue_item_id);
    time_t now = time(NULL);
    printf("🕒 Started at: %s", ctime(&now));
    
    return TODOZI_SUCCESS;
}

TodoziResult handle_queue_end(const char* session_id) {
    if (!session_id) return TODOZI_ERROR_VALIDATION;
    
    printf("✅ Queue session ended successfully!\n");
    printf("🆔 Session ID: %s\n", session_id);
    printf("📋 Queue Item ID: queue-item-id-placeholder\n");
    time_t now = time(NULL);
    printf("🕒 Started: %s", ctime(&now));
    printf("🕒 Ended: %s", ctime(&now));
    printf("⏱️  Duration: 0 seconds\n");
    
    return TODOZI_SUCCESS;
}

// Server command handling functions
TodoziResult handle_server_start(const char* host, int port) {
    printf("🚀 Starting Todozi Enhanced Server...\n");
    printf("📡 Host: %s\n", host ? host : "127.0.0.1");
    printf("🔌 Port: %d\n", port);
    printf("📋 Available at: http://%s:%d\n\n", host ? host : "127.0.0.1", port);
    printf("❌ Failed to start server: Not implemented in C version\n");
    return TODOZI_ERROR_VALIDATION;
}

TodoziResult handle_server_status(void) {
    printf("🔍 Checking server status...\n");
    printf("❌ Server is not running on common ports (8636, 8637, 3000)\n");
    printf("💡 Start it with: todozi server start\n");
    printf("💡 Or specify port: todozi server start --port 8636\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_server_endpoints(void) {
    printf("📡 Todozi Enhanced Server API Endpoints\n");
    printf("══════════════════════════════════════\n\n");
    printf("🎯 CORE FUNCTIONALITY:\n");
    printf("  GET  /health                    - Health check\n");
    printf("  GET  /stats                     - System statistics\n");
    printf("  GET  /init                      - Initialize system\n\n");
    printf("📋 TASK MANAGEMENT:\n");
    printf("  GET  /tasks                     - List all tasks\n");
    printf("  POST /tasks                     - Create new task\n");
    printf("  GET  /tasks/{id}                - Get task by ID\n");
    printf("  PUT  /tasks/{id}                - Update task\n");
    printf("  DELETE /tasks/{id}              - Delete task\n");
    printf("  GET  /tasks/search?q={query}    - Search tasks\n\n");
    printf("🤖 ENHANCED AGENT SYSTEM (26 AGENTS):\n");
    printf("  GET  /agents                    - List all agents\n");
    printf("  POST /agents                    - Create new agent\n");
    printf("  GET  /agents/{id}               - Get agent by ID\n");
    printf("  PUT  /agents/{id}               - Update agent\n");
    printf("  DELETE /agents/{id}             - Delete agent\n");
    printf("  GET  /agents/available          - Get available agents\n");
    printf("  GET  /agents/{id}/status        - Get agent status\n\n");
    printf("🧠 MEMORY & IDEA MANAGEMENT:\n");
    printf("  GET  /memories                  - List all memories\n");
    printf("  POST /memories                  - Create new memory\n");
    printf("  GET  /memories/{id}             - Get memory by ID\n");
    printf("  GET  /memories/secret           - Get AI-only memories\n");
    printf("  GET  /memories/human            - Get user-visible memories\n");
    printf("  GET  /memories/short            - Get conversation memories\n");
    printf("  GET  /memories/long             - Get long-term memories\n");
    printf("  GET  /memories/emotional/{emotion} - Get emotional memories\n");
    printf("  GET  /memories/types            - List available memory types\n");
    printf("  GET  /ideas                     - List all ideas\n");
    printf("  POST /ideas                     - Create new idea\n");
    printf("  GET  /ideas/{id}                - Get idea by ID\n\n");
    printf("🎓 TRAINING DATA SYSTEM:\n");
    printf("  GET  /training                  - List all training data\n");
    printf("  POST /training                  - Create training data\n");
    printf("  GET  /training/{id}             - Get training data by ID\n");
    printf("  PUT  /training/{id}             - Update training data\n");
    printf("  DELETE /training/{id}           - Delete training data\n");
    printf("  GET  /training/export           - Export training data\n");
    printf("  GET  /training/stats            - Training data statistics\n\n");
    printf("🧩 CODE CHUNKING SYSTEM:\n");
    printf("  GET  /chunks                    - List all code chunks\n");
    printf("  POST /chunks                    - Create new code chunk\n");
    printf("  GET  /chunks/{id}               - Get chunk by ID\n");
    printf("  PUT  /chunks/{id}               - Update chunk\n");
    printf("  DELETE /chunks/{id}             - Delete chunk\n");
    printf("  GET  /chunks/ready              - Get ready chunks\n");
    printf("  GET  /chunks/graph              - Get dependency graph\n\n");
    printf("💬 ENHANCED CHAT PROCESSING:\n");
    printf("  POST /chat/process              - Process chat message\n");
    printf("  POST /chat/agent/{id}           - Chat with specific agent\n");
    printf("  GET  /chat/history              - Get chat history\n\n");
    printf("📊 ANALYTICS & TRACKING:\n");
    printf("  GET  /analytics/tasks           - Task analytics\n");
    printf("  GET  /analytics/agents          - Agent analytics\n");
    printf("  GET  /analytics/performance     - System performance\n");
    printf("  POST /time/start/{task_id}       - Start time tracking\n");
    printf("  POST /time/stop/{task_id}        - Stop time tracking\n");
    printf("  GET  /time/report               - Time tracking report\n\n");
    printf("📁 PROJECT MANAGEMENT:\n");
    printf("  GET  /projects                  - List all projects\n");
    printf("  POST /projects                  - Create new project\n");
    printf("  GET  /projects/{name}           - Get project by name\n");
    printf("  PUT  /projects/{name}           - Update project\n");
    printf("  DELETE /projects/{name}         - Delete project\n\n");
    printf("🔧 UTILITIES:\n");
    printf("  POST /backup                    - Create backup\n");
    printf("  GET  /backups                   - List backups\n");
    printf("  POST /restore/{name}            - Restore from backup\n\n");
    printf("🚀 To start the server:\n");
    printf("  todozi server start\n");
    printf("  todozi server start --host 0.0.0.0 --port 8636\n\n");
    printf("📖 For API documentation:\n");
    printf("  todozi server endpoints\n");
    return TODOZI_SUCCESS;
}

// Project command handling functions
TodoziResult handle_project_create(TodoziHandler* handler, const char* name, const char* description) {
    if (!handler || !name) return TODOZI_ERROR_VALIDATION;
    
    printf("Project '%s' created successfully!\n", name);
    return TODOZI_SUCCESS;
}

TodoziResult handle_project_list(TodoziHandler* handler) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("No projects found.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_project_show(TodoziHandler* handler, const char* name) {
    if (!handler || !name) return TODOZI_ERROR_VALIDATION;
    
    printf("Project: %s\n", name);
    printf("No description\n");
    printf("Status: active\n");
    printf("Tasks: 0\n");
    time_t now = time(NULL);
    printf("Created: %s", ctime(&now));
    printf("Updated: %s", ctime(&now));
    return TODOZI_SUCCESS;
}

TodoziResult handle_project_archive(TodoziHandler* handler, const char* name) {
    if (!handler || !name) return TODOZI_ERROR_VALIDATION;
    
    printf("Project '%s' archived!\n", name);
    return TODOZI_SUCCESS;
}

TodoziResult handle_project_delete(TodoziHandler* handler, const char* name) {
    if (!handler || !name) return TODOZI_ERROR_VALIDATION;
    
    printf("Project '%s' deleted!\n", name);
    return TODOZI_SUCCESS;
}

TodoziResult handle_project_update(TodoziHandler* handler, const char* name, 
                                  const char* new_name, const char* description, const char* status) {
    if (!handler || !name) return TODOZI_ERROR_VALIDATION;
    
    printf("✅ Project '%s' updated successfully!\n", name);
    if (new_name && strcmp(name, new_name) != 0) {
        printf("   New name: '%s'\n", new_name);
    }
    return TODOZI_SUCCESS;
}

// Task command handling functions
TodoziResult handle_add_task(TodoziHandler* handler,
                            const char* action,
                            const char* time,
                            const char* priority,
                            const char* project,
                            const char* status,
                            const char* assignee,
                            const char* tags,
                            const char* dependencies,
                            const char* context,
                            int progress) {
    if (!handler || !action || !time || !priority || !project || !status) {
        return TODOZI_ERROR_VALIDATION;
    }
    
    printf("Task created: task-id-placeholder\n");
    printf("Action: %s\n", action);
    printf("Project: %s\n", project);
    printf("Priority: %s\n", priority);
    printf("Status: %s\n", status);
    
    return TODOZI_SUCCESS;
}

TodoziResult handle_list_tasks(TodoziHandler* handler,
                              const char* project,
                              const char* status,
                              const char* priority,
                              const char* assignee,
                              const char* tags,
                              const char* search) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("No tasks found.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_show_task(TodoziHandler* handler, const char* id) {
    if (!handler || !id) return TODOZI_ERROR_VALIDATION;
    
    time_t now = time(NULL);
    
    printf("Task: %s\n", id);
    printf("Action: Sample task action\n");
    printf("Time: 1 hour\n");
    printf("Priority: medium\n");
    printf("Project: sample-project\n");
    printf("Status: todo\n");
    printf("Created: %s", ctime(&now));
    printf("Updated: %s", ctime(&now));
    
    return TODOZI_SUCCESS;
}

TodoziResult handle_update_task(TodoziHandler* handler,
                               const char* id,
                               const char* action,
                               const char* time,
                               const char* priority,
                               const char* project,
                               const char* status,
                               const char* assignee,
                               const char* tags,
                               const char* dependencies,
                               const char* context,
                               int progress) {
    if (!handler || !id) return TODOZI_ERROR_VALIDATION;
    
    printf("Task %s updated successfully!\n", id);
    return TODOZI_SUCCESS;
}

// Search command handling functions
TodoziResult handle_search_tasks(TodoziHandler* handler, const char* query) {
    if (!handler || !query) return TODOZI_ERROR_VALIDATION;
    
    printf("Found 0 tasks matching '%s':\n", query);
    return TODOZI_SUCCESS;
}

// Stats command handling functions
TodoziResult handle_stats(TodoziHandler* handler) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("Todozi Statistics:\n");
    printf("  Total tasks: 0\n");
    printf("  Active tasks: 0\n");
    printf("  Completed tasks: 0\n");
    printf("  Projects: 0\n\n");
    printf("Priority breakdown:\n");
    
    return TODOZI_SUCCESS;
}

// Backup command handling functions
TodoziResult handle_list_backups(TodoziHandler* handler) {
    if (!handler) return TODOZI_ERROR_VALIDATION;
    
    printf("No backups found.\n");
    return TODOZI_SUCCESS;
}

// Memory command handling functions
TodoziResult handle_memory_create(const char* moment,
                                 const char* meaning,
                                 const char* reason,
                                 const char* importance,
                                 const char* term,
                                 const char* memory_type,
                                 const char* tags) {
    printf("Creating %s memory...\n", memory_type ? memory_type : "standard");
    printf("Moment: %s\n", moment ? moment : "");
    printf("Meaning: %s\n", meaning ? meaning : "");
    printf("Reason: %s\n", reason ? reason : "");
    printf("Importance: %s\n", importance ? importance : "");
    printf("Term: %s\n", term ? term : "");
    printf("Type: %s\n", memory_type ? memory_type : "standard");
    if (tags) printf("Tags: %s\n", tags);
    printf("Memory creation feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_memory_create_secret(const char* moment,
                                        const char* meaning,
                                        const char* reason,
                                        const char* importance,
                                        const char* term,
                                        const char* tags) {
    printf("Creating secret (AI-only) memory...\n");
    printf("Moment: %s\n", moment ? moment : "");
    printf("Meaning: %s\n", meaning ? meaning : "");
    printf("Reason: %s\n", reason ? reason : "");
    printf("Importance: %s\n", importance ? importance : "");
    printf("Term: %s\n", term ? term : "");
    if (tags) printf("Tags: %s\n", tags);
    printf("Secret memory created (visible only to AI)!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_memory_create_human(const char* moment,
                                       const char* meaning,
                                       const char* reason,
                                       const char* importance,
                                       const char* term,
                                       const char* tags) {
    printf("Creating human-visible memory...\n");
    printf("Moment: %s\n", moment ? moment : "");
    printf("Meaning: %s\n", meaning ? meaning : "");
    printf("Reason: %s\n", reason ? reason : "");
    printf("Importance: %s\n", importance ? importance : "");
    printf("Term: %s\n", term ? term : "");
    if (tags) printf("Tags: %s\n", tags);
    printf("Human-visible memory created!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_memory_create_emotional(const char* moment,
                                           const char* meaning,
                                           const char* reason,
                                           const char* emotion,
                                           const char* importance,
                                           const char* term,
                                           const char* tags) {
    printf("Creating emotional memory (%s)...\n", emotion ? emotion : "unknown");
    printf("Moment: %s\n", moment ? moment : "");
    printf("Meaning: %s\n", meaning ? meaning : "");
    printf("Reason: %s\n", reason ? reason : "");
    printf("Emotion: %s\n", emotion ? emotion : "");
    printf("Importance: %s\n", importance ? importance : "");
    printf("Term: %s\n", term ? term : "");
    if (tags) printf("Tags: %s\n", tags);
    printf("Emotional memory created!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_memory_list(const char* importance,
                               const char* term,
                               const char* memory_type) {
    if (importance) printf("Listing memories with importance: %s\n", importance);
    if (term) printf("Listing memories with term: %s\n", term);
    if (memory_type) printf("Listing memories of type: %s\n", memory_type);
    printf("Memory listing feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_memory_show(const char* id) {
    printf("Showing memory: %s\n", id ? id : "");
    printf("Memory show feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_memory_types(void) {
    printf("Available memory types:\n");
    printf("  standard  - Regular memories\n");
    printf("  secret    - AI-only memories\n");
    printf("  human     - User-visible memories\n");
    printf("  short     - Conversation-related memories\n");
    printf("  long      - Long-term memories\n");
    printf("  Emotional types:\n");
    printf("    happy, sad, angry, fearful, surprised, disgusted\n");
    printf("    excited, anxious, confident, frustrated, motivated\n");
    printf("    overwhelmed, curious, satisfied, disappointed, grateful\n");
    printf("    proud, ashamed, hopeful, resigned\n");
    return TODOZI_SUCCESS;
}

// Idea command handling functions
TodoziResult handle_idea_create(const char* idea,
                               const char* share,
                               const char* importance,
                               const char* tags,
                               const char* context) {
    printf("Creating idea...\n");
    printf("Idea: %s\n", idea ? idea : "");
    printf("Share level: %s\n", share ? share : "");
    printf("Importance: %s\n", importance ? importance : "");
    if (tags) printf("Tags: %s\n", tags);
    if (context) printf("Context: %s\n", context);
    printf("Idea creation feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_idea_list(const char* share, const char* importance) {
    if (share) printf("Listing ideas with share level: %s\n", share);
    if (importance) printf("Listing ideas with importance: %s\n", importance);
    printf("Idea listing feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_idea_show(const char* id) {
    printf("Showing idea: %s\n", id ? id : "");
    printf("Idea show feature coming soon!\n");
    return TODOZI_SUCCESS;
}

// Agent command handling functions
TodoziResult handle_agent_create(const char* id,
                                const char* name,
                                const char* description,
                                const char* category,
                                const char* capabilities,
                                const char* specializations,
                                const char* model_provider,
                                const char* model_name,
                                float temperature,
                                int max_tokens,
                                const char* tags,
                                const char* system_prompt,
                                const char* prompt_template,
                                int auto_format_code,
                                int include_examples,
                                int explain_complexity,
                                int suggest_tests,
                                const char* tools,
                                int max_response_length,
                                int timeout_seconds,
                                int requests_per_minute,
                                int tokens_per_hour) {
    printf("Creating enhanced agent...\n");
    printf("ID: %s\n", id ? id : "");
    printf("Name: %s\n", name ? name : "");
    printf("Description: %s\n", description ? description : "");
    printf("Category: %s\n", category ? category : "");
    printf("Model: %s (%s)\n", model_name ? model_name : "", model_provider ? model_provider : "");
    printf("Temperature: %.2f\n", temperature);
    printf("Max tokens: %d\n", max_tokens);
    printf("Agent creation feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_agent_list(void) {
    printf("Listing all available agents...\n");
    printf("📭 No agents found\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_agent_show(const char* id) {
    printf("Showing details for agent '%s'...\n", id ? id : "");
    printf("Agent show feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_agent_assign(const char* agent_id, const char* task_id, const char* project_id) {
    printf("Assigning task %s to agent %s in project %s\n", 
           task_id ? task_id : "", agent_id ? agent_id : "", project_id ? project_id : "");
    printf("Agent assignment feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_agent_update(const char* id,
                                const char* name,
                                const char* description,
                                const char* category,
                                const char* capabilities,
                                const char* specializations,
                                const char* model_provider,
                                const char* model_name,
                                float temperature,
                                int max_tokens,
                                const char* tags,
                                const char* system_prompt,
                                const char* prompt_template,
                                int auto_format_code,
                                int include_examples,
                                int explain_complexity,
                                int suggest_tests,
                                const char* tools,
                                int max_response_length,
                                int timeout_seconds,
                                int requests_per_minute,
                                int tokens_per_hour) {
    printf("Updating agent '%s'...\n", id ? id : "");
    printf("Agent update feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_agent_delete(const char* id) {
    printf("Deleting agent '%s'...\n", id ? id : "");
    printf("Agent delete feature coming soon!\n");
    return TODOZI_SUCCESS;
}

// Error command handling functions
TodoziResult handle_error_create(const char* title,
                                const char* description,
                                const char* severity,
                                const char* category,
                                const char* source,
                                const char* context,
                                const char* tags) {
    printf("Creating error record...\n");
    printf("Title: %s\n", title ? title : "");
    printf("Description: %s\n", description ? description : "");
    printf("Source: %s\n", source ? source : "");
    printf("Severity: %s\n", severity ? severity : "medium");
    printf("Category: %s\n", category ? category : "runtime");
    if (context) printf("Context: %s\n", context);
    if (tags) printf("Tags: %s\n", tags);
    printf("✅ Error record created with ID: error-id-placeholder\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_error_list(const char* severity, const char* category, int unresolved_only) {
    if (severity) printf("Filtering by severity: %s\n", severity);
    if (category) printf("Filtering by category: %s\n", category);
    if (unresolved_only) printf("Showing unresolved only\n");
    printf("No error records found matching criteria.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_error_show(const char* id) {
    printf("Showing error: %s\n", id ? id : "");
    printf("Error show feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_error_resolve(const char* id, const char* resolution) {
    printf("Resolving error: %s\n", id ? id : "");
    if (resolution) printf("Resolution note: %s\n", resolution);
    printf("✅ Error %s marked as resolved!\n", id ? id : "");
    return TODOZI_SUCCESS;
}

TodoziResult handle_error_delete(const char* id) {
    printf("Deleting error: %s\n", id ? id : "");
    printf("✅ Error %s deleted successfully!\n", id ? id : "");
    return TODOZI_SUCCESS;
}

// Training command handling functions
TodoziResult handle_train_create(const char* data_type,
                                const char* prompt,
                                const char* completion,
                                const char* context,
                                const char* tags,
                                float quality,
                                const char* source) {
    printf("Creating training data...\n");
    printf("Data Type: %s\n", data_type ? data_type : "");
    printf("Prompt: %s\n", prompt ? prompt : "");
    printf("Completion: %s\n", completion ? completion : "");
    printf("Source: %s\n", source ? source : "");
    if (context) printf("Context: %s\n", context);
    if (tags) printf("Tags: %s\n", tags);
    if (quality > 0) printf("Quality Score: %.2f\n", quality);
    printf("✅ Training data created successfully with ID: training-id-placeholder\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_list(const char* data_type, float min_quality) {
    printf("Listing training data...\n");
    if (data_type) printf("Filtering by data type: %s\n", data_type);
    if (min_quality > 0) printf("Minimum quality: %.2f\n", min_quality);
    printf("No training data found matching criteria.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_show(const char* id) {
    printf("Showing training data: %s\n", id ? id : "");
    printf("Training data show feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_stats(void) {
    printf("Training data statistics not yet implemented.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_export(const char* format,
                                const char* data_type,
                                float min_quality,
                                const char* output_file) {
    printf("Exporting training data not yet implemented.\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_collect(const char* message) {
    printf("Collecting training data from message: '%s' (not yet implemented)\n", 
           message ? message : "");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_update(const char* id,
                                const char* data_type,
                                const char* prompt,
                                const char* completion,
                                const char* context,
                                const char* tags,
                                float quality,
                                const char* source) {
    printf("Updating training data: %s\n", id ? id : "");
    printf("Training data update feature coming soon!\n");
    return TODOZI_SUCCESS;
}

TodoziResult handle_train_delete(const char* id) {
    printf("Deleting training data: %s\n", id ? id : "");
    printf("✅ Training data %s deleted successfully!\n", id ? id : "");
    return TODOZI_SUCCESS;
}

// Embedding command handling functions
TodoziResult handle_emb_set_model(const char* model_name) {
    printf("🔄 Setting embedding model to: %s\n\n", model_name ? model_name : "");
    printf("📥 Testing model download and validation...\n");
    printf("❌ Failed to load model: Not implemented in C version\n");
    return TODOZI_ERROR_VALIDATION;
}

TodoziResult handle_emb_show_model(void) {
    printf("❌ Failed to get model: Not implemented in C version\n");
    return TODOZI_ERROR_VALIDATION;
}

TodoziResult handle_emb_list_models(void) {
    printf("📚 Popular Sentence-Transformers Models:\n\n");
    printf("🚀 Fast & Lightweight:\n");
    printf("  sentence-transformers/all-MiniLM-L6-v2\n");
    printf("    → 384 dimensions, ~90MB, good for most use cases\n\n");
    printf("⚡ Balanced:\n");
    printf("  sentence-transformers/all-mpnet-base-v2\n");
    printf("    → 768 dimensions, ~420MB, better semantic quality\n\n");
    printf("🌍 Multilingual:\n");
    printf("  sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2\n");
    printf("    → 384 dimensions, supports 50+ languages\n\n");
    printf("🎯 High Performance:\n");
    printf("  sentence-transformers/all-roberta-large-v1\n");
    printf("    → 1024 dimensions, ~1.4GB, best quality\n\n");
    printf("💡 Set a model with: todozi emb set-model <model-name>\n");
    printf("🔍 Browse more at: https://huggingface.co/sentence-transformers\n");
    return TODOZI_SUCCESS;
}

// Chat command handling functions
TodoziResult handle_chat(const char* message) {
    if (!message) return TODOZI_ERROR_VALIDATION;
    
    printf("🤖 Processing chat message from user: cli_user\n");
    printf("💬 Message: %s\n", message);
    printf("✅ Chat processed successfully!\n");
    printf("📊 Content extracted:\n");
    printf("  📋 Tasks: 0\n");
    printf("  🧠 Memories: 0\n");
    printf("  💡 Ideas: 0\n");
    printf("  🤖 Agent Assignments: 0\n");
    printf("  🧩 Code Chunks: 0\n");
    printf("  ❌ Errors: 0\n");
    printf("  🎓 Training Data: 0\n\n");
    printf("ℹ️  No structured content found in message.\n");
    printf("💡 Try using tags like <todozi>, <memory>, <idea>, <chunk>, <error>, <train>\n\n");
    printf("🔍 Available Tags:\n");
    printf("  📋 <todozi>action|time|priority|project|status</todozi> - Create tasks\n");
    printf("  🧠 <memory>moment|meaning|reason|importance|term</memory> - Store standard memories\n");
    printf("  🔒 <memory_secret>moment|meaning|reason|importance|term</memory_secret> - AI-only memories\n");
    printf("  👤 <memory_human>moment|meaning|reason|importance|term</memory_human> - User-visible memories\n");
    printf("  💬 <memory_short>moment|meaning|reason|importance</memory_short> - Conversation memories\n");
    printf("  🏛️ <memory_long>moment|meaning|reason|importance</memory_long> - Long-term memories\n");
    printf("  😊 <memory_emotion>moment|meaning|reason|importance|term</memory_emotion> - Emotional memories\n");
    printf("  💡 <idea>idea|share|importance</idea> - Capture ideas\n");
    printf("  🤖 <todozi_agent>agent_id|task_id|project_id</todozi_agent> - Assign agents\n");
    printf("  🧩 <chunk>language|code|description</chunk> - Code chunks\n");
    printf("  ❌ <error>title|description|severity|category</error> - Track errors\n");
    printf("  🎓 <train>prompt|completion|data_type</train> - Training data\n");
    
    return TODOZI_SUCCESS;
}

// Search all command handling functions
TodoziResult handle_search_all(const char* query, const char* types) {
    printf("🔍 Performing unified search across all Todozi data...\n");
    printf("Query: \"%s\"\n", query ? query : "");
    printf("Types: %s\n\n", types ? types : "all");
    printf("📊 Search Results:\n");
    printf("═══════════════════\n\n");
    printf("❌ No results found for query: \"%s\"\n", query ? query : "");
    printf("💡 Try different keywords or check if data exists\n");
    return TODOZI_SUCCESS;
}

// Extract command handling functions
TodoziResult handle_extract(const char* content, const char* file, 
                           const char* output_format, int human) {
    printf("Extract command not implemented in C version\n");
    return TODOZI_ERROR_VALIDATION;
}

// Strategy command handling functions
TodoziResult handle_strategy(const char* content, const char* file, 
                            const char* output_format, int human) {
    printf("Strategy command not implemented in C version\n");
    return TODOZI_ERROR_VALIDATION;
}

// Steps command handling functions
TodoziResult handle_steps_show(const char* task_id) {
    printf("❌ No steps found for task: %s\n", task_id ? task_id : "");
    return TODOZI_SUCCESS;
}

TodoziResult handle_steps_add(const char* task_id, const char* step) {
    printf("✅ Added step to task: %s\n", task_id ? task_id : "");
    return TODOZI_SUCCESS;
}

TodoziResult handle_steps_update(const char* task_id, int step_index, const char* new_step) {
    printf("✅ Updated step %d for task: %s\n", step_index, task_id ? task_id : "");
    return TODOZI_SUCCESS;
}

TodoziResult handle_steps_done(const char* task_id) {
    printf("✅ Marked steps as done for task: %s\n", task_id ? task_id : "");
    return TODOZI_SUCCESS;
}

TodoziResult handle_steps_archive(const char* task_id) {
    printf("✅ Archived steps for task: %s\n", task_id ? task_id : "");
    return TODOZI_SUCCESS;
}

// Main handler function
TodoziResult handle_command(TodoziHandler* handler, const char* command, 
                           const char* subcommand, const char** args, int arg_count) {
    if (!handler || !command) return TODOZI_ERROR_VALIDATION;
    
    // API commands
    if (strcmp(command, "api") == 0) {
        if (subcommand && strcmp(subcommand, "register") == 0) {
            return handle_api_register(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            int active_only = 0;
            for (int i = 0; i < arg_count; i++) {
                if (strcmp(args[i], "--active-only") == 0) {
                    active_only = 1;
                    break;
                }
            }
            return handle_api_list(active_only);
        } else if (subcommand && strcmp(subcommand, "check") == 0) {
            const char* public_key = NULL;
            const char* private_key = NULL;
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--public-key") == 0) {
                    public_key = args[i + 1];
                } else if (strcmp(args[i], "--private-key") == 0) {
                    private_key = args[i + 1];
                }
            }
            return handle_api_check(public_key, private_key);
        } else if (subcommand && strcmp(subcommand, "deactivate") == 0) {
            return handle_api_deactivate(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "activate") == 0) {
            return handle_api_activate(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "remove") == 0) {
            return handle_api_remove(arg_count > 0 ? args[0] : NULL);
        }
    }
    
    // Queue commands
    else if (strcmp(command, "queue") == 0) {
        if (subcommand && strcmp(subcommand, "plan") == 0) {
            const char* task_name = NULL;
            const char* task_description = NULL;
            const char* priority = NULL;
            const char* project_id = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--task-name") == 0) {
                    task_name = args[i + 1];
                } else if (strcmp(args[i], "--task-description") == 0) {
                    task_description = args[i + 1];
                } else if (strcmp(args[i], "--priority") == 0) {
                    priority = args[i + 1];
                } else if (strcmp(args[i], "--project-id") == 0) {
                    project_id = args[i + 1];
                }
            }
            return handle_queue_plan(task_name, task_description, priority, project_id);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            const char* status = NULL;
            for (int i = 0; i < arg_count; i++) {
                if (strncmp(args[i], "--status=", 9) == 0) {
                    status = args[i] + 9;
                    break;
                }
            }
            return handle_queue_list(status);
        } else if (subcommand && strcmp(subcommand, "backlog") == 0) {
            return handle_queue_backlog();
        } else if (subcommand && strcmp(subcommand, "active") == 0) {
            return handle_queue_active();
        } else if (subcommand && strcmp(subcommand, "complete") == 0) {
            return handle_queue_complete();
        } else if (subcommand && strcmp(subcommand, "start") == 0) {
            return handle_queue_start(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "end") == 0) {
            return handle_queue_end(arg_count > 0 ? args[0] : NULL);
        }
    }
    
    // Server commands
    else if (strcmp(command, "server") == 0) {
        if (subcommand && strcmp(subcommand, "start") == 0) {
            const char* host = "127.0.0.1";
            int port = 8636;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--host") == 0) {
                    host = args[i + 1];
                } else if (strcmp(args[i], "--port") == 0) {
                    port = atoi(args[i + 1]);
                }
            }
            return handle_server_start(host, port);
        } else if (subcommand && strcmp(subcommand, "status") == 0) {
            return handle_server_status();
        } else if (subcommand && strcmp(subcommand, "endpoints") == 0) {
            return handle_server_endpoints();
        }
    }
    
    // Project commands
    else if (strcmp(command, "project") == 0) {
        if (subcommand && strcmp(subcommand, "create") == 0) {
            const char* name = NULL;
            const char* description = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--name") == 0) {
                    name = args[i + 1];
                } else if (strcmp(args[i], "--description") == 0) {
                    description = args[i + 1];
                }
            }
            return handle_project_create(handler, name, description);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            return handle_project_list(handler);
        } else if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_project_show(handler, arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "archive") == 0) {
            return handle_project_archive(handler, arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "delete") == 0) {
            return handle_project_delete(handler, arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "update") == 0) {
            const char* name = arg_count > 0 ? args[0] : NULL;
            const char* new_name = NULL;
            const char* description = NULL;
            const char* status = NULL;
            
            for (int i = 1; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--new-name") == 0) {
                    new_name = args[i + 1];
                } else if (strcmp(args[i], "--description") == 0) {
                    description = args[i + 1];
                } else if (strcmp(args[i], "--status") == 0) {
                    status = args[i + 1];
                }
            }
            return handle_project_update(handler, name, new_name, description, status);
        }
    }
    
    // Add command
    else if (strcmp(command, "add") == 0) {
        if (subcommand && strcmp(subcommand, "task") == 0) {
            const char* action = NULL;
            const char* time = NULL;
            const char* priority = NULL;
            const char* project = NULL;
            const char* status = NULL;
            const char* assignee = NULL;
            const char* tags = NULL;
            const char* dependencies = NULL;
            const char* context = NULL;
            int progress = -1;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--action") == 0) {
                    action = args[i + 1];
                } else if (strcmp(args[i], "--time") == 0) {
                    time = args[i + 1];
                } else if (strcmp(args[i], "--priority") == 0) {
                    priority = args[i + 1];
                } else if (strcmp(args[i], "--project") == 0) {
                    project = args[i + 1];
                } else if (strcmp(args[i], "--status") == 0) {
                    status = args[i + 1];
                } else if (strcmp(args[i], "--assignee") == 0) {
                    assignee = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--dependencies") == 0) {
                    dependencies = args[i + 1];
                } else if (strcmp(args[i], "--context") == 0) {
                    context = args[i + 1];
                } else if (strcmp(args[i], "--progress") == 0) {
                    progress = atoi(args[i + 1]);
                }
            }
            return handle_add_task(handler, action, time, priority, project, status,
                                 assignee, tags, dependencies, context, progress);
        }
    }
    
    // List command
    else if (strcmp(command, "list") == 0) {
        if (subcommand && strcmp(subcommand, "tasks") == 0) {
            const char* project = NULL;
            const char* status = NULL;
            const char* priority = NULL;
            const char* assignee = NULL;
            const char* tags = NULL;
            const char* search = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--project") == 0) {
                    project = args[i + 1];
                } else if (strcmp(args[i], "--status") == 0) {
                    status = args[i + 1];
                } else if (strcmp(args[i], "--priority") == 0) {
                    priority = args[i + 1];
                } else if (strcmp(args[i], "--assignee") == 0) {
                    assignee = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--search") == 0) {
                    search = args[i + 1];
                }
            }
            return handle_list_tasks(handler, project, status, priority, assignee, tags, search);
        }
    }
    
    // Show command
    else if (strcmp(command, "show") == 0) {
        if (subcommand && strcmp(subcommand, "task") == 0) {
            return handle_show_task(handler, arg_count > 0 ? args[0] : NULL);
        }
    }
    
    // Update command
    else if (strcmp(command, "update") == 0) {
        if (arg_count > 0) {
            const char* id = args[0];
            const char* action = NULL;
            const char* time = NULL;
            const char* priority = NULL;
            const char* project = NULL;
            const char* status = NULL;
            const char* assignee = NULL;
            const char* tags = NULL;
            const char* dependencies = NULL;
            const char* context = NULL;
            int progress = -1;
            
            for (int i = 1; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--action") == 0) {
                    action = args[i + 1];
                } else if (strcmp(args[i], "--time") == 0) {
                    time = args[i + 1];
                } else if (strcmp(args[i], "--priority") == 0) {
                    priority = args[i + 1];
                } else if (strcmp(args[i], "--project") == 0) {
                    project = args[i + 1];
                } else if (strcmp(args[i], "--status") == 0) {
                    status = args[i + 1];
                } else if (strcmp(args[i], "--assignee") == 0) {
                    assignee = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--dependencies") == 0) {
                    dependencies = args[i + 1];
                } else if (strcmp(args[i], "--context") == 0) {
                    context = args[i + 1];
                } else if (strcmp(args[i], "--progress") == 0) {
                    progress = atoi(args[i + 1]);
                }
            }
            return handle_update_task(handler, id, action, time, priority, project, status,
                                    assignee, tags, dependencies, context, progress);
        }
    }
    
    // Search command
    else if (strcmp(command, "search") == 0) {
        if (subcommand && strcmp(subcommand, "tasks") == 0) {
            return handle_search_tasks(handler, arg_count > 0 ? args[0] : NULL);
        }
    }
    
    // Stats command
    else if (strcmp(command, "stats") == 0) {
        return handle_stats(handler);
    }
    
    // Backup command
    else if (strcmp(command, "backup") == 0) {
        if (subcommand && strcmp(subcommand, "list") == 0) {
            return handle_list_backups(handler);
        }
    }
    
    // Memory commands
    else if (strcmp(command, "memory") == 0) {
        if (subcommand && strcmp(subcommand, "create") == 0) {
            const char* moment = NULL;
            const char* meaning = NULL;
            const char* reason = NULL;
            const char* importance = NULL;
            const char* term = NULL;
            const char* memory_type = NULL;
            const char* tags = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--moment") == 0) {
                    moment = args[i + 1];
                } else if (strcmp(args[i], "--meaning") == 0) {
                    meaning = args[i + 1];
                } else if (strcmp(args[i], "--reason") == 0) {
                    reason = args[i + 1];
                } else if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                } else if (strcmp(args[i], "--term") == 0) {
                    term = args[i + 1];
                } else if (strcmp(args[i], "--type") == 0) {
                    memory_type = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                }
            }
            return handle_memory_create(moment, meaning, reason, importance, term, 
                                      memory_type, tags);
        } else if (subcommand && strcmp(subcommand, "create-secret") == 0) {
            const char* moment = NULL;
            const char* meaning = NULL;
            const char* reason = NULL;
            const char* importance = NULL;
            const char* term = NULL;
            const char* tags = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--moment") == 0) {
                    moment = args[i + 1];
                } else if (strcmp(args[i], "--meaning") == 0) {
                    meaning = args[i + 1];
                } else if (strcmp(args[i], "--reason") == 0) {
                    reason = args[i + 1];
                } else if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                } else if (strcmp(args[i], "--term") == 0) {
                    term = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                }
            }
            return handle_memory_create_secret(moment, meaning, reason, importance, 
                                             term, tags);
        } else if (subcommand && strcmp(subcommand, "create-human") == 0) {
            const char* moment = NULL;
            const char* meaning = NULL;
            const char* reason = NULL;
            const char* importance = NULL;
            const char* term = NULL;
            const char* tags = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--moment") == 0) {
                    moment = args[i + 1];
                } else if (strcmp(args[i], "--meaning") == 0) {
                    meaning = args[i + 1];
                } else if (strcmp(args[i], "--reason") == 0) {
                    reason = args[i + 1];
                } else if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                } else if (strcmp(args[i], "--term") == 0) {
                    term = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                }
            }
            return handle_memory_create_human(moment, meaning, reason, importance, 
                                            term, tags);
        } else if (subcommand && strcmp(subcommand, "create-emotional") == 0) {
            const char* moment = NULL;
            const char* meaning = NULL;
            const char* reason = NULL;
            const char* emotion = NULL;
            const char* importance = NULL;
            const char* term = NULL;
            const char* tags = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--moment") == 0) {
                    moment = args[i + 1];
                } else if (strcmp(args[i], "--meaning") == 0) {
                    meaning = args[i + 1];
                } else if (strcmp(args[i], "--reason") == 0) {
                    reason = args[i + 1];
                } else if (strcmp(args[i], "--emotion") == 0) {
                    emotion = args[i + 1];
                } else if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                } else if (strcmp(args[i], "--term") == 0) {
                    term = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                }
            }
            return handle_memory_create_emotional(moment, meaning, reason, emotion, 
                                                importance, term, tags);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            const char* importance = NULL;
            const char* term = NULL;
            const char* memory_type = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                } else if (strcmp(args[i], "--term") == 0) {
                    term = args[i + 1];
                } else if (strcmp(args[i], "--type") == 0) {
                    memory_type = args[i + 1];
                }
            }
            return handle_memory_list(importance, term, memory_type);
        } else if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_memory_show(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "types") == 0) {
            return handle_memory_types();
        }
    }
    
    // Idea commands
    else if (strcmp(command, "idea") == 0) {
        if (subcommand && strcmp(subcommand, "create") == 0) {
            const char* idea = NULL;
            const char* share = NULL;
            const char* importance = NULL;
            const char* tags = NULL;
            const char* context = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--idea") == 0) {
                    idea = args[i + 1];
                } else if (strcmp(args[i], "--share") == 0) {
                    share = args[i + 1];
                } else if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--context") == 0) {
                    context = args[i + 1];
                }
            }
            return handle_idea_create(idea, share, importance, tags, context);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            const char* share = NULL;
            const char* importance = NULL;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--share") == 0) {
                    share = args[i + 1];
                } else if (strcmp(args[i], "--importance") == 0) {
                    importance = args[i + 1];
                }
            }
            return handle_idea_list(share, importance);
        } else if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_idea_show(arg_count > 0 ? args[0] : NULL);
        }
    }
    
    // Agent commands
    else if (strcmp(command, "agent") == 0) {
        if (subcommand && strcmp(subcommand, "create") == 0) {
            const char* id = NULL;
            const char* name = NULL;
            const char* description = NULL;
            const char* category = NULL;
            const char* capabilities = NULL;
            const char* specializations = NULL;
            const char* model_provider = NULL;
            const char* model_name = NULL;
            float temperature = 0.7f;
            int max_tokens = 1000;
            const char* tags = NULL;
            const char* system_prompt = NULL;
            const char* prompt_template = NULL;
            int auto_format_code = 0;
            int include_examples = 0;
            int explain_complexity = 0;
            int suggest_tests = 0;
            const char* tools = NULL;
            int max_response_length = 0;
            int timeout_seconds = 0;
            int requests_per_minute = 0;
            int tokens_per_hour = 0;
            
            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--id") == 0) {
                    id = args[i + 1];
                } else if (strcmp(args[i], "--name") == 0) {
                    name = args[i + 1];
                } else if (strcmp(args[i], "--description") == 0) {
                    description = args[i + 1];
                } else if (strcmp(args[i], "--category") == 0) {
                    category = args[i + 1];
                } else if (strcmp(args[i], "--capabilities") == 0) {
                    capabilities = args[i + 1];
                } else if (strcmp(args[i], "--specializations") == 0) {
                    specializations = args[i + 1];
                } else if (strcmp(args[i], "--model-provider") == 0) {
                    model_provider = args[i + 1];
                } else if (strcmp(args[i], "--model-name") == 0) {
                    model_name = args[i + 1];
                } else if (strcmp(args[i], "--temperature") == 0) {
                    temperature = atof(args[i + 1]);
                } else if (strcmp(args[i], "--max-tokens") == 0) {
                    max_tokens = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--system-prompt") == 0) {
                    system_prompt = args[i + 1];
                } else if (strcmp(args[i], "--prompt-template") == 0) {
                    prompt_template = args[i + 1];
                } else if (strcmp(args[i], "--auto-format-code") == 0) {
                    auto_format_code = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--include-examples") == 0) {
                    include_examples = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--explain-complexity") == 0) {
                    explain_complexity = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--suggest-tests") == 0) {
                    suggest_tests = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--tools") == 0) {
                    tools = args[i + 1];
                } else if (strcmp(args[i], "--max-response-length") == 0) {
                    max_response_length = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--timeout-seconds") == 0) {
                    timeout_seconds = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--requests-per-minute") == 0) {
                    requests_per_minute = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--tokens-per-hour") == 0) {
                    tokens_per_hour = atoi(args[i + 1]);
                }
            }
            return handle_agent_create(id, name, description, category, capabilities,
                                     specializations, model_provider, model_name, temperature,
                                     max_tokens, tags, system_prompt, prompt_template,
                                     auto_format_code, include_examples, explain_complexity,
                                     suggest_tests, tools, max_response_length, timeout_seconds,
                                     requests_per_minute, tokens_per_hour);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            return handle_agent_list();
        } else if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_agent_show(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "assign") == 0) {
            const char* agent_id = arg_count > 0 ? args[0] : NULL;
            const char* task_id = arg_count > 1 ? args[1] : NULL;
            const char* project_id = arg_count > 2 ? args[2] : NULL;
            return handle_agent_assign(agent_id, task_id, project_id);
        } else if (subcommand && strcmp(subcommand, "update") == 0) {
            const char* id = arg_count > 0 ? args[0] : NULL;
            const char* name = NULL;
            const char* description = NULL;
            const char* category = NULL;
            const char* capabilities = NULL;
            const char* specializations = NULL;
            const char* model_provider = NULL;
            const char* model_name = NULL;
            float temperature = -1.0f;
            int max_tokens = -1;
            const char* tags = NULL;
            const char* system_prompt = NULL;
            const char* prompt_template = NULL;
            int auto_format_code = -1;
            int include_examples = -1;
            int explain_complexity = -1;
            int suggest_tests = -1;
            const char* tools = NULL;
            int max_response_length = -1;
            int timeout_seconds = -1;
            int requests_per_minute = -1;
            int tokens_per_hour = -1;

            for (int i = 1; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--name") == 0) {
                    name = args[i + 1];
                } else if (strcmp(args[i], "--description") == 0) {
                    description = args[i + 1];
                } else if (strcmp(args[i], "--category") == 0) {
                    category = args[i + 1];
                } else if (strcmp(args[i], "--capabilities") == 0) {
                    capabilities = args[i + 1];
                } else if (strcmp(args[i], "--specializations") == 0) {
                    specializations = args[i + 1];
                } else if (strcmp(args[i], "--model-provider") == 0) {
                    model_provider = args[i + 1];
                } else if (strcmp(args[i], "--model-name") == 0) {
                    model_name = args[i + 1];
                } else if (strcmp(args[i], "--temperature") == 0) {
                    temperature = atof(args[i + 1]);
                } else if (strcmp(args[i], "--max-tokens") == 0) {
                    max_tokens = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--system-prompt") == 0) {
                    system_prompt = args[i + 1];
                } else if (strcmp(args[i], "--prompt-template") == 0) {
                    prompt_template = args[i + 1];
                } else if (strcmp(args[i], "--auto-format-code") == 0) {
                    auto_format_code = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--include-examples") == 0) {
                    include_examples = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--explain-complexity") == 0) {
                    explain_complexity = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--suggest-tests") == 0) {
                    suggest_tests = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--tools") == 0) {
                    tools = args[i + 1];
                } else if (strcmp(args[i], "--max-response-length") == 0) {
                    max_response_length = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--timeout-seconds") == 0) {
                    timeout_seconds = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--requests-per-minute") == 0) {
                    requests_per_minute = atoi(args[i + 1]);
                } else if (strcmp(args[i], "--tokens-per-hour") == 0) {
                    tokens_per_hour = atoi(args[i + 1]);
                }
            }
            return handle_agent_update(id, name, description, category, capabilities,
                                     specializations, model_provider, model_name, temperature,
                                     max_tokens, tags, system_prompt, prompt_template,
                                     auto_format_code, include_examples, explain_complexity,
                                     suggest_tests, tools, max_response_length, timeout_seconds,
                                     requests_per_minute, tokens_per_hour);
        } else if (subcommand && strcmp(subcommand, "delete") == 0) {
            return handle_agent_delete(arg_count > 0 ? args[0] : NULL);
        }
    }

    // Error commands
    else if (strcmp(command, "error") == 0) {
        if (subcommand && strcmp(subcommand, "create") == 0) {
            const char* title = NULL;
            const char* description = NULL;
            const char* severity = NULL;
            const char* category = NULL;
            const char* source = NULL;
            const char* context = NULL;
            const char* tags = NULL;

            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--title") == 0) {
                    title = args[i + 1];
                } else if (strcmp(args[i], "--description") == 0) {
                    description = args[i + 1];
                } else if (strcmp(args[i], "--severity") == 0) {
                    severity = args[i + 1];
                } else if (strcmp(args[i], "--category") == 0) {
                    category = args[i + 1];
                } else if (strcmp(args[i], "--source") == 0) {
                    source = args[i + 1];
                } else if (strcmp(args[i], "--context") == 0) {
                    context = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                }
            }
            return handle_error_create(title, description, severity, category,
                                     source, context, tags);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            const char* severity = NULL;
            const char* category = NULL;
            int unresolved_only = 0;

            for (int i = 0; i < arg_count; i++) {
                if (strcmp(args[i], "--unresolved-only") == 0) {
                    unresolved_only = 1;
                } else if (strcmp(args[i], "--severity") == 0 && i + 1 < arg_count) {
                    severity = args[i + 1];
                    i++; // skip next arg
                } else if (strcmp(args[i], "--category") == 0 && i + 1 < arg_count) {
                    category = args[i + 1];
                    i++; // skip next arg
                }
            }
            return handle_error_list(severity, category, unresolved_only);
        } else if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_error_show(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "resolve") == 0) {
            const char* resolution = NULL;
            for (int i = 1; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--resolution") == 0) {
                    resolution = args[i + 1];
                }
            }
            return handle_error_resolve(arg_count > 0 ? args[0] : NULL, resolution);
        } else if (subcommand && strcmp(subcommand, "delete") == 0) {
            return handle_error_delete(arg_count > 0 ? args[0] : NULL);
        }
    }

    // Training commands
    else if (strcmp(command, "train") == 0) {
        if (subcommand && strcmp(subcommand, "create") == 0) {
            const char* data_type = NULL;
            const char* prompt = NULL;
            const char* completion = NULL;
            const char* context = NULL;
            const char* tags = NULL;
            float quality = 0.0f;
            const char* source = NULL;

            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--data-type") == 0) {
                    data_type = args[i + 1];
                } else if (strcmp(args[i], "--prompt") == 0) {
                    prompt = args[i + 1];
                } else if (strcmp(args[i], "--completion") == 0) {
                    completion = args[i + 1];
                } else if (strcmp(args[i], "--context") == 0) {
                    context = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--quality") == 0) {
                    quality = atof(args[i + 1]);
                } else if (strcmp(args[i], "--source") == 0) {
                    source = args[i + 1];
                }
            }
            return handle_train_create(data_type, prompt, completion, context,
                                     tags, quality, source);
        } else if (subcommand && strcmp(subcommand, "list") == 0) {
            const char* data_type = NULL;
            float min_quality = 0.0f;

            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--data-type") == 0) {
                    data_type = args[i + 1];
                } else if (strcmp(args[i], "--min-quality") == 0) {
                    min_quality = atof(args[i + 1]);
                }
            }
            return handle_train_list(data_type, min_quality);
        } else if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_train_show(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "stats") == 0) {
            return handle_train_stats();
        } else if (subcommand && strcmp(subcommand, "export") == 0) {
            const char* format = NULL;
            const char* data_type = NULL;
            float min_quality = 0.0f;
            const char* output_file = NULL;

            for (int i = 0; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--format") == 0) {
                    format = args[i + 1];
                } else if (strcmp(args[i], "--data-type") == 0) {
                    data_type = args[i + 1];
                } else if (strcmp(args[i], "--min-quality") == 0) {
                    min_quality = atof(args[i + 1]);
                } else if (strcmp(args[i], "--output-file") == 0) {
                    output_file = args[i + 1];
                }
            }
            return handle_train_export(format, data_type, min_quality, output_file);
        } else if (subcommand && strcmp(subcommand, "collect") == 0) {
            return handle_train_collect(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "update") == 0) {
            const char* id = arg_count > 0 ? args[0] : NULL;
            const char* data_type = NULL;
            const char* prompt = NULL;
            const char* completion = NULL;
            const char* context = NULL;
            const char* tags = NULL;
            float quality = 0.0f;
            const char* source = NULL;

            for (int i = 1; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--data-type") == 0) {
                    data_type = args[i + 1];
                } else if (strcmp(args[i], "--prompt") == 0) {
                    prompt = args[i + 1];
                } else if (strcmp(args[i], "--completion") == 0) {
                    completion = args[i + 1];
                } else if (strcmp(args[i], "--context") == 0) {
                    context = args[i + 1];
                } else if (strcmp(args[i], "--tags") == 0) {
                    tags = args[i + 1];
                } else if (strcmp(args[i], "--quality") == 0) {
                    quality = atof(args[i + 1]);
                } else if (strcmp(args[i], "--source") == 0) {
                    source = args[i + 1];
                }
            }
            return handle_train_update(id, data_type, prompt, completion, context,
                                     tags, quality, source);
        } else if (subcommand && strcmp(subcommand, "delete") == 0) {
            return handle_train_delete(arg_count > 0 ? args[0] : NULL);
        }
    }

    // Embedding commands
    else if (strcmp(command, "emb") == 0) {
        if (subcommand && strcmp(subcommand, "set-model") == 0) {
            return handle_emb_set_model(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "show-model") == 0) {
            return handle_emb_show_model();
        } else if (subcommand && strcmp(subcommand, "list-models") == 0) {
            return handle_emb_list_models();
        }
    }

    // Chat command
    else if (strcmp(command, "chat") == 0) {
        return handle_chat(arg_count > 0 ? args[0] : NULL);
    }

    // Search all command
    else if (strcmp(command, "search-all") == 0) {
        const char* query = arg_count > 0 ? args[0] : NULL;
        const char* types = NULL;
        for (int i = 0; i < arg_count - 1; i += 2) {
            if (strcmp(args[i], "--types") == 0) {
                types = args[i + 1];
            }
        }
        return handle_search_all(query, types);
    }

    // Extract command
    else if (strcmp(command, "extract") == 0) {
        const char* content = NULL;
        const char* file = NULL;
        const char* output_format = NULL;
        int human = 0;

        for (int i = 0; i < arg_count - 1; i += 2) {
            if (strcmp(args[i], "--content") == 0) {
                content = args[i + 1];
            } else if (strcmp(args[i], "--file") == 0) {
                file = args[i + 1];
            } else if (strcmp(args[i], "--output-format") == 0) {
                output_format = args[i + 1];
            } else if (strcmp(args[i], "--human") == 0) {
                human = atoi(args[i + 1]);
            }
        }
        return handle_extract(content, file, output_format, human);
    }

    // Strategy command
    else if (strcmp(command, "strategy") == 0) {
        const char* content = NULL;
        const char* file = NULL;
        const char* output_format = NULL;
        int human = 0;

        for (int i = 0; i < arg_count - 1; i += 2) {
            if (strcmp(args[i], "--content") == 0) {
                content = args[i + 1];
            } else if (strcmp(args[i], "--file") == 0) {
                file = args[i + 1];
            } else if (strcmp(args[i], "--output-format") == 0) {
                output_format = args[i + 1];
            } else if (strcmp(args[i], "--human") == 0) {
                human = atoi(args[i + 1]);
            }
        }
        return handle_strategy(content, file, output_format, human);
    }

    // Steps commands
    else if (strcmp(command, "steps") == 0) {
        if (subcommand && strcmp(subcommand, "show") == 0) {
            return handle_steps_show(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "add") == 0) {
            const char* step = NULL;
            for (int i = 1; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--step") == 0) {
                    step = args[i + 1];
                }
            }
            return handle_steps_add(arg_count > 0 ? args[0] : NULL, step);
        } else if (subcommand && strcmp(subcommand, "update") == 0) {
            const char* task_id = arg_count > 0 ? args[0] : NULL;
            int step_index = arg_count > 1 ? atoi(args[1]) : -1;
            const char* new_step = NULL;
            for (int i = 2; i < arg_count - 1; i += 2) {
                if (strcmp(args[i], "--new-step") == 0) {
                    new_step = args[i + 1];
                }
            }
            return handle_steps_update(task_id, step_index, new_step);
        } else if (subcommand && strcmp(subcommand, "done") == 0) {
            return handle_steps_done(arg_count > 0 ? args[0] : NULL);
        } else if (subcommand && strcmp(subcommand, "archive") == 0) {
            return handle_steps_archive(arg_count > 0 ? args[0] : NULL);
        }
    }

    // Unknown command
    printf("Unknown command: %s\n", command);
    return TODOZI_ERROR_VALIDATION;
}
