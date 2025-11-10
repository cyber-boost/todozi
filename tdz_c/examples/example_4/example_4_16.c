#include <stdio.h>
#include <stdlib.h>
#include "summary.c"  // Include the provided implementation

int main() {
    // Create a new summary manager
    SummaryManager* manager = summary_manager_new();
    if (!manager) {
        printf("Failed to create summary manager\n");
        return 1;
    }

    // Create sample summaries
    Summary summary1 = {
        .content = "Complete project proposal",
        .context = "Work",
        .priority = SUMMARY_PRIORITY_HIGH
    };
    summary1.tags.data = NULL;
    summary1.tags.size = 0;
    summary1.tags.capacity = 0;
    string_vec_push(&summary1.tags, "work");
    string_vec_push(&summary1.tags, "project");

    Summary summary2 = {
        .content = "Buy groceries",
        .context = "Personal",
        .priority = SUMMARY_PRIORITY_MEDIUM
    };
    summary2.tags.data = NULL;
    summary2.tags.size = 0;
    summary2.tags.capacity = 0;
    string_vec_push(&summary2.tags, "personal");
    string_vec_push(&summary2.tags, "shopping");

    Summary summary3 = {
        .content = "Fix critical bug in production",
        .context = "Work",
        .priority = SUMMARY_PRIORITY_CRITICAL
    };
    summary3.tags.data = NULL;
    summary3.tags.size = 0;
    summary3.tags.capacity = 0;
    string_vec_push(&summary3.tags, "work");
    string_vec_push(&summary3.tags, "bug");

    // Add summaries to manager
    char* id1 = summary_manager_create_summary(manager, &summary1);
    char* id2 = summary_manager_create_summary(manager, &summary2);
    char* id3 = summary_manager_create_summary(manager, &summary3);

    if (!id1 || !id2 || !id3) {
        printf("Failed to create summaries\n");
        return 1;
    }

    printf("Created summaries with IDs: %s, %s, %s\n", id1, id2, id3);

    // Get all summaries
    size_t count;
    Summary** all_summaries = summary_manager_get_all_summaries(manager, &count);
    printf("\nAll summaries (%zu):\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("- %s (Priority: %d)\n", all_summaries[i]->content, all_summaries[i]->priority);
    }
    free(all_summaries);

    // Search summaries
    Summary** search_results = summary_manager_search_summaries(manager, "work", &count);
    printf("\nSearch results for 'work' (%zu):\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("- %s\n", search_results[i]->content);
    }
    free(search_results);

    // Get high priority summaries
    Summary** high_priority = summary_manager_get_high_priority_summaries(manager, &count);
    printf("\nHigh priority summaries (%zu):\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("- %s\n", high_priority[i]->content);
    }
    free(high_priority);

    // Update a summary
    SummaryUpdate* update = summary_update_new();
    summary_update_content(update, "Complete project proposal and presentation");
    summary_update_priority(update, SUMMARY_PRIORITY_CRITICAL);
    
    if (summary_manager_update_summary(manager, id1, update)) {
        printf("\nUpdated summary %s\n", id1);
        Summary* updated = summary_manager_get_summary(manager, id1);
        printf("New content: %s\n", updated->content);
        printf("New priority: %d\n", updated->priority);
    }
    summary_update_free(update);

    // Get statistics
    SummaryStatistics* stats = summary_manager_get_summary_statistics(manager);
    printf("\nSummary Statistics:\n");
    printf("Total summaries: %zu\n", stats->total_summaries);
    printf("High priority summaries: %zu\n", stats->high_priority_summaries);
    printf("Unique tags: %zu\n", stats->unique_tags);
    printf("High priority percentage: %.2f%%\n", 
           summary_statistics_high_priority_percentage(stats));
    summary_statistics_free(stats);

    // Get all tags
    char** all_tags = summary_manager_get_all_tags(manager, &count);
    printf("\nAll tags (%zu):\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("- %s\n", all_tags[i]);
        free(all_tags[i]);
    }
    free(all_tags);

    // Get tag statistics
    StringStringMap* tag_stats = summary_manager_get_tag_statistics(manager);
    printf("\nTag Statistics:\n");
    for (size_t i = 0; i < tag_stats->size; i++) {
        printf("- %s: %s\n", tag_stats->keys[i], tag_stats->values[i]);
    }
    string_string_map_free(tag_stats);

    // Clean up
    free(id1);
    free(id2);
    free(id3);
    summary_manager_free(manager);
    
    return 0;
}
