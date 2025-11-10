// example4.c - Complete Task Management Workflow
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include storage functions (assuming they're in storage.h)
// #include "storage.h"

// Forward declarations for structures used in this example
typedef struct Task Task;
typedef struct Project Project;
typedef struct Config Config;

// Simplified structure definitions for this example
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

struct Project {
    char* name;
    char* description;
};

struct Config {
    char* version;
    char* default_project;
    int auto_backup;
    char* backup_interval;
    int ai_enabled;
    char* default_assignee;
    char* date_format;
    char* timezone;
};

// External function declarations (from storage.c)
extern int init_storage();
extern int save_task(Task* task);
extern Task* load_task(const char* task_id);
extern int save_project(Project* project);
extern Project* load_project(const char* project_name);
extern Config* load_config();
extern int save_config(Config* config);
extern void free_task(Task* task);
extern void free_project(Project* project);
extern void free_config(Config* config);

// Helper function to generate unique task IDs
char* generate_task_id() {
    static int counter = 1000;
    char* id = malloc(16);
    snprintf(id, 16, "task_%d", counter++);
    return id;
}

// Helper function to create a new task
Task* create_task(const char* action, const char* project_name) {
    Task* task = malloc(sizeof(Task));
    if (!task) return NULL;
    
    task->id = generate_task_id();
    task->action = strdup(action);
    task->status = strdup("active");
    task->priority = strdup("medium");
    task->parent_project = strdup(project_name);
    task->created_at = time(NULL);
    task->updated_at = task->created_at;
    task->context_notes = NULL;
    task->embedding_vector = NULL;
    task->embedding_size = 0;
    
    return task;
}

// Helper function to create a new project
Project* create_project(const char* name, const char* description) {
    Project* project = malloc(sizeof(Project));
    if (!project) return NULL;
    
    project->name = strdup(name);
    project->description = strdup(description);
    
    return project;
}

// Helper function to create default config
Config* create_default_config() {
    Config* config = malloc(sizeof(Config));
    if (!config) return NULL;
    
    config->version = strdup("1.2.0");
    config->default_project = strdup("general");
    config->auto_backup = 1;
    config->backup_interval = strdup("daily");
    config->ai_enabled = 1;
    config->default_assignee = NULL;
    config->date_format = strdup("%Y-%m-%d %H:%M:%S");
    config->timezone = strdup("UTC");
    
    return config;
}

int main() {
    printf("🚀 Todozi Task Management Workflow Example\n");
    printf("==========================================\n");
    
    // Step 1: Initialize storage system
    printf("1. Initializing storage system...\n");
    if (init_storage() != 0) {
        printf("❌ Failed to initialize storage\n");
        return 1;
    }
    printf("✅ Storage initialized successfully\n\n");
    
    // Step 2: Load or create configuration
    printf("2. Loading configuration...\n");
    Config* config = load_config();
    if (!config) {
        printf("📝 Creating default configuration...\n");
        config = create_default_config();
        // In a real app, we would save it here
    }
    printf("✅ Configuration loaded (version: %s)\n\n", config->version);
    
    // Step 3: Create a new project
    printf("3. Creating new project...\n");
    Project* project = create_project("website-redesign", "Complete redesign of company website");
    if (!project) {
        printf("❌ Failed to create project\n");
        free_config(config);
        return 1;
    }
    
    // Save project to storage
    if (save_project(project) != 0) {
        printf("❌ Failed to save project\n");
        free_project(project);
        free_config(config);
        return 1;
    }
    printf("✅ Project '%s' created and saved\n\n", project->name);
    
    // Step 4: Create multiple tasks
    printf("4. Creating tasks...\n");
    Task* tasks[3];
    const char* actions[] = {
        "Design new homepage layout",
        "Implement responsive navigation",
        "Write SEO-optimized content"
    };
    
    for (int i = 0; i < 3; i++) {
        tasks[i] = create_task(actions[i], project->name);
        if (!tasks[i]) {
            printf("❌ Failed to create task %d\n", i+1);
            // Cleanup previously created tasks
            for (int j = 0; j < i; j++) {
                free_task(tasks[j]);
            }
            free_project(project);
            free_config(config);
            return 1;
        }
        
        // Save task to storage
        if (save_task(tasks[i]) != 0) {
            printf("❌ Failed to save task %s\n", tasks[i]->id);
            free_task(tasks[i]);
            // Cleanup other tasks
            for (int j = 0; j < i; j++) {
                free_task(tasks[j]);
            }
            free_project(project);
            free_config(config);
            return 1;
        }
        printf("✅ Task '%s' created: %s\n", tasks[i]->id, tasks[i]->action);
    }
    printf("\n");
    
    // Step 5: Load and verify tasks
    printf("5. Verifying saved tasks...\n");
    for (int i = 0; i < 3; i++) {
        Task* loaded_task = load_task(tasks[i]->id);
        if (loaded_task) {
            printf("✅ Loaded task %s: %s (Status: %s)\n", 
                   loaded_task->id, 
                   loaded_task->action, 
                   loaded_task->status);
            free_task(loaded_task);
        } else {
            printf("❌ Failed to load task %s\n", tasks[i]->id);
        }
    }
    printf("\n");
    
    // Step 6: Load and verify project
    printf("6. Verifying saved project...\n");
    Project* loaded_project = load_project(project->name);
    if (loaded_project) {
        printf("✅ Loaded project '%s': %s\n", 
               loaded_project->name, 
               loaded_project->description);
        free_project(loaded_project);
    } else {
        printf("❌ Failed to load project '%s'\n", project->name);
    }
    printf("\n");
    
    // Step 7: Demonstrate task completion workflow
    printf("7. Completing a task...\n");
    Task* task_to_complete = tasks[0];  // First task
    free(task_to_complete->status);
    task_to_complete->status = strdup("completed");
    task_to_complete->updated_at = time(NULL);
    
    if (save_task(task_to_complete) == 0) {
        printf("✅ Task %s marked as completed\n", task_to_complete->id);
    } else {
        printf("❌ Failed to update task status\n");
    }
    printf("\n");
    
    // Step 8: Summary
    printf("8. Workflow Summary\n");
    printf("==================\n");
    printf("Project: %s\n", project->name);
    printf("Tasks created: %d\n", 3);
    printf("Tasks completed: %d\n", 1);
    printf("Storage location: ~/.todozi\n");
    printf("\n✅ Task management workflow completed successfully!\n");
    
    // Cleanup
    for (int i = 0; i < 3; i++) {
        free_task(tasks[i]);
    }
    free_project(project);
    free_config(config);
    
    return 0;
}
