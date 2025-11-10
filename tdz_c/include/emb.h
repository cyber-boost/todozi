#ifndef TODOZI_EMB_H
#define TODOZI_EMB_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stddef.h>
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declarations
typedef struct TodoziEmbeddingConfig TodoziEmbeddingConfig;
typedef struct TodoziEmbeddingCache TodoziEmbeddingCache;
typedef struct TodoziContentType TodoziContentType;
typedef struct HashMapEntry HashMapEntry;
typedef struct HashMap HashMap;
typedef struct Vec Vec;
typedef struct String String;
typedef struct SimilarityResult SimilarityResult;
typedef struct ClusteringResult ClusteringResult;
typedef struct AggregationType AggregationType;
typedef struct SearchFilters SearchFilters;
typedef struct HierarchicalCluster HierarchicalCluster;
typedef struct LabeledCluster LabeledCluster;
typedef struct DriftReport DriftReport;
typedef struct DriftSnapshot DriftSnapshot;
typedef struct SimilarityGraph SimilarityGraph;
typedef struct GraphNode GraphNode;
typedef struct GraphEdge GraphEdge;
typedef struct ModelComparisonResult ModelComparisonResult;
typedef struct ModelEmbeddingResult ModelEmbeddingResult;
typedef struct ValidationReport ValidationReport;
typedef struct ValidationIssue ValidationIssue;
typedef struct PerformanceMetrics PerformanceMetrics;
typedef struct DiagnosticReport DiagnosticReport;
typedef struct EmbeddingStats EmbeddingStats;
typedef struct LRUEmbeddingCache LRUEmbeddingCache;
typedef struct EmbeddingModel EmbeddingModel;
typedef struct TodoziEmbeddingService TodoziEmbeddingService;
typedef struct TodoziEmbeddingTool TodoziEmbeddingTool;
typedef struct Task Task;
typedef struct Tag Tag;
typedef struct Idea Idea;
typedef struct Memory Memory;
typedef struct TagManager TagManager;
typedef struct Storage Storage;

// Content type enum
typedef enum {
    TASK,
    TAG,
    MEMORY,
    IDEA,
    CHUNK,
    FEEL,
    TRAIN,
    ERROR,
    SUMMARY,
    REMINDER,
    TDZ
} TodoziContentTypeEnum;

// Aggregation type enum
typedef enum {
    AVERAGE,
    MAX,
    MIN,
    WEIGHTED
} AggregationTypeEnum;

// Error codes
typedef enum {
    EMB_SUCCESS = 0,
    EMB_ERROR_NULL_POINTER = -1,
    EMB_ERROR_INVALID_INPUT = -2,
    EMB_ERROR_MEMORY = -3,
    EMB_ERROR_NOT_FOUND = -4,
    EMB_ERROR_INVALID_STATE = -5
} EmbeddingErrorCode;

// Basic types
struct String {
    char* data;
    size_t len;
    size_t capacity;
};

struct Vec {
    void** data;
    size_t size;
    size_t capacity;
};

struct HashMapEntry {
    char* key;
    void* value;
    struct HashMapEntry* next;
};

struct HashMap {
    struct HashMapEntry** buckets;
    size_t size;
    size_t capacity;
};

struct TodoziContentType {
    TodoziContentTypeEnum type;
};

// Configuration structure
struct TodoziEmbeddingConfig {
    struct String model_name;
    size_t dimensions;
    float similarity_threshold;
    size_t max_results;
    unsigned long cache_ttl_seconds;
    bool enable_clustering;
    float clustering_threshold;
};

// Cache structure
struct TodoziEmbeddingCache {
    struct Vec vector;  // struct Vec<f32>
    struct TodoziContentType content_type;
    struct String content_id;
    struct String text_content;
    struct Vec tags;    // struct Vec<struct String>
    time_t created_at;
    unsigned long ttl_seconds;
};

// Similarity result structure
struct SimilarityResult {
    struct String content_id;
    struct TodoziContentType content_type;
    float similarity_score;
    struct String text_content;
    struct Vec tags;
    struct HashMap metadata;
};

// Clustering result structure
struct ClusteringResult {
    struct String cluster_id;
    struct Vec content_items;
    struct Vec cluster_center;
    size_t cluster_size;
    float average_similarity;
};

// Aggregation types
struct AggregationType {
    AggregationTypeEnum type;
    struct Vec weights;
};

// Search filters
struct SearchFilters {
    struct Vec* tags;
    struct Vec* priority;
    struct Vec* status;
    struct Vec* assignee;
    time_t* date_from;
    time_t* date_to;
    unsigned char* min_progress;
    unsigned char* max_progress;
};

// Hierarchical cluster
struct HierarchicalCluster {
    struct String cluster_id;
    size_t level;
    struct Vec content_items;
    struct Vec cluster_center;
    struct Vec children;
    struct String* parent_id;
    float average_similarity;
};

// Labeled cluster
struct LabeledCluster {
    struct String cluster_id;
    struct String label;
    struct String* description;
    float confidence;
    struct Vec content_items;
};

// Drift report
struct DriftReport {
    struct String content_id;
    float current_similarity_to_original;
    float drift_percentage;
    bool significant_drift;
    struct Vec history;
};

// Drift snapshot
struct DriftSnapshot {
    time_t timestamp;
    float similarity_to_original;
    struct String text_sample;
};

// Similarity graph
struct SimilarityGraph {
    struct Vec nodes;
    struct Vec edges;
};

// Graph node
struct GraphNode {
    struct String id;
    struct TodoziContentType content_type;
    struct String label;
    struct HashMap metadata;
};

// Graph edge
struct GraphEdge {
    struct String from;
    struct String to;
    float similarity;
    bool bidirectional;
};

// Model comparison result
struct ModelComparisonResult {
    struct String text;
    struct HashMap models;
};

// Model embedding result
struct ModelEmbeddingResult {
    struct String model_name;
    struct Vec embedding;
    size_t dimensions;
    unsigned long long generation_time_ms;
};

// Validation report
struct ValidationReport {
    size_t total_embeddings;
    size_t invalid_embeddings;
    size_t nan_count;
    size_t infinity_count;
    size_t zero_vector_count;
    struct Vec abnormal_distributions;
    struct Vec issues;
};

// Validation issue
struct ValidationIssue {
    struct String content_id;
    struct String issue_type;
    struct String severity;
    struct String description;
};

// Performance metrics
struct PerformanceMetrics {
    struct String query;
    size_t iterations;
    double avg_time_ms;
    unsigned long long min_time_ms;
    unsigned long long max_time_ms;
    double std_dev_ms;
    size_t results_per_iteration;
};

// Embedding stats
struct EmbeddingStats {
    struct Vec mean;
    struct Vec std_dev;
    struct Vec min;
    struct Vec max;
};

// Diagnostic report
struct DiagnosticReport {
    time_t timestamp;
    float cache_hit_rate;
    float avg_similarity_score;
    struct EmbeddingStats embedding_distribution_stats;
    struct HashMap content_type_breakdown;
    struct Vec top_similar_pairs;
};

// LRU Cache implementation
struct LRUEmbeddingCache {
    size_t max_memory_mb;
    struct Vec cache;
    struct HashMap* access_counts;
    size_t current_memory_bytes;
};

// Embedding model structure
struct EmbeddingModel {
    void* model;
    void* tokenizer;
    void* device;
    size_t dimensions;
};

// Main service structure
struct TodoziEmbeddingService {
    struct TodoziEmbeddingConfig* config;
    struct HashMap* cache;
    struct EmbeddingModel* embedding_model;
    struct HashMap* embedding_models;
    struct TagManager* tag_manager;
    struct Storage* storage;
};

// Tool structure
struct TodoziEmbeddingTool {
    struct TodoziEmbeddingService* service;
};

// Simplified structures
struct Task {
    struct String id;
    struct String action;
    struct String parent_project;
    struct Vec tags;
    struct String* context_notes;
    float* embedding_vector;
};

struct Tag {
    struct String id;
    struct String name;
    struct String* description;
    size_t usage_count;
};

struct Idea {
    struct String id;
    struct String idea;
    struct Vec tags;
};

struct Memory {
    struct String id;
    struct String moment;
    struct String meaning;
    struct String reason;
    struct Vec tags;
};

struct TagManager {
    // Implementation details
};

struct Storage {
    // Implementation details
};

// Utility functions
struct String string_new(const char* str);
struct String string_new_with_capacity(size_t capacity);
void string_free(struct String* s);
int string_append(struct String* s, const char* str);
int string_resize(struct String* s, size_t new_capacity);

struct Vec vec_new(void);
struct Vec vec_new_with_capacity(size_t capacity);
void vec_push(struct Vec* v, void* item);
void* vec_get(struct Vec* v, size_t index);
void vec_free(struct Vec* v);
void vec_clear(struct Vec* v);

struct HashMap* hashmap_new(void);
struct HashMap* hashmap_new_with_capacity(size_t capacity);
void hashmap_free(struct HashMap* map);
int hashmap_insert(struct HashMap* map, const char* key, void* value);
void* hashmap_get(struct HashMap* map, const char* key);
int hashmap_remove(struct HashMap* map, const char* key);
size_t hashmap_size(struct HashMap* map);

// Configuration functions
struct TodoziEmbeddingConfig config_default(void);
void config_free(struct TodoziEmbeddingConfig* config);
int config_validate(struct TodoziEmbeddingConfig* config);

// LRU Cache functions
struct LRUEmbeddingCache* lru_cache_new(size_t max_memory_mb);
void lru_cache_free(struct LRUEmbeddingCache* cache);
int lru_cache_get(struct LRUEmbeddingCache* cache, const char* key, struct TodoziEmbeddingCache** value);
int lru_cache_put(struct LRUEmbeddingCache* cache, const char* key, struct TodoziEmbeddingCache* value);

// Cache functions
struct TodoziEmbeddingCache* cache_get(struct HashMap* cache, const char* key);
int cache_insert(struct HashMap* cache, const char* key, struct TodoziEmbeddingCache* value);
int cache_remove(struct HashMap* cache, const char* key);
void cache_entry_free(struct TodoziEmbeddingCache* entry);

// Embedding model functions
struct EmbeddingModel* embedding_model_load(const char* model_name);
void embedding_model_free(struct EmbeddingModel* model);
struct Vec embedding_model_encode(struct EmbeddingModel* model, const char** texts, size_t text_count);
int embedding_model_validate(struct EmbeddingModel* model);

// Similarity functions
float cosine_similarity(const float* a, const float* b, size_t len);
float euclidean_distance(const float* a, const float* b, size_t len);
float dot_product(const float* a, const float* b, size_t len);

// Service functions
struct TodoziEmbeddingService* service_new(struct TodoziEmbeddingConfig config);
void service_free(struct TodoziEmbeddingService* service);
int service_initialize(struct TodoziEmbeddingService* service);
struct Vec service_find_similar_tasks(struct TodoziEmbeddingService* service, const char* task_description, size_t limit);
struct Vec service_semantic_search(struct TodoziEmbeddingService* service, const char* query, struct Vec* content_types, size_t limit);
struct Vec service_cluster_content(struct TodoziEmbeddingService* service);
struct HashMap* service_get_stats(struct TodoziEmbeddingService* service);
size_t service_cleanup_expired(struct TodoziEmbeddingService* service);

// Tool functions
struct TodoziEmbeddingTool* tool_new(struct TodoziEmbeddingConfig config);
void tool_free(struct TodoziEmbeddingTool* tool);
int tool_initialize(struct TodoziEmbeddingTool* tool);

#ifdef __cplusplus
}
#endif

#endif // TODOZI_EMB_H

