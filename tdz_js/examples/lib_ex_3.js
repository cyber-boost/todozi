import { Done } from '../todozi/lib.js';

async function automatedWorkflowPlanning() {
    // Define a complex goal
    const projectGoal = "Launch new customer dashboard with real-time analytics";
    
    // Use AI to break down the goal into actionable tasks
    const plannedTasks = await Done.planTasks(
        projectGoal,
        "high",
        "2 weeks",
        "React frontend, Python backend, PostgreSQL database"
    );
    
    console.log(`📋 AI-generated plan for: ${projectGoal}`);
    console.log(`Created ${plannedTasks.length} tasks:\n`);
    
    // Display and organize the planned tasks
    const tasksByType = {
        frontend: [],
        backend: [],
        database: [],
        testing: [],
        deployment: []
    };
    
    plannedTasks.forEach(task => {
        const action = task.action.toLowerCase();
        if (action.includes('frontend') || action.includes('ui') || action.includes('react')) {
            tasksByType.frontend.push(task);
        } else if (action.includes('backend') || action.includes('api') || action.includes('python')) {
            tasksByType.backend.push(task);
        } else if (action.includes('database') || action.includes('sql')) {
            tasksByType.database.push(task);
        } else if (action.includes('test') || action.includes('testing')) {
            tasksByType.testing.push(task);
        } else if (action.includes('deploy') || action.includes('launch')) {
            tasksByType.deployment.push(task);
        }
    });
    
    // Display tasks by category
    Object.entries(tasksByType).forEach(([category, tasks]) => {
        if (tasks.length > 0) {
            console.log(`\n${category.toUpperCase()} (${tasks.length}):`);
            tasks.forEach(task => {
                console.log(`  • ${task.action} [${task.priority}]`);
            });
        }
    });
    
    // Set up dependencies between tasks
    if (plannedTasks.length > 1) {
        // Make database tasks dependencies for backend
        const dbTask = tasksByType.database[0];
        if (dbTask) {
            tasksByType.backend.forEach(backendTask => {
                Done.updateTaskFull(backendTask.id, {
                    dependencies: [dbTask.id]
                });
            });
        }
    }
}

/*
async function demonstrateSemanticSearch() {
    // Find semantically similar tasks
    const paymentTasks = await Done.aiTasks("payment processing", 5);
    
    console.log('🔍 Semantic Search Results:');
    for (const task of paymentTasks) {
        console.log(`  • ${task.textContent} (${task.similarityScore.toFixed(2)} similarity)`);
    }
    
    // Get AI-powered task suggestions for a project
    const allTasks = await Done.allTasks();
    const activeTasks = allTasks.filter(t => t.status === 'todo');
    
    if (activeTasks.length > 0) {
        console.log('\n💡 AI Task Suggestions:');
        
        for (const task of activeTasks.slice(0, 3)) {
            // Find related tasks that might help complete this one
            const suggestions = await Done.findTasksAi(task.action, 3);
            
            if (suggestions.length > 0) {
                console.log(`\nFor task: "${task.action}"`);
                console.log('Related tasks:');
                suggestions.forEach((s, i) => {
                    console.log(`  ${i + 1}. ${s.action}`);
                });
            }
        }
    }
}

/ *
async function createDevelopmentTasks() {
    // Create a complex development task with AI assistance
    const architectureTask = await Done.ai(
        "Design microservices architecture for new payment system",
    );
    
    // Create related human tasks for implementation
    const implementAuth = await Done.human(
        "Implement authentication service for payment system",
    );
    
    // Create collaborative code review task
    const codeReview = await Done.collab(
        "Review and test payment system integration",
    );
    
    console.log(`Created tasks: ${architectureTask}, ${implementAuth}, ${codeReview}`);
    
    // Add tasks to a specific project
    await Done.createProject("payment-system", "New payment processing platform");
    
    // Update tasks to assign them to the project
    await Done.updateTaskFull(architectureTask, {
        parentProject: "payment-system",
        priority: "high"
    });
    
    await Done.updateTaskFull(implementAuth, {
        parentProject: "payment-system",
        priority: "high"
    });
    
    await Done.updateTaskFull(codeReview, {
        parentProject: "payment-system",
        priority: "medium"
    });
}

/ *

async function initializeTodozi() {
    // Initialize the Todozi system
    await Done.ensureTodoziInitialized();
    
    // Set up the embedding service for semantic search
    await Done.initializeEmbeddingService();
    
    console.log('✅ Todozi initialized with semantic search capabilities');
}

/ *
# Example 3: Advanced Task Management with Semantic Search

This example demonstrates advanced task management features including semantic search, AI-powered task suggestions, and collaborative workflow automation.

## Setup and Initialization

## Creating Tasks with Context

## Semantic Search and AI Suggestions

## Workflow Automation with AI Planning

## Memory and Idea Integration

async function knowledgeManagement() {
    // Store important architectural decisions
    await Done.remember(
        "Chose microservices over monolith for scalability",
        "Microservices allow independent scaling and deployment",
        "Future growth expectations and team structure"
    );
    
    // Capture technical ideas
    await Done.ideate(
        "Implement event-driven architecture with message queue"
    );
    
    await Done.ideate(
        "Use Kubernetes for container orchestration"
    );
    
    // Create a breakthrough idea
    await Done.breakthrough(
        "Implement real-time analytics using WebSocket + Redis streams"
    );
    
    // Search memories for relevant context
    const architectureMemories = await Done.findMemory("architecture");
    
    console.log('🧠 Relevant Memories:');
    architectureMemories.forEach(memory => {
        console.log(`  • ${memory.moment}: ${memory.meaning}`);
    });
    
    // Find related ideas
    const technicalIdeas = await Done.findIdea("architecture");
    
    console.log('\n💡 Related Ideas:');
    technicalIdeas.forEach(idea => {
        console.log(`  • ${idea.idea}`);
    });
}

## Advanced Filtering and Analytics

async function advancedTaskAnalytics() {
    // Create custom filters for different views
    const highPriorityTasks = await Done.createTaskFilters(
        null,           // project
        "todo",         // status
        "high",         // priority
        null,           // assignee
        null,           // tags
        null            // search
    );
    
    const aiAssignedTasks = await Done.createTaskFilters(
        "payment-system",
        null,
        null,
        "ai",
        null,
        null
    );
    
    // Get tasks with custom filters
    const urgentTasks = await Done.searchWithFilters(highPriorityTasks, 10);
    const aiTasks = await Done.searchWithFilters(aiAssignedTasks, 10);
    
    console.log(`🚨 Found ${urgentTasks.length} urgent tasks`);
    console.log(`🤖 Found ${aiTasks.length} AI-assigned tasks`);
    
    // Calculate completion metrics
    const allTasks = await Done.allTasks();
    const completedTasks = allTasks.filter(t => t.status === 'done');
    const completionRate = (completedTasks.length / allTasks.length * 100).toFixed(1);
    
    console.log(`\n📊 Analytics:`);
    console.log(`  Total tasks: ${allTasks.length}`);
    console.log(`  Completed: ${completedTasks.length}`);
    console.log(`  Completion rate: ${completionRate}%`);
    
    // Show project statistics
    const projects = await Done.listProjects();
    console.log(`\n📁 Projects: ${projects.length}`);
    
    for (const project of projects.slice(0, 3)) {
        const projectTasks = await Done.tasks(project);
        const projectCompleted = projectTasks.filter(t => t.status === 'done').length;
        const projectRate = projectTasks.length > 0 
            ? (projectCompleted / projectTasks.length * 100).toFixed(1)
            : 0;
        
        console.log(`  ${project}: ${projectTasks.length} tasks (${projectRate}% complete)`);
    }
}

## Collaborative Agent Management

async function agentCollaboration() {
    // Create specialized agents for different roles
    const architectAgent = await Done.createArchitectAgent();
    const coderAgent = await Done.createCoder();
    const testerAgent = await Done.createTesterAgent();
    
    console.log(`🤖 Created agents: architect, coder, tester`);
    
    // Get all available agents
    const agents = await Done.getAllAgents();
    
    console.log('\n📋 Available Agents:');
    agents.forEach(agent => {
        console.log(`  • ${agent.id}: ${agent.description}`);
    });
    
    // Find the best agent for a specific task
    const bestArchitect = await Done.findBestAgent("system architecture");
    console.log(`\n🎯 Best agent for architecture: ${bestArchitect.id}`);
    
    // Create a task and assign it to an agent
    const systemDesign = await Done.createTask(
        "Design scalable system architecture",
        "high",
        "payment-system",
        "1 week",
        "Focus on microservices and data flow"
    );
    
    // Assign the architect agent to this task
    // This would typically create an agent assignment
    console.log(`\n📝 Assigned architect agent to task: ${systemDesign}`);
}

## Complete Workflow Example
*/