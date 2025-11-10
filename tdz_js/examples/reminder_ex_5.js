import { v4: uuidv4 } from 'uuid.js';

Below is a runnable Node.js example that demonstrates realistic usage of reminder.js by extending the code with a minimal Reminder model and a short demo. It shows:
- Creating reminders (manual and via parseReminderFormat)
- Updating reminders with ReminderUpdate builder
- Querying and filtering (status, priority, tags)
- Marking reminders complete/cancelled
- Getting overdue and due-soon reminders
- Getting statistics and percentages

How to run:
- Save this file as example5.js
- Run: node example5.js

Code:
javascript
// example5.js
// Demonstrates usage/extension of reminder.js with a practical scenario.


// Minimal Reminder class to satisfy ReminderManager usages.
class Reminder {
  constructor({ id, content, remind_at, priority, status = 'pending', tags = [] }) {
    this.id = id || uuidv4();
    this.content = content;
    this.remind_at = new Date(remind_at);
    this.priority = priority; // 'low' | 'medium' | 'high' | 'critical'
    this.status = status; // 'pending' | 'active' | 'completed' | 'cancelled'
    this.tags = Array.isArray(tags) ? [...tags] : [];
    this.created_at = new Date();
    this.updated_at = new Date();
  }

  markCompleted() {
    this.status = 'completed';
    this.updated_at = new Date();
  }

  markCancelled() {
    this.status = 'cancelled';
    this.updated_at = new Date();
  }

  activate() {
    this.status = 'active';
    this.updated_at = new Date();
  }
}

// Minimal ReminderStatus for this example
const ReminderStatus = {
  Pending: 'pending',
  Active: 'active',
  Completed: 'completed',
  Cancelled: 'cancelled',
};

// Extended reminder.js content (since reminder.js exists but lacks the Reminder class)
class ReminderManagerExt {
  constructor() {
    this.reminders = new Map();
    this.reminder_tags = new Map();
  }

  async createReminder(reminder) {
    const r = new Reminder(reminder);
    r.id = r.id || uuidv4();
    r.created_at = new Date();
    r.updated_at = new Date();
    this.reminder_tags.set(r.id, [...r.tags]);
    this.reminders.set(r.id, r);
    return r.id;
  }

  getReminder(reminderId) {
    return this.reminders.get(reminderId);
  }

  getAllReminders() {
    return Array.from(this.reminders.values());
  }

  async updateReminder(reminderId, updates) {
    const reminder = this.reminders.get(reminderId);
    if (!reminder) throw new Error(`Reminder ${reminderId} not found`);

    if (updates.content !== undefined) reminder.content = updates.content;
    if (updates.remind_at !== undefined) reminder.remind_at = new Date(updates.remind_at);
    if (updates.priority !== undefined) reminder.priority = updates.priority;
    if (updates.status !== undefined) reminder.status = updates.status;
    if (updates.tags !== undefined) {
      reminder.tags = [...updates.tags];
      this.reminder_tags.set(reminderId, [...updates.tags]);
    }
    reminder.updated_at = new Date();
  }

  async deleteReminder(reminderId) {
    if (!this.reminders.delete(reminderId)) {
      throw new Error(`Reminder ${reminderId} not found`);
    }
    this.reminder_tags.delete(reminderId);
  }

  searchReminders(query) {
    const q = query.toLowerCase();
    return this.getAllReminders().filter(rem =>
      rem.content.toLowerCase().includes(q) ||
      rem.tags.some(tag => tag.toLowerCase().includes(q))
    );
  }

  getRemindersByPriority(priority) {
    return this.getAllReminders().filter(r => r.priority === priority);
  }

  getRemindersByStatus(status) {
    return this.getAllReminders().filter(r => r.status === status);
  }

  getRemindersByTag(tag) {
    const tagLower = tag.toLowerCase();
    return this.getAllReminders().filter(r =>
      r.tags.some(t => t.toLowerCase() === tagLower)
    );
  }

  getPendingReminders() {
    return this.getRemindersByStatus(ReminderStatus.Pending);
  }

  getActiveReminders() {
    return this.getRemindersByStatus(ReminderStatus.Active);
  }

  getOverdueReminders() {
    const now = new Date();
    return this.getAllReminders().filter(r =>
      r.remind_at < now &&
      (r.status === ReminderStatus.Pending || r.status === ReminderStatus.Active)
    );
  }

  getRemindersDueSoon(durationMs) {
    const now = new Date();
    const dueTime = new Date(now.getTime() + durationMs);
    return this.getAllReminders().filter(r =>
      r.remind_at <= dueTime && r.remind_at > now &&
      (r.status === ReminderStatus.Pending || r.status === ReminderStatus.Active)
    );
  }

  getRecentReminders(limit) {
    return this.getAllReminders()
      .sort((a, b) => b.created_at - a.created_at)
      .slice(0, limit);
  }

  getAllTags() {
    const set = new Set();
    for (const tags of this.reminder_tags.values()) {
      for (const tag of tags) set.add(tag);
    }
    return Array.from(set);
  }

  getTagStatistics() {
    const stats = new Map();
    for (const tags of this.reminder_tags.values()) {
      for (const tag of tags) {
        stats.set(tag, (stats.get(tag) || 0) + 1);
      }
    }
    return Object.fromEntries(stats);
  }

  getReminderStatistics() {
    const total = this.reminders.size;
    const pending = this.getPendingReminders().length;
    const active = this.getActiveReminders().length;
    const overdue = this.getOverdueReminders().length;
    const uniqueTags = this.getAllTags().length;
    return {
      total_reminders: total,
      pending_reminders: pending,
      active_reminders: active,
      overdue_reminders: overdue,
      unique_tags: uniqueTags,
      pendingPercentage: () => (total === 0 ? 0 : (pending / total) * 100),
      activePercentage: () => (total === 0 ? 0 : (active / total) * 100),
      overduePercentage: () => (total === 0 ? 0 : (overdue / total) * 100),
    };
  }

  async markReminderCompleted(reminderId) {
    const r = this.getReminder(reminderId);
    if (!r) throw new Error(`Reminder ${reminderId} not found`);
    r.markCompleted();
  }

  async markReminderCancelled(reminderId) {
    const r = this.getReminder(reminderId);
    if (!r) throw new Error(`Reminder ${reminderId} not found`);
    r.markCancelled();
  }

  async activateReminder(reminderId) {
    const r = this.getReminder(reminderId);
    if (!r) throw new Error(`Reminder ${reminderId} not found`);
    r.activate();
  }
}

class ReminderUpdate {
  constructor() {
    this.content = null;
    this.remind_at = null;
    this.priority = null;
    this.status = null;
    this.tags = null;
  }
  static new() { return new ReminderUpdate(); }
  content(v) { this.content = v; return this; }
  remindAt(v) { this.remind_at = v; return this; }
  priority(v) { this.priority = v; return this; }
  status(v) { this.status = v; return this; }
  tags(v) { this.tags = v; return this; }
}

class ReminderStatistics {
  constructor(data) {
    this.total_reminders = data.total_reminders;
    this.pending_reminders = data.pending_reminders;
    this.active_reminders = data.active_reminders;
    this.overdue_reminders = data.overdue_reminders;
    this.unique_tags = data.unique_tags;
  }
  pendingPercentage() {
    return this.total_reminders === 0 ? 0 : (this.pending_reminders / this.total_reminders) * 100;
  }
  activePercentage() {
    return this.total_reminders === 0 ? 0 : (this.active_reminders / this.total_reminders) * 100;
  }
  overduePercentage() {
    return this.total_reminders === 0 ? 0 : (this.overdue_reminders / this.total_reminders) * 100;
  }
}

const ReminderPriority = {
  Low: 'low',
  Medium: 'medium',
  High: 'high',
  Critical: 'critical',
};

// Lightweight parseReminderFormat for demo (mirroring the original shape)
function parseReminderFormat(reminderText) {
  const startTag = "<reminder>";
  const endTag = "</reminder>";
  const startIndex = reminderText.indexOf(startTag);
  const endIndex = reminderText.indexOf(endTag);

  if (startIndex === -1) throw new Error("Missing <reminder> start tag");
  if (endIndex === -1) throw new Error("Missing </reminder> end tag");

  const content = reminderText.substring(startIndex + startTag.length, endIndex);
  const parts = content.split(';').map(s => s.trim());
  if (parts.length < 3) throw new Error("Invalid reminder format: need at least 3 parts (content; remind_at; priority)");

  const remindAt = new Date(parts[1]);
  if (isNaN(remindAt.getTime())) throw new Error("Invalid reminder date format");

  const priorityMap = { low: 'low', medium: 'medium', high: 'high', critical: 'critical' };
  const priority = priorityMap[(parts[2] || '').toLowerCase()];
  if (!priority) throw new Error("Invalid reminder priority");

  const statusMap = { pending: 'pending', active: 'active', completed: 'completed', cancelled: 'cancelled' };
  const status = parts.length > 3 && parts[3] ? (statusMap[(parts[3] || '').toLowerCase()] || 'pending') : 'pending';

  const tags = parts.length > 4 && parts[4] ? parts[4].split(',').map(s => s.trim()) : [];

  return new Reminder({
    id: uuidv4(),
    content: parts[0],
    remind_at: remindAt,
    priority,
    status,
    tags,
  });
}

// Demo runner
async function main() {
  const manager = new ReminderManagerExt();

  // 1) Create a few reminders
  const now = new Date();
  const in1min = new Date(now.getTime() + 60 * 1000);
  const in2min = new Date(now.getTime() + 2 * 60 * 1000);
  const ago1min = new Date(now.getTime() - 60 * 1000);

  const r1 = await manager.createReminder({
    content: "Water plants",
    remind_at: in1min,
    priority: 'medium',
    status: 'pending',
    tags: ["home", "daily"]
  });

  const r2 = await manager.createReminder({
    content: "Email weekly report",
    remind_at: in2min,
    priority: 'high',
    status: 'pending',
    tags: ["work", "email"]
  });

  const r3 = await manager.createReminder({
    content: "Code review PR #42",
    remind_at: ago1min,
    priority: 'high',
    status: 'active',
    tags: ["work", "code"]
  });

  const r4 = await manager.createReminder({
    content: "Pay credit card bill",
    remind_at: new Date(now.getTime() + 24 * 60 * 60 * 1000), // tomorrow
    priority: 'critical',
    status: 'pending',
    tags: ["finance", "monthly"]
  });

  // 2) Update one using builder
  await manager.updateReminder(r2, new ReminderUpdate()
    .priority('critical')
    .tags(["work", "email", "review"])
    .content("Email weekly report to leadership")
  );

  // 3) Search reminders
  const searchResults = manager.searchReminders("email");
  console.log(`Search "email": ${searchResults.length} result(s):`);
  searchResults.forEach(r => console.log(` - [${r.id.slice(0, 8)}] ${r.content} (${r.priority}, ${r.status})`));

  // 4) Filter by status/priority/tag
  const pending = manager.getRemindersByStatus('pending');
  console.log(`Pending reminders: ${pending.length}`);

  const high = manager.getRemindersByPriority('high');
  console.log(`High priority reminders: ${high.length}`);

  const workTag = manager.getRemindersByTag("work");
  console.log(`Reminders tagged "work": ${workTag.length}`);

  // 5) Overdue and due soon
  const overdue = manager.getOverdueReminders();
  console.log(`Overdue reminders: ${overdue.length}`);
  overdue.forEach(r => console.log(` - [${r.id.slice(0, 8)}] ${r.content} (was due at ${r.remind_at.toISOString()})`));

  const dueSoon = manager.getRemindersDueSoon(90 * 1000); // next 90 seconds
  console.log(`Due soon (90s): ${dueSoon.length}`);

  // 6) Mark some complete/cancelled
  await manager.markReminderCompleted(r1);
  await manager.markReminderCancelled(r4);

  // 7) Statistics
  const stats = manager.getReminderStatistics();
  const statsObj = {
    total: stats.total_reminders,
    pending: stats.pending_reminders,
    active: stats.active_reminders,
    overdue: stats.overdue_reminders,
    uniqueTags: stats.unique_tags,
    pendingPct: stats.pendingPercentage().toFixed(1),
    activePct: stats.activePercentage().toFixed(1),
    overduePct: stats.overduePercentage().toFixed(1),
  };
  console.log("Reminder statistics:", statsObj);

  // 8) Parse from formatted text
  const parsedReminder = parseReminderFormat(
    "<reminder>Prepare presentation; 2025-01-20T10:00:00Z; high; pending; work,slides</reminder>"
  );
  console.log("Parsed reminder content:", parsedReminder.content, "priority:", parsedReminder.priority);

  // 9) Add parsed reminder to manager
  await manager.createReminder(parsedReminder);

  // Final counts after all operations
  console.log(`Total reminders in system: ${manager.getAllReminders().length}`);
}

// Only run demo if this file is executed directly
// Run if executed directly
  main().catch(err => {
    console.error("Demo failed:", err.message);
    process.exit(1);
  });
}

  Reminder,
  ReminderStatus,
  ReminderPriority,
  ReminderManagerExt,
  ReminderUpdate,
  ReminderStatistics,
  parseReminderFormat,
};