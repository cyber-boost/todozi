#include <stdio.h>
#include <stdlib.h>
#include "todozi.c"  // Include the main Todozi implementation

int main() {
    // Example 4: Processing a complex chat message with multiple structured elements
    const char* user_id = "user_12345";
    const char* complex_message = 
        "Let's plan our project:\n"
        "<todozi>Create project documentation;2 weeks;high;ProjectX;todo;assignee=ai;docs,planning;task_001;Need comprehensive docs;0</todozi>\n"
        "Also, I had a realization:\n"
        "<mm>excited;During team meeting;Found innovative solution;Team collaboration sparked creativity;high;long;inspiration,team</mm>\n"
        "Here's a great idea:\n"
        "<id>Implement voice commands;share;high;ux,innovation,accessibility</id>\n"
        "We should track this error:\n"
        "<er>Database connection timeout;Failed to connect to PostgreSQL;high;database;db_module;Production environment;critical,backend</er>\n"
        "This is how we should handle user auth:\n"
        "<tn>qa;How to authenticate a user?;Provide JWT token after validating credentials;Auth flow example;security,api</tn>\n"
        "Feeling about the project:\n"
        "<fe>motivated;8;Excited about the new features;Project kickoff;positive,energy</fe>";

    ChatContent content;
    TodoziError result = process_chat_message_extended(complex_message, user_id, &content);

    if (result == TODOZI_SUCCESS) {
        printf("=== Todozi Processing Results ===\n\n");

        // Display tasks
        printf("Tasks (%zu found):\n", content.tasks_count);
        for (size_t i = 0; i < content.tasks_count; i++) {
            Task* task = &content.tasks[i];
            printf("  [%s] %s\n", task->id, task->action);
            printf("    Priority: %d, Project: %s, Status: %d\n", 
                   task->priority, task->parent_project, task->status);
            if (task->assignee_type == ASSIGNEE_AI) {
                printf("    Assignee: AI\n");
            }
            printf("    Tags: ");
            for (size_t j = 0; j < task->tags_count; j++) {
                printf("%s ", task->tags[j]);
            }
            printf("\n\n");
        }

        // Display memories
        printf("Memories (%zu found):\n", content.memories_count);
        for (size_t i = 0; i < content.memories_count; i++) {
            Memory* memory = &content.memories[i];
            printf("  [%s] %s\n", memory->id, memory->emotion);
            printf("    Moment: %s\n", memory->moment);
            printf("    Meaning: %s\n", memory->meaning);
            printf("    Importance: %d, Term: %d\n", memory->importance, memory->term);
            printf("    Tags: ");
            for (size_t j = 0; j < memory->tags_count; j++) {
                printf("%s ", memory->tags[j]);
            }
            printf("\n\n");
        }

        // Display ideas
        printf("Ideas (%zu found):\n", content.ideas_count);
        for (size_t i = 0; i < content.ideas_count; i++) {
            Idea* idea = &content.ideas[i];
            printf("  [%s] %s\n", idea->id, idea->idea);
            printf("    Share Level: %d, Importance: %d\n", idea->share, idea->importance);
            printf("    Tags: ");
            for (size_t j = 0; j < idea->tags_count; j++) {
                printf("%s ", idea->tags[j]);
            }
            printf("\n\n");
        }

        // Display errors
        printf("Errors (%zu found):\n", content.errors_count);
        for (size_t i = 0; i < content.errors_count; i++) {
            Error* error = &content.errors[i];
            printf("  [%s] %s\n", error->id, error->title);
            printf("    Description: %s\n", error->description);
            printf("    Severity: %d, Category: %d\n", error->severity, error->category);
            printf("    Source: %s\n", error->source);
            printf("    Tags: ");
            for (size_t j = 0; j < error->tags_count; j++) {
                printf("%s ", error->tags[j]);
            }
            printf("\n\n");
        }

        // Display training data
        printf("Training Data (%zu found):\n", content.training_data_count);
        for (size_t i = 0; i < content.training_data_count; i++) {
            TrainingData* data = &content.training_data[i];
            printf("  [%s] Type: %d\n", data->id, data->data_type);
            printf("    Prompt: %s\n", data->prompt);
            printf("    Completion: %s\n", data->completion);
            if (data->has_quality_score) {
                printf("    Quality Score: %.2f\n", data->quality_score);
            }
            printf("    Tags: ");
            for (size_t j = 0; j < data->tags_count; j++) {
                printf("%s ", data->tags[j]);
            }
            printf("\n\n");
        }

        // Display feelings
        printf("Feelings (%zu found):\n", content.feelings_count);
        for (size_t i = 0; i < content.feelings_count; i++) {
            Feeling* feeling = &content.feelings[i];
            printf("  [%s] %s (Intensity: %d)\n", feeling->id, feeling->emotion, feeling->intensity);
            printf("    Description: %s\n", feeling->description);
            printf("    Context: %s\n", feeling->context);
            printf("    Tags: ");
            for (size_t j = 0; j < feeling->tags_count; j++) {
                printf("%s ", feeling->tags[j]);
            }
            printf("\n\n");
        }

        // Clean up
        free_chat_content(&content);
    } else {
        printf("Error processing message: %d\n", result);
    }

    return 0;
}
