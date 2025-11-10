// example4.c - Advanced Memory Management Usage
#include "memory.c"  // Include the main implementation

void demonstrate_advanced_features() {
    printf("\n=== Example 4: Advanced Memory Management ===\n");
    
    // Create memory manager
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        printf("Failed to create manager\n");
        return;
    }
    
    // Create sample memories using parse_memory_format
    const char* memories[] = {
        "<memory>standard; Meeting with team; Discussed project timeline; Need to finalize deliverables; high; short; meeting,timeline,project</memory>",
        "<memory>secret; Private conversation with client; Discussed budget concerns; Client hesitant about costs; critical; long; client,budget,confidential</memory>",
        "<memory>happy; Finished major project milestone; Team worked hard; Celebrated with pizza party; medium; long; celebration,team,milestone</memory>",
        "<memory>frustrated; Debugging session; Spent 4 hours on segfault; Lack of proper documentation; high; short; debugging,frustration,technical</memory>",
        "<memory>human; Coffee chat with Sarah; Learned about her new pet; Building team rapport; low; long; personal,team,relationship</memory>"
    };
    
    // Add memories to manager
    for (int i = 0; i < 5; i++) {
        Memory* memory;
        TodoziError* error = parse_memory_format(memories[i], "user_001", &memory);
        if (error) {
            printf("Parse error: %s\n", error->message);
            todozi_error_free(error);
            continue;
        }
        
        error = memory_manager_create_memory(manager, memory);
        if (error) {
            printf("Creation error: %s\n", error->message);
            todozi_error_free(error);
            memory_destroy(memory);
        } else {
            printf("Added memory: %s\n", memory->moment);
        }
    }
    
    // Demonstrate search functionality
    printf("\n--- Search Results for 'project' ---\n");
    Vector* search_results = memory_manager_search_memories(manager, "project");
    if (search_results) {
        for (size_t i = 0; i < vector_size(search_results); i++) {
            Memory* mem = (Memory*)vector_get(search_results, i);
            printf("Found: %s\n", mem->moment);
        }
        vector_destroy(search_results, NULL);
    }
    
    // Demonstrate filtering by importance
    printf("\n--- High Importance Memories ---\n");
    Vector* high_priority = memory_manager_get_memories_by_importance(manager, MEMORY_IMPORTANCE_HIGH);
    if (high_priority) {
        for (size_t i = 0; i < vector_size(high_priority); i++) {
            Memory* mem = (Memory*)vector_get(high_priority, i);
            printf("High priority: %s\n", mem->moment);
        }
        vector_destroy(high_priority, NULL);
    }
    
    // Demonstrate filtering by emotion
    printf("\n--- Emotional Memories (Happy) ---\n");
    Vector* happy_memories = memory_manager_get_emotional_memories(manager, "happy");
    if (happy_memories) {
        for (size_t i = 0; i < vector_size(happy_memories); i++) {
            Memory* mem = (Memory*)vector_get(happy_memories, i);
            printf("Happy memory: %s\n", mem->moment);
        }
        vector_destroy(happy_memories, NULL);
    }
    
    // Demonstrate filtering by tag
    printf("\n--- Memories tagged 'client' ---\n");
    Vector* client_memories = memory_manager_get_memories_by_tag(manager, "client");
    if (client_memories) {
        for (size_t i = 0; i < vector_size(client_memories); i++) {
            Memory* mem = (Memory*)vector_get(client_memories, i);
            printf("Client-related: %s\n", mem->moment);
        }
        vector_destroy(client_memories, NULL);
    }
    
    // Demonstrate statistics
    printf("\n--- Memory Statistics ---\n");
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    if (stats) {
        printf("Total memories: %zu\n", stats->total_memories);
        printf("Short-term: %.1f%%\n", memory_statistics_short_term_percentage(stats));
        printf("Long-term: %.1f%%\n", memory_statistics_long_term_percentage(stats));
        printf("Critical: %.1f%%\n", memory_statistics_critical_percentage(stats));
        printf("Unique tags: %zu\n", stats->unique_tags);
        printf("Emotional memories: %zu\n", stats->emotional_memories);
        memory_statistics_destroy(stats);
    }
    
    // Demonstrate recent memories
    printf("\n--- 3 Most Recent Memories ---\n");
    Vector* recent = memory_manager_get_recent_memories(manager, 3);
    if (recent) {
        for (size_t i = 0; i < vector_size(recent); i++) {
            Memory* mem = (Memory*)vector_get(recent, i);
            printf("%zu. %s (Created: %s)", i+1, mem->moment, ctime(&mem->created_at));
        }
        vector_destroy(recent, NULL);
    }
    
    // Demonstrate tag statistics
    printf("\n--- Tag Usage Statistics ---\n");
    HashMap* tag_stats = memory_manager_get_tag_statistics(manager);
    if (tag_stats) {
        for (size_t i = 0; i < tag_stats->capacity; i++) {
            HashMapEntry* entry = tag_stats->buckets[i];
            while (entry) {
                printf("%s: %d occurrences\n", (char*)entry->key, *(int*)entry->value);
                entry = entry->next;
            }
        }
        hashmap_destroy(tag_stats, free, free);
    }
    
    // Demonstrate memory update
    printf("\n--- Updating First Memory ---\n");
    Vector* all_memories = memory_manager_get_all_memories(manager);
    if (vector_size(all_memories) > 0) {
        Memory* first_memory = (Memory*)vector_get(all_memories, 0);
        printf("Before update: %s\n", first_memory->meaning);
        
        MemoryUpdate* update = memory_update_create();
        memory_update_meaning(update, "Project timeline discussion with key stakeholders");
        memory_update_importance(update, MEMORY_IMPORTANCE_CRITICAL);
        
        TodoziError* error = memory_manager_update_memory(manager, first_memory->id, update);
        if (error) {
            printf("Update error: %s\n", error->message);
            todozi_error_free(error);
        } else {
            printf("After update: %s (importance: %d)\n", 
                   first_memory->meaning, first_memory->importance);
        }
        memory_update_destroy(update);
    }
    vector_destroy(all_memories, NULL);
    
    // Cleanup
    memory_manager_destroy(manager);
    printf("\nAdvanced features demonstration completed.\n");
}

int main() {
    demonstrate_advanced_features();
    return 0;
}
