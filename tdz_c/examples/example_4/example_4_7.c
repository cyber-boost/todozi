// example4.c - Advanced Tag Management and Search Example

#include "tags.c"  // Include the main implementation

void demonstrate_advanced_tag_management() {
    printf("=== Advanced Tag Management Example ===\n");
    
    // Create a tag manager
    TagManager* manager = tag_manager_create();
    if (!manager) {
        printf("Failed to create TagManager\n");
        return;
    }
    
    // Create several tags with different properties
    Tag* tag1 = tag_create();
    tag1->name = string_clone("C Programming");
    tag1->description = string_clone("Tags related to C programming language");
    tag1->color = string_clone("#007ACC");
    tag1->category = string_clone("Programming");
    
    Tag* tag2 = tag_create();
    tag2->name = string_clone("Data Structures");
    tag2->description = string_clone("Tags related to data structures");
    tag2->color = string_clone("#FF6B6B");
    tag2->category = string_clone("Computer Science");
    
    Tag* tag3 = tag_create();
    tag3->name = string_clone("Algorithms");
    tag3->description = string_clone("Tags related to algorithms");
    tag3->color = string_clone("#4ECDC4");
    tag3->category = string_clone("Computer Science");
    
    Tag* tag4 = tag_create();
    tag4->name = string_clone("C++ Programming");
    tag4->description = string_clone("Tags related to C++ programming language");
    tag4->color = string_clone("#00599C");
    tag4->category = string_clone("Programming");
    
    // Add tags to manager
    char* id1, *id2, *id3, *id4;
    tag_manager_create_tag(manager, tag1, &id1);
    tag_manager_create_tag(manager, tag2, &id2);
    tag_manager_create_tag(manager, tag3, &id3);
    tag_manager_create_tag(manager, tag4, &id4);
    
    printf("Created tags with IDs: %s, %s, %s, %s\n", id1, id2, id3, id4);
    
    // Create relationships between tags
    tag_manager_add_tag_relationship(manager, id1, id2);  // C Programming -> Data Structures
    tag_manager_add_tag_relationship(manager, id1, id3);  // C Programming -> Algorithms
    tag_manager_add_tag_relationship(manager, id2, id3);  // Data Structures -> Algorithms
    tag_manager_add_tag_relationship(manager, id4, id2);  // C++ Programming -> Data Structures
    
    printf("Created tag relationships\n");
    
    // Increment usage counts
    tag_manager_increment_tag_usage(manager, "C Programming");
    tag_manager_increment_tag_usage(manager, "C Programming");
    tag_manager_increment_tag_usage(manager, "Data Structures");
    tag_manager_increment_tag_usage(manager, "Algorithms");
    tag_manager_increment_tag_usage(manager, "Algorithms");
    tag_manager_increment_tag_usage(manager, "Algorithms");
    
    printf("Incremented tag usage counts\n");
    
    // Get related tags for C Programming
    Vector* related = tag_manager_get_related_tags(manager, id1);
    printf("Tags related to 'C Programming':\n");
    for (size_t i = 0; i < vector_size(related); i++) {
        Tag* tag = (Tag*)vector_get(related, i);
        printf("  - %s\n", tag->name);
    }
    vector_free(related, NULL);
    
    // Get tags by category
    Vector* programming_tags = tag_manager_get_tags_by_category(manager, "Programming");
    printf("Tags in 'Programming' category:\n");
    for (size_t i = 0; i < vector_size(programming_tags); i++) {
        Tag* tag = (Tag*)vector_get(programming_tags, i);
        printf("  - %s\n", tag->name);
    }
    vector_free(programming_tags, NULL);
    
    // Get all categories
    Vector* categories = tag_manager_get_all_categories(manager);
    printf("All categories:\n");
    for (size_t i = 0; i < vector_size(categories); i++) {
        char* category = (char*)vector_get(categories, i);
        printf("  - %s\n", category);
    }
    vector_free(categories, free);
    
    // Get most used tags
    Vector* most_used = tag_manager_get_most_used_tags(manager, 3);
    printf("Top 3 most used tags:\n");
    for (size_t i = 0; i < vector_size(most_used); i++) {
        Tag* tag = (Tag*)vector_get(most_used, i);
        printf("  - %s (used %u times)\n", tag->name, tag->usage_count);
    }
    vector_free(most_used, NULL);
    
    // Get recent tags
    Vector* recent = tag_manager_get_recent_tags(manager, 2);
    printf("2 most recent tags:\n");
    for (size_t i = 0; i < vector_size(recent); i++) {
        Tag* tag = (Tag*)vector_get(recent, i);
        printf("  - %s\n", tag->name);
    }
    vector_free(recent, NULL);
    
    // Get tag statistics
    TagStatistics* stats = tag_manager_get_tag_statistics(manager);
    printf("Tag Statistics:\n");
    printf("  Total tags: %zu\n", stats->total_tags);
    printf("  Total categories: %zu\n", stats->total_categories);
    printf("  Total relationships: %zu\n", stats->total_relationships);
    printf("  Average usage: %.2f\n", stats->average_usage);
    printf("  Relationships per tag: %.2f\n", tag_statistics_relationships_per_tag(stats));
    tag_statistics_free(stats);
    
    // Demonstrate advanced search
    TagSearchEngine* search_engine = tag_search_engine_create(manager);
    
    // Create a complex search query
    TagSearchQuery* query = tag_search_query_create();
    query->category = string_clone("Programming");
    query->min_usage = malloc(sizeof(unsigned int));
    *query->min_usage = 1;
    query->sort_by = TAG_SORT_BY_USAGE;
    query->limit = malloc(sizeof(size_t));
    *query->limit = 5;
    
    Vector* search_results = tag_search_engine_advanced_search(search_engine, query);
    printf("Advanced search results (Programming category, min usage 1, sorted by usage):\n");
    for (size_t i = 0; i < vector_size(search_results); i++) {
        Tag* tag = (Tag*)vector_get(search_results, i);
        printf("  - %s (used %u times)\n", tag->name, tag->usage_count);
    }
    vector_free(search_results, NULL);
    
    // Demonstrate fuzzy search
    Vector* fuzzy_results = tag_search_engine_fuzzy_search(search_engine, "C Prog", 2);
    printf("Fuzzy search for 'C Prog' (max distance 2):\n");
    for (size_t i = 0; i < vector_size(fuzzy_results); i++) {
        void** pair = (void**)vector_get(fuzzy_results, i);
        Tag* tag = (Tag*)pair[0];
        size_t distance = (size_t)pair[1];
        printf("  - %s (distance: %zu)\n", tag->name, distance);
    }
    vector_free_fuzzy_results(fuzzy_results);
    
    // Demonstrate tag suggestions
    Vector* current_tags = vector_create();
    vector_push(current_tags, string_clone("C Programming"));
    Vector* suggestions = tag_search_engine_get_suggestions(search_engine, current_tags, 5);
    printf("Tag suggestions based on 'C Programming':\n");
    for (size_t i = 0; i < vector_size(suggestions); i++) {
        char* suggestion = (char*)vector_get(suggestions, i);
        printf("  - %s\n", suggestion);
    }
    vector_free(suggestions, free);
    vector_free(current_tags, free);
    
    // Cleanup
    tag_search_query_free(query);
    tag_search_engine_free(search_engine);
    tag_manager_free(manager);
    free(id1);
    free(id2);
    free(id3);
    free(id4);
    tag_free(tag1);
    tag_free(tag2);
    tag_free(tag3);
    tag_free(tag4);
    
    printf("Advanced tag management example completed\n\n");
}

int main() {
    demonstrate_advanced_tag_management();
    return 0;
}
