#include <stdio.h>
#include "todozi.c"  // Include the main Todozi implementation

int main() {
    // Sample chat message with multiple Todozi items using shorthand tags
    const char* chat_message = 
        "Let's plan our project! <tz>Implement login system;2 weeks;high;Project Alpha;todo;assignee=ai</tz> "
        "Also <mm>happy;Team meeting;Great collaboration;Positive atmosphere;high;long</mm> "
        "Idea: <id>New dashboard design;team;high</id> "
        "Agent assignment: <tdz>agent-123;task-456;project-alpha</tdz> "
        "Error found: <er>Database connection;Failed to connect;high;database;backend;critical</er> "
        "Training: <tn>example;How to use Todozi;Create tagged items;Documentation</tn> "
        "Feeling: <fe>excited;8;Working on new features;Development</fe>";

    // Process the chat message
    ChatContent content;
    TodoziError result = process_chat_message_extended(chat_message, "user-001", &content);
    
    if (result != TODOZI_SUCCESS) {
        printf("Error processing message: %d\n", result);
        return 1;
    }

    // Print extracted tasks
    printf("Tasks (%zu):\n", content.tasks_count);
    for (size_t i = 0; i < content.tasks_count; i++) {
        Task* task = &content.tasks[i];
        printf("  [%s] %s (Priority: %d, Status: %d)\n", 
               task->id, task->action, task->priority, task->status);
    }

    // Print extracted memories
    printf("\nMemories (%zu):\n", content.memories_count);
    for (size_t i = 0; i < content.memories_count; i++) {
        Memory* memory = &content.memories[i];
        printf("  [%s] %s - %s (Type: %d)\n", 
               memory->id, memory->moment, memory->meaning, memory->memory_type);
    }

    // Print extracted ideas
    printf("\nIdeas (%zu):\n", content.ideas_count);
    for (size_t i = 0; i < content.ideas_count; i++) {
        Idea* idea = &content.ideas[i];
        printf("  [%s] %s (Share: %d, Importance: %d)\n", 
               idea->id, idea->idea, idea->share, idea->importance);
    }

    // Print agent assignments
    printf("\nAgent Assignments (%zu):\n", content.agent_assignments_count);
    for (size_t i = 0; i < content.agent_assignments_count; i++) {
        AgentAssignment* assignment = &content.agent_assignments[i];
        printf("  Agent %s assigned to task %s\n", 
               assignment->agent_id, assignment->task_id);
    }

    // Print errors
    printf("\nErrors (%zu):\n", content.errors_count);
    for (size_t i = 0; i < content.errors_count; i++) {
        Error* error = &content.errors[i];
        printf("  [%s] %s (Severity: %d)\n", 
               error->id, error->title, error->severity);
    }

    // Print training data
    printf("\nTraining Data (%zu):\n", content.training_data_count);
    for (size_t i = 0; i < content.training_data_count; i++) {
        TrainingData* data = &content.training_data[i];
        printf("  [%s] %s -> %s\n", 
               data->id, data->prompt, data->completion);
    }

    // Print feelings
    printf("\nFeelings (%zu):\n", content.feelings_count);
    for (size_t i = 0; i < content.feelings_count; i++) {
        Feeling* feeling = &content.feelings[i];
        printf("  [%s] %s (Intensity: %d) - %s\n", 
               feeling->id, feeling->emotion, feeling->intensity, feeling->description);
    }

    // Clean up allocated memory
    free_chat_content(&content);
    
    return 0;
}
