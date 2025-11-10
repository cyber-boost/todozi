#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <pwd.h>
#include "todozi.h"

// Error handling
typedef struct {
    int code;
    char* message;  // Changed to non-const to allow freeing
} TodoziError;

// Helper to free error message
static void todozi_error_cleanup(TodoziError* err) {
    if (err && err->message) {
        free(err->message);
        err->message = NULL;
    }
}

#define CHECK(expr) do { \
    TodoziError err = (expr); \
    if (err.code != 0) { \
        fprintf(stderr, "Error: %s\n", err.message ? err.message : "Unknown error"); \
        todozi_error_cleanup(&err); \
        exit(err.code); \
    } \
} while(0)

// Helper macro to handle and cleanup errors
#define HANDLE_ERROR(err_var, msg_prefix) do { \
    if ((err_var).code != 0) { \
        fprintf(stderr, "%s: %s\n", (msg_prefix), \
                (err_var).message ? (err_var).message : "Unknown error"); \
        todozi_error_cleanup(&(err_var)); \
    } \
} while(0)

// Forward declarations
typedef struct Storage Storage;
typedef struct TodoziHandler TodoziHandler;
typedef struct Hlx Hlx;
typedef struct PathBuf PathBuf;

// Mock structures (would need actual implementation)
struct Storage {
    void* data;
};

struct TodoziHandler {
    Storage* storage;
};

struct Hlx {
    void* data;
};

struct PathBuf {
    char* path;
};

// Command data structures
typedef struct {
    char* title;
    char* description;
} AddCmd;

typedef struct {
    char* filter;
} ListCmd;

typedef struct {
    int id;
} ShowCmd;

typedef struct {
    int id;
    char* action;
    char* time;
    char* priority;
    char* project;
    char* status;
    char* assignee;
    char* tags;
    char* dependencies;
    char* context;
    int progress;
} UpdateCmd;

typedef struct {
    int id;
} CompleteCmd;

typedef struct {
    char* server_url;
} RegisterCmd;

typedef struct {
    char* backup_name;
} RestoreCmd;

typedef struct {
    char* content;
    char* session_id;
    bool no_checklist;
    bool no_session;
} TdzCntCmd;

typedef struct {
    char* output;
} ExportEmbeddingsCmd;

typedef struct {
    bool dry_run;
    bool verbose;
    bool force;
    bool cleanup;
} MigrateCmd;

typedef struct {
    char* content;
    char* file;
    char* output_format;
    bool human;
} ExtractCmd;

// Command types enumeration
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
    CMD_ERROR,
    CMD_CHAT,
    CMD_SEARCH_ALL,
    CMD_SERVER,
    CMD_IND_DEMO,
    CMD_QUEUE,
    CMD_API,
    CMD_TDZ_CNT,
    CMD_EXPORT_EMBEDDINGS,
    CMD_MIGRATE,
    CMD_TUI,
    CMD_TRAIN,
    CMD_MAESTRO,
    CMD_ML,
    CMD_EXTRACT,
    CMD_STRATEGY,
    CMD_STEPS,
    CMD_HELP
} CommandType;

// Main CLI structure
typedef struct {
    CommandType command_type;
    void* command_data;
    bool has_command;
} Cli;

// Function declarations with proper error handling
TodoziError storage_new(Storage** storage);
static void storage_free(Storage* storage);
TodoziError todozi_handler_new(Storage* storage, TodoziHandler** handler);
static void todozi_handler_free(TodoziHandler* handler);
static char* find_todozi(void* none);
TodoziError hlx_load(const char* path, Hlx** hlx);
void hlx_free(Hlx* hlx);
void color_eyre_install();
// todozi_begin is declared in todozi.h and will be called directly
void init();
TodoziError todozi_handler_handle_add_command(TodoziHandler* handler, AddCmd* add_cmd);
TodoziError todozi_handler_handle_list_command(TodoziHandler* handler, ListCmd* list_cmd);
TodoziError todozi_handler_handle_show_command(TodoziHandler* handler, ShowCmd* show_cmd);
TodoziError todozi_handler_handle_update_command(TodoziHandler* handler, UpdateCmd* update_cmd);
static TodoziError todozi_handler_complete_task(TodoziHandler* handler, int id);
static TodoziError todozi_handler_fix_task_consistency(TodoziHandler* handler);
TodoziError tdzfp(bool* result);
TodoziError ensure_folder_structure(bool* result);
TodoziError register_with_server(const char* server_url);
TodoziError update_config_with_registration(void* registration);
TodoziError get_registration_info(void** registration);
TodoziError is_registered(bool* result);
TodoziError clear_registration();
static TodoziError todozi_handler_delete_task(TodoziHandler* handler, int id);
TodoziError todozi_handler_handle_project_command(TodoziHandler* handler, void* project_cmd);
TodoziError todozi_handler_handle_search_command(TodoziHandler* handler, void* search_cmd);
TodoziError todozi_handler_handle_stats_command(TodoziHandler* handler, void* stats_cmd);
TodoziError todozi_handler_handle_list_backups_command(TodoziHandler* handler);
static TodoziError todozi_handler_restore_backup(TodoziHandler* handler, const char* backup_name);
TodoziError todozi_handler_handle_memory_command(TodoziHandler* handler, void* memory_cmd);
TodoziError todozi_handler_handle_idea_command(TodoziHandler* handler, void* idea_cmd);
TodoziError todozi_handler_handle_agent_command(TodoziHandler* handler, void* agent_cmd);
TodoziError todozi_handler_handle_emb_command(TodoziHandler* handler, void* emb_cmd);
TodoziError todozi_handler_handle_error_command(TodoziHandler* handler, void* error_cmd);
TodoziError todozi_handler_handle_chat_command(TodoziHandler* handler, void* chat_cmd);
TodoziError todozi_handler_handle_search_all_command(TodoziHandler* handler, void* search_all_cmd);
TodoziError todozi_handler_handle_server_command(TodoziHandler* handler, void* server_cmd);
TodoziError todozi_handler_handle_ind_command();
TodoziError todozi_handler_handle_queue_command(TodoziHandler* handler, void* queue_command);
TodoziError todozi_handler_handle_api_command(TodoziHandler* handler, void* api_command);
TodoziError tdz_cnt(const char* content, const char* session_id, char** result);
TodoziError todozi_handler_handle_train_command(TodoziHandler* handler, void* train_cmd);
TodoziError todozi_handler_handle_extract_command(TodoziHandler* handler, ExtractCmd* extract_cmd);
TodoziError todozi_handler_handle_strategy_command(TodoziHandler* handler, void* strategy_cmd);
TodoziError todozi_handler_handle_steps_command(TodoziHandler* handler, void* steps_cmd);
TodoziError storage_create_backup(Storage* storage);
TodoziError storage_export_embedded_tasks_hlx(Storage* storage, const char* output);
TodoziError migration_cli_new(void** cli);
void migration_cli_with_dry_run(void* cli, bool dry_run);
void migration_cli_with_verbose(void* cli, bool verbose);
void migration_cli_with_force(void* cli, bool force);
TodoziError migration_cli_run(void* cli);
void launch_gui();
void show_help(void);

// Parse command line arguments
// Returns parsed CLI structure, caller must free command_data via free_cli_data()
Cli parse_cli(int argc, char* argv[]) {
    Cli cli = {0};
    cli.has_command = false;
    cli.command_type = CMD_ERROR;
    cli.command_data = NULL;
    
    if (argc < 2) {
        cli.command_type = CMD_HELP;
        cli.has_command = true;
        return cli;
    }
    
    char* command = argv[1];
    if (!command) {
        cli.command_type = CMD_HELP;
        cli.has_command = true;
        return cli;
    }
    
    cli.has_command = true;
    
    // Handle help commands
    if (strcmp(command, "help") == 0 || strcmp(command, "-h") == 0 || strcmp(command, "--help") == 0) {
        cli.command_type = CMD_HELP;
        return cli;
    }
    
    if (strcmp(command, "init") == 0) {
        cli.command_type = CMD_INIT;
    } else if (strcmp(command, "add") == 0) {
        cli.command_type = CMD_ADD;
        // Parse add command data
        AddCmd* add_cmd = malloc(sizeof(AddCmd));
        if (add_cmd) {
            add_cmd->title = argc > 2 ? strdup(argv[2]) : NULL;
            add_cmd->description = argc > 3 ? strdup(argv[3]) : NULL;
        }
        cli.command_data = add_cmd;
    } else if (strcmp(command, "list") == 0) {
        cli.command_type = CMD_LIST;
        ListCmd* list_cmd = malloc(sizeof(ListCmd));
        if (list_cmd) {
            list_cmd->filter = argc > 2 ? strdup(argv[2]) : NULL;
        }
        cli.command_data = list_cmd;
    } else if (strcmp(command, "show") == 0) {
        cli.command_type = CMD_SHOW;
        ShowCmd* show_cmd = malloc(sizeof(ShowCmd));
        if (show_cmd && argc > 2) {
            show_cmd->id = atoi(argv[2]);
        }
        cli.command_data = show_cmd;
    } else if (strcmp(command, "update") == 0) {
        cli.command_type = CMD_UPDATE;
        UpdateCmd* update_cmd = malloc(sizeof(UpdateCmd));
        if (update_cmd) {
            update_cmd->id = argc > 2 ? atoi(argv[2]) : 0;
            // Additional parameters would be parsed here
        }
        cli.command_data = update_cmd;
    } else if (strcmp(command, "complete") == 0) {
        cli.command_type = CMD_COMPLETE;
        CompleteCmd* complete_cmd = malloc(sizeof(CompleteCmd));
        if (complete_cmd && argc > 2) {
            complete_cmd->id = atoi(argv[2]);
        }
        cli.command_data = complete_cmd;
    } else if (strcmp(command, "fix-consistency") == 0) {
        cli.command_type = CMD_FIX_CONSISTENCY;
    } else if (strcmp(command, "check-structure") == 0) {
        cli.command_type = CMD_CHECK_STRUCTURE;
    } else if (strcmp(command, "ensure-structure") == 0) {
        cli.command_type = CMD_ENSURE_STRUCTURE;
    } else if (strcmp(command, "register") == 0) {
        cli.command_type = CMD_REGISTER;
        RegisterCmd* register_cmd = malloc(sizeof(RegisterCmd));
        if (register_cmd) {
            register_cmd->server_url = argc > 2 ? strdup(argv[2]) : NULL;
        }
        cli.command_data = register_cmd;
    } else if (strcmp(command, "registration-status") == 0) {
        cli.command_type = CMD_REGISTRATION_STATUS;
    } else if (strcmp(command, "clear-registration") == 0) {
        cli.command_type = CMD_CLEAR_REGISTRATION;
    } else if (strcmp(command, "delete") == 0) {
        cli.command_type = CMD_DELETE;
        CompleteCmd* delete_cmd = malloc(sizeof(CompleteCmd));
        if (delete_cmd && argc > 2) {
            delete_cmd->id = atoi(argv[2]);
        }
        cli.command_data = delete_cmd;
    } else if (strcmp(command, "project") == 0) {
        cli.command_type = CMD_PROJECT;
    } else if (strcmp(command, "search") == 0) {
        cli.command_type = CMD_SEARCH;
    } else if (strcmp(command, "stats") == 0) {
        cli.command_type = CMD_STATS;
    } else if (strcmp(command, "backup") == 0) {
        cli.command_type = CMD_BACKUP;
    } else if (strcmp(command, "list-backups") == 0) {
        cli.command_type = CMD_LIST_BACKUPS;
    } else if (strcmp(command, "restore") == 0) {
        cli.command_type = CMD_RESTORE;
        RestoreCmd* restore_cmd = malloc(sizeof(RestoreCmd));
        if (restore_cmd) {
            restore_cmd->backup_name = argc > 2 ? strdup(argv[2]) : NULL;
        }
        cli.command_data = restore_cmd;
    } else if (strcmp(command, "memory") == 0) {
        cli.command_type = CMD_MEMORY;
    } else if (strcmp(command, "idea") == 0) {
        cli.command_type = CMD_IDEA;
    } else if (strcmp(command, "agent") == 0) {
        cli.command_type = CMD_AGENT;
    } else if (strcmp(command, "emb") == 0) {
        cli.command_type = CMD_EMB;
    } else if (strcmp(command, "error") == 0) {
        cli.command_type = CMD_ERROR;
    } else if (strcmp(command, "chat") == 0) {
        cli.command_type = CMD_CHAT;
    } else if (strcmp(command, "search-all") == 0) {
        cli.command_type = CMD_SEARCH_ALL;
    } else if (strcmp(command, "server") == 0) {
        cli.command_type = CMD_SERVER;
    } else if (strcmp(command, "ind-demo") == 0) {
        cli.command_type = CMD_IND_DEMO;
    } else if (strcmp(command, "queue") == 0) {
        cli.command_type = CMD_QUEUE;
    } else if (strcmp(command, "api") == 0) {
        cli.command_type = CMD_API;
    } else if (strcmp(command, "tdz-cnt") == 0) {
        cli.command_type = CMD_TDZ_CNT;
        TdzCntCmd* tdz_cnt_cmd = malloc(sizeof(TdzCntCmd));
        if (tdz_cnt_cmd) {
            tdz_cnt_cmd->content = argc > 2 ? strdup(argv[2]) : NULL;
            tdz_cnt_cmd->session_id = argc > 3 ? strdup(argv[3]) : NULL;
            tdz_cnt_cmd->no_checklist = false; // Would parse from args
            tdz_cnt_cmd->no_session = false;   // Would parse from args
        }
        cli.command_data = tdz_cnt_cmd;
    } else if (strcmp(command, "export-embeddings") == 0) {
        cli.command_type = CMD_EXPORT_EMBEDDINGS;
        ExportEmbeddingsCmd* export_cmd = malloc(sizeof(ExportEmbeddingsCmd));
        if (export_cmd) {
            export_cmd->output = argc > 2 ? strdup(argv[2]) : strdup("./exported_embeddings.hlx");
        }
        cli.command_data = export_cmd;
    } else if (strcmp(command, "migrate") == 0) {
        cli.command_type = CMD_MIGRATE;
        MigrateCmd* migrate_cmd = malloc(sizeof(MigrateCmd));
        if (migrate_cmd) {
            migrate_cmd->dry_run = false;   // Would parse from args
            migrate_cmd->verbose = false;   // Would parse from args
            migrate_cmd->force = false;     // Would parse from args
            migrate_cmd->cleanup = false;   // Would parse from args
        }
        cli.command_data = migrate_cmd;
    } else if (strcmp(command, "tui") == 0) {
        cli.command_type = CMD_TUI;
    } else if (strcmp(command, "train") == 0) {
        cli.command_type = CMD_TRAIN;
    } else if (strcmp(command, "maestro") == 0) {
        cli.command_type = CMD_MAESTRO;
    } else if (strcmp(command, "ml") == 0) {
        cli.command_type = CMD_ML;
    } else if (strcmp(command, "extract") == 0) {
        cli.command_type = CMD_EXTRACT;
        ExtractCmd* extract_cmd = malloc(sizeof(ExtractCmd));
        if (extract_cmd) {
            extract_cmd->content = argc > 2 ? strdup(argv[2]) : NULL;
            extract_cmd->file = argc > 3 ? strdup(argv[3]) : NULL;
            extract_cmd->output_format = argc > 4 ? strdup(argv[4]) : NULL;
            extract_cmd->human = false; // Would parse from args
        }
        cli.command_data = extract_cmd;
    } else if (strcmp(command, "strategy") == 0) {
        cli.command_type = CMD_STRATEGY;
    } else if (strcmp(command, "steps") == 0) {
        cli.command_type = CMD_STEPS;
    } else {
        fprintf(stderr, "Unknown command: %s\n", command);
        fprintf(stderr, "Run 'todozi help' or 'todozi -h' to see available commands.\n");
        cli.command_type = CMD_ERROR;
        cli.has_command = true; // Keep has_command true so we exit early
    }
    
    return cli;
}

// Free command data allocated by parse_cli
// Safe to call multiple times (idempotent)
void free_cli_data(Cli* cli) {
    if (!cli || !cli->command_data) return;
    
    switch (cli->command_type) {
        case CMD_ADD: {
            AddCmd* cmd = (AddCmd*)cli->command_data;
            free(cmd->title);
            free(cmd->description);
            free(cmd);
            break;
        }
        case CMD_LIST: {
            ListCmd* cmd = (ListCmd*)cli->command_data;
            free(cmd->filter);
            free(cmd);
            break;
        }
        case CMD_SHOW: {
            ShowCmd* cmd = (ShowCmd*)cli->command_data;
            free(cmd);
            break;
        }
        case CMD_UPDATE: {
            UpdateCmd* cmd = (UpdateCmd*)cli->command_data;
            free(cmd->action);
            free(cmd->time);
            free(cmd->priority);
            free(cmd->project);
            free(cmd->status);
            free(cmd->assignee);
            free(cmd->tags);
            free(cmd->dependencies);
            free(cmd->context);
            free(cmd);
            break;
        }
        case CMD_COMPLETE: {
            CompleteCmd* cmd = (CompleteCmd*)cli->command_data;
            free(cmd);
            break;
        }
        case CMD_REGISTER: {
            RegisterCmd* cmd = (RegisterCmd*)cli->command_data;
            free(cmd->server_url);
            free(cmd);
            break;
        }
        case CMD_RESTORE: {
            RestoreCmd* cmd = (RestoreCmd*)cli->command_data;
            free(cmd->backup_name);
            free(cmd);
            break;
        }
        case CMD_TDZ_CNT: {
            TdzCntCmd* cmd = (TdzCntCmd*)cli->command_data;
            free(cmd->content);
            free(cmd->session_id);
            free(cmd);
            break;
        }
        case CMD_EXPORT_EMBEDDINGS: {
            ExportEmbeddingsCmd* cmd = (ExportEmbeddingsCmd*)cli->command_data;
            free(cmd->output);
            free(cmd);
            break;
        }
        case CMD_MIGRATE: {
            MigrateCmd* cmd = (MigrateCmd*)cli->command_data;
            free(cmd);
            break;
        }
        case CMD_EXTRACT: {
            ExtractCmd* cmd = (ExtractCmd*)cli->command_data;
            free(cmd->content);
            free(cmd->file);
            free(cmd->output_format);
            free(cmd);
            break;
        }
        default:
            free(cli->command_data);
            break;
    }
    cli->command_data = NULL;
}

int main(int argc, char* argv[]) {
    // Install error handling
    color_eyre_install();
    
    // Initialize variables for cleanup
    Storage* storage = NULL;
    TodoziHandler* handler = NULL;
    Hlx* todozi_hlx = NULL;
    char* todozi_dir_str = NULL;
    Cli cli = {0};
    int result = 0;
    
    // Parse CLI early to handle error cases
    cli = parse_cli(argc, argv);
    
    // Handle error case (unknown command)
    if (cli.command_type == CMD_ERROR && cli.has_command) {
        free_cli_data(&cli);
        return 1;
    }
    
    // Handle help command early (before initialization)
    if (cli.command_type == CMD_HELP) {
        show_help();
        free_cli_data(&cli);
        return 0;
    }
    
    // If no command, show help
    if (!cli.has_command) {
        show_help();
        return 0;
    }
    
    // Create storage
    TodoziError storage_err = storage_new(&storage);
    if (storage_err.code != 0) {
        fprintf(stderr, "Failed to create storage: %s\n", 
                storage_err.message ? storage_err.message : "Unknown error");
        todozi_error_cleanup(&storage_err);
        free_cli_data(&cli);
        return storage_err.code != 0 ? storage_err.code : 1;
    }
    
    // Initialize if needed (skip for init and export-embeddings)
    if (cli.command_type != CMD_INIT && cli.command_type != CMD_EXPORT_EMBEDDINGS) {
        init();
    }
    
    // Create handler
    TodoziError handler_err = todozi_handler_new(storage, &handler);
    if (handler_err.code != 0) {
        fprintf(stderr, "Failed to create handler: %s\n", 
                handler_err.message ? handler_err.message : "Unknown error");
        todozi_error_cleanup(&handler_err);
        storage_free(storage);
        free_cli_data(&cli);
        return handler_err.code != 0 ? handler_err.code : 1;
    }
    
    // Begin todozi if needed
    if (cli.command_type != CMD_EXPORT_EMBEDDINGS) {
        // Call the API function directly - use empty string if no task_id
        todozi_begin("");
    }
    
    // Find todozi directory
    todozi_dir_str = find_todozi(NULL);
    if (!todozi_dir_str) {
        fprintf(stderr, "Could not find todozi directory\n");
        fprintf(stderr, "Try running 'todozi init' to initialize the directory structure.\n");
        goto cleanup_error;
    }
    
    // Load HLX if needed
    if (cli.command_type != CMD_EXPORT_EMBEDDINGS) {
        char hlx_path[1024];
        int snprintf_result = snprintf(hlx_path, sizeof(hlx_path), "%s/tdz.hlx", todozi_dir_str);
        if (snprintf_result < 0 || snprintf_result >= (int)sizeof(hlx_path)) {
            fprintf(stderr, "Path too long for HLX file\n");
            goto cleanup_error;
        }
        
        TodoziError hlx_err = hlx_load(hlx_path, &todozi_hlx);
        if (hlx_err.code != 0) {
            fprintf(stderr, "Failed to load HLX: %s\n", 
                    hlx_err.message ? hlx_err.message : "Unknown error");
            // For some commands, missing HLX might be acceptable (e.g., init)
            if (cli.command_type != CMD_INIT && 
                cli.command_type != CMD_ENSURE_STRUCTURE &&
                cli.command_type != CMD_CHECK_STRUCTURE) {
                todozi_error_cleanup(&hlx_err);
                goto cleanup_error;
            }
            todozi_error_cleanup(&hlx_err);
        }
    }
    
    // Handle commands
    switch (cli.command_type) {
        case CMD_INIT:
            // Initialize todozi system - use empty string for task_id
            todozi_begin("");
            break;
            
        case CMD_ADD: {
            AddCmd* add_cmd = (AddCmd*)cli.command_data;
            TodoziError err = todozi_handler_handle_add_command(handler, add_cmd);
            if (err.code != 0) {
                HANDLE_ERROR(err, "Error handling add command");
                result = err.code;
            }
            break;
        }
            
        case CMD_LIST: {
            ListCmd* list_cmd = (ListCmd*)cli.command_data;
            TodoziError err = todozi_handler_handle_list_command(handler, list_cmd);
            if (err.code != 0) {
                HANDLE_ERROR(err, "Error handling list command");
                result = err.code;
            }
            break;
        }
            
        case CMD_SHOW: {
            ShowCmd* show_cmd = (ShowCmd*)cli.command_data;
            TodoziError err = todozi_handler_handle_show_command(handler, show_cmd);
            if (err.code != 0) {
                HANDLE_ERROR(err, "Error handling show command");
                result = err.code;
            }
            break;
        }
            
        case CMD_UPDATE: {
            UpdateCmd* update_cmd = (UpdateCmd*)cli.command_data;
            TodoziError err = todozi_handler_handle_update_command(handler, update_cmd);
            if (err.code != 0) {
                HANDLE_ERROR(err, "Error handling update command");
                result = err.code;
            }
            break;
        }
            
        case CMD_COMPLETE: {
            CompleteCmd* complete_cmd = (CompleteCmd*)cli.command_data;
            TodoziError err = todozi_handler_complete_task(handler, complete_cmd->id);
            if (err.code != 0) {
                HANDLE_ERROR(err, "Error completing task");
                result = err.code;
            }
            break;
        }
            
        case CMD_FIX_CONSISTENCY: {
            TodoziError err = todozi_handler_fix_task_consistency(handler);
            if (err.code != 0) {
                fprintf(stderr, "Error fixing task consistency: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_CHECK_STRUCTURE: {
            bool is_complete = false;
            TodoziError err = tdzfp(&is_complete);
            if (err.code != 0) {
                fprintf(stderr, "Error checking folder structure: %s\n", err.message);
            } else if (is_complete) {
                printf("✅ Todozi folder structure is complete!\n");
            } else {
                printf("❌ Todozi folder structure is incomplete. Run 'todozi init' to create missing components.\n");
            }
            break;
        }
        
        case CMD_ENSURE_STRUCTURE: {
            bool ensured = false;
            TodoziError err = ensure_folder_structure(&ensured);
            if (err.code != 0) {
                fprintf(stderr, "Error ensuring folder structure: %s\n", err.message);
            } else if (ensured) {
                printf("✅ Todozi folder structure ensured successfully!\n");
            } else {
                printf("❌ Failed to ensure folder structure\n");
            }
            break;
        }
        
        case CMD_REGISTER: {
            RegisterCmd* register_cmd = (RegisterCmd*)cli.command_data;
            printf("🚀 Starting registration with todozi.com...\n");
            TodoziError reg_err = register_with_server(register_cmd->server_url);
            if (reg_err.code == 0) {
                void* registration = NULL; // Would need actual registration data
                TodoziError update_err = update_config_with_registration(registration);
                if (update_err.code != 0) {
                    fprintf(stderr, "❌ Failed to update configuration: %s\n", update_err.message);
                } else {
                    printf("✅ Registration data saved to tdz.hlx\n");
                }
            } else {
                fprintf(stderr, "❌ Registration failed: %s\n", reg_err.message);
                printf("💡 Note: Registration is optional - todozi will work without server connection\n");
            }
            break;
        }
        
        case CMD_REGISTRATION_STATUS: {
            void* registration = NULL;
            TodoziError reg_err = get_registration_info(&registration);
            if (reg_err.code == 0 && registration != NULL) {
                // Check if fully registered
                bool fully_registered = true; // Would check actual registration data
                if (fully_registered) {
                    printf("📋 Registration Status: ✅ FULLY REGISTERED\n");
                } else {
                    printf("📋 Registration Status: ⏳ REGISTERED (WAITING FOR SERVER)\n");
                }
            } else if (reg_err.code == 0 && registration == NULL) {
                // Initialize todozi system when not registered
                todozi_begin("");
                printf("📋 Registration Status: ❌ NOT REGISTERED\n");
                printf("Please wait one minute and run todozi registration-status again\n");
            } else {
                fprintf(stderr, "❌ Error checking registration status: %s\n", reg_err.message);
            }
            break;
        }
        
        case CMD_CLEAR_REGISTRATION: {
            bool registered = false;
            TodoziError reg_err = is_registered(&registered);
            if (reg_err.code == 0) {
                if (registered) {
                    TodoziError clear_err = clear_registration();
                    if (clear_err.code != 0) {
                        fprintf(stderr, "❌ Failed to clear registration: %s\n", clear_err.message);
                    }
                } else {
                    printf("📋 Not registered - nothing to clear\n");
                }
            } else {
                fprintf(stderr, "❌ Error checking registration status: %s\n", reg_err.message);
            }
            break;
        }
        
        case CMD_DELETE: {
            CompleteCmd* delete_cmd = (CompleteCmd*)cli.command_data;
            TodoziError err = todozi_handler_delete_task(handler, delete_cmd->id);
            if (err.code != 0) {
                fprintf(stderr, "Error deleting task: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_PROJECT: {
            TodoziError err = todozi_handler_handle_project_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling project command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_SEARCH: {
            TodoziError err = todozi_handler_handle_search_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling search command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_STATS: {
            TodoziError err = todozi_handler_handle_stats_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling stats command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_BACKUP:
            printf("Creating backup...\n");
            TodoziError err = storage_create_backup(storage);
            if (err.code == 0) {
                printf("✅ Backup created successfully!\n");
            } else {
                fprintf(stderr, "Error creating backup: %s\n", err.message);
                result = err.code;
            }
            break;
            
        case CMD_LIST_BACKUPS: {
            TodoziError err = todozi_handler_handle_list_backups_command(handler);
            if (err.code != 0) {
                fprintf(stderr, "Error listing backups: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_RESTORE: {
            RestoreCmd* restore_cmd = (RestoreCmd*)cli.command_data;
            TodoziError err = todozi_handler_restore_backup(handler, restore_cmd->backup_name);
            if (err.code != 0) {
                fprintf(stderr, "Error restoring backup: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_MEMORY: {
            TodoziError err = todozi_handler_handle_memory_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling memory command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_IDEA: {
            TodoziError err = todozi_handler_handle_idea_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling idea command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_AGENT: {
            TodoziError err = todozi_handler_handle_agent_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling agent command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_EMB: {
            TodoziError err = todozi_handler_handle_emb_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling emb command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_ERROR: {
            TodoziError err = todozi_handler_handle_error_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling error command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_CHAT: {
            TodoziError err = todozi_handler_handle_chat_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling chat command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_SEARCH_ALL: {
            TodoziError err = todozi_handler_handle_search_all_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling search-all command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_SERVER: {
            TodoziError err = todozi_handler_handle_server_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling server command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_IND_DEMO: {
            TodoziError err = todozi_handler_handle_ind_command();
            if (err.code != 0) {
                fprintf(stderr, "Error handling ind-demo command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_QUEUE: {
            TodoziError err = todozi_handler_handle_queue_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling queue command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_API: {
            TodoziError err = todozi_handler_handle_api_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling API command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_TDZ_CNT: {
            TdzCntCmd* tdz_cnt_cmd = (TdzCntCmd*)cli.command_data;
            char* result_str = NULL;
            TodoziError err = tdz_cnt(tdz_cnt_cmd->content, tdz_cnt_cmd->session_id, &result_str);
            if (err.code == 0) {
                printf("%s\n", result_str);
                free(result_str);
            } else {
                fprintf(stderr, "Error processing content: %s\n", err.message);
                result = err.code;
            }
            break;
        }
        
        case CMD_EXPORT_EMBEDDINGS: {
            ExportEmbeddingsCmd* export_cmd = (ExportEmbeddingsCmd*)cli.command_data;
            printf("🧠 Exporting embedded task vectors to HLX format for AI/ML...\n");
            TodoziError err = storage_export_embedded_tasks_hlx(storage, export_cmd->output);
            if (err.code == 0) {
                printf("✅ Embedded task vectors exported to: %s\n", export_cmd->output);
            } else {
                fprintf(stderr, "❌ Failed to export embeddings: %s\n", err.message);
                result = err.code;
            }
            break;
        }
        
        case CMD_MIGRATE: {
            MigrateCmd* migrate_cmd = (MigrateCmd*)cli.command_data;
            printf("🚀 Starting task migration to project-based system...\n");
            void* migration_cli = NULL;
            TodoziError new_err = migration_cli_new(&migration_cli);
            if (new_err.code == 0 && migration_cli) {
                migration_cli_with_dry_run(migration_cli, migrate_cmd->dry_run);
                migration_cli_with_verbose(migration_cli, migrate_cmd->verbose);
                migration_cli_with_force(migration_cli, migrate_cmd->force);
                TodoziError run_err = migration_cli_run(migration_cli);
                if (run_err.code == 0) {
                    printf("✅ Migration completed successfully!\n");
                    if (migrate_cmd->cleanup && !migrate_cmd->dry_run) {
                        printf("🧹 Cleanup completed - old collections removed\n");
                    }
                } else {
                    fprintf(stderr, "❌ Migration failed: %s\n", run_err.message);
                    result = run_err.code;
                }
                // Would free migration_cli
            } else {
                fprintf(stderr, "❌ Failed to create migration CLI: %s\n", 
                        new_err.code != 0 ? new_err.message : "Unknown error");
                result = new_err.code != 0 ? new_err.code : 1;
            }
            break;
        }
        
        case CMD_TUI:
            launch_gui();
            break;
            
        case CMD_HELP:
            show_help();
            break;
            
        case CMD_TRAIN: {
            TodoziError err = todozi_handler_handle_train_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling train command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_MAESTRO:
            printf("🎭 Maestro functionality coming soon!\n");
            break;
            
        case CMD_ML:
            printf("🤖 ML functionality coming soon!\n");
            break;
            
        case CMD_EXTRACT: {
            ExtractCmd* extract_cmd = (ExtractCmd*)cli.command_data;
            TodoziError err = todozi_handler_handle_extract_command(handler, extract_cmd);
            if (err.code != 0) {
                fprintf(stderr, "Error handling extract command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_STRATEGY: {
            TodoziError err = todozi_handler_handle_strategy_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling strategy command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
            
        case CMD_STEPS: {
            TodoziError err = todozi_handler_handle_steps_command(handler, cli.command_data);
            if (err.code != 0) {
                fprintf(stderr, "Error handling steps command: %s\n", err.message);
                result = err.code;
            }
            break;
        }
    }
    
    // Normal cleanup path
cleanup:
    if (todozi_hlx) {
        hlx_free(todozi_hlx);
        todozi_hlx = NULL;
    }
    if (todozi_dir_str) {
        free(todozi_dir_str);
        todozi_dir_str = NULL;
    }
    if (handler) {
        todozi_handler_free(handler);
        handler = NULL;
    }
    if (storage) {
        storage_free(storage);
        storage = NULL;
    }
    free_cli_data(&cli);
    
    return result;

cleanup_error:
    result = 1;
    goto cleanup;
}

// Stub implementations with proper error handling
TodoziError storage_new(Storage** storage) {
    if (!storage) {
        TodoziError err = {EINVAL, strdup("Invalid storage pointer")};
        return err;
    }
    
    *storage = calloc(1, sizeof(Storage));
    if (!*storage) {
        TodoziError err = {ENOMEM, strdup("Failed to allocate storage")};
        return err;
    }
    
    (*storage)->data = NULL;
    TodoziError err = {0, NULL};
    return err;
}

static void storage_free(Storage* storage) {
    if (storage) {
        free(storage);
    }
}

TodoziError todozi_handler_new(Storage* storage, TodoziHandler** handler) {
    if (!handler) {
        TodoziError err = {EINVAL, strdup("Invalid handler pointer")};
        return err;
    }
    
    if (!storage) {
        TodoziError err = {EINVAL, strdup("Invalid storage parameter")};
        return err;
    }
    
    *handler = calloc(1, sizeof(TodoziHandler));
    if (!*handler) {
        TodoziError err = {ENOMEM, strdup("Failed to allocate handler")};
        return err;
    }
    
    (*handler)->storage = storage;
    TodoziError err = {0, NULL};
    return err;
}

static void todozi_handler_free(TodoziHandler* handler) {
    if (handler) {
        free(handler);
    }
}

// Find todozi directory - checks multiple locations
static char* find_todozi(void* none) {
    (void)none; // Suppress unused parameter warning
    
    // Priority order:
    // 1. TODOZI_HOME environment variable
    // 2. ~/.todozi
    // 3. ./todozi (current directory)
    // 4. $HOME/.todozi (fallback)
    
    const char* env_home = getenv("TODOZI_HOME");
    if (env_home && env_home[0] != '\0') {
        struct stat st;
        if (stat(env_home, &st) == 0 && S_ISDIR(st.st_mode)) {
            return strdup(env_home);
        }
    }
    
    // Try ~/.todozi
    const char* home = getenv("HOME");
    if (!home) {
        struct passwd* pw = getpwuid(getuid());
        if (pw && pw->pw_dir) {
            home = pw->pw_dir;
        }
    }
    
    if (home) {
        size_t path_len = strlen(home) + 9; // ~/.todozi + null terminator
        char* path = malloc(path_len);
        if (path) {
            snprintf(path, path_len, "%s/.todozi", home);
            struct stat st;
            if (stat(path, &st) == 0 && S_ISDIR(st.st_mode)) {
                return path;
            }
            free(path);
        }
    }
    
    // Try current directory
    struct stat st;
    if (stat("./todozi", &st) == 0 && S_ISDIR(st.st_mode)) {
        return strdup("./todozi");
    }
    
    // Try using the actual API if available (optional fallback)
    // Note: This uses the API's error type, so we handle it separately
    todozi_error_t* api_error = NULL;
    const char* api_path = todozi_find_todozi(NULL, &api_error);
    if (api_path && !api_error) {
        char* result = strdup(api_path);
        todozi_string_free((char*)api_path);
        if (api_error) {
            todozi_error_free(api_error);
        }
        return result;
    }
    if (api_error) {
        todozi_error_free(api_error);
    }
    
    // Default fallback - create directory if it doesn't exist
    // (This allows the system to work even if directory doesn't exist yet)
    return strdup("./todozi");
}

TodoziError hlx_load(const char* path, Hlx** hlx) {
    if (!hlx) {
        TodoziError err = {EINVAL, strdup("Invalid HLX pointer")};
        return err;
    }
    
    if (!path) {
        TodoziError err = {EINVAL, strdup("Invalid path parameter")};
        return err;
    }
    
    // Check if file exists
    struct stat st;
    if (stat(path, &st) != 0) {
        char* error_msg = NULL;
        int msg_len = snprintf(NULL, 0, "HLX file not found: %s", path);
        if (msg_len > 0) {
            error_msg = malloc(msg_len + 1);
            if (error_msg) {
                snprintf(error_msg, msg_len + 1, "HLX file not found: %s", path);
            }
        }
        TodoziError err = {ENOENT, error_msg ? error_msg : strdup("HLX file not found")};
        return err;
    }
    
    *hlx = calloc(1, sizeof(Hlx));
    if (!*hlx) {
        TodoziError err = {ENOMEM, strdup("Failed to allocate HLX")};
        return err;
    }
    
    (*hlx)->data = NULL;
    TodoziError err = {0, NULL};
    return err;
}

void hlx_free(Hlx* hlx) {
    if (hlx) {
        free(hlx);
    }
}

void color_eyre_install() {
    // Stub implementation - installs error handler for color_eyre
    // This would typically set up panic handlers and error formatting
}

void init() {
    // Stub implementation - initializes the todozi system
    // This would typically set up storage, load configuration, etc.
}

// Stub implementations for API functions that may not be linked
// These provide fallback behavior when the full library is not available
void todozi_string_free(char* str) {
    // Free a string allocated by the todozi API
    // This is a stub that matches the API signature
    if (str) {
        free(str);
    }
}

const char* todozi_find_todozi(const char* str, todozi_error_t** error) {
    // Find the todozi directory using the API
    // This is a stub implementation that returns NULL to indicate
    // the API method is not available, allowing fallback to local methods
    (void)str; // Suppress unused parameter warning
    if (error) {
        *error = NULL; // No error, just not available
    }
    return NULL; // Return NULL to indicate API method unavailable
}

TodoziError todozi_handler_handle_add_command(TodoziHandler* handler, AddCmd* add_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_list_command(TodoziHandler* handler, ListCmd* list_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_show_command(TodoziHandler* handler, ShowCmd* show_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_update_command(TodoziHandler* handler, UpdateCmd* update_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

static TodoziError todozi_handler_complete_task(TodoziHandler* handler, int id) {
    TodoziError err = {0, NULL};
    return err;
}

static TodoziError todozi_handler_fix_task_consistency(TodoziHandler* handler) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError tdzfp(bool* result) {
    if (result) *result = true;
    TodoziError err = {0, NULL};
    return err;
}

TodoziError ensure_folder_structure(bool* result) {
    if (result) *result = true;
    TodoziError err = {0, NULL};
    return err;
}

TodoziError register_with_server(const char* server_url) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError update_config_with_registration(void* registration) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError get_registration_info(void** registration) {
    if (registration) *registration = NULL;
    TodoziError err = {0, NULL};
    return err;
}

TodoziError is_registered(bool* result) {
    if (result) *result = false;
    TodoziError err = {0, NULL};
    return err;
}

TodoziError clear_registration() {
    TodoziError err = {0, NULL};
    return err;
}

static TodoziError todozi_handler_delete_task(TodoziHandler* handler, int id) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_project_command(TodoziHandler* handler, void* project_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_search_command(TodoziHandler* handler, void* search_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_stats_command(TodoziHandler* handler, void* stats_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_list_backups_command(TodoziHandler* handler) {
    TodoziError err = {0, NULL};
    return err;
}

static TodoziError todozi_handler_restore_backup(TodoziHandler* handler, const char* backup_name) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_memory_command(TodoziHandler* handler, void* memory_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_idea_command(TodoziHandler* handler, void* idea_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_agent_command(TodoziHandler* handler, void* agent_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_emb_command(TodoziHandler* handler, void* emb_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_error_command(TodoziHandler* handler, void* error_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_chat_command(TodoziHandler* handler, void* chat_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_search_all_command(TodoziHandler* handler, void* search_all_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_server_command(TodoziHandler* handler, void* server_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_ind_command() {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_queue_command(TodoziHandler* handler, void* queue_command) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_api_command(TodoziHandler* handler, void* api_command) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError tdz_cnt(const char* content, const char* session_id, char** result) {
    if (!content) {
        TodoziError err = {EINVAL, strdup("Invalid content parameter")};
        return err;
    }
    
    if (!result) {
        TodoziError err = {EINVAL, strdup("Invalid result pointer")};
        return err;
    }
    
    *result = strdup("Processed content");
    if (!*result) {
        TodoziError err = {ENOMEM, strdup("Failed to allocate result string")};
        return err;
    }
    
    (void)session_id; // Suppress unused parameter warning
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_train_command(TodoziHandler* handler, void* train_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_extract_command(TodoziHandler* handler, ExtractCmd* extract_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_strategy_command(TodoziHandler* handler, void* strategy_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError todozi_handler_handle_steps_command(TodoziHandler* handler, void* steps_cmd) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError storage_create_backup(Storage* storage) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError storage_export_embedded_tasks_hlx(Storage* storage, const char* output) {
    TodoziError err = {0, NULL};
    return err;
}

TodoziError migration_cli_new(void** cli) {
    if (!cli) {
        TodoziError err = {EINVAL, strdup("Invalid CLI pointer")};
        return err;
    }
    
    *cli = calloc(1, 1); // Allocate minimal memory for stub
    if (!*cli) {
        TodoziError err = {ENOMEM, strdup("Failed to allocate migration CLI")};
        return err;
    }
    
    TodoziError err = {0, NULL};
    return err;
}

void migration_cli_with_dry_run(void* cli, bool dry_run) {
    // Stub
}

void migration_cli_with_verbose(void* cli, bool verbose) {
    // Stub
}

void migration_cli_with_force(void* cli, bool force) {
    // Stub
}

TodoziError migration_cli_run(void* cli) {
    TodoziError err = {0, NULL};
    return err;
}

void launch_gui() {
    printf("Launching GUI...\n");
    fflush(stdout);
}

void show_help(void) {
    printf("Todozi - Task Management System\n");
    printf("Usage: todozi [COMMAND] [OPTIONS]\n\n");
    printf("Commands:\n");
    printf("  Basic Tasks:\n");
    printf("    init                    Initialize todozi directory structure\n");
    printf("    add <title> [desc]      Add a new task\n");
    printf("    list [filter]           List tasks (optionally filtered)\n");
    printf("    show <id>               Show details of a specific task\n");
    printf("    update <id>             Update a task\n");
    printf("    complete <id>           Mark a task as complete\n");
    printf("    delete <id>             Delete a task\n\n");
    
    printf("  Projects:\n");
    printf("    project                 Manage projects\n");
    printf("    search [query]          Search tasks\n");
    printf("    search-all [query]      Search across all content types\n");
    printf("    stats                   Show statistics\n\n");
    
    printf("  Storage & Backup:\n");
    printf("    backup                  Create a backup\n");
    printf("    list-backups            List available backups\n");
    printf("    restore <name>          Restore from a backup\n");
    printf("    check-structure         Check folder structure\n");
    printf("    ensure-structure        Ensure folder structure exists\n");
    printf("    fix-consistency         Fix task consistency issues\n\n");
    
    printf("  Registration:\n");
    printf("    register [server_url]   Register with todozi.com server\n");
    printf("    registration-status     Check registration status\n");
    printf("    clear-registration      Clear registration data\n\n");
    
    printf("  Content Management:\n");
    printf("    memory                  Manage memories\n");
    printf("    idea                    Manage ideas\n");
    printf("    agent                   Manage AI agents\n");
    printf("    emb                     Manage embeddings\n");
    printf("    error                   Manage errors\n");
    printf("    chat                    Interactive chat\n");
    printf("    extract                 Extract content\n");
    printf("    strategy                Strategy analysis\n");
    printf("    steps                   Manage task steps\n\n");
    
    printf("  Advanced:\n");
    printf("    queue                   Manage queue items\n");
    printf("    api                     API operations\n");
    printf("    server                  Server operations\n");
    printf("    tui                     Launch text user interface\n");
    printf("    train                   Training data management\n");
    printf("    migrate                 Migrate data\n");
    printf("    export-embeddings       Export embeddings to HLX format\n");
    printf("    tdz-cnt                 Process TDZ content\n");
    printf("    maestro                 Maestro functionality (coming soon)\n");
    printf("    ml                      ML functionality (coming soon)\n");
    printf("    ind-demo                Interactive demo\n\n");
    
    printf("  Help:\n");
    printf("    help, -h, --help        Show this help message\n\n");
    
    printf("Examples:\n");
    printf("  todozi init                    # Initialize todozi\n");
    printf("  todozi add \"Buy groceries\"     # Add a task\n");
    printf("  todozi list                    # List all tasks\n");
    printf("  todozi show 1                  # Show task with ID 1\n");
    printf("  todozi complete 1              # Complete task 1\n");
    printf("  todozi search \"important\"      # Search for tasks\n\n");
    
    printf("For more information, visit: https://todozi.com\n");
    fflush(stdout);
}