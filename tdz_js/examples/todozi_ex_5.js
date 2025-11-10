/**
import { processChatMessageExtended, processWorkflow, TodoziError, Assignee, } from '../todozi/todozi.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig, LRUEmbeddingCache, EmbeddingModel, } from '../todozi/emb.js';

import path from 'path';
import originalExecuteAiTask from '../todozi/todozi.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

 * Example 5: Semantic Task Orchestrator (runnable demo)
 *
 * This example shows how to:
 * - Parse a complex chat message with mixed tags (<todozi>, <memory>, <idea>, <error>, <train>, <feel>)
 * - Route tasks to AI, Human, Collaborative, or a named Agent based on assignee rules
 * - Simulate storage operations (add queue items, save agent assignments, and update tasks)
 * - Generate embeddings for tasks, and run a simple semantic search
 *
 * Requirements:
 * - Node.js 16+
 * - This file is placed in the same folder as the provided todozi.js and emb.js
 *
 * Run:
 *   node example5_semantic_orchestrator.js
 */

/**
 * DemoStorage simulates persistence and queueing needed by executeTask().
 * It logs operations and keeps small in-memory collections so you can see
 * what would happen in a real system.
 */
class DemoStorage {
  constructor() {
    this.tasks = new Map();
    this.ideas = new Map();
    this.memories = new Map();
    this.errors = new Map();
    this.training = new Map();
    this.assignments = [];
    this.queueItems = [];
    this.projects = new Set(['general', 'ai-research', 'data-pipeline']);
  }

  async addTaskToProject(task) {
    // Ensure project exists
    this.projects.add(task.parent_project || 'general');
    this.tasks.set(task.id, structuredClone(task));
    console.log(`[storage] Task saved to project "${task.parent_project}" -> id=${task.id}`);
  }

  async updateTaskInProject(taskId, updates) {
    const t = this.tasks.get(taskId);
    if (!t) {
      console.log(`[storage] No task found to update: ${taskId}`);
      return;
    }
    Object.assign(t, updates);
    t.updated_at = new Date().toISOString();
    this.tasks.set(taskId, t);
    console.log(`[storage] Task updated id=${taskId}`, updates);
  }

  async getTaskFromProject(_projectName, taskId) {
    return this.tasks.get(taskId) || null;
  }

  async getTaskFromAnyProject(taskId) {
    return this.tasks.get(taskId) || null;
  }

  async listTasksAcrossProjects(_filters = {}) {
    return Array.from(this.tasks.values());
  }

  async addQueueItem(item) {
    this.queueItems.push(item);
    console.log(`[queue] Enqueued item -> id=${item.id} title="${item.title}"`);
  }

  async saveAgentAssignment(assignment) {
    this.assignments.push(assignment);
    console.log(`[assignment] Saved -> agent=${assignment.agent_id} task=${assignment.task_id} project=${assignment.project_id}`);
  }

  // Semantic search simulation
  async searchTasksSemantic(_query, _limit = 5) {
    // In a real system, this would query a vector database or an embedding index.
    // Here we return a few synthetic results for demonstration.
    const fake = [];
    const all = Array.from(this.tasks.values());
    for (let i = 0; i < Math.min(3, all.length); i++) {
      const task = all[i];
      fake.push({
        task,
        similarityScore: 0.5 + Math.random() * 0.4,
        matchedContent: task.action,
      });
    }
    return fake;
  }
}

/**
 * Tiny bootstrap for an in-memory embedding service using the emb.js module.
 * This initializes a simple LRU cache and a placeholder embedding model that returns
 * random vectors. In a real setup, you'd use @xenova/transformers here.
 */
class DemoEmbeddingService {
  constructor() {
    this.config = new TodoziEmbeddingConfig();
    this.cache = new LRUEmbeddingCache(64);
    // Placeholder model; real use would load transformers and encode text.
    this.model = {
      async encode(texts) {
        // Return random vectors of 384 dims per the default config
        return texts.map(() => Array(384).fill(0).map(() => Math.random() - 0.5));
      },
    };
  }

  async generateEmbedding(text) {
    const vectors = await this.model.encode([text]);
    return vectors[0];
  }

  async findSimilarTasks(text, limit = 5) {
    // This is a thin wrapper; real similarity uses cosine over stored vectors.
    const q = await this.generateEmbedding(text);
    const items = Array.from(this.cache.cache).map(([, v]) => v);
    const scored = items
      .filter((e) => e.content_type === 'Task')
      .map((e) => ({
        content_id: e.content_id,
        content_type: e.content_type,
        similarity_score: cosineSimilarity(q, e.vector),
        text_content: e.text_content,
        tags: e.tags,
      }))
      .sort((a, b) => b.similarity_score - a.similarity_score)
      .slice(0, limit);
    return scored;
  }

  async embedTask(task) {
    const text = [
      `Task: ${task.action}`,
      task.context_notes ? `Description: ${task.context_notes}` : null,
      `Priority: ${task.priority}`,
      `Status: ${task.status}`,
      task.tags?.length ? `Tags: ${task.tags.join(', ')}` : null,
      task.assignee ? `Assignee: ${typeof task.assignee === 'string' ? task.assignee : JSON.stringify(task.assignee)}` : null,
      task.progress != null ? `Progress: ${task.progress}` : null,
    ]
      .filter(Boolean)
      .join('\n');

    const vector = await this.generateEmbedding(text);
    const key = `Task_${task.id}`;
    const entry = {
      vector,
      content_type: 'Task',
      content_id: task.id,
      text_content: text,
      tags: task.tags || [],
      created_at: new Date(),
      ttl_seconds: 24 * 3600,
    };
    this.cache.insert(key, entry);
    return task.id;
  }
}

// Simple cosine similarity for demo
function cosineSimilarity(a, b) {
  if (!a || !b || a.length !== b.length) return 0;
  let dot = 0, na = 0, nb = 0;
  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i];
    na += a[i] * a[i];
    nb += b[i] * b[i];
  }
  const denom = Math.sqrt(na) * Math.sqrt(nb);
  return denom === 0 ? 0 : dot / denom;
}

function sleep(ms) {
  return new Promise((res) => setTimeout(res, ms));
}

/**
 * Enhanced executeAiTask that also embeds the task to demonstrate semantic search.
 */
async function executeAiTaskEnhanced(storage, embeddingService, task) {
  const queueItem = {
    id: `aiq_${Math.random().toString(36).slice(2, 10)}`,
    title: `AI: ${task.action}`,
    description: `AI processing required for task: ${task.action}`,
    priority: task.priority,
    project_id: task.parent_project,
    created_at: new Date().toISOString(),
  };

  // Embed the task so we can search it later
  try {
    await embeddingService.embedTask(task);
  } catch (e) {
    console.warn(`[embed] Failed to embed task ${task.id}: ${e.message}`);
  }

  await storage.addQueueItem(queueItem);
  return `Task queued for AI processing: ${task.action} (Queue ID: ${queueItem.id})`;
}

/**
 * Main demo runner
 */
async function runExample5() {
  console.log('Example 5: Semantic Task Orchestrator');
  console.log('=====================================\n');

  const storage = new DemoStorage();
  const embeddingService = new DemoEmbeddingService();

  // Simulated "chat" message containing mixed tags
  const message = `
Please help with the following:

<todozi>
Analyze competitive features; 2h; high; ai-research; in_progress; ai
</todozi>

<memory>
standard; ran 5k this morning; improved cardio; nice weather; high; short; cardio,wellness
</memory>

<idea>
Implement semantic task search; share; high; ai,search
</idea>

<error>
UI freeze on load; intermittent; high; ui; web; tag1,tag2
</error>

<train>
instruction; how do I write clean code?; follow SOLID and DRY principles; dev-guide; quality,clean
</train>

<feel>
motivated; 8; Ready to ship this sprint!; weekly-summary
</feel>

<todozi>
Refactor auth module; 4h; critical; data-pipeline; todo; collaborative
</todozi>

<todozi>
Onboard data transformations; 1d; high; data-pipeline; todo; agent=data-lake-bot
</todozi>
  `.trim();

  const userId = 'demo_user_5';

  // 1) Parse the chat message to extract structured content
  console.log('1) Parsing chat message...\n');
  const content = processChatMessageExtended(message, userId);

  console.log(`- tasks: ${content.tasks.length}`);
  console.log(`- memories: ${content.memories.length}`);
  console.log(`- ideas: ${content.ideas.length}`);
  console.log(`- errors: ${content.errors.length}`);
  console.log(`- training_data: ${content.training_data.length}`);
  console.log(`- feelings: ${content.feelings.length}\n`);

  // 2) Persist memories, ideas, and errors locally (simulated)
  for (const memory of content.memories) storage.memories.set(memory.id, memory);
  for (const idea of content.ideas) storage.ideas.set(idea.id, idea);
  for (const error of content.errors) storage.errors.set(error.id, error);
  for (const t of content.training_data) storage.training.set(t.id, t);
  for (const f of content.feelings) {
    // feelings not stored in this demo; just log
    console.log(`[feel] ${f.emotion} (${f.intensity}/10): ${f.description}`);
  }

  // 3) Prepare tasks for workflow execution
  //    - In a real flow you'd also save tasks to storage here.
  const tasks = content.tasks;
  console.log(`\n2) Prepared ${tasks.length} tasks for execution\n`);

  // 4) Execute workflow with enhanced AI handler that embeds tasks
  console.log('3) Executing tasks...\n');

  // Note: In ES modules, monkey-patching exported functions is not recommended.
  // Instead, you would pass a custom handler to processWorkflow or use dependency injection.
  // For this example, we'll use the standard workflow execution.
  const results = await processWorkflow(tasks);

  console.log('\n4) Execution results:\n');
  results.forEach((r, i) => console.log(`- Task ${i + 1}: ${r}`));

  // 5) Demonstrate semantic search over embedded tasks
  console.log('\n5) Semantic search demo:\n');
  const query = 'clean code and refactoring';
  const similar = await embeddingService.findSimilarTasks(query, 5);
  if (similar.length === 0) {
    console.log(`No similar tasks found for "${query}" (you may need to embed tasks first).`);
  } else {
    console.log(`Similar tasks for "${query}":`);
    similar.forEach((s, i) => {
      console.log(`  ${i + 1}. [${(s.similarity_score * 100).toFixed(1)}%] ${s.text_content.split('\n')[0]}`);
    });
  }

  // 6) Show queue and assignments
  console.log('\n6) Queue and assignments snapshot:\n');
  console.log(`Queue items: ${storage.queueItems.length}`);
  storage.queueItems.forEach((q) => {
    console.log(` - ${q.id} | ${q.title} | prio=${q.priority} | proj=${q.project_id}`);
  });
  console.log(`Agent assignments: ${storage.assignments.length}`);
  storage.assignments.forEach((a) => {
    console.log(` - agent=${a.agent_id} -> task=${a.task_id} (${a.status})`);
  });

  console.log('\nDemo completed. ✅');
}

// Run the example when executed directly
runExample5().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});