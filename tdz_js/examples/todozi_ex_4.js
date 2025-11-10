#!/usr/bin/env node
/* -------------------------------------------------------------
import { TodoziError, Priority, Status, Assignee, ItemStatus, ShareLevel, IdeaImportance, AssignmentStatus, ErrorSeverity, ErrorCategory, TrainingDataType, transformShorthandTags, parseTodoziFormat, processChatMessageExtended, } from '../todozi/todozi.js';
import { TodoziEmbeddingConfig, TodoziEmbeddingService, } from '../todozi/emb.js';

import { Storage } from '../todozi/storage.js';
import { TagManager } from '../todozi/tags.js';

   Example 4 – Auto‑tagging Todozi tasks with TagManager + Embedding
   ------------------------------------------------------------- */

'use strict';

// ----------------------------------------------------------------
// 1️⃣  Core Todozi utilities (parsing, enums, errors)
// ----------------------------------------------------------------
// 2️⃣  Storage – we need a real storage instance to persist tasks

// ----------------------------------------------------------------
// 3️⃣  Tag manager – holds tags and relationships

// ----------------------------------------------------------------
// 4️⃣  Embedding service – produces semantic vectors for tasks
// ----------------------------------------------------------------
// 5️⃣  Helper: pretty‑print a task (tiny UI helper)
function printTask(task) {
  console.log('─'.repeat(60));
  console.log(`🆔  ${task.id}`);
  console.log(`🗒️  ${task.action}`);
  console.log(`⏰  ${task.time}`);
  console.log(`⚡  Priority: ${task.priority}`);
  console.log(`📂  Project : ${task.parent_project}`);
  console.log(`📌  Status  : ${task.status}`);
  console.log(`🏷️  Tags    : ${task.tags.join(', ') || '(none)'}`);
  console.log(`🤖  Assignee: ${JSON.stringify(task.assignee)}`);
  console.log('─'.repeat(60));
}

// ----------------------------------------------------------------
// 6️⃣  MAIN – async IIFE so we can use await at top level
(async () => {
  // ----------------------------------------------------------------
  //   A. Initialise storage (creates ~/.todozi folder if missing)
  // ----------------------------------------------------------------
  await Storage.new(); // just to make sure the folder structure exists

  // ----------------------------------------------------------------
  //   B. Build a TagManager and add a few sample tags
  // ----------------------------------------------------------------
  const tm = TagManager.new();

  // Pretend we have a taxonomy of tags used in the organisation
  const tagIds = await Promise.all([
    tm.createTag({ name: 'frontend', description: 'UI / web client' }),
    tm.createTag({ name: 'backend', description: 'Server‑side logic' }),
    tm.createTag({ name: 'devops', description: 'Infrastructure & CI' }),
    tm.createTag({ name: 'research', description: 'Exploratory work' }),
    tm.createTag({ name: 'bug', description: 'Defect / error' }),
  ]);

  // ----------------------------------------------------------------
  //   C. Initialise the embedding service (loads the default model)
  // ----------------------------------------------------------------
  const embConfig = TodoziEmbeddingConfig.default(); // uses MiniLM‑L6‑v2
  const embService = await TodoziEmbeddingService.new(embConfig);

  // ----------------------------------------------------------------
  //   D. Example chat message that a user might type in the CLI
  // ----------------------------------------------------------------
  const rawMessage = `
    Hey team, I need a few things done today:

    <todozi>
      Build the login page; 2h; high; webapp; todo; ai; frontend,bug
    </todozi>

    <todozi>
      Deploy the new API to staging; 30m; medium; api; todo; human; devops
    </todozi>

    <todozi>
      Research the latest GraphQL caching strategies; 3h; low; research; todo; collaborative; research
    </todozi>
  `;

  // ----------------------------------------------------------------
  //   E. Parse the message – we now have raw task objects
  // ----------------------------------------------------------------
  const content = processChatMessageExtended(rawMessage, 'example_user');
  const rawTasks = content.tasks; // <-- array of plain JS objects

  // ----------------------------------------------------------------
  //   F. For each task:
  //        1️⃣  Generate an embedding (semantic vector)
  //        2️⃣  Ask the embedding service for the *most similar* tags
  //        3️⃣  Merge those tags with any user‑provided tags
  //        4️⃣  Persist the task using the storage layer
  // ----------------------------------------------------------------
  for (const rawTask of rawTasks) {
    // 1️⃣  Create a dense embedding for the task description
    const taskText = `${rawTask.action} ${rawTask.time} ${rawTask.priority}`;
    const taskEmbedding = await embService.generateEmbedding(taskText);
    rawTask.embedding_vector = taskEmbedding; // keep it on the object (optional)

    // 2️⃣  Find the 2 best matching tags (you can tune the number)
    const similarTagResults = await embService.findSimilarTags(taskText, 2);
    const similarTagIds = similarTagResults.map(r => r.content_id);

    // 3️⃣  Resolve tag IDs → tag objects → their names
    const similarTagNames = similarTagIds
      .map(id => tm.getTag(id))
      .filter(Boolean)               // ignore missing tags
      .map(tag => tag.name);

    // 4️⃣  Merge with any explicit tags the user already wrote
    const finalTagSet = new Set([
      ...(rawTask.tags || []),          // explicit tags from <todozi>
      ...similarTagNames,               // auto‑discovered tags
    ]);
    rawTask.tags = Array.from(finalTagSet);

    // 5️⃣  Persist the task (using the high‑level storage API)
    const storage = await Storage.new();
    await storage.addTaskToProject(rawTask);   // <-- saves into the proper project container

    // 6️⃣  Show the result
    printTask(rawTask);
  }

  // ----------------------------------------------------------------
  //   G. Bonus – Search for tasks that contain the word “login”
  // ----------------------------------------------------------------
  console.log('\n🔎 Searching for tasks that contain “login”…');
  const storage = await Storage.new();
  const allProjects = await storage.listProjects(); // returns an array of project objects

  // Pull every task from every project (quick‑and‑dirty demo)
  const allTasks = [];
  for (const proj of allProjects) {
    const container = await storage.loadProjectTaskContainer(proj.name);
    allTasks.push(...container.get_all_tasks());
  }

  const loginTasks = allTasks.filter(t => t.action.toLowerCase().includes('login'));
  for (const t of loginTasks) {
    console.log(` • ${t.id}: ${t.action} [tags: ${t.tags.join(', ')}]`);
  }

  console.log('\n✅ Example finished – you now have auto‑tagged tasks!');

})().catch(err => {
  console.error('❌ Example crashed:', err);
  process.exit(1);
});

/*
## Example 4 – Auto‑tagging Todozi tasks with the **Tag Manager** & **Embedding Service**

In a real‑world Todozi workflow you often want tasks to be *tagged automatically* so they can be found later with a single click.  
The code below shows how to:

1. **Boot‑strap the storage layer** (`Storage`) – the same class the CLI uses.  
2. **Create a few tags** (`TagManager`).  
3. **Parse a free‑form chat message** that contains `<todozi>` blocks (using the existing `processChatMessageExtended`).  
4. **Generate an embedding for each new task** (`TodoziEmbeddingService`).  
5. **Find the most‑relevant tags** (semantic similarity) and add them to the task before persisting it.  

> **What you’ll see** – a runnable Node script (`auto‑tag‑example.js`) that prints the created tasks together with the tags that were inferred automatically.

---

### 1️⃣  Install the required dependencies

/ *
bash
# From the root of the Todozi repo
npm install uuid luxon   # already required by Todozi core
npm install @xenova/transformers   # optional – used by the embedding model

/ *
> The embedding code in *emb.js* already contains a placeholder for the transformer library.
> If you don’t have it you can still run the example – the `EmbeddingModel.encode()` method will return
> random vectors (good enough for a demo).

---

### 2️⃣  `auto-tag-example.js`

#### What the script does, step‑by‑step

| Step | Action | Reason |
|------|--------|--------|
| **A** | `await Storage.new()` | Guarantees the `~/.todozi` folder hierarchy exists. |
| **B** | Create a `TagManager` and add tags like `frontend`, `backend`, `devops`, … | In a production install tags would live in a JSON file, but for the demo we keep them in‑memory. |
| **C** | Initialise `TodoziEmbeddingService` with the default MiniLM model. | The service will later turn a piece of text into a 384‑dimensional vector. |
| **D** | Define a raw chat string containing three `<todozi>` blocks. | This mimics what a user would type into the CLI (`todozi chat "…"`). |
| **E** | `processChatMessageExtended` → extracts an array of plain task objects. | Re‑uses the same parser the real CLI uses, so the data format is identical. |
| **F** | For each task:<br>1️⃣ Generate embedding<br>2️⃣ Find similar tags (semantic search)<br>3️⃣ Merge with explicit tags<br>4️⃣ Save the task | This is the *auto‑tag* pipeline. |
| **G** | Demonstrate a simple keyword search across all projects. | Shows that the saved tasks are now queryable. |

---

### 3️⃣  Run the example

chmod +x auto-tag-example.js
./auto-tag-example.js

Typical output (your UUIDs and timestamps will differ):
*/