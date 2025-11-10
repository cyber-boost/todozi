import { TodoziEmbeddingService, TodoziEmbeddingConfig, } from '../todozi/emb.js';
import { Tag, TagManager, TagSearchEngine, } from '../todozi/tags.js';

import path from 'path';
import { Storage } from '../todozi/storage.js';
import { createProject } from '../todozi/storage.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

await (await TodoziEmbeddingService.new())
          .addTask(task);               // stores + indexes
await (await TodoziEmbeddingService.new())
          .hybridSearch("dark mode", ["dark","mode"], null, 0.7, 5);

/*

 * Example 4 – Semantic tagging with Todo‑zi
 *
 *   1️⃣ Initialise storage
 *   2️⃣ Create tags (TagManager)
 *   3️⃣ Create a task (models.Task)
 *   4️⃣ Add the task to storage & to the embedding service
 *   5️⃣ Run a hybrid search (keyword + embedding)
 *   6️⃣ Ask TagManager for tag suggestions based on the best match
 *
 * Run with:
 *     node example-4-semantic-tagging.js
 */
"use strict";

// ---------------------------------------------------
// 0️⃣  Imports (all the heavy lifting lives in the repo)
// ---------------------------------------------------

// ----- Storage -------------------------------------------------

// ----- Core domain objects --------------------------------------
import {
  Task,               // from models.js
  Priority,
  Status,
  Assignee,
} from "../todozi/models.js";

// ----- Embedding service ----------------------------------------
// ----- Tag manager & search --------------------------------------
// ---------------------------------------------------
// 1️⃣  Initialise storage (creates ~/.todozi if needed)
// ---------------------------------------------------
async function initStorage() {
  const storage = await Storage.new();
  // Ensure the default project “general” exists – the CLI does this automatically
  // but we do it manually for the script.
  try {
    await createProject("general", "Catch‑all project for examples");
  } catch (_) {
    // project may already exist – ignore
  }
  return storage;
}

// ---------------------------------------------------
// 2️⃣  Build a tiny tag catalogue
// ---------------------------------------------------
async function buildTagCatalogue() {
  const manager = TagManager.new();

  // A few handy tags we want to reuse later
  const tags = [
    new Tag({ name: "frontend", description: "UI / client side work" }),
    new Tag({ name: "backend", description: "Server‑side logic" }),
    new Tag({ name: "bug", description: "Defect that needs fixing", color: "#e74c3c" }),
    new Tag({ name: "feature", description: "New functionality", color: "#2ecc71" }),
    new Tag({ name: "documentation", description: "Docs / READMEs" }),
  ];

  for (const t of tags) {
    await manager.createTag(t);
  }

  // Also build a search engine – we’ll use it for fuzzy tag suggestions later
  const searchEngine = new TagSearchEngine(manager);
  return { manager, searchEngine };
}

// ---------------------------------------------------
// 3️⃣  Create a new task (mirrors what the CLI does)
// ---------------------------------------------------
function makeSampleTask() {
  // Same signature as Todo‑zi CLI `add` command
  const task = Task.newFull(
    "example_user",               // userId
    "Add a dark‑mode toggle to the settings page", // action
    "2h",                         // time estimate
    Priority.MEDIUM,              // priority (enum from models.js)
    "general",                    // parent project
    Status.TODO,                  // status
    Assignee.HUMAN,               // assignee – we want a human to do it
    ["frontend"],                 // tags (string array, will be validated later)
    [],                           // dependencies
    "The UI currently only supports light theme.", // contextNotes
    null                          // progress (null = not started yet)
  );

  return task;
}

// ---------------------------------------------------
// 4️⃣  Persist the task & index it for embeddings
// ---------------------------------------------------
async function storeAndIndexTask(storage, task, embeddingService) {
  // 4‑a) Persist to the project‑specific container
  await storage.addTaskToProject(task);
  console.log(`📁 Task stored → ${task.id}`);

  // 4‑b) Index it in the embedding service (creates the vector and caches it)
  const taskId = await embeddingService.addTask(task);
  console.log(`🪄 Embedding generated (task id = ${taskId})`);
}

// ---------------------------------------------------
// 5️⃣  Hybrid search – keyword + semantic similarity
// ---------------------------------------------------
async function hybridSearch(embeddingService, query, keywords) {
  // The `hybridSearch` method lives in TodoziEmbeddingService (emb.js)
  const results = await embeddingService.hybridSearch(query, keywords, null, 0.7, 5);
  console.log("\n🔎 Hybrid search results:");
  results.forEach((r, i) => {
    console.log(
      `  ${i + 1}. ${r.textContent.split("\n")[0].slice(0, 80)}…\n` +
      `     ↳ similarity: ${(r.similarity_score * 100).toFixed(1)}%`
    );
  });
  return results;
}

// ---------------------------------------------------
// 6️⃣  Suggest extra tags for the *best* match
// ---------------------------------------------------
async function suggestTagsForBestMatch(searchEngine, bestResult, tagManager) {
  // The bestResult carries the original content_id (the Task ID) and the vector.
  const taskId = bestResult.content_id;
  const task = await tagManager.storage?.getTaskFromAnyProject?.(taskId);
  // In our tiny script we already have the task object, so we’ll just reuse it:
  const taskObj = bestResult._originalTask || null;

  // Pull out the existing tags from the task (if we have the object)
  const existingTagNames = taskObj?.tags || [];

  // Ask the tag manager for suggestions based on those tags.
  // Our `TagSearchEngine` has a method `getSuggestions(currentTags, limit)`.
  const suggestions = searchEngine.getSuggestions(existingTagNames, 3);
  console.log("\n💡 Tag suggestions based on the found task:");
  suggestions.forEach((s) => console.log(`  • ${s}`));
}

// ---------------------------------------------------
// 7️⃣  Main driver – glue everything together
// ---------------------------------------------------
(async () => {
  // ----- (1) storage -----
  const storage = await initStorage();

  // ----- (2) tag catalogue -----
  const { manager: tagManager, searchEngine: tagSearchEngine } = await buildTagCatalogue();

  // ----- (3) make a task -----
  const task = makeSampleTask();

  // ----- (4) embedding service + indexing -----
  const embConfig = TodoziEmbeddingConfig.default();
  const embeddingService = await TodoziEmbeddingService.new(embConfig);
  await storeAndIndexTask(storage, task, embeddingService);

  // ----- (5) run a hybrid search -----
  const query = "dark mode toggle for UI";
  const keywordList = ["dark", "mode", "toggle"];
  const hybridResults = await hybridSearch(embeddingService, query, keywordList);

  if (hybridResults.length === 0) {
    console.log("\n❌ No results – try a different query.");
    return;
  }

  // Save the original task object into the result (so we can read tags later)
  hybridResults[0]._originalTask = task;

  // ----- (6) suggest extra tags -----
  await suggestTagsForBestMatch(tagSearchEngine, hybridResults[0], tagManager);

  // ----- (7) clean‑up -------------------------------------------------
  // Close the embedding service (it may hold file‑handles, caches, etc.)
  if (embeddingService.cache?.clear) embeddingService.cache.clear();

  console.log("\n🏁 Example finished.");
})();

/ *
## Example 4 – Putting It All Together  
**Goal:**  

* Initialise the Todo‑zi storage layer.  
* Create a few tags, a new task and an agent.  
* Store the task in the embedding service so it can be searched semantically.  
* Run a *hybrid* search (keyword + embedding) that returns the most relevant tasks **and** suggests extra tags for the found task.  

The example showcases the most interesting moving parts of the code base:

| Piece | What we’ll do |
|------|----------------|
|`storage.js`|Create a `Storage` instance, add a task to a project.|
|`models.js`|Create a task object directly (the same class the CLI uses).|
|`emb.js`|Instantiate `TodoziEmbeddingService`, index the new task, then run a similarity query.|
|`tags.js`|Create a `TagManager`, add a few tags, and ask it for tag‑suggestions based on the similarity results.|

> **Why this is useful** – In a real workflow you would type `todozi add …`, but a programmatic flow lets you *auto‑tag* a task after you create it, or even suggest existing tasks when a user writes a free‑form description.

---

### 1️⃣ Prerequisites

/ *
bash
# Inside the Todo‑zi repo
npm install               # installs uuid, luxon, etc.
node -e "require('./cli.js')"   # sanity‑check that the CLI can be required

/ *
> The code assumes the Todo‑zi folder structure (`~/.todozi`) already exists.  
> If it does not, run `node -e "require('./storage.js').initStorage()"` once first.

---

### 2️⃣ Complete Script – `example‑4‑semantic‑tagging.js`

---

### 3️⃣  Walk‑through of What Happens

| Step | What the code does | Why it matters |
|------|-------------------|----------------|
| **0 – Imports** | Pulls in the exact classes the CLI uses (`Task`, `TodoziEmbeddingService`, `TagManager`, …) | Guarantees we are *re‑using* the same logic the command line does, not a duplicate implementation. |
| **1 – `initStorage`** | Calls `Storage.new()` → loads `~/.todozi/tdz.hlx`. If the default project `general` does not exist it creates it. | Storage is the single source‑of‑truth for every object in Todo‑zi. |
| **2 – Tag catalogue** | Instantiates a `TagManager`, creates five tags (`frontend`, `backend`, `bug`, …), and builds a `TagSearchEngine`. | Tags are stored *in‑memory* (the real CLI persists them to `~/.todozi/tags/ *.json`; here we keep it simple). |
| **3 – Sample task** | Calls `Task.newFull(...)` – exactly the same constructor the CLI uses when a user types `todozi add …`. | The task has a human assignee, a `frontend` tag, and a short description. |
| **4 – Persist + Embedding** | `storage.addTaskToProject(task)` writes the task into the project container JSON file.<br>`embeddingService.addTask(task)` creates a 384‑dimensional vector (placeholder random numbers in our stub) and caches it. | Both the *disk* and *vector* representations are now available for later queries. |
| **5 – Hybrid search** | Calls `embeddingService.hybridSearch(query, keywords, …)`. The method mixes:<br> • **Semantic similarity** (dot‑product of vectors) <br> • **Keyword boost** (checks that the raw text contains each keyword). | You get richer results than a plain full‑text search – tasks that are *conceptually* similar but don’t share every keyword appear. |
| **6 – Tag suggestions** | Takes the first (best) result, extracts its current tags (`["frontend"]`), then asks `TagSearchEngine.getSuggestions` for up to three more tags that commonly appear together (e.g. `["feature", "documentation"]`). | The user sees *auto‑complete* style suggestions that keep the taxonomy consistent. |
| **7 – Cleanup** | Clears any in‑memory cache to avoid file‑handle leaks. | Good practice when embedding services keep large vectors in RAM. |

---

### 4️⃣  Expected Console Output