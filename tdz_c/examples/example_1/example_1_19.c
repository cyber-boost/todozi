#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include all the declarations from tests.c
// (In a real application, you'd include a header file)

// Forward declarations from tests.c
typedef struct Task Task;
typedef struct Project Project;
typedef struct TaskCollection TaskCollection;
typedef struct Config Config;

typedef enum {
    PRIORITY_LOW, PRIORITY_MEDIUM, PRIORITY_HIGH, PRIORITY_CRITICAL, PRIORITY_URGENT
} Priority;

typedef enum {
    STATUS_TODO, STATUS_IN_PROGRESS, STATUS_BLOCKED, STATUS_REVIEW, STATUS_DONE, STATUS_CANCELLED, STATUS_DEFERRED
} Status;

typedef enum {
    ASSIGNEE_AI, ASSIGNEE_HUMAN, ASSIGNEE_COLLABORATIVE
} Assignee;

// Function declarations (would normally be in a header)
extern Task* task_new_full(const char* action, const char* time, Priority priority,
                          const char* parent_project, Status status, Assignee* assignee,
                          char** tags, int tags_count, char** dependencies, int dependencies_count,
                          const char* context_notes, int* progress);
extern void task_update(Task* task, const char* new_action, Priority* new_priority,
                       Status* new_status, int* new_progress);
extern void task_complete(Task* task);
extern int task_is_active(Task* task);
extern void task_free(Task* task);

extern Project* project_new(const char* name, const char* description);
extern void project_add_task(Project* project, const char* task_id);
extern void project_free(Project* project);

extern TaskCollection* task_collection_new();
extern void task_collection_add_task(TaskCollection* collection, Task* task);
extern Task** task_collection_get_filtered_tasks(TaskCollection* collection, 
                                                Priority* priority_filter,
                                                const char* project_filter,
                                                Status* status_filter,
                                                int* count);
extern void task_collection_free(TaskCollection* collection);

extern int parse_priority(const char* str, Priority* result);
extern int parse_status(const char* str, Status* result);

// Helper function to print task details
void print_task(Task* task) {
    if (!task) return;
    
    const char* priority_str[] = {"Low", "Medium", "High", "Critical", "Urgent"};
    const char* status_str[] = {"To Do", "In Progress", "Blocked", "Review", "Done", "Cancelled", "Deferred"};
    
    printf("Task ID: %s\n", task->id);
    printf("Action: %s\n", task->action);
    printf("Time: %s\n", task->time);
    printf("Priority: %s\n", priority_str[task->priority]);
    printf("Status: %s\n", status_str[task->status]);
    printf("Project: %s\n", task->parent_project);
    if (task->assignee) {
        printf("Assignee: %s\n", task->assignee);
    }
    if (task->progress) {
        printf("Progress: %d%%\n", *task->progress);
    }
    printf("\n");
}

// Example usage of the task management system
int main() {
    printf("=== Task Management System Example ===\n\n");
    
    // Create a task collection to manage all tasks
    TaskCollection* collection = task_collection_new();
    
    // Create a project
    Project* project = project_new("Website Redesign", "Complete overhaul of company website");
    printf("Created project: %s\n\n", project->name);
    
    // Create tags for tasks
    char* frontend_tags[] = {"frontend", "ui", "design", NULL};
    char* backend_tags[] = {"backend", "api", "database", NULL};
    char* testing_tags[] = {"testing", "qa", NULL};
    
    // Create assignees
    Assignee ai_assignee = ASSIGNEE_AI;
    Assignee human_assignee = ASSIGNEE_HUMAN;
    Assignee collaborative_assignee = ASSIGNEE_COLLABORATIVE;
    
    // Create tasks with different properties
    printf("Creating tasks...\n");
    
    // Task 1: Frontend design
    int progress_25 = 25;
    Task* task1 = task_new_full(
        "Design new homepage layout",
        "8 hours",
        PRIORITY_HIGH,
        "Website Redesign",
        STATUS_IN_PROGRESS,
        &human_assignee,
        frontend_tags,
        3,
        NULL,  // No dependencies
        0,
        "Focus on mobile-first approach",
        &progress_25
    );
    
    // Task 2: Backend API
    Task* task2 = task_new_full(
        "Implement user authentication API",
        "12 hours",
        PRIORITY_CRITICAL,
        "Website Redesign",
        STATUS_TODO,
        &collaborative_assignee,
        backend_tags,
        3,
        NULL,  // No dependencies
        0,
        "Use JWT for token management",
        NULL  // No progress yet
    );
    
    // Task 3: Testing (depends on task1 and task2)
    char* dependencies[] = {task1->id, task2->id, NULL};
    Task* task3 = task_new_full(
        "Perform end-to-end testing",
        "6 hours",
        PRIORITY_MEDIUM,
        "Website Redesign",
        STATUS_TODO,
        &ai_assignee,
        testing_tags,
        2,
        dependencies,
        2,
        "Automate test cases where possible",
        NULL
    );
    
    // Add tasks to collection
    task_collection_add_task(collection, task1);
    task_collection_add_task(collection, task2);
    task_collection_add_task(collection, task3);
    
    // Add tasks to project
    project_add_task(project, task1->id);
    project_add_task(project, task2->id);
    project_add_task(project, task3->id);
    
    printf("Created %d tasks\n\n", collection->tasks_count);
    
    // Display all tasks
    printf("All tasks:\n");
    printf("----------\n");
    for (int i = 0; i < collection->tasks_count; i++) {
        print_task(collection->tasks[i]);
    }
    
    // Update a task
    printf("Updating task...\n");
    Priority new_priority = PRIORITY_URGENT;
    Status new_status = STATUS_IN_PROGRESS;
    int new_progress = 60;
    task_update(task2, "Implement user authentication and profile API", &new_priority, &new_status, &new_progress);
    
    // Complete a task
    printf("Completing task...\n");
    task_complete(task1);
    
    // Display updated tasks
    printf("\nUpdated tasks:\n");
    printf("--------------\n");
    for (int i = 0; i < collection->tasks_count; i++) {
        print_task(collection->tasks[i]);
    }
    
    // Filter tasks by priority
    printf("High priority tasks:\n");
    printf("-------------------\n");
    Priority high_priority = PRIORITY_HIGH;
    int high_count;
    Task** high_priority_tasks = task_collection_get_filtered_tasks(
        collection, &high_priority, NULL, NULL, &high_count);
    
    for (int i = 0; i < high_count; i++) {
        print_task(high_priority_tasks[i]);
    }
    free(high_priority_tasks);
    
    // Filter tasks by status
    printf("In-progress tasks:\n");
    printf("------------------\n");
    Status in_progress = STATUS_IN_PROGRESS;
    int in_progress_count;
    Task** in_progress_tasks = task_collection_get_filtered_tasks(
        collection, NULL, NULL, &in_progress, &in_progress_count);
    
    for (int i = 0; i < in_progress_count; i++) {
        print_task(in_progress_tasks[i]);
    }
    free(in_progress_tasks);
    
    // Show active tasks
    printf("Active tasks:\n");
    printf("-------------\n");
    for (int i = 0; i < collection->tasks_count; i++) {
        if (task_is_active(collection->tasks[i])) {
            print_task(collection->tasks[i]);
        }
    }
    
    // Clean up
    task_free(task1);
    task_free(task2);
    task_free(task3);
    project_free(project);
    task_collection_free(collection);
    
    printf("Example completed successfully!\n");
    return 0;
}
