// ============================================================================
// EXAMPLE 5: Task Similarity Search System
// ============================================================================

#include "emb.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Sample task data structure
struct Task {
    struct String id;
    struct String title;
    struct String description;
    struct Vec tags;
};

// Create a new task
struct Task* task_new(const char* id, const char* title, const char* description) {
    struct Task* task = malloc(sizeof(struct Task));
    if (!task) return NULL;
    
    task->id = string_new(id);
    task->title = string_new(title);
    task->description = string_new(description);
    task->tags = vec_new();
    
    return task;
}

// Free a task
void task_free(struct Task* task) {
    if (task) {
        string_free(&task->id);
        string_free(&task->title);
        string_free(&task->description);
        vec_free(&task->tags);
        free(task);
    }
}

// Add a tag to a task
void task_add_tag(struct Task* task, const char* tag) {
    if (!task || !tag) return;
    
    struct String* tag_str = malloc(sizeof(struct String));
    if (tag_str) {
        *tag_str = string_new(tag);
        vec_push(&task->tags, tag_str);
    }
}

// Simulate embedding and caching tasks (simplified)
int cache_task_embedding(struct TodoziEmbeddingService* service, struct Task* task) {
    if (!service || !task) return EMB_ERROR_NULL_POINTER;
    
    // Create cache entry
    struct TodoziEmbeddingCache* entry = malloc(sizeof(struct TodoziEmbeddingCache));
    if (!entry) return EMB_ERROR_MEMORY;
    
    entry->content_id = string_new(task->id.data);
    entry->text_content = string_new(task->description.data);
    entry->vector = vec_new(); // In real implementation, this would contain actual embeddings
    entry->tags = vec_new();
    entry->created_at = time(NULL);
    entry->ttl_seconds = service->config->cache_ttl_seconds;
    
    // Copy tags
    for (size_t i = 0; i < task->tags.size; i++) {
        struct String* original_tag = (struct String*)vec_get(&task->tags, i);
        if (original_tag) {
            struct String* tag_copy = malloc(sizeof(struct String));
            if (tag_copy) {
                *tag_copy = string_new(original_tag->data);
                vec_push(&entry->tags, tag_copy);
            }
        }
    }
    
    // Insert into cache
    int result = cache_insert(service->cache, task->id.data, entry);
    if (result != EMB_SUCCESS) {
        cache_entry_free(entry);
    }
    
    return result;
}

// Example usage
int main(void) {
    printf("Example 5: Task Similarity Search System\n");
    printf("========================================\n\n");
    
    // Initialize service
    struct TodoziEmbeddingConfig config = config_default();
    struct TodoziEmbeddingTool* tool = tool_new(config);
    
    if (!tool) {
        printf("❌ Failed to create tool\n");
        config_free(&config);
        return 1;
    }
    
    if (tool_initialize(tool) != EMB_SUCCESS) {
        printf("❌ Tool initialization failed\n");
        tool_free(tool);
        config_free(&config);
        return 1;
    }
    
    printf("✅ Service initialized successfully\n\n");
    
    // Create sample tasks
    struct Task* task1 = task_new("task-001", "Bug Fix", "Fix memory leak in user authentication module");
    task_add_tag(task1, "bug");
    task_add_tag(task1, "security");
    
    struct Task* task2 = task_new("task-002", "Feature Implementation", "Implement password reset functionality");
    task_add_tag(task2, "feature");
    task_add_tag(task2, "security");
    
    struct Task* task3 = task_new("task-003", "Performance Optimization", "Optimize database queries for user dashboard");
    task_add_tag(task3, "performance");
    task_add_tag(task3, "database");
    
    struct Task* task4 = task_new("task-004", "Security Enhancement", "Add two-factor authentication support");
    task_add_tag(task4, "security");
    task_add_tag(task4, "feature");
    
    // Cache tasks (in a real system, this would happen when tasks are created/updated)
    cache_task_embedding(tool->service, task1);
    cache_task_embedding(tool->service, task2);
    cache_task_embedding(tool->service, task3);
    cache_task_embedding(tool->service, task4);
    
    printf("✅ Cached %zu tasks\n\n", hashmap_size(tool->service->cache));
    
    // Perform similarity search
    const char* query = "Need to fix a security issue with user login";
    printf("Searching for tasks similar to: \"%s\"\n", query);
    
    struct Vec results = service_find_similar_tasks(tool->service, query, 3);
    
    if (results.size > 0) {
        printf("\nFound %zu similar tasks:\n", results.size);
        for (size_t i = 0; i < results.size; i++) {
            struct TodoziEmbeddingCache* result = (struct TodoziEmbeddingCache*)vec_get(&results, i);
            if (result) {
                printf("  %zu. %s\n", i + 1, result->content_id.data);
            }
        }
    } else {
        printf("\nNo similar tasks found (this is expected in the demo since embeddings are not computed)\n");
    }
    
    // Perform semantic search with tag filtering
    printf("\nPerforming semantic search for \"database optimization\" with 'performance' tag...\n");
    
    struct Vec content_types = vec_new();
    struct String* performance_tag = malloc(sizeof(struct String));
    *performance_tag = string_new("performance");
    vec_push(&content_types, performance_tag);
    
    struct Vec search_results = service_semantic_search(tool->service, "database optimization", &content_types, 5);
    
    if (search_results.size > 0) {
        printf("\nFound %zu matching tasks:\n", search_results.size);
        for (size_t i = 0; i < search_results.size; i++) {
            struct TodoziEmbeddingCache* result = (struct TodoziEmbeddingCache*)vec_get(&search_results, i);
            if (result) {
                printf("  %zu. %s\n", i + 1, result->content_id.data);
            }
        }
    } else {
        printf("\nNo matching tasks found (this is expected in the demo since embeddings are not computed)\n");
    }
    
    // Cleanup
    string_free(&performance_tag->data);
    free(performance_tag);
    vec_free(&content_types);
    vec_free(&results);
    vec_free(&search_results);
    
    task_free(task1);
    task_free(task2);
    task_free(task3);
    task_free(task4);
    
    tool_free(tool);
    config_free(&config);
    
    printf("\n✅ Example completed successfully\n");
    return 0;
}
