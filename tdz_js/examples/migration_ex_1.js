// Example 1: Basic migration with dry run
import { TaskMigrator, MigrationCli } from '../todozi/migration.js';

async function basicMigrationExample() {
    
    // Create migrator with verbose output and dry run mode
    const migrator = TaskMigrator.new()
        .withVerbose(true)
        .withDryRun(true);  // No actual changes will be made
    
    console.log("🏃 Starting migration in dry run mode...");
    const report = await migrator.migrate();
    
    console.log("\n📋 Migration Report:");
    console.log(`Tasks found: ${report.tasks_found}`);
    console.log(`Tasks migrated: ${report.tasks_migrated}`);
    console.log(`Projects processed: ${report.projects_migrated}`);
}

// Example 2: Force migration with cleanup
async function forceMigrationExample() {
    
    // Create migrator that will overwrite existing projects
    const migrator = TaskMigrator.new()
        .withVerbose(true)
        .withForceOverwrite(true);  // Overwrite existing projects
    
    console.log("🔄 Starting forced migration...");
    const report = await migrator.migrate();
    
    // Validate the migration
    const isValid = migrator.validateMigration();
    console.log(`Migration validation: ${isValid ? '✅ PASSED' : '❌ FAILED'}`);
    
    // Clean up empty legacy collections
    if (isValid) {
        migrator.cleanupLegacy();
    }
}

// Example 3: Migration with specific scenarios
async function scenarioBasedMigration() {
    
    const migrator = TaskMigrator.new().withVerbose(true);
    
    // Simulate different project scenarios
    console.log("📊 Testing migration scenarios...");
    
    // Scenario 1: Tasks with different projects
    const testTasks = [
        {
            id: "task_1",
            action: "Fix login bug",
            parent_project: "webapp",
            status: "todo"
        },
        {
            id: "task_2", 
            action: "Design database schema",
            parent_project: "backend",
            status: "in_progress"
        },
        {
            id: "task_3",
            action: "General documentation",
            parent_project: "",  // Will become "general" project
            status: "done"
        }
    ];
    
    // Save test data to legacy collections (simplified for example)
    // In real usage, this would be saved through the storage system
    
    const report = await migrator.migrate();
    
    console.log("\n🎯 Migration Results by Project:");
    report.project_stats.forEach(stat => {
        console.log(`• ${stat.project_name}: ${stat.initial_tasks} → ${stat.final_tasks} tasks`);
    });
}

// Example 4: CLI integration example
async function cliMigrationExample() {
    
    // Simulate command line arguments
    const args = {
        dryRun: process.argv.includes('--dry-run'),
        verbose: process.argv.includes('--verbose'),
        force: process.argv.includes('--force')
    };
    
    const cli = MigrationCli.new()
        .withDryRun(args.dryRun)
        .withVerbose(args.verbose)
        .withForce(args.force);
    
    console.log("🚀 Running migration via CLI interface...");
    await cli.run();
    
    console.log("✅ Migration process completed!");
}

// Run examples
(async () => {
    try {
        console.log("=== Example 1: Basic Dry Run ===");
        await basicMigrationExample();
        
        console.log("\n=== Example 2: Forced Migration ===");
        await forceMigrationExample();
        
        console.log("\n=== Example 3: Scenario Testing ===");
        await scenarioBasedMigration();
        
    } catch (error) {
        console.error("❌ Migration failed:", error.message);
    }
})();

/*
Here's a practical example demonstrating how to use the Task Migrator to convert legacy task collections to a project-based system:

## Example: Migrating Legacy Tasks to Project-Based Structure

This example shows how to use the `TaskMigrator` class to migrate tasks from old collections ("active", "completed", "archived") to the new project-based system.

## Expected Output Example:

/ *
=== Example 1: Basic Dry Run ===
🚀 Starting task migration to project-based system...
🔍 DRY RUN MODE - No actual changes will be made
📂 Loaded 15 tasks from 'active' collection
📂 Loaded 8 tasks from 'completed' collection  
⚠️  Could not load 'archived' collection (may not exist)
📊 Found 3 unique projects
   • webapp: 10 tasks
   • backend: 8 tasks
   • general: 5 tasks
📁 Creating new project container for 'webapp'
   🧠 Generated embedding for task: task_abc123
   ✅ Migrated task: task_abc123 (status: todo)
   ... (more task migrations)
🔍 DRY RUN: Would save project container: webapp (10 tasks)

📋 Migration Report:
Tasks found: 23
Tasks migrated: 23
Projects processed: 3
*/