// ------------------------------------------------------------
//  example4.js
// ------------------------------------------------------------
// This script demonstrates:
//   • parsing <tdz> blocks from a markdown file
//   • executing the generated commands against a Todozi server
//   • performing a semantic search on the created data
// ------------------------------------------------------------

/* eslint-disable no-console */
import fs from 'fs/promises';
import path from 'path';
import { fileURLToPath } from 'url';

// ----  Todozi core helpers (from the repo) ------------------
import {
  processTdzCommands,        // from tdz.js
  parseTdzCommand,           // you can also call it directly if you want
} from './tdz.js';

import {
  TodoziEmbeddingTool,       // from emb.js
  TodoziEmbeddingConfig,
} from './emb.js';

// ------------------------------------------------------------
// Helper: locate the markdown file next to this script
// ------------------------------------------------------------
const __filename = fileURLToPath(import.meta.url);
const __dirname  = path.dirname(__filename);
const INPUT_MD   = path.join(__dirname, 'sample-with-tdz.md');

// ------------------------------------------------------------
// 1️⃣  Load the markdown content that contains <tdz> blocks
// ------------------------------------------------------------
async function loadInputFile() {
  try {
    const data = await fs.readFile(INPUT_MD, 'utf8');
    console.log('📄 Loaded input file →', INPUT_MD);
    return data;
  } catch (err) {
    console.error('❌ Could not read input file:', err);
    process.exit(1);
  }
}

// ------------------------------------------------------------
// 2️⃣  Parse the <tdz> blocks into TdzCommand objects
// ------------------------------------------------------------
function showParsedCommands(commands) {
  console.log('\n🔎 Parsed commands (', commands.length, ')');
  commands.forEach((c, i) => {
    console.log(`  ${i + 1}. ${c.command.toUpperCase()} → target: ${c.target}`);
    console.log('     parameters :', c.parameters);
    console.log('     options    :', c.options);
  });
}

// ------------------------------------------------------------
// 3️⃣  Execute the commands against the Todozi API
// ------------------------------------------------------------
async function runCommands(commands) {
  // 👉 TODO: change these to point at your real Todozi server
  const BASE_URL = 'http://127.0.0.1:8636/api';
  const API_KEY  = null;          // or set a real key if your server requires auth

  try {
    const results = await processTdzCommands(
      // NOTE: `processTdzCommands` expects the **original text**,
      // not the already‑parsed array.  It will call `parseTdzCommand`
      // internally, but we already did that for the demo output,
      // so we just pass the array manually.
      // To keep the flow simple we call the low‑level helper:
      //   executeTdzCommand(command, BASE_URL, API_KEY)
      // inside a small wrapper.
      '',
      BASE_URL,
      API_KEY
    );

    // The above call would do nothing because we passed an empty string.
    // Instead we iterate ourselves:
    const execResults = [];
    for (const cmd of commands) {
      // eslint-disable-next-line no-await-in-loop
      const r = await (await import('./tdz.js')).executeTdzCommand(cmd, BASE_URL, API_KEY);
      execResults.push(r);
    }
    console.log('\n✅ All commands completed – server responses:');
    execResults.forEach((r, i) => console.log(`  ${i + 1}.`, JSON.stringify(r, null, 2)));
    return execResults;
  } catch (e) {
    console.error('❌ Error while executing commands:', e);
    process.exit(1);
  }
}

// ------------------------------------------------------------
// 4️⃣  Initialise the embedding tool (uses the default model)
// ------------------------------------------------------------
async function initialiseEmbeddingTool() {
  const cfg   = TodoziEmbeddingConfig.default();      // you can tweak the thresholds if you like
  const tool  = await TodoziEmbeddingTool.new(cfg);
  await tool.initialize();                           // loads the sentence‑transformer model
  console.log('\n🤖 Embedding tool ready – model loaded.');
  return tool;
}

// ------------------------------------------------------------
// 5️⃣  Perform a semantic search on the freshly created tasks
// ------------------------------------------------------------
async function semanticSearch(tool, query) {
  const limit = 5;
  console.log(`\n🔎 Semantic search for: "${query}" (max ${limit} results)…`);
  const result = await tool.service.findSimilarTasks(query, limit);
  if (result.length === 0) {
    console.log('🙅‍♀️ No similar tasks found.');
    return;
  }

  console.log('\n✨ Top matches:');
  result.forEach((item, i) => {
    console.log(`  ${i + 1}. ${item.text_content.split('\n')[0] || '(no title)'}`);
    console.log(`     → similarity: ${(item.similarity_score * 100).toFixed(1)}%`);
    console.log(`     → id        : ${item.content_id}`);
  });
}

// ------------------------------------------------------------
// Main driver – orchestrates everything
// ------------------------------------------------------------
(async () => {
  const markdown = await loadInputFile();

  // ----------------------------------------------------------
  // 1️⃣  Parse the <tdz> blocks
  // ----------------------------------------------------------
  const commands = parseTdzCommand(markdown);
  showParsedCommands(commands);

  // ----------------------------------------------------------
  // 2️⃣  Execute them against the server
  // ----------------------------------------------------------
  await runCommands(commands);

  // ----------------------------------------------------------
  // 3️⃣  Initialise the embedding layer
  // ----------------------------------------------------------
  const embeddingTool = await initialiseEmbeddingTool();

  // ----------------------------------------------------------
  // 4️⃣  Search – e.g. “quick bug fix”
  // ----------------------------------------------------------
  await semanticSearch(embeddingTool, 'quick bug fix');
})();

/ *
## Example 4 – Parse `<tdz>` tags, run the commands and then use the **Todozi Embedding Tool** for a semantic search  

In many real‑world workflows you will have a markdown (or any text) file that contains a mixture of human‑written prose and special Todozi commands wrapped in `<tdz> … </tdz>`.  
The code in **`tdz.js`** can turn those snippets into `TdzCommand` objects, call the proper HTTP endpoint and return the JSON result.  
When you also want to **search** the created tasks (or tags, memories, ideas …) you can immediately reuse the **embedding service** that lives in **`emb.js`**.

Below is a **complete, self‑contained example** that shows:

1. **Reading a file** that contains several `<tdz>` commands.  
2. **Parsing** those commands with `parseTdzCommand`.  
3. **Executing** each command with `processTdzCommands`.  
4. **Loading** the embedding service (`TodoziEmbeddingTool`).  
5. **Running a semantic search** for “quick bug fix” and printing the most‑relevant tasks.  

> **What you need** – the example assumes the Todozi repository (the files you posted) is installed locally.  
> It also needs `node-fetch` (or any `fetch` polyfill) – you can add it with `npm i node-fetch`.  

---  

### 1️⃣  `example4.js` – the driver script

---

### 2️⃣  `sample-with-tdz.md` –  input file containing `<tdz>` commands

/ *
markdown
# My sprint backlog

Below are the tasks we need for the next two weeks.

<tdz>create; task; action=Fix login bug; time=2h; priority=high; project=frontend; status=todo; assignee=ai; tags=bug,login</tdz>

<tdz>create; task; action=Write unit tests for payment service; time=4h; priority=medium; project=backend; status=todo; assignee=human; tags=testing,payment</tdz>

<tdz>create; task; action=Update README with deployment instructions; time=1h; priority=low; project=docs; status=todo; assignee=collaborative; tags=documentation</tdz>

<tdz>list; tasks</tdz>

<tdz>search; tasks; q=login</tdz>

/ *
*Each `<tdz>` block follows the format used by `parseTdzCommand`:*  

<command> ; <target> ; <key>=<value> ; …  

The parser turns them into a `TdzCommand` instance that the executor knows how to translate into a concrete HTTP request (see `getEndpointPath` in `tdz.js`).  

---

### 3️⃣  Walk‑through of the script  

| Step | What happens | Key functions |
|------|--------------|----------------|
| **Load markdown** | Reads `sample-with-tdz.md` from disk. | `fs.readFile` |
| **Parse commands** | `parseTdzCommand` extracts each `<tdz>` block, splits by `;`, recognises `key=value` pairs → builds `TdzCommand` objects. | `parseTdzCommand` in *tdz.js* |
| **Show parsed commands** | Simple console output – handy for debugging. | `showParsedCommands` helper |
| **Execute commands** | For each command we call `executeTdzCommand` which: <br>  • builds the final REST endpoint (`getEndpointPath`) <br>  • selects the correct HTTP verb (GET/POST/PUT/DELETE) <br>  • adds `X‑API‑Key` header if supplied <br>  • builds JSON body for `create`/`update`/`run` commands <br>  • sends the request with `fetch` and returns the parsed JSON. | `executeTdzCommand` (tdz.js) |
| **Initialise embedding tool** | `TodoziEmbeddingTool.new()` creates a service instance, then `initialize()` loads the default sentence‑transformer model (`all‑MiniLM‑L6‑v2`). | `TodoziEmbeddingTool` & `TodoziEmbeddingService` (emb.js) |
| **Semantic search** | Calls `service.findSimilarTasks('quick bug fix', 5)` → <br> 1️⃣ generates an embedding for the query <br> 2️⃣ scans the in‑memory cache for tasks (the `addTask` calls from the `create` commands have stored embeddings) <br> 3️⃣ returns the top‑N most similar tasks ordered by cosine similarity. | `TodoziEmbeddingService.findSimilarTasks` |
| **Print results** | Nicely formatted list with similarity % and the Todozi task ID. | `semanticSearch` helper |

> **Note:** The cache lives only in memory for this demo run. In a real deployment the service would persist embeddings to a database or a vector store (e.g. Pinecone, Qdrant, or a local SQLite + Faiss index).  

---

### 4️⃣  How to run the example  

# 1️⃣  Install the tiny dependencies (fetch polyfill & luxon)
npm install node-fetch@2 luxon uuid

# 2️⃣  Make sure the Todozi server is running on the address used in the script
#     (default in the example: http://127.0.0.1:8636/api)
#     If you don't have the server, start it with:
#       node cli.js server start --host 127.0.0.1 --port 8636
#     (or adjust BASE_URL in the script)

# 3️⃣  Run the demo
node example4.js

You should see an output similar to: