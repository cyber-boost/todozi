// example2_todozi_usage.js
import { SimpleTodoziTool, CreateTaskTool, SearchTasksTool, UpdateTaskTool, CreateMemoryTool, CreateIdeaTool, UnifiedSearchTool, Storage } from '../todozi/todozi_tool.js';

async function runTodoziExample() {
    try {
        // Initialize Todozi system
        const { todozi } = await initializeTodoziSystem();
        const storage = new Storage();
        
        console.log("=== Todozi Usage Example ===\n");

        // 1. Simple task creation using SimpleTodoziTool
        console.log("1. Creating tasks with SimpleTodoziTool:");
        const simpleTool = new SimpleTodoziTool();
        
        let result = await simpleTool.execute({
            action: "task",
            content: "Review project requirements",
            extra: "Check with stakeholders"
        });
        console.log(result.content);
        
        result = await simpleTool.execute({
            action: "urgent",
            content: "Fix critical bug in production",
            extra: "Database connection issue"
        });
        console.log(result.content);
        
        result = await simpleTool.execute({
            action: "ai",
            content: "Generate user personas",
            extra: "Based on analytics data"
        });
        console.log(result.content);
        console.log();

        // 2. Detailed task creation with CreateTaskTool
        console.log("2. Creating detailed task:");
        const createTaskTool = new CreateTaskTool(todozi);
        
        result = await createTaskTool.execute({
            action: "Implement authentication module",
            time: "3 days",
            priority: "high",
            project: "WebApp Development",
            assignee: "ai",
            tags: "backend,security,authentication",
            context: "Use OAuth 2.0 with JWT tokens"
        });
        console.log(result.content);
        console.log();

        // 3. Searching tasks
        console.log("3. Searching tasks:");
        const searchTool = new SearchTasksTool(todozi);
        
        result = await searchTool.execute({
            query: "authentication",
            semantic: false
        });
        console.log(result.content);
        console.log();

        // 4. Updating task status
        console.log("4. Updating task:");
        const updateTool = new UpdateTaskTool(todozi);
        
        // First create a task to update
        const taskResult = await createTaskTool.execute({
            action: "Write API documentation",
            project: "WebApp Development",
            priority: "medium"
        });
        console.log(taskResult.content);
        
        // Extract task ID (in real usage, you'd get this from the result)
        const taskId = "sample-task-id-12345"; // Placeholder
        
        result = await updateTool.execute({
            task_id: taskId,
            status: "in_progress",
            progress: 50,
            context: "Started with authentication endpoints"
        });
        console.log(result.content);
        console.log();

        // 5. Creating memories
        console.log("5. Creating memory:");
        const memoryTool = new CreateMemoryTool(todozi);
        
        result = await memoryTool.execute({
            moment: "Team decided to use React for frontend",
            meaning: "Standardizes our frontend development",
            reason: "Improves team efficiency and code consistency",
            importance: "high",
            term: "long"
        });
        console.log(result.content);
        console.log();

        // 6. Creating ideas
        console.log("6. Creating idea:");
        const ideaTool = new CreateIdeaTool(todozi);
        
        result = await ideaTool.execute({
            idea: "Implement dark mode toggle",
            share: "team",
            importance: "medium",
            tags: "ux,frontend,accessibility",
            context: "User preference and eye strain reduction"
        });
        console.log(result.content);
        console.log();

        // 7. Unified search across all data types
        console.log("7. Unified search:");
        const unifiedSearchTool = new UnifiedSearchTool(todozi);
        
        result = await unifiedSearchTool.execute({
            query: "project planning",
            semantic: true,
            data_types: "tasks,memories,ideas",
            limit: 3
        });
        console.log(result.content);
        console.log();

        console.log("=== Example completed successfully ===");

    } catch (error) {
        console.error("Error in Todozi example:", error.message);
    }
}

// Helper function to simulate system initialization
async function initializeTodoziSystem() {
    return {
        todozi: {
            lock: async () => new Storage()
        }
    };
}

// Run the example
runTodoziExample();

/*
Here's a practical example demonstrating how to use the Todozi tools for task management and AI-powered features:

This example demonstrates:

1. **Simple Task Creation**: Using the `SimpleTodoziTool` for quick task creation with different action types
2. **Detailed Task Management**: Creating tasks with full parameters using `CreateTaskTool`
3. **Task Searching**: Finding tasks using both keyword and semantic search
4. **Task Updates**: Modifying task status and progress
5. **Memory Creation**: Storing important project decisions and learnings
6. **Idea Capture**: Recording team ideas with sharing options
7. **Unified Search**: Searching across tasks, memories, and ideas simultaneously

Key features showcased:
- Priority levels (low/medium/high/critical/urgent)
- Assignee types (ai/human/collaborative)
- Tagging system for organization
- Context notes for detailed information
- Progress tracking
- Semantic AI search capabilities
- Cross-data-type search functionality

To run this example:
1. Save it as `example2_todozi_usage.js`
2. Ensure you have the `todozi_tool.js` file in the same directory
3. Install required dependencies: `npm install uuid axios`
4. Run with: `node example2_todozi_usage.js`

The example will output the results of each operation, showing how Todozi tools can be used to manage tasks, memories, and ideas in an AI-enhanced workflow.
*/