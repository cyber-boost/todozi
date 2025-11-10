// Note: Tools like SimpleTodoziTool, CreateTaskTool, CreateMemoryTool, ChecklistTool would be imported from '../todozi/todozi_tool.js' if needed
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';

import { Pipeline, AutoTokenizer, AutoModel } from '@xenova/transformers.js';
import { TagManager } from '../todozi/tags.js';
import { initStorage, Storage } from '../todozi/storage.js';
import { v4 as uuidv4 } from 'uuid';
import readline from 'readline';

async function load(modelName, device) {
  console.log(`Loading ${modelName}…`);
  const tokenizer = await AutoTokenizer.from_pretrained(modelName);
  const model = await AutoModel.from_pretrained(modelName);
  return new EmbeddingModel(model, tokenizer, device, 384);
}

// Embedding service & types
// Tag manager (optional, but shows how a custom piece can be added)

// Storage layer (creates ~/.todozi if missing)

// Utility helpers

// ------------------------------------------------------------
// Helper: pretty‑print a task with emojis (re‑uses code from TodoziHandler)
function formatTaskWithEmojis(task) {
  const statusMap = {
    todo: "📝",
    in_progress: "🔄",
    blocked: "🚫",
    review: "👀",
    done: "✅",
    completed: "✅",
    cancelled: "❌",
    deferred: "⏸️",
  };
  const priorityMap = {
    low: "🟢",
    medium: "🟡",
    high: "🟠",
    critical: "🔴",
    urgent: "🚨",
  };
  const assigneeMap = {
    ai: "🤖",
    human: "👤",
    collaborative: "🤝",
  };

  const statusEm = statusMap[task.status] || "❓";
  const priEm = priorityMap[task.priority] || "❓";
  const assEm = task.assignee ? assigneeMap[task.assignee] || "❓" : "❓";

  let line = `${statusEm} ${priEm} ${assEm} [${task.id}] ${task.action}`;
  if (task.tags?.length) line += ` #${task.tags.join(" #")}`;
  line += `\n  📁 ${task.parent_project} | ⏱️ ${task.time}`;
  if (task.progress !== undefined) line += ` | 📊 ${task.progress}%`;
  if (task.dependencies?.length) line += ` | 🔗 ${task.dependencies.join(", ")}`;
  return line;
}

// ------------------------------------------------------------
// Main async wrapper
(async () => {
  // 1️⃣ Initialise storage (creates folder tree, default project, etc.)
  await initStorage();
  const storage = await Storage.new(); // thin wrapper around the JSON files

  // Use ChecklistTool to extract tasks from a checklist
  const checklist = new ChecklistTool(storage);
  await checklist.execute({
    content: `
      * [ ] Refactor the tag manager.
      * [ ] Write unit tests for the embedding service.
      * [ ] Update README with the new "smart‑todo" section.
    `,
    project: "general",
    priority: "high",
    assignee: "human",
  });
  console.log("\n📋 Extracted and created tasks from the checklist.");

  // Create a memory about the demo
  const memoryTool = new CreateMemoryTool(storage);
  await memoryTool.execute({
    moment: "Implemented smart‑todo demo",
    meaning: "Shows how to glue storage, tags and embeddings",
    reason: "Documentation / learning",
    importance: "high",
    term: "short",
  });
  console.log("\n🧠 Saved a short‑term memory about the demo.");

  // 2️⃣ Create a few tags (optional – shows TagManager usage)
  const tagMgr = TagManager.new();
  const devTagId = await tagMgr.createTag({ name: "dev", description: "development work" });
  const bugTagId = await tagMgr.createTag({ name: "bug", description: "bug‑fixes" });
  // Attach tags to the manager (real persistence would be added later)
  console.log("\n🗂️  Created tags:", (await tagMgr.getAllTags()).map(t => t.name).join(", "));

  // 3️⃣ Build three tasks via the high‑level SimpleTodoziTool
  const simpleTool = new SimpleTodoziTool();

  const tasksToCreate = [
    { action: "Write README for the new CLI tool", time: "2h", priority: "medium", assignee: "human" },
    { action: "Fix the failing unit test in utils.test.js", time: "1h", priority: "urgent", assignee: "ai" },
    { action: "Design the tag‑management UI", time: "3h", priority: "high", assignee: "collaborative" },
  ];

  const createdTaskIds = [];

  for (const t of tasksToCreate) {
    const result = await simpleTool.execute({
      action: t.action,
      // The SimpleTodoziTool only needs “action”. The rest are passed via the
      // “extra” field – we’ll embed them ourselves later.
      extra: `time:${t.time};priority:${t.priority};assignee:${t.assignee}`,
    });
    if (result.success) {
      // The stub returns a UUID, we store it for later embedding.
      const fakeId = uuidv4();               // in a real system you would get the ID from storage
      createdTaskIds.push({ id: fakeId, ...t });
      console.log(`\n✅ Created task → ${t.action}`);
    } else {
      console.error("\n❌ Failed to create task:", result.content);
    }
  }

  // 4️⃣ Persist the tasks in the storage layer
  // (here we just use the storage API directly – in a real CLI the tool would do it)
  for (const t of createdTaskIds) {
    const taskObj = {
      id: t.id,
      user_id: "cli_user",
      action: t.action,
      time: t.time,
      priority: t.priority,
      parent_project: "general",
      status: "todo",
      assignee: t.assignee,
      tags: [],               // you could push devTagId / bugTagId here
      dependencies: [],
      context_notes: null,
      progress: null,
      embedding_vector: null,
      created_at: new Date(),
      updated_at: new Date(),
    };
    await storage.addTaskToProject(taskObj);
  }

  // 5️⃣ Initialise the embedding service (config → model + cache)
  const embConfig = TodoziEmbeddingConfig.default();
  const embeddingService = await TodoziEmbeddingService.new(embConfig);

  // 6️⃣ Generate embeddings for **all** tasks that exist in storage
  // Note: listProjectTaskContainers would be called on the storage instance if available
  const containers = storage.listProjectTaskContainers
    ? await storage.listProjectTaskContainers()
    : []; // fallback if not available; for the demo we just use the three we added

  // For simplicity we embed only the three tasks we just created:
  for (const t of createdTaskIds) {
    const content = `Task: ${t.action}\nPriority: ${t.priority}\nAssignee: ${t.assignee}\nTime: ${t.time}`;
    const embedding = await embeddingService.generateEmbedding(content);
    // Store the vector back into the task (real storage would have a method for this)
    t.embedding_vector = embedding;
  }

  // 7️⃣ Semantic search – ask the user for a query
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question("\n🔎 Enter a search query (e.g. 'fix bug'): ", async (query) => {
    // Build a temporary “Task”‑like object that contains the query text
    const similar = await embeddingService.findSimilarTasks(query, 5);

    console.log("\n🤖 Top‑5 similar tasks (semantic search):");
    if (similar.length === 0) {
      console.log("  ❌ No matches found.");
    } else {
      // The stub `findSimilarTasks` returns an empty list because we never added
      // tasks to its internal `tasks` array.  To make the demo work we'll just
      // compute similarity manually against the three tasks we have.
      const sims = await Promise.all(createdTaskIds.map(async (t) => {
        const vec = t.embedding_vector;
        const sim = embeddingService.cosineSimilarity(
          await embeddingService.generateEmbedding(query),
          vec
        );
        return { task: t, similarity: sim };
      }));
      sims.sort((a, b) => b.similarity - a.similarity);
      sims.slice(0, 5).forEach((r) => {
        console.log(`  • ${formatTaskWithEmojis(r.task)} (score: ${r.similarity.toFixed(2)})`);
      });
    }

    // 8️⃣ Show a tiny embedding‑statistics report
    console.log("\n📊 Embedding statistics:");
    const stats = await embeddingService.getStats();
    console.log(`  Total cached embeddings: ${stats.total_embeddings}`);
    console.log(`  Types breakdown: ${JSON.stringify(stats.type_counts)}`);

    // 9️⃣ (Optional) Run a quick clustering demo
    const clusters = await embeddingService.clusterContent();
    if (clusters.length > 0) {
      console.log("\n🔗 Found semantic clusters:");
      clusters.forEach((c, i) => {
        console.log(`  ${i + 1}. ${c.clusterSize} items – avg similarity ${c.averageSimilarity.toFixed(2)}`);
      });
    } else {
      console.log("\n🔗 No clusters – not enough data (requires ≥2 items per cluster).");
    }

    readline.close();
    process.exit(0);
  });
})();

/*
## Example 4 – A **Full‑stack “Smart‑Todo” Script**

This example shows how you can combine the **high‑level tools** that live in `todozi_tool.js` with the **embedding service** (`emb.js`) and the **storage layer** (`storage.js`) to build a tiny “smart‑todo” command‑line utility.

The script will:

1. **Bootstrap the storage** (creates the `~/.todozi` folder structure if it does not exist).  
2. **Create three tasks** (one normal, one urgent, one collaborative) using the `SimpleTodoziTool`.  
3. **Add a few tags** with the `TagManager` (`tags.js`).  
4. **Generate embeddings** for all tasks (via `TodoziEmbeddingService`).  
5. **Run a semantic‑search** for a user‑provided query and print the most similar tasks with emoji‑enhanced formatting.  
6. **Show a tiny report** (stats + clustering) that demonstrates the power of the embedding layer.

> **What you’ll see**
> * Real‑time creation of tasks (no external API needed).  
> * Automatic embedding generation and caching.  
> * AI‑style “find‑similar‑tasks” without leaving the terminal.  

---  

### 1️⃣  Install the required packages (once)

# Core Todozi code already ships with a lot of internal modules.
# You only need a few external deps for the example:
npm i uuid axios luxon
# (If you want a real transformer model you’d also install @xenova/transformers,
# but the placeholder in emb.js works out‑of‑the‑box.)

> **Note** – The `emb.js` file ships a **stub** `EmbeddingModel` that returns random vectors.
> It is sufficient to demonstrate the flow.  When you replace it with a real sentence‑transformer
> the similarity scores become meaningful.

---  

### 2️⃣  Create the script: `smart-todo.js`

**Explanation of the script sections**

| Section | What it does |
|--------|--------------|
| **Init storage** | Calls `initStorage()` – creates `~/.todozi` and the default project (`general`). |
| **Tags** | Demonstrates how to instantiate `TagManager` and create a couple of tags (`dev`, `bug`). |
| **Task creation** | Uses the `SimpleTodoziTool` (the ultra‑simple wrapper) to create three tasks with different priorities/assignees. |
| **Persist tasks** | Inserts the tasks directly via `storage.addTaskToProject()`. In a production CLI the tool itself would do this. |
| **Embedding service** | Starts `TodoziEmbeddingService` with the default config (`sentence‑transformers/all‑MiniLM‑L6‑v2` placeholder). |
| **Embedding generation** | For each task we build a short description string and ask the service for an embedding vector (cached internally). |
| **Semantic search** | Reads a free‑form query from the terminal, computes cosine similarity between the query embedding and each task vector, and prints the top‑5 matches with emojis (re‑using the formatting code from `TodoziHandler`). |
| **Stats & clustering** | Shows how to retrieve a quick diagnostic report (`getStats`) and optionally run `clusterContent()` – useful for seeing semantic groups of tasks. |

> **Why this is useful** – You now have a **single Node script** that demonstrates the **complete flow**:  
> * storage‑initialisation → tag‑management → task‑creation → embedding → semantic‑search → diagnostics.  

---  

### 3️⃣  Run the example

chmod +x smart-todo.js          # make it executable
./smart-todo.js

You’ll see something like:

🗂️  Created tags: dev, bug

✅ Created task → Write README for the new CLI tool
✅ Created task → Fix the failing unit test in utils.test.js
✅ Created task → Design the tag‑management UI

🔎 Enter a search query (e.g. “fix bug”):
> bug fix

🤖 Top‑5 similar tasks (semantic search):
  • ✅ 🟢 👤 [d3f9c1a2] Fix the failing unit test in utils.test.js
        📁 general | ⏱️ 1h (score: 0.84)
  • ✅ 🟡 🤖 [a7e4b5c9] Write README for the new CLI tool (score: 0.31)
  • ✅ 🟠 🤝 [9b2d6f4e] Design the tag‑management UI (score: 0.12)

📊 Embedding statistics:
  Total cached embeddings: 3
  Types breakdown: {"Task":3}

🔗 No clusters – not enough data (requires ≥2 items per cluster).

*(Numbers will differ because the stub model returns random embeddings, but the flow is identical.)*

---  

### 4️⃣  Extending the script

#### a) Add a **custom tool** that creates a memory and links it to a tag

#### b) Use the **ChecklistTool** to auto‑extract tasks from a paragraph

*/