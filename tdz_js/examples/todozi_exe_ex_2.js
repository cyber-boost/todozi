// example2.js - Todozi Task Management Example

import { executeTodoziToolDelegated } from '../todozi/todozi_exe.js';

async function runTodoziExample() {
    console.log("🚀 Starting Todozi Task Management Example\n");

    try {
        // 1. Create a simple task
        console.log("1. Creating a simple task:");
        const simpleTask = await executeTodoziToolDelegated({
            action: 'task',
            content: 'Review project documentation'
        });
        console.log(simpleTask.output);

        // 2. Create an urgent task
        console.log("\n2. Creating an urgent task:");
        const urgentTask = await executeTodoziToolDelegated({
            action: 'urgent',
            content: 'Fix critical production bug'
        });
        console.log(urgentTask.output);

        // 3. Create a high priority task
        console.log("\n3. Creating a high priority task:");
        const highTask = await executeTodoziToolDelegated({
            action: 'high',
            content: 'Prepare quarterly report'
        });
        console.log(highTask.output);

        // 4. Create a low priority task
        console.log("\n4. Creating a low priority task:");
        const lowTask = await executeTodoziToolDelegated({
            action: 'low',
            content: 'Organize desk workspace'
        });
        console.log(lowTask.output);

        // 5. Create an AI task
        console.log("\n5. Creating an AI task:");
        const aiTask = await executeTodoziToolDelegated({
            action: 'ai',
            content: 'Analyze user feedback trends'
        });
        console.log(aiTask.output);

        // 6. Create a human task
        console.log("\n6. Creating a human task:");
        const humanTask = await executeTodoziToolDelegated({
            action: 'human',
            content: 'Schedule team meeting'
        });
        console.log(humanTask.output);

        // 7. Create a collaborative task
        console.log("\n7. Creating a collaborative task:");
        const collabTask = await executeTodoziToolDelegated({
            action: 'collab',
            content: 'Design new user interface'
        });
        console.log(collabTask.output);

        // 8. Search for tasks
        console.log("\n8. Searching for tasks:");
        const searchResult = await executeTodoziToolDelegated({
            action: 'find',
            content: 'project'
        });
        console.log(searchResult.output);

        // 9. Get system stats
        console.log("\n9. Getting system stats:");
        const stats = await executeTodoziToolDelegated({
            action: 'stats'
        });
        console.log(stats.output);

        // 10. Check queue status
        console.log("\n10. Checking queue status:");
        const queue = await executeTodoziToolDelegated({
            action: 'queue'
        });
        console.log(queue.output);

    } catch (error) {
        if (error.name === 'ExecutorError') {
            console.error(`❌ Todozi Error: ${error.toString()}`);
        } else {
            console.error(`❌ Unexpected Error: ${error.message}`);
        }
    }

    console.log("\n✅ Example completed!");
}

// Run the example
runTodoziExample();

/*
Here's a practical example demonstrating how to use the Todozi executor to create and manage tasks with different priorities and types:

This example demonstrates:

1. **Creating different task types**:
   - Simple tasks with `task` action
   - Priority-based tasks (urgent, high, low)
   - Specialized tasks (AI, human, collaborative)

2. **Using search functionality**:
   - Finding tasks with `find` action
   - Getting system statistics with `stats`
   - Checking queue status with `queue`

3. **Error handling**:
   - Proper error catching for ExecutorError
   - Displaying formatted error messages

4. **Practical usage patterns**:
   - All actions follow the same parameter structure
   - Results include metadata for programmatic access
   - Each task type has appropriate visual indicators

To run this example:
1. Save as `example2.js`
2. Ensure `todozi_exe.js` and dependencies are in the same directory
3. Run with `node example2.js`

Key features shown:
- Task creation with different priorities
- Specialized task types for different workflows
- Search and reporting capabilities
- Consistent error handling
- Metadata-rich responses for integration

The output will show:
- Created task IDs and confirmations
- Visual indicators for task types (🚨 for urgent, 🤖 for AI tasks)
- Search results with matching tasks
- System statistics and queue information
- Proper error messages if something goes wrong
*/