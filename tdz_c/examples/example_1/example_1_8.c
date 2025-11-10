#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "memory.c"  // Include the provided implementation

void print_memory_summary(Memory* memory) {
    if (!memory) return;
    
    const char* importance_str[] = {"Low", "Medium", "High", "Critical"};
    const char* term_str[] = {"Short", "Long"};
    const char* type_str[] = {"Standard", "Secret", "Human", "Emotional", "Short", "Long"};
    
    printf("ID: %.8s...\n", memory->id);
    printf("Moment: %s\n", memory->moment);
    printf("Meaning: %s\n", memory->meaning);
    printf("Importance: %s\n", importance_str[memory->importance]);
    printf("Term: %s\n", term_str[memory->term]);
    printf("Type: %s\n", type_str[memory->memory_type]);
    
    if (memory->emotion) {
        printf("Emotion: %s\n", memory->emotion);
    }
    
    if (vector_size(memory->tags) > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < vector_size(memory->tags); i++) {
            printf("%s ", string_vector_get(memory->tags, i));
        }
        printf("\n");
    }
    printf("\n");
}

void print_statistics(MemoryStatistics* stats) {
    printf("=== Memory Statistics ===\n");
    printf("Total Memories: %zu\n", stats->total_memories);
    printf("Short Term: %.1f%% (%zu)\n", 
           memory_statistics_short_term_percentage(stats),
           stats->short_term_memories);
    printf("Long Term: %.1f%% (%zu)\n",
           memory_statistics_long_term_percentage(stats),
           stats->long_term_memories);
    printf("Critical: %.1f%% (%zu)\n",
           memory_statistics_critical_percentage(stats),
           stats->critical_memories);
    printf("Unique Tags: %zu\n", stats->unique_tags);
    printf("Secret Memories: %zu\n", stats->secret_memories);
    printf("Human Memories: %zu\n", stats->human_memories);
    printf("Emotional Memories: %zu\n", stats->emotional_memories);
    printf("Standard Memories: %zu\n\n", stats->standard_memories);
}

int main() {
    // Create memory manager
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        printf("Failed to create memory manager\n");
        return 1;
    }
    
    printf("=== Personal Knowledge Management System ===\n\n");
    
    // Create different types of memories
    Memory* memory1 = memory_create();
    memory1->user_id = strdup("user_001");
    memory1->moment = strdup("Completed project presentation");
    memory1->meaning = strdup("Successfully delivered key project updates");
    memory1->reason = strdup("Quarterly stakeholder meeting");
    memory1->importance = MEMORY_IMPORTANCE_HIGH;
    memory1->term = MEMORY_TERM_LONG;
    memory1->memory_type = MEMORY_TYPE_STANDARD;
    memory1->status = ITEM_STATUS_COMPLETED;
    string_vector_push(memory1->tags, "work");
    string_vector_push(memory1->tags, "presentation");
    string_vector_push(memory1->tags, "success");
    
    Memory* memory2 = memory_create();
    memory2->user_id = strdup("user_001");
    memory2->moment = strdup("Felt anxious before public speaking");
    memory2->meaning = strdup("Performance anxiety affects communication");
    memory2->reason = strdup("Personal growth challenge");
    memory2->importance = MEMORY_IMPORTANCE_MEDIUM;
    memory2->term = MEMORY_TERM_LONG;
    memory2->memory_type = MEMORY_TYPE_EMOTIONAL;
    memory2->emotion = strdup("anxious");
    memory2->status = ITEM_STATUS_ACTIVE;
    string_vector_push(memory2->tags, "emotion");
    string_vector_push(memory2->tags, "public-speaking");
    string_vector_push(memory2->tags, "growth");
    
    Memory* memory3 = memory_create();
    memory3->user_id = strdup("user_001");
    memory3->moment = strdup("Remembered childhood birthday party");
    memory3->meaning = strdup("Family traditions create lasting bonds");
    memory3->reason = strdup("Nostalgic reflection");
    memory3->importance = MEMORY_IMPORTANCE_LOW;
    memory3->term = MEMORY_TERM_LONG;
    memory3->memory_type = MEMORY_TYPE_HUMAN;
    memory3->status = ITEM_STATUS_ACTIVE;
    string_vector_push(memory3->tags, "family");
    string_vector_push(memory3->tags, "nostalgia");
    string_vector_push(memory3->tags, "tradition");
    
    // Add memories to manager
    TodoziError* error;
    error = memory_manager_create_memory(manager, memory1);
    if (error) {
        printf("Error creating memory1: %s\n", error->message);
        todozi_error_free(error);
    }
    
    error = memory_manager_create_memory(manager, memory2);
    if (error) {
        printf("Error creating memory2: %s\n", error->message);
        todozi_error_free(error);
    }
    
    error = memory_manager_create_memory(manager, memory3);
    if (error) {
        printf("Error creating memory3: %s\n", error->message);
        todozi_error_free(error);
    }
    
    printf("=== Created Memories ===\n");
    print_memory_summary(memory1);
    print_memory_summary(memory2);
    print_memory_summary(memory3);
    
    // Update a memory
    MemoryUpdate* update = memory_update_create();
    memory_update_importance(update, MEMORY_IMPORTANCE_CRITICAL);
    memory_update_reason(update, "Key career milestone");
    string_vector_push(update->tags, "career");
    string_vector_push(update->tags, "milestone");
    string_vector_push(update->tags, "success");
    
    error = memory_manager_update_memory(manager, memory1->id, update);
    if (error) {
        printf("Error updating memory: %s\n", error->message);
        todozi_error_free(error);
    }
    memory_update_destroy(update);
    
    printf("=== Updated Memory ===\n");
    Memory* updated_memory = memory_manager_get_memory(manager, memory1->id);
    print_memory_summary(updated_memory);
    
    // Search memories
    printf("=== Search Results for 'success' ===\n");
    Vector* search_results = memory_manager_search_memories(manager, "success");
    if (search_results) {
        for (size_t i = 0; i < vector_size(search_results); i++) {
            Memory* mem = (Memory*)vector_get(search_results, i);
            printf("- %s\n", mem->moment);
        }
        vector_destroy(search_results, NULL);
    }
    
    // Get memories by criteria
    printf("\n=== Critical Memories ===\n");
    Vector* critical = memory_manager_get_critical_memories(manager);
    if (critical) {
        for (size_t i = 0; i < vector_size(critical); i++) {
            Memory* mem = (Memory*)vector_get(critical, i);
            printf("- %s\n", mem->moment);
        }
        vector_destroy(critical, NULL);
    }
    
    printf("\n=== Emotional Memories ===\n");
    Vector* emotional = memory_manager_get_emotional_memories(manager, "anxious");
    if (emotional) {
        for (size_t i = 0; i < vector_size(emotional); i++) {
            Memory* mem = (Memory*)vector_get(emotional, i);
            printf("- %s (%s)\n", mem->moment, mem->emotion);
        }
        vector_destroy(emotional, NULL);
    }
    
    // Show statistics
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    if (stats) {
        print_statistics(stats);
        memory_statistics_destroy(stats);
    }
    
    // Show all tags
    printf("=== All Tags ===\n");
    Vector* all_tags = memory_manager_get_all_tags(manager);
    if (all_tags) {
        for (size_t i = 0; i < vector_size(all_tags); i++) {
            printf("%s ", (char*)vector_get(all_tags, i));
        }
        printf("\n\n");
        vector_destroy(all_tags, free);
    }
    
    // Show tag statistics
    printf("=== Tag Usage Statistics ===\n");
    HashMap* tag_stats = memory_manager_get_tag_statistics(manager);
    if (tag_stats) {
        for (size_t i = 0; i < tag_stats->capacity; i++) {
            HashMapEntry* entry = tag_stats->buckets[i];
            while (entry) {
                printf("%s: %d\n", entry->key, *(int*)entry->value);
                entry = entry->next;
            }
        }
        hashmap_destroy(tag_stats, free, free);
    }
    
    // Cleanup
    memory_manager_destroy(manager);
    return 0;
}
