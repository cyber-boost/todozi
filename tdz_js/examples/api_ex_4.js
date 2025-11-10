// Example 4: API Key + Embedding Demo
// This is example code - uncomment and convert to ES modules if needed
/*
const task2 = Task.newFull('demo_user','Explain back‑pressure in Node.js streams','1h',Priority.High,'demo‑project',Status.Todo,null,['nodejs','streams'],[],null,null);
await storage.addTaskToProject(task2);
*/

// demo-api-embedding.js
// ------------------------------------------------------------
// 1️⃣  Load the core libraries we need
// ------------------------------------------------------------
import {
  // API‑key helpers
  createApiKey,
  createApiKeyWithUserId,
  listApiKeys,
  listActiveApiKeys,
  checkApiKeyAuth,
  deactivateApiKey,
  activateApiKey,
  removeApiKey,
} from '../todozi/api.js';

import {
  // Embedding service + tool
  TodoziEmbeddingConfig,
  TodoziEmbeddingService,
  TodoziEmbeddingTool,
} from '../todozi/emb.js';

import { Storage } from '../todozi/storage.js';   // Todozi storage layer
import { Task, Status, Priority } from '../todozi/models.js';

// ------------------------------------------------------------
// 2️⃣  Helper: pretty‑print objects (JSON with colours)
// ------------------------------------------------------------
const pretty = obj => console.log(JSON.stringify(obj, null, 2));

// ------------------------------------------------------------
// 3️⃣  MAIN – an async IIFE so we can use await everywhere
// ------------------------------------------------------------
(async () => {
  console.log('\n=== Todozi API‑Key + Embedding Demo ===\n');

  // -----------------------------------------------------------------
  // 3.1️⃣  ==== API‑KEY MANAGEMENT =====================================
  // -----------------------------------------------------------------
  console.log('🔐 Creating a fresh API‑key for a temporary user...');
  const apiKey = createApiKey();                 // → random key & userId
  pretty({
    userId: apiKey.userId,
    publicKey: apiKey.publicKey,
    privateKey: apiKey.privateKey,
    active: apiKey.active,
    createdAt: apiKey.createdAt,
  });

  console.log('\n🔎 Listing ALL keys (including the one we just created)…');
  const allKeys = listApiKeys();
  pretty(allKeys.map(k => ({ userId: k.userId, active: k.active })));

  console.log('\n🔎 Listing ONLY ACTIVE keys…');
  const activeKeys = listActiveApiKeys();
  pretty(activeKeys.map(k => ({ userId: k.userId, active: k.active })));

  // -----------------------------------------------------------------
  // 3.2️⃣  ==== AUTHENTICATE A REQUEST ================================
  // -----------------------------------------------------------------
  console.log('\n✅ Checking authentication with the PUBLIC key only (read‑only)…');
  const readOnly = checkApiKeyAuth(apiKey.publicKey);
  console.log(`→ User ID: ${readOnly[0]}, admin? ${readOnly[1]}`);

  console.log('\n✅ Checking authentication with PUBLIC + PRIVATE key (admin)…');
  const admin = checkApiKeyAuth(apiKey.publicKey, apiKey.privateKey);
  console.log(`→ User ID: ${admin[0]}, admin? ${admin[1]}`);

  // -----------------------------------------------------------------
  // 3.3️⃣  ==== DEACTIVATE / RE‑ACTIVATE KEY =========================
  // -----------------------------------------------------------------
  console.log('\n🔒 Deactivating the key…');
  deactivateApiKey(apiKey.userId);
  console.log('→ Key deactivated.');

  console.log('\n🔎 Trying to authenticate with a deactivated key (should fail)…');
  try {
    checkApiKeyAuth(apiKey.publicKey);
  } catch (e) {
    console.error('❗ Expected error →', e.message);
  }

  console.log('\n🔓 Reactivating the key…');
  activateApiKey(apiKey.userId);
  console.log('→ Key is active again.');

  // -----------------------------------------------------------------
  // 3.4️⃣  ==== EMBEDDING SERVICE – create a task ====================
  // -----------------------------------------------------------------
  console.log('\n🚀 Initialising the embedding service (default model)…');
  const embConfig = TodoziEmbeddingConfig.default(); // uses all‑MiniLM‑L6‑v2
  const embService = await TodoziEmbeddingService.new(embConfig);
  console.log('✅ Service ready.');

  // -----------------------------------------------------------------
  // 3.5️⃣  ==== CREATE a Todozi task (plain JS object) ==============
  // -----------------------------------------------------------------
  console.log('\n📝 Creating a new Todozi task object (no storage yet)…');
  const rawTask = Task.newFull(
    'demo_user',                     // userId
    'Write a short blog post about “Node.js streams”', // action
    '2h',                            // time estimate
    Priority.Medium,                 // priority
    'demo‑project',                  // parentProject
    Status.Todo,                     // status
    null,                            // assignee (null = collaborative)
    ['writing', 'nodejs'],           // tags
    [],                              // dependencies
    'Include code snippets showing pipe()', // contextNotes
    null                             // progress
  );
  pretty({
    id: rawTask.id,
    action: rawTask.action,
    project: rawTask.parentProject,
  });

  // -----------------------------------------------------------------
  // 3.6️⃣  ==== Generate & attach an embedding =======================
  // -----------------------------------------------------------------
  console.log('\n🔗 Generating an embedding vector for the task description…');
  // The service expects a “prepared” text; we can reuse the helper:
  const prepared = embService.prepareTaskContent(rawTask);
  const embedding = await embService.generateEmbedding(prepared);
  rawTask.embeddingVector = embedding;          // store on the task object
  console.log('✅ Embedding (first 5 values) →', embedding.slice(0, 5));

  // -----------------------------------------------------------------
  // 3.7️⃣  ==== Persist the task in Todozi storage ===================
  // -----------------------------------------------------------------
  console.log('\n💾 Saving the task to the file‑system storage…');
  const storage = await Storage.new();          // loads the user config
  await storage.addTaskToProject(rawTask);      // creates the project folder if needed
  console.log('✅ Task saved.');

  // -----------------------------------------------------------------
  // 3.8️⃣  ==== Search for “similar” tasks ==========================
  // -----------------------------------------------------------------
  console.log('\n🔎 Searching for tasks similar to: “Explain Node.js streams”');
  const similar = await embService.findSimilarTasks('Explain Node.js streams', 5);
  if (similar.length === 0) {
    console.log('⚠️  No similar tasks found (empty index).');
  } else {
    console.log(`✅ Found ${similar.length} similar task(s):`);
    similar.forEach((s, i) => {
      console.log(`   ${i + 1}. ${s.text_content.split('\n')[0].substring(0, 80)}…`);
      console.log(`      similarity: ${(s.similarity_score * 100).toFixed(1)}%`);
    });
  }

  // -----------------------------------------------------------------
  // 3.9️⃣  ==== OPTIONAL – clean‑up (remove the demo key) ==============
  // -----------------------------------------------------------------
  console.log('\n🗑️  Removing the demo API‑key from the collection…');
  const removed = removeApiKey(apiKey.userId);
  console.log('✅ Removed key for userId →', removed.userId);

  console.log('\n🎉 Demo finished – all steps executed without error!');
})().catch(err => {
  console.error('\n❌ Demo crashed:', err);
  process.exit(1);
});

/*
## Example 4 – Full‑stack “Todozi API‑Key + Embedding” Demo  
**Goal** – Show how a developer can:

1. **Create / manage API keys** with `api.js` (persisted on disk).  
2. **Load the key collection** and use it to **authenticate** a client request.  
3. **Generate an embedding** for a new task with the **embedding service** (`emb.js`).  
4. **Store the task** in the Todozi storage layer (`storage.js`).  
5. **Search** for similar tasks using the embedding index.

The example is a **stand‑alone Node script** (`demo‑api‑embedding.js`).  
All required modules are already part of the repository, so you only need to run the file after an `npm install` (or `yarn add` for the external deps you actually use – see the notes at the bottom).

---  

### 1️⃣ Project layout (relevant files)

/*
project-root/
│
├─ api.js               ← API‑key helpers (create, list, activate, …)
├─ emb.js               ← TodoziEmbeddingService / TodoziEmbeddingTool
├─ storage.js           ← File‑system storage (tasks, projects, …)
├─ models.js            ← Core data‑model definitions (Task, ApiKey, …)
└─ demo-api-embedding.js   ← **Our example script**

/ *
---  

### 2️⃣ The demo script

---  

### 3️⃣ What the script does (step‑by‑step)

| Step | Action | Why it matters |
|------|--------|----------------|
| **1** | Load the API‑key helpers (`api.js`) | Gives us `createApiKey()`, `listApiKeys()`, … |
| **2** | Load the embedding service (`emb.js`) | Provides `TodoziEmbeddingService` (semantic search) |
| **3** | Load the storage layer (`storage.js`) | Persists tasks/projects on disk (`~/.todozi/...`) |
| **4** | **Create** a fresh API key (no user‑supplied ID) | Demonstrates `ApiKey.new()` & the on‑disk JSON collection |
| **5** | List **all** and **active** keys | Shows the JSON collection format |
| **6** | Authenticate a **read‑only** request (public key only) | `checkApiKeyAuth()` returns `[userId, isAdmin]` |
| **7** | Authenticate an **admin** request (public + private) | Same function, but with the private key |
| **8** | **Deactivate** → **reactivate** the key | Shows how a key can be temporarily disabled |
| **9** | Initialise the **embedding service** (default model) | Loads the sentence‑transformers model (lazy‑load) |
| **10**| Build a **Task** object (`Task.newFull`) | Uses the model from `models.js` |
| **11**| **Generate** an embedding vector for the task text | `embService.generateEmbedding()` |
| **12**| Attach the vector to the task & **store** it (`storage.addTaskToProject`) | Persists the task + its embedding inside the project folder |
| **13**| **Search** for similar tasks (`findSimilarTasks`) | Demonstrates semantic similarity lookup |
| **14**| **Remove** the demo API key (cleanup) | Keeps the key collection tidy |

---  

### 4️⃣ Running the demo  

1. **Install required runtime packages** (once).  
   The repository already has `package.json` but the embedding‑related code needs a few extra libs:

      # Core deps (already in the repo)
   npm i uuid luxon

   # Embedding deps – optional, but required for the demo
   npm i @xenova/transformers   # (or any transformer library you prefer)
   
   > **Tip** – If you cannot install `@xenova/transformers` (it pulls big model files), replace the `EmbeddingModel` placeholder with a mock that returns a deterministic dummy vector.

2. **Run the script**:

      node demo-api-embedding.js
   
   You should see console output similar to:
*/