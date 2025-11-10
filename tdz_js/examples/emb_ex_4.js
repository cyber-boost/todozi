import { TodoziEmbeddingService, TodoziEmbeddingConfig } from './emb.js';
import { Tag, TagManager } from './tags.js';
import { Storage } from './storage.js';
import { Task } from './models.js';

(async () => {
  const storage = await Storage.new();
  const tagMgr  = TagManager.new();
  const tagId   = await tagMgr.createTag(new Tag({ name: 'backend' }));
  const embSrv  = await TodoziEmbeddingService.new(TodoziEmbeddingConfig.default());
  await embSrv.embedTag(tagMgr.getTag(tagId));

  const task = Task.newFull('cli_user','Fix API auth','1h','high','backend','todo');
  await storage.addTaskToProject(task);
  const results = await embSrv.hybridSearch('auth backend', ['auth','backend']);
  console.log(results);
})();

/*
// ----------------------------------------------------------
// Example 4 – Advanced Todozi workflow
// ----------------------------------------------------------

/*  ────────────────────────────────────────────────────────
    What we are going to do:

    1️⃣  Initialise the storage layer (creates ~/.todozi if it does not exist)
    2️⃣  Build a TagManager and add a couple of tags.
    3️⃣  Initialise TodoziEmbeddingService (loads the default sentence‑transformers model).
    4️⃣  Create a Task, embed it, and persist it via Storage.
    5️⃣  Embed the previously created tags (they go straight into the embedding cache).
    6️⃣  Run a **hybrid search** (`semantic + keyword`) for a user query.
    7️⃣  Build semantic clusters and print a short summary.
   ──────────────────────────────────────────────────────── */
// Commented out example code - uncomment and convert to ES modules if needed
/*
import path from 'path';

// ---- 1️⃣  Load the core modules ---------------------------------
import {
  TodoziEmbeddingService,
  TodoziEmbeddingTool,
  TodoziEmbeddingConfig,
} from './emb.js';

import {
  Tag,
  TagManager,
  TagSortBy,
} from './tags.js';

import {
  Storage,
} from './storage.js';

import {
  // The data‑model classes – we only need Task in this example
  Task,
} from './models.js';

// ---- 2️⃣  Helper: tiny async wrapper ---------------------------------
(async () => {
  // -------------------------------------------------------------
  // 1️⃣ Initialise storage (creates the folder structure if missing)
  // -------------------------------------------------------------
  console.log('🗄️  Initialising storage …');
  const storage = await Storage.new();      // loads/creates ~/.todozi/tdz.hlx
  await storage.updateConfig(storage.config());   // (no‑op if config already exists)

  // -------------------------------------------------------------
  // 2️⃣ Build a TagManager and add a few tags
  // -------------------------------------------------------------
  console.log('🏷️  Setting up TagManager …');
  const tagMgr = TagManager.new();

  const tagIds = await Promise.all([
    tagMgr.createTag(new Tag({ name: 'frontend', category: 'area' })),
    tagMgr.createTag(new Tag({ name: 'backend',  category: 'area' })),
    tagMgr.createTag(new Tag({ name: 'urgent',   category: 'priority' })),
  ]);

  console.log('✅  Created tags:', tagIds);

  // -------------------------------------------------------------
  // 3️⃣ Initialise the embedding service
  // -------------------------------------------------------------
  console.log('🤖  Loading embedding model (may download ~90 MB)…');
  const embCfg = TodoziEmbeddingConfig.default();   // 384‑dim MiniLM‑L6‑v2
  const embService = await TodoziEmbeddingService.new(embCfg);
  await embService.initialize();                    // loads the model

  // -------------------------------------------------------------
  // 4️⃣ Create a Task and persist it (embedding is also stored)
  // -------------------------------------------------------------
  console.log('📝  Creating a new task …');

  const task = Task.newFull(
    'cli_user',                         // userId
    'Implement the new login flow',      // action
    '2 h',                               // time estimate
    'high',                              // priority (use string from models.js)
    'frontend',                         // parentProject
    'todo',                              // status
    null,                               // assignee (none)
    ['frontend', 'urgent'],             // tags – we reference the tags we just created
    [],                                 // dependencies
    'Use OAuth2 with PKCE',             // contextNotes
    null                                // progress
  );

  // Persist the task via the storage layer (adds it to the per‑project container)
  await storage.addTaskToProject(task);
  console.log('✅  Task stored with id =', task.id);

  // -------------------------------------------------------------
  // 5️⃣ Embed the tags – they go straight into the service cache
  // -------------------------------------------------------------
  console.log('🔖  Embedding tags …');
  for (const tagId of tagIds) {
    const tag = tagMgr.getTag(tagId);
    await embService.embedTag(tag);   // -> cached under key `tag_<id>`
  }
  console.log('✅  All tags cached.');

  // -------------------------------------------------------------
  // 6️⃣ Hybrid search: “login” + “backend” (semantic + keyword)
  // -------------------------------------------------------------
  console.log('\n🔎  Hybrid‑search for: "login backend" …');
  const query = 'login backend';
  const keywords = ['login', 'backend'];          // keyword part of the query
  const hybridResults = await embService.hybridSearch(
    query,
    keywords,
    null,               // search *all* content types (tasks, tags, …)
    0.7,                // semantic weight (0 … 1)
    10                  // limit
  );

  console.log(`\n=== ${hybridResults.length} results (combined score ≥ 0.7) ===`);
  hybridResults.forEach((r, i) => {
    const snippet = r.text_content.split('\n')[0];      // first line only
    console.log(
      `${i + 1}. [${r.content_type}] ${snippet}\n` +
      `   similarity: ${(r.similarity_score * 100).toFixed(1)}%` +
      `   tags: ${r.tags.join(', ')}`)
  });

  // -------------------------------------------------------------
  // 7️⃣ Build semantic clusters and print a tiny report
  // -------------------------------------------------------------
  console.log('\n🗂️  Building semantic clusters …');
  const clusters = await embService.clusterContent();

  console.log(`\n=== ${clusters.length} clusters found (threshold = ${embCfg.clustering_threshold}) ===`);
  clusters.forEach((c, idx) => {
    console.log(
      `\nCluster #${idx + 1} – ${c.cluster_size} items – ` +
      `avg similarity: ${(c.average_similarity * 100).toFixed(1)}%`);
    c.content_items.forEach(item => {
      const short = item.text_content.split('\n')[0];
      console.log(`   • (${item.content_type}) ${short}`);
    });
  });

  // -------------------------------------------------------------
  // 8️⃣  Use the high‑level “tool” wrapper – just for demonstration
  // -------------------------------------------------------------
  console.log('\n🛠️  Running the same hybrid search via TodoziEmbeddingTool …');
  const tool = await TodoziEmbeddingTool.new(embCfg);
  const toolResult = await tool.execute({
    action: 'semantic_search',   // any of the actions defined in the tool
    content: query,
    limit: 5,
  });

  console.log('\nTool result (JSON pretty‑printed):');
  console.log(JSON.stringify(JSON.parse(toolResult.output), null, 2));

  // -------------------------------------------------------------
  // DONE – you now have a fully‑functional, end‑to‑end flow
  // -------------------------------------------------------------
  console.log('\n🎉  Example 4 finished successfully.');
})().catch(err => {
  console.error('❌  Example 4 failed:', err);
  process.exit(1);
});

*/
/*
## Example 4 – Advanced Todozi Workflow  
**Goal:**  

* Create a **Tag** and a **Task** in Todozi.  
* Store their embeddings with the **TodoziEmbeddingService** (the heavy‑lifting is done by the `EmbeddingModel` placeholder).  
* Run a **semantic + keyword hybrid search** over all cached items.  
* Build **semantic clusters** and print a short report.  

The example shows how the different building blocks (`Storage`, `TagManager`, `TodoziEmbeddingService`, `TodoziEmbeddingTool`) can be wired together in a real‑world script.

---

### 1️⃣  Project layout (only the files we need)

/*
my‑app/
│
├─ emb.js            ← (provided)   – embedding service & tool
├─ models.js         ← (provided)   – data models (Task, Tag, …)
├─ storage.js        ← (provided)   – persistence layer
├─ tags.js           ← (provided)   – tag manager & search engine
└─ example‑4‑workflow.js   ← **our new script**

*/
/*
> **Important:** All the `require` statements below assume the files live in the same folder.  
> If you place the script elsewhere, adjust the relative paths accordingly.

---

### 2️⃣  The script – `example‑4‑workflow.js`

---

### 3️⃣  How the script works (step‑by‑step)

| Step | What happens | Key API used |
|------|--------------|--------------|
| **1** | `Storage.new()` loads (or creates) `~/.todozi` and the `tdz.hlx` config file. | `Storage.new()` |
| **2** | `TagManager` creates three tags (`frontend`, `backend`, `urgent`). | `TagManager.createTag()` |
| **3** | `TodoziEmbeddingService` loads the default sentence‑transformers model (`all‑MiniLM‑L6‑v2`). | `TodoziEmbeddingService.new()` → `initialize()` |
| **4** | A `Task` instance is built with our tags, then persisted via `storage.addTaskToProject()`. | `Task.newFull()`, `Storage.addTaskToProject()` |
| **5** | Each tag is embedded by `embService.embedTag(tag)`. The resulting vectors are kept in the service’s in‑memory cache. | `TodoziEmbeddingService.embedTag()` |
| **6** | **Hybrid search** – combines a semantic similarity (via the model) with a simple keyword boost. Returns up to 10 results sorted by the combined score. | `TodoziEmbeddingService.hybridSearch()` |
| **7** | `embService.clusterContent()` walks the cache and groups items whose cosine similarity ≥ `clustering_threshold` (0.8 by default). The report prints cluster size and a short preview of each item. | `TodoziEmbeddingService.clusterContent()` |
| **8** | The same query is executed through the higher‑level **tool** (`TodoziEmbeddingTool`). It simply forwards to the service and returns a JSON string. | `TodoziEmbeddingTool.execute()` |

---

### 4️⃣  Running the example

# 1️⃣  Install the minimal dependencies (uuid, luxon, etc.)
npm i uuid luxon

# 2️⃣  Run the script
node example-4-workflow.js

You should see output similar to: