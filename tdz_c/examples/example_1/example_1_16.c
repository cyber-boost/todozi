#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include the summary.c file or its header here
// For this example, we'll assume the functions are available

int main() {
    // Create a new summary manager
    SummaryManager* manager = summary_manager_new();
    if (!manager) {
        printf("Failed to create summary manager\n");
        return 1;
    }

    // Create a new summary
    Summary* summary1 = summary_new();
    summary1->content = string_clone("Complete project documentation");
    summary1->context = string_clone("Work");
    summary1->priority = SUMMARY_PRIORITY_HIGH;
    
    // Add tags to the summary
    string_vec_push(&summary1->tags, "documentation");
    string_vec_push(&summary1->tags, "project");

    // Add the summary to the manager
    char* id1 = summary_manager_create_summary(manager, summary1);
    if (id1) {
        printf("Created summary with ID: %s\n", id1);
        free(id1);
    } else {
        printf("Failed to create summary\n");
    }

    // Create another summary
    Summary* summary2 = summary_new();
    summary2->content = string_clone("Buy groceries for dinner");
    summary2->context = string_clone("Personal");
    summary2->priority = SUMMARY_PRIORITY_MEDIUM;
    
    // Add tags to the second summary
    string_vec_push(&summary2->tags, "shopping");
    string_vec_push(&summary2->tags, "cooking");

    // Add the second summary to the manager
    char* id2 = summary_manager_create_summary(manager, summary2);
    if (id2) {
        printf("Created summary with ID: %s\n", id2);
        free(id2);
    } else {
        printf("Failed to create summary\n");
    }

    // Retrieve all summaries
    size_t count;
    Summary** all_summaries = summary_manager_get_all_summaries(manager, &count);
    printf("\nAll summaries (%zu):\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("- %s (Priority: %d)\n", all_summaries[i]->content, all_summaries[i]->priority);
    }
    free(all_summaries);

    // Search for summaries containing "project"
    Summary** search_results = summary_manager_search_summaries(manager, "project", &count);
    printf("\nSearch results for 'project' (%zu):\n", count);
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
    summary_update_content(update, "Complete project documentation and submit for review");
    summary_update_priority(update, SUMMARY_PRIORITY_CRITICAL);
    
    // Create new tags for update
    StringVec* new_tags = string_vec_new();
    string_vec_push(new_tags, "documentation");
    string_vec_push(new_tags, "review");
    string_vec_push(new_tags, "urgent");
    summary_update_tags(update, new_tags);
    
    // Apply the update
    if (summary_manager_update_summary(manager, summary1->id, update)) {
        printf("\nSuccessfully updated summary\n");
    } else {
        printf("\nFailed to update summary\n");
    }
    
    // Clean up update resources
    string_vec_free(new_tags);
    summary_update_free(update);

    // Get summary statistics
    SummaryStatistics* stats = summary_manager_get_summary_statistics(manager);
    printf("\nSummary Statistics:\n");
    printf("- Total summaries: %zu\n", stats->total_summaries);
    printf("- High priority summaries: %zu\n", stats->high_priority_summaries);
    printf("- Unique tags: %zu\n", stats->unique_tags);
    printf("- High priority percentage: %.2f%%\n", 
           summary_statistics_high_priority_percentage(stats));
    summary_statistics_free(stats);

    // Get all tags
    char** all_tags = summary_manager_get_all_tags(manager, &count);
    printf("\nAll tags (%zu):\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("- %s\n", all_tags[i]);
        free(all_tags[i]);  // Free each tag string
    }
    free(all_tags);

    // Get tag statistics
    StringStringMap* tag_stats = summary_manager_get_tag_statistics(manager);
    printf("\nTag Statistics:\n");
    for (size_t i = 0; i < tag_stats->size; i++) {
        printf("- %s: %s occurrences\n", tag_stats->keys[i], tag_stats->values[i]);
    }
    string_string_map_free(tag_stats);

    // Clean up summaries (they're managed by the manager)
    summary_free(summary1);
    summary_free(summary2);

    // Clean up the manager
    summary_manager_free(manager);

    return 0;
}
