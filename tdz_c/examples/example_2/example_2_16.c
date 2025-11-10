// example2.c - Advanced Summary Management Example
#include <stdio.h>
#include <stdlib.h>
#include "summary.c"  // Include the main implementation

void print_summary(Summary* summary) {
    if (!summary) return;
    
    printf("ID: %s\n", summary->id ? summary->id : "N/A");
    printf("Content: %s\n", summary->content ? summary->content : "N/A");
    printf("Context: %s\n", summary->context ? summary->context : "N/A");
    printf("Priority: %d\n", summary->priority);
    printf("Tags: ");
    for (size_t i = 0; i < summary->tags.size; i++) {
        printf("%s%s", summary->tags.data[i], (i < summary->tags.size - 1) ? ", " : "");
    }
    printf("\n\n");
}

void print_summaries(const char* title, Summary** summaries, size_t count) {
    printf("=== %s (%zu found) ===\n", title, count);
    if (!summaries || count == 0) {
        printf("No summaries found.\n\n");
        return;
    }
    
    for (size_t i = 0; i < count; i++) {
        printf("%zu. ", i + 1);
        print_summary(summaries[i]);
    }
}

int main() {
    // Create a summary manager
    SummaryManager* manager = summary_manager_new();
    if (!manager) {
        printf("Failed to create summary manager\n");
        return 1;
    }
    
    printf("=== Summary Management System Example ===\n\n");
    
    // Create sample summaries
    Summary summary1;
    summary1.content = "Complete project documentation";
    summary1.context = "Work";
    summary1.priority = SUMMARY_PRIORITY_HIGH;
    string_vec_push(&summary1.tags, "documentation");
    string_vec_push(&summary1.tags, "project");
    
    Summary summary2;
    summary2.content = "Buy groceries for the week";
    summary2.context = "Personal";
    summary2.priority = SUMMARY_PRIORITY_MEDIUM;
    string_vec_push(&summary2.tags, "shopping");
    string_vec_push(&summary2.tags, "personal");
    
    Summary summary3;
    summary3.content = "Fix critical security vulnerability";
    summary3.context = "Work";
    summary3.priority = SUMMARY_PRIORITY_CRITICAL;
    string_vec_push(&summary3.tags, "security");
    string_vec_push(&summary3.tags, "urgent");
    string_vec_push(&summary3.tags, "project");
    
    Summary summary4;
    summary4.content = "Plan weekend hiking trip";
    summary4.context = "Personal";
    summary4.priority = SUMMARY_PRIORITY_LOW;
    string_vec_push(&summary4.tags, "outdoors");
    string_vec_push(&summary4.tags, "leisure");
    
    // Add summaries to manager
    char* id1 = summary_manager_create_summary(manager, &summary1);
    char* id2 = summary_manager_create_summary(manager, &summary2);
    char* id3 = summary_manager_create_summary(manager, &summary3);
    char* id4 = summary_manager_create_summary(manager, &summary4);
    
    printf("Created summaries with IDs:\n");
    printf("1. %s\n", id1 ? id1 : "Failed");
    printf("2. %s\n", id2 ? id2 : "Failed");
    printf("3. %s\n", id3 ? id3 : "Failed");
    printf("4. %s\n\n", id4 ? id4 : "Failed");
    
    // Retrieve and display all summaries
    size_t count;
    Summary** all_summaries = summary_manager_get_all_summaries(manager, &count);
    print_summaries("All Summaries", all_summaries, count);
    free(all_summaries);
    
    // Search for summaries containing "project"
    Summary** search_results = summary_manager_search_summaries(manager, "project", &count);
    print_summaries("Search: 'project'", search_results, count);
    free(search_results);
    
    // Get summaries by priority
    Summary** high_priority = summary_manager_get_summaries_by_priority(manager, SUMMARY_PRIORITY_HIGH, &count);
    print_summaries("High Priority Summaries", high_priority, count);
    free(high_priority);
    
    // Get summaries by tag
    Summary** tagged_summaries = summary_manager_get_summaries_by_tag(manager, "urgent", &count);
    print_summaries("Tag: 'urgent'", tagged_summaries, count);
    free(tagged_summaries);
    
    // Get recent summaries (limit to 2)
    Summary** recent = summary_manager_get_recent_summaries(manager, 2, &count);
    print_summaries("2 Most Recent Summaries", recent, count);
    free(recent);
    
    // Get high priority summaries
    Summary** critical = summary_manager_get_high_priority_summaries(manager, &count);
    print_summaries("High/Critical Priority Summaries", critical, count);
    free(critical);
    
    // Update a summary
    SummaryUpdate* update = summary_update_new();
    summary_update_content(update, "Complete project documentation and submit for review");
    summary_update_context(update, "Work - Q3 Goals");
    summary_update_priority(update, SUMMARY_PRIORITY_CRITICAL);
    
    StringVec new_tags;
    new_tags.data = NULL;
    new_tags.size = 0;
    new_tags.capacity = 0;
    string_vec_push(&new_tags, "documentation");
    string_vec_push(&new_tags, "review");
    string_vec_push(&new_tags, "important");
    summary_update_tags(update, &new_tags);
    
    if (summary_manager_update_summary(manager, id1, update)) {
        printf("Successfully updated summary %s\n", id1);
    } else {
        printf("Failed to update summary %s\n", id1);
    }
    
    summary_update_free(update);
    string_vec_free(&new_tags);
    
    // Show updated summary
    Summary* updated_summary = summary_manager_get_summary(manager, id1);
    printf("\n=== Updated Summary ===\n");
    print_summary(updated_summary);
    
    // Get statistics
    char** all_tags = summary_manager_get_all_tags(manager, &count);
    printf("=== All Tags (%zu found) ===\n", count);
    for (size_t i = 0; i < count; i++) {
        printf("%s ", all_tags[i]);
        free(all_tags[i]);
    }
    free(all_tags);
    printf("\n\n");
    
    // Tag statistics
    StringStringMap* tag_stats = summary_manager_get_tag_statistics(manager);
    printf("=== Tag Statistics ===\n");
    for (size_t i = 0; i < tag_stats->size; i++) {
        printf("%s: %s\n", tag_stats->keys[i], tag_stats->values[i]);
    }
    string_string_map_free(tag_stats);
    
    // Summary statistics
    SummaryStatistics* stats = summary_manager_get_summary_statistics(manager);
    printf("\n=== Summary Statistics ===\n");
    printf("Total summaries: %zu\n", stats->total_summaries);
    printf("High priority summaries: %zu\n", stats->high_priority_summaries);
    printf("Unique tags: %zu\n", stats->unique_tags);
    printf("High priority percentage: %.2f%%\n", 
           summary_statistics_high_priority_percentage(stats));
    summary_statistics_free(stats);
    
    // Clean up IDs
    if (id1) free(id1);
    if (id2) free(id2);
    if (id3) free(id3);
    if (id4) free(id4);
    
    // Clean up manager
    summary_manager_free(manager);
    
    return 0;
}
