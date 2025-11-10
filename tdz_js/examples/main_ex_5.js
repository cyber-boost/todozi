import { TodoziEmbeddingConfig, TodoziEmbeddingService, LRUEmbeddingCache, TodoziEmbeddingCache, TodoziContentType, Priority, Status } from '../todozi/emb.js';
import path from 'path';
import fs from 'fs';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**

 * example5.js
 *
 * A practical, self-contained example that uses the Todozi embedding service
 * to:
 *  - Initialize an in-memory cache of embedded tasks
 *  - Add tasks and auto-embed them
 *  - Perform semantic search across tasks
 *  - Cluster embedded content
 *  - Export diagnostics
 *  - Export embedded tasks to HLX format
 *
 * This file is executable with: node example5.js
 *
 * Notes:
 * - The actual embedding model is stubbed in emb.js (EmbeddingModel). The service
 *   will still run and produce randomized vectors to demonstrate the workflow.
 * - If you later plug in a real transformer model in emb.js, the same APIs here
 *   will work with real embeddings.
 */


// -----------------------
// Simple in-memory storage
// -----------------------
class MockStorage {
  constructor() {
    this.tasks = new Map();
  }

  addTask(task) {
    this.tasks.set(task.id, task);
  }

  async listTasksAcrossProjects() {
    return Array.from(this.tasks.values());
  }
}

// -----------------------
// Utilities
// -----------------------
function nowISO() {
  return new Date().toISOString();
}

function makeTask({ id, action, priority, status, project, tags = [], contextNotes = '' }) {
  return {
    id,
    action,
    priority,
    status,
    parentProject: project,
    tags,
    contextNotes,
    createdAt: nowISO(),
    updatedAt: nowISO(),
    embedding_vector: null
  };
}

// -----------------------
// Main
// -----------------------
async function main() {
  console.log('============================================================');
  console.log('Todozi Embedding Service - Practical Example #5');
  console.log('============================================================\n');

  // 1) Prepare dependencies
  const storage = new MockStorage();

  // Seed some tasks
  const tasks = [
    makeTask({
      id: 'task_1001',
      action: 'Implement user authentication with JWT',
      priority: Priority.High,
      status: Status.Todo,
      project: 'auth-service',
      tags: ['auth', 'security', 'backend'],
      contextNotes: 'Use short-lived access tokens and refresh tokens'
    }),
    makeTask({
      id: 'task_1002',
      action: 'Design database schema for orders and invoices',
      priority: Priority.Medium,
      status: Status.InProgress,
      project: 'billing',
      tags: ['database', 'schema', 'billing'],
      contextNotes: 'Ensure normalized design and foreign key constraints'
    }),
    makeTask({
      id: 'task_1003',
      action: 'Write unit tests for payment gateway integration',
      priority: Priority.High,
      status: Status.Todo,
      project: 'payments',
      tags: ['tests', 'payments', 'integration'],
      contextNotes: 'Mock external APIs; cover success and failure paths'
    }),
    makeTask({
      id: 'task_1004',
      action: 'Create Dockerfile for Node.js API service',
      priority: Priority.Low,
      status: Status.Todo,
      project: 'devops',
      tags: ['docker', 'devops', 'infrastructure'],
      contextNotes: 'Use multi-stage build and non-root user'
    }),
    makeTask({
      id: 'task_1005',
      action: 'Optimize slow SQL query in customer report',
      priority: Priority.Critical,
      status: Status.InProgress,
      project: 'analytics',
      tags: ['sql', 'performance', 'database'],
      contextNotes: 'Add composite index and avoid SELECT *'
    })
  ];

  tasks.forEach(t => storage.addTask(t));

  // 2) Initialize Embedding Service
  const config = new TodoziEmbeddingConfig();
  config.similarity_threshold = 0.2;   // Lower threshold for demo with random vectors
  config.max_results = 10;
  config.cache_ttl_seconds = 3600;
  config.enable_clustering = true;
  config.clustering_threshold = 0.6;

  const cache = new LRUEmbeddingCache(64); // 64 MB budget (approximate)

  const service = TodoziEmbeddingService.withSharedComponents(
    config,
    cache,
    null,  // embeddingModel (stubbed)
    new Map(),
    null,  // tagManager
    storage
  );

  await service.initialize();
  console.log('✅ Embedding service initialized\n');

  // 3) Embed tasks
  console.log('📝 Embedding tasks...');
  for (const task of tasks) {
    const content = service.prepareTaskContent(task);
    const vector = await service.generateEmbedding(content);
    task.embedding_vector = vector;

    const cacheKey = `${TodoziContentType.Task}_${task.id}`;
    const cacheEntry = new TodoziEmbeddingCache(
      vector,
      TodoziContentType.Task,
      task.id,
      content,
      task.tags,
      new Date(),
      config.cache_ttl_seconds
    );
    cache.insert(cacheKey, cacheEntry);

    console.log(`   • Embedded: [${task.id}] ${task.action.substring(0, 48)}...`);
  }
  console.log('✅ All tasks embedded\n');

  // 4) Semantic search examples
  console.log('🔍 Semantic search examples\n');

  const queries = [
    'Database performance tuning and indexing',
    'Authentication and token management',
    'Containerize Node.js service',
    'Write automated tests for payments'
  ];

  for (const q of queries) {
    console.log(`Query: "${q}"`);
    const results = await service.semanticSearch(q, null, 5);
    if (results.length === 0) {
      console.log('   No similar tasks found.\n');
      continue;
    }
    for (const r of results) {
      const preview = r.text_content.split('\n')[0].substring(0, 64);
      console.log(`   • [${r.content_id}] ${preview} (score=${r.similarity_score.toFixed(3)})`);
    }
    console.log('');
  }

  // 5) Clustering
  console.log('🧩 Clustering embedded content...');
  const clusters = await service.clusterContent();
  if (clusters.length === 0) {
    console.log('   No clusters found (threshold may be too high).\n');
  } else {
    for (const c of clusters) {
      console.log(`   Cluster #${c.cluster_id}: ${c.cluster_size} items, avg_similarity=${c.average_similarity.toFixed(3)}`);
      for (const item of c.content_items.slice(0, 3)) {
        const textPreview = item.text_content.split('\n')[0].substring(0, 48);
        console.log(`      - [${item.content_id}] ${textPreview} (score=${item.similarity_score.toFixed(3)})`);
      }
      if (c.content_items.length > 3) {
        console.log(`      ... and ${c.content_items.length - 3} more`);
      }
      console.log('');
    }
  }

  // 6) Export diagnostics
  console.log('📊 Exporting diagnostics...');
  const diag = await service.exportDiagnostics();
  const diagPath = path.join(process.cwd(), 'example5_embeddings_diagnostics.json');
  await fs.writeFile(diagPath, JSON.stringify(diag, null, 2), 'utf8');
  console.log(`✅ Diagnostics written to: ${diagPath}\n`);

  // 7) Export embedded tasks to HLX format
  console.log('🧠 Exporting embedded task vectors to HLX format...');
  const exportPath = path.join(process.cwd(), 'example5_embeddings_export.json');

  // Implement a minimal exporter compatible with storage.exportEmbeddedTasksHlx signature
  async function exportEmbeddedTasksHlx(outputFile) {
    const allTasks = await storage.listTasksAcrossProjects();
    const payload = [];

    for (const t of allTasks) {
      if (!t.embedding_vector || t.embedding_vector.length === 0) continue;

      payload.push({
        type: 'task',
        id: t.id,
        project: t.parentProject,
        action: t.action,
        priority: t.priority,
        status: t.status,
        tags: t.tags,
        context: t.contextNotes || '',
        createdAt: t.createdAt,
        updatedAt: t.updatedAt,
        embedding: t.embedding_vector,
        embedding_dimensions: t.embedding_vector.length
      });
    }

    await fs.writeFile(outputFile, JSON.stringify({ tasks: payload }, null, 2), 'utf8');
  }

  await exportEmbeddedTasksHlx(exportPath);
  console.log(`✅ Embedded task vectors exported to: ${exportPath}\n`);

  // 8) Manual similarity examples
  console.log('📐 Manual cosine similarity examples\n');
  const t1 = tasks[1]; // 'Design database schema for orders and invoices'
  const t2 = tasks[4]; // 'Optimize slow SQL query in customer report'

  if (t1.embedding_vector && t2.embedding_vector) {
    const sim = service.cosineSimilarity(t1.embedding_vector, t2.embedding_vector);
    console.log(`Similarity between:
  A) ${t1.action}
  B) ${t2.action}
  Score: ${sim.toFixed(3)} (higher is more similar)\n`);
  }

  // 9) Cache info
  console.log(`💾 Cache status: ${cache.len()} entries, empty=${cache.isEmpty()}\n`);

  console.log('============================================================');
  console.log('Example #5 complete');
  console.log('============================================================');
}

main().catch((err) => {
  console.error('❌ Fatal error:', err);
  process.exit(1);
});