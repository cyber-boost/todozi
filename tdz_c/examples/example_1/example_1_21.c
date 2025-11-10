#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "reminder.c"  // Include the provided code

// Helper function to print reminder details
void print_reminder(Reminder* reminder) {
    if (!reminder) return;
    
    printf("ID: %s\n", reminder->id ? reminder->id : "N/A");
    printf("Content: %s\n", reminder->content ? reminder->content : "N/A");
    printf("Priority: %s\n", 
           reminder->priority == REMINDER_PRIORITY_LOW ? "Low" :
           reminder->priority == REMINDER_PRIORITY_MEDIUM ? "Medium" : "High");
    printf("Status: %s\n", 
           reminder->status == REMINDER_STATUS_PENDING ? "Pending" :
           reminder->status == REMINDER_STATUS_ACTIVE ? "Active" :
           reminder->status == REMINDER_STATUS_COMPLETED ? "Completed" : "Cancelled");
    
    if (reminder->tags && reminder->tags->size > 0) {
        printf("Tags: ");
        for (size_t i = 0; i < reminder->tags->size; i++) {
            printf("%s%s", (char*)vector_get(reminder->tags, i), 
                   i < reminder->tags->size - 1 ? ", " : "");
        }
        printf("\n");
    }
    printf("\n");
}

// Function to demonstrate creating and managing reminders
void demonstrate_reminder_management() {
    printf("=== Reminder Management Demo ===\n\n");
    
    // Create a reminder manager
    ReminderManager* manager = reminder_manager_create();
    if (!manager) {
        printf("Failed to create reminder manager\n");
        return;
    }
    
    // Create a new reminder
    Reminder* reminder1 = reminder_create();
    if (!reminder1) {
        printf("Failed to create reminder\n");
        reminder_manager_destroy(manager);
        return;
    }
    
    // Set reminder properties
    reminder1->content = string_duplicate("Complete project proposal");
    reminder1->priority = REMINDER_PRIORITY_HIGH;
    reminder1->remind_at = datetime_add_days(datetime_now(), 3); // 3 days from now
    
    // Add tags
    vector_push(reminder1->tags, string_duplicate("work"));
    vector_push(reminder1->tags, string_duplicate("project"));
    vector_push(reminder1->tags, string_duplicate("urgent"));
    
    // Add the reminder to the manager
    char* id1 = reminder_manager_create_reminder(manager, reminder1);
    if (!id1) {
        printf("Failed to add reminder to manager\n");
        reminder_destroy(reminder1);
        reminder_manager_destroy(manager);
        return;
    }
    
    printf("Created reminder with ID: %s\n", id1);
    print_reminder(reminder1);
    
    // Create another reminder
    Reminder* reminder2 = reminder_create();
    if (reminder2) {
        reminder2->content = string_duplicate("Buy groceries");
        reminder2->priority = REMINDER_PRIORITY_MEDIUM;
        reminder2->remind_at = datetime_add_days(datetime_now(), 1); // Tomorrow
        vector_push(reminder2->tags, string_duplicate("personal"));
        vector_push(reminder2->tags, string_duplicate("shopping"));
        
        char* id2 = reminder_manager_create_reminder(manager, reminder2);
        if (id2) {
            printf("Created reminder with ID: %s\n", id2);
            print_reminder(reminder2);
            free(id2);
        } else {
            printf("Failed to add second reminder\n");
            reminder_destroy(reminder2);
        }
    }
    
    // Retrieve and display all reminders
    printf("All reminders:\n");
    Vector* all_reminders = reminder_manager_get_all_reminders(manager);
    if (all_reminders) {
        for (size_t i = 0; i < all_reminders->size; i++) {
            Reminder* r = (Reminder*)vector_get(all_reminders, i);
            print_reminder(r);
        }
        vector_destroy(all_reminders, NULL); // Don't destroy the reminders themselves
    }
    
    // Update the first reminder
    ReminderUpdate* update = reminder_update_create();
    if (update) {
        reminder_update_content(update, "Complete project proposal and send to client");
        reminder_update_priority(update, REMINDER_PRIORITY_HIGH);
        
        if (reminder_manager_update_reminder(manager, id1, update)) {
            printf("Updated reminder %s:\n", id1);
            Reminder* updated = reminder_manager_get_reminder(manager, id1);
            print_reminder(updated);
        } else {
            printf("Failed to update reminder\n");
        }
        reminder_update_destroy(update);
    }
    
    // Search for reminders
    printf("Searching for 'project'...\n");
    Vector* search_results = reminder_manager_search_reminders(manager, "project");
    if (search_results) {
        for (size_t i = 0; i < search_results->size; i++) {
            Reminder* r = (Reminder*)vector_get(search_results, i);
            print_reminder(r);
        }
        vector_destroy(search_results, NULL);
    }
    
    // Get reminders by tag
    printf("Reminders tagged 'work':\n");
    Vector* work_reminders = reminder_manager_get_reminders_by_tag(manager, "work");
    if (work_reminders) {
        for (size_t i = 0; i < work_reminders->size; i++) {
            Reminder* r = (Reminder*)vector_get(work_reminders, i);
            print_reminder(r);
        }
        vector_destroy(work_reminders, NULL);
    }
    
    // Mark reminder as completed
    if (reminder_manager_mark_reminder_completed(manager, id1)) {
        printf("Marked reminder %s as completed\n", id1);
        Reminder* completed = reminder_manager_get_reminder(manager, id1);
        print_reminder(completed);
    }
    
    // Get statistics
    printf("Reminder Statistics:\n");
    ReminderStatistics stats = reminder_manager_get_reminder_statistics(manager);
    printf("Total: %zu\n", stats.total_reminders);
    printf("Pending: %zu (%.1f%%)\n", stats.pending_reminders, 
           reminder_statistics_pending_percentage(&stats));
    printf("Active: %zu (%.1f%%)\n", stats.active_reminders,
           reminder_statistics_active_percentage(&stats));
    printf("Overdue: %zu (%.1f%%)\n", stats.overdue_reminders,
           reminder_statistics_overdue_percentage(&stats));
    printf("Unique Tags: %zu\n", stats.unique_tags);
    
    // Get all tags
    printf("\nAll Tags:\n");
    Vector* all_tags = reminder_manager_get_all_tags(manager);
    if (all_tags) {
        for (size_t i = 0; i < all_tags->size; i++) {
            printf("- %s\n", (char*)vector_get(all_tags, i));
        }
        vector_destroy(all_tags, free);
    }
    
    // Cleanup
    free(id1);
    reminder_manager_destroy(manager);
}

int main() {
    demonstrate_reminder_management();
    return 0;
}
