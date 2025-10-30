//! JavaScript Client for Todozi Server
//!
//! This example shows how to interact with the Todozi REST API
//! from JavaScript/Node.js applications.

const BASE_URL = 'http://127.0.0.1:8636';

class TodoziClient {
    constructor(baseUrl = BASE_URL) {
        this.baseUrl = baseUrl;
    }

    async request(endpoint, options = {}) {
        const url = `${this.baseUrl}${endpoint}`;
        const response = await fetch(url, {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers
            },
            ...options
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        return await response.json();
    }

    // Health check
    async health() {
        return await this.request('/health');
    }

    // Task management
    async getTasks() {
        return await this.request('/tasks');
    }

    async createTask(task) {
        return await this.request('/tasks', {
            method: 'POST',
            body: JSON.stringify(task)
        });
    }

    async getTask(id) {
        return await this.request(`/tasks/${id}`);
    }

    async updateTask(id, task) {
        return await this.request(`/tasks/${id}`, {
            method: 'PUT',
            body: JSON.stringify(task)
        });
    }

    async deleteTask(id) {
        return await this.request(`/tasks/${id}`, {
            method: 'DELETE'
        });
    }

    // Memory management
    async getMemories() {
        return await this.request('/memories');
    }

    async createMemory(memory) {
        return await this.request('/memories', {
            method: 'POST',
            body: JSON.stringify(memory)
        });
    }

    // Idea management
    async getIdeas() {
        return await this.request('/ideas');
    }

    async createIdea(idea) {
        return await this.request('/ideas', {
            method: 'POST',
            body: JSON.stringify(idea)
        });
    }

    // Agent management
    async getAgents() {
        return await this.request('/agents');
    }

    async getAgent(id) {
        return await this.request(`/agents/${id}`);
    }

    // Chunking management
    async getChunks() {
        return await this.request('/chunks');
    }

    async createChunk(chunk) {
        return await this.request('/chunks', {
            method: 'POST',
            body: JSON.stringify(chunk)
        });
    }

    async getReadyChunks() {
        return await this.request('/chunks/ready');
    }

    async getChunkGraph() {
        return await this.request('/chunks/graph');
    }

    // Chat processing
    async processChat(message) {
        return await this.request('/chat/process', {
            method: 'POST',
            body: JSON.stringify({ message })
        });
    }

    // Project management
    async getProjects() {
        return await this.request('/projects');
    }

    async createProject(project) {
        return await this.request('/projects', {
            method: 'POST',
            body: JSON.stringify(project)
        });
    }
}

// Example usage
async function main() {
    const client = new TodoziClient();

    try {
        console.log('🚀 Todozi JavaScript Client Demo');
        console.log('📡 Connecting to port 8636 (TODO in dial language!)');
        console.log();

        // Health check
        console.log('🔍 Health check...');
        const health = await client.health();
        console.log('✅ Server status:', health);
        console.log();

        // Get agents
        console.log('🤖 Available agents...');
        const agents = await client.getAgents();
        console.log('Agents:', agents);
        console.log();

        // Create a task
        console.log('📝 Creating a task...');
        const task = {
            action: "Implement chunking system",
            time: "2 hours",
            priority: "high",
            project: "chunking-demo",
            status: "todo",
            assignee: "agent:coder",
            tags: ["chunking", "api", "demo"]
        };
        const createdTask = await client.createTask(task);
        console.log('✅ Task created:', createdTask);
        console.log();

        // Get tasks
        console.log('📋 Getting all tasks...');
        const tasks = await client.getTasks();
        console.log('Tasks:', tasks);
        console.log();

        // Create a memory
        console.log('🧠 Creating a memory...');
        const memory = {
            moment: "2025-01-13 10:30 AM",
            meaning: "Client prefers iterative development",
            reason: "Affects testing cycle",
            importance: "high",
            term: "long"
        };
        const createdMemory = await client.createMemory(memory);
        console.log('✅ Memory created:', createdMemory);
        console.log();

        // Create an idea
        console.log('💡 Creating an idea...');
        const idea = {
            idea: "Use microservices for better scalability",
            share: "public",
            importance: "high"
        };
        const createdIdea = await client.createIdea(idea);
        console.log('✅ Idea created:', createdIdea);
        console.log();

        // Create code chunks
        console.log('🧩 Creating code chunks...');
        const chunks = [
            {
                id: "project_1",
                level: "project",
                description: "Build web scraper with database storage",
                dependencies: [],
                code: "High-level project planning"
            },
            {
                id: "module_1",
                level: "module",
                description: "Create database handler module",
                dependencies: ["project_1"],
                code: "import sqlite3, import json"
            },
            {
                id: "class_1",
                level: "class",
                description: "Implement DatabaseConnection class",
                dependencies: ["module_1"],
                code: "class DatabaseConnection:\n    def __init__(self, db_path):\n        self.db_path = db_path"
            }
        ];

        for (const chunk of chunks) {
            const createdChunk = await client.createChunk(chunk);
            console.log(`✅ Chunk ${chunk.id} created:`, createdChunk);
        }
        console.log();

        // Get ready chunks
        console.log('🎯 Getting ready chunks...');
        const readyChunks = await client.getReadyChunks();
        console.log('Ready chunks:', readyChunks);
        console.log();

        // Get chunk graph
        console.log('📊 Getting chunk dependency graph...');
        const graph = await client.getChunkGraph();
        console.log('Graph:', graph);
        console.log();

        // Process chat message
        console.log('💬 Processing chat message...');
        const chatMessage = `
        I need to build a web scraper with database storage:
        
        <chunk>project_1; project; Build web scraper with database storage; ; High-level planning</chunk>
        
        <chunk>module_1; module; Create database handler module; project_1; import sqlite3</chunk>
        
        <todozi>Implement user authentication; 3 days; high; development; todo; assignee=agent:coder; tags=auth</todozi>
        
        <memory>2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long term</memory>
        
        <idea>Use microservices for better scalability; share; high</idea>
        `;
        
        const processedChat = await client.processChat(chatMessage);
        console.log('✅ Chat processed:', processedChat);
        console.log();

        // Get projects
        console.log('📁 Getting projects...');
        const projects = await client.getProjects();
        console.log('Projects:', projects);
        console.log();

        console.log('🎉 Demo completed successfully!');
        console.log('📡 All API endpoints are working on port 8636 (TODO in dial language!)');

    } catch (error) {
        console.error('❌ Error:', error.message);
        console.log('💡 Make sure the Todozi server is running on port 8636');
    }
}

// Run the demo
if (require.main === module) {
    main().catch(console.error);
}

module.exports = { TodoziClient };
