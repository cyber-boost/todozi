import fetch from 'node-fetch';
import { TodoziServer, ServerConfig } from '../todozi/server.js';

async function analyzeTaskPerformance(apiKey) {
    // Get comprehensive analytics
    const analytics = await fetch(`${SERVER_URL}/analytics/tasks`, {
        method: 'GET',
        headers: {
            'X-API-Key': apiKey.public_key
        }
    }).then(r => r.json());
    
    console.log('📊 Task Analytics:');
    console.log('- Total tasks:', analytics.total_tasks);
    console.log('- Completion rate:', (analytics.completion_rate * 100).toFixed(1) + '%');
    console.log('- Tasks by status:', analytics.by_status);
    
    // Semantic search for related tasks
    const searchResults = await fetch(`${SERVER_URL}/semantic/search?q=database optimization`, {
        method: 'GET',
        headers: {
            'X-API-Key': apiKey.public_key
        }
    }).then(r => r.json());
    
    console.log('🔍 Semantic Search Results:', searchResults.length, 'matches found');
    
    return { analytics, searchResults };
}

/*
async function createAgentAssignedTask(apiKey) {
    // Create a task specifically for an AI agent
    const taskData = {
        action: 'Generate Python script for data visualization with matplotlib',
        time: '2 hours',
        priority: 'medium',
        parent_project: 'data-analysis',
        assignee: 'agent:coder', // Assign to the coder agent
        tags: ['python', 'matplotlib', 'data-visualization', 'ai-assigned'],
        dependencies: ['data_cleaning_completed'],
        context_notes: 'Create interactive charts with hover effects and export to PNG'
    };
    
    const response = await fetch(`${SERVER_URL}/tasks`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'X-API-Key': apiKey.public_key,
            'X-API-Private-Key': apiKey.private_key
        },
        body: JSON.stringify(taskData)
    });
    
    if (!response.ok) {
        throw new Error('Failed to create agent-assigned task');
    }
    
    const result = await response.json();
    
    // Check agent assignment through the queue system
    const queueResponse = await fetch(`${SERVER_URL}/queue/list/active`, {
        method: 'GET',
        headers: {
            'X-API-Key': apiKey.public_key
        }
    });
    
    if (queueResponse.ok) {
        const queueItems = await queueResponse.json();
        console.log('Active queue items:', queueItems);
    }
    
    return result.task;
}

/ *

// Server configuration
const SERVER_URL = 'http://127.0.0.1:8636';

async function main() {
    try {
        // Step 1: Register for an API key
        const apiKey = await registerApiKey();
        console.log('🔑 API Key Registered:', apiKey.user_id);
        
        // Step 2: Create a new task
        const task = await createTask(apiKey);
        console.log('✅ Task created:', task.id);
        
        // Step 3: Search for similar tasks
        const similarTasks = await searchTasks(apiKey, 'code review');
        console.log('🔍 Found similar tasks:', similarTasks.length);
        
        // Step 4: Update the task progress
        const updatedTask = await updateTask(apiKey, task.id, 75);
        console.log('📊 Task progress updated to 75%');
        
        // Step 5: Get task insights
        const insights = await getTaskInsights(apiKey, task.id);
        console.log('🤖 AI Insights:', insights);
        
        // Step 6: Complete the task
        const completedTask = await completeTask(apiKey, task.id);
        console.log('🏁 Task completed successfully');
        
    } catch (error) {
        console.error('❌ Error:', error.message);
    }
}

// Helper functions
async function registerApiKey() {
    const response = await fetch(`${SERVER_URL}/api/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
    });
    
    if (!response.ok) {
        throw new Error('Failed to register API key');
    }
    
    return await response.json();
}

async function createTask(apiKey) {
    const taskData = {
        action: 'Review and optimize SQL queries for performance',
        time: '3 hours',
        priority: 'high',
        parent_project: 'database-optimization',
        assignee: 'ai',
        tags: ['sql', 'performance', 'optimization'],
        context_notes: 'Focus on indexing and query structure improvements'
    };
    
    const response = await fetch(`${SERVER_URL}/tasks`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'X-API-Key': apiKey.public_key,
            'X-API-Private-Key': apiKey.private_key
        },
        body: JSON.stringify(taskData)
    });
    
    if (!response.ok) {
        throw new Error('Failed to create task');
    }
    
    const result = await response.json();
    return result.task;
}

async function searchTasks(apiKey, query) {
    const response = await fetch(`${SERVER_URL}/tasks/search?q=${encodeURIComponent(query)}`, {
        method: 'GET',
        headers: {
            'X-API-Key': apiKey.public_key
        }
    });
    
    if (!response.ok) {
        throw new Error('Failed to search tasks');
    }
    
    return await response.json();
}

async function updateTask(apiKey, taskId, progress) {
    const updateData = {
        progress: progress,
        status: 'in_progress'
    };
    
    const response = await fetch(`${SERVER_URL}/tasks/${taskId}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'X-API-Key': apiKey.public_key,
            'X-API-Private-Key': apiKey.private_key
        },
        body: JSON.stringify(updateData)
    });
    
    if (!response.ok) {
        throw new Error('Failed to update task');
    }
    
    return await response.json();
}

async function getTaskInsights(apiKey, taskId) {
    const response = await fetch(`${SERVER_URL}/tasks/${taskId}/insights`, {
        method: 'GET',
        headers: {
            'X-API-Key': apiKey.public_key
        }
    });
    
    if (!response.ok) {
        throw new Error('Failed to get task insights');
    }
    
    return await response.json();
}

async function completeTask(apiKey, taskId) {
    const updateData = {
        status: 'done',
        progress: 100
    };
    
    const response = await fetch(`${SERVER_URL}/tasks/${taskId}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            'X-API-Key': apiKey.public_key,
            'X-API-Private-Key': apiKey.private_key
        },
        body: JSON.stringify(updateData)
    });
    
    if (!response.ok) {
        throw new Error('Failed to complete task');
    }
    
    return await response.json();
}

// Run the example
main();

/ *

async function startServer() {
    const config = ServerConfig.default();
    const server = await TodoziServer.new(config);
    
    try {
        await server.start();
        console.log('Todozi server is running!');
    } catch (error) {
        console.error('Failed to start server:', error);
    }
}

startServer();

/ *
# Example: Creating and Managing Tasks with Todozi Server

This example demonstrates how to use the Todozi server to create, update, and manage tasks through its REST API.

## Basic Setup

First, start the Todozi server:

## Example 1: Complete Task Management Workflow

## Example 2: Advanced Task with AI Agent Assignment

## Example 3: Semantic Search and Analytics

## Expected Output

When running these examples, you should see output like:
*/