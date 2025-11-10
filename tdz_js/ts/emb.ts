import {
    TodoziError,
    Result,
    Task,
    Memory,
    Idea,
    ErrorRecord,
    TrainingData
} from './models.js';
import { DateUtils, ArrayUtils } from './utils.js';

// Embedding configuration
export interface EmbeddingConfig {
    model: string;
    dimensions: number;
    batchSize: number;
    cacheEnabled: boolean;
    similarityThreshold: number;
    cacheTtlSeconds: number;
    clusteringThreshold: number;
    enableClustering: boolean;
    maxRetries: number;
    timeoutMs: number;
}

// Embedding vector type
export type EmbeddingVector = number[];

// Similarity result
export interface SimilarityResult {
    id: string;
    score: number;
    content: string;
    contentType: 'task' | 'memory' | 'idea' | 'error' | 'training';
}

// Clustering result
export interface ClusteringResult {
    clusterId: string;
    items: string[];
    centroid: EmbeddingVector;
    score: number;
}

// Embedding service interface
export interface EmbeddingService {
    generateEmbedding(text: string): Promise<Result<EmbeddingVector>>;
    generateEmbeddings(texts: string[]): Promise<Result<EmbeddingVector[]>>;
    calculateSimilarity(vec1: EmbeddingVector, vec2: EmbeddingVector): number;
    findSimilar(queryEmbedding: EmbeddingVector, embeddings: Map<string, EmbeddingVector>, limit?: number): SimilarityResult[];
    clusterEmbeddings(embeddings: Map<string, EmbeddingVector>, threshold?: number): ClusteringResult[];
}

// Cache entry
interface CacheEntry {
    embedding: EmbeddingVector;
    timestamp: number;
    ttl: number;
}

// Embedding cache
class EmbeddingCache {
    private cache: Map<string, CacheEntry> = new Map();
    private ttlSeconds: number;

    constructor(ttlSeconds: number = 3600) { // 1 hour default
        this.ttlSeconds = ttlSeconds;
    }

    get(key: string): EmbeddingVector | null {
        const entry = this.cache.get(key);
        if (!entry) return null;

        const now = Date.now();
        if (now - entry.timestamp > entry.ttl * 1000) {
            this.cache.delete(key);
            return null;
        }

        return entry.embedding;
    }

    set(key: string, embedding: EmbeddingVector, ttl?: number): void {
        this.cache.set(key, {
            embedding,
            timestamp: Date.now(),
            ttl: ttl || this.ttlSeconds
        });
    }

    clear(): void {
        this.cache.clear();
    }

    size(): number {
        return this.cache.size;
    }

    cleanup(): void {
        const now = Date.now();
        for (const [key, entry] of this.cache.entries()) {
            if (now - entry.timestamp > entry.ttl * 1000) {
                this.cache.delete(key);
            }
        }
    }
}

// Base embedding service implementation
export abstract class BaseEmbeddingService implements EmbeddingService {
    protected config: EmbeddingConfig;
    protected cache: EmbeddingCache;

    constructor(config: EmbeddingConfig) {
        this.config = config;
        this.cache = new EmbeddingCache(config.cacheTtlSeconds);
    }

    abstract generateEmbedding(text: string): Promise<Result<EmbeddingVector>>;
    abstract generateEmbeddings(texts: string[]): Promise<Result<EmbeddingVector[]>>;

    calculateSimilarity(vec1: EmbeddingVector, vec2: EmbeddingVector): number {
        if (vec1.length !== vec2.length) {
            throw new Error('Embedding vectors must have the same dimensions');
        }

        // Cosine similarity
        let dotProduct = 0;
        let norm1 = 0;
        let norm2 = 0;

        for (let i = 0; i < vec1.length; i++) {
            dotProduct += vec1[i] * vec2[i];
            norm1 += vec1[i] * vec1[i];
            norm2 += vec2[i] * vec2[i];
        }

        norm1 = Math.sqrt(norm1);
        norm2 = Math.sqrt(norm2);

        if (norm1 === 0 || norm2 === 0) {
            return 0;
        }

        return dotProduct / (norm1 * norm2);
    }

    findSimilar(
        queryEmbedding: EmbeddingVector,
        embeddings: Map<string, EmbeddingVector>,
        limit: number = 10
    ): SimilarityResult[] {
        const results: SimilarityResult[] = [];

        for (const [id, embedding] of embeddings.entries()) {
            const similarity = this.calculateSimilarity(queryEmbedding, embedding);
            if (similarity >= this.config.similarityThreshold) {
                results.push({
                    id,
                    score: similarity,
                    content: '', // Will be filled by caller
                    contentType: 'task' // Will be filled by caller
                });
            }
        }

        // Sort by similarity score (descending) and limit results
        return results
            .sort((a, b) => b.score - a.score)
            .slice(0, limit);
    }

    clusterEmbeddings(
        embeddings: Map<string, EmbeddingVector>,
        threshold: number = this.config.clusteringThreshold
    ): ClusteringResult[] {
        const clusters: ClusteringResult[] = [];
        const processed = new Set<string>();

        for (const [id, embedding] of embeddings.entries()) {
            if (processed.has(id)) continue;

            const cluster: string[] = [id];
            const centroid = [...embedding];
            processed.add(id);

            // Find similar embeddings for this cluster
            for (const [otherId, otherEmbedding] of embeddings.entries()) {
                if (processed.has(otherId) || otherId === id) continue;

                const similarity = this.calculateSimilarity(embedding, otherEmbedding);
                if (similarity >= threshold) {
                    cluster.push(otherId);
                    processed.add(otherId);

                    // Update centroid
                    for (let i = 0; i < centroid.length; i++) {
                        centroid[i] = (centroid[i] + otherEmbedding[i]) / 2;
                    }
                }
            }

            if (cluster.length > 1) { // Only include clusters with multiple items
                clusters.push({
                    clusterId: `cluster_${clusters.length}`,
                    items: cluster,
                    centroid,
                    score: cluster.length // Simple score based on cluster size
                });
            }
        }

        return clusters.sort((a, b) => b.score - a.score);
    }

    protected getCacheKey(text: string): string {
        // Simple hash function for cache key
        let hash = 0;
        for (let i = 0; i < text.length; i++) {
            const char = text.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash; // Convert to 32-bit integer
        }
        return hash.toString();
    }

    protected async getCachedEmbedding(text: string): Promise<EmbeddingVector | null> {
        if (!this.config.cacheEnabled) return null;
        return this.cache.get(this.getCacheKey(text));
    }

    protected setCachedEmbedding(text: string, embedding: EmbeddingVector): void {
        if (this.config.cacheEnabled) {
            this.cache.set(this.getCacheKey(text), embedding);
        }
    }

    clearCache(): void {
        this.cache.clear();
    }

    getCacheSize(): number {
        return this.cache.size();
    }

    cleanupCache(): void {
        this.cache.cleanup();
    }
}

// Mock embedding service for testing/development
export class MockEmbeddingService extends BaseEmbeddingService {
    constructor(config: EmbeddingConfig) {
        super(config);
    }

    async generateEmbedding(text: string): Promise<Result<EmbeddingVector>> {
        // Check cache first
        const cached = await this.getCachedEmbedding(text);
        if (cached) {
            return { ok: true, value: cached };
        }

        try {
            // Simulate API call delay
            await new Promise(resolve => setTimeout(resolve, 50));

            // Generate mock embedding (random vector normalized to unit length)
            const embedding: EmbeddingVector = [];
            let sumSquares = 0;

            for (let i = 0; i < this.config.dimensions; i++) {
                const value = (Math.random() - 0.5) * 2; // Random between -1 and 1
                embedding.push(value);
                sumSquares += value * value;
            }

            // Normalize to unit vector
            const norm = Math.sqrt(sumSquares);
            for (let i = 0; i < embedding.length; i++) {
                embedding[i] /= norm;
            }

            // Cache the result
            this.setCachedEmbedding(text, embedding);

            return { ok: true, value: embedding };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Mock embedding generation failed')
            };
        }
    }

    async generateEmbeddings(texts: string[]): Promise<Result<EmbeddingVector[]>> {
        try {
            const embeddings: EmbeddingVector[] = [];

            // Process in batches
            for (let i = 0; i < texts.length; i += this.config.batchSize) {
                const batch = texts.slice(i, i + this.config.batchSize);
                const batchPromises = batch.map(text => this.generateEmbedding(text));

                const batchResults = await Promise.all(batchPromises);

                for (const result of batchResults) {
                    if (result.ok) {
                        embeddings.push(result.value);
                    } else {
                        return result; // Return first error
                    }
                }
            }

            return { ok: true, value: embeddings };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Mock batch embedding generation failed')
            };
        }
    }
}

// OpenAI embedding service (placeholder for real implementation)
export class OpenAIEmbeddingService extends BaseEmbeddingService {
    private apiKey: string;

    constructor(config: EmbeddingConfig, apiKey: string) {
        super(config);
        this.apiKey = apiKey;
    }

    async generateEmbedding(text: string): Promise<Result<EmbeddingVector>> {
        // Check cache first
        const cached = await this.getCachedEmbedding(text);
        if (cached) {
            return { ok: true, value: cached };
        }

        try {
            // This would make a real API call to OpenAI
            // For now, fall back to mock implementation
            const mockService = new MockEmbeddingService(this.config);
            return await mockService.generateEmbedding(text);
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'OpenAI embedding generation failed')
            };
        }
    }

    async generateEmbeddings(texts: string[]): Promise<Result<EmbeddingVector[]>> {
        try {
            // This would make a real API call to OpenAI
            // For now, fall back to mock implementation
            const mockService = new MockEmbeddingService(this.config);
            return await mockService.generateEmbeddings(texts);
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'OpenAI batch embedding generation failed')
            };
        }
    }
}

// Embedding manager for handling different content types
export class EmbeddingManager {
    private service: EmbeddingService;
    private taskEmbeddings: Map<string, EmbeddingVector> = new Map();
    private memoryEmbeddings: Map<string, EmbeddingVector> = new Map();
    private ideaEmbeddings: Map<string, EmbeddingVector> = new Map();
    private errorEmbeddings: Map<string, EmbeddingVector> = new Map();
    private trainingEmbeddings: Map<string, EmbeddingVector> = new Map();

    constructor(service: EmbeddingService) {
        this.service = service;
    }

    static createMock(config?: Partial<EmbeddingConfig>): EmbeddingManager {
        const defaultConfig: EmbeddingConfig = {
            model: 'mock-embedding-model',
            dimensions: 384,
            batchSize: 10,
            cacheEnabled: true,
            similarityThreshold: 0.7,
            cacheTtlSeconds: 3600,
            clusteringThreshold: 0.8,
            enableClustering: true,
            maxRetries: 3,
            timeoutMs: 30000,
            ...config
        };

        return new EmbeddingManager(new MockEmbeddingService(defaultConfig));
    }

    async addTask(task: Task): Promise<Result<void>> {
        try {
            const content = this.taskToContent(task);
            const result = await this.service.generateEmbedding(content);

            if (result.ok) {
                this.taskEmbeddings.set(task.id, result.value);
                return { ok: true, value: undefined };
            } else {
                return result;
            }
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to embed task ${task.id}`)
            };
        }
    }

    async addMemory(memory: Memory): Promise<Result<void>> {
        try {
            const content = this.memoryToContent(memory);
            const result = await this.service.generateEmbedding(content);

            if (result.ok) {
                this.memoryEmbeddings.set(memory.id, result.value);
                return { ok: true, value: undefined };
            } else {
                return result;
            }
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to embed memory ${memory.id}`)
            };
        }
    }

    async addIdea(idea: Idea): Promise<Result<void>> {
        try {
            const content = this.ideaToContent(idea);
            const result = await this.service.generateEmbedding(content);

            if (result.ok) {
                this.ideaEmbeddings.set(idea.id, result.value);
                return { ok: true, value: undefined };
            } else {
                return result;
            }
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to embed idea ${idea.id}`)
            };
        }
    }

    async addError(error: ErrorRecord): Promise<Result<void>> {
        try {
            const content = this.errorToContent(error);
            const result = await this.service.generateEmbedding(content);

            if (result.ok) {
                this.errorEmbeddings.set(error.id, result.value);
                return { ok: true, value: undefined };
            } else {
                return result;
            }
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to embed error ${error.id}`)
            };
        }
    }

    async addTrainingData(trainingData: TrainingData): Promise<Result<void>> {
        try {
            const content = this.trainingDataToContent(trainingData);
            const result = await this.service.generateEmbedding(content);

            if (result.ok) {
                this.trainingEmbeddings.set(trainingData.id, result.value);
                return { ok: true, value: undefined };
            } else {
                return result;
            }
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Failed to embed training data ${trainingData.id}`)
            };
        }
    }

    async findSimilarTasks(query: string, limit: number = 10): Promise<Result<SimilarityResult[]>> {
        try {
            const queryResult = await this.service.generateEmbedding(query);
            if (!queryResult.ok) return queryResult;

            const results = this.service.findSimilar(queryResult.value, this.taskEmbeddings, limit);

            // Add content and content type
            for (const result of results) {
                result.content = 'Task content'; // Would need to look up actual content
                result.contentType = 'task';
            }

            return { ok: true, value: results };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Task similarity search failed')
            };
        }
    }

    async findSimilarMemories(query: string, limit: number = 10): Promise<Result<SimilarityResult[]>> {
        try {
            const queryResult = await this.service.generateEmbedding(query);
            if (!queryResult.ok) return queryResult;

            const results = this.service.findSimilar(queryResult.value, this.memoryEmbeddings, limit);

            for (const result of results) {
                result.content = 'Memory content';
                result.contentType = 'memory';
            }

            return { ok: true, value: results };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Memory similarity search failed')
            };
        }
    }

    async unifiedSearch(query: string, limit: number = 10): Promise<Result<SimilarityResult[]>> {
        try {
            const queryResult = await this.service.generateEmbedding(query);
            if (!queryResult.ok) return queryResult;

            const allEmbeddings = new Map<string, EmbeddingVector>();

            // Combine all embeddings with prefixed IDs
            for (const [id, embedding] of this.taskEmbeddings.entries()) {
                allEmbeddings.set(`task_${id}`, embedding);
            }
            for (const [id, embedding] of this.memoryEmbeddings.entries()) {
                allEmbeddings.set(`memory_${id}`, embedding);
            }
            for (const [id, embedding] of this.ideaEmbeddings.entries()) {
                allEmbeddings.set(`idea_${id}`, embedding);
            }
            for (const [id, embedding] of this.errorEmbeddings.entries()) {
                allEmbeddings.set(`error_${id}`, embedding);
            }

            const results = this.service.findSimilar(queryResult.value, allEmbeddings, limit);

            // Add content type based on ID prefix
            for (const result of results) {
                const [type, id] = result.id.split('_', 2);
                result.id = id;
                result.contentType = type as 'task' | 'memory' | 'idea' | 'error';
                result.content = `${type} content`;
            }

            return { ok: true, value: results };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Unified similarity search failed')
            };
        }
    }

    getClusters(): ClusteringResult[] {
        const allEmbeddings = new Map<string, EmbeddingVector>();

        // Combine all embeddings
        for (const [id, embedding] of this.taskEmbeddings.entries()) {
            allEmbeddings.set(`task_${id}`, embedding);
        }
        for (const [id, embedding] of this.memoryEmbeddings.entries()) {
            allEmbeddings.set(`memory_${id}`, embedding);
        }
        for (const [id, embedding] of this.ideaEmbeddings.entries()) {
            allEmbeddings.set(`idea_${id}`, embedding);
        }

        return this.service.clusterEmbeddings(allEmbeddings);
    }

    clear(): void {
        this.taskEmbeddings.clear();
        this.memoryEmbeddings.clear();
        this.ideaEmbeddings.clear();
        this.errorEmbeddings.clear();
        this.trainingEmbeddings.clear();
        this.service.clearCache?.();
    }

    getStatistics(): {
        tasksEmbedded: number;
        memoriesEmbedded: number;
        ideasEmbedded: number;
        errorsEmbedded: number;
        trainingDataEmbedded: number;
        totalEmbedded: number;
        cacheSize: number;
        clustersFound: number;
    } {
        const clusters = this.getClusters();

        return {
            tasksEmbedded: this.taskEmbeddings.size,
            memoriesEmbedded: this.memoryEmbeddings.size,
            ideasEmbedded: this.ideaEmbeddings.size,
            errorsEmbedded: this.errorEmbeddings.size,
            trainingDataEmbedded: this.trainingEmbeddings.size,
            totalEmbedded: this.taskEmbeddings.size + this.memoryEmbeddings.size +
                          this.ideaEmbeddings.size + this.errorEmbeddings.size + this.trainingEmbeddings.size,
            cacheSize: this.service.getCacheSize?.() || 0,
            clustersFound: clusters.length
        };
    }

    private taskToContent(task: Task): string {
        return `${task.action} ${task.time} ${task.priority} ${task.parentProject} ${task.contextNotes || ''} ${task.tags.join(' ')}`;
    }

    private memoryToContent(memory: Memory): string {
        return `${memory.moment} ${memory.meaning} ${memory.reason} ${memory.importance} ${memory.term} ${memory.tags.join(' ')}`;
    }

    private ideaToContent(idea: Idea): string {
        return `${idea.idea} ${idea.importance} ${idea.share} ${idea.context || ''} ${idea.tags.join(' ')}`;
    }

    private errorToContent(error: ErrorRecord): string {
        return `${error.title} ${error.description} ${error.category} ${error.severity} ${error.source} ${error.context || ''} ${error.tags.join(' ')}`;
    }

    private trainingDataToContent(trainingData: TrainingData): string {
        return `${trainingData.prompt} ${trainingData.completion} ${trainingData.context || ''} ${trainingData.tags.join(' ')}`;
    }
}

// Export singleton instance
export const embeddingManager = EmbeddingManager.createMock();
