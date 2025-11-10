/**
import { Task, TaskCollection, Project, ProjectTaskContainer, TaskFilters, ApiKey, ApiKeyCollection, QueueItem, QueueSession, QueueCollection, Priority, Status, Assignee, ItemStatus, ProjectStatus, } from '../todozi/models.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig, } from '../todozi/emb.js';
import { SearchEngine, } from '../todozi/types.js';

import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

 * Example 5: Practical, end-to-end usage of the Todozi core models + embeddings + storage patterns
 *
 * What this shows:
 * - Create projects and tasks using the models from models.js
 * - Store tasks in-memory with a simple storage adapter (ProjectTaskContainer)
 * - Use the embedding service from emb.js to generate embeddings and find similar tasks
 * - Use the search engine from types.js to run unified queries across tasks
 * - Use the API key models to register and verify API credentials
 * - Plan queue items and start/end work sessions
 *
 * How to run:
 *   1) Save this file as example5_usage.js
 *   2) Place it alongside models.js, emb.js, tags.js, types.js, and todozi.js (same folder)
 *   3) Run: node example5_usage.js
 */


// Try to load the CommonJS exports from the provided modules.
// If your setup differs, adjust these require paths accordingly.
/**
 * Minimal, in-memory storage adapter that demonstrates the storage shape
 * used throughout the rest of the system. Not persistent (demo only).
 */
class InMemoryStorage {
  constructor() {
    this.projects = new Map(); // name -> Project
    this.projectTaskContainers = new Map(); // name -> ProjectTaskContainer
    this.globalTasks = new TaskCollection(); // TaskCollection
    this.apiKeys = new ApiKeyCollection();
    this.queues = new QueueCollection();
  }

  // ---------- Projects ----------
  async createProject(name, description = '') {
    if (this.projects.has(name)) {
      throw new Error(`Project already exists: ${name}`);
    }
    const project = new Project(name, description);
    this.projects.set(name, project);
    this.projectTaskContainers.set(name, ProjectTaskContainer.new(name));
    return project;
  }

  async getProject(name) {
    const project = this.projects.get(name);
    if (!project) throw new Error(`Project not found: ${name}`);
    return project;
  }

  async listProjects() {
    return Array.from(this.projects.values());
  }

  async getProjectTasks(projectName) {
    const container = this.projectTaskContainers.get(projectName);
    if (!container) throw new Error(`Project not found: ${projectName}`);
    return container.getAllTasks();
  }

  async archiveProject(name) {
    const project = await this.getProject(name);
    project.archive();
    this.projects.set(name, project);
  }

  async deleteProject(name) {
    if (!this.projects.has(name)) throw new Error(`Project not found: ${name}`);
    this.projects.delete(name);
    this.projectTaskContainers.delete(name);
  }

  async updateProject(project) {
    if (!this.projects.has(project.name)) throw new Error(`Project not found: ${project.name}`);
    this.projects.set(project.name, project);
  }

  // ---------- Tasks ----------
  async addTaskToProject(task) {
    if (!task.parentProject) {
      task.parentProject = 'general';
    }
    let container = this.projectTaskContainers.get(task.parentProject);
    if (!container) {
      await this.createProject(task.parentProject, '');
      container = this.projectTaskContainers.get(task.parentProject);
    }
    container.addTask(task);
    this.globalTasks.addTask(task);
    return task.id;
  }

  async getTaskFromAnyProject(id) {
    for (const container of this.projectTaskContainers.values()) {
      const task = container.getTask(id);
      if (task) return task;
    }
    const globalTask = this.globalTasks.getTask(id);
    if (globalTask) return globalTask;
    throw new Error(`Task not found: ${id}`);
  }

  async getTaskFromProject(projectName, id) {
    const container = this.projectTaskContainers.get(projectName);
    if (!container) throw new Error(`Project not found: ${projectName}`);
    const task = container.getTask(id);
    if (!task) throw new Error(`Task not found in project ${projectName}: ${id}`);
    return task;
  }

  async updateTaskInProject(id, updates) {
    const task = await this.getTaskFromAnyProject(id);
    task.update(updates);
    return task;
  }

  async completeTaskInProject(id) {
    const task = await this.getTaskFromAnyProject(id);
    task.complete();
    return task;
  }

  async deleteTaskFromProject(id) {
    let found = false;
    for (const container of this.projectTaskContainers.values()) {
      const t = container.removeTask(id);
      if (t) found = true;
    }
    if (this.globalTasks.removeTask(id)) found = true;
    if (!found) throw new Error(`Task not found: ${id}`);
  }

  async listTasksAcrossProjects(filters = {}) {
    // Use ProjectTaskContainer.getFilteredTasks to honor search on action/contextNotes
    const taskFilters = new TaskFilters({
      project: filters.project || null,
      status: filters.status || null,
      priority: filters.priority || null,
      assignee: filters.assignee || null,
      tags: filters.tags || null,
      search: filters.search || null,
    });

    const results = [];
    for (const container of this.projectTaskContainers.values()) {
      const tasks = container.getFilteredTasks(taskFilters);
      results.push(...tasks);
    }
    return results;
  }

  async searchTasks(query) {
    // Simple client-side search: filter by action and contextNotes
    const all = this.globalTasks.getAllTasks();
    const q = query.toLowerCase();
    return all.filter(t => {
      const actionMatch = t.action.toLowerCase().includes(q);
      const contextMatch = t.contextNotes ? t.contextNotes.toLowerCase().includes(q) : false;
      return actionMatch || contextMatch;
    });
  }

  // ---------- API Keys ----------
  async createApiKey(userId) {
    const apiKey = ApiKey.withUserId(userId);
    this.apiKeys.addKey(apiKey);
    return apiKey;
  }

  async listApiKeys() {
    return this.apiKeys.getAllKeys();
  }

  async listActiveApiKeys() {
    return this.apiKeys.getActiveKeys();
  }

  async deactivateApiKey(userId) {
    this.apiKeys.deactivateKey(userId);
  }

  async activateApiKey(userId) {
    this.apiKeys.activateKey(userId);
  }

  async removeApiKey(userId) {
    const key = this.apiKeys.removeKey(userId);
    if (!key) throw new Error(`API key not found for userId: ${userId}`);
    return key;
  }

  async checkApiKeyAuth(publicKey, privateKey) {
    const key = this.apiKeys.getKeyByPublic(publicKey);
    if (!key) throw new Error('Invalid public key');
    const isAdmin = key.isAdmin(publicKey, privateKey);
    return { userId: key.userId, isAdmin };
  }

  // ---------- Queues ----------
  async addQueueItem(item) {
    this.queues.addItem(item);
    return item.id;
  }

  async listQueueItems() {
    return this.queues.getAllItems();
  }

  async listQueueItemsByStatus(status) {
    return this.queues.getItemsByStatus(status);
  }

  async listBacklogItems() {
    return this.queues.getBacklogItems();
  }

  async listActiveItems() {
    return this.queues.getActiveItems();
  }

  async listCompleteItems() {
    return this.queues.getCompleteItems();
  }

  async startQueueSession(queueItemId) {
    return this.queues.startSession(queueItemId);
  }

  async endQueueSession(sessionId) {
    this.queues.endSession(sessionId);
  }

  async getQueueSession(sessionId) {
    return this.queues.getSession(sessionId);
  }
}

/**
 * Helper to pretty-print tasks
 */
function formatTask(task) {
  return [
    `Task ${task.id} | ${task.action}`,
    `  Project: ${task.parentProject}`,
    `  Priority: ${task.priority} | Status: ${task.status}`,
    `  Assignee: ${Assignee.toString(task.assignee)}`,
    `  Tags: ${task.tags.join(', ') || '-'}`,
    `  Progress: ${task.progress ?? 0}%`,
    `  Created: ${task.createdAt}`,
    `  Updated: ${task.updatedAt}`,
  ].join('\n');
}

/**
 * Main demonstration routine
 */
async function main() {
  console.log('Example 5: Todozi models + embeddings + storage + queues\n');

  // 1) Storage adapter (in-memory)
  const storage = new InMemoryStorage();

  // 2) Embeddings
  const embeddingConfig = new TodoziEmbeddingConfig();
  const embeddingService = await TodoziEmbeddingService.new(embeddingConfig);
  await embeddingService.initialize();

  // 3) Search
  const searchEngine = new SearchEngine();

  // 4) Create a project
  await storage.createProject('general', 'Default project');

  // 5) Add tasks (models.js)
  const t1 = Task.newFull(
    'demo-user',
    'Implement user authentication',
    '3h',
    Priority.High,
    'general',
    Status.Todo,
    Assignee.Ai,
    ['auth', 'security'],
    [],
    'Build email + password + JWT auth',
    0
  );
  await storage.addTaskToProject(t1);
  // Embed the task content
  const content1 = embeddingService.prepareTaskContent(t1);
  const emb1 = await embeddingService.generateEmbedding(content1);
  t1.embeddingVector = emb1;

  const t2 = Task.newFull(
    'demo-user',
    'Set up CI pipeline with GitHub Actions',
    '2h',
    Priority.Medium,
    'general',
    Status.InProgress,
    Assignee.Human,
    ['ci', 'automation'],
    [],
    'Lint, test, build on every push',
    10
  );
  await storage.addTaskToProject(t2);
  const content2 = embeddingService.prepareTaskContent(t2);
  const emb2 = await embeddingService.generateEmbedding(content2);
  t2.embeddingVector = emb2;

  const t3 = Task.newFull(
    'demo-user',
    'Draft API documentation for /auth endpoints',
    '1.5h',
    Priority.Medium,
    'general',
    Status.Review,
    Assignee.Collaborative,
    ['docs', 'api'],
    [t1.id],
    'Include request/response examples and errors',
    60
  );
  await storage.addTaskToProject(t3);
  const content3 = embeddingService.prepareTaskContent(t3);
  const emb3 = await embeddingService.generateEmbedding(content3);
  t3.embeddingVector = emb3;

  console.log('Created tasks:');
  console.log(formatTask(t1));
  console.log(formatTask(t2));
  console.log(formatTask(t3));
  console.log('');

  // 6) Search by similarity using embedding service
  const similarToCI = await embeddingService.findSimilarTasks('CI pipeline configuration and setup', 5);
  console.log('Similar tasks to "CI pipeline configuration and setup":');
  if (similarToCI.length === 0) {
    console.log('  (no similar tasks found)');
  } else {
    for (const r of similarToCI) {
      console.log(`  - ${r.content_id} score=${r.similarity_score.toFixed(3)} snippet="${r.text_content.split('\n')[0]}"`);
    }
  }
  console.log('');

  // 7) Search across projects
  console.log('All tasks in project "general":');
  const generalTasks = await storage.getProjectTasks('general');
  for (const t of generalTasks) {
    console.log(`  - ${t.id}: ${t.action} (${t.status})`);
  }
  console.log('');

  // 8) Update a task
  await storage.updateTaskInProject(t1.id, {
    status: Status.InProgress,
    progress: 35,
    contextNotes: 'Implementing JWT issuance and middleware guard',
  });
  const updated = await storage.getTaskFromAnyProject(t1.id);
  console.log('Updated task:');
  console.log(formatTask(updated));
  console.log('');

  // 9) Complete a task
  await storage.completeTaskInProject(t3.id);
  const completed = await storage.getTaskFromAnyProject(t3.id);
  console.log('Completed task:');
  console.log(formatTask(completed));
  console.log('');

  // 10) API key management (ApiKey, ApiKeyCollection)
  const apiKey = await storage.createApiKey('user_12345');
  console.log('API key created:');
  console.log(`  userId: ${apiKey.userId}`);
  console.log(`  publicKey: ${apiKey.publicKey}`);
  console.log(`  privateKey: ${apiKey.privateKey}`);
  console.log(`  active: ${apiKey.active}`);
  console.log('');

  const check = await storage.checkApiKeyAuth(apiKey.publicKey, apiKey.privateKey);
  console.log('API key auth check:');
  console.log(`  userId: ${check.userId}`);
  console.log(`  isAdmin: ${check.isAdmin}`);
  console.log('');

  // 11) Queue planning and sessions
  const qi = new QueueItem({
    taskName: 'Review authentication flow',
    taskDescription: 'Walk through the new auth flow and verify edge cases',
    priority: Priority.High,
    projectId: 'general',
  });
  await storage.addQueueItem(qi);
  console.log('Planned queue item:');
  console.log(`  id: ${qi.id} | ${qi.taskName} (${qi.priority})`);
  console.log(`  status: ${qi.status}`);
  console.log('');

  const sessionId = await storage.startQueueSession(qi.id);
  console.log('Started session:');
  console.log(`  sessionId: ${sessionId}`);
  console.log(`  queueItemId: ${qi.id}`);
  console.log('');

  // Simulate some time passing (1s)
  await new Promise(r => setTimeout(r, 1000));
  await storage.endQueueSession(sessionId);
  const session = await storage.getQueueSession(sessionId);
  console.log('Ended session:');
  console.log(`  sessionId: ${session.id}`);
  console.log(`  durationSeconds: ${session.durationSeconds}`);
  console.log('');

  // 12) Unified search across tasks via SearchEngine (types.js)
  // Build a simple index content object
  const unifiedContent = {
    tasks: await storage.listTasksAcrossProjects({}),
    memories: [],      // not used in this demo
    ideas: [],         // not used in this demo
    agentAssignments: [],
    codeChunks: [],
    errors: [],
    trainingData: [],
    feelings: [],
  };
  searchEngine.updateIndex(unifiedContent);

  const results = searchEngine.search('authentication documentation', { limit: 10 });
  console.log('Unified search results for "authentication documentation":');
  console.log(`  tasks: ${results.taskResults.length}`);
  for (const t of results.taskResults) {
    console.log(`    - ${t.action} (${t.status})`);
  }
  console.log('');

  // 13) Export diagnostics from embedding service
  const diagnostics = await embeddingService.exportDiagnostics();
  console.log('Embedding diagnostics:');
  console.log(`  timestamp: ${diagnostics.timestamp}`);
  console.log(`  total embeddings: ${diagnostics.cache_hit_rate !== undefined ? 'N/A' : 'N/A'}`);
  console.log(`  (Content breakdown)`);
  for (const [k, v] of Object.entries(diagnostics.content_type_breakdown || {})) {
    console.log(`    - ${k}: ${v}`);
  }
  console.log('');

  // 14) Example of using ProjectTaskContainer filtering
  const container = await storage.getProjectTasks('general');
  const containerObj = (await (async () => {
    // For demo: rewrap into a ProjectTaskContainer to show filtering
    const cont = ProjectTaskContainer.new('general');
    for (const t of container) cont.addTask(t);
    return cont;
  })());

  const filtered = containerObj.getFilteredTasks(
    new TaskFilters({
      status: Status.InProgress,
      priority: Priority.High,
      search: 'auth',
    })
  );
  console.log('Filtered tasks (status=InProgress, priority=High, search="auth"):');
  for (const t of filtered) {
    console.log(`  - ${t.id}: ${t.action} (${t.status})`);
  }
  console.log('');

  console.log('✅ Example 5 completed successfully.');
}

// Run the demo
main().catch(err => {
  console.error('❌ Example 5 failed:', err);
  process.exit(1);
});