import { Assignee, Priority } from './models.js';

// Configuration interfaces
export interface ServerConfig {
    host: string;
    port: number;
    maxConnections: number;
    apiKeyRequired: boolean;
    corsEnabled: boolean;
}

export interface EmbeddingConfig {
    model: string;
    dimensions: number;
    batchSize: number;
    cacheEnabled: boolean;
    similarityThreshold: number;
}

export interface DatabaseConfig {
    type: 'file' | 'sqlite' | 'postgres';
    path?: string;
    host?: string;
    port?: number;
    database?: string;
    username?: string;
    password?: string;
}

export interface TodoziConfig {
    server: ServerConfig;
    embedding: EmbeddingConfig;
    database: DatabaseConfig;
    defaultProject: string;
    defaultPriority: Priority;
    defaultAssignee: Assignee;
    autoSave: boolean;
    backupEnabled: boolean;
    backupInterval: number; // minutes
    logLevel: 'debug' | 'info' | 'warn' | 'error';
    apiKeys: string[];
}

// Configuration class
export class ConfigManager {
    private config: TodoziConfig;
    private configPath: string;

    constructor(configPath: string = './config.json') {
        this.configPath = configPath;
        this.config = this.getDefaultConfig();
    }

    private getDefaultConfig(): TodoziConfig {
        return {
            server: {
                host: 'localhost',
                port: 3000,
                maxConnections: 100,
                apiKeyRequired: false,
                corsEnabled: true
            },
            embedding: {
                model: 'all-MiniLM-L6-v2',
                dimensions: 384,
                batchSize: 32,
                cacheEnabled: true,
                similarityThreshold: 0.7
            },
            database: {
                type: 'file',
                path: './data'
            },
            defaultProject: 'default',
            defaultPriority: Priority.Medium,
            defaultAssignee: { type: Assignee.Human },
            autoSave: true,
            backupEnabled: true,
            backupInterval: 60,
            logLevel: 'info',
            apiKeys: []
        };
    }

    async load(): Promise<void> {
        try {
            const fs = await import('fs/promises');
            const data = await fs.readFile(this.configPath, 'utf-8');
            const loadedConfig = JSON.parse(data);

            // Merge with defaults to ensure all properties exist
            this.config = { ...this.getDefaultConfig(), ...loadedConfig };
        } catch (error) {
            // If config file doesn't exist, use defaults
            console.warn('Config file not found, using defaults');
        }
    }

    async save(): Promise<void> {
        try {
            const fs = await import('fs/promises');
            await fs.writeFile(this.configPath, JSON.stringify(this.config, null, 2));
        } catch (error) {
            throw new Error(`Failed to save config: ${error}`);
        }
    }

    get(): TodoziConfig {
        return { ...this.config };
    }

    update(updates: Partial<TodoziConfig>): void {
        this.config = { ...this.config, ...updates };
    }

    getServerConfig(): ServerConfig {
        return { ...this.config.server };
    }

    getEmbeddingConfig(): EmbeddingConfig {
        return { ...this.config.embedding };
    }

    getDatabaseConfig(): DatabaseConfig {
        return { ...this.config.database };
    }

    validate(): { valid: boolean; errors: string[] } {
        const errors: string[] = [];

        if (this.config.server.port < 1 || this.config.server.port > 65535) {
            errors.push('Server port must be between 1 and 65535');
        }

        if (this.config.embedding.dimensions < 1) {
            errors.push('Embedding dimensions must be positive');
        }

        if (this.config.backupInterval < 1) {
            errors.push('Backup interval must be positive');
        }

        return {
            valid: errors.length === 0,
            errors
        };
    }
}

// Global config instance
let globalConfig: ConfigManager | null = null;

export function getConfigManager(configPath?: string): ConfigManager {
    if (!globalConfig) {
        globalConfig = new ConfigManager(configPath);
    }
    return globalConfig;
}

export function getConfig(): TodoziConfig {
    if (!globalConfig) {
        throw new Error('Config manager not initialized. Call getConfigManager() first.');
    }
    return globalConfig.get();
}

export default ConfigManager;
