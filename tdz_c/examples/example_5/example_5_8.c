#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Include all the structures and functions from memory.c
// (In a real project, these would be in header files)

// Forward declarations and all the code from memory.c would be here
// For brevity, we're assuming they're available

void example_5_complete_workflow() {
    printf("\n=== Example 5: Complete Memory Management Workflow ===\n");
    
    // 1. Create memory manager
    MemoryManager* manager = memory_manager_create();
    if (!manager) {
        printf("Failed to create memory manager\n");
        return;
    }
    printf("✓ Memory manager created\n");

    // 2. Create several memories using parse_memory_format
    const char* memory_texts[] = {
        "<memory>standard; Meeting with client; Discussed project scope; Need to define requirements; high; short; client,meeting,planning</memory>",
        "<memory>secret; Password change; Updated root password; Security requirement; critical; long; security,password,admin</memory>",
        "<memory>happy; Team lunch; Great team bonding; Improved morale; medium; short; team,morale,lunch</memory>",
        "<memory>human; John's birthday; Remember to send wishes; Personal relationship; medium; long; personal,john,birthday</memory>",
        "<memory>frustrated; Server downtime; Affected user experience; Need better monitoring; high; short; server,downtime,monitoring</memory>"
    };
    
    Memory* memories[5];
    for (int i = 0; i < 5; i++) {
        TodoziError* error = parse_memory_format(memory_texts[i], "user_001", &memories[i]);
        if (error) {
            printf("Error parsing memory %d: %s\n", i, error->message);
            todozi_error_free(error);
            memory_manager_destroy(manager);
            return;
        }
        
        error = memory_manager_create_memory(manager, memories[i]);
        if (error) {
            printf("Error creating memory %d: %s\n", i, error->message);
            todozi_error_free(error);
            memory_manager_destroy(manager);
            return;
        }
    }
    printf("✓ Created 5 memories\n");

    // 3. Display all memories
    printf("\nAll Memories:\n");
    Vector* all_memories = memory_manager_get_all_memories(manager);
    for (size_t i = 0; i < vector_size(all_memories); i++) {
        Memory* mem = (Memory*)vector_get(all_memories, i);
        printf("  %s: %s (Importance: %d, Term: %d)\n", 
               mem->id, mem->moment, mem->importance, mem->term);
    }
    vector_destroy(all_memories, NULL);

    // 4. Search for memories
    printf("\nSearching for 'client':\n");
    Vector* search_results = memory_manager_search_memories(manager, "client");
    for (size_t i = 0; i < vector_size(search_results); i++) {
        Memory* mem = (Memory*)vector_get(search_results, i);
        printf("  Found: %s\n", mem->moment);
    }
    vector_destroy(search_results, NULL);

    // 5. Get memories by importance
    printf("\nHigh importance memories:\n");
    Vector* high_importance = memory_manager_get_memories_by_importance(manager, MEMORY_IMPORTANCE_HIGH);
    for (size_t i = 0; i < vector_size(high_importance); i++) {
        Memory* mem = (Memory*)vector_get(high_importance, i);
        printf("  %s\n", mem->moment);
    }
    vector_destroy(high_importance, NULL);

    // 6. Update a memory
    printf("\nUpdating memory...\n");
    MemoryUpdate* update = memory_update_create();
    memory_update_meaning(update, "Defined detailed project requirements");
    memory_update_importance(update, MEMORY_IMPORTANCE_CRITICAL);
    
    // Get the first memory's ID
    Memory* first_memory = memory_manager_get_memory(manager, memories[0]->id);
    if (first_memory) {
        TodoziError* error = memory_manager_update_memory(manager, first_memory->id, update);
        if (error) {
            printf("Error updating memory: %s\n", error->message);
            todozi_error_free(error);
        } else {
            printf("✓ Memory updated\n");
            printf("  New meaning: %s\n", first_memory->meaning);
            printf("  New importance: %d\n", first_memory->importance);
        }
    }
    memory_update_destroy(update);

    // 7. Get statistics
    printf("\nMemory Statistics:\n");
    MemoryStatistics* stats = memory_manager_get_memory_statistics(manager);
    if (stats) {
        printf("  Total memories: %zu\n", stats->total_memories);
        printf("  Short term: %.1f%%\n", memory_statistics_short_term_percentage(stats));
        printf("  Critical: %.1f%%\n", memory_statistics_critical_percentage(stats));
        printf("  Unique tags: %zu\n", stats->unique_tags);
        memory_statistics_destroy(stats);
    }

    // 8. Get tag statistics
    printf("\nTag Statistics:\n");
    HashMap* tag_stats = memory_manager_get_tag_statistics(manager);
    if (tag_stats) {
        // Simple iteration through buckets
        for (size_t i = 0; i < tag_stats->capacity; i++) {
            HashMapEntry* entry = tag_stats->buckets[i];
            while (entry) {
                printf("  %s: %d\n", entry->key, *(int*)entry->value);
                entry = entry->next;
            }
        }
        hashmap_destroy(tag_stats, free, free);
    }

    // 9. Get recent memories
    printf("\nMost recent memory:\n");
    Vector* recent = memory_manager_get_recent_memories(manager, 1);
    if (vector_size(recent) > 0) {
        Memory* mem = (Memory*)vector_get(recent, 0);
        printf("  %s\n", mem->moment);
    }
    vector_destroy(recent, NULL);

    // 10. Clean up
    memory_manager_destroy(manager);
    printf("\n✓ Workflow completed and resources cleaned up\n");
}

int main() {
    example_5_complete_workflow();
    return 0;
}
