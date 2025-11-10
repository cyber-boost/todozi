#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "todozi.c" // Assuming all code is in this file

int main() {
    // Sample complex chat message with multiple todozi elements
    const char* chat_message = 
        "Let's plan our project:\n"
        "<todozi>Implement user authentication;2 weeks;high;ProjectA;todo;assignee=ai;security,backend;task-001;Need secure login;75</todozi>\n"
        "Also remember this important detail:\n"
        "<memory>excited;During team meeting;yesterday's brainstorming session was productive;It led to new feature ideas;high;long;meeting,productivity</memory>\n"
        "Here's a great idea I had:\n"
        "<idea>Create voice commands for navigation;share;high;ux,accessibility</idea>\n"
        "Don't forget to handle this error:\n"
        "<error>Database connection timeout;Failed to connect to PostgreSQL after 30s;high;database;auth_service;During peak hours;network,timeout</error>\n"
        "And capture how I'm feeling:\n"
        "<feel>motivated;8;Making good progress on the project;Working from home;productivity,happiness</feel>\n"
        "Let's train the model with this example:\n"
        "<train>example;How do I create a new user?;Call the /api/users POST endpoint with email and password;User management;api,users</train>";

    const char* user_id = "user-123";
    ChatContent content;
    
    // Process the chat message
    TodoziError result = process_chat_message_extended(chat_message, user_id, &content);
    
    if (result != TODOZI_SUCCESS) {
        printf("Failed to process chat message: %d\n", result);
        return 1;
    }
    
    // Print extracted tasks
    printf("=== TASKS (%zu) ===\n", content.tasks_count);
    for (size_t i = 0; i < content.tasks_count; i++) {
        Task* task = &content.tasks[i];
        printf("Task %zu:\n", i+1);
        printf("  Action: %s\n", task->action);
        printf("  Time: %s\n", task->time);
        printf("  Priority: %d\n", task->priority);
        printf("  Project: %s\n", task->parent_project);
        printf("  Status: %d\n", task->status);
        printf("  Assignee Type: %d\n", task->assignee_type);
        if (task->assignee_name) {
            printf("  Assignee Name: %s\n", task->assignee_name);
        }
        printf("  Progress: %d%%\n", task->has_progress ? task->progress : 0);
        printf("  Tags: ");
        for (size_t j = 0; j < task->tags_count; j++) {
            printf("%s%s", task->tags[j], (j < task->tags_count - 1) ? ", " : "");
        }
        printf("\n  Dependencies: ");
        for (size_t j = 0; j < task->dependencies_count; j++) {
            printf("%s%s", task->dependencies[j], (j < task->dependencies_count - 1) ? ", " : "");
        }
        printf("\n  Notes: %s\n\n", task->context_notes ? task->context_notes : "None");
    }
    
    // Print extracted memories
    printf("=== MEMORIES (%zu) ===\n", content.memories_count);
    for (size_t i = 0; i < content.memories_count; i++) {
        Memory* memory = &content.memories[i];
        printf("Memory %zu:\n", i+1);
        printf("  Type: %d\n", memory->memory_type);
        if (memory->emotion) {
            printf("  Emotion: %s\n", memory->emotion);
        }
        printf("  Moment: %s\n", memory->moment);
        printf("  Meaning: %s\n", memory->meaning);
        printf("  Reason: %s\n", memory->reason);
        printf("  Importance: %d\n", memory->importance);
        printf("  Term: %d\n", memory->term);
        printf("  Tags: ");
        for (size_t j = 0; j < memory->tags_count; j++) {
            printf("%s%s", memory->tags[j], (j < memory->tags_count - 1) ? ", " : "");
        }
        printf("\n\n");
    }
    
    // Print extracted ideas
    printf("=== IDEAS (%zu) ===\n", content.ideas_count);
    for (size_t i = 0; i < content.ideas_count; i++) {
        Idea* idea = &content.ideas[i];
        printf("Idea %zu:\n", i+1);
        printf("  Description: %s\n", idea->idea);
        printf("  Share Level: %d\n", idea->share);
        printf("  Importance: %d\n", idea->importance);
        printf("  Tags: ");
        for (size_t j = 0; j < idea->tags_count; j++) {
            printf("%s%s", idea->tags[j], (j < idea->tags_count - 1) ? ", " : "");
        }
        printf("\n\n");
    }
    
    // Print extracted errors
    printf("=== ERRORS (%zu) ===\n", content.errors_count);
    for (size_t i = 0; i < content.errors_count; i++) {
        Error* error = &content.errors[i];
        printf("Error %zu:\n", i+1);
        printf("  Title: %s\n", error->title);
        printf("  Description: %s\n", error->description);
        printf("  Severity: %d\n", error->severity);
        printf("  Category: %d\n", error->category);
        printf("  Source: %s\n", error->source);
        printf("  Context: %s\n", error->context ? error->context : "None");
        printf("  Tags: ");
        for (size_t j = 0; j < error->tags_count; j++) {
            printf("%s%s", error->tags[j], (j < error->tags_count - 1) ? ", " : "");
        }
        printf("\n\n");
    }
    
    // Print extracted feelings
    printf("=== FEELINGS (%zu) ===\n", content.feelings_count);
    for (size_t i = 0; i < content.feelings_count; i++) {
        Feeling* feeling = &content.feelings[i];
        printf("Feeling %zu:\n", i+1);
        printf("  Emotion: %s\n", feeling->emotion);
        printf("  Intensity: %d/10\n", feeling->intensity);
        printf("  Description: %s\n", feeling->description);
        printf("  Context: %s\n", feeling->context);
        printf("  Tags: ");
        for (size_t j = 0; j < feeling->tags_count; j++) {
            printf("%s%s", feeling->tags[j], (j < feeling->tags_count - 1) ? ", " : "");
        }
        printf("\n\n");
    }
    
    // Print extracted training data
    printf("=== TRAINING DATA (%zu) ===\n", content.training_data_count);
    for (size_t i = 0; i < content.training_data_count; i++) {
        TrainingData* data = &content.training_data[i];
        printf("Training Data %zu:\n", i+1);
        printf("  Type: %d\n", data->data_type);
        printf("  Prompt: %s\n", data->prompt);
        printf("  Completion: %s\n", data->completion);
        printf("  Context: %s\n", data->context ? data->context : "None");
        printf("  Quality Score: %.2f\n", data->has_quality_score ? data->quality_score : 0.0);
        printf("  Source: %s\n", data->source);
        printf("  Tags: ");
        for (size_t j = 0; j < data->tags_count; j++) {
            printf("%s%s", data->tags[j], (j < data->tags_count - 1) ? ", " : "");
        }
        printf("\n\n");
    }
    
    // Clean up
    free_chat_content(&content);
    
    return 0;
}
