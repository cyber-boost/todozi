# Todozi Embedding Service Documentation

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Performance Analysis](#performance-analysis)
7. [Security Considerations](#security-considerations)
8. [Testing Strategies](#testing-strategies)
9. [Deployment Instructions](#deployment-instructions)
10. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi Embedding Service is a comprehensive semantic search and embedding management system designed for task management and knowledge organization. It provides functionality for generating, storing, and querying embeddings for various content types including tasks, tags, memories, and ideas.

### Key Features

- Semantic search across content types
- Content clustering and similarity detection
- Embedding caching with TTL management
- Hierarchical clustering capabilities
- Drift tracking for content evolution
- Performance profiling and diagnostics
- Multi-model embedding support

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Todozi Embedding Service                     │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐ │
│  │   Main Service  │  │   Embedding     │  │   Configuration     │ │
│  │                 │  │   Model         │  │                     │ │
│  │  - Search       │  │                 │  │  - Config           │ │
│  │  - Clustering   │  │  - Generation   │  │  - Cache Settings   │ │
│  │  - Management   │  │  - Encoding     │  │                     │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────────┘ │
│                                                                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐ │
│  │     Caching     │  │   Utilities     │  │   Data Structures   │ │
│  │                 │  │                 │  │                     │ │
│  │  - LRU Cache    │  │  - Similarity   │  │  - Content Types    │ │
│  │  - TTL Mgmt     │  │  - Validation   │  │  - Results          │ │
│  │                 │  │  - Profiling    │  │  - Clusters         │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────────┐
                    │   Storage Layer     │
                    │ (File System/DB)    │
                    └─────────────────────┘
```

### Design Patterns Used

1. **Service Pattern**: Centralized embedding service with clear separation of concerns
2. **Factory Pattern**: Service initialization with configuration
3. **Strategy Pattern**: Multiple aggregation strategies for multi-query search
4. **Cache Pattern**: LRU caching with TTL management
5. **Builder Pattern**: Complex result object construction
6. **Singleton Pattern**: Configuration objects

## Core Components

### TodoziEmbeddingService

The main service class that orchestrates all embedding operations.

#### Constructor Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| config | TodoziEmbeddingConfig | Configuration settings |
| cache | Map | Cache storage |
| embeddingModel | EmbeddingModel | Primary embedding model |
| embeddingModels | Map | Additional embedding models |
| tagManager | Object | Tag management system |
| storage | Object | Storage interface |

#### Methods

##### `static async new(config)`
Creates and initializes a new embedding service instance.

**Parameters:**
- `config` (TodoziEmbeddingConfig): Configuration object

**Returns:** Promise<TodoziEmbeddingService>

##### `async initialize()`
Initializes the embedding service by loading the default model.

##### `async createProject(name, description)`
Creates a new project.

**Parameters:**
- `name` (string): Project name
- `description` (string): Project description

**Returns:** string (project name)

##### `async addTask(task)`
Adds a new task with embedding.

**Parameters:**
- `task` (Object): Task object with properties

**Returns:** string (task ID)

##### `async findSimilarTasks(taskDescription, limit)`
Finds tasks similar to the given description.

**Parameters:**
- `taskDescription` (string): Description to search for
- `limit` (number): Maximum results to return

**Returns:** Array<SimilarityResult>

##### `async semanticSearch(query, contentTypes, limit)`
Performs semantic search across content types.

**Parameters:**
- `query` (string): Search query
- `contentTypes` (Array<string>): Content types to search
- `limit` (number): Maximum results to return

**Returns:** Array<SimilarityResult>

##### `async clusterContent()`
Clusters content based on similarity.

**Returns:** Array<ClusteringResult>

##### `async getStats()`
Gets embedding statistics.

**Returns:** Object

##### `cosineSimilarity(a, b)`
Calculates cosine similarity between two vectors.

**Parameters:**
- `a` (Array<number>): First vector
- `b` (Array<number>): Second vector

**Returns:** number

### TodoziEmbeddingConfig

Configuration class for embedding service settings.

#### Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| model_name | string | "sentence-transformers/all-MiniLM-L6-v2" | Embedding model name |
| dimensions | number | 384 | Vector dimensions |
| similarity_threshold | number | 0.7 | Minimum similarity threshold |
| max_results | number | 50 | Maximum search results |
| cache_ttl_seconds | number | 86400 | Cache TTL in seconds |
| enable_clustering | boolean | true | Enable content clustering |
| clustering_threshold | number | 0.8 | Clustering similarity threshold |

### TodoziEmbeddingCache

Cache entry for embedding data.

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| vector | Array<number> | Embedding vector |
| content_type | string | Content type |
| content_id | string | Content identifier |
| text_content | string | Original text content |
| tags | Array<string> | Associated tags |
| created_at | Date | Creation timestamp |
| ttl_seconds | number | Time-to-live in seconds |

### SimilarityResult

Result object for similarity searches.

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| content_id | string | Content identifier |
| content_type | string | Content type |
| similarity_score | number | Similarity score |
| text_content | string | Text content |
| tags | Array<string> | Associated tags |
| metadata | Object | Additional metadata |

### ClusteringResult

Result object for clustering operations.

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| cluster_id | string | Cluster identifier |
| content_items | Array<SimilarityResult> | Clustered items |
| cluster_center | Array<number> | Cluster center vector |
| cluster_size | number | Number of items in cluster |
| average_similarity | number | Average similarity within cluster |

## API Reference

### Enums

#### TodoziContentType
```javascript
const TodoziContentType = {
  Task: 'Task',
  Tag: 'Tag',
  Memory: 'Memory',
  Idea: 'Idea',
  Chunk: 'Chunk',
  Feel: 'Feel',
  Train: 'Train',
  Error: 'Error',
  Summary: 'Summary',
  Reminder: 'Reminder',
  Tdz: 'Tdz'
};
```

#### Priority
```javascript
const Priority = {
  Low: 'Low',
  Medium: 'Medium',
  High: 'High',
  Critical: 'Critical'
};
```

#### Status
```javascript
const Status = {
  Todo: 'Todo',
  InProgress: 'InProgress',
  Done: 'Done',
  Blocked: 'Blocked'
};
```

### Classes

#### LRUEmbeddingCache
LRU (Least Recently Used) cache implementation for embeddings.

##### Methods
- `get(key)`: Retrieves cached value
- `insert(key, value)`: Inserts value into cache
- `estimateSize(entry)`: Estimates memory size of entry
- `len()`: Returns cache size
- `isEmpty()`: Checks if cache is empty

#### EmbeddingModel
Placeholder for actual embedding model implementation.

##### Methods
- `static async load(modelName, device)`: Loads model
- `async encode(texts)`: Encodes texts to embeddings

#### TodoziEmbeddingTool
Tool interface for embedding operations.

##### Methods
- `static async new(config)`: Creates new tool instance
- `definition()`: Returns tool definition
- `async execute(kwargs)`: Executes tool action

## Usage Examples

### Basic Setup and Initialization

```javascript
const { TodoziEmbeddingService, TodoziEmbeddingConfig } = require('./todozi-embedding');

// Create configuration
const config = new TodoziEmbeddingConfig();
config.similarity_threshold = 0.8;
config.max_results = 100;

// Initialize service
const service = await TodoziEmbeddingService.new(config);
```

### Adding and Searching Tasks

```javascript
// Add a task
const task = {
  id: 'task-001',
  action: 'Complete documentation',
  context_notes: 'Write comprehensive documentation for embedding service',
  priority: 'High',
  status: 'InProgress',
  tags: ['documentation', 'development'],
  assignee: 'john.doe'
};

const taskId = await service.addTask(task);

// Find similar tasks
const similarTasks = await service.findSimilarTasks(
  'Write technical documentation',
  10
);

console.log('Similar tasks:', similarTasks);
```

### Semantic Search

```javascript
// Search across content types
const results = await service.semanticSearch(
  'machine learning concepts',
  ['Task', 'Memory', 'Idea'],
  20
);

results.forEach(result => {
  console.log(`${result.content_type}: ${result.text_content}`);
  console.log(`Similarity: ${result.similarity_score}`);
});
```

### Content Clustering

```javascript
// Enable clustering in config
const config = new TodoziEmbeddingConfig();
config.enable_clustering = true;
config.clustering_threshold = 0.75;

const service = await TodoziEmbeddingService.new(config);

// Perform clustering
const clusters = await service.clusterContent();

clusters.forEach(cluster => {
  console.log(`Cluster ${cluster.cluster_id}: ${cluster.cluster_size} items`);
  console.log(`Average similarity: ${cluster.average_similarity}`);
  
  cluster.content_items.forEach(item => {
    console.log(`  - ${item.text_content.substring(0, 50)}...`);
  });
});
```

### Hybrid Search

```javascript
// Combine semantic and keyword search
const results = await service.hybridSearch(
  'project planning',
  ['schedule', 'timeline', 'milestone'],
  ['Task', 'Memory'],
  0.7, // 70% semantic weight
  15
);

results.forEach(result => {
  console.log(`Score: ${result.similarity_score}`);
  console.log(`Semantic: ${result.metadata.semantic_score}`);
  console.log(`Keyword: ${result.metadata.keyword_score}`);
});
```

### Multi-Query Search

```javascript
// Search with multiple queries and aggregation
const queries = [
  'software development',
  'coding practices',
  'programming techniques'
];

const results = await service.multiQuerySearch(
  queries,
  'Average', // Aggregation strategy
  ['Task', 'Idea'],
  10
);

// Weighted aggregation example
const weightedResults = await service.multiQuerySearch(
  queries,
  { type: 'Weighted', weights: [0.5, 0.3, 0.2] },
  ['Task', 'Idea'],
  10
);
```

### Hierarchical Clustering

```javascript
// Perform hierarchical clustering
const hierarchicalClusters = await service.hierarchicalClustering(
  ['Task', 'Idea'],
  3 // Maximum depth
);

function printCluster(cluster, indent = '') {
  console.log(`${indent}Cluster ${cluster.cluster_id} (Level ${cluster.level})`);
  console.log(`${indent}  Items: ${cluster.cluster_size}`);
  console.log(`${indent}  Similarity: ${cluster.average_similarity}`);
  
  cluster.children.forEach(child => {
    printCluster(child, indent + '  ');
  });
}

hierarchicalClusters.forEach(cluster => printCluster(cluster));
```

### Drift Tracking

```javascript
// Track content evolution
const driftReport = await service.trackEmbeddingDrift(
  'content-001',
  'Updated content text here...'
);

console.log(`Drift percentage: ${driftReport.drift_percentage}%`);
console.log(`Significant drift: ${driftReport.significant_drift}`);

if (driftReport.significant_drift) {
  console.log('Content has significantly changed!');
}
```

### Performance Profiling

```javascript
// Profile search performance
const performanceMetrics = await service.profileSearchPerformance(
  'machine learning',
  20 // iterations
);

console.log(`Average time: ${performanceMetrics.avg_time_ms}ms`);
console.log(`Min time: ${performanceMetrics.min_time_ms}ms`);
console.log(`Max time: ${performanceMetrics.max_time_ms}ms`);
console.log(`Std deviation: ${performanceMetrics.std_dev_ms}ms`);
```

### Using the Tool Interface

```javascript
const { TodoziEmbeddingTool } = require('./todozi-embedding');

// Create tool
const tool = await TodoziEmbeddingTool.new();

// Get tool definition
const definition = tool.definition();
console.log('Tool name:', definition.name);
console.log('Description:', definition.description);

// Execute tool actions
const result = await tool.execute({
  action: 'semantic_search',
  content: 'project management',
  limit: 5
});

if (result.success) {
  console.log('Search results:', result.output);
} else {
  console.error('Error:', result.error);
}
```

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Semantic Search | O(n) | Linear scan of cache |
| Clustering | O(n²) | Pairwise similarity calculation |
| Cosine Similarity | O(d) | d = vector dimensions |
| Cache Lookup | O(1) | Hash map lookup |
| Cache Insertion | O(1) | Amortized constant time |

### Memory Usage

The service uses several caching mechanisms:

1. **Main Cache**: Stores embedding vectors and metadata
2. **LRU Cache**: Memory-limited cache with eviction policy
3. **Query Results Cache**: Temporary storage for recent searches

Memory usage can be controlled through:
- `max_memory_mb` parameter in LRUCache
- `cache_ttl_seconds` for automatic cleanup
- Manual cache cleanup with `cleanupExpired()`

### Optimization Strategies

1. **Batch Processing**: Use `generateEmbeddingsBatch()` for multiple texts
2. **Caching**: Leverage `getOrGenerateEmbedding()` to avoid recomputation
3. **Filtering**: Apply content type filters to reduce search space
4. **Indexing**: Consider external indexing for large datasets

## Security Considerations

### Data Privacy

1. **Content Storage**: Embeddings contain semantic information that could potentially be reverse-engineered
2. **Cache Management**: Sensitive content should have shorter TTL values
3. **File Storage**: The mega log file contains detailed task information

### Access Control

```javascript
// Example: Add access control to service methods
class SecureTodoziEmbeddingService extends TodoziEmbeddingService {
  constructor(config, userContext) {
    super(config);
    this.userContext = userContext;
  }
  
  async addTask(task) {
    // Check user permissions
    if (!this.userContext.canCreateTask(task.parent_project)) {
      throw new Error('Insufficient permissions');
    }
    return super.addTask(task);
  }
  
  async semanticSearch(query, contentTypes, limit) {
    // Filter results based on user permissions
    const results = await super.semanticSearch(query, contentTypes, limit);
    return results.filter(result => 
      this.userContext.canViewContent(result.content_id)
    );
  }
}
```

### Input Validation

```javascript
// Example: Input sanitization
async validateAndAddTask(task) {
  // Validate input
  if (!task.action || task.action.length > 1000) {
    throw new Error('Invalid task action');
  }
  
  if (task.tags && task.tags.length > 50) {
    throw new Error('Too many tags');
  }
  
  // Sanitize content
  task.action = this.sanitizeText(task.action);
  if (task.context_notes) {
    task.context_notes = this.sanitizeText(task.context_notes);
  }
  
  return await this.addTask(task);
}
```

## Testing Strategies

### Unit Tests

```javascript
const { TodoziEmbeddingService, TodoziEmbeddingConfig } = require('./todozi-embedding');

describe('TodoziEmbeddingService', () => {
  let service;
  
  beforeEach(async () => {
    const config = new TodoziEmbeddingConfig();
    service = await TodoziEmbeddingService.new(config);
  });
  
  test('cosine similarity calculation', () => {
    const a = [1, 0, 0];
    const b = [0, 1, 0];
    const similarity = service.cosineSimilarity(a, b);
    expect(similarity).toBeCloseTo(0);
  });
  
  test('task embedding generation', async () => {
    const task = {
      id: 'test-001',
      action: 'Test task',
      priority: 'Medium',
      status: 'Todo'
    };
    
    const taskId = await service.addTask(task);
    expect(taskId).toBe('test-001');
  });
  
  test('semantic search functionality', async () => {
    // Add test data
    await service.addTask({
      id: 'task-001',
      action: 'Write documentation',
      context_notes: 'Technical writing'
    });
    
    const results = await service.findSimilarTasks('documentation writing');
    expect(results.length).toBeGreaterThan(0);
  });
});
```

### Integration Tests

```javascript
describe('Embedding Service Integration', () => {
  test('full workflow', async () => {
    const service = await TodoziEmbeddingService.new();
    
    // Add multiple tasks
    const tasks = [
      { id: 'task-1', action: 'Machine learning research' },
      { id: 'task-2', action: 'Data analysis and visualization' },
      { id: 'task-3', action: 'Software development' }
    ];
    
    for (const task of tasks) {
      await service.addTask(task);
    }
    
    // Perform semantic search
    const results = await service.semanticSearch('AI and data science');
    expect(results).toHaveLength(3);
    
    // Check clustering
    const clusters = await service.clusterContent();
    expect(clusters).toBeDefined();
  });
});
```

### Performance Tests

```javascript
describe('Performance Tests', () => {
  test('search performance under load', async () => {
    const service = await TodoziEmbeddingService.new();
    
    // Add large dataset
    for (let i = 0; i < 1000; i++) {
      await service.addTask({
        id: `task-${i}`,
        action: `Task ${i} description`
      });
    }
    
    // Measure search time
    const start = Date.now();
    const results = await service.semanticSearch('test query', null, 10);
    const end = Date.now();
    
    const searchTime = end - start;
    expect(searchTime).toBeLessThan(1000); // Should complete in under 1 second
    expect(results).toHaveLength(10);
  });
});
```

## Deployment Instructions

### Prerequisites

1. Node.js v16+ installed
2. npm or yarn package manager
3. Sufficient disk space for cache and logs
4. Memory allocation based on expected dataset size

### Installation

```bash
# Clone repository
git clone <repository-url>
cd todozi-embedding

# Install dependencies
npm install

# For production deployment
npm install --production
```

### Configuration

Create a configuration file `config.json`:

```json
{
  "embedding": {
    "model_name": "sentence-transformers/all-MiniLM-L6-v2",
    "dimensions": 384,
    "similarity_threshold": 0.7,
    "max_results": 50,
    "cache_ttl_seconds": 86400,
    "enable_clustering": true,
    "clustering_threshold": 0.8
  },
  "cache": {
    "max_memory_mb": 512,
    "cleanup_interval_seconds": 3600
  },
  "storage": {
    "embed_directory": "./embed",
    "log_file": "embedding_mega_log.jsonl"
  }
}
```

### Environment Variables

```bash
# Set environment variables
export TODOZI_EMBED_MODEL="sentence-transformers/all-MiniLM-L6-v2"
export TODOZI_EMBED_CACHE_TTL=86400
export TODOZI_EMBED_MAX_RESULTS=100
export TODOZI_EMBED_DIR="./data/embed"
```

### Docker Deployment

Create `Dockerfile`:

```dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 3000

CMD ["node", "server.js"]
```

Build and run:

```bash
docker build -t todozi-embedding .
docker run -p 3000:3000 -v ./data:/app/data todozi-embedding
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: todozi-embedding
spec:
  replicas: 3
  selector:
    matchLabels:
      app: todozi-embedding
  template:
    metadata:
      labels:
        app: todozi-embedding
    spec:
      containers:
      - name: todozi-embedding
        image: todozi-embedding:latest
        ports:
        - containerPort: 3000
        env:
        - name: TODOZI_EMBED_CACHE_TTL
          value: "86400"
        volumeMounts:
        - name: data
          mountPath: /app/data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: todozi-embedding-pvc
```

## Troubleshooting Guide

### Common Issues

#### 1. Model Loading Failures

**Symptoms:** Service fails to initialize with "Model not found" error

**Solutions:**
```javascript
// Check model availability
try {
  const modelName = await EmbeddingModel.getDefaultModel();
  console.log(`Default model: ${modelName}`);
  
  // Try loading with explicit device
  const model = await EmbeddingModel.load(modelName, 'cpu');
} catch (error) {
  console.error('Model loading failed:', error.message);
  
  // Fallback to basic initialization
  const service = new TodoziEmbeddingService(config);
  // Continue without embedding model for basic operations
}
```

#### 2. Memory Issues

**Symptoms:** High memory usage, slow performance, OOM errors

**Solutions:**
```javascript
// Monitor cache size
const cacheSize = service.cache.size;
console.log(`Current cache size: ${cacheSize}`);

// Implement periodic cleanup
setInterval(async () => {
  const cleaned = await service.cleanupExpired();
  console.log(`Cleaned ${cleaned} expired entries`);
}, 3600000); // Every hour

// Reduce cache size
const config = new TodoziEmbeddingConfig();
config.cache_ttl_seconds = 3600; // 1 hour instead of 24
config.max_results = 25; // Reduce result set size
```

#### 3. Search Performance Issues

**Symptoms:** Slow search responses, high CPU usage

**Solutions:**
```javascript
// Profile slow queries
const metrics = await service.profileSearchPerformance('slow query');
console.log(`Average search time: ${metrics.avg_time_ms}ms`);

// Optimize by filtering content types
const results = await service.semanticSearch(
  query,
  ['Task'], // Only search tasks instead of all types
  limit
);

// Implement result caching for repeated queries
const queryCache = new Map();
const cachedResults = queryCache.get(query);
if (cachedResults) {
  return cachedResults;
}
```

#### 4. Cache Invalidation Issues

**Symptoms:** Stale results, outdated embeddings

**Solutions:**
```javascript
// Force refresh embeddings
const embedding = await service.getOrGenerateEmbedding(
  contentId,
  text,
  contentType,
  true // refreshIfStale = true
);

// Manual cache invalidation
service.cache.delete(`task_${taskId}`);

// Implement cache versioning
const cacheVersion = 'v1.0';
const cacheKey = `${contentType}_${contentId}_${cacheVersion}`;
```

#### 5. File System Errors

**Symptoms:** Cannot write to log files, directory not found

**Solutions:**
```javascript
// Check directory permissions
try {
  await fs.access(embedDir, fs.constants.W_OK);
} catch (error) {
  console.error('No write permission for embed directory');
  // Create directory with proper permissions
  await fs.mkdir(embedDir, { recursive: true, mode: 0o755 });
}

// Handle file write errors gracefully
try {
  await service.logToMegaFile(task);
} catch (error) {
  console.error('Failed to write to mega file:', error.message);
  // Continue without logging or use alternative storage
}
```

### Debugging Tools

```javascript
// Enable debug logging
const debug = require('debug')('todozi:embedding');

class TodoziEmbeddingService {
  async addTask(task) {
    debug(`Adding task: ${task.id}`);
    debug(`Task content: ${this.prepareTaskContent(task)}`);
    
    const start = Date.now();
    const result = await super.addTask(task);
    const duration = Date.now() - start;
    
    debug(`Task added in ${duration}ms`);
    return result;
  }
}

// Monitor cache hit rate
class MonitoringService extends TodoziEmbeddingService {
  constructor() {
    super();
    this.cacheHits = 0;
    this.cacheMisses = 0;
  }
  
  async getOrGenerateEmbedding(contentId, text, contentType) {
    const cacheKey = `${contentType}_${contentId}`;
    if (this.cache.has(cacheKey)) {
      this.cacheHits++;
    } else {
      this.cacheMisses++;
    }
    
    return super.getOrGenerateEmbedding(contentId, text, contentType);
  }
  
  getCacheHitRate() {
    const total = this.cacheHits + this.cacheMisses;
    return total > 0 ? this.cacheHits / total : 0;
  }
}
```

### Recovery Procedures

1. **Cache Corruption**: Clear cache and restart service
2. **Model Issues**: Reinstall transformer dependencies
3. **Storage Problems**: Check disk space and permissions
4. **Performance Degradation**: Review and optimize configuration

This comprehensive documentation provides a complete overview of the Todozi Embedding Service, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.