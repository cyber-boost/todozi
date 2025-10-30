#![cfg(feature = "python")]

use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use crate::{Done, Tdz, Actions, Projects, Memories, Ideas, Queue, Find, Emb, Stats, Easy, Tags, init, init_with_auto_registration, todozi_begin, get_tdz_api_key, ensure_todozi_initialized, tdzfp, ApiKeys, Tasks, Checklist, Chunking};
use crate::models::{Task, Memory, Idea, QueueItem, Priority, Status, Reminder, ApiKey, Config, RegistrationInfo, Project, Tag, Summary};
use crate::emb::{SimilarityResult, ClusteringResult, TodoziEmbeddingService, TodoziEmbeddingConfig};
use crate::storage::{Storage, check_folder_structure, delete_project as storage_delete_project, ensure_folder_structure, get_registration_info, get_storage_dir, init_storage, is_registered, list_projects, load_config, load_project, load_task_collection, register_with_server, save_config, save_project, save_task_collection};
use crate::api::{load_api_key_collection, activate_api_key};
use crate::reminder::{ReminderManager as ReminderMgr};
use crate::tags::{TagManager};
use crate::search::{SearchEngine, SearchOptions};

#[pyclass]
pub struct PyTodozi {
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyTodozi {
    #[new]
    pub fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyException::new_err(format!("Runtime error: {}", e)))?;
        Ok(PyTodozi { runtime })
    }

    // ========== Top-level init functions ==========
    pub fn todozi_init(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            init().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn todozi_init_with_auto_registration(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            init_with_auto_registration().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn todozi_begin(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            todozi_begin().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_tdz_api_key(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            get_tdz_api_key().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn ensure_todozi_initialized(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            ensure_todozi_initialized().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn tdzfp(&self) -> PyResult<bool> {
        tdzfp().map_err(|e| PyException::new_err(format!("{}", e)))
    }

    // ========== Tdz API (10 methods) ==========
    pub fn task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Tdz::task(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn urgent(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Tdz::urgent(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn high(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Tdz::high(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn low(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Tdz::low(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn find(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tdz::find(&query).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn ai_find(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tdz::ai_find(&query).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Tdz::done(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn start(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Tdz::start(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn all(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tdz::all().await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn remember(&self, moment: String, meaning: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Tdz::remember(&moment, &meaning).await.map(PyTask::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn idea(&self, idea: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Tdz::idea(&idea).await.map(PyTask::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn chat(&self, message: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Tdz::chat(&message).await.map(|_| "Chat processed".to_string()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Done API (40+ methods) ==========
    
    pub fn done_init(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_api_key(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::api_key().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_storage(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::storage().await.map(|_| "Storage created".to_string()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_embedding_service(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::embedding_service().await.map(|_| "Embedding service created".to_string()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_types(&self) -> PyResult<String> {
        Ok(Done::types().to_string())
    }

    pub fn done_sample_task(&self) -> PyResult<PyTask> {
        Ok(PyTask::from(Done::sample_task()))
    }

    pub fn done_embedding_config(&self) -> PyResult<String> {
        Ok("TodoziEmbeddingConfig default values".to_string())
    }

    pub fn create_task(&self, action: String, priority: Option<String>, project: Option<String>, time: Option<String>, context: Option<String>) -> PyResult<PyTask> {
        let priority = priority.and_then(|p| p.parse().ok());
        self.runtime.block_on(async {
            Done::create_task(&action, priority, project.as_deref(), time.as_deref(), context.as_deref())
                .await.map(PyTask::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn search_tasks(&self, query: String, semantic: bool, limit: Option<usize>) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::search_tasks(&query, semantic, limit).await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn update_task_status(&self, task_id: String, status: String) -> PyResult<()> {
        let status: Status = status.parse().map_err(|e: crate::error::TodoziError| PyException::new_err(format!("{}", e)))?;
        self.runtime.block_on(async {
            Done::update_task_status(&task_id, status).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn extract_tasks(&self, content: String, context: Option<String>) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::extract_tasks(&content, context.as_deref()).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn plan_tasks(&self, goal: String, complexity: Option<String>, timeline: Option<String>, context: Option<String>) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::plan_tasks(&goal, complexity.as_deref(), timeline.as_deref(), context.as_deref())
                .await.map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn list_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::list_tasks().await.map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_task(&self, task_id: String) -> PyResult<Option<PyTask>> {
        self.runtime.block_on(async {
            Done::get_task(&task_id).await.map(|opt| opt.map(PyTask::from))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn delete_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_task(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn quick_task(&self, action: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::quick_task(&action).await.map(PyTask::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_tasks(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::find_tasks(&query).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_tasks_ai(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::find_tasks_ai(&query).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn all_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::all_tasks().await.map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn complete_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::complete_task(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn start_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::start_task(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn extract_task_actions(&self, content: String) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::extract_task_actions(&content).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn plan_task_actions(&self, goal: String) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::plan_task_actions(&goal).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_process_chat(&self, message: String, user_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::process_chat(&message, &user_id).await
                .map(|content| format!("Chat processed: {} tasks, {} memories, {} ideas",
                    content.tasks.len(), content.memories.len(), content.ideas.len()))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn tdz_cnt(&self, content: String, session_id: Option<String>) -> PyResult<String> {
        self.runtime.block_on(async {
            use crate::tdz_tls::tdz_cnt;
            tdz_cnt(&content, session_id.as_deref()).await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_create_storage(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_storage().await.map(|_| "Storage created".to_string()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_create_embedding_service(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_embedding_service().await.map(|_| "Embedding service created".to_string()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Actions API (8 methods) ==========
    pub fn ai_task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Actions::ai(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn human_task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Actions::human(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn collab_task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async { Actions::collab(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn complete(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Actions::complete(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn delete(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Actions::delete(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn get(&self, task_id: String) -> PyResult<Option<PyTask>> {
        self.runtime.block_on(async {
            Actions::get(&task_id).await.map(|opt| opt.map(PyTask::from)).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn list(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Actions::list().await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn begin(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Actions::begin(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Projects API (4 methods) ==========
    pub fn create_project(&self, name: String, description: Option<String>) -> PyResult<()> {
        self.runtime.block_on(async { Projects::create(&name, description.as_deref()).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn list_projects(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async { Projects::list().await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn project_tasks(&self, project_name: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Projects::tasks(&project_name).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn delete_project(&self, project_name: String) -> PyResult<()> {
        self.runtime.block_on(async { Projects::delete(&project_name).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Memories API (4 methods) ==========
    pub fn create_memory(&self, moment: String, meaning: String, reason: String) -> PyResult<String> {
        self.runtime.block_on(async { Memories::create(&moment, &meaning, &reason).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn important_memory(&self, moment: String, meaning: String, reason: String) -> PyResult<String> {
        self.runtime.block_on(async { Memories::important(&moment, &meaning, &reason).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn list_memories(&self) -> PyResult<Vec<PyMemory>> {
        self.runtime.block_on(async {
            Memories::list().await.map(|memories| memories.into_iter().map(PyMemory::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_memories(&self, query: String) -> PyResult<Vec<PyMemory>> {
        self.runtime.block_on(async {
            Memories::find(&query).await.map(|memories| memories.into_iter().map(PyMemory::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Ideas API (4 methods) ==========
    pub fn create_idea(&self, idea: String) -> PyResult<String> {
        self.runtime.block_on(async { Ideas::create(&idea).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn breakthrough_idea(&self, idea: String) -> PyResult<String> {
        self.runtime.block_on(async { Ideas::breakthrough(&idea).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn list_ideas(&self) -> PyResult<Vec<PyIdea>> {
        self.runtime.block_on(async {
            Ideas::list().await.map(|ideas| ideas.into_iter().map(PyIdea::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_ideas(&self, query: String) -> PyResult<Vec<PyIdea>> {
        self.runtime.block_on(async {
            Ideas::find(&query).await.map(|ideas| ideas.into_iter().map(PyIdea::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Queue API (6 methods) ==========
    pub fn queue_add(&self, task_name: String, description: String) -> PyResult<String> {
        self.runtime.block_on(async { Queue::add(&task_name, &description).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn queue_list(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::list().await.map(|items| items.into_iter().map(PyQueueItem::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn queue_backlog(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::backlog().await.map(|items| items.into_iter().map(PyQueueItem::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn queue_active(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::active().await.map(|items| items.into_iter().map(PyQueueItem::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn queue_start(&self, item_id: String) -> PyResult<String> {
        self.runtime.block_on(async { Queue::start(&item_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn queue_complete(&self, session_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Queue::complete(&session_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Find API (9 methods) ==========
    pub fn tdz_find(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async { Find::tdz_find(&query).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn ai_search(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::ai_search(&query).await.map(|results| results.into_iter().map(PySimilarityResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn keyword_search(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async { Find::keyword_search(&query).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn smart_search(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async { Find::smart(&query).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn ai_tasks(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::ai_tasks(&query).await.map(|results| results.into_iter().map(PySimilarityResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn keyword_tasks(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Find::keyword_tasks(&query).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn similar_tasks(&self, task_id: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::similar_tasks(&task_id).await.map(|results| results.into_iter().map(PySimilarityResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn fast_search(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async { Find::fast(&query).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn deep_search(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::deep(&query).await.map(|results| results.into_iter().map(PySimilarityResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Emb API (6 methods) ==========
    pub fn embed(&self, text: String) -> PyResult<Vec<f64>> {
        self.runtime.block_on(async { 
            Emb::embed(&text).await
                .map(|vec| vec.into_iter().map(|f| f as f64).collect())
                .map_err(|e| PyException::new_err(format!("{}", e))) 
        })
    }

    pub fn similar(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Emb::similar(&query).await.map(|results| results.into_iter().map(PySimilarityResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn similar_tasks_emb(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Emb::similar_tasks(&query).await.map(|results| results.into_iter().map(PySimilarityResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn cluster(&self) -> PyResult<Vec<PyClusteringResult>> {
        self.runtime.block_on(async {
            Emb::cluster().await.map(|results| results.into_iter().map(PyClusteringResult::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn embed_stats(&self) -> PyResult<String> {
        self.runtime.block_on(async { Emb::stats().await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn embed_task(&self, task_id: String) -> PyResult<String> {
        self.runtime.block_on(async { Emb::embed_task(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Stats API (2 methods) ==========
    pub fn stats(&self) -> PyResult<String> {
        self.runtime.block_on(async { Stats::quick().await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn detailed_stats(&self) -> PyResult<String> {
        self.runtime.block_on(async { Stats::detailed().await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Easy API (6 methods) ==========
    pub fn do_it(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async { Easy::do_it(&what).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn easy_find(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async { Easy::find(&what).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn easy_remember(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async { Easy::remember(&what).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn easy_idea(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async { Easy::idea(&what).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn easy_done(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async { Easy::done(&task_id).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn see_all(&self) -> PyResult<String> {
        self.runtime.block_on(async { Easy::see_all().await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Tags API (4 methods) ==========

    pub fn find_by_tag(&self, tag_name: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tags::find(&tag_name).await.map(|tasks| tasks.into_iter().map(PyTask::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn add_tag_to_task(&self, task_id: String, tag: String) -> PyResult<()> {
        self.runtime.block_on(async { Tags::add_to_task(&task_id, &tag).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn remove_tag_from_task(&self, task_id: String, tag: String) -> PyResult<()> {
        self.runtime.block_on(async { Tags::remove_from_task(&task_id, &tag).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Configuration (2 methods) ==========
    pub fn set_project(&self, project_name: String) {
        Done::set_project(project_name);
    }

    pub fn get_project(&self) -> String {
        Done::project_name()
    }

    // ========== Storage API (18+ methods) ==========
    pub fn storage_init(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            init_storage().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_check_folder_structure(&self) -> PyResult<bool> {
        check_folder_structure().map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_ensure_folder_structure(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            ensure_folder_structure().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_is_registered(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            is_registered().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_clear_registration(&self) -> PyResult<String> {
        Ok("clear_registration not yet implemented".to_string())
    }

    pub fn storage_list_projects(&self) -> PyResult<Vec<String>> {
        list_projects()
            .map(|projects| projects.into_iter().map(|p| p.name).collect())
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_load_project(&self, name: String) -> PyResult<String> {
        load_project(&name)
            .map(|_| format!("Project '{}' loaded", name))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_save_project(&self, name: String) -> PyResult<()> {
        load_project(&name)
            .and_then(|project| save_project(&project))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_delete_project_by_name(&self, name: String) -> PyResult<()> {
        storage_delete_project(&name)
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_get_storage_dir(&self) -> PyResult<String> {
        get_storage_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_load_config(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            load_config().await.map(|_| "Config loaded".to_string()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_save_config(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let config = load_config().await.map_err(|e| PyException::new_err(format!("{}", e)))?;
            save_config(&config).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_load_task_collection(&self, name: String) -> PyResult<String> {
        load_task_collection(&name)
            .map(|_| format!("Task collection '{}' loaded", name))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_save_task_collection(&self, name: String) -> PyResult<()> {
        load_task_collection(&name)
            .and_then(|collection| save_task_collection(&name, &collection))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_get_registration_info(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            get_registration_info().await
                .map(|info| format!("Registration info available: {}", info.is_some()))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_register_with_server(&self, server_url: String) -> PyResult<String> {
        self.runtime.block_on(async {
            register_with_server(&server_url).await
                .map(|_info| format!("Registered with server: {}", server_url))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Reminder API ==========
    pub fn activate_reminder(&self, reminder_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut reminder_mgr = ReminderMgr::new();
            reminder_mgr.activate_reminder(&reminder_id).await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn active_percentage(&self) -> PyResult<f64> {
        let reminder_mgr = ReminderMgr::new();
        let stats = reminder_mgr.get_reminder_statistics();
        Ok(stats.active_percentage())
    }

    // ========== API Key Management ==========
    pub fn activate_api_key(&self, user_id: String) -> PyResult<()> {
        activate_api_key(&user_id)
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn activate_key(&self, user_id: String) -> PyResult<bool> {
        let mut collection = load_api_key_collection()
            .map_err(|e| PyException::new_err(format!("{}", e)))?;
        Ok(collection.activate_key(&user_id))
    }

    // ========== Chunking API ==========
    pub fn add_chunk(&self, chunk_id: String, level: String, deps: Vec<String>) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_chunk(chunk_id, level, deps).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_completed_module(&self, module: String) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_completed_module(module).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_dependency(&self, dep: String) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_dependency(dep).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_error_pattern(&self, pattern: String) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_error_pattern(pattern).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_function_signature(&self, name: String, signature: String) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_function_signature(name, signature).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_import(&self, import_stmt: String) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_import(import_stmt).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_pending_module(&self, module: String) -> PyResult<()> {
        self.runtime.block_on(async { Chunking::add_pending_module(module).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Models API ==========
    pub fn add_item(&self, content: String, priority: String) -> PyResult<()> {
        let priority_enum = match priority.to_lowercase().as_str() {
            "urgent" => Priority::Urgent,
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        };
        self.runtime.block_on(async {
            Done::create_task(&content, Some(priority_enum), None, None, None).await.map(|_| ()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn add_key(&self, key: PyApiKey) -> PyResult<()> {
        self.runtime.block_on(async { ApiKeys::add(key.into()).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_task(&self, task: PyTask) -> PyResult<()> {
        self.runtime.block_on(async { Tasks::add(task.into()).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Emb API ==========
    pub fn add_task_emb(&self, task: PyTask) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut emb_service = TodoziEmbeddingService::new(
                TodoziEmbeddingConfig::default(),
            )
            .await
            .map_err(|e| PyException::new_err(format!("Failed to create embedding service: {}", e)))?;
            emb_service.initialize().await
                .map_err(|e| PyException::new_err(format!("Failed to initialize embedding service: {}", e)))?;
            emb_service.add_task(task.into()).await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== TDZ TLS API ==========
    pub fn add_checklist_item(&self, item: String) -> PyResult<()> {
        self.runtime.block_on(async { Checklist::add_item(&item).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    pub fn add_recent_action(&self, action: String) -> PyResult<()> {
        self.runtime.block_on(async { Actions::add_recent(&action).await.map_err(|e| PyException::new_err(format!("{}", e))) })
    }

    // ========== Tags API ==========
    pub fn add_tag_relationship(&self, tag1: String, tag2: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            manager.add_tag_relationship(&tag1, &tag2).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn bulk_create_tags(&self, tags: Vec<String>, category: Option<String>) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            manager.bulk_create_tags(tags, category).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn create_tag(&self, name: String, description: Option<String>, category: Option<String>) -> PyResult<String> {
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
            manager.create_tag(tag).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_all_categories(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let manager = crate::tags::TagManager::new();
            Ok(manager.get_all_categories())
        })
    }

    // ========== Storage API ==========
    pub fn add_queue_item(&self, content: String, priority: String) -> PyResult<()> {
        use crate::storage::add_queue_item;
        use crate::models::{QueueItem, Priority};
        let priority_enum = match priority.to_lowercase().as_str() {
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        };
        let queue_item = QueueItem::new(content.to_string(), content.to_string(), priority_enum, None);
        self.runtime.block_on(async {
            add_queue_item(queue_item).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn add_task_to_project(&self, task: PyTask) -> PyResult<()> {
        self.runtime.block_on(async {
            let storage = Storage::new().await.map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.add_task_to_project(task.into()).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn archive_project(&self, project_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let storage = Storage::new().await.map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.archive_project(&project_name).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn clear_registration(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::storage::clear_registration().await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn load_project(&self, project_name: String) -> PyResult<PyProject> {
        self.runtime.block_on(async {
            crate::storage::load_project(&project_name).map(PyProject::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn save_project(&self, project: PyProject) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::storage::save_project(&project.into()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn save_task(&self, task: PyTask) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::storage::save_task(&task.into()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn load_task(&self, task_id: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            crate::storage::load_task(&task_id).map(PyTask::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Agent API ==========
    pub fn delete_agent(&self, agent_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            manager.delete_agent(&agent_id).await.map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== API Key Management ==========
    pub fn create_api_key(&self) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::create_api_key().map(PyApiKey::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn create_api_key_with_user_id(&self, user_id: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::create_api_key_with_user_id(user_id).map(PyApiKey::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_api_key(&self, user_id: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::get_api_key(&user_id).map(PyApiKey::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_api_key_by_public(&self, public_key: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::get_api_key_by_public(&public_key).map(PyApiKey::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn list_api_keys(&self) -> PyResult<Vec<PyApiKey>> {
        self.runtime.block_on(async {
            crate::api::list_api_keys().map(|keys| keys.into_iter().map(PyApiKey::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn list_active_api_keys(&self) -> PyResult<Vec<PyApiKey>> {
        self.runtime.block_on(async {
            crate::api::list_active_api_keys().map(|keys| keys.into_iter().map(PyApiKey::from).collect()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn check_api_key_auth(&self, public_key: String, private_key: Option<String>) -> PyResult<(String, bool)> {
        self.runtime.block_on(async {
            crate::api::check_api_key_auth(&public_key, private_key.as_deref()).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn deactivate_api_key(&self, user_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::api::deactivate_api_key(&user_id).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn remove_api_key(&self, user_id: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::remove_api_key(&user_id).map(PyApiKey::from).map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }


    // ========== Search API ==========
    pub fn advanced_search(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let mut search_engine = SearchEngine::new();
            // Load some data into the search engine (this is a simplified version)
            let tasks = Done::list_tasks().await.unwrap_or_default();
            for task in &tasks {
                search_engine.tasks.push(task.clone());
            }
            let options = SearchOptions::default();
            let results = search_engine.search(&query, options);
            Ok(results.task_results.into_iter().map(|r| PyTask::from(r.task)).collect())
        })
    }

    pub fn tags_advanced_search(&self, query: String) -> PyResult<Vec<PyTag>> {
        self.runtime.block_on(async {
            Tags::advanced_search(&query).await
                .map(|tags| tags.into_iter().map(PyTag::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

}

// ========== Data Types ==========

#[pyclass]
#[derive(Clone)]
pub struct PyTask {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub user_id: String,
    #[pyo3(get)]
    pub action: String,
    #[pyo3(get)]
    pub time: String,
    #[pyo3(get)]
    pub priority: String,
    #[pyo3(get)]
    pub status: String,
    #[pyo3(get)]
    pub parent_project: String,
    #[pyo3(get)]
    pub tags: Vec<String>,
    #[pyo3(get)]
    pub progress: Option<u8>,
    #[pyo3(get)]
    pub created_at: String,
}

impl From<Task> for PyTask {
    fn from(task: Task) -> Self {
        PyTask {
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

impl From<PyTask> for Task {
    fn from(pytask: PyTask) -> Self {
        Task {
            id: pytask.id,
            user_id: pytask.user_id,
            action: pytask.action,
            time: pytask.time,
            priority: pytask.priority.parse().unwrap_or(Priority::Medium),
            status: pytask.status.parse().unwrap_or(Status::Todo),
            assignee: None, // Not exposed in PyTask
            parent_project: pytask.parent_project,
            tags: pytask.tags,
            dependencies: Vec::new(), // Not exposed in PyTask
            context_notes: None, // Not exposed in PyTask
            progress: pytask.progress,
            embedding_vector: None, // Not exposed in PyTask
            created_at: chrono::DateTime::parse_from_rfc3339(&pytask.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::Utc::now(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyMemory {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub moment: String,
    #[pyo3(get)]
    pub meaning: String,
    #[pyo3(get)]
    pub reason: String,
    #[pyo3(get)]
    pub importance: String,
    #[pyo3(get)]
    pub tags: Vec<String>,
}

impl From<Memory> for PyMemory {
    fn from(memory: Memory) -> Self {
        PyMemory {
            id: memory.id,
            moment: memory.moment,
            meaning: memory.meaning,
            reason: memory.reason,
            importance: memory.importance.to_string(),
            tags: memory.tags,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyIdea {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub idea: String,
    #[pyo3(get)]
    pub importance: String,
    #[pyo3(get)]
    pub tags: Vec<String>,
}

impl From<Idea> for PyIdea {
    fn from(idea: Idea) -> Self {
        PyIdea {
            id: idea.id,
            idea: idea.idea,
            importance: idea.importance.to_string(),
            tags: idea.tags,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyQueueItem {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub task_name: String,
    #[pyo3(get)]
    pub task_description: String,
    #[pyo3(get)]
    pub priority: String,
    #[pyo3(get)]
    pub status: String,
}

impl From<QueueItem> for PyQueueItem {
    fn from(item: QueueItem) -> Self {
        PyQueueItem {
            id: item.id,
            task_name: item.task_name,
            task_description: item.task_description,
            priority: item.priority.to_string(),
            status: item.status.to_string(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PySimilarityResult {
    #[pyo3(get)]
    pub content_id: String,
    #[pyo3(get)]
    pub similarity_score: f32,
    #[pyo3(get)]
    pub text_content: String,
    #[pyo3(get)]
    pub tags: Vec<String>,
}

impl From<SimilarityResult> for PySimilarityResult {
    fn from(result: SimilarityResult) -> Self {
        PySimilarityResult {
            content_id: result.content_id,
            similarity_score: result.similarity_score,
            text_content: result.text_content,
            tags: result.tags,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyClusteringResult {
    #[pyo3(get)]
    pub cluster_id: String,
    #[pyo3(get)]
    pub cluster_size: usize,
    #[pyo3(get)]
    pub average_similarity: f32,
}

impl From<ClusteringResult> for PyClusteringResult {
    fn from(result: ClusteringResult) -> Self {
        PyClusteringResult {
            cluster_id: result.cluster_id,
            cluster_size: result.cluster_size,
            average_similarity: result.average_similarity,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyReminder {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub remind_at: String,
    #[pyo3(get)]
    pub priority: String,
    #[pyo3(get)]
    pub status: String,
    #[pyo3(get)]
    pub tags: Vec<String>,
    #[pyo3(get)]
    pub created_at: String,
}

impl From<Reminder> for PyReminder {
    fn from(reminder: Reminder) -> Self {
        PyReminder {
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

#[pyclass]
#[derive(Clone)]
pub struct PyApiKey {
    #[pyo3(get)]
    pub user_id: String,
    #[pyo3(get)]
    pub public_key: String,
    #[pyo3(get)]
    pub active: bool,
}

impl From<ApiKey> for PyApiKey {
    fn from(key: ApiKey) -> Self {
        PyApiKey {
            user_id: key.user_id,
            public_key: key.public_key,
            active: key.active,
        }
    }
}

impl From<PyApiKey> for ApiKey {
    fn from(pykey: PyApiKey) -> Self {
        let now = chrono::Utc::now();
        ApiKey {
            user_id: pykey.user_id,
            public_key: pykey.public_key,
            private_key: "".to_string(), // Not exposed in PyApiKey
            active: pykey.active,
            created_at: now,
            updated_at: now,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyConfig {
    #[pyo3(get)]
    pub version: String,
    #[pyo3(get)]
    pub default_project: String,
    #[pyo3(get)]
    pub auto_backup: bool,
    #[pyo3(get)]
    pub backup_interval: String,
    #[pyo3(get)]
    pub ai_enabled: bool,
    #[pyo3(get)]
    pub date_format: String,
    #[pyo3(get)]
    pub timezone: String,
}

impl From<Config> for PyConfig {
    fn from(config: Config) -> Self {
        PyConfig {
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

#[pyclass]
#[derive(Clone)]
pub struct PyRegistrationInfo {
    #[pyo3(get)]
    pub api_key: String,
    #[pyo3(get)]
    pub user_id: Option<String>,
    #[pyo3(get)]
    pub fingerprint: Option<String>,
}

impl From<RegistrationInfo> for PyRegistrationInfo {
    fn from(info: RegistrationInfo) -> Self {
        PyRegistrationInfo {
            api_key: info.api_key,
            user_id: info.user_id,
            fingerprint: info.fingerprint,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyProject {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub description: Option<String>,
    #[pyo3(get)]
    pub status: String,
    #[pyo3(get)]
    pub created_at: String,
}

impl From<Project> for PyProject {
    fn from(project: Project) -> Self {
        PyProject {
            name: project.name,
            description: project.description,
            status: project.status.to_string(),
            created_at: project.created_at.to_rfc3339(),
        }
    }
}

impl From<PyProject> for Project {
    fn from(pyproject: PyProject) -> Self {
        use crate::models::ProjectStatus;
        let status = match pyproject.status.as_str() {
            "active" => ProjectStatus::Active,
            "archived" => ProjectStatus::Archived,
            "completed" => ProjectStatus::Completed,
            _ => ProjectStatus::Active,
        };
        Project {
            name: pyproject.name,
            description: pyproject.description,
            status,
            tasks: Vec::new(), // PyProject doesn't expose tasks
            created_at: chrono::Utc::now(), // We don't store the original created_at in PyProject
            updated_at: chrono::Utc::now(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyTag {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub description: Option<String>,
    #[pyo3(get)]
    pub color: Option<String>,
    #[pyo3(get)]
    pub category: Option<String>,
    #[pyo3(get)]
    pub usage_count: u32,
}

impl From<Tag> for PyTag {
    fn from(tag: Tag) -> Self {
        PyTag {
            id: tag.id,
            name: tag.name,
            description: tag.description,
            color: tag.color,
            category: tag.category,
            usage_count: tag.usage_count,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PySummary {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub content: String,
    #[pyo3(get)]
    pub priority: String,
    #[pyo3(get)]
    pub created_at: String,
}

impl From<Summary> for PySummary {
    fn from(summary: Summary) -> Self {
        PySummary {
            id: summary.id,
            content: summary.content,
            priority: summary.priority.to_string(),
            created_at: summary.created_at.to_rfc3339(),
        }
    }
}

#[pymodule]
fn todozi(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTodozi>()?;
    m.add_class::<PyTask>()?;
    m.add_class::<PyMemory>()?;
    m.add_class::<PyIdea>()?;
    m.add_class::<PyQueueItem>()?;
    m.add_class::<PySimilarityResult>()?;
    m.add_class::<PyClusteringResult>()?;
    m.add_class::<PyReminder>()?;
    m.add_class::<PyApiKey>()?;
    m.add_class::<PyConfig>()?;
    m.add_class::<PyRegistrationInfo>()?;
    m.add_class::<PyProject>()?;
    m.add_class::<PyTag>()?;
    m.add_class::<PySummary>()?;
    Ok(())
}
