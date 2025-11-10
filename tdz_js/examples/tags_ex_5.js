

/*
import { randomUUID } from 'crypto';

 * Example 5 — Practical usage and extension of the Tag module
 *
 * This standalone script:
 * - Seeds reproducible demo data
 * - Uses TagManager to create tags
 * - Demonstrates basic, advanced, fuzzy, and suggestion search
 * - Extends TagSearchEngine with a BoostingTagSearchEngine that
 *   supports category/tag boosts and query-term boosting
 *
 * To run:
 *   1) Ensure Node.js >= 16 (for crypto.randomUUID) is installed
 *   2) Save this file as example5.js
 *   3) Run: node example5.js
 *
 * No external dependencies required.
 */

'use strict';

// Fallback UUID (uses crypto.randomUUID if available)
const uuidv4 = () => {
  try {
    return randomUUID();
  } catch {
    // Fallback if crypto.randomUUID is not available
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, c => {
      const r = (Math.random() * 16) | 0;
      const v = c === 'x' ? r : (r & 0x3) | 0x8;
      return v.toString(16);
    });
  }
};

// Seeded RNG for deterministic demo data
function seededRng(seed) {
  let s = 0;
  for (let i = 0; i < seed.length; i++) s = (s * 31 + seed.charCodeAt(i)) >>> 0;
  return function () {
    s ^= s << 13;
    s ^= s >>> 17;
    s ^= s << 5;
    return (s >>> 0) / 0xffffffff;
  };
}

// Minimal Tag* classes extracted for self-contained execution
// (Matching the public API in tags.js)
class TodoziError extends Error {
  constructor(type, message) {
    super(message);
    this.type = type;
  }
}
class ValidationError extends TodoziError {
  constructor(message) {
    super('ValidationError', message);
  }
}
class Tag {
  constructor({
    id = '',
    name,
    description = null,
    color = null,
    category = null,
    usage_count = 0,
    created_at = new Date(),
    updated_at = new Date()
  } = {}) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.color = color;
    this.category = category;
    this.usage_count = usage_count;
    this.created_at = created_at;
    this.updated_at = updated_at;
  }
}
class TagUpdate {
  constructor() {
    this.name = null;
    this.description = null;
    this.color = null;
    this.category = null;
  }
  static new() {
    return new TagUpdate();
  }
  name(name) {
    this.name = name;
    return this;
  }
  description(description) {
    this.description = description;
    return this;
  }
  color(color) {
    this.color = color;
    return this;
  }
  category(category) {
    this.category = category;
    return this;
  }
}
class TagStatistics {
  constructor({
    total_tags = 0,
    total_categories = 0,
    total_relationships = 0,
    average_usage = 0.0
  } = {}) {
    this.total_tags = total_tags;
    this.total_categories = total_categories;
    this.total_relationships = total_relationships;
    this.average_usage = average_usage;
  }
  relationshipsPerTag() {
    if (this.total_tags === 0) return 0.0;
    return this.total_relationships / this.total_tags;
  }
}
class TagManager {
  constructor() {
    this.tags = new Map();
    this.tag_relationships = new Map();
    this.category_tags = new Map();
  }
  static new() {
    return new TagManager();
  }
  async createTag(tag) {
    const newTag = { ...tag };
    newTag.id = uuidv4();
    newTag.created_at = new Date();
    newTag.updated_at = new Date();
    if (newTag.category) {
      if (!this.category_tags.has(newTag.category)) {
        this.category_tags.set(newTag.category, []);
      }
      this.category_tags.get(newTag.category).push(newTag.id);
    }
    this.tags.set(newTag.id, newTag);
    return newTag.id;
  }
  getTag(tagId) {
    return this.tags.get(tagId);
  }
  getTagByName(name) {
    for (const tag of this.tags.values()) {
      if (tag.name === name) return tag;
    }
    return undefined;
  }
  getAllTags() {
    return Array.from(this.tags.values());
  }
  async updateTag(tagId, updates) {
    const tag = this.tags.get(tagId);
    if (!tag) throw new ValidationError(`Tag ${tagId} not found`);
    const oldCategory = tag.category;
    if (updates.name !== undefined) tag.name = updates.name;
    if (updates.description !== undefined) tag.description = updates.description;
    if (updates.color !== undefined) tag.color = updates.color;
    if (updates.category !== undefined) {
      tag.category = updates.category;
      if (oldCategory) {
        const tagIds = this.category_tags.get(oldCategory);
        if (tagIds) {
          const index = tagIds.indexOf(tagId);
          if (index > -1) tagIds.splice(index, 1);
        }
      }
      if (!this.category_tags.has(updates.category)) {
        this.category_tags.set(updates.category, []);
      }
      this.category_tags.get(updates.category).push(tagId);
    }
    tag.updated_at = new Date();
  }
  async deleteTag(tagId) {
    const tag = this.tags.get(tagId);
    if (!tag) throw new ValidationError(`Tag ${tagId} not found`);
    if (tag.category) {
      const tagIds = this.category_tags.get(tag.category);
      if (tagIds) {
        const index = tagIds.indexOf(tagId);
        if (index > -1) tagIds.splice(index, 1);
      }
    }
    this.tag_relationships.delete(tagId);
    for (const relationships of this.tag_relationships.values()) {
      const index = relationships.indexOf(tagId);
      if (index > -1) relationships.splice(index, 1);
    }
    this.tags.delete(tagId);
  }
  async addTagRelationship(tagId, relatedTagId) {
    if (!this.tags.has(tagId)) throw new ValidationError(`Tag ${tagId} not found`);
    if (!this.tags.has(relatedTagId)) throw new ValidationError(`Related tag ${relatedTagId} not found`);
    if (!this.tag_relationships.has(tagId)) this.tag_relationships.set(tagId, []);
    this.tag_relationships.get(tagId).push(relatedTagId);
  }
  getRelatedTags(tagId) {
    const related = [];
    const relatedIds = this.tag_relationships.get(tagId);
    if (relatedIds) {
      for (const relatedId of relatedIds) {
        const tag = this.tags.get(relatedId);
        if (tag) related.push(tag);
      }
    }
    return related;
  }
  searchTags(query) {
    const queryLower = query.toLowerCase();
    const results = [];
    for (const tag of this.tags.values()) {
      const nameMatch = tag.name.toLowerCase().includes(queryLower);
      const descriptionMatch = tag.description && tag.description.toLowerCase().includes(queryLower);
      if (nameMatch || descriptionMatch) results.push(tag);
    }
    return results;
  }
  getTagsByCategory(category) {
    const tags = [];
    const tagIds = this.category_tags.get(category);
    if (tagIds) {
      for (const tagId of tagIds) {
        const tag = this.tags.get(tagId);
        if (tag) tags.push(tag);
      }
    }
    return tags;
  }
  getAllCategories() {
    return Array.from(this.category_tags.keys());
  }
  async incrementTagUsage(tagName) {
    for (const tag of this.tags.values()) {
      if (tag.name === tagName) {
        tag.usage_count += 1;
        tag.updated_at = new Date();
        break;
      }
    }
  }
  getMostUsedTags(limit) {
    const tags = Array.from(this.tags.values());
    tags.sort((a, b) => b.usage_count - a.usage_count);
    return tags.slice(0, limit);
  }
  getRecentTags(limit) {
    const tags = Array.from(this.tags.values());
    tags.sort((a, b) => b.created_at - a.created_at);
    return tags.slice(0, limit);
  }
  getTagStatistics() {
    const totalTags = this.tags.size;
    const totalCategories = this.category_tags.size;
    const totalRelationships = Array.from(this.tag_relationships.values()).reduce((sum, rels) => sum + rels.length, 0);
    const averageUsage = totalTags > 0 ? Array.from(this.tags.values()).reduce((sum, tag) => sum + tag.usage_count, 0) / totalTags : 0.0;
    return new TagStatistics({
      total_tags: totalTags,
      total_categories: totalCategories,
      total_relationships: totalRelationships,
      average_usage: averageUsage
    });
  }
  async bulkCreateTags(tagNames, category) {
    const createdIds = [];
    for (const name of tagNames) {
      const tag = new Tag({ name, category, usage_count: 0 });
      const id = await this.createTag(tag);
      createdIds.push(id);
    }
    return createdIds;
  }
  async mergeTags(primaryTagId, duplicateTagIds) {
    const primaryTag = this.tags.get(primaryTagId);
    if (!primaryTag) throw new ValidationError(`Primary tag ${primaryTagId} not found`);
    for (const duplicateId of duplicateTagIds) {
      const duplicateTag = this.tags.get(duplicateId);
      if (duplicateTag) {
        this.tags.delete(duplicateId);
        const primary = this.tags.get(primaryTagId);
        if (primary) {
          primary.usage_count += duplicateTag.usage_count;
          primary.updated_at = new Date();
        }
        const relationships = this.tag_relationships.get(duplicateId);
        if (relationships) {
          this.tag_relationships.delete(duplicateId);
          if (!this.tag_relationships.has(primaryTagId)) this.tag_relationships.set(primaryTagId, []);
          this.tag_relationships.get(primaryTagId).push(...relationships);
        }
      }
    }
  }
}
const TagSortBy = {
  Name: 'Name',
  Usage: 'Usage',
  Created: 'Created',
  Updated: 'Updated'
};
class TagSearchQuery {
  constructor({
    name_contains = null,
    description_contains = null,
    category = null,
    color = null,
    min_usage = null,
    max_usage = null,
    sort_by = TagSortBy.Name,
    limit = null
  } = {}) {
    this.name_contains = name_contains;
    this.description_contains = description_contains;
    this.category = category;
    this.color = color;
    this.min_usage = min_usage;
    this.max_usage = max_usage;
    this.sort_by = sort_by;
    this.limit = limit;
  }
  static default() {
    return new TagSearchQuery();
  }
}
function levenshteinDistance(s1, s2) {
  const s1Chars = [...s1];
  const s2Chars = [...s2];
  const len1 = s1Chars.length;
  const len2 = s2Chars.length;
  const matrix = Array(len1 + 1).fill(null).map(() => Array(len2 + 1).fill(0));
  for (let i = 0; i <= len1; i++) matrix[i][0] = i;
  for (let j = 0; j <= len2; j++) matrix[0][j] = j;
  for (let i = 1; i <= len1; i++) {
    for (let j = 1; j <= len2; j++) {
      const cost = s1Chars[i - 1] === s2Chars[j - 1] ? 0 : 1;
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1,
        matrix[i][j - 1] + 1,
        matrix[i - 1][j - 1] + cost
      );
    }
  }
  return matrix[len1][len2];
}
class TagSearchEngine {
  constructor(tagManager) {
    this.tagManager = tagManager;
  }
  static new(tagManager) {
    return new TagSearchEngine(tagManager);
  }
  advancedSearch(query) {
    let results = Array.from(this.tagManager.tags.values());
    if (query.name_contains) {
      const nameLower = query.name_contains.toLowerCase();
      results = results.filter(tag => tag.name.toLowerCase().includes(nameLower));
    }
    if (query.description_contains) {
      const descLower = query.description_contains.toLowerCase();
      results = results.filter(tag => tag.description && tag.description.toLowerCase().includes(descLower));
    }
    if (query.category) {
      results = results.filter(tag => tag.category === query.category);
    }
    if (query.min_usage !== null) {
      results = results.filter(tag => tag.usage_count >= query.min_usage);
    }
    if (query.max_usage !== null) {
      results = results.filter(tag => tag.usage_count <= query.max_usage);
    }
    if (query.color) {
      results = results.filter(tag => tag.color === query.color);
    }
    switch (query.sort_by) {
      case TagSortBy.Name:
        results.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case TagSortBy.Usage:
        results.sort((a, b) => b.usage_count - a.usage_count);
        break;
      case TagSortBy.Created:
        results.sort((a, b) => b.created_at - a.created_at);
        break;
      case TagSortBy.Updated:
        results.sort((a, b) => b.updated_at - a.updated_at);
        break;
    }
    if (query.limit !== null) {
      results = results.slice(0, query.limit);
    }
    return results;
  }
  fuzzySearch(query, maxDistance) {
    const queryLower = query.toLowerCase();
    const results = [];
    for (const tag of this.tagManager.tags.values()) {
      const nameLower = tag.name.toLowerCase();
      const distance = levenshteinDistance(queryLower, nameLower);
      if (distance <= maxDistance) {
        results.push([tag, distance]);
      }
    }
    results.sort((a, b) => a[1] - b[1]);
    return results;
  }
  getSuggestions(currentTags, limit) {
    const suggestions = new Map();
    for (const tagName of currentTags) {
      const currentTag = this.tagManager.getTagByName(tagName);
      if (currentTag) {
        const relatedTags = this.tagManager.getRelatedTags(currentTag.id);
        for (const relatedTag of relatedTags) {
          const count = suggestions.get(relatedTag.name) || 0;
          suggestions.set(relatedTag.name, count + 1);
        }
      }
    }
    const suggestionList = Array.from(suggestions.entries());
    suggestionList.sort((a, b) => b[1] - a[1]);
    return suggestionList.slice(0, limit).map(([name]) => name);
  }
}

// Extension: BoostingTagSearchEngine
// - Adds category and tag boosts (multipliers) to advancedSearch
// - Adds query-term boosting (tokens receive extra weight)
class BoostingTagSearchEngine extends TagSearchEngine {
  constructor(tagManager, boosts = {}) {
    super(tagManager);
    this.categoryBoosts = boosts.categoryBoosts || {};
    this.tagBoosts = boosts.tagBoosts || {};
    this.termBoosts = boosts.termBoosts || [];
  }
  withCategoryBoost(category, weight = 1.5) {
    this.categoryBoosts[category] = weight;
    return this;
  }
  withTagBoost(tagName, weight = 1.5) {
    this.tagBoosts[tagName] = weight;
    return this;
  }
  withTermBoosts(terms = []) {
    this.termBoosts = terms;
    return this;
  }
  advancedSearch(query) {
    let base = super.advancedSearch(query);
    const tokens = [];
    if (query.name_contains) tokens.push(query.name_contains.toLowerCase());
    if (query.description_contains) tokens.push(query.description_contains.toLowerCase());
    const termBoostSet = new Set(this.termBoosts.map(t => t.toLowerCase()));
    // Score tags based on boosts
    const scored = base.map(tag => {
      let score = 1.0;
      // Category boost
      if (tag.category && this.categoryBoosts[tag.category]) {
        score *= this.categoryBoosts[tag.category];
      }
      // Tag name boost
      if (this.tagBoosts[tag.name]) {
        score *= this.tagBoosts[tag.name];
      }
      // Term boosts: name or description contains boosted term
      const nameLower = tag.name.toLowerCase();
      const descLower = (tag.description || '').toLowerCase();
      for (const term of tokens) {
        if (termBoostSet.has(term)) {
          if (nameLower.includes(term) || descLower.includes(term)) {
            score *= 1.2; // additional term boost
          }
        }
      }
      return { tag, score };
    });
    // Sort by score desc, then name for stable output
    scored.sort((a, b) => {
      if (b.score !== a.score) return b.score - a.score;
      return a.tag.name.localeCompare(b.tag.name);
    });
    return scored.map(s => s.tag);
  }
}

// Demo helpers
function addTagSet(manager, category, names, rng) {
  const colors = ['red', 'green', 'blue', 'yellow', 'purple', 'orange', 'teal', 'pink', 'gray'];
  for (const name of names) {
    const usage = Math.floor(rng() * 50);
    const color = colors[Math.floor(rng() * colors.length)];
    const desc = `Description for ${name} under ${category}`;
    const tag = new Tag({ name, category, color, usage_count: usage, description: desc });
    manager.createTag(tag);
  }
}
function linkTags(manager, aName, bName) {
  const a = manager.getTagByName(aName);
  const b = manager.getTagByName(bName);
  if (a && b) manager.addTagRelationship(a.id, b.id);
}
function usageDemo(manager, names) {
  for (const n of names) manager.incrementTagUsage(n);
}

// Pretty-printers
function printHeader(text) {
  console.log('\n' + '='.repeat(60));
  console.log(text);
  console.log('='.repeat(60));
}
function printTags(title, tags) {
  console.log(`\n${title}:`);
  if (!tags || tags.length === 0) {
    console.log('  (no results)');
    return;
  }
  for (const t of tags) {
    console.log(`  - [${t.category}] ${t.name}  usage=${t.usage_count}  color=${t.color}  desc="${t.description || ''}"`);
  }
}
function printFuzzy(title, results) {
  console.log(`\n${title}:`);
  if (!results || results.length === 0) {
    console.log('  (no results)');
    return;
  }
  for (const [tag, distance] of results) {
    console.log(`  - [${tag.category}] ${tag.name}  (distance=${distance})`);
  }
}

// Main demonstration
async function main() {
  printHeader('Example 5 — TagManager and TagSearchEngine usage & extension');

  const rng = seededRng('example5-seed-42');
  const manager = TagManager.new();
  const engine = TagSearchEngine.new(manager);
  const booster = new BoostingTagSearchEngine(manager)
    .withCategoryBoost('work', 1.6)
    .withTagBoost('urgent', 2.0)
    .withTermBoosts(['bug', 'urgent', 'refactor']);

  // Create demo data
  addTagSet(manager, 'work', [
    'bug', 'feature', 'refactor', 'urgent', 'meeting', 'review', 'release', 'deploy', 'monitoring'
  ], rng);
  addTagSet(manager, 'personal', [
    'reading', 'fitness', 'cooking', 'travel', 'finance', 'learning'
  ], rng);
  addTagSet(manager, 'study', [
    'algorithms', 'database', 'networking', 'security', 'cloud', 'patterns'
  ], rng);

  // Tag relationships for suggestions demo
  linkTags(manager, 'bug', 'urgent');
  linkTags(manager, 'bug', 'deploy');
  linkTags(manager, 'refactor', 'release');
  linkTags(manager, 'feature', 'review');
  linkTags(manager, 'feature', 'release');
  linkTags(manager, 'deploy', 'monitoring');
  linkTags(manager, 'reading', 'learning');
  linkTags(manager, 'fitness', 'health');
  linkTags(manager, 'security', 'network'); // intentional typo to test fuzzy (network vs networking)

  // Usage increments
  usageDemo(manager, ['bug', 'urgent', 'feature', 'release', 'fitness', 'learning']);

  // Statistics
  const stats = manager.getTagStatistics();
  console.log('\nTag Statistics:');
  console.log(`  total_tags = ${stats.total_tags}`);
  console.log(`  total_categories = ${stats.total_categories}`);
  console.log(`  total_relationships = ${stats.total_relationships}`);
  console.log(`  average_usage = ${stats.average_usage.toFixed(2)}`);
  console.log(`  relationships_per_tag = ${stats.relationshipsPerTag().toFixed(2)}`);

  // Basic search
  printTags('Basic search: name contains "bug" (engine.searchTags)', manager.searchTags('bug'));

  // Advanced search
  const q1 = new TagSearchQuery({
    name_contains: 're',
    category: null,
    sort_by: TagSortBy.Usage,
    limit: 6
  });
  printTags('Advanced search: name contains "re" sorted by usage (top 6)', engine.advancedSearch(q1));

  // Category filter
  const q2 = TagSearchQuery.default();
  q2.category = 'work';
  q2.sort_by = TagSortBy.Name;
  printTags('Advanced search: category="work" sorted by name', engine.advancedSearch(q2));

  // Fuzzy search
  printFuzzy('Fuzzy search: "netwrk" within distance 2', engine.fuzzySearch('netwrk', 2));

  // Boosting search (extension)
  const q3 = TagSearchQuery.default();
  q3.name_contains = 're';
  q3.limit = 6;
  printTags('Boosting search: name contains "re" with category/tag boosts', booster.advancedSearch(q3));

  // Suggestions
  const suggestions = engine.getSuggestions(['bug', 'feature'], 6);
  console.log(`\nTag suggestions based on ["bug", "feature"]: ${suggestions.join(', ') || '(none)'}`);

  // Most used / Recent
  printTags('Most used tags (top 5)', manager.getMostUsedTags(5));
  printTags('Recent tags (top 5)', manager.getRecentTags(5));

  printHeader('End of Example 5');
}

main().catch(err => {
  console.error('Error running example:', err);
  process.exit(1);
});
