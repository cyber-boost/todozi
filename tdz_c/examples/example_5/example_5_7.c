// Example 5: Advanced Tag Management System
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Assuming all the previous structures and functions are available

void example_advanced_tag_management() {
    printf("=== Example 5: Advanced Tag Management ===\n");
    
    // Create tag manager
    TagManager* manager = tag_manager_create();
    if (!manager) {
        printf("Failed to create tag manager\n");
        return;
    }
    
    // Create project tags
    Tag* frontend = tag_create();
    frontend->name = strdup("Frontend");
    frontend->category = strdup("Development");
    frontend->color = strdup("#3498db");
    frontend->description = strdup("User interface development");
    
    Tag* backend = tag_create();
    backend->name = strdup("Backend");
    backend->category = strdup("Development");
    backend->color = strdup("#e74c3c");
    backend->description = strdup("Server-side logic");
    
    Tag* database = tag_create();
    database->name = strdup("Database");
    database->category = strdup("Infrastructure");
    database->color = strdup("#2ecc71");
    
    Tag* api = tag_create();
    api->name = strdup("API");
    api->category = strdup("Development");
    api->color = strdup("#f39c12");
    
    // Add tags to manager
    char* frontend_id, *backend_id, *database_id, *api_id;
    tag_manager_create_tag(manager, frontend, &frontend_id);
    tag_manager_create_tag(manager, backend, &backend_id);
    tag_manager_create_tag(manager, database, &database_id);
    tag_manager_create_tag(manager, api, &api_id);
    
    printf("Created tags with IDs:\n");
    printf("- Frontend: %s\n", frontend_id);
    printf("- Backend: %s\n", backend_id);
    printf("- Database: %s\n", database_id);
    printf("- API: %s\n", api_id);
    
    // Create relationships
    tag_manager_add_tag_relationship(manager, frontend_id, api_id);
    tag_manager_add_tag_relationship(manager, backend_id, api_id);
    tag_manager_add_tag_relationship(manager, backend_id, database_id);
    tag_manager_add_tag_relationship(manager, frontend_id, backend_id);
    
    printf("\nCreated tag relationships\n");
    
    // Get related tags
    Vector* related_to_frontend = tag_manager_get_related_tags(manager, frontend_id);
    printf("Tags related to Frontend: ");
    for (size_t i = 0; i < vector_size(related_to_frontend); i++) {
        Tag* tag = (Tag*)vector_get(related_to_frontend, i);
        printf("%s ", tag->name);
    }
    printf("\n");
    vector_free(related_to_frontend, NULL);
    
    // Bulk create tags
    Vector* new_tag_names = vector_create();
    vector_push(new_tag_names, strdup("Testing"));
    vector_push(new_tag_names, strdup("Deployment"));
    vector_push(new_tag_names, strdup("Documentation"));
    
    Vector* new_ids;
    tag_manager_bulk_create_tags(manager, new_tag_names, "Operations", &new_ids);
    
    printf("\nBulk created tags:\n");
    for (size_t i = 0; i < vector_size(new_ids); i++) {
        char* id = (char*)vector_get(new_ids, i);
        Tag* tag = tag_manager_get_tag(manager, id);
        if (tag) {
            printf("- %s (ID: %s)\n", tag->name, id);
        }
    }
    
    // Increment usage counts
    tag_manager_increment_tag_usage(manager, "Frontend");
    tag_manager_increment_tag_usage(manager, "Frontend");
    tag_manager_increment_tag_usage(manager, "Backend");
    tag_manager_increment_tag_usage(manager, "API");
    tag_manager_increment_tag_usage(manager, "API");
    tag_manager_increment_tag_usage(manager, "API");
    
    printf("\nIncremented usage counts\n");
    
    // Get most used tags
    Vector* popular_tags = tag_manager_get_most_used_tags(manager, 3);
    printf("Top 3 most used tags:\n");
    for (size_t i = 0; i < vector_size(popular_tags); i++) {
        Tag* tag = (Tag*)vector_get(popular_tags, i);
        printf("- %s (%u uses)\n", tag->name, tag->usage_count);
    }
    vector_free(popular_tags, NULL);
    
    // Get statistics
    TagStatistics* stats = tag_manager_get_tag_statistics(manager);
    printf("\nTag Statistics:\n");
    printf("- Total tags: %zu\n", stats->total_tags);
    printf("- Total categories: %zu\n", stats->total_categories);
    printf("- Total relationships: %zu\n", stats->total_relationships);
    printf("- Average usage: %.2f\n", stats->average_usage);
    printf("- Relationships per tag: %.2f\n", tag_statistics_relationships_per_tag(stats));
    tag_statistics_free(stats);
    
    // Advanced search
    TagSearchEngine* search_engine = tag_search_engine_create(manager);
    TagSearchQuery* query = tag_search_query_create();
    query->category = strdup("Development");
    query->min_usage = malloc(sizeof(unsigned int));
    *query->min_usage = 1;
    query->sort_by = TAG_SORT_BY_USAGE;
    
    Vector* search_results = tag_search_engine_advanced_search(search_engine, query);
    printf("\nAdvanced search results (Development tags with >=1 usage):\n");
    for (size_t i = 0; i < vector_size(search_results); i++) {
        Tag* tag = (Tag*)vector_get(search_results, i);
        printf("- %s (%u uses)\n", tag->name, tag->usage_count);
    }
    vector_free(search_results, NULL);
    
    // Fuzzy search
    Vector* fuzzy_results = tag_search_engine_fuzzy_search(search_engine, "frntend", 2);
    printf("\nFuzzy search for 'frntend' (max distance 2):\n");
    for (size_t i = 0; i < vector_size(fuzzy_results); i++) {
        void** pair = (void**)vector_get(fuzzy_results, i);
        Tag* tag = (Tag*)pair[0];
        size_t distance = (size_t)pair[1];
        printf("- %s (distance: %zu)\n", tag->name, distance);
    }
    vector_free_fuzzy_results(fuzzy_results);
    
    // Cleanup
    tag_search_query_free(query);
    tag_search_engine_free(search_engine);
    vector_free(new_tag_names, free);
    vector_free(new_ids, free);
    tag_manager_free(manager);
    free(frontend_id);
    free(backend_id);
    free(database_id);
    free(api_id);
    tag_free(frontend);
    tag_free(backend);
    tag_free(database);
    tag_free(api);
    
    printf("\nAdvanced tag management example completed\n\n");
}

int main() {
    example_advanced_tag_management();
    return 0;
}
