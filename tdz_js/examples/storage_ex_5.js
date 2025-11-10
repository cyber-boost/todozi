// Example 5: Semantic search, recommendations, clustering, and diagnostics using embeddings
// Run: node example5.js
// Requires Node.js 18+
// Optional dependencies (for richer output): npm i luxon
// This example self-contains a mock embedding service, so it runs without external ML libraries.


// Optional dependency
import path from 'path';
import fs from 'fs';
import crypto from 'crypto';
import emb from './emb.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import { DateTime } from 'luxon';

// -----------------------------
// Minimal safe imports with graceful fallback
// -----------------------------
let EmbeddingServiceClass = null;
let SimilarityResultClass = null;
let ClusteringResultClass = null;
let ModelEmbeddingResultClass = null;
let ValidationReportClass = null;
let PerformanceMetricsClass = null;
let DiagnosticReportClass = null;
let EmbeddingStatsClass = null;
let TodoziContentType = null;

try {
  // If you place emb.js next to this file, this will work.
  EmbeddingServiceClass = emb.TodoziEmbeddingService;
  SimilarityResultClass = emb.SimilarityResult;
  ClusteringResultClass = emb.ClusteringResult;
  ModelEmbeddingResultClass = emb.ModelEmbeddingResult;
  ValidationReportClass = emb.ValidationReport;
  PerformanceMetricsClass = emb.PerformanceMetrics;
  DiagnosticReportClass = emb.DiagnosticReport;
  EmbeddingStatsClass = emb.EmbeddingStats;
  TodoziContentType = emb.TodoziContentType;
} catch (e) {
  // We'll keep a mock implementation to ensure the example stays executable.
  console.warn('Note: emb.js not found or could not be loaded. Using a mock embedding service for demonstration.');
}

// -----------------------------
// Mock Embedding Service (self-contained demo)
// -----------------------------

function hashStringToSeed(s) {
  const h = crypto.createHash('sha256').update(s).digest();
  // Use first 4 bytes as a 32-bit integer seed
  return h.readUInt32BE(0);
}

function mulberry32(seed) {
  return function () {
    let t = (seed += 0x6D2B79F5);
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function randomVector(len, seed) {
  const rand = mulberry32(seed);
  const v = new Array(len);
  let norm = 0;
  for (let i = 0; i < len; i++) {
    const x = (rand() * 2) - 1; // [-1, 1]
    v[i] = x;
    norm += x * x;
  }
  norm = Math.sqrt(norm) || 1;
  for (let i = 0; i < len; i++) v[i] /= norm;
  return v;
}

function cosineSimilarity(a, b) {
  if (a.length !== b.length) return 0;
  let dot = 0, na = 0, nb = 0;
  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i];
    na += a[i] * a[i];
    nb += b[i] * b[i];
  }
  const denom = Math.sqrt(na) * Math.sqrt(nb) || 1;
  return dot / denom;
}

const MockContentType = {
  Task: 'Task',
  Memory: 'Memory',
  Idea: 'Idea',
  Chunk: 'Chunk',
  Error: 'Error',
  Train: 'Train',
};

// Embedding cache entry
class MockCacheEntry {
  constructor(vector, content_type, content_id, text_content, tags, created_at, ttl_seconds) {
    this.vector = vector;
    this.content_type = content_type;
    this.content_id = content_id;
    this.text_content = text_content;
    this.tags = tags;
    this.created_at = created_at;
    this.ttl_seconds = ttl_seconds;
  }
}

class MockSimilarityResult {
  constructor(content_id, content_type, similarity_score, text_content, tags, metadata = {}) {
    this.content_id = content_id;
    this.content_type = content_type;
    this.similarity_score = similarity_score;
    this.text_content = text_content;
    this.tags = tags;
    this.metadata = metadata;
  }
}

class MockClusteringResult {
  constructor(cluster_id, content_items, cluster_center, cluster_size, average_similarity) {
    this.cluster_id = cluster_id;
    this.content_items = content_items;
    this.cluster_center = cluster_center;
    this.cluster_size = cluster_size;
    this.average_similarity = average_similarity;
  }
}

class MockEmbeddingModel {
  constructor(dimensions = 384) {
    this.dimensions = dimensions;
  }
  static async load(_modelName, _device) {
    return new MockEmbeddingModel(384);
  }
  async encode(texts) {
    return texts.map((t, i) => randomVector(this.dimensions, hashStringToSeed(String(t)) + i * 97));
  }
  static async getDefaultModel() {
    return 'mock/all-MiniLM-L6-v2';
  }
}

class MockTodoziEmbeddingService {
  constructor(config) {
    this.config = config || { similarity_threshold: 0.6, max_results: 25, clustering_threshold: 0.85, enable_clustering: true, cache_ttl_seconds: 86400, model_name: 'mock/all-MiniLM-L6-v2' };
    this.cache = new Map();
    this.model = new MockEmbeddingModel(384);
    this.accessCounts = new Map();
  }

  static async new(config) {
    const svc = new MockTodoziEmbeddingService(config);
    await svc.initialize();
    return svc;
  }

  async initialize() {
    // Nothing to do for mock
  }

  async generateEmbedding(text) {
    const v = await this.model.encode([text]);
    return v[0];
  }

  async semanticSearch(query, contentTypes = null, limit = null) {
    const q = await this.generateEmbedding(query);
    const L = limit || this.config.max_results;
    const out = [];
    for (const [key, entry] of this.cache.entries()) {
      if (contentTypes && !contentTypes.includes(entry.content_type)) continue;
      const score = cosineSimilarity(q, entry.vector);
      if (score >= this.config.similarity_threshold) {
        out.push(new MockSimilarityResult(entry.content_id, entry.content_type, score, entry.text_content, entry.tags, {}));
      }
    }
    out.sort((a, b) => b.similarity_score - a.similarity_score);
    return out.slice(0, L);
  }

  async findSimilarTasks(taskDescription, limit = null) {
    const q = await this.generateEmbedding(taskDescription);
    const L = limit || this.config.max_results;
    const out = [];
    for (const [key, entry] of this.cache.entries()) {
      if (entry.content_type !== MockContentType.Task) continue;
      const score = cosineSimilarity(q, entry.vector);
      if (score >= this.config.similarity_threshold) {
        out.push(new MockSimilarityResult(entry.content_id, entry.content_type, score, entry.text_content, entry.tags, {}));
      }
    }
    out.sort((a, b) => b.similarity_score - a.similarity_score);
    return out.slice(0, L);
  }

  async findSimilarTags(tagName, limit = null) {
    const q = await this.generateEmbedding(tagName);
    const L = limit || this.config.max_results;
    const out = [];
    for (const [key, entry] of this.cache.entries()) {
      if (entry.content_type !== 'Tag') continue;
      const score = cosineSimilarity(q, entry.vector);
      if (score >= this.config.similarity_threshold) {
        out.push(new MockSimilarityResult(entry.content_id, entry.content_type, score, entry.text_content, entry.tags, {}));
      }
    }
    out.sort((a, b) => b.similarity_score - a.similarity_score);
    return out.slice(0, L);
  }

  async clusterContent() {
    if (!this.config.enable_clustering) return [];
    const entries = Array.from(this.cache.values());
    const threshold = this.config.clustering_threshold;
    const used = new Set();
    const clusters = [];

    for (let i = 0; i < entries.length; i++) {
      if (used.has(i)) continue;
      const seed = entries[i];
      const cluster = [new MockSimilarityResult(seed.content_id, seed.content_type, 1.0, seed.text_content, seed.tags, {})];
      used.add(i);
      for (let j = i + 1; j < entries.length; j++) {
        if (used.has(j)) continue;
        const other = entries[j];
        const sim = cosineSimilarity(seed.vector, other.vector);
        if (sim >= threshold) {
          cluster.push(new MockSimilarityResult(other.content_id, other.content_type, sim, other.text_content, other.tags, {}));
          used.add(j);
        }
      }
      if (cluster.length > 1) {
        const avgSim = cluster.slice(1).reduce((s, it) => s + it.similarity_score, 0) / (cluster.length - 1);
        clusters.push(new MockClusteringResult(crypto.randomUUID(), cluster, seed.vector, cluster.length, avgSim));
      }
    }
    return clusters;
  }

  async buildSimilarityGraph(threshold = 0.8) {
    const nodes = [];
    const edges = [];
    const entries = Array.from(this.cache.values());
    for (const entry of entries) {
      nodes.push({
        id: entry.content_id,
        content_type: entry.content_type,
        label: entry.text_content.slice(0, 60),
        metadata: { tags: entry.tags }
      });
    }
    for (let i = 0; i < entries.length; i++) {
      for (let j = i + 1; j < entries.length; j++) {
        const a = entries[i], b = entries[j];
        const s = cosineSimilarity(a.vector, b.vector);
        if (s >= threshold) {
          edges.push({ from: a.content_id, to: b.content_id, similarity: s, bidirectional: true });
        }
      }
    }
    return { nodes, edges };
  }

  async findOutliers(contentType, threshold = 0.3) {
    const items = Array.from(this.cache.values()).filter(e => e.content_type === contentType);
    const out = [];
    for (let i = 0; i < items.length; i++) {
      let maxSim = 0;
      for (let j = 0; j < items.length; j++) {
        if (i === j) continue;
        maxSim = Math.max(maxSim, cosineSimilarity(items[i].vector, items[j].vector));
      }
      if (maxSim < threshold) out.push(items[i].content_id);
    }
    return out;
  }

  async recommendSimilar(basedOn, exclude = [], limit = 10) {
    const excludeSet = new Set(exclude);
    const basedOnSet = new Set(basedOn);
    const vecs = [];
    const dims = 384;
    for (const id of basedOn) {
      const entry = Array.from(this.cache.values()).find(e => e.content_id === id);
      if (entry) vecs.push(entry.vector);
    }
    if (vecs.length === 0) return [];
    const centroid = new Array(dims).fill(0);
    for (const v of vecs) for (let i = 0; i < dims; i++) centroid[i] += v[i];
    for (let i = 0; i < dims; i++) centroid[i] /= vecs.length;

    const out = [];
    for (const [key, entry] of this.cache.entries()) {
      if (excludeSet.has(entry.content_id) || basedOnSet.has(entry.content_id)) continue;
      const score = cosineSimilarity(centroid, entry.vector);
      if (score >= this.config.similarity_threshold) {
        out.push(new MockSimilarityResult(entry.content_id, entry.content_type, score, entry.text_content, entry.tags, {}));
      }
    }
    out.sort((a, b) => b.similarity_score - a.similarity_score);
    return out.slice(0, limit);
  }

  async getOrGenerateEmbedding(contentId, text, contentType, refreshIfStale = false) {
    const key = `${contentType}_${contentId}`;
    const now = new Date();
    const entry = this.cache.get(key);
    if (entry) {
      const expiry = new Date(new Date(entry.created_at).getTime() + entry.ttl_seconds * 1000);
      if (expiry > now || !refreshIfStale) {
        return entry.vector;
      }
    }
    const vec = await this.generateEmbedding(text);
    this.cache.set(key, new MockCacheEntry(vec, contentType, contentId, text, [], new Date(), this.config.cache_ttl_seconds));
    return vec;
  }

  async embedIdea(idea) {
    const content = `Idea: ${idea.idea} Importance: ${idea.importance || 'medium'} Share: ${idea.share || 'private'} Tags: ${(idea.tags || []).join(', ')}`;
    const id = idea.id || crypto.randomUUID();
    const vec = await this.generateEmbedding(content);
    this.cache.set(`Idea_${id}`, new MockCacheEntry(vec, MockContentType.Idea, id, content, idea.tags || [], new Date(), this.config.cache_ttl_seconds));
    return id;
  }

  async embedMemory(memory) {
    const content = `Memory: ${memory.moment} Meaning: ${memory.meaning} Reason: ${memory.reason} Importance: ${memory.importance || 'medium'} Term: ${memory.term || 'short'} Tags: ${(memory.tags || []).join(', ')}`;
    const id = memory.id || crypto.randomUUID();
    const vec = await this.generateEmbedding(content);
    this.cache.set(`Memory_${id}`, new MockCacheEntry(vec, MockContentType.Memory, id, content, memory.tags || [], new Date(), this.config.cache_ttl_seconds));
    return id;
  }

  async cleanupExpired() {
    const now = new Date();
    const keys = [];
    for (const [k, v] of this.cache.entries()) {
      const expiry = new Date(new Date(v.created_at).getTime() + v.ttl_seconds * 1000);
      if (expiry <= now) keys.push(k);
    }
    for (const k of keys) this.cache.delete(k);
    return keys.length;
  }

  async getStats() {
    const typeCounts = {};
    for (const e of this.cache.values()) {
      typeCounts[e.content_type] = (typeCounts[e.content_type] || 0) + 1;
    }
    return { total_embeddings: this.cache.size, type_counts: typeCounts };
  }

  async exportDiagnostics() {
    const total = this.cache.size;
    const items = Array.from(this.cache.values());
    const typeBreakdown = {};
    for (const e of items) typeBreakdown[e.content_type] = (typeBreakdown[e.content_type] || 0) + 1;
    let avgSim = 0, count = 0;
    const dims = 384;
    const means = new Array(dims).fill(0);
    const mins = new Array(dims).fill(Infinity);
    const maxs = new Array(dims).fill(-Infinity);
    for (const e of items) {
      for (let i = 0; i < dims; i++) {
        const v = e.vector[i];
        means[i] += v;
        mins[i] = Math.min(mins[i], v);
        maxs[i] = Math.max(maxs[i], v);
      }
    }
    for (let i = 0; i < dims; i++) means[i] /= Math.max(1, total);
    for (let i = 0; i < items.length; i++) {
      for (let j = i + 1; j < items.length; j++) {
        avgSim += cosineSimilarity(items[i].vector, items[j].vector);
        count++;
      }
    }
    avgSim = count > 0 ? avgSim / count : 0;
    const stdDevs = new Array(dims).fill(0);
    for (const e of items) {
      for (let i = 0; i < dims; i++) {
        const diff = e.vector[i] - means[i];
        stdDevs[i] += diff * diff;
      }
    }
    for (let i = 0; i < dims; i++) stdDevs[i] = Math.sqrt(stdDevs[i] / Math.max(1, total));
    return {
      timestamp: new Date().toISOString(),
      cache_hit_rate: 0.0, // unknown in mock
      avg_similarity_score: avgSim,
      embedding_distribution_stats: { mean: means, std_dev: stdDevs, min: mins, max: maxs },
      content_type_breakdown: typeBreakdown,
      top_similar_pairs: []
    };
  }
}

// -----------------------------
// Demo Data
// -----------------------------

const DEMO_TASKS = [
  { id: 'task-1', action: 'Implement semantic search for tasks', project: 'search', priority: 'high', status: 'in_progress', tags: ['search', 'embedding'] },
  { id: 'task-2', action: 'Write unit tests for clustering', project: 'search', priority: 'medium', status: 'todo', tags: ['testing', 'clustering'] },
  { id: 'task-3', action: 'Refactor auth module and fix security issues', project: 'security', priority: 'critical', status: 'todo', tags: ['security', 'auth'] },
  { id: 'task-4', action: 'Design dashboard wireframes', project: 'ui', priority: 'low', status: 'done', tags: ['design', 'ui'] },
  { id: 'task-5', action: 'Optimize database queries', project: 'backend', priority: 'high', status: 'in_progress', tags: ['database', 'performance'] },
];

const DEMO_MEMORIES = [
  { id: 'mem-1', moment: 'User asked about semantic search', meaning: 'They want better relevance', reason: 'Feedback', importance: 'high', term: 'short', tags: ['feedback', 'search'] },
  { id: 'mem-2', moment: 'Clustering thresholds too strict', meaning: 'Few clusters formed', reason: 'A/B test', importance: 'medium', term: 'long', tags: ['clustering'] },
  { id: 'mem-3', moment: 'Security patch deployed', meaning: 'Auth fixed', reason: 'Incident', importance: 'critical', term: 'short', tags: ['security', 'patch'] },
];

const DEMO_IDEAS = [
  { id: 'idea-1', idea: 'Use hybrid search (keywords + embeddings)', importance: 'high', share: 'team', tags: ['search', 'hybrid'] },
  { id: 'idea-2', idea: 'Introduce tag-based recommendations', importance: 'medium', share: 'private', tags: ['recommendations', 'tags'] },
];

const DEMO_ERRORS = [
  { id: 'err-1', title: 'Unhandled promise rejection', severity: 'high', category: 'runtime', source: 'api' },
  { id: 'err-2', title: 'Index out of bounds', severity: 'medium', category: 'logic', source: 'search' },
];

// -----------------------------
// Utilities
// -----------------------------

function prepareTaskContent(task) {
  return `Task: ${task.action} Priority: ${task.priority} Status: ${task.status} Tags: ${(task.tags || []).join(', ')} Project: ${task.project}`;
}

function prepareMemoryContent(m) {
  return `Memory: ${m.moment} Meaning: ${m.meaning} Reason: ${m.reason} Importance: ${m.importance} Term: ${m.term} Tags: ${(m.tags || []).join(', ')}`;
}

function prepareIdeaContent(idea) {
  return `Idea: ${idea.idea} Importance: ${idea.importance} Share: ${idea.share} Tags: ${(idea.tags || []).join(', ')}`;
}

// -----------------------------
// Main demonstration
// -----------------------------
(async function main() {
  const svc = await MockTodoziEmbeddingService.new({ similarity_threshold: 0.65, max_results: 10, clustering_threshold: 0.85, enable_clustering: true });

  // Index demo data
  for (const t of DEMO_TASKS) {
    const key = `Task_${t.id}`;
    const content = prepareTaskContent(t);
    const vec = await svc.generateEmbedding(content);
    svc.cache.set(key, new MockCacheEntry(vec, MockContentType.Task, t.id, content, t.tags, new Date(), 86400));
  }
  for (const m of DEMO_MEMORIES) {
    const key = `Memory_${m.id}`;
    const content = prepareMemoryContent(m);
    const vec = await svc.generateEmbedding(content);
    svc.cache.set(key, new MockCacheEntry(vec, MockContentType.Memory, m.id, content, m.tags, new Date(), 86400));
  }
  for (const i of DEMO_IDEAS) {
    const key = `Idea_${i.id}`;
    const content = prepareIdeaContent(i);
    const vec = await svc.generateEmbedding(content);
    svc.cache.set(key, new MockCacheEntry(vec, MockContentType.Idea, i.id, content, i.tags, new Date(), 86400));
  }
  for (const e of DEMO_ERRORS) {
    const key = `Error_${e.id}`;
    const content = `Error: ${e.title} Severity: ${e.severity} Category: ${e.category} Source: ${e.source}`;
    const vec = await svc.generateEmbedding(content);
    svc.cache.set(key, new MockCacheEntry(vec, MockContentType.Error, e.id, content, [], new Date(), 86400));
  }

  // 1) Semantic search
  console.log('\n1) Semantic search for "search relevance and embeddings"');
  const results = await svc.semanticSearch('search relevance and embeddings', [MockContentType.Task, MockContentType.Idea], 5);
  for (const r of results) {
    console.log(` - [${(r.similarity_score * 100).toFixed(1)}%] ${r.content_type}: ${r.text_content.split('\n')[0]}`);
  }

  // 2) Task similar to "unit tests"
  console.log('\n2) Tasks similar to "unit tests"');
  const simTasks = await svc.findSimilarTasks('unit tests', 3);
  for (const r of simTasks) {
    console.log(` - [${(r.similarity_score * 100).toFixed(1)}%] ${r.text_content.split('\n')[0]}`);
  }

  // 3) Clustering
  console.log('\n3) Content clusters');
  const clusters = await svc.clusterContent();
  clusters.forEach((c, idx) => {
    console.log(` Cluster #${idx + 1} (size=${c.cluster_size}, avgSim=${(c.average_similarity * 100).toFixed(1)}%)`);
    c.content_items.forEach((it, j) => {
      console.log(`   - [${(it.similarity_score * 100).toFixed(1)}%] ${it.content_type}: ${it.text_content.split('\n')[0]}`);
    });
  });

  // 4) Similarity graph (compact summary)
  console.log('\n4) Similarity graph edges (threshold 0.8)');
  const graph = await svc.buildSimilarityGraph(0.8);
  const topEdges = graph.edges
    .slice()
    .sort((a, b) => b.similarity - a.similarity)
    .slice(0, 8);
  for (const e of topEdges) {
    const a = graph.nodes.find(n => n.id === e.from);
    const b = graph.nodes.find(n => n.id === e.to);
    console.log(` - ${a.label} <-> ${b.label} (${(e.similarity * 100).toFixed(1)}%)`);
  }

  // 5) Outliers
  console.log('\n5) Outlier tasks (low similarity to others)');
  const outliers = await svc.findOutliers(MockContentType.Task, 0.35);
  const taskById = Object.fromEntries(DEMO_TASKS.map(t => [t.id, t]));
  outliers.forEach(id => console.log(` - ${id}: ${taskById[id]?.action || 'Unknown task'}`));

  // 6) Recommendations based on a seed task
  console.log('\n6) Recommendations based on "task-2" (unit tests)');
  const recs = await svc.recommendSimilar(['task-2'], [], 5);
  for (const r of recs) {
    console.log(` - [${(r.similarity_score * 100).toFixed(1)}%] ${r.content_type}: ${r.text_content.split('\n')[0]}`);
  }

  // 7) Diagnostics
  console.log('\n7) Diagnostics summary');
  const diag = await svc.exportDiagnostics();
  console.log(` - Total embeddings: ${diag.content_type_breakdown ? Object.values(diag.content_type_breakdown).reduce((a, b) => a + b, 0) : 'n/a'}`);
  console.log(` - Avg pairwise similarity: ${(diag.avg_similarity_score * 100).toFixed(2)}%`);
  console.log(` - Type breakdown: ${JSON.stringify(diag.content_type_breakdown)}`);

  // 8) Cleanup expired (mock: none will be expired)
  console.log('\n8) Cleanup expired entries');
  const removed = await svc.cleanupExpired();
  console.log(` - Removed: ${removed}`);

  // 9) Embed a new idea on the fly and search again
  const newIdea = { id: 'idea-3', idea: 'Add tag autocomplete to search bar', importance: 'medium', share: 'team', tags: ['ui', 'search'] };
  await svc.embedIdea(newIdea);
  console.log('\n9) After embedding a new idea, search "autocomplete search"');
  const newResults = await svc.semanticSearch('autocomplete search', [MockContentType.Idea], 3);
  for (const r of newResults) {
    console.log(` - [${(r.similarity_score * 100).toFixed(1)}%] ${r.content_type}: ${r.text_content.split('\n')[0]}`);
  }

  // 10) Performance profile for a query
  console.log('\n10) Performance profile for "clustering threshold tuning"');
  const iterations = 25;
  const times = [];
  for (let i = 0; i < iterations; i++) {
    const t0 = process.hrtime.bigint();
    await svc.semanticSearch('clustering threshold tuning', [MockContentType.Task, MockContentType.Idea, MockContentType.Memory], 10);
    const t1 = process.hrtime.bigint();
    times.push(Number(t1 - t0) / 1e6);
  }
  const avg = times.reduce((a, b) => a + b, 0) / times.length;
  const min = Math.min(...times);
  const max = Math.max(...times);
  const variance = times.reduce((s, x) => s + Math.pow(x - avg, 2), 0) / times.length;
  const stdDev = Math.sqrt(variance);
  console.log(` - Iterations: ${iterations}, avg: ${avg.toFixed(2)}ms, min: ${min.toFixed(2)}ms, max: ${max.toFixed(2)}ms, stdDev: ${stdDev.toFixed(2)}ms`);

  console.log('\nDone.');
})();