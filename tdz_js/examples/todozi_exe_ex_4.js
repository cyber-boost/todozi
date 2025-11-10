#!/usr/bin/env node
// ---------------------------------------------------------------
// Example 4 – Automated Project Planning with AI Search & Embeddings
// ---------------------------------------------------------------

/* ------------------------------------------------------------
   1️⃣  Imports – everything we need from the repo
   ------------------------------------------------------------ */
import {   // note: ES‑module import (node ≥18)
  // Core executor & result helpers
  executeTodoziToolDelegated,
  ExecutorError,
  ExecutionResult,

  // Storage layer (creates/loads the global Todozi storage)
  getStorage,

  // Embedding service & tool (semantic search, clustering, etc.)
  TodoziEmbeddingService,
  TodoziEmbeddingConfig,

  // Model helpers (used by the service)
  EmbeddingModel,

  // Misc utils
  Done,               // proxy that points to the simple‑interface wrappers
} from './todozi_exe.js';

import { Project } from './models.js';
import { v4 as uuidv4 } from 'uuid';
import { DateTime } from 'luxon';

/* ------------------------------------------------------------
   2️⃣  Helper: tiny wrapper around the simple‑interface (Done)
   ------------------------------------------------------------ */
async function createTask({action, time, priority, project, status = 'todo', assignee = null, tags = []}) {
  // The `Done` namespace comes from todozi_exe.js and provides:
  //   Done.task, Done.urgent, Done.high, Done.low, etc.
  // We will just use the generic `task` interface here.
  const taskId = await Done.task(action);
  console.log(`📋 Created task ${taskId}: "${action}"`);
  return taskId;
}

/* ------------------------------------------------------------
   3️⃣  Main flow – async IIFE so we can use await at top level
   ------------------------------------------------------------ */
(async () => {
  console.log('\n=== 🚀 Todozi Project‑Planning Demo ===\n');

  // --------------------------------------------------------
  // 3.1 Initialise the Todozi system (mutex‑protected)
  // --------------------------------------------------------
  try {
    // `executeTodoziToolDelegated` will internally call `ensureTodoziSystem()`
    // but we also want a clean storage reference for the later steps.
    await executeTodoziToolDelegated({ action: 'stats' }); // trivial call just to init
  } catch (e) {
    console.error('❌ Failed to initialise Todozi system:', e);
    process.exit(1);
  }

  const storage = await getStorage(); // global storage singleton

  // --------------------------------------------------------
  // 3.2  Create (or fetch) a project called “Demo‑Planner”
  // --------------------------------------------------------
  const projectName = 'Demo-Planner';
  try {
    const existing = await storage.getProject(projectName);
    console.log(`📁 Project "${projectName}" already exists.`);
  } catch (_) {
    // Project not found – create it
    const proj = new Project(projectName, 'A sandbox project used by the demo script');
    await storage.createProject(proj.name, proj.description);
    console.log(`📁 Created new project "${projectName}"`);
  }

  // --------------------------------------------------------
  // 3.3  Add a few sample tasks (some with tags)
  // --------------------------------------------------------
  const sampleTasks = [
    {
      action: 'Design the landing page UI',
      time: '2h',
      priority: 'high',
      project: projectName,
      tags: ['design', 'ui'],
    },
    {
      action: 'Implement authentication flow (OAuth2)',
      time: '4h',
      priority: 'critical',
      project: projectName,
      tags: ['backend', 'security'],
    },
    {
      action: 'Write unit tests for the payment module',
      time: '3h',
      priority: 'medium',
      project: projectName,
      tags: ['testing', 'payments'],
    },
    {
      action: 'Prepare release notes for v1.0',
      time: '1h',
      priority: 'low',
      project: projectName,
      tags: ['documentation'],
    },
  ];

  // Keep the generated task IDs – we’ll need them later
  const createdTaskIds = [];

  for (const t of sampleTasks) {
    const id = await createTask({
      action: t.action,
      time: t.time,
      priority: t.priority,
      project: t.project,
      tags: t.tags,
    });
    createdTaskIds.push(id);
  }

  // --------------------------------------------------------
  // 3.4  Initialise the embedding service (loads the default model)
  // --------------------------------------------------------
  const embedConfig = TodoziEmbeddingConfig.default();
  const embedService = await TodoziEmbeddingService.new(embedConfig);
  console.log('\n🔎 Embedding service ready – model loaded.');

  // --------------------------------------------------------
  // 3.5  Generate embeddings for *all* tasks in the project
  // --------------------------------------------------------
  console.log('\n📦 Generating embeddings for existing tasks...');
  const allProjectTasks = await storage.listTasksAcrossProjects({ project: projectName });

  // Note: `embedService.addTask(task)` expects a *full* Task object.
  // The storage returns tasks in the shape defined in `models.js`, which
  // already contains all fields (including `embedding_vector` if it existed).
  for (const task of allProjectTasks) {
    // If this task already has an embedding we can skip it
    if (!task.embedding_vector) {
      await embedService.addTask(task);
      console.log(`   • Embedded task ${task.id}`);
    }
  }

  // --------------------------------------------------------
  // 3.6  Run an *AI semantic search* for a new feature request
  // --------------------------------------------------------
  const newFeature = 'Add a dark‑mode toggle to the settings page';
  console.log(`\n🔎 Performing AI‑semantic search for: "${newFeature}"`);

  // Use the same embedding service directly (bypasses the generic executor)
  const similar = await embedService.findSimilarTasks(newFeature, 5);
  console.log(`\n🔎 Top ${similar.length} similar tasks:`);

  similar.forEach((sim, idx) => {
    console.log(
      `   ${idx + 1}. ${sim.text_content.split('\n')[0]} ` +
      `(score: ${(sim.similarity_score * 100).toFixed(1)}%)`
    );
  });

  // --------------------------------------------------------
  // 3.7  Build a quick *project plan* from the search results
  // --------------------------------------------------------
  const planSteps = similar.map((s, i) => `${i + 1}. ${s.text_content.split('\n')[0]}`);

  const planContent = `
  <todozi>
    Draft plan for dark‑mode feature; 1h; high; ${projectName}; todo
    ${planSteps.join('\n')}
  </todozi>
  `.trim();

  console.log('\n📝 Creating a plan task using the “plan” tool...');
  const planResult = await executeTodoziToolDelegated({
    action: 'plan',
    content: planContent,
    // The simple‑interface `plan` expects the raw `<todozi>` block;
    // the executor will forward it to the `executePlanApi` implementation.
  });

  console.log('\n✅ Plan task created – ExecutionResult:');
  console.log(JSON.stringify(planResult, null, 2));

  // --------------------------------------------------------
  // 3.8  (Optional)  Show overall stats – proof that everything persisted
  // --------------------------------------------------------
  const stats = await executeTodoziToolDelegated({ action: 'stats' });
  console.log('\n📊 Current Todozi Stats:');
  console.log(stats.output);

  console.log('\n=== 🎉 Demo finished! ===\n');
})().catch(err => {
  if (err instanceof ExecutorError) {
    console.error('🛑 ExecutorError:', err.toString());
  } else {
    console.error('🛑 Unexpected error:', err);
  }
  process.exit(1);
});

/*
## Example 4 – Automated Project Planning with AI Search & Embeddings  

Below is a **stand‑alone script** that shows how the different pieces of the Todozi codebase can be combined to:

1. **Initialize** the Todozi system (mutex‑protected).  
2. **Create a new project** (if it does not already exist).  
3. **Add a handful of tasks** (some with explicit tags).  
4. **Generate embeddings** for those tasks with the `TodoziEmbeddingService`.  
5. **Run an AI semantic search** (`ai_search`) to discover the most relevant existing tasks for a new “feature‑request”.  
6. **Create a short project‑plan** (`plan` action) that stitches the discovered tasks together.  

The script works from a regular Node.js environment – no TUI/GUI is needed – and shows the **full request‑‑>response cycle** that the rest of the code (executors, storage, embedding service, and the “simple‑interface” tool wrappers) expects.

---  
### 1️⃣ Prerequisites  

| What you need | How to get it |
|---------------|---------------|
| **Node ≥ 18** (supports ES‑modules) | `nvm use 20` or download from nodejs.org |
| **Dependencies** (the same ones used by the repo) | From the repo root: <br>`npm install async-mutex axios luxon uuid` |
| **Todozi storage directory** (`~/.todozi`) – created automatically on first run | No manual step required |
| **Sentence‑Transformers model** (default `all‑MiniLM‑L6‑v2`) – the **EmbeddingTool** will download it on‑demand | First call to `emb set-model …` will fetch it automatically |

> **Tip** – Run `node todozi_exe.js` once (or `node cli.js version`) to let the initialization code create the storage layout (`~/.todozi/...`).  

---  
### 2️⃣ The script (`project‑plan‑demo.js`)

---  
### 3️⃣ What the script does (step‑by‑step)  

| Step | Code fragment | Effect |
|------|---------------|--------|
| **3.1** | `import … from './todozi_exe.js'` | Pulls in the **executor**, **storage** singleton, **embedding service**, and the *simple‑interface* façade (`Done`). |
| **3.2** | `createTask()` helper | Wraps `Done.task()` – the simplest way to create a task via the *enhanced simple interfaces*. |
| **3.3** | `await getStorage();` | Retrieves the global `Storage` instance that talks to `~/.todozi`. |
| **3.4** | `await storage.createProject(...)` | Guarantees a project named **Demo‑Planner** exists. |
| **3.5** | Loop over `sampleTasks` → `createTask()` | Adds four concrete tasks (design, auth, tests, release‑notes) and prints their IDs. |
| **3.6** | `TodoziEmbeddingService.new()` | Instantiates the embedding service (loads the default Sentence‑Transformers model). |
| **3.7** | `embedService.addTask(task)` loop | Generates and stores a 384‑dimensional embedding for every task that didn’t have one yet. |
| **3.8** | `embedService.findSimilarTasks(newFeature, 5)` | Performs a **semantic similarity search** – the core of the `ai_search` tool – returning the 5 most relevant existing tasks. |
| **3.9** | Build a textual plan (`planContent`) | Takes the search results, numbers them, and wraps everything in a `<todozi>` block that the **plan** tool understands. |
| **3.10** | `executeTodoziToolDelegated({action:'plan', …})` | Calls the *delegated* executor → `executePlanApi` → **AI project planning** (in the demo it simply echoes the plan, but in a real deployment it would call `extractContent` / a LLM). |
| **3.11** | `executeTodoziToolDelegated({action:'stats'})` | Shows a quick overview (`📊 Todozi Stats`) proving that the tasks, embeddings and plan were persisted. |

---  
### 4️⃣ Running the demo

/ *
bash
# 1️⃣ Make the script executable (optional)
chmod +x project-plan-demo.js

# 2️⃣ Execute it
node project-plan-demo.js

/ *
**Expected output (trimmed for brevity)**:
*/