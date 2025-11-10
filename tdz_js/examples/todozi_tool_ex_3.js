import { initializeTodoziSystem, createTodoziTools, createGrokLevelTodoziTools } from '../todozi/todozi_tool.js';
import { Storage } from '../todozi/storage.js';
import {
    Priority, 
    Status, 
    Assignee,
    MemoryImportance,
    MemoryTerm
} from '../todozi/models.js';

async function manageDevProject() {
    console.log('🚀 Starting Software Development Project Management\n');
    
    // Initialize Todozi system
    const { todozi, embeddingService } = await initializeTodoziSystem();
    const tools = createGrokLevelTodoziTools(todozi);
    
    // Get tool instances
    const simpleTool = tools.find(t => t.name === 'simple_todozi');
    const searchTool = tools.find(t => t.definition().name === 'search_tasks');
    const memoryTool = tools.find(t => t.definition().name === 'create_memory');
    const processChatTool = tools.find(t => t.definition().name === 'process_chat_message');
    
    // 1. Create project tasks
    console.log('📋 Creating Project Tasks:');
    console.log('═════════════════════════════\n');
    
    const tasks = [
        // High priority tasks
        { action: 'Setup CI/CD pipeline for the new microservice', priority: 'high', assignee: 'ai' },
        { action: 'Implement user authentication API endpoint', priority: 'high', assignee: 'human' },
        { action: 'Design database schema for user data', priority: 'high', assignee: 'collaborative' },
        
        // Medium priority tasks
        { action: 'Write unit tests for authentication module', priority: 'medium', assignee: 'ai' },
        { action: 'Create API documentation', priority: 'medium', assignee: 'human' },
        { action: 'Setup monitoring and logging', priority: 'medium', assignee: 'ai' },
        
        // Low priority tasks
        { action: 'Refactor legacy code in payment module', priority: 'low', assignee: 'human' },
        { action: 'Update dependencies and fix security vulnerabilities', priority: 'low', assignee: 'ai' }
    ];
    
    const createdTasks = [];
    for (const task of tasks) {
        const result = await simpleTool.execute({
            action: task.assignee === 'ai' ? 'ai' : 
                   task.assignee === 'human' ? 'human' : 'collab',
            content: task.action
        });
        console.log(result.content);
        createdTasks.push(result.content);
    }
    
    console.log('\n' + '─'.repeat(50) + '\n');
    
    // 2. Search for similar tasks
    console.log('🔍 Semantic Search Examples:');
    console.log('══════════════════════════════\n');
    
    const searchQueries = [
        'database setup',
        'testing',
        'security',
        'documentation'
    ];
    
    for (const query of searchQueries) {
        console.log(`🔎 Searching for: "${query}"`);
        const result = await searchTool.execute({
            query: query,
            semantic: true,
            limit: 3
        });
        console.log(result.content);
        console.log();
    }
    
    console.log('─'.repeat(50) + '\n');
    
    // 3. Store project memories
    console.log('🧠 Storing Project Memories:');
    console.log('═════════════════════════════\n');
    
    const memories = [
        {
            moment: 'Client specified OAuth 2.0 is required for authentication',
            meaning: 'Must implement OAuth 2.0 instead of basic auth',
            reason: 'Security requirement from client',
            importance: MemoryImportance.High,
            term: MemoryTerm.Long
        },
        {
            moment: 'Team decided to use PostgreSQL for production',
            meaning: 'Database choice finalized as PostgreSQL',
            reason: 'Scalability and JSON support requirements',
            importance: MemoryImportance.Medium,
            term: MemoryTerm.Long
        },
        {
            moment: 'Docker containerization discussion',
            meaning: 'All microservices will be containerized',
            reason: 'Consistent deployment environment',
            importance: MemoryImportance.High,
            term: MemoryTerm.Long
        }
    ];
    
    for (const memory of memories) {
        const result = await memoryTool.execute(memory);
        console.log(result.content);
    }
    
    console.log('\n' + '─'.repeat(50) + '\n');
    
    // 4. Process team chat message
    console.log('💬 Processing Team Communication:');
    console.log('══════════════════════════════════\n');
    
    const teamMessage = `Hey team, just finished the sprint planning. Here's what we need:

<todozi>Review and merge pull request #123 for user auth;2 hours;high;authentication;in_progress;human;auth,review</todozi>

We should also remember:
<memory>Client wants JWT tokens to expire after 24 hours;JWT token expiry requirement;Security best practice;high;short;auth,security</memory>

And I have an idea:
<idea>Implement automatic security scanning in CI/CD pipeline;team;high;automation,security,ci-cd;Would catch vulnerabilities early</idea>

Also, we're hitting an API rate limit:
<error>API rate limit exceeded on user endpoint;Getting 429 errors when fetching user data;high;api;user-service;api,rate-limit</error>

Let's prioritize the auth review first!`;
    
    console.log('📨 Processing message:\n');
    console.log(teamMessage);
    console.log('\n' + '─'.repeat(50) + '\n');
    
    const processResult = await processChatTool.execute({
        message: teamMessage,
        user_id: 'team_lead'
    });
    
    console.log('📊 Processed Content:');
    console.log(processResult.content);
    
    console.log('\n' + '─'.repeat(50) + '\n');
    
    // 5. Create urgent task from critical finding
    console.log('🚨 Creating Critical Task:');
    console.log('═══════════════════════════\n');
    
    const urgentResult = await simpleTool.execute({
        action: 'urgent',
        content: 'Fix API rate limiting issue on user endpoint - clients experiencing 429 errors'
    });
    console.log(urgentResult.content);
    
    console.log('\n' + '─'.repeat(50) + '\n');
    
    // 6. Show project statistics
    console.log('📊 Project Statistics:');
    console.log('═════════════════════════\n');
    
    const statsResult = await simpleTool.execute({
        action: 'stats',
        content: ''
    });
    console.log(statsResult.content);
    
    console.log('\n' + '─'.repeat(50) + '\n');
    
    // 7. AI-powered task expansion
    console.log('🤖 AI Task Expansion:');
    console.log('═════════════════════════\n');
    
    const complexTask = 'Setup complete monitoring and observability stack';
    const expansionResult = await simpleTool.execute({
        action: 'expand',
        content: complexTask,
        extra: 'Include logging, metrics, tracing, and alerting'
    });
    console.log(expansionResult.content);
    
    console.log('\n' + '─'.repeat(50) + '\n');
    
    // 8. Smart search across all content types
    console.log('🧠 Unified Semantic Search:');
    console.log('═════════════════════════════\n');
    
    const unifiedResult = await simpleTool.execute({
        action: 'smart_search',
        content: 'security authentication requirements'
    });
    console.log(unifiedResult.content);
    
    console.log('\n🎉 Project Management Demo Complete!\n');
    console.log('Key Features Demonstrated:');
    console.log('• ✅ Task creation with different priorities and assignees');
    console.log('• 🔍 Semantic search for finding relevant tasks');
    console.log('• 🧠 Memory storage for important project information');
    console.log('• 💬 Structured chat message processing');
    console.log('• 🚨 Urgent task creation for critical issues');
    console.log('• 📊 Project statistics and overview');
    console.log('• 🤖 AI-powered task expansion');
    console.log('• 🧠 Unified search across all content types');
}

// Helper function to simulate project dashboard
async function createProjectDashboard(todozi) {
    console.log('\n📊 Project Dashboard:');
    console.log('═══════════════════════\n');
    
    const storage = new Storage();
    
    // Simulate getting project statistics
    const stats = {
        totalTasks: 12,
        activeTasks: 8,
        completedTasks: 3,
        blockedTasks: 1
    };
    
    console.log(`📋 Total Tasks: ${stats.totalTasks}`);
    console.log(`🔄 Active Tasks: ${stats.activeTasks}`);
    console.log(`✅ Completed Tasks: ${stats.completedTasks}`);
    console.log(`🚫 Blocked Tasks: ${stats.blockedTasks}`);
    
    const completionRate = ((stats.completedTasks / stats.totalTasks) * 100).toFixed(1);
    console.log(`📊 Completion Rate: ${completionRate}%`);
    
    console.log('\n🔥 High Priority Items:');
    console.log('• Setup CI/CD pipeline');
    console.log('• Implement user authentication');
    console.log('• Design database schema');
    
    console.log('\n🤖 AI-Assigned Tasks:');
    console.log('• Write unit tests for authentication');
    console.log('• Setup monitoring and logging');
    console.log('• Update dependencies');
    
    console.log('\n👤 Human-Assigned Tasks:');
    console.log('• Implement user authentication API');
    console.log('• Create API documentation');
    console.log('• Refactor legacy payment code');
}

// Error handling wrapper
async function runDemo() {
    try {
        await manageDevProject();
        await createProjectDashboard();
    } catch (error) {
        console.error('❌ Demo failed:', error.message);
        console.error(error.stack);
    }
}

// Export for use as module
    manageDevProject,
    createProjectDashboard,
    runDemo
};

// Run demo if called directly
// Run if executed directly
    runDemo();
}

/*
# Example 3: Managing a Software Development Project with Todozi

This example demonstrates how to use Todozi to manage a software development project, including creating tasks, using semantic search, and processing team communications.

## Key Features Demonstrated:

1. **Task Management**:
   - Create tasks with different priorities (high, medium, low)
   - Assign tasks to AI, human, or collaborative teams
   - Track task status and progress

2. **Semantic Search**:
   - AI-powered search that understands context
   - Find similar tasks using vector embeddings
   - Search across all content types (tasks, memories, ideas, errors)

3. **Memory Storage**:
   - Store important project decisions and requirements
   - Categorize memories by importance and term
   - Retrieve relevant memories during task planning

4. **Chat Processing**:
   - Parse structured tags from team messages
   - Automatically create tasks, memories, ideas, and error reports
   - Extract actionable items from natural language

5. **AI Integration**:
   - Automatic task assignment based on content analysis
   - Task expansion into detailed subtasks
   - Smart recommendations and insights

6. **Project Analytics**:
   - Real-time project statistics
   - Completion rates and progress tracking
   - Task distribution by priority and assignee

This example shows how Todozi can serve as a comprehensive project management tool that combines traditional task tracking with AI-powered features for intelligent automation and insights.
*/