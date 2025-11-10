// example5.c - Task Management with Storage
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the storage functions (in practice, you'd use a header file)
extern int init_storage();
extern int save_task(struct Task* task);
extern struct Task* load_task(const char* task_id);
extern void free_task(struct Task* task);

// Task structure (simplified from storage.c)
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

int main() {
    // Initialize the storage system
    if (init_storage() != 0) {
        printf("❌ Failed to initialize storage\n");
        return 1;
    }
    printf("✅ Storage initialized successfully\n");

    // Create a new task
    struct Task* new_task = malloc(sizeof(struct Task));
    if (!new_task) {
        printf("❌ Memory allocation failed\n");
        return 1;
    }

    // Initialize task fields
    new_task->id = strdup("task-001");
    new_task->action = strdup("Implement user authentication");
    new_task->status = strdup("active");
    new_task->priority = strdup("high");
    new_task->parent_project = strdup("general");
    new_task->created_at = time(NULL);
    new_task->updated_at = time(NULL);
    new_task->context_notes = strdup("Use OAuth 2.0 for secure login");
    new_task->embedding_vector = NULL;
    new_task->embedding_size = 0;

    // Save the task to storage
    if (save_task(new_task) != 0) {
        printf("❌ Failed to save task\n");
        free_task(new_task);
        return 1;
    }
    printf("✅ Task saved successfully\n");
    printf("   ID: %s\n", new_task->id);
    printf("   Action: %s\n", new_task->action);
    printf("   Status: %s\n", new_task->status);

    // Load the task back from storage
    struct Task* loaded_task = load_task("task-001");
    if (!loaded_task) {
        printf("❌ Failed to load task\n");
        free_task(new_task);
        return 1;
    }

    // Verify loaded task data
    printf("✅ Task loaded successfully\n");
    printf("   ID: %s\n", loaded_task->id);
    printf("   Action: %s\n", loaded_task->action);
    printf("   Status: %s\n", loaded_task->status);
    printf("   Priority: %s\n", loaded_task->priority);
    printf("   Project: %s\n", loaded_task->parent_project);

    // Clean up memory
    free_task(new_task);
    free_task(loaded_task);

    return 0;
}
