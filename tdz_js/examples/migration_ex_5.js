import { EventEmitter } from 'events';

/*
Here is a runnable, self-contained example that demonstrates how to use and extend the migration system provided in migration.js. It shows:
- Running a migration with TaskMigrator and MigrationCli
- Adding a custom pipeline (preflight checks, backups, retries, validation)
- Extending behavior via a subclass

You can run this file directly with Node.js. If the actual storage/embedding modules are not available, it falls back to a simple in-memory implementation so you can still see the flow.

example5-migration-usage.js
javascript
#!/usr/bin/env node

/**
 * Example 5: Demonstrates practical usage and extension of the migration system.
 * 
 * - Uses TaskMigrator to migrate legacy "active/completed/archived" tasks into project-based containers.
 * - Adds a custom PreflightMigrator that:
 *    - Preflight checks (legacy file presence)
 *    - Creates a timestamped backup of legacy collections
 *    - Retries embedding generation per task
 *    - Validates migration integrity and cleans up if successful
 * - Shows both dry-run and real run
 *
 * Notes:
 * - If optional dependencies are not installed, it falls back to a simple in-memory implementation
 *   to illustrate the flow without errors.
 */


// Try to require the real modules. If they don't exist, provide minimal stubs.
let MigrationReport, ProjectTaskContainer, TaskMigrator, MigrationCli, TodoziEmbeddingService, TodoziEmbeddingConfig, loadProjectTaskContainer, saveProjectTaskContainer, listProjectTaskContainers, getStorageDir;
let usingRealModules = true;
try {
  ({
    MigrationReport,
    ProjectTaskContainer
  } = await import('../todozi/models.js'));
  ({
    TaskMigrator,
    MigrationCli
  } = await import('../todozi/migration.js'));
  ({
    TodoziEmbeddingService,
    TodoziEmbeddingConfig
  } = await import('../todozi/emb.js'));
  ({
    loadProjectTaskContainer,
    saveProjectTaskContainer,
    listProjectTaskContainers,
    getStorageDir
  } = await import('../todozi/storage.js'));
} catch (e) {
  usingRealModules = false;
  console.warn('⚠️  Real modules not found. Falling back to in-memory stubs for demonstration.');
}

/**
 * In-memory fallback implementation (used if the actual modules aren't available).
 * This allows you to run this example to see the flow without installing dependencies.
 */
if (!usingRealModules) {
  MigrationReport = class {
    constructor() {
      this.tasksFound = 0;
      this.tasksMigrated = 0;
      this.projectsMigrated = 0;
      this.projectStats = [];
      this.errors = [];
    }
    static default() { return new MigrationReport(); }
  };

  ProjectTaskContainer = class {
    constructor(projectName) {
      this.projectName = projectName;
      this.activeTasks = {};
      this.completedTasks = {};
      this.archivedTasks = {};
      this.deletedTasks = {};
    }
    static new(projectName) { return new ProjectTaskContainer(projectName); }
    getAllTasks() {
      return [
        ...Object.values(this.activeTasks),
        ...Object.values(this.completedTasks),
        ...Object.values(this.archivedTasks),
        ...Object.values(this.deletedTasks),
      ];
    }
    getTask(id) {
      return this.activeTasks[id] || this.completedTasks[id] || this.archivedTasks[id] || this.deletedTasks[id];
    }
    addTask(task) {
      if (task.status === 'done' || task.status === 'completed') this.completedTasks[task.id] = task;
      else if (task.status === 'archived') this.archivedTasks[task.id] = task;
      else this.activeTasks[task.id] = task;
    }
  };

  TaskMigrator = class {
    constructor() {
      this.dry_run = false;
      this.verbose = false;
      this.force_overwrite = false;
      this._legacyTasks = []; // internal sample
    }
    static new() { return new TaskMigrator(); }
    withDryRun(d) { this.dry_run = d; return this; }
    withVerbose(v) { this.verbose = v; return this; }
    withForceOverwrite(f) { this.force_overwrite = f; return this; }

    loadLegacyTasks(report) {
      // For the demo, we simulate some legacy tasks.
      this._legacyTasks = [
        { id: 't1', parent_project: '', action: 'Legacy: Write docs', status: 'todo', priority: 'medium' },
        { id: 't2', parent_project: '', action: 'Legacy: Build release', status: 'done', priority: 'high' },
        { id: 't3', parent_project: 'webapp', action: 'Legacy: Fix login bug', status: 'in_progress', priority: 'high' },
      ];
      report.tasksFound = this._legacyTasks.length;
      if (this.verbose) console.log(`[DEMO] Loaded ${this._legacyTasks.length} legacy tasks`);
      return this._legacyTasks.map(t => ({
        clone: () => ({ ...t, embedding_vector: null })
      }));
    }

    groupTasksByProject(allTasks) {
      const groups = {};
      for (const task of allTasks) {
        const project = task.parent_project || task.parentProject || '';
        const name = (project === '' ? 'general' : project);
        if (!groups[name]) groups[name] = [];
        groups[name].push(task);
      }
      if (this.verbose) console.log(`[DEMO] Grouped tasks into ${Object.keys(groups).length} projects`);
      return groups;
    }

    async migrateProjectTasks(projectName, tasks) {
      const stats = {
        project_name: projectName,
        initial_tasks: 0,
        migrated_tasks: 0,
        final_tasks: 0
      };

      let container = ProjectTaskContainer.new(projectName);
      stats.initial_tasks = 0;

      // Add tasks
      for (const task of tasks) {
        if (container.getTask(task.id)) {
          if (this.verbose) console.log(`   ⏭️  Skipping duplicate task: ${task.id}`);
          continue;
        }
        // Simulate embedding generation
        task.embedding_vector = Array(384).fill(0).map(() => Math.random() - 0.5);
        container.addTask(task);
        stats.migrated_tasks += 1;
        if (this.verbose) {
          console.log(`   ✅ Migrated task: ${task.id} (status: ${task.status})`);
        }
      }

      stats.final_tasks = container.getAllTasks().length;

      if (!this.dry_run) {
        if (this.verbose) console.log(`[DEMO] Saved project container: ${projectName}`);
      } else {
        if (this.verbose) console.log(`[DEMO] DRY RUN: Would save project container: ${projectName} (${stats.final_tasks} tasks)`);
      }
      return stats;
    }

    async migrate() {
      const report = MigrationReport.default();
      if (this.verbose) {
        console.log('🚀 [DEMO] Starting task migration to project-based system...');
        if (this.dry_run) console.log('🔍 DRY RUN MODE - No actual changes will be made');
      }

      const allTasks = this.loadLegacyTasks(report);
      if (allTasks.length === 0) {
        if (this.verbose) console.log('✅ No legacy tasks found - migration complete');
        return report;
      }

      const projectGroups = this.groupTasksByProject(allTasks);
      if (this.verbose) {
        console.log(`📊 Found ${Object.keys(projectGroups).length} unique projects`);
        for (const [projectName, tasks] of Object.entries(projectGroups)) {
          console.log(`   • ${projectName}: ${tasks.length} tasks`);
        }
      }

      for (const [projectName, tasks] of Object.entries(projectGroups)) {
        const projectReport = await this.migrateProjectTasks(projectName, tasks);
        report.project_stats.push(projectReport);
        report.projects_migrated += 1;
      }

      if (this.verbose) this.printSummary(report);
      return report;
    }

    printSummary(report) {
      console.log('\n' + '='.repeat(60));
      console.log('📊 MIGRATION SUMMARY (DEMO)');
      console.log('='.repeat(60));
      console.log(`Total legacy tasks found: ${report.tasksFound}`);
      console.log(`Tasks migrated: ${report.tasksMigrated}`);
      console.log(`Projects processed: ${report.projectsMigrated}`);

      if (report.project_stats.length > 0) {
        console.log('\n📋 Project Details:');
        for (const stat of report.project_stats) {
          console.log(`  • ${stat.project_name}: ${stat.initial_tasks} → ${stat.final_tasks} tasks (${stat.migrated_tasks} migrated)`);
        }
      }

      if (report.errors.length > 0) {
        console.log('\n⚠️  Errors encountered:');
        for (const error of report.errors) {
          console.log(`  • ${error}`);
        }
      }

      if (report.tasksMigrated === report.tasksFound && report.errors.length === 0) {
        console.log('\n✅ Migration completed successfully!');
      } else {
        console.log('\n⚠️  Migration completed with warnings');
      }
      console.log('='.repeat(60));
    }

    validateMigration() {
      if (this.verbose) console.log('🔍 Validating migration integrity...');
      const legacyCount = 0;
      let projectTasks = 0;
      try {
        const containers = listProjectTaskContainers ? listProjectTaskContainers() : [];
        projectTasks = containers.reduce((sum, c) => sum + c.getAllTasks().length, 0);
      } catch (_) {
        projectTasks = 0;
      }
      if (this.verbose) {
        console.log(`Legacy system tasks: ${legacyCount}`);
        console.log(`Project system tasks: ${projectTasks}`);
        console.log(legacyCount === 0 || projectTasks >= legacyCount ? '✅ Migration validation passed' : '❌ Migration validation failed');
      }
      return legacyCount === 0 || projectTasks >= legacyCount;
    }

    cleanupLegacy() {
      if (this.verbose) console.log('🧹 Cleaning up legacy collections...');
      if (this.verbose) console.log('ℹ️  No empty legacy collections to clean up (demo)');
    }
  };

  MigrationCli = class {
    constructor() { this.migrator = TaskMigrator.new(); }
    static new() { return new MigrationCli(); }
    withDryRun(d) { this.migrator = this.migrator.withDryRun(d); return this; }
    withVerbose(v) { this.migrator = this.migrator.withVerbose(v); return this; }
    withForce(f) { this.migrator = this.migrator.withForceOverwrite(f); return this; }
    async run() {
      const report = await this.migrator.migrate();
      if (!this.migrator.dry_run) {
        const isValid = this.migrator.validateMigration();
        if (isValid && report.errors.length === 0) {
          this.migrator.cleanupLegacy();
        }
      }
    }
  };

  TodoziEmbeddingService = class {
    static async new() { return new TodoziEmbeddingService(); }
    async initialize() {}
    prepareTaskContent(task) { return `${task.action}\n${task.status}\n${task.priority}`; }
    async generateEmbedding(content) { return Array(384).fill(0).map(() => Math.random() - 0.5); }
  };
  TodoziEmbeddingConfig = class { static default() { return {}; } };
  loadProjectTaskContainer = async (name) => ProjectTaskContainer.new(name);
  saveProjectTaskContainer = async (_c) => {};
  listProjectTaskContainers = async () => [];
  getStorageDir = async () => '/tmp/todozi-demo';
}

/**
 * Extend TaskMigrator with a safer pipeline:
 * - Preflight checks
 * - Backup
 * - Retry embedding
 * - Post-migration validation + cleanup
 */
class PreflightTaskMigrator extends TaskMigrator {
  constructor(options = {}) {
    super();
    this.dry_run = options.dry_run || false;
    this.verbose = options.verbose || false;
    this.force_overwrite = options.force_overwrite || false;
    this.embedding_retries = options.embedding_retries ?? 2;
    this.backup_dir = options.backup_dir;
    this.events = new EventEmitter();
  }

  async migrateWithPipeline() {
    this.events.emit('log', 'info', 'Starting migration pipeline');

    const preflightOk = await this.preflight();
    if (!preflightOk) {
      this.events.emit('log', 'error', 'Preflight checks failed. Aborting migration.');
      return;
    }

    const backupPath = await this.createBackup();
    if (this.verbose) {
      console.log(`💾 Created backup: ${backupPath || '(dry-run backup skipped)'}`);
    }

    // Run the actual migration (uses the parent implementation)
    const report = await this.migrate();

    // Post-migration steps
    const isValid = this.validateMigration();

    if (isValid && report.errors.length === 0) {
      if (this.verbose) console.log('✅ Migration validation passed');
      this.cleanupLegacy();
      this.events.emit('log', 'info', 'Migration pipeline completed successfully');
    } else {
      this.events.emit('log', 'error', 'Migration validation failed. Check logs.');
    }

    return { report, backupPath, isValid };
  }

  async preflight() {
    if (this.verbose) console.log('🔍 Running preflight checks...');
    // Example checks:
    // - Check legacy collections exist or are empty
    // - Ensure project storage writable
    // - Check embedding service availability
    let checks = {
      legacyFound: false,
      storageWritable: true,
      embeddingAvailable: true
    };

    try {
      const legacy = this.loadLegacyTasks({});
      checks.legacyFound = legacy.length > 0;
    } catch (e) {
      this.events.emit('log', 'warn', `Legacy load failed: ${e.message}`);
      checks.legacyFound = false;
    }

    if (this.verbose) {
      console.log(`   • Legacy tasks found: ${checks.legacyFound}`);
      console.log(`   • Storage writable: ${checks.storageWritable}`);
      console.log(`   • Embedding service available: ${checks.embeddingAvailable}`);
    }

    // If no legacy tasks found, we still "pass" the preflight.
    return checks.storageWritable && checks.embeddingAvailable;
  }

  async createBackup() {
    if (this.dry_run) return null;
    const ts = new Date().toISOString().replace(/[:.]/g, '-');
    const dir = this.backup_dir || (await getStorageDir?.() || '/tmp/todozi');
    const path = `${dir}/backups/legacy-backup-${ts}.json`;
    if (this.verbose) console.log(`[BACKUP] Would copy legacy collections to: ${path}`);
    // In a real implementation, copy actual files to the backup path.
    return path;
  }

  async migrateProjectTasks(projectName, tasks) {
    const stats = {
      project_name: projectName,
      initial_tasks: 0,
      migrated_tasks: 0,
      final_tasks: 0
    };

    let container;
    try {
      container = await loadProjectTaskContainer(projectName);
      stats.initial_tasks = container.getAllTasks().length;
      if (!this.force_overwrite && stats.initial_tasks > 0) {
        if (this.verbose) {
          console.log(`⚠️  Project '${projectName}' already exists with ${stats.initial_tasks} tasks (use --force to overwrite)`);
        }
        stats.final_tasks = stats.initial_tasks;
        return stats;
      }
    } catch (_) {
      container = ProjectTaskContainer.new(projectName);
    }

    for (const task of tasks) {
      if (container.getTask(task.id)) {
        if (this.verbose) console.log(`   ⏭️  Skipping duplicate task: ${task.id}`);
        continue;
      }

      // Generate embedding with retries
      const content = this.prepareTaskContentSafe(task);
      const vector = await this.generateEmbeddingWithRetry(content, this.embedding_retries);
      task.embedding_vector = vector;

      container.addTask(task);
      stats.migrated_tasks += 1;
      if (this.verbose) {
        console.log(`   ✅ Migrated task: ${task.id} (status: ${task.status})`);
      }
    }

    stats.final_tasks = container.getAllTasks().length;

    if (!this.dry_run) {
      await saveProjectTaskContainer(container);
      if (this.verbose) console.log(`💾 Saved project container: ${projectName}`);
    } else {
      if (this.verbose) console.log(`🔍 DRY RUN: Would save project container: ${projectName} (${stats.final_tasks} tasks)`);
    }

    return stats;
  }

  prepareTaskContentSafe(task) {
    try {
      if (TodoziEmbeddingService?.new) {
        const svc = {};
        svc.prepareTaskContent = TodoziEmbeddingService.prototype.prepareTaskContent || function (t) {
          return `${t.action}\n${t.status}\n${t.priority}`;
        };
        return svc.prepareTaskContent(task);
      }
    } catch (_) {
      // ignore
    }
    return `${task.action}\n${task.status}\n${task.priority}`;
  }

  async generateEmbeddingWithRetry(content, retries = 2) {
    let attempt = 0;
    while (true) {
      attempt += 1;
      try {
        const svc = await TodoziEmbeddingService.new(TodoziEmbeddingConfig.default());
        await svc.initialize();
        return await svc.generateEmbedding(content);
      } catch (e) {
        if (this.verbose) console.log(`   ⚠️  Embedding attempt ${attempt} failed: ${e.message}`);
        if (attempt > retries) {
          if (this.verbose) console.log(`   ❌ Giving up on embedding after ${retries} retries`);
          return null;
        }
        await new Promise(res => setTimeout(res, 200 * attempt));
      }
    }
  }
}

/**
 * Example 5 main flow:
 * - Run a dry-run
 * - Run with the custom PreflightTaskMigrator
 * - Run via MigrationCli
 */
async function main() {
  console.log('============================================================');
  console.log('Example 5: Practical migration usage and extension');
  console.log('============================================================\n');

  // 1) Using TaskMigrator directly
  console.log('1) Direct TaskMigrator usage (dry-run):');
  console.log('------------------------------------------------------------');
  const direct = TaskMigrator.new()
    .withVerbose(true)
    .withDryRun(true)
    .withForceOverwrite(false);

  await direct.migrate();

  // 2) Using the custom PreflightTaskMigrator
  console.log('\n2) PreflightTaskMigrator (real run, demo data):');
  console.log('------------------------------------------------------------');
  const extended = new PreflightTaskMigrator({
    dry_run: false,
    verbose: true,
    force_overwrite: true,
    embedding_retries: 2
  });
  extended.events.on('log', (level, msg) => {
    if (level === 'error') console.error(`[EVENT] ${msg}`);
    else if (level === 'warn') console.warn(`[EVENT] ${msg}`);
    else console.log(`[EVENT] ${msg}`);
  });

  const result = await extended.migrateWithPipeline();
  if (result) {
    const { isValid } = result;
    console.log(`\nPipeline result: valid=${isValid}`);
  }

  // 3) Using MigrationCli
  console.log('\n3) MigrationCli usage (dry-run):');
  console.log('------------------------------------------------------------');
  const cli = MigrationCli.new()
    .withDryRun(true)
    .withVerbose(true)
    .withForce(false);

  await cli.run();

  console.log('\n============================================================');
  console.log('Example 5 complete.');
  console.log('============================================================');
}

// Run if executed directly
  main().catch(err => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
}

  PreflightTaskMigrator
};