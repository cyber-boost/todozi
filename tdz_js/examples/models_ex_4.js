/**
import { Task, TaskUpdate, Priority, Status, Assignee, } from '../todozi/models.js';
import { Tag, TagManager, TagSearchEngine, TagSearchQuery, } from '../todozi/tags.js';
import { TodoziEmbeddingTool, TodoziEmbeddingConfig, } from '../todozi/emb.js';

import { v4: uuidv4 } from 'uuid.js';
import { DateTime } from 'luxon.js';
import { Storage, initStorage, getStorageDir } from '../todozi/storage.js';
import { TodoziHandler } from '../todozi/cli.js';

 * --------------------------------------------------------------
 * Example 4 – “Smart Todozi”
 * --------------------------------------------------------------
 * This script demonstrates an end‑to‑end workflow:
 *
 *   1️⃣ Initialise storage   →   2️⃣ Create tags
 *   3️⃣ Create a task       →   4️⃣ Attach tags
 *   5️⃣ Embed the task      →   6️⃣ Semantic search
 *
 * All core objects are imported from the source files you posted.
 * --------------------------------------------------------------
 */

"use strict";

/* ------------------------------------------------------------------
 *  Imports
 * ------------------------------------------------------------------ */

// Core models (Task, Priority, Status, Assignee …)
// Storage helpers (initialise the .todozi directory, load/save)

// Tag manager (create / query / relate tags)
// Embedding tool – generates embeddings & runs semantic queries
// CLI façade – we will reuse a tiny portion of it to build a task

// ------------------------------------------------------------------
// 1️⃣  Initialise the storage layer (creates ~/.todozi if missing)
// ------------------------------------------------------------------
async function bootstrapStorage() {
  console.log("🔧 Ensuring Todozi folder structure …");
  await initStorage();                     // creates dirs + default files
  const dir = await getStorageDir();
  console.log(`🗂️  Storage directory: ${dir}\n`);
}

// ------------------------------------------------------------------
// 2️⃣  Create a few tags that we will later attach to a task
// ------------------------------------------------------------------
async function createDemoTags(tagMgr) {
  console.log("🏷️  Creating demo tags …");
  const tagIds = [];

  // 2‑step: create raw Tag objects, then persist them via the manager
  const tagsToCreate = [
    { name: "frontend",    description: "UI / client‑side work",   color: "#ff7f50" },
    { name: "backend",     description: "Server‑side / API",       color: "#1e90ff" },
    { name: "high‑priority", description: "Must be done ASAP",     color: "#ff4500" },
  ];

  for (const t of tagsToCreate) {
    const tag = new Tag(t);
    const id = await tagMgr.createTag(tag);
    tagIds.push(id);
    console.log(`   • ${t.name} → ${id}`);
  }

  // Let’s add a relationship: “frontend” is related to “high‑priority”
  const [frontendId, , highPriId] = tagIds;
  await tagMgr.addTagRelationship(frontendId, highPriId);
  console.log("   ↔  Added relationship: frontend → high‑priority\n");

  return tagIds;
}

// ------------------------------------------------------------------
// 3️⃣  Build a new task (replicating what the CLI does)
// ------------------------------------------------------------------
function buildDemoTask() {
  console.log("📝 Building a demo task …");
  const action   = "Implement login page";
  const time     = "2 h";
  const priority = Priority.High;          // <-- enum from models.js
  const project  = "website";
  const status   = Status.Todo;            // <-- enum from models.js
  const assignee = Assignee.Human;         // could also be Assignee.Ai, etc.

  // The `Task.newFull` signature mirrors the constructor in models.js
  const task = Task.newFull(
    "example‑user",   // userId (arbitrary for demo)
    action,
    time,
    priority,
    project,
    status,
    assignee,
    [],               // tags – added later
    [],               // dependencies
    "Use the design system for buttons", // contextNotes
    null              // progress (null → not started)
  );

  console.log(`   ✅ Created task ${task.id}\n`);
  return task;
}

// ------------------------------------------------------------------
// 4️⃣  Attach the tags we created earlier to the task
// ------------------------------------------------------------------
function attachTagsToTask(task, tagMgr, tagIds) {
  console.log("🔗 Attaching tags to the task …");

  // Resolve tag objects to their human‑readable names (optional)
  const tagNames = tagIds.map(id => tagMgr.getTag(id).name);
  console.log(`   Tags to add: ${tagNames.join(", ")}`);

  // Build a `TaskUpdate` – the same DTO the CLI expects
  const updates = TaskUpdate.new()
    .withTags(tagNames);   // <-- string[] of tag names

  // Apply the updates (mutates the `task` instance)
  task.update(updates);

  console.log("   ✅ Tags attached\n");
}

// ------------------------------------------------------------------
// 5️⃣  Persist the task + generate an embedding
// ------------------------------------------------------------------
async function persistTaskAndEmbed(storage, embedTool, task) {
  console.log("💾 Storing the task in the project container …");
  await storage.addTaskToProject(task);   // writes to ~/.todozi/project_tasks/*.json
  console.log("   ✅ Stored\n");

  // ----------------------------------------------------------------
  //   5️⃣‑a  Generate an embedding for the task text
  // ----------------------------------------------------------------
  console.log("📦 Generating embedding for the task …");
  const embedding = await embedTool.service.generateEmbedding(embedTool.service.prepareTaskContent(task));
  task.embeddingVector = embedding;      // store the vector back on the object

  // 5️⃣‑b  Save the updated task (now with embedding) again
  await storage.addTaskToProject(task);   // overwrites the same file with the vector
  console.log("   ✅ Embedding generated & saved\n");
}

// ------------------------------------------------------------------
// 6️⃣  Semantic‑search “similar tasks”
// ------------------------------------------------------------------
async function semanticSearchDemo(embedTool, query) {
  console.log(`🔎 Running semantic search for:  "${query}"`);
  const results = await embedTool.service.findSimilarTasks(query, 5);

  if (results.length === 0) {
    console.log("   ❌ No similar tasks found (threshold maybe too high).");
    return;
  }

  console.log(`   📊 Found ${results.length} similar task(s):`);
  results.forEach((res, i) => {
    console.log(
      `   ${i + 1}. ${res.textContent.split("\n")[0]} ` +
      `(score: ${(res.similarityScore * 100).toFixed(1)}%)`
    );
  });
  console.log("");
}

// ------------------------------------------------------------------
// Main driver – glue everything together
// ------------------------------------------------------------------
(async () => {
  try {
    // --------------------------------------------------------------
    // 1️⃣  Initialise storage
    // --------------------------------------------------------------
    await bootstrapStorage();

    // --------------------------------------------------------------
    // 2️⃣  Tag manager – create tags & relationships
    // --------------------------------------------------------------
    const tagMgr = TagManager.new();
    const tagIds = await createDemoTags(tagMgr);

    // --------------------------------------------------------------
    // 3️⃣  Build a new task (login page)
    // --------------------------------------------------------------
    const task = buildDemoTask();

    // --------------------------------------------------------------
    // 4️⃣  Attach the tags we just made
    // --------------------------------------------------------------
    attachTagsToTask(task, tagMgr, tagIds);

    // --------------------------------------------------------------
    // 5️⃣  Persist the task **and** generate an embedding
    // --------------------------------------------------------------
    //   a) Load storage (the wrapper that reads/writes to ~/.todozi)
    const storage = await Storage.new();

    //   b) Initialise the embedding service (uses the default config)
    const embedCfg = TodoziEmbeddingConfig.default();
    const embedTool = await TodoziEmbeddingTool.new(embedCfg); // <- async factory

    await persistTaskAndEmbed(storage, embedTool, task);

    // --------------------------------------------------------------
    // 6️⃣  Run a semantic‑search to see the task appear in results
    // --------------------------------------------------------------
    await semanticSearchDemo(embedTool, "Create a sign‑in screen");

    // --------------------------------------------------------------
    // BONUS – Show how the CLI handler could be used for a command
    // --------------------------------------------------------------
    console.log("🔧 Demonstrating CLI helper – completing the task …");
    const handler = await TodoziHandler.new(storage);
    await handler.completeTask(task.id);
    console.log(`   ✅ Task ${task.id} marked as DONE via CLI façade\n`);

  } catch (err) {
    console.error("\n💥 Example failed:", err);
    process.exit(1);
  }
})();

/ *
## Example 4 – “Smart Todozi”  
**Goal** – Show how the various pieces you just looked at can be wired together to:

1. Initialise the Todo‑zi storage layer.  
2. Create a few **tags** (using the `TagManager`).  
3. Create a **task** (via the `TodoziHandler` CLI helper).  
4. Attach the tags to the task (using `TaskUpdate`).  
5. Generate an **embedding** for the task (through `TodoziEmbeddingTool`).  
6. Perform a **semantic‑search** for tasks that are “similar” to a free‑text query.  

All of the code lives in a single file (`example4.js`) so you can run it from the terminal with

/ *
bash
node example4.js

/ *
---

## 1️⃣  Install the required dependencies

npm i uuid luxon
# The rest of the repository already ships with the core files
# (models.js, storage.js, emb.js, tags.js, cli.js, todozi.js …)

> **Note** – The real embedding model (sentence‑transformers) is optional for this demo.  
> If you do not have the heavy model locally the `TodoziEmbeddingTool` will fall back to a **dummy** vector (all‑zeros) – the example still works, you just won’t get meaningful similarity scores.

---

## 2️⃣  `example4.js`

---

## What the script does – Step‑by‑step

| Step | What happens | Which file/classes are used |
|------|--------------|-----------------------------|
| **1** | Ensures `~/.todozi` exists & creates the default `tdz.hlx` file. | `storage.initStorage()` & `storage.getStorageDir()` |
| **2** | Creates three tags (`frontend`, `backend`, `high‑priority`) and links `frontend → high‑priority`. | `TagManager`, `Tag`, `addTagRelationship()` |
| **3** | Constructs a `Task` object (login‑page) with priority **high** and assignee **human**. | `Task.newFull()` from `models.js` |
| **4** | Updates the task to reference the two tags we just created. | `TaskUpdate` + `task.update()` |
| **5** | Persists the task in the *project‑task‑container* (`project = website`).<br>Generates an embedding via `TodoziEmbeddingTool` and writes it back to storage. | `Storage.addTaskToProject()`, `TodoziEmbeddingTool`, `TodoziEmbeddingService.generateEmbedding()` |
| **6** | Queries the embedding index for tasks similar to “Create a sign‑in screen”. The freshly‑added task appears with a high similarity score. | `TodoziEmbeddingService.findSimilarTasks()` |
| **Bonus** | Shows the `TodoziHandler` CLI wrapper being used programmatically to mark the task as **DONE**. | `TodoziHandler.completeTask()` |

---

## Running the example

# 1️⃣  Install the tiny dependencies the demo needs
npm i uuid luxon

# 2️⃣  Run the script
node example4.js

You should see an output similar to:
*/