#!/usr/bin/env node

import { Command } from 'commander';
import * as fs from 'fs';
import * as path from 'path';

// Import our modules
import {
    todozi,
    Todozi,
    healthCheck,
    VERSION,
    Commands,
    isCommandType,
    createCommand,
    CommandExecutionError,
    TodoziError
} from './index.js';

// CLI setup
const program = new Command();

program
    .name('todozi')
    .description('Todozi - AI-powered task and knowledge management system')
    .version(VERSION);

// Health check command
program
    .command('health')
    .description('Check system health')
    .action(() => {
        const health = healthCheck();
        console.log(`Status: ${health.status.toUpperCase()}`);
        console.log(`Version: ${health.version}`);
        console.log(`Timestamp: ${health.timestamp}`);
        console.log('\nComponents:');
        Object.entries(health.components).forEach(([component, healthy]) => {
            console.log(`  ${component}: ${healthy ? '✓' : '✗'}`);
        });

        if (health.status === 'unhealthy') {
            process.exit(1);
        }
    });

// Initialize command
program
    .command('init')
    .description('Initialize Todozi system')
    .option('--path <path>', 'Storage path', './todozi_data')
    .action(async (options) => {
        try {
            console.log('Initializing Todozi...');

            // Create storage directory
            if (!fs.existsSync(options.path)) {
                fs.mkdirSync(options.path, { recursive: true });
                console.log(`Created storage directory: ${options.path}`);
            }

            // Initialize the system
            await todozi.initialize();
            console.log('✓ Todozi initialized successfully');

            // Create some sample data if requested
            if (process.argv.includes('--sample')) {
                console.log('Creating sample data...');
                // Add sample tasks, etc.
            }

        } catch (error) {
            console.error('Failed to initialize Todozi:', error);
            process.exit(1);
        }
    });

// Task management commands
const taskCmd = program
    .command('task')
    .description('Task management commands');

taskCmd
    .command('add <action>')
    .description('Add a new task')
    .option('-p, --priority <priority>', 'Priority (low|medium|high|urgent)', 'medium')
    .option('-t, --time <time>', 'Time estimate', '1h')
    .option('-j, --project <project>', 'Project name', 'general')
    .option('-a, --assignee <assignee>', 'Assignee (ai|human|collaborative)', 'ai')
    .option('-T, --tags <tags>', 'Comma-separated tags')
    .action(async (action, options) => {
        try {
            const tags = options.tags ? options.tags.split(',').map((t: string) => t.trim()) : [];

            const result = await todozi.task(action, {
                priority: options.priority,
                time: options.time,
                project: options.project,
                assignee: options.assignee,
                tags
            });

            console.log(`✓ Task created: ${result}`);
        } catch (error) {
            console.error('Failed to create task:', error);
            process.exit(1);
        }
    });

taskCmd
    .command('list')
    .description('List tasks')
    .option('-p, --project <project>', 'Filter by project')
    .option('-s, --status <status>', 'Filter by status')
    .option('-a, --assignee <assignee>', 'Filter by assignee')
    .action(async (options) => {
        try {
            const tasks = todozi.getStorage().getAllTasks();

            let filteredTasks = tasks;
            if (options.project) {
                filteredTasks = filteredTasks.filter(t => t.parentProject === options.project);
            }
            if (options.status) {
                filteredTasks = filteredTasks.filter(t => t.status === options.status);
            }
            if (options.assignee) {
                filteredTasks = filteredTasks.filter(t => t.assignee?.type === options.assignee);
            }

            if (filteredTasks.length === 0) {
                console.log('No tasks found');
                return;
            }

            console.log(`Found ${filteredTasks.length} task(s):`);
            filteredTasks.forEach(task => {
                console.log(`• ${task.action} [${task.status}] (${task.priority})`);
            });
        } catch (error) {
            console.error('Failed to list tasks:', error);
            process.exit(1);
        }
    });

taskCmd
    .command('complete <id>')
    .description('Mark task as completed')
    .action(async (id) => {
        try {
            await todozi.complete(id);
            console.log(`✓ Task ${id} marked as completed`);
        } catch (error) {
            console.error('Failed to complete task:', error);
            process.exit(1);
        }
    });

// Memory management commands
const memoryCmd = program
    .command('memory')
    .description('Memory management commands');

memoryCmd
    .command('add <moment>')
    .description('Add a new memory')
    .requiredOption('-m, --meaning <meaning>', 'Meaning of the memory')
    .requiredOption('-r, --reason <reason>', 'Reason for remembering')
    .option('-i, --importance <importance>', 'Importance (low|medium|high|critical)', 'medium')
    .option('-t, --term <term>', 'Term (short|long)', 'long')
    .option('-T, --tags <tags>', 'Comma-separated tags')
    .action(async (moment, options) => {
        try {
            const tags = options.tags ? options.tags.split(',').map((t: string) => t.trim()) : [];

            const result = await todozi.create_memory(moment, options.meaning, options.reason, {
                importance: options.importance,
                term: options.term,
                tags
            });

            console.log(`✓ Memory created: ${result.id}`);
        } catch (error) {
            console.error('Failed to create memory:', error);
            process.exit(1);
        }
    });

// Idea management commands
const ideaCmd = program
    .command('idea')
    .description('Idea management commands');

ideaCmd
    .command('add <idea>')
    .description('Add a new idea')
    .option('-s, --share <share>', 'Sharing level (private|team|public)', 'private')
    .option('-i, --importance <importance>', 'Importance (low|medium|high|breakthrough)', 'medium')
    .option('-c, --context <context>', 'Additional context')
    .option('-T, --tags <tags>', 'Comma-separated tags')
    .action(async (idea, options) => {
        try {
            const tags = options.tags ? options.tags.split(',').map((t: string) => t.trim()) : [];

            const result = await todozi.create_idea(idea, {
                share: options.share,
                importance: options.importance,
                context: options.context,
                tags
            });

            console.log(`✓ Idea created: ${result.id}`);
        } catch (error) {
            console.error('Failed to create idea:', error);
            process.exit(1);
        }
    });

// Search commands
program
    .command('search <query>')
    .description('Search across all content')
    .option('-t, --type <type>', 'Content type (task|memory|idea|error)')
    .option('-l, --limit <limit>', 'Result limit', '10')
    .action(async (query, options) => {
        try {
            const results = await todozi.tdz_find(query, {
                types: options.type ? [options.type] : undefined,
                limit: parseInt(options.limit)
            });

            console.log(`Search results for "${query}":`);
            console.log(`Total: ${results.total}`);

            if (results.tasks.length > 0) {
                console.log('\nTasks:');
                results.tasks.forEach(task => {
                    console.log(`• ${task.action} (${task.priority})`);
                });
            }

            if (results.memories.length > 0) {
                console.log('\nMemories:');
                results.memories.forEach(memory => {
                    console.log(`• ${memory.moment} (${memory.importance})`);
                });
            }

            if (results.ideas.length > 0) {
                console.log('\nIdeas:');
                results.ideas.forEach(idea => {
                    console.log(`• ${idea.idea} (${idea.importance})`);
                });
            }

            if (results.errors.length > 0) {
                console.log('\nErrors:');
                results.errors.forEach(error => {
                    console.log(`• ${error.title} (${error.severity})`);
                });
            }
        } catch (error) {
            console.error('Search failed:', error);
            process.exit(1);
        }
    });

// Extract content commands
program
    .command('extract')
    .description('Extract tasks and content from text')
    .option('-c, --content <content>', 'Content to extract from')
    .option('-f, --file <file>', 'File to extract from')
    .option('-o, --output <format>', 'Output format (json|csv|md)', 'json')
    .option('-h, --human', 'Generate human-readable output')
    .action(async (options) => {
        try {
            let content: string;

            if (options.file) {
                if (!fs.existsSync(options.file)) {
                    console.error(`File not found: ${options.file}`);
                    process.exit(1);
                }
                content = fs.readFileSync(options.file, 'utf8');
            } else if (options.content) {
                content = options.content;
            } else {
                console.error('Either --content or --file must be provided');
                process.exit(1);
            }

            const result = await todozi.extractContent(content, {
                outputFormat: options.output,
                human: options.human
            });

            console.log(result);
        } catch (error) {
            console.error('Extraction failed:', error);
            process.exit(1);
        }
    });

// Strategy commands
program
    .command('strategy')
    .description('Generate strategic plans from content')
    .option('-c, --content <content>', 'Content to analyze')
    .option('-f, --file <file>', 'File to analyze')
    .option('-o, --output <format>', 'Output format (json|csv|md)', 'json')
    .option('-h, --human', 'Generate human-readable output')
    .action(async (options) => {
        try {
            let content: string;

            if (options.file) {
                if (!fs.existsSync(options.file)) {
                    console.error(`File not found: ${options.file}`);
                    console.error('Please provide content via --content or --file');
                    process.exit(1);
                }
                content = fs.readFileSync(options.file, 'utf8');
            } else if (options.content) {
                content = options.content;
            } else {
                console.error('Either --content or --file must be provided');
                process.exit(1);
            }

            const result = await todozi.strategyContent(content, {
                outputFormat: options.output,
                human: options.human
            });

            console.log(result);
        } catch (error) {
            console.error('Strategy generation failed:', error);
            process.exit(1);
        }
    });

// AI chat command
program
    .command('chat <message>')
    .description('Chat with AI assistant')
    .action(async (message) => {
        try {
            const result = await todozi.chat(message);
            console.log('AI Response:');
            console.log(result.response);

            if (result.tasks.length > 0) {
                console.log('\nExtracted Tasks:');
                result.tasks.forEach(task => {
                    console.log(`• ${task.action}`);
                });
            }
        } catch (error) {
            console.error('Chat failed:', error);
            process.exit(1);
        }
    });

// Statistics command
program
    .command('stats')
    .description('Show system statistics')
    .action(() => {
        try {
            const stats = todozi.getStatistics();

            console.log('Todozi Statistics:');
            console.log('==================');

            console.log('\nStorage:');
            Object.entries(stats.storage).forEach(([key, value]) => {
                console.log(`  ${key}: ${value}`);
            });

            console.log('\nEmbedding:');
            Object.entries(stats.embedding).forEach(([key, value]) => {
                console.log(`  ${key}: ${value}`);
            });

            console.log('\nAgents:');
            console.log(`  Total: ${stats.agents.totalAgents}`);
            console.log(`  Available: ${stats.agents.agentsByStatus.available || 0}`);
            console.log(`  Busy: ${stats.agents.agentsByStatus.busy || 0}`);

        } catch (error) {
            console.error('Failed to get statistics:', error);
            process.exit(1);
        }
    });

// Backup commands
const backupCmd = program
    .command('backup')
    .description('Backup management commands');

backupCmd
    .command('create [name]')
    .description('Create a backup')
    .action(async (name) => {
        try {
            const backupName = await todozi.createBackup(name);
            if (backupName) {
                console.log(`✓ Backup created: ${backupName}`);
            } else {
                console.error('Failed to create backup');
                process.exit(1);
            }
        } catch (error) {
            console.error('Backup creation failed:', error);
            process.exit(1);
        }
    });

backupCmd
    .command('list')
    .description('List available backups')
    .action(async () => {
        try {
            const backups = await todozi.getStorage().listBackups();
            if (backups.ok && backups.value.length > 0) {
                console.log('Available backups:');
                backups.value.forEach(backup => {
                    console.log(`• ${backup}`);
                });
            } else {
                console.log('No backups found');
            }
        } catch (error) {
            console.error('Failed to list backups:', error);
            process.exit(1);
        }
    });

backupCmd
    .command('restore <name>')
    .description('Restore from backup')
    .action(async (name) => {
        try {
            const success = await todozi.restoreBackup(name);
            if (success) {
                console.log(`✓ Restored from backup: ${name}`);
            } else {
                console.error('Failed to restore backup');
                process.exit(1);
            }
        } catch (error) {
            console.error('Backup restore failed:', error);
            process.exit(1);
        }
    });

// Server command
program
    .command('server')
    .description('Start Todozi HTTP server')
    .option('-h, --host <host>', 'Host to bind to', 'localhost')
    .option('-p, --port <port>', 'Port to listen on', '3000')
    .action(async (options) => {
        try {
            console.log(`Starting Todozi server on ${options.host}:${options.port}...`);

            // Import server dynamically to avoid loading it for CLI commands
            const { startServer } = await import('./server.js');

            await startServer(options.host, parseInt(options.port));
        } catch (error) {
            console.error('Failed to start server:', error);
            process.exit(1);
        }
    });

// TUI command
program
    .command('tui')
    .description('Start Terminal User Interface')
    .action(async () => {
        try {
            console.log('Starting TUI...');

            // Import TUI dynamically
            const { startTUI } = await import('./tui.js');

            await startTUI();
        } catch (error) {
            console.error('Failed to start TUI:', error);
            process.exit(1);
        }
    });

// Error handling
program.on('command:*', (unknownCommand) => {
    console.error(`Unknown command: ${unknownCommand[0]}`);
    console.log('Run "todozi --help" for available commands');
    process.exit(1);
});

// Global error handler
process.on('uncaughtException', (error) => {
    console.error('Uncaught exception:', error);
    process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
    console.error('Unhandled rejection at:', promise, 'reason:', reason);
    process.exit(1);
});

// Graceful shutdown
process.on('SIGINT', async () => {
    console.log('\nShutting down gracefully...');
    try {
        await todozi.save();
        console.log('✓ Data saved');
    } catch (error) {
        console.error('Error saving data:', error);
    }
    process.exit(0);
});

process.on('SIGTERM', async () => {
    console.log('\nReceived SIGTERM, shutting down...');
    try {
        await todozi.save();
        console.log('✓ Data saved');
    } catch (error) {
        console.error('Error saving data:', error);
    }
    process.exit(0);
});

// Parse command line arguments
program.parse();
