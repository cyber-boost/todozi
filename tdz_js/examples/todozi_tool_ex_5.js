// File: examples/ex5_task_tag_embedding.js
//
// Example 5: End-to-end integration across models, tags, and embeddings
// - Creates a Task using models.js
// - Manages Tags with the TagManager (from tags.js)
// - Simulates embeddings for semantic matching
// - Shows basic filtering, fuzzy search, and simple statistics
//
// How to run:
//   node examples/ex5_task_tag_embedding.js
//
// Notes:
// - This example focuses on pure in-memory usage for clarity.
// - It uses helpers from models.js and tags.js, and minimal stubs for embeddings.
// - No external services or network calls are required.

import { Task, TaskCollection, TaskFilters, Priority, Status, Assignee } from '../todozi/models.js';
import { Tag, TagManager, TagSearchQuery, TagSearchEngine, TagSortBy } from '../todozi/tags.js';

import { v4: uuidv4 } from 'uuid.js';
import fs from 'fs';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// -------------------------------------------------------------------------------------------------
// Minimal embedding support (stub) - no external ML dependency
// -------------------------------------------------------------------------------------------------
class LRUEmbeddingCache {
  constructor(max_memory_mb = 8) {
    this.max_memory_mb = max_memory_mb;
    this.cache = new Map(); // key -> { vector, createdAt }
  }
  get(key) { return this.cache.get(key)?.vector; }
  set(key, vector) {
    if (this.cache.size >= 100) {
      const firstKey = this.cache.keys().next().value;
      this.cache.delete(firstKey);
    }
    this.cache.set(key, { vector, createdAt: new Date() });
  }
  keys() { return this.cache.keys(); }
  values() { return Array.from(this.cache.values()).map(v => v.vector); }
}

class EmbeddingStats {
  constructor(mean, stddev, min, max) {
    this.mean = mean; this.stddev = stddev; this.min = min; this.max = max;
  }
  static fromArray(arr) {
    const n = arr.length;
    const mean = arr.reduce((a, b) => a + b, 0) / (n || 1);
    const variance = arr.reduce((a, b) => a + (b - mean) * (b - mean), 0) / (n || 1);
    return new EmbeddingStats(mean, Math.sqrt(variance), Math.min(...arr), Math.max(...arr));
  }
}

class ModelEmbeddingResult {
  constructor(model, embedding, dim, ms) {
    this.model = model;
    this.embedding = embedding;
    this.dimensions = dim;
    this.generation_time_ms = ms;
  }
}

// Very simple PRNG-free 384-dim random vector using crypto
function randomEmbedding384() {
  const out = new Array(384);
  for (let i = 0; i < out.length; i++) {
    // Map random byte to [-1, 1]
    out[i] = (Math.random() - 0.5) * 2.0; // using Math.random for demo; could use crypto.randomBytes for full determinism
  }
  return out;
}

function cosineSim(a, b) {
  if (a.length !== b.length) return 0;
  let dot = 0, na = 0, nb = 0;
  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i];
    na += a[i] * a[i];
    nb += b[i] * b[i];
  }
  return (na === 0 || nb === 0) ? 0 : dot / (Math.sqrt(na) * Math.sqrt(nb));
}

// -------------------------------------------------------------------------------------------------
// Demo: Build a mini project with tasks, tags, and embedding stubs
// -------------------------------------------------------------------------------------------------
async function main() {
  console.log('Example 5: Task + Tag + Embedding Integration\n');

  // 1) Create some tags (using TagManager)
  console.log('1) Creating Tags...');
  const tagManager = new TagManager();
  const tag1 = await tagManager.createTag(new Tag({ name: 'frontend', category: 'area', color: '#4f46e5' }));
  const tag2 = await tagManager.createTag(new Tag({ name: 'bug', category: 'type', color: '#ef4444' }));
  const tag3 = await tagManager.createTag(new Tag({ name: 'urgent', category: 'priority', color: '#f59e0b' }));
  const tag4 = await tagManager.createTag(new Tag({ name: 'api', category: 'area', color: '#10b981' }));
  const tag5 = await tagManager.createTag(new Tag({ name: 'docs', category: 'type', color: '#3b82f6' }));
  console.log(`   Created ${tagManager.getAllTags().length} tags\n`);

  // 2) Create tasks (using models.js)
  console.log('2) Creating Tasks...');
  const tasks = new TaskCollection();

  const t1 = Task.newFull(
    'u_123',
    'Fix login form validation bug on Safari',
    '2 hours',
    Priority.High,
    'auth-service',
    Status.Todo,
    'human',
    ['frontend', 'bug'],
    [],
    'Related to Issue #2231',
    0
  );
  tasks.addTask(t1);

  const t2 = Task.newFull(
    'u_123',
    'Document new API rate limit policy',
    '3 hours',
    Priority.Medium,
    'platform-docs',
    Status.InProgress,
    'ai',
    ['api', 'docs'],
    [],
    'Collaborate with legal on wording',
    25
  );
  tasks.addTask(t2);

  const t3 = Task.newFull(
    'u_123',
    'Refactor CSS for dark mode support',
    '1 day',
    Priority.Low,
    'web-client',
    Status.Todo,
    'human',
    ['frontend'],
    [],
    'Better color contrast for accessibility',
    0
  );
  tasks.addTask(t3);

  const t4 = Task.newFull(
    'u_123',
    'Emergency: patch security vulnerability in auth middleware',
    '4 hours',
    Priority.Urgent,
    'auth-service',
    Status.InProgress,
    'ai',
    ['api', 'bug', 'urgent'],
    [],
    'CVE-2024-XXXX',
    60
  );
  tasks.addTask(t4);

  console.log(`   Created ${tasks.getAllTasks().length} tasks\n`);

  // 3) Tag usage + relationships
  console.log('3) Tag Relationships & Analytics...');
  await tagManager.addTagRelationship(tag1, tag2); // frontend -> bug
  await tagManager.addTagRelationship(tag4, tag2); // api -> bug
  await tagManager.addTagRelationship(tag4, tag5); // api -> docs

  const stats = tagManager.getTagStatistics();
  console.log(`   Total tags: ${stats.total_tags}`);
  console.log(`   Total categories: ${stats.total_categories}`);
  console.log(`   Total relationships: ${stats.total_relationships}`);
  console.log(`   Average usage: ${stats.average_usage.toFixed(2)}\n`);

  // 4) Search tasks by filters
  console.log('4) Filtering Tasks...');
  const openTasks = tasks.getFilteredTasks(new TaskFilters({ status: Status.Todo }));
  console.log(`   Open tasks: ${openTasks.length}`);
  const highOrUrgent = tasks.getFilteredTasks(new TaskFilters({ priority: Priority.High }));
  console.log(`   High priority tasks: ${highOrUrgent.length}`);

  const frontendTasks = tasks.getFilteredTasks(new TaskFilters({ search: 'frontend' }));
  console.log(`   Frontend-related tasks: ${frontendTasks.length}\n`);

  // 5) Advanced tag search: categories and fuzzy matching
  console.log('5) Advanced Tag Search...');
  const bugTags = tagManager.searchTags('bug');
  console.log(`   Tags matching "bug": ${bugTags.map(t => t.name).join(', ') || '(none)'}`);

  const searchEngine = new TagSearchEngine(tagManager);
  const areaCategory = searchEngine.advancedSearch(new TagSearchQuery({ category: 'area', sort_by: TagSortBy.Name }));
  console.log(`   Tags in 'area' category: ${areaCategory.map(t => t.name).join(', ') || '(none)'}`);

  const fuzzy = searchEngine.fuzzySearch('doc', 2);
  console.log(`   Fuzzy suggestions for "doc": ${fuzzy.map(([t, d]) => `${t.name}(${d})`).join(', ') || '(none)'}\n`);

  // 6) Embedding simulation for semantic similarity across tasks
  console.log('6) Simulated Embeddings & Similarity...');
  const embeddingCache = new LRUEmbeddingCache(32);

  function getOrCreateTaskVector(task) {
    const key = `task:${task.id}`;
    const cached = embeddingCache.get(key);
    if (cached) return cached;
    const vec = randomEmbedding384();
    // bias a few dimensions based on tags for demo clustering
    for (const tagName of task.tags || []) {
      const h = hashStringToIndex(tagName, vec.length);
      vec[h] += (tagName === 'bug' || tagName === 'urgent') ? 0.8 : 0.3;
    }
    embeddingCache.set(key, vec);
    return vec;
  }

  function hashStringToIndex(str, mod) {
    let h = 2166136261;
    for (let i = 0; i < str.length; i++) {
      h ^= str.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return Math.abs(h) % mod;
  }

  const allTasks = tasks.getAllTasks();
  const vectors = allTasks.map(t => getOrCreateTaskVector(t));

  // Task-level stats
  const dimStats = Array.from({ length: vectors[0].length }, (_, d) => {
    const col = vectors.map(v => v[d]);
    return EmbeddingStats.fromArray(col);
  });
  const globalStats = new EmbeddingStats(
    dimStats.map(s => s.mean).reduce((a, b) => a + b, 0) / dimStats.length,
    Math.sqrt(dimStats.map(s => s.stddev * s.stddev).reduce((a, b) => a + b, 0) / dimStats.length),
    Math.min(...dimStats.map(s => s.min)),
    Math.max(...dimStats.map(s => s.max))
  );
  console.log(`   Embedding dim: ${vectors[0].length}`);
  console.log(`   Global mean: ${globalStats.mean.toFixed(4)}, stddev: ${globalStats.stddev.toFixed(4)}`);

  // Find most similar pair
  let best = { a: 0, b: 1, sim: -1 };
  for (let i = 0; i < allTasks.length; i++) {
    for (let j = i + 1; j < allTasks.length; j++) {
      const sim = cosineSim(vectors[i], vectors[j]);
      if (sim > best.sim) best = { a: i, b: j, sim };
    }
  }
  const ta = allTasks[best.a], tb = allTasks[best.b];
  console.log(`   Most similar tasks: "${ta.action}" vs "${tb.action}" (sim=${best.sim.toFixed(3)})\n`);

  // Semantic query: find tasks similar to "fix api bug urgently"
  const queryVec = randomEmbedding384();
  const query = 'fix api bug urgently';
  const keywords = ['fix', 'api', 'bug', 'urgent'];
  const results = allTasks
    .map((t, i) => ({ task: t, score: cosineSim(queryVec, vectors[i]) }))
    .sort((a, b) => b.score - a.score)
    .slice(0, 3);
  console.log(`7) Semantic Query: "${query}"`);
  results.forEach((r, idx) => {
    console.log(`   ${idx + 1}. [${(r.score * 100).toFixed(1)}%] ${r.task.action}`);
  });
  console.log();

  // 8) Example: suggest tags based on co-occurrence
  console.log('8) Tag Suggestions (basic co-occurrence)...');
  const tagCounts = {};
  for (const t of allTasks) {
    (t.tags || []).forEach(tag => { tagCounts[tag] = (tagCounts[tag] || 0) + 1; });
  }
  const suggestions = Object.entries(tagCounts)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 5)
    .map(([name]) => name);
  console.log(`   Top tags: ${suggestions.join(', ') || '(none)'}\n`);

  // 9) Persist to disk (optional, if you want to see files)
  const outputDir = './examples/output';
  fs.mkdirSync(outputDir, { recursive: true });

  fs.writeFileSync(path.join(outputDir, 'tasks.json'), JSON.stringify(allTasks, null, 2));
  fs.writeFileSync(path.join(outputDir, 'tags.json'), JSON.stringify(tagManager.getAllTags(), null, 2));
  console.log(`9) Wrote demo artifacts to: ${outputDir}`);
  console.log(`   - tasks.json`);
  console.log(`   - tags.json\n`);

  console.log('Done.');
}

main().catch(err => {
  console.error('Error running example:', err);
  process.exit(1);
});