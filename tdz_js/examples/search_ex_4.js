/*********************************************************************
 *  example-4-search-all.js
 *
 *  Demonstrates a “search‑all” workflow:
 *   • Load the persisted data from Todozi storage.
 *   • Build a unified index with SearchEngine.
 *   • Run a query that mixes full‑text matching and embeddings.
 *   • Restrict the output to a subset of data‑types.
 *
 *  Prerequisites
 *   – The Todozi storage has been initialised (run `todozi init` once).
 *   – At least a few tasks / memories / ideas / errors / training records
 *     already exist in your `~/.todozi` directory.
 *
 *  Usage
 *      node example-4-search-all.js "<your query>" [types]
 *
 *  where <your query>  – free‑form search string.
 *        [types]       – optional comma‑separated list of types to keep:
 *                        tasks, memories, ideas, errors, training
 *                        (default = all)
 *
 *********************************************************************/

import { Storage } from "./storage.js";
import { SearchEngine, SearchOptions, SearchDataType } from "./search.js";
import {
  listMemories,
  listIdeas,
  listErrors,
  listTrainingData,
} from "./todozi.js";   // these are thin wrappers around storage

// ------------------------------------------------------------------
// 1️⃣  Helper – pretty‑print a single result block
// ------------------------------------------------------------------
function printBlock(title, items, formatter) {
  if (!items.length) return;
  console.log(`\n${title} (${items.length}):`);
  console.log("─".repeat(title.length + items.length.toString().length + 4));
  const limit = Math.min(5, items.length); // show at most 5 examples per block
  for (let i = 0; i < limit; i++) {
    console.log(`  ${formatter(items[i])}`);
  }
  if (items.length > limit) {
    console.log(`  … and ${items.length - limit} more`);
  }
}

// ------------------------------------------------------------------
// 2️⃣  Main async driver
// ------------------------------------------------------------------
async function main() {
  // --------------------------------------------------------------
  //   Parse CLI arguments
  // --------------------------------------------------------------
  const [, , rawQuery, rawTypes] = process.argv;
  if (!rawQuery) {
    console.error("❌  Usage: node example-4-search-all.js \"<query>\" [types]");
    process.exit(1);
  }
  const query = rawQuery.trim();

  // If the caller supplies a CSV list of data‑types we honour it,
  // otherwise we search *all* types.
  const typeSet = rawTypes
    ? new Set(
        rawTypes
          .split(",")
          .map((s) => s.trim().toLowerCase())
          .filter(Boolean)
      )
    : new Set(["tasks", "memories", "ideas", "errors", "training"]);

  // --------------------------------------------------------------
  //   Initialise storage – the same class the CLI uses internally
  // --------------------------------------------------------------
  const storage = await Storage.new();

  // --------------------------------------------------------------
  //   Load every kind of object from the disk
  // --------------------------------------------------------------
  console.log("🔎  Loading data from storage …");
  const [
    tasks,
    memories,
    ideas,
    errors,
    trainingData,
  ] = await Promise.all([
    storage.listTasksAcrossProjects({}), // ← all tasks
    listMemories(),                     // ← all memories
    listIdeas(),                        // ← all ideas
    listErrors(),                       // ← all errors
    listTrainingData(),                 // ← all training records
  ]);

  // --------------------------------------------------------------
  //   Build the unified index
  // --------------------------------------------------------------
  const searchEngine = new SearchEngine();
  searchEngine.updateIndex({
    tasks,
    memories,
    ideas,
    errors,
    trainingData,
  });
  console.log(
    `✅  Indexed ${tasks.length} tasks, ${memories.length} memories, ${ideas.length} ideas, ${errors.length} errors, ${trainingData.length} training records.`
  );

  // --------------------------------------------------------------
  //   Create a SearchOptions object.
  //   • limit   – how many hits per category (default 50, we use 20)
  //   • dataTypes – we will filter later, but the engine still needs the
  //                 full list so that scores are comparable across types.
  // --------------------------------------------------------------
  const options = new SearchOptions();
  options.limit = 20; // show at most 20 hits per type
  // Leaving `options.dataTypes` as `null` tells the engine to keep everything.
  // We’ll prune the result set ourselves (see step 6).

  // --------------------------------------------------------------
  //   Run the search
  // --------------------------------------------------------------
  console.log(`🔎  Running query → "${query}"`);
  const rawResults = searchEngine.search(query, options);

  // --------------------------------------------------------------
  //   5️⃣  Helper – turn a raw result entry into a short string.
  // --------------------------------------------------------------
  const formatTask = (r) => `${r.task.action} [${r.task.status}]`;
  const formatMemory = (r) => `${r.memory.moment}: ${r.memory.meaning}`;
  const formatIdea = (r) => `${r.idea.idea}`;
  const formatError = (r) => `${r.error.title} (${r.error.severity})`;
  const formatTraining = (r) => `${r.trainingData.prompt} (${r.trainingData.dataType})`;

  // --------------------------------------------------------------
  //   6️⃣  Filter according to the optional `types=` CSV list.
  // --------------------------------------------------------------
  const results = {
    tasks: typeSet.has("tasks") ? rawResults.taskResults : [],
    memories: typeSet.has("memories") ? rawResults.memoryResults : [],
    ideas: typeSet.has("ideas") ? rawResults.ideaResults : [],
    errors: typeSet.has("errors") ? rawResults.errorResults : [],
    training: typeSet.has("training") ? rawResults.trainingResults : [],
  };

  // --------------------------------------------------------------
  //   7️⃣  Pretty‑print the result blocks
  // --------------------------------------------------------------
  if (
    results.tasks.length ||
    results.memories.length ||
    results.ideas.length ||
    results.errors.length ||
    results.training.length
  ) {
    console.log("\n=== SEARCH RESULTS ===");
    printBlock("🗂️  Tasks", results.tasks, formatTask);
    printBlock("🧠  Memories", results.memories, formatMemory);
    printBlock("💡  Ideas", results.ideas, formatIdea);
    printBlock("❌  Errors", results.errors, formatError);
    printBlock("📚  Training", results.training, formatTraining);
  } else {
    console.log("\n❌  No matching items found.");
  }

  // --------------------------------------------------------------
  //   8️⃣  Bonus – show a small statistics summary
  // --------------------------------------------------------------
  const totalFound =
    results.tasks.length +
    results.memories.length +
    results.ideas.length +
    results.errors.length +
    results.training.length;
  console.log(`\n✅  Total hits: ${totalFound}`);
}

// ------------------------------------------------------------------
//  Run the driver
// ------------------------------------------------------------------
main().catch((e) => {
  console.error("💥  Unexpected error:", e);
  process.exit(1);
});

/*
## Example 4 – Unified “Search‑All” with `SearchEngine`
**Goal** – Show how a CLI command (or any other entry‑point) can

1. load all Todozi data (tasks, memories, ideas, errors, training data) from the
   persistence layer,  
2. feed that data into a single `SearchEngine` instance,  
3. run a semantic‑plus‑keyword search with custom `SearchOptions`, and  
4. print a nicely‑formatted, filtered result set.

The example is completely self‑contained; you can drop the file into the
project root and run it with `node example‑4-search‑all.js`.  
(It re‑uses the core classes you already have in `search.js`, `storage.js`,
`todozi.js`, … – no extra dependencies are required.)

### What the script does – step‑by‑step

| # | Action | Why it matters |
|---|--------|----------------|
| 1 | Parse the user’s query and an optional CSV list of data‑types (e.g. `tasks,ideas`). | Gives the caller fine‑grained control over which sections appear in the output. |
| 2 | Initialise the **Todozi** `Storage` class (the same object the CLI uses). | Guarantees that we read the exact same files/format the rest of the system expects. |
| 3 | Load *all* items of each domain (`tasks`, `memories`, …). | `SearchEngine` works on in‑memory arrays – we must supply them. |
| 4 | Call `searchEngine.updateIndex({…})`. | Populates the engine’s internal arrays (`this.tasks`, `this.memories`, …). |
| 5 | Build a `SearchOptions` instance (`limit = 20`). | Prevents the engine from returning thousands of hits for a very common word. |
| 6 | Run `searchEngine.search(query, options)`. | The heavy‑lifting: keyword matching, relevance scoring, optional date‑filters, sorting by score. |
| 7 | Convert raw results into short human‑readable strings (`formatTask`, …). | Makes the console output concise yet informative. |
| 8 | Apply the optional *type* filter (step 1) and pretty‑print each block. | Users can ask for “only tasks and ideas” without having to post‑process the raw JSON. |
| 9 | (Optional) display a tiny statistics line. | Quick feedback on how many hits were found overall. |

### How to adapt it

* **Custom scoring** – `SearchEngine.calculateRelevanceScore` can be tweaked (e.g. give higher weight to tags). Just edit the method in `search.js`.
* **Date range** – Add `options.since = new Date("2024‑01‑01").getTime();` to limit to recent items.
* **Different limit per type** – Call `searchEngine.search` once, then slice each array manually (`results.taskResults = results.taskResults.slice(0, 10);` etc.).
* **Integration with the existing CLI** – The logic inside `handleSearchAllCommand` (in `cli.js`) is almost identical; you can drop the `main()` body into that method if you want the same behaviour from the `todozi search-all` command.

---

#### Quick test

/ *
bash
# Add a couple of dummy tasks so we have something to find
todozi add task "Write example‑4 script; 1h; high; general; todo"
todozi add task "Review search engine code; 30m; medium; general; todo"

# Run the example
node example-4-search-all.js "search engine" tasks,ideas
*/