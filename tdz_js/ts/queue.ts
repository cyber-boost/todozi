import { Priority, Status, Task } from './models.js';
import { TodoziError } from './error.js';

// Queue item status
export enum QueueStatus {
    Backlog = 'backlog',
    Active = 'active',
    Complete = 'complete',
    Paused = 'paused'
}

// Queue item interface
export interface QueueItem {
    id: string;
    taskName: string;
    taskDescription: string;
    priority: Priority;
    projectId?: string;
    status: QueueStatus;
    createdAt: Date;
    updatedAt: Date;
    startedAt?: Date;
    completedAt?: Date;
    estimatedDuration?: number; // minutes
    actualDuration?: number; // minutes
    dependencies: string[]; // task IDs that must be completed first
    tags: string[];
}

// Queue session for tracking work time
export interface QueueSession {
    id: string;
    queueItemId: string;
    startTime: Date;
    endTime?: Date;
    durationSeconds?: number;
    notes?: string;
}

// Queue statistics
export interface QueueStats {
    totalItems: number;
    backlogCount: number;
    activeCount: number;
    completedCount: number;
    pausedCount: number;
    averageCompletionTime: number;
    totalTimeSpent: number;
}

// Queue manager class
export class QueueManager {
    private items: Map<string, QueueItem> = new Map();
    private sessions: Map<string, QueueSession> = new Map();
    private activeSession: QueueSession | null = null;

    constructor() {}

    // Queue item management
    addItem(item: Omit<QueueItem, 'id' | 'createdAt' | 'updatedAt'>): string {
        const id = this.generateId();
        const now = new Date();

        const queueItem: QueueItem = {
            ...item,
            id,
            createdAt: now,
            updatedAt: now
        };

        this.items.set(id, queueItem);
        return id;
    }

    getItem(id: string): QueueItem | undefined {
        return this.items.get(id);
    }

    updateItem(id: string, updates: Partial<QueueItem>): void {
        const item = this.items.get(id);
        if (!item) {
            throw TodoziError.validation(`Queue item ${id} not found`);
        }

        this.items.set(id, {
            ...item,
            ...updates,
            updatedAt: new Date()
        });
    }

    removeItem(id: string): boolean {
        return this.items.delete(id);
    }

    // Status management
    startItem(id: string): void {
        const item = this.items.get(id);
        if (!item) {
            throw TodoziError.validation(`Queue item ${id} not found`);
        }

        if (item.status === QueueStatus.Complete) {
            throw TodoziError.validation(`Cannot start completed item ${id}`);
        }

        this.updateItem(id, {
            status: QueueStatus.Active,
            startedAt: new Date()
        });

        // Start a session
        this.startSession(id);
    }

    pauseItem(id: string): void {
        const item = this.items.get(id);
        if (!item) {
            throw TodoziError.validation(`Queue item ${id} not found`);
        }

        if (item.status !== QueueStatus.Active) {
            throw TodoziError.validation(`Cannot pause item ${id} that is not active`);
        }

        this.updateItem(id, { status: QueueStatus.Paused });

        // End current session
        if (this.activeSession && this.activeSession.queueItemId === id) {
            this.endSession();
        }
    }

    completeItem(id: string): void {
        const item = this.items.get(id);
        if (!item) {
            throw TodoziError.validation(`Queue item ${id} not found`);
        }

        const completedAt = new Date();
        const actualDuration = item.startedAt
            ? Math.round((completedAt.getTime() - item.startedAt.getTime()) / (1000 * 60))
            : undefined;

        this.updateItem(id, {
            status: QueueStatus.Complete,
            completedAt,
            actualDuration
        });

        // End current session
        if (this.activeSession && this.activeSession.queueItemId === id) {
            this.endSession();
        }
    }

    // Session management
    startSession(queueItemId: string): string {
        // End any active session
        if (this.activeSession) {
            this.endSession();
        }

        const sessionId = this.generateId();
        const session: QueueSession = {
            id: sessionId,
            queueItemId,
            startTime: new Date()
        };

        this.sessions.set(sessionId, session);
        this.activeSession = session;

        return sessionId;
    }

    endSession(): void {
        if (!this.activeSession) {
            return;
        }

        const endTime = new Date();
        const durationSeconds = Math.round(
            (endTime.getTime() - this.activeSession.startTime.getTime()) / 1000
        );

        this.activeSession.endTime = endTime;
        this.activeSession.durationSeconds = durationSeconds;

        this.sessions.set(this.activeSession.id, this.activeSession);
        this.activeSession = null;
    }

    getActiveSession(): QueueSession | null {
        return this.activeSession;
    }

    getSession(id: string): QueueSession | undefined {
        return this.sessions.get(id);
    }

    // Query methods
    getAllItems(): QueueItem[] {
        return Array.from(this.items.values());
    }

    getItemsByStatus(status: QueueStatus): QueueItem[] {
        return this.getAllItems().filter(item => item.status === status);
    }

    getBacklogItems(): QueueItem[] {
        return this.getItemsByStatus(QueueStatus.Backlog);
    }

    getActiveItems(): QueueItem[] {
        return this.getItemsByStatus(QueueStatus.Active);
    }

    getCompletedItems(): QueueItem[] {
        return this.getItemsByStatus(QueueStatus.Complete);
    }

    getItemsByPriority(priority: Priority): QueueItem[] {
        return this.getAllItems().filter(item => item.priority === priority);
    }

    getItemsByProject(projectId: string): QueueItem[] {
        return this.getAllItems().filter(item => item.projectId === projectId);
    }

    getNextItem(): QueueItem | null {
        // Get highest priority backlog item
        const backlogItems = this.getBacklogItems();

        if (backlogItems.length === 0) {
            return null;
        }

        // Sort by priority (higher priority first)
        const priorityOrder = {
            [Priority.Urgent]: 5,
            [Priority.Critical]: 4,
            [Priority.High]: 3,
            [Priority.Medium]: 2,
            [Priority.Low]: 1
        };

        return backlogItems.sort((a, b) =>
            priorityOrder[b.priority] - priorityOrder[a.priority]
        )[0];
    }

    // Statistics
    getStats(): QueueStats {
        const allItems = this.getAllItems();
        const completedItems = this.getCompletedItems();

        const totalTimeSpent = completedItems.reduce(
            (total, item) => total + (item.actualDuration || 0),
            0
        );

        const averageCompletionTime = completedItems.length > 0
            ? totalTimeSpent / completedItems.length
            : 0;

        return {
            totalItems: allItems.length,
            backlogCount: this.getBacklogItems().length,
            activeCount: this.getActiveItems().length,
            completedCount: completedItems.length,
            pausedCount: this.getItemsByStatus(QueueStatus.Paused).length,
            averageCompletionTime,
            totalTimeSpent
        };
    }

    // Session queries
    getSessionsForItem(queueItemId: string): QueueSession[] {
        return Array.from(this.sessions.values())
            .filter(session => session.queueItemId === queueItemId)
            .sort((a, b) => a.startTime.getTime() - b.startTime.getTime());
    }

    getTotalTimeForItem(queueItemId: string): number {
        return this.getSessionsForItem(queueItemId)
            .reduce((total, session) => total + (session.durationSeconds || 0), 0);
    }

    // Utility methods
    private generateId(): string {
        return `queue_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }

    clear(): void {
        this.items.clear();
        this.sessions.clear();
        this.activeSession = null;
    }

    // Serialization
    toJSON(): any {
        return {
            items: Array.from(this.items.entries()),
            sessions: Array.from(this.sessions.entries()),
            activeSession: this.activeSession
        };
    }

    static fromJSON(data: any): QueueManager {
        const manager = new QueueManager();

        // Restore items
        for (const [id, item] of data.items || []) {
            manager.items.set(id, {
                ...item,
                createdAt: new Date(item.createdAt),
                updatedAt: new Date(item.updatedAt),
                startedAt: item.startedAt ? new Date(item.startedAt) : undefined,
                completedAt: item.completedAt ? new Date(item.completedAt) : undefined
            });
        }

        // Restore sessions
        for (const [id, session] of data.sessions || []) {
            manager.sessions.set(id, {
                ...session,
                startTime: new Date(session.startTime),
                endTime: session.endTime ? new Date(session.endTime) : undefined
            });
        }

        manager.activeSession = data.activeSession || null;

        return manager;
    }
}

export default QueueManager;
