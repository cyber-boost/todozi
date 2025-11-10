

/*

 * Example 5: Full workflow — queues, agents, embeddings, and chat parsing
 *
 * What this demonstrates:
 * - Initialize storage
 * - Create an event-planner agent and other agents
 * - Use AgentManager to manage agents, assignments, and statistics
 * - Plan a multi-step project using the queue and steps manager
 * - Use TodoziEmbeddingService for basic semantic similarity over tasks
 * - Parse structured content from a free-form chat message (tasks, memories, ideas, errors, training data)
 * - Build a minimal in-memory "storage" adapter to make the demo run without external files
 *
 * How to run:
 *   1) Save this file as example5.js
 *   2) Run: node example5.js
 *
 * Notes:
 * - This is an end-to-end, self-contained example. It builds a minimal storage adapter and stubs some behavior for demo purposes.
 * - It imports from the provided modules (agent.js, models.js, emb.js, tags.js, todozi.js, storage.js).
 * - For the "embedding similarity" demo, we use a toy embedding (simple hash-based vector) so it runs without heavy dependencies.
 */
import { EventEmitter } from 'events';

function nowISO() {
  return new Date().toISOString();
}

function sleep(ms) {
  return new Promise(res => setTimeout(res, ms));
}

function uuidShort() {
  return Math.random().toString(36).slice(2, 10);
}

// ------------------------------
// Import provided modules
// ------------------------------
import {
  AgentManager,
  AgentUpdate,
  parseAgentAssignmentFormat,
} from '../todozi/agent.js';

import {
  Task,
  TaskUpdate,
  TaskFilters,
  QueueItem,
  QueueSession,
  QueueCollection,
  Tag,
  Memory,
  Idea,
  Error: TodoziErrorRecord,
  TrainingData,
  ApiKeyCollection,
  ProjectTaskContainer,
  Config,
  RegistrationInfo,
} from '../todozi/models.js';

import {
  TodoziEmbeddingService,
  TodoziEmbeddingConfig,
  LRUEmbeddingCache,
  EmbeddingModel,
} from '../todozi/emb.js';

import {
  TagManager,
} from '../todozi/tags.js';

import {
  processChatMessageExtended,
  transformShorthandTags,
} from '../todozi/todozi.js';

// ------------------------------
// Minimal "Storage Adapter" to make the demo run without file I/O
// ------------------------------
class MemoryStorageAdapter {
  constructor() {
    this.projects = new Map(); // name -> { name, description, tasks: Map<taskId, Task> }
    this.tasksIndex = new Map(); // taskId -> Task
    this.agents = new Map(); // agentId -> agentObj
    this.queues = new Map(); // projectName -> QueueCollection
    this.ideas = new Map(); // ideaId -> Idea
    this.memories = new Map(); // memoryId -> Memory
    this.errors = new Map(); // errorId -> Error
    this.training = new Map(); // trainingId -> TrainingData
    this.apiKeys = new ApiKeyCollection(); // unused here
    this.defaultProjectName = 'general';
    // seed with default project and queue
    this.createProject(this.defaultProjectName, 'General tasks');
    this.queues.set(this.defaultProjectName, new QueueCollection());
  }

  // Project ops
  createProject(name, description) {
    const p = { name, description, tasks: new Map() };
    this.projects.set(name, p);
    if (!this.queues.has(name)) {
      this.queues.set(name, new QueueCollection());
    }
    return p;
  }

  getProject(name) {
    return this.projects.get(name);
  }

  listProjects() {
    return Array.from(this.projects.values());
  }

  getOrCreateProject(name) {
    if (!this.projects.has(name)) {
      this.createProject(name, '');
    }
    return this.getProject(name);
  }

  // Task ops
  async addTaskToProject(task) {
    const project = this.getOrCreateProject(task.parentProject || this.defaultProjectName);
    project.tasks.set(task.id, task);
    this.tasksIndex.set(task.id, task);
    // seed a toy embedding vector for similarity demo
    if (!task.embeddingVector) {
      task.embeddingVector = seedEmbedding(task.action || '');
    }
  }

  async updateTaskInProject(taskId, updates) {
    const task = this.tasksIndex.get(taskId);
    if (!task) return;
    if (updates.action !== undefined) task.action = updates.action;
    if (updates.status !== undefined) task.status = updates.status;
    if (updates.priority !== undefined) task.priority = updates.priority;
    if (updates.parentProject !== undefined) task.parentProject = updates.parentProject;
    if (updates.tags !== undefined) task.tags = updates.tags;
    if (updates.dependencies !== undefined) task.dependencies = updates.dependencies;
    if (updates.contextNotes !== undefined) task.contextNotes = updates.contextNotes;
    if (updates.progress !== undefined) task.progress = updates.progress;
    task.updatedAt = nowISO();
  }

  async getTaskFromProject(projectName, taskId) {
    const project = this.projects.get(projectName);
    if (!project) return null;
    return project.tasks.get(taskId) || null;
  }

  async getTaskFromAnyProject(taskId) {
    return this.tasksIndex.get(taskId) || null;
  }

  async listTasksAcrossProjects(filters = {}) {
    const all = Array.from(this.tasksIndex.values());
    return all.filter(t => {
      if (filters.status && t.status !== filters.status) return false;
      if (filters.project && t.parentProject !== filters.project) return false;
      if (filters.priority && t.priority !== filters.priority) return false;
      if (filters.search && !(t.action || '').toLowerCase().includes(filters.search.toLowerCase())) return false;
      return true;
    });
  }

  async searchTasks(query) {
    const q = (query || '').toLowerCase();
    return Array.from(this.tasksIndex.values()).filter(t =>
      (t.action || '').toLowerCase().includes(q) ||
      (t.contextNotes || '').toLowerCase().includes(q)
    );
  }

  // Agent ops (we will store Agents in memory and use AgentManager)
  async saveAgent(agent) {
    this.agents.set(agent.id, agent);
  }

  async listAgents() {
    return Array.from(this.agents.values());
  }

  async loadAgent(agentId) {
    return this.agents.get(agentId) || null;
  }

  async saveAgentAssignment(assignment) {
    // In this demo we just log it
    // eslint-disable-next-line no-console
    console.log('[ASSIGNMENT]', JSON.stringify(assignment, null, 2));
  }

  // Queue ops
  async addQueueItem(item) {
    const project = this.getOrCreateProject(item.projectId || this.defaultProjectName);
    const q = this.queues.get(project.name);
    q.addItem(item);
  }

  async listQueueItems() {
    const all = [];
    for (const q of this.queues.values()) {
      all.push(...q.getAllItems());
    }
    return all;
  }

  async startQueueSession(queueItemId) {
    for (const q of this.queues.values()) {
      try {
        return q.startSession(queueItemId);
      } catch (_) {
        // not in this queue
      }
    }
    throw new Error('Queue item not found for session start');
  }

  async endQueueSession(sessionId) {
    for (const q of this.queues.values()) {
      if (q.sessions[sessionId]) {
        q.endSession(sessionId);
        return;
      }
    }
    throw new Error('Session not found');
  }

  async getQueueSession(sessionId) {
    for (const q of this.queues.values()) {
      if (q.sessions[sessionId]) {
        return q.sessions[sessionId];
      }
    }
    throw new Error('Session not found');
  }

  // Idea / Memory / Error / Training stubs
  async saveIdea(idea) {
    this.ideas.set(idea.id, idea);
  }
  async saveMemory(memory) {
    this.memories.set(memory.id, memory);
  }
  async saveError(error) {
    this.errors.set(error.id, error);
  }
  async saveTrainingData(td) {
    this.training.set(td.id, td);
  }
}

// ------------------------------
// Minimal "embedding" service (toy) that satisfies the required interface
// ------------------------------
class ToyEmbeddingModel {
  constructor(dimensions = 64) {
    this.dimensions = dimensions;
  }
  static async load(_modelName, _device) {
    return new ToyEmbeddingModel(64);
  }
  static async getDefaultModel() {
    return 'todozi-toy-embedding';
  }
  async encode(texts) {
    return texts.map(seedEmbedding);
  }
}
function seedEmbedding(text) {
  // Simple deterministic hash -> vector of small floats in [-1, 1]
  const dim = 64;
  const vec = new Array(dim).fill(0);
  const str = text || '';
  let h1 = 2166136261 >>> 0;
  for (let i = 0; i < str.length; i++) {
    h1 ^= str.charCodeAt(i);
    h1 = Math.imul(h1, 16777619);
  }
  // Fill vector with pseudo-random based on hash
  for (let i = 0; i < dim; i++) {
    const x = (h1 >>> 0) / 0xffffffff;
    vec[i] = ((x * (i + 1)) % 2) - 1; // value in [-1, 1]
  }
  // normalize
  const norm = Math.sqrt(vec.reduce((s, v) => s + v * v, 0)) || 1;
  return vec.map(v => v / norm);
}
function cosineSim(a, b) {
  if (!a || !b || a.length !== b.length) return 0;
  let dot = 0, na = 0, nb = 0;
  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i];
    na += a[i] * a[i];
    nb += b[i] * b[i];
  }
  const denom = Math.sqrt(na) * Math.sqrt(nb) || 1;
  return dot / denom;
}

// ------------------------------
// Steps Manager (file-based for demonstration only)
// ------------------------------
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
class StepsManager {
  constructor(baseDir = path.join(process.cwd(), '.todozi_steps')) {
    this.baseDir = baseDir;
    fs.mkdirSync(this.baseDir, { recursive: true });
  }
  _fileFor(taskId) {
    return path.join(this.baseDir, `${taskId}.json`);
  }
  async show(taskId) {
    const file = this._fileFor(taskId);
    if (!fs.existsSync(file)) return null;
    const data = JSON.parse(fs.readFileSync(file, 'utf8'));
    return data;
  }
  async add(taskId, step) {
    const file = this._fileFor(taskId);
    let data = { taskId, steps: [], status: 'active', createdAt: nowISO(), updatedAt: nowISO() };
    if (fs.existsSync(file)) {
      data = JSON.parse(fs.readFileSync(file, 'utf8'));
    }
    data.steps.push(step);
    data.updatedAt = nowISO();
    fs.writeFileSync(file, JSON.stringify(data, null, 2));
  }
  async done(taskId) {
    const file = this._fileFor(taskId);
    if (!fs.existsSync(file)) return;
    const data = JSON.parse(fs.readFileSync(file, 'utf8'));
    data.status = 'done';
    data.updatedAt = nowISO();
    fs.writeFileSync(file, JSON.stringify(data, null, 2));
  }
}

// ------------------------------
// Main Demo
// ------------------------------
async function main() {
  console.log('=== Example 5: Full workflow (queues, agents, embeddings, chat parsing) ===\n');

  // 1) Setup storage and services
  const storage = new MemoryStorageAdapter();

  // Embedding service (toy)
  const embConfig = new TodoziEmbeddingConfig();
  const embCache = new LRUEmbeddingCache(50);
  const toyModel = await ToyEmbeddingModel.load('todozi-toy-embedding', 'cpu');
  const embService = new TodoziEmbeddingService(embConfig, embCache, toyModel, new Map(), new TagManager(), storage);
  await embService.initialize(); // attaches model

  // AgentManager + register default agents
  const agentManager = new AgentManager();
  await agentManager.loadAgents();

  // Steps manager
  const steps = new StepsManager();

  // 2) Create additional specialized agents and a task
  const eventPlanner = {
    id: 'event_planner',
    name: 'Event Planner',
    description: 'Plans and coordinates events, budgets, schedules, and logistics',
    capabilities: ['task_planning', 'budgeting', 'scheduling', 'logistics'],
    specializations: ['events'],
    metadata: { status: 'available' }
  };
  const marketingSpecialist = {
    id: 'marketing_specialist',
    name: 'Marketing Specialist',
    description: 'Drives promotion, partnerships, and public relations',
    capabilities: ['content_production', 'partnerships', 'public_relations'],
    specializations: ['marketing'],
    metadata: { status: 'available' }
  };
  const coder = {
    id: 'coder',
    name: 'Coder',
    description: 'Builds tools, websites, apps, and automation',
    capabilities: ['code_development', 'code_review', 'testing', 'automation'],
    specializations: ['software', 'devops'],
    metadata: { status: 'available' }
  };

  await storage.saveAgent(eventPlanner);
  await storage.saveAgent(marketingSpecialist);
  await storage.saveAgent(coder);

  await agentManager.createAgent(eventPlanner);
  await agentManager.createAgent(marketingSpecialist);
  await agentManager.createAgent(coder);

  console.log('Agents created:');
  for (const a of agentManager.getAllAgents()) {
    console.log(` - ${a.id} (${a.name}) [${a.metadata?.status}]`);
  }
  console.log();

  // 3) Plan a multi-step project: "Organize a developer conference"
  const conferenceProject = 'conferences';
  storage.createProject(conferenceProject, 'Developer conference planning');

  const taskPlanId = `task_${uuidShort()}`;
  const planTask = new Task({
    userId: 'demo_user',
    action: 'Organize a developer conference (end-to-end)',
    time: '2 weeks',
    priority: 'high',
    parentProject: conferenceProject,
    status: 'todo',
    assignee: 'human',
    tags: ['conference', 'planning', 'kickoff'],
    dependencies: [],
    contextNotes: 'High-level kickoff task for planning a developer conference',
    progress: 0,
  });
  planTask.id = taskPlanId;
  await storage.addTaskToProject(planTask);
  await steps.add(taskPlanId, 'Define goals, audience, theme, and success metrics');
  await steps.add(taskPlanId, 'Set budget and secure sponsors');
  await steps.add(taskPlanId, 'Choose date and venue; book rooms');
  await steps.add(taskPlanId, 'Create schedule and speaker lineup');
  await steps.add(taskPlanId, 'Launch website and ticket sales');
  await steps.add(taskPlanId, 'Publicize the event (social media, partners, PR)');
  await steps.add(taskPlanId, 'Coordinate logistics (catering, AV, volunteers)');
  await steps.add(taskPlanId, 'Run event and collect feedback');

  console.log('Created plan task and steps:');
  const planSteps = await steps.show(taskPlanId);
  console.log(`  Task: ${planTask.action} [${planTask.id}]`);
  console.log('  Steps:');
  planSteps.steps.forEach((s, i) => console.log(`   ${i + 1}. ${s}`));
  console.log();

  // 4) Queue the kickoff as a human item; start a session
  const queueItem = new QueueItem({
    taskName: 'Kickoff planning session for developer conference',
    taskDescription: 'Define goals, audience, theme, and success metrics',
    priority: 'high',
    projectId: conferenceProject,
  });
  await storage.addQueueItem(queueItem);
  console.log(`Queued item: ${queueItem.taskName} [${queueItem.id}]`);

  const sessionId = await storage.startQueueSession(queueItem.id);
  console.log(`Started queue session: ${sessionId}\n`);

  // 5) AgentManager usage: find best agent and assign a task
  const bestAgent = agentManager.findBestAgent('events', 'task_planning');
  if (bestAgent) {
    console.log(`Best available agent for specialization=events (prefer task_planning): ${bestAgent.id} (${bestAgent.name})`);
    const assignTaskId = `task_${uuidShort()}`;
    const assignTask = new Task({
      userId: 'demo_user',
      action: 'Draft initial conference schedule',
      time: '1 day',
      priority: 'high',
      parentProject: conferenceProject,
      status: 'todo',
      assignee: 'agent:planner',
      tags: ['schedule', 'planning'],
      dependencies: [planTask.id],
      contextNotes: 'Create a draft schedule with track layout and session slots',
      progress: 0,
    });
    assignTask.id = assignTaskId;
    await storage.addTaskToProject(assignTask);
    const assignedTaskId = await agentManager.assignTaskToAgent(assignTask.id, bestAgent.id, conferenceProject);
    console.log(`Assigned task ${assignedTaskId} to agent ${bestAgent.id}\n`);
  } else {
    console.log('No available agent matches the specialization preference.\n');
  }

  // 6) Embedding similarity demo: find similar tasks to a query
  const similarTo = 'schedule planning and logistics';
  const allTasks = await storage.listTasksAcrossProjects({});
  // Compute similarities with our toy embedding
  const qVec = await embService.generateEmbedding(similarTo);
  const scored = allTasks
    .map(t => ({ task: t, score: cosineSim(t.embeddingVector, qVec) }))
    .sort((a, b) => b.score - a.score);

  console.log(`Similar tasks to "${similarTo}":`);
  for (let i = 0; i < Math.min(3, scored.length); i++) {
    const { task, score } = scored[i];
    console.log(`  - [${task.id}] ${task.action} (sim=${score.toFixed(3)})`);
  }
  console.log();

  // 7) AgentManager stats
  const stats = agentManager.getAgentStatistics();
  console.log('Agent Statistics:');
  console.log(`  Total agents: ${stats.total_agents}`);
  console.log(`  Available: ${stats.available_agents}, Busy: ${stats.busy_agents}, Inactive: ${stats.inactive_agents}`);
  console.log(`  Total assignments: ${stats.total_assignments}, Completed: ${stats.completed_assignments}`);
  console.log(`  Completion rate: ${stats.completionRate().toFixed(1)}%\n`);

  // 8) Chat parsing demo: structured content extraction from free text
  const chatText = `
    Hi! Please:
    - Create a todozi task: action: Draft sponsorship prospectus; time: 3 days; priority: high; project: conferences; status: todo; assignee: marketing_specialist; tags: sponsor, pitch
    - Create a memory: type: standard; moment: I realized we need a strong value proposition; meaning: easier to sell sponsorships; reason: improves outreach; importance: high; term: short; tags: sponsor, value_prop
    - Create an idea: idea: Offer event swag bundles to sponsors; share: team; importance: medium
    - Create an error: title: Ticket page timeout; description: Stripe checkout timed out for VIP tickets; severity: high; category: integration; source: web
    - Create training data: data_type: instruction; prompt: How to write a concise sponsorship email; completion: Subject: Partnership with DevConf — outline benefits and ask for a quick call; source: outreach
    - Agent assignment: <todozi_agent>marketing_specialist;${assignTask.id};${conferenceProject}</todozi_agent>
  `;

  const parsed = processChatMessageExtended(chatText, 'demo_user');
  console.log('Parsed structured content from chat:');
  console.log(`  Tasks: ${parsed.tasks.length}`);
  console.log(`  Memories: ${parsed.memories.length}`);
  console.log(`  Ideas: ${parsed.ideas.length}`);
  console.log(`  Errors: ${parsed.errors.length}`);
  console.log(`  Training: ${parsed.training_data.length}`);
  console.log(`  Agent assignments: ${parsed.agent_assignments.length}\n`);

  // Persist tasks, memories, ideas, errors, training
  for (const t of parsed.tasks) {
    t.embeddingVector = seedEmbedding(t.action);
    await storage.addTaskToProject(t);
  }
  for (const m of parsed.memories) {
    await storage.saveMemory(m);
  }
  for (const i of parsed.ideas) {
    await storage.saveIdea(i);
  }
  for (const e of parsed.errors) {
    await storage.saveError(e);
  }
  for (const tr of parsed.training_data) {
    await storage.saveTrainingData(tr);
  }
  for (const a of parsed.agent_assignments) {
    await storage.saveAgentAssignment(a);
  }

  // 9) Finish up: complete queue session
  await storage.endQueueSession(sessionId);
  const session = await storage.getQueueSession(sessionId);
  console.log(`Ended queue session: ${sessionId}. Duration seconds: ${session.durationSeconds || 0}\n`);

  // Mark plan steps as done (optional)
  await steps.done(taskPlanId);
  console.log('Plan marked as done. Demo complete.\n');
}

main().catch(err => {
  // eslint-disable-next-line no-console
  console.error('Fatal error:', err);
  process.exit(1);
});