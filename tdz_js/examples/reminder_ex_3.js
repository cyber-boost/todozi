// Example 3: Smart Reminder System for Project Management
// File: smart_reminder_system.js


import { ReminderManager, ReminderPriority, ReminderStatus, parseReminderFormat } from './reminder.js';

class SmartReminderSystem {
    constructor() {
        this.reminderManager = new ReminderManager();
        this.projects = ['Development', 'Marketing', 'Sales', 'HR'];
    }

    // Initialize the system with sample reminders for different projects
    async initialize() {
        console.log('🚀 Initializing Smart Reminder System...');
        
        // Create development reminders
        await this.addRemindersFromFile('Development', [
            '<reminder>Complete code review for PR #1234; 2024-01-20T10:00:00Z; high; pending; review,development</reminder>',
            '<reminder>Deploy v2.1.0 to staging environment; 2024-01-21T14:00:00Z; critical; active; deployment,production</reminder>',
            '<reminder>Update dependencies in package.json; 2024-01-22T11:00:00Z; medium; pending; maintenance,dependencies</reminder>',
            '<reminder>Fix critical bug in payment gateway; 2024-01-20T08:30:00Z; critical; pending; bug,urgent,payments</reminder>'
        ]);

        // Create marketing reminders
        await this.addRemindersFromFile('Marketing', [
            '<reminder>Submit monthly blog post; 2024-01-20T09:00:00Z; medium; pending; content,marketing</reminder>',
            '<reminder>Schedule social media campaign; 2024-01-21T12:00:00Z; high; active; social,marketing,campaign</reminder>',
            '<reminder>Review Q1 marketing metrics; 2024-01-25T15:00:00Z; medium; pending; analytics,reporting</reminder>'
        ]);

        // Create sales reminders
        await this.addRemindersFromFile('Sales', [
            '<reminder>Call leads from webinar; 2024-01-20T11:00:00Z; high; active; leads,follow-up</reminder>',
            '<reminder>Prepare sales presentation for client ABC; 2024-01-22T13:00:00Z; critical; pending; client,presentation</reminder>',
            '<reminder>Review quarterly sales targets; 2024-01-30T16:00:00Z; medium; pending; review,targets</reminder>'
        ]);

        console.log('✅ System initialized with sample reminders\n');
    }

    async addRemindersFromFile(project, reminderTexts) {
        console.log(`📁 Adding ${reminderTexts.length} reminders for ${project} project:`);
        
        for (const text of reminderTexts) {
            try {
                const reminder = parseReminderFormat(text);
                reminder.project = project; // Add project context
                const id = await this.reminderManager.createReminder(reminder);
                console.log(`  ✓ Created: ${reminder.content.substring(0, 30)}...`);
            } catch (error) {
                console.log(`  ❌ Failed to parse: ${error.message}`);
            }
        }
        console.log('');
    }

    // Demonstrate different filtering and search capabilities
    async demonstrateSearchAndFilter() {
        console.log('🔍 Demonstrating Search and Filter Features');
        console.log('═════════════════════════════════════════════════\n');

        // 1. Get all overdue reminders
        console.log('1️⃣ Checking for Overdue Reminders:');
        const overdueReminders = this.reminderManager.getOverdueReminders();
        if (overdueReminders.length > 0) {
            console.log(`   Found ${overdueReminders.length} overdue reminder(s):`);
            overdueReminders.forEach(r => {
                console.log(`   ⚠️  ${r.content} (Priority: ${r.priority})`);
            });
        } else {
            console.log('   ✅ No overdue reminders found');
        }
        console.log('');

        // 2. Get reminders due in the next 24 hours
        console.log('2️⃣ Reminders Due in Next 24 Hours:');
        const dueSoon = this.reminderManager.getRemindersDueSoon(24 * 60 * 60 * 1000);
        if (dueSoon.length > 0) {
            console.log(`   Found ${dueSoon.length} reminder(s) due soon:`);
            dueSoon.forEach(r => {
                const hoursUntilDue = (new Date(r.remind_at) - new Date()) / (1000 * 60 * 60);
                console.log(`   ⏰ ${r.content} (Due in ${hoursUntilDue.toFixed(1)} hours)`);
            });
        } else {
            console.log('   📅 No reminders due in the next 24 hours');
        }
        console.log('');

        // 3. Search by keyword
        console.log('3️⃣ Search Results for "payment":');
        const searchResults = this.reminderManager.searchReminders('payment');
        searchResults.forEach(r => {
            console.log(`   💳 ${r.content} (Tags: ${r.tags.join(', ')})`);
        });
        console.log('');

        // 4. Filter by priority
        console.log('4️⃣ High Priority Reminders:');
        const highPriority = this.reminderManager.getRemindersByPriority(ReminderPriority.High);
        highPriority.forEach(r => {
            console.log(`   🔴 ${r.content} (${r.status})`);
        });
        console.log('');

        // 5. Filter by status
        console.log('5️⃣ Active Reminders:');
        const activeReminders = this.reminderManager.getActiveReminders();
        activeReminders.forEach(r => {
            console.log(`   🟢 ${r.content} (Due: ${r.remind_at})`);
        });
        console.log('');

        // 6. Filter by tag
        console.log('6️⃣ Development-related Reminders:');
        const devReminders = this.reminderManager.getRemindersByTag('development');
        devReminders.forEach(r => {
            console.log(`   💻 ${r.content} (Priority: ${r.priority})`);
        });
        console.log('');
    }

    // Demonstrate reminder management operations
    async demonstrateManagementOperations() {
        console.log('⚙️  Demonstrating Reminder Management');
        console.log('═════════════════════════════════════════════════\n');

        // 1. Create a new reminder dynamically
        console.log('1️⃣ Creating New Reminder:');
        const newReminder = {
            content: 'Update project documentation in Confluence',
            remind_at: new Date(Date.now() + 2 * 60 * 60 * 1000).toISOString(), // 2 hours from now
            priority: ReminderPriority.Medium,
            status: ReminderStatus.Pending,
            tags: ['documentation', 'confluence', 'update']
        };
        
        const reminderId = await this.reminderManager.createReminder(newReminder);
        console.log(`   ✅ Created reminder with ID: ${reminderId}`);
        console.log(`   📝 Content: ${newReminder.content}`);
        console.log('');

        // 2. Update a reminder
        console.log('2️⃣ Updating Reminder Priority:');
        await this.reminderManager.updateReminder(reminderId, { 
            priority: ReminderPriority.High,
            tags: [...newReminder.tags, 'urgent']
        });
        const updatedReminder = this.reminderManager.getReminder(reminderId);
        console.log(`   🔼 Updated priority to: ${updatedReminder.priority}`);
        console.log(`   🏷️  Updated tags: ${updatedReminder.tags.join(', ')}`);
        console.log('');

        // 3. Complete a reminder
        console.log('3️⃣ Marking Reminder as Completed:');
        await this.reminderManager.markReminderCompleted(reminderId);
        console.log(`   ✅ Reminder marked as completed`);
        console.log('');

        // 4. Cancel a reminder
        const testReminderId = await this.reminderManager.createReminder({
            content: 'Test reminder for cancellation',
            remind_at: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
            priority: ReminderPriority.Low,
            status: ReminderStatus.Pending,
            tags: ['test']
        });
        
        console.log('4️⃣ Cancelling Test Reminder:');
        await this.reminderManager.markReminderCancelled(testReminderId);
        console.log(`   ❌ Test reminder cancelled`);
        console.log('');

        // 5. Delete a reminder
        console.log('5️⃣ Deleting Cancelled Reminder:');
        await this.reminderManager.deleteReminder(testReminderId);
        console.log(`   🗑️  Test reminder deleted permanently`);
        console.log('');
    }

    // Display comprehensive statistics
    displayStatistics() {
        console.log('📊 Reminder System Statistics');
        console.log('═════════════════════════════════════════════════\n');

        const stats = this.reminderManager.getReminderStatistics();
        
        console.log('🔢 Overall Statistics:');
        console.log(`   Total reminders: ${stats.total_reminders}`);
        console.log(`   Pending: ${stats.pending_reminders} (${stats.pendingPercentage().toFixed(1)}%)`);
        console.log(`   Active: ${stats.active_reminders} (${stats.activePercentage().toFixed(1)}%)`);
        console.log(`   Overdue: ${stats.overdue_reminders} (${stats.overduePercentage().toFixed(1)}%)`);
        console.log(`   Unique tags: ${stats.unique_tags}`);
        console.log('');

        // Tag statistics
        const tagStats = this.reminderManager.getTagStatistics();
        console.log('🏷️  Tag Usage Statistics:');
        const sortedTags = Object.entries(tagStats)
            .sort((a, b) => b[1] - a[1])
            .slice(0, 5);
        sortedTags.forEach(([tag, count]) => {
            console.log(`   ${tag}: ${count} reminder(s)`);
        });
        console.log('');

        // Recent reminders
        console.log('⏰ 5 Most Recent Reminders:');
        const recent = this.reminderManager.getRecentReminders(5);
        recent.forEach((reminder, index) => {
            const date = new Date(reminder.created_at).toLocaleString();
            console.log(`   ${index + 1}. ${reminder.content} (${date})`);
        });
        console.log('');

        // All available tags
        const allTags = this.reminderManager.getAllTags();
        console.log('📋 All Available Tags:');
        console.log(`   ${allTags.join(', ')}`);
        console.log('');
    }

    // Practical use case: Daily reminder briefing
    generateDailyBriefing() {
        console.log('📅 Daily Reminder Briefing');
        console.log('═════════════════════════════════════════════════\n');
        console.log(`Generated: ${new Date().toLocaleString()}\n`);

        const overdue = this.reminderManager.getOverdueReminders();
        const dueSoon = this.reminderManager.getRemindersDueSoon(24 * 60 * 60 * 1000);
        const highPriority = this.reminderManager.getRemindersByPriority(ReminderPriority.Critical);

        console.log('🚨 URGENT ATTENTION REQUIRED:');
        if (overdue.length > 0 || highPriority.length > 0) {
            [...overdue, ...highPriority].forEach(r => {
                const icon = overdue.includes(r) ? '⚠️' : '🔴';
                console.log(`   ${icon} ${r.content}`);
            });
        } else {
            console.log('   ✅ No urgent items requiring immediate attention');
        }
        console.log('');

        console.log('⏰ DUE TODAY:');
        const today = new Date();
        today.setHours(23, 59, 59, 999);
        const todayReminders = this.reminderManager.getAllReminders().filter(r => {
            const remindDate = new Date(r.remind_at);
            return remindDate <= today && remindDate >= new Date(today.getFullYear(), today.getMonth(), today.getDate());
        });
        
        if (todayReminders.length > 0) {
            todayReminders.forEach(r => {
                const time = new Date(r.remind_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                const icon = r.priority === ReminderPriority.High ? '🔴' : 
                           r.priority === ReminderPriority.Medium ? '🟡' : '🟢';
                console.log(`   ${icon} ${time} - ${r.content}`);
            });
        } else {
            console.log('   📅 No reminders due today');
        }
        console.log('');

        console.log('📈 SUMMARY:');
        console.log(`   Total reminders in system: ${this.reminderManager.getAllReminders().length}`);
        console.log(`   Overdue: ${overdue.length}`);
        console.log(`   Due today: ${todayReminders.length}`);
        console.log(`   Due this week: ${dueSoon.length}`);
        console.log('');
    }

    // Run the complete demonstration
    async runDemo() {
        try {
            await this.initialize();
            await this.demonstrateSearchAndFilter();
            await this.demonstrateManagementOperations();
            this.displayStatistics();
            this.generateDailyBriefing();
            
            console.log('🎉 Smart Reminder System Demo Complete!');
            console.log('\n💡 Key Features Demonstrated:');
            console.log('   • Creating reminders from formatted text');
            console.log('   • Advanced filtering (priority, status, tags, dates)');
            console.log('   • Search functionality');
            console.log('   • CRUD operations');
            console.log('   • Statistical analysis');
            console.log('   • Daily briefing generation');
        } catch (error) {
            console.error('❌ Demo failed:', error.message);
        }
    }
}

// Run the demonstration
// Run if executed directly
    const system = new SmartReminderSystem();
    system.runDemo().catch(console.error);
}


/*
Based on the provided code, I'll generate a practical example that demonstrates how to use the ReminderManager class to build a smart reminder system for a project manager.

This example demonstrates a comprehensive use of the ReminderManager class by creating a smart reminder system for project management. The system showcases:

1. **Initialization**: Setting up reminders for multiple departments using the parseReminderFormat function
2. **Search & Filter**: Various ways to find reminders (overdue, due soon, by keyword, priority, status, and tags)
3. **Management Operations**: Creating, updating, completing, cancelling, and deleting reminders
4. **Statistics**: Generating comprehensive reports about reminder usage
5. **Practical Application**: A daily briefing feature that provides actionable insights

The example is practical because it addresses real-world needs:
- Department-based organization
- Priority-based task management
- Time-sensitive notifications
- Tag-based categorization
- Statistical reporting for project management

This demonstrates how the ReminderManager can be extended to create a full-featured reminder system that integrates with project workflows.
*/