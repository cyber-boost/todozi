#![cfg(feature = "ruby")]
use magnus::{prelude::*, Error as MagnusError, Ruby, exception::runtime_error};
use crate::{Done, Tdz, Actions, Projects, Memories, Ideas, Queue, Find, Emb, Stats, Easy, Tags};
use crate::models::{Task, Memory, Idea, QueueItem, Project, ApiKey, Tag, Summary, Reminder};
use tokio::runtime::Runtime;
use std::sync::OnceLock;

// Global runtime for Ruby extension
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> Result<&'static Runtime, MagnusError> {
    RUNTIME.get_or_try_init(|| {
        Runtime::new().map_err(|e| MagnusError::new(
            runtime_error(),
            format!("Failed to create runtime: {}", e)
        ))
    }).map_err(|e| MagnusError::new(
        runtime_error(),
        format!("Runtime error: {}", e)
    ))
}

// Ruby class for Todozi
#[magnus::wrap(class = "Todozi")]
pub struct Todozi {
    // Instance state if needed
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), MagnusError> {
    let class = ruby.define_class("Todozi", ruby.class_object())?;
    
    Ok(())
}

impl Todozi {
    
    // activate_api_key
    pub fn activate_api_key(&self, user_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::activate_api_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // activate_key
    pub fn activate_key(&self, user_id: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::activate_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // activate_reminder
    pub fn activate_reminder(&self, reminder_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::activate_reminder(reminder_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // active
    pub fn active(&self) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::active().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // active_percentage
    pub fn active_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::active_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add
    pub fn add(&self, task_name: &str, description: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add(task_name, description).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_checklist_item
    pub fn add_checklist_item(&self, item: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_checklist_item(item).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_chunk
    pub fn add_chunk(&self, chunk_id: &str, level: &str, deps: Vec<String>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_chunk(chunk_id, level, deps).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_completed_module
    pub fn add_completed_module(&self, module: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_completed_module(module).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_dependency
    pub fn add_dependency(&self, dep: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_dependency(dep).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_error_pattern
    pub fn add_error_pattern(&self, pattern: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_error_pattern(pattern).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_function_signature
    pub fn add_function_signature(&self, name: &str, signature: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_function_signature(name, signature).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_import
    pub fn add_import(&self, import_stmt: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_import(import_stmt).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_item
    pub fn add_item(&self, content: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_item(content).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_key
    pub fn add_key(&self, key: JsApiKey) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_key(key).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_pending_module
    pub fn add_pending_module(&self, module: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_pending_module(module).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_processed_action
    pub fn add_processed_action(&self, action_type: &str, description: &str, success: bool, #[magnus(default = None)] result: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_processed_action(action_type, description, success, result.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_queue_item
    pub fn add_queue_item(&self, item: QueueItem) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_queue_item(item).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_recent
    pub fn add_recent(&self, description: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_recent(description).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_recent_action
    pub fn add_recent_action(&self, action: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_recent_action(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_tag_relationship
    pub fn add_tag_relationship(&self, tag_id: &str, related_tag_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_tag_relationship(tag_id, related_tag_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_tag_to_task
    pub fn add_tag_to_task(&self, task_id: &str, tag: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_tag_to_task(task_id, tag).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_task
    pub fn add_task(&self, task: Task) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_task(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_task_emb
    pub fn add_task_emb(&self, task: JsTask) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_task_emb(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_task_to_project
    pub fn add_task_to_project(&self, task: Task) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::add_task_to_project(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // add_to_task
    pub fn add_to_task(&self, task_id: &str, tag: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::add_to_task(task_id, tag).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // advanced_search
    pub fn advanced_search(&self, query: &str) -> Result<Vec<Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // advanced_search_with_filters
    pub fn advanced_search_with_filters(&self, query: &str, #[magnus(default = None)] data_types: Option<Vec<String>>, #[magnus(default = None)] limit: Option<u32>) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::advanced_search_with_filters(query, data_types.as_deref(), limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ai
    pub fn ai(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::ai(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ai_find
    pub fn ai_find(&self, query: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::ai_find(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ai_search
    pub fn ai_search(&self, query: &str) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::ai_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ai_task
    pub fn ai_task(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::ai_task(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ai_tasks
    pub fn ai_tasks(&self, query: &str) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::ai_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // all
    pub fn all(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::all().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // all_tasks
    pub fn all_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::all_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // analyze_code_quality
    pub fn analyze_code_quality(&self, _features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::analyze_code_quality(_features).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // api
    pub fn api(&self, message: impl Into<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::api(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // api_key
    pub fn api_key(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::api_key().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // archive_project
    pub fn archive_project(&self, name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::archive_project(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // as_str
    pub fn as_str(&self) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::as_str().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // assign_task_to_agent
    pub fn assign_task_to_agent(&self, task_id: &str, agent_id: &str, project_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::assign_task_to_agent(task_id, agent_id, project_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // auto_categorize_content
    pub fn auto_categorize_content(&self, text: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::auto_categorize_content(text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // auto_label_clusters
    pub fn auto_label_clusters(&self, clusters: Vec<ClusteringResult>) -> Result<Vec<LabeledCluster>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::auto_label_clusters(clusters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // backlog
    pub fn backlog(&self) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Queue::backlog().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // backup_embeddings
    pub fn backup_embeddings(&self, #[magnus(default = None)] backup_path: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::backup_embeddings(backup_path.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // batch_embed_content
    pub fn batch_embed_content(&self, content_list: Vec<String>) -> Result<Vec<Vec<f64>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::batch_embed_content(content_list).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // begin
    pub fn begin(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Actions::begin(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // breakthrough
    pub fn breakthrough(&self, idea: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::breakthrough(idea).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // breakthrough_idea
    pub fn breakthrough_idea(&self, idea: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::breakthrough_idea(idea).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // breakthrough_percentage
    pub fn breakthrough_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::breakthrough_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // build_similarity_graph
    pub fn build_similarity_graph(&self, threshold: f32) -> Result<SimilarityGraph, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::build_similarity_graph(threshold).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // bulk_create_tags
    pub fn bulk_create_tags(&self, tag_names: Vec<String>, #[magnus(default = None)] category: Option<String>) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::bulk_create_tags(tag_names, category.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // calculate_diversity
    pub fn calculate_diversity(&self, content_ids: Vec<String>) -> Result<f32, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::calculate_diversity(content_ids).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // capabilities
    pub fn capabilities(&self, capabilities: Vec<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::capabilities(capabilities).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // category
    pub fn category(&self, category: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::category(category).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // chat
    pub fn chat(&self, message: &str) -> Result<ChatContent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::chat(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // check_api_key_auth
    pub fn check_api_key_auth(&self, public_key: &str, #[magnus(default = None)] private_key: Option<String>) -> Result<JsApiKeyAuth, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::check_api_key_auth(public_key, private_key.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // check_folder_structure
    pub fn check_folder_structure(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::check_folder_structure().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // check_migration_status
    pub fn check_migration_status(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::check_migration_status().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cleanup_expired
    pub fn cleanup_expired(&self) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cleanup_expired().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cleanup_legacy
    pub fn cleanup_legacy(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cleanup_legacy().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // clear_registration
    pub fn clear_registration(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::clear_registration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_add_task
    pub fn cli_add_task(&self, content: &str, #[magnus(default = None)] priority: Option<String>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_add_task(content, priority.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_chat
    pub fn cli_chat(&self, message: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_chat(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_clear_registration
    pub fn cli_clear_registration(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_clear_registration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_complete_task
    pub fn cli_complete_task(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_complete_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_create_backup
    pub fn cli_create_backup(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_create_backup().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_create_idea
    pub fn cli_create_idea(&self, content: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_create_idea(content).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_create_memory
    pub fn cli_create_memory(&self, moment: &str, meaning: &str, reason: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_create_memory(moment, meaning, reason).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_delete_task
    pub fn cli_delete_task(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_delete_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_fix_consistency
    pub fn cli_fix_consistency(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_fix_consistency().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_get_registration_status
    pub fn cli_get_registration_status(&self) -> Result<Option<JsRegistrationInfo>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_get_registration_status().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_list_backups
    pub fn cli_list_backups(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_list_backups().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_list_tasks
    pub fn cli_list_tasks(&self) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_list_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_register_with_server
    pub fn cli_register_with_server(&self, server_url: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_register_with_server(server_url).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_restore_backup
    pub fn cli_restore_backup(&self, backup_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_restore_backup(backup_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_search_tasks
    pub fn cli_search_tasks(&self, query: &str) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_search_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_show_task
    pub fn cli_show_task(&self, task_id: &str) -> Result<JsTask, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_show_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cli_update_task
    pub fn cli_update_task(&self, task_id: &str, #[magnus(default = None)] action: Option<String>, #[magnus(default = None)] priority: Option<String>, #[magnus(default = None)] status: Option<String>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cli_update_task(task_id, action.as_deref(), priority.as_deref(), status.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cluster
    pub fn cluster(&self) -> Result<Vec<ClusteringResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cluster().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cluster_content
    pub fn cluster_content(&self) -> Result<Vec<ClusteringResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cluster_content().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // cluster_similar_tasks
    pub fn cluster_similar_tasks(&self) -> Result<Vec<JsClusteringResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::cluster_similar_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // collab
    pub fn collab(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::collab(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // collab_task
    pub fn collab_task(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::collab_task(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // color
    pub fn color(&self, color: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::color(color).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // compare_models
    pub fn compare_models(&self, text: &str, model_aliases: Vec<String>) -> Result<ModelComparisonResult, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::compare_models(text, model_aliases).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // complete
    pub fn complete(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::complete(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // complete_agent_assignment
    pub fn complete_agent_assignment(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::complete_agent_assignment(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // complete_task
    pub fn complete_task(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::complete_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // complete_task_in_project
    pub fn complete_task_in_project(&self, id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::complete_task_in_project(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // completion_rate
    pub fn completion_rate(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::completion_rate().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // config
    pub fn config(&self) -> Result<&Config, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::config().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // configure_display
    pub fn configure_display(&self, config_json: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::configure_display(config_json).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // configure_server
    pub fn configure_server(&self, #[magnus(default = None)] host: Option<String>, #[magnus(default = None)] port: Option<u32>, #[magnus(default = None)] max_connections: Option<u32>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::configure_server(host.as_deref(), port.as_deref(), max_connections.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // content
    pub fn content(&self, content: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::content(content).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // context
    pub fn context(&self, context: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::context(context).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // craft_embedding
    pub fn craft_embedding(&self, _features: &str) -> Result<std::result::Result<Vec<f32>, Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::craft_embedding(_features).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create
    pub fn create(&self, name: &str, #[magnus(default = None)] description: Option<&str>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create(name, description.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_advanced_todozi_tools
    pub fn create_advanced_todozi_tools(&self, todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_advanced_todozi_tools(todozi).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_agent
    pub fn create_agent(&self, agent: Agent) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_agent(agent).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_api_key
    pub fn create_api_key(&self) -> Result<JsApiKey, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_api_key().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_api_key_with_user_id
    pub fn create_api_key_with_user_id(&self, user_id: &str) -> Result<JsApiKey, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_api_key_with_user_id(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_architect_agent
    pub fn create_architect_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_architect_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_backup
    pub fn create_backup(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_backup().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_coder
    pub fn create_coder(&self) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_coder().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_comrad_agent
    pub fn create_comrad_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_comrad_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_custom_agent
    pub fn create_custom_agent(&self, id: &str, name: &str, description: &str, capabilities: Vec<String>, specializations: Vec<String>, category: &str, #[magnus(default = None)] author: Option<String>) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_custom_agent(id, name, description, capabilities, specializations, category, author.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_default_agents
    pub fn create_default_agents(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_default_agents().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_designer_agent
    pub fn create_designer_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_designer_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_detective_agent
    pub fn create_detective_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_detective_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_devops_agent
    pub fn create_devops_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_devops_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_embedding_service
    pub fn create_embedding_service(&self) -> Result<TodoziEmbeddingService, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_embedding_service().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_embedding_version
    pub fn create_embedding_version(&self, content_id: &str, version_label: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_embedding_version(content_id, version_label).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_error
    pub fn create_error(&self, error: Error) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_error(error).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_error_result
    pub fn create_error_result(&self, error_msg: &str, execution_time_ms: u64, error_type: ErrorType, #[magnus(default = None)] metadata: Option<HashMap<Strin>, serde_json: :Value>>) -> Result<ToolResult, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_error_result(error_msg, execution_time_ms, error_type, metadata.as_deref(), serde_json).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_filters
    pub fn create_filters(&self) -> Result<TaskFilters, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_filters().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_finisher_agent
    pub fn create_finisher_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_finisher_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_framer_agent
    pub fn create_framer_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_framer_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_friend_agent
    pub fn create_friend_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_friend_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_grok_level_todozi_tools
    pub fn create_grok_level_todozi_tools(&self, todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_grok_level_todozi_tools(todozi).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_hoarder_agent
    pub fn create_hoarder_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_hoarder_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_idea
    pub fn create_idea(&self, idea: Idea) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_idea(idea).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_investigator_agent
    pub fn create_investigator_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_investigator_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_mason_agent
    pub fn create_mason_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_mason_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_memory
    pub fn create_memory(&self, memory: Memory) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_memory(memory).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_nerd_agent
    pub fn create_nerd_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_nerd_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_nun_agent
    pub fn create_nun_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_nun_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_overlord_agent
    pub fn create_overlord_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_overlord_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_party_agent
    pub fn create_party_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_party_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_planner_agent
    pub fn create_planner_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_planner_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_project
    pub fn create_project(&self, name: &str, #[magnus(default = None)] description: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Projects::create_project(name, description.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_recycler_agent
    pub fn create_recycler_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_recycler_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_reminder
    pub fn create_reminder(&self, reminder: Reminder) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_reminder(reminder).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_search_options
    pub fn create_search_options(&self, #[magnus(default = None)] data_types: Option<Vec<String>>, #[magnus(default = None)] limit: Option<u32>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_search_options(data_types.as_deref(), limit.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_skeleton_agent
    pub fn create_skeleton_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_skeleton_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_snitch_agent
    pub fn create_snitch_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_snitch_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_storage
    pub fn create_storage(&self) -> Result<Storage, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_storage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_success_result
    pub fn create_success_result(&self, output: &str, execution_time_ms: u64, #[magnus(default = None)] metadata: Option<HashMap<Strin>, serde_json: :Value>>) -> Result<ToolResult, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_success_result(output, execution_time_ms, metadata.as_deref(), serde_json).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_summary
    pub fn create_summary(&self, summary: Summary) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_summary(summary).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tag
    pub fn create_tag(&self, tag: Tag) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::create_tag(tag).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_task
    pub fn create_task(&self, action: &str, #[magnus(default = None)] priority: Option<Priority>, #[magnus(default = None)] project: Option<&str>, #[magnus(default = None)] time: Option<&str>, #[magnus(default = None)] context: Option<&str>) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_task(action, priority.as_deref(), project.as_deref(), time.as_deref(), context.as_deref()).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_task_filters
    pub fn create_task_filters(&self, #[magnus(default = None)] project: Option<String>, #[magnus(default = None)] status: Option<String>, #[magnus(default = None)] priority: Option<String>, #[magnus(default = None)] assignee: Option<String>, #[magnus(default = None)] tags: Option<String>, #[magnus(default = None)] search: Option<String>) -> Result<TaskFilters, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_task_filters(project.as_deref(), status.as_deref(), priority.as_deref(), assignee.as_deref(), tags.as_deref(), search.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tdz_content_processor_tool
    pub fn create_tdz_content_processor_tool(&self, state: SharedTodoziState) -> Result<Box<dyn Tool + Send + Sync>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tdz_content_processor_tool(state).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tdz_tool
    pub fn create_tdz_tool(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tdz_tool().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tester_agent
    pub fn create_tester_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tester_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_todozi_tools
    pub fn create_todozi_tools(&self, todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_todozi_tools(todozi).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_todozi_tools_with_embedding
    pub fn create_todozi_tools_with_embedding(&self, todozi: SharedTodozi, #[magnus(default = None)] embedding_service: Option<TodoziEmbeddingService>) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_todozi_tools_with_embedding(todozi, embedding_service.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tool_definition
    pub fn create_tool_definition(&self, name: &str, description: &str, category: &str, parameters: Vec<JsToolParameter>) -> Result<JsToolDefinition, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tool_definition(name, description, category, parameters).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tool_definition_with_locks
    pub fn create_tool_definition_with_locks(&self, name: &str, description: &str, category: &str, parameters: Vec<ToolParameter>, resource_locks: Vec<ResourceLock>) -> Result<ToolDefinition, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tool_definition_with_locks(name, description, category, parameters, resource_locks).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tool_parameter
    pub fn create_tool_parameter(&self, name: &str, type_: &str, description: &str, required: bool) -> Result<JsToolParameter, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tool_parameter(name, type_, description, required).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tool_parameter_with_default
    pub fn create_tool_parameter_with_default(&self, name: &str, type_: &str, description: &str, required: bool, default: &str) -> Result<JsToolParameter, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tool_parameter_with_default(name, type_, description, required, default).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tui_app
    pub fn create_tui_app(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tui_app().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_tuner_agent
    pub fn create_tuner_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_tuner_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_update
    pub fn create_update(&self) -> Result<TaskUpdate, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_update().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // create_writer_agent
    pub fn create_writer_agent(&self) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::create_writer_agent().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // critical_percentage
    pub fn critical_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::critical_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // deactivate_api_key
    pub fn deactivate_api_key(&self, user_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::deactivate_api_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // deactivate_key
    pub fn deactivate_key(&self, user_id: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::deactivate_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // deep
    pub fn deep(&self, query: &str) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::deep(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // deep_search
    pub fn deep_search(&self, query: &str) -> Result<Vec<JsSimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::deep_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // default_filters
    pub fn default_filters(&self) -> Result<TaskFilters, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::default_filters().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // default_update
    pub fn default_update(&self) -> Result<TaskUpdate, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::default_update().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete
    pub fn delete(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_agent
    pub fn delete_agent(&self, agent_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_agent(agent_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_agent_assignment
    pub fn delete_agent_assignment(&self, agent_id: &str, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_agent_assignment(agent_id, task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_code_chunk
    pub fn delete_code_chunk(&self, chunk_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_code_chunk(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_error
    pub fn delete_error(&self, error_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_error(error_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_feeling
    pub fn delete_feeling(&self, id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_feeling(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_idea
    pub fn delete_idea(&self, idea_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_idea(idea_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_memory
    pub fn delete_memory(&self, memory_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_memory(memory_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_project
    pub fn delete_project(&self, project_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_project(project_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_project_task_container
    pub fn delete_project_task_container(&self, project_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_project_task_container(project_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_reminder
    pub fn delete_reminder(&self, reminder_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_reminder(reminder_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_summary
    pub fn delete_summary(&self, summary_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_summary(summary_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_tag
    pub fn delete_tag(&self, tag_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_tag(tag_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_task
    pub fn delete_task(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_task_from_project
    pub fn delete_task_from_project(&self, id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_task_from_project(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // delete_training_data
    pub fn delete_training_data(&self, training_data_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::delete_training_data(training_data_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // description
    pub fn description(&self, description: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::description(description).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // detailed
    pub fn detailed(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Stats::detailed().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // detailed_stats
    pub fn detailed_stats(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::detailed_stats().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // display_task
    pub fn display_task(&self, task_id: &str) -> Result<TaskDisplay, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::display_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // display_tasks
    pub fn display_tasks(&self, task_ids: Vec<String>) -> Result<TaskListDisplay, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::display_tasks(task_ids).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // do_it
    pub fn do_it(&self, what: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Easy::do_it(what).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done
    pub fn done(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::done(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_api_key
    pub fn done_api_key(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_api_key().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_create_embedding_service
    pub fn done_create_embedding_service(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_create_embedding_service().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_create_storage
    pub fn done_create_storage(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_create_storage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_default_filters
    pub fn done_default_filters(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_default_filters().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_default_update
    pub fn done_default_update(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_default_update().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_embedding_config
    pub fn done_embedding_config(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_embedding_config().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_embedding_service
    pub fn done_embedding_service(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_embedding_service().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_init
    pub fn done_init(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_init().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_process_chat
    pub fn done_process_chat(&self, message: &str, user_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_process_chat(message, user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_sample_task
    pub fn done_sample_task(&self) -> Result<JsTask, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_sample_task().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_storage
    pub fn done_storage(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_storage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // done_types
    pub fn done_types(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::done_types().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // dry_run
    pub fn dry_run(&self, dry_run: bool) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::dry_run(dry_run).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // easy_done
    pub fn easy_done(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::easy_done(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // easy_find
    pub fn easy_find(&self, what: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::easy_find(what).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // easy_idea
    pub fn easy_idea(&self, what: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::easy_idea(what).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // easy_remember
    pub fn easy_remember(&self, what: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::easy_remember(what).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embed
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embed(text).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embed_idea
    pub fn embed_idea(&self, idea: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embed_idea(idea).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embed_memory
    pub fn embed_memory(&self, memory: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embed_memory(memory).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embed_stats
    pub fn embed_stats(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embed_stats().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embed_tag
    pub fn embed_tag(&self, tag: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embed_tag(tag).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embed_task
    pub fn embed_task(&self, task_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Emb::embed_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embedding_config
    pub fn embedding_config(&self) -> Result<TodoziEmbeddingConfig, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embedding_config().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // embedding_service
    pub fn embedding_service(&self) -> Result<TodoziEmbeddingService, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::embedding_service().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // encode
    pub fn encode(&self, texts: &str) -> Result<Vec<Vec<f32>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::encode(texts).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // end_queue_session
    pub fn end_queue_session(&self, session_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::end_queue_session(session_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // end_session
    pub fn end_session(&self, session_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::end_session(session_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ensure_folder_structure
    pub fn ensure_folder_structure(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::ensure_folder_structure().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ensure_todozi_initialized
    pub fn ensure_todozi_initialized(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::ensure_todozi_initialized().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // error
    pub fn error(&self, error: &str, execution_time_ms: u64) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::error(error, execution_time_ms).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // example
    pub fn example(&self) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::example().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // example_usage
    pub fn example_usage(&self) -> Result<std::result::Result<(), Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::example_usage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // execute_task
    pub fn execute_task(&self, storage: &str, task: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::execute_task(storage, task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // execute_tdz_command
    pub fn execute_tdz_command(&self, command: &str, base_url: &str, #[magnus(default = None)] api_key: Option<&str>) -> Result<Value, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::execute_tdz_command(command, base_url, api_key.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // execute_todozi_tool_delegated
    pub fn execute_todozi_tool_delegated(&self, params: &str) -> Result<ExecutorResult<ExecutionResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::execute_todozi_tool_delegated(params).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // execute_tool
    pub fn execute_tool(&self, tool_name: &str, kwargs: HashMap<String, serde_json: :Value>) -> Result<ToolResult, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::execute_tool(tool_name, kwargs, serde_json).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // explain_search_result
    pub fn explain_search_result(&self, query: &str, result: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::explain_search_result(query, result).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // export_diagnostics
    pub fn export_diagnostics(&self) -> Result<DiagnosticReport, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::export_diagnostics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // export_embedded_tasks_hlx
    pub fn export_embedded_tasks_hlx(&self, output_path: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::export_embedded_tasks_hlx(output_path).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // export_for_fine_tuning
    pub fn export_for_fine_tuning(&self, output_path: &str) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::export_for_fine_tuning(output_path).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // export_to_format
    pub fn export_to_format(&self, format: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::export_to_format(format).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // extract_content
    pub fn extract_content(&self, #[magnus(default = None)] content: Option<String>, #[magnus(default = None)] file_path: Option<String>, output_format: &str, human: bool) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::extract_content(content.as_deref(), file_path.as_deref(), output_format, human).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // extract_content_from_text
    pub fn extract_content_from_text(&self, text: &str, #[magnus(default = None)] format: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::extract_content_from_text(text, format.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // extract_task_actions
    pub fn extract_task_actions(&self, content: &str) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::extract_task_actions(content).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // extract_tasks
    pub fn extract_tasks(&self, content: &str, #[magnus(default = None)] context: Option<&str>) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::extract_tasks(content, context.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // fast
    pub fn fast(&self, query: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::fast(query).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // fast_search
    pub fn fast_search(&self, query: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::fast_search(query).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // filtered_semantic_search
    pub fn filtered_semantic_search(&self, query: &str, filters: SearchFilters, limit: usize) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::filtered_semantic_search(query, filters, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find
    pub fn find(&self, query: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::find(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_best_agent
    pub fn find_best_agent(&self, required_specialization: &str, #[magnus(default = None)] preferred_capability: Option<&str>) -> Result<Option<&Agent>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_best_agent(required_specialization, preferred_capability.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_by_tag
    pub fn find_by_tag(&self, tag_name: &str) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_by_tag(tag_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_cross_content_relationships
    pub fn find_cross_content_relationships(&self, content_id: &str, content_type: TodoziContentType, min_similarity: f32) -> Result<HashMap<TodoziContentType, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_cross_content_relationships(content_id, content_type, min_similarity).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_ideas
    pub fn find_ideas(&self, query: &str) -> Result<Vec<JsIdea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_ideas(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_memories
    pub fn find_memories(&self, query: &str) -> Result<Vec<JsMemory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_memories(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_outliers
    pub fn find_outliers(&self, content_type: TodoziContentType, threshold: f32) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_outliers(content_type, threshold).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_similar_tags
    pub fn find_similar_tags(&self, tag_name: &str, #[magnus(default = None)] limit: Option<usize>) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_similar_tags(tag_name, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_similar_tasks
    pub fn find_similar_tasks(&self, task_description: &str, #[magnus(default = None)] limit: Option<usize>) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_similar_tasks(task_description, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_tasks
    pub fn find_tasks(&self, query: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_tasks_ai
    pub fn find_tasks_ai(&self, query: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_tasks_ai(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_tdz
    pub fn find_tdz(&self, #[magnus(default = None)] str: Option<&str>) -> Result<Option<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_tdz(str.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // find_todozi
    pub fn find_todozi(&self, #[magnus(default = None)] str: Option<&str>) -> Result<Option<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::find_todozi(str.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // fix_completed_tasks_consistency
    pub fn fix_completed_tasks_consistency(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::fix_completed_tasks_consistency().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // fix_task_consistency
    pub fn fix_task_consistency(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::fix_task_consistency().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // force_overwrite
    pub fn force_overwrite(&self, force_overwrite: bool) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::force_overwrite(force_overwrite).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_duration
    pub fn format_duration(&self, from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_duration(from, to).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_error_with_context
    pub fn format_error_with_context(&self, error_message: &str, context: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_error_with_context(error_message, context).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_project_stats
    pub fn format_project_stats(&self, project_name: &str, task_count: usize, completed_count: usize) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_project_stats(project_name, task_count, completed_count).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_task
    pub fn format_task(&self, task: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_task(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_task_list
    pub fn format_task_list(&self, tasks: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_task_list(tasks).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_task_with_emojis
    pub fn format_task_with_emojis(&self, task: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_task_with_emojis(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // format_time_estimate
    pub fn format_time_estimate(&self, time: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::format_time_estimate(time).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // fuzzy_search
    pub fn fuzzy_search(&self, query: &str, max_distance: usize) -> Result<Vec<(&Tag, usize)>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::fuzzy_search(query, max_distance).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // generate_embedding
    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::generate_embedding(text).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // generate_embeddings_batch
    pub fn generate_embeddings_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::generate_embeddings_batch(texts).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // generate_task_embedding
    pub fn generate_task_embedding(&self, task: &str) -> Result<Option<Vec<f32>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::generate_task_embedding(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get
    pub fn get(&self, task_id: &str) -> Result<Option<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_active_items
    pub fn get_active_items(&self) -> Result<Vec<&QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_active_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_active_keys
    pub fn get_active_keys(&self) -> Result<Vec<&ApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_active_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_active_reminders
    pub fn get_active_reminders(&self) -> Result<Vec<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_active_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_active_sessions
    pub fn get_active_sessions(&self) -> Result<Vec<crate::models::QueueSession>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_active_sessions().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agent
    pub fn get_agent(&self, agent_id: &str) -> Result<Option<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agent(agent_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agent_assignments
    pub fn get_agent_assignments(&self, agent_id: &str) -> Result<Vec<&AgentAssignment>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agent_assignments(agent_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agent_assignments_dir
    pub fn get_agent_assignments_dir(&self, agent_id: &str) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agent_assignments_dir(agent_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agent_statistics
    pub fn get_agent_statistics(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agent_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agents_by_capability
    pub fn get_agents_by_capability(&self, capability: &str) -> Result<Vec<&Agent>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agents_by_capability(capability).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agents_by_specialization
    pub fn get_agents_by_specialization(&self, specialization: &str) -> Result<Vec<&Agent>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agents_by_specialization(specialization).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agents_dir
    pub fn get_agents_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agents_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_agents_with_assignments
    pub fn get_agents_with_assignments(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_agents_with_assignments().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_ai_tasks
    pub fn get_ai_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_active_tasks
    pub fn get_all_active_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_agents
    pub fn get_all_agents(&self) -> Result<Vec<&Agent>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_categories
    pub fn get_all_categories(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_categories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_completed_tasks
    pub fn get_all_completed_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_ideas
    pub fn get_all_ideas(&self) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_all_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_items
    pub fn get_all_items(&self) -> Result<Vec<&QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_keys
    pub fn get_all_keys(&self) -> Result<Vec<&ApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_memories
    pub fn get_all_memories(&self) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_reminders
    pub fn get_all_reminders(&self) -> Result<Vec<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_summaries
    pub fn get_all_summaries(&self) -> Result<Vec<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_summaries().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_tags
    pub fn get_all_tags(&self) -> Result<Vec<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_all_tags().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_tasks
    pub fn get_all_tasks(&self) -> Result<Vec<&Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_all_tools
    pub fn get_all_tools(&self) -> Result<Vec<&Box<dyn Tool>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_all_tools().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_api_key
    pub fn get_api_key(&self, user_id: &str) -> Result<JsApiKey, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_api_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_api_key_by_public
    pub fn get_api_key_by_public(&self, public_key: &str) -> Result<JsApiKey, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_api_key_by_public(public_key).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_assignee_emoji
    pub fn get_assignee_emoji(&self, assignee: &str) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_assignee_emoji(assignee).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_assignments_dir
    pub fn get_assignments_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_assignments_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_available_agents
    pub fn get_available_agents(&self) -> Result<Vec<Agent>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_available_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_backlog_items
    pub fn get_backlog_items(&self) -> Result<Vec<&QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_backlog_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_breakthrough_ideas
    pub fn get_breakthrough_ideas(&self) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_breakthrough_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunk
    pub fn get_chunk(&self, chunk_id: &str) -> Result<Option<&CodeChunk>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunk(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunk_mut
    pub fn get_chunk_mut(&self, chunk_id: &str) -> Result<Option<&mut CodeChunk>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunk_mut(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunk_status
    pub fn get_chunk_status(&self, chunk_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunk_status(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunking_level_info
    pub fn get_chunking_level_info(&self, level: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunking_level_info(level).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunking_levels
    pub fn get_chunking_levels(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunking_levels().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunks_by_level
    pub fn get_chunks_by_level(&self, level: ChunkingLevel) -> Result<Vec<&CodeChunk>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunks_by_level(level).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_chunks_dir
    pub fn get_chunks_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_chunks_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_collaborative_tasks
    pub fn get_collaborative_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_command_documentation
    pub fn get_command_documentation(&self, #[magnus(default = None)] command_name: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_command_documentation(command_name.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_complete_items
    pub fn get_complete_items(&self) -> Result<Vec<&QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_complete_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_critical_memories
    pub fn get_critical_memories(&self) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_critical_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_current_duration
    pub fn get_current_duration(&self) -> Result<u64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_current_duration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_default_model
    pub fn get_default_model(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_default_model().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_dependency_chain
    pub fn get_dependency_chain(&self, chunk_id: &str) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_dependency_chain(chunk_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_embedding_statistics
    pub fn get_embedding_statistics(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_embedding_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_emotional_memories
    pub fn get_emotional_memories(&self, emotion: &str) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_emotional_memories(emotion).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_enabled_tools
    pub fn get_enabled_tools(&self) -> Result<Vec<&AgentTool>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_enabled_tools().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_error_details
    pub fn get_error_details(&self, error_message: &str) -> Result<JsErrorDetails, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_error_details(error_message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_errors_dir
    pub fn get_errors_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_errors_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_filtered_tasks
    pub fn get_filtered_tasks(&self, filters: &str) -> Result<Vec<&Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_filtered_tasks(filters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_formatted_prompt
    pub fn get_formatted_prompt(&self, variables: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_formatted_prompt(variables).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_high_priority_summaries
    pub fn get_high_priority_summaries(&self) -> Result<Vec<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_high_priority_summaries().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_human_memories
    pub fn get_human_memories(&self) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_human_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_human_tasks
    pub fn get_human_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_idea
    pub fn get_idea(&self, idea_id: &str) -> Result<Option<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_idea(idea_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_idea_statistics
    pub fn get_idea_statistics(&self) -> Result<IdeaStatistics, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_idea_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_ideas_by_importance
    pub fn get_ideas_by_importance(&self, importance: IdeaImportance) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_ideas_by_importance(importance).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_ideas_by_share_level
    pub fn get_ideas_by_share_level(&self, share_level: ShareLevel) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_ideas_by_share_level(share_level).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_ideas_by_tag
    pub fn get_ideas_by_tag(&self, tag: &str) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_ideas_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_ideas_dir
    pub fn get_ideas_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_ideas_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_item
    pub fn get_item(&self, id: &str) -> Result<Option<&QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_item(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_item_mut
    pub fn get_item_mut(&self, id: &str) -> Result<Option<&mut QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_item_mut(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_items_by_status
    pub fn get_items_by_status(&self, status: QueueStatus) -> Result<Vec<&QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_items_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_key
    pub fn get_key(&self, user_id: &str) -> Result<Option<&ApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_key_by_public
    pub fn get_key_by_public(&self, public_key: &str) -> Result<Option<&ApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_key_by_public(public_key).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_long_term_memories
    pub fn get_long_term_memories(&self) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_long_term_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memories_by_importance
    pub fn get_memories_by_importance(&self, importance: MemoryImportance) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_memories_by_importance(importance).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memories_by_tag
    pub fn get_memories_by_tag(&self, tag: &str) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_memories_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memories_by_term
    pub fn get_memories_by_term(&self, term: MemoryTerm) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_memories_by_term(term).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memories_by_type
    pub fn get_memories_by_type(&self, memory_type: &str) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_memories_by_type(memory_type).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memories_dir
    pub fn get_memories_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_memories_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memory
    pub fn get_memory(&self, memory_id: &str) -> Result<Option<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Memories::get_memory(memory_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_memory_statistics
    pub fn get_memory_statistics(&self) -> Result<MemoryStatistics, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Memories::get_memory_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_most_used_tags
    pub fn get_most_used_tags(&self, limit: usize) -> Result<Vec<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_most_used_tags(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_next_chunk_to_work_on
    pub fn get_next_chunk_to_work_on(&self) -> Result<Option<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_next_chunk_to_work_on().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_or_generate_embedding
    pub fn get_or_generate_embedding(&self, content_id: &str, text: &str, content_type: TodoziContentType, refresh_if_stale: bool) -> Result<Vec<f32>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_or_generate_embedding(content_id, text, content_type, refresh_if_stale).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_overdue_reminders
    pub fn get_overdue_reminders(&self) -> Result<Vec<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_overdue_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_pending_reminders
    pub fn get_pending_reminders(&self) -> Result<Vec<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_pending_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_priority_emoji
    pub fn get_priority_emoji(&self, priority: &str) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_priority_emoji(priority).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_private_ideas
    pub fn get_private_ideas(&self) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_private_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_processor_state
    pub fn get_processor_state(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_processor_state().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_project
    pub fn get_project(&self, name: &str) -> Result<Project, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_project(name).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_project_stats
    pub fn get_project_stats(&self, project_name: &str) -> Result<ProjectStats, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Projects::get_project_stats(project_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_project_summary
    pub fn get_project_summary(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Projects::get_project_summary().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_project_tasks
    pub fn get_project_tasks(&self, project_name: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_project_tasks_dir
    pub fn get_project_tasks_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_project_tasks_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_public_ideas
    pub fn get_public_ideas(&self) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_public_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_queue_item
    pub fn get_queue_item(&self, id: &str) -> Result<QueueItem, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Queue::get_queue_item(id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_queue_session
    pub fn get_queue_session(&self, session_id: &str) -> Result<crate::models::QueueSession, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Queue::get_queue_session(session_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_queue_statistics
    pub fn get_queue_statistics(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Queue::get_queue_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_ready_chunks
    pub fn get_ready_chunks(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_ready_chunks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_recent_actions
    pub fn get_recent_actions(&self, #[magnus(default = None)] limit: Option<u32>) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_recent_actions(limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_recent_ideas
    pub fn get_recent_ideas(&self, limit: usize) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_recent_ideas(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_recent_memories
    pub fn get_recent_memories(&self, limit: usize) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_recent_memories(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_recent_reminders
    pub fn get_recent_reminders(&self, limit: usize) -> Result<Vec<&Reminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_recent_reminders(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_recent_summaries
    pub fn get_recent_summaries(&self, limit: usize) -> Result<Vec<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_recent_summaries(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_recent_tags
    pub fn get_recent_tags(&self, limit: usize) -> Result<Vec<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_recent_tags(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_registration_info
    pub fn get_registration_info(&self) -> Result<Option<RegistrationInfo>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_registration_info().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_related_tags
    pub fn get_related_tags(&self, tag_id: &str) -> Result<Vec<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_related_tags(tag_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_reminder
    pub fn get_reminder(&self, reminder_id: &str) -> Result<Option<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_reminder(reminder_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_reminder_statistics
    pub fn get_reminder_statistics(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_reminder_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_reminders_by_priority
    pub fn get_reminders_by_priority(&self, priority: ReminderPriority) -> Result<Vec<&Reminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_reminders_by_priority(priority).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_reminders_by_status
    pub fn get_reminders_by_status(&self, status: ReminderStatus) -> Result<Vec<&Reminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_reminders_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_reminders_by_tag
    pub fn get_reminders_by_tag(&self, tag: &str) -> Result<Vec<&Reminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_reminders_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_reminders_due_soon
    pub fn get_reminders_due_soon(&self, duration: Duration) -> Result<Vec<&Reminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_reminders_due_soon(duration).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_search_analytics
    pub fn get_search_analytics(&self) -> Result<SearchAnalytics, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_search_analytics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_search_history
    pub fn get_search_history(&self, #[magnus(default = None)] limit: Option<u32>) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_search_history(limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_search_suggestions
    pub fn get_search_suggestions(&self, query: &str, limit: usize) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_search_suggestions(query, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_secret_memories
    pub fn get_secret_memories(&self) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_secret_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_server_status
    pub fn get_server_status(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_server_status().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_session
    pub fn get_session(&self, id: &str) -> Result<Option<&QueueSession>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_session(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_short_term_memories
    pub fn get_short_term_memories(&self) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_short_term_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_stats
    pub fn get_stats(&self) -> Result<HashMap<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_stats().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_status_emoji
    pub fn get_status_emoji(&self, status: &str) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_status_emoji(status).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_storage_dir
    pub fn get_storage_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_storage_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_suggestions
    pub fn get_suggestions(&self, current_tags: &str, limit: usize) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_suggestions(current_tags, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_summaries_by_priority
    pub fn get_summaries_by_priority(&self, priority: SummaryPriority) -> Result<Vec<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_summaries_by_priority(priority).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_summaries_by_tag
    pub fn get_summaries_by_tag(&self, tag: &str) -> Result<Vec<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_summaries_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_summary
    pub fn get_summary(&self, summary_id: &str) -> Result<Option<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_summary(summary_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_summary_statistics
    pub fn get_summary_statistics(&self) -> Result<SummaryStatistics, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_summary_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tag
    pub fn get_tag(&self, tag_id: &str) -> Result<Option<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_tag(tag_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tag_by_name
    pub fn get_tag_by_name(&self, name: &str) -> Result<Option<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_tag_by_name(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tag_statistics
    pub fn get_tag_statistics(&self) -> Result<TagStatistics, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_tag_statistics().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tags_by_category
    pub fn get_tags_by_category(&self, category: &str) -> Result<Vec<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::get_tags_by_category(category).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_task
    pub fn get_task(&self, id: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_task(id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_task_assignments
    pub fn get_task_assignments(&self, task_id: &str) -> Result<Vec<&AgentAssignment>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_task_assignments(task_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_task_from_any_project
    pub fn get_task_from_any_project(&self, id: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_task_from_any_project(id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_task_from_project
    pub fn get_task_from_project(&self, project_name: &str, task_id: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_task_from_project(project_name, task_id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_task_mut
    pub fn get_task_mut(&self, id: &str) -> Result<Option<&mut Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_task_mut(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tasks_dir
    pub fn get_tasks_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_tasks_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tdz_api_key
    pub fn get_tdz_api_key(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_tdz_api_key().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_team_ideas
    pub fn get_team_ideas(&self) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::get_team_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tool
    pub fn get_tool(&self, name: &str) -> Result<Option<&Box<dyn Tool>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_tool(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tool_definitions
    pub fn get_tool_definitions(&self) -> Result<Vec<serde_json::Value>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_tool_definitions().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_training_dir
    pub fn get_training_dir(&self) -> Result<PathBuf, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_training_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_tsne_coordinates
    pub fn get_tsne_coordinates(&self, content_ids: Vec<String>, dimensions: usize) -> Result<Vec<(Strin>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_tsne_coordinates(content_ids, dimensions).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_unresolved_errors
    pub fn get_unresolved_errors(&self) -> Result<Vec<&Error>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_unresolved_errors().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // get_version_history
    pub fn get_version_history(&self, content_id: &str) -> Result<Vec<serde_json::Value>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::get_version_history(content_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_add_command
    pub fn handle_add_command(&self, command: AddCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_add_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_agent_command
    pub fn handle_agent_command(&self, command: Commands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_agent_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_ai_commands
    pub fn handle_ai_commands(&self, command: &str, args: Vec<String>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_ai_commands(command, args).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_api_command
    pub fn handle_api_command(&self, command: ApiCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_api_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_chat_command
    pub fn handle_chat_command(&self, command: Commands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_chat_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_emb_command
    pub fn handle_emb_command(&self, command: Commands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_emb_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_error_command
    pub fn handle_error_command(&self, command: Commands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_error_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_extract_command
    pub fn handle_extract_command(&self, #[magnus(default = None)] content: Option<String>, #[magnus(default = None)] file: Option<String>, output_format: &str, human: bool) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_extract_command(content.as_deref(), file.as_deref(), output_format, human).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_idea_command
    pub fn handle_idea_command(&self, command: IdeaCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_idea_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_ind_command
    pub fn handle_ind_command(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_ind_command().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_list_backups_command
    pub fn handle_list_backups_command(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_list_backups_command().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_list_command
    pub fn handle_list_command(&self, command: ListCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_list_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_memory_command
    pub fn handle_memory_command(&self, command: MemoryCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_memory_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_project_command
    pub fn handle_project_command(&self, command: ProjectCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_project_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_queue_command
    pub fn handle_queue_command(&self, command: QueueCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_queue_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_search_all_command
    pub fn handle_search_all_command(&self, command: Commands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_search_all_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_search_command
    pub fn handle_search_command(&self, command: SearchCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_search_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_server_command
    pub fn handle_server_command(&self, command: ServerCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_server_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_show_command
    pub fn handle_show_command(&self, command: ShowCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_show_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_stats_command
    pub fn handle_stats_command(&self, _command: StatsCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_stats_command(_command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_strategy_command
    pub fn handle_strategy_command(&self, #[magnus(default = None)] content: Option<String>, #[magnus(default = None)] file: Option<String>, output_format: &str, human: bool) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_strategy_command(content.as_deref(), file.as_deref(), output_format, human).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_train_command
    pub fn handle_train_command(&self, command: TrainingCommands) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_train_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // handle_update_command
    pub fn handle_update_command(&self, id: &str, #[magnus(default = None)] action: Option<String>, #[magnus(default = None)] time: Option<String>, #[magnus(default = None)] priority: Option<String>, #[magnus(default = None)] project: Option<String>, #[magnus(default = None)] status: Option<String>, #[magnus(default = None)] assignee: Option<String>, #[magnus(default = None)] tags: Option<String>, #[magnus(default = None)] dependencies: Option<String>, #[magnus(default = None)] context: Option<String>, #[magnus(default = None)] progress: Option<u8>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::handle_update_command(id, action.as_deref(), time.as_deref(), priority.as_deref(), project.as_deref(), status.as_deref(), assignee.as_deref(), tags.as_deref(), dependencies.as_deref(), context.as_deref(), progress.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // has_capability
    pub fn has_capability(&self, capability: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::has_capability(capability).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // has_results
    pub fn has_results(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::has_results().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // has_specialization
    pub fn has_specialization(&self, specialization: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::has_specialization(specialization).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // has_tool
    pub fn has_tool(&self, tool_name: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::has_tool(tool_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // hash_project_name
    pub fn hash_project_name(&self, project_name: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::hash_project_name(project_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // hierarchical_clustering
    pub fn hierarchical_clustering(&self, content_types: Vec<TodoziContentType>, max_depth: usize) -> Result<Vec<HierarchicalCluster>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::hierarchical_clustering(content_types, max_depth).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // high
    pub fn high(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::high(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // high_priority_percentage
    pub fn high_priority_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::high_priority_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // human
    pub fn human(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Actions::human(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // human_task
    pub fn human_task(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::human_task(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // hybrid_search
    pub fn hybrid_search(&self, query: &str, keywords: Vec<String>, #[magnus(default = None)] content_types: Option<Vec<TodoziContentType>>, semantic_weight: f32, // 0.0-1.0
        limit: usize) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::hybrid_search(query, keywords, content_types.as_deref(), semantic_weight, // 0.0-1.0
        limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // idea
    pub fn idea(&self, idea: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::idea(idea).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ideate
    pub fn ideate(&self, idea: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::ideate(idea).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // import_from_format
    pub fn import_from_format(&self, data: &str, format: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::import_from_format(data, format).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // importance
    pub fn importance(&self, importance: IdeaImportance) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::importance(importance).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // important
    pub fn important(&self, moment: &str, meaning: &str, reason: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Memories::important(moment, meaning, reason).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // important_memory
    pub fn important_memory(&self, moment: &str, meaning: &str, reason: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::important_memory(moment, meaning, reason).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // increment_tag_usage
    pub fn increment_tag_usage(&self, tag_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::increment_tag_usage(tag_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // init
    pub fn init(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::init().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // init_storage
    pub fn init_storage(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::init_storage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // init_with_auto_registration
    pub fn init_with_auto_registration(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::init_with_auto_registration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // initialize
    pub fn initialize(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::initialize().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // initialize_grok_level_todozi_system
    pub fn initialize_grok_level_todozi_system(&self) -> Result<std::result::Result<
    SharedTodozi,
    TodoziError,
>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::initialize_grok_level_todozi_system().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // initialize_grok_level_todozi_system_with_embedding
    pub fn initialize_grok_level_todozi_system_with_embedding(&self, enable_embeddings: bool) -> Result<std::result::Result<(SharedTodozi, Option<TodoziEmbeddingService>), TodoziError>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::initialize_grok_level_todozi_system_with_embedding(enable_embeddings).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // initialize_tdz_content_processor
    pub fn initialize_tdz_content_processor(&self) -> Result<SharedTodoziState, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::initialize_tdz_content_processor().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // initialize_tdz_processor
    pub fn initialize_tdz_processor(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::initialize_tdz_processor().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // interactive_create_task
    pub fn interactive_create_task(&self) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::interactive_create_task().await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // io
    pub fn io(&self, message: impl Into<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::io(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_active
    pub fn is_active(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_active().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_admin
    pub fn is_admin(&self, public_key: &str, private_key: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_admin(public_key, private_key).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_available
    pub fn is_available(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_available().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_backlog
    pub fn is_backlog(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_backlog().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_complete
    pub fn is_complete(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_complete().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_completed
    pub fn is_completed(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_completed().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_empty
    pub fn is_empty(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_empty().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_overdue
    pub fn is_overdue(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_overdue().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // is_registered
    pub fn is_registered(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::is_registered().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // keyword_search
    pub fn keyword_search(&self, query: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::keyword_search(query).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // keyword_tasks
    pub fn keyword_tasks(&self, query: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::keyword_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // launch_gui
    pub fn launch_gui(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::launch_gui().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // launch_tui
    pub fn launch_tui(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::launch_tui().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // len
    pub fn len(&self) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::len().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list
    pub fn list(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_active_api_keys
    pub fn list_active_api_keys(&self) -> Result<Vec<JsApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_active_api_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_active_items
    pub fn list_active_items(&self) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_active_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_agent_assignments
    pub fn list_agent_assignments(&self, agent_id: &str) -> Result<Vec<AgentAssignment>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_agent_assignments(agent_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_agents
    pub fn list_agents(&self) -> Result<Vec<Agent>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_all_agent_assignments
    pub fn list_all_agent_assignments(&self) -> Result<Vec<AgentAssignment>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_all_agent_assignments().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_api_keys
    pub fn list_api_keys(&self) -> Result<Vec<JsApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_api_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_available_agents
    pub fn list_available_agents(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_available_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_available_commands
    pub fn list_available_commands(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_available_commands().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_available_tools
    pub fn list_available_tools(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_available_tools().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_backlog_items
    pub fn list_backlog_items(&self) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_backlog_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_backups
    pub fn list_backups(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_backups().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_code_chunks
    pub fn list_code_chunks(&self) -> Result<Vec<CodeChunk>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_code_chunks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_complete_items
    pub fn list_complete_items(&self) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_complete_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_errors
    pub fn list_errors(&self) -> Result<Vec<Error>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_errors().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_feelings
    pub fn list_feelings(&self) -> Result<Vec<Feeling>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_feelings().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_ideas
    pub fn list_ideas(&self) -> Result<Vec<Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Ideas::list_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_memories
    pub fn list_memories(&self) -> Result<Vec<Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_project_task_containers
    pub fn list_project_task_containers(&self) -> Result<Vec<ProjectTaskContainer>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_project_task_containers().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_projects
    pub fn list_projects(&self) -> Result<Vec<Project>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Projects::list_projects().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_queue_items
    pub fn list_queue_items(&self) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Queue::list_queue_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_queue_items_by_status
    pub fn list_queue_items_by_status(&self, status: QueueStatus) -> Result<Vec<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Queue::list_queue_items_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_reminders
    pub fn list_reminders(&self) -> Result<Vec<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_summaries
    pub fn list_summaries(&self) -> Result<Vec<JsSummary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_summaries().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_tasks
    pub fn list_tasks(&self) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_tasks_across_projects
    pub fn list_tasks_across_projects(&self, filters: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_tasks_across_projects(filters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_tasks_in_project
    pub fn list_tasks_in_project(&self, project_name: &str, filters: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_tasks_in_project(project_name, filters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // list_training_data
    pub fn list_training_data(&self) -> Result<Vec<TrainingData>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::list_training_data().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load
    pub fn load(&self, model_name: &str, device: Device) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load(model_name, device).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_additional_model
    pub fn load_additional_model(&self, model_name: &str, model_alias: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_additional_model(model_name, model_alias).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_agent
    pub fn load_agent(&self, agent_id: &str) -> Result<Agent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_agent(agent_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_agent_assignment
    pub fn load_agent_assignment(&self, agent_id: &str, task_id: &str) -> Result<AgentAssignment, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_agent_assignment(agent_id, task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_agents
    pub fn load_agents(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_agents().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_api_key_collection
    pub fn load_api_key_collection(&self) -> Result<ApiKeyCollection, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_api_key_collection().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_api_keys
    pub fn load_api_keys(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_api_keys().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_code_chunk
    pub fn load_code_chunk(&self, chunk_id: &str) -> Result<CodeChunk, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_code_chunk(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_config
    pub fn load_config(&self) -> Result<Config, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_config().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_error
    pub fn load_error(&self, error_id: &str) -> Result<Error, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_error(error_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_extended_data
    pub fn load_extended_data(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_extended_data().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_feeling
    pub fn load_feeling(&self, id: &str) -> Result<Feeling, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_feeling(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_idea
    pub fn load_idea(&self, idea_id: &str) -> Result<Idea, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_idea(idea_id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_memory
    pub fn load_memory(&self, memory_id: &str) -> Result<Memory, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_memory(memory_id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_project
    pub fn load_project(&self, project_name: &str) -> Result<Project, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_project(project_name).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_project_task_container
    pub fn load_project_task_container(&self, project_name: &str) -> Result<ProjectTaskContainer, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_project_task_container(project_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_project_task_container_by_hash
    pub fn load_project_task_container_by_hash(&self, project_hash: &str) -> Result<ProjectTaskContainer, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_project_task_container_by_hash(project_hash).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_queue_collection
    pub fn load_queue_collection(&self) -> Result<QueueCollection, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_queue_collection().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_task
    pub fn load_task(&self, task_id: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_task(task_id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_task_collection
    pub fn load_task_collection(&self, collection_name: &str) -> Result<TaskCollection, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_task_collection(collection_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_tasks
    pub fn load_tasks(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_tasks().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // load_training_data
    pub fn load_training_data(&self, training_data_id: &str) -> Result<TrainingData, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::load_training_data(training_data_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // long_term_percentage
    pub fn long_term_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::long_term_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // low
    pub fn low(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::low(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // mark_chunk_completed
    pub fn mark_chunk_completed(&self, chunk_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::mark_chunk_completed(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // mark_chunk_validated
    pub fn mark_chunk_validated(&self, chunk_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::mark_chunk_validated(chunk_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // mark_reminder_cancelled
    pub fn mark_reminder_cancelled(&self, reminder_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::mark_reminder_cancelled(reminder_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // mark_reminder_completed
    pub fn mark_reminder_completed(&self, reminder_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::mark_reminder_completed(reminder_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // matches
    pub fn matches(&self, public_key: &str, #[magnus(default = None)] private_key: Option<&str>) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::matches(public_key, private_key.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // max_tokens
    pub fn max_tokens(&self) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::max_tokens().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // meaning
    pub fn meaning(&self, meaning: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::meaning(meaning).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // merge_tags
    pub fn merge_tags(&self, primary_tag_id: &str, duplicate_tag_ids: Vec<String>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::merge_tags(primary_tag_id, duplicate_tag_ids).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // migrate
    pub fn migrate(&self) -> Result<MigrationReport, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::migrate().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // migrate_tasks
    pub fn migrate_tasks(&self, #[magnus(default = None)] dry_run: Option<bool>, #[magnus(default = None)] verbose: Option<bool>, #[magnus(default = None)] force_overwrite: Option<bool>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::migrate_tasks(dry_run.as_deref(), verbose.as_deref(), force_overwrite.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // migrate_to_project_based
    pub fn migrate_to_project_based(&self) -> Result<MigrationReport, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::migrate_to_project_based().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // moment
    pub fn moment(&self, moment: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::moment(moment).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // move_task
    pub fn move_task(&self, id: &str, from_collection: &str, to_collection: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::move_task(id, from_collection, to_collection).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // multi_query_search
    pub fn multi_query_search(&self, queries: Vec<&str>, aggregation: AggregationType, #[magnus(default = None)] content_types: Option<Vec<TodoziContentType>>, limit: usize) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::multi_query_search(queries, aggregation, content_types.as_deref(), limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // name
    pub fn name(&self, name: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::name(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // new_full
    pub fn new_full(&self, user_id: &str, action: &str, time: &str, priority: Priority, parent_project: &str, status: Status, #[magnus(default = None)] assignee: Option<Assignee>, tags: Vec<String>, dependencies: Vec<String>, #[magnus(default = None)] context_notes: Option<String>, #[magnus(default = None)] progress: Option<u8>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::new_full(user_id, action, time, priority, parent_project, status, assignee.as_deref(), tags, dependencies, context_notes.as_deref(), progress.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // new_idea
    pub fn new_idea(&self, idea: Idea) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::new_idea(idea).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // new_memory
    pub fn new_memory(&self, memory: Memory) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::new_memory(memory).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // new_with_hashes
    pub fn new_with_hashes(&self, server_url: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::new_with_hashes(server_url).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // ok
    pub fn ok(&self, body: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::ok(body).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // overdue_percentage
    pub fn overdue_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::overdue_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_agent_assignment_format
    pub fn parse_agent_assignment_format(&self, agent_text: &str) -> Result<AgentAssignment, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_agent_assignment_format(agent_text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_chunking_format
    pub fn parse_chunking_format(&self, chunk_text: &str) -> Result<CodeChunk, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_chunking_format(chunk_text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_dependencies
    pub fn parse_dependencies(&self, deps_str: &str) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_dependencies(deps_str).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_error_format
    pub fn parse_error_format(&self, error_text: &str) -> Result<Error, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_error_format(error_text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_feeling_format
    pub fn parse_feeling_format(&self, feel_text: &str) -> Result<Feeling, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_feeling_format(feel_text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_idea_format
    pub fn parse_idea_format(&self, idea_text: &str) -> Result<Idea, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_idea_format(idea_text).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_memory_format
    pub fn parse_memory_format(&self, memory_text: &str, user_id: &str) -> Result<Memory, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_memory_format(memory_text, user_id).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_reminder_format
    pub fn parse_reminder_format(&self, reminder_text: &str) -> Result<Reminder, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_reminder_format(reminder_text).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_summary_format
    pub fn parse_summary_format(&self, summary_text: &str) -> Result<Summary, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_summary_format(summary_text).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_tags
    pub fn parse_tags(&self, tags_str: &str) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_tags(tags_str).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_tdz_command
    pub fn parse_tdz_command(&self, text: &str) -> Result<Vec<TdzCommand>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_tdz_command(text).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_todozi_format
    pub fn parse_todozi_format(&self, todozi_text: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_todozi_format(todozi_text).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // parse_training_data_format
    pub fn parse_training_data_format(&self, train_text: &str) -> Result<TrainingData, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::parse_training_data_format(train_text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // pending_percentage
    pub fn pending_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::pending_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // plan_task_actions
    pub fn plan_task_actions(&self, goal: &str) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::plan_task_actions(goal).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // plan_tasks
    pub fn plan_tasks(&self, goal: &str, #[magnus(default = None)] complexity: Option<&str>, #[magnus(default = None)] timeline: Option<&str>, #[magnus(default = None)] context: Option<&str>) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::plan_tasks(goal, complexity.as_deref(), timeline.as_deref(), context.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // predict_relevance
    pub fn predict_relevance(&self, _features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::predict_relevance(_features).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // preload_related_embeddings
    pub fn preload_related_embeddings(&self, content_id: &str, depth: usize) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::preload_related_embeddings(content_id, depth).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // prepare_task_content
    pub fn prepare_task_content(&self, task: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::prepare_task_content(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // prioritize_queue_item
    pub fn prioritize_queue_item(&self, item_id: &str, priority: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::prioritize_queue_item(item_id, priority).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // priority
    pub fn priority(&self, priority: SummaryPriority) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::priority(priority).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // private_percentage
    pub fn private_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::private_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_chat
    pub fn process_chat(&self, message: &str, user_id: &str) -> Result<ChatContent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_chat(message, user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_chat_message
    pub fn process_chat_message(&self, message: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_chat_message(message).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_chat_message_extended
    pub fn process_chat_message_extended(&self, message: &str, user_id: &str) -> Result<ChatContent, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_chat_message_extended(message, user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_chunking_message
    pub fn process_chunking_message(&self, message: &str) -> Result<Vec<CodeChunk>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_chunking_message(message).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_json_examples
    pub fn process_json_examples(&self, json_data: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_json_examples(json_data).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_tdz_commands
    pub fn process_tdz_commands(&self, text: &str, base_url: &str, #[magnus(default = None)] api_key: Option<&str>) -> Result<Vec<Value>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_tdz_commands(text, base_url, api_key.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // process_workflow
    pub fn process_workflow(&self, tasks: Vec<Task>) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::process_workflow(tasks).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // profile_search_performance
    pub fn profile_search_performance(&self, query: &str, iterations: usize) -> Result<PerformanceMetrics, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::profile_search_performance(query, iterations).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // project_name
    pub fn project_name(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::project_name().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // project_tasks
    pub fn project_tasks(&self, project_name: &str) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // public_percentage
    pub fn public_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::public_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_active
    pub fn queue_active(&self) -> Result<Vec<JsQueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_active().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_add
    pub fn queue_add(&self, task_name: &str, description: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_add(task_name, description).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_backlog
    pub fn queue_backlog(&self) -> Result<Vec<JsQueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_backlog().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_bulk_operations
    pub fn queue_bulk_operations(&self, operations: Vec<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_bulk_operations(operations).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_complete
    pub fn queue_complete(&self, session_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_complete(session_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_list
    pub fn queue_list(&self) -> Result<Vec<JsQueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_list().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // queue_start
    pub fn queue_start(&self, item_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::queue_start(item_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // quick
    pub fn quick(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::quick().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // quick_task
    pub fn quick_task(&self, action: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::quick_task(action).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // reason
    pub fn reason(&self, reason: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::reason(reason).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // recommend_similar
    pub fn recommend_similar(&self, based_on: Vec<String>, exclude: Vec<String>, limit: usize) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::recommend_similar(based_on, exclude, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // register_tool
    pub fn register_tool(&self, tool_def: JsToolDefinition) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::register_tool(tool_def).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // register_with_server
    pub fn register_with_server(&self, server_url: &str) -> Result<RegistrationInfo, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::register_with_server(server_url).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // relationships_per_tag
    pub fn relationships_per_tag(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::relationships_per_tag().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remember
    pub fn remember(&self, moment: &str, meaning: &str) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::remember(moment, meaning).await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remind_at
    pub fn remind_at(&self, remind_at: DateTime<Utc>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::remind_at(remind_at).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remove_api_key
    pub fn remove_api_key(&self, user_id: &str) -> Result<JsApiKey, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::remove_api_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remove_from_task
    pub fn remove_from_task(&self, task_id: &str, tag: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tags::remove_from_task(task_id, tag).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remove_item
    pub fn remove_item(&self, id: &str) -> Result<Option<QueueItem>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::remove_item(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remove_key
    pub fn remove_key(&self, user_id: &str) -> Result<Option<ApiKey>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::remove_key(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remove_tag_from_task
    pub fn remove_tag_from_task(&self, task_id: &str, tag: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::remove_tag_from_task(task_id, tag).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // remove_task
    pub fn remove_task(&self, id: &str) -> Result<Option<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::remove_task(id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // render
    pub fn render(&self, config: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::render(config).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // render_compact
    pub fn render_compact(&self, config: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::render_compact(config).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // render_detailed
    pub fn render_detailed(&self, config: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::render_detailed(config).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // reorder_queue
    pub fn reorder_queue(&self, item_ids: Vec<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::reorder_queue(item_ids).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // resolve_error
    pub fn resolve_error(&self, error_id: &str, resolution: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::resolve_error(error_id, resolution).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // restore_backup
    pub fn restore_backup(&self, backup_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::restore_backup(backup_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // restore_embeddings
    pub fn restore_embeddings(&self, backup_path: &str) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::restore_embeddings(backup_path).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // run
    pub fn run(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::run().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // run_interactive
    pub fn run_interactive(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::run_interactive().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // sample_task
    pub fn sample_task(&self) -> Result<Task, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::sample_task().await.map(|item| item.to_ruby())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_agent
    pub fn save_agent(&self, agent: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_agent(agent).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_agent_assignment
    pub fn save_agent_assignment(&self, assignment: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_agent_assignment(assignment).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_api_key_collection
    pub fn save_api_key_collection(&self, collection: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_api_key_collection(collection).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_as_default
    pub fn save_as_default(&self, model_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_as_default(model_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_code_chunk
    pub fn save_code_chunk(&self, chunk: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_code_chunk(chunk).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_config
    pub fn save_config(&self, config: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_config(config).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_error
    pub fn save_error(&self, error: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_error(error).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_feeling
    pub fn save_feeling(&self, feeling: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_feeling(feeling).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_idea
    pub fn save_idea(&self, idea: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_idea(idea).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_memory
    pub fn save_memory(&self, memory: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_memory(memory).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_processed_content
    pub fn save_processed_content(&self, raw: &str, cleaned: &str, session_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_processed_content(raw, cleaned, session_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_project
    pub fn save_project(&self, project: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_project(project).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_project_task_container
    pub fn save_project_task_container(&self, container: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_project_task_container(container).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_queue_collection
    pub fn save_queue_collection(&self, collection: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_queue_collection(collection).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_search_query
    pub fn save_search_query(&self, query: &str, #[magnus(default = None)] name: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_search_query(query, name.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_task
    pub fn save_task(&self, task: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_task(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_task_collection
    pub fn save_task_collection(&self, collection_name: &str, collection: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_task_collection(collection_name, collection).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_task_with_embedding
    pub fn save_task_with_embedding(&self, task: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_task_with_embedding(task).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_tasks
    pub fn save_tasks(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_tasks().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // save_training_data
    pub fn save_training_data(&self, training_data: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::save_training_data(training_data).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search
    pub fn search(&self, query: &str, options: SearchOptions) -> Result<SearchResults, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search(query, options).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_ideas
    pub fn search_ideas(&self, query: &str) -> Result<Vec<&Idea>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_ideas(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_memories
    pub fn search_memories(&self, query: &str) -> Result<Vec<&Memory>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_memories(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_reminders
    pub fn search_reminders(&self, query: &str) -> Result<Vec<JsReminder>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_reminders(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_summaries
    pub fn search_summaries(&self, query: &str) -> Result<Vec<&Summary>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_summaries(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_tags
    pub fn search_tags(&self, query: &str) -> Result<Vec<&Tag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_tags(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_tasks
    pub fn search_tasks(&self, query: &str, semantic: bool, #[magnus(default = None)] limit: Option<usize>) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_tasks(query, semantic, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_tasks_semantic
    pub fn search_tasks_semantic(&self, query: &str, max_results: usize) -> Result<Vec<SemanticSearchResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_tasks_semantic(query, max_results).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // search_with_filters
    pub fn search_with_filters(&self, filters: TaskFilters, #[magnus(default = None)] limit: Option<usize>) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::search_with_filters(filters, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // see_all
    pub fn see_all(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Easy::see_all().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // semantic_search
    pub fn semantic_search(&self, query: &str, #[magnus(default = None)] content_types: Option<Vec<TodoziContentType>>, #[magnus(default = None)] limit: Option<usize>) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::semantic_search(query, content_types.as_deref(), limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // serialization
    pub fn serialization(&self, message: impl Into<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::serialization(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // set_color_scheme
    pub fn set_color_scheme(&self, scheme_json: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::set_color_scheme(scheme_json).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // share
    pub fn share(&self, share: ShareLevel) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::share(share).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // short_term_percentage
    pub fn short_term_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::short_term_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // show_loading_screen
    pub fn show_loading_screen(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::show_loading_screen().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // similar
    pub fn similar(&self, query: &str) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::similar(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // similar_tasks
    pub fn similar_tasks(&self, task_id: &str) -> Result<Vec<SimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::similar_tasks(task_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // similar_tasks_emb
    pub fn similar_tasks_emb(&self, query: &str) -> Result<Vec<JsSimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::similar_tasks_emb(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // similarity_threshold_search
    pub fn similarity_threshold_search(&self, query: &str, threshold: f64, #[magnus(default = None)] limit: Option<u32>) -> Result<Vec<JsSimilarityResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::similarity_threshold_search(query, threshold, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // smart
    pub fn smart(&self, query: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::smart(query).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // smart_search
    pub fn smart_search(&self, query: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::smart_search(query).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // specializations
    pub fn specializations(&self, specializations: Vec<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::specializations(specializations).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start
    pub fn start(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::start(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_edit
    pub fn start_edit(&self, _task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_edit(_task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_edit_session
    pub fn start_edit_session(&self, task_id: &str) -> Result<EditSession, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_edit_session(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_local_server
    pub fn start_local_server(&self, #[magnus(default = None)] host: Option<String>, #[magnus(default = None)] port: Option<u32>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_local_server(host.as_deref(), port.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_queue_session
    pub fn start_queue_session(&self, queue_item_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_queue_session(queue_item_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_server
    pub fn start_server(&self, #[magnus(default = None)] host: Option<String>, #[magnus(default = None)] port: Option<u16>) -> Result<std::result::Result<(), Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_server(host.as_deref(), port.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_session
    pub fn start_session(&self, queue_item_id: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_session(queue_item_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // start_task
    pub fn start_task(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::start_task(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // stats
    pub fn stats(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Emb::stats().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // status
    pub fn status(&self, status: AgentStatus) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::status(status).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // stop_server
    pub fn stop_server(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::stop_server().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage
    pub fn storage(&self) -> Result<Storage, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_check_folder_structure
    pub fn storage_check_folder_structure(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_check_folder_structure().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_clear_registration
    pub fn storage_clear_registration(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_clear_registration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_create_backup
    pub fn storage_create_backup(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_create_backup().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_delete_project_by_name
    pub fn storage_delete_project_by_name(&self, name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_delete_project_by_name(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_delete_task_from_project
    pub fn storage_delete_task_from_project(&self, task_id: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_delete_task_from_project(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_ensure_folder_structure
    pub fn storage_ensure_folder_structure(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_ensure_folder_structure().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_ai_tasks
    pub fn storage_get_ai_tasks(&self) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_all_active_tasks
    pub fn storage_get_all_active_tasks(&self) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_all_completed_tasks
    pub fn storage_get_all_completed_tasks(&self) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_collaborative_tasks
    pub fn storage_get_collaborative_tasks(&self) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_human_tasks
    pub fn storage_get_human_tasks(&self) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_project_stats
    pub fn storage_get_project_stats(&self, project_name: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_project_stats(project_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_registration_info
    pub fn storage_get_registration_info(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_registration_info().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_storage_dir
    pub fn storage_get_storage_dir(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_storage_dir().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_get_task_from_any_project
    pub fn storage_get_task_from_any_project(&self, task_id: &str) -> Result<JsTask, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_get_task_from_any_project(task_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_init
    pub fn storage_init(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_init().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_is_registered
    pub fn storage_is_registered(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_is_registered().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_list_backups
    pub fn storage_list_backups(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_list_backups().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_list_projects
    pub fn storage_list_projects(&self) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_list_projects().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_load_config
    pub fn storage_load_config(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_load_config().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_load_project
    pub fn storage_load_project(&self, name: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_load_project(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_load_task_collection
    pub fn storage_load_task_collection(&self, name: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_load_task_collection(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_register_with_server
    pub fn storage_register_with_server(&self, server_url: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_register_with_server(server_url).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_restore_backup
    pub fn storage_restore_backup(&self, backup_name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_restore_backup(backup_name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_save_config
    pub fn storage_save_config(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_save_config().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_save_project
    pub fn storage_save_project(&self, name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_save_project(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_save_task_collection
    pub fn storage_save_task_collection(&self, name: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_save_task_collection(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // storage_search_tasks_semantic
    pub fn storage_search_tasks_semantic(&self, query: &str, #[magnus(default = None)] max_results: Option<u32>) -> Result<Vec<JsTask>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::storage_search_tasks_semantic(query, max_results.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // strategy_content
    pub fn strategy_content(&self, #[magnus(default = None)] content: Option<String>, #[magnus(default = None)] file_path: Option<String>, output_format: &str, human: bool) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::strategy_content(content.as_deref(), file_path.as_deref(), output_format, human).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // strategy_content_analysis
    pub fn strategy_content_analysis(&self, text: &str, #[magnus(default = None)] format: Option<String>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::strategy_content_analysis(text, format.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // strike_cluster
    pub fn strike_cluster(&self, _embedding: &str) -> Result<std::result::Result<i32, Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::strike_cluster(_embedding).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // strike_tags
    pub fn strike_tags(&self, _features: &str) -> Result<std::result::Result<Vec<f32>, Box<dyn std::error::Error>>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::strike_tags(_features).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // success
    pub fn success(&self, output: &str, execution_time_ms: u64) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::success(output, execution_time_ms).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // suggest_tags
    pub fn suggest_tags(&self, content_id: &str, top_k: usize) -> Result<Vec<String>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::suggest_tags(content_id, top_k).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tags
    pub fn tags(&self, tags: Vec<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tags(tags).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tags_advanced_search
    pub fn tags_advanced_search(&self, query: &str) -> Result<Vec<JsTag>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tags_advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // task
    pub fn task(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::task(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tasks
    pub fn tasks(&self, project_name: &str) -> Result<Vec<Task>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Projects::tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tdz_cnt
    pub fn tdz_cnt(&self, content: &str, #[magnus(default = None)] session_id: Option<&str>) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tdz_cnt(content, session_id.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tdz_execute_command
    pub fn tdz_execute_command(&self, command: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tdz_execute_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tdz_find
    pub fn tdz_find(&self, query: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Find::tdz_find(query).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tdz_parse_command
    pub fn tdz_parse_command(&self, input: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tdz_parse_command(input).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tdzfp
    pub fn tdzfp(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tdzfp().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // team_percentage
    pub fn team_percentage(&self) -> Result<f64, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::team_percentage().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // term
    pub fn term(&self, term: MemoryTerm) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::term(term).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // title
    pub fn title(&self) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::title().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // to_context_string
    pub fn to_context_string(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::to_context_string().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // to_ollama_format
    pub fn to_ollama_format(&self) -> Result<serde_json::Value, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::to_ollama_format().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // to_state_string
    pub fn to_state_string(&self) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::to_state_string().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // todozi_begin
    pub fn todozi_begin(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::todozi_begin().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // todozi_init
    pub fn todozi_init(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::todozi_init().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // todozi_init_with_auto_registration
    pub fn todozi_init_with_auto_registration(&self) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::todozi_init_with_auto_registration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // tool_count
    pub fn tool_count(&self) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::tool_count().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // total_results
    pub fn total_results(&self) -> Result<usize, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::total_results().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // track_embedding_drift
    pub fn track_embedding_drift(&self, content_id: &str, current_text: &str) -> Result<DriftReport, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::track_embedding_drift(content_id, current_text).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // transform_shorthand_tags
    pub fn transform_shorthand_tags(&self, message: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::transform_shorthand_tags(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // types
    pub fn types(&self) -> Result<&'static str, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::types().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // unregister
    pub fn unregister(&self, name: &str) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::unregister(name).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update
    pub fn update(&self, updates: TaskUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update(updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_agent
    pub fn update_agent(&self, agent_id: &str, updates: AgentUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_agent(agent_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_agent_assignment_status
    pub fn update_agent_assignment_status(&self, agent_id: &str, task_id: &str, status: AssignmentStatus) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_agent_assignment_status(agent_id, task_id, status).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_agent_status
    pub fn update_agent_status(&self, agent_id: &str, status: AgentStatus) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_agent_status(agent_id, status).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_chunk_code
    pub fn update_chunk_code(&self, chunk_id: &str, code: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_chunk_code(chunk_id, code).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_chunk_tests
    pub fn update_chunk_tests(&self, chunk_id: &str, tests: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_chunk_tests(chunk_id, tests).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_config
    pub fn update_config(&self, config: Config) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_config(config).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_config_with_registration
    pub fn update_config_with_registration(&self, registration: RegistrationInfo) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_config_with_registration(registration).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_feeling
    pub fn update_feeling(&self, feeling: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_feeling(feeling).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_idea
    pub fn update_idea(&self, idea_id: &str, updates: IdeaUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_idea(idea_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_memory
    pub fn update_memory(&self, memory_id: &str, updates: MemoryUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_memory(memory_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_project
    pub fn update_project(&self, project: Project) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_project(project).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_registration_api_key
    pub fn update_registration_api_key(&self, api_key: &str) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_registration_api_key(api_key).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_registration_keys
    pub fn update_registration_keys(&self, api_key: &str, #[magnus(default = None)] user_id: Option<String>, #[magnus(default = None)] fingerprint: Option<String>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_registration_keys(api_key, user_id.as_deref(), fingerprint.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_reminder
    pub fn update_reminder(&self, reminder_id: &str, updates: ReminderUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_reminder(reminder_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_summary
    pub fn update_summary(&self, summary_id: &str, updates: SummaryUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_summary(summary_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_tag
    pub fn update_tag(&self, tag_id: &str, updates: TagUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_tag(tag_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_task
    pub fn update_task(&self, id: &str, updates: TaskUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_task(id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_task_full
    pub fn update_task_full(&self, task_id: &str, updates: TaskUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_task_full(task_id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_task_in_project
    pub fn update_task_in_project(&self, id: &str, updates: crate::models::TaskUpdate) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_task_in_project(id, updates).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // update_task_status
    pub fn update_task_status(&self, task_id: &str, status: Status) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::update_task_status(task_id, status).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // urgent
    pub fn urgent(&self, action: &str) -> Result<String, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Tdz::urgent(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validate_embeddings
    pub fn validate_embeddings(&self) -> Result<ValidationReport, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validate_embeddings().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validate_migration
    pub fn validate_migration(&self) -> Result<bool, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validate_migration().await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validate_required_params
    pub fn validate_required_params(&self, kwargs: &str, serde_json: :Value>, required_params: &str) -> Result<Option<ToolResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validate_required_params(kwargs, serde_json, required_params).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validate_string_param
    pub fn validate_string_param(&self, value: &str, param_name: &str, min_length: usize, max_length: usize, #[magnus(default = None)] pattern: Option<&str>) -> Result<Option<ToolResult>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validate_string_param(value, param_name, min_length, max_length, pattern.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validate_task_input
    pub fn validate_task_input(&self, action: &str, time: &str, priority: &str, project: &str, status: &str, #[magnus(default = None)] assignee: Option<&str>, #[magnus(default = None)] progress: Option<u8>) -> Result<(), MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validate_task_input(action, time, priority, project, status, assignee.as_deref(), progress.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validate_tdz_command
    pub fn validate_tdz_command(&self, command: &str) -> Result<JsCommandValidation, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validate_tdz_command(command).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // validation
    pub fn validation(&self, message: impl Into<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::validation(message).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // verbose
    pub fn verbose(&self, verbose: bool) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::verbose(verbose).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_action
    pub fn with_action(&self, action: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_action(action).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_assignee
    pub fn with_assignee(&self, assignee: Assignee) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_assignee(assignee).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_context
    pub fn with_context(&self, context: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_context(context).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_context_notes
    pub fn with_context_notes(&self, context_notes: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_context_notes(context_notes).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_dependencies
    pub fn with_dependencies(&self, dependencies: Vec<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_dependencies(dependencies).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_dry_run
    pub fn with_dry_run(&self, dry_run: bool) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_dry_run(dry_run).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_embedding_service
    pub fn with_embedding_service(&self, service: TodoziEmbeddingService) -> Result<PyResult<Self>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_embedding_service(service).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_embedding_service_option
    pub fn with_embedding_service_option(&self, #[magnus(default = None)] service: Option<TodoziEmbeddingService>) -> Result<PyResult<Self>, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_embedding_service_option(service.as_deref()).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_force
    pub fn with_force(&self, force: bool) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_force(force).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_max_tokens
    pub fn with_max_tokens(&self, max_tokens: u32) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_max_tokens(max_tokens).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_parent_project
    pub fn with_parent_project(&self, parent_project: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_parent_project(parent_project).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_priority
    pub fn with_priority(&self, priority: Priority) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_priority(priority).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_progress
    pub fn with_progress(&self, progress: u8) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_progress(progress).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_shared_components
    pub fn with_shared_components(&self, config: Arc<Mutex<TodoziEmbeddingConfig>>, cache: Arc<Mutex<HashMap<String, embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>, embedding_models: Arc<Mutex<HashMap<String, tag_manager: Arc<Mutex<TagManager>>, storage: Arc<Mutex<Storage>>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_shared_components(config, cache, embedding_model, embedding_models, tag_manager, storage).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_status
    pub fn with_status(&self, status: Status) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_status(status).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_tags
    pub fn with_tags(&self, tags: Vec<String>) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_tags(tags).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_temperature
    pub fn with_temperature(&self, temperature: f32) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_temperature(temperature).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_time
    pub fn with_time(&self, time: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_time(time).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_user_id
    pub fn with_user_id(&self, user_id: &str) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_user_id(user_id).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
    // with_verbose
    pub fn with_verbose(&self, verbose: bool) -> Result<Self, MagnusError> {
        let runtime = get_runtime()?;
        runtime.block_on(async {
            Done::with_verbose(verbose).await
                .map_err(|e| MagnusError::new(runtime_error(), format!("{}", e)))
        })
    }
    
}

// Standalone Ruby functions (module-level functions)
// activate_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_activate_api_key(user_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::activate_api_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// activate_key
#[magnus::wrap(class = "Todozi")]
fn todozi_activate_key(user_id: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::activate_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// activate_reminder
#[magnus::wrap(class = "Todozi")]
fn todozi_activate_reminder(reminder_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::activate_reminder(reminder_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// active
#[magnus::wrap(class = "Todozi")]
fn todozi_active() -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::active().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// active_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_active_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::active_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add
#[magnus::wrap(class = "Todozi")]
fn todozi_add(task_name: &str, description: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add(task_name, description).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_checklist_item
#[magnus::wrap(class = "Todozi")]
fn todozi_add_checklist_item(item: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_checklist_item(item).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_chunk
#[magnus::wrap(class = "Todozi")]
fn todozi_add_chunk(chunk_id: &str, level: &str, deps: Vec<String>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_chunk(chunk_id, level, deps).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_completed_module
#[magnus::wrap(class = "Todozi")]
fn todozi_add_completed_module(module: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_completed_module(module).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_dependency
#[magnus::wrap(class = "Todozi")]
fn todozi_add_dependency(dep: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_dependency(dep).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_error_pattern
#[magnus::wrap(class = "Todozi")]
fn todozi_add_error_pattern(pattern: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_error_pattern(pattern).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_function_signature
#[magnus::wrap(class = "Todozi")]
fn todozi_add_function_signature(name: &str, signature: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_function_signature(name, signature).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_import
#[magnus::wrap(class = "Todozi")]
fn todozi_add_import(import_stmt: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_import(import_stmt).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_item
#[magnus::wrap(class = "Todozi")]
fn todozi_add_item(content: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_item(content).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_key
#[magnus::wrap(class = "Todozi")]
fn todozi_add_key(key: JsApiKey) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_key(key).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_pending_module
#[magnus::wrap(class = "Todozi")]
fn todozi_add_pending_module(module: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_pending_module(module).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_processed_action
#[magnus::wrap(class = "Todozi")]
fn todozi_add_processed_action(action_type: &str, description: &str, success: bool, result: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_processed_action(action_type, description, success, result.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_queue_item
#[magnus::wrap(class = "Todozi")]
fn todozi_add_queue_item(item: QueueItem) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_queue_item(item).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_recent
#[magnus::wrap(class = "Todozi")]
fn todozi_add_recent(description: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_recent(description).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_recent_action
#[magnus::wrap(class = "Todozi")]
fn todozi_add_recent_action(action: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_recent_action(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_tag_relationship
#[magnus::wrap(class = "Todozi")]
fn todozi_add_tag_relationship(tag_id: &str, related_tag_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_tag_relationship(tag_id, related_tag_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_tag_to_task
#[magnus::wrap(class = "Todozi")]
fn todozi_add_tag_to_task(task_id: &str, tag: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_tag_to_task(task_id, tag).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_task
#[magnus::wrap(class = "Todozi")]
fn todozi_add_task(task: Task) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_task(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_task_emb
#[magnus::wrap(class = "Todozi")]
fn todozi_add_task_emb(task: JsTask) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_task_emb(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_task_to_project
#[magnus::wrap(class = "Todozi")]
fn todozi_add_task_to_project(task: Task) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::add_task_to_project(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// add_to_task
#[magnus::wrap(class = "Todozi")]
fn todozi_add_to_task(task_id: &str, tag: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::add_to_task(task_id, tag).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// advanced_search
#[magnus::wrap(class = "Todozi")]
fn todozi_advanced_search(query: &str) -> Result<Vec<Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// advanced_search_with_filters
#[magnus::wrap(class = "Todozi")]
fn todozi_advanced_search_with_filters(query: &str, data_types: Option<Vec<String>>, limit: Option<u32>) -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::advanced_search_with_filters(query, data_types.as_deref(), limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ai
#[magnus::wrap(class = "Todozi")]
fn todozi_ai(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::ai(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ai_find
#[magnus::wrap(class = "Todozi")]
fn todozi_ai_find(query: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::ai_find(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ai_search
#[magnus::wrap(class = "Todozi")]
fn todozi_ai_search(query: &str) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::ai_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ai_task
#[magnus::wrap(class = "Todozi")]
fn todozi_ai_task(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::ai_task(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ai_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_ai_tasks(query: &str) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::ai_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// all
#[magnus::wrap(class = "Todozi")]
fn todozi_all() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::all().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// all_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_all_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::all_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// analyze_code_quality
#[magnus::wrap(class = "Todozi")]
fn todozi_analyze_code_quality(_features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::analyze_code_quality(_features).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// api
#[magnus::wrap(class = "Todozi")]
fn todozi_api(message: impl Into<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::api(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_api_key() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::api_key().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// archive_project
#[magnus::wrap(class = "Todozi")]
fn todozi_archive_project(name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::archive_project(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// as_str
#[magnus::wrap(class = "Todozi")]
fn todozi_as_str() -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::as_str().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// assign_task_to_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_assign_task_to_agent(task_id: &str, agent_id: &str, project_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::assign_task_to_agent(task_id, agent_id, project_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// auto_categorize_content
#[magnus::wrap(class = "Todozi")]
fn todozi_auto_categorize_content(text: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::auto_categorize_content(text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// auto_label_clusters
#[magnus::wrap(class = "Todozi")]
fn todozi_auto_label_clusters(clusters: Vec<ClusteringResult>) -> Result<Vec<LabeledCluster>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::auto_label_clusters(clusters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// backlog
#[magnus::wrap(class = "Todozi")]
fn todozi_backlog() -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::backlog().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// backup_embeddings
#[magnus::wrap(class = "Todozi")]
fn todozi_backup_embeddings(backup_path: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::backup_embeddings(backup_path.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// batch_embed_content
#[magnus::wrap(class = "Todozi")]
fn todozi_batch_embed_content(content_list: Vec<String>) -> Result<Vec<Vec<f64>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::batch_embed_content(content_list).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// begin
#[magnus::wrap(class = "Todozi")]
fn todozi_begin(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Actions::begin(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// breakthrough
#[magnus::wrap(class = "Todozi")]
fn todozi_breakthrough(idea: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::breakthrough(idea).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// breakthrough_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_breakthrough_idea(idea: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::breakthrough_idea(idea).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// breakthrough_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_breakthrough_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::breakthrough_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// build_similarity_graph
#[magnus::wrap(class = "Todozi")]
fn todozi_build_similarity_graph(threshold: f32) -> Result<SimilarityGraph, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::build_similarity_graph(threshold).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// bulk_create_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_bulk_create_tags(tag_names: Vec<String>, category: Option<String>) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::bulk_create_tags(tag_names, category.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// calculate_diversity
#[magnus::wrap(class = "Todozi")]
fn todozi_calculate_diversity(content_ids: Vec<String>) -> Result<f32, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::calculate_diversity(content_ids).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// capabilities
#[magnus::wrap(class = "Todozi")]
fn todozi_capabilities(capabilities: Vec<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::capabilities(capabilities).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// category
#[magnus::wrap(class = "Todozi")]
fn todozi_category(category: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::category(category).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// chat
#[magnus::wrap(class = "Todozi")]
fn todozi_chat(message: &str) -> Result<ChatContent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::chat(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// check_api_key_auth
#[magnus::wrap(class = "Todozi")]
fn todozi_check_api_key_auth(public_key: &str, private_key: Option<String>) -> Result<JsApiKeyAuth, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::check_api_key_auth(public_key, private_key.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// check_folder_structure
#[magnus::wrap(class = "Todozi")]
fn todozi_check_folder_structure() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::check_folder_structure().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// check_migration_status
#[magnus::wrap(class = "Todozi")]
fn todozi_check_migration_status() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::check_migration_status().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cleanup_expired
#[magnus::wrap(class = "Todozi")]
fn todozi_cleanup_expired() -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cleanup_expired().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cleanup_legacy
#[magnus::wrap(class = "Todozi")]
fn todozi_cleanup_legacy() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cleanup_legacy().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// clear_registration
#[magnus::wrap(class = "Todozi")]
fn todozi_clear_registration() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::clear_registration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_add_task
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_add_task(content: &str, priority: Option<String>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_add_task(content, priority.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_chat
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_chat(message: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_chat(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_clear_registration
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_clear_registration() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_clear_registration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_complete_task
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_complete_task(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_complete_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_create_backup
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_create_backup() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_create_backup().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_create_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_create_idea(content: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_create_idea(content).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_create_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_create_memory(moment: &str, meaning: &str, reason: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_create_memory(moment, meaning, reason).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_delete_task
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_delete_task(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_delete_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_fix_consistency
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_fix_consistency() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_fix_consistency().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_get_registration_status
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_get_registration_status() -> Result<Option<JsRegistrationInfo>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_get_registration_status().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_list_backups
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_list_backups() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_list_backups().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_list_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_list_tasks() -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_list_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_register_with_server
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_register_with_server(server_url: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_register_with_server(server_url).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_restore_backup
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_restore_backup(backup_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_restore_backup(backup_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_search_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_search_tasks(query: &str) -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_search_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_show_task
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_show_task(task_id: &str) -> Result<JsTask, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_show_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cli_update_task
#[magnus::wrap(class = "Todozi")]
fn todozi_cli_update_task(task_id: &str, action: Option<String>, priority: Option<String>, status: Option<String>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cli_update_task(task_id, action.as_deref(), priority.as_deref(), status.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cluster
#[magnus::wrap(class = "Todozi")]
fn todozi_cluster() -> Result<Vec<ClusteringResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cluster().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cluster_content
#[magnus::wrap(class = "Todozi")]
fn todozi_cluster_content() -> Result<Vec<ClusteringResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cluster_content().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// cluster_similar_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_cluster_similar_tasks() -> Result<Vec<JsClusteringResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::cluster_similar_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// collab
#[magnus::wrap(class = "Todozi")]
fn todozi_collab(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::collab(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// collab_task
#[magnus::wrap(class = "Todozi")]
fn todozi_collab_task(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::collab_task(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// color
#[magnus::wrap(class = "Todozi")]
fn todozi_color(color: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::color(color).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// compare_models
#[magnus::wrap(class = "Todozi")]
fn todozi_compare_models(text: &str, model_aliases: Vec<String>) -> Result<ModelComparisonResult, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::compare_models(text, model_aliases).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// complete
#[magnus::wrap(class = "Todozi")]
fn todozi_complete(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// complete_agent_assignment
#[magnus::wrap(class = "Todozi")]
fn todozi_complete_agent_assignment(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete_agent_assignment(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// complete_task
#[magnus::wrap(class = "Todozi")]
fn todozi_complete_task(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// complete_task_in_project
#[magnus::wrap(class = "Todozi")]
fn todozi_complete_task_in_project(id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::complete_task_in_project(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// completion_rate
#[magnus::wrap(class = "Todozi")]
fn todozi_completion_rate() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::completion_rate().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// config
#[magnus::wrap(class = "Todozi")]
fn todozi_config() -> Result<&Config, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::config().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// configure_display
#[magnus::wrap(class = "Todozi")]
fn todozi_configure_display(config_json: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::configure_display(config_json).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// configure_server
#[magnus::wrap(class = "Todozi")]
fn todozi_configure_server(host: Option<String>, port: Option<u32>, max_connections: Option<u32>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::configure_server(host.as_deref(), port.as_deref(), max_connections.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// content
#[magnus::wrap(class = "Todozi")]
fn todozi_content(content: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::content(content).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// context
#[magnus::wrap(class = "Todozi")]
fn todozi_context(context: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::context(context).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// craft_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_craft_embedding(_features: &str) -> Result<std::result::Result<Vec<f32>, Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::craft_embedding(_features).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create
#[magnus::wrap(class = "Todozi")]
fn todozi_create(name: &str, description: Option<&str>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create(name, description.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_advanced_todozi_tools
#[magnus::wrap(class = "Todozi")]
fn todozi_create_advanced_todozi_tools(todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_advanced_todozi_tools(todozi).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_agent(agent: Agent) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_agent(agent).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_create_api_key() -> Result<JsApiKey, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_api_key().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_api_key_with_user_id
#[magnus::wrap(class = "Todozi")]
fn todozi_create_api_key_with_user_id(user_id: &str) -> Result<JsApiKey, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_api_key_with_user_id(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_architect_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_architect_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_architect_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_backup
#[magnus::wrap(class = "Todozi")]
fn todozi_create_backup() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_backup().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_coder
#[magnus::wrap(class = "Todozi")]
fn todozi_create_coder() -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_coder().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_comrad_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_comrad_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_comrad_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_custom_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_custom_agent(id: &str, name: &str, description: &str, capabilities: Vec<String>, specializations: Vec<String>, category: &str, author: Option<String>) -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_custom_agent(id, name, description, capabilities, specializations, category, author.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_default_agents
#[magnus::wrap(class = "Todozi")]
fn todozi_create_default_agents() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_default_agents().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_designer_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_designer_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_designer_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_detective_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_detective_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_detective_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_devops_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_devops_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_devops_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_embedding_service
#[magnus::wrap(class = "Todozi")]
fn todozi_create_embedding_service() -> Result<TodoziEmbeddingService, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_embedding_service().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_embedding_version
#[magnus::wrap(class = "Todozi")]
fn todozi_create_embedding_version(content_id: &str, version_label: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_embedding_version(content_id, version_label).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_error
#[magnus::wrap(class = "Todozi")]
fn todozi_create_error(error: Error) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_error(error).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_error_result
#[magnus::wrap(class = "Todozi")]
fn todozi_create_error_result(error_msg: &str, execution_time_ms: u64, error_type: ErrorType, metadata: Option<HashMap<Strin>, serde_json: :Value>>) -> Result<ToolResult, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_error_result(error_msg, execution_time_ms, error_type, metadata.as_deref(), serde_json).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_filters
#[magnus::wrap(class = "Todozi")]
fn todozi_create_filters() -> Result<TaskFilters, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_filters().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_finisher_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_finisher_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_finisher_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_framer_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_framer_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_framer_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_friend_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_friend_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_friend_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_grok_level_todozi_tools
#[magnus::wrap(class = "Todozi")]
fn todozi_create_grok_level_todozi_tools(todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_grok_level_todozi_tools(todozi).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_hoarder_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_hoarder_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_hoarder_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_create_idea(idea: Idea) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_idea(idea).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_investigator_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_investigator_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_investigator_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_mason_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_mason_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_mason_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_create_memory(memory: Memory) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_memory(memory).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_nerd_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_nerd_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_nerd_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_nun_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_nun_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_nun_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_overlord_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_overlord_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_overlord_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_party_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_party_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_party_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_planner_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_planner_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_planner_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_project
#[magnus::wrap(class = "Todozi")]
fn todozi_create_project(name: &str, description: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::create_project(name, description.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_recycler_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_recycler_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_recycler_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_reminder
#[magnus::wrap(class = "Todozi")]
fn todozi_create_reminder(reminder: Reminder) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_reminder(reminder).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_search_options
#[magnus::wrap(class = "Todozi")]
fn todozi_create_search_options(data_types: Option<Vec<String>>, limit: Option<u32>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_search_options(data_types.as_deref(), limit.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_skeleton_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_skeleton_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_skeleton_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_snitch_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_snitch_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_snitch_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_storage
#[magnus::wrap(class = "Todozi")]
fn todozi_create_storage() -> Result<Storage, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_storage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_success_result
#[magnus::wrap(class = "Todozi")]
fn todozi_create_success_result(output: &str, execution_time_ms: u64, metadata: Option<HashMap<Strin>, serde_json: :Value>>) -> Result<ToolResult, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_success_result(output, execution_time_ms, metadata.as_deref(), serde_json).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_summary
#[magnus::wrap(class = "Todozi")]
fn todozi_create_summary(summary: Summary) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_summary(summary).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tag(tag: Tag) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::create_tag(tag).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_task
#[magnus::wrap(class = "Todozi")]
fn todozi_create_task(action: &str, priority: Option<Priority>, project: Option<&str>, time: Option<&str>, context: Option<&str>) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_task(action, priority.as_deref(), project.as_deref(), time.as_deref(), context.as_deref()).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_task_filters
#[magnus::wrap(class = "Todozi")]
fn todozi_create_task_filters(project: Option<String>, status: Option<String>, priority: Option<String>, assignee: Option<String>, tags: Option<String>, search: Option<String>) -> Result<TaskFilters, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_task_filters(project.as_deref(), status.as_deref(), priority.as_deref(), assignee.as_deref(), tags.as_deref(), search.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tdz_content_processor_tool
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tdz_content_processor_tool(state: SharedTodoziState) -> Result<Box<dyn Tool + Send + Sync>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tdz_content_processor_tool(state).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tdz_tool
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tdz_tool() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tdz_tool().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tester_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tester_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tester_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_todozi_tools
#[magnus::wrap(class = "Todozi")]
fn todozi_create_todozi_tools(todozi: SharedTodozi) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_todozi_tools(todozi).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_todozi_tools_with_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_create_todozi_tools_with_embedding(todozi: SharedTodozi, embedding_service: Option<TodoziEmbeddingService>) -> Result<PyResult<Vec<Box<dyn Tool + Send + Sync>>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_todozi_tools_with_embedding(todozi, embedding_service.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tool_definition
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tool_definition(name: &str, description: &str, category: &str, parameters: Vec<JsToolParameter>) -> Result<JsToolDefinition, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_definition(name, description, category, parameters).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tool_definition_with_locks
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tool_definition_with_locks(name: &str, description: &str, category: &str, parameters: Vec<ToolParameter>, resource_locks: Vec<ResourceLock>) -> Result<ToolDefinition, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_definition_with_locks(name, description, category, parameters, resource_locks).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tool_parameter
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tool_parameter(name: &str, type_: &str, description: &str, required: bool) -> Result<JsToolParameter, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_parameter(name, type_, description, required).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tool_parameter_with_default
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tool_parameter_with_default(name: &str, type_: &str, description: &str, required: bool, default: &str) -> Result<JsToolParameter, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tool_parameter_with_default(name, type_, description, required, default).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tui_app
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tui_app() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tui_app().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_tuner_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_tuner_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_tuner_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_update
#[magnus::wrap(class = "Todozi")]
fn todozi_create_update() -> Result<TaskUpdate, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_update().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// create_writer_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_create_writer_agent() -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::create_writer_agent().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// critical_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_critical_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::critical_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// deactivate_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_deactivate_api_key(user_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deactivate_api_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// deactivate_key
#[magnus::wrap(class = "Todozi")]
fn todozi_deactivate_key(user_id: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deactivate_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// deep
#[magnus::wrap(class = "Todozi")]
fn todozi_deep(query: &str) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::deep(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// deep_search
#[magnus::wrap(class = "Todozi")]
fn todozi_deep_search(query: &str) -> Result<Vec<JsSimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::deep_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// default_filters
#[magnus::wrap(class = "Todozi")]
fn todozi_default_filters() -> Result<TaskFilters, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::default_filters().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// default_update
#[magnus::wrap(class = "Todozi")]
fn todozi_default_update() -> Result<TaskUpdate, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::default_update().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete
#[magnus::wrap(class = "Todozi")]
fn todozi_delete(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_agent(agent_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_agent(agent_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_agent_assignment
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_agent_assignment(agent_id: &str, task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_agent_assignment(agent_id, task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_code_chunk
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_code_chunk(chunk_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_code_chunk(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_error
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_error(error_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_error(error_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_feeling
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_feeling(id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_feeling(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_idea(idea_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_idea(idea_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_memory(memory_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_memory(memory_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_project
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_project(project_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_project(project_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_project_task_container
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_project_task_container(project_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_project_task_container(project_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_reminder
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_reminder(reminder_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_reminder(reminder_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_summary
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_summary(summary_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_summary(summary_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_tag(tag_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_tag(tag_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_task
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_task(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_task_from_project
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_task_from_project(id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_task_from_project(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// delete_training_data
#[magnus::wrap(class = "Todozi")]
fn todozi_delete_training_data(training_data_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::delete_training_data(training_data_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// description
#[magnus::wrap(class = "Todozi")]
fn todozi_description(description: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::description(description).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// detailed
#[magnus::wrap(class = "Todozi")]
fn todozi_detailed() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Stats::detailed().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// detailed_stats
#[magnus::wrap(class = "Todozi")]
fn todozi_detailed_stats() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::detailed_stats().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// display_task
#[magnus::wrap(class = "Todozi")]
fn todozi_display_task(task_id: &str) -> Result<TaskDisplay, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::display_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// display_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_display_tasks(task_ids: Vec<String>) -> Result<TaskListDisplay, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::display_tasks(task_ids).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// do_it
#[magnus::wrap(class = "Todozi")]
fn todozi_do_it(what: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Easy::do_it(what).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done
#[magnus::wrap(class = "Todozi")]
fn todozi_done(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::done(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_done_api_key() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_api_key().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_create_embedding_service
#[magnus::wrap(class = "Todozi")]
fn todozi_done_create_embedding_service() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_create_embedding_service().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_create_storage
#[magnus::wrap(class = "Todozi")]
fn todozi_done_create_storage() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_create_storage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_default_filters
#[magnus::wrap(class = "Todozi")]
fn todozi_done_default_filters() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_default_filters().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_default_update
#[magnus::wrap(class = "Todozi")]
fn todozi_done_default_update() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_default_update().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_embedding_config
#[magnus::wrap(class = "Todozi")]
fn todozi_done_embedding_config() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_embedding_config().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_embedding_service
#[magnus::wrap(class = "Todozi")]
fn todozi_done_embedding_service() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_embedding_service().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_init
#[magnus::wrap(class = "Todozi")]
fn todozi_done_init() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_init().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_process_chat
#[magnus::wrap(class = "Todozi")]
fn todozi_done_process_chat(message: &str, user_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_process_chat(message, user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_sample_task
#[magnus::wrap(class = "Todozi")]
fn todozi_done_sample_task() -> Result<JsTask, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_sample_task().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_storage
#[magnus::wrap(class = "Todozi")]
fn todozi_done_storage() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_storage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// done_types
#[magnus::wrap(class = "Todozi")]
fn todozi_done_types() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::done_types().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// dry_run
#[magnus::wrap(class = "Todozi")]
fn todozi_dry_run(dry_run: bool) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::dry_run(dry_run).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// easy_done
#[magnus::wrap(class = "Todozi")]
fn todozi_easy_done(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_done(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// easy_find
#[magnus::wrap(class = "Todozi")]
fn todozi_easy_find(what: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_find(what).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// easy_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_easy_idea(what: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_idea(what).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// easy_remember
#[magnus::wrap(class = "Todozi")]
fn todozi_easy_remember(what: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::easy_remember(what).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embed
#[magnus::wrap(class = "Todozi")]
fn todozi_embed(text: &str) -> Result<Vec<f32>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed(text).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embed_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_embed_idea(idea: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_idea(idea).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embed_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_embed_memory(memory: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_memory(memory).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embed_stats
#[magnus::wrap(class = "Todozi")]
fn todozi_embed_stats() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_stats().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embed_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_embed_tag(tag: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embed_tag(tag).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embed_task
#[magnus::wrap(class = "Todozi")]
fn todozi_embed_task(task_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Emb::embed_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embedding_config
#[magnus::wrap(class = "Todozi")]
fn todozi_embedding_config() -> Result<TodoziEmbeddingConfig, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embedding_config().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// embedding_service
#[magnus::wrap(class = "Todozi")]
fn todozi_embedding_service() -> Result<TodoziEmbeddingService, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::embedding_service().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// encode
#[magnus::wrap(class = "Todozi")]
fn todozi_encode(texts: &str) -> Result<Vec<Vec<f32>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::encode(texts).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// end_queue_session
#[magnus::wrap(class = "Todozi")]
fn todozi_end_queue_session(session_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::end_queue_session(session_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// end_session
#[magnus::wrap(class = "Todozi")]
fn todozi_end_session(session_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::end_session(session_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ensure_folder_structure
#[magnus::wrap(class = "Todozi")]
fn todozi_ensure_folder_structure() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ensure_folder_structure().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ensure_todozi_initialized
#[magnus::wrap(class = "Todozi")]
fn todozi_ensure_todozi_initialized() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ensure_todozi_initialized().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// error
#[magnus::wrap(class = "Todozi")]
fn todozi_error(error: &str, execution_time_ms: u64) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::error(error, execution_time_ms).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// example
#[magnus::wrap(class = "Todozi")]
fn todozi_example() -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::example().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// example_usage
#[magnus::wrap(class = "Todozi")]
fn todozi_example_usage() -> Result<std::result::Result<(), Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::example_usage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// execute_task
#[magnus::wrap(class = "Todozi")]
fn todozi_execute_task(storage: &str, task: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_task(storage, task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// execute_tdz_command
#[magnus::wrap(class = "Todozi")]
fn todozi_execute_tdz_command(command: &str, base_url: &str, api_key: Option<&str>) -> Result<Value, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_tdz_command(command, base_url, api_key.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// execute_todozi_tool_delegated
#[magnus::wrap(class = "Todozi")]
fn todozi_execute_todozi_tool_delegated(params: &str) -> Result<ExecutorResult<ExecutionResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_todozi_tool_delegated(params).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// execute_tool
#[magnus::wrap(class = "Todozi")]
fn todozi_execute_tool(tool_name: &str, kwargs: HashMap<String, serde_json: :Value>) -> Result<ToolResult, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::execute_tool(tool_name, kwargs, serde_json).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// explain_search_result
#[magnus::wrap(class = "Todozi")]
fn todozi_explain_search_result(query: &str, result: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::explain_search_result(query, result).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// export_diagnostics
#[magnus::wrap(class = "Todozi")]
fn todozi_export_diagnostics() -> Result<DiagnosticReport, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_diagnostics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// export_embedded_tasks_hlx
#[magnus::wrap(class = "Todozi")]
fn todozi_export_embedded_tasks_hlx(output_path: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_embedded_tasks_hlx(output_path).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// export_for_fine_tuning
#[magnus::wrap(class = "Todozi")]
fn todozi_export_for_fine_tuning(output_path: &str) -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_for_fine_tuning(output_path).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// export_to_format
#[magnus::wrap(class = "Todozi")]
fn todozi_export_to_format(format: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::export_to_format(format).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// extract_content
#[magnus::wrap(class = "Todozi")]
fn todozi_extract_content(content: Option<String>, file_path: Option<String>, output_format: &str, human: bool) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_content(content.as_deref(), file_path.as_deref(), output_format, human).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// extract_content_from_text
#[magnus::wrap(class = "Todozi")]
fn todozi_extract_content_from_text(text: &str, format: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_content_from_text(text, format.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// extract_task_actions
#[magnus::wrap(class = "Todozi")]
fn todozi_extract_task_actions(content: &str) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_task_actions(content).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// extract_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_extract_tasks(content: &str, context: Option<&str>) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::extract_tasks(content, context.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// fast
#[magnus::wrap(class = "Todozi")]
fn todozi_fast(query: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::fast(query).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// fast_search
#[magnus::wrap(class = "Todozi")]
fn todozi_fast_search(query: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fast_search(query).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// filtered_semantic_search
#[magnus::wrap(class = "Todozi")]
fn todozi_filtered_semantic_search(query: &str, filters: SearchFilters, limit: usize) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::filtered_semantic_search(query, filters, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find
#[magnus::wrap(class = "Todozi")]
fn todozi_find(query: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::find(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_best_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_find_best_agent(required_specialization: &str, preferred_capability: Option<&str>) -> Result<Option<&Agent>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_best_agent(required_specialization, preferred_capability.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_by_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_find_by_tag(tag_name: &str) -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_by_tag(tag_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_cross_content_relationships
#[magnus::wrap(class = "Todozi")]
fn todozi_find_cross_content_relationships(content_id: &str, content_type: TodoziContentType, min_similarity: f32) -> Result<HashMap<TodoziContentType, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_cross_content_relationships(content_id, content_type, min_similarity).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_find_ideas(query: &str) -> Result<Vec<JsIdea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_ideas(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_find_memories(query: &str) -> Result<Vec<JsMemory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_memories(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_outliers
#[magnus::wrap(class = "Todozi")]
fn todozi_find_outliers(content_type: TodoziContentType, threshold: f32) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_outliers(content_type, threshold).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_similar_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_find_similar_tags(tag_name: &str, limit: Option<usize>) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_similar_tags(tag_name, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_similar_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_find_similar_tasks(task_description: &str, limit: Option<usize>) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_similar_tasks(task_description, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_find_tasks(query: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_tasks_ai
#[magnus::wrap(class = "Todozi")]
fn todozi_find_tasks_ai(query: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_tasks_ai(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_tdz
#[magnus::wrap(class = "Todozi")]
fn todozi_find_tdz(str: Option<&str>) -> Result<Option<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_tdz(str.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// find_todozi
#[magnus::wrap(class = "Todozi")]
fn todozi_find_todozi(str: Option<&str>) -> Result<Option<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::find_todozi(str.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// fix_completed_tasks_consistency
#[magnus::wrap(class = "Todozi")]
fn todozi_fix_completed_tasks_consistency() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fix_completed_tasks_consistency().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// fix_task_consistency
#[magnus::wrap(class = "Todozi")]
fn todozi_fix_task_consistency() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fix_task_consistency().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// force_overwrite
#[magnus::wrap(class = "Todozi")]
fn todozi_force_overwrite(force_overwrite: bool) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::force_overwrite(force_overwrite).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_duration
#[magnus::wrap(class = "Todozi")]
fn todozi_format_duration(from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_duration(from, to).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_error_with_context
#[magnus::wrap(class = "Todozi")]
fn todozi_format_error_with_context(error_message: &str, context: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_error_with_context(error_message, context).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_project_stats
#[magnus::wrap(class = "Todozi")]
fn todozi_format_project_stats(project_name: &str, task_count: usize, completed_count: usize) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_project_stats(project_name, task_count, completed_count).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_task
#[magnus::wrap(class = "Todozi")]
fn todozi_format_task(task: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_task(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_task_list
#[magnus::wrap(class = "Todozi")]
fn todozi_format_task_list(tasks: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_task_list(tasks).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_task_with_emojis
#[magnus::wrap(class = "Todozi")]
fn todozi_format_task_with_emojis(task: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_task_with_emojis(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// format_time_estimate
#[magnus::wrap(class = "Todozi")]
fn todozi_format_time_estimate(time: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::format_time_estimate(time).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// fuzzy_search
#[magnus::wrap(class = "Todozi")]
fn todozi_fuzzy_search(query: &str, max_distance: usize) -> Result<Vec<(&Tag, usize)>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::fuzzy_search(query, max_distance).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// generate_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_generate_embedding(text: &str) -> Result<Vec<f32>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::generate_embedding(text).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// generate_embeddings_batch
#[magnus::wrap(class = "Todozi")]
fn todozi_generate_embeddings_batch(texts: Vec<String>) -> Result<Vec<Vec<f32>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::generate_embeddings_batch(texts).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// generate_task_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_generate_task_embedding(task: &str) -> Result<Option<Vec<f32>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::generate_task_embedding(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get
#[magnus::wrap(class = "Todozi")]
fn todozi_get(task_id: &str) -> Result<Option<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_active_items
#[magnus::wrap(class = "Todozi")]
fn todozi_get_active_items() -> Result<Vec<&QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_active_keys
#[magnus::wrap(class = "Todozi")]
fn todozi_get_active_keys() -> Result<Vec<&ApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_active_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_get_active_reminders() -> Result<Vec<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_active_sessions
#[magnus::wrap(class = "Todozi")]
fn todozi_get_active_sessions() -> Result<Vec<crate::models::QueueSession>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_active_sessions().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agent(agent_id: &str) -> Result<Option<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent(agent_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agent_assignments
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agent_assignments(agent_id: &str) -> Result<Vec<&AgentAssignment>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent_assignments(agent_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agent_assignments_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agent_assignments_dir(agent_id: &str) -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent_assignments_dir(agent_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agent_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agent_statistics() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agent_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agents_by_capability
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agents_by_capability(capability: &str) -> Result<Vec<&Agent>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_by_capability(capability).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agents_by_specialization
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agents_by_specialization(specialization: &str) -> Result<Vec<&Agent>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_by_specialization(specialization).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agents_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agents_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_agents_with_assignments
#[magnus::wrap(class = "Todozi")]
fn todozi_get_agents_with_assignments() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_agents_with_assignments().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_ai_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_ai_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_active_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_active_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_agents
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_agents() -> Result<Vec<&Agent>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_categories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_categories() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_categories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_completed_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_completed_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_ideas() -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_all_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_items
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_items() -> Result<Vec<&QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_keys
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_keys() -> Result<Vec<&ApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_memories() -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_reminders() -> Result<Vec<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_summaries
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_summaries() -> Result<Vec<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_summaries().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_tags() -> Result<Vec<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_all_tags().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_tasks() -> Result<Vec<&Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_all_tools
#[magnus::wrap(class = "Todozi")]
fn todozi_get_all_tools() -> Result<Vec<&Box<dyn Tool>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_all_tools().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_get_api_key(user_id: &str) -> Result<JsApiKey, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_api_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_api_key_by_public
#[magnus::wrap(class = "Todozi")]
fn todozi_get_api_key_by_public(public_key: &str) -> Result<JsApiKey, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_api_key_by_public(public_key).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_assignee_emoji
#[magnus::wrap(class = "Todozi")]
fn todozi_get_assignee_emoji(assignee: &str) -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_assignee_emoji(assignee).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_assignments_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_assignments_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_assignments_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_available_agents
#[magnus::wrap(class = "Todozi")]
fn todozi_get_available_agents() -> Result<Vec<Agent>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_available_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_backlog_items
#[magnus::wrap(class = "Todozi")]
fn todozi_get_backlog_items() -> Result<Vec<&QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_backlog_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_breakthrough_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_get_breakthrough_ideas() -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_breakthrough_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunk
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunk(chunk_id: &str) -> Result<Option<&CodeChunk>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunk(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunk_mut
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunk_mut(chunk_id: &str) -> Result<Option<&mut CodeChunk>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunk_mut(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunk_status
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunk_status(chunk_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunk_status(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunking_level_info
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunking_level_info(level: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunking_level_info(level).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunking_levels
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunking_levels() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunking_levels().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunks_by_level
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunks_by_level(level: ChunkingLevel) -> Result<Vec<&CodeChunk>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunks_by_level(level).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_chunks_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_chunks_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_chunks_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_collaborative_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_collaborative_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_command_documentation
#[magnus::wrap(class = "Todozi")]
fn todozi_get_command_documentation(command_name: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_command_documentation(command_name.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_complete_items
#[magnus::wrap(class = "Todozi")]
fn todozi_get_complete_items() -> Result<Vec<&QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_complete_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_critical_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_critical_memories() -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_critical_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_current_duration
#[magnus::wrap(class = "Todozi")]
fn todozi_get_current_duration() -> Result<u64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_current_duration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_default_model
#[magnus::wrap(class = "Todozi")]
fn todozi_get_default_model() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_default_model().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_dependency_chain
#[magnus::wrap(class = "Todozi")]
fn todozi_get_dependency_chain(chunk_id: &str) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_dependency_chain(chunk_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_embedding_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_embedding_statistics() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_embedding_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_emotional_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_emotional_memories(emotion: &str) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_emotional_memories(emotion).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_enabled_tools
#[magnus::wrap(class = "Todozi")]
fn todozi_get_enabled_tools() -> Result<Vec<&AgentTool>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_enabled_tools().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_error_details
#[magnus::wrap(class = "Todozi")]
fn todozi_get_error_details(error_message: &str) -> Result<JsErrorDetails, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_error_details(error_message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_errors_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_errors_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_errors_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_filtered_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_filtered_tasks(filters: &str) -> Result<Vec<&Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_filtered_tasks(filters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_formatted_prompt
#[magnus::wrap(class = "Todozi")]
fn todozi_get_formatted_prompt(variables: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_formatted_prompt(variables).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_high_priority_summaries
#[magnus::wrap(class = "Todozi")]
fn todozi_get_high_priority_summaries() -> Result<Vec<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_high_priority_summaries().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_human_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_human_memories() -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_human_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_human_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_human_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_get_idea(idea_id: &str) -> Result<Option<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_idea(idea_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_idea_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_idea_statistics() -> Result<IdeaStatistics, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_idea_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_ideas_by_importance
#[magnus::wrap(class = "Todozi")]
fn todozi_get_ideas_by_importance(importance: IdeaImportance) -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_by_importance(importance).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_ideas_by_share_level
#[magnus::wrap(class = "Todozi")]
fn todozi_get_ideas_by_share_level(share_level: ShareLevel) -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_by_share_level(share_level).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_ideas_by_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_get_ideas_by_tag(tag: &str) -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_ideas_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_ideas_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_ideas_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_item
#[magnus::wrap(class = "Todozi")]
fn todozi_get_item(id: &str) -> Result<Option<&QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_item(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_item_mut
#[magnus::wrap(class = "Todozi")]
fn todozi_get_item_mut(id: &str) -> Result<Option<&mut QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_item_mut(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_items_by_status
#[magnus::wrap(class = "Todozi")]
fn todozi_get_items_by_status(status: QueueStatus) -> Result<Vec<&QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_items_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_key
#[magnus::wrap(class = "Todozi")]
fn todozi_get_key(user_id: &str) -> Result<Option<&ApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_key_by_public
#[magnus::wrap(class = "Todozi")]
fn todozi_get_key_by_public(public_key: &str) -> Result<Option<&ApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_key_by_public(public_key).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_long_term_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_long_term_memories() -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_long_term_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memories_by_importance
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memories_by_importance(importance: MemoryImportance) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_by_importance(importance).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memories_by_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memories_by_tag(tag: &str) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_memories_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memories_by_term
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memories_by_term(term: MemoryTerm) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_by_term(term).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memories_by_type
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memories_by_type(memory_type: &str) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_by_type(memory_type).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memories_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memories_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_memories_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memory(memory_id: &str) -> Result<Option<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Memories::get_memory(memory_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_memory_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_memory_statistics() -> Result<MemoryStatistics, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Memories::get_memory_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_most_used_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_get_most_used_tags(limit: usize) -> Result<Vec<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_most_used_tags(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_next_chunk_to_work_on
#[magnus::wrap(class = "Todozi")]
fn todozi_get_next_chunk_to_work_on() -> Result<Option<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_next_chunk_to_work_on().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_or_generate_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_get_or_generate_embedding(content_id: &str, text: &str, content_type: TodoziContentType, refresh_if_stale: bool) -> Result<Vec<f32>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_or_generate_embedding(content_id, text, content_type, refresh_if_stale).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_overdue_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_get_overdue_reminders() -> Result<Vec<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_overdue_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_pending_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_get_pending_reminders() -> Result<Vec<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_pending_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_priority_emoji
#[magnus::wrap(class = "Todozi")]
fn todozi_get_priority_emoji(priority: &str) -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_priority_emoji(priority).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_private_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_get_private_ideas() -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_private_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_processor_state
#[magnus::wrap(class = "Todozi")]
fn todozi_get_processor_state() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_processor_state().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_project
#[magnus::wrap(class = "Todozi")]
fn todozi_get_project(name: &str) -> Result<Project, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_project(name).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_project_stats
#[magnus::wrap(class = "Todozi")]
fn todozi_get_project_stats(project_name: &str) -> Result<ProjectStats, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::get_project_stats(project_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_project_summary
#[magnus::wrap(class = "Todozi")]
fn todozi_get_project_summary() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::get_project_summary().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_project_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_project_tasks(project_name: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_project_tasks_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_project_tasks_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_project_tasks_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_public_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_get_public_ideas() -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_public_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_queue_item
#[magnus::wrap(class = "Todozi")]
fn todozi_get_queue_item(id: &str) -> Result<QueueItem, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::get_queue_item(id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_queue_session
#[magnus::wrap(class = "Todozi")]
fn todozi_get_queue_session(session_id: &str) -> Result<crate::models::QueueSession, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::get_queue_session(session_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_queue_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_queue_statistics() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::get_queue_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_ready_chunks
#[magnus::wrap(class = "Todozi")]
fn todozi_get_ready_chunks() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_ready_chunks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_recent_actions
#[magnus::wrap(class = "Todozi")]
fn todozi_get_recent_actions(limit: Option<u32>) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_actions(limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_recent_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_get_recent_ideas(limit: usize) -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_recent_ideas(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_recent_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_recent_memories(limit: usize) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_memories(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_recent_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_get_recent_reminders(limit: usize) -> Result<Vec<&Reminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_reminders(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_recent_summaries
#[magnus::wrap(class = "Todozi")]
fn todozi_get_recent_summaries(limit: usize) -> Result<Vec<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_recent_summaries(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_recent_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_get_recent_tags(limit: usize) -> Result<Vec<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_recent_tags(limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_registration_info
#[magnus::wrap(class = "Todozi")]
fn todozi_get_registration_info() -> Result<Option<RegistrationInfo>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_registration_info().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_related_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_get_related_tags(tag_id: &str) -> Result<Vec<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_related_tags(tag_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_reminder
#[magnus::wrap(class = "Todozi")]
fn todozi_get_reminder(reminder_id: &str) -> Result<Option<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminder(reminder_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_reminder_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_reminder_statistics() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminder_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_reminders_by_priority
#[magnus::wrap(class = "Todozi")]
fn todozi_get_reminders_by_priority(priority: ReminderPriority) -> Result<Vec<&Reminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminders_by_priority(priority).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_reminders_by_status
#[magnus::wrap(class = "Todozi")]
fn todozi_get_reminders_by_status(status: ReminderStatus) -> Result<Vec<&Reminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminders_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_reminders_by_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_get_reminders_by_tag(tag: &str) -> Result<Vec<&Reminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_reminders_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_reminders_due_soon
#[magnus::wrap(class = "Todozi")]
fn todozi_get_reminders_due_soon(duration: Duration) -> Result<Vec<&Reminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_reminders_due_soon(duration).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_search_analytics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_search_analytics() -> Result<SearchAnalytics, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_search_analytics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_search_history
#[magnus::wrap(class = "Todozi")]
fn todozi_get_search_history(limit: Option<u32>) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_search_history(limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_search_suggestions
#[magnus::wrap(class = "Todozi")]
fn todozi_get_search_suggestions(query: &str, limit: usize) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_search_suggestions(query, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_secret_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_secret_memories() -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_secret_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_server_status
#[magnus::wrap(class = "Todozi")]
fn todozi_get_server_status() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_server_status().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_session
#[magnus::wrap(class = "Todozi")]
fn todozi_get_session(id: &str) -> Result<Option<&QueueSession>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_session(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_short_term_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_get_short_term_memories() -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_short_term_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_stats
#[magnus::wrap(class = "Todozi")]
fn todozi_get_stats() -> Result<HashMap<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_stats().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_status_emoji
#[magnus::wrap(class = "Todozi")]
fn todozi_get_status_emoji(status: &str) -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_status_emoji(status).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_storage_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_storage_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_storage_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_suggestions
#[magnus::wrap(class = "Todozi")]
fn todozi_get_suggestions(current_tags: &str, limit: usize) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_suggestions(current_tags, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_summaries_by_priority
#[magnus::wrap(class = "Todozi")]
fn todozi_get_summaries_by_priority(priority: SummaryPriority) -> Result<Vec<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_summaries_by_priority(priority).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_summaries_by_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_get_summaries_by_tag(tag: &str) -> Result<Vec<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_summaries_by_tag(tag).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_summary
#[magnus::wrap(class = "Todozi")]
fn todozi_get_summary(summary_id: &str) -> Result<Option<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_summary(summary_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_summary_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_summary_statistics() -> Result<SummaryStatistics, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_summary_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tag(tag_id: &str) -> Result<Option<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tag(tag_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tag_by_name
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tag_by_name(name: &str) -> Result<Option<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tag_by_name(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tag_statistics
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tag_statistics() -> Result<TagStatistics, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tag_statistics().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tags_by_category
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tags_by_category(category: &str) -> Result<Vec<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::get_tags_by_category(category).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_task
#[magnus::wrap(class = "Todozi")]
fn todozi_get_task(id: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task(id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_task_assignments
#[magnus::wrap(class = "Todozi")]
fn todozi_get_task_assignments(task_id: &str) -> Result<Vec<&AgentAssignment>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_assignments(task_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_task_from_any_project
#[magnus::wrap(class = "Todozi")]
fn todozi_get_task_from_any_project(id: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_from_any_project(id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_task_from_project
#[magnus::wrap(class = "Todozi")]
fn todozi_get_task_from_project(project_name: &str, task_id: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_from_project(project_name, task_id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_task_mut
#[magnus::wrap(class = "Todozi")]
fn todozi_get_task_mut(id: &str) -> Result<Option<&mut Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_task_mut(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tasks_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tasks_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tasks_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tdz_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tdz_api_key() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tdz_api_key().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_team_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_get_team_ideas() -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::get_team_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tool
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tool(name: &str) -> Result<Option<&Box<dyn Tool>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tool(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tool_definitions
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tool_definitions() -> Result<Vec<serde_json::Value>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tool_definitions().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_training_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_get_training_dir() -> Result<PathBuf, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_training_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_tsne_coordinates
#[magnus::wrap(class = "Todozi")]
fn todozi_get_tsne_coordinates(content_ids: Vec<String>, dimensions: usize) -> Result<Vec<(Strin>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_tsne_coordinates(content_ids, dimensions).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_unresolved_errors
#[magnus::wrap(class = "Todozi")]
fn todozi_get_unresolved_errors() -> Result<Vec<&Error>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_unresolved_errors().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// get_version_history
#[magnus::wrap(class = "Todozi")]
fn todozi_get_version_history(content_id: &str) -> Result<Vec<serde_json::Value>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::get_version_history(content_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_add_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_add_command(command: AddCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_add_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_agent_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_agent_command(command: Commands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_agent_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_ai_commands
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_ai_commands(command: &str, args: Vec<String>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_ai_commands(command, args).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_api_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_api_command(command: ApiCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_api_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_chat_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_chat_command(command: Commands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_chat_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_emb_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_emb_command(command: Commands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_emb_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_error_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_error_command(command: Commands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_error_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_extract_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_extract_command(content: Option<String>, file: Option<String>, output_format: &str, human: bool) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_extract_command(content.as_deref(), file.as_deref(), output_format, human).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_idea_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_idea_command(command: IdeaCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_idea_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_ind_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_ind_command() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_ind_command().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_list_backups_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_list_backups_command() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_list_backups_command().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_list_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_list_command(command: ListCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_list_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_memory_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_memory_command(command: MemoryCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_memory_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_project_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_project_command(command: ProjectCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_project_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_queue_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_queue_command(command: QueueCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_queue_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_search_all_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_search_all_command(command: Commands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_search_all_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_search_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_search_command(command: SearchCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_search_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_server_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_server_command(command: ServerCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_server_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_show_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_show_command(command: ShowCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_show_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_stats_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_stats_command(_command: StatsCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_stats_command(_command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_strategy_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_strategy_command(content: Option<String>, file: Option<String>, output_format: &str, human: bool) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_strategy_command(content.as_deref(), file.as_deref(), output_format, human).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_train_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_train_command(command: TrainingCommands) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_train_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// handle_update_command
#[magnus::wrap(class = "Todozi")]
fn todozi_handle_update_command(id: &str, action: Option<String>, time: Option<String>, priority: Option<String>, project: Option<String>, status: Option<String>, assignee: Option<String>, tags: Option<String>, dependencies: Option<String>, context: Option<String>, progress: Option<u8>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::handle_update_command(id, action.as_deref(), time.as_deref(), priority.as_deref(), project.as_deref(), status.as_deref(), assignee.as_deref(), tags.as_deref(), dependencies.as_deref(), context.as_deref(), progress.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// has_capability
#[magnus::wrap(class = "Todozi")]
fn todozi_has_capability(capability: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_capability(capability).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// has_results
#[magnus::wrap(class = "Todozi")]
fn todozi_has_results() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_results().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// has_specialization
#[magnus::wrap(class = "Todozi")]
fn todozi_has_specialization(specialization: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_specialization(specialization).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// has_tool
#[magnus::wrap(class = "Todozi")]
fn todozi_has_tool(tool_name: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::has_tool(tool_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// hash_project_name
#[magnus::wrap(class = "Todozi")]
fn todozi_hash_project_name(project_name: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::hash_project_name(project_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// hierarchical_clustering
#[magnus::wrap(class = "Todozi")]
fn todozi_hierarchical_clustering(content_types: Vec<TodoziContentType>, max_depth: usize) -> Result<Vec<HierarchicalCluster>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::hierarchical_clustering(content_types, max_depth).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// high
#[magnus::wrap(class = "Todozi")]
fn todozi_high(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::high(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// high_priority_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_high_priority_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::high_priority_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// human
#[magnus::wrap(class = "Todozi")]
fn todozi_human(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Actions::human(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// human_task
#[magnus::wrap(class = "Todozi")]
fn todozi_human_task(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::human_task(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// hybrid_search
#[magnus::wrap(class = "Todozi")]
fn todozi_hybrid_search(query: &str, keywords: Vec<String>, content_types: Option<Vec<TodoziContentType>>, semantic_weight: f32, // 0.0-1.0
        limit: usize) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::hybrid_search(query, keywords, content_types.as_deref(), semantic_weight, // 0.0-1.0
        limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// idea
#[magnus::wrap(class = "Todozi")]
fn todozi_idea(idea: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::idea(idea).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ideate
#[magnus::wrap(class = "Todozi")]
fn todozi_ideate(idea: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ideate(idea).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// import_from_format
#[magnus::wrap(class = "Todozi")]
fn todozi_import_from_format(data: &str, format: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::import_from_format(data, format).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// importance
#[magnus::wrap(class = "Todozi")]
fn todozi_importance(importance: IdeaImportance) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::importance(importance).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// important
#[magnus::wrap(class = "Todozi")]
fn todozi_important(moment: &str, meaning: &str, reason: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Memories::important(moment, meaning, reason).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// important_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_important_memory(moment: &str, meaning: &str, reason: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::important_memory(moment, meaning, reason).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// increment_tag_usage
#[magnus::wrap(class = "Todozi")]
fn todozi_increment_tag_usage(tag_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::increment_tag_usage(tag_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// init
#[magnus::wrap(class = "Todozi")]
fn todozi_init() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::init().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// init_storage
#[magnus::wrap(class = "Todozi")]
fn todozi_init_storage() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::init_storage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// init_with_auto_registration
#[magnus::wrap(class = "Todozi")]
fn todozi_init_with_auto_registration() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::init_with_auto_registration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// initialize
#[magnus::wrap(class = "Todozi")]
fn todozi_initialize() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// initialize_grok_level_todozi_system
#[magnus::wrap(class = "Todozi")]
fn todozi_initialize_grok_level_todozi_system() -> Result<std::result::Result<
    SharedTodozi,
    TodoziError,
>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_grok_level_todozi_system().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// initialize_grok_level_todozi_system_with_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_initialize_grok_level_todozi_system_with_embedding(enable_embeddings: bool) -> Result<std::result::Result<(SharedTodozi, Option<TodoziEmbeddingService>), TodoziError>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_grok_level_todozi_system_with_embedding(enable_embeddings).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// initialize_tdz_content_processor
#[magnus::wrap(class = "Todozi")]
fn todozi_initialize_tdz_content_processor() -> Result<SharedTodoziState, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_tdz_content_processor().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// initialize_tdz_processor
#[magnus::wrap(class = "Todozi")]
fn todozi_initialize_tdz_processor() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::initialize_tdz_processor().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// interactive_create_task
#[magnus::wrap(class = "Todozi")]
fn todozi_interactive_create_task() -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::interactive_create_task().await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// io
#[magnus::wrap(class = "Todozi")]
fn todozi_io(message: impl Into<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::io(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_active
#[magnus::wrap(class = "Todozi")]
fn todozi_is_active() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_active().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_admin
#[magnus::wrap(class = "Todozi")]
fn todozi_is_admin(public_key: &str, private_key: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_admin(public_key, private_key).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_available
#[magnus::wrap(class = "Todozi")]
fn todozi_is_available() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_available().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_backlog
#[magnus::wrap(class = "Todozi")]
fn todozi_is_backlog() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_backlog().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_complete
#[magnus::wrap(class = "Todozi")]
fn todozi_is_complete() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_complete().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_completed
#[magnus::wrap(class = "Todozi")]
fn todozi_is_completed() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_completed().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_empty
#[magnus::wrap(class = "Todozi")]
fn todozi_is_empty() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_empty().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_overdue
#[magnus::wrap(class = "Todozi")]
fn todozi_is_overdue() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_overdue().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// is_registered
#[magnus::wrap(class = "Todozi")]
fn todozi_is_registered() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::is_registered().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// keyword_search
#[magnus::wrap(class = "Todozi")]
fn todozi_keyword_search(query: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::keyword_search(query).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// keyword_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_keyword_tasks(query: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::keyword_tasks(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// launch_gui
#[magnus::wrap(class = "Todozi")]
fn todozi_launch_gui() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::launch_gui().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// launch_tui
#[magnus::wrap(class = "Todozi")]
fn todozi_launch_tui() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::launch_tui().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// len
#[magnus::wrap(class = "Todozi")]
fn todozi_len() -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::len().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list
#[magnus::wrap(class = "Todozi")]
fn todozi_list() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_active_api_keys
#[magnus::wrap(class = "Todozi")]
fn todozi_list_active_api_keys() -> Result<Vec<JsApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_active_api_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_active_items
#[magnus::wrap(class = "Todozi")]
fn todozi_list_active_items() -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_active_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_agent_assignments
#[magnus::wrap(class = "Todozi")]
fn todozi_list_agent_assignments(agent_id: &str) -> Result<Vec<AgentAssignment>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_agent_assignments(agent_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_agents
#[magnus::wrap(class = "Todozi")]
fn todozi_list_agents() -> Result<Vec<Agent>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_all_agent_assignments
#[magnus::wrap(class = "Todozi")]
fn todozi_list_all_agent_assignments() -> Result<Vec<AgentAssignment>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_all_agent_assignments().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_api_keys
#[magnus::wrap(class = "Todozi")]
fn todozi_list_api_keys() -> Result<Vec<JsApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_api_keys().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_available_agents
#[magnus::wrap(class = "Todozi")]
fn todozi_list_available_agents() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_available_agents().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_available_commands
#[magnus::wrap(class = "Todozi")]
fn todozi_list_available_commands() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_available_commands().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_available_tools
#[magnus::wrap(class = "Todozi")]
fn todozi_list_available_tools() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_available_tools().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_backlog_items
#[magnus::wrap(class = "Todozi")]
fn todozi_list_backlog_items() -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_backlog_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_backups
#[magnus::wrap(class = "Todozi")]
fn todozi_list_backups() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_backups().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_code_chunks
#[magnus::wrap(class = "Todozi")]
fn todozi_list_code_chunks() -> Result<Vec<CodeChunk>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_code_chunks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_complete_items
#[magnus::wrap(class = "Todozi")]
fn todozi_list_complete_items() -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_complete_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_errors
#[magnus::wrap(class = "Todozi")]
fn todozi_list_errors() -> Result<Vec<Error>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_errors().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_feelings
#[magnus::wrap(class = "Todozi")]
fn todozi_list_feelings() -> Result<Vec<Feeling>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_feelings().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_list_ideas() -> Result<Vec<Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Ideas::list_ideas().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_list_memories() -> Result<Vec<Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_memories().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_project_task_containers
#[magnus::wrap(class = "Todozi")]
fn todozi_list_project_task_containers() -> Result<Vec<ProjectTaskContainer>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_project_task_containers().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_projects
#[magnus::wrap(class = "Todozi")]
fn todozi_list_projects() -> Result<Vec<Project>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::list_projects().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_queue_items
#[magnus::wrap(class = "Todozi")]
fn todozi_list_queue_items() -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::list_queue_items().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_queue_items_by_status
#[magnus::wrap(class = "Todozi")]
fn todozi_list_queue_items_by_status(status: QueueStatus) -> Result<Vec<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Queue::list_queue_items_by_status(status).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_list_reminders() -> Result<Vec<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_reminders().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_summaries
#[magnus::wrap(class = "Todozi")]
fn todozi_list_summaries() -> Result<Vec<JsSummary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_summaries().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_list_tasks() -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_tasks_across_projects
#[magnus::wrap(class = "Todozi")]
fn todozi_list_tasks_across_projects(filters: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_tasks_across_projects(filters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_tasks_in_project
#[magnus::wrap(class = "Todozi")]
fn todozi_list_tasks_in_project(project_name: &str, filters: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_tasks_in_project(project_name, filters).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// list_training_data
#[magnus::wrap(class = "Todozi")]
fn todozi_list_training_data() -> Result<Vec<TrainingData>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::list_training_data().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load
#[magnus::wrap(class = "Todozi")]
fn todozi_load(model_name: &str, device: Device) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load(model_name, device).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_additional_model
#[magnus::wrap(class = "Todozi")]
fn todozi_load_additional_model(model_name: &str, model_alias: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_additional_model(model_name, model_alias).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_load_agent(agent_id: &str) -> Result<Agent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_agent(agent_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_agent_assignment
#[magnus::wrap(class = "Todozi")]
fn todozi_load_agent_assignment(agent_id: &str, task_id: &str) -> Result<AgentAssignment, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_agent_assignment(agent_id, task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_agents
#[magnus::wrap(class = "Todozi")]
fn todozi_load_agents() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_agents().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_api_key_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_load_api_key_collection() -> Result<ApiKeyCollection, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_api_key_collection().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_api_keys
#[magnus::wrap(class = "Todozi")]
fn todozi_load_api_keys() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_api_keys().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_code_chunk
#[magnus::wrap(class = "Todozi")]
fn todozi_load_code_chunk(chunk_id: &str) -> Result<CodeChunk, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_code_chunk(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_config
#[magnus::wrap(class = "Todozi")]
fn todozi_load_config() -> Result<Config, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_config().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_error
#[magnus::wrap(class = "Todozi")]
fn todozi_load_error(error_id: &str) -> Result<Error, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_error(error_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_extended_data
#[magnus::wrap(class = "Todozi")]
fn todozi_load_extended_data() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_extended_data().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_feeling
#[magnus::wrap(class = "Todozi")]
fn todozi_load_feeling(id: &str) -> Result<Feeling, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_feeling(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_load_idea(idea_id: &str) -> Result<Idea, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_idea(idea_id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_load_memory(memory_id: &str) -> Result<Memory, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_memory(memory_id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_project
#[magnus::wrap(class = "Todozi")]
fn todozi_load_project(project_name: &str) -> Result<Project, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_project(project_name).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_project_task_container
#[magnus::wrap(class = "Todozi")]
fn todozi_load_project_task_container(project_name: &str) -> Result<ProjectTaskContainer, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_project_task_container(project_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_project_task_container_by_hash
#[magnus::wrap(class = "Todozi")]
fn todozi_load_project_task_container_by_hash(project_hash: &str) -> Result<ProjectTaskContainer, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_project_task_container_by_hash(project_hash).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_queue_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_load_queue_collection() -> Result<QueueCollection, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_queue_collection().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_task
#[magnus::wrap(class = "Todozi")]
fn todozi_load_task(task_id: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_task(task_id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_task_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_load_task_collection(collection_name: &str) -> Result<TaskCollection, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_task_collection(collection_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_load_tasks() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_tasks().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// load_training_data
#[magnus::wrap(class = "Todozi")]
fn todozi_load_training_data(training_data_id: &str) -> Result<TrainingData, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::load_training_data(training_data_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// long_term_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_long_term_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::long_term_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// low
#[magnus::wrap(class = "Todozi")]
fn todozi_low(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::low(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// mark_chunk_completed
#[magnus::wrap(class = "Todozi")]
fn todozi_mark_chunk_completed(chunk_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_chunk_completed(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// mark_chunk_validated
#[magnus::wrap(class = "Todozi")]
fn todozi_mark_chunk_validated(chunk_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_chunk_validated(chunk_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// mark_reminder_cancelled
#[magnus::wrap(class = "Todozi")]
fn todozi_mark_reminder_cancelled(reminder_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_reminder_cancelled(reminder_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// mark_reminder_completed
#[magnus::wrap(class = "Todozi")]
fn todozi_mark_reminder_completed(reminder_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::mark_reminder_completed(reminder_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// matches
#[magnus::wrap(class = "Todozi")]
fn todozi_matches(public_key: &str, private_key: Option<&str>) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::matches(public_key, private_key.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// max_tokens
#[magnus::wrap(class = "Todozi")]
fn todozi_max_tokens() -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::max_tokens().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// meaning
#[magnus::wrap(class = "Todozi")]
fn todozi_meaning(meaning: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::meaning(meaning).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// merge_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_merge_tags(primary_tag_id: &str, duplicate_tag_ids: Vec<String>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::merge_tags(primary_tag_id, duplicate_tag_ids).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// migrate
#[magnus::wrap(class = "Todozi")]
fn todozi_migrate() -> Result<MigrationReport, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::migrate().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// migrate_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_migrate_tasks(dry_run: Option<bool>, verbose: Option<bool>, force_overwrite: Option<bool>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::migrate_tasks(dry_run.as_deref(), verbose.as_deref(), force_overwrite.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// migrate_to_project_based
#[magnus::wrap(class = "Todozi")]
fn todozi_migrate_to_project_based() -> Result<MigrationReport, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::migrate_to_project_based().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// moment
#[magnus::wrap(class = "Todozi")]
fn todozi_moment(moment: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::moment(moment).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// move_task
#[magnus::wrap(class = "Todozi")]
fn todozi_move_task(id: &str, from_collection: &str, to_collection: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::move_task(id, from_collection, to_collection).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// multi_query_search
#[magnus::wrap(class = "Todozi")]
fn todozi_multi_query_search(queries: Vec<&str>, aggregation: AggregationType, content_types: Option<Vec<TodoziContentType>>, limit: usize) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::multi_query_search(queries, aggregation, content_types.as_deref(), limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// name
#[magnus::wrap(class = "Todozi")]
fn todozi_name(name: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::name(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// new_full
#[magnus::wrap(class = "Todozi")]
fn todozi_new_full(user_id: &str, action: &str, time: &str, priority: Priority, parent_project: &str, status: Status, assignee: Option<Assignee>, tags: Vec<String>, dependencies: Vec<String>, context_notes: Option<String>, progress: Option<u8>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_full(user_id, action, time, priority, parent_project, status, assignee.as_deref(), tags, dependencies, context_notes.as_deref(), progress.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// new_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_new_idea(idea: Idea) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_idea(idea).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// new_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_new_memory(memory: Memory) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_memory(memory).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// new_with_hashes
#[magnus::wrap(class = "Todozi")]
fn todozi_new_with_hashes(server_url: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::new_with_hashes(server_url).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// ok
#[magnus::wrap(class = "Todozi")]
fn todozi_ok(body: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::ok(body).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// overdue_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_overdue_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::overdue_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_agent_assignment_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_agent_assignment_format(agent_text: &str) -> Result<AgentAssignment, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_agent_assignment_format(agent_text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_chunking_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_chunking_format(chunk_text: &str) -> Result<CodeChunk, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_chunking_format(chunk_text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_dependencies
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_dependencies(deps_str: &str) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_dependencies(deps_str).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_error_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_error_format(error_text: &str) -> Result<Error, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_error_format(error_text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_feeling_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_feeling_format(feel_text: &str) -> Result<Feeling, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_feeling_format(feel_text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_idea_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_idea_format(idea_text: &str) -> Result<Idea, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_idea_format(idea_text).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_memory_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_memory_format(memory_text: &str, user_id: &str) -> Result<Memory, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_memory_format(memory_text, user_id).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_reminder_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_reminder_format(reminder_text: &str) -> Result<Reminder, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_reminder_format(reminder_text).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_summary_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_summary_format(summary_text: &str) -> Result<Summary, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_summary_format(summary_text).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_tags(tags_str: &str) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_tags(tags_str).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_tdz_command
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_tdz_command(text: &str) -> Result<Vec<TdzCommand>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_tdz_command(text).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_todozi_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_todozi_format(todozi_text: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_todozi_format(todozi_text).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// parse_training_data_format
#[magnus::wrap(class = "Todozi")]
fn todozi_parse_training_data_format(train_text: &str) -> Result<TrainingData, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::parse_training_data_format(train_text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// pending_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_pending_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::pending_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// plan_task_actions
#[magnus::wrap(class = "Todozi")]
fn todozi_plan_task_actions(goal: &str) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::plan_task_actions(goal).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// plan_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_plan_tasks(goal: &str, complexity: Option<&str>, timeline: Option<&str>, context: Option<&str>) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::plan_tasks(goal, complexity.as_deref(), timeline.as_deref(), context.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// predict_relevance
#[magnus::wrap(class = "Todozi")]
fn todozi_predict_relevance(_features: &str) -> Result<std::result::Result<f32, Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::predict_relevance(_features).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// preload_related_embeddings
#[magnus::wrap(class = "Todozi")]
fn todozi_preload_related_embeddings(content_id: &str, depth: usize) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::preload_related_embeddings(content_id, depth).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// prepare_task_content
#[magnus::wrap(class = "Todozi")]
fn todozi_prepare_task_content(task: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::prepare_task_content(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// prioritize_queue_item
#[magnus::wrap(class = "Todozi")]
fn todozi_prioritize_queue_item(item_id: &str, priority: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::prioritize_queue_item(item_id, priority).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// priority
#[magnus::wrap(class = "Todozi")]
fn todozi_priority(priority: SummaryPriority) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::priority(priority).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// private_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_private_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::private_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_chat
#[magnus::wrap(class = "Todozi")]
fn todozi_process_chat(message: &str, user_id: &str) -> Result<ChatContent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chat(message, user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_chat_message
#[magnus::wrap(class = "Todozi")]
fn todozi_process_chat_message(message: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chat_message(message).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_chat_message_extended
#[magnus::wrap(class = "Todozi")]
fn todozi_process_chat_message_extended(message: &str, user_id: &str) -> Result<ChatContent, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chat_message_extended(message, user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_chunking_message
#[magnus::wrap(class = "Todozi")]
fn todozi_process_chunking_message(message: &str) -> Result<Vec<CodeChunk>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_chunking_message(message).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_json_examples
#[magnus::wrap(class = "Todozi")]
fn todozi_process_json_examples(json_data: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_json_examples(json_data).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_tdz_commands
#[magnus::wrap(class = "Todozi")]
fn todozi_process_tdz_commands(text: &str, base_url: &str, api_key: Option<&str>) -> Result<Vec<Value>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_tdz_commands(text, base_url, api_key.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// process_workflow
#[magnus::wrap(class = "Todozi")]
fn todozi_process_workflow(tasks: Vec<Task>) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::process_workflow(tasks).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// profile_search_performance
#[magnus::wrap(class = "Todozi")]
fn todozi_profile_search_performance(query: &str, iterations: usize) -> Result<PerformanceMetrics, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::profile_search_performance(query, iterations).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// project_name
#[magnus::wrap(class = "Todozi")]
fn todozi_project_name() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::project_name().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// project_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_project_tasks(project_name: &str) -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::project_tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// public_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_public_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::public_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_active
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_active() -> Result<Vec<JsQueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_active().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_add
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_add(task_name: &str, description: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_add(task_name, description).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_backlog
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_backlog() -> Result<Vec<JsQueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_backlog().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_bulk_operations
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_bulk_operations(operations: Vec<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_bulk_operations(operations).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_complete
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_complete(session_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_complete(session_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_list
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_list() -> Result<Vec<JsQueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_list().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// queue_start
#[magnus::wrap(class = "Todozi")]
fn todozi_queue_start(item_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::queue_start(item_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// quick
#[magnus::wrap(class = "Todozi")]
fn todozi_quick() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::quick().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// quick_task
#[magnus::wrap(class = "Todozi")]
fn todozi_quick_task(action: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::quick_task(action).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// reason
#[magnus::wrap(class = "Todozi")]
fn todozi_reason(reason: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::reason(reason).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// recommend_similar
#[magnus::wrap(class = "Todozi")]
fn todozi_recommend_similar(based_on: Vec<String>, exclude: Vec<String>, limit: usize) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::recommend_similar(based_on, exclude, limit).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// register_tool
#[magnus::wrap(class = "Todozi")]
fn todozi_register_tool(tool_def: JsToolDefinition) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::register_tool(tool_def).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// register_with_server
#[magnus::wrap(class = "Todozi")]
fn todozi_register_with_server(server_url: &str) -> Result<RegistrationInfo, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::register_with_server(server_url).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// relationships_per_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_relationships_per_tag() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::relationships_per_tag().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remember
#[magnus::wrap(class = "Todozi")]
fn todozi_remember(moment: &str, meaning: &str) -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::remember(moment, meaning).await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remind_at
#[magnus::wrap(class = "Todozi")]
fn todozi_remind_at(remind_at: DateTime<Utc>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remind_at(remind_at).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remove_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_remove_api_key(user_id: &str) -> Result<JsApiKey, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_api_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remove_from_task
#[magnus::wrap(class = "Todozi")]
fn todozi_remove_from_task(task_id: &str, tag: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tags::remove_from_task(task_id, tag).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remove_item
#[magnus::wrap(class = "Todozi")]
fn todozi_remove_item(id: &str) -> Result<Option<QueueItem>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_item(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remove_key
#[magnus::wrap(class = "Todozi")]
fn todozi_remove_key(user_id: &str) -> Result<Option<ApiKey>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_key(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remove_tag_from_task
#[magnus::wrap(class = "Todozi")]
fn todozi_remove_tag_from_task(task_id: &str, tag: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_tag_from_task(task_id, tag).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// remove_task
#[magnus::wrap(class = "Todozi")]
fn todozi_remove_task(id: &str) -> Result<Option<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::remove_task(id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// render
#[magnus::wrap(class = "Todozi")]
fn todozi_render(config: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::render(config).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// render_compact
#[magnus::wrap(class = "Todozi")]
fn todozi_render_compact(config: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::render_compact(config).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// render_detailed
#[magnus::wrap(class = "Todozi")]
fn todozi_render_detailed(config: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::render_detailed(config).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// reorder_queue
#[magnus::wrap(class = "Todozi")]
fn todozi_reorder_queue(item_ids: Vec<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::reorder_queue(item_ids).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// resolve_error
#[magnus::wrap(class = "Todozi")]
fn todozi_resolve_error(error_id: &str, resolution: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::resolve_error(error_id, resolution).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// restore_backup
#[magnus::wrap(class = "Todozi")]
fn todozi_restore_backup(backup_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::restore_backup(backup_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// restore_embeddings
#[magnus::wrap(class = "Todozi")]
fn todozi_restore_embeddings(backup_path: &str) -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::restore_embeddings(backup_path).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// run
#[magnus::wrap(class = "Todozi")]
fn todozi_run() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::run().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// run_interactive
#[magnus::wrap(class = "Todozi")]
fn todozi_run_interactive() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::run_interactive().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// sample_task
#[magnus::wrap(class = "Todozi")]
fn todozi_sample_task() -> Result<Task, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::sample_task().await.map(|item| item.to_ruby())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_save_agent(agent: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_agent(agent).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_agent_assignment
#[magnus::wrap(class = "Todozi")]
fn todozi_save_agent_assignment(assignment: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_agent_assignment(assignment).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_api_key_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_save_api_key_collection(collection: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_api_key_collection(collection).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_as_default
#[magnus::wrap(class = "Todozi")]
fn todozi_save_as_default(model_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_as_default(model_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_code_chunk
#[magnus::wrap(class = "Todozi")]
fn todozi_save_code_chunk(chunk: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_code_chunk(chunk).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_config
#[magnus::wrap(class = "Todozi")]
fn todozi_save_config(config: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_config(config).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_error
#[magnus::wrap(class = "Todozi")]
fn todozi_save_error(error: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_error(error).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_feeling
#[magnus::wrap(class = "Todozi")]
fn todozi_save_feeling(feeling: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_feeling(feeling).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_save_idea(idea: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_idea(idea).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_save_memory(memory: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_memory(memory).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_processed_content
#[magnus::wrap(class = "Todozi")]
fn todozi_save_processed_content(raw: &str, cleaned: &str, session_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_processed_content(raw, cleaned, session_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_project
#[magnus::wrap(class = "Todozi")]
fn todozi_save_project(project: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_project(project).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_project_task_container
#[magnus::wrap(class = "Todozi")]
fn todozi_save_project_task_container(container: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_project_task_container(container).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_queue_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_save_queue_collection(collection: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_queue_collection(collection).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_search_query
#[magnus::wrap(class = "Todozi")]
fn todozi_save_search_query(query: &str, name: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_search_query(query, name.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_task
#[magnus::wrap(class = "Todozi")]
fn todozi_save_task(task: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_task(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_task_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_save_task_collection(collection_name: &str, collection: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_task_collection(collection_name, collection).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_task_with_embedding
#[magnus::wrap(class = "Todozi")]
fn todozi_save_task_with_embedding(task: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_task_with_embedding(task).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_save_tasks() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_tasks().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// save_training_data
#[magnus::wrap(class = "Todozi")]
fn todozi_save_training_data(training_data: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::save_training_data(training_data).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search
#[magnus::wrap(class = "Todozi")]
fn todozi_search(query: &str, options: SearchOptions) -> Result<SearchResults, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search(query, options).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_ideas
#[magnus::wrap(class = "Todozi")]
fn todozi_search_ideas(query: &str) -> Result<Vec<&Idea>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_ideas(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_memories
#[magnus::wrap(class = "Todozi")]
fn todozi_search_memories(query: &str) -> Result<Vec<&Memory>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_memories(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_reminders
#[magnus::wrap(class = "Todozi")]
fn todozi_search_reminders(query: &str) -> Result<Vec<JsReminder>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_reminders(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_summaries
#[magnus::wrap(class = "Todozi")]
fn todozi_search_summaries(query: &str) -> Result<Vec<&Summary>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_summaries(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_search_tags(query: &str) -> Result<Vec<&Tag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_tags(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_search_tasks(query: &str, semantic: bool, limit: Option<usize>) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_tasks(query, semantic, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_tasks_semantic
#[magnus::wrap(class = "Todozi")]
fn todozi_search_tasks_semantic(query: &str, max_results: usize) -> Result<Vec<SemanticSearchResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_tasks_semantic(query, max_results).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// search_with_filters
#[magnus::wrap(class = "Todozi")]
fn todozi_search_with_filters(filters: TaskFilters, limit: Option<usize>) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::search_with_filters(filters, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// see_all
#[magnus::wrap(class = "Todozi")]
fn todozi_see_all() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Easy::see_all().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// semantic_search
#[magnus::wrap(class = "Todozi")]
fn todozi_semantic_search(query: &str, content_types: Option<Vec<TodoziContentType>>, limit: Option<usize>) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::semantic_search(query, content_types.as_deref(), limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// serialization
#[magnus::wrap(class = "Todozi")]
fn todozi_serialization(message: impl Into<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::serialization(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// set_color_scheme
#[magnus::wrap(class = "Todozi")]
fn todozi_set_color_scheme(scheme_json: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::set_color_scheme(scheme_json).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// share
#[magnus::wrap(class = "Todozi")]
fn todozi_share(share: ShareLevel) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::share(share).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// short_term_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_short_term_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::short_term_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// show_loading_screen
#[magnus::wrap(class = "Todozi")]
fn todozi_show_loading_screen() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::show_loading_screen().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// similar
#[magnus::wrap(class = "Todozi")]
fn todozi_similar(query: &str) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::similar(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// similar_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_similar_tasks(task_id: &str) -> Result<Vec<SimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::similar_tasks(task_id).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// similar_tasks_emb
#[magnus::wrap(class = "Todozi")]
fn todozi_similar_tasks_emb(query: &str) -> Result<Vec<JsSimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::similar_tasks_emb(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// similarity_threshold_search
#[magnus::wrap(class = "Todozi")]
fn todozi_similarity_threshold_search(query: &str, threshold: f64, limit: Option<u32>) -> Result<Vec<JsSimilarityResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::similarity_threshold_search(query, threshold, limit.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// smart
#[magnus::wrap(class = "Todozi")]
fn todozi_smart(query: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::smart(query).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// smart_search
#[magnus::wrap(class = "Todozi")]
fn todozi_smart_search(query: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::smart_search(query).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// specializations
#[magnus::wrap(class = "Todozi")]
fn todozi_specializations(specializations: Vec<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::specializations(specializations).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start
#[magnus::wrap(class = "Todozi")]
fn todozi_start(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::start(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_edit
#[magnus::wrap(class = "Todozi")]
fn todozi_start_edit(_task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_edit(_task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_edit_session
#[magnus::wrap(class = "Todozi")]
fn todozi_start_edit_session(task_id: &str) -> Result<EditSession, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_edit_session(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_local_server
#[magnus::wrap(class = "Todozi")]
fn todozi_start_local_server(host: Option<String>, port: Option<u32>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_local_server(host.as_deref(), port.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_queue_session
#[magnus::wrap(class = "Todozi")]
fn todozi_start_queue_session(queue_item_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_queue_session(queue_item_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_server
#[magnus::wrap(class = "Todozi")]
fn todozi_start_server(host: Option<String>, port: Option<u16>) -> Result<std::result::Result<(), Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_server(host.as_deref(), port.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_session
#[magnus::wrap(class = "Todozi")]
fn todozi_start_session(queue_item_id: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_session(queue_item_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// start_task
#[magnus::wrap(class = "Todozi")]
fn todozi_start_task(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::start_task(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// stats
#[magnus::wrap(class = "Todozi")]
fn todozi_stats() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Emb::stats().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// status
#[magnus::wrap(class = "Todozi")]
fn todozi_status(status: AgentStatus) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::status(status).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// stop_server
#[magnus::wrap(class = "Todozi")]
fn todozi_stop_server() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::stop_server().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage
#[magnus::wrap(class = "Todozi")]
fn todozi_storage() -> Result<Storage, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_check_folder_structure
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_check_folder_structure() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_check_folder_structure().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_clear_registration
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_clear_registration() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_clear_registration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_create_backup
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_create_backup() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_create_backup().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_delete_project_by_name
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_delete_project_by_name(name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_delete_project_by_name(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_delete_task_from_project
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_delete_task_from_project(task_id: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_delete_task_from_project(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_ensure_folder_structure
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_ensure_folder_structure() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_ensure_folder_structure().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_ai_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_ai_tasks() -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_ai_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_all_active_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_all_active_tasks() -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_all_active_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_all_completed_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_all_completed_tasks() -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_all_completed_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_collaborative_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_collaborative_tasks() -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_collaborative_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_human_tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_human_tasks() -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_human_tasks().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_project_stats
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_project_stats(project_name: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_project_stats(project_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_registration_info
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_registration_info() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_registration_info().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_storage_dir
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_storage_dir() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_storage_dir().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_get_task_from_any_project
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_get_task_from_any_project(task_id: &str) -> Result<JsTask, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_get_task_from_any_project(task_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_init
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_init() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_init().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_is_registered
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_is_registered() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_is_registered().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_list_backups
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_list_backups() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_list_backups().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_list_projects
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_list_projects() -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_list_projects().await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_load_config
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_load_config() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_load_config().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_load_project
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_load_project(name: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_load_project(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_load_task_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_load_task_collection(name: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_load_task_collection(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_register_with_server
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_register_with_server(server_url: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_register_with_server(server_url).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_restore_backup
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_restore_backup(backup_name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_restore_backup(backup_name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_save_config
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_save_config() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_save_config().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_save_project
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_save_project(name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_save_project(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_save_task_collection
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_save_task_collection(name: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_save_task_collection(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// storage_search_tasks_semantic
#[magnus::wrap(class = "Todozi")]
fn todozi_storage_search_tasks_semantic(query: &str, max_results: Option<u32>) -> Result<Vec<JsTask>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::storage_search_tasks_semantic(query, max_results.as_deref()).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// strategy_content
#[magnus::wrap(class = "Todozi")]
fn todozi_strategy_content(content: Option<String>, file_path: Option<String>, output_format: &str, human: bool) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strategy_content(content.as_deref(), file_path.as_deref(), output_format, human).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// strategy_content_analysis
#[magnus::wrap(class = "Todozi")]
fn todozi_strategy_content_analysis(text: &str, format: Option<String>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strategy_content_analysis(text, format.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// strike_cluster
#[magnus::wrap(class = "Todozi")]
fn todozi_strike_cluster(_embedding: &str) -> Result<std::result::Result<i32, Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strike_cluster(_embedding).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// strike_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_strike_tags(_features: &str) -> Result<std::result::Result<Vec<f32>, Box<dyn std::error::Error>>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::strike_tags(_features).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// success
#[magnus::wrap(class = "Todozi")]
fn todozi_success(output: &str, execution_time_ms: u64) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::success(output, execution_time_ms).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// suggest_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_suggest_tags(content_id: &str, top_k: usize) -> Result<Vec<String>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::suggest_tags(content_id, top_k).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tags
#[magnus::wrap(class = "Todozi")]
fn todozi_tags(tags: Vec<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tags(tags).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tags_advanced_search
#[magnus::wrap(class = "Todozi")]
fn todozi_tags_advanced_search(query: &str) -> Result<Vec<JsTag>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tags_advanced_search(query).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// task
#[magnus::wrap(class = "Todozi")]
fn todozi_task(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::task(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tasks
#[magnus::wrap(class = "Todozi")]
fn todozi_tasks(project_name: &str) -> Result<Vec<Task>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Projects::tasks(project_name).await.map(|items| items.into_iter().map(|item| item.to_ruby()).collect())
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tdz_cnt
#[magnus::wrap(class = "Todozi")]
fn todozi_tdz_cnt(content: &str, session_id: Option<&str>) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdz_cnt(content, session_id.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tdz_execute_command
#[magnus::wrap(class = "Todozi")]
fn todozi_tdz_execute_command(command: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdz_execute_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tdz_find
#[magnus::wrap(class = "Todozi")]
fn todozi_tdz_find(query: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Find::tdz_find(query).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tdz_parse_command
#[magnus::wrap(class = "Todozi")]
fn todozi_tdz_parse_command(input: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdz_parse_command(input).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tdzfp
#[magnus::wrap(class = "Todozi")]
fn todozi_tdzfp() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tdzfp().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// team_percentage
#[magnus::wrap(class = "Todozi")]
fn todozi_team_percentage() -> Result<f64, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::team_percentage().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// term
#[magnus::wrap(class = "Todozi")]
fn todozi_term(term: MemoryTerm) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::term(term).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// title
#[magnus::wrap(class = "Todozi")]
fn todozi_title() -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::title().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// to_context_string
#[magnus::wrap(class = "Todozi")]
fn todozi_to_context_string() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::to_context_string().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// to_ollama_format
#[magnus::wrap(class = "Todozi")]
fn todozi_to_ollama_format() -> Result<serde_json::Value, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::to_ollama_format().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// to_state_string
#[magnus::wrap(class = "Todozi")]
fn todozi_to_state_string() -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::to_state_string().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// todozi_begin
#[magnus::wrap(class = "Todozi")]
fn todozi_todozi_begin() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::todozi_begin().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// todozi_init
#[magnus::wrap(class = "Todozi")]
fn todozi_todozi_init() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::todozi_init().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// todozi_init_with_auto_registration
#[magnus::wrap(class = "Todozi")]
fn todozi_todozi_init_with_auto_registration() -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::todozi_init_with_auto_registration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// tool_count
#[magnus::wrap(class = "Todozi")]
fn todozi_tool_count() -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::tool_count().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// total_results
#[magnus::wrap(class = "Todozi")]
fn todozi_total_results() -> Result<usize, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::total_results().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// track_embedding_drift
#[magnus::wrap(class = "Todozi")]
fn todozi_track_embedding_drift(content_id: &str, current_text: &str) -> Result<DriftReport, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::track_embedding_drift(content_id, current_text).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// transform_shorthand_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_transform_shorthand_tags(message: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::transform_shorthand_tags(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// types
#[magnus::wrap(class = "Todozi")]
fn todozi_types() -> Result<&'static str, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::types().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// unregister
#[magnus::wrap(class = "Todozi")]
fn todozi_unregister(name: &str) -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::unregister(name).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update
#[magnus::wrap(class = "Todozi")]
fn todozi_update(updates: TaskUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update(updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_agent
#[magnus::wrap(class = "Todozi")]
fn todozi_update_agent(agent_id: &str, updates: AgentUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_agent(agent_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_agent_assignment_status
#[magnus::wrap(class = "Todozi")]
fn todozi_update_agent_assignment_status(agent_id: &str, task_id: &str, status: AssignmentStatus) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_agent_assignment_status(agent_id, task_id, status).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_agent_status
#[magnus::wrap(class = "Todozi")]
fn todozi_update_agent_status(agent_id: &str, status: AgentStatus) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_agent_status(agent_id, status).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_chunk_code
#[magnus::wrap(class = "Todozi")]
fn todozi_update_chunk_code(chunk_id: &str, code: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_chunk_code(chunk_id, code).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_chunk_tests
#[magnus::wrap(class = "Todozi")]
fn todozi_update_chunk_tests(chunk_id: &str, tests: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_chunk_tests(chunk_id, tests).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_config
#[magnus::wrap(class = "Todozi")]
fn todozi_update_config(config: Config) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_config(config).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_config_with_registration
#[magnus::wrap(class = "Todozi")]
fn todozi_update_config_with_registration(registration: RegistrationInfo) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_config_with_registration(registration).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_feeling
#[magnus::wrap(class = "Todozi")]
fn todozi_update_feeling(feeling: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_feeling(feeling).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_idea
#[magnus::wrap(class = "Todozi")]
fn todozi_update_idea(idea_id: &str, updates: IdeaUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_idea(idea_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_memory
#[magnus::wrap(class = "Todozi")]
fn todozi_update_memory(memory_id: &str, updates: MemoryUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_memory(memory_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_project
#[magnus::wrap(class = "Todozi")]
fn todozi_update_project(project: Project) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_project(project).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_registration_api_key
#[magnus::wrap(class = "Todozi")]
fn todozi_update_registration_api_key(api_key: &str) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_registration_api_key(api_key).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_registration_keys
#[magnus::wrap(class = "Todozi")]
fn todozi_update_registration_keys(api_key: &str, user_id: Option<String>, fingerprint: Option<String>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_registration_keys(api_key, user_id.as_deref(), fingerprint.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_reminder
#[magnus::wrap(class = "Todozi")]
fn todozi_update_reminder(reminder_id: &str, updates: ReminderUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_reminder(reminder_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_summary
#[magnus::wrap(class = "Todozi")]
fn todozi_update_summary(summary_id: &str, updates: SummaryUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_summary(summary_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_tag
#[magnus::wrap(class = "Todozi")]
fn todozi_update_tag(tag_id: &str, updates: TagUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_tag(tag_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_task
#[magnus::wrap(class = "Todozi")]
fn todozi_update_task(id: &str, updates: TaskUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task(id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_task_full
#[magnus::wrap(class = "Todozi")]
fn todozi_update_task_full(task_id: &str, updates: TaskUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task_full(task_id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_task_in_project
#[magnus::wrap(class = "Todozi")]
fn todozi_update_task_in_project(id: &str, updates: crate::models::TaskUpdate) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task_in_project(id, updates).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// update_task_status
#[magnus::wrap(class = "Todozi")]
fn todozi_update_task_status(task_id: &str, status: Status) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::update_task_status(task_id, status).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// urgent
#[magnus::wrap(class = "Todozi")]
fn todozi_urgent(action: &str) -> Result<String, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Tdz::urgent(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validate_embeddings
#[magnus::wrap(class = "Todozi")]
fn todozi_validate_embeddings() -> Result<ValidationReport, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_embeddings().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validate_migration
#[magnus::wrap(class = "Todozi")]
fn todozi_validate_migration() -> Result<bool, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_migration().await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validate_required_params
#[magnus::wrap(class = "Todozi")]
fn todozi_validate_required_params(kwargs: &str, serde_json: :Value>, required_params: &str) -> Result<Option<ToolResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_required_params(kwargs, serde_json, required_params).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validate_string_param
#[magnus::wrap(class = "Todozi")]
fn todozi_validate_string_param(value: &str, param_name: &str, min_length: usize, max_length: usize, pattern: Option<&str>) -> Result<Option<ToolResult>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_string_param(value, param_name, min_length, max_length, pattern.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validate_task_input
#[magnus::wrap(class = "Todozi")]
fn todozi_validate_task_input(action: &str, time: &str, priority: &str, project: &str, status: &str, assignee: Option<&str>, progress: Option<u8>) -> Result<(), magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_task_input(action, time, priority, project, status, assignee.as_deref(), progress.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validate_tdz_command
#[magnus::wrap(class = "Todozi")]
fn todozi_validate_tdz_command(command: &str) -> Result<JsCommandValidation, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validate_tdz_command(command).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// validation
#[magnus::wrap(class = "Todozi")]
fn todozi_validation(message: impl Into<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::validation(message).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// verbose
#[magnus::wrap(class = "Todozi")]
fn todozi_verbose(verbose: bool) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::verbose(verbose).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_action
#[magnus::wrap(class = "Todozi")]
fn todozi_with_action(action: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_action(action).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_assignee
#[magnus::wrap(class = "Todozi")]
fn todozi_with_assignee(assignee: Assignee) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_assignee(assignee).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_context
#[magnus::wrap(class = "Todozi")]
fn todozi_with_context(context: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_context(context).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_context_notes
#[magnus::wrap(class = "Todozi")]
fn todozi_with_context_notes(context_notes: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_context_notes(context_notes).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_dependencies
#[magnus::wrap(class = "Todozi")]
fn todozi_with_dependencies(dependencies: Vec<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_dependencies(dependencies).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_dry_run
#[magnus::wrap(class = "Todozi")]
fn todozi_with_dry_run(dry_run: bool) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_dry_run(dry_run).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_embedding_service
#[magnus::wrap(class = "Todozi")]
fn todozi_with_embedding_service(service: TodoziEmbeddingService) -> Result<PyResult<Self>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_embedding_service(service).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_embedding_service_option
#[magnus::wrap(class = "Todozi")]
fn todozi_with_embedding_service_option(service: Option<TodoziEmbeddingService>) -> Result<PyResult<Self>, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_embedding_service_option(service.as_deref()).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_force
#[magnus::wrap(class = "Todozi")]
fn todozi_with_force(force: bool) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_force(force).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_max_tokens
#[magnus::wrap(class = "Todozi")]
fn todozi_with_max_tokens(max_tokens: u32) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_max_tokens(max_tokens).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_parent_project
#[magnus::wrap(class = "Todozi")]
fn todozi_with_parent_project(parent_project: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_parent_project(parent_project).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_priority
#[magnus::wrap(class = "Todozi")]
fn todozi_with_priority(priority: Priority) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_priority(priority).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_progress
#[magnus::wrap(class = "Todozi")]
fn todozi_with_progress(progress: u8) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_progress(progress).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_shared_components
#[magnus::wrap(class = "Todozi")]
fn todozi_with_shared_components(config: Arc<Mutex<TodoziEmbeddingConfig>>, cache: Arc<Mutex<HashMap<String, embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>, embedding_models: Arc<Mutex<HashMap<String, tag_manager: Arc<Mutex<TagManager>>, storage: Arc<Mutex<Storage>>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_shared_components(config, cache, embedding_model, embedding_models, tag_manager, storage).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_status
#[magnus::wrap(class = "Todozi")]
fn todozi_with_status(status: Status) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_status(status).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_tags
#[magnus::wrap(class = "Todozi")]
fn todozi_with_tags(tags: Vec<String>) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_tags(tags).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_temperature
#[magnus::wrap(class = "Todozi")]
fn todozi_with_temperature(temperature: f32) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_temperature(temperature).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_time
#[magnus::wrap(class = "Todozi")]
fn todozi_with_time(time: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_time(time).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_user_id
#[magnus::wrap(class = "Todozi")]
fn todozi_with_user_id(user_id: &str) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_user_id(user_id).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

// with_verbose
#[magnus::wrap(class = "Todozi")]
fn todozi_with_verbose(verbose: bool) -> Result<Self, magnus::Error> {
    let runtime = get_runtime()?;
    runtime.block_on(async {
        Done::with_verbose(verbose).await
            .map_err(|e| magnus::Error::new(magnus::exception::runtime_error(), format!("{}", e)))
    })
}

#[magnus::init]
fn init() -> Result<(), MagnusError> {
    let class = Ruby::get().unwrap().define_class("Todozi", Ruby::get().unwrap().class_object())?;
    
    class.define_method("activate_api_key", magnus::function!(Todozi::activate_api_key, 1))?;
    
    class.define_method("activate_key", magnus::function!(Todozi::activate_key, 1))?;
    
    class.define_method("activate_reminder", magnus::function!(Todozi::activate_reminder, 1))?;
    
    class.define_method("active", magnus::function!(Todozi::active, 0))?;
    
    class.define_method("active_percentage", magnus::function!(Todozi::active_percentage, 0))?;
    
    class.define_method("add", magnus::function!(Todozi::add, 2))?;
    
    class.define_method("add_checklist_item", magnus::function!(Todozi::add_checklist_item, 1))?;
    
    class.define_method("add_chunk", magnus::function!(Todozi::add_chunk, 3))?;
    
    class.define_method("add_completed_module", magnus::function!(Todozi::add_completed_module, 1))?;
    
    class.define_method("add_dependency", magnus::function!(Todozi::add_dependency, 1))?;
    
    class.define_method("add_error_pattern", magnus::function!(Todozi::add_error_pattern, 1))?;
    
    class.define_method("add_function_signature", magnus::function!(Todozi::add_function_signature, 2))?;
    
    class.define_method("add_import", magnus::function!(Todozi::add_import, 1))?;
    
    class.define_method("add_item", magnus::function!(Todozi::add_item, 1))?;
    
    class.define_method("add_key", magnus::function!(Todozi::add_key, 1))?;
    
    class.define_method("add_pending_module", magnus::function!(Todozi::add_pending_module, 1))?;
    
    class.define_method("add_processed_action", magnus::function!(Todozi::add_processed_action, 4))?;
    
    class.define_method("add_queue_item", magnus::function!(Todozi::add_queue_item, 1))?;
    
    class.define_method("add_recent", magnus::function!(Todozi::add_recent, 1))?;
    
    class.define_method("add_recent_action", magnus::function!(Todozi::add_recent_action, 1))?;
    
    class.define_method("add_tag_relationship", magnus::function!(Todozi::add_tag_relationship, 2))?;
    
    class.define_method("add_tag_to_task", magnus::function!(Todozi::add_tag_to_task, 2))?;
    
    class.define_method("add_task", magnus::function!(Todozi::add_task, 1))?;
    
    class.define_method("add_task_emb", magnus::function!(Todozi::add_task_emb, 1))?;
    
    class.define_method("add_task_to_project", magnus::function!(Todozi::add_task_to_project, 1))?;
    
    class.define_method("add_to_task", magnus::function!(Todozi::add_to_task, 2))?;
    
    class.define_method("advanced_search", magnus::function!(Todozi::advanced_search, 1))?;
    
    class.define_method("advanced_search_with_filters", magnus::function!(Todozi::advanced_search_with_filters, 3))?;
    
    class.define_method("ai", magnus::function!(Todozi::ai, 1))?;
    
    class.define_method("ai_find", magnus::function!(Todozi::ai_find, 1))?;
    
    class.define_method("ai_search", magnus::function!(Todozi::ai_search, 1))?;
    
    class.define_method("ai_task", magnus::function!(Todozi::ai_task, 1))?;
    
    class.define_method("ai_tasks", magnus::function!(Todozi::ai_tasks, 1))?;
    
    class.define_method("all", magnus::function!(Todozi::all, 0))?;
    
    class.define_method("all_tasks", magnus::function!(Todozi::all_tasks, 0))?;
    
    class.define_method("analyze_code_quality", magnus::function!(Todozi::analyze_code_quality, 1))?;
    
    class.define_method("api", magnus::function!(Todozi::api, 1))?;
    
    class.define_method("api_key", magnus::function!(Todozi::api_key, 0))?;
    
    class.define_method("archive_project", magnus::function!(Todozi::archive_project, 1))?;
    
    class.define_method("as_str", magnus::function!(Todozi::as_str, 0))?;
    
    class.define_method("assign_task_to_agent", magnus::function!(Todozi::assign_task_to_agent, 3))?;
    
    class.define_method("auto_categorize_content", magnus::function!(Todozi::auto_categorize_content, 1))?;
    
    class.define_method("auto_label_clusters", magnus::function!(Todozi::auto_label_clusters, 1))?;
    
    class.define_method("backlog", magnus::function!(Todozi::backlog, 0))?;
    
    class.define_method("backup_embeddings", magnus::function!(Todozi::backup_embeddings, 1))?;
    
    class.define_method("batch_embed_content", magnus::function!(Todozi::batch_embed_content, 1))?;
    
    class.define_method("begin", magnus::function!(Todozi::begin, 1))?;
    
    class.define_method("breakthrough", magnus::function!(Todozi::breakthrough, 1))?;
    
    class.define_method("breakthrough_idea", magnus::function!(Todozi::breakthrough_idea, 1))?;
    
    class.define_method("breakthrough_percentage", magnus::function!(Todozi::breakthrough_percentage, 0))?;
    
    class.define_method("build_similarity_graph", magnus::function!(Todozi::build_similarity_graph, 1))?;
    
    class.define_method("bulk_create_tags", magnus::function!(Todozi::bulk_create_tags, 2))?;
    
    class.define_method("calculate_diversity", magnus::function!(Todozi::calculate_diversity, 1))?;
    
    class.define_method("capabilities", magnus::function!(Todozi::capabilities, 1))?;
    
    class.define_method("category", magnus::function!(Todozi::category, 1))?;
    
    class.define_method("chat", magnus::function!(Todozi::chat, 1))?;
    
    class.define_method("check_api_key_auth", magnus::function!(Todozi::check_api_key_auth, 2))?;
    
    class.define_method("check_folder_structure", magnus::function!(Todozi::check_folder_structure, 0))?;
    
    class.define_method("check_migration_status", magnus::function!(Todozi::check_migration_status, 0))?;
    
    class.define_method("cleanup_expired", magnus::function!(Todozi::cleanup_expired, 0))?;
    
    class.define_method("cleanup_legacy", magnus::function!(Todozi::cleanup_legacy, 0))?;
    
    class.define_method("clear_registration", magnus::function!(Todozi::clear_registration, 0))?;
    
    class.define_method("cli_add_task", magnus::function!(Todozi::cli_add_task, 2))?;
    
    class.define_method("cli_chat", magnus::function!(Todozi::cli_chat, 1))?;
    
    class.define_method("cli_clear_registration", magnus::function!(Todozi::cli_clear_registration, 0))?;
    
    class.define_method("cli_complete_task", magnus::function!(Todozi::cli_complete_task, 1))?;
    
    class.define_method("cli_create_backup", magnus::function!(Todozi::cli_create_backup, 0))?;
    
    class.define_method("cli_create_idea", magnus::function!(Todozi::cli_create_idea, 1))?;
    
    class.define_method("cli_create_memory", magnus::function!(Todozi::cli_create_memory, 3))?;
    
    class.define_method("cli_delete_task", magnus::function!(Todozi::cli_delete_task, 1))?;
    
    class.define_method("cli_fix_consistency", magnus::function!(Todozi::cli_fix_consistency, 0))?;
    
    class.define_method("cli_get_registration_status", magnus::function!(Todozi::cli_get_registration_status, 0))?;
    
    class.define_method("cli_list_backups", magnus::function!(Todozi::cli_list_backups, 0))?;
    
    class.define_method("cli_list_tasks", magnus::function!(Todozi::cli_list_tasks, 0))?;
    
    class.define_method("cli_register_with_server", magnus::function!(Todozi::cli_register_with_server, 1))?;
    
    class.define_method("cli_restore_backup", magnus::function!(Todozi::cli_restore_backup, 1))?;
    
    class.define_method("cli_search_tasks", magnus::function!(Todozi::cli_search_tasks, 1))?;
    
    class.define_method("cli_show_task", magnus::function!(Todozi::cli_show_task, 1))?;
    
    class.define_method("cli_update_task", magnus::function!(Todozi::cli_update_task, 4))?;
    
    class.define_method("cluster", magnus::function!(Todozi::cluster, 0))?;
    
    class.define_method("cluster_content", magnus::function!(Todozi::cluster_content, 0))?;
    
    class.define_method("cluster_similar_tasks", magnus::function!(Todozi::cluster_similar_tasks, 0))?;
    
    class.define_method("collab", magnus::function!(Todozi::collab, 1))?;
    
    class.define_method("collab_task", magnus::function!(Todozi::collab_task, 1))?;
    
    class.define_method("color", magnus::function!(Todozi::color, 1))?;
    
    class.define_method("compare_models", magnus::function!(Todozi::compare_models, 2))?;
    
    class.define_method("complete", magnus::function!(Todozi::complete, 1))?;
    
    class.define_method("complete_agent_assignment", magnus::function!(Todozi::complete_agent_assignment, 1))?;
    
    class.define_method("complete_task", magnus::function!(Todozi::complete_task, 1))?;
    
    class.define_method("complete_task_in_project", magnus::function!(Todozi::complete_task_in_project, 1))?;
    
    class.define_method("completion_rate", magnus::function!(Todozi::completion_rate, 0))?;
    
    class.define_method("config", magnus::function!(Todozi::config, 0))?;
    
    class.define_method("configure_display", magnus::function!(Todozi::configure_display, 1))?;
    
    class.define_method("configure_server", magnus::function!(Todozi::configure_server, 3))?;
    
    class.define_method("content", magnus::function!(Todozi::content, 1))?;
    
    class.define_method("context", magnus::function!(Todozi::context, 1))?;
    
    class.define_method("craft_embedding", magnus::function!(Todozi::craft_embedding, 1))?;
    
    class.define_method("create", magnus::function!(Todozi::create, 2))?;
    
    class.define_method("create_advanced_todozi_tools", magnus::function!(Todozi::create_advanced_todozi_tools, 1))?;
    
    class.define_method("create_agent", magnus::function!(Todozi::create_agent, 1))?;
    
    class.define_method("create_api_key", magnus::function!(Todozi::create_api_key, 0))?;
    
    class.define_method("create_api_key_with_user_id", magnus::function!(Todozi::create_api_key_with_user_id, 1))?;
    
    class.define_method("create_architect_agent", magnus::function!(Todozi::create_architect_agent, 0))?;
    
    class.define_method("create_backup", magnus::function!(Todozi::create_backup, 0))?;
    
    class.define_method("create_coder", magnus::function!(Todozi::create_coder, 0))?;
    
    class.define_method("create_comrad_agent", magnus::function!(Todozi::create_comrad_agent, 0))?;
    
    class.define_method("create_custom_agent", magnus::function!(Todozi::create_custom_agent, 7))?;
    
    class.define_method("create_default_agents", magnus::function!(Todozi::create_default_agents, 0))?;
    
    class.define_method("create_designer_agent", magnus::function!(Todozi::create_designer_agent, 0))?;
    
    class.define_method("create_detective_agent", magnus::function!(Todozi::create_detective_agent, 0))?;
    
    class.define_method("create_devops_agent", magnus::function!(Todozi::create_devops_agent, 0))?;
    
    class.define_method("create_embedding_service", magnus::function!(Todozi::create_embedding_service, 0))?;
    
    class.define_method("create_embedding_version", magnus::function!(Todozi::create_embedding_version, 2))?;
    
    class.define_method("create_error", magnus::function!(Todozi::create_error, 1))?;
    
    class.define_method("create_error_result", magnus::function!(Todozi::create_error_result, 5))?;
    
    class.define_method("create_filters", magnus::function!(Todozi::create_filters, 0))?;
    
    class.define_method("create_finisher_agent", magnus::function!(Todozi::create_finisher_agent, 0))?;
    
    class.define_method("create_framer_agent", magnus::function!(Todozi::create_framer_agent, 0))?;
    
    class.define_method("create_friend_agent", magnus::function!(Todozi::create_friend_agent, 0))?;
    
    class.define_method("create_grok_level_todozi_tools", magnus::function!(Todozi::create_grok_level_todozi_tools, 1))?;
    
    class.define_method("create_hoarder_agent", magnus::function!(Todozi::create_hoarder_agent, 0))?;
    
    class.define_method("create_idea", magnus::function!(Todozi::create_idea, 1))?;
    
    class.define_method("create_investigator_agent", magnus::function!(Todozi::create_investigator_agent, 0))?;
    
    class.define_method("create_mason_agent", magnus::function!(Todozi::create_mason_agent, 0))?;
    
    class.define_method("create_memory", magnus::function!(Todozi::create_memory, 1))?;
    
    class.define_method("create_nerd_agent", magnus::function!(Todozi::create_nerd_agent, 0))?;
    
    class.define_method("create_nun_agent", magnus::function!(Todozi::create_nun_agent, 0))?;
    
    class.define_method("create_overlord_agent", magnus::function!(Todozi::create_overlord_agent, 0))?;
    
    class.define_method("create_party_agent", magnus::function!(Todozi::create_party_agent, 0))?;
    
    class.define_method("create_planner_agent", magnus::function!(Todozi::create_planner_agent, 0))?;
    
    class.define_method("create_project", magnus::function!(Todozi::create_project, 2))?;
    
    class.define_method("create_recycler_agent", magnus::function!(Todozi::create_recycler_agent, 0))?;
    
    class.define_method("create_reminder", magnus::function!(Todozi::create_reminder, 1))?;
    
    class.define_method("create_search_options", magnus::function!(Todozi::create_search_options, 2))?;
    
    class.define_method("create_skeleton_agent", magnus::function!(Todozi::create_skeleton_agent, 0))?;
    
    class.define_method("create_snitch_agent", magnus::function!(Todozi::create_snitch_agent, 0))?;
    
    class.define_method("create_storage", magnus::function!(Todozi::create_storage, 0))?;
    
    class.define_method("create_success_result", magnus::function!(Todozi::create_success_result, 4))?;
    
    class.define_method("create_summary", magnus::function!(Todozi::create_summary, 1))?;
    
    class.define_method("create_tag", magnus::function!(Todozi::create_tag, 1))?;
    
    class.define_method("create_task", magnus::function!(Todozi::create_task, 5))?;
    
    class.define_method("create_task_filters", magnus::function!(Todozi::create_task_filters, 6))?;
    
    class.define_method("create_tdz_content_processor_tool", magnus::function!(Todozi::create_tdz_content_processor_tool, 1))?;
    
    class.define_method("create_tdz_tool", magnus::function!(Todozi::create_tdz_tool, 0))?;
    
    class.define_method("create_tester_agent", magnus::function!(Todozi::create_tester_agent, 0))?;
    
    class.define_method("create_todozi_tools", magnus::function!(Todozi::create_todozi_tools, 1))?;
    
    class.define_method("create_todozi_tools_with_embedding", magnus::function!(Todozi::create_todozi_tools_with_embedding, 2))?;
    
    class.define_method("create_tool_definition", magnus::function!(Todozi::create_tool_definition, 4))?;
    
    class.define_method("create_tool_definition_with_locks", magnus::function!(Todozi::create_tool_definition_with_locks, 5))?;
    
    class.define_method("create_tool_parameter", magnus::function!(Todozi::create_tool_parameter, 4))?;
    
    class.define_method("create_tool_parameter_with_default", magnus::function!(Todozi::create_tool_parameter_with_default, 5))?;
    
    class.define_method("create_tui_app", magnus::function!(Todozi::create_tui_app, 0))?;
    
    class.define_method("create_tuner_agent", magnus::function!(Todozi::create_tuner_agent, 0))?;
    
    class.define_method("create_update", magnus::function!(Todozi::create_update, 0))?;
    
    class.define_method("create_writer_agent", magnus::function!(Todozi::create_writer_agent, 0))?;
    
    class.define_method("critical_percentage", magnus::function!(Todozi::critical_percentage, 0))?;
    
    class.define_method("deactivate_api_key", magnus::function!(Todozi::deactivate_api_key, 1))?;
    
    class.define_method("deactivate_key", magnus::function!(Todozi::deactivate_key, 1))?;
    
    class.define_method("deep", magnus::function!(Todozi::deep, 1))?;
    
    class.define_method("deep_search", magnus::function!(Todozi::deep_search, 1))?;
    
    class.define_method("default_filters", magnus::function!(Todozi::default_filters, 0))?;
    
    class.define_method("default_update", magnus::function!(Todozi::default_update, 0))?;
    
    class.define_method("delete", magnus::function!(Todozi::delete, 1))?;
    
    class.define_method("delete_agent", magnus::function!(Todozi::delete_agent, 1))?;
    
    class.define_method("delete_agent_assignment", magnus::function!(Todozi::delete_agent_assignment, 2))?;
    
    class.define_method("delete_code_chunk", magnus::function!(Todozi::delete_code_chunk, 1))?;
    
    class.define_method("delete_error", magnus::function!(Todozi::delete_error, 1))?;
    
    class.define_method("delete_feeling", magnus::function!(Todozi::delete_feeling, 1))?;
    
    class.define_method("delete_idea", magnus::function!(Todozi::delete_idea, 1))?;
    
    class.define_method("delete_memory", magnus::function!(Todozi::delete_memory, 1))?;
    
    class.define_method("delete_project", magnus::function!(Todozi::delete_project, 1))?;
    
    class.define_method("delete_project_task_container", magnus::function!(Todozi::delete_project_task_container, 1))?;
    
    class.define_method("delete_reminder", magnus::function!(Todozi::delete_reminder, 1))?;
    
    class.define_method("delete_summary", magnus::function!(Todozi::delete_summary, 1))?;
    
    class.define_method("delete_tag", magnus::function!(Todozi::delete_tag, 1))?;
    
    class.define_method("delete_task", magnus::function!(Todozi::delete_task, 1))?;
    
    class.define_method("delete_task_from_project", magnus::function!(Todozi::delete_task_from_project, 1))?;
    
    class.define_method("delete_training_data", magnus::function!(Todozi::delete_training_data, 1))?;
    
    class.define_method("description", magnus::function!(Todozi::description, 1))?;
    
    class.define_method("detailed", magnus::function!(Todozi::detailed, 0))?;
    
    class.define_method("detailed_stats", magnus::function!(Todozi::detailed_stats, 0))?;
    
    class.define_method("display_task", magnus::function!(Todozi::display_task, 1))?;
    
    class.define_method("display_tasks", magnus::function!(Todozi::display_tasks, 1))?;
    
    class.define_method("do_it", magnus::function!(Todozi::do_it, 1))?;
    
    class.define_method("done", magnus::function!(Todozi::done, 1))?;
    
    class.define_method("done_api_key", magnus::function!(Todozi::done_api_key, 0))?;
    
    class.define_method("done_create_embedding_service", magnus::function!(Todozi::done_create_embedding_service, 0))?;
    
    class.define_method("done_create_storage", magnus::function!(Todozi::done_create_storage, 0))?;
    
    class.define_method("done_default_filters", magnus::function!(Todozi::done_default_filters, 0))?;
    
    class.define_method("done_default_update", magnus::function!(Todozi::done_default_update, 0))?;
    
    class.define_method("done_embedding_config", magnus::function!(Todozi::done_embedding_config, 0))?;
    
    class.define_method("done_embedding_service", magnus::function!(Todozi::done_embedding_service, 0))?;
    
    class.define_method("done_init", magnus::function!(Todozi::done_init, 0))?;
    
    class.define_method("done_process_chat", magnus::function!(Todozi::done_process_chat, 2))?;
    
    class.define_method("done_sample_task", magnus::function!(Todozi::done_sample_task, 0))?;
    
    class.define_method("done_storage", magnus::function!(Todozi::done_storage, 0))?;
    
    class.define_method("done_types", magnus::function!(Todozi::done_types, 0))?;
    
    class.define_method("dry_run", magnus::function!(Todozi::dry_run, 1))?;
    
    class.define_method("easy_done", magnus::function!(Todozi::easy_done, 1))?;
    
    class.define_method("easy_find", magnus::function!(Todozi::easy_find, 1))?;
    
    class.define_method("easy_idea", magnus::function!(Todozi::easy_idea, 1))?;
    
    class.define_method("easy_remember", magnus::function!(Todozi::easy_remember, 1))?;
    
    class.define_method("embed", magnus::function!(Todozi::embed, 1))?;
    
    class.define_method("embed_idea", magnus::function!(Todozi::embed_idea, 1))?;
    
    class.define_method("embed_memory", magnus::function!(Todozi::embed_memory, 1))?;
    
    class.define_method("embed_stats", magnus::function!(Todozi::embed_stats, 0))?;
    
    class.define_method("embed_tag", magnus::function!(Todozi::embed_tag, 1))?;
    
    class.define_method("embed_task", magnus::function!(Todozi::embed_task, 1))?;
    
    class.define_method("embedding_config", magnus::function!(Todozi::embedding_config, 0))?;
    
    class.define_method("embedding_service", magnus::function!(Todozi::embedding_service, 0))?;
    
    class.define_method("encode", magnus::function!(Todozi::encode, 1))?;
    
    class.define_method("end_queue_session", magnus::function!(Todozi::end_queue_session, 1))?;
    
    class.define_method("end_session", magnus::function!(Todozi::end_session, 1))?;
    
    class.define_method("ensure_folder_structure", magnus::function!(Todozi::ensure_folder_structure, 0))?;
    
    class.define_method("ensure_todozi_initialized", magnus::function!(Todozi::ensure_todozi_initialized, 0))?;
    
    class.define_method("error", magnus::function!(Todozi::error, 2))?;
    
    class.define_method("example", magnus::function!(Todozi::example, 0))?;
    
    class.define_method("example_usage", magnus::function!(Todozi::example_usage, 0))?;
    
    class.define_method("execute_task", magnus::function!(Todozi::execute_task, 2))?;
    
    class.define_method("execute_tdz_command", magnus::function!(Todozi::execute_tdz_command, 3))?;
    
    class.define_method("execute_todozi_tool_delegated", magnus::function!(Todozi::execute_todozi_tool_delegated, 1))?;
    
    class.define_method("execute_tool", magnus::function!(Todozi::execute_tool, 3))?;
    
    class.define_method("explain_search_result", magnus::function!(Todozi::explain_search_result, 2))?;
    
    class.define_method("export_diagnostics", magnus::function!(Todozi::export_diagnostics, 0))?;
    
    class.define_method("export_embedded_tasks_hlx", magnus::function!(Todozi::export_embedded_tasks_hlx, 1))?;
    
    class.define_method("export_for_fine_tuning", magnus::function!(Todozi::export_for_fine_tuning, 1))?;
    
    class.define_method("export_to_format", magnus::function!(Todozi::export_to_format, 1))?;
    
    class.define_method("extract_content", magnus::function!(Todozi::extract_content, 4))?;
    
    class.define_method("extract_content_from_text", magnus::function!(Todozi::extract_content_from_text, 2))?;
    
    class.define_method("extract_task_actions", magnus::function!(Todozi::extract_task_actions, 1))?;
    
    class.define_method("extract_tasks", magnus::function!(Todozi::extract_tasks, 2))?;
    
    class.define_method("fast", magnus::function!(Todozi::fast, 1))?;
    
    class.define_method("fast_search", magnus::function!(Todozi::fast_search, 1))?;
    
    class.define_method("filtered_semantic_search", magnus::function!(Todozi::filtered_semantic_search, 3))?;
    
    class.define_method("find", magnus::function!(Todozi::find, 1))?;
    
    class.define_method("find_best_agent", magnus::function!(Todozi::find_best_agent, 2))?;
    
    class.define_method("find_by_tag", magnus::function!(Todozi::find_by_tag, 1))?;
    
    class.define_method("find_cross_content_relationships", magnus::function!(Todozi::find_cross_content_relationships, 3))?;
    
    class.define_method("find_ideas", magnus::function!(Todozi::find_ideas, 1))?;
    
    class.define_method("find_memories", magnus::function!(Todozi::find_memories, 1))?;
    
    class.define_method("find_outliers", magnus::function!(Todozi::find_outliers, 2))?;
    
    class.define_method("find_similar_tags", magnus::function!(Todozi::find_similar_tags, 2))?;
    
    class.define_method("find_similar_tasks", magnus::function!(Todozi::find_similar_tasks, 2))?;
    
    class.define_method("find_tasks", magnus::function!(Todozi::find_tasks, 1))?;
    
    class.define_method("find_tasks_ai", magnus::function!(Todozi::find_tasks_ai, 1))?;
    
    class.define_method("find_tdz", magnus::function!(Todozi::find_tdz, 1))?;
    
    class.define_method("find_todozi", magnus::function!(Todozi::find_todozi, 1))?;
    
    class.define_method("fix_completed_tasks_consistency", magnus::function!(Todozi::fix_completed_tasks_consistency, 0))?;
    
    class.define_method("fix_task_consistency", magnus::function!(Todozi::fix_task_consistency, 0))?;
    
    class.define_method("force_overwrite", magnus::function!(Todozi::force_overwrite, 1))?;
    
    class.define_method("format_duration", magnus::function!(Todozi::format_duration, 2))?;
    
    class.define_method("format_error_with_context", magnus::function!(Todozi::format_error_with_context, 2))?;
    
    class.define_method("format_project_stats", magnus::function!(Todozi::format_project_stats, 3))?;
    
    class.define_method("format_task", magnus::function!(Todozi::format_task, 1))?;
    
    class.define_method("format_task_list", magnus::function!(Todozi::format_task_list, 1))?;
    
    class.define_method("format_task_with_emojis", magnus::function!(Todozi::format_task_with_emojis, 1))?;
    
    class.define_method("format_time_estimate", magnus::function!(Todozi::format_time_estimate, 1))?;
    
    class.define_method("fuzzy_search", magnus::function!(Todozi::fuzzy_search, 2))?;
    
    class.define_method("generate_embedding", magnus::function!(Todozi::generate_embedding, 1))?;
    
    class.define_method("generate_embeddings_batch", magnus::function!(Todozi::generate_embeddings_batch, 1))?;
    
    class.define_method("generate_task_embedding", magnus::function!(Todozi::generate_task_embedding, 1))?;
    
    class.define_method("get", magnus::function!(Todozi::get, 1))?;
    
    class.define_method("get_active_items", magnus::function!(Todozi::get_active_items, 0))?;
    
    class.define_method("get_active_keys", magnus::function!(Todozi::get_active_keys, 0))?;
    
    class.define_method("get_active_reminders", magnus::function!(Todozi::get_active_reminders, 0))?;
    
    class.define_method("get_active_sessions", magnus::function!(Todozi::get_active_sessions, 0))?;
    
    class.define_method("get_agent", magnus::function!(Todozi::get_agent, 1))?;
    
    class.define_method("get_agent_assignments", magnus::function!(Todozi::get_agent_assignments, 1))?;
    
    class.define_method("get_agent_assignments_dir", magnus::function!(Todozi::get_agent_assignments_dir, 1))?;
    
    class.define_method("get_agent_statistics", magnus::function!(Todozi::get_agent_statistics, 0))?;
    
    class.define_method("get_agents_by_capability", magnus::function!(Todozi::get_agents_by_capability, 1))?;
    
    class.define_method("get_agents_by_specialization", magnus::function!(Todozi::get_agents_by_specialization, 1))?;
    
    class.define_method("get_agents_dir", magnus::function!(Todozi::get_agents_dir, 0))?;
    
    class.define_method("get_agents_with_assignments", magnus::function!(Todozi::get_agents_with_assignments, 0))?;
    
    class.define_method("get_ai_tasks", magnus::function!(Todozi::get_ai_tasks, 0))?;
    
    class.define_method("get_all_active_tasks", magnus::function!(Todozi::get_all_active_tasks, 0))?;
    
    class.define_method("get_all_agents", magnus::function!(Todozi::get_all_agents, 0))?;
    
    class.define_method("get_all_categories", magnus::function!(Todozi::get_all_categories, 0))?;
    
    class.define_method("get_all_completed_tasks", magnus::function!(Todozi::get_all_completed_tasks, 0))?;
    
    class.define_method("get_all_ideas", magnus::function!(Todozi::get_all_ideas, 0))?;
    
    class.define_method("get_all_items", magnus::function!(Todozi::get_all_items, 0))?;
    
    class.define_method("get_all_keys", magnus::function!(Todozi::get_all_keys, 0))?;
    
    class.define_method("get_all_memories", magnus::function!(Todozi::get_all_memories, 0))?;
    
    class.define_method("get_all_reminders", magnus::function!(Todozi::get_all_reminders, 0))?;
    
    class.define_method("get_all_summaries", magnus::function!(Todozi::get_all_summaries, 0))?;
    
    class.define_method("get_all_tags", magnus::function!(Todozi::get_all_tags, 0))?;
    
    class.define_method("get_all_tasks", magnus::function!(Todozi::get_all_tasks, 0))?;
    
    class.define_method("get_all_tools", magnus::function!(Todozi::get_all_tools, 0))?;
    
    class.define_method("get_api_key", magnus::function!(Todozi::get_api_key, 1))?;
    
    class.define_method("get_api_key_by_public", magnus::function!(Todozi::get_api_key_by_public, 1))?;
    
    class.define_method("get_assignee_emoji", magnus::function!(Todozi::get_assignee_emoji, 1))?;
    
    class.define_method("get_assignments_dir", magnus::function!(Todozi::get_assignments_dir, 0))?;
    
    class.define_method("get_available_agents", magnus::function!(Todozi::get_available_agents, 0))?;
    
    class.define_method("get_backlog_items", magnus::function!(Todozi::get_backlog_items, 0))?;
    
    class.define_method("get_breakthrough_ideas", magnus::function!(Todozi::get_breakthrough_ideas, 0))?;
    
    class.define_method("get_chunk", magnus::function!(Todozi::get_chunk, 1))?;
    
    class.define_method("get_chunk_mut", magnus::function!(Todozi::get_chunk_mut, 1))?;
    
    class.define_method("get_chunk_status", magnus::function!(Todozi::get_chunk_status, 1))?;
    
    class.define_method("get_chunking_level_info", magnus::function!(Todozi::get_chunking_level_info, 1))?;
    
    class.define_method("get_chunking_levels", magnus::function!(Todozi::get_chunking_levels, 0))?;
    
    class.define_method("get_chunks_by_level", magnus::function!(Todozi::get_chunks_by_level, 1))?;
    
    class.define_method("get_chunks_dir", magnus::function!(Todozi::get_chunks_dir, 0))?;
    
    class.define_method("get_collaborative_tasks", magnus::function!(Todozi::get_collaborative_tasks, 0))?;
    
    class.define_method("get_command_documentation", magnus::function!(Todozi::get_command_documentation, 1))?;
    
    class.define_method("get_complete_items", magnus::function!(Todozi::get_complete_items, 0))?;
    
    class.define_method("get_critical_memories", magnus::function!(Todozi::get_critical_memories, 0))?;
    
    class.define_method("get_current_duration", magnus::function!(Todozi::get_current_duration, 0))?;
    
    class.define_method("get_default_model", magnus::function!(Todozi::get_default_model, 0))?;
    
    class.define_method("get_dependency_chain", magnus::function!(Todozi::get_dependency_chain, 1))?;
    
    class.define_method("get_embedding_statistics", magnus::function!(Todozi::get_embedding_statistics, 0))?;
    
    class.define_method("get_emotional_memories", magnus::function!(Todozi::get_emotional_memories, 1))?;
    
    class.define_method("get_enabled_tools", magnus::function!(Todozi::get_enabled_tools, 0))?;
    
    class.define_method("get_error_details", magnus::function!(Todozi::get_error_details, 1))?;
    
    class.define_method("get_errors_dir", magnus::function!(Todozi::get_errors_dir, 0))?;
    
    class.define_method("get_filtered_tasks", magnus::function!(Todozi::get_filtered_tasks, 1))?;
    
    class.define_method("get_formatted_prompt", magnus::function!(Todozi::get_formatted_prompt, 1))?;
    
    class.define_method("get_high_priority_summaries", magnus::function!(Todozi::get_high_priority_summaries, 0))?;
    
    class.define_method("get_human_memories", magnus::function!(Todozi::get_human_memories, 0))?;
    
    class.define_method("get_human_tasks", magnus::function!(Todozi::get_human_tasks, 0))?;
    
    class.define_method("get_idea", magnus::function!(Todozi::get_idea, 1))?;
    
    class.define_method("get_idea_statistics", magnus::function!(Todozi::get_idea_statistics, 0))?;
    
    class.define_method("get_ideas_by_importance", magnus::function!(Todozi::get_ideas_by_importance, 1))?;
    
    class.define_method("get_ideas_by_share_level", magnus::function!(Todozi::get_ideas_by_share_level, 1))?;
    
    class.define_method("get_ideas_by_tag", magnus::function!(Todozi::get_ideas_by_tag, 1))?;
    
    class.define_method("get_ideas_dir", magnus::function!(Todozi::get_ideas_dir, 0))?;
    
    class.define_method("get_item", magnus::function!(Todozi::get_item, 1))?;
    
    class.define_method("get_item_mut", magnus::function!(Todozi::get_item_mut, 1))?;
    
    class.define_method("get_items_by_status", magnus::function!(Todozi::get_items_by_status, 1))?;
    
    class.define_method("get_key", magnus::function!(Todozi::get_key, 1))?;
    
    class.define_method("get_key_by_public", magnus::function!(Todozi::get_key_by_public, 1))?;
    
    class.define_method("get_long_term_memories", magnus::function!(Todozi::get_long_term_memories, 0))?;
    
    class.define_method("get_memories_by_importance", magnus::function!(Todozi::get_memories_by_importance, 1))?;
    
    class.define_method("get_memories_by_tag", magnus::function!(Todozi::get_memories_by_tag, 1))?;
    
    class.define_method("get_memories_by_term", magnus::function!(Todozi::get_memories_by_term, 1))?;
    
    class.define_method("get_memories_by_type", magnus::function!(Todozi::get_memories_by_type, 1))?;
    
    class.define_method("get_memories_dir", magnus::function!(Todozi::get_memories_dir, 0))?;
    
    class.define_method("get_memory", magnus::function!(Todozi::get_memory, 1))?;
    
    class.define_method("get_memory_statistics", magnus::function!(Todozi::get_memory_statistics, 0))?;
    
    class.define_method("get_most_used_tags", magnus::function!(Todozi::get_most_used_tags, 1))?;
    
    class.define_method("get_next_chunk_to_work_on", magnus::function!(Todozi::get_next_chunk_to_work_on, 0))?;
    
    class.define_method("get_or_generate_embedding", magnus::function!(Todozi::get_or_generate_embedding, 4))?;
    
    class.define_method("get_overdue_reminders", magnus::function!(Todozi::get_overdue_reminders, 0))?;
    
    class.define_method("get_pending_reminders", magnus::function!(Todozi::get_pending_reminders, 0))?;
    
    class.define_method("get_priority_emoji", magnus::function!(Todozi::get_priority_emoji, 1))?;
    
    class.define_method("get_private_ideas", magnus::function!(Todozi::get_private_ideas, 0))?;
    
    class.define_method("get_processor_state", magnus::function!(Todozi::get_processor_state, 0))?;
    
    class.define_method("get_project", magnus::function!(Todozi::get_project, 1))?;
    
    class.define_method("get_project_stats", magnus::function!(Todozi::get_project_stats, 1))?;
    
    class.define_method("get_project_summary", magnus::function!(Todozi::get_project_summary, 0))?;
    
    class.define_method("get_project_tasks", magnus::function!(Todozi::get_project_tasks, 1))?;
    
    class.define_method("get_project_tasks_dir", magnus::function!(Todozi::get_project_tasks_dir, 0))?;
    
    class.define_method("get_public_ideas", magnus::function!(Todozi::get_public_ideas, 0))?;
    
    class.define_method("get_queue_item", magnus::function!(Todozi::get_queue_item, 1))?;
    
    class.define_method("get_queue_session", magnus::function!(Todozi::get_queue_session, 1))?;
    
    class.define_method("get_queue_statistics", magnus::function!(Todozi::get_queue_statistics, 0))?;
    
    class.define_method("get_ready_chunks", magnus::function!(Todozi::get_ready_chunks, 0))?;
    
    class.define_method("get_recent_actions", magnus::function!(Todozi::get_recent_actions, 1))?;
    
    class.define_method("get_recent_ideas", magnus::function!(Todozi::get_recent_ideas, 1))?;
    
    class.define_method("get_recent_memories", magnus::function!(Todozi::get_recent_memories, 1))?;
    
    class.define_method("get_recent_reminders", magnus::function!(Todozi::get_recent_reminders, 1))?;
    
    class.define_method("get_recent_summaries", magnus::function!(Todozi::get_recent_summaries, 1))?;
    
    class.define_method("get_recent_tags", magnus::function!(Todozi::get_recent_tags, 1))?;
    
    class.define_method("get_registration_info", magnus::function!(Todozi::get_registration_info, 0))?;
    
    class.define_method("get_related_tags", magnus::function!(Todozi::get_related_tags, 1))?;
    
    class.define_method("get_reminder", magnus::function!(Todozi::get_reminder, 1))?;
    
    class.define_method("get_reminder_statistics", magnus::function!(Todozi::get_reminder_statistics, 0))?;
    
    class.define_method("get_reminders_by_priority", magnus::function!(Todozi::get_reminders_by_priority, 1))?;
    
    class.define_method("get_reminders_by_status", magnus::function!(Todozi::get_reminders_by_status, 1))?;
    
    class.define_method("get_reminders_by_tag", magnus::function!(Todozi::get_reminders_by_tag, 1))?;
    
    class.define_method("get_reminders_due_soon", magnus::function!(Todozi::get_reminders_due_soon, 1))?;
    
    class.define_method("get_search_analytics", magnus::function!(Todozi::get_search_analytics, 0))?;
    
    class.define_method("get_search_history", magnus::function!(Todozi::get_search_history, 1))?;
    
    class.define_method("get_search_suggestions", magnus::function!(Todozi::get_search_suggestions, 2))?;
    
    class.define_method("get_secret_memories", magnus::function!(Todozi::get_secret_memories, 0))?;
    
    class.define_method("get_server_status", magnus::function!(Todozi::get_server_status, 0))?;
    
    class.define_method("get_session", magnus::function!(Todozi::get_session, 1))?;
    
    class.define_method("get_short_term_memories", magnus::function!(Todozi::get_short_term_memories, 0))?;
    
    class.define_method("get_stats", magnus::function!(Todozi::get_stats, 0))?;
    
    class.define_method("get_status_emoji", magnus::function!(Todozi::get_status_emoji, 1))?;
    
    class.define_method("get_storage_dir", magnus::function!(Todozi::get_storage_dir, 0))?;
    
    class.define_method("get_suggestions", magnus::function!(Todozi::get_suggestions, 2))?;
    
    class.define_method("get_summaries_by_priority", magnus::function!(Todozi::get_summaries_by_priority, 1))?;
    
    class.define_method("get_summaries_by_tag", magnus::function!(Todozi::get_summaries_by_tag, 1))?;
    
    class.define_method("get_summary", magnus::function!(Todozi::get_summary, 1))?;
    
    class.define_method("get_summary_statistics", magnus::function!(Todozi::get_summary_statistics, 0))?;
    
    class.define_method("get_tag", magnus::function!(Todozi::get_tag, 1))?;
    
    class.define_method("get_tag_by_name", magnus::function!(Todozi::get_tag_by_name, 1))?;
    
    class.define_method("get_tag_statistics", magnus::function!(Todozi::get_tag_statistics, 0))?;
    
    class.define_method("get_tags_by_category", magnus::function!(Todozi::get_tags_by_category, 1))?;
    
    class.define_method("get_task", magnus::function!(Todozi::get_task, 1))?;
    
    class.define_method("get_task_assignments", magnus::function!(Todozi::get_task_assignments, 1))?;
    
    class.define_method("get_task_from_any_project", magnus::function!(Todozi::get_task_from_any_project, 1))?;
    
    class.define_method("get_task_from_project", magnus::function!(Todozi::get_task_from_project, 2))?;
    
    class.define_method("get_task_mut", magnus::function!(Todozi::get_task_mut, 1))?;
    
    class.define_method("get_tasks_dir", magnus::function!(Todozi::get_tasks_dir, 0))?;
    
    class.define_method("get_tdz_api_key", magnus::function!(Todozi::get_tdz_api_key, 0))?;
    
    class.define_method("get_team_ideas", magnus::function!(Todozi::get_team_ideas, 0))?;
    
    class.define_method("get_tool", magnus::function!(Todozi::get_tool, 1))?;
    
    class.define_method("get_tool_definitions", magnus::function!(Todozi::get_tool_definitions, 0))?;
    
    class.define_method("get_training_dir", magnus::function!(Todozi::get_training_dir, 0))?;
    
    class.define_method("get_tsne_coordinates", magnus::function!(Todozi::get_tsne_coordinates, 2))?;
    
    class.define_method("get_unresolved_errors", magnus::function!(Todozi::get_unresolved_errors, 0))?;
    
    class.define_method("get_version_history", magnus::function!(Todozi::get_version_history, 1))?;
    
    class.define_method("handle_add_command", magnus::function!(Todozi::handle_add_command, 1))?;
    
    class.define_method("handle_agent_command", magnus::function!(Todozi::handle_agent_command, 1))?;
    
    class.define_method("handle_ai_commands", magnus::function!(Todozi::handle_ai_commands, 2))?;
    
    class.define_method("handle_api_command", magnus::function!(Todozi::handle_api_command, 1))?;
    
    class.define_method("handle_chat_command", magnus::function!(Todozi::handle_chat_command, 1))?;
    
    class.define_method("handle_emb_command", magnus::function!(Todozi::handle_emb_command, 1))?;
    
    class.define_method("handle_error_command", magnus::function!(Todozi::handle_error_command, 1))?;
    
    class.define_method("handle_extract_command", magnus::function!(Todozi::handle_extract_command, 4))?;
    
    class.define_method("handle_idea_command", magnus::function!(Todozi::handle_idea_command, 1))?;
    
    class.define_method("handle_ind_command", magnus::function!(Todozi::handle_ind_command, 0))?;
    
    class.define_method("handle_list_backups_command", magnus::function!(Todozi::handle_list_backups_command, 0))?;
    
    class.define_method("handle_list_command", magnus::function!(Todozi::handle_list_command, 1))?;
    
    class.define_method("handle_memory_command", magnus::function!(Todozi::handle_memory_command, 1))?;
    
    class.define_method("handle_project_command", magnus::function!(Todozi::handle_project_command, 1))?;
    
    class.define_method("handle_queue_command", magnus::function!(Todozi::handle_queue_command, 1))?;
    
    class.define_method("handle_search_all_command", magnus::function!(Todozi::handle_search_all_command, 1))?;
    
    class.define_method("handle_search_command", magnus::function!(Todozi::handle_search_command, 1))?;
    
    class.define_method("handle_server_command", magnus::function!(Todozi::handle_server_command, 1))?;
    
    class.define_method("handle_show_command", magnus::function!(Todozi::handle_show_command, 1))?;
    
    class.define_method("handle_stats_command", magnus::function!(Todozi::handle_stats_command, 1))?;
    
    class.define_method("handle_strategy_command", magnus::function!(Todozi::handle_strategy_command, 4))?;
    
    class.define_method("handle_train_command", magnus::function!(Todozi::handle_train_command, 1))?;
    
    class.define_method("handle_update_command", magnus::function!(Todozi::handle_update_command, 11))?;
    
    class.define_method("has_capability", magnus::function!(Todozi::has_capability, 1))?;
    
    class.define_method("has_results", magnus::function!(Todozi::has_results, 0))?;
    
    class.define_method("has_specialization", magnus::function!(Todozi::has_specialization, 1))?;
    
    class.define_method("has_tool", magnus::function!(Todozi::has_tool, 1))?;
    
    class.define_method("hash_project_name", magnus::function!(Todozi::hash_project_name, 1))?;
    
    class.define_method("hierarchical_clustering", magnus::function!(Todozi::hierarchical_clustering, 2))?;
    
    class.define_method("high", magnus::function!(Todozi::high, 1))?;
    
    class.define_method("high_priority_percentage", magnus::function!(Todozi::high_priority_percentage, 0))?;
    
    class.define_method("human", magnus::function!(Todozi::human, 1))?;
    
    class.define_method("human_task", magnus::function!(Todozi::human_task, 1))?;
    
    class.define_method("hybrid_search", magnus::function!(Todozi::hybrid_search, 5))?;
    
    class.define_method("idea", magnus::function!(Todozi::idea, 1))?;
    
    class.define_method("ideate", magnus::function!(Todozi::ideate, 1))?;
    
    class.define_method("import_from_format", magnus::function!(Todozi::import_from_format, 2))?;
    
    class.define_method("importance", magnus::function!(Todozi::importance, 1))?;
    
    class.define_method("important", magnus::function!(Todozi::important, 3))?;
    
    class.define_method("important_memory", magnus::function!(Todozi::important_memory, 3))?;
    
    class.define_method("increment_tag_usage", magnus::function!(Todozi::increment_tag_usage, 1))?;
    
    class.define_method("init", magnus::function!(Todozi::init, 0))?;
    
    class.define_method("init_storage", magnus::function!(Todozi::init_storage, 0))?;
    
    class.define_method("init_with_auto_registration", magnus::function!(Todozi::init_with_auto_registration, 0))?;
    
    class.define_method("initialize", magnus::function!(Todozi::initialize, 0))?;
    
    class.define_method("initialize_grok_level_todozi_system", magnus::function!(Todozi::initialize_grok_level_todozi_system, 0))?;
    
    class.define_method("initialize_grok_level_todozi_system_with_embedding", magnus::function!(Todozi::initialize_grok_level_todozi_system_with_embedding, 1))?;
    
    class.define_method("initialize_tdz_content_processor", magnus::function!(Todozi::initialize_tdz_content_processor, 0))?;
    
    class.define_method("initialize_tdz_processor", magnus::function!(Todozi::initialize_tdz_processor, 0))?;
    
    class.define_method("interactive_create_task", magnus::function!(Todozi::interactive_create_task, 0))?;
    
    class.define_method("io", magnus::function!(Todozi::io, 1))?;
    
    class.define_method("is_active", magnus::function!(Todozi::is_active, 0))?;
    
    class.define_method("is_admin", magnus::function!(Todozi::is_admin, 2))?;
    
    class.define_method("is_available", magnus::function!(Todozi::is_available, 0))?;
    
    class.define_method("is_backlog", magnus::function!(Todozi::is_backlog, 0))?;
    
    class.define_method("is_complete", magnus::function!(Todozi::is_complete, 0))?;
    
    class.define_method("is_completed", magnus::function!(Todozi::is_completed, 0))?;
    
    class.define_method("is_empty", magnus::function!(Todozi::is_empty, 0))?;
    
    class.define_method("is_overdue", magnus::function!(Todozi::is_overdue, 0))?;
    
    class.define_method("is_registered", magnus::function!(Todozi::is_registered, 0))?;
    
    class.define_method("keyword_search", magnus::function!(Todozi::keyword_search, 1))?;
    
    class.define_method("keyword_tasks", magnus::function!(Todozi::keyword_tasks, 1))?;
    
    class.define_method("launch_gui", magnus::function!(Todozi::launch_gui, 0))?;
    
    class.define_method("launch_tui", magnus::function!(Todozi::launch_tui, 0))?;
    
    class.define_method("len", magnus::function!(Todozi::len, 0))?;
    
    class.define_method("list", magnus::function!(Todozi::list, 0))?;
    
    class.define_method("list_active_api_keys", magnus::function!(Todozi::list_active_api_keys, 0))?;
    
    class.define_method("list_active_items", magnus::function!(Todozi::list_active_items, 0))?;
    
    class.define_method("list_agent_assignments", magnus::function!(Todozi::list_agent_assignments, 1))?;
    
    class.define_method("list_agents", magnus::function!(Todozi::list_agents, 0))?;
    
    class.define_method("list_all_agent_assignments", magnus::function!(Todozi::list_all_agent_assignments, 0))?;
    
    class.define_method("list_api_keys", magnus::function!(Todozi::list_api_keys, 0))?;
    
    class.define_method("list_available_agents", magnus::function!(Todozi::list_available_agents, 0))?;
    
    class.define_method("list_available_commands", magnus::function!(Todozi::list_available_commands, 0))?;
    
    class.define_method("list_available_tools", magnus::function!(Todozi::list_available_tools, 0))?;
    
    class.define_method("list_backlog_items", magnus::function!(Todozi::list_backlog_items, 0))?;
    
    class.define_method("list_backups", magnus::function!(Todozi::list_backups, 0))?;
    
    class.define_method("list_code_chunks", magnus::function!(Todozi::list_code_chunks, 0))?;
    
    class.define_method("list_complete_items", magnus::function!(Todozi::list_complete_items, 0))?;
    
    class.define_method("list_errors", magnus::function!(Todozi::list_errors, 0))?;
    
    class.define_method("list_feelings", magnus::function!(Todozi::list_feelings, 0))?;
    
    class.define_method("list_ideas", magnus::function!(Todozi::list_ideas, 0))?;
    
    class.define_method("list_memories", magnus::function!(Todozi::list_memories, 0))?;
    
    class.define_method("list_project_task_containers", magnus::function!(Todozi::list_project_task_containers, 0))?;
    
    class.define_method("list_projects", magnus::function!(Todozi::list_projects, 0))?;
    
    class.define_method("list_queue_items", magnus::function!(Todozi::list_queue_items, 0))?;
    
    class.define_method("list_queue_items_by_status", magnus::function!(Todozi::list_queue_items_by_status, 1))?;
    
    class.define_method("list_reminders", magnus::function!(Todozi::list_reminders, 0))?;
    
    class.define_method("list_summaries", magnus::function!(Todozi::list_summaries, 0))?;
    
    class.define_method("list_tasks", magnus::function!(Todozi::list_tasks, 0))?;
    
    class.define_method("list_tasks_across_projects", magnus::function!(Todozi::list_tasks_across_projects, 1))?;
    
    class.define_method("list_tasks_in_project", magnus::function!(Todozi::list_tasks_in_project, 2))?;
    
    class.define_method("list_training_data", magnus::function!(Todozi::list_training_data, 0))?;
    
    class.define_method("load", magnus::function!(Todozi::load, 2))?;
    
    class.define_method("load_additional_model", magnus::function!(Todozi::load_additional_model, 2))?;
    
    class.define_method("load_agent", magnus::function!(Todozi::load_agent, 1))?;
    
    class.define_method("load_agent_assignment", magnus::function!(Todozi::load_agent_assignment, 2))?;
    
    class.define_method("load_agents", magnus::function!(Todozi::load_agents, 0))?;
    
    class.define_method("load_api_key_collection", magnus::function!(Todozi::load_api_key_collection, 0))?;
    
    class.define_method("load_api_keys", magnus::function!(Todozi::load_api_keys, 0))?;
    
    class.define_method("load_code_chunk", magnus::function!(Todozi::load_code_chunk, 1))?;
    
    class.define_method("load_config", magnus::function!(Todozi::load_config, 0))?;
    
    class.define_method("load_error", magnus::function!(Todozi::load_error, 1))?;
    
    class.define_method("load_extended_data", magnus::function!(Todozi::load_extended_data, 0))?;
    
    class.define_method("load_feeling", magnus::function!(Todozi::load_feeling, 1))?;
    
    class.define_method("load_idea", magnus::function!(Todozi::load_idea, 1))?;
    
    class.define_method("load_memory", magnus::function!(Todozi::load_memory, 1))?;
    
    class.define_method("load_project", magnus::function!(Todozi::load_project, 1))?;
    
    class.define_method("load_project_task_container", magnus::function!(Todozi::load_project_task_container, 1))?;
    
    class.define_method("load_project_task_container_by_hash", magnus::function!(Todozi::load_project_task_container_by_hash, 1))?;
    
    class.define_method("load_queue_collection", magnus::function!(Todozi::load_queue_collection, 0))?;
    
    class.define_method("load_task", magnus::function!(Todozi::load_task, 1))?;
    
    class.define_method("load_task_collection", magnus::function!(Todozi::load_task_collection, 1))?;
    
    class.define_method("load_tasks", magnus::function!(Todozi::load_tasks, 0))?;
    
    class.define_method("load_training_data", magnus::function!(Todozi::load_training_data, 1))?;
    
    class.define_method("long_term_percentage", magnus::function!(Todozi::long_term_percentage, 0))?;
    
    class.define_method("low", magnus::function!(Todozi::low, 1))?;
    
    class.define_method("mark_chunk_completed", magnus::function!(Todozi::mark_chunk_completed, 1))?;
    
    class.define_method("mark_chunk_validated", magnus::function!(Todozi::mark_chunk_validated, 1))?;
    
    class.define_method("mark_reminder_cancelled", magnus::function!(Todozi::mark_reminder_cancelled, 1))?;
    
    class.define_method("mark_reminder_completed", magnus::function!(Todozi::mark_reminder_completed, 1))?;
    
    class.define_method("matches", magnus::function!(Todozi::matches, 2))?;
    
    class.define_method("max_tokens", magnus::function!(Todozi::max_tokens, 0))?;
    
    class.define_method("meaning", magnus::function!(Todozi::meaning, 1))?;
    
    class.define_method("merge_tags", magnus::function!(Todozi::merge_tags, 2))?;
    
    class.define_method("migrate", magnus::function!(Todozi::migrate, 0))?;
    
    class.define_method("migrate_tasks", magnus::function!(Todozi::migrate_tasks, 3))?;
    
    class.define_method("migrate_to_project_based", magnus::function!(Todozi::migrate_to_project_based, 0))?;
    
    class.define_method("moment", magnus::function!(Todozi::moment, 1))?;
    
    class.define_method("move_task", magnus::function!(Todozi::move_task, 3))?;
    
    class.define_method("multi_query_search", magnus::function!(Todozi::multi_query_search, 4))?;
    
    class.define_method("name", magnus::function!(Todozi::name, 1))?;
    
    class.define_method("new_full", magnus::function!(Todozi::new_full, 11))?;
    
    class.define_method("new_idea", magnus::function!(Todozi::new_idea, 1))?;
    
    class.define_method("new_memory", magnus::function!(Todozi::new_memory, 1))?;
    
    class.define_method("new_with_hashes", magnus::function!(Todozi::new_with_hashes, 1))?;
    
    class.define_method("ok", magnus::function!(Todozi::ok, 1))?;
    
    class.define_method("overdue_percentage", magnus::function!(Todozi::overdue_percentage, 0))?;
    
    class.define_method("parse_agent_assignment_format", magnus::function!(Todozi::parse_agent_assignment_format, 1))?;
    
    class.define_method("parse_chunking_format", magnus::function!(Todozi::parse_chunking_format, 1))?;
    
    class.define_method("parse_dependencies", magnus::function!(Todozi::parse_dependencies, 1))?;
    
    class.define_method("parse_error_format", magnus::function!(Todozi::parse_error_format, 1))?;
    
    class.define_method("parse_feeling_format", magnus::function!(Todozi::parse_feeling_format, 1))?;
    
    class.define_method("parse_idea_format", magnus::function!(Todozi::parse_idea_format, 1))?;
    
    class.define_method("parse_memory_format", magnus::function!(Todozi::parse_memory_format, 2))?;
    
    class.define_method("parse_reminder_format", magnus::function!(Todozi::parse_reminder_format, 1))?;
    
    class.define_method("parse_summary_format", magnus::function!(Todozi::parse_summary_format, 1))?;
    
    class.define_method("parse_tags", magnus::function!(Todozi::parse_tags, 1))?;
    
    class.define_method("parse_tdz_command", magnus::function!(Todozi::parse_tdz_command, 1))?;
    
    class.define_method("parse_todozi_format", magnus::function!(Todozi::parse_todozi_format, 1))?;
    
    class.define_method("parse_training_data_format", magnus::function!(Todozi::parse_training_data_format, 1))?;
    
    class.define_method("pending_percentage", magnus::function!(Todozi::pending_percentage, 0))?;
    
    class.define_method("plan_task_actions", magnus::function!(Todozi::plan_task_actions, 1))?;
    
    class.define_method("plan_tasks", magnus::function!(Todozi::plan_tasks, 4))?;
    
    class.define_method("predict_relevance", magnus::function!(Todozi::predict_relevance, 1))?;
    
    class.define_method("preload_related_embeddings", magnus::function!(Todozi::preload_related_embeddings, 2))?;
    
    class.define_method("prepare_task_content", magnus::function!(Todozi::prepare_task_content, 1))?;
    
    class.define_method("prioritize_queue_item", magnus::function!(Todozi::prioritize_queue_item, 2))?;
    
    class.define_method("priority", magnus::function!(Todozi::priority, 1))?;
    
    class.define_method("private_percentage", magnus::function!(Todozi::private_percentage, 0))?;
    
    class.define_method("process_chat", magnus::function!(Todozi::process_chat, 2))?;
    
    class.define_method("process_chat_message", magnus::function!(Todozi::process_chat_message, 1))?;
    
    class.define_method("process_chat_message_extended", magnus::function!(Todozi::process_chat_message_extended, 2))?;
    
    class.define_method("process_chunking_message", magnus::function!(Todozi::process_chunking_message, 1))?;
    
    class.define_method("process_json_examples", magnus::function!(Todozi::process_json_examples, 1))?;
    
    class.define_method("process_tdz_commands", magnus::function!(Todozi::process_tdz_commands, 3))?;
    
    class.define_method("process_workflow", magnus::function!(Todozi::process_workflow, 1))?;
    
    class.define_method("profile_search_performance", magnus::function!(Todozi::profile_search_performance, 2))?;
    
    class.define_method("project_name", magnus::function!(Todozi::project_name, 0))?;
    
    class.define_method("project_tasks", magnus::function!(Todozi::project_tasks, 1))?;
    
    class.define_method("public_percentage", magnus::function!(Todozi::public_percentage, 0))?;
    
    class.define_method("queue_active", magnus::function!(Todozi::queue_active, 0))?;
    
    class.define_method("queue_add", magnus::function!(Todozi::queue_add, 2))?;
    
    class.define_method("queue_backlog", magnus::function!(Todozi::queue_backlog, 0))?;
    
    class.define_method("queue_bulk_operations", magnus::function!(Todozi::queue_bulk_operations, 1))?;
    
    class.define_method("queue_complete", magnus::function!(Todozi::queue_complete, 1))?;
    
    class.define_method("queue_list", magnus::function!(Todozi::queue_list, 0))?;
    
    class.define_method("queue_start", magnus::function!(Todozi::queue_start, 1))?;
    
    class.define_method("quick", magnus::function!(Todozi::quick, 0))?;
    
    class.define_method("quick_task", magnus::function!(Todozi::quick_task, 1))?;
    
    class.define_method("reason", magnus::function!(Todozi::reason, 1))?;
    
    class.define_method("recommend_similar", magnus::function!(Todozi::recommend_similar, 3))?;
    
    class.define_method("register_tool", magnus::function!(Todozi::register_tool, 1))?;
    
    class.define_method("register_with_server", magnus::function!(Todozi::register_with_server, 1))?;
    
    class.define_method("relationships_per_tag", magnus::function!(Todozi::relationships_per_tag, 0))?;
    
    class.define_method("remember", magnus::function!(Todozi::remember, 2))?;
    
    class.define_method("remind_at", magnus::function!(Todozi::remind_at, 1))?;
    
    class.define_method("remove_api_key", magnus::function!(Todozi::remove_api_key, 1))?;
    
    class.define_method("remove_from_task", magnus::function!(Todozi::remove_from_task, 2))?;
    
    class.define_method("remove_item", magnus::function!(Todozi::remove_item, 1))?;
    
    class.define_method("remove_key", magnus::function!(Todozi::remove_key, 1))?;
    
    class.define_method("remove_tag_from_task", magnus::function!(Todozi::remove_tag_from_task, 2))?;
    
    class.define_method("remove_task", magnus::function!(Todozi::remove_task, 1))?;
    
    class.define_method("render", magnus::function!(Todozi::render, 1))?;
    
    class.define_method("render_compact", magnus::function!(Todozi::render_compact, 1))?;
    
    class.define_method("render_detailed", magnus::function!(Todozi::render_detailed, 1))?;
    
    class.define_method("reorder_queue", magnus::function!(Todozi::reorder_queue, 1))?;
    
    class.define_method("resolve_error", magnus::function!(Todozi::resolve_error, 2))?;
    
    class.define_method("restore_backup", magnus::function!(Todozi::restore_backup, 1))?;
    
    class.define_method("restore_embeddings", magnus::function!(Todozi::restore_embeddings, 1))?;
    
    class.define_method("run", magnus::function!(Todozi::run, 0))?;
    
    class.define_method("run_interactive", magnus::function!(Todozi::run_interactive, 0))?;
    
    class.define_method("sample_task", magnus::function!(Todozi::sample_task, 0))?;
    
    class.define_method("save_agent", magnus::function!(Todozi::save_agent, 1))?;
    
    class.define_method("save_agent_assignment", magnus::function!(Todozi::save_agent_assignment, 1))?;
    
    class.define_method("save_api_key_collection", magnus::function!(Todozi::save_api_key_collection, 1))?;
    
    class.define_method("save_as_default", magnus::function!(Todozi::save_as_default, 1))?;
    
    class.define_method("save_code_chunk", magnus::function!(Todozi::save_code_chunk, 1))?;
    
    class.define_method("save_config", magnus::function!(Todozi::save_config, 1))?;
    
    class.define_method("save_error", magnus::function!(Todozi::save_error, 1))?;
    
    class.define_method("save_feeling", magnus::function!(Todozi::save_feeling, 1))?;
    
    class.define_method("save_idea", magnus::function!(Todozi::save_idea, 1))?;
    
    class.define_method("save_memory", magnus::function!(Todozi::save_memory, 1))?;
    
    class.define_method("save_processed_content", magnus::function!(Todozi::save_processed_content, 3))?;
    
    class.define_method("save_project", magnus::function!(Todozi::save_project, 1))?;
    
    class.define_method("save_project_task_container", magnus::function!(Todozi::save_project_task_container, 1))?;
    
    class.define_method("save_queue_collection", magnus::function!(Todozi::save_queue_collection, 1))?;
    
    class.define_method("save_search_query", magnus::function!(Todozi::save_search_query, 2))?;
    
    class.define_method("save_task", magnus::function!(Todozi::save_task, 1))?;
    
    class.define_method("save_task_collection", magnus::function!(Todozi::save_task_collection, 2))?;
    
    class.define_method("save_task_with_embedding", magnus::function!(Todozi::save_task_with_embedding, 1))?;
    
    class.define_method("save_tasks", magnus::function!(Todozi::save_tasks, 0))?;
    
    class.define_method("save_training_data", magnus::function!(Todozi::save_training_data, 1))?;
    
    class.define_method("search", magnus::function!(Todozi::search, 2))?;
    
    class.define_method("search_ideas", magnus::function!(Todozi::search_ideas, 1))?;
    
    class.define_method("search_memories", magnus::function!(Todozi::search_memories, 1))?;
    
    class.define_method("search_reminders", magnus::function!(Todozi::search_reminders, 1))?;
    
    class.define_method("search_summaries", magnus::function!(Todozi::search_summaries, 1))?;
    
    class.define_method("search_tags", magnus::function!(Todozi::search_tags, 1))?;
    
    class.define_method("search_tasks", magnus::function!(Todozi::search_tasks, 3))?;
    
    class.define_method("search_tasks_semantic", magnus::function!(Todozi::search_tasks_semantic, 2))?;
    
    class.define_method("search_with_filters", magnus::function!(Todozi::search_with_filters, 2))?;
    
    class.define_method("see_all", magnus::function!(Todozi::see_all, 0))?;
    
    class.define_method("semantic_search", magnus::function!(Todozi::semantic_search, 3))?;
    
    class.define_method("serialization", magnus::function!(Todozi::serialization, 1))?;
    
    class.define_method("set_color_scheme", magnus::function!(Todozi::set_color_scheme, 1))?;
    
    class.define_method("share", magnus::function!(Todozi::share, 1))?;
    
    class.define_method("short_term_percentage", magnus::function!(Todozi::short_term_percentage, 0))?;
    
    class.define_method("show_loading_screen", magnus::function!(Todozi::show_loading_screen, 0))?;
    
    class.define_method("similar", magnus::function!(Todozi::similar, 1))?;
    
    class.define_method("similar_tasks", magnus::function!(Todozi::similar_tasks, 1))?;
    
    class.define_method("similar_tasks_emb", magnus::function!(Todozi::similar_tasks_emb, 1))?;
    
    class.define_method("similarity_threshold_search", magnus::function!(Todozi::similarity_threshold_search, 3))?;
    
    class.define_method("smart", magnus::function!(Todozi::smart, 1))?;
    
    class.define_method("smart_search", magnus::function!(Todozi::smart_search, 1))?;
    
    class.define_method("specializations", magnus::function!(Todozi::specializations, 1))?;
    
    class.define_method("start", magnus::function!(Todozi::start, 1))?;
    
    class.define_method("start_edit", magnus::function!(Todozi::start_edit, 1))?;
    
    class.define_method("start_edit_session", magnus::function!(Todozi::start_edit_session, 1))?;
    
    class.define_method("start_local_server", magnus::function!(Todozi::start_local_server, 2))?;
    
    class.define_method("start_queue_session", magnus::function!(Todozi::start_queue_session, 1))?;
    
    class.define_method("start_server", magnus::function!(Todozi::start_server, 2))?;
    
    class.define_method("start_session", magnus::function!(Todozi::start_session, 1))?;
    
    class.define_method("start_task", magnus::function!(Todozi::start_task, 1))?;
    
    class.define_method("stats", magnus::function!(Todozi::stats, 0))?;
    
    class.define_method("status", magnus::function!(Todozi::status, 1))?;
    
    class.define_method("stop_server", magnus::function!(Todozi::stop_server, 0))?;
    
    class.define_method("storage", magnus::function!(Todozi::storage, 0))?;
    
    class.define_method("storage_check_folder_structure", magnus::function!(Todozi::storage_check_folder_structure, 0))?;
    
    class.define_method("storage_clear_registration", magnus::function!(Todozi::storage_clear_registration, 0))?;
    
    class.define_method("storage_create_backup", magnus::function!(Todozi::storage_create_backup, 0))?;
    
    class.define_method("storage_delete_project_by_name", magnus::function!(Todozi::storage_delete_project_by_name, 1))?;
    
    class.define_method("storage_delete_task_from_project", magnus::function!(Todozi::storage_delete_task_from_project, 1))?;
    
    class.define_method("storage_ensure_folder_structure", magnus::function!(Todozi::storage_ensure_folder_structure, 0))?;
    
    class.define_method("storage_get_ai_tasks", magnus::function!(Todozi::storage_get_ai_tasks, 0))?;
    
    class.define_method("storage_get_all_active_tasks", magnus::function!(Todozi::storage_get_all_active_tasks, 0))?;
    
    class.define_method("storage_get_all_completed_tasks", magnus::function!(Todozi::storage_get_all_completed_tasks, 0))?;
    
    class.define_method("storage_get_collaborative_tasks", magnus::function!(Todozi::storage_get_collaborative_tasks, 0))?;
    
    class.define_method("storage_get_human_tasks", magnus::function!(Todozi::storage_get_human_tasks, 0))?;
    
    class.define_method("storage_get_project_stats", magnus::function!(Todozi::storage_get_project_stats, 1))?;
    
    class.define_method("storage_get_registration_info", magnus::function!(Todozi::storage_get_registration_info, 0))?;
    
    class.define_method("storage_get_storage_dir", magnus::function!(Todozi::storage_get_storage_dir, 0))?;
    
    class.define_method("storage_get_task_from_any_project", magnus::function!(Todozi::storage_get_task_from_any_project, 1))?;
    
    class.define_method("storage_init", magnus::function!(Todozi::storage_init, 0))?;
    
    class.define_method("storage_is_registered", magnus::function!(Todozi::storage_is_registered, 0))?;
    
    class.define_method("storage_list_backups", magnus::function!(Todozi::storage_list_backups, 0))?;
    
    class.define_method("storage_list_projects", magnus::function!(Todozi::storage_list_projects, 0))?;
    
    class.define_method("storage_load_config", magnus::function!(Todozi::storage_load_config, 0))?;
    
    class.define_method("storage_load_project", magnus::function!(Todozi::storage_load_project, 1))?;
    
    class.define_method("storage_load_task_collection", magnus::function!(Todozi::storage_load_task_collection, 1))?;
    
    class.define_method("storage_register_with_server", magnus::function!(Todozi::storage_register_with_server, 1))?;
    
    class.define_method("storage_restore_backup", magnus::function!(Todozi::storage_restore_backup, 1))?;
    
    class.define_method("storage_save_config", magnus::function!(Todozi::storage_save_config, 0))?;
    
    class.define_method("storage_save_project", magnus::function!(Todozi::storage_save_project, 1))?;
    
    class.define_method("storage_save_task_collection", magnus::function!(Todozi::storage_save_task_collection, 1))?;
    
    class.define_method("storage_search_tasks_semantic", magnus::function!(Todozi::storage_search_tasks_semantic, 2))?;
    
    class.define_method("strategy_content", magnus::function!(Todozi::strategy_content, 4))?;
    
    class.define_method("strategy_content_analysis", magnus::function!(Todozi::strategy_content_analysis, 2))?;
    
    class.define_method("strike_cluster", magnus::function!(Todozi::strike_cluster, 1))?;
    
    class.define_method("strike_tags", magnus::function!(Todozi::strike_tags, 1))?;
    
    class.define_method("success", magnus::function!(Todozi::success, 2))?;
    
    class.define_method("suggest_tags", magnus::function!(Todozi::suggest_tags, 2))?;
    
    class.define_method("tags", magnus::function!(Todozi::tags, 1))?;
    
    class.define_method("tags_advanced_search", magnus::function!(Todozi::tags_advanced_search, 1))?;
    
    class.define_method("task", magnus::function!(Todozi::task, 1))?;
    
    class.define_method("tasks", magnus::function!(Todozi::tasks, 1))?;
    
    class.define_method("tdz_cnt", magnus::function!(Todozi::tdz_cnt, 2))?;
    
    class.define_method("tdz_execute_command", magnus::function!(Todozi::tdz_execute_command, 1))?;
    
    class.define_method("tdz_find", magnus::function!(Todozi::tdz_find, 1))?;
    
    class.define_method("tdz_parse_command", magnus::function!(Todozi::tdz_parse_command, 1))?;
    
    class.define_method("tdzfp", magnus::function!(Todozi::tdzfp, 0))?;
    
    class.define_method("team_percentage", magnus::function!(Todozi::team_percentage, 0))?;
    
    class.define_method("term", magnus::function!(Todozi::term, 1))?;
    
    class.define_method("title", magnus::function!(Todozi::title, 0))?;
    
    class.define_method("to_context_string", magnus::function!(Todozi::to_context_string, 0))?;
    
    class.define_method("to_ollama_format", magnus::function!(Todozi::to_ollama_format, 0))?;
    
    class.define_method("to_state_string", magnus::function!(Todozi::to_state_string, 0))?;
    
    class.define_method("todozi_begin", magnus::function!(Todozi::todozi_begin, 0))?;
    
    class.define_method("todozi_init", magnus::function!(Todozi::todozi_init, 0))?;
    
    class.define_method("todozi_init_with_auto_registration", magnus::function!(Todozi::todozi_init_with_auto_registration, 0))?;
    
    class.define_method("tool_count", magnus::function!(Todozi::tool_count, 0))?;
    
    class.define_method("total_results", magnus::function!(Todozi::total_results, 0))?;
    
    class.define_method("track_embedding_drift", magnus::function!(Todozi::track_embedding_drift, 2))?;
    
    class.define_method("transform_shorthand_tags", magnus::function!(Todozi::transform_shorthand_tags, 1))?;
    
    class.define_method("types", magnus::function!(Todozi::types, 0))?;
    
    class.define_method("unregister", magnus::function!(Todozi::unregister, 1))?;
    
    class.define_method("update", magnus::function!(Todozi::update, 1))?;
    
    class.define_method("update_agent", magnus::function!(Todozi::update_agent, 2))?;
    
    class.define_method("update_agent_assignment_status", magnus::function!(Todozi::update_agent_assignment_status, 3))?;
    
    class.define_method("update_agent_status", magnus::function!(Todozi::update_agent_status, 2))?;
    
    class.define_method("update_chunk_code", magnus::function!(Todozi::update_chunk_code, 2))?;
    
    class.define_method("update_chunk_tests", magnus::function!(Todozi::update_chunk_tests, 2))?;
    
    class.define_method("update_config", magnus::function!(Todozi::update_config, 1))?;
    
    class.define_method("update_config_with_registration", magnus::function!(Todozi::update_config_with_registration, 1))?;
    
    class.define_method("update_feeling", magnus::function!(Todozi::update_feeling, 1))?;
    
    class.define_method("update_idea", magnus::function!(Todozi::update_idea, 2))?;
    
    class.define_method("update_memory", magnus::function!(Todozi::update_memory, 2))?;
    
    class.define_method("update_project", magnus::function!(Todozi::update_project, 1))?;
    
    class.define_method("update_registration_api_key", magnus::function!(Todozi::update_registration_api_key, 1))?;
    
    class.define_method("update_registration_keys", magnus::function!(Todozi::update_registration_keys, 3))?;
    
    class.define_method("update_reminder", magnus::function!(Todozi::update_reminder, 2))?;
    
    class.define_method("update_summary", magnus::function!(Todozi::update_summary, 2))?;
    
    class.define_method("update_tag", magnus::function!(Todozi::update_tag, 2))?;
    
    class.define_method("update_task", magnus::function!(Todozi::update_task, 2))?;
    
    class.define_method("update_task_full", magnus::function!(Todozi::update_task_full, 2))?;
    
    class.define_method("update_task_in_project", magnus::function!(Todozi::update_task_in_project, 2))?;
    
    class.define_method("update_task_status", magnus::function!(Todozi::update_task_status, 2))?;
    
    class.define_method("urgent", magnus::function!(Todozi::urgent, 1))?;
    
    class.define_method("validate_embeddings", magnus::function!(Todozi::validate_embeddings, 0))?;
    
    class.define_method("validate_migration", magnus::function!(Todozi::validate_migration, 0))?;
    
    class.define_method("validate_required_params", magnus::function!(Todozi::validate_required_params, 3))?;
    
    class.define_method("validate_string_param", magnus::function!(Todozi::validate_string_param, 5))?;
    
    class.define_method("validate_task_input", magnus::function!(Todozi::validate_task_input, 7))?;
    
    class.define_method("validate_tdz_command", magnus::function!(Todozi::validate_tdz_command, 1))?;
    
    class.define_method("validation", magnus::function!(Todozi::validation, 1))?;
    
    class.define_method("verbose", magnus::function!(Todozi::verbose, 1))?;
    
    class.define_method("with_action", magnus::function!(Todozi::with_action, 1))?;
    
    class.define_method("with_assignee", magnus::function!(Todozi::with_assignee, 1))?;
    
    class.define_method("with_context", magnus::function!(Todozi::with_context, 1))?;
    
    class.define_method("with_context_notes", magnus::function!(Todozi::with_context_notes, 1))?;
    
    class.define_method("with_dependencies", magnus::function!(Todozi::with_dependencies, 1))?;
    
    class.define_method("with_dry_run", magnus::function!(Todozi::with_dry_run, 1))?;
    
    class.define_method("with_embedding_service", magnus::function!(Todozi::with_embedding_service, 1))?;
    
    class.define_method("with_embedding_service_option", magnus::function!(Todozi::with_embedding_service_option, 1))?;
    
    class.define_method("with_force", magnus::function!(Todozi::with_force, 1))?;
    
    class.define_method("with_max_tokens", magnus::function!(Todozi::with_max_tokens, 1))?;
    
    class.define_method("with_parent_project", magnus::function!(Todozi::with_parent_project, 1))?;
    
    class.define_method("with_priority", magnus::function!(Todozi::with_priority, 1))?;
    
    class.define_method("with_progress", magnus::function!(Todozi::with_progress, 1))?;
    
    class.define_method("with_shared_components", magnus::function!(Todozi::with_shared_components, 6))?;
    
    class.define_method("with_status", magnus::function!(Todozi::with_status, 1))?;
    
    class.define_method("with_tags", magnus::function!(Todozi::with_tags, 1))?;
    
    class.define_method("with_temperature", magnus::function!(Todozi::with_temperature, 1))?;
    
    class.define_method("with_time", magnus::function!(Todozi::with_time, 1))?;
    
    class.define_method("with_user_id", magnus::function!(Todozi::with_user_id, 1))?;
    
    class.define_method("with_verbose", magnus::function!(Todozi::with_verbose, 1))?;
    
    Ok(())
}
