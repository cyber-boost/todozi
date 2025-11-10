// example5.c - Complete Reminder Management Workflow
#include "reminder.c"  // Include the main implementation

void print_reminder_summary(Reminder* reminder) {
    if (!reminder) return;
    
    char time_str[100];
    strftime(time_str, sizeof(time_str), "%Y-%m-%d %H:%M", 
             localtime(&reminder->remind_at.timestamp));
    
    const char* priority_str[] = {"Low", "Medium", "High"};
    const char* status_str[] = {"Pending", "Active", "Completed", "Cancelled"};
    
    printf("ID: %.8s... | %s | %s | %s | ", 
           reminder->id, reminder->content, time_str, 
           priority_str[reminder->priority], status_str[reminder->status]);
    
    printf("Tags: ");
    for (size_t i = 0; i < reminder->tags->size; i++) {
        if (i > 0) printf(", ");
        printf("%s", (char*)vector_get(reminder->tags, i));
    }
    printf("\n");
}

void print_statistics(ReminderManager* manager) {
    ReminderStatistics stats = reminder_manager_get_reminder_statistics(manager);
    printf("\n=== Reminder Statistics ===\n");
    printf("Total: %zu\n", stats.total_reminders);
    printf("Pending: %zu (%.1f%%)\n", stats.pending_reminders, 
           reminder_statistics_pending_percentage(&stats));
    printf("Active: %zu (%.1f%%)\n", stats.active_reminders,
           reminder_statistics_active_percentage(&stats));
    printf("Overdue: %zu (%.1f%%)\n", stats.overdue_reminders,
           reminder_statistics_overdue_percentage(&stats));
    printf("Unique Tags: %zu\n", stats.unique_tags);
}

int main() {
    printf("=== Example 5: Complete Reminder Management Workflow ===\n\n");
    
    // 1. Create reminder manager
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Failed to create reminder manager\n");
        return 1;
    }
    
    // 2. Create sample reminders
    Reminder* reminder1 = reminder_create();
    reminder1->content = string_duplicate("Team meeting preparation");
    reminder1->remind_at = datetime_add_days(datetime_now(), 1);
    reminder1->priority = REMINDER_PRIORITY_HIGH;
    vector_push(reminder1->tags, string_duplicate("work"));
    vector_push(reminder1->tags, string_duplicate("meeting"));
    
    Reminder* reminder2 = reminder_create();
    reminder2->content = string_duplicate("Buy groceries");
    reminder2->remind_at = datetime_add_days(datetime_now(), -1); // Overdue
    reminder2->priority = REMINDER_PRIORITY_MEDIUM;
    vector_push(reminder2->tags, string_duplicate("personal"));
    vector_push(reminder2->tags, string_duplicate("shopping"));
    
    Reminder* reminder3 = reminder_create();
    reminder3->content = string_duplicate("Read new book");
    reminder3->remind_at = datetime_add_days(datetime_now(), 7);
    reminder3->priority = REMINDER_PRIORITY_LOW;
    vector_push(reminder3->tags, string_duplicate("personal"));
    vector_push(reminder3->tags, string_duplicate("reading"));
    
    // 3. Add reminders to manager
    char* id1 = reminder_manager_create_reminder(manager, reminder1);
    char* id2 = reminder_manager_create_reminder(manager, reminder2);
    char* id3 = reminder_manager_create_reminder(manager, reminder3);
    
    printf("Created reminders with IDs:\n");
    printf("1. %s\n", id1);
    printf("2. %s\n", id2);
    printf("3. %s\n\n", id3);
    
    // 4. Display all reminders
    printf("All Reminders:\n");
    Vector* all_reminders = reminder_manager_get_all_reminders(manager);
    for (size_t i = 0; i < all_reminders->size; i++) {
        print_reminder_summary((Reminder*)vector_get(all_reminders, i));
    }
    vector_destroy(all_reminders, NULL);
    
    // 5. Update a reminder
    printf("\nUpdating reminder 1...\n");
    ReminderUpdate* update = reminder_update_create();
    reminder_update_content(update, "Team meeting preparation (with agenda)");
    reminder_update_priority(update, REMINDER_PRIORITY_HIGH);
    reminder_manager_update_reminder(manager, id1, update);
    reminder_update_destroy(update);
    
    // 6. Activate and complete reminders
    printf("\nActivating reminder 1...\n");
    reminder_manager_activate_reminder(manager, id1);
    
    printf("Completing reminder 2...\n");
    reminder_manager_mark_reminder_completed(manager, id2);
    
    // 7. Search functionality
    printf("\nSearching for 'meeting'...\n");
    Vector* search_results = reminder_manager_search_reminders(manager, "meeting");
    for (size_t i = 0; i < search_results->size; i++) {
        print_reminder_summary((Reminder*)vector_get(search_results, i));
    }
    vector_destroy(search_results, NULL);
    
    // 8. Filter by criteria
    printf("\nHigh priority reminders:\n");
    Vector* high_priority = reminder_manager_get_reminders_by_priority(manager, REMINDER_PRIORITY_HIGH);
    for (size_t i = 0; i < high_priority->size; i++) {
        print_reminder_summary((Reminder*)vector_get(high_priority, i));
    }
    vector_destroy(high_priority, NULL);
    
    printf("\nPersonal tag reminders:\n");
    Vector* personal_reminders = reminder_manager_get_reminders_by_tag(manager, "personal");
    for (size_t i = 0; i < personal_reminders->size; i++) {
        print_reminder_summary((Reminder*)vector_get(personal_reminders, i));
    }
    vector_destroy(personal_reminders, NULL);
    
    // 9. Show overdue and due soon
    printf("\nOverdue reminders:\n");
    Vector* overdue = reminder_manager_get_overdue_reminders(manager);
    for (size_t i = 0; i < overdue->size; i++) {
        print_reminder_summary((Reminder*)vector_get(overdue, i));
    }
    vector_destroy(overdue, NULL);
    
    printf("\nDue in next 3 days:\n");
    Vector* due_soon = reminder_manager_get_reminders_due_soon(manager, 3);
    for (size_t i = 0; i < due_soon->size; i++) {
        print_reminder_summary((Reminder*)vector_get(due_soon, i));
    }
    vector_destroy(due_soon, NULL);
    
    // 10. Show statistics
    print_statistics(manager);
    
    // 11. Show tag statistics
    printf("\n=== Tag Statistics ===\n");
    HashMap* tag_stats = reminder_manager_get_tag_statistics(manager);
    for (size_t i = 0; i < tag_stats->capacity; i++) {
        HashNode* node = tag_stats->buckets[i];
        while (node) {
            printf("%s: %d reminders\n", node->key, *(int*)node->value);
            node = node->next;
        }
    }
    reminder_manager_free_tag_statistics(tag_stats);
    
    // 12. Clean up
    free(id1);
    free(id2);
    free(id3);
    reminder_manager_destroy(manager);
    
    printf("\nWorkflow completed successfully!\n");
    return 0;
}
