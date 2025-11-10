import { IdeaManager, IdeaUpdate, parseIdeaFormat } from '../todozi/idea.js';
import { ShareLevel, IdeaImportance, ItemStatus } from '../todozi/models.js';

Here’s a self-contained, runnable example (example5.js) that demonstrates the IdeaManager and parsing ideas from a simple DSL-like format. It uses the classes from the provided files and shows create, search, update, and delete flows, plus tag statistics.

How to run:
- Ensure Node.js >= 16
- Save as example5.js
- Run: node example5.js

Code:

javascript
// example5.js
// Demonstrates IdeaManager usage, parsing ideas from a simple format,
// and performing common operations (create, search, update, delete, stats).


// Patch enums in case the consuming environment uses const enums as strings
// If enums are string-based (as in types.js), make sure they match usage in idea.js
// This is a safe guard:
function normalizeShareLevel(v) {
  if (v == null) return ShareLevel.PRIVATE;
  const s = String(v).toLowerCase();
  if (s === 'public') return ShareLevel.PUBLIC;
  if (s === 'team') return ShareLevel.TEAM;
  if (s === 'private') return ShareLevel.PRIVATE;
  return ShareLevel.PRIVATE;
}
function normalizeIdeaImportance(v) {
  // idea.js expects numbers 1..4
  if (v == null) return IdeaImportance.Low;
  const s = String(v).toLowerCase();
  if (['low', 1, '1'].includes(s)) return IdeaImportance.Low;
  if (['medium', 2, '2'].includes(s)) return IdeaImportance.Medium;
  if (['high', 3, '3'].includes(s)) return IdeaImportance.High;
  if (['breakthrough', 4, '4'].includes(s)) return IdeaImportance.Breakthrough;
  return IdeaImportance.Low;
}

function now() {
  return new Date();
}

class IdeaManagerDemo {
  constructor() {
    this.manager = new IdeaManager();
  }

  // Adds multiple ideas by parsing their DSL format
  addFromRawTextLines(lines) {
    const results = [];
    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed) continue;
      const wrapped = `<idea>${trimmed}</idea>`;
      try {
        const idea = parseIdeaFormat(wrapped);
        // Normalize to ensure string shares and string importance for safety
        idea.share = normalizeShareLevel(idea.share);
        idea.importance = normalizeIdeaImportance(idea.importance);

        // Save via manager
        this.manager.ideas.set(idea.id, { ...idea });
        // keep tags in separate index
        this.manager.idea_tags.set(idea.id, [...(idea.tags || [])]);

        results.push({ id: idea.id, ok: true, idea });
      } catch (e) {
        results.push({ ok: false, line, error: e.message || String(e) });
      }
    }
    return results;
  }

  addIdea({ idea, share = 'private', importance = 1, tags = [], context = null }) {
    const id = `idea_${Date.now()}_${Math.floor(Math.random() * 10000)}`;
    const item = {
      id,
      idea,
      projectId: null,
      status: ItemStatus.Active,
      share: normalizeShareLevel(share),
      importance: normalizeIdeaImportance(importance),
      tags: Array.isArray(tags) ? tags : [],
      context: context || null,
      createdAt: now(),
      updatedAt: now(),
    };
    this.manager.ideas.set(id, { ...item });
    this.manager.idea_tags.set(id, [...item.tags]);
    return id;
  }

  updateIdea(id, updates) {
    // convert possible string importance to number
    if (updates.importance !== undefined) {
      updates.importance = normalizeIdeaImportance(updates.importance);
    }
    return this.manager.updateIdea(id, updates);
  }

  deleteIdea(id) {
    return this.manager.deleteIdea(id);
  }

  listAll() {
    return this.manager.getAllIdeas();
  }

  search(q) {
    return this.manager.searchIdeas(q);
  }

  tags() {
    return this.manager.getAllTags();
  }

  tagStats() {
    return this.manager.getTagStatistics();
  }

  ideaStats() {
    return this.manager.getIdeaStatistics();
  }

  recent(limit = 3) {
    return this.manager.getRecentIdeas(limit);
  }
}

async function main() {
  console.log('=== IdeaManager Demo (Example 5) ===\n');

  const demo = new IdeaManagerDemo();

  // 1) Create ideas from simple DSL format
  console.log('1) Creating ideas from simple format (idea; share; importance; tags; context;)');
  const rawLines = [
    // share can be: public | team | private
    // importance can be: 1-4 or low/medium/high/breakthrough
    'Automate CI checks; public; 2; ci, quality; Use GitHub Actions for fast feedback on PRs',
    'Adopt GraphQL for mobile clients; team; 3; api, mobile; Reduce overfetching for mobile views',
    'Explore state machines for checkout; private; 4; ux, flow; Simplify complex form flows',
    'Weekly tech debt triage; team; 1; process, quality; Keep 30 min every Friday',
  ];

  const parsed = demo.addFromRawTextLines(rawLines);
  for (const r of parsed) {
    if (r.ok) {
      console.log(`  ✓ Created idea ${r.id}: "${r.idea.idea}"`);
    } else {
      console.log(`  ✗ Failed to parse: "${r.line}" -> ${r.error}`);
    }
  }

  // 2) Add one idea programmatically
  const extraId = demo.addIdea({
    idea: 'Introduce typed queries for analytics',
    share: 'team',
    importance: 3,
    tags: ['analytics', 'typescript', 'api'],
    context: 'Ship types for better DX and fewer runtime bugs',
  });
  console.log(`\n2) Created idea programmatically: ${extraId}`);

  // 3) List all ideas
  console.log('\n3) All ideas:');
  for (const idea of demo.listAll()) {
    console.log(`  - [${idea.importance}] (${idea.share}) ${idea.idea} ${idea.tags.length ? `#${idea.tags.join(' #')}` : ''}`);
  }

  // 4) Search ideas
  console.log('\n4) Search for "api":');
  for (const idea of demo.search('api')) {
    console.log(`  - ${idea.idea} (share=${idea.share}, importance=${idea.importance})`);
  }

  // 5) Update idea
  console.log('\n5) Updating the first CI idea to private and add a new tag');
  const firstIdea = demo.listAll()[0];
  if (firstIdea) {
    await demo.updateIdea(firstIdea.id, IdeaUpdate.new()
      .withShare('private')
      .withImportance('high') // "high" normalizes to 3
      .withTags([...(firstIdea.tags || []), 'review'])
      .withContext('Tighten visibility while prototyping')
    );
    console.log(`  ✓ Updated idea ${firstIdea.id}`);
  }

  // 6) Delete an idea
  console.log('\n6) Deleting the tech debt triage idea');
  const debtIdea = demo.search('tech debt triage')[0];
  if (debtIdea) {
    await demo.deleteIdea(debtIdea.id);
    console.log(`  ✓ Deleted idea ${debtIdea.id}`);
  } else {
    console.log('  - No matching idea found (may have been parsed differently).');
  }

  // 7) Tag utilities
  console.log('\n7) Tag utilities:');
  console.log('  All tags:', demo.tags().join(', '));
  console.log('  Tag statistics:');
  for (const [tag, count] of demo.tagStats().entries()) {
    console.log(`    - ${tag}: ${count}`);
  }

  // 8) Idea statistics
  console.log('\n8) Idea statistics:');
  const stats = demo.ideaStats();
  console.log(JSON.stringify({
    totalIdeas: stats.totalIdeas,
    publicIdeas: stats.publicIdeas,
    teamIdeas: stats.teamIdeas,
    privateIdeas: stats.privateIdeas,
    breakthroughIdeas: stats.breakthroughIdeas,
    uniqueTags: stats.uniqueTags,
    publicPercentage: stats.publicPercentage(),
    teamPercentage: stats.teamPercentage(),
    privatePercentage: stats.privatePercentage(),
    breakthroughPercentage: stats.breakthroughPercentage(),
  }, null, 2));

  // 9) Recent ideas
  console.log('\n9) Recent ideas (most recent 3):');
  for (const idea of demo.recent(3)) {
    console.log(`  - ${idea.idea} (importance=${idea.importance})`);
  }

  console.log('\n=== Demo Complete ===');
}

main().catch((e) => {
  console.error('Fatal error:', e);
  process.exit(1);
});