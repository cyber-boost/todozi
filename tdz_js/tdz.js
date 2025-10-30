#!/usr/bin/env node
/**
 * Todozi CLI - Command Line Interface
 * JavaScript version of the Rust tdz CLI tool
 */

const { Command } = require('commander');
const { TodoziClient } = require('./todozi');
const fs = require('fs').promises;
const path = require('path');
const os = require('os');

class TodoziCLI {
    constructor() {
        this.program = new Command();
        this.client = new TodoziClient();
        this.setupCommands();
    }

    async run() {
        try {
            await this.program.parseAsync();
        } catch (error) {
            console.error('❌ Error:', error.message);
            process.exit(1);
        }
    }

    setupCommands() {
        this.program
            .name('tdz')
            .description('AI/Human task management system')
            .version('0.1.0');

        // Core task commands
        this.program
            .command('task <action>')
            .description('Create a new task')
            .option('-p, --priority <priority>', 'Priority (low, medium, high, urgent, critical)', 'medium')
            .option('-t, --time <time>', 'Time estimate')
            .option('-j, --project <project>', 'Project name')
            .option('--tags <tags>', 'Comma-separated tags')
            .option('--assignee <assignee>', 'Assignee (ai, human, collaborative)')
            .action(async (action, options) => {
                try {
                    const task = await this.client.createTask(action, options.priority, options.project, options.time);
                    console.log(`✅ Task created: ${task.id}`);
                } catch (error) {
                    console.error('❌ Failed to create task:', error.message);
                }
            });

        // Priority shortcuts
        this.program
            .command('urgent <action>')
            .description('Create urgent priority task')
            .action(async (action) => {
                try {
                    const taskId = await this.client.urgent(action);
                    console.log(`⚡ Urgent task created: ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to create urgent task:', error.message);
                }
            });

        this.program
            .command('high <action>')
            .description('Create high priority task')
            .action(async (action) => {
                try {
                    const taskId = await this.client.high(action);
                    console.log(`🔴 High priority task created: ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to create high priority task:', error.message);
                }
            });

        this.program
            .command('low <action>')
            .description('Create low priority task')
            .action(async (action) => {
                try {
                    const taskId = await this.client.low(action);
                    console.log(`🔵 Low priority task created: ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to create low priority task:', error.message);
                }
            });

        // AI task types
        this.program
            .command('ai <action>')
            .description('Create AI-assigned task')
            .action(async (action) => {
                try {
                    const taskId = await this.client.aiTask(action);
                    console.log(`🤖 AI task created: ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to create AI task:', error.message);
                }
            });

        this.program
            .command('human <action>')
            .description('Create human-assigned task')
            .action(async (action) => {
                try {
                    const taskId = await this.client.humanTask(action);
                    console.log(`👤 Human task created: ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to create human task:', error.message);
                }
            });

        this.program
            .command('collab <action>')
            .description('Create collaborative task')
            .action(async (action) => {
                try {
                    const taskId = await this.client.collabTask(action);
                    console.log(`🤝 Collaborative task created: ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to create collaborative task:', error.message);
                }
            });

        // Memory and Idea commands
        this.program
            .command('remember <moment> <meaning>')
            .description('Create a memory')
            .option('--reason <reason>', 'Why to remember')
            .option('--importance <importance>', 'Importance level', 'medium')
            .option('--tags <tags>', 'Comma-separated tags')
            .action(async (moment, meaning, options) => {
                try {
                    const task = await this.client.remember(moment, meaning);
                    console.log(`🧠 Memory created: ${task.id}`);
                } catch (error) {
                    console.error('❌ Failed to create memory:', error.message);
                }
            });

        this.program
            .command('idea <idea>')
            .description('Create an idea')
            .option('--share <share>', 'Sharing level (private, team, public)', 'team')
            .option('--importance <importance>', 'Importance level', 'medium')
            .action(async (idea, options) => {
                try {
                    const task = await this.client.idea(idea);
                    console.log(`💡 Idea created: ${task.id}`);
                } catch (error) {
                    console.error('❌ Failed to create idea:', error.message);
                }
            });

        // Task management
        this.program
            .command('done <taskId>')
            .description('Mark task as completed')
            .action(async (taskId) => {
                try {
                    await this.client.done(taskId);
                    console.log(`✅ Task ${taskId} marked as completed`);
                } catch (error) {
                    console.error('❌ Failed to complete task:', error.message);
                }
            });

        this.program
            .command('start <taskId>')
            .description('Start working on task')
            .action(async (taskId) => {
                try {
                    await this.client.start(taskId);
                    console.log(`▶️ Started working on task ${taskId}`);
                } catch (error) {
                    console.error('❌ Failed to start task:', error.message);
                }
            });

        // List commands
        this.program
            .command('list')
            .description('List all tasks')
            .option('--status <status>', 'Filter by status')
            .option('--project <project>', 'Filter by project')
            .action(async (options) => {
                try {
                    const tasks = await this.client.all();
                    if (tasks.length === 0) {
                        console.log('📋 No tasks found');
                        return;
                    }

                    console.log('📋 Tasks:');
                    tasks.forEach(task => {
                        const status = task.status.toUpperCase();
                        const priority = task.priority.charAt(0).toUpperCase();
                        const project = task.parent_project ? ` [${task.parent_project}]` : '';
                        console.log(`  ${task.id}: ${task.action} (${status}, ${priority})${project}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to list tasks:', error.message);
                }
            });

        // Show task details
        this.program
            .command('show <taskId>')
            .description('Show task details')
            .action(async (taskId) => {
                try {
                    const task = await this.client.getTask(taskId);
                    if (task) {
                        console.log(`📋 Task: ${task.id}`);
                        console.log(`   Action: ${task.action}`);
                        console.log(`   Status: ${task.status}`);
                        console.log(`   Priority: ${task.priority}`);
                        console.log(`   Time: ${task.time}`);
                        console.log(`   Project: ${task.parent_project}`);
                        console.log(`   Tags: ${task.tags.join(', ')}`);
                        console.log(`   Progress: ${task.progress || 0}%`);
                        console.log(`   Created: ${task.created_at}`);
                        console.log(`   Updated: ${task.updated_at}`);
                    } else {
                        console.log('❌ Task not found');
                    }
                } catch (error) {
                    console.error('❌ Failed to show task:', error.message);
                }
            });

        // Update task
        this.program
            .command('update <taskId>')
            .description('Update task properties')
            .option('-a, --action <action>', 'New action')
            .option('-s, --status <status>', 'New status')
            .option('-p, --priority <priority>', 'New priority')
            .option('-j, --project <project>', 'New project')
            .option('-t, --time <time>', 'New time estimate')
            .option('--progress <progress>', 'Progress percentage', parseInt)
            .action(async (taskId, options) => {
                try {
                    // For now, we'll use a simple approach
                    if (options.status === 'done') {
                        await this.client.done(taskId);
                        console.log(`✅ Task ${taskId} marked as completed`);
                    } else {
                        console.log('ℹ️ Advanced update features coming soon');
                    }
                } catch (error) {
                    console.error('❌ Failed to update task:', error.message);
                }
            });

        // Delete task
        this.program
            .command('delete <taskId>')
            .description('Delete a task')
            .action(async (taskId) => {
                try {
                    await this.client.deleteTask(taskId);
                    console.log(`🗑️ Task ${taskId} deleted`);
                } catch (error) {
                    console.error('❌ Failed to delete task:', error.message);
                }
            });

        // Search commands
        this.program
            .command('find <query>')
            .description('Search tasks by keyword')
            .action(async (query) => {
                try {
                    const tasks = await this.client.find(query);
                    if (tasks.length === 0) {
                        console.log('🔍 No tasks found matching query');
                        return;
                    }

                    console.log(`🔍 Found ${tasks.length} tasks:`);
                    tasks.forEach(task => {
                        console.log(`  ${task.id}: ${task.action}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to search tasks:', error.message);
                }
            });

        this.program
            .command('ai-find <query>')
            .description('AI-powered semantic search')
            .action(async (query) => {
                try {
                    const tasks = await this.client.aiFind(query);
                    if (tasks.length === 0) {
                        console.log('🎯 No semantically similar tasks found');
                        return;
                    }

                    console.log(`🎯 AI found ${tasks.length} semantically similar tasks:`);
                    tasks.forEach(task => {
                        console.log(`  ${task.id}: ${task.action}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to perform AI search:', error.message);
                }
            });

        // Stats command
        this.program
            .command('stats')
            .description('Show system statistics')
            .action(async () => {
                try {
                    const stats = await this.client.stats();
                    console.log('📊 System Statistics:');
                    console.log(stats);
                } catch (error) {
                    console.error('❌ Failed to get stats:', error.message);
                }
            });

        // Project commands
        const projectCmd = this.program
            .command('project')
            .description('Project management commands');

        projectCmd
            .command('create <name>')
            .description('Create a new project')
            .option('-d, --description <description>', 'Project description')
            .action(async (name, options) => {
                try {
                    await this.client.createProject(name, options.description);
                    console.log(`📁 Project "${name}" created`);
                } catch (error) {
                    console.error('❌ Failed to create project:', error.message);
                }
            });

        projectCmd
            .command('list')
            .description('List all projects')
            .action(async () => {
                try {
                    const projects = await this.client.listProjects();
                    if (projects.length === 0) {
                        console.log('📁 No projects found');
                        return;
                    }

                    console.log('📁 Projects:');
                    projects.forEach(project => {
                        console.log(`  • ${project}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to list projects:', error.message);
                }
            });

        projectCmd
            .command('tasks <projectName>')
            .description('List tasks for a project')
            .action(async (projectName) => {
                try {
                    const tasks = await this.client.projectTasks(projectName);
                    if (tasks.length === 0) {
                        console.log(`📋 No tasks found for project "${projectName}"`);
                        return;
                    }

                    console.log(`📋 Tasks for project "${projectName}":`);
                    tasks.forEach(task => {
                        console.log(`  ${task.id}: ${task.action} (${task.status})`);
                    });
                } catch (error) {
                    console.error('❌ Failed to get project tasks:', error.message);
                }
            });

        // Memory commands
        const memoryCmd = this.program
            .command('memory')
            .description('Memory management commands');

        memoryCmd
            .command('list')
            .description('List all memories')
            .action(async () => {
                try {
                    const memories = await this.client.listMemories();
                    if (memories.length === 0) {
                        console.log('🧠 No memories found');
                        return;
                    }

                    console.log('🧠 Memories:');
                    memories.forEach(memory => {
                        console.log(`  ${memory.id}: ${memory.moment}`);
                        console.log(`     Meaning: ${memory.meaning}`);
                        console.log(`     Type: ${memory.importance}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to list memories:', error.message);
                }
            });

        // Idea commands
        const ideaCmd = this.program
            .command('idea')
            .description('Idea management commands');

        ideaCmd
            .command('list')
            .description('List all ideas')
            .action(async () => {
                try {
                    const ideas = await this.client.listIdeas();
                    if (ideas.length === 0) {
                        console.log('💡 No ideas found');
                        return;
                    }

                    console.log('💡 Ideas:');
                    ideas.forEach(idea => {
                        console.log(`  ${idea.id}: ${idea.idea}`);
                        console.log(`     Importance: ${idea.importance}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to list ideas:', error.message);
                }
            });

        // Chat command
        this.program
            .command('chat <message>')
            .description('Send a message to the AI chat system')
            .action(async (message) => {
                try {
                    const response = await this.client.chat(message);
                    console.log('💬 AI Response:');
                    console.log(response);
                } catch (error) {
                    console.error('❌ Failed to chat:', error.message);
                }
            });

        // Advanced search command
        this.program
            .command('search-all <query>')
            .description('Search across all content types')
            .option('--types <types>', 'Content types to search (comma-separated)', 'tasks,memories,ideas')
            .action(async (query, options) => {
                try {
                    console.log(`🔍 Searching for "${query}" across: ${options.types}`);

                    // Search tasks
                    if (options.types.includes('tasks')) {
                        const tasks = await this.client.find(query);
                        console.log(`📋 Tasks (${tasks.length}):`);
                        tasks.slice(0, 3).forEach(task => {
                            console.log(`  • ${task.action}`);
                        });
                    }

                    // Search memories
                    if (options.types.includes('memories')) {
                        const memories = await this.client.findMemories(query);
                        console.log(`🧠 Memories (${memories.length}):`);
                        memories.slice(0, 3).forEach(memory => {
                            console.log(`  • ${memory.moment}`);
                        });
                    }

                    // Search ideas
                    if (options.types.includes('ideas')) {
                        const ideas = await this.client.findIdeas(query);
                        console.log(`💡 Ideas (${ideas.length}):`);
                        ideas.slice(0, 3).forEach(idea => {
                            console.log(`  • ${idea.idea}`);
                        });
                    }
                } catch (error) {
                    console.error('❌ Failed to search:', error.message);
                }
            });

        // Initialization commands
        this.program
            .command('init')
            .description('Initialize Todozi system')
            .action(async () => {
                try {
                    await this.client.init();
                    console.log('✅ Todozi system initialized successfully');
                } catch (error) {
                    console.error('❌ Failed to initialize:', error.message);
                }
            });

        this.program
            .command('check-structure')
            .description('Check if folder structure is complete')
            .action(async () => {
                try {
                    const result = await this.client.tdzfp();
                    if (result) {
                        console.log('✅ Todozi folder structure is complete!');
                    } else {
                        console.log('❌ Todozi folder structure is incomplete. Run "tdz init" to create missing components.');
                    }
                } catch (error) {
                    console.error('❌ Error checking folder structure:', error.message);
                }
            });

        // API Key commands
        const apiCmd = this.program
            .command('api')
            .description('API key management');

        apiCmd
            .command('create')
            .description('Create a new API key')
            .action(async () => {
                try {
                    const apiKey = await this.client.createApiKey();
                    console.log('🔐 API Key created:');
                    console.log(`   User ID: ${apiKey.user_id}`);
                    console.log(`   Public Key: ${apiKey.public_key}`);
                    console.log(`   Active: ${apiKey.active}`);
                } catch (error) {
                    console.error('❌ Failed to create API key:', error.message);
                }
            });

        apiCmd
            .command('list')
            .description('List all API keys')
            .action(async () => {
                try {
                    const keys = await this.client.listApiKeys();
                    if (keys.length === 0) {
                        console.log('🔑 No API keys found');
                        return;
                    }

                    console.log('🔑 API Keys:');
                    keys.forEach(key => {
                        console.log(`  ${key.user_id}: ${key.public_key.substring(0, 16)}... (${key.active ? 'active' : 'inactive'})`);
                    });
                } catch (error) {
                    console.error('❌ Failed to list API keys:', error.message);
                }
            });

        // Queue commands
        const queueCmd = this.program
            .command('queue')
            .description('Queue management commands');

        queueCmd
            .command('add <taskName>')
            .description('Add item to queue')
            .option('-d, --description <description>', 'Task description')
            .action(async (taskName, options) => {
                try {
                    const queueItem = await this.client.queueAdd(taskName, options.description || taskName);
                    console.log(`➕ Added to queue: ${queueItem}`);
                } catch (error) {
                    console.error('❌ Failed to add to queue:', error.message);
                }
            });

        queueCmd
            .command('list')
            .description('List queue items')
            .action(async () => {
                try {
                    const items = await this.client.queueList();
                    if (items.length === 0) {
                        console.log('📋 Queue is empty');
                        return;
                    }

                    console.log('📋 Queue Items:');
                    items.forEach(item => {
                        console.log(`  ${item.id}: ${item.task_name} (${item.status})`);
                    });
                } catch (error) {
                    console.error('❌ Failed to list queue:', error.message);
                }
            });

        queueCmd
            .command('start <itemId>')
            .description('Start working on queue item')
            .action(async (itemId) => {
                try {
                    const sessionId = await this.client.queueStart(itemId);
                    console.log(`▶️ Started queue session: ${sessionId}`);
                } catch (error) {
                    console.error('❌ Failed to start queue item:', error.message);
                }
            });

        queueCmd
            .command('complete <sessionId>')
            .description('Complete queue session')
            .action(async (sessionId) => {
                try {
                    await this.client.queueComplete(sessionId);
                    console.log(`✅ Queue session ${sessionId} completed`);
                } catch (error) {
                    console.error('❌ Failed to complete queue session:', error.message);
                }
            });

        // Embedding and AI commands
        this.program
            .command('embed <text>')
            .description('Generate embeddings for text')
            .action(async (text) => {
                try {
                    const embeddings = await this.client.embed(text);
                    console.log(`🧠 Generated embeddings (${embeddings.length} dimensions)`);
                    console.log(`First 5 values: ${embeddings.slice(0, 5).map(x => x.toFixed(4)).join(', ')}`);
                } catch (error) {
                    console.error('❌ Failed to generate embeddings:', error.message);
                }
            });

        // TUI command
        this.program
            .command('tui')
            .description('Launch Todozi Terminal User Interface')
            .action(async () => {
                console.log('🚀 Launching Todozi TUI...');
                console.log('ℹ️ TUI functionality coming soon in JavaScript version');
                console.log('💡 For now, use the web interface at: http://localhost:8275');
            });

        // Server command
        this.program
            .command('server')
            .description('Start Todozi HTTP server')
            .option('-p, --port <port>', 'Port to listen on', '8275')
            .option('-h, --host <host>', 'Host to bind to', '0.0.0.0')
            .action(async (options) => {
                console.log(`🚀 Starting Todozi server on ${options.host}:${options.port}...`);

                // Import and start the server directly
                const { TodoziServer, ServerConfig } = require('./server.js');

                const config = new ServerConfig();
                config.port = parseInt(options.port);
                config.host = options.host;
                const server = new TodoziServer(config);

                // Handle graceful shutdown
                process.on('SIGINT', () => {
                    console.log('\n⚠️  Received SIGINT, shutting down gracefully...');
                    server.stop();
                    process.exit(0);
                });

                process.on('SIGTERM', () => {
                    console.log('\n⚠️  Received SIGTERM, shutting down gracefully...');
                    server.stop();
                    process.exit(0);
                });

                try {
                    await server.start();
                } catch (error) {
                    console.error('💥 Failed to start server:', error);
                    process.exit(1);
                }
            });

        // Backup commands
        const backupCmd = this.program
            .command('backup')
            .description('Backup management commands');

        backupCmd
            .command('create')
            .description('Create a backup')
            .action(async () => {
                console.log('💾 Creating backup...');
                console.log('ℹ️ Backup functionality coming soon');
            });

        backupCmd
            .command('list')
            .description('List backups')
            .action(async () => {
                console.log('📋 Available backups:');
                console.log('ℹ️ Backup functionality coming soon');
            });

        // Registration commands
        this.program
            .command('register <serverUrl>')
            .description('Register with Todozi server')
            .action(async (serverUrl) => {
                console.log(`🚀 Starting registration with ${serverUrl}...`);
                console.log('ℹ️ Registration functionality coming soon');
            });

        this.program
            .command('registration-status')
            .description('Check registration status')
            .action(async () => {
                console.log('📋 Registration Status:');
                console.log('ℹ️ Registration functionality coming soon');
            });

        // Easy AI commands
        this.program
            .command('do-it <what>')
            .description('Do something with AI assistance')
            .action(async (what) => {
                try {
                    const result = await this.client.doIt(what);
                    console.log('✨ AI Result:');
                    console.log(result);
                } catch (error) {
                    console.error('❌ Failed:', error.message);
                }
            });

        this.program
            .command('easy-find <what>')
            .description('Easy search functionality')
            .action(async (what) => {
                try {
                    const result = await this.client.easyFind(what);
                    console.log('🔍 Easy Search Result:');
                    console.log(result);
                } catch (error) {
                    console.error('❌ Failed:', error.message);
                }
            });

        // Content processing
        this.program
            .command('extract <content>')
            .description('Extract tasks from content')
            .action(async (content) => {
                try {
                    const tasks = await this.client.extractTasks(content);
                    console.log(`📋 Extracted ${tasks.length} tasks:`);
                    tasks.forEach((task, i) => {
                        console.log(`  ${i + 1}. ${task}`);
                    });
                } catch (error) {
                    console.error('❌ Failed to extract tasks:', error.message);
                }
            });

        // Help and version
        this.program
            .command('version')
            .description('Show version information')
            .action(() => {
                console.log('Todozi CLI v0.1.0');
                console.log('JavaScript Edition');
            });
    }
}

// Run the CLI
if (require.main === module) {
    const cli = new TodoziCLI();
    cli.run().catch(error => {
        console.error('💥 Fatal error:', error.message);
        process.exit(1);
    });
}

module.exports = TodoziCLI;
