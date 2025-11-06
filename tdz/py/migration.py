"""Task migration utilities for Todozi."""

from typing import Dict, List
from dataclasses import dataclass

from .models import Task, MigrationReport, ProjectMigrationStats, ProjectTaskContainer


@dataclass
class TaskMigrator:
    """Handles task migration."""
    dry_run: bool = False
    verbose: bool = False
    force_overwrite: bool = False

    async def migrate(self) -> MigrationReport:
        """Migrate tasks to project-based system."""
        report = MigrationReport()

        if self.verbose:
            print("🚀 Starting task migration to project-based system...")
            if self.dry_run:
                print("🔍 DRY RUN MODE - No actual changes will be made")

        # Load legacy tasks
        all_tasks = self.load_legacy_tasks(report)

        if not all_tasks:
            if self.verbose:
                print("✅ No legacy tasks found - migration complete")
            return report

        # Group by project
        project_groups = self.group_tasks_by_project(all_tasks)

        if self.verbose:
            print(f"📊 Found {len(project_groups)} unique projects")
            for project_name, tasks in project_groups.items():
                print(f"   • {project_name}: {len(tasks)} tasks")

        # Migrate each project
        for project_name, tasks in project_groups.items():
            project_report = await self.migrate_project_tasks(project_name, tasks)
            report.project_stats.append(project_report)
            report.projects_migrated += 1

        if self.verbose:
            self.print_summary(report)

        return report

    def load_legacy_tasks(self, report: MigrationReport) -> List[Task]:
        """Load tasks from legacy collections."""
        from .storage import load_task_collection

        all_tasks = []
        for collection_name in ["active", "completed", "archived"]:
            try:
                collection = load_task_collection(collection_name)
                all_tasks.extend(collection.get_all_tasks())
                report.tasks_found += len(collection.tasks)
                if self.verbose:
                    print(f"📂 Loaded {len(collection.tasks)} tasks from '{collection_name}' collection")
            except:
                if self.verbose:
                    print(f"⚠️  Could not load '{collection_name}' collection (may not exist)")

        return all_tasks

    def group_tasks_by_project(self, tasks: List[Task]) -> Dict[str, List[Task]]:
        """Group tasks by project."""
        project_groups: Dict[str, List[Task]] = {}

        for task in tasks:
            project = task.parent_project if task.parent_project else "general"
            if project not in project_groups:
                project_groups[project] = []
            project_groups[project].append(task)

        return project_groups

    async def migrate_project_tasks(self, project_name: str, tasks: List[Task]) -> ProjectMigrationStats:
        """Migrate tasks for a specific project."""
        from .storage import load_project_task_container, save_project_task_container

        stats = ProjectMigrationStats(
            project_name=project_name,
            initial_tasks=0,
            migrated_tasks=0,
            final_tasks=0
        )

        # Load or create container
        try:
            container = load_project_task_container(project_name)
            stats.initial_tasks = len(container.get_all_tasks())
        except:
            container = ProjectTaskContainer.new(project_name)
            if self.verbose:
                print(f"📁 Creating new project container for '{project_name}'")

        # Migrate tasks
        for task in tasks:
            if not container.get_task(task.id):
                container.add_task(task)
                stats.migrated_tasks += 1
                if self.verbose:
                    print(f"   ✅ Migrated task: {task.id}")

        stats.final_tasks = len(container.get_all_tasks())

        # Save if not dry run
        if not self.dry_run:
            save_project_task_container(project_name, container)

        return stats

    def print_summary(self, report: MigrationReport):
        """Print migration summary."""
        print("\n" + "=" * 60)
        print("📊 MIGRATION SUMMARY")
        print("=" * 60)
        print(f"Tasks found: {report.tasks_found}")
        print(f"Tasks migrated: {report.tasks_migrated}")
        print(f"Projects migrated: {report.projects_migrated}")
        print("=" * 60)
