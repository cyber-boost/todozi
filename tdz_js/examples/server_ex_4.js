import { TodoziEmbeddingService, TodoziEmbeddingConfig, TodoziContentType, SimilarityResult, TodoziEmbeddingCache, EmbeddingModel } from '../todozi/emb.js';
import { v4 as uuidv4 } from 'uuid';
import { DateTime } from 'luxon';
import { Storage } from '../todozi/storage.js';
import { TagManager, TagSearchEngine } from '../todozi/tags.js';

/**

 * Example 4 – Semantic Todozi Assistant
 *
 * Demonstrates how to:
 *   • Load the Todozi storage layer
 *   • Create sample tasks / memories / ideas
 *   • Initialise the embedding service (emb.js)
 *   • Index the data
 *   • Run pure‑semantic, hybrid and tag‑suggestion queries
 *
 * Run with:
 *   $ node example4-semantic-assistant.js
 *
 * Prerequisite:
 *   • `npm install uuid luxon` (the rest of the dependencies are already used by the
 *     Todozi source files)
 */

'use strict';

// ---------------------------------------------------------------------------
// 2️⃣  Imports – everything we need from the existing code base
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// 3️⃣  Helper – pretty‑print a list of results
// ---------------------------------------------------------------------------
function printResults(title, items) {
  console.log(`\n=== ${title} (${items.length}) ===`);
  if (items.length === 0) {
    console.log('  (no matches)');
    return;
  }
  items.forEach((it, idx) => {
    // `it` is a SimilarityResult (see emb.js)
    const excerpt = it.text_content.length > 80
      ? it.text_content.slice(0, 77) + '…'
      : it.text_content;
    console.log(
      `${idx + 1}. [${it.content_type}] score=${(it.similarity_score * 100).toFixed(1)}%`,
      `\n   ${excerpt}`
    );
  });
}

// ---------------------------------------------------------------------------
// 4️⃣  Main async function – everything runs here
// ---------------------------------------------------------------------------
(async () => {
  // -----------------------------------------------------------------------
  // 4.1️⃣ Load (or create) the Todozi storage directory
  // -----------------------------------------------------------------------
  const storage = await Storage.new(); // reads ~/.todozi/... or creates defaults
  console.log('📦 Storage ready');

  // -----------------------------------------------------------------------
  // 4.2️⃣ Create a few sample objects (tasks, memories, ideas)
  // -----------------------------------------------------------------------
  const sampleData = [
    {
      type: TodoziContentType.Task,
      obj: {
        id: uuidv4(),
        user_id: 'example_user',
        action: 'Write a blog post about semantic search in Todozi',
        time: '2h',
        priority: 'high',
        parent_project: 'writing',
        status: 'todo',
        tags: ['blog', 'search'],
        created_at: DateTime.utc().toISO(),
        updated_at: DateTime.utc().toISO(),
      },
    },
    {
      type: TodoziContentType.Task,
      obj: {
        id: uuidv4(),
        user_id: 'example_user',
        action: 'Refactor the authentication middleware',
        time: '4h',
        priority: 'critical',
        parent_project: 'backend',
        status: 'in_progress',
        tags: ['backend', 'security'],
        created_at: DateTime.utc().toISO(),
        updated_at: DateTime.utc().toISO(),
      },
    },
    {
      type: TodoziContentType.Memory,
      obj: {
        id: uuidv4(),
        user_id: 'example_user',
        project_id: null,
        status: 'active',
        moment: '2025‑04‑01 09:15',
        meaning: 'Discovered a nice trick for fast vector similarity using dot‑product',
        reason: 'Performance optimisation research',
        importance: 'high',
        term: 'short',
        memory_type: 'standard',
        tags: ['performance', 'vectors'],
        created_at: DateTime.utc().toISO(),
        updated_at: DateTime.utc().toISO(),
      },
    },
    {
      type: TodoziContentType.Idea,
      obj: {
        id: uuidv4(),
        idea: 'Add a visual graph view of task dependencies',
        project_id: null,
        status: 'active',
        share: 'team',
        importance: 'high',
        tags: ['ui', 'graph'],
        created_at: DateTime.utc().toISO(),
        updated_at: DateTime.utc().toISO(),
      },
    },
  ];

  // Store each object in the proper collection (tasks → storage, memories → storage, …)
  for (const { type, obj } of sampleData) {
    switch (type) {
      case TodoziContentType.Task:
        await storage.addTaskToProject(obj);
        console.log(`🗒️  Saved task "${obj.action}"`);
        break;
      case TodoziContentType.Memory:
        // In the real code you would have a `storage.addMemory` – here we just
        // demonstrate the embedding step, so we skip persistence.
        console.log(`🧠 Saved memory "${obj.meaning}"`);
        break;
      case TodoziContentType.Idea:
        // Same comment as above – we only need the object for embedding.
        console.log(`💡 Saved idea "${obj.idea}"`);
        break;
      default:
        console.warn('⚠️  Unknown content type');
    }
  }

  // -----------------------------------------------------------------------
  // 4.3️⃣ Initialise the embedding service
  // -----------------------------------------------------------------------
  const embConfig = TodoziEmbeddingConfig.default();   // default model, thresholds…
  const embService = await TodoziEmbeddingService.new(embConfig);
  await embService.initialize(); // loads the default sentence‑transformer model
  // Note: EmbeddingModel.getDefaultModel() would be called if available
  console.log('🧩 Embedding service initialised');

  // -----------------------------------------------------------------------
  // 4.4️⃣ Index every object we just created
  // -----------------------------------------------------------------------
  // The embedding service works with *plain text* representations.  We reuse the
  // helper methods that already exist in `emb.js` to generate that text.
  for (const { type, obj } of sampleData) {
    let text;
    switch (type) {
      case TodoziContentType.Task:
        text = `Task: ${obj.action}
Priority: ${obj.priority}
Project: ${obj.parent_project}
Tags: ${obj.tags.join(', ')}`;
        break;
      case TodoziContentType.Memory:
        text = `Memory: ${obj.meaning}
Context: ${obj.reason}
Importance: ${obj.importance}
Term: ${obj.term}`;
        break;
      case TodoziContentType.Idea:
        text = `Idea: ${obj.idea}
Importance: ${obj.importance}
Share: ${obj.share}`;
        break;
      default:
        text = JSON.stringify(obj, null, 2);
    }
    // Store the vector in the service cache (the key is `${type}_${id}`)
    const cacheKey = `${type}_${obj.id}`;
    const vector = await embService.generateEmbedding(text);
    const cacheEntry = new TodoziEmbeddingCache(
      vector,
      type,
      obj.id,
      text,
      obj.tags || [],
      new Date(),
      embConfig.cache_ttl_seconds
    );
    embService.cache.set(cacheKey, cacheEntry);
    console.log(`🔎 Indexed ${type.toLowerCase()} "${obj.id}"`);
  }

  // -----------------------------------------------------------------------
  // 4.5️⃣ **Pure semantic search**
  // -----------------------------------------------------------------------
  const semanticQuery = 'I need to improve the speed of my search feature';
  const semanticResults = await embService.semanticSearch(
    semanticQuery,
    // Search *all* content types – you could restrict to [TodoziContentType.Task]
    [TodoziContentType.Task, TodoziContentType.Memory, TodoziContentType.Idea],
    5
  );
  printResults('Pure semantic search', semanticResults);

  // -----------------------------------------------------------------------
  // 4.6️⃣ **Hybrid search** – mix keywords and embeddings
  // -----------------------------------------------------------------------
  const hybridQuery = 'graph view for tasks';
  const keywords = ['graph', 'visual', 'dependencies'];
  const hybridResults = await embService.hybridSearch(
    hybridQuery,
    keywords,
    // Again we search all types, but you can limit it
    [TodoziContentType.Task, TodoziContentType.Idea],
    0.6,   // give 60 % weight to the embeddings, 40 % to keyword match
    5
  );
  printResults('Hybrid search (semantic + keywords)', hybridResults);

  // -----------------------------------------------------------------------
  // 4.7️⃣ **Tag suggestions** – ask the model which tags would fit an item
  // -----------------------------------------------------------------------
  // Pick the first task we stored and request tag suggestions.
  const firstTask = sampleData.find(i => i.type === TodoziContentType.Task);
  const suggestedTags = await embService.suggestTags(firstTask.obj.id, 5);
  console.log('\n=== Tag suggestions for the first task ===');
  console.log('🔖 Suggested tags:', suggestedTags.join(', ') || '(none)');

  // -----------------------------------------------------------------------
  // 4.8️⃣ (Optional) Use the TagManager for a quick lookup / creation demo
  // -----------------------------------------------------------------------
  const tagMgr = TagManager.new();
  // Create some tags – they will be stored only in memory for this demo
  await tagMgr.createTag({ name: 'search', description: 'Anything about search' });
  await tagMgr.createTag({ name: 'performance', description: 'Speed‑related items' });
  await tagMgr.createTag({ name: 'ui', description: 'User interface' });

  // Look‑up a tag by name
  const tag = tagMgr.getTagByName('search');
  console.log('\n🔎 Tag lookup: ', tag ? tag.id + ' → ' + tag.description : 'not found');

  // Get tag suggestions based on the tags we just created
  const tagSugEngine = new TagSearchEngine(tagMgr);
  const suggestions = tagSugEngine.getSuggestions(['search', 'ui'], 3);
  console.log('💡 Tag‑search suggestions based on “search, ui”:', suggestions.join(', ') || '(none)');

  // -----------------------------------------------------------------------
  // 5️⃣  Done
  // -----------------------------------------------------------------------
  console.log('\n✅ Example finished – you now have a working end‑to‑end semantic assistant!\n');
})();

/*
## 🎯 Example 4 – “Semantic Todozi Assistant”

**What the example does**

1. **Boot‑strap the Todozi storage** – loads the on‑disk storage that the server‑side code already uses.  
2. **Creates a few sample tasks, memories and ideas** – each object is saved with the normal Todozi schema.  
3. **Initialises the `TodoziEmbeddingService`** (the AI‑powered embedding layer from **`emb.js`**) and adds the new objects to the embedding index.  
4. **Runs three different kinds of searches**  
   * *pure semantic search* – find items that “feel like” the query.  
   * *hybrid search* – combine keyword matching with semantic similarity.  
   * *tag‑suggestion* – ask the embedding service which tags would be useful for a given item.  
5. **Prints the results in a readable table** so you can see how the whole pipeline works end‑to‑end.

---

### 📂 Project layout (only the files we touch)

/ *
your‑project/
│
├─ server.js          ← unchanged (the core HTTP server)
├─ storage.js         ← unchanged (file‑system storage helpers)
├─ emb.js             ← unchanged (embedding service)
├─ tags.js            ← unchanged (tag manager)
│
└─ example4‑semantic‑assistant.js   ← **the new example**

/ *
---

## 🧾 1️⃣  Full source – `example4-semantic-assistant.js`

---

## 📖 2️⃣  Walk‑through of the example  

| Step | What the code does | Why it matters |
|------|-------------------|----------------|
| **1️⃣ Imports** | Pulls in `Storage`, `TodoziEmbeddingService`, `TagManager`, `uuid`, `luxon` | All the public APIs we need are already exported by the original files – no extra dependency needed. |
| **2️⃣ Helper `printResults`** | Nicely formats `SimilarityResult` objects | Makes the console output readable for humans. |
| **3️⃣ Load storage** | `await Storage.new()` reads `~/.todozi/…` or creates a fresh layout | Guarantees the same directory layout the server expects. |
| **4️⃣ Create sample data** | Builds a tiny knowledge‑base (2 tasks, 1 memory, 1 idea) using the same shape the server would receive via its REST API. | Shows that the embedding layer can handle any Todozi content type. |
| **5️⃣ Initialise the embedding service** | `TodoziEmbeddingService.new()` → loads the default sentence‑transformer model (`all‑MiniLM‑L6‑v2`). | The heavy lifting (tokeniser + model) is lazy‑loaded only once. |
| **6️⃣ Index objects** | Turns each object into a deterministic text blob (using the same helpers the server uses) → generates a 384‑dim vector → stores it in the service’s in‑memory cache. | After this step the objects are “searchable”. |
| **7️⃣ Pure semantic search** | `semanticSearch(query, types, limit)` returns the most similar items based only on cosine similarity. | Perfect for “find everything that feels like X”. |
| **8️⃣ Hybrid search** | `hybridSearch(query, keywords, types, semanticWeight, limit)` mixes keyword matches with the embedding similarity. | Gives you the best of both worlds – you can boost exact word matches when you need them. |
| **9️⃣ Tag‑suggestion** | `suggestTags(contentId, topK)` returns the most frequent tags among the nearest neighbours. | Helps users keep their tag vocabulary consistent without manual browsing. |
| **🔟 Optional TagManager demo** | Shows how to create tags, fetch a tag by name and run the `TagSearchEngine` to get tag suggestions based on a set of seed tags. | Demonstrates the *stand‑alone* tag subsystem that the server also uses. |

---

## 🚀 3️⃣  How to run the example

# 1️⃣ Install the two tiny runtime deps that are not already in the repo
npm install uuid luxon

# 2️⃣ Execute
node example4-semantic-assistant.js

You should see output similar to:
*/