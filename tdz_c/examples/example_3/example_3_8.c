#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "memory.c"  // Include the main implementation

void example_emotional_memory_management() {
    printf("\n=== Example 3: Emotional Memory Management ===\n");
    
    // Create memory manager
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        printf("Failed to create memory manager\n");
        return;
    }
    
    // Create emotional memories
    Memory* happy_memory = memory_create();
    happy_memory->user_id = strdup("user_001");
    happy_memory->moment = strdup("Received promotion at work");
    happy_memory->meaning = strdup("Career advancement achieved");
    happy_memory->reason = strdup("Hard work and dedication recognized");
    happy_memory->importance = MEMORY_IMPORTANCE_HIGH;
    happy_memory->term = MEMORY_TERM_LONG;
    happy_memory->memory_type = MEMORY_TYPE_EMOTIONAL;
    happy_memory->emotion = strdup("happy");
    string_vector_push(happy_memory->tags, "career");
    string_vector_push(happy_memory->tags, "achievement");
    string_vector_push(happy_memory->tags, "work");
    
    Memory* sad_memory = memory_create();
    sad_memory->user_id = strdup("user_001");
    sad_memory->moment = strdup("Lost childhood pet");
    sad_memory->meaning = strdup("Experiencing grief and loss");
    sad_memory->reason = strdup("Natural part of life");
    sad_memory->importance = MEMORY_IMPORTANCE_MEDIUM;
    sad_memory->term = MEMORY_TERM_LONG;
    sad_memory->memory_type = MEMORY_TYPE_EMOTIONAL;
    sad_memory->emotion = strdup("sad");
    string_vector_push(sad_memory->tags, "grief");
    string_vector_push(sad_memory->tags, "pets");
    string_vector_push(sad_memory->tags, "loss");
    
    Memory* anxious_memory = memory_create();
    anxious_memory->user_id = strdup("user_001");
    anxious_memory->moment = strdup("Upcoming important presentation");
    anxious_memory->meaning = strdup("Feeling nervous about performance");
    anxious_memory->reason = strdup("Fear of judgment");
    anxious_memory->importance = MEMORY_IMPORTANCE_HIGH;
    anxious_memory->term = MEMORY_TERM_SHORT;
    anxious_memory->memory_type = MEMORY_TYPE_EMOTIONAL;
    anxious_memory->emotion = strdup("anxious");
    string_vector_push(anxious_memory->tags, "work");
    string_vector_push(anxious_memory->tags, "presentation");
    string_vector_push(anxious_memory->tags, "stress");
    
    // Add memories to manager
    TodoziError* error;
    error = memory_manager_create_memory(manager, happy_memory);
    if (error) {
        printf("Error creating happy memory: %s\n", error->message);
        todozi_error_free(error);
    }
    
    error = memory_manager_create_memory(manager, sad_memory);
    if (error) {
        printf("Error creating sad memory: %s\n", error->message);
        todozi_error_free(error);
    }
    
    error = memory_manager_create_memory(manager, anxious_memory);
    if (error) {
        printf("Error creating anxious memory: %s\n", error->message);
        todozi_error_free(error);
    }
    
    // Retrieve emotional memories by emotion
    printf("\n--- Happy Memories ---\n");
    Vector* happy_memories = memory_manager_get_emotional_memories(manager, "happy");
    if (happy_memories) {
        for (size_t i = 0; i < vector_size(happy_memories); i++) {
            Memory* mem = (Memory*)vector_get(happy_memories, i);
            printf("ID: %s\n", mem->id);
            printf("Moment: %s\n", mem->moment);
            printf("Emotion: %s\n", mem->emotion);
            printf("Tags: ");
            for (size_t j = 0; j < vector_size(mem->tags); j++) {
                printf("%s ", string_vector_get(mem->tags, j));
            }
            printf("\n\n");
        }
        vector_destroy(happy_memories, NULL);
    }
    
    printf("--- Anxious Memories ---\n");
    Vector* anxious_memories = memory_manager_get_emotional_memories(manager, "anxious");
    if (anxious_memories) {
        for (size_t i = 0; i < vector_size(anxious_memories); i++) {
            Memory* mem = (Memory*)vector_get(anxious_memories, i);
            printf("ID: %s\n", mem->id);
            printf("Moment: %s\n", mem->moment);
            printf("Emotion: %s\n", mem->emotion);
            printf("Tags: ");
            for (size_t j = 0; j < vector_size(mem->tags); j++) {
                printf("%s ", string_vector_get(mem->tags, j));
            }
            printf("\n\n");
        }
        vector_destroy(anxious_memories, NULL);
    }
    
    // Search memories by tag
    printf("--- Memories tagged 'work' ---\n");
    Vector* work_memories = memory_manager_get_memories_by_tag(manager, "work");
    if (work_memories) {
        for (size_t i = 0; i < vector_size(work_memories); i++) {
            Memory* mem = (Memory*)vector_get(work_memories, i);
            printf("Moment: %s (Emotion: %s)\n", 
                   mem->moment, 
                   mem->emotion ? mem->emotion : "none");
        }
        vector_destroy(work_memories, NULL);
    }
    
    // Get all emotional memories
    printf("\n--- All Emotional Memories ---\n");
    Vector* all_emotional = memory_manager_get_emotional_memories(manager, "happy"); // Start with one emotion
    Vector* temp = memory_manager_get_emotional_memories(manager, "sad");
    if (temp) {
        for (size_t i = 0; i < vector_size(temp); i++) {
            vector_push(all_emotional, vector_get(temp, i));
        }
        vector_destroy(temp, NULL);
    }
    temp = memory_manager_get_emotional_memories(manager, "anxious");
    if (temp) {
        for (size_t i = 0; i < vector_size(temp); i++) {
            vector_push(all_emotional, vector_get(temp, i));
        }
        vector_destroy(temp, NULL);
    }
    
    printf("Total emotional memories: %zu\n", vector_size(all_emotional));
    vector_destroy(all_emotional, NULL);
    
    // Get memory statistics
    printf("\n--- Memory Statistics ---\n");
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    if (stats) {
        printf("Total memories: %zu\n", stats->total_memories);
        printf("Emotional memories: %zu\n", stats->emotional_memories);
        printf("Short-term memories: %zu (%.1f%%)\n", 
               stats->short_term_memories, 
               memory_statistics_short_term_percentage(stats));
        printf("Long-term memories: %zu (%.1f%%)\n", 
               stats->long_term_memories, 
               memory_statistics_long_term_percentage(stats));
        printf("Unique tags: %zu\n", stats->unique_tags);
        memory_statistics_destroy(stats);
    }
    
    // Get all tags
    printf("\n--- All Tags ---\n");
    Vector* all_tags = memory_manager_get_all_tags(manager);
    if (all_tags) {
        for (size_t i = 0; i < vector_size(all_tags); i++) {
            printf("%s ", (char*)vector_get(all_tags, i));
        }
        printf("\n");
        vector_destroy(all_tags, free);
    }
    
    // Clean up
    memory_manager_destroy(manager);
    printf("\nEmotional memory management example completed.\n");
}

int main() {
    example_emotional_memory_management();
    return 0;
}
