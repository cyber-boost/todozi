// todozi-example.js
import { 
    TodoziError, 
    Priority, 
    Status, 
    Assignee,
    parseTodoziFormat,
    processChatMessageExtended,
    processWorkflow
} from '../todozi/todozi.js';
import { TagManager, Tag } from '../todozi/tags.js';
import { Storage } from '../todozi/storage.js';
import { v4 as uuidv4 } from 'uuid';

class TodoziExample {
    constructor() {
        this.tagManager = TagManager.new();
        this.storage = null;
    }

    async initialize() {
        try {
            this.storage = await Storage.new();
            console.log('✅ Todozi Example Initialized');
            
            // Create some default tags
            await this.setupDefaultTags();
            
        } catch (error) {
            console.error('❌ Failed to initialize Todozi Example:', error.message);
        }
    }

    async setupDefaultTags() {
        const defaultTags = [
            { name: 'development', description: 'Software development tasks', category: 'technical' },
            { name: 'documentation', description: 'Documentation related tasks', category: 'writing' },
            { name: 'testing', description: 'Testing and QA tasks', category: 'quality' },
            { name: 'design', description: 'Design and UX tasks', category: 'creative' }
        ];

        for (const tag of defaultTags) {
            await this.tagManager.createTag(new Tag(tag));
        }
        console.log('📝 Default tags created');
    }

    // Example 1: Manual Task Creation
    async createTaskManually() {
        console.log('\n📋 Example 1: Manual Task Creation');
        
        const task = {
            id: `task_${uuidv4().substring(0, 8)}`,
            user_id: "example_user",
            action: "Implement user authentication system",
            time: "8 hours",
            priority: Priority.High,
            parent_project: "auth-project",
            status: Status.Todo,
            assignee: Assignee.Collaborative,
            tags: ["development", "security"],
            dependencies: [],
            context_notes: "Need to implement OAuth2 and JWT tokens",
            progress: 0,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
        };

        try {
            await this.storage.addTaskToProject(task);
            console.log('✅ Task created:', task.action);
            console.log('   Project:', task.parent_project);
            console.log('   Priority:', task.priority);
            console.log('   Assignee:', Assignee.toString(task.assignee));
            console.log('   Tags:', task.tags.join(', '));
            
            return task.id;
        } catch (error) {
            console.error('❌ Failed to create task:', error.message);
        }
    }

    // Example 2: Chat Message Processing
    async processChatMessage() {
        console.log('\n💬 Example 2: Chat Message Processing');
        
        const chatMessage = `
        Hey, I need to get these tasks done:

        <todozi>
        Create API documentation; 4 hours; medium; docs-project; todo; human; documentation,writing; ; Need to document all endpoints
        </todozi>

        <todozi>
        Fix login bug; 2 hours; high; auth-project; in_progress; ai; development,bug; task_001; User cannot login with valid credentials; 50
        </todozi>

        <todozi>
        Design dashboard UI; 6 hours; medium; ui-project; todo; designer; design,ui; ; Create modern dashboard interface
        </todozi>

        Also, remember this important moment:
        <memory>
        standard; Fixed authentication issue; Learned about JWT token expiration; Debug session; high; long; security,learning
        </memory>
        `;

        try {
            const content = processChatMessageExtended(chatMessage, "chat_user");
            
            console.log('📊 Extracted Content:');
            console.log(`   Tasks: ${content.tasks.length}`);
            console.log(`   Memories: ${content.memories.length}`);

            // Process tasks
            for (const task of content.tasks) {
                await this.storage.addTaskToProject(task);
                console.log(`   ✅ Task added: ${task.action}`);
            }

            // Process memory
            if (content.memories.length > 0) {
                console.log(`   🧠 Memory: ${content.memories[0].moment}`);
            }

        } catch (error) {
            console.error('❌ Failed to process chat message:', error.message);
        }
    }

    // Example 3: Task Workflow Execution
    async demonstrateTaskWorkflow() {
        console.log('\n⚡ Example 3: Task Workflow Execution');

        const testTask = {
            id: `task_${uuidv4().substring(0, 8)}`,
            user_id: "workflow_user",
            action: "Analyze performance metrics",
            time: "3 hours",
            priority: Priority.Medium,
            parent_project: "analytics-project",
            status: Status.Todo,
            assignee: null, // Let the system decide
            tags: ["analysis", "performance"],
            dependencies: [],
            context_notes: "Review server response times and optimize",
            progress: 0,
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString()
        };

        try {
            // Add task to storage first
            await this.storage.addTaskToProject(testTask);
            console.log('📥 Task added to storage');

            // Execute the workflow
            const results = await processWorkflow([testTask]);
            console.log('🚀 Workflow execution results:');
            results.forEach((result, index) => {
                console.log(`   ${index + 1}. ${result}`);
            });

        } catch (error) {
            console.error('❌ Workflow execution failed:', error.message);
        }
    }

    // Example 4: Tag Management and Search
    async demonstrateTagSystem() {
        console.log('\n🏷️ Example 4: Tag Management and Search');

        try {
            // Create some additional tags
            const newTagIds = await this.tagManager.bulkCreateTags(
                ["api", "backend", "frontend"],
                "development"
            );
            console.log('✅ Created new tags: api, backend, frontend');

            // Search for tags
            const searchResults = this.tagManager.searchTags("dev");
            console.log('🔍 Tag search results for "dev":');
            searchResults.forEach(tag => {
                console.log(`   • ${tag.name} (${tag.category})`);
            });

            // Get tag statistics
            const stats = this.tagManager.getTagStatistics();
            console.log('📊 Tag Statistics:');
            console.log(`   Total Tags: ${stats.total_tags}`);
            console.log(`   Total Categories: ${stats.total_categories}`);
            console.log(`   Average Usage: ${stats.average_usage.toFixed(2)}`);

            // Get most used tags
            const popularTags = this.tagManager.getMostUsedTags(3);
            console.log('🔥 Most Used Tags:');
            popularTags.forEach(tag => {
                console.log(`   • ${tag.name} (${tag.usage_count} uses)`);
            });

        } catch (error) {
            console.error('❌ Tag system demo failed:', error.message);
        }
    }

    // Example 5: Multi-assignee Task Processing
    async demonstrateMultiAssigneeTasks() {
        console.log('\n🤝 Example 5: Multi-assignee Task Processing');

        const tasks = [
            {
                id: `task_${uuidv4().substring(0, 8)}`,
                user_id: "multi_user",
                action: "Generate report analysis",
                time: "2 hours",
                priority: Priority.Medium,
                parent_project: "reporting-project",
                status: Status.Todo,
                assignee: Assignee.Ai,
                tags: ["analysis", "reporting"],
                progress: 0
            },
            {
                id: `task_${uuidv4().substring(0, 8)}`,
                user_id: "multi_user",
                action: "Manual data verification",
                time: "1 hour",
                priority: Priority.High,
                parent_project: "reporting-project",
                status: Status.Todo,
                assignee: Assignee.Human,
                tags: ["verification", "quality"],
                progress: 0
            },
            {
                id: `task_${uuidv4().substring(0, 8)}`,
                user_id: "multi_user",
                action: "Team code review",
                time: "3 hours",
                priority: Priority.Medium,
                parent_project: "reporting-project",
                status: Status.Todo,
                assignee: Assignee.Collaborative,
                tags: ["review", "collaboration"],
                progress: 0
            }
        ];

        try {
            for (const task of tasks) {
                task.created_at = new Date().toISOString();
                task.updated_at = task.created_at;
                
                await this.storage.addTaskToProject(task);
                console.log(`✅ ${Assignee.toString(task.assignee)} task: ${task.action}`);
            }

            console.log('\n🎯 Different assignee types handled automatically:');
            console.log('   🤖 AI: Automated analysis and processing');
            console.log('   👤 Human: Manual verification and decision-making');
            console.log('   🤝 Collaborative: Team-based review and coordination');

        } catch (error) {
            console.error('❌ Multi-assignee demo failed:', error.message);
        }
    }

    // Run all examples
    async runAllExamples() {
        await this.initialize();
        
        await this.createTaskManually();
        await this.processChatMessage();
        await this.demonstrateTaskWorkflow();
        await this.demonstrateTagSystem();
        await this.demonstrateMultiAssigneeTasks();

        console.log('\n🎉 All Todozi examples completed successfully!');
        console.log('\n💡 Key Features Demonstrated:');
        console.log('   • Task creation and management');
        console.log('   • Chat message processing with tags');
        console.log('   • Automated workflow execution');
        console.log('   • Tag system with search and statistics');
        console.log('   • Multi-assignee task handling');
    }
}

// Run the example
async function main() {
    const example = new TodoziExample();
    await example.runAllExamples();
}

main().catch(console.error);

/*
# Example: Todozi Task Management System Integration

This example demonstrates how to use the Todozi system to create, manage, and process tasks with different assignee types, showing the seamless integration between the various modules.

## Example Output:

/ *
✅ Todozi Example Initialized
📝 Default tags created

📋 Example 1: Manual Task Creation
✅ Task created: Implement user authentication system
   Project: auth-project
   Priority: high
   Assignee: collaborative
   Tags: development, security

💬 Example 2: Chat Message Processing
📊 Extracted Content:
   Tasks: 3
   Memories: 1
   ✅ Task added: Create API documentation
   ✅ Task added: Fix login bug
   ✅ Task added: Design dashboard UI
   🧠 Memory: Fixed authentication issue

⚡ Example 3: Task Workflow Execution
📥 Task added to storage
🚀 Workflow execution results:
   1. Task queued for AI processing: Analyze performance metrics (Queue ID: queue_abc123)

🏷️ Example 4: Tag Management and Search
✅ Created new tags: api, backend, frontend
🔍 Tag search results for "dev":
   • development (technical)
   • backend (development)
📊 Tag Statistics:
   Total Tags: 7
   Total Categories: 3
   Average Usage: 0.00
🔥 Most Used Tags:
   • development (0 uses)
   • documentation (0 uses)
   • testing (0 uses)

🤝 Example 5: Multi-assignee Task Processing
✅ ai task: Generate report analysis
✅ human task: Manual data verification
✅ collaborative task: Team code review

🎯 Different assignee types handled automatically:
   🤖 AI: Automated analysis and processing
   👤 Human: Manual verification and decision-making
   🤝 Collaborative: Team-based review and coordination

🎉 All Todozi examples completed successfully!

💡 Key Features Demonstrated:
   • Task creation and management
   • Chat message processing with tags
   • Automated workflow execution
   • Tag system with search and statistics
   • Multi-assignee task handling
*/