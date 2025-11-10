\nconst similar = await embService.findSimilarTasks("buy", 5);\npp(similar);\n

/*
#!/usr/bin/env node

 * Example 4 – End‑to‑End extraction → embedding → storage →
 *             human‑readable checklist
 *
 * Run with:
 *    node run-extract-embed.js sample-input.md
 */
import { resolve } from 'path';
import { fileURLToPath } from 'url';
import { readFile } from 'fs/promises';

import { extractContent } from './extract.js';
import { Storage } from './storage.js';
import { TodoziEmbeddingService } from './emb.js';
import { Task, Priority, Status, hashProjectName } from './models.js';
import { TagManager } from './tags.js';

// ---- Helpers ---------------------------------------------------------


 * Pretty‑print a JSON object (used for debugging).
 */
function pp(obj) {
  console.log(JSON.stringify(obj, null, 2));
}


 * Create a real `Task` instance from the raw extraction object.
 */
function buildTaskFromExtracted(extracted) {
  const task = new Task({
    userId: 'cli_user',
    action: extracted.action,
    time: extracted.time,
    priority: Priority.fromStr(extracted.priority),
    parentProject: extracted.project || 'general',
    status: Status.fromStr(extracted.status),
    assignee: extracted.assignee ?? null,
    tags: extracted.tags ?? [],
    dependencies: [], // not present in the demo payload
    contextNotes: null,
    progress: null,
  });
  return task;
}


 * Small wrapper that runs the whole pipeline.
 */
async function main() {
  // ---------------------------------------------------------------
  // 1️⃣  Get the file (or plain‑text) we want to process
  // ---------------------------------------------------------------
  const inputPath = process.argv[2];
  if (!inputPath) {
    console.error('❌  Usage: node run-extract-embed.js <file-or‑text>');
    process.exit(1);
  }

  const absolutePath = resolve(fileURLToPath(import.meta.url), '..', inputPath);
  const rawContent = await readFile(absolutePath, 'utf8');

  // ---------------------------------------------------------------
  // 2️⃣  Extract the structured data (tasks, memories, ideas, …)
  // ---------------------------------------------------------------
  console.log('🚀  Extracting...');
  const extractJson = await extractContent(
    rawContent,               // content
    null,                     // filePath (we already read the file)
    'json',                   // output format (we need the raw object)
    false                     // human checklist → we will do it ourselves later
  );
  const extracted = JSON.parse(extractJson);
  pp(extracted);   // <-- look at the raw output if you want

  // ---------------------------------------------------------------
  // 3️⃣  Initialise storage (creates ~/.todozi if missing)
  // ---------------------------------------------------------------
  const storage = await Storage.new();   // loads config, creates dirs etc.

  // ---------------------------------------------------------------
  // 4️⃣  Initialise the embedding service (loads default model)
  // ---------------------------------------------------------------
  const embService = await TodoziEmbeddingService.new(); // uses the default
  // (In a real deployment you could pass a custom config or cache)

  // ---------------------------------------------------------------
  // 5️⃣  Persist each extracted task, **embedding** it on the fly.
  // ---------------------------------------------------------------
  console.log('\n🗂️  Storing tasks with embeddings…');
  for (const extTask of extracted.tasks) {
    // 5a – turn the extraction DTO into a real `Task`
    const task = buildTaskFromExtracted(extTask);

    // 5b – generate an embedding (the stub returns a random vector)
    const embedding = await embService.generateEmbedding(
      `${task.action}\n${task.time}\n${task.priority}\n${task.parentProject}`
    );
    task.embeddingVector = embedding; // keep it for later queries

    // 5c – store the task in the project container
    await storage.addTaskToProject(task);
    console.log(`✅  Saved task "${task.action}" → project "${task.parentProject}"`);
  }

  // ---------------------------------------------------------------
  // 6️⃣  (Optional) Create the tags that appeared in the file
  // ---------------------------------------------------------------
  if (extracted.tasks?.some(t => t.tags?.length)) {
    console.log('\n🏷️  Creating / updating tags…');
    const tagMgr = TagManager.new();
    for (const t of extracted.tasks) {
      for (const tagName of t.tags) {
        // If the tag already exists we just bump its usage counter.
        const existing = tagMgr.getTagByName(tagName);
        if (existing) {
          await tagMgr.incrementTagUsage(tagName);
        } else {
          // Simple tag creation – you could also add a description or colour
          const newTagId = await tagMgr.createTag({ name: tagName });
          console.log(`   📌 Created tag "${tagName}" (id=${newTagId})`);
        }
      }
    }
  }

  // ---------------------------------------------------------------
  // 7️⃣  Generate a **human‑readable checklist** (Markdown file)
  // ---------------------------------------------------------------
  console.log('\n📋  Generating human checklist…');
  // Re‑use the same helper that the CLI uses – it lives inside extract.js
  // but is exported as a *private* method.  We simply call the same formatting
  // logic that the CLI command `todozi extract … --human` would have used.
  // The function is not exported, so we duplicate the minimal version here:
  function formatAsHumanChecklist(resp) {
    let checklist = '';
    checklist += '# 📋 Todozi Human Checklist\n\n';
    checklist += `Generated: ${new Date().toISOString()}\n\n`;
    checklist += '---\n\n';

    // Tasks
    if (resp.tasks?.length) {
      checklist += '## 📝 Tasks\n\n';
      resp.tasks.forEach(task => {
        checklist += `- [ ] **${task.action}**\n`;
        checklist += `  - 📁 Project: \`${task.project}\`\n`;
        checklist += `  - ⏱️ Time: \`${task.time}\`\n`;
        checklist += `  - 🎯 Priority: \`${task.priority}\`\n`;
        checklist += `  - 📊 Status: \`${task.status}\`\n`;
        if (task.assignee) {
          checklist += `  - 👤 Assignee: \`${task.assignee}\`\n`;
        }
        if (task.tags?.length) {
          checklist += `  - 🏷️ Tags: ${task.tags.map(t => \`\\\`${t}\\\`\`).join(', ')}\n`;
        }
        checklist += '\n';
      });
    }

    // Memories
    if (resp.memories?.length) {
      checklist += '## 🧠 Memories\n\n';
      resp.memories.forEach(m => {
        checklist += `- **${m.moment}**: ${m.meaning}\n`;
        checklist += `  - Reason: ${m.reason}\n`;
        checklist += `  - Importance: ${m.importance}\n`;
        checklist += `  - Term: ${m.term}\n\n`;
      });
    }

    // Ideas
    if (resp.ideas?.length) {
      checklist += '## 💡 Ideas\n\n';
      resp.ideas.forEach(i => {
        checklist += `- **${i.idea}** (${i.importance})\n`;
        checklist += `  - Share: ${i.share}\n\n`;
      });
    }

    // Errors
    if (resp.errors?.length) {
      checklist += '## ❌ Errors\n\n';
      resp.errors.forEach(e => {
        checklist += `- **${e.title}** – ${e.description}\n`;
        checklist += `  - Severity: ${e.severity} | Category: ${e.category}\n\n`;
      });
    }

    // Summary
    checklist += '\n---\n\n';
    checklist += `## 📊 Summary\n\n`;
    checklist += `- Total Tasks: **${resp.tasks?.length ?? 0}**\n`;
    checklist += `- Total Memories: **${resp.memories?.length ?? 0}**\n`;
    checklist += `- Total Ideas: **${resp.ideas?.length ?? 0}**\n`;
    checklist += `- Total Errors: **${resp.errors?.length ?? 0}**\n`;

    return checklist;
  }

  const checklistMarkdown = formatAsHumanChecklist(extracted);
  const timestamp = new Date()
    .toISOString()
    .replace(/[:.]/g, '-')
    .slice(0, -5);
  const checklistFile = `todozi_checklist_plan_${timestamp}.md`;

  await import('fs/promises').then(({ writeFile }) =>
    writeFile(checklistFile, checklistMarkdown, 'utf8')
  );
  console.log(`✅  Checklist written to ${checklistFile}`);

  console.log('\n🎉  All done!  You can now:');
  console.log('  • Open the markdown checklist and tick off items.');
  console.log('  • Run `todozi tasks list` (or similar) to see the persisted tasks.');
  console.log('  • Use the embedding service later for semantic search, e.g.:');
  console.log('        const results = await embService.findSimilarTasks("run", 5);');
}

// --------------------------------------------------------------------
main().catch(err => {
  console.error('❌  Something went wrong:', err);
  process.exit(1);
});

/ *
## Example 4 – End‑to‑End **“Extract → Embed → Store → Human‑readable Checklist”**  

This example shows how you can take an arbitrary piece of text (or a file), let **`extract.js`** turn it into Todo‑zi objects (tasks, memories, ideas, …), automatically embed the extracted tasks with the **Todozi Embedding Service**, persist everything with the **storage layer**, and finally write a nice markdown checklist that a human can read or print.

> **What you’ll see**
> * A tiny CLI wrapper (`run‑extract‑embed.js`) that orchestrates the whole flow.  
> * A sample input file (`sample‑input.md`) containing a mix of Todo‑zi tags.  
> * How the extracted JSON is turned into real `Task` objects, embedded and saved.  
> * The generation of a *human checklist* (`todozi_checklist_…md`).  

---

### 1️⃣  Project layout (only the parts we need for the demo)

/ *
.
├─ extract.js          ← code you posted (extractContent / strategyContent)
├─ models.js           ← core data‑models (Task, Priority, Status, …)
├─ storage.js          ← persistence helpers (Storage, initStorage, …)
├─ emb.js              ← embedding service (TodoziEmbeddingService)
├─ tags.js             ← Tag manager (optional, shown later)
├─ sample-input.md     ← our test data (see below)
├─ run-extract-embed.js← **Example 4** – the script we will run
└─ package.json        ← make sure dependencies are installed

/ *
> **⚠️  Prerequisites**:  
> * Node ≥ 18 (for native ESM support).  
> * Install the required packages – you already have the source files, but you’ll also need a few runtime deps:

npm i uuid luxon crypto fs-extra   # core deps (already used in the code)
# The embedding service is a stub – no external AI library is required for the demo

---

### 2️⃣  Sample input (`sample‑input.md`)

# My weekend plan

<todozi>Buy groceries; tomorrow 10am; high; personal; todo; ai; groceries,shopping</todozi>

<todozi>Read "Clean Code"; Saturday 18:00; medium; personal; todo; human; reading</todozi>

<memory>
standard; 2024‑11‑05 09:15; Ran a 5 km run; Feeling energized; high; short; run,exercise
</memory>

<idea>
Add automated backups to my dotfiles repo; private; high
</idea>

<error>
Database connection failed; Could not reach DB at 10.0.0.5; high; network; backup_service
</error>


*The file mixes the different Todo‑zi tags that the extraction code knows how to parse.*

---

### 3️⃣  The “driver” script (`run‑extract‑embed.js`)

**What the script does, step‑by‑step**

| Step | Description |
|------|-------------|
| **1** | Loads the raw markdown (or any string) you want to extract from. |
| **2** | Calls `extractContent()` from *extract.js* – this contacts the Todo‑zi API (mocked by the stub) and returns a JSON structure containing `tasks`, `memories`, `ideas`, `errors`, … |
| **3** | Initializes the **storage layer** (`Storage.new()`). This creates `~/.todozi` if it does not exist and loads the configuration file. |
| **4** | Starts the **embedding service** (`TodoziEmbeddingService`). The default model is `"sentence‑transformers/all‑MiniLM‑L6‑v2"` (placeholder implementation returns a random vector). |
| **5** | For each extracted task: <br>• Convert the DTO into a real `Task` instance (validated enums). <br>• Generate an embedding using the service and attach it to the task. <br>• Persist the task in the appropriate project container (`addTaskToProject`). |
| **6** *(optional)* | Walks through all tags that appeared in the input and creates them (or increments usage) via `TagManager`. |
| **7** | Re‑creates the **human‑readable checklist** that the CLI would have produced (`--human`). The file is written as `todozi_checklist_plan_<timestamp>.md`. |

> **Tip** – The embedding step is totally optional for a simple “store‑and‑list” workflow. If you skip it, just omit `embeddingVector = …` and `TodoziEmbeddingService` calls.

---

### 4️⃣  Run the example

# Make sure the script is executable (optional)
chmod +x run-extract-embed.js

# Run it – it will read sample-input.md, extract, embed & store
node run-extract-embed.js sample-input.md

**Typical console output**