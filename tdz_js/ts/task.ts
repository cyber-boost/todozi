import {
    Task,
    TaskClass,
    TaskUpdate,
    TaskUpdateBuilder,
    Priority,
    Status,
    Assignee,
    TodoziError,
    Result,
    parsePriority,
    parseStatus,
    parseAssignee
} from './models.js';
import { DateUtils, ArrayUtils } from './utils.js';

// Task management functionality
export class TaskManager {
    private tasks: Map<string, Task> = new Map();

    constructor() {
        this.tasks = new Map();
    }

    static new(): TaskManager {
        return new TaskManager();
    }

    async createTask(taskData: {
        action: string;
        time?: string;
        priority?: Priority | string;
        parentProject?: string;
        status?: Status | string;
        assignee?: Assignee | string;
        tags?: string[];
        dependencies?: string[];
        contextNotes?: string;
        progress?: number;
    }): Promise<Result<string>> {
        try {
            // Parse and validate inputs
            let priority: Priority = Priority.Medium;
            if (taskData.priority !== undefined) {
                if (typeof taskData.priority === 'string') {
                    const parsed = parsePriority(taskData.priority);
                    if (parsed instanceof TodoziError) return parsed;
                    priority = parsed;
                } else {
                    priority = taskData.priority;
                }
            }

            let status: Status = Status.Todo;
            if (taskData.status !== undefined) {
                if (typeof taskData.status === 'string') {
                    const parsed = parseStatus(taskData.status);
                    if (parsed instanceof TodoziError) return parsed;
                    status = parsed;
                } else {
                    status = taskData.status;
                }
            }

            let assignee: Assignee | undefined;
            if (taskData.assignee !== undefined) {
                if (typeof taskData.assignee === 'string') {
                    const parsed = parseAssignee(taskData.assignee);
                    if (parsed instanceof TodoziError) return parsed;
                    assignee = parsed;
                } else {
                    assignee = taskData.assignee;
                }
            }

            const task = TaskClass.newFull(
                'system', // userId - should be configurable
                taskData.action,
                taskData.time || '1h',
                priority,
                taskData.parentProject || 'general',
                status,
                assignee,
                taskData.tags || [],
                taskData.dependencies || [],
                taskData.contextNotes,
                taskData.progress
            );

            if (task instanceof TodoziError) return task;

            this.tasks.set(task.id, task);
            return { ok: true, value: task.id };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Task creation failed')
            };
        }
    }

    getTask(taskId: string): Task | undefined {
        return this.tasks.get(taskId);
    }

    getAllTasks(): Task[] {
        return Array.from(this.tasks.values());
    }

    async updateTask(taskId: string, updates: TaskUpdate): Promise<Result<void>> {
        const task = this.tasks.get(taskId);
        if (!task) {
            return {
                ok: false,
                error: TodoziError.notFound(`Task ${taskId} not found`)
            };
        }

        try {
            const taskClass = task as TaskClass;
            const result = taskClass.update(updates);

            if (result instanceof TodoziError) return result;

            this.tasks.set(taskId, taskClass);
            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Task update failed')
            };
        }
    }

    async deleteTask(taskId: string): Promise<Result<void>> {
        if (!this.tasks.has(taskId)) {
            return {
                ok: false,
                error: TodoziError.notFound(`Task ${taskId} not found`)
            };
        }

        this.tasks.delete(taskId);
        return { ok: true, value: undefined };
    }

    async completeTask(taskId: string): Promise<Result<void>> {
        const task = this.tasks.get(taskId);
        if (!task) {
            return {
                ok: false,
                error: TodoziError.notFound(`Task ${taskId} not found`)
            };
        }

        const taskClass = task as TaskClass;
        taskClass.complete();
        this.tasks.set(taskId, taskClass);

        return { ok: true, value: undefined };
    }

    async startTask(taskId: string): Promise<Result<void>> {
        return await this.updateTask(taskId, { status: Status.InProgress });
    }

    async blockTask(taskId: string, reason?: string): Promise<Result<void>> {
        const update: TaskUpdate = { status: Status.Blocked };
        if (reason) {
            update.contextNotes = reason;
        }
        return await this.updateTask(taskId, update);
    }

    // Query methods
    getTasksByStatus(status: Status): Task[] {
        return Array.from(this.tasks.values()).filter(task => task.status === status);
    }

    getTasksByPriority(priority: Priority): Task[] {
        return Array.from(this.tasks.values()).filter(task => task.priority === priority);
    }

    getTasksByProject(project: string): Task[] {
        return Array.from(this.tasks.values()).filter(task => task.parentProject === project);
    }

    getTasksByAssignee(assignee: Assignee): Task[] {
        return Array.from(this.tasks.values()).filter(task => {
            if (!task.assignee) return false;
            return task.assignee.type === assignee.type &&
                   (assignee.type !== 'agent' || task.assignee.name === assignee.name);
        });
    }

    getTasksByTag(tag: string): Task[] {
        return Array.from(this.tasks.values()).filter(task => task.tags.includes(tag));
    }

    getCompletedTasks(): Task[] {
        return this.getTasksByStatus(Status.Done);
    }

    getActiveTasks(): Task[] {
        return Array.from(this.tasks.values()).filter(task =>
            task.status === Status.InProgress ||
            task.status === Status.Todo ||
            task.status === Status.Review
        );
    }

    getBlockedTasks(): Task[] {
        return this.getTasksByStatus(Status.Blocked);
    }

    getOverdueTasks(): Task[] {
        const now = new Date();
        return Array.from(this.tasks.values()).filter(task => {
            // Simple overdue check - could be enhanced with proper date parsing
            return task.status !== Status.Done && task.status !== Status.Cancelled;
        });
    }

    getHighPriorityTasks(): Task[] {
        return Array.from(this.tasks.values()).filter(task =>
            task.priority === Priority.Urgent ||
            task.priority === Priority.Critical ||
            task.priority === Priority.High
        );
    }

    getRecentTasks(limit: number = 10): Task[] {
        return Array.from(this.tasks.values())
            .sort((a, b) => b.updatedAt.getTime() - a.updatedAt.getTime())
            .slice(0, limit);
    }

    getTasksByDateRange(from: Date, to: Date): Task[] {
        return Array.from(this.tasks.values()).filter(task =>
            task.createdAt >= from && task.createdAt <= to
        );
    }

    // Search functionality
    searchTasks(query: string): Task[] {
        const lowerQuery = query.toLowerCase();
        return Array.from(this.tasks.values()).filter(task =>
            task.action.toLowerCase().includes(lowerQuery) ||
            task.parentProject.toLowerCase().includes(lowerQuery) ||
            task.contextNotes?.toLowerCase().includes(lowerQuery) ||
            task.tags.some(tag => tag.toLowerCase().includes(lowerQuery))
        );
    }

    // Statistics
    getTaskStatistics(): {
        totalTasks: number;
        completedTasks: number;
        activeTasks: number;
        blockedTasks: number;
        completionRate: number;
        tasksByPriority: Record<Priority, number>;
        tasksByStatus: Record<Status, number>;
        tasksByProject: Record<string, number>;
        averageTasksPerProject: number;
        totalTags: number;
    } {
        const tasks = Array.from(this.tasks.values());
        const totalTasks = tasks.length;
        const completedTasks = tasks.filter(t => t.status === Status.Done).length;
        const activeTasks = tasks.filter(t =>
            t.status === Status.InProgress ||
            t.status === Status.Todo ||
            t.status === Status.Review
        ).length;
        const blockedTasks = tasks.filter(t => t.status === Status.Blocked).length;

        const tasksByPriority = {
            [Priority.Low]: 0,
            [Priority.Medium]: 0,
            [Priority.High]: 0,
            [Priority.Critical]: 0,
            [Priority.Urgent]: 0
        };

        const tasksByStatus = {
            [Status.Todo]: 0,
            [Status.InProgress]: 0,
            [Status.Blocked]: 0,
            [Status.Review]: 0,
            [Status.Done]: 0,
            [Status.Cancelled]: 0,
            [Status.Deferred]: 0,
            [Status.Completed]: 0
        };

        const tasksByProject: Record<string, number> = {};
        const allTags = new Set<string>();

        tasks.forEach(task => {
            tasksByPriority[task.priority]++;
            tasksByStatus[task.status]++;

            const project = task.parentProject;
            tasksByProject[project] = (tasksByProject[project] || 0) + 1;

            task.tags.forEach(tag => allTags.add(tag));
        });

        const projectCount = Object.keys(tasksByProject).length;

        return {
            totalTasks,
            completedTasks,
            activeTasks,
            blockedTasks,
            completionRate: totalTasks > 0 ? (completedTasks / totalTasks) * 100 : 0,
            tasksByPriority,
            tasksByStatus,
            tasksByProject,
            averageTasksPerProject: projectCount > 0 ? totalTasks / projectCount : 0,
            totalTags: allTags.size
        };
    }

    // Bulk operations
    async bulkCreateTasks(taskData: Array<{
        action: string;
        time?: string;
        priority?: Priority | string;
        parentProject?: string;
        status?: Status | string;
        assignee?: Assignee | string;
        tags?: string[];
        dependencies?: string[];
        contextNotes?: string;
        progress?: number;
    }>): Promise<Result<string[]>> {
        const results: string[] = [];
        const errors: TodoziError[] = [];

        for (const data of taskData) {
            const result = await this.createTask(data);
            if (result.ok) {
                results.push(result.value);
            } else {
                errors.push(result.error);
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Bulk creation failed: ${errors.length} errors`)
            };
        }

        return { ok: true, value: results };
    }

    async bulkUpdateTasks(updates: Array<{ id: string; updates: TaskUpdate }>): Promise<Result<void>> {
        const errors: TodoziError[] = [];

        for (const update of updates) {
            const result = await this.updateTask(update.id, update.updates);
            if (!result.ok) {
                errors.push(result.error);
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Bulk update failed: ${errors.length} errors`)
            };
        }

        return { ok: true, value: undefined };
    }

    async bulkDeleteTasks(taskIds: string[]): Promise<Result<void>> {
        const errors: string[] = [];

        for (const id of taskIds) {
            const result = await this.deleteTask(id);
            if (!result.ok) {
                errors.push(id);
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Bulk deletion failed for IDs: ${errors.join(', ')}`)
            };
        }

        return { ok: true, value: undefined };
    }

    // Export/Import
    exportToJSON(): string {
        const tasks = Array.from(this.tasks.values()).map(task => ({
            id: task.id,
            userId: task.userId,
            action: task.action,
            time: task.time,
            priority: task.priority,
            parentProject: task.parentProject,
            status: task.status,
            assignee: task.assignee,
            tags: task.tags,
            dependencies: task.dependencies,
            contextNotes: task.contextNotes,
            progress: task.progress,
            embeddingVector: task.embeddingVector,
            createdAt: task.createdAt.toISOString(),
            updatedAt: task.updatedAt.toISOString()
        }));

        return JSON.stringify(tasks, null, 2);
    }

    async importFromJSON(jsonData: string): Promise<Result<number>> {
        try {
            const tasks = JSON.parse(jsonData);
            if (!Array.isArray(tasks)) {
                return {
                    ok: false,
                    error: TodoziError.validation('Invalid JSON format: expected array')
                };
            }

            let importedCount = 0;
            for (const taskData of tasks) {
                // Create a new task without ID to generate a new one
                const { id, createdAt, updatedAt, ...taskWithoutMeta } = taskData;
                const result = await this.createTask(taskWithoutMeta);
                if (result.ok) {
                    importedCount++;
                }
            }

            return { ok: true, value: importedCount };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'JSON import failed')
            };
        }
    }

    clear(): void {
        this.tasks.clear();
    }
}

// Task builder for fluent API
export class TaskBuilder {
    private data: Partial<{
        action: string;
        time: string;
        priority: Priority;
        parentProject: string;
        status: Status;
        assignee: Assignee;
        tags: string[];
        dependencies: string[];
        contextNotes: string;
        progress: number;
    }> = {};

    static new(): TaskBuilder {
        return new TaskBuilder();
    }

    action(action: string): this {
        this.data.action = action;
        return this;
    }

    time(time: string): this {
        this.data.time = time;
        return this;
    }

    priority(priority: Priority): this {
        this.data.priority = priority;
        return this;
    }

    project(project: string): this {
        this.data.parentProject = project;
        return this;
    }

    status(status: Status): this {
        this.data.status = status;
        return this;
    }

    assignee(assignee: Assignee): this {
        this.data.assignee = assignee;
        return this;
    }

    tags(tags: string[]): this {
        this.data.tags = tags;
        return this;
    }

    addTag(tag: string): this {
        if (!this.data.tags) this.data.tags = [];
        this.data.tags.push(tag);
        return this;
    }

    dependencies(dependencies: string[]): this {
        this.data.dependencies = dependencies;
        return this;
    }

    addDependency(depId: string): this {
        if (!this.data.dependencies) this.data.dependencies = [];
        this.data.dependencies.push(depId);
        return this;
    }

    context(context: string): this {
        this.data.contextNotes = context;
        return this;
    }

    progress(progress: number): this {
        this.data.progress = progress;
        return this;
    }

    async build(manager: TaskManager): Promise<Result<string>> {
        if (!this.data.action) {
            return {
                ok: false,
                error: TodoziError.validation('Task action is required')
            };
        }

        return await manager.createTask(this.data as any);
    }
}

// Export singleton instance
export const taskManager = TaskManager.new();
