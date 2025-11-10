import { Task, Memory, Idea, Error as TodoziErrorRecord } from './models.js';
import { TodoziEmbeddingService } from './extract.js';

// Search result types
export interface TaskSearchResult {
    task: Task;
    score: number;
    matches: string[];
}

export interface MemorySearchResult {
    memory: Memory;
    score: number;
    matches: string[];
}

export interface IdeaSearchResult {
    idea: Idea;
    score: number;
    matches: string[];
}

export interface ErrorSearchResult {
    error: TodoziErrorRecord;
    score: number;
    matches: string[];
}

export interface UnifiedSearchResults {
    tasks: TaskSearchResult[];
    memories: MemorySearchResult[];
    ideas: IdeaSearchResult[];
    errors: ErrorSearchResult[];
    totalCount: number;
    queryTime: number;
}

// Search filters
export interface SearchFilters {
    project?: string;
    status?: string;
    priority?: string;
    assignee?: string;
    tags?: string[];
    dateFrom?: Date;
    dateTo?: Date;
    limit?: number;
    offset?: number;
}

// Search options
export interface SearchOptions {
    useEmbeddings?: boolean;
    fuzzy?: boolean;
    caseSensitive?: boolean;
    exactMatch?: boolean;
    includeArchived?: boolean;
    sortBy?: 'relevance' | 'date' | 'priority';
    sortOrder?: 'asc' | 'desc';
}

// Search engine class
export class SearchEngine {
    private embeddingService?: TodoziEmbeddingService;
    private taskIndex: Map<string, Task> = new Map();
    private memoryIndex: Map<string, Memory> = new Map();
    private ideaIndex: Map<string, Idea> = new Map();
    private errorIndex: Map<string, TodoziErrorRecord> = new Map();

    constructor(embeddingService?: TodoziEmbeddingService) {
        this.embeddingService = embeddingService;
    }

    // Index management
    addTask(task: Task): void {
        this.taskIndex.set(task.id, task);
    }

    addMemory(memory: Memory): void {
        this.memoryIndex.set(memory.id, memory);
    }

    addIdea(idea: Idea): void {
        this.ideaIndex.set(idea.id, idea);
    }

    addError(error: TodoziErrorRecord): void {
        this.errorIndex.set(error.id, error);
    }

    removeTask(taskId: string): void {
        this.taskIndex.delete(taskId);
    }

    removeMemory(memoryId: string): void {
        this.memoryIndex.delete(memoryId);
    }

    removeIdea(ideaId: string): void {
        this.ideaIndex.delete(ideaId);
    }

    removeError(errorId: string): void {
        this.errorIndex.delete(errorId);
    }

    clearIndex(): void {
        this.taskIndex.clear();
        this.memoryIndex.clear();
        this.ideaIndex.clear();
        this.errorIndex.clear();
    }

    // Search methods
    async searchTasks(query: string, filters?: SearchFilters, options?: SearchOptions): Promise<TaskSearchResult[]> {
        const startTime = Date.now();
        const tasks = Array.from(this.taskIndex.values());

        let filteredTasks = this.applyFilters(tasks, filters);

        if (options?.useEmbeddings && this.embeddingService) {
            return this.semanticSearch(query, filteredTasks, options);
        } else {
            return this.textSearch(query, filteredTasks, options);
        }
    }

    async searchMemories(query: string, filters?: SearchFilters, options?: SearchOptions): Promise<MemorySearchResult[]> {
        const memories = Array.from(this.memoryIndex.values());
        let filteredMemories = this.applyMemoryFilters(memories, filters);

        if (options?.useEmbeddings && this.embeddingService) {
            return this.semanticSearchMemories(query, filteredMemories, options);
        } else {
            return this.textSearchMemories(query, filteredMemories, options);
        }
    }

    async searchIdeas(query: string, filters?: SearchFilters, options?: SearchOptions): Promise<IdeaSearchResult[]> {
        const ideas = Array.from(this.ideaIndex.values());
        let filteredIdeas = this.applyIdeaFilters(ideas, filters);

        if (options?.useEmbeddings && this.embeddingService) {
            return this.semanticSearchIdeas(query, filteredIdeas, options);
        } else {
            return this.textSearchIdeas(query, filteredIdeas, options);
        }
    }

    async searchErrors(query: string, filters?: SearchFilters, options?: SearchOptions): Promise<ErrorSearchResult[]> {
        const errors = Array.from(this.errorIndex.values());
        let filteredErrors = this.applyErrorFilters(errors, filters);

        return this.textSearchErrors(query, filteredErrors, options);
    }

    async unifiedSearch(query: string, filters?: SearchFilters, options?: SearchOptions): Promise<UnifiedSearchResults> {
        const startTime = Date.now();

        const [tasks, memories, ideas, errors] = await Promise.all([
            this.searchTasks(query, filters, options),
            this.searchMemories(query, filters, options),
            this.searchIdeas(query, filters, options),
            this.searchErrors(query, filters, options)
        ]);

        const queryTime = Date.now() - startTime;
        const totalCount = tasks.length + memories.length + ideas.length + errors.length;

        return {
            tasks,
            memories,
            ideas,
            errors,
            totalCount,
            queryTime
        };
    }

    // Private search implementations
    private textSearch(query: string, tasks: Task[], options?: SearchOptions): TaskSearchResult[] {
        const results: TaskSearchResult[] = [];
        const searchQuery = options?.caseSensitive ? query : query.toLowerCase();

        for (const task of tasks) {
            const matches = this.findMatches(searchQuery, task, options);
            if (matches.length > 0) {
                const score = this.calculateRelevanceScore(searchQuery, task, matches);
                results.push({ task, score, matches });
            }
        }

        return this.sortAndLimit(results, options);
    }

    private textSearchMemories(query: string, memories: Memory[], options?: SearchOptions): MemorySearchResult[] {
        const results: MemorySearchResult[] = [];
        const searchQuery = options?.caseSensitive ? query : query.toLowerCase();

        for (const memory of memories) {
            const matches = this.findMatchesInMemory(searchQuery, memory, options);
            if (matches.length > 0) {
                const score = this.calculateMemoryRelevanceScore(searchQuery, memory, matches);
                results.push({ memory, score, matches });
            }
        }

        return this.sortAndLimit(results, options);
    }

    private textSearchIdeas(query: string, ideas: Idea[], options?: SearchOptions): IdeaSearchResult[] {
        const results: IdeaSearchResult[] = [];
        const searchQuery = options?.caseSensitive ? query : query.toLowerCase();

        for (const idea of ideas) {
            const matches = this.findMatchesInIdea(searchQuery, idea, options);
            if (matches.length > 0) {
                const score = this.calculateIdeaRelevanceScore(searchQuery, idea, matches);
                results.push({ idea, score, matches });
            }
        }

        return this.sortAndLimit(results, options);
    }

    private textSearchErrors(query: string, errors: TodoziErrorRecord[], options?: SearchOptions): ErrorSearchResult[] {
        const results: ErrorSearchResult[] = [];
        const searchQuery = options?.caseSensitive ? query : query.toLowerCase();

        for (const error of errors) {
            const matches = this.findMatchesInError(searchQuery, error, options);
            if (matches.length > 0) {
                const score = this.calculateErrorRelevanceScore(searchQuery, error, matches);
                results.push({ error, score, matches });
            }
        }

        return this.sortAndLimit(results, options);
    }

    private async semanticSearch(query: string, tasks: Task[], options?: SearchOptions): Promise<TaskSearchResult[]> {
        if (!this.embeddingService) {
            return this.textSearch(query, tasks, options);
        }

        try {
            const queryEmbedding = await this.embeddingService.generateEmbedding(query);
            const results: TaskSearchResult[] = [];

            for (const task of tasks) {
                const taskText = `${task.action} ${task.contextNotes || ''}`;
                const taskEmbedding = await this.embeddingService.generateEmbedding(taskText);
                const similarity = this.cosineSimilarity(queryEmbedding, taskEmbedding);

                if (similarity >= (options?.useEmbeddings ? 0.7 : 0.8)) {
                    results.push({
                        task,
                        score: similarity,
                        matches: ['semantic']
                    });
                }
            }

            return this.sortAndLimit(results, options);
        } catch (error) {
            console.warn('Semantic search failed, falling back to text search:', error);
            return this.textSearch(query, tasks, options);
        }
    }

    private async semanticSearchMemories(query: string, memories: Memory[], options?: SearchOptions): Promise<MemorySearchResult[]> {
        if (!this.embeddingService) {
            return this.textSearchMemories(query, memories, options);
        }

        try {
            const queryEmbedding = await this.embeddingService.generateEmbedding(query);
            const results: MemorySearchResult[] = [];

            for (const memory of memories) {
                const memoryText = `${memory.moment} ${memory.meaning} ${memory.reason}`;
                const memoryEmbedding = await this.embeddingService.generateEmbedding(memoryText);
                const similarity = this.cosineSimilarity(queryEmbedding, memoryEmbedding);

                if (similarity >= 0.7) {
                    results.push({
                        memory,
                        score: similarity,
                        matches: ['semantic']
                    });
                }
            }

            return this.sortAndLimit(results, options);
        } catch (error) {
            console.warn('Semantic search failed, falling back to text search:', error);
            return this.textSearchMemories(query, memories, options);
        }
    }

    private async semanticSearchIdeas(query: string, ideas: Idea[], options?: SearchOptions): Promise<IdeaSearchResult[]> {
        if (!this.embeddingService) {
            return this.textSearchIdeas(query, ideas, options);
        }

        try {
            const queryEmbedding = await this.embeddingService.generateEmbedding(query);
            const results: IdeaSearchResult[] = [];

            for (const idea of ideas) {
                const ideaText = `${idea.idea} ${idea.context || ''}`;
                const ideaEmbedding = await this.embeddingService.generateEmbedding(ideaText);
                const similarity = this.cosineSimilarity(queryEmbedding, ideaEmbedding);

                if (similarity >= 0.7) {
                    results.push({
                        idea,
                        score: similarity,
                        matches: ['semantic']
                    });
                }
            }

            return this.sortAndLimit(results, options);
        } catch (error) {
            console.warn('Semantic search failed, falling back to text search:', error);
            return this.textSearchIdeas(query, ideas, options);
        }
    }

    // Helper methods
    private findMatches(query: string, task: Task, options?: SearchOptions): string[] {
        const matches: string[] = [];
        const fields = [
            { name: 'action', value: task.action },
            { name: 'context', value: task.contextNotes },
            { name: 'project', value: task.parentProject }
        ];

        for (const field of fields) {
            if (field.value && this.containsMatch(field.value, query, options)) {
                matches.push(field.name);
            }
        }

        // Check tags
        if (task.tags.some(tag => this.containsMatch(tag, query, options))) {
            matches.push('tags');
        }

        return matches;
    }

    private findMatchesInMemory(query: string, memory: Memory, options?: SearchOptions): string[] {
        const matches: string[] = [];
        const fields = [
            { name: 'moment', value: memory.moment },
            { name: 'meaning', value: memory.meaning },
            { name: 'reason', value: memory.reason }
        ];

        for (const field of fields) {
            if (field.value && this.containsMatch(field.value, query, options)) {
                matches.push(field.name);
            }
        }

        return matches;
    }

    private findMatchesInIdea(query: string, idea: Idea, options?: SearchOptions): string[] {
        const matches: string[] = [];
        const fields = [
            { name: 'idea', value: idea.idea },
            { name: 'context', value: idea.context }
        ];

        for (const field of fields) {
            if (field.value && this.containsMatch(field.value, query, options)) {
                matches.push(field.name);
            }
        }

        return matches;
    }

    private findMatchesInError(query: string, error: TodoziErrorRecord, options?: SearchOptions): string[] {
        const matches: string[] = [];
        const fields = [
            { name: 'title', value: error.title },
            { name: 'description', value: error.description },
            { name: 'source', value: error.source },
            { name: 'context', value: error.context }
        ];

        for (const field of fields) {
            if (field.value && this.containsMatch(field.value, query, options)) {
                matches.push(field.name);
            }
        }

        return matches;
    }

    private containsMatch(text: string, query: string, options?: SearchOptions): boolean {
        const searchText = options?.caseSensitive ? text : text.toLowerCase();
        const searchQuery = options?.caseSensitive ? query : query.toLowerCase();

        if (options?.exactMatch) {
            return searchText === searchQuery;
        } else if (options?.fuzzy) {
            return this.fuzzyMatch(searchText, searchQuery);
        } else {
            return searchText.includes(searchQuery);
        }
    }

    private fuzzyMatch(text: string, query: string): boolean {
        // Simple fuzzy matching - check if all characters of query appear in order
        let queryIndex = 0;
        for (let i = 0; i < text.length && queryIndex < query.length; i++) {
            if (text[i] === query[queryIndex]) {
                queryIndex++;
            }
        }
        return queryIndex === query.length;
    }

    private calculateRelevanceScore(query: string, task: Task, matches: string[]): number {
        let score = 0;

        // Base score from matches
        score += matches.length * 10;

        // Bonus for exact matches in action
        if (matches.includes('action') && task.action.toLowerCase().includes(query)) {
            score += 20;
        }

        // Priority bonus
        const priorityBonus = { 'urgent': 15, 'critical': 12, 'high': 8, 'medium': 5, 'low': 2 };
        score += priorityBonus[task.priority.toLowerCase()] || 0;

        return Math.min(score, 100);
    }

    private calculateMemoryRelevanceScore(query: string, memory: Memory, matches: string[]): number {
        let score = matches.length * 10;

        // Importance bonus
        const importanceBonus = { 'critical': 15, 'high': 10, 'medium': 5, 'low': 2 };
        score += importanceBonus[memory.importance.toLowerCase()] || 0;

        return Math.min(score, 100);
    }

    private calculateIdeaRelevanceScore(query: string, idea: Idea, matches: string[]): number {
        let score = matches.length * 10;

        // Importance bonus
        const importanceBonus = { 'breakthrough': 20, 'high': 10, 'medium': 5, 'low': 2 };
        score += importanceBonus[idea.importance.toLowerCase()] || 0;

        return Math.min(score, 100);
    }

    private calculateErrorRelevanceScore(query: string, error: TodoziErrorRecord, matches: string[]): number {
        let score = matches.length * 10;

        // Severity bonus
        const severityBonus = { 'critical': 20, 'high': 15, 'medium': 10, 'low': 5 };
        score += severityBonus[error.severity.toLowerCase()] || 0;

        return Math.min(score, 100);
    }

    private cosineSimilarity(a: number[], b: number[]): number {
        if (a.length !== b.length) return 0;

        let dotProduct = 0;
        let normA = 0;
        let normB = 0;

        for (let i = 0; i < a.length; i++) {
            dotProduct += a[i] * b[i];
            normA += a[i] * a[i];
            normB += b[i] * b[i];
        }

        if (normA === 0 || normB === 0) return 0;

        return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
    }

    private sortAndLimit<T extends { score: number }>(results: T[], options?: SearchOptions): T[] {
        const sortBy = options?.sortBy || 'relevance';
        const sortOrder = options?.sortOrder || 'desc';

        results.sort((a, b) => {
            let comparison = 0;

            switch (sortBy) {
                case 'relevance':
                    comparison = b.score - a.score;
                    break;
                case 'date':
                    // This would need date field access
                    comparison = 0;
                    break;
                case 'priority':
                    // This would need priority field access
                    comparison = 0;
                    break;
            }

            return sortOrder === 'asc' ? -comparison : comparison;
        });

        const limit = options?.limit || 50;
        return results.slice(0, limit);
    }

    // Filter application methods
    private applyFilters(tasks: Task[], filters?: SearchFilters): Task[] {
        if (!filters) return tasks;

        return tasks.filter(task => {
            if (filters.project && task.parentProject !== filters.project) return false;
            if (filters.status && task.status !== filters.status) return false;
            if (filters.priority && task.priority !== filters.priority) return false;
            if (filters.assignee && task.assignee?.type !== filters.assignee) return false;
            if (filters.tags && !filters.tags.some(tag => task.tags.includes(tag))) return false;
            if (filters.dateFrom && task.createdAt < filters.dateFrom) return false;
            if (filters.dateTo && task.createdAt > filters.dateTo) return false;

            return true;
        });
    }

    private applyMemoryFilters(memories: Memory[], filters?: SearchFilters): Memory[] {
        if (!filters) return memories;

        return memories.filter(memory => {
            if (filters.project && memory.projectId !== filters.project) return false;
            if (filters.tags && !filters.tags.some(tag => memory.tags.includes(tag))) return false;
            if (filters.dateFrom && memory.createdAt < filters.dateFrom) return false;
            if (filters.dateTo && memory.createdAt > filters.dateTo) return false;

            return true;
        });
    }

    private applyIdeaFilters(ideas: Idea[], filters?: SearchFilters): Idea[] {
        if (!filters) return ideas;

        return ideas.filter(idea => {
            if (filters.project && idea.projectId !== filters.project) return false;
            if (filters.tags && !filters.tags.some(tag => idea.tags.includes(tag))) return false;
            if (filters.dateFrom && idea.createdAt < filters.dateFrom) return false;
            if (filters.dateTo && idea.createdAt > filters.dateTo) return false;

            return true;
        });
    }

    private applyErrorFilters(errors: TodoziErrorRecord[], filters?: SearchFilters): TodoziErrorRecord[] {
        if (!filters) return errors;

        return errors.filter(error => {
            if (filters.tags && !filters.tags.some(tag => error.tags.includes(tag))) return false;
            if (filters.dateFrom && error.createdAt < filters.dateFrom) return false;
            if (filters.dateTo && error.createdAt > filters.dateTo) return false;

            return true;
        });
    }
}

export default SearchEngine;
