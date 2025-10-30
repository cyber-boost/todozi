use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{Tdz, Storage, Find, Actions, Memories, Ideas, Queue, Stats, Easy, extract_content, strategy_content};
use crate::emb::TodoziEmbeddingService;

#[derive(Debug)]
pub enum ExecutorError {
    ExecutionError(String),
    BashToolError(String),
    MissingParameter(String),
    UnknownAction(String),
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            ExecutorError::BashToolError(msg) => write!(f, "Bash tool error: {}", msg),
            ExecutorError::MissingParameter(param) => write!(f, "Missing parameter: {}", param),
            ExecutorError::UnknownAction(action) => write!(f, "Unknown action: {}", action),
        }
    }
}

impl std::error::Error for ExecutorError {}

#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub tool_used: String,
    pub execution_type: String,
}

type ExecutorResult<T> = std::result::Result<T, ExecutorError>;
static TDZ_SYSTEM: std::sync::OnceLock<Arc<Mutex<Storage>>> = std::sync::OnceLock::new();
static TDZ_EMBEDDING_SERVICE: std::sync::OnceLock<Option<TodoziEmbeddingService>> = std::sync::OnceLock::new();
async fn ensure_todozi_system() -> ExecutorResult<()> {
    if TDZ_SYSTEM.get().is_none() {
        if let Err(e) = crate::init().await {
            eprintln!("Warning: Failed to initialize Todozi system: {}", e);
            return Err(
                ExecutorError::ExecutionError(
                    format!("Failed to initialize Todozi: {}", e),
                ),
            );
        }
        let storage = Storage::new().await
            .map_err(|e| ExecutorError::ExecutionError(format!("Failed to create storage: {}", e)))?;
        let embedding_service = None;
        TDZ_SYSTEM.set(Arc::new(Mutex::new(storage))).unwrap();
        TDZ_EMBEDDING_SERVICE.set(embedding_service).unwrap();
    }
    Ok(())
}
fn get_storage() -> ExecutorResult<Arc<Mutex<Storage>>> {
    TDZ_SYSTEM
        .get()
        .cloned()
        .ok_or_else(|| ExecutorError::ExecutionError(
            "Todozi system not initialized".to_string(),
        ))
}
fn get_embedding_service() -> Option<&'static TodoziEmbeddingService> {
    TDZ_EMBEDDING_SERVICE.get().and_then(|opt| opt.as_ref())
}
/// Get the API key using the proper tdz library function
async fn get_todozi_api_key() -> ExecutorResult<String> {
    use crate::get_tdz_api_key;
    get_tdz_api_key()
        .await
        .map_err(|e| ExecutorError::ExecutionError(
            format!("Failed to get API key: {}", e),
        ))
}
/// Helper function to make authenticated requests to todozi.com
async fn make_todozi_request(
    endpoint: &str,
    payload: serde_json::Value,
) -> ExecutorResult<serde_json::Value> {
    let api_key = get_todozi_api_key().await?;
    let client = reqwest::Client::new();
    let url = format!("https://todozi.com{}", endpoint);
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| ExecutorError::BashToolError(format!("Request failed: {}", e)))?;
    if !response.status().is_success() {
        return Err(
            ExecutorError::BashToolError(
                format!("API request failed with status: {}", response.status()),
            ),
        );
    }
    response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| ExecutorError::BashToolError(
            format!("Failed to parse response: {}", e),
        ))
}
/// Enhanced Todozi tool integration using hybrid approach (local + API)
pub async fn execute_todozi_tool_delegated(
    params: &serde_json::Map<String, Value>,
) -> ExecutorResult<ExecutionResult> {
    println!("🎯 Executing Todozi operation using enhanced simple interfaces");
    if let Err(e) = ensure_todozi_system().await {
        eprintln!("⚠️ Warning: Failed to initialize Todozi system: {}", e);
    }
    let action = params
        .get("action")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("action".to_string()))?;
    println!("🎯 Todozi action: {}", action);
    match action {
        "task" => execute_simple_task(params).await,
        "urgent" => execute_urgent_task(params).await,
        "high" => execute_high_task(params).await,
        "low" => execute_low_task(params).await,
        "ai" => execute_ai_task(params).await,
        "human" => execute_human_task(params).await,
        "collab" => execute_collab_task(params).await,
        "find" => execute_find(params).await,
        "ai_search" => execute_ai_search(params).await,
        "fast_search" => execute_fast_search(params).await,
        "smart_search" => execute_smart_search(params).await,
        "remember" => execute_remember(params).await,
        "important_memory" => execute_important_memory(params).await,
        "idea" => execute_idea(params).await,
        "breakthrough_idea" => execute_breakthrough_idea(params).await,
        "complete" => execute_complete(params).await,
        "start" => execute_start(params).await,
        "stats" => execute_stats(params).await,
        "queue" => execute_queue(params).await,
        "chat" => execute_chat(params).await,
        "extract" => execute_extract_api(params).await,
        "expand" => execute_expand_api(params).await,
        "plan" => execute_plan_api(params).await,
        "strategy" => execute_strategy_api(params).await,
        _ => {
            Err(
                ExecutorError::UnknownAction(
                    format!("Unsupported Todozi action: {}", action),
                ),
            )
        }
    }
}
/// Execute simple task creation using Easy interface
async fn execute_simple_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Easy::do_it(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("✅ Task created: {}", task_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("task")),
                        ("task_id".to_string(), json!(task_id)),
                        ("execution_type".to_string(), json!("simple_interface")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "easy_interface".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to create task: {}", e)))
        }
    }
}
/// Execute urgent task creation
async fn execute_urgent_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Tdz::urgent(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🚨 Urgent task created: {}", task_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("urgent")),
                        ("task_id".to_string(), json!(task_id)),
                        ("priority".to_string(), json!("urgent")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "priority_interface".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to create urgent task: {}", e),
                ),
            )
        }
    }
}
/// Execute high priority task creation
async fn execute_high_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Tdz::high(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🟠 High priority task created: {}", task_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("high")),
                        ("task_id".to_string(), json!(task_id)),
                        ("priority".to_string(), json!("high")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "priority_interface".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to create high priority task: {}", e),
                ),
            )
        }
    }
}
/// Execute low priority task creation
async fn execute_low_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Tdz::low(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🟢 Low priority task created: {}", task_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("low")),
                        ("task_id".to_string(), json!(task_id)),
                        ("priority".to_string(), json!("low")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "priority_interface".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to create low priority task: {}", e),
                ),
            )
        }
    }
}
/// Execute AI task creation (queued for AI systems)
async fn execute_ai_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Actions::ai(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!(
                    "🤖 AI task queued: {} (available for Maestro/Claude/etc.)",
                    task_id
                ),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("ai")),
                        ("task_id".to_string(), json!(task_id)),
                        ("assignee".to_string(), json!("ai")),
                        ("queued_for".to_string(), json!("external_ai_systems")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "ai_assignment".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(format!("Failed to create AI task: {}", e)),
            )
        }
    }
}
/// Execute human task creation (appears in TUI)
async fn execute_human_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Actions::human(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("👤 Human task created: {} (visible in TUI)", task_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("human")),
                        ("task_id".to_string(), json!(task_id)),
                        ("assignee".to_string(), json!("human")),
                        ("visible_in".to_string(), json!("tui_interface")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "human_assignment".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to create human task: {}", e),
                ),
            )
        }
    }
}
/// Execute collaborative task creation
async fn execute_collab_task(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Actions::collab(content).await {
        Ok(task_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!(
                    "🤝 Collaborative task created: {} (AI+Human coordination)",
                    task_id
                ),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("collab")),
                        ("task_id".to_string(), json!(task_id)),
                        ("assignee".to_string(), json!("collaborative")),
                        ("coordination".to_string(), json!("ai_human")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "collaborative_assignment".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to create collaborative task: {}", e),
                ),
            )
        }
    }
}
/// Execute smart find (AI + keyword combination)
async fn execute_find(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Find::tdz_find(content).await {
        Ok(results) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🔍 Smart search results:\n{}", results),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("find")),
                        ("query".to_string(), json!(content)),
                        ("search_type".to_string(), json!("ai_plus_keyword")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "smart_search".to_string(),
            })
        }
        Err(e) => Err(ExecutorError::ExecutionError(format!("Search failed: {}", e))),
    }
}
/// Execute AI-only semantic search
async fn execute_ai_search(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Find::deep(content).await {
        Ok(results) => {
            let formatted = results
                .iter()
                .map(|r| {
                    format!(
                        "• {} (similarity: {:.2})", r.text_content, r.similarity_score
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            Ok(ExecutionResult {
                success: true,
                output: format!("🤖 AI semantic search results:\n{}", formatted),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("ai_search")),
                        ("query".to_string(), json!(content)),
                        ("search_type".to_string(), json!("semantic_only")),
                        ("results_count".to_string(), json!(results.len())),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "semantic_search".to_string(),
            })
        }
        Err(e) => Err(ExecutorError::ExecutionError(format!("AI search failed: {}", e))),
    }
}
/// Execute fast keyword search
async fn execute_fast_search(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Find::fast(content).await {
        Ok(results) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("⚡ Fast search results:\n{}", results),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("fast_search")),
                        ("query".to_string(), json!(content)),
                        ("search_type".to_string(), json!("keyword_only")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "fast_search".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Fast search failed: {}", e)))
        }
    }
}
/// Execute smart intent-aware search
async fn execute_smart_search(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Find::smart(content).await {
        Ok(results) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🧠 Smart intent search results:\n{}", results),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("smart_search")),
                        ("query".to_string(), json!(content)),
                        ("search_type".to_string(), json!("intent_aware")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "intent_search".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Smart search failed: {}", e)))
        }
    }
}
/// Execute memory creation
async fn execute_remember(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    let extra = params.get("extra").and_then(|v| v.as_str()).unwrap_or(content);
    match Memories::create(content, extra, "Created via simple interface").await {
        Ok(memory_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🧠 Memory saved: {}", memory_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("remember")),
                        ("memory_id".to_string(), json!(memory_id)),
                        ("importance".to_string(), json!("medium")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "memory_creation".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to save memory: {}", e)))
        }
    }
}
/// Execute important memory creation
async fn execute_important_memory(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    let extra = params.get("extra").and_then(|v| v.as_str()).unwrap_or(content);
    match Memories::important(content, extra, "Important via simple interface").await {
        Ok(memory_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🧠⭐ Important memory saved: {}", memory_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("important_memory")),
                        ("memory_id".to_string(), json!(memory_id)),
                        ("importance".to_string(), json!("high")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "important_memory".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to save important memory: {}", e),
                ),
            )
        }
    }
}
/// Execute idea creation
async fn execute_idea(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Ideas::create(content).await {
        Ok(idea_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("💡 Idea saved: {}", idea_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("idea")),
                        ("idea_id".to_string(), json!(idea_id)),
                        ("importance".to_string(), json!("medium")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "idea_creation".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to save idea: {}", e)))
        }
    }
}
/// Execute breakthrough idea creation
async fn execute_breakthrough_idea(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Ideas::breakthrough(content).await {
        Ok(idea_id) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("💡🚀 Breakthrough idea saved: {}", idea_id),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("breakthrough_idea")),
                        ("idea_id".to_string(), json!(idea_id)),
                        ("importance".to_string(), json!("breakthrough")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "breakthrough_idea".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to save breakthrough idea: {}", e),
                ),
            )
        }
    }
}
/// Execute task completion
async fn execute_complete(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Actions::complete(content).await {
        Ok(_) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("✅ Task {} marked as completed", content),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("complete")),
                        ("task_id".to_string(), json!(content)),
                        ("status".to_string(), json!("completed")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "task_completion".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to complete task: {}", e)))
        }
    }
}
/// Execute task start
async fn execute_start(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Actions::begin(content).await {
        Ok(_) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🔄 Task {} started", content),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("start")),
                        ("task_id".to_string(), json!(content)),
                        ("status".to_string(), json!("in_progress")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "task_start".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to start task: {}", e)))
        }
    }
}
/// Execute stats retrieval
async fn execute_stats(
    _params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    match Stats::quick().await {
        Ok(stats) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("📊 Todozi Stats:\n{}", stats),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("stats")),
                        ("stats_type".to_string(), json!("quick_overview")),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "stats_retrieval".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to get stats: {}", e)))
        }
    }
}
/// Execute queue status
async fn execute_queue(
    _params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    match Queue::list().await {
        Ok(items) => {
            let backlog = Queue::backlog().await.unwrap_or_default();
            let active = Queue::active().await.unwrap_or_default();
            Ok(ExecutionResult {
                success: true,
                output: format!(
                    "📋 Queue Status:\n  Total: {} items\n  Backlog: {} items\n  Active: {} items",
                    items.len(), backlog.len(), active.len()
                ),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("queue")),
                        ("total_items".to_string(), json!(items.len())),
                        ("backlog_items".to_string(), json!(backlog.len())),
                        ("active_items".to_string(), json!(active.len())),
                    ]),
                tool_used: "todozi_simple".to_string(),
                execution_type: "queue_status".to_string(),
            })
        }
        Err(e) => {
            Err(
                ExecutorError::ExecutionError(
                    format!("Failed to get queue status: {}", e),
                ),
            )
        }
    }
}
/// Execute chat processing
async fn execute_chat(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    match Tdz::chat(content).await {
        Ok(chat_content) => {
            let mut results = Vec::new();
            let mut metadata = HashMap::new();
            if !chat_content.tasks.is_empty() {
                results.push(format!("📋 Created {} tasks", chat_content.tasks.len()));
                metadata
                    .insert(
                        "tasks_created".to_string(),
                        json!(chat_content.tasks.len()),
                    );
            }
            if !chat_content.memories.is_empty() {
                results
                    .push(
                        format!("🧠 Created {} memories", chat_content.memories.len()),
                    );
                metadata
                    .insert(
                        "memories_created".to_string(),
                        json!(chat_content.memories.len()),
                    );
            }
            if !chat_content.ideas.is_empty() {
                results.push(format!("💡 Created {} ideas", chat_content.ideas.len()));
                metadata
                    .insert(
                        "ideas_created".to_string(),
                        json!(chat_content.ideas.len()),
                    );
            }
            let summary = if results.is_empty() {
                "✅ Chat processed - no structured content extracted".to_string()
            } else {
                format!("✅ Chat processed: {}", results.join(", "))
            };
            metadata.insert("action".to_string(), json!("chat"));
            metadata.insert("total_items".to_string(), json!(results.len()));
            Ok(ExecutionResult {
                success: true,
                output: summary,
                error: None,
                metadata: metadata,
                tool_used: "todozi_simple".to_string(),
                execution_type: "chat_processing".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to process chat: {}", e)))
        }
    }
}
/// Execute task extraction using todozi.com API
async fn execute_extract_api(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    let extra = params.get("extra").and_then(|v| v.as_str()).unwrap_or("");
    
    // Use direct API call like the shop version
    let payload = serde_json::json!({ "message": content, "context": extra });
    match make_todozi_request("/api/todozi/extract", payload).await {
        Ok(response) => {
            if let Some(extracted) = response.get("extracted_content") {
                let mut results = Vec::new();
                if let Some(tasks) = extracted.get("tasks").and_then(|t| t.as_array()) {
                    results.push(format!("📋 Extracted {} tasks", tasks.len()));
                    for (i, task) in tasks.iter().enumerate() {
                        if let Some(action) = task.get("action").and_then(|a| a.as_str()) {
                            results.push(format!("{}. {}", i + 1, action));
                        }
                    }
                }
                if let Some(memories) = extracted.get("memories").and_then(|m| m.as_array()) {
                    results.push(format!("🧠 Created {} memories", memories.len()));
                }
                if let Some(ideas) = extracted.get("ideas").and_then(|i| i.as_array()) {
                    results.push(format!("💡 Generated {} ideas", ideas.len()));
                }
                Ok(ExecutionResult {
                    success: true,
                    output: if results.is_empty() {
                        "✅ Message processed successfully - no structured content extracted".to_string()
                    } else {
                        results.join("\n")
                    },
                    error: None,
                    metadata: HashMap::from([
                            ("action".to_string(), json!("extract")),
                            ("execution_type".to_string(), json!("todozi_com_api")),
                    ]),
                    tool_used: "todozi_api".to_string(),
                    execution_type: "ai_extraction".to_string(),
                })
            } else {
                Ok(ExecutionResult {
                    success: true,
                    output: "✅ Message processed successfully - no structured content extracted".to_string(),
                    error: None,
                    metadata: HashMap::from([("action".to_string(), json!("extract"))]),
                    tool_used: "todozi_api".to_string(),
                    execution_type: "ai_extraction".to_string(),
                })
            }
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to extract tasks: {}", e)))
        }
    }
}
/// Execute task expansion using todozi.com API
async fn execute_expand_api(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;
    let extra = params.get("extra").and_then(|v| v.as_str()).unwrap_or("");
    
    // Convert single task to array format for API
    let tasks_array = vec![content.to_string()];
    let payload = serde_json::json!({ "tasks": tasks_array, "context": extra });
    
    match make_todozi_request("/api/todozi/expand", payload).await {
        Ok(response) => {
            if let Some(expanded) = response.get("expanded_tasks") {
                if let Some(tasks_array) = expanded.as_array() {
                    let expanded_tasks: Vec<String> = tasks_array
                        .iter()
                        .filter_map(|t| t.as_str().map(|s| s.to_string()))
                        .collect();
                    
                    let task_list = expanded_tasks
                        .iter()
                        .enumerate()
                        .map(|(i, task)| format!("{}. {}", i + 1, task))
                        .collect::<Vec<_>>()
                        .join("\n");
                    
                    Ok(ExecutionResult {
                        success: true,
                        output: if expanded_tasks.is_empty() {
                            "🤖 No task expansion generated".to_string()
                        } else {
                            format!(
                                "🚀 Expanded into {} detailed tasks:\n{}",
                                expanded_tasks.len(), task_list
                            )
                        },
                        error: None,
                        metadata: HashMap::from([
                            ("action".to_string(), json!("expand")),
                            ("execution_type".to_string(), json!("todozi_com_api")),
                            ("tasks_created".to_string(), json!(expanded_tasks.len())),
                        ]),
                        tool_used: "todozi_api".to_string(),
                        execution_type: "ai_expansion".to_string(),
                    })
                } else {
                    Err(ExecutorError::ExecutionError("Invalid response format from API".to_string()))
                }
            } else {
                Err(ExecutorError::ExecutionError("No expanded tasks in API response".to_string()))
            }
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to expand tasks: {}", e)))
        }
    }
}
/// Execute AI project planning using todozi plan command
async fn execute_plan_api(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;

    let output_format = params
        .get("output_format")
        .and_then(|v| v.as_str())
        .unwrap_or("json");

    let extra = params.get("extra").and_then(|v| v.as_str());

    // Use the new extract_content function (which calls /api/tdz/plan)
    match extract_content(Some(content.to_string()), extra.map(|s| s.to_string()), output_format.to_string(), false).await {
        Ok(result) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🎯 AI Project Planning Complete:\n{}", result),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("plan")),
                        ("execution_type".to_string(), json!("todozi_plan_api")),
                        ("output_format".to_string(), json!(output_format)),
                        ("endpoint".to_string(), json!("/api/tdz/plan")),
                    ]),
                tool_used: "todozi_plan".to_string(),
                execution_type: "ai_project_planning".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to execute AI planning: {}", e)))
        }
    }
}
/// Execute AI strategic planning using todozi strategy command
async fn execute_strategy_api(
    params: &serde_json::Map<String, Value>,
) -> Result<ExecutionResult, ExecutorError> {
    let content = params
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ExecutorError::MissingParameter("content".to_string()))?;

    let output_format = params
        .get("output_format")
        .and_then(|v| v.as_str())
        .unwrap_or("json");

    let extra = params.get("extra").and_then(|v| v.as_str());

    // Use the new strategy_content function (which calls /api/tdz/strategy)
    match strategy_content(Some(content.to_string()), extra.map(|s| s.to_string()), output_format.to_string(), false).await {
        Ok(result) => {
            Ok(ExecutionResult {
                success: true,
                output: format!("🎭 AI Strategic Planning Complete:\n{}", result),
                error: None,
                metadata: HashMap::from([
                        ("action".to_string(), json!("strategy")),
                        ("execution_type".to_string(), json!("todozi_strategy_api")),
                        ("output_format".to_string(), json!(output_format)),
                        ("endpoint".to_string(), json!("/api/tdz/strategy")),
                    ]),
                tool_used: "todozi_strategy".to_string(),
                execution_type: "ai_strategic_planning".to_string(),
            })
        }
        Err(e) => {
            Err(ExecutorError::ExecutionError(format!("Failed to execute AI strategy: {}", e)))
        }
    }
}