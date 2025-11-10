// Todozi TypeScript SDK - Main Entry Point
// =========================================

// Core exports
export * from './models.js';
export * from './error.js';
export * from './utils.js';
export * from './base.js';
export * from './config.js';

// Data management
export * from './storage.js';
export * from './validation.js';

// Content processing
export * from './extract.js';
export * from './chunking.js';

// AI and ML
export * from './agent.js';
export * from './emb.js';
export * from './training.js';

// Domain modules
export * from './task.js';
export * from './memory.js';
export * from './idea.js';
export * from './queue.js';
export * from './reminder.js';
export * from './summary.js';
export * from './tags.js';

// API and networking
export * from './api.js';
export * from './server.js';
export * from './tdz.js';
export * from './todozi.js';

// Legacy compatibility
export * from './lib.js';
export * from './migration.js';

// Main Todozi class for easy access to all functionality
import { TodoziStorage } from './storage.js';
import { EmbeddingManager } from './emb.js';
import { AgentManager, AgentRunner } from './agent.js';
import { SummaryManager, SummaryGenerator } from './summary.js';
import { TrainingDataManager, TrainingDataGenerator } from './training.js';
import { TodoziValidator } from './validation.js';

export class Todozi {
    private storage: TodoziStorage;
    private embedding: EmbeddingManager;
    private agents: AgentManager;
    private agentRunner: AgentRunner;
    private summaries: SummaryManager;
    private summaryGenerator: SummaryGenerator;
    private training: TrainingDataManager;
    private trainingGenerator: TrainingDataGenerator;
    private validator: TodoziValidator;

    constructor(storage?: TodoziStorage) {
        this.storage = storage || TodoziStorage.createFileStorage();

        // Initialize AI/ML components
        this.embedding = EmbeddingManager.createMock();
        this.agents = new AgentManager();
        this.agentRunner = new AgentRunner(this.agents);

        // Initialize content generators
        this.summaries = new SummaryManager();
        this.summaryGenerator = new SummaryGenerator(this.summaries);
        this.training = new TrainingDataManager();
        this.trainingGenerator = new TrainingDataGenerator(this.training);

        // Initialize validator
        this.validator = TodoziValidator;
    }

    // Storage access
    getStorage(): TodoziStorage {
        return this.storage;
    }

    // AI/ML access
    getEmbedding(): EmbeddingManager {
        return this.embedding;
    }

    getAgents(): AgentManager {
        return this.agents;
    }

    getAgentRunner(): AgentRunner {
        return this.agentRunner;
    }

    // Content generation access
    getSummaries(): SummaryManager {
        return this.summaries;
    }

    getSummaryGenerator(): SummaryGenerator {
        return this.summaryGenerator;
    }

    getTraining(): TrainingDataManager {
        return this.training;
    }

    getTrainingGenerator(): TrainingDataGenerator {
        return this.trainingGenerator;
    }

    // Validation access
    getValidator(): typeof TodoziValidator {
        return TodoziValidator;
    }

    // Utility methods
    async initialize(): Promise<void> {
        await this.storage.loadAll();
    }

    async save(): Promise<void> {
        await this.storage.saveAll();
    }

    getStatistics(): any {
        return {
            storage: this.storage.getStatistics(),
            embedding: this.embedding.getStatistics(),
            agents: this.agents.getAgentStatistics(),
            summaries: this.summaries.getSummaryStatistics(),
            training: this.training.getTrainingDataStatistics()
        };
    }

    async createBackup(name?: string): Promise<string | null> {
        const result = await this.storage.createBackup(name);
        return result.ok ? result.value : null;
    }

    async restoreBackup(name: string): Promise<boolean> {
        const result = await this.storage.restoreBackup(name);
        return result.ok;
    }

    clear(): void {
        this.storage.clear();
        this.embedding.clear();
        this.agents.clear();
        this.summaries.clear();
        this.training.clear();
    }

    // Factory methods
    static createInMemory(): Todozi {
        return new Todozi(TodoziStorage.createMemoryStorage());
    }

    static createFileStorage(basePath?: string): Todozi {
        return new Todozi(TodoziStorage.createFileStorage({ basePath }));
    }
}

// Export default instance
export const todozi = new Todozi();

// Export convenience functions
export const {
    task,
    urgent,
    high,
    low,
    ai: aiTask,
    human,
    collab,
    find,
    deep,
    fast,
    smart,
    tdz_find,
    create_memory,
    important,
    create_idea,
    breakthrough,
    complete,
    begin,
    quick,
    list_queue_items,
    chat,
    extract_tasks,
    plan_tasks,
    update_task_full,
    add_to_task
} = require('./todozi.js');

// Version info
export const VERSION = '1.0.0';
export const BUILD_DATE = new Date().toISOString();

// Health check
export function healthCheck(): {
    status: 'healthy' | 'degraded' | 'unhealthy';
    version: string;
    timestamp: string;
    components: Record<string, boolean>;
} {
    const components = {
        storage: true, // Assume healthy if loaded
        embedding: true,
        agents: true,
        summaries: true,
        training: true,
        validation: true
    };

    const healthyCount = Object.values(components).filter(Boolean).length;
    const totalCount = Object.keys(components).length;

    let status: 'healthy' | 'degraded' | 'unhealthy';
    if (healthyCount === totalCount) {
        status = 'healthy';
    } else if (healthyCount >= totalCount / 2) {
        status = 'degraded';
    } else {
        status = 'unhealthy';
    }

    return {
        status,
        version: VERSION,
        timestamp: new Date().toISOString(),
        components
    };
}
