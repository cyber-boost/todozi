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
#[cfg(feature = "nodejs")]
pub mod nodejs;
#[cfg(feature = "php")]
pub mod php;
#[cfg(feature = "python")]
pub mod python;
pub mod reminder;
#[cfg(feature = "ruby")]
pub mod ruby;
pub mod search;
pub mod server;
pub mod storage;
pub mod summary;
pub mod tags;
pub mod tdz;
pub mod tdz_tls;
pub mod todozi;
pub mod toolbox;
#[cfg(feature = "tui")]
pub mod tui;
pub mod types;
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
pub use server::{start_server, ServerConfig, TodoziServer};
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
    parse_agent_assignment_format, parse_error_format, parse_idea_format, parse_memory_format,
    parse_todozi_format, parse_training_data_format, process_chat_message, process_chat_message_extended,
    process_json_examples, ChatContent,
};
pub use toolbox::*;
#[cfg(feature = "tui")]
pub use tui::{
    ColorScheme, DisplayConfig, TaskDisplay, TaskEditor, TaskEvolutionAnalyzer,
    TaskEvolutionSummary, TaskListDisplay, TodoziApp, TuiService,
};
pub async fn init() -> Result<()> {
    storage::init_storage().await
}
pub async fn init_with_auto_registration() -> Result<()> {
    storage::init_storage().await?;
    let mut hlx = helix::Hlx::new().await?;
    let user_name_uuid = helix::xlh(&hlx, "@uuid", "").await?;
    let _user_name = if let helix::DnaValue::String(uuid) = user_name_uuid {
        let uname = format!("user_{}", &uuid[..8]);
        hlx.set("registration", "user_name", &uname);
        println!("✅ Set user_name: {}", uname);
        uname
    } else {
        "user_default".to_string()
    };
    let email_uuid = helix::xlh(&hlx, "@uuid", "").await?;
    let _user_email = if let helix::DnaValue::String(uuid) = email_uuid {
        let email = format!("hash_{}@example.com", &uuid[..8]);
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
                helix::DnaValue::String(registration.fingerprint.clone().unwrap_or_default()),
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
    let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
    let config_path = std::path::PathBuf::from(home_dir)
        .join(".todozi")
        .join("tdz.hlx");
    if !config_path.exists() {
        init_with_auto_registration().await?;
    } else {
        let hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
        let api_key = hlx
            .get("registration", "api_key")
            .and_then(|v| {
                if let helix::DnaValue::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        if api_key.is_empty() {
            init_with_auto_registration().await?;
        }
    }
    Ok(())
}
pub async fn get_tdz_api_key() -> Result<String> {
    let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
    let config_path = std::path::PathBuf::from(home_dir)
        .join(".todozi")
        .join("tdz.hlx");
    let hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
    let api_key = hlx
        .get("registration", "api_key")
        .and_then(|v| {
            if let helix::DnaValue::String(s) = v {
                Some(s.clone())
            } else {
                None
            }
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
    if str.is_some() {
        Some(format!("{}/{}", todozi_home, str.unwrap()))
    } else {
        Some(todozi_home)
    }
}
pub struct Ready; 
impl Ready {
    pub async fn init() -> Result<()> {
        ensure_todozi_initialized().await
    }
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
    pub async fn init_with_auto_registration() -> Result<()> {
        storage::init_storage().await?;
        let mut hlx = helix::Hlx::new().await?;
        let user_name_uuid = helix::xlh(&hlx, "@uuid", "").await?;
        let _user_name = if let helix::DnaValue::String(uuid) = user_name_uuid {
            let uname = format!("user_{}", &uuid[..8]);
            hlx.set("registration", "user_name", &uname);
            println!("✅ Set user_name: {}", uname);
            uname
        } else {
            "user_default".to_string()
        };
        let email_uuid = helix::xlh(&hlx, "@uuid", "").await?;
        let _user_email = if let helix::DnaValue::String(uuid) = email_uuid {
            let email = format!("hash_{}@example.com", &uuid[..8]);
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
                    helix::DnaValue::String(registration.fingerprint.clone().unwrap_or_default()),
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
    pub async fn todozi_begin() -> Result<()> {
        let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
        let config_path = std::path::PathBuf::from(home_dir)
            .join(".todozi")
            .join("tdz.hlx");
        if !config_path.exists() {
            init_with_auto_registration().await?;
        } else {
            let hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
            let api_key = hlx
                .get("registration", "api_key")
                .and_then(|v| {
                    if let helix::DnaValue::String(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            if api_key.is_empty() {
                init_with_auto_registration().await?;
            }
        }
        Ok(())
    }
    pub async fn get_tdz_api_key() -> Result<String> {
        let home_dir = std::env::var("HOME").expect("Could not get HOME environment variable");
        let config_path = std::path::PathBuf::from(home_dir)
            .join(".todozi")
            .join("tdz.hlx");
        let hlx = helix::Hlx::load(&*config_path.to_string_lossy()).await?;
        let api_key = hlx
            .get("registration", "api_key")
            .and_then(|v| {
                if let helix::DnaValue::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
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
        if str.is_some() {
            Some(format!("{}/{}", todozi_home, str.unwrap()))
        } else {
            Some(todozi_home)
        }
    }
    pub fn tdzfp() -> Result<bool> {
        storage::check_folder_structure()
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
        Ready::init().await?; 
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
        let embedded_task = if let Ok(mut emb_service) =
            TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await
        {
            if emb_service.initialize().await.is_ok() {
                let mut task_with_embedding = task.clone();
                task_with_embedding.embedding_vector =
                    Some(emb_service.generate_embedding(action).await?);
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
        Ready::init().await?; 
        let storage = Storage::new().await?;
        if semantic {
            if let Ok(mut emb_service) =
                TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await
            {
                if emb_service.initialize().await.is_ok() {
                    let similar = emb_service.find_similar_tasks(query, limit).await?;
                    let task_ids: Vec<String> = similar
                        .iter()
                        .filter_map(|s| s.text_content.split(' ').next().map(|s| s.to_string()))
                        .collect();
                    let all_tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;
                    return Ok(all_tasks
                        .into_iter()
                        .filter(|t| task_ids.contains(&t.id))
                        .take(limit.unwrap_or(10))
                        .collect());
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
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let updates = TaskUpdate {
            status: Some(status),
            ..Default::default()
        };
        storage.update_task_in_project(task_id, updates).await?;
        Ok(())
    }
    pub async fn extract_tasks(content: &str, context: Option<&str>) -> Result<Vec<String>> {
        Ready::init().await?; 
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
            return Err(TodoziError::api(format!(
                "API request failed: {}",
                response.status()
            )));
        }
        let result: serde_json::Value = response.json().await?;
        if let Some(extracted) = result.get("extracted_content") {
            if let Some(tasks) = extracted.get("tasks").and_then(|t| t.as_array()) {
                let task_actions: Vec<String> = tasks
                    .iter()
                    .filter_map(|t| {
                        t.get("action")
                            .and_then(|a| a.as_str())
                            .map(|s| s.to_string())
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
        Ready::init().await?; 
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
            return Err(TodoziError::api(format!(
                "API request failed: {}",
                response.status()
            )));
        }
        let result: serde_json::Value = response.json().await?;
        let mut tasks = Vec::new();
        if let Some(planning_result) = result.get("planning_result") {
            if let Some(api_tasks) = planning_result.get("tasks").and_then(|t| t.as_array()) {
                let storage = Storage::new().await?;
                for api_task in api_tasks {
                    if let Some(action) = api_task.get("action").and_then(|a| a.as_str()) {
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
                            tags: vec!["planned".to_string(), "ai_generated".to_string()],
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
        Ready::init().await?; 
        let storage = Storage::new().await?;
        storage.list_tasks_across_projects(&TaskFilters::default())
    }
    pub async fn get_task(task_id: &str) -> Result<Option<Task>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;
        Ok(tasks.into_iter().find(|t| t.id == task_id))
    }
    pub async fn delete_task(task_id: &str) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        storage.delete_task_from_project(task_id)?;
        Ok(())
    }
    pub async fn create_memory(moment: &str, meaning: &str, reason: &str) -> Result<Task> {
        Ready::init().await?; 
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
        Ready::init().await?; 
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
        Ready::init().await?; 
        process_chat_message_extended(message, user_id)
    }
    pub async fn storage() -> Result<Storage> {
        Ready::init().await?; 
        Storage::new().await
    }
    pub async fn embedding_service() -> Result<TodoziEmbeddingService> {
        TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await
    }
    pub async fn search_with_filters(
        filters: TaskFilters,
        limit: Option<usize>,
    ) -> Result<Vec<Task>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let mut tasks = storage.list_tasks_across_projects(&filters)?;
        if let Some(limit) = limit {
            tasks.truncate(limit);
        }
        Ok(tasks)
    }
    pub async fn update_task_full(task_id: &str, updates: TaskUpdate) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        storage.update_task_in_project(task_id, updates).await?;
        Ok(())
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
        Ready::init().await?; 
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

    // ========== Additional methods for Python bindings ==========

    pub async fn create_task_filters(
        project: Option<&str>,
        status: Option<&str>,
        priority: Option<&str>,
        assignee: Option<&str>,
        tags: Option<&str>,
        search: Option<&str>,
    ) -> Result<TaskFilters> {
        let mut filters = TaskFilters::default();
        if let Some(p) = project {
            filters.project = Some(p.to_string());
        }
        if let Some(s) = status {
            filters.status = s.parse().ok();
        }
        if let Some(p) = priority {
            filters.priority = p.parse().ok();
        }
        if let Some(a) = assignee {
            filters.assignee = a.parse().ok();
        }
        if let Some(t) = tags {
            filters.tags = Some(t.split(',').map(|s| s.trim().to_string()).collect());
        }
        if let Some(s) = search {
            filters.search = Some(s.to_string());
        }
        Ok(filters)
    }

    pub async fn create_task_update(
        action: Option<&str>,
        priority: Option<&str>,
        status: Option<&str>,
        project: Option<&str>,
    ) -> Result<TaskUpdate> {
        let mut update = TaskUpdate::default();
        if let Some(a) = action {
            update.action = Some(a.to_string());
        }
        if let Some(p) = priority {
            update.priority = p.parse().ok();
        }
        if let Some(s) = status {
            update.status = s.parse().ok();
        }
        if let Some(p) = project {
            update.parent_project = Some(p.to_string());
        }
        Ok(update)
    }

    pub async fn complete_task_in_project(task_id: &str) -> Result<()> {
        Self::update_task_status(task_id, Status::Done).await
    }

    // Methods that should delegate to other modules or return errors
    pub async fn add(_action: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: "Use Actions::ai(), Actions::human(), or Actions::collab() instead"
                .to_string(),
        })
    }

    pub async fn analyze_code_quality(_features: &[f32]) -> Result<f32> {
        Err(TodoziError::ValidationError {
            message: "analyze_code_quality not yet implemented".to_string(),
        })
    }

    pub async fn api(_message: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "api"),
        })
    }

    pub async fn as_str() -> Result<&'static str> {
        Ok("Done")
    }

    pub async fn auto_label_clusters(
        _clusters: Vec<emb::ClusteringResult>,
    ) -> Result<Vec<emb::LabeledCluster>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "auto_label_clusters"),
        })
    }

    pub async fn backup_embeddings(_backup_path: Option<&str>) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "backup_embeddings"),
        })
    }

    pub async fn breakthrough_percentage() -> Result<f64> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "breakthrough_percentage"),
        })
    }

    pub async fn build_similarity_graph(_threshold: f32) -> Result<emb::SimilarityGraph> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "build_similarity_graph"),
        })
    }

    pub async fn calculate_diversity(_content_ids: Vec<String>) -> Result<f32> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "calculate_diversity"),
        })
    }

    pub async fn capabilities(_capabilities: Vec<String>) -> Result<()> {
        Ok(())
    }

    pub async fn category(_category: &str) -> Result<()> {
        Ok(())
    }

    pub async fn check_folder_structure() -> Result<bool> {
        Ready::init().await?; 
        storage::check_folder_structure()
    }

    pub async fn cleanup_expired() -> Result<usize> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "cleanup_expired"),
        })
    }

    pub async fn cleanup_legacy() -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "cleanup_legacy"),
        })
    }

    pub async fn cli_fix_consistency() -> Result<()> {
        Ready::init().await?; 
        let _storage = Storage::new().await?;
        // TODO: Implement fix_consistency
        Ok(())
    }


    pub async fn color(_color: &str) -> Result<()> {
        Ok(())
    }

    pub async fn compare_models(
        _text: &str,
        _model_aliases: Vec<String>,
    ) -> Result<emb::ModelComparisonResult> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "compare_models"),
        })
    }

    pub async fn completion_rate() -> Result<f64> {
        let tasks = Self::all_tasks().await?;
        let completed = tasks.iter().filter(|t| t.status == Status::Done).count();
        Ok(if tasks.is_empty() {
            0.0
        } else {
            completed as f64 / tasks.len() as f64
        })
    }

    pub async fn config() -> Result<Config> {
        Ready::init().await?; 
        storage::load_config().await
    }

    pub async fn content(_content: &str) -> Result<()> {
        Ok(())
    }

    pub async fn context(_context: &str) -> Result<()> {
        Ok(())
    }

    pub async fn craft_embedding(_features: &[f32]) -> Result<Vec<f32>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "craft_embedding"),
        })
    }

    pub async fn create(_name: &str, _description: Option<&str>) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: "Use specific create methods like create_task() instead".to_string(),
        })
    }

    pub async fn create_advanced_todozi_tools(
        _todozi: crate::todozi_tool::SharedTodozi,
    ) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_advanced_todozi_tools"),
        })
    }

    pub async fn create_architect_agent() -> Result<Agent> {
        use crate::agent::AgentManager;
        let mut manager = AgentManager::new();
        let agent = Agent::create_coder(); // Use existing method as template
        let id = manager.create_agent(agent.clone()).await?;
        Ok(agent)
    }

    pub async fn create_backup() -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_backup"),
        })
    }

    pub async fn create_coder() -> Result<Agent> {
        Ok(Agent::create_coder())
    }

    pub async fn create_comrad_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_comrad_agent"),
        })
    }

    pub async fn create_custom_agent(
        id: &str,
        name: &str,
        description: &str,
        capabilities: Vec<String>,
        specializations: Vec<String>,
        _category: &str,
        _author: Option<&str>,
    ) -> Result<String> {
        use crate::agent::AgentManager;
        let mut manager = AgentManager::new();
        let mut agent = Agent::new(id.to_string(), name.to_string(), description.to_string());
        agent.capabilities = capabilities;
        agent.specializations = specializations;
        manager.create_agent(agent).await
    }

    pub async fn create_default_agents() -> Result<()> {
        use crate::agent::AgentManager;
        let mut manager = AgentManager::new();
        manager.load_agents().await?;
        Ok(())
    }

    pub async fn create_designer_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_designer_agent"),
        })
    }

    pub async fn create_detective_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_detective_agent"),
        })
    }

    pub async fn create_devops_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_devops_agent"),
        })
    }

    pub async fn create_embedding_version(
        _content_id: &str,
        _version_label: &str,
    ) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_embedding_version"),
        })
    }

    pub async fn create_error(_error: Error) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_error"),
        })
    }

    pub async fn create_error_result(
        error_msg: &str,
        execution_time_ms: u64,
        error_type: crate::base::ErrorType,
        _metadata: Option<&std::collections::HashMap<String, serde_json::Value>>,
    ) -> Result<crate::base::ToolResult> {
        Ok(crate::base::ToolResult {
            success: false,
            output: error_msg.to_string(),
            error: Some(error_msg.to_string()),
            execution_time_ms,
            metadata: None,
            recovery_context: None,
        })
    }

    pub async fn create_finisher_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_finisher_agent"),
        })
    }

    pub async fn create_framer_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_framer_agent"),
        })
    }

    pub async fn create_friend_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_friend_agent"),
        })
    }

    pub async fn create_grok_level_todozi_tools(
        _todozi: std::sync::Arc<std::sync::Mutex<Storage>>,
    ) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_grok_level_todozi_tools"),
        })
    }

    pub async fn create_hoarder_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_hoarder_agent"),
        })
    }

    pub async fn create_investigator_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_investigator_agent"),
        })
    }

    pub async fn create_mason_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_mason_agent"),
        })
    }

    pub async fn create_nerd_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_nerd_agent"),
        })
    }

    pub async fn create_nun_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_nun_agent"),
        })
    }

    pub async fn create_overlord_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_overlord_agent"),
        })
    }

    pub async fn create_party_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_party_agent"),
        })
    }

    pub async fn create_planner_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_planner_agent"),
        })
    }

    pub async fn create_recycler_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_recycler_agent"),
        })
    }

    pub async fn create_skeleton_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_skeleton_agent"),
        })
    }

    pub async fn create_snitch_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_snitch_agent"),
        })
    }

    pub async fn create_success_result(
        output: &str,
        execution_time_ms: u64,
        _metadata: Option<&std::collections::HashMap<String, serde_json::Value>>,
    ) -> Result<crate::base::ToolResult> {
        Ok(crate::base::ToolResult {
            success: true,
            output: output.to_string(),
            error: None,
            execution_time_ms,
            metadata: None,
            recovery_context: None,
        })
    }

    pub async fn create_tdz_content_processor_tool(
        _state: crate::tdz_tls::SharedTodoziState,
    ) -> Result<Box<dyn crate::base::Tool + Send + Sync>> {
        Err(TodoziError::ValidationError {
            message: format!(
                "{} not yet implemented",
                "create_tdz_content_processor_tool"
            ),
        })
    }

    pub async fn create_tester_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_tester_agent"),
        })
    }

    pub async fn create_todozi_tools(
        _todozi: std::sync::Arc<std::sync::Mutex<Storage>>,
    ) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_todozi_tools"),
        })
    }

    pub async fn create_todozi_tools_with_embedding(
        _todozi: crate::todozi_tool::SharedTodozi,
        _embedding_service: Option<&TodoziEmbeddingService>,
    ) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!(
                "{} not yet implemented",
                "create_todozi_tools_with_embedding"
            ),
        })
    }

    pub async fn create_tool_definition_with_locks(
        _name: &str,
        _description: &str,
        _category: &str,
        _parameters: Vec<crate::base::ToolParameter>,
        _locks: Vec<crate::ResourceLock>,
    ) -> Result<crate::base::ToolDefinition> {
        Err(TodoziError::ValidationError {
            message: format!(
                "{} not yet implemented",
                "create_tool_definition_with_locks"
            ),
        })
    }

    pub async fn create_tuner_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_tuner_agent"),
        })
    }

    pub async fn create_writer_agent() -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "create_writer_agent"),
        })
    }

    pub async fn critical_percentage() -> Result<f64> {
        let tasks = Self::all_tasks().await?;
        let critical = tasks
            .iter()
            .filter(|t| t.priority == Priority::Critical)
            .count();
        Ok(if tasks.is_empty() {
            0.0
        } else {
            critical as f64 / tasks.len() as f64
        })
    }

    pub async fn deactivate_key(_key_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "deactivate_key"),
        })
    }

    pub async fn delete_agent_assignment(_assignment_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_agent_assignment"),
        })
    }

    pub async fn delete_code_chunk(_chunk_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_code_chunk"),
        })
    }

    pub async fn delete_error(_error_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_error"),
        })
    }

    pub async fn delete_feeling(_feeling_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_feeling"),
        })
    }

    pub async fn delete_idea(_idea_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_idea"),
        })
    }

    pub async fn delete_memory(_memory_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_memory"),
        })
    }

    pub async fn delete_project_task_container(_container_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_project_task_container"),
        })
    }

    pub async fn delete_task_from_project(_task_id: &str) -> Result<()> {
        Self::delete_task(_task_id).await
    }

    pub async fn delete_training_data(_training_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "delete_training_data"),
        })
    }

    pub async fn description(_description: &str) -> Result<()> {
        Ok(())
    }

    pub async fn display_task(_task_id: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "display_task"),
        })
    }

    pub async fn display_tasks() -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "display_tasks"),
        })
    }

    pub async fn dry_run(_command: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "dry_run"),
        })
    }

    pub async fn embed_idea(_idea_id: &str) -> Result<Vec<f32>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "embed_idea"),
        })
    }

    pub async fn embed_memory(_memory_id: &str) -> Result<Vec<f32>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "embed_memory"),
        })
    }

    pub async fn embed_tag(_tag_id: &str) -> Result<Vec<f32>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "embed_tag"),
        })
    }

    pub async fn encode(_text: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "encode"),
        })
    }

    pub async fn end_queue_session(_session_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "end_queue_session"),
        })
    }

    pub async fn end_session(_session_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "end_session"),
        })
    }

    pub async fn ensure_folder_structure() -> Result<bool> {
        Ready::init().await?; 
        storage::ensure_folder_structure().await
    }

    pub async fn error(_error: &str) -> Result<()> {
        Ok(())
    }

    pub async fn example() -> Result<&'static str> {
        Ok("Example usage")
    }

    pub async fn example_usage() -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "example_usage"),
        })
    }

    pub async fn execute_task(_task_id: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "execute_task"),
        })
    }

    pub async fn execute_tdz_command(_command: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "execute_tdz_command"),
        })
    }

    pub async fn execute_todozi_tool_delegated(
        _params: &str,
    ) -> Result<crate::toolbox::todozi_exe::ExecutionResult> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "execute_todozi_tool_delegated"),
        })
    }

    pub async fn explain_search_result(_result_id: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "explain_search_result"),
        })
    }

    pub async fn export_diagnostics(_path: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "export_diagnostics"),
        })
    }

    pub async fn export_embedded_tasks_hlx(_path: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "export_embedded_tasks_hlx"),
        })
    }

    pub async fn export_for_fine_tuning(_path: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "export_for_fine_tuning"),
        })
    }

    pub async fn filtered_semantic_search(
        _query: &str,
        _filters: &emb::SearchFilters,
    ) -> Result<Vec<emb::SimilarityResult>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "filtered_semantic_search"),
        })
    }

    pub async fn find_best_agent(_task_description: &str) -> Result<Agent> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "find_best_agent"),
        })
    }

    pub async fn find_cross_content_relationships(
        _content_ids: Vec<String>,
    ) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "find_cross_content_relationships"),
        })
    }

    pub async fn find_outliers(_content_ids: Vec<String>) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "find_outliers"),
        })
    }

    pub async fn find_similar_tags(_tag_id: &str) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "find_similar_tags"),
        })
    }

    pub async fn get_agent(_agent_id: &str) -> Result<Option<Agent>> {
        use crate::agent::AgentManager;
        let manager = AgentManager::new();
        Ok(manager.get_agent(_agent_id).cloned())
    }

    pub async fn get_all_agents() -> Result<Vec<Agent>> {
        use crate::agent::AgentManager;
        let manager = AgentManager::new();
        Ok(manager.get_all_agents().into_iter().cloned().collect())
    }

    pub async fn get_available_agents() -> Result<Vec<Agent>> {
        use crate::agent::AgentManager;
        let manager = AgentManager::new();
        Ok(manager
            .get_available_agents()
            .into_iter()
            .cloned()
            .collect())
    }

    pub async fn get_code_chunk(_chunk_id: &str) -> Result<Option<crate::chunking::CodeChunk>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_code_chunk"),
        })
    }

    pub async fn get_error(_error_id: &str) -> Result<Option<Error>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_error"),
        })
    }

    pub async fn get_feeling(_feeling_id: &str) -> Result<Option<Feeling>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_feeling"),
        })
    }

    pub async fn get_idea(_idea_id: &str) -> Result<Option<Idea>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_idea"),
        })
    }

    pub async fn get_memory(_memory_id: &str) -> Result<Option<Memory>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_memory"),
        })
    }

    pub async fn get_project_task_container(
        _container_id: &str,
    ) -> Result<Option<ProjectTaskContainer>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_project_task_container"),
        })
    }

    pub async fn get_training_data(_training_id: &str) -> Result<Option<TrainingData>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "get_training_data"),
        })
    }

    pub async fn hierarchical_cluster(
        _content_ids: Vec<String>,
        _depth: usize,
    ) -> Result<emb::HierarchicalCluster> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "hierarchical_cluster"),
        })
    }

    pub async fn idea_statistics() -> Result<crate::idea::IdeaStatistics> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "idea_statistics"),
        })
    }

    pub async fn import_embeddings(_path: &str) -> Result<usize> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "import_embeddings"),
        })
    }

    pub async fn import_project(_path: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "import_project"),
        })
    }

    pub async fn initialize_embedding_service() -> Result<()> {
        let mut service = Self::embedding_service().await?;
        service.initialize().await?;
        Ok(())
    }

    pub async fn is_registered() -> Result<bool> {
        storage::is_registered().await
    }

    pub async fn list_agent_assignments() -> Result<Vec<AgentAssignment>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_agent_assignments"),
        })
    }

    pub async fn list_code_chunks() -> Result<Vec<crate::chunking::CodeChunk>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_code_chunks"),
        })
    }

    pub async fn list_errors() -> Result<Vec<Error>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_errors"),
        })
    }

    pub async fn list_feelings() -> Result<Vec<Feeling>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_feelings"),
        })
    }

    pub async fn list_ideas() -> Result<Vec<Idea>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_ideas"),
        })
    }

    pub async fn list_memories() -> Result<Vec<Memory>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_memories"),
        })
    }

    pub async fn list_project_task_containers() -> Result<Vec<ProjectTaskContainer>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_project_task_containers"),
        })
    }

    pub async fn list_projects() -> Result<Vec<String>> {
        Ready::init().await?; 
        storage::list_projects().map(|projects| projects.into_iter().map(|p| p.name).collect())
    }

    pub async fn list_training_data() -> Result<Vec<TrainingData>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "list_training_data"),
        })
    }

    pub async fn list_all_agent_assignments() -> Result<Vec<AgentAssignment>> {
        Ready::init().await?; 
        crate::storage::list_all_agent_assignments()
    }

    pub async fn list_backlog_items() -> Result<Vec<QueueItem>> {
        Ready::init().await?; 
        crate::storage::list_backlog_items()
    }

    pub async fn list_backups() -> Result<Vec<String>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        storage.list_backups()
    }

    pub async fn list_complete_items() -> Result<Vec<QueueItem>> {
        Ready::init().await?; 
        crate::storage::list_complete_items()
    }

    pub async fn list_tasks_across_projects(filters: &TaskFilters) -> Result<Vec<Task>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        storage.list_tasks_across_projects(filters)
    }

    pub async fn list_tasks_in_project(project_name: &str, filters: &TaskFilters) -> Result<Vec<Task>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        storage.list_tasks_in_project(project_name, filters)
    }

    pub async fn load(_model_name: &str, _device: candle_core::Device) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "load"),
        })
    }

    pub async fn load_additional_model(_model_name: &str, _model_alias: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "load_additional_model"),
        })
    }

    pub async fn load_agent(agent_id: &str) -> Result<Agent> {
        Ready::init().await?; 
        crate::storage::load_agent(agent_id)
    }

    pub async fn load_agent_assignment(agent_id: &str, task_id: &str) -> Result<AgentAssignment> {
        Ready::init().await?; 
        crate::storage::load_agent_assignment(agent_id, task_id)
    }

    pub async fn load_agents() -> Result<()> {
        Ready::init().await?; 
        Ok(())
    }

    pub async fn load_api_key_collection() -> Result<ApiKeyCollection> {
        Ready::init().await?; 
        crate::api::load_api_key_collection()
    }

    pub async fn load_api_keys() -> Result<()> {
        Ready::init().await?; 
        Ok(())
    }

    pub async fn load_code_chunk(chunk_id: &str) -> Result<crate::chunking::CodeChunk> {
        Ready::init().await?; 
        crate::storage::load_code_chunk(chunk_id)
    }

    pub async fn load_config() -> Result<Config> {
        Ready::init().await?; 
        crate::storage::load_config().await
    }

    pub async fn load_error(error_id: &str) -> Result<Error> {
        Ready::init().await?; 
        crate::storage::load_error(error_id)
    }

    pub async fn load_extended_data() -> Result<()> {
        Ready::init().await?; 
        Ok(())
    }

    pub async fn load_feeling(id: &str) -> Result<Feeling> {
        Ready::init().await?; 
        crate::storage::load_feeling(id)
    }

    pub async fn load_idea(idea_id: &str) -> Result<Idea> {
        Ready::init().await?; 
        crate::storage::load_idea(idea_id)
    }

    pub async fn load_memory(memory_id: &str) -> Result<Memory> {
        Ready::init().await?; 
        crate::storage::load_memory(memory_id)
    }

    pub async fn load_project_task_container(project_name: &str) -> Result<ProjectTaskContainer> {
        Ready::init().await?; 
        crate::storage::load_project_task_container(project_name)
    }

    pub async fn load_project_task_container_by_hash(project_hash: &str) -> Result<ProjectTaskContainer> {
        Ready::init().await?; 
        crate::storage::load_project_task_container_by_hash(project_hash)
    }

    pub async fn load_queue_collection() -> Result<QueueCollection> {
        Ready::init().await?; 
        crate::storage::load_queue_collection()
    }

    pub async fn load_task_collection(collection_name: &str) -> Result<TaskCollection> {
        Ready::init().await?; 
        crate::storage::load_task_collection(collection_name)
    }

    pub async fn memory_statistics() -> Result<crate::memory::MemoryStatistics> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "memory_statistics"),
        })
    }

    pub async fn migrate_project(_project_name: &str) -> Result<crate::models::MigrationReport> {
        Err(TodoziError::ValidationError {
            message: "migrate_project not yet implemented".to_string(),
        })
    }

    pub async fn project_statistics(_project_name: &str) -> Result<ProjectStats> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "project_statistics"),
        })
    }

    pub async fn register(_server_url: &str) -> Result<crate::models::RegistrationInfo> {
        Ready::init().await?; 
        storage::register_with_server(_server_url).await
    }

    pub async fn registration_status() -> Result<Option<crate::models::RegistrationInfo>> {
        Ready::init().await?; 
        storage::get_registration_info().await
    }

    pub async fn search_analytics() -> Result<crate::search::SearchAnalytics> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "search_analytics"),
        })
    }

    pub async fn search_results(_query: &str) -> Result<crate::search::SearchResults> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "search_results"),
        })
    }

    pub async fn semantic_search(
        _query: &str,
        _limit: Option<usize>,
    ) -> Result<Vec<emb::SimilarityResult>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "semantic_search"),
        })
    }

    pub async fn start_queue_session(_description: &str) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "start_queue_session"),
        })
    }

    pub async fn summary_statistics() -> Result<crate::summary::SummaryStatistics> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "summary_statistics"),
        })
    }

    pub async fn tag_statistics() -> Result<crate::tags::TagStatistics> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "tag_statistics"),
        })
    }

    pub async fn update_agent(_agent_id: &str, _updates: crate::agent::AgentUpdate) -> Result<()> {
        use crate::agent::AgentManager;
        let mut manager = AgentManager::new();
        manager.update_agent(_agent_id, _updates).await
    }

    pub async fn update_idea(_idea_id: &str, _updates: crate::idea::IdeaUpdate) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "update_idea"),
        })
    }

    pub async fn update_memory(
        _memory_id: &str,
        _updates: crate::memory::MemoryUpdate,
    ) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "update_memory"),
        })
    }

    pub async fn validate_commands(_commands: Vec<crate::tdz::TdzCommand>) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "validate_commands"),
        })
    }

    pub async fn validate_project(_project_name: &str) -> Result<crate::emb::ValidationReport> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "validate_project"),
        })
    }

    // Methods for Python bindings that were missing

    pub async fn tool_count() -> Result<usize> {
        Ok(0) // Placeholder implementation
    }

    pub async fn total_results() -> Result<usize> {
        Ok(0) // Placeholder implementation
    }

    pub async fn to_ollama_format() -> Result<serde_json::Value> {
        let registry = ToolRegistry::new();
        let definitions = registry.get_tool_definitions();
        Ok(serde_json::json!(definitions))
    }

    pub async fn to_state_string() -> Result<String> {
        let state = crate::chunking::ProjectState::new(1000);
        Ok(state.to_state_string())
    }

    pub async fn title() -> Result<&'static str> {
        Ok("Todozi")
    }

    pub async fn to_context_string() -> Result<String> {
        let context = crate::chunking::ContextWindow::new();
        Ok(context.to_context_string())
    }

    pub async fn update_task_in_project(task_id: &str, updates: crate::models::TaskUpdate) -> Result<()> {
        Self::update_task_full(task_id, updates).await
    }

    pub async fn track_embedding_drift(
        _content_id: &str,
        _current_text: &str,
    ) -> Result<crate::emb::DriftReport> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "track_embedding_drift"),
        })
    }

    pub async fn transform_shorthand_tags(_message: &str) -> Result<String> {
        Ok(_message.to_string()) // Placeholder implementation
    }

    pub async fn types() -> Result<&'static str> {
        Ok("Available types: Task, Priority, Status, Assignee, TaskFilters, TaskUpdate, ChatContent, TodoziEmbeddingService, TodoziEmbeddingConfig")
    }

    pub async fn unregister(_name: &str) -> Result<bool> {
        Ok(false) // Placeholder implementation
    }

    pub async fn update(_updates: crate::models::TaskUpdate) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_agent_assignment_status(
        _agent_id: &str,
        _task_id: &str,
        _status: crate::models::AssignmentStatus,
    ) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_chunk_code(_chunk_id: &str, _code: &str) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_chunk_tests(_chunk_id: &str, _tests: &str) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_config(_config: crate::models::Config) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_config_with_registration(
        _registration: crate::models::RegistrationInfo,
    ) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_feeling(_feeling: &crate::models::Feeling) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_project(_project: crate::models::Project) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_registration_api_key(_api_key: &str) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_registration_keys(
        _api_key: &str,
        _user_id: Option<&str>,
        _fingerprint: Option<&str>,
    ) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn update_task(_id: &str, _updates: crate::models::TaskUpdate) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn validate_embeddings() -> Result<crate::emb::ValidationReport> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "validate_embeddings"),
        })
    }

    pub async fn validate_migration() -> Result<bool> {
        Ok(true) // Placeholder implementation
    }

    pub async fn validate_required_params(
        _kwargs: &std::collections::HashMap<String, serde_json::Value>,
        _required_params: &[String],
    ) -> Result<Option<crate::base::ToolResult>> {
        Ok(None) // Placeholder implementation
    }

    pub async fn validate_string_param(
        _value: &serde_json::Value,
        _param_name: &str,
        _min_length: usize,
        _max_length: usize,
        _pattern: Option<&str>,
    ) -> Result<Option<crate::base::ToolResult>> {
        Ok(None) // Placeholder implementation
    }

    pub async fn validate_task_input(
        _action: &str,
        _time: &str,
        _priority: &str,
        _project: &str,
        _status: &str,
        _assignee: Option<&str>,
        _progress: Option<&u8>,
    ) -> Result<()> {
        Ok(()) // Placeholder implementation
    }

    pub async fn validation(_message: impl Into<String>) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn verbose(_verbose: bool) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_action(_action: &str) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_assignee(_assignee: crate::models::Assignee) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_context(_context: &str) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_context_notes(_context_notes: &str) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_dependencies(_dependencies: Vec<String>) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_dry_run(_dry_run: bool) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_embedding_service(
        _service: crate::emb::TodoziEmbeddingService,
    ) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_embedding_service_option(
        _service: Option<&crate::emb::TodoziEmbeddingService>,
    ) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_force(_force: bool) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_max_tokens(_max_tokens: u32) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_parent_project(_parent_project: &str) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_priority(_priority: crate::models::Priority) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_progress(_progress: u8) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_shared_components(
        _config: std::sync::Arc<tokio::sync::Mutex<crate::emb::TodoziEmbeddingConfig>>,
        _cache: std::sync::Arc<
            tokio::sync::Mutex<std::collections::HashMap<String, crate::emb::TodoziEmbeddingCache>>,
        >,
        _embedding_model: std::sync::Arc<
            tokio::sync::Mutex<Option<std::sync::Arc<crate::emb::EmbeddingModel>>>,
        >,
        _embedding_models: std::sync::Arc<
            tokio::sync::Mutex<
                std::collections::HashMap<String, std::sync::Arc<crate::emb::EmbeddingModel>>,
            >,
        >,
        _tag_manager: std::sync::Arc<tokio::sync::Mutex<crate::tags::TagManager>>,
        _storage: std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>,
    ) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_status(_status: crate::models::Status) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_tags(_tags: Vec<String>) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub async fn with_time(_time: &str) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    pub fn with_temperature(_temperature: f32) -> Self {
        Self // Placeholder implementation
    }

    pub async fn with_user_id(_user_id: &str) -> Result<Self> {
        Ok(Self) // Placeholder implementation
    }

    // ========== Methods for Python bindings ==========

    pub async fn load_tasks() -> Result<()> {
        Ready::init().await?; 
        Ok(())
    }

    pub async fn load_training_data(training_data_id: &str) -> Result<TrainingData> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "load_training_data"),
        })
    }

    pub async fn long_term_percentage() -> Result<f64> {
        let tasks = Self::all_tasks().await?;
        let long_term = tasks
            .iter()
            .filter(|t| t.time.to_lowercase().contains("long") || t.time.to_lowercase().contains("future"))
            .count();
        Ok(if tasks.is_empty() {
            0.0
        } else {
            long_term as f64 / tasks.len() as f64
        })
    }

    pub async fn mark_chunk_completed(chunk_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "mark_chunk_completed"),
        })
    }

    pub async fn mark_chunk_validated(chunk_id: &str) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "mark_chunk_validated"),
        })
    }

    pub async fn matches(public_key: &str, private_key: Option<&str>) -> Result<bool> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "matches"),
        })
    }

    pub async fn max_tokens() -> Result<usize> {
        Ok(4096)
    }

    pub async fn meaning(meaning: &str) -> Result<Self> {
        Ok(Self)
    }

    pub async fn migrate() -> Result<MigrationReport> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "migrate"),
        })
    }

    pub async fn migrate_to_project_based() -> Result<MigrationReport> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "migrate_to_project_based"),
        })
    }

    pub async fn moment(moment: &str) -> Result<Self> {
        Ok(Self)
    }

    pub async fn move_task(id: &str, from_collection: &str, to_collection: &str) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        if let Some(mut task) = Self::get_task(id).await? {
            task.parent_project = to_collection.to_string();
            let updates = TaskUpdate {
                parent_project: Some(to_collection.to_string()),
                ..Default::default()
            };
            storage.update_task_in_project(id, updates).await?;
        }
        Ok(())
    }

    pub async fn multi_query_search(
        queries: Vec<String>,
        aggregation: emb::AggregationType,
        content_types: Option<&[emb::TodoziContentType]>,
        limit: usize,
    ) -> Result<Vec<emb::SimilarityResult>> {
        Ready::init().await?; 
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let queries_ref: Vec<&str> = queries.iter().map(|s| s.as_str()).collect();
        let content_types_vec = content_types.map(|ct| ct.to_vec());
        emb_service.multi_query_search(queries_ref, aggregation, content_types_vec, limit).await
    }

    pub async fn name(name: &str) -> Result<Self> {
        Ok(Self)
    }

    pub async fn new_full(
        user_id: &str,
        action: &str,
        time: &str,
        priority: Priority,
        parent_project: &str,
        status: Status,
        assignee: Option<&Assignee>,
        tags: Vec<String>,
        dependencies: Vec<String>,
        context_notes: Option<&str>,
        progress: Option<&u8>,
    ) -> Result<Self> {
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            action: action.to_string(),
            time: time.to_string(),
            priority,
            parent_project: parent_project.to_string(),
            status,
            assignee: assignee.cloned(),
            tags,
            dependencies,
            context_notes: context_notes.map(|s| s.to_string()),
            progress: progress.copied(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            embedding_vector: None,
        };
        let storage = Storage::new().await?;
        storage.add_task_to_project(task).await?;
        Ok(Self)
    }

    pub async fn new_idea(idea: Idea) -> Result<String> {
        Ready::init().await?; 
        crate::storage::save_idea(&idea)?;
        Ok(idea.id)
    }

    pub async fn new_memory(memory: Memory) -> Result<String> {
        Ready::init().await?; 
        crate::storage::save_memory(&memory)?;
        Ok(memory.id)
    }

    pub async fn new_with_hashes(server_url: &str) -> Result<Self> {
        Ready::init().await?; 
        Ok(Self)
    }

    pub async fn ok(body: &str) -> Result<Self> {
        Ok(Self)
    }

    pub async fn overdue_percentage() -> Result<f64> {
        let tasks = Self::all_tasks().await?;
        let now = chrono::Utc::now();
        let overdue = tasks
            .iter()
            .filter(|t| {
                if let Ok(deadline) = chrono::DateTime::parse_from_rfc3339(&t.time) {
                    deadline < now
                } else {
                    false
                }
            })
            .count();
        Ok(if tasks.is_empty() {
            0.0
        } else {
            overdue as f64 / tasks.len() as f64
        })
    }

    pub async fn parse_agent_assignment_format(agent_text: &str) -> Result<AgentAssignment> {
        crate::todozi::parse_agent_assignment_format(agent_text)
    }

    pub async fn parse_chunking_format(chunk_text: &str) -> Result<crate::chunking::CodeChunk> {
        crate::chunking::parse_chunking_format(chunk_text).map_err(|e| TodoziError::ValidationError {
            message: e,
        })
    }

    pub async fn parse_dependencies(deps_str: &str) -> Result<Vec<String>> {
        Ok(deps_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    pub async fn parse_error_format(error_text: &str) -> Result<Error> {
        crate::todozi::parse_error_format(error_text)
    }

    pub async fn parse_feeling_format(feel_text: &str) -> Result<Feeling> {
        crate::todozi::parse_feeling_format(feel_text)
    }

    pub async fn parse_idea_format(idea_text: &str) -> Result<Idea> {
        crate::todozi::parse_idea_format(idea_text)
    }

    pub async fn parse_memory_format(memory_text: &str, user_id: &str) -> Result<Memory> {
        crate::todozi::parse_memory_format(memory_text, user_id)
    }

    pub async fn parse_reminder_format(reminder_text: &str) -> Result<Reminder> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "parse_reminder_format"),
        })
    }

    pub async fn parse_summary_format(summary_text: &str) -> Result<Summary> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "parse_summary_format"),
        })
    }

    pub async fn parse_tags(tags_str: &str) -> Result<Vec<String>> {
        Ok(tags_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
    }

    pub async fn parse_tdz_command(text: &str) -> Result<Vec<TdzCommand>> {
        crate::tdz::parse_tdz_command(text)
    }

    pub async fn parse_todozi_format(todozi_text: &str) -> Result<Task> {
        crate::todozi::parse_todozi_format(todozi_text)
    }

    pub async fn parse_training_data_format(train_text: &str) -> Result<TrainingData> {
        crate::todozi::parse_training_data_format(train_text)
    }

    pub async fn pending_percentage() -> Result<f64> {
        let tasks = Self::all_tasks().await?;
        let pending = tasks
            .iter()
            .filter(|t| t.status == Status::Todo)
            .count();
        Ok(if tasks.is_empty() {
            0.0
        } else {
            (pending as f64 / tasks.len() as f64) * 100.0
        })
    }

    pub async fn predict_relevance(_features: &[f32]) -> Result<f32> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "predict_relevance"),
        })
    }

    pub async fn preload_related_embeddings(_content_id: &str, _depth: usize) -> Result<()> {
        if let Ok(mut emb_service) = Self::embedding_service().await {
            if emb_service.initialize().await.is_ok() {
                emb_service.preload_related_embeddings(_content_id, _depth).await?;
                return Ok(());
            }
        }
        Err(TodoziError::ValidationError {
            message: "Embedding service not available".to_string(),
        })
    }

    pub async fn prepare_task_content(task: &Task) -> Result<String> {
        if let Ok(mut emb_service) = Self::embedding_service().await {
            if emb_service.initialize().await.is_ok() {
                return Ok(emb_service.prepare_task_content(task));
            }
        }
        Ok(format!("{} - {}", task.action, task.context_notes.as_deref().unwrap_or("")))
    }

    pub async fn priority(_priority: SummaryPriority) -> Result<Self> {
        Ok(Self)
    }

    pub async fn private_percentage() -> Result<f64> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "private_percentage"),
        })
    }

    pub async fn process_chat_message(message: &str) -> Result<Vec<Task>> {
        Ok(todozi::process_chat_message(message)?)
    }

    pub async fn process_chat_message_extended(message: &str, user_id: &str) -> Result<ChatContent> {
        Ok(todozi::process_chat_message_extended(message, user_id)?)
    }

    pub async fn process_chunking_message(message: &str) -> Result<Vec<crate::chunking::CodeChunk>> {
        Ok(crate::chunking::process_chunking_message(message).map_err(|e| TodoziError::ValidationError { message: e })?)
    }

    pub async fn process_json_examples(json_data: &str) -> Result<Vec<Task>> {
        Ok(todozi::process_json_examples(json_data)?)
    }

    pub async fn process_tdz_commands(text: &str, base_url: &str, api_key: Option<&str>) -> Result<Vec<serde_json::Value>> {
        crate::tdz::process_tdz_commands(text, base_url, api_key).await
    }

    pub async fn process_workflow(_tasks: Vec<Task>) -> Result<Vec<String>> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "process_workflow"),
        })
    }

    pub async fn profile_search_performance(query: &str, iterations: usize) -> Result<emb::PerformanceMetrics> {
        if let Ok(mut emb_service) = Self::embedding_service().await {
            if emb_service.initialize().await.is_ok() {
                return Ok(emb_service.profile_search_performance(query, iterations).await?);
            }
        }
        Err(TodoziError::ValidationError {
            message: "Embedding service not available".to_string(),
        })
    }

    pub async fn public_percentage() -> Result<f64> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "public_percentage"),
        })
    }


    pub async fn reason(_reason: &str) -> Result<Self> {
        Ok(Self)
    }

    pub async fn register_with_server(server_url: &str) -> Result<RegistrationInfo> {
        Ready::init().await?; 
        storage::register_with_server(server_url).await
    }

    pub async fn relationships_per_tag() -> Result<f64> {
        Err(TodoziError::ValidationError {
            message: format!("{} not yet implemented", "relationships_per_tag"),
        })
    }

    pub async fn recommend_similar(
        based_on: Vec<String>,
        exclude: Vec<String>,
        limit: usize,
    ) -> Result<Vec<SimilarityResult>> {
        Ready::init().await?; 
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        emb_service.recommend_similar(based_on, exclude, limit).await
    }

    pub async fn remove_item(id: &str) -> Result<Option<QueueItem>> {
        Ready::init().await?; 
        let items = crate::storage::list_queue_items()?;
        if let Some(item) = items.iter().find(|i| i.id == id) {
            Ok(Some(item.clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_key(user_id: &str) -> Result<Option<ApiKey>> {
        Ready::init().await?; 
        let collection = crate::api::load_api_key_collection()?;
        Ok(collection.keys.get(user_id).cloned())
    }

    pub async fn remove_task(id: &str) -> Result<Option<Task>> {
        Ready::init().await?; 
        let task = Self::get_task(id).await?;
        if task.is_some() {
            Self::delete_task(id).await?;
        }
        Ok(task)
    }

    #[cfg(feature = "tui")]
    pub async fn render(config: &DisplayConfig) -> Result<String> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let _app = TodoziApp::new(emb_service, config.clone());
        let output = format!("{} tasks loaded", tasks.len());
        Ok(output)
    }

    #[cfg(not(feature = "tui"))]
    pub async fn render(_config: &DisplayConfig) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: "TUI feature not enabled".to_string(),
        })
    }

    #[cfg(feature = "tui")]
    pub async fn render_compact(config: &DisplayConfig) -> Result<String> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let _app = TodoziApp::new(emb_service, config.clone());
        let output = format!("{} tasks loaded (compact)", tasks.len());
        Ok(output)
    }

    #[cfg(not(feature = "tui"))]
    pub async fn render_compact(_config: &DisplayConfig) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: "TUI feature not enabled".to_string(),
        })
    }

    #[cfg(feature = "tui")]
    pub async fn render_detailed(config: &DisplayConfig) -> Result<String> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let tasks = storage.list_tasks_across_projects(&TaskFilters::default())?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let _app = TodoziApp::new(emb_service, config.clone());
        let output = format!("{} tasks loaded (detailed)", tasks.len());
        Ok(output)
    }

    #[cfg(not(feature = "tui"))]
    pub async fn render_detailed(_config: &DisplayConfig) -> Result<String> {
        Err(TodoziError::ValidationError {
            message: "TUI feature not enabled".to_string(),
        })
    }

    pub async fn resolve_error(error_id: &str, resolution: &str) -> Result<()> {
        Ready::init().await?; 
        let mut error_manager = crate::error::ErrorManager::new();
        error_manager.resolve_error(error_id, resolution.to_string()).await
    }

    pub async fn restore_backup(backup_name: &str) -> Result<()> {
        Ready::init().await?; 
        Err(TodoziError::ValidationError {
            message: format!("Backup restore not yet implemented: {}", backup_name),
        })
    }

    pub async fn restore_embeddings(backup_path: &str) -> Result<usize> {
        Ready::init().await?; 
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        emb_service.restore_embeddings(backup_path.to_string()).await
    }

    #[cfg(feature = "tui")]
    pub async fn run() -> Result<()> {
        Ready::init().await?; 
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let config = DisplayConfig::default();
        let app = TodoziApp::new(emb_service, config);
        app.run().map_err(|e| TodoziError::ValidationError {
            message: format!("TUI error: {}", e),
        })
    }

    #[cfg(not(feature = "tui"))]
    pub async fn run() -> Result<()> {
        Err(TodoziError::ValidationError {
            message: "TUI feature not enabled".to_string(),
        })
    }

    #[cfg(feature = "tui")]
    pub async fn run_interactive() -> Result<String> {
        Ready::init().await?; 
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let config = DisplayConfig::default();
        let _app = TodoziApp::new(emb_service, config);
        let mut editor = crate::tui::TaskEditor::default();
        editor.run_interactive().await.map_err(|e| TodoziError::ValidationError {
            message: format!("Interactive error: {}", e),
        })
    }

    #[cfg(not(feature = "tui"))]
    pub async fn run_interactive() -> Result<String> {
        Err(TodoziError::ValidationError {
            message: "TUI feature not enabled".to_string(),
        })
    }

    pub async fn sample_task_async() -> Result<Task> {
        Ok(Self::sample_task())
    }

    pub async fn save_agent(agent: &Agent) -> Result<()> {
        Ready::init().await?; 
        crate::storage::save_agent(agent)
    }

    pub async fn save_agent_assignment(assignment: &AgentAssignment) -> Result<()> {
        Ready::init().await?; 
        crate::storage::save_agent_assignment(assignment)
    }

    pub async fn save_api_key_collection(collection: &ApiKeyCollection) -> Result<()> {
        Ready::init().await?; 
        crate::api::save_api_key_collection(collection)?;
        Ok(())
    }

    pub async fn save_as_default(model_name: &str) -> Result<()> {
        Ready::init().await?; 
        let config = load_config().await?;
        save_config(&config).await?;
        Ok(())
    }

    pub async fn save_code_chunk(chunk: &crate::chunking::CodeChunk) -> Result<()> {
        Ready::init().await?; 
        crate::storage::save_code_chunk(chunk)
    }

    pub async fn save_config(config: &Config) -> Result<()> {
        Ready::init().await?; 
        save_config(config).await?;
        Ok(())
    }

    pub async fn save_error(error: &Error) -> Result<()> {
        Ready::init().await?; 
        crate::storage::save_error(error)
    }

    pub async fn save_feeling(feeling: &Feeling) -> Result<()> {
        Ready::init().await?; 
        crate::storage::save_feeling(feeling)?;
        Ok(())
    }

    pub async fn save_idea(idea: &Idea) -> Result<()> {
        Ready::init().await?; 
        crate::storage::save_idea(idea)
    }

    pub async fn remind_at(task_id: &str, when: chrono::DateTime<chrono::Utc>) -> Result<()> {
        Ready::init().await?; 
        let reminder = Reminder::new(
            format!("Reminder for task {}", task_id),
            when,
            ReminderPriority::Medium,
        );
        let mut reminder_manager = crate::reminder::ReminderManager::new();
        reminder_manager.create_reminder(reminder).await?;
        Ok(())
    }

    /// Quick task creation - just pass a string
    pub async fn task(action: &str) -> Result<String> {
        let task = Self::create_task(action, None, None, None, None).await?;
        Ok(task.id)
    }
    /// Quick task with priority
    pub async fn urgent(action: &str) -> Result<String> {
        let task = Self::create_task(action, Some(Priority::Urgent), None, None, None).await?;
        Ok(task.id)
    }
    /// Quick high priority task
    pub async fn high(action: &str) -> Result<String> {
        let task = Self::create_task(action, Some(Priority::High), None, None, None).await?;
        Ok(task.id)
    }
    /// Quick low priority task
    pub async fn low(action: &str) -> Result<String> {
        let task = Self::create_task(action, Some(Priority::Low), None, None, None).await?;
        Ok(task.id)
    }
    /// Find tasks by text
    pub async fn find(query: &str) -> Result<Vec<Task>> {
        Self::find_tasks(query).await
    }
    /// Find tasks with AI
    pub async fn ai_find(query: &str) -> Result<Vec<Task>> {
        Self::find_tasks_ai(query).await
    }
    /// Complete a task
    pub async fn done(task_id: &str) -> Result<()> {
        Self::complete_task(task_id).await
    }
    /// Start working on a task
    pub async fn start(task_id: &str) -> Result<()> {
        Self::start_task(task_id).await
    }
    /// Get all tasks
    pub async fn all() -> Result<Vec<Task>> {
        Self::all_tasks().await
    }
    
    /// Save an idea
    pub async fn idea(idea: &str) -> Result<Task> {
        Self::ideate(idea).await
    }
    

    /// Create a task that needs AI processing
    pub async fn ai(action: &str) -> Result<String> {
        let task = Task {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "actions_user".to_string(),
            action: action.to_string(),
            time: "ASAP".to_string(),
            priority: Priority::Medium,
            parent_project: format!("{}_ai", Self::project_name()),
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
            parent_project: format!("{}_human", Self::project_name()),
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
            parent_project: format!("{}_collaborative", Self::project_name()),
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
        Self::complete_task(task_id).await
    }
    /// Start working on task
    pub async fn begin(task_id: &str) -> Result<()> {
        Self::start_task(task_id).await
    }
    /// Delete a task
    pub async fn delete(task_id: &str) -> Result<()> {
        Self::delete_task(task_id).await
    }
    /// Get task details
    pub async fn get(task_id: &str) -> Result<Option<Task>> {
        Self::get_task(task_id).await
    }
    /// List all tasks
    pub async fn list() -> Result<Vec<Task>> {
        Self::all_tasks().await
    }
    /// Add a recent action to the processor state
    pub async fn add_recent(description: &str) -> Result<()> {
        use crate::tdz_tls::{ProcessedAction, TodoziProcessorState};
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

    /// Create a new tag
    pub async fn create_tag(name: &str, description: Option<&str>) -> Result<String> {
        Ready::init().await?;
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
    pub async fn find_tag(tag_name: &str) -> Result<Vec<Task>> {
        let filters = TaskFilters {
            tags: Some(vec![tag_name.to_string()]),
            ..Default::default()
        };
        Self::search_with_filters(filters, None).await
    }
    pub async fn add_to_task(task_id: &str, tag: &str) -> Result<()> {
        if let Some(mut task) = Self::get_task(task_id).await? {
            if !task.tags.contains(&tag.to_string()) {
                task.tags.push(tag.to_string());
                let updates = TaskUpdate {
                    tags: Some(task.tags),
                    ..Default::default()
                };
                Self::update_task_full(task_id, updates).await?;
            }
        }
        Ok(())
    }
    pub async fn remove_from_task(task_id: &str, tag: &str) -> Result<()> {
        if let Some(mut task) = Self::get_task(task_id).await? {
            task.tags.retain(|t| t != tag);
            let updates = TaskUpdate {
                tags: Some(task.tags),
                ..Default::default()
            };
            Self::update_task_full(task_id, updates).await?;
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
        Ok(search_engine
            .advanced_search(search_query)
            .into_iter()
            .cloned()
            .collect())
    }

    pub async fn create_project(name: &str, description: Option<&str>) -> Result<()> {
        Ready::init().await?;
        let storage = Storage::new().await?;
        storage.create_project(name.to_string(), description.map(|s| s.to_string()))?;
        Ok(())
    }
    pub async fn tasks(project_name: &str) -> Result<Vec<Task>> {
        let filters = TaskFilters {
            project: Some(project_name.to_string()),
            ..Default::default()
        };
        Self::search_with_filters(filters, None).await
    }
    pub async fn delete_project(project_name: &str) -> Result<()> {
        Ready::init().await?;
        let storage = Storage::new().await?;
        storage.delete_project(project_name)?;
        Ok(())
    }

    pub async fn important(moment: &str, meaning: &str, reason: &str) -> Result<String> {
        Ready::init().await?;
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
    pub async fn find_memory(query: &str) -> Result<Vec<Memory>> {
        let memories = Self::list_memories().await?;
        Ok(memories
            .into_iter()
            .filter(|m| {
                m.moment.to_lowercase().contains(&query.to_lowercase())
                    || m.meaning.to_lowercase().contains(&query.to_lowercase())
            })
            .collect())
    }

    pub async fn breakthrough(idea: &str) -> Result<String> {
        Ready::init().await?;
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
    pub async fn find_idea(query: &str) -> Result<Vec<Idea>> {
        let ideas = Self::list_ideas().await?;
        Ok(ideas
            .into_iter()
            .filter(|i| i.idea.to_lowercase().contains(&query.to_lowercase()))
            .collect())
    }

    pub async fn add_to_queue(task_name: &str, description: &str) -> Result<String> {
        Ready::init().await?;
        let item = QueueItem::new(
            task_name.to_string(),
            description.to_string(),
            Priority::Medium,
            None,
        );
        crate::storage::add_queue_item(item.clone())?;
        Ok(item.id)
    }
    pub async fn list_queue_items() -> Result<Vec<QueueItem>> {
        Ready::init().await?;
        crate::storage::list_queue_items()
    }
    pub async fn list_queue_items_by_status(status: QueueStatus) -> Result<Vec<QueueItem>> {
        Ready::init().await?;
        crate::storage::list_queue_items_by_status(status)
    }
    pub async fn backlog() -> Result<Vec<QueueItem>> {
        Ready::init().await?;
        crate::storage::list_backlog_items()
    }
    pub async fn active() -> Result<Vec<QueueItem>> {
        Ready::init().await?;
        crate::storage::list_active_items()
    }

    pub async fn tdz_find(query: &str) -> Result<String> {
        Ready::init().await?;
        let mut results = Vec::new();
        if let Ok(ai_results) = Self::ai_search(query).await {
            if !ai_results.is_empty() {
                results.push("🤖 AI SEMANTIC SEARCH:".to_string());
                for result in ai_results.iter().take(5) {
                    results.push(format!(
                        "  • {} [Task] (similarity: {:.2})",
                        result.text_content, result.similarity_score
                    ));
                }
                if ai_results.len() > 5 {
                    results.push(format!(
                        "  ... and {} more AI matches",
                        ai_results.len() - 5
                    ));
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
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let results = emb_service.semantic_search(query, None, Some(20)).await?;
        Ok(results)
    }
    pub async fn keyword_search(query: &str) -> Result<String> {
        Ready::init().await?;
        let mut results = Vec::new();
        if let Ok(tasks) = Self::find_tasks(query).await {
            if !tasks.is_empty() {
                results.push(format!("📋 TASKS ({}):", tasks.len()));
                for task in tasks.iter().take(5) {
                    results.push(format!(
                        "  • {} [{}] [{}]",
                        task.action, task.status, task.priority
                    ));
                }
                if tasks.len() > 5 {
                    results.push(format!("  ... and {} more", tasks.len() - 5));
                }
                results.push("".to_string());
            }
        }
        if let Ok(memories) = Self::find_memory(query).await {
            if !memories.is_empty() {
                results.push(format!("🧠 MEMORIES ({}):", memories.len()));
                for memory in memories.iter().take(3) {
                    results.push(format!("  • {} - {}", memory.moment, memory.meaning));
                }
                if memories.len() > 3 {
                    results.push(format!("  ... and {} more", memories.len() - 3));
                }
                results.push("".to_string());
            }
        }
        if let Ok(ideas) = Self::find_idea(query).await {
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
        if let Ok(queue_items) = Self::list_queue_items().await {
            let filtered: Vec<_> = queue_items
                .into_iter()
                .filter(|q| {
                    q.task_name.to_lowercase().contains(&query.to_lowercase())
                        || q.task_description
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
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        emb_service
            .semantic_search(query, Some(vec![TodoziContentType::Task]), Some(10))
            .await
    }
    pub async fn keyword_tasks(query: &str) -> Result<Vec<Task>> {
        Self::find_tasks(query).await
    }
    pub async fn similar_tasks(task_id: &str) -> Result<Vec<SimilarityResult>> {
        Ready::init().await?;
        let task = Self::get_task(task_id).await?;
        if let Some(task) = task {
            let mut emb_service =
                TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
            emb_service.initialize().await?;
            emb_service.find_similar_tasks(&task.action, Some(10)).await
        } else {
            Ok(Vec::new())
        }
    }
    pub async fn smart(query: &str) -> Result<String> {
        Ready::init().await?;
        let query_lower = query.to_lowercase();
        if query_lower.contains("task")
            || query_lower.contains("todo")
            || query_lower.contains("do")
        {
            let tasks = Self::ai_tasks(query).await?;
            if !tasks.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - TASKS FOCUS:\n\n");
                for task in tasks.iter().take(7) {
                    result.push_str(&format!(
                        "  • {} (similarity: {:.2})\n",
                        task.text_content, task.similarity_score
                    ));
                }
                return Ok(result);
            }
        }
        if query_lower.contains("remember")
            || query_lower.contains("memory")
            || query_lower.contains("recall")
        {
            let memories = Self::find_memory(query).await?;
            if !memories.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - MEMORIES FOCUS:\n\n");
                for memory in memories.iter().take(5) {
                    result.push_str(&format!("  • {} - {}\n", memory.moment, memory.meaning));
                }
                return Ok(result);
            }
        }
        if query_lower.contains("idea")
            || query_lower.contains("concept")
            || query_lower.contains("innovation")
        {
            let ideas = Self::find_idea(query).await?;
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

    pub async fn embed(text: &str) -> Result<Vec<f32>> {
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        emb_service.generate_embedding(text).await
    }
    pub async fn find_similar(query: &str) -> Result<Vec<SimilarityResult>> {
        Self::ai_search(query).await
    }
    pub async fn find_ai_tasks(query: &str) -> Result<Vec<SimilarityResult>> {
        Self::ai_tasks(query).await
    }
    pub async fn cluster_content() -> Result<Vec<ClusteringResult>> {
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        emb_service.cluster_content().await
    }
    pub async fn stats() -> Result<String> {
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let stats = emb_service.get_stats().await?;
        Ok(format!(
            "🧠 EMBEDDING STATS:\n{}",
            serde_json::to_string_pretty(&stats)?
        ))
    }
    pub async fn embed_task(task_id: &str) -> Result<String> {
        Ready::init().await?;
        if let Some(task) = Self::get_task(task_id).await? {
            let mut emb_service =
                TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
            emb_service.initialize().await?;
            let content = emb_service.prepare_task_content(&task);
            let embedding = emb_service.generate_embedding(&content).await?;
            Ok(format!(
                "Task '{}' embedded successfully ({} dimensions)",
                task.action,
                embedding.len()
            ))
        } else {
            Err(TodoziError::TaskNotFound {
                id: task_id.to_string(),
            })
        }
    }

    pub async fn quick() -> Result<String> {
        Ready::init().await?;
        let tasks = Self::all_tasks().await?;
        let total = tasks.len();
        let done = tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Done))
            .count();
        let in_progress = tasks
            .iter()
            .filter(|t| matches!(t.status, Status::InProgress))
            .count();
        let blocked = tasks
            .iter()
            .filter(|t| matches!(t.status, Status::Blocked))
            .count();
        let memories = Self::list_memories().await?.len();
        let ideas = Self::list_ideas().await?.len();
        let queue_items = Self::list_queue_items().await?.len();
        Ok(
            format!(
                "📊 TODOZI STATS\n\n📋 Tasks: {} total\n  ✅ Done: {}\n  🔄 In Progress: {}\n  🚫 Blocked: {}\n\n💡 Ideas: {}\n🧠 Memories: {}\n📋 Queue: {}",
                total, done, in_progress, blocked, ideas, memories, queue_items
            ),
        )
    }
    pub async fn detailed() -> Result<String> {
        let quick_stats = Self::quick().await?;
        let tasks = Self::all_tasks().await?;
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
        let projects = Self::list_projects().await?;
        let project_stats = format!("\n📁 Projects: {} total", projects.len());
        Ok(format!("{}{}{}", quick_stats, by_priority, project_stats))
    }


    pub async fn add_task_to_project(task: Task) -> Result<()> {
        let storage = Storage::new().await?;
        storage.add_task_to_project(task).await
    }

    pub async fn add_item(content: &str) -> Result<()> {
        use crate::tdz_tls::{ChecklistItem, TodoziProcessorState};
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

    pub async fn add_chunk(chunk_id: String, level: String, deps: Vec<String>) -> Result<()> {
        use crate::chunking::{ChunkingLevel, CodeGenerationGraph};
        let mut graph = CodeGenerationGraph::new(1000); // Default max lines
        let chunk_level = level
            .parse::<ChunkingLevel>()
            .unwrap_or(ChunkingLevel::Method);
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

    pub async fn do_it(what: &str, action: &str, priority: Option<Priority>,project: Option<&str>,time: Option<&str>,context: Option<&str>) -> Result<String> {
        let task = Self::create_task(action, priority, project, time, context).await?;
        Ok(task.id)
    }
    pub async fn find_task(what: &str) -> Result<String> {
        let tasks = Self::find_tasks(what).await?;
        if tasks.is_empty() {
            Ok(format!("No tasks found for: {}", what))
        } else {
            Ok(format!("Found {} task(s)", tasks.len()))
        }
    }
    pub async fn see_all() -> Result<String> {
        Self::quick().await
    }

    pub async fn has_results() -> Result<bool> {
        Ok(false)
    }

    pub async fn has_specialization(specialization: &str) -> Result<bool> {
        use crate::agent::AgentManager;
        let manager = AgentManager::new();
        let agents = manager.get_all_agents();
        Ok(agents.iter().any(|a| a.has_specialization(specialization)))
    }

    pub async fn has_tool(tool_name: &str) -> Result<bool> {
        use crate::agent::AgentManager;
        let manager = AgentManager::new();
        let agents = manager.get_all_agents();
        Ok(agents.iter().any(|a| a.has_tool(tool_name)))
    }

    pub async fn hash_project_name(project_name: &str) -> Result<String> {
        models::hash_project_name(project_name)
    }

    pub async fn hierarchical_clustering(
        content_types: Vec<TodoziContentType>,
        max_depth: usize,
    ) -> Result<Vec<emb::HierarchicalCluster>> {
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        emb_service.hierarchical_clustering(content_types, max_depth).await
    }

    pub async fn high_priority_percentage() -> Result<f64> {
        let tasks = Self::all_tasks().await?;
        let high_priority = tasks.iter().filter(|t| t.priority == Priority::High || t.priority == Priority::Critical).count();
        Ok(if tasks.is_empty() {
            0.0
        } else {
            high_priority as f64 / tasks.len() as f64
        })
    }

    pub async fn hybrid_search(
        query: &str,
        keywords: Vec<String>,
        content_types: Option<&[TodoziContentType]>,
        semantic_weight: f32,
        limit: usize,
    ) -> Result<Vec<SimilarityResult>> {
        Ready::init().await?;
        let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?;
        emb_service.initialize().await?;
        let content_types_vec = content_types.map(|s| s.to_vec());
        emb_service.hybrid_search(query, keywords, content_types_vec, semantic_weight, limit).await
    }

    pub async fn importance(_importance: IdeaImportance) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: "importance() should be called on Idea builder, not Easy".to_string(),
        })
    }
    pub async fn initialize_grok_level_todozi_system() -> Result<std::result::Result<crate::toolbox::todozi_tool::SharedTodozi, TodoziError>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let shared: crate::toolbox::todozi_tool::SharedTodozi = std::sync::Arc::new(tokio::sync::Mutex::new(storage));
        Ok(Ok(shared))
    }

    pub async fn initialize_grok_level_todozi_system_with_embedding(
        enable_embeddings: bool,
    ) -> Result<std::result::Result<(crate::toolbox::todozi_tool::SharedTodozi, Option<TodoziEmbeddingService>), TodoziError>> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let shared: crate::toolbox::todozi_tool::SharedTodozi = std::sync::Arc::new(tokio::sync::Mutex::new(storage));
        let embedding = if enable_embeddings {
            Some(TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await?)
        } else {
            None
        };
        Ok(Ok((shared, embedding)))
    }

    pub async fn initialize_tdz_content_processor() -> Result<SharedTodoziState> {
        crate::tdz_tls::initialize_tdz_content_processor().await
    }

    pub async fn interactive_create_task() -> Result<Task> {
        Err(TodoziError::ValidationError {
            message: "interactive_create_task() requires interactive terminal, use create_task() instead".to_string(),
        })
    }

    pub async fn io(message: String) -> Result<()> {
        Err(TodoziError::ValidationError {
            message: format!("io() should be called on Error builder, not Done. Message: {}", message),
        })
    }

    pub async fn is_active() -> Result<bool> {
        Ok(true)
    }

    pub async fn is_admin(public_key: &str, private_key: &str) -> Result<bool> {
        use crate::models::ApiKey;
        let key = ApiKey::new();
        Ok(key.is_admin(public_key, private_key))
    }

    pub async fn is_available() -> Result<bool> {
        Ok(true)
    }

    pub async fn is_backlog() -> Result<bool> {
        Ok(false)
    }

    pub async fn is_complete() -> Result<bool> {
        Ok(false)
    }

    pub async fn is_completed() -> Result<bool> {
        Ok(false)
    }

    pub async fn is_empty() -> Result<bool> {
        let tasks = Self::all_tasks().await?;
        Ok(tasks.is_empty())
    }

    pub async fn is_overdue() -> Result<bool> {
        Ok(false)
    }
    pub async fn smart_search(query: &str) -> Result<String> {
        Ready::init().await?;
        let query_lower = query.to_lowercase();
        if query_lower.contains("task")
            || query_lower.contains("todo")
            || query_lower.contains("do")
        {
            let tasks = Self::ai_tasks(query).await?;
            if !tasks.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - TASKS FOCUS:\n\n");
                for task in tasks.iter().take(7) {
                    result.push_str(&format!(
                        "  • {} (similarity: {:.2})\n",
                        task.text_content, task.similarity_score
                    ));
                }
                return Ok(result);
            }
        }
        if query_lower.contains("remember")
            || query_lower.contains("memory")
            || query_lower.contains("recall")
        {
            let memories = Self::find_memory(query).await?;
            if !memories.is_empty() {
                let mut result = format!("🎯 SMART SEARCH - MEMORIES FOCUS:\n\n");
                for memory in memories.iter().take(5) {
                    result.push_str(&format!("  • {} - {} - {}\n", memory.created_at, memory.moment, memory.meaning));
                }
                return Ok(result);
            }
        }
        if query_lower.contains("idea")
            || query_lower.contains("concept")
            || query_lower.contains("innovation")
        {
            let ideas = Self::find_idea(query).await?;
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

    pub async fn handle_memory_command(command: types::MemoryCommands) -> Result<()> {
        Ready::init().await?;
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_memory_command(command).await
    }

    pub async fn handle_project_command(command: types::ProjectCommands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_project_command(command).await
    }

    pub async fn handle_queue_command(command: types::QueueCommands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_queue_command(command).await
    }

    pub async fn handle_search_all_command(command: types::Commands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_search_all_command(command).await
    }

    pub async fn handle_search_command(command: types::SearchCommands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_search_command(command).await
    }

    pub async fn handle_server_command(command: types::ServerCommands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_server_command(command).await
    }

    pub async fn handle_show_command(command: types::ShowCommands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_show_command(command).await
    }

    pub async fn handle_stats_command(command: types::StatsCommands) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_stats_command(command).await
    }

    pub async fn handle_strategy_command(
        content: Option<&str>,
        file: Option<&str>,
        output_format: &str,
        human: bool,
    ) -> Result<()> {
        Ready::init().await?;
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_strategy_command(content.map(|s| s.to_string()), file.map(|s| s.to_string()), output_format.to_string(), human).await
    }

    pub async fn handle_train_command(command: types::TrainingCommands) -> Result<()> {
        Ready::init().await?;
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_train_command(command).await
    }

    pub async fn handle_update_command(
        id: &str,
        action: Option<&str>,
        time: Option<&str>,
        priority: Option<&str>,
        project: Option<&str>,
        status: Option<&str>,
        assignee: Option<&str>,
        tags: Option<&str>,
        dependencies: Option<&str>,
        context: Option<&str>,
        progress: Option<&u8>,
    ) -> Result<()> {
        Ready::init().await?; 
        let storage = Storage::new().await?;
        let handler = cli::TodoziHandler::new(storage);
        handler.handle_update_command(
            id.to_string(),
            action.map(|s| s.to_string()),
            time.map(|s| s.to_string()),
            priority.map(|s| s.to_string()),
            project.map(|s| s.to_string()),
            status.map(|s| s.to_string()),
            assignee.map(|s| s.to_string()),
            tags.map(|s| s.to_string()),
            dependencies.map(|s| s.to_string()),
            context.map(|s| s.to_string()),
            progress.copied(),
        ).await
    }

    pub async fn has_capability(capability: &str) -> Result<bool> {
        use crate::agent::AgentManager;
        let manager = AgentManager::new();
        let agents = manager.get_all_agents();
        Ok(agents.iter().any(|a| a.capabilities.contains(&capability.to_string())))
    }
}
