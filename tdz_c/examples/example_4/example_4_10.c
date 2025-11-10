#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "models.c"  // Include the main models implementation

int main() {
    // Initialize error handling
    TodoziError err = {0};
    TodoziResult result;

    // Create a human-assigned task
    Task* task1 = NULL;
    result = todozi_task_new_full(
        "user123",                           // user_id
        "Review project documentation",      // action
        "2023-12-01T10:00:00Z",              // time
        PRIORITY_HIGH,                       // priority
        "Documentation Project",             // parent_project
        STATUS_TODO,                         // status
        ASSIGNEE_HUMAN,                      // assignee_type
        NULL,                                // assignee_agent_name
        (char*[]){"documentation", "review"},// tags
        2,                                   // tags_count
        NULL,                                // dependencies
        0,                                   // dependencies_count
        "Check for completeness and clarity",// context_notes
        NULL,                                // progress
        &task1,                              // output task
        &err                                 // error
    );

    if (result != TODOZI_OK) {
        printf("Error creating task1: %s\n", err.msg);
        free(err.msg);
        return 1;
    }

    printf("Created task1:\n");
    printf("  ID: %s\n", todozi_task_id(task1));
    printf("  Action: %s\n", todozi_task_action(task1));
    printf("  Assignee: ");
    char assignee_buf[128];
    todozi_assignee_to_string(
        todozi_task_assignee_type(task1),
        todozi_task_assignee_agent_name(task1),
        assignee_buf,
        sizeof(assignee_buf)
    );
    printf("%s\n", assignee_buf);

    // Create an AI-assigned task
    Task* task2 = NULL;
    result = todozi_task_new_full(
        "user123",
        "Generate summary report",
        "2023-12-02T14:00:00Z",
        PRIORITY_MEDIUM,
        "Analytics Project",
        STATUS_PENDING,
        ASSIGNEE_AI,
        NULL,
        (char*[]){"report", "analytics"},
        2,
        (char*[]){todozi_task_id(task1)},    // depends on task1
        1,
        "Use last quarter's data",
        NULL,
        &task2,
        &err
    );

    if (result != TODOZI_OK) {
        printf("Error creating task2: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task1);
        return 1;
    }

    printf("\nCreated task2:\n");
    printf("  ID: %s\n", todozi_task_id(task2));
    printf("  Action: %s\n", todozi_task_action(task2));
    printf("  Dependencies: ");
    size_t dep_count;
    const char* const* deps = todozi_task_dependencies(task2, &dep_count);
    for (size_t i = 0; i < dep_count; i++) {
        printf("%s ", deps[i]);
    }
    printf("\n");

    // Create an agent-assigned task
    Task* task3 = NULL;
    result = todozi_task_new_full(
        "user123",
        "Schedule team meeting",
        "2023-12-05T09:00:00Z",
        PRIORITY_LOW,
        "Team Coordination",
        STATUS_TODO,
        ASSIGNEE_AGENT,
        "CalendarBot",
        (char*[]){"meeting", "schedule"},
        2,
        NULL,
        0,
        "Include all project stakeholders",
        NULL,
        &task3,
        &err
    );

    if (result != TODOZI_OK) {
        printf("Error creating task3: %s\n", err.msg);
        free(err.msg);
        todozi_task_free(task1);
        todozi_task_free(task2);
        return 1;
    }

    printf("\nCreated task3:\n");
    printf("  ID: %s\n", todozi_task_id(task3));
    printf("  Action: %s\n", todozi_task_action(task3));
    printf("  Assignee: ");
    todozi_assignee_to_string(
        todozi_task_assignee_type(task3),
        todozi_task_assignee_agent_name(task3),
        assignee_buf,
        sizeof(assignee_buf)
    );
    printf("%s\n", assignee_buf);

    // Update task1 to mark as in progress
    TaskUpdate* update = NULL;
    result = todozi_task_update_new(&update, &err);
    if (result != TODOZI_OK) {
        printf("Error creating update: %s\n", err.msg);
        free(err.msg);
        goto cleanup;
    }

    Status new_status = STATUS_IN_PROGRESS;
    result = todozi_task_update_with_status(update, new_status, &err);
    if (result != TODOZI_OK) {
        printf("Error setting status: %s\n", err.msg);
        free(err.msg);
        goto cleanup;
    }

    uint8_t progress = 30;
    result = todozi_task_update_with_progress(update, progress, &err);
    if (result != TODOZI_OK) {
        printf("Error setting progress: %s\n", err.msg);
        free(err.msg);
        goto cleanup;
    }

    result = todozi_task_update(task1, update, &err);
    if (result != TODOZI_OK) {
        printf("Error updating task1: %s\n", err.msg);
        free(err.msg);
        goto cleanup;
    }

    printf("\nUpdated task1:\n");
    printf("  Status: ");
    char status_buf[32];
    todozi_status_to_string(todozi_task_status(task1), status_buf, sizeof(status_buf));
    printf("%s\n", status_buf);
    printf("  Progress: %d%%\n", *todozi_task_progress(task1));

    // Complete task2
    todozi_task_complete(task2);
    printf("\nCompleted task2:\n");
    printf("  Status: ");
    todozi_status_to_string(todozi_task_status(task2), status_buf, sizeof(status_buf));
    printf("%s\n", status_buf);
    printf("  Progress: %d%%\n", *todozi_task_progress(task2));

cleanup:
    todozi_task_update_free(update);
    todozi_task_free(task1);
    todozi_task_free(task2);
    todozi_task_free(task3);

    return 0;
}
