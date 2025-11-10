#![cfg(feature = "php")]
// PHP bindings for Todozi using ext-php-rs
// Generated automatically - review before use

#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use crate::{Done, Tdz, Actions, Projects, Memories, Ideas, Queue, Find, Emb, Stats, Easy, Tags};
use crate::models::{Task, Memory, Idea, QueueItem, Project, ApiKey, Tag, Summary, Reminder};
use tokio::runtime::Runtime;
use std::sync::OnceLock;
use ext_php_rs::ffi::E_ALL;
use ext_php_rs::error::php_error;
use ext_php_rs::exception::PhpException;
use crate::TodoziContentType;
use crate::IdeaImportance;
use crate::types::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use serde_json;
use serde_json::Value;
use crate::emb::SimilarityResult;
use crate::emb::SearchFilters;
use crate::emb::ClusteringResult;
use crate::emb::LabeledCluster;
use crate::emb::SimilarityGraph;
use crate::emb::ModelComparisonResult;
use crate::emb::DiagnosticReport;
use crate::emb::TodoziEmbeddingConfig;
use crate::emb::TodoziEmbeddingService;
use crate::emb::TodoziEmbeddingCache;
use crate::emb::EmbeddingModel;
use crate::emb::HierarchicalCluster;
use crate::chunking::CodeChunk;
use crate::chunking::ChunkingLevel;
use crate::models::Error;
use crate::ErrorType;
use crate::TodoziError;
use crate::ToolResult;
use crate::Tool;
use crate::ToolParameter;
use crate::ResourceLock;
use crate::ToolDefinition;
use crate::models::MemoryImportance;
use crate::models::MemoryTerm;
use crate::memory::MemoryStatistics;
use crate::idea::IdeaStatistics;
use crate::models::ShareLevel;
use crate::project::ProjectStats;
use crate::models::SummaryPriority;
use crate::summary::SummaryStatistics;
use crate::tag::TagStatistics;
use crate::models::Agent;
use crate::models::AgentAssignment;
use crate::models::AgentTool;
use crate::models::ReminderPriority;
use crate::models::ReminderStatus;
use crate::models::QueueStatus;
use crate::tui::TaskFilters;
use crate::tui::TaskDisplay;
use crate::tui::TaskListDisplay;
use crate::todozi_tool::SharedTodozi;
use crate::todozi_exe::ExecutionResult;
use crate::Config;
use crate::Priority;
use crate::Storage;
use crate::SearchAnalytics;
use crate::QueueSession;
use crate::RegistrationInfo;
use crate::models::Status;
use crate::models::Assignee;
use crate::tdz::TdzCommand;
use crate::models::Feeling;
use crate::models::ProjectTaskContainer;
use crate::models::ApiKeyCollection;
use crate::models::QueueCollection;
use crate::models::TaskCollection;
use crate::models::TrainingData;
use crate::models::MigrationReport;
use crate::emb::AggregationType;
use crate::models::PerformanceMetrics;
use crate::emb::SemanticSearchResult;
use crate::models::EditSession;
use crate::models::AgentStatus;
use crate::models::DriftReport;
use crate::models::AgentUpdate;
use crate::models::AssignmentStatus;
use crate::models::IdeaUpdate;
use crate::models::MemoryUpdate;
use crate::models::ReminderUpdate;
use crate::models::SummaryUpdate;
use crate::models::TagUpdate;
use crate::models::ValidationReport;
use crate::models::JsCommandValidation;
use crate::tag::TagManager;
use chrono::{DateTime, Utc};
use std::time::Duration;

type E = PhpException;
type PyResult<T> = Result<T, E>;

// Type aliases for PHP bindings (Js* types are typically just the base types)
type JsApiKey = ApiKey;
type JsTask = Task;
type JsIdea = Idea;
type JsMemory = Memory;
type JsReminder = Reminder;
type JsApiKeyAuth = ApiKey;
type JsRegistrationInfo = RegistrationInfo;
type JsSimilarityResult = SimilarityResult;
type JsClusteringResult = ClusteringResult;
type JsToolParameter = ToolParameter;
type JsToolDefinition = ToolDefinition;
type JsErrorDetails = Error;
type JsQueueItem = QueueItem;
type JsSummary = Summary;
type JsTag = Tag;
type SharedTodoziState = SharedTodozi;
type ExecutorResult<T> = Result<T, TodoziError>;

// Global runtime for PHP extension
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> Result<&'static Runtime, Box<dyn std::error::Error>> {
    RUNTIME.get_or_try_init(|| {
        Runtime::new().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create runtime: {}", e))) as Box<dyn std::error::Error>)
    }).map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Runtime error: {}", e))) as Box<dyn std::error::Error>)
}

// Custom exception class for PHP
#[php_class]
#[derive(Default)]
pub struct TodoziException {
    message: String,
}

#[php_impl]
impl TodoziException {
    #[php_constructor]
    pub fn __construct(&mut self, message: String) {
        self.message = message;
    }
    
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

// PHP class for Todozi
#[php_class]
pub struct Todozi {
    // Instance state if needed
}

#[php_impl]
impl Todozi {
    #[php_constructor]
    pub fn __construct() -> Self {
        Todozi {}
    }
    
    // activate_api_key
    pub fn activate_api_key(&self, user_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::activate_api_key(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // activate_key
    pub fn activate_key(&self, user_id: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::activate_key(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // activate_reminder
    pub fn activate_reminder(&self, reminder_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::activate_reminder(reminder_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // active
    pub fn active(&self) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::active().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // active_percentage
    pub fn active_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::active_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add
    pub fn add(&self, task_name: &str, description: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add(task_name, description).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_checklist_item
    pub fn add_checklist_item(&self, item: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_checklist_item(item).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_chunk
    pub fn add_chunk(&self, chunk_id: &str, level: &str, deps: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_chunk(chunk_id, level, deps).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_completed_module
    pub fn add_completed_module(&self, module: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_completed_module(module).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_dependency
    pub fn add_dependency(&self, dep: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_dependency(dep).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_error_pattern
    pub fn add_error_pattern(&self, pattern: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_error_pattern(pattern).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_function_signature
    pub fn add_function_signature(&self, name: &str, signature: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_function_signature(name, signature).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_import
    pub fn add_import(&self, import_stmt: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_import(import_stmt).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_item
    pub fn add_item(&self, content: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_item(content).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_key
    pub fn add_key(&self, key: JsApiKey) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_key(key).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_pending_module
    pub fn add_pending_module(&self, module: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_pending_module(module).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_processed_action
    pub fn add_processed_action(&self, action_type: &str, description: &str, success: bool, result: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_processed_action(action_type, description, success, result.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_queue_item
    pub fn add_queue_item(&self, item: QueueItem) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_queue_item(item).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_recent
    pub fn add_recent(&self, description: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_recent(description).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_recent_action
    pub fn add_recent_action(&self, action: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_recent_action(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_tag_relationship
    pub fn add_tag_relationship(&self, tag_id: &str, related_tag_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_tag_relationship(tag_id, related_tag_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_tag_to_task
    pub fn add_tag_to_task(&self, task_id: &str, tag: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_tag_to_task(task_id, tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_task
    pub fn add_task(&self, task: Task) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_task(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_task_emb
    pub fn add_task_emb(&self, task: JsTask) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_task_emb(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_task_to_project
    pub fn add_task_to_project(&self, task: Task) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::add_task_to_project(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // add_to_task
    pub fn add_to_task(&self, task_id: &str, tag: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::add_to_task(task_id, tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // advanced_search
    pub fn advanced_search(&self, query: &str) -> Result<Vec<Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // advanced_search_with_filters
    pub fn advanced_search_with_filters(&self, query: &str, data_types: Option<Vec<String>>, limit: Option<u32>) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::advanced_search_with_filters(query, data_types.as_ref(), limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ai
    pub fn ai(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::ai(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ai_find
    pub fn ai_find(&self, query: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::ai_find(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ai_search
    pub fn ai_search(&self, query: &str) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Find::ai_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ai_task
    pub fn ai_task(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Find::ai_task(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ai_tasks
    pub fn ai_tasks(&self, query: &str) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Find::ai_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // all
    pub fn all(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::all().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // all_tasks
    pub fn all_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::all_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // analyze_code_quality
    pub fn analyze_code_quality(&self, _features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::analyze_code_quality(_features).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // api
    pub fn api(&self, message: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::api(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // api_key
    pub fn api_key(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::api_key().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // archive_project
    pub fn archive_project(&self, name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::archive_project(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // as_str
    pub fn as_str(&self) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::as_str().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // assign_task_to_agent
    pub fn assign_task_to_agent(&self, task_id: &str, agent_id: &str, project_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::assign_task_to_agent(task_id, agent_id, project_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // auto_categorize_content
    pub fn auto_categorize_content(&self, text: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::auto_categorize_content(text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // auto_label_clusters
    pub fn auto_label_clusters(&self, clusters: Vec<ClusteringResult>) -> Result<Vec<LabeledCluster, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::auto_label_clusters(clusters).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // backlog
    pub fn backlog(&self) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Queue::backlog().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // backup_embeddings
    pub fn backup_embeddings(&self, backup_path: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::backup_embeddings(backup_path.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // batch_embed_content
    pub fn batch_embed_content(&self, content_list: Vec<String>) -> Result<Vec<Vec<f64, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::batch_embed_content(content_list).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // begin
    pub fn begin(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Actions::begin(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // breakthrough
    pub fn breakthrough(&self, idea: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::breakthrough(idea).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // breakthrough_idea
    pub fn breakthrough_idea(&self, idea: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::breakthrough_idea(idea).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // breakthrough_percentage
    pub fn breakthrough_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::breakthrough_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // build_similarity_graph
    pub fn build_similarity_graph(&self, threshold: f32) -> Result<SimilarityGraph, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::build_similarity_graph(threshold).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // bulk_create_tags
    pub fn bulk_create_tags(&self, tag_names: Vec<String>, category: Option<String>) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::bulk_create_tags(tag_names, category.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // calculate_diversity
    pub fn calculate_diversity(&self, content_ids: Vec<String>) -> Result<f32, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::calculate_diversity(content_ids).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // capabilities
    pub fn capabilities(&self, capabilities: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::capabilities(capabilities).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // category
    pub fn category(&self, category: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::category(category).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // chat
    pub fn chat(&self, message: &str) -> Result<ChatContent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::chat(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // check_api_key_auth
    pub fn check_api_key_auth(&self, public_key: &str, private_key: Option<String>) -> Result<JsApiKeyAuth, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::check_api_key_auth(public_key, private_key.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // check_folder_structure
    pub fn check_folder_structure(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::check_folder_structure().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // check_migration_status
    pub fn check_migration_status(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::check_migration_status().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cleanup_expired
    pub fn cleanup_expired(&self) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cleanup_expired().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cleanup_legacy
    pub fn cleanup_legacy(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cleanup_legacy().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // clear_registration
    pub fn clear_registration(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::clear_registration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_add_task
    pub fn cli_add_task(&self, content: &str, priority: Option<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_add_task(content, priority.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_chat
    pub fn cli_chat(&self, message: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_chat(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_clear_registration
    pub fn cli_clear_registration(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_clear_registration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_complete_task
    pub fn cli_complete_task(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_complete_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_create_backup
    pub fn cli_create_backup(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_create_backup().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_create_idea
    pub fn cli_create_idea(&self, content: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_create_idea(content).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_create_memory
    pub fn cli_create_memory(&self, moment: &str, meaning: &str, reason: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_create_memory(moment, meaning, reason).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_delete_task
    pub fn cli_delete_task(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_delete_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_fix_consistency
    pub fn cli_fix_consistency(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_fix_consistency().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_get_registration_status
    pub fn cli_get_registration_status(&self) -> Result<Option<JsRegistrationInfo, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_get_registration_status().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_list_backups
    pub fn cli_list_backups(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_list_backups().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_list_tasks
    pub fn cli_list_tasks(&self) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_list_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_register_with_server
    pub fn cli_register_with_server(&self, server_url: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_register_with_server(server_url).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_restore_backup
    pub fn cli_restore_backup(&self, backup_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_restore_backup(backup_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_search_tasks
    pub fn cli_search_tasks(&self, query: &str) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_search_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_show_task
    pub fn cli_show_task(&self, task_id: &str) -> Result<JsTask, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_show_task(task_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cli_update_task
    pub fn cli_update_task(&self, task_id: &str, action: Option<String>, priority: Option<String>, status: Option<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cli_update_task(task_id, action.as_deref(), priority.as_deref(), status.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cluster
    pub fn cluster(&self) -> Result<Vec<ClusteringResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cluster().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cluster_content
    pub fn cluster_content(&self) -> Result<Vec<ClusteringResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cluster_content().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // cluster_similar_tasks
    pub fn cluster_similar_tasks(&self) -> Result<Vec<JsClusteringResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::cluster_similar_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // collab
    pub fn collab(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::collab(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // collab_task
    pub fn collab_task(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::collab_task(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // color
    pub fn color(&self, color: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::color(color).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // compare_models
    pub fn compare_models(&self, text: &str, model_aliases: Vec<String>) -> Result<ModelComparisonResult, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::compare_models(text, model_aliases).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // complete
    pub fn complete(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::complete(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // complete_agent_assignment
    pub fn complete_agent_assignment(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::complete_agent_assignment(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // complete_task
    pub fn complete_task(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::complete_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // complete_task_in_project
    pub fn complete_task_in_project(&self, id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::complete_task_in_project(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // completion_rate
    pub fn completion_rate(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::completion_rate().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // config
    pub fn config(&self) -> Result<&Config, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::config().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // configure_display
    pub fn configure_display(&self, config_json: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::configure_display(config_json).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // configure_server
    pub fn configure_server(&self, host: Option<String>, port: Option<u32>, max_connections: Option<u32>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::configure_server(host.as_deref(), port.as_ref(), max_connections.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // content
    pub fn content(&self, content: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::content(content).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // context
    pub fn context(&self, context: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::context(context).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // craft_embedding
    pub fn craft_embedding(&self, _features: &str) -> Result<std::result::Result<Vec<f32, E>, Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::craft_embedding(_features).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create
    pub fn create(&self, name: &str, description: Option<&str>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create(name, description.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_advanced_todozi_tools
    pub fn create_advanced_todozi_tools(&self, todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_advanced_todozi_tools(todozi).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_agent
    pub fn create_agent(&self, agent: Agent) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_agent(agent).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_api_key
    pub fn create_api_key(&self) -> Result<JsApiKey, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_api_key().await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_api_key_with_user_id
    pub fn create_api_key_with_user_id(&self, user_id: &str) -> Result<JsApiKey, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_api_key_with_user_id(user_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_architect_agent
    pub fn create_architect_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_architect_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_backup
    pub fn create_backup(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_backup().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_coder
    pub fn create_coder(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_coder().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_comrad_agent
    pub fn create_comrad_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_comrad_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_custom_agent
    pub fn create_custom_agent(&self, id: &str, name: &str, description: &str, capabilities: Vec<String>, specializations: Vec<String>, category: &str, author: Option<String>) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_custom_agent(id, name, description, capabilities, specializations, category, author.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_default_agents
    pub fn create_default_agents(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_default_agents().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_designer_agent
    pub fn create_designer_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_designer_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_detective_agent
    pub fn create_detective_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_detective_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_devops_agent
    pub fn create_devops_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_devops_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_embedding_service
    pub fn create_embedding_service(&self) -> Result<TodoziEmbeddingService, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_embedding_service().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_embedding_version
    pub fn create_embedding_version(&self, content_id: &str, version_label: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_embedding_version(content_id, version_label).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_error
    pub fn create_error(&self, error: Error) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_error(error).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_error_result
    pub fn create_error_result(&self, error_msg: &str, execution_time_ms: u64, error_type: ErrorType, metadata: Option<HashMap<String, serde_json::Value>>) -> Result<ToolResult, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_error_result(error_msg, execution_time_ms, error_type, metadata.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_filters
    pub fn create_filters(&self) -> Result<TaskFilters, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_filters().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_finisher_agent
    pub fn create_finisher_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_finisher_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_framer_agent
    pub fn create_framer_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_framer_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_friend_agent
    pub fn create_friend_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_friend_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_grok_level_todozi_tools
    pub fn create_grok_level_todozi_tools(&self, todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_grok_level_todozi_tools(todozi).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_hoarder_agent
    pub fn create_hoarder_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_hoarder_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_idea
    pub fn create_idea(&self, idea: Idea) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_idea(idea).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_investigator_agent
    pub fn create_investigator_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_investigator_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_mason_agent
    pub fn create_mason_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_mason_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_memory
    pub fn create_memory(&self, memory: Memory) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_memory(memory).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_nerd_agent
    pub fn create_nerd_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_nerd_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_nun_agent
    pub fn create_nun_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_nun_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_overlord_agent
    pub fn create_overlord_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_overlord_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_party_agent
    pub fn create_party_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_party_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_planner_agent
    pub fn create_planner_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_planner_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_project
    pub fn create_project(&self, name: &str, description: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Projects::create_project(name, description.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_recycler_agent
    pub fn create_recycler_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_recycler_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_reminder
    pub fn create_reminder(&self, reminder: Reminder) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_reminder(reminder).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_search_options
    pub fn create_search_options(&self, data_types: Option<Vec<String>>, limit: Option<u32>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_search_options(data_types.as_ref(), limit.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_skeleton_agent
    pub fn create_skeleton_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_skeleton_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_snitch_agent
    pub fn create_snitch_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_snitch_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_storage
    pub fn create_storage(&self) -> Result<Storage, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_storage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_success_result
    pub fn create_success_result(&self, output: &str, execution_time_ms: u64, metadata: Option<HashMap<String, serde_json::Value>>) -> Result<ToolResult, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_success_result(output, execution_time_ms, metadata.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_summary
    pub fn create_summary(&self, summary: Summary) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_summary(summary).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tag
    pub fn create_tag(&self, tag: Tag) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::create_tag(tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_task
    pub fn create_task(&self, action: &str, priority: Option<Priority>, project: Option<&str>, time: Option<&str>, context: Option<&str>) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_task(action, priority.as_ref(), project.as_deref(), time.as_deref(), context.as_deref()).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_task_filters
    pub fn create_task_filters(&self, project: Option<String>, status: Option<String>, priority: Option<String>, assignee: Option<String>, tags: Option<String>, search: Option<String>) -> Result<TaskFilters, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_task_filters(project.as_deref(), status.as_deref(), priority.as_deref(), assignee.as_deref(), tags.as_deref(), search.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tdz_content_processor_tool
    pub fn create_tdz_content_processor_tool(&self, state: SharedTodoziState) -> Result<Box<dyn Tool + Send + Sync, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tdz_content_processor_tool(state).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tdz_tool
    pub fn create_tdz_tool(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tdz_tool().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tester_agent
    pub fn create_tester_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tester_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_todozi_tools
    pub fn create_todozi_tools(&self, todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_todozi_tools(todozi).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_todozi_tools_with_embedding
    pub fn create_todozi_tools_with_embedding(&self, todozi: SharedTodozi, embedding_service: Option<TodoziEmbeddingService>) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_todozi_tools_with_embedding(todozi, embedding_service.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tool_definition
    pub fn create_tool_definition(&self, name: &str, description: &str, category: &str, parameters: Vec<JsToolParameter>) -> Result<JsToolDefinition, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tool_definition(name, description, category, parameters).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tool_definition_with_locks
    pub fn create_tool_definition_with_locks(&self, name: &str, description: &str, category: &str, parameters: Vec<ToolParameter>, resource_locks: Vec<ResourceLock>) -> Result<ToolDefinition, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tool_definition_with_locks(name, description, category, parameters, resource_locks).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tool_parameter
    pub fn create_tool_parameter(&self, name: &str, type_: &str, description: &str, required: bool) -> Result<JsToolParameter, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tool_parameter(name, type_, description, required).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tool_parameter_with_default
    pub fn create_tool_parameter_with_default(&self, name: &str, type_: &str, description: &str, required: bool, default: &str) -> Result<JsToolParameter, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tool_parameter_with_default(name, type_, description, required, default).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tui_app
    pub fn create_tui_app(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tui_app().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_tuner_agent
    pub fn create_tuner_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_tuner_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_update
    pub fn create_update(&self) -> Result<TaskUpdate, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_update().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // create_writer_agent
    pub fn create_writer_agent(&self) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::create_writer_agent().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // critical_percentage
    pub fn critical_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::critical_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // deactivate_api_key
    pub fn deactivate_api_key(&self, user_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::deactivate_api_key(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // deactivate_key
    pub fn deactivate_key(&self, user_id: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::deactivate_key(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // deep
    pub fn deep(&self, query: &str) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::deep(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // deep_search
    pub fn deep_search(&self, query: &str) -> Result<Vec<JsSimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::deep_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // default_filters
    pub fn default_filters(&self) -> Result<TaskFilters, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::default_filters().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // default_update
    pub fn default_update(&self) -> Result<TaskUpdate, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::default_update().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete
    pub fn delete(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_agent
    pub fn delete_agent(&self, agent_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_agent(agent_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_agent_assignment
    pub fn delete_agent_assignment(&self, agent_id: &str, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_agent_assignment(agent_id, task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_code_chunk
    pub fn delete_code_chunk(&self, chunk_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_code_chunk(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_error
    pub fn delete_error(&self, error_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_error(error_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_feeling
    pub fn delete_feeling(&self, id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_feeling(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_idea
    pub fn delete_idea(&self, idea_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_idea(idea_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_memory
    pub fn delete_memory(&self, memory_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_memory(memory_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_project
    pub fn delete_project(&self, project_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_project(project_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_project_task_container
    pub fn delete_project_task_container(&self, project_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_project_task_container(project_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_reminder
    pub fn delete_reminder(&self, reminder_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_reminder(reminder_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_summary
    pub fn delete_summary(&self, summary_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_summary(summary_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_tag
    pub fn delete_tag(&self, tag_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_tag(tag_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_task
    pub fn delete_task(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_task_from_project
    pub fn delete_task_from_project(&self, id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_task_from_project(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // delete_training_data
    pub fn delete_training_data(&self, training_data_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::delete_training_data(training_data_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // description
    pub fn description(&self, description: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::description(description).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // detailed
    pub fn detailed(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::detailed().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // detailed_stats
    pub fn detailed_stats(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::detailed_stats().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // display_task
    pub fn display_task(&self, task_id: &str) -> Result<TaskDisplay, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::display_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // display_tasks
    pub fn display_tasks(&self, task_ids: Vec<String>) -> Result<TaskListDisplay, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::display_tasks(task_ids).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // do_it
    pub fn do_it(&self, what: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Easy::do_it(what).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done
    pub fn done(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::done(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_api_key
    pub fn done_api_key(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_api_key().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_create_embedding_service
    pub fn done_create_embedding_service(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_create_embedding_service().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_create_storage
    pub fn done_create_storage(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_create_storage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_default_filters
    pub fn done_default_filters(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_default_filters().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_default_update
    pub fn done_default_update(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_default_update().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_embedding_config
    pub fn done_embedding_config(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_embedding_config().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_embedding_service
    pub fn done_embedding_service(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_embedding_service().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_init
    pub fn done_init(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_init().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_process_chat
    pub fn done_process_chat(&self, message: &str, user_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_process_chat(message, user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_sample_task
    pub fn done_sample_task(&self) -> Result<JsTask, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_sample_task().await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_storage
    pub fn done_storage(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_storage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // done_types
    pub fn done_types(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::done_types().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // dry_run
    pub fn dry_run(&self, dry_run: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::dry_run(dry_run).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // easy_done
    pub fn easy_done(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::easy_done(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // easy_find
    pub fn easy_find(&self, what: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::easy_find(what).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // easy_idea
    pub fn easy_idea(&self, what: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::easy_idea(what).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // easy_remember
    pub fn easy_remember(&self, what: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::easy_remember(what).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embed
    pub fn embed(&self, text: &str) -> Result<Vec<f32, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embed(text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embed_idea
    pub fn embed_idea(&self, idea: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embed_idea(idea).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embed_memory
    pub fn embed_memory(&self, memory: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embed_memory(memory).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embed_stats
    pub fn embed_stats(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embed_stats().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embed_tag
    pub fn embed_tag(&self, tag: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embed_tag(tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embed_task
    pub fn embed_task(&self, task_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Emb::embed_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embedding_config
    pub fn embedding_config(&self) -> Result<TodoziEmbeddingConfig, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embedding_config().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // embedding_service
    pub fn embedding_service(&self) -> Result<TodoziEmbeddingService, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::embedding_service().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // encode
    pub fn encode(&self, texts: &str) -> Result<Vec<Vec<f32, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::encode(texts).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // end_queue_session
    pub fn end_queue_session(&self, session_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::end_queue_session(session_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // end_session
    pub fn end_session(&self, session_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::end_session(session_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ensure_folder_structure
    pub fn ensure_folder_structure(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::ensure_folder_structure().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ensure_todozi_initialized
    pub fn ensure_todozi_initialized(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::ensure_todozi_initialized().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // error
    pub fn error(&self, error: &str, execution_time_ms: u64) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::error(error, execution_time_ms).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // example
    pub fn example(&self) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::example().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // example_usage
    pub fn example_usage(&self) -> Result<std::result::Result<(, E), Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::example_usage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // execute_task
    pub fn execute_task(&self, storage: &str, task: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::execute_task(storage, task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // execute_tdz_command
    pub fn execute_tdz_command(&self, command: &str, base_url: &str, api_key: Option<&str>) -> Result<Value, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::execute_tdz_command(command, base_url, api_key.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // execute_todozi_tool_delegated
    pub fn execute_todozi_tool_delegated(&self, params: &str) -> Result<ExecutorResult<ExecutionResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::execute_todozi_tool_delegated(params).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // execute_tool
    pub fn execute_tool(&self, tool_name: &str, kwargs: HashMap<String, serde_json::Value>) -> Result<ToolResult, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::execute_tool(tool_name, kwargs).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // explain_search_result
    pub fn explain_search_result(&self, query: &str, result: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::explain_search_result(query, result).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // export_diagnostics
    pub fn export_diagnostics(&self) -> Result<DiagnosticReport, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::export_diagnostics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // export_embedded_tasks_hlx
    pub fn export_embedded_tasks_hlx(&self, output_path: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::export_embedded_tasks_hlx(output_path).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // export_for_fine_tuning
    pub fn export_for_fine_tuning(&self, output_path: &str) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::export_for_fine_tuning(output_path).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // export_to_format
    pub fn export_to_format(&self, format: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::export_to_format(format).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // extract_content
    pub fn extract_content(&self, content: Option<String>, file_path: Option<String>, output_format: &str, human: bool) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::extract_content(content.as_deref(), file_path.as_deref(), output_format, human).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // extract_content_from_text
    pub fn extract_content_from_text(&self, text: &str, format: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::extract_content_from_text(text, format.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // extract_task_actions
    pub fn extract_task_actions(&self, content: &str) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::extract_task_actions(content).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // extract_tasks
    pub fn extract_tasks(&self, content: &str, context: Option<&str>) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::extract_tasks(content, context.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // fast
    pub fn fast(&self, query: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::fast(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // fast_search
    pub fn fast_search(&self, query: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::fast_search(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // filtered_semantic_search
    pub fn filtered_semantic_search(&self, query: &str, filters: SearchFilters, limit: usize) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::filtered_semantic_search(query, filters, limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find
    pub fn find(&self, query: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::find(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_best_agent
    pub fn find_best_agent(&self, required_specialization: &str, preferred_capability: Option<String>) -> Result<Option<&Agent, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_best_agent(required_specialization, preferred_capability.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_by_tag
    pub fn find_by_tag(&self, tag_name: &str) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_by_tag(tag_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_cross_content_relationships
    pub fn find_cross_content_relationships(&self, content_id: &str, content_type: TodoziContentType, min_similarity: f32) -> Result<HashMap<TodoziContentType, Vec<SimilarityResult, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_cross_content_relationships(content_id, content_type, min_similarity).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_ideas
    pub fn find_ideas(&self, query: &str) -> Result<Vec<JsIdea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_ideas(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_memories
    pub fn find_memories(&self, query: &str) -> Result<Vec<JsMemory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_memories(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_outliers
    pub fn find_outliers(&self, content_type: TodoziContentType, threshold: f32) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_outliers(content_type, threshold).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_similar_tags
    pub fn find_similar_tags(&self, tag_name: &str, limit: Option<usize>) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_similar_tags(tag_name, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_similar_tasks
    pub fn find_similar_tasks(&self, task_description: &str, limit: Option<usize>) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_similar_tasks(task_description, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_tasks
    pub fn find_tasks(&self, query: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_tasks_ai
    pub fn find_tasks_ai(&self, query: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_tasks_ai(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_tdz
    pub fn find_tdz(&self, str: Option<&str>) -> Result<Option<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_tdz(str.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // find_todozi
    pub fn find_todozi(&self, str: Option<String>) -> Result<Option<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::find_todozi(str.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // fix_completed_tasks_consistency
    pub fn fix_completed_tasks_consistency(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::fix_completed_tasks_consistency().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // fix_task_consistency
    pub fn fix_task_consistency(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::fix_task_consistency().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // force_overwrite
    pub fn force_overwrite(&self, force_overwrite: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::force_overwrite(force_overwrite).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_duration
    pub fn format_duration(&self, from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_duration(from, to).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_error_with_context
    pub fn format_error_with_context(&self, error_message: &str, context: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_error_with_context(error_message, context).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_project_stats
    pub fn format_project_stats(&self, project_name: &str, task_count: usize, completed_count: usize) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_project_stats(project_name, task_count, completed_count).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_task
    pub fn format_task(&self, task: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_task(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_task_list
    pub fn format_task_list(&self, tasks: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_task_list(tasks).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_task_with_emojis
    pub fn format_task_with_emojis(&self, task: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_task_with_emojis(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // format_time_estimate
    pub fn format_time_estimate(&self, time: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::format_time_estimate(time).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // fuzzy_search
    pub fn fuzzy_search(&self, query: &str, max_distance: usize) -> Result<Vec<(&Tag, usize), E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::fuzzy_search(query, max_distance).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // generate_embedding
    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::generate_embedding(text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // generate_embeddings_batch
    pub fn generate_embeddings_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::generate_embeddings_batch(texts).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // generate_task_embedding
    pub fn generate_task_embedding(&self, task: &str) -> Result<Option<Vec<f32, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::generate_task_embedding(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get
    pub fn get(&self, task_id: &str) -> Result<Option<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_active_items
    pub fn get_active_items(&self) -> Result<Vec<&QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_active_items().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_active_keys
    pub fn get_active_keys(&self) -> Result<Vec<&ApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_active_keys().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_active_reminders
    pub fn get_active_reminders(&self) -> Result<Vec<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_active_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_active_sessions
    pub fn get_active_sessions(&self) -> Result<Vec<crate::models::QueueSession, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_active_sessions().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agent
    pub fn get_agent(&self, agent_id: &str) -> Result<Option<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agent(agent_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agent_assignments
    pub fn get_agent_assignments(&self, agent_id: &str) -> Result<Vec<&AgentAssignment, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agent_assignments(agent_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agent_assignments_dir
    pub fn get_agent_assignments_dir(&self, agent_id: &str) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agent_assignments_dir(agent_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agent_statistics
    pub fn get_agent_statistics(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agent_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agents_by_capability
    pub fn get_agents_by_capability(&self, capability: &str) -> Result<Vec<&Agent, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agents_by_capability(capability).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agents_by_specialization
    pub fn get_agents_by_specialization(&self, specialization: &str) -> Result<Vec<&Agent, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agents_by_specialization(specialization).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agents_dir
    pub fn get_agents_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agents_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_agents_with_assignments
    pub fn get_agents_with_assignments(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_agents_with_assignments().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_ai_tasks
    pub fn get_ai_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_active_tasks
    pub fn get_all_active_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_agents
    pub fn get_all_agents(&self) -> Result<Vec<&Agent, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_agents().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_categories
    pub fn get_all_categories(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_categories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_completed_tasks
    pub fn get_all_completed_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_ideas
    pub fn get_all_ideas(&self) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_all_ideas().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_items
    pub fn get_all_items(&self) -> Result<Vec<&QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_items().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_keys
    pub fn get_all_keys(&self) -> Result<Vec<&ApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_keys().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_memories
    pub fn get_all_memories(&self) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_memories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_reminders
    pub fn get_all_reminders(&self) -> Result<Vec<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_summaries
    pub fn get_all_summaries(&self) -> Result<Vec<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_summaries().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_tags
    pub fn get_all_tags(&self) -> Result<Vec<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_all_tags().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_tasks
    pub fn get_all_tasks(&self) -> Result<Vec<&Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_tasks().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_all_tools
    pub fn get_all_tools(&self) -> Result<Vec<&Box<dyn Tool, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_all_tools().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_api_key
    pub fn get_api_key(&self, user_id: &str) -> Result<JsApiKey, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_api_key(user_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_api_key_by_public
    pub fn get_api_key_by_public(&self, public_key: &str) -> Result<JsApiKey, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_api_key_by_public(public_key).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_assignee_emoji
    pub fn get_assignee_emoji(&self, assignee: &str) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_assignee_emoji(assignee).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_assignments_dir
    pub fn get_assignments_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_assignments_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_available_agents
    pub fn get_available_agents(&self) -> Result<Vec<Agent, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_available_agents().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_backlog_items
    pub fn get_backlog_items(&self) -> Result<Vec<&QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_backlog_items().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_breakthrough_ideas
    pub fn get_breakthrough_ideas(&self) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_breakthrough_ideas().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunk
    pub fn get_chunk(&self, chunk_id: &str) -> Result<Option<&CodeChunk, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunk(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunk_mut
    pub fn get_chunk_mut(&self, chunk_id: &str) -> Result<Option<&mut CodeChunk, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunk_mut(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunk_status
    pub fn get_chunk_status(&self, chunk_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunk_status(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunking_level_info
    pub fn get_chunking_level_info(&self, level: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunking_level_info(level).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunking_levels
    pub fn get_chunking_levels(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunking_levels().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunks_by_level
    pub fn get_chunks_by_level(&self, level: ChunkingLevel) -> Result<Vec<&CodeChunk, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunks_by_level(level).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_chunks_dir
    pub fn get_chunks_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_chunks_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_collaborative_tasks
    pub fn get_collaborative_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_command_documentation
    pub fn get_command_documentation(&self, command_name: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_command_documentation(command_name.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_complete_items
    pub fn get_complete_items(&self) -> Result<Vec<&QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_complete_items().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_critical_memories
    pub fn get_critical_memories(&self) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_critical_memories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_current_duration
    pub fn get_current_duration(&self) -> Result<u64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_current_duration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_default_model
    pub fn get_default_model(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_default_model().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_dependency_chain
    pub fn get_dependency_chain(&self, chunk_id: &str) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_dependency_chain(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_embedding_statistics
    pub fn get_embedding_statistics(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_embedding_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_emotional_memories
    pub fn get_emotional_memories(&self, emotion: &str) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_emotional_memories(emotion).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_enabled_tools
    pub fn get_enabled_tools(&self) -> Result<Vec<&AgentTool, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_enabled_tools().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_error_details
    pub fn get_error_details(&self, error_message: &str) -> Result<JsErrorDetails, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_error_details(error_message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_errors_dir
    pub fn get_errors_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_errors_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_filtered_tasks
    pub fn get_filtered_tasks(&self, filters: &str) -> Result<Vec<&Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_filtered_tasks(filters).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_formatted_prompt
    pub fn get_formatted_prompt(&self, variables: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_formatted_prompt(variables).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_high_priority_summaries
    pub fn get_high_priority_summaries(&self) -> Result<Vec<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_high_priority_summaries().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_human_memories
    pub fn get_human_memories(&self) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_human_memories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_human_tasks
    pub fn get_human_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_idea
    pub fn get_idea(&self, idea_id: &str) -> Result<Option<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_idea(idea_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_idea_statistics
    pub fn get_idea_statistics(&self) -> Result<IdeaStatistics, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_idea_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_ideas_by_importance
    pub fn get_ideas_by_importance(&self, importance: IdeaImportance) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_ideas_by_importance(importance).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_ideas_by_share_level
    pub fn get_ideas_by_share_level(&self, share_level: ShareLevel) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_ideas_by_share_level(share_level).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_ideas_by_tag
    pub fn get_ideas_by_tag(&self, tag: &str) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_ideas_by_tag(tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_ideas_dir
    pub fn get_ideas_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_ideas_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_item
    pub fn get_item(&self, id: &str) -> Result<Option<&QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_item(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_item_mut
    pub fn get_item_mut(&self, id: &str) -> Result<Option<&mut QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_item_mut(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_items_by_status
    pub fn get_items_by_status(&self, status: QueueStatus) -> Result<Vec<&QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_items_by_status(status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_key
    pub fn get_key(&self, user_id: &str) -> Result<Option<&ApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_key(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_key_by_public
    pub fn get_key_by_public(&self, public_key: &str) -> Result<Option<&ApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_key_by_public(public_key).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_long_term_memories
    pub fn get_long_term_memories(&self) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_long_term_memories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memories_by_importance
    pub fn get_memories_by_importance(&self, importance: MemoryImportance) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_memories_by_importance(importance).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memories_by_tag
    pub fn get_memories_by_tag(&self, tag: &str) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_memories_by_tag(tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memories_by_term
    pub fn get_memories_by_term(&self, term: MemoryTerm) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_memories_by_term(term).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memories_by_type
    pub fn get_memories_by_type(&self, memory_type: &str) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_memories_by_type(memory_type).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memories_dir
    pub fn get_memories_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_memories_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memory
    pub fn get_memory(&self, memory_id: &str) -> Result<Option<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Memories::get_memory(memory_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_memory_statistics
    pub fn get_memory_statistics(&self) -> Result<MemoryStatistics, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Memories::get_memory_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_most_used_tags
    pub fn get_most_used_tags(&self, limit: usize) -> Result<Vec<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_most_used_tags(limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_next_chunk_to_work_on
    pub fn get_next_chunk_to_work_on(&self) -> Result<Option<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_next_chunk_to_work_on().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_or_generate_embedding
    pub fn get_or_generate_embedding(&self, content_id: &str, text: &str, content_type: TodoziContentType, refresh_if_stale: bool) -> Result<Vec<f32, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_or_generate_embedding(content_id, text, content_type, refresh_if_stale).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_overdue_reminders
    pub fn get_overdue_reminders(&self) -> Result<Vec<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_overdue_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_pending_reminders
    pub fn get_pending_reminders(&self) -> Result<Vec<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_pending_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_priority_emoji
    pub fn get_priority_emoji(&self, priority: &str) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_priority_emoji(priority).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_private_ideas
    pub fn get_private_ideas(&self) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_private_ideas().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_processor_state
    pub fn get_processor_state(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_processor_state().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_project
    pub fn get_project(&self, name: &str) -> Result<Project, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_project(name).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_project_stats
    pub fn get_project_stats(&self, project_name: &str) -> Result<ProjectStats, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Projects::get_project_stats(project_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_project_summary
    pub fn get_project_summary(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Projects::get_project_summary().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_project_tasks
    pub fn get_project_tasks(&self, project_name: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_project_tasks_dir
    pub fn get_project_tasks_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_project_tasks_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_public_ideas
    pub fn get_public_ideas(&self) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_public_ideas().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_queue_item
    pub fn get_queue_item(&self, id: &str) -> Result<QueueItem, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Queue::get_queue_item(id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_queue_session
    pub fn get_queue_session(&self, session_id: &str) -> Result<crate::models::QueueSession, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Queue::get_queue_session(session_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_queue_statistics
    pub fn get_queue_statistics(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Queue::get_queue_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_ready_chunks
    pub fn get_ready_chunks(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_ready_chunks().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_recent_actions
    pub fn get_recent_actions(&self, limit: Option<u32>) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_recent_actions(limit.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_recent_ideas
    pub fn get_recent_ideas(&self, limit: usize) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_recent_ideas(limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_recent_memories
    pub fn get_recent_memories(&self, limit: usize) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_recent_memories(limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_recent_reminders
    pub fn get_recent_reminders(&self, limit: usize) -> Result<Vec<&Reminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_recent_reminders(limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_recent_summaries
    pub fn get_recent_summaries(&self, limit: usize) -> Result<Vec<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_recent_summaries(limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_recent_tags
    pub fn get_recent_tags(&self, limit: usize) -> Result<Vec<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_recent_tags(limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_registration_info
    pub fn get_registration_info(&self) -> Result<Option<RegistrationInfo, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_registration_info().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_related_tags
    pub fn get_related_tags(&self, tag_id: &str) -> Result<Vec<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_related_tags(tag_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_reminder
    pub fn get_reminder(&self, reminder_id: &str) -> Result<Option<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_reminder(reminder_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_reminder_statistics
    pub fn get_reminder_statistics(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_reminder_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_reminders_by_priority
    pub fn get_reminders_by_priority(&self, priority: ReminderPriority) -> Result<Vec<&Reminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_reminders_by_priority(priority).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_reminders_by_status
    pub fn get_reminders_by_status(&self, status: ReminderStatus) -> Result<Vec<&Reminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_reminders_by_status(status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_reminders_by_tag
    pub fn get_reminders_by_tag(&self, tag: &str) -> Result<Vec<&Reminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_reminders_by_tag(tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_reminders_due_soon
    pub fn get_reminders_due_soon(&self, duration: Duration) -> Result<Vec<&Reminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_reminders_due_soon(duration).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_search_analytics
    pub fn get_search_analytics(&self) -> Result<SearchAnalytics, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_search_analytics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_search_history
    pub fn get_search_history(&self, limit: Option<u32>) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_search_history(limit.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_search_suggestions
    pub fn get_search_suggestions(&self, query: &str, limit: usize) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_search_suggestions(query, limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_secret_memories
    pub fn get_secret_memories(&self) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_secret_memories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_server_status
    pub fn get_server_status(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_server_status().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_session
    pub fn get_session(&self, id: &str) -> Result<Option<&QueueSession, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_session(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_short_term_memories
    pub fn get_short_term_memories(&self) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_short_term_memories().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_stats
    pub fn get_stats(&self) -> Result<HashMap<String, serde_json::Value, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_stats().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_status_emoji
    pub fn get_status_emoji(&self, status: &str) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_status_emoji(status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_storage_dir
    pub fn get_storage_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_storage_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_suggestions
    pub fn get_suggestions(&self, current_tags: &str, limit: usize) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_suggestions(current_tags, limit).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_summaries_by_priority
    pub fn get_summaries_by_priority(&self, priority: SummaryPriority) -> Result<Vec<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_summaries_by_priority(priority).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_summaries_by_tag
    pub fn get_summaries_by_tag(&self, tag: &str) -> Result<Vec<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_summaries_by_tag(tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_summary
    pub fn get_summary(&self, summary_id: &str) -> Result<Option<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_summary(summary_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_summary_statistics
    pub fn get_summary_statistics(&self) -> Result<SummaryStatistics, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_summary_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tag
    pub fn get_tag(&self, tag_id: &str) -> Result<Option<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_tag(tag_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tag_by_name
    pub fn get_tag_by_name(&self, name: &str) -> Result<Option<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_tag_by_name(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tag_statistics
    pub fn get_tag_statistics(&self) -> Result<TagStatistics, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_tag_statistics().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tags_by_category
    pub fn get_tags_by_category(&self, category: &str) -> Result<Vec<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::get_tags_by_category(category).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_task
    pub fn get_task(&self, id: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_task(id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_task_assignments
    pub fn get_task_assignments(&self, task_id: &str) -> Result<Vec<&AgentAssignment, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_task_assignments(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_task_from_any_project
    pub fn get_task_from_any_project(&self, id: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_task_from_any_project(id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_task_from_project
    pub fn get_task_from_project(&self, project_name: &str, task_id: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_task_from_project(project_name, task_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_task_mut
    pub fn get_task_mut(&self, id: &str) -> Result<Option<&mut Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_task_mut(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tasks_dir
    pub fn get_tasks_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_tasks_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tdz_api_key
    pub fn get_tdz_api_key(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_tdz_api_key().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_team_ideas
    pub fn get_team_ideas(&self) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::get_team_ideas().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tool
    pub fn get_tool(&self, name: &str) -> Result<Option<&Box<dyn Tool, E>>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_tool(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tool_definitions
    pub fn get_tool_definitions(&self) -> Result<Vec<serde_json::Value, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_tool_definitions().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_training_dir
    pub fn get_training_dir(&self) -> Result<PathBuf, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_training_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_tsne_coordinates
    pub fn get_tsne_coordinates(&self, content_ids: Vec<String>, dimensions: usize) -> Result<Vec<(String, Vec<f32, E>)>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_tsne_coordinates(content_ids, dimensions).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_unresolved_errors
    pub fn get_unresolved_errors(&self) -> Result<Vec<&Error, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_unresolved_errors().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // get_version_history
    pub fn get_version_history(&self, content_id: &str) -> Result<Vec<serde_json::Value, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::get_version_history(content_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_add_command
    pub fn handle_add_command(&self, command: AddCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_add_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_agent_command
    pub fn handle_agent_command(&self, command: Commands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_agent_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_ai_commands
    pub fn handle_ai_commands(&self, command: &str, args: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_ai_commands(command, args).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_api_command
    pub fn handle_api_command(&self, command: ApiCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_api_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_chat_command
    pub fn handle_chat_command(&self, command: Commands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_chat_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_emb_command
    pub fn handle_emb_command(&self, command: Commands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_emb_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_error_command
    pub fn handle_error_command(&self, command: Commands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_error_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_extract_command
    pub fn handle_extract_command(&self, content: Option<String>, file: Option<String>, output_format: &str, human: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_extract_command(content.as_deref(), file.as_deref(), output_format, human).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_idea_command
    pub fn handle_idea_command(&self, command: IdeaCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_idea_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_ind_command
    pub fn handle_ind_command(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_ind_command().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_list_backups_command
    pub fn handle_list_backups_command(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_list_backups_command().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_list_command
    pub fn handle_list_command(&self, command: ListCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_list_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_memory_command
    pub fn handle_memory_command(&self, command: MemoryCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_memory_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_project_command
    pub fn handle_project_command(&self, command: ProjectCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_project_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_queue_command
    pub fn handle_queue_command(&self, command: QueueCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_queue_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_search_all_command
    pub fn handle_search_all_command(&self, command: Commands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_search_all_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_search_command
    pub fn handle_search_command(&self, command: SearchCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_search_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_server_command
    pub fn handle_server_command(&self, command: ServerCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_server_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_show_command
    pub fn handle_show_command(&self, command: ShowCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_show_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_stats_command
    pub fn handle_stats_command(&self, _command: StatsCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_stats_command(_command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_strategy_command
    pub fn handle_strategy_command(&self, content: Option<String>, file: Option<String>, output_format: &str, human: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_strategy_command(content.as_deref(), file.as_deref(), output_format, human).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_train_command
    pub fn handle_train_command(&self, command: TrainingCommands) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_train_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // handle_update_command
    pub fn handle_update_command(&self, id: &str, action: Option<String>, time: Option<String>, priority: Option<String>, project: Option<String>, status: Option<String>, assignee: Option<String>, tags: Option<String>, dependencies: Option<String>, context: Option<String>, progress: Option<u8>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::handle_update_command(id, action.as_deref(), time.as_deref(), priority.as_deref(), project.as_deref(), status.as_deref(), assignee.as_deref(), tags.as_deref(), dependencies.as_deref(), context.as_deref(), progress.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // has_capability
    pub fn has_capability(&self, capability: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::has_capability(capability).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // has_results
    pub fn has_results(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::has_results().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // has_specialization
    pub fn has_specialization(&self, specialization: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::has_specialization(specialization).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // has_tool
    pub fn has_tool(&self, tool_name: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::has_tool(tool_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // hash_project_name
    pub fn hash_project_name(&self, project_name: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::hash_project_name(project_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // hierarchical_clustering
    pub fn hierarchical_clustering(&self, content_types: Vec<TodoziContentType>, max_depth: usize) -> Result<Vec<HierarchicalCluster, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::hierarchical_clustering(content_types, max_depth).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // high
    pub fn high(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::high(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // high_priority_percentage
    pub fn high_priority_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::high_priority_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // human
    pub fn human(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::human(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // human_task
    pub fn human_task(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::human_task(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // hybrid_search
    pub fn hybrid_search(&self, query: &str, keywords: Vec<String>, content_types: Option<Vec<TodoziContentType>>, semantic_weight: f32, // 0.0-1.0
        limit: usize) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::hybrid_search(query, keywords, content_types.as_ref(), semantic_weight, // 0.0-1.0
        limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // idea
    pub fn idea(&self, idea: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::idea(idea).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ideate
    pub fn ideate(&self, idea: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::ideate(idea).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // import_from_format
    pub fn import_from_format(&self, data: &str, format: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::import_from_format(data, format).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // importance
    pub fn importance(&self, importance: IdeaImportance) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::importance(importance).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // important
    pub fn important(&self, moment: &str, meaning: &str, reason: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::important(moment, meaning, reason).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // important_memory
    pub fn important_memory(&self, moment: &str, meaning: &str, reason: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::important_memory(moment, meaning, reason).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // increment_tag_usage
    pub fn increment_tag_usage(&self, tag_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::increment_tag_usage(tag_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // init
    pub fn init(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::init().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // init_storage
    pub fn init_storage(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::init_storage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // init_with_auto_registration
    pub fn init_with_auto_registration(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::init_with_auto_registration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // initialize
    pub fn initialize(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::initialize().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // initialize_grok_level_todozi_system
    pub fn initialize_grok_level_todozi_system(&self) -> Result<std::result::Result<
    SharedTodozi,
    TodoziError,
>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::initialize_grok_level_todozi_system().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // initialize_grok_level_todozi_system_with_embedding
    pub fn initialize_grok_level_todozi_system_with_embedding(&self, enable_embeddings: bool) -> Result<std::result::Result<(SharedTodozi, Option<TodoziEmbeddingService>), TodoziError>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::initialize_grok_level_todozi_system_with_embedding(enable_embeddings).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // initialize_tdz_content_processor
    pub fn initialize_tdz_content_processor(&self) -> Result<SharedTodoziState, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::initialize_tdz_content_processor().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // initialize_tdz_processor
    pub fn initialize_tdz_processor(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::initialize_tdz_processor().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // interactive_create_task
    pub fn interactive_create_task(&self) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::interactive_create_task().await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // io
    pub fn io(&self, message: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::io(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_active
    pub fn is_active(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_active().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_admin
    pub fn is_admin(&self, public_key: &str, private_key: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_admin(public_key, private_key).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_available
    pub fn is_available(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_available().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_backlog
    pub fn is_backlog(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_backlog().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_complete
    pub fn is_complete(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_complete().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_completed
    pub fn is_completed(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_completed().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_empty
    pub fn is_empty(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_empty().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_overdue
    pub fn is_overdue(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_overdue().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // is_registered
    pub fn is_registered(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::is_registered().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // keyword_search
    pub fn keyword_search(&self, query: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Find::keyword_search(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // keyword_tasks
    pub fn keyword_tasks(&self, query: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Find::keyword_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // launch_gui
    pub fn launch_gui(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::launch_gui().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // launch_tui
    pub fn launch_tui(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::launch_tui().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // len
    pub fn len(&self) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::len().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list
    pub fn list(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_active_api_keys
    pub fn list_active_api_keys(&self) -> Result<Vec<JsApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_active_api_keys().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_active_items
    pub fn list_active_items(&self) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_active_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_agent_assignments
    pub fn list_agent_assignments(&self, agent_id: &str) -> Result<Vec<AgentAssignment, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_agent_assignments(agent_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_agents
    pub fn list_agents(&self) -> Result<Vec<Agent, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_agents().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_all_agent_assignments
    pub fn list_all_agent_assignments(&self) -> Result<Vec<AgentAssignment, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_all_agent_assignments().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_api_keys
    pub fn list_api_keys(&self) -> Result<Vec<JsApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_api_keys().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_available_agents
    pub fn list_available_agents(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_available_agents().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_available_commands
    pub fn list_available_commands(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_available_commands().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_available_tools
    pub fn list_available_tools(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_available_tools().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_backlog_items
    pub fn list_backlog_items(&self) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_backlog_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_backups
    pub fn list_backups(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_backups().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_code_chunks
    pub fn list_code_chunks(&self) -> Result<Vec<CodeChunk, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_code_chunks().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_complete_items
    pub fn list_complete_items(&self) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_complete_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_errors
    pub fn list_errors(&self) -> Result<Vec<Error, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_errors().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_feelings
    pub fn list_feelings(&self) -> Result<Vec<Feeling, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_feelings().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_ideas
    pub fn list_ideas(&self) -> Result<Vec<Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Ideas::list_ideas().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_memories
    pub fn list_memories(&self) -> Result<Vec<Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_memories().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_project_task_containers
    pub fn list_project_task_containers(&self) -> Result<Vec<ProjectTaskContainer, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_project_task_containers().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_projects
    pub fn list_projects(&self) -> Result<Vec<Project, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Projects::list_projects().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_queue_items
    pub fn list_queue_items(&self) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Queue::list_queue_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_queue_items_by_status
    pub fn list_queue_items_by_status(&self, status: QueueStatus) -> Result<Vec<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Queue::list_queue_items_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_reminders
    pub fn list_reminders(&self) -> Result<Vec<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_summaries
    pub fn list_summaries(&self) -> Result<Vec<JsSummary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_summaries().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_tasks
    pub fn list_tasks(&self) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_tasks_across_projects
    pub fn list_tasks_across_projects(&self, filters: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_tasks_across_projects(filters).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_tasks_in_project
    pub fn list_tasks_in_project(&self, project_name: &str, filters: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_tasks_in_project(project_name, filters).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // list_training_data
    pub fn list_training_data(&self) -> Result<Vec<TrainingData, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::list_training_data().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load
    pub fn load(&self, model_name: &str, device: Device) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load(model_name, device).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_additional_model
    pub fn load_additional_model(&self, model_name: &str, model_alias: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_additional_model(model_name, model_alias).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_agent
    pub fn load_agent(&self, agent_id: &str) -> Result<Agent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_agent(agent_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_agent_assignment
    pub fn load_agent_assignment(&self, agent_id: &str, task_id: &str) -> Result<AgentAssignment, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_agent_assignment(agent_id, task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_agents
    pub fn load_agents(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_agents().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_api_key_collection
    pub fn load_api_key_collection(&self) -> Result<ApiKeyCollection, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_api_key_collection().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_api_keys
    pub fn load_api_keys(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_api_keys().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_code_chunk
    pub fn load_code_chunk(&self, chunk_id: &str) -> Result<CodeChunk, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_code_chunk(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_config
    pub fn load_config(&self) -> Result<Config, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_config().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_error
    pub fn load_error(&self, error_id: &str) -> Result<Error, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_error(error_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_extended_data
    pub fn load_extended_data(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_extended_data().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_feeling
    pub fn load_feeling(&self, id: &str) -> Result<Feeling, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_feeling(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_idea
    pub fn load_idea(&self, idea_id: &str) -> Result<Idea, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_idea(idea_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_memory
    pub fn load_memory(&self, memory_id: &str) -> Result<Memory, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_memory(memory_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_project
    pub fn load_project(&self, project_name: &str) -> Result<Project, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_project(project_name).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_project_task_container
    pub fn load_project_task_container(&self, project_name: &str) -> Result<ProjectTaskContainer, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_project_task_container(project_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_project_task_container_by_hash
    pub fn load_project_task_container_by_hash(&self, project_hash: &str) -> Result<ProjectTaskContainer, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_project_task_container_by_hash(project_hash).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_queue_collection
    pub fn load_queue_collection(&self) -> Result<QueueCollection, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_queue_collection().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_task
    pub fn load_task(&self, task_id: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_task(task_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_task_collection
    pub fn load_task_collection(&self, collection_name: &str) -> Result<TaskCollection, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_task_collection(collection_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_tasks
    pub fn load_tasks(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_tasks().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // load_training_data
    pub fn load_training_data(&self, training_data_id: &str) -> Result<TrainingData, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::load_training_data(training_data_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // long_term_percentage
    pub fn long_term_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::long_term_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // low
    pub fn low(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::low(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // mark_chunk_completed
    pub fn mark_chunk_completed(&self, chunk_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::mark_chunk_completed(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // mark_chunk_validated
    pub fn mark_chunk_validated(&self, chunk_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::mark_chunk_validated(chunk_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // mark_reminder_cancelled
    pub fn mark_reminder_cancelled(&self, reminder_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::mark_reminder_cancelled(reminder_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // mark_reminder_completed
    pub fn mark_reminder_completed(&self, reminder_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::mark_reminder_completed(reminder_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // matches
    pub fn matches(&self, public_key: &str, private_key: Option<&str>) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::matches(public_key, private_key.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // max_tokens
    pub fn max_tokens(&self) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::max_tokens().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // meaning
    pub fn meaning(&self, meaning: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::meaning(meaning).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // merge_tags
    pub fn merge_tags(&self, primary_tag_id: &str, duplicate_tag_ids: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::merge_tags(primary_tag_id, duplicate_tag_ids).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // migrate
    pub fn migrate(&self) -> Result<MigrationReport, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::migrate().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // migrate_tasks
    pub fn migrate_tasks(&self, dry_run: Option<bool>, verbose: Option<bool>, force_overwrite: Option<bool>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::migrate_tasks(dry_run.as_ref(), verbose.as_ref(), force_overwrite.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // migrate_to_project_based
    pub fn migrate_to_project_based(&self) -> Result<MigrationReport, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::migrate_to_project_based().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // moment
    pub fn moment(&self, moment: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::moment(moment).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // move_task
    pub fn move_task(&self, id: &str, from_collection: &str, to_collection: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::move_task(id, from_collection, to_collection).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // multi_query_search
    pub fn multi_query_search(&self, queries: Vec<&str>, aggregation: AggregationType, content_types: Option<Vec<TodoziContentType>>, limit: usize) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::multi_query_search(queries, aggregation, content_types.as_ref(), limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // name
    pub fn name(&self, name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::name(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // new_full
    pub fn new_full(&self, user_id: &str, action: &str, time: &str, priority: Priority, parent_project: &str, status: Status, assignee: Option<Assignee>, tags: Vec<String>, dependencies: Vec<String>, context_notes: Option<String>, progress: Option<u8>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::new_full(user_id, action, time, priority, parent_project, status, assignee.as_ref(), tags, dependencies, context_notes.as_deref(), progress.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // new_idea
    pub fn new_idea(&self, idea: Idea) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::new_idea(idea).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // new_memory
    pub fn new_memory(&self, memory: Memory) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::new_memory(memory).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // new_with_hashes
    pub fn new_with_hashes(&self, server_url: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::new_with_hashes(server_url).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // ok
    pub fn ok(&self, body: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::ok(body).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // overdue_percentage
    pub fn overdue_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::overdue_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_agent_assignment_format
    pub fn parse_agent_assignment_format(&self, agent_text: &str) -> Result<AgentAssignment, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_agent_assignment_format(agent_text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_chunking_format
    pub fn parse_chunking_format(&self, chunk_text: &str) -> Result<CodeChunk, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_chunking_format(chunk_text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_dependencies
    pub fn parse_dependencies(&self, deps_str: &str) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_dependencies(deps_str).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_error_format
    pub fn parse_error_format(&self, error_text: &str) -> Result<Error, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_error_format(error_text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_feeling_format
    pub fn parse_feeling_format(&self, feel_text: &str) -> Result<Feeling, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_feeling_format(feel_text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_idea_format
    pub fn parse_idea_format(&self, idea_text: &str) -> Result<Idea, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_idea_format(idea_text).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_memory_format
    pub fn parse_memory_format(&self, memory_text: &str, user_id: &str) -> Result<Memory, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_memory_format(memory_text, user_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_reminder_format
    pub fn parse_reminder_format(&self, reminder_text: &str) -> Result<Reminder, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_reminder_format(reminder_text).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_summary_format
    pub fn parse_summary_format(&self, summary_text: &str) -> Result<Summary, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_summary_format(summary_text).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_tags
    pub fn parse_tags(&self, tags_str: &str) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_tags(tags_str).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_tdz_command
    pub fn parse_tdz_command(&self, text: &str) -> Result<Vec<TdzCommand, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_tdz_command(text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_todozi_format
    pub fn parse_todozi_format(&self, todozi_text: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_todozi_format(todozi_text).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // parse_training_data_format
    pub fn parse_training_data_format(&self, train_text: &str) -> Result<TrainingData, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::parse_training_data_format(train_text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // pending_percentage
    pub fn pending_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::pending_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // plan_task_actions
    pub fn plan_task_actions(&self, goal: &str) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::plan_task_actions(goal).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // plan_tasks
    pub fn plan_tasks(&self, goal: &str, complexity: Option<&str>, timeline: Option<&str>, context: Option<&str>) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::plan_tasks(goal, complexity.as_deref(), timeline.as_deref(), context.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // predict_relevance
    pub fn predict_relevance(&self, _features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::predict_relevance(_features).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // preload_related_embeddings
    pub fn preload_related_embeddings(&self, content_id: &str, depth: usize) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::preload_related_embeddings(content_id, depth).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // prepare_task_content
    pub fn prepare_task_content(&self, task: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::prepare_task_content(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // prioritize_queue_item
    pub fn prioritize_queue_item(&self, item_id: &str, priority: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::prioritize_queue_item(item_id, priority).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // priority
    pub fn priority(&self, priority: SummaryPriority) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::priority(priority).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // private_percentage
    pub fn private_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::private_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_chat
    pub fn process_chat(&self, message: &str, user_id: &str) -> Result<ChatContent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_chat(message, user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_chat_message
    pub fn process_chat_message(&self, message: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_chat_message(message).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_chat_message_extended
    pub fn process_chat_message_extended(&self, message: &str, user_id: &str) -> Result<ChatContent, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_chat_message_extended(message, user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_chunking_message
    pub fn process_chunking_message(&self, message: &str) -> Result<Vec<CodeChunk, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_chunking_message(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_json_examples
    pub fn process_json_examples(&self, json_data: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_json_examples(json_data).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_tdz_commands
    pub fn process_tdz_commands(&self, text: &str, base_url: &str, api_key: Option<&str>) -> Result<Vec<serde_json::Value, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_tdz_commands(text, base_url, api_key.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // process_workflow
    pub fn process_workflow(&self, tasks: Vec<Task>) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::process_workflow(tasks).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // profile_search_performance
    pub fn profile_search_performance(&self, query: &str, iterations: usize) -> Result<PerformanceMetrics, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::profile_search_performance(query, iterations).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // project_name
    pub fn project_name(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::project_name().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // project_tasks
    pub fn project_tasks(&self, project_name: &str) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // public_percentage
    pub fn public_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::public_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_active
    pub fn queue_active(&self) -> Result<Vec<JsQueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_active().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_add
    pub fn queue_add(&self, task_name: &str, description: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_add(task_name, description).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_backlog
    pub fn queue_backlog(&self) -> Result<Vec<JsQueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_backlog().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_bulk_operations
    pub fn queue_bulk_operations(&self, operations: Vec<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_bulk_operations(operations).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_complete
    pub fn queue_complete(&self, session_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_complete(session_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_list
    pub fn queue_list(&self) -> Result<Vec<JsQueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_list().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // queue_start
    pub fn queue_start(&self, item_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::queue_start(item_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // quick
    pub fn quick(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::quick().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // quick_task
    pub fn quick_task(&self, action: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::quick_task(action).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // reason
    pub fn reason(&self, reason: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::reason(reason).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // recommend_similar
    pub fn recommend_similar(&self, based_on: Vec<String>, exclude: Vec<String>, limit: usize) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::recommend_similar(based_on, exclude, limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // register_tool
    pub fn register_tool(&self, tool_def: JsToolDefinition) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::register_tool(tool_def).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // register_with_server
    pub fn register_with_server(&self, server_url: &str) -> Result<RegistrationInfo, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::register_with_server(server_url).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // relationships_per_tag
    pub fn relationships_per_tag(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::relationships_per_tag().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remember
    pub fn remember(&self, moment: &str, meaning: &str) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::remember(moment, meaning).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remind_at
    pub fn remind_at(&self, remind_at: DateTime<Utc>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::remind_at(remind_at).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remove_api_key
    pub fn remove_api_key(&self, user_id: &str) -> Result<JsApiKey, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::remove_api_key(user_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remove_from_task
    pub fn remove_from_task(&self, task_id: &str, tag: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tags::remove_from_task(task_id, tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remove_item
    pub fn remove_item(&self, id: &str) -> Result<Option<QueueItem, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::remove_item(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remove_key
    pub fn remove_key(&self, user_id: &str) -> Result<Option<ApiKey, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::remove_key(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remove_tag_from_task
    pub fn remove_tag_from_task(&self, task_id: &str, tag: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::remove_tag_from_task(task_id, tag).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // remove_task
    pub fn remove_task(&self, id: &str) -> Result<Option<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::remove_task(id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // render
    pub fn render(&self, config: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::render(config).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // render_compact
    pub fn render_compact(&self, config: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::render_compact(config).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // render_detailed
    pub fn render_detailed(&self, config: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::render_detailed(config).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // reorder_queue
    pub fn reorder_queue(&self, item_ids: Vec<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::reorder_queue(item_ids).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // resolve_error
    pub fn resolve_error(&self, error_id: &str, resolution: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::resolve_error(error_id, resolution).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // restore_backup
    pub fn restore_backup(&self, backup_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::restore_backup(backup_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // restore_embeddings
    pub fn restore_embeddings(&self, backup_path: &str) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::restore_embeddings(backup_path).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // run
    pub fn run(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::run().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // run_interactive
    pub fn run_interactive(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::run_interactive().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // sample_task
    pub fn sample_task(&self) -> Result<Task, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::sample_task().await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_agent
    pub fn save_agent(&self, agent: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_agent(agent).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_agent_assignment
    pub fn save_agent_assignment(&self, assignment: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_agent_assignment(assignment).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_api_key_collection
    pub fn save_api_key_collection(&self, collection: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_api_key_collection(collection).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_as_default
    pub fn save_as_default(&self, model_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_as_default(model_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_code_chunk
    pub fn save_code_chunk(&self, chunk: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_code_chunk(chunk).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_config
    pub fn save_config(&self, config: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_config(config).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_error
    pub fn save_error(&self, error: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_error(error).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_feeling
    pub fn save_feeling(&self, feeling: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_feeling(feeling).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_idea
    pub fn save_idea(&self, idea: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_idea(idea).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_memory
    pub fn save_memory(&self, memory: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_memory(memory).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_processed_content
    pub fn save_processed_content(&self, raw: &str, cleaned: &str, session_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_processed_content(raw, cleaned, session_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_project
    pub fn save_project(&self, project: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_project(project).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_project_task_container
    pub fn save_project_task_container(&self, container: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_project_task_container(container).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_queue_collection
    pub fn save_queue_collection(&self, collection: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_queue_collection(collection).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_search_query
    pub fn save_search_query(&self, query: &str, name: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_search_query(query, name.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_task
    pub fn save_task(&self, task: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_task(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_task_collection
    pub fn save_task_collection(&self, collection_name: &str, collection: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_task_collection(collection_name, collection).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_task_with_embedding
    pub fn save_task_with_embedding(&self, task: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_task_with_embedding(task).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_tasks
    pub fn save_tasks(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_tasks().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // save_training_data
    pub fn save_training_data(&self, training_data: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::save_training_data(training_data).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search
    pub fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResults, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search(query, options).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_ideas
    pub fn search_ideas(&self, query: &str) -> Result<Vec<&Idea, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_ideas(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_memories
    pub fn search_memories(&self, query: &str) -> Result<Vec<&Memory, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_memories(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_reminders
    pub fn search_reminders(&self, query: &str) -> Result<Vec<JsReminder, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_reminders(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_summaries
    pub fn search_summaries(&self, query: &str) -> Result<Vec<&Summary, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_summaries(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_tags
    pub fn search_tags(&self, query: &str) -> Result<Vec<&Tag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_tags(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_tasks
    pub fn search_tasks(&self, query: &str, semantic: bool, limit: Option<usize>) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_tasks(query, semantic, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_tasks_semantic
    pub fn search_tasks_semantic(&self, query: &str, max_results: usize) -> Result<Vec<SemanticSearchResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_tasks_semantic(query, max_results).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // search_with_filters
    pub fn search_with_filters(&self, filters: TaskFilters, limit: Option<usize>) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::search_with_filters(filters, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // see_all
    pub fn see_all(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Easy::see_all().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // semantic_search
    pub fn semantic_search(&self, query: &str, content_types: Option<Vec<TodoziContentType>>, limit: Option<usize>) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::semantic_search(query, content_types.as_ref(), limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // serialization
    pub fn serialization(&self, message: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::serialization(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // set_color_scheme
    pub fn set_color_scheme(&self, scheme_json: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::set_color_scheme(scheme_json).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // share
    pub fn share(&self, share: ShareLevel) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::share(share).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // short_term_percentage
    pub fn short_term_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::short_term_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // show_loading_screen
    pub fn show_loading_screen(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::show_loading_screen().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // similar
    pub fn similar(&self, query: &str) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::similar(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // similar_tasks
    pub fn similar_tasks(&self, task_id: &str) -> Result<Vec<SimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::similar_tasks(task_id).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // similar_tasks_emb
    pub fn similar_tasks_emb(&self, query: &str) -> Result<Vec<JsSimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::similar_tasks_emb(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // similarity_threshold_search
    pub fn similarity_threshold_search(&self, query: &str, threshold: f64, limit: Option<u32>) -> Result<Vec<JsSimilarityResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::similarity_threshold_search(query, threshold, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // smart
    pub fn smart(&self, query: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::smart(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // smart_search
    pub fn smart_search(&self, query: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::smart_search(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // specializations
    pub fn specializations(&self, specializations: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::specializations(specializations).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start
    pub fn start(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::start(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_edit
    pub fn start_edit(&self, _task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_edit(_task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_edit_session
    pub fn start_edit_session(&self, task_id: &str) -> Result<EditSession, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_edit_session(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_local_server
    pub fn start_local_server(&self, host: Option<String>, port: Option<u32>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_local_server(host.as_deref(), port.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_queue_session
    pub fn start_queue_session(&self, queue_item_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_queue_session(queue_item_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_server
    pub fn start_server(&self, host: Option<String>, port: Option<u16>) -> Result<std::result::Result<(, E), Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_server(host.as_deref(), port.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_session
    pub fn start_session(&self, queue_item_id: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_session(queue_item_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // start_task
    pub fn start_task(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::start_task(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // stats
    pub fn stats(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Emb::stats().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // status
    pub fn status(&self, status: AgentStatus) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::status(status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // stop_server
    pub fn stop_server(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::stop_server().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage
    pub fn storage(&self) -> Result<Storage, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_check_folder_structure
    pub fn storage_check_folder_structure(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_check_folder_structure().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_clear_registration
    pub fn storage_clear_registration(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_clear_registration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_create_backup
    pub fn storage_create_backup(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_create_backup().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_delete_project_by_name
    pub fn storage_delete_project_by_name(&self, name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_delete_project_by_name(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_delete_task_from_project
    pub fn storage_delete_task_from_project(&self, task_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_delete_task_from_project(task_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_ensure_folder_structure
    pub fn storage_ensure_folder_structure(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_ensure_folder_structure().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_ai_tasks
    pub fn storage_get_ai_tasks(&self) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_all_active_tasks
    pub fn storage_get_all_active_tasks(&self) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_all_completed_tasks
    pub fn storage_get_all_completed_tasks(&self) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_collaborative_tasks
    pub fn storage_get_collaborative_tasks(&self) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_human_tasks
    pub fn storage_get_human_tasks(&self) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_project_stats
    pub fn storage_get_project_stats(&self, project_name: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_project_stats(project_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_registration_info
    pub fn storage_get_registration_info(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_registration_info().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_storage_dir
    pub fn storage_get_storage_dir(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_storage_dir().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_get_task_from_any_project
    pub fn storage_get_task_from_any_project(&self, task_id: &str) -> Result<JsTask, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_get_task_from_any_project(task_id).await.map(|item| item.to_php())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_init
    pub fn storage_init(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_init().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_is_registered
    pub fn storage_is_registered(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_is_registered().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_list_backups
    pub fn storage_list_backups(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_list_backups().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_list_projects
    pub fn storage_list_projects(&self) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_list_projects().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_load_config
    pub fn storage_load_config(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_load_config().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_load_project
    pub fn storage_load_project(&self, name: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_load_project(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_load_task_collection
    pub fn storage_load_task_collection(&self, name: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_load_task_collection(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_register_with_server
    pub fn storage_register_with_server(&self, server_url: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_register_with_server(server_url).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_restore_backup
    pub fn storage_restore_backup(&self, backup_name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_restore_backup(backup_name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_save_config
    pub fn storage_save_config(&self) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_save_config().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_save_project
    pub fn storage_save_project(&self, name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_save_project(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_save_task_collection
    pub fn storage_save_task_collection(&self, name: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_save_task_collection(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // storage_search_tasks_semantic
    pub fn storage_search_tasks_semantic(&self, query: &str, max_results: Option<u32>) -> Result<Vec<JsTask, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::storage_search_tasks_semantic(query, max_results.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // strategy_content
    pub fn strategy_content(&self, content: Option<String>, file_path: Option<String>, output_format: &str, human: bool) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::strategy_content(content.as_deref(), file_path.as_deref(), output_format, human).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // strategy_content_analysis
    pub fn strategy_content_analysis(&self, text: &str, format: Option<String>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::strategy_content_analysis(text, format.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // strike_cluster
    pub fn strike_cluster(&self, _embedding: &str) -> Result<std::result::Result<i32, Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::strike_cluster(_embedding).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // strike_tags
    pub fn strike_tags(&self, _features: &str) -> Result<std::result::Result<Vec<f32, E>, Box<dyn std::error::Error>>, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::strike_tags(_features).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // success
    pub fn success(&self, output: &str, execution_time_ms: u64) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::success(output, execution_time_ms).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // suggest_tags
    pub fn suggest_tags(&self, content_id: &str, top_k: usize) -> Result<Vec<String, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::suggest_tags(content_id, top_k).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tags
    pub fn tags(&self, tags: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tags(tags).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tags_advanced_search
    pub fn tags_advanced_search(&self, query: &str) -> Result<Vec<JsTag, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tags_advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // task
    pub fn task(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::task(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tasks
    pub fn tasks(&self, project_name: &str) -> Result<Vec<Task, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Projects::tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tdz_cnt
    pub fn tdz_cnt(&self, content: &str, session_id: Option<&str>) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tdz_cnt(content, session_id.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tdz_execute_command
    pub fn tdz_execute_command(&self, command: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tdz_execute_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tdz_find
    pub fn tdz_find(&self, query: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Find::tdz_find(query).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tdz_parse_command
    pub fn tdz_parse_command(&self, input: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tdz_parse_command(input).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tdzfp
    pub fn tdzfp(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tdzfp().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // team_percentage
    pub fn team_percentage(&self) -> Result<f64, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::team_percentage().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // term
    pub fn term(&self, term: MemoryTerm) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::term(term).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // title
    pub fn title(&self) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::title().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // to_context_string
    pub fn to_context_string(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::to_context_string().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // to_ollama_format
    pub fn to_ollama_format(&self) -> Result<serde_json::Value, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::to_ollama_format().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // to_state_string
    pub fn to_state_string(&self) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::to_state_string().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // todozi_begin
    pub fn todozi_begin(&self) -> Result<Todozi, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::todozi_begin().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // todozi_init
    pub fn todozi_init(&self) -> Result<Todozi, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::todozi_init().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // todozi_init_with_auto_registration
    pub fn todozi_init_with_auto_registration(&self) -> Result<Todozi, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::todozi_init_with_auto_registration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // tool_count
    pub fn tool_count(&self) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::tool_count().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // total_results
    pub fn total_results(&self) -> Result<usize, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::total_results().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // track_embedding_drift
    pub fn track_embedding_drift(&self, content_id: &str, current_text: &str) -> Result<DriftReport, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::track_embedding_drift(content_id, current_text).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // transform_shorthand_tags
    pub fn transform_shorthand_tags(&self, message: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::transform_shorthand_tags(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // types
    pub fn types(&self) -> Result<&'static str, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::types().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // unregister
    pub fn unregister(&self, name: &str) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::unregister(name).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update
    pub fn update(&self, updates: TaskUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update(updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_agent
    pub fn update_agent(&self, agent_id: &str, updates: AgentUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_agent(agent_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_agent_assignment_status
    pub fn update_agent_assignment_status(&self, agent_id: &str, task_id: &str, status: AssignmentStatus) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_agent_assignment_status(agent_id, task_id, status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_agent_status
    pub fn update_agent_status(&self, agent_id: &str, status: AgentStatus) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_agent_status(agent_id, status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_chunk_code
    pub fn update_chunk_code(&self, chunk_id: &str, code: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_chunk_code(chunk_id, code).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_chunk_tests
    pub fn update_chunk_tests(&self, chunk_id: &str, tests: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_chunk_tests(chunk_id, tests).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_config
    pub fn update_config(&self, config: Config) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_config(config).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_config_with_registration
    pub fn update_config_with_registration(&self, registration: RegistrationInfo) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_config_with_registration(registration).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_feeling
    pub fn update_feeling(&self, feeling: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_feeling(feeling).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_idea
    pub fn update_idea(&self, idea_id: &str, updates: IdeaUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_idea(idea_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_memory
    pub fn update_memory(&self, memory_id: &str, updates: MemoryUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_memory(memory_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_project
    pub fn update_project(&self, project: Project) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_project(project).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_registration_api_key
    pub fn update_registration_api_key(&self, api_key: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_registration_api_key(api_key).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_registration_keys
    pub fn update_registration_keys(&self, api_key: &str, user_id: Option<String>, fingerprint: Option<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_registration_keys(api_key, user_id.as_deref(), fingerprint.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_reminder
    pub fn update_reminder(&self, reminder_id: &str, updates: ReminderUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_reminder(reminder_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_summary
    pub fn update_summary(&self, summary_id: &str, updates: SummaryUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_summary(summary_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_tag
    pub fn update_tag(&self, tag_id: &str, updates: TagUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_tag(tag_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_task
    pub fn update_task(&self, id: &str, updates: TaskUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_task(id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_task_full
    pub fn update_task_full(&self, task_id: &str, updates: TaskUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_task_full(task_id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_task_in_project
    pub fn update_task_in_project(&self, id: &str, updates: crate::models::TaskUpdate) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_task_in_project(id, updates).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // update_task_status
    pub fn update_task_status(&self, task_id: &str, status: Status) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::update_task_status(task_id, status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // urgent
    pub fn urgent(&self, action: &str) -> Result<String, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Tdz::urgent(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validate_embeddings
    pub fn validate_embeddings(&self) -> Result<ValidationReport, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validate_embeddings().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validate_migration
    pub fn validate_migration(&self) -> Result<bool, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validate_migration().await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validate_required_params
    pub fn validate_required_params(&self, kwargs: HashMap<String, serde_json::Value>, required_params: &[String]) -> Result<Option<ToolResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validate_required_params(kwargs, required_params).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validate_string_param
    pub fn validate_string_param(&self, value: &str, param_name: &str, min_length: usize, max_length: usize, pattern: Option<String>) -> Result<Option<ToolResult, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validate_string_param(value, param_name, min_length, max_length, pattern.as_deref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validate_task_input
    pub fn validate_task_input(&self, action: &str, time: &str, priority: &str, project: &str, status: &str, assignee: Option<&str>, progress: Option<u8>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validate_task_input(action, time, priority, project, status, assignee.as_deref(), progress.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validate_tdz_command
    pub fn validate_tdz_command(&self, command: &str) -> Result<JsCommandValidation, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validate_tdz_command(command).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // validation
    pub fn validation(&self, message: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::validation(message).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // verbose
    pub fn verbose(&self, verbose: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::verbose(verbose).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_action
    pub fn with_action(&self, action: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_action(action).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_assignee
    pub fn with_assignee(&self, assignee: Assignee) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_assignee(assignee).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_context
    pub fn with_context(&self, context: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_context(context).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_context_notes
    pub fn with_context_notes(&self, context_notes: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_context_notes(context_notes).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_dependencies
    pub fn with_dependencies(&self, dependencies: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_dependencies(dependencies).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_dry_run
    pub fn with_dry_run(&self, dry_run: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_dry_run(dry_run).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_embedding_service
    pub fn with_embedding_service(&self, service: TodoziEmbeddingService) -> Result<PyResult<Self, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_embedding_service(service).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_embedding_service_option
    pub fn with_embedding_service_option(&self, service: Option<TodoziEmbeddingService>) -> Result<PyResult<Self, E>> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_embedding_service_option(service.as_ref()).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_force
    pub fn with_force(&self, force: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_force(force).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_max_tokens
    pub fn with_max_tokens(&self, max_tokens: u32) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_max_tokens(max_tokens).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_parent_project
    pub fn with_parent_project(&self, parent_project: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_parent_project(parent_project).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_priority
    pub fn with_priority(&self, priority: Priority) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_priority(priority).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_progress
    pub fn with_progress(&self, progress: u8) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_progress(progress).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_shared_components
    pub fn with_shared_components(&self, config: Arc<Mutex<TodoziEmbeddingConfig>>, cache: Arc<Mutex<HashMap<String, TodoziEmbeddingCache>>>, embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>, embedding_models: Arc<Mutex<HashMap<String, Arc<EmbeddingModel>>>>, tag_manager: Arc<Mutex<TagManager>>, storage: Arc<Mutex<Storage>>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_shared_components(config, cache, embedding_model, embedding_models, tag_manager, storage).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_status
    pub fn with_status(&self, status: Status) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_status(status).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_tags
    pub fn with_tags(&self, tags: Vec<String>) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_tags(tags).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_temperature
    pub fn with_temperature(&self, temperature: f32) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_temperature(temperature).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_time
    pub fn with_time(&self, time: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_time(time).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_user_id
    pub fn with_user_id(&self, user_id: &str) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_user_id(user_id).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
    // with_verbose
    pub fn with_verbose(&self, verbose: bool) -> Result<Self, E> {
        let runtime = get_runtime().map_err(|e| php_error(E_ALL, &e.to_string()))?;
        runtime.block_on(async {
            Done::with_verbose(verbose).await
                .map_err(|e| php_error(E_ALL, e.to_string()))
        })
    }
    
}

// Standalone PHP functions (alternative to class methods)
// activate_api_key
#[php_function]
pub fn todozi_activate_api_key(user_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::activate_api_key(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// activate_key
#[php_function]
pub fn todozi_activate_key(user_id: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::activate_key(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// activate_reminder
#[php_function]
pub fn todozi_activate_reminder(reminder_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::activate_reminder(reminder_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// active
#[php_function]
pub fn todozi_active() -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::active().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// active_percentage
#[php_function]
pub fn todozi_active_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::active_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add
#[php_function]
pub fn todozi_add(task_name: &str, description: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add(task_name, description).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_checklist_item
#[php_function]
pub fn todozi_add_checklist_item(item: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_checklist_item(item).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_chunk
#[php_function]
pub fn todozi_add_chunk(chunk_id: &str, level: &str, deps: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_chunk(chunk_id, level, deps).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_completed_module
#[php_function]
pub fn todozi_add_completed_module(module: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_completed_module(module).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_dependency
#[php_function]
pub fn todozi_add_dependency(dep: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_dependency(dep).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_error_pattern
#[php_function]
pub fn todozi_add_error_pattern(pattern: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_error_pattern(pattern).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_function_signature
#[php_function]
pub fn todozi_add_function_signature(name: &str, signature: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_function_signature(name, signature).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_import
#[php_function]
pub fn todozi_add_import(import_stmt: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_import(import_stmt).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_item
#[php_function]
pub fn todozi_add_item(content: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_item(content).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_key
#[php_function]
pub fn todozi_add_key(key: JsApiKey) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_key(key).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_pending_module
#[php_function]
pub fn todozi_add_pending_module(module: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_pending_module(module).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_processed_action
#[php_function]
pub fn todozi_add_processed_action(action_type: &str, description: &str, success: bool, result: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_processed_action(action_type, description, success, result.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_queue_item
#[php_function]
pub fn todozi_add_queue_item(item: QueueItem) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_queue_item(item).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_recent
#[php_function]
pub fn todozi_add_recent(description: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_recent(description).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_recent_action
#[php_function]
pub fn todozi_add_recent_action(action: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_recent_action(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_tag_relationship
#[php_function]
pub fn todozi_add_tag_relationship(tag_id: &str, related_tag_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_tag_relationship(tag_id, related_tag_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_tag_to_task
#[php_function]
pub fn todozi_add_tag_to_task(task_id: &str, tag: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_tag_to_task(task_id, tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_task
#[php_function]
pub fn todozi_add_task(task: Task) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_task(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_task_emb
#[php_function]
pub fn todozi_add_task_emb(task: JsTask) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_task_emb(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_task_to_project
#[php_function]
pub fn todozi_add_task_to_project(task: Task) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_task_to_project(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// add_to_task
#[php_function]
pub fn todozi_add_to_task(task_id: &str, tag: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::add_to_task(task_id, tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// advanced_search
#[php_function]
pub fn todozi_advanced_search(query: &str) -> Result<Vec<Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// advanced_search_with_filters
#[php_function]
pub fn todozi_advanced_search_with_filters(query: &str, data_types: Option<Vec<String>>, limit: Option<u32>) -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::advanced_search_with_filters(query, data_types.as_ref(), limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ai
#[php_function]
pub fn todozi_ai(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::ai(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ai_find
#[php_function]
pub fn todozi_ai_find(query: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::ai_find(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ai_search
#[php_function]
pub fn todozi_ai_search(query: &str) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::ai_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ai_task
#[php_function]
pub fn todozi_ai_task(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::ai_task(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ai_tasks
#[php_function]
pub fn todozi_ai_tasks(query: &str) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::ai_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// all
#[php_function]
pub fn todozi_all() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::all().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// all_tasks
#[php_function]
pub fn todozi_all_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::all_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// analyze_code_quality
#[php_function]
pub fn todozi_analyze_code_quality(_features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::analyze_code_quality(_features).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// api
#[php_function]
pub fn todozi_api(message: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::api(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// api_key
#[php_function]
pub fn todozi_api_key() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::api_key().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// archive_project
#[php_function]
pub fn todozi_archive_project(name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::archive_project(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// as_str
#[php_function]
pub fn todozi_as_str() -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::as_str().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// assign_task_to_agent
#[php_function]
pub fn todozi_assign_task_to_agent(task_id: &str, agent_id: &str, project_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::assign_task_to_agent(task_id, agent_id, project_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// auto_categorize_content
#[php_function]
pub fn todozi_auto_categorize_content(text: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::auto_categorize_content(text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// auto_label_clusters
#[php_function]
pub fn todozi_auto_label_clusters(clusters: Vec<ClusteringResult>) -> Result<Vec<LabeledCluster, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::auto_label_clusters(clusters).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// backlog
#[php_function]
pub fn todozi_backlog() -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::backlog().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// backup_embeddings
#[php_function]
pub fn todozi_backup_embeddings(backup_path: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::backup_embeddings(backup_path.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// batch_embed_content
#[php_function]
pub fn todozi_batch_embed_content(content_list: Vec<String>) -> Result<Vec<Vec<f64, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::batch_embed_content(content_list).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// begin
#[php_function]
pub fn todozi_begin(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Actions::begin(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// breakthrough
#[php_function]
pub fn todozi_breakthrough(idea: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::breakthrough(idea).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// breakthrough_idea
#[php_function]
pub fn todozi_breakthrough_idea(idea: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::breakthrough_idea(idea).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// breakthrough_percentage
#[php_function]
pub fn todozi_breakthrough_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::breakthrough_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// build_similarity_graph
#[php_function]
pub fn todozi_build_similarity_graph(threshold: f32) -> Result<SimilarityGraph, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::build_similarity_graph(threshold).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// bulk_create_tags
#[php_function]
pub fn todozi_bulk_create_tags(tag_names: Vec<String>, category: Option<String>) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::bulk_create_tags(tag_names, category.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// calculate_diversity
#[php_function]
pub fn todozi_calculate_diversity(content_ids: Vec<String>) -> Result<f32, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::calculate_diversity(content_ids).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// capabilities
#[php_function]
pub fn todozi_capabilities(capabilities: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::capabilities(capabilities).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// category
#[php_function]
pub fn todozi_category(category: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::category(category).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// chat
#[php_function]
pub fn todozi_chat(message: &str) -> Result<ChatContent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::chat(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// check_api_key_auth
#[php_function]
pub fn todozi_check_api_key_auth(public_key: &str, private_key: Option<String>) -> Result<JsApiKeyAuth, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::check_api_key_auth(public_key, private_key.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// check_folder_structure
#[php_function]
pub fn todozi_check_folder_structure() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::check_folder_structure().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// check_migration_status
#[php_function]
pub fn todozi_check_migration_status() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::check_migration_status().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cleanup_expired
#[php_function]
pub fn todozi_cleanup_expired() -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cleanup_expired().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cleanup_legacy
#[php_function]
pub fn todozi_cleanup_legacy() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cleanup_legacy().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// clear_registration
#[php_function]
pub fn todozi_clear_registration() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::clear_registration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_add_task
#[php_function]
pub fn todozi_cli_add_task(content: &str, priority: Option<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_add_task(content, priority.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_chat
#[php_function]
pub fn todozi_cli_chat(message: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_chat(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_clear_registration
#[php_function]
pub fn todozi_cli_clear_registration() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_clear_registration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_complete_task
#[php_function]
pub fn todozi_cli_complete_task(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_complete_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_create_backup
#[php_function]
pub fn todozi_cli_create_backup() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_create_backup().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_create_idea
#[php_function]
pub fn todozi_cli_create_idea(content: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_create_idea(content).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_create_memory
#[php_function]
pub fn todozi_cli_create_memory(moment: &str, meaning: &str, reason: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_create_memory(moment, meaning, reason).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_delete_task
#[php_function]
pub fn todozi_cli_delete_task(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_delete_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_fix_consistency
#[php_function]
pub fn todozi_cli_fix_consistency() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_fix_consistency().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_get_registration_status
#[php_function]
pub fn todozi_cli_get_registration_status() -> Result<Option<JsRegistrationInfo, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_get_registration_status().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_list_backups
#[php_function]
pub fn todozi_cli_list_backups() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_list_backups().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_list_tasks
#[php_function]
pub fn todozi_cli_list_tasks() -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_list_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_register_with_server
#[php_function]
pub fn todozi_cli_register_with_server(server_url: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_register_with_server(server_url).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_restore_backup
#[php_function]
pub fn todozi_cli_restore_backup(backup_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_restore_backup(backup_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_search_tasks
#[php_function]
pub fn todozi_cli_search_tasks(query: &str) -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_search_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_show_task
#[php_function]
pub fn todozi_cli_show_task(task_id: &str) -> Result<JsTask, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_show_task(task_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cli_update_task
#[php_function]
pub fn todozi_cli_update_task(task_id: &str, action: Option<String>, priority: Option<String>, status: Option<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_update_task(task_id, action.as_deref(), priority.as_deref(), status.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cluster
#[php_function]
pub fn todozi_cluster() -> Result<Vec<ClusteringResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cluster().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cluster_content
#[php_function]
pub fn todozi_cluster_content() -> Result<Vec<ClusteringResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cluster_content().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// cluster_similar_tasks
#[php_function]
pub fn todozi_cluster_similar_tasks() -> Result<Vec<JsClusteringResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cluster_similar_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// collab
#[php_function]
pub fn todozi_collab(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::collab(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// collab_task
#[php_function]
pub fn todozi_collab_task(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::collab_task(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// color
#[php_function]
pub fn todozi_color(color: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::color(color).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// compare_models
#[php_function]
pub fn todozi_compare_models(text: &str, model_aliases: Vec<String>) -> Result<ModelComparisonResult, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::compare_models(text, model_aliases).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// complete
#[php_function]
pub fn todozi_complete(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// complete_agent_assignment
#[php_function]
pub fn todozi_complete_agent_assignment(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete_agent_assignment(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// complete_task
#[php_function]
pub fn todozi_complete_task(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// complete_task_in_project
#[php_function]
pub fn todozi_complete_task_in_project(id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete_task_in_project(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// completion_rate
#[php_function]
pub fn todozi_completion_rate() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::completion_rate().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// config
#[php_function]
pub fn todozi_config() -> Result<&Config, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::config().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// configure_display
#[php_function]
pub fn todozi_configure_display(config_json: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::configure_display(config_json).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// configure_server
#[php_function]
pub fn todozi_configure_server(host: Option<String>, port: Option<u32>, max_connections: Option<u32>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::configure_server(host.as_deref(), port.as_ref(), max_connections.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// content
#[php_function]
pub fn todozi_content(content: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::content(content).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// context
#[php_function]
pub fn todozi_context(context: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::context(context).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// craft_embedding
#[php_function]
pub fn todozi_craft_embedding(_features: &str) -> Result<std::result::Result<Vec<f32, E>, Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::craft_embedding(_features).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create
#[php_function]
pub fn todozi_create(name: &str, description: Option<&str>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create(name, description.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_advanced_todozi_tools
#[php_function]
pub fn todozi_create_advanced_todozi_tools(todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_advanced_todozi_tools(todozi).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_agent
#[php_function]
pub fn todozi_create_agent(agent: Agent) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_agent(agent).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_api_key
#[php_function]
pub fn todozi_create_api_key() -> Result<JsApiKey, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_api_key().await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_api_key_with_user_id
#[php_function]
pub fn todozi_create_api_key_with_user_id(user_id: &str) -> Result<JsApiKey, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_api_key_with_user_id(user_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_architect_agent
#[php_function]
pub fn todozi_create_architect_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_architect_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_backup
#[php_function]
pub fn todozi_create_backup() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_backup().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_coder
#[php_function]
pub fn todozi_create_coder() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_coder().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_comrad_agent
#[php_function]
pub fn todozi_create_comrad_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_comrad_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_custom_agent
#[php_function]
pub fn todozi_create_custom_agent(id: &str, name: &str, description: &str, capabilities: Vec<String>, specializations: Vec<String>, category: &str, author: Option<String>) -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_custom_agent(id, name, description, capabilities, specializations, category, author.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_default_agents
#[php_function]
pub fn todozi_create_default_agents() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_default_agents().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_designer_agent
#[php_function]
pub fn todozi_create_designer_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_designer_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_detective_agent
#[php_function]
pub fn todozi_create_detective_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_detective_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_devops_agent
#[php_function]
pub fn todozi_create_devops_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_devops_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_embedding_service
#[php_function]
pub fn todozi_create_embedding_service() -> Result<TodoziEmbeddingService, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_embedding_service().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_embedding_version
#[php_function]
pub fn todozi_create_embedding_version(content_id: &str, version_label: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_embedding_version(content_id, version_label).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_error
#[php_function]
pub fn todozi_create_error(error: Error) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_error(error).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_error_result
#[php_function]
pub fn todozi_create_error_result(error_msg: &str, execution_time_ms: u64, error_type: ErrorType, metadata: Option<HashMap<String, serde_json::Value>>) -> Result<ToolResult, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_error_result(error_msg, execution_time_ms, error_type, metadata.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_filters
#[php_function]
pub fn todozi_create_filters() -> Result<TaskFilters, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_filters().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_finisher_agent
#[php_function]
pub fn todozi_create_finisher_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_finisher_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_framer_agent
#[php_function]
pub fn todozi_create_framer_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_framer_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_friend_agent
#[php_function]
pub fn todozi_create_friend_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_friend_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_grok_level_todozi_tools
#[php_function]
pub fn todozi_create_grok_level_todozi_tools(todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_grok_level_todozi_tools(todozi).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_hoarder_agent
#[php_function]
pub fn todozi_create_hoarder_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_hoarder_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_idea
#[php_function]
pub fn todozi_create_idea(idea: Idea) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_idea(idea).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_investigator_agent
#[php_function]
pub fn todozi_create_investigator_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_investigator_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_mason_agent
#[php_function]
pub fn todozi_create_mason_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_mason_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_memory
#[php_function]
pub fn todozi_create_memory(memory: Memory) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_memory(memory).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_nerd_agent
#[php_function]
pub fn todozi_create_nerd_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_nerd_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_nun_agent
#[php_function]
pub fn todozi_create_nun_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_nun_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_overlord_agent
#[php_function]
pub fn todozi_create_overlord_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_overlord_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_party_agent
#[php_function]
pub fn todozi_create_party_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_party_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_planner_agent
#[php_function]
pub fn todozi_create_planner_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_planner_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_project
#[php_function]
pub fn todozi_create_project(name: &str, description: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::create_project(name, description.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_recycler_agent
#[php_function]
pub fn todozi_create_recycler_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_recycler_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_reminder
#[php_function]
pub fn todozi_create_reminder(reminder: Reminder) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_reminder(reminder).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_search_options
#[php_function]
pub fn todozi_create_search_options(data_types: Option<Vec<String>>, limit: Option<u32>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_search_options(data_types.as_ref(), limit.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_skeleton_agent
#[php_function]
pub fn todozi_create_skeleton_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_skeleton_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_snitch_agent
#[php_function]
pub fn todozi_create_snitch_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_snitch_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_storage
#[php_function]
pub fn todozi_create_storage() -> Result<Storage, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_storage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_success_result
#[php_function]
pub fn todozi_create_success_result(output: &str, execution_time_ms: u64, metadata: Option<HashMap<String, serde_json::Value>>) -> Result<ToolResult, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_success_result(output, execution_time_ms, metadata.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_summary
#[php_function]
pub fn todozi_create_summary(summary: Summary) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_summary(summary).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tag
#[php_function]
pub fn todozi_create_tag(tag: Tag) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::create_tag(tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_task
#[php_function]
pub fn todozi_create_task(action: &str, priority: Option<Priority>, project: Option<&str>, time: Option<&str>, context: Option<&str>) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_task(action, priority.as_ref(), project.as_deref(), time.as_deref(), context.as_deref()).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_task_filters
#[php_function]
pub fn todozi_create_task_filters(project: Option<String>, status: Option<String>, priority: Option<String>, assignee: Option<String>, tags: Option<String>, search: Option<String>) -> Result<TaskFilters, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_task_filters(project.as_deref(), status.as_deref(), priority.as_deref(), assignee.as_deref(), tags.as_deref(), search.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tdz_content_processor_tool
#[php_function]
pub fn todozi_create_tdz_content_processor_tool(state: SharedTodoziState) -> Result<Box<dyn Tool + Send + Sync, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tdz_content_processor_tool(state).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tdz_tool
#[php_function]
pub fn todozi_create_tdz_tool() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tdz_tool().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tester_agent
#[php_function]
pub fn todozi_create_tester_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tester_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_todozi_tools
#[php_function]
pub fn todozi_create_todozi_tools(todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_todozi_tools(todozi).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_todozi_tools_with_embedding
#[php_function]
pub fn todozi_create_todozi_tools_with_embedding(todozi: SharedTodozi, embedding_service: Option<TodoziEmbeddingService>) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync, E>>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_todozi_tools_with_embedding(todozi, embedding_service.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tool_definition
#[php_function]
pub fn todozi_create_tool_definition(name: &str, description: &str, category: &str, parameters: Vec<JsToolParameter>) -> Result<JsToolDefinition, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_definition(name, description, category, parameters).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tool_definition_with_locks
#[php_function]
pub fn todozi_create_tool_definition_with_locks(name: &str, description: &str, category: &str, parameters: Vec<ToolParameter>, resource_locks: Vec<ResourceLock>) -> Result<ToolDefinition, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_definition_with_locks(name, description, category, parameters, resource_locks).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tool_parameter
#[php_function]
pub fn todozi_create_tool_parameter(name: &str, type_: &str, description: &str, required: bool) -> Result<JsToolParameter, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_parameter(name, type_, description, required).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tool_parameter_with_default
#[php_function]
pub fn todozi_create_tool_parameter_with_default(name: &str, type_: &str, description: &str, required: bool, default: &str) -> Result<JsToolParameter, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_parameter_with_default(name, type_, description, required, default).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tui_app
#[php_function]
pub fn todozi_create_tui_app() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tui_app().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_tuner_agent
#[php_function]
pub fn todozi_create_tuner_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tuner_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_update
#[php_function]
pub fn todozi_create_update() -> Result<TaskUpdate, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_update().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// create_writer_agent
#[php_function]
pub fn todozi_create_writer_agent() -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_writer_agent().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// critical_percentage
#[php_function]
pub fn todozi_critical_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::critical_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// deactivate_api_key
#[php_function]
pub fn todozi_deactivate_api_key(user_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deactivate_api_key(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// deactivate_key
#[php_function]
pub fn todozi_deactivate_key(user_id: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deactivate_key(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// deep
#[php_function]
pub fn todozi_deep(query: &str) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deep(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// deep_search
#[php_function]
pub fn todozi_deep_search(query: &str) -> Result<Vec<JsSimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deep_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// default_filters
#[php_function]
pub fn todozi_default_filters() -> Result<TaskFilters, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::default_filters().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// default_update
#[php_function]
pub fn todozi_default_update() -> Result<TaskUpdate, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::default_update().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete
#[php_function]
pub fn todozi_delete(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_agent
#[php_function]
pub fn todozi_delete_agent(agent_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_agent(agent_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_agent_assignment
#[php_function]
pub fn todozi_delete_agent_assignment(agent_id: &str, task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_agent_assignment(agent_id, task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_code_chunk
#[php_function]
pub fn todozi_delete_code_chunk(chunk_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_code_chunk(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_error
#[php_function]
pub fn todozi_delete_error(error_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_error(error_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_feeling
#[php_function]
pub fn todozi_delete_feeling(id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_feeling(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_idea
#[php_function]
pub fn todozi_delete_idea(idea_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_idea(idea_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_memory
#[php_function]
pub fn todozi_delete_memory(memory_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_memory(memory_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_project
#[php_function]
pub fn todozi_delete_project(project_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_project(project_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_project_task_container
#[php_function]
pub fn todozi_delete_project_task_container(project_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_project_task_container(project_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_reminder
#[php_function]
pub fn todozi_delete_reminder(reminder_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_reminder(reminder_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_summary
#[php_function]
pub fn todozi_delete_summary(summary_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_summary(summary_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_tag
#[php_function]
pub fn todozi_delete_tag(tag_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_tag(tag_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_task
#[php_function]
pub fn todozi_delete_task(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_task_from_project
#[php_function]
pub fn todozi_delete_task_from_project(id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_task_from_project(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// delete_training_data
#[php_function]
pub fn todozi_delete_training_data(training_data_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_training_data(training_data_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// description
#[php_function]
pub fn todozi_description(description: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::description(description).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// detailed
#[php_function]
pub fn todozi_detailed() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::detailed().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// detailed_stats
#[php_function]
pub fn todozi_detailed_stats() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::detailed_stats().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// display_task
#[php_function]
pub fn todozi_display_task(task_id: &str) -> Result<TaskDisplay, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::display_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// display_tasks
#[php_function]
pub fn todozi_display_tasks(task_ids: Vec<String>) -> Result<TaskListDisplay, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::display_tasks(task_ids).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// do_it
#[php_function]
pub fn todozi_do_it(what: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Easy::do_it(what).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done
#[php_function]
pub fn todozi_done(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::done(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_api_key
#[php_function]
pub fn todozi_done_api_key() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_api_key().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_create_embedding_service
#[php_function]
pub fn todozi_done_create_embedding_service() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_create_embedding_service().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_create_storage
#[php_function]
pub fn todozi_done_create_storage() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_create_storage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_default_filters
#[php_function]
pub fn todozi_done_default_filters() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_default_filters().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_default_update
#[php_function]
pub fn todozi_done_default_update() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_default_update().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_embedding_config
#[php_function]
pub fn todozi_done_embedding_config() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_embedding_config().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_embedding_service
#[php_function]
pub fn todozi_done_embedding_service() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_embedding_service().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_init
#[php_function]
pub fn todozi_done_init() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_init().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_process_chat
#[php_function]
pub fn todozi_done_process_chat(message: &str, user_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_process_chat(message, user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_sample_task
#[php_function]
pub fn todozi_done_sample_task() -> Result<JsTask, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_sample_task().await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_storage
#[php_function]
pub fn todozi_done_storage() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_storage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// done_types
#[php_function]
pub fn todozi_done_types() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_types().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// dry_run
#[php_function]
pub fn todozi_dry_run(dry_run: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::dry_run(dry_run).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// easy_done
#[php_function]
pub fn todozi_easy_done(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_done(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// easy_find
#[php_function]
pub fn todozi_easy_find(what: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_find(what).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// easy_idea
#[php_function]
pub fn todozi_easy_idea(what: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_idea(what).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// easy_remember
#[php_function]
pub fn todozi_easy_remember(what: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_remember(what).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embed
#[php_function]
pub fn todozi_embed(text: &str) -> Result<Vec<f32, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed(text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embed_idea
#[php_function]
pub fn todozi_embed_idea(idea: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_idea(idea).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embed_memory
#[php_function]
pub fn todozi_embed_memory(memory: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_memory(memory).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embed_stats
#[php_function]
pub fn todozi_embed_stats() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_stats().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embed_tag
#[php_function]
pub fn todozi_embed_tag(tag: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_tag(tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embed_task
#[php_function]
pub fn todozi_embed_task(task_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Emb::embed_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embedding_config
#[php_function]
pub fn todozi_embedding_config() -> Result<TodoziEmbeddingConfig, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embedding_config().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// embedding_service
#[php_function]
pub fn todozi_embedding_service() -> Result<TodoziEmbeddingService, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embedding_service().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// encode
#[php_function]
pub fn todozi_encode(texts: &str) -> Result<Vec<Vec<f32, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::encode(texts).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// end_queue_session
#[php_function]
pub fn todozi_end_queue_session(session_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::end_queue_session(session_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// end_session
#[php_function]
pub fn todozi_end_session(session_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::end_session(session_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ensure_folder_structure
#[php_function]
pub fn todozi_ensure_folder_structure() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ensure_folder_structure().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ensure_todozi_initialized
#[php_function]
pub fn todozi_ensure_todozi_initialized() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ensure_todozi_initialized().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// error
#[php_function]
pub fn todozi_error(error: &str, execution_time_ms: u64) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::error(error, execution_time_ms).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// example
#[php_function]
pub fn todozi_example() -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::example().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// example_usage
#[php_function]
pub fn todozi_example_usage() -> Result<std::result::Result<(, E), Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::example_usage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// execute_task
#[php_function]
pub fn todozi_execute_task(storage: &str, task: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_task(storage, task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// execute_tdz_command
#[php_function]
pub fn todozi_execute_tdz_command(command: &str, base_url: &str, api_key: Option<&str>) -> Result<Value, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_tdz_command(command, base_url, api_key.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// execute_todozi_tool_delegated
#[php_function]
pub fn todozi_execute_todozi_tool_delegated(params: &str) -> Result<ExecutorResult<ExecutionResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_todozi_tool_delegated(params).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// execute_tool
#[php_function]
pub fn todozi_execute_tool(tool_name: &str, kwargs: HashMap<String, serde_json::Value>) -> Result<ToolResult, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_tool(tool_name, kwargs).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// explain_search_result
#[php_function]
pub fn todozi_explain_search_result(query: &str, result: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::explain_search_result(query, result).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// export_diagnostics
#[php_function]
pub fn todozi_export_diagnostics() -> Result<DiagnosticReport, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_diagnostics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// export_embedded_tasks_hlx
#[php_function]
pub fn todozi_export_embedded_tasks_hlx(output_path: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_embedded_tasks_hlx(output_path).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// export_for_fine_tuning
#[php_function]
pub fn todozi_export_for_fine_tuning(output_path: &str) -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_for_fine_tuning(output_path).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// export_to_format
#[php_function]
pub fn todozi_export_to_format(format: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_to_format(format).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// extract_content
#[php_function]
pub fn todozi_extract_content(content: Option<String>, file_path: Option<String>, output_format: &str, human: bool) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_content(content.as_deref(), file_path.as_deref(), output_format, human).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// extract_content_from_text
#[php_function]
pub fn todozi_extract_content_from_text(text: &str, format: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_content_from_text(text, format.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// extract_task_actions
#[php_function]
pub fn todozi_extract_task_actions(content: &str) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_task_actions(content).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// extract_tasks
#[php_function]
pub fn todozi_extract_tasks(content: &str, context: Option<&str>) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_tasks(content, context.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// fast
#[php_function]
pub fn todozi_fast(query: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fast(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// fast_search
#[php_function]
pub fn todozi_fast_search(query: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fast_search(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// filtered_semantic_search
#[php_function]
pub fn todozi_filtered_semantic_search(query: &str, filters: SearchFilters, limit: usize) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::filtered_semantic_search(query, filters, limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find
#[php_function]
pub fn todozi_find(query: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::find(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_best_agent
#[php_function]
pub fn todozi_find_best_agent(required_specialization: &str, preferred_capability: Option<String>) -> Result<Option<&Agent, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_best_agent(required_specialization, preferred_capability.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_by_tag
#[php_function]
pub fn todozi_find_by_tag(tag_name: &str) -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_by_tag(tag_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_cross_content_relationships
#[php_function]
pub fn todozi_find_cross_content_relationships(content_id: &str, content_type: TodoziContentType, min_similarity: f32) -> Result<HashMap<TodoziContentType, Vec<SimilarityResult, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_cross_content_relationships(content_id, content_type, min_similarity).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_ideas
#[php_function]
pub fn todozi_find_ideas(query: &str) -> Result<Vec<JsIdea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_ideas(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_memories
#[php_function]
pub fn todozi_find_memories(query: &str) -> Result<Vec<JsMemory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_memories(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_outliers
#[php_function]
pub fn todozi_find_outliers(content_type: TodoziContentType, threshold: f32) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_outliers(content_type, threshold).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_similar_tags
#[php_function]
pub fn todozi_find_similar_tags(tag_name: &str, limit: Option<usize>) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_similar_tags(tag_name, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_similar_tasks
#[php_function]
pub fn todozi_find_similar_tasks(task_description: &str, limit: Option<usize>) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_similar_tasks(task_description, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_tasks
#[php_function]
pub fn todozi_find_tasks(query: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_tasks_ai
#[php_function]
pub fn todozi_find_tasks_ai(query: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_tasks_ai(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_tdz
#[php_function]
pub fn todozi_find_tdz(str: Option<&str>) -> Result<Option<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_tdz(str.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// find_todozi
#[php_function]
pub fn todozi_find_todozi(str: Option<String>) -> Result<Option<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_todozi(str.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// fix_completed_tasks_consistency
#[php_function]
pub fn todozi_fix_completed_tasks_consistency() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fix_completed_tasks_consistency().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// fix_task_consistency
#[php_function]
pub fn todozi_fix_task_consistency() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fix_task_consistency().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// force_overwrite
#[php_function]
pub fn todozi_force_overwrite(force_overwrite: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::force_overwrite(force_overwrite).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_duration
#[php_function]
pub fn todozi_format_duration(from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_duration(from, to).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_error_with_context
#[php_function]
pub fn todozi_format_error_with_context(error_message: &str, context: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_error_with_context(error_message, context).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_project_stats
#[php_function]
pub fn todozi_format_project_stats(project_name: &str, task_count: usize, completed_count: usize) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_project_stats(project_name, task_count, completed_count).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_task
#[php_function]
pub fn todozi_format_task(task: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_task(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_task_list
#[php_function]
pub fn todozi_format_task_list(tasks: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_task_list(tasks).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_task_with_emojis
#[php_function]
pub fn todozi_format_task_with_emojis(task: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_task_with_emojis(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// format_time_estimate
#[php_function]
pub fn todozi_format_time_estimate(time: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_time_estimate(time).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// fuzzy_search
#[php_function]
pub fn todozi_fuzzy_search(query: &str, max_distance: usize) -> Result<Vec<(&Tag, usize), E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fuzzy_search(query, max_distance).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// generate_embedding
#[php_function]
pub fn todozi_generate_embedding(text: &str) -> Result<Vec<f32, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::generate_embedding(text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// generate_embeddings_batch
#[php_function]
pub fn todozi_generate_embeddings_batch(texts: Vec<String>) -> Result<Vec<Vec<f32, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::generate_embeddings_batch(texts).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// generate_task_embedding
#[php_function]
pub fn todozi_generate_task_embedding(task: &str) -> Result<Option<Vec<f32, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::generate_task_embedding(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get
#[php_function]
pub fn todozi_get(task_id: &str) -> Result<Option<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_active_items
#[php_function]
pub fn todozi_get_active_items() -> Result<Vec<&QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_items().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_active_keys
#[php_function]
pub fn todozi_get_active_keys() -> Result<Vec<&ApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_keys().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_active_reminders
#[php_function]
pub fn todozi_get_active_reminders() -> Result<Vec<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_active_sessions
#[php_function]
pub fn todozi_get_active_sessions() -> Result<Vec<crate::models::QueueSession, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_sessions().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agent
#[php_function]
pub fn todozi_get_agent(agent_id: &str) -> Result<Option<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent(agent_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agent_assignments
#[php_function]
pub fn todozi_get_agent_assignments(agent_id: &str) -> Result<Vec<&AgentAssignment, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent_assignments(agent_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agent_assignments_dir
#[php_function]
pub fn todozi_get_agent_assignments_dir(agent_id: &str) -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent_assignments_dir(agent_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agent_statistics
#[php_function]
pub fn todozi_get_agent_statistics() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agents_by_capability
#[php_function]
pub fn todozi_get_agents_by_capability(capability: &str) -> Result<Vec<&Agent, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_by_capability(capability).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agents_by_specialization
#[php_function]
pub fn todozi_get_agents_by_specialization(specialization: &str) -> Result<Vec<&Agent, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_by_specialization(specialization).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agents_dir
#[php_function]
pub fn todozi_get_agents_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_agents_with_assignments
#[php_function]
pub fn todozi_get_agents_with_assignments() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_with_assignments().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_ai_tasks
#[php_function]
pub fn todozi_get_ai_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_active_tasks
#[php_function]
pub fn todozi_get_all_active_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_agents
#[php_function]
pub fn todozi_get_all_agents() -> Result<Vec<&Agent, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_agents().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_categories
#[php_function]
pub fn todozi_get_all_categories() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_categories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_completed_tasks
#[php_function]
pub fn todozi_get_all_completed_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_ideas
#[php_function]
pub fn todozi_get_all_ideas() -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_all_ideas().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_items
#[php_function]
pub fn todozi_get_all_items() -> Result<Vec<&QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_items().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_keys
#[php_function]
pub fn todozi_get_all_keys() -> Result<Vec<&ApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_keys().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_memories
#[php_function]
pub fn todozi_get_all_memories() -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_memories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_reminders
#[php_function]
pub fn todozi_get_all_reminders() -> Result<Vec<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_summaries
#[php_function]
pub fn todozi_get_all_summaries() -> Result<Vec<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_summaries().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_tags
#[php_function]
pub fn todozi_get_all_tags() -> Result<Vec<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_all_tags().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_tasks
#[php_function]
pub fn todozi_get_all_tasks() -> Result<Vec<&Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_tasks().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_all_tools
#[php_function]
pub fn todozi_get_all_tools() -> Result<Vec<&Box<dyn Tool, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_tools().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_api_key
#[php_function]
pub fn todozi_get_api_key(user_id: &str) -> Result<JsApiKey, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_api_key(user_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_api_key_by_public
#[php_function]
pub fn todozi_get_api_key_by_public(public_key: &str) -> Result<JsApiKey, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_api_key_by_public(public_key).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_assignee_emoji
#[php_function]
pub fn todozi_get_assignee_emoji(assignee: &str) -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_assignee_emoji(assignee).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_assignments_dir
#[php_function]
pub fn todozi_get_assignments_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_assignments_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_available_agents
#[php_function]
pub fn todozi_get_available_agents() -> Result<Vec<Agent, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_available_agents().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_backlog_items
#[php_function]
pub fn todozi_get_backlog_items() -> Result<Vec<&QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_backlog_items().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_breakthrough_ideas
#[php_function]
pub fn todozi_get_breakthrough_ideas() -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_breakthrough_ideas().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunk
#[php_function]
pub fn todozi_get_chunk(chunk_id: &str) -> Result<Option<&CodeChunk, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunk(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunk_mut
#[php_function]
pub fn todozi_get_chunk_mut(chunk_id: &str) -> Result<Option<&mut CodeChunk, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunk_mut(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunk_status
#[php_function]
pub fn todozi_get_chunk_status(chunk_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunk_status(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunking_level_info
#[php_function]
pub fn todozi_get_chunking_level_info(level: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunking_level_info(level).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunking_levels
#[php_function]
pub fn todozi_get_chunking_levels() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunking_levels().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunks_by_level
#[php_function]
pub fn todozi_get_chunks_by_level(level: ChunkingLevel) -> Result<Vec<&CodeChunk, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunks_by_level(level).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_chunks_dir
#[php_function]
pub fn todozi_get_chunks_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunks_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_collaborative_tasks
#[php_function]
pub fn todozi_get_collaborative_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_command_documentation
#[php_function]
pub fn todozi_get_command_documentation(command_name: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_command_documentation(command_name.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_complete_items
#[php_function]
pub fn todozi_get_complete_items() -> Result<Vec<&QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_complete_items().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_critical_memories
#[php_function]
pub fn todozi_get_critical_memories() -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_critical_memories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_current_duration
#[php_function]
pub fn todozi_get_current_duration() -> Result<u64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_current_duration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_default_model
#[php_function]
pub fn todozi_get_default_model() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_default_model().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_dependency_chain
#[php_function]
pub fn todozi_get_dependency_chain(chunk_id: &str) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_dependency_chain(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_embedding_statistics
#[php_function]
pub fn todozi_get_embedding_statistics() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_embedding_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_emotional_memories
#[php_function]
pub fn todozi_get_emotional_memories(emotion: &str) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_emotional_memories(emotion).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_enabled_tools
#[php_function]
pub fn todozi_get_enabled_tools() -> Result<Vec<&AgentTool, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_enabled_tools().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_error_details
#[php_function]
pub fn todozi_get_error_details(error_message: &str) -> Result<JsErrorDetails, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_error_details(error_message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_errors_dir
#[php_function]
pub fn todozi_get_errors_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_errors_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_filtered_tasks
#[php_function]
pub fn todozi_get_filtered_tasks(filters: &str) -> Result<Vec<&Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_filtered_tasks(filters).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_formatted_prompt
#[php_function]
pub fn todozi_get_formatted_prompt(variables: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_formatted_prompt(variables).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_high_priority_summaries
#[php_function]
pub fn todozi_get_high_priority_summaries() -> Result<Vec<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_high_priority_summaries().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_human_memories
#[php_function]
pub fn todozi_get_human_memories() -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_human_memories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_human_tasks
#[php_function]
pub fn todozi_get_human_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_idea
#[php_function]
pub fn todozi_get_idea(idea_id: &str) -> Result<Option<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_idea(idea_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_idea_statistics
#[php_function]
pub fn todozi_get_idea_statistics() -> Result<IdeaStatistics, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_idea_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_ideas_by_importance
#[php_function]
pub fn todozi_get_ideas_by_importance(importance: IdeaImportance) -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_by_importance(importance).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_ideas_by_share_level
#[php_function]
pub fn todozi_get_ideas_by_share_level(share_level: ShareLevel) -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_by_share_level(share_level).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_ideas_by_tag
#[php_function]
pub fn todozi_get_ideas_by_tag(tag: &str) -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_by_tag(tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_ideas_dir
#[php_function]
pub fn todozi_get_ideas_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_item
#[php_function]
pub fn todozi_get_item(id: &str) -> Result<Option<&QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_item(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_item_mut
#[php_function]
pub fn todozi_get_item_mut(id: &str) -> Result<Option<&mut QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_item_mut(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_items_by_status
#[php_function]
pub fn todozi_get_items_by_status(status: QueueStatus) -> Result<Vec<&QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_items_by_status(status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_key
#[php_function]
pub fn todozi_get_key(user_id: &str) -> Result<Option<&ApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_key(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_key_by_public
#[php_function]
pub fn todozi_get_key_by_public(public_key: &str) -> Result<Option<&ApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_key_by_public(public_key).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_long_term_memories
#[php_function]
pub fn todozi_get_long_term_memories() -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_long_term_memories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memories_by_importance
#[php_function]
pub fn todozi_get_memories_by_importance(importance: MemoryImportance) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_by_importance(importance).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memories_by_tag
#[php_function]
pub fn todozi_get_memories_by_tag(tag: &str) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_memories_by_tag(tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memories_by_term
#[php_function]
pub fn todozi_get_memories_by_term(term: MemoryTerm) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_by_term(term).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memories_by_type
#[php_function]
pub fn todozi_get_memories_by_type(memory_type: &str) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_by_type(memory_type).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memories_dir
#[php_function]
pub fn todozi_get_memories_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memory
#[php_function]
pub fn todozi_get_memory(memory_id: &str) -> Result<Option<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Memories::get_memory(memory_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_memory_statistics
#[php_function]
pub fn todozi_get_memory_statistics() -> Result<MemoryStatistics, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Memories::get_memory_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_most_used_tags
#[php_function]
pub fn todozi_get_most_used_tags(limit: usize) -> Result<Vec<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_most_used_tags(limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_next_chunk_to_work_on
#[php_function]
pub fn todozi_get_next_chunk_to_work_on() -> Result<Option<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_next_chunk_to_work_on().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_or_generate_embedding
#[php_function]
pub fn todozi_get_or_generate_embedding(content_id: &str, text: &str, content_type: TodoziContentType, refresh_if_stale: bool) -> Result<Vec<f32, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_or_generate_embedding(content_id, text, content_type, refresh_if_stale).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_overdue_reminders
#[php_function]
pub fn todozi_get_overdue_reminders() -> Result<Vec<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_overdue_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_pending_reminders
#[php_function]
pub fn todozi_get_pending_reminders() -> Result<Vec<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_pending_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_priority_emoji
#[php_function]
pub fn todozi_get_priority_emoji(priority: &str) -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_priority_emoji(priority).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_private_ideas
#[php_function]
pub fn todozi_get_private_ideas() -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_private_ideas().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_processor_state
#[php_function]
pub fn todozi_get_processor_state() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_processor_state().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_project
#[php_function]
pub fn todozi_get_project(name: &str) -> Result<Project, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_project(name).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_project_stats
#[php_function]
pub fn todozi_get_project_stats(project_name: &str) -> Result<ProjectStats, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::get_project_stats(project_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_project_summary
#[php_function]
pub fn todozi_get_project_summary() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::get_project_summary().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_project_tasks
#[php_function]
pub fn todozi_get_project_tasks(project_name: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_project_tasks_dir
#[php_function]
pub fn todozi_get_project_tasks_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_project_tasks_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_public_ideas
#[php_function]
pub fn todozi_get_public_ideas() -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_public_ideas().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_queue_item
#[php_function]
pub fn todozi_get_queue_item(id: &str) -> Result<QueueItem, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::get_queue_item(id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_queue_session
#[php_function]
pub fn todozi_get_queue_session(session_id: &str) -> Result<crate::models::QueueSession, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::get_queue_session(session_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_queue_statistics
#[php_function]
pub fn todozi_get_queue_statistics() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::get_queue_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_ready_chunks
#[php_function]
pub fn todozi_get_ready_chunks() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_ready_chunks().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_recent_actions
#[php_function]
pub fn todozi_get_recent_actions(limit: Option<u32>) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_actions(limit.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_recent_ideas
#[php_function]
pub fn todozi_get_recent_ideas(limit: usize) -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_recent_ideas(limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_recent_memories
#[php_function]
pub fn todozi_get_recent_memories(limit: usize) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_memories(limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_recent_reminders
#[php_function]
pub fn todozi_get_recent_reminders(limit: usize) -> Result<Vec<&Reminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_reminders(limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_recent_summaries
#[php_function]
pub fn todozi_get_recent_summaries(limit: usize) -> Result<Vec<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_summaries(limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_recent_tags
#[php_function]
pub fn todozi_get_recent_tags(limit: usize) -> Result<Vec<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_recent_tags(limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_registration_info
#[php_function]
pub fn todozi_get_registration_info() -> Result<Option<RegistrationInfo, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_registration_info().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_related_tags
#[php_function]
pub fn todozi_get_related_tags(tag_id: &str) -> Result<Vec<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_related_tags(tag_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_reminder
#[php_function]
pub fn todozi_get_reminder(reminder_id: &str) -> Result<Option<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminder(reminder_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_reminder_statistics
#[php_function]
pub fn todozi_get_reminder_statistics() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminder_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_reminders_by_priority
#[php_function]
pub fn todozi_get_reminders_by_priority(priority: ReminderPriority) -> Result<Vec<&Reminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminders_by_priority(priority).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_reminders_by_status
#[php_function]
pub fn todozi_get_reminders_by_status(status: ReminderStatus) -> Result<Vec<&Reminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminders_by_status(status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_reminders_by_tag
#[php_function]
pub fn todozi_get_reminders_by_tag(tag: &str) -> Result<Vec<&Reminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_reminders_by_tag(tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_reminders_due_soon
#[php_function]
pub fn todozi_get_reminders_due_soon(duration: Duration) -> Result<Vec<&Reminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminders_due_soon(duration).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_search_analytics
#[php_function]
pub fn todozi_get_search_analytics() -> Result<SearchAnalytics, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_search_analytics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_search_history
#[php_function]
pub fn todozi_get_search_history(limit: Option<u32>) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_search_history(limit.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_search_suggestions
#[php_function]
pub fn todozi_get_search_suggestions(query: &str, limit: usize) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_search_suggestions(query, limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_secret_memories
#[php_function]
pub fn todozi_get_secret_memories() -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_secret_memories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_server_status
#[php_function]
pub fn todozi_get_server_status() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_server_status().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_session
#[php_function]
pub fn todozi_get_session(id: &str) -> Result<Option<&QueueSession, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_session(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_short_term_memories
#[php_function]
pub fn todozi_get_short_term_memories() -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_short_term_memories().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_stats
#[php_function]
pub fn todozi_get_stats() -> Result<HashMap<String, serde_json::Value, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_stats().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_status_emoji
#[php_function]
pub fn todozi_get_status_emoji(status: &str) -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_status_emoji(status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_storage_dir
#[php_function]
pub fn todozi_get_storage_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_storage_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_suggestions
#[php_function]
pub fn todozi_get_suggestions(current_tags: &str, limit: usize) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_suggestions(current_tags, limit).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_summaries_by_priority
#[php_function]
pub fn todozi_get_summaries_by_priority(priority: SummaryPriority) -> Result<Vec<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_summaries_by_priority(priority).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_summaries_by_tag
#[php_function]
pub fn todozi_get_summaries_by_tag(tag: &str) -> Result<Vec<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_summaries_by_tag(tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_summary
#[php_function]
pub fn todozi_get_summary(summary_id: &str) -> Result<Option<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_summary(summary_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_summary_statistics
#[php_function]
pub fn todozi_get_summary_statistics() -> Result<SummaryStatistics, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_summary_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tag
#[php_function]
pub fn todozi_get_tag(tag_id: &str) -> Result<Option<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tag(tag_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tag_by_name
#[php_function]
pub fn todozi_get_tag_by_name(name: &str) -> Result<Option<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tag_by_name(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tag_statistics
#[php_function]
pub fn todozi_get_tag_statistics() -> Result<TagStatistics, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tag_statistics().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tags_by_category
#[php_function]
pub fn todozi_get_tags_by_category(category: &str) -> Result<Vec<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tags_by_category(category).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_task
#[php_function]
pub fn todozi_get_task(id: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task(id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_task_assignments
#[php_function]
pub fn todozi_get_task_assignments(task_id: &str) -> Result<Vec<&AgentAssignment, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_assignments(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_task_from_any_project
#[php_function]
pub fn todozi_get_task_from_any_project(id: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_from_any_project(id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_task_from_project
#[php_function]
pub fn todozi_get_task_from_project(project_name: &str, task_id: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_from_project(project_name, task_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_task_mut
#[php_function]
pub fn todozi_get_task_mut(id: &str) -> Result<Option<&mut Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_mut(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tasks_dir
#[php_function]
pub fn todozi_get_tasks_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tasks_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tdz_api_key
#[php_function]
pub fn todozi_get_tdz_api_key() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tdz_api_key().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_team_ideas
#[php_function]
pub fn todozi_get_team_ideas() -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_team_ideas().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tool
#[php_function]
pub fn todozi_get_tool(name: &str) -> Result<Option<&Box<dyn Tool, E>>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tool(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tool_definitions
#[php_function]
pub fn todozi_get_tool_definitions() -> Result<Vec<serde_json::Value, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tool_definitions().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_training_dir
#[php_function]
pub fn todozi_get_training_dir() -> Result<PathBuf, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_training_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_tsne_coordinates
#[php_function]
pub fn todozi_get_tsne_coordinates(content_ids: Vec<String>, dimensions: usize) -> Result<Vec<(String, Vec<f32, E>)>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tsne_coordinates(content_ids, dimensions).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_unresolved_errors
#[php_function]
pub fn todozi_get_unresolved_errors() -> Result<Vec<&Error, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_unresolved_errors().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// get_version_history
#[php_function]
pub fn todozi_get_version_history(content_id: &str) -> Result<Vec<serde_json::Value, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_version_history(content_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_add_command
#[php_function]
pub fn todozi_handle_add_command(command: AddCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_add_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_agent_command
#[php_function]
pub fn todozi_handle_agent_command(command: Commands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_agent_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_ai_commands
#[php_function]
pub fn todozi_handle_ai_commands(command: &str, args: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_ai_commands(command, args).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_api_command
#[php_function]
pub fn todozi_handle_api_command(command: ApiCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_api_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_chat_command
#[php_function]
pub fn todozi_handle_chat_command(command: Commands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_chat_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_emb_command
#[php_function]
pub fn todozi_handle_emb_command(command: Commands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_emb_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_error_command
#[php_function]
pub fn todozi_handle_error_command(command: Commands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_error_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_extract_command
#[php_function]
pub fn todozi_handle_extract_command(content: Option<String>, file: Option<String>, output_format: &str, human: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_extract_command(content.as_deref(), file.as_deref(), output_format, human).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_idea_command
#[php_function]
pub fn todozi_handle_idea_command(command: IdeaCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_idea_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_ind_command
#[php_function]
pub fn todozi_handle_ind_command() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_ind_command().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_list_backups_command
#[php_function]
pub fn todozi_handle_list_backups_command() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_list_backups_command().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_list_command
#[php_function]
pub fn todozi_handle_list_command(command: ListCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_list_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_memory_command
#[php_function]
pub fn todozi_handle_memory_command(command: MemoryCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_memory_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_project_command
#[php_function]
pub fn todozi_handle_project_command(command: ProjectCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_project_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_queue_command
#[php_function]
pub fn todozi_handle_queue_command(command: QueueCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_queue_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_search_all_command
#[php_function]
pub fn todozi_handle_search_all_command(command: Commands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_search_all_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_search_command
#[php_function]
pub fn todozi_handle_search_command(command: SearchCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_search_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_server_command
#[php_function]
pub fn todozi_handle_server_command(command: ServerCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_server_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_show_command
#[php_function]
pub fn todozi_handle_show_command(command: ShowCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_show_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_stats_command
#[php_function]
pub fn todozi_handle_stats_command(_command: StatsCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_stats_command(_command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_strategy_command
#[php_function]
pub fn todozi_handle_strategy_command(content: Option<String>, file: Option<String>, output_format: &str, human: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_strategy_command(content.as_deref(), file.as_deref(), output_format, human).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_train_command
#[php_function]
pub fn todozi_handle_train_command(command: TrainingCommands) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_train_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// handle_update_command
#[php_function]
pub fn todozi_handle_update_command(id: &str, action: Option<String>, time: Option<String>, priority: Option<String>, project: Option<String>, status: Option<String>, assignee: Option<String>, tags: Option<String>, dependencies: Option<String>, context: Option<String>, progress: Option<u8>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_update_command(id, action.as_deref(), time.as_deref(), priority.as_deref(), project.as_deref(), status.as_deref(), assignee.as_deref(), tags.as_deref(), dependencies.as_deref(), context.as_deref(), progress.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// has_capability
#[php_function]
pub fn todozi_has_capability(capability: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_capability(capability).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// has_results
#[php_function]
pub fn todozi_has_results() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_results().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// has_specialization
#[php_function]
pub fn todozi_has_specialization(specialization: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_specialization(specialization).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// has_tool
#[php_function]
pub fn todozi_has_tool(tool_name: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_tool(tool_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// hash_project_name
#[php_function]
pub fn todozi_hash_project_name(project_name: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::hash_project_name(project_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// hierarchical_clustering
#[php_function]
pub fn todozi_hierarchical_clustering(content_types: Vec<TodoziContentType>, max_depth: usize) -> Result<Vec<HierarchicalCluster, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::hierarchical_clustering(content_types, max_depth).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// high
#[php_function]
pub fn todozi_high(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::high(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// high_priority_percentage
#[php_function]
pub fn todozi_high_priority_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::high_priority_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// human
#[php_function]
pub fn todozi_human(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::human(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// human_task
#[php_function]
pub fn todozi_human_task(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::human_task(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// hybrid_search
#[php_function]
pub fn todozi_hybrid_search(query: &str, keywords: Vec<String>, content_types: Option<Vec<TodoziContentType>>, semantic_weight: f32, // 0.0-1.0
        limit: usize) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::hybrid_search(query, keywords, content_types.as_ref(), semantic_weight, // 0.0-1.0
        limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// idea
#[php_function]
pub fn todozi_idea(idea: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::idea(idea).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ideate
#[php_function]
pub fn todozi_ideate(idea: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ideate(idea).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// import_from_format
#[php_function]
pub fn todozi_import_from_format(data: &str, format: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::import_from_format(data, format).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// importance
#[php_function]
pub fn todozi_importance(importance: IdeaImportance) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::importance(importance).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// important
#[php_function]
pub fn todozi_important(moment: &str, meaning: &str, reason: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::important(moment, meaning, reason).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// important_memory
#[php_function]
pub fn todozi_important_memory(moment: &str, meaning: &str, reason: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::important_memory(moment, meaning, reason).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// increment_tag_usage
#[php_function]
pub fn todozi_increment_tag_usage(tag_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::increment_tag_usage(tag_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// init
#[php_function]
pub fn todozi_init() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::init().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// init_storage
#[php_function]
pub fn todozi_init_storage() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::init_storage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// init_with_auto_registration
#[php_function]
pub fn todozi_init_with_auto_registration() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::init_with_auto_registration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// initialize
#[php_function]
pub fn todozi_initialize() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// initialize_grok_level_todozi_system
#[php_function]
pub fn todozi_initialize_grok_level_todozi_system() -> Result<std::result::Result<
    SharedTodozi,
    TodoziError,
>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_grok_level_todozi_system().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// initialize_grok_level_todozi_system_with_embedding
#[php_function]
pub fn todozi_initialize_grok_level_todozi_system_with_embedding(enable_embeddings: bool) -> Result<std::result::Result<(SharedTodozi, Option<TodoziEmbeddingService>), TodoziError>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_grok_level_todozi_system_with_embedding(enable_embeddings).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// initialize_tdz_content_processor
#[php_function]
pub fn todozi_initialize_tdz_content_processor() -> Result<SharedTodoziState, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_tdz_content_processor().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// initialize_tdz_processor
#[php_function]
pub fn todozi_initialize_tdz_processor() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_tdz_processor().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// interactive_create_task
#[php_function]
pub fn todozi_interactive_create_task() -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::interactive_create_task().await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// io
#[php_function]
pub fn todozi_io(message: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::io(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_active
#[php_function]
pub fn todozi_is_active() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_active().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_admin
#[php_function]
pub fn todozi_is_admin(public_key: &str, private_key: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_admin(public_key, private_key).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_available
#[php_function]
pub fn todozi_is_available() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_available().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_backlog
#[php_function]
pub fn todozi_is_backlog() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_backlog().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_complete
#[php_function]
pub fn todozi_is_complete() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_complete().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_completed
#[php_function]
pub fn todozi_is_completed() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_completed().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_empty
#[php_function]
pub fn todozi_is_empty() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_empty().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_overdue
#[php_function]
pub fn todozi_is_overdue() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_overdue().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// is_registered
#[php_function]
pub fn todozi_is_registered() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_registered().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// keyword_search
#[php_function]
pub fn todozi_keyword_search(query: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::keyword_search(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// keyword_tasks
#[php_function]
pub fn todozi_keyword_tasks(query: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::keyword_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// launch_gui
#[php_function]
pub fn todozi_launch_gui() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::launch_gui().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// launch_tui
#[php_function]
pub fn todozi_launch_tui() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::launch_tui().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// len
#[php_function]
pub fn todozi_len() -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::len().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list
#[php_function]
pub fn todozi_list() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_active_api_keys
#[php_function]
pub fn todozi_list_active_api_keys() -> Result<Vec<JsApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_active_api_keys().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_active_items
#[php_function]
pub fn todozi_list_active_items() -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_active_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_agent_assignments
#[php_function]
pub fn todozi_list_agent_assignments(agent_id: &str) -> Result<Vec<AgentAssignment, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_agent_assignments(agent_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_agents
#[php_function]
pub fn todozi_list_agents() -> Result<Vec<Agent, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_agents().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_all_agent_assignments
#[php_function]
pub fn todozi_list_all_agent_assignments() -> Result<Vec<AgentAssignment, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_all_agent_assignments().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_api_keys
#[php_function]
pub fn todozi_list_api_keys() -> Result<Vec<JsApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_api_keys().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_available_agents
#[php_function]
pub fn todozi_list_available_agents() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_available_agents().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_available_commands
#[php_function]
pub fn todozi_list_available_commands() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_available_commands().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_available_tools
#[php_function]
pub fn todozi_list_available_tools() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_available_tools().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_backlog_items
#[php_function]
pub fn todozi_list_backlog_items() -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_backlog_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_backups
#[php_function]
pub fn todozi_list_backups() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_backups().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_code_chunks
#[php_function]
pub fn todozi_list_code_chunks() -> Result<Vec<CodeChunk, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_code_chunks().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_complete_items
#[php_function]
pub fn todozi_list_complete_items() -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_complete_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_errors
#[php_function]
pub fn todozi_list_errors() -> Result<Vec<Error, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_errors().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_feelings
#[php_function]
pub fn todozi_list_feelings() -> Result<Vec<Feeling, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_feelings().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_ideas
#[php_function]
pub fn todozi_list_ideas() -> Result<Vec<Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::list_ideas().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_memories
#[php_function]
pub fn todozi_list_memories() -> Result<Vec<Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_memories().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_project_task_containers
#[php_function]
pub fn todozi_list_project_task_containers() -> Result<Vec<ProjectTaskContainer, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_project_task_containers().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_projects
#[php_function]
pub fn todozi_list_projects() -> Result<Vec<Project, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::list_projects().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_queue_items
#[php_function]
pub fn todozi_list_queue_items() -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::list_queue_items().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_queue_items_by_status
#[php_function]
pub fn todozi_list_queue_items_by_status(status: QueueStatus) -> Result<Vec<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::list_queue_items_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_reminders
#[php_function]
pub fn todozi_list_reminders() -> Result<Vec<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_reminders().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_summaries
#[php_function]
pub fn todozi_list_summaries() -> Result<Vec<JsSummary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_summaries().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_tasks
#[php_function]
pub fn todozi_list_tasks() -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_tasks_across_projects
#[php_function]
pub fn todozi_list_tasks_across_projects(filters: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_tasks_across_projects(filters).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_tasks_in_project
#[php_function]
pub fn todozi_list_tasks_in_project(project_name: &str, filters: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_tasks_in_project(project_name, filters).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// list_training_data
#[php_function]
pub fn todozi_list_training_data() -> Result<Vec<TrainingData, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_training_data().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load
#[php_function]
pub fn todozi_load(model_name: &str, device: Device) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load(model_name, device).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_additional_model
#[php_function]
pub fn todozi_load_additional_model(model_name: &str, model_alias: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_additional_model(model_name, model_alias).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_agent
#[php_function]
pub fn todozi_load_agent(agent_id: &str) -> Result<Agent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_agent(agent_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_agent_assignment
#[php_function]
pub fn todozi_load_agent_assignment(agent_id: &str, task_id: &str) -> Result<AgentAssignment, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_agent_assignment(agent_id, task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_agents
#[php_function]
pub fn todozi_load_agents() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_agents().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_api_key_collection
#[php_function]
pub fn todozi_load_api_key_collection() -> Result<ApiKeyCollection, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_api_key_collection().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_api_keys
#[php_function]
pub fn todozi_load_api_keys() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_api_keys().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_code_chunk
#[php_function]
pub fn todozi_load_code_chunk(chunk_id: &str) -> Result<CodeChunk, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_code_chunk(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_config
#[php_function]
pub fn todozi_load_config() -> Result<Config, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_config().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_error
#[php_function]
pub fn todozi_load_error(error_id: &str) -> Result<Error, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_error(error_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_extended_data
#[php_function]
pub fn todozi_load_extended_data() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_extended_data().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_feeling
#[php_function]
pub fn todozi_load_feeling(id: &str) -> Result<Feeling, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_feeling(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_idea
#[php_function]
pub fn todozi_load_idea(idea_id: &str) -> Result<Idea, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_idea(idea_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_memory
#[php_function]
pub fn todozi_load_memory(memory_id: &str) -> Result<Memory, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_memory(memory_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_project
#[php_function]
pub fn todozi_load_project(project_name: &str) -> Result<Project, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_project(project_name).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_project_task_container
#[php_function]
pub fn todozi_load_project_task_container(project_name: &str) -> Result<ProjectTaskContainer, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_project_task_container(project_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_project_task_container_by_hash
#[php_function]
pub fn todozi_load_project_task_container_by_hash(project_hash: &str) -> Result<ProjectTaskContainer, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_project_task_container_by_hash(project_hash).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_queue_collection
#[php_function]
pub fn todozi_load_queue_collection() -> Result<QueueCollection, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_queue_collection().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_task
#[php_function]
pub fn todozi_load_task(task_id: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_task(task_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_task_collection
#[php_function]
pub fn todozi_load_task_collection(collection_name: &str) -> Result<TaskCollection, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_task_collection(collection_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_tasks
#[php_function]
pub fn todozi_load_tasks() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_tasks().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// load_training_data
#[php_function]
pub fn todozi_load_training_data(training_data_id: &str) -> Result<TrainingData, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_training_data(training_data_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// long_term_percentage
#[php_function]
pub fn todozi_long_term_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::long_term_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// low
#[php_function]
pub fn todozi_low(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::low(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// mark_chunk_completed
#[php_function]
pub fn todozi_mark_chunk_completed(chunk_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_chunk_completed(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// mark_chunk_validated
#[php_function]
pub fn todozi_mark_chunk_validated(chunk_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_chunk_validated(chunk_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// mark_reminder_cancelled
#[php_function]
pub fn todozi_mark_reminder_cancelled(reminder_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_reminder_cancelled(reminder_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// mark_reminder_completed
#[php_function]
pub fn todozi_mark_reminder_completed(reminder_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_reminder_completed(reminder_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// matches
#[php_function]
pub fn todozi_matches(public_key: &str, private_key: Option<&str>) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::matches(public_key, private_key.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// max_tokens
#[php_function]
pub fn todozi_max_tokens() -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::max_tokens().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// meaning
#[php_function]
pub fn todozi_meaning(meaning: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::meaning(meaning).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// merge_tags
#[php_function]
pub fn todozi_merge_tags(primary_tag_id: &str, duplicate_tag_ids: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::merge_tags(primary_tag_id, duplicate_tag_ids).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// migrate
#[php_function]
pub fn todozi_migrate() -> Result<MigrationReport, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::migrate().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// migrate_tasks
#[php_function]
pub fn todozi_migrate_tasks(dry_run: Option<bool>, verbose: Option<bool>, force_overwrite: Option<bool>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::migrate_tasks(dry_run.as_ref(), verbose.as_ref(), force_overwrite.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// migrate_to_project_based
#[php_function]
pub fn todozi_migrate_to_project_based() -> Result<MigrationReport, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::migrate_to_project_based().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// moment
#[php_function]
pub fn todozi_moment(moment: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::moment(moment).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// move_task
#[php_function]
pub fn todozi_move_task(id: &str, from_collection: &str, to_collection: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::move_task(id, from_collection, to_collection).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// multi_query_search
#[php_function]
pub fn todozi_multi_query_search(queries: Vec<&str>, aggregation: AggregationType, content_types: Option<Vec<TodoziContentType>>, limit: usize) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::multi_query_search(queries, aggregation, content_types.as_ref(), limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// name
#[php_function]
pub fn todozi_name(name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::name(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// new_full
#[php_function]
pub fn todozi_new_full(user_id: &str, action: &str, time: &str, priority: Priority, parent_project: &str, status: Status, assignee: Option<Assignee>, tags: Vec<String>, dependencies: Vec<String>, context_notes: Option<String>, progress: Option<u8>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_full(user_id, action, time, priority, parent_project, status, assignee.as_ref(), tags, dependencies, context_notes.as_deref(), progress.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// new_idea
#[php_function]
pub fn todozi_new_idea(idea: Idea) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_idea(idea).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// new_memory
#[php_function]
pub fn todozi_new_memory(memory: Memory) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_memory(memory).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// new_with_hashes
#[php_function]
pub fn todozi_new_with_hashes(server_url: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_with_hashes(server_url).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// ok
#[php_function]
pub fn todozi_ok(body: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ok(body).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// overdue_percentage
#[php_function]
pub fn todozi_overdue_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::overdue_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_agent_assignment_format
#[php_function]
pub fn todozi_parse_agent_assignment_format(agent_text: &str) -> Result<AgentAssignment, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_agent_assignment_format(agent_text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_chunking_format
#[php_function]
pub fn todozi_parse_chunking_format(chunk_text: &str) -> Result<CodeChunk, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_chunking_format(chunk_text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_dependencies
#[php_function]
pub fn todozi_parse_dependencies(deps_str: &str) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_dependencies(deps_str).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_error_format
#[php_function]
pub fn todozi_parse_error_format(error_text: &str) -> Result<Error, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_error_format(error_text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_feeling_format
#[php_function]
pub fn todozi_parse_feeling_format(feel_text: &str) -> Result<Feeling, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_feeling_format(feel_text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_idea_format
#[php_function]
pub fn todozi_parse_idea_format(idea_text: &str) -> Result<Idea, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_idea_format(idea_text).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_memory_format
#[php_function]
pub fn todozi_parse_memory_format(memory_text: &str, user_id: &str) -> Result<Memory, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_memory_format(memory_text, user_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_reminder_format
#[php_function]
pub fn todozi_parse_reminder_format(reminder_text: &str) -> Result<Reminder, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_reminder_format(reminder_text).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_summary_format
#[php_function]
pub fn todozi_parse_summary_format(summary_text: &str) -> Result<Summary, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_summary_format(summary_text).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_tags
#[php_function]
pub fn todozi_parse_tags(tags_str: &str) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_tags(tags_str).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_tdz_command
#[php_function]
pub fn todozi_parse_tdz_command(text: &str) -> Result<Vec<TdzCommand, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_tdz_command(text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_todozi_format
#[php_function]
pub fn todozi_parse_todozi_format(todozi_text: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_todozi_format(todozi_text).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// parse_training_data_format
#[php_function]
pub fn todozi_parse_training_data_format(train_text: &str) -> Result<TrainingData, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_training_data_format(train_text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// pending_percentage
#[php_function]
pub fn todozi_pending_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::pending_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// plan_task_actions
#[php_function]
pub fn todozi_plan_task_actions(goal: &str) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::plan_task_actions(goal).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// plan_tasks
#[php_function]
pub fn todozi_plan_tasks(goal: &str, complexity: Option<&str>, timeline: Option<&str>, context: Option<&str>) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::plan_tasks(goal, complexity.as_deref(), timeline.as_deref(), context.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// predict_relevance
#[php_function]
pub fn todozi_predict_relevance(_features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::predict_relevance(_features).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// preload_related_embeddings
#[php_function]
pub fn todozi_preload_related_embeddings(content_id: &str, depth: usize) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::preload_related_embeddings(content_id, depth).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// prepare_task_content
#[php_function]
pub fn todozi_prepare_task_content(task: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::prepare_task_content(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// prioritize_queue_item
#[php_function]
pub fn todozi_prioritize_queue_item(item_id: &str, priority: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::prioritize_queue_item(item_id, priority).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// priority
#[php_function]
pub fn todozi_priority(priority: SummaryPriority) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::priority(priority).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// private_percentage
#[php_function]
pub fn todozi_private_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::private_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_chat
#[php_function]
pub fn todozi_process_chat(message: &str, user_id: &str) -> Result<ChatContent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chat(message, user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_chat_message
#[php_function]
pub fn todozi_process_chat_message(message: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chat_message(message).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_chat_message_extended
#[php_function]
pub fn todozi_process_chat_message_extended(message: &str, user_id: &str) -> Result<ChatContent, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chat_message_extended(message, user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_chunking_message
#[php_function]
pub fn todozi_process_chunking_message(message: &str) -> Result<Vec<CodeChunk, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chunking_message(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_json_examples
#[php_function]
pub fn todozi_process_json_examples(json_data: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_json_examples(json_data).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_tdz_commands
#[php_function]
pub fn todozi_process_tdz_commands(text: &str, base_url: &str, api_key: Option<&str>) -> Result<Vec<serde_json::Value, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_tdz_commands(text, base_url, api_key.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// process_workflow
#[php_function]
pub fn todozi_process_workflow(tasks: Vec<Task>) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_workflow(tasks).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// profile_search_performance
#[php_function]
pub fn todozi_profile_search_performance(query: &str, iterations: usize) -> Result<PerformanceMetrics, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::profile_search_performance(query, iterations).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// project_name
#[php_function]
pub fn todozi_project_name() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::project_name().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// project_tasks
#[php_function]
pub fn todozi_project_tasks(project_name: &str) -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// public_percentage
#[php_function]
pub fn todozi_public_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::public_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_active
#[php_function]
pub fn todozi_queue_active() -> Result<Vec<JsQueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_active().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_add
#[php_function]
pub fn todozi_queue_add(task_name: &str, description: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_add(task_name, description).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_backlog
#[php_function]
pub fn todozi_queue_backlog() -> Result<Vec<JsQueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_backlog().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_bulk_operations
#[php_function]
pub fn todozi_queue_bulk_operations(operations: Vec<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_bulk_operations(operations).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_complete
#[php_function]
pub fn todozi_queue_complete(session_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_complete(session_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_list
#[php_function]
pub fn todozi_queue_list() -> Result<Vec<JsQueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_list().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// queue_start
#[php_function]
pub fn todozi_queue_start(item_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_start(item_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// quick
#[php_function]
pub fn todozi_quick() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::quick().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// quick_task
#[php_function]
pub fn todozi_quick_task(action: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::quick_task(action).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// reason
#[php_function]
pub fn todozi_reason(reason: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::reason(reason).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// recommend_similar
#[php_function]
pub fn todozi_recommend_similar(based_on: Vec<String>, exclude: Vec<String>, limit: usize) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::recommend_similar(based_on, exclude, limit).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// register_tool
#[php_function]
pub fn todozi_register_tool(tool_def: JsToolDefinition) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::register_tool(tool_def).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// register_with_server
#[php_function]
pub fn todozi_register_with_server(server_url: &str) -> Result<RegistrationInfo, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::register_with_server(server_url).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// relationships_per_tag
#[php_function]
pub fn todozi_relationships_per_tag() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::relationships_per_tag().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remember
#[php_function]
pub fn todozi_remember(moment: &str, meaning: &str) -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::remember(moment, meaning).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remind_at
#[php_function]
pub fn todozi_remind_at(remind_at: DateTime<Utc>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remind_at(remind_at).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remove_api_key
#[php_function]
pub fn todozi_remove_api_key(user_id: &str) -> Result<JsApiKey, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_api_key(user_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remove_from_task
#[php_function]
pub fn todozi_remove_from_task(task_id: &str, tag: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::remove_from_task(task_id, tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remove_item
#[php_function]
pub fn todozi_remove_item(id: &str) -> Result<Option<QueueItem, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_item(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remove_key
#[php_function]
pub fn todozi_remove_key(user_id: &str) -> Result<Option<ApiKey, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_key(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remove_tag_from_task
#[php_function]
pub fn todozi_remove_tag_from_task(task_id: &str, tag: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_tag_from_task(task_id, tag).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// remove_task
#[php_function]
pub fn todozi_remove_task(id: &str) -> Result<Option<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_task(id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// render
#[php_function]
pub fn todozi_render(config: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::render(config).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// render_compact
#[php_function]
pub fn todozi_render_compact(config: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::render_compact(config).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// render_detailed
#[php_function]
pub fn todozi_render_detailed(config: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::render_detailed(config).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// reorder_queue
#[php_function]
pub fn todozi_reorder_queue(item_ids: Vec<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::reorder_queue(item_ids).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// resolve_error
#[php_function]
pub fn todozi_resolve_error(error_id: &str, resolution: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::resolve_error(error_id, resolution).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// restore_backup
#[php_function]
pub fn todozi_restore_backup(backup_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::restore_backup(backup_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// restore_embeddings
#[php_function]
pub fn todozi_restore_embeddings(backup_path: &str) -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::restore_embeddings(backup_path).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// run
#[php_function]
pub fn todozi_run() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::run().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// run_interactive
#[php_function]
pub fn todozi_run_interactive() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::run_interactive().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// sample_task
#[php_function]
pub fn todozi_sample_task() -> Result<Task, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::sample_task().await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_agent
#[php_function]
pub fn todozi_save_agent(agent: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_agent(agent).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_agent_assignment
#[php_function]
pub fn todozi_save_agent_assignment(assignment: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_agent_assignment(assignment).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_api_key_collection
#[php_function]
pub fn todozi_save_api_key_collection(collection: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_api_key_collection(collection).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_as_default
#[php_function]
pub fn todozi_save_as_default(model_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_as_default(model_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_code_chunk
#[php_function]
pub fn todozi_save_code_chunk(chunk: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_code_chunk(chunk).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_config
#[php_function]
pub fn todozi_save_config(config: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_config(config).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_error
#[php_function]
pub fn todozi_save_error(error: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_error(error).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_feeling
#[php_function]
pub fn todozi_save_feeling(feeling: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_feeling(feeling).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_idea
#[php_function]
pub fn todozi_save_idea(idea: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_idea(idea).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_memory
#[php_function]
pub fn todozi_save_memory(memory: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_memory(memory).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_processed_content
#[php_function]
pub fn todozi_save_processed_content(raw: &str, cleaned: &str, session_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_processed_content(raw, cleaned, session_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_project
#[php_function]
pub fn todozi_save_project(project: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_project(project).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_project_task_container
#[php_function]
pub fn todozi_save_project_task_container(container: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_project_task_container(container).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_queue_collection
#[php_function]
pub fn todozi_save_queue_collection(collection: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_queue_collection(collection).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_search_query
#[php_function]
pub fn todozi_save_search_query(query: &str, name: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_search_query(query, name.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_task
#[php_function]
pub fn todozi_save_task(task: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_task(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_task_collection
#[php_function]
pub fn todozi_save_task_collection(collection_name: &str, collection: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_task_collection(collection_name, collection).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_task_with_embedding
#[php_function]
pub fn todozi_save_task_with_embedding(task: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_task_with_embedding(task).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_tasks
#[php_function]
pub fn todozi_save_tasks() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_tasks().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// save_training_data
#[php_function]
pub fn todozi_save_training_data(training_data: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_training_data(training_data).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search
#[php_function]
pub fn todozi_search(query: &str, options: SearchOptions) -> Result<SearchResults, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search(query, options).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_ideas
#[php_function]
pub fn todozi_search_ideas(query: &str) -> Result<Vec<&Idea, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_ideas(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_memories
#[php_function]
pub fn todozi_search_memories(query: &str) -> Result<Vec<&Memory, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_memories(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_reminders
#[php_function]
pub fn todozi_search_reminders(query: &str) -> Result<Vec<JsReminder, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_reminders(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_summaries
#[php_function]
pub fn todozi_search_summaries(query: &str) -> Result<Vec<&Summary, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_summaries(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_tags
#[php_function]
pub fn todozi_search_tags(query: &str) -> Result<Vec<&Tag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_tags(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_tasks
#[php_function]
pub fn todozi_search_tasks(query: &str, semantic: bool, limit: Option<usize>) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_tasks(query, semantic, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_tasks_semantic
#[php_function]
pub fn todozi_search_tasks_semantic(query: &str, max_results: usize) -> Result<Vec<SemanticSearchResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_tasks_semantic(query, max_results).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// search_with_filters
#[php_function]
pub fn todozi_search_with_filters(filters: TaskFilters, limit: Option<usize>) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_with_filters(filters, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// see_all
#[php_function]
pub fn todozi_see_all() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Easy::see_all().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// semantic_search
#[php_function]
pub fn todozi_semantic_search(query: &str, content_types: Option<Vec<TodoziContentType>>, limit: Option<usize>) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::semantic_search(query, content_types.as_ref(), limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// serialization
#[php_function]
pub fn todozi_serialization(message: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::serialization(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// set_color_scheme
#[php_function]
pub fn todozi_set_color_scheme(scheme_json: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::set_color_scheme(scheme_json).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// share
#[php_function]
pub fn todozi_share(share: ShareLevel) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::share(share).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// short_term_percentage
#[php_function]
pub fn todozi_short_term_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::short_term_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// show_loading_screen
#[php_function]
pub fn todozi_show_loading_screen() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::show_loading_screen().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// similar
#[php_function]
pub fn todozi_similar(query: &str) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::similar(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// similar_tasks
#[php_function]
pub fn todozi_similar_tasks(task_id: &str) -> Result<Vec<SimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::similar_tasks(task_id).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// similar_tasks_emb
#[php_function]
pub fn todozi_similar_tasks_emb(query: &str) -> Result<Vec<JsSimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::similar_tasks_emb(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// similarity_threshold_search
#[php_function]
pub fn todozi_similarity_threshold_search(query: &str, threshold: f64, limit: Option<u32>) -> Result<Vec<JsSimilarityResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::similarity_threshold_search(query, threshold, limit.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// smart
#[php_function]
pub fn todozi_smart(query: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::smart(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// smart_search
#[php_function]
pub fn todozi_smart_search(query: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::smart_search(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// specializations
#[php_function]
pub fn todozi_specializations(specializations: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::specializations(specializations).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start
#[php_function]
pub fn todozi_start(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::start(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_edit
#[php_function]
pub fn todozi_start_edit(_task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_edit(_task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_edit_session
#[php_function]
pub fn todozi_start_edit_session(task_id: &str) -> Result<EditSession, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_edit_session(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_local_server
#[php_function]
pub fn todozi_start_local_server(host: Option<String>, port: Option<u32>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_local_server(host.as_deref(), port.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_queue_session
#[php_function]
pub fn todozi_start_queue_session(queue_item_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_queue_session(queue_item_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_server
#[php_function]
pub fn todozi_start_server(host: Option<String>, port: Option<u16>) -> Result<std::result::Result<(, E), Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_server(host.as_deref(), port.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_session
#[php_function]
pub fn todozi_start_session(queue_item_id: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_session(queue_item_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// start_task
#[php_function]
pub fn todozi_start_task(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_task(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// stats
#[php_function]
pub fn todozi_stats() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Emb::stats().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// status
#[php_function]
pub fn todozi_status(status: AgentStatus) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::status(status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// stop_server
#[php_function]
pub fn todozi_stop_server() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::stop_server().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage
#[php_function]
pub fn todozi_storage() -> Result<Storage, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_check_folder_structure
#[php_function]
pub fn todozi_storage_check_folder_structure() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_check_folder_structure().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_clear_registration
#[php_function]
pub fn todozi_storage_clear_registration() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_clear_registration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_create_backup
#[php_function]
pub fn todozi_storage_create_backup() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_create_backup().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_delete_project_by_name
#[php_function]
pub fn todozi_storage_delete_project_by_name(name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_delete_project_by_name(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_delete_task_from_project
#[php_function]
pub fn todozi_storage_delete_task_from_project(task_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_delete_task_from_project(task_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_ensure_folder_structure
#[php_function]
pub fn todozi_storage_ensure_folder_structure() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_ensure_folder_structure().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_ai_tasks
#[php_function]
pub fn todozi_storage_get_ai_tasks() -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_all_active_tasks
#[php_function]
pub fn todozi_storage_get_all_active_tasks() -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_all_completed_tasks
#[php_function]
pub fn todozi_storage_get_all_completed_tasks() -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_collaborative_tasks
#[php_function]
pub fn todozi_storage_get_collaborative_tasks() -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_human_tasks
#[php_function]
pub fn todozi_storage_get_human_tasks() -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_project_stats
#[php_function]
pub fn todozi_storage_get_project_stats(project_name: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_project_stats(project_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_registration_info
#[php_function]
pub fn todozi_storage_get_registration_info() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_registration_info().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_storage_dir
#[php_function]
pub fn todozi_storage_get_storage_dir() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_storage_dir().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_get_task_from_any_project
#[php_function]
pub fn todozi_storage_get_task_from_any_project(task_id: &str) -> Result<JsTask, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_task_from_any_project(task_id).await.map(|item| item.to_php())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_init
#[php_function]
pub fn todozi_storage_init() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_init().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_is_registered
#[php_function]
pub fn todozi_storage_is_registered() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_is_registered().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_list_backups
#[php_function]
pub fn todozi_storage_list_backups() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_list_backups().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_list_projects
#[php_function]
pub fn todozi_storage_list_projects() -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_list_projects().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_load_config
#[php_function]
pub fn todozi_storage_load_config() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_load_config().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_load_project
#[php_function]
pub fn todozi_storage_load_project(name: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_load_project(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_load_task_collection
#[php_function]
pub fn todozi_storage_load_task_collection(name: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_load_task_collection(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_register_with_server
#[php_function]
pub fn todozi_storage_register_with_server(server_url: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_register_with_server(server_url).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_restore_backup
#[php_function]
pub fn todozi_storage_restore_backup(backup_name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_restore_backup(backup_name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_save_config
#[php_function]
pub fn todozi_storage_save_config() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_save_config().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_save_project
#[php_function]
pub fn todozi_storage_save_project(name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_save_project(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_save_task_collection
#[php_function]
pub fn todozi_storage_save_task_collection(name: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_save_task_collection(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// storage_search_tasks_semantic
#[php_function]
pub fn todozi_storage_search_tasks_semantic(query: &str, max_results: Option<u32>) -> Result<Vec<JsTask, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_search_tasks_semantic(query, max_results.as_ref()).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// strategy_content
#[php_function]
pub fn todozi_strategy_content(content: Option<String>, file_path: Option<String>, output_format: &str, human: bool) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strategy_content(content.as_deref(), file_path.as_deref(), output_format, human).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// strategy_content_analysis
#[php_function]
pub fn todozi_strategy_content_analysis(text: &str, format: Option<String>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strategy_content_analysis(text, format.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// strike_cluster
#[php_function]
pub fn todozi_strike_cluster(_embedding: &str) -> Result<std::result::Result<i32, Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strike_cluster(_embedding).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// strike_tags
#[php_function]
pub fn todozi_strike_tags(_features: &str) -> Result<std::result::Result<Vec<f32, E>, Box<dyn std::error::Error>>, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strike_tags(_features).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// success
#[php_function]
pub fn todozi_success(output: &str, execution_time_ms: u64) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::success(output, execution_time_ms).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// suggest_tags
#[php_function]
pub fn todozi_suggest_tags(content_id: &str, top_k: usize) -> Result<Vec<String, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::suggest_tags(content_id, top_k).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tags
#[php_function]
pub fn todozi_tags(tags: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tags(tags).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tags_advanced_search
#[php_function]
pub fn todozi_tags_advanced_search(query: &str) -> Result<Vec<JsTag, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tags_advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// task
#[php_function]
pub fn todozi_task(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::task(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tasks
#[php_function]
pub fn todozi_tasks(project_name: &str) -> Result<Vec<Task, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_php()).collect())
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tdz_cnt
#[php_function]
pub fn todozi_tdz_cnt(content: &str, session_id: Option<&str>) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdz_cnt(content, session_id.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tdz_execute_command
#[php_function]
pub fn todozi_tdz_execute_command(command: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdz_execute_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tdz_find
#[php_function]
pub fn todozi_tdz_find(query: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::tdz_find(query).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tdz_parse_command
#[php_function]
pub fn todozi_tdz_parse_command(input: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdz_parse_command(input).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tdzfp
#[php_function]
pub fn todozi_tdzfp() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdzfp().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// team_percentage
#[php_function]
pub fn todozi_team_percentage() -> Result<f64, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::team_percentage().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// term
#[php_function]
pub fn todozi_term(term: MemoryTerm) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::term(term).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// title
#[php_function]
pub fn todozi_title() -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::title().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// to_context_string
#[php_function]
pub fn todozi_to_context_string() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::to_context_string().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// to_ollama_format
#[php_function]
pub fn todozi_to_ollama_format() -> Result<serde_json::Value, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::to_ollama_format().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// to_state_string
#[php_function]
pub fn todozi_to_state_string() -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::to_state_string().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// todozi_begin
#[php_function]
pub fn todozi_todozi_begin() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::todozi_begin().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// todozi_init
#[php_function]
pub fn todozi_todozi_init() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::todozi_init().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// todozi_init_with_auto_registration
#[php_function]
pub fn todozi_todozi_init_with_auto_registration() -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::todozi_init_with_auto_registration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// tool_count
#[php_function]
pub fn todozi_tool_count() -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tool_count().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// total_results
#[php_function]
pub fn todozi_total_results() -> Result<usize, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::total_results().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// track_embedding_drift
#[php_function]
pub fn todozi_track_embedding_drift(content_id: &str, current_text: &str) -> Result<DriftReport, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::track_embedding_drift(content_id, current_text).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// transform_shorthand_tags
#[php_function]
pub fn todozi_transform_shorthand_tags(message: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::transform_shorthand_tags(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// types
#[php_function]
pub fn todozi_types() -> Result<&'static str, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::types().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// unregister
#[php_function]
pub fn todozi_unregister(name: &str) -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::unregister(name).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update
#[php_function]
pub fn todozi_update(updates: TaskUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update(updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_agent
#[php_function]
pub fn todozi_update_agent(agent_id: &str, updates: AgentUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_agent(agent_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_agent_assignment_status
#[php_function]
pub fn todozi_update_agent_assignment_status(agent_id: &str, task_id: &str, status: AssignmentStatus) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_agent_assignment_status(agent_id, task_id, status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_agent_status
#[php_function]
pub fn todozi_update_agent_status(agent_id: &str, status: AgentStatus) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_agent_status(agent_id, status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_chunk_code
#[php_function]
pub fn todozi_update_chunk_code(chunk_id: &str, code: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_chunk_code(chunk_id, code).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_chunk_tests
#[php_function]
pub fn todozi_update_chunk_tests(chunk_id: &str, tests: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_chunk_tests(chunk_id, tests).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_config
#[php_function]
pub fn todozi_update_config(config: Config) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_config(config).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_config_with_registration
#[php_function]
pub fn todozi_update_config_with_registration(registration: RegistrationInfo) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_config_with_registration(registration).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_feeling
#[php_function]
pub fn todozi_update_feeling(feeling: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_feeling(feeling).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_idea
#[php_function]
pub fn todozi_update_idea(idea_id: &str, updates: IdeaUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_idea(idea_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_memory
#[php_function]
pub fn todozi_update_memory(memory_id: &str, updates: MemoryUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_memory(memory_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_project
#[php_function]
pub fn todozi_update_project(project: Project) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_project(project).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_registration_api_key
#[php_function]
pub fn todozi_update_registration_api_key(api_key: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_registration_api_key(api_key).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_registration_keys
#[php_function]
pub fn todozi_update_registration_keys(api_key: &str, user_id: Option<String>, fingerprint: Option<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_registration_keys(api_key, user_id.as_deref(), fingerprint.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_reminder
#[php_function]
pub fn todozi_update_reminder(reminder_id: &str, updates: ReminderUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_reminder(reminder_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_summary
#[php_function]
pub fn todozi_update_summary(summary_id: &str, updates: SummaryUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_summary(summary_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_tag
#[php_function]
pub fn todozi_update_tag(tag_id: &str, updates: TagUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_tag(tag_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_task
#[php_function]
pub fn todozi_update_task(id: &str, updates: TaskUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task(id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_task_full
#[php_function]
pub fn todozi_update_task_full(task_id: &str, updates: TaskUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task_full(task_id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_task_in_project
#[php_function]
pub fn todozi_update_task_in_project(id: &str, updates: crate::models::TaskUpdate) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task_in_project(id, updates).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// update_task_status
#[php_function]
pub fn todozi_update_task_status(task_id: &str, status: Status) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task_status(task_id, status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// urgent
#[php_function]
pub fn todozi_urgent(action: &str) -> Result<String, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::urgent(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validate_embeddings
#[php_function]
pub fn todozi_validate_embeddings() -> Result<ValidationReport, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_embeddings().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validate_migration
#[php_function]
pub fn todozi_validate_migration() -> Result<bool, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_migration().await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validate_required_params
#[php_function]
pub fn todozi_validate_required_params(kwargs: HashMap<String, serde_json::Value>, required_params: &[String]) -> Result<Option<ToolResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_required_params(kwargs, required_params).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validate_string_param
#[php_function]
pub fn todozi_validate_string_param(value: &str, param_name: &str, min_length: usize, max_length: usize, pattern: Option<String>) -> Result<Option<ToolResult, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_string_param(value, param_name, min_length, max_length, pattern.as_deref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validate_task_input
#[php_function]
pub fn todozi_validate_task_input(action: &str, time: &str, priority: &str, project: &str, status: &str, assignee: Option<&str>, progress: Option<u8>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_task_input(action, time, priority, project, status, assignee.as_deref(), progress.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validate_tdz_command
#[php_function]
pub fn todozi_validate_tdz_command(command: &str) -> Result<JsCommandValidation, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_tdz_command(command).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// validation
#[php_function]
pub fn todozi_validation(message: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validation(message).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// verbose
#[php_function]
pub fn todozi_verbose(verbose: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::verbose(verbose).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_action
#[php_function]
pub fn todozi_with_action(action: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_action(action).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_assignee
#[php_function]
pub fn todozi_with_assignee(assignee: Assignee) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_assignee(assignee).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_context
#[php_function]
pub fn todozi_with_context(context: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_context(context).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_context_notes
#[php_function]
pub fn todozi_with_context_notes(context_notes: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_context_notes(context_notes).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_dependencies
#[php_function]
pub fn todozi_with_dependencies(dependencies: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_dependencies(dependencies).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_dry_run
#[php_function]
pub fn todozi_with_dry_run(dry_run: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_dry_run(dry_run).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_embedding_service
#[php_function]
pub fn todozi_with_embedding_service(service: TodoziEmbeddingService) -> Result<PyResult<Self, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_embedding_service(service).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_embedding_service_option
#[php_function]
pub fn todozi_with_embedding_service_option(service: Option<TodoziEmbeddingService>) -> Result<PyResult<Self, E>> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_embedding_service_option(service.as_ref()).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_force
#[php_function]
pub fn todozi_with_force(force: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_force(force).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_max_tokens
#[php_function]
pub fn todozi_with_max_tokens(max_tokens: u32) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_max_tokens(max_tokens).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_parent_project
#[php_function]
pub fn todozi_with_parent_project(parent_project: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_parent_project(parent_project).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_priority
#[php_function]
pub fn todozi_with_priority(priority: Priority) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_priority(priority).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_progress
#[php_function]
pub fn todozi_with_progress(progress: u8) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_progress(progress).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_shared_components
#[php_function]
pub fn todozi_with_shared_components(config: Arc<Mutex<TodoziEmbeddingConfig>>, cache: Arc<Mutex<HashMap<String, TodoziEmbeddingCache>>>, embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>, embedding_models: Arc<Mutex<HashMap<String, Arc<EmbeddingModel>>>>, tag_manager: Arc<Mutex<TagManager>>, storage: Arc<Mutex<Storage>>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_shared_components(config, cache, embedding_model, embedding_models, tag_manager, storage).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_status
#[php_function]
pub fn todozi_with_status(status: Status) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_status(status).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_tags
#[php_function]
pub fn todozi_with_tags(tags: Vec<String>) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_tags(tags).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_temperature
#[php_function]
pub fn todozi_with_temperature(temperature: f32) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_temperature(temperature).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_time
#[php_function]
pub fn todozi_with_time(time: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_time(time).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_user_id
#[php_function]
pub fn todozi_with_user_id(user_id: &str) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_user_id(user_id).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// with_verbose
#[php_function]
pub fn todozi_with_verbose(verbose: bool) -> Result<Todozi, E> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_verbose(verbose).await
            .map_err(|e| php_error(E_ALL, e.to_string()))
    })
}

// Module initialization
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
