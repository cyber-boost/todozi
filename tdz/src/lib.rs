use chrono;
use std::env;
use uuid;
pub mod agent;
pub mod api;
pub mod base;
pub mod chunking;
pub mod cli;
pub mod emb;
pub mod error;
pub mod extract;
pub mod idea;
pub mod memory;
pub mod migration;
pub mod models;
pub mod reminder;
pub mod search;
pub mod server;
pub mod storage;
pub mod summary;
pub mod tags;
pub mod tdz;
pub mod tdz_tls;
pub mod todozi;
pub mod types;
pub mod toolbox;
#[cfg(feature = "tui")]
pub mod tui;
#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "nodejs")]
pub mod nodejs;
pub use api::*;
pub use base::*;
pub use emb::{
    ClusteringResult, SimilarityResult, TodoziContentType, TodoziEmbeddingConfig,
    TodoziEmbeddingService, TodoziEmbeddingTool,
};
pub use error::{Result, TodoziError};
pub use extract::{extract_content, strategy_content};
pub use models::*;
pub use search::{SearchEngine, SearchOptions};
#[cfg(feature = "tui")]
pub use tui::{
    ColorScheme, DisplayConfig, TaskDisplay, TaskEditor, TaskEvolutionAnalyzer,
    TaskEvolutionSummary, TaskListDisplay, TodoziApp, TuiService,
};
pub use server::{start_server, ServerConfig, TodoziServer};
pub use toolbox::*;
pub use storage::{
    check_folder_structure, clear_registration, delete_project, ensure_folder_structure,
    get_registration_info, get_storage_dir, init_storage, is_registered, list_projects,
    load_config, load_project, load_task_collection, register_with_server, save_config,
    save_project, save_task_collection, update_config_with_registration,
    update_registration_api_key, Storage,
};
pub use tdz::{execute_tdz_command, parse_tdz_command, process_tdz_commands, TdzCommand};
pub use tdz_tls::{
    create_tdz_content_processor_tool, initialize_tdz_content_processor, tdz_cnt,
    SharedTodoziState, TodoziProcessorState,
};
pub use todozi::{
    parse_agent_assignment_format, parse_error_format, parse_idea_format,
    parse_memory_format, parse_todozi_format, process_chat_message,
    process_chat_message_extended, process_json_examples, ChatContent,
};
pub async fn init() -> Result<()> {
    storage::init_storage().await
}
pub async fn init_with_auto_registration() -> Result<()> {
    storage::init_storage().await?;
    let mut hlx = helix::Hlx::new().await?;
    let user_name_uuid = helix::xlh(&hlx, "@uuid", "").await?;
    let _user_name = if let helix::DnaValue::String(uuid) = user_name_uuid {
        let uname = format!("user_{}", & uuid[..8]);
        hlx.set("registration", "user_name", &uname);
        println!("✅ Set user_name: {}", uname);
        uname
    } else {
        "user_default".to_string()
    };
    let email_uuid = helix::xlh(&hlx, "@uuid", "").await?;
    let _user_email = if let helix::DnaValue::String(uuid) = email_uuid {
        let email = format!("hash_{}@example.com", & uuid[..8]);
        hlx.set("registration", "user_email", &email);
        println!("✅ Set user_email: {}", email);
        email
    } else {
        "hash_default@example.com".to_string()
    };
    let now_val = helix::xlh(&hlx, "@now", "").await?;
    let _registered_at = if let helix::DnaValue::String(now_str) = now_val {
        hlx.set("registration", "registered_at", &now_str);
        println!("✅ Set registered_at: {}", now_str);
        now_str
    } else {
        "1970-01-01T00:00:00Z".to_string()
    };
    let server_url = "https://todozi.com";
    hlx.set("registration", "server_url", server_url);
    println!("✅ Set server_url: {}", server_url);
    hlx.set("config", "version", "1.2.0");
    hlx.set("config", "default_project", "general");
    hlx.set("config", "auto_backup", true);
    hlx.set("config", "backup_interval", "daily");
    hlx.set("config", "ai_enabled", true);
    hlx.set("config", "default_assignee", "collaborative");
    hlx.set("config", "date_format", "%Y-%m-%d %H:%M:%S");
    hlx.set("config", "timezone", "UTC");
    let storage_dir = storage::get_storage_dir()?;
    let config_path = storage_dir.join("tdz.hlx");
    hlx.file_path = Some(config_path.clone());
    hlx.save()?;
    println!("✅ Saved user config to tdz.hlx");
    println!("🔗 Attempting to register with todozi.com server...");
    match storage::register_with_server("https://todozi.com").await {
        Ok(registration) => {
            let mut hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
            hlx.set(
                "registration",
                "api_key",
                helix::DnaValue::String(registration.api_key.clone()),
            );
            hlx.set(
                "registration",
                "user_id",
                helix::DnaValue::String(registration.user_id.clone().unwrap_or_default()),
            );
            hlx.set(
                "registration",
                "fingerprint",
                helix::DnaValue::String(
                    registration.fingerprint.clone().unwrap_or_default(),
                ),
            );
            hlx.file_path = Some(config_path);
            hlx.save()?;
            println!("✅ Successfully registered with todozi.com!");
            println!("🔑 API Key: {}", registration.api_key);
            if let Some(uid) = &registration.user_id {
                println!("👤 User ID: {}", uid);
            }
            if let Some(fp) = &registration.fingerprint {
                println!("🔐 Fingerprint: {}", fp);
            }
        }
        Err(e) => {
            println!("⚠️  Auto-registration failed: {}", e);
            println!("💡 Run 'todozi register' to complete registration manually");
            println!("📝 Local configuration saved with placeholder values");
        }
    }
    Ok(())
}
pub fn tdzfp() -> Result<bool> {
    storage::check_folder_structure()
}
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub async fn todozi_begin() -> Result<()> {
    let home_dir = std::env::var("HOME")
        .expect("Could not get HOME environment variable");
    let config_path = std::path::PathBuf::from(home_dir).join(".todozi").join("tdz.hlx");
    if !config_path.exists() {
        init_with_auto_registration().await?;
    } else {
        let hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
        let api_key = hlx
            .get("registration", "api_key")
            .and_then(|v| {
                if let helix::DnaValue::String(s) = v { Some(s.clone()) } else { None }
            })
            .unwrap_or_default();
        if api_key.is_empty() {
            init_with_auto_registration().await?;
        }
    }
    Ok(())
}
pub async fn get_tdz_api_key() -> Result<String> {
    let home_dir = std::env::var("HOME")
        .expect("Could not get HOME environment variable");
    let config_path = std::path::PathBuf::from(home_dir).join(".todozi").join("tdz.hlx");
    let hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
    let api_key = hlx
        .get("registration", "api_key")
        .and_then(|v| {
            if let helix::DnaValue::String(s) = v { Some(s.clone()) } else { None }
        })
        .unwrap_or_default();
    Ok(api_key)
}
pub async fn ensure_todozi_initialized() -> Result<()> {
    let home = env::var("HOME")
        .map_err(|_| TodoziError::config("Could not get HOME environment variable"))?;
    let todozi_dir = std::path::PathBuf::from(home).join(".todozi");
    if !todozi_dir.exists() {
        todozi_begin().await?;
    }
    Ok(())
}
pub async fn find_tdz(str: Option<&str>) -> Option<String> {
    let home = env::var("HOME").ok()?;
    let todozi_home = format!("{}/.todozi", home);
    if str.is_some() { Some(format!("{}/{}", todozi_home, str.unwrap())) } else { Some(todozi_home) }
}
pub struct Done;
use std::cell::RefCell;
thread_local! {
    static PROJECT_NAME : RefCell < String > = RefCell::new("external_apps".to_string());
}
impl Done {
    pub fn set_project<S: Into<String>>(project_name: S) {
        PROJECT_NAME.with(|p| *p.borrow_mut() = project_name.into());
    }
    pub fn project_name() -> String {
        PROJECT_NAME.with(|p| p.borrow().clone())
    }
    pub async fn init() -> Result<()> {
        ensure_todozi_initialized().await
    }
    pub async fn api_key() -> Result<String> {
        get_tdz_api_key().await
    }
    pub async fn create_task(
        action: &str,
        priority: Option<Priority>,
        project: Option<&str>,
        time: Option<&str>,
        context: Option<&str>,
    ) -> Result<Task> {
        Self::init().await?;
        let storage = Storage::new().await?;
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "external_app".to_string(),
            action: action.to_string(),
            time: time.unwrap_or("ASAP").to_string(),
            priority: priority.unwrap_or(Priority::Medium),
            parent_project: project
                .map(|s| s.to_string())
                .unwrap_or_else(|| Self::project_name()),
            status: Status::Todo,
            assignee: Some(Assignee::Human),
            tags: vec!["external".to_string()],
            dependencies: Vec::new(),
            context_notes: context.map(|s| s.to_string()),
            progress: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        let embedded_task = if let Ok(mut emb_service) = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await
        {
            if emb_service.initialize().await.is_ok() {
                let mut task_with_embedding = task.clone();
                task_with_embedding.embedding_vector = Some(
                    emb_service.generate_embedding(action).await?,
                );
                task_with_embedding
            } else {
                task
            }
        } else {
            task
        };
        storage.add_task_to_project(embedded_task.clone()).await?;
        Ok(embedded_task)
    }
    pub async fn search_tasks(
        query: &str,
        semantic: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Task>> {
        Self::init().await?;
        let storage = Storage::new().await?;
        if semantic {
            if let Ok(mut emb_service) = TodoziEmbeddingService::new(
                    TodoziEmbeddingConfig::default(),
                )
                .await
            {
                if emb_service.initialize().await.is_ok() {
                    let similar = emb_service.find_similar_tasks(query, limit).await?;
                    let task_ids: Vec<String> = similar
                        .iter()
                        .filter_map(|s| {
                            s.text_content.split(' ').next().map(|s| s.to_string())
                        })
                        .collect();
                    let all_tasks = storage
                        .list_tasks_across_projects(&TaskFilters::default())?;
                    return Ok(
                        all_tasks
                            .into_iter()
                            .filter(|t| task_ids.contains(&t.id))
                            .take(limit.unwrap_or(10))
                            .collect(),
                    );
                }
            }
        }
        let filters = TaskFilters {
            search: Some(query.to_string()),
            ..Default::default()
        };
        let tasks = storage.list_tasks_across_projects(&filters)?;
        Ok(tasks.into_iter().take(limit.unwrap_or(10)).collect())
    }
    pub async fn update_task_status(task_id: &str, status: Status) -> Result<()> {
        Self::init().await?;
        let storage = Storage::new().await?;
        let updates = TaskUpdate {
            status: Some(status),
            ..Default::default()
        };
        storage.update_task_in_project(task_id, updates).await?;
        Ok(())
    }
    pub async fn extract_tasks(
        content: &str,
        context: Option<&str>,
    ) -> Result<Vec<String>> {
        Self::init().await?;
        let api_key = Self::api_key().await?;
        let client = reqwest::Client::new();
        let url = "https://todozi.com/api/todozi/extract";
        let payload = serde_json::json!(
            { "message" : content, "context" : context.unwrap_or("") }
        );
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(
                TodoziError::api(format!("API request failed: {}", response.status())),
            );
        }
        let result: serde_json::Value = response.json().await?;
        if let Some(extracted) = result.get("extracted_content") {
            if let Some(tasks) = extracted.get("tasks").and_then(|t| t.as_array()) {
                let task_actions: Vec<String> = tasks
                    .iter()
                    .filter_map(|t| {
                        t.get("action").and_then(|a| a.as_str()).map(|s| s.to_string())
                    })
                    .collect();
                return Ok(task_actions);
            }
        }
        Ok(vec![])
    }
    pub async fn plan_tasks(
        goal: &str,
        complexity: Option<&str>,
        timeline: Option<&str>,
        context: Option<&str>,
    ) -> Result<Vec<Task>> {
        Self::init().await?;
        let api_key = Self::api_key().await?;
        let client = reqwest::Client::new();
        let url = "https://todozi.com/api/todozi/plan";
        let payload = serde_json::json!(
            { "goal" : goal, "complexity" : complexity.unwrap_or("medium"), "timeline" :
            timeline.unwrap_or("ASAP"), "context" : context.unwrap_or("") }
        );
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(
                TodoziError::api(format!("API request failed: {}", response.status())),
            );
        }
        let result: serde_json::Value = response.json().await?;
        let mut tasks = Vec::new();
        if let Some(planning_result) = result.get("planning_result") {
            if let Some(api_tasks) = planning_result
                .get("tasks")
                .and_then(|t| t.as_array())
            {
                let storage = Storage::new().await?;
                for api_task in api_tasks {
                    if let Some(action) = api_task.get("action").and_then(|a| a.as_str())
                    {
                        let task = Task {
                            id: uuid::Uuid::new_v4().to_string(),
                            user_id: "ai_planner".to_string(),
                            action: action.to_string(),
                            time: api_task
                                .get("time")
                                .and_then(|t| t.as_str())
                                .unwrap_or("ASAP")
                                .to_string(),
                            priority: api_task
                                .get("priority")
                                .and_then(|p| p.as_str())
                                .and_then(|p| p.parse().ok())
                                .unwrap_or(Priority::Medium),
                            parent_project: format!("{}_plans", Self::project_name()),
                            status: Status::Todo,
                            assignee: Some(Assignee::Ai),
                            tags: vec![
                                "planned".to_string(), "ai_generated".to_string()
                            ],
                            dependencies: Vec::new(),
                            context_notes: Some(format!("AI planned for: {}", goal)),
                            progress: Some(0),
                            created_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                            embedding_vector: None,
                        };
                        storage.add_task_to_project(task.clone()).await?;
                        tasks.push(task);
                    }
                }
            }
        }
        Ok(tasks)
    }
    pub async fn list_tasks() -> Result<Vec<Task>> {
        Self::init().await?;
        let storage = Storage::new().await?;
        storage.list_tasks_across_projects(&TaskFilters::default())
    }
    pub async fn get_task(task_id: &str) -> Result<Option<Task>> {
        Self::init().await?;
        let storage = Storage::new().await?;
        let tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;
        Ok(tasks.into_iter().find(|t| t.id == task_id))
    }
    pub async fn delete_task(task_id: &str) -> Result<()> {
        Self::init().await?;
        let storage = Storage::new().await?;
        storage.delete_task_from_project(task_id)?;
        Ok(())
    }
    pub async fn create_memory(
        moment: &str,
        meaning: &str,
        reason: &str,
    ) -> Result<Task> {
        Self::init().await?;
        let storage = Storage::new().await?;
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "memory_creator".to_string(),
            action: format!("Memory: {} - {}", moment, meaning),
            time: "Long-term".to_string(),
            priority: Priority::Low,
            parent_project: format!("{}_memories", Self::project_name()),
            status: Status::Done,
            assignee: Some(Assignee::Human),
            tags: vec!["memory".to_string()],
            dependencies: Vec::new(),
            context_notes: Some(format!("Reason: {}", reason)),
            progress: Some(100),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        storage.add_task_to_project(task.clone()).await?;
        Ok(task)
    }
    pub async fn create_idea(idea: &str, context: Option<&str>) -> Result<Task> {
        Self::init().await?;
        let storage = Storage::new().await?;
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "idea_creator".to_string(),
            action: format!("Idea: {}", idea),
            time: "Future consideration".to_string(),
            priority: Priority::Low,
            parent_project: format!("{}_ideas", Self::project_name()),
            status: Status::Todo,
            assignee: Some(Assignee::Human),
            tags: vec!["idea".to_string()],
            dependencies: Vec::new(),
            context_notes: context.map(|s| s.to_string()),
            progress: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        storage.add_task_to_project(task.clone()).await?;
        Ok(task)
    }
    pub async fn process_chat(message: &str, user_id: &str) -> Result<ChatContent> {
        Self::init().await?;
        process_chat_message_extended(message, user_id)
    }
    pub async fn storage() -> Result<Storage> {
        Self::init().await?;
        Storage::new().await
    }
    pub async fn embedding_service() -> Result<TodoziEmbeddingService> {
        TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await
    }
    pub async fn search_with_filters(
        filters: TaskFilters,
        limit: Option<usize>,
    ) -> Result<Vec<Task>> {
        Self::init().await?;
        let storage = Storage::new().await?;
        let mut tasks = storage.list_tasks_across_projects(&filters)?;
        if let Some(limit) = limit {
            tasks.truncate(limit);
        }
        Ok(tasks)
    }
    pub async fn update_task_full(task_id: &str, updates: TaskUpdate) -> Result<()> {
        Self::init().await?;
        let storage = Storage::new().await?;
        storage.update_task_in_project(task_id, updates).await?;
        Ok(())
    }
    pub fn types() -> &'static str {
        "Available types: Task, Priority, Status, Assignee, TaskFilters, TaskUpdate, ChatContent, TodoziEmbeddingService, TodoziEmbeddingConfig"
    }
    /// Get a sample task for reference (for external usage)
    pub fn sample_task() -> Task {
        Task {
            id: "sample_id".to_string(),
            user_id: "sample_user".to_string(),
            action: "Sample task action".to_string(),
            time: "ASAP".to_string(),
            priority: Priority::Medium,
            parent_project: format!("{}_samples", Self::project_name()),
            status: Status::Todo,
            assignee: Some(Assignee::Human),
            tags: vec!["sample".to_string()],
            dependencies: Vec::new(),
            context_notes: Some("Sample context".to_string()),
            progress: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        }
    }
    /// Get default task filters (for external usage)
    pub fn default_filters() -> TaskFilters {
        TaskFilters::default()
    }
    /// Get default task update (for external usage)
    pub fn default_update() -> TaskUpdate {
        TaskUpdate::default()
    }
    /// Create a new embedding config (for external usage)
    pub fn embedding_config() -> TodoziEmbeddingConfig {
        TodoziEmbeddingConfig::default()
    }
    /// Create a storage instance (convenience wrapper)
    pub async fn create_storage() -> Result<Storage> {
        Self::init().await?;
        Storage::new().await
    }
    /// Create an embedding service instance (convenience wrapper)
    pub async fn create_embedding_service() -> Result<TodoziEmbeddingService> {
        TodoziEmbeddingService::new(Self::embedding_config()).await
    }
    /// Create task filters (convenience wrapper)
    pub fn create_filters() -> TaskFilters {
        TaskFilters::default()
    }
    /// Create task update (convenience wrapper)
    pub fn create_update() -> TaskUpdate {
        TaskUpdate::default()
    }
    /// Execute task extraction and return task actions (API wrapper)
    pub async fn extract_task_actions(content: &str) -> Result<Vec<String>> {
        Self::extract_tasks(content, None).await
    }
    /// Execute intelligent planning and return task actions (API wrapper)
    pub async fn plan_task_actions(goal: &str) -> Result<Vec<String>> {
        let tasks = Self::plan_tasks(goal, None, None, None).await?;
        Ok(tasks.into_iter().map(|t| t.action).collect())
    }
    /// Simple task creation with minimal parameters
    pub async fn quick_task(action: &str) -> Result<Task> {
        Self::create_task(action, None, None, None, None).await
    }
    /// Simple task search
    pub async fn find_tasks(query: &str) -> Result<Vec<Task>> {
        Self::search_tasks(query, false, None).await
    }
    /// Simple semantic search
    pub async fn find_tasks_ai(query: &str) -> Result<Vec<Task>> {
        Self::search_tasks(query, true, None).await
    }
    /// Get all tasks (convenience wrapper)
    pub async fn all_tasks() -> Result<Vec<Task>> {
        Self::list_tasks().await
    }
    /// Complete a task (convenience wrapper)
    pub async fn complete_task(task_id: &str) -> Result<()> {
        Self::update_task_status(task_id, Status::Done).await
    }
    /// Start working on a task (convenience wrapper)
    pub async fn start_task(task_id: &str) -> Result<()> {
        Self::update_task_status(task_id, Status::InProgress).await
    }
    /// Simple chat processing
    pub async fn chat(message: &str) -> Result<ChatContent> {
        Self::process_chat(message, "external_user").await
    }
    /// Create a memory (convenience wrapper)
    pub async fn remember(moment: &str, meaning: &str) -> Result<Task> {
        Self::create_memory(moment, meaning, "Created via external API").await
    }
    /// Create an idea (convenience wrapper)
    pub async fn ideate(idea: &str) -> Result<Task> {
        Self::create_idea(idea, None).await
    }
}
/// Simple Todozi interface - the easiest way to use Todozi
pub struct Tdz;
impl Tdz {
    /// Quick task creation - just pass a string
    pub async fn task(action: &str) -> Result<String> {
        let task = Done::create_task(action, None, None, None, None).await?;
        Ok(task.id)
    }
    /// Quick task with priority
    pub async fn urgent(action: &str) -> Result<String> {
        let task = Done::create_task(action, Some(Priority::Urgent), None, None, None)
            .await?;
        Ok(task.id)
    }
    /// Quick high priority task
    pub async fn high(action: &str) -> Result<String> {
        let task = Done::create_task(action, Some(Priority::High), None, None, None)
            .await?;
        Ok(task.id)
    }
    /// Quick low priority task
    pub async fn low(action: &str) -> Result<String> {
        let task = Done::create_task(action, Some(Priority::Low), None, None, None)
            .await?;
        Ok(task.id)
    }
    /// Find tasks by text
    pub async fn find(query: &str) -> Result<Vec<Task>> {
        Done::find_tasks(query).await
    }
    /// Find tasks with AI
    pub async fn ai_find(query: &str) -> Result<Vec<Task>> {
        Done::find_tasks_ai(query).await
    }
    /// Complete a task
    pub async fn done(task_id: &str) -> Result<()> {
        Done::complete_task(task_id).await
    }
    /// Start working on a task
    pub async fn start(task_id: &str) -> Result<()> {
        Done::start_task(task_id).await
    }
    /// Get all tasks
    pub async fn all() -> Result<Vec<Task>> {
        Done::all_tasks().await
    }
    /// Remember something
    pub async fn remember(moment: &str, meaning: &str) -> Result<Task> {
        Done::remember(moment, meaning).await
    }
    /// Save an idea
    pub async fn idea(idea: &str) -> Result<Task> {
        Done::ideate(idea).await
    }
    /// Process a chat message
    pub async fn chat(message: &str) -> Result<ChatContent> {
        Done::chat(message).await
    }
}
/// Simple Actions interface for task operations
pub struct Actions;
impl Actions {
    /// Create a task that needs AI processing
    pub async fn ai(action: &str) -> Result<String> {
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "actions_user".to_string(),
            action: action.to_string(),
            time: "ASAP".to_string(),
            priority: Priority::Medium,
            parent_project: format!("{}_ai", Done::project_name()),
            status: Status::Todo,
            assignee: Some(Assignee::Ai),
            tags: vec!["ai".to_string()],
            dependencies: Vec::new(),
            context_notes: Some("Created via Actions::ai".to_string()),
            progress: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        let storage = Storage::new().await?;
        storage.add_task_to_project(task.clone()).await?;
        Ok(task.id)
    }
    /// Create a task for humans
    pub async fn human(action: &str) -> Result<String> {
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "actions_user".to_string(),
            action: action.to_string(),
            time: "ASAP".to_string(),
            priority: Priority::Medium,
            parent_project: format!("{}_human", Done::project_name()),
            status: Status::Todo,
            assignee: Some(Assignee::Human),
            tags: vec!["human".to_string()],
            dependencies: Vec::new(),
            context_notes: Some("Created via Actions::human".to_string()),
            progress: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        let storage = Storage::new().await?;
        storage.add_task_to_project(task.clone()).await?;
        Ok(task.id)
    }
    /// Create a collaborative task
    pub async fn collab(action: &str) -> Result<String> {
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "actions_user".to_string(),
            action: action.to_string(),
            time: "ASAP".to_string(),
            priority: Priority::Medium,
            parent_project: format!("{}_collaborative", Done::project_name()),
            status: Status::Todo,
            assignee: Some(Assignee::Collaborative),
            tags: vec!["collaborative".to_string()],
            dependencies: Vec::new(),
            context_notes: Some("Created via Actions::collab".to_string()),
            progress: Some(0),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        let storage = Storage::new().await?;
        storage.add_task_to_project(task.clone()).await?;
        Ok(task.id)
    }
    /// Mark task as done
    pub async fn complete(task_id: &str) -> Result<()> {
        Done::complete_task(task_id).await
    }
    /// Start working on task
    pub async fn begin(task_id: &str) -> Result<()> {
        Done::start_task(task_id).await
    }
    /// Delete a task
    pub async fn delete(task_id: &str) -> Result<()> {
        Done::delete_task(task_id).await
    }
    /// Get task details
    pub async fn get(task_id: &str) -> Result<Option<Task>> {
        Done::get_task(task_id).await
    }
    /// List all tasks
    pub async fn list() -> Result<Vec<Task>> {
        Done::all_tasks().await
    }
    /// Add a recent action to the processor state
    pub async fn add_recent(description: &str) -> Result<()> {
        use crate::tdz_tls::{TodoziProcessorState, ProcessedAction};
        let mut state = TodoziProcessorState::new()?;
        let action = ProcessedAction {
            id: uuid::Uuid::new_v4().to_string(),
            action_type: "api".to_string(),
            description: description.to_string(),
            timestamp: chrono::Utc::now(),
            success: true,
            result: None,
        };
        state.add_recent_action(action);
        Ok(())
    }
}
/// Simple Tags interface for tag operations
pub struct Tags;
impl Tags {
    /// Create a new tag
    pub async fn create(name: &str, description: Option<&str>) -> Result<String> {
        Done::init().await?;
        let tag = Tag {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            color: None,
            category: None,
            usage_count: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        Ok(tag.id)
    }
    pub async fn find(tag_name: &str) -> Result<Vec<Task>> {
        let filters = TaskFilters {
            tags: Some(vec![tag_name.to_string()]),
            ..Default::default()
        };
        Done::search_with_filters(filters, None).await
    }
    pub async fn add_to_task(task_id: &str, tag: &str) -> Result<()> {
        if let Some(mut task) = Done::get_task(task_id).await? {
            if !task.tags.contains(&tag.to_string()) {
                task.tags.push(tag.to_string());
                let updates = TaskUpdate {
                    tags: Some(task.tags),
                    ..Default::default()
                };
                Done::update_task_full(task_id, updates).await?;
            }
        }
        Ok(())
    }
    pub async fn remove_from_task(task_id: &str, tag: &str) -> Result<()> {
        if let Some(mut task) = Done::get_task(task_id).await? {
            task.tags.retain(|t| t != tag);
            let updates = TaskUpdate {
                tags: Some(task.tags),
                ..Default::default()
            };
            Done::update_task_full(task_id, updates).await?;
        }
        Ok(())
    }
    pub async fn advanced_search(query: &str) -> Result<Vec<Tag>> {
        use crate::tags::{TagManager, TagSearchEngine, TagSearchQuery, TagSortBy};
        let tag_manager = TagManager::new();
        let search_engine = TagSearchEngine::new(tag_manager);
        let search_query = TagSearchQuery {
            name_contains: Some(query.to_string()),
            description_contains: None,
            category: None,
            color: None,
            min_usage: None,
            max_usage: None,
            sort_by: TagSortBy::Name,
            limit: Some(50),
        };
        Ok(search_engine.advanced_search(search_query).into_iter().cloned().collect())
    }
}
pub struct Projects;
impl Projects {
    pub async fn create(name: &str, description: Option<&str>) -> Result<()> {
        Done::init().await?;
        let storage = Storage::new().await?;
        storage.create_project(name.to_string(), description.map(|s| s.to_string()))?;
        Ok(())
    }
    pub async fn list() -> Result<Vec<String>> {
        Done::init().await?;
        let storage = Storage::new().await?;
        let projects = storage.list_projects()?;
        Ok(projects.into_iter().map(|p| p.name).collect())
    }
    pub async fn tasks(project_name: &str) -> Result<Vec<Task>> {
        let filters = TaskFilters {
            project: Some(project_name.to_string()),
            ..Default::default()
        };
        Done::search_with_filters(filters, None).await
    }
    pub async fn delete(project_name: &str) -> Result<()> {
        Done::init().await?;
        let storage = Storage::new().await?;
        storage.delete_project(project_name)?;
        Ok(())
    }
}
pub struct Memories;
impl Memories {
    pub async fn create(moment: &str, meaning: &str, reason: &str) -> Result<String> {
        Done::init().await?;
        let memory = Memory {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "memories_user".to_string(),
            project_id: None,
            status: ItemStatus::Active,
            moment: moment.to_string(),
            meaning: meaning.to_string(),
            reason: reason.to_string(),
            importance: MemoryImportance::Medium,
            term: MemoryTerm::Short,
            memory_type: MemoryType::Standard,
            tags: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        crate::storage::save_memory(&memory)?;
        Ok(memory.id)
    }
    pub async fn important(moment: &str, meaning: &str, reason: &str) -> Result<String> {
        Done::init().await?;
        let memory = Memory {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "memories_user".to_string(),
            project_id: None,
            status: ItemStatus::Active,
            moment: moment.to_string(),
            meaning: meaning.to_string(),
            reason: reason.to_string(),
            importance: MemoryImportance::High,
            term: MemoryTerm::Long,
            memory_type: MemoryType::Standard,
            tags: vec!["important".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        crate::storage::save_memory(&memory)?;
        Ok(memory.id)
    }
    pub async fn list() -> Result<Vec<Memory>> {
        Done::init().await?;
        crate::storage::list_memories()
    }
    pub async fn find(query: &str) -> Result<Vec<Memory>> {
        let memories = Self::list().await?;
        Ok(
            memories
                .into_iter()
                .filter(|m| {
                    m.moment.to_lowercase().contains(&query.to_lowercase())
                        || m.meaning.to_lowercase().contains(&query.to_lowercase())
                })
                .collect(),
        )
    }
}
pub struct Ideas;
impl Ideas {
    pub async fn create(idea: &str) -> Result<String> {
        Done::init().await?;
        let idea_obj = Idea {
            id: uuid::Uuid::new_v4().to_string(),
            idea: idea.to_string(),
            project_id: None,
            status: ItemStatus::Active,
            share: ShareLevel::Private,
            importance: IdeaImportance::Medium,
            tags: vec![],
            context: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        crate::storage::save_idea(&idea_obj)?;
        Ok(idea_obj.id)
    }
    pub async fn breakthrough(idea: &str) -> Result<String> {
        Done::init().await?;
        let idea_obj = Idea {
            id: uuid::Uuid::new_v4().to_string(),
            idea: idea.to_string(),
            project_id: None,
            status: ItemStatus::Active,
            share: ShareLevel::Team,
            importance: IdeaImportance::Breakthrough,
            tags: vec!["breakthrough".to_string()],
            context: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        crate::storage::save_idea(&idea_obj)?;
        Ok(idea_obj.id)
    }
    pub async fn list() -> Result<Vec<Idea>> {
        Done::init().await?;
        crate::storage::list_ideas()
    }
    pub async fn find(query: &str) -> Result<Vec<Idea>> {
        let ideas = Self::list().await?;
        Ok(
            ideas
                .into_iter()
                .filter(|i| i.idea.to_lowercase().contains(&query.to_lowercase()))
                .collect(),
        )
    }
}
pub struct Queue;
impl Queue {
    pub async fn add(task_name: &str, description: &str) -> Result<String> {
        Done::init().await?;
        let item = QueueItem::new(
            task_name.to_string(),
            description.to_string(),
            Priority::Medium,
            None,
        );
        crate::storage::add_queue_item(item.clone())?;
        Ok(item.id)
    }
    pub async fn list() -> Result<Vec<QueueItem>> {
        Done::init().await?;
        crate::storage::list_queue_items()
    }
    pub async fn backlog() -> Result<Vec<QueueItem>> {
        Done::init().await?;
        crate::storage::list_backlog_items()
    }
    pub async fn active() -> Result<Vec<QueueItem>> {
        Done::init().await?;
        crate::storage::list_active_items()
    }
    pub async fn start(item_id: &str) -> Result<String> {
        Done::init().await?;
        crate::storage::start_queue_session(item_id)
    }
    pub async fn complete(session_id: &str) -> Result<()> {
        Done::init().await?;
        crate::storage::end_queue_session(session_id)
    }
}
pub struct Find;
impl Find {
    pub async fn tdz_find(query: &str) -> Result<String> {
        Done::init().await?;
        let mut results = Vec::new();
        if let Ok(ai_results) = Self::ai_search(query).await {
            if !ai_results.is_empty() {
                results.push("🤖 AI SEMANTIC SEARCH:".to_string());
                for result in ai_results.iter().take(5) {
                    results
                        .push(
                            format!(
                                "  • {} [Task] (similarity: {:.2})", result.text_content,
                                result.similarity_score
                            ),
                        );
                }
                if ai_results.len() > 5 {
                    results
                        .push(
                            format!("  ... and {} more AI matches", ai_results.len() - 5),
                        );
                }
                results.push("".to_string());
            }
        }
        let keyword_results = Self::keyword_search(query).await?;
        if !keyword_results.is_empty() {
            results.push("🔍 KEYWORD SEARCH:".to_string());
            results.push(keyword_results);
        }
        if results.is_empty() {
            Ok(format!("🔍 No results found for: '{}'", query))
        } else {
            Ok(results.join("\n"))
        }
    }
    pub async fn ai_search(query: &str) -> Result<Vec<SimilarityResult>> {
        Done::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await?;
        emb_service.initialize().await?;
        let results = emb_service.semantic_search(query, None, Some(20)).await?;
        Ok(results)
    }
    pub async fn keyword_search(query: &str) -> Result<String> {
        Done::init().await?;
        let mut results = Vec::new();
        if let Ok(tasks) = Done::find_tasks(query).await {
            if !tasks.is_empty() {
                results.push(format!("📋 TASKS ({}):", tasks.len()));
                for task in tasks.iter().take(5) {
                    results
                        .push(
                            format!(
                                "  • {} [{}] [{}]", task.action, task.status, task
                                .priority
                            ),
                        );
                }
                if tasks.len() > 5 {
                    results.push(format!("  ... and {} more", tasks.len() - 5));
                }
                results.push("".to_string());
            }
        }
        if let Ok(memories) = Memories::find(query).await {
            if !memories.is_empty() {
                results.push(format!("🧠 MEMORIES ({}):", memories.len()));
                for memory in memories.iter().take(3) {
                    results
                        .push(format!("  • {} - {}", memory.moment, memory.meaning));
                }
                if memories.len() > 3 {
                    results.push(format!("  ... and {} more", memories.len() - 3));
                }
                results.push("".to_string());
            }
        }
        if let Ok(ideas) = Ideas::find(query).await {
            if !ideas.is_empty() {
                results.push(format!("💡 IDEAS ({}):", ideas.len()));
                for idea in ideas.iter().take(3) {
                    results.push(format!("  • {}", idea.idea));
                }
                if ideas.len() > 3 {
                    results.push(format!("  ... and {} more", ideas.len() - 3));
                }
                results.push("".to_string());
            }
        }
        if let Ok(queue_items) = Queue::list().await {
            let filtered: Vec<_> = queue_items
                .into_iter()
                .filter(|q| {
                    q.task_name.to_lowercase().contains(&query.to_lowercase())
                        || q
                            .task_description
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                })
                .collect();
            if !filtered.is_empty() {
                results.push(format!("📋 QUEUE ({}):", filtered.len()));
                for item in filtered.iter().take(3) {
                    results.push(format!("  • {} [{}]", item.task_name, item.status));
                }
                if filtered.len() > 3 {
                    results.push(format!("  ... and {} more", filtered.len() - 3));
                }
            }
        }
        if results.is_empty() {
            Ok(format!("No keyword results found for: '{}'", query))
        } else {
            Ok(results.join("\n"))
        }
    }
    pub async fn ai_tasks(query: &str) -> Result<Vec<SimilarityResult>> {
        Done::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await?;
        emb_service.initialize().await?;
        emb_service
            .semantic_search(query, Some(vec![TodoziContentType::Task]), Some(10))
            .await
    }
    pub async fn keyword_tasks(query: &str) -> Result<Vec<Task>> {
        Done::find_tasks(query).await
    }
    pub async fn similar_tasks(task_id: &str) -> Result<Vec<SimilarityResult>> {
        Done::init().await?;
        let task = Done::get_task(task_id).await?;
        if let Some(task) = task {
            let mut emb_service = TodoziEmbeddingService::new(
                    TodoziEmbeddingConfig::default(),
                )
                .await?;
            emb_service.initialize().await?;
            emb_service.find_similar_tasks(&task.action, Some(10)).await
        } else {
            Ok(Vec::new())
        }
    }
    pub async fn smart(query: &str) -> Result<String> {
        Done::init().await?;
        let query_lower = query.to_lowercase();
        if query_lower.contains("task") || query_lower.contains("todo")
            || query_lower.contains("do")
        {
            let tasks = Self::ai_tasks(query).await?;
            if !tasks.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - TASKS FOCUS:\n\n");
                for task in tasks.iter().take(7) {
                    result
                        .push_str(
                            &format!(
                                "  • {} (similarity: {:.2})\n", task.text_content, task
                                .similarity_score
                            ),
                        );
                }
                return Ok(result);
            }
        }
        if query_lower.contains("remember") || query_lower.contains("memory")
            || query_lower.contains("recall")
        {
            let memories = Memories::find(query).await?;
            if !memories.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - MEMORIES FOCUS:\n\n");
                for memory in memories.iter().take(5) {
                    result
                        .push_str(
                            &format!("  • {} - {}\n", memory.moment, memory.meaning),
                        );
                }
                return Ok(result);
            }
        }
        if query_lower.contains("idea") || query_lower.contains("concept")
            || query_lower.contains("innovation")
        {
            let ideas = Ideas::find(query).await?;
            if !ideas.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - IDEAS FOCUS:\n\n");
                for idea in ideas.iter().take(5) {
                    result.push_str(&format!("  • {}\n", idea.idea));
                }
                return Ok(result);
            }
        }
        Self::tdz_find(query).await
    }
    pub async fn fast(query: &str) -> Result<String> {
        Self::keyword_search(query).await
    }
    pub async fn deep(query: &str) -> Result<Vec<SimilarityResult>> {
        Self::ai_search(query).await
    }
}
pub struct Emb;
impl Emb {
    pub async fn embed(text: &str) -> Result<Vec<f32>> {
        Done::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await?;
        emb_service.initialize().await?;
        emb_service.generate_embedding(text).await
    }
    pub async fn similar(query: &str) -> Result<Vec<SimilarityResult>> {
        Find::ai_search(query).await
    }
    pub async fn similar_tasks(query: &str) -> Result<Vec<SimilarityResult>> {
        Find::ai_tasks(query).await
    }
    pub async fn cluster() -> Result<Vec<ClusteringResult>> {
        Done::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await?;
        emb_service.initialize().await?;
        emb_service.cluster_content().await
    }
    pub async fn stats() -> Result<String> {
        Done::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await?;
        emb_service.initialize().await?;
        let stats = emb_service.get_stats().await?;
        Ok(format!("🧠 EMBEDDING STATS:\n{}", serde_json::to_string_pretty(& stats) ?))
    }
    pub async fn embed_task(task_id: &str) -> Result<String> {
        Done::init().await?;
        if let Some(task) = Done::get_task(task_id).await? {
            let mut emb_service = TodoziEmbeddingService::new(
                    TodoziEmbeddingConfig::default(),
                )
                .await?;
            emb_service.initialize().await?;
            let content = emb_service.prepare_task_content(&task);
            let embedding = emb_service.generate_embedding(&content).await?;
            Ok(
                format!(
                    "Task '{}' embedded successfully ({} dimensions)", task.action,
                    embedding.len()
                ),
            )
        } else {
            Err(TodoziError::TaskNotFound {
                id: task_id.to_string(),
            })
        }
    }
}
pub struct Stats;
impl Stats {
    pub async fn quick() -> Result<String> {
        Done::init().await?;
        let tasks = Done::all_tasks().await?;
        let total = tasks.len();
        let done = tasks.iter().filter(|t| matches!(t.status, Status::Done)).count();
        let in_progress = tasks
            .iter()
            .filter(|t| matches!(t.status, Status::InProgress))
            .count();
        let blocked = tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Blocked))
            .count();
        let memories = Memories::list().await?.len();
        let ideas = Ideas::list().await?.len();
        let queue_items = Queue::list().await?.len();
        Ok(
            format!(
                "📊 TODOZI STATS\n\n📋 Tasks: {} total\n  ✅ Done: {}\n  🔄 In Progress: {}\n  🚫 Blocked: {}\n\n💡 Ideas: {}\n🧠 Memories: {}\n📋 Queue: {}",
                total, done, in_progress, blocked, ideas, memories, queue_items
            ),
        )
    }
    pub async fn detailed() -> Result<String> {
        let quick_stats = Self::quick().await?;
        let tasks = Done::all_tasks().await?;
        let by_priority = {
            let critical = tasks
                .iter()
                .filter(|t| matches!(t.priority, Priority::Critical))
                .count();
            let urgent = tasks
                .iter()
                .filter(|t| matches!(t.priority, Priority::Urgent))
                .count();
            let high = tasks
                .iter()
                .filter(|t| matches!(t.priority, Priority::High))
                .count();
            let medium = tasks
                .iter()
                .filter(|t| matches!(t.priority, Priority::Medium))
                .count();
            let low = tasks
                .iter()
                .filter(|t| matches!(t.priority, Priority::Low))
                .count();
            format!(
                "\n🎯 By Priority:\n  🔴 Critical: {}\n  🚨 Urgent: {}\n  🟠 High: {}\n  🟡 Medium: {}\n  🟢 Low: {}",
                critical, urgent, high, medium, low
            )
        };
        let projects = Projects::list().await?;
        let project_stats = format!("\n📁 Projects: {} total", projects.len());
        Ok(format!("{}{}{}", quick_stats, by_priority, project_stats))
    }
}
pub struct ApiKeys;
impl ApiKeys {
    pub async fn add(key: ApiKey) -> Result<()> {
        let mut collection = load_api_key_collection()?;
        collection.add_key(key);
        // Note: Collection is automatically saved when modified
        Ok(())
    }
}

pub struct Tasks;
impl Tasks {
    pub async fn add(task: Task) -> Result<()> {
        let storage = Storage::new().await?;
        storage.add_task_to_project(task).await
    }
}

pub struct Checklist;
impl Checklist {
    pub async fn add_item(content: &str) -> Result<()> {
        use crate::tdz_tls::{TodoziProcessorState, ChecklistItem};
        let mut state = TodoziProcessorState::new()?;
        let item = ChecklistItem {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.to_string(),
            priority: "Medium".to_string(),
            completed: false,
            created_at: chrono::Utc::now(),
            source: "api".to_string(),
        };
        state.add_checklist_item(item);
        Ok(())
    }
}

pub struct Chunking;
impl Chunking {
    pub async fn add_chunk(chunk_id: String, level: String, deps: Vec<String>) -> Result<()> {
        use crate::chunking::{CodeGenerationGraph, ChunkingLevel};
        let mut graph = CodeGenerationGraph::new(1000); // Default max lines
        let chunk_level = level.parse::<ChunkingLevel>().unwrap_or(ChunkingLevel::Method);
        graph.add_chunk(chunk_id, chunk_level, deps);
        Ok(())
    }
    pub async fn add_completed_module(module: String) -> Result<()> {
        use crate::chunking::ProjectState;
        let mut state = ProjectState::new(1000); // Default max lines
        state.add_completed_module(module);
        Ok(())
    }
    pub async fn add_dependency(dep: String) -> Result<()> {
        use crate::chunking::CodeGenerationGraph;
        let mut graph = CodeGenerationGraph::new(1000);
        // For dependency operations, we need a chunk to exist first
        // This is a simplified implementation - in practice you'd specify which chunk
        if let Some(chunk) = graph.chunks.values_mut().next() {
            chunk.add_dependency(dep);
        }
        Ok(())
    }
    pub async fn add_error_pattern(pattern: String) -> Result<()> {
        use crate::chunking::ContextWindow;
        let mut context = ContextWindow::new();
        context.add_error_pattern(pattern);
        Ok(())
    }
    pub async fn add_function_signature(name: String, signature: String) -> Result<()> {
        use crate::chunking::ContextWindow;
        let mut context = ContextWindow::new();
        context.add_function_signature(name, signature);
        Ok(())
    }
    pub async fn add_import(import_stmt: String) -> Result<()> {
        use crate::chunking::ContextWindow;
        let mut context = ContextWindow::new();
        context.add_import(import_stmt);
        Ok(())
    }
    pub async fn add_pending_module(module: String) -> Result<()> {
        use crate::chunking::ProjectState;
        let mut state = ProjectState::new(1000); // Default max lines
        state.add_pending_module(module);
        Ok(())
    }
}


pub struct Easy;
impl Easy {
    pub async fn do_it(what: &str) -> Result<String> {
        Tdz::task(what).await
    }
    pub async fn find(what: &str) -> Result<String> {
        Find::tdz_find(what).await
    }
    pub async fn remember(what: &str) -> Result<String> {
        Memories::create(what, what, "Important to remember").await
    }
    pub async fn idea(what: &str) -> Result<String> {
        Ideas::create(what).await
    }
    pub async fn done(task_id: &str) -> Result<()> {
        Actions::complete(task_id).await
    }
    pub async fn see_all() -> Result<String> {
        Stats::quick().await
    }
}