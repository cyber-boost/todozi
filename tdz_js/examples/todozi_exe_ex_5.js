// example5_practical_usage.js
/**
import { v4: uuidv4 } from 'uuid.js';

 * Practical Example 5: Todozi System Usage
 * 
 * This example demonstrates how to use the Todozi system for:
 * - Task creation and management
 * - AI-powered task assignment
 * - Semantic search with embeddings
 * - Project organization
 * - Agent coordination
 */


// Simulated implementation of core Todozi functionality
class TodoziSystem {
    constructor() {
        this.tasks = new Map();
        this.projects = new Map();
        this.agents = new Map();
        this.memories = new Map();
        this.ideas = new Map();
        this.embeddings = new Map();
        this.taskIdCounter = 1;
    }

    /**
     * Initialize the Todozi system with default projects and agents
     */
    async initialize() {
        console.log('🚀 Initializing Todozi System...');
        
        // Create default projects
        await this.createProject('personal', 'Personal tasks and goals');
        await this.createProject('work', 'Work-related projects');
        await this.createProject('learning', 'Learning and skill development');
        
        // Create default agents
        await this.createAgent('planner', 'Project Planning Agent', 'strategic_planning');
        await this.createAgent('coder', 'Development Agent', 'software_development');
        await this.createAgent('researcher', 'Research Agent', 'information_gathering');
        await this.createAgent('reviewer', 'Quality Review Agent', 'quality_assurance');
        
        console.log('✅ System initialized successfully!');
    }

    /**
     * Create a new project
     */
    async createProject(name, description) {
        const project = {
            id: `proj_${uuidv4().substring(0, 8)}`,
            name,
            description,
            createdAt: new Date().toISOString(),
            taskCount: 0
        };
        this.projects.set(project.id, project);
        console.log(`📁 Project created: ${name}`);
        return project;
    }

    /**
     * Create a new agent
     */
    async createAgent(id, name, specialty) {
        const agent = {
            id,
            name,
            specialty,
            status: 'available',
            capabilities: this.getCapabilitiesForSpecialty(specialty),
            createdAt: new Date().toISOString()
        };
        this.agents.set(id, agent);
        console.log(`🤖 Agent created: ${name} (${specialty})`);
        return agent;
    }

    /**
     * Get capabilities for a specialty
     */
    getCapabilitiesForSpecialty(specialty) {
        const capabilities = {
            'strategic_planning': ['task_breakdown', 'timeline_estimation', 'resource_planning'],
            'software_development': ['coding', 'debugging', 'code_review', 'testing'],
            'information_gathering': ['research', 'data_analysis', 'summarization'],
            'quality_assurance': ['testing', 'review', 'validation', 'optimization']
        };
        return capabilities[specialty] || ['general_assistance'];
    }

    /**
     * Create a task with automatic AI assignment
     */
    async createTask(params) {
        const { content, priority = 'medium', projectName, context, estimatedTime } = params;
        
        console.log(`\n📝 Creating task: "${content}"`);
        
        // Determine the best agent for this task
        const assignedAgent = this.determineBestAgent(content, context);
        
        // Create the task
        const task = {
            id: `task_${this.taskIdCounter++}`,
            content,
            priority,
            projectName,
            context,
            estimatedTime,
            status: 'created',
            assignedAgent: assignedAgent.name,
            assigneeType: assignedAgent.type, // 'ai', 'human', 'collaborative'
            createdAt: new Date().toISOString(),
            embedding: await this.generateEmbedding(content + ' ' + (context || ''))
        };
        
        this.tasks.set(task.id, task);
        this.embeddings.set(task.id, task.embedding);
        
        // Update project task count
        const project = Array.from(this.projects.values()).find(p => p.name === projectName);
        if (project) {
            project.taskCount++;
        }
        
        console.log(`✅ Task created: ${task.id}`);
        console.log(`   Priority: ${priority}`);
        console.log(`   Project: ${projectName}`);
        console.log(`   Assigned to: ${assignedAgent.name} (${assignedAgent.type})`);
        console.log(`   Estimated time: ${estimatedTime || 'Not specified'}`);
        
        return task;
    }

    /**
     * Determine the best agent for a task based on content analysis
     */
    determineBestAgent(content, context) {
        const content_lower = (content + ' ' + (context || '')).toLowerCase();
        
        // Check for keywords that indicate specific agent needs
        if (content_lower.includes('code') || content_lower.includes('develop') || 
            content_lower.includes('program') || content_lower.includes('bug') ||
            content_lower.includes('feature') || content_lower.includes('api')) {
            return { name: 'Coder Agent', type: 'ai' };
        }
        
        if (content_lower.includes('plan') || content_lower.includes('strategy') ||
            content_lower.includes('timeline') || content_lower.includes('roadmap')) {
            return { name: 'Planner Agent', type: 'ai' };
        }
        
        if (content_lower.includes('research') || content_lower.includes('analyze') ||
            content_lower.includes('investigate') || content_lower.includes('study')) {
            return { name: 'Researcher Agent', type: 'ai' };
        }
        
        if (content_lower.includes('review') || content_lower.includes('check') ||
            content_lower.includes('test') || content_lower.includes('validate')) {
            return { name: 'Reviewer Agent', type: 'ai' };
        }
        
        // Default to human for ambiguous tasks
        return { name: 'Human', type: 'human' };
    }

    /**
     * Generate a simple embedding (in real implementation, this would use actual ML models)
     */
    async generateEmbedding(text) {
        // Simple hash-based embedding for demonstration
        const words = text.toLowerCase().split(/\s+/);
        const embedding = new Array(384).fill(0);
        
        words.forEach((word, index) => {
            const hash = this.simpleHash(word);
            embedding[hash % 384] += 1 / (index + 1);
        });
        
        // Normalize
        const norm = Math.sqrt(embedding.reduce((sum, val) => sum + val * val, 0));
        return embedding.map(val => val / (norm || 1));
    }

    /**
     * Simple hash function
     */
    simpleHash(str) {
        let hash = 0;
        for (let i = 0; i < str.length; i++) {
            const char = str.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash; // Convert to 32-bit integer
        }
        return Math.abs(hash);
    }

    /**
     * Perform semantic search across tasks
     */
    async semanticSearch(query, options = {}) {
        const { limit = 5, projectName, minSimilarity = 0.1 } = options;
        
        console.log(`\n🔍 Performing semantic search for: "${query}"`);
        
        const queryEmbedding = await this.generateEmbedding(query);
        const results = [];
        
        for (const [taskId, task] of this.tasks.entries()) {
            if (projectName && task.projectName !== projectName) {
                continue;
            }
            
            const taskEmbedding = this.embeddings.get(taskId);
            if (!taskEmbedding) continue;
            
            const similarity = this.cosineSimilarity(queryEmbedding, taskEmbedding);
            
            if (similarity >= minSimilarity) {
                results.push({
                    task,
                    similarity,
                    matchedContent: this.findMatchedContent(task, query)
                });
            }
        }
        
        results.sort((a, b) => b.similarity - a.similarity);
        const topResults = results.slice(0, limit);
        
        console.log(`Found ${results.length} similar tasks (showing top ${topResults.length}):`);
        topResults.forEach((result, index) => {
            console.log(`  ${index + 1}. [${(result.similarity * 100).toFixed(1)}% similar] ${result.task.content}`);
            console.log(`     Project: ${result.task.projectName} | Priority: ${result.task.priority}`);
            console.log(`     Matched: "${result.matchedContent}"\n`);
        });
        
        return topResults;
    }

    /**
     * Find which part of the task content matched the query
     */
    findMatchedContent(task, query) {
        const content = (task.content + ' ' + (task.context || '')).toLowerCase();
        const queryWords = query.toLowerCase().split(/\s+/);
        
        for (const word of queryWords) {
            if (word.length > 2 && content.includes(word)) {
                return word;
            }
        }
        
        return 'semantic similarity';
    }

    /**
     * Calculate cosine similarity between two vectors
     */
    cosineSimilarity(a, b) {
        let dotProduct = 0;
        let normA = 0;
        let normB = 0;
        
        for (let i = 0; i < Math.min(a.length, b.length); i++) {
            dotProduct += a[i] * b[i];
            normA += a[i] * a[i];
            normB += b[i] * b[i];
        }
        
        normA = Math.sqrt(normA);
        normB = Math.sqrt(normB);
        
        if (normA === 0 || normB === 0) return 0;
        return dotProduct / (normA * normB);
    }

    /**
     * Create a memory (for AI context retention)
     */
    async createMemory(moment, meaning, importance = 'medium') {
        const memory = {
            id: `mem_${uuidv4().substring(0, 8)}`,
            moment,
            meaning,
            importance,
            createdAt: new Date().toISOString(),
            embedding: await this.generateEmbedding(moment + ' ' + meaning)
        };
        
        this.memories.set(memory.id, memory);
        console.log(`🧠 Memory created: ${moment} → ${meaning}`);
        return memory;
    }

    /**
     * Create an idea
     */
    async createIdea(content, importance = 'medium', shareLevel = 'private') {
        const idea = {
            id: `idea_${uuidv4().substring(0, 8)}`,
            content,
            importance,
            shareLevel,
            createdAt: new Date().toISOString(),
            embedding: await this.generateEmbedding(content)
        };
        
        this.ideas.set(idea.id, idea);
        console.log(`💡 Idea created: ${content}`);
        return idea;
    }

    /**
     * Get system statistics
     */
    getStats() {
        const stats = {
            totalTasks: this.tasks.size,
            totalProjects: this.projects.size,
            totalAgents: this.agents.size,
            totalMemories: this.memories.size,
            totalIdeas: this.ideas.size,
            tasksByProject: {},
            tasksByPriority: {},
            tasksByAssignee: {}
        };
        
        // Analyze tasks
        for (const task of this.tasks.values()) {
            // By project
            stats.tasksByProject[task.projectName] = (stats.tasksByProject[task.projectName] || 0) + 1;
            
            // By priority
            stats.tasksByPriority[task.priority] = (stats.tasksByPriority[task.priority] || 0) + 1;
            
            // By assignee
            stats.tasksByAssignee[task.assignedAgent] = (stats.tasksByAssignee[task.assignedAgent] || 0) + 1;
        }
        
        return stats;
    }

    /**
     * Display comprehensive system overview
     */
    async displayOverview() {
        console.log('\n' + '='.repeat(60));
        console.log('📊 TODOZI SYSTEM OVERVIEW');
        console.log('='.repeat(60));
        
        const stats = this.getStats();
        
        console.log(`📁 Projects: ${stats.totalProjects}`);
        for (const [project, count] of Object.entries(stats.tasksByProject)) {
            console.log(`   • ${project}: ${count} tasks`);
        }
        
        console.log(`\n🤖 Agents: ${stats.totalAgents}`);
        for (const agent of this.agents.values()) {
            console.log(`   • ${agent.name} (${agent.specialty}) - ${agent.status}`);
        }
        
        console.log(`\n📝 Tasks: ${stats.totalTasks} total`);
        console.log('   By Priority:');
        for (const [priority, count] of Object.entries(stats.tasksByPriority)) {
            console.log(`   • ${priority}: ${count}`);
        }
        console.log('   By Assignee:');
        for (const [assignee, count] of Object.entries(stats.tasksByAssignee)) {
            console.log(`   • ${assignee}: ${count}`);
        }
        
        console.log(`\n🧠 Memories: ${stats.totalMemories}`);
        console.log(`💡 Ideas: ${stats.totalIdeas}`);
        
        console.log('='.repeat(60));
    }
}

/**
 * Main example function
 */
async function runTodoziExample5() {
    console.log('🎯 Todozi Example 5: Practical Usage Demonstration\n');
    
    // Initialize the system
    const todozi = new TodoziSystem();
    await todozi.initialize();
    
    // Create some tasks
    console.log('\n📋 Creating sample tasks...');
    
    await todozi.createTask({
        content: "Implement user authentication system with JWT tokens",
        priority: "high",
        projectName: "work",
        context: "Need secure login system for web application",
        estimatedTime: "3-5 days"
    });
    
    await todozi.createTask({
        content: "Research best practices for React performance optimization",
        priority: "medium",
        projectName: "learning",
        context: "Improve app loading times and user experience",
        estimatedTime: "1-2 days"
    });
    
    await todozi.createTask({
        content: "Plan Q1 product roadmap and feature prioritization",
        priority: "high",
        projectName: "work",
        context: "Strategic planning for next quarter",
        estimatedTime: "2-3 days"
    });
    
    await todozi.createTask({
        content: "Fix memory leak in data processing module",
        priority: "urgent",
        projectName: "work",
        context: "Performance issue causing system crashes",
        estimatedTime: "1 day"
    });
    
    await todozi.createTask({
        content: "Organize personal finance tracking spreadsheet",
        priority: "low",
        projectName: "personal",
        context: "Track monthly expenses and budget",
        estimatedTime: "2 hours"
    });
    
    // Create memories and ideas
    console.log('\n🧠 Creating memories...');
    await todozi.createMemory(
        "Team prefers morning standup meetings",
        "Most productive time for team synchronization",
        "medium"
    );
    
    await todozi.createMemory(
        "User feedback indicates need for mobile app",
        "Market research shows mobile-first approach needed",
        "high"
    );
    
    console.log('\n💡 Creating ideas...');
    await todozi.createIdea(
        "Develop AI-powered task prioritization system",
        "high",
        "team"
    );
    
    await todozi.createIdea(
        "Create automated testing framework for legacy code",
        "medium",
        "private"
    );
    
    // Demonstrate semantic search
    console.log('\n' + '-'.repeat(60));
    await todozi.semanticSearch("performance optimization", { limit: 3 });
    
    await todozi.semanticSearch("planning and strategy", { projectName: "work" });
    
    await todozi.semanticSearch("authentication security", { limit: 2 });
    
    // Display system overview
    await todozi.displayOverview();
    
    console.log('\n✨ Example completed successfully!');
    console.log('\nKey Features Demonstrated:');
    console.log('• Project-based task organization');
    console.log('• AI-powered task assignment');
    console.log('• Semantic search with embeddings');
    console.log('• Memory creation for context retention');
    console.log('• Idea management');
    console.log('• Comprehensive statistics');
    
    return todozi;
}

// Run the example if this file is executed directly
// Run if executed directly
    runTodoziExample5()
        .then(todozi => {
            console.log(`\n🎉 Example 5 completed! System contains ${todozi.tasks.size} tasks.`);
        })
        .catch(error => {
            console.error('❌ Example failed:', error);
        });
}

    runTodoziExample5,
    TodoziSystem
};

/*
Based on the Todozi system code provided, I'll create a practical example that demonstrates how to use the core features. This example will show task creation, AI assignment, semantic search, and project management.

## How to Run This Example

/ *
bash
# Save the code as example5_practical_usage.js
# Run with Node.js
node example5_practical_usage.js
*/