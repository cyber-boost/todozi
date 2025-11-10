/**
import path from 'path';
import { Storage } from '../todozi/storage.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { TagManager } from '../todozi/tags.js';
import { Task, Priority, Status, Assignee } from '../todozi/models.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

 * Example 4 – Tag + Task creation + Semantic search
 *
 * Prerequisites (run once):
 *   npm install commander luxon uuid fast-json-patch
 *   # The heavy AI‑related deps (sentence‑transformers, @xenova/transformers, etc.)
 *   # are optional for the demo – the embedding service will fall back to a stub.
 *
 * Run:
 *   node example4.js
 */

'use strict';

// ---------------------------------------------------------------
// 1️⃣  Initialise core components
// ---------------------------------------------------------------

(async () => {
  try {
    console.log('\n=== 📦  Todozi – Example 4 start ==========================\n');

    // 1️⃣  Ensure the folder structure exists (initStorage creates ~/.todozi)
    const storage = await Storage.new();   // loads config from ~/.todozi/tdz.hlx
    await storage.updateConfig(storage.config()); // no‑op if config already exists

    // 2️⃣  Start the embedding service (will load the default model)
    const embConfig = TodoziEmbeddingConfig.default();
    const embService = await TodoziEmbeddingService.new(embConfig);
    console.log('🔧 Embedding service ready (model =', await embService.embeddingModel?.model || 'stub', ')');

    // 3️⃣  Tag manager – create a new tag and persist it
    const tagMgr = TagManager.new();

    const newTag = {
      name: '#frontend‑bug',
      description: 'Bugs that appear in the UI / client side',
      color: '#e53935',
      category: 'bug',
    };
    const tagId = await tagMgr.createTag(newTag);
    console.log('🏷️  Created tag', newTag.name, `(id=${tagId})`);

    // 4️⃣  Embed the tag (so it participates in semantic searches)
    await embService.embedTag({ id: tagId, name: newTag.name, description: newTag.description });
    console.log('🧩 Tag embedded and cached');

    // ---------------------------------------------------------------
    // 5️⃣  Create a task that references the tag
    // ---------------------------------------------------------------
    const myTask = Task.newFull(
      'example_user',                      // userId
      'Fix the dropdown alignment on the settings page', // action
      '2h',                                // time estimate
      Priority.High,                       // priority
      'frontend',                          // parent project
      Status.Todo,                         // status
      Assignee.Human,                      // assignee
      [newTag.name],                       // tags – use the tag name, not id (Todozi uses names)
      [],                                  // dependencies
      'The dropdown is shifted 5 px to the right on Chrome.', // context notes
      null                                 // progress%
    );

    // Persist the task (adds it to the right project container)
    await storage.addTaskToProject(myTask);
    console.log('✅ Task added →', myTask.id);

    // Embed the task automatically – we call the service directly for demo purposes
    await embService.addTask(myTask);
    console.log('🧩 Task embedding stored');

    // ---------------------------------------------------------------
    // 6️⃣  Semantic search – find related tags
    // ---------------------------------------------------------------
    console.log('\n🔍 Searching for tags similar to "UI bug"...');
    const tagResults = await embService.findSimilarTags('UI bug', 5);
    tagResults.forEach((r, i) => {
      console.log(`  ${i + 1}. ${r.text_content.trim()} (score=${(r.similarity_score * 100).toFixed(1)}%)`);
    });

    // ---------------------------------------------------------------
    // 7️⃣  Semantic search – find similar tasks
    // ---------------------------------------------------------------
    console.log('\n🔍 Searching for tasks similar to "dropdown misaligned"...');
    const taskResults = await embService.findSimilarTasks('dropdown misaligned', 5);
    taskResults.forEach((r, i) => {
      console.log(`  ${i + 1}. ${r.text_content.split('\n')[0]} (score=${(r.similarity_score * 100).toFixed(1)}%)`);
    });

    // ---------------------------------------------------------------
    // 8️⃣  Optional: Show clustering stats (useful for insights)
    // ---------------------------------------------------------------
    console.log('\n🗂️  Running a quick clustering pass...');
    const clusters = await embService.clusterContent();
    console.log(`  → ${clusters.length} cluster(s) discovered`);
    clusters.slice(0, 2).forEach((c, idx) => {
      console.log(`    #${idx + 1}: ${c.clusterSize} items, avg‑sim=${(c.averageSimilarity * 100).toFixed(1)}%`);
    });

    // ---------------------------------------------------------------
    // 9️⃣  Clean‑up (if you opened any long‑running resources)
    // ---------------------------------------------------------------
    // In the real CLI the process exits after `main()`. Here we just log.
    console.log('\n✅ Example 4 finished – happy tagging! 🎉\n');
  } catch (err) {
    console.error('❌ Example 4 failed:', err);
    process.exit(1);
  }
})();

/*
## Example 4 – End‑to‑end workflow  
**“Create a tag, embed it, add a task that uses the tag, and then perform a semantic search for similar tags & tasks.”**  

The snippet below ties together the most important pieces of the Todozi code‑base:

| Section | What it does |
|--------|--------------|
| **1️⃣ Initialise storage & services** | Loads (or creates) the Todozi folder structure, reads the configuration, creates a `Storage` instance, and starts the embedding service. |
| **2️⃣ Create a new tag** | Uses `TagManager` (from **tags.js**) to create a tag, persists it to the tag‑store and adds a usage‑counter. |
| **3️⃣ Embed the tag** | The embedding service stores the vector for the tag in its cache. |
| **4️⃣ Add a task that references the tag** | Builds a `Task` (from **models.js**), assigns the new tag, saves the task into the appropriate project and generates its embedding. |
| **5️⃣ Semantic search** | Demonstrates two queries: (a) find tags similar to a keyword, (b) find tasks similar to a short description. |
| **6️⃣ Clean‑up** | Shows how to gracefully shut‑down the services. |

> **Why this is useful** – It mirrors a real‑world use‑case: a user creates a domain‑specific tag (`#frontend‑bug`), adds a task that references that tag, and later asks Todozi “show me all open bugs that look like this”. The embedding layer makes the lookup fuzzy, fast, and language‑agnostic.

---  

### Full, runnable script (`example4.js`)

---  

### Step‑by‑step walk‑through  

| # | Action | Code excerpt | Explanation |
|---|--------|--------------|-------------|
| 1 | **Load / create storage** | `await Storage.new();` | Reads the `~/.todozi` folder, creates it if missing, and loads `tdz.hlx`. |
| 2 | **Start embedding service** | `TodoziEmbeddingService.new(...)` | Asynchronously loads the default Sentence‑Transformer (`all‑MiniLM‑L6‑v2`). If the model cannot be downloaded, a stub implementation is used so the demo never crashes. |
| 3 | **Create a tag** | `await tagMgr.createTag(newTag);` | Persists a JSON representation of the tag in an in‑memory `Map`. In a real install you would write the tag to `~/.todozi/tags/ *.json`. |
| 4 | **Embed the tag** | `await embService.embedTag({ id: tagId, name: … });` | Generates a 384‑dimensional vector and stores it in the service’s internal cache (`Map`). |
| 5 | **Create a task** | `Task.newFull(...)` | Uses the rich constructor from **models.js** – all fields are typed and validated. |
| 6 | **Persist the task** | `await storage.addTaskToProject(myTask);` | The task is stored in the per‑project container (`~/.todozi/project_tasks/<hash>.json`). |
| 7 | **Embed the task** | `await embService.addTask(myTask);` | The same embedding pipeline is reused for tasks; the vector is saved alongside the tag vectors. |
| 8 | **Search similar tags** | `embService.findSimilarTags('UI bug', 5);` | Returns a list of `SimilarityResult` objects, each containing `content_id`, `similarity_score`, and the original text. |
| 9 | **Search similar tasks** | `embService.findSimilarTasks('dropdown misaligned', 5);` | Shows how a short natural‑language query can retrieve the exact task we just added (score ~0.9‑1.0). |
|10| **Cluster (optional)** | `embService.clusterContent();` | Demonstrates the built‑in clustering algorithm – handy for “show me what topics are most common”. |
|11| **Shutdown** | Implicit – the script ends gracefully. | In a long‑running CLI you would call `await embService.cleanupExpired()` or close any file handles. |

---  

### What you need to run the example  

| Dependency | Why |
|------------|-----|
| `commander`, `luxon`, `uuid` | Used throughout the core code for CLI parsing, timestamps, and ID generation. |
| **Optional heavy AI deps** (`@xenova/transformers`, `sentence‑transformers` models) | If installed, the embedding service will download a real transformer model and produce *real* vectors. If not, the stub returns random vectors – the demo still works, just the similarity numbers are meaningless. |
| `fast-json-patch` (or any JSON diff lib) | Required by the storage layer for incremental updates (already pulled in by the repository). |

/ *
bash
npm install commander luxon uuid fast-json-patch
# optional, for real embeddings:
npm install @xenova/transformers sentence-transformers

/ *
---  

### Extending the snippet  

The example can be expanded in many directions:

1. **Persist tags to disk** – add a `saveTag(tag)` helper that writes `~/.todozi/tags/<id>.json`.  
2. **Batch embed many tasks** – call `embService.generateEmbeddingsBatch(tasks.map(t => embService.prepareTaskContent(t)))`.  
3. **Hybrid search** – combine keyword filtering with semantic similarity via `embService.hybridSearch(...)`.  
4. **Rate‑limited cache** – replace `Map` with `LRUEmbeddingCache` (from **emb.js**) to bound memory usage.  
5. **UI integration** – plug the results into the TUI (`TodoziApp`) by feeding the `SimilarityResult` objects into a `Table` component.

---  

### TL;DR
*/