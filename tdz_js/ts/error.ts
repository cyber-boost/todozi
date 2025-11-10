// Type definitions
export type Result<T, E = TodoziError> = { ok: true; value: T } | { ok: false; error: E };

export enum ErrorSeverity {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical',
    Urgent = 'urgent'
}

export enum ErrorCategory {
    Network = 'network',
    Database = 'database',
    Authentication = 'authentication',
    Validation = 'validation',
    Configuration = 'configuration',
    IO = 'io',
    API = 'api',
    Storage = 'storage',
    Embedding = 'embedding',
    General = 'general'
}

export enum TaskStatus {
    Todo = 'todo',
    InProgress = 'in_progress',
    Blocked = 'blocked',
    Review = 'review',
    Done = 'done',
    Cancelled = 'cancelled',
    Deferred = 'deferred'
}

export enum TaskPriority {
    Low = 'low',
    Medium = 'medium',
    High = 'high',
    Critical = 'critical',
    Urgent = 'urgent'
}

export enum Assignee {
    AI = 'ai',
    Human = 'human',
    Collaborative = 'collaborative'
}

export interface TodoziErrorRecord {
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

export interface Task {
    id: string;
    title: string;
    description?: string;
    status: TaskStatus;
    priority: TaskPriority;
    assignee: Assignee;
    progress: number; // 0-100
    project_id?: string;
    created_at: Date;
    updated_at: Date;
}

export interface Project {
    id: string;
    name: string;
    description?: string;
    tasks: string[];
    created_at: Date;
    updated_at: Date;
}

export interface Feeling {
    id: string;
    content: string;
    sentiment: string;
    created_at: Date;
}

export class TodoziError extends Error {
    public readonly code: string;
    public readonly details?: Record<string, any>;
    public readonly cause?: Error;

    public constructor(
        code: string,
        message: string,
        details?: Record<string, any>,
        cause?: Error
    ) {
        super(message);
        this.name = 'TodoziError';
        this.code = code;
        this.details = details;
        this.cause = cause;

        // Preserve stack trace
        if (Error.captureStackTrace) {
            Error.captureStackTrace(this, TodoziError);
        }

        if (cause && cause.stack) {
            this.stack = `${this.stack}\nCaused by: ${cause.stack}`;
        }
    }

    // Factory methods with consistent detail handling
    static taskNotFound(id: string): TodoziError {
        return new TodoziError('TASK_NOT_FOUND', `Task not found: ${id}`, { id });
    }

    static projectNotFound(name: string): TodoziError {
        return new TodoziError('PROJECT_NOT_FOUND', `Project not found: ${name}`, { name });
    }

    static feelingNotFound(id: string): TodoziError {
        return new TodoziError('FEELING_NOT_FOUND', `Feeling not found: ${id}`, { id });
    }

    static invalidPriority(priority: string): TodoziError {
        return new TodoziError(
            'INVALID_PRIORITY',
            `Invalid priority: ${priority}. Must be one of: low, medium, high, critical, urgent`,
            { priority }
        );
    }

    static invalidStatus(status: string): TodoziError {
        return new TodoziError(
            'INVALID_STATUS',
            `Invalid status: ${status}. Must be one of: todo, in_progress, blocked, review, done, cancelled, deferred`,
            { status }
        );
    }

    static invalidAssignee(assignee: string): TodoziError {
        return new TodoziError(
            'INVALID_ASSIGNEE',
            `Invalid assignee: ${assignee}. Must be one of: ai, human, collaborative`,
            { assignee }
        );
    }

    static invalidProgress(progress: number): TodoziError {
        return new TodoziError(
            'INVALID_PROGRESS',
            `Invalid progress: ${progress}. Must be between 0 and 100`,
            { progress }
        );
    }

    static validation(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('VALIDATION_ERROR', `Validation error: ${message}`, details);
    }

    static storage(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('STORAGE_ERROR', `Storage error: ${message}`, details);
    }

    static config(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('CONFIG_ERROR', `Configuration error: ${message}`, details);
    }

    static api(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('API_ERROR', `API error: ${message}`, details);
    }

    static io(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('IO_ERROR', `IO error: ${message}`, details);
    }

    static json(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('JSON_ERROR', `JSON error: ${message}`, details);
    }

    static uuid(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('UUID_ERROR', `UUID error: ${message}`, details);
    }

    static chrono(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('CHRONO_ERROR', `Chrono error: ${message}`, details);
    }

    static dir(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('DIR_ERROR', `Directory error: ${message}`, details);
    }

    static embedding(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('EMBEDDING_ERROR', `Embedding error: ${message}`, details);
    }

    static candle(message: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('CANDLE_ERROR', `Candle error: ${message}`, details);
    }

    static notImplemented(feature: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('NOT_IMPLEMENTED', `Feature not implemented: ${feature}`, details);
    }

    // Conversion methods
    static fromError(error: Error, context?: string): TodoziError {
        return new TodoziError(
            'UNKNOWN_ERROR',
            error.message,
            { context, originalError: error.name },
            error
        );
    }

    static notFound(resource: string, details?: Record<string, any>): TodoziError {
        return new TodoziError('NOT_FOUND', `Resource not found: ${resource}`, { resource, ...details });
    }

    // Validation methods
    static isValidSeverity(value: string): value is ErrorSeverity {
        return Object.values(ErrorSeverity).includes(value as ErrorSeverity);
    }

    static isValidCategory(value: string): value is ErrorCategory {
        return Object.values(ErrorCategory).includes(value as ErrorCategory);
    }

    static isValidStatus(value: string): value is TaskStatus {
        return Object.values(TaskStatus).includes(value as TaskStatus);
    }

    static isValidPriority(value: string): value is TaskPriority {
        return Object.values(TaskPriority).includes(value as TaskPriority);
    }

    static isValidAssignee(value: string): value is Assignee {
        return Object.values(Assignee).includes(value as Assignee);
    }

    // Serialization methods
    toJSON(): object {
        return {
            code: this.code,
            message: this.message,
            details: this.details,
            stack: this.stack,
            cause: this.cause ? {
                message: this.cause.message,
                stack: this.cause.stack
            } : undefined
        };
    }

    static fromJSON(json: string | object): TodoziError {
        const data = typeof json === 'string' ? JSON.parse(json) : json;
        return new TodoziError(
            data.code || 'UNKNOWN_ERROR',
            data.message || 'Unknown error',
            data.details,
            data.cause ? new Error(data.cause.message) : undefined
        );
    }
}

export class ErrorManager {
    private errors: Map<string, TodoziErrorRecord> = new Map();

    constructor() {
        this.errors = new Map();
    }

    async createError(errorInput: Partial<TodoziErrorRecord>): Promise<Result<string>> {
        try {
            const now = new Date();
            const id = this.generateId();
            const error: TodoziErrorRecord = {
                id,
                title: errorInput.title || '',
                description: errorInput.description || '',
                severity: errorInput.severity || ErrorSeverity.Medium,
                category: errorInput.category || ErrorCategory.General,
                source: errorInput.source || '',
                context: errorInput.context,
                tags: errorInput.tags || [],
                resolved: false,
                created_at: now,
                updated_at: now,
                resolution: undefined,
                resolved_at: undefined
            };

            this.errors.set(id, error);
            return { ok: true, value: id };
        } catch (e) {
            return { ok: false, error: TodoziError.fromError(e as Error, 'createError') };
        }
    }

    getUnresolvedErrors(): TodoziErrorRecord[] {
        return Array.from(this.errors.values()).filter(error => !error.resolved);
    }

    async resolveError(errorId: string, resolution: string): Promise<Result<void>> {
        const error = this.errors.get(errorId);
        if (!error) {
            return { 
                ok: false, 
                error: TodoziError.validation(`Error ${errorId} not found`, { errorId }) 
            };
        }

        // Immutable approach - create a new error object
        const updatedError: TodoziErrorRecord = {
            ...error,
            resolved: true,
            resolution,
            resolved_at: new Date(),
            updated_at: new Date()
        };

        this.errors.set(errorId, updatedError);
        return { ok: true, value: undefined };
    }

    // Enhanced utility methods
    getError(errorId: string): TodoziErrorRecord | undefined {
        return this.errors.get(errorId);
    }

    getErrorsBySeverity(severity: ErrorSeverity): TodoziErrorRecord[] {
        return Array.from(this.errors.values()).filter(e => e.severity === severity);
    }

    getErrorsByCategory(category: ErrorCategory): TodoziErrorRecord[] {
        return Array.from(this.errors.values()).filter(e => e.category === category);
    }

    getErrorsByTag(tag: string): TodoziErrorRecord[] {
        return Array.from(this.errors.values()).filter(e => e.tags.includes(tag));
    }

    clearResolvedErrors(): void {
        for (const [id, error] of this.errors) {
            if (error.resolved) {
                this.errors.delete(id);
            }
        }
    }

    getAllErrors(): TodoziErrorRecord[] {
        return Array.from(this.errors.values());
    }

    getErrorCount(): number {
        return this.errors.size;
    }

    getUnresolvedErrorCount(): number {
        return this.getUnresolvedErrors().length;
    }

    private generateId(): string {
        return crypto.randomUUID();
    }
}

export function parseErrorFormat(errorText: string): Result<TodoziErrorRecord> {
    const startTag = '<error>';
    const endTag = '</error>';
    
    const startIndex = errorText.indexOf(startTag);
    if (startIndex === -1) {
        return { 
            ok: false, 
            error: TodoziError.validation('Missing <error> start tag') 
        };
    }

    const endIndex = errorText.indexOf(endTag);
    if (endIndex === -1) {
        return { 
            ok: false, 
            error: TodoziError.validation('Missing </error> end tag') 
        };
    }

    const content = errorText.substring(startIndex + startTag.length, endIndex);
    const parts = content.split(';').map(s => s.trim()).filter(s => s.length > 0);

    if (parts.length < 5) {
        return { 
            ok: false, 
            error: TodoziError.validation(
                'Invalid error format: need at least 5 parts (title; description; severity; category; source)'
            ) 
        };
    }

    const severity = parts[2].toLowerCase();
    if (!TodoziError.isValidSeverity(severity)) {
        return { 
            ok: false, 
            error: TodoziError.validation('Invalid error severity') 
        };
    }

    const category = parts[3].toLowerCase();
    if (!TodoziError.isValidCategory(category)) {
        return { 
            ok: false, 
            error: TodoziError.validation('Invalid error category') 
        };
    }

    const tags = parts.length > 6 && parts[6] 
        ? parts[6].split(',').map(s => s.trim()).filter(s => s.length > 0)
        : [];

    const now = new Date();
    const error: Error = {
        id: crypto.randomUUID(),
        title: parts[0],
        description: parts[1],
        severity: severity as ErrorSeverity,
        category: category as ErrorCategory,
        source: parts[4],
        context: parts.length > 5 && parts[5] ? parts[5] : undefined,
        tags,
        resolved: false,
        created_at: now,
        updated_at: now,
        resolution: undefined,
        resolved_at: undefined
    };

    return { ok: true, value: error };
}

// Test function
export function testParseErrorFormat(): void {
    const errorText = '<error>Database connection failed; Unable to connect to PostgreSQL database; critical; network; database-service; Connection timeout after 30 seconds; database,postgres,connection</error>';
    const result = parseErrorFormat(errorText);
    
    if (result.ok) {
        const error = result.value;
        console.assert(error.title === 'Database connection failed');
        console.assert(error.description === 'Unable to connect to PostgreSQL database');
        console.assert(error.severity === ErrorSeverity.Critical);
        console.assert(error.category === ErrorCategory.Network);
        console.assert(error.source === 'database-service');
        console.assert(error.context === 'Connection timeout after 30 seconds');
        console.assert(error.tags.length === 3);
        console.assert(error.tags.includes('database'));
        console.assert(error.tags.includes('postgres'));
        console.assert(error.tags.includes('connection'));
        console.assert(error.resolved === false);
        console.log('All tests passed!');
    } else {
        console.error('Test failed:', result.error);
    }
}

// Utility functions for converting other error types
export function convertNodeError(error: NodeJS.ErrnoException): TodoziError {
    if (error.code === 'ENOENT') {
        return TodoziError.io(`File not found: ${error.path}`, { path: error.path });
    }
    if (error.code === 'EACCES') {
        return TodoziError.io(`Permission denied: ${error.path}`, { path: error.path });
    }
    return TodoziError.io(error.message, { code: error.code });
}

export function convertFetchError(error: Error): TodoziError {
    if (error.name === 'AbortError') {
        return TodoziError.api('Request was aborted', { type: 'abort' });
    }
    if (error.name === 'TypeError') {
        return TodoziError.api('Network error occurred', { type: 'network' });
    }
    return TodoziError.api(error.message, { type: 'unknown' });
}

// Export all types and classes
export default {
    TodoziError,
    ErrorManager,
    parseErrorFormat,
    ErrorSeverity,
    ErrorCategory,
    TaskStatus,
    TaskPriority,
    Assignee,
    convertNodeError,
    convertFetchError,
    testParseErrorFormat
};
