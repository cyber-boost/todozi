import axios, { AxiosInstance, AxiosResponse } from 'axios';
import { Task, Memory, Idea, TrainingData } from './models.js';
import { TodoziErrorRecord } from './error.js';
import { TodoziError } from './error.js';

// API Response types
export interface ApiResponse<T> {
    success: boolean;
    data?: T;
    error?: string;
    message?: string;
}

export interface PaginatedResponse<T> {
    items: T[];
    total: number;
    page: number;
    pageSize: number;
    hasMore: boolean;
}

// API Client configuration
export interface ApiConfig {
    baseUrl: string;
    apiKey?: string;
    timeout: number;
    retryAttempts: number;
    retryDelay: number;
}

// API Client class
export class TodoziApiClient {
    private client: AxiosInstance;
    private config: ApiConfig;

    constructor(config: ApiConfig) {
        this.config = config;
        this.client = axios.create({
            baseURL: config.baseUrl,
            timeout: config.timeout,
            headers: {
                'Content-Type': 'application/json',
                ...(config.apiKey && { 'Authorization': `Bearer ${config.apiKey}` })
            }
        });

        this.setupInterceptors();
    }

    private setupInterceptors(): void {
        // Request interceptor for retry logic
        this.client.interceptors.request.use(
            (config) => config,
            (error) => Promise.reject(error)
        );

        // Response interceptor for error handling
        this.client.interceptors.response.use(
            (response) => response,
            async (error) => {
                if (error.code === 'ECONNABORTED' || error.response?.status >= 500) {
                    return this.retryRequest(error.config);
                }
                return Promise.reject(error);
            }
        );
    }

    private async retryRequest(config: any, attempt: number = 1): Promise<AxiosResponse> {
        if (attempt > this.config.retryAttempts) {
            throw TodoziError.api(`Request failed after ${this.config.retryAttempts} attempts`);
        }

        await new Promise(resolve => setTimeout(resolve, this.config.retryDelay * attempt));

        try {
            return await this.client.request(config);
        } catch (error) {
            return this.retryRequest(config, attempt + 1);
        }
    }

    // Task API methods
    async getTasks(filters?: {
        project?: string;
        status?: string;
        priority?: string;
        assignee?: string;
        tags?: string[];
        limit?: number;
        offset?: number;
    }): Promise<PaginatedResponse<Task>> {
        try {
            const response = await this.client.get('/api/tasks', { params: filters });
            return this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getTask(id: string): Promise<Task> {
        try {
            const response = await this.client.get(`/api/tasks/${id}`);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createTask(task: Omit<Task, 'id' | 'createdAt' | 'updatedAt'>): Promise<Task> {
        try {
            const response = await this.client.post('/api/tasks', task);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async updateTask(id: string, updates: Partial<Task>): Promise<Task> {
        try {
            const response = await this.client.put(`/api/tasks/${id}`, updates);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async deleteTask(id: string): Promise<void> {
        try {
            const response = await this.client.delete(`/api/tasks/${id}`);
            this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async searchTasks(query: string, options?: {
        semantic?: boolean;
        limit?: number;
        filters?: any;
    }): Promise<Task[]> {
        try {
            const response = await this.client.get('/api/search/tasks', {
                params: { q: query, ...options }
            });
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Memory API methods
    async getMemories(filters?: {
        project?: string;
        importance?: string;
        term?: string;
        limit?: number;
        offset?: number;
    }): Promise<PaginatedResponse<Memory>> {
        try {
            const response = await this.client.get('/api/memories', { params: filters });
            return this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getMemory(id: string): Promise<Memory> {
        try {
            const response = await this.client.get(`/api/memories/${id}`);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createMemory(memory: Omit<Memory, 'id' | 'createdAt' | 'updatedAt'>): Promise<Memory> {
        try {
            const response = await this.client.post('/api/memories', memory);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async updateMemory(id: string, updates: Partial<Memory>): Promise<Memory> {
        try {
            const response = await this.client.put(`/api/memories/${id}`, updates);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async deleteMemory(id: string): Promise<void> {
        try {
            const response = await this.client.delete(`/api/memories/${id}`);
            this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Idea API methods
    async getIdeas(filters?: {
        project?: string;
        importance?: string;
        share?: string;
        limit?: number;
        offset?: number;
    }): Promise<PaginatedResponse<Idea>> {
        try {
            const response = await this.client.get('/api/ideas', { params: filters });
            return this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getIdea(id: string): Promise<Idea> {
        try {
            const response = await this.client.get(`/api/ideas/${id}`);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createIdea(idea: Omit<Idea, 'id' | 'createdAt' | 'updatedAt'>): Promise<Idea> {
        try {
            const response = await this.client.post('/api/ideas', idea);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async updateIdea(id: string, updates: Partial<Idea>): Promise<Idea> {
        try {
            const response = await this.client.put(`/api/ideas/${id}`, updates);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async deleteIdea(id: string): Promise<void> {
        try {
            const response = await this.client.delete(`/api/ideas/${id}`);
            this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Error API methods
    async getErrors(filters?: {
        severity?: string;
        category?: string;
        resolved?: boolean;
        limit?: number;
        offset?: number;
    }): Promise<PaginatedResponse<TodoziErrorRecord>> {
        try {
            const response = await this.client.get('/api/errors', { params: filters });
            return this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getError(id: string): Promise<TodoziErrorRecord> {
        try {
            const response = await this.client.get(`/api/errors/${id}`);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createError(error: Omit<TodoziErrorRecord, 'id' | 'createdAt' | 'updatedAt'>): Promise<TodoziErrorRecord> {
        try {
            const response = await this.client.post('/api/errors', error);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async resolveError(id: string, resolution: string): Promise<TodoziErrorRecord> {
        try {
            const response = await this.client.post(`/api/errors/${id}/resolve`, { resolution });
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Training Data API methods
    async getTrainingData(filters?: {
        dataType?: string;
        source?: string;
        minQuality?: number;
        limit?: number;
        offset?: number;
    }): Promise<PaginatedResponse<TrainingData>> {
        try {
            const response = await this.client.get('/api/training', { params: filters });
            return this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createTrainingData(data: Omit<TrainingData, 'id' | 'createdAt' | 'updatedAt'>): Promise<TrainingData> {
        try {
            const response = await this.client.post('/api/training', data);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async updateTrainingData(id: string, updates: Partial<TrainingData>): Promise<TrainingData> {
        try {
            const response = await this.client.put(`/api/training/${id}`, updates);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async deleteTrainingData(id: string): Promise<void> {
        try {
            const response = await this.client.delete(`/api/training/${id}`);
            this.handleResponse(response);
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Unified search
    async unifiedSearch(query: string, options?: {
        types?: string[];
        semantic?: boolean;
        limit?: number;
    }): Promise<{
        tasks: Task[];
        memories: Memory[];
        ideas: Idea[];
        errors: TodoziErrorRecord[];
        totalCount: number;
    }> {
        try {
            const response = await this.client.get('/api/search/unified', {
                params: { q: query, ...options }
            });
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Analytics and insights
    async getAnalytics(timeRange?: {
        from: Date;
        to: Date;
    }): Promise<{
        taskStats: any;
        memoryStats: any;
        ideaStats: any;
        errorStats: any;
    }> {
        try {
            const response = await this.client.get('/api/analytics', {
                params: timeRange ? {
                    from: timeRange.from.toISOString(),
                    to: timeRange.to.toISOString()
                } : undefined
            });
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Queue management
    async getQueueItems(status?: string): Promise<any[]> {
        try {
            const response = await this.client.get('/api/queue', { params: { status } });
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createQueueItem(item: {
        taskName: string;
        taskDescription: string;
        priority: string;
        projectId?: string;
    }): Promise<any> {
        try {
            const response = await this.client.post('/api/queue', item);
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // System methods
    async healthCheck(): Promise<{ status: string; version: string; uptime: number }> {
        try {
            const response = await this.client.get('/api/health');
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getSystemStats(): Promise<any> {
        try {
            const response = await this.client.get('/api/system/stats');
            return this.handleResponse(response).data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Utility methods
    private handleResponse<T>(response: AxiosResponse): T {
        if (response.data.success === false) {
            throw TodoziError.api(response.data.error || response.data.message || 'API request failed');
        }
        return response.data;
    }

    private handleError(error: any): TodoziError {
        if (error.response) {
            // Server responded with error status
            const status = error.response.status;
            const message = error.response.data?.error || error.response.data?.message || error.message;

            switch (status) {
                case 400:
                    return TodoziError.validation(message);
                case 401:
                    return TodoziError.api('Authentication failed');
                case 403:
                    return TodoziError.api('Access forbidden');
                case 404:
                    return TodoziError.api('Resource not found');
                case 429:
                    return TodoziError.api('Rate limit exceeded');
                case 500:
                    return TodoziError.api('Internal server error');
                default:
                    return TodoziError.api(`${status}: ${message}`);
            }
        } else if (error.request) {
            // Network error
            return TodoziError.api('Network error - unable to reach server');
        } else {
            // Other error
            return TodoziError.api(error.message || 'Unknown API error');
        }
    }

    // Configuration methods
    updateApiKey(apiKey: string): void {
        this.config.apiKey = apiKey;
        this.client.defaults.headers.Authorization = `Bearer ${apiKey}`;
    }

    updateBaseUrl(baseUrl: string): void {
        this.config.baseUrl = baseUrl;
        this.client.defaults.baseURL = baseUrl;
    }

    getConfig(): ApiConfig {
        return { ...this.config };
    }
}

// Factory function
export function createApiClient(config: Partial<ApiConfig> = {}): TodoziApiClient {
    const defaultConfig: ApiConfig = {
        baseUrl: 'http://localhost:3000',
        timeout: 30000,
        retryAttempts: 3,
        retryDelay: 1000,
        ...config
    };

    return new TodoziApiClient(defaultConfig);
}

export default TodoziApiClient;
