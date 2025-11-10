// Utility: mock repository to hold our data
import { TodoziEmbeddingConfig, TodoziEmbeddingService, TodoziEmbeddingTool, TodoziEmbeddingCache } from '../todozi/emb.js';

const db = {
  tasks: [],
  memories: [],
  ideas: []
};

// Helper: create mock tasks
function mockTask(id, action, project, priority, status, tags = [], contextNotes = '') {
  const now = new Date().toISOString();
  return {
    id,
    userId: 'demo',
    action,
    time: '1h',
    priority,
    parentProject: project,
    status,
    assignee: null,
    tags,
    dependencies: [],
    contextNotes,
    progress: 0,
    embedding_vector: null,
    createdAt: now,
    updatedAt: now
  };
}

// Helper: create mock memories
function mockMemory(id, moment, meaning, reason, importance, term, memoryType, tags = []) {
  return {
    id,
    userId: 'demo',
    projectId: null,
    status: 'active',
    moment,
    meaning,
    reason,
    importance,
    term,
    memoryType,
    tags,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  };
}

// Helper: create mock ideas
function mockIdea(id, idea, share, importance, tags = []) {
  return {
    id,
    idea,
    projectId: null,
    status: 'active',
    share,
    importance,
    tags,
    context: null,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  };
}

async function main() {
  // 1) Configure embedding service
  const config = TodoziEmbeddingConfig.default();
  config.model_name = 'sentence-transformers/all-MiniLM-L6-v2';
  config.similarity_threshold = 0.6;  // lower threshold to get more demo results
  config.max_results = 10;
  config.enable_clustering = true;
  config.clustering_threshold = 0.8;

  // 2) Init embedding service and tool
  const service = await TodoziEmbeddingService.new(config);
  const tool = await TodoziEmbeddingTool.new(config);

  // 3) Seed some mock data
  const tasks = [
    mockTask('t1', 'Set up CI with GitHub Actions', 'platform', 'high', 'in_progress', ['ci', 'devops'], 'Automate builds and tests on push'),
    mockTask('t2', 'Refactor user authentication logic', 'webapp', 'medium', 'todo', ['security', 'refactor'], 'Move to OAuth 2.0 and improve session handling'),
    mockTask('t3', 'Add unit tests for payment module', 'billing', 'high', 'in_progress', ['testing', 'payments']),
    mockTask('t4', 'Optimize database queries for dashboard', 'analytics', 'high', 'todo', ['performance', 'database']),
    mockTask('t5', 'Create a code style guide', 'process', 'low', 'done', ['docs', 'style'])
  ];

  const memories = [
    mockMemory('m1', 'Customer reported slow login on Safari', 'Impact on conversion rate', 'User feedback', 'high', 'short', 'standard', ['safari', 'login']),
    mockMemory('m2', 'Billing outage on 2025-08-01', 'Lost revenue and trust', 'Postmortem review', 'critical', 'long', 'standard', ['billing', 'outage']),
    mockMemory('m3', 'Performance regression after upgrading Node', 'CPU usage spike under load', 'Monitoring alert', 'high', 'short', 'standard', ['node', 'performance']),
  ];

  const ideas = [
    mockIdea('i1', 'Introduce feature flags to decouple deploys from releases', 'team', 'high', ['feature-flags', 'devops']),
    mockIdea('i2', 'Adopt semantic versioning across all services', 'private', 'medium', ['semver', 'process']),
    mockIdea('i3', 'Build internal incident dashboard', 'public', 'high', ['incident', 'dashboard'])
  ];

  db.tasks.push(...tasks);
  db.memories.push(...memories);
  db.ideas.push(...ideas);

  // 4) Index content into the embedding service
  console.log('Indexing tasks, memories, and ideas...\n');

  for (const t of db.tasks) {
    const text = service.prepareTaskContent(t);
    const vec = await service.generateEmbedding(text);
    t.embedding_vector = vec;
    const cacheKey = `Task_${t.id}`;
    const entry = new TodoziEmbeddingCache(
      vec,
      'Task',
      t.id,
      text,
      t.tags,
      new Date(),
      3600
    );
    service.cache.set(cacheKey, entry);
  }

  for (const m of db.memories) {
    const text = service.prepareMemoryContent(m);
    const vec = await service.generateEmbedding(text);
    const cacheKey = `Memory_${m.id}`;
    const entry = new TodoziEmbeddingCache(
      vec,
      'Memory',
      m.id,
      text,
      m.tags,
      new Date(),
      3600
    );
    service.cache.set(cacheKey, entry);
  }

  for (const idea of db.ideas) {
    const text = service.prepareIdeaContent(idea);
    const vec = await service.generateEmbedding(text);
    const cacheKey = `Idea_${idea.id}`;
    const entry = new TodoziEmbeddingCache(
      vec,
      'Idea',
      idea.id,
      text,
      idea.tags,
      new Date(),
      3600
    );
    service.cache.set(cacheKey, entry);
  }

  console.log('Indexing complete.\n');

  // 5) Run semantic search for a user query
  const query = 'improve authentication and security';
  console.log(`Semantic search for: "${query}"`);
  const semResults = await service.semanticSearch(query, null, 5);
  console.log(`Found ${semResults.length} results:`);
  for (const r of semResults) {
    console.log(`- [${r.content_type}] (${(r.similarity_score * 100).toFixed(1)}%) ${r.text_content.split('\n')[0]}`);
  }
  console.log('');

  // 6) Find similar tasks to a given task
  const refTask = db.tasks[1]; // "Refactor user authentication logic"
  console.log(`Find tasks similar to: "${refTask.action}"`);
  const simTasks = await service.findSimilarTasks(refTask.action, 5);
  console.log(`Found ${simTasks.length} similar tasks:`);
  for (const s of simTasks) {
    console.log(`- (${(s.similarity_score * 100).toFixed(1)}%) ${s.text_content.split('\n')[0]}`);
  }
  console.log('');

  // 7) Cluster content
  console.log('Clustering content...');
  const clusters = await service.clusterContent();
  console.log(`Found ${clusters.length} clusters:`);
  clusters.forEach((c, idx) => {
    console.log(`Cluster ${idx + 1} (size=${c.cluster_size}, avg_sim=${(c.average_similarity * 100).toFixed(1)}%)`);
    c.content_items.forEach(ci => {
      console.log(`  - [${ci.content_type}] ${ci.text_content.split('\n')[0]}`);
    });
  });
  console.log('');

  // 8) Use TodoziEmbeddingTool as a tool (semantic search and clustering)
  console.log('Using TodoziEmbeddingTool for "performance regression" search:');
  const toolRes1 = await tool.execute({ action: 'semantic_search', content: 'performance regression', limit: 5 });
  if (toolRes1.success) {
    const results = JSON.parse(toolRes1.output);
    console.log(`Tool found ${results.length} results:`);
    results.forEach(r => {
      console.log(`- [${r.content_type}] (${(r.similarity_score * 100).toFixed(1)}%) ${r.text_content.split('\n')[0]}`);
    });
  } else {
    console.log('Tool error:', toolRes1.error);
  }
  console.log('');

  console.log('Using TodoziEmbeddingTool for clustering:');
  const toolRes2 = await tool.execute({ action: 'cluster' });
  if (toolRes2.success) {
    const clusters = JSON.parse(toolRes2.output);
    console.log(`Tool found ${clusters.length} clusters:`);
    clusters.forEach((c, idx) => {
      console.log(`Cluster ${idx + 1} (size=${c.cluster_size}, avg_sim=${(c.average_similarity * 100).toFixed(1)}%)`);
    });
  } else {
    console.log('Tool error:', toolRes2.error);
  }

  // 9) Basic stats
  console.log('\nEmbedding store stats:');
  const stats = await service.getStats();
  Object.entries(stats).forEach(([k, v]) => console.log(`- ${k}: ${typeof v === 'object' ? JSON.stringify(v) : v}`));
}

main().catch(err => {
  console.error('Example failed:', err);
  process.exit(1);
});

