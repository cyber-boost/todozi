use async_trait::async_trait;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, Duration};
use regex::Regex;
use reqwest::Client;
use crate::base::*;
use crate::chunking::{ChunkingLevel, ChunkStatus, CodeChunk};
use crate::storage::*;
use crate::models::*;
use crate::error::{TodoziError, Result};
use crate::emb::*;
pub type SharedTodozi = Arc<Mutex<Storage>>;
pub struct CreateTaskTool {
    todozi: SharedTodozi,
}
impl CreateTaskTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
async fn get_todozi_api_key() -> Result<String> {
    use crate::get_tdz_api_key;
    get_tdz_api_key()
        .await
        .map_err(|e| TodoziError::validation(format!("Failed to get API key: {}", e)))
}
async fn make_todozi_request(
    endpoint: &str,
    payload: serde_json::Value,
) -> Result<serde_json::Value> {
    let api_key = get_todozi_api_key().await?;
    let client = Client::new();
    let url = format!("https://todozi.com{}", endpoint);
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| TodoziError::validation(format!("Request failed: {}", e)))?;
    if !response.status().is_success() {
        return Err(
            TodoziError::validation(
                format!("API request failed with status: {}", response.status()),
            ),
        );
    }
    response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| TodoziError::validation(format!("Failed to parse response: {}", e)))
}
#[async_trait]
impl Tool for CreateTaskTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "create_task".to_string(),
            "Create a new task in the Todozi system with automatic AI assignment and queue management"
                .to_string(),
            vec![
                create_tool_parameter("action", "string",
                "Task description/action to perform", true),
                create_tool_parameter("time", "string",
                "Time estimate (e.g., '2 hours', '1 day')", false),
                create_tool_parameter("priority", "string",
                "Priority level (low/medium/high/critical/urgent)", false),
                create_tool_parameter("project", "string",
                "Project name to associate with task", false),
                create_tool_parameter("assignee", "string",
                "Assignee type (ai/human/collaborative)", false),
                create_tool_parameter("tags", "string",
                "Comma-separated tags for the task", false),
                create_tool_parameter("context", "string", "Additional context or notes",
                false),
            ],
            "Task Management".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let action = match kwargs.get("action") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'action' parameter".to_string(),
                    100,
                );
            }
        };
        if action.trim().is_empty() || action.len() > 500 {
            return ToolResult::error("Action must be 1-500 characters".to_string(), 100);
        }
        let assignee_str = kwargs
            .get("assignee")
            .and_then(|v| v.as_str())
            .unwrap_or("human");
        let priority_str = kwargs
            .get("priority")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let context = kwargs.get("context").and_then(|v| v.as_str());
        let project = kwargs.get("project").and_then(|v| v.as_str());
        let priority = match priority_str.parse::<Priority>() {
            Ok(p) => Some(p),
            Err(_) => Some(Priority::Medium),
        };
        let task_id = match (assignee_str, priority_str) {
            ("ai", _) => {
                match crate::Actions::ai(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create AI task: {}", e),
                            100,
                        );
                    }
                }
            }
            ("human", _) => {
                match crate::Actions::human(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create human task: {}", e),
                            100,
                        );
                    }
                }
            }
            ("collaborative", _) => {
                match crate::Actions::collab(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create collaborative task: {}", e),
                            100,
                        );
                    }
                }
            }
            (_, "urgent") => {
                match crate::Tdz::urgent(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create urgent task: {}", e),
                            100,
                        );
                    }
                }
            }
            (_, "critical") => {
                match crate::Tdz::urgent(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create critical task: {}", e),
                            100,
                        );
                    }
                }
            }
            (_, "high") => {
                match crate::Tdz::high(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create high priority task: {}", e),
                            100,
                        );
                    }
                }
            }
            (_, "low") => {
                match crate::Tdz::low(action).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create low priority task: {}", e),
                            100,
                        );
                    }
                }
            }
            _ => {
                match crate::Done::create_task(
                        action,
                        priority,
                        project,
                        kwargs.get("time").and_then(|v| v.as_str()),
                        context,
                    )
                    .await
                {
                    Ok(task) => task.id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create task: {}", e),
                            100,
                        );
                    }
                }
            }
        };
        if let Some(tags_str) = kwargs.get("tags").and_then(|v| v.as_str()) {
            for tag in tags_str.split(',') {
                let tag = tag.trim();
                if !tag.is_empty() {
                    let _ = crate::Tags::add_to_task(&task_id, tag).await;
                }
            }
        }
        ToolResult::success(
            format!(
                "✅ Created task '{}' with ID: {} (queued for {})", action, task_id,
                assignee_str
            ),
            100,
        )
    }
}
pub struct SearchTasksTool {
    todozi: SharedTodozi,
    embedding_service: Option<TodoziEmbeddingService>,
}
impl SearchTasksTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            embedding_service: None,
        }
    }
    pub fn with_embedding_service(mut self, service: TodoziEmbeddingService) -> Self {
        self.embedding_service = Some(service);
        self
    }
    pub fn with_embedding_service_option(
        mut self,
        service: Option<TodoziEmbeddingService>,
    ) -> Self {
        self.embedding_service = service;
        self
    }
}
#[async_trait]
impl Tool for SearchTasksTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "search_tasks".to_string(),
            "Search for tasks in the Todozi system with semantic AI capabilities"
                .to_string(),
            vec![
                create_tool_parameter("query", "string",
                "Search query to match against task content", true),
                create_tool_parameter("semantic", "boolean",
                "Use AI semantic search instead of keyword matching", false),
                create_tool_parameter("project", "string", "Filter by project name",
                false), create_tool_parameter("status", "string",
                "Filter by status (todo/in_progress/blocked/review/done)", false),
                create_tool_parameter("assignee", "string",
                "Filter by assignee (ai/human/collaborative)", false),
                create_tool_parameter("limit", "number",
                "Maximum number of results to return", false),
            ],
            "Task Management".to_string(),
            vec![ResourceLock::FilesystemRead],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let query = match kwargs.get("query") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'query' parameter".to_string(),
                    150,
                );
            }
        };
        if query.trim().is_empty() || query.len() > 100 {
            return ToolResult::error("Query must be 1-100 characters".to_string(), 150);
        }
        let semantic = kwargs.get("semantic").and_then(|v| v.as_bool()).unwrap_or(false);
        if semantic {
            match crate::Find::deep(query).await {
                Ok(ai_results) => {
                    if ai_results.is_empty() {
                        return ToolResult::success(
                            format!("🤖 No AI semantic results found for: {}", query),
                            150,
                        );
                    }
                    let results = ai_results
                        .iter()
                        .map(|result| {
                            format!(
                                "ID: {} | {} | Similarity: {:.2} | Type: {:?}", result
                                .content_id, result.text_content, result.similarity_score,
                                result.content_type
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    ToolResult::success(
                        format!(
                            "🤖 AI Semantic Search - Found {} results:\n{}", ai_results
                            .len(), results
                        ),
                        150,
                    )
                }
                Err(e) => {
                    ToolResult::error(format!("AI semantic search failed: {}", e), 150)
                }
            }
        } else {
            match crate::Find::fast(query).await {
                Ok(results) => {
                    if results.is_empty() || results.contains("No keyword results") {
                        ToolResult::success(
                            format!("🔍 No keyword results found for: {}", query),
                            150,
                        )
                    } else {
                        ToolResult::success(
                            format!("🔍 Keyword Search Results:\n{}", results),
                            150,
                        )
                    }
                }
                Err(e) => ToolResult::error(format!("Keyword search failed: {}", e), 150),
            }
        }
    }
}
pub struct UpdateTaskTool {
    todozi: SharedTodozi,
}
impl UpdateTaskTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for UpdateTaskTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "update_task".to_string(),
            "Update an existing task in the Todozi system".to_string(),
            vec![
                create_tool_parameter("task_id", "string", "ID of the task to update",
                true), create_tool_parameter("status", "string",
                "New status (todo/in_progress/blocked/review/done)", false),
                create_tool_parameter("progress", "number",
                "Progress percentage (0-100)", false), create_tool_parameter("priority",
                "string", "New priority level", false), create_tool_parameter("assignee",
                "string", "New assignee", false), create_tool_parameter("context",
                "string", "Additional context or notes", false),
            ],
            "Task Management".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let task_id = match kwargs.get("task_id") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'task_id' parameter".to_string(),
                    120,
                );
            }
        };
        if task_id.trim().is_empty() || task_id.len() > 50 {
            return ToolResult::error("Task ID must be 1-50 characters".to_string(), 120);
        }
        if let Some(status_str) = kwargs.get("status").and_then(|v| v.as_str()) {
            match status_str.to_lowercase().as_str() {
                "completed" | "done" => {
                    match crate::Actions::complete(task_id).await {
                        Ok(_) => {
                            return ToolResult::success(
                                format!("✅ Task {} marked as completed", task_id),
                                120,
                            );
                        }
                        Err(e) => {
                            return ToolResult::error(
                                format!("Failed to complete task: {}", e),
                                120,
                            );
                        }
                    }
                }
                "in_progress" | "started" => {
                    match crate::Actions::begin(task_id).await {
                        Ok(_) => {
                            return ToolResult::success(
                                format!("🔄 Task {} marked as in progress", task_id),
                                120,
                            );
                        }
                        Err(e) => {
                            return ToolResult::error(
                                format!("Failed to start task: {}", e),
                                120,
                            );
                        }
                    }
                }
                _ => {
                    let updates = TaskUpdate {
                        action: None,
                        time: None,
                        priority: kwargs
                            .get("priority")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok()),
                        parent_project: None,
                        status: kwargs
                            .get("status")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok()),
                        assignee: kwargs
                            .get("assignee")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok()),
                        tags: None,
                        dependencies: None,
                        context_notes: kwargs
                            .get("context")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        progress: kwargs
                            .get("progress")
                            .and_then(|v| v.as_u64())
                            .map(|n| n as u8),
                        embedding_vector: None,
                    };
                    match crate::Done::update_task_full(task_id, updates).await {
                        Ok(_) => {
                            ToolResult::success(
                                format!("✅ Updated task {}", task_id),
                                120,
                            )
                        }
                        Err(e) => {
                            ToolResult::error(
                                format!("Failed to update task: {}", e),
                                120,
                            )
                        }
                    }
                }
            }
        } else {
            let updates = TaskUpdate {
                action: None,
                time: None,
                priority: kwargs
                    .get("priority")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse().ok()),
                parent_project: None,
                status: None,
                assignee: kwargs
                    .get("assignee")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse().ok()),
                tags: None,
                dependencies: None,
                context_notes: kwargs
                    .get("context")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                progress: kwargs
                    .get("progress")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as u8),
                embedding_vector: None,
            };
            match crate::Done::update_task_full(task_id, updates).await {
                Ok(_) => {
                    ToolResult::success(format!("✅ Updated task {}", task_id), 120)
                }
                Err(e) => ToolResult::error(format!("Failed to update task: {}", e), 120),
            }
        }
    }
}
pub struct CreateMemoryTool {
    todozi: SharedTodozi,
}
impl CreateMemoryTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for CreateMemoryTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "create_memory".to_string(),
            "Create a new memory for learning and context".to_string(),
            vec![
                create_tool_parameter("moment", "string", "What happened (the moment)",
                true), create_tool_parameter("meaning", "string",
                "What it means or why it's important", true),
                create_tool_parameter("reason", "string",
                "The reason for remembering this", true),
                create_tool_parameter("importance", "string",
                "Importance level (low/medium/high/critical)", false),
                create_tool_parameter("term", "string", "Memory term (short/long)",
                false), create_tool_parameter("tags", "string", "Comma-separated tags",
                false),
            ],
            "Memory Management".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let moment = match kwargs.get("moment") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'moment' parameter".to_string(),
                    200,
                );
            }
        };
        let meaning = match kwargs.get("meaning") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'meaning' parameter".to_string(),
                    200,
                );
            }
        };
        let reason = match kwargs.get("reason") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'reason' parameter".to_string(),
                    200,
                );
            }
        };
        if moment.len() > 1000 || meaning.len() > 1000 || reason.len() > 1000 {
            return ToolResult::error(
                "Parameters must be <= 1000 characters".to_string(),
                200,
            );
        }
        let importance = kwargs
            .get("importance")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let memory_id = match importance.to_lowercase().as_str() {
            "high" | "critical" => {
                match crate::Memories::important(moment, meaning, reason).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create important memory: {}", e),
                            200,
                        );
                    }
                }
            }
            _ => {
                match crate::Memories::create(moment, meaning, reason).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create memory: {}", e),
                            200,
                        );
                    }
                }
            }
        };
        ToolResult::success(
            format!("🧠 Created memory '{}' with ID: {}", moment, memory_id),
            200,
        )
    }
}
pub struct CreateIdeaTool {
    todozi: SharedTodozi,
}
impl CreateIdeaTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for CreateIdeaTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "create_idea".to_string(),
            "Create a new creative idea or concept".to_string(),
            vec![
                create_tool_parameter("idea", "string", "The idea content", true),
                create_tool_parameter("share", "string",
                "Share level (private/team/public)", false),
                create_tool_parameter("importance", "string",
                "Importance level (low/medium/high/breakthrough)", false),
                create_tool_parameter("tags", "string", "Comma-separated tags", false),
                create_tool_parameter("context", "string", "Additional context", false),
            ],
            "Idea Management".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let idea_content = match kwargs.get("idea") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'idea' parameter".to_string(),
                    180,
                );
            }
        };
        if idea_content.trim().is_empty() || idea_content.len() > 1000 {
            return ToolResult::error("Idea must be 1-1000 characters".to_string(), 180);
        }
        let importance = kwargs
            .get("importance")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let idea_id = match importance.to_lowercase().as_str() {
            "breakthrough" | "high" => {
                match crate::Ideas::breakthrough(idea_content).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create breakthrough idea: {}", e),
                            180,
                        );
                    }
                }
            }
            _ => {
                match crate::Ideas::create(idea_content).await {
                    Ok(id) => id,
                    Err(e) => {
                        return ToolResult::error(
                            format!("Failed to create idea: {}", e),
                            180,
                        );
                    }
                }
            }
        };
        ToolResult::success(
            format!("💡 Created idea '{}' with ID: {}", idea_content, idea_id),
            180,
        )
    }
}
pub struct UnifiedSearchTool {
    todozi: SharedTodozi,
    embedding_service: Option<TodoziEmbeddingService>,
}
impl UnifiedSearchTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            embedding_service: None,
        }
    }
    pub fn with_embedding_service(mut self, service: TodoziEmbeddingService) -> Self {
        self.embedding_service = Some(service);
        self
    }
    pub fn with_embedding_service_option(
        mut self,
        service: Option<TodoziEmbeddingService>,
    ) -> Self {
        self.embedding_service = service;
        self
    }
}
#[async_trait]
impl Tool for UnifiedSearchTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "unified_search".to_string(),
            "Search across all Todozi data types with AI semantic capabilities (tasks, memories, ideas, errors)"
                .to_string(),
            vec![
                create_tool_parameter("query", "string", "Search query", true),
                create_tool_parameter("semantic", "boolean",
                "Use AI semantic search instead of keyword matching", false),
                create_tool_parameter("data_types", "string",
                "Comma-separated data types to search (tasks,memories,ideas,errors)",
                false), create_tool_parameter("limit", "number",
                "Maximum results per type", false),
            ],
            "Search".to_string(),
            vec![ResourceLock::FilesystemRead],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let query = match kwargs.get("query") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'query' parameter".to_string(),
                    300,
                );
            }
        };
        if query.trim().is_empty() || query.len() > 100 {
            return ToolResult::error("Query must be 1-100 characters".to_string(), 300);
        }
        let semantic = kwargs.get("semantic").and_then(|v| v.as_bool()).unwrap_or(false);
        let _data_types = kwargs
            .get("data_types")
            .and_then(|v| v.as_str())
            .unwrap_or("tasks,memories,ideas,errors");
        let _limit = kwargs.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
        if semantic {
            match crate::Find::deep(query).await {
                Ok(ai_results) => {
                    if ai_results.is_empty() {
                        return ToolResult::success(
                            format!("🤖 No AI semantic results found for: {}", query),
                            300,
                        );
                    }
                    let results = ai_results
                        .iter()
                        .map(|result| {
                            format!(
                                "• {} | Type: {:?} | Similarity: {:.2}", result
                                .text_content, result.content_type, result.similarity_score
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    return ToolResult::success(
                        format!(
                            "🤖 AI Unified Search - Found {} semantic matches:\n{}",
                            ai_results.len(), results
                        ),
                        300,
                    );
                }
                Err(e) => {
                    return ToolResult::error(
                        format!("AI semantic search failed: {}", e),
                        300,
                    );
                }
            }
        } else {
            match crate::Find::tdz_find(query).await {
                Ok(unified_results) => {
                    if unified_results.is_empty()
                        || unified_results.contains("No results found")
                    {
                        ToolResult::success(
                            format!("🔍 No unified results found for: {}", query),
                            300,
                        )
                    } else {
                        ToolResult::success(
                            format!("🔍 Unified Search Results:\n{}", unified_results),
                            300,
                        )
                    }
                }
                Err(e) => ToolResult::error(format!("Unified search failed: {}", e), 300),
            }
        }
    }
}
pub struct ProcessChatMessageTool {
    todozi: SharedTodozi,
}
impl ProcessChatMessageTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for ProcessChatMessageTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "process_chat_message".to_string(),
            "Process a chat message containing Todozi tags and create corresponding items"
                .to_string(),
            vec![
                create_tool_parameter("message", "string",
                "Chat message with Todozi tags", true), create_tool_parameter("user_id",
                "string", "User ID for created items", false),
            ],
            "Message Processing".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let message = match kwargs.get("message") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'message' parameter".to_string(),
                    250,
                );
            }
        };
        if message.trim().is_empty() || message.len() > 10000 {
            return ToolResult::error(
                "Message must be 1-10000 characters".to_string(),
                250,
            );
        }
        let _user_id = kwargs
            .get("user_id")
            .and_then(|v| v.as_str())
            .unwrap_or("ai_agent");
        match crate::Tdz::chat(message).await {
            Ok(content) => {
                let mut results = Vec::new();
                if !content.tasks.is_empty() {
                    results.push(format!("📋 Created {} tasks", content.tasks.len()));
                    for task in &content.tasks {
                        results
                            .push(
                                format!(
                                    "  • {} [{}]", task.action, task.assignee.as_ref().map(| a
                                    | format!("{:?}", a)).unwrap_or("unassigned".to_string())
                                ),
                            );
                    }
                }
                if !content.memories.is_empty() {
                    results
                        .push(
                            format!("🧠 Created {} memories", content.memories.len()),
                        );
                    for memory in &content.memories {
                        results.push(format!("  • {}", memory.moment));
                    }
                }
                if !content.ideas.is_empty() {
                    results.push(format!("💡 Created {} ideas", content.ideas.len()));
                    for idea in &content.ideas {
                        results.push(format!("  • {}", idea.idea));
                    }
                }
                if !content.errors.is_empty() {
                    results
                        .push(
                            format!("❌ Created {} error records", content.errors.len()),
                        );
                }
                if !content.feelings.is_empty() {
                    results
                        .push(
                            format!("😊 Created {} feelings", content.feelings.len()),
                        );
                }
                if results.is_empty() {
                    ToolResult::success(
                        "✅ Message processed - no structured content extracted"
                            .to_string(),
                        250,
                    )
                } else {
                    ToolResult::success(results.join("\n"), 250)
                }
            }
            Err(e) => {
                ToolResult::error(format!("Failed to process chat message: {}", e), 250)
            }
        }
    }
}
pub struct CreateErrorTool {
    todozi: SharedTodozi,
}
impl CreateErrorTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for CreateErrorTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "create_error".to_string(),
            "Create an error record for tracking issues".to_string(),
            vec![
                create_tool_parameter("title", "string", "Error title/summary", true),
                create_tool_parameter("description", "string",
                "Detailed error description", true), create_tool_parameter("severity",
                "string", "Severity level (low/medium/high/critical)", false),
                create_tool_parameter("category", "string", "Error category", false),
                create_tool_parameter("source", "string", "Source file/component",
                false), create_tool_parameter("context", "string", "Additional context",
                false), create_tool_parameter("tags", "string", "Comma-separated tags",
                false),
            ],
            "Error Tracking".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let title = match kwargs.get("title") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'title' parameter".to_string(),
                    220,
                );
            }
        };
        let description = match kwargs.get("description") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'description' parameter".to_string(),
                    220,
                );
            }
        };
        let source = match kwargs.get("source") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'source' parameter".to_string(),
                    220,
                );
            }
        };
        if title.len() > 200 || description.len() > 1000 || source.len() > 200 {
            return ToolResult::error("Parameters exceed length limits".to_string(), 220);
        }
        let error = Error {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            severity: kwargs
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or("medium")
                .parse()
                .unwrap_or(ErrorSeverity::Medium),
            category: kwargs
                .get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("runtime")
                .parse()
                .unwrap_or(ErrorCategory::Runtime),
            source: source.to_string(),
            context: kwargs
                .get("context")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            tags: kwargs
                .get("tags")
                .and_then(|v| v.as_str())
                .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                .unwrap_or_default(),
            resolved: false,
            resolution: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            resolved_at: None,
        };
        match save_error(&error) {
            Ok(_) => {
                ToolResult::success(
                    format!(
                        "Created error record '{}' with ID: {}", error.title, error.id
                    ),
                    220,
                )
            }
            Err(e) => {
                ToolResult::error(format!("Failed to create error record: {}", e), 220)
            }
        }
    }
}
pub struct CreateCodeChunkTool {
    todozi: SharedTodozi,
}
impl CreateCodeChunkTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for CreateCodeChunkTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "create_code_chunk".to_string(),
            "Create a code chunk for hierarchical task decomposition".to_string(),
            vec![
                create_tool_parameter("chunk_id", "string", "Unique chunk identifier",
                true), create_tool_parameter("level", "string",
                "Chunking level (project/module/class/method/block)", true),
                create_tool_parameter("description", "string",
                "What this chunk accomplishes", true),
                create_tool_parameter("dependencies", "string",
                "Comma-separated dependency chunk IDs", false),
                create_tool_parameter("code", "string", "The actual code content",
                false),
            ],
            "Code Chunking".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let chunk_id = match kwargs.get("chunk_id") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'chunk_id' parameter".to_string(),
                    180,
                );
            }
        };
        let level_str = match kwargs.get("level") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'level' parameter".to_string(),
                    180,
                );
            }
        };
        let description = match kwargs.get("description") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'description' parameter".to_string(),
                    180,
                );
            }
        };
        if chunk_id.len() > 100 || level_str.len() > 50 || description.len() > 500 {
            return ToolResult::error("Parameters exceed length limits".to_string(), 180);
        }
        let level = match level_str.to_lowercase().as_str() {
            "project" => ChunkingLevel::Project,
            "module" => ChunkingLevel::Module,
            "class" => ChunkingLevel::Class,
            "method" => ChunkingLevel::Method,
            "block" => ChunkingLevel::Block,
            _ => {
                return ToolResult::error(
                    format!("Invalid chunking level: {}", level_str),
                    180,
                );
            }
        };
        let dependencies = kwargs
            .get("dependencies")
            .and_then(|v| v.as_str())
            .map(|s| s.split(',').map(|d| d.trim().to_string()).collect())
            .unwrap_or_default();
        let chunk = CodeChunk {
            chunk_id: chunk_id.to_string(),
            status: ChunkStatus::Pending,
            dependencies,
            code: kwargs.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            tests: String::new(),
            validated: false,
            level,
            estimated_tokens: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        match save_code_chunk(&chunk) {
            Ok(_) => {
                ToolResult::success(
                    format!(
                        "Created code chunk '{}' at level {:?}", chunk.chunk_id, chunk
                        .level
                    ),
                    180,
                )
            }
            Err(e) => {
                ToolResult::error(format!("Failed to create code chunk: {}", e), 180)
            }
        }
    }
}
pub struct ChecklistTool {
    todozi: SharedTodozi,
}
impl ChecklistTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self { todozi }
    }
}
#[async_trait]
impl Tool for ChecklistTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "extract_tasks".to_string(),
            "Extract actionable tasks from message content and create them in Todozi"
                .to_string(),
            vec![
                create_tool_parameter("content", "string",
                "Message content to extract tasks from", true),
                create_tool_parameter("project", "string",
                "Project to associate extracted tasks with", false),
                create_tool_parameter("priority", "string",
                "Default priority for extracted tasks (low/medium/high/critical/urgent)",
                false), create_tool_parameter("assignee", "string",
                "Default assignee for extracted tasks (ai/human/collaborative)", false),
            ],
            "Task Management".to_string(),
            vec![ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let content = match kwargs.get("content") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'content' parameter".to_string(),
                    150,
                );
            }
        };
        if content.trim().is_empty() || content.len() > 10000 {
            return ToolResult::error(
                "Content must be 1-10000 characters".to_string(),
                150,
            );
        }
        let default_project = kwargs
            .get("project")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let default_priority = kwargs
            .get("priority")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok());
        let default_assignee = kwargs
            .get("assignee")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok());
        let extracted_tasks = self.extract_tasks_from_content(content);
        if extracted_tasks.is_empty() {
            return ToolResult::success("No tasks found in content".to_string(), 150);
        }
        let mut created_count = 0;
        let storage = self.todozi.lock().await;
        for task_action in extracted_tasks {
            let task = Task {
                id: uuid::Uuid::new_v4().to_string(),
                user_id: "ai_agent".to_string(),
                action: task_action,
                time: "ASAP".to_string(),
                priority: default_priority.unwrap_or(Priority::Medium),
                parent_project: default_project.to_string(),
                status: Status::Todo,
                assignee: default_assignee.clone(),
                tags: vec!["extracted".to_string()],
                dependencies: Vec::new(),
                context_notes: Some("Extracted from message content".to_string()),
                progress: Some(0),
                embedding_vector: Some(Vec::new()),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            if storage.add_task_to_project(task).await.is_ok() {
                created_count += 1;
            }
        }
        ToolResult::success(
            format!("Extracted and created {} tasks from content", created_count),
            150,
        )
    }
}
impl ChecklistTool {
    fn extract_tasks_from_content(&self, content: &str) -> Vec<String> {
        let mut tasks = Vec::new();
        let task_indicators = vec![
            r"(?i)^\s*[\*\-\•]\s*\[ \]\s*(.+)", r"(?i)^\s*[\*\-\•]\s*(.+)",
            r"(?i)^\s*\d+\.\s*(.+)", r"(?i)^\s*todo:\s*(.+)", r"(?i)^\s*task:\s*(.+)",
            r"(?i)need to\s+(.+)", r"(?i)should\s+(.+)", r"(?i)must\s+(.+)",
        ];
        for line in content.lines() {
            for pattern in &task_indicators {
                if let Ok(regex) = Regex::new(pattern) {
                    if let Some(captures) = regex.captures(line) {
                        if let Some(task_text) = captures.get(1) {
                            let task = task_text.as_str().trim().to_string();
                            if !task.is_empty() && task.len() <= 200 {
                                tasks.push(task);
                                break;
                            }
                        }
                    }
                }
            }
        }
        if tasks.is_empty() {
            let sentence_patterns = vec![
                r"(?i)I will\s+(.+?)(?:\.|$)", r"(?i)We need to\s+(.+?)(?:\.|$)",
                r"(?i)Let's\s+(.+?)(?:\.|$)",
            ];
            for pattern in &sentence_patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    for captures in regex.captures_iter(content) {
                        if let Some(task_text) = captures.get(1) {
                            let task = task_text.as_str().trim().to_string();
                            if !task.is_empty() && task.len() <= 200 {
                                tasks.push(task);
                            }
                        }
                    }
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        tasks.retain(|task| seen.insert(task.to_lowercase()));
        tasks
    }
    pub fn extract_tasks(content: &str) -> Vec<String> {
        let mut tasks = Vec::new();
        let task_indicators = vec![
            r"(?i)^\s*[\*\-\•]\s*\[ \]\s*(.+)", r"(?i)^\s*[\*\-\•]\s*(.+)",
            r"(?i)^\s*\d+\.\s*(.+)", r"(?i)^\s*todo:\s*(.+)", r"(?i)^\s*task:\s*(.+)",
            r"(?i)need to\s+(.+)", r"(?i)should\s+(.+)", r"(?i)must\s+(.+)",
        ];
        for line in content.lines() {
            for pattern in &task_indicators {
                if let Ok(regex) = Regex::new(pattern) {
                    if let Some(captures) = regex.captures(line) {
                        if let Some(task_text) = captures.get(1) {
                            let task = task_text.as_str().trim().to_string();
                            if !task.is_empty() && task.len() <= 200 {
                                tasks.push(task);
                                break;
                            }
                        }
                    }
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        tasks.retain(|task| seen.insert(task.to_lowercase()));
        tasks
    }
}
fn search_tasks_in_files(
    query: &str,
    filters: &TaskFilters,
    limit: usize,
) -> Result<(Vec<Task>, usize)> {
    use std::fs;
    let tasks_dir = get_tasks_dir()?;
    let mut matching_tasks = Vec::new();
    let mut total_found = 0;
    if tasks_dir.exists() {
        for entry in fs::read_dir(tasks_dir).map_err(|e| TodoziError::IoError(e.into()))?
        {
            let entry = entry.map_err(|e| TodoziError::IoError(e.into()))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json")
                && !path.file_name().unwrap().to_str().unwrap().starts_with("active")
                && !path.file_name().unwrap().to_str().unwrap().starts_with("completed")
                && !path.file_name().unwrap().to_str().unwrap().starts_with("archived")
            {
                if let Ok(task_json) = fs::read_to_string(&path) {
                    if let Ok(task) = serde_json::from_str::<Task>(&task_json) {
                        let matches = matches_filters(&task, query, filters);
                        if matches {
                            total_found += 1;
                            if matching_tasks.len() < limit {
                                matching_tasks.push(task);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok((matching_tasks, total_found))
}
fn matches_filters(task: &Task, query: &str, filters: &TaskFilters) -> bool {
    let text_match = query.is_empty()
        || task.action.to_lowercase().contains(&query.to_lowercase())
        || task
            .context_notes
            .as_ref()
            .map_or(false, |ctx| ctx.to_lowercase().contains(&query.to_lowercase()));
    let status_match = filters.status.is_none()
        || task.status == filters.status.unwrap();
    let priority_match = filters.priority.is_none()
        || task.priority == filters.priority.unwrap();
    let project_match = filters.project.is_none()
        || task.parent_project == filters.project.clone().unwrap();
    let assignee_match = filters.assignee.is_none() || task.assignee == filters.assignee;
    let tags_match = filters
        .tags
        .as_ref()
        .map_or(
            true,
            |filter_tags| {
                filter_tags
                    .iter()
                    .any(|filter_tag| {
                        task.tags
                            .iter()
                            .any(|task_tag| {
                                task_tag.to_lowercase().contains(&filter_tag.to_lowercase())
                            })
                    })
            },
        );
    text_match && status_match && priority_match && project_match && assignee_match
        && tags_match
}
fn update_task_in_file(
    task_id: &str,
    kwargs: &HashMap<String, serde_json::Value>,
) -> Result<()> {
    let mut task = load_task(task_id)?;
    if let Some(status) = kwargs
        .get("status")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
    {
        task.status = status;
    }
    if let Some(progress) = kwargs.get("progress").and_then(|v| v.as_u64()) {
        task.progress = Some(progress as u8);
    }
    if let Some(priority) = kwargs
        .get("priority")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
    {
        task.priority = priority;
    }
    if let Some(assignee) = kwargs
        .get("assignee")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
    {
        task.assignee = Some(assignee);
    }
    if let Some(context) = kwargs.get("context").and_then(|v| v.as_str()) {
        task.context_notes = Some(context.to_string());
    }
    task.updated_at = chrono::Utc::now();
    save_task(&task)?;
    Ok(())
}
pub fn create_todozi_tools(todozi: SharedTodozi) -> Vec<Box<dyn Tool + Send + Sync>> {
    create_todozi_tools_with_embedding(todozi, None)
}
pub struct SimpleTodoziTool;
impl SimpleTodoziTool {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Tool for SimpleTodoziTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "simple_todozi".to_string(),
            "Ultra-simple Todozi interface with automatic AI/human coordination and smart search"
                .to_string(),
            vec![
                create_tool_parameter("action", "string",
                "What to do: 'task', 'urgent', 'find', 'remember', 'idea', 'stats', 'ai_search', 'complete', 'start'",
                true), create_tool_parameter("content", "string",
                "The content/description for the action", true),
                create_tool_parameter("extra", "string",
                "Extra context, meaning, or details", false),
            ],
            "Simple Task Management".to_string(),
            vec![ResourceLock::FilesystemWrite, ResourceLock::FilesystemRead],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let action = match kwargs.get("action") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => return ToolResult::error("Missing 'action' parameter".to_string(), 50),
        };
        let content = match kwargs.get("content") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => return ToolResult::error("Missing 'content' parameter".to_string(), 50),
        };
        let extra = kwargs.get("extra").and_then(|v| v.as_str()).unwrap_or("");
        match action.to_lowercase().as_str() {
            "task" => {
                match crate::Easy::do_it(content).await {
                    Ok(task_id) => {
                        ToolResult::success(format!("✅ Task created: {}", task_id), 50)
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to create task: {}", e), 50)
                    }
                }
            }
            "urgent" => {
                match crate::Tdz::urgent(content).await {
                    Ok(task_id) => {
                        ToolResult::success(
                            format!("🚨 Urgent task created: {}", task_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to create urgent task: {}", e),
                            50,
                        )
                    }
                }
            }
            "high" => {
                match crate::Tdz::high(content).await {
                    Ok(task_id) => {
                        ToolResult::success(
                            format!("🟠 High priority task created: {}", task_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to create high priority task: {}", e),
                            50,
                        )
                    }
                }
            }
            "low" => {
                match crate::Tdz::low(content).await {
                    Ok(task_id) => {
                        ToolResult::success(
                            format!("🟢 Low priority task created: {}", task_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to create low priority task: {}", e),
                            50,
                        )
                    }
                }
            }
            "ai" => {
                match crate::Actions::ai(content).await {
                    Ok(task_id) => {
                        ToolResult::success(
                            format!("🤖 AI task queued: {}", task_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to create AI task: {}", e), 50)
                    }
                }
            }
            "human" => {
                match crate::Actions::human(content).await {
                    Ok(task_id) => {
                        ToolResult::success(
                            format!(
                                "👤 Human task created (visible in TUI): {}", task_id
                            ),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to create human task: {}", e),
                            50,
                        )
                    }
                }
            }
            "collab" => {
                match crate::Actions::collab(content).await {
                    Ok(task_id) => {
                        ToolResult::success(
                            format!("🤝 Collaborative task created: {}", task_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to create collaborative task: {}", e),
                            50,
                        )
                    }
                }
            }
            "find" => {
                match crate::Find::tdz_find(content).await {
                    Ok(results) => {
                        ToolResult::success(
                            format!("🔍 Smart search results:\n{}", results),
                            50,
                        )
                    }
                    Err(e) => ToolResult::error(format!("Search failed: {}", e), 50),
                }
            }
            "ai_search" => {
                match crate::Find::deep(content).await {
                    Ok(results) => {
                        let formatted = results
                            .iter()
                            .map(|r| {
                                format!(
                                    "• {} (similarity: {:.2})", r.text_content, r
                                    .similarity_score
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        ToolResult::success(
                            format!("🤖 AI semantic search:\n{}", formatted),
                            50,
                        )
                    }
                    Err(e) => ToolResult::error(format!("AI search failed: {}", e), 50),
                }
            }
            "fast_search" => {
                match crate::Find::fast(content).await {
                    Ok(results) => {
                        ToolResult::success(
                            format!("⚡ Fast keyword search:\n{}", results),
                            50,
                        )
                    }
                    Err(e) => ToolResult::error(format!("Fast search failed: {}", e), 50),
                }
            }
            "smart_search" => {
                match crate::Find::smart(content).await {
                    Ok(results) => {
                        ToolResult::success(
                            format!("🧠 Smart intent search:\n{}", results),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(format!("Smart search failed: {}", e), 50)
                    }
                }
            }
            "remember" => {
                match crate::Memories::create(content, extra, "Created via simple tool")
                    .await
                {
                    Ok(memory_id) => {
                        ToolResult::success(
                            format!("🧠 Memory saved: {}", memory_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to save memory: {}", e), 50)
                    }
                }
            }
            "important_memory" => {
                match crate::Memories::important(
                        content,
                        extra,
                        "Important via simple tool",
                    )
                    .await
                {
                    Ok(memory_id) => {
                        ToolResult::success(
                            format!("🧠⭐ Important memory saved: {}", memory_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to save important memory: {}", e),
                            50,
                        )
                    }
                }
            }
            "idea" => {
                match crate::Ideas::create(content).await {
                    Ok(idea_id) => {
                        ToolResult::success(format!("💡 Idea saved: {}", idea_id), 50)
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to save idea: {}", e), 50)
                    }
                }
            }
            "breakthrough_idea" => {
                match crate::Ideas::breakthrough(content).await {
                    Ok(idea_id) => {
                        ToolResult::success(
                            format!("💡🚀 Breakthrough idea saved: {}", idea_id),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(
                            format!("Failed to save breakthrough idea: {}", e),
                            50,
                        )
                    }
                }
            }
            "complete" => {
                match crate::Actions::complete(content).await {
                    Ok(_) => {
                        ToolResult::success(
                            format!("✅ Task {} completed", content),
                            50,
                        )
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to complete task: {}", e), 50)
                    }
                }
            }
            "start" => {
                match crate::Actions::begin(content).await {
                    Ok(_) => {
                        ToolResult::success(format!("🔄 Task {} started", content), 50)
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to start task: {}", e), 50)
                    }
                }
            }
            "stats" => {
                match crate::Stats::quick().await {
                    Ok(stats) => {
                        ToolResult::success(format!("📊 Quick stats:\n{}", stats), 50)
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to get stats: {}", e), 50)
                    }
                }
            }
            "queue" => {
                match crate::Queue::list().await {
                    Ok(items) => {
                        let summary = format!("📋 Queue: {} total items", items.len());
                        ToolResult::success(summary, 50)
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to get queue: {}", e), 50)
                    }
                }
            }
            "chat" => {
                match crate::Tdz::chat(content).await {
                    Ok(chat_content) => {
                        let mut results = Vec::new();
                        if !chat_content.tasks.is_empty() {
                            results
                                .push(format!("📋 {} tasks", chat_content.tasks.len()));
                        }
                        if !chat_content.memories.is_empty() {
                            results
                                .push(
                                    format!("🧠 {} memories", chat_content.memories.len()),
                                );
                        }
                        if !chat_content.ideas.is_empty() {
                            results
                                .push(format!("💡 {} ideas", chat_content.ideas.len()));
                        }
                        let summary = if results.is_empty() {
                            "✅ Chat processed - no structured content".to_string()
                        } else {
                            format!("✅ Chat processed: {}", results.join(", "))
                        };
                        ToolResult::success(summary, 50)
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to process chat: {}", e), 50)
                    }
                }
            }
            "extract" => {
                match crate::Done::extract_tasks(content, Some(extra)).await {
                    Ok(extracted_tasks) => {
                        if extracted_tasks.is_empty() {
                            ToolResult::success(
                                "🤖 No tasks extracted from content".to_string(),
                                50,
                            )
                        } else {
                            let task_list = extracted_tasks
                                .iter()
                                .enumerate()
                                .map(|(i, task)| format!("{}. {}", i + 1, task))
                                .collect::<Vec<_>>()
                                .join("\n");
                            ToolResult::success(
                                format!(
                                    "🤖 Extracted {} tasks via todozi.com AI:\n{}",
                                    extracted_tasks.len(), task_list
                                ),
                                50,
                            )
                        }
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to extract tasks: {}", e), 50)
                    }
                }
            }
            "expand" => {
                match crate::Done::plan_tasks(
                        content,
                        Some("medium"),
                        Some("ASAP"),
                        Some(extra),
                    )
                    .await
                {
                    Ok(planned_tasks) => {
                        if planned_tasks.is_empty() {
                            ToolResult::success(
                                "🤖 No task expansion generated".to_string(),
                                50,
                            )
                        } else {
                            let task_list = planned_tasks
                                .iter()
                                .enumerate()
                                .map(|(i, task)| format!("{}. {}", i + 1, task.action))
                                .collect::<Vec<_>>()
                                .join("\n");
                            ToolResult::success(
                                format!(
                                    "🤖 Expanded into {} subtasks via todozi.com AI:\n{}",
                                    planned_tasks.len(), task_list
                                ),
                                50,
                            )
                        }
                    }
                    Err(e) => {
                        ToolResult::error(format!("Failed to expand tasks: {}", e), 50)
                    }
                }
            }
            _ => {
                ToolResult::error(
                    format!(
                        "❌ Unknown action: '{}'. Available: task, urgent, high, low, ai, human, collab, find, ai_search, fast_search, smart_search, remember, important_memory, idea, breakthrough_idea, complete, start, stats, queue, chat, extract, expand",
                        action
                    ),
                    50,
                )
            }
        }
    }
}
pub fn create_todozi_tools_with_embedding(
    todozi: SharedTodozi,
    embedding_service: Option<TodoziEmbeddingService>,
) -> Vec<Box<dyn Tool + Send + Sync>> {
    vec![
        Box::new(SimpleTodoziTool::new()), Box::new(CreateTaskTool::new(todozi.clone())),
        Box::new(SearchTasksTool::new(todozi.clone())
        .with_embedding_service_option(embedding_service.clone())),
        Box::new(UpdateTaskTool::new(todozi.clone())),
        Box::new(CreateMemoryTool::new(todozi.clone())),
        Box::new(CreateIdeaTool::new(todozi.clone())),
        Box::new(UnifiedSearchTool::new(todozi.clone())
        .with_embedding_service_option(embedding_service)),
        Box::new(ProcessChatMessageTool::new(todozi.clone())),
        Box::new(CreateErrorTool::new(todozi.clone())),
        Box::new(CreateCodeChunkTool::new(todozi.clone())),
        Box::new(ChecklistTool::new(todozi.clone())),
    ]
}
pub struct IntelligentTaskPlannerTool {
    todozi: SharedTodozi,
    context_memory: Arc<Mutex<HashMap<String, Vec<String>>>>,
}
impl IntelligentTaskPlannerTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            context_memory: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
#[async_trait]
impl Tool for IntelligentTaskPlannerTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "intelligent_task_planning".to_string(),
            "AI-powered task planning with predictive analytics, resource optimization, and intelligent scheduling"
                .to_string(),
            vec![
                create_tool_parameter("goal", "string",
                "High-level goal or objective to plan for", true),
                create_tool_parameter("context", "string",
                "Current project context and constraints", false),
                create_tool_parameter("timeline", "string",
                "Desired timeline (e.g., '2 weeks', 'end of month')", false),
                create_tool_parameter("resources", "string",
                "Available resources and team members", false),
                create_tool_parameter("complexity", "string",
                "Project complexity level (simple/medium/complex/extreme)", false),
                create_tool_parameter("conversation_id", "string",
                "Conversation context ID for continuity", false),
            ],
            "Intelligent Planning".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::Memory],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let goal = match kwargs.get("goal") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'goal' parameter".to_string(),
                    500,
                );
            }
        };
        if goal.trim().is_empty() || goal.len() > 2000 {
            return ToolResult::error("Goal must be 1-2000 characters".to_string(), 500);
        }
        let conversation_id = kwargs
            .get("conversation_id")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        let context_memory = self.context_memory.lock().await;
        let context_items = context_memory
            .get(conversation_id)
            .cloned()
            .unwrap_or_default();
        drop(context_memory);
        let storage = self.todozi.lock().await;
        let existing_tasks = match storage.list_tasks_across_projects(&TaskFilters::default()) {
            Ok(tasks) => tasks,
            Err(_) => Vec::new(),
        };
        let plan = self
            .generate_intelligent_plan(goal, &context_items, &existing_tasks)
            .await;
        let mut updated_context = context_items;
        updated_context.push(format!("Planned: {}", goal));
        if updated_context.len() > 10 {
            updated_context.remove(0);
        }
        let mut context_memory = self.context_memory.lock().await;
        context_memory.insert(conversation_id.to_string(), updated_context);
        ToolResult::success(plan, 500)
    }
}
impl IntelligentTaskPlannerTool {
    async fn generate_intelligent_plan(
        &self,
        goal: &str,
        context: &[String],
        existing_tasks: &[Task],
    ) -> String {
        let mut plan = format!(
            "🎯 **INTELLIGENT TASK PLAN: {}**\n\n", goal.to_uppercase()
        );
        let complexity = self.analyze_complexity(goal, context);
        plan.push_str(&format!("📊 **Complexity Analysis**: {}\n\n", complexity));
        let tasks = self.break_down_goal(goal, &complexity, existing_tasks);
        plan.push_str("📋 **Generated Task Breakdown**:\n");
        for (i, task) in tasks.iter().enumerate() {
            plan.push_str(&format!("{}. {}\n", i + 1, task));
        }
        plan.push_str("\n");
        let resources = self.recommend_resources(&tasks);
        plan.push_str("👥 **Recommended Resources**:\n");
        for resource in resources {
            plan.push_str(&format!("• {}\n", resource));
        }
        plan.push_str("\n");
        let risks = self.assess_risks(&tasks, existing_tasks);
        plan.push_str("⚠️ **Risk Assessment**:\n");
        for risk in risks {
            plan.push_str(&format!("• {}\n", risk));
        }
        plan.push_str("\n");
        plan.push_str("📈 **Success Metrics**:\n");
        plan.push_str("• All subtasks completed on time\n");
        plan.push_str("• Quality standards met\n");
        plan.push_str("• Stakeholder satisfaction achieved\n");
        plan.push_str("• Lessons learned documented\n");
        plan
    }
    fn analyze_complexity(&self, goal: &str, _context: &[String]) -> String {
        let word_count = goal.split_whitespace().count();
        let has_technical_terms = goal.to_lowercase().contains("api")
            || goal.to_lowercase().contains("database")
            || goal.to_lowercase().contains("integration");
        let has_multiple_steps = goal.contains("and") || goal.contains(",")
            || goal.contains("then");
        match (word_count, has_technical_terms, has_multiple_steps) {
            (0..=10, false, false) => {
                "🟢 SIMPLE: Straightforward task with clear requirements".to_string()
            }
            (11..=25, false, true) => {
                "🟡 MEDIUM: Multi-step process with moderate complexity".to_string()
            }
            (11..=25, true, _) => {
                "🟠 COMPLEX: Technical implementation required".to_string()
            }
            _ => {
                "🔴 EXTREME: Highly complex project requiring specialized expertise"
                    .to_string()
            }
        }
    }
    fn break_down_goal(
        &self,
        goal: &str,
        complexity: &str,
        existing_tasks: &[Task],
    ) -> Vec<String> {
        let mut tasks = Vec::new();
        if complexity.contains("SIMPLE") {
            tasks.push(format!("Execute: {}", goal));
            tasks.push("Verify completion and quality".to_string());
        } else if complexity.contains("MEDIUM") {
            tasks.push(format!("Research and plan: {}", goal));
            tasks.push(format!("Break down into specific steps: {}", goal));
            tasks.push("Execute planned steps".to_string());
            tasks.push("Test and validate results".to_string());
        } else {
            tasks.push(format!("Conduct feasibility analysis: {}", goal));
            tasks.push("Gather requirements and constraints".to_string());
            tasks.push("Design solution architecture".to_string());
            tasks.push("Implement core functionality".to_string());
            tasks.push("Test thoroughly and iterate".to_string());
            tasks.push("Deploy and monitor".to_string());
            tasks.push("Document and hand off".to_string());
        }
        let existing_actions: HashSet<_> = existing_tasks
            .iter()
            .map(|t| t.action.to_lowercase())
            .collect();
        tasks.retain(|task| !existing_actions.contains(&task.to_lowercase()));
        tasks
    }
    fn recommend_resources(&self, tasks: &[String]) -> Vec<String> {
        let mut resources = Vec::new();
        for task in tasks {
            if task.to_lowercase().contains("research") {
                resources
                    .push("Research specialist or AI research assistant".to_string());
            }
            if task.to_lowercase().contains("design") {
                resources.push("UX/UI designer or design thinking expert".to_string());
            }
            if task.to_lowercase().contains("code")
                || task.to_lowercase().contains("implement")
            {
                resources.push("Software developer with relevant expertise".to_string());
            }
            if task.to_lowercase().contains("test") {
                resources.push("QA engineer or testing specialist".to_string());
            }
        }
        if resources.is_empty() {
            resources.push("General task execution resources".to_string());
        }
        resources.dedup();
        resources
    }
    fn assess_risks(&self, tasks: &[String], existing_tasks: &[Task]) -> Vec<String> {
        let mut risks = Vec::new();
        if tasks.len() > 5 {
            risks.push("High task count may lead to scope creep".to_string());
        }
        let technical_tasks = tasks
            .iter()
            .filter(|t| {
                t.to_lowercase().contains("api") || t.to_lowercase().contains("database")
                    || t.to_lowercase().contains("integration")
            })
            .count();
        if technical_tasks > tasks.len() / 2 {
            risks
                .push(
                    "High technical complexity requires specialized expertise"
                        .to_string(),
                );
        }
        if existing_tasks.iter().any(|t| t.status == Status::Blocked) {
            risks.push("Existing blocked tasks may impact timeline".to_string());
        }
        if risks.is_empty() {
            risks.push("Low risk - standard project execution".to_string());
        }
        risks
    }
}
pub struct MemorySynthesisTool {
    todozi: SharedTodozi,
    learning_patterns: HashMap<String, Vec<String>>,
}
impl MemorySynthesisTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            learning_patterns: HashMap::new(),
        }
    }
}
#[async_trait]
impl Tool for MemorySynthesisTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "memory_synthesis".to_string(),
            "Advanced memory synthesis creating new insights from existing knowledge patterns"
                .to_string(),
            vec![
                create_tool_parameter("topic", "string",
                "Topic or concept to synthesize knowledge about", true),
                create_tool_parameter("depth", "string",
                "Synthesis depth (basic/detailed/comprehensive)", false),
                create_tool_parameter("context", "string",
                "Additional context for synthesis", false),
                create_tool_parameter("include_patterns", "boolean",
                "Include pattern recognition in synthesis", false),
            ],
            "Knowledge Synthesis".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::Memory],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let topic = match kwargs.get("topic") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'topic' parameter".to_string(),
                    600,
                );
            }
        };
        let depth = kwargs.get("depth").and_then(|v| v.as_str()).unwrap_or("detailed");
        let context = kwargs.get("context").and_then(|v| v.as_str()).unwrap_or("");
        let include_patterns = kwargs
            .get("include_patterns")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let memories = match list_memories() {
            Ok(mems) => mems,
            Err(e) => {
                return ToolResult::error(
                    format!("Failed to retrieve memories: {}", e),
                    600,
                );
            }
        };
        let relevant_memories: Vec<_> = memories
            .into_iter()
            .filter(|m| {
                m.moment.to_lowercase().contains(&topic.to_lowercase())
                    || m.meaning.to_lowercase().contains(&topic.to_lowercase())
                    || m
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&topic.to_lowercase()))
            })
            .collect();
        if relevant_memories.is_empty() {
            return ToolResult::success(
                format!("No existing memories found for topic: {}", topic),
                600,
            );
        }
        let synthesis = self
            .synthesize_knowledge(
                &topic,
                &relevant_memories,
                depth,
                context,
                include_patterns,
            )
            .await;
        let synthesis_memory = Memory {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: "synthesis_agent".to_string(),
            project_id: None,
            status: ItemStatus::Active,
            moment: format!("Knowledge synthesis completed for: {}", topic),
            meaning: format!("Synthesized comprehensive understanding of {}", topic),
            reason: "Created through intelligent memory synthesis process".to_string(),
            importance: MemoryImportance::High,
            term: MemoryTerm::Long,
            memory_type: MemoryType::Standard,
            tags: vec![
                "synthesis".to_string(), "knowledge".to_string(), topic.to_string()
            ],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let _ = save_memory(&synthesis_memory);
        ToolResult::success(synthesis, 600)
    }
}
impl MemorySynthesisTool {
    async fn synthesize_knowledge(
        &self,
        topic: &str,
        memories: &[Memory],
        depth: &str,
        _context: &str,
        include_patterns: bool,
    ) -> String {
        let mut synthesis = format!(
            "🧠 **KNOWLEDGE SYNTHESIS: {}**\n\n", topic.to_uppercase()
        );
        synthesis
            .push_str(
                &format!(
                    "📊 **Overview**: Synthesized {} memories related to {}\n\n",
                    memories.len(), topic
                ),
            );
        let insights = self.extract_key_insights(memories);
        synthesis.push_str("💡 **Key Insights**:\n");
        for insight in insights {
            synthesis.push_str(&format!("• {}\n", insight));
        }
        synthesis.push_str("\n");
        if depth == "detailed" || depth == "comprehensive" {
            if include_patterns {
                let patterns = self.identify_patterns(memories);
                synthesis.push_str("🔄 **Patterns & Trends**:\n");
                for pattern in patterns {
                    synthesis.push_str(&format!("• {}\n", pattern));
                }
                synthesis.push_str("\n");
            }
            let recommendations = self.generate_recommendations(memories, topic);
            synthesis.push_str("🎯 **Recommendations**:\n");
            for rec in recommendations {
                synthesis.push_str(&format!("• {}\n", rec));
            }
            synthesis.push_str("\n");
        }
        if depth == "comprehensive" {
            synthesis.push_str("🔮 **Future Implications**:\n");
            synthesis.push_str("• Potential challenges and opportunities\n");
            synthesis.push_str("• Areas requiring further investigation\n");
            synthesis.push_str("• Strategic considerations for implementation\n\n");
            synthesis.push_str("❓ **Knowledge Gaps**:\n");
            synthesis.push_str("• Areas needing additional research\n");
            synthesis.push_str("• Questions that remain unanswered\n");
            synthesis.push_str("• Dependencies on external factors\n");
        }
        synthesis
    }
    fn extract_key_insights(&self, memories: &[Memory]) -> Vec<String> {
        let mut insights = Vec::new();
        let mut importance_count = HashMap::new();
        for memory in memories {
            *importance_count.entry(memory.importance.clone()).or_insert(0) += 1;
        }
        if let Some(&high_count) = importance_count.get(&MemoryImportance::High) {
            if high_count > memories.len() / 2 {
                insights
                    .push("High importance topic with significant impact".to_string());
            }
        }
        let recent_memories: Vec<_> = memories
            .iter()
            .filter(|m| m.created_at > chrono::Utc::now() - chrono::Duration::days(30))
            .collect();
        if recent_memories.len() > memories.len() / 2 {
            insights.push("Recently active topic with current relevance".to_string());
        }
        let all_tags: Vec<_> = memories.iter().flat_map(|m| &m.tags).collect();
        let mut tag_counts = HashMap::new();
        for tag in all_tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
        if let Some((most_common_tag, _)) = tag_counts
            .iter()
            .max_by_key(|&(_, count)| count)
        {
            insights.push(format!("Frequently associated with: {}", most_common_tag));
        }
        if insights.is_empty() {
            insights.push("Topic shows consistent learning patterns".to_string());
        }
        insights
    }
    fn identify_patterns(&self, memories: &[Memory]) -> Vec<String> {
        let mut patterns = Vec::new();
        let success_indicators = ["success", "completed", "achieved", "resolved"];
        let failure_indicators = ["failed", "error", "problem", "issue"];
        let success_count = memories
            .iter()
            .filter(|m| {
                success_indicators
                    .iter()
                    .any(|&word| m.meaning.to_lowercase().contains(word))
            })
            .count();
        let failure_count = memories
            .iter()
            .filter(|m| {
                failure_indicators
                    .iter()
                    .any(|&word| m.meaning.to_lowercase().contains(word))
            })
            .count();
        if success_count > failure_count {
            patterns.push("Generally positive outcomes observed".to_string());
        } else if failure_count > success_count {
            patterns
                .push(
                    "Challenges and failures more common - requires careful approach"
                        .to_string(),
                );
        }
        let old_memories = memories
            .iter()
            .filter(|m| m.created_at < chrono::Utc::now() - chrono::Duration::days(90))
            .count();
        let new_memories = memories.len() - old_memories;
        if new_memories > old_memories {
            patterns.push("Increasing frequency of related activities".to_string());
        } else if old_memories > new_memories {
            patterns
                .push(
                    "Decreasing activity - may indicate completion or decreased relevance"
                        .to_string(),
                );
        }
        if patterns.is_empty() {
            patterns.push("Stable, consistent patterns observed".to_string());
        }
        patterns
    }
    fn generate_recommendations(&self, memories: &[Memory], topic: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        let high_importance = memories
            .iter()
            .filter(|m| {
                matches!(
                    m.importance, MemoryImportance::High | MemoryImportance::Critical
                )
            })
            .count();
        if high_importance > memories.len() / 3 {
            recommendations
                .push(format!("Prioritize {} due to high importance pattern", topic));
        }
        let long_term = memories.iter().filter(|m| m.term == MemoryTerm::Long).count();
        if long_term > memories.len() / 2 {
            recommendations
                .push(
                    format!(
                        "{} should be treated as long-term strategic priority", topic
                    ),
                );
        }
        let technical_tags = [
            "api",
            "database",
            "integration",
            "architecture",
            "security",
        ];
        let has_technical = memories
            .iter()
            .any(|m| {
                m
                    .tags
                    .iter()
                    .any(|t| technical_tags.contains(&t.to_lowercase().as_str()))
            });
        if has_technical {
            recommendations
                .push("Technical expertise recommended for implementation".to_string());
        }
        if recommendations.is_empty() {
            recommendations.push(format!("Standard approach suitable for {}", topic));
        }
        recommendations
    }
}
pub struct IdeaRefinementTool {
    todozi: SharedTodozi,
    refinement_history: Arc<Mutex<HashMap<String, Vec<String>>>>,
}
impl IdeaRefinementTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            refinement_history: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
#[async_trait]
impl Tool for IdeaRefinementTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "idea_refinement".to_string(),
            "Intelligent idea refinement with collaborative enhancement, feasibility analysis, and evolution tracking"
                .to_string(),
            vec![
                create_tool_parameter("idea_id", "string", "ID of the idea to refine",
                true), create_tool_parameter("refinement_type", "string",
                "Type of refinement (expand/feasibility/critique/improve)", false),
                create_tool_parameter("collaborators", "string",
                "Comma-separated list of AI agents to involve", false),
                create_tool_parameter("constraints", "string",
                "Project constraints and limitations", false),
                create_tool_parameter("depth", "string",
                "Analysis depth (quick/detailed/comprehensive)", false),
            ],
            "Creative Collaboration".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let idea_id = match kwargs.get("idea_id") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'idea_id' parameter".to_string(),
                    700,
                );
            }
        };
        let refinement_type = kwargs
            .get("refinement_type")
            .and_then(|v| v.as_str())
            .unwrap_or("expand");
        let idea = match list_ideas() {
            Ok(ideas) => ideas.into_iter().find(|i| i.id == idea_id),
            Err(e) => {
                return ToolResult::error(format!("Failed to retrieve ideas: {}", e), 700);
            }
        };
        let idea = match idea {
            Some(i) => i,
            None => return ToolResult::error(format!("Idea {} not found", idea_id), 700),
        };
        let refinement = self.perform_refinement(&idea, refinement_type).await;
        let mut refinement_history = self.refinement_history.lock().await;
        let mut history = refinement_history.get(idea_id).cloned().unwrap_or_default();
        history.push(format!("{} refinement completed", refinement_type));
        refinement_history.insert(idea_id.to_string(), history);
        if refinement.contains("SIGNIFICANTLY IMPROVED") {
            let mut updated_idea = idea.clone();
            updated_idea.idea = self.extract_refined_idea(&refinement);
            updated_idea.updated_at = chrono::Utc::now();
            let _ = save_idea(&updated_idea);
        }
        ToolResult::success(refinement, 700)
    }
}
impl IdeaRefinementTool {
    async fn perform_refinement(&self, idea: &Idea, refinement_type: &str) -> String {
        let mut refinement = format!(
            "🎨 **IDEA REFINEMENT: {}**\n\n", idea.idea.to_uppercase()
        );
        refinement.push_str(&format!("📝 **Original Idea**: {}\n\n", idea.idea));
        match refinement_type {
            "expand" => {
                refinement.push_str("🔍 **EXPANSION ANALYSIS**:\n");
                let expansions = self.expand_idea(idea);
                for exp in expansions {
                    refinement.push_str(&format!("• {}\n", exp));
                }
            }
            "feasibility" => {
                refinement.push_str("⚖️ **FEASIBILITY ASSESSMENT**:\n");
                let assessment = self.assess_feasibility(idea);
                refinement.push_str(&assessment);
            }
            "critique" => {
                refinement.push_str("🔬 **CRITICAL ANALYSIS**:\n");
                let critique = self.critique_idea(idea);
                refinement.push_str(&critique);
            }
            "improve" => {
                refinement.push_str("🚀 **IMPROVEMENT SUGGESTIONS**:\n");
                let improvements = self.suggest_improvements(idea);
                for imp in improvements {
                    refinement.push_str(&format!("• {}\n", imp));
                }
            }
            _ => {
                refinement.push_str("🤔 **GENERAL REFINEMENT**:\n");
                refinement.push_str("• Idea shows potential for development\n");
                refinement.push_str("• Consider technical feasibility\n");
                refinement.push_str("• Evaluate market fit and timing\n");
            }
        }
        refinement.push_str("\n📊 **REFINEMENT SUMMARY**:\n");
        refinement.push_str(&format!("• Refinement Type: {}\n", refinement_type));
        refinement
            .push_str(&format!("• Original Importance: {:?}\n", idea.importance));
        refinement.push_str(&format!("• Tags: {}\n", idea.tags.join(", ")));
        refinement
    }
    fn expand_idea(&self, idea: &Idea) -> Vec<String> {
        let mut expansions = Vec::new();
        expansions
            .push(
                format!("Potential use cases: {}", self.generate_use_cases(& idea.idea)),
            );
        if idea.tags.iter().any(|t| t.to_lowercase().contains("tech")) {
            expansions.push("Technical implementation considerations added".to_string());
        }
        if idea.tags.iter().any(|t| t.to_lowercase().contains("business")) {
            expansions
                .push("Business model and monetization aspects explored".to_string());
        }
        if idea.tags.iter().any(|t| t.to_lowercase().contains("social")) {
            expansions.push("Social impact and community aspects analyzed".to_string());
        }
        expansions
    }
    fn generate_use_cases(&self, idea_text: &str) -> String {
        let idea_lower = idea_text.to_lowercase();
        if idea_lower.contains("app") || idea_lower.contains("application") {
            "mobile app, web app, desktop app, API service".to_string()
        } else if idea_lower.contains("platform") || idea_lower.contains("system") {
            "B2B platform, B2C platform, internal tool, open-source project".to_string()
        } else if idea_lower.contains("tool") || idea_lower.contains("utility") {
            "developer tool, productivity tool, automation tool, analysis tool"
                .to_string()
        } else if idea_lower.contains("service") {
            "SaaS, API service, microservice, cloud service".to_string()
        } else {
            "consumer product, enterprise solution, educational tool, research project"
                .to_string()
        }
    }
    fn assess_feasibility(&self, idea: &Idea) -> String {
        let mut assessment = String::new();
        let technical_score = if idea.idea.to_lowercase().contains("ai")
            || idea.idea.to_lowercase().contains("ml")
        {
            "High - Leverages existing AI capabilities"
        } else if idea.idea.to_lowercase().contains("blockchain")
            || idea.idea.to_lowercase().contains("quantum")
        {
            "Medium - Requires specialized expertise"
        } else {
            "High - Standard technology stack applicable"
        };
        assessment
            .push_str(
                &format!("🛠️ **Technical Feasibility**: {}\n", technical_score),
            );
        let resource_score = match idea.importance {
            IdeaImportance::Breakthrough => "High - Justifies resource allocation",
            IdeaImportance::High => "Medium-High - Significant but manageable",
            IdeaImportance::Medium => "Medium - Standard resources sufficient",
            IdeaImportance::Low => "Low - Minimal resource requirements",
        };
        assessment
            .push_str(&format!("💰 **Resource Requirements**: {}\n", resource_score));
        let time_score = if idea.idea.len() > 200 {
            "Medium-Long - Complex concept requires extended development"
        } else {
            "Short-Medium - Focused idea can be implemented quickly"
        };
        assessment.push_str(&format!("⏱️ **Timeline Estimate**: {}\n", time_score));
        assessment
    }
    fn critique_idea(&self, idea: &Idea) -> String {
        let mut critique = String::new();
        critique.push_str("✅ **Strengths**:\n");
        if idea.idea.len() > 50 {
            critique.push_str("• Well-developed concept with clear articulation\n");
        }
        if !idea.tags.is_empty() {
            critique.push_str("• Properly categorized and tagged\n");
        }
        if matches!(
            idea.importance, IdeaImportance::High | IdeaImportance::Breakthrough
        ) {
            critique.push_str("• High potential impact identified\n");
        }
        critique.push_str("\n❌ **Areas for Improvement**:\n");
        if idea.context.is_none() {
            critique
                .push_str("• Missing context - additional background would help\n");
        }
        if idea.tags.len() < 2 {
            critique.push_str("• Could benefit from more specific categorization\n");
        }
        if idea.share == ShareLevel::Private {
            critique.push_str("• Consider sharing for collaborative input\n");
        }
        critique
    }
    fn suggest_improvements(&self, idea: &Idea) -> Vec<String> {
        let mut improvements = Vec::new();
        if idea.context.is_none() {
            improvements
                .push("Add detailed context and background information".to_string());
        }
        if idea.tags.len() < 3 {
            improvements
                .push("Expand tagging system for better discoverability".to_string());
        }
        if idea.share == ShareLevel::Private {
            improvements
                .push("Consider team sharing for collaborative refinement".to_string());
        }
        if idea.idea.len() < 100 {
            improvements
                .push(
                    "Develop idea further with specific implementation details"
                        .to_string(),
                );
        }
        if improvements.is_empty() {
            improvements
                .push("Idea is well-developed - focus on prototyping".to_string());
        }
        improvements
    }
    fn extract_refined_idea(&self, refinement: &str) -> String {
        if refinement.contains("SIGNIFICANTLY IMPROVED") {
            "Refined version with enhanced clarity and detail".to_string()
        } else {
            "Moderately improved concept".to_string()
        }
    }
}
pub struct PredictiveErrorPreventionTool {
    todozi: SharedTodozi,
    error_patterns: HashMap<String, Vec<String>>,
}
impl PredictiveErrorPreventionTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            error_patterns: HashMap::new(),
        }
    }
    pub fn initialize_patterns(&mut self) {
        self.error_patterns
            .insert(
                "validation".to_string(),
                vec![
                    "missing required fields".to_string(), "invalid data format"
                    .to_string(), "constraint violations".to_string(),
                ],
            );
        self.error_patterns
            .insert(
                "integration".to_string(),
                vec![
                    "api connection failures".to_string(), "authentication issues"
                    .to_string(), "data synchronization problems".to_string(),
                ],
            );
        self.error_patterns
            .insert(
                "performance".to_string(),
                vec![
                    "memory leaks".to_string(), "slow queries".to_string(),
                    "resource exhaustion".to_string(),
                ],
            );
    }
}
#[async_trait]
impl Tool for PredictiveErrorPreventionTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "predictive_error_prevention".to_string(),
            "AI-powered error prediction and prevention with proactive risk mitigation strategies"
                .to_string(),
            vec![
                create_tool_parameter("action", "string",
                "Action or task to analyze for potential errors", true),
                create_tool_parameter("context", "string",
                "Current system context and constraints", false),
                create_tool_parameter("risk_level", "string",
                "Acceptable risk level (low/medium/high)", false),
                create_tool_parameter("include_mitigation", "boolean",
                "Include specific mitigation strategies", false),
            ],
            "Error Intelligence".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::Memory],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let action = match kwargs.get("action") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'action' parameter".to_string(),
                    800,
                );
            }
        };
        let context = kwargs.get("context").and_then(|v| v.as_str()).unwrap_or("");
        let risk_level = kwargs
            .get("risk_level")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let include_mitigation = kwargs
            .get("include_mitigation")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let errors = match list_errors() {
            Ok(errs) => errs,
            Err(e) => {
                return ToolResult::error(
                    format!("Failed to retrieve errors: {}", e),
                    800,
                );
            }
        };
        let predictions = self.predict_errors(action, context, &errors).await;
        let report = self
            .generate_prevention_report(
                action,
                &predictions,
                risk_level,
                include_mitigation,
            )
            .await;
        ToolResult::success(report, 800)
    }
}
impl PredictiveErrorPreventionTool {
    async fn predict_errors(
        &self,
        action: &str,
        context: &str,
        historical_errors: &[Error],
    ) -> Vec<PredictedError> {
        let mut predictions = Vec::new();
        let action_lower = action.to_lowercase();
        if action_lower.contains("database") || action_lower.contains("query")
            || action_lower.contains("data")
        {
            let db_errors: Vec<_> = historical_errors
                .iter()
                .filter(|e| e.category == ErrorCategory::Database)
                .collect();
            if !db_errors.is_empty() {
                predictions
                    .push(PredictedError {
                        error_type: "Database Error".to_string(),
                        probability: self
                            .calculate_probability(
                                db_errors.len(),
                                historical_errors.len(),
                            ),
                        description: "Potential database connectivity or query issues"
                            .to_string(),
                        severity: ErrorSeverity::Medium,
                    });
            }
        }
        if action_lower.contains("api") || action_lower.contains("network")
            || action_lower.contains("external")
        {
            let network_errors: Vec<_> = historical_errors
                .iter()
                .filter(|e| e.category == ErrorCategory::Network)
                .collect();
            if !network_errors.is_empty() {
                predictions
                    .push(PredictedError {
                        error_type: "Network Error".to_string(),
                        probability: self
                            .calculate_probability(
                                network_errors.len(),
                                historical_errors.len(),
                            ),
                        description: "Potential connectivity or API integration issues"
                            .to_string(),
                        severity: ErrorSeverity::High,
                    });
            }
        }
        if action_lower.contains("input") || action_lower.contains("form")
            || action_lower.contains("validate")
        {
            let validation_errors: Vec<_> = historical_errors
                .iter()
                .filter(|e| e.category == ErrorCategory::Validation)
                .collect();
            if !validation_errors.is_empty() {
                predictions
                    .push(PredictedError {
                        error_type: "Validation Error".to_string(),
                        probability: self
                            .calculate_probability(
                                validation_errors.len(),
                                historical_errors.len(),
                            ),
                        description: "Potential input validation or data integrity issues"
                            .to_string(),
                        severity: ErrorSeverity::Medium,
                    });
            }
        }
        if context.to_lowercase().contains("production")
            || context.to_lowercase().contains("live")
        {
            predictions
                .push(PredictedError {
                    error_type: "Production Risk".to_string(),
                    probability: 0.7,
                    description: "Higher risk due to production environment constraints"
                        .to_string(),
                    severity: ErrorSeverity::Critical,
                });
        }
        if predictions.is_empty() {
            predictions
                .push(PredictedError {
                    error_type: "General Risk".to_string(),
                    probability: 0.2,
                    description: "Standard operational risks apply".to_string(),
                    severity: ErrorSeverity::Low,
                });
        }
        predictions
    }
    fn calculate_probability(&self, category_errors: usize, total_errors: usize) -> f64 {
        if total_errors == 0 {
            0.1
        } else {
            (category_errors as f64 / total_errors as f64).min(0.9)
        }
    }
    async fn generate_prevention_report(
        &self,
        action: &str,
        predictions: &[PredictedError],
        risk_level: &str,
        include_mitigation: bool,
    ) -> String {
        let mut report = format!(
            "🔮 **ERROR PREDICTION REPORT: {}**\n\n", action.to_uppercase()
        );
        let overall_risk = predictions
            .iter()
            .map(|p| p.probability * self.severity_weight(&p.severity))
            .sum::<f64>() / predictions.len() as f64;
        let risk_assessment = match overall_risk {
            r if r < 0.3 => "🟢 LOW RISK",
            r if r < 0.6 => "🟡 MEDIUM RISK",
            r if r < 0.8 => "🟠 HIGH RISK",
            _ => "🔴 CRITICAL RISK",
        };
        report
            .push_str(
                &format!(
                    "📊 **Overall Risk Assessment**: {} ({:.1}%)\n\n", risk_assessment,
                    overall_risk * 100.0
                ),
            );
        report.push_str("🎯 **Predicted Error Scenarios**:\n");
        for prediction in predictions {
            let severity_icon = match prediction.severity {
                ErrorSeverity::Low => "🟢",
                ErrorSeverity::Medium => "🟡",
                ErrorSeverity::High => "🟠",
                ErrorSeverity::Critical => "🔴",
            };
            report
                .push_str(
                    &format!(
                        "{} **{}** ({:.1}% probability)\n", severity_icon, prediction
                        .error_type, prediction.probability * 100.0
                    ),
                );
            report.push_str(&format!("   {}\n", prediction.description));
            if include_mitigation {
                let mitigation = self.generate_mitigation(&prediction);
                report.push_str(&format!("   💡 **Prevention**: {}\n", mitigation));
            }
            report.push_str("\n");
        }
        let acceptable_threshold = match risk_level {
            "low" => 0.3,
            "medium" => 0.6,
            "high" => 0.9,
            _ => 0.6,
        };
        if overall_risk > acceptable_threshold {
            report
                .push_str(
                    &format!(
                        "⚠️ **ACTION REQUIRED**: Risk level exceeds {} threshold\n",
                        risk_level
                    ),
                );
            report.push_str("• Consider additional testing\n");
            report.push_str("• Implement additional safeguards\n");
            report.push_str("• Review mitigation strategies\n");
        } else {
            report.push_str("✅ **ACCEPTABLE RISK**: Within acceptable parameters\n");
        }
        report
    }
    fn severity_weight(&self, severity: &ErrorSeverity) -> f64 {
        match severity {
            ErrorSeverity::Low => 1.0,
            ErrorSeverity::Medium => 2.0,
            ErrorSeverity::High => 3.0,
            ErrorSeverity::Critical => 4.0,
        }
    }
    fn generate_mitigation(&self, prediction: &PredictedError) -> String {
        match prediction.error_type.as_str() {
            "Database Error" => {
                "Implement connection pooling and retry logic".to_string()
            }
            "Network Error" => {
                "Add circuit breaker pattern and timeout handling".to_string()
            }
            "Validation Error" => {
                "Implement comprehensive input validation and sanitization".to_string()
            }
            "Production Risk" => {
                "Add feature flags and gradual rollout strategy".to_string()
            }
            _ => "Implement comprehensive error handling and monitoring".to_string(),
        }
    }
}
#[derive(Debug, Clone)]
struct PredictedError {
    error_type: String,
    probability: f64,
    description: String,
    severity: ErrorSeverity,
}
pub fn create_advanced_todozi_tools(
    todozi: SharedTodozi,
) -> Vec<Box<dyn Tool + Send + Sync>> {
    let mut tools: Vec<Box<dyn Tool + Send + Sync>> = vec![
        Box::new(CreateTaskTool::new(todozi.clone())),
        Box::new(SearchTasksTool::new(todozi.clone())),
        Box::new(UpdateTaskTool::new(todozi.clone())),
        Box::new(CreateMemoryTool::new(todozi.clone())),
        Box::new(CreateIdeaTool::new(todozi.clone())),
        Box::new(UnifiedSearchTool::new(todozi.clone())),
        Box::new(ProcessChatMessageTool::new(todozi.clone())),
        Box::new(CreateErrorTool::new(todozi.clone())),
        Box::new(CreateCodeChunkTool::new(todozi.clone())),
        Box::new(ChecklistTool::new(todozi.clone())),
        Box::new(IntelligentTaskPlannerTool::new(todozi.clone())),
        Box::new(MemorySynthesisTool::new(todozi.clone())),
        Box::new(IdeaRefinementTool::new(todozi.clone())),
    ];
    let mut error_prevention_tool = PredictiveErrorPreventionTool::new(todozi.clone());
    error_prevention_tool.initialize_patterns();
    tools.push(Box::new(error_prevention_tool));
    tools
}
pub struct AIAgentOrchestratorTool {
    todozi: SharedTodozi,
    agent_performance: HashMap<String, AgentMetrics>,
    collaboration_patterns: HashMap<String, Vec<String>>,
}
impl AIAgentOrchestratorTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            agent_performance: HashMap::new(),
            collaboration_patterns: HashMap::new(),
        }
    }
    pub fn initialize_patterns(&mut self) {
        self.collaboration_patterns
            .insert(
                "planning".to_string(),
                vec!["architect".to_string(), "strategist".to_string(),],
            );
        self.collaboration_patterns
            .insert(
                "development".to_string(),
                vec![
                    "developer".to_string(), "tester".to_string(), "reviewer"
                    .to_string(),
                ],
            );
        self.collaboration_patterns
            .insert(
                "creative".to_string(),
                vec!["designer".to_string(), "innovator".to_string(),],
            );
    }
}
#[async_trait]
impl Tool for AIAgentOrchestratorTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "ai_agent_orchestration".to_string(),
            "Intelligent AI agent coordination and task assignment with performance optimization"
                .to_string(),
            vec![
                create_tool_parameter("task_description", "string",
                "Description of the task to orchestrate", true),
                create_tool_parameter("task_type", "string",
                "Type of task (planning/development/creative/research)", false),
                create_tool_parameter("complexity", "string", "Task complexity level",
                false), create_tool_parameter("deadline", "string",
                "Task deadline if applicable", false),
                create_tool_parameter("required_skills", "string",
                "Comma-separated required skills", false),
            ],
            "Agent Orchestration".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::Memory],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let task_description = match kwargs.get("task_description") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'task_description' parameter".to_string(),
                    900,
                );
            }
        };
        let task_type = kwargs
            .get("task_type")
            .and_then(|v| v.as_str())
            .unwrap_or("general");
        let complexity = kwargs
            .get("complexity")
            .and_then(|v| v.as_str())
            .unwrap_or("medium");
        let required_skills: Vec<String> = kwargs
            .get("required_skills")
            .and_then(|v| v.as_str())
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|| Vec::new());
        let orchestration_plan = self
            .create_orchestration_plan(
                task_description,
                task_type,
                complexity,
                &required_skills,
            )
            .await;
        ToolResult::success(orchestration_plan, 900)
    }
}
impl AIAgentOrchestratorTool {
    async fn create_orchestration_plan(
        &self,
        task: &str,
        task_type: &str,
        complexity: &str,
        skills: &[String],
    ) -> String {
        let mut plan = format!(
            "🎭 **AI AGENT ORCHESTRATION PLAN**\n\n📋 **Task**: {}\n\n", task
        );
        let agent_team = self.select_optimal_agent_team(task_type, complexity, skills);
        plan.push_str("👥 **Recommended Agent Team**:\n");
        for (role, agent) in &agent_team {
            let performance = self
                .agent_performance
                .get(agent)
                .map(|m| format!(" (Success: {:.1}%)", m.success_rate * 100.0))
                .unwrap_or_default();
            plan.push_str(&format!("• **{}**: {}{}\n", role, agent, performance));
        }
        let workflow = self.design_workflow(&agent_team, complexity);
        plan.push_str("\n🔄 **Execution Workflow**:\n");
        for (step, details) in workflow {
            plan.push_str(&format!("{}. {}\n", step, details));
        }
        let risks = self.assess_orchestration_risks(&agent_team, complexity);
        plan.push_str("\n⚠️ **Orchestration Risks**:\n");
        for risk in risks {
            plan.push_str(&format!("• {}\n", risk));
        }
        plan.push_str("\n📊 **Success Metrics**:\n");
        plan.push_str("• Task completion within estimated time\n");
        plan.push_str("• Quality standards met by all agents\n");
        plan.push_str("• Effective collaboration achieved\n");
        plan.push_str("• Knowledge transfer successful\n");
        plan
    }
    fn select_optimal_agent_team(
        &self,
        task_type: &str,
        complexity: &str,
        skills: &[String],
    ) -> BTreeMap<String, String> {
        let mut team = BTreeMap::new();
        let base_agents = self
            .collaboration_patterns
            .get(task_type)
            .cloned()
            .unwrap_or(vec!["general_agent".to_string()]);
        match complexity {
            "low" => {
                team.insert(
                    "Primary".to_string(),
                    base_agents.get(0).cloned().unwrap_or_default(),
                );
            }
            "medium" => {
                team.insert(
                    "Lead".to_string(),
                    base_agents.get(0).cloned().unwrap_or_default(),
                );
                if base_agents.len() > 1 {
                    team.insert("Support".to_string(), base_agents[1].clone());
                }
            }
            "high" | "extreme" => {
                let mut role_counter = 1;
                for agent in &base_agents {
                    team.insert(format!("Agent {}", role_counter), agent.clone());
                    role_counter += 1;
                }
                team.insert("Coordinator".to_string(), "orchestrator_agent".to_string());
            }
            _ => {
                team.insert("Primary".to_string(), "general_agent".to_string());
            }
        }
        for skill in skills {
            let specialized_agent = match skill.to_lowercase().as_str() {
                "security" => "security_specialist",
                "performance" => "performance_engineer",
                "ux" | "ui" => "ux_specialist",
                "database" => "data_engineer",
                "api" => "integration_specialist",
                _ => continue,
            };
            team.insert(skill.clone(), specialized_agent.to_string());
        }
        team
    }
    fn design_workflow(
        &self,
        _team: &BTreeMap<String, String>,
        complexity: &str,
    ) -> Vec<(u32, String)> {
        let mut workflow = Vec::new();
        let mut step = 1;
        match complexity {
            "low" => {
                workflow.push((step, "Single agent executes complete task".to_string()));
            }
            "medium" => {
                workflow
                    .push((
                        step,
                        "Lead agent analyzes requirements and creates plan".to_string(),
                    ));
                step += 1;
                workflow
                    .push((
                        step,
                        "Support agent provides additional capabilities".to_string(),
                    ));
                step += 1;
                workflow
                    .push((
                        step,
                        "Lead agent integrates work and finalizes".to_string(),
                    ));
            }
            "high" | "extreme" => {
                workflow
                    .push((
                        step,
                        "Coordinator establishes communication protocols".to_string(),
                    ));
                step += 1;
                workflow
                    .push((
                        step,
                        "Parallel execution by specialized agents".to_string(),
                    ));
                step += 1;
                workflow.push((step, "Regular progress synchronization".to_string()));
                step += 1;
                workflow
                    .push((
                        step,
                        "Coordinator integrates all contributions".to_string(),
                    ));
                step += 1;
                workflow.push((step, "Quality assurance and final review".to_string()));
            }
            _ => {
                workflow
                    .push((step, "Sequential agent execution as needed".to_string()));
            }
        }
        workflow
    }
    fn assess_orchestration_risks(
        &self,
        team: &BTreeMap<String, String>,
        complexity: &str,
    ) -> Vec<String> {
        let mut risks = Vec::new();
        if team.len() > 3 {
            risks.push("Communication overhead may impact efficiency".to_string());
        }
        if complexity == "extreme" {
            risks
                .push(
                    "High coordination complexity requires robust orchestration"
                        .to_string(),
                );
        }
        for (role, agent) in team {
            if let Some(metrics) = self.agent_performance.get(agent) {
                if metrics.success_rate < 0.8 {
                    risks.push(format!("{} agent has lower success rate", role));
                }
                if metrics.average_completion_time > Duration::hours(24) {
                    risks
                        .push(
                            format!(
                                "{} agent typically takes longer to complete tasks", role
                            ),
                        );
                }
            }
        }
        if risks.is_empty() {
            risks
                .push(
                    "Standard orchestration risks apply - monitor progress closely"
                        .to_string(),
                );
        }
        risks
    }
}
#[derive(Debug, Clone)]
struct AgentMetrics {
    success_rate: f64,
    average_completion_time: Duration,
    specialization_score: f64,
    collaboration_rating: f64,
}
pub async fn initialize_grok_level_todozi_system_with_embedding(
    enable_embeddings: bool,
) -> std::result::Result<(SharedTodozi, Option<TodoziEmbeddingService>), TodoziError> {
    crate::init().await?;
    let storage = Storage::new().await?;
    let todozi = Arc::new(Mutex::new(storage));
    let embedding_service = if enable_embeddings {
        let config = TodoziEmbeddingConfig {
            model_name: "all-MiniLM-L6-v2".to_string(),
            max_results: 10,
            similarity_threshold: 0.7,
            cache_ttl_seconds: 3600,
            clustering_threshold: 0.8,
            dimensions: 384,
            enable_clustering: false,
        };
        match TodoziEmbeddingService::new(config).await {
            Ok(service) => Some(service),
            Err(e) => {
                eprintln!("Warning: Failed to initialize embedding service: {}", e);
                None
            }
        }
    } else {
        None
    };
    Ok((todozi, embedding_service))
}
pub struct CodeQualityIntelligenceTool {
    todozi: SharedTodozi,
    quality_patterns: HashMap<String, QualityMetrics>,
}
impl CodeQualityIntelligenceTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            quality_patterns: HashMap::new(),
        }
    }
    pub fn initialize_quality_patterns(&mut self) {
        self.quality_patterns
            .insert(
                "rust".to_string(),
                QualityMetrics {
                    max_complexity: 15,
                    min_test_coverage: 80.0,
                    max_duplication: 5.0,
                    required_patterns: vec![
                        "error_handling".to_string(), "documentation".to_string()
                    ],
                },
            );
        self.quality_patterns
            .insert(
                "javascript".to_string(),
                QualityMetrics {
                    max_complexity: 10,
                    min_test_coverage: 70.0,
                    max_duplication: 10.0,
                    required_patterns: vec![
                        "async_handling".to_string(), "error_boundaries".to_string()
                    ],
                },
            );
        self.quality_patterns
            .insert(
                "python".to_string(),
                QualityMetrics {
                    max_complexity: 12,
                    min_test_coverage: 75.0,
                    max_duplication: 8.0,
                    required_patterns: vec![
                        "type_hints".to_string(), "docstrings".to_string()
                    ],
                },
            );
    }
}
#[async_trait]
impl Tool for CodeQualityIntelligenceTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "code_quality_intelligence".to_string(),
            "AI-powered code quality analysis with intelligent recommendations and automated improvements"
                .to_string(),
            vec![
                create_tool_parameter("code", "string", "Code to analyze for quality",
                true), create_tool_parameter("language", "string",
                "Programming language", false), create_tool_parameter("context",
                "string", "Code context and purpose", false),
                create_tool_parameter("quality_level", "string",
                "Required quality level (basic/good/excellent)", false),
                create_tool_parameter("include_fixes", "boolean",
                "Include automated fix suggestions", false),
            ],
            "Code Quality".to_string(),
            vec![ResourceLock::Memory],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let code = match kwargs.get("code") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'code' parameter".to_string(),
                    1000,
                );
            }
        };
        let language = kwargs
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let context = kwargs.get("context").and_then(|v| v.as_str()).unwrap_or("");
        let quality_level = kwargs
            .get("quality_level")
            .and_then(|v| v.as_str())
            .unwrap_or("good");
        let include_fixes = kwargs
            .get("include_fixes")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let analysis = self
            .perform_quality_analysis(
                code,
                language,
                context,
                quality_level,
                include_fixes,
            )
            .await;
        ToolResult::success(analysis, 1000)
    }
}
impl CodeQualityIntelligenceTool {
    async fn perform_quality_analysis(
        &self,
        code: &str,
        language: &str,
        context: &str,
        quality_level: &str,
        include_fixes: bool,
    ) -> String {
        let mut analysis = format!(
            "🔍 **CODE QUALITY ANALYSIS**\n\n📝 **Language**: {}\n📋 **Context**: {}\n\n",
            language, context
        );
        let structure_score = self.analyze_code_structure(code, language);
        analysis
            .push_str(
                &format!(
                    "🏗️ **Structural Quality**: {:.1}/10\n{}\n\n", structure_score
                    .score, structure_score.feedback
                ),
            );
        let complexity_score = self.analyze_complexity(code, language);
        analysis
            .push_str(
                &format!(
                    "🧠 **Complexity Analysis**: {:.1}/10\n{}\n\n", complexity_score
                    .score, complexity_score.feedback
                ),
            );
        let practices_score = self.check_best_practices(code, language, quality_level);
        analysis
            .push_str(
                &format!(
                    "✨ **Best Practices**: {:.1}/10\n{}\n\n", practices_score.score,
                    practices_score.feedback
                ),
            );
        let security_score = self.analyze_security(code, language);
        analysis
            .push_str(
                &format!(
                    "🔒 **Security Assessment**: {:.1}/10\n{}\n\n", security_score
                    .score, security_score.feedback
                ),
            );
        let maintainability_score = self.analyze_maintainability(code, language);
        analysis
            .push_str(
                &format!(
                    "🔧 **Maintainability**: {:.1}/10\n{}\n\n", maintainability_score
                    .score, maintainability_score.feedback
                ),
            );
        let overall_score = (structure_score.score + complexity_score.score
            + practices_score.score + security_score.score + maintainability_score.score)
            / 5.0;
        let overall_assessment = match overall_score {
            s if s >= 8.5 => "🏆 EXCELLENT - Production ready",
            s if s >= 7.0 => "✅ GOOD - Minor improvements suggested",
            s if s >= 5.0 => "⚠️ FAIR - Significant improvements needed",
            _ => "❌ POOR - Major refactoring required",
        };
        analysis
            .push_str(
                &format!(
                    "🎯 **OVERALL ASSESSMENT**: {} ({:.1}/10)\n\n", overall_assessment,
                    overall_score
                ),
            );
        if include_fixes && overall_score < 7.0 {
            let fixes = self
                .generate_fixes(
                    code,
                    language,
                    &structure_score,
                    &complexity_score,
                    &practices_score,
                );
            analysis.push_str("💡 **RECOMMENDED FIXES**:\n");
            for fix in fixes {
                analysis.push_str(&format!("• {}\n", fix));
            }
            analysis.push_str("\n");
        }
        if overall_score < 9.0 {
            analysis.push_str("🚀 **QUALITY IMPROVEMENT ROADMAP**:\n");
            let roadmap = self.create_improvement_roadmap(overall_score, quality_level);
            for item in roadmap {
                analysis.push_str(&format!("• {}\n", item));
            }
        }
        analysis
    }
    fn analyze_code_structure(&self, code: &str, language: &str) -> QualityScore {
        let lines = code.lines().count();
        let functions = self.count_functions(code, language);
        let classes = self.count_classes(code, language);
        let mut score: f64 = 5.0;
        let mut feedback = Vec::new();
        if functions > 0 {
            let avg_lines_per_function = lines as f64 / functions as f64;
            match avg_lines_per_function {
                l if l < 20.0 => {
                    score += 2.0;
                    feedback.push("Excellent function size distribution".to_string());
                }
                l if l < 50.0 => {
                    score += 1.0;
                    feedback
                        .push(
                            "Good function size - consider breaking down very large functions"
                                .to_string(),
                        );
                }
                _ => {
                    feedback
                        .push(
                            "Some functions are too large - consider refactoring"
                                .to_string(),
                        );
                }
            }
        }
        if classes > 0 && functions > 0 {
            let functions_per_class = functions as f64 / classes as f64;
            if functions_per_class > 10.0 {
                feedback
                    .push(
                        "Classes may be doing too much - consider single responsibility principle"
                            .to_string(),
                    );
            } else {
                score += 1.0;
                feedback.push("Good class organization".to_string());
            }
        }
        QualityScore {
            score: score.min(10.0),
            feedback: feedback.join("; "),
        }
    }
    fn analyze_complexity(&self, code: &str, language: &str) -> QualityScore {
        let mut score: f64 = 8.0;
        let mut feedback = Vec::new();
        let max_nesting = self.calculate_max_nesting(code, language);
        match max_nesting {
            n if n <= 3 => {
                feedback.push("Good nesting level - code is readable".to_string());
            }
            n if n <= 5 => {
                score -= 1.0;
                feedback
                    .push("Moderate nesting - consider extracting methods".to_string());
            }
            _ => {
                score -= 2.0;
                feedback.push("High nesting complexity - refactor urgently".to_string());
            }
        }
        let long_lines = code.lines().filter(|line| line.len() > 120).count();
        if long_lines > code.lines().count() / 10 {
            score -= 1.0;
            feedback.push("Some lines are too long - break them up".to_string());
        }
        QualityScore {
            score: score.max(1.0),
            feedback: feedback.join("; "),
        }
    }
    fn check_best_practices(
        &self,
        code: &str,
        language: &str,
        quality_level: &str,
    ) -> QualityScore {
        let mut score: f64 = 6.0;
        let mut feedback = Vec::new();
        match language.to_lowercase().as_str() {
            "rust" => {
                if code.contains("unwrap()") && !code.contains("expect(") {
                    score -= 1.0;
                    feedback
                        .push(
                            "Consider using expect() instead of unwrap() for better error messages"
                                .to_string(),
                        );
                }
                if code.contains("///") {
                    score += 1.0;
                    feedback.push("Good documentation practices".to_string());
                }
            }
            "javascript" | "typescript" => {
                if code.contains("console.log") && quality_level == "excellent" {
                    score -= 1.0;
                    feedback
                        .push(
                            "Remove console.log statements in production code"
                                .to_string(),
                        );
                }
                if code.contains("async") && code.contains("await") {
                    score += 1.0;
                    feedback.push("Proper async/await usage".to_string());
                }
            }
            "python" => {
                if code.contains("def ") && code.contains("\"\"\"") {
                    score += 1.0;
                    feedback.push("Good docstring usage".to_string());
                }
                if code.contains("type:") || code.contains("->") {
                    score += 0.5;
                    feedback.push("Type hints are present".to_string());
                }
            }
            _ => {}
        }
        QualityScore {
            score: score.min(10.0).max(1.0),
            feedback: feedback.join("; "),
        }
    }
    fn analyze_security(&self, code: &str, language: &str) -> QualityScore {
        let mut score: f64 = 8.0;
        let mut feedback = Vec::new();
        match language.to_lowercase().as_str() {
            "javascript" | "typescript" => {
                if code.contains("eval(") {
                    score -= 3.0;
                    feedback.push("Dangerous eval() usage detected".to_string());
                }
                if code.contains("innerHTML") && !code.contains("sanitiz") {
                    score -= 1.0;
                    feedback.push("innerHTML usage without sanitization".to_string());
                }
            }
            "rust" => {
                if code.contains("unsafe") {
                    score -= 1.0;
                    feedback
                        .push(
                            "Unsafe code blocks present - review carefully".to_string(),
                        );
                }
            }
            "python" => {
                if code.contains("exec(") || code.contains("eval(") {
                    score -= 2.0;
                    feedback.push("Dangerous exec/eval usage".to_string());
                }
            }
            _ => {}
        }
        if code.to_lowercase().contains("select") && code.contains("+")
            && !code.contains("prepare")
        {
            score -= 1.0;
            feedback.push("Potential SQL injection vulnerability".to_string());
        }
        if feedback.is_empty() {
            feedback.push("No obvious security issues detected".to_string());
        }
        QualityScore {
            score: score.max(1.0),
            feedback: feedback.join("; "),
        }
    }
    fn analyze_maintainability(&self, code: &str, language: &str) -> QualityScore {
        let mut score: f64 = 7.0;
        let mut feedback = Vec::new();
        let comment_lines = code
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                match language {
                    "rust" => trimmed.starts_with("//") || trimmed.starts_with("///"),
                    "javascript" | "typescript" => {
                        trimmed.starts_with("//") || trimmed.starts_with("/*")
                    }
                    "python" => trimmed.starts_with("#"),
                    _ => false,
                }
            })
            .count();
        let total_lines = code.lines().count();
        let comment_ratio = if total_lines > 0 {
            comment_lines as f64 / total_lines as f64
        } else {
            0.0
        };
        match comment_ratio {
            r if r > 0.3 => {
                score += 1.0;
                feedback.push("Excellent documentation coverage".to_string());
            }
            r if r > 0.15 => {
                feedback.push("Good documentation level".to_string());
            }
            r if r > 0.05 => {
                score -= 0.5;
                feedback.push("Documentation could be improved".to_string());
            }
            _ => {
                score -= 1.0;
                feedback.push("Code lacks documentation".to_string());
            }
        }
        let magic_numbers = code
            .split_whitespace()
            .filter(|word| {
                word.parse::<i32>().is_ok() && word.len() > 1 && !word.starts_with("0")
            })
            .count();
        if magic_numbers > 5 {
            score -= 0.5;
            feedback
                .push(
                    "Consider replacing magic numbers with named constants".to_string(),
                );
        }
        QualityScore {
            score: score.min(10.0).max(1.0),
            feedback: feedback.join("; "),
        }
    }
    fn count_functions(&self, code: &str, language: &str) -> usize {
        match language.to_lowercase().as_str() {
            "rust" => code.matches("fn ").count(),
            "javascript" | "typescript" => {
                code.matches("function ").count() + code.matches("=> ").count()
            }
            "python" => code.matches("def ").count(),
            _ => 0,
        }
    }
    fn count_classes(&self, code: &str, language: &str) -> usize {
        match language.to_lowercase().as_str() {
            "rust" => code.matches("struct ").count() + code.matches("enum ").count(),
            "javascript" | "typescript" => code.matches("class ").count(),
            "python" => code.matches("class ").count(),
            _ => 0,
        }
    }
    fn calculate_max_nesting(&self, code: &str, language: &str) -> usize {
        let mut max_nesting = 0;
        let mut current_nesting: usize = 0;
        for line in code.lines() {
            let trimmed = line.trim();
            match language.to_lowercase().as_str() {
                "rust" | "javascript" | "typescript" => {
                    for ch in trimmed.chars() {
                        match ch {
                            '{' | '(' => current_nesting += 1,
                            '}' | ')' => {
                                max_nesting = max_nesting.max(current_nesting);
                                current_nesting = current_nesting.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                }
                "python" => {
                    if trimmed.ends_with(':') {
                        current_nesting += 1;
                        max_nesting = max_nesting.max(current_nesting);
                    } else if trimmed.is_empty() && current_nesting > 0 {
                        current_nesting = current_nesting.saturating_sub(1);
                    }
                }
                _ => {}
            }
        }
        max_nesting
    }
    fn generate_fixes(
        &self,
        _code: &str,
        _language: &str,
        structure: &QualityScore,
        complexity: &QualityScore,
        practices: &QualityScore,
    ) -> Vec<String> {
        let mut fixes = Vec::new();
        if structure.score < 7.0 {
            fixes
                .push(
                    "Refactor large functions into smaller, focused methods".to_string(),
                );
            fixes.push("Extract common functionality into shared utilities".to_string());
        }
        if complexity.score < 7.0 {
            fixes.push("Reduce nested conditionals by extracting methods".to_string());
            fixes.push("Use early returns to reduce nesting depth".to_string());
            fixes.push("Break long lines into readable chunks".to_string());
        }
        if practices.score < 7.0 {
            fixes.push("Add comprehensive error handling".to_string());
            fixes
                .push(
                    "Implement proper logging instead of print statements".to_string(),
                );
            fixes.push("Add input validation and sanitization".to_string());
        }
        if fixes.is_empty() {
            fixes
                .push(
                    "Code structure is generally good - focus on documentation"
                        .to_string(),
                );
        }
        fixes
    }
    fn create_improvement_roadmap(
        &self,
        current_score: f64,
        target_level: &str,
    ) -> Vec<String> {
        let target_score = match target_level {
            "excellent" => 9.0,
            "good" => 7.5,
            "basic" => 6.0,
            _ => 7.0,
        };
        let gap = target_score - current_score;
        let mut roadmap = Vec::new();
        if gap > 0.0 {
            roadmap
                .push(format!("Target: Achieve {:.1}/10 quality score", target_score));
            if gap > 2.0 {
                roadmap
                    .push(
                        "Phase 1: Address critical issues (security, complexity)"
                            .to_string(),
                    );
                roadmap
                    .push("Phase 2: Improve structure and best practices".to_string());
                roadmap
                    .push(
                        "Phase 3: Enhance documentation and maintainability".to_string(),
                    );
            } else {
                roadmap.push("Focus on incremental improvements".to_string());
                roadmap.push("Prioritize maintainability and readability".to_string());
            }
            roadmap.push("Implement automated testing for quality gates".to_string());
            roadmap.push("Set up code review processes".to_string());
        } else {
            roadmap
                .push("Quality standards already met - maintain excellence".to_string());
        }
        roadmap
    }
}
#[derive(Debug, Clone)]
struct QualityScore {
    score: f64,
    feedback: String,
}
#[derive(Debug, Clone)]
struct QualityMetrics {
    max_complexity: u32,
    min_test_coverage: f64,
    max_duplication: f64,
    required_patterns: Vec<String>,
}
pub struct LearningAnalyticsTool {
    todozi: SharedTodozi,
    analytics_cache: HashMap<String, LearningInsights>,
}
impl LearningAnalyticsTool {
    pub fn new(todozi: SharedTodozi) -> Self {
        Self {
            todozi,
            analytics_cache: HashMap::new(),
        }
    }
}
#[async_trait]
impl Tool for LearningAnalyticsTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "learning_analytics".to_string(),
            "Advanced learning analytics providing insights into knowledge acquisition, skill development, and performance trends"
                .to_string(),
            vec![
                create_tool_parameter("time_period", "string",
                "Analysis time period (week/month/quarter/year)", false),
                create_tool_parameter("focus_area", "string",
                "Specific focus area (tasks/memories/ideas/errors)", false),
                create_tool_parameter("user_id", "string",
                "User ID to analyze (optional)", false),
                create_tool_parameter("include_predictions", "boolean",
                "Include predictive insights", false),
                create_tool_parameter("detailed_metrics", "boolean",
                "Include detailed performance metrics", false),
            ],
            "Learning Analytics".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::Memory],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let time_period = kwargs
            .get("time_period")
            .and_then(|v| v.as_str())
            .unwrap_or("month");
        let focus_area = kwargs
            .get("focus_area")
            .and_then(|v| v.as_str())
            .unwrap_or("all");
        let user_id = kwargs.get("user_id").and_then(|v| v.as_str());
        let include_predictions = kwargs
            .get("include_predictions")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let detailed_metrics = kwargs
            .get("detailed_metrics")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let analytics = self
            .generate_learning_analytics(
                time_period,
                focus_area,
                user_id,
                include_predictions,
                detailed_metrics,
            )
            .await;
        ToolResult::success(analytics, 1100)
    }
}
impl LearningAnalyticsTool {
    async fn generate_learning_analytics(
        &self,
        time_period: &str,
        focus_area: &str,
        user_id: Option<&str>,
        include_predictions: bool,
        detailed_metrics: bool,
    ) -> String {
        let mut analytics = format!(
            "📊 **LEARNING ANALYTICS DASHBOARD**\n\n📅 **Period**: {}\n🎯 **Focus**: {}\n\n",
            time_period, focus_area
        );
        let (start_date, end_date) = self.calculate_date_range(time_period);
        let data = self
            .collect_learning_data(focus_area, user_id, start_date, end_date)
            .await;
        let core_metrics = self.calculate_core_metrics(&data);
        analytics.push_str("📈 **CORE LEARNING METRICS**:\n");
        for (metric, value) in core_metrics {
            analytics.push_str(&format!("• {}: {}\n", metric, value));
        }
        analytics.push_str("\n");
        let patterns = self.analyze_learning_patterns(&data);
        analytics.push_str("🔄 **LEARNING PATTERNS**:\n");
        for pattern in patterns {
            analytics.push_str(&format!("• {}\n", pattern));
        }
        analytics.push_str("\n");
        let skills = self.analyze_skill_development(&data);
        analytics.push_str("🎓 **SKILL DEVELOPMENT**:\n");
        for skill in skills {
            analytics.push_str(&format!("• {}\n", skill));
        }
        analytics.push_str("\n");
        let retention = self.analyze_knowledge_retention(&data);
        analytics.push_str("🧠 **KNOWLEDGE RETENTION**:\n");
        analytics.push_str(&retention);
        analytics.push_str("\n");
        if detailed_metrics {
            let performance = self.calculate_detailed_performance(&data);
            analytics.push_str("📊 **DETAILED PERFORMANCE METRICS**:\n");
            for (category, metrics) in performance {
                analytics.push_str(&format!("**{}**:\n", category));
                for (metric, value) in metrics {
                    analytics.push_str(&format!("  • {}: {}\n", metric, value));
                }
            }
            analytics.push_str("\n");
        }
        if include_predictions {
            let predictions = self.generate_predictive_insights(&data);
            analytics.push_str("🔮 **PREDICTIVE INSIGHTS**:\n");
            for prediction in predictions {
                analytics.push_str(&format!("• {}\n", prediction));
            }
            analytics.push_str("\n");
        }
        let recommendations = self.generate_learning_recommendations(&data);
        analytics.push_str("💡 **LEARNING RECOMMENDATIONS**:\n");
        for rec in recommendations {
            analytics.push_str(&format!("• {}\n", rec));
        }
        analytics
    }
    fn calculate_date_range(&self, time_period: &str) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let start_date = match time_period {
            "week" => now - Duration::days(7),
            "month" => now - Duration::days(30),
            "quarter" => now - Duration::days(90),
            "year" => now - Duration::days(365),
            _ => now - Duration::days(30),
        };
        (start_date, now)
    }
    async fn collect_learning_data(
        &self,
        focus_area: &str,
        user_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> LearningData {
        let mut data = LearningData {
            tasks: Vec::new(),
            memories: Vec::new(),
            ideas: Vec::new(),
            errors: Vec::new(),
        };
        let collect_all = focus_area == "all";
        if collect_all || focus_area == "tasks" {
            if let Ok(tasks) = self
                .todozi
                .lock()
                .await
                .list_tasks_across_projects(&TaskFilters::default())
            {
                data.tasks = tasks
                    .into_iter()
                    .filter(|t| t.created_at >= start_date && t.created_at <= end_date)
                    .filter(|t| user_id.map_or(true, |uid| t.user_id == uid))
                    .collect();
            }
        }
        if collect_all || focus_area == "memories" {
            if let Ok(memories) = list_memories() {
                data.memories = memories
                    .into_iter()
                    .filter(|m| m.created_at >= start_date && m.created_at <= end_date)
                    .filter(|m| user_id.map_or(true, |uid| m.user_id == uid))
                    .collect();
            }
        }
        if collect_all || focus_area == "ideas" {
            if let Ok(ideas) = list_ideas() {
                data.ideas = ideas
                    .into_iter()
                    .filter(|i| i.created_at >= start_date && i.created_at <= end_date)
                    .collect();
            }
        }
        if collect_all || focus_area == "errors" {
            if let Ok(errors) = list_errors() {
                data.errors = errors
                    .into_iter()
                    .filter(|e| e.created_at >= start_date && e.created_at <= end_date)
                    .collect();
            }
        }
        data
    }
    fn calculate_core_metrics(&self, data: &LearningData) -> Vec<(String, String)> {
        let mut metrics = Vec::new();
        let completed_tasks = data
            .tasks
            .iter()
            .filter(|t| t.status == Status::Done)
            .count();
        let total_tasks = data.tasks.len();
        let completion_rate = if total_tasks > 0 {
            (completed_tasks as f64 / total_tasks as f64 * 100.0).round()
        } else {
            0.0
        };
        metrics
            .push((
                "Task Completion Rate".to_string(),
                format!("{:.1}%", completion_rate),
            ));
        let long_term_memories = data
            .memories
            .iter()
            .filter(|m| m.term == MemoryTerm::Long)
            .count();
        let total_memories = data.memories.len();
        let retention_rate = if total_memories > 0 {
            (long_term_memories as f64 / total_memories as f64 * 100.0).round()
        } else {
            0.0
        };
        metrics
            .push((
                "Long-term Memory Rate".to_string(),
                format!("{:.1}%", retention_rate),
            ));
        let total_ideas = data.ideas.len();
        metrics.push(("Ideas Generated".to_string(), total_ideas.to_string()));
        let resolved_errors = data.errors.iter().filter(|e| e.resolved).count();
        let total_errors = data.errors.len();
        let error_resolution_rate = if total_errors > 0 {
            (resolved_errors as f64 / total_errors as f64 * 100.0).round()
        } else {
            0.0
        };
        metrics
            .push((
                "Error Resolution Rate".to_string(),
                format!("{:.1}%", error_resolution_rate),
            ));
        metrics
    }
    fn analyze_learning_patterns(&self, data: &LearningData) -> Vec<String> {
        let mut patterns = Vec::new();
        let urgent_tasks = data
            .tasks
            .iter()
            .filter(|t| {
                t.priority == Priority::Critical || t.priority == Priority::Urgent
            })
            .count();
        if urgent_tasks > data.tasks.len() / 2 {
            patterns
                .push(
                    "High priority task focus - potentially reactive work style"
                        .to_string(),
                );
        }
        let emotion_memories: HashSet<_> = data
            .memories
            .iter()
            .filter_map(|m| match &m.memory_type {
                MemoryType::Emotional(emotion) => Some(emotion.clone()),
                _ => None,
            })
            .collect();
        if emotion_memories.len() > 2 {
            patterns
                .push(
                    "Strong emotional learning component - experiences drive learning"
                        .to_string(),
                );
        }
        let high_impact_ideas = data
            .ideas
            .iter()
            .filter(|i| {
                i.importance == IdeaImportance::High
                    || i.importance == IdeaImportance::Breakthrough
            })
            .count();
        if high_impact_ideas > data.ideas.len() / 3 {
            patterns.push("Focus on high-impact innovations".to_string());
        }
        if patterns.is_empty() {
            patterns.push("Balanced learning across multiple domains".to_string());
        }
        patterns
    }
    fn analyze_skill_development(&self, data: &LearningData) -> Vec<String> {
        let mut skills = Vec::new();
        let mut skill_counts = HashMap::new();
        for task in &data.tasks {
            for tag in &task.tags {
                *skill_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        let mut skill_vec: Vec<_> = skill_counts.into_iter().collect();
        skill_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for (skill, count) in skill_vec.into_iter().take(5) {
            skills.push(format!("{}: {} tasks", skill, count));
        }
        if skills.is_empty() {
            skills.push("No specific skill patterns identified yet".to_string());
        }
        skills
    }
    fn analyze_knowledge_retention(&self, data: &LearningData) -> String {
        let long_term = data
            .memories
            .iter()
            .filter(|m| m.term == MemoryTerm::Long)
            .count();
        let short_term = data
            .memories
            .iter()
            .filter(|m| m.term == MemoryTerm::Short)
            .count();
        if long_term + short_term == 0 {
            return "No memory data available for retention analysis".to_string();
        }
        let retention_ratio = long_term as f64 / (long_term + short_term) as f64;
        let retention_percentage = (retention_ratio * 100.0).round();
        match retention_percentage as u32 {
            r if r > 70 => {
                format!(
                    "Excellent retention ({:.1}%) - Strong long-term learning focus",
                    retention_percentage
                )
            }
            r if r > 50 => {
                format!(
                    "Good retention ({:.1}%) - Balanced short and long-term learning",
                    retention_percentage
                )
            }
            r if r > 30 => {
                format!(
                    "Fair retention ({:.1}%) - Consider more long-term knowledge building",
                    retention_percentage
                )
            }
            _ => {
                format!(
                    "Low retention ({:.1}%) - Focus on building lasting knowledge",
                    retention_percentage
                )
            }
        }
    }
    fn calculate_detailed_performance(
        &self,
        data: &LearningData,
    ) -> BTreeMap<String, Vec<(String, String)>> {
        let mut performance = BTreeMap::new();
        let mut task_metrics = Vec::new();
        let avg_progress = if !data.tasks.is_empty() {
            data.tasks.iter().filter_map(|t| t.progress).sum::<u8>() as f64
                / data.tasks.len() as f64
        } else {
            0.0
        };
        task_metrics
            .push(("Average Progress".to_string(), format!("{:.1}%", avg_progress)));
        let active_tasks = data
            .tasks
            .iter()
            .filter(|t| t.status != Status::Done)
            .count();
        task_metrics.push(("Active Tasks".to_string(), active_tasks.to_string()));
        performance.insert("Task Performance".to_string(), task_metrics);
        let mut memory_metrics = Vec::new();
        let critical_memories = data
            .memories
            .iter()
            .filter(|m| m.importance == MemoryImportance::Critical)
            .count();
        memory_metrics
            .push(("Critical Memories".to_string(), critical_memories.to_string()));
        let memory_diversity = data
            .memories
            .iter()
            .map(|m| format!("{:?}", m.memory_type))
            .collect::<HashSet<_>>()
            .len();
        memory_metrics
            .push(("Memory Type Diversity".to_string(), memory_diversity.to_string()));
        performance.insert("Memory Performance".to_string(), memory_metrics);
        performance
    }
    fn generate_predictive_insights(&self, data: &LearningData) -> Vec<String> {
        let mut predictions = Vec::new();
        let completion_trend = self.calculate_completion_trend(&data.tasks);
        predictions.push(format!("Task completion trend: {}", completion_trend));
        let learning_rate = self.calculate_learning_rate(&data.memories);
        predictions.push(format!("Learning acceleration: {}", learning_rate));
        let skill_gaps = self.identify_skill_gaps(&data.tasks, &data.memories);
        if !skill_gaps.is_empty() {
            predictions.push(format!("Potential skill gaps: {}", skill_gaps.join(", ")));
        }
        predictions
    }
    fn calculate_completion_trend(&self, tasks: &[Task]) -> String {
        let recent_tasks: Vec<_> = tasks
            .iter()
            .filter(|t| t.created_at > Utc::now() - Duration::days(7))
            .collect();
        let older_tasks: Vec<_> = tasks
            .iter()
            .filter(|t| t.created_at <= Utc::now() - Duration::days(7))
            .collect();
        let recent_completion_rate = if !recent_tasks.is_empty() {
            recent_tasks.iter().filter(|t| t.status == Status::Done).count() as f64
                / recent_tasks.len() as f64
        } else {
            0.0
        };
        let older_completion_rate = if !older_tasks.is_empty() {
            older_tasks.iter().filter(|t| t.status == Status::Done).count() as f64
                / older_tasks.len() as f64
        } else {
            0.0
        };
        match recent_completion_rate - older_completion_rate {
            d if d > 0.2 => "Strongly improving - excellent progress".to_string(),
            d if d > 0.1 => "Moderately improving - good momentum".to_string(),
            d if d > -0.1 => "Stable - consistent performance".to_string(),
            d if d > -0.2 => "Slightly declining - monitor closely".to_string(),
            _ => "Declining - intervention may be needed".to_string(),
        }
    }
    fn calculate_learning_rate(&self, memories: &[Memory]) -> String {
        let recent_memories = memories
            .iter()
            .filter(|m| m.created_at > Utc::now() - Duration::days(7))
            .count();
        let older_memories = memories
            .iter()
            .filter(|m| m.created_at <= Utc::now() - Duration::days(7))
            .count();
        if older_memories == 0 {
            return "New learning phase - establishing baseline".to_string();
        }
        let acceleration_ratio = recent_memories as f64 / (older_memories as f64 / 4.0);
        match acceleration_ratio {
            r if r > 1.5 => "Accelerating - rapid learning increase".to_string(),
            r if r > 1.2 => "Steady growth - good learning momentum".to_string(),
            r if r > 0.8 => "Stable - consistent learning pace".to_string(),
            r if r > 0.5 => "Slowing down - consider learning interventions".to_string(),
            _ => "Declining - may need motivation or method adjustments".to_string(),
        }
    }
    fn identify_skill_gaps(&self, tasks: &[Task], memories: &[Memory]) -> Vec<String> {
        let mut task_skills = HashSet::new();
        let mut memory_skills = HashSet::new();
        for task in tasks {
            for tag in &task.tags {
                task_skills.insert(tag.clone());
            }
        }
        for memory in memories {
            for tag in &memory.tags {
                memory_skills.insert(tag.clone());
            }
        }
        let gaps: Vec<_> = task_skills.difference(&memory_skills).cloned().collect();
        gaps
    }
    fn generate_learning_recommendations(&self, data: &LearningData) -> Vec<String> {
        let mut recommendations = Vec::new();
        let completion_rate = data
            .tasks
            .iter()
            .filter(|t| t.status == Status::Done)
            .count() as f64 / data.tasks.len().max(1) as f64;
        if completion_rate < 0.5 {
            recommendations
                .push(
                    "Improve task completion strategies - consider breaking down complex tasks"
                        .to_string(),
                );
        }
        if data.memories.len() < 5 {
            recommendations
                .push(
                    "Increase memory creation - document lessons learned from experiences"
                        .to_string(),
                );
        }
        if data.ideas.is_empty() {
            recommendations
                .push(
                    "Start capturing creative ideas - innovation is key to growth"
                        .to_string(),
                );
        }
        let unresolved_errors = data.errors.iter().filter(|e| !e.resolved).count();
        if unresolved_errors > data.errors.len() / 2 {
            recommendations
                .push(
                    "Focus on error resolution - many unresolved issues detected"
                        .to_string(),
                );
        }
        if recommendations.is_empty() {
            recommendations
                .push(
                    "Continue current learning practices - showing good progress"
                        .to_string(),
                );
        }
        recommendations
    }
}
#[derive(Debug, Clone)]
struct LearningData {
    tasks: Vec<Task>,
    memories: Vec<Memory>,
    ideas: Vec<Idea>,
    errors: Vec<Error>,
}
#[derive(Debug, Clone)]
struct LearningInsights {
    patterns: Vec<String>,
    predictions: Vec<String>,
    recommendations: Vec<String>,
    metrics: HashMap<String, f64>,
}
pub struct TaskExtractionTool;
impl TaskExtractionTool {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Tool for TaskExtractionTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "extract_tasks".to_string(),
            "Extract actionable tasks from natural language using AI-powered todozi.com API"
                .to_string(),
            vec![
                create_tool_parameter("message", "string",
                "Natural language message to extract tasks from", true),
                create_tool_parameter("context", "string",
                "Optional context about the project or situation", false),
            ],
            "AI Task Management".to_string(),
            vec![ResourceLock::Network],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let message = match kwargs.get("message") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'message' parameter".to_string(),
                    400,
                );
            }
        };
        if message.trim().is_empty() || message.len() > 10000 {
            return ToolResult::error(
                "Message must be 1-10000 characters".to_string(),
                400,
            );
        }
        let context = kwargs.get("context").and_then(|v| v.as_str()).unwrap_or("");
        let payload = serde_json::json!({ "message" : message, "context" : context });
        match make_todozi_request("/api/todozi/extract", payload).await {
            Ok(response) => {
                if let Some(extracted) = response.get("extracted_content") {
                    let mut results = Vec::new();
                    if let Some(tasks) = extracted
                        .get("tasks")
                        .and_then(|t| t.as_array())
                    {
                        results.push(format!("📋 Extracted {} tasks", tasks.len()));
                        for (i, task) in tasks.iter().enumerate() {
                            if let Some(action) = task
                                .get("action")
                                .and_then(|a| a.as_str())
                            {
                                results.push(format!("{}. {}", i + 1, action));
                            }
                        }
                    }
                    if let Some(memories) = extracted
                        .get("memories")
                        .and_then(|m| m.as_array())
                    {
                        results
                            .push(format!("🧠 Created {} memories", memories.len()));
                    }
                    if let Some(ideas) = extracted
                        .get("ideas")
                        .and_then(|i| i.as_array())
                    {
                        results.push(format!("💡 Generated {} ideas", ideas.len()));
                    }
                    ToolResult::success(results.join("\n"), 400)
                } else {
                    ToolResult::success(
                        "✅ Message processed successfully - no structured content extracted"
                            .to_string(),
                        400,
                    )
                }
            }
            Err(e) => ToolResult::error(format!("Failed to extract tasks: {}", e), 400),
        }
    }
}
pub struct TaskExpansionTool;
impl TaskExpansionTool {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Tool for TaskExpansionTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "expand_tasks".to_string(),
            "Expand high-level tasks into detailed, actionable subtasks using AI-powered todozi.com API"
                .to_string(),
            vec![
                create_tool_parameter("tasks", "string", "JSON array of tasks to expand",
                true), create_tool_parameter("context", "string",
                "Optional project context for better expansion", false),
            ],
            "AI Task Management".to_string(),
            vec![ResourceLock::Network],
        )
    }
    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let tasks_json = match kwargs.get("tasks") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'tasks' parameter".to_string(),
                    500,
                );
            }
        };
        let tasks: Vec<String> = match serde_json::from_str(tasks_json) {
            Ok(t) => t,
            Err(_) => {
                return ToolResult::error(
                    "Invalid JSON format for tasks".to_string(),
                    500,
                );
            }
        };
        let context = kwargs.get("context").and_then(|v| v.as_str()).unwrap_or("");
        let payload = serde_json::json!({ "tasks" : tasks, "context" : context });
        let expanded_tasks = match make_todozi_request("/api/todozi/expand", payload)
            .await
        {
            Ok(response) => {
                if let Some(expanded) = response.get("expanded_tasks") {
                    if let Some(tasks_array) = expanded.as_array() {
                        tasks_array
                            .iter()
                            .filter_map(|t| t.as_str().map(|s| s.to_string()))
                            .collect::<Vec<String>>()
                    } else {
                        return ToolResult::error(
                            "Invalid response format from API".to_string(),
                            500,
                        );
                    }
                } else {
                    return ToolResult::error(
                        "No expanded tasks in API response".to_string(),
                        500,
                    );
                }
            }
            Err(e) => {
                return ToolResult::error(format!("Failed to expand tasks: {}", e), 500);
            }
        };
        let mut results = Vec::new();
        results
            .push(
                format!("🚀 Expanded into {} detailed tasks:", expanded_tasks.len()),
            );
        for (i, task) in expanded_tasks.iter().enumerate() {
            results.push(format!("{}. {}", i + 1, task));
        }
        ToolResult::success(results.join("\n"), 500)
    }
}

/// Tool for AI-powered project planning using todozi.com API
pub struct PlanTool;

impl PlanTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for PlanTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "plan".to_string(),
            "Generate comprehensive AI-powered project plans using todozi.com strategic planning API".to_string(),
            vec![
                create_tool_parameter("content", "string", "Project description or goal to plan for", true),
                create_tool_parameter("extra", "string", "Additional context, constraints, or requirements", false),
                create_tool_parameter("output_format", "string", "Output format (json/text/markdown)", false),
            ],
            "AI Strategic Planning".to_string(),
            vec![ResourceLock::Network],
        )
    }

    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let content = match kwargs.get("content") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => return ToolResult::error("Missing or invalid 'content' parameter".to_string(), 600),
        };

        if content.trim().is_empty() || content.len() > 10000 {
            return ToolResult::error("Content must be 1-10000 characters".to_string(), 600);
        }

        let extra = kwargs.get("extra")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let output_format = kwargs.get("output_format")
            .and_then(|v| v.as_str())
            .unwrap_or("json");

        // Use the new extract_content function (which calls /api/tdz/plan)
        // Combine content and extra into a single content string
        let combined_content = if extra.is_empty() {
            content.to_string()
        } else {
            format!("{}\n\nAdditional Context: {}", content, extra)
        };
        match crate::extract_content(Some(combined_content), None, output_format.to_string(), false).await {
            Ok(result) => {
                ToolResult::success(format!("🎯 AI Project Planning Complete:\n{}", result), 600)
            }
            Err(e) => ToolResult::error(format!("Failed to execute AI planning: {}", e), 600),
        }
    }
}

/// Tool for AI-powered strategic planning using todozi.com API
pub struct StrategyTool;

impl StrategyTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for StrategyTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "strategy".to_string(),
            "Generate comprehensive AI-powered strategic plans using todozi.com strategic planning API".to_string(),
            vec![
                create_tool_parameter("content", "string", "Strategic goal or scenario to plan for", true),
                create_tool_parameter("extra", "string", "Additional context, market conditions, or constraints", false),
                create_tool_parameter("output_format", "string", "Output format (json/text/markdown)", false),
            ],
            "AI Strategic Planning".to_string(),
            vec![ResourceLock::Network],
        )
    }

    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let content = match kwargs.get("content") {
            Some(value) if value.is_string() => value.as_str().unwrap(),
            _ => return ToolResult::error("Missing or invalid 'content' parameter".to_string(), 700),
        };

        if content.trim().is_empty() || content.len() > 10000 {
            return ToolResult::error("Content must be 1-10000 characters".to_string(), 700);
        }

        let extra = kwargs.get("extra")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let output_format = kwargs.get("output_format")
            .and_then(|v| v.as_str())
            .unwrap_or("json");

        // Use the new strategy_content function (which calls /api/tdz/strategy)
        // Combine content and extra into a single content string
        let combined_content = if extra.is_empty() {
            content.to_string()
        } else {
            format!("{}\n\nAdditional Context: {}", content, extra)
        };
        match crate::strategy_content(Some(combined_content), None, output_format.to_string(), false).await {
            Ok(result) => {
                ToolResult::success(format!("🎭 AI Strategic Planning Complete:\n{}", result), 700)
            }
            Err(e) => ToolResult::error(format!("Failed to execute AI strategy: {}", e), 700),
        }
    }
}

pub fn create_grok_level_todozi_tools(
    todozi: SharedTodozi,
) -> Vec<Box<dyn Tool + Send + Sync>> {
    let mut tools: Vec<Box<dyn Tool + Send + Sync>> = vec![
        Box::new(CreateTaskTool::new(todozi.clone())),
        Box::new(SearchTasksTool::new(todozi.clone())),
        Box::new(UpdateTaskTool::new(todozi.clone())),
        Box::new(CreateMemoryTool::new(todozi.clone())),
        Box::new(CreateIdeaTool::new(todozi.clone())),
        Box::new(UnifiedSearchTool::new(todozi.clone())),
        Box::new(ProcessChatMessageTool::new(todozi.clone())),
        Box::new(CreateErrorTool::new(todozi.clone())),
        Box::new(CreateCodeChunkTool::new(todozi.clone())),
        Box::new(ChecklistTool::new(todozi.clone())),
        Box::new(IntelligentTaskPlannerTool::new(todozi.clone())),
        Box::new(MemorySynthesisTool::new(todozi.clone())),
        Box::new(IdeaRefinementTool::new(todozi.clone())),
    ];
    let mut error_prevention_tool = PredictiveErrorPreventionTool::new(todozi.clone());
    error_prevention_tool.initialize_patterns();
    tools.push(Box::new(error_prevention_tool));
    let mut agent_orchestrator = AIAgentOrchestratorTool::new(todozi.clone());
    agent_orchestrator.initialize_patterns();
    tools.push(Box::new(agent_orchestrator));
    let mut code_quality_tool = CodeQualityIntelligenceTool::new(todozi.clone());
    code_quality_tool.initialize_quality_patterns();
    tools.push(Box::new(code_quality_tool));
    tools.push(Box::new(LearningAnalyticsTool::new(todozi.clone())));
    tools.push(Box::new(TaskExtractionTool::new()));
    tools.push(Box::new(TaskExpansionTool::new()));
    tools.push(Box::new(PlanTool::new()));
    tools.push(Box::new(StrategyTool::new()));
    tools
}
pub async fn initialize_grok_level_todozi_system() -> std::result::Result<
    SharedTodozi,
    TodoziError,
> {
    crate::init().await?;
    let storage = Storage::new().await?;
    Ok(Arc::new(Mutex::new(storage)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_create_task_tool_definition() {
        let todozi = initialize_grok_level_todozi_system().await.unwrap();
        let tool = CreateTaskTool::new(todozi);
        let definition = tool.definition();
        assert_eq!(definition.name, "create_task");
        assert_eq!(definition.category, "Task Management");
        assert!(definition.parameters.len() > 0);
    }
    #[tokio::test]
    async fn test_unified_search_tool_definition() {
        let todozi = initialize_grok_level_todozi_system().await.unwrap();
        let tool = UnifiedSearchTool::new(todozi);
        let definition = tool.definition();
        assert_eq!(definition.name, "unified_search");
        assert_eq!(definition.category, "Search");
        assert!(definition.parameters.iter().any(| p | p.name == "query"));
    }
    #[tokio::test]
    async fn test_create_memory_tool_definition() {
        let todozi = initialize_grok_level_todozi_system().await.unwrap();
        let tool = CreateMemoryTool::new(todozi);
        let definition = tool.definition();
        assert_eq!(definition.name, "create_memory");
        assert_eq!(definition.category, "Memory Management");
        assert!(definition.parameters.iter().any(| p | p.name == "moment"));
    }
    #[test]
    fn test_extract_tasks_from_checklist() {
        let content = r#"
        Here are the tasks we need to complete:
        - [ ] Review the project requirements
        - [ ] Set up the development environment
        - [ ] Create initial project structure
        * [ ] Implement user authentication
        * [ ] Design database schema
        1. Write API documentation
        2. Test core functionality
        TODO: Deploy to staging environment
        "#;
        let tasks = ChecklistTool::extract_tasks(content);
        assert!(! tasks.is_empty());
        assert!(tasks.contains(& "Review the project requirements".to_string()));
        assert!(tasks.contains(& "Set up the development environment".to_string()));
        assert!(tasks.contains(& "Create initial project structure".to_string()));
    }
    #[test]
    fn test_extract_tasks_from_natural_language() {
        let content = r#"
        For this project, I will need to review the requirements and set up the development environment.
        We should also create the initial project structure and implement user authentication.
        The database schema design must be completed before we start coding.
        "#;
        let tasks = ChecklistTool::extract_tasks(content);
        assert!(! tasks.is_empty());
        assert!(
            tasks.contains(&
            "review the requirements and set up the development environment".to_string())
        );
        assert!(
            tasks.contains(&
            "create the initial project structure and implement user authentication"
            .to_string())
        );
    }
    #[test]
    fn test_extract_tasks_deduplication() {
        let content = r#"
        - [ ] Review requirements
        - [ ] Review requirements
        * Review requirements
        1. Review requirements
        "#;
        let tasks = ChecklistTool::extract_tasks(content);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], "Review requirements");
    }
    #[tokio::test]
    async fn test_checklist_tool_definition() {
        let todozi = initialize_grok_level_todozi_system().await.unwrap();
        let tool = ChecklistTool::new(todozi);
        let definition = tool.definition();
        assert_eq!(definition.name, "extract_tasks");
        assert_eq!(definition.category, "Task Management");
        assert!(definition.parameters.iter().any(| p | p.name == "content"));
        assert!(definition.parameters.iter().any(| p | p.name == "project"));
    }
}