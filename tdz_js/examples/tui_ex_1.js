import { 
    TuiService, 
    TodoziApp, 
    DisplayConfig, 
    ColorScheme,
    Status,
    Priority,
    Assignee,
    Task
} from './tui.js';

// Mock embedding service for demonstration
class MockEmbeddingService {
    constructor() {
        this.tasks = new Map();
        this.similarityCache = new Map();
    }

    async addTask(task) {
        this.tasks.set(task.id, task);
        return task.id;
    }

    async getTask(taskId) {
        return this.tasks.get(taskId);
    }

    async findSimilarTasks(query, maxResults = 5) {
        // Simple similarity based on action words
        const queryWords = query.toLowerCase().split(' ');
        const results = [];
        
        for (const task of this.tasks.values()) {
            const taskWords = task.action.toLowerCase().split(' ');
            const commonWords = queryWords.filter(word => 
                taskWords.some(taskWord => taskWord.includes(word) || word.includes(taskWord))
            );
            
            if (commonWords.length > 0) {
                const similarity = commonWords.length / Math.max(queryWords.length, taskWords.length);
                results.push({
                    taskId: task.id,
                    similarity,
                    tags: task.tags || []
                });
            }
        }
        
        return results.sort((a, b) => b.similarity - a.similarity).slice(0, maxResults);
    }

    async semanticSearch(query, contentTypes, limit) {
        // Mock semantic search
        return this.findSimilarTasks(query, limit);
    }
}

// Create sample tasks
const sampleTasks = [
    new Task({
        action: "Implement user authentication system",
        time: "8 hours",
        priority: Priority.High,
        status: Status.InProgress,
        assignee: Assignee.Human,
        parentProject: "Authentication",
        tags: ["authentication", "security", "backend"],
        progress: 60
    }),
    new Task({
        action: "Design login page UI",
        time: "4 hours",
        priority: Priority.Medium,
        status: Status.Todo,
        assignee: Assignee.Human,
        parentProject: "Authentication",
        tags: ["ui", "design", "frontend"],
        progress: 0
    }),
    new Task({
        action: "Write API documentation for auth endpoints",
        time: "3 hours",
        priority: Priority.Low,
        status: Status.Review,
        assignee: Assignee.Ai,
        parentProject: "Authentication",
        tags: ["documentation", "api", "backend"],
        progress: 100
    }),
    new Task({
        action: "Fix password reset bug",
        time: "2 hours",
        priority: Priority.High,
        status: Status.Blocked,
        assignee: Assignee.Collaborative,
        parentProject: "Authentication",
        tags: ["bug", "security", "backend"],
        progress: 20
    })
];

// Task Dashboard Component
class TaskDashboard {
    constructor(embeddingService, displayConfig) {
        this.tuiService = new TuiService(embeddingService, displayConfig);
        this.app = new TodoziApp(embeddingService, displayConfig);
        this.tasks = [];
    }

    async initialize() {
        // Load sample tasks
        for (const task of sampleTasks) {
            await this.tuiService.embeddingService.addTask(task);
            this.tasks.push(task);
            this.app.tasks.push(task);
        }
        this.app.applyFilters();
    }

    async displayTaskWithInsights(taskId) {
        console.log('\n' + '='.repeat(60));
        console.log('📋 TASK DETAILS WITH AI INSIGHTS');
        console.log('='.repeat(60));
        
        const taskDisplay = await this.tuiService.displayTask(taskId);
        
        // Display basic task info
        const task = taskDisplay.task;
        const colorScheme = this.tuiService.displayConfig.colorScheme;
        
        console.log(`\n${colorScheme.primary.bold('Task:')} ${task.action}`);
        console.log(`${colorScheme.info('Status:')} ${task.status}`);
        console.log(`${colorScheme.warning('Priority:')} ${task.priority}`);
        console.log(`${colorScheme.secondary('Assignee:')} ${task.assignee || 'Unassigned'}`);
        console.log(`${colorScheme.muted('Project:')} ${task.parentProject}`);
        console.log(`${colorScheme.success('Progress:')} ${task.progress || 0}%`);
        
        if (task.tags.length > 0) {
            console.log(`${colorScheme.info('Tags:')} ${task.tags.map(tag => 
                colorScheme.primaryLight(`#${tag}`)).join(' ')}`);
        }

        // Display AI insights
        console.log(`\n${colorScheme.primary.bold('🤖 AI INSIGHTS')}`);
        console.log('-'.repeat(40));
        
        if (taskDisplay.aiSuggestions.length > 0) {
            console.log(colorScheme.success('💡 Suggestions:'));
            taskDisplay.aiSuggestions.forEach(suggestion => 
                console.log(`  • ${suggestion}`));
        }

        console.log(`${colorScheme.info('Confidence Score:')} ${(taskDisplay.confidenceScore * 100).toFixed(1)}%`);

        if (taskDisplay.similarTasks.length > 0) {
            console.log(`\n${colorScheme.warning('🔗 Similar Tasks:')}`);
            taskDisplay.similarTasks.forEach((similar, index) => {
                const similarTask = this.tasks.find(t => t.id === similar.taskId);
                if (similarTask) {
                    console.log(`  ${index + 1}. ${similarTask.action} (${(similar.similarity * 100).toFixed(1)}% similar)`);
                }
            });
        }

        if (taskDisplay.semanticTags.length > 0) {
            console.log(`\n${colorScheme.secondary('🏷️  Suggested Tags:')}`);
            console.log(`  ${taskDisplay.semanticTags.join(', ')}`);
        }
    }

    async displayTaskList() {
        console.log('\n' + '='.repeat(60));
        console.log('📊 TASK LIST OVERVIEW');
        console.log('='.repeat(60));
        
        const taskIds = this.tasks.map(t => t.id);
        const taskListDisplay = await this.tuiService.displayTasks(taskIds);
        
        const colorScheme = this.tuiService.displayConfig.colorScheme;
        
        console.log(`\n${colorScheme.primary.bold('Summary:')} ${taskListDisplay.aiSummary}`);
        
        console.log(`\n${colorScheme.info.bold('Tasks:')}`);
        this.tasks.forEach((task, index) => {
            const statusColor = task.status === Status.Done ? colorScheme.success :
                              task.status === Status.InProgress ? colorScheme.warning :
                              task.status === Status.Blocked ? colorScheme.danger : colorScheme.info;
            
            console.log(`${index + 1}. ${task.action}`);
            console.log(`   ${statusColor(`[${task.status}]`)} | ${colorScheme.muted(task.priority)} | ${task.progress || 0}%`);
        });

        if (taskListDisplay.semanticClusters.length > 0) {
            console.log(`\n${colorScheme.primary.bold('🎯 Semantic Clusters:')}`);
            taskListDisplay.semanticClusters.forEach((cluster, index) => {
                console.log(`  Cluster ${index + 1}: ${cluster.length} related tasks`);
            });
        }
    }

    async startInteractiveSession() {
        console.log('\n' + '='.repeat(60));
        console.log('🎮 INTERACTIVE TASK EDITING');
        console.log('='.repeat(60));
        
        // Start edit session for first task
        const taskId = this.tasks[0].id;
        const editSession = await this.tuiService.startEditSession(taskId);
        
        console.log(`\nEditing task: ${editSession.originalTask.action}`);
        console.log(`Found ${editSession.aiSuggestions.length} AI suggestions`);
        
        editSession.aiSuggestions.forEach((suggestion, index) => {
            console.log(`${index + 1}. ${suggestion}`);
        });

        return editSession;
    }

    getCompletionStats() {
        const completed = this.tasks.filter(t => 
            t.status === Status.Done || t.status === Status.Completed
        ).length;
        const total = this.tasks.length;
        const percentage = total > 0 ? Math.round((completed / total) * 100) : 0;
        
        return { completed, total, percentage };
    }

    displayProgressChart() {
        const { completed, total, percentage } = this.getCompletionStats();
        const colorScheme = this.tuiService.displayConfig.colorScheme;
        
        console.log('\n' + '='.repeat(60));
        console.log('📈 PROGRESS DASHBOARD');
        console.log('='.repeat(60));
        
        console.log(`\n${colorScheme.primary.bold('Completion:')} ${completed}/${total} tasks (${percentage}%)`);
        
        // Simple ASCII progress bar
        const barLength = 30;
        const filled = Math.round((percentage / 100) * barLength);
        const empty = barLength - filled;
        const bar = '█'.repeat(filled) + '░'.repeat(empty);
        
        console.log(`[${colorScheme.success(bar)}] ${percentage}%`);
        
        // Priority distribution
        const priorityCounts = {};
        this.tasks.forEach(task => {
            priorityCounts[task.priority] = (priorityCounts[task.priority] || 0) + 1;
        });
        
        console.log(`\n${colorScheme.primary.bold('Priority Distribution:')}`);
        Object.entries(priorityCounts).forEach(([priority, count]) => {
            const ratio = (count / total) * 100;
            console.log(`  ${priority}: ${count} (${ratio.toFixed(1)}%)`);
        });
    }
}

// Usage Example
async function runTaskDashboard() {
    try {
        // Configure display settings
        const displayConfig = new DisplayConfig();
        displayConfig.showAiInsights = true;
        displayConfig.showSimilarityScores = true;
        displayConfig.showRelatedTasks = true;
        displayConfig.maxRelatedTasks = 3;
        displayConfig.compactMode = false;
        
        // Use appropriate color scheme based on terminal capabilities
        if (ColorScheme.supportsTrueColor()) {
            displayConfig.colorScheme = ColorScheme.indexedColorScheme();
        } else {
            displayConfig.colorScheme = ColorScheme.ansiFallbackScheme();
        }

        // Initialize services
        const embeddingService = new MockEmbeddingService();
        const dashboard = new TaskDashboard(embeddingService, displayConfig);
        
        await dashboard.initialize();
        
        // Show loading screen
        await dashboard.tuiService.showLoadingScreen();
        
        // Display different views
        await dashboard.displayTaskList();
        await dashboard.displayTaskWithInsights(dashboard.tasks[0].id);
        dashboard.displayProgressChart();
        
        // Start interactive session
        const editSession = await dashboard.startInteractiveSession();
        console.log(`\nEdit session ready! Session ID: ${editSession.taskId}`);
        
    } catch (error) {
        console.error('❌ Error running dashboard:', error);
    }
}

// Export for use in other modules
export { TaskDashboard, MockEmbeddingService, runTaskDashboard };

// Run the example if this file is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
    runTaskDashboard();
}

/*
I'll create a practical example demonstrating how to create a Task Management Dashboard using the TUI components from the provided code.

## Example: Task Management Dashboard with AI Insights

This example shows how to create a simple dashboard that displays tasks with AI-powered similarity analysis and interactive editing capabilities.

## Key Features Demonstrated:

1. **AI-Powered Insights**: Shows similar tasks and AI suggestions based on semantic analysis
2. **Visual Progress Tracking**: ASCII progress bars and completion statistics
3. **Color-Coded Display**: Uses the `ColorScheme` class for terminal coloring
4. **Interactive Editing**: Prepares edit sessions with AI suggestions
5. **Task Filtering**: Uses the built-in filtering system from `TodoziApp`

## Usage:

/ *
bash
node task-dashboard.js
*/