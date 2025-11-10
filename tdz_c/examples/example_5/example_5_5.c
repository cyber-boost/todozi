#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// External function declarations (from cli.c)
typedef enum {
    TODOZI_SUCCESS,
    TODOZI_ERROR_VALIDATION,
    TODOZI_ERROR_IO,
    TODOZI_ERROR_PARSE
} TodoziResult;

typedef struct Storage Storage;
typedef struct TodoziHandler TodoziHandler;

extern Storage* storage_new(void);
extern TodoziHandler* todozi_handler_new(Storage* storage);
extern void todozi_handler_free(TodoziHandler* handler);
extern void storage_free(Storage* storage);
extern TodoziResult handle_command(TodoziHandler* handler, const char* command, 
                                  const char* subcommand, const char** args, int arg_count);

int main() {
    // Initialize Todozi system
    Storage* storage = storage_new();
    if (!storage) {
        fprintf(stderr, "Failed to initialize storage\n");
        return 1;
    }

    TodoziHandler* handler = todozi_handler_new(storage);
    if (!handler) {
        fprintf(stderr, "Failed to create handler\n");
        storage_free(storage);
        return 1;
    }

    printf("=== Todozi Project Management Workflow ===\n\n");

    // 1. Create a new project
    const char* project_args[] = {
        "--name", "WebsiteRedesign",
        "--description", "Complete redesign of company website"
    };
    handle_command(handler, "project", "create", project_args, 4);

    // 2. Add tasks to the project
    const char* task1_args[] = {
        "--action", "Design homepage mockup",
        "--time", "4 hours",
        "--priority", "high",
        "--project", "WebsiteRedesign",
        "--status", "todo",
        "--assignee", "human",
        "--tags", "design,ui"
    };
    handle_command(handler, "add", "task", task1_args, 14);

    const char* task2_args[] = {
        "--action", "Implement responsive navigation",
        "--time", "6 hours",
        "--priority", "medium",
        "--project", "WebsiteRedesign",
        "--status", "todo",
        "--assignee", "ai",
        "--tags", "frontend,css"
    };
    handle_command(handler, "add", "task", task2_args, 14);

    // 3. Create a memory about project goals
    const char* memory_args[] = {
        "--moment", "Project kickoff meeting",
        "--meaning", "Align team on redesign objectives",
        "--reason", "Ensure consistent vision",
        "--importance", "high",
        "--term", "user experience",
        "--type", "human"
    };
    handle_command(handler, "memory", "create", memory_args, 12);

    // 4. Create an AI agent for code review
    const char* agent_args[] = {
        "--id", "code-reviewer",
        "--name", "Code Review Assistant",
        "--description", "Reviews frontend code for best practices",
        "--category", "code-quality",
        "--model-provider", "openai",
        "--model-name", "gpt-4",
        "--temperature", "0.3",
        "--max-tokens", "500"
    };
    handle_command(handler, "agent", "create", agent_args, 16);

    // 5. Assign agent to a task
    const char* assign_args[] = {
        "code-reviewer",
        "task-id-placeholder",
        "WebsiteRedesign"
    };
    handle_command(handler, "agent", "assign", assign_args, 3);

    // 6. List all tasks in the project
    const char* list_args[] = {
        "--project", "WebsiteRedesign"
    };
    handle_command(handler, "list", "tasks", list_args, 2);

    // 7. Show project statistics
    handle_command(handler, "stats", NULL, NULL, 0);

    // Cleanup
    todozi_handler_free(handler);
    storage_free(storage);

    printf("\n=== Workflow completed successfully ===\n");
    return 0;
}
