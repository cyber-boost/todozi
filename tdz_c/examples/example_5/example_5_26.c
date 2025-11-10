#include <stdio.h>
#include <stdlib.h>
#include "todozi.h" // Assuming todozi.c functions are exposed through this header

int main() {
    // Complex chat message with multiple todozi elements
    const char* chat_message = 
        "Let's plan our project:\n"
        "<tz>Implement user authentication;2 weeks;high;Project Alpha;todo;assignee=ai;security,auth;task-001;Need secure login system;0</tz>\n"
        "Also remember this important insight:\n"
        "<mm>user frustration with login;Users abandon app after 3 failed attempts;Poor error messaging causes confusion;Improve error handling;high;short;standard;ux,login</mm>\n"
        "I have a great idea:\n"
        "<id>New onboarding flow;share;high;Improve first-time user experience;onboarding,ux</id>\n"
        "Feeling about this project:\n"
        "<fe>excited;8;Ready to build something amazing;project kickoff;motivation,team</fe>";
    
    const char* user_id = "user-123";
    ChatContent content;
    
    // Process the chat message
    TodoziError result = process_chat_message_extended(chat_message, user_id, &content);
    
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
        printf("  Assignee: %d\n", task->assignee_type);
        printf("  Tags: ");
        for (size_t j = 0; j < task->tags_count; j++) {
            printf("%s ", task->tags[j]);
        }
        printf("\n\n");
    }
    
    // Print extracted memories
    printf("=== Extracted Memories (%zu) ===\n", content.memories_count);
    for (size_t i = 0; i < content.memories_count; i++) {
        Memory* memory = &content.memories[i];
        printf("Memory %zu:\n", i+1);
        printf("  Moment: %s\n", memory->moment);
        printf("  Meaning: %s\n", memory->meaning);
        printf("  Reason: %s\n", memory->reason);
        printf("  Importance: %d\n", memory->importance);
        printf("  Term: %d\n", memory->term);
        printf("  Type: %d\n", memory->memory_type);
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
        printf("  Description: %s\n", idea->idea);
        printf("  Share Level: %d\n", idea->share);
        printf("  Importance: %d\n", idea->importance);
        printf("  Tags: ");
        for (size_t j = 0; j < idea->tags_count; j++) {
            printf("%s ", idea->tags[j]);
        }
        printf("\n\n");
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
    
    // Clean up
    free_chat_content(&content);
    
    return 0;
}
