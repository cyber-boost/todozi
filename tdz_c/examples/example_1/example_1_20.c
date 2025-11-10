#include "types.c"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Helper function to create a new string (simulates dynamic allocation)
char* create_string(const char* str) {
    if (!str) return NULL;
    char* new_str = malloc(strlen(str) + 1);
    strcpy(new_str, str);
    return new_str;
}

// Example function to create an AddTask command
Command create_add_task_command(
    const char* action,
    const char* project,
    const char* priority,
    unsigned char progress
) {
    Command cmd;
    cmd.type = CMD_ADD;
    
    // Initialize the AddCommand structure
    cmd.data.add.type = CMD_ADD_TASK;
    
    // Initialize the AddTaskCommand structure
    AddTaskCommand* task_cmd = &cmd.data.add.data.task;
    
    // Set required fields
    task_cmd->action = create_string(action);
    task_cmd->project = create_string(project);
    task_cmd->priority = create_string(priority);
    
    // Set optional fields with default values
    task_cmd->time = NULL;
    task_cmd->status = create_string("pending");
    task_cmd->assignee = NULL;
    task_cmd->tags = NULL;
    task_cmd->dependencies = NULL;
    task_cmd->context = NULL;
    
    // Set progress as optional value
    task_cmd->progress.value = progress;
    task_cmd->progress.present = true;
    
    return cmd;
}

// Example function to process a command
void process_command(Command* cmd) {
    switch (cmd->type) {
        case CMD_ADD:
            if (cmd->data.add.type == CMD_ADD_TASK) {
                AddTaskCommand* task = &cmd->data.add.data.task;
                printf("Adding task:\n");
                printf("  Action: %s\n", task->action);
                printf("  Project: %s\n", task->project ? task->project : "None");
                printf("  Priority: %s\n", task->priority ? task->priority : "None");
                if (task->progress.present) {
                    printf("  Progress: %d%%\n", task->progress.value);
                }
            }
            break;
            
        default:
            printf("Command type not handled in this example\n");
            break;
    }
}

// Example function to free command resources
void free_add_task_command(Command* cmd) {
    if (cmd->type == CMD_ADD && cmd->data.add.type == CMD_ADD_TASK) {
        AddTaskCommand* task = &cmd->data.add.data.task;
        free(task->action);
        free(task->project);
        free(task->priority);
        free(task->status);
        // Note: In a real implementation, we'd also free optional fields
    }
}

int main() {
    // Create a new task command
    Command cmd = create_add_task_command(
        "Implement user authentication",
        "WebApp Project",
        "high",
        25  // 25% complete
    );
    
    // Process the command
    process_command(&cmd);
    
    // Clean up resources
    free_add_task_command(&cmd);
    
    return 0;
}
