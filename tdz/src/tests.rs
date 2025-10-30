#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use crate::storage::Storage;
    use tempfile::TempDir;
    use std::fs;
    async fn create_test_storage() -> (TempDir, Storage) {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join(".todozi");
        fs::create_dir_all(&storage_dir).unwrap();
        fs::create_dir_all(storage_dir.join("tasks")).unwrap();
        fs::create_dir_all(storage_dir.join("projects")).unwrap();
        fs::create_dir_all(storage_dir.join("templates")).unwrap();
        fs::create_dir_all(storage_dir.join("backups")).unwrap();
        let config = Config::default();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(storage_dir.join("config.json"), config_json).unwrap();
        let project = Project::new(
            "general".to_string(),
            Some("General tasks".to_string()),
        );
        let project_json = serde_json::to_string_pretty(&project).unwrap();
        fs::write(storage_dir.join("projects").join("general.json"), project_json)
            .unwrap();
        let collection = TaskCollection::new();
        let collection_json = serde_json::to_string_pretty(&collection).unwrap();
        fs::write(storage_dir.join("tasks").join("active.json"), collection_json.clone())
            .unwrap();
        fs::write(
                storage_dir.join("tasks").join("completed.json"),
                collection_json.clone(),
            )
            .unwrap();
        fs::write(storage_dir.join("tasks").join("archived.json"), collection_json)
            .unwrap();
        let storage = Storage::new().await.unwrap();
        (temp_dir, storage)
    }
    #[test]
    fn test_task_creation() {
        let task = Task::new(
            "user_123".to_string(),
            "Test task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Todo,
        );
        assert_eq!(task.action, "Test task");
        assert_eq!(task.time, "1 hour");
        assert_eq!(task.priority, Priority::Medium);
        assert_eq!(task.parent_project, "test-project");
        assert_eq!(task.status, Status::Todo);
        assert!(task.id.starts_with("task_"));
        assert!(task.assignee.is_none());
        assert!(task.tags.is_empty());
        assert!(task.dependencies.is_empty());
        assert!(task.context_notes.is_none());
        assert!(task.progress.is_none());
    }
    #[test]
    fn test_task_creation_full() {
        let task = Task::new_full(
                "Test task".to_string(),
                "2 hours".to_string(),
                Priority::High,
                "test-project".to_string(),
                Status::InProgress,
                Some(Assignee::Human),
                vec!["test".to_string(), "example".to_string()],
                vec!["task_001".to_string()],
                Some("Test context".to_string()),
                Some(50),
            )
            .unwrap();
        assert_eq!(task.action, "Test task");
        assert_eq!(task.time, "2 hours");
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.parent_project, "test-project");
        assert_eq!(task.status, Status::InProgress);
        assert_eq!(task.assignee, Some(Assignee::Human));
        assert_eq!(task.tags, vec!["test", "example"]);
        assert_eq!(task.dependencies, vec!["task_001"]);
        assert_eq!(task.context_notes, Some("Test context".to_string()));
        assert_eq!(task.progress, Some(50));
    }
    #[test]
    fn test_task_creation_invalid_progress() {
        let result = Task::new_full(
            "Test task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Todo,
            None,
            Vec::new(),
            Vec::new(),
            None,
            Some(150),
        );
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::error::TodoziError::InvalidProgress { progress } => {
                assert_eq!(progress, 150);
            }
            _ => panic!("Expected InvalidProgress error"),
        }
    }
    #[test]
    fn test_task_update() {
        let mut task = Task::new(
            "user_123".to_string(),
            "Original task".to_string(),
            "1 hour".to_string(),
            Priority::Low,
            "test-project".to_string(),
            Status::Todo,
        );
        let updates = TaskUpdate::new()
            .with_action("Updated task".to_string())
            .with_priority(Priority::High)
            .with_status(Status::InProgress)
            .with_progress(75);
        task.update(updates).unwrap();
        assert_eq!(task.action, "Updated task");
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.status, Status::InProgress);
        assert_eq!(task.progress, Some(75));
    }
    #[test]
    fn test_task_complete() {
        let mut task = Task::new(
            "Test task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Todo,
        );
        task.complete();
        assert_eq!(task.status, Status::Done);
        assert_eq!(task.progress, Some(100));
        assert!(task.is_completed());
    }
    #[test]
    fn test_task_is_active() {
        let active_task = Task::new(
            "user_123".to_string(),
            "Active task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Todo,
        );
        let mut completed_task = Task::new(
            "user_123".to_string(),
            "Completed task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Todo,
        );
        completed_task.complete();
        let cancelled_task = Task::new(
            "user_123".to_string(),
            "Cancelled task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Cancelled,
        );
        assert!(active_task.is_active());
        assert!(! completed_task.is_active());
        assert!(! cancelled_task.is_active());
    }
    #[test]
    fn test_priority_parsing() {
        assert_eq!("low".parse::< Priority > ().unwrap(), Priority::Low);
        assert_eq!("medium".parse::< Priority > ().unwrap(), Priority::Medium);
        assert_eq!("high".parse::< Priority > ().unwrap(), Priority::High);
        assert_eq!("critical".parse::< Priority > ().unwrap(), Priority::Critical);
        assert_eq!("urgent".parse::< Priority > ().unwrap(), Priority::Urgent);
        assert!("invalid".parse::< Priority > ().is_err());
    }
    #[test]
    fn test_status_parsing() {
        assert_eq!("todo".parse::< Status > ().unwrap(), Status::Todo);
        assert_eq!("in_progress".parse::< Status > ().unwrap(), Status::InProgress);
        assert_eq!("in-progress".parse::< Status > ().unwrap(), Status::InProgress);
        assert_eq!("blocked".parse::< Status > ().unwrap(), Status::Blocked);
        assert_eq!("review".parse::< Status > ().unwrap(), Status::Review);
        assert_eq!("done".parse::< Status > ().unwrap(), Status::Done);
        assert_eq!("cancelled".parse::< Status > ().unwrap(), Status::Cancelled);
        assert_eq!("canceled".parse::< Status > ().unwrap(), Status::Cancelled);
        assert_eq!("deferred".parse::< Status > ().unwrap(), Status::Deferred);
        assert!("invalid".parse::< Status > ().is_err());
    }
    #[test]
    fn test_assignee_parsing() {
        assert_eq!("ai".parse::< Assignee > ().unwrap(), Assignee::Ai);
        assert_eq!("human".parse::< Assignee > ().unwrap(), Assignee::Human);
        assert_eq!(
            "collaborative".parse::< Assignee > ().unwrap(), Assignee::Collaborative
        );
        assert!("invalid".parse::< Assignee > ().is_err());
    }
    #[test]
    fn test_project_creation() {
        let project = Project::new(
            "test-project".to_string(),
            Some("Test project description".to_string()),
        );
        assert_eq!(project.name, "test-project");
        assert_eq!(project.description, Some("Test project description".to_string()));
        assert_eq!(project.status, ProjectStatus::Active);
        assert!(project.tasks.is_empty());
    }
    #[test]
    fn test_project_add_task() {
        let mut project = Project::new("test-project".to_string(), None);
        project.add_task("task_001".to_string());
        project.add_task("task_002".to_string());
        project.add_task("task_001".to_string());
        assert_eq!(project.tasks.len(), 2);
        assert!(project.tasks.contains(& "task_001".to_string()));
        assert!(project.tasks.contains(& "task_002".to_string()));
    }
    #[test]
    fn test_project_remove_task() {
        let mut project = Project::new("test-project".to_string(), None);
        project.add_task("task_001".to_string());
        project.add_task("task_002".to_string());
        project.remove_task("task_001");
        assert_eq!(project.tasks.len(), 1);
        assert!(! project.tasks.contains(& "task_001".to_string()));
        assert!(project.tasks.contains(& "task_002".to_string()));
    }
    #[test]
    fn test_project_archive() {
        let mut project = Project::new("test-project".to_string(), None);
        project.archive();
        assert_eq!(project.status, ProjectStatus::Archived);
    }
    #[test]
    fn test_project_complete() {
        let mut project = Project::new("test-project".to_string(), None);
        project.complete();
        assert_eq!(project.status, ProjectStatus::Completed);
    }
    #[test]
    fn test_task_collection() {
        let mut collection = TaskCollection::new();
        let task1 = Task::new(
            "user_123".to_string(),
            "Task 1".to_string(),
            "1 hour".to_string(),
            Priority::Low,
            "project1".to_string(),
            Status::Todo,
        );
        let task2 = Task::new(
            "user_123".to_string(),
            "Task 2".to_string(),
            "2 hours".to_string(),
            Priority::High,
            "project2".to_string(),
            Status::InProgress,
        );
        collection.add_task(task1.clone());
        collection.add_task(task2.clone());
        assert_eq!(collection.tasks.len(), 2);
        assert!(collection.get_task(& task1.id).is_some());
        assert!(collection.get_task(& task2.id).is_some());
        assert!(collection.get_task("nonexistent").is_none());
        let all_tasks = collection.get_all_tasks();
        assert_eq!(all_tasks.len(), 2);
        let removed_task = collection.remove_task(&task1.id);
        assert!(removed_task.is_some());
        assert_eq!(collection.tasks.len(), 1);
    }
    #[test]
    fn test_task_collection_filtering() {
        let mut collection = TaskCollection::new();
        let task1 = Task::new(
            "user_123".to_string(),
            "Low priority task".to_string(),
            "1 hour".to_string(),
            Priority::Low,
            "project1".to_string(),
            Status::Todo,
        );
        let task2 = Task::new(
            "user_123".to_string(),
            "High priority task".to_string(),
            "2 hours".to_string(),
            Priority::High,
            "project2".to_string(),
            Status::InProgress,
        );
        collection.add_task(task1);
        collection.add_task(task2);
        let high_priority_filter = TaskFilters {
            priority: Some(Priority::High),
            ..Default::default()
        };
        let high_priority_tasks = collection.get_filtered_tasks(&high_priority_filter);
        assert_eq!(high_priority_tasks.len(), 1);
        assert_eq!(high_priority_tasks[0].priority, Priority::High);
        let project1_filter = TaskFilters {
            project: Some("project1".to_string()),
            ..Default::default()
        };
        let project1_tasks = collection.get_filtered_tasks(&project1_filter);
        assert_eq!(project1_tasks.len(), 1);
        assert_eq!(project1_tasks[0].parent_project, "project1");
        let todo_filter = TaskFilters {
            status: Some(Status::Todo),
            ..Default::default()
        };
        let todo_tasks = collection.get_filtered_tasks(&todo_filter);
        assert_eq!(todo_tasks.len(), 1);
        assert_eq!(todo_tasks[0].status, Status::Todo);
    }
    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, "1.2.0");
        assert_eq!(config.default_project, "general");
        assert!(config.auto_backup);
        assert_eq!(config.backup_interval, "daily");
        assert!(config.ai_enabled);
        assert_eq!(config.default_assignee, Some(Assignee::Collaborative));
        assert_eq!(config.date_format, "%Y-%m-%d %H:%M:%S");
        assert_eq!(config.timezone, "UTC");
    }
    #[test]
    fn test_task_update_validation() {
        let mut task = Task::new(
            "user_123".to_string(),
            "Test task".to_string(),
            "1 hour".to_string(),
            Priority::Medium,
            "test-project".to_string(),
            Status::Todo,
        );
        let invalid_progress_update = TaskUpdate::new().with_progress(150);
        assert!(task.update(invalid_progress_update).is_err());
        let valid_progress_update = TaskUpdate::new().with_progress(75);
        assert!(task.update(valid_progress_update).is_ok());
        assert_eq!(task.progress, Some(75));
    }
    #[test]
    fn test_error_types() {
        let error = crate::error::TodoziError::TaskNotFound {
            id: "test".to_string(),
        };
        assert!(error.to_string().contains("Task not found"));
        let error = crate::error::TodoziError::InvalidPriority {
            priority: "invalid".to_string(),
        };
        assert!(error.to_string().contains("Invalid priority"));
        let error = crate::error::TodoziError::validation("Test validation error");
        assert!(error.to_string().contains("Validation error"));
    }
}