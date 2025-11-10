// example3.c - Advanced Tag Management System Usage
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include the main tag management system
extern TagManager* tag_manager_create();
extern void tag_manager_free(TagManager* manager);
extern Tag* tag_create();
extern void tag_free(Tag* tag);
extern TodoziError tag_manager_create_tag(TagManager* manager, Tag* tag, char** out_id);
extern Tag* tag_manager_get_tag(TagManager* manager, const char* tag_id);
extern TodoziError tag_manager_update_tag(TagManager* manager, const char* tag_id, TagUpdate* updates);
extern TodoziError tag_manager_delete_tag(TagManager* manager, const char* tag_id);
extern TodoziError tag_manager_add_tag_relationship(TagManager* manager, const char* tag_id, const char* related_tag_id);
extern Vector* tag_manager_get_related_tags(TagManager* manager, const char* tag_id);
extern Vector* tag_manager_search_tags(TagManager* manager, const char* query);
extern Vector* tag_manager_get_tags_by_category(TagManager* manager, const char* category);
extern Vector* tag_manager_get_all_categories(TagManager* manager);
extern TodoziError tag_manager_increment_tag_usage(TagManager* manager, const char* tag_name);
extern Vector* tag_manager_get_most_used_tags(TagManager* manager, size_t limit);
extern Vector* tag_manager_get_recent_tags(TagManager* manager, size_t limit);
extern TagStatistics* tag_manager_get_tag_statistics(TagManager* manager);
extern void tag_statistics_free(TagStatistics* stats);
extern TagUpdate* tag_update_new();
extern TagUpdate* tag_update_name(TagUpdate* update, const char* name);
extern TagUpdate* tag_update_description(TagUpdate* update, const char* description);
extern TagUpdate* tag_update_color(TagUpdate* update, const char* color);
extern TagUpdate* tag_update_category(TagUpdate* update, const char* category);
extern void tag_update_free(TagUpdate* update);
extern Vector* vector_create();
extern void vector_push(Vector* vec, void* item);
extern void* vector_get(Vector* vec, size_t index);
extern size_t vector_size(Vector* vec);
extern void vector_free(Vector* vec, void (*free_func)(void*));
extern void vector_free_fuzzy_results(Vector* results);
extern TagSearchEngine* tag_search_engine_create(TagManager* tag_manager);
extern void tag_search_engine_free(TagSearchEngine* engine);
extern Vector* tag_search_engine_advanced_search(TagSearchEngine* engine, TagSearchQuery* query);
extern Vector* tag_search_engine_fuzzy_search(TagSearchEngine* engine, const char* query, size_t max_distance);
extern Vector* tag_search_engine_get_suggestions(TagSearchEngine* engine, Vector* current_tags, size_t limit);
extern TagSearchQuery* tag_search_query_create();
extern void tag_search_query_free(TagSearchQuery* query);

// Helper function to print tag information
void print_tag(Tag* tag) {
    if (!tag) {
        printf("NULL tag\n");
        return;
    }
    printf("ID: %s\n", tag->id ? tag->id : "NULL");
    printf("Name: %s\n", tag->name ? tag->name : "NULL");
    printf("Description: %s\n", tag->description ? tag->description : "NULL");
    printf("Color: %s\n", tag->color ? tag->color : "NULL");
    printf("Category: %s\n", tag->category ? tag->category : "NULL");
    printf("Usage Count: %u\n", tag->usage_count);
    printf("---\n");
}

// Helper function to print tag names from a vector
void print_tag_names(Vector* tags) {
    if (!tags) {
        printf("No tags\n");
        return;
    }
    for (size_t i = 0; i < vector_size(tags); i++) {
        Tag* tag = (Tag*)vector_get(tags, i);
        if (tag && tag->name) {
            printf("- %s\n", tag->name);
        }
    }
}

int main() {
    printf("=== Advanced Tag Management System Example ===\n\n");

    // Create tag manager
    TagManager* manager = tag_manager_create();
    if (!manager) {
        printf("Failed to create tag manager\n");
        return 1;
    }
    printf("✓ Tag manager created\n");

    // Create search engine
    TagSearchEngine* search_engine = tag_search_engine_create(manager);
    if (!search_engine) {
        printf("Failed to create search engine\n");
        tag_manager_free(manager);
        return 1;
    }
    printf("✓ Search engine created\n\n");

    // Create sample tags
    char* tag_ids[10];
    for (int i = 0; i < 10; i++) {
        tag_ids[i] = NULL;
    }

    // Create technology tags
    Tag* tag1 = tag_create();
    tag1->name = strdup("C Programming");
    tag1->description = strdup("Tags related to C programming language");
    tag1->color = strdup("#00599C");
    tag1->category = strdup("Programming");
    tag_manager_create_tag(manager, tag1, &tag_ids[0]);
    tag_free(tag1);

    Tag* tag2 = tag_create();
    tag2->name = strdup("Data Structures");
    tag2->description = strdup("Tags related to data structures");
    tag2->color = strdup("#FF6B6B");
    tag2->category = strdup("Computer Science");
    tag_manager_create_tag(manager, tag2, &tag_ids[1]);
    tag_free(tag2);

    Tag* tag3 = tag_create();
    tag3->name = strdup("Algorithms");
    tag3->description = strdup("Tags related to algorithms");
    tag3->color = strdup("#4ECDC4");
    tag3->category = strdup("Computer Science");
    tag_manager_create_tag(manager, tag3, &tag_ids[2]);
    tag_free(tag3);

    Tag* tag4 = tag_create();
    tag4->name = strdup("Database Design");
    tag4->description = strdup("Tags related to database design");
    tag4->color = strdup("#45B7D1");
    tag4->category = strdup("Databases");
    tag_manager_create_tag(manager, tag4, &tag_ids[3]);
    tag_free(tag4);

    Tag* tag5 = tag_create();
    tag5->name = strdup("Web Development");
    tag5->description = strdup("Tags related to web development");
    tag5->color = strdup("#96CEB4");
    tag5->category = strdup("Web");
    tag_manager_create_tag(manager, tag5, &tag_ids[4]);
    tag_free(tag5);

    printf("✓ Created 5 sample tags\n\n");

    // Create tag relationships
    if (tag_ids[0] && tag_ids[1]) {
        tag_manager_add_tag_relationship(manager, tag_ids[0], tag_ids[1]);
    }
    if (tag_ids[1] && tag_ids[2]) {
        tag_manager_add_tag_relationship(manager, tag_ids[1], tag_ids[2]);
    }
    if (tag_ids[2] && tag_ids[0]) {
        tag_manager_add_tag_relationship(manager, tag_ids[2], tag_ids[0]);
    }
    if (tag_ids[3] && tag_ids[4]) {
        tag_manager_add_tag_relationship(manager, tag_ids[3], tag_ids[4]);
    }
    printf("✓ Created tag relationships\n\n");

    // Increment usage counts
    tag_manager_increment_tag_usage(manager, "C Programming");
    tag_manager_increment_tag_usage(manager, "C Programming");
    tag_manager_increment_tag_usage(manager, "Data Structures");
    tag_manager_increment_tag_usage(manager, "Data Structures");
    tag_manager_increment_tag_usage(manager, "Data Structures");
    printf("✓ Incremented tag usage counts\n\n");

    // Retrieve and display a specific tag
    if (tag_ids[0]) {
        Tag* retrieved_tag = tag_manager_get_tag(manager, tag_ids[0]);
        printf("Retrieved tag:\n");
        print_tag(retrieved_tag);
    }

    // Update a tag
    if (tag_ids[0]) {
        TagUpdate* update = tag_update_new();
        tag_update_description(update, "Advanced C programming concepts and best practices");
        tag_update_color(update, "#007ACC");
        tag_manager_update_tag(manager, tag_ids[0], update);
        tag_update_free(update);
        printf("✓ Updated 'C Programming' tag\n\n");
    }

    // Get related tags
    if (tag_ids[0]) {
        Vector* related = tag_manager_get_related_tags(manager, tag_ids[0]);
        printf("Tags related to 'C Programming':\n");
        print_tag_names(related);
        vector_free(related, NULL);
        printf("\n");
    }

    // Search tags
    Vector* search_results = tag_manager_search_tags(manager, "data");
    printf("Search results for 'data':\n");
    print_tag_names(search_results);
    vector_free(search_results, NULL);
    printf("\n");

    // Get tags by category
    Vector* cs_tags = tag_manager_get_tags_by_category(manager, "Computer Science");
    printf("Tags in 'Computer Science' category:\n");
    print_tag_names(cs_tags);
    vector_free(cs_tags, NULL);
    printf("\n");

    // Get all categories
    Vector* categories = tag_manager_get_all_categories(manager);
    printf("All categories:\n");
    for (size_t i = 0; i < vector_size(categories); i++) {
        char* category = (char*)vector_get(categories, i);
        if (category) {
            printf("- %s\n", category);
        }
    }
    vector_free(categories, free);
    printf("\n");

    // Get most used tags
    Vector* most_used = tag_manager_get_most_used_tags(manager, 3);
    printf("Top 3 most used tags:\n");
    print_tag_names(most_used);
    vector_free(most_used, NULL);
    printf("\n");

    // Get recent tags
    Vector* recent = tag_manager_get_recent_tags(manager, 3);
    printf("3 most recently created tags:\n");
    print_tag_names(recent);
    vector_free(recent, NULL);
    printf("\n");

    // Get tag statistics
    TagStatistics* stats = tag_manager_get_tag_statistics(manager);
    if (stats) {
        printf("Tag Statistics:\n");
        printf("- Total tags: %zu\n", stats->total_tags);
        printf("- Total categories: %zu\n", stats->total_categories);
        printf("- Total relationships: %zu\n", stats->total_relationships);
        printf("- Average usage: %.2f\n", stats->average_usage);
        tag_statistics_free(stats);
    }
    printf("\n");

    // Advanced search example
    TagSearchQuery* query = tag_search_query_create();
    query->category = strdup("Computer Science");
    query->min_usage = malloc(sizeof(unsigned int));
    *query->min_usage = 1;
    query->sort_by = TAG_SORT_BY_USAGE;
    Vector* advanced_results = tag_search_engine_advanced_search(search_engine, query);
    printf("Advanced search (Computer Science tags with usage >= 1):\n");
    print_tag_names(advanced_results);
    vector_free(advanced_results, NULL);
    tag_search_query_free(query);
    printf("\n");

    // Fuzzy search example
    Vector* fuzzy_results = tag_search_engine_fuzzy_search(search_engine, "prog", 2);
    printf("Fuzzy search for 'prog' (max distance 2):\n");
    for (size_t i = 0; i < vector_size(fuzzy_results); i++) {
        void** pair = (void**)vector_get(fuzzy_results, i);
        Tag* tag = (Tag*)pair[0];
        size_t distance = (size_t)pair[1];
        if (tag && tag->name) {
            printf("- %s (distance: %zu)\n", tag->name, distance);
        }
    }
    vector_free_fuzzy_results(fuzzy_results);
    printf("\n");

    // Tag suggestions example
    Vector* current_tags = vector_create();
    vector_push(current_tags, "C Programming");
    vector_push(current_tags, "Data Structures");
    Vector* suggestions = tag_search_engine_get_suggestions(search_engine, current_tags, 5);
    printf("Tag suggestions based on current tags:\n");
    for (size_t i = 0; i < vector_size(suggestions); i++) {
        char* suggestion = (char*)vector_get(suggestions, i);
        if (suggestion) {
            printf("- %s\n", suggestion);
        }
    }
    vector_free(suggestions, free);
    vector_free(current_tags, NULL);
    printf("\n");

    // Cleanup
    for (int i = 0; i < 10; i++) {
        if (tag_ids[i]) {
            free(tag_ids[i]);
        }
    }
    tag_search_engine_free(search_engine);
    tag_manager_free(manager);

    printf("✓ All resources cleaned up\n");
    printf("=== Example completed successfully ===\n");
    return 0;
}
