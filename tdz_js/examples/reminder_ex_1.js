// reminder-app.js

import { ReminderManager, ReminderPriority, ReminderStatus, parseReminderFormat } from '../todozi/reminder.js';

class ReminderApp {
    constructor() {
        this.manager = new ReminderManager();
        this.setupEventListeners();
        this.loadSampleData();
    }

    setupEventListeners() {
        // Simulated event listeners for a real application
        process.on('SIGINT', () => this.handleShutdown());
        setInterval(() => this.checkDueReminders(), 60000); // Check every minute
    }

    async loadSampleData() {
        try {
            // Create some sample reminders
            await this.manager.createReminder({
                content: "Team meeting with engineering department",
                remind_at: new Date(Date.now() + 2 * 60 * 60 * 1000), // 2 hours from now
                priority: ReminderPriority.High,
                status: ReminderStatus.Pending,
                tags: ["meeting", "engineering", "work"]
            });

            await this.manager.createReminder({
                content: "Submit quarterly report to management",
                remind_at: new Date(Date.now() + 24 * 60 * 60 * 1000), // 24 hours from now
                priority: ReminderPriority.Critical,
                status: ReminderStatus.Pending,
                tags: ["report", "management", "deadline"]
            });

            await this.manager.createReminder({
                content: "Follow up with John about project proposal",
                remind_at: new Date(Date.now() - 30 * 60 * 1000), // 30 minutes ago (overdue)
                priority: ReminderPriority.Medium,
                status: ReminderStatus.Pending,
                tags: ["follow-up", "john", "proposal"]
            });

            console.log("✅ Sample reminders loaded successfully!");
        } catch (error) {
            console.error("❌ Failed to load sample data:", error.message);
        }
    }

    async addReminderFromText(reminderText) {
        try {
            const reminder = parseReminderFormat(reminderText);
            const reminderId = await this.manager.createReminder(reminder);
            console.log(`📝 Reminder created with ID: ${reminderId}`);
            return reminderId;
        } catch (error) {
            console.error("❌ Failed to parse reminder format:", error.message);
            throw error;
        }
    }

    async checkDueReminders() {
        try {
            const now = new Date();
            const dueSoon = this.manager.getRemindersDueSoon(30 * 60 * 1000); // Due in next 30 minutes
            const overdue = this.manager.getOverdueReminders();

            if (dueSoon.length > 0) {
                console.log("\n🔔 REMINDERS DUE SOON:");
                dueSoon.forEach(reminder => {
                    console.log(`   ⏰ ${reminder.content} (Priority: ${reminder.priority})`);
                });
            }

            if (overdue.length > 0) {
                console.log("\n⚠️  OVERDUE REMINDERS:");
                overdue.forEach(reminder => {
                    console.log(`   ❗ ${reminder.content} (Was due: ${reminder.remind_at.toLocaleString()})`);
                });
            }

            return { dueSoon, overdue };
        } catch (error) {
            console.error("❌ Error checking due reminders:", error.message);
            return { dueSoon: [], overdue: [] };
        }
    }

    async searchReminders(query) {
        try {
            const results = this.manager.searchReminders(query);
            console.log(`\n🔍 Search results for "${query}":`);
            results.forEach(reminder => {
                console.log(`   📋 ${reminder.content}`);
                console.log(`      Tags: ${reminder.tags.join(', ')}`);
                console.log(`      Due: ${reminder.remind_at.toLocaleString()}`);
                console.log(`      Status: ${reminder.status}`);
                console.log('   ---');
            });
            return results;
        } catch (error) {
            console.error("❌ Search failed:", error.message);
            return [];
        }
    }

    async getReminderStats() {
        try {
            const stats = this.manager.getReminderStatistics();
            const tagStats = this.manager.getTagStatistics();
            
            console.log("\n📊 REMINDER STATISTICS:");
            console.log(`   Total reminders: ${stats.total_reminders}`);
            console.log(`   Pending: ${stats.pending_reminders} (${stats.pendingPercentage().toFixed(1)}%)`);
            console.log(`   Active: ${stats.active_reminders} (${stats.activePercentage().toFixed(1)}%)`);
            console.log(`   Overdue: ${stats.overdue_reminders} (${stats.overduePercentage().toFixed(1)}%)`);
            console.log(`   Unique tags: ${stats.unique_tags}`);
            
            console.log("\n🏷️  TAG STATISTICS:");
            Object.entries(tagStats).forEach(([tag, count]) => {
                console.log(`   ${tag}: ${count} reminders`);
            });

            return { reminderStats: stats, tagStats };
        } catch (error) {
            console.error("❌ Error getting statistics:", error.message);
            return null;
        }
    }

    async updateReminderStatus(reminderId, newStatus) {
        try {
            const update = ReminderUpdate.new().status(newStatus);
            await this.manager.updateReminder(reminderId, update);
            console.log(`✅ Reminder ${reminderId} status updated to: ${newStatus}`);
        } catch (error) {
            console.error(`❌ Failed to update reminder ${reminderId}:`, error.message);
            throw error;
        }
    }

    handleShutdown() {
        console.log("\n📤 Saving reminder data before shutdown...");
        this.getReminderStats(); // Show final stats
        console.log("👋 Goodbye!");
        process.exit(0);
    }
}

// Usage Example
async function demoReminderApp() {
    const app = new ReminderApp();

    // Wait a moment for sample data to load
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Demo: Add a reminder using text format
    console.log("\n=== ADDING REMINDER FROM TEXT ===");
    const reminderText = "<reminder>Call client about new requirements; 2024-01-15T14:30:00Z; high; pending; client,call,urgent</reminder>";
    await app.addReminderFromText(reminderText);

    // Demo: Check due reminders
    console.log("\n=== CHECKING DUE REMINDERS ===");
    await app.checkDueReminders();

    // Demo: Search reminders
    console.log("\n=== SEARCHING REMINDERS ===");
    await app.searchReminders("meeting");
    await app.searchReminders("report");

    // Demo: Get statistics
    console.log("\n=== GETTING STATISTICS ===");
    await app.getReminderStats();

    // Simulate some time passing and check again
    console.log("\n=== SIMULATING TIME PASSING ===");
    await new Promise(resolve => setTimeout(resolve, 2000));
    await app.checkDueReminders();
}

// Run the demo if this file is executed directly
// Run if executed directly
    demoReminderApp().catch(console.error);
}


/*
# Example: Reminder System Application

Here's a practical example of using the ReminderManager class to build a simple reminder system application:

## Key Features Demonstrated:

1. **Text-based Reminder Creation**: Uses `parseReminderFormat()` to create reminders from structured text
2. **Automatic Monitoring**: Regularly checks for due and overdue reminders
3. **Search Functionality**: Full-text search across reminder content and tags
4. **Statistics Tracking**: Shows comprehensive statistics about reminders and tag usage
5. **Status Management**: Easy updating of reminder statuses

## Sample Output:

/ *
✅ Sample reminders loaded successfully!

=== ADDING REMINDER FROM TEXT ===
📝 Reminder created with ID: abc123-456-def

=== CHECKING DUE REMINDERS ===
⚠️  OVERDUE REMINDERS:
   ❗ Follow up with John about project proposal (Was due: 1/14/2024, 2:30:00 PM)

=== SEARCHING REMINDERS ===
🔍 Search results for "meeting":
   📋 Team meeting with engineering department
      Tags: meeting, engineering, work
      Due: 1/14/2024, 4:30:00 PM
      Status: pending
   ---

=== GETTING STATISTICS ===
📊 REMINDER STATISTICS:
   Total reminders: 4
   Pending: 3 (75.0%)
   Active: 0 (0.0%)
   Overdue: 1 (25.0%)
   Unique tags: 8
*/