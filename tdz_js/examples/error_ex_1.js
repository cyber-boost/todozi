// error-demo.js
import { TodoziError, ErrorManager, parseErrorFormat } from '../todozi/error.js';

class TaskValidator {
    constructor(errorManager) {
        this.errorManager = errorManager;
    }

    async validateTask(task) {
        // Validate priority
        const validPriorities = ['low', 'medium', 'high', 'critical', 'urgent'];
        if (!validPriorities.includes(task.priority)) {
            throw TodoziError.invalidPriority(task.priority);
        }

        // Validate status
        const validStatuses = ['todo', 'in_progress', 'blocked', 'review', 'done', 'cancelled', 'deferred'];
        if (!validStatuses.includes(task.status)) {
            throw TodoziError.invalidStatus(task.status);
        }

        // Validate progress (0-100)
        if (task.progress < 0 || task.progress > 100) {
            throw TodoziError.invalidProgress(task.progress);
        }

        // Validate required fields
        if (!task.action || task.action.trim().length < 3) {
            throw TodoziError.validation('Task action must be at least 3 characters long');
        }
    }

    async handleTaskNotFound(taskId) {
        const error = TodoziError.taskNotFound(taskId);
        const errorId = await this.errorManager.createError(error);
        
        console.log(`❌ Task Error Created: ${error.message}`);
        console.log(`📋 Error ID: ${errorId}`);
        console.log(`🔍 Error Details:`, error.details);
        
        return errorId;
    }
}

// Example usage
async function demonstrateErrorHandling() {
    const errorManager = new ErrorManager();
    const validator = new TaskValidator(errorManager);

    // Example 1: Handle a task not found scenario
    console.log('=== Example 1: Task Not Found ===');
    const taskId = 'task_abc123';
    const errorId1 = await validator.handleTaskNotFound(taskId);
    
    // Example 2: Validate an invalid task
    console.log('\n=== Example 2: Invalid Task Validation ===');
    const invalidTask = {
        action: 't', // Too short
        priority: 'super_high', // Invalid priority
        status: 'pending', // Invalid status
        progress: 150 // Invalid progress
    };

    try {
        await validator.validateTask(invalidTask);
    } catch (error) {
        if (error instanceof TodoziError) {
            const errorId2 = await errorManager.createError(error);
            console.log(`❌ Validation Error: ${error.message}`);
            console.log(`📋 Error ID: ${errorId2}`);
            console.log(`🔍 Error Type: ${error.type}`);
            console.log(`📊 Error Details:`, error.details);
        }
    }

    // Example 3: Parse error from formatted text
    console.log('\n=== Example 3: Parse Error Format ===');
    const errorText = `<error>Database Connection Failed;Unable to connect to database;high;database;todozi-storage;Connection timeout after 30 seconds;network,storage,timeout</error>`;
    
    try {
        const parsedError = parseErrorFormat(errorText);
        const errorId3 = await errorManager.createError(parsedError);
        
        console.log(`✅ Parsed Error Successfully:`);
        console.log(`📋 ID: ${errorId3}`);
        console.log(`📝 Title: ${parsedError.title}`);
        System.out.println(`📄 Description: ${parsedError.description}`);
        console.log(`🚨 Severity: ${parsedError.severity}`);
        console.log(`🏷️  Category: ${parsedError.category}`);
        console.log(`🔗 Tags: ${parsedError.tags.join(', ')}`);
    } catch (parseError) {
        console.log(`❌ Parse Error: ${parseError.message}`);
    }

    // Example 4: List unresolved errors and resolve one
    console.log('\n=== Example 4: Error Management ===');
    const unresolvedErrors = errorManager.getUnresolvedErrors();
    console.log(`📊 Unresolved Errors: ${unresolvedErrors.length}`);
    
    if (unresolvedErrors.length > 0) {
        const firstError = unresolvedErrors[0];
        console.log(`🔍 First Unresolved Error: ${firstError.message}`);
        
        // Resolve the error
        await errorManager.resolveError(firstError.id, 'Fixed database connection timeout');
        console.log(`✅ Error ${firstError.id} resolved!`);
        
        // Show remaining unresolved errors
        const remainingErrors = errorManager.getUnresolvedErrors();
        console.log(`📊 Remaining Unresolved Errors: ${remainingErrors.length}`);
    }

    // Example 5: Custom error creation
    console.log('\n=== Example 5: Custom Error ===');
    const customError = new TodoziError(
        'Custom business logic failed',
        'BusinessLogic',
        { userId: 'user123', operation: 'task_creation', reason: 'insufficient_permissions' }
    );
    
    const customErrorId = await errorManager.createError(customError);
    console.log(`📋 Custom Error ID: ${customErrorId}`);
    console.log(`📝 Message: ${customError.message}`);
    console.log(`🔍 Type: ${customError.type}`);
    console.log(`📊 Details:`, customError.details);
}

// Run the demonstration
demonstrateErrorHandling().catch(console.error);

/*
**Example 1: Creating and Handling Custom Task Errors**

This example demonstrates how to use the `TodoziError` class to handle common task-related errors in a Todozi application, showing practical error creation, management, and custom parsing.

**Key Features Demonstrated:**

1. **Specific Error Types**: Using static methods for common task errors like `taskNotFound()`, `invalidPriority()`, etc.
2. **Error Manager Integration**: Creating, tracking, and resolving errors with the ErrorManager class
3. **Custom Error Format Parsing**: Parsing structured error text with the `<error>` tag format
4. **Error Metadata**: Including detailed information like severity, category, and custom tags
5. **Error Resolution Workflow**: Tracking unresolved errors and marking them as resolved

**Expected Output:**

/ *
=== Example 1: Task Not Found ===
❌ Task Error Created: Task not found: task_abc123
📋 Error ID: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
🔍 Error Details: { id: 'task_abc123' }

=== Example 2: Invalid Task Validation ===
❌ Validation Error: Invalid priority: super_high. Must be one of: low, medium, high, critical, urgent
📋 Error ID: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
🔍 Error Type: InvalidPriority
📊 Error Details: { priority: 'super_high' }

=== Example 3: Parse Error Format ===
✅ Parsed Error Successfully:
📋 ID: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
📝 Title: Database Connection Failed
📄 Description: Unable to connect to database
🚨 Severity: high
🏷️  Category: database
🔗 Tags: network, storage, timeout

=== Example 4: Error Management ===
📊 Unresolved Errors: 2
🔍 First Unresolved Error: Task not found: task_abc123
✅ Error xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx resolved!
📊 Remaining Unresolved Errors: 1

=== Example 5: Custom Error ===
📋 Custom Error ID: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
📝 Message: Custom business logic failed
🔍 Type: BusinessLogic
📊 Details: { userId: 'user123', operation: 'task_creation', reason: 'insufficient_permissions' }
*/