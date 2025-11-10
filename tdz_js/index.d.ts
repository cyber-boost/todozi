/**
 * Todozi JavaScript SDK - TypeScript Definitions
 * 
 * Type definitions for the Todozi JavaScript SDK.
 * This file provides TypeScript support for the CommonJS/ES module hybrid structure.
 * 
 * The actual implementation uses CommonJS modules, but exports are re-exported
 * through todozi/index.js as ES modules for better tree-shaking and modern tooling.
 */

// Main entry point - matches package.json "main": "todozi/main.js"
// For TypeScript, we declare the types based on what's exported from todozi/index.js

// Type definitions for todozi/index.js exports
// This matches the actual exports from the todozi/index.js ES module
// Users can import like: import { Storage, Task } from 'todozi-javascript/todozi'
declare module 'todozi-javascript/todozi' {
  // Agent Management
  export class AgentManager {
    agents: Map<string, Agent>;
    agent_assignments: AgentAssignment[];
    
    constructor();
    static new(): AgentManager;
    loadAgents(): Promise<void>;
    createAgent(agent: Agent): Promise<string>;
    updateAgent(agentId: string, updates: AgentUpdate): Promise<void>;
    deleteAgent(agentId: string): Promise<void>;
    getAgent(agentId: string): Agent | undefined;
    getAllAgents(): Agent[];
    getAvailableAgents(): Agent[];
    getAgentsBySpecialization(specialization: string): Agent[];
    getAgentsByCapability(capability: string): Agent[];
    assignTaskToAgent(taskId: string, agentId: string, projectId: string): Promise<string>;
    completeAgentAssignment(taskId: string): Promise<void>;
    saveAgent(agent: Agent): Promise<void>;
  }
  
  export interface AgentStatistics {
    total: number;
    available: number;
    busy: number;
  }
  
  export interface AgentUpdate {
    name?: string;
    description?: string;
    status?: string;
  }

  // API Key Management
  export class ApiKeyManager {
    static new(): ApiKeyManager;
    createApiKey(userId?: string): any;
    checkApiKeyAuth(publicKey: string, privateKey?: string | null): [string, boolean];
  }
  
  export function activateApiKey(userId: string): void;
  export function checkApiKeyAuth(publicKey: string, privateKey?: string | null): [string, boolean];
  export function createApiKey(): any;
  export function createApiKeyWithUserId(userId: string): any;
  export function deactivateApiKey(userId: string): any;
  export function getApiKey(userId: string): any;
  export function getApiKeyByPublic(publicKey: string): any;
  export function listActiveApiKeys(): any[];
  export function listApiKeys(): any[];
  export function loadApiKeyCollection(): any;
  export function saveApiKeyCollection(collection: any): any;

  // Storage
  export class Storage {
    static new(): Promise<Storage>;
    listTasksAcrossProjects(filters?: any): Promise<any[]>;
    getTaskFromAnyProject(id: string): any;
    addTaskToProject(task: any): Promise<void>;
    updateTaskInProject(id: string, updates: any): Promise<any>;
    deleteTaskFromProject(id: string): Promise<void>;
    searchTasks(query: string): Promise<any[]>;
    searchTasksSemantic(query: string, limit?: number): Promise<any[]>;
    createProject(name: string, description?: string): Promise<void>;
    getProject(name: string): Promise<any>;
    listProjects(): Promise<any[]>;
  }
  
  export function initStorage(): Promise<void>;
  export function getStorageDir(): string;
  export function checkFolderStructure(): any;
  export function listProjects(): Promise<any[]>;
  export function loadProject(name: string): Promise<any>;
  export function saveProject(project: any): Promise<void>;
  export function loadConfig(): any;
  export function saveConfig(config: any): Promise<void>;
  export function listProjectTaskContainers(): any[];
  export function loadProjectTaskContainer(name: string): any;
  export function saveProjectTaskContainer(container: any): Promise<void>;

  // Models
  export class Task {
    id: string;
    user_id: string;
    action: string;
    time: string;
    priority: string;
    parent_project: string;
    status: string;
    assignee?: string;
    tags: string[];
    dependencies: string[];
    context_notes?: string;
    progress: number;
    created_at: Date;
    updated_at: Date;
    
    static new(userId: string, action: string, time: string, priority: any, parentProject: string, status: any): Task;
    static newFull(
      userId: string,
      action: string,
      time: string,
      priority: any,
      parentProject: string,
      status: any,
      assignee: any,
      tags: string[],
      dependencies: string[],
      contextNotes: string | null,
      progress: number
    ): Task;
  }
  
  export class Priority {
    static Low: string;
    static Medium: string;
    static High: string;
    static Critical: string;
    static Urgent: string;
    static fromStr(s: string): string;
  }
  
  export class Status {
    static Todo: string;
    static Pending: string;
    static InProgress: string;
    static Blocked: string;
    static Review: string;
    static Done: string;
    static Completed: string;
    static Cancelled: string;
    static Deferred: string;
    static fromStr(s: string): string;
  }
  
  export class Assignee {
    static Ai: string;
    static Human: string;
    static Collaborative: string;
    static fromStr(s: string): string;
  }
  
  export class Project {
    name: string;
    description?: string;
    status: string;
    created_at: Date;
    updated_at: Date;
  }
  
  export class QueueItem {
    id: string;
    task_name: string;
    task_description: string;
    priority: string;
    project_id: string;
    status: string;
    created_at: Date;
    updated_at: Date;
  }
  
  export class Memory {
    id: string;
    moment: string;
    meaning: string;
    reason?: string;
    importance: string;
    term: string;
    tags: string[];
    created_at: Date;
    updated_at: Date;
  }
  
  export class Idea {
    id: string;
    idea: string;
    share: string;
    importance: string;
    tags: string[];
    created_at: Date;
    updated_at: Date;
  }
  
  export class ApiKey {
    userId: string;
    publicKey: string;
    privateKey: string;
    active: boolean;
    createdAt: Date;
    updatedAt: Date;
    
    constructor(options: {
      userId: string;
      publicKey: string;
      privateKey: string;
      active?: boolean;
      createdAt?: Date;
      updatedAt?: Date;
    });
    static new(): ApiKey;
    static withUserId(userId: string): ApiKey;
    deactivate(): void;
    activate(): void;
    isActive(): boolean;
    matches(publicKey: string, privateKey?: string | null): boolean;
    isAdmin(publicKey: string, privateKey: string): boolean;
  }

  // Memory Management
  export class MemoryManager {
    memories: Map<string, Memory>;
    memoryTags: Map<string, string[]>;
    
    constructor();
    createMemory(memory: Memory): Promise<string>;
    getMemory(memoryId: string): Memory | undefined;
    getAllMemories(): Memory[];
    updateMemory(memoryId: string, updates: MemoryUpdate): Promise<void>;
    deleteMemory(memoryId: string): Promise<void>;
    searchMemories(query: string): Memory[];
    getMemoriesByImportance(importance: MemoryImportance): Memory[];
    getMemoriesByTerm(term: MemoryTerm): Memory[];
    getMemoriesByTag(tag: string): Memory[];
    getRecentMemories(limit: number): Memory[];
    getCriticalMemories(): Memory[];
    getShortTermMemories(): Memory[];
    getLongTermMemories(): Memory[];
    getMemoriesByType(memoryType: MemoryType): Memory[];
    getSecretMemories(): Memory[];
    getHumanMemories(): Memory[];
    getEmotionalMemories(emotion: string): Memory[];
    getAllTags(): string[];
    getMemoryStatistics(): MemoryStatistics;
  }
  
  export interface MemoryStatistics {
    total: number;
    byImportance: Record<string, number>;
    byTerm: Record<string, number>;
  }
  
  export interface MemoryUpdate {
    moment?: string;
    meaning?: string;
    importance?: string;
  }

  // Idea Management
  export class IdeaManager {
    ideas: Map<string, Idea>;
    idea_tags: Map<string, string[]>;
    
    constructor();
    createIdea(idea: Idea): Promise<string>;
    getIdea(ideaId: string): Idea | undefined;
    getAllIdeas(): Idea[];
    updateIdea(ideaId: string, updates: IdeaUpdate): Promise<void>;
    deleteIdea(ideaId: string): Promise<void>;
    searchIdeas(query: string): Idea[];
    getIdeasByImportance(importance: IdeaImportance): Idea[];
    getIdeasByShareLevel(shareLevel: ShareLevel): Idea[];
    getIdeasByTag(tag: string): Idea[];
    getPublicIdeas(): Idea[];
    getTeamIdeas(): Idea[];
    getPrivateIdeas(): Idea[];
    getBreakthroughIdeas(): Idea[];
    getRecentIdeas(limit: number): Idea[];
    getAllTags(): string[];
    getTagStatistics(): Map<string, number>;
    getIdeaStatistics(): IdeaStatistics;
  }
  
  export interface IdeaStatistics {
    total: number;
    byImportance: Record<string, number>;
  }
  
  export interface IdeaUpdate {
    idea?: string;
    share?: string;
    importance?: string;
  }

  // Embedding Service
  export class TodoziEmbeddingService {
    config: TodoziEmbeddingConfig;
    cache: Map<string, any>;
    embeddingModel: any;
    embeddingModels: Map<string, any>;
    tagManager: any;
    storage: any;
    
    constructor(config?: TodoziEmbeddingConfig, cache?: Map<string, any>, embeddingModel?: any, embeddingModels?: Map<string, any>, tagManager?: any, storage?: any);
    static new(config?: TodoziEmbeddingConfig): Promise<TodoziEmbeddingService>;
    static withSharedComponents(config: TodoziEmbeddingConfig, cache: Map<string, any>, embeddingModel: any, embeddingModels: Map<string, any>, tagManager: any, storage: any): TodoziEmbeddingService;
    initialize(): Promise<void>;
    createProject(name: string, description?: string): Promise<string>;
    addTask(task: any): Promise<string>;
    updateTask(id: string, updates: any): Promise<void>;
    newIdea(idea: any): Promise<string>;
    newMemory(memory: any): Promise<string>;
    embedTag(tag: any): Promise<string>;
    embedIdea(idea: any): Promise<string>;
    embedMemory(memory: any): Promise<string>;
    generateEmbedding(text: string): Promise<number[]>;
    findSimilarTasks(taskDescription: string, limit?: number | null): Promise<SimilarityResult[]>;
    findSimilarTags(tagName: string, limit?: number | null): Promise<SimilarityResult[]>;
    semanticSearch(query: string, contentTypes?: string[] | null, limit?: number | null): Promise<SimilarityResult[]>;
    getTask(taskId: string): Promise<any>;
  }
  
  export class TodoziEmbeddingConfig {
    model_name: string;
    dimensions: number;
    similarity_threshold: number;
    max_results: number;
    cache_ttl_seconds: number;
    enable_clustering: boolean;
    clustering_threshold: number;
    
    constructor();
    static default(): TodoziEmbeddingConfig;
  }

  // CLI Handler
  export class TodoziHandler {
    static new(storage: Storage): Promise<TodoziHandler>;
    handleAddCommand(command: any): Promise<any>;
    handleMemoryCommand(command: any): Promise<any>;
    handleIdeaCommand(command: any): Promise<any>;
    handleListCommand(command: any): Promise<any>;
    handleSearchCommand(command: any): Promise<any>;
    handleUpdateCommand(id: string, updates: any): Promise<any>;
    handleDeleteCommand(id: string): Promise<void>;
  }

  // Server
  export class TodoziServer {
    static new(config: ServerConfig): Promise<TodoziServer>;
    start(): Promise<void>;
  }
  
  export class ServerConfig {
    static default(): ServerConfig;
    host: string;
    port: number;
    max_connections: number;
  }

  // Processing Functions
  export function processChatMessageExtended(message: string, userId: string): {
    tasks: any[];
    memories: any[];
    ideas: any[];
    agent_assignments: any[];
    code_chunks: any[];
  };
  
  export function processChatMessage(message: string): any[];
  export function processWorkflow(tasks: Task[]): any[];

  // Parsing Functions
  export function parseMemoryFormat(text: string, userId: string): any;
  export function parseIdeaFormat(text: string): any;
  export function parseTodoziFormat(text: string): any;
  export function parseAgentAssignmentFormat(text: string): any;
  export function parseErrorFormat(text: string): any;
  export function parseFeelingFormat(text: string): any;
  export function parseTrainingDataFormat(text: string): any;
  export function transformShorthandTags(message: string): string;

  // Task Execution
  export function executeTask(storage: Storage, task: Task): any;
  export function executeAiTask(storage: Storage, task: Task): any;
  export function executeHumanTask(storage: Storage, task: Task): any;
  export function executeCollaborativeTask(storage: Storage, task: Task): any;
  export function executeAgentTask(storage: Storage, task: Task, agentId: string): any;

  // Additional Manager Classes
  export class ErrorHandler {
    static handleError(error: any, context: string): ToolResult;
    static validateRequiredParams(kwargs: Record<string, any>, required_params: string[]): ToolResult | null;
    static validateStringParam(value: any, param_name: string, min_length: number, max_length: number, pattern?: string | null): ToolResult | null;
  }
  
  export class ErrorManager {
    errors: Map<string, any>;
    
    constructor();
    createError(error: any): Promise<string>;
    getUnresolvedErrors(): any[];
    resolveError(errorId: string, resolution: string): Promise<void>;
    generateUUID(): string;
  }
  
  export class TagManager {
    tags: Map<string, Tag>;
    tag_relationships: Map<string, string[]>;
    category_tags: Map<string, string[]>;
    
    constructor();
    static new(): TagManager;
    createTag(tag: Tag): Promise<string>;
    getTag(tagId: string): Tag | undefined;
    getTagByName(name: string): Tag | undefined;
    getAllTags(): Tag[];
    updateTag(tagId: string, updates: TagUpdate): Promise<void>;
    deleteTag(tagId: string): Promise<void>;
    addTagRelationship(tagId: string, relatedTagId: string): Promise<void>;
    getRelatedTags(tagId: string): Tag[];
    searchTags(query: string): Tag[];
    getTagsByCategory(category: string): Tag[];
    getAllCategories(): string[];
    incrementTagUsage(tagName: string): Promise<void>;
    getMostUsedTags(limit: number): Tag[];
    getRecentTags(limit: number): Tag[];
    getTagStatistics(): TagStatistics;
  }
  
  export class ReminderManager {
    reminders: Map<string, Reminder>;
    reminder_tags: Map<string, string[]>;
    
    constructor();
    static new(): ReminderManager;
    createReminder(reminder: Reminder): Promise<string>;
    getReminder(reminderId: string): Reminder | undefined;
    getAllReminders(): Reminder[];
    updateReminder(reminderId: string, updates: ReminderUpdate): Promise<void>;
    deleteReminder(reminderId: string): Promise<void>;
    searchReminders(query: string): Reminder[];
    getRemindersByPriority(priority: ReminderPriority): Reminder[];
    getRemindersByStatus(status: ReminderStatus): Reminder[];
    getRemindersByTag(tag: string): Reminder[];
    getPendingReminders(): Reminder[];
    getActiveReminders(): Reminder[];
    getOverdueReminders(): Reminder[];
    getRemindersDueSoon(duration: number): Reminder[];
    getRecentReminders(limit: number): Reminder[];
    getAllTags(): string[];
    getTagStatistics(): Record<string, number>;
    getReminderStatistics(): ReminderStatistics;
  }
  
  export class SummaryManager {
    summaries: Map<string, Summary>;
    summary_tags: Map<string, string[]>;
    
    constructor();
    static new(): SummaryManager;
    createSummary(summary: Summary): Promise<string>;
    getSummary(summaryId: string): Summary | undefined;
    getAllSummaries(): Summary[];
    updateSummary(summaryId: string, updates: SummaryUpdate): Promise<void>;
    deleteSummary(summaryId: string): Promise<void>;
    searchSummaries(query: string): Summary[];
    getSummariesByPriority(priority: SummaryPriority): Summary[];
    getSummariesByTag(tag: string): Summary[];
    getRecentSummaries(limit: number): Summary[];
    getHighPrioritySummaries(): Summary[];
    getAllTags(): string[];
    getTagStatistics(): Record<string, number>;
    getSummaryStatistics(): SummaryStatistics;
  }
  
  export class QueueCollection {
    static new(): QueueCollection;
    add_item(item: QueueItem): void;
    get_all_items(): QueueItem[];
    get_items_by_status(status: string): QueueItem[];
    get_active_sessions(): any[];
    start_session(itemId: string): string;
    end_session(sessionId: string): void;
  }
  
  // Additional Classes
  export class ApiKeyCollection {
    version: string;
    createdAt: Date;
    updatedAt: Date;
    keys: Map<string, ApiKey>;
    
    static new(): ApiKeyCollection;
    static default(): ApiKeyCollection;
    addKey(key: ApiKey): void;
    getKey(userId: string): ApiKey | undefined;
    getKeyByPublic(publicKey: string): ApiKey | undefined;
    getAllKeys(): ApiKey[];
    getActiveKeys(): ApiKey[];
    removeKey(userId: string): ApiKey | undefined;
    deactivateKey(userId: string): boolean;
    activateKey(userId: string): boolean;
  }
  
  export class Config {
    registration: any | null;
    version: string;
    defaultProject: string;
    autoBackup: boolean;
    backupInterval: string;
    aiEnabled: boolean;
    defaultAssignee: string | null;
    dateFormat: string;
    timezone: string;
    
    constructor(options?: {
      registration?: any | null;
      version?: string;
      defaultProject?: string;
      autoBackup?: boolean;
      backupInterval?: string;
      aiEnabled?: boolean;
      defaultAssignee?: string | null;
      dateFormat?: string;
      timezone?: string;
    });
    static default(): Config;
  }
  
  export class DisplayConfig {
    showAiInsights: boolean;
    showSimilarityScores: boolean;
    showRelatedTasks: boolean;
    maxRelatedTasks: number;
    colorScheme: any;
    compactMode: boolean;
    showEmbeddings: boolean;
    showIds: boolean;
    showCreatedAt: boolean;
    showDependencies: boolean;
    showContext: boolean;
    showProgress: boolean;
    
    constructor();
  }
  
  export class MLEngine {
    modelName: string;
    temperature: number;
    maxTokens: number;
    
    constructor(modelName: string);
    static new(modelName: string): MLEngine;
    withTemperature(temperature: number): MLEngine;
    withMaxTokens(maxTokens: number): MLEngine;
    predictRelevance(features: any): Promise<number>;
    craftEmbedding(features: any): Promise<number[]>;
    strikeTags(features: any): Promise<number[]>;
    strikeCluster(embedding: number[]): Promise<number>;
    analyzeCodeQuality(features: any): Promise<number>;
  }
  
  export class ModelConfig {
    provider: string;
    name: string;
    temperature: number;
    maxTokens: number;
    
    constructor(options: {
      provider: string;
      name: string;
      temperature: number;
      maxTokens: number;
    });
  }
  
  export class TagSearchEngine {
    tagManager: any;
    
    constructor(tagManager: any);
    static new(tagManager: any): TagSearchEngine;
    advancedSearch(query: TagSearchQuery): any[];
    fuzzySearch(query: string, maxDistance: number): Array<[any, number]>;
    getSuggestions(currentTags: string[], limit: number): string[];
  }
  
  export class TaskCollection {
    version: string;
    createdAt: Date;
    updatedAt: Date;
    tasks: Map<string, Task>;
    
    constructor();
    static new(): TaskCollection;
    static default(): TaskCollection;
    addTask(task: Task): void;
    getTask(id: string): Task | undefined;
    getTaskMut(id: string): Task | undefined;
    removeTask(id: string): Task | undefined;
    getAllTasks(): Task[];
    getFilteredTasks(filters: TaskFilters): Task[];
  }
  
  export class TuiService extends NodeJS.EventEmitter {
    embeddingService: any;
    displayConfig: DisplayConfig;
    
    constructor(embeddingService: any, displayConfig?: DisplayConfig);
    displayTask(taskId: string): Promise<TaskDisplay>;
    displayTasks(taskIds: string[]): Promise<TaskListDisplay>;
    startEditSession(taskId: string): Promise<EditSession>;
    generateAiSuggestions(task: Task, similarTasks: any[]): Promise<string[]>;
    extractSemanticTags(similarTasks: any[]): string[];
    calculateConfidenceScore(similarTasks: any[]): number;
    generateAiSummary(taskDisplays: TaskDisplay[]): Promise<string>;
    findSemanticClusters(taskDisplays: TaskDisplay[]): Promise<string[][]>;
    calculateTaskSimilarity(task1: TaskDisplay, task2: TaskDisplay): number;
    findCommonTags(similarTasks: any[]): string[];
    showLoadingScreen(): Promise<void>;
  }

  // Embedding Service Types
  export class AggregationType {
    static Average: string;
    static Max: string;
    static Min: string;
    static Weighted(weights: number[]): { type: string; weights: number[] };
  }
  
  export class ClusteringResult {
    cluster_id: string;
    content_items: SimilarityResult[];
    cluster_center: number[];
    cluster_size: number;
    average_similarity: number;
    
    constructor(cluster_id: string, content_items: SimilarityResult[], cluster_center: number[], cluster_size: number, average_similarity: number);
  }
  
  export type DiagnosticReport = any;
  
  export class DriftReport {
    content_id: string;
    current_similarity_to_original: number;
    drift_percentage: number;
    significant_drift: boolean;
    history: DriftSnapshot[];
    
    constructor(content_id: string, current_similarity_to_original: number, drift_percentage: number, significant_drift: boolean, history: DriftSnapshot[]);
  }
  
  export class DriftSnapshot {
    timestamp: Date;
    similarity_to_original: number;
    text_sample: string;
    
    constructor(timestamp: Date, similarity_to_original: number, text_sample: string);
  }
  
  export type EmbeddingModel = any;
  export type EmbeddingStats = any;
  
  export class GraphEdge {
    from: string;
    to: string;
    similarity: number;
    bidirectional: boolean;
    
    constructor(from: string, to: string, similarity: number, bidirectional?: boolean);
  }
  
  export class GraphNode {
    id: string;
    content_type: string;
    label: string;
    metadata: Record<string, any>;
    
    constructor(id: string, content_type: string, label: string, metadata?: Record<string, any>);
  }
  
  export class HierarchicalCluster {
    cluster_id: string;
    level: number;
    content_items: SimilarityResult[];
    cluster_center: number[];
    children: HierarchicalCluster[];
    parent_id: string | null;
    average_similarity: number;
    
    constructor(cluster_id: string, level: number, content_items: SimilarityResult[], cluster_center: number[], children?: HierarchicalCluster[], parent_id?: string | null, average_similarity?: number);
  }
  
  export type LRUEmbeddingCache = any;
  
  export class LabeledCluster {
    cluster_id: string;
    label: string;
    description: string;
    confidence: number;
    content_items: SimilarityResult[];
    
    constructor(cluster_id: string, label: string, description: string, confidence: number, content_items: SimilarityResult[]);
  }
  
  export type ModelComparisonResult = any;
  
  export class ModelEmbeddingResult {
    model_name: string;
    embedding: number[];
    dimensions: number;
    generation_time_ms: number;
    
    constructor(model_name: string, embedding: number[], dimensions: number, generation_time_ms: number);
  }
  
  export type PerformanceMetrics = any;
  
  export class SearchFilters {
    tags: string[] | null;
    priority: string | null;
    status: string | null;
    assignee: string | null;
    date_from: Date | null;
    date_to: Date | null;
    min_progress: number | null;
    max_progress: number | null;
    
    constructor();
  }
  
  export class SimilarityGraph {
    nodes: GraphNode[];
    edges: GraphEdge[];
    
    constructor(nodes?: GraphNode[], edges?: GraphEdge[]);
  }
  
  export class SimilarityResult {
    content_id: string;
    content_type: string;
    similarity_score: number;
    text_content: string;
    tags: string[];
    metadata: Record<string, any>;
    
    constructor(content_id: string, content_type: string, similarity_score: number, text_content: string, tags: string[], metadata?: Record<string, any>);
  }
  
  export type TodoziContentType = any;
  
  export class TodoziEmbeddingCache {
    vector: number[];
    content_type: string;
    content_id: string;
    text_content: string;
    tags: string[];
    created_at: Date;
    ttl_seconds: number;
    
    constructor(vector: number[], content_type: string, content_id: string, text_content: string, tags: string[], created_at?: Date, ttl_seconds?: number);
  }
  
  export type TodoziEmbeddingTool = any;
  export type ValidationIssue = any;
  export type ValidationReport = any;

  // Base Tool Types
  export class ErrorType {
    static ValidationError: string;
    static PermissionError: string;
    static FileNotFound: string;
    static TimeoutError: string;
    static ResourceError: string;
    static NetworkError: string;
    static SecurityError: string;
    static InternalError: string;
    static toString(errorType: string): string;
  }
  
  export class ResourceLock {
    static FilesystemWrite: string;
    static FilesystemRead: string;
    static Git: string;
    static Memory: string;
    static Shell: string;
    static Network: string;
    static asStr(lock: string): string;
  }
  
  export abstract class Tool {
    abstract execute(kwargs: Record<string, any>): Promise<ToolResult>;
    abstract definition(): ToolDefinition;
    name(): string;
    validateParameters(kwargs: Record<string, any>): boolean;
  }
  
  export class ToolDefinition {
    name: string;
    description: string;
    parameters: ToolParameter[];
    category: string;
    resource_locks: string[];
    
    constructor(name: string, description: string, parameters: ToolParameter[], category: string, resource_locks: string[]);
    static new(name: string, description: string, parameters: ToolParameter[], category: string, resource_locks: string[]): ToolDefinition;
    toOllamaFormat(): any;
  }
  
  export class ToolError extends Error {
    name: string;
    message: string;
    error_type: string;
    details: Record<string, any>;
    
    constructor(message: string, error_type: string, details?: Record<string, any>);
    static new(message: string, error_type: string, details?: Record<string, any>): ToolError;
    toString(): string;
  }
  
  export class ToolParameter {
    name: string;
    type_: string;
    description: string;
    required: boolean;
    default: any;
    
    constructor(name: string, type_: string, description: string, required: boolean, default_?: any);
    static new(name: string, type_: string, description: string, required: boolean, default_?: any): ToolParameter;
  }
  
  export class ToolRegistry {
    tools: Map<string, Tool>;
    
    constructor();
    register(tool: Tool): void;
    registerCoreTools(): void;
    getTool(name: string): Tool | undefined;
    getAllTools(): Tool[];
    getToolDefinitions(): any[];
    executeTool(tool_name: string, kwargs: Record<string, any>): Promise<ToolResult>;
  }
  
  export class ToolResult {
    success: boolean;
    output: string;
    error: string | null;
    execution_time_ms: number;
    metadata: Record<string, any> | null;
    recovery_context: any | null;
    
    constructor(success: boolean, output: string, error: string | null, execution_time_ms: number, metadata?: Record<string, any> | null, recovery_context?: any | null);
    static new(success: boolean, output: string, error: string | null, execution_time_ms: number, metadata?: Record<string, any> | null, recovery_context?: any | null): ToolResult;
    static success(output: string, execution_time_ms: number): ToolResult;
    static error(error: string, execution_time_ms: number): ToolResult;
    toString(): string;
  }
  export function createToolDefinition(name: string, description: string, category: string, parameters: any[]): any;
  export function createToolDefinitionWithLocks(name: any, description: any, category: any, parameters: any[], resourceLocks: any[]): any;
  export function createToolParameter(name: string, type_: string, description: string, required: boolean): any;
  export function createToolParameterWithDefault(name: string, type_: string, description: string, required: boolean, def: string): any;

  // Additional Model Types
  export type Agent = any;
  export type AgentAssignment = any;
  export type AgentBehaviors = any;
  export type AgentConstraints = any;
  export type AgentMetadata = any;
  export type AgentStatus = any;
  export type AgentTool = any;
  export type AssignmentStatus = any;
  export type CoreEmotion = any;
  export type Error = any;
  export type ErrorCategory = any;
  export type ErrorSeverity = any;
  export type Feeling = any;
  export type IdeaImportance = any;
  export type InvalidPriority = any;
  export type InvalidProgress = any;
  export type InvalidStatus = any;
  export type ItemStatus = any;
  export type MemoryImportance = any;
  export type MemoryTerm = any;
  export type MemoryType = any;
  export type MigrationReport = any;
  export type ProjectMigrationStats = any;
  export type ProjectStats = any;
  export type ProjectStatus = any;
  export type ProjectTaskContainer = any;
  export type QueueSession = any;
  export type QueueStatus = any;
  export type RateLimit = any;
  export type RegistrationInfo = any;
  export type SemanticSearchResult = any;
  export type ShareLevel = any;
  export class Tag {
    id: string;
    name: string;
    description: string | null;
    color: string | null;
    category: string | null;
    usage_count: number;
    created_at: Date;
    updated_at: Date;
    
    constructor(options?: {
      id?: string;
      name: string;
      description?: string | null;
      color?: string | null;
      category?: string | null;
      usage_count?: number;
      created_at?: Date;
      updated_at?: Date;
    });
  }
  
  export type TaskFilters = any;
  export type TaskUpdate = any;
  
  export class TodoziError extends Error {
    name: string;
    type?: string;
    details?: any;
    
    constructor(message: string, type?: string, details?: any);
    static taskNotFound(id: string): TodoziError;
    static projectNotFound(name: string): TodoziError;
    static feelingNotFound(id: string): TodoziError;
    static invalidPriority(priority: string): TodoziError;
    static invalidStatus(status: string): TodoziError;
    static invalidAssignee(assignee: string): TodoziError;
    static invalidProgress(progress: number): TodoziError;
    static validation(message: string): TodoziError;
    static storage(message: string): TodoziError;
    static config(message: string): TodoziError;
    static io(message: string): TodoziError;
    static json(error: Error): TodoziError;
    static uuid(error: Error): TodoziError;
    static chrono(error: Error): TodoziError;
    static dialoguer(error: Error): TodoziError;
    static hlx(error: Error): TodoziError;
    static reqwest(error: Error): TodoziError;
    static dir(message: string): TodoziError;
    static embedding(message: string): TodoziError;
  }
  export type TrainingData = any;
  export type TrainingDataType = any;
  export class ValidationError extends TodoziError {
    constructor(message: string);
  }
  
  export function hashProjectName(projectName: string): string;

  // Reminder Types
  export class Reminder {
    id: string;
    content: string;
    remind_at: Date;
    priority: ReminderPriority;
    status: ReminderStatus;
    tags: string[];
    created_at: Date;
    updated_at: Date;
    
    constructor(content: string, remindAt: Date, priority: ReminderPriority);
    static new(content: string, remindAt: Date, priority: ReminderPriority): Reminder;
    withTags(tags: string[]): Reminder;
    isOverdue(): boolean;
    markCompleted(): void;
    markCancelled(): void;
    activate(): void;
  }
  
  export class ReminderPriority {
    static Low: string;
    static Medium: string;
    static High: string;
    static Critical: string;
    static fromStr(s: string): string;
  }
  
  export class ReminderStatus {
    static Pending: string;
    static Active: string;
    static Completed: string;
    static Cancelled: string;
    static Overdue: string;
    static fromStr(s: string): string;
  }
  
  export class ReminderStatistics {
    total_reminders: number;
    pending_reminders: number;
    active_reminders: number;
    overdue_reminders: number;
    unique_tags: number;
    
    constructor(data: {
      total_reminders: number;
      pending_reminders: number;
      active_reminders: number;
      overdue_reminders: number;
      unique_tags: number;
    });
    pendingPercentage(): number;
    activePercentage(): number;
    overduePercentage(): number;
  }
  
  export class ReminderUpdate {
    content: string | null;
    remind_at: Date | null;
    priority: ReminderPriority | null;
    status: ReminderStatus | null;
    tags: string[] | null;
    
    constructor();
    static new(): ReminderUpdate;
    content(content: string): ReminderUpdate;
    remindAt(remindAt: Date): ReminderUpdate;
    priority(priority: ReminderPriority): ReminderUpdate;
    status(status: ReminderStatus): ReminderUpdate;
    tags(tags: string[]): ReminderUpdate;
  }
  
  export function parseReminderFormat(text: string): Reminder;

  // Summary Types
  export class Summary {
    id: string;
    content: string;
    context: string | null;
    priority: SummaryPriority;
    tags: string[];
    created_at: Date;
    updated_at: Date;
    
    constructor(content: string, priority: SummaryPriority);
    static new(content: string, priority: SummaryPriority): Summary;
    withContext(context: string): Summary;
    withTags(tags: string[]): Summary;
  }
  
  export class SummaryPriority {
    static Low: string;
    static Medium: string;
    static High: string;
    static Critical: string;
    static fromStr(s: string): string;
  }
  
  export class SummaryStatistics {
    total_summaries: number;
    high_priority_summaries: number;
    unique_tags: number;
    
    constructor(totalSummaries: number, highPrioritySummaries: number, uniqueTags: number);
    highPriorityPercentage(): number;
  }
  
  export class SummaryUpdate {
    content: string | undefined;
    context: string | undefined;
    priority: SummaryPriority | undefined;
    tags: string[] | undefined;
    
    constructor();
    static new(): SummaryUpdate;
    setContent(content: string): SummaryUpdate;
    setContext(context: string): SummaryUpdate;
    setPriority(priority: SummaryPriority): SummaryUpdate;
    setTags(tags: string[]): SummaryUpdate;
  }

  // Tag Types
  export class TagSearchQuery {
    name_contains: string | null;
    description_contains: string | null;
    category: string | null;
    color: string | null;
    min_usage: number | null;
    max_usage: number | null;
    sort_by: TagSortBy;
    limit: number | null;
    
    constructor(options?: {
      name_contains?: string | null;
      description_contains?: string | null;
      category?: string | null;
      color?: string | null;
      min_usage?: number | null;
      max_usage?: number | null;
      sort_by?: TagSortBy;
      limit?: number | null;
    });
    static default(): TagSearchQuery;
  }
  
  export class TagSortBy {
    static Name: string;
    static Usage: string;
    static Created: string;
    static Updated: string;
  }
  
  export class TagStatistics {
    total_tags: number;
    total_categories: number;
    total_relationships: number;
    average_usage: number;
    
    constructor(options?: {
      total_tags?: number;
      total_categories?: number;
      total_relationships?: number;
      average_usage?: number;
    });
    relationshipsPerTag(): number;
  }
  
  export class TagUpdate {
    name: string | null;
    description: string | null;
    color: string | null;
    category: string | null;
    
    constructor();
    static new(): TagUpdate;
    name(name: string): TagUpdate;
    description(description: string): TagUpdate;
    color(color: string): TagUpdate;
    category(category: string): TagUpdate;
  }

  // TDZ TLS Types
  export type ChecklistItem = any;
  export type ConversationSession = any;
  export type ExtractedAction = any;
  export type ProcessedAction = any;
  export type ProcessedContent = any;
  export type ProcessingStats = any;
  export type TdzContentProcessorTool = any;
  export type TodoziProcessorState = any;
  export function createTdzContentProcessorTool(state: any): any;
  export function initializeTdzContentProcessor(): any;

  // Todozi Execution Types
  export type ExecutionResult = any;
  export type ExecutorError = any;
  export function ensureTodoziSystem(): any;
  export function executeTodoziToolDelegated(params: any): any;
  export function getEmbeddingService(): any;
  export function getStorage(): any;
  export function getTodoziApiKey(): any;

  // Todozi Tool Types
  export type ChecklistTool = any;
  export type ChunkStatus = any;
  export type ChunkingLevel = any;
  export type CodeChunk = any;
  export type CreateCodeChunkTool = any;
  export type CreateErrorTool = any;
  export type CreateIdeaTool = any;
  export type CreateMemoryTool = any;
  export type CreateTaskTool = any;
  export type PlanTool = any;
  export type ProcessChatMessageTool = any;
  export type SearchTasksTool = any;
  export type SimpleTodoziTool = any;
  export type StrategyTool = any;
  export type TaskExpansionTool = any;
  export type TaskExtractionTool = any;
  export type UnifiedSearchTool = any;
  export type UpdateTaskTool = any;
  export function createTodoziTools(todozi: any): any;
  export function create_tool_parameter(name: string, type_: string, description: string, required: boolean): any;
  export function initializeTodoziSystem(): any;

  // TUI Types
  export type AppTab = any;
  export type ColorScheme = any;
  export type EditSession = any;
  export type MoreTabSection = any;
  export type SortOrder = any;
  export type TaskDisplay = any;
  export type TaskEditor = any;
  export type TaskEvolutionAnalyzer = any;
  export type TaskEvolutionSummary = any;
  export type TaskListDisplay = any;
  export type TaskSortBy = any;
  export type ToastNotification = any;
  export type ToastType = any;
  export type TodoziApp = any;
  export function centeredRect(config: any): any;
  export function createDefaultTaskEditor(): any;
  export function createDefaultTaskEvolutionAnalyzer(): any;
  export function createDefaultTaskEvolutionSummary(): any;
  export function createDefaultTuiService(): any;

  // Types module (for documentation generation)
  export class SearchEngine {
    constructor();
    updateIndex(content: any): void;
    search(query: string, options?: any): any;
  }
  
  // Additional types from types.js (documentation-related)
  export type JSDoc = any;
  export type JavaScript = any;
  export type Note = any;
  export type Type = any;
  export type TypeScript = any;
  export type Types = any;
  
  // These are exported from types.js but use reserved keywords
  // They can be accessed via bracket notation: module['for'], module['this'], etc.
  // Not declaring them here to avoid TypeScript syntax errors

  // Namespaced type aliases (matching index.js exports)
  // Base module
  export type base_ResourceLock = ResourceLock;
  export type base_ToolDefinition = ToolDefinition;
  export type base_ToolResult = ToolResult;
  
  // Embedding module
  export type emb_Priority = Priority;
  export type emb_SimilarityResult = SimilarityResult;
  
  // Error module
  export type error_TodoziError = TodoziError;
  
  // Idea module
  export const idea_parseIdeaFormat: typeof parseIdeaFormat;
  
  // Memory module
  export type memory_MemoryImportance = MemoryImportance;
  export const memory_parseMemoryFormat: typeof parseMemoryFormat;
  
  // Models module
  export type models_Assignee = Assignee;
  export type models_AssignmentStatus = AssignmentStatus;
  export type models_Error = Error;
  export type models_ErrorCategory = ErrorCategory;
  export type models_ErrorSeverity = ErrorSeverity;
  export type models_Idea = Idea;
  export type models_IdeaImportance = IdeaImportance;
  export type models_ItemStatus = ItemStatus;
  export type models_Memory = Memory;
  export type models_MemoryImportance = MemoryImportance;
  export type models_MemoryTerm = MemoryTerm;
  export type models_MemoryType = MemoryType;
  export type models_Priority = Priority;
  export type models_ReminderPriority = ReminderPriority;
  export type models_ReminderStatus = ReminderStatus;
  export type models_ShareLevel = ShareLevel;
  export type models_Status = Status;
  export type models_SummaryPriority = SummaryPriority;
  export type models_Tag = Tag;
  export type models_Task = Task;
  export type models_TaskFilters = TaskFilters;
  export type models_TodoziError = TodoziError;
  export type models_TrainingDataType = TrainingDataType;
  
  // Reminder module
  export type reminder_ReminderPriority = ReminderPriority;
  export type reminder_ReminderStatus = ReminderStatus;
  
  // Storage module
  export type storage_Storage = Storage;
  
  // Summary module
  export type summary_SummaryPriority = SummaryPriority;
  
  // Tags module
  export type tags_Tag = Tag;
  export type tags_TodoziError = TodoziError;
  
  // Todozi module
  export type todozi_Assignee = Assignee;
  export type todozi_AssignmentStatus = AssignmentStatus;
  export type todozi_ErrorCategory = ErrorCategory;
  export type todozi_ErrorSeverity = ErrorSeverity;
  export type todozi_IdeaImportance = IdeaImportance;
  export type todozi_ItemStatus = ItemStatus;
  export type todozi_MemoryImportance = MemoryImportance;
  export type todozi_MemoryTerm = MemoryTerm;
  export type todozi_MemoryType = MemoryType;
  export type todozi_Priority = Priority;
  export type todozi_ShareLevel = ShareLevel;
  export type todozi_Status = Status;
  export type todozi_TodoziError = TodoziError;
  export type todozi_TrainingDataType = TrainingDataType;
  export const todozi_parseIdeaFormat: typeof parseIdeaFormat;
  export const todozi_parseMemoryFormat: typeof parseMemoryFormat;
  
  // Todozi Tool module
  export type todozi_tool_Error = Error;
  export type todozi_tool_ErrorCategory = ErrorCategory;
  export type todozi_tool_ErrorSeverity = ErrorSeverity;
  export type todozi_tool_Idea = Idea;
  export type todozi_tool_IdeaImportance = IdeaImportance;
  export type todozi_tool_ItemStatus = ItemStatus;
  export type todozi_tool_Memory = Memory;
  export type todozi_tool_MemoryImportance = MemoryImportance;
  export type todozi_tool_MemoryTerm = MemoryTerm;
  export type todozi_tool_MemoryType = MemoryType;
  export type todozi_tool_Priority = Priority;
  export type todozi_tool_ResourceLock = ResourceLock;
  export type todozi_tool_ShareLevel = ShareLevel;
  export type todozi_tool_Status = Status;
  export type todozi_tool_Storage = Storage;
  export type todozi_tool_Task = Task;
  export type todozi_tool_ToolDefinition = ToolDefinition;
  export type todozi_tool_ToolResult = ToolResult;
  
  // TUI module
  export type tui_Assignee = Assignee;
  export type tui_Priority = Priority;
  export type tui_SimilarityResult = SimilarityResult;
  export type tui_Status = Status;
  export type tui_Task = Task;
  export type tui_TaskFilters = TaskFilters;
}

// For CommonJS require() usage - matches: require('todozi-javascript/todozi')
// This is the same as the main todozi module
declare module 'todozi-javascript/todozi/index' {
  // Same exports as 'todozi-javascript/todozi' - see that module declaration above
  export * from 'todozi-javascript/todozi';
}

// Export for server - matches: require('todozi-javascript/server') or require('./server')
declare module 'todozi-javascript/server' {
  export class TodoziServer {
    static new(config: ServerConfig): Promise<TodoziServer>;
    start(): Promise<void>;
  }
  
  export class ServerConfig {
    static default(): ServerConfig;
    host: string;
    port: number;
    max_connections: number;
  }
}

// For direct file imports - matches: require('todozi-javascript/todozi/storage')
declare module 'todozi-javascript/todozi/storage' {
  export class Storage {
    static new(): Promise<Storage>;
    listTasksAcrossProjects(filters?: any): Promise<any[]>;
    getTaskFromAnyProject(id: string): any;
    addTaskToProject(task: any): Promise<void>;
    updateTaskInProject(id: string, updates: any): Promise<any>;
    deleteTaskFromProject(id: string): Promise<void>;
    searchTasks(query: string): Promise<any[]>;
    searchTasksSemantic(query: string, limit?: number): Promise<any[]>;
    createProject(name: string, description?: string): Promise<void>;
    getProject(name: string): Promise<any>;
    listProjects(): Promise<any[]>;
  }
  
  export function initStorage(): Promise<void>;
  export function getStorageDir(): string;
  export function checkFolderStructure(): any;
  export function listProjects(): Promise<any[]>;
  export function loadProject(name: string): Promise<any>;
  export function saveProject(project: any): Promise<void>;
  export function loadConfig(): any;
  export function saveConfig(config: any): Promise<void>;
  export function listProjectTaskContainers(): any[];
  export function loadProjectTaskContainer(name: string): any;
  export function saveProjectTaskContainer(container: any): Promise<void>;
}

// Matches: require('todozi-javascript/todozi/models')
declare module 'todozi-javascript/todozi/models' {
  export class Task {
    id: string;
    user_id: string;
    action: string;
    time: string;
    priority: string;
    parent_project: string;
    status: string;
    assignee?: string;
    tags: string[];
    dependencies: string[];
    context_notes?: string;
    progress: number;
    created_at: Date;
    updated_at: Date;
    static new(userId: string, action: string, time: string, priority: any, parentProject: string, status: any): Task;
    static newFull(userId: string, action: string, time: string, priority: any, parentProject: string, status: any, assignee: any, tags: string[], dependencies: string[], contextNotes: string | null, progress: number): Task;
  }
  
  export class Priority {
    static Low: string;
    static Medium: string;
    static High: string;
    static Critical: string;
    static Urgent: string;
    static fromStr(s: string): string;
  }
  
  export class Status {
    static Todo: string;
    static Pending: string;
    static InProgress: string;
    static Blocked: string;
    static Review: string;
    static Done: string;
    static Completed: string;
    static Cancelled: string;
    static Deferred: string;
    static fromStr(s: string): string;
  }
  
  export class Assignee {
    static Ai: string;
    static Human: string;
    static Collaborative: string;
    static fromStr(s: string): string;
  }
  
  export class Project {
    name: string;
    description?: string;
    status: string;
    created_at: Date;
    updated_at: Date;
  }
  
  export class QueueItem {
    id: string;
    task_name: string;
    task_description: string;
    priority: string;
    project_id: string;
    status: string;
    created_at: Date;
    updated_at: Date;
  }
  
  export class Memory {
    id: string;
    moment: string;
    meaning: string;
    reason?: string;
    importance: string;
    term: string;
    tags: string[];
    created_at: Date;
    updated_at: Date;
  }
  
  export class Idea {
    id: string;
    idea: string;
    share: string;
    importance: string;
    tags: string[];
    created_at: Date;
    updated_at: Date;
  }
  
  export class ApiKey {
    userId: string;
    publicKey: string;
    privateKey: string;
    active: boolean;
    createdAt: Date;
    updatedAt: Date;
    
    constructor(options: {
      userId: string;
      publicKey: string;
      privateKey: string;
      active?: boolean;
      createdAt?: Date;
      updatedAt?: Date;
    });
    static new(): ApiKey;
    static withUserId(userId: string): ApiKey;
    deactivate(): void;
    activate(): void;
    isActive(): boolean;
    matches(publicKey: string, privateKey?: string | null): boolean;
    isAdmin(publicKey: string, privateKey: string): boolean;
  }
  
  export class QueueCollection {
    static new(): QueueCollection;
    add_item(item: QueueItem): void;
    get_all_items(): QueueItem[];
    get_items_by_status(status: string): QueueItem[];
    get_active_sessions(): any[];
    start_session(itemId: string): string;
    end_session(sessionId: string): void;
  }
}

// Matches: require('todozi-javascript/todozi/cli')
declare module 'todozi-javascript/todozi/cli' {
  export class TodoziHandler {
    static new(storage: any): Promise<TodoziHandler>;
    handleAddCommand(command: any): Promise<any>;
    handleMemoryCommand(command: any): Promise<any>;
    handleIdeaCommand(command: any): Promise<any>;
    handleListCommand(command: any): Promise<any>;
    handleSearchCommand(command: any): Promise<any>;
    handleUpdateCommand(id: string, updates: any): Promise<any>;
    handleDeleteCommand(id: string): Promise<void>;
  }
}

// Main package entry - matches: require('todozi-javascript')
// This is the main entry point when importing the package
// Since package.json "main" points to "todozi/main.js" (CLI), 
// this provides types for the main SDK exports from todozi/index.js
declare module 'todozi-javascript' {
  // Re-export main classes and functions for convenience
  // These match what's exported from 'todozi-javascript/todozi'
  export {
    Storage,
    Task,
    Priority,
    Status,
    Assignee,
    TodoziHandler,
    AgentManager,
    MemoryManager,
    IdeaManager,
    TodoziEmbeddingService,
    TodoziEmbeddingConfig,
    initStorage,
    processChatMessageExtended,
    processChatMessage
  } from 'todozi-javascript/todozi';
  
  // Re-export server types
  export { TodoziServer, ServerConfig } from 'todozi-javascript/server';
}
