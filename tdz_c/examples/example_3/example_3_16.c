#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "summary.c" // Assuming all the above code is in summary.c

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
    printf("\n---\n");
}

int main() {
    // Create a summary manager
    SummaryManager* manager = summary_manager_new();
    if (!manager) {
        fprintf(stderr, "Failed to create summary manager\n");
        return 1;
    }

    // Create sample summaries
    Summary* summary1 = summary_new();
    summary1->content = string_clone("Prepare quarterly report");
    summary1->context = string_clone("Work");
    summary1->priority = SUMMARY_PRIORITY_HIGH;
    string_vec_push(&summary1->tags, "report");
    string_vec_push(&summary1->tags, "quarterly");

    Summary* summary2 = summary_new();
    summary2->content = string_clone("Buy groceries");
    summary2->context = string_clone("Personal");
    summary2->priority = SUMMARY_PRIORITY_MEDIUM;
    string_vec_push(&summary2->tags, "shopping");
    string_vec_push(&summary2->tags, "urgent");

    Summary* summary3 = summary_new();
    summary3->content = string_clone("Fix critical bug in login module");
    summary3->context = string_clone("Work");
    summary3->priority = SUMMARY_PRIORITY_CRITICAL;
    string_vec_push(&summary3->tags, "bug");
    string_vec_push(&summary3->tags, "login");

    // Add summaries to manager
    char* id1 = summary_manager_create_summary(manager, summary1);
    char* id2 = summary_manager_create_summary(manager, summary2);
    char* id3 = summary_manager_create_summary(manager, summary3);

    printf("Created summaries with IDs:\n%s\n%s\n%s\n\n", id1, id2, id3);

    // Get all summaries
    printf("All summaries:\n");
    size_t count;
    Summary** all_summaries = summary_manager_get_all_summaries(manager, &count);
    for (size_t i = 0; i < count; i++) {
        print_summary(all_summaries[i]);
    }
    free(all_summaries);

    // Search summaries
    printf("Search results for 'work':\n");
    Summary** search_results = summary_manager_search_summaries(manager, "work", &count);
    for (size_t i = 0; i < count; i++) {
        print_summary(search_results[i]);
    }
    free(search_results);

    // Get high priority summaries
    printf("High priority summaries:\n");
    Summary** high_priority = summary_manager_get_high_priority_summaries(manager, &count);
    for (size_t i = 0; i < count; i++) {
        print_summary(high_priority[i]);
    }
    free(high_priority);

    // Get summaries by tag
    printf("Summaries tagged with 'bug':\n");
    Summary** bug_summaries = summary_manager_get_summaries_by_tag(manager, "bug", &count);
    for (size_t i = 0; i < count; i++) {
        print_summary(bug_summaries[i]);
    }
    free(bug_summaries);

    // Get recent summaries (limit to 2)
    printf("2 most recent summaries:\n");
    Summary** recent = summary_manager_get_recent_summaries(manager, 2, &count);
    for (size_t i = 0; i < count; i++) {
        print_summary(recent[i]);
    }
    free(recent);

    // Update a summary
    SummaryUpdate* update = summary_update_new();
    summary_update_content(update, "Prepare and present quarterly report");
    summary_update_priority(update, SUMMARY_PRIORITY_CRITICAL);
    string_vec_push(&update->tags, "presentation");
    summary_manager_update_summary(manager, id1, update);
    summary_update_free(update);

    printf("Updated summary %s:\n", id1);
    Summary* updated = summary_manager_get_summary(manager, id1);
    print_summary(updated);

    // Get statistics
    SummaryStatistics* stats = summary_manager_get_summary_statistics(manager);
    printf("Summary Statistics:\n");
    printf("Total summaries: %zu\n", stats->total_summaries);
    printf("High priority summaries: %zu\n", stats->high_priority_summaries);
    printf("High priority percentage: %.2f%%\n", 
           summary_statistics_high_priority_percentage(stats));
    printf("Unique tags: %zu\n", stats->unique_tags);
    summary_statistics_free(stats);

    // Get all tags
    printf("\nAll tags:\n");
    char** tags = summary_manager_get_all_tags(manager, &count);
    for (size_t i = 0; i < count; i++) {
        printf("%s ", tags[i]);
        free(tags[i]);
    }
    free(tags);
    printf("\n");

    // Get tag statistics
    printf("\nTag statistics:\n");
    StringStringMap* tag_stats = summary_manager_get_tag_statistics(manager);
    for (size_t i = 0; i < tag_stats->size; i++) {
        printf("%s: %s\n", tag_stats->keys[i], tag_stats->values[i]);
    }
    string_string_map_free(tag_stats);

    // Clean up
    free(id1);
    free(id2);
    free(id3);
    summary_free(summary1);
    summary_free(summary2);
    summary_free(summary3);
    summary_manager_free(manager);

    return 0;
}
