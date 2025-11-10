# Task Migration System Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Classes](#core-classes)
4. [Detailed API Documentation](#detailed-api-documentation)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Task Migration System is a comprehensive solution for migrating legacy task data to a project-based organizational structure. This system provides robust migration capabilities with features like dry-run mode, verbose logging, force overwrite protection, and automatic cleanup of legacy data.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                           TaskMigrator                              │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    Migration Operations                       │  │
│  │  ┌──────────────┐  ┌──────────────────┐  ┌─────────────────┐  │  │
│  │  │ loadLegacy   │  │ groupTasksBy     │  │ migrateProject  │  │  │
│  │  │ Tasks        │  │ Project          │  │ Tasks           │  │  │
│  │  └──────────────┘  └──────────────────┘  └─────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    Configuration & Control                   │  │
│  │  ┌──────────────┐  ┌──────────────────┐  ┌─────────────────┐  │  │
│  │  │ withDryRun   │  │ withVerbose      │  │ withForceOver   │  │  │
│  │  │              │  │                  │  │ write           │  │  │
│  │  └──────────────┘  └──────────────────┘  └─────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    Utility Operations                        │  │
│  │  ┌──────────────┐  ┌──────────────────┐  ┌─────────────────┐  │  │
│  │  │ printSummary │  │ validate         │  │ cleanupLegacy   │  │  │
│  │  │              │  │ Migration        │  │                 │  │  │
│  │  └──────────────┘  └──────────────────┘  └─────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                            MigrationCli                             │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    CLI Interface Layer                        │  │
│  │  ┌──────────────┐  ┌──────────────────┐  ┌─────────────────┐  │  │
│  │  │ withDryRun   │  │ withVerbose      │  │ withForce       │  │  │
│  │  └──────────────┘  └──────────────────┘  └─────────────────┘  │  │
│  │  ┌───────────────────────────────────────────────────────────┐  │  │
│  │  │                         run()                             │  │  │
│  │  └───────────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        External Dependencies                        │
│  ┌──────────────┐  ┌──────────────────┐  ┌─────────────────┐       │
│  │   storage    │  │      emb         │  │     error       │       │
│  │              │  │                  │  │                 │       │
│  └──────────────┘  └──────────────────┘  └─────────────────┘       │
└─────────────────────────────────────────────────────────────────────┘
```

## Core Classes

### TaskMigrator

The primary class responsible for migrating tasks from legacy collections to project-based containers.

#### Constructor
```javascript
constructor()
```

Initializes a new TaskMigrator instance with default configuration:
- `dry_run`: false
- `verbose`: false
- `force_overwrite`: false

#### Static Methods

##### new()
```javascript
static new() → TaskMigrator
```
Factory method to create a new TaskMigrator instance.

**Returns:** A new TaskMigrator instance

#### Configuration Methods

##### withDryRun()
```javascript
withDryRun(dryRun) → TaskMigrator
```
Sets the dry run mode. When enabled, no actual changes are made to the system.

**Parameters:**
- `dryRun` (boolean): Whether to enable dry run mode

**Returns:** The current TaskMigrator instance for method chaining

##### withVerbose()
```javascript
withVerbose(verbose) → TaskMigrator
```
Sets verbose logging mode to provide detailed output during migration.

**Parameters:**
- `verbose` (boolean): Whether to enable verbose logging

**Returns:** The current TaskMigrator instance for method chaining

##### withForceOverwrite()
```javascript
withForceOverwrite(forceOverwrite) → TaskMigrator
```
Sets force overwrite mode to allow overwriting existing project containers.

**Parameters:**
- `forceOverwrite` (boolean): Whether to enable force overwrite mode

**Returns:** The current TaskMigrator instance for method chaining

#### Migration Methods

##### migrate()
```javascript
async migrate() → MigrationReport
```
Executes the complete migration process, including loading legacy tasks, grouping by project, and migrating to project containers.

**Returns:** A MigrationReport object containing migration statistics and results

**Throws:** TodoziError if migration fails

##### loadLegacyTasks()
```javascript
loadLegacyTasks(report) → Array<Task>
```
Loads tasks from legacy collections (active, completed, archived).

**Parameters:**
- `report` (MigrationReport): Report object to update with statistics

**Returns:** Array of Task objects loaded from legacy collections

##### groupTasksByProject()
```javascript
groupTasksByProject(tasks) → Object
```
Groups tasks by their project names.

**Parameters:**
- `tasks` (Array<Task>): Array of tasks to group

**Returns:** Object mapping project names to arrays of tasks

##### migrateProjectTasks()
```javascript
async migrateProjectTasks(projectName, tasks) → Object
```
Migrates tasks for a specific project.

**Parameters:**
- `projectName` (string): Name of the project to migrate
- `tasks` (Array<Task>): Array of tasks to migrate

**Returns:** Object containing migration statistics for the project

##### printSummary()
```javascript
printSummary(report) → void
```
Prints a detailed summary of the migration process.

**Parameters:**
- `report` (MigrationReport): Report containing migration statistics

##### validateMigration()
```javascript
validateMigration() → boolean
```
Validates that the migration was successful by comparing task counts.

**Returns:** Boolean indicating whether migration validation passed

##### cleanupLegacy()
```javascript
cleanupLegacy() → void
```
Cleans up empty legacy collections after successful migration.

### MigrationCli

Command-line interface wrapper for the TaskMigrator.

#### Constructor
```javascript
constructor()
```

Initializes a new MigrationCli instance with a default TaskMigrator.

#### Static Methods

##### new()
```javascript
static new() → MigrationCli
```
Factory method to create a new MigrationCli instance.

**Returns:** A new MigrationCli instance

#### Configuration Methods

##### withDryRun()
```javascript
withDryRun(dryRun) → MigrationCli
```
Sets dry run mode for the underlying migrator.

**Parameters:**
- `dryRun` (boolean): Whether to enable dry run mode

**Returns:** The current MigrationCli instance for method chaining

##### withVerbose()
```javascript
withVerbose(verbose) → MigrationCli
```
Sets verbose logging mode for the underlying migrator.

**Parameters:**
- `verbose` (boolean): Whether to enable verbose logging

**Returns:** The current MigrationCli instance for method chaining

##### withForce()
```javascript
withForce(force) → MigrationCli
```
Sets force overwrite mode for the underlying migrator.

**Parameters:**
- `force` (boolean): Whether to enable force overwrite mode

**Returns:** The current MigrationCli instance for method chaining

#### Execution Methods

##### run()
```javascript
async run() → void
```
Executes the migration process through the CLI interface.

## Detailed API Documentation

### MigrationReport Structure

```javascript
{
    tasks_found: 0,
    tasks_migrated: 0,
    projects_migrated: 0,
    project_stats: [],
    errors: []
}
```

### External Dependencies

#### Storage Module
- `loadProjectTaskContainer(projectName)`: Loads a project task container
- `loadTaskCollection(collectionName)`: Loads a legacy task collection
- `saveProjectTaskContainer(container)`: Saves a project task container
- `listProjectTaskContainers()`: Lists all project task containers
- `getStorageDir()`: Gets the storage directory path

#### Embedding Module
- `TodoziEmbeddingService`: Service for generating task embeddings
- `TodoziEmbeddingConfig`: Configuration for embedding service

#### Error Module
- `TodoziError`: Custom error class for the system

## Usage Examples

### Basic Migration
```javascript
const migrator = TaskMigrator.new()
    .withVerbose(true);
    
const report = await migrator.migrate();
console.log(`Migrated ${report.tasks_migrated} tasks across ${report.projects_migrated} projects`);
```

### Dry Run Migration
```javascript
const migrator = TaskMigrator.new()
    .withDryRun(true)
    .withVerbose(true);
    
const report = await migrator.migrate();
console.log("Dry run completed - no changes made");
```

### Force Overwrite Migration
```javascript
const migrator = TaskMigrator.new()
    .withForceOverwrite(true)
    .withVerbose(true);
    
const report = await migrator.migrate();
```

### CLI Usage
```javascript
const cli = MigrationCli.new()
    .withVerbose(true)
    .withDryRun(false)
    .withForce(true);
    
await cli.run();
```

### Validation Only
```javascript
const migrator = TaskMigrator.new()
    .withVerbose(true);
    
const isValid = migrator.validateMigration();
console.log(`Migration validation: ${isValid ? 'PASSED' : 'FAILED'}`);
```

## Design Patterns

### Builder Pattern
The TaskMigrator and MigrationCli classes implement the Builder pattern through method chaining:
```javascript
const migrator = TaskMigrator.new()
    .withDryRun(true)
    .withVerbose(true)
    .withForceOverwrite(false);
```

### Factory Pattern
Factory methods are used for object creation:
```javascript
static new() {
    return new TaskMigrator();
}
```

### Strategy Pattern
Different migration strategies are implemented through configuration options (dry run, force overwrite, verbose).

### Observer Pattern
Progress reporting through the verbose logging mechanism.

## Performance Analysis

### Time Complexity
- **Overall Migration**: O(n × m) where n is the number of tasks and m is the average embedding generation time
- **Task Loading**: O(n) where n is the number of tasks in legacy collections
- **Project Grouping**: O(n) where n is the number of tasks
- **Embedding Generation**: O(m) where m is the complexity of the embedding algorithm

### Space Complexity
- **Memory Usage**: O(n) where n is the total number of tasks being processed
- **Storage**: Additional space required for project containers proportional to legacy data size

### Performance Optimization Recommendations
1. **Batch Processing**: For large datasets, implement batch processing of tasks
2. **Parallel Embedding Generation**: Use concurrent embedding generation for multiple tasks
3. **Memory Management**: Implement streaming for very large task collections
4. **Caching**: Cache embedding service initialization to avoid repeated setup

## Security Considerations

### Data Integrity
- **Validation**: Migration validation ensures data consistency
- **Atomic Operations**: Each project migration is atomic
- **Error Handling**: Comprehensive error handling prevents partial migrations

### Access Control
- **File Permissions**: Ensure proper file permissions on storage directories
- **Configuration Security**: Secure storage of embedding service configurations

### Data Protection
- **Backup**: Automatic validation helps prevent data loss
- **Dry Run**: Dry run mode allows testing without data modification
- **Cleanup**: Safe cleanup of legacy data after successful migration

## Testing Strategies

### Unit Tests
```javascript
// Test basic migration
test('should migrate tasks to projects', async () => {
    const migrator = TaskMigrator.new().withVerbose(true);
    const report = await migrator.migrate();
    expect(report.tasks_migrated).toBeGreaterThan(0);
});

// Test dry run mode
test('should not modify data in dry run mode', async () => {
    const migrator = TaskMigrator.new().withDryRun(true);
    const report = await migrator.migrate();
    // Verify no actual changes were made
});
```

### Integration Tests
```javascript
// Test complete migration workflow
test('should complete full migration workflow', async () => {
    const cli = MigrationCli.new()
        .withVerbose(true)
        .withForce(true);
    
    await cli.run();
    
    // Verify migration results
    const isValid = cli.migrator.validateMigration();
    expect(isValid).toBe(true);
});
```

### Mock Testing
```javascript
// Mock storage operations
jest.mock('./storage', () => ({
    loadTaskCollection: jest.fn(),
    saveProjectTaskContainer: jest.fn(),
    // ... other mocks
}));
```

## Deployment Instructions

### Prerequisites
1. Node.js 14+
2. Required dependencies installed
3. Proper file system permissions
4. Storage directory configured

### Installation
```bash
npm install
# or
yarn install
```

### Configuration
1. Ensure storage directory is properly configured
2. Configure embedding service if required
3. Set appropriate environment variables

### Running the Migration
```bash
# Basic migration
node migrate.js

# Verbose migration
node migrate.js --verbose

# Dry run
node migrate.js --dry-run --verbose

# Force overwrite
node migrate.js --force --verbose
```

### Monitoring
1. Check console output for progress information
2. Review migration reports for statistics
3. Monitor system logs for errors

## Troubleshooting Guide

### Common Issues

#### "No legacy tasks found"
**Cause:** Legacy collections don't exist or are empty
**Solution:** Verify legacy data exists in expected locations

#### "Embedding service initialization failed"
**Cause:** Embedding service configuration issues
**Solution:** Check embedding service configuration and dependencies

#### "Failed to save project container"
**Cause:** File system permissions or disk space issues
**Solution:** Check file permissions and available disk space

### Error Recovery

#### Partial Migration
If migration fails partway through:
1. Run validation to check current state
2. Fix underlying issues
3. Re-run migration with force overwrite if needed

#### Data Corruption
If data corruption is detected:
1. Stop migration process
2. Restore from backups if available
3. Investigate root cause
4. Re-attempt migration with fixes

### Debugging Tips

#### Enable Verbose Logging
```javascript
const migrator = TaskMigrator.new().withVerbose(true);
```

#### Use Dry Run Mode
```javascript
const migrator = TaskMigrator.new().withDryRun(true);
```

#### Check System State
```javascript
const migrator = TaskMigrator.new();
migrator.validateMigration();
migrator.cleanupLegacy();
```

### Performance Monitoring

#### Memory Usage
Monitor Node.js process memory usage during migration of large datasets.

#### Progress Tracking
Use verbose mode to track migration progress in real-time.

#### Error Logging
Implement comprehensive error logging for production deployments.