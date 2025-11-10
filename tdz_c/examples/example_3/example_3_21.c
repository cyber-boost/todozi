// example3.c - Complete reminder management workflow
#include "reminder.c"  // Include the main implementation

void print_reminder_summary(Reminder* reminder) {
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
            printf("%s", (char*)vector_get(reminder->tags, i));
            if (i < reminder->tags->size - 1) printf(", ");
        }
        printf("\n");
    }
    printf("\n");
}

void print_statistics(ReminderManager* manager) {
    ReminderStatistics stats = reminder_manager_get_reminder_statistics(manager);
    printf("=== Reminder Statistics ===\n");
    printf("Total Reminders: %zu\n", stats.total_reminders);
    printf("Pending: %zu (%.1f%%)\n", stats.pending_reminders, 
           reminder_statistics_pending_percentage(&stats));
    printf("Active: %zu (%.1f%%)\n", stats.active_reminders,
           reminder_statistics_active_percentage(&stats));
    printf("Overdue: %zu (%.1f%%)\n", stats.overdue_reminders,
           reminder_statistics_overdue_percentage(&stats));
    printf("Unique Tags: %zu\n\n", stats.unique_tags);
}

int main() {
    // Create reminder manager
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Failed to create reminder manager\n");
        return 1;
    }

    printf("=== Reminder Management System Demo ===\n\n");

    // 1. Create reminders with different properties
    printf("1. Creating sample reminders...\n");
    
    // Create first reminder
    Reminder* reminder1 = reminder_create();
    reminder1->content = string_duplicate("Complete project proposal");
    reminder1->remind_at = datetime_add_days(datetime_now(), 3); // 3 days from now
    reminder1->priority = REMINDER_PRIORITY_HIGH;
    
    // Add tags
    vector_push(reminder1->tags, string_duplicate("work"));
    vector_push(reminder1->tags, string_duplicate("project"));
    vector_push(reminder1->tags, string_duplicate("urgent"));

    char* id1 = reminder_manager_create_reminder(manager, reminder1);
    printf("Created reminder with ID: %s\n", id1);

    // Create second reminder
    Reminder* reminder2 = reminder_create();
    reminder2->content = string_duplicate("Buy groceries");
    reminder2->remind_at = datetime_add_days(datetime_now(), 1); // Tomorrow
    reminder2->priority = REMINDER_PRIORITY_MEDIUM;
    
    vector_push(reminder2->tags, string_duplicate("personal"));
    vector_push(reminder2->tags, string_duplicate("shopping"));

    char* id2 = reminder_manager_create_reminder(manager, reminder2);
    printf("Created reminder with ID: %s\n", id2);

    // Create third reminder
    Reminder* reminder3 = reminder_create();
    reminder3->content = string_duplicate("Team meeting preparation");
    reminder3->remind_at = datetime_add_days(datetime_now(), -1); // Overdue
    reminder3->priority = REMINDER_PRIORITY_HIGH;
    reminder3->status = REMINDER_STATUS_ACTIVE;
    
    vector_push(reminder3->tags, string_duplicate("work"));
    vector_push(reminder3->tags, string_duplicate("meeting"));

    char* id3 = reminder_manager_create_reminder(manager, reminder3);
    printf("Created reminder with ID: %s\n", id3);

    printf("\n");

    // 2. Display all reminders
    printf("2. All reminders:\n");
    Vector* all_reminders = reminder_manager_get_all_reminders(manager);
    for (size_t i = 0; i < all_reminders->size; i++) {
        print_reminder_summary((Reminder*)vector_get(all_reminders, i));
    }
    vector_destroy(all_reminders, NULL);

    // 3. Show statistics
    print_statistics(manager);

    // 4. Search functionality
    printf("4. Searching for 'work' related reminders:\n");
    Vector* search_results = reminder_manager_search_reminders(manager, "work");
    for (size_t i = 0; i < search_results->size; i++) {
        print_reminder_summary((Reminder*)vector_get(search_results, i));
    }
    vector_destroy(search_results, NULL);

    // 5. Filter by priority
    printf("5. High priority reminders:\n");
    Vector* high_priority = reminder_manager_get_reminders_by_priority(manager, REMINDER_PRIORITY_HIGH);
    for (size_t i = 0; i < high_priority->size; i++) {
        print_reminder_summary((Reminder*)vector_get(high_priority, i));
    }
    vector_destroy(high_priority, NULL);

    // 6. Show overdue reminders
    printf("6. Overdue reminders:\n");
    Vector* overdue = reminder_manager_get_overdue_reminders(manager);
    for (size_t i = 0; i < overdue->size; i++) {
        print_reminder_summary((Reminder*)vector_get(overdue, i));
    }
    vector_destroy(overdue, NULL);

    // 7. Update a reminder
    printf("7. Updating reminder %s...\n", id1);
    ReminderUpdate* update = reminder_update_create();
    reminder_update_content(update, "Complete project proposal and submit for review");
    reminder_update_status(update, REMINDER_STATUS_ACTIVE);
    
    Vector* new_tags = vector_create();
    vector_push(new_tags, string_duplicate("work"));
    vector_push(new_tags, string_duplicate("project"));
    vector_push(new_tags, string_duplicate("urgent"));
    vector_push(new_tags, string_duplicate("review"));
    reminder_update_tags(update, new_tags);
    
    if (reminder_manager_update_reminder(manager, id1, update)) {
        printf("Reminder updated successfully\n");
        Reminder* updated = reminder_manager_get_reminder(manager, id1);
        print_reminder_summary(updated);
    } else {
        printf("Failed to update reminder\n");
    }
    reminder_update_destroy(update);

    // 8. Mark reminder as completed
    printf("8. Marking reminder %s as completed...\n", id2);
    if (reminder_manager_mark_reminder_completed(manager, id2)) {
        Reminder* completed = reminder_manager_get_reminder(manager, id2);
        print_reminder_summary(completed);
    }

    // 9. Show final statistics
    print_statistics(manager);

    // 10. Show all tags
    printf("10. All tags in system:\n");
    Vector* all_tags = reminder_manager_get_all_tags(manager);
    for (size_t i = 0; i < all_tags->size; i++) {
        printf("- %s\n", (char*)vector_get(all_tags, i));
    }
    vector_destroy(all_tags, free);
    printf("\n");

    // 11. Tag statistics
    printf("11. Tag usage statistics:\n");
    HashMap* tag_stats = reminder_manager_get_tag_statistics(manager);
    for (size_t i = 0; i < tag_stats->capacity; i++) {
        HashNode* node = tag_stats->buckets[i];
        while (node) {
            printf("- %s: %d reminders\n", node->key, *(int*)node->value);
            node = node->next;
        }
    }
    reminder_manager_free_tag_statistics(tag_stats);

    // Cleanup
    free(id1);
    free(id2);
    free(id3);
    reminder_manager_destroy(manager);
    
    return 0;
}
