import path from "path";
import { fileURLToPath } from "url";

// ---- 1️⃣  Load repository modules (relative to this file) --------------------
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const {
  // storage helpers
  Storage,
  initStorage,
} = await import(path.join(__dirname, "storage.js"));
const {
  // parsing helpers (create objects from <todozi> … </todozi> tags)
  parseTodoziFormat,
  parseMemoryFormat,
  parseIdeaFormat,
} = await import(path.join(__dirname, "todozi.js"));
const {
  // embedding engine (the heavy‑weight part)
  TodoziEmbeddingService,
  TodoziEmbeddingConfig,
} = await import(path.join(__dirname, "emb.js"));

// ---- 2️⃣  Helper: quick‑and‑dirty “add sample data” -------------------------
async function seedSampleData(storage) {
  // ── a) 3 tasks ---------------------------------------------------------
  const taskTexts = [
    `<todozi>Buy coffee beans; tomorrow morning; high; personal; todo; human; coffee,shopping</todozi>`,
    `<todozi>Refactor auth middleware; next sprint; critical; backend; todo; ai; code,backend</todozi>`,
    `<todozi>Write blog post about embedding search; next week; medium; personal; todo; human; writing,blog</todozi>`,
  ];

  for (const txt of taskTexts) {
    const task = parseTodoziFormat(txt);
    await storage.addTaskToProject(task); // automatically creates the default project if needed
  }

  // ── b) 2 memories -------------------------------------------------------
  const memTexts = [
    // standard memory
    `<memory>standard; 2024‑10‑01; First prototype of Todo‑zi ran; prototype; high; short</memory>`,
    // emotional memory (AI‑only)
    `<memory>happy; 2024‑09‑15; Got praised for redesigning the UI; appreciation; medium; long</memory>`,
  ];

  for (const txt of memTexts) {
    const mem = parseMemoryFormat(txt, "demo_user");
    // The repository does *not* expose a direct “saveMemory” helper,
    // but the storage layer would normally have one.  For the demo we
    // simply push it into a JSON file – mimic the real implementation:
    const memFile = path.join(
      (await storage.getStorageDir()),
      "memories",
      `${mem.id}.json`
    );
    await import("fs/promises").then((fs) => fs.writeFile(memFile, JSON.stringify(mem, null, 2)));
  }

  // ── c) 2 ideas ---------------------------------------------------------
  const ideaTexts = [
    `<idea>Introduce vector‑based tag recommendations; team; high</idea>`,
    `<idea>Create a VS‑Code extension for quick Todo‑zi entry; private; medium</idea>`,
  ];

  for (const txt of ideaTexts) {
    const idea = parseIdeaFormat(txt);
    const ideaFile = path.join(
      (await storage.getStorageDir()),
      "ideas",
      `${idea.id}.json`
    );
    await import("fs/promises").then((fs) => fs.writeFile(ideaFile, JSON.stringify(idea, null, 2)));
  }

  console.log("✅ Sample data seeded.");
}

// ---- 3️⃣  Main routine ----------------------------------------------------
async function main() {
  const query = process.argv[2] ?? "embedding search";

  // ---- a) Make sure the Todo‑zi folder structure exists -----------------
  await initStorage(); // creates ~/.todozi and default dirs if missing
  const storage = await Storage.new();

  // ---- b) Seed the demo data (run only once) ----------------------------
  // If you run the script repeatedly you will get duplicate entries.
  // In a real world scenario you’d guard this with a flag or a check.
  await seedSampleData(storage);

  // ---- c) Load / initialise the embedding service -----------------------
  const embedConfig = TodoziEmbeddingConfig.default();
  const embedService = await TodoziEmbeddingService.new(embedConfig);

  // ---- d) Pull all content we want to index -----------------------------
  // (In a production CLI this is done via `storage.listTasksAcrossProjects`,
  //  `listMemories`, `listIdeas`, etc.)
  const tasks = await storage.listTasksAcrossProjects({}); // all projects, no filter
  const memories = []; // we wrote them manually above – read them back:
  const memDir = path.join((await storage.getStorageDir()), "memories");
  for (const f of await import("fs/promises").then((fs) => fs.readdir(memDir))) {
    if (f.endsWith(".json")) {
      const mem = JSON.parse(
        await import("fs/promises").then((fs) => fs.readFile(path.join(memDir, f), "utf8"))
      );
      memories.push(mem);
    }
  }
  const ideas = []; // same for ideas
  const ideaDir = path.join((await storage.getStorageDir()), "ideas");
  for (const f of await import("fs/promises").then((fs) => fs.readdir(ideaDir))) {
    if (f.endsWith(".json")) {
      const idea = JSON.parse(
        await import("fs/promises").then((fs) => fs.readFile(path.join(ideaDir, f), "utf8"))
      );
      ideas.push(idea);
    }
  }

  // ---- e) Build the ChatContent structure expected by SearchEngine -------
  const chatContent = {
    tasks,
    memories,
    ideas,
    agentAssignments: [], // not needed for this demo
    codeChunks: [],       // not needed for this demo
    errors: [],           // not needed for this demo
    trainingData: [],     // not needed for this demo
    feelings: [],         // not needed for this demo
  };

  // ---- f) Index everything ------------------------------------------------
  embedService.updateIndex(chatContent);
  console.log("🔎 Index built – ready for semantic search.");

  // ---- g) Run the unified “search‑all” ------------------------------------
  const searchOptions = {
    limit: 20,
    dataTypes: null,
    since: null,
    until: null,
  };

  const results = embedService.search(query, searchOptions);

  // ---- h) Pretty‑print the results (mirrors the CLI output) -------------
  console.log("\n=== 🔎  Unified Semantic Search Results ===");
  console.log(`Query : "${query}"`);
  console.log(`Found ${results.taskResults.length} tasks`);
  console.log(`Found ${results.memoryResults.length} memories`);
  console.log(`Found ${results.ideaResults.length} ideas\n`);

  // Helper to truncate long strings
  const truncate = (s, max = 70) => (s.length > max ? s.slice(0, max - 3) + "…" : s);

  if (results.taskResults.length) {
    console.log("📋 Tasks:");
    results.taskResults.slice(0, 5).forEach((r) => {
      console.log(` • ${truncate(r.action)} (priority=${r.priority}, status=${r.status})`);
    });
    if (results.taskResults.length > 5) {
      console.log(`   …and ${results.taskResults.length - 5} more`);
    }
    console.log("");
  }

  if (results.memoryResults.length) {
    console.log("🧠 Memories:");
    results.memoryResults.slice(0, 3).forEach((r) => {
      console.log(` • ${r.moment}: ${truncate(r.meaning)}`);
    });
    if (results.memoryResults.length > 3) {
      console.log(`   …and ${results.memoryResults.length - 3} more`);
    }
    console.log("");
  }

  if (results.ideaResults.length) {
    console.log("💡 Ideas:");
    results.ideaResults.slice(0, 3).forEach((r) => {
      console.log(` • ${truncate(r.idea)}`);
    });
    if (results.ideaResults.length > 3) {
      console.log(`   …and ${results.ideaResults.length - 3} more`);
    }
    console.log("");
  }

  if (
    !results.taskResults.length &&
    !results.memoryResults.length &&
    !results.ideaResults.length
  ) {
    console.log("❌ No matching results – try a different query.");
  }

  console.log("=== ✅  Search finished ===\n");
}

// -------------------------------------------------------------------------
main().catch((e) => {
  console.error("🚨  Example failed:");
  console.error(e);
  process.exit(1);
});

/*
## Example 4 – Using Todozi’s Embedding Engine for Semantic “Search‑All”

**Goal**  
Create a tiny script that

1. Initialises the Todo‑zi storage layer.  
2. Adds a few *tasks*, *memories* and *ideas* (using the high‑level parsers from `todozi.js`).  
3. Instantiates the **embedding service** (`TodoziEmbeddingService` from *emb.js*).  
4. Indexes the new content.  
5. Runs a **single unified semantic search** across all data‑types (tasks + memories + ideas) – the same operation the CLI performs for `todozi searchall`.

The script can be run directly from the command line and can be used as a template for custom tooling (e.g., a web‑hook, a VS‑Code extension, or a CI pipeline that needs a quick “what‑does‑this‑mean” lookup).

---

### 1️⃣  Prerequisites  

| Tool | Version | Why? |
|------|---------|------|
| **Node.js** | `>=18` | All the Todo‑zi source files use native ESM (`import … from`) and `async/await`. |
| **npm** | any | To install the tiny runtime dependencies required for the example. |
| **Git** (optional) | – | To clone the Todo‑zi repo if you don’t already have the sources. |

**Install the only external dependency that the example needs**

/ *
bash
npm i uuid@9.0.1   # used by a few model classes

/ *
> *All other modules (`emb.js`, `storage.js`, `todozi.js`, …) are part of the repository, so no extra packages are required.*

---

### 2️⃣  Directory Layout (for the example)

my‑todozi‑demo/
├─ emb.js           ← copy from the repository (embedding engine)
├─ storage.js       ← copy from the repository
├─ todozi.js        ← copy from the repository (parsers)
├─ models.js        ← copy from the repository (domain models)
├─ tags.js          ← copy from the repository (tag manager – not used here)
├─ example‑4.js     ← **the script you are about to run**
└─ package.json

---

### 3️⃣  `example‑4.js` – Full Source

**What the script does (step‑by‑step)**  

| Step | Code fragment | What happens |
|------|----------------|--------------|
| ① | `await initStorage();` | Creates `~/.todozi` and all sub‑folders if they do not exist. |
| ② | `await seedSampleData(storage);` | Parses three `<todozi>` tasks, two `<memory>` entries and two `<idea>` entries and writes them to the proper storage files. |
| ③ | `TodoziEmbeddingService.new()` | Loads the default Sentence‑Transformer model (`all‑MiniLM‑L6‑v2`).  The model is *lazy* – it is downloaded the first time you run the script. |
| ④ | `embedService.updateIndex(chatContent);` | Feeds every task/memory/idea into the service so that an internal vector cache is populated. |
| ⑤ | `embedService.search(query, options);` | Performs a **cosine‑similarity** search across *all* indexed vectors, returning `SearchResults` (exactly the shape used by the CLI). |
| ⑥ | Pretty‑print | Mirrors the output you would see with `todozi searchall "<your query>"`. |

---

### 4️⃣  Running the Example  

# 1️⃣ Make the file executable (optional)
chmod +x example-4.js

# 2️⃣ Run it – supply any query you like
node example-4.js "embedding search"

**First run**  
You will see the model being downloaded (≈ 200 MB) and a short progress bar printed by the underlying transformer library. After that the script prints something like:

✅ Sample data seeded.
🔎 Index built – ready for semantic search.

=== 🔎  Unified Semantic Search Results ===
Query : "embedding search"
Found 1 tasks
Found 0 memories
Found 1 ideas

📋 Tasks:
 • Write blog post about embedding search (priority=medium, status=todo)

💡 Ideas:
 • Introduce vector‑based tag recommendations

Feel free to try different queries, e.g.:

node example-4.js "coffee"
node example-4.js "refactor auth"
node example-4.js "VS Code extension"

Each time the script rebuilds the index (fast for a handful of items) and returns the most semantically similar objects.

---

### 5️⃣  Extending the Code – Add a Custom CLI Sub‑command  

Now that we have a **stand‑alone** embedding workflow, it is trivial to expose it as a first‑class Todo‑zi CLI command (e.g. `todozi embed-search "<query>"`).  

Below is a *minimal* addition to `cli.js` that re‑uses the logic from the example.

// cli.js  (add near the bottom of the file)
+  async handleEmbedSearchCommand(command) {
+    // command.type === 'EmbedSearch'  and command.query is a string
+    const query = command.query;
+
+    // 1️⃣ Initialise embedding service (same config as the example)
+    const embedConfig = TodoziEmbeddingConfig.default();
+    const embedService = await TodoziEmbeddingService.new(embedConfig);
+
+    // 2️⃣ Pull everything we want to search
+    const tasks = await this.storage.listTasksAcrossProjects({});
+    const memories = await listMemories();          // ← helper already in cli.js
+    const ideas = await listIdeas();                // ← helper already in cli.js
+
+    const chatContent = {
+      tasks,
+      memories,
+      ideas,
+      agentAssignments: [],
+      codeChunks: [],
+      errors: [],
+      trainingData: [],
+      feelings: []
+    };
+
+    // 3️⃣ Build the index
+    embedService.updateIndex(chatContent);
+
+    // 4️⃣ Run the search
+    const opts = { limit: 20 };
+    const results = embedService.search(query, opts);
+
+    // 5️⃣ Print – reuse the pretty printer from the example
+    console.log("\n=== 🔎  Embedding Search Results ===");
+    console.log(`Query: "${query}"`);
+    console.log(`Tasks   : ${results.taskResults.length}`);
+    console.log(`Memories: ${results.memoryResults.length}`);
+    console.log(`Ideas   : ${results.ideaResults.length}\n`);
+
+    // (you could add the same pretty‑printing loops as in example‑4.js)
+  }
+
+  // Register the new command in the main dispatcher (somewhere near the top)
+  async dispatch(command) {
+    // … existing switch …
+    case 'EmbedSearch':
+      await this.handleEmbedSearchCommand(command);
+      break;
+    // …
+  }

**How to invoke it**
*/