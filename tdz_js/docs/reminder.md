# Reminder Manager System - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Code Analysis](#code-analysis)
4. [Classes and Methods](#classes-and-methods)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Reminder Manager System is a comprehensive JavaScript application designed to manage reminders with features including creation, updating, deletion, searching, and categorization. It provides a robust foundation for reminder management with support for priorities, statuses, tags, and time-based operations.

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────────────┐
│                        Reminder Manager System                      │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐ │
│  │  ReminderManager│    │ ReminderUpdate  │    │ReminderStatistics│ │
│  │                 │    │                 │    │                 │ │
│  │ - reminders     │    │ - content       │    │ - total_reminders│ │
│  │ - reminder_tags │    │ - remind_at     │    │ - pending_reminders││
│  │                 │    │ - priority      │    │ - active_reminders││
│  │ + createReminder│    │ - status        │    │ - overdue_reminders││
│  │ + getReminder   │    │ - tags          │    │ - unique_tags    │ │
│  │ + updateReminder│    │                 │    │                 │ │
│  │ + deleteReminder│    │ + content()     │    │ + pendingPercentage()│
│  │ + searchReminders│   │ + remindAt()    │    │ + activePercentage()│
│  │ + ...           │    │ + priority()    │    │ + overduePercentage()│
│  └─────────────────┘    │ + status()      │    └─────────────────┘ │
│                         │ + tags()        │                        │
│                         └─────────────────┘                        │
│                                                                    │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐ │
│  │ parseReminderFormat│  │ ReminderTests   │    │ Helper Functions│ │
│  │                 │    │                 │    │                 │ │
│  │ + Parses text   │    │ + Unit tests    │    │ + parseReminderPriority│
│  │   format to     │    │                 │    │ + parseReminderStatus│
│  │   reminder obj  │    │                 │    │                 │ │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

## Code Analysis

### Core Components

1. **ReminderManager**: Main class responsible for managing all reminder operations
2. **ReminderUpdate**: Builder pattern implementation for updating reminders
3. **ReminderStatistics**: Statistical analysis of reminders
4. **parseReminderFormat**: Utility function for parsing text-based reminders
5. **Helper Functions**: Support functions for parsing priorities and statuses
6. **Enums**: Constants for reminder priorities and statuses

### Key Features
- UUID-based unique identifier generation
- Full CRUD operations for reminders
- Tag-based categorization system
- Priority and status management
- Time-based filtering (overdue, due soon)
- Statistical analysis capabilities
- Text-based reminder parsing

## Classes and Methods

### ReminderManager Class

#### Constructor
```javascript
constructor()
```
Initializes the ReminderManager with empty Maps for reminders and reminder_tags.

#### Methods

##### `createReminder(reminder)`
Creates a new reminder with a unique ID and timestamps.

**Parameters:**
- `reminder` (Object): Reminder object containing content, remind_at, priority, status, and tags

**Returns:**
- `Promise<string>`: UUID of the created reminder

**Example:**
```javascript
const reminderId = await manager.createReminder({
    content: "Meeting with team",
    remind_at: new Date("2024-12-31T10:00:00Z"),
    priority: ReminderPriority.High,
    status: ReminderStatus.Pending,
    tags: ["meeting", "work"]
});
```

##### `getReminder(reminderId)`
Retrieves a specific reminder by its ID.

**Parameters:**
- `reminderId` (string): UUID of the reminder

**Returns:**
- `Object|null`: Reminder object or null if not found

##### `getAllReminders()`
Retrieves all reminders.

**Returns:**
- `Array<Object>`: Array of all reminder objects

##### `updateReminder(reminderId, updates)`
Updates an existing reminder with new values.

**Parameters:**
- `reminderId` (string): UUID of the reminder to update
- `updates` (Object): Object containing fields to update

**Returns:**
- `Promise<void>`

**Throws:**
- `Error`: If reminder is not found

##### `deleteReminder(reminderId)`
Deletes a reminder by its ID.

**Parameters:**
- `reminderId` (string): UUID of the reminder to delete

**Returns:**
- `Promise<void>`

**Throws:**
- `Error`: If reminder is not found

##### `searchReminders(query)`
Searches reminders by content or tags.

**Parameters:**
- `query` (string): Search query string

**Returns:**
- `Array<Object>`: Array of matching reminders

##### `getRemindersByPriority(priority)`
Filters reminders by priority level.

**Parameters:**
- `priority` (string): Priority level (low, medium, high)

**Returns:**
- `Array<Object>`: Array of reminders with specified priority

##### `getRemindersByStatus(status)`
Filters reminders by status.

**Parameters:**
- `status` (string): Status value (pending, active, completed, cancelled)

**Returns:**
- `Array<Object>`: Array of reminders with specified status

##### `getRemindersByTag(tag)`
Filters reminders by tag.

**Parameters:**
- `tag` (string): Tag to filter by

**Returns:**
- `Array<Object>`: Array of reminders containing the specified tag

##### `getPendingReminders()`
Retrieves all pending reminders.

**Returns:**
- `Array<Object>`: Array of pending reminders

##### `getActiveReminders()`
Retrieves all active reminders.

**Returns:**
- `Array<Object>`: Array of active reminders

##### `getOverdueReminders()`
Retrieves all overdue reminders (past remind_at time with pending/active status).

**Returns:**
- `Array<Object>`: Array of overdue reminders

##### `getRemindersDueSoon(duration)`
Retrieves reminders due within a specified time duration.

**Parameters:**
- `duration` (number): Duration in milliseconds

**Returns:**
- `Array<Object>`: Array of reminders due soon

##### `getRecentReminders(limit)`
Retrieves most recently created reminders.

**Parameters:**
- `limit` (number): Maximum number of reminders to return

**Returns:**
- `Array<Object>`: Array of recent reminders

##### `getAllTags()`
Retrieves all unique tags across all reminders.

**Returns:**
- `Array<string>`: Array of unique tags

##### `getTagStatistics()`
Calculates frequency of each tag.

**Returns:**
- `Object`: Object with tags as keys and frequencies as values

##### `getReminderStatistics()`
Generates comprehensive statistics about reminders.

**Returns:**
- `ReminderStatistics`: Statistics object

##### `markReminderCompleted(reminderId)`
Marks a reminder as completed.

**Parameters:**
- `reminderId` (string): UUID of the reminder

**Returns:**
- `Promise<void>`

##### `markReminderCancelled(reminderId)`
Marks a reminder as cancelled.

**Parameters:**
- `reminderId` (string): UUID of the reminder

**Returns:**
- `Promise<void>`

##### `activateReminder(reminderId)`
Activates a reminder.

**Parameters:**
- `reminderId` (string): UUID of the reminder

**Returns:**
- `Promise<void>`

### ReminderUpdate Class

#### Constructor
```javascript
constructor()
```
Initializes a new ReminderUpdate builder with null values.

#### Static Methods

##### `new()`
Creates a new ReminderUpdate instance.

**Returns:**
- `ReminderUpdate`: New builder instance

#### Methods

##### `content(content)`
Sets the content field.

**Parameters:**
- `content` (string): Reminder content

**Returns:**
- `ReminderUpdate`: Current instance for chaining

##### `remindAt(remindAt)`
Sets the remind_at field.

**Parameters:**
- `remindAt` (Date): Reminder date/time

**Returns:**
- `ReminderUpdate`: Current instance for chaining

##### `priority(priority)`
Sets the priority field.

**Parameters:**
- `priority` (string): Priority level

**Returns:**
- `ReminderUpdate`: Current instance for chaining

##### `status(status)`
Sets the status field.

**Parameters:**
- `status` (string): Reminder status

**Returns:**
- `ReminderUpdate`: Current instance for chaining

##### `tags(tags)`
Sets the tags field.

**Parameters:**
- `tags` (Array<string>): Array of tags

**Returns:**
- `ReminderUpdate`: Current instance for chaining

### ReminderStatistics Class

#### Constructor
```javascript
constructor(data)
```
Initializes statistics with provided data.

**Parameters:**
- `data` (Object): Statistics data object

#### Methods

##### `pendingPercentage()`
Calculates percentage of pending reminders.

**Returns:**
- `number`: Percentage value

##### `activePercentage()`
Calculates percentage of active reminders.

**Returns:**
- `number`: Percentage value

##### `overduePercentage()`
Calculates percentage of overdue reminders.

**Returns:**
- `number`: Percentage value

### parseReminderFormat Function

Parses a text-based reminder format into a reminder object.

**Parameters:**
- `reminderText` (string): Text in format `<reminder>content; remind_at; priority; status; tags</reminder>`

**Returns:**
- `Object`: Parsed reminder object

**Throws:**
- `Error`: If format is invalid

## Usage Examples

### Basic Reminder Management

```javascript
const { ReminderManager, ReminderPriority, ReminderStatus } = require('./reminder-manager');

// Initialize manager
const manager = new ReminderManager();

// Create a reminder
const reminderId = await manager.createReminder({
    content: "Complete project documentation",
    remind_at: new Date("2024-12-15T14:00:00Z"),
    priority: ReminderPriority.High,
    status: ReminderStatus.Pending,
    tags: ["documentation", "project", "urgent"]
});

console.log(`Created reminder with ID: ${reminderId}`);

// Retrieve the reminder
const reminder = manager.getReminder(reminderId);
console.log("Retrieved reminder:", reminder);

// Update the reminder
await manager.updateReminder(reminderId, {
    content: "Complete project documentation and review",
    priority: ReminderPriority.Medium
});

// Search reminders
const searchResults = manager.searchReminders("documentation");
console.log("Search results:", searchResults);

// Get all reminders
const allReminders = manager.getAllReminders();
console.log(`Total reminders: ${allReminders.length}`);
```

### Using ReminderUpdate Builder

```javascript
const { ReminderUpdate, ReminderPriority } = require('./reminder-manager');

// Using the builder pattern
const update = ReminderUpdate.new()
    .content("Updated reminder content")
    .remindAt(new Date("2024-12-20T09:00:00Z"))
    .priority(ReminderPriority.High)
    .tags(["updated", "important"]);

// Apply the update
await manager.updateReminder(reminderId, update);
```

### Statistical Analysis

```javascript
// Get reminder statistics
const stats = manager.getReminderStatistics();
console.log(`Total reminders: ${stats.total_reminders}`);
console.log(`Pending percentage: ${stats.pendingPercentage().toFixed(2)}%`);
console.log(`Active percentage: ${stats.activePercentage().toFixed(2)}%`);

// Get tag statistics
const tagStats = manager.getTagStatistics();
console.log("Tag frequencies:", tagStats);

// Get all unique tags
const allTags = manager.getAllTags();
console.log("All tags:", allTags);
```

### Time-based Operations

```javascript
// Get overdue reminders
const overdue = manager.getOverdueReminders();
console.log("Overdue reminders:", overdue);

// Get reminders due within 24 hours
const dueSoon = manager.getRemindersDueSoon(24 * 60 * 60 * 1000); // 24 hours in ms
console.log("Due soon:", dueSoon);

// Get recent reminders (last 5)
const recent = manager.getRecentReminders(5);
console.log("Recent reminders:", recent);
```

### Text-based Reminder Parsing

```javascript
const { parseReminderFormat } = require('./reminder-manager');

// Parse a text-based reminder
const reminderText = "<reminder>Review quarterly reports; 2024-12-31T15:00:00Z; high; pending; reports,quarterly,review</reminder>";

try {
    const reminder = parseReminderFormat(reminderText);
    console.log("Parsed reminder:", reminder);
    
    // Add to manager
    const reminderId = await manager.createReminder(reminder);
    console.log(`Added parsed reminder with ID: ${reminderId}`);
} catch (error) {
    console.error("Failed to parse reminder:", error.message);
}
```

### Filtering Operations

```javascript
// Filter by priority
const highPriorityReminders = manager.getRemindersByPriority(ReminderPriority.High);
console.log("High priority reminders:", highPriorityReminders);

// Filter by status
const activeReminders = manager.getActiveReminders();
console.log("Active reminders:", activeReminders);

// Filter by tag
const projectReminders = manager.getRemindersByTag("project");
console.log("Project reminders:", projectReminders);
```

## Design Patterns

### 1. Builder Pattern
The `ReminderUpdate` class implements the Builder pattern to provide a fluent interface for constructing update objects.

**Benefits:**
- Improves code readability
- Enables method chaining
- Provides type safety through explicit methods

### 2. Singleton Pattern (Implicit)
The `ReminderManager` class acts as a central repository for all reminder operations, following singleton-like principles within its scope.

### 3. Factory Pattern
The `parseReminderFormat` function acts as a factory method that creates reminder objects from text input.

### 4. Strategy Pattern
Different filtering methods (`getRemindersByPriority`, `getRemindersByStatus`, etc.) implement different strategies for retrieving reminders.

## Performance Analysis

### Time Complexity

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| createReminder | O(1) | Hash map insertion |
| getReminder | O(1) | Hash map lookup |
| updateReminder | O(1) | Hash map update |
| deleteReminder | O(1) | Hash map deletion |
| getAllReminders | O(n) | Where n is number of reminders |
| searchReminders | O(n) | Linear search through all reminders |
| getRemindersBy* | O(n) | Filter operations |
| getTagStatistics | O(n×m) | Where m is average tags per reminder |

### Space Complexity
- **Reminders Storage**: O(n) where n is the number of reminders
- **Tag Index**: O(n×m) where m is average number of tags per reminder
- **Total**: O(n×m) space complexity

### Performance Optimization Recommendations

1. **Indexing**: For large datasets, consider adding indexes for frequently queried fields
2. **Pagination**: Implement pagination for large result sets
3. **Caching**: Cache frequently accessed statistics
4. **Batch Operations**: Add batch creation/update methods for bulk operations

### Memory Usage
The system uses JavaScript Maps for storage, which are memory-efficient for key-value pairs. However, for applications with thousands of reminders, consider:

```javascript
// Example of memory optimization for large datasets
class OptimizedReminderManager extends ReminderManager {
    constructor(options = {}) {
        super();
        this.maxReminders = options.maxReminders || 10000;
        this.autoCleanup = options.autoCleanup || false;
    }
    
    async createReminder(reminder) {
        if (this.reminders.size >= this.maxReminders && this.autoCleanup) {
            this.cleanupOldReminders();
        }
        return super.createReminder(reminder);
    }
    
    cleanupOldReminders() {
        // Remove completed/cancelled reminders older than 30 days
        const cutoffDate = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000);
        for (const [id, reminder] of this.reminders.entries()) {
            if ((reminder.status === ReminderStatus.Completed || 
                 reminder.status === ReminderStatus.Cancelled) &&
                reminder.updated_at < cutoffDate) {
                this.reminders.delete(id);
                this.reminder_tags.delete(id);
            }
        }
    }
}
```

## Security Considerations

### 1. Input Validation
The system includes basic validation but should be enhanced for production use:

```javascript
// Enhanced validation example
class SecureReminderManager extends ReminderManager {
    validateReminder(reminder) {
        if (!reminder.content || typeof reminder.content !== 'string') {
            throw new Error('Invalid reminder content');
        }
        
        if (!(reminder.remind_at instanceof Date) || isNaN(reminder.remind_at.getTime())) {
            throw new Error('Invalid reminder date');
        }
        
        if (reminder.content.length > 1000) {
            throw new Error('Reminder content too long');
        }
        
        if (Array.isArray(reminder.tags)) {
            if (reminder.tags.length > 50) {
                throw new Error('Too many tags');
            }
            reminder.tags = reminder.tags.filter(tag => 
                typeof tag === 'string' && tag.length <= 50
            );
        }
    }
    
    async createReminder(reminder) {
        this.validateReminder(reminder);
        return super.createReminder(reminder);
    }
}
```

### 2. Data Sanitization
Implement sanitization for user inputs:

```javascript
function sanitizeInput(input) {
    if (typeof input !== 'string') return '';
    return input
        .replace(/[<>]/g, '') // Remove HTML tags
        .trim()
        .substring(0, 1000); // Limit length
}
```

### 3. Access Control
For multi-user environments, implement access control:

```javascript
class MultiUserReminderManager extends ReminderManager {
    constructor() {
        super();
        this.userReminders = new Map(); // userId -> Set of reminderIds
    }
    
    async createReminder(reminder, userId) {
        const reminderId = await super.createReminder(reminder);
        
        // Associate reminder with user
        if (!this.userReminders.has(userId)) {
            this.userReminders.set(userId, new Set());
        }
        this.userReminders.get(userId).add(reminderId);
        
        return reminderId;
    }
    
    getUserReminders(userId) {
        const userReminderIds = this.userReminders.get(userId) || new Set();
        return Array.from(userReminderIds).map(id => this.getReminder(id));
    }
}
```

### 4. Rate Limiting
Implement rate limiting for API endpoints:

```javascript
class RateLimitedReminderManager extends ReminderManager {
    constructor() {
        super();
        this.requestCounts = new Map();
        this.rateLimit = 100; // requests per minute
        this.timeWindow = 60000; // 1 minute in ms
    }
    
    checkRateLimit(userId) {
        const now = Date.now();
        const userRequests = this.requestCounts.get(userId) || [];
        
        // Remove old requests
        const recentRequests = userRequests.filter(time => now - time < this.timeWindow);
        
        if (recentRequests.length >= this.rateLimit) {
            throw new Error('Rate limit exceeded');
        }
        
        recentRequests.push(now);
        this.requestCounts.set(userId, recentRequests);
    }
}
```

## Testing Strategies

### Unit Testing

The system includes basic test cases in the `ReminderTests` class. Here's an expanded testing approach:

```javascript
// Enhanced test suite
class ComprehensiveReminderTests {
    static async runAllTests() {
        console.log('Running comprehensive tests...');
        
        try {
            await this.testBasicOperations();
            await this.testUpdateOperations();
            await this.testSearchOperations();
            await this.testFilterOperations();
            await this.testStatistics();
            await this.testEdgeCases();
            await this.testErrorHandling();
            
            console.log('All tests passed!');
        } catch (error) {
            console.error('Test failed:', error.message);
            throw error;
        }
    }
    
    static async testBasicOperations() {
        const manager = new ReminderManager();
        
        // Test creation
        const reminderId = await manager.createReminder({
            content: "Test reminder",
            remind_at: new Date("2025-01-01T10:00:00Z"),
            priority: ReminderPriority.Medium,
            status: ReminderStatus.Pending,
            tags: ["test"]
        });
        
        console.assert(reminderId, "Reminder creation should return ID");
        
        // Test retrieval
        const reminder = manager.getReminder(reminderId);
        console.assert(reminder, "Should retrieve created reminder");
        console.assert(reminder.content === "Test reminder", "Content should match");
        
        // Test deletion
        await manager.deleteReminder(reminderId);
        const deletedReminder = manager.getReminder(reminderId);
        console.assert(deletedReminder === undefined, "Reminder should be deleted");
    }
    
    static async testUpdateOperations() {
        const manager = new ReminderManager();
        
        const reminderId = await manager.createReminder({
            content: "Original content",
            remind_at: new Date("2025-01-01T10:00:00Z"),
            priority: ReminderPriority.Low,
            status: ReminderStatus.Pending,
            tags: ["original"]
        });
        
        // Test partial update
        await manager.updateReminder(reminderId, {
            content: "Updated content",
            priority: ReminderPriority.High
        });
        
        const updatedReminder = manager.getReminder(reminderId);
        console.assert(updatedReminder.content === "Updated content", "Content should be updated");
        console.assert(updatedReminder.priority === ReminderPriority.High, "Priority should be updated");
        console.assert(updatedReminder.status === ReminderStatus.Pending, "Unchanged fields should remain");
    }
    
    static async testSearchOperations() {
        const manager = new ReminderManager();
        
        await manager.createReminder({
            content: "Meeting with John",
            remind_at: new Date("2025-01-01T10:00:00Z"),
            priority: ReminderPriority.Medium,
            status: ReminderStatus.Pending,
            tags: ["meeting", "john"]
        });
        
        await manager.createReminder({
            content: "Project deadline",
            remind_at: new Date("2025-01-02T10:00:00Z"),
            priority: ReminderPriority.High,
            status: ReminderStatus.Pending,
            tags: ["project", "deadline"]
        });
        
        // Test content search
        const contentResults = manager.searchReminders("Meeting");
        console.assert(contentResults.length === 1, "Should find one reminder by content");
        
        // Test tag search
        const tagResults = manager.searchReminders("project");
        console.assert(tagResults.length === 1, "Should find one reminder by tag");
    }
    
    static async testFilterOperations() {
        const manager = new ReminderManager();
        
        // Create test data
        await manager.createReminder({
            content: "High priority pending",
            remind_at: new Date("2025-01-01T10:00:00Z"),
            priority: ReminderPriority.High,
            status: ReminderStatus.Pending,
            tags: ["high"]
        });
        
        await manager.createReminder({
            content: "Medium priority active",
            remind_at: new Date("2025-01-02T10:00:00Z"),
            priority: ReminderPriority.Medium,
            status: ReminderStatus.Active,
            tags: ["medium"]
        });
        
        // Test priority filtering
        const highPriority = manager.getRemindersByPriority(ReminderPriority.High);
        console.assert(highPriority.length === 1, "Should find one high priority reminder");
        
        // Test status filtering
        const activeReminders = manager.getActiveReminders();
        console.assert(activeReminders.length === 1, "Should find one active reminder");
        
        // Test tag filtering
        const highTagReminders = manager.getRemindersByTag("high");
        console.assert(highTagReminders.length === 1, "Should find one reminder with 'high' tag");
    }
    
    static async testStatistics() {
        const manager = new ReminderManager();
        
        // Create test data
        await manager.createReminder({
            content: "Pending reminder",
            remind_at: new Date("2025-01-01T10:00:00Z"),
            priority: ReminderPriority.High,
            status: ReminderStatus.Pending,
            tags: ["test1"]
        });
        
        await manager.createReminder({
            content: "Active reminder",
            remind_at: new Date("2025-01-02T10:00:00Z"),
            priority: ReminderPriority.Medium,
            status: ReminderStatus.Active,
            tags: ["test2"]
        });
        
        // Test statistics
        const stats = manager.getReminderStatistics();
        console.assert(stats.total_reminders === 2, "Should have 2 total reminders");
        console.assert(stats.pending_reminders === 1, "Should have 1 pending reminder");
        console.assert(stats.active_reminders === 1, "Should have 1 active reminder");
        
        // Test tag statistics
        const tagStats = manager.getTagStatistics();
        console.assert(Object.keys(tagStats).length === 2, "Should have 2 unique tags");
        console.assert(tagStats.test1 === 1, "Tag 'test1' should appear once");
        console.assert(tagStats.test2 === 1, "Tag 'test2' should appear once");
    }
    
    static async testEdgeCases() {
        const manager = new ReminderManager();
        
        // Test empty manager
        const emptyStats = manager.getReminderStatistics();
        console.assert(emptyStats.total_reminders === 0, "Empty manager should have 0 reminders");
        
        // Test getAllTags with no reminders
        const noTags = manager.getAllTags();
        console.assert(Array.isArray(noTags) && noTags.length === 0, "Should return empty array for no tags");
        
        // Test search with no results
        const noResults = manager.searchReminders("nonexistent");
        console.assert(Array.isArray(noResults) && noResults.length === 0, "Should return empty array for no matches");
    }
    
    static async testErrorHandling() {
        const manager = new ReminderManager();
        
        // Test getting non-existent reminder
        const nonExistent = manager.getReminder("non-existent-id");
        console.assert(nonExistent === undefined, "Should return undefined for non-existent reminder");
        
        // Test updating non-existent reminder
        try {
            await manager.updateReminder("non-existent-id", { content: "test" });
            console.assert(false, "Should throw error for non-existent reminder update");
        } catch (error) {
            console.assert(error.message.includes("not found"), "Should throw 'not found' error");
        }
        
        // Test deleting non-existent reminder
        try {
            await manager.deleteReminder("non-existent-id");
            console.assert(false, "Should throw error for non-existent reminder deletion");
        } catch (error) {
            console.assert(error.message.includes("not found"), "Should throw 'not found' error");
        }
    }
}
```

### Integration Testing

```javascript
// Integration test example
async function integrationTest() {
    const manager = new ReminderManager();
    
    console.log('Starting integration test...');
    
    // Create multiple reminders
    const reminderIds = [];
    for (let i = 0; i < 10; i++) {
        const id = await manager.createReminder({
            content: `Reminder ${i}`,
            remind_at: new Date(Date.now() + (i * 3600000)), // 1 hour apart
            priority: i % 3 === 0 ? ReminderPriority.High : 
                     i % 3 === 1 ? ReminderPriority.Medium : ReminderPriority.Low,
            status: i % 2 === 0 ? ReminderStatus.Pending : ReminderStatus.Active,
            tags: [`tag${i % 3}`, `common`]
        });
        reminderIds.push(id);
    }
    
    console.log(`Created ${reminderIds.length} reminders`);
    
    // Test various operations
    const allReminders = manager.getAllReminders();
    console.assert(allReminders.length === 10, "Should have 10 reminders");
    
    const highPriority = manager.getRemindersByPriority(ReminderPriority.High);
    console.assert(highPriority.length === 4, "Should have 4 high priority reminders");
    
    const commonTag = manager.getRemindersByTag("common");
    console.assert(commonTag.length === 10, "All reminders should have 'common' tag");
    
    // Test updates
    await manager.updateReminder(reminderIds[0], {
        content: "Updated first reminder",
        priority: ReminderPriority.High
    });
    
    const updatedReminder = manager.getReminder(reminderIds[0]);
    console.assert(updatedReminder.content === "Updated first reminder", "Reminder should be updated");
    
    // Test statistics
    const stats = manager.getReminderStatistics();
    console.assert(stats.total_reminders === 10, "Statistics should show 10 total reminders");
    
    console.log('Integration test completed successfully!');
}
```

### Performance Testing

```javascript
// Performance test
async function performanceTest() {
    const manager = new ReminderManager();
    const testSize = 1000;
    
    console.log(`Starting performance test with ${testSize} reminders...`);
    
    // Measure creation time
    const startTime = Date.now();
    const reminderIds = [];
    
    for (let i = 0; i < testSize; i++) {
        const id = await manager.createReminder({
            content: `Performance test reminder ${i}`,
            remind_at: new Date(Date.now() + (i * 60000)),
            priority: ReminderPriority.Medium,
            status: ReminderStatus.Pending,
            tags: [`perf${i % 10}`]
        });
        reminderIds.push(id);
    }
    
    const creationTime = Date.now() - startTime;
    console.log(`Created ${testSize} reminders in ${creationTime}ms (${(testSize/creationTime*1000).toFixed(2)} reminders/sec)`);
    
    // Measure search time
    const searchStartTime = Date.now();
    const searchResults = manager.searchReminders("Performance test");
    const searchTime = Date.now() - searchStartTime;
    console.log(`Searched ${testSize} reminders in ${searchTime}ms (found ${searchResults.length})`);
    
    // Measure statistics time
    const statsStartTime = Date.now();
    const stats = manager.getReminderStatistics();
    const statsTime = Date.now() - statsStartTime;
    console.log(`Generated statistics in ${statsTime}ms`);
    
    console.log('Performance test completed!');
}
```

## Deployment Instructions

### 1. Environment Setup

```bash
# Install Node.js (version 14 or higher recommended)
# Download from https://nodejs.org/

# Verify installation
node --version
npm --version
```

### 2. Project Setup

```bash
# Create project directory
mkdir reminder-manager
cd reminder-manager

# Initialize npm package
npm init -y

# Install dependencies
npm install uuid

# Create the main file
touch reminder-manager.js

# Add the provided code to reminder-manager.js
```

### 3. Package Configuration

Update `package.json`:

```json
{
  "name": "reminder-manager",
  "version": "1.0.0",
  "description": "A comprehensive reminder management system",
  "main": "reminder-manager.js",
  "scripts": {
    "start": "node reminder-manager.js",
    "test": "node -e 'require(\"./reminder-manager.js\").ReminderTests.testReminderManagerCreation()' && node -e 'require(\"./reminder-manager.js\").ReminderTests.testReminderUpdateBuilder()'",
    "test-all": "node -e 'require(\"./reminder-manager.js\").ComprehensiveReminderTests.runAllTests()'"
  },
  "dependencies": {
    "uuid": "^9.0.0"
  },
  "keywords": ["reminder", "management", "task", "todo"],
  "author": "Your Name",
  "license": "MIT"
}
```

### 4. Usage as Module

```javascript
// In your application
const { 
    ReminderManager, 
    ReminderUpdate, 
    ReminderStatistics,
    ReminderPriority,
    ReminderStatus
} = require('./reminder-manager');

// Initialize and use
const manager = new ReminderManager();
```

### 5. Docker Deployment (Optional)

Create `Dockerfile`:

```dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

EXPOSE 3000

CMD ["npm", "start"]
```

Build and run:

```bash
# Build image
docker build -t reminder-manager .

# Run container
docker run -p 3000:3000 reminder-manager
```

### 6. Environment Variables

For production deployment, consider using environment variables:

```javascript
// config.js
module.exports = {
    MAX_REMINDERS: process.env.MAX_REMINDERS || 10000,
    CLEANUP_INTERVAL: process.env.CLEANUP_INTERVAL || 3600000, // 1 hour
    RATE_LIMIT: process.env.RATE_LIMIT || 100
};
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. UUID Module Not Found
**Error:** `Error: Cannot find module 'uuid'`

**Solution:**
```bash
npm install uuid
```

#### 2. Invalid Date Format
**Error:** `Invalid reminder date format`

**Solution:** Ensure dates are in valid ISO format:
```javascript
// Correct formats
new Date("2024-12-31T23:59:59Z")
new Date("2024-12-31")
new Date(Date.now() + 3600000) // 1 hour from now
```

#### 3. Missing Reminder Tags
**Issue:** Tags not being stored properly

**Solution:** Ensure tags are provided as arrays:
```javascript
// Correct
tags: ["work", "urgent"]

// Incorrect
tags: "work,urgent" // This won't work
```

#### 4. Performance Issues with Large Datasets
**Issue:** Slow operations with many reminders

**Solutions:**
1. Implement pagination for large result sets
2. Add database indexing
3. Use more efficient data structures
4. Implement caching for frequently accessed data

#### 5. Memory Leaks
**Issue:** Application consuming increasing memory over time

**Solutions:**
1. Implement automatic cleanup of old completed reminders
2. Use weak references where appropriate
3. Monitor memory usage with Node.js profiling tools

### Debugging Techniques

#### 1. Enable Logging
```javascript
class DebugReminderManager extends ReminderManager {
    async createReminder(reminder) {
        console.log('Creating reminder:', reminder);
        const result = await super.createReminder(reminder);
        console.log('Created reminder ID:', result);
        return result;
    }
    
    getReminder(reminderId) {
        console.log('Retrieving reminder:', reminderId);
        const result = super.getReminder(reminderId);
        console.log('Retrieved reminder:', result);
        return result;
    }
}
```

#### 2. Memory Monitoring
```javascript
function logMemoryUsage() {
    const usage = process.memoryUsage();
    console.log('Memory usage:');
    console.log(`  RSS: ${Math.round(usage.rss / 1024 / 1024)} MB`);
    console.log(`  Heap Total: ${Math.round(usage.heapTotal / 1024 / 1024)} MB`);
    console.log(`  Heap Used: ${Math.round(usage.heapUsed / 1024 / 1024)} MB`);
}
```

#### 3. Performance Profiling
```javascript
// Using console.time for simple profiling
console.time('createReminder');
const reminderId = await manager.createReminder(reminder);
console.timeEnd('createReminder');

// For more detailed profiling, use Node.js built-in profiler
// node --prof app.js
// node --prof-process isolate-*.log > processed.txt
```

### Error Handling Best Practices

#### 1. Comprehensive Error Handling
```javascript
class RobustReminderManager extends ReminderManager {
    async createReminder(reminder) {
        try {
            // Validate input
            if (!reminder || typeof reminder !== 'object') {
                throw new Error('Invalid reminder object');
            }
            
            if (!reminder.content) {
                throw new Error('Reminder content is required');
            }
            
            return await super.createReminder(reminder);
        } catch (error) {
            console.error('Error creating reminder:', error);
            throw new Error(`Failed to create reminder: ${error.message}`);
        }
    }
    
    async updateReminder(reminderId, updates) {
        try {
            await super.updateReminder(reminderId, updates);
        } catch (error) {
            if (error.message.includes('not found')) {
                throw new Error(`Cannot update reminder ${reminderId}: ${error.message}`);
            }
            throw error;
        }
    }
}
```

#### 2. Graceful Degradation
```javascript
class GracefulReminderManager extends ReminderManager {
    getReminderStatistics() {
        try {
            return super.getReminderStatistics();
        } catch (error) {
            console.warn('Failed to generate statistics, returning defaults:', error.message);
            return new ReminderStatistics({
                total_reminders: 0,
                pending_reminders: 0,
                active_reminders: 0,
                overdue_reminders: 0,
                unique_tags: 0
            });
        }
    }
}
```

This comprehensive documentation covers all aspects of the Reminder Manager system, from basic usage to advanced deployment and troubleshooting. The system provides a solid foundation for reminder management applications with room for extension and customization based on specific requirements.