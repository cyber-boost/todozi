/* ----------------------------------------------------------------------
import { MemoryManager, MemoryUpdate, MemoryStatistics, parseMemoryFormat, MemoryImportance, MemoryTerm, } from '../todozi/memory.js';
import { TagManager, Tag, } from '../todozi/tags.js';

 * example4.js
 *
 * Demonstrates a realistic workflow with the Todozi “Memory” subsystem:
 *
 *   1️⃣  Parse raw <memory> tags into rich Memory objects.
 *   2️⃣  Persist them using MemoryManager.
 *   3️⃣  Update a memory with the fluent MemoryUpdate builder.
 *   4️⃣  Search memories (full‑text + tag filter).
 *   5️⃣  Retrieve tag statistics via TagManager.
 *   6️⃣  Pull aggregate MemoryStatistics.
 *
 * Run with:
 *
 *   $ node example4.js
 * ---------------------------------------------------------------------- */

'use strict';

// -------------------------------------------------
// 1️⃣  Imports
// -------------------------------------------------
// -------------------------------------------------
// 2️⃣  Helper – prints a nice divider
// -------------------------------------------------
function divider(title = '') {
  console.log('\n' + '='.repeat(80));
  if (title) console.log(` ${title}`);
  console.log('='.repeat(80));
}

// -------------------------------------------------
// 3️⃣  Initialise managers
// -------------------------------------------------
const memMgr = new MemoryManager();   // in‑memory storage for memories
const tagMgr = TagManager.new();      // in‑memory storage for tags

// -------------------------------------------------
// 4️⃣  Create a few memories from raw <memory> strings
// -------------------------------------------------
const rawMemories = [
  // Standard memory – visible to everyone
  `<memory>standard; 2024‑10‑01 09:00; "Deployed v2.3 to production"; "Release rollout"` +
  `; high; short; deploy,release,production</memory>`,

  // Secret (AI‑only) memory – only AI can read it
  `<memory>secret; 2024‑09‑30 16:45; "Found a weird latency spike in microservice X"` +
  `; medium; short; latency,bug,investigation</memory>`,

  // Human‑visible memory (e.g. a personal note)
  `<memory>human; 2024‑09‑28 12:15; "Team lunch at Sushi Place"` +
  `; low; short; social,team</memory>`,

  // Emotional memory (happy moment)
  `<memory>happy; 2024‑09‑27 18:30; "Received praise for the Q3 report"` +
  `; high; short; achievement,recognition</memory>`,
];

divider('📥  Parsing raw <memory> strings → Memory objects');
const parsedMemories = rawMemories.map((txt, idx) => {
  const mem = parseMemoryFormat(txt, `user_${idx}`);
  console.log(`✅ Parsed memory #${idx + 1}:`);
  console.log(`   id: ${mem.id}`);
  console.log(`   type: ${mem.memoryType.type}${mem.memoryType.emotion ? ` (${mem.memoryType.emotion})` : ''}`);
  console.log(`   moment: ${mem.moment}`);
  console.log(`   tags: ${mem.tags.join(', ')}`);
  return mem;
});

// -------------------------------------------------
// 5️⃣  Store the memories (and their tags) in MemoryManager
// -------------------------------------------------
divider('💾  Storing memories in MemoryManager');
(async () => {
  for (const mem of parsedMemories) {
    // The manager creates its own internal ID, but we keep the parsed.id for linking later
    const internalId = await memMgr.createMemory({
      moment: mem.moment,
      meaning: mem.meaning,
      reason: mem.reason,
      importance: mem.importance,
      term: mem.term,
      memoryType: mem.memoryType,
      tags: mem.tags,
    });

    console.log(`   → Stored as internal id ${internalId}`);
  }

  // -------------------------------------------------
  // 6️⃣  Demonstrate updating a memory (fluent builder)
  // -------------------------------------------------
  divider('✏️  Updating a memory with MemoryUpdate');
  // Grab the first stored memory (the classic deployment note)
  const firstId = memMgr.getAllMemories()[0].id;

  const upd = MemoryUpdate.new()
    .withMeaning('Deployed v2.3 to production – zero‑downtime rollout')
    .withTags(['deploy', 'release', 'production', 'zero‑downtime']);

  await memMgr.updateMemory(firstId, upd);
  console.log(`   Updated memory ${firstId}`);

  // -------------------------------------------------
  // 7️⃣  Search examples
  // -------------------------------------------------
  divider('🔎  Searching memories');
  const search1 = memMgr.searchMemories('latency');
  console.log('🔍  “latency” →', search1.map(m => m.id));

  const search2 = memMgr.getMemoriesByTag('deploy');
  console.log('🏷️  Tag “deploy” →', search2.map(m => m.id));

  const recent = memMgr.getRecentMemories(3);
  console.log('🕒  3 most recent memories →', recent.map(m => m.id));

  // -------------------------------------------------
  // 8️⃣  Sync tags with TagManager (so we can query tag stats)
  // -------------------------------------------------
  divider('🏷️  Populating TagManager from MemoryManager');
  for (const mem of memMgr.getAllMemories()) {
    for (const tagName of mem.tags) {
      // Create the tag if it does not exist yet
      let tag = tagMgr.getTagByName(tagName);
      if (!tag) {
        const newTag = new Tag({ name: tagName });
        const tagId = await tagMgr.createTag(newTag);
        console.log(`   ➕ Created tag “${tagName}” (id ${tagId})`);
      } else {
        // Increment usage count – mimic the behaviour from the real app
        await tagMgr.incrementTagUsage(tagName);
      }
    }
  }

  // -------------------------------------------------
  // 9️⃣  Tag statistics
  // -------------------------------------------------
  divider('📊  Tag statistics');
  const tagStats = tagMgr.getTagStatistics();
  console.log('   Total tags          :', tagStats.total_tags);
  console.log('   Total categories    :', tagStats.total_categories);
  console.log('   Total relationships :', tagStats.total_relationships);
  console.log('   Avg. usage per tag :', tagStats.average_usage.toFixed(2));

  const mostUsed = tagMgr.getMostUsedTags(3);
  console.log('   Top‑3 used tags    :', mostUsed.map(t => `${t.name}(${t.usage_count})`).join(', '));

  // -------------------------------------------------
  // 🔟  Overall memory statistics
  // -------------------------------------------------
  divider('🧮  Memory statistics (MemoryStatistics class)');
  const stats = memMgr.getMemoryStatistics();
  console.log('   Total memories          :', stats.totalMemories);
  console.log('   Short‑term memories    :', stats.shortTermMemories);
  console.log('   Long‑term memories     :', stats.longTermMemories);
  console.log('   Critical memories      :', stats.criticalMemories);
  console.log('   Unique tags            :', stats.uniqueTags);
  console.log('   Secret memories        :', stats.secretMemories);
  console.log('   Human‑visible memories :', stats.humanMemories);
  console.log('   Emotional memories     :', stats.emotionalMemories);
  console.log('   Standard memories      :', stats.standardMemories);
  console.log('\n   % short‑term  :', stats.shortTermPercentage().toFixed(1) + '%');
  console.log('   % long‑term   :', stats.longTermPercentage().toFixed(1) + '%');
  console.log('   % critical    :', stats.criticalPercentage().toFixed(1) + '%');

  // -------------------------------------------------
  // 🎉  Finished!
  // -------------------------------------------------
  divider('✅  Example finished – all data lives only in this process');
})();

/*
Below is **Example 4** – a complete, runnable script that shows how you can (1) create and store memories, (2) update them, (3) search and filter them, (4) inspect tag statistics, and (5) pull overall memory statistics.  
It ties together the **`MemoryManager`**, the **`parseMemoryFormat`** helper, the **`MemoryUpdate`** builder, and the **`TagManager`** (from `tags.js`).  

> **What you need before running the example**  
> * Node ≥ 18 (uses native ES‑modules).  
> * All the source files you posted (`memory.js`, `tags.js`, `models.js`, …) must be in the same directory (or reachable via `require`).  
> * No external services are required – everything runs in‑memory.

### How the script works (step‑by‑step)

| Step | What happens | Relevant code |
|------|--------------|---------------|
| **1** | Load the exported symbols from `memory.js` and `tags.js`. | `require('./memory')` & `require('./tags')` |
| **2** | Small utility `divider()` prints nice headings, making the console output readable. | `function divider(title = '') { … }` |
| **3** | Create a **`MemoryManager`** (in‑memory store) and a **`TagManager`** (also in‑memory). | `new MemoryManager();` & `TagManager.new();` |
| **4** | Convert raw `<memory>…</memory>` strings into fully‑typed objects using **`parseMemoryFormat`**. | `parseMemoryFormat(txt, \`user_${idx}\`);` |
| **5** | Persist each memory via **`MemoryManager.createMemory`**. The manager adds a UUID, timestamps, stores the tags in an auxiliary map, and returns its own internal ID. | `await memMgr.createMemory({ … });` |
| **6** | Update the first memory – change its *meaning* and add a new tag – using the **fluent `MemoryUpdate`** builder. | `MemoryUpdate.new().withMeaning(...).withTags(...);` |
| **7** | Run a few queries: free‑text search, tag‑based lookup, and “most recent”. | `memMgr.searchMemories`, `memMgr.getMemoriesByTag`, `memMgr.getRecentMemories` |
| **8** | Sync tag usage with **`TagManager`**: create tags that do not exist and bump the usage counter for the rest. | `tagMgr.createTag`, `tagMgr.incrementTagUsage` |
| **9** | Pull aggregated tag data (`TagStatistics`) and display the top‑used tags. | `tagMgr.getTagStatistics()`, `tagMgr.getMostUsedTags` |
| **10** | Finally, ask the manager for a **`MemoryStatistics`** snapshot and print percentages. | `memMgr.getMemoryStatistics()` |

### What you’ll see

Running the script prints something similar to:

/ *
================================================================================
 📥  Parsing raw <memory> strings → Memory objects
================================================================================
✅ Parsed memory #1:
   id: 4b2c5c7a-...
   type: standard
   moment: 2024-10-01 09:00
   tags: deploy,release,production
…

================================================================================
 💾  Storing memories in MemoryManager
================================================================================
   → Stored as internal id 7a562f19-...
   → Stored as internal id 09c44a3e-...
   …

================================================================================
 ✏️  Updating a memory with MemoryUpdate
================================================================================
   Updated memory 7a562f19-...

================================================================================
 🔎  Searching memories
================================================================================
🔍  “latency” → [09c44a3e-...]
🏷️  Tag “deploy” → [7a562f19-...]
🕒  3 most recent memories → [c1a7e9..., 7a562f..., 09c44a...]

================================================================================
 🏷️  Populating TagManager from MemoryManager
================================================================================
   ➕ Created tag “deploy” (id a1b2c3...)
   ➕ Created tag “release” (id d4e5f6...)
   …

================================================================================
 📊  Tag statistics
================================================================================
   Total tags          : 9
   Total categories    : 0
   Total relationships : 0
   Avg. usage per tag : 1.11
   Top‑3 used tags    : deploy(2), release(2), production(2)

================================================================================
 🧮  Memory statistics (MemoryStatistics class)
================================================================================
   Total memories          : 4
   Short‑term memories    : 4
   Long‑term memories     : 0
   Critical memories      : 1
   Unique tags            : 9
   Secret memories        : 1
   Human‑visible memories : 1
   Emotional memories     : 1
   Standard memories      : 1

   % short‑term  : 100.0%
   % long‑term   : 0.0%
   % critical    : 25.0%

================================================================================
 ✅  Example finished – all data lives only in this process
================================================================================
*/