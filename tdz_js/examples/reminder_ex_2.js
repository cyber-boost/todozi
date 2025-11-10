// example2.js - Reminder Management Example


import { ReminderManager, ReminderPriority, ReminderStatus } from '../todozi/reminder.js';

async function runReminderExample() {
    // Create a new reminder manager
    const manager = new ReminderManager();
    
    // Create reminders
    const reminderId1 = await manager.createReminder({
        content: "Complete project proposal",
        remind_at: new Date(Date.now() + 86400000), // Tomorrow
        priority: ReminderPriority.High,
        tags: ["work", "project"]
    });
    
    const reminderId2 = await manager.createReminder({
        content: "Buy groceries",
        remind_at: new Date(Date.now() + 3600000), // In 1 hour
        priority: ReminderPriority.Medium,
        tags: ["personal", "shopping"]
    });
    
    const reminderId3 = await manager.createReminder({
        content: "Call dentist for appointment",
        remind_at: new Date(Date.now() - 3600000), // 1 hour ago (overdue)
        priority: ReminderPriority.Low,
        tags: ["personal", "health"]
    });
    
    console.log("Created reminders:");
    console.log(`- ${reminderId1}`);
    console.log(`- ${reminderId2}`);
    console.log(`- ${reminderId3}`);
    
    // Get specific reminder
    const reminder = manager.getReminder(reminderId1);
    console.log("\nFirst reminder details:");
    console.log(`Content: ${reminder.content}`);
    console.log(`Priority: ${reminder.priority}`);
    console.log(`Status: ${reminder.status}`);
    console.log(`Tags: ${reminder.tags.join(', ')}`);
    
    // Update a reminder
    await manager.updateReminder(reminderId2, {
        content: "Buy groceries and cleaning supplies",
        priority: ReminderPriority.High,
        tags: ["personal", "shopping", "cleaning"]
    });
    
    console.log("\nUpdated reminder content:", manager.getReminder(reminderId2).content);
    
    // Search reminders
    const searchResults = manager.searchReminders("groceries");
    console.log("\nSearch results for 'groceries':");
    searchResults.forEach(r => console.log(`- ${r.content}`));
    
    // Get reminders by priority
    const highPriorityReminders = manager.getRemindersByPriority(ReminderPriority.High);
    console.log("\nHigh priority reminders:");
    highPriorityReminders.forEach(r => console.log(`- ${r.content}`));
    
    // Get overdue reminders
    const overdueReminders = manager.getOverdueReminders();
    console.log("\nOverdue reminders:");
    overdueReminders.forEach(r => console.log(`- ${r.content} (was due: ${r.remind_at})`));
    
    // Get reminders by tag
    const personalReminders = manager.getRemindersByTag("personal");
    console.log("\nPersonal reminders:");
    personalReminders.forEach(r => console.log(`- ${r.content}`));
    
    // Get all tags
    const allTags = manager.getAllTags();
    console.log("\nAll tags:", allTags);
    
    // Get tag statistics
    const tagStats = manager.getTagStatistics();
    console.log("\nTag statistics:", tagStats);
    
    // Get reminder statistics
    const stats = manager.getReminderStatistics();
    console.log("\nReminder statistics:");
    console.log(`Total: ${stats.total_reminders}`);
    console.log(`Pending: ${stats.pending_reminders} (${stats.pendingPercentage().toFixed(1)}%)`);
    console.log(`Overdue: ${stats.overdue_reminders} (${stats.overduePercentage().toFixed(1)}%)`);
    
    // Mark reminder as completed
    await manager.markReminderCompleted(reminderId1);
    console.log("\nMarked first reminder as completed");
    console.log(`Status: ${manager.getReminder(reminderId1).status}`);
    
    // Delete a reminder
    await manager.deleteReminder(reminderId3);
    console.log("\nDeleted overdue reminder");
    console.log(`Total reminders now: ${manager.getAllReminders().length}`);
}

// Run the example
runReminderExample().catch(console.error);

/*
Here's a practical example demonstrating how to use the `ReminderManager` class from `reminder.js`:

This example demonstrates:

1. **Creating reminders** with different priorities, content, and tags
2. **Retrieving reminders** by ID
3. **Updating reminders** with new content and properties
4. **Searching reminders** by content
5. **Filtering reminders** by priority, tags, and overdue status
6. **Getting statistics** about reminders and tags
7. **Marking reminders** as completed
8. **Deleting reminders**

Key features shown:
- Using the `ReminderManager` class to manage reminders
- Working with reminder priorities and statuses
- Searching and filtering capabilities
- Statistical analysis of reminders
- Complete lifecycle management (create, update, complete, delete)

To run this example:
1. Save it as `example2.js`
2. Make sure `reminder.js` is in the same directory
3. Run with Node.js: `node example2.js`

The output will show:
- Created reminder IDs
- Details of a specific reminder
- Results of searches and filters
- Statistics about reminders and tags
- Status changes after updates and completions
*/