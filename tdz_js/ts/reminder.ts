import { generateUUID } from './models.js';
import { DateTime } from 'luxon';

// Enums
export enum ReminderPriority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Urgent = 'urgent'
}

export enum ReminderStatus {
  Pending = 'pending',
  Active = 'active',
  Completed = 'completed',
  Cancelled = 'cancelled'
}

// Interfaces
export interface Reminder {
  id: string;
  content: string;
  remind_at: Date;
  priority: ReminderPriority;
  status: ReminderStatus;
  tags: string[];
  created_at: Date;
  updated_at: Date;
}

export interface ReminderUpdate {
  content?: string;
  remind_at?: Date;
  priority?: ReminderPriority;
  status?: ReminderStatus;
  tags?: string[];
}

export interface ReminderStatistics {
  total_reminders: number;
  pending_reminders: number;
  active_reminders: number;
  overdue_reminders: number;
  unique_tags: number;
}

// Error handling
export class TodoziError extends Error {
  constructor(public message: string, public code?: string) {
    super(message);
    this.name = 'TodoziError';
    Error.captureStackTrace?.(this, TodoziError);
  }

  static notFound(id: string): TodoziError {
    return new TodoziError(`Reminder ${id} not found`, 'NOT_FOUND');
  }

  static validation(message: string): TodoziError {
    return new TodoziError(message, 'VALIDATION_ERROR');
  }
}

// Builder classes
export class ReminderUpdateBuilder {
  private update: ReminderUpdate = {};

  content(content: string): this {
    this.update.content = content;
    return this;
  }

  remindAt(date: Date): this {
    this.update.remind_at = date;
    return this;
  }

  priority(priority: ReminderPriority): this {
    this.update.priority = priority;
    return this;
  }

  status(status: ReminderStatus): this {
    this.update.status = status;
    return this;
  }

  tags(tags: string[]): this {
    this.update.tags = tags;
    return this;
  }

  build(): ReminderUpdate {
    return { ...this.update };
  }
}

export class ReminderBuilder {
  private reminder: Omit<Reminder, 'id' | 'created_at' | 'updated_at'>;

  constructor(content: string, remindAt: Date) {
    if (!content?.trim()) {
      throw TodoziError.validation('Reminder content cannot be empty');
    }
    if (remindAt <= new Date()) {
      throw TodoziError.validation('Reminder date must be in the future');
    }
    
    this.reminder = {
      content: content.trim(),
      remind_at: remindAt,
      priority: ReminderPriority.Medium,
      status: ReminderStatus.Pending,
      tags: []
    };
  }

  priority(priority: ReminderPriority): this {
    this.reminder.priority = priority;
    return this;
  }

  status(status: ReminderStatus): this {
    this.reminder.status = status;
    return this;
  }

  tags(tags: string[]): this {
    this.reminder.tags = tags.map(t => t.trim()).filter(t => t.length > 0);
    return this;
  }

  build(): Omit<Reminder, 'id' | 'created_at' | 'updated_at'> {
    return { ...this.reminder };
  }
}

// Main class
export class ReminderManager {
  private reminders: Map<string, Reminder> = new Map();
  private reminderTags: Map<string, string[]> = new Map();

  /**
   * Creates a new reminder
   * @param reminder - Reminder data without auto-generated fields
   * @returns Promise resolving to the new reminder ID
   */
  public async createReminder(reminder: Omit<Reminder, 'id' | 'created_at' | 'updated_at'>): Promise<string> {
    const id = this.generateId();
    const now = new Date();
    
    const newReminder: Reminder = {
      ...reminder,
      id,
      created_at: now,
      updated_at: now
    };

    this.reminderTags.set(id, [...reminder.tags]);
    this.reminders.set(id, newReminder);
    
    return id;
  }

  /**
   * Retrieves a reminder by ID (immutable copy)
   * @param reminderId - The reminder ID
   * @returns Readonly reminder or undefined
   */
  public getReminder(reminderId: string): Readonly<Reminder> | undefined {
    const reminder = this.reminders.get(reminderId);
    return reminder ? Object.freeze({ ...reminder }) : undefined;
  }

  /**
   * Retrieves all reminders
   * @returns Array of readonly reminders
   */
  public getAllReminders(): Readonly<Reminder>[] {
    return Array.from(this.reminders.values()).map(r => Object.freeze({ ...r }));
  }

  /**
   * Updates a reminder with provided changes
   * @param reminderId - The reminder ID
   * @param updates - Partial reminder updates
   * @returns Promise resolving when update is complete
   */
  public async updateReminder(reminderId: string, updates: ReminderUpdate): Promise<void> {
    const reminder = this.reminders.get(reminderId);
    if (!reminder) {
      throw TodoziError.notFound(reminderId);
    }

    if (updates.content !== undefined) {
      if (!updates.content?.trim()) {
        throw TodoziError.validation('Reminder content cannot be empty');
      }
      reminder.content = updates.content.trim();
    }

    if (updates.remind_at !== undefined) {
      if (updates.remind_at <= new Date()) {
        throw TodoziError.validation('Reminder date must be in the future');
      }
      reminder.remind_at = updates.remind_at;
    }

    if (updates.priority !== undefined) {
      reminder.priority = updates.priority;
    }

    if (updates.status !== undefined) {
      reminder.status = updates.status;
    }

    if (updates.tags !== undefined) {
      reminder.tags = updates.tags.map(t => t.trim()).filter(t => t.length > 0);
      this.reminderTags.set(reminderId, [...reminder.tags]);
    }

    reminder.updated_at = new Date();
  }

  /**
   * Deletes a reminder
   * @param reminderId - The reminder ID
   * @returns Promise resolving when deletion is complete
   */
  public async deleteReminder(reminderId: string): Promise<void> {
    const deleted = this.reminders.delete(reminderId);
    if (!deleted) {
      throw TodoziError.notFound(reminderId);
    }
    this.reminderTags.delete(reminderId);
  }

  /**
   * Searches reminders by content or tags
   * @param query - Search query
   * @returns Array of matching reminders
   */
  public searchReminders(query: string): Readonly<Reminder>[] {
    if (!query?.trim()) {
      return [];
    }

    const queryLower = query.toLowerCase().trim();
    const regex = new RegExp(queryLower.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'i');

    return Array.from(this.reminders.values())
      .filter(reminder => 
        regex.test(reminder.content) ||
        reminder.tags.some(tag => regex.test(tag))
      )
      .map(r => Object.freeze({ ...r }));
  }

  /**
   * Gets reminders by priority level
   * @param priority - Priority level to filter by
   * @returns Array of reminders with specified priority
   */
  public getRemindersByPriority(priority: ReminderPriority): Readonly<Reminder>[] {
    return Array.from(this.reminders.values())
      .filter(r => r.priority === priority)
      .map(r => Object.freeze({ ...r }));
  }

  /**
   * Gets reminders by status
   * @param status - Status to filter by
   * @returns Array of reminders with specified status
   */
  public getRemindersByStatus(status: ReminderStatus): Readonly<Reminder>[] {
    return Array.from(this.reminders.values())
      .filter(r => r.status === status)
      .map(r => Object.freeze({ ...r }));
  }

  /**
   * Gets all pending reminders
   * @returns Array of pending reminders
   */
  public getPendingReminders(): Readonly<Reminder>[] {
    return this.getRemindersByStatus(ReminderStatus.Pending);
  }

  /**
   * Gets all active reminders
   * @returns Array of active reminders
   */
  public getActiveReminders(): Readonly<Reminder>[] {
    return this.getRemindersByStatus(ReminderStatus.Active);
  }

  /**
   * Gets all overdue reminders
   * @returns Array of overdue reminders
   */
  public getOverdueReminders(): Readonly<Reminder>[] {
    const now = new Date();
    return Array.from(this.reminders.values())
      .filter(reminder => 
        reminder.remind_at < now &&
        (reminder.status === ReminderStatus.Pending || 
         reminder.status === ReminderStatus.Active)
      )
      .map(r => Object.freeze({ ...r }));
  }

  /**
   * Gets reminders due within specified duration
   * @param duration - Duration in milliseconds
   * @returns Array of reminders due soon
   */
  public getRemindersDueSoon(duration: number): Readonly<Reminder>[] {
    const now = new Date();
    const dueTime = new Date(now.getTime() + duration);
    
    return Array.from(this.reminders.values())
      .filter(reminder => 
        reminder.remind_at <= dueTime && 
        reminder.remind_at > now &&
        (reminder.status === ReminderStatus.Pending || 
         reminder.status === ReminderStatus.Active)
      )
      .map(r => Object.freeze({ ...r }));
  }

  /**
   * Gets most recent reminders
   * @param limit - Maximum number of reminders to return
   * @returns Array of recent reminders
   */
  public getRecentReminders(limit: number): Readonly<Reminder>[] {
    return Array.from(this.reminders.values())
      .sort((a, b) => b.created_at.getTime() - a.created_at.getTime())
      .slice(0, limit)
      .map(r => Object.freeze({ ...r }));
  }

  /**
   * Gets all unique tags
   * @returns Array of unique tag strings
   */
  public getAllTags(): string[] {
    const allTags = new Set<string>();
    for (const tags of this.reminderTags.values()) {
      for (const tag of tags) {
        if (tag.trim()) {
          allTags.add(tag.trim());
        }
      }
    }
    return Array.from(allTags).sort();
  }

  /**
   * Gets tag usage statistics
   * @returns Map of tag names to usage counts
   */
  public getTagStatistics(): Map<string, number> {
    const stats = new Map<string, number>();
    for (const tags of this.reminderTags.values()) {
      for (const tag of tags) {
        const trimmedTag = tag.trim();
        if (trimmedTag) {
          stats.set(trimmedTag, (stats.get(trimmedTag) || 0) + 1);
        }
      }
    }
    return stats;
  }

  /**
   * Gets reminder statistics
   * @returns ReminderStatistics object
   */
  public getReminderStatistics(): ReminderStatistics {
    const totalReminders = this.reminders.size;
    const pending = this.getPendingReminders().length;
    const active = this.getActiveReminders().length;
    const overdue = this.getOverdueReminders().length;
    const uniqueTags = this.getAllTags().length;

    return {
      total_reminders: totalReminders,
      pending_reminders: pending,
      active_reminders: active,
      overdue_reminders: overdue,
      unique_tags
    };
  }

  /**
   * Marks a reminder as completed
   * @param reminderId - The reminder ID
   * @returns Promise resolving when update is complete
   */
  public async markReminderCompleted(reminderId: string): Promise<void> {
    const reminder = this.reminders.get(reminderId);
    if (!reminder) {
      throw TodoziError.notFound(reminderId);
    }
    reminder.status = ReminderStatus.Completed;
    reminder.updated_at = new Date();
  }

  /**
   * Marks a reminder as cancelled
   * @param reminderId - The reminder ID
   * @returns Promise resolving when update is complete
   */
  public async markReminderCancelled(reminderId: string): Promise<void> {
    const reminder = this.reminders.get(reminderId);
    if (!reminder) {
      throw TodoziError.notFound(reminderId);
    }
    reminder.status = ReminderStatus.Cancelled;
    reminder.updated_at = new Date();
  }

  /**
   * Activates a reminder
   * @param reminderId - The reminder ID
   * @returns Promise resolving when update is complete
   */
  public async activateReminder(reminderId: string): Promise<void> {
    const reminder = this.reminders.get(reminderId);
    if (!reminder) {
      throw TodoziError.notFound(reminderId);
    }
    reminder.status = ReminderStatus.Active;
    reminder.updated_at = new Date();
  }

  /**
   * Generates a UUID v4
   * @returns UUID string
   */
  private generateId(): string {
    return generateUUID();
  }
}

// Helper class for statistics calculations
export class ReminderStatisticsHelper {
  static pendingPercentage(stats: ReminderStatistics): number {
    if (stats.total_reminders === 0) return 0;
    return (stats.pending_reminders / stats.total_reminders) * 100;
  }

  static activePercentage(stats: ReminderStatistics): number {
    if (stats.total_reminders === 0) return 0;
    return (stats.active_reminders / stats.total_reminders) * 100;
  }

  static overduePercentage(stats: ReminderStatistics): number {
    if (stats.total_reminders === 0) return 0;
    return (stats.overdue_reminders / stats.total_reminders) * 100;
  }
}

// Parsing function
export function parseReminderFormat(reminderText: string): Omit<Reminder, 'id' | 'created_at' | 'updated_at'> {
  const startTag = '<reminder>';
  const endTag = '</reminder>';
  
  const start = reminderText.indexOf(startTag);
  const end = reminderText.indexOf(endTag);
  
  if (start === -1) {
    throw TodoziError.validation('Missing <reminder> start tag');
  }
  if (end === -1) {
    throw TodoziError.validation('Missing </reminder> end tag');
  }
  
  const content = reminderText.substring(start + startTag.length, end);
  const parts = content.split(';').map(s => s.trim()).filter(s => s.length > 0);
  
  if (parts.length < 3) {
    throw TodoziError.validation(
      'Invalid reminder format: need at least 3 parts (content; remind_at; priority)'
    );
  }
  
  const remindAt = DateTime.fromISO(parts[1]).toJSDate();
  if (!remindAt || isNaN(remindAt.getTime())) {
    throw TodoziError.validation('Invalid reminder date format');
  }
  
  const priority = parts[2].toLowerCase() as ReminderPriority;
  if (!Object.values(ReminderPriority).includes(priority)) {
    throw TodoziError.validation('Invalid reminder priority');
  }
  
  const status = parts.length > 3 && parts[3] 
    ? parts[3].toLowerCase() as ReminderStatus
    : ReminderStatus.Pending;
    
  if (!Object.values(ReminderStatus).includes(status)) {
    throw TodoziError.validation('Invalid reminder status');
  }
  
  const tags = parts.length > 4 && parts[4]
    ? parts[4].split(',').map(s => s.trim()).filter(s => s.length > 0)
    : [];
  
  return {
    content: parts[0],
    remind_at: remindAt,
    priority,
    status,
    tags
  };
}

// Example usage:
/*
const manager = new ReminderManager();

// Create a reminder using the builder
const reminderData = new ReminderBuilder('Review project proposal', new Date('2025-01-20T10:00:00Z'))
  .priority(ReminderPriority.High)
  .tags(['review', 'project', 'deadline'])
  .build();

const reminderId = await manager.createReminder(reminderData);

// Update using the builder
const update = new ReminderUpdateBuilder()
  .content('Updated content')
  .status(ReminderStatus.Active)
  .build();

await manager.updateReminder(reminderId, update);

// Get statistics
const stats = manager.getReminderStatistics();
console.log(`Pending: ${ReminderStatisticsHelper.pendingPercentage(stats)}%`);
*/
