
import { TaskMigrator, MigrationReport } from '../todozi/migration.js';
import { loadProjectTaskContainer, listProjects } from '../todozi/storage.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import os from 'os';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**

 * Example 3: Advanced Task Migration with Validation
 * 
 * This example shows:
 * 1. Running a dry-run migration to preview changes
 * 2. Executing the actual migration with verbose logging
 * 3. Validating the migration results
 * 4. Cleaning up legacy data after successful migration
 */

async function performAdvancedMigration() {
    console.log('🚀 Starting Advanced Task Migration Process\n');

    // Step 1: Check current system state before migration
    console.log('📊 PRE-MIGRATION ANALYSIS');
    console.log('═'.repeat(50));
    
    const preMigrationProjects = await listProjects();
    console.log(`Existing projects: ${preMigrationProjects.length}`);
    preMigrationProjects.forEach(p => {
        console.log(`  • ${p.name} (${p.description || 'No description'})`);
    });

    // Step 2: Run dry-run migration to preview changes
    console.log('\n🔍 STEP 1: DRY-RUN MIGRATION');
    console.log('═'.repeat(50));
    
    const dryRunReport = await TaskMigrator.new()
        .withDryRun(true)
        .withVerbose(true)
        .withForceOverwrite(false)
        .migrate();

    // Display dry-run results
    console.log('\n📋 Dry-run Summary:');
    console.log(`  Tasks found in legacy system: ${dryRunReport.tasks_found}`);
    console.log(`  Tasks that would be migrated: ${dryRunReport.tasks_migrated}`);
    console.log(`  Projects that would be created/updated: ${dryRunReport.projects_migrated}`);
    
    if (dryRunReport.errors.length > 0) {
        console.log('\n⚠️  Issues detected:');
        dryRunReport.errors.forEach(error => console.log(`  • ${error}`));
    }

    // Step 3: Prompt for confirmation (in real app, you'd use actual user input)
    console.log('\n❓ Dry-run complete. Continue with actual migration? (y/n)');
    // Simulating user confirmation
    const userConfirmed = true; // In real app: read from stdin
    
    if (!userConfirmed) {
        console.log('❌ Migration cancelled by user');
        return;
    }

    // Step 4: Execute actual migration
    console.log('\n⚡ STEP 2: EXECUTING MIGRATION');
    console.log('═'.repeat(50));
    
    const migrator = TaskMigrator.new()
        .withDryRun(false)
        .withVerbose(true)
        .withForceOverwrite(true);

    const migrationReport = await migrator.migrate();
    
    // Display migration results
    console.log('\n✅ Migration Results:');
    console.log(`  Total tasks found: ${migrationReport.tasks_found}`);
    console.log(`  Tasks migrated: ${migrationReport.tasks_migrated}`);
    console.log(`  Projects processed: ${migrationReport.projects_migrated}`);
    
    // Display project details
    if (migrationReport.project_stats.length > 0) {
        console.log('\n📁 Project Migration Details:');
        migrationReport.project_stats.forEach(stats => {
            const icon = stats.initial_tasks > 0 ? '🔄' : '🆕';
            console.log(`  ${icon} ${stats.project_name}:`);
            console.log(`    Initial tasks: ${stats.initial_tasks}`);
            console.log(`    Migrated tasks: ${stats.migrated_tasks}`);
            console.log(`    Final task count: ${stats.final_tasks}`);
        });
    }

    // Step 5: Validate migration integrity
    console.log('\n🔍 STEP 3: VALIDATING MIGRATION');
    console.log('═'.repeat(50));
    
    const isValid = migrator.validateMigration();
    
    if (isValid) {
        console.log('✅ Migration validation passed!');
        
        // Step 6: Show post-migration statistics
        console.log('\n📊 POST-MIGRATION STATISTICS');
        console.log('═'.repeat(50));
        
        const postMigrationProjects = await listProjects();
        console.log(`Total projects after migration: ${postMigrationProjects.length}`);
        
        let totalTasks = 0;
        for (const project of postMigrationProjects) {
            const container = await loadProjectTaskContainer(project.name);
            const projectTasks = container.getAllTasks();
            totalTasks += projectTasks.length;
            
            console.log(`\n📂 Project: ${project.name}`);
            console.log(`  Tasks: ${projectTasks.length}`);
            console.log(`  Status distribution:`);
            
            const statusCounts = {};
            projectTasks.forEach(task => {
                statusCounts[task.status] = (statusCounts[task.status] || 0) + 1;
            });
            
            Object.entries(statusCounts).forEach(([status, count]) => {
                console.log(`    ${status}: ${count}`);
            });
        }
        
        console.log(`\n📈 Total tasks across all projects: ${totalTasks}`);
        
        // Step 7: Clean up legacy data
        console.log('\n🧹 STEP 4: CLEANING UP LEGACY DATA');
        console.log('═'.repeat(50));
        
        // Check what legacy data exists
        const storageDir = path.join(os.homedir(), '.todozi');
        const tasksDir = path.join(storageDir, 'tasks');
        const legacyCollections = ['active.json', 'completed.json', 'archived.json'];
        
        console.log('Checking for legacy task collections...');
        let hasLegacyData = false;
        
        for (const collection of legacyCollections) {
            const collectionPath = path.join(tasksDir, collection);
            if (fs.existsSync(collectionPath)) {
                hasLegacyData = true;
                const stats = fs.statSync(collectionPath);
                console.log(`  ✓ Found ${collection} (${stats.size} bytes)`);
            }
        }
        
        if (hasLegacyData) {
            console.log('\n🗑️  Cleaning up legacy collections...');
            migrator.cleanupLegacy();
            console.log('✅ Legacy cleanup completed');
        } else {
            console.log('ℹ️  No legacy collections found to clean up');
        }
        
        // Step 8: Generate migration summary report
        console.log('\n📄 MIGRATION SUMMARY REPORT');
        console.log('═'.repeat(50));
        
        const summary = {
            migration_timestamp: new Date().toISOString(),
            tasks_found: migrationReport.tasks_found,
            tasks_migrated: migrationReport.tasks_migrated,
            projects_created: migrationReport.projects_migrated,
            validation_passed: isValid,
            legacy_cleaned: hasLegacyData,
            errors: migrationReport.errors,
            project_details: migrationReport.project_stats
        };
        
        // Save summary to file
        const summaryPath = path.join(storageDir, 'migration_summary.json');
        fs.writeFileSync(summaryPath, JSON.stringify(summary, null, 2));
        console.log(`📝 Migration summary saved to: ${summaryPath}`);
        
        // Display final summary
        console.log('\n🎉 Migration completed successfully!');
        console.log(`📊 ${migrationReport.tasks_migrated} tasks migrated to ${migrationReport.projects_migrated} projects`);
        console.log('✅ All validations passed');
        console.log('🧹 Legacy data cleaned up');
        
    } else {
        console.log('❌ Migration validation failed!');
        console.log('Please check the migration logs and retry if necessary');
        
        // Display any errors that occurred
        if (migrationReport.errors.length > 0) {
            console.log('\n🚨 Errors encountered:');
            migrationReport.errors.forEach(error => {
                console.log(`  • ${error}`);
            });
        }
    }
}

/**
 * Helper function to check migration prerequisites
 */
async function checkMigrationPrerequisites() {
    console.log('🔍 Checking migration prerequisites...\n');
    
    const checks = [];
    
    // Check storage directory exists
    const storageDir = path.join(os.homedir(), '.todozi');
    if (fs.existsSync(storageDir)) {
        checks.push('✅ Storage directory exists');
    } else {
        checks.push('❌ Storage directory missing');
        return false;
    }
    
    // Check if we have write permissions
    try {
        const testFile = path.join(storageDir, 'migration_test.tmp');
        fs.writeFileSync(testFile, 'test');
        fs.unlinkSync(testFile);
        checks.push('✅ Write permissions confirmed');
    } catch (e) {
        checks.push('❌ No write permissions');
        return false;
    }
    
    // Check if legacy task collections exist
    const tasksDir = path.join(storageDir, 'tasks');
    const hasLegacyCollections = ['active.json', 'completed.json', 'archived.json']
        .some(file => fs.existsSync(path.join(tasksDir, file)));
    
    if (hasLegacyCollections) {
        checks.push('✅ Legacy task collections found');
    } else {
        checks.push('ℹ️  No legacy task collections found');
    }
    
    checks.forEach(check => console.log(check));
    return true;
}

// Main execution
async function main() {
    console.log('═══════════════════════════════════════════════════════════════');
    console.log('🚀 TODOZI ADVANCED MIGRATION EXAMPLE');
    console.log('═══════════════════════════════════════════════════════════════\n');
    
    // Check prerequisites
    if (!await checkMigrationPrerequisites()) {
        console.log('\n❌ Migration prerequisites not met. Please resolve issues and try again.');
        process.exit(1);
    }
    
    // Perform the migration
    try {
        await performAdvancedMigration();
    } catch (error) {
        console.error('\n💥 Migration failed:', error.message);
        console.error('Please check the error details above and try again.');
        process.exit(1);
    }
}

// Run the example
main().catch(console.error);

/*
# Example 3: Advanced Task Migration with Validation and Cleanup

This example demonstrates how to use the `TaskMigrator` class to perform a complete migration from the legacy task system to the new project-based architecture, including validation and cleanup of old data.

## Key Features Demonstrated:

1. **Dry-Run Migration**: Preview what changes will be made before executing
2. **Verbose Logging**: Detailed output showing migration progress
3. **Validation**: Ensures data integrity after migration
4. **Cleanup**: Removes legacy collections after successful migration
5. **Error Handling**: Comprehensive error checking and reporting
6. **Statistics**: Detailed pre and post-migration analytics

## Usage:

/ *
bash
# Run the advanced migration example
node example3_advanced_migration.js
*/