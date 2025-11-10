#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Assuming all the structures and functions from tags.c are available

void print_tag(Tag* tag) {
    if (!tag) return;
    printf("ID: %s\n", tag->id ? tag->id : "NULL");
    printf("Name: %s\n", tag->name ? tag->name : "NULL");
    printf("Description: %s\n", tag->description ? tag->description : "NULL");
    printf("Category: %s\n", tag->category ? tag->category : "NULL");
    printf("Color: %s\n", tag->color ? tag->color : "NULL");
    printf("Usage Count: %u\n", tag->usage_count);
    printf("---\n");
}

int main() {
    // Create tag manager
    TagManager* manager = tag_manager_create();
    if (!manager) {
        printf("Failed to create TagManager\n");
        return 1;
    }

    // Create search engine
    TagSearchEngine* search_engine = tag_search_engine_create(manager);
    if (!search_engine) {
        printf("Failed to create TagSearchEngine\n");
        tag_manager_free(manager);
        return 1;
    }

    // Create some tags
    Tag* tag1 = tag_create();
    tag1->name = strdup("C Programming");
    tag1->description = strdup("Tags related to C language development");
    tag1->category = strdup("Programming");
    tag1->color = strdup("#007ACC");

    Tag* tag2 = tag_create();
    tag2->name = strdup("Data Structures");
    tag2->description = strdup("Tags related to data organization");
    tag2->category = strdup("Computer Science");
    tag2->color = strdup("#FF6B6B");

    Tag* tag3 = tag_create();
    tag3->name = strdup("Algorithms");
    tag3->description = strdup("Problem-solving techniques");
    tag3->category = strdup("Computer Science");
    tag3->color = strdup("#4ECDC4");

    // Add tags to manager
    char* id1, *id2, *id3;
    if (tag_manager_create_tag(manager, tag1, &id1) != TODOZI_SUCCESS) {
        printf("Failed to create tag1\n");
        goto cleanup;
    }
    if (tag_manager_create_tag(manager, tag2, &id2) != TODOZI_SUCCESS) {
        printf("Failed to create tag2\n");
        goto cleanup;
    }
    if (tag_manager_create_tag(manager, tag3, &id3) != TODOZI_SUCCESS) {
        printf("Failed to create tag3\n");
        goto cleanup;
    }

    printf("Created tags:\n");
    print_tag(tag_manager_get_tag(manager, id1));
    print_tag(tag_manager_get_tag(manager, id2));
    print_tag(tag_manager_get_tag(manager, id3));

    // Create tag relationships
    tag_manager_add_tag_relationship(manager, id1, id2);
    tag_manager_add_tag_relationship(manager, id1, id3);
    tag_manager_add_tag_relationship(manager, id2, id3);

    // Increment usage count
    tag_manager_increment_tag_usage(manager, "C Programming");
    tag_manager_increment_tag_usage(manager, "C Programming");
    tag_manager_increment_tag_usage(manager, "Data Structures");

    // Get related tags
    printf("\nRelated tags for 'C Programming':\n");
    Vector* related = tag_manager_get_related_tags(manager, id1);
    for (size_t i = 0; i < vector_size(related); i++) {
        Tag* tag = (Tag*)vector_get(related, i);
        printf("- %s\n", tag->name);
    }
    vector_free(related, NULL);

    // Search tags
    printf("\nSearch results for 'program':\n");
    Vector* search_results = tag_manager_search_tags(manager, "program");
    for (size_t i = 0; i < vector_size(search_results); i++) {
        Tag* tag = (Tag*)vector_get(search_results, i);
        printf("- %s\n", tag->name);
    }
    vector_free(search_results, NULL);

    // Get tags by category
    printf("\nTags in 'Computer Science' category:\n");
    Vector* category_tags = tag_manager_get_tags_by_category(manager, "Computer Science");
    for (size_t i = 0; i < vector_size(category_tags); i++) {
        Tag* tag = (Tag*)vector_get(category_tags, i);
        printf("- %s\n", tag->name);
    }
    vector_free(category_tags, NULL);

    // Get most used tags
    printf("\nTop 2 most used tags:\n");
    Vector* top_tags = tag_manager_get_most_used_tags(manager, 2);
    for (size_t i = 0; i < vector_size(top_tags); i++) {
        Tag* tag = (Tag*)vector_get(top_tags, i);
        printf("- %s (used %u times)\n", tag->name, tag->usage_count);
    }
    vector_free(top_tags, NULL);

    // Advanced search
    printf("\nAdvanced search - tags with usage >= 1:\n");
    TagSearchQuery* query = tag_search_query_create();
    unsigned int min_usage = 1;
    query->min_usage = &min_usage;
    query->sort_by = TAG_SORT_BY_USAGE;
    Vector* advanced_results = tag_search_engine_advanced_search(search_engine, query);
    for (size_t i = 0; i < vector_size(advanced_results); i++) {
        Tag* tag = (Tag*)vector_get(advanced_results, i);
        printf("- %s (used %u times)\n", tag->name, tag->usage_count);
    }
    vector_free(advanced_results, NULL);
    tag_search_query_free(query);

    // Fuzzy search
    printf("\nFuzzy search for 'algoritm' (max distance 2):\n");
    Vector* fuzzy_results = tag_search_engine_fuzzy_search(search_engine, "algoritm", 2);
    for (size_t i = 0; i < vector_size(fuzzy_results); i++) {
        void** pair = (void**)vector_get(fuzzy_results, i);
        Tag* tag = (Tag*)pair[0];
        size_t distance = (size_t)pair[1];
        printf("- %s (distance: %zu)\n", tag->name, distance);
    }
    vector_free_fuzzy_results(fuzzy_results);

    // Get tag statistics
    TagStatistics* stats = tag_manager_get_tag_statistics(manager);
    printf("\nTag Statistics:\n");
    printf("Total tags: %zu\n", stats->total_tags);
    printf("Total categories: %zu\n", stats->total_categories);
    printf("Total relationships: %zu\n", stats->total_relationships);
    printf("Average usage: %.2f\n", stats->average_usage);
    tag_statistics_free(stats);

cleanup:
    free(id1);
    free(id2);
    free(id3);
    tag_free(tag1);
    tag_free(tag2);
    tag_free(tag3);
    tag_search_engine_free(search_engine);
    tag_manager_free(manager);
    return 0;
}
