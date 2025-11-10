/**
import { Tag, TagUpdate, TagManager, TagSearchEngine, TagSearchQuery, TagSortBy, ValidationError, } from '../todozi/tags.js';

 * tag-demo.js
 *
 * Demonstrates the full tag‑workflow:
 *   • Create individual tags
 *   • Bulk‑create a set of tags belonging to a category
 *   • Update / delete a tag
 *   • Add relationships (e.g. “frontend” → “react”)
 *   • Perform advanced, fuzzy and suggestion searches
 *   • Merge duplicate tags (e.g. “js” & “javascript”)
 *
 * Run:
 *   node tag-demo.js
 */

// ---------------------------------------------------------------------
// Helper: pretty‑print a tag (or a list of tags)
// ---------------------------------------------------------------------
function printTag(tag) {
  console.log(
    `- ${tag.id.slice(0, 8)} | ${tag.name}` +
    (tag.category ? `  [cat: ${tag.category}]` : '') +
    (tag.usage_count ? `  (used ${tag.usage_count}×)` : '')
  );
}

// ---------------------------------------------------------------------
// Main async driver
// ---------------------------------------------------------------------
(async () => {
  // -----------------------------------------------------------------
  // 1️⃣ Initialise a TagManager (in‑memory – no persistence)
  // -----------------------------------------------------------------
  const manager = TagManager.new();

  // -----------------------------------------------------------------
  // 2️⃣ Create a few single tags
  // -----------------------------------------------------------------
  const tagReact = await manager.createTag(
    new Tag({ name: 'react', description: 'React UI library', category: 'frontend' })
  );
  const tagNode = await manager.createTag(
    new Tag({ name: 'node', description: 'Node.js runtime', category: 'backend' })
  );

  console.log('\n✅ Created two tags:');
  printTag(manager.getTag(tagReact));
  printTag(manager.getTag(tagNode));

  // -----------------------------------------------------------------
  // 3️⃣ Bulk‑create a set of language tags under the same category
  // -----------------------------------------------------------------
  const languages = ['javascript', 'typescript', 'python', 'go', 'rust'];
  const bulkIds = await manager.bulkCreateTags(languages, 'language');

  console.log('\n✅ Bulk‑created language tags:');
  bulkIds.forEach(id => printTag(manager.getTag(id)));

  // -----------------------------------------------------------------
  // 4️⃣ Add relationships – e.g. “frontend” → “react”, “backend” → “node”
  // -----------------------------------------------------------------
  // First we need the “frontend” and “backend” tags; they don't exist yet,
  // so we create them (they will automatically be added to the category map).
  const tagFrontend = await manager.createTag(new Tag({ name: 'frontend' }));
  const tagBackend  = await manager.createTag(new Tag({ name: 'backend' }));

  // Connect the categories with the concrete tags
  await manager.addTagRelationship(tagFrontend, tagReact);
  await manager.addTagRelationship(tagBackend,  tagNode);

  console.log('\n✅ Added relationships:');
  console.log('Frontend →', manager.getRelatedTags(tagFrontend).map(t => t.name));
  console.log('Backend  →', manager.getRelatedTags(tagBackend).map(t => t.name));

  // -----------------------------------------------------------------
  // 5️⃣ Increment usage counts (simulating “tagging” tasks)
  // -----------------------------------------------------------------
  await manager.incrementTagUsage('react');
  await manager.incrementTagUsage('react');
  await manager.incrementTagUsage('javascript');

  console.log('\n✅ Updated usage counters:');
  console.log('react      usage →', manager.getTagByName('react').usage_count);
  console.log('javascript usage →', manager.getTagByName('javascript').usage_count);

  // -----------------------------------------------------------------
  // 6️⃣ Advanced search – all tags that contain “js” in the name,
  //    belong to the “language” category and have at least 1 usage.
  // -----------------------------------------------------------------
  const advQuery = new TagSearchQuery({
    name_contains: 'js',
    category: 'language',
    min_usage: 1,
    sort_by: TagSortBy.Usage,   // most used first
    limit: 5,
  });

  const found = new TagSearchEngine(manager).advancedSearch(advQuery);
  console.log('\n🔎 Advanced search results (name contains “js”, usage ≥1):');
  found.forEach(printTag);

  // -----------------------------------------------------------------
  // 7️⃣ Fuzzy search – tolerate up to 2 edit‑distance errors.
  //    Look for tags similar to “javascrip” (typo on purpose).
  // -----------------------------------------------------------------
  const fuzzy = new TagSearchEngine(manager).fuzzySearch('javascrip', 2);
  console.log('\n🔎 Fuzzy‑search results for “javascrip” (max distance = 2):');
  fuzzy.forEach(([tag, distance]) => {
    console.log(`- ${tag.name} (distance: ${distance})`);
  });

  // -----------------------------------------------------------------
  // 8️⃣ Tag suggestions – given a set of already‑used tags,
  //    propose the most common related tags.
  // -----------------------------------------------------------------
  const current = ['frontend', 'react'];   // imagine a task already has these tags
  const suggestions = new TagSearchEngine(manager).getSuggestions(current, 3);
  console.log('\n💡 Tag suggestions for', current.join(', '), ':');
  console.log(suggestions.join(', '));

  // -----------------------------------------------------------------
  // 9️⃣ Updating a tag (change description & category)
  // -----------------------------------------------------------------
  const upd = TagUpdate.new()
    .description('Node.js – JavaScript runtime for server side')
    .category('runtime');

  await manager.updateTag(tagNode, upd);
  console.log('\n✅ Updated “node” tag:');
  printTag(manager.getTag(tagNode));

  // -----------------------------------------------------------------
  // 🔟 Merge duplicate tags – imagine “js” and “javascript” refer to the same thing.
  //    Keep “javascript” as the primary tag.
  // -----------------------------------------------------------------
  // First create a short “js” tag.
  const tagJs = await manager.createTag(new Tag({ name: 'js', usage_count: 5 }));
  // Add relationships from “js” to the language tags as an example.
  await manager.addTagRelationship(tagJs, manager.getTagByName('javascript').id);

  console.log('\nBefore merge:');
  console.log('js usage →', manager.getTagByName('js').usage_count);
  console.log('javascript usage →', manager.getTagByName('javascript').usage_count);
  console.log('js related →', manager.getRelatedTags(tagJs).map(t => t.name));

  // Merge: primary = “javascript”, duplicate = [“js”]
  await manager.mergeTags(manager.getTagByName('javascript').id, [tagJs]);

  console.log('\n✅ After merge (javascript absorbs js):');
  console.log('javascript usage →', manager.getTagByName('javascript').usage_count);
  console.log('Related to javascript →',
    manager.getRelatedTags(manager.getTagByName('javascript').id).map(t => t.name));

  // -----------------------------------------------------------------
  // 11️⃣  Show overall tag statistics
  // -----------------------------------------------------------------
  const stats = manager.getTagStatistics();
  console.log('\n📊 Tag statistics:');
  console.log(`Total tags          : ${stats.total_tags}`);
  console.log(`Total categories    : ${stats.total_categories}`);
  console.log(`Total relationships : ${stats.total_relationships}`);
  console.log(`Average usage/count : ${stats.average_usage.toFixed(2)}`);
  console.log(`Relationships/tag   : ${stats.relationshipsPerTag().toFixed(2)}`);

  // -----------------------------------------------------------------
  // 12️⃣ Delete a tag (e.g. the temporary “js” – already merged, now gone)
  // -----------------------------------------------------------------
  try {
    await manager.deleteTag(tagJs);   // will throw because it no longer exists
  } catch (e) {
    if (e instanceof ValidationError) {
      console.log('\n⚠️  Tag “js” already removed by merge – nothing to delete.');
    } else {
      throw e;
    }
  }

  console.log('\n🚀 Demo completed!\n');
})();

/*
## Example 4 – Full‑Featured Tag Management & Search  
*Shows how to:*

1. **Create, update & delete tags** (including bulk creation).  
2. **Add relationships** between tags.  
3. **Run advanced, fuzzy and suggestion searches** with `TagSearchEngine`.  
4. **Merge duplicate tags** while preserving usage‑counts and relationships.  

The script can be dropped into any Todozi project (or run as a standalone Node file) because it only depends on the `tags.js` module that ships with Todozi.

---  

### 1️⃣  Install the required dependency  

`tags.js` uses the `uuid` package – make sure it is present in your project:

/ *
bash
npm i uuid

/ *
---  

### 2️⃣  The example script (`tag‑demo.js`)

---  

### 3️⃣  What the script does (step‑by‑step)

| Step | Action | Key API used |
|------|--------|--------------|
| 1 | Initialise `TagManager` | `TagManager.new()` |
| 2 | Create two explicit tags | `manager.createTag(new Tag(...))` |
| 3 | Bulk‑create many tags in one go | `manager.bulkCreateTags(tagNames, category)` |
| 4 | Add directed relationships | `manager.addTagRelationship(parentId, childId)` |
| 5 | Simulate “using” a tag (increment usage) | `manager.incrementTagUsage(tagName)` |
| 6 | Run an **advanced** filtered, sorted search | `new TagSearchEngine(manager).advancedSearch(query)` |
| 7 | Perform a **fuzzy** search (Levenshtein distance) | `new TagSearchEngine(manager).fuzzySearch(query, maxDist)` |
| 8 | Get **suggested** tags based on already‑present tags | `new TagSearchEngine(manager).getSuggestions(currentTags, limit)` |
| 9 | Update a tag’s metadata | `manager.updateTag(tagId, TagUpdate.new().description(...).category(...))` |
| 10 | **Merge** duplicate tags – usage & relationships are combined | `manager.mergeTags(primaryId, [duplicateId])` |
| 11 | Gather overall statistics | `manager.getTagStatistics()` |
| 12 | Attempt to delete a now‑non‑existent tag (shows error handling) | `manager.deleteTag(tagId)` |

All operations are **asynchronous** (`await`) because real Todozi storage‑backends may involve file‑IO or a database. In this demo the data lives only in memory, which makes it fast and safe to run repeatedly.

---  

### 4️⃣  Run the demo

node tag-demo.js

You should see a nicely formatted console output similar to:
*/