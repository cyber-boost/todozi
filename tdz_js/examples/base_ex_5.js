/*

 * Example 5: Register and use an embedding-based tool via the ToolRegistry
 *
 * This example demonstrates:
 * - Creating a tool that wraps a semantic embedding service (with a pluggable model)
 * - Registering the tool with the ToolRegistry
 * - Executing tool calls via registry.executeTool
 *
 * How to run:
 *   1) Save this file as example5.js
 *   2) Create a package.json in the same folder and add the file below
 *   3) npm install (uuid is the only external dep)
 *   4) node example5.js <action> [options...]
 *
 * Examples:
 *   node example5.js find_similar --content "review pull requests" --limit 5
 *   node example5.js semantic_search --content "code quality" --limit 10
 *   node example5.js cluster
 *   node example5.js stats
 *   node example5.js hybrid_search --content "test data" --keywords "test, quality, coverage"
 *
 * Notes:
 * - This example uses a simple in-memory dataset (no filesystem or external ML needed)
 * - The EmbeddingModel is pluggable; a mock model is used so it runs out of the box
 */
import { v4 as uuidv4 } from 'uuid';

// ---------------------------------------------
// base.js re-exports (inline to keep this file self-contained)
// ---------------------------------------------
class ToolParameter {
  constructor(name, type_, description, required, default = null) {
    this.name = name;
    this.type_ = type_;
    this.description = description;
    this.required = required;
    this.default = default;
  }
  static new(name, type_, description, required, default = null) {
    return new ToolParameter(name, type_, description, required, default);
  }
}

const ResourceLock = {
  FilesystemWrite: 'FilesystemWrite',
  FilesystemRead: 'FilesystemRead',
  Git: 'Git',
  Memory: 'Memory',
  Shell: 'Shell',
  Network: 'Network',
};
ResourceLock.asStr = function(lock) {
  switch (lock) {
    case ResourceLock.FilesystemWrite: return 'filesystem_write';
    case ResourceLock.FilesystemRead: return 'filesystem_read';
    case ResourceLock.Network: return 'network';
    case ResourceLock.Git: return 'git';
    case ResourceLock.Memory: return 'memory';
    case ResourceLock.Shell: return 'shell';
    default: throw new Error('Unknown ResourceLock');
  }
};

class ToolDefinition {
  constructor(name, description, parameters, category, resource_locks) {
    this.name = name;
    this.description = description;
    this.parameters = parameters;
    this.category = category;
    this.resource_locks = resource_locks;
  }
  static new(name, description, parameters, category, resource_locks) {
    return new ToolDefinition(name, description, parameters, category, resource_locks);
  }
  toOllamaFormat() {
    const properties = {};
    const required = [];
    for (const param of this.parameters) {
      const propValue = {
        type: param.type_,
        description: param.description
      };
      if (param.default !== null) {
        propValue.default = param.default;
      }
      if (param.required) {
        required.push(param.name);
      }
      properties[param.name] = propValue;
    }
    return {
      type: 'function',
      function: {
        name: this.name,
        description: this.description,
        parameters: {
          type: 'object',
          properties: properties,
          required: required
        }
      }
    };
  }
}

class ToolResult {
  constructor(success, output, error, execution_time_ms, metadata = null, recovery_context = null) {
    this.success = success;
    this.output = output;
    this.error = error;
    this.execution_time_ms = execution_time_ms;
    this.metadata = metadata;
    this.recovery_context = recovery_context;
  }
  static new(success, output, error, execution_time_ms, metadata = null, recovery_context = null) {
    return new ToolResult(success, output, error, execution_time_ms, metadata, recovery_context);
  }
  static success(output, execution_time_ms) {
    return new ToolResult(true, output, null, execution_time_ms);
  }
  static error(error, execution_time_ms) {
    return new ToolResult(false, '', error, execution_time_ms);
  }
  toString() {
    if (this.success) return this.output;
    return `Error: ${this.error || 'Unknown error'}`;
  }
}

const ErrorType = {
  ValidationError: 'ValidationError',
  PermissionError: 'PermissionError',
  FileNotFound: 'FileNotFound',
  TimeoutError: 'TimeoutError',
  ResourceError: 'ResourceError',
  NetworkError: 'NetworkError',
  SecurityError: 'SecurityError',
  InternalError: 'InternalError',
};
ErrorType.toString = function(errorType) {
  switch (errorType) {
    case ErrorType.ValidationError: return 'validation_error';
    case ErrorType.PermissionError: return 'permission_error';
    case ErrorType.FileNotFound: return 'file_not_found';
    case ErrorType.TimeoutError: return 'timeout_error';
    case ErrorType.ResourceError: return 'resource_error';
    case ErrorType.NetworkError: return 'network_error';
    case ErrorType.SecurityError: return 'security_error';
    case ErrorType.InternalError: return 'internal_error';
    default: throw new Error('Unknown ErrorType');
  }
};

class ToolError extends Error {
  constructor(message, error_type, details = {}) {
    super(message);
    this.name = 'ToolError';
    this.message = message;
    this.error_type = error_type;
    this.details = details;
  }
  static new(message, error_type, details = {}) {
    return new ToolError(message, error_type, details);
  }
  toString() {
    return this.message;
  }
}

class ErrorHandler {
  static handleError(error, context) {
    let error_msg;
    const metadata = { context };
    const errorString = error.toString();
    if (errorString.includes('ToolError')) {
      error_msg = errorString;
      metadata.error_type = ErrorType.toString(ErrorType.InternalError);
    } else if (errorString.includes('I/O')) {
      error_msg = `I/O error: ${error}`;
      metadata.error_type = ErrorType.toString(ErrorType.ResourceError);
    } else {
      error_msg = `Unexpected error: ${error}`;
      metadata.error_type = ErrorType.toString(ErrorType.InternalError);
      metadata.exception_type = error.constructor.name;
    }
    console.error(`Tool error in ${context}: ${error_msg}`);
    return ToolResult.new(false, '', error_msg, 0, metadata);
  }
  static validateRequiredParams(kwargs, required_params) {
    const missingParams = required_params.filter(param => !(param in kwargs));
    if (missingParams.length > 0) {
      const error_msg = `Missing required parameters: ${missingParams.join(', ')}`;
      const metadata = {
        error_type: ErrorType.toString(ErrorType.ValidationError),
        missing_params: missingParams
      };
      return ToolResult.new(false, '', error_msg, 0, metadata);
    }
    return null;
  }
  static validateStringParam(value, param_name, min_length, max_length, pattern = null) {
    if (typeof value !== 'string') {
      const error_msg = `Parameter '${param_name}' must be a string, got ${typeof value}`;
      const metadata = {
        error_type: ErrorType.toString(ErrorType.ValidationError),
        param_name: param_name,
        actual_type: typeof value
      };
      return ToolResult.new(false, '', error_msg, 0, metadata);
    }
    const stringValue = value;
    if (stringValue.length < min_length) {
      const error_msg = `Parameter '${param_name}' must be at least ${min_length} characters, got ${stringValue.length}`;
      const metadata = {
        error_type: ErrorType.toString(ErrorType.ValidationError),
        param_name: param_name,
        actual_length: stringValue.length,
        min_length: min_length
      };
      return ToolResult.new(false, '', error_msg, 0, metadata);
    }
    if (stringValue.length > max_length) {
      const error_msg = `Parameter '${param_name}' must be at most ${max_length} characters, got ${stringValue.length}`;
      const metadata = {
        error_type: ErrorType.toString(ErrorType.ValidationError),
        param_name: param_name,
        actual_length: stringValue.length,
        max_length: max_length
      };
      return ToolResult.new(false, '', error_msg, 0, metadata);
    }
    if (pattern) {
      const regex = new RegExp(pattern);
      if (!regex.test(stringValue)) {
        const error_msg = `Parameter '${param_name}' does not match required pattern`;
        const metadata = {
          error_type: ErrorType.toString(ErrorType.ValidationError),
          param_name: param_name,
          pattern: pattern
        };
        return ToolResult.new(false, '', error_msg, 0, metadata);
      }
    }
    return null;
  }
  static createSuccessResult(output, execution_time_ms, metadata = null) {
    return ToolResult.new(true, output, null, execution_time_ms, metadata);
  }
  static createErrorResult(error_msg, execution_time_ms, error_type, metadata = null) {
    const result_metadata = {
      error_type: ErrorType.toString(error_type),
      ...metadata
    };
    return ToolResult.new(false, '', error_msg, execution_time_ms, result_metadata);
  }
}

class Tool {
  async execute(kwargs) {
    throw new Error('execute must be implemented by subclass');
  }
  definition() {
    throw new Error('definition must be implemented by subclass');
  }
  name() {
    return this.definition().name;
  }
  validateParameters(kwargs) {
    const definition = this.definition();
    for (const param of definition.parameters) {
      if (param.required && !(param.name in kwargs)) {
        return false;
      }
    }
    for (const [param_name, value] of Object.entries(kwargs)) {
      const paramDef = definition.parameters.find(p => p.name === param_name);
      if (paramDef) {
        if (param_name === 'value' && paramDef.description.includes('JSON-serializable')) {
          continue;
        } else if (paramDef.type_ === 'string' && typeof value !== 'string') {
          return false;
        } else if (paramDef.type_ === 'number' && typeof value !== 'number') {
          return false;
        } else if (paramDef.type_ === 'boolean' && typeof value !== 'boolean') {
          return false;
        } else if (paramDef.type_ === 'array' && !Array.isArray(value)) {
          return false;
        } else if (paramDef.type_ === 'object' && (typeof value !== 'object' || Array.isArray(value))) {
          return false;
        }
      }
    }
    return true;
  }
}

class ToolRegistry {
  constructor() {
    this.tools = new Map();
  }
  register(tool) {
    const toolName = tool.name();
    this.tools.set(toolName, tool);
  }
  registerCoreTools() {
    console.log('Core tools registration structure prepared - individual tool implementations would be registered here');
  }
  getTool(name) {
    return this.tools.get(name);
  }
  getAllTools() {
    return Array.from(this.tools.values());
  }
  getToolDefinitions() {
    return Array.from(this.tools.values()).map(tool => tool.definition().toOllamaFormat());
  }
  async executeTool(tool_name, kwargs) {
    const tool = this.getTool(tool_name);
    if (!tool) {
      return ToolResult.new(false, '', `Tool '${tool_name}' not found`, 0);
    }
    if (!tool.validateParameters(kwargs)) {
      return ToolResult.new(false, '', `Invalid parameters for tool '${tool_name}'`, 0);
    }
    return await tool.execute(kwargs);
  }
  toolCount() {
    return this.tools.size;
  }
  hasTool(name) {
    return this.tools.has(name);
  }
  unregister(name) {
    return this.tools.delete(name);
  }
  clear() {
    this.tools.clear();
  }
}

function createToolParameter(name, type_, description, required) {
  return ToolParameter.new(name, type_, description, required);
}
function createToolParameterWithDefault(name, type_, description, required, defaultValue) {
  return ToolParameter.new(name, type_, description, required, defaultValue);
}
function createToolDefinition(name, description, category, parameters) {
  return ToolDefinition.new(name, description, parameters, category, []);
}
function createToolDefinitionWithLocks(name, description, category, parameters, resource_locks) {
  return ToolDefinition.new(name, description, parameters, category, resource_locks);
}

// ---------------------------------------------
// Simple, self-contained embedding service (mock model)
// ---------------------------------------------
class MockEmbeddingModel {
  constructor(dimensions = 128) {
    this.dimensions = dimensions;
    console.log(`[MockEmbeddingModel] Initialized with ${dimensions} dimensions`);
  }
  async encode(texts) {
    // Deterministic mock: hash text to seed a pseudo-random vector
    return texts.map(text => {
      const vec = new Array(this.dimensions).fill(0);
      let seed = 0;
      for (let i = 0; i < text.length; i++) {
        seed = (seed * 31 + text.charCodeAt(i)) >>> 0;
      }
      // LCG to generate deterministic values
      let s = seed || 1;
      const a = 1664525;
      const c = 1013904223;
      for (let i = 0; i < this.dimensions; i++) {
        s = (a * s + c) % 4294967296;
        vec[i] = (s / 4294967296) * 2 - 1; // range [-1, 1]
      }
      return vec;
    });
  }
}

class InMemoryEmbeddingCache {
  constructor() {
    this.map = new Map();
  }
  get(key) { return this.map.get(key); }
  set(key, value) { this.map.set(key, value); }
  has(key) { return this.map.has(key); }
  delete(key) { this.map.delete(key); }
  clear() { this.map.clear(); }
  get size() { return this.map.size; }
  values() { return this.map.values(); }
  entries() { return this.map.entries(); }
}

class SimpleEmbeddingService {
  constructor({ dimensions = 128, similarityThreshold = 0.6, maxResults = 20 } = {}) {
    this.dimensions = dimensions;
    this.similarityThreshold = similarityThreshold;
    this.maxResults = maxResults;
    this.model = new MockEmbeddingModel(dimensions);
    this.cache = new InMemoryEmbeddingCache();
    this.index = []; // { id, type, text, vector, tags }
  }

  async addItem(id, type, text, tags = []) {
    const vectors = await this.model.encode([text]);
    const vector = vectors[0];
    this.cache.set(id, vector);
    this.index.push({ id, type, text, vector, tags });
    return id;
  }

  async addItems(items) {
    // items: [{ id, type, text, tags }]
    const texts = items.map(i => i.text);
    const vectors = await this.model.encode(texts);
    items.forEach((item, idx) => {
      const vector = vectors[idx];
      this.cache.set(item.id, vector);
      this.index.push({ id: item.id, type: item.type, text: item.text, vector, tags: item.tags || [] });
    });
  }

  async semanticSearch(query, contentTypes = null, limit = null) {
    const actualLimit = limit || this.maxResults;
    const [queryVector] = await this.model.encode([query]);
    const results = [];
    for (const item of this.index) {
      if (contentTypes && !contentTypes.includes(item.type)) continue;
      const sim = this.cosineSimilarity(queryVector, item.vector);
      if (sim >= this.similarityThreshold) {
        results.push({
          id: item.id,
          type: item.type,
          text: item.text,
          tags: item.tags,
          similarity: sim
        });
      }
    }
    results.sort((a, b) => b.similarity - a.similarity);
    return results.slice(0, actualLimit);
  }

  async findSimilarByText(text, limit = null) {
    const [vec] = await this.model.encode([text]);
    const results = [];
    for (const item of this.index) {
      const sim = this.cosineSimilarity(vec, item.vector);
      if (sim >= this.similarityThreshold) {
        results.push({
          id: item.id,
          type: item.type,
          text: item.text,
          tags: item.tags,
          similarity: sim
        });
      }
    }
    results.sort((a, b) => b.similarity - a.similarity);
    return results.slice(0, (limit || this.maxResults));
  }

  async clusterContent() {
    // Simple clustering: group items with similarity >= threshold among all pairs
    const threshold = Math.max(0.7, this.similarityThreshold);
    const clusters = [];
    const processed = new Set();
    let clusterId = 1;

    for (let i = 0; i < this.index.length; i++) {
      const a = this.index[i];
      if (processed.has(a.id)) continue;

      const group = [a];
      processed.add(a.id);

      for (let j = i + 1; j < this.index.length; j++) {
        const b = this.index[j];
        if (processed.has(b.id)) continue;
        const sim = this.cosineSimilarity(a.vector, b.vector);
        if (sim >= threshold) {
          group.push(b);
          processed.add(b.id);
        }
      }

      if (group.length > 1) {
        const avgSim = this.averageSimilarity(group);
        clusters.push({
          clusterId: `c${clusterId++}`,
          size: group.length,
          averageSimilarity: avgSim,
          items: group.map(g => ({ id: g.id, type: g.type, text: g.text }))
        });
      }
    }
    return clusters;
  }

  async stats() {
    const typeCount = {};
    for (const item of this.index) {
      typeCount[item.type] = (typeCount[item.type] || 0) + 1;
    }
    return {
      total: this.index.length,
      typeCounts: typeCount,
      dimensions: this.dimensions,
      similarityThreshold: this.similarityThreshold
    };
  }

  async hybridSearch(query, keywords = [], contentTypes = null, semanticWeight = 0.7, limit = null) {
    const actualLimit = limit || this.maxResults;
    const [queryVector] = await this.model.encode([query]);
    const results = [];

    for (const item of this.index) {
      if (contentTypes && !contentTypes.includes(item.type)) continue;

      const semanticScore = this.cosineSimilarity(queryVector, item.vector);

      let keywordScore = 0.0;
      const textLower = item.text.toLowerCase();
      if (textLower.includes(query.toLowerCase())) keywordScore += 0.5;
      for (const kw of keywords) {
        if (kw && textLower.includes(kw.toLowerCase())) keywordScore += 0.3;
      }
      keywordScore = Math.min(keywordScore, 1.0);

      const combined = semanticScore * semanticWeight + keywordScore * (1 - semanticWeight);
      if (combined >= this.similarityThreshold) {
        results.push({
          id: item.id,
          type: item.type,
          text: item.text,
          tags: item.tags,
          similarity: combined,
          meta: { semanticScore, keywordScore }
        });
      }
    }
    results.sort((a, b) => b.similarity - a.similarity);
    return results.slice(0, actualLimit);
  }

  // ---------------- Utils ----------------
  cosineSimilarity(a, b) {
    if (a.length !== b.length) return 0.0;
    let dot = 0, na = 0, nb = 0;
    for (let i = 0; i < a.length; i++) {
      dot += a[i] * b[i];
      na += a[i] * a[i];
      nb += b[i] * b[i];
    }
    const denom = Math.sqrt(na) * Math.sqrt(nb);
    return denom === 0 ? 0 : dot / denom;
  }
  averageSimilarity(group) {
    let count = 0;
    let sum = 0;
    for (let i = 0; i < group.length; i++) {
      for (let j = i + 1; j < group.length; j++) {
        sum += this.cosineSimilarity(group[i].vector, group[j].vector);
        count++;
      }
    }
    return count === 0 ? 0 : sum / count;
  }
}

// ---------------------------------------------
// Tool implementation that wraps the embedding service
// ---------------------------------------------
class TodoziEmbeddingsTool extends Tool {
  constructor(service) {
    super();
    this.service = service;
  }

  definition() {
    return createToolDefinitionWithLocks(
      'todozi_embeddings',
      'Semantic search, similarity, clustering, and stats over indexed content using embeddings',
      [
        createToolParameter('action', 'string', 'Action to perform: find_similar|semantic_search|cluster|stats|hybrid_search', true),
        createToolParameter('content', 'string', 'Query text or content for search/similarity', false),
        createToolParameter('limit', 'number', 'Max number of results (default 10)', false, 10),
        createToolParameter('content_types', 'string', 'Comma-separated content types (Task,Idea,Memory,Error,Train)', false),
        createToolParameter('keywords', 'string', 'Comma-separated keywords for hybrid search', false),
        createToolParameter('semantic_weight', 'number', 'Weight for semantic score in hybrid search (0..1)', false, 0.7),
      ],
      'AI/Search',
      [ResourceLock.Memory]
    );
  }

  async execute(kwargs) {
    const t0 = Date.now();
    const action = (kwargs.action || '').toLowerCase();

    const contentTypes = kwargs.content_types
      ? kwargs.content_types.split(',').map(s => s.trim()).filter(Boolean)
      : null;

    const limit = typeof kwargs.limit === 'number' ? kwargs.limit : 10;
    const semanticWeight = typeof kwargs.semantic_weight === 'number' ? kwargs.semanticWeight : kwargs.semantic_weight;

    try {
      switch (action) {
        case 'find_similar': {
          if (!kwargs.content) return ErrorHandler.createErrorResult('content is required for find_similar', 0, ErrorType.ValidationError);
          const results = await this.service.findSimilarByText(kwargs.content, limit);
          return ErrorHandler.createSuccessResult(JSON.stringify(results, null, 2), Date.now() - t0, { action, count: results.length });
        }
        case 'semantic_search': {
          if (!kwargs.content) return ErrorHandler.createErrorResult('content is required for semantic_search', 0, ErrorType.ValidationError);
          const results = await this.service.semanticSearch(kwargs.content, contentTypes, limit);
          return ErrorHandler.createSuccessResult(JSON.stringify(results, null, 2), Date.now() - t0, { action, count: results.length });
        }
        case 'hybrid_search': {
          if (!kwargs.content) return ErrorHandler.createErrorResult('content is required for hybrid_search', 0, ErrorType.ValidationError);
          const keywords = (kwargs.keywords || '').split(',').map(s => s.trim()).filter(Boolean);
          const results = await this.service.hybridSearch(kwargs.content, keywords, contentTypes, semanticWeight, limit);
          return ErrorHandler.createSuccessResult(JSON.stringify(results, null, 2), Date.now() - t0, { action, count: results.length });
        }
        case 'cluster': {
          const clusters = await this.service.clusterContent();
          return ErrorHandler.createSuccessResult(JSON.stringify(clusters, null, 2), Date.now() - t0, { action, clusterCount: clusters.length });
        }
        case 'stats': {
          const stats = await this.service.stats();
          return ErrorHandler.createSuccessResult(JSON.stringify(stats, null, 2), Date.now() - t0, { action });
        }
        default:
          return ErrorHandler.createErrorResult(`Unknown action: ${action}`, 0, ErrorType.ValidationError);
      }
    } catch (e) {
      return ErrorHandler.handleError(e, 'todozi_embeddings.execute');
    }
  }
}

// ---------------------------------------------
// Demo: build a small dataset and run commands
// ---------------------------------------------
async function buildDemoDataset(service) {
  const items = [
    { id: 'task-1', type: 'Task', text: 'Implement user authentication with JWT', tags: ['auth', 'security'] },
    { id: 'task-2', type: 'Task', text: 'Write unit tests for billing module', tags: ['testing', 'billing'] },
    { id: 'task-3', type: 'Task', text: 'Refactor error handling across services', tags: ['refactor', 'errors'] },
    { id: 'idea-1', type: 'Idea', text: 'Add semantic search to knowledge base', tags: ['search', 'nlp'] },
    { id: 'idea-2', type: 'Idea', text: 'Create onboarding checklist for new engineers', tags: ['onboarding', 'process'] },
    { id: 'mem-1', type: 'Memory', text: 'Project kickoff emphasized reliability and incremental delivery', tags: ['project', 'delivery'] },
    { id: 'err-1', type: 'Error', text: 'Timeout errors spike under heavy load', tags: ['performance', 'stability'] },
    { id: 'train-1', type: 'Train', text: 'Prompt: write tests; Completion: use jest and coverage thresholds', tags: ['testing', 'jest'] },
    { id: 'task-4', type: 'Task', text: 'Set up CI pipeline with staged test jobs', tags: ['ci', 'testing'] },
    { id: 'task-5', type: 'Task', text: 'Add feature flag framework for gradual rollouts', tags: ['feature-flags', 'release'] },
  ];
  await service.addItems(items);
}

async function main() {
  const service = new SimpleEmbeddingService({ dimensions: 96, similarityThreshold: 0.65, maxResults: 15 });
  await buildDemoDataset(service);

  const registry = new ToolRegistry();
  const tool = new TodoziEmbeddingsTool(service);
  registry.register(tool);

  // CLI wiring: node example5.js <action> [options...]
  const [, , actionArg, ...rest] = process.argv;
  const args = parseArgs(rest);

  // If no action provided, show usage and run a few examples
  if (!actionArg || actionArg === 'example5.js') {
    printUsage();
    console.log('\n--- Running built-in examples ---\n');
    // Example 1: Find similar to a query
    let res = await registry.executeTool('todozi_embeddings', { action: 'find_similar', content: 'automated testing and coverage', limit: 5 });
    console.log('find_similar: "automated testing and coverage"\n', res.toString());

    // Example 2: Semantic search
    res = await registry.executeTool('todozi_embeddings', { action: 'semantic_search', content: 'reliability and error handling', content_types: 'Task,Error' });
    console.log('\nsemantic_search: "reliability and error handling" (Task,Error)\n', res.toString());

    // Example 3: Hybrid search
    res = await registry.executeTool('todozi_embeddings', { action: 'hybrid_search', content: 'unit tests', keywords: 'jest, coverage, CI', limit: 6 });
    console.log('\nhybrid_search: "unit tests" with keywords ["jest","coverage","CI"]\n', res.toString());

    // Example 4: Stats
    res = await registry.executeTool('todozi_embeddings', { action: 'stats' });
    console.log('\nstats\n', res.toString());

    // Example 5: Cluster
    res = await registry.executeTool('todozi_embeddings', { action: 'cluster' });
    console.log('\ncluster\n', res.toString());
    return;
  }

  const action = actionArg;
  const result = await registry.executeTool('todozi_embeddings', { action, ...args });
  console.log(result.toString());
}

// --------------- tiny CLI arg parser ----------------
function parseArgs(args) {
  const out = {};
  for (let i = 0; i < args.length; i++) {
    const a = args[i];
    if (a.startsWith('--')) {
      const key = a.slice(2);
      const next = args[i + 1];
      if (next && !next.startsWith('--')) {
        // number?
        if (!isNaN(next)) {
          out[key] = Number(next);
        } else {
          out[key] = next;
        }
        i++;
      } else {
        out[key] = true;
      }
    }
  }
  return out;
}

function printUsage() {
  console.log('Usage: node example5.js <action> [options...]');
  console.log('Actions: find_similar | semantic_search | hybrid_search | cluster | stats');
  console.log('Options:');
  console.log('  --content <text>        Query text (find_similar, semantic_search, hybrid_search)');
  console.log('  --limit <number>        Max results (default 10)');
  console.log('  --content_types <csv>   Comma-separated types: Task,Idea,Memory,Error,Train');
  console.log('  --keywords <csv>        Keywords for hybrid search');
  console.log('  --semantic_weight <0-1> Semantic weight for hybrid search (default 0.7)');
}

main().catch(err => {
  console.error('Fatal error:', err);
  process.exit(1);
});