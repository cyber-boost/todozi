// example3.c - Project and Task Management Example
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Include the storage functions (in practice, you'd use a header file)
extern int init_storage();
extern int save_project(Project* project);
extern Project* load_project(const char* project_name);
extern int save_task(Task* task);
extern Task* load_task(const char* task_id);
extern int save_project_task_container(ProjectTaskContainer* container);
extern ProjectTaskContainer* load_project_task_container(const char* project_name);
extern void free_project(Project* project);
extern void free_task(Task* task);
extern void free_project_task_container(ProjectTaskContainer* container);

// Helper function to generate unique IDs
char* generate_task_id() {
    static int counter = 1;
    char* id = malloc(20);
    snprintf(id, 20, "task_%d", counter++);
    return id;
}

int main() {
    printf("🔧 Initializing Todozi storage system...\n");
    
    // Initialize the storage system
    if (init_storage() != 0) {
        printf("❌ Failed to initialize storage\n");
        return 1;
    }
    printf("✅ Storage initialized successfully\n\n");
    
    // Create a new project
    printf("📁 Creating a new project...\n");
    Project* project = malloc(sizeof(Project));
    project->name = strdup("Website Redesign");
    project->description = strdup("Complete overhaul of company website");
    
    if (save_project(project) != 0) {
        printf("❌ Failed to save project\n");
        free_project(project);
        return 1;
    }
    printf("✅ Project '%s' created successfully\n\n", project->name);
    
    // Load the project to verify
    Project* loaded_project = load_project("Website Redesign");
    if (loaded_project) {
        printf("🔍 Loaded project: %s\n", loaded_project->name);
        printf("📝 Description: %s\n\n", 
               loaded_project->description ? loaded_project->description : "No description");
        free_project(loaded_project);
    }
    
    // Create project task container
    printf("📦 Creating project task container...\n");
    ProjectTaskContainer* container = load_project_task_container("Website Redesign");
    if (container) {
        if (save_project_task_container(container) == 0) {
            printf("✅ Project task container saved\n");
            printf("🔑 Project hash: %s\n\n", container->project_hash);
        }
        free_project_task_container(container);
    }
    
    // Create some tasks for the project
    printf("📋 Creating tasks for the project...\n");
    
    // Task 1
    Task* task1 = malloc(sizeof(Task));
    task1->id = generate_task_id();
    task1->action = strdup("Design new homepage layout");
    task1->status = strdup("active");
    task1->priority = strdup("high");
    task1->parent_project = strdup("Website Redesign");
    task1->created_at = time(NULL);
    task1->updated_at = time(NULL);
    task1->context_notes = strdup("Focus on mobile-first design");
    task1->embedding_vector = NULL;
    task1->embedding_size = 0;
    
    if (save_task(task1) == 0) {
        printf("✅ Task created: %s\n", task1->action);
    }
    
    // Task 2
    Task* task2 = malloc(sizeof(Task));
    task2->id = generate_task_id();
    task2->action = strdup("Implement responsive navigation");
    task2->status = strdup("pending");
    task2->priority = strdup("medium");
    task2->parent_project = strdup("Website Redesign");
    task2->created_at = time(NULL);
    task2->updated_at = time(NULL);
    task2->context_notes = NULL;
    task2->embedding_vector = NULL;
    task2->embedding_size = 0;
    
    if (save_task(task2) == 0) {
        printf("✅ Task created: %s\n", task2->action);
    }
    
    // Task 3
    Task* task3 = malloc(sizeof(Task));
    task3->id = generate_task_id();
    task3->action = strdup("Optimize images for web");
    task3->status = strdup("completed");
    task3->priority = strdup("low");
    task3->parent_project = strdup("Website Redesign");
    task3->created_at = time(NULL);
    task3->updated_at = time(NULL);
    task3->context_notes = strdup("Use WebP format for better compression");
    task3->embedding_vector = NULL;
    task3->embedding_size = 0;
    
    if (save_task(task3) == 0) {
        printf("✅ Task created: %s\n\n", task3->action);
    }
    
    // Load and display tasks
    printf("📂 Loading tasks...\n");
    
    Task* loaded_task1 = load_task(task1->id);
    if (loaded_task1) {
        printf("📋 Task ID: %s\n", loaded_task1->id);
        printf("📝 Action: %s\n", loaded_task1->action);
        printf("📊 Status: %s\n", loaded_task1->status);
        printf("📌 Priority: %s\n", loaded_task1->priority);
        printf("🏢 Project: %s\n", loaded_task1->parent_project);
        if (loaded_task1->context_notes) {
            printf("📄 Notes: %s\n", loaded_task1->context_notes);
        }
        printf("--------------------\n");
        free_task(loaded_task1);
    }
    
    Task* loaded_task2 = load_task(task2->id);
    if (loaded_task2) {
        printf("📋 Task ID: %s\n", loaded_task2->id);
        printf("📝 Action: %s\n", loaded_task2->action);
        printf("📊 Status: %s\n", loaded_task2->status);
        printf("📌 Priority: %s\n", loaded_task2->priority);
        printf("🏢 Project: %s\n", loaded_task2->parent_project);
        printf("--------------------\n");
        free_task(loaded_task2);
    }
    
    Task* loaded_task3 = load_task(task3->id);
    if (loaded_task3) {
        printf("📋 Task ID: %s\n", loaded_task3->id);
        printf("📝 Action: %s\n", loaded_task3->action);
        printf("📊 Status: %s\n", loaded_task3->status);
        printf("📌 Priority: %s\n", loaded_task3->priority);
        printf("🏢 Project: %s\n", loaded_task3->parent_project);
        if (loaded_task3->context_notes) {
            printf("📄 Notes: %s\n", loaded_task3->context_notes);
        }
        printf("--------------------\n");
        free_task(loaded_task3);
    }
    
    // Cleanup
    free_task(task1);
    free_task(task2);
    free_task(task3);
    free_project(project);
    
    printf("\n🎉 Example completed successfully!\n");
    printf("📁 Check your ~/.todozi directory to see the created files\n");
    
    return 0;
}
