// Utility types and base classes
export class TodoziError extends Error {
    constructor(
        message: string,
        public type?: 
            | 'InvalidPriority' 
            | 'InvalidStatus'
            | 'ValidationError'
            | 'InvalidProgress'
            | 'InvalidQueueStatus'
            | 'InvalidSummaryPriority'
            | 'InvalidReminderPriority'
            | 'InvalidReminderStatus',
        public details?: any
    ) {
        super(message);
        this.name = 'TodoziError';
    }
}

export type Result<T, E = TodoziError> = { ok: true; value: T } | { ok: false; error: E };

// Enums
export enum Priority {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical',
    Urgent = 'urgent'
}

export function parsePriority(s: string): Result<Priority> {
    switch (s.toLowerCase()) {
        case 'low': return Priority.Low;
        case 'medium': return Priority.Medium;
        case 'high': return Priority.High;
        case 'critical': return Priority.Critical;
        case 'urgent': return Priority.Urgent;
        default: return new TodoziError(`Invalid priority: ${s}`, 'InvalidPriority');
    }
}

export enum Status {
    Todo = 'todo',
    Pending = 'pending',
    InProgress = 'in_progress',
    Blocked = 'blocked',
    Review = 'review',
    Done = 'done',
    Completed = 'completed',
    Cancelled = 'cancelled',
    Deferred = 'deferred'
}

export function parseStatus(s: string): Result<Status> {
    switch (s.toLowerCase()) {
        case 'todo':
        case 'pending': return Status.Todo;
        case 'in_progress':
        case 'in-progress': return Status.InProgress;
        case 'blocked': return Status.Blocked;
        case 'review': return Status.Review;
        case 'done':
        case 'completed': return Status.Done;
        case 'cancelled':
        case 'canceled': return Status.Cancelled;
        case 'deferred': return Status.Deferred;
        default: return new TodoziError(`Invalid status: ${s}`, 'InvalidStatus');
    }
}

// Discriminated union for Assignee
export enum AssigneeType {
    Ai = 'ai',
    Human = 'human',
    Collaborative = 'collaborative',
    Agent = 'agent'
}

export type Assignee = 
    | { type: AssigneeType.Ai }
    | { type: AssigneeType.Human }
    | { type: AssigneeType.Collaborative }
    | { type: AssigneeType.Agent; name: string };

export function parseAssignee(s: string): Result<Assignee> {
    const lower = s.toLowerCase();
    if (lower === 'ai') return { type: AssigneeType.Ai };
    if (lower === 'human') return { type: AssigneeType.Human };
    if (lower === 'collaborative') return { type: AssigneeType.Collaborative };
    
    if (lower.startsWith('agent:')) {
        return { type: AssigneeType.Agent, name: lower.substring(6) };
    }
    
    return { type: AssigneeType.Agent, name: s };
}

export function formatAssignee(assignee: Assignee): string {
    switch (assignee.type) {
        case AssigneeType.Ai: return 'ai';
        case AssigneeType.Human: return 'human';
        case AssigneeType.Collaborative: return 'collaborative';
        case AssigneeType.Agent: return `agent:${assignee.name}`;
    }
}

// Memory related enums
export enum MemoryImportance {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical'
}

export function parseMemoryImportance(s: string): Result<MemoryImportance> {
    switch (s.toLowerCase()) {
        case 'low': return MemoryImportance.Low;
        case 'medium': return MemoryImportance.Medium;
        case 'high': return MemoryImportance.High;
        case 'critical': return MemoryImportance.Critical;
        default: return new TodoziError(`Invalid memory importance: ${s}`, 'ValidationError');
    }
}

export enum MemoryTerm {
    Short = 'short',
    Long = 'long'
}

export function parseMemoryTerm(s: string): Result<MemoryTerm> {
    switch (s.toLowerCase()) {
        case 'short': return MemoryTerm.Short;
        case 'long': return MemoryTerm.Long;
        default: return new TodoziError(`Invalid memory term: ${s}`, 'ValidationError');
    }
}

export enum MemoryType {
    Standard = 'standard',
    Secret = 'secret',
    Human = 'human',
    Short = 'short',
    Long = 'long',
    Emotional = 'emotional'
}

export function parseMemoryType(s: string): Result<MemoryType> {
    const lower = s.toLowerCase();
    switch (lower) {
        case 'standard': return MemoryType.Standard;
        case 'secret': return MemoryType.Secret;
        case 'human': return MemoryType.Human;
        case 'short': return MemoryType.Short;
        case 'long': return MemoryType.Long;
        default:
            // Check if it's a valid emotion
            try {
                parseCoreEmotion(lower);
                return MemoryType.Emotional;
            } catch {
                return new TodoziError(`Invalid memory type: ${s}`, 'ValidationError');
            }
    }
}

export enum CoreEmotion {
    Happy = 'happy',
    Sad = 'sad',
    Angry = 'angry',
    Fearful = 'fearful',
    Surprised = 'surprised',
    Disgusted = 'disgusted',
    Excited = 'excited',
    Anxious = 'anxious',
    Confident = 'confident',
    Frustrated = 'frustrated',
    Motivated = 'motivated',
    Overwhelmed = 'overwhelmed',
    Curious = 'curious',
    Satisfied = 'satisfied',
    Disappointed = 'disappointed',
    Grateful = 'grateful',
    Proud = 'proud',
    Ashamed = 'ashamed',
    Hopeful = 'hopeful',
    Resigned = 'resigned'
}

export function parseCoreEmotion(s: string): Result<CoreEmotion> {
    switch (s.toLowerCase()) {
        case 'happy': return CoreEmotion.Happy;
        case 'sad': return CoreEmotion.Sad;
        case 'angry': return CoreEmotion.Angry;
        case 'fearful': return CoreEmotion.Fearful;
        case 'surprised': return CoreEmotion.Surprised;
        case 'disgusted': return CoreEmotion.Disgusted;
        case 'excited': return CoreEmotion.Excited;
        case 'anxious': return CoreEmotion.Anxious;
        case 'confident': return CoreEmotion.Confident;
        case 'frustrated': return CoreEmotion.Frustrated;
        case 'motivated': return CoreEmotion.Motivated;
        case 'overwhelmed': return CoreEmotion.Overwhelmed;
        case 'curious': return CoreEmotion.Curious;
        case 'satisfied': return CoreEmotion.Satisfied;
        case 'disappointed': return CoreEmotion.Disappointed;
        case 'grateful': return CoreEmotion.Grateful;
        case 'proud': return CoreEmotion.Proud;
        case 'ashamed': return CoreEmotion.Ashamed;
        case 'hopeful': return CoreEmotion.Hopeful;
        case 'resigned': return CoreEmotion.Resigned;
        default: return new TodoziError(`Invalid core emotion: ${s}`, 'ValidationError');
    }
}

export enum ShareLevel {
    Private = 'private',
    Team = 'team',
    Public = 'public'
}

export function parseShareLevel(s: string): Result<ShareLevel> {
    switch (s.toLowerCase()) {
        case 'private': return ShareLevel.Private;
        case 'team': return ShareLevel.Team;
        case 'public': return ShareLevel.Public;
        default: return new TodoziError(`Invalid share level: ${s}`, 'ValidationError');
    }
}

export enum IdeaImportance {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Breakthrough = 'breakthrough'
}

export function parseIdeaImportance(s: string): Result<IdeaImportance> {
    switch (s.toLowerCase()) {
        case 'low': return IdeaImportance.Low;
        case 'medium': return IdeaImportance.Medium;
        case 'high': return IdeaImportance.High;
        case 'breakthrough': return IdeaImportance.Breakthrough;
        default: return new TodoziError(`Invalid idea importance: ${s}`, 'ValidationError');
    }
}

export enum ItemStatus {
    Active = 'active',
    Archived = 'archived',
    Deleted = 'deleted'
}

export enum ErrorSeverity {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical'
}

export function parseErrorSeverity(s: string): Result<ErrorSeverity> {
    switch (s.toLowerCase()) {
        case 'low': return ErrorSeverity.Low;
        case 'medium': return ErrorSeverity.Medium;
        case 'high': return ErrorSeverity.High;
        case 'critical': return ErrorSeverity.Critical;
        default: return new TodoziError(`Invalid error severity: ${s}`, 'ValidationError');
    }
}

export enum ErrorCategory {
    Network = 'network',
    Database = 'database',
    Authentication = 'authentication',
    Authorization = 'authorization',
    Validation = 'validation',
    Performance = 'performance',
    Security = 'security',
    Integration = 'integration',
    Configuration = 'configuration',
    Runtime = 'runtime',
    Compilation = 'compilation',
    Dependency = 'dependency',
    UserError = 'user_error',
    SystemError = 'system_error'
}

export function parseErrorCategory(s: string): Result<ErrorCategory> {
    switch (s.toLowerCase()) {
        case 'network': return ErrorCategory.Network;
        case 'database': return ErrorCategory.Database;
        case 'authentication': return ErrorCategory.Authentication;
        case 'authorization': return ErrorCategory.Authorization;
        case 'validation': return ErrorCategory.Validation;
        case 'performance': return ErrorCategory.Performance;
        case 'security': return ErrorCategory.Security;
        case 'integration': return ErrorCategory.Integration;
        case 'configuration': return ErrorCategory.Configuration;
        case 'runtime': return ErrorCategory.Runtime;
        case 'compilation': return ErrorCategory.Compilation;
        case 'dependency': return ErrorCategory.Dependency;
        case 'usererror':
        case 'user_error': return ErrorCategory.UserError;
        case 'systemerror':
        case 'system_error': return ErrorCategory.SystemError;
        default: return new TodoziError(`Invalid error category: ${s}`, 'ValidationError');
    }
}

export enum TrainingDataType {
    Instruction = 'instruction',
    Completion = 'completion',
    Conversation = 'conversation',
    Code = 'code',
    Analysis = 'analysis',
    Planning = 'planning',
    Review = 'review',
    Documentation = 'documentation',
    Example = 'example',
    Test = 'test',
    Validation = 'validation'
}

export function parseTrainingDataType(s: string): Result<TrainingDataType> {
    switch (s.toLowerCase()) {
        case 'instruction': return TrainingDataType.Instruction;
        case 'completion': return TrainingDataType.Completion;
        case 'conversation': return TrainingDataType.Conversation;
        case 'code': return TrainingDataType.Code;
        case 'analysis': return TrainingDataType.Analysis;
        case 'planning': return TrainingDataType.Planning;
        case 'review': return TrainingDataType.Review;
        case 'documentation': return TrainingDataType.Documentation;
        case 'example': return TrainingDataType.Example;
        case 'test': return TrainingDataType.Test;
        case 'validation': return TrainingDataType.Validation;
        default: return new TodoziError(`Invalid training data type: ${s}`, 'ValidationError');
    }
}

// Base interfaces
export interface Task {
    id: string;
    userId: string;
    action: string;
    time: string;
    priority: Priority;
    parentProject: string;
    status: Status;
    assignee?: Assignee;
    tags: string[];
    dependencies: string[];
    contextNotes?: string;
    progress?: number;
    embeddingVector?: number[];
    createdAt: Date;
    updatedAt: Date;
}

// Builder pattern for TaskUpdate
export interface TaskUpdate {
    action?: string;
    time?: string;
    priority?: Priority;
    parentProject?: string;
    status?: Status;
    assignee?: Assignee;
    tags?: string[];
    dependencies?: string[];
    contextNotes?: string;
    progress?: number;
    embeddingVector?: number[];
}

export class TaskUpdateBuilder {
    private updates: Partial<TaskUpdate> = {};

    withAction(action: string): this {
        this.updates.action = action;
        return this;
    }

    withTime(time: string): this {
        this.updates.time = time;
        return this;
    }

    withPriority(priority: Priority): this {
        this.updates.priority = priority;
        return this;
    }

    withParentProject(parentProject: string): this {
        this.updates.parentProject = parentProject;
        return this;
    }

    withStatus(status: Status): this {
        this.updates.status = status;
        return this;
    }

    withAssignee(assignee: Assignee): this {
        this.updates.assignee = assignee;
        return this;
    }

    withTags(tags: string[]): this {
        this.updates.tags = tags;
        return this;
    }

    withDependencies(dependencies: string[]): this {
        this.updates.dependencies = dependencies;
        return this;
    }

    withContextNotes(contextNotes: string): this {
        this.updates.contextNotes = contextNotes;
        return this;
    }

    withProgress(progress: number): this {
        this.updates.progress = progress;
        return this;
    }

    build(): TaskUpdate {
        return { ...this.updates };
    }
}

export interface TaskFilters {
    project?: string;
    status?: Status;
    priority?: Priority;
    assignee?: Assignee;
    tags?: string[];
    search?: string;
}

export class TaskClass implements Task {
    constructor(
        public id: string,
        public userId: string,
        public action: string,
        public time: string,
        public priority: Priority,
        public parentProject: string,
        public status: Status,
        public assignee?: Assignee,
        public tags: string[] = [],
        public dependencies: string[] = [],
        public contextNotes?: string,
        public progress?: number,
        public embeddingVector?: number[],
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    static new(
        userId: string,
        action: string,
        time: string,
        priority: Priority,
        parentProject: string,
        status: Status
    ): TaskClass {
        const now = new Date();
        const id = `task_${generateUUID8()}`;
        return new TaskClass(
            id, userId, action, time, priority,
            parentProject, status
        );
    }

    static newFull(
        userId: string,
        action: string,
        time: string,
        priority: Priority,
        parentProject: string,
        status: Status,
        assignee?: Assignee,
        tags: string[] = [],
        dependencies: string[] = [],
        contextNotes?: string,
        progress?: number
    ): Result<TaskClass> {
        if (progress !== undefined && progress > 100) {
            return new TodoziError(`Invalid progress: ${progress}`, 'InvalidProgress');
        }

        const now = new Date();
        const id = `task_${generateUUID8()}`;
        return new TaskClass(
            id, userId, action, time, priority, parentProject, status,
            assignee, tags, dependencies, contextNotes, progress
        );
    }

    update(updates: TaskUpdate): Result<void> {
        if (updates.progress !== undefined && updates.progress > 100) {
            return new TodoziError(`Invalid progress: ${updates.progress}`, 'InvalidProgress');
        }

        Object.assign(this, updates);
        this.updatedAt = new Date();
        return undefined;
    }

    complete(): void {
        this.status = Status.Done;
        this.progress = 100;
        this.updatedAt = new Date();
    }

    isCompleted(): boolean {
        return this.status === Status.Done;
    }

    isActive(): boolean {
        return !([Status.Done, Status.Cancelled].includes(this.status));
    }
}

// Project related types
export enum ProjectStatus {
    Active = 'active',
    Archived = 'archived',
    Completed = 'completed'
}

export function parseProjectStatus(s: string): Result<ProjectStatus> {
    switch (s.toLowerCase()) {
        case 'active': return ProjectStatus.Active;
        case 'archived': return ProjectStatus.Archived;
        case 'completed': return ProjectStatus.Completed;
        default: return new TodoziError(`Invalid project status: ${s}`, 'ValidationError');
    }
}

export interface Project {
    name: string;
    description?: string;
    createdAt: Date;
    updatedAt: Date;
    status: ProjectStatus;
    tasks: string[];
}

export class ProjectClass implements Project {
    constructor(
        public name: string,
        public description?: string,
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date(),
        public status: ProjectStatus = ProjectStatus.Active,
        public tasks: string[] = []
    ) {}

    static new(name: string, description?: string): ProjectClass {
        const now = new Date();
        return new ProjectClass(name, description, now, now);
    }

    addTask(taskId: string): void {
        if (!this.tasks.includes(taskId)) {
            this.tasks.push(taskId);
            this.updatedAt = new Date();
        }
    }

    removeTask(taskId: string): void {
        this.tasks = this.tasks.filter(id => id !== taskId);
        this.updatedAt = new Date();
    }

    archive(): void {
        this.status = ProjectStatus.Archived;
        this.updatedAt = new Date();
    }

    complete(): void {
        this.status = ProjectStatus.Completed;
        this.updatedAt = new Date();
    }
}

// Configuration
export interface Config {
    registration?: RegistrationInfo;
    version: string;
    defaultProject: string;
    autoBackup: boolean;
    backupInterval: string;
    aiEnabled: boolean;
    defaultAssignee?: Assignee;
    dateFormat: string;
    timezone: string;
}

export interface RegistrationInfo {
    userName: string;
    userEmail: string;
    apiKey: string;
    userId?: string;
    fingerprint?: string;
    registeredAt: Date;
    serverUrl: string;
}

export class ConfigClass implements Config {
    constructor(
        public version: string = '1.2.0',
        public defaultProject: string = 'general',
        public autoBackup: boolean = true,
        public backupInterval: string = 'daily',
        public aiEnabled: boolean = true,
        public defaultAssignee?: Assignee,
        public dateFormat: string = '%Y-%m-%d %H:%M:%S',
        public timezone: string = 'UTC',
        public registration?: RegistrationInfo
    ) {}
}

export class RegistrationInfoClass implements RegistrationInfo {
    constructor(
        public userName: string,
        public userEmail: string,
        public apiKey: string,
        public serverUrl: string,
        public userId?: string,
        public fingerprint?: string,
        public registeredAt: Date = new Date()
    ) {}

    static new(
        userName: string,
        userEmail: string,
        apiKey: string,
        serverUrl: string
    ): RegistrationInfoClass {
        return new RegistrationInfoClass(userName, userEmail, apiKey, serverUrl);
    }

    static newWithHashes(serverUrl: string): RegistrationInfoClass {
        const userId = `user_${generateUUID8()}`;
        const emailHash = `hash_${generateUUID8()}@example.com`;
        return new RegistrationInfoClass(userId, emailHash, '', serverUrl);
    }
}

// Task Collection
export class TaskCollection {
    constructor(
        public version: string = '1.2.0',
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date(),
        private tasksMap: Map<string, Task> = new Map()
    ) {}

    get tasks(): ReadonlyMap<string, Task> {
        return this.tasksMap;
    }

    addTask(task: Task): void {
        this.tasksMap.set(task.id, task);
        this.updatedAt = new Date();
    }

    getTask(id: string): Readonly<Task> | undefined {
        const task = this.tasksMap.get(id);
        return task ? Object.freeze({ ...task }) : undefined;
    }

    getTaskMut(id: string): Task | undefined {
        return this.tasksMap.get(id);
    }

    removeTask(id: string): Task | undefined {
        const task = this.tasksMap.get(id);
        if (task) {
            this.tasksMap.delete(id);
            this.updatedAt = new Date();
            return task;
        }
        return undefined;
    }

    getAllTasks(): Task[] {
        return Array.from(this.tasksMap.values());
    }

    getFilteredTasks(filters: TaskFilters): Task[] {
        return Array.from(this.tasksMap.values()).filter(task => {
            if (filters.project && task.parentProject !== filters.project) return false;
            if (filters.status && task.status !== filters.status) return false;
            if (filters.priority && task.priority !== filters.priority) return false;
            if (filters.assignee) {
                if (!task.assignee || !this.assigneeEquals(task.assignee, filters.assignee)) {
                    return false;
                }
            }
            if (filters.tags && !filters.tags.some(tag => task.tags.includes(tag))) return false;
            if (filters.search) {
                const searchLower = filters.search.toLowerCase();
                if (!task.action.toLowerCase().includes(searchLower) &&
                    !task.contextNotes?.toLowerCase().includes(searchLower)) {
                    return false;
                }
            }
            return true;
        });
    }

    private assigneeEquals(a1: Assignee, a2: Assignee): boolean {
        if (a1.type !== a2.type) return false;
        if (a1.type === AssigneeType.Agent && a2.type === AssigneeType.Agent) {
            return a1.name === a2.name;
        }
        return true;
    }
}

// Memory structure
export interface Memory {
    id: string;
    userId: string;
    projectId?: string;
    status: ItemStatus;
    moment: string;
    meaning: string;
    reason: string;
    importance: MemoryImportance;
    term: MemoryTerm;
    memoryType: MemoryType;
    tags: string[];
    createdAt: Date;
    updatedAt: Date;
}

// Agent related types
export interface ModelConfig {
    provider: string;
    name: string;
    temperature: number;
    maxTokens: number;
}

export interface AgentTool {
    name: string;
    enabled: boolean;
    config?: any;
}

export interface AgentBehaviors {
    autoFormatCode: boolean;
    includeExamples: boolean;
    explainComplexity: boolean;
    suggestTests: boolean;
}

export interface AgentConstraints {
    maxResponseLength?: number;
    timeoutSeconds?: number;
    rateLimit?: RateLimit;
}

export interface RateLimit {
    requestsPerMinute?: number;
    tokensPerHour?: number;
}

export interface AgentMetadata {
    author: string;
    tags: string[];
    category: string;
    status: AgentStatus;
}

export enum AgentStatus {
    Active = 'active',
    Inactive = 'inactive',
    Busy = 'busy',
    Available = 'available'
}

export interface Agent {
    id: string;
    name: string;
    description: string;
    version: string;
    model: ModelConfig;
    systemPrompt: string;
    promptTemplate?: string;
    capabilities: string[];
    specializations: string[];
    tools: AgentTool[];
    behaviors: AgentBehaviors;
    constraints: AgentConstraints;
    metadata: AgentMetadata;
    createdAt: Date;
    updatedAt: Date;
}

export class AgentClass implements Agent {
    constructor(
        public id: string,
        public name: string,
        public description: string,
        public version: string = '1.0.0',
        public model: ModelConfig = {
            provider: 'anthropic',
            name: 'claude-3-opus-20240229',
            temperature: 0.2,
            maxTokens: 4096
        },
        public systemPrompt: string = '',
        public promptTemplate?: string,
        public capabilities: string[] = [],
        public specializations: string[] = [],
        public tools: AgentTool[] = [],
        public behaviors: AgentBehaviors = {
            autoFormatCode: true,
            includeExamples: true,
            explainComplexity: true,
            suggestTests: true
        },
        public constraints: AgentConstraints = {
            maxResponseLength: 10000,
            timeoutSeconds: 300,
            rateLimit: {
                requestsPerMinute: 10,
                tokensPerHour: 100000
            }
        },
        public metadata: AgentMetadata = {
            author: 'system',
            tags: ['ai', 'assistant'],
            category: 'general',
            status: AgentStatus.Available
        },
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {
        this.systemPrompt = this.systemPrompt || 
            `You are ${this.id}, an AI assistant specialized in ${this.description}.`;
    }

    static new(id: string, name: string, description: string): AgentClass {
        return new AgentClass(id, name, description);
    }

    static createCoder(): AgentClass {
        const agent = new AgentClass(
            'coder',
            'Coder',
            'Software development and programming specialist'
        );

        agent.systemPrompt = 'You are an expert software developer with deep knowledge of multiple programming languages and best practices. Your role is to:\n- Write clean, efficient, and well-documented code\n- Follow language-specific conventions and idioms\n- Consider security, performance, and maintainability\n- Provide clear explanations of your code and decisions\n- Suggest improvements and alternatives when appropriate';

        agent.promptTemplate = 'Task: {task}\nLanguage: {language}\nContext: {context}\n\nRequirements:\n{requirements}\n\nPlease provide a solution with explanations.';

        agent.capabilities = [
            'code_development', 'code_review', 'debugging', 'refactoring',
            'testing', 'documentation', 'architecture_design'
        ];

        agent.specializations = [
            'rust', 'python', 'javascript', 'typescript',
            'go', 'sql', 'docker'
        ];

        agent.tools = [
            { name: 'code_executor', enabled: true },
            { name: 'linter', enabled: true },
            { name: 'test_runner', enabled: true }
        ];

        agent.metadata.tags = ['development', 'programming', 'technical'];
        agent.metadata.category = 'technical';

        return agent;
    }

    hasCapability(capability: string): boolean {
        return this.capabilities.includes(capability);
    }

    hasSpecialization(specialization: string): boolean {
        return this.specializations.includes(specialization);
    }

    hasTool(toolName: string): boolean {
        return this.tools.some(t => t.name === toolName && t.enabled);
    }

    getEnabledTools(): AgentTool[] {
        return this.tools.filter(t => t.enabled);
    }

    setStatus(status: AgentStatus): void {
        this.metadata.status = status;
        this.updatedAt = new Date();
    }

    isAvailable(): boolean {
        return this.metadata.status === AgentStatus.Available;
    }

    getFormattedPrompt(variables: Record<string, string>): string {
        let prompt = this.systemPrompt;
        if (this.promptTemplate) {
            let formatted = this.promptTemplate;
            for (const [key, value] of Object.entries(variables)) {
                formatted = formatted.replace(new RegExp(`\\{${key}\\}`, 'g'), value);
            }
            prompt += '\n\n' + formatted;
        }
        return prompt;
    }
}

// Idea structure
export interface Idea {
    id: string;
    idea: string;
    projectId?: string;
    status: ItemStatus;
    share: ShareLevel;
    importance: IdeaImportance;
    tags: string[];
    context?: string;
    createdAt: Date;
    updatedAt: Date;
}

// Assignment types
export interface AgentAssignment {
    agentId: string;
    taskId: string;
    projectId: string;
    assignedAt: Date;
    status: AssignmentStatus;
}

export enum AssignmentStatus {
    Assigned = 'assigned',
    Accepted = 'accepted',
    InProgress = 'in_progress',
    Completed = 'completed',
    Rejected = 'rejected'
}

// Error structure
export interface ErrorRecord {
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
    createdAt: Date;
    updatedAt: Date;
    resolvedAt?: Date;
}

export class ErrorRecordClass implements ErrorRecord {
    constructor(
        public id: string = generateUUID(),
        public title: string,
        public description: string,
        public severity: ErrorSeverity = ErrorSeverity.Medium,
        public category: ErrorCategory = ErrorCategory.Runtime,
        public source: string,
        public context?: string,
        public tags: string[] = [],
        public resolved: boolean = false,
        public resolution?: string,
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date(),
        public resolvedAt?: Date
    ) {}

    static new(title: string, description: string, source: string): ErrorRecordClass {
        return new ErrorRecordClass(undefined, title, description, undefined, undefined, source);
    }
}

// Training data
export interface TrainingData {
    id: string;
    dataType: TrainingDataType;
    prompt: string;
    completion: string;
    context?: string;
    tags: string[];
    qualityScore?: number;
    source: string;
    createdAt: Date;
    updatedAt: Date;
}

export class TrainingDataClass implements TrainingData {
    constructor(
        public id: string = generateUUID(),
        public dataType: TrainingDataType,
        public prompt: string,
        public completion: string,
        public context?: string,
        public tags: string[] = [],
        public qualityScore?: number,
        public source: string,
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    static new(
        dataType: string,
        prompt: string,
        completion: string,
        source: string
    ): Result<TrainingDataClass> {
        const parsedType = parseTrainingDataType(dataType);
        if (parsedType instanceof TodoziError) return parsedType;
        
        return new TrainingDataClass(undefined, parsedType, prompt, completion, undefined, undefined, undefined, source);
    }
}

// Feeling structure
export interface Feeling {
    id: string;
    emotion: string;
    intensity: number;
    description: string;
    context: string;
    tags: string[];
    createdAt: Date;
    updatedAt: Date;
}

// Tag structure
export interface Tag {
    id: string;
    name: string;
    description?: string;
    color?: string;
    category?: string;
    usageCount: number;
    createdAt: Date;
    updatedAt: Date;
}

// Queue related types
export enum QueueStatus {
    Backlog = 'backlog',
    Active = 'active',
    Complete = 'complete'
}

export function parseQueueStatus(s: string): Result<QueueStatus> {
    switch (s.toLowerCase()) {
        case 'backlog': return QueueStatus.Backlog;
        case 'active': return QueueStatus.Active;
        case 'complete': return QueueStatus.Complete;
        default: return new TodoziError(`Invalid queue status: ${s}`, 'InvalidQueueStatus');
    }
}

export interface QueueItem {
    id: string;
    taskName: string;
    taskDescription: string;
    priority: Priority;
    projectId?: string;
    status: QueueStatus;
    createdAt: Date;
    updatedAt: Date;
}

export class QueueItemClass implements QueueItem {
    constructor(
        public id: string = `queue_${generateUUID8()}`,
        public taskName: string,
        public taskDescription: string,
        public priority: Priority,
        public projectId?: string,
        public status: QueueStatus = QueueStatus.Backlog,
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    start(): void {
        this.status = QueueStatus.Active;
        this.updatedAt = new Date();
    }

    complete(): void {
        this.status = QueueStatus.Complete;
        this.updatedAt = new Date();
    }

    isBacklog(): boolean {
        return this.status === QueueStatus.Backlog;
    }

    isActive(): boolean {
        return this.status === QueueStatus.Active;
    }

    isComplete(): boolean {
        return this.status === QueueStatus.Complete;
    }
}

export interface QueueSession {
    id: string;
    queueItemId: string;
    startTime: Date;
    endTime?: Date;
    durationSeconds?: number;
    createdAt: Date;
    updatedAt: Date;
}

export class QueueSessionClass implements QueueSession {
    constructor(
        public id: string = `session_${generateUUID8()}`,
        public queueItemId: string,
        public startTime: Date = new Date(),
        public endTime?: Date,
        public durationSeconds?: number,
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    end(): void {
        this.endTime = new Date();
        this.durationSeconds = Math.floor((this.endTime.getTime() - this.startTime.getTime()) / 1000);
        this.updatedAt = this.endTime;
    }

    isActive(): boolean {
        return this.endTime === undefined;
    }

    getCurrentDuration(): number {
        if (this.isActive()) {
            return Math.floor((new Date().getTime() - this.startTime.getTime()) / 1000);
        }
        return this.durationSeconds || 0;
    }
}

export class QueueCollection {
    constructor(
        public version: string = '1.0.0',
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date(),
        private itemsMap: Map<string, QueueItem> = new Map(),
        private sessionsMap: Map<string, QueueSession> = new Map()
    ) {}

    get items(): ReadonlyMap<string, QueueItem> {
        return this.itemsMap;
    }

    get sessions(): ReadonlyMap<string, QueueSession> {
        return this.sessionsMap;
    }

    addItem(item: QueueItem): void {
        this.itemsMap.set(item.id, item);
        this.updatedAt = new Date();
    }

    getItem(id: string): QueueItem | undefined {
        return this.itemsMap.get(id);
    }

    getItemMut(id: string): QueueItem | undefined {
        return this.itemsMap.get(id);
    }

    removeItem(id: string): QueueItem | undefined {
        const item = this.itemsMap.get(id);
        if (item) {
            this.itemsMap.delete(id);
            this.updatedAt = new Date();
            return item;
        }
        return undefined;
    }

    getAllItems(): QueueItem[] {
        return Array.from(this.itemsMap.values());
    }

    getItemsByStatus(status: QueueStatus): QueueItem[] {
        return Array.from(this.itemsMap.values()).filter(item => item.status === status);
    }

    getBacklogItems(): QueueItem[] {
        return this.getItemsByStatus(QueueStatus.Backlog);
    }

    getActiveItems(): QueueItem[] {
        return this.getItemsByStatus(QueueStatus.Active);
    }

    getCompleteItems(): QueueItem[] {
        return this.getItemsByStatus(QueueStatus.Complete);
    }

    startSession(queueItemId: string): Result<string> {
        const item = this.itemsMap.get(queueItemId);
        if (!item) {
            return new TodoziError('Queue item not found', 'ValidationError');
        }
        
        if (!item.isBacklog()) {
            return new TodoziError('Item is not in backlog status', 'ValidationError');
        }

        const session = new QueueSessionClass(undefined, queueItemId);
        this.sessionsMap.set(session.id, session);

        const itemMut = this.itemsMap.get(queueItemId);
        if (itemMut) {
            (itemMut as QueueItemClass).start();
        }

        this.updatedAt = new Date();
        return session.id;
    }

    endSession(sessionId: string): Result<void> {
        const session = this.sessionsMap.get(sessionId);
        if (!session) {
            return new TodoziError('Session not found', 'ValidationError');
        }

        if (!session.isActive()) {
            return new TodoziError('Session is already ended', 'ValidationError');
        }

        (session as QueueSessionClass).end();

        const item = this.itemsMap.get(session.queueItemId);
        if (item) {
            (item as QueueItemClass).complete();
        }

        this.updatedAt = new Date();
        return undefined;
    }

    getActiveSessions(): QueueSession[] {
        return Array.from(this.sessionsMap.values()).filter(s => s.isActive());
    }

    getSession(id: string): QueueSession | undefined {
        return this.sessionsMap.get(id);
    }
}

// API Key types
export interface ApiKey {
    userId: string;
    publicKey: string;
    privateKey: string;
    active: boolean;
    createdAt: Date;
    updatedAt: Date;
}

export class ApiKeyClass implements ApiKey {
    constructor(
        public userId: string,
        public publicKey: string,
        public privateKey: string,
        public active: boolean = true,
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    static new(): ApiKeyClass {
        const now = new Date();
        const userId = `user_${generateUUID8()}`;
        const timeStr = now.getTime().toString();
        const mtRand = Math.random().toString(36).substring(2);
        const randStr = Math.random().toString(36).substring(2);
        const input = `${timeStr}${mtRand}${randStr}`;
        const publicKey = hashString(input);
        const privateKey = hashString(publicKey);

        return new ApiKeyClass(userId, publicKey, privateKey);
    }

    static withUserId(userId: string): ApiKeyClass {
        const now = new Date();
        const timeStr = now.getTime().toString();
        const mtRand = Math.random().toString(36).substring(2);
        const randStr = Math.random().toString(36).substring(2);
        const input = `${timeStr}${mtRand}${randStr}`;
        const publicKey = hashString(input);
        const privateKey = hashString(publicKey);

        return new ApiKeyClass(userId, publicKey, privateKey);
    }

    deactivate(): void {
        this.active = false;
        this.updatedAt = new Date();
    }

    activate(): void {
        this.active = true;
        this.updatedAt = new Date();
    }

    isActive(): boolean {
        return this.active;
    }

    matches(publicKey: string, privateKey?: string): boolean {
        if (!this.isActive() || this.publicKey !== publicKey) {
            return false;
        }
        if (privateKey !== undefined) {
            return this.privateKey === privateKey;
        }
        return true;
    }

    isAdmin(publicKey: string, privateKey: string): boolean {
        return this.matches(publicKey, privateKey);
    }
}

export class ApiKeyCollection {
    constructor(
        public version: string = '1.0.0',
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date(),
        private keysMap: Map<string, ApiKey> = new Map()
    ) {}

    get keys(): ReadonlyMap<string, ApiKey> {
        return this.keysMap;
    }

    addKey(key: ApiKey): void {
        this.keysMap.set(key.userId, key);
        this.updatedAt = new Date();
    }

    getKey(userId: string): ApiKey | undefined {
        return this.keysMap.get(userId);
    }

    getKeyByPublic(publicKey: string): ApiKey | undefined {
        return Array.from(this.keysMap.values()).find(key => key.publicKey === publicKey);
    }

    getAllKeys(): ApiKey[] {
        return Array.from(this.keysMap.values());
    }

    getActiveKeys(): ApiKey[] {
        return Array.from(this.keysMap.values()).filter(key => key.isActive());
    }

    removeKey(userId: string): ApiKey | undefined {
        const key = this.keysMap.get(userId);
        if (key) {
            this.keysMap.delete(userId);
            this.updatedAt = new Date();
            return key;
        }
        return undefined;
    }

    deactivateKey(userId: string): boolean {
        const key = this.keysMap.get(userId);
        if (key) {
            (key as ApiKeyClass).deactivate();
            this.updatedAt = new Date();
            return true;
        }
        return false;
    }

    activateKey(userId: string): boolean {
        const key = this.keysMap.get(userId);
        if (key) {
            (key as ApiKeyClass).activate();
            this.updatedAt = new Date();
            return true;
        }
        return false;
    }
}

// Summary and Reminder types
export enum SummaryPriority {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical'
}

export function parseSummaryPriority(s: string): Result<SummaryPriority> {
    switch (s.toLowerCase()) {
        case 'low': return SummaryPriority.Low;
        case 'medium': return SummaryPriority.Medium;
        case 'high': return SummaryPriority.High;
        case 'critical': return SummaryPriority.Critical;
        default: return new TodoziError(`Invalid summary priority: ${s}`, 'InvalidSummaryPriority');
    }
}

export interface Summary {
    id: string;
    content: string;
    context?: string;
    priority: SummaryPriority;
    tags: string[];
    createdAt: Date;
    updatedAt: Date;
}

export class SummaryClass implements Summary {
    constructor(
        public id: string = generateUUID(),
        public content: string,
        public context?: string,
        public priority: SummaryPriority,
        public tags: string[] = [],
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    static new(content: string, priority: SummaryPriority): SummaryClass {
        return new SummaryClass(undefined, content, undefined, priority);
    }

    withContext(context: string): SummaryClass {
        this.context = context;
        this.updatedAt = new Date();
        return this;
    }

    withTags(tags: string[]): SummaryClass {
        this.tags = tags;
        this.updatedAt = new Date();
        return this;
    }
}

export enum ReminderPriority {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical'
}

export function parseReminderPriority(s: string): Result<ReminderPriority> {
    switch (s.toLowerCase()) {
        case 'low': return ReminderPriority.Low;
        case 'medium': return ReminderPriority.Medium;
        case 'high': return ReminderPriority.High;
        case 'critical': return ReminderPriority.Critical;
        default: return new TodoziError(`Invalid reminder priority: ${s}`, 'InvalidReminderPriority');
    }
}

export enum ReminderStatus {
    Pending = 'pending',
    Active = 'active',
    Completed = 'completed',
    Cancelled = 'cancelled',
    Overdue = 'overdue'
}

export function parseReminderStatus(s: string): Result<ReminderStatus> {
    switch (s.toLowerCase()) {
        case 'pending': return ReminderStatus.Pending;
        case 'active': return ReminderStatus.Active;
        case 'completed': return ReminderStatus.Completed;
        case 'cancelled':
        case 'canceled': return ReminderStatus.Cancelled;
        case 'overdue': return ReminderStatus.Overdue;
        default: return new TodoziError(`Invalid reminder status: ${s}`, 'InvalidReminderStatus');
    }
}

export interface Reminder {
    id: string;
    content: string;
    remindAt: Date;
    priority: ReminderPriority;
    status: ReminderStatus;
    tags: string[];
    createdAt: Date;
    updatedAt: Date;
}

export class ReminderClass implements Reminder {
    constructor(
        public id: string = generateUUID(),
        public content: string,
        public remindAt: Date,
        public priority: ReminderPriority,
        public status: ReminderStatus = ReminderStatus.Pending,
        public tags: string[] = [],
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date()
    ) {}

    static new(content: string, remindAt: Date, priority: ReminderPriority): ReminderClass {
        return new ReminderClass(undefined, content, remindAt, priority);
    }

    withTags(tags: string[]): ReminderClass {
        this.tags = tags;
        this.updatedAt = new Date();
        return this;
    }

    isOverdue(): boolean {
        return this.remindAt < new Date() && this.status === ReminderStatus.Pending;
    }

    markCompleted(): void {
        this.status = ReminderStatus.Completed;
        this.updatedAt = new Date();
    }

    markCancelled(): void {
        this.status = ReminderStatus.Cancelled;
        this.updatedAt = new Date();
    }

    activate(): void {
        this.status = ReminderStatus.Active;
        this.updatedAt = new Date();
    }
}

// ML Engine
export class MLEngine {
    constructor(
        public modelName: string,
        public temperature: number = 0.7,
        public maxTokens: number = 4096
    ) {}

    static new(modelName: string): MLEngine {
        return new MLEngine(modelName);
    }

    withTemperature(temperature: number): MLEngine {
        this.temperature = temperature;
        return this;
    }

    withMaxTokens(maxTokens: number): MLEngine {
        this.maxTokens = maxTokens;
        return this;
    }

    async predictRelevance(features: number[]): Promise<number> {
        // Placeholder implementation
        return 0.5;
    }

    async craftEmbedding(features: number[]): Promise<number[]> {
        // Placeholder implementation
        return new Array(384).fill(0.1);
    }

    async strikeTags(features: number[]): Promise<number[]> {
        // Placeholder implementation
        return new Array(10).fill(0.1);
    }

    async strikeCluster(embedding: number[]): Promise<number> {
        // Placeholder implementation
        return 0;
    }

    async analyzeCodeQuality(features: number[]): Promise<number> {
        // Placeholder implementation
        return 0.7;
    }
}

// Statistics and reporting types
export interface ProjectStats {
    projectName: string;
    totalTasks: number;
    activeTasks: number;
    completedTasks: number;
    archivedTasks: number;
    deletedTasks: number;
}

export interface SemanticSearchResult {
    task: Task;
    similarityScore: number;
    matchedContent: string;
}

export interface MigrationReport {
    tasksFound: number;
    tasksMigrated: number;
    projectsMigrated: number;
    projectStats: ProjectMigrationStats[];
    errors: string[];
}

export interface ProjectMigrationStats {
    projectName: string;
    initialTasks: number;
    migratedTasks: number;
    finalTasks: number;
}

export interface ProjectTaskContainer {
    projectName: string;
    projectHash: string;
    createdAt: Date;
    updatedAt: Date;
    activeTasks: Map<string, Task>;
    completedTasks: Map<string, Task>;
    archivedTasks: Map<string, Task>;
    deletedTasks: Map<string, Task>;
}

export class ProjectTaskContainerClass implements ProjectTaskContainer {
    constructor(
        public projectName: string,
        public projectHash: string = hashProjectName(projectName),
        public createdAt: Date = new Date(),
        public updatedAt: Date = new Date(),
        public activeTasks: Map<string, Task> = new Map(),
        public completedTasks: Map<string, Task> = new Map(),
        public archivedTasks: Map<string, Task> = new Map(),
        public deletedTasks: Map<string, Task> = new Map()
    ) {}

    static new(projectName: string): ProjectTaskContainerClass {
        return new ProjectTaskContainerClass(projectName);
    }

    addTask(task: Task): void {
        const taskId = task.id;
        switch (task.status) {
            case Status.Todo:
            case Status.Pending:
            case Status.InProgress:
            case Status.Blocked:
            case Status.Review:
                this.activeTasks.set(taskId, task);
                break;
            case Status.Done:
            case Status.Completed:
                this.completedTasks.set(taskId, task);
                break;
            case Status.Cancelled:
            case Status.Deferred:
                this.archivedTasks.set(taskId, task);
                break;
        }
        this.updatedAt = new Date();
    }

    getTask(taskId: string): Readonly<Task> | undefined {
        const task = this.activeTasks.get(taskId) ||
                     this.completedTasks.get(taskId) ||
                     this.archivedTasks.get(taskId) ||
                     this.deletedTasks.get(taskId);
        return task ? Object.freeze({ ...task }) : undefined;
    }

    getTaskMut(taskId: string): Task | undefined {
        return this.activeTasks.get(taskId) ||
               this.completedTasks.get(taskId) ||
               this.archivedTasks.get(taskId) ||
               this.deletedTasks.get(taskId);
    }

    removeTask(taskId: string): Task | undefined {
        let task = this.activeTasks.get(taskId);
        if (task) {
            this.activeTasks.delete(taskId);
        } else {
            task = this.completedTasks.get(taskId);
            if (task) {
                this.completedTasks.delete(taskId);
            } else {
                task = this.archivedTasks.get(taskId);
                if (task) {
                    this.archivedTasks.delete(taskId);
                } else {
                    task = this.deletedTasks.get(taskId);
                    if (task) {
                        this.deletedTasks.delete(taskId);
                    }
                }
            }
        }

        if (task) {
            this.updatedAt = new Date();
        }
        return task;
    }

    updateTaskStatus(taskId: string, newStatus: Status): void {
        const task = this.removeTask(taskId);
        if (task) {
            const updatedTask = { ...task, status: newStatus, updatedAt: new Date() };
            this.addTask(updatedTask);
        }
    }

    getAllTasks(): Task[] {
        return [
            ...Array.from(this.activeTasks.values()),
            ...Array.from(this.completedTasks.values()),
            ...Array.from(this.archivedTasks.values()),
            ...Array.from(this.deletedTasks.values())
        ];
    }

    getFilteredTasks(filters: TaskFilters): Task[] {
        return this.getAllTasks().filter(task => {
            if (filters.project && task.parentProject !== filters.project) return false;
            if (filters.status && task.status !== filters.status) return false;
            if (filters.priority && task.priority !== filters.priority) return false;
            if (filters.assignee) {
                if (!task.assignee || !this.assigneeEquals(task.assignee, filters.assignee)) {
                    return false;
                }
            }
            if (filters.tags && !filters.tags.some(tag => task.tags.includes(tag))) return false;
            if (filters.search) {
                const searchLower = filters.search.toLowerCase();
                if (!task.action.toLowerCase().includes(searchLower) &&
                    !task.contextNotes?.toLowerCase().includes(searchLower)) {
                    return false;
                }
            }
            return true;
        });
    }

    private assigneeEquals(a1: Assignee, a2: Assignee): boolean {
        if (a1.type !== a2.type) return false;
        if (a1.type === AssigneeType.Agent && a2.type === AssigneeType.Agent) {
            return a1.name === a2.name;
        }
        return true;
    }
}

// Utility functions
export function generateUUID(): string {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        const r = Math.random() * 16 | 0;
        const v = c === 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

function generateUUID8(): string {
    return Math.random().toString(36).substring(2, 10);
}

function hashProjectName(projectName: string): string {
    // Simple hash function for project names
    let hash = 0;
    for (let i = 0; i < projectName.length; i++) {
        const char = projectName.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash).toString(16);
}

function hashString(input: string): string {
    // Simple hash function for generating keys
    let hash = 0;
    for (let i = 0; i < input.length; i++) {
        const char = input.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash;
    }
    return Math.abs(hash).toString(16);
}

// Default exports
export const DefaultConfig = new ConfigClass();
export const DefaultTaskCollection = new TaskCollection();
export const DefaultQueueCollection = new QueueCollection();
export const DefaultApiKeyCollection = new ApiKeyCollection();
export const DefaultProjectTaskContainer = new ProjectTaskContainerClass('default');
