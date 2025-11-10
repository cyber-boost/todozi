import {
    TodoziError,
    Result,
    Priority,
    Status,
    Assignee,
    parsePriority,
    parseStatus,
    parseAssignee,
    parseMemoryImportance,
    parseMemoryTerm,
    parseMemoryType,
    parseShareLevel,
    parseIdeaImportance,
    parseErrorSeverity,
    parseErrorCategory,
    parseSummaryPriority,
    parseReminderPriority,
    parseReminderStatus,
    parseQueueStatus,
    parseTrainingDataType,
    MemoryImportance,
    MemoryTerm,
    MemoryType,
    ShareLevel,
    IdeaImportance,
    ErrorSeverity,
    ErrorCategory,
    SummaryPriority,
    ReminderPriority,
    ReminderStatus,
    QueueStatus,
    TrainingDataType,
    ItemStatus,
    AgentStatus
} from './models.js';
import { ValidationUtils } from './utils.js';

// Enhanced validation utilities for Todozi
export class TodoziValidator {
    // Task validation
    static validateTaskData(data: {
        action?: string;
        time?: string;
        priority?: string | Priority;
        parentProject?: string;
        status?: string | Status;
        assignee?: string | Assignee;
        tags?: string[];
        dependencies?: string[];
        contextNotes?: string;
        progress?: number;
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.action || typeof data.action !== 'string' || data.action.trim().length === 0) {
            errors.push('Action is required and must be a non-empty string');
        } else if (data.action.length > 1000) {
            errors.push('Action must be less than 1000 characters');
        }

        if (!data.time || typeof data.time !== 'string' || data.time.trim().length === 0) {
            errors.push('Time is required and must be a non-empty string');
        }

        // Priority validation
        if (data.priority !== undefined) {
            if (typeof data.priority === 'string') {
                const parsed = parsePriority(data.priority);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid priority: ${parsed.message}`);
                }
            } else if (!Object.values(Priority).includes(data.priority)) {
                errors.push('Invalid priority value');
            }
        }

        // Status validation
        if (data.status !== undefined) {
            if (typeof data.status === 'string') {
                const parsed = parseStatus(data.status);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid status: ${parsed.message}`);
                }
            } else if (!Object.values(Status).includes(data.status)) {
                errors.push('Invalid status value');
            }
        }

        // Assignee validation
        if (data.assignee !== undefined) {
            if (typeof data.assignee === 'string') {
                const parsed = parseAssignee(data.assignee);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid assignee: ${parsed.message}`);
                }
            } else if (typeof data.assignee === 'object') {
                if (!data.assignee.type || !Object.values(Assignee).includes(data.assignee.type)) {
                    errors.push('Invalid assignee type');
                }
                if (data.assignee.type === 'agent' && !data.assignee.name) {
                    errors.push('Agent assignee must have a name');
                }
            }
        }

        // Progress validation
        if (data.progress !== undefined) {
            if (typeof data.progress !== 'number' || data.progress < 0 || data.progress > 100) {
                errors.push('Progress must be a number between 0 and 100');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        // Dependencies validation
        if (data.dependencies !== undefined) {
            if (!Array.isArray(data.dependencies)) {
                errors.push('Dependencies must be an array');
            } else {
                data.dependencies.forEach((dep, index) => {
                    if (typeof dep !== 'string' || dep.trim().length === 0) {
                        errors.push(`Dependency at index ${index} must be a non-empty string`);
                    }
                });
            }
        }

        // Optional fields length validation
        if (data.parentProject && data.parentProject.length > 100) {
            errors.push('Parent project must be less than 100 characters');
        }

        if (data.contextNotes && data.contextNotes.length > 5000) {
            errors.push('Context notes must be less than 5000 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Task validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Memory validation
    static validateMemoryData(data: {
        moment?: string;
        meaning?: string;
        reason?: string;
        importance?: string | MemoryImportance;
        term?: string | MemoryTerm;
        memoryType?: string | MemoryType;
        tags?: string[];
        projectId?: string;
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.moment || typeof data.moment !== 'string' || data.moment.trim().length === 0) {
            errors.push('Moment is required and must be a non-empty string');
        } else if (data.moment.length > 1000) {
            errors.push('Moment must be less than 1000 characters');
        }

        if (!data.meaning || typeof data.meaning !== 'string' || data.meaning.trim().length === 0) {
            errors.push('Meaning is required and must be a non-empty string');
        } else if (data.meaning.length > 2000) {
            errors.push('Meaning must be less than 2000 characters');
        }

        if (!data.reason || typeof data.reason !== 'string' || data.reason.trim().length === 0) {
            errors.push('Reason is required and must be a non-empty string');
        } else if (data.reason.length > 2000) {
            errors.push('Reason must be less than 2000 characters');
        }

        // Importance validation
        if (data.importance !== undefined) {
            if (typeof data.importance === 'string') {
                const parsed = parseMemoryImportance(data.importance);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid importance: ${parsed.message}`);
                }
            } else if (!Object.values(MemoryImportance).includes(data.importance)) {
                errors.push('Invalid importance value');
            }
        }

        // Term validation
        if (data.term !== undefined) {
            if (typeof data.term === 'string') {
                const parsed = parseMemoryTerm(data.term);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid term: ${parsed.message}`);
                }
            } else if (!Object.values(MemoryTerm).includes(data.term)) {
                errors.push('Invalid term value');
            }
        }

        // Memory type validation
        if (data.memoryType !== undefined) {
            if (typeof data.memoryType === 'string') {
                const parsed = parseMemoryType(data.memoryType);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid memory type: ${parsed.message}`);
                }
            } else if (!Object.values(MemoryType).includes(data.memoryType)) {
                errors.push('Invalid memory type value');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        if (data.projectId && data.projectId.length > 100) {
            errors.push('Project ID must be less than 100 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Memory validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Idea validation
    static validateIdeaData(data: {
        idea?: string;
        share?: string | ShareLevel;
        importance?: string | IdeaImportance;
        tags?: string[];
        context?: string;
        projectId?: string;
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.idea || typeof data.idea !== 'string' || data.idea.trim().length === 0) {
            errors.push('Idea is required and must be a non-empty string');
        } else if (data.idea.length > 2000) {
            errors.push('Idea must be less than 2000 characters');
        }

        // Share level validation
        if (data.share !== undefined) {
            if (typeof data.share === 'string') {
                const parsed = parseShareLevel(data.share);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid share level: ${parsed.message}`);
                }
            } else if (!Object.values(ShareLevel).includes(data.share)) {
                errors.push('Invalid share level value');
            }
        }

        // Importance validation
        if (data.importance !== undefined) {
            if (typeof data.importance === 'string') {
                const parsed = parseIdeaImportance(data.importance);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid importance: ${parsed.message}`);
                }
            } else if (!Object.values(IdeaImportance).includes(data.importance)) {
                errors.push('Invalid importance value');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        // Optional fields length validation
        if (data.context && data.context.length > 5000) {
            errors.push('Context must be less than 5000 characters');
        }

        if (data.projectId && data.projectId.length > 100) {
            errors.push('Project ID must be less than 100 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Idea validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Error record validation
    static validateErrorData(data: {
        title?: string;
        description?: string;
        severity?: string | ErrorSeverity;
        category?: string | ErrorCategory;
        source?: string;
        context?: string;
        tags?: string[];
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.title || typeof data.title !== 'string' || data.title.trim().length === 0) {
            errors.push('Title is required and must be a non-empty string');
        } else if (data.title.length > 200) {
            errors.push('Title must be less than 200 characters');
        }

        if (!data.description || typeof data.description !== 'string' || data.description.trim().length === 0) {
            errors.push('Description is required and must be a non-empty string');
        } else if (data.description.length > 5000) {
            errors.push('Description must be less than 5000 characters');
        }

        if (!data.source || typeof data.source !== 'string' || data.source.trim().length === 0) {
            errors.push('Source is required and must be a non-empty string');
        } else if (data.source.length > 100) {
            errors.push('Source must be less than 100 characters');
        }

        // Severity validation
        if (data.severity !== undefined) {
            if (typeof data.severity === 'string') {
                const parsed = parseErrorSeverity(data.severity);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid severity: ${parsed.message}`);
                }
            } else if (!Object.values(ErrorSeverity).includes(data.severity)) {
                errors.push('Invalid severity value');
            }
        }

        // Category validation
        if (data.category !== undefined) {
            if (typeof data.category === 'string') {
                const parsed = parseErrorCategory(data.category);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid category: ${parsed.message}`);
                }
            } else if (!Object.values(ErrorCategory).includes(data.category)) {
                errors.push('Invalid category value');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        // Optional fields length validation
        if (data.context && data.context.length > 5000) {
            errors.push('Context must be less than 5000 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Error validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Training data validation
    static validateTrainingData(data: {
        dataType?: string | TrainingDataType;
        prompt?: string;
        completion?: string;
        source?: string;
        context?: string;
        tags?: string[];
        qualityScore?: number;
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.prompt || typeof data.prompt !== 'string' || data.prompt.trim().length === 0) {
            errors.push('Prompt is required and must be a non-empty string');
        } else if (data.prompt.length > 10000) {
            errors.push('Prompt must be less than 10000 characters');
        }

        if (!data.completion || typeof data.completion !== 'string' || data.completion.trim().length === 0) {
            errors.push('Completion is required and must be a non-empty string');
        } else if (data.completion.length > 10000) {
            errors.push('Completion must be less than 10000 characters');
        }

        if (!data.source || typeof data.source !== 'string' || data.source.trim().length === 0) {
            errors.push('Source is required and must be a non-empty string');
        } else if (data.source.length > 100) {
            errors.push('Source must be less than 100 characters');
        }

        // Data type validation
        if (data.dataType !== undefined) {
            if (typeof data.dataType === 'string') {
                const parsed = parseTrainingDataType(data.dataType);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid data type: ${parsed.message}`);
                }
            } else if (!Object.values(TrainingDataType).includes(data.dataType)) {
                errors.push('Invalid data type value');
            }
        }

        // Quality score validation
        if (data.qualityScore !== undefined) {
            if (typeof data.qualityScore !== 'number' || data.qualityScore < 0 || data.qualityScore > 1) {
                errors.push('Quality score must be a number between 0 and 1');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        // Optional fields length validation
        if (data.context && data.context.length > 5000) {
            errors.push('Context must be less than 5000 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Training data validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Summary validation
    static validateSummaryData(data: {
        content?: string;
        priority?: string | SummaryPriority;
        context?: string;
        tags?: string[];
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.content || typeof data.content !== 'string' || data.content.trim().length === 0) {
            errors.push('Content is required and must be a non-empty string');
        } else if (data.content.length > 10000) {
            errors.push('Content must be less than 10000 characters');
        }

        // Priority validation
        if (data.priority !== undefined) {
            if (typeof data.priority === 'string') {
                const parsed = parseSummaryPriority(data.priority);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid priority: ${parsed.message}`);
                }
            } else if (!Object.values(SummaryPriority).includes(data.priority)) {
                errors.push('Invalid priority value');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        // Optional fields length validation
        if (data.context && data.context.length > 5000) {
            errors.push('Context must be less than 5000 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Summary validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Reminder validation
    static validateReminderData(data: {
        content?: string;
        remindAt?: Date;
        priority?: string | ReminderPriority;
        status?: string | ReminderStatus;
        tags?: string[];
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.content || typeof data.content !== 'string' || data.content.trim().length === 0) {
            errors.push('Content is required and must be a non-empty string');
        } else if (data.content.length > 1000) {
            errors.push('Content must be less than 1000 characters');
        }

        if (!data.remindAt || !(data.remindAt instanceof Date) || isNaN(data.remindAt.getTime())) {
            errors.push('RemindAt is required and must be a valid Date');
        } else if (data.remindAt <= new Date()) {
            errors.push('RemindAt must be in the future');
        }

        // Priority validation
        if (data.priority !== undefined) {
            if (typeof data.priority === 'string') {
                const parsed = parseReminderPriority(data.priority);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid priority: ${parsed.message}`);
                }
            } else if (!Object.values(ReminderPriority).includes(data.priority)) {
                errors.push('Invalid priority value');
            }
        }

        // Status validation
        if (data.status !== undefined) {
            if (typeof data.status === 'string') {
                const parsed = parseReminderStatus(data.status);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid status: ${parsed.message}`);
                }
            } else if (!Object.values(ReminderStatus).includes(data.status)) {
                errors.push('Invalid status value');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Reminder validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Queue item validation
    static validateQueueItemData(data: {
        taskName?: string;
        taskDescription?: string;
        priority?: string | Priority;
        projectId?: string;
        status?: string | QueueStatus;
        estimatedDuration?: number;
        dependencies?: string[];
        tags?: string[];
    }): Result<void> {
        const errors: string[] = [];

        // Required fields
        if (!data.taskName || typeof data.taskName !== 'string' || data.taskName.trim().length === 0) {
            errors.push('Task name is required and must be a non-empty string');
        } else if (data.taskName.length > 200) {
            errors.push('Task name must be less than 200 characters');
        }

        if (!data.taskDescription || typeof data.taskDescription !== 'string' || data.taskDescription.trim().length === 0) {
            errors.push('Task description is required and must be a non-empty string');
        } else if (data.taskDescription.length > 2000) {
            errors.push('Task description must be less than 2000 characters');
        }

        // Priority validation
        if (data.priority !== undefined) {
            if (typeof data.priority === 'string') {
                const parsed = parsePriority(data.priority);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid priority: ${parsed.message}`);
                }
            } else if (!Object.values(Priority).includes(data.priority)) {
                errors.push('Invalid priority value');
            }
        }

        // Status validation
        if (data.status !== undefined) {
            if (typeof data.status === 'string') {
                const parsed = parseQueueStatus(data.status);
                if (parsed instanceof TodoziError) {
                    errors.push(`Invalid status: ${parsed.message}`);
                }
            } else if (!Object.values(QueueStatus).includes(data.status)) {
                errors.push('Invalid status value');
            }
        }

        // Estimated duration validation
        if (data.estimatedDuration !== undefined) {
            if (typeof data.estimatedDuration !== 'number' || data.estimatedDuration <= 0) {
                errors.push('Estimated duration must be a positive number (minutes)');
            } else if (data.estimatedDuration > 10080) { // 1 week in minutes
                errors.push('Estimated duration must be less than 1 week (10080 minutes)');
            }
        }

        // Tags validation
        if (data.tags !== undefined) {
            if (!Array.isArray(data.tags)) {
                errors.push('Tags must be an array');
            } else {
                data.tags.forEach((tag, index) => {
                    if (typeof tag !== 'string' || tag.trim().length === 0) {
                        errors.push(`Tag at index ${index} must be a non-empty string`);
                    } else if (tag.length > 50) {
                        errors.push(`Tag at index ${index} must be less than 50 characters`);
                    }
                });
            }
        }

        // Dependencies validation
        if (data.dependencies !== undefined) {
            if (!Array.isArray(data.dependencies)) {
                errors.push('Dependencies must be an array');
            } else {
                data.dependencies.forEach((dep, index) => {
                    if (typeof dep !== 'string' || dep.trim().length === 0) {
                        errors.push(`Dependency at index ${index} must be a non-empty string`);
                    }
                });
            }
        }

        // Optional fields length validation
        if (data.projectId && data.projectId.length > 100) {
            errors.push('Project ID must be less than 100 characters');
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Queue item validation failed: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Generic ID validation
    static validateId(id: string, fieldName: string = 'ID'): Result<void> {
        if (!id || typeof id !== 'string' || id.trim().length === 0) {
            return {
                ok: false,
                error: TodoziError.validation(`${fieldName} is required and must be a non-empty string`)
            };
        }

        if (id.length > 100) {
            return {
                ok: false,
                error: TodoziError.validation(`${fieldName} must be less than 100 characters`)
            };
        }

        // Basic UUID format validation (optional, can be customized)
        const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
        if (!uuidRegex.test(id)) {
            return {
                ok: false,
                error: TodoziError.validation(`${fieldName} must be a valid UUID format`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Generic email validation
    static validateEmail(email: string): Result<void> {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!email || !emailRegex.test(email)) {
            return {
                ok: false,
                error: TodoziError.validation('Invalid email format')
            };
        }

        if (email.length > 254) { // RFC 5321 limit
            return {
                ok: false,
                error: TodoziError.validation('Email must be less than 254 characters')
            };
        }

        return { ok: true, value: undefined };
    }

    // Generic URL validation
    static validateUrl(url: string): Result<void> {
        try {
            const urlObj = new URL(url);
            if (!['http:', 'https:'].includes(urlObj.protocol)) {
                return {
                    ok: false,
                    error: TodoziError.validation('URL must use HTTP or HTTPS protocol')
                };
            }

            if (url.length > 2000) {
                return {
                    ok: false,
                    error: TodoziError.validation('URL must be less than 2000 characters')
                };
            }

            return { ok: true, value: undefined };
        } catch {
            return {
                ok: false,
                error: TodoziError.validation('Invalid URL format')
            };
        }
    }

    // Batch validation for multiple items
    static async validateBatch<T>(
        items: T[],
        validator: (item: T) => Result<void>,
        maxErrors: number = 10
    ): Promise<Result<{ validCount: number; invalidCount: number; errors: string[] }>> {
        const errors: string[] = [];
        let validCount = 0;
        let invalidCount = 0;

        for (let i = 0; i < items.length; i++) {
            const result = validator(items[i]);
            if (result.ok) {
                validCount++;
            } else {
                invalidCount++;
                if (errors.length < maxErrors) {
                    errors.push(`Item ${i + 1}: ${result.error.message}`);
                }
            }
        }

        if (invalidCount > 0 && errors.length >= maxErrors) {
            errors.push(`... and ${invalidCount - maxErrors} more errors`);
        }

        return {
            ok: true,
            value: { validCount, invalidCount, errors }
        };
    }
}

// Export the validator as a singleton
export const todoziValidator = TodoziValidator;
