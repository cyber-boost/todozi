// example_usage.c - Demonstrates practical usage of the reminder system

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "reminder.c"  // Include the main reminder system

// Helper function to print reminder details
void print_reminder(Reminder* reminder) {
    if (!reminder) return;
    
    const char* priority_str[] = {"Low", "Medium", "High"};
    const char* status_str[] = {"Pending", "Active", "Completed", "Cancelled"};
    
    printf("ID: %s\n", reminder->id);
    printf("Content: %s\n", reminder->content);
    printf("Priority: %s\n", priority_str[reminder->priority]);
    printf("Status: %s\n", status_str[reminder->status]);
    
    if (reminder->tags && reminder->tags->size > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < reminder->tags->size; i++) {
            if (i > 0) printf(", ");
            printf("%s", (char*)vector_get(reminder->tags, i));
        }
        printf("\n");
    }
    printf("\n");
}

// Helper function to print all reminders in a vector
void print_reminders_list(const char* title, Vector* reminders) {
    printf("=== %s ===\n", title);
    if (!reminders || reminders->size == 0) {
        printf("No reminders found.\n\n");
        return;
    }
    
    for (size_t i = 0; i < reminders->size; i++) {
        Reminder* reminder = (Reminder*)vector_get(reminders, i);
        print_reminder(reminder);
    }
}

int main() {
    printf("=== Reminder Manager Demo ===\n\n");
    
    // Create a reminder manager
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Failed to create reminder manager\n");
        return 1;
    }
    
    // Create some reminders
    printf("1. Creating reminders...\n");
    
    // Reminder 1: High priority work task
    Reminder* reminder1 = reminder_create();
    reminder1->content = string_duplicate("Complete quarterly report");
    reminder1->priority = REMINDER_PRIORITY_HIGH;
    reminder1->remind_at = datetime_add_days(datetime_now(), 3); // Due in 3 days
    
    // Add tags
    vector_push(reminder1->tags, string_duplicate("work"));
    vector_push(reminder1->tags, string_duplicate("report"));
    vector_push(reminder1->tags, string_duplicate("urgent"));
    
    char* id1 = reminder_manager_create_reminder(manager, reminder1);
    printf("Created reminder with ID: %s\n", id1);
    
    // Reminder 2: Medium priority personal task
    Reminder* reminder2 = reminder_create();
    reminder2->content = string_duplicate("Buy groceries");
    reminder2->priority = REMINDER_PRIORITY_MEDIUM;
    reminder2->remind_at = datetime_add_days(datetime_now(), 1); // Due tomorrow
    
    // Add tags
    vector_push(reminder2->tags, string_duplicate("personal"));
    vector_push(reminder2->tags, string_duplicate("shopping"));
    
    char* id2 = reminder_manager_create_reminder(manager, reminder2);
    printf("Created reminder with ID: %s\n", id2);
    
    // Reminder 3: Low priority hobby task
    Reminder* reminder3 = reminder_create();
    reminder3->content = string_duplicate("Practice guitar");
    reminder3->priority = REMINDER_PRIORITY_LOW;
    reminder3->remind_at = datetime_add_days(datetime_now(), 7); // Due in a week
    
    // Add tags
    vector_push(reminder3->tags, string_duplicate("hobby"));
    vector_push(reminder3->tags, string_duplicate("music"));
    
    char* id3 = reminder_manager_create_reminder(manager, reminder3);
    printf("Created reminder with ID: %s\n", id3);
    
    // Activate the first reminder
    reminder_manager_activate_reminder(manager, id1);
    
    printf("\n");
    
    // 2. Retrieve and display all reminders
    Vector* all_reminders = reminder_manager_get_all_reminders(manager);
    print_reminders_list("All Reminders", all_reminders);
    vector_destroy(all_reminders, NULL);
    
    // 3. Filter reminders by priority
    Vector* high_priority = reminder_manager_get_reminders_by_priority(manager, REMINDER_PRIORITY_HIGH);
    print_reminders_list("High Priority Reminders", high_priority);
    vector_destroy(high_priority, NULL);
    
    // 4. Filter reminders by tag
    Vector* work_reminders = reminder_manager_get_reminders_by_tag(manager, "work");
    print_reminders_list("Work Reminders", work_reminders);
    vector_destroy(work_reminders, NULL);
    
    // 5. Search reminders
    Vector* search_results = reminder_manager_search_reminders(manager, "groceries");
    print_reminders_list("Search Results for 'groceries'", search_results);
    vector_destroy(search_results, NULL);
    
    // 6. Update a reminder
    printf("6. Updating reminder...\n");
    ReminderUpdate* update = reminder_update_create();
    reminder_update_content(update, "Buy groceries and household items");
    reminder_update_priority(update, REMINDER_PRIORITY_HIGH);
    
    // Create new tags vector
    Vector* new_tags = vector_create();
    vector_push(new_tags, string_duplicate("personal"));
    vector_push(new_tags, string_duplicate("shopping"));
    vector_push(new_tags, string_duplicate("urgent"));
    reminder_update_tags(update, new_tags);
    
    if (reminder_manager_update_reminder(manager, id2, update)) {
        printf("Successfully updated reminder %s\n", id2);
    }
    
    reminder_update_destroy(update);
    
    // Show updated reminder
    Reminder* updated_reminder = reminder_manager_get_reminder(manager, id2);
    printf("\nUpdated reminder:\n");
    print_reminder(updated_reminder);
    
    // 7. Get statistics
    printf("7. Reminder Statistics:\n");
    ReminderStatistics stats = reminder_manager_get_reminder_statistics(manager);
    printf("Total reminders: %zu\n", stats.total_reminders);
    printf("Pending reminders: %zu (%.1f%%)\n", 
           stats.pending_reminders, 
           reminder_statistics_pending_percentage(&stats));
    printf("Active reminders: %zu (%.1f%%)\n", 
           stats.active_reminders, 
           reminder_statistics_active_percentage(&stats));
    printf("Overdue reminders: %zu (%.1f%%)\n", 
           stats.overdue_reminders, 
           reminder_statistics_overdue_percentage(&stats));
    printf("Unique tags: %zu\n\n", stats.unique_tags);
    
    // 8. Get recent reminders
    Vector* recent = reminder_manager_get_recent_reminders(manager, 2);
    print_reminders_list("2 Most Recent Reminders", recent);
    vector_destroy(recent, NULL);
    
    // 9. Mark reminder as completed
    printf("9. Marking reminder as completed...\n");
    if (reminder_manager_mark_reminder_completed(manager, id1)) {
        printf("Reminder %s marked as completed\n", id1);
        Reminder* completed = reminder_manager_get_reminder(manager, id1);
        print_reminder(completed);
    }
    
    // 10. Show all tags
    printf("10. All Tags:\n");
    Vector* all_tags = reminder_manager_get_all_tags(manager);
    if (all_tags) {
        for (size_t i = 0; i < all_tags->size; i++) {
            printf("- %s\n", (char*)vector_get(all_tags, i));
        }
        vector_destroy(all_tags, free);
    }
    printf("\n");
    
    // 11. Tag statistics
    printf("11. Tag Statistics:\n");
    HashMap* tag_stats = reminder_manager_get_tag_statistics(manager);
    if (tag_stats) {
        for (size_t i = 0; i < tag_stats->capacity; i++) {
            HashNode* node = tag_stats->buckets[i];
            while (node) {
                printf("- %s: %d reminders\n", node->key, *(int*)node->value);
                node = node->next;
            }
        }
        reminder_manager_free_tag_statistics(tag_stats);
    }
    printf("\n");
    
    // Clean up
    free(id1);
    free(id2);
    free(id3);
    reminder_manager_destroy(manager);
    
    printf("Demo completed successfully!\n");
    return 0;
}
