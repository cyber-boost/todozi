import {
    Task,
    TaskClass,
    Memory,
    Idea,
    ErrorRecord,
    Summary,
    TrainingData,
    Agent,
    Reminder,
    QueueItem,
    ApiKey,
    Project,
    Tag,
    Feeling,
    TodoziError,
    Result
} from './models.js';
import { DateUtils, ArrayUtils } from './utils.js';
import * as fs from 'fs';
import * as path from 'path';

// Storage configuration
export interface StorageConfig {
    basePath: string;
    autoSave: boolean;
    saveInterval: number; // milliseconds
    maxBackups: number;
    compressionEnabled: boolean;
}

// Storage backend interface
export interface StorageBackend {
    save(key: string, data: any): Promise<Result<void>>;
    load(key: string): Promise<Result<any>>;
    exists(key: string): Promise<boolean>;
    delete(key: string): Promise<Result<void>>;
    list(prefix?: string): Promise<Result<string[]>>;
    clear(): Promise<Result<void>>;
}

// File system storage backend
export class FileStorageBackend implements StorageBackend {
    constructor(private basePath: string) {
        // Ensure base directory exists
        if (!fs.existsSync(basePath)) {
            fs.mkdirSync(basePath, { recursive: true });
        }
    }

    async save(key: string, data: any): Promise<Result<void>> {
        try {
            const filePath = this.getFilePath(key);
            const dirPath = path.dirname(filePath);

            // Ensure directory exists
            if (!fs.existsSync(dirPath)) {
                fs.mkdirSync(dirPath, { recursive: true });
            }

            const jsonData = JSON.stringify(data, null, 2);
            fs.writeFileSync(filePath, jsonData, 'utf8');

            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to save ${key}`)
            };
        }
    }

    async load(key: string): Promise<Result<any>> {
        try {
            const filePath = this.getFilePath(key);

            if (!fs.existsSync(filePath)) {
                return {
                    ok: false,
                    error: TodoziError.notFound(`File not found: ${key}`)
                };
            }

            const jsonData = fs.readFileSync(filePath, 'utf8');
            const data = JSON.parse(jsonData);

            return { ok: true, value: data };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to load ${key}`)
            };
        }
    }

    async exists(key: string): Promise<boolean> {
        const filePath = this.getFilePath(key);
        return fs.existsSync(filePath);
    }

    async delete(key: string): Promise<Result<void>> {
        try {
            const filePath = this.getFilePath(key);

            if (fs.existsSync(filePath)) {
                fs.unlinkSync(filePath);
            }

            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to delete ${key}`)
            };
        }
    }

    async list(prefix?: string): Promise<Result<string[]>> {
        try {
            const searchPath = prefix ? path.join(this.basePath, prefix) : this.basePath;
            const files: string[] = [];

            function scanDirectory(dirPath: string, relativePath: string = '') {
                if (!fs.existsSync(dirPath)) return;

                const items = fs.readdirSync(dirPath);

                for (const item of items) {
                    const fullPath = path.join(dirPath, item);
                    const relPath = path.join(relativePath, item);
                    const stat = fs.statSync(fullPath);

                    if (stat.isDirectory()) {
                        scanDirectory(fullPath, relPath);
                    } else if (item.endsWith('.json')) {
                        files.push(relPath.replace(/\.json$/, ''));
                    }
                }
            }

            scanDirectory(searchPath);
            return { ok: true, value: files };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Failed to list files')
            };
        }
    }

    async clear(): Promise<Result<void>> {
        try {
            function deleteDirectory(dirPath: string) {
                if (!fs.existsSync(dirPath)) return;

                const items = fs.readdirSync(dirPath);

                for (const item of items) {
                    const fullPath = path.join(dirPath, item);
                    const stat = fs.statSync(fullPath);

                    if (stat.isDirectory()) {
                        deleteDirectory(fullPath);
                        fs.rmdirSync(fullPath);
                    } else {
                        fs.unlinkSync(fullPath);
                    }
                }
            }

            deleteDirectory(this.basePath);
            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Failed to clear storage')
            };
        }
    }

    private getFilePath(key: string): string {
        return path.join(this.basePath, `${key}.json`);
    }
}

// In-memory storage backend for testing
export class MemoryStorageBackend implements StorageBackend {
    private data: Map<string, any> = new Map();

    async save(key: string, data: any): Promise<Result<void>> {
        this.data.set(key, data);
        return { ok: true, value: undefined };
    }

    async load(key: string): Promise<Result<any>> {
        const value = this.data.get(key);
        if (value === undefined) {
            return {
                ok: false,
                error: TodoziError.notFound(`Key not found: ${key}`)
            };
        }
        return { ok: true, value };
    }

    async exists(key: string): Promise<boolean> {
        return this.data.has(key);
    }

    async delete(key: string): Promise<Result<void>> {
        this.data.delete(key);
        return { ok: true, value: undefined };
    }

    async list(prefix?: string): Promise<Result<string[]>> {
        const keys = Array.from(this.data.keys());
        const filteredKeys = prefix ? keys.filter(key => key.startsWith(prefix)) : keys;
        return { ok: true, value: filteredKeys };
    }

    async clear(): Promise<Result<void>> {
        this.data.clear();
        return { ok: true, value: undefined };
    }
}

// Main storage manager
export class TodoziStorage {
    private backend: StorageBackend;
    private config: StorageConfig;
    private autoSaveTimer?: NodeJS.Timeout;

    // Data collections
    private tasks: Map<string, Task> = new Map();
    private memories: Map<string, Memory> = new Map();
    private ideas: Map<string, Idea> = new Map();
    private errors: Map<string, ErrorRecord> = new Map();
    private summaries: Map<string, Summary> = new Map();
    private trainingData: Map<string, TrainingData> = new Map();
    private agents: Map<string, Agent> = new Map();
    private reminders: Map<string, Reminder> = new Map();
    private queueItems: Map<string, QueueItem> = new Map();
    private apiKeys: Map<string, ApiKey> = new Map();
    private projects: Map<string, Project> = new Map();
    private tags: Map<string, Tag> = new Map();
    private feelings: Map<string, Feeling> = new Map();

    constructor(config: StorageConfig, backend?: StorageBackend) {
        this.config = config;
        this.backend = backend || new FileStorageBackend(config.basePath);

        if (config.autoSave) {
            this.startAutoSave();
        }
    }

    static createFileStorage(config?: Partial<StorageConfig>): TodoziStorage {
        const defaultConfig: StorageConfig = {
            basePath: './todozi_data',
            autoSave: true,
            saveInterval: 30000, // 30 seconds
            maxBackups: 10,
            compressionEnabled: false,
            ...config
        };

        return new TodoziStorage(defaultConfig);
    }

    static createMemoryStorage(config?: Partial<StorageConfig>): TodoziStorage {
        const defaultConfig: StorageConfig = {
            basePath: '',
            autoSave: false,
            saveInterval: 0,
            maxBackups: 0,
            compressionEnabled: false,
            ...config
        };

        return new TodoziStorage(defaultConfig, new MemoryStorageBackend());
    }

    // Task operations
    async saveTask(task: Task): Promise<Result<void>> {
        this.tasks.set(task.id, task);
        if (this.config.autoSave) {
            return await this.backend.save(`tasks/${task.id}`, task);
        }
        return { ok: true, value: undefined };
    }

    async loadTask(id: string): Promise<Result<Task | null>> {
        // Try memory first
        const memoryTask = this.tasks.get(id);
        if (memoryTask) {
            return { ok: true, value: memoryTask };
        }

        // Try backend
        const result = await this.backend.load(`tasks/${id}`);
        if (result.ok) {
            this.tasks.set(id, result.value);
            return { ok: true, value: result.value };
        } else if (result.error.code === 'NOT_FOUND') {
            return { ok: true, value: null };
        }

        return result;
    }

    async deleteTask(id: string): Promise<Result<void>> {
        this.tasks.delete(id);
        return await this.backend.delete(`tasks/${id}`);
    }

    getAllTasks(): Task[] {
        return Array.from(this.tasks.values());
    }

    // Memory operations
    async saveMemory(memory: Memory): Promise<Result<void>> {
        this.memories.set(memory.id, memory);
        if (this.config.autoSave) {
            return await this.backend.save(`memories/${memory.id}`, memory);
        }
        return { ok: true, value: undefined };
    }

    async loadMemory(id: string): Promise<Result<Memory | null>> {
        const memoryMemory = this.memories.get(id);
        if (memoryMemory) {
            return { ok: true, value: memoryMemory };
        }

        const result = await this.backend.load(`memories/${id}`);
        if (result.ok) {
            this.memories.set(id, result.value);
            return { ok: true, value: result.value };
        } else if (result.error.code === 'NOT_FOUND') {
            return { ok: true, value: null };
        }

        return result;
    }

    // Idea operations
    async saveIdea(idea: Idea): Promise<Result<void>> {
        this.ideas.set(idea.id, idea);
        if (this.config.autoSave) {
            return await this.backend.save(`ideas/${idea.id}`, idea);
        }
        return { ok: true, value: undefined };
    }

    async loadIdea(id: string): Promise<Result<Idea | null>> {
        const memoryIdea = this.ideas.get(id);
        if (memoryIdea) {
            return { ok: true, value: memoryIdea };
        }

        const result = await this.backend.load(`ideas/${id}`);
        if (result.ok) {
            this.ideas.set(id, result.value);
            return { ok: true, value: result.value };
        } else if (result.error.code === 'NOT_FOUND') {
            return { ok: true, value: null };
        }

        return result;
    }

    // Error operations
    async saveError(error: ErrorRecord): Promise<Result<void>> {
        this.errors.set(error.id, error);
        if (this.config.autoSave) {
            return await this.backend.save(`errors/${error.id}`, error);
        }
        return { ok: true, value: undefined };
    }

    // Summary operations
    async saveSummary(summary: Summary): Promise<Result<void>> {
        this.summaries.set(summary.id, summary);
        if (this.config.autoSave) {
            return await this.backend.save(`summaries/${summary.id}`, summary);
        }
        return { ok: true, value: undefined };
    }

    // Training data operations
    async saveTrainingData(trainingData: TrainingData): Promise<Result<void>> {
        this.trainingData.set(trainingData.id, trainingData);
        if (this.config.autoSave) {
            return await this.backend.save(`training/${trainingData.id}`, trainingData);
        }
        return { ok: true, value: undefined };
    }

    // Agent operations
    async saveAgent(agent: Agent): Promise<Result<void>> {
        this.agents.set(agent.id, agent);
        if (this.config.autoSave) {
            return await this.backend.save(`agents/${agent.id}`, agent);
        }
        return { ok: true, value: undefined };
    }

    // Bulk operations
    async saveAll(): Promise<Result<void>> {
        const errors: TodoziError[] = [];

        // Save all tasks
        for (const task of this.tasks.values()) {
            const result = await this.backend.save(`tasks/${task.id}`, task);
            if (!result.ok) errors.push(result.error);
        }

        // Save all memories
        for (const memory of this.memories.values()) {
            const result = await this.backend.save(`memories/${memory.id}`, memory);
            if (!result.ok) errors.push(result.error);
        }

        // Save all ideas
        for (const idea of this.ideas.values()) {
            const result = await this.backend.save(`ideas/${idea.id}`, idea);
            if (!result.ok) errors.push(result.error);
        }

        // Save all errors
        for (const error of this.errors.values()) {
            const result = await this.backend.save(`errors/${error.id}`, error);
            if (!result.ok) errors.push(result.error);
        }

        // Save all summaries
        for (const summary of this.summaries.values()) {
            const result = await this.backend.save(`summaries/${summary.id}`, summary);
            if (!result.ok) errors.push(result.error);
        }

        // Save all training data
        for (const trainingData of this.trainingData.values()) {
            const result = await this.backend.save(`training/${trainingData.id}`, trainingData);
            if (!result.ok) errors.push(result.error);
        }

        // Save all agents
        for (const agent of this.agents.values()) {
            const result = await this.backend.save(`agents/${agent.id}`, agent);
            if (!result.ok) errors.push(result.error);
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Bulk save failed: ${errors.length} errors`)
            };
        }

        return { ok: true, value: undefined };
    }

    async loadAll(): Promise<Result<void>> {
        const errors: TodoziError[] = [];

        // Load tasks
        const taskListResult = await this.backend.list('tasks/');
        if (taskListResult.ok) {
            for (const taskKey of taskListResult.value) {
                const result = await this.backend.load(taskKey);
                if (result.ok) {
                    this.tasks.set(result.value.id, result.value);
                } else {
                    errors.push(result.error);
                }
            }
        }

        // Load memories
        const memoryListResult = await this.backend.list('memories/');
        if (memoryListResult.ok) {
            for (const memoryKey of memoryListResult.value) {
                const result = await this.backend.load(memoryKey);
                if (result.ok) {
                    this.memories.set(result.value.id, result.value);
                } else {
                    errors.push(result.error);
                }
            }
        }

        // Load ideas
        const ideaListResult = await this.backend.list('ideas/');
        if (ideaListResult.ok) {
            for (const ideaKey of ideaListResult.value) {
                const result = await this.backend.load(ideaKey);
                if (result.ok) {
                    this.ideas.set(result.value.id, result.value);
                } else {
                    errors.push(result.error);
                }
            }
        }

        // Similar loading for other collections...

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Bulk load failed: ${errors.length} errors`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Backup operations
    async createBackup(name?: string): Promise<Result<string>> {
        const backupName = name || `backup_${Date.now()}`;
        const backupPath = `backups/${backupName}`;

        try {
            // Create backup directory
            const fullBackupPath = path.join(this.config.basePath, backupPath);
            if (!fs.existsSync(fullBackupPath)) {
                fs.mkdirSync(fullBackupPath, { recursive: true });
            }

            // Copy all data to backup
            const data = {
                tasks: Object.fromEntries(this.tasks),
                memories: Object.fromEntries(this.memories),
                ideas: Object.fromEntries(this.ideas),
                errors: Object.fromEntries(this.errors),
                summaries: Object.fromEntries(this.summaries),
                trainingData: Object.fromEntries(this.trainingData),
                agents: Object.fromEntries(this.agents),
                metadata: {
                    createdAt: new Date().toISOString(),
                    version: '1.0.0',
                    itemCounts: {
                        tasks: this.tasks.size,
                        memories: this.memories.size,
                        ideas: this.ideas.size,
                        errors: this.errors.size,
                        summaries: this.summaries.size,
                        trainingData: this.trainingData.size,
                        agents: this.agents.size
                    }
                }
            };

            await this.backend.save(`${backupPath}/data`, data);

            // Clean up old backups
            await this.cleanupOldBackups();

            return { ok: true, value: backupName };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Backup creation failed: ${backupName}`)
            };
        }
    }

    async restoreBackup(backupName: string): Promise<Result<void>> {
        try {
            const backupKey = `backups/${backupName}/data`;
            const result = await this.backend.load(backupKey);

            if (!result.ok) return result;

            const data = result.value;

            // Clear current data
            this.clear();

            // Restore data
            if (data.tasks) {
                this.tasks = new Map(Object.entries(data.tasks));
            }
            if (data.memories) {
                this.memories = new Map(Object.entries(data.memories));
            }
            if (data.ideas) {
                this.ideas = new Map(Object.entries(data.ideas));
            }
            if (data.errors) {
                this.errors = new Map(Object.entries(data.errors));
            }
            if (data.summaries) {
                this.summaries = new Map(Object.entries(data.summaries));
            }
            if (data.trainingData) {
                this.trainingData = new Map(Object.entries(data.trainingData));
            }
            if (data.agents) {
                this.agents = new Map(Object.entries(data.agents));
            }

            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Backup restore failed: ${backupName}`)
            };
        }
    }

    async listBackups(): Promise<Result<string[]>> {
        const result = await this.backend.list('backups/');
        if (!result.ok) return result;

        return {
            ok: true,
            value: result.value.map(backup => backup.replace('backups/', ''))
        };
    }

    // Statistics
    getStatistics(): {
        tasks: number;
        memories: number;
        ideas: number;
        errors: number;
        summaries: number;
        trainingData: number;
        agents: number;
        total: number;
    } {
        const stats = {
            tasks: this.tasks.size,
            memories: this.memories.size,
            ideas: this.ideas.size,
            errors: this.errors.size,
            summaries: this.summaries.size,
            trainingData: this.trainingData.size,
            agents: this.agents.size,
            total: 0
        };

        stats.total = Object.values(stats).reduce((sum, count) => sum + count, 0) - stats.total;
        return stats;
    }

    // Utility methods
    clear(): void {
        this.tasks.clear();
        this.memories.clear();
        this.ideas.clear();
        this.errors.clear();
        this.summaries.clear();
        this.trainingData.clear();
        this.agents.clear();
        this.reminders.clear();
        this.queueItems.clear();
        this.apiKeys.clear();
        this.projects.clear();
        this.tags.clear();
        this.feelings.clear();
    }

    private startAutoSave(): void {
        this.autoSaveTimer = setInterval(async () => {
            await this.saveAll();
        }, this.config.saveInterval);
    }

    private stopAutoSave(): void {
        if (this.autoSaveTimer) {
            clearInterval(this.autoSaveTimer);
            this.autoSaveTimer = undefined;
        }
    }

    private async cleanupOldBackups(): Promise<void> {
        const listResult = await this.listBackups();
        if (!listResult.ok) return;

        const backups = listResult.value
            .sort()
            .reverse(); // Most recent first

        if (backups.length > this.config.maxBackups) {
            const toDelete = backups.slice(this.config.maxBackups);

            for (const backup of toDelete) {
                await this.backend.delete(`backups/${backup}`);
            }
        }
    }

    // Cleanup
    destroy(): void {
        this.stopAutoSave();
        this.clear();
    }
}

// Export singleton instances
export const fileStorage = TodoziStorage.createFileStorage();
export const memoryStorage = TodoziStorage.createMemoryStorage();
