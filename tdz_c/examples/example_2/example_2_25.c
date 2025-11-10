// example2.c - Semantic Search with Caching Example
#include "emb.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Helper function to create a cache entry
struct TodoziEmbeddingCache* create_cache_entry(const char* id, const char* text, const float* vector, size_t dim) {
    struct TodoziEmbeddingCache* entry = malloc(sizeof(struct TodoziEmbeddingCache));
    if (!entry) return NULL;
    
    entry->content_id = string_new(id);
    entry->text_content = string_new(text);
    entry->vector = vec_new_with_capacity(dim);
    entry->created_at = time(NULL);
    entry->ttl_seconds = 3600; // 1 hour
    entry->tags = vec_new();
    
    // Copy vector data (placeholder - in real implementation would be actual embeddings)
    for (size_t i = 0; i < dim; i++) {
        float* val = malloc(sizeof(float));
        *val = vector[i];
        vec_push(&entry->vector, val);
    }
    
    return entry;
}

// Helper function to print search results
void print_search_results(struct Vec* results) {
    printf("Found %zu results:\n", results->size);
    for (size_t i = 0; i < results->size; i++) {
        struct TodoziEmbeddingCache* entry = (struct TodoziEmbeddingCache*)vec_get(results, i);
        if (entry) {
            printf("  [%zu] %s: \"%s\"\n", i+1, entry->content_id.data, entry->text_content.data);
        }
    }
}

int main(void) {
    printf("Example 2: Semantic Search with Caching\n");
    printf("========================================\n\n");
    
    // 1. Initialize service with custom configuration
    struct TodoziEmbeddingConfig config = config_default();
    config.dimensions = 384;
    config.similarity_threshold = 0.5f;
    config.max_results = 10;
    
    struct TodoziEmbeddingTool* tool = tool_new(config);
    if (!tool) {
        printf("❌ Failed to create tool\n");
        return -1;
    }
    
    if (tool_initialize(tool) != EMB_SUCCESS) {
        printf("❌ Tool initialization failed\n");
        tool_free(tool);
        return -1;
    }
    
    printf("✅ Service initialized\n");
    
    // 2. Add sample documents to cache
    struct TodoziEmbeddingService* service = tool->service;
    
    // Sample document vectors (simplified - in practice these would be real embeddings)
    float doc1_vec[] = {0.8f, 0.1f, 0.2f, 0.3f};
    float doc2_vec[] = {0.1f, 0.9f, 0.1f, 0.2f};
    float doc3_vec[] = {0.2f, 0.1f, 0.8f, 0.4f};
    
    struct TodoziEmbeddingCache* doc1 = create_cache_entry(
        "doc1", "Machine learning algorithms for beginners", doc1_vec, 4);
    struct TodoziEmbeddingCache* doc2 = create_cache_entry(
        "doc2", "Advanced neural network architectures", doc2_vec, 4);
    struct TodoziEmbeddingCache* doc3 = create_cache_entry(
        "doc3", "Data preprocessing techniques in Python", doc3_vec, 4);
    
    // Insert into cache
    cache_insert(service->cache, "doc1", doc1);
    cache_insert(service->cache, "doc2", doc2);
    cache_insert(service->cache, "doc3", doc3);
    
    printf("✅ Added 3 documents to cache\n");
    
    // 3. Perform semantic search
    printf("\n🔍 Searching for 'deep learning models'...\n");
    struct Vec results = service_semantic_search(service, "deep learning models", NULL, 5);
    print_search_results(&results);
    vec_free(&results);
    
    // 4. Find similar tasks
    printf("\n🔍 Finding similar tasks to 'building neural networks'...\n");
    struct Vec similar = service_find_similar_tasks(service, "building neural networks", 5);
    print_search_results(&similar);
    vec_free(&similar);
    
    // 5. Show cache statistics
    printf("\n📊 Cache statistics:\n");
    printf("  Documents in cache: %zu\n", hashmap_size(service->cache));
    
    // 6. Cleanup expired entries
    size_t cleaned = service_cleanup_expired(service);
    printf("  Expired entries cleaned: %zu\n", cleaned);
    
    // 7. Test similarity functions
    printf("\n📐 Similarity calculations:\n");
    float query[] = {0.7f, 0.2f, 0.1f, 0.3f};
    float cosine_sim = cosine_similarity(query, doc1_vec, 4);
    float euclid_dist = euclidean_distance(query, doc1_vec, 4);
    float dot_prod = dot_product(query, doc1_vec, 4);
    
    printf("  Cosine similarity: %.3f\n", cosine_sim);
    printf("  Euclidean distance: %.3f\n", euclid_dist);
    printf("  Dot product: %.3f\n", dot_prod);
    
    // 8. Cleanup
    cache_entry_free(doc1);
    cache_entry_free(doc2);
    cache_entry_free(doc3);
    tool_free(tool);
    config_free(&config);
    
    printf("\n✅ Example completed successfully\n");
    return 0;
}
