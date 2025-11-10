import { processChatMessageExtended, executeTask, processWorkflow, parseTodoziFormat } from '../todozi/todozi.js';

import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { Storage } from '../todozi/storage.js';
import { TodoziHandler } from '../todozi/cli.js';
import { fileURLToPath } from 'url';
import path from 'path';
import fs from 'fs/promises';
import {
    Task, 
    Agent, 
    Memory, 
    Idea, 
    Priority, 
    Status, 
    Assignee 
} from '../todozi/models.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

class TodoziProjectManager {
    constructor() {
        this.embeddingService = null;
        this.storage = null;
        this.handler = null;
        this.projectAgents = new Map();
    }

    async initialize() {
        // Initialize storage
        this.storage = await Storage.new();
        
        // Initialize embedding service with custom configuration
        const config = new TodoziEmbeddingConfig();
        config.model_name = "sentence-transformers/all-mpnet-base-v2";
        config.dimensions = 768;
        config.similarity_threshold = 0.65;
        config.enable_clustering = true;
        
        this.embeddingService = await TodoziEmbeddingService.new(config);
        
        // Initialize CLI handler
        this.handler = await TodoziHandler.new(this.storage);
        
        // Setup project-specific agents
        await this.setupProjectAgents();
        
        console.log('✅ Todozi Project Manager initialized');
    }

    async setupProjectAgents() {
        // Create specialized agents for the project
        const frontendDev = Agent.new('frontend-dev', 'Frontend Developer', 
            'Specializes in React, TypeScript, and UI/UX implementation');
        frontendDev.capabilities = ['react', 'typescript', 'css', 'testing'];
        frontendDev.specializations = ['frontend', 'ui', 'ux'];
        
        const backendDev = Agent.new('backend-dev', 'Backend Developer',
            'Specializes in Node.js, databases, and API development');
        backendDev.capabilities = ['nodejs', 'postgresql', 'rest-api', 'graphql'];
        backendDev.specializations = ['backend', 'database', 'api'];
        
        const devopsEng = Agent.new('devops-eng', 'DevOps Engineer',
            'Specializes in CI/CD, cloud infrastructure, and monitoring');
        devopsEng.capabilities = ['docker', 'kubernetes', 'aws', 'monitoring'];
        devopsEng.specializations = ['infrastructure', 'deployment', 'monitoring'];
        
        // Save agents
        await this.saveAgent(frontendDev);
        await this.saveAgent(backendDev);
        await this.saveAgent(devopsEng);
        
        this.projectAgents.set('frontend-dev', frontendDev);
        this.projectAgents.set('backend-dev', backendDev);
        this.projectAgents.set('devops-eng', devopsEng);
        
        console.log('🤖 Project agents created and configured');
    }

    async saveAgent(agent) {
        await fs.mkdir(agentsDir, { recursive: true });
        await fs.writeFile(agentPath, JSON.stringify(agent, null, 2));
    }

    async processDevelopmentSprint(sprintName, userMessages) {
        console.log(`\n🚀 Processing sprint: ${sprintName}`);
        console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
        
        const allTasks = [];
        const allMemories = [];
        const allIdeas = [];
        const allErrors = [];
        
        // Process each user message
        for (const message of userMessages) {
            console.log(`\n📝 Processing message: "${message.substring(0, 50)}..."`);
            
            // Extract structured content
            const content = processChatMessageExtended(message, 'project_manager');
            
            // Process and embed each type of content
            if (content.tasks.length > 0) {
                for (const task of content.tasks) {
                    await this.processIntelligentTask(task);
                    allTasks.push(task);
                }
            }
            
            if (content.memories.length > 0) {
                for (const memory of content.memories) {
                    await this.embeddingService.embedMemory(memory);
                    allMemories.push(memory);
                }
            }
            
            if (content.ideas.length > 0) {
                for (const idea of content.ideas) {
                    await this.embeddingService.embedIdea(idea);
                    allIdeas.push(idea);
                }
            }
            
            if (content.errors.length > 0) {
                for (const error of content.errors) {
                    allErrors.push(error);
                }
            }
        }
        
        // Generate semantic clusters for better organization
        console.log('\n🔗 Analyzing semantic relationships...');
        const clusters = await this.embeddingService.clusterContent();
        console.log(`📊 Found ${clusters.length} semantic clusters in the sprint`);
        
        // Provide sprint summary
        await this.generateSprintSummary(sprintName, allTasks, allMemories, allIdeas, allErrors, clusters);
        
        return {
            tasks: allTasks,
            memories: allMemories,
            ideas: allIdeas,
            errors: allErrors,
            clusters
        };
    }

    async processIntelligentTask(task) {
        // Determine the best agent for this task
        const assignedAgent = await this.assignTaskToAgent(task);
        
        // Update task with assignee
        if (assignedAgent) {
            task.assignee = `agent:${assignedAgent.id}`;
            console.log(`🤖 Task "${task.action}" assigned to ${assignedAgent.name}`);
        }
        
        // Add task to storage with embedding
        await this.storage.addTaskToProject(task);
        
        // Generate embedding for semantic search
        const taskContent = this.embeddingService.prepareTaskContent(task);
        const embedding = await this.embeddingService.generateEmbedding(taskContent);
        task.embedding_vector = embedding;
        
        // Execute task workflow
        const result = await executeTask(this.storage, task);
        console.log(`✅ Task processed: ${result}`);
    }

    async assignTaskToAgent(task) {
        // Use semantic similarity to find the best agent
        const taskEmbedding = await this.embeddingService.generateEmbedding(task.action);
        
        let bestAgent = null;
        let bestScore = 0;
        
        for (const agent of this.projectAgents.values()) {
            // Calculate similarity between task and agent capabilities
            const agentDescription = `${agent.name} ${agent.description} ${agent.capabilities.join(' ')}`;
            const agentEmbedding = await this.embeddingService.generateEmbedding(agentDescription);
            
            const similarity = this.embeddingService.cosineSimilarity(taskEmbedding, agentEmbedding);
            
            if (similarity > bestScore && similarity > 0.3) {
                bestScore = similarity;
                bestAgent = agent;
            }
        }
        
        return bestAgent;
    }

    async findSimilarTasks(query, limit = 5) {
        console.log(`\n🔍 Searching for tasks similar to: "${query}"`);
        
        const similarTasks = await this.embeddingService.findSimilarTasks(query, limit);
        
        if (similarTasks.length === 0) {
            console.log('❌ No similar tasks found');
            return [];
        }
        
        console.log(`\n📋 Found ${similarTasks.length} similar tasks:`);
        for (let i = 0; i < similarTasks.length; i++) {
            const result = similarTasks[i];
            console.log(`  ${i + 1}. [${result.content_id}] ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
        }
        
        return similarTasks;
    }

    async generateSprintSummary(sprintName, tasks, memories, ideas, errors, clusters) {
        console.log(`\n📊 Sprint Summary: ${sprintName}`);
        console.log('══════════════════════════════════════════════━━━━');
        
        // Task statistics
        const tasksByPriority = {
            [Priority.Low]: tasks.filter(t => t.priority === Priority.Low).length,
            [Priority.Medium]: tasks.filter(t => t.priority === Priority.Medium).length,
            [Priority.High]: tasks.filter(t => t.priority === Priority.High).length,
            [Priority.Critical]: tasks.filter(t => t.priority === Priority.Critical).length
        };
        
        const tasksByStatus = {
            [Status.Todo]: tasks.filter(t => t.status === Status.Todo).length,
            [Status.InProgress]: tasks.filter(t => t.status === Status.InProgress).length,
            [Status.Done]: tasks.filter(t => t.status === Status.Done).length,
            [Status.Blocked]: tasks.filter(t => t.status === Status.Blocked).length
        };
        
        console.log('\n📋 Task Breakdown:');
        console.log(`  Total tasks: ${tasks.length}`);
        console.log(`  By priority: Low(${tasksByPriority[Priority.Low]}) Medium(${tasksByPriority[Priority.Medium]}) High(${tasksByPriority[Priority.High]}) Critical(${tasksByPriority[Priority.Critical]})`);
        console.log(`  By status: Todo(${tasksByStatus[Status.Todo]}) In Progress(${tasksByStatus[Status.InProgress]}) Done(${tasksByStatus[Status.Done]}) Blocked(${tasksByStatus[Status.Blocked]})`);
        
        // Agent assignments
        const agentAssignments = new Map();
        for (const task of tasks) {
            if (task.assignee && task.assignee.startsWith('agent:')) {
                const agentId = task.assignee.substring(6);
                agentAssignments.set(agentId, (agentAssignments.get(agentId) || 0) + 1);
            }
        }
        
        if (agentAssignments.size > 0) {
            console.log('\n🤖 Agent Assignments:');
            for (const [agentId, count] of agentAssignments.entries()) {
                const agent = this.projectAgents.get(agentId);
                console.log(`  ${agent ? agent.name : agentId}: ${count} tasks`);
            }
        }
        
        // Content summary
        console.log('\n📝 Content Summary:');
        console.log(`  Memories: ${memories.length}`);
        console.log(`  Ideas: ${ideas.length}`);
        console.log(`  Errors: ${errors.length}`);
        console.log(`  Semantic Clusters: ${clusters.length}`);
        
        // Top similar tasks based on criticality
        const criticalTasks = tasks.filter(t => t.priority === Priority.Critical);
        if (criticalTasks.length > 0) {
            console.log('\n🚨 Critical Tasks:');
            for (const task of criticalTasks.slice(0, 3)) {
                console.log(`  • ${task.action} (Assigned to: ${task.assignee || 'Unassigned'})`);
            }
        }
    }

    async exportProjectData(outputFile) {
        console.log(`\n💾 Exporting project data to ${outputFile}`);
        
        const data = {
            timestamp: new Date().toISOString(),
            project: {
                name: 'Software Development Project',
                agents: Array.from(this.projectAgents.values()).map(a => ({
                    id: a.id,
                    name: a.name,
                    capabilities: a.capabilities,
                    specializations: a.specializations
                }))
            },
            statistics: await this.embeddingService.getStats(),
            embedding_config: {
                model: this.embeddingService.config.model_name,
                dimensions: this.embeddingService.config.dimensions,
                similarity_threshold: this.embeddingService.config.similarity_threshold
            }
        };
        
        await fs.writeFile(outputFile, JSON.stringify(data, null, 2));
        console.log('✅ Project data exported successfully');
    }
}

// Usage Example
async function runProjectManagementExample() {
    const projectManager = new TodoziProjectManager();
    await projectManager.initialize();
    
    // Simulate a sprint planning session with multiple team members
    const sprintMessages = [
        `<todozi>Implement user authentication with JWT; 2 days; high; authentication; todo; agent:backend-dev;security,jwt;user-service;</todozi>`,
        `<memory>Team decided to use OAuth2 for third-party authentication;OAuth2 provides better security and user experience;Security team recommendation;high;short;auth,strategy;</memory>`,
        `<idea>Implement biometric authentication for mobile app;Could use fingerprint or face recognition;high;security,innovation;</idea>`,
        `<todozi>Create responsive login page design; 1 day; medium; frontend; todo; agent:frontend-dev;ui,design;login-page;</todozi>`,
        `<error>Database connection pool exhaustion during load testing;Connection limit reached under 1000 concurrent users;critical;database;connection-pool;performance,scaling;</error>`,
        `<todozi>Optimize database queries for user profile loading; 3 days; critical; backend; in_progress; agent:backend-dev;performance,database;query-optimization;</todozi>`,
        `<memory>Client requires GDPR compliance for all user data;Legal requirement for European markets;critical;long;gdpr,compliance;</memory>`,
        `<todozi>Set up CI/CD pipeline with automated testing; 2 days; high; devops; todo; agent:devops-eng;ci,cd,testing;</todozi>`,
        `<idea>Use feature flags for gradual rollout of new authentication system;Reduces risk of deployment issues;medium;deployment,features;</idea>`,
        `<todozi>Write unit tests for authentication service; 1 day; medium; backend; todo; agent:backend-dev;testing,quality;jest,unit-tests;</todozi>`
    ];
    
    // Process the sprint
    const sprintResults = await projectManager.processDevelopmentSprint('Sprint 12', sprintMessages);
    
    // Demonstrate semantic search capabilities
    await projectManager.findSimilarTasks('database optimization');
    await projectManager.findSimilarTasks('user interface design');
    
    // Export project data for analysis
    await projectManager.exportProjectData('project_export.json');
    
    console.log('\n🎉 Project management workflow completed successfully!');
}

// Run the example
runProjectManagementExample().catch(console.error);


/*
# Example 3: Advanced Todozi Workflow with Semantic Search and Agent Collaboration

This example demonstrates a comprehensive Todozi workflow that integrates semantic search, agent collaboration, and intelligent task management for a software development project.

## Key Features Demonstrated:

1. **Intelligent Task Assignment**: Uses semantic similarity to automatically assign tasks to the most suitable agents based on their capabilities and specializations.

2. **Semantic Search Integration**: Leverages the embedding service to find similar tasks and organize content into meaningful clusters.

3. **Multi-Modal Content Processing**: Handles tasks, memories, ideas, and errors in a unified workflow.

4. **Agent Collaboration**: Sets up specialized agents (frontend, backend, DevOps) with specific capabilities and automatically routes tasks accordingly.

5. **Sprint Management**: Processes multiple user messages to simulate team collaboration during sprint planning.

6. **Data Export**: Provides comprehensive project analytics and data export for further analysis.

7. **Error Tracking**: Integrates error reporting with priority-based task management.

## Usage:

/ *
bash
# Run the project management example
node example3.js
*/