#![cfg(feature = "nodejs")]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::{Done, Tdz, Actions, Projects, Memories, Ideas, Queue, Find, Emb, Stats, Easy, Tags, init, init_with_auto_registration, todozi_begin, get_tdz_api_key, ensure_todozi_initialized, tdzfp, ChatContent, execute_tdz_command, parse_tdz_command, extract_content, strategy_content};
use crate::models::{Task, Memory, Idea, QueueItem, Priority, Status, Assignee, TaskFilters, TaskUpdate, Reminder, ReminderPriority, ReminderStatus, ApiKey, Config, RegistrationInfo, Project, Tag, Summary};
use crate::emb::{SimilarityResult, ClusteringResult, TodoziEmbeddingService, TodoziEmbeddingConfig};
use crate::storage::{Storage, check_folder_structure, delete_project as storage_delete_project, ensure_folder_structure, get_registration_info, get_storage_dir, init_storage, is_registered, list_projects, load_config, load_project, load_task_collection, register_with_server, save_config, save_project, save_task_collection, add_queue_item as storage_add_queue_item, clear_registration};
use crate::api::{load_api_key_collection};
use crate::reminder::{ReminderManager as ReminderMgr};
use crate::api::{activate_api_key};
use crate::chunking::{ProjectState};
use crate::tags::{TagManager};
use crate::search::{SearchEngine, SearchOptions};
use sha2::Digest;

#[napi]
pub struct Todozi {
    runtime: tokio::runtime::Runtime,
}

#[napi]
impl Todozi {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| Error::from_reason(format!("Runtime error: {}", e)))?;
        Ok(Todozi { runtime })
    }

    // ========== Top-level init functions ==========
    #[napi]
    pub fn todozi_init(&self) -> Result<()> {
        self.runtime.block_on(async {
            init().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn todozi_init_with_auto_registration(&self) -> Result<()> {
        self.runtime.block_on(async {
            init_with_auto_registration().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn todozi_begin(&self) -> Result<()> {
        self.runtime.block_on(async {
            todozi_begin().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn get_tdz_api_key(&self) -> Result<String> {
        self.runtime.block_on(async {
            get_tdz_api_key().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn ensure_todozi_initialized(&self) -> Result<()> {
        self.runtime.block_on(async {
            ensure_todozi_initialized().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn tdzfp(&self) -> Result<bool> {
        tdzfp().map_err(|e| Error::from_reason(format!("{}", e)))
    }

    // ========== Tdz API (10 methods) ==========
    #[napi]
    pub fn task(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Tdz::task(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn urgent(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Tdz::urgent(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn high(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Tdz::high(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn low(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Tdz::low(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn find(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Tdz::find(&query).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn ai_find(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Tdz::ai_find(&query).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async { Tdz::done(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn start(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async { Tdz::start(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn all(&self) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Tdz::all().await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn remember(&self, moment: String, meaning: String) -> Result<JsTask> {
        self.runtime.block_on(async {
            Tdz::remember(&moment, &meaning).await.map(JsTask::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn idea(&self, idea: String) -> Result<JsTask> {
        self.runtime.block_on(async {
            Tdz::idea(&idea).await.map(JsTask::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn chat(&self, message: String) -> Result<String> {
        self.runtime.block_on(async {
            Tdz::chat(&message).await.map(|_| "Chat processed".to_string()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Done API (40+ methods) ==========
    
    #[napi]
    pub fn done_init(&self) -> Result<()> {
        self.runtime.block_on(async {
            Done::init().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_api_key(&self) -> Result<String> {
        self.runtime.block_on(async {
            Done::api_key().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_storage(&self) -> Result<String> {
        self.runtime.block_on(async {
            Done::storage().await.map(|_| "Storage created".to_string()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_embedding_service(&self) -> Result<String> {
        self.runtime.block_on(async {
            Done::embedding_service().await.map(|_| "Embedding service created".to_string()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_types(&self) -> Result<String> {
        Ok(Done::types().to_string())
    }

    #[napi]
    pub fn done_sample_task(&self) -> Result<JsTask> {
        Ok(JsTask::from(Done::sample_task()))
    }

    #[napi]
    pub fn done_embedding_config(&self) -> Result<String> {
        Ok("TodoziEmbeddingConfig default values".to_string())
    }

    #[napi]
    pub fn create_task(&self, action: String, priority: Option<String>, project: Option<String>, time: Option<String>, context: Option<String>) -> Result<JsTask> {
        let priority = priority.and_then(|p| p.parse().ok());
        self.runtime.block_on(async {
            Done::create_task(&action, priority, project.as_deref(), time.as_deref(), context.as_deref())
                .await.map(JsTask::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn search_tasks(&self, query: String, semantic: bool, limit: Option<u32>) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::search_tasks(&query, semantic, limit.map(|l| l as usize)).await
                .map(|tasks| tasks.into_iter().map(JsTask::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn update_task_status(&self, task_id: String, status: String) -> Result<()> {
        let status: Status = status.parse().map_err(|e: crate::error::TodoziError| Error::from_reason(format!("{}", e)))?;
        self.runtime.block_on(async {
            Done::update_task_status(&task_id, status).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn extract_tasks(&self, content: String, context: Option<String>) -> Result<Vec<String>> {
        self.runtime.block_on(async {
            Done::extract_tasks(&content, context.as_deref()).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn plan_tasks(&self, goal: String, complexity: Option<String>, timeline: Option<String>, context: Option<String>) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::plan_tasks(&goal, complexity.as_deref(), timeline.as_deref(), context.as_deref())
                .await.map(|tasks| tasks.into_iter().map(JsTask::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn list_tasks(&self) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::list_tasks().await.map(|tasks| tasks.into_iter().map(JsTask::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn get_task(&self, task_id: String) -> Result<Option<JsTask>> {
        self.runtime.block_on(async {
            Done::get_task(&task_id).await.map(|opt| opt.map(JsTask::from))
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn delete_task(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::delete_task(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn quick_task(&self, action: String) -> Result<JsTask> {
        self.runtime.block_on(async {
            Done::quick_task(&action).await.map(JsTask::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn find_tasks(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::find_tasks(&query).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn find_tasks_ai(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::find_tasks_ai(&query).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn all_tasks(&self) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::all_tasks().await.map(|tasks| tasks.into_iter().map(JsTask::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn complete_task(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::complete_task(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn start_task(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::start_task(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn extract_task_actions(&self, content: String) -> Result<Vec<String>> {
        self.runtime.block_on(async {
            Done::extract_task_actions(&content).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn plan_task_actions(&self, goal: String) -> Result<Vec<String>> {
        self.runtime.block_on(async {
            Done::plan_task_actions(&goal).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_process_chat(&self, message: String, user_id: String) -> Result<String> {
        self.runtime.block_on(async {
            Done::process_chat(&message, &user_id).await
                .map(|content| format!("Chat processed: {} tasks, {} memories, {} ideas", 
                    content.tasks.len(), content.memories.len(), content.ideas.len()))
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_create_storage(&self) -> Result<String> {
        self.runtime.block_on(async {
            Done::create_storage().await.map(|_| "Storage created".to_string()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn done_create_embedding_service(&self) -> Result<String> {
        self.runtime.block_on(async {
            Done::create_embedding_service().await.map(|_| "Embedding service created".to_string()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Actions API (8 methods) ==========
    #[napi]
    pub fn ai_task(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Actions::ai(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn human_task(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Actions::human(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn collab_task(&self, action: String) -> Result<String> {
        self.runtime.block_on(async { Actions::collab(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn complete(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async { Actions::complete(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn delete(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async { Actions::delete(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn get(&self, task_id: String) -> Result<Option<JsTask>> {
        self.runtime.block_on(async {
            Actions::get(&task_id).await.map(|opt| opt.map(JsTask::from)).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn list(&self) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Actions::list().await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn begin(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async { Actions::begin(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Projects API (4 methods) ==========
    #[napi]
    pub fn create_project(&self, name: String, description: Option<String>) -> Result<()> {
        self.runtime.block_on(async { Projects::create(&name, description.as_deref()).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn list_projects(&self) -> Result<Vec<String>> {
        self.runtime.block_on(async { Projects::list().await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn project_tasks(&self, project_name: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Projects::tasks(&project_name).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn delete_project(&self, project_name: String) -> Result<()> {
        self.runtime.block_on(async { Projects::delete(&project_name).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Memories API (4 methods) ==========
    #[napi]
    pub fn create_memory(&self, moment: String, meaning: String, reason: String) -> Result<String> {
        self.runtime.block_on(async { Memories::create(&moment, &meaning, &reason).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn important_memory(&self, moment: String, meaning: String, reason: String) -> Result<String> {
        self.runtime.block_on(async { Memories::important(&moment, &meaning, &reason).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn list_memories(&self) -> Result<Vec<JsMemory>> {
        self.runtime.block_on(async {
            Memories::list().await.map(|memories| memories.into_iter().map(JsMemory::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn find_memories(&self, query: String) -> Result<Vec<JsMemory>> {
        self.runtime.block_on(async {
            Memories::find(&query).await.map(|memories| memories.into_iter().map(JsMemory::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Ideas API (4 methods) ==========
    #[napi]
    pub fn create_idea(&self, idea: String) -> Result<String> {
        self.runtime.block_on(async { Ideas::create(&idea).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn breakthrough_idea(&self, idea: String) -> Result<String> {
        self.runtime.block_on(async { Ideas::breakthrough(&idea).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn list_ideas(&self) -> Result<Vec<JsIdea>> {
        self.runtime.block_on(async {
            Ideas::list().await.map(|ideas| ideas.into_iter().map(JsIdea::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn find_ideas(&self, query: String) -> Result<Vec<JsIdea>> {
        self.runtime.block_on(async {
            Ideas::find(&query).await.map(|ideas| ideas.into_iter().map(JsIdea::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Queue API (6 methods) ==========
    #[napi]
    pub fn queue_add(&self, task_name: String, description: String) -> Result<String> {
        self.runtime.block_on(async { Queue::add(&task_name, &description).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn queue_list(&self) -> Result<Vec<JsQueueItem>> {
        self.runtime.block_on(async {
            Queue::list().await.map(|items| items.into_iter().map(JsQueueItem::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn queue_backlog(&self) -> Result<Vec<JsQueueItem>> {
        self.runtime.block_on(async {
            Queue::backlog().await.map(|items| items.into_iter().map(JsQueueItem::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn queue_active(&self) -> Result<Vec<JsQueueItem>> {
        self.runtime.block_on(async {
            Queue::active().await.map(|items| items.into_iter().map(JsQueueItem::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn queue_start(&self, item_id: String) -> Result<String> {
        self.runtime.block_on(async { Queue::start(&item_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn queue_complete(&self, session_id: String) -> Result<()> {
        self.runtime.block_on(async { Queue::complete(&session_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Find API (9 methods) ==========
    #[napi]
    pub fn tdz_find(&self, query: String) -> Result<String> {
        self.runtime.block_on(async { Find::tdz_find(&query).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn ai_search(&self, query: String) -> Result<Vec<JsSimilarityResult>> {
        self.runtime.block_on(async {
            Find::ai_search(&query).await.map(|results| results.into_iter().map(JsSimilarityResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn keyword_search(&self, query: String) -> Result<String> {
        self.runtime.block_on(async { Find::keyword_search(&query).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn smart_search(&self, query: String) -> Result<String> {
        self.runtime.block_on(async { Find::smart(&query).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn ai_tasks(&self, query: String) -> Result<Vec<JsSimilarityResult>> {
        self.runtime.block_on(async {
            Find::ai_tasks(&query).await.map(|results| results.into_iter().map(JsSimilarityResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn keyword_tasks(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Find::keyword_tasks(&query).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn similar_tasks(&self, task_id: String) -> Result<Vec<JsSimilarityResult>> {
        self.runtime.block_on(async {
            Find::similar_tasks(&task_id).await.map(|results| results.into_iter().map(JsSimilarityResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn fast_search(&self, query: String) -> Result<String> {
        self.runtime.block_on(async { Find::fast(&query).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn deep_search(&self, query: String) -> Result<Vec<JsSimilarityResult>> {
        self.runtime.block_on(async {
            Find::deep(&query).await.map(|results| results.into_iter().map(JsSimilarityResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Emb API (6 methods) ==========
    #[napi(ts_return_type = "number[]")]
    pub fn embed(&self, text: String) -> Result<Vec<f64>> {
        self.runtime.block_on(async { 
            Emb::embed(&text).await
                .map(|vec| vec.into_iter().map(|f| f as f64).collect())
                .map_err(|e| Error::from_reason(format!("{}", e))) 
        })
    }

    #[napi]
    pub fn similar(&self, query: String) -> Result<Vec<JsSimilarityResult>> {
        self.runtime.block_on(async {
            Emb::similar(&query).await.map(|results| results.into_iter().map(JsSimilarityResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn similar_tasks_emb(&self, query: String) -> Result<Vec<JsSimilarityResult>> {
        self.runtime.block_on(async {
            Emb::similar_tasks(&query).await.map(|results| results.into_iter().map(JsSimilarityResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cluster(&self) -> Result<Vec<JsClusteringResult>> {
        self.runtime.block_on(async {
            Emb::cluster().await.map(|results| results.into_iter().map(JsClusteringResult::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn embed_stats(&self) -> Result<String> {
        self.runtime.block_on(async { Emb::stats().await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn embed_task(&self, task_id: String) -> Result<String> {
        self.runtime.block_on(async { Emb::embed_task(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Stats API (2 methods) ==========
    #[napi]
    pub fn stats(&self) -> Result<String> {
        self.runtime.block_on(async { Stats::quick().await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn detailed_stats(&self) -> Result<String> {
        self.runtime.block_on(async { Stats::detailed().await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Easy API (6 methods) ==========
    #[napi]
    pub fn do_it(&self, what: String) -> Result<String> {
        self.runtime.block_on(async { Easy::do_it(&what).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn easy_find(&self, what: String) -> Result<String> {
        self.runtime.block_on(async { Easy::find(&what).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn easy_remember(&self, what: String) -> Result<String> {
        self.runtime.block_on(async { Easy::remember(&what).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn easy_idea(&self, what: String) -> Result<String> {
        self.runtime.block_on(async { Easy::idea(&what).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn easy_done(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async { Easy::done(&task_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn see_all(&self) -> Result<String> {
        self.runtime.block_on(async { Easy::see_all().await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Tags API ==========

    #[napi]
    pub fn find_by_tag(&self, tag_name: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Tags::find(&tag_name).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn add_tag_to_task(&self, task_id: String, tag: String) -> Result<()> {
        self.runtime.block_on(async { Tags::add_to_task(&task_id, &tag).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn remove_tag_from_task(&self, task_id: String, tag: String) -> Result<()> {
        self.runtime.block_on(async { Tags::remove_from_task(&task_id, &tag).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Configuration (2 methods) ==========
    #[napi]
    pub fn set_project(&self, project_name: String) {
        Done::set_project(project_name);
    }

    #[napi]
    pub fn get_project(&self) -> String {
        Done::project_name()
    }

    // ========== Storage API (18+ methods) ==========
    #[napi]
    pub fn storage_init(&self) -> Result<()> {
        self.runtime.block_on(async {
            init_storage().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn storage_check_folder_structure(&self) -> Result<bool> {
        check_folder_structure().map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_ensure_folder_structure(&self) -> Result<bool> {
        self.runtime.block_on(async {
            ensure_folder_structure().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn storage_is_registered(&self) -> Result<bool> {
        self.runtime.block_on(async {
            is_registered().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn storage_clear_registration(&self) -> Result<String> {
        Ok("clear_registration not yet implemented".to_string())
    }

    #[napi]
    pub fn storage_list_projects(&self) -> Result<Vec<String>> {
        list_projects()
            .map(|projects| projects.into_iter().map(|p| p.name).collect())
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_load_project(&self, name: String) -> Result<String> {
        load_project(&name)
            .map(|_| format!("Project '{}' loaded", name))
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_save_project(&self, name: String) -> Result<()> {
        load_project(&name)
            .and_then(|project| save_project(&project))
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_delete_project_by_name(&self, name: String) -> Result<()> {
        storage_delete_project(&name)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_get_storage_dir(&self) -> Result<String> {
        get_storage_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_load_config(&self) -> Result<String> {
        self.runtime.block_on(async {
            load_config().await.map(|_| "Config loaded".to_string()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn storage_save_config(&self) -> Result<()> {
        self.runtime.block_on(async {
            let config = load_config().await.map_err(|e| Error::from_reason(format!("{}", e)))?;
            save_config(&config).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn storage_load_task_collection(&self, name: String) -> Result<String> {
        load_task_collection(&name)
            .map(|_| format!("Task collection '{}' loaded", name))
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_save_task_collection(&self, name: String) -> Result<()> {
        load_task_collection(&name)
            .and_then(|collection| save_task_collection(&name, &collection))
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn storage_get_registration_info(&self) -> Result<String> {
        self.runtime.block_on(async {
            get_registration_info().await
                .map(|info| format!("Registration info available: {}", info.is_some()))
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn storage_register_with_server(&self, server_url: String) -> Result<String> {
        self.runtime.block_on(async {
            register_with_server(&server_url).await
                .map(|info| format!("Registered with server: {}", server_url))
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== TDZ Commands API (3 methods) ==========
    #[napi]
    pub fn tdz_execute_command(&self, command: String) -> Result<String> {
        self.runtime.block_on(async {
            let commands = parse_tdz_command(&command).map_err(|e| Error::from_reason(format!("{}", e)))?;
            if commands.is_empty() {
                return Err(Error::from_reason("No commands found in input"));
            }
            let result = execute_tdz_command(&commands[0], "https://todozi.com/api", None).await
                .map_err(|e| Error::from_reason(format!("{}", e)))?;
            Ok(serde_json::to_string(&result).unwrap_or_else(|_| "Failed to serialize result".to_string()))
        })
    }

    #[napi]
    pub fn tdz_parse_command(&self, input: String) -> Result<String> {
        parse_tdz_command(&input)
            .map(|cmd| format!("Command parsed: {:?}", cmd))
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }

    // ========== Extract API (2 methods) ==========
    #[napi]
    pub fn extract_content(&self, text: String) -> Result<String> {
        self.runtime.block_on(async {
            extract_content(Some(text), None, "json".to_string(), true).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn strategy_content(&self, text: String) -> Result<String> {
        self.runtime.block_on(async {
            strategy_content(Some(text), None, "json".to_string(), true).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Reminder API (methods) ==========

    // ========== Reminder API ==========
    #[napi]
    pub fn activate_reminder(&self, reminder_id: String) -> Result<()> {
        self.runtime.block_on(async {
            let mut reminder_mgr = ReminderMgr::new();
            reminder_mgr.activate_reminder(&reminder_id).await
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }


    // ========== Emb API ==========
    #[napi]
    pub fn add_task_emb(&self, task: JsTask) -> Result<String> {
        self.runtime.block_on(async {
            let mut emb_service = crate::emb::TodoziEmbeddingService::new(
                crate::emb::TodoziEmbeddingConfig::default(),
            )
            .await
            .map_err(|e| Error::from_reason(format!("Failed to create embedding service: {}", e)))?;
            emb_service.initialize().await
                .map_err(|e| Error::from_reason(format!("Failed to initialize embedding service: {}", e)))?;
            emb_service.add_task(task.into()).await
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== TDZ TLS API ==========
    #[napi]
    pub fn add_checklist_item(&self, item: String) -> Result<()> {
        // Placeholder implementation - checklist functionality not implemented
        Ok(())
    }

    #[napi]
    pub fn add_recent_action(&self, action: String) -> Result<()> {
        self.runtime.block_on(async { Actions::add_recent(&action).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    // ========== Tags API ==========
    #[napi]
    pub fn add_tag_relationship(&self, tag1: String, tag2: String) -> Result<()> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            manager.add_tag_relationship(&tag1, &tag2).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn bulk_create_tags(&self, tags: Vec<String>, category: Option<String>) -> Result<Vec<String>> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            manager.bulk_create_tags(tags, category).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn create_tag(&self, name: String, description: Option<String>, category: Option<String>) -> Result<String> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            use crate::models::Tag;
            let tag = Tag {
                id: String::new(),
                name: name.to_string(),
                description,
                color: None,
                category,
                usage_count: 0,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            manager.create_tag(tag).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn get_all_categories(&self) -> Result<Vec<String>> {
        self.runtime.block_on(async {
            let manager = crate::tags::TagManager::new();
            Ok(manager.get_all_categories())
        })
    }

    // ========== Models API ==========
    #[napi]
    pub fn add_item(&self, content: String, priority: String) -> Result<()> {
        let priority_enum = match priority.to_lowercase().as_str() {
            "urgent" => crate::models::Priority::Urgent,
            "high" => crate::models::Priority::High,
            "low" => crate::models::Priority::Low,
            _ => crate::models::Priority::Medium,
        };
        self.runtime.block_on(async {
            Done::create_task(&content, Some(priority_enum), None, None, None).await.map(|_| ()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Storage API ==========
    #[napi]
    pub fn add_queue_item(&self, content: String, priority: String) -> Result<()> {
        use crate::storage::add_queue_item;
        use crate::models::{QueueItem, Priority};
        let priority_enum = match priority.to_lowercase().as_str() {
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        };
        let queue_item = QueueItem::new(content.to_string(), content.to_string(), priority_enum, None);
        self.runtime.block_on(async {
            add_queue_item(queue_item).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn add_task_to_project(&self, task: JsTask) -> Result<()> {
        self.runtime.block_on(async {
            let storage = Storage::new().await?;
            storage.add_task_to_project(task.into()).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn archive_project(&self, project_name: String) -> Result<()> {
        self.runtime.block_on(async {
            let storage = Storage::new().await?;
            storage.archive_project(&project_name).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn clear_registration(&self) -> Result<()> {
        self.runtime.block_on(async {
            crate::storage::clear_registration().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }


    #[napi]
    pub fn load_project(&self, project_name: String) -> Result<JsProject> {
        self.runtime.block_on(async {
            crate::storage::load_project(&project_name).map(JsProject::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn save_project(&self, project: JsProject) -> Result<()> {
        self.runtime.block_on(async {
            crate::storage::save_project(&project.into()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn save_task(&self, task: JsTask) -> Result<()> {
        self.runtime.block_on(async {
            crate::storage::save_task(&task.into()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn load_task(&self, task_id: String) -> Result<JsTask> {
        self.runtime.block_on(async {
            crate::storage::load_task(&task_id).map(JsTask::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Search API ==========
    #[napi]
    pub fn advanced_search(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            let mut search_engine = crate::search::SearchEngine::new();
            // Load some data into the search engine (this is a simplified version)
            let tasks = crate::Done::list_tasks().await.unwrap_or_default();
            for task in &tasks {
                search_engine.tasks.push(task.clone());
            }
            let options = crate::search::SearchOptions::default();
            let results = search_engine.search(&query, options);
            Ok(results.task_results.into_iter().map(|r| JsTask::from(r.task)).collect())
        })
    }


    #[napi]
    pub fn add_key(&self, key: JsApiKey) -> Result<()> {
        self.runtime.block_on(async {
            let mut collection = crate::api::load_api_key_collection()?;
            collection.add_key(key.into());
            crate::api::save_api_key_collection(&collection).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn add_task(&self, task: JsTask) -> Result<()> {
        self.runtime.block_on(async {
            let storage = Storage::new().await?;
            storage.add_task_to_project(task.into()).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Search API ==========
    #[napi]
    pub fn tags_advanced_search(&self, query: String) -> Result<Vec<JsTag>> {
        self.runtime.block_on(async {
            Tags::advanced_search(&query).await
                .map(|tags| tags.into_iter().map(JsTag::from).collect())
                .map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== Chunking API ==========
    #[napi]
    pub fn add_chunk(&self, chunk_id: String, level: String, deps: Vec<String>) -> Result<()> {
        // Placeholder - chunking functionality not fully implemented
        Ok(())
    }

    #[napi]
    pub fn add_completed_module(&self, module: String) -> Result<()> {
        // Placeholder - completed module tracking not implemented
        Ok(())
    }

    #[napi]
    pub fn add_dependency(&self, dep: String) -> Result<()> {
        // Placeholder - dependency functionality not implemented
        Ok(())
    }

    #[napi]
    pub fn add_error_pattern(&self, pattern: String) -> Result<()> {
        // Placeholder - error pattern tracking not implemented
        Ok(())
    }

    #[napi]
    pub fn add_function_signature(&self, name: String, signature: String) -> Result<()> {
        // Placeholder - function signature tracking not implemented
        Ok(())
    }

    #[napi]
    pub fn add_import(&self, import_stmt: String) -> Result<()> {
        // Placeholder - import tracking not implemented
        Ok(())
    }

    #[napi]
    pub fn add_pending_module(&self, module: String) -> Result<()> {
        // Placeholder - pending module tracking not implemented
        Ok(())
    }

    // ========== Agent API ==========
    #[napi]
    pub fn delete_agent(&self, agent_id: String) -> Result<()> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            manager.delete_agent(&agent_id).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== API Key Management ==========
    #[napi]
    pub fn create_api_key(&self) -> Result<JsApiKey> {
        self.runtime.block_on(async {
            crate::api::create_api_key().map(JsApiKey::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn create_api_key_with_user_id(&self, user_id: String) -> Result<JsApiKey> {
        self.runtime.block_on(async {
            crate::api::create_api_key_with_user_id(user_id).map(JsApiKey::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn get_api_key(&self, user_id: String) -> Result<JsApiKey> {
        self.runtime.block_on(async {
            crate::api::get_api_key(&user_id).map(JsApiKey::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn get_api_key_by_public(&self, public_key: String) -> Result<JsApiKey> {
        self.runtime.block_on(async {
            crate::api::get_api_key_by_public(&public_key).map(JsApiKey::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn list_api_keys(&self) -> Result<Vec<JsApiKey>> {
        self.runtime.block_on(async {
            crate::api::list_api_keys().map(|keys| keys.into_iter().map(JsApiKey::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn list_active_api_keys(&self) -> Result<Vec<JsApiKey>> {
        self.runtime.block_on(async {
            crate::api::list_active_api_keys().map(|keys| keys.into_iter().map(JsApiKey::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn check_api_key_auth(&self, public_key: String, private_key: Option<String>) -> Result<JsApiKeyAuth> {
        self.runtime.block_on(async {
            crate::api::check_api_key_auth(&public_key, private_key.as_deref()).map(|(user_id, is_admin)| JsApiKeyAuth { user_id, is_admin }).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn deactivate_api_key(&self, user_id: String) -> Result<()> {
        self.runtime.block_on(async { crate::api::deactivate_api_key(&user_id).await.map_err(|e| Error::from_reason(format!("{}", e))) })
    }

    #[napi]
    pub fn remove_api_key(&self, user_id: String) -> Result<JsApiKey> {
        self.runtime.block_on(async {
            crate::api::remove_api_key(&user_id).map(JsApiKey::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    // ========== CLI Operations ==========
    #[napi]
    pub fn cli_add_task(&self, content: String, priority: Option<String>) -> Result<()> {
        self.runtime.block_on(async {
            let priority = priority.as_deref().and_then(|p| match p {
                "urgent" => Some(Priority::Urgent),
                "high" => Some(Priority::High),
                "medium" => Some(Priority::Medium),
                "low" => Some(Priority::Low),
                _ => None,
            });
            Done::create_task(&content, priority, None, None, None).await.map(|_| ()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_list_tasks(&self) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::search_tasks("", false, None).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_show_task(&self, task_id: String) -> Result<JsTask> {
        self.runtime.block_on(async {
            Done::init().await?;
            let storage = Storage::new().await?;
            storage.get_task_from_any_project(&task_id).map(JsTask::from).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_update_task(&self, task_id: String, action: Option<String>, priority: Option<String>, status: Option<String>) -> Result<()> {
        self.runtime.block_on(async {
            let status = status.as_deref().and_then(|s| match s {
                "todo" => Some(Status::Todo),
                "in_progress" => Some(Status::InProgress),
                "done" => Some(Status::Done),
                _ => None,
            });
            if let Some(status) = status {
                Done::update_task_status(&task_id, status).await.map_err(|e| Error::from_reason(format!("{}", e)))
            } else {
                Ok(())
            }
        })
    }

    #[napi]
    pub fn cli_complete_task(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::update_task_status(&task_id, Status::Done).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_delete_task(&self, task_id: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::init().await?;
            let storage = Storage::new().await?;
            storage.delete_task_from_project(&task_id).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_search_tasks(&self, query: String) -> Result<Vec<JsTask>> {
        self.runtime.block_on(async {
            Done::search_tasks(&query, false, None).await.map(|tasks| tasks.into_iter().map(JsTask::from).collect()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_create_backup(&self) -> Result<String> {
        self.runtime.block_on(async {
            Done::init().await?;
            let storage = Storage::new().await?;
            storage.create_backup().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_list_backups(&self) -> Result<Vec<String>> {
        self.runtime.block_on(async {
            Done::init().await?;
            let storage = Storage::new().await?;
            storage.list_backups().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_restore_backup(&self, backup_name: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::init().await?;
            let storage = Storage::new().await?;
            storage.restore_backup(&backup_name).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_fix_consistency(&self) -> Result<()> {
        self.runtime.block_on(async {
            // This is a placeholder - the actual implementation would need to be added
            Ok(())
        })
    }

    #[napi]
    pub fn cli_create_memory(&self, moment: String, meaning: String, reason: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::create_memory(&moment, &meaning, &reason).await.map(|_| ()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_create_idea(&self, content: String) -> Result<()> {
        self.runtime.block_on(async {
            Done::create_idea(&content, None).await.map(|_| ()).map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_chat(&self, message: String) -> Result<String> {
        self.runtime.block_on(async {
            Tdz::chat(&message).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_register_with_server(&self, server_url: String) -> Result<()> {
        self.runtime.block_on(async {
            register_with_server(&server_url).await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }

    #[napi]
    pub fn cli_get_registration_status(&self) -> Result<Option<JsRegistrationInfo>> {
        self.runtime.block_on(async {
            let info = get_registration_info().await.map_err(|e| Error::from_reason(format!("{}", e)))?;
            Ok(info.map(JsRegistrationInfo::from))
        })
    }

    #[napi]
    pub fn cli_clear_registration(&self) -> Result<()> {
        self.runtime.block_on(async {
            clear_registration().await.map_err(|e| Error::from_reason(format!("{}", e)))
        })
    }
}

#[napi(object)]
pub struct JsTask {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub time: String,
    pub priority: String,
    pub status: String,
    pub parent_project: String,
    pub tags: Vec<String>,
    pub progress: Option<u8>,
    pub created_at: String,
}

impl From<Task> for JsTask {
    fn from(task: Task) -> Self {
        JsTask {
            id: task.id,
            user_id: task.user_id,
            action: task.action,
            time: task.time,
            priority: task.priority.to_string(),
            status: task.status.to_string(),
            parent_project: task.parent_project,
            tags: task.tags,
            progress: task.progress,
            created_at: task.created_at.to_rfc3339(),
        }
    }
}

impl From<JsTask> for Task {
    fn from(jstask: JsTask) -> Self {
        Task {
            id: jstask.id,
            user_id: jstask.user_id,
            action: jstask.action,
            time: jstask.time,
            priority: jstask.priority.parse().unwrap_or(Priority::Medium),
            status: jstask.status.parse().unwrap_or(Status::Todo),
            assignee: None, // Not exposed in JsTask
            parent_project: jstask.parent_project,
            tags: jstask.tags,
            dependencies: Vec::new(), // Not exposed in JsTask
            context_notes: None, // Not exposed in JsTask
            progress: jstask.progress,
            embedding_vector: None, // Not exposed in JsTask
            created_at: chrono::DateTime::parse_from_rfc3339(&jstask.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::Utc::now(),
        }
    }
}

#[napi(object)]
pub struct JsMemory {
    pub id: String,
    pub moment: String,
    pub meaning: String,
    pub reason: String,
    pub importance: String,
    pub tags: Vec<String>,
}

impl From<Memory> for JsMemory {
    fn from(memory: Memory) -> Self {
        JsMemory {
            id: memory.id,
            moment: memory.moment,
            meaning: memory.meaning,
            reason: memory.reason,
            importance: memory.importance.to_string(),
            tags: memory.tags,
        }
    }
}

#[napi(object)]
pub struct JsIdea {
    pub id: String,
    pub idea: String,
    pub importance: String,
    pub tags: Vec<String>,
}

impl From<Idea> for JsIdea {
    fn from(idea: Idea) -> Self {
        JsIdea {
            id: idea.id,
            idea: idea.idea,
            importance: idea.importance.to_string(),
            tags: idea.tags,
        }
    }
}

#[napi(object)]
pub struct JsQueueItem {
    pub id: String,
    pub task_name: String,
    pub task_description: String,
    pub priority: String,
    pub status: String,
}

impl From<QueueItem> for JsQueueItem {
    fn from(item: QueueItem) -> Self {
        JsQueueItem {
            id: item.id,
            task_name: item.task_name,
            task_description: item.task_description,
            priority: item.priority.to_string(),
            status: item.status.to_string(),
        }
    }
}

#[napi(object)]
pub struct JsSimilarityResult {
    pub content_id: String,
    pub similarity_score: f64,
    pub text_content: String,
    pub tags: Vec<String>,
}

impl From<SimilarityResult> for JsSimilarityResult {
    fn from(result: SimilarityResult) -> Self {
        JsSimilarityResult {
            content_id: result.content_id,
            similarity_score: result.similarity_score as f64,
            text_content: result.text_content,
            tags: result.tags,
        }
    }
}

#[napi(object)]
pub struct JsClusteringResult {
    pub cluster_id: String,
    pub cluster_size: u32,
    pub average_similarity: f64,
}

impl From<ClusteringResult> for JsClusteringResult {
    fn from(result: ClusteringResult) -> Self {
        JsClusteringResult {
            cluster_id: result.cluster_id,
            cluster_size: result.cluster_size as u32,
            average_similarity: result.average_similarity as f64,
        }
    }
}

#[napi(object)]
pub struct JsReminder {
    pub id: String,
    pub content: String,
    pub remind_at: String,
    pub priority: String,
    pub status: String,
    pub tags: Vec<String>,
    pub created_at: String,
}

impl From<Reminder> for JsReminder {
    fn from(reminder: Reminder) -> Self {
        JsReminder {
            id: reminder.id,
            content: reminder.content,
            remind_at: reminder.remind_at.to_rfc3339(),
            priority: reminder.priority.to_string(),
            status: reminder.status.to_string(),
            tags: reminder.tags,
            created_at: reminder.created_at.to_rfc3339(),
        }
    }
}

#[napi(object)]
pub struct JsApiKey {
    pub user_id: String,
    pub public_key: String,
    pub active: bool,
}

impl From<ApiKey> for JsApiKey {
    fn from(key: ApiKey) -> Self {
        JsApiKey {
            user_id: key.user_id,
            public_key: key.public_key,
            active: key.active,
        }
    }
}

impl From<JsApiKey> for ApiKey {
    fn from(key: JsApiKey) -> Self {
        let now = chrono::Utc::now();
        let private_key = format!("{:x}", sha2::Sha512::digest(key.public_key.as_bytes()));
        ApiKey {
            user_id: key.user_id,
            public_key: key.public_key,
            private_key,
            active: key.active,
            created_at: now,
            updated_at: now,
        }
    }
}

#[napi(object)]
pub struct JsApiKeyAuth {
    pub user_id: String,
    pub is_admin: bool,
}

#[napi(object)]
pub struct JsConfig {
    pub version: String,
    pub default_project: String,
    pub auto_backup: bool,
    pub backup_interval: String,
    pub ai_enabled: bool,
    pub date_format: String,
    pub timezone: String,
}

impl From<Config> for JsConfig {
    fn from(config: Config) -> Self {
        JsConfig {
            version: config.version,
            default_project: config.default_project,
            auto_backup: config.auto_backup,
            backup_interval: config.backup_interval,
            ai_enabled: config.ai_enabled,
            date_format: config.date_format,
            timezone: config.timezone,
        }
    }
}

#[napi(object)]
pub struct JsRegistrationInfo {
    pub api_key: String,
    pub user_id: Option<String>,
    pub fingerprint: Option<String>,
}

impl From<RegistrationInfo> for JsRegistrationInfo {
    fn from(info: RegistrationInfo) -> Self {
        JsRegistrationInfo {
            api_key: info.api_key,
            user_id: info.user_id,
            fingerprint: info.fingerprint,
        }
    }
}

#[napi(object)]
pub struct JsProject {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
}

impl From<Project> for JsProject {
    fn from(project: Project) -> Self {
        JsProject {
            name: project.name,
            description: project.description,
            status: project.status.to_string(),
            created_at: project.created_at.to_rfc3339(),
        }
    }
}

impl From<JsProject> for Project {
    fn from(project: JsProject) -> Self {
        use crate::models::ProjectStatus;
        let status = match project.status.to_lowercase().as_str() {
            "active" => ProjectStatus::Active,
            "completed" => ProjectStatus::Completed,
            "on_hold" => ProjectStatus::OnHold,
            "cancelled" => ProjectStatus::Cancelled,
            _ => ProjectStatus::Active,
        };
        Project {
            name: project.name,
            description: project.description,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            status,
            tasks: Vec::new(), // Not exposed in JsProject
        }
    }
}

#[napi(object)]
pub struct JsTag {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub category: Option<String>,
    pub usage_count: u32,
}

impl From<Tag> for JsTag {
    fn from(tag: Tag) -> Self {
        JsTag {
            id: tag.id,
            name: tag.name,
            description: tag.description,
            color: tag.color,
            category: tag.category,
            usage_count: tag.usage_count,
        }
    }
}

#[napi(object)]
pub struct JsSummary {
    pub id: String,
    pub content: String,
    pub priority: String,
    pub created_at: String,
}

impl From<Summary> for JsSummary {
    fn from(summary: Summary) -> Self {
        JsSummary {
            id: summary.id,
            content: summary.content,
            priority: summary.priority.to_string(),
            created_at: summary.created_at.to_rfc3339(),
        }
    }
}
