use crate::error::{Result, TodoziError};
use crate::models::{MigrationReport, ProjectMigrationStats, ProjectTaskContainer, Task};
use crate::storage::{
    load_project_task_container, load_task_collection, save_project_task_container,
};
use std::collections::HashMap;
pub struct TaskMigrator {
    dry_run: bool,
    verbose: bool,
    force_overwrite: bool,
}
impl TaskMigrator {
    pub fn new() -> Self {
        Self {
            dry_run: false,
            verbose: false,
            force_overwrite: false,
        }
    }
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    pub fn force_overwrite(mut self, force_overwrite: bool) -> Self {
        self.force_overwrite = force_overwrite;
        self
    }
    pub async fn migrate(&self) -> Result<MigrationReport> {
        let mut report = MigrationReport::default();
        if self.verbose {
            println!("🚀 Starting task migration to project-based system...");
            if self.dry_run {
                println!("🔍 DRY RUN MODE - No actual changes will be made");
            }
        }
        let all_tasks = self.load_legacy_tasks(&mut report)?;
        if all_tasks.is_empty() {
            if self.verbose {
                println!("✅ No legacy tasks found - migration complete");
            }
            return Ok(report);
        }
        let project_groups = self.group_tasks_by_project(all_tasks);
        if self.verbose {
            println!("📊 Found {} unique projects", project_groups.len());
            for (project_name, tasks) in &project_groups {
                println!("   • {}: {} tasks", project_name, tasks.len());
            }
        }
        for (project_name, tasks) in project_groups {
            let project_report = self.migrate_project_tasks(&project_name, tasks).await?;
            report.project_stats.push(project_report);
            report.projects_migrated += 1;
        }
        if self.verbose {
            self.print_summary(&report);
        }
        Ok(report)
    }
    fn load_legacy_tasks(&self, report: &mut MigrationReport) -> Result<Vec<Task>> {
        let collections = ["active", "completed", "archived"];
        let mut all_tasks = Vec::new();
        for collection_name in &collections {
            match load_task_collection(collection_name) {
                Ok(collection) => {
                    for task in collection.tasks.values() {
                        all_tasks.push(task.clone());
                        report.tasks_found += 1;
                    }
                    if self.verbose {
                        println!(
                            "📂 Loaded {} tasks from '{}' collection", collection.tasks
                            .len(), collection_name
                        );
                    }
                }
                Err(_) => {
                    if self.verbose {
                        println!(
                            "⚠️  Could not load '{}' collection (may not exist)",
                            collection_name
                        );
                    }
                }
            }
        }
        Ok(all_tasks)
    }
    fn group_tasks_by_project(&self, tasks: Vec<Task>) -> HashMap<String, Vec<Task>> {
        let mut project_groups: HashMap<String, Vec<Task>> = HashMap::new();
        for task in tasks {
            let project = if task.parent_project.is_empty() {
                "general".to_string()
            } else {
                task.parent_project.clone()
            };
            project_groups.entry(project).or_insert_with(Vec::new).push(task);
        }
        project_groups
    }
    async fn migrate_project_tasks(
        &self,
        project_name: &str,
        tasks: Vec<Task>,
    ) -> Result<ProjectMigrationStats> {
        let mut stats = ProjectMigrationStats {
            project_name: project_name.to_string(),
            initial_tasks: 0,
            migrated_tasks: 0,
            final_tasks: 0,
        };
        match load_project_task_container(project_name) {
            Ok(existing_container) => {
                stats.initial_tasks = existing_container.get_all_tasks().len();
                if !self.force_overwrite && stats.initial_tasks > 0 {
                    if self.verbose {
                        println!(
                            "⚠️  Project '{}' already exists with {} tasks (use --force to overwrite)",
                            project_name, stats.initial_tasks
                        );
                    }
                    stats.final_tasks = stats.initial_tasks;
                    return Ok(stats);
                }
            }
            Err(_) => {
                if self.verbose {
                    println!(
                        "📁 Creating new project container for '{}'", project_name
                    );
                }
            }
        }
        let mut container = load_project_task_container(project_name)
            .unwrap_or_else(|_| ProjectTaskContainer::new(project_name));
        let _initial_count = container.get_all_tasks().len();
        for mut task in tasks {
            if container.get_task(&task.id).is_some() {
                if self.verbose {
                    println!("   ⏭️  Skipping duplicate task: {}", task.id);
                }
                continue;
            }
            
            if let Ok(mut emb_service) = crate::emb::TodoziEmbeddingService::new(
                    crate::emb::TodoziEmbeddingConfig::default(),
                )
                .await
            {
                match emb_service.initialize().await {
                    Ok(_) => {
                        let embedding_content = emb_service
                            .prepare_task_content(&task);
                        match emb_service
                            .generate_embedding(&embedding_content)
                            .await
                        {
                            Ok(vector) => {
                                task.embedding_vector = Some(vector);
                                if self.verbose {
                                    println!(
                                        "   🧠 Generated embedding for task: {}", task.id
                                    );
                                }
                            }
                            Err(e) => {
                                if self.verbose {
                                    println!(
                                        "   ❌ Embedding generation failed for task {}: {}", task
                                        .id, e
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if self.verbose {
                            println!(
                                "   ⚠️  Embedding service initialization failed: {}", e
                            );
                        }
                    }
                }
            }
            
            container.add_task(task);
            stats.migrated_tasks += 1;
            if self.verbose {
                println!(
                    "   ✅ Migrated task: {} (status: {:?})", container.get_all_tasks()
                    .last().unwrap().id, container.get_all_tasks().last().unwrap().status
                );
            }
        }
        stats.final_tasks = container.get_all_tasks().len();
        if !self.dry_run {
            match save_project_task_container(&container) {
                Ok(_) => {
                    if self.verbose {
                        println!("💾 Saved project container: {}", project_name);
                    }
                }
                Err(e) => {
                    let error_msg = format!(
                        "Failed to save project container '{}': {}", project_name, e
                    );
                    if self.verbose {
                        println!("❌ {}", error_msg);
                    }
                    return Err(TodoziError::storage(&error_msg));
                }
            }
        } else {
            if self.verbose {
                println!(
                    "🔍 DRY RUN: Would save project container: {} ({} tasks)",
                    project_name, stats.final_tasks
                );
            }
        }
        Ok(stats)
    }
    fn print_summary(&self, report: &MigrationReport) {
        println!("\n{}", "=".repeat(60));
        println!("📊 MIGRATION SUMMARY");
        println!("{}", "=".repeat(60));
        println!("Total legacy tasks found: {}", report.tasks_found);
        println!("Tasks migrated: {}", report.tasks_migrated);
        println!("Projects processed: {}", report.projects_migrated);
        if !report.project_stats.is_empty() {
            println!("\n📋 Project Details:");
            for stat in &report.project_stats {
                println!(
                    "  • {}: {} → {} tasks ({} migrated)", stat.project_name, stat
                    .initial_tasks, stat.final_tasks, stat.migrated_tasks
                );
            }
        }
        if !report.errors.is_empty() {
            println!("\n⚠️  Errors encountered:");
            for error in &report.errors {
                println!("  • {}", error);
            }
        }
        if report.tasks_migrated == report.tasks_found && report.errors.is_empty() {
            println!("\n✅ Migration completed successfully!");
        } else {
            println!("\n⚠️  Migration completed with warnings");
        }
        println!("{}", "=".repeat(60));
    }
    pub fn validate_migration(&self) -> Result<bool> {
        if self.verbose {
            println!("🔍 Validating migration integrity...");
        }
        let legacy_tasks = ["active", "completed", "archived"]
            .iter()
            .map(|collection| {
                load_task_collection(collection).map(|c| c.tasks.len()).unwrap_or(0)
            })
            .sum::<usize>();
        let project_tasks = crate::storage::list_project_task_containers()
            .map(|containers| {
                containers.iter().map(|c| c.get_all_tasks().len()).sum::<usize>()
            })
            .unwrap_or(0);
        if self.verbose {
            println!("Legacy system tasks: {}", legacy_tasks);
            println!("Project system tasks: {}", project_tasks);
        }
        let is_valid = legacy_tasks == 0
            || (legacy_tasks > 0 && project_tasks >= legacy_tasks);
        if is_valid {
            if self.verbose {
                println!("✅ Migration validation passed");
            }
        } else {
            if self.verbose {
                println!("❌ Migration validation failed");
            }
        }
        Ok(is_valid)
    }
    pub fn cleanup_legacy(&self) -> Result<()> {
        if self.verbose {
            println!("🧹 Cleaning up legacy collections...");
        }
        let collections = ["active", "completed", "archived"];
        let mut cleaned_count = 0;
        for collection_name in &collections {
            match load_task_collection(collection_name) {
                Ok(collection) => {
                    if collection.tasks.is_empty() {
                        let storage_dir = crate::storage::get_storage_dir()?;
                        let collection_path = storage_dir
                            .join("tasks")
                            .join(format!("{}.json", collection_name));
                        if collection_path.exists() {
                            if self.dry_run {
                                if self.verbose {
                                    println!(
                                        "   🔍 DRY RUN: Would remove empty collection '{}'",
                                        collection_name
                                    );
                                }
                            } else {
                                match std::fs::remove_file(&collection_path) {
                                    Ok(_) => {
                                        if self.verbose {
                                            println!(
                                                "   🗑️  Removed empty collection '{}'", collection_name
                                            );
                                        }
                                        cleaned_count += 1;
                                    }
                                    Err(e) => {
                                        if self.verbose {
                                            println!(
                                                "   ⚠️  Could not remove '{}': {}", collection_name, e
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if self.verbose {
                            println!(
                                "   ⚠️  Collection '{}' still has {} tasks - not removing",
                                collection_name, collection.tasks.len()
                            );
                        }
                    }
                }
                Err(_) => {
                    if self.verbose {
                        println!(
                            "   ℹ️  Collection '{}' does not exist", collection_name
                        );
                    }
                }
            }
        }
        if self.verbose {
            if cleaned_count > 0 {
                println!("✅ Cleaned up {} empty legacy collections", cleaned_count);
            } else {
                println!("ℹ️  No empty legacy collections to clean up");
            }
        }
        Ok(())
    }
}
impl Default for TaskMigrator {
    fn default() -> Self {
        Self::new()
    }
}
pub struct MigrationCli {
    migrator: TaskMigrator,
}
impl MigrationCli {
    pub fn new() -> Self {
        Self {
            migrator: TaskMigrator::new(),
        }
    }
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.migrator = self.migrator.dry_run(dry_run);
        self
    }
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.migrator = self.migrator.verbose(verbose);
        self
    }
    pub fn with_force(mut self, force: bool) -> Self {
        self.migrator = self.migrator.force_overwrite(force);
        self
    }
    pub async fn run(self) -> Result<()> {
        let report = self.migrator.migrate().await?;
        if !self.migrator.dry_run {
            let is_valid = self.migrator.validate_migration()?;
            if is_valid && report.errors.is_empty() {
                self.migrator.cleanup_legacy()?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_migrator_creation() {
        let migrator = TaskMigrator::new();
        assert!(! migrator.dry_run);
        assert!(! migrator.verbose);
        assert!(! migrator.force_overwrite);
    }
    #[test]
    fn test_task_migrator_builder() {
        let migrator = TaskMigrator::new()
            .dry_run(true)
            .verbose(true)
            .force_overwrite(true);
        assert!(migrator.dry_run);
        assert!(migrator.verbose);
        assert!(migrator.force_overwrite);
    }
    #[test]
    fn test_migration_cli_builder() {
        let _cli = MigrationCli::new()
            .with_dry_run(true)
            .with_verbose(true)
            .with_force(true);
        assert!(true);
    }
}