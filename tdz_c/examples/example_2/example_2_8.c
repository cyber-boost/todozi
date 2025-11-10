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
    
    if (memory->tags && vector_size(memory->tags) > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < vector_size(memory->tags); i++) {
            printf("%s ", string_vector_get(memory->tags, i));
        }
        printf("\n");
    }
    printf("---\n");
}

void print_statistics(MemoryStatistics* stats) {
    if (!stats) return;
    
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
    printf("Standard Memories: %zu\n", stats->standard_memories);
    printf("========================\n");
}

int main() {
    // Create memory manager
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        printf("Failed to create memory manager\n");
        return 1;
    }
    
    printf("=== Memory Management System Demo ===\n\n");
    
    // Create sample memories
    Memory* memory1 = memory_create();
    memory1->user_id = strdup("user_001");
    memory1->moment = strdup("Team meeting about Q4 goals");
    memory1->meaning = strdup("Align team on quarterly objectives");
    memory1->reason = strdup("Ensure project success");
    memory1->importance = MEMORY_IMPORTANCE_HIGH;
    memory1->term = MEMORY_TERM_LONG;
    memory1->memory_type = MEMORY_TYPE_STANDARD;
    memory1->status = ITEM_STATUS_ACTIVE;
    string_vector_push(memory1->tags, "meeting");
    string_vector_push(memory1->tags, "planning");
    string_vector_push(memory1->tags, "Q4");
    
    Memory* memory2 = memory_create();
    memory2->user_id = strdup("user_001");
    memory2->moment = strdup("Forgot client's birthday");
    memory2->meaning = strdup("Damaged client relationship");
    memory2->reason = strdup("Personal connection lost");
    memory2->importance = MEMORY_IMPORTANCE_CRITICAL;
    memory2->term = MEMORY_TERM_LONG;
    memory2->memory_type = MEMORY_TYPE_EMOTIONAL;
    memory2->emotion = strdup("ashamed");
    memory2->status = ITEM_STATUS_ACTIVE;
    string_vector_push(memory2->tags, "relationship");
    string_vector_push(memory2->tags, "client");
    
    Memory* memory3 = memory_create();
    memory3->user_id = strdup("user_001");
    memory3->moment = strdup("Implemented new caching strategy");
    memory3->meaning = strdup("Reduced API response time by 40%");
    memory3->reason = strdup("Performance optimization");
    memory3->importance = MEMORY_IMPORTANCE_MEDIUM;
    memory3->term = MEMORY_TERM_LONG;
    memory3->memory_type = MEMORY_TYPE_STANDARD;
    memory3->status = ITEM_STATUS_ACTIVE;
    string_vector_push(memory3->tags, "optimization");
    string_vector_push(memory3->tags, "performance");
    string_vector_push(memory3->tags, "tech");
    
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
    
    printf("Created %zu memories\n\n", hashmap_size(manager->memories));
    
    // Retrieve and display all memories
    printf("=== All Memories ===\n");
    Vector* all_memories = memory_manager_get_all_memories(manager);
    for (size_t i = 0; i < vector_size(all_memories); i++) {
        Memory* mem = (Memory*)vector_get(all_memories, i);
        print_memory_summary(mem);
    }
    vector_destroy(all_memories, NULL);
    
    // Search for memories
    printf("=== Search Results for 'client' ===\n");
    Vector* search_results = memory_manager_search_memories(manager, "client");
    if (search_results) {
        for (size_t i = 0; i < vector_size(search_results); i++) {
            Memory* mem = (Memory*)vector_get(search_results, i);
            print_memory_summary(mem);
        }
        vector_destroy(search_results, NULL);
    }
    
    // Get memories by importance
    printf("=== High Importance Memories ===\n");
    Vector* high_importance = memory_manager_get_memories_by_importance(manager, MEMORY_IMPORTANCE_HIGH);
    for (size_t i = 0; i < vector_size(high_importance); i++) {
        Memory* mem = (Memory*)vector_get(high_importance, i);
        print_memory_summary(mem);
    }
    vector_destroy(high_importance, NULL);
    
    // Get memories by tag
    printf("=== Memories tagged 'performance' ===\n");
    Vector* perf_memories = memory_manager_get_memories_by_tag(manager, "performance");
    for (size_t i = 0; i < vector_size(perf_memories); i++) {
        Memory* mem = (Memory*)vector_get(perf_memories, i);
        print_memory_summary(mem);
    }
    vector_destroy(perf_memories, NULL);
    
    // Update a memory
    printf("=== Updating Memory ===\n");
    Memory* to_update = memory_manager_get_memory(manager, memory1->id);
    if (to_update) {
        MemoryUpdate* update = memory_update_create();
        memory_update_meaning(update, "Align team on quarterly objectives and KPIs");
        memory_update_importance(update, MEMORY_IMPORTANCE_CRITICAL);
        string_vector_push(update->tags, "meeting");
        string_vector_push(update->tags, "planning");
        string_vector_push(update->tags, "Q4");
        string_vector_push(update->tags, "KPI");
        
        error = memory_manager_update_memory(manager, memory1->id, update);
        if (error) {
            printf("Error updating memory: %s\n", error->message);
            todozi_error_free(error);
        } else {
            printf("Memory updated successfully\n");
            print_memory_summary(to_update);
        }
        memory_update_destroy(update);
    }
    
    // Get statistics
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    print_statistics(stats);
    memory_statistics_destroy(stats);
    
    // Get recent memories
    printf("=== Most Recent Memory ===\n");
    Vector* recent = memory_manager_get_recent_memories(manager, 1);
    if (recent && vector_size(recent) > 0) {
        Memory* mem = (Memory*)vector_get(recent, 0);
        print_memory_summary(mem);
    }
    vector_destroy(recent, NULL);
    
    // Cleanup
    memory_manager_destroy(manager);
    printf("Memory manager destroyed\n");
    
    return 0;
}
