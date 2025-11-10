// example4.js – minimal version

import { MigrationCli, TaskMigrator } from '../todozi/migration.js';
import { initStorage, getStorageDir } from '../todozi/storage.js';
import path from 'path';
import fs from 'fs';
import { getStorageDir } from '../todozi/storage.js';
import { initStorage } from '../todozi/storage.js';
import { MigrationCli } from '../todozi/migration.js';
import { TaskMigrator } from '../todozi/migration.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

(async () => {
  // 1️⃣ Initialise storage (creates ~/.todozi structure)
  await initStorage();

  // 2️⃣ Optionally seed legacy collections here (see full example)

  // 3️⃣ Run a *dry‑run* migration (like the CLI)
  await MigrationCli.new()
    .withDryRun(true)
    .withVerbose(true)
    .withForce(true)
    .run();

  // 4️⃣ Run the *real* migration and get the report
  const migrator = TaskMigrator.new()
    .withDryRun(false)
    .withVerbose(true)
    .withForceOverwrite(true);
  const report = await migrator.migrate();

  console.log('🚀 Migration finished →', report);
  console.log('✅ Validation:', migrator.validateMigration());

  // 5️⃣ Clean up old collections if everything is OK
  if (migrator.validateMigration()) migrator.cleanupLegacy();
})();

/*
class JsonLogger {
  log(...args) {
    const entry = {
      ts: new Date().toISOString(),
      level: 'info',
      msg: args.join(' ')
    };
    console.log(JSON.stringify(entry));
  }
  error(...args) {
    const entry = {
      ts: new Date().toISOString(),
      level: 'error',
      msg: args.join(' ')
    };
    console.error(JSON.stringify(entry));
  }
}

// …later in the demo:
const logger = new JsonLogger();
const cli = MigrationCli.new()
  .withDryRun(false)
  .withVerbose(true)
  .withForce(true);

// Monkey‑patch the migrator to use our logger
cli.migrator.printSummary = function (report) {
  logger.log('Migration summary', JSON.stringify(report, null, 2));
};

await cli.run();

/ *
/ * ------------------------------------------------------------------
   example4-migrate.js
   Demonstrates how to drive the migration logic programmatically.
   ------------------------------------------------------------------ */

// ------------------------------------------------------------------
// 1️⃣  Helper: create a few dummy legacy tasks in the old collections
// ------------------------------------------------------------------
async function seedLegacyCollections(storageDir) {
  const collections = ['active', 'completed', 'archived'];
  const sampleTasks = {
    active: [
      {
        id: 'legacy_1',
        action: 'Write project‑migration documentation',
        time: '2h',
        priority: 'high',
        parent_project: '',
        status: 'todo',
        tags: ['migration', 'docs'],
        context_notes: 'Explain each step clearly.',
      },
      {
        id: 'legacy_2',
        action: 'Backup current task DB',
        time: '30m',
        priority: 'critical',
        parent_project: '',
        status: 'todo',
        tags: ['backup'],
      },
    ],
    completed: [
      {
        id: 'legacy_3',
        action: 'Export legacy JSON dump',
        time: '5m',
        priority: 'medium',
        parent_project: '',
        status: 'done',
        tags: ['export'],
      },
    ],
    archived: [
      {
        id: 'legacy_4',
        action: 'Old feature‑flag cleanup',
        time: '1h',
        priority: 'low',
        parent_project: '',
        status: 'cancelled',
        tags: ['cleanup'],
      },
    ],
  };

  // Write the collections as simple JSON files (the real storage uses
  // the TaskCollection class, but for the demo we can cheat a little).
  for (const col of collections) {
    const filePath = path.join(storageDir, 'tasks', `${col}.json`);
    const collection = {
      version: '1.2.0',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      tasks: {}
    };
    for (const t of sampleTasks[col] || []) {
      collection.tasks[t.id] = t; // mimics TaskCollection internal map
    }
    await fs.mkdir(path.dirname(filePath), { recursive: true });
    await fs.writeFile(filePath, JSON.stringify(collection, null, 2));
  }

  console.log('✅ Seeded legacy collections with sample data');
}

// ------------------------------------------------------------------
// 2️⃣  Main driver – create the migrator, set options, run it
// ------------------------------------------------------------------
async function runMigrationDemo() {
  // 2️⃣a – Make sure storage folder exists (the repo normally creates it
  //      on first run, but we want a clean sandbox for the demo)
  const storageDir = await getStorageDir();

  // Clean any previous demo data (so repeated runs are idempotent)
  await fs.rm(storageDir, { recursive: true, force: true });
  await fs.mkdir(storageDir, { recursive: true });

  // Initialise the storage structure (folders, default config, default agents…)
  await initStorage();

  // Seed our fake legacy tasks
  await seedLegacyCollections(storageDir);

  // ----------------------------------------------------------------
  // 2️⃣b – Load the MigrationCli class (the public entry‑point)
  // ----------------------------------------------------------------

  // Create a fresh CLI instance, then configure it exactly like the
  // command line flags you would use:
  //   todozi migrate --dry-run --verbose --force
  const cli = MigrationCli.new()
    .withDryRun(true)      // <- simulate migration, never write to disk
    .withVerbose(true)     // <- get the nice progress logs
    .withForce(true);      // <- overwrite existing project containers

  // ----------------------------------------------------------------
  // 2️⃣c – Run the migration.
  // ----------------------------------------------------------------
  console.log('\n🚀 Starting migration (dry‑run, verbose, force)…\n');
  await cli.run(); // → returns nothing, but internal logs are printed

  // ----------------------------------------------------------------
  // 2️⃣d – After the (dry‑run) migration we can still inspect the
  //        report object that the migrator produced.
  // ----------------------------------------------------------------
  // The MigrationCli does not expose the report directly, but we can
  // create a *raw* migrator instance and call `migrate()` ourselves so
  // we capture the return value.
  const migrator = TaskMigrator.new()
    .withDryRun(true)   // keep it a dry‑run for safety
    .withVerbose(true)
    .withForceOverwrite(true);

  const report = await migrator.migrate();

  console.log('\n📊 MigrationReport (dry‑run) --------------------------');
  console.log(JSON.stringify(report, null, 2));
  console.log('------------------------------------------------------\n');

  // ----------------------------------------------------------------
  // 2️⃣e – If you are satisfied, run a *real* migration (remove dry‑run)
  // ----------------------------------------------------------------
  const realMigrator = TaskMigrator.new()
    .withDryRun(false)   // actually write the new project containers
    .withVerbose(true)
    .withForceOverwrite(true);

  const realReport = await realMigrator.migrate();

  console.log('\n✅ Real migration finished – here is the final report:');
  console.log(JSON.stringify(realReport, null, 2));

  // ----------------------------------------------------------------
  // 2️⃣f – Validate that everything migrated correctly.
  // ----------------------------------------------------------------
  const isValid = realMigrator.validateMigration();
  console.log(`\n🔍 Validation result: ${isValid ? '✅ OK' : '❌ FAILED'}`);

  // ----------------------------------------------------------------
  // 2️⃣g – Clean up legacy collections (only when validation passed)
  // ----------------------------------------------------------------
  if (isValid && realReport.errors.length === 0) {
    console.log('\n🧹 Removing empty legacy collections...');
    realMigrator.cleanupLegacy();
    console.log('✅ Cleanup complete');
  } else {
    console.log('⚠️  Skipping cleanup – something went wrong');
  }
}

// ------------------------------------------------------------------
// 3️⃣  Execute the demo (async entry point)
// ------------------------------------------------------------------
runMigrationDemo()
  .catch(err => {
    console.error('\n❌ Migration demo failed:', err);
    process.exit(1);
  });

/ *
## Example 4 – Programmatic Migration of Legacy Todozi Tasks to the New Project‑Based System  

Below is a **self‑contained, end‑to‑end** example that shows how you can use the classes in `migration.js` (and the storage helpers) from **your own Node script**.  

It demonstrates:  

1. **Setting up a tiny in‑memory legacy environment** (three collections: `active`, `completed`, `archived`).  
2. **Running a migration with the options you usually expose on the CLI** (`--dry‑run`, `--verbose`, `--force`).  
3. **Validating the migration** and **cleaning up** the old collections when everything looks good.  
4. **Inspecting the generated `MigrationReport`** to see exactly what happened.  

> **Why this example?**  
> The original `migration.js` is written to be used from a CLI (`MigrationCli`).  
> However, many automation scripts, CI pipelines, or UI actions may need to **drive the migrator program‑matically**.  
> This example shows a clean way to do that while still getting the same nice console output the CLI provides.

---

### 1️⃣  Install the required repo files (once)

/ *
bash
# Assuming you have the whole Todozi repository cloned:
cd /path/to/todozi

# Install only the minimal dependencies needed for the demo.
npm install uuid luxon

> The demo does **not** need heavy AI/embedding libraries – they’re optional in the migration flow.

---

### 2️⃣  Create the demo script  – `example4-migrate.js`

---

### 3️⃣  What the script does – step‑by‑step

| Step | What happens | Why it matters |
|------|--------------|----------------|
| **Prepare storage folder** | Calls `initStorage()` → creates `~/.todozi` hierarchy | Guarantees a clean environment (same as first `todozi init`). |
| **Seed legacy collections** | Writes JSON files `active.json`, `completed.json`, `archived.json` with a few hand‑crafted tasks. | Gives the migrator something to read; mimics a real user’s old data. |
| **Create `MigrationCli`** | `.withDryRun(true).withVerbose(true).withForce(true)` | Shows the same output you would get from `todozi migrate --dry-run --verbose --force`. |
| **Run the migration** | `await cli.run()` – the CLI prints progress (`🚀 Starting…`, per‑project stats, summary). | Demonstrates the *public* entry point. |
| **Capture the report** | Instantiates a raw `TaskMigrator`, calls `migrate()` directly, stores the returned `MigrationReport`. | Gives you programmatic access to the detailed stats (`tasksFound`, `projectStats`, `errors`). |
| **Run a real migration** | Same as before, but `dryRun = false`. This writes the new `ProjectTaskContainer` JSON files under `~/.todozi/project_tasks/`. | Shows how to actually move data to the new system. |
| **Validate** | Calls `validateMigration()`. The method counts tasks in the legacy collections vs. tasks in all project containers and checks that the numbers match (or that legacy is empty). | Guarantees nothing got lost. |
| **Cleanup** | If validation succeeded, calls `cleanupLegacy()` which deletes empty legacy collection files. | Leaves the old storage tidy, just like the real CLI does after a successful migration. |

---

### 4️⃣  Run the example

# From the repository root:
node example4-migrate.js

You should see console output similar to:

✅ Seeded legacy collections with sample data

🚀 Starting migration (dry‑run, verbose, force)…

🚀 Starting task migration to project‑based system...
🔍 DRY RUN MODE - No actual changes will be made
📂 Loaded 2 tasks from 'active' collection
📂 Loaded 1 tasks from 'completed' collection
📂 Loaded 1 tasks from 'archived' collection
📊 Found 3 unique projects
   • general: 4 tasks
...
📊 MIGRATION SUMMARY
...
✅ Migration completed successfully!

📊 MigrationReport (dry‑run) --------------------------
{
  "tasksFound": 4,
  "tasksMigrated": 0,
  "projectsMigrated": 3,
  "projectStats": [
    { "project_name":"general", "initial_tasks":0, "migrated_tasks":0, "final_tasks":0 },
    { "project_name":"default", "initial_tasks":0, "migrated_tasks":0, "final_tasks":0 }
  ],
  "errors": []
}
------------------------------------------------------
...
✅ Real migration finished – here is the final report:
{
  "tasksFound": 4,
  "tasksMigrated": 4,
  "projectsMigrated": 3,
  "projectStats": [
    { "project_name":"general", "initial_tasks":0, "migrated_tasks":4, "final_tasks":4 }
  ],
  "errors": []
}
🔍 Validation result: ✅ OK

🧹 Removing empty legacy collections...
✅ Cleaned up 3 empty legacy collections
✅ Cleanup complete