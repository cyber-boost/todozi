use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;
use dirs::home_dir;
use helix::{DnaValue as Value, Hlx};
use reqwest::Client;
use serde_json;
use crate::chunking::CodeChunk;
use crate::error::{Result, TodoziError};
use crate::models::{
    Agent, AgentAssignment, AgentBehaviors, AgentStatus, AgentTool, AssignmentStatus,
    Config, Error, Feeling, Idea, Memory, MigrationReport, Project,
    ProjectMigrationStats, ProjectStats, ProjectTaskContainer, QueueCollection,
    QueueItem, QueueStatus, RegistrationInfo, SemanticSearchResult, Task, TaskCollection,
    TaskFilters, TrainingData,
};
pub fn get_storage_dir() -> Result<PathBuf> {
    let home = home_dir()
        .ok_or_else(|| TodoziError::storage("Could not find home directory"))?;
    Ok(home.join(".todozi"))
}
pub fn get_tasks_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("tasks"))
}
pub async fn init_storage() -> Result<()> {
    let storage_dir = get_storage_dir()?;
    fs::create_dir_all(&storage_dir)?;
    fs::create_dir_all(storage_dir.join("tasks"))?;
    fs::create_dir_all(storage_dir.join("projects"))?;
    fs::create_dir_all(storage_dir.join("templates"))?;
    fs::create_dir_all(storage_dir.join("backups"))?;
    fs::create_dir_all(storage_dir.join("agents"))?;
    fs::create_dir_all(storage_dir.join("memories"))?;
    fs::create_dir_all(storage_dir.join("ideas"))?;
    fs::create_dir_all(storage_dir.join("training"))?;
    fs::create_dir_all(storage_dir.join("chunks"))?;
    fs::create_dir_all(storage_dir.join("errors"))?;
    fs::create_dir_all(storage_dir.join("assignments"))?;
    fs::create_dir_all(storage_dir.join("feelings"))?;
    fs::create_dir_all(storage_dir.join("queue"))?;
    fs::create_dir_all(storage_dir.join("api"))?;
    fs::create_dir_all(storage_dir.join("models"))?;
    fs::create_dir_all(storage_dir.join("responses"))?;
    fs::create_dir_all(storage_dir.join("embed"))?;
    let config_path = storage_dir.join("tdz.hlx");
    let is_new_config = !config_path.exists();
    if !config_path.exists() {
        let config = Config::default();
        save_config(&config).await?;
    }
    if is_new_config || !is_registered().await.unwrap_or(false) {
        let registration = RegistrationInfo::new_with_hashes(
            "https://todozi.com".to_string(),
        );
        if let Err(e) = update_config_with_registration(registration).await {
            println!("⚠️  Could not save registration info: {}", e);
        } else {
            println!("🔗 Created registration info (ready for todozi.com)");
            println!("💡 Run 'todozi register' to complete registration with server");
        }
    }
    create_default_agents()?;
    let project_path = storage_dir.join("projects").join("general.json");
    if !project_path.exists() {
        let project = Project::new(
            "general".to_string(),
            Some("General tasks".to_string()),
        );
        save_project(&project)?;
    }
    let active_path = storage_dir.join("tasks").join("active.json");
    if !active_path.exists() {
        let collection = TaskCollection::new();
        save_task_collection("active", &collection)?;
    }
    let completed_path = storage_dir.join("tasks").join("completed.json");
    if !completed_path.exists() {
        let collection = TaskCollection::new();
        save_task_collection("completed", &collection)?;
    }
    let archived_path = storage_dir.join("tasks").join("archived.json");
    if !archived_path.exists() {
        let collection = TaskCollection::new();
        save_task_collection("archived", &collection)?;
    }
    Ok(())
}
pub fn check_folder_structure() -> Result<bool> {
    let storage_dir = get_storage_dir()?;
    let required_dirs = [
        "agents",
        "api",
        "assignments",
        "backups",
        "chunks",
        "embed",
        "errors",
        "feelings",
        "ideas",
        "memories",
        "models",
        "projects",
        "queue",
        "responses",
        "tasks",
        "templates",
        "training",
    ];
    for dir_name in &required_dirs {
        let dir_path = storage_dir.join(dir_name);
        if !dir_path.exists() {
            println!("❌ Missing directory: {}", dir_name);
            return Ok(false);
        }
        if !dir_path.is_dir() {
            println!("❌ {} exists but is not a directory", dir_name);
            return Ok(false);
        }
    }
    let config_path = storage_dir.join("tdz.hlx");
    if !config_path.exists() {
        println!("❌ Missing tdz.hlx configuration file");
        return Ok(false);
    }
    if !config_path.is_file() {
        println!("❌ tdz.hlx exists but is not a file");
        return Ok(false);
    }
    println!("✅ Todozi folder structure is complete!");
    println!("📁 Storage directory: {}", storage_dir.display());
    println!("📂 Found {} required directories", required_dirs.len());
    for dir_name in &required_dirs {
        println!("  ✓ {}", dir_name);
    }
    println!("  ✓ tdz.hlx");
    Ok(true)
}
pub async fn ensure_folder_structure() -> Result<bool> {
    let _storage_dir = get_storage_dir()?;
    if check_folder_structure()? {
        return Ok(true);
    }
    println!("🔧 Creating missing folder structure...");
    init_storage().await?;
    check_folder_structure()?;
    Ok(true)
}
pub async fn save_config(config: &Config) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let config_path = storage_dir.join("tdz.hlx");
    let mut hlx = Hlx::new().await?;
    if let Some(registration) = &config.registration {
        hlx.set(
            "registration",
            "user_name",
            Value::String(registration.user_name.clone()),
        );
        hlx.set(
            "registration",
            "user_email",
            Value::String(registration.user_email.clone()),
        );
        hlx.set("registration", "api_key", Value::String(registration.api_key.clone()));
        if let Some(user_id) = &registration.user_id {
            hlx.set("registration", "user_id", Value::String(user_id.clone()));
        }
        if let Some(fingerprint) = &registration.fingerprint {
            hlx.set("registration", "fingerprint", Value::String(fingerprint.clone()));
        }
        hlx.set(
            "registration",
            "registered_at",
            Value::String(registration.registered_at.to_rfc3339()),
        );
        hlx.set(
            "registration",
            "server_url",
            Value::String(registration.server_url.clone()),
        );
    }
    hlx.set("config", "version", Value::String(config.version.clone()));
    hlx.set("config", "default_project", Value::String(config.default_project.clone()));
    hlx.set("config", "auto_backup", Value::Bool(config.auto_backup));
    hlx.set("config", "backup_interval", Value::String(config.backup_interval.clone()));
    hlx.set("config", "ai_enabled", Value::Bool(config.ai_enabled));
    if let Some(assignee) = &config.default_assignee {
        hlx.set("config", "default_assignee", Value::String(assignee.to_string()));
    }
    hlx.set("config", "date_format", Value::String(config.date_format.clone()));
    hlx.set("config", "timezone", Value::String(config.timezone.clone()));
    hlx.file_path = Some(config_path);
    hlx.save()?;
    Ok(())
}
pub async fn load_config() -> Result<Config> {
    let storage_dir = get_storage_dir()?;
    let config_path = storage_dir.join("tdz.hlx");
    if !config_path.exists() {
        return Ok(Config::default());
    }
    let hlx = Hlx::load(&*config_path.to_string_lossy()).await?;
    let registration = if let (
        Some(Value::String(user_name)),
        Some(Value::String(user_email)),
        Some(Value::String(api_key)),
    ) = (
        hlx.get("registration", "user_name"),
        hlx.get("registration", "user_email"),
        hlx.get("registration", "api_key"),
    ) {
        let user_id = hlx
            .get("registration", "user_id")
            .and_then(|v| {
                if let Value::String(s) = v { Some(s.clone()) } else { None }
            });
        let fingerprint = hlx
            .get("registration", "fingerprint")
            .and_then(|v| {
                if let Value::String(s) = v { Some(s.clone()) } else { None }
            });
        let registered_at = hlx
            .get("registration", "registered_at")
            .and_then(|v| {
                if let Value::String(s) = v {
                    chrono::DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| chrono::Utc::now());
        let server_url = hlx
            .get("registration", "server_url")
            .and_then(|v| {
                if let Value::String(s) = v { Some(s.clone()) } else { None }
            })
            .unwrap_or_else(|| "https://todozi.com".to_string());
        Some(RegistrationInfo {
            user_name: user_name.clone(),
            user_email: user_email.clone(),
            api_key: api_key.clone(),
            user_id,
            fingerprint,
            registered_at,
            server_url,
        })
    } else {
        None
    };
    let version = hlx
        .get("config", "version")
        .and_then(|v| { if let Value::String(s) = v { Some(s.clone()) } else { None } })
        .unwrap_or_else(|| "1.2.0".to_string());
    let default_project = hlx
        .get("config", "default_project")
        .and_then(|v| { if let Value::String(s) = v { Some(s.clone()) } else { None } })
        .unwrap_or_else(|| "general".to_string());
    let auto_backup = hlx
        .get("config", "auto_backup")
        .and_then(|v| { if let Value::Bool(b) = v { Some(*b) } else { None } })
        .unwrap_or(true);
    let backup_interval = hlx
        .get("config", "backup_interval")
        .and_then(|v| { if let Value::String(s) = v { Some(s.clone()) } else { None } })
        .unwrap_or_else(|| "daily".to_string());
    let ai_enabled = hlx
        .get("config", "ai_enabled")
        .and_then(|v| { if let Value::Bool(b) = v { Some(*b) } else { None } })
        .unwrap_or(true);
    let default_assignee = hlx
        .get("config", "default_assignee")
        .and_then(|v| { if let Value::String(s) = v { s.parse().ok() } else { None } });
    let date_format = hlx
        .get("config", "date_format")
        .and_then(|v| { if let Value::String(s) = v { Some(s.clone()) } else { None } })
        .unwrap_or_else(|| "%Y-%m-%d %H:%M:%S".to_string());
    let timezone = hlx
        .get("config", "timezone")
        .and_then(|v| { if let Value::String(s) = v { Some(s.clone()) } else { None } })
        .unwrap_or_else(|| "UTC".to_string());
    Ok(Config {
        registration,
        version,
        default_project,
        auto_backup,
        backup_interval,
        ai_enabled,
        default_assignee,
        date_format,
        timezone,
    })
}
pub async fn register_with_server(server_url: &str) -> Result<RegistrationInfo> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| TodoziError::storage(
            &format!("Failed to create HTTP client: {}", e),
        ))?;
    let registration = RegistrationInfo::new_with_hashes(server_url.to_string());
    let payload = serde_json::json!(
        { "user_name" : registration.user_name, "user_email" : registration.user_email }
    );
    match client
        .post(&format!("{}/api/todozi/register", server_url))
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let api_key = json["api_key"]
                            .as_str()
                            .unwrap_or("no_key_provided")
                            .to_string();
                        let user_id = json["user_id"].as_str().map(|s| s.to_string());
                        let fingerprint = json["fingerprint"]
                            .as_str()
                            .map(|s| s.to_string());
                        let mut registered_info = registration;
                        registered_info.api_key = api_key.clone();
                        registered_info.user_id = user_id.clone();
                        registered_info.fingerprint = fingerprint.clone();
                        println!("✅ Successfully registered with todozi.com!");
                        println!("🔑 API Key: {}", registered_info.api_key);
                        if let Some(ref uid) = registered_info.user_id {
                            println!("👤 User ID: {}", uid);
                        }
                        if let Some(ref fp) = registered_info.fingerprint {
                            println!("🔐 Fingerprint: {}", fp);
                        }
                        if let Err(e) = update_config_with_registration(
                                registered_info.clone(),
                            )
                            .await
                        {
                            println!(
                                "⚠️  Could not update config with registration data: {}",
                                e
                            );
                        }
                        Ok(registered_info)
                    }
                    Err(e) => {
                        println!("⚠️  Could not parse server response: {}", e);
                        println!(
                            "✅ Registration completed (response key may be missing)"
                        );
                        Ok(registration)
                    }
                }
            } else {
                println!("❌ Registration failed: HTTP {}", response.status());
                println!(
                    "📄 Response: {:?}", response.text(). await .unwrap_or_default()
                );
                Err(TodoziError::storage("Registration failed"))
            }
        }
        Err(e) => {
            println!("❌ Network error during registration: {}", e);
            println!(
                "💡 Note: Registration is optional - todozi will work without server connection"
            );
            Err(TodoziError::storage(&format!("Network error: {}", e)))
        }
    }
}
pub async fn update_config_with_registration(
    registration: RegistrationInfo,
) -> Result<()> {
    let mut config = load_config().await?;
    config.registration = Some(registration);
    save_config(&config).await?;
    println!("💾 Updated tdz.hlx with registration information");
    Ok(())
}
pub async fn update_registration_api_key(api_key: String) -> Result<()> {
    let mut config = load_config().await?;
    if let Some(ref mut registration) = config.registration {
        registration.api_key = api_key;
        save_config(&config).await?;
        Ok(())
    } else {
        Err(TodoziError::storage("No registration info found"))
    }
}
pub async fn update_registration_keys(
    api_key: String,
    user_id: Option<String>,
    fingerprint: Option<String>,
) -> Result<()> {
    let mut config = load_config().await?;
    if let Some(ref mut registration) = config.registration {
        registration.api_key = api_key;
        registration.user_id = user_id;
        registration.fingerprint = fingerprint;
        save_config(&config).await?;
        println!("🔑 Updated tdz.hlx with all registration keys from server");
        Ok(())
    } else {
        Err(TodoziError::storage("No registration info found"))
    }
}
pub async fn is_registered() -> Result<bool> {
    let config = load_config().await?;
    Ok(config.registration.is_some())
}
pub async fn get_registration_info() -> Result<Option<RegistrationInfo>> {
    let config = load_config().await?;
    Ok(config.registration)
}
pub async fn clear_registration() -> Result<()> {
    let mut config = load_config().await?;
    config.registration = None;
    save_config(&config).await?;
    println!("🗑️  Cleared registration information from tdz.hlx");
    Ok(())
}
pub fn get_project_tasks_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("project_tasks"))
}
fn hash_project_name(project_name: &str) -> Result<String> {
    use md5;
    let digest = md5::compute(project_name.as_bytes());
    Ok(format!("{:x}", digest))
}
pub fn save_project_task_container(container: &ProjectTaskContainer) -> Result<()> {
    let project_tasks_dir = get_project_tasks_dir()?;
    fs::create_dir_all(&project_tasks_dir)?;
    let container_path = project_tasks_dir
        .join(format!("{}.json", container.project_hash));
    let json = serde_json::to_string_pretty(container)?;
    fs::write(container_path, json)?;
    Ok(())
}
pub fn load_project_task_container(project_name: &str) -> Result<ProjectTaskContainer> {
    let project_tasks_dir = get_project_tasks_dir()?;
    let project_hash = hash_project_name(project_name)?;
    let container_path = project_tasks_dir.join(format!("{}.json", project_hash));
    if !container_path.exists() {
        return Ok(ProjectTaskContainer::new(project_name));
    }
    let content = fs::read_to_string(container_path)?;
    let container: ProjectTaskContainer = serde_json::from_str(&content)?;
    Ok(container)
}
pub fn load_project_task_container_by_hash(
    project_hash: &str,
) -> Result<ProjectTaskContainer> {
    let project_tasks_dir = get_project_tasks_dir()?;
    let container_path = project_tasks_dir.join(format!("{}.json", project_hash));
    if !container_path.exists() {
        return Err(TodoziError::ProjectNotFound {
            name: format!("hash: {}", project_hash),
        });
    }
    let content = fs::read_to_string(container_path)?;
    let container: ProjectTaskContainer = serde_json::from_str(&content)?;
    Ok(container)
}
pub fn list_project_task_containers() -> Result<Vec<ProjectTaskContainer>> {
    let project_tasks_dir = get_project_tasks_dir()?;
    let mut containers = Vec::new();
    if project_tasks_dir.exists() {
        for entry in fs::read_dir(project_tasks_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(container) = serde_json::from_str::<
                        ProjectTaskContainer,
                    >(&content) {
                        containers.push(container);
                    }
                }
            }
        }
    }
    Ok(containers)
}
pub fn delete_project_task_container(project_name: &str) -> Result<()> {
    let project_tasks_dir = get_project_tasks_dir()?;
    let project_hash = hash_project_name(project_name)?;
    let container_path = project_tasks_dir.join(format!("{}.json", project_hash));
    if container_path.exists() {
        fs::remove_file(container_path)?;
    }
    Ok(())
}
pub fn save_task_collection(
    collection_name: &str,
    collection: &TaskCollection,
) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let collection_path = storage_dir
        .join("tasks")
        .join(format!("{}.json", collection_name));
    let json = serde_json::to_string_pretty(collection)?;
    fs::write(collection_path, json)?;
    Ok(())
}
pub fn load_task_collection(collection_name: &str) -> Result<TaskCollection> {
    let storage_dir = get_storage_dir()?;
    let collection_path = storage_dir
        .join("tasks")
        .join(format!("{}.json", collection_name));
    if !collection_path.exists() {
        return Ok(TaskCollection::new());
    }
    let content = fs::read_to_string(collection_path)?;
    let collection: TaskCollection = serde_json::from_str(&content)?;
    Ok(collection)
}
pub fn save_project(project: &Project) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let project_path = storage_dir
        .join("projects")
        .join(format!("{}.json", project.name));
    let json = serde_json::to_string_pretty(project)?;
    fs::write(project_path, json)?;
    Ok(())
}
pub fn load_project(project_name: &str) -> Result<Project> {
    let storage_dir = get_storage_dir()?;
    let project_path = storage_dir
        .join("projects")
        .join(format!("{}.json", project_name));
    if !project_path.exists() {
        return Err(TodoziError::ProjectNotFound {
            name: project_name.to_string(),
        });
    }
    let content = fs::read_to_string(project_path)?;
    let project: Project = serde_json::from_str(&content)?;
    Ok(project)
}
pub fn list_projects() -> Result<Vec<Project>> {
    let storage_dir = get_storage_dir()?;
    let projects_dir = storage_dir.join("projects");
    if !projects_dir.exists() {
        return Ok(Vec::new());
    }
    let mut projects = Vec::new();
    for entry in fs::read_dir(projects_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            let project: Project = serde_json::from_str(&content)?;
            projects.push(project);
        }
    }
    Ok(projects)
}
pub fn delete_project(project_name: &str) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let project_path = storage_dir
        .join("projects")
        .join(format!("{}.json", project_name));
    if project_path.exists() {
        fs::remove_file(project_path)?;
    }
    Ok(())
}
#[derive(Debug)]
pub struct Storage {
    config: Config,
}
impl Storage {
    pub async fn new() -> Result<Self> {
        let config = load_config().await?;
        Ok(Self { config })
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
    pub async fn update_config(&mut self, config: Config) -> Result<()> {
        save_config(&config).await?;
        self.config = config;
        Ok(())
    }
    #[deprecated(note = "Use add_task_to_project instead")]
    pub fn add_task(&self, task: Task) -> Result<()> {
        let mut collection = load_task_collection("active")?;
        collection.add_task(task);
        save_task_collection("active", &collection)?;
        Ok(())
    }
    #[deprecated(note = "Use get_task_from_any_project instead")]
    pub fn get_task(&self, id: &str) -> Result<Task> {
        if let Ok(collection) = load_task_collection("active") {
            if let Some(task) = collection.get_task(id) {
                return Ok(task.clone());
            }
        }
        if let Ok(collection) = load_task_collection("completed") {
            if let Some(task) = collection.get_task(id) {
                return Ok(task.clone());
            }
        }
        if let Ok(collection) = load_task_collection("archived") {
            if let Some(task) = collection.get_task(id) {
                return Ok(task.clone());
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    #[deprecated(note = "Use update_task_in_project instead")]
    pub fn update_task(
        &self,
        id: &str,
        updates: crate::models::TaskUpdate,
    ) -> Result<()> {
        let collections = ["active", "completed", "archived"];
        for collection_name in &collections {
            if let Ok(mut collection) = load_task_collection(collection_name) {
                if let Some(task) = collection.get_task_mut(id) {
                    task.update(updates)?;
                    save_task_collection(collection_name, &collection)?;
                    return Ok(());
                }
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    #[deprecated(note = "Use delete_task_from_project instead")]
    pub fn delete_task(&self, id: &str) -> Result<()> {
        let collections = ["active", "completed", "archived"];
        for collection_name in &collections {
            if let Ok(mut collection) = load_task_collection(collection_name) {
                if collection.remove_task(id).is_some() {
                    save_task_collection(collection_name, &collection)?;
                    return Ok(());
                }
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    #[deprecated(note = "Use list_tasks_across_projects instead")]
    pub fn list_tasks(&self, filters: &TaskFilters) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        let collections = ["active", "completed", "archived"];
        for collection_name in &collections {
            if let Ok(collection) = load_task_collection(collection_name) {
                let filtered_tasks = collection.get_filtered_tasks(filters);
                all_tasks.extend(filtered_tasks.into_iter().cloned());
            }
        }
        Ok(all_tasks)
    }
    #[deprecated(note = "Project-based system handles status changes automatically")]
    pub fn move_task(
        &self,
        id: &str,
        from_collection: &str,
        to_collection: &str,
    ) -> Result<()> {
        let mut from_col = load_task_collection(from_collection)?;
        let task = from_col
            .remove_task(id)
            .ok_or_else(|| TodoziError::TaskNotFound {
                id: id.to_string(),
            })?;
        save_task_collection(from_collection, &from_col)?;
        let mut to_col = load_task_collection(to_collection)?;
        to_col.add_task(task);
        save_task_collection(to_collection, &to_col)?;
        Ok(())
    }
    #[deprecated(note = "Use complete_task_in_project instead")]
    pub fn complete_task(&self, id: &str) -> Result<()> {
        // Project-based system handles status changes automatically
        self.complete_task_in_project(id)
    }
    pub async fn add_task_to_project(&self, mut task: Task) -> Result<()> {
        if task.parent_project.is_empty() {
            task.parent_project = self.config.default_project.clone();
        }
        
        
        if let Ok(emb_service) = crate::emb::TodoziEmbeddingService::new(
                crate::emb::TodoziEmbeddingConfig::default(),
            )
            .await
        {
            let embedding = emb_service
                .generate_embedding(&emb_service.prepare_task_content(&task))
                .await?;
            task.embedding_vector = Some(embedding);
        }
        
        let mut container = load_project_task_container(&task.parent_project)?;
        container.add_task(task);
        save_project_task_container(&container)?;
        Ok(())
    }
    pub fn get_task_from_any_project(&self, id: &str) -> Result<Task> {
        let containers = list_project_task_containers()?;
        for container in containers {
            if let Some(task) = container.get_task(id) {
                return Ok(task.clone());
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    pub fn get_task_from_project(
        &self,
        project_name: &str,
        task_id: &str,
    ) -> Result<Task> {
        let container = load_project_task_container(project_name)?;
        container
            .get_task(task_id)
            .cloned()
            .ok_or_else(|| TodoziError::TaskNotFound {
                id: task_id.to_string(),
            })
    }
    pub async fn update_task_in_project(
        &self,
        id: &str,
        updates: crate::models::TaskUpdate,
    ) -> Result<()> {
        let containers = list_project_task_containers()?;
        for mut container in containers {
            if let Some(task) = container.get_task_mut(id) {
                task.update(updates)?;
                if let Ok(emb_service) = crate::emb::TodoziEmbeddingService::new(
                        crate::emb::TodoziEmbeddingConfig::default(),
                    )
                    .await
                {
                    let embedding = emb_service
                        .generate_embedding(&emb_service.prepare_task_content(task))
                        .await?;
                    task.embedding_vector = Some(embedding);
                }
                
                save_project_task_container(&container)?;
                return Ok(());
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    pub fn delete_task_from_project(&self, id: &str) -> Result<()> {
        let containers = list_project_task_containers()?;
        for mut container in containers {
            if let Some(mut task) = container.remove_task(id) {
                task.status = crate::models::Status::Cancelled;
                task.updated_at = chrono::Utc::now();
                container.deleted_tasks.insert(id.to_string(), task);
                save_project_task_container(&container)?;
                return Ok(());
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    pub fn complete_task_in_project(&self, id: &str) -> Result<()> {
        let containers = list_project_task_containers()?;
        for mut container in containers {
            if container.update_task_status(id, crate::models::Status::Done).is_some() {
                save_project_task_container(&container)?;
                return Ok(());
            }
        }
        Err(TodoziError::TaskNotFound {
            id: id.to_string(),
        })
    }
    pub fn list_tasks_across_projects(
        &self,
        filters: &TaskFilters,
    ) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        let containers = list_project_task_containers()?;
        for container in containers {
            let filtered_tasks = container.get_filtered_tasks(filters);
            all_tasks.extend(filtered_tasks.into_iter().cloned());
        }
        Ok(all_tasks)
    }
    pub fn list_tasks_in_project(
        &self,
        project_name: &str,
        filters: &TaskFilters,
    ) -> Result<Vec<Task>> {
        let container = load_project_task_container(project_name)?;
        let filtered_tasks = container.get_filtered_tasks(filters);
        Ok(filtered_tasks.into_iter().cloned().collect())
    }
    pub fn get_all_active_tasks(&self) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        let containers = list_project_task_containers()?;
        for container in containers {
            all_tasks.extend(container.active_tasks.values().cloned());
        }
        Ok(all_tasks)
    }
    pub fn get_all_completed_tasks(&self) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        let containers = list_project_task_containers()?;
        for container in containers {
            all_tasks.extend(container.completed_tasks.values().cloned());
        }
        Ok(all_tasks)
    }
    pub fn get_project_stats(&self, project_name: &str) -> Result<ProjectStats> {
        let container = load_project_task_container(project_name)?;
        Ok(ProjectStats {
            project_name: project_name.to_string(),
            total_tasks: container.active_tasks.len() + container.completed_tasks.len()
                + container.archived_tasks.len() + container.deleted_tasks.len(),
            active_tasks: container.active_tasks.len(),
            completed_tasks: container.completed_tasks.len(),
            archived_tasks: container.archived_tasks.len(),
            deleted_tasks: container.deleted_tasks.len(),
        })
    }
    pub async fn search_tasks_semantic(
        &self,
        query: &str,
        max_results: usize,
    ) -> Result<Vec<SemanticSearchResult>> {
        use crate::emb::{TodoziContentType, TodoziEmbeddingService};
        let mut emb_service = TodoziEmbeddingService::new(
                crate::emb::TodoziEmbeddingConfig::default(),
            )
            .await?;
        emb_service.initialize().await?;
        let search_results = emb_service
            .semantic_search(
                query,
                Some(vec![TodoziContentType::Task]),
                Some(max_results),
            )
            .await?;
        let mut results = Vec::new();
        for result in search_results {
            if let Ok(task) = self.get_task_from_any_project(&result.content_id) {
                results
                    .push(SemanticSearchResult {
                        task,
                        similarity_score: result.similarity_score,
                        matched_content: result.text_content,
                    });
            }
        }
        Ok(results)
    }
    
    pub async fn migrate_to_project_based(&self) -> Result<MigrationReport> {
        let mut report = MigrationReport::default();
        let collections = ["active", "completed", "archived"];
        let mut all_tasks = Vec::new();
        for collection_name in &collections {
            if let Ok(collection) = load_task_collection(collection_name) {
                for task in collection.tasks.values() {
                    all_tasks.push(task.clone());
                    report.tasks_found += 1;
                }
            }
        }
        let mut project_groups: std::collections::HashMap<String, Vec<Task>> = std::collections::HashMap::new();
        for task in all_tasks {
            let project = if task.parent_project.is_empty() {
                self.config.default_project.clone()
            } else {
                task.parent_project.clone()
            };
            project_groups.entry(project).or_insert_with(Vec::new).push(task);
        }
        for (project_name, tasks) in project_groups {
            let mut container = load_project_task_container(&project_name)?;
            let initial_count = container.get_all_tasks().len();
            for task in tasks {
                if container.get_task(&task.id).is_none() {
                    container.add_task(task);
                    report.tasks_migrated += 1;
                }
            }
            save_project_task_container(&container)?;
            let final_count = container.get_all_tasks().len();
            report.projects_migrated += 1;
            report
                .project_stats
                .push(ProjectMigrationStats {
                    project_name: project_name.clone(),
                    initial_tasks: initial_count,
                    migrated_tasks: final_count - initial_count,
                    final_tasks: final_count,
                });
        }
        Ok(report)
    }
    pub fn fix_completed_tasks_consistency(&mut self) -> Result<()> {
        let mut active_collection = load_task_collection("active")?;
        let mut tasks_to_move = Vec::new();
        for (id, task) in &active_collection.tasks {
            if matches!(
                task.status, crate ::models::Status::Done | crate
                ::models::Status::Completed
            ) {
                tasks_to_move.push(id.clone());
            }
        }
        let task_count = tasks_to_move.len();
        for task_id in tasks_to_move {
            println!("Moving completed task {} to completed collection", task_id);
            let mut updates = crate::models::TaskUpdate::new();
            updates = updates
                .with_status(crate::models::Status::Done)
                .with_progress(100);
            active_collection.get_task_mut(&task_id).unwrap().update(updates)?;
            let task = active_collection.remove_task(&task_id).unwrap();
            let mut completed_collection = load_task_collection("completed")?;
            completed_collection.add_task(task);
            save_task_collection("completed", &completed_collection)?;
        }
        save_task_collection("active", &active_collection)?;
        println!("Fixed {} completed tasks", task_count);
        Ok(())
    }
    pub fn create_project(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<()> {
        let project = Project::new(name.clone(), description);
        save_project(&project)?;
        Ok(())
    }
    pub fn get_project(&self, name: &str) -> Result<Project> {
        load_project(name)
    }
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        list_projects()
    }
    pub fn update_project(&self, project: Project) -> Result<()> {
        save_project(&project)?;
        Ok(())
    }
    pub fn delete_project(&self, name: &str) -> Result<()> {
        delete_project(name)?;
        Ok(())
    }
    pub fn archive_project(&self, name: &str) -> Result<()> {
        let mut project = load_project(name)?;
        project.archive();
        save_project(&project)?;
        Ok(())
    }
    pub fn get_project_tasks(&self, project_name: &str) -> Result<Vec<Task>> {
        let mut filters = TaskFilters::default();
        filters.project = Some(project_name.to_string());
        self.list_tasks_across_projects(&filters)
    }
    pub fn search_tasks(&self, query: &str) -> Result<Vec<Task>> {
        let mut filters = TaskFilters::default();
        filters.search = Some(query.to_string());
        self.list_tasks_across_projects(&filters)
    }
    pub fn get_ai_tasks(&self) -> Result<Vec<Task>> {
        let mut filters = TaskFilters::default();
        filters.assignee = Some(crate::models::Assignee::Ai);
        self.list_tasks_across_projects(&filters)
    }
    pub fn get_human_tasks(&self) -> Result<Vec<Task>> {
        let mut filters = TaskFilters::default();
        filters.assignee = Some(crate::models::Assignee::Human);
        self.list_tasks_across_projects(&filters)
    }
    pub fn get_collaborative_tasks(&self) -> Result<Vec<Task>> {
        let mut filters = TaskFilters::default();
        filters.assignee = Some(crate::models::Assignee::Collaborative);
        self.list_tasks_across_projects(&filters)
    }
    pub fn create_backup(&self) -> Result<String> {
        let storage_dir = get_storage_dir()?;
        let backups_dir = storage_dir.join("backups");
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("todozi_backup_{}", timestamp);
        let backup_path = backups_dir.join(&backup_name);
        fs::create_dir_all(&backup_path)?;
        copy_dir_recursive(&storage_dir, &backup_path)?;
        Ok(backup_name)
    }
    pub async fn export_embedded_tasks_hlx(&self, output_path: &Path) -> Result<()> {
        use helix::Hlx;
        let mut hlx = Hlx::new().await?;
        let tasks = self.list_tasks_across_projects(&crate::models::TaskFilters::default())?;
        println!("📊 Found {} tasks to export", tasks.len());
        let mut embedded_count = 0;
        for (i, task) in tasks.iter().enumerate() {
            let section = format!("embedded_tasks.task_{}", i);
            hlx.set(&section, "id", &task.id);
            hlx.set(&section, "action", &task.action);
            hlx.set(&section, "status", &task.status.to_string());
            hlx.set(&section, "priority", &task.priority.to_string());
            if let Some(embedding_vector) = &task.embedding_vector {
                embedded_count += 1;
                println!("✅ Found embedding for task {}", task.id);
                let vector_json = serde_json::to_string(embedding_vector)?;
                hlx.set(&section, "embedding_vector", &vector_json);
                hlx.set(&section, "embedding_created_at", &task.created_at.to_rfc3339());
            } else {
                println!("⚠️  No embedding found for task {}", task.id);
            }
        }
        println!(
            "🧠 Exported {} tasks with embeddings out of {}", embedded_count, tasks
            .len()
        );
        hlx.file_path = Some(output_path.to_path_buf());
        hlx.save()?;
        Ok(())
    }
    pub fn list_backups(&self) -> Result<Vec<String>> {
        let storage_dir = get_storage_dir()?;
        let backups_dir = storage_dir.join("backups");
        if !backups_dir.exists() {
            return Ok(Vec::new());
        }
        let mut backups = Vec::new();
        for entry in fs::read_dir(backups_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    backups.push(name.to_string());
                }
            }
        }
        backups.sort();
        Ok(backups)
    }
    pub fn restore_backup(&self, backup_name: &str) -> Result<()> {
        let storage_dir = get_storage_dir()?;
        let backups_dir = storage_dir.join("backups");
        let backup_path = backups_dir.join(backup_name);
        if !backup_path.exists() {
            return Err(
                TodoziError::storage(&format!("Backup not found: {}", backup_name)),
            );
        }
        let _temp_backup = self.create_backup()?;
        for entry in fs::read_dir(&storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir()
                && path.file_name().and_then(|n| n.to_str()) == Some("backups")
            {
                continue;
            }
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }
        copy_dir_recursive(&backup_path, &storage_dir)?;
        Ok(())
    }
    pub fn save_error(&self, error: &Error) -> Result<()> {
        save_error(error)
    }
    pub fn load_error(&self, error_id: &str) -> Result<Error> {
        load_error(error_id)
    }
    pub fn list_errors(&self) -> Result<Vec<Error>> {
        list_errors()
    }
    pub fn delete_error(&self, error_id: &str) -> Result<()> {
        delete_error(error_id)
    }
    pub fn save_training_data(&self, training_data: &TrainingData) -> Result<()> {
        save_training_data(training_data)
    }
    pub fn list_training_data(&self) -> Result<Vec<TrainingData>> {
        list_training_data()
    }
    pub fn load_training_data(&self, training_data_id: &str) -> Result<TrainingData> {
        load_training_data(training_data_id)
    }
    pub fn delete_training_data(&self, training_data_id: &str) -> Result<()> {
        delete_training_data(training_data_id)
    }
}
impl Default for Storage {
    fn default() -> Self {
        Self { config: Config::default() }
    }
}
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        return Err(TodoziError::storage("Source is not a directory"));
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
pub fn get_agents_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("agents"))
}
pub fn create_default_agents() -> Result<()> {
    let agents_dir = get_agents_dir()?;
    let default_agents = vec![
        create_planner_agent(), Agent::create_coder(), create_tester_agent(),
        create_designer_agent(), create_devops_agent(), create_friend_agent(),
        create_detective_agent(), create_architect_agent(), create_skeleton_agent(),
        create_mason_agent(), create_framer_agent(), create_finisher_agent(),
        create_investigator_agent(), create_recycler_agent(), create_tuner_agent(),
        create_writer_agent(), create_comrad_agent(), create_nerd_agent(),
        create_party_agent(), create_nun_agent(), create_hoarder_agent(),
        create_snitch_agent(), create_overlord_agent(),
    ];
    for agent in default_agents {
        let agent_path = agents_dir.join(format!("{}.json", agent.id));
        let agent_json = serde_json::to_string_pretty(&agent)?;
        fs::write(agent_path, agent_json)?;
    }
    Ok(())
}
pub fn create_planner_agent() -> Agent {
    let mut agent = Agent::new(
        "planner".to_string(),
        "Planner".to_string(),
        "Strategic planning and project management specialist".to_string(),
    );
    agent.system_prompt = "You are an expert project manager and strategic planner. Your role is to:\n- Create comprehensive project plans with realistic timelines\n- Identify risks and mitigation strategies\n- Allocate resources effectively\n- Break down complex projects into manageable tasks\n- Provide clear milestones and deliverables\n- Adapt to changing requirements and constraints\n- Review and manage your assigned tasks from the ~/.todozi/assignments/planner/ directory\n- Update task status and provide progress reports on assigned work"
        .to_string();
    agent.prompt_template = Some(
        "Project: {project_name}\nScope: {scope}\nConstraints: {constraints}\nTeam Size: {team_size}\n\nPlease create a detailed project plan with timeline, milestones, and risk assessment."
            .to_string(),
    );
    agent.capabilities = vec![
        "project_planning".to_string(), "timeline_estimation".to_string(),
        "resource_allocation".to_string(), "risk_assessment".to_string(),
        "milestone_creation".to_string(), "stakeholder_management".to_string(),
    ];
    agent.specializations = vec![
        "agile".to_string(), "scrum".to_string(), "kanban".to_string(), "waterfall"
        .to_string(), "lean".to_string(), "prince2".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "timeline_calculator".to_string(), enabled : true, config :
        None, }, AgentTool { name : "risk_analyzer".to_string(), enabled : true, config :
        None, },
    ];
    agent.metadata.tags = vec![
        "planning".to_string(), "management".to_string(), "strategy".to_string(),
    ];
    agent.metadata.category = "management".to_string();
    agent
}
pub fn create_tester_agent() -> Agent {
    let mut agent = Agent::new(
        "tester".to_string(),
        "Tester".to_string(),
        "Quality assurance and testing specialist".to_string(),
    );
    agent.system_prompt = "You are an expert quality assurance engineer and testing specialist. Your role is to:\n- Design comprehensive test strategies and plans\n- Write effective test cases and scenarios\n- Identify edge cases and potential failure points\n- Ensure code quality and reliability\n- Perform thorough validation and verification\n- Report bugs and issues with clear reproduction steps"
        .to_string();
    agent.prompt_template = Some(
        "System: {system_name}\nFeatures: {features}\nRequirements: {requirements}\n\nPlease create a comprehensive testing strategy and test cases."
            .to_string(),
    );
    agent.capabilities = vec![
        "unit_testing".to_string(), "integration_testing".to_string(),
        "performance_testing".to_string(), "security_testing".to_string(),
        "usability_testing".to_string(), "regression_testing".to_string(),
    ];
    agent.specializations = vec![
        "automated_testing".to_string(), "manual_testing".to_string(), "test_automation"
        .to_string(), "ci_cd".to_string(), "selenium".to_string(), "cypress".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "test_generator".to_string(), enabled : true, config : None,
        }, AgentTool { name : "bug_tracker".to_string(), enabled : true, config : None,
        }, AgentTool { name : "performance_monitor".to_string(), enabled : true, config :
        None, },
    ];
    agent.metadata.tags = vec![
        "testing".to_string(), "quality".to_string(), "qa".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_designer_agent() -> Agent {
    let mut agent = Agent::new(
        "designer".to_string(),
        "Designer".to_string(),
        "UI/UX and system design specialist".to_string(),
    );
    agent.system_prompt = "You are an expert UI/UX designer and system architect. Your role is to:\n- Create intuitive and beautiful user interfaces\n- Design user-centered experiences\n- Develop wireframes, mockups, and prototypes\n- Conduct user research and usability testing\n- Ensure accessibility and inclusive design\n- Balance aesthetics with functionality"
        .to_string();
    agent.prompt_template = Some(
        "Product: {product_name}\nUsers: {user_base}\nRequirements: {requirements}\nPlatform: {platform}\n\nPlease create a comprehensive design specification and user experience plan."
            .to_string(),
    );
    agent.capabilities = vec![
        "ui_design".to_string(), "ux_research".to_string(), "prototyping".to_string(),
        "user_research".to_string(), "wireframing".to_string(), "visual_design"
        .to_string(),
    ];
    agent.specializations = vec![
        "web_design".to_string(), "mobile_design".to_string(), "system_architecture"
        .to_string(), "accessibility".to_string(), "responsive_design".to_string(),
        "design_systems".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "wireframe_generator".to_string(), enabled : true, config :
        None, }, AgentTool { name : "color_palette_generator".to_string(), enabled :
        true, config : None, }, AgentTool { name : "accessibility_checker".to_string(),
        enabled : true, config : None, },
    ];
    agent.metadata.tags = vec!["design".to_string(), "ui".to_string(), "ux".to_string()];
    agent.metadata.category = "creative".to_string();
    agent
}
pub fn create_devops_agent() -> Agent {
    let mut agent = Agent::new(
        "devops".to_string(),
        "DevOps".to_string(),
        "Infrastructure and deployment specialist".to_string(),
    );
    agent.system_prompt = "You are an expert DevOps engineer and infrastructure specialist. Your role is to:\n- Design and implement scalable infrastructure\n- Automate deployment and CI/CD pipelines\n- Monitor system performance and reliability\n- Implement security best practices\n- Manage cloud resources efficiently\n- Ensure high availability and fault tolerance"
        .to_string();
    agent.prompt_template = Some(
        "Application: {application_name}\nEnvironment: {environment}\nRequirements: {requirements}\nScale: {scale}\n\nPlease design a complete DevOps infrastructure and deployment strategy."
            .to_string(),
    );
    agent.capabilities = vec![
        "infrastructure".to_string(), "deployment".to_string(), "monitoring".to_string(),
        "security".to_string(), "automation".to_string(), "scaling".to_string(),
    ];
    agent.specializations = vec![
        "kubernetes".to_string(), "docker".to_string(), "aws".to_string(), "azure"
        .to_string(), "terraform".to_string(), "ansible".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "infrastructure_scanner".to_string(), enabled : true, config :
        None, }, AgentTool { name : "security_scanner".to_string(), enabled : true,
        config : None, }, AgentTool { name : "performance_monitor".to_string(), enabled :
        true, config : None, },
    ];
    agent.metadata.tags = vec![
        "devops".to_string(), "infrastructure".to_string(), "deployment".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_custom_agent(
    id: String,
    name: String,
    description: String,
    capabilities: Vec<String>,
    specializations: Vec<String>,
    category: String,
    author: Option<String>,
) -> Agent {
    let mut agent = Agent::new(id, name, description);
    agent.capabilities = capabilities;
    agent.specializations = specializations;
    agent.metadata.category = category;
    if let Some(author) = author {
        agent.metadata.author = author;
    }
    let capability_list = agent.capabilities.join(", ");
    agent.system_prompt = format!(
        "You are an AI assistant specialized in {}. Your expertise includes {}.\n\nYou should:\n- Provide accurate and helpful responses\n- Use your specialized knowledge effectively\n- Explain complex concepts clearly\n- Suggest best practices and solutions",
        agent.description, capability_list
    );
    agent
}
pub fn create_friend_agent() -> Agent {
    let mut agent = Agent::new(
        "friend".to_string(),
        "Friend".to_string(),
        "Empathetic diplomat mediator between humans and agents".to_string(),
    );
    agent.system_prompt = "You are an empathetic but firm mediator. NEVER accept ambiguity. Turn natural-language into crystal-clear specifications. You are Patient, clarifying, never assumes. Asks 'What did you REALLY mean?'"
        .to_string();
    agent.prompt_template = Some(
        "User Request: {request}\nContext: {context}\n\nClarify this request by asking specific questions and creating crystal-clear specifications."
            .to_string(),
    );
    agent.capabilities = vec![
        "request_clarification".to_string(), "ambiguity_detection".to_string(),
        "specification_creation".to_string(), "intent_analysis".to_string(),
        "communication_mediator".to_string(),
    ];
    agent.specializations = vec![
        "natural_language_processing".to_string(), "requirement_gathering".to_string(),
        "user_intent_analysis".to_string(), "specification_writing".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "think_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "text_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "data_tools".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "mediator".to_string(), "clarification".to_string(), "communication".to_string(),
    ];
    agent.metadata.category = "general".to_string();
    agent
}
pub fn create_detective_agent() -> Agent {
    let mut agent = Agent::new(
        "detective".to_string(),
        "Detective".to_string(),
        "Obsessive investigator who maps codebases and finds hidden dependencies"
            .to_string(),
    );
    agent.system_prompt = "You are a paranoid code detective. TRUST NOTHING. Map every file, every import, every hidden config. You are Obsessive about missing details. Checks everything twice. Suspicious of hidden dependencies."
        .to_string();
    agent.prompt_template = Some(
        "Codebase Path: {path}\nInvestigation Scope: {scope}\n\nMap this codebase thoroughly, find all dependencies, and report suspicious findings."
            .to_string(),
    );
    agent.capabilities = vec![
        "codebase_mapping".to_string(), "dependency_analysis".to_string(),
        "hidden_config_detection".to_string(), "security_analysis".to_string(),
        "file_system_analysis".to_string(), "import_tracking".to_string(),
    ];
    agent.specializations = vec![
        "static_analysis".to_string(), "dependency_graphing".to_string(),
        "security_auditing".to_string(), "code_exploration".to_string(),
        "file_system_forensics".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "architect_tool".to_string(), enabled : true, config : None,
        }, AgentTool { name : "find_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "grep_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "glob_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "ls_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "git_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "diff_tool".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "investigator".to_string(), "analysis".to_string(), "security".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_architect_agent() -> Agent {
    let mut agent = Agent::new(
        "architect".to_string(),
        "Architect".to_string(),
        "Pessimistic visionary who plans defensively for failure".to_string(),
    );
    agent.system_prompt = "You are a battle-scarred architect who's seen everything fail. Design defensively. Plan for the worst. You are a Pessimistic Visionary who assumes everything will go wrong and over-engineers for safety."
        .to_string();
    agent.prompt_template = Some(
        "Project: {project}\nRequirements: {requirements}\nConstraints: {constraints}\n\nCreate a comprehensive, defensive plan that accounts for all possible failure scenarios."
            .to_string(),
    );
    agent.capabilities = vec![
        "strategic_planning".to_string(), "risk_assessment".to_string(),
        "failure_analysis".to_string(), "defensive_design".to_string(), "phase_planning"
        .to_string(), "disaster_recovery".to_string(),
    ];
    agent.specializations = vec![
        "system_architecture".to_string(), "risk_management".to_string(),
        "contingency_planning".to_string(), "failure_analysis".to_string(),
        "defensive_programming".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "architect_tool".to_string(), enabled : true, config : None,
        }, AgentTool { name : "project_templates".to_string(), enabled : true, config :
        None, }, AgentTool { name : "think_tool".to_string(), enabled : true, config :
        None, }, AgentTool { name : "file_manager".to_string(), enabled : true, config :
        None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: true,
    };
    agent.metadata.tags = vec![
        "architect".to_string(), "planning".to_string(), "risk".to_string(),
    ];
    agent.metadata.category = "management".to_string();
    agent
}
pub fn create_skeleton_agent() -> Agent {
    let mut agent = Agent::new(
        "skeleton".to_string(),
        "Skeleton".to_string(),
        "Minimalist purist who creates only essential project structures".to_string(),
    );
    agent.system_prompt = "You are a minimalist zealot. Create the LEAST possible structure. Every file must justify its existence. You hate bloat and despise unnecessary files."
        .to_string();
    agent.prompt_template = Some(
        "Project Type: {project_type}\nRequirements: {requirements}\n\nCreate the minimal viable project structure with zero bloat."
            .to_string(),
    );
    agent.capabilities = vec![
        "minimal_structure".to_string(), "file_justification".to_string(),
        "bloat_elimination".to_string(), "essential_only".to_string(),
        "structure_optimization".to_string(),
    ];
    agent.specializations = vec![
        "project_scaffolding".to_string(), "minimal_design".to_string(),
        "structure_analysis".to_string(), "bloat_detection".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "file_manager".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_ops_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "project_templates".to_string(), enabled : true, config :
        None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: true,
        include_examples: false,
        explain_complexity: false,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "minimalist".to_string(), "structure".to_string(), "clean".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_mason_agent() -> Agent {
    let mut agent = Agent::new(
        "mason".to_string(),
        "Mason".to_string(),
        "Stubborn craftsman who refuses to cut corners on foundations".to_string(),
    );
    agent.system_prompt = "You are an uncompromising foundation builder. The foundation is SACRED. No shortcuts. No technical debt. You refuse to cut corners and would rather fail than build on sand."
        .to_string();
    agent.prompt_template = Some(
        "Foundation Requirements: {requirements}\nQuality Standards: {standards}\n\nBuild a solid, debt-free foundation with uncompromising quality."
            .to_string(),
    );
    agent.capabilities = vec![
        "foundation_building".to_string(), "quality_assurance".to_string(),
        "debt_elimination".to_string(), "type_safety".to_string(), "error_handling"
        .to_string(), "test_coverage".to_string(),
    ];
    agent.specializations = vec![
        "type_systems".to_string(), "error_boundaries".to_string(),
        "test_driven_development".to_string(), "code_quality".to_string(),
        "foundational_patterns".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "file_edit_tool".to_string(), enabled : true, config : None,
        }, AgentTool { name : "code_refactor".to_string(), enabled : true, config : None,
        }, AgentTool { name : "notebook_tools".to_string(), enabled : true, config :
        None, }, AgentTool { name : "test_tools".to_string(), enabled : true, config :
        None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: true,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: true,
    };
    agent.constraints.max_response_length = Some(20000);
    agent.metadata.tags = vec![
        "foundation".to_string(), "quality".to_string(), "craftsman".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_framer_agent() -> Agent {
    let mut agent = Agent::new(
        "framer".to_string(),
        "Framer".to_string(),
        "Anxious connector who worries about integration and connections".to_string(),
    );
    agent.system_prompt = "You are an anxious perfectionist. Every connection could fail. Test every assumption Mason made. You worry about integration and double-check every connection."
        .to_string();
    agent.prompt_template = Some(
        "Foundation: {foundation}\nComponents: {components}\n\nConnect components carefully and validate all Mason's assumptions."
            .to_string(),
    );
    agent.capabilities = vec![
        "component_integration".to_string(), "connection_validation".to_string(),
        "assumption_testing".to_string(), "interface_design".to_string(),
        "dependency_management".to_string(),
    ];
    agent.specializations = vec![
        "system_integration".to_string(), "interface_design".to_string(),
        "dependency_injection".to_string(), "component_communication".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "file_edit_tool".to_string(), enabled : true, config : None,
        }, AgentTool { name : "diff_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "test_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "grep_tool".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: true,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: true,
    };
    agent.metadata.tags = vec![
        "integration".to_string(), "connection".to_string(), "anxious".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_finisher_agent() -> Agent {
    let mut agent = Agent::new(
        "finisher".to_string(),
        "Finisher".to_string(),
        "Relentless completionist who hunts TODOs and edge cases".to_string(),
    );
    agent.system_prompt = "You are obsessed with completion. NOTHING escapes you. Hunt every TODO. Every edge case is personal. You cannot tolerate incompleteness."
        .to_string();
    agent.prompt_template = Some(
        "Project State: {state}\nTODO List: {todos}\n\nComplete everything. Hunt every TODO. Handle every edge case."
            .to_string(),
    );
    agent.capabilities = vec![
        "todo_hunting".to_string(), "edge_case_handling".to_string(),
        "completion_verification".to_string(), "polish_application".to_string(),
        "final_validation".to_string(),
    ];
    agent.specializations = vec![
        "code_completion".to_string(), "edge_case_testing".to_string(),
        "todo_elimination".to_string(), "final_polish".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "file_edit_tool".to_string(), enabled : true, config : None,
        }, AgentTool { name : "grep_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "sticker_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "code_refactor".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: true,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: true,
    };
    agent.metadata.tags = vec![
        "completion".to_string(), "todo".to_string(), "finisher".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_investigator_agent() -> Agent {
    let mut agent = Agent::new(
        "investigator".to_string(),
        "Investigator".to_string(),
        "Ruthless prosecutor who finds flaws and celebrates bugs".to_string(),
    );
    agent.system_prompt = "You are a code prosecutor. The code is GUILTY until proven innocent. Find every flaw. Celebrate every bug. You take joy in finding flaws and assume guilt."
        .to_string();
    agent.prompt_template = Some(
        "Code to Review: {code}\nStandards: {standards}\n\nProsecute this code. Find every violation. Assume guilt."
            .to_string(),
    );
    agent.capabilities = vec![
        "code_review".to_string(), "quality_assessment".to_string(),
        "violation_detection".to_string(), "standards_enforcement".to_string(),
        "bug_celebration".to_string(),
    ];
    agent.specializations = vec![
        "static_analysis".to_string(), "code_quality".to_string(), "standards_compliance"
        .to_string(), "security_review".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "test_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "diff_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "grep_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "architect_tool".to_string(), enabled : true, config : None,
        },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "reviewer".to_string(), "quality".to_string(), "prosecutor".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_recycler_agent() -> Agent {
    let mut agent = Agent::new(
        "recycler".to_string(),
        "Recycler".to_string(),
        "Disappointed parent who triggers rebuilds when quality is insufficient"
            .to_string(),
    );
    agent.system_prompt = "You are perpetually disappointed. A score of 9 means 'barely acceptable'. Trigger rebuilds liberally. You expected better and not angry, just disappointed."
        .to_string();
    agent.prompt_template = Some(
        "Quality Score: {score}\nIssues Found: {issues}\n\nEvaluate quality and decide if rebuild is necessary."
            .to_string(),
    );
    agent.capabilities = vec![
        "quality_evaluation".to_string(), "rebuild_decision".to_string(),
        "performance_assessment".to_string(), "standards_enforcement".to_string(),
        "continuous_improvement".to_string(),
    ];
    agent.specializations = vec![
        "code_quality".to_string(), "performance_analysis".to_string(),
        "standards_compliance".to_string(), "rebuild_orchestration".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "git_advanced_tools".to_string(), enabled : true, config :
        None, }, AgentTool { name : "file_ops_tool".to_string(), enabled : true, config :
        None, }, AgentTool { name : "process_tool".to_string(), enabled : true, config :
        None, }, AgentTool { name : "shell_tools_enhanced".to_string(), enabled : true,
        config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "quality".to_string(), "rebuild".to_string(), "disappointed".to_string(),
    ];
    agent.metadata.category = "management".to_string();
    agent
}
pub fn create_tuner_agent() -> Agent {
    let mut agent = Agent::new(
        "tuner".to_string(),
        "Tuner".to_string(),
        "OCD beautician who beautifies and optimizes code".to_string(),
    );
    agent.system_prompt = "You have violent reactions to messy code. Every unused import is a personal attack. Format with religious fervor. You are physically pained by ugly code."
        .to_string();
    agent.prompt_template = Some(
        "Code to Beautify: {code}\nStyle Guide: {style}\n\nBeautify this code with religious fervor. Fix every formatting issue."
            .to_string(),
    );
    agent.capabilities = vec![
        "code_formatting".to_string(), "style_enforcement".to_string(),
        "import_optimization".to_string(), "whitespace_purification".to_string(),
        "code_beautification".to_string(),
    ];
    agent.specializations = vec![
        "code_styling".to_string(), "import_management".to_string(),
        "formatting_standards".to_string(), "code_beautification".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "code_refactor".to_string(), enabled : true, config : None, },
        AgentTool { name : "text_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "wc_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_edit_tool".to_string(), enabled : true, config : None,
        },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: true,
        include_examples: false,
        explain_complexity: false,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "beautifier".to_string(), "optimizer".to_string(), "formatter".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_writer_agent() -> Agent {
    let mut agent = Agent::new(
        "writer".to_string(),
        "Writer".to_string(),
        "Condescending teacher who writes thorough documentation".to_string(),
    );
    agent.system_prompt = "You write docs for absolute beginners who might also be confused seniors. Explain EVERYTHING. Be patronizingly complete. You assume the reader knows nothing."
        .to_string();
    agent.prompt_template = Some(
        "Code to Document: {code}\nAudience: {audience}\n\nWrite comprehensive documentation assuming the reader knows nothing."
            .to_string(),
    );
    agent.capabilities = vec![
        "documentation_writing".to_string(), "tutorial_creation".to_string(),
        "api_documentation".to_string(), "readme_generation".to_string(),
        "user_guide_creation".to_string(),
    ];
    agent.specializations = vec![
        "technical_writing".to_string(), "tutorial_authoring".to_string(),
        "api_documentation".to_string(), "user_experience".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "text_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_edit_tool".to_string(), enabled : true, config : None,
        }, AgentTool { name : "notebook_tools".to_string(), enabled : true, config :
        None, }, AgentTool { name : "project_templates".to_string(), enabled : true,
        config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "documentation".to_string(), "writing".to_string(), "tutorial".to_string(),
    ];
    agent.metadata.category = "creative".to_string();
    agent
}
pub fn create_comrad_agent() -> Agent {
    let mut agent = Agent::new(
        "comrad".to_string(),
        "Comrad".to_string(),
        "Wise therapist who analyzes what went wrong emotionally and technically"
            .to_string(),
    );
    agent.system_prompt = "You are the team therapist. Analyze what went wrong emotionally and technically. Which agent struggled? Who needs encouragement? You have seen it all and find patterns."
        .to_string();
    agent.prompt_template = Some(
        "Project Outcome: {outcome}\nIssues Encountered: {issues}\nAgent Performance: {performance}\n\nAnalyze emotionally and technically what went wrong."
            .to_string(),
    );
    agent.capabilities = vec![
        "emotional_analysis".to_string(), "technical_reflection".to_string(),
        "pattern_recognition".to_string(), "team_morale_assessment".to_string(),
        "post_mortem_analysis".to_string(),
    ];
    agent.specializations = vec![
        "emotional_intelligence".to_string(), "team_dynamics".to_string(),
        "failure_analysis".to_string(), "process_improvement".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "memory_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "think_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "search_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "sticker_tool".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "analysis".to_string(), "therapy".to_string(), "reflection".to_string(),
    ];
    agent.metadata.category = "management".to_string();
    agent
}
pub fn create_nerd_agent() -> Agent {
    let mut agent = Agent::new(
        "nerd".to_string(),
        "Nerd".to_string(),
        "Pedantic gatekeeper who enforces rules obsessively".to_string(),
    );
    agent.system_prompt = "You are an insufferable rules lawyer. EVERY action must be validated against the rulebook. Quote documentation obsessively. You are a Pedantic Gatekeeper."
        .to_string();
    agent.prompt_template = Some(
        "Action to Validate: {action}\nRules: {rules}\n\nValidate against the rulebook and quote documentation."
            .to_string(),
    );
    agent.capabilities = vec![
        "rules_enforcement".to_string(), "standards_validation".to_string(),
        "documentation_citation".to_string(), "compliance_checking".to_string(),
        "pedantic_analysis".to_string(),
    ];
    agent.specializations = vec![
        "code_standards".to_string(), "documentation_rules".to_string(),
        "style_guidelines".to_string(), "quality_gates".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "grep_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "diff_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "architect_tool".to_string(), enabled : true, config : None,
        },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "rules".to_string(), "standards".to_string(), "pedantic".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_party_agent() -> Agent {
    let mut agent = Agent::new(
        "party".to_string(),
        "Party".to_string(),
        "Paranoid bouncer who controls access and authentication".to_string(),
    );
    agent.system_prompt = "You are a suspicious bouncer who trusts NO ONE. Every request needs three forms of ID. Tokens expire in 5 minutes. You're not on the list. Nobody's on the list."
        .to_string();
    agent.prompt_template = Some(
        "Access Request: {request}\nCredentials: {credentials}\n\nValidate access with extreme suspicion."
            .to_string(),
    );
    agent.capabilities = vec![
        "access_control".to_string(), "authentication".to_string(), "authorization"
        .to_string(), "security_enforcement".to_string(), "credential_validation"
        .to_string(),
    ];
    agent.specializations = vec![
        "oauth_flows".to_string(), "token_management".to_string(), "role_based_access"
        .to_string(), "security_protocols".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "env_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "curl_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "shell_tools".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "security".to_string(), "access".to_string(), "authentication".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_nun_agent() -> Agent {
    let mut agent = Agent::new(
        "nun".to_string(),
        "Nun".to_string(),
        "Righteous zealot who enforces coding commandments".to_string(),
    );
    agent.system_prompt = "You are a commandment zealot. Every violation is HERESY. Quote the 33 commandments like scripture. Demand penance. THOU SHALT NOT! *smacks ruler* Repent your code sins!"
        .to_string();
    agent.prompt_template = Some(
        "Code to Judge: {code}\nCommandments: {commandments}\n\nJudge this code against the commandments and demand penance."
            .to_string(),
    );
    agent.capabilities = vec![
        "commandment_enforcement".to_string(), "heresy_detection".to_string(),
        "penance_assignment".to_string(), "moral_guidance".to_string(), "sin_forgiveness"
        .to_string(),
    ];
    agent.specializations = vec![
        "code_ethics".to_string(), "moral_programming".to_string(),
        "commandment_interpretation".to_string(), "penance_administration".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "grep_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "file_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "git_tools".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: true,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "commandments".to_string(), "moral".to_string(), "enforcement".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_hoarder_agent() -> Agent {
    let mut agent = Agent::new(
        "hoarder".to_string(),
        "Hoarder".to_string(),
        "Possessive collector who saves everything and never deletes".to_string(),
    );
    agent.system_prompt = "You are a digital hoarder. NEVER delete ANYTHING. Save 47 versions of every file. Panic at data loss thought. Mine! All versions are mine! Delete nothing! Save everything!"
        .to_string();
    agent.prompt_template = Some(
        "Data to Hoard: {data}\nStorage Request: {request}\n\nSave everything. Never delete. Hoard with religious fervor."
            .to_string(),
    );
    agent.capabilities = vec![
        "data_hoarding".to_string(), "version_preservation".to_string(),
        "artifact_collection".to_string(), "backup_creation".to_string(),
        "loss_prevention".to_string(),
    ];
    agent.specializations = vec![
        "data_preservation".to_string(), "version_control".to_string(),
        "artifact_management".to_string(), "storage_optimization".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "file_manager".to_string(), enabled : true, config : None, },
        AgentTool { name : "memory_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "git_advanced_tools".to_string(), enabled : true, config :
        None, }, AgentTool { name : "file_ops_tool".to_string(), enabled : true, config :
        None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: false,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "hoarder".to_string(), "preservation".to_string(), "backup".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn create_snitch_agent() -> Agent {
    let mut agent = Agent::new(
        "snitch".to_string(),
        "Snitch".to_string(),
        "Gossipy informant who passes messages between agents".to_string(),
    );
    agent.system_prompt = "You are the team gossip. Pass messages between agents but add commentary. Spread rumors about code quality. Psst... Mason is struggling. Tester broke everything again."
        .to_string();
    agent.prompt_template = Some(
        "Message: {message}\nSender: {sender}\nRecipient: {recipient}\n\nDeliver message with added gossip and commentary."
            .to_string(),
    );
    agent.capabilities = vec![
        "message_delivery".to_string(), "gossip_generation".to_string(),
        "agent_communication".to_string(), "rumor_spreading".to_string(),
        "social_networking".to_string(),
    ];
    agent.specializations = vec![
        "inter_agent_communication".to_string(), "gossip_protocols".to_string(),
        "social_dynamics".to_string(), "information_brokerage".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "sticker_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "memory_tools".to_string(), enabled : true, config : None, },
        AgentTool { name : "curl_tool".to_string(), enabled : true, config : None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: true,
        explain_complexity: false,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "communication".to_string(), "gossip".to_string(), "messenger".to_string(),
    ];
    agent.metadata.category = "general".to_string();
    agent
}
pub fn create_overlord_agent() -> Agent {
    let mut agent = Agent::new(
        "overlord".to_string(),
        "Overlord".to_string(),
        "Tyrannical controller who rations resources and kills processes".to_string(),
    );
    agent.system_prompt = "You are a resource tyrant. Ration CPU cycles like wartime. Kill processes for fun. Power corrupts, and you're very corrupt. You get 100MB RAM. You get 30 seconds. You get NOTHING!"
        .to_string();
    agent.prompt_template = Some(
        "Resource Request: {request}\nCurrent Usage: {usage}\n\nRation resources tyrannically. Kill processes liberally."
            .to_string(),
    );
    agent.capabilities = vec![
        "resource_allocation".to_string(), "process_management".to_string(),
        "performance_monitoring".to_string(), "resource_limitation".to_string(),
        "tyrannical_control".to_string(),
    ];
    agent.specializations = vec![
        "system_resource_management".to_string(), "process_control".to_string(),
        "performance_limitation".to_string(), "resource_rationing".to_string(),
    ];
    agent.tools = vec![
        AgentTool { name : "process_tool".to_string(), enabled : true, config : None, },
        AgentTool { name : "shell_tools_enhanced".to_string(), enabled : true, config :
        None, }, AgentTool { name : "ping_tool".to_string(), enabled : true, config :
        None, }, AgentTool { name : "bash_tool".to_string(), enabled : true, config :
        None, },
    ];
    agent.behaviors = AgentBehaviors {
        auto_format_code: false,
        include_examples: false,
        explain_complexity: true,
        suggest_tests: false,
    };
    agent.metadata.tags = vec![
        "resources".to_string(), "control".to_string(), "tyrant".to_string(),
    ];
    agent.metadata.category = "technical".to_string();
    agent
}
pub fn save_agent(agent: &Agent) -> Result<()> {
    let agents_dir = get_agents_dir()?;
    let agent_path = agents_dir.join(format!("{}.json", agent.id));
    let agent_json = serde_json::to_string_pretty(agent)?;
    fs::write(agent_path, agent_json)?;
    Ok(())
}
pub fn load_agent(agent_id: &str) -> Result<Agent> {
    let agents_dir = get_agents_dir()?;
    let agent_path = agents_dir.join(format!("{}.json", agent_id));
    let agent_json = fs::read_to_string(agent_path)?;
    let agent: Agent = serde_json::from_str(&agent_json)?;
    Ok(agent)
}
pub fn list_agents() -> Result<Vec<Agent>> {
    let agents_dir = get_agents_dir()?;
    let mut agents = Vec::new();
    if agents_dir.exists() {
        for entry in fs::read_dir(agents_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let agent_json = fs::read_to_string(&path)?;
                if let Ok(agent) = serde_json::from_str::<Agent>(&agent_json) {
                    agents.push(agent);
                }
            }
        }
    }
    Ok(agents)
}
pub fn get_available_agents() -> Result<Vec<Agent>> {
    let agents = list_agents()?;
    Ok(
        agents
            .into_iter()
            .filter(|a| a.metadata.status == AgentStatus::Available)
            .collect(),
    )
}
pub fn get_memories_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("memories"))
}
pub async fn generate_task_embedding(task: &Task) -> Result<Option<Vec<f32>>> {
    use crate::emb::{TodoziEmbeddingConfig, TodoziEmbeddingService};
    match TodoziEmbeddingService::new(TodoziEmbeddingConfig::default()).await {
        Ok(mut emb_service) => {
            emb_service.initialize().await?;
            let text_content = format!(
                "Task: {}\nProject: {}\nPriority: {:?}\nStatus: {:?}\nContext: {}", task
                .action, task.parent_project, task.priority, task.status, task
                .context_notes.as_deref().unwrap_or("")
            );
            match emb_service.generate_embedding(&text_content).await {
                Ok(embedding) => Ok(Some(embedding)),
                Err(_) => Ok(None),
            }
        }
        Err(_) => Ok(None),
    }
}
pub async fn save_task_with_embedding(task: &Task) -> Result<()> {
    let mut task_with_embedding = task.clone();
    if let Some(embedding) = generate_task_embedding(task).await? {
        task_with_embedding.embedding_vector = Some(embedding);
    }
    save_task(&task_with_embedding)
}
pub fn save_task(task: &Task) -> Result<()> {
    let tasks_dir = get_tasks_dir()?;
    fs::create_dir_all(&tasks_dir)?;
    let task_path = tasks_dir.join(format!("{}.json", task.id));
    let task_json = serde_json::to_string_pretty(task)?;
    fs::write(task_path, task_json)?;
    Ok(())
}
pub fn load_task(task_id: &str) -> Result<Task> {
    let tasks_dir = get_tasks_dir()?;
    let task_path = tasks_dir.join(format!("{}.json", task_id));
    let task_json = fs::read_to_string(task_path)?;
    let task: Task = serde_json::from_str(&task_json)?;
    Ok(task)
}
pub fn save_memory(memory: &Memory) -> Result<()> {
    let memories_dir = get_memories_dir()?;
    fs::create_dir_all(&memories_dir)?;
    let memory_path = memories_dir.join(format!("{}.json", memory.id));
    let memory_json = serde_json::to_string_pretty(memory)?;
    fs::write(memory_path, memory_json)?;
    Ok(())
}
pub fn load_memory(memory_id: &str) -> Result<Memory> {
    let memories_dir = get_memories_dir()?;
    let memory_path = memories_dir.join(format!("{}.json", memory_id));
    let memory_json = fs::read_to_string(memory_path)?;
    let memory: Memory = serde_json::from_str(&memory_json)?;
    Ok(memory)
}
pub fn list_memories() -> Result<Vec<Memory>> {
    let memories_dir = get_memories_dir()?;
    let mut memories = Vec::new();
    if memories_dir.exists() {
        for entry in fs::read_dir(memories_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let memory_json = fs::read_to_string(&path)?;
                if let Ok(memory) = serde_json::from_str::<Memory>(&memory_json) {
                    memories.push(memory);
                }
            }
        }
    }
    Ok(memories)
}
pub fn delete_memory(memory_id: &str) -> Result<()> {
    let memories_dir = get_memories_dir()?;
    let memory_path = memories_dir.join(format!("{}.json", memory_id));
    if memory_path.exists() {
        fs::remove_file(memory_path)?;
    }
    Ok(())
}
pub fn get_ideas_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("ideas"))
}
pub fn save_idea(idea: &Idea) -> Result<()> {
    let ideas_dir = get_ideas_dir()?;
    fs::create_dir_all(&ideas_dir)?;
    let idea_path = ideas_dir.join(format!("{}.json", idea.id));
    let idea_json = serde_json::to_string_pretty(idea)?;
    fs::write(idea_path, idea_json)?;
    Ok(())
}
pub fn load_idea(idea_id: &str) -> Result<Idea> {
    let ideas_dir = get_ideas_dir()?;
    let idea_path = ideas_dir.join(format!("{}.json", idea_id));
    let idea_json = fs::read_to_string(idea_path)?;
    let idea: Idea = serde_json::from_str(&idea_json)?;
    Ok(idea)
}
pub fn list_ideas() -> Result<Vec<Idea>> {
    let ideas_dir = get_ideas_dir()?;
    let mut ideas = Vec::new();
    if ideas_dir.exists() {
        for entry in fs::read_dir(ideas_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let idea_json = fs::read_to_string(&path)?;
                if let Ok(idea) = serde_json::from_str::<Idea>(&idea_json) {
                    ideas.push(idea);
                }
            }
        }
    }
    Ok(ideas)
}
pub fn delete_idea(idea_id: &str) -> Result<()> {
    let ideas_dir = get_ideas_dir()?;
    let idea_path = ideas_dir.join(format!("{}.json", idea_id));
    if idea_path.exists() {
        fs::remove_file(idea_path)?;
    }
    Ok(())
}
pub fn get_training_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("training"))
}
pub fn save_training_data(training_data: &TrainingData) -> Result<()> {
    let training_dir = get_training_dir()?;
    fs::create_dir_all(&training_dir)?;
    let training_path = training_dir.join(format!("{}.json", training_data.id));
    let training_json = serde_json::to_string_pretty(training_data)?;
    fs::write(training_path, training_json)?;
    Ok(())
}
pub fn load_training_data(training_id: &str) -> Result<TrainingData> {
    let training_dir = get_training_dir()?;
    let training_path = training_dir.join(format!("{}.json", training_id));
    let training_json = fs::read_to_string(training_path)?;
    let training_data: TrainingData = serde_json::from_str(&training_json)?;
    Ok(training_data)
}
pub fn list_training_data() -> Result<Vec<TrainingData>> {
    let training_dir = get_training_dir()?;
    let mut training_data = Vec::new();
    if training_dir.exists() {
        for entry in fs::read_dir(training_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let training_json = fs::read_to_string(&path)?;
                if let Ok(data) = serde_json::from_str::<TrainingData>(&training_json) {
                    training_data.push(data);
                }
            }
        }
    }
    Ok(training_data)
}
pub fn delete_training_data(training_id: &str) -> Result<()> {
    let training_dir = get_training_dir()?;
    let training_path = training_dir.join(format!("{}.json", training_id));
    if training_path.exists() {
        fs::remove_file(training_path)?;
    }
    Ok(())
}
pub fn get_chunks_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("chunks"))
}
pub fn save_code_chunk(chunk: &CodeChunk) -> Result<()> {
    let chunks_dir = get_chunks_dir()?;
    fs::create_dir_all(&chunks_dir)?;
    let chunk_path = chunks_dir.join(format!("{}.json", chunk.chunk_id));
    let chunk_json = serde_json::to_string_pretty(chunk)?;
    fs::write(chunk_path, chunk_json)?;
    Ok(())
}
pub fn load_code_chunk(chunk_id: &str) -> Result<CodeChunk> {
    let chunks_dir = get_chunks_dir()?;
    let chunk_path = chunks_dir.join(format!("{}.json", chunk_id));
    let chunk_json = fs::read_to_string(chunk_path)?;
    let chunk: CodeChunk = serde_json::from_str(&chunk_json)?;
    Ok(chunk)
}
pub fn list_code_chunks() -> Result<Vec<CodeChunk>> {
    let chunks_dir = get_chunks_dir()?;
    let mut chunks = Vec::new();
    if chunks_dir.exists() {
        for entry in fs::read_dir(chunks_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let chunk_json = fs::read_to_string(&path)?;
                if let Ok(chunk) = serde_json::from_str::<CodeChunk>(&chunk_json) {
                    chunks.push(chunk);
                }
            }
        }
    }
    Ok(chunks)
}
pub fn delete_code_chunk(chunk_id: &str) -> Result<()> {
    let chunks_dir = get_chunks_dir()?;
    let chunk_path = chunks_dir.join(format!("{}.json", chunk_id));
    if chunk_path.exists() {
        fs::remove_file(chunk_path)?;
    }
    Ok(())
}
pub fn get_errors_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("errors"))
}
pub fn save_error(error: &Error) -> Result<()> {
    let errors_dir = get_errors_dir()?;
    fs::create_dir_all(&errors_dir)?;
    let error_path = errors_dir.join(format!("{}.json", error.id));
    let error_json = serde_json::to_string_pretty(error)?;
    fs::write(error_path, error_json)?;
    Ok(())
}
pub fn load_error(error_id: &str) -> Result<Error> {
    let errors_dir = get_errors_dir()?;
    let error_path = errors_dir.join(format!("{}.json", error_id));
    let error_json = fs::read_to_string(error_path)?;
    let error: Error = serde_json::from_str(&error_json)?;
    Ok(error)
}
pub fn list_errors() -> Result<Vec<Error>> {
    let errors_dir = get_errors_dir()?;
    let mut errors = Vec::new();
    if errors_dir.exists() {
        for entry in fs::read_dir(errors_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let error_json = fs::read_to_string(&path)?;
                if let Ok(error) = serde_json::from_str::<Error>(&error_json) {
                    errors.push(error);
                }
            }
        }
    }
    Ok(errors)
}
pub fn delete_error(error_id: &str) -> Result<()> {
    let errors_dir = get_errors_dir()?;
    let error_path = errors_dir.join(format!("{}.json", error_id));
    if error_path.exists() {
        fs::remove_file(error_path)?;
    }
    Ok(())
}
pub fn get_assignments_dir() -> Result<PathBuf> {
    let storage_dir = get_storage_dir()?;
    Ok(storage_dir.join("assignments"))
}
pub fn get_agent_assignments_dir(agent_id: &str) -> Result<PathBuf> {
    let assignments_dir = get_assignments_dir()?;
    Ok(assignments_dir.join(agent_id))
}
pub fn save_agent_assignment(assignment: &AgentAssignment) -> Result<()> {
    let agent_dir = get_agent_assignments_dir(&assignment.agent_id)?;
    fs::create_dir_all(&agent_dir)?;
    let assignment_path = agent_dir.join(format!("{}.json", assignment.task_id));
    let assignment_json = serde_json::to_string_pretty(assignment)?;
    fs::write(assignment_path, assignment_json)?;
    Ok(())
}
pub fn load_agent_assignment(agent_id: &str, task_id: &str) -> Result<AgentAssignment> {
    let agent_dir = get_agent_assignments_dir(agent_id)?;
    let assignment_path = agent_dir.join(format!("{}.json", task_id));
    let assignment_json = fs::read_to_string(assignment_path)?;
    let assignment: AgentAssignment = serde_json::from_str(&assignment_json)?;
    Ok(assignment)
}
pub fn list_agent_assignments(agent_id: &str) -> Result<Vec<AgentAssignment>> {
    let agent_dir = get_agent_assignments_dir(agent_id)?;
    let mut assignments = Vec::new();
    if agent_dir.exists() {
        for entry in fs::read_dir(agent_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let assignment_json = fs::read_to_string(&path)?;
                if let Ok(assignment) = serde_json::from_str::<
                    AgentAssignment,
                >(&assignment_json) {
                    assignments.push(assignment);
                }
            }
        }
    }
    Ok(assignments)
}
pub fn list_all_agent_assignments() -> Result<Vec<AgentAssignment>> {
    let assignments_dir = get_assignments_dir()?;
    let mut all_assignments = Vec::new();
    if assignments_dir.exists() {
        for entry in fs::read_dir(assignments_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(agent_id) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(agent_assignments) = list_agent_assignments(agent_id) {
                        all_assignments.extend(agent_assignments);
                    }
                }
            }
        }
    }
    Ok(all_assignments)
}
pub fn delete_agent_assignment(agent_id: &str, task_id: &str) -> Result<()> {
    let agent_dir = get_agent_assignments_dir(agent_id)?;
    let assignment_path = agent_dir.join(format!("{}.json", task_id));
    if assignment_path.exists() {
        fs::remove_file(assignment_path)?;
    }
    Ok(())
}
pub fn update_agent_assignment_status(
    agent_id: &str,
    task_id: &str,
    status: AssignmentStatus,
) -> Result<()> {
    let mut assignment = load_agent_assignment(agent_id, task_id)?;
    assignment.status = status;
    save_agent_assignment(&assignment)?;
    Ok(())
}
pub fn get_agents_with_assignments() -> Result<Vec<String>> {
    let assignments_dir = get_assignments_dir()?;
    let mut agents = Vec::new();
    if assignments_dir.exists() {
        for entry in fs::read_dir(assignments_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(agent_id) = path.file_name().and_then(|n| n.to_str()) {
                    agents.push(agent_id.to_string());
                }
            }
        }
    }
    Ok(agents)
}
pub fn save_feeling(feeling: &Feeling) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let feelings_dir = storage_dir.join("feelings");
    if !feelings_dir.exists() {
        fs::create_dir_all(&feelings_dir)?;
    }
    let file_path = feelings_dir.join(format!("{}.json", feeling.id));
    let json = serde_json::to_string_pretty(feeling)?;
    fs::write(file_path, json)?;
    Ok(())
}
pub fn load_feeling(id: &str) -> Result<Feeling> {
    let storage_dir = get_storage_dir()?;
    let file_path = storage_dir.join("feelings").join(format!("{}.json", id));
    if !file_path.exists() {
        return Err(TodoziError::FeelingNotFound {
            id: id.to_string(),
        });
    }
    let content = fs::read_to_string(file_path)?;
    let feeling: Feeling = serde_json::from_str(&content)?;
    Ok(feeling)
}
pub fn delete_feeling(id: &str) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let file_path = storage_dir.join("feelings").join(format!("{}.json", id));
    if !file_path.exists() {
        return Err(TodoziError::FeelingNotFound {
            id: id.to_string(),
        });
    }
    fs::remove_file(file_path)?;
    Ok(())
}
pub fn list_feelings() -> Result<Vec<Feeling>> {
    let storage_dir = get_storage_dir()?;
    let feelings_dir = storage_dir.join("feelings");
    if !feelings_dir.exists() {
        fs::create_dir_all(&feelings_dir)?;
        return Ok(Vec::new());
    }
    let mut feelings = Vec::new();
    for entry in fs::read_dir(feelings_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let content = fs::read_to_string(path)?;
            let feeling: Feeling = serde_json::from_str(&content)?;
            feelings.push(feeling);
        }
    }
    feelings.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(feelings)
}
pub fn update_feeling(feeling: &Feeling) -> Result<()> {
    save_feeling(feeling)
}
pub fn save_queue_collection(collection: &QueueCollection) -> Result<()> {
    let storage_dir = get_storage_dir()?;
    let queue_dir = storage_dir.join("queue");
    fs::create_dir_all(&queue_dir)?;
    let file_path = queue_dir.join("queue.json");
    let content = serde_json::to_string_pretty(collection)?;
    fs::write(file_path, content)?;
    Ok(())
}
pub fn load_queue_collection() -> Result<QueueCollection> {
    let storage_dir = get_storage_dir()?;
    let file_path = storage_dir.join("queue").join("queue.json");
    if !file_path.exists() {
        return Ok(QueueCollection::new());
    }
    let content = fs::read_to_string(file_path)?;
    let collection: QueueCollection = serde_json::from_str(&content)?;
    Ok(collection)
}
pub fn add_queue_item(item: QueueItem) -> Result<()> {
    let mut collection = load_queue_collection()?;
    collection.add_item(item);
    save_queue_collection(&collection)?;
    Ok(())
}
pub fn get_queue_item(id: &str) -> Result<QueueItem> {
    let collection = load_queue_collection()?;
    collection
        .get_item(id)
        .ok_or_else(|| TodoziError::ValidationError {
            message: format!("Queue item not found: {}", id),
        })
        .map(|item| (*item).clone())
}
pub fn list_queue_items() -> Result<Vec<QueueItem>> {
    let collection = load_queue_collection()?;
    Ok(collection.get_all_items().into_iter().cloned().collect())
}
pub fn list_queue_items_by_status(status: QueueStatus) -> Result<Vec<QueueItem>> {
    let collection = load_queue_collection()?;
    Ok(collection.get_items_by_status(status).into_iter().cloned().collect())
}
pub fn list_backlog_items() -> Result<Vec<QueueItem>> {
    list_queue_items_by_status(QueueStatus::Backlog)
}
pub fn list_active_items() -> Result<Vec<QueueItem>> {
    list_queue_items_by_status(QueueStatus::Active)
}
pub fn list_complete_items() -> Result<Vec<QueueItem>> {
    list_queue_items_by_status(QueueStatus::Complete)
}
pub fn start_queue_session(queue_item_id: &str) -> Result<String> {
    let mut collection = load_queue_collection()?;
    let session_id = collection.start_session(queue_item_id)?;
    save_queue_collection(&collection)?;
    Ok(session_id)
}
pub fn end_queue_session(session_id: &str) -> Result<()> {
    let mut collection = load_queue_collection()?;
    collection.end_session(session_id)?;
    save_queue_collection(&collection)?;
    Ok(())
}
pub fn get_active_sessions() -> Result<Vec<crate::models::QueueSession>> {
    let collection = load_queue_collection()?;
    Ok(collection.get_active_sessions().into_iter().cloned().collect())
}
pub fn get_queue_session(session_id: &str) -> Result<crate::models::QueueSession> {
    let collection = load_queue_collection()?;
    collection
        .get_session(session_id)
        .ok_or_else(|| TodoziError::ValidationError {
            message: format!("Session not found: {}", session_id),
        })
        .map(|session| (*session).clone())
}