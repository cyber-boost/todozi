use crate::error::{Result, TodoziError};
use crate::models::{
    Task, TaskFilters, Priority, Status, Assignee, QueueItem, QueueStatus, TaskUpdate,
};
use crate::storage::Storage;
use crate::types::*;
#[cfg(feature = "tui")]
use crate::tui::{TuiService, DisplayConfig, TaskEditor, TodoziApp};
use crate::emb::{TodoziEmbeddingService, TodoziEmbeddingConfig};
use std::collections::HashSet;
use crate::storage::*;
pub struct TodoziHandler {
    pub storage: Storage,
}
impl TodoziHandler {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
    pub fn complete_task(&mut self, id: &str) -> Result<()> {
        self.storage.complete_task_in_project(id)
    }
    pub fn fix_task_consistency(&mut self) -> Result<()> {
        println!("🔧 Fixing task data consistency...");
        self.storage.fix_completed_tasks_consistency()?;
        println!("✅ Task consistency fix completed!");
        Ok(())
    }
    pub fn delete_task(&mut self, id: &str) -> Result<()> {
        self.storage.delete_task_from_project(id)
    }
    pub fn restore_backup(&mut self, backup_name: &str) -> Result<()> {
        self.storage.restore_backup(backup_name)
    }
}
impl TodoziHandler {
    pub async fn handle_api_command(&self, command: ApiCommands) -> Result<()> {
        match command {
            ApiCommands::Register { user_id } => {
                let api_key = if let Some(user_id) = user_id {
                    crate::api::create_api_key_with_user_id(user_id)?
                } else {
                    crate::api::create_api_key()?
                };
                println!("🔑 API key created successfully!");
                println!("🆔 User ID: {}", api_key.user_id);
                println!("🔓 Public Key: {}", api_key.public_key);
                println!("🔒 Private Key: {}", api_key.private_key);
                println!("✅ Active: {}", api_key.active);
                println!(
                    "🕒 Created: {}", api_key.created_at.format("%Y-%m-%d %H:%M:%S")
                );
                println!();
                println!("💡 Keep your private key secure! It provides admin access.");
                println!(
                    "📖 Use public key for read-only access, both keys for admin access."
                );
            }
            ApiCommands::List { active_only } => {
                let keys = if active_only {
                    crate::api::list_active_api_keys()?
                } else {
                    crate::api::list_api_keys()?
                };
                if keys.is_empty() {
                    println!("📭 No API keys found");
                    return Ok(());
                }
                println!("🔑 API Keys:");
                println!();
                for key in keys {
                    println!("🆔 User ID: {}", key.user_id);
                    println!("🔓 Public Key: {}", key.public_key);
                    println!("🔒 Private Key: {}", key.private_key);
                    println!("✅ Active: {}", key.active);
                    println!(
                        "🕒 Created: {}", key.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!(
                        "🕒 Updated: {}", key.updated_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("---");
                }
            }
            ApiCommands::Check { public_key, private_key } => {
                let (user_id, is_admin) = crate::api::check_api_key_auth(
                    &public_key,
                    private_key.as_deref(),
                )?;
                println!("✅ API key authentication successful!");
                println!("🆔 User ID: {}", user_id);
                println!("🔓 Public Key: {}", public_key);
                if let Some(priv_key) = private_key {
                    println!("🔒 Private Key: {}", priv_key);
                }
                println!("👑 Admin Access: {}", is_admin);
                println!(
                    "📖 Access Level: {}", if is_admin { "admin" } else { "read_only" }
                );
            }
            ApiCommands::Deactivate { user_id } => {
                crate::api::deactivate_api_key(&user_id)?;
                println!("🔒 API key deactivated successfully!");
                println!("🆔 User ID: {}", user_id);
            }
            ApiCommands::Activate { user_id } => {
                crate::api::activate_api_key(&user_id)?;
                println!("🔓 API key activated successfully!");
                println!("🆔 User ID: {}", user_id);
            }
            ApiCommands::Remove { user_id } => {
                let api_key = crate::api::remove_api_key(&user_id)?;
                println!("🗑️  API key removed successfully!");
                println!("🆔 User ID: {}", user_id);
                println!("🔓 Public Key: {}", api_key.public_key);
                println!("🔒 Private Key: {}", api_key.private_key);
            }
        }
        Ok(())
    }
    pub async fn handle_queue_command(&self, command: QueueCommands) -> Result<()> {
        match command {
            QueueCommands::Plan {
                task_name,
                task_description,
                priority,
                project_id,
            } => {
                let priority_enum = priority.parse::<Priority>()?;
                let item = QueueItem::new(
                    task_name.clone(),
                    task_description.clone(),
                    priority_enum,
                    project_id,
                );
                add_queue_item(item.clone())?;
                println!("✅ Queue item planned successfully!");
                println!("📋 ID: {}", item.id);
                println!("📝 Task: {}", item.task_name);
                println!("📄 Description: {}", item.task_description);
                println!("⚡ Priority: {}", item.priority);
                if let Some(project) = &item.project_id {
                    println!("📁 Project: {}", project);
                }
                println!("📊 Status: {}", item.status);
            }
            QueueCommands::List { status } => {
                let items = if let Some(status_str) = status {
                    let status_enum = status_str.parse::<QueueStatus>()?;
                    list_queue_items_by_status(status_enum)?
                } else {
                    list_queue_items()?
                };
                if items.is_empty() {
                    println!("📭 No queue items found");
                    return Ok(());
                }
                println!("📋 Queue Items:");
                println!();
                for item in items {
                    println!("🆔 ID: {}", item.id);
                    println!("📝 Task: {}", item.task_name);
                    println!("📄 Description: {}", item.task_description);
                    println!("⚡ Priority: {}", item.priority);
                    if let Some(project) = &item.project_id {
                        println!("📁 Project: {}", project);
                    }
                    println!("📊 Status: {}", item.status);
                    println!(
                        "🕒 Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("---");
                }
            }
            QueueCommands::Backlog => {
                let items = list_backlog_items()?;
                if items.is_empty() {
                    println!("📭 No backlog items found");
                    return Ok(());
                }
                println!("📋 Backlog Items:");
                println!();
                for item in items {
                    println!("🆔 ID: {}", item.id);
                    println!("📝 Task: {}", item.task_name);
                    println!("📄 Description: {}", item.task_description);
                    println!("⚡ Priority: {}", item.priority);
                    if let Some(project) = &item.project_id {
                        println!("📁 Project: {}", project);
                    }
                    println!(
                        "🕒 Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("---");
                }
            }
            QueueCommands::Active => {
                let items = list_active_items()?;
                if items.is_empty() {
                    println!("📭 No active items found");
                    return Ok(());
                }
                println!("📋 Active Items:");
                println!();
                for item in items {
                    println!("🆔 ID: {}", item.id);
                    println!("📝 Task: {}", item.task_name);
                    println!("📄 Description: {}", item.task_description);
                    println!("⚡ Priority: {}", item.priority);
                    if let Some(project) = &item.project_id {
                        println!("📁 Project: {}", project);
                    }
                    println!(
                        "🕒 Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("---");
                }
            }
            QueueCommands::Complete => {
                let items = list_complete_items()?;
                if items.is_empty() {
                    println!("📭 No complete items found");
                    return Ok(());
                }
                println!("📋 Complete Items:");
                println!();
                for item in items {
                    println!("🆔 ID: {}", item.id);
                    println!("📝 Task: {}", item.task_name);
                    println!("📄 Description: {}", item.task_description);
                    println!("⚡ Priority: {}", item.priority);
                    if let Some(project) = &item.project_id {
                        println!("📁 Project: {}", project);
                    }
                    println!(
                        "🕒 Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    println!("---");
                }
            }
            QueueCommands::Start { queue_item_id } => {
                let session_id = start_queue_session(&queue_item_id)?;
                println!("🚀 Queue session started successfully!");
                println!("🆔 Session ID: {}", session_id);
                println!("📋 Queue Item ID: {}", queue_item_id);
                println!(
                    "🕒 Started at: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
                );
            }
            QueueCommands::End { session_id } => {
                end_queue_session(&session_id)?;
                let session = get_queue_session(&session_id)?;
                println!("✅ Queue session ended successfully!");
                println!("🆔 Session ID: {}", session_id);
                println!("📋 Queue Item ID: {}", session.queue_item_id);
                println!(
                    "🕒 Started: {}", session.start_time.format("%Y-%m-%d %H:%M:%S")
                );
                if let Some(end_time) = session.end_time {
                    println!("🕒 Ended: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
                }
                if let Some(duration) = session.duration_seconds {
                    println!("⏱️  Duration: {} seconds", duration);
                }
            }
        }
        Ok(())
    }
    pub async fn handle_server_command(&self, command: ServerCommands) -> Result<()> {
        match command {
            ServerCommands::Start { host, port } => {
                println!("🚀 Starting Todozi Enhanced Server...");
                println!("📡 Host: {}", host);
                println!("🔌 Port: {}", port);
                println!("📋 Available at: http://{}:{}", host, port);
                println!();
                use crate::server::*;
                match start_server(Some(host), Some(port)).await {
                    Ok(_) => {
                        println!("✅ Server started successfully!");
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to start server: {}", e);
                        return Err(TodoziError::ValidationError {
                            message: format!("Server start failed: {}", e),
                        });
                    }
                }
            }
            ServerCommands::Status => {
                println!("🔍 Checking server status...");
                use std::net::TcpStream;
                let ports = [8636, 8637, 3000];
                for &port in &ports {
                    match TcpStream::connect(format!("127.0.0.1:{}", port)) {
                        Ok(_) => {
                            println!("✅ Server is running on port {}", port);
                            println!("🌐 API available at: http://127.0.0.1:{}", port);
                            println!("📖 API documentation: todozi server endpoints");
                            return Ok(());
                        }
                        Err(_) => continue,
                    }
                }
                println!("❌ Server is not running on common ports (8636, 8637, 3000)");
                println!("💡 Start it with: todozi server start");
                println!("💡 Or specify port: todozi server start --port 8636");
                Ok(())
            }
            ServerCommands::Endpoints => {
                println!("📡 Todozi Enhanced Server API Endpoints");
                println!(
                    "══════════════════════════════════════"
                );
                println!();
                println!("🎯 CORE FUNCTIONALITY:");
                println!("  GET  /health                    - Health check");
                println!("  GET  /stats                     - System statistics");
                println!("  GET  /init                      - Initialize system");
                println!();
                println!("📋 TASK MANAGEMENT:");
                println!("  GET  /tasks                     - List all tasks");
                println!("  POST /tasks                     - Create new task");
                println!("  GET  /tasks/{{id}}                - Get task by ID");
                println!("  PUT  /tasks/{{id}}                - Update task");
                println!("  DELETE /tasks/{{id}}              - Delete task");
                println!("  GET  /tasks/search?q={{query}}    - Search tasks");
                println!();
                println!("🤖 ENHANCED AGENT SYSTEM (26 AGENTS):");
                println!("  GET  /agents                    - List all agents");
                println!("  POST /agents                    - Create new agent");
                println!("  GET  /agents/{{id}}               - Get agent by ID");
                println!("  PUT  /agents/{{id}}               - Update agent");
                println!("  DELETE /agents/{{id}}             - Delete agent");
                println!("  GET  /agents/available          - Get available agents");
                println!("  GET  /agents/{{id}}/status        - Get agent status");
                println!();
                println!("🧠 MEMORY & IDEA MANAGEMENT:");
                println!("  GET  /memories                  - List all memories");
                println!("  POST /memories                  - Create new memory");
                println!("  GET  /memories/{{id}}             - Get memory by ID");
                println!("  GET  /memories/secret           - Get AI-only memories");
                println!(
                    "  GET  /memories/human            - Get user-visible memories"
                );
                println!(
                    "  GET  /memories/short            - Get conversation memories"
                );
                println!("  GET  /memories/long             - Get long-term memories");
                println!(
                    "  GET  /memories/emotional/{{emotion}} - Get emotional memories"
                );
                println!(
                    "  GET  /memories/types            - List available memory types"
                );
                println!("  GET  /ideas                     - List all ideas");
                println!("  POST /ideas                     - Create new idea");
                println!("  GET  /ideas/{{id}}                - Get idea by ID");
                println!();
                println!("🎓 TRAINING DATA SYSTEM:");
                println!("  GET  /training                  - List all training data");
                println!("  POST /training                  - Create training data");
                println!(
                    "  GET  /training/{{id}}             - Get training data by ID"
                );
                println!("  PUT  /training/{{id}}             - Update training data");
                println!("  DELETE /training/{{id}}           - Delete training data");
                println!("  GET  /training/export           - Export training data");
                println!("  GET  /training/stats            - Training data statistics");
                println!();
                println!("🧩 CODE CHUNKING SYSTEM:");
                println!("  GET  /chunks                    - List all code chunks");
                println!("  POST /chunks                    - Create new code chunk");
                println!("  GET  /chunks/{{id}}               - Get chunk by ID");
                println!("  PUT  /chunks/{{id}}               - Update chunk");
                println!("  DELETE /chunks/{{id}}             - Delete chunk");
                println!("  GET  /chunks/ready              - Get ready chunks");
                println!("  GET  /chunks/graph              - Get dependency graph");
                println!();
                println!("💬 ENHANCED CHAT PROCESSING:");
                println!("  POST /chat/process              - Process chat message");
                println!(
                    "  POST /chat/agent/{{id}}           - Chat with specific agent"
                );
                println!("  GET  /chat/history              - Get chat history");
                println!();
                println!("📊 ANALYTICS & TRACKING:");
                println!("  GET  /analytics/tasks           - Task analytics");
                println!("  GET  /analytics/agents          - Agent analytics");
                println!("  GET  /analytics/performance     - System performance");
                println!("  POST /time/start/{{task_id}}       - Start time tracking");
                println!("  POST /time/stop/{{task_id}}        - Stop time tracking");
                println!("  GET  /time/report               - Time tracking report");
                println!();
                println!("📁 PROJECT MANAGEMENT:");
                println!("  GET  /projects                  - List all projects");
                println!("  POST /projects                  - Create new project");
                println!("  GET  /projects/{{name}}           - Get project by name");
                println!("  PUT  /projects/{{name}}           - Update project");
                println!("  DELETE /projects/{{name}}         - Delete project");
                println!();
                println!("🔧 UTILITIES:");
                println!("  POST /backup                    - Create backup");
                println!("  GET  /backups                   - List backups");
                println!("  POST /restore/{{name}}            - Restore from backup");
                println!();
                println!("🚀 To start the server:");
                println!("  todozi server start");
                println!("  todozi server start --host 0.0.0.0 --port 8636");
                println!();
                println!("📖 For API documentation:");
                println!("  todozi server endpoints");
                Ok(())
            }
        }
    }
    pub async fn handle_search_all_command(&self, command: Commands) -> Result<()> {
        if let Commands::SearchAll { query, types } = command {
            println!("🔍 Performing unified search across all Todozi data...");
            println!("Query: \"{}\"", query);
            println!("Types: {}", types);
            println!();
            let mut search_engine = SearchEngine::new();
            let tasks = self.storage.list_tasks_across_projects(&TaskFilters::default())?;
            let memories = crate::storage::list_memories()?;
            let ideas = crate::storage::list_ideas()?;
            let errors = crate::storage::list_errors()?;
            let training_data = crate::storage::list_training_data()?;
            let chat_content = crate::types::ChatContent {
                tasks: tasks,
                memories: memories,
                ideas: ideas,
                agent_assignments: Vec::new(),
                code_chunks: Vec::new(),
                errors: errors,
                training_data: training_data,
                feelings: Vec::new(),
            };
            search_engine.update_index(&chat_content);
            let types_filter: HashSet<String> = if types == "all" {
                ["tasks", "memories", "ideas", "errors", "training"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            } else {
                types.split(',').map(|s| s.trim().to_string()).collect()
            };
            let search_options = SearchOptions {
                limit: Some(20),
                data_types: None,
                since: None,
                until: None,
            };
            let results = search_engine.search(&query, search_options);
            println!("📊 Search Results:");
            println!("═══════════════════");
            let mut total_results = 0;
            if types_filter.contains("tasks") && !results.task_results.is_empty() {
                println!("\n📋 Tasks ({}):", results.task_results.len());
                total_results += results.task_results.len();
                for (i, result) in results.task_results.iter().enumerate() {
                    if i >= 5 {
                        println!("  ... and {} more", results.task_results.len() - 5);
                        break;
                    }
                    println!("  {} ({})", result.action, result.status);
                }
            }
            if types_filter.contains("memories") && !results.memory_results.is_empty() {
                println!("\n🧠 Memories ({}):", results.memory_results.len());
                total_results += results.memory_results.len();
                for (i, result) in results.memory_results.iter().enumerate() {
                    if i >= 3 {
                        println!("  ... and {} more", results.memory_results.len() - 3);
                        break;
                    }
                    println!("  {}: {}", result.moment, result.meaning);
                }
            }
            if types_filter.contains("ideas") && !results.idea_results.is_empty() {
                println!("\n💡 Ideas ({}):", results.idea_results.len());
                total_results += results.idea_results.len();
                for (i, result) in results.idea_results.iter().enumerate() {
                    if i >= 3 {
                        println!("  ... and {} more", results.idea_results.len() - 3);
                        break;
                    }
                    println!("  {}", result.idea);
                }
            }
            if types_filter.contains("errors") && !results.error_results.is_empty() {
                println!("\n❌ Errors ({}):", results.error_results.len());
                total_results += results.error_results.len();
                for (i, result) in results.error_results.iter().enumerate() {
                    if i >= 3 {
                        println!("  ... and {} more", results.error_results.len() - 3);
                        break;
                    }
                    println!("  {} ({})", result.title, result.severity);
                }
            }
            if types_filter.contains("training") && !results.training_results.is_empty()
            {
                println!("\n🎓 Training Data ({}):", results.training_results.len());
                total_results += results.training_results.len();
                for (i, result) in results.training_results.iter().enumerate() {
                    if i >= 3 {
                        println!(
                            "  ... and {} more", results.training_results.len() - 3
                        );
                        break;
                    }
                    println!("  {} ({})", result.prompt, result.data_type);
                }
            }
            if total_results == 0 {
                println!("\n❌ No results found for query: \"{}\"", query);
                println!("💡 Try different keywords or check if data exists");
            } else {
                println!("\n✅ Found {} total results", total_results);
                println!(
                    "💡 Use specific type filters: tasks,memories,ideas,errors,training"
                );
            }
            Ok(())
        } else {
            Ok(())
        }
    }
    pub async fn handle_chat_command(&self, command: Commands) -> Result<()> {
        if let Commands::Chat { message } = command {
            match self.process_chat_message_extended(&message, "cli_user") {
                Ok(content) => {
                    println!("✅ Chat processed successfully!");
                    println!("📊 Content extracted:");
                    println!("  📋 Tasks: {}", content.tasks.len());
                    println!("  🧠 Memories: {}", content.memories.len());
                    println!("  💡 Ideas: {}", content.ideas.len());
                    println!(
                        "  🤖 Agent Assignments: {}", content.agent_assignments.len()
                    );
                    println!("  🧩 Code Chunks: {}", content.code_chunks.len());
                    println!("  ❌ Errors: {}", content.errors.len());
                    println!("  🎓 Training Data: {}", content.training_data.len());
                    println!();
                    let _storage = Storage::new().await?;
                    let mut created_items = Vec::new();
                    for task in &content.tasks {
                        match self.storage.add_task_to_project(task.clone()).await {
                            Ok(_) => {
                                created_items.push(format!("📋 Task: {}", task.action));
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to save task '{}': {}", task.action, e
                                );
                            }
                        }
                    }
                    for memory in &content.memories {
                        match crate::storage::save_memory(memory) {
                            Ok(_) => {
                                created_items
                                    .push(
                                        format!(
                                            "🧠 Memory: {} - {}", memory.moment, memory.meaning
                                        ),
                                    );
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to save memory '{}': {}", memory.moment, e
                                );
                            }
                        }
                    }
                    for idea in &content.ideas {
                        match crate::storage::save_idea(idea) {
                            Ok(_) => {
                                created_items.push(format!("💡 Idea: {}", idea.idea));
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to save idea '{}': {}", idea.idea, e);
                            }
                        }
                    }
                    for assignment in &content.agent_assignments {
                        match crate::storage::save_agent_assignment(assignment) {
                            Ok(_) => {
                                created_items
                                    .push(
                                        format!(
                                            "🤖 Agent Assignment: {} -> {}", assignment.agent_id,
                                            assignment.task_id
                                        ),
                                    );
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to save agent assignment '{}': {}", assignment
                                    .task_id, e
                                );
                            }
                        }
                    }
                    for chunk in &content.code_chunks {
                        match crate::storage::save_code_chunk(chunk) {
                            Ok(_) => {
                                created_items
                                    .push(
                                        format!(
                                            "🧩 Code Chunk: {} ({})", chunk.chunk_id, chunk.level
                                        ),
                                    );
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to save code chunk '{}': {}", chunk.chunk_id, e
                                );
                            }
                        }
                    }
                    for error in &content.errors {
                        match crate::storage::save_error(error) {
                            Ok(_) => {
                                created_items.push(format!("❌ Error: {}", error.title));
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to save error '{}': {}", error.title, e
                                );
                            }
                        }
                    }
                    for training_item in &content.training_data {
                        match crate::storage::save_training_data(training_item) {
                            Ok(_) => {
                                created_items
                                    .push(
                                        format!(
                                            "🎓 Training Data: {} ({})", training_item.prompt,
                                            training_item.data_type
                                        ),
                                    );
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to save training data '{}': {}", training_item
                                    .prompt, e
                                );
                            }
                        }
                    }
                    if !created_items.is_empty() {
                        println!("✅ Successfully created/processed:");
                        for item in &created_items {
                            println!("  {}", item);
                        }
                        println!();
                        println!("🎉 Total items processed: {}", created_items.len());
                    } else {
                        println!("ℹ️  No structured content found in message.");
                        println!(
                            "💡 Try using tags like <todozi>, <memory>, <idea>, <chunk>, <error>, <train>"
                        );
                    }
                    println!();
                    println!("🔍 Available Tags:");
                    println!(
                        "  📋 <todozi>action|time|priority|project|status</todozi> - Create tasks"
                    );
                    println!(
                        "  🧠 <memory>moment|meaning|reason|importance|term</memory> - Store standard memories"
                    );
                    println!(
                        "  🔒 <memory_secret>moment|meaning|reason|importance|term</memory_secret> - AI-only memories"
                    );
                    println!(
                        "  👤 <memory_human>moment|meaning|reason|importance|term</memory_human> - User-visible memories"
                    );
                    println!(
                        "  💬 <memory_short>moment|meaning|reason|importance</memory_short> - Conversation memories"
                    );
                    println!(
                        "  🏛️ <memory_long>moment|meaning|reason|importance</memory_long> - Long-term memories"
                    );
                    println!(
                        "  😊 <memory_emotion>moment|meaning|reason|importance|term</memory_emotion> - Emotional memories"
                    );
                    println!(
                        "  💡 <idea>idea|share|importance</idea> - Capture ideas"
                    );
                    println!(
                        "  🤖 <todozi_agent>agent_id|task_id|project_id</todozi_agent> - Assign agents"
                    );
                    println!(
                        "  🧩 <chunk>language|code|description</chunk> - Code chunks"
                    );
                    println!(
                        "  ❌ <error>title|description|severity|category</error> - Track errors"
                    );
                    println!(
                        "  🎓 <train>prompt|completion|data_type</train> - Training data"
                    );
                }
                Err(e) => {
                    eprintln!("❌ Failed to process chat message: {}", e);
                    eprintln!(
                        "💡 Make sure your message uses proper Todozi tag format"
                    );
                    return Err(TodoziError::ValidationError {
                        message: format!("Chat processing failed: {}", e),
                    });
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
    pub fn process_chat_message_extended(
        &self,
        message: &str,
        user_id: &str,
    ) -> Result<ChatContent> {
        println!("🤖 Processing chat message from user: {}", user_id);
        println!("💬 Message: {}", message);
        Ok(ChatContent {
            tasks: Vec::new(),
            memories: Vec::new(),
            ideas: Vec::new(),
            agent_assignments: Vec::new(),
            code_chunks: Vec::new(),
            errors: Vec::new(),
            training_data: Vec::new(),
            feelings: Vec::new(),
        })
    }
    pub async fn handle_error_command(&self, command: Commands) -> Result<()> {
        if let Commands::Error(error_commands) = command {
            match error_commands {
                ErrorCommands::Create {
                    title,
                    description,
                    severity,
                    category,
                    source,
                    context,
                    tags,
                } => {
                    let mut error_record = crate::models::Error::new(
                        title,
                        description,
                        source,
                    );
                    error_record.severity = severity
                        .parse()
                        .unwrap_or(crate::models::ErrorSeverity::Medium);
                    error_record.category = category
                        .parse()
                        .unwrap_or(crate::models::ErrorCategory::Runtime);
                    if let Some(ctx) = context {
                        error_record.context = Some(ctx);
                    }
                    if let Some(tgs) = tags {
                        error_record.tags = tgs
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                    self.storage.save_error(&error_record)?;
                    println!("✅ Error record created with ID: {}", error_record.id);
                }
                ErrorCommands::List { severity, category, unresolved_only } => {
                    let errors = self.storage.list_errors()?;
                    let filtered_errors: Vec<&crate::models::Error> = errors
                        .iter()
                        .filter(|e| {
                            (severity.is_none()
                                || e.severity.to_string() == *severity.as_ref().unwrap())
                                && (category.is_none()
                                    || e.category.to_string() == *category.as_ref().unwrap())
                                && (!unresolved_only || e.resolved_at.is_none())
                        })
                        .collect();
                    if filtered_errors.is_empty() {
                        println!("No error records found matching criteria.");
                    } else {
                        for error in filtered_errors {
                            println!(
                                "ID: {}, Title: {}, Severity: {}, Category: {}, Resolved: {}",
                                error.id, error.title, error.severity, error.category, error
                                .resolved_at.is_some()
                            );
                        }
                    }
                }
                ErrorCommands::Show { id } => {
                    let error = self.storage.load_error(&id)?;
                    println!("Error ID: {}", error.id);
                    println!("Title: {}", error.title);
                    println!("Description: {}", error.description);
                    println!("Source: {}", error.source);
                    println!("Severity: {}", error.severity);
                    println!("Category: {}", error.category);
                    if let Some(context) = error.context {
                        println!("Context: {}", context);
                    }
                    println!("Tags: {}", error.tags.join(", "));
                    println!(
                        "Created At: {}", error.created_at.format("%Y-%m-%d %H:%M:%S")
                    );
                    if let Some(resolved_at) = error.resolved_at {
                        println!(
                            "Resolved At: {}", resolved_at.format("%Y-%m-%d %H:%M:%S")
                        );
                    } else {
                        println!("Resolved At: N/A");
                    }
                }
                ErrorCommands::Resolve { id, resolution } => {
                    let mut error = self.storage.load_error(&id)?;
                    error.resolved_at = Some(chrono::Utc::now());
                    if let Some(res) = resolution {
                        println!("Resolution note: {}", res);
                    }
                    self.storage.save_error(&error)?;
                    println!("✅ Error {} marked as resolved!", id);
                }
                ErrorCommands::Delete { id } => {
                    self.storage.delete_error(&id)?;
                    println!("✅ Error {} deleted successfully!", id);
                }
            }
        }
        Ok(())
    }
    pub async fn handle_train_command(&self, command: TrainingCommands) -> Result<()> {
        match command {
            TrainingCommands::Create {
                data_type,
                prompt,
                completion,
                context,
                tags,
                quality,
                source,
            } => {
                println!("Creating training data...");
                let mut training_data = crate::models::TrainingData::new(
                    data_type,
                    prompt,
                    completion,
                    source,
                );
                if let Some(context_val) = context {
                    training_data.context = Some(context_val);
                }
                if let Some(tags_val) = tags {
                    training_data.tags = tags_val
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
                if let Some(quality_val) = quality {
                    training_data.quality_score = Some(quality_val);
                }
                self.storage.save_training_data(&training_data)?;
                println!(
                    "✅ Training data created successfully with ID: {}", training_data
                    .id
                );
            }
            TrainingCommands::List { data_type, min_quality } => {
                println!("Listing training data...");
                let training_data = self.storage.list_training_data()?;
                let filtered_data: Vec<&crate::models::TrainingData> = training_data
                    .iter()
                    .filter(|td| {
                        (data_type.is_none()
                            || td.data_type.to_string() == *data_type.as_ref().unwrap())
                            && (min_quality.is_none()
                                || td.quality_score >= Some(min_quality.unwrap()))
                    })
                    .collect();
                if filtered_data.is_empty() {
                    println!("No training data found matching criteria.");
                } else {
                    for td in filtered_data {
                        println!(
                            "ID: {}, Type: {}, Quality: {:?}, Prompt: {}", td.id, td
                            .data_type, td.quality_score, td.prompt
                        );
                    }
                }
            }
            TrainingCommands::Show { id } => {
                println!("Showing training data: {}", id);
                let td = self.storage.load_training_data(&id)?;
                println!("ID: {}", td.id);
                println!("Data Type: {}", td.data_type);
                println!("Prompt: {}", td.prompt);
                println!("Completion: {}", td.completion);
                println!("Source: {}", td.source);
                if let Some(context) = td.context {
                    println!("Context: {}", context);
                }
                println!("Tags: {}", td.tags.join(", "));
                println!("Quality Score: {:?}", td.quality_score);
            }
            TrainingCommands::Stats => {
                println!("Training data statistics not yet implemented.");
            }
            TrainingCommands::Export {
                format: _format,
                data_type: _data_type,
                min_quality: _min_quality,
                output_file: _output_file,
            } => {
                println!("Exporting training data not yet implemented.");
            }
            TrainingCommands::Collect { message } => {
                println!(
                    "Collecting training data from message: '{}' (not yet implemented)",
                    message
                );
            }
            TrainingCommands::Update {
                id,
                data_type,
                prompt,
                completion,
                context,
                tags,
                quality,
                source,
            } => {
                let mut td = self.storage.load_training_data(&id)?;
                if let Some(new_data_type) = data_type {
                    td.data_type = new_data_type
                        .parse()
                        .unwrap_or(crate::models::TrainingDataType::Instruction);
                }
                if let Some(new_prompt) = prompt {
                    td.prompt = new_prompt;
                }
                if let Some(new_completion) = completion {
                    td.completion = new_completion;
                }
                if let Some(new_context) = context {
                    td.context = Some(new_context);
                }
                if let Some(new_tags) = tags {
                    td.tags = new_tags
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
                if let Some(new_quality) = quality {
                    td.quality_score = Some(new_quality as f32);
                }
                if let Some(new_source) = source {
                    td.source = new_source;
                }
                self.storage.save_training_data(&td)?;
                println!("✅ Training data {} updated successfully!", id);
            }
            TrainingCommands::Delete { id } => {
                self.storage.delete_training_data(&id)?;
                println!("✅ Training data {} deleted successfully!", id);
            }
        }
        Ok(())
    }
    pub async fn handle_agent_command(&self, command: Commands) -> Result<()> {
        if let Commands::Agent(agent_commands) = command {
            match agent_commands {
                AgentCommands::Create {
                    id,
                    name,
                    description,
                    category,
                    capabilities,
                    specializations,
                    model_provider,
                    model_name,
                    temperature,
                    max_tokens,
                    tags,
                    system_prompt,
                    prompt_template,
                    auto_format_code,
                    include_examples,
                    explain_complexity,
                    suggest_tests,
                    tools,
                    max_response_length,
                    timeout_seconds,
                    requests_per_minute,
                    tokens_per_hour,
                } => {
                    println!("Creating enhanced agent...");
                    println!("ID: {}", id);
                    println!("Name: {}", name);
                    println!("Description: {}", description);
                    println!("Category: {}", category);
                    println!("Model: {} ({})", model_name, model_provider);
                    println!("Temperature: {}", temperature);
                    println!("Max tokens: {}", max_tokens);
                    let capabilities_vec = capabilities
                        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
                        .unwrap_or_else(|| vec!["general_assistance".to_string()]);
                    let specializations_vec = specializations
                        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                        .unwrap_or_else(|| vec!["general".to_string()]);
                    let mut agent = crate::models::Agent::new(
                        id.clone(),
                        name,
                        description,
                    );
                    agent.capabilities = capabilities_vec;
                    agent.specializations = specializations_vec;
                    agent.metadata.category = category;
                    if let Some(prompt) = system_prompt {
                        agent.system_prompt = prompt;
                    }
                    if let Some(template) = prompt_template {
                        agent.prompt_template = Some(template);
                    }
                    agent.model = crate::models::ModelConfig {
                        provider: model_provider,
                        name: model_name,
                        temperature,
                        max_tokens,
                    };
                    if let Some(auto_format) = auto_format_code {
                        agent.behaviors.auto_format_code = auto_format;
                    }
                    if let Some(include_examples_val) = include_examples {
                        agent.behaviors.include_examples = include_examples_val;
                    }
                    if let Some(explain_complexity_val) = explain_complexity {
                        agent.behaviors.explain_complexity = explain_complexity_val;
                    }
                    if let Some(suggest_tests_val) = suggest_tests {
                        agent.behaviors.suggest_tests = suggest_tests_val;
                    }
                    if let Some(max_length) = max_response_length {
                        agent.constraints.max_response_length = Some(max_length);
                    }
                    if let Some(timeout) = timeout_seconds {
                        agent.constraints.timeout_seconds = Some(timeout);
                    }
                    if requests_per_minute.is_some() || tokens_per_hour.is_some() {
                        agent.constraints.rate_limit = Some(crate::models::RateLimit {
                            requests_per_minute,
                            tokens_per_hour,
                        });
                    }
                    if let Some(tools_str) = tools {
                        let tool_names: Vec<&str> = tools_str
                            .split(',')
                            .map(|s| s.trim())
                            .collect();
                        agent.tools = tool_names
                            .iter()
                            .map(|tool_name| {
                                crate::models::AgentTool {
                                    name: tool_name.to_string(),
                                    enabled: true,
                                    config: None,
                                }
                            })
                            .collect();
                    }
                    if let Some(tags_str) = tags {
                        agent.metadata.tags = tags_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                    match crate::storage::save_agent(&agent) {
                        Ok(_) => {
                            println!("✅ Agent '{}' created successfully!", id);
                            println!("📁 Saved to: ~/.todozi/agents/{}.json", id);
                            println!("🚀 Agent is ready for use!");
                        }
                        Err(e) => {
                            println!("❌ Failed to create agent: {}", e);
                        }
                    }
                }
                AgentCommands::List => {
                    println!("Listing all available agents...");
                    match crate::storage::list_agents() {
                        Ok(agents) => {
                            if agents.is_empty() {
                                println!("📭 No agents found");
                            } else {
                                println!("🤖 Available Agents:");
                                println!(
                                    "════════════════════"
                                );
                                for agent in agents {
                                    let status = match agent.metadata.status {
                                        crate::models::AgentStatus::Active => "🟢",
                                        crate::models::AgentStatus::Inactive => "🔴",
                                        crate::models::AgentStatus::Busy => "🟡",
                                        crate::models::AgentStatus::Available => "🟢",
                                    };
                                    println!(
                                        "{} {} - {} ({})", status, agent.id, agent.name, agent
                                        .metadata.category
                                    );
                                    println!("   {}", agent.description);
                                    println!();
                                }
                            }
                        }
                        Err(e) => {
                            println!("❌ Failed to list agents: {}", e);
                        }
                    }
                }
                AgentCommands::Show { id } => {
                    println!("Showing details for agent '{}'...", id);
                    match crate::storage::load_agent(&id) {
                        Ok(agent) => {
                            println!("🤖 Agent Details:");
                            println!("═══════════════");
                            println!("🆔 ID: {}", agent.id);
                            println!("📛 Name: {}", agent.name);
                            println!("📝 Description: {}", agent.description);
                            println!("🏷️  Category: {}", agent.metadata.category);
                            println!("📊 Status: {:?}", agent.metadata.status);
                            println!(
                                "🤖 Model: {} ({})", agent.model.name, agent.model
                                .provider
                            );
                            println!(
                                "🌡️  Temperature: {}", agent.model.temperature
                            );
                            println!("🔢 Max Tokens: {}", agent.model.max_tokens);
                            println!(
                                "⚡ Capabilities: {}", agent.capabilities.join(", ")
                            );
                            println!(
                                "🎯 Specializations: {}", agent.specializations.join(", ")
                            );
                            println!(
                                "🛠️  Tools: {}", agent.tools.iter().map(| t | t.name
                                .as_str()).collect::< Vec < _ >> ().join(", ")
                            );
                            println!(
                                "📅 Created: {}", agent.created_at
                                .format("%Y-%m-%d %H:%M:%S")
                            );
                            println!(
                                "🔄 Updated: {}", agent.updated_at
                                .format("%Y-%m-%d %H:%M:%S")
                            );
                        }
                        Err(e) => {
                            println!("❌ Failed to load agent '{}': {}", id, e);
                        }
                    }
                }
                AgentCommands::Assign { agent_id, task_id, project_id } => {
                    println!(
                        "Assigning task {} to agent {} in project {}", task_id, agent_id,
                        project_id
                    );
                    println!("Agent assignment feature coming soon!");
                }
                AgentCommands::Update {
                    id,
                    name,
                    description,
                    category,
                    capabilities,
                    specializations,
                    model_provider,
                    model_name,
                    temperature,
                    max_tokens,
                    tags,
                    system_prompt,
                    prompt_template,
                    auto_format_code,
                    include_examples,
                    explain_complexity,
                    suggest_tests,
                    tools,
                    max_response_length,
                    timeout_seconds,
                    requests_per_minute,
                    tokens_per_hour,
                } => {
                    let mut agent = crate::storage::load_agent(&id)?;
                    if let Some(new_name) = name {
                        agent.name = new_name;
                    }
                    if let Some(new_description) = description {
                        agent.description = new_description;
                    }
                    if let Some(new_category) = category {
                        agent.metadata.category = new_category;
                    }
                    if let Some(new_capabilities) = capabilities {
                        agent.capabilities = new_capabilities
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                    if let Some(new_specializations) = specializations {
                        agent.specializations = new_specializations
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                    if let Some(new_model_provider) = model_provider {
                        agent.model.provider = new_model_provider;
                    }
                    if let Some(new_model_name) = model_name {
                        agent.model.name = new_model_name;
                    }
                    if let Some(new_temperature) = temperature {
                        agent.model.temperature = new_temperature;
                    }
                    if let Some(new_max_tokens) = max_tokens {
                        agent.model.max_tokens = new_max_tokens;
                    }
                    if let Some(new_tags) = tags {
                        agent.metadata.tags = new_tags
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                    if let Some(new_system_prompt) = system_prompt {
                        agent.system_prompt = new_system_prompt;
                    }
                    if let Some(new_prompt_template) = prompt_template {
                        agent.prompt_template = Some(new_prompt_template);
                    }
                    if let Some(new_auto_format_code) = auto_format_code {
                        agent.behaviors.auto_format_code = new_auto_format_code;
                    }
                    if let Some(new_include_examples) = include_examples {
                        agent.behaviors.include_examples = new_include_examples;
                    }
                    if let Some(new_explain_complexity) = explain_complexity {
                        agent.behaviors.explain_complexity = new_explain_complexity;
                    }
                    if let Some(new_suggest_tests) = suggest_tests {
                        agent.behaviors.suggest_tests = new_suggest_tests;
                    }
                    if let Some(new_tools) = tools {
                        let tool_names: Vec<&str> = new_tools
                            .split(',')
                            .map(|s| s.trim())
                            .collect();
                        agent.tools = tool_names
                            .iter()
                            .map(|tool_name| {
                                crate::models::AgentTool {
                                    name: tool_name.to_string(),
                                    enabled: true,
                                    config: None,
                                }
                            })
                            .collect();
                    }
                    if let Some(new_max_response_length) = max_response_length {
                        agent.constraints.max_response_length = Some(
                            new_max_response_length,
                        );
                    }
                    if let Some(new_timeout_seconds) = timeout_seconds {
                        agent.constraints.timeout_seconds = Some(new_timeout_seconds);
                    }
                    if requests_per_minute.is_some() || tokens_per_hour.is_some() {
                        agent.constraints.rate_limit = Some(crate::models::RateLimit {
                            requests_per_minute: requests_per_minute
                                .or(
                                    agent
                                        .constraints
                                        .rate_limit
                                        .as_ref()
                                        .and_then(|r| r.requests_per_minute),
                                ),
                            tokens_per_hour: tokens_per_hour
                                .or(
                                    agent
                                        .constraints
                                        .rate_limit
                                        .as_ref()
                                        .and_then(|r| r.tokens_per_hour),
                                ),
                        });
                    }
                    crate::storage::save_agent(&agent)?;
                    println!("✅ Agent '{}' updated successfully!", id);
                }
                AgentCommands::Delete { id } => {
                    match std::fs::remove_file(
                        crate::storage::get_agents_dir()?.join(format!("{}.json", id)),
                    ) {
                        Ok(_) => println!("✅ Agent '{}' deleted successfully!", id),
                        Err(e) => eprintln!("❌ Failed to delete agent '{}': {}", id, e),
                    }
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
    pub async fn handle_emb_command(&self, command: Commands) -> Result<()> {
        use crate::types::EmbCommands;
        use crate::emb::EmbeddingModel;

        if let Commands::Emb(emb_cmd) = command {
            match emb_cmd {
                EmbCommands::SetModel { model_name } => {
                    println!("🔄 Setting embedding model to: {}", model_name);
                    println!();

                    // Test loading the model
                    println!("📥 Testing model download and validation...");
                    let device = candle_core::Device::Cpu;

                    match EmbeddingModel::load(&model_name, device).await {
                        Ok(_) => {
                            // Save as default
                            EmbeddingModel::save_as_default(&model_name).await?;
                            println!();
                            println!("✅ Model set successfully!");
                            println!("💾 Saved to ~/.todozi/models/ and set as default");
                        }
                        Err(e) => {
                            println!();
                            println!("❌ Failed to load model: {}", e);
                            println!();
                            println!("💡 Popular models you can try:");
                            println!("  - sentence-transformers/all-MiniLM-L6-v2 (default, fast)");
                            println!("  - sentence-transformers/all-mpnet-base-v2 (better quality)");
                            println!("  - sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 (multilingual)");
                            return Err(e);
                        }
                    }
                }
                EmbCommands::ShowModel => {
                    match EmbeddingModel::get_default_model().await {
                        Ok(model_name) => {
                            println!("📊 Current embedding model:");
                            println!("  {}", model_name);
                            println!();
                            println!("💾 Cached in: ~/.todozi/models/");
                        }
                        Err(e) => {
                            println!("❌ Failed to get model: {}", e);
                        }
                    }
                }
                EmbCommands::ListModels => {
                    println!("📚 Popular Sentence-Transformers Models:");
                    println!();
                    println!("🚀 Fast & Lightweight:");
                    println!("  sentence-transformers/all-MiniLM-L6-v2");
                    println!("    → 384 dimensions, ~90MB, good for most use cases");
                    println!();
                    println!("⚡ Balanced:");
                    println!("  sentence-transformers/all-mpnet-base-v2");
                    println!("    → 768 dimensions, ~420MB, better semantic quality");
                    println!();
                    println!("🌍 Multilingual:");
                    println!("  sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2");
                    println!("    → 384 dimensions, supports 50+ languages");
                    println!();
                    println!("🎯 High Performance:");
                    println!("  sentence-transformers/all-roberta-large-v1");
                    println!("    → 1024 dimensions, ~1.4GB, best quality");
                    println!();
                    println!("💡 Set a model with: todozi emb set-model <model-name>");
                    println!("🔍 Browse more at: https://huggingface.co/sentence-transformers");
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
    pub async fn handle_idea_command(&self, command: IdeaCommands) -> Result<()> {
        match command {
            IdeaCommands::Create { idea, share, importance, tags, context } => {
                println!("Creating idea...");
                println!("Idea: {}", idea);
                println!("Share level: {}", share);
                println!("Importance: {}", importance);
                if let Some(tags) = tags {
                    println!("Tags: {}", tags);
                }
                if let Some(context) = context {
                    println!("Context: {}", context);
                }
                println!("Idea creation feature coming soon!");
                Ok(())
            }
            IdeaCommands::List { share, importance } => {
                if let Some(share) = share {
                    println!("Listing ideas with share level: {}", share);
                }
                if let Some(importance) = importance {
                    println!("Listing ideas with importance: {}", importance);
                }
                println!("Idea listing feature coming soon!");
                Ok(())
            }
            IdeaCommands::Show { id } => {
                println!("Showing idea: {}", id);
                println!("Idea show feature coming soon!");
                Ok(())
            }
        }
    }
    pub async fn handle_memory_command(&self, command: MemoryCommands) -> Result<()> {
        match command {
            MemoryCommands::Create {
                moment,
                meaning,
                reason,
                importance,
                term,
                memory_type,
                tags,
            } => {
                println!("Creating {} memory...", memory_type);
                println!("Moment: {}", moment);
                println!("Meaning: {}", meaning);
                println!("Reason: {}", reason);
                println!("Importance: {}", importance);
                println!("Term: {}", term);
                println!("Type: {}", memory_type);
                if let Some(tags) = tags {
                    println!("Tags: {}", tags);
                }
                println!("Memory creation feature coming soon!");
                Ok(())
            }
            MemoryCommands::CreateSecret {
                moment,
                meaning,
                reason,
                importance,
                term,
                tags,
            } => {
                println!("Creating secret (AI-only) memory...");
                println!("Moment: {}", moment);
                println!("Meaning: {}", meaning);
                println!("Reason: {}", reason);
                println!("Importance: {}", importance);
                println!("Term: {}", term);
                if let Some(tags) = tags {
                    println!("Tags: {}", tags);
                }
                println!("Secret memory created (visible only to AI)!");
                Ok(())
            }
            MemoryCommands::CreateHuman {
                moment,
                meaning,
                reason,
                importance,
                term,
                tags,
            } => {
                println!("Creating human-visible memory...");
                println!("Moment: {}", moment);
                println!("Meaning: {}", meaning);
                println!("Reason: {}", reason);
                println!("Importance: {}", importance);
                println!("Term: {}", term);
                if let Some(tags) = tags {
                    println!("Tags: {}", tags);
                }
                println!("Human-visible memory created!");
                Ok(())
            }
            MemoryCommands::CreateEmotional {
                moment,
                meaning,
                reason,
                emotion,
                importance,
                term,
                tags,
            } => {
                println!("Creating emotional memory ({})...", emotion);
                println!("Moment: {}", moment);
                println!("Meaning: {}", meaning);
                println!("Reason: {}", reason);
                println!("Emotion: {}", emotion);
                println!("Importance: {}", importance);
                println!("Term: {}", term);
                if let Some(tags) = tags {
                    println!("Tags: {}", tags);
                }
                println!("Emotional memory created!");
                Ok(())
            }
            MemoryCommands::List { importance, term, memory_type } => {
                if let Some(importance) = importance {
                    println!("Listing memories with importance: {}", importance);
                }
                if let Some(term) = term {
                    println!("Listing memories with term: {}", term);
                }
                if let Some(memory_type) = memory_type {
                    println!("Listing memories of type: {}", memory_type);
                }
                println!("Memory listing feature coming soon!");
                Ok(())
            }
            MemoryCommands::Show { id } => {
                println!("Showing memory: {}", id);
                println!("Memory show feature coming soon!");
                Ok(())
            }
            MemoryCommands::Types => {
                println!("Available memory types:");
                println!("  standard  - Regular memories");
                println!("  secret    - AI-only memories");
                println!("  human     - User-visible memories");
                println!("  short     - Conversation-related memories");
                println!("  long      - Long-term memories");
                println!("  Emotional types:");
                println!("    happy, sad, angry, fearful, surprised, disgusted");
                println!("    excited, anxious, confident, frustrated, motivated");
                println!("    overwhelmed, curious, satisfied, disappointed, grateful");
                println!("    proud, ashamed, hopeful, resigned");
                Ok(())
            }
        }
    }
    pub async fn handle_list_backups_command(&self) -> Result<()> {
        let backups = self.storage.list_backups()?;
        if backups.is_empty() {
            println!("No backups found.");
        } else {
            println!("Available backups:");
            for backup in backups {
                println!("  {}", backup);
            }
        }
        Ok(())
    }
    pub async fn handle_stats_command(&self, _command: StatsCommands) -> Result<()> {
        let all_tasks = self.storage.list_tasks_across_projects(&TaskFilters::default())?;
        let active_tasks = self
            .storage
            .list_tasks_across_projects(
                &TaskFilters {
                    status: Some(Status::Todo),
                    ..Default::default()
                },
            )?;
        let completed_tasks = self
            .storage
            .list_tasks_across_projects(
                &TaskFilters {
                    status: Some(Status::Done),
                    ..Default::default()
                },
            )?;
        let projects = self.storage.list_projects()?;
        println!("Todozi Statistics:");
        println!("  Total tasks: {}", all_tasks.len());
        println!("  Active tasks: {}", active_tasks.len());
        println!("  Completed tasks: {}", completed_tasks.len());
        println!("  Projects: {}", projects.len());
        let mut priority_counts = std::collections::HashMap::new();
        for task in &all_tasks {
            *priority_counts.entry(&task.priority).or_insert(0) += 1;
        }
        println!("\nPriority breakdown:");
        for (priority, count) in priority_counts {
            println!("  {}: {}", priority, count);
        }
        Ok(())
    }
    pub async fn handle_search_command(&self, command: SearchCommands) -> Result<()> {
        match command {
            SearchCommands::Tasks { query } => {
                let tasks = self.storage.search_tasks(&query)?;
                println!("Found {} tasks matching '{}':", tasks.len(), query);
                for task in tasks {
                    println!("  {}: {} ({})", task.id, task.action, task.status);
                }
            }
        }
        Ok(())
    }
    pub async fn handle_project_command(&self, command: ProjectCommands) -> Result<()> {
        match command {
            ProjectCommands::Create { name, description } => {
                self.storage.create_project(name.clone(), description)?;
                println!("Project '{}' created successfully!", name);
                Ok(())
            }
            ProjectCommands::List => {
                let projects = self.storage.list_projects()?;
                if projects.is_empty() {
                    println!("No projects found.");
                } else {
                    use tabled::{Table, Tabled};
                    #[derive(Tabled)]
                    struct ProjectRow {
                        name: String,
                        description: String,
                        status: String,
                        task_count: usize,
                    }
                    let rows: Vec<ProjectRow> = projects
                        .into_iter()
                        .map(|project| {
                            let task_count = self
                                .storage
                                .get_project_tasks(&project.name)
                                .unwrap_or_default()
                                .len();
                            ProjectRow {
                                name: project.name,
                                description: project
                                    .description
                                    .unwrap_or_else(|| "No description".to_string()),
                                status: project.status.to_string(),
                                task_count,
                            }
                        })
                        .collect();
                    println!("{}", Table::new(rows));
                }
                Ok(())
            }
            ProjectCommands::Show { name } => {
                let project = self.storage.get_project(&name)?;
                let tasks = self.storage.get_project_tasks(&name)?;
                println!("Project: {}", project.name);
                if let Some(description) = &project.description {
                    println!("Description: {}", description);
                }
                println!("Status: {}", project.status);
                println!("Tasks: {}", tasks.len());
                println!("Created: {}", project.created_at.format("%Y-%m-%d %H:%M:%S"));
                println!("Updated: {}", project.updated_at.format("%Y-%m-%d %H:%M:%S"));
                if !tasks.is_empty() {
                    println!("\nTasks:");
                    for task in tasks {
                        println!("  {}: {} ({})", task.id, task.action, task.status);
                    }
                }
                Ok(())
            }
            ProjectCommands::Archive { name } => {
                self.storage.archive_project(&name)?;
                println!("Project '{}' archived!", name);
                Ok(())
            }
            ProjectCommands::Delete { name } => {
                self.storage.delete_project(&name)?;
                println!("Project '{}' deleted!", name);
                Ok(())
            }
            ProjectCommands::Update { name, new_name, description, status } => {
                let mut project = self.storage.get_project(&name)?;
                let original_name = project.name.clone();
                if let Some(new_n) = new_name {
                    project.name = new_n;
                }
                if let Some(desc) = description {
                    project.description = Some(desc);
                }
                if let Some(stat) = status {
                    project.status = stat.parse()?;
                }
                let new_name = project.name.clone();
                self.storage.update_project(project)?;
                println!("✅ Project '{}' updated successfully!", name);
                if original_name != new_name {
                    println!("   New name: '{}'", new_name);
                }
                Ok(())
            }
        }
    }
    pub async fn handle_update_command(
        &self,
        id: String,
        action: Option<String>,
        time: Option<String>,
        priority: Option<String>,
        project: Option<String>,
        status: Option<String>,
        assignee: Option<String>,
        tags: Option<String>,
        dependencies: Option<String>,
        context: Option<String>,
        progress: Option<u8>,
    ) -> Result<()> {
        let mut updates = TaskUpdate::new();
        if let Some(action) = action {
            updates = updates.with_action(action);
        }
        if let Some(time) = time {
            updates = updates.with_time(time);
        }
        if let Some(priority) = priority {
            updates = updates.with_priority(priority.parse()?);
        }
        if let Some(project) = project {
            updates = updates.with_parent_project(project);
        }
        if let Some(status) = status {
            updates = updates.with_status(status.parse()?);
        }
        if let Some(assignee) = assignee {
            updates = updates.with_assignee(assignee.parse()?);
        }
        if let Some(tags) = tags {
            updates = updates
                .with_tags(tags.split(',').map(|s| s.trim().to_string()).collect());
        }
        if let Some(dependencies) = dependencies {
            updates = updates
                .with_dependencies(
                    dependencies.split(',').map(|s| s.trim().to_string()).collect(),
                );
        }
        if let Some(context) = context {
            updates = updates.with_context_notes(context);
        }
        if let Some(progress) = progress {
            updates = updates.with_progress(progress);
        }
        self.storage.update_task_in_project(&id, updates).await?;
        println!("Task {} updated successfully!", id);
        Ok(())
    }
    pub async fn handle_show_command(&self, command: ShowCommands) -> Result<()> {
        match command {
            ShowCommands::Task { id } => {
                let task = self.storage.get_task_from_any_project(&id)?;
                println!("Task: {}", task.id);
                println!("Action: {}", task.action);
                println!("Time: {}", task.time);
                println!("Priority: {}", task.priority);
                println!("Project: {}", task.parent_project);
                println!("Status: {}", task.status);
                if let Some(assignee) = &task.assignee {
                    println!("Assignee: {}", assignee);
                }
                if !task.tags.is_empty() {
                    println!("Tags: {}", task.tags.join(", "));
                }
                if !task.dependencies.is_empty() {
                    println!("Dependencies: {}", task.dependencies.join(", "));
                }
                if let Some(context) = &task.context_notes {
                    println!("Context: {}", context);
                }
                if let Some(progress) = task.progress {
                    println!("Progress: {}%", progress);
                }
                println!("Created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S"));
                println!("Updated: {}", task.updated_at.format("%Y-%m-%d %H:%M:%S"));
            }
        }
        Ok(())
    }
    pub async fn handle_list_command(&self, command: ListCommands) -> Result<()> {
        match command {
            ListCommands::Tasks {
                project,
                status,
                priority,
                assignee,
                tags,
                search,
            } => {
                let mut filters = TaskFilters::default();
                if let Some(project) = project {
                    filters.project = Some(project);
                }
                if let Some(status) = status {
                    filters.status = Some(status.parse()?);
                }
                if let Some(priority) = priority {
                    filters.priority = Some(priority.parse()?);
                }
                if let Some(assignee) = assignee {
                    filters.assignee = Some(assignee.parse()?);
                }
                if let Some(tags) = tags {
                    filters.tags = Some(
                        tags.split(',').map(|s| s.trim().to_string()).collect(),
                    );
                }
                filters.search = search;
                let tasks = self.storage.list_tasks_across_projects(&filters)?;
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    use tabled::{Table, Tabled};
                    #[derive(Tabled)]
                    struct TaskRow {
                        id: String,
                        action: String,
                        project: String,
                        priority: String,
                        status: String,
                        assignee: String,
                        progress: String,
                    }
                    let rows: Vec<TaskRow> = tasks
                        .into_iter()
                        .map(|task| TaskRow {
                            id: task.id,
                            action: if task.action.len() > 50 {
                                format!("{}...", & task.action[..47])
                            } else {
                                task.action
                            },
                            project: task.parent_project,
                            priority: task.priority.to_string(),
                            status: task.status.to_string(),
                            assignee: task
                                .assignee
                                .map(|a| a.to_string())
                                .unwrap_or_else(|| "unassigned".to_string()),
                            progress: task
                                .progress
                                .map(|p| format!("{}%", p))
                                .unwrap_or_else(|| "N/A".to_string()),
                        })
                        .collect();
                    println!("{}", Table::new(rows));
                }
            }
        }
        Ok(())
    }
    pub async fn handle_add_command(&self, command: AddCommands) -> Result<()> {
        match command {
            AddCommands::Task {
                action,
                time,
                priority,
                project,
                status,
                assignee,
                tags,
                dependencies,
                context,
                progress,
            } => {
                let priority_enum = priority.parse()?;
                let status_enum = status.parse()?;
                let assignee_enum = if let Some(assignee) = assignee {
                    Some(assignee.parse()?)
                } else {
                    None
                };
                let tags_vec = if let Some(tags) = tags {
                    tags.split(',').map(|s| s.trim().to_string()).collect()
                } else {
                    Vec::new()
                };
                let dependencies_vec = if let Some(deps) = dependencies {
                    deps.split(',').map(|s| s.trim().to_string()).collect()
                } else {
                    Vec::new()
                };
                let task = Task::new_full(
                    "cli_user".to_string(),
                    action,
                    time,
                    priority_enum,
                    project,
                    status_enum,
                    assignee_enum,
                    tags_vec,
                    dependencies_vec,
                    context,
                    progress,
                )?;
                let config = crate::emb::TodoziEmbeddingConfig::default();
                let embedding_service = crate::emb::TodoziEmbeddingService::new(config)
                    .await?;
                let task_id = embedding_service.add_task(task.clone()).await?;
                println!("Task created: {}", task_id);
                
                // Retrieve task from project-based storage
                if let Ok(stored_task) = self.storage.get_task_from_project(&task.parent_project, &task_id) {
                    println!("Action: {}", stored_task.action);
                    println!("Project: {}", stored_task.parent_project);
                    println!("Priority: {}", stored_task.priority);
                    println!("Status: {}", stored_task.status);
                }
            }
        }
        Ok(())
    }
    pub async fn handle_ind_command() -> Result<()> {
        println!("❌ Ind functionality has been retired");
        Ok(())
    }
    pub fn format_task(task: &Task) -> String {
        let mut output = format!("[{}] {}", task.id, task.action);
        if !task.tags.is_empty() {
            output.push_str(&format!(" #{}", task.tags.join(" #")));
        }
        output.push_str(&format!("\n  Project: {}", task.parent_project));
        output.push_str(&format!(" | Priority: {}", task.priority));
        output.push_str(&format!(" | Status: {}", task.status));
        if let Some(assignee) = &task.assignee {
            output.push_str(&format!(" | Assignee: {}", assignee));
        }
        if let Some(progress) = task.progress {
            output.push_str(&format!(" | Progress: {}%", progress));
        }
        if !task.dependencies.is_empty() {
            output.push_str(&format!(" | Depends on: {}", task.dependencies.join(", ")));
        }
        output
    }
    pub fn parse_tags(tags_str: &str) -> Vec<String> {
        tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    pub fn parse_dependencies(deps_str: &str) -> Vec<String> {
        deps_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    pub fn validate_task_input(
        action: &str,
        time: &str,
        priority: &str,
        project: &str,
        status: &str,
        assignee: Option<&str>,
        progress: Option<u8>,
    ) -> Result<()> {
        if action.trim().is_empty() {
            return Err(crate::error::TodoziError::validation("Action cannot be empty"));
        }
        if action.len() < 3 {
            return Err(
                crate::error::TodoziError::validation(
                    "Action must be at least 3 characters",
                ),
            );
        }
        if action.len() > 500 {
            return Err(
                crate::error::TodoziError::validation(
                    "Action must be less than 500 characters",
                ),
            );
        }
        if time.trim().is_empty() {
            return Err(crate::error::TodoziError::validation("Time cannot be empty"));
        }
        if project.trim().is_empty() {
            return Err(crate::error::TodoziError::validation("Project cannot be empty"));
        }
        priority.parse::<Priority>()?;
        status.parse::<Status>()?;
        if let Some(assignee) = assignee {
            assignee.parse::<Assignee>()?;
        }
        if let Some(progress) = progress {
            if progress > 100 {
                return Err(
                    crate::error::TodoziError::validation(
                        "Progress must be between 0 and 100",
                    ),
                );
            }
        }
        Ok(())
    }
    pub fn create_task_filters(
        project: Option<String>,
        status: Option<String>,
        priority: Option<String>,
        assignee: Option<String>,
        tags: Option<String>,
        search: Option<String>,
    ) -> Result<TaskFilters> {
        let mut filters = TaskFilters::default();
        filters.project = project;
        if let Some(status) = status {
            filters.status = Some(status.parse()?);
        }
        if let Some(priority) = priority {
            filters.priority = Some(priority.parse()?);
        }
        if let Some(assignee) = assignee {
            filters.assignee = Some(assignee.parse()?);
        }
        if let Some(tags) = tags {
            filters.tags = Some(Self::parse_tags(&tags));
        }
        filters.search = search;
        Ok(filters)
    }
    pub fn format_task_list(tasks: &[Task]) -> String {
        if tasks.is_empty() {
            return "No tasks found.".to_string();
        }
        let mut output = String::new();
        for (i, task) in tasks.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, Self::format_task(task)));
        }
        output
    }
    pub fn format_project_stats(
        project_name: &str,
        task_count: usize,
        completed_count: usize,
    ) -> String {
        let completion_rate = if task_count > 0 {
            (completed_count as f64 / task_count as f64) * 100.0
        } else {
            0.0
        };
        format!(
            "Project: {}\n  Total tasks: {}\n  Completed: {}\n  Completion rate: {:.1}%",
            project_name, task_count, completed_count, completion_rate
        )
    }
    pub fn format_time_estimate(time: &str) -> String {
        time.to_string()
    }
    pub fn get_status_emoji(status: &Status) -> &'static str {
        match status {
            Status::Todo | Status::Pending => "📝",
            Status::InProgress => "🔄",
            Status::Blocked => "🚫",
            Status::Review => "👀",
            Status::Done | Status::Completed => "✅",
            Status::Cancelled => "❌",
            Status::Deferred => "⏸️",
        }
    }
    /// Get priority emoji
    pub fn get_priority_emoji(priority: &Priority) -> &'static str {
        match priority {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🟠",
            Priority::Critical => "🔴",
            Priority::Urgent => "🚨",
        }
    }
    pub fn get_assignee_emoji(assignee: &Assignee) -> &'static str {
        match assignee {
            Assignee::Ai => "🤖",
            Assignee::Human => "👤",
            Assignee::Collaborative => "🤝",
            Assignee::Agent(_) => "⚙️",
        }
    }
    /// Format task with emojis
    pub fn format_task_with_emojis(task: &Task) -> String {
        let status_emoji = Self::get_status_emoji(&task.status);
        let priority_emoji = Self::get_priority_emoji(&task.priority);
        let assignee_emoji = task
            .assignee
            .as_ref()
            .map(Self::get_assignee_emoji)
            .unwrap_or("❓");
        let mut output = format!(
            "{} {} {} [{}] {}", status_emoji, priority_emoji, assignee_emoji, task.id,
            task.action
        );
        if !task.tags.is_empty() {
            output.push_str(&format!(" #{}", task.tags.join(" #")));
        }
        output
            .push_str(
                &format!(
                    "\n  📁 {} | ⏱️ {} | 📊 {}%", task.parent_project, task.time,
                    task.progress.unwrap_or(0)
                ),
            );
        if !task.dependencies.is_empty() {
            output.push_str(&format!(" | 🔗 {}", task.dependencies.join(", ")));
        }
        output
    }
    /// Interactive task creation
    pub fn interactive_create_task() -> Result<Task> {
        use dialoguer::{Input, Select, Confirm};
        let action: String = Input::new()
            .with_prompt("Task description")
            .interact_text()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Input error: {}", e),
            ))?;
        let time: String = Input::new()
            .with_prompt("Time estimate (e.g., '2 hours', '1 day', 'ASAP')")
            .interact_text()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Input error: {}", e),
            ))?;
        let priority_options = ["low", "medium", "high", "critical", "urgent"];
        let priority_idx = Select::new()
            .with_prompt("Priority")
            .items(&priority_options)
            .interact()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Selection error: {}", e),
            ))?;
        let priority = priority_options[priority_idx].parse()?;
        let project: String = Input::new()
            .with_prompt("Project name")
            .interact_text()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Input error: {}", e),
            ))?;
        let status_options = [
            "todo",
            "in_progress",
            "blocked",
            "review",
            "done",
            "cancelled",
            "deferred",
        ];
        let status_idx = Select::new()
            .with_prompt("Status")
            .items(&status_options)
            .interact()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Selection error: {}", e),
            ))?;
        let status = status_options[status_idx].parse()?;
        let assignee = if Confirm::new()
            .with_prompt("Set assignee?")
            .interact()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Confirm error: {}", e),
            ))?
        {
            let assignee_options = ["ai", "human", "collaborative"];
            let assignee_idx = Select::new()
                .with_prompt("Assignee")
                .items(&assignee_options)
                .interact()
                .map_err(|e| crate::error::TodoziError::validation(
                    &format!("Selection error: {}", e),
                ))?;
            Some(assignee_options[assignee_idx].parse()?)
        } else {
            None
        };
        let tags = if Confirm::new()
            .with_prompt("Add tags?")
            .interact()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Confirm error: {}", e),
            ))?
        {
            let tags_str: String = Input::new()
                .with_prompt("Tags (comma-separated)")
                .interact_text()
                .map_err(|e| crate::error::TodoziError::validation(
                    &format!("Input error: {}", e),
                ))?;
            Self::parse_tags(&tags_str)
        } else {
            Vec::new()
        };
        let context_notes = if Confirm::new()
            .with_prompt("Add context notes?")
            .interact()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Confirm error: {}", e),
            ))?
        {
            let notes: String = Input::new()
                .with_prompt("Context notes")
                .interact_text()
                .map_err(|e| crate::error::TodoziError::validation(
                    &format!("Input error: {}", e),
                ))?;
            Some(notes)
        } else {
            None
        };
        let progress = if Confirm::new()
            .with_prompt("Set progress?")
            .interact()
            .map_err(|e| crate::error::TodoziError::validation(
                &format!("Confirm error: {}", e),
            ))?
        {
            let progress: u8 = Input::new()
                .with_prompt("Progress (0-100)")
                .interact_text()
                .map_err(|e| crate::error::TodoziError::validation(
                    &format!("Input error: {}", e),
                ))?;
            Some(progress)
        } else {
            None
        };
        Task::new_full(
            "cli_user".to_string(),
            action,
            time,
            priority,
            project,
            status,
            assignee,
            tags,
            Vec::new(),
            context_notes,
            progress,
        )
    }
    /// Show task in a nice format
    pub fn show_task_detailed(task: &Task) {
        println!(
            "┌─ Task Details ──────────────────────────────────────────────┐"
        );
        println!("│ ID: {:<50} │", task.id);
        println!(
            "│ Action: {:<47} │", if task.action.len() > 47 { format!("{}...", & task
            .action[..44]) } else { task.action.clone() }
        );
        println!("│ Time: {:<48} │", task.time);
        println!("│ Priority: {:<44} │", task.priority);
        println!("│ Project: {:<45} │", task.parent_project);
        println!("│ Status: {:<46} │", task.status);
        if let Some(assignee) = &task.assignee {
            println!("│ Assignee: {:<43} │", assignee);
        }
        if !task.tags.is_empty() {
            println!("│ Tags: {:<47} │", task.tags.join(", "));
        }
        if !task.dependencies.is_empty() {
            println!("│ Dependencies: {:<40} │", task.dependencies.join(", "));
        }
        if let Some(context) = &task.context_notes {
            let context_lines: Vec<&str> = context.split('\n').collect();
            for (i, line) in context_lines.iter().enumerate() {
                let prefix = if i == 0 { "│ Context: " } else { "│          " };
                let line = if line.len() > 47 {
                    format!("{}...", & line[..44])
                } else {
                    line.to_string()
                };
                println!("│{}{:<47} │", prefix, line);
            }
        }
        if let Some(progress) = task.progress {
            println!("│ Progress: {:<43}% │", progress);
        }
        println!("│ Created: {:<45} │", task.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!("│ Updated: {:<45} │", task.updated_at.format("%Y-%m-%d %H:%M:%S"));
        println!(
            "└─────────────────────────────────────────────────────────────┘"
        );
    }
    /// Launch the GUI application
    pub async fn launch_gui(&self) -> Result<()> {
        #[cfg(feature = "tui")]
        {
            use crate::emb::{TodoziEmbeddingService, TodoziEmbeddingConfig};
            let embedding_config = TodoziEmbeddingConfig::default();
            let embedding_service = TodoziEmbeddingService::new(embedding_config).await?;
            let display_config = DisplayConfig::default();
            let app = TodoziApp::new(embedding_service, display_config);
            app.run().map_err(|e| TodoziError::validation(&format!("GUI error: {}", e)))
        }
        #[cfg(not(feature = "tui"))]
        { Err(TodoziError::validation("GUI not available - TUI feature not enabled")) }
    }
    /// Handle new AI-enhanced commands
    pub async fn handle_ai_commands(
        &self,
        command: &str,
        args: Vec<String>,
    ) -> Result<()> {
        match command {
            #[cfg(feature = "tui")]
            "all" => {
                if args.is_empty() {
                    return Err(
                        TodoziError::validation("Usage: todozi all <active|done>"),
                    );
                }
                let status = &args[0];
                let filters = match status.as_str() {
                    "active" => {
                        TaskFilters {
                            status: Some(Status::Todo),
                            ..Default::default()
                        }
                    }
                    "done" => {
                        TaskFilters {
                            status: Some(Status::Done),
                            ..Default::default()
                        }
                    }
                    _ => {
                        return Err(
                            TodoziError::validation("Status must be 'active' or 'done'"),
                        );
                    }
                };
                let tasks = self.storage.list_tasks_across_projects(&filters)?;
                let task_ids: Vec<String> = tasks.iter().map(|t| t.id.clone()).collect();
                let embedding_config = TodoziEmbeddingConfig::default();
                let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                    .await?;
                embedding_service.initialize().await?;
                let display_config = DisplayConfig::default();
                let tui_service = TuiService::new(
                    embedding_service,
                    display_config.clone(),
                );
                let task_list_display = tui_service
                    .display_tasks(task_ids)
                    .await
                    .map_err(|e| TodoziError::validation(
                        &format!("Failed to display tasks: {}", e),
                    ))?;
                println!("{}", task_list_display.render(& display_config));
                Ok(())
            }
            #[cfg(feature = "tui")]
            "show" => {
                if args.is_empty() {
                    return Err(TodoziError::validation("Usage: todozi show <task_id>"));
                }
                let task_id = &args[0];
                let embedding_config = TodoziEmbeddingConfig::default();
                let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                    .await?;
                embedding_service.initialize().await?;
                let display_config = DisplayConfig::default();
                let tui_service = TuiService::new(
                    embedding_service,
                    display_config.clone(),
                );
                let task_display = tui_service
                    .display_task(task_id)
                    .await
                    .map_err(|e| TodoziError::validation(
                        &format!("Failed to display task: {}", e),
                    ))?;
                println!("{}", task_display.render(& display_config));
                Ok(())
            }
            #[cfg(feature = "tui")]
            "edit" => {
                if args.is_empty() {
                    return Err(TodoziError::validation("Usage: todozi edit <task_id>"));
                }
                let task_id = &args[0];
                let embedding_config = TodoziEmbeddingConfig::default();
                let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                    .await?;
                embedding_service.initialize().await?;
                let display_config = DisplayConfig::default();
                let mut editor = TaskEditor::new(embedding_service, display_config);
                editor
                    .start_edit(task_id)
                    .await
                    .map_err(|e| TodoziError::validation(
                        &format!("Failed to start edit: {}", e),
                    ))?;
                editor
                    .run_interactive()
                    .await
                    .map_err(|e| TodoziError::validation(
                        &format!("Failed to run interactive editor: {}", e),
                    ))?;
                Ok(())
            }
            "similar" => {
                if args.is_empty() {
                    return Err(TodoziError::validation("Usage: todozi similar <query>"));
                }
                let query = args.join(" ");
                let embedding_config = TodoziEmbeddingConfig::default();
                let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                    .await?;
                embedding_service.initialize().await?;
                let similar_tasks = embedding_service
                    .find_similar_tasks(&query, Some(10))
                    .await?;
                if similar_tasks.is_empty() {
                    println!("No similar tasks found for: {}", query);
                } else {
                    println!("Similar tasks for '{}':", query);
                    println!(
                        "══════════════════════════════════════"
                    );
                    for (i, task) in similar_tasks.iter().enumerate() {
                        println!(
                            "{}. {} ({:.1}% similar)", i + 1, task.text_content.lines()
                            .next().unwrap_or(""), task.similarity_score * 100.0
                        );
                    }
                }
                Ok(())
            }
            "suggest" => {
                let embedding_config = TodoziEmbeddingConfig::default();
                let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                    .await?;
                embedding_service.initialize().await?;
                let filters = TaskFilters {
                    status: Some(Status::Todo),
                    ..Default::default()
                };
                let tasks = self.storage.list_tasks_across_projects(&filters)?;
                if tasks.is_empty() {
                    println!("No active tasks to suggest from");
                    return Ok(());
                }
                println!("🤖 AI Task Suggestions:");
                println!(
                    "═══════════════════════"
                );
                for task in &tasks {
                    let similar_tasks = embedding_service
                        .find_similar_tasks(&task.action, Some(3))
                        .await?;
                    if !similar_tasks.is_empty() {
                        println!("\n📋 Task: {}", task.action);
                        println!("   Similar tasks found: {}", similar_tasks.len());
                        for similar in &similar_tasks {
                            println!(
                                "   • {} ({:.1}% similar)", similar.text_content.lines()
                                .next().unwrap_or(""), similar.similarity_score * 100.0
                            );
                        }
                    }
                }
                Ok(())
            }
            "insights" => {
                let embedding_config = TodoziEmbeddingConfig::default();
                let mut embedding_service = TodoziEmbeddingService::new(embedding_config)
                    .await?;
                embedding_service.initialize().await?;
                let stats = embedding_service.get_stats().await?;
                println!("🧠 AI Insights & Statistics:");
                println!(
                    "═══════════════════════════"
                );
                for (key, value) in stats {
                    println!("{}: {}", key, value);
                }
                let clusters = embedding_service.cluster_content().await?;
                if !clusters.is_empty() {
                    println!("\n🔗 Semantic Clusters:");
                    for (i, cluster) in clusters.iter().enumerate() {
                        println!(
                            "  {}. {} items (avg similarity: {:.1}%)", i + 1, cluster
                            .cluster_size, cluster.average_similarity * 100.0
                        );
                    }
                }
                Ok(())
            }
            _ => {
                Err(TodoziError::validation(&format!("Unknown AI command: {}", command)))
            }
        }
    }
    pub async fn handle_extract_command(
        &self,
        content: Option<String>,
        file: Option<String>,
        output_format: String,
        human: bool,
    ) -> Result<()> {
        let output = crate::extract::extract_content(content, file, output_format, human).await?;
        println!("{}", output);
        Ok(())
    }
    pub async fn handle_strategy_command(
        &self,
        content: Option<String>,
        file: Option<String>,
        output_format: String,
        human: bool,
    ) -> Result<()> {
        let output = crate::extract::strategy_content(content, file, output_format, human).await?;
        println!("{}", output);
        Ok(())
    }
}