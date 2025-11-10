// example_usage.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tags.c"  // Include the main tags implementation

void print_tag(Tag* tag) {
    if (!tag) return;
    printf("Tag: %s (ID: %s)\n", tag->name, tag->id);
    printf("  Description: %s\n", tag->description ? tag->description : "N/A");
    printf("  Category: %s\n", tag->category ? tag->category : "N/A");
    printf("  Color: %s\n", tag->color ? tag->color : "N/A");
    printf("  Usage Count: %u\n", tag->usage_count);
    printf("  Created: %s", ctime(&tag->created_at.timestamp));
    printf("  Updated: %s", ctime(&tag->updated_at.timestamp));
    printf("\n");
}

void print_tags(Vector* tags, const char* title) {
    printf("=== %s ===\n", title);
    if (!tags) {
        printf("No tags found\n\n");
        return;
    }
    
    for (size_t i = 0; i < vector_size(tags); i++) {
        Tag* tag = (Tag*)vector_get(tags, i);
        print_tag(tag);
    }
    printf("\n");
}

int main() {
    printf("=== Tag Management System Example ===\n\n");
    
    // Create a tag manager
    TagManager* manager = tag_manager_create();
    if (!manager) {
        printf("Failed to create tag manager\n");
        return 1;
    }
    
    // Create some tags
    printf("1. Creating tags...\n");
    Tag* tag1 = tag_create();
    tag1->name = strdup("Bug");
    tag1->description = strdup("Software defect or issue");
    tag1->category = strdup("Status");
    tag1->color = strdup("#FF0000");
    
    Tag* tag2 = tag_create();
    tag2->name = strdup("Feature");
    tag2->description = strdup("New functionality");
    tag2->category = strdup("Type");
    tag2->color = strdup("#00FF00");
    
    Tag* tag3 = tag_create();
    tag3->name = strdup("Documentation");
    tag3->description = strdup("Documentation tasks");
    tag3->category = strdup("Type");
    tag3->color = strdup("#0000FF");
    
    Tag* tag4 = tag_create();
    tag4->name = strdup("Priority");
    tag4->description = strdup("High priority items");
    tag4->category = strdup("Status");
    tag4->color = strdup("#FFFF00");
    
    // Add tags to manager
    char* id1, *id2, *id3, *id4;
    tag_manager_create_tag(manager, tag1, &id1);
    tag_manager_create_tag(manager, tag2, &id2);
    tag_manager_create_tag(manager, tag3, &id3);
    tag_manager_create_tag(manager, tag4, &id4);
    
    printf("Created tags with IDs: %s, %s, %s, %s\n\n", id1, id2, id3, id4);
    
    // Retrieve and display all tags
    Vector* all_tags = tag_manager_get_all_tags(manager);
    print_tags(all_tags, "All Tags");
    vector_free(all_tags, NULL);
    
    // Create relationships between tags
    printf("2. Creating tag relationships...\n");
    tag_manager_add_tag_relationship(manager, id1, id2);  // Bug -> Feature
    tag_manager_add_tag_relationship(manager, id1, id4);  // Bug -> Priority
    tag_manager_add_tag_relationship(manager, id2, id3);  // Feature -> Documentation
    
    // Get related tags
    Vector* related_to_bug = tag_manager_get_related_tags(manager, id1);
    print_tags(related_to_bug, "Tags related to 'Bug'");
    vector_free(related_to_bug, NULL);
    
    // Update a tag
    printf("3. Updating tag...\n");
    TagUpdate* update = tag_update_new();
    tag_update_description(update, "Critical software defect requiring immediate attention");
    tag_update_color(update, "#FF5555");
    tag_manager_update_tag(manager, id1, update);
    tag_update_free(update);
    
    Tag* updated_tag = tag_manager_get_tag(manager, id1);
    printf("Updated tag:\n");
    print_tag(updated_tag);
    
    // Search for tags
    printf("4. Searching for tags...\n");
    Vector* search_results = tag_manager_search_tags(manager, "doc");
    print_tags(search_results, "Tags containing 'doc'");
    vector_free(search_results, NULL);
    
    // Get tags by category
    Vector* type_tags = tag_manager_get_tags_by_category(manager, "Type");
    print_tags(type_tags, "Tags in 'Type' category");
    vector_free(type_tags, NULL);
    
    // Get all categories
    Vector* categories = tag_manager_get_all_categories(manager);
    printf("=== All Categories ===\n");
    for (size_t i = 0; i < vector_size(categories); i++) {
        char* category = (char*)vector_get(categories, i);
        printf("- %s\n", category);
    }
    printf("\n");
    vector_free(categories, free);
    
    // Increment usage count
    printf("5. Incrementing usage counts...\n");
    tag_manager_increment_tag_usage(manager, "Bug");
    tag_manager_increment_tag_usage(manager, "Bug");
    tag_manager_increment_tag_usage(manager, "Feature");
    
    // Get most used tags
    Vector* most_used = tag_manager_get_most_used_tags(manager, 3);
    print_tags(most_used, "Most Used Tags (Top 3)");
    vector_free(most_used, NULL);
    
    // Get recent tags
    Vector* recent = tag_manager_get_recent_tags(manager, 2);
    print_tags(recent, "Most Recent Tags (Top 2)");
    vector_free(recent, NULL);
    
    // Get tag statistics
    printf("6. Tag Statistics...\n");
    TagStatistics* stats = tag_manager_get_tag_statistics(manager);
    if (stats) {
        printf("Total Tags: %zu\n", stats->total_tags);
        printf("Total Categories: %zu\n", stats->total_categories);
        printf("Total Relationships: %zu\n", stats->total_relationships);
        printf("Average Usage: %.2f\n", stats->average_usage);
        printf("Relationships per Tag: %.2f\n", tag_statistics_relationships_per_tag(stats));
        tag_statistics_free(stats);
    }
    printf("\n");
    
    // Advanced search using search engine
    printf("7. Advanced search...\n");
    TagSearchEngine* search_engine = tag_search_engine_create(manager);
    TagSearchQuery* query = tag_search_query_create();
    query->category = strdup("Type");
    query->sort_by = TAG_SORT_BY_NAME;
    
    Vector* advanced_results = tag_search_engine_advanced_search(search_engine, query);
    print_tags(advanced_results, "Advanced Search Results (Type category, sorted by name)");
    vector_free(advanced_results, NULL);
    
    tag_search_query_free(query);
    tag_search_engine_free(search_engine);
    
    // Fuzzy search
    printf("8. Fuzzy search...\n");
    search_engine = tag_search_engine_create(manager);
    Vector* fuzzy_results = tag_search_engine_fuzzy_search(search_engine, "bog", 2);
    printf("=== Fuzzy Search Results for 'bog' (max distance 2) ===\n");
    for (size_t i = 0; i < vector_size(fuzzy_results); i++) {
        void** pair = (void**)vector_get(fuzzy_results, i);
        Tag* tag = (Tag*)pair[0];
        size_t distance = (size_t)pair[1];
        printf("- %s (distance: %zu)\n", tag->name, distance);
    }
    printf("\n");
    vector_free_fuzzy_results(fuzzy_results);
    tag_search_engine_free(search_engine);
    
    // Bulk create tags
    printf("9. Bulk creating tags...\n");
    Vector* new_tag_names = vector_create();
    vector_push(new_tag_names, strdup("Refactor"));
    vector_push(new_tag_names, strdup("Testing"));
    vector_push(new_tag_names, strdup("Review"));
    
    Vector* created_ids;
    tag_manager_bulk_create_tags(manager, new_tag_names, "Workflow", &created_ids);
    
    printf("Created %zu new tags in 'Workflow' category:\n", vector_size(created_ids));
    for (size_t i = 0; i < vector_size(created_ids); i++) {
        char* id = (char*)vector_get(created_ids, i);
        Tag* tag = tag_manager_get_tag(manager, id);
        if (tag) {
            printf("- %s\n", tag->name);
        }
    }
    printf("\n");
    
    // Clean up bulk creation vectors
    vector_free(new_tag_names, free);
    vector_free(created_ids, free);
    
    // Cleanup
    free(id1);
    free(id2);
    free(id3);
    free(id4);
    tag_free(tag1);
    tag_free(tag2);
    tag_free(tag3);
    tag_free(tag4);
    tag_manager_free(manager);
    
    printf("Example completed successfully!\n");
    return 0;
}
