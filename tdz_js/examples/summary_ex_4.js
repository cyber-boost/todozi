// ---------------------------------------------------------------
// Example 4 – Full Summary + Tag + Embedding workflow
// ---------------------------------------------------------------

// 1️⃣  Import the pieces we need
//---------------------------------------------------------------
import { SummaryManager, parseSummaryFormat } from "../todozi/summary.js";
import { TagManager } from "../todozi/tags.js";
import {
  TodoziEmbeddingConfig,
  TodoziEmbeddingService,
  TodoziEmbeddingCache,
} from "../todozi/emb.js";

// 2️⃣  Helper to pretty‑print JSON objects
//---------------------------------------------------------------
function pp(obj) {
  console.log(JSON.stringify(obj, null, 2));
}

// ---------------------------------------------------------------
// MAIN (async IIFE so we can use await at top level)
// ---------------------------------------------------------------
(async () => {
  console.log("\n=== 📚 Todozi – Summary + Tag + Embedding demo ===\n");

  // -----------------------------------------------------------------
  //  A. Initialise the managers we will use
  // -----------------------------------------------------------------
  const summaryMgr = new SummaryManager();
  const tagMgr = TagManager.new();               // in‑memory only for this demo
  const embConfig = TodoziEmbeddingConfig.default();
  const embService = await TodoziEmbeddingService.new(embConfig);

  // -----------------------------------------------------------------
  //  B. Parse a raw <summary> block (as a user might type in the CLI)
  // -----------------------------------------------------------------
  const rawSummary = `
    <summary>
      Project completed successfully; high; Final project delivery;
      project,completion,success
    </summary>
  `;

  console.log("🔎 Parsing raw <summary> block …");
  const parsed = parseSummaryFormat(rawSummary);
  pp(parsed);
  // parsed now contains: { id, content, priority, context?, tags, … }

  // -----------------------------------------------------------------
  //  C. Store the summary via SummaryManager (adds timestamps & id)
  // -----------------------------------------------------------------
  console.log("\n🗂️  Storing summary …");
  const storedId = await summaryMgr.createSummary(parsed);
  const storedSummary = summaryMgr.getSummary(storedId);
  console.log(`✅ Summary stored with id = ${storedId}`);
  pp(storedSummary);

  // -----------------------------------------------------------------
  //  D. Ensure every tag used by the summary exists in TagManager
  // -----------------------------------------------------------------
  console.log("\n🏷️  Ensuring tags exist (create if missing) …");
  for (const tagName of storedSummary.tags) {
    // Try to find an existing tag with the same name
    let tag = tagMgr.getTagByName(tagName);
    if (!tag) {
      // Tag does not exist → create it
      const newTag = {
        name: tagName,
        description: `Auto‑generated tag for summary ${storedId}`,
      };
      const tagId = await tagMgr.createTag(newTag);
      tag = tagMgr.getTag(tagId);
      console.log(`   ➕ Created tag '${tagName}' (id=${tagId})`);
    } else {
      // Tag already exists → bump usage counter
      await tagMgr.incrementTagUsage(tagName);
      console.log(`   🔄 Tag '${tagName}' already exists – usage incremented`);
    }
  }

  // Show current tag state
  console.log("\n📦 All tags now in the manager:");
  pp(tagMgr.getAllTags());

  // -----------------------------------------------------------------
  //  E. Generate embeddings for the summary text and for each tag
  // -----------------------------------------------------------------
  console.log("\n🧠 Generating embeddings …");

  // 1️⃣ Summary embedding
  const summaryEmbedding = await embService.generateEmbedding(storedSummary.content);
  // Save it into the service's internal cache so we can search later
  embService.cache.set(
    `summary_${storedId}`,
    new TodoziEmbeddingCache(
      summaryEmbedding,
      "Summary",
      storedId,
      storedSummary.content,
      storedSummary.tags,
      new Date(),
      embConfig.cache_ttl_seconds
    )
  );
  console.log("   👉 Summary embedding cached.");

  // 2️⃣ Tag embeddings (one per tag)
  for (const tagName of storedSummary.tags) {
    const tagEmbedding = await embService.generateEmbedding(tagName);
    embService.cache.set(
      `tag_${tagName}`,
      new TodoziEmbeddingCache(
        tagEmbedding,
        "Tag",
        tagName,
        tagName,
        [tagName],
        new Date(),
        embConfig.cache_ttl_seconds
      )
    );
    console.log(`   👉 Tag '${tagName}' embedding cached.`);
  }

  // -----------------------------------------------------------------
  //  F. Demo a **semantic search** – find similar summaries/tags
  // -----------------------------------------------------------------
  const query = "final delivery of the project";
  console.log(`\n🔎 Semantic search for: "${query}"`);
  const semResults = await embService.semanticSearch(query, null, 5);
  console.log(`   📋 Found ${semResults.length} results:`);
  for (const r of semResults) {
    console.log(
      `   - [${r.content_type}] ${r.text_content.slice(0, 60)}… (score=${(
        r.similarity_score * 100
      ).toFixed(1)}%)`
    );
  }

  // -----------------------------------------------------------------
  //  G. Demo a **hybrid search** (semantic + keyword boost)
  // -----------------------------------------------------------------
  const hybridQuery = "project success";
  const keywords = ["project", "success"];
  console.log(`\n⚡ Hybrid search for: "${hybridQuery}" + keywords ${keywords}`);
  const hybridResults = await embService.hybridSearch(
    hybridQuery,
    keywords,
    null,
    0.6, // give 60 % weight to semantics, 40 % to keyword boost
    5
  );
  console.log(`   📋 Hybrid found ${hybridResults.length} results:`);
  for (const r of hybridResults) {
    console.log(
      `   - [${r.content_type}] ${r.text_content.slice(0, 60)}… (combined=${(
        r.similarity_score * 100
      ).toFixed(1)}%)`
    );
  }

  // -----------------------------------------------------------------
  //  H. Show clustering output (groups of similar items)
  // -----------------------------------------------------------------
  console.log("\n🔗 Running a quick clustering on cached items …");
  const clusters = await embService.clusterContent();
  console.log(`   📊 ${clusters.length} cluster(s) detected`);
  clusters.forEach((c, idx) => {
    console.log(`   #${idx + 1} – ${c.clusterSize} items, avgSim=${(
      c.averageSimilarity * 100
    ).toFixed(1)}%`);
    c.clusterItems.forEach((itm) => {
      console.log(`      • ${itm.content_type}: ${itm.text_content.slice(0, 40)}…`);
    });
  });

  // -----------------------------------------------------------------
  //  I. Wrap‑up
  // -----------------------------------------------------------------
  console.log("\n🎉 Demo finished! Everything is in‑memory – no files were written.");
  console.log(
    "   You can now experiment further: add more summaries, update tags, or call other methods on `embService`."
  );
})();

/*
Below is **Example 4** – a small “demo‑script” that ties together the **SummaryManager**, **TagManager**, and the **embedding / search** utilities (`emb.js`).  
It shows a common workflow you might use in a real Todozi installation:

1. **Parse a `<summary>` block** that a user typed in the CLI.  
2. **Create the summary** (the manager assigns an `id`, timestamps, and stores the tags).  
3. **Create / update the tags** that belong to the summary (via `TagManager`).  
4. **Generate embeddings** for the summary text and its tags (using `TodoziEmbeddingService`).  
5. **Run a few semantic / hybrid searches** to prove the data is searchable.  

> **What you need to run the script**  
> * Node ≥ 18 (or any version that supports ES‑modules).  
> * All files from the repo (`summary.js`, `tags.js`, `emb.js`, `models.js`, `storage.js`, `todozi.js`).  
> * Install the tiny runtime dependencies used in the stubbed files:

/ *
bash
npm install uuid luxon

/ *
> The stubbed embedding model (`EmbeddingModel`) in *emb.js* simply returns random vectors, so the similarity scores are only demonstrative –‑ they still let you see the whole pipeline working.

---

## `example4.js`

---

### What the script does (step‑by‑step)

| Step | Action | Reason |
|------|--------|--------|
| **A** | Import `SummaryManager`, `TagManager`, and the embedding service. | Gives us the three core APIs we’ll use. |
| **B** | Call `parseSummaryFormat` on a raw `<summary>` block. | Shows how the parser extracts `content`, `priority`, `context`, and `tags`. |
| **C** | Store the parsed object with `SummaryManager.createSummary`. | The manager adds an `id`, `created_at`, `updated_at` and returns the id. |
| **D** | For each tag extracted from the summary, either create the tag (if missing) or increment its usage. | Demonstrates the tag‑life‑cycle (`TagManager`). |
| **E** | Generate a random embedding for the summary text and for each tag, then put them in `TodoziEmbeddingService`’s internal cache (a `Map`). | Real‑world code would persist the cache, but for a demo an in‑memory cache suffices. |
| **F** | Perform a **pure semantic search** (`semanticSearch`) using only the vector similarity. | Returns the summary and tag entries that are “close” to the query string. |
| **G** | Perform a **hybrid search** (`hybridSearch`) that mixes semantic similarity with literal keyword boosts. | Shows how you can prioritize known keywords while still using embeddings. |
| **H** | Run `clusterContent()` to group similar cached items together. | Helpful for visualising how the embeddings cluster (e.g. tags that often appear together). |
| **I** | Print a friendly wrap‑up message. | Leaves the console tidy. |

---

### Running the example

# 1️⃣  Make sure you are in the repository root
npm install uuid luxon   # (once – if you haven’t already)

# 2️⃣  Run the demo
node example4.js

You should see output similar to:
*/