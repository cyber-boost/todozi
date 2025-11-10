// example_commands.c
#include "types.c"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Helper function to create a new string (simulates allocation)
char* create_string(const char* str) {
    if (!str) return NULL;
    char* new_str = malloc(strlen(str) + 1);
    if (new_str) strcpy(new_str, str);
    return new_str;
}

// Example: Creating a complex command structure
Command create_add_task_command() {
    Command cmd = {0};
    cmd.type = CMD_ADD;
    cmd.data.add.type = CMD_ADD_TASK;
    
    // Fill in task details
    cmd.data.add.data.task.action = create_string("Implement user authentication");
    cmd.data.add.data.task.project = create_string("Security Module");
    cmd.data.add.data.task.priority = create_string("high");
    cmd.data.add.data.task.status = create_string("todo");
    cmd.data.add.data.task.assignee = create_string("dev-team");
    cmd.data.add.data.task.has_assignee = true;
    
    // Set progress (0-100%)
    cmd.data.add.data.task.progress.value = 0;
    cmd.data.add.data.task.progress.present = true;
    
    return cmd;
}

// Example: Creating an agent creation command
Command create_agent_command() {
    Command cmd = {0};
    cmd.type = CMD_AGENT;
    cmd.data.agent.type = CMD_AGENT_CREATE;
    
    // Fill agent details
    cmd.data.agent.data.create.id = create_string("agent-001");
    cmd.data.agent.data.create.name = create_string("Code Assistant");
    cmd.data.agent.data.create.description = create_string("AI helper for code generation");
    cmd.data.agent.data.create.model_provider = create_string("openai");
    cmd.data.agent.data.create.model_name = create_string("gpt-4");
    cmd.data.agent.data.create.temperature.value = 0.7f;
    cmd.data.agent.data.create.temperature.present = true;
    cmd.data.agent.data.create.max_tokens.value = 1024;
    cmd.data.agent.data.create.max_tokens.present = true;
    
    // Optional fields
    cmd.data.agent.data.create.has_capabilities = true;
    cmd.data.agent.data.create.capabilities = create_string("code-generation,explanation");
    
    return cmd;
}

// Example: Creating a training data command
Command create_training_command() {
    Command cmd = {0};
    cmd.type = CMD_TRAIN;
    cmd.data.train.type = CMD_TRAINING_CREATE;
    
    cmd.data.train.data.create.data_type = create_string("code-review");
    cmd.data.train.data.create.prompt = create_string("Review this Python function for security issues");
    cmd.data.train.data.create.completion = create_string("The function is secure, but consider adding input validation");
    cmd.data.train.data.create.source = create_string("internal-audit");
    cmd.data.train.data.create.quality.value = 0.95f;
    cmd.data.train.data.create.quality.present = true;
    
    return cmd;
}

// Example: Processing commands
void process_command(Command* cmd) {
    switch (cmd->type) {
        case CMD_ADD:
            if (cmd->data.add.type == CMD_ADD_TASK) {
                printf("Adding task: %s\n", cmd->data.add.data.task.action);
                printf("Project: %s, Priority: %s\n", 
                       cmd->data.add.data.task.project, 
                       cmd->data.add.data.task.priority);
            }
            break;
            
        case CMD_AGENT:
            if (cmd->data.agent.type == CMD_AGENT_CREATE) {
                printf("Creating agent: %s (%s)\n", 
                       cmd->data.agent.data.create.name,
                       cmd->data.agent.data.create.model_name);
            }
            break;
            
        case CMD_TRAIN:
            if (cmd->data.train.type == CMD_TRAINING_CREATE) {
                printf("Adding training data: %s\n", 
                       cmd->data.train.data.create.data_type);
                if (cmd->data.train.data.create.quality.present) {
                    printf("Quality score: %.2f\n", 
                           cmd->data.train.data.create.quality.value);
                }
            }
            break;
            
        default:
            printf("Processing command type: %d\n", cmd->type);
    }
}

// Cleanup function for commands
void cleanup_command(Command* cmd) {
    // In a real implementation, you would free all allocated strings
    // This is a simplified example
    switch (cmd->type) {
        case CMD_ADD:
            if (cmd->data.add.type == CMD_ADD_TASK) {
                free(cmd->data.add.data.task.action);
                free(cmd->data.add.data.task.project);
                free(cmd->data.add.data.task.priority);
                free(cmd->data.add.data.task.status);
                free(cmd->data.add.data.task.assignee);
            }
            break;
        // Add cleanup for other command types as needed
        default:
            break;
    }
}

// Main example
int main() {
    // Create example commands
    Command task_cmd = create_add_task_command();
    Command agent_cmd = create_agent_command();
    Command train_cmd = create_training_command();
    
    // Process the commands
    printf("=== Processing Commands ===\n");
    process_command(&task_cmd);
    printf("\n");
    process_command(&agent_cmd);
    printf("\n");
    process_command(&train_cmd);
    
    // Cleanup
    cleanup_command(&task_cmd);
    cleanup_command(&agent_cmd);
    cleanup_command(&train_cmd);
    
    return 0;
}
