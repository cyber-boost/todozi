// example3.c - Semantic Search with Caching
#include "emb.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void example_semantic_search_with_caching() {
    printf("=== Example 3: Semantic Search with Caching ===\n\n");

    // 1. Create default configuration
    struct TodoziEmbeddingConfig config = config_default();
    config.similarity_threshold = 0.5f;  // Lower threshold for demo
    config.max_results = 10;

    // 2. Initialize embedding tool
    struct TodoziEmbeddingTool* tool = tool_new(config);
    if (!tool) {
        printf("❌ Failed to create embedding tool\n");
        return;
    }

    if (tool_initialize(tool) != EMB_SUCCESS) {
        printf("❌ Failed to initialize tool\n");
        tool_free(tool);
        return;
    }

    struct TodoziEmbeddingService* service = tool->service;

    // 3. Simulate adding some content to cache
    // In a real app, these would come from your database or files
    const char* documents[] = {
        "Learn C programming language fundamentals",
        "Build machine learning models with Python",
        "Master data structures and algorithms",
        "Develop web applications using JavaScript",
        "Understand database design principles"
    };
    size_t doc_count = sizeof(documents) / sizeof(documents[0]);

    printf("Adding %zu documents to cache...\n", doc_count);

    for (size_t i = 0; i < doc_count; i++) {
        // Create cache entry (simplified version)
        struct TodoziEmbeddingCache* entry = calloc(1, sizeof(struct TodoziEmbeddingCache));
        if (!entry) continue;

        char key[64];
        snprintf(key, sizeof(key), "doc_%zu", i);

        entry->content_id = string_new(key);
        entry->text_content = string_new(documents[i]);
        entry->created_at = time(NULL);
        entry->ttl_seconds = 3600; // 1 hour
        entry->tags = vec_new();

        // Add to cache
        cache_insert(service->cache, key, entry);
    }

    printf("✅ Added %zu documents to cache\n\n", doc_count);

    // 4. Perform semantic search
    const char* query = "programming languages";
    printf("Searching for: \"%s\"\n", query);

    struct Vec results = service_semantic_search(service, query, NULL, 5);

    if (results.size > 0) {
        printf("Found %zu similar documents:\n", results.size);
        for (size_t i = 0; i < results.size; i++) {
            // In real implementation, results would contain similarity scores
            printf("  [%zu] %s\n", i + 1, documents[i]);
        }
    } else {
        printf("No matching documents found\n");
    }

    vec_free(&results);

    // 5. Show cache statistics
    size_t cache_size = hashmap_size(service->cache);
    printf("\nCache contains %zu entries\n", cache_size);

    // 6. Cleanup expired entries
    size_t cleaned = service_cleanup_expired(service);
    printf("Cleaned %zu expired entries\n", cleaned);

    // 7. Cleanup
    tool_free(tool);
    config_free(&config);
    printf("\n✅ Example completed successfully\n");
}

int main(void) {
    example_semantic_search_with_caching();
    return 0;
}
