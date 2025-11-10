// Replace the in‑memory manager with a Hlx‑backed one:
import { Storage } from '../todozi/storage.js';

(async () => {
  const storage = await Storage.new();          // loads or creates ~/.todozi/tdz.hlx
  const errMgr = new ErrorManager();

  // Persist errors in the Hlx config section "errors"
  // (you would need to add tiny wrappers around errMgr.save / load)
})();

/*

 * Example 4 – Using TodoziError + ErrorManager
 *
 * Demonstrates:
 *   • Creating typed TodoziError instances (validation, storage, etc.)
 *   • Adding them to an in‑memory ErrorManager
 *   • Querying unresolved errors
 *   • Resolving an error
 *   • Converting a raw <error>…</error> string into a proper record
 */
'use strict';

// --------------------------------------------------
// 1️⃣  Import the Todozi error utilities
// --------------------------------------------------
import { TodoziError, ErrorManager, parseErrorFormat } from '../todozi/error.js';

// --------------------------------------------------
// 2️⃣  Helper – pretty‑print a list of errors
// --------------------------------------------------
function printErrors(title, errors) {
  console.log('\n===', title, '===\n');

  if (errors.length === 0) {
    console.log('✅ No errors to show.\n');
    return;
  }

  errors.forEach((e, idx) => {
    console.log(`${idx + 1}. [${e.id}] ${e.title}`);
    console.log(`   Description : ${e.description}`);
    console.log(`   Severity    : ${e.severity}`);
    console.log(`   Category    : ${e.category}`);
    console.log(`   Source      : ${e.source}`);
    console.log(`   Resolved?   : ${e.resolved ? 'YES' : 'NO'}`);
    console.log(`   Created At  : ${e.createdAt.toISOString()}\n`);
  });
}

// --------------------------------------------------
// 3️⃣  Main async routine – all Todozi APIs are async
// --------------------------------------------------
(async () => {
  // ------------------------------------------------
  // 3.1️⃣  Instantiate an in‑memory ErrorManager
  // ------------------------------------------------
  const errMgr = new ErrorManager();

  // ------------------------------------------------
  // 3.2️⃣  Create a few error objects using the helpers
  // ------------------------------------------------
  const validationErr = TodoziError.validation(
    'Task description must be at least 10 characters'
  );

  const storageErr = TodoziError.storage(
    'Failed to write task data to disk – permission denied'
  );

  const customErr = TodoziError.new(
    'Unexpected state while syncing agents',
    'AgentSyncError',
    { agentId: 'coder', step: 3 }
  );

  // ------------------------------------------------
  // 3.3️⃣  Store them – createError() returns a UUID
  // ------------------------------------------------
  const ids = await Promise.all([
    errMgr.createError({
      title: validationErr.message,
      description: validationErr.message,
      severity: 'low',
      category: 'validation',
      source: 'cli_add',
    }),
    errMgr.createError({
      title: storageErr.message,
      description: storageErr.message,
      severity: 'high',
      category: 'storage',
      source: 'storage_write',
    }),
    errMgr.createError({
      title: customErr.message,
      description: customErr.message,
      severity: 'critical',
      category: 'agent',
      source: 'agent_sync',
    }),
  ]);

  console.log('\n🟢 Errors have been created with IDs:', ids.join(', '));

  // ------------------------------------------------
  // 3.4️⃣  List **unresolved** errors
  // ------------------------------------------------
  const unresolved = errMgr.getUnresolvedErrors();
  printErrors('Unresolved Errors', unresolved);

  // ------------------------------------------------
  // 3.5️⃣  Resolve the *storage* error (2nd entry)
  // ------------------------------------------------
  const storageErrorId = ids[1];
  await errMgr.resolveError(storageErrorId, {
    note: 'Fixed folder permissions; re‑run the command',
  });

  console.log(`\n✅ Resolved error ID ${storageErrorId}`);

  // ------------------------------------------------
  // 3.6️⃣  Show **all** errors again – resolved flag should be true now
  // ------------------------------------------------
  const allErrors = Array.from(errMgr.errors.values());
  printErrors('All Errors (after resolution)', allErrors);

  // ------------------------------------------------
  // 3.7️⃣  Parse a raw <error> … </error> string
  // ------------------------------------------------
  const rawErrorString = `
    <error>
      Database connection lost; Unable to reach DB server; high; database; myApp
      ; context=retry‑after‑5‑seconds
      ; network,critical
    </error>
  `;

  try {
    const parsed = parseErrorFormat(rawErrorString);
    console.log('\n🧩 Parsed error from raw XML‑like string:');
    console.log(JSON.stringify(parsed, null, 2));

    // Store the parsed error as a regular TodoziError record
    const parsedId = await errMgr.createError({
      title: parsed.title,
      description: parsed.description,
      severity: parsed.severity,
      category: parsed.category,
      source: parsed.source,
      context: parsed.context,
      tags: parsed.tags,
    });

    console.log(`\n🟣 Parsed error stored with UUID ${parsedId}`);
  } catch (e) {
    // If the XML is malformed TodoziError.validation will be thrown
    console.error('❌ Failed to parse raw error string:', e.message);
  }

  // ------------------------------------------------
  // 3.8️⃣  Final dump – show every error we now have
  // ------------------------------------------------
  const finalList = Array.from(errMgr.errors.values());
  printErrors('Final Error Collection', finalList);
})();

/ *
## Example 4 – Working with Errors in Todozi  
**Goal** – Show a realistic flow that:

1. **Creates** custom error objects (using the `TodoziError` helpers).  
2. **Stores** them with the `ErrorManager`.  
3. **Lists** the unresolved errors.  
4. **Resolves** a specific error.  
5. **Parses** a raw `<error>…</error>` string into a full error record.  

The example is completely self‑contained; you can drop the file next to the Todozi source tree and run it with Node ≥ 18.

/ *
bash
# From the repo root
node examples/example4.js

/ *
---

### 1️⃣ Directory layout

project‑root/
│
├─ error.js               ← core Todozi error class & manager (provided)
├─ storage.js             ← optional – not required for this demo
├─ examples/
│   └─ example4.js        ← **this file**
└─ package.json

> **NOTE** – The demo only imports `error.js`.  
> If you later want to persist errors to disk you can replace the in‑memory `ErrorManager` with a storage‑backed implementation (see `storage.js`).

---

### 2️⃣ The code ( `examples/example4.js` )

---

### 3️⃣ What the script does (step‑by‑step)

| Step | Action | Result |
|------|--------|--------|
| **2️⃣** | Imports `TodoziError`, `ErrorManager`, `parseErrorFormat` from `error.js`. | You now have the core API. |
| **3.1** | Instantiates `ErrorManager`. | In‑memory store (`Map`). |
| **3.2** | Calls the static helpers (`validation`, `storage`, `new`) to build three distinct errors. | Each error is a `TodoziError` with a clear `type` and optional `details`. |
| **3.3** | Persists the three objects via `createError`. | Generates a UUID for each entry and adds timestamps (`createdAt`, `updatedAt`). |
| **3.4** | Retrieves **unresolved** errors (`resolved === false`). | Demonstrates filtering logic. |
| **3.5** | Resolves the second error (`storageErr`) with a note. | `resolved` flips to `true`; `resolvedAt` set. |
| **3.6** | Prints **all** errors – you can see the resolved flag. | Shows the state transition. |
| **3.7** | Parses a raw `<error>…</error>` string using `parseErrorFormat`. | Validates the format (throws `TodoziError.validation` on bad data). |
| **3.7‑b** | Stores the parsed error into `ErrorManager`. | Demonstrates bridging external error logs into Todozi. |
| **3.8** | Final dump of every error record. | Gives you a full picture of the error collection. |

---

### 4️⃣ Why this matters

* **Typed errors** (`TodoziError.validation`, `TodoziError.storage`, …) give you **semantic** information that can be filtered and acted upon later.  
* The **`ErrorManager`** abstracts persistence – replace the in‑memory `Map` with a DB or file‑based store without touching the rest of the code.  
* `parseErrorFormat` lets you ingest error logs from external tools (shell scripts, CI pipelines, etc.) and turn them into first‑class Todozi records.  
* The **resolution flow** (`resolveError`) records who resolved the issue and when, which is perfect for audit trails or a “bug‑bash” dashboard.

---

### 5️⃣ Extending the demo (optional)

If you want to **persist** errors between runs, replace the in‑memory manager with the storage layer:

Or, you could expose a **CLI command** (`todozi error list`, `todozi error resolve <id>`) that simply forwards to the methods shown above.

---

### 6️⃣ Run it and see the output