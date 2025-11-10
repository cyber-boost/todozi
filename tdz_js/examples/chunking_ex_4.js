/**
 * Example 4 – Automated Code‑Chunk Management + Semantic Search
 *
 * Prerequisites (install once):
 *   npm i uuid luxon
 *
 * Place this file next to the Todozi source root, then run:
 *   node example4.js
 *
 * The script will:
 *   • Create a new code‑chunk (module level) with a tiny SQLite helper.
 *   • Save the chunk in a CodeGenerationGraph.
 *   • Generate an embedding for the chunk and store it in TodoziEmbeddingService.
 *   • Tag the chunk with domain‑specific tags.
 *   • Perform a semantic search for “store scraped data in a DB”.
 *   • Print a readable summary.
 */

/// ----------------------------------------------------------------------
/// 1️⃣  Imports – everything we need from the existing Todozi code base
/// ----------------------------------------------------------------------
import { v4 as uuidv4 } from 'uuid';
import { DateTime } from 'luxon';

// Chunking
import {
  ChunkingLevel,
  CodeChunk,
  CodeGenerationGraph,
  parseChunkingFormat,
} from '../todozi/chunking.js';

// Embedding
import {
  TodoziEmbeddingConfig,
  TodoziEmbeddingService,
  SimilarityResult,
  TodoziEmbeddingCache,
  TodoziContentType,
} from '../todozi/emb.js';

// Tagging
import { TagManager, Tag } from '../todozi/tags.js';

// Storage – only needed if you want to persist the graph (optional)
import { Storage } from '../todozi/storage.js';

/// ----------------------------------------------------------------------
/// 2️⃣  Helper – pretty‑print a chunk (status, tags, code snippet)
/// ----------------------------------------------------------------------
function formatChunk(chunk, tags = []) {
  const status = chunk.status.toString();
  const level = chunk.level.toString();
  const codeSnippet = chunk.code
    .split('\n')
    .slice(0, 4)                 // first 4 lines
    .join('\n')
    .trim()
    .replace(/\s+$/g, '');

  return `
🧩 Chunk ID   : ${chunk.chunkId}
🔹 Level      : ${level}
🔹 Status     : ${status}
🔹 Tags       : ${tags.map(t => t.name).join(', ') || 'none'}
🔹 Code (first 4 lines):
${codeSnippet}
`;
}

/// ----------------------------------------------------------------------
/// 3️⃣  Main async workflow
/// ----------------------------------------------------------------------
(async () => {
  console.log('🚀 Starting Example 4 – Code‑Chunk + Embedding demo\n');

  // --------------------------------------------------------------
  // (a) Initialise the three core services
  // --------------------------------------------------------------
  const storage = await Storage.new();                         // persistence (optional)
  const embedConfig = TodoziEmbeddingConfig.default();          // defaults → MiniLM‑L6‑v2
  const embedService = await TodoziEmbeddingService.new(embedConfig);
  const tagMgr = TagManager.new();                             // in‑memory tag index

  // --------------------------------------------------------------
  // (b) Build a fresh CodeGenerationGraph (max 10 000 lines for demo)
  // --------------------------------------------------------------
  const graph = new CodeGenerationGraph(10_000);

  // --------------------------------------------------------------
  // (c) Define a *raw* chunk text in the Todozi <chunk> format.
  //     This is the same format that `processChunkingMessage()` expects.
  // --------------------------------------------------------------
  const rawChunk = `
<chunk>
  db_helper; module; SQLite helper for the scraper; ; 
  import sqlite3; 
  class DatabaseConnection:
      def __init__(self, db_path):
          self.conn = sqlite3.connect(db_path)
      def insert_record(self, table, data):
          # placeholder – real implementation later
          pass
</chunk>
`.trim();

  // Parse it into a CodeChunk instance
  const parsedChunk = parseChunkingFormat(rawChunk);
  // Give the chunk a deterministic ID (you could also let the constructor generate one)
  parsedChunk.chunkId = `chunk_${uuidv4().slice(0, 8)}`;

  // --------------------------------------------------------------
  // (d) Add the chunk to the graph – no deps for this demo.
  // --------------------------------------------------------------
  graph.addChunk(parsedChunk.chunkId, parsedChunk.level, []);
  // Retrieve the mutable chunk and fill in its code (the parser set it already,
  // but we show the explicit API anyway)
  const mutChunk = graph.getChunkMut(parsedChunk.chunkId);
  mutChunk.setCode(parsedChunk.code); // same code as parsed, just for demo

  // --------------------------------------------------------------
  // (e) Persist the graph (optional).  In a real CLI you'd call
  //     `storage.saveProjectTaskContainer()` or a dedicated graph‑save
  //     routine.  Here we skip the file I/O – the graph lives in RAM.
  // --------------------------------------------------------------

  // --------------------------------------------------------------
  // (f) **Generate an embedding** for the chunk’s source code.
  // --------------------------------------------------------------
  const embedding = await embedService.generateEmbedding(mutChunk.code);
  // Store the embedding in the service's cache (keyed by chunk ID)
  const cacheKey = `chunk_${mutChunk.chunkId}`;
  embedService.cache.set(
    cacheKey,
    new TodoziEmbeddingCache(
      embedding,               // vector
      TodoziContentType.Chunk, // type
      mutChunk.chunkId,        // content_id
      mutChunk.code,           // text_content
      [],                      // tags (empty for now)
      new Date(),
      embedConfig.cache_ttl_seconds
    )
  );

  // --------------------------------------------------------------
  // (g) **Create tags** for the chunk and link them.
  // --------------------------------------------------------------
  const tagsToCreate = [
    { name: 'web-scraping', color: '#ff8800' },
    { name: 'sqlite',       color: '#0088ff' },
  ];
  const createdTagIds = [];
  for (const t of tagsToCreate) {
    const tag = new Tag({ name: t.name, color: t.color });
    const tagId = await tagMgr.createTag(tag);
    createdTagIds.push(tagId);
  }

  // Associate the tags with the chunk – we simply store the tag IDs
  // inside the chunk (Todozi doesn't have a built‑in field for this,
  // but you can extend `CodeChunk` with a `tags` array if you like).
  mutChunk.tags = createdTagIds; // add custom property for demo

  console.log('✅ Chunk created & embedded:');
  console.log(formatChunk(mutChunk, tagsToCreate));

  // --------------------------------------------------------------
  // (h) **Semantic search** – ask the embedding service for chunks
  //     that are similar to the *query* “store scraped data in a DB”.
  // --------------------------------------------------------------
  const query = 'store scraped data in a SQLite database';
  const similar = await embedService.semanticSearch(query, [TodoziContentType.Chunk], 5);

  // --------------------------------------------------------------
  // (i) Pretty‑print the search results
  // --------------------------------------------------------------
  console.log('\n🔎 Semantic search results for query:', `"${query}"`);
  if (similar.length === 0) {
    console.log('No similar chunks found (threshold too high or cache empty).');
  } else {
    for (const [i, result] of similar.entries()) {
      // Resolve the original chunk from graph using the content_id
      const foundChunk = graph.getChunk(result.content_id);
      // Fetch tag names for the chunk (if any)
      const chunkTags = (foundChunk.tags || []).map(id => tagMgr.getTag(id));
      console.log(`\n#${i + 1} – similarity ${(result.similarity_score * 100).toFixed(1)}%`);
      console.log(formatChunk(foundChunk, chunkTags));
    }
  }

  // --------------------------------------------------------------
  // (j) **Mark the chunk as validated** – we simulate a review pass.
  // --------------------------------------------------------------
  mutChunk.markValidated();
  console.log('\n✅ Chunk status updated to VALIDATED.');
  console.log(formatChunk(mutChunk, tagsToCreate));

  // --------------------------------------------------------------
  // (k) Clean‑up (optional) – e.g., remove expired embeddings.
  // --------------------------------------------------------------
  const cleaned = await embedService.cleanupExpired();
  console.log(`\n🧹 Cleaned ${cleaned} expired embedding entries from the cache.`);

  console.log('\n🎉 Example 4 finished.\n');
})().catch(err => {
  console.error('💥 Example 4 failed:', err);
});

/*
## Example 4 – Automated **Code‑Chunk** Management + **Semantic Search**  

In this example we combine three big‑picture pieces of the Todozi code base:

| Piece | What we use | Why it matters |
|------|--------------|----------------|
| **`chunking.js`** | `ChunkingLevel`, `CodeChunk`, `CodeGenerationGraph`, `parseChunkingFormat` | Create, edit, validate, and wire‑up code chunks that belong to a project. |
| **`emb.js`** | `TodoziEmbeddingService`, `TodoziEmbeddingTool` | Generate an embedding for every chunk and later find “similar” chunks (e.g., to re‑use a scraper routine). |
| **`tags.js`** | `TagManager` | Tag chunks with domain‑specific keywords ( `web‑scraping`, `database` …) and use the tag‑index for fast lookup. |
| **`storage.js`** (optional) | `Storage` | Persist the generated chunks so they survive a process restart. |

The script below shows a **complete workflow**:

1. **Boot the environment** – load storage, the embedding service and a tag manager.  
2. **Create a new code‑chunk** (a “module” chunk that implements a simple SQLite DB wrapper).  
3. **Persist the chunk** in a `CodeGenerationGraph`.  
4. **Generate an embedding** for the chunk’s source code and store it inside the embedding service.  
5. **Tag the chunk** (`web‑scraping`, `sqlite`).  
6. **Run a semantic search** – ask the embedding service for chunks that look like a *“fetch‑and‑store data”* query.  
7. **Print a friendly report** that includes the chunk’s status, its tags, and any similar chunks that were found.

> **What you get** – a runnable Node.js file (`example4.js`) that can be dropped into the Todozi repo and executed with `node example4.js`.  
> All the heavy‑lifting (graph handling, embedding, tags) is already coded; we only orchestrate them.

---

### `example4.js`

---

### How the script works – step‑by‑step explanation  

| Step | Code fragment | What happens |
|------|---------------|--------------|
| **1** | `require('./chunking')`, `require('./emb')`, `require('./tags')` | Pulls in the existing Todozi modules. |
| **2** | `function formatChunk …` | Small helper to print a chunk with its tags – makes the console output readable. |
| **3** | `const storage = await Storage.new()` | Instantiates the persistence layer (you can ignore it if you only need an in‑memory demo). |
| **4** | `const embedService = await TodoziEmbeddingService.new(embedConfig)` | Boots the embedding service; it lazy‑loads the MiniLM model (first run will download ~90 MB). |
| **5** | `const rawChunk = \`<chunk> … </chunk>\`` | Defines a chunk in the same `<chunk>` format that the CLI already parses. |
| **6** | `parseChunkingFormat(rawChunk)` | Turns the textual representation into a `CodeChunk` object. |
| **7** | `graph.addChunk …` | Inserts the chunk into a `CodeGenerationGraph` – the graph tracks dependencies, status, etc. |
| **8** | `embedService.generateEmbedding(mutChunk.code)` | Sends the source code through the transformer model and gets a 384‑dimensional vector. |
| **9** | `embedService.cache.set …` | Stores the embedding in the service’s LRU cache; the key is `"chunk_<id>"`. |
| **10**| `TagManager.new()` & `createTag` | Creates two tags (`web‑scraping`, `sqlite`) and registers them in the tag manager. |
| **11**| `mutChunk.tags = createdTagIds` | We attach the tag IDs to the chunk (a custom property – you could subclass `CodeChunk` to make it official). |
| **12**| `embedService.semanticSearch(query, …, 5)` | Performs a cosine‑similarity search against the cached embeddings, limited to 5 results. |
| **13**| Loop over `similar` | Looks up each matching chunk in the graph, resolves its tags, and prints a report. |
| **14**| `mutChunk.markValidated()` | Demonstrates how a chunk moves from *Pending → Completed → Validated*. |
| **15**| `embedService.cleanupExpired()` | Shows housekeeping – deletes embeddings older than the TTL (default 24 h). |

---

### Running the example

/ *
bash
# 1️⃣  Install the tiny runtime deps (if you haven’t already)
npm i uuid luxon

# 2️⃣  Execute the demo
node example4.js

/ *
The first run will download the default embedding model (`sentence‑transformers/all‑MiniLM‑L6‑v2`).  
After the download finishes you’ll see output similar to:
*/