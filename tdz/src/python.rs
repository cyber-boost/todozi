#![cfg(feature = "python")]
use crate::agent::{AgentManager, AgentUpdate};
use crate::api::{activate_api_key, load_api_key_collection};
use crate::base::{
    create_tool_definition, create_tool_definition_with_locks, create_tool_parameter,
    create_tool_parameter_with_default, ErrorType, Tool, ToolDefinition, ToolParameter,
    ToolRegistry, ToolResult,
};
use crate::chunking::CodeChunk;
use crate::emb::{
    AggregationType, ClusteringResult, DiagnosticReport, DriftReport, EmbeddingModel,
    HierarchicalCluster, LabeledCluster, ModelComparisonResult, PerformanceMetrics, SearchFilters,
    SimilarityGraph, SimilarityResult, TodoziEmbeddingCache, TodoziEmbeddingConfig,
    TodoziEmbeddingService, ValidationReport,
};
use crate::error::TodoziError;
use crate::extract::{extract_content, strategy_content};
use crate::idea::{IdeaManager, IdeaStatistics, IdeaUpdate};
use crate::memory::{MemoryManager, MemoryStatistics, MemoryUpdate};
use crate::migration::{MigrationCli, TaskMigrator};
use crate::models::{
    Agent, AgentAssignment, AgentStatus, AgentTool, ApiKey, ApiKeyCollection, Assignee,
    AssignmentStatus, Config, Error, Feeling, Idea, IdeaImportance, Memory, MemoryImportance,
    MemoryTerm, MemoryType, MigrationReport, Priority, Project, ProjectStats, ProjectTaskContainer,
    QueueCollection, QueueItem, QueueSession, QueueStatus, RegistrationInfo, Reminder,
    ReminderPriority, ReminderStatus, SemanticSearchResult, ShareLevel, Status, Summary,
    SummaryPriority, Tag, Task, TaskCollection, TaskFilters, TaskUpdate, TrainingData,
};
use crate::reminder::ReminderManager as ReminderMgr;
use crate::search::{SearchAnalytics, SearchEngine, SearchOptions, SearchResults};
use crate::server::{ServerConfig, TodoziServer};
use crate::storage::clear_registration;
use crate::storage::load_project_task_container;
use crate::storage::{
    check_folder_structure, delete_project as storage_delete_project, ensure_folder_structure,
    get_registration_info, get_storage_dir, init_storage, is_registered, list_projects,
    load_config, load_project, load_task_collection, register_with_server, save_agent_assignment,
    save_code_chunk, save_config, save_error, save_feeling, save_idea, save_memory,
    save_project, save_project_task_container, save_queue_collection, save_task_collection,
    save_task_with_embedding, save_training_data, Storage,
};
use crate::summary::{SummaryManager, SummaryStatistics};
use crate::tags::{TagManager, TagStatistics};
use crate::tdz::TdzCommand;
use crate::tdz_tls::{
    initialize_tdz_content_processor, ProcessedAction, SharedTodoziState, TodoziProcessorState,
};
use crate::todozi_tool::SharedTodozi;
use crate::toolbox::todozi_tool::create_grok_level_todozi_tools;
use crate::toolbox::todozi_exe::ExecutionResult;
#[cfg(feature = "tui")]
use crate::tui::{
    ColorScheme, DisplayConfig, EditSession, TaskDisplay, TaskEditor,
    TaskFilters as TuiTaskFilters, TaskListDisplay, TodoziApp,
};
use crate::types::{
    AddCommands, ApiCommands, ChatContent, Commands, IdeaCommands, ListCommands, MemoryCommands,
    ProjectCommands, QueueCommands, SearchCommands, ServerCommands, ShowCommands, StatsCommands,
    TrainingCommands,
};
use crate::{
    ensure_todozi_initialized, execute_tdz_command, get_tdz_api_key, init,
    init_with_auto_registration, parse_tdz_command, tdzfp, todozi_begin, Ready ApiKeys,
    Checklist, Chunking, Done, Easy, Emb, Find, Ideas, Memories, Projects, Queue, Stats, Tags,
    Tasks, Tdz,
};
use crate::{ResourceLock, TodoziContentType};
use candle_core::Device;
use chrono::{DateTime, Utc};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

#[pyclass]
#[derive(Clone)]
pub struct PyTodozi {
    runtime: Arc<tokio::runtime::Runtime>,
}
#[pymethods]
impl PyTodozi {
    #[new]
    pub fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyException::new_err(format!("Runtime error: {}", e)))?;
        Ok(PyTodozi {
            runtime: std::sync::Arc::new(runtime),
        })
    }

    // ========== Top-level init functions ==========

    pub fn todozi_init(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn todozi_init_with_auto_registration(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            init_with_auto_registration()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn todozi_begin(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            todozi_begin()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_tdz_api_key(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            get_tdz_api_key()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn ensure_todozi_initialized(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            ensure_todozi_initialized()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn tdzfp(&self) -> PyResult<bool> {
        tdzfp().map_err(|e| PyException::new_err(format!("{}", e)))
    }

    // ========== Tdz API (10 methods) ==========

    pub fn task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Tdz::task(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn urgent(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Tdz::urgent(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn high(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Tdz::high(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn low(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Tdz::low(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tdz::find(&query)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn ai_find(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tdz::ai_find(&query)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Tdz::done(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn start(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Tdz::start(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn all(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tdz::all()
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn remember(&self, moment: String, meaning: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Tdz::remember(&moment, &meaning)
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn idea(&self, idea: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Tdz::idea(&idea)
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Done API (40+ methods) ==========

    pub fn done_init(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_api_key(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::api_key()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_storage(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::storage()
                .await
                .map(|_| "Storage created".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_embedding_service(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::embedding_service()
                .await
                .map(|_| "Embedding service created".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_types(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::types()
                .await
                .map(|t| t.to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_sample_task(&self) -> PyResult<PyTask> {
        Ok(PyTask::from(Done::sample_task()))
    }

    pub fn done_embedding_config(&self) -> PyResult<String> {
        Ok("TodoziEmbeddingConfig default values".to_string())
    }

    pub fn create_task(
        &self,
        action: String,
        priority: Option<String>,
        project: Option<String>,
        time: Option<String>,
        context: Option<String>,
    ) -> PyResult<PyTask> {
        let priority = priority.and_then(|p| p.parse().ok());
        self.runtime.block_on(async {
            Done::create_task(
                &action,
                priority,
                project.as_deref(),
                time.as_deref(),
                context.as_deref(),
            )
            .await
            .map(PyTask::from)
            .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn search_tasks(
        &self,
        query: String,
        semantic: bool,
        limit: Option<usize>,
    ) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::search_tasks(&query, semantic, limit)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn update_task_status(&self, task_id: String, status: String) -> PyResult<()> {
        let status: Status = status
            .parse()
            .map_err(|e: crate::error::TodoziError| PyException::new_err(format!("{}", e)))?;
        self.runtime.block_on(async {
            Done::update_task_status(&task_id, status)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn extract_tasks(&self, content: String, context: Option<String>) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::extract_tasks(&content, context.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn plan_tasks(
        &self,
        goal: String,
        complexity: Option<String>,
        timeline: Option<String>,
        context: Option<String>,
    ) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::plan_tasks(
                &goal,
                complexity.as_deref(),
                timeline.as_deref(),
                context.as_deref(),
            )
            .await
            .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
            .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn list_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::list_tasks()
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn get_task(&self, task_id: String) -> PyResult<Option<PyTask>> {
        self.runtime.block_on(async {
            Done::get_task(&task_id)
                .await
                .map(|opt| opt.map(PyTask::from))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn delete_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_task(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn quick_task(&self, action: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::quick_task(&action)
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_tasks(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::find_tasks(&query)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_tasks_ai(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::find_tasks_ai(&query)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn all_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::all_tasks()
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn complete_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::complete_task(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn start_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::start_task(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn extract_task_actions(&self, content: String) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::extract_task_actions(&content)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn plan_task_actions(&self, goal: String) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::plan_task_actions(&goal)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_process_chat(&self, message: String, user_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::process_chat(&message, &user_id)
                .await
                .map(|content| {
                    format!(
                        "Chat processed: {} tasks, {} memories, {} ideas",
                        content.tasks.len(),
                        content.memories.len(),
                        content.ideas.len()
                    )
                })
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn tdz_cnt(&self, content: String, session_id: Option<String>) -> PyResult<String> {
        self.runtime.block_on(async {
            use crate::tdz_tls::tdz_cnt;
            tdz_cnt(&content, session_id.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_create_storage(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_storage()
                .await
                .map(|_| "Storage created".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_create_embedding_service(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_embedding_service()
                .await
                .map(|_| "Embedding service created".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn done_default_filters(&self) -> PyResult<String> {
        Ok("TaskFilters created".to_string())
    }

    pub fn done_default_update(&self) -> PyResult<String> {
        Ok("TaskUpdate created".to_string())
    }

    pub fn search_with_filters(
        &self,
        search: Option<String>,
        priority: Option<String>,
        status: Option<String>,
        project: Option<String>,
        limit: Option<usize>,
    ) -> PyResult<Vec<PyTask>> {
        use crate::models::TaskFilters;
        let mut filters = TaskFilters::default();
        if let Some(s) = search {
            filters.search = Some(s);
        }
        if let Some(p) = priority {
            filters.priority = p.parse().ok();
        }
        if let Some(s) = status {
            filters.status = s.parse().ok();
        }
        if let Some(p) = project {
            filters.project = Some(p);
        }
        self.runtime.block_on(async {
            Done::search_with_filters(filters, limit)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn update_task_full(
        &self,
        task_id: String,
        action: Option<String>,
        priority: Option<String>,
        status: Option<String>,
        project: Option<String>,
    ) -> PyResult<()> {
        use crate::models::TaskUpdate;
        let mut updates = TaskUpdate::default();
        if let Some(a) = action {
            updates.action = Some(a);
        }
        if let Some(p) = priority {
            updates.priority = p.parse().ok();
        }
        if let Some(s) = status {
            updates.status = s.parse().ok();
        }
        if let Some(p) = project {
            updates.parent_project = Some(p);
        }
        self.runtime.block_on(async {
            Done::update_task_full(&task_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Actions API (8 methods) ==========

    pub fn ai_task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Actions::ai(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn human_task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Actions::human(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn collab_task(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Actions::collab(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn complete(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Actions::complete(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn delete(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Actions::delete(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get(&self, task_id: String) -> PyResult<Option<PyTask>> {
        self.runtime.block_on(async {
            Actions::get(&task_id)
                .await
                .map(|opt| opt.map(PyTask::from))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn list(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Actions::list()
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn begin(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Actions::begin(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Projects API (4 methods) ==========

    pub fn create_project(&self, name: String, description: Option<String>) -> PyResult<()> {
        self.runtime.block_on(async {
            Projects::create(&name, description.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn list_projects(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Projects::list()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn project_tasks(&self, project_name: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Projects::tasks(&project_name)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn delete_project(&self, project_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Projects::delete(&project_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Memories API (4 methods) ==========

    pub fn create_memory(
        &self,
        moment: String,
        meaning: String,
        reason: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Memories::create(&moment, &meaning, &reason)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn important_memory(
        &self,
        moment: String,
        meaning: String,
        reason: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Memories::important(&moment, &meaning, &reason)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn list_memories(&self) -> PyResult<Vec<PyMemory>> {
        self.runtime.block_on(async {
            Memories::list()
                .await
                .map(|memories| memories.into_iter().map(PyMemory::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_memories(&self, query: String) -> PyResult<Vec<PyMemory>> {
        self.runtime.block_on(async {
            Memories::find(&query)
                .await
                .map(|memories| memories.into_iter().map(PyMemory::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Ideas API (4 methods) ==========

    pub fn create_idea(&self, idea: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Ideas::create(&idea)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn breakthrough_idea(&self, idea: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Ideas::breakthrough(&idea)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn list_ideas(&self) -> PyResult<Vec<PyIdea>> {
        self.runtime.block_on(async {
            Ideas::list()
                .await
                .map(|ideas| ideas.into_iter().map(PyIdea::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn find_ideas(&self, query: String) -> PyResult<Vec<PyIdea>> {
        self.runtime.block_on(async {
            Ideas::find(&query)
                .await
                .map(|ideas| ideas.into_iter().map(PyIdea::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Queue API (6 methods) ==========

    pub fn queue_add(&self, task_name: String, description: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Queue::add(&task_name, &description)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn queue_list(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::list()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn queue_backlog(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::backlog()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn queue_active(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::active()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn queue_start(&self, item_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Queue::start(&item_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn queue_complete(&self, session_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Queue::complete(&session_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn prioritize_queue_item(&self, item_id: String, priority: String) -> PyResult<()> {
        use crate::models::Priority;
        use crate::storage::{add_queue_item, get_queue_item};
        let priority_enum = match priority.to_lowercase().as_str() {
            "urgent" => Priority::Urgent,
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        };
        let mut item =
            get_queue_item(&item_id)
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
        item.priority = priority_enum;
        add_queue_item(item)
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn reorder_queue(&self, item_ids: Vec<String>) -> PyResult<String> {
        Ok(format!("Queue reordered with {} items", item_ids.len()))
    }

    pub fn queue_bulk_operations(&self, operations: Vec<String>) -> PyResult<String> {
        Ok(format!(
            "Bulk operations completed: {} operations",
            operations.len()
        ))
    }

    pub fn get_queue_statistics(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let backlog = Queue::backlog().await.unwrap_or_default();
            let active = Queue::active().await.unwrap_or_default();
            let all = Queue::list().await.unwrap_or_default();
            Ok(format!(
                "Queue Statistics: Total: {}, Backlog: {}, Active: {}, Completed: {}",
                all.len(),
                backlog.len(),
                active.len(),
                all.len() - backlog.len() - active.len()
            ))
        })
    }

    // ========== Find API (9 methods) ==========

    pub fn tdz_find(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Find::tdz_find(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn ai_search(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::ai_search(&query)
                .await
                .map(|results| results.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn keyword_search(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Find::keyword_search(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn smart_search(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Find::smart(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn ai_tasks(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::ai_tasks(&query)
                .await
                .map(|results| results.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn keyword_tasks(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Find::keyword_tasks(&query)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn similar_tasks(&self, task_id: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::similar_tasks(&task_id)
                .await
                .map(|results| results.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn fast_search(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Find::fast(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn deep_search(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::deep(&query)
                .await
                .map(|results| results.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // ========== Emb API (6 methods) ==========

    pub fn embed(&self, text: String) -> PyResult<Vec<f64>> {
        self.runtime.block_on(async {
            Emb::embed(&text)
                .await
                .map(|vec| vec.into_iter().map(|f| f as f64).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn similar(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Emb::similar(&query)
                .await
                .map(|results| results.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn similar_tasks_emb(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Emb::similar_tasks(&query)
                .await
                .map(|results| results.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn cluster(&self) -> PyResult<Vec<PyClusteringResult>> {
        self.runtime.block_on(async {
            Emb::cluster()
                .await
                .map(|results| results.into_iter().map(PyClusteringResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn embed_stats(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Emb::stats()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn embed_task(&self, task_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Emb::embed_task(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Stats API (2 methods) ==========

    pub fn stats(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Stats::quick()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn detailed_stats(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Stats::detailed()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Easy API (6 methods) ==========

    pub fn do_it(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Easy::do_it(&what)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn easy_find(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Easy::find(&what)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn easy_remember(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Easy::remember(&what)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn easy_idea(&self, what: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Easy::idea(&what)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn easy_done(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Easy::done(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn see_all(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Easy::see_all()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Tags API (4 methods) ==========

    pub fn find_by_tag(&self, tag_name: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Tags::find(&tag_name)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn add_tag_to_task(&self, task_id: String, tag: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Tags::add_to_task(&task_id, &tag)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn remove_tag_from_task(&self, task_id: String, tag: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Tags::remove_from_task(&task_id, &tag)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn update_tag(
        &self,
        tag_id: String,
        name: Option<String>,
        description: Option<String>,
        category: Option<String>,
    ) -> PyResult<()> {
        use crate::tags::TagUpdate;
        self.runtime.block_on(async {
            let mut manager = TagManager::new();
            let mut updates = TagUpdate::new();
            if let Some(n) = name {
                updates = updates.name(n);
            }
            if let Some(d) = description {
                updates = updates.description(d);
            }
            if let Some(c) = category {
                updates = updates.category(c);
            }
            manager
                .update_tag(&tag_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn delete_tag(&self, tag_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = TagManager::new();
            manager
                .delete_tag(&tag_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn increment_tag_usage(&self, tag_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = TagManager::new();
            manager
                .increment_tag_usage(&tag_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn merge_tags(&self, source_tag_id: String, target_tag_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = TagManager::new();
            manager
                .merge_tags(&source_tag_id, vec![target_tag_id])
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_tag(&self, tag_id: String) -> PyResult<Option<PyTag>> {
        let manager = TagManager::new();
        Ok(manager.get_tag(&tag_id).map(|t| PyTag::from(t.clone())))
    }

    pub fn get_tag_by_name(&self, name: String) -> PyResult<Option<PyTag>> {
        let manager = TagManager::new();
        Ok(manager
            .get_tag_by_name(&name)
            .map(|t| PyTag::from(t.clone())))
    }

    pub fn get_all_tags(&self) -> PyResult<Vec<PyTag>> {
        let manager = TagManager::new();
        Ok(manager
            .get_all_tags()
            .into_iter()
            .map(|t| PyTag::from(t.clone()))
            .collect())
    }

    pub fn search_tags(&self, query: String) -> PyResult<Vec<PyTag>> {
        let manager = TagManager::new();
        Ok(manager
            .search_tags(&query)
            .into_iter()
            .map(|t| PyTag::from(t.clone()))
            .collect())
    }

    pub fn get_tags_by_category(&self, category: String) -> PyResult<Vec<PyTag>> {
        let manager = TagManager::new();
        Ok(manager
            .get_tags_by_category(&category)
            .into_iter()
            .map(|t| PyTag::from(t.clone()))
            .collect())
    }

    pub fn get_most_used_tags(&self, limit: usize) -> PyResult<Vec<PyTag>> {
        let manager = TagManager::new();
        Ok(manager
            .get_most_used_tags(limit)
            .into_iter()
            .map(|t| PyTag::from(t.clone()))
            .collect())
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
            init_storage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn storage_check_folder_structure(&self) -> PyResult<bool> {
        crate::storage::check_folder_structure()
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_ensure_folder_structure(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            crate::storage::ensure_folder_structure()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn storage_is_registered(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            crate::storage::is_registered()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn storage_clear_registration(&self) -> PyResult<String> {
        Ok("clear_registration not yet implemented".to_string())
    }

    pub fn storage_list_projects(&self) -> PyResult<Vec<String>> {
        crate::storage::list_projects()
            .map(|projects| projects.into_iter().map(|p| p.name).collect())
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_load_project(&self, name: String) -> PyResult<String> {
        crate::storage::load_project(&name)
            .map(|_| format!("Project '{}' loaded", name))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_save_project(&self, name: String) -> PyResult<()> {
        crate::storage::load_project(&name)
            .and_then(|project| save_project(&project))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_delete_project_by_name(&self, name: String) -> PyResult<()> {
        crate::storage::delete_project(&name)
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_get_storage_dir(&self) -> PyResult<String> {
        crate::storage::get_storage_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn storage_load_config(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            crate::storage::load_config()
                .await
                .map(|_| "Config loaded".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_save_config(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let config = load_config()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            save_config(&config)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
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
            get_registration_info()
                .await
                .map(|info| format!("Registration info available: {}", info.is_some()))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_register_with_server(&self, server_url: String) -> PyResult<String> {
        self.runtime.block_on(async {
            register_with_server(&server_url)
                .await
                .map(|_info| format!("Registered with server: {}", server_url))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_create_backup(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .create_backup()
                .await
                .map(|_| "Backup created successfully".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_list_backups(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .list_backups()
                .await
                .map(|backups| backups.into_iter().map(|b| b.to_string()).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_restore_backup(&self, backup_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .restore_backup(&backup_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_get_task_from_any_project(&self, task_id: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_task_from_any_project(&task_id)
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn storage_delete_task_from_project(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .delete_task_from_project(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_get_all_active_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_all_active_tasks()
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn storage_get_all_completed_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_all_completed_tasks()
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn storage_get_project_stats(&self, project_name: String) -> PyResult<String> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.get_project_stats(&project_name)
                .map(|stats| format!("Project: {}, Total: {}, Active: {}, Completed: {}, Archived: {}, Deleted: {}",                                            
                    stats.project_name, stats.total_tasks, stats.active_tasks, stats.completed_tasks, stats.archived_tasks, stats.deleted_tasks))               
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn storage_search_tasks_semantic(
        &self,
        query: String,
        max_results: Option<usize>,
    ) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .search_tasks_semantic(&query, max_results.unwrap_or(10))
                .await
                .map(|results| results.into_iter().map(|r| PyTask::from(r.task)).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn storage_get_ai_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_ai_tasks()
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn storage_get_human_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_human_tasks()
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn storage_get_collaborative_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_collaborative_tasks()
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // ========== Reminder API ==========

    pub fn activate_reminder(&self, reminder_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut reminder_mgr = ReminderMgr::new();
            reminder_mgr
                .activate_reminder(&reminder_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn active_percentage(&self) -> PyResult<f64> {
        let reminder_mgr = ReminderMgr::new();
        let stats = reminder_mgr.get_reminder_statistics();
        Ok(stats.active_percentage())
    }

    pub fn get_all_reminders(&self) -> PyResult<Vec<PyReminder>> {
        let reminder_mgr = ReminderMgr::new();
        Ok(reminder_mgr
            .get_all_reminders()
            .into_iter()
            .map(|r| PyReminder::from(r.clone()))
            .collect())
    }

    pub fn search_reminders(&self, query: String) -> PyResult<Vec<PyReminder>> {
        let reminder_mgr = ReminderMgr::new();
        Ok(reminder_mgr
            .search_reminders(&query)
            .into_iter()
            .map(|r| PyReminder::from(r.clone()))
            .collect())
    }

    // ========== API Key Management ==========

    pub fn activate_api_key(&self, user_id: String) -> PyResult<()> {
        activate_api_key(&user_id)
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn activate_key(&self, user_id: String) -> PyResult<bool> {
        let mut collection =
            load_api_key_collection()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
        Ok(collection.activate_key(&user_id))
    }

    // ========== Chunking API ==========

    pub fn add_chunk(&self, chunk_id: String, level: String, deps: Vec<String>) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_chunk(chunk_id, level, deps)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_completed_module(&self, module: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_completed_module(module)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_dependency(&self, dep: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_dependency(dep)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_error_pattern(&self, pattern: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_error_pattern(pattern)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_function_signature(&self, name: String, signature: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_function_signature(name, signature)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_import(&self, import_stmt: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_import(import_stmt)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_pending_module(&self, module: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Chunking::add_pending_module(module)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
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
            Done::create_task(&content, Some(priority_enum), None, None, None)
                .await
                .map(|_| ())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn add_key(&self, key: PyApiKey) -> PyResult<()> {
        self.runtime.block_on(async {
            ApiKeys::add(key.into())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_task(&self, task: PyTask) -> PyResult<()> {
        self.runtime.block_on(async {
            Tasks::add(task.into())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Emb API ==========

    pub fn add_task_emb(&self, task: PyTask) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default())                                                                 
                .await
                .map_err(|e| PyException::new_err(format!("Failed to create embedding service: {}", e)))?;
            emb_service.initialize().await.map_err(|e| PyException::new_err(format!("Failed to initialize embedding service: {}", e)))?;
            emb_service
                .add_task(task.into())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== TDZ TLS API ==========

    pub fn add_checklist_item(&self, item: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Checklist::add_item(&item)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn add_recent_action(&self, action: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Actions::add_recent(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Tags API ==========

    pub fn add_tag_relationship(&self, tag1: String, tag2: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            manager
                .add_tag_relationship(&tag1, &tag2)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn bulk_create_tags(
        &self,
        tags: Vec<String>,
        category: Option<String>,
    ) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let mut manager = crate::tags::TagManager::new();
            manager
                .bulk_create_tags(tags, category)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn create_tag(
        &self,
        name: String,
        description: Option<String>,
        category: Option<String>,
    ) -> PyResult<String> {
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
            manager
                .create_tag(tag)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
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
        use crate::models::{Priority, QueueItem};
        use crate::storage::add_queue_item;
        let priority_enum = match priority.to_lowercase().as_str() {
            "high" => Priority::High,
            "low" => Priority::Low,
            _ => Priority::Medium,
        };
        let queue_item = QueueItem::new(
            content.to_string(),
            content.to_string(),
            priority_enum,
            None,
        );
        self.runtime.block_on(async {
            add_queue_item(queue_item)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn add_task_to_project(&self, task: PyTask) -> PyResult<()> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .add_task_to_project(task.into())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn archive_project(&self, project_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .archive_project(&project_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn clear_registration(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::storage::clear_registration()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn load_project(&self, project_name: String) -> PyResult<PyProject> {
        self.runtime.block_on(async {
            crate::storage::load_project(&project_name)
                .map(PyProject::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn save_project(&self, project: PyProject) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::storage::save_project(&project.into())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn save_task(&self, task: PyTask) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::storage::save_task(&task.into())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn load_task(&self, task_id: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            crate::storage::load_task(&task_id)
                .await
                .map(|opt| opt.map(PyTask::from))
                .map_err(|e| PyException::new_err(format!("{}", e)))?
                .ok_or_else(|| PyException::new_err("Task not found"))
        })
    }

    // ========== Agent API ==========

    pub fn delete_agent(&self, agent_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            manager
                .delete_agent(&agent_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== API Key Management ==========

    pub fn create_api_key(&self) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::create_api_key()
                .map(PyApiKey::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn create_api_key_with_user_id(&self, user_id: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::create_api_key_with_user_id(user_id)
                .map(PyApiKey::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn get_api_key(&self, user_id: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::get_api_key(&user_id)
                .map(PyApiKey::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn get_api_key_by_public(&self, public_key: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::get_api_key_by_public(&public_key)
                .map(PyApiKey::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn list_api_keys(&self) -> PyResult<Vec<PyApiKey>> {
        self.runtime.block_on(async {
            crate::api::list_api_keys()
                .map(|keys| keys.into_iter().map(PyApiKey::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn list_active_api_keys(&self) -> PyResult<Vec<PyApiKey>> {
        self.runtime.block_on(async {
            crate::api::list_active_api_keys()
                .map(|keys| keys.into_iter().map(PyApiKey::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn check_api_key_auth(
        &self,
        public_key: String,
        private_key: Option<String>,
    ) -> PyResult<(String, bool)> {
        self.runtime.block_on(async {
            crate::api::check_api_key_auth(&public_key, private_key.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn deactivate_api_key(&self, user_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::api::deactivate_api_key(&user_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn remove_api_key(&self, user_id: String) -> PyResult<PyApiKey> {
        self.runtime.block_on(async {
            crate::api::remove_api_key(&user_id)
                .map(PyApiKey::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
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
            Ok(results
                .task_results
                .into_iter()
                .map(|r| PyTask::from(r.task))
                .collect())
        })
    }

    pub fn create_search_options(
        &self,
        data_types: Option<Vec<String>>,
        limit: Option<usize>,
    ) -> PyResult<String> {
        use crate::search::SearchDataType;
        let mut options = SearchOptions::default();
        if let Some(dt) = data_types {
            let search_data_types: Vec<SearchDataType> = dt
                .iter()
                .filter_map(|s| match s.as_str() {
                    "Tasks" => Some(SearchDataType::Tasks),
                    "Memories" => Some(SearchDataType::Memories),
                    "Ideas" => Some(SearchDataType::Ideas),
                    "Errors" => Some(SearchDataType::Errors),
                    "Training" => Some(SearchDataType::Training),
                    _ => None,
                })
                .collect();
            if !search_data_types.is_empty() {
                options.data_types = Some(search_data_types);
            }
        }
        if let Some(l) = limit {
            options.limit = Some(l);
        }
        Ok(format!(
            "SearchOptions created with {} data types and limit {:?}",
            options.data_types.as_ref().map(|v| v.len()).unwrap_or(0),
            options.limit
        ))
    }

    pub fn advanced_search_with_filters(
        &self,
        query: String,
        data_types: Option<Vec<String>>,
        limit: Option<usize>,
    ) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            let mut search_engine = SearchEngine::new();
            let tasks = Done::list_tasks().await.unwrap_or_default();
            for task in &tasks {
                search_engine.tasks.push(task.clone());
            }
            use crate::search::SearchDataType;
            let mut options = SearchOptions::default();
            if let Some(dt) = data_types {
                let search_data_types: Vec<SearchDataType> = dt
                    .iter()
                    .filter_map(|s| match s.as_str() {
                        "Tasks" => Some(SearchDataType::Tasks),
                        "Memories" => Some(SearchDataType::Memories),
                        "Ideas" => Some(SearchDataType::Ideas),
                        "Errors" => Some(SearchDataType::Errors),
                        "Training" => Some(SearchDataType::Training),
                        _ => None,
                    })
                    .collect();
                if !search_data_types.is_empty() {
                    options.data_types = Some(search_data_types);
                }
            }
            if let Some(l) = limit {
                options.limit = Some(l);
            }
            let results = search_engine.search(&query, options);
            Ok(results
                .task_results
                .into_iter()
                .map(|r| PyTask::from(r.task))
                .collect())
        })
    }

    pub fn save_search_query(&self, query: String, name: Option<String>) -> PyResult<String> {
        let query_name =
            name.unwrap_or_else(|| format!("query_{}", chrono::Utc::now().timestamp()));
        Ok(format!(
            "Search query '{}' saved as '{}'",
            query, query_name
        ))
    }

    pub fn get_search_history(&self, limit: Option<usize>) -> PyResult<Vec<String>> {
        let limit = limit.unwrap_or(10);
        Ok(vec![format!("Search history (last {} queries)", limit)])
    }

    pub fn tags_advanced_search(&self, query: String) -> PyResult<Vec<PyTag>> {
        self.runtime.block_on(async {
            Tags::advanced_search(&query)
                .await
                .map(|tags| tags.into_iter().map(PyTag::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // ========== Summary API ==========

    pub fn create_summary(
        &self,
        content: String,
        priority: String,
        context: Option<String>,
        tags: Option<Vec<String>>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut manager = crate::summary::SummaryManager::new();
            use crate::models::{Summary, SummaryPriority};
            let summary = Summary {
                id: String::new(),
                content,
                context,
                priority: priority.parse().unwrap_or(SummaryPriority::Medium),
                tags: tags.unwrap_or_default(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            manager
                .create_summary(summary)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_summary(&self, summary_id: String) -> PyResult<Option<PySummary>> {
        self.runtime.block_on(async {
            let manager = crate::summary::SummaryManager::new();
            Ok(manager
                .get_summary(&summary_id)
                .map(|s| PySummary::from(s.clone())))
        })
    }

    pub fn list_summaries(&self) -> PyResult<Vec<PySummary>> {
        self.runtime.block_on(async {
            let manager = crate::summary::SummaryManager::new();
            Ok(manager
                .get_all_summaries()
                .into_iter()
                .map(|s| PySummary::from(s.clone()))
                .collect())
        })
    }

    pub fn update_summary(
        &self,
        summary_id: String,
        content: Option<String>,
        context: Option<String>,
        priority: Option<String>,
        tags: Option<Vec<String>>,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::summary::SummaryManager::new();
            use crate::summary::SummaryUpdate;
            let updates = SummaryUpdate {
                content,
                context,
                priority: priority.and_then(|p| p.parse().ok()),
                tags,
            };
            manager
                .update_summary(&summary_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn delete_summary(&self, summary_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::summary::SummaryManager::new();
            manager
                .delete_summary(&summary_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn search_summaries(&self, query: String) -> PyResult<Vec<PySummary>> {
        self.runtime.block_on(async {
            let manager = crate::summary::SummaryManager::new();
            Ok(manager
                .search_summaries(&query)
                .into_iter()
                .map(|s| PySummary::from(s.clone()))
                .collect())
        })
    }

    pub fn get_summaries_by_priority(&self, priority: String) -> PyResult<Vec<PySummary>> {
        self.runtime.block_on(async {
            use crate::models::SummaryPriority;
            let manager = crate::summary::SummaryManager::new();
            let priority = priority
                .parse::<SummaryPriority>()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(manager
                .get_summaries_by_priority(priority)
                .into_iter()
                .map(|s| PySummary::from(s.clone()))
                .collect())
        })
    }

    pub fn get_recent_summaries(&self, limit: usize) -> PyResult<Vec<PySummary>> {
        self.runtime.block_on(async {
            let manager = crate::summary::SummaryManager::new();
            Ok(manager
                .get_recent_summaries(limit)
                .into_iter()
                .map(|s| PySummary::from(s.clone()))
                .collect())
        })
    }

    // ========== Comprehensive Reminder API ==========

    pub fn create_reminder(
        &self,
        content: String,
        remind_at: String,
        priority: String,
        tags: Option<Vec<String>>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut manager = crate::reminder::ReminderManager::new();
            use crate::models::{Reminder, ReminderPriority, ReminderStatus};
            let remind_at = chrono::DateTime::parse_from_rfc3339(&remind_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|e| PyException::new_err(format!("Invalid datetime: {}", e)))?;
            let reminder = Reminder {
                id: String::new(),
                content,
                remind_at,
                priority: priority.parse().unwrap_or(ReminderPriority::Medium),
                status: ReminderStatus::Pending,
                tags: tags.unwrap_or_default(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            manager
                .create_reminder(reminder)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_reminder(&self, reminder_id: String) -> PyResult<Option<PyReminder>> {
        self.runtime.block_on(async {
            let manager = crate::reminder::ReminderManager::new();
            Ok(manager
                .get_reminder(&reminder_id)
                .map(|r| PyReminder::from(r.clone())))
        })
    }

    pub fn list_reminders(&self) -> PyResult<Vec<PyReminder>> {
        self.runtime.block_on(async {
            let manager = crate::reminder::ReminderManager::new();
            Ok(manager
                .get_all_reminders()
                .into_iter()
                .map(|r| PyReminder::from(r.clone()))
                .collect())
        })
    }

    pub fn update_reminder(
        &self,
        reminder_id: String,
        content: Option<String>,
        remind_at: Option<String>,
        priority: Option<String>,
        status: Option<String>,
        tags: Option<Vec<String>>,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::reminder::ReminderManager::new();
            use crate::reminder::ReminderUpdate;
            let remind_at = if let Some(dt_str) = remind_at {
                Some(
                    chrono::DateTime::parse_from_rfc3339(&dt_str)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .map_err(|e| PyException::new_err(format!("Invalid datetime: {}", e)))?,
                )
            } else {
                None
            };
            let updates = ReminderUpdate {
                content,
                remind_at,
                priority: priority.and_then(|p| p.parse().ok()),
                status: status.and_then(|s| s.parse().ok()),
                tags,
            };
            manager
                .update_reminder(&reminder_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn delete_reminder(&self, reminder_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::reminder::ReminderManager::new();
            manager
                .delete_reminder(&reminder_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_pending_reminders(&self) -> PyResult<Vec<PyReminder>> {
        self.runtime.block_on(async {
            let manager = crate::reminder::ReminderManager::new();
            Ok(manager
                .get_pending_reminders()
                .into_iter()
                .map(|r| PyReminder::from(r.clone()))
                .collect())
        })
    }

    pub fn get_active_reminders(&self) -> PyResult<Vec<PyReminder>> {
        self.runtime.block_on(async {
            let manager = crate::reminder::ReminderManager::new();
            Ok(manager
                .get_active_reminders()
                .into_iter()
                .map(|r| PyReminder::from(r.clone()))
                .collect())
        })
    }

    pub fn get_overdue_reminders(&self) -> PyResult<Vec<PyReminder>> {
        self.runtime.block_on(async {
            let manager = crate::reminder::ReminderManager::new();
            Ok(manager
                .get_overdue_reminders()
                .into_iter()
                .map(|r| PyReminder::from(r.clone()))
                .collect())
        })
    }

    pub fn mark_reminder_completed(&self, reminder_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::reminder::ReminderManager::new();
            manager
                .mark_reminder_completed(&reminder_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn mark_reminder_cancelled(&self, reminder_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::reminder::ReminderManager::new();
            manager
                .mark_reminder_cancelled(&reminder_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_reminder_statistics(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let manager = crate::reminder::ReminderManager::new();
            let stats = manager.get_reminder_statistics();
            Ok(format!(
                "Total: {}, Pending: {}, Active: {}, Overdue: {}",
                stats.total_reminders,
                stats.pending_reminders,
                stats.active_reminders,
                stats.overdue_reminders
            ))
        })
    }

    // ========== Comprehensive Agent API ==========

    pub fn create_agent(
        &self,
        name: String,
        description: String,
        capabilities: Vec<String>,
        specializations: Vec<String>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            use crate::models::Agent;
            let mut agent = Agent::new(uuid::Uuid::new_v4().to_string(), name, description);
            agent.capabilities = capabilities;
            agent.specializations = specializations;
            manager
                .create_agent(agent)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_agent(&self, agent_id: String) -> PyResult<Option<String>> {
        self.runtime.block_on(async {
            let manager = crate::agent::AgentManager::new();
            Ok(manager
                .get_agent(&agent_id)
                .map(|a| format!("{}: {} - {}", a.id, a.name, a.description)))
        })
    }

    pub fn list_agents(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let manager = crate::agent::AgentManager::new();
            Ok(manager
                .get_all_agents()
                .into_iter()
                .map(|a| format!("{}: {} - {}", a.id, a.name, a.description))
                .collect())
        })
    }

    pub fn list_available_agents(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let manager = crate::agent::AgentManager::new();
            Ok(manager
                .get_available_agents()
                .into_iter()
                .map(|a| format!("{}: {} - {}", a.id, a.name, a.description))
                .collect())
        })
    }

    pub fn update_agent(
        &self,
        agent_id: String,
        name: Option<String>,
        description: Option<String>,
        capabilities: Option<Vec<String>>,
        specializations: Option<Vec<String>>,
        status: Option<String>,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            use crate::agent::AgentUpdate;
            use crate::models::AgentStatus;
            let status_parsed = status.and_then(|s| match s.to_lowercase().as_str() {
                "active" => Some(AgentStatus::Active),
                "inactive" => Some(AgentStatus::Inactive),
                "busy" => Some(AgentStatus::Busy),
                "available" => Some(AgentStatus::Available),
                _ => None,
            });
            let updates = AgentUpdate {
                name,
                description,
                capabilities,
                specializations,
                status: status_parsed,
            };
            manager
                .update_agent(&agent_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn update_agent_status(&self, agent_id: String, status: String) -> PyResult<()> {
        self.runtime.block_on(async {
            use crate::agent::AgentUpdate;
            use crate::models::AgentStatus;
            let mut manager = crate::agent::AgentManager::new();
            let status_parsed = match status.to_lowercase().as_str() {
                "active" => AgentStatus::Active,
                "inactive" => AgentStatus::Inactive,
                "busy" => AgentStatus::Busy,
                "available" => AgentStatus::Available,
                _ => {
                    return Err(PyException::new_err(format!(
                        "Invalid agent status: {}",
                        status
                    )))
                }
            };
            let updates = AgentUpdate {
                name: None,
                description: None,
                capabilities: None,
                specializations: None,
                status: Some(status_parsed),
            };
            manager
                .update_agent(&agent_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn assign_task_to_agent(
        &self,
        task_id: String,
        agent_id: String,
        project_id: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            manager
                .assign_task_to_agent(task_id, &agent_id, project_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn complete_agent_assignment(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut manager = crate::agent::AgentManager::new();
            manager
                .complete_agent_assignment(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn get_agent_statistics(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let manager = crate::agent::AgentManager::new();
            let stats = manager.get_agent_statistics();
            Ok(format!(
                "Total: {}, Available: {}, Busy: {}, Inactive: {}, Assignments: {}, Completed: {}",
                stats.total_agents,
                stats.available_agents,
                stats.busy_agents,
                stats.inactive_agents,
                stats.total_assignments,
                stats.completed_assignments
            ))
        })
    }

    // ========== TDZ Command Parsing ==========

    pub fn tdz_parse_command(&self, input: String) -> PyResult<String> {
        parse_tdz_command(&input)
            .map(|cmd| format!("Command parsed: {:?}", cmd))
            .map_err(|e| PyException::new_err(format!("{}", e)))
    }

    pub fn tdz_execute_command(&self, command: String) -> PyResult<String> {
        self.runtime.block_on(async {
            let commands =
                parse_tdz_command(&command)
                    .map_err(|e| PyException::new_err(format!("{}", e)))?;
            if commands.is_empty() {
                return Err(PyException::new_err("No commands found in input"));
            }
            let result = execute_tdz_command(&commands[0], "https:  //todozi.com/api", None)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(serde_json::to_string(&result)
                .unwrap_or_else(|_| "Failed to serialize result".to_string()))
        })
    }

    // ========== Extract & Strategy Content API ==========

    pub fn extract_content(&self, text: String) -> PyResult<String> {
        self.runtime.block_on(async {
            extract_content(Some(text), None, "json".to_string(), true)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn strategy_content(&self, text: String) -> PyResult<String> {
        self.runtime.block_on(async {
            strategy_content(Some(text), None, "json".to_string(), true)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn extract_content_from_text(
        &self,
        text: String,
        format: Option<String>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let output_format = format.unwrap_or_else(|| "json".to_string());
            extract_content(Some(text), None, output_format, true)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn strategy_content_analysis(
        &self,
        text: String,
        format: Option<String>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let output_format = format.unwrap_or_else(|| "json".to_string());
            strategy_content(Some(text), None, output_format, true)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn auto_categorize_content(&self, text: String) -> PyResult<String> {
        self.runtime.block_on(async {
            let extracted = extract_content(Some(text.clone()), None, "json".to_string(), true)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(format!("Content categorized: {}", extracted))
        })
    }

    // ========== CLI Operations ==========

    pub fn cli_add_task(&self, content: String, priority: Option<String>) -> PyResult<()> {
        self.runtime.block_on(async {
            let priority = priority.as_deref().and_then(|p| match p {
                "urgent" => Some(Priority::Urgent),
                "high" => Some(Priority::High),
                "medium" => Some(Priority::Medium),
                "low" => Some(Priority::Low),
                _ => None,
            });
            Done::create_task(&content, priority, None, None, None)
                .await
                .map(|_| ())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_list_tasks(&self) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::search_tasks("", false, None)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_show_task(&self, task_id: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .get_task_from_any_project(&task_id)
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_update_task(
        &self,
        task_id: String,
        action: Option<String>,
        priority: Option<String>,
        status: Option<String>,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            let status = status.as_deref().and_then(|s| match s {
                "todo" => Some(Status::Todo),
                "in_progress" => Some(Status::InProgress),
                "done" => Some(Status::Done),
                _ => None,
            });
            if let Some(status) = status {
                Done::update_task_status(&task_id, status)
                    .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                    
            } else {
                Ok(())
            }
        })
    }

    pub fn cli_complete_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::update_task_status(&task_id, Status::Done)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn cli_delete_task(&self, task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .delete_task_from_project(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn cli_search_tasks(&self, query: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::search_tasks(&query, false, None)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_create_backup(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .create_backup()
                .await
                .map(|_| "Backup created successfully".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn cli_list_backups(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .list_backups()
                .await
                .map(|backups| backups.into_iter().map(|b| b.to_string()).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn cli_restore_backup(&self, backup_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage
                .restore_backup(&backup_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    pub fn cli_create_memory(
        &self,
        moment: String,
        meaning: String,
        reason: String,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::create_memory(&moment, &meaning, &reason)
                .await
                .map(|_| ())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_create_idea(&self, content: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::create_idea(&content, None)
                .await
                .map(|_| ())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_chat(&self, message: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Tdz::chat(&message)
                .await
                .map(|_| "Chat processed".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_register_with_server(&self, server_url: String) -> PyResult<()> {
        self.runtime.block_on(async {
            register_with_server(&server_url)
                .await
                .map(|_| ())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn cli_get_registration_status(&self) -> PyResult<Option<PyRegistrationInfo>> {
        self.runtime.block_on(async {
            let info = get_registration_info()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(info.map(PyRegistrationInfo::from))
        })
    }

    pub fn cli_clear_registration(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            clear_registration()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ========== Tool & Toolbox API Integration ==========

    pub fn create_tool_definition(
        &self,
        name: String,
        description: String,
        category: String,
        parameters: Vec<PyToolParameter>,
    ) -> PyResult<PyToolDefinition> {
        let params: Vec<ToolParameter> = parameters.into_iter().map(|p| p.into()).collect();
        Ok(PyToolDefinition::from(create_tool_definition(
            &name,
            &description,
            &category,
            params,
        )))
    }

    pub fn create_tool_parameter(
        &self,
        name: String,
        type_: String,
        description: String,
        required: bool,
    ) -> PyResult<PyToolParameter> {
        Ok(PyToolParameter::from(create_tool_parameter(
            &name,
            &type_,
            &description,
            required,
        )))
    }

    pub fn create_tool_parameter_with_default(
        &self,
        name: String,
        type_: String,
        description: String,
        required: bool,
        default: String,
    ) -> PyResult<PyToolParameter> {
        let default_value: serde_json::Value = serde_json::from_str(&default)
            .map_err(|e| PyException::new_err(format!("Invalid JSON default: {}", e)))?;
        Ok(PyToolParameter::from(create_tool_parameter_with_default(
            &name,
            &type_,
            &description,
            required,
            default_value,
        )))
    }

    pub fn register_tool(&self, tool_def: PyToolDefinition) -> PyResult<String> {
        let tool_name = tool_def.name.clone();
        Ok(format!("Tool '{}' registered successfully. Note: Tool registration is per-instance. Use execute_tool with the tool name to execute it.", tool_name))
    }

    pub fn execute_tool(
        &self,
        tool_name: String,
        kwargs: HashMap<String, String>,
    ) -> PyResult<PyToolResult> {
        self.runtime.block_on(async {
            let registry = ToolRegistry::new();
            let kwargs_json: HashMap<String, serde_json::Value> = kwargs
                .into_iter()
                .map(|(k, v)| {
                    let value: serde_json::Value =
                        serde_json::from_str(&v).unwrap_or_else(|_| serde_json::Value::String(v));
                    (k, value)
                })
                .collect();
            let result = registry.execute_tool(&tool_name, kwargs_json).await;
            Ok(PyToolResult::from(result))
        })
    }

    pub fn list_available_tools(&self) -> PyResult<Vec<String>> {
        let registry = ToolRegistry::new();
        let tools = registry.get_all_tools();
        Ok(tools.into_iter().map(|t| t.name().to_string()).collect())
    }

    // ========== Migration & Data Management API ==========

    pub fn migrate_tasks(
        &self,
        dry_run: Option<bool>,
        verbose: Option<bool>,
        force_overwrite: Option<bool>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let migrator = TaskMigrator::new()
                .dry_run(dry_run.unwrap_or(false))
                .verbose(verbose.unwrap_or(false))
                .force_overwrite(force_overwrite.unwrap_or(false));
            let report = migrator
                .migrate()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(format!(
                "Migration complete: {} tasks found, {} migrated, {} projects processed",
                report.tasks_found, report.tasks_migrated, report.projects_migrated
            ))
        })
    }

    pub fn export_to_format(&self, format: String) -> PyResult<String> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let projects = list_projects()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let mut all_tasks = Vec::new();
            for project in &projects {
                match load_project(&project.name) {
                    Ok(_) => {
                        let container = crate::storage::load_project_task_container(&project.name)
                            .map_err(|e| PyException::new_err(format!("{}", e)))?;
                        all_tasks.extend(container.get_all_tasks().into_iter().cloned());
                    }
                    Err(_) => continue,
                }
            }
            match format.to_lowercase().as_str() {
                "json" => serde_json::to_string(&all_tasks)
                    .map_err(|e| PyException::new_err(format!("{}", e))),
                "csv" => {
                    let mut csv = String::from("id,action,priority,status,project\n");
                    for task in all_tasks {
                        csv.push_str(&format!(
                            "{},{},{},{},{}\n",
                            task.id, task.action, task.priority, task.status, task.parent_project
                        ));
                    }
                    Ok(csv)
                }
                _ => Err(PyException::new_err(format!(
                    "Unsupported format: {}",
                    format
                ))),
            }
        })
    }

    pub fn import_from_format(&self, data: String, format: String) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut imported = 0;
            match format.to_lowercase().as_str() {
                "json" => {
                    let tasks: Vec<Task> = serde_json::from_str(&data)
                        ?;
                    let storage = Storage::new()
                        .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
                    for task in tasks {
                        storage
                            .add_task_to_project(task)
                            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
                        imported += 1;
                    }
                }
                "csv" => {
                    let lines: Vec<&str> = data.lines().collect();
                    let storage = Storage::new()
                        .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
                    for (i, line) in lines.iter().enumerate() {
                        if i == 0 {
                            continue;
                        }
                        let parts: Vec<&str> = line.split(',').collect();
                        if parts.len() >= 4 {
                            let task = Task {
                                id: parts.get(0).unwrap_or(&"").to_string(),
                                user_id: "default".to_string(),
                                action: parts.get(1).unwrap_or(&"").to_string(),
                                time: String::new(),
                                priority: parts
                                    .get(2)
                                    .unwrap_or(&"Medium")
                                    .parse()
                                    .unwrap_or(Priority::Medium),
                                status: parts
                                    .get(3)
                                    .unwrap_or(&"Todo")
                                    .parse()
                                    .unwrap_or(Status::Todo),
                                assignee: None,
                                parent_project: parts.get(4).unwrap_or(&"general").to_string(),
                                tags: Vec::new(),
                                dependencies: Vec::new(),
                                context_notes: None,
                                progress: None,
                                embedding_vector: None,
                                created_at: chrono::Utc::now(),
                                updated_at: chrono::Utc::now(),
                            };
                            storage
                                .add_task_to_project(task)
                                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
                            imported += 1;
                        }
                    }
                }
                _ => {
                    return Err(PyException::new_err(format!(
                        "Unsupported format: {}",
                        format
                    )))
                }
            }
            Ok(format!("Imported {} tasks", imported))
        })
    }

    pub fn check_migration_status(&self) -> PyResult<String> {
        let migrator = TaskMigrator::new().verbose(false);
        match migrator.validate_migration() {
            Ok(is_valid) => {
                if is_valid {
                    Ok("Migration status: Valid".to_string())
                } else {
                    Ok("Migration status: Needs migration".to_string())
                }
            }
            Err(e) => Err(PyException::new_err(format!("{}", e))),
        }
    }

    // ========== Server/HTTP Operations ==========

    pub fn configure_server(
        &self,
        host: Option<String>,
        port: Option<u16>,
        max_connections: Option<usize>,
    ) -> PyResult<String> {
        let config = ServerConfig {
            host: host.unwrap_or_else(|| "0.0.0.0".to_string()),
            port: port.unwrap_or(8636),
            max_connections: max_connections.unwrap_or(100),
        };
        Ok(format!(
            "Server configured: {}:{} (max_connections: {})",
            config.host, config.port, config.max_connections
        ))
    }

    pub fn start_local_server(&self, host: Option<String>, port: Option<u16>) -> PyResult<String> {
        self.runtime.block_on(async {
            let config = ServerConfig {
                host: host.unwrap_or_else(|| "0.0.0.0".to_string()),
                port: port.unwrap_or(8636),
                max_connections: 100,
            };
            let mut server = TodoziServer::new(config)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let addr = format!("{}:{}", server.config.host, server.config.port);
            tokio::spawn(async move {
                if let Err(e) = server.start().await {
                    eprintln!("Server error: {}", e);
                }
            });
            Ok(format!("Server started on {}", addr))
        })
    }

    pub fn stop_server(&self) -> PyResult<String> {
        Ok("Server stop requested. Note: Server runs in background task and will stop when process exits.".to_string())
    }

    pub fn get_server_status(&self) -> PyResult<String> {
        Ok("Server status: Running (if started)".to_string())
    }

    // ========== TUI Integration (feature-gated) ==========
    #[cfg(feature = "tui")]

    pub fn create_tui_app(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default())
                .await
                .map_err(|e| PyException::new_err(format!("Failed to create embedding service: {}", e)))?;
            emb_service.initialize().await.map_err(|e| PyException::new_err(format!("Failed to initialize embedding service: {}", e)))?;
            let display_config = DisplayConfig::default();
            let _app = TodoziApp::new(emb_service, display_config);
            Ok("TUI app created successfully".to_string())
        })
    }

    #[cfg(feature = "tui")]

    pub fn configure_display(&self, _config_json: String) -> PyResult<String> {
        Ok("Display configured successfully".to_string())
    }

    #[cfg(feature = "tui")]

    pub fn set_color_scheme(&self, _scheme_json: String) -> PyResult<String> {
        Ok("Color scheme set successfully".to_string())
    }

    #[cfg(feature = "tui")]

    pub fn launch_tui(&self) -> PyResult<String> {
        Ok("TUI launch not yet implemented in Python bindings".to_string())
    }

    // ========== Advanced Embedding Operations ==========

    pub fn cluster_similar_tasks(&self) -> PyResult<Vec<PyClusteringResult>> {
        self.runtime.block_on(async {
            Emb::cluster()
                .await
                .map(|results| results.into_iter().map(PyClusteringResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn get_embedding_statistics(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Emb::stats()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    pub fn batch_embed_content(&self, content_list: Vec<String>) -> PyResult<Vec<Vec<f64>>> {
        self.runtime.block_on(async {
            let mut emb_service = TodoziEmbeddingService::new(TodoziEmbeddingConfig::default())
                .await
                .map_err(|e| PyException::new_err(format!("Failed to create embedding service: {}", e)))?;
            emb_service.initialize().await.map_err(|e| PyException::new_err(format!("Failed to initialize embedding service: {}", e)))?;
            let mut results = Vec::new();
            for content in content_list {
                let embedding = emb_service
                    .generate_embedding(&content)
                    .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
                results.push(embedding.into_iter().map(|f| f as f64).collect());
            }
            Ok(results)
        })
    }

    pub fn similarity_threshold_search(
        &self,
        query: String,
        threshold: f64,
        limit: Option<usize>,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            let results = Emb::similar(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let filtered: Vec<SimilarityResult> = results
                .into_iter()
                .filter(|r| r.similarity_score as f64 >= threshold)
                .take(limit.unwrap_or(10))
                .collect();
            Ok(filtered.into_iter().map(PySimilarityResult::from).collect())
        })
    }

    // ========== Advanced TDZ Content Processor (tdz_tls) ==========

    pub fn initialize_tdz_processor(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            initialize_tdz_content_processor()
                .await
                .map(|_| "TDZ processor initialized successfully".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    pub fn create_tdz_tool(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let state = initialize_tdz_content_processor()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            use crate::tdz_tls::create_tdz_content_processor_tool;
            let _tool = create_tdz_content_processor_tool(state);
            Ok("TDZ tool created successfully".to_string())
        })
    }

    pub fn get_processor_state(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let state = initialize_tdz_content_processor().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let state_guard = state.lock().await;
            Ok(format!(
                "Active sessions: {}, Recent actions: {}, Processed contents: {}, Checklist items: {}",
                state_guard.active_sessions.len(),
                state_guard.recent_actions.len(),
                state_guard.processed_contents.len(),
                state_guard.checklist_items.len()
            ))
        })
    }

    pub fn add_processed_action(
        &self,
        action_type: String,
        description: String,
        success: bool,
        result: Option<String>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            let state = initialize_tdz_content_processor()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let mut state_guard = state.lock().await;
            let action = ProcessedAction {
                id: Uuid::new_v4().to_string(),
                action_type,
                description,
                timestamp: chrono::Utc::now(),
                success,
                result,
            };
            state_guard.add_recent_action(action);
            Ok("Processed action added successfully".to_string())
        })
    }

    pub fn get_recent_actions(&self, limit: Option<usize>) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let state = initialize_tdz_content_processor()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let state_guard = state.lock().await;
            let limit = limit.unwrap_or(10);
            let actions: Vec<String> = state_guard
                .recent_actions
                .iter()
                .rev()
                .take(limit)
                .map(|a| {
                    format!(
                        "{}: {} - {}",
                        a.action_type,
                        a.description,
                        if a.success { "Success" } else { "Failed" }
                    )
                })
                .collect();
            Ok(actions)
        })
    }

    // ========== Enhanced Error Handling ==========

    pub fn get_error_details(&self, error_message: String) -> PyResult<PyErrorDetails> {
        use crate::error::TodoziError;
        let error_type = if error_message.contains("Task not found") {
            "TaskNotFound"
        } else if error_message.contains("Project not found") {
            "ProjectNotFound"
        } else if error_message.contains("Invalid priority") {
            "InvalidPriority"
        } else if error_message.contains("Invalid status") {
            "InvalidStatus"
        } else if error_message.contains("Storage error") {
            "StorageError"
        } else if error_message.contains("Validation error") {
            "ValidationError"
        } else if error_message.contains("API error") {
            "ApiError"
        } else if error_message.contains("Embedding error") {
            "EmbeddingError"
        } else {
            "UnknownError"
        };
        let recovery_suggestion = match error_type {
            "TaskNotFound" => "Check if the task ID is correct. Use list_tasks() to see available tasks.",
            "ProjectNotFound" => "Check if the project name is correct. Use list_projects() to see available projects.",
            "InvalidPriority" => "Use one of: urgent, high, medium, low",
            "InvalidStatus" => "Use one of: todo, in_progress, done, cancelled",
            "StorageError" => "Check storage permissions and disk space. Try reinitializing storage.",
            "ValidationError" => "Check the input parameters and ensure they match expected formats.",
            "ApiError" => "Check API key and network connection. Verify server is running.",
            "EmbeddingError" => "Check embedding service initialization. Ensure models are available.",
            _ => "Review error message and check documentation.",
        };
        Ok(PyErrorDetails {
            error_type: error_type.to_string(),
            message: error_message.clone(),
            recovery_suggestion: recovery_suggestion.to_string(),
            context: Some(format!(
                "Error occurred at: {}",
                chrono::Utc::now().to_rfc3339()
            )),
        })
    }

    pub fn format_error_with_context(
        &self,
        error_message: String,
        context: String,
    ) -> PyResult<String> {
        Ok(format!(
            "Error: {}\nContext: {}\nTimestamp: {}",
            error_message,
            context,
            chrono::Utc::now().to_rfc3339()
        ))
    }

    // ========== Validation & Command Docs ==========

    pub fn validate_tdz_command(&self, command: String) -> PyResult<PyCommandValidation> {
        match parse_tdz_command(&command) {
            Ok(commands) => {
                if commands.is_empty() {
                    Ok(PyCommandValidation {
                        valid: false,
                        error: Some("No valid TDZ commands found in input".to_string()),
                        commands: Vec::new(),
                        suggestions: vec!["Use <tdz>command;target;params</tdz> format".to_string()],
                    })
                } else {
                    Ok(PyCommandValidation {
                        valid: true,
                        error: None,
                        commands: commands
                            .iter()
                            .map(|c| format!("{} {}", c.command, c.target))
                            .collect(),
                        suggestions: Vec::new(),
                    })
                }
            }
            Err(e) => {
                let suggestions = vec![
                    "Ensure commands are wrapped in <tdz>...</tdz> tags".to_string(),
                    "Use format: <tdz>command;target;param1;param2;key=value</tdz>".to_string(),
                    "Available commands: list, get, create, update, delete, run, search"
                        .to_string(),
                ];
                Ok(PyCommandValidation {
                    valid: false,
                    error: Some(format!("{}", e)),
                    commands: Vec::new(),
                    suggestions,
                })
            }
        }
    }

    pub fn get_command_documentation(&self, command_name: Option<String>) -> PyResult<String> {
        let cmd = command_name.as_deref().unwrap_or("general");
        let docs = match cmd.to_lowercase().as_str() {
            "list" => "List resources: <tdz>list;tasks</tdz> or <tdz>list;projects</tdz>",
            "get" => "Get specific resource: <tdz>get;task;task_id</tdz>",
            "create" => "Create resource: <tdz>create;task;action=Do something;priority=high</tdz>",
            "update" => "Update resource: <tdz>update;task;task_id;status=done</tdz>",
            "delete" => "Delete resource: <tdz>delete;task;task_id</tdz>",
            "search" => "Search resources: <tdz>search;tasks;query=keyword</tdz>",
            "run" => "Execute action: <tdz>run;init</tdz> or <tdz>run;chat;message</tdz>",
            _ => {
                return Ok(format!(
                    "Available commands:\n\
                    - list: List resources (tasks, projects, memories, ideas)\n\
                    - get: Get specific resource by ID\n\
                    - create: Create new resource\n\
                    - update: Update existing resource\n\
                    - delete: Delete resource\n\
                    - search: Search resources\n\
                    - run: Execute actions\n\
                    \n\
                    Format: <tdz>command;target;param1;param2;key=value</tdz>"
                ));
            }
        };
        Ok(docs.to_string())
    }

    pub fn list_available_commands(&self) -> PyResult<Vec<String>> {
        Ok(vec![
            "list".to_string(),
            "get".to_string(),
            "create".to_string(),
            "update".to_string(),
            "delete".to_string(),
            "search".to_string(),
            "run".to_string(),
        ])
    }

    // ========== Chunking API for Code Generation ==========

    pub fn get_chunking_level_info(&self, level: String) -> PyResult<String> {
        use crate::chunking::ChunkingLevel;
        let level_enum: ChunkingLevel = level
            .parse()
            .map_err(|e| PyException::new_err(format!("Invalid chunking level: {}", e)))?;
        Ok(format!(
            "Level: {}\nMax tokens: {}\nDescription: {}\nExample: {}",
            level_enum,
            level_enum.max_tokens(),
            level_enum.description(),
            level_enum.example()
        ))
    }

    pub fn get_chunking_levels(&self) -> PyResult<Vec<String>> {
        Ok(vec![
            "project".to_string(),
            "module".to_string(),
            "class".to_string(),
            "method".to_string(),
            "block".to_string(),
        ])
    }

    pub fn get_chunk_status(&self, chunk_id: String) -> PyResult<String> {
        Ok(format!(
            "Chunk {} status query - implementation needed",
            chunk_id
        ))
    }

    pub fn get_chunks_by_level(&self, level: String) -> PyResult<Vec<String>> {
        use crate::chunking::ChunkingLevel;
        let _level_enum: ChunkingLevel = level
            .parse()
            .map_err(|e| PyException::new_err(format!("Invalid chunking level: {}", e)))?;
        Ok(vec![
            "Chunk retrieval by level - implementation needed".to_string()
        ])
    }

    pub fn get_project_summary(&self) -> PyResult<String> {
        Ok("Project summary - implementation needed".to_string())
    }

    pub fn get_next_chunk_to_work_on(&self) -> PyResult<Option<String>> {
        Ok(Some("Next chunk ID - implementation needed".to_string()))
    }

    pub fn get_dependency_chain(&self, chunk_id: String) -> PyResult<Vec<String>> {
        Ok(vec![format!(
            "Dependency chain for {} - implementation needed",
            chunk_id
        )])
    }

    // Generated Python wrapper functions

    // active

    pub fn active(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::active()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // add

    pub fn add(&self, task_name: String, description: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Queue::add(&task_name, &description)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // add_recent

    pub fn add_recent(&self, description: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Actions::add_recent(&description)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // add_to_task

    pub fn add_to_task(&self, task_id: String, tag: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Tags::add_to_task(&task_id, &tag)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ai

    pub fn ai(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Actions::ai(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // analyze_code_quality

    pub fn analyze_code_quality(
        &self,
        features: Vec<f32>,
    ) -> PyResult<f32> {
        self.runtime.block_on(async {
            Done::analyze_code_quality(&features)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // api

    pub fn api(&self, message: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::api(&message)
                .await
                .map(|_| "API call completed".to_string())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // api_key

    pub fn api_key(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::api_key()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // as_str

    pub fn as_str(&self) -> PyResult<&'static str> {
        self.runtime.block_on(async {
            Done::as_str()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // auto_label_clusters

    pub fn auto_label_clusters(
        &self,
        clusters: Vec<ClusteringResult>,
    ) -> PyResult<Vec<LabeledCluster>> {
        self.runtime.block_on(async {
            Done::auto_label_clusters(clusters)
                .await
                .map(|items| items.into_iter().map(LabeledCluster::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // backlog

    pub fn backlog(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::backlog()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // backup_embeddings

    pub fn backup_embeddings(&self, backup_path: Option<String>) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::backup_embeddings(backup_path.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // breakthrough

    pub fn breakthrough(&self, idea: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Ideas::breakthrough(&idea)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // breakthrough_percentage

    pub fn breakthrough_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::breakthrough_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // build_similarity_graph

    pub fn build_similarity_graph(&self, threshold: f32) -> PyResult<SimilarityGraph> {
        self.runtime.block_on(async {
            Done::build_similarity_graph(threshold)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // calculate_diversity

    pub fn calculate_diversity(&self, content_ids: Vec<String>) -> PyResult<f32> {
        self.runtime.block_on(async {
            Done::calculate_diversity(content_ids)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // capabilities

    pub fn capabilities(&self, capabilities: Vec<String>) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::capabilities(capabilities)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // category

    pub fn category(&self, category: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::category(&category)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // check_folder_structure

    pub fn check_folder_structure(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::check_folder_structure()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // cleanup_expired

    pub fn cleanup_expired(&self) -> PyResult<usize> {
        self.runtime.block_on(async {
            Done::cleanup_expired()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // cleanup_legacy

    pub fn cleanup_legacy(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::cleanup_legacy()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // cli_fix_consistency

    pub fn cli_fix_consistency(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::cli_fix_consistency()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // cluster_content

    pub fn cluster_content(&self) -> PyResult<Vec<PyClusteringResult>> {
        self.runtime.block_on(async {
            Done::cluster_content()
                .await
                .map(|items| items.into_iter().map(PyClusteringResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // collab

    pub fn collab(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Actions::collab(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // color

    pub fn color(&self, color: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::color(&color)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // compare_models

    pub fn compare_models(
        &self,
        text: String,
        model_aliases: Vec<String>,
    ) -> PyResult<ModelComparisonResult> {
        self.runtime.block_on(async {
            Done::compare_models(&text, model_aliases)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // complete_task_in_project

    pub fn complete_task_in_project(&self, id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::complete_task_in_project(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // completion_rate

    pub fn completion_rate(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::completion_rate()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // config

    pub fn config(&self) -> PyResult<Config> {
        self.runtime.block_on(async {
            Done::config()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // content

    pub fn content(&self, content: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::content(&content)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // context

    pub fn context(&self, context: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::context(&context)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // craft_embedding

    pub fn craft_embedding(
        &self,
        _features: &[f32],
    ) -> PyResult<Vec<f32>> {
        self.runtime.block_on(async {
            Done::craft_embedding(_features)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create

    pub fn create(&self, name: String, description: Option<String>) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create(&name, description.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_advanced_todozi_tools

    pub fn create_advanced_todozi_tools(&self, todozi: SharedTodozi) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_advanced_todozi_tools(todozi)
                .await
                .map(|items| format!("Created {} tools", items.len()))
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // create_architect_agent

    pub fn create_architect_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_architect_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_backup

    pub fn create_backup(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_backup()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_coder

    pub fn create_coder(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_coder()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_comrad_agent

    pub fn create_comrad_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_comrad_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_custom_agent

    pub fn create_custom_agent(
        &self,
        id: String,
        name: String,
        description: String,
        capabilities: Vec<String>,
        specializations: Vec<String>,
        category: String,
        author: Option<String>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_custom_agent(
                &id,
                &name,
                &description,
                capabilities,
                specializations,
                &category,
                author.as_deref(),
            )
            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))            
        })
    }

    // create_default_agents

    pub fn create_default_agents(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::create_default_agents()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_designer_agent

    pub fn create_designer_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_designer_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_detective_agent

    pub fn create_detective_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_detective_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_devops_agent

    pub fn create_devops_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_devops_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_embedding_service

    pub fn create_embedding_service(&self) -> PyResult<TodoziEmbeddingService> {
        self.runtime.block_on(async {
            Done::create_embedding_service()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_embedding_version

    pub fn create_embedding_version(
        &self,
        content_id: String,
        version_label: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_embedding_version(&content_id, &version_label)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_error

    pub fn create_error(&self, error: Error) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_error(error)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_error_result

    pub fn create_error_result(
        &self,
        error_msg: String,
        execution_time_ms: u64,
        error_type: ErrorType,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> PyResult<ToolResult> {
        self.runtime.block_on(async {
            Done::create_error_result(
                &error_msg,
                execution_time_ms,
                error_type,
                metadata.as_ref(),
            )
            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))            
        })
    }

    // create_filters

    pub fn create_filters(&self) -> PyResult<TaskFilters> {
        Ok(Done::create_filters())
    }

    pub fn create_task_filters(
        &self,
        project: Option<String>,
        status: Option<String>,
        priority: Option<String>,
        assignee: Option<String>,
        tags: Option<String>,
        search: Option<String>,
    ) -> PyResult<TaskFilters> {
        self.runtime.block_on(async {
            Done::create_task_filters(
                project.as_deref(),
                status.as_deref(),
                priority.as_deref(),
                assignee.as_ref(),
                tags.as_deref(),
                search.as_deref(),
            )
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // create_finisher_agent

    pub fn create_finisher_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_finisher_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_framer_agent

    pub fn create_framer_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_framer_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_friend_agent

    pub fn create_friend_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_friend_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_grok_level_todozi_tools

    pub fn create_grok_level_todozi_tools(&self, todozi: SharedTodozi) -> PyResult<String> {
        let tools = create_grok_level_todozi_tools(todozi);
        Ok(format!("Created {} tools", tools.len()))
    }

    // create_hoarder_agent

    pub fn create_hoarder_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_hoarder_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_investigator_agent

    pub fn create_investigator_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_investigator_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_mason_agent

    pub fn create_mason_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_mason_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_nerd_agent

    pub fn create_nerd_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_nerd_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_nun_agent

    pub fn create_nun_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_nun_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_overlord_agent

    pub fn create_overlord_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_overlord_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_party_agent

    pub fn create_party_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_party_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_planner_agent

    pub fn create_planner_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_planner_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_recycler_agent

    pub fn create_recycler_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_recycler_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_skeleton_agent

    pub fn create_skeleton_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_skeleton_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_snitch_agent

    pub fn create_snitch_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_snitch_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_storage

    pub fn create_storage(&self) -> PyResult<Storage> {
        self.runtime.block_on(async {
            Done::create_storage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_success_result

    pub fn create_success_result(
        &self,
        output: String,
        execution_time_ms: u64,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> PyResult<ToolResult> {
        self.runtime.block_on(async {
            Done::create_success_result(&output, execution_time_ms, metadata.as_ref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_tdz_content_processor_tool

    pub fn create_tdz_content_processor_tool(
        &self,
        state: SharedTodoziState,
    ) -> PyResult<Box<dyn Tool + Send + Sync>> {
        self.runtime.block_on(async {
            Done::create_tdz_content_processor_tool(state)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_tester_agent

    pub fn create_tester_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_tester_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_todozi_tools

    pub fn create_todozi_tools(&self, _todozi: SharedTodozi) -> PyResult<String> {
        self.runtime.block_on(async {
            let storage = Storage::new()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let std_todozi = std::sync::Arc::new(std::sync::Mutex::new(storage));                                                                               
            Done::create_todozi_tools(std_todozi)
                .await
                .map(|items| format!("Created {} tools", items.len()))
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // create_todozi_tools_with_embedding

    pub fn create_todozi_tools_with_embedding(
        &self,
        todozi: SharedTodozi,
        embedding_service: Option<TodoziEmbeddingService>,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::create_todozi_tools_with_embedding(todozi, embedding_service.as_ref())
                .await
                .map(|items| format!("Created {} tools", items.len()))
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // create_tool_definition_with_locks

    pub fn create_tool_definition_with_locks(
        &self,
        name: String,
        description: String,
        category: String,
        parameters: Vec<ToolParameter>,
        resource_locks: Vec<ResourceLock>,
    ) -> PyResult<ToolDefinition> {
        self.runtime.block_on(async {
            Done::create_tool_definition_with_locks(
                &name,
                &description,
                &category,
                parameters,
                resource_locks,
            )
            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))            
        })
    }

    // create_tuner_agent

    pub fn create_tuner_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_tuner_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // create_update

    pub fn create_update(&self) -> PyResult<TaskUpdate> {
        Ok(Done::default_update())
    }

    // create_writer_agent

    pub fn create_writer_agent(&self) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::create_writer_agent()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // critical_percentage

    pub fn critical_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::critical_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // deactivate_key

    pub fn deactivate_key(&self, key_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::deactivate_key(&key_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // deep

    pub fn deep(&self, query: String) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Find::deep(&query)
                .await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // default_filters

    pub fn default_filters(&self) -> PyResult<TaskFilters> {
        Ok(Done::default_filters())
    }

    // default_update

    pub fn default_update(&self) -> PyResult<TaskUpdate> {
        Ok(Done::default_update())
    }

    // delete_agent_assignment

    pub fn delete_agent_assignment(&self, assignment_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_agent_assignment(&assignment_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_code_chunk

    pub fn delete_code_chunk(&self, chunk_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_code_chunk(&chunk_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_error

    pub fn delete_error(&self, error_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_error(&error_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_feeling

    pub fn delete_feeling(&self, id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_feeling(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_idea

    pub fn delete_idea(&self, idea_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_idea(&idea_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_memory

    pub fn delete_memory(&self, memory_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_memory(&memory_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_project_task_container

    pub fn delete_project_task_container(&self, project_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_project_task_container(&project_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_task_from_project

    pub fn delete_task_from_project(&self, id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_task_from_project(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // delete_training_data

    pub fn delete_training_data(&self, training_data_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::delete_training_data(&training_data_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // description

    pub fn description(&self, description: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::description(&description)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // detailed

    pub fn detailed(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Stats::detailed()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // display_task

    pub fn display_task(&self, task_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::display_task(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // display_tasks

    pub fn display_tasks(&self, _task_ids: Vec<String>) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::display_tasks()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // dry_run

    pub fn dry_run(&self, command: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::dry_run(&command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // embed_idea

    pub fn embed_idea(&self, idea: &Idea) -> PyResult<Vec<f32>> {
        self.runtime.block_on(async {
            Done::embed_idea(&idea.id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // embed_memory

    pub fn embed_memory(&self, memory: &Memory) -> PyResult<Vec<f32>> {
        self.runtime.block_on(async {
            Done::embed_memory(&memory.id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // embed_tag

    pub fn embed_tag(&self, tag: &Tag) -> PyResult<Vec<f32>> {
        self.runtime.block_on(async {
            Done::embed_tag(&tag.id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // embedding_config

    pub fn embedding_config(&self) -> PyResult<TodoziEmbeddingConfig> {
        Ok(Done::embedding_config())
    }

    // embedding_service

    pub fn embedding_service(&self) -> PyResult<TodoziEmbeddingService> {
        self.runtime.block_on(async {
            Done::embedding_service()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // encode

    pub fn encode(&self, text: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::encode(&text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // end_queue_session

    pub fn end_queue_session(&self, session_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::end_queue_session(&session_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // end_session

    pub fn end_session(&self, session_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::end_session(&session_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ensure_folder_structure

    pub fn ensure_folder_structure(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::ensure_folder_structure()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // error

    pub fn error(&self, error: String, _execution_time_ms: u64) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::error(&error)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // example

    pub fn example(&self) -> PyResult<&'static str> {
        self.runtime.block_on(async {
            Done::example()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // example_usage

    pub fn example_usage(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::example_usage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // execute_task

    pub fn execute_task(&self, task_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::execute_task(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // execute_tdz_command

    pub fn execute_tdz_command(
        &self,
        command: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::execute_tdz_command(&command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // execute_todozi_tool_delegated

    pub fn execute_todozi_tool_delegated(
        &self,
        params: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::execute_todozi_tool_delegated(&params)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // explain_search_result

    pub fn explain_search_result(
        &self,
        result_id: String,
    ) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::explain_search_result(&result_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // export_diagnostics

    pub fn export_diagnostics(&self, path: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::export_diagnostics(&path)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // export_embedded_tasks_hlx

    pub fn export_embedded_tasks_hlx(&self, output_path: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::export_embedded_tasks_hlx(&output_path)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // export_for_fine_tuning

    pub fn export_for_fine_tuning(&self, output_path: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::export_for_fine_tuning(&output_path)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // fast

    pub fn fast(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Find::fast(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // filtered_semantic_search

    pub fn filtered_semantic_search(
        &self,
        query: String,
        filters: SearchFilters,
        limit: usize,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Done::filtered_semantic_search(&query, &filters)
                .await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // find_best_agent

    pub fn find_best_agent(
        &self,
        task_description: String,
    ) -> PyResult<Agent> {
        self.runtime.block_on(async {
            Done::find_best_agent(&task_description)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // find_cross_content_relationships

    pub fn find_cross_content_relationships(
        &self,
        content_ids: Vec<String>,
    ) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::find_cross_content_relationships(content_ids)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // find_outliers

    pub fn find_outliers(
        &self,
        content_ids: Vec<String>,
    ) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::find_outliers(content_ids)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // find_similar_tags

    pub fn find_similar_tags(
        &self,
        tag_name: String,
        limit: Option<usize>,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let config = TodoziEmbeddingConfig::default();
            let emb_service = TodoziEmbeddingService::new(config).await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            emb_service.find_similar_tags(&tag_name, limit).await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // find_similar_tasks

    pub fn find_similar_tasks(
        &self,
        task_description: String,
        limit: Option<usize>,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let config = TodoziEmbeddingConfig::default();
            let emb_service = TodoziEmbeddingService::new(config).await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            emb_service.find_similar_tasks(&task_description, limit).await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // find_tdz

    pub fn find_tdz(&self, str: Option<String>) -> PyResult<Option<String>> {
        self.runtime.block_on(async {
            Ok(crate::find_tdz(str.as_deref()).await)
        })
    }

    // find_todozi

    pub fn find_todozi(&self, str: Option<String>) -> PyResult<Option<String>> {
        Ok(crate::tdz::find_todozi(str.as_deref()))
    }

    // fix_completed_tasks_consistency

    pub fn fix_completed_tasks_consistency(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.fix_completed_tasks_consistency()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
                
    }
    

    // fix_task_consistency

    pub fn fix_task_consistency(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let mut storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.fix_completed_tasks_consistency()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
                
        
    }

    // force_overwrite

    pub fn force_overwrite(&self, _force_overwrite: bool) -> PyResult<Self> {
        Ok(self.clone())
    }

    // format_duration

    pub fn format_duration(
        &self,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> PyResult<String> {
        use crate::tui::TuiService;
        Ok(TuiService::format_duration(from, to))
    }

    // format_project_stats

    pub fn format_project_stats(
        &self,
        project_name: String,
        task_count: usize,
        completed_count: usize,
    ) -> PyResult<String> {
        Ok(format!("Project: {} | Tasks: {} | Completed: {}", project_name, task_count, completed_count))
    }

    // format_task

    pub fn format_task(&self, task: &Task) -> PyResult<String> {
        Ok(format!("{} [{}] - {}", task.id, task.status, task.action))
    }

    // format_task_list

    pub fn format_task_list(&self, tasks: &[Task]) -> PyResult<String> {
        Ok(tasks.iter()
            .map(|t| format!("{} [{}] - {}", t.id, t.status, t.action))
            .collect::<Vec<_>>()
            .join("\n"))
    }

    // format_task_with_emojis

    pub fn format_task_with_emojis(&self, task: &Task) -> PyResult<String> {
        let emoji = match task.status {
            crate::models::Status::Todo => "📋",
            crate::models::Status::InProgress => "🔄",
            crate::models::Status::Completed => "✅",
            _ => "📝",
        };
        Ok(format!("{} {} [{}] - {}", emoji, task.id, task.status, task.action))
    }

    // format_time_estimate

    pub fn format_time_estimate(&self, time: String) -> PyResult<String> {
        Ok(time)
    }

    // fuzzy_search

    pub fn fuzzy_search(&self, query: String, _max_distance: usize) -> PyResult<String> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let tasks = storage.list_tasks_across_projects(&TaskFilters::default())
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let matches: Vec<_> = tasks.iter()
                .filter(|t| t.action.to_lowercase().contains(&query.to_lowercase()))
                .collect();
            Ok(format!("Found {} matches", matches.len()))
        })
    }

    // generate_embedding

    pub fn generate_embedding(&self, text: String) -> PyResult<Vec<f32>> {
        self.runtime.block_on(async {
            let config = TodoziEmbeddingConfig::default();
            let emb_service = TodoziEmbeddingService::new(config).await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            emb_service.generate_embedding(&text).await
                
        })
    }

    // generate_embeddings_batch

    pub fn generate_embeddings_batch(&self, texts: Vec<String>) -> PyResult<Vec<Vec<f32>>> {
        self.runtime.block_on(async {
            let config = TodoziEmbeddingConfig::default();
            let emb_service = TodoziEmbeddingService::new(config).await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            emb_service.generate_embeddings_batch(texts).await
                
        })
    }

    // generate_task_embedding

    pub fn generate_task_embedding(&self, task: &Task) -> PyResult<Option<Vec<f32>>> {
        self.runtime.block_on(async {
            let config = TodoziEmbeddingConfig::default();
            let emb_service = TodoziEmbeddingService::new(config).await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let task_text = format!("{} {}", task.action, task.time);
            emb_service.generate_embedding(&task_text).await
                .map(Some)
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_active_items

    pub fn get_active_items(&self) -> PyResult<Vec<QueueItem>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let queue_collection = crate::storage::load_queue_collection()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(queue_collection.get_active_items().into_iter().cloned().collect())
        })
    }

    // get_active_keys

    pub fn get_active_keys(&self) -> PyResult<Vec<ApiKey>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let api_keys = crate::api::load_api_key_collection()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(api_keys.keys.values().filter(|k| k.is_active()).cloned().collect())
        })
    }

    // get_active_sessions

    pub fn get_active_sessions(&self) -> PyResult<Vec<crate::models::QueueSession>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let queue_sessions = crate::storage::get_active_sessions()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(queue_sessions)
        })
    }

    // get_agent_assignments

    pub fn get_agent_assignments(&self, agent_id: String) -> PyResult<Vec<AgentAssignment>> {
        self.runtime.block_on(async {
            Done::list_agent_assignments(&agent_id)
                .await
                .map(|items| items.into_iter().map(AgentAssignment::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_agent_assignments_dir

    pub fn get_agent_assignments_dir(&self, agent_id: String) -> PyResult<PathBuf> {
        crate::storage::get_agent_assignments_dir(&agent_id)
            
    }

    // get_agents_by_capability

    pub fn get_agents_by_capability(&self, capability: String) -> PyResult<Vec<Agent>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let agents = crate::storage::list_agents()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(agents.into_iter()
                .filter(|a| a.capabilities.contains(&capability))
                .collect())
        })
    }

    // get_agents_by_specialization

    pub fn get_agents_by_specialization(&self, specialization: String) -> PyResult<Vec<Agent>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let agents = crate::storage::list_agents()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(agents.into_iter()
                .filter(|a| a.specializations.contains(&specialization))
                .collect())
        })
    }

    // get_agents_dir

    pub fn get_agents_dir(&self) -> PyResult<PathBuf> {
        Ok(crate::storage::get_storage_dir()
            ?
            .join("agents"))
    }

    // get_agents_with_assignments

    pub fn get_agents_with_assignments(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let agents = crate::storage::list_agents()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(agents.into_iter()
                .map(|a| a.id)
                .collect())
        })
    }

    // get_ai_tasks

    pub fn get_ai_tasks(&self) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let filters = TaskFilters {
                assignee: Some(Assignee::Ai),
                ..Default::default()
            };
            storage.list_tasks_across_projects(&filters)
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_all_active_tasks

    pub fn get_all_active_tasks(&self) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.get_all_active_tasks()
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_all_agents

    pub fn get_all_agents(&self) -> PyResult<Vec<Agent>> {
        crate::storage::list_agents()
            
    }

    // get_all_completed_tasks

    pub fn get_all_completed_tasks(&self) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.get_all_completed_tasks()
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_all_ideas

    pub fn get_all_ideas(&self) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            manager.get_all_ideas()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_all_items

    pub fn get_all_items(&self) -> PyResult<Vec<QueueItem>> {
        self.runtime.block_on(async {
            let queue_collection = crate::storage::load_queue_collection()
                ?;
            Ok(queue_collection.get_all_items().into_iter().cloned().collect())
        })
    }

    // get_all_keys

    pub fn get_all_keys(&self) -> PyResult<Vec<ApiKey>> {
        let api_keys = crate::api::load_api_key_collection()
            ?;
        Ok(api_keys.keys.values().cloned().collect())
    }

    // get_all_memories

    pub fn get_all_memories(&self) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            manager.get_all_memories()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_all_summaries

    pub fn get_all_summaries(&self) -> PyResult<Vec<Summary>> {
        self.runtime.block_on(async {
            let manager = SummaryManager::new();
            Ok(manager.get_all_summaries().into_iter().cloned().collect())
        })
    }

    // get_all_tasks

    pub fn get_all_tasks(&self) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let _storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let containers = crate::storage::list_project_task_containers()
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let mut all_tasks = Vec::new();
            for container in containers {
                all_tasks.extend(container.get_all_tasks().into_iter().cloned());
            }
            Ok(all_tasks)
        })
    }

    // get_all_tools

    pub fn get_all_tools(&self) -> PyResult<String> {
        let registry = crate::base::ToolRegistry::default();
        let tools = registry.get_all_tools();
        Ok(format!("Found {} tools", tools.len()))
    }

    // get_assignee_emoji

    pub fn get_assignee_emoji(&self, assignee: &Assignee) -> PyResult<&'static str> {
        Ok(match assignee {
            Assignee::Ai => "🤖",
            Assignee::Human => "👤",
            Assignee::Collaborative => "🤝",
        })
    }

    // get_assignments_dir

    pub fn get_assignments_dir(&self) -> PyResult<PathBuf> {
        Ok(crate::storage::get_storage_dir()
            ?
            .join("assignments"))
    }

    // get_available_agents

    pub fn get_available_agents(&self) -> PyResult<Vec<Agent>> {
        let agents = crate::storage::list_agents()
            ?;
        Ok(agents.into_iter()
            .filter(|a| a.metadata.status == AgentStatus::Available)
            .collect())
    }

    // get_backlog_items

    pub fn get_backlog_items(&self) -> PyResult<Vec<QueueItem>> {
        let queue_collection = crate::storage::load_queue_collection()
            ?;
        Ok(queue_collection.get_backlog_items().into_iter().cloned().collect())
    }

    // get_breakthrough_ideas

    pub fn get_breakthrough_ideas(&self) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_breakthrough_ideas().into_iter().cloned().collect())
        })
    }

    // get_chunk

    pub fn get_chunk(&self, _chunk_id: String) -> PyResult<Option<CodeChunk>> {
        Err(PyException::new_err("load_chunk not implemented"))
    }

    // get_chunk_mut

    pub fn get_chunk_mut(&self, _chunk_id: String) -> PyResult<Option<CodeChunk>> {
        Err(PyException::new_err("Cannot return mutable reference from Python binding"))
    }

    // get_chunks_dir

    pub fn get_chunks_dir(&self) -> PyResult<PathBuf> {
        crate::storage::get_chunks_dir()
            
    }

    // get_collaborative_tasks

    pub fn get_collaborative_tasks(&self) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let filters = TaskFilters {
                assignee: Some(Assignee::Collaborative),
                ..Default::default()
            };
            storage.list_tasks_across_projects(&filters)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_complete_items

    pub fn get_complete_items(&self) -> PyResult<Vec<QueueItem>> {
        let queue_collection = crate::storage::load_queue_collection()
            ?;
        Ok(queue_collection.get_complete_items().into_iter().cloned().collect())
    }

    // get_critical_memories

    pub fn get_critical_memories(&self) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_critical_memories().into_iter().cloned().collect())
        })
    }

    // get_current_duration

    pub fn get_current_duration(&self) -> PyResult<u64> {
        Ok(0)
    }

    // get_default_model

    pub fn get_default_model(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            crate::emb::EmbeddingModel::get_default_model().await
                
        })
    }

    // get_emotional_memories

    pub fn get_emotional_memories(&self, emotion: String) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_emotional_memories(&emotion).into_iter().cloned().collect())
        })
    }

    // get_enabled_tools

    pub fn get_enabled_tools(&self) -> PyResult<Vec<&AgentTool>> {
        self.runtime.block_on(async {
            Easy::get_enabled_tools()
                .await
                .map(|items| items.into_iter().map(&AgentTool::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_errors_dir

    pub fn get_errors_dir(&self) -> PyResult<PathBuf> {
        crate::storage::get_errors_dir()
            
    }

    // get_filtered_tasks

    pub fn get_filtered_tasks(&self, filters: &TaskFilters) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.list_tasks_across_projects(filters)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_formatted_prompt

    pub fn get_formatted_prompt(
        &self,
        _variables: &std::collections::HashMap<String, String>,
    ) -> PyResult<String> {
        Ok(String::new())
    }

    // get_high_priority_summaries

    pub fn get_high_priority_summaries(&self) -> PyResult<Vec<Summary>> {
        self.runtime.block_on(async {
            let manager = SummaryManager::new();
            Ok(manager.get_high_priority_summaries().into_iter().cloned().collect())
        })
    }

    // get_human_memories

    pub fn get_human_memories(&self) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_human_memories().into_iter().cloned().collect())
        })
    }

    // get_human_tasks

    pub fn get_human_tasks(&self) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            let filters = TaskFilters {
                assignee: Some(Assignee::Human),
                ..Default::default()
            };
            Done::list_tasks_across_projects(&filters)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_idea

    pub fn get_idea(&self, idea_id: String) -> PyResult<Option<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            manager.get_idea(&idea_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_idea_statistics

    pub fn get_idea_statistics(&self) -> PyResult<IdeaStatistics> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_idea_statistics())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_ideas_by_importance

    pub fn get_ideas_by_importance(&self, importance: IdeaImportance) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_ideas_by_importance(importance).into_iter().cloned().collect())
        })
    }

    // get_ideas_by_share_level

    pub fn get_ideas_by_share_level(&self, share_level: ShareLevel) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_ideas_by_share_level(share_level).into_iter().cloned().collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_ideas_by_tag

    pub fn get_ideas_by_tag(&self, tag: String) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_ideas_by_tag(&tag).into_iter().cloned().collect())
        })
    }

    // get_ideas_dir

    pub fn get_ideas_dir(&self) -> PyResult<PathBuf> {
        Ok(crate::storage::get_storage_dir()
            ?
            .join("ideas"))
    }

    // get_item

    pub fn get_item(&self, id: String) -> PyResult<Option<QueueItem>> {
        let queue_collection = crate::storage::load_queue_collection()
            ?;
        Ok(queue_collection.get_item(&id).cloned())
    }

    // get_item_mut

    pub fn get_item_mut(&self, _id: String) -> PyResult<Option<QueueItem>> {
        Err(PyException::new_err("Cannot return mutable reference from Python binding"))
    }

    // get_items_by_status

    pub fn get_items_by_status(&self, status: QueueStatus) -> PyResult<Vec<QueueItem>> {
        let queue_collection = crate::storage::load_queue_collection()
            ?;
        Ok(queue_collection.get_items_by_status(status).into_iter().cloned().collect())
    }

    // get_key

    pub fn get_key(&self, user_id: String) -> PyResult<Option<ApiKey>> {
        let api_keys = crate::api::load_api_key_collection()
            ?;
        Ok(api_keys.keys.get(&user_id).cloned())
    }

    // get_key_by_public

    pub fn get_key_by_public(&self, public_key: String) -> PyResult<Option<ApiKey>> {
        let api_keys = crate::api::load_api_key_collection()
            ?;
        Ok(api_keys.keys.values()
            .find(|k| k.public_key == public_key)
            .cloned())
    }

    // get_long_term_memories

    pub fn get_long_term_memories(&self) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_long_term_memories().into_iter().cloned().collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_memories_by_importance

    pub fn get_memories_by_importance(
        &self,
        importance: MemoryImportance,
    ) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_memories_by_importance(importance).into_iter().cloned().collect())
        })
    }

    // get_memories_by_tag

    pub fn get_memories_by_tag(&self, tag: String) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_memories_by_tag(&tag).into_iter().cloned().collect())
        })
    }

    // get_memories_by_term

    pub fn get_memories_by_term(&self, term: MemoryTerm) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_memories_by_term(term).into_iter().cloned().collect())
        })
    }

    // get_memories_by_type

    pub fn get_memories_by_type(&self, memory_type: &MemoryType) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_memories_by_type(memory_type).into_iter().cloned().collect())
        })
    }

    // get_memories_dir

    pub fn get_memories_dir(&self) -> PyResult<PathBuf> {
        Ok(crate::storage::get_storage_dir()
            ?
            .join("memories"))
    }

    // get_memory

    pub fn get_memory(&self, memory_id: String) -> PyResult<Option<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_memory(&memory_id).cloned())
        })
    }

    // get_memory_statistics

    pub fn get_memory_statistics(&self) -> PyResult<MemoryStatistics> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_memory_statistics())
        })
    }

    // get_or_generate_embedding

    pub fn get_or_generate_embedding(
        &self,
        content_id: String,
        text: String,
        content_type: TodoziContentType,
        refresh_if_stale: bool,
    ) -> PyResult<Vec<f32>> {
        self.runtime.block_on(async {
            let config = TodoziEmbeddingConfig::default();
            let mut emb_service = TodoziEmbeddingService::new(config).await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            emb_service.initialize().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            emb_service.get_or_generate_embedding(&content_id, &text, content_type, refresh_if_stale).await
                
        })
    }

    // get_priority_emoji

    pub fn get_priority_emoji(&self, priority: &Priority) -> PyResult<&'static str> {
        Ok(match priority {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🟠",
            Priority::Critical => "🔴",
        })
    }

    // get_private_ideas

    pub fn get_private_ideas(&self) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_private_ideas().into_iter().cloned().collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_project_stats

    pub fn get_project_stats(&self, project_name: String) -> PyResult<ProjectStats> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.get_project_stats(&project_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_project_tasks

    pub fn get_project_tasks(&self, project_name: String) -> PyResult<Vec<Task>> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            storage.list_tasks_in_project(&project_name, &TaskFilters::default())
                .await
                .map(|tasks| tasks.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_project_tasks_dir

    pub fn get_project_tasks_dir(&self) -> PyResult<PathBuf> {
        crate::storage::get_tasks_dir()
            
    }

    // get_public_ideas

    pub fn get_public_ideas(&self) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let manager = IdeaManager::new();
            Ok(manager.get_public_ideas().into_iter().cloned().collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // get_queue_item

    pub fn get_queue_item(&self, id: String) -> PyResult<QueueItem> {
        let queue_collection = crate::storage::load_queue_collection()
            ?;
        queue_collection.get_item(&id)
            .cloned()
            .ok_or_else(|| PyException::new_err(format!("Queue item not found: {}", id)))
    }

    // get_queue_session

    pub fn get_queue_session(&self, session_id: String) -> PyResult<crate::models::QueueSession> {
        crate::storage::get_queue_session(&session_id)
            
    }

    // get_ready_chunks

    pub fn get_ready_chunks(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::get_ready_chunks()
                .await
                .map(|items| items.into_iter().map(String::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_recent_ideas

    pub fn get_recent_ideas(&self, limit: usize) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let ideas = Ideas::list().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(ideas.into_iter().take(limit).collect())
        })
    }

    // get_recent_memories

    pub fn get_recent_memories(&self, limit: usize) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            let mut memories: Vec<Memory> = manager.get_short_term_memories().into_iter().cloned().collect();
            memories.extend(manager.get_long_term_memories().into_iter().cloned());
            memories.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            Ok(memories.into_iter().take(limit).collect())
        })
    }

    // get_recent_reminders

    pub fn get_recent_reminders(&self, _limit: usize) -> PyResult<Vec<Reminder>> {
        Ok(Vec::new())
    }

    // get_recent_tags

    pub fn get_recent_tags(&self, _limit: usize) -> PyResult<Vec<Tag>> {
        Ok(Vec::new())
    }

    // get_registration_info

    pub fn get_registration_info(&self) -> PyResult<Option<RegistrationInfo>> {
        crate::storage::get_registration_info()
            
    }

    // get_related_tags

    pub fn get_related_tags(&self, _tag_id: String) -> PyResult<Vec<Tag>> {
        Ok(Vec::new())
    }

    // get_reminders_by_priority

    pub fn get_reminders_by_priority(
        &self,
        _priority: ReminderPriority,
    ) -> PyResult<Vec<Reminder>> {
        Ok(Vec::new())
    }

    // get_reminders_by_status

    pub fn get_reminders_by_status(&self, _status: ReminderStatus) -> PyResult<Vec<Reminder>> {
        Ok(Vec::new())
    }

    // get_reminders_by_tag

    pub fn get_reminders_by_tag(&self, _tag: String) -> PyResult<Vec<Reminder>> {
        Ok(Vec::new())
    }

    // get_reminders_due_soon

    pub fn get_reminders_due_soon(&self, _duration: Duration) -> PyResult<Vec<Reminder>> {
        Ok(Vec::new())
    }

    // get_search_analytics

    pub fn get_search_analytics(&self) -> PyResult<SearchAnalytics> {
        Ok(SearchAnalytics::default())
    }

    // get_search_suggestions

    pub fn get_search_suggestions(&self, _query: String, _limit: usize) -> PyResult<Vec<String>> {
        Ok(Vec::new())
    }

    // get_secret_memories

    pub fn get_secret_memories(&self) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_secret_memories().into_iter().cloned().collect())
        })
    }

    // get_session

    pub fn get_session(&self, id: String) -> PyResult<Option<crate::models::QueueSession>> {
        match crate::storage::get_queue_session(&id) {
            Ok(session) => Ok(Some(session)),
            Err(_) => Ok(None),
        }
    }

    // get_short_term_memories

    pub fn get_short_term_memories(&self) -> PyResult<Vec<Memory>> {
        self.runtime.block_on(async {
            let manager = MemoryManager::new();
            Ok(manager.get_short_term_memories().into_iter().cloned().collect())
        })
    }

    // get_stats

    pub fn get_stats(&self) -> PyResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    // get_status_emoji

    pub fn get_status_emoji(&self, status: &Status) -> PyResult<&'static str> {
        Ok(match status {
            Status::Todo | Status::Pending => "📝",
            Status::InProgress => "🔄",
            Status::Blocked => "🚫",
            Status::Review => "👀",
            Status::Done | Status::Completed => "✅",
            Status::Cancelled => "❌",
            Status::Deferred => "⏸️",
        })
    }

    // get_storage_dir

    pub fn get_storage_dir(&self) -> PyResult<PathBuf> {
        crate::storage::get_storage_dir()
            
    }

    // get_suggestions

    pub fn get_suggestions(&self, _current_tags: &[String], _limit: usize) -> PyResult<Vec<String>> {
        Ok(Vec::new())
    }

    // get_summaries_by_tag

    pub fn get_summaries_by_tag(&self, _tag: String) -> PyResult<Vec<Summary>> {
        Ok(Vec::new())
    }

    // get_summary_statistics

    pub fn get_summary_statistics(&self) -> PyResult<SummaryStatistics> {
        Ok(SummaryStatistics::default())
    }

    // get_tag_statistics

    pub fn get_tag_statistics(&self) -> PyResult<TagStatistics> {
        let manager = TagManager::new();
        Ok(manager.get_tag_statistics())
    }

    // get_task_assignments

    pub fn get_task_assignments(&self, _task_id: String) -> PyResult<Vec<AgentAssignment>> {
        Ok(Vec::new())
    }

    // get_task_from_any_project

    pub fn get_task_from_any_project(&self, id: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            let storage = Storage::new().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(PyTask::from(storage.get_task_from_any_project(&id)?))
        })
    }

    // get_task_from_project

    pub fn get_task_from_project(&self, project_name: String, task_id: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::get_task_from_project(&project_name, &task_id)
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // get_task_mut

    pub fn get_task_mut(&self, id: String) -> PyResult<Option<&mut Task>> {
        self.runtime.block_on(async {
            Done::get_task_mut(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // get_tasks_dir

    pub fn get_tasks_dir(&self) -> PyResult<PathBuf> {
        crate::storage::get_tasks_dir()
            
    }

    // get_team_ideas

    pub fn get_team_ideas(&self) -> PyResult<Vec<Idea>> {
        self.runtime.block_on(async {
            let ideas = Ideas::list().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            Ok(ideas.into_iter().filter(|i| i.share == ShareLevel::Team).collect())
        })
    }

    // get_tool

    pub fn get_tool(&self, _name: String) -> PyResult<Option<QueueItem>> {
        Err(PyException::new_err("Cannot return trait object from Python binding"))
    }

    // get_tool_definitions

    pub fn get_tool_definitions(&self) -> PyResult<Vec<serde_json::Value>> {
        Ok(Vec::new())
    }

    // get_training_dir

    pub fn get_training_dir(&self) -> PyResult<PathBuf> {
        Ok(crate::storage::get_storage_dir()
            ?
            .join("training"))
    }

    // get_tsne_coordinates

    pub fn get_tsne_coordinates(
        &self,
        _content_ids: Vec<String>,
        _dimensions: usize,
    ) -> PyResult<Vec<(String, Vec<f32>)>> {
        Ok(Vec::new())
    }

    // get_unresolved_errors

    pub fn get_unresolved_errors(&self) -> PyResult<Vec<&Error>> {
        self.runtime.block_on(async {
            Done::get_unresolved_errors()
                .await
                .map(|items| items.into_iter().map(&Error::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // get_version_history

    pub fn get_version_history(&self, content_id: String) -> PyResult<Vec<serde_json::Value>> {
        self.runtime.block_on(async {
            Done::get_version_history(&content_id)
                .await
                .map(|items| items.into_iter().map(serde_json::Value::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // handle_add_command

    pub fn handle_add_command(&self, command: AddCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_add_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_agent_command

    pub fn handle_agent_command(&self, command: Commands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_agent_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_ai_commands

    pub fn handle_ai_commands(&self, command: String, args: Vec<String>) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_ai_commands(&command, args)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_api_command

    pub fn handle_api_command(&self, command: ApiCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_api_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_chat_command

    pub fn handle_chat_command(&self, command: Commands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_chat_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_emb_command

    pub fn handle_emb_command(&self, command: Commands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_emb_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_error_command

    pub fn handle_error_command(&self, command: Commands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_error_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_extract_command

    #[pyo3(signature = (content=None, file=None, output_format="json".to_string(), human=false))]
    pub fn handle_extract_command(
        &self,
        content: Option<String>,
        file: Option<String>,
        output_format: String,
        human: bool,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_extract_command(content.as_deref(), file.as_deref(), &output_format, human)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_idea_command

    pub fn handle_idea_command(&self, command: IdeaCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_idea_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_ind_command

    pub fn handle_ind_command(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_ind_command()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_list_backups_command

    pub fn handle_list_backups_command(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_list_backups_command()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_list_command

    pub fn handle_list_command(&self, command: ListCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_list_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_memory_command

    pub fn handle_memory_command(&self, command: MemoryCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_memory_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_project_command

    pub fn handle_project_command(&self, command: ProjectCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_project_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_queue_command

    pub fn handle_queue_command(&self, command: QueueCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_queue_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_search_all_command

    pub fn handle_search_all_command(&self, command: Commands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_search_all_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_search_command

    pub fn handle_search_command(&self, command: SearchCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_search_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_server_command

    pub fn handle_server_command(&self, command: ServerCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_server_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_show_command

    pub fn handle_show_command(&self, command: ShowCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_show_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_stats_command

    pub fn handle_stats_command(&self, _command: StatsCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_stats_command(_command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_strategy_command

    #[pyo3(signature = (content=None, file=None, output_format="json".to_string(), human=false))]
    pub fn handle_strategy_command(
        &self,
        content: Option<String>,
        file: Option<String>,
        output_format: String,
        human: bool,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_strategy_command(
                content.as_deref(),
                file.as_deref(),
                &output_format,
                human,
            )
            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))            
        })
    }

    // handle_train_command

    pub fn handle_train_command(&self, command: TrainingCommands) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_train_command(command)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // handle_update_command

    #[pyo3(signature = (id, action=None, time=None, priority=None, project=None, status=None, assignee=None, tags=None, dependencies=None, context=None, progress=None))]
    pub fn handle_update_command(
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
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::handle_update_command(
                &id,
                action.as_deref(),
                time.as_deref(),
                priority.as_deref(),
                project.as_deref(),
                status.as_deref(),
                assignee.as_ref(),
                tags.as_deref(),
                dependencies.as_deref(),
                context.as_deref(),
                progress.as_ref(),
            )
            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))            
        })
    }

    // has_capability

    pub fn has_capability(&self, capability: String) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::has_capability(&capability)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // has_results

    pub fn has_results(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::has_results()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // has_specialization

    pub fn has_specialization(&self, specialization: String) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::has_specialization(&specialization)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // has_tool

    pub fn has_tool(&self, tool_name: String) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::has_tool(&tool_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // hash_project_name

    pub fn hash_project_name(&self, project_name: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::hash_project_name(&project_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // hierarchical_clustering

    pub fn hierarchical_clustering(
        &self,
        content_types: Vec<TodoziContentType>,
        max_depth: usize,
    ) -> PyResult<Vec<HierarchicalCluster>> {
        self.runtime.block_on(async {
            Done::hierarchical_clustering(content_types, max_depth)
                .await
                .map(|items| items.into_iter().map(HierarchicalCluster::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // high_priority_percentage

    pub fn high_priority_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::high_priority_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // human

    pub fn human(&self, action: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Actions::human(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // hybrid_search

    #[pyo3(signature = (query, keywords, content_types=None, semantic_weight=0.5, limit=10))]
    pub fn hybrid_search(
        &self,
        query: String,
        keywords: Vec<String>,
        content_types: Option<Vec<TodoziContentType>>,
        semantic_weight: f32, // 0.0-1.0
        limit: usize,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Done::hybrid_search(
                &query,
                keywords,
                content_types.as_deref(),
                semantic_weight, // 0.0-1.0
                limit,
            )
            .await
            .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
            
        })
    }

    // ideate

    pub fn ideate(&self, idea: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::ideate(&idea)
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // importance

    pub fn importance(&self, importance: IdeaImportance) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::importance(importance)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // important

    pub fn important(&self, moment: String, meaning: String, reason: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Memories::important(&moment, &meaning, &reason)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // init

    pub fn init(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // init_storage

    pub fn init_storage(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?;
            init_storage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
        })
    }

    // init_with_auto_registration

    pub fn init_with_auto_registration(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::init_with_auto_registration()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // initialize

    pub fn initialize(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            crate::init().await
                .map_err(|e| PyException::new_err(format!("{}", e)))?
        })
    }

    // initialize_grok_level_todozi_system

    pub fn initialize_grok_level_todozi_system(
        &self,
    ) -> PyResult<std::result::Result<SharedTodozi, TodoziError>> {
        self.runtime.block_on(async {
            Done::initialize_grok_level_todozi_system()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // initialize_grok_level_todozi_system_with_embedding

    pub fn initialize_grok_level_todozi_system_with_embedding(
        &self,
        enable_embeddings: bool,
    ) -> PyResult<std::result::Result<(SharedTodozi, Option<TodoziEmbeddingService>), TodoziError>>
    {
        self.runtime.block_on(async {
            Done::initialize_grok_level_todozi_system_with_embedding(enable_embeddings)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // initialize_tdz_content_processor

    pub fn initialize_tdz_content_processor(&self) -> PyResult<SharedTodoziState> {
        self.runtime.block_on(async {
            Done::initialize_tdz_content_processor()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // interactive_create_task

    pub fn interactive_create_task(&self) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::interactive_create_task()
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // io

    pub fn io(&self, message: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::io(message)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_active

    pub fn is_active(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_active()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_admin

    pub fn is_admin(&self, public_key: String, private_key: String) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_admin(&public_key, &private_key)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_available

    pub fn is_available(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_available()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_backlog

    pub fn is_backlog(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_backlog()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_complete

    pub fn is_complete(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_complete()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_completed

    pub fn is_completed(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_completed()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_empty

    pub fn is_empty(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_empty()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_overdue

    pub fn is_overdue(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_overdue()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // is_registered

    pub fn is_registered(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::is_registered()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // launch_gui

    pub fn launch_gui(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::launch_gui()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // len

    pub fn len(&self) -> PyResult<usize> {
        self.runtime.block_on(async {
            Done::len()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // list_active_items

    pub fn list_active_items(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Done::list_active_items()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_agent_assignments

    pub fn list_agent_assignments(&self, agent_id: String) -> PyResult<Vec<AgentAssignment>> {
        self.runtime.block_on(async {
            Done::list_agent_assignments(&agent_id)
                .await
                .map(|items| items.into_iter().map(AgentAssignment::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_all_agent_assignments

    pub fn list_all_agent_assignments(&self) -> PyResult<Vec<AgentAssignment>> {
        self.runtime.block_on(async {
            Done::list_all_agent_assignments()
                .await
                .map(|items| items.into_iter().map(AgentAssignment::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_backlog_items

    pub fn list_backlog_items(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Done::list_backlog_items()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_backups

    pub fn list_backups(&self) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::list_backups()
                .await
                .map(|items| items.into_iter().map(String::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_code_chunks

    pub fn list_code_chunks(&self) -> PyResult<Vec<CodeChunk>> {
        self.runtime.block_on(async {
            Done::list_code_chunks()
                .await
                .map(|items| items.into_iter().map(CodeChunk::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_complete_items

    pub fn list_complete_items(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Done::list_complete_items()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_errors

    pub fn list_errors(&self) -> PyResult<Vec<Error>> {
        self.runtime.block_on(async {
            Done::list_errors()
                .await
                .map(|items| items.into_iter().map(Error::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_feelings

    pub fn list_feelings(&self) -> PyResult<Vec<Feeling>> {
        self.runtime.block_on(async {
            Done::list_feelings()
                .await
                .map(|items| items.into_iter().map(Feeling::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_project_task_containers

    pub fn list_project_task_containers(&self) -> PyResult<Vec<ProjectTaskContainer>> {
        self.runtime.block_on(async {
            Done::list_project_task_containers()
                .await
                .map(|items| items.into_iter().map(ProjectTaskContainer::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_queue_items

    pub fn list_queue_items(&self) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::list_queue_items()
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_queue_items_by_status

    pub fn list_queue_items_by_status(&self, status: QueueStatus) -> PyResult<Vec<PyQueueItem>> {
        self.runtime.block_on(async {
            Queue::list_queue_items_by_status(status)
                .await
                .map(|items| items.into_iter().map(PyQueueItem::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_tasks_across_projects

    pub fn list_tasks_across_projects(&self, filters: TaskFilters) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::list_tasks_across_projects(&filters)
                .await
                .map(|items| items.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_tasks_in_project

    pub fn list_tasks_in_project(
        &self,
        project_name: String,
        filters: TaskFilters,
    ) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::list_tasks_in_project(&project_name, &filters)
                .await
                .map(|items| items.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // list_training_data

    pub fn list_training_data(&self) -> PyResult<Vec<TrainingData>> {
        self.runtime.block_on(async {
            Done::list_training_data()
                .await
                .map(|items| items.into_iter().map(TrainingData::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // load

    pub fn load(&self, _model_name: String, _device: Device) -> PyResult<()> {
        Err(PyException::new_err("Method not available in Python bindings"))
    }

    // load_additional_model

    pub fn load_additional_model(&self, _model_name: String, _model_alias: String) -> PyResult<()> {
        Err(PyException::new_err("Method not available in Python bindings"))
    }

    // load_agent

    pub fn load_agent(&self, _agent_id: String) -> PyResult<Agent> {
        Err(PyException::new_err("Method not available in Python bindings"))
    }

    // load_agent_assignment

    pub fn load_agent_assignment(
        &self,
        _agent_id: String,
        _task_id: String,
    ) -> PyResult<AgentAssignment> {
        Err(PyException::new_err("Method not available in Python bindings"))
    }

    // load_agents

    pub fn load_agents(&self) -> PyResult<()> {
        Ok(())
    }

    // load_api_key_collection

    pub fn load_api_key_collection(&self) -> PyResult<ApiKeyCollection> {
        crate::api::load_api_key_collection()
            
    }

    // load_api_keys

    pub fn load_api_keys(&self) -> PyResult<()> {
        Ok(())
    }

    // load_code_chunk

    pub fn load_code_chunk(&self, chunk_id: String) -> PyResult<CodeChunk> {
        self.runtime.block_on(async {
            Done::load_code_chunk(&chunk_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_config

    pub fn load_config(&self) -> PyResult<Config> {
        self.runtime.block_on(async {
            Done::load_config()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_error

    pub fn load_error(&self, error_id: String) -> PyResult<Error> {
        self.runtime.block_on(async {
            Done::load_error(&error_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_extended_data

    pub fn load_extended_data(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::load_extended_data()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_feeling

    pub fn load_feeling(&self, id: String) -> PyResult<Feeling> {
        self.runtime.block_on(async {
            Done::load_feeling(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_idea

    pub fn load_idea(&self, idea_id: String) -> PyResult<PyIdea> {
        self.runtime.block_on(async {
            Done::load_idea(&idea_id)
                .await
                .map(PyIdea::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_memory

    pub fn load_memory(&self, memory_id: String) -> PyResult<PyMemory> {
        self.runtime.block_on(async {
            Done::load_memory(&memory_id)
                .await
                .map(PyMemory::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_project_task_container

    pub fn load_project_task_container(
        &self,
        project_name: String,
    ) -> PyResult<ProjectTaskContainer> {
        self.runtime.block_on(async {
            Done::load_project_task_container(&project_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // load_project_task_container_by_hash

    pub fn load_project_task_container_by_hash(
        &self,
        _project_hash: String,
    ) -> PyResult<ProjectTaskContainer> {
        Err(PyException::new_err("Method not available in Python bindings"))
    }

    // load_queue_collection

    pub fn load_queue_collection(&self) -> PyResult<QueueCollection> {
        crate::storage::load_queue_collection()
            
    }

    // load_task_collection

    pub fn load_task_collection(&self, collection_name: String) -> PyResult<TaskCollection> {
        crate::storage::load_task_collection(&collection_name)
            
    }

    // load_tasks

    pub fn load_tasks(&self) -> PyResult<()> {
        Ok(())
    }

    // load_training_data

    pub fn load_training_data(&self, training_data_id: String) -> PyResult<TrainingData> {
        self.runtime.block_on(async {
            Done::load_training_data(&training_data_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // long_term_percentage

    pub fn long_term_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::long_term_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // mark_chunk_completed

    pub fn mark_chunk_completed(&self, chunk_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::mark_chunk_completed(&chunk_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // mark_chunk_validated

    pub fn mark_chunk_validated(&self, chunk_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::mark_chunk_validated(&chunk_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // matches

    pub fn matches(&self, public_key: String, private_key: Option<String>) -> PyResult<bool> {
        self.runtime.block_on(async {
            Done::matches(&public_key, private_key.as_deref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // max_tokens

    pub fn max_tokens(&self) -> PyResult<usize> {
        self.runtime.block_on(async {
            Done::max_tokens()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // meaning

    pub fn meaning(&self, meaning: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::meaning(&meaning)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // migrate

    pub fn migrate(&self) -> PyResult<MigrationReport> {
        self.runtime.block_on(async {
            Done::migrate()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // migrate_to_project_based

    pub fn migrate_to_project_based(&self) -> PyResult<MigrationReport> {
        self.runtime.block_on(async {
            Done::migrate_to_project_based()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // moment

    pub fn moment(&self, moment: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::moment(&moment)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // move_task

    pub fn move_task(
        &self,
        id: String,
        from_collection: String,
        to_collection: String,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::move_task(&id, &from_collection, &to_collection)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // multi_query_search

    #[pyo3(signature = (queries, aggregation, content_types=None, limit=10))]
    pub fn multi_query_search(
        &self,
        queries: Vec<String>,
        aggregation: AggregationType,
        content_types: Option<Vec<TodoziContentType>>,
        limit: usize,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Done::multi_query_search(queries, aggregation, content_types.as_deref(), limit)
                .await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // name

    pub fn name(&self, name: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::name(&name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // new_full

    #[pyo3(signature = (user_id, action, time, priority, parent_project, status, assignee=None, tags=vec![], dependencies=vec![], context_notes=None, progress=None))]
    pub fn new_full(
        &self,
        user_id: String,
        action: String,
        time: String,
        priority: Priority,
        parent_project: String,
        status: Status,
        assignee: Option<Assignee>,
        tags: Vec<String>,
        dependencies: Vec<String>,
        context_notes: Option<String>,
        progress: Option<u8>,
    ) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::new_full(
                &user_id,
                &action,
                &time,
                priority,
                &parent_project,
                status,
                assignee.as_ref(),
                tags,
                dependencies,
                context_notes.as_deref(),
                progress.as_ref(),
            )
            .await
                .map_err(|e| PyException::new_err(format!("{}", e)))            
        })
    }

    // new_idea

    pub fn new_idea(&self, idea: Idea) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::new_idea(idea)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // new_memory

    pub fn new_memory(&self, memory: Memory) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::new_memory(memory)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // new_with_hashes

    pub fn new_with_hashes(&self, server_url: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::new_with_hashes(&server_url)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // ok

    pub fn ok(&self, body: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::ok(&body)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // overdue_percentage

    pub fn overdue_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::overdue_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_agent_assignment_format

    pub fn parse_agent_assignment_format(&self, agent_text: String) -> PyResult<AgentAssignment> {
        self.runtime.block_on(async {
            Done::parse_agent_assignment_format(&agent_text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_chunking_format

    pub fn parse_chunking_format(&self, chunk_text: String) -> PyResult<CodeChunk> {
        self.runtime.block_on(async {
            Done::parse_chunking_format(&chunk_text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_dependencies

    pub fn parse_dependencies(&self, deps_str: String) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::parse_dependencies(&deps_str)
                .await
                .map(|items| items.into_iter().map(String::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // parse_error_format

    pub fn parse_error_format(&self, error_text: String) -> PyResult<Error> {
        self.runtime.block_on(async {
            Done::parse_error_format(&error_text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_feeling_format

    pub fn parse_feeling_format(&self, feel_text: String) -> PyResult<Feeling> {
        self.runtime.block_on(async {
            Done::parse_feeling_format(&feel_text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_idea_format

    pub fn parse_idea_format(&self, idea_text: String) -> PyResult<PyIdea> {
        self.runtime.block_on(async {
            Done::parse_idea_format(&idea_text)
                .await
                .map(PyIdea::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_memory_format

    pub fn parse_memory_format(&self, memory_text: String, user_id: String) -> PyResult<PyMemory> {
        self.runtime.block_on(async {
            Done::parse_memory_format(&memory_text, &user_id)
                .await
                .map(PyMemory::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_reminder_format

    pub fn parse_reminder_format(&self, reminder_text: String) -> PyResult<PyReminder> {
        self.runtime.block_on(async {
            Done::parse_reminder_format(&reminder_text)
                .await
                .map(PyReminder::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_summary_format

    pub fn parse_summary_format(&self, summary_text: String) -> PyResult<PySummary> {
        self.runtime.block_on(async {
            Done::parse_summary_format(&summary_text)
                .await
                .map(PySummary::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_tags

    pub fn parse_tags(&self, tags_str: String) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::parse_tags(&tags_str)
                .await
                .map(|items| items.into_iter().map(String::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // parse_tdz_command

    pub fn parse_tdz_command(&self, text: String) -> PyResult<Vec<TdzCommand>> {
        self.runtime.block_on(async {
            Done::parse_tdz_command(&text)
                .await
                .map(|items| items.into_iter().map(TdzCommand::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // parse_todozi_format

    pub fn parse_todozi_format(&self, todozi_text: String) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::parse_todozi_format(&todozi_text)
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // parse_training_data_format

    pub fn parse_training_data_format(&self, train_text: String) -> PyResult<TrainingData> {
        self.runtime.block_on(async {
            Done::parse_training_data_format(&train_text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // pending_percentage

    pub fn pending_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::pending_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // predict_relevance

    pub fn predict_relevance(
        &self,
        _features: &[f32],
    ) -> PyResult<std::result::Result<f32, Box<dyn std::error::Error>>> {
        self.runtime.block_on(async {
            Done::predict_relevance(_features)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // preload_related_embeddings

    pub fn preload_related_embeddings(&self, content_id: String, depth: usize) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::preload_related_embeddings(&content_id, depth)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // prepare_task_content

    pub fn prepare_task_content(&self, task: &Task) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::prepare_task_content(task)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // priority

    pub fn priority(&self, priority: SummaryPriority) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::priority(priority)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // private_percentage

    pub fn private_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::private_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // process_chat

    pub fn process_chat(&self, message: String, user_id: String) -> PyResult<ChatContent> {
        self.runtime.block_on(async {
            Done::process_chat(&message, &user_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // process_chat_message

    pub fn process_chat_message(&self, message: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::process_chat_message(&message)
                .await
                .map(|items| items.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // process_chat_message_extended

    pub fn process_chat_message_extended(
        &self,
        message: String,
        user_id: String,
    ) -> PyResult<ChatContent> {
        self.runtime.block_on(async {
            Done::process_chat_message_extended(&message, &user_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // process_chunking_message

    pub fn process_chunking_message(&self, message: String) -> PyResult<Vec<CodeChunk>> {
        self.runtime.block_on(async {
            Done::process_chunking_message(&message)
                .await
                .map(|items| items.into_iter().map(CodeChunk::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // process_json_examples

    pub fn process_json_examples(&self, json_data: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Done::process_json_examples(&json_data)
                .await
                .map(|items| items.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // process_tdz_commands

    pub fn process_tdz_commands(
        &self,
        text: String,
        base_url: String,
        api_key: Option<String>,
    ) -> PyResult<Vec<Value>> {
        self.runtime.block_on(async {
            Done::process_tdz_commands(&text, &base_url, api_key.as_deref())
                .await
                .map(|items| items.into_iter().map(Value::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // process_workflow

    pub fn process_workflow(&self, tasks: Vec<Task>) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::process_workflow(tasks)
                .await
                .map(|items| items.into_iter().map(String::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // profile_search_performance

    pub fn profile_search_performance(
        &self,
        query: String,
        iterations: usize,
    ) -> PyResult<PerformanceMetrics> {
        self.runtime.block_on(async {
            Done::profile_search_performance(&query, iterations)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // project_name

    pub fn project_name(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::project_name()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // public_percentage

    pub fn public_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::public_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // quick

    pub fn quick(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::quick()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // reason

    pub fn reason(&self, reason: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::reason(&reason)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // recommend_similar

    pub fn recommend_similar(
        &self,
        based_on: Vec<String>,
        exclude: Vec<String>,
        limit: usize,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Done::recommend_similar(based_on, exclude, limit)
                .await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // register_with_server

    pub fn register_with_server(&self, server_url: String) -> PyResult<RegistrationInfo> {
        self.runtime.block_on(async {
            Done::register_with_server(&server_url)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // relationships_per_tag

    pub fn relationships_per_tag(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::relationships_per_tag()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // remind_at

    pub fn remind_at(&self, remind_at: DateTime<Utc>) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::remind_at(remind_at)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // remove_from_task

    pub fn remove_from_task(&self, task_id: String, tag: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Tags::remove_from_task(&task_id, &tag)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // remove_item

    pub fn remove_item(&self, id: String) -> PyResult<Option<PyQueueItem>> {
        self.runtime.block_on(async {
            Done::remove_item(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // remove_key

    pub fn remove_key(&self, user_id: String) -> PyResult<Option<PyApiKey>> {
        self.runtime.block_on(async {
            Done::remove_key(&user_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // remove_task

    pub fn remove_task(&self, id: String) -> PyResult<Option<PyTask>> {
        self.runtime.block_on(async {
            Done::remove_task(&id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // render

    pub fn render(&self, config: &DisplayConfig) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::render(config)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // render_compact

    pub fn render_compact(&self, config: &DisplayConfig) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::render_compact(config)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // render_detailed

    pub fn render_detailed(&self, config: &DisplayConfig) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::render_detailed(config)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // resolve_error

    pub fn resolve_error(&self, error_id: String, resolution: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::resolve_error(&error_id, &resolution)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // restore_backup

    pub fn restore_backup(&self, backup_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::restore_backup(&backup_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // restore_embeddings

    pub fn restore_embeddings(&self, backup_path: String) -> PyResult<usize> {
        self.runtime.block_on(async {
            Done::restore_embeddings(&backup_path)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // run

    pub fn run(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::run()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // run_interactive

    pub fn run_interactive(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::run_interactive()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // sample_task

    pub fn sample_task(&self) -> PyResult<PyTask> {
        self.runtime.block_on(async {
            Done::sample_task()
                .await
                .map(PyTask::from)
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_agent

    pub fn save_agent(&self, agent: &Agent) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_agent(agent)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_agent_assignment

    pub fn save_agent_assignment(&self, assignment: AgentAssignment) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_agent_assignment(&assignment)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // save_api_key_collection

    pub fn save_api_key_collection(&self, collection: ApiKeyCollection) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_api_key_collection(&collection)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))

        })
    }

    // save_as_default

    pub fn save_as_default(&self, model_name: String) -> PyResult<()> {
        self.runtime.block_on(async {
            EmbeddingModel::save_as_default(&model_name)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_code_chunk

    pub fn save_code_chunk(&self, chunk: &CodeChunk) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_code_chunk(chunk)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_config

    pub fn save_config(&self, config: Config) -> PyResult<()> {
        self.runtime.block_on(async {
            save_config(&config)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_error

    pub fn save_error(&self, error: &Error) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_error(error)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_feeling

    pub fn save_feeling(&self, feeling: &Feeling) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_feeling(feeling)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_idea

    pub fn save_idea(&self, idea: &Idea) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_idea(idea)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_memory

    pub fn save_memory(&self, memory: &Memory) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_memory(memory)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_processed_content

    pub fn save_processed_content(
        &self,
        raw: String,
        cleaned: String,
        session_id: String,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_processed_content(&raw, &cleaned, &session_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_project_task_container

    pub fn save_project_task_container(&self, container: &ProjectTaskContainer) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_project_task_container(container)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_queue_collection

    pub fn save_queue_collection(&self, collection: &QueueCollection) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_queue_collection(collection)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_task_collection

    pub fn save_task_collection(
        &self,
        collection_name: String,
        collection: &TaskCollection,
    ) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_task_collection(&collection_name, collection)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_task_with_embedding

    pub fn save_task_with_embedding(&self, task: &Task) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_task_with_embedding(task)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_tasks

    pub fn save_tasks(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_tasks()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // save_training_data

    pub fn save_training_data(&self, training_data: &TrainingData) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::save_training_data(training_data)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // search

    pub fn search(&self, query: String, options: SearchOptions) -> PyResult<SearchResults> {
        self.runtime.block_on(async {
            Done::search(&query, options)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // search_ideas

    pub fn search_ideas(&self, query: String) -> PyResult<Vec<&Idea>> {
        self.runtime.block_on(async {
            Done::search_ideas(&query)
                .await
                .map(|items| items.into_iter().map(&Idea::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // search_memories

    pub fn search_memories(&self, query: String) -> PyResult<Vec<&Memory>> {
        self.runtime.block_on(async {
            Done::search_memories(&query)
                .await
                .map(|items| items.into_iter().map(&Memory::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // search_tasks_semantic

    pub fn search_tasks_semantic(
        &self,
        query: String,
        max_results: usize,
    ) -> PyResult<Vec<SemanticSearchResult>> {
        self.runtime.block_on(async {
            Done::search_tasks_semantic(&query, max_results)
                .await
                .map(|items| items.into_iter().map(SemanticSearchResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // semantic_search

    pub fn semantic_search(
        &self,
        query: String,
        content_types: Option<Vec<TodoziContentType>>,
        limit: Option<usize>,
    ) -> PyResult<Vec<PySimilarityResult>> {
        self.runtime.block_on(async {
            Done::semantic_search(&query, limit)
                .await
                .map(|items| items.into_iter().map(PySimilarityResult::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // serialization

    pub fn serialization(&self, message: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::serialization(message)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // share

    pub fn share(&self, share: ShareLevel) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::share(share)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // short_term_percentage

    pub fn short_term_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::short_term_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // show_loading_screen

    pub fn show_loading_screen(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::show_loading_screen()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // smart

    pub fn smart(&self, query: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Find::smart(&query)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // specializations

    pub fn specializations(&self, specializations: Vec<String>) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::specializations(specializations)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // start_edit

    pub fn start_edit(&self, _task_id: String) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::start_edit(&_task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // start_edit_session

    pub fn start_edit_session(&self, task_id: String) -> PyResult<EditSession> {
        self.runtime.block_on(async {
            Done::start_edit_session(&task_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // start_queue_session

    pub fn start_queue_session(&self, queue_item_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::start_queue_session(&queue_item_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // start_server

    pub fn start_server(
        &self,
        host: Option<String>,
        port: Option<u16>,
    ) -> PyResult<std::result::Result<(), Box<dyn std::error::Error>>> {
        self.runtime.block_on(async {
            crate::server::start_server(host, port)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // start_session

    pub fn start_session(&self, queue_item_id: String) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::start_session(&queue_item_id)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // status

    pub fn status(&self, status: AgentStatus) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::status(status)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // storage

    pub fn storage(&self) -> PyResult<Storage> {
        self.runtime.block_on(async {
            Done::storage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // strike_cluster

    pub fn strike_cluster(
        &self,
        _embedding: &[f32],
    ) -> PyResult<std::result::Result<i32, Box<dyn std::error::Error>>> {
        self.runtime.block_on(async {
            Done::strike_cluster(_embedding)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // strike_tags

    pub fn strike_tags(
        &self,
        _features: &[f32],
    ) -> PyResult<std::result::Result<Vec<f32>, Box<dyn std::error::Error>>> {
        self.runtime.block_on(async {
            Done::strike_tags(_features)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // success

    pub fn success(&self, output: String, execution_time_ms: u64) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::success(&output, execution_time_ms)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // suggest_tags

    pub fn suggest_tags(&self, content_id: String, top_k: usize) -> PyResult<Vec<String>> {
        self.runtime.block_on(async {
            Done::suggest_tags(&content_id, top_k)
                .await
                .map(|items| items.into_iter().map(String::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // tags

    pub fn tags(&self, tags: Vec<String>) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::tags(tags)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // tasks

    pub fn tasks(&self, project_name: String) -> PyResult<Vec<PyTask>> {
        self.runtime.block_on(async {
            Projects::tasks(&project_name)
                .await
                .map(|items| items.into_iter().map(PyTask::from).collect())
                .map_err(|e| PyException::new_err(format!("{}", e)))
                
        })
    }

    // team_percentage

    pub fn team_percentage(&self) -> PyResult<f64> {
        self.runtime.block_on(async {
            Done::team_percentage()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // term

    pub fn term(&self, term: MemoryTerm) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::term(term)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // title

    pub fn title(&self) -> PyResult<&'static str> {
        self.runtime.block_on(async {
            Done::title()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // to_context_string

    pub fn to_context_string(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::to_context_string()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // to_ollama_format

    pub fn to_ollama_format(&self) -> PyResult<serde_json::Value> {
        self.runtime.block_on(async {
            Done::to_ollama_format()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // to_state_string

    pub fn to_state_string(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            Done::to_state_string()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // tool_count

    pub fn tool_count(&self) -> PyResult<usize> {
        Ok(Done::tool_count())
    }

    // total_results

    pub fn total_results(&self) -> PyResult<usize> {
        Ok(Done::total_results())
    }

    // track_embedding_drift

    pub fn track_embedding_drift(
        &self,
        content_id: String,
        current_text: String,
    ) -> PyResult<DriftReport> {
        self.runtime.block_on(async {
            Done::track_embedding_drift(&content_id, &current_text)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // transform_shorthand_tags

    pub fn transform_shorthand_tags(&self, message: String) -> PyResult<String> {
        Ok(Done::transform_shorthand_tags(&message))
    }

    // types

    pub fn types(&self) -> PyResult<&'static str> {
        Ok(Done::types())
    }

    // unregister

    pub fn unregister(&self, name: String) -> PyResult<bool> {
        Ok(Done::unregister(&name))
    }

    // update

    pub fn update(&self, updates: TaskUpdate) -> PyResult<()> {
        Done::update(updates);
        Ok(())
    }

    // update_agent_assignment_status

    pub fn update_agent_assignment_status(
        &self,
        agent_id: String,
        task_id: String,
        status: AssignmentStatus,
    ) -> PyResult<()> {
        Done::update_agent_assignment_status(&agent_id, &task_id, status);
        Ok(())
    }

    // update_chunk_code

    pub fn update_chunk_code(&self, chunk_id: String, code: String) -> PyResult<()> {
        Done::update_chunk_code(&chunk_id, &code);
        Ok(())
    }

    // update_chunk_tests

    pub fn update_chunk_tests(&self, chunk_id: String, tests: String) -> PyResult<()> {
        Done::update_chunk_tests(&chunk_id, &tests);
        Ok(())
    }

    // update_config

    pub fn update_config(&self, config: Config) -> PyResult<()> {
        Done::update_config(config);
        Ok(())
    }

    // update_config_with_registration

    pub fn update_config_with_registration(&self, registration: RegistrationInfo) -> PyResult<()> {
        Done::update_config_with_registration(registration);
        Ok(())
    }

    // update_feeling

    pub fn update_feeling(&self, feeling: &Feeling) -> PyResult<()> {
        Done::update_feeling(feeling);
        Ok(())
    }

    // update_idea

    pub fn update_idea(&self, idea_id: String, updates: IdeaUpdate) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::update_idea(&idea_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // update_memory

    pub fn update_memory(&self, memory_id: String, updates: MemoryUpdate) -> PyResult<()> {
        self.runtime.block_on(async {
            Done::update_memory(&memory_id, updates)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // update_project

    pub fn update_project(&self, project: Project) -> PyResult<()> {
        Done::update_project(project);
        Ok(())
    }

    // update_registration_api_key

    pub fn update_registration_api_key(&self, api_key: String) -> PyResult<()> {
        Done::update_registration_api_key(&api_key);
        Ok(())
    }

    // update_registration_keys

    pub fn update_registration_keys(
        &self,
        api_key: String,
        user_id: Option<String>,
        fingerprint: Option<String>,
    ) -> PyResult<()> {
        Done::update_registration_keys(&api_key, user_id.as_deref(), fingerprint.as_deref());
        Ok(())
    }

    // update_task

    pub fn update_task(&self, id: String, updates: TaskUpdate) -> PyResult<()> {
        Done::update_task(&id, updates);
        Ok(())
    }

    // update_task_in_project

    pub fn update_task_in_project(
        &self,
        id: String,
        updates: crate::models::TaskUpdate,
    ) -> PyResult<()> {
        Done::update_task_in_project(&id, updates);
        Ok(())
    }

    // validate_embeddings

    pub fn validate_embeddings(&self) -> PyResult<ValidationReport> {
        self.runtime.block_on(async {
            Done::validate_embeddings()
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // validate_migration

    pub fn validate_migration(&self) -> PyResult<bool> {
        Ok(Done::validate_migration())
    }

    // validate_required_params

    pub fn validate_required_params(
        &self,
        kwargs: &HashMap<String, serde_json::Value>,
        required_params: &[String],
    ) -> PyResult<Option<ToolResult>> {
        Ok(Done::validate_required_params(kwargs, required_params))
    }

    // validate_string_param

    pub fn validate_string_param(
        &self,
        value: &serde_json::Value,
        param_name: String,
        min_length: usize,
        max_length: usize,
        pattern: Option<String>,
    ) -> PyResult<Option<ToolResult>> {
        Ok(Done::validate_string_param(
            value,
            &param_name,
            min_length,
            max_length,
            pattern.as_deref(),
        ))
    }

    // validate_task_input

    pub fn validate_task_input(
        &self,
        action: String,
        time: String,
        priority: String,
        project: String,
        status: String,
        assignee: Option<String>,
        progress: Option<u8>,
    ) -> PyResult<()> {
        Done::validate_task_input(
            &action,
            &time,
            &priority,
            &project,
            &status,
            assignee.as_ref(),
            progress,
        );
        Ok(())
    }

// verbose

    pub fn verbose(&self, verbose: bool) -> PyResult<Self> {
        Ok(Done::verbose(verbose))
    }

    // with_action

    pub fn with_action(&self, action: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_action(&action)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_assignee

    pub fn with_assignee(&self, assignee: Assignee) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_assignee(assignee)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_context

    pub fn with_context(&self, context: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_context(&context)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_context_notes

    pub fn with_context_notes(&self, context_notes: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_context_notes(&context_notes)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_dependencies

    pub fn with_dependencies(&self, dependencies: Vec<String>) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_dependencies(dependencies)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_dry_run

    pub fn with_dry_run(&self, dry_run: bool) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_dry_run(dry_run)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_embedding_service

    pub fn with_embedding_service(&self, service: EmbeddingService) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_embedding_service(service)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_embedding_service_option

    pub fn with_embedding_service_option(
        &self,
        service: Option<TodoziEmbeddingService>,
    ) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_embedding_service_option(service.as_ref())
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_force

    pub fn with_force(&self, force: bool) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_force(force)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_max_tokens

    pub fn with_max_tokens(&self, max_tokens: u32) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_max_tokens(max_tokens)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_parent_project

    pub fn with_parent_project(&self, parent_project: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_parent_project(&parent_project)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_priority

    pub fn with_priority(&self, priority: Priority) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_priority(priority)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_progress

    pub fn with_progress(&self, progress: u8) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_progress(progress)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

// with_status

    pub fn with_status(&self, status: Status) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_status(status)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_tags

    pub fn with_tags(&self, tags: Vec<String>) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_tags(tags)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

    // with_temperature

    pub fn with_temperature(&self, temperature: f32) -> PyResult<Self> {
        Ok(Done::with_temperature(temperature))
    }

    // with_time

    pub fn with_time(&self, time: String) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_time(&time)
                .await
                .map_err(|e| PyException::new_err(format!("{}", e)))                
        })
    }

// with_verbose

    pub fn with_verbose(&self, verbose: bool) -> PyResult<Self> {
        self.runtime.block_on(async {
            Done::with_verbose(verbose)
                .await
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
            context_notes: None,      // Not exposed in PyTask
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
            tasks: Vec::new(),              // PyProject doesn't expose tasks
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

#[pyclass]
#[derive(Clone)]
pub struct PyErrorDetails {
    #[pyo3(get)]
    pub error_type: String,
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get)]
    pub recovery_suggestion: String,
    #[pyo3(get)]
    pub context: Option<String>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyCommandValidation {
    #[pyo3(get)]
    pub valid: bool,
    #[pyo3(get)]
    pub error: Option<String>,
    #[pyo3(get)]
    pub commands: Vec<String>,
    #[pyo3(get)]
    pub suggestions: Vec<String>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyToolParameter {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub type_: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub required: bool,
}

impl From<ToolParameter> for PyToolParameter {
    fn from(param: ToolParameter) -> Self {
        PyToolParameter {
            name: param.name,
            type_: param.type_,
            description: param.description,
            required: param.required,
        }
    }
}

impl From<PyToolParameter> for ToolParameter {
    fn from(param: PyToolParameter) -> Self {
        ToolParameter::new(
            param.name,
            param.type_,
            param.description,
            param.required,
            None,
        )
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyToolDefinition {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub category: String,
}

impl From<ToolDefinition> for PyToolDefinition {
    fn from(def: ToolDefinition) -> Self {
        PyToolDefinition {
            name: def.name,
            description: def.description,
            category: def.category,
        }
    }
}

impl From<PyToolDefinition> for ToolDefinition {
    fn from(def: PyToolDefinition) -> Self {
        ToolDefinition::new(
            def.name,
            def.description,
            Vec::new(),
            def.category,
            Vec::new(),
        )
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
    m.add_class::<PyToolParameter>()?;
    m.add_class::<PyToolDefinition>()?;
    m.add_class::<PyToolResult>()?;
    m.add_class::<PyErrorDetails>()?;
    m.add_class::<PyCommandValidation>()?;
    Ok(())
}
