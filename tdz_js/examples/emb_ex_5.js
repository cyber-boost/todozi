import { createRequire } from 'module.js';

Here is a self-contained, executable example that shows how to use and extend the Todozi embedding service. It provides:

- A deterministic fake embedding model (no external ML dependencies).
- A simple in-memory storage adapter to make findSimilarTasks work.
- Practical workflows: embedding ideas/memories/tags, semantic search, hybrid search, multi-query search, clustering, similarity graph, recommendations, diagnostics, validation, and drift tracking.

/*

 * Example 5: Demonstrate practical usage of TodoziEmbeddingService
 *
 * How to run:
 *   1) Save this file as example5.js
 *   2) Run: node example5.js
 *
 * No external ML dependencies required. Uses a deterministic fake embedding model.
 */
const requirePkg = createRequire(import.meta.url);

// Re-export what we need from emb.js
const {
  TodoziEmbeddingService,
  TodoziEmbeddingConfig,
  EmbeddingModel,
  TodoziContentType,
  SimilarityResult,
  AggregationType,
} = requirePkg('./emb.js');

const { Priority, Status, Assignee } = requirePkg('./models.js');

// ---------- Simple In-Memory Storage Adapter ----------
class MemoryStorageAdapter {
  constructor() {
    this.tasks = new Map();
    this.memories = new Map();
    this.ideas = new Map();
    this.tags = new Map();
  }

  async addTaskToProject(task) {
    this.tasks.set(task.id, task);
    return task.id;
  }

  async listTasksAcrossProjects() {
    return Array.from(this.tasks.values());
  }

  // Helper to create tasks for demo
  static sampleTasks() {
    const now = new Date().toISOString();
    return [
      {
        id: 'task_001',
        action: 'Implement user authentication module',
        time: '4 hours',
        priority: Priority.High,
        parentProject: 'auth-service',
        status: Status.InProgress,
        assignee: Assignee.Agent('coder'),
        tags: ['security', 'auth', 'node'],
        dependencies: [],
        context_notes: 'Focus on password hashing and JWT issuance',
        progress: 35,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
      {
        id: 'task_002',
        action: 'Design database schema for orders and customers',
        time: '3 hours',
        priority: Priority.Medium,
        parentProject: 'catalog-service',
        status: Status.Todo,
        assignee: Assignee.Human,
        tags: ['database', 'schema', 'design'],
        dependencies: [],
        context_notes: 'Normalize customer data and consider indexing strategy',
        progress: 0,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
      {
        id: 'task_003',
        action: 'Set up CI/CD pipeline for staging',
        time: '2 hours',
        priority: Priority.High,
        parentProject: 'devops',
        status: Status.Todo,
        assignee: Assignee.Agent('devops'),
        tags: ['ci', 'cd', 'devops'],
        dependencies: [],
        context_notes: 'Use GitHub Actions; add smoke tests',
        progress: 0,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
      {
        id: 'task_004',
        action: 'Optimize slow database queries',
        time: '5 hours',
        priority: Priority.Critical,
        parentProject: 'backend',
        status: Status.Blocked,
        assignee: Assignee.Collaborative,
        tags: ['database', 'performance', 'sql'],
        dependencies: ['task_002'],
        context_notes: 'Need to identify top offenders with EXPLAIN ANALYZE',
        progress: 10,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
      {
        id: 'task_005',
        action: 'Create login UI components',
        time: '3 hours',
        priority: Priority.Medium,
        parentProject: 'frontend',
        status: Status.Todo,
        assignee: Assignee.Agent('designer'),
        tags: ['ui', 'frontend', 'components'],
        dependencies: [],
        context_notes: 'Follow design system; ensure accessibility',
        progress: 0,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
      {
        id: 'task_006',
        action: 'Refactor legacy authentication code',
        time: '6 hours',
        priority: Priority.High,
        parentProject: 'auth-service',
        status: Status.Todo,
        assignee: Assignee.Ai,
        tags: ['refactor', 'auth', 'security'],
        dependencies: [],
        context_notes: 'Reduce complexity and add tests',
        progress: 0,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
      {
        id: 'task_007',
        action: 'Write unit tests for user service',
        time: '2 hours',
        priority: Priority.Medium,
        parentProject: 'auth-service',
        status: Status.Todo,
        assignee: Assignee.Agent('tester'),
        tags: ['testing', 'unit', 'coverage'],
        dependencies: [],
        context_notes: 'Aim for >85% coverage',
        progress: 0,
        createdAt: now,
        updatedAt: now,
        embedding_vector: null,
      },
    ];
  }
}

// ---------- Deterministic Fake Embedding Model ----------
class FakeEmbeddingModel extends EmbeddingModel {
  constructor(dimensions = 384) {
    super(null, null, 'cpu', dimensions);
  }

  static async load(modelName, device) {
    console.log(`[FakeEmbeddingModel] Loading model: ${modelName} on device: ${device}`);
    return new FakeEmbeddingModel(384);
  }

  static async getDefaultModel() {
    return 'fake/all-minilm-deterministic-v1';
  }

  async saveAsDefault(modelName) {
    // No-op for demo
  }

  // Deterministic pseudo-random vector based on string content
  async encode(texts) {
    return texts.map((text) => FakeEmbeddingModel.vectorFor(text));
  }

  static vectorFor(text) {
    const dim = 384;
    const vec = new Array(dim).fill(0);
    // Simple rolling hash
    let h1 = 2166136261;
    let h2 = 2654435761;

    for (let i = 0; i < text.length; i++) {
      const c = text.charCodeAt(i);
      h1 ^= c;
      h1 += (h1 << 1) + (h1 << 4) + (h1 << 7) + (h1 << 8) + (h1 << 24);
      h2 ^= c;
      h2 += (h2 << 1) + (h2 << 4) + (h2 << 7) + (h2 << 8) + (h2 << 24);
    }

    for (let i = 0; i < dim; i++) {
      // Derive two pseudo-random sequences from h1/h2
      const x = Math.sin(h1 + i * 0.12345) * 43758.5453;
      const y = Math.sin(h2 + i * 0.67890) * 24634.6345;
      const val = (x - Math.floor(x)) + (y - Math.floor(y)) - 1.0; // roughly in [-1, 1]
      vec[i] = isFinite(val) ? val : 0.0;
    }
    // Normalize
    const norm = Math.sqrt(vec.reduce((s, v) => s + v * v, 0)) || 1.0;
    for (let i = 0; i < dim; i++) vec[i] /= norm;
    return vec;
  }
}

// ---------- Demo Runner ----------
(async function main() {
  try {
    // 1) Create config and service
    const config = TodoziEmbeddingConfig.default();
    config.similarity_threshold = 0.55; // Lower for demo to get more hits
    config.clustering_threshold = 0.6;

    const storage = new MemoryStorageAdapter();
    const embeddingModel = await FakeEmbeddingModel.load('fake/all-minilm-deterministic-v1', 'cpu');
    const service = TodoziEmbeddingService.withSharedComponents(
      config,
      new Map(),           // internal cache
      embeddingModel,      // our fake model
      new Map(),           // model registry (unused here)
      null,                // tag manager (unused here)
      storage              // storage adapter (for tasks)
    );
    await service.initialize();

    console.log('✅ TodoziEmbeddingService initialized\n');

    // 2) Seed tasks
    const tasks = MemoryStorageAdapter.sampleTasks();
    for (const t of tasks) {
      await service.addTask(t);
    }
    console.log(`📋 Seeded ${tasks.length} tasks\n`);

    // 3) Embed some ideas and memories and tags (for other content types)
    const idea = {
      id: 'idea_001',
      idea: 'Use feature flags for gradual rollouts',
      importance: 'high',
      share: 'team',
      tags: ['feature-flags', 'release', 'risk'],
      context: 'We can reduce risk by turning on features for subsets of users'
    };
    await service.embedIdea(idea);

    const memory = {
      id: 'mem_001',
      moment: 'Deployed auth changes late on Friday',
      meaning: 'Deployment without proper safety caused incidents',
      reason: 'Rushed release',
      importance: 'high',
      term: 'long',
      memory_type: 'standard',
      tags: ['deployment', 'incident', 'process']
    };
    await service.embedMemory(memory);

    const tag = {
      id: 'tag_001',
      name: 'security',
      description: 'Security-related items',
      category: 'quality',
      color: '#e11d48',
      usage_count: 5
    };
    await service.embedTag(tag);

    console.log('🧠 Embedded 1 idea, 1 memory, 1 tag\n');

    // 4) Semantic search across all cached content
    const q1 = await service.semanticSearch('user login authentication', null, 8);
    console.log('🔎 Semantic search: "user login authentication"');
    printSimilarityResults(q1);

    // 5) Hybrid search (semantic + keyword)
    const q2 = await service.hybridSearch('database schema design', ['schema', 'design'], null, 0.6, 8);
    console.log('🔎 Hybrid search: "database schema design" (semanticWeight=0.6)');
    printSimilarityResults(q2);

    // 6) Multi-query search with aggregation
    const q3 = await service.multiQuerySearch(
      ['auth', 'CI/CD'],
      AggregationType.Average,
      null,
      8
    );
    console.log('🔎 Multi-query search: ["auth", "CI/CD"] (Average)');
    printSimilarityResults(q3);

    // 7) Find similar tasks (uses storage adapter)
    const q4 = await service.findSimilarTasks('Build a login page and token issuance flow', 5);
    console.log('🔎 Similar tasks: "Build a login page and token issuance flow"');
    printSimilarityResults(q4);

    // 8) Clustering of cached content
    const clusters = await service.clusterContent();
    console.log(`🔗 Found ${clusters.length} cluster(s)`);
    clusters.forEach((c, i) => {
      console.log(`  Cluster ${i + 1}: size=${c.cluster_size}, avg_similarity=${(c.average_similarity * 100).toFixed(1)}%`);
      c.content_items.slice(0, 3).forEach((item) => {
        console.log(`    - [${item.content_type}] ${firstLine(item.text_content)} (sim=${(item.similarity_score * 100).toFixed(1)}%)`);
      });
    });
    console.log('');

    // 9) Similarity graph
    const graph = await service.buildSimilarityGraph(0.5);
    console.log(`🕸️ Similarity graph: nodes=${graph.nodes.length}, edges=${graph.edges.length}`);
    if (graph.nodes.length > 0) {
      console.log('  Nodes:');
      graph.nodes.slice(0, 5).forEach((n) => {
        console.log(`    - ${n.id} (${n.content_type}) "${truncate(n.label, 40)}"`);
      });
    }
    console.log('');

    // 10) Recommend similar items (exclude some IDs)
    const q5 = await service.recommendSimilar(['task_001', 'task_002'], ['task_005'], 5);
    console.log('💡 Recommendations based on task_001 and task_002 (excluding task_005)');
    printSimilarityResults(q5);

    // 11) Tag suggestions for a content ID
    const suggestedTags = await service.suggestTags('task_004', 5);
    console.log(`🏷️ Suggested tags for task_004: ${suggestedTags.join(', ') || '(none)'}\n`);

    // 12) Drift tracking
    const drift = await service.trackEmbeddingDrift('idea_001', 'Use feature flags for safe, gradual rollouts to users');
    console.log('📈 Drift report for idea_001:');
    console.log(`  similarity_to_original=${(drift.current_similarity_to_original * 100).toFixed(1)}%`);
    console.log(`  drift=${drift.drift_percentage.toFixed(1)}%`);
    console.log(`  significant_drift=${drift.significant_drift}\n`);

    // 13) Validate embeddings
    const validation = await service.validateEmbeddings();
    console.log('🧪 Validation report:');
    console.log(`  total=${validation.total_embeddings}`);
    console.log(`  invalid=${validation.invalid_embeddings}`);
    console.log(`  nan_count=${validation.nan_count}, infinity_count=${validation.infinity_count}, zero_vector_count=${validation.zero_vector_count}`);
    if (validation.issues.length > 0) {
      console.log('  Sample issues:');
      validation.issues.slice(0, 3).forEach((iss) => {
        console.log(`    - ${iss.content_id} [${iss.severity}] ${iss.issue_type}: ${iss.description}`);
      });
    }
    console.log('');

    // 14) Performance profiling
    const perf = await service.profileSearchPerformance('auth security', 7);
    console.log('⏱️ Performance profile:');
    console.log(`  query="${perf.query}" iterations=${perf.iterations}`);
    console.log(`  avg=${perf.avg_time_ms.toFixed(1)}ms min=${perf.min_time_ms}ms max=${perf.max_time_ms}ms std=${perf.std_dev_ms.toFixed(1)}ms`);
    console.log(`  results/iter=${perf.results_per_iteration}\n`);

    // 15) Export diagnostics
    const diag = await service.exportDiagnostics();
    console.log('📊 Diagnostics:');
    console.log(`  timestamp=${new Date(diag.timestamp).toISOString()}`);
    console.log(`  cache_size=${Object.values(diag.content_type_breakdown).reduce((a, b) => a + b, 0)}`);
    console.log(`  avg_similarity=${(diag.avg_similarity_score * 100).toFixed(1)}%`);
    console.log(`  top_pairs=${diag.top_similar_pairs.length}`);
    if (diag.top_similar_pairs.length > 0) {
      const [a, b, s] = diag.top_similar_pairs[0];
      console.log(`  top_pair -> ${a} <-> ${b} : ${(s * 100).toFixed(1)}%`);
    }
    console.log('');

    console.log('✅ Demo completed successfully.');
  } catch (err) {
    console.error('❌ Demo failed:', err);
    process.exit(1);
  }
})();

// ---------- Utilities ----------
function firstLine(text) {
  return (text || '').split('\n')[0].trim();
}

function truncate(text, n) {
  if (!text) return '';
  return text.length > n ? text.slice(0, n - 1) + '…' : text;
}

function printSimilarityResults(results) {
  if (!results || results.length === 0) {
    console.log('  (no results)\n');
    return;
  }
  results.forEach((r, i) => {
    console.log(`  ${i + 1}. [${r.content_type}] ${truncate(firstLine(r.text_content), 60)} (${(r.similarity_score * 100).toFixed(1)}%)`);
  });
  console.log('');
}