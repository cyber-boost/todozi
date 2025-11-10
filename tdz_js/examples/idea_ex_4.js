// ---------------------------------------------------------------------------
//  Imagine the user typed this in a note:
//
//  <idea>Make the onboarding wizard learn from the user; share; 3; tags: ux, onboarding; context: first‑time users</idea>
//
//  The format is:  <idea> <text> ; <share> ; <importance> ; <tags> ; <context> </idea>
// ---------------------------------------------------------------------------

import { IdeaManager, IdeaUpdate, IdeaStatistics, parseIdeaFormat, } from '../todozi/idea.js';
import { ShareLevel, IdeaImportance, TodoziError, } from '../todozi/models.js';

const rawIdea = `
  <idea>
    Make the onboarding wizard learn from the user ; share ; 3 ; ux, onboarding ; first‑time users
  </idea>
`;

let parsedIdea;
try {
  parsedIdea = parseIdeaFormat(rawIdea);
  console.log('✅  Parsed raw idea successfully!\n');
} catch (e) {
  if (e instanceof TodoziError) {
    console.error('❌  Validation error while parsing the <idea> block:', e.details.message);
  } else {
    console.error('❌  Unexpected error:', e);
  }
  process.exit(1);
}

/*
const manager = new IdeaManager();   // in‑memory, no persistence layer needed

/ *
function prettyPrintIdea(idea) {
  console.log('────────────────────────────────────────────');
  console.log(`💡  Idea ID   : ${idea.id}`);
  console.log(`📝  Text      : ${idea.idea}`);
  console.log(`🔐  Share     : ${idea.share}`);
  console.log(`⭐  Importance: ${idea.importance}`);
  console.log(`🏷️  Tags      : ${idea.tags.join(', ') || '(none)'}`);
  console.log(`🗒️  Context   : ${idea.context || '(none)'}`);
  console.log(`⏰  Created   : ${idea.createdAt.toISOString()}`);
  console.log('────────────────────────────────────────────\n');
}

/ *
// ---------------------------------------------------------------------------
//  Example 4 – Idea lifecycle demo
// ---------------------------------------------------------------------------

/ *
## Example 4 – Full‑stack workflow for **Ideas**  

This example shows how to combine the pieces that live in the repository:

| Step | What we do | Which file(s) are used |
|------|------------|------------------------|
| 1️⃣  | Initialise an `IdeaManager` (in‑memory store). | `idea.js` |
| 2️⃣  | Parse a raw user‑written `<idea>` block with `parseIdeaFormat`. | `idea.js` (uses `models.js`) |
| 3️⃣  | Persist the parsed idea in the manager (`createIdea`). | `idea.js` |
| 4️⃣  | Update an idea later with an `IdeaUpdate` builder. | `idea.js` |
| 5️⃣  | Query the manager (search, filter by importance / tags). | `idea.js` |
| 6️⃣  | Produce statistics (`IdeaStatistics` & tag stats). | `idea.js` |
| 7️⃣  | (Bonus) Export the whole collection to JSON for later backup. | user‑code only |

> **Why this is useful** –  
> * A typical CLI command (`todozi idea create …`) first turns a fancy tag‑based string into a proper `Idea` object.  
> * The manager keeps the data in a `Map` and mirrors all tags in a second `Map` (`idea_tags`) so that tag‑centric queries are fast.  
> * The statistics helpers give you quick insight (how many public ideas, break‑through ideas, unique tags …) without having to write extra loops.

---

### 1️⃣  Boiler‑plate – import everything we need

---

### 2️⃣  Helper: pretty‑print an idea

---

### 3️⃣  Initialise the manager

---

### 4️⃣  Parse a raw `<idea>` block (the same format you would type in the CLI)

> **Result** – `parsedIdea` is a fully‑fledged `Idea` instance (with generated `id`, timestamps, …) but **not yet stored** in the manager.

---

### 5️⃣  Store the idea  

(async () => {
  const ideaId = await manager.createIdea(parsedIdea);
  console.log(`📦  Idea stored with internal ID: ${ideaId}\n`);

  // Retrieve it back just to prove it works
  const storedIdea = manager.getIdea(ideaId);
  prettyPrintIdea(storedIdea);
})();

---

### 6️⃣  Update the idea later (using the fluent `IdeaUpdate` builder)

(async () => {
  // Pretend the user has decided to make the idea private and add a new tag
  const update = IdeaUpdate.new()
    .withShare(ShareLevel.PRIVATE)               // ← change visibility
    .withTags(['ux', 'onboarding', 'ai‑assistant']) // ← add a tag
    .withContext('aimed at new users who have never used the product');

  try {
    await manager.updateIdea(storedIdea.id, update);
    console.log('🔧  Idea updated successfully!\n');
  } catch (e) {
    console.error('❌  Failed to update idea:', e);
  }

  // Show the fresh version
  const refreshed = manager.getIdea(storedIdea.id);
  prettyPrintIdea(refreshed);
})();

---

### 7️⃣  Create a couple more ideas (to make the statistics more interesting)

(async () => {
  const ideasToAdd = [
    // Break‑through, public
    '<idea>AI‑driven automatic UI refactor; share; 4; ai, refactor; </idea>',
    // Private, low importance
    '<idea>Write a blog post about our 2024 roadmap; private; 1; marketing; </idea>',
    // Team‑only, medium importance, extra tags
    '<idea>Implement dark‑mode toggle; team; 2; ui, theme; </idea>',
  ];

  for (const raw of ideasToAdd) {
    const i = parseIdeaFormat(raw);
    await manager.createIdea(i);
  }

  console.log('🧩  Added three more ideas.\n');
})();

---

### 8️⃣  Search & filter  

// ---------------------------------------------------------------------------
//  a) Full‑text search (matches either the idea text OR any tag)
// ---------------------------------------------------------------------------
console.log('🔎  Search for “AI” (should hit two ideas):');
const aiMatches = manager.searchIdeas('AI');
aiMatches.forEach(prettyPrintIdea);

// ---------------------------------------------------------------------------
//  b) Filter by importance (break‑through only)
// ---------------------------------------------------------------------------
console.log('📈  All “breakthrough” ideas (importance = 4):');
const breakthrough = manager.getBreakthroughIdeas();
breakthrough.forEach(prettyPrintIdea);

// ---------------------------------------------------------------------------
//  c) Filter by share level (public only)
// ---------------------------------------------------------------------------
console.log('🌍  Public ideas:');
manager.getPublicIdeas().forEach(prettyPrintIdea);

---

### 9️⃣  Generate **IdeaStatistics** (overall numbers)

(async () => {
  const stats = manager.getIdeaStatistics(); // → returns an IdeaStatistics object
  console.log('📊  Idea statistics');
  console.table({
    'Total ideas'          : stats.totalIdeas,
    'Public ideas'         : stats.publicIdeas,
    'Team ideas'           : stats.teamIdeas,
    'Private ideas'        : stats.privateIdeas,
    'Break‑through ideas' : stats.breakthroughIdeas,
    'Unique tags'          : stats.uniqueTags,
    'Public %'             : `${stats.publicPercentage().toFixed(1)}%`,
    'Team %'               : `${stats.teamPercentage().toFixed(1)}%`,
    'Private %'            : `${stats.privatePercentage().toFixed(1)}%`,
    'Break‑through %'      : `${stats.breakthroughPercentage().toFixed(1)}%`,
  });
})();

---

### 🔟  Bonus – **export the whole collection** (e.g. for a backup file)
*/