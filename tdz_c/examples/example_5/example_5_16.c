#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "summary.c" // Include the provided implementation

void print_summary(Summary* summary) {
    if (!summary) return;
    
    printf("ID: %s\n", summary->id);
    printf("Content: %s\n", summary->content);
    printf("Context: %s\n", summary->context ? summary->context : "None");
    printf("Priority: %d\n", summary->priority);
    
    printf("Tags: ");
    for (size_t i = 0; i < summary->tags.size; i++) {
        printf("%s%s", summary->tags.data[i], (i < summary->tags.size - 1) ? ", " : "");
    }
    printf("\n\n");
}

int main() {
    // Create a summary manager
    SummaryManager* manager = summary_manager_new();
    if (!manager) {
        printf("Failed to create summary manager\n");
        return 1;
    }

    // Create sample summaries
    Summary* summary1 = summary_new();
    summary1->content = string_clone("Prepare quarterly report for management");
    summary1->context = string_clone("Work");
    summary1->priority = SUMMARY_PRIORITY_HIGH;
    string_vec_push(&summary1->tags, "report");
    string_vec_push(&summary1->tags, "management");
    string_vec_push(&summary1->tags, "quarterly");

    Summary* summary2 = summary_new();
    summary2->content = string_clone("Buy groceries for the week");
    summary2->context = string_clone("Personal");
    summary2->priority = SUMMARY_PRIORITY_MEDIUM;
    string_vec_push(&summary2->tags, "shopping");
    string_vec_push(&summary2->tags, "groceries");

    Summary* summary3 = summary_new();
    summary3->content = string_clone("Fix critical bug in payment system");
    summary3->context = string_clone("Work");
    summary3->priority = SUMMARY_PRIORITY_CRITICAL;
    string_vec_push(&summary3->tags, "bug");
    string_vec_push(&summary3->tags, "payment");
    string_vec_push(&summary3->tags, "urgent");

    // Add summaries to manager
    char* id1 = summary_manager_create_summary(manager, summary1);
    char* id2 = summary_manager_create_summary(manager, summary2);
    char* id3 = summary_manager_create_summary(manager, summary3);

    printf("Created summaries with IDs:\n%s\n%s\n%s\n\n", id1, id2, id3);

    // Get statistics
    SummaryStatistics* stats = summary_manager_get_summary_statistics(manager);
    if (stats) {
        printf("=== Summary Statistics ===\n");
        printf("Total summaries: %zu\n", stats->total_summaries);
        printf("High priority summaries: %zu\n", stats->high_priority_summaries);
        printf("High priority percentage: %.2f%%\n", 
               summary_statistics_high_priority_percentage(stats));
        printf("Unique tags: %zu\n\n", stats->unique_tags);
        summary_statistics_free(stats);
    }

    // Get tag statistics
    StringStringMap* tag_stats = summary_manager_get_tag_statistics(manager);
    if (tag_stats) {
        printf("=== Tag Statistics ===\n");
        for (size_t i = 0; i < tag_stats->size; i++) {
            printf("%s: %s occurrences\n", tag_stats->keys[i], tag_stats->values[i]);
        }
        printf("\n");
        string_string_map_free(tag_stats);
    }

    // Search for summaries
    printf("=== Search Results for 'work' ===\n");
    size_t search_count;
    Summary** search_results = summary_manager_search_summaries(manager, "work", &search_count);
    if (search_results) {
        for (size_t i = 0; i < search_count; i++) {
            print_summary(search_results[i]);
        }
        free(search_results);
    }

    // Get summaries by priority
    printf("=== High Priority Summaries ===\n");
    size_t high_priority_count;
    Summary** high_priority = summary_manager_get_high_priority_summaries(manager, &high_priority_count);
    if (high_priority) {
        for (size_t i = 0; i < high_priority_count; i++) {
            print_summary(high_priority[i]);
        }
        free(high_priority);
    }

    // Get summaries by tag
    printf("=== Summaries with 'urgent' tag ===\n");
    size_t urgent_count;
    Summary** urgent_summaries = summary_manager_get_summaries_by_tag(manager, "urgent", &urgent_count);
    if (urgent_summaries) {
        for (size_t i = 0; i < urgent_count; i++) {
            print_summary(urgent_summaries[i]);
        }
        free(urgent_summaries);
    }

    // Get recent summaries
    printf("=== 2 Most Recent Summaries ===\n");
    size_t recent_count;
    Summary** recent = summary_manager_get_recent_summaries(manager, 2, &recent_count);
    if (recent) {
        for (size_t i = 0; i < recent_count; i++) {
            print_summary(recent[i]);
        }
        free(recent);
    }

    // Update a summary
    SummaryUpdate* update = summary_update_new();
    summary_update_content(update, "Prepare quarterly report and presentation");
    summary_update_priority(update, SUMMARY_PRIORITY_CRITICAL);
    string_vec_push(&update->tags, "presentation");
    
    if (summary_manager_update_summary(manager, id1, update)) {
        printf("=== Updated Summary ===\n");
        Summary* updated = summary_manager_get_summary(manager, id1);
        print_summary(updated);
    }
    summary_update_free(update);

    // Cleanup
    free(id1);
    free(id2);
    free(id3);
    summary_free(summary1);
    summary_free(summary2);
    summary_free(summary3);
    summary_manager_free(manager);

    return 0;
}
