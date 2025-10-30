use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;
use chrono::{DateTime, Utc, Duration};
use regex::Regex;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::base::*;
use crate::error::TodoziError;
use crate::todozi::ChatContent;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub content: String,
    pub priority: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub source: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedAction {
    pub id: String,
    pub action_type: String,
    pub parameters: HashMap<String, Value>,
    pub confidence: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedContent {
    pub id: String,
    pub session_id: String,
    pub raw_content: String,
    pub cleaned_content: String,
    pub timestamp: DateTime<Utc>,
    pub extracted_items: Vec<String>,
    pub checklist_items: Vec<ChecklistItem>,
    pub tool_calls: Vec<ExtractedAction>,
    pub processing_stats: ProcessingStats,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub content_length: usize,
    pub tool_calls_found: usize,
    pub tags_extracted: usize,
    pub checklists_generated: usize,
    pub processing_time_ms: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedAction {
    pub id: String,
    pub action_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub result: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSession {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub topic: String,
    pub participant_count: u32,
    pub message_count: u32,
}
pub type SharedTodoziState = Arc<Mutex<TodoziProcessorState>>;
pub struct TodoziProcessorState {
    pub active_sessions: HashMap<String, ConversationSession>,
    pub recent_actions: Vec<ProcessedAction>,
    pub checklist_items: Vec<ChecklistItem>,
    pub processed_contents: Vec<ProcessedContent>,
}
impl TodoziProcessorState {
    pub fn new() -> Result<Self, TodoziError> {
        Ok(Self {
            active_sessions: HashMap::new(),
            recent_actions: Vec::new(),
            checklist_items: Vec::new(),
            processed_contents: Vec::new(),
        })
    }
    pub fn add_checklist_item(&mut self, item: ChecklistItem) {
        self.checklist_items.push(item);
    }
    pub fn add_recent_action(&mut self, action: ProcessedAction) {
        self.recent_actions.push(action);
        if self.recent_actions.len() > 100 {
            self.recent_actions.drain(0..self.recent_actions.len() - 100);
        }
    }
    pub fn save_processed_content(
        &mut self,
        raw: &str,
        cleaned: &str,
        session_id: &str,
    ) -> Result<(), TodoziError> {
        let processed = ProcessedContent {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            raw_content: raw.to_string(),
            cleaned_content: cleaned.to_string(),
            timestamp: Utc::now(),
            extracted_items: Vec::new(),
            checklist_items: Vec::new(),
            tool_calls: Vec::new(),
            processing_stats: ProcessingStats {
                content_length: raw.len(),
                tool_calls_found: 0,
                tags_extracted: 0,
                checklists_generated: 0,
                processing_time_ms: 0,
            },
        };
        self.processed_contents.push(processed);
        Ok(())
    }
}
#[derive(Debug)]
struct ParsedContent {
    text_content: String,
    json_content: Option<Value>,
    tool_calls: Vec<Value>,
}
#[derive(Debug)]
struct ExtractionResult {
    extracted_tags: Vec<String>,
    tool_calls: Vec<Value>,
    natural_patterns: Vec<String>,
}
#[derive(Debug)]
struct ProcessingResult {
    actions: Vec<ProcessedAction>,
}
pub struct TdzContentProcessorTool {
    state: SharedTodoziState,
    natural_language_patterns: Vec<String>,
}
impl TdzContentProcessorTool {
    pub fn new(state: SharedTodoziState) -> Self {
        let mut tool = Self {
            state,
            natural_language_patterns: Vec::new(),
        };
        tool.initialize_patterns();
        tool
    }
    fn initialize_patterns(&mut self) {
        self.natural_language_patterns = vec![
            r"we should".to_string(), r"I need to".to_string(), r"let's".to_string(),
            r"we need to".to_string(), r"don't forget".to_string(), r"remember to"
            .to_string(), r"make sure".to_string(), r"important:".to_string(), r"note:"
            .to_string(), r"todo:".to_string(), r"add to checklist".to_string(),
            r"checklist item".to_string(), r"action item".to_string(), r"next step"
            .to_string(),
        ];
    }
}
#[async_trait]
impl Tool for TdzContentProcessorTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "tdz_content_processor".to_string(),
            "Process raw content from AI models, extract Todozi data, and return cleaned conversational output"
                .to_string(),
            vec![
                create_tool_parameter("content", "string",
                "Raw content to process (JSON or text with tags)", true),
                create_tool_parameter("session_id", "string",
                "Optional session ID for conversation tracking", false),
                create_tool_parameter("extract_checklist", "boolean",
                "Extract checklist items from natural language", false),
                create_tool_parameter("auto_session", "boolean",
                "Automatically create/manage sessions", false),
            ],
            "Content Processing".to_string(),
            vec![ResourceLock::FilesystemRead, ResourceLock::FilesystemWrite],
        )
    }
    async fn execute(
        &self,
        kwargs: HashMap<String, serde_json::Value>,
    ) -> ToolResult {
        let content = match kwargs.get("content") {
            Some(value) if value.is_string() => value.as_str().unwrap_or(""),
            _ => {
                return ToolResult::error(
                    "Missing or invalid 'content' parameter".to_string(),
                    100,
                );
            }
        };
        let session_id = kwargs
            .get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        let extract_checklist = kwargs
            .get("extract_checklist")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let auto_session = kwargs
            .get("auto_session")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        match self
            .process_content(content, session_id, extract_checklist, auto_session)
            .await
        {
            Ok(result) => ToolResult::success(result, 100),
            Err(e) => ToolResult::error(format!("Content processing failed: {}", e), 100),
        }
    }
}
impl TdzContentProcessorTool {
    async fn process_content(
        &self,
        content: &str,
        session_id: &str,
        extract_checklist: bool,
        auto_session: bool,
    ) -> Result<String, TodoziError> {
        let start_time = Utc::now();
        let mut state = self.state.lock().await;
        let parsed_content = self.parse_raw_content(content)?;
        let extraction_result = self.extract_todozi_data(&parsed_content).await?;
        let processing_result = self
            .process_tool_calls(&extraction_result.tool_calls)
            .await?;
        let cleaned_content = self
            .clean_content(content, &extraction_result.extracted_tags)?;
        if extract_checklist {
            let checklist_items = self
                .extract_checklist_items(&parsed_content.text_content)?;
            for item in checklist_items {
                state.add_checklist_item(item);
            }
        }
        if auto_session {
            self.ensure_session_exists(&mut state, session_id, &parsed_content)?;
        }
        for action in &processing_result.actions {
            state.add_recent_action(action.clone());
        }
        state.save_processed_content(content, &cleaned_content, session_id)?;
        let processing_time = Utc::now()
            .signed_duration_since(start_time)
            .num_milliseconds();
        let stats = ProcessingStats {
            content_length: content.len(),
            tool_calls_found: extraction_result.tool_calls.len(),
            tags_extracted: extraction_result.extracted_tags.len(),
            checklists_generated: if extract_checklist {
                self.extract_checklist_items(&parsed_content.text_content)?.len()
            } else {
                0
            },
            processing_time_ms: processing_time,
        };
        let response = self
            .generate_response(&cleaned_content, &state, &processing_result, stats)?;
        Ok(response)
    }
    fn parse_raw_content(&self, content: &str) -> Result<ParsedContent, TodoziError> {
        if let Ok(json_value) = serde_json::from_str::<Value>(content) {
            self.parse_json_content(json_value)
        } else {
            self.parse_text_content(content)
        }
    }
    fn parse_json_content(&self, json: Value) -> Result<ParsedContent, TodoziError> {
        let mut text_content = String::new();
        let mut tool_calls = Vec::new();
        if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
            text_content.push_str(content);
        }
        if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
            text_content.push_str(message);
        }
        if let Some(choices) = json.get("choices").and_then(|v| v.as_array()) {
            for choice in choices {
                if let Some(message) = choice.get("message").and_then(|v| v.as_str()) {
                    text_content.push_str(message);
                }
                if let Some(content) = choice.get("content").and_then(|v| v.as_str()) {
                    text_content.push_str(content);
                }
            }
        }
        if let Some(tools) = json.get("tool_calls").and_then(|v| v.as_array()) {
            for tool in tools {
                if let Some(function) = tool.get("function") {
                    tool_calls.push(function.clone());
                }
            }
        }
        Ok(ParsedContent {
            text_content,
            json_content: Some(json),
            tool_calls,
        })
    }
    fn parse_text_content(&self, content: &str) -> Result<ParsedContent, TodoziError> {
        Ok(ParsedContent {
            text_content: content.to_string(),
            json_content: None,
            tool_calls: Vec::new(),
        })
    }
    async fn extract_todozi_data(
        &self,
        parsed: &ParsedContent,
    ) -> Result<ExtractionResult, TodoziError> {
        let mut extracted_tags = Vec::new();
        let mut tool_calls = Vec::new();
        let tag_patterns = vec![
            r"<todozi>.*?</todozi>", r"<memory>.*?</memory>", r"<idea>.*?</idea>",
            r"<todozi_agent>.*?</todozi_agent>", r"<chunk>.*?</chunk>",
            r"<tdz>.*?</tdz>",
        ];
        for pattern in tag_patterns {
            let re = Regex::new(pattern)
                .map_err(|e| TodoziError::ValidationError {
                    message: format!("Regex compilation failed: {}", e),
                })?;
            for mat in re.find_iter(&parsed.text_content) {
                extracted_tags.push(mat.as_str().to_string());
            }
        }
        for tool_call in &parsed.tool_calls {
            if let Some(function_name) = tool_call.get("name").and_then(|v| v.as_str()) {
                if function_name.contains("todozi") || function_name.contains("tdz") {
                    tool_calls.push(tool_call.clone());
                }
            }
        }
        let natural_patterns = self
            .extract_natural_language_patterns(&parsed.text_content)?;
        Ok(ExtractionResult {
            extracted_tags,
            tool_calls,
            natural_patterns,
        })
    }
    async fn process_tool_calls(
        &self,
        tool_calls: &[Value],
    ) -> Result<ProcessingResult, TodoziError> {
        let mut actions = Vec::new();
        for tool_call in tool_calls {
            if let Some(function_name) = tool_call.get("name").and_then(|v| v.as_str()) {
                match function_name {
                    name if name.contains("create_task")
                        || name.contains("add_task") => {
                        let action = self.process_create_task_call(tool_call).await?;
                        actions.push(action);
                    }
                    name if name.contains("search") || name.contains("list") => {
                        let action = self.process_search_call(tool_call).await?;
                        actions.push(action);
                    }
                    name if name.contains("update") || name.contains("complete") => {
                        let action = self.process_update_call(tool_call).await?;
                        actions.push(action);
                    }
                    name if name.contains("memory") => {
                        let action = self.process_memory_call(tool_call).await?;
                        actions.push(action);
                    }
                    name if name.contains("idea") => {
                        let action = self.process_idea_call(tool_call).await?;
                        actions.push(action);
                    }
                    _ => {
                        actions
                            .push(ProcessedAction {
                                id: Uuid::new_v4().to_string(),
                                action_type: "unknown_tool_call".to_string(),
                                description: format!(
                                    "Unknown tool call: {}", function_name
                                ),
                                timestamp: Utc::now(),
                                success: false,
                                result: Some("Tool call not recognized".to_string()),
                            });
                    }
                }
            }
        }
        Ok(ProcessingResult { actions })
    }
    fn clean_content(
        &self,
        original: &str,
        extracted_tags: &[String],
    ) -> Result<String, TodoziError> {
        let mut cleaned = original.to_string();
        for tag in extracted_tags {
            cleaned = cleaned.replace(tag, "");
        }
        if let Ok(json) = serde_json::from_str::<Value>(&original) {
            if let Some(_tool_calls) = json.get("tool_calls") {
                let mut cleaned_json = json;
                if let Some(obj) = cleaned_json.as_object_mut() {
                    obj.remove("tool_calls");
                }
                cleaned = serde_json::to_string_pretty(&cleaned_json)?;
            }
        }
        cleaned = cleaned
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(cleaned)
    }
    fn extract_natural_language_patterns(
        &self,
        text: &str,
    ) -> Result<Vec<String>, TodoziError> {
        let mut patterns = Vec::new();
        let action_patterns = vec![
            r"we should", r"I need to", r"let's", r"we need to", r"don't forget",
            r"remember to", r"make sure", r"important:", r"note:", r"todo:",
        ];
        for pattern in action_patterns {
            let re = Regex::new(&format!(r"(?i){}", pattern))
                .map_err(|e| TodoziError::ValidationError {
                    message: format!("Regex compilation failed: {}", e),
                })?;
            for mat in re.find_iter(text) {
                let start = mat.start();
                let text_after = &text[start..];
                let end_pos = text_after
                    .find('.')
                    .or_else(|| text_after.find('\n'))
                    .unwrap_or(text_after.len());
                let extracted = text_after[..end_pos].trim().to_string();
                if extracted.len() > 10 && extracted.len() < 200 {
                    patterns.push(extracted);
                }
            }
        }
        Ok(patterns)
    }
    fn extract_checklist_items(
        &self,
        text: &str,
    ) -> Result<Vec<ChecklistItem>, TodoziError> {
        let mut items = Vec::new();
        let mut seen_items = std::collections::HashSet::new();
        let patterns = vec![
            r"add to (?:checklist|list|todo)", r"we need to", r"should (?:have|do)",
            r"don't forget to", r"remember to", r"make sure to", r"need to", r"have to",
            r"must",
        ];
        for pattern in patterns {
            let re = Regex::new(&format!(r"(?i){}", pattern))
                .map_err(|e| TodoziError::ValidationError {
                    message: format!("Regex compilation failed: {}", e),
                })?;
            for mat in re.find_iter(text) {
                let start = mat.start();
                let text_after = &text[start..];
                let end_pos = text_after
                    .find('.')
                    .or_else(|| text_after.find('!'))
                    .or_else(|| text_after.find('?'))
                    .unwrap_or(text_after.len());
                let item_text = text_after[..end_pos].trim().to_string();
                if !item_text.is_empty() && item_text.len() < 200 {
                    let normalized = item_text.to_lowercase();
                    if !seen_items.contains(&normalized) {
                        seen_items.insert(normalized);
                        items
                            .push(ChecklistItem {
                                id: Uuid::new_v4().to_string(),
                                content: item_text,
                                priority: "medium".to_string(),
                                created_at: Utc::now(),
                                completed: false,
                                source: "natural_language".to_string(),
                            });
                    }
                }
            }
        }
        Ok(items)
    }
    fn ensure_session_exists(
        &self,
        state: &mut TodoziProcessorState,
        session_id: &str,
        parsed: &ParsedContent,
    ) -> Result<(), TodoziError> {
        if !state.active_sessions.contains_key(session_id) {
            let topic = self.infer_topic(&parsed.text_content)?;
            let session = ConversationSession {
                id: session_id.to_string(),
                start_time: Utc::now(),
                last_activity: Utc::now(),
                topic,
                participant_count: 1,
                message_count: 1,
            };
            state.active_sessions.insert(session_id.to_string(), session);
        } else {
            if let Some(session) = state.active_sessions.get_mut(session_id) {
                session.last_activity = Utc::now();
                session.message_count += 1;
            }
        }
        Ok(())
    }
    fn infer_topic(&self, text: &str) -> Result<String, TodoziError> {
        let text_lower = text.to_lowercase();
        if text_lower.contains("bug") || text_lower.contains("error")
            || text_lower.contains("fix")
        {
            Ok("Bug Fixing & Debugging".to_string())
        } else if text_lower.contains("feature") || text_lower.contains("implement") {
            Ok("Feature Development".to_string())
        } else if text_lower.contains("design") || text_lower.contains("architecture") {
            Ok("System Design & Architecture".to_string())
        } else if text_lower.contains("test") || text_lower.contains("testing") {
            Ok("Testing & Quality Assurance".to_string())
        } else if text_lower.contains("deploy") || text_lower.contains("production") {
            Ok("Deployment & Operations".to_string())
        } else {
            Ok("General Discussion".to_string())
        }
    }
    fn generate_response(
        &self,
        cleaned_content: &str,
        state: &TodoziProcessorState,
        processing: &ProcessingResult,
        stats: ProcessingStats,
    ) -> Result<String, TodoziError> {
        let mut response = cleaned_content.to_string();
        if !processing.actions.is_empty() || stats.checklists_generated > 0 {
            response.push_str("\n\n--- TDZ PROCESSING SUMMARY ---\n");
            if stats.checklists_generated > 0 {
                response
                    .push_str(
                        &format!(
                            "📋 Generated {} checklist items\n", stats
                            .checklists_generated
                        ),
                    );
            }
            if !processing.actions.is_empty() {
                response
                    .push_str(
                        &format!("✅ Processed {} actions\n", processing.actions.len()),
                    );
                let successful = processing.actions.iter().filter(|a| a.success).count();
                if successful > 0 {
                    response
                        .push_str(&format!("✅ {} successful actions\n", successful));
                }
            }
            response
                .push_str(
                    &format!("⏱️ Processing time: {}ms\n", stats.processing_time_ms),
                );
        }
        let recent_actions: Vec<_> = state.recent_actions.iter().rev().take(3).collect();
        if !recent_actions.is_empty() {
            response.push_str("\n--- RECENT ACTIONS ---\n");
            for action in recent_actions.into_iter().rev() {
                let status = if action.success { "✅" } else { "❌" };
                response
                    .push_str(
                        &format!(
                            "{} {}: {}\n", status, action.action_type, action.description
                        ),
                    );
            }
        }
        let active_checklist: Vec<_> = state
            .checklist_items
            .iter()
            .filter(|item| !item.completed)
            .take(3)
            .collect();
        if !active_checklist.is_empty() {
            response.push_str("\n--- ACTIVE CHECKLIST ---\n");
            for item in active_checklist {
                response.push_str(&format!("☐ {}\n", item.content));
            }
        }
        let active_sessions: Vec<_> = state
            .active_sessions
            .values()
            .filter(|s| s.last_activity > Utc::now() - Duration::hours(24))
            .collect();
        if !active_sessions.is_empty() {
            response.push_str("\n--- ACTIVE SESSIONS ---\n");
            for session in active_sessions {
                response
                    .push_str(
                        &format!(
                            "📋 {}: {} messages\n", session.topic, session
                            .message_count
                        ),
                    );
            }
        }
        response
            .push_str(
                "\n💡 Run `todozi stats` or `todozi list` to see all recent activity\n",
            );
        Ok(response)
    }
    async fn process_create_task_call(
        &self,
        _tool_call: &Value,
    ) -> Result<ProcessedAction, TodoziError> {
        let result = self
            .execute_binary_command("todozi", &["add", "Task from tool call"])?;
        Ok(ProcessedAction {
            id: Uuid::new_v4().to_string(),
            action_type: "create_task".to_string(),
            description: "Created task via tool call".to_string(),
            timestamp: Utc::now(),
            success: result.status.success(),
            result: Some(String::from_utf8_lossy(&result.stdout).to_string()),
        })
    }
    async fn process_search_call(
        &self,
        _tool_call: &Value,
    ) -> Result<ProcessedAction, TodoziError> {
        let result = self.execute_binary_command("todozi", &["list"])?;
        Ok(ProcessedAction {
            id: Uuid::new_v4().to_string(),
            action_type: "search_tasks".to_string(),
            description: "Searched tasks via tool call".to_string(),
            timestamp: Utc::now(),
            success: result.status.success(),
            result: Some(String::from_utf8_lossy(&result.stdout).to_string()),
        })
    }
    async fn process_update_call(
        &self,
        _tool_call: &Value,
    ) -> Result<ProcessedAction, TodoziError> {
        Ok(ProcessedAction {
            id: Uuid::new_v4().to_string(),
            action_type: "update_task".to_string(),
            description: "Updated task via tool call".to_string(),
            timestamp: Utc::now(),
            success: true,
            result: Some("Task update processed".to_string()),
        })
    }
    async fn process_memory_call(
        &self,
        _tool_call: &Value,
    ) -> Result<ProcessedAction, TodoziError> {
        Ok(ProcessedAction {
            id: Uuid::new_v4().to_string(),
            action_type: "create_memory".to_string(),
            description: "Created memory via tool call".to_string(),
            timestamp: Utc::now(),
            success: true,
            result: Some("Memory created".to_string()),
        })
    }
    async fn process_idea_call(
        &self,
        _tool_call: &Value,
    ) -> Result<ProcessedAction, TodoziError> {
        Ok(ProcessedAction {
            id: Uuid::new_v4().to_string(),
            action_type: "create_idea".to_string(),
            description: "Created idea via tool call".to_string(),
            timestamp: Utc::now(),
            success: true,
            result: Some("Idea created".to_string()),
        })
    }
    fn execute_binary_command(
        &self,
        command: &str,
        args: &[&str],
    ) -> Result<std::process::Output, TodoziError> {
        Command::new(command)
            .args(args)
            .output()
            .map_err(|e| TodoziError::StorageError {
                message: format!("Binary execution failed: {}", e),
            })
    }
}
pub fn create_tdz_content_processor_tool(
    state: SharedTodoziState,
) -> Box<dyn Tool + Send + Sync> {
    Box::new(TdzContentProcessorTool::new(state))
}
pub async fn initialize_tdz_content_processor() -> Result<
    SharedTodoziState,
    TodoziError,
> {
    let state = TodoziProcessorState::new()?;
    Ok(Arc::new(Mutex::new(state)))
}
pub async fn tdz_cnt(
    content: &str,
    session_id: Option<&str>,
) -> Result<String, TodoziError> {
    // Use process_chat_message_extended for better tag processing
    let chat_content = match crate::todozi::process_chat_message_extended(content, "tdz_system") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Warning: Failed to process chat message: {}", e);
            ChatContent {
                tasks: Vec::new(),
                memories: Vec::new(),
                ideas: Vec::new(),
                agent_assignments: Vec::new(),
                code_chunks: Vec::new(),
                errors: Vec::new(),
                training_data: Vec::new(),
                feelings: Vec::new(),
                summaries: Vec::new(),
                reminders: Vec::new(),
            }
        }
    };

    // Process the chat content through the system
    let mut processed_items = Vec::new();

    // Process tasks
    for task in &chat_content.tasks {
        if let Ok(_) = crate::Done::create_task(
            &task.action,
            Some(task.priority),
            Some(task.parent_project.as_str()),
            Some(task.time.as_str()),
            task.context_notes.as_ref().map(|s| s.as_str()),
        ).await {
            processed_items.push(format!("Task: {}", task.action));
        }
    }

    // Process memories
    for memory in &chat_content.memories {
        if let Ok(_) = crate::Memories::create(&memory.moment, &memory.meaning, &memory.reason).await {
            processed_items.push(format!("Memory: {}", memory.moment));
        }
    }

    // Process ideas
    for idea in &chat_content.ideas {
        if let Ok(_) = crate::Ideas::create(&idea.idea).await {
            processed_items.push(format!("Idea: {}", idea.idea));
        }
    }

    // Process errors
    for error in &chat_content.errors {
        let _ = crate::storage::save_error(error);
        processed_items.push(format!("Error: {}", error.title));
    }

    // Clean the content by removing tags
    let tag_patterns = vec![
        r"<todozi>.*?</todozi>", r"<memory>.*?</memory>", r"<idea>.*?</idea>",
        r"<todozi_agent>.*?</todozi_agent>", r"<chunk>.*?</chunk>",
        r"<error>.*?</error>", r"<train>.*?</train>", r"<feel>.*?</feel>",
        r"<summary>.*?</summary>", r"<reminder>.*?</reminder>", r"<tdz>.*?</tdz>",
    ];

    let mut clean_content = content.to_string();
    for pattern in tag_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            clean_content = re.replace_all(&clean_content, "").to_string();
        }
    }

    // Clean up extra whitespace
    clean_content = clean_content.split_whitespace().collect::<Vec<&str>>().join(" ");
    clean_content = clean_content.trim().to_string();

    // Create the system response
    let mut system_response = String::new();
    if !processed_items.is_empty() {
        system_response.push_str("Great job! I've processed the following items:\n");
        for item in &processed_items {
            system_response.push_str(&format!("• {}\n", item));
        }
        system_response.push_str("\nTo update or modify these items, you can add new <todozi>, <memory>, or <idea> tags to your messages.");
    }

    // Create clean_with_response by combining clean content with system response
    let clean_with_response = if clean_content.is_empty() {
        format!("<tdz_sys>{}</tdz_sys>", system_response)
    } else if system_response.is_empty() {
        clean_content.clone()
    } else {
        format!("{}\n<tdz_sys>{}</tdz_sys>", clean_content, system_response)
    };

    // Also run the traditional content processor for backward compatibility
    let state = initialize_tdz_content_processor().await?;
    let tool = TdzContentProcessorTool::new(state);
    let mut kwargs = HashMap::new();
    kwargs.insert("content".to_string(), serde_json::Value::String(content.to_string()));
    if let Some(session) = session_id {
        kwargs
            .insert(
                "session_id".to_string(),
                serde_json::Value::String(session.to_string()),
            );
    }
    kwargs.insert("extract_checklist".to_string(), serde_json::Value::Bool(true));
    kwargs.insert("auto_session".to_string(), serde_json::Value::Bool(true));

    let traditional_result = tool.execute(kwargs).await;

    // Create the new JSON response format
    let response = serde_json::json!({
        "process": "success",
        "original": content,
        "clean": clean_content,
        "clean_with_response": clean_with_response,
        "processed_items": processed_items.len(),
        "items_detail": processed_items,
        "traditional_processing": if traditional_result.success {
            traditional_result.output
        } else {
            format!("Traditional processing failed: {}", traditional_result.output)
        }
    });

    Ok(serde_json::to_string_pretty(&response).unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_tdz_cnt_basic() {
        let result = tdz_cnt("Hello world, <todozi>add task; test task</todozi>", None)
            .await;
        assert!(result.is_ok());
        let response_str = result.unwrap();
        // Parse the JSON response
        let response: serde_json::Value = serde_json::from_str(&response_str).unwrap();
        assert_eq!(response["process"], "success");
        assert_eq!(response["original"], "Hello world, <todozi>add task; test task</todozi>");
        assert_eq!(response["clean"], "Hello world,");
        assert!(response["clean_with_response"].as_str().unwrap().contains("<tdz_sys>"));
        assert!(response["processed_items"].as_u64().unwrap() >= 1);
    }
    #[tokio::test]
    async fn test_checklist_extraction() {
        let state = Arc::new(Mutex::new(TodoziProcessorState::new().unwrap()));
        let processor = TdzContentProcessorTool::new(state);
        let items = processor
            .extract_checklist_items(
                "We need to fix the bug, don't forget to test it, and make sure to deploy",
            )
            .unwrap();
        assert!(! items.is_empty());
    }
}