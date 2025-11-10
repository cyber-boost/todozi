#include "emb.h"
#include <math.h>
#include <assert.h>
#include <errno.h>

// ============================================================================
// UTILITY FUNCTIONS - String
// ============================================================================

struct String string_new(const char* str) {
    struct String s = {0};
    if (!str) {
        s.data = NULL;
        s.len = 0;
        s.capacity = 0;
        return s;
    }
    
    s.len = strlen(str);
    s.capacity = s.len + 1;
    s.data = malloc(s.capacity);
    if (!s.data) {
        s.len = 0;
        s.capacity = 0;
        return s;
    }
    memcpy(s.data, str, s.capacity);
    return s;
}

struct String string_new_with_capacity(size_t capacity) {
    struct String s = {0};
    if (capacity == 0) capacity = 16;
    s.capacity = capacity;
    s.data = malloc(s.capacity);
    if (!s.data) {
        s.len = 0;
        s.capacity = 0;
        return s;
    }
    s.data[0] = '\0';
    s.len = 0;
    return s;
}

void string_free(struct String* s) {
    if (s && s->data) {
        free(s->data);
        s->data = NULL;
        s->len = 0;
        s->capacity = 0;
    }
}

int string_append(struct String* s, const char* str) {
    if (!s || !str) return EMB_ERROR_NULL_POINTER;
    
    size_t str_len = strlen(str);
    size_t new_len = s->len + str_len;
    
    if (new_len + 1 > s->capacity) {
        size_t new_capacity = s->capacity * 2;
        if (new_capacity < new_len + 1) {
            new_capacity = new_len + 1;
        }
        char* new_data = realloc(s->data, new_capacity);
        if (!new_data) return EMB_ERROR_MEMORY;
        s->data = new_data;
        s->capacity = new_capacity;
    }
    
    memcpy(s->data + s->len, str, str_len + 1);
    s->len = new_len;
    return EMB_SUCCESS;
}

int string_resize(struct String* s, size_t new_capacity) {
    if (!s) return EMB_ERROR_NULL_POINTER;
    if (new_capacity < s->len + 1) new_capacity = s->len + 1;
    
    char* new_data = realloc(s->data, new_capacity);
    if (!new_data && new_capacity > 0) return EMB_ERROR_MEMORY;
    
    s->data = new_data;
    s->capacity = new_capacity;
    return EMB_SUCCESS;
}

// ============================================================================
// UTILITY FUNCTIONS - Vector
// ============================================================================

struct Vec vec_new(void) {
    struct Vec v = {0};
    v.size = 0;
    v.capacity = 4;
    v.data = malloc(v.capacity * sizeof(void*));
    if (!v.data) {
        v.capacity = 0;
    }
    return v;
}

struct Vec vec_new_with_capacity(size_t capacity) {
    struct Vec v = {0};
    v.size = 0;
    v.capacity = capacity > 0 ? capacity : 4;
    v.data = malloc(v.capacity * sizeof(void*));
    if (!v.data) {
        v.capacity = 0;
    }
    return v;
}

void vec_push(struct Vec* v, void* item) {
    if (!v) return;
    
    if (v->size >= v->capacity) {
        size_t new_capacity = v->capacity * 2;
        if (new_capacity == 0) new_capacity = 4;
        void** new_data = realloc(v->data, new_capacity * sizeof(void*));
        if (!new_data) return; // Out of memory
        v->data = new_data;
        v->capacity = new_capacity;
    }
    v->data[v->size++] = item;
}

void* vec_get(struct Vec* v, size_t index) {
    if (!v || index >= v->size) return NULL;
    return v->data[index];
}

void vec_free(struct Vec* v) {
    if (v) {
        if (v->data) {
            free(v->data);
        }
        v->data = NULL;
        v->size = 0;
        v->capacity = 0;
    }
}

void vec_clear(struct Vec* v) {
    if (v) {
        v->size = 0;
    }
}

// ============================================================================
// UTILITY FUNCTIONS - HashMap
// ============================================================================

// Simple hash function (djb2)
static size_t hash_string(const char* str) {
    if (!str) return 0;
    size_t hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c; // hash * 33 + c
    }
    return hash;
}

struct HashMap* hashmap_new(void) {
    return hashmap_new_with_capacity(16);
}

struct HashMap* hashmap_new_with_capacity(size_t capacity) {
    if (capacity == 0) capacity = 16;
    
    struct HashMap* map = malloc(sizeof(struct HashMap));
    if (!map) return NULL;
    
    map->capacity = capacity;
    map->size = 0;
    map->buckets = calloc(map->capacity, sizeof(struct HashMapEntry*));
    if (!map->buckets) {
        free(map);
        return NULL;
    }
    return map;
}

void hashmap_free(struct HashMap* map) {
    if (map) {
        for (size_t i = 0; i < map->capacity; i++) {
            struct HashMapEntry* entry = map->buckets[i];
            while (entry) {
                struct HashMapEntry* next = entry->next;
                free(entry->key);
                // Note: value is not freed here as it may be owned by caller
                free(entry);
                entry = next;
            }
        }
        free(map->buckets);
        free(map);
    }
}

int hashmap_insert(struct HashMap* map, const char* key, void* value) {
    if (!map || !key) return EMB_ERROR_NULL_POINTER;
    
    size_t hash = hash_string(key);
    size_t index = hash % map->capacity;
    
    // Check if key already exists
    struct HashMapEntry* entry = map->buckets[index];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            entry->value = value;
            return EMB_SUCCESS;
        }
        entry = entry->next;
    }
    
    // Create new entry
    entry = malloc(sizeof(struct HashMapEntry));
    if (!entry) return EMB_ERROR_MEMORY;
    
    entry->key = strdup(key);
    if (!entry->key) {
        free(entry);
        return EMB_ERROR_MEMORY;
    }
    entry->value = value;
    entry->next = map->buckets[index];
    map->buckets[index] = entry;
    map->size++;
    
    return EMB_SUCCESS;
}

void* hashmap_get(struct HashMap* map, const char* key) {
    if (!map || !key) return NULL;
    
    size_t hash = hash_string(key);
    size_t index = hash % map->capacity;
    
    struct HashMapEntry* entry = map->buckets[index];
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            return entry->value;
        }
        entry = entry->next;
    }
    return NULL;
}

int hashmap_remove(struct HashMap* map, const char* key) {
    if (!map || !key) return EMB_ERROR_NULL_POINTER;
    
    size_t hash = hash_string(key);
    size_t index = hash % map->capacity;
    
    struct HashMapEntry* entry = map->buckets[index];
    struct HashMapEntry* prev = NULL;
    
    while (entry) {
        if (strcmp(entry->key, key) == 0) {
            if (prev) {
                prev->next = entry->next;
            } else {
                map->buckets[index] = entry->next;
            }
            free(entry->key);
            free(entry);
            map->size--;
            return EMB_SUCCESS;
        }
        prev = entry;
        entry = entry->next;
    }
    return EMB_ERROR_NOT_FOUND;
}

size_t hashmap_size(struct HashMap* map) {
    return map ? map->size : 0;
}

// ============================================================================
// CONFIGURATION FUNCTIONS
// ============================================================================

struct TodoziEmbeddingConfig config_default(void) {
    struct TodoziEmbeddingConfig config = {0};
    config.model_name = string_new("sentence-transformers/all-MiniLM-L6-v2");
    config.dimensions = 384;
    config.similarity_threshold = 0.7f;
    config.max_results = 50;
    config.cache_ttl_seconds = 3600 * 24;
    config.enable_clustering = true;
    config.clustering_threshold = 0.8f;
    return config;
}

void config_free(struct TodoziEmbeddingConfig* config) {
    if (config) {
        string_free(&config->model_name);
    }
}

int config_validate(struct TodoziEmbeddingConfig* config) {
    if (!config) return EMB_ERROR_NULL_POINTER;
    if (!config->model_name.data) return EMB_ERROR_INVALID_INPUT;
    if (config->dimensions == 0) return EMB_ERROR_INVALID_INPUT;
    if (config->similarity_threshold < 0.0f || config->similarity_threshold > 1.0f) {
        return EMB_ERROR_INVALID_INPUT;
    }
    if (config->clustering_threshold < 0.0f || config->clustering_threshold > 1.0f) {
        return EMB_ERROR_INVALID_INPUT;
    }
    return EMB_SUCCESS;
}

// ============================================================================
// LRU CACHE IMPLEMENTATION
// ============================================================================

struct LRUEmbeddingCache* lru_cache_new(size_t max_memory_mb) {
    struct LRUEmbeddingCache* cache = malloc(sizeof(struct LRUEmbeddingCache));
    if (!cache) return NULL;
    
    cache->max_memory_mb = max_memory_mb;
    cache->cache = vec_new();
    cache->access_counts = hashmap_new();
    cache->current_memory_bytes = 0;
    return cache;
}

void lru_cache_free(struct LRUEmbeddingCache* cache) {
    if (cache) {
        // Free all cache entries
        for (size_t i = 0; i < cache->cache.size; i++) {
            struct TodoziEmbeddingCache* entry = (struct TodoziEmbeddingCache*)vec_get(&cache->cache, i);
            if (entry) {
                cache_entry_free(entry);
            }
        }
        vec_free(&cache->cache);
        hashmap_free(cache->access_counts);
        free(cache);
    }
}

int lru_cache_get(struct LRUEmbeddingCache* cache, const char* key, struct TodoziEmbeddingCache** value) {
    if (!cache || !key || !value) return EMB_ERROR_NULL_POINTER;
    
    // Find entry in cache
    for (size_t i = 0; i < cache->cache.size; i++) {
        struct TodoziEmbeddingCache* entry = (struct TodoziEmbeddingCache*)vec_get(&cache->cache, i);
        if (entry && strcmp(entry->content_id.data, key) == 0) {
            // Check if expired
            time_t now = time(NULL);
            if (entry->ttl_seconds > 0 && (now - entry->created_at) > (time_t)entry->ttl_seconds) {
                return EMB_ERROR_NOT_FOUND;
            }
            *value = entry;
            return EMB_SUCCESS;
        }
    }
    return EMB_ERROR_NOT_FOUND;
}

int lru_cache_put(struct LRUEmbeddingCache* cache, const char* key, struct TodoziEmbeddingCache* value) {
    if (!cache || !key || !value) return EMB_ERROR_NULL_POINTER;
    
    // Check memory limits
    size_t entry_size = sizeof(struct TodoziEmbeddingCache) + value->text_content.len;
    if (cache->current_memory_bytes + entry_size > cache->max_memory_mb * 1024 * 1024) {
        // Evict oldest entry (simple FIFO for now)
        if (cache->cache.size > 0) {
            struct TodoziEmbeddingCache* oldest = (struct TodoziEmbeddingCache*)vec_get(&cache->cache, 0);
            if (oldest) {
                cache_entry_free(oldest);
                // Remove from vector (simplified - would need proper vector remove)
            }
        }
    }
    
    vec_push(&cache->cache, value);
    cache->current_memory_bytes += entry_size;
    return EMB_SUCCESS;
}

// ============================================================================
// CACHE FUNCTIONS
// ============================================================================

struct TodoziEmbeddingCache* cache_get(struct HashMap* cache, const char* key) {
    if (!cache || !key) return NULL;
    return (struct TodoziEmbeddingCache*)hashmap_get(cache, key);
}

int cache_insert(struct HashMap* cache, const char* key, struct TodoziEmbeddingCache* value) {
    if (!cache || !key || !value) return EMB_ERROR_NULL_POINTER;
    return hashmap_insert(cache, key, value);
}

int cache_remove(struct HashMap* cache, const char* key) {
    if (!cache || !key) return EMB_ERROR_NULL_POINTER;
    return hashmap_remove(cache, key);
}

void cache_entry_free(struct TodoziEmbeddingCache* entry) {
    if (entry) {
        vec_free(&entry->vector);
        string_free(&entry->content_id);
        string_free(&entry->text_content);
        // Free tags
        for (size_t i = 0; i < entry->tags.size; i++) {
            struct String* tag = (struct String*)vec_get(&entry->tags, i);
            if (tag) {
                string_free(tag);
                free(tag);
            }
        }
        vec_free(&entry->tags);
        free(entry);
    }
}

// ============================================================================
// EMBEDDING MODEL FUNCTIONS
// ============================================================================

struct EmbeddingModel* embedding_model_load(const char* model_name) {
    if (!model_name) return NULL;
    
    struct EmbeddingModel* model = malloc(sizeof(struct EmbeddingModel));
    if (!model) return NULL;
    
    model->model = NULL;
    model->tokenizer = NULL;
    model->device = NULL;
    
    // Default dimensions based on common models
    if (strstr(model_name, "MiniLM-L6") != NULL) {
        model->dimensions = 384;
    } else if (strstr(model_name, "mpnet-base") != NULL) {
        model->dimensions = 768;
    } else if (strstr(model_name, "roberta-large") != NULL) {
        model->dimensions = 1024;
    } else {
        model->dimensions = 384; // Default
    }
    
    // TODO: In a real implementation, this would load the actual model
    // For now, this is a placeholder
    
    return model;
}

void embedding_model_free(struct EmbeddingModel* model) {
    if (model) {
        // TODO: Free model resources if allocated
        free(model);
    }
}

struct Vec embedding_model_encode(struct EmbeddingModel* model, const char** texts, size_t text_count) {
    struct Vec embeddings = vec_new();
    
    if (!model || !texts || text_count == 0) {
        return embeddings;
    }
    
    // TODO: In a real implementation, this would generate actual embeddings
    // For now, return empty vector as placeholder
    
    return embeddings;
}

int embedding_model_validate(struct EmbeddingModel* model) {
    if (!model) return EMB_ERROR_NULL_POINTER;
    if (model->dimensions == 0) return EMB_ERROR_INVALID_STATE;
    return EMB_SUCCESS;
}

// ============================================================================
// SIMILARITY FUNCTIONS
// ============================================================================

float cosine_similarity(const float* a, const float* b, size_t len) {
    if (!a || !b || len == 0) return 0.0f;
    
    float dot_product = 0.0f;
    float norm_a = 0.0f;
    float norm_b = 0.0f;
    
    for (size_t i = 0; i < len; i++) {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    
    norm_a = sqrtf(norm_a);
    norm_b = sqrtf(norm_b);
    
    if (norm_a == 0.0f || norm_b == 0.0f) {
        return 0.0f;
    }
    
    return dot_product / (norm_a * norm_b);
}

float euclidean_distance(const float* a, const float* b, size_t len) {
    if (!a || !b || len == 0) return 0.0f;
    
    float sum = 0.0f;
    for (size_t i = 0; i < len; i++) {
        float diff = a[i] - b[i];
        sum += diff * diff;
    }
    return sqrtf(sum);
}

float dot_product(const float* a, const float* b, size_t len) {
    if (!a || !b || len == 0) return 0.0f;
    
    float sum = 0.0f;
    for (size_t i = 0; i < len; i++) {
        sum += a[i] * b[i];
    }
    return sum;
}

// ============================================================================
// SERVICE FUNCTIONS
// ============================================================================

struct TodoziEmbeddingService* service_new(struct TodoziEmbeddingConfig config) {
    struct TodoziEmbeddingService* service = malloc(sizeof(struct TodoziEmbeddingService));
    if (!service) return NULL;
    
    service->config = malloc(sizeof(struct TodoziEmbeddingConfig));
    if (!service->config) {
        free(service);
        return NULL;
    }
    *service->config = config;
    
    service->cache = hashmap_new();
    if (!service->cache) {
        config_free(service->config);
        free(service->config);
        free(service);
        return NULL;
    }
    
    service->embedding_model = NULL;
    service->embedding_models = hashmap_new();
    if (!service->embedding_models) {
        hashmap_free(service->cache);
        config_free(service->config);
        free(service->config);
        free(service);
        return NULL;
    }
    
    service->tag_manager = malloc(sizeof(struct TagManager));
    service->storage = malloc(sizeof(struct Storage));
    
    if (!service->tag_manager || !service->storage) {
        if (service->tag_manager) free(service->tag_manager);
        if (service->storage) free(service->storage);
        hashmap_free(service->embedding_models);
        hashmap_free(service->cache);
        config_free(service->config);
        free(service->config);
        free(service);
        return NULL;
    }
    
    return service;
}

void service_free(struct TodoziEmbeddingService* service) {
    if (service) {
        if (service->config) {
            config_free(service->config);
            free(service->config);
        }
        
        // Free cache entries
        if (service->cache) {
            // Iterate and free all cache entries
            for (size_t i = 0; i < service->cache->capacity; i++) {
                struct HashMapEntry* entry = service->cache->buckets[i];
                while (entry) {
                    struct HashMapEntry* next = entry->next;
                    if (entry->value) {
                        cache_entry_free((struct TodoziEmbeddingCache*)entry->value);
                    }
                    entry = next;
                }
            }
            hashmap_free(service->cache);
        }
        
        if (service->embedding_model) {
            embedding_model_free(service->embedding_model);
        }
        
        if (service->embedding_models) {
            // Free all models in the hashmap
            for (size_t i = 0; i < service->embedding_models->capacity; i++) {
                struct HashMapEntry* entry = service->embedding_models->buckets[i];
                while (entry) {
                    struct HashMapEntry* next = entry->next;
                    if (entry->value) {
                        embedding_model_free((struct EmbeddingModel*)entry->value);
                    }
                    entry = next;
                }
            }
            hashmap_free(service->embedding_models);
        }
        
        if (service->tag_manager) {
            free(service->tag_manager);
        }
        if (service->storage) {
            free(service->storage);
        }
        free(service);
    }
}

int service_initialize(struct TodoziEmbeddingService* service) {
    if (!service) return EMB_ERROR_NULL_POINTER;
    
    if (config_validate(service->config) != EMB_SUCCESS) {
        return EMB_ERROR_INVALID_INPUT;
    }
    
    // Load embedding model
    service->embedding_model = embedding_model_load(service->config->model_name.data);
    if (!service->embedding_model) {
        return EMB_ERROR_INVALID_STATE;
    }
    
    if (embedding_model_validate(service->embedding_model) != EMB_SUCCESS) {
        embedding_model_free(service->embedding_model);
        service->embedding_model = NULL;
        return EMB_ERROR_INVALID_STATE;
    }
    
    return EMB_SUCCESS;
}

struct Vec service_find_similar_tasks(struct TodoziEmbeddingService* service, const char* task_description, size_t limit) {
    struct Vec results = vec_new();
    (void)limit; // TODO: Use limit when implementing full search
    
    if (!service || !task_description) {
        return results;
    }
    
    // Generate query embedding
    const char* texts[] = {task_description};
    struct Vec query_embeddings = embedding_model_encode(service->embedding_model, texts, 1);
    
    if (query_embeddings.size == 0) {
        vec_free(&query_embeddings);
        return results;
    }
    
    // TODO: In a real implementation, this would search through cached embeddings
    // and compute similarity scores
    
    vec_free(&query_embeddings);
    return results;
}

struct Vec service_semantic_search(struct TodoziEmbeddingService* service, const char* query, struct Vec* content_types, size_t limit) {
    struct Vec results = vec_new();
    (void)content_types; // TODO: Use content_types when implementing full search
    (void)limit; // TODO: Use limit when implementing full search
    
    if (!service || !query) {
        return results;
    }
    
    // Generate query embedding
    const char* texts[] = {query};
    struct Vec query_embeddings = embedding_model_encode(service->embedding_model, texts, 1);
    
    if (query_embeddings.size == 0) {
        vec_free(&query_embeddings);
        return results;
    }
    
    // TODO: In a real implementation, this would search through cached embeddings
    // and filter by content types
    
    vec_free(&query_embeddings);
    return results;
}

struct Vec service_cluster_content(struct TodoziEmbeddingService* service) {
    struct Vec clusters = vec_new();
    
    if (!service) {
        return clusters;
    }
    
    // TODO: In a real implementation, this would perform clustering
    // using algorithms like K-means or hierarchical clustering
    
    return clusters;
}

struct HashMap* service_get_stats(struct TodoziEmbeddingService* service) {
    struct HashMap* stats = hashmap_new();
    
    if (!service) {
        return stats;
    }
    
    // TODO: In a real implementation, this would collect statistics
    // about cache usage, embedding counts, etc.
    
    return stats;
}

size_t service_cleanup_expired(struct TodoziEmbeddingService* service) {
    if (!service || !service->cache) {
        return 0;
    }
    
    size_t cleaned = 0;
    time_t now = time(NULL);
    
    // Iterate through cache and remove expired entries
    for (size_t i = 0; i < service->cache->capacity; i++) {
        struct HashMapEntry* entry = service->cache->buckets[i];
        struct HashMapEntry* prev = NULL;
        
        while (entry) {
            struct HashMapEntry* next = entry->next;
            struct TodoziEmbeddingCache* cache_entry = (struct TodoziEmbeddingCache*)entry->value;
            
            if (cache_entry && cache_entry->ttl_seconds > 0) {
                if ((now - cache_entry->created_at) > (time_t)cache_entry->ttl_seconds) {
                    // Remove expired entry
                    if (prev) {
                        prev->next = next;
                    } else {
                        service->cache->buckets[i] = next;
                    }
                    cache_entry_free(cache_entry);
                    free(entry->key);
                    free(entry);
                    service->cache->size--;
                    cleaned++;
                    entry = next;
                    continue;
                }
            }
            prev = entry;
            entry = next;
        }
    }
    
    return cleaned;
}

// ============================================================================
// TOOL FUNCTIONS
// ============================================================================

struct TodoziEmbeddingTool* tool_new(struct TodoziEmbeddingConfig config) {
    struct TodoziEmbeddingTool* tool = malloc(sizeof(struct TodoziEmbeddingTool));
    if (!tool) return NULL;
    
    tool->service = service_new(config);
    if (!tool->service) {
        free(tool);
        return NULL;
    }
    
    return tool;
}

void tool_free(struct TodoziEmbeddingTool* tool) {
    if (tool) {
        if (tool->service) {
            service_free(tool->service);
        }
        free(tool);
    }
}

int tool_initialize(struct TodoziEmbeddingTool* tool) {
    if (!tool || !tool->service) {
        return EMB_ERROR_NULL_POINTER;
    }
    
    return service_initialize(tool->service);
}

// ============================================================================
// TEST FUNCTIONS
// ============================================================================

void test_config_defaults(void) {
    struct TodoziEmbeddingConfig config = config_default();
    printf("Model name: %s\n", config.model_name.data);
    printf("Dimensions: %zu\n", config.dimensions);
    printf("Similarity threshold: %f\n", config.similarity_threshold);
    config_free(&config);
}

void test_cosine_similarity(void) {
    float a[] = {1.0f, 0.0f, 0.0f};
    float b[] = {1.0f, 0.0f, 0.0f};
    float similarity = cosine_similarity(a, b, 3);
    printf("Cosine similarity: %f (expected: 1.0)\n", similarity);
    
    float c[] = {1.0f, 0.0f, 0.0f};
    float d[] = {0.0f, 1.0f, 0.0f};
    similarity = cosine_similarity(c, d, 3);
    printf("Cosine similarity (orthogonal): %f (expected: 0.0)\n", similarity);
}

void test_hashmap(void) {
    struct HashMap* map = hashmap_new();
    int value1 = 42;
    int value2 = 100;
    
    hashmap_insert(map, "key1", &value1);
    hashmap_insert(map, "key2", &value2);
    
    int* result = (int*)hashmap_get(map, "key1");
    if (result && *result == 42) {
        printf("HashMap test: PASSED\n");
    } else {
        printf("HashMap test: FAILED\n");
    }
    
    hashmap_free(map);
}

int main(void) {
    printf("Todozi Embedding Service in C\n");
    printf("=============================\n\n");
    
    // Run tests
    printf("Running tests...\n");
    test_config_defaults();
    printf("\n");
    test_cosine_similarity();
    printf("\n");
    test_hashmap();
    printf("\n");
    
    // Example usage
    printf("Example usage:\n");
    struct TodoziEmbeddingConfig config = config_default();
    struct TodoziEmbeddingTool* tool = tool_new(config);
    
    if (tool) {
        int result = tool_initialize(tool);
        if (result == EMB_SUCCESS) {
            printf("✅ Tool initialized successfully\n");
        } else {
            printf("❌ Tool initialization failed with error: %d\n", result);
        }
        
        tool_free(tool);
    } else {
        printf("❌ Failed to create tool\n");
    }
    
    config_free(&config);
    
    return 0;
}
