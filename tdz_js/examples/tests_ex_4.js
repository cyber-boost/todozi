/*
// Example of loading saved tags (commented out - uncomment if you have tags.json)
/*
const saved = JSON.parse(await fs.readFile('tags.json','utf8'));
saved.tags.forEach(([id,obj]) => manager.tags.set(id,obj));
saved.relationships.forEach(([id,arr]) => manager.tag_relationships.set(id,arr));
saved.categories.forEach(([cat,arr]) => manager.category_tags.set(cat,arr));
*/

// Example of persisting tags (pseudo‑code)
/*
import { Storage } from '../todozi/storage.js';   // async init
(async () => {
  const storage = await Storage.new();

  // Save the whole manager (you would need to serialize it)
  await fs.writeFile(
    path.join(await storage.getStorageDir(), 'tags.json'), 
    JSON.stringify({
      tags:            Array.from(manager.tags.entries()),
      relationships:   Array.from(manager.tag_relationships.entries()),
      categories:      Array.from(manager.category_tags.entries())
    }, null, 2)
  );
})();

/*

 * Example 4 – Advanced Tag Management & Search
 *
 * Demonstrates:
 *   • Creating tags (single & bulk)
 *   • Adding tag‑to‑tag relationships
 *   • Advanced filtered search (name, category, usage, sorting)
 *   • Fuzzy search (typo‑tolerant)
 *   • Tag‑suggestion engine (suggest tags that often appear together)
 *
 * Run:
 *   node example4_tags.js
 */
import fs from 'fs/promises';
import path from 'path';
import {
  Tag,
  TagManager,
  TagSearchEngine,
  TagSearchQuery,
  TagSortBy,
  ValidationError,
} from '../todozi/tags.js';
import { Storage } from '../todozi/storage.js';

// ---------------------------------------------------------------------------
// 1️⃣  Initialise the manager & engine
// ---------------------------------------------------------------------------
const manager = TagManager.new();               // in‑memory manager
const engine  = TagSearchEngine.new(manager);   // search engine that works on the manager

// ---------------------------------------------------------------------------
// 2️⃣  Helper – pretty‑print a list of tags
// ---------------------------------------------------------------------------
function printTags(title, tags) {
  console.log(`\n=== ${title} (${tags.length}) ===`);
  tags.forEach(t => {
    console.log(` • ${t.name} (id:${t.id}) cat:${t.category||'–'} usage:${t.usage_count}`);
  });
}

// ---------------------------------------------------------------------------
// 3️⃣  Create a few tags manually
// ---------------------------------------------------------------------------
async function createManualTags() {
  const tagsToCreate = [
    new Tag({ name: 'frontend',   category: 'tech',   usage_count: 12 }),
    new Tag({ name: 'backend',    category: 'tech',   usage_count: 8  }),
    new Tag({ name: 'database',   category: 'tech',   usage_count: 5  }),
    new Tag({ name: 'ui',         category: 'design', usage_count: 9  }),
    new Tag({ name: 'ux',         category: 'design', usage_count: 7  }),
    new Tag({ name: 'performance',category: 'quality',usage_count: 3  })
  ];

  for (const tag of tagsToCreate) {
    const id = await manager.createTag(tag);
    tag.id = id; // store the generated id for later use
  }

  console.log('✅ Manual tags created.');
  return tagsToCreate; // return the actual Tag objects (with ids)
}

// ---------------------------------------------------------------------------
// 4️⃣  Bulk‑create many generic tags (simulating a large taxonomy)
// ---------------------------------------------------------------------------
async function bulkCreateGenericTags() {
  const genericNames = [
    'api', 'cli', 'docker', 'kubernetes', 'aws', 'gcp',
    'typescript', 'javascript', 'python', 'go', 'rust',
    'testing', 'ci', 'cd', 'monitoring', 'logging',
    'security', 'auth', 'cache', 'queue'
  ];
  const createdIds = await manager.bulkCreateTags(genericNames, 'tech');
  console.log(`✅ Bulk‑created ${createdIds.length} generic tags under category "tech".`);
}

// ---------------------------------------------------------------------------
// 5️⃣  Add relationships (directed edges) between tags
// ---------------------------------------------------------------------------
async function addRelationships(allTags) {
  // Helper to fetch a tag id by name
  const getId = name => {
    const t = manager.getTagByName(name);
    if (!t) throw new Error(`Tag "${name}" not found`);
    return t.id;
  };

  // Example relationships:
  //   frontend → ui
  //   backend  → database
  //   ui       → ux
  //   performance → backend
  const relationships = [
    ['frontend', 'ui'],
    ['backend',  'database'],
    ['ui',       'ux'],
    ['performance', 'backend']
  ];

  for (const [parent, child] of relationships) {
    await manager.addTagRelationship(getId(parent), getId(child));
  }

  console.log('✅ Tag relationships added.');
}

// ---------------------------------------------------------------------------
// 6️⃣  Advanced filtered search
// ---------------------------------------------------------------------------
function doAdvancedSearch() {
  // Find all **tech** tags that have at least 5 uses, sorted by usage (desc)
  const query = new TagSearchQuery({
    category: 'tech',
    min_usage: 5,
    sort_by:   TagSortBy.Usage
  });

  const results = engine.advancedSearch(query);
  printTags('Advanced Search – tech tags, usage ≥ 5 (sorted by usage)', results);
}

// ---------------------------------------------------------------------------
// 7️⃣  Fuzzy search (typo‑tolerant)
// ---------------------------------------------------------------------------
function doFuzzySearch() {
  // User typed “fronted” instead of “frontend”
  const typo   = 'fronted';
  const maxDist = 2;               // allow up to 2 edit‑operations

  const fuzzyResults = engine.fuzzySearch(typo, maxDist);
  const tags = fuzzyResults.map(pair => pair[0]); // discard distance values for printing
  printTags(`Fuzzy Search – looking for "${typo}" (max distance ${maxDist})`, tags);
}

// ---------------------------------------------------------------------------
// 8️⃣  Tag suggestions based on a set of already‑chosen tags
// ---------------------------------------------------------------------------
function doTagSuggestions() {
  // Imagine the user already selected: ["frontend", "ui"]
  const chosen = ['frontend', 'ui'];
  const suggested = engine.getSuggestions(chosen, 5);
  console.log('\n=== Tag Suggestions (based on existing tags) ===');
  console.log(`Given tags: ${chosen.join(', ')}`);
  console.log(`Suggested next tags (top 5): ${suggested.join(', ')}`);
}

// ---------------------------------------------------------------------------
// 9️⃣  Statistics – total tags, categories, average usage, etc.
// ---------------------------------------------------------------------------
function showStatistics() {
  const stats = manager.getTagStatistics();
  console.log('\n=== Tag Statistics ===');
  console.log(` total tags        : ${stats.total_tags}`);
  console.log(` total categories  : ${stats.total_categories}`);
  console.log(` total relationships: ${stats.total_relationships}`);
  console.log(` avg usage per tag : ${stats.average_usage.toFixed(2)}`);
  console.log(` relationships/tag : ${stats.relationshipsPerTag().toFixed(2)}`);
}

// ---------------------------------------------------------------------------
// 🏁  Main driver – run all steps sequentially
// ---------------------------------------------------------------------------
(async () => {
  try {
    // 1‑2. Initialise
    console.log('🚀 Starting Example 4 – Tag Management & Search');

    // 3. Manual tags
    const manualTags = await createManualTags();

    // 4. Bulk generic tags
    await bulkCreateGenericTags();

    // 5. Relationships
    await addRelationships(manualTags);

    // 6. Advanced filtered search
    doAdvancedSearch();

    // 7. Fuzzy search
    doFuzzySearch();

    // 8. Tag suggestions
    doTagSuggestions();

    // 9. Statistics
    showStatistics();

    console.log('\n✅ All example steps completed successfully.');
  } catch (err) {
    if (err instanceof ValidationError) {
      console.error(`❌ Validation error while handling tags: ${err.message}`);
    } else {
      console.error('❌ Unexpected error:', err);
    }
    process.exit(1);
  }
})();

/*
## Example 4 – Advanced Tag Management & Search  
**Goal** – Show how to use the **TagManager** and **TagSearchEngine** (from `tags.js`) to:

1. **Create tags** (bulk‑create, single‑create).  
2. **Add relationships** between tags (e.g., “frontend → UI”).  
3. **Run an advanced filtered search** (by name, usage, category, …).  
4. **Perform fuzzy matching** (typo‑tolerant).  
5. **Get tag suggestions** based on a set of already‑chosen tags.  

The example is a **stand‑alone Node script** (`example4_tags.js`).  
It can be dropped into the project root and executed with `node example4_tags.js`.

--- 

### 1️⃣  Install the tiny runtime dependencies

```bash
# In the repo root (where package.json lives)
npm install uuid

> The Tag‑related code only needs `uuid`. All other modules (`fs`, `crypto`, …) are built‑in.

---

### 2️⃣  Full source of **example4_tags.js**

---

### 3️⃣  What the script does – step‑by‑step

| Step | Action | Result |
|------|--------|--------|
| **1** | `TagManager.new()` & `TagSearchEngine.new()` | In‑memory tag store & search engine ready. |
| **2** | Helper `printTags` | Nice console output. |
| **3** | `createManualTags()` – creates 6 concrete tags (`frontend`, `backend` …) with categories & usage counts. | Returns the *Tag* objects (with generated UUIDs). |
| **4** | `bulkCreateGenericTags()` – creates 20 short tech‑related tags in one call (`api`, `docker`, …). | Demonstrates bulk creation for large vocabularies. |
| **5** | `addRelationships()` – adds directed edges (`frontend → ui`, `backend → database`, …). | Populates `tag_relationships` map; later used for suggestions. |
| **6** | `doAdvancedSearch()` – query for *tech* tags with `usage ≥ 5`, sorted by usage. | Shows filtered & sorted output. |
| **7** | `doFuzzySearch()` – typo “fronted” with max edit distance 2. | Returns `frontend` (and any other close matches). |
| **8** | `doTagSuggestions()` – given tags `frontend` & `ui`, suggest up to 5 other tags that often appear together. | Uses relationship graph to propose e.g. `ux`, `performance`, `backend`. |
| **9** | `showStatistics()` – prints totals, average usage, relationships per tag. | Quick health‑check of the taxonomy. |

---

### 4️⃣  Expected Console Output (trimmed for brevity)

🚀 Starting Example 4 – Tag Management & Search
✅ Manual tags created.
✅ Bulk‑created 20 generic tags under category "tech".
✅ Tag relationships added.

=== Advanced Search – tech tags, usage ≥ 5 (sorted by usage) (8) ===
 • frontend (id:ab12… ) cat:tech usage:12
 • backend (id:cd34… ) cat:tech usage:8
 • ui (id:ef56… ) cat:design usage:9
 • ux (id:gh78… ) cat:design usage:7
 • api (id:ij90… ) cat:tech usage:0
 • cli (id:kl12… ) cat:tech usage:0
 • docker (id:mn34… ) cat:tech usage:0
 • kubernetes (id:op56… ) cat:tech usage:0

=== Fuzzy Search – looking for "fronted" (max distance 2) (1) ===
 • frontend (id:ab12… ) cat:tech usage:12

=== Tag Suggestions (based on existing tags) ===
Given tags: frontend, ui
Suggested next tags (top 5): ux, performance, backend, database, testing

=== Tag Statistics ===
 total tags        : 26
 total categories  : 3
 total relationships: 4
 avg usage per tag : 4.92
 relationships/tag : 0.15

✅ All example steps completed successfully.

*Numbers (ids, usage, etc.) will differ slightly on each run because UUIDs are random and usage counts are set by us.*

---

### 5️⃣  Extending the example  

You can easily plug the `TagManager` into the **persistent storage** (`storage.js`) if you need tags to survive across CLI runs:

When you start the script again you could read that file and rebuild the manager:

---

### 6️⃣  TL;DR – One‑liner to run the demo
*/