import * as readline from 'readline';
import { todozi, TodoziError, Task, Memory, Idea, ErrorRecord } from './index.js';

// TUI state
interface TUIState {
    mode: 'main' | 'tasks' | 'memories' | 'ideas' | 'search' | 'chat';
    currentPage: number;
    pageSize: number;
    filter: string;
    selectedIndex: number;
}

// TUI colors and formatting
class TUIFormatter {
    static colors = {
        reset: '\x1b[0m',
        bright: '\x1b[1m',
        dim: '\x1b[2m',

        // Foreground colors
        red: '\x1b[31m',
        green: '\x1b[32m',
        yellow: '\x1b[33m',
        blue: '\x1b[34m',
        magenta: '\x1b[35m',
        cyan: '\x1b[36m',
        white: '\x1b[37m',

        // Background colors
        bgRed: '\x1b[41m',
        bgGreen: '\x1b[42m',
        bgYellow: '\x1b[43m',
        bgBlue: '\x1b[44m',
        bgMagenta: '\x1b[45m',
        bgCyan: '\x1b[46m',
    };

    static priorityColor(priority: string): string {
        switch (priority.toLowerCase()) {
            case 'urgent': return this.colors.bgRed;
            case 'critical': return this.colors.red;
            case 'high': return this.colors.yellow;
            case 'medium': return this.colors.blue;
            case 'low': return this.colors.dim;
            default: return this.colors.white;
        }
    }

    static statusColor(status: string): string {
        switch (status.toLowerCase()) {
            case 'done':
            case 'completed': return this.colors.green;
            case 'in_progress':
            case 'inprogress': return this.colors.blue;
            case 'blocked': return this.colors.red;
            case 'review': return this.colors.yellow;
            case 'todo': return this.colors.cyan;
            default: return this.colors.white;
        }
    }

    static formatTask(task: Task, index?: number): string {
        const prefix = index !== undefined ? `${index + 1}. ` : '';
        const priorityColor = this.priorityColor(task.priority);
        const statusColor = this.statusColor(task.status);

        return `${prefix}${priorityColor}[${task.priority.toUpperCase()}]${this.colors.reset} ${this.colors.bright}${task.action}${this.colors.reset}\n` +
               `   ${statusColor}${task.status.toUpperCase()}${this.colors.reset} | ${task.parentProject} | ${task.time} | ${task.assignee ? task.assignee.type : 'unassigned'}`;
    }

    static formatMemory(memory: Memory, index?: number): string {
        const prefix = index !== undefined ? `${index + 1}. ` : '';
        const importanceColor = this.priorityColor(memory.importance);

        return `${prefix}${importanceColor}[${memory.importance.toUpperCase()}]${this.colors.reset} ${this.colors.bright}${memory.moment}${this.colors.reset}\n` +
               `   ${this.colors.dim}${memory.meaning}${this.colors.reset}`;
    }

    static formatIdea(idea: Idea, index?: number): string {
        const prefix = index !== undefined ? `${index + 1}. ` : '';
        const importanceColor = idea.importance === 'breakthrough' ? this.colors.bgMagenta : this.priorityColor(idea.importance);

        return `${prefix}${importanceColor}[${idea.importance.toUpperCase()}]${this.colors.reset} ${this.colors.bright}${idea.idea}${this.colors.reset}\n` +
               `   ${this.colors.dim}${idea.share} | ${idea.context || 'No context'}${this.colors.reset}`;
    }

    static formatError(error: ErrorRecord, index?: number): string {
        const prefix = index !== undefined ? `${index + 1}. ` : '';
        const severityColor = this.priorityColor(error.severity);

        return `${prefix}${severityColor}[${error.severity.toUpperCase()}]${this.colors.reset} ${this.colors.bright}${error.title}${this.colors.reset}\n` +
               `   ${this.colors.dim}${error.category} | ${error.source}${this.colors.reset}`;
    }

    static formatStats(stats: any): string {
        return `${this.colors.bright}System Statistics${this.colors.reset}\n` +
               `${this.colors.cyan}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${this.colors.reset}\n\n` +

               `${this.colors.yellow}Storage:${this.colors.reset}\n` +
               `  Tasks: ${stats.storage?.tasks || 0}\n` +
               `  Memories: ${stats.storage?.memories || 0}\n` +
               `  Ideas: ${stats.storage?.ideas || 0}\n` +
               `  Errors: ${stats.storage?.errors || 0}\n\n` +

               `${this.colors.yellow}AI/ML:${this.colors.reset}\n` +
               `  Embedded Items: ${stats.embedding?.totalEmbedded || 0}\n` +
               `  Cache Size: ${stats.embedding?.cacheSize || 0}\n` +
               `  Agents: ${stats.agents?.totalAgents || 0}\n\n` +

               `${this.colors.yellow}Content:${this.colors.reset}\n` +
               `  Summaries: ${stats.summaries?.totalSummaries || 0}\n` +
               `  Training Data: ${stats.training?.totalTrainingData || 0}`;
    }
}

// Main TUI class
export class TodoziTUI {
    private rl: readline.Interface;
    private state: TUIState;

    constructor() {
        this.state = {
            mode: 'main',
            currentPage: 0,
            pageSize: 10,
            filter: '',
            selectedIndex: 0
        };

        this.rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            prompt: 'todozi> '
        });

        this.setupEventHandlers();
    }

    private setupEventHandlers(): void {
        this.rl.on('line', (input) => {
            this.handleInput(input.trim());
        });

        this.rl.on('close', () => {
            console.log('\nGoodbye! 👋');
            process.exit(0);
        });

        // Handle Ctrl+C
        process.on('SIGINT', () => {
            this.rl.close();
        });
    }

    private async handleInput(input: string): Promise<void> {
        if (!input) {
            this.showPrompt();
            return;
        }

        const [command, ...args] = input.split(' ');

        try {
            switch (this.state.mode) {
                case 'main':
                    await this.handleMainCommand(command, args);
                    break;
                case 'tasks':
                    await this.handleTaskCommand(command, args);
                    break;
                case 'memories':
                    await this.handleMemoryCommand(command, args);
                    break;
                case 'ideas':
                    await this.handleIdeaCommand(command, args);
                    break;
                case 'search':
                    await this.handleSearchCommand(command, args);
                    break;
                case 'chat':
                    await this.handleChatCommand(input);
                    break;
            }
        } catch (error) {
            console.log(`${TUIFormatter.colors.red}Error: ${error instanceof TodoziError ? error.message : String(error)}${TUIFormatter.colors.reset}`);
        }

        this.showPrompt();
    }

    private async handleMainCommand(command: string, args: string[]): Promise<void> {
        switch (command.toLowerCase()) {
            case 'tasks':
            case 't':
                this.state.mode = 'tasks';
                await this.showTasks();
                break;

            case 'memories':
            case 'm':
                this.state.mode = 'memories';
                await this.showMemories();
                break;

            case 'ideas':
            case 'i':
                this.state.mode = 'ideas';
                await this.showIdeas();
                break;

            case 'search':
            case 's':
                this.state.mode = 'search';
                if (args.length > 0) {
                    await this.performSearch(args.join(' '));
                } else {
                    console.log('Enter search query:');
                }
                break;

            case 'chat':
            case 'c':
                this.state.mode = 'chat';
                if (args.length > 0) {
                    await this.performChat(args.join(' '));
                    this.state.mode = 'main'; // Return to main after chat
                } else {
                    console.log('Enter message to chat with AI:');
                }
                break;

            case 'add':
            case 'a':
                await this.handleQuickAdd(args.join(' '));
                break;

            case 'stats':
                this.showStats();
                break;

            case 'help':
            case 'h':
            case '?':
                this.showHelp();
                break;

            case 'quit':
            case 'q':
            case 'exit':
                this.rl.close();
                break;

            default:
                console.log(`${TUIFormatter.colors.yellow}Unknown command. Type 'help' for available commands.${TUIFormatter.colors.reset}`);
        }
    }

    private async handleTaskCommand(command: string, args: string[]): Promise<void> {
        switch (command.toLowerCase()) {
            case 'list':
            case 'l':
                await this.showTasks();
                break;

            case 'add':
            case 'a':
                if (args.length > 0) {
                    await this.addTask(args.join(' '));
                } else {
                    console.log('Enter task description:');
                }
                break;

            case 'complete':
            case 'c':
                if (args.length > 0) {
                    await this.completeTask(args[0]);
                } else {
                    console.log('Enter task ID to complete:');
                }
                break;

            case 'back':
            case 'b':
                this.state.mode = 'main';
                this.showMainMenu();
                break;

            default:
                console.log(`${TUIFormatter.colors.yellow}Task commands: list, add, complete, back${TUIFormatter.colors.reset}`);
        }
    }

    private async handleMemoryCommand(command: string, args: string[]): Promise<void> {
        switch (command.toLowerCase()) {
            case 'list':
            case 'l':
                await this.showMemories();
                break;

            case 'add':
            case 'a':
                console.log('Memory creation not implemented in TUI yet. Use CLI: todozi memory add');
                break;

            case 'back':
            case 'b':
                this.state.mode = 'main';
                this.showMainMenu();
                break;

            default:
                console.log(`${TUIFormatter.colors.yellow}Memory commands: list, add, back${TUIFormatter.colors.reset}`);
        }
    }

    private async handleIdeaCommand(command: string, args: string[]): Promise<void> {
        switch (command.toLowerCase()) {
            case 'list':
            case 'l':
                await this.showIdeas();
                break;

            case 'add':
            case 'a':
                console.log('Idea creation not implemented in TUI yet. Use CLI: todozi idea add');
                break;

            case 'back':
            case 'b':
                this.state.mode = 'main';
                this.showMainMenu();
                break;

            default:
                console.log(`${TUIFormatter.colors.yellow}Idea commands: list, add, back${TUIFormatter.colors.reset}`);
        }
    }

    private async handleSearchCommand(command: string, args: string[]): Promise<void> {
        if (command === 'back' || command === 'b') {
            this.state.mode = 'main';
            this.showMainMenu();
            return;
        }

        const query = command + ' ' + args.join(' ');
        if (query.trim()) {
            await this.performSearch(query);
        }
    }

    private async handleChatCommand(message: string): Promise<void> {
        await this.performChat(message);
        this.state.mode = 'main';
    }

    private async showTasks(): Promise<void> {
        const tasks = todozi.getStorage().getAllTasks();

        if (tasks.length === 0) {
            console.log(`${TUIFormatter.colors.yellow}No tasks found. Create one with 'add <description>'${TUIFormatter.colors.reset}`);
            return;
        }

        console.log(`${TUIFormatter.colors.bright}Tasks (${tasks.length})${TUIFormatter.colors.reset}`);
        console.log(`${TUIFormatter.colors.cyan}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${TUIFormatter.colors.reset}\n`);

        tasks.slice(0, this.state.pageSize).forEach((task, index) => {
            console.log(TUIFormatter.formatTask(task, index));
            console.log();
        });
    }

    private async showMemories(): Promise<void> {
        const memories = todozi.getStorage().getAllMemories();

        if (memories.length === 0) {
            console.log(`${TUIFormatter.colors.yellow}No memories found. Create one with CLI: todozi memory add${TUIFormatter.colors.reset}`);
            return;
        }

        console.log(`${TUIFormatter.colors.bright}Memories (${memories.length})${TUIFormatter.colors.reset}`);
        console.log(`${TUIFormatter.colors.cyan}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${TUIFormatter.colors.reset}\n`);

        memories.slice(0, this.state.pageSize).forEach((memory, index) => {
            console.log(TUIFormatter.formatMemory(memory, index));
            console.log();
        });
    }

    private async showIdeas(): Promise<void> {
        const ideas = todozi.getStorage().getAllIdeas();

        if (ideas.length === 0) {
            console.log(`${TUIFormatter.colors.yellow}No ideas found. Create one with CLI: todozi idea add${TUIFormatter.colors.reset}`);
            return;
        }

        console.log(`${TUIFormatter.colors.bright}Ideas (${ideas.length})${TUIFormatter.colors.reset}`);
        console.log(`${TUIFormatter.colors.cyan}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${TUIFormatter.colors.reset}\n`);

        ideas.slice(0, this.state.pageSize).forEach((idea, index) => {
            console.log(TUIFormatter.formatIdea(idea, index));
            console.log();
        });
    }

    private async performSearch(query: string): Promise<void> {
        try {
            console.log(`${TUIFormatter.colors.blue}Searching for: ${query}${TUIFormatter.colors.reset}\n`);

            const results = await todozi.tdz_find(query, { limit: 5 });

            if (results.total === 0) {
                console.log(`${TUIFormatter.colors.yellow}No results found${TUIFormatter.colors.reset}`);
                return;
            }

            if (results.tasks.length > 0) {
                console.log(`${TUIFormatter.colors.green}Tasks:${TUIFormatter.colors.reset}`);
                results.tasks.forEach(task => {
                    console.log(`  ${TUIFormatter.formatTask(task)}`);
                });
                console.log();
            }

            if (results.memories.length > 0) {
                console.log(`${TUIFormatter.colors.green}Memories:${TUIFormatter.colors.reset}`);
                results.memories.forEach(memory => {
                    console.log(`  ${TUIFormatter.formatMemory(memory)}`);
                });
                console.log();
            }

            if (results.ideas.length > 0) {
                console.log(`${TUIFormatter.colors.green}Ideas:${TUIFormatter.colors.reset}`);
                results.ideas.forEach(idea => {
                    console.log(`  ${TUIFormatter.formatIdea(idea)}`);
                });
                console.log();
            }

        } catch (error) {
            console.log(`${TUIFormatter.colors.red}Search failed: ${error}${TUIFormatter.colors.reset}`);
        }
    }

    private async performChat(message: string): Promise<void> {
        try {
            console.log(`${TUIFormatter.colors.blue}Thinking...${TUIFormatter.colors.reset}`);

            const result = await todozi.chat(message);

            console.log(`${TUIFormatter.colors.green}AI Response:${TUIFormatter.colors.reset}`);
            console.log(result.response);

            if (result.tasks.length > 0) {
                console.log(`\n${TUIFormatter.colors.yellow}Extracted Tasks:${TUIFormatter.colors.reset}`);
                result.tasks.forEach(task => {
                    console.log(`  • ${task.action}`);
                });
            }

        } catch (error) {
            console.log(`${TUIFormatter.colors.red}Chat failed: ${error}${TUIFormatter.colors.reset}`);
        }
    }

    private async addTask(description: string): Promise<void> {
        try {
            const result = await todozi.task(description);
            console.log(`${TUIFormatter.colors.green}✓ Task created: ${result}${TUIFormatter.colors.reset}`);
        } catch (error) {
            console.log(`${TUIFormatter.colors.red}Failed to create task: ${error}${TUIFormatter.colors.reset}`);
        }
    }

    private async completeTask(taskId: string): Promise<void> {
        try {
            await todozi.complete(taskId);
            console.log(`${TUIFormatter.colors.green}✓ Task ${taskId} completed${TUIFormatter.colors.reset}`);
        } catch (error) {
            console.log(`${TUIFormatter.colors.red}Failed to complete task: ${error}${TUIFormatter.colors.reset}`);
        }
    }

    private async handleQuickAdd(input: string): Promise<void> {
        if (!input) {
            console.log(`${TUIFormatter.colors.yellow}Usage: add <task description>${TUIFormatter.colors.reset}`);
            return;
        }

        await this.addTask(input);
    }

    private showStats(): void {
        const stats = todozi.getStatistics();
        console.log(TUIFormatter.formatStats(stats));
    }

    private showMainMenu(): void {
        console.log(`${TUIFormatter.colors.bright}Todozi TUI - Main Menu${TUIFormatter.colors.reset}`);
        console.log(`${TUIFormatter.colors.cyan}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${TUIFormatter.colors.reset}`);
        console.log();
        console.log(`${TUIFormatter.colors.green}Available commands:${TUIFormatter.colors.reset}`);
        console.log(`  ${TUIFormatter.colors.yellow}tasks${TUIFormatter.colors.reset} (t)  - Task management`);
        console.log(`  ${TUIFormatter.colors.yellow}memories${TUIFormatter.colors.reset} (m)  - Memory management`);
        console.log(`  ${TUIFormatter.colors.yellow}ideas${TUIFormatter.colors.reset} (i)  - Idea management`);
        console.log(`  ${TUIFormatter.colors.yellow}search${TUIFormatter.colors.reset} (s)  - Search all content`);
        console.log(`  ${TUIFormatter.colors.yellow}chat${TUIFormatter.colors.reset} (c)  - Chat with AI`);
        console.log(`  ${TUIFormatter.colors.yellow}add${TUIFormatter.colors.reset} (a)  - Quick add task`);
        console.log(`  ${TUIFormatter.colors.yellow}stats${TUIFormatter.colors.reset}  - System statistics`);
        console.log(`  ${TUIFormatter.colors.yellow}help${TUIFormatter.colors.reset} (h)  - Show this help`);
        console.log(`  ${TUIFormatter.colors.yellow}quit${TUIFormatter.colors.reset} (q)  - Exit`);
        console.log();
    }

    private showHelp(): void {
        this.showMainMenu();

        console.log(`${TUIFormatter.colors.bright}Command Examples:${TUIFormatter.colors.reset}`);
        console.log(`  add "Review project proposal"`);
        console.log(`  search "machine learning"`);
        console.log(`  chat "What are my high priority tasks?"`);
        console.log(`  tasks list`);
        console.log(`  tasks add "Implement user authentication"`);
        console.log();
    }

    private showPrompt(): void {
        const modeIndicator = this.state.mode === 'main' ? '' : `[${this.state.mode.toUpperCase()}] `;
        this.rl.setPrompt(`${modeIndicator}todozi> `);
        this.rl.prompt();
    }

    public async start(): Promise<void> {
        console.clear();
        console.log(`${TUIFormatter.colors.magenta}🚀 Welcome to Todozi TUI!${TUIFormatter.colors.reset}`);
        console.log();

        this.showMainMenu();
        this.showPrompt();
    }
}

// Export function to start TUI
export async function startTUI(): Promise<void> {
    const tui = new TodoziTUI();
    await tui.start();
}

// Handle direct execution
if (require.main === module) {
    startTUI().catch(error => {
        console.error('Failed to start TUI:', error);
        process.exit(1);
    });
}
