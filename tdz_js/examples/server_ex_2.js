// example2.js - Task Management with Todozi API

// API Configuration
import http from 'http';

const API_BASE = 'http://localhost:8636';
const API_KEY = 'your-public-api-key'; // Replace with actual key

// Helper function to make HTTP requests
function apiRequest(method, path, data = null) {
    return new Promise((resolve, reject) => {
        const options = {
            hostname: 'localhost',
            port: 8636,
            path: path,
            method: method,
            headers: {
                'Content-Type': 'application/json',
                'X-API-Key': API_KEY
            }
        };

        const req = http.request(options, (res) => {
            let body = '';
            res.on('data', chunk => body += chunk);
            res.on('end', () => {
                try {
                    const result = JSON.parse(body);
                    resolve(result);
                } catch (e) {
                    reject(e);
                }
            });
        });

        req.on('error', reject);
        if (data) {
            req.write(JSON.stringify(data));
        }
        req.end();
    });
}

// Task management functions
async function createTask(action, time, priority = 'medium', project = 'general') {
    const taskData = {
        action,
        time,
        priority,
        parent_project: project,
        status: 'todo'
    };
    
    try {
        const result = await apiRequest('POST', '/tasks', taskData);
        console.log('✅ Task created:', result.task.id);
        return result.task.id;
    } catch (error) {
        console.error('❌ Failed to create task:', error.message);
    }
}

async function getTask(id) {
    try {
        const task = await apiRequest('GET', `/tasks/${id}`);
        console.log('📋 Task details:', task.action);
        return task;
    } catch (error) {
        console.error('❌ Failed to get task:', error.message);
    }
}

async function updateTask(id, updates) {
    try {
        const result = await apiRequest('PUT', `/tasks/${id}`, updates);
        console.log('✅ Task updated:', result.task.id);
        return result.task;
    } catch (error) {
        console.error('❌ Failed to update task:', error.message);
    }
}

async function deleteTask(id) {
    try {
        const result = await apiRequest('DELETE', `/tasks/${id}`);
        console.log('🗑️ Task deleted:', result.id);
    } catch (error) {
        console.error('❌ Failed to delete task:', error.message);
    }
}

async function listTasks() {
    try {
        const tasks = await apiRequest('GET', '/tasks');
        console.log(`📋 Found ${tasks.length} tasks`);
        return tasks;
    } catch (error) {
        console.error('❌ Failed to list tasks:', error.message);
    }
}

async function searchTasks(query) {
    try {
        const results = await apiRequest('GET', `/tasks/search?q=${encodeURIComponent(query)}`);
        console.log(`🔍 Found ${results.length} matching tasks`);
        return results;
    } catch (error) {
        console.error('❌ Search failed:', error.message);
    }
}

// Main execution
async function main() {
    console.log('🚀 Todozi Task Management Example\n');

    // Create sample tasks
    console.log('1. Creating tasks...');
    const taskId1 = await createTask(
        'Implement user authentication',
        '2 hours',
        'high',
        'backend'
    );
    
    const taskId2 = await createTask(
        'Design dashboard UI',
        '3 hours',
        'medium',
        'frontend'
    );

    // List all tasks
    console.log('\n2. Listing all tasks...');
    await listTasks();

    // Get specific task
    console.log('\n3. Getting task details...');
    if (taskId1) {
        await getTask(taskId1);
    }

    // Update task
    console.log('\n4. Updating task...');
    if (taskId2) {
        await updateTask(taskId2, {
            status: 'in_progress',
            progress: 50
        });
    }

    // Search tasks
    console.log('\n5. Searching tasks...');
    await searchTasks('authentication');

    // Delete task
    console.log('\n6. Deleting task...');
    if (taskId1) {
        await deleteTask(taskId1);
    }

    // Final task list
    console.log('\n7. Final task list...');
    await listTasks();
}

// Run the example
main().catch(console.error);

/*
Here's a practical example showing how to use the Todozi server API to manage tasks with authentication:

To use this example:

1. First start the Todozi server:

/ *
bash
node server.js

/ *
2. Register for an API key:
curl -X POST http://localhost:8636/api/register

3. Replace `your-public-api-key` in the example with the actual key

4. Run the example:
node example2.js

Key features demonstrated:
- Creating tasks with different priorities and projects
- Retrieving task details
- Updating task status and progress
- Searching tasks by keywords
- Deleting tasks
- Listing all tasks
- Proper error handling
- Authentication with API keys

Expected output:
*/