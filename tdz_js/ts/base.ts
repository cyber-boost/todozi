// Import necessary dependencies
import { EventEmitter } from 'events';

// Type definitions for better type safety
export type JsonValue = string | number | boolean | null | JsonObject | JsonArray;
export interface JsonObject { [key: string]: JsonValue }
export interface JsonArray extends Array<JsonValue> {}

// ToolParameter definition
export interface IToolParameter {
    name: string;
    type: string;
    description: string;
    required: boolean;
    defaultValue?: JsonValue;
}

export class ToolParameter implements IToolParameter {
    constructor(
        public readonly name: string,
        public readonly type: string,
        public readonly description: string,
        public readonly required: boolean,
        public readonly defaultValue?: JsonValue
    ) {}

    static create(
        name: string,
        type: string,
        description: string,
        required: boolean,
        defaultValue?: JsonValue
    ): ToolParameter {
        return new ToolParameter(name, type, description, required, defaultValue);
    }
}

// ResourceLock enum with display functionality
export enum ResourceLock {
    FilesystemWrite = 'FilesystemWrite',
    FilesystemRead = 'FilesystemRead',
    Git = 'Git',
    Memory = 'Memory',
    Shell = 'Shell',
    Network = 'Network'
}

export class ResourceLockClass {
    static display(lock: ResourceLock): string {
        return lock;
    }

    static asStr(lock: ResourceLock): string {
        switch (lock) {
            case ResourceLock.FilesystemWrite:
                return 'filesystem_write';
            case ResourceLock.FilesystemRead:
                return 'filesystem_read';
            case ResourceLock.Git:
                return 'git';
            case ResourceLock.Memory:
                return 'memory';
            case ResourceLock.Shell:
                return 'shell';
            case ResourceLock.Network:
                return 'network';
            default:
                return 'unknown';
        }
    }
}

// ToolDefinition interface and class
export interface IToolDefinition {
    name: string;
    description: string;
    parameters: IToolParameter[];
    category: string;
    resourceLocks: ResourceLock[];
}

export class ToolDefinition implements IToolDefinition {
    constructor(
        public readonly name: string,
        public readonly description: string,
        public readonly parameters: IToolParameter[],
        public readonly category: string,
        public readonly resourceLocks: ResourceLock[]
    ) {}

    static create(
        name: string,
        description: string,
        parameters: IToolParameter[],
        category: string,
        resourceLocks: ResourceLock[] = []
    ): ToolDefinition {
        return new ToolDefinition(name, description, parameters, category, resourceLocks);
    }

    toOllamaFormat(): JsonObject {
        const properties: JsonObject = {};
        const required: string[] = [];

        for (const param of this.parameters) {
            const propValue: JsonObject = {
                type: param.type,
                description: param.description
            };

            if (param.defaultValue !== undefined) {
                propValue.default = param.defaultValue;
            }

            if (param.required) {
                required.push(param.name);
            }

            properties[param.name] = propValue;
        }

        const parameters: JsonObject = {
            type: 'object',
            properties,
            required
        };

        return {
            type: 'function',
            function: {
                name: this.name,
                description: this.description,
                parameters
            }
        };
    }
}

// ToolResult interface and class with JSON serialization control
export interface IToolResult {
    success: boolean;
    output: string;
    error?: string;
    executionTimeMs: number;
    metadata?: Record<string, unknown>;
    recoveryContext?: Record<string, unknown>;
}

export class ToolResult implements IToolResult {
    constructor(
        public readonly success: boolean,
        public readonly output: string,
        public readonly error?: string,
        public readonly executionTimeMs: number = 0,
        public readonly metadata?: Record<string, unknown>,
        public readonly recoveryContext?: Record<string, unknown>
    ) {}

    static create(
        success: boolean,
        output: string,
        error?: string,
        executionTimeMs: number = 0,
        metadata?: Record<string, unknown>,
        recoveryContext?: Record<string, unknown>
    ): ToolResult {
        return new ToolResult(success, output, error, executionTimeMs, metadata, recoveryContext);
    }

    static success(output: string, executionTimeMs: number): ToolResult {
        return new ToolResult(true, output, undefined, executionTimeMs);
    }

    static error(error: string, executionTimeMs: number): ToolResult {
        return new ToolResult(false, '', error, executionTimeMs);
    }

    toJSON(): JsonObject {
        const result: JsonObject = {
            success: this.success,
            output: this.output,
            executionTimeMs: this.executionTimeMs
        };

        if (this.error !== undefined) {
            result.error = this.error;
        }

        if (this.metadata !== undefined) {
            result.metadata = this.metadata as JsonObject;
        }

        if (this.recoveryContext !== undefined) {
            result.recoveryContext = this.recoveryContext as JsonObject;
        }

        return result;
    }

    toString(): string {
        if (this.success) {
            return this.output;
        } else {
            return `Error: ${this.error || 'Unknown error'}`;
        }
    }
}

// ErrorType enum with display functionality
export enum ErrorType {
    ValidationError = 'validation_error',
    PermissionError = 'permission_error',
    FileNotFound = 'file_not_found',
    TimeoutError = 'timeout_error',
    ResourceError = 'resource_error',
    NetworkError = 'network_error',
    SecurityError = 'security_error',
    InternalError = 'internal_error'
}

export class ErrorTypeClass {
    static display(errorType: ErrorType): string {
        return errorType;
    }
}

// ToolError class extending Error
export class ToolError extends Error {
    constructor(
        message: string,
        public readonly errorType: ErrorType,
        public readonly details: Record<string, unknown>
    ) {
        super(message);
        this.name = 'ToolError';
    }

    static create(
        message: string,
        errorType: ErrorType,
        details: Record<string, unknown>
    ): ToolError {
        return new ToolError(message, errorType, details);
    }

    toString(): string {
        return this.message;
    }
}

// Logger interface for better abstraction
export interface ILogger {
    error(message: string, ...args: unknown[]): void;
    info(message: string, ...args: unknown[]): void;
    warn(message: string, ...args: unknown[]): void;
    debug(message: string, ...args: unknown[]): void;
}

// Default console logger
class ConsoleLogger implements ILogger {
    error(message: string, ...args: unknown[]): void {
        console.error(message, ...args);
    }

    info(message: string, ...args: unknown[]): void {
        console.info(message, ...args);
    }

    warn(message: string, ...args: unknown[]): void {
        console.warn(message, ...args);
    }

    debug(message: string, ...args: unknown[]): void {
        console.debug(message, ...args);
    }
}

// ErrorHandler class with comprehensive error handling
export class ErrorHandler {
    private static logger: ILogger = new ConsoleLogger();

    static getLogger(): ILogger {
        return ErrorHandler.logger;
    }

    static setLogger(logger: ILogger): void {
        ErrorHandler.logger = logger;
    }

    static handleError(error: Error, context: string): ToolResult {
        const metadata: Record<string, unknown> = {
            context,
            errorType: ErrorType.InternalError
        };

        const errorString = error.toString();
        let errorMessage: string;

        if (errorString.includes('ToolError')) {
            errorMessage = errorString;
        } else if (errorString.includes('I/O') || errorString.includes('ENOENT')) {
            errorMessage = `I/O error: ${error.message}`;
            metadata.errorType = ErrorType.ResourceError;
        } else if (errorString.includes('Permission denied')) {
            errorMessage = `Permission error: ${error.message}`;
            metadata.errorType = ErrorType.PermissionError;
        } else {
            errorMessage = `Unexpected error: ${error.message}`;
            metadata.errorType = ErrorType.InternalError;
            metadata.exceptionType = error.constructor.name;
        }

        ErrorHandler.logger.error(`Tool error in ${context}: ${errorMessage}`);

        return ToolResult.create(false, '', errorMessage, 0, metadata);
    }

    static validateRequiredParams(
        kwargs: Record<string, JsonValue>,
        requiredParams: string[]
    ): ToolResult | null {
        const missingParams = requiredParams.filter(param => !(param in kwargs));

        if (missingParams.length > 0) {
            const errorMessage = `Missing required parameters: ${missingParams.join(', ')}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                missingParams
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        return null;
    }

    static validateStringParam(
        value: JsonValue,
        paramName: string,
        minLength: number,
        maxLength: number,
        pattern?: string
    ): ToolResult | null {
        if (typeof value !== 'string') {
            const errorMessage = `Parameter '${paramName}' must be a string, got ${typeof value}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualType: typeof value
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        const stringValue = value;

        if (stringValue.length < minLength) {
            const errorMessage = `Parameter '${paramName}' must be at least ${minLength} characters, got ${stringValue.length}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualLength: stringValue.length,
                minLength
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (stringValue.length > maxLength) {
            const errorMessage = `Parameter '${paramName}' must be at most ${maxLength} characters, got ${stringValue.length}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualLength: stringValue.length,
                maxLength
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (pattern) {
            try {
                const regex = new RegExp(pattern);
                if (!regex.test(stringValue)) {
                    const errorMessage = `Parameter '${paramName}' does not match required pattern`;
                    const metadata: Record<string, unknown> = {
                        errorType: ErrorType.ValidationError,
                        paramName,
                        pattern
                    };

                    return ToolResult.create(false, '', errorMessage, 0, metadata);
                }
            } catch (error) {
                const errorMessage = `Invalid regex pattern: ${error}`;
                return ToolResult.error(errorMessage, 0);
            }
        }

        return null;
    }

    static validateNumberParam(
        value: JsonValue,
        paramName: string,
        min?: number,
        max?: number
    ): ToolResult | null {
        if (typeof value !== 'number') {
            const errorMessage = `Parameter '${paramName}' must be a number, got ${typeof value}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualType: typeof value
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (min !== undefined && value < min) {
            const errorMessage = `Parameter '${paramName}' must be at least ${min}, got ${value}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualValue: value,
                min
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (max !== undefined && value > max) {
            const errorMessage = `Parameter '${paramName}' must be at most ${max}, got ${value}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualValue: value,
                max
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        return null;
    }

    static validateArrayParam(
        value: JsonValue,
        paramName: string,
        minLength?: number,
        maxLength?: number,
        elementType?: string
    ): ToolResult | null {
        if (!Array.isArray(value)) {
            const errorMessage = `Parameter '${paramName}' must be an array, got ${typeof value}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualType: typeof value
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (minLength !== undefined && value.length < minLength) {
            const errorMessage = `Parameter '${paramName}' must have at least ${minLength} elements, got ${value.length}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualLength: value.length,
                minLength
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (maxLength !== undefined && value.length > maxLength) {
            const errorMessage = `Parameter '${paramName}' must have at most ${maxLength} elements, got ${value.length}`;
            const metadata: Record<string, unknown> = {
                errorType: ErrorType.ValidationError,
                paramName,
                actualLength: value.length,
                maxLength
            };

            return ToolResult.create(false, '', errorMessage, 0, metadata);
        }

        if (elementType) {
            for (let i = 0; i < value.length; i++) {
                const element = value[i];
                if (elementType === 'string' && typeof element !== 'string') {
                    const errorMessage = `Element at index ${i} in parameter '${paramName}' must be a string`;
                    const metadata: Record<string, unknown> = {
                        errorType: ErrorType.ValidationError,
                        paramName,
                        index: i,
                        expectedType: elementType,
                        actualType: typeof element
                    };

                    return ToolResult.create(false, '', errorMessage, 0, metadata);
                } else if (elementType === 'number' && typeof element !== 'number') {
                    const errorMessage = `Element at index ${i} in parameter '${paramName}' must be a number`;
                    const metadata: Record<string, unknown> = {
                        errorType: ErrorType.ValidationError,
                        paramName,
                        index: i,
                        expectedType: elementType,
                        actualType: typeof element
                    };

                    return ToolResult.create(false, '', errorMessage, 0, metadata);
                }
            }
        }

        return null;
    }

    static createSuccessResult(
        output: string,
        executionTimeMs: number,
        metadata?: Record<string, unknown>
    ): ToolResult {
        return ToolResult.create(true, output, undefined, executionTimeMs, metadata);
    }

    static createErrorResult(
        errorMsg: string,
        executionTimeMs: number,
        errorType: ErrorType,
        metadata?: Record<string, unknown>
    ): ToolResult {
        const resultMetadata: Record<string, unknown> = {
            errorType,
            ...metadata
        };

        return ToolResult.create(false, '', errorMsg, executionTimeMs, resultMetadata);
    }
}

// Tool interface
export interface ITool {
    readonly definition: IToolDefinition;
    execute(kwargs: Record<string, JsonValue>): Promise<ToolResult>;
    readonly name: string;
    validateParameters(kwargs: Record<string, JsonValue>): boolean;
}

export abstract class Tool implements ITool {
    constructor(public readonly definition: IToolDefinition) {}

    get name(): string {
        return this.definition.name;
    }

    abstract execute(kwargs: Record<string, JsonValue>): Promise<ToolResult>;

    validateParameters(kwargs: Record<string, JsonValue>): boolean {
        // Check required parameters
        for (const param of this.definition.parameters) {
            if (param.required && !(param.name in kwargs)) {
                return false;
            }
        }

        // Validate parameter types
        for (const [paramName, value] of Object.entries(kwargs)) {
            const paramDef = this.definition.parameters.find(p => p.name === paramName);
            if (!paramDef) continue;

            // Skip validation for JSON-serializable values
            if (paramName === 'value' && paramDef.description.includes('JSON-serializable')) {
                continue;
            }

            switch (paramDef.type) {
                case 'string':
                    if (typeof value !== 'string') return false;
                    break;
                case 'number':
                    if (typeof value !== 'number') return false;
                    break;
                case 'boolean':
                    if (typeof value !== 'boolean') return false;
                    break;
                case 'array':
                    if (!Array.isArray(value)) return false;
                    break;
                case 'object':
                    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
                        return false;
                    }
                    break;
            }
        }

        return true;
    }
}

// ToolRegistry interface
export interface IToolRegistry {
    hasTool(name: string): boolean;
    toolCount(): number;
}

// ToolRegistry implementation
export class ToolRegistry implements IToolRegistry {
    private tools: Map<string, ITool> = new Map();

    constructor() {}

    static create(): ToolRegistry {
        return new ToolRegistry();
    }

    register(tool: ITool): void {
        this.tools.set(tool.name, tool);
    }

    registerCoreTools(): void {
        ErrorHandler.getLogger().info(
            'Core tools registration structure prepared - individual tool implementations would be registered here'
        );
    }

    getTool(name: string): ITool | undefined {
        return this.tools.get(name);
    }

    getAllTools(): ITool[] {
        return Array.from(this.tools.values());
    }

    getToolDefinitions(): JsonObject[] {
        return Array.from(this.tools.values()).map(tool => 
            (tool.definition as ToolDefinition).toOllamaFormat()
        );
    }

    async executeTool(
        toolName: string,
        kwargs: Record<string, JsonValue>
    ): Promise<ToolResult> {
        const tool = this.getTool(toolName);
        if (!tool) {
            return ToolResult.error(`Tool '${toolName}' not found`, 0);
        }

        if (!tool.validateParameters(kwargs)) {
            return ToolResult.error(`Invalid parameters for tool '${toolName}'`, 0);
        }

        try {
            return await tool.execute(kwargs);
        } catch (error) {
            return ErrorHandler.handleError(error as Error, `executeTool(${toolName})`);
        }
    }

    toolCount(): number {
        return this.tools.size;
    }

    hasTool(name: string): boolean {
        return this.tools.has(name);
    }

    unregister(name: string): boolean {
        return this.tools.delete(name);
    }

    clear(): void {
        this.tools.clear();
    }
}

// Helper functions
export function createToolParameter(
    name: string,
    type: string,
    description: string,
    required: boolean
): ToolParameter {
    return ToolParameter.create(name, type, description, required);
}

export function createToolParameterWithDefault(
    name: string,
    type: string,
    description: string,
    required: boolean,
    defaultValue: JsonValue
): ToolParameter {
    return ToolParameter.create(name, type, description, required, defaultValue);
}

export function createToolDefinition(
    name: string,
    description: string,
    category: string,
    parameters: IToolParameter[]
): ToolDefinition {
    return ToolDefinition.create(name, description, parameters, category);
}

export function createToolDefinitionWithLocks(
    name: string,
    description: string,
    category: string,
    parameters: IToolParameter[],
    resourceLocks: ResourceLock[]
): ToolDefinition {
    return ToolDefinition.create(name, description, parameters, category, resourceLocks);
}

// Export utility namespace
export const Utils = {
    ErrorHandler,
    ErrorType,
    ResourceLock: ResourceLockClass,
    ToolDefinition,
    ToolError,
    ToolParameter,
    ToolRegistry,
    ToolResult,
    createToolParameter,
    createToolParameterWithDefault,
    createToolDefinition,
    createToolDefinitionWithLocks
};

// Example tool implementation for testing
export class ExampleTool extends Tool {
    constructor() {
        const definition = createToolDefinition(
            'example',
            'An example tool for demonstration',
            'Example',
            [
                createToolParameter('message', 'string', 'Message to process', true),
                createToolParameterWithDefault('repeat', 'number', 'Number of times to repeat', false, 1)
            ]
        );
        super(definition);
    }

    async execute(kwargs: Record<string, JsonValue>): Promise<ToolResult> {
        const startTime = Date.now();
        
        const validationError = ErrorHandler.validateRequiredParams(kwargs, ['message']);
        if (validationError) {
            return validationError;
        }

        const message = kwargs.message as string;
        const repeat = (kwargs.repeat as number) || 1;
        
        if (repeat < 1) {
            return ErrorHandler.createErrorResult(
                'Repeat count must be at least 1',
                0,
                ErrorType.ValidationError
            );
        }

        const output = message.repeat(repeat);
        const executionTime = Date.now() - startTime;
        
        return ErrorHandler.createSuccessResult(output, executionTime, {
            repeat,
            messageLength: message.length
        });
    }
}
