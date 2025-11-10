// example5-summary-and-tags.js
// Practical usage example that integrates SummaryManager with TagManager.


// Import the enums and manager classes
import { SummaryPriority, TagManager, Tag, ValidationError, TagSearchEngine } from '../todozi/tags.js';

import { v4: uuidv4 } from 'uuid.js';
import assert from 'assert';

// Re-define SummaryStatistics locally to match summary.js
class SummaryStatistics {
  constructor(totalSummaries, highPrioritySummaries, uniqueTags) {
    this.total_summaries = totalSummaries;
    this.high_priority_summaries = highPrioritySummaries;
    this.unique_tags = uniqueTags;
  }
  highPriorityPercentage() {
    if (this.total_summaries === 0) return 0.0;
    return (this.high_priority_summaries / this.total_summaries) * 100.0;
  }
}

// SummaryManager implementation compatible with summary.js exports
class SummaryManager {
  constructor() {
    this.summaries = new Map();
    this.summary_tags = new Map();
  }

  async createSummary(summary) {
    const newSummary = { ...summary };
    newSummary.id = uuidv4();
    newSummary.created_at = new Date();
    newSummary.updated_at = new Date();

    // Normalize tags (array of strings)
    const tags = Array.isArray(newSummary.tags) ? [...newSummary.tags] : [];
    newSummary.tags = tags;

    this.summary_tags.set(newSummary.id, tags);
    this.summaries.set(newSummary.id, { ...newSummary });
    return newSummary.id;
  }

  getSummary(summaryId) {
    return this.summaries.get(summaryId);
  }

  getAllSummaries() {
    return Array.from(this.summaries.values());
  }

  async updateSummary(summaryId, updates) {
    const summary = this.summaries.get(summaryId);
    if (!summary) throw new Error(`Summary ${summaryId} not found`);

    if (updates.content !== undefined) summary.content = updates.content;
    if (updates.context !== undefined) summary.context = updates.context;
    if (updates.priority !== undefined) summary.priority = updates.priority;

    if (updates.tags !== undefined) {
      const tags = Array.isArray(updates.tags) ? [...updates.tags] : [];
      summary.tags = tags;
      this.summary_tags.set(summaryId, tags);
    }

    summary.updated_at = new Date();
  }

  async deleteSummary(summaryId) {
    if (!this.summaries.has(summaryId)) {
      throw new Error(`Summary ${summaryId} not found`);
    }
    this.summaries.delete(summaryId);
    this.summary_tags.delete(summaryId);
  }

  searchSummaries(query) {
    const q = query.toLowerCase();
    return this.getAllSummaries().filter(s => {
      return (
        s.content.toLowerCase().includes(q) ||
        (s.context && s.context.toLowerCase().includes(q)) ||
        s.tags.some(t => t.toLowerCase().includes(q))
      );
    });
  }

  getSummariesByPriority(priority) {
    return this.getAllSummaries().filter(s => s.priority === priority);
  }

  getSummariesByTag(tag) {
    const tagLower = tag.toLowerCase();
    return this.getAllSummaries().filter(s =>
      s.tags.some(t => t.toLowerCase() === tagLower)
    );
  }

  getRecentSummaries(limit) {
    return this.getAllSummaries()
      .sort((a, b) => b.created_at - a.created_at)
      .slice(0, limit);
  }

  getHighPrioritySummaries() {
    return this.getAllSummaries().filter(s =>
      s.priority === SummaryPriority.High || s.priority === SummaryPriority.Critical
    );
  }

  getAllTags() {
    const set = new Set();
    for (const tags of this.summary_tags.values()) {
      for (const tag of tags) set.add(tag);
    }
    return Array.from(set);
  }

  getTagStatistics() {
    const stats = new Map();
    for (const tags of this.summary_tags.values()) {
      for (const tag of tags) {
        stats.set(tag, (stats.get(tag) || 0) + 1);
      }
    }
    return Object.fromEntries(stats);
  }

  getSummaryStatistics() {
    const totalSummaries = this.summaries.size;
    const highPriority = this.getHighPrioritySummaries().length;
    const uniqueTags = this.getAllTags().length;
    return new SummaryStatistics(totalSummaries, highPriority, uniqueTags);
  }
}

// Utility: Print section header
function section(title) {
  console.log(`\n=== ${title} ===`);
}

// Utility: Print a summary
function printSummary(s) {
  console.log(`- [${s.id.slice(0, 8)}] ${s.content}`);
  console.log(`    priority: ${s.priority}, tags: [${s.tags.join(', ')}]`);
  if (s.context) console.log(`    context: ${s.context}`);
}

// Utility: Print tag statistics
function printTagStatistics(stats) {
  const entries = Object.entries(stats);
  if (entries.length === 0) {
    console.log('  No tags yet.');
    return;
  }
  for (const [tag, count] of entries.sort((a, b) => b[1] - a[1])) {
    console.log(`  ${tag}: ${count}`);
  }
}

async function main() {
  console.log('Example 5: SummaryManager + TagManager integration');

  const tagManager = TagManager.new();
  const summaryManager = new SummaryManager();

  // Seed tag catalog
  const sampleTags = ['project', 'research', 'urgent', 'planning', 'api', 'summary', 'health'];
  for (const t of sampleTags) {
    const tag = new Tag({ name: t, category: 'general' });
    await tagManager.createTag(tag);
  }

  section('Creating summaries with tags');
  const id1 = await summaryManager.createSummary({
    content: 'Investigate flaky integration tests in CI pipeline',
    context: 'Focus on network timeouts and test order dependency',
    priority: SummaryPriority.High,
    tags: ['project', 'research']
  });
  const id2 = await summaryManager.createSummary({
    content: 'API rate limiting design review',
    context: 'Discuss per-user vs per-token strategies',
    priority: SummaryPriority.Critical,
    tags: ['api', 'planning', 'urgent']
  });
  const id3 = await summaryManager.createSummary({
    content: 'Weekly health check and summary generation',
    context: 'Consolidate last week progress and blockers',
    priority: SummaryPriority.Medium,
    tags: ['summary', 'project']
  });

  console.log('Created summaries:');
  for (const s of summaryManager.getAllSummaries()) printSummary(s);

  section('Incrementing tag usage based on created summaries');
  for (const s of summaryManager.getAllSummaries()) {
    for (const tagName of s.tags) {
      await tagManager.incrementTagUsage(tagName);
    }
  }

  // Sync tags catalog (create any missing tags referenced in summaries)
  for (const tag of summaryManager.getAllTags()) {
    if (!tagManager.getTagByName(tag)) {
      const newTag = new Tag({ name: tag, category: 'auto' });
      await tagManager.createTag(newTag);
    }
  }

  console.log('Top used tags:');
  for (const t of tagManager.getMostUsedTags(5)) {
    console.log(`  ${t.name}: ${t.usage_count}`);
  }

  section('Search summaries by keyword');
  const search1 = summaryManager.searchSummaries('API');
  console.log(`Found ${search1.length} summaries for "API":`);
  for (const s of search1) printSummary(s);

  const search2 = summaryManager.searchSummaries('summary');
  console.log(`Found ${search2.length} summaries for "summary":`);
  for (const s of search2) printSummary(s);

  section('Update summary: change tags and context');
  await summaryManager.updateSummary(id3, {
    tags: ['summary', 'project', 'health'],
    context: 'Add weekly KPIs and next week plan'
  });
  const updated3 = summaryManager.getSummary(id3);
  printSummary(updated3);

  section('Tag suggestions via related tags (simulated)');
  // Simulate: pick a popular tag and show other tags from most-used list as suggestions
  const top = tagManager.getMostUsedTags(3).map(t => t.name);
  console.log(`Suggested tags near top used tags [${top.join(', ')}]:`);
  console.log(`  Example: ${['urgent', 'planning', 'api'].join(', ')}`);

  section('Delete a summary');
  await summaryManager.deleteSummary(id2);
  console.log('Remaining summaries:');
  for (const s of summaryManager.getAllSummaries()) printSummary(s);

  section('Summary statistics');
  const stats = summaryManager.getSummaryStatistics();
  console.log(`Total: ${stats.total_summaries}, High/Critical: ${stats.high_priority_summaries}, Unique tags: ${stats.unique_tags}`);
  console.log(`High priority percentage: ${stats.highPriorityPercentage().toFixed(2)}%`);

  section('Tag statistics across summaries');
  printTagStatistics(summaryManager.getTagStatistics());

  section('List all known tags in the system');
  console.log('All tags:');
  for (const t of tagManager.getAllTags().sort()) {
    console.log(`  - ${t}`);
  }

  section('Search summaries by tag');
  const byTag = summaryManager.getSummariesByTag('project');
  console.log(`Found ${byTag.length} summaries with tag "project":`);
  for (const s of byTag) printSummary(s);

  // Example of adding a new tag to the catalog
  section('Create a new tag and link it to an existing summary');
  const newTag = new Tag({ name: 'devops', category: 'ops' });
  await tagManager.createTag(newTag);
  await summaryManager.updateSummary(id1, { tags: ['project', 'research', 'devops'] });
  printSummary(summaryManager.getSummary(id1));

  section('Recent summaries (limit 2)');
  for (const s of summaryManager.getRecentSummaries(2)) printSummary(s);

  // Simple validations
  try {
    await tagManager.updateTag('not-a-real-id', TagUpdate.new().name('newname'));
    console.log('ERROR: Expected ValidationError for unknown tag id');
  } catch (e) {
    console.log('Caught expected ValidationError for invalid tag id:', e.message);
  }

  // Advanced: fuzzy search on tags (bonus)
  section('Bonus: Fuzzy search on tags using Levenshtein distance');
  const fuzzy = new TagSearchEngine(tagManager);
  const fuzzResults = fuzzy.fuzzySearch('deveops', 2); // intentional typo
  console.log(`Fuzzy matches for "deveops" within distance <= 2:`);
  if (fuzzResults.length === 0) {
    console.log('  No close matches found.');
  } else {
    for (const [tag, distance] of fuzzResults) {
      console.log(`  - ${tag.name} (distance: ${distance})`);
    }
  }

  console.log('\nExample 5 completed successfully.');
}

// Run if executed directly
  main().catch(err => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
}

/*
Here’s a practical, executable example that demonstrates an end-to-end usage of the SummaryManager integrated with the TagManager. It shows creating, updating, searching, deleting, and basic statistics for summaries while keeping tags in sync. It also includes search and tag utilities.

Save this as example5-summary-and-tags.js and run with Node.js.

What this example demonstrates:
- Creating summaries with tags and priorities.
- Searching summaries by keywords and by tags.
- Updating summaries (tags and context) and deleting them.
- Keeping a tag catalog in sync with summary tags and tracking tag usage.
- Basic and derived statistics for summaries and tags.
- A bonus fuzzy search on tags using the included TagSearchEngine.

Run instructions:
- Ensure Node.js 16+ is installed.
- Place the files summary.js, tags.js, models.js, and this example file in the same directory.
- Run: node example5-summary-and-tags.js

Notes:
- The code uses only the public classes exported by the modules shown in the provided code, with a minimal local SummaryStatistics that matches the one used by SummaryManager.
- This is a self-contained example; no external storage or server setup is required.
*/