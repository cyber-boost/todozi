#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "todozi.c"  // Include the main implementation

int main() {
    // Example chat message with multiple Todozi elements
    const char* chat_message = 
        "Let's plan our project:\n"
        "<todozi>Implement user authentication;2 weeks;high;Project Alpha;todo;assignee=ai;security,backend;task-001;Need to follow OAuth2 standards;0</todozi>\n"
        "<memory>excited;Today's demo went well;Team collaborated effectively;Positive feedback from stakeholders;high;short;team,success</memory>\n"
        "<idea>Integrate voice commands;share;high;accessibility,innovation</idea>\n"
        "<error>Database connection timeout;Failed to connect to PostgreSQL;high;database;user_service;During peak hours;critical,infrastructure</error>\n"
        "<train>example;How to authenticate a user?;Use OAuth2 with JWT tokens;User management service</train>\n"
        "<feel>motivated;8;Made good progress on the project;Development milestone</feel>";

    ChatContent content;
    TodoziError result = process_chat_message_extended(chat_message, "user123", &content);
    
    if (result != TODOZI_SUCCESS) {
        printf("Error processing message: %d\n", result);
        return 1;
    }

    // Print extracted tasks
    printf("=== Extracted Tasks (%zu) ===\n", content.tasks_count);
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
        printf("  Tags: ");
        for (size_t j = 0; j < task->tags_count; j++) {
            printf("%s ", task->tags[j]);
        }
        printf("\n  Dependencies: ");
        for (size_t j = 0; j < task->dependencies_count; j++) {
            printf("%s ", task->dependencies[j]);
        }
        printf("\n  Context: %s\n", task->context_notes ? task->context_notes : "None");
        if (task->has_progress) {
            printf("  Progress: %d%%\n", task->progress);
        }
        printf("\n");
    }

    // Print extracted memories
    printf("=== Extracted Memories (%zu) ===\n", content.memories_count);
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
            printf("%s ", memory->tags[j]);
        }
        printf("\n\n");
    }

    // Print extracted ideas
    printf("=== Extracted Ideas (%zu) ===\n", content.ideas_count);
    for (size_t i = 0; i < content.ideas_count; i++) {
        Idea* idea = &content.ideas[i];
        printf("Idea %zu:\n", i+1);
        printf("  Content: %s\n", idea->idea);
        printf("  Share Level: %d\n", idea->share);
        printf("  Importance: %d\n", idea->importance);
        printf("  Tags: ");
        for (size_t j = 0; j < idea->tags_count; j++) {
            printf("%s ", idea->tags[j]);
        }
        printf("\n\n");
    }

    // Print extracted errors
    printf("=== Extracted Errors (%zu) ===\n", content.errors_count);
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
            printf("%s ", error->tags[j]);
        }
        printf("\n\n");
    }

    // Print extracted training data
    printf("=== Extracted Training Data (%zu) ===\n", content.training_data_count);
    for (size_t i = 0; i < content.training_data_count; i++) {
        TrainingData* data = &content.training_data[i];
        printf("Training Data %zu:\n", i+1);
        printf("  Type: %d\n", data->data_type);
        printf("  Prompt: %s\n", data->prompt);
        printf("  Completion: %s\n", data->completion);
        printf("  Context: %s\n", data->context ? data->context : "None");
        printf("  Tags: ");
        for (size_t j = 0; j < data->tags_count; j++) {
            printf("%s ", data->tags[j]);
        }
        if (data->has_quality_score) {
            printf("\n  Quality Score: %.2f", data->quality_score);
        }
        printf("\n  Source: %s\n\n", data->source);
    }

    // Print extracted feelings
    printf("=== Extracted Feelings (%zu) ===\n", content.feelings_count);
    for (size_t i = 0; i < content.feelings_count; i++) {
        Feeling* feeling = &content.feelings[i];
        printf("Feeling %zu:\n", i+1);
        printf("  Emotion: %s\n", feeling->emotion);
        printf("  Intensity: %d\n", feeling->intensity);
        printf("  Description: %s\n", feeling->description);
        printf("  Context: %s\n", feeling->context);
        printf("  Tags: ");
        for (size_t j = 0; j < feeling->tags_count; j++) {
            printf("%s ", feeling->tags[j]);
        }
        printf("\n\n");
    }

    // Clean up allocated memory
    free_chat_content(&content);
    
    return 0;
}
