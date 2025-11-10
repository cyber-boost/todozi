// example5-unified-search-and-semantic-recommendations.js
//
// A compact, runnable example showing how to:
// 1) Seed storage with tasks, memories, ideas, errors, and training data
// 2) Build a unified SearchEngine index (from search.js)
// 3) Run keyword, advanced, and semantic searches
// 4) Use optional embeddings (emb.js) to power semantic search & recommendations
// 5) Use a simple CLI interaction via TodoziHandler to add/search tasks
//
// How to run:
//   1) Ensure the provided files are present in your project:
//      - search.js  (SearchEngine, SearchOptions, SearchResults, SearchDataType)
//      - models.js  (Task, Memory, Idea, TrainingData, Error, etc.)
//      - storage.js (Storage class and helpers)
//      - emb.js     (TodoziEmbeddingService and EmbeddingModel placeholders)
//   2) Run this file with Node.js (v16+):
//      node example5-unified-search-and-semantic-recommendations.js
//
// Notes:
// - This script uses a minimal in-memory storage adapter so you can run it without
//   setting up a full filesystem. It still uses the real SearchEngine from search.js.
// - Embeddings are optional. If @xenova/transformers is not installed, the script
//   falls back gracefully to plain SearchEngine queries (keyword/advanced).
// - The TodoziHandler methods used here (addTaskToProject, listTasksAcrossProjects,
//   saveMemory, listMemories, saveIdea, listIdeas, saveError, listErrors,
//   saveTrainingData, listTrainingData, searchTasks) are simple mocks in this file.

import path from 'path';
import { fileURLToPath } from 'url';

// -------------------------
// Minimal in-memory "storage"
// -------------------------
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import the models and helpers we need
import {
  Task,
  Priority,
  Status,
  Memory,
  Idea,
  Error,
  TrainingData,
  TrainingDataType,
} from './models.js';

// NOTE: The actual storage.js uses filesystem and is Node/CJS.
// For this example, we'll provide a tiny in-memory store so it runs anywhere.

class MemoryStorage {
  constructor() {
    this.tasks = [];
    this.memories = [];
    this.ideas = [];
    this.errors = [];
    this.trainingData = [];
  }

  // Tasks
  async addTaskToProject(task) {
    this.tasks.push(task);
    return task.id;
  }
  async getTaskFromAnyProject(id) {
    const t = this.tasks.find(x => x.id === id);
    if (!t) throw new Error(`Task not found: ${id}`);
    return t;
  }
  async listTasksAcrossProjects(filters = {}) {
    let results = [...this.tasks];
    if (filters.status) results = results.filter(t => t.status === filters.status);
    if (filters.priority) results = results.filter(t => t.priority === filters.priority);
    if (filters.project) results = results.filter(t => t.parentProject === filters.project);
    if (filters.search) {
      const s = filters.search.toLowerCase();
      results = results.filter(t => t.action.toLowerCase().includes(s));
    }
    return results;
  }

  // Memoires
  async saveMemory(memory) {
    this.memories.push(memory);
    return memory.id;
  }
  async listMemories() {
    return [...this.memories];
  }

  // Ideas
  async saveIdea(idea) {
    this.ideas.push(idea);
    return idea.id;
  }
  async listIdeas() {
    return [...this.ideas];
  }

  // Errors
  async saveError(error) {
    this.errors.push(error);
    return error.id;
  }
  async listErrors() {
    return [...this.errors];
  }

  // Training
  async saveTrainingData(td) {
    this.trainingData.push(td);
    return td.id;
  }
  async listTrainingData() {
    return [...this.trainingData];
  }

  // Task search (simple contains)
  async searchTasks(query) {
    const q = query.toLowerCase();
    return this.tasks.filter(t =>
      t.action.toLowerCase().includes(q) ||
      (t.contextNotes && t.contextNotes.toLowerCase().includes(q))
    );
  }
}

const memoryStorage = new MemoryStorage();

// -------------------------
// Minimal CLI handler with a few methods used by the example
// -------------------------
import { SearchEngine, SearchOptions, SearchDataType, SearchResults } from './search.js';
import { Assignee } from './models.js';

class SimpleCliHandler {
  constructor(storage) {
    this.storage = storage;
  }

  async handleAddTaskCommand(command) {
    const priorityEnum = Priority.Medium;
    const statusEnum = Status.TODO;
    const assigneeEnum = command.assignee ? Assignee.Human : null;
    const tagsVec = command.tags ? command.tags.split(',').map(s => s.trim()) : [];
    const dependenciesVec = command.dependencies ? command.dependencies.split(',').map(s => s.trim()) : [];

    const task = Task.newFull(
      'cli_user',
      command.action,
      command.time || '1 hour',
      priorityEnum,
      command.project || 'general',
      statusEnum,
      assigneeEnum,
      tagsVec,
      dependenciesVec,
      command.context || null,
      command.progress || null,
    );

    await this.storage.addTaskToProject(task);
    console.log(`✅ Task created: ${task.id} - ${task.action}`);
    return task;
  }

  async handleSearchTasksCommand(command) {
    const tasks = await this.storage.searchTasks(command.query);
    console.log(`Found ${tasks.length} task(s) matching "${command.query}":`);
    for (const t of tasks) {
      console.log(`  - ${t.id} [${t.parentProject}] (${t.priority}/${t.status}) ${t.action}`);
    }
    return tasks;
  }

  async handleStatsCommand() {
    const allTasks = await this.storage.listTasksAcrossProjects({});
    const activeTasks = await this.storage.listTasksAcrossProjects({ status: Status.Todo });
    const completedTasks = await this.storage.listTasksAcrossProjects({ status: Status.Done });

    console.log('Todozi Statistics:');
    console.log(`  Total tasks: ${allTasks.length}`);
    console.log(`  Active tasks: ${activeTasks.length}`);
    console.log(`  Completed tasks: ${completedTasks.length}`);
  }
}

const cliHandler = new SimpleCliHandler(memoryStorage);

// -------------------------
// Embeddings support (optional)
// -------------------------
let EmbeddingService = null;
let EmbeddingConfig = null;
let EmbeddingModel = null;
let embeddingService = null;

async function tryInitEmbeddings() {
  try {
    // dynamic import so it doesn't fail if not installed
    const emb = await import('./emb.js');
    EmbeddingService = emb.TodoziEmbeddingService;
    EmbeddingConfig = emb.TodoziEmbeddingConfig;
    EmbeddingModel = emb.EmbeddingModel;
    const config = new EmbeddingConfig();
    // Using a placeholder model. In real use, load a real transformer model.
    const model = await EmbeddingModel.load(config.model_name, 'cpu');
    embeddingService = await EmbeddingService.withSharedComponents(config, new Map(), model, new Map(), null, null);
    console.log('✅ Embeddings initialized (placeholder model).');
    return true;
  } catch (e) {
    console.log('⚠️  Embeddings not available. Semantic search will be keyword-based only.');
    return false;
  }
}

// -------------------------
// Unified search across all content
// -------------------------
function buildSearchIndexFromStorage(storage) {
  const engine = new SearchEngine();
  // Push content arrays into the engine
  engine.updateIndex({
    tasks: storage.tasks,
    memories: storage.memories,
    ideas: storage.ideas,
    errors: storage.errors,
    training_data: storage.trainingData,
  });
  return engine;
}

// -------------------------
// Example 5: Unified search and semantic recommendations
// -------------------------
async function main() {
  console.log('=== Example 5: Unified Search + Semantic Recommendations ===\n');

  // 1) Seed storage with diverse content
  console.log('1) Seeding storage with sample content...');
  const t1 = Task.newFull(
    'u1',
    'Implement user authentication with JWT',
    '3 hours',
    Priority.High,
    'webapp',
    Status.Todo,
    Assignee.Agent('coder'),
    ['auth', 'security', 'backend'],
    [],
    'Use stateless JWT and refresh tokens',
    0
  );
  const t2 = Task.newFull(
    'u1',
    'Design database schema for analytics',
    '2 hours',
    Priority.Medium,
    'data',
    Status.InProgress,
    Assignee.Human,
    ['database', 'analytics', 'schema'],
    [],
    'Think about partitioning and indexes',
    30
  );
  const t3 = Task.newFull(
    'u1',
    'Fix memory leak in image processing worker',
    '4 hours',
    Priority.Critical,
    'ml',
    Status.Todo,
    Assignee.Ai,
    ['performance', 'bug', 'worker'],
    [],
    'Profile and identify GC roots',
    0
  );

  await memoryStorage.addTaskToProject(t1);
  await memoryStorage.addTaskToProject(t2);
  await memoryStorage.addTaskToProject(t3);

  const m1 = new Memory({
    userId: 'u1',
    moment: 'Discovered high latency on GET /reports',
    meaning: 'Database queries missing indexes',
    reason: 'Large table scans on aggregates',
    importance: MemoryImportance.High,
    term: MemoryTerm.Long,
    memoryType: { type: 'Standard' },
    tags: ['performance', 'database', 'reports'],
  });
  await memoryStorage.saveMemory(m1);

  const m2 = new Memory({
    userId: 'u1',
    moment: 'Customer reported error during checkout',
    meaning: 'Race condition in inventory service',
    reason: 'Locks not acquired properly',
    importance: MemoryImportance.Critical,
    term: MemoryTerm.Short,
    memoryType: { type: 'Emotional', emotion: 'frustrated' },
    tags: ['bug', 'checkout', 'inventory'],
  });
  await memoryStorage.saveMemory(m2);

  const i1 = new Idea({
    idea: 'Use change data capture for near real-time dashboards',
    importance: { type: 'High' },
    share: { type: 'Team' },
    tags: ['cdc', 'analytics'],
    context: 'Instead of periodic ETL, stream changes',
  });
  await memoryStorage.saveIdea(i1);

  const e1 = Error.new('checkout race', 'Inventory inconsistency under concurrent checkout', 'inventory-service');
  e1.severity = 'high';
  e1.category = 'runtime';
  e1.tags = ['race', 'checkout', 'inventory'];
  await memoryStorage.saveError(e1);

  const tr1 = TrainingData.new(
    TrainingDataType.Instruction,
    'Summarize the key points from the following report:',
    'The report highlights 3 main trends: ...',
    'manual-curation'
  );
  await memoryStorage.saveTrainingData(tr1);

  console.log(`   - Tasks: ${memoryStorage.tasks.length}`);
  console.log(`   - Memories: ${memoryStorage.memories.length}`);
  console.log(`   - Ideas: ${memoryStorage.ideas.length}`);
  console.log(`   - Errors: ${memoryStorage.errors.length}`);
  console.log(`   - Training: ${memoryStorage.trainingData.length}\n`);

  // 2) Build unified search index
  console.log('2) Building unified search index...');
  const searchEngine = buildSearchIndexFromStorage(memoryStorage);
  const analytics = searchEngine.getSearchAnalytics();
  console.log(`   Indexed items: ${analytics.totalIndexedItems}`);
  console.log(`   Breakdown -> tasks: ${analytics.tasksCount}, memories: ${analytics.memoriesCount}, ideas: ${analytics.ideasCount}, errors: ${analytics.errorsCount}, training: ${analytics.trainingCount}\n`);

  // 3) Keyword search across everything
  console.log('3) Keyword search: "performance"');
  let results = searchEngine.search('performance', { limit: 10 });
  console.log(`   Total results: ${results.totalResults()}`);
  for (const r of results.taskResults.slice(0, 3)) {
    console.log(`   - Task: ${r.task.action} (score=${r.score.toFixed(2)})`);
  }
  for (const r of results.memoryResults.slice(0, 2)) {
    console.log(`   - Memory: ${r.memory.moment} => ${r.memory.meaning} (score=${r.score.toFixed(2)})`);
  }
  for (const r of results.ideaResults.slice(0, 2)) {
    console.log(`   - Idea: ${r.idea.idea} (score=${r.score.toFixed(2)})`);
  }
  for (const r of results.errorResults.slice(0, 2)) {
    console.log(`   - Error: ${r.error.title} (severity=${r.error.severity}, score=${r.score.toFixed(2)})`);
  }
  for (const r of results.trainingResults.slice(0, 2)) {
    console.log(`   - Training: ${r.trainingData.prompt} (type=${r.trainingData.dataType}, score=${r.score.toFixed(2)})`);
  }
  console.log();

  // 4) Advanced search using criteria
  console.log('4) Advanced search: tasks with priority=High and tag includes "backend"');
  const advanced = searchEngine.advancedSearch({
    taskCriteria: {
      priority: 'high',
      requiredTag: 'backend',
    },
    memoryCriteria: {},
  });
  console.log(`   Matched tasks: ${advanced.taskResults.length}`);
  for (const r of advanced.taskResults) {
    console.log(`   - ${r.task.action} (${r.task.priority}) tags=[${r.task.tags.join(', ')}]`);
  }
  console.log();

  // 5) Optional: Semantic search + recommendations via embeddings
  const embeddingsEnabled = await tryInitEmbeddings();

  if (embeddingsEnabled && embeddingService) {
    console.log('5) Semantic search (embeddings): "user authentication"');
    const semanticResults = await embeddingService.semanticSearch('user authentication', null, 10);
    console.log(`   Semantic hits: ${semanticResults.length}`);
    for (const s of semanticResults.slice(0, 5)) {
      console.log(`   - [${s.content_type}] (score=${s.similarity_score.toFixed(2)}) ${s.text_content.split('\n')[0]}`);
    }

    console.log('\n5b) Recommendations based on a task:');
    const basedOnTaskId = memoryStorage.tasks[0].id;
    const recommendations = await embeddingService.recommendSimilar([basedOnTaskId], [], 5);
    console.log(`   Recommendations for ${basedOnTaskId}: ${recommendations.length}`);
    for (const r of recommendations) {
      console.log(`   - [${r.content_type}] (score=${r.similarity_score.toFixed(2)}) ${r.text_content.split('\n')[0]}`);
    }
  } else {
    console.log('5) Semantic search skipped (no embeddings).');
  }

  // 6) Using CLI: add a task, then search tasks
  console.log('\n6) CLI interactions:');
  const added = await cliHandler.handleAddTaskCommand({
    action: 'Optimize image pipeline throughput',
    time: '2 hours',
    priority: 'high',
    project: 'ml',
    tags: 'performance,images,throughput',
    context: 'Batch processing & concurrent queues',
  });

  await cliHandler.handleSearchTasksCommand({ query: 'image' });
  await cliHandler.handleStatsCommand();

  // 7) Filter search by data type
  console.log('\n7) Filtered search by data type:');
  const filtered = searchEngine.search('bug', {
    dataTypes: [SearchDataType.Errors, SearchDataType.Memories],
    limit: 10
  });
  console.log(`   Memory results: ${filtered.memoryResults.length}`);
  console.log(`   Error results: ${filtered.errorResults.length}`);
  for (const r of filtered.memoryResults) {
    console.log(`   - Memory: ${r.memory.moment}`);
  }
  for (const r of filtered.errorResults) {
    console.log(`   - Error: ${r.error.title} (${r.error.severity})`);
  }

  // 8) Suggestions (keywords from indexed content)
  console.log('\n8) Search suggestions for "perfo":');
  const suggestions = searchEngine.getSearchSuggestions('perfo', 5);
  console.log(`   Suggestions: ${suggestions.join(', ')}`);

  console.log('\n=== Example 5 Complete ===');
}

main().catch(err => {
  console.error('Error running Example 5:', err);
  process.exit(1);
});