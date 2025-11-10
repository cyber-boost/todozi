// example5.js
// Run with: node example5.js
// This is a self-contained demo that mirrors patterns from the provided codebase.
// No external dependencies required.

import { randomUUID } from 'crypto';

// -----------------------
// Minimal domain models
// -----------------------

class Task {
  constructor({ id, action, status, priority, project, tags = [] }) {
    this.id = id || `task_${randomUUID().slice(0, 8)}`;
    this.action = action;
    this.status = status || 'todo';
    this.priority = priority || 'medium';
    this.project = project || 'general';
    this.tags = tags;
  }
}

class Memory {
  constructor({ id, moment, meaning, importance = 'medium', tags = [] }) {
    this.id = id || `mem_${randomUUID().slice(0, 8)}`;
    this.moment = moment;
    this.meaning = meaning;
    this.importance = importance;
    this.tags = tags;
  }
}

class Idea {
  constructor({ id, idea, share = 'private', importance = 'medium', tags = [] }) {
    this.id = id || `idea_${randomUUID().slice(0, 8)}`;
    this.idea = idea;
    this.share = share;
    this.importance = importance;
    this.tags = tags;
  }
}

class TdzError {
  constructor({ id, title, severity = 'medium', category = 'runtime' }) {
    this.id = id || `err_${randomUUID().slice(0, 8)}`;
    this.title = title;
    this.severity = severity;
    this.category = category;
  }
}

class TrainingData {
  constructor({ id, prompt, completion, dataType = 'instruction' }) {
    this.id = id || `tr_${randomUUID().slice(0, 8)}`;
    this.prompt = prompt;
    this.completion = completion;
    this.dataType = dataType;
  }
}

// -----------------------
// Simple Search Engine
// -----------------------

/**
 * @typedef {Object} SearchOptions
 * @property {number} [limit]
 * @property {string[]} [dataTypes]
 * @property {string} [since]
 * @property {string} [until]
 */

/**
 * @typedef {Object} SearchResults
 * @property {Array<any>} taskResults
 * @property {Array<any>} memoryResults
 * @property {Array<any>} ideaResults
 * @property {Array<any>} errorResults
 * @property {Array<any>} trainingResults
 */

class SearchEngine {
  constructor() {
    this.index = {
      tasks: [],
      memories: [],
      ideas: [],
      errors: [],
      training: [],
    };
  }

  updateIndex(content) {
    if (content.tasks) this.index.tasks.push(...content.tasks);
    if (content.memories) this.index.memories.push(...content.memories);
    if (content.ideas) this.index.ideas.push(...content.ideas);
    if (content.errors) this.index.errors.push(...content.errors);
    if (content.trainingData) this.index.training.push(...content.trainingData);
  }

  /**
   * Unified search
   * @param {string} query
   * @param {SearchOptions} [options]
   * @returns {SearchResults}
   */
  search(query, options = {}) {
    const limit = typeof options.limit === 'number' ? options.limit : 10;
    const dataTypes =
      Array.isArray(options.dataTypes) && options.dataTypes.length > 0
        ? new Set(options.dataTypes)
        : new Set(['tasks', 'memories', 'ideas', 'errors', 'training']);

    const q = query.toLowerCase();

    const taskResults = dataTypes.has('tasks')
      ? this.index.tasks
          .map((t) => ({
            type: 'task',
            id: t.id,
            title: t.action,
            snippet: `Project: ${t.project}, Status: ${t.status}, Priority: ${t.priority}`,
            score: this.scoreText(q, [t.action, t.project, t.status, t.priority, ...(t.tags || [])]),
          }))
          .filter((r) => r.score > 0)
          .sort((a, b) => b.score - a.score)
          .slice(0, limit)
      : [];

    const memoryResults = dataTypes.has('memories')
      ? this.index.memories
          .map((m) => ({
            type: 'memory',
            id: m.id,
            title: m.moment,
            snippet: `Meaning: ${m.meaning} (importance: ${m.importance})`,
            score: this.scoreText(q, [m.moment, m.meaning, m.importance, ...(m.tags || [])]),
          }))
          .filter((r) => r.score > 0)
          .sort((a, b) => b.score - a.score)
          .slice(0, limit)
      : [];

    const ideaResults = dataTypes.has('ideas')
      ? this.index.ideas
          .map((i) => ({
            type: 'idea',
            id: i.id,
            title: i.idea,
            snippet: `Share: ${i.share}, Importance: ${i.importance}`,
            score: this.scoreText(q, [i.idea, i.share, i.importance, ...(i.tags || [])]),
          }))
          .filter((r) => r.score > 0)
          .sort((a, b) => b.score - a.score)
          .slice(0, limit)
      : [];

    const errorResults = dataTypes.has('errors')
      ? this.index.errors
          .map((e) => ({
            type: 'error',
            id: e.id,
            title: e.title,
            snippet: `Severity: ${e.severity}, Category: ${e.category}`,
            score: this.scoreText(q, [e.title, e.severity, e.category]),
          }))
          .filter((r) => r.score > 0)
          .sort((a, b) => b.score - a.score)
          .slice(0, limit)
      : [];

    const trainingResults = dataTypes.has('training')
      ? this.index.training
          .map((t) => ({
            type: 'training',
            id: t.id,
            title: t.prompt,
            snippet: `Completion: ${t.completion} (type: ${t.dataType})`,
            score: this.scoreText(q, [t.prompt, t.completion, t.dataType]),
          }))
          .filter((r) => r.score > 0)
          .sort((a, b) => b.score - a.score)
          .slice(0, limit)
      : [];

    return {
      taskResults,
      memoryResults,
      ideaResults,
      errorResults,
      trainingResults,
    };
  }

  // Simple scoring: 2 points for word boundary match, 1 point for substring match
  scoreText(queryLower, fields) {
    let score = 0;
    const words = queryLower.split(/\s+/).filter(Boolean);
    const haystack = fields
      .filter(Boolean)
      .join(' ')
      .toLowerCase();

    for (const w of words) {
      if (new RegExp(`\\b${escapeRegExp(w)}`).test(haystack)) score += 2;
      else if (haystack.includes(w)) score += 1;
    }
    return score;
  }
}

function escapeRegExp(s) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// -----------------------
// Demo dataset
// -----------------------

function seedData() {
  const tasks = [
    new Task({
      action: 'Set up project repository',
      status: 'todo',
      priority: 'high',
      project: 'platform',
      tags: ['git', 'setup'],
    }),
    new Task({
      action: 'Implement user authentication',
      status: 'in_progress',
      priority: 'high',
      project: 'platform',
      tags: ['security', 'auth', 'backend'],
    }),
    new Task({
      action: 'Design database schema',
      status: 'todo',
      priority: 'medium',
      project: 'platform',
      tags: ['database', 'design'],
    }),
    new Task({
      action: 'Write unit tests for billing module',
      status: 'todo',
      priority: 'medium',
      project: 'billing',
      tags: ['tests', 'billing'],
    }),
    new Task({
      action: 'Create CI pipeline',
      status: 'done',
      priority: 'high',
      project: 'platform',
      tags: ['ci', 'automation'],
    }),
  ];

  const memories = [
    new Memory({
      moment: 'Kickoff meeting decided monorepo',
      meaning: 'Simplifies sharing libraries across services',
      importance: 'high',
      tags: ['architecture', 'monorepo'],
    }),
    new Memory({
      moment: 'Billing latency issues observed in staging',
      meaning: 'May need to optimize DB queries or add cache',
      importance: 'high',
      tags: ['billing', 'performance'],
    }),
    new Memory({
      moment: 'Users requesting dark mode',
      meaning: 'Low effort, high impact UI improvement',
      importance: 'medium',
      tags: ['ui', 'feature'],
    }),
  ];

  const ideas = [
    new Idea({
      idea: 'Add semantic search to internal docs',
      share: 'team',
      importance: 'high',
      tags: ['search', 'docs'],
    }),
    new Idea({
      idea: 'Introduce agent-based code review',
      share: 'private',
      importance: 'medium',
      tags: ['ai', 'code-review'],
    }),
    new Idea({
      idea: 'Gamify test coverage goals',
      share: 'private',
      importance: 'low',
      tags: ['tests', 'engagement'],
    }),
  ];

  const errors = [
    new TdzError({
      title: 'NullPointerException in PaymentService',
      severity: 'high',
      category: 'runtime',
    }),
    new TdzError({
      title: 'Unhandled promise rejection in auth flow',
      severity: 'medium',
      category: 'runtime',
    }),
    new TdzError({
      title: 'Schema mismatch detected on deploy',
      severity: 'critical',
      category: 'database',
    }),
  ];

  const training = [
    new TrainingData({
      prompt: 'Summarize the key points of this PR description',
      completion: 'Extract goals, changes, risks, and testing strategy.',
      dataType: 'instruction',
    }),
    new TrainingData({
      prompt: 'Generate a polite code review comment for minor style issues',
      completion: 'Start with appreciation, mention the issue briefly, suggest a fix, and offer to help.',
      dataType: 'completion',
    }),
    new TrainingData({
      prompt: 'Explain the difference between unit and integration tests',
      completion: 'Unit tests focus on isolated components; integration tests verify interactions between components.',
      dataType: 'documentation',
    }),
  ];

  return { tasks, memories, ideas, errors, training };
}

// -----------------------
// Pretty-print helpers
// -----------------------

function printHeader(text) {
  console.log('\n' + text);
  console.log('='.repeat(text.length));
}

function printSubheader(text) {
  console.log('\n' + text);
  console.log('-'.repeat(text.length));
}

function printResultsList(label, results) {
  if (!results || results.length === 0) {
    console.log(`  (no ${label} results)`);
    return;
  }
  results.forEach((r, i) => {
    console.log(
      `  ${i + 1}. [${r.type}] ${r.title} — ${r.snippet} (score: ${r.score})`
    );
  });
}

function printAggregatedCounts(results) {
  const counts = {
    tasks: results.taskResults.length,
    memories: results.memoryResults.length,
    ideas: results.ideaResults.length,
    errors: results.errorResults.length,
    training: results.trainingResults.length,
  };
  console.log('  Total counts =>', counts);
}

// -----------------------
// Main demo
// -----------------------

async function main() {
  console.log('Example 5: Unified Search across tasks, memories, ideas, errors, and training data\n');

  const { tasks, memories, ideas, errors, training } = seedData();

  const content = {
    tasks,
    memories,
    ideas,
    errors,
    trainingData: training,
  };

  const engine = new SearchEngine();
  engine.updateIndex(content);

  const scenarios = [
    { query: 'billing', types: undefined, limit: undefined },
    { query: 'tests', types: undefined, limit: undefined },
    { query: 'schema', types: undefined, limit: undefined },
    { query: 'auth flow', types: ['tasks', 'errors'], limit: 5 },
    { query: 'semantic search docs', types: ['ideas', 'training'], limit: 5 },
    { query: 'ci automation', types: undefined, limit: 3 },
  ];

  for (const s of scenarios) {
    const results = engine.search(s.query, {
      limit: s.limit,
      dataTypes: s.types,
    });

    printHeader(`Search: "${s.query}"`);
    if (s.types && s.types.length) {
      console.log('Filters => dataTypes:', s.types.join(', '));
    }
    if (s.limit) console.log('Limits => limit:', s.limit);

    printSubheader('Results by type');
    printResultsList('tasks', results.taskResults);
    printResultsList('memories', results.memoryResults);
    printResultsList('ideas', results.ideaResults);
    printResultsList('errors', results.errorResults);
    printResultsList('training', results.trainingResults);

    printAggregatedCounts(results);
  }

  // Additional: demonstrate tag-centric searches (tags are included in scoring)
  printHeader('Tag-centric search: "ui"');
  const uiResults = engine.search('ui', { limit: 10 });
  printResultsList('tasks', uiResults.taskResults);
  printResultsList('memories', uiResults.memoryResults);
  printResultsList('ideas', uiResults.ideaResults);
  printAggregatedCounts(uiResults);

  // Additional: demonstrate category/severity-centric search via error title keywords
  printHeader('Search: "database"');
  const dbResults = engine.search('database', { limit: 10 });
  printResultsList('errors', dbResults.errorResults);
  printResultsList('tasks', dbResults.taskResults);
  printAggregatedCounts(dbResults);

  console.log('\nDemo complete.');
}

main().catch((err) => {
  console.error('Demo failed:', err);
  process.exit(1);
});

/*
Below is a self-contained, runnable example (Example 5) that demonstrates the unified search feature and related data handling in a minimal, practical way. It uses only Node’s built-in modules and a few tiny stubs for dependencies, so you can run it directly with: node example5.js

This example:
- Simulates core data structures (Task, Memory, Idea, Error, TrainingData)
- Implements a simple SearchEngine with updateIndex and search
- Shows unified search across tasks, memories, ideas, errors, and training data
- Prints practical, readable results

Save as example5.js and run with Node 18+.

What this demonstrates:
- Unified search across multiple data types (tasks, memories, ideas, errors, training data)
- Lightweight scoring and type-based filtering
- Simple, dependency-free code you can extend or adapt to your own codebase

If you want this to hit the full TodoziHandler pipeline shown in the provided code, you can reuse the SearchEngine instance and plug it into the handler’s handleSearchAllCommand flow, feeding it with the live storage and content sources.
*/