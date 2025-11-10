// example5_development_workflow.js
import express from 'express.js';
import sqlite3 from 'sqlite3.js';
import axios from 'axios.js';
import cheerio from 'cheerio.js';

import { 
    TodoziEmbeddingService, 
    TodoziEmbeddingConfig 
} from '../todozi/emb.js';
import { 
    Task, 
    Priority, 
    Status, 
    Agent, 
    Project 
} from '../todozi/models.js';
import { 
    processChatMessageExtended, 
    executeTask, 
    parseTodoziFormat 
} from '../todozi/todozi.js';
import { Storage } from '../todozi/storage.js';
import path from 'path';

/**
 * Example 5: Complete Development Workflow
 * 
 * This example demonstrates a complete software development workflow using Todozi,
 * including task management, AI assistance, semantic search, and code chunking.
 */

class DevelopmentWorkflowExample {
    constructor() {
        this.storage = null;
        this.embeddingService = null;
        this.projectName = 'web_scraper_project';
        this.userId = 'developer_alice';
    }

    async initialize() {
        console.log('🚀 Initializing Todozi Development Environment...');
        console.log('═'.repeat(60));
        
        // Initialize storage
        this.storage = await Storage.new();
        console.log('✅ Storage initialized');

        // Initialize embedding service
        const config = new TodoziEmbeddingConfig();
        this.embeddingService = await TodoziEmbeddingService.new(config);
        await this.embeddingService.initialize();
        console.log('✅ Embedding service initialized');

        // Create project if it doesn't exist
        try {
            await this.storage.getProject(this.projectName);
            console.log(`✅ Project "${this.projectName}" loaded`);
        } catch (e) {
            const project = new Project(this.projectName, 'Web scraper with database storage');
            await this.storage.createProject(project.name, project.description);
            console.log(`✅ Project "${this.projectName}" created`);
        }
    }

    async demonstrateTaskCreation() {
        console.log('\n📋 Creating Development Tasks...');
        console.log('─'.repeat(40));
        
        // Create a series of development tasks
        const tasks = [
            {
                action: 'Set up project structure and dependencies',
                time: '2 hours',
                priority: Priority.High,
                status: Status.Todo,
                context: 'Initialize Node.js project with Express, SQLite, and scraping libraries'
            },
            {
                action: 'Design database schema for scraped data',
                time: '1 hour',
                priority: Priority.High,
                status: Status.Todo,
                context: 'Create tables for URLs, content, metadata, and processing status'
            },
            {
                action: 'Implement HTTP client with retry logic',
                time: '3 hours',
                priority: Priority.Medium,
                status: Status.Todo,
                context: 'Use axios with exponential backoff and rate limiting'
            },
            {
                action: 'Build HTML parser for target websites',
                time: '4 hours',
                priority: Priority.Medium,
                status: Status.Todo,
                context: 'Parse product listings with Cheerio, handle dynamic content'
            },
            {
                action: 'Create data storage and retrieval APIs',
                time: '2 hours',
                priority: Priority.Medium,
                status: Status.Todo,
                context: 'REST endpoints for CRUD operations on scraped data'
            }
        ];

        for (const taskData of tasks) {
            const task = Task.new(
                this.userId,
                taskData.action,
                taskData.time,
                taskData.priority,
                this.projectName,
                taskData.status
            );
            
            task.contextNotes = taskData.context;
            task.tags = ['development', 'web-scraper', 'backend'];
            
            await this.storage.addTaskToProject(task);
            
            // Generate embedding for semantic search
            await this.embeddingService.addTask(task);
            
            console.log(`  📝 Created: ${task.action} (Priority: ${task.priority})`);
        }
    }

    async demonstrateAgentCreation() {
        console.log('\n🤖 Creating Specialized Development Agents...');
        console.log('─'.repeat(40));
        
        // Create a code generation agent
        const coderAgent = Agent.createCoder();
        coderAgent.id = 'scraper_coder';
        coderAgent.name = 'Web Scraper Developer';
        coderAgent.description = 'Specialized in web scraping, Node.js, and data processing';
        coderAgent.specializations = ['nodejs', 'scraping', 'sqlite', 'express', 'async-programming'];
        coderAgent.capabilities = ['code_development', 'api_design', 'database_optimization'];
        
        // Create a testing agent
        const testerAgent = Agent.new('tester', 'QA Tester', 'Quality assurance and testing specialist');
        testerAgent.systemPrompt = "You are an expert QA engineer specializing in web scraping applications. Focus on: 1) Testing data integrity and completeness, 2) Validating retry mechanisms and error handling, 3) Performance testing under various load conditions";
        testerAgent.specializations = ['testing', 'scraping', 'data-validation', 'performance'];
        testerAgent.capabilities = ['test_automation', 'quality_assurance', 'performance_testing'];
        
        console.log(`  👨‍💻 Created: ${coderAgent.name} (${coderAgent.specializations.join(', ')})`);
        console.log(`  🧪 Created: ${testerAgent.name} (${testerAgent.specializations.join(', ')})`);
        
        return { coderAgent, testerAgent };
    }

    async demonstrateChatProcessing() {
        console.log('\n💬 Processing Development Chat Messages...');
        console.log('─'.repeat(40));
        
        // Simulate a development conversation with structured commands
        const chatMessage = `
I need to create some new tasks for the web scraper project:

<todozi>Implement rate limiting middleware;1 hour;high;web_scraper_project;todo;ai;backend,api;main_task;</todozi>

<idea>Use a queue system for managing concurrent scraping requests;share;high</idea>

<memory>Encountered 429 errors when scraping too aggressively;standard;Need to implement better rate limiting;Rate limiting is crucial for respectful scraping;high;long;scraping,errors</memory>

<todozi_agent>scraper_coder;TASK-123;web_scraper_project</todozi_agent>

Also, I should add error tracking:
<error>Rate limit exceeded;Getting 429 errors from target site;high;integration;scraping_service;Need to implement exponential backoff;scraping,rate-limiting</error>
        `;

        const processedContent = processChatMessageExtended(chatMessage, this.userId);
        
        // Process each type of extracted content
        for (const task of processedContent.tasks) {
            await this.storage.addTaskToProject(task);
            await this.embeddingService.addTask(task);
            console.log(`  📝 Extracted task: ${task.action}`);
        }
        
        for (const idea of processedContent.ideas) {
            console.log(`  💡 Extracted idea: ${idea.idea}`);
            // In a real implementation, you'd save this to storage
        }
        
        for (const memory of processedContent.memories) {
            console.log(`  🧠 Extracted memory: ${memory.moment} - ${memory.meaning}`);
            // In a real implementation, you'd save this to storage
        }
        
        for (const agentAssignment of processedContent.agent_assignments) {
            console.log(`  🤖 Agent assignment: ${agentAssignment.agent_id} → ${agentAssignment.task_id}`);
            // In a real implementation, you'd save this to storage
        }
        
        for (const error of processedContent.errors) {
            console.log(`  ❌ Extracted error: ${error.title} (${error.severity})`);
            // In a real implementation, you'd save this to storage
        }
        
        return processedContent;
    }

    async demonstrateSemanticSearch() {
        console.log('\n🔍 Semantic Search Examples...');
        console.log('─'.repeat(40));
        
        // Search for tasks related to database operations
        const dbResults = await this.embeddingService.findSimilarTasks(
            'database schema design and SQL operations',
            3
        );
        console.log('🔎 Searching for "database schema design and SQL operations":');
        dbResults.forEach((result, index) => {
            console.log(`  ${index + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
        });
        
        // Search for API-related tasks
        const apiResults = await this.embeddingService.findSimilarTasks(
            'REST API endpoints and data retrieval',
            3
        );
        console.log('\n🔎 Searching for "REST API endpoints and data retrieval":');
        apiResults.forEach((result, index) => {
            console.log(`  ${index + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
        });
        
        return { dbResults, apiResults };
    }

    async demonstrateTaskExecution() {
        console.log('\n⚡ Task Execution Workflow...');
        console.log('─'.repeat(40));
        
        // Get all tasks for the project
        const allTasks = await this.storage.listTasksAcrossProjects({
            project: this.projectName
        });
        
        console.log(`📋 Found ${allTasks.length} tasks in project "${this.projectName}"`);
        
        // Execute a high-priority task with AI agent
        const highPriorityTask = allTasks.find(task => task.priority === Priority.High);
        if (highPriorityTask) {
            console.log(`\n🎯 Executing high-priority task: ${highPriorityTask.action}`);
            
            // Assign to AI for processing
            highPriorityTask.assignee = 'ai';
            await this.storage.updateTaskInProject(highPriorityTask.id, {
                status: Status.InProgress,
                assignee: 'ai'
            });
            
            // Simulate task execution
            const executionResult = await executeTask(this.storage, highPriorityTask);
            console.log(`  ⚙️ ${executionResult}`);
            
            // Mark as completed
            await this.storage.completeTaskInProject(highPriorityTask.id);
            console.log(`  ✅ Task completed: ${highPriorityTask.action}`);
        }
    }

    async demonstrateCodeChunking() {
        console.log('\n🧩 Code Chunking for Development...');
        console.log('─'.repeat(40));
        
        // Import chunking functionality
        // Note: chunking module would be imported at top if needed
        // import { CodeGenerationGraph } from '../todozi/chunking.js';
        
        // Create a code generation graph
        const graph = new chunking.CodeGenerationGraph(1000);
        
        // Add chunks for different development phases
        graph.addChunk('setup', chunking.ChunkingLevel.Module, []);
        graph.addChunk('database', chunking.ChunkingLevel.Class, ['setup']);
        graph.addChunk('http_client', chunking.ChunkingLevel.Class, ['setup']);
        graph.addChunk('parser', chunking.ChunkingLevel.Class, ['http_client']);
        graph.addChunk('api_routes', chunking.ChunkingLevel.Method, ['database', 'parser']);
        
        console.log('📊 Project structure created with dependencies:');
        console.log('  setup → database, http_client');
        console.log('  http_client → parser');
        console.log('  database, parser → api_routes');
        
        // Update some chunks with code
        graph.updateChunkCode('setup', `
// Project setup and dependencies

const app = express();
app.use(express.json());
        `);
        
        graph.updateChunkCode('database', `
// Database connection and schema
class DatabaseConnection {
    constructor() {
        this.db = new sqlite3.Database('./scraper.db');
        this.createTables();
    }
    
    createTables() {
        const createUrlsTable = \`
            CREATE TABLE IF NOT EXISTS urls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT UNIQUE NOT NULL,
                status TEXT DEFAULT 'pending',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        \`;
        
        this.db.run(createUrlsTable);
    }
}
        `);
        
        // Mark setup as completed
        graph.markChunkCompleted('setup');
        
        console.log('💾 Code chunks populated and setup marked as completed');
        
        return graph;
    }

    async demonstrateClustering() {
        console.log('\n🔗 Content Clustering Analysis...');
        console.log('─'.repeat(40));
        
        // Get all tasks and ideas for clustering
        const allTasks = await this.storage.listTasksAcrossProjects({});
        const taskContents = allTasks.map(task => 
            `Task: ${task.action}\nPriority: ${task.priority}\nStatus: ${task.status}`
        );
        
        // Perform clustering
        const clusters = await this.embeddingService.clusterContent();
        
        if (clusters.length > 0) {
            console.log(`🔍 Found ${clusters.length} semantic clusters:`);
            clusters.forEach((cluster, index) => {
                console.log(`\n  Cluster ${index + 1} (${cluster.clusterSize} items):`);
                cluster.contentItems.forEach(item => {
                    const preview = item.textContent.split('\n')[0];
                    console.log(`    • ${preview.substring(0, 60)}...`);
                });
            });
        } else {
            console.log('ℹ️ No significant clusters found (items may be too diverse)');
        }
        
        return clusters;
    }

    async demonstrateProgressTracking() {
        console.log('\n📈 Project Progress Tracking...');
        console.log('─'.repeat(40));
        
        // Get project statistics
        const allTasks = await this.storage.listTasksAcrossProjects({
            project: this.projectName
        });
        
        const completedTasks = allTasks.filter(task => 
            task.status === Status.Done
        );
        const inProgressTasks = allTasks.filter(task => 
            task.status === Status.InProgress
        );
        const todoTasks = allTasks.filter(task => 
            task.status === Status.Todo
        );
        
        const completionRate = allTasks.length > 0 
            ? (completedTasks.length / allTasks.length) * 100 
            : 0;
        
        console.log(`📊 Project "${this.projectName}" Progress:`);
        console.log(`  📋 Total tasks: ${allTasks.length}`);
        console.log(`  ✅ Completed: ${completedTasks.length}`);
        console.log(`  🔄 In Progress: ${inProgressTasks.length}`);
        console.log(`  📝 Todo: ${todoTasks.length}`);
        console.log(`  📈 Completion rate: ${completionRate.toFixed(1)}%`);
        
        // Show priority breakdown
        const priorityCounts = allTasks.reduce((acc, task) => {
            acc[task.priority] = (acc[task.priority] || 0) + 1;
            return acc;
        }, {});
        
        console.log('\n🎯 Priority Distribution:');
        Object.entries(priorityCounts).forEach(([priority, count]) => {
            console.log(`  ${priority}: ${count} tasks`);
        });
        
        return {
            total: allTasks.length,
            completed: completedTasks.length,
            inProgress: inProgressTasks.length,
            todo: todoTasks.length,
            completionRate: completionRate
        };
    }

    async runCompleteExample() {
        try {
            console.log('🎯 TODOZI DEVELOPMENT WORKFLOW EXAMPLE');
            console.log('═'.repeat(60));
            console.log('This example demonstrates a complete software development workflow');
            console.log('using Todozi for task management, AI assistance, and semantic search.\n');
            
            // Initialize the system
            await this.initialize();
            
            // Create development tasks
            await this.demonstrateTaskCreation();
            
            // Create specialized agents
            const agents = await this.demonstrateAgentCreation();
            
            // Process chat messages with structured content
            await this.demonstrateChatProcessing();
            
            // Demonstrate semantic search
            await this.demonstrateSemanticSearch();
            
            // Execute tasks with AI assistance
            await this.demonstrateTaskExecution();
            
            // Show code chunking for development
            const graph = await this.demonstrateCodeChunking();
            
            // Analyze content clustering
            await this.demonstrateClustering();
            
            // Track project progress
            const progress = await this.demonstrateProgressTracking();
            
            console.log('\n🎉 Development Workflow Example Completed!');
            console.log('═'.repeat(60));
            console.log('📋 Summary:');
            console.log(`  • Created and managed development tasks`);
            console.log(`  • Set up AI agents for specialized work`);
            console.log(`  • Processed natural language into structured data`);
            console.log(`  • Used semantic search to find related tasks`);
            console.log(`  • Executed tasks with AI assistance`);
            console.log(`  • Organized code into dependency-based chunks`);
            console.log(`  • Analyzed content for semantic clustering`);
            console.log(`  • Tracked project progress: ${progress.completionRate.toFixed(1)}% complete`);
            
            console.log('\n💡 Key Benefits Demonstrated:');
            console.log('  • Natural language task creation via chat');
            console.log('  • AI-powered task execution and assistance');
            console.log('  • Semantic search for finding related work');
            console.log('  • Structured code organization with dependencies');
            console.log('  • Progress tracking and project analytics');
            console.log('  • Multi-agent collaboration for complex tasks');
            
        } catch (error) {
            console.error('❌ Error in development workflow example:', error);
        }
    }
}

// Example usage and demonstration
async function main() {
    const workflow = new DevelopmentWorkflowExample();
    await workflow.runCompleteExample();
}

// Export for use in other modules
    DevelopmentWorkflowExample,
    main
};

// Run the example if this file is executed directly
// Run if executed directly
    main().catch(console.error);
}

/*
I'll create a comprehensive practical example that demonstrates how to use the Todozi system for managing a software development project. This example will show the integration of multiple components working together.

/ *
bash
# example5_usage.sh
#!/bin/bash

# Example 5: Complete Development Workflow
# This script demonstrates the Todozi development workflow example

echo "🚀 Running Todozi Development Workflow Example..."
echo "=================================================="

# Navigate to the project directory (adjust path as needed)
cd /path/to/your/todozi/project

# Run the development workflow example
node example5_development_workflow.js

echo ""
echo "📊 Example completed successfully!"
echo "💡 This example demonstrated:"
echo "  • Complete development task management"
echo "  • AI agent collaboration"
echo "  • Semantic search capabilities"
echo "  • Code chunking and dependencies"
echo "  • Progress tracking and analytics"
*/