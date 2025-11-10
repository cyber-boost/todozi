import axios from 'axios';
import { generateUUID } from './models.js';
import * as chrono from 'chrono-node';

// ==================== TYPES AND INTERFACES ====================

interface ValidationResult {
  isValid: boolean;
  errors: string[];
}

interface ExecuteParams {
  [key: string]: any;
}

interface ResourceLock {
  type: 'FilesystemRead' | 'FilesystemWrite' | 'Network' | 'Memory';
}

interface ToolParameter {
  name: string;
  type: string;
  description: string;
  required: boolean;
}

interface ToolDefinition {
  name: string;
  description: string;
  parameters: ToolParameter[];
  category: string;
  resourceLocks: ResourceLock[];
}

interface ToolResult {
  success: boolean;
  message: string;
  code: number;
}

interface TaskFilters {
  status?: Status;
  priority?: Priority;
  project?: string;
  assignee?: Assignee;
  tags?: string[];
}

interface TaskUpdate {
  action?: string | null;
  time?: string | null;
  priority?: Priority | null;
  parent_project?: string | null;
  status?: Status | null;
  assignee?: Assignee | null;
  tags?: string[] | null;
  dependencies?: string[] | null;
  context_notes?: string | null;
  progress?: number | null;
  embedding_vector?: number[] | null;
}

interface Task {
  id: string;
  user_id: string;
  action: string;
  time: string;
  priority: Priority;
  parent_project: string;
  status: Status;
  assignee?: Assignee;
  tags: string[];
  dependencies: string[];
  context_notes?: string;
  progress?: number;
  embedding_vector?: number[];
  created_at: Date;
  updated_at: Date;
}

interface Memory {
  id: string;
  user_id: string;
  project_id?: string;
  status: ItemStatus;
  moment: string;
  meaning: string;
  reason: string;
  importance: MemoryImportance;
  term: MemoryTerm;
  memory_type: MemoryType;
  tags: string[];
  created_at: Date;
  updated_at: Date;
}

interface Idea {
  id: string;
  user_id: string;
  project_id?: string;
  status: ItemStatus;
  idea: string;
  context?: string;
  importance: IdeaImportance;
  share: ShareLevel;
  tags: string[];
  created_at: Date;
  updated_at: Date;
}

interface Error {
  id: string;
  title: string;
  description: string;
  severity: ErrorSeverity;
  category: ErrorCategory;
  source: string;
  context?: string;
  tags: string[];
  resolved: boolean;
  resolution?: string;
  created_at: Date;
  updated_at: Date;
  resolved_at?: Date;
}

interface CodeChunk {
  chunk_id: string;
  status: ChunkStatus;
  dependencies: string[];
  code: string;
  tests: string;
  validated: boolean;
  level: ChunkingLevel;
  estimated_tokens: number;
  created_at: Date;
  updated_at: Date;
}

interface ChatContent {
  tasks: Task[];
  memories: Memory[];
  ideas: Idea[];
  errors: Error[];
  feelings: string[];
}

interface SearchResult {
  content_id: string;
  text_content: string;
  similarity_score: number;
}

interface PredictedError {
  error_type: string;
  probability: number;
  description: string;
  severity: ErrorSeverity;
}

interface QualityScore {
  score: number;
  feedback: string;
}

interface QualityMetrics {
  max_complexity: number;
  min_test_coverage: number;
  max_duplication: number;
  required_patterns: string[];
}

interface LearningData {
  tasks: Task[];
  memories: Memory[];
  ideas: Idea[];
  errors: Error[];
}

interface LearningInsights {
  patterns: string[];
  predictions: string[];
  recommendations: string[];
  metrics: Map<string, number>;
}

interface AgentMetrics {
  success_rate: number;
  average_completion_time: number;
  specialization_score: number;
  collaboration_rating: number;
}

interface TodoziConfig {
  baseUrl: string;
  timeout: number;
  retryAttempts: number;
  apiKey?: string;
}

interface ToolFactoryConfig {
  enableEmbeddings: boolean;
  defaultTimeout: number;
  maxRetries: number;
}

interface ToolOptions {
  includeAdvanced: boolean;
  enableAnalytics: boolean;
}

interface TodoziEmbeddingConfig {
  model_name: string;
  max_results: number;
  similarity_threshold: number;
  cache_ttl_seconds: number;
  clustering_threshold: number;
  dimensions: number;
  enable_clustering: boolean;
}

interface UnifiedSearchResults {
  tasks: Task[];
  memories: Memory[];
  ideas: Idea[];
  errors: Error[];
  total: number;
}

interface QueueItem {
  id: string;
  type: 'task' | 'memory' | 'idea' | 'error';
  priority: Priority;
  content: string;
  created_at: Date;
}

// ==================== ENUMS ====================

enum Priority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical',
  Urgent = 'urgent'
}

enum Status {
  Todo = 'todo',
  InProgress = 'in_progress',
  Blocked = 'blocked',
  Review = 'review',
  Done = 'done'
}

enum Assignee {
  AI = 'ai',
  Human = 'human',
  Collaborative = 'collaborative'
}

enum ItemStatus {
  Active = 'active',
  Archived = 'archived',
  Deleted = 'deleted'
}

enum MemoryImportance {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

enum MemoryTerm {
  Short = 'short',
  Long = 'long'
}

enum MemoryType {
  Standard = 'standard',
  Secret = 'secret',
  Human = 'human',
  Short = 'short',
  Long = 'long',
  Emotional = 'emotional'
}

enum IdeaImportance {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Breakthrough = 'breakthrough'
}

enum ShareLevel {
  Private = 'private',
  Team = 'team',
  Public = 'public'
}

enum ErrorSeverity {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

enum ErrorCategory {
  Runtime = 'runtime',
  Validation = 'validation',
  Database = 'database',
  Network = 'network',
  Integration = 'integration',
  Performance = 'performance'
}

enum ChunkingLevel {
  Project = 'project',
  Module = 'module',
  Class = 'class',
  Method = 'method',
  Block = 'block'
}

enum ChunkStatus {
  Pending = 'pending',
  InProgress = 'in_progress',
  Completed = 'completed',
  Validated = 'validated'
}

// ==================== CUSTOM ERROR CLASSES ====================

class TodoziError extends Error {
  constructor(
    public code: string,
    message: string,
    public details?: any
  ) {
    super(message);
    this.name = 'TodoziError';
  }
}

class ValidationError extends TodoziError {
  constructor(message: string, details?: any) {
    super('VALIDATION_ERROR', message, details);
  }
}

class ApiError extends TodoziError {
  constructor(
    public status: number,
    message: string,
    details?: any
  ) {
    super('API_ERROR', message, details);
  }
}

class StorageError extends TodoziError {
  constructor(message: string, details?: any) {
    super('STORAGE_ERROR', message, details);
  }
}

// ==================== CONFIGURATION ====================

const DEFAULT_TODOZI_CONFIG: TodoziConfig = {
  baseUrl: 'https://todozi.com',
  timeout: 30000,
  retryAttempts: 3
};

// ==================== RESOURCE MANAGEMENT ====================

class ResourceManager {
  private static locks = new Map<string, Promise<void>>();

  static async acquireLock(type: ResourceLock): Promise<void> {
    const lockKey = `${type.type}`;
    
    if (this.locks.has(lockKey)) {
      throw new TodoziError('RESOURCE_LOCKED', `Resource ${type.type} is already locked`);
    }

    const lockPromise = new Promise<void>((resolve) => {
      setTimeout(resolve, 100); // Simple lock acquisition
    });

    this.locks.set(lockKey, lockPromise);
    await lockPromise;
  }

  static releaseLock(type: ResourceLock): void {
    const lockKey = `${type.type}`;
    this.locks.delete(lockKey);
  }
}

// ==================== PARAMETER VALIDATION ====================

class ParameterValidator {
  static validateString(value: any, name: string, required: boolean, minLength = 1, maxLength = 1000): ValidationResult {
    const errors: string[] = [];
    
    if (value === undefined || value === null) {
      if (required) {
        errors.push(`Missing required parameter: ${name}`);
      }
    } else if (typeof value !== 'string') {
      errors.push(`${name} must be a string`);
    } else {
      const trimmed = value.trim();
      if (required && trimmed.length === 0) {
        errors.push(`${name} cannot be empty`);
      }
      if (trimmed.length < minLength || trimmed.length > maxLength) {
        errors.push(`${name} must be between ${minLength} and ${maxLength} characters`);
      }
    }

    return { isValid: errors.length === 0, errors };
  }

  static validateTaskParams(params: ExecuteParams): ValidationResult {
    const errors: string[] = [];
    
    const action = this.validateString(params.action, 'action', true, 1, 500);
    if (!action.isValid) {
      errors.push(...action.errors);
    }

    if (params.priority && !Object.values(Priority).includes(params.priority)) {
      errors.push('Invalid priority value');
    }

    if (params.assignee && !Object.values(Assignee).includes(params.assignee)) {
      errors.push('Invalid assignee value');
    }

    return { isValid: errors.length === 0, errors };
  }
}

// ==================== STORAGE LAYER ====================

interface StorageBackend {
  save(key: string, value: any): Promise<void>;
  load(key: string): Promise<any>;
  list(prefix: string): Promise<string[]>;
  delete(key: string): Promise<void>;
}

class FileStorageBackend implements StorageBackend {
  private basePath: string;

  constructor(basePath = './todozi_data') {
    this.basePath = basePath;
    // In a real implementation, use fs-extra or similar
  }

  async save(key: string, value: any): Promise<void> {
    // Implementation would save to file system
    console.log(`Saving ${key} to file system`);
  }

  async load(key: string): Promise<any> {
    // Implementation would load from file system
    console.log(`Loading ${key} from file system`);
    return null;
  }

  async list(prefix: string): Promise<string[]> {
    // Implementation would list files
    console.log(`Listing files with prefix ${prefix}`);
    return [];
  }

  async delete(key: string): Promise<void> {
    // Implementation would delete file
    console.log(`Deleting ${key} from file system`);
  }
}

class Storage {
  private tasks = new Map<string, Task>();
  private memories = new Map<string, Memory>();
  private ideas = new Map<string, Idea>();
  private errors = new Map<string, Error>();
  private codeChunks = new Map<string, CodeChunk>();
  private backend: StorageBackend;

  constructor(backend?: StorageBackend) {
    this.backend = backend || new FileStorageBackend();
  }

  async saveTask(task: Task): Promise<void> {
    this.tasks.set(task.id, task);
    await this.backend.save(`task_${task.id}`, task);
  }

  async loadTask(id: string): Promise<Task | null> {
    const cached = this.tasks.get(id);
    if (cached) return cached;
    
    const loaded = await this.backend.load(`task_${id}`);
    if (loaded) {
      this.tasks.set(id, loaded);
      return loaded;
    }
    return null;
  }

  async listTasks(filters?: TaskFilters): Promise<Task[]> {
    let tasks = Array.from(this.tasks.values());
    
    if (filters) {
      tasks = tasks.filter(task => {
        if (filters.status && task.status !== filters.status) return false;
        if (filters.priority && task.priority !== filters.priority) return false;
        if (filters.project && task.parent_project !== filters.project) return false;
        if (filters.assignee && task.assignee !== filters.assignee) return false;
        if (filters.tags && !filters.tags.some(tag => task.tags.includes(tag))) return false;
        return true;
      });
    }

    return tasks;
  }

  async saveMemory(memory: Memory): Promise<void> {
    this.memories.set(memory.id, memory);
    await this.backend.save(`memory_${memory.id}`, memory);
  }

  async saveIdea(idea: Idea): Promise<void> {
    this.ideas.set(idea.id, idea);
    await this.backend.save(`idea_${idea.id}`, idea);
  }

  async saveError(error: Error): Promise<void> {
    this.errors.set(error.id, error);
    await this.backend.save(`error_${error.id}`, error);
  }

  async saveCodeChunk(chunk: CodeChunk): Promise<void> {
    this.codeChunks.set(chunk.chunk_id, chunk);
    await this.backend.save(`chunk_${chunk.chunk_id}`, chunk);
  }

  async addTaskToProject(task: Task): Promise<void> {
    await this.saveTask(task);
  }

  async listTasksAcrossProjects(filters?: TaskFilters): Promise<Task[]> {
    return this.listTasks(filters);
  }

  async updateTask(id: string, updates: Partial<Task>): Promise<void> {
    const task = await this.loadTask(id);
    if (!task) {
      throw new StorageError(`Task ${id} not found`);
    }

    Object.assign(task, updates, { updated_at: new Date() });
    await this.saveTask(task);
  }
}

// ==================== API CLIENT ====================

class TodoziClient {
  constructor(private config: TodoziConfig) {}

  async makeRequest(endpoint: string, payload: any): Promise<any> {
    const url = `${this.config.baseUrl}${endpoint}`;
    
    for (let attempt = 1; attempt <= this.config.retryAttempts; attempt++) {
      try {
        const response = await axios.post(url, payload, {
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${this.config.apiKey || await this.getApiKey()}`
          },
          timeout: this.config.timeout
        });

        if (response.status >= 200 && response.status < 300) {
          return response.data;
        } else {
          throw new ApiError(response.status, `API request failed with status: ${response.status}`);
        }
      } catch (error) {
        if (attempt === this.config.retryAttempts) {
          if (axios.isAxiosError(error)) {
            throw new ApiError(
              error.response?.status || 0,
              `Request failed: ${error.message}`,
              error.response?.data
            );
          }
          throw new TodoziError('REQUEST_FAILED', `Request failed: ${error}`);
        }
        await new Promise(resolve => setTimeout(resolve, 1000 * attempt));
      }
    }

    throw new TodoziError('MAX_RETRIES_EXCEEDED', 'Maximum retry attempts exceeded');
  }

  private async getApiKey(): Promise<string> {
    // Implementation would fetch API key from secure storage
    return 'demo-api-key';
  }
}

// ==================== DONE API ABSTRACTION ====================

class Done {
  private static client = new TodoziClient(DEFAULT_TODOZI_CONFIG);

  static async task(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/task', { action });
    return result.id;
  }

  static async urgent(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/urgent', { action });
    return result.id;
  }

  static async high(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/high', { action });
    return result.id;
  }

  static async low(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/low', { action });
    return result.id;
  }

  static async ai(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/ai', { action });
    return result.id;
  }

  static async human(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/human', { action });
    return result.id;
  }

  static async collab(action: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/collab', { action });
    return result.id;
  }

  static async find(query: string): Promise<any[]> {
    const result = await this.client.makeRequest('/api/todozi/find', { query });
    return result.results || [];
  }

  static async deep(query: string): Promise<SearchResult[]> {
    const result = await this.client.makeRequest('/api/todozi/deep', { query });
    return result.results || [];
  }

  static async fast(query: string): Promise<any[]> {
    const result = await this.client.makeRequest('/api/todozi/fast', { query });
    return result.results || [];
  }

  static async smart(query: string): Promise<any[]> {
    const result = await this.client.makeRequest('/api/todozi/smart', { query });
    return result.results || [];
  }

  static async tdz_find(query: string): Promise<UnifiedSearchResults> {
    const result = await this.client.makeRequest('/api/todozi/tdz_find', { query });
    return result;
  }

  static async create_memory(moment: string, meaning: string, reason: string): Promise<{ id: string }> {
    const result = await this.client.makeRequest('/api/todozi/memory', { moment, meaning, reason });
    return result;
  }

  static async important(moment: string, meaning: string, reason: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/important', { moment, meaning, reason });
    return result.id;
  }

  static async create_idea(idea: string, context?: string): Promise<{ id: string }> {
    const result = await this.client.makeRequest('/api/todozi/idea', { idea, context });
    return result;
  }

  static async breakthrough(idea: string): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/breakthrough', { idea });
    return result.id;
  }

  static async complete(taskId: string): Promise<void> {
    await this.client.makeRequest('/api/todozi/complete', { task_id: taskId });
  }

  static async begin(taskId: string): Promise<void> {
    await this.client.makeRequest('/api/todozi/begin', { task_id: taskId });
  }

  static async quick(): Promise<string> {
    const result = await this.client.makeRequest('/api/todozi/quick', {});
    return result.stats || 'No stats available';
  }

  static async list_queue_items(): Promise<QueueItem[]> {
    const result = await this.client.makeRequest('/api/todozi/queue', {});
    return result.items || [];
  }

  static async chat(message: string): Promise<ChatContent> {
    const result = await this.client.makeRequest('/api/todozi/chat', { message });
    return result.content || { tasks: [], memories: [], ideas: [], errors: [], feelings: [] };
  }

  static async extract_tasks(content: string, context?: string): Promise<string[]> {
    const result = await this.client.makeRequest('/api/todozi/extract', { content, context });
    return result.tasks || [];
  }

  static async plan_tasks(
    goal: string,
    priority?: string,
    time?: string,
    context?: string
  ): Promise<Task[]> {
    const result = await this.client.makeRequest('/api/todozi/plan', {
      goal,
      priority,
      time,
      context
    });
    return result.tasks || [];
  }

  static async update_task_full(taskId: string, updates: TaskUpdate): Promise<void> {
    await this.client.makeRequest('/api/todozi/update', { task_id: taskId, ...updates });
  }

  static async add_to_task(taskId: string, tag: string): Promise<void> {
    await this.client.makeRequest('/api/todozi/add_tag', { task_id: taskId, tag });
  }

  static async create_task(
    action: string,
    priority?: Priority,
    project?: string,
    time?: string,
    context?: string
  ): Promise<Task> {
    const result = await this.client.makeRequest('/api/todozi/create_task', {
      action,
      priority,
      project,
      time,
      context
    });
    return result.task;
  }
}

// ==================== TOOL BASE CLASS ====================

abstract class Tool {
  abstract definition(): ToolDefinition;
  abstract execute(kwargs: ExecuteParams): Promise<ToolResult>;

  protected createToolParameter(
    name: string,
    type: string,
    description: string,
    required: boolean
  ): ToolParameter {
    return { name, type, description, required };
  }

  protected async success(message: string, code: number): Promise<ToolResult> {
    return { success: true, message, code };
  }

  protected error(message: string, code: number): Promise<ToolResult> {
    return Promise.resolve({ success: false, message, code });
  }
}

// ==================== EMBEDDING SERVICE ====================

class TodoziEmbeddingService {
  constructor(private config: TodoziEmbeddingConfig) {}

  async new(config: TodoziEmbeddingConfig): Promise<TodoziEmbeddingService> {
    return new TodoziEmbeddingService(config);
  }

  async search(query: string): Promise<SearchResult[]> {
    // Implementation would use embedding models
    return [];
  }
}

// ==================== TOOL IMPLEMENTATIONS ====================

class CreateTaskTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'create_task',
      description: 'Create a new task in the Todozi system with automatic AI assignment and queue management',
      parameters: [
        this.createToolParameter('action', 'string', 'Task description/action to perform', true),
        this.createToolParameter('time', 'string', 'Time estimate (e.g., \'2 hours\', \'1 day\')', false),
        this.createToolParameter('priority', 'string', 'Priority level (low/medium/high/critical/urgent)', false),
        this.createToolParameter('project', 'string', 'Project name to associate with task', false),
        this.createToolParameter('assignee', 'string', 'Assignee type (ai/human/collaborative)', false),
        this.createToolParameter('tags', 'string', 'Comma-separated tags for the task', false),
        this.createToolParameter('context', 'string', 'Additional context or notes', false)
      ],
      category: 'Task Management',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const validation = ParameterValidator.validateTaskParams(kwargs);
    if (!validation.isValid) {
      return this.error(validation.errors.join('; '), 100);
    }

    const action = kwargs.action as string;
    const assignee = kwargs.assignee as string || 'human';
    const priority = kwargs.priority as string || 'medium';
    const context = kwargs.context as string;
    const project = kwargs.project as string;

    let taskId: string;
    try {
      switch (assignee) {
        case 'ai':
          taskId = await Done.ai(action);
          break;
        case 'human':
          taskId = await Done.human(action);
          break;
        case 'collaborative':
          taskId = await Done.collab(action);
          break;
        case 'urgent':
          taskId = await Done.urgent(action);
          break;
        case 'critical':
          taskId = await Done.urgent(action);
          break;
        case 'high':
          taskId = await Done.high(action);
          break;
        case 'low':
          taskId = await Done.low(action);
          break;
        default:
          const task = await Done.create_task(
            action,
            priority as Priority,
            project,
            kwargs.time as string,
            context
          );
          taskId = task.id;
      }

      // Add tags if provided
      if (kwargs.tags) {
        const tags = (kwargs.tags as string).split(',');
        for (const tag of tags) {
          const trimmed = tag.trim();
          if (trimmed) {
            await Done.add_to_task(taskId, trimmed);
          }
        }
      }

      return this.success(
        `✅ Created task '${action}' with ID: ${taskId} (queued for ${assignee})`,
        100
      );
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 100);
      }
      return this.error(`Failed to create task: ${e}`, 100);
    }
  }
}

class SearchTasksTool extends Tool {
  private embeddingService?: TodoziEmbeddingService;

  constructor(todozi: Storage) {
    super();
  }

  withEmbeddingService(service: TodoziEmbeddingService): SearchTasksTool {
    this.embeddingService = service;
    return this;
  }

  definition(): ToolDefinition {
    return {
      name: 'search_tasks',
      description: 'Search for tasks in the Todozi system with semantic AI capabilities',
      parameters: [
        this.createToolParameter('query', 'string', 'Search query to match against task content', true),
        this.createToolParameter('semantic', 'boolean', 'Use AI semantic search instead of keyword matching', false),
        this.createToolParameter('project', 'string', 'Filter by project name', false),
        this.createToolParameter('status', 'string', 'Filter by status (todo/in_progress/blocked/review/done)', false),
        this.createToolParameter('assignee', 'string', 'Filter by assignee (ai/human/collaborative)', false),
        this.createToolParameter('limit', 'number', 'Maximum number of results to return', false)
      ],
      category: 'Task Management',
      resourceLocks: [{ type: 'FilesystemRead' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const queryValidation = ParameterValidator.validateString(kwargs.query, 'query', true, 1, 100);
    if (!queryValidation.isValid) {
      return this.error(queryValidation.errors.join('; '), 150);
    }

    const query = kwargs.query as string;
    const semantic = kwargs.semantic as boolean || false;

    try {
      if (semantic) {
        const results = await Done.deep(query);
        if (results.length === 0) {
          return this.success(`🤖 No AI semantic results found for: ${query}`, 150);
        }

        const formattedResults = results.map(result =>
          `ID: ${result.content_id} | ${result.text_content} | Similarity: ${result.similarity_score.toFixed(2)} | Type: task`
        ).join('\n');

        return this.success(
          `🤖 AI Semantic Search - Found ${results.length} results:\n${formattedResults}`,
          150
        );
      } else {
        const results = await Done.fast(query);
        if (results.length === 0) {
          return this.success(`🔍 No keyword results found for: ${query}`, 150);
        }

        return this.success(`🔍 Keyword Search Results:\n${JSON.stringify(results, null, 2)}`, 150);
      }
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 150);
      }
      return this.error(`Search failed: ${e}`, 150);
    }
  }
}

class UpdateTaskTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'update_task',
      description: 'Update an existing task in the Todozi system',
      parameters: [
        this.createToolParameter('task_id', 'string', 'ID of the task to update', true),
        this.createToolParameter('status', 'string', 'New status (todo/in_progress/blocked/review/done)', false),
        this.createToolParameter('progress', 'number', 'Progress percentage (0-100)', false),
        this.createToolParameter('priority', 'string', 'New priority level', false),
        this.createToolParameter('assignee', 'string', 'New assignee', false),
        this.createToolParameter('context', 'string', 'Additional context or notes', false)
      ],
      category: 'Task Management',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const taskIdValidation = ParameterValidator.validateString(kwargs.task_id, 'task_id', true, 1, 50);
    if (!taskIdValidation.isValid) {
      return this.error(taskIdValidation.errors.join('; '), 120);
    }

    const taskId = kwargs.task_id as string;

    try {
      if (kwargs.status) {
        const status = kwargs.status as string;
        switch (status.toLowerCase()) {
          case 'completed':
          case 'done':
            await Done.complete(taskId);
            return this.success(`✅ Task ${taskId} marked as completed`, 120);
          case 'in_progress':
          case 'started':
            await Done.begin(taskId);
            return this.success(`🔄 Task ${taskId} marked as in progress`, 120);
        }
      }

      const updates: TaskUpdate = {
        priority: kwargs.priority ? kwargs.priority as Priority : undefined,
        assignee: kwargs.assignee ? kwargs.assignee as Assignee : undefined,
        context_notes: kwargs.context as string,
        progress: kwargs.progress ? Number(kwargs.progress) : undefined
      };

      await Done.update_task_full(taskId, updates);
      return this.success(`✅ Updated task ${taskId}`, 120);
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 120);
      }
      return this.error(`Failed to update task: ${e}`, 120);
    }
  }
}

class CreateMemoryTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'create_memory',
      description: 'Create a new memory for learning and context',
      parameters: [
        this.createToolParameter('moment', 'string', 'What happened (the moment)', true),
        this.createToolParameter('meaning', 'string', 'What it means or why it\'s important', true),
        this.createToolParameter('reason', 'string', 'The reason for remembering this', true),
        this.createToolParameter('importance', 'string', 'Importance level (low/medium/high/critical)', false),
        this.createToolParameter('term', 'string', 'Memory term (short/long)', false),
        this.createToolParameter('tags', 'string', 'Comma-separated tags', false)
      ],
      category: 'Memory Management',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const momentValidation = ParameterValidator.validateString(kwargs.moment, 'moment', true, 1, 1000);
    const meaningValidation = ParameterValidator.validateString(kwargs.meaning, 'meaning', true, 1, 1000);
    const reasonValidation = ParameterValidator.validateString(kwargs.reason, 'reason', true, 1, 1000);

    if (!momentValidation.isValid || !meaningValidation.isValid || !reasonValidation.isValid) {
      return this.error(
        [...momentValidation.errors, ...meaningValidation.errors, ...reasonValidation.errors].join('; '),
        200
      );
    }

    const moment = kwargs.moment as string;
    const meaning = kwargs.meaning as string;
    const reason = kwargs.reason as string;
    const importance = kwargs.importance as string || 'medium';

    try {
      let memoryId: string;
      if (importance.toLowerCase() === 'high' || importance.toLowerCase() === 'critical') {
        memoryId = await Done.important(moment, meaning, reason);
      } else {
        const result = await Done.create_memory(moment, meaning, reason);
        memoryId = result.id;
      }

      return this.success(`🧠 Created memory '${moment}' with ID: ${memoryId}`, 200);
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 200);
      }
      return this.error(`Failed to create memory: ${e}`, 200);
    }
  }
}

class CreateIdeaTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'create_idea',
      description: 'Create a new creative idea or concept',
      parameters: [
        this.createToolParameter('idea', 'string', 'The idea content', true),
        this.createToolParameter('share', 'string', 'Share level (private/team/public)', false),
        this.createToolParameter('importance', 'string', 'Importance level (low/medium/high/breakthrough)', false),
        this.createToolParameter('tags', 'string', 'Comma-separated tags', false),
        this.createToolParameter('context', 'string', 'Additional context', false)
      ],
      category: 'Idea Management',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const ideaValidation = ParameterValidator.validateString(kwargs.idea, 'idea', true, 1, 1000);
    if (!ideaValidation.isValid) {
      return this.error(ideaValidation.errors.join('; '), 180);
    }

    const ideaContent = kwargs.idea as string;
    const importance = kwargs.importance as string || 'medium';

    try {
      let ideaId: string;
      if (importance.toLowerCase() === 'breakthrough' || importance.toLowerCase() === 'high') {
        ideaId = await Done.breakthrough(ideaContent);
      } else {
        const result = await Done.create_idea(ideaContent, kwargs.context as string);
        ideaId = result.id;
      }

      return this.success(`💡 Created idea '${ideaContent}' with ID: ${ideaId}`, 180);
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 180);
      }
      return this.error(`Failed to create idea: ${e}`, 180);
    }
  }
}

class UnifiedSearchTool extends Tool {
  private embeddingService?: TodoziEmbeddingService;

  constructor(private todozi: Storage) {
    super();
  }

  withEmbeddingService(service: TodoziEmbeddingService): UnifiedSearchTool {
    this.embeddingService = service;
    return this;
  }

  definition(): ToolDefinition {
    return {
      name: 'unified_search',
      description: 'Search across all Todozi data types with AI semantic capabilities (tasks, memories, ideas, errors)',
      parameters: [
        this.createToolParameter('query', 'string', 'Search query', true),
        this.createToolParameter('semantic', 'boolean', 'Use AI semantic search instead of keyword matching', false),
        this.createToolParameter('data_types', 'string', 'Comma-separated data types to search (tasks,memories,ideas,errors)', false),
        this.createToolParameter('limit', 'number', 'Maximum results per type', false)
      ],
      category: 'Search',
      resourceLocks: [{ type: 'FilesystemRead' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const queryValidation = ParameterValidator.validateString(kwargs.query, 'query', true, 1, 100);
    if (!queryValidation.isValid) {
      return this.error(queryValidation.errors.join('; '), 300);
    }

    const query = kwargs.query as string;
    const semantic = kwargs.semantic as boolean || false;

    try {
      if (semantic) {
        const results = await Done.deep(query);
        if (results.length === 0) {
          return this.success(`🤖 No AI semantic results found for: ${query}`, 300);
        }

        const formattedResults = results.map(result =>
          `• ${result.text_content} | Type: task | Similarity: ${result.similarity_score.toFixed(2)}`
        ).join('\n');

        return this.success(
          `🤖 AI Unified Search - Found ${results.length} semantic matches:\n${formattedResults}`,
          300
        );
      } else {
        const results = await Done.tdz_find(query);
        if (results.total === 0) {
          return this.success(`🔍 No unified results found for: ${query}`, 300);
        }

        return this.success(`🔍 Unified Search Results:\n${JSON.stringify(results, null, 2)}`, 300);
      }
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 300);
      }
      return this.error(`Unified search failed: ${e}`, 300);
    }
  }
}

class ProcessChatMessageTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'process_chat_message',
      description: 'Process a chat message containing Todozi tags and create corresponding items',
      parameters: [
        this.createToolParameter('message', 'string', 'Chat message with Todozi tags', true),
        this.createToolParameter('user_id', 'string', 'User ID for created items', false)
      ],
      category: 'Message Processing',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const messageValidation = ParameterValidator.validateString(kwargs.message, 'message', true, 1, 10000);
    if (!messageValidation.isValid) {
      return this.error(messageValidation.errors.join('; '), 250);
    }

    const message = kwargs.message as string;
    const userId = kwargs.user_id as string || 'ai_agent';

    try {
      const content = await Done.chat(message);
      const results: string[] = [];

      if (content.tasks.length > 0) {
        results.push(`📋 Created ${content.tasks.length} tasks`);
        content.tasks.forEach(task => {
          const assigneeStr = task.assignee ? task.assignee : 'unassigned';
          results.push(`  • ${task.action} [${assigneeStr}]`);
        });
      }

      if (content.memories.length > 0) {
        results.push(`🧠 Created ${content.memories.length} memories`);
        content.memories.forEach(memory => {
          results.push(`  • ${memory.moment}`);
        });
      }

      if (content.ideas.length > 0) {
        results.push(`💡 Created ${content.ideas.length} ideas`);
        content.ideas.forEach(idea => {
          results.push(`  • ${idea.idea}`);
        });
      }

      if (content.errors.length > 0) {
        results.push(`❌ Created ${content.errors.length} error records`);
      }

      if (content.feelings.length > 0) {
        results.push(`😊 Created ${content.feelings.length} feelings`);
      }

      if (results.length === 0) {
        return this.success('✅ Message processed - no structured content extracted', 250);
      }

      return this.success(results.join('\n'), 250);
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 250);
      }
      return this.error(`Failed to process chat message: ${e}`, 250);
    }
  }
}

class CreateErrorTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'create_error',
      description: 'Create an error record for tracking issues',
      parameters: [
        this.createToolParameter('title', 'string', 'Error title/summary', true),
        this.createToolParameter('description', 'string', 'Detailed error description', true),
        this.createToolParameter('severity', 'string', 'Severity level (low/medium/high/critical)', false),
        this.createToolParameter('category', 'string', 'Error category', false),
        this.createToolParameter('source', 'string', 'Source file/component', false),
        this.createToolParameter('context', 'string', 'Additional context', false),
        this.createToolParameter('tags', 'string', 'Comma-separated tags', false)
      ],
      category: 'Error Tracking',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const titleValidation = ParameterValidator.validateString(kwargs.title, 'title', true, 1, 200);
    const descriptionValidation = ParameterValidator.validateString(kwargs.description, 'description', true, 1, 1000);
    const sourceValidation = ParameterValidator.validateString(kwargs.source, 'source', true, 1, 200);

    if (!titleValidation.isValid || !descriptionValidation.isValid || !sourceValidation.isValid) {
      return this.error(
        [...titleValidation.errors, ...descriptionValidation.errors, ...sourceValidation.errors].join('; '),
        220
      );
    }

    const error: Error = {
      id: generateUUID(),
      title: kwargs.title as string,
      description: kwargs.description as string,
      severity: (kwargs.severity as string) as ErrorSeverity || ErrorSeverity.Medium,
      category: (kwargs.category as string) as ErrorCategory || ErrorCategory.Runtime,
      source: kwargs.source as string,
      context: kwargs.context as string,
      tags: kwargs.tags ? (kwargs.tags as string).split(',').map(t => t.trim()).filter(t => t) : [],
      resolved: false,
      created_at: new Date(),
      updated_at: new Date()
    };

    try {
      await this.todozi.saveError(error);
      return this.success(`Created error record '${error.title}' with ID: ${error.id}`, 220);
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 220);
      }
      return this.error(`Failed to create error record: ${e}`, 220);
    }
  }
}

class CreateCodeChunkTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'create_code_chunk',
      description: 'Create a code chunk for hierarchical task decomposition',
      parameters: [
        this.createToolParameter('chunk_id', 'string', 'Unique chunk identifier', true),
        this.createToolParameter('level', 'string', 'Chunking level (project/module/class/method/block)', true),
        this.createToolParameter('description', 'string', 'What this chunk accomplishes', true),
        this.createToolParameter('dependencies', 'string', 'Comma-separated dependency chunk IDs', false),
        this.createToolParameter('code', 'string', 'The actual code content', false)
      ],
      category: 'Code Chunking',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const chunkIdValidation = ParameterValidator.validateString(kwargs.chunk_id, 'chunk_id', true, 1, 100);
    const levelValidation = ParameterValidator.validateString(kwargs.level, 'level', true, 1, 50);
    const descriptionValidation = ParameterValidator.validateString(kwargs.description, 'description', true, 1, 500);

    if (!chunkIdValidation.isValid || !levelValidation.isValid || !descriptionValidation.isValid) {
      return this.error(
        [...chunkIdValidation.errors, ...levelValidation.errors, ...descriptionValidation.errors].join('; '),
        180
      );
    }

    const chunkId = kwargs.chunk_id as string;
    const levelStr = kwargs.level as string;

    let level: ChunkingLevel;
    switch (levelStr.toLowerCase()) {
      case 'project': level = ChunkingLevel.Project; break;
      case 'module': level = ChunkingLevel.Module; break;
      case 'class': level = ChunkingLevel.Class; break;
      case 'method': level = ChunkingLevel.Method; break;
      case 'block': level = ChunkingLevel.Block; break;
      default:
        return this.error(`Invalid chunking level: ${levelStr}`, 180);
    }

    const dependencies = kwargs.dependencies
      ? (kwargs.dependencies as string).split(',').map(d => d.trim()).filter(d => d)
      : [];

    const chunk: CodeChunk = {
      chunk_id: chunkId,
      status: ChunkStatus.Pending,
      dependencies,
      code: kwargs.code as string || '',
      tests: '',
      validated: false,
      level,
      estimated_tokens: 0,
      created_at: new Date(),
      updated_at: new Date()
    };

    try {
      await this.todozi.saveCodeChunk(chunk);
      return this.success(`Created code chunk '${chunk.chunk_id}' at level ${chunk.level}`, 180);
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 180);
      }
      return this.error(`Failed to create code chunk: ${e}`, 180);
    }
  }
}

class ChecklistTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'extract_tasks',
      description: 'Extract actionable tasks from message content and create them in Todozi',
      parameters: [
        this.createToolParameter('content', 'string', 'Message content to extract tasks from', true),
        this.createToolParameter('project', 'string', 'Project to associate extracted tasks with', false),
        this.createToolParameter('priority', 'string', 'Default priority for extracted tasks (low/medium/high/critical/urgent)', false),
        this.createToolParameter('assignee', 'string', 'Default assignee for extracted tasks (ai/human/collaborative)', false)
      ],
      category: 'Task Management',
      resourceLocks: [{ type: 'FilesystemWrite' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const contentValidation = ParameterValidator.validateString(kwargs.content, 'content', true, 1, 10000);
    if (!contentValidation.isValid) {
      return this.error(contentValidation.errors.join('; '), 150);
    }

    const content = kwargs.content as string;
    const defaultProject = kwargs.project as string || '';
    const defaultPriority = kwargs.priority as Priority || Priority.Medium;
    const defaultAssignee = kwargs.assignee as Assignee;

    const extractedTasks = this.extractTasksFromContent(content);

    if (extractedTasks.length === 0) {
      return this.success('No tasks found in content', 150);
    }

    let createdCount = 0;

    for (const taskAction of extractedTasks) {
      try {
        const task: Task = {
          id: generateUUID(),
          user_id: 'ai_agent',
          action: taskAction,
          time: 'ASAP',
          priority: defaultPriority,
          parent_project: defaultProject,
          status: Status.Todo,
          assignee: defaultAssignee,
          tags: ['extracted'],
          dependencies: [],
          context_notes: 'Extracted from message content',
          progress: 0,
          created_at: new Date(),
          updated_at: new Date()
        };

        await this.todozi.addTaskToProject(task);
        createdCount++;
      } catch (e) {
        console.error(`Failed to create task: ${e}`);
      }
    }

    return this.success(`Extracted and created ${createdCount} tasks from content`, 150);
  }

  extractTasksFromContent(content: string): string[] {
    const tasks: string[] = [];
    const taskIndicators = [
      /^\s*[\-\*]\s*\[\s*\]\s*(.+)$/i,
      /^\s*[\-\*]\s*(.+)$/i,
      /^\s*\d+\.\s*(.+)$/i,
      /^\s*todo:\s*(.+)$/i,
      /^\s*task:\s*(.+)$/i,
      /need to\s+(.+)$/i,
      /should\s+(.+)$/i,
      /must\s+(.+)$/i
    ];

    for (const line of content.split('\n')) {
      for (const pattern of taskIndicators) {
        const match = line.match(pattern);
        if (match) {
          const task = match[1].trim();
          if (task && task.length <= 200) {
            tasks.push(task);
            break;
          }
        }
      }
    }

    // Extract from sentences if no checklist items found
    if (tasks.length === 0) {
      const sentencePatterns = [
        /I will\s+(.+?)(?:\.|$)/i,
        /We need to\s+(.+?)(?:\.|$)/i,
        /Let's\s+(.+?)(?:\.|$)/i
      ];

      for (const pattern of sentencePatterns) {
        const matches = content.matchAll(pattern);
        for (const match of matches) {
          const task = match[1].trim();
          if (task && task.length <= 200) {
            tasks.push(task);
          }
        }
      }
    }

    // Deduplicate
    const seen = new Set<string>();
    return tasks.filter(task => {
      const lower = task.toLowerCase();
      if (seen.has(lower)) return false;
      seen.add(lower);
      return true;
    });
  }

  static extractTasks(content: string): string[] {
    // Static version for testing
    const tool = new ChecklistTool(new Storage());
    return tool.extractTasksFromContent(content);
  }
}

class SimpleTodoziTool extends Tool {
  constructor(private todozi: Storage) {
    super();
  }

  definition(): ToolDefinition {
    return {
      name: 'simple_todozi',
      description: 'Ultra-simple Todozi interface with automatic AI/human coordination and smart search',
      parameters: [
        this.createToolParameter('action', 'string', 'What to do: \'task\', \'urgent\', \'find\', \'remember\', \'idea\', \'stats\', \'ai_search\', \'complete\', \'start\'', true),
        this.createToolParameter('content', 'string', 'The content/description for the action', true),
        this.createToolParameter('extra', 'string', 'Extra context, meaning, or details', false)
      ],
      category: 'Simple Task Management',
      resourceLocks: [{ type: 'FilesystemWrite' }, { type: 'FilesystemRead' }]
    };
  }

  async execute(kwargs: ExecuteParams): Promise<ToolResult> {
    const actionValidation = ParameterValidator.validateString(kwargs.action, 'action', true);
    const contentValidation = ParameterValidator.validateString(kwargs.content, 'content', true);

    if (!actionValidation.isValid || !contentValidation.isValid) {
      return this.error(
        [...actionValidation.errors, ...contentValidation.errors].join('; '),
        50
      );
    }

    const action = kwargs.action as string;
    const content = kwargs.content as string;
    const extra = kwargs.extra as string || '';

    try {
      switch (action.toLowerCase()) {
        case 'task':
          const taskId = await Done.task(content);
          return this.success(`✅ Task created: ${taskId}`, 50);
        case 'urgent':
          const urgentId = await Done.urgent(content);
          return this.success(`🚨 Urgent task created: ${urgentId}`, 50);
        case 'high':
          const highId = await Done.high(content);
          return this.success(`🟠 High priority task created: ${highId}`, 50);
        case 'low':
          const lowId = await Done.low(content);
          return this.success(`🟢 Low priority task created: ${lowId}`, 50);
        case 'ai':
          const aiId = await Done.ai(content);
          return this.success(`🤖 AI task queued: ${aiId}`, 50);
        case 'human':
          const humanId = await Done.human(content);
          return this.success(`👤 Human task created (visible in TUI): ${humanId}`, 50);
        case 'collab':
          const collabId = await Done.collab(content);
          return this.success(`🤝 Collaborative task created: ${collabId}`, 50);
        case 'find':
          const findResults = await Done.find(content);
          return this.success(`🔍 Smart search results:\n${JSON.stringify(findResults, null, 2)}`, 50);
        case 'ai_search':
          const aiResults = await Done.deep(content);
          const formatted = aiResults.map(r => `• ${r.text_content} [ID: ${r.content_id}]`).join('\n');
          return this.success(`🤖 AI semantic search:\n${formatted}`, 50);
        case 'fast_search':
          const fastResults = await Done.fast(content);
          return this.success(`⚡ Fast keyword search:\n${JSON.stringify(fastResults, null, 2)}`, 50);
        case 'smart_search':
          const smartResults = await Done.smart(content);
          return this.success(`🧠 Smart intent search:\n${JSON.stringify(smartResults, null, 2)}`, 50);
        case 'remember':
          const memResult = await Done.create_memory(content, extra, 'Created via simple tool');
          return this.success(`🧠 Memory saved: ${memResult.id}`, 50);
        case 'important_memory':
          const impId = await Done.important(content, extra, 'Important via simple tool');
          return this.success(`🧠⭐ Important memory saved: ${impId}`, 50);
        case 'idea':
          const ideaResult = await Done.create_idea(content, extra);
          return this.success(`💡 Idea saved: ${ideaResult.id}`, 50);
        case 'breakthrough_idea':
          const breakId = await Done.breakthrough(content);
          return this.success(`💡🚀 Breakthrough idea saved: ${breakId}`, 50);
        case 'complete':
          await Done.complete(content);
          return this.success(`✅ Task ${content} completed`, 50);
        case 'start':
          await Done.begin(content);
          return this.success(`🔄 Task ${content} started`, 50);
        case 'stats':
          const stats = await Done.quick();
          return this.success(`📊 Quick stats:\n${stats}`, 50);
        case 'queue':
          const items = await Done.list_queue_items();
          return this.success(`📋 Queue: ${items.length} total items`, 50);
        case 'chat':
          const chatContent = await Done.chat(content);
          const results: string[] = [];
          if (chatContent.tasks.length > 0) results.push(`📋 ${chatContent.tasks.length} tasks`);
          if (chatContent.memories.length > 0) results.push(`🧠 ${chatContent.memories.length} memories`);
          if (chatContent.ideas.length > 0) results.push(`💡 ${chatContent.ideas.length} ideas`);
          const summary = results.length > 0
            ? `✅ Chat processed: ${results.join(', ')}`
            : '✅ Chat processed - no structured content';
          return this.success(summary, 50);
        case 'extract':
          const extractedTasks = await Done.extract_tasks(content, extra);
          if (extractedTasks.length === 0) {
            return this.success('🤖 No tasks extracted from content', 50);
          }
          const taskList = extractedTasks.map((task, i) => `${i + 1}. ${task}`).join('\n');
          return this.success(
            `🤖 Extracted ${extractedTasks.length} tasks via todozi.com AI:\n${taskList}`,
            50
          );
        case 'expand':
          const plannedTasks = await Done.plan_tasks(content, 'medium', 'ASAP', extra);
          if (plannedTasks.length === 0) {
            return this.success('🤖 No task expansion generated', 50);
          }
          const plannedList = plannedTasks.map((task, i) => `${i + 1}. ${task.action}`).join('\n');
          return this.success(
            `🤖 Expanded into ${plannedTasks.length} subtasks via todozi.com AI:\n${plannedList}`,
            50
          );
        default:
          return this.error(
            `❌ Unknown action: '${action}'. Available: task, urgent, high, low, ai, human, collab, find, ai_search, fast_search, smart_search, remember, important_memory, idea, breakthrough_idea, complete, start, stats, queue, chat, extract, expand`,
            50
          );
      }
    } catch (e) {
      if (e instanceof TodoziError) {
        return this.error(e.message, 50);
      }
      return this.error(`Operation failed: ${e}`, 50);
    }
  }
}

// ==================== TOOL FACTORY ====================

class ToolFactory {
  constructor(private config: ToolFactoryConfig) {}

  async createTools(todozi: Storage, options?: ToolOptions): Promise<Tool[]> {
    const tools: Tool[] = [
      new SimpleTodoziTool(todozi),
      new CreateTaskTool(todozi),
      new SearchTasksTool(todozi),
      new UpdateTaskTool(todozi),
      new CreateMemoryTool(todozi),
      new CreateIdeaTool(todozi),
      new UnifiedSearchTool(todozi),
      new ProcessChatMessageTool(todozi),
      new CreateErrorTool(todozi),
      new CreateCodeChunkTool(todozi),
      new ChecklistTool(todozi)
    ];

    if (options?.includeAdvanced) {
      // Add advanced tools here
    }

    return tools;
  }
}

// ==================== INITIALIZATION ====================

async function initializeGrokLevelTodoziSystem(enableEmbeddings = false): Promise<{ storage: Storage, embeddingService?: TodoziEmbeddingService }> {
  // Initialize storage with persistent backend
  const storage = new Storage(new FileStorageBackend());
  
  let embeddingService: TodoziEmbeddingService | undefined;
  if (enableEmbeddings) {
    const config: TodoziEmbeddingConfig = {
      model_name: 'all-MiniLM-L6-v2',
      max_results: 10,
      similarity_threshold: 0.7,
      cache_ttl_seconds: 3600,
      clustering_threshold: 0.8,
      dimensions: 384,
      enable_clustering: false
    };
    
    try {
      embeddingService = await TodoziEmbeddingService.new(config);
    } catch (e) {
      console.error(`Warning: Failed to initialize embedding service: ${e}`);
    }
  }

  return { storage, embeddingService };
}

// ==================== UTILITY FUNCTIONS ====================

async function createTodoziTools(storage: Storage): Promise<Tool[]> {
  const config: ToolFactoryConfig = {
    enableEmbeddings: false,
    defaultTimeout: 30000,
    maxRetries: 3
  };

  const factory = new ToolFactory(config);
  return factory.createTools(storage);
}

// ==================== TESTING UTILITIES ====================

class ToolTestHarness {
  static async testTool(
    tool: Tool,
    params: ExecuteParams,
    expected: Partial<ToolResult>
  ): Promise<void> {
    const result = await tool.execute(params);
    
    if (expected.success !== undefined) {
      console.assert(result.success === expected.success, `Expected success ${expected.success}, got ${result.success}`);
    }
    
    if (expected.code) {
      console.assert(result.code === expected.code, `Expected code ${expected.code}, got ${result.code}`);
    }
    
    if (expected.message) {
      console.assert(result.message.includes(expected.message), `Expected message to contain ${expected.message}`);
    }
  }
}

// ==================== EXPORTS ====================

export {
  // Types
  Task,
  Memory,
  Idea,
  Error,
  CodeChunk,
  ToolDefinition,
  ToolResult,
  ExecuteParams,
  TaskUpdate,
  TaskFilters,
  SearchResult,
  PredictedError,
  QualityScore,
  QualityMetrics,
  LearningData,
  LearningInsights,
  AgentMetrics,
  UnifiedSearchResults,
  QueueItem,
  ValidationResult,
  TodoziConfig,
  ToolFactoryConfig,
  ToolOptions,
  TodoziEmbeddingConfig,
  StorageBackend,

  // Enums
  Priority,
  Status,
  Assignee,
  ItemStatus,
  MemoryImportance,
  MemoryTerm,
  MemoryType,
  IdeaImportance,
  ShareLevel,
  ErrorSeverity,
  ErrorCategory,
  ChunkingLevel,
  ChunkStatus,

  // Classes
  TodoziError,
  ValidationError,
  ApiError,
  StorageError,
  ResourceManager,
  ParameterValidator,
  Storage,
  FileStorageBackend,
  TodoziClient,
  Done,
  TodoziEmbeddingService,
  Tool,
  CreateTaskTool,
  SearchTasksTool,
  UpdateTaskTool,
  CreateMemoryTool,
  CreateIdeaTool,
  UnifiedSearchTool,
  ProcessChatMessageTool,
  CreateErrorTool,
  CreateCodeChunkTool,
  ChecklistTool,
  SimpleTodoziTool,
  ToolFactory,
  ToolTestHarness,

  // Functions
  initializeGrokLevelTodoziSystem,
  createTodoziTools
};
