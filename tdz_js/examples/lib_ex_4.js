import { TagManager } from './tags.js';
const tagMgr = await TagManager.loadFromFile(
  path.join(await Done.tdzfp(), 'tags.json')   // or any path you prefer
);

/*
#!/usr/bin/env node
/ *  --------------------------------------------------------------
    Example 4 – AI‑assisted project starter
    --------------------------------------------------------------
    Demonstrates:
      • Storage initialisation (Done.init())
      • Project creation
      • Task creation (normal, AI, custom‑tagged)
      • Memory creation
      • Semantic search across tasks + memories
      • Updating a task
      • Creating a custom Tag and attaching it to a task
    -------------------------------------------------------------- */
import path from 'path';
import { fileURLToPath } from 'url';

// -----------------------------------------------------------------
// 1️⃣  Load the Todozi public façade (`Done`) and the low‑level tools
// -----------------------------------------------------------------
import { Done } from './lib.js';               // <-- main entry point
import { TagManager, TagSortBy } from './tags.js'; // Tag subsystem
import { TodoziEmbeddingConfig } from './emb.js'; // Embedding config type

// -----------------------------------------------------------------
// Helper – pretty‑print a list of items (tasks, memories, tags …)
// -----------------------------------------------------------------
const printHeader = (title) => console.log(`\n=== ${title} ===`);
const printItems = (items) => {
  if (!items.length) {
    console.log('   (none)');
    return;
  }
  items.forEach((it, i) => console.log(`   ${i + 1}. ${it.id} – ${it.action || it.moment || it.idea}`));
};

// -----------------------------------------------------------------
// 2️⃣  Async IIFE – everything runs inside a single async block
// -----------------------------------------------------------------
(async () => {
  // ---------------------------------------------------------------
  // 2️⃣  Initialise the storage / folder structure if needed
  // ---------------------------------------------------------------
  printHeader('Initialising Todozi');
  await Done.init();          // creates ~/.todozi if missing, runs auto‑registration
  console.log('✅ Storage ready');

  // ---------------------------------------------------------------
  // 3️⃣  Create a *new* project called “AI‑Demo”
  // ---------------------------------------------------------------
  const projectName = 'AI-Demo';
  printHeader(`Creating project “${projectName}”`);
  await Done.createProject(projectName, 'Demo project used by Example 4');
  console.log('✅ Project created');

  // ---------------------------------------------------------------
  // 4️⃣  Create three tasks in the new project
  // ---------------------------------------------------------------
  printHeader('Creating tasks');

  // 4.1 – a plain “normal” task
  const normalTask = await Done.createTask(
    'Write the initial README for the demo project',
    Done.Priority.Medium,
    projectName,
    '2025-01-01',
    null                     // no special assignee → defaults to collaborative
  );
  console.log(`✅ Normal task created – ${normalTask.id}`);

  // 4.2 – an AI‑only task (automatically flagged for the AI assignee)
  const aiTask = await Done.ai('Generate a project scaffold (package.json, src/, tests/)');
  console.log(`✅ AI task created – ${aiTask}`);

  // 4.3 – a task that we will later tag
  const taggedTask = await Done.createTask(
    'Design the logo for the demo project',
    Done.Priority.Low,
    projectName,
    '2025-01-10',
    null
  );
  console.log(`✅ Tagged task created – ${taggedTask}`);

  // ---------------------------------------------------------------
  // 5️⃣  Persist a *memory* that the AI can later retrieve
  // ---------------------------------------------------------------
  printHeader('Saving a memory');
  const memory = await Done.createMemory(
    'First demo run on 2025‑01‑03',
    'The AI model performed a perfect semantic search',
    'To remember the quality of our initial setup'
  );
  console.log(`✅ Memory saved – ${memory.id}`);

  // ---------------------------------------------------------------
  // 6️⃣  Semantic (AI) search – find everything that matches
  // ---------------------------------------------------------------
  const query = 'How did the AI perform on the first run?';
  printHeader(`Semantic search for: "${query}"`);

  // `Done.aiFind()` internally calls the embedding service, so we
  // don’t have to instantiate it ourselves.
  const searchResults = await Done.aiFind(query);
  console.log('🔎 Matching items (ordered by similarity):');
  printItems(searchResults);

  // ---------------------------------------------------------------
  // 7️⃣  Update the “logo” task – mark it as “In‑Progress” and set 30 % done
  // ---------------------------------------------------------------
  printHeader('Updating a task (progress & status)');
  await Done.updateTaskStatus(taggedTask.id, Done.Status.InProgress);
  await Done.updateTaskFull(taggedTask.id, {
    progress: 30,
    // we can also change the assignee, tags, etc. – here we keep it unchanged
  });
  console.log(`✅ Task ${taggedTask.id} now In‑Progress (30 % complete)`);

  // ---------------------------------------------------------------
  // 8️⃣  Create a **custom tag** and attach it to the “logo” task
  // ---------------------------------------------------------------
  printHeader('Creating a custom tag “graphics”');

  // The TagManager lives in its own file – we instantiate it directly.
  const tagMgr = TagManager.new();                     // <-- fresh manager (in‑memory)
  const tagId = await tagMgr.createTag({
    name: 'graphics',
    description: 'All visual‑design related items',
    color: '#ff8800',
    category: 'design',
  });

  console.log(`✅ Tag created – ${tagId}`);

  // Attach the tag to the previously‑created task.
  // We could also use `Done.addToTask(taskId, tagName)` but that method only
  // accepts the *name* of an existing tag.  Since we already have the id we
  // add it manually:
  const task = await Done.getTask(taggedTask.id);
  if (!task.tags.includes('graphics')) task.tags.push('graphics');
  await Done.updateTaskFull(task.id, { tags: task.tags });
  console.log(`✅ Tag “graphics” attached to task ${task.id}`);

  // ---------------------------------------------------------------
  // 9️⃣  Verify that the tag shows up in a tag‑search
  // ---------------------------------------------------------------
  printHeader('Searching tags that contain “graph”');
  const tagSearchEngine = new (await import('./tags.js')).TagSearchEngine(tagMgr);
  const matchingTags = tagSearchEngine.advancedSearch({
    name_contains: 'graph',
    sort_by: TagSortBy.Name,
    limit: 5,
  });
  printItems(matchingTags);

  // ---------------------------------------------------------------
  // 🎉  All done – the script exits with a friendly summary
  // ---------------------------------------------------------------
  printHeader('Summary of created artefacts');
  console.log(`Project:          ${projectName}`);
  console.log(`Normal task:      ${normalTask.id}`);
  console.log(`AI task:          ${aiTask}`);
  console.log(`Tagged task:      ${taggedTask.id}`);
  console.log(`Memory:           ${memory.id}`);
  console.log(`Custom tag:       ${tagId}`);

  console.log('\n✅ Example 4 completed successfully!\n');
})().catch((err) => {
  console.error('\n❌ Example 4 failed:');
  console.error(err);
  process.exit(1);
});

/ *
## Example 4 – Full‑stack “AI‑assisted project starter”

This example shows how to **wire together the high‑level `Done` façade**, the **embedding service**, and the **tag manager** to:

1. **Bootstrap a new Todozi project** (creates the folder structure if it does not exist).  
2. **Create a few tasks** – one normal, one that is flagged for AI processing and one that will be linked to a custom tag.  
3. **Persist a memory** (so that semantic search works across both tasks *and* memories).  
4. **Run a semantic (AI) search** that finds the most relevant items for a free‑text query.  
5. **Update a task** (change its status and add a progress value).  
6. **Create and attach a custom tag** with the built‑in `TagManager`.  

All of this is done with a single, self‑contained Node script that can be dropped into any project that already has the Todozi source tree on its `node_modules` path.

> **What you’ll need**  
> * Node ≥ 18 (uses native `fs/promises` and top‑level `await`).  
> * The Todozi repository (the files you posted) installed as a local module – e.g. `npm i ./todozi`.  
> * Internet access the first time you run the script (the embedding model will be downloaded).

---

### 1️⃣  `project‑starter.js` – the full script

> **Save the file** as `project-starter.js` (or any name you like) **next to the `lib.js` file** that re‑exports Todozi’s public API.  
> Make it executable (`chmod +x project-starter.js`) and run it:

/ *
bash
$ ./project-starter.js

/ *
---

### 2️⃣  What the script does – step‑by‑step explanation

| Step | Code fragment | What happens |
|------|---------------|--------------|
| **Init** | `await Done.init();` | Ensures `~/.todozi` exists, runs `initWithAutoRegistration()` if needed, and loads the default config. |
| **Project** | `await Done.createProject(projectName, …);` | Persists a `Project` JSON file under `~/.todozi/projects/`. |
| **Normal task** | `await Done.createTask(..);` | Generates a UUID, builds a `Task` object, runs the optional embedding service, stores the task inside a `ProjectTaskContainer`. |
| **AI‑only task** | `await Done.ai(..);` | Shortcut that builds a task with `assignee = "ai"`; the task is immediately saved. |
| **Memory** | `await Done.createMemory(..);` | Builds a `Memory` model (standard type) and saves it in `~/.todozi/memories/`. |
| **Semantic search** | `await Done.aiFind(query);` | • Instantiates a `TodoziEmbeddingService` (model *all‑MiniLM‑L6‑v2*). <br>• Generates the embedding for the query. <br>• Retrieves all stored embeddings (tasks + memories) from the in‑memory cache (populated lazily on first use). <br>• Returns the most similar items (cosine similarity ≥ 0.7). |
| **Update task** | `await Done.updateTaskStatus(..)` + `await Done.updateTaskFull(..)` | Demonstrates both the “status‑only” helper and the generic full‑update helper (which can change any mutable field). |
| **Tag creation** | `TagManager.new(); … await tagMgr.createTag({ … });` | Directly uses the low‑level tag subsystem. The tag is stored **in‑memory** only for this demo (persisting tags would require a custom storage layer – Todozi ships one, but the example keeps it simple). |
| **Attach tag** | fetch task → `task.tags.push('graphics')` → `Done.updateTaskFull(..)` | Shows how you can mutate a task’s tag list directly. |
| **Tag search** | `new TagSearchEngine(tagMgr).advancedSearch({ … })` | Uses the advanced fuzzy / sorted search API from `tags.js`. |

---

### 3️⃣  Extending the example – a quick idea

If you want the **tag to survive across runs**, replace the in‑memory `TagManager` with the built‑in persistent version:

`TagManager` already ships with `save()` / `load()` helpers (not shown in the snippet above), so you can persist tags to `~/.todozi/tags.json` and reuse them later.

---

### 4️⃣  TL;DR – one‑liner cheat sheet