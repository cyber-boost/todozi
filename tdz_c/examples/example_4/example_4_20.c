#include "types.c"  // Include the provided types
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Helper function to safely duplicate strings
char* safe_strdup(const char* str) {
    if (!str) return NULL;
    char* dup = malloc(strlen(str) + 1);
    if (dup) strcpy(dup, str);
    return dup;
}

// Function to create an AddTaskCommand
AddTaskCommand create_add_task_command(
    const char* action,
    const char* time,
    const char* priority,
    const char* project,
    const char* status,
    const char* assignee,
    const char* tags,
    const char* dependencies,
    const char* context,
    unsigned char progress
) {
    AddTaskCommand cmd = {0};
    cmd.action = safe_strdup(action);
    cmd.time = safe_strdup(time);
    cmd.priority = safe_strdup(priority);
    cmd.project = safe_strdup(project);
    cmd.status = safe_strdup(status);
    cmd.assignee = safe_strdup(assignee);
    cmd.tags = safe_strdup(tags);
    cmd.dependencies = safe_strdup(dependencies);
    cmd.context = safe_strdup(context);
    cmd.progress.value = progress;
    cmd.progress.present = true;
    return cmd;
}

// Function to free AddTaskCommand resources
void free_add_task_command(AddTaskCommand* cmd) {
    if (!cmd) return;
    free(cmd->action);
    free(cmd->time);
    free(cmd->priority);
    free(cmd->project);
    free(cmd->status);
    free(cmd->assignee);
    free(cmd->tags);
    free(cmd->dependencies);
    free(cmd->context);
    memset(cmd, 0, sizeof(AddTaskCommand));
}

// Main example function
int main() {
    // 1. Create an AddTaskCommand
    AddTaskCommand add_task = create_add_task_command(
        "Implement user authentication",
        "2025-04-15T14:00:00Z",
        "high",
        "auth-module",
        "pending",
        "dev-team",
        "security,backend",
        "database-setup",
        "Part of login feature",
        0  // 0% progress
    );

    // 2. Wrap it in the main Command structure
    Command cmd = {0};
    cmd.type = CMD_ADD;
    cmd.data.add.type = CMD_ADD_TASK;
    cmd.data.add.data.task = add_task;

    // 3. Simulate processing (in real app, this would call command handler)
    printf("Created ADD TASK command:\n");
    printf("  Action: %s\n", cmd.data.add.data.task.action);
    printf("  Priority: %s\n", cmd.data.add.data.task.priority);
    printf("  Progress: %u%%\n", cmd.data.add.data.task.progress.value);

    // 4. Cleanup
    free_add_task_command(&cmd.data.add.data.task);

    return 0;
}
