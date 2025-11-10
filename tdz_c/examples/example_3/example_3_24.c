// example3_workflow.c - Practical workflow using multiple Todozi tools
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Assuming todozi_tool.c is compiled as a library or included
// For this example, we'll simulate the interface

// Simulated function declarations (would be in header file in real usage)
typedef struct Tool Tool;
typedef struct HashMap HashMap;
typedef struct ToolResult ToolResult;
typedef struct SharedTodozi SharedTodozi;
typedef struct Storage Storage;

// Factory functions
extern Tool* create_task_tool_new(SharedTodozi* todozi);
extern Tool* create_memory_tool_new(SharedTodozi* todozi);
extern Tool* create_idea_tool_new(SharedTodozi* todozi);
extern Tool* checklist_tool_new(SharedTodozi* todozi);
extern Tool* unified_search_tool_new(SharedTodozi* todozi);

// Utility functions
extern HashMap* hashmap_new();
extern void hashmap_free(HashMap* map);
extern void hashmap_set(HashMap* map, const char* key, const char* value);
extern ToolResult* tool_result_success(const char* message, int confidence);
extern void tool_result_free(ToolResult* result);
extern SharedTodozi* shared_todozi_new(Storage* storage);
extern void shared_todozi_free(SharedTodozi* todozi);

// Tool methods
extern void (*tool_destroy_fn)(Tool* self);
extern ToolResult* (*tool_exec_fn)(const Tool* self, const HashMap* kwargs);

// Simulated project planning workflow
void run_project_planning_workflow() {
    printf("=== Todozi Project Planning Workflow ===\n\n");
    
    // Initialize system
    Storage storage = {0};
    SharedTodozi* todozi = shared_todozi_new(&storage);
    if (!todozi) {
        printf("❌ Failed to initialize Todozi system\n");
        return;
    }
    
    // Create tools
    Tool* task_tool = create_task_tool_new(todozi);
    Tool* memory_tool = create_memory_tool_new(todozi);
    Tool* idea_tool = create_idea_tool_new(todozi);
    Tool* checklist_tool = checklist_tool_new(todozi);
    Tool* search_tool = unified_search_tool_new(todozi);
    
    if (!task_tool || !memory_tool || !idea_tool || !checklist_tool || !search_tool) {
        printf("❌ Failed to create tools\n");
        return;
    }
    
    // Step 1: Create initial project idea
    printf("Step 1: Creating project idea...\n");
    HashMap* idea_params = hashmap_new();
    hashmap_set(idea_params, "idea", "Build a mobile app for habit tracking");
    hashmap_set(idea_params, "importance", "high");
    hashmap_set(idea_params, "tags", "mobile,productivity,health");
    
    ToolResult* idea_result = idea_tool->execute(idea_tool, idea_params);
    if (idea_result && idea_result->success) {
        printf("✅ %s\n\n", idea_result->message);
    }
    tool_result_free(idea_result);
    hashmap_free(idea_params);
    
    // Step 2: Extract tasks from project specification
    printf("Step 2: Extracting tasks from specification...\n");
    HashMap* spec_params = hashmap_new();
    hashmap_set(spec_params, "content", 
        "We need to build a habit tracking app with these features:\n"
        "- User authentication\n"
        "- Daily habit logging\n"
        "- Progress visualization\n"
        "- Social sharing capabilities");
    hashmap_set(spec_params, "project", "HabitTracker App");
    hashmap_set(spec_params, "priority", "medium");
    hashmap_set(spec_params, "assignee", "human");
    
    ToolResult* checklist_result = checklist_tool->execute(checklist_tool, spec_params);
    if (checklist_result && checklist_result->success) {
        printf("✅ %s\n\n", checklist_result->message);
    }
    tool_result_free(checklist_result);
    hashmap_free(spec_params);
    
    // Step 3: Create specific development tasks
    printf("Step 3: Creating development tasks...\n");
    
    // Task 1: User authentication
    HashMap* auth_params = hashmap_new();
    hashmap_set(auth_params, "action", "Implement user authentication system");
    hashmap_set(auth_params, "time", "3 days");
    hashmap_set(auth_params, "priority", "high");
    hashmap_set(auth_params, "project", "HabitTracker App");
    hashmap_set(auth_params, "assignee", "ai");
    hashmap_set(auth_params, "tags", "backend,security");
    
    ToolResult* auth_result = task_tool->execute(task_tool, auth_params);
    if (auth_result && auth_result->success) {
        printf("✅ %s\n", auth_result->message);
    }
    tool_result_free(auth_result);
    hashmap_free(auth_params);
    
    // Task 2: Habit logging feature
    HashMap* logging_params = hashmap_new();
    hashmap_set(logging_params, "action", "Design and implement habit logging UI");
    hashmap_set(logging_params, "time", "2 days");
    hashmap_set(logging_params, "priority", "high");
    hashmap_set(logging_params, "project", "HabitTracker App");
    hashmap_set(logging_params, "assignee", "human");
    hashmap_set(logging_params, "tags", "frontend,ui");
    
    ToolResult* logging_result = task_tool->execute(task_tool, logging_params);
    if (logging_result && logging_result->success) {
        printf("✅ %s\n\n", logging_result->message);
    }
    tool_result_free(logging_result);
    hashmap_free(logging_params);
    
    // Step 4: Record important project decisions
    printf("Step 4: Recording project decisions...\n");
    HashMap* memory_params = hashmap_new();
    hashmap_set(memory_params, "moment", "Decided to use React Native for cross-platform development");
    hashmap_set(memory_params, "meaning", "Faster development and single codebase for iOS/Android");
    hashmap_set(memory_params, "reason", "Team has React experience and budget is limited");
    hashmap_set(memory_params, "importance", "high");
    hashmap_set(memory_params, "term", "long");
    hashmap_set(memory_params, "tags", "tech-stack,mobile");
    
    ToolResult* memory_result = memory_tool->execute(memory_tool, memory_params);
    if (memory_result && memory_result->success) {
        printf("✅ %s\n\n", memory_result->message);
    }
    tool_result_free(memory_result);
    hashmap_free(memory_params);
    
    // Step 5: Search for related information
    printf("Step 5: Searching for related information...\n");
    HashMap* search_params = hashmap_new();
    hashmap_set(search_params, "query", "habit tracking app best practices");
    hashmap_set(search_params, "semantic", "true");
    hashmap_set(search_params, "data_types", "tasks,memories,ideas");
    hashmap_set(search_params, "limit", "5");
    
    ToolResult* search_result = search_tool->execute(search_tool, search_params);
    if (search_result && search_result->success) {
        printf("✅ %s\n\n", search_result->message);
    }
    tool_result_free(search_result);
    hashmap_free(search_params);
    
    // Cleanup
    task_tool->destroy(task_tool);
    memory_tool->destroy(memory_tool);
    idea_tool->destroy(idea_tool);
    checklist_tool->destroy(checklist_tool);
    search_tool->destroy(search_tool);
    shared_todozi_free(todozi);
    
    printf("=== Workflow completed successfully! ===\n");
}

int main() {
    run_project_planning_workflow();
    return 0;
}
