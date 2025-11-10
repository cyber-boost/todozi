#!/usr/bin/env node
/* ========================================================================
   Example 4 – Tag‑Task‑Embedding‑Search
   ========================================================================

   Demonstrates:
   • Storage initialisation (`storage.js`)
   • Tag creation & management (`tags.js`)
   • Task creation, storage and embedding (`models.js`, `emb.js`, `storage.js`)
   • Semantic similarity search (`emb.js')
   ======================================================================== */

import path from 'path';
import { v4 as uuidv4 } from 'uuid';
import { DateTime } from 'luxon';

// ----- 1️⃣  Storage ---------------------------------------------------------
import { Storage, initStorage, getStorageDir } from './storage.js';

// ----- 2️⃣  Tag management --------------------------------------------------
import {
  TagManager,
  Tag,
  TagSearchEngine,
  TagSearchQuery,
  TagSortBy,
} from './tags.js';

// ----- 3️⃣  Embedding --------------------------------------------------------
import {
  TodoziEmbeddingConfig,
  TodoziEmbeddingService,
} from './emb.js';

// ----- 4️⃣  Model helpers ----------------------------------------------------
import { Task, Priority, Status, Assignee } from './models.js';

// -------------------------------------------------------------------------
// Helper – pretty‑print a task
function formatTask(task) {
  const assignee = typeof task.assignee === 'string'
    ? task.assignee
    : task.assignee?.type === 'agent'
      ? `agent:${task.assignee.name}`
      : 'none';
  return `[${task.id}] ${task.action}
   └─ Project   : ${task.parent_project}
   └─ Priority  : ${task.priority}
   └─ Status    : ${task.status}
   └─ Assignee  : ${assignee}
   └─ Tags      : ${task.tags?.join(', ') || '—'}
   └─ Created   : ${task.created_at}`;
}

// -------------------------------------------------------------------------
// MAIN – async IIFE so we can use await at top level
(async () => {
  console.log('\n=== Todozi Example 4 – Tag‑Task‑Embedding‑Search ===\n');

  // -------------------------------------------------- 1️⃣ Initialise storage
  console.log('🔧 Ensuring storage folder structure...');
  await initStorage();                     // creates ~/.todozi/* if missing
  const storage = await Storage.new();     // loads config + ready to use
  console.log('✅ Storage ready.\n');

  // -------------------------------------------------- 2️⃣ Tag manager & sample tags
  console.log('🏷️  Creating a TagManager and a few tags...');
  const tagMgr = TagManager.new();

  // Create three tags (the method returns the generated UUID)
  const bugTagId = await tagMgr.createTag(new Tag({ name: 'bug', color: '#e53935' }));
  const authTagId = await tagMgr.createTag(new Tag({ name: 'auth', color: '#1e88e5' }));
  const refactorTagId = await tagMgr.createTag(new Tag({ name: 'refactor', color: '#43a047' }));

  // Retrieve full tag objects for later use
  const bugTag = tagMgr.getTag(bugTagId);
  const authTag = tagMgr.getTag(authTagId);
  const refactorTag = tagMgr.getTag(refactorTagId);

  console.log('✅ Tags created:');
  console.log(`   • ${bugTag.name}   (${bugTag.id})`);
  console.log(`   • ${authTag.name}  (${authTag.id})`);
  console.log(`   • ${refactorTag.name} (${refactorTag.id})\n`);

  // -------------------------------------------------- 3️⃣ Create a task and store it
  console.log('📝 Creating a task and attaching the tags...');
  const task = new Task();
  task.action = 'Fix login race condition and add unit tests';
  task.time = '2h';
  task.priority = Priority.High;               // <-- from models.js
  task.parent_project = 'general';             // default project created by initStorage()
  task.status = Status.Todo;
  task.assignee = Assignee.Human;              // pick a human assignee
  task.tags = [bugTag.name, authTag.name];    // use the *names* of the tags
  task.context_notes = 'The bug appears only under high load';
  task.created_at = DateTime.utc().toISO();
  task.updated_at = task.created_at;

  // Store the task in the appropriate project container
  await storage.addTaskToProject(task);
  console.log('✅ Task stored in project “general”.');
  console.log(formatTask(task) + '\n');

  // -------------------------------------------------- 4️⃣ Generate an embedding for the task
  console.log('🤖 Initialising the embedding service...');
  const embConfig = TodoziEmbeddingConfig.default(); // default = MiniLM‑L6‑v2
  const embService = await TodoziEmbeddingService.new(embConfig);
  await embService.initialize();                    // loads the dummy model

  // Prepare the text that will be embedded – you can use any representation you like.
  const taskContent = [
    task.action,
    task.context_notes,
    `priority:${task.priority}`,
    `status:${task.status}`,
    `tags:${task.tags.join(',')}`,
  ].join('\n');

  console.log('🔎 Generating embedding for the newly created task...');
  const taskEmbedding = await embService.generateEmbedding(taskContent);
  task.embedding_vector = taskEmbedding;   // store the vector in the task object
  // Persist the vector back to storage (the example uses the same `addTaskToProject` path)
  await storage.addTaskToProject(task);
  console.log('✅ Embedding generated and saved.\n');

  // -------------------------------------------------- 5️⃣ Semantic search for similar tasks
  const query = 'refactor the authentication flow';
  console.log(`🔎 Performing a semantic search for: "${query}"`);
  const similar = await embService.findSimilarTasks(query, 5);

  if (similar.length === 0) {
    console.log('❌ No similar tasks found – this is expected for a fresh database.');
  } else {
    console.log('\n📋 Similar tasks (ordered by similarity):');
    for (const result of similar) {
      // `result` is a SimilarityResult; we can fetch the original task if needed:
      const originalTask = await storage.getTaskFromAnyProject(result.content_id);
      console.log(`   • ${result.content_id} – similarity ${(result.similarity_score * 100).toFixed(1)}%`);
      console.log(formatTask(originalTask));
      console.log('---');
    }
  }

  // -------------------------------------------------- 6️⃣ Tag‑search demo (optional)
  console.log('\n🔎 Advanced tag search – find tags that contain “auth” and sort by usage');
  const tagQuery = new TagSearchQuery({
    name_contains: 'auth',
    sort_by: TagSortBy.Usage,
    limit: 10,
  });
  const tagEngine = TagSearchEngine.new(tagMgr);
  const matchingTags = tagEngine.advancedSearch(tagQuery);
  for (const t of matchingTags) {
    console.log(`   • ${t.name} (id:${t.id}, usage:${t.usage_count})`);
  }

  // -------------------------------------------------- DONE
  console.log('\n✅ Example 4 finished. All resources are stored under:');
  console.log(`   ${await getStorageDir()}`);
  console.log('\nYou can now run `todozi` CLI commands to view/list the data, e.g.:');
  console.log('   $ todozi project list');
  console.log('   $ todozi task list');
  console.log('   $ todozi tag list\n');
})().catch(err => {
  console.error('\n💥 Example 4 failed:', err);
  process.exit(1);
});

/ *
## Example 4 – Full‑stack “Create‑Tag‑Task → Embed → Semantic‑Search” workflow  

This example shows how the different building blocks that ship with **Todozi** can be combined:

1. **Initialize the storage layer** (creates ~/.todozi if it does not exist).  
2. **Create a few tags** with the `TagManager`.  
3. **Create a task**, attach the tags you just created and store the task in a project.  
4. **Generate an embedding** for the task with `TodoziEmbeddingService`.  
5. **Run a semantic similarity search** for a free‑text query and print the matching tasks.  

> **What you’ll see** – after running the script you’ll get a list of tasks that are semantically close to the query *“refactor the authentication flow”*.  
> The script is completely self‑contained; you only need the Todozi source tree (the files you posted) and a recent Node ≥ 18 runtime.

---

### 1️⃣  Install the runtime dependencies

/ *
bash
# From the root of the repository (where the files live)
npm i uuid luxon   # needed by the code (already in the repo but good practice)
# The embedding service uses a dummy model – no extra packages are required for the demo.

/ *
---

### 2️⃣  `example4.js` – the complete script

> **Save this file as `example4.js`** (next to the other source files).  
> Make it executable (`chmod +x example4.js`) and run it with `./example4.js` (or `node example4.js`).

---

### 3️⃣  What the script does – step‑by‑step  

| Step | Code snippet | Result |
|------|--------------|--------|
| **1️⃣ Initialise storage** | `await initStorage();` | Creates `~/.todozi/…` folder hierarchy and a default project called `general`. |
| **2️⃣ Tag creation** | `await tagMgr.createTag(new Tag({…}));` | Persists three tags (`bug`, `auth`, `refactor`) inside an in‑memory `TagManager`. In a real app you would later serialize the manager to disk. |
| **3️⃣ Task creation** | `new Task(); … await storage.addTaskToProject(task);` | Stores a new task (`Fix login race condition…`) in the `general` project container (`project_tasks/<hash>.json`). |
| **4️⃣ Embedding** | `await embService.generateEmbedding(taskContent);` | Produces a 384‑dimensional dummy embedding (real implementation would call a transformer model). The vector is stored on the task and persisted again. |
| **5️⃣ Semantic search** | `await embService.findSimilarTasks(query, 5);` | Looks for tasks whose embeddings are cosine‑similar to the query. The demo prints the matching task(s) with similarity scores. |
| **6️⃣ Tag‑search (optional)** | `TagSearchEngine.advancedSearch(tagQuery);` | Demonstrates the powerful fuzzy‑/filter‑search capabilities of the tag subsystem. |

---

### 4️⃣  Extending the example  

* **Persist the `TagManager`** – replace the in‑memory `TagManager` with a JSON file stored at `~/.todozi/tags.json`.  
* **Add more tasks** – call `storage.addTaskToProject()` multiple times and watch the similarity list grow.  
* **Use real embeddings** – swap the placeholder `EmbeddingModel` implementation in `emb.js` with a real transformer (e.g. `@xenova/transformers`). The rest of the workflow stays unchanged.  
* **Combine with the CLI** – after running the script you can run `todozi task list` or `todozi tag list` to see the same data from the command‑line interface.  

---

### 5️⃣  TL;DR – run it in one line (assuming the repo is cloned)
*/