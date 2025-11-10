#include "types.c"  // Include the header where all types are defined
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // Step 1: Initialize an AddTaskCommand
    AddTaskCommand add_task = {
        .action = "Implement user authentication",
        .time = "2025-04-05T14:00:00Z",
        .priority = "high",
        .project = "auth-system",
        .status = "pending",
        .assignee = NULL,             // Optional field, not set
        .tags = NULL,                 // Optional field, not set
        .dependencies = NULL,         // Optional field, not set
        .context = NULL,              // Optional field, not set
        .progress = {0, false}        // Not started yet
    };

    // Step 2: Wrap it inside an AddCommand
    AddCommand add_cmd = {
        .type = CMD_ADD_TASK,
        .data.task = add_task
    };

    // Step 3: Wrap the AddCommand into the main Command struct
    Command cmd = {
        .type = CMD_ADD,
        .data.add = add_cmd
    };

    // Step 4: Print out some fields to verify
    printf("Command Type: %d\n", cmd.type);
    printf("Add Command Type: %d\n", cmd.data.add.type);
    printf("Task Action: %s\n", cmd.data.add.data.task.action);
    printf("Task Priority: %s\n", cmd.data.add.data.task.priority);
    printf("Has Assignee? %s\n", cmd.data.add.data.task.has_assignee ? "Yes" : "No");

    return 0;
}
