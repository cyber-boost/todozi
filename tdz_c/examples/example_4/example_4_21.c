#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Include all the provided code here (reminder.c content)
// ... (all the code from reminder.c would be included here)

// Example 4: Comprehensive Reminder Management Demo
void example_reminder_management_demo() {
    printf("\n=== Example 4: Comprehensive Reminder Management Demo ===\n");
    
    // Create reminder manager
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Failed to create reminder manager\n");
        return;
    }
    
    // Create sample reminders
    Reminder* reminder1 = reminder_create();
    reminder1->content = string_duplicate("Complete project proposal");
    reminder1->remind_at = datetime_add_days(datetime_now(), 2);
    reminder1->priority = REMINDER_PRIORITY_HIGH;
    
    // Add tags to reminder1
    vector_push(reminder1->tags, string_duplicate("work"));
    vector_push(reminder1->tags, string_duplicate("project"));
    vector_push(reminder1->tags, string_duplicate("deadline"));
    
    char* id1 = reminder_manager_create_reminder(manager, reminder1);
    if (id1) {
        printf("Created reminder 1 with ID: %s\n", id1);
        free(id1);
    } else {
        printf("Failed to create reminder 1\n");
        reminder_destroy(reminder1);
    }
    
    Reminder* reminder2 = reminder_create();
    reminder2->content = string_duplicate("Buy groceries");
    reminder2->remind_at = datetime_add_days(datetime_now(), 1);
    reminder2->priority = REMINDER_PRIORITY_MEDIUM;
    
    // Add tags to reminder2
    vector_push(reminder2->tags, string_duplicate("personal"));
    vector_push(reminder2->tags, string_duplicate("shopping"));
    
    char* id2 = reminder_manager_create_reminder(manager, reminder2);
    if (id2) {
        printf("Created reminder 2 with ID: %s\n", id2);
        free(id2);
    } else {
        printf("Failed to create reminder 2\n");
        reminder_destroy(reminder2);
    }
    
    Reminder* reminder3 = reminder_create();
    reminder3->content = string_duplicate("Call dentist for appointment");
    reminder3->remind_at = datetime_add_days(datetime_now(), 3);
    reminder3->priority = REMINDER_PRIORITY_LOW;
    
    // Add tags to reminder3
    vector_push(reminder3->tags, string_duplicate("personal"));
    vector_push(reminder3->tags, string_duplicate("health"));
    
    char* id3 = reminder_manager_create_reminder(manager, reminder3);
    if (id3) {
        printf("Created reminder 3 with ID: %s\n", id3);
        free(id3);
    } else {
        printf("Failed to create reminder 3\n");
        reminder_destroy(reminder3);
    }
    
    // Retrieve and display all reminders
    printf("\n--- All Reminders ---\n");
    Vector* all_reminders = reminder_manager_get_all_reminders(manager);
    if (all_reminders) {
        for (size_t i = 0; i < all_reminders->size; i++) {
            Reminder* r = (Reminder*)vector_get(all_reminders, i);
            printf("ID: %s | Content: %s | Priority: %d | Status: %d\n", 
                   r->id, r->content, r->priority, r->status);
        }
        vector_destroy(all_reminders, NULL);
    }
    
    // Search for reminders containing "project"
    printf("\n--- Search Results for 'project' ---\n");
    Vector* search_results = reminder_manager_search_reminders(manager, "project");
    if (search_results) {
        for (size_t i = 0; i < search_results->size; i++) {
            Reminder* r = (Reminder*)vector_get(search_results, i);
            printf("Found: %s (ID: %s)\n", r->content, r->id);
        }
        vector_destroy(search_results, NULL);
    }
    
    // Get reminders by priority
    printf("\n--- High Priority Reminders ---\n");
    Vector* high_priority = reminder_manager_get_reminders_by_priority(manager, REMINDER_PRIORITY_HIGH);
    if (high_priority) {
        for (size_t i = 0; i < high_priority->size; i++) {
            Reminder* r = (Reminder*)vector_get(high_priority, i);
            printf("High Priority: %s\n", r->content);
        }
        vector_destroy(high_priority, NULL);
    }
    
    // Get reminders by tag
    printf("\n--- Reminders tagged 'personal' ---\n");
    Vector* personal_reminders = reminder_manager_get_reminders_by_tag(manager, "personal");
    if (personal_reminders) {
        for (size_t i = 0; i < personal_reminders->size; i++) {
            Reminder* r = (Reminder*)vector_get(personal_reminders, i);
            printf("Personal: %s\n", r->content);
        }
        vector_destroy(personal_reminders, NULL);
    }
    
    // Update a reminder
    printf("\n--- Updating Reminder ---\n");
    ReminderUpdate* update = reminder_update_create();
    reminder_update_content(update, "Complete project proposal and send to client");
    reminder_update_priority(update, REMINDER_PRIORITY_HIGH);
    
    // Create new tags vector for update
    Vector* new_tags = vector_create();
    vector_push(new_tags, string_duplicate("work"));
    vector_push(new_tags, string_duplicate("project"));
    vector_push(new_tags, string_duplicate("client"));
    vector_push(new_tags, string_duplicate("urgent"));
    reminder_update_tags(update, new_tags);
    
    if (reminder_manager_update_reminder(manager, reminder1->id, update)) {
        printf("Updated reminder %s successfully\n", reminder1->id);
    } else {
        printf("Failed to update reminder\n");
    }
    reminder_update_destroy(update);
    
    // Mark a reminder as completed
    if (reminder_manager_mark_reminder_completed(manager, reminder2->id)) {
        printf("Marked reminder %s as completed\n", reminder2->id);
    }
    
    // Get statistics
    printf("\n--- Reminder Statistics ---\n");
    ReminderStatistics stats = reminder_manager_get_reminder_statistics(manager);
    printf("Total reminders: %zu\n", stats.total_reminders);
    printf("Pending reminders: %zu (%.1f%%)\n", 
           stats.pending_reminders, reminder_statistics_pending_percentage(&stats));
    printf("Active reminders: %zu (%.1f%%)\n", 
           stats.active_reminders, reminder_statistics_active_percentage(&stats));
    printf("Overdue reminders: %zu (%.1f%%)\n", 
           stats.overdue_reminders, reminder_statistics_overdue_percentage(&stats));
    printf("Unique tags: %zu\n", stats.unique_tags);
    
    // Get all tags
    printf("\n--- All Tags ---\n");
    Vector* all_tags = reminder_manager_get_all_tags(manager);
    if (all_tags) {
        for (size_t i = 0; i < all_tags->size; i++) {
            char* tag = (char*)vector_get(all_tags, i);
            printf("%s ", tag);
        }
        printf("\n");
        vector_destroy(all_tags, free);
    }
    
    // Get tag statistics
    printf("\n--- Tag Statistics ---\n");
    HashMap* tag_stats = reminder_manager_get_tag_statistics(manager);
    if (tag_stats) {
        for (size_t i = 0; i < tag_stats->capacity; i++) {
            HashNode* node = tag_stats->buckets[i];
            while (node) {
                int* count = (int*)node->value;
                printf("%s: %d\n", node->key, *count);
                node = node->next;
            }
        }
        reminder_manager_free_tag_statistics(tag_stats);
    }
    
    // Get recent reminders
    printf("\n--- Recent Reminders ---\n");
    Vector* recent = reminder_manager_get_recent_reminders(manager, 2);
    if (recent) {
        for (size_t i = 0; i < recent->size; i++) {
            Reminder* r = (Reminder*)vector_get(recent, i);
            printf("Recent: %s\n", r->content);
        }
        vector_destroy(recent, NULL);
    }
    
    // Get reminders due soon (within 3 days)
    printf("\n--- Reminders Due Soon ---\n");
    Vector* due_soon = reminder_manager_get_reminders_due_soon(manager, 3);
    if (due_soon) {
        for (size_t i = 0; i < due_soon->size; i++) {
            Reminder* r = (Reminder*)vector_get(due_soon, i);
            printf("Due soon: %s\n", r->content);
        }
        vector_destroy(due_soon, NULL);
    }
    
    // Clean up
    reminder_manager_destroy(manager);
    printf("\nExample 4 completed successfully!\n");
}

int main() {
    example_reminder_management_demo();
    return 0;
}
