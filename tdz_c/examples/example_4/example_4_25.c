// Example 4: Semantic Search with Caching
#include "emb.h"
#include <stdio.h>
#include <time.h>

void example_semantic_search_with_caching() {
    printf("=== Example 4: Semantic Search with Caching ===\n");
    
    // 1. Create service with custom configuration
    struct TodoziEmbeddingConfig config = config_default();
    config.similarity_threshold = 0.5f;
    config.max_results = 10;
    
    struct TodoziEmbeddingTool* tool = tool_new(config);
    if (!tool) {
        printf("Failed to create tool\n");
        return;
    }
    
    if (tool_initialize(tool) != EMB_SUCCESS) {
        printf("Failed to initialize tool\n");
        tool_free(tool);
        return;
    }
    
    struct TodoziEmbeddingService* service = tool->service;
    
    // 2. Simulate adding cached embeddings (normally done during indexing)
    {
        // Create sample cache entries
        struct TodoziEmbeddingCache* entry1 = malloc(sizeof(struct TodoziEmbeddingCache));
        entry1->content_id = string_new("task_001");
        entry1->text_content = string_new("Implement user authentication system");
        entry1->vector = vec_new_with_capacity(3);
        vec_push(&entry1->vector, &(float){0.8f});
        vec_push(&entry1->vector, &(float){0.1f});
        vec_push(&entry1->vector, &(float){0.2f});
        entry1->created_at = time(NULL);
        entry1->ttl_seconds = 3600;
        entry1->tags = vec_new();
        
        struct TodoziEmbeddingCache* entry2 = malloc(sizeof(struct TodoziEmbeddingCache));
        entry2->content_id = string_new("task_002");
        entry2->text_content = string_new("Design database schema for users");
        entry2->vector = vec_new_with_capacity(3);
        vec_push(&entry2->vector, &(float){0.7f});
        vec_push(&entry2->vector, &(float){0.2f});
        vec_push(&entry2->vector, &(float){0.1f});
        entry2->created_at = time(NULL);
        entry2->ttl_seconds = 3600;
        entry2->tags = vec_new();
        
        // Add to cache
        cache_insert(service->cache, "task_001", entry1);
        cache_insert(service->cache, "task_002", entry2);
    }
    
    // 3. Perform semantic search
    const char* query = "User login functionality";
    struct Vec content_types = vec_new();  // Empty for all types
    struct Vec results = service_semantic_search(service, query, &content_types, 5);
    
    printf("Search query: \"%s\"\n", query);
    printf("Found %zu results\n", results.size);
    
    // 4. Show cache statistics
    struct HashMap* stats = service_get_stats(service);
    printf("Cache size: %zu entries\n", hashmap_size(service->cache));
    
    // 5. Cleanup expired entries
    size_t cleaned = service_cleanup_expired(service);
    printf("Cleaned %zu expired entries\n", cleaned);
    
    // 6. Cleanup
    vec_free(&results);
    vec_free(&content_types);
    hashmap_free(stats);
    tool_free(tool);
    config_free(&config);
    
    printf("Example 4 completed successfully\n\n");
}

int main() {
    example_semantic_search_with_caching();
    return 0;
}
