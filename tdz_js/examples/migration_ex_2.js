// example2_task_migration.js

import { MigrationCli } from '../todozi/migration.js';

async function runMigrationExample() {
  console.log("=== Todozi Task Migration Example ===\n");
  
  // Create migration CLI with verbose output
  const migrator = MigrationCli.new()
    .withVerbose(true)      // Show detailed progress
    .withDryRun(false)      // Actually perform migration
    .withForce(false);      // Don't overwrite existing projects

  try {
    // Run the migration
    console.log("Starting migration process...\n");
    await migrator.run();
    
    console.log("\n✅ Migration process completed!");
    
  } catch (error) {
    console.error(`❌ Migration failed: ${error.message}`);
    process.exit(1);
  }
}

// Execute the example
runMigrationExample().catch(console.error);

/*
Here's **Example 2** demonstrating how to use the `TaskMigrator` class to migrate tasks from the legacy system to the new project-based structure:

**Key Features Demonstrated:**
1. **Verbose Mode**: Shows detailed progress information
2. **Actual Migration**: Performs real migration (not dry run)
3. **Safe Operation**: Prevents overwriting existing projects
4. **Error Handling**: Catches and reports migration failures
5. **Progress Reporting**: Displays migration summary

**What This Example Does:**
- Loads tasks from legacy collections (`active`, `completed`, `archived`)
- Groups tasks by project (uses "general" for unassigned tasks)
- Creates new project containers with embedded vectors
- Saves migrated projects to `~/.todozi/project_tasks/`
- Generates embeddings for semantic search capabilities
- Provides detailed progress reporting
- Validates migration integrity
- Cleans up empty legacy collections

**Sample Output:**

/ *
=== Todozi Task Migration Example ===

Starting migration process...

🚀 Starting task migration to project-based system...
📂 Loaded 15 tasks from 'active' collection
📂 Loaded 8 tasks from 'completed' collection
⚠️  Could not load 'archived' collection (may not exist)
📊 Found 3 unique projects
   • general: 12 tasks
   • web-app: 5 tasks
   • mobile-app: 6 tasks
📁 Creating new project container for 'general'
   ✅ Migrated task: task_1234 (status: todo)
   ✅ Migrated task: task_5678 (status: in_progress)
💾 Saved project container: general
📁 Creating new project container for 'web-app'
   ✅ Migrated task: task_abcd (status: todo)
   ✅ Migrated task: task_efgh (status: done)
💾 Saved project container: web-app
...

============================================================
📊 MIGRATION SUMMARY
============================================================
Total legacy tasks found: 23
Tasks migrated: 23
Projects processed: 3

📋 Project Details:
  • general: 0 → 12 tasks (12 migrated)
  • web-app: 0 → 5 tasks (5 migrated)
  • mobile-app: 0 → 6 tasks (6 migrated)

✅ Migration completed successfully!
============================================================

🔍 Validating migration integrity...
Legacy system tasks: 0
Project system tasks: 23
✅ Migration validation passed

🧹 Cleaning up legacy collections...
   🗑️  Removed empty collection 'active'
   🗑️  Removed empty collection 'completed'
✅ Cleaned up 2 empty legacy collections
*/