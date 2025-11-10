// example5-tags.js
// A self-contained, executable example that demonstrates:
// - Using the TagManager (tags.js) to create and manage tags
// - Building a simple similarity-based tag suggestion engine
// - Attaching suggested tags to tasks and logging changes
//
// Run with: node example5-tags.js
//
// Notes:
// - This file uses dynamic import() to load CommonJS modules (tags.js, models.js, storage.js)
//   from the same project folder. Place this file next to those modules, or adjust the paths.

import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

// Resolve to current file's directory to find sibling modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Dynamic import wrappers for CommonJS modules
const importTags = () => import('./tags.js');
const importModels = () => import('./models.js');
const importStorage = () => import('./storage.js');

// A tiny similarity helper using Jaccard over word tokens
function jaccardSimilarity(a, b) {
  const tokenize = (s) =>
    new Set(
      (s || '')
        .toLowerCase()
        .replace(/[^a-z0-9\s]/g, ' ')
        .split(/\s+/)
        .filter(Boolean)
    );

  const A = tokenize(a);
  const B = tokenize(b);

  if (A.size === 0 && B.size === 0) return 0;
  const intersection = new Set([...A].filter((x) => B.has(x)));
  const union = new Set([...A, ...B]);
  return intersection.size / union.size;
}

// Demo tasks mimicking the Task shape used by models.js
const demoTasks = [
  {
    id: 'task_01',
    action: 'Implement OAuth2 login with Google',
    contextNotes: 'Handle callback and store refresh tokens securely',
    tags: ['auth', 'oauth', 'security'],
    parentProject: 'web_app',
    priority: 'high',
    status: 'todo'
  },
  {
    id: 'task_02',
    action: 'Optimize SQL query for reports dashboard',
    contextNotes: 'Add index on created_at and user_id; avoid N+1',
    tags: ['database', 'performance', 'sql'],
    parentProject: 'analytics',
    priority: 'medium',
    status: 'in_progress'
  },
  {
    id: 'task_03',
    action: 'Write unit tests for email validation',
    contextNotes: 'Cover RFC5322 patterns and edge cases',
    tags: ['testing', 'validation', 'email'],
    parentProject: 'core',
    priority: 'medium',
    status: 'todo'
  },
  {
    id: 'task_04',
    action: 'Add dark mode toggle to settings page',
    contextNotes: 'Persist user preference in localStorage',
    tags: ['ui', 'ux', 'frontend'],
    parentProject: 'web_app',
    priority: 'low',
    status: 'todo'
  }
];

async function main() {
  console.log('Example 5: Tag Manager and Tag Suggestions\n');

  // 1) Load TagManager
  const { TagManager } = await importTags();
  const tagManager = new TagManager();

  // 2) Create a set of canonical tags
  const tagDefs = [
    { name: 'auth', category: 'security', color: '#e67e22' },
    { name: 'security', category: 'security', color: '#e74c3c' },
    { name: 'database', category: 'backend', color: '#2980b9' },
    { name: 'performance', category: 'backend', color: '#27ae60' },
    { name: 'testing', category: 'qa', color: '#8e44ad' },
    { name: 'ui', category: 'frontend', color: '#16a085' },
    { name: 'ux', category: 'frontend', color: '#2ecc71' },
    { name: 'frontend', category: 'web', color: '#3498db' },
    { name: 'api', category: 'backend', color: '#9b59b6' },
    { name: 'email', category: 'communication', color: '#f39c12' }
  ];

  const createdIds = [];
  for (const t of tagDefs) {
    const id = await tagManager.createTag({
      name: t.name,
      description: `${t.name} tag`,
      category: t.category,
      color: t.color,
      usage_count: 0
    });
    createdIds.push({ id, ...t });
  }
  console.log(`Created ${createdIds.length} tags.`);

  // 3) Create some relationships to make suggestions richer
  // e.g., 'auth' relates to 'security' and 'api'
  const findTagId = (name) => createdIds.find((x) => x.name === name)?.id;
  await tagManager.addTagRelationship(findTagId('auth'), findTagId('security'));
  await tagManager.addTagRelationship(findTagId('auth'), findTagId('api'));
  await tagManager.addTagRelationship(findTagId('database'), findTagId('performance'));
  await tagManager.addTagRelationship(findTagId('ui'), findTagId('ux'));
  console.log('Added tag relationships.\n');

  // 4) Tag suggestion engine (simple, transparent)
  function suggestTagsForTask(task, topK = 5) {
    const text = `${task.action} ${task.contextNotes || ''}`;
    const candidates = tagManager.getAllTags().map((t) => {
      const label = `${t.name} ${t.description || ''}`;
      const score = jaccardSimilarity(text, label);
      return { tag: t, score };
    });

    // Boost by relationship and category match
    const relatedBoost = (tag) => {
      // Example heuristic: if any existing task tag is related, boost
      const currentTagNames = (task.tags || []).map((n) => n.toLowerCase());
      const related = tagManager.getRelatedTags(tag.id).map((t) => t.name.toLowerCase());
      const overlap = related.filter((n) => currentTagNames.includes(n)).length;
      return overlap * 0.1; // small boost
    };

    const categoryBoost = (tag) => {
      // Example heuristic: if action/context mentions category keywords, boost
      const categoryMap = {
        frontend: ['ui', 'ux', 'frontend', 'css', 'html', 'react', 'vue', 'svelte'],
        backend: ['api', 'server', 'sql', 'database', 'performance', 'backend', 'node'],
        security: ['auth', 'security', 'oauth', 'jwt', 'encryption'],
        qa: ['test', 'testing', 'qa', 'coverage']
      };
      const words = text.toLowerCase();
      const cats = Object.entries(categoryMap).filter(([_, kws]) =>
        kws.some((kw) => words.includes(kw))
      ).map(([cat]) => cat);

      return cats.includes((tag.category || '').toLowerCase()) ? 0.1 : 0;
    };

    for (const c of candidates) {
      c.score += relatedBoost(c.tag) + categoryBoost(c.tag);
    }

    candidates.sort((a, b) => b.score - a.score);
    return candidates.slice(0, topK);
  }

  // 5) Apply suggestions to demo tasks and log results
  console.log('Suggested tags per task:\n');
  for (const task of demoTasks) {
    const suggestions = suggestTagsForTask(task, 5);
    console.log(`Task: ${task.action}`);
    console.log(`- Existing tags: [${(task.tags || []).join(', ')}]`);
    console.log(
      `- Suggestions: ${suggestions
        .map((s) => `${s.tag.name} (${s.score.toFixed(2)})`)
        .join(', ')}`
    );

    // Attach top suggestion not already present
    const newTags = new Set(task.tags || []);
    for (const s of suggestions) {
      if (!newTags.has(s.tag.name)) {
        newTags.add(s.tag.name);
        // increment usage to simulate real use
        await tagManager.incrementTagUsage(s.tag.name);
        break;
      }
    }
    task.tags = [...newTags];
    console.log(`- Updated tags: [${task.tags.join(', ')}]`);
    console.log('');
  }

  // 6) Show updated tag stats
  const stats = tagManager.getTagStatistics();
  console.log('Tag Statistics:');
  console.log(`- Total tags: ${stats.total_tags}`);
  console.log(`- Total categories: ${stats.total_categories}`);
  console.log(`- Total relationships: ${stats.total_relationships}`);
  console.log(`- Average usage: ${stats.average_usage.toFixed(2)}`);

  const mostUsed = tagManager.getMostUsedTags(5);
  console.log(`- Most used: ${mostUsed.map((t) => `${t.name}(${t.usage_count})`).join(', ')}`);

  // 7) Search and fuzzy-find examples
  console.log('\nSearch examples:');
  const searchResults = tagManager.searchTags('frontend');
  console.log(`- searchTags('frontend') => [${searchResults.map((t) => t.name).join(', ')}]`);

  const fuzzyResults = tagManager
    .searchEngine()
    .fuzzySearch('performace', 2) // note typo; fuzzy search will still match
    .map(([t, d]) => `${t.name}[d=${d}]`);
  console.log(`- fuzzySearch('performace', 2) => [${fuzzyResults.join(', ')}]`);

  // 8) Export a simple tag report
  const reportPath = path.join(__dirname, 'example5-tag-report.json');
  const report = {
    generated_at: new Date().toISOString(),
    tags: tagManager.getAllTags(),
    stats,
    tasks: demoTasks
  };
  await fs.writeFile(reportPath, JSON.stringify(report, null, 2), 'utf8');
  console.log(`\nWrote report to: ${reportPath}`);

  console.log('\nExample 5 completed successfully.');
}

main().catch((err) => {
  console.error('Example 5 failed:', err);
  process.exit(1);
});