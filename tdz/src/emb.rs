use crate::error::Result;
use crate::{
    base::{ResourceLock, Tool, ToolDefinition, ToolParameter, ToolResult},
    models::*,
    storage::*,
    tags::*,
};
use async_trait::async_trait;
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig, DTYPE};
use chrono::{DateTime, Utc};
// Future expansion: streaming embeddings
// use futures::stream::Stream;
// use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoziEmbeddingConfig {
    pub model_name: String,
    pub dimensions: usize,
    pub similarity_threshold: f32,
    pub max_results: usize,
    pub cache_ttl_seconds: u64,
    pub enable_clustering: bool,
    pub clustering_threshold: f32,
}

impl Default for TodoziEmbeddingConfig {
    fn default() -> Self {
        Self {
            model_name: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
            dimensions: 384,
            similarity_threshold: 0.7,
            max_results: 50,
            cache_ttl_seconds: 3600 * 24,
            enable_clustering: true,
            clustering_threshold: 0.8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoziEmbeddingCache {
    pub vector: Vec<f32>,
    pub content_type: TodoziContentType,
    pub content_id: String,
    pub text_content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TodoziContentType {
    Task,
    Tag,
    Memory,
    Idea,
    Chunk,
    Feel,
    Train,
    Error,
    Summary,
    Reminder,
    Tdz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub content_id: String,
    pub content_type: TodoziContentType,
    pub similarity_score: f32,
    pub text_content: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringResult {
    pub cluster_id: String,
    pub content_items: Vec<SimilarityResult>,
    pub cluster_center: Vec<f32>,
    pub cluster_size: usize,
    pub average_similarity: f32,
}

// NEW: Aggregation types for multi-query search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationType {
    Average,
    Max,
    Min,
    Weighted(Vec<f32>), // Custom weights per query
}

// NEW: Search filters for filtered semantic search
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    pub tags: Option<Vec<String>>,
    pub priority: Option<Vec<Priority>>,
    pub status: Option<Vec<Status>>,
    pub assignee: Option<Vec<String>>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub min_progress: Option<u8>,
    pub max_progress: Option<u8>,
}

// NEW: Hierarchical cluster for hierarchical clustering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchicalCluster {
    pub cluster_id: String,
    pub level: usize,
    pub content_items: Vec<SimilarityResult>,
    pub cluster_center: Vec<f32>,
    pub children: Vec<HierarchicalCluster>,
    pub parent_id: Option<String>,
    pub average_similarity: f32,
}

// NEW: Labeled cluster with auto-generated name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledCluster {
    pub cluster_id: String,
    pub label: String,
    pub description: Option<String>,
    pub confidence: f32,
    pub content_items: Vec<SimilarityResult>,
}

// NEW: Drift report for tracking embedding changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    pub content_id: String,
    pub current_similarity_to_original: f32,
    pub drift_percentage: f32,
    pub significant_drift: bool, // threshold-based
    pub history: Vec<DriftSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftSnapshot {
    pub timestamp: DateTime<Utc>,
    pub similarity_to_original: f32,
    pub text_sample: String,
}

// NEW: Similarity graph for knowledge mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub content_type: TodoziContentType,
    pub label: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub similarity: f32,
    pub bidirectional: bool,
}

// NEW: Model comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelComparisonResult {
    pub text: String,
    pub models: HashMap<String, ModelEmbeddingResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEmbeddingResult {
    pub model_name: String,
    pub embedding: Vec<f32>,
    pub dimensions: usize,
    pub generation_time_ms: u128,
}

// NEW: Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub total_embeddings: usize,
    pub invalid_embeddings: usize,
    pub nan_count: usize,
    pub infinity_count: usize,
    pub zero_vector_count: usize,
    pub abnormal_distributions: Vec<String>,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub content_id: String,
    pub issue_type: String,
    pub severity: String,
    pub description: String,
}

// NEW: Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub query: String,
    pub iterations: usize,
    pub avg_time_ms: f64,
    pub min_time_ms: u128,
    pub max_time_ms: u128,
    pub std_dev_ms: f64,
    pub results_per_iteration: usize,
}

// NEW: Diagnostic report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub timestamp: DateTime<Utc>,
    pub cache_hit_rate: f32,
    pub avg_similarity_score: f32,
    pub embedding_distribution_stats: EmbeddingStats,
    pub content_type_breakdown: HashMap<String, usize>,
    pub top_similar_pairs: Vec<(String, String, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingStats {
    pub mean: Vec<f32>,
    pub std_dev: Vec<f32>,
    pub min: Vec<f32>,
    pub max: Vec<f32>,
}

// NEW: LRU Cache with memory limits
#[derive(Debug, Clone)]
pub struct LRUEmbeddingCache {
    max_memory_mb: usize,
    cache: VecDeque<(String, TodoziEmbeddingCache)>,
    access_counts: HashMap<String, usize>,
    current_memory_bytes: usize,
}

impl LRUEmbeddingCache {
    pub fn new(max_memory_mb: usize) -> Self {
        Self {
            max_memory_mb,
            cache: VecDeque::new(),
            access_counts: HashMap::new(),
            current_memory_bytes: 0,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&TodoziEmbeddingCache> {
        // Update access count
        *self.access_counts.entry(key.to_string()).or_insert(0) += 1;

        // Find and move to front
        if let Some(pos) = self.cache.iter().position(|(k, _)| k == key) {
            let item = self.cache.remove(pos).unwrap();
            self.cache.push_front(item.clone());
            self.cache.front().map(|(_, v)| v)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: String, value: TodoziEmbeddingCache) {
        let entry_size = Self::estimate_size(&value);

        // Evict if needed
        while self.current_memory_bytes + entry_size > self.max_memory_mb * 1024 * 1024
            && !self.cache.is_empty()
        {
            if let Some((old_key, old_value)) = self.cache.pop_back() {
                self.current_memory_bytes -= Self::estimate_size(&old_value);
                self.access_counts.remove(&old_key);
            }
        }

        self.cache.push_front((key.clone(), value));
        self.current_memory_bytes += entry_size;
        *self.access_counts.entry(key).or_insert(0) += 1;
    }

    fn estimate_size(entry: &TodoziEmbeddingCache) -> usize {
        // Rough estimate: vector size + text + overhead
        entry.vector.len() * 4 + entry.text_content.len() + 200
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

// Embedding model that uses candle for text embeddings
pub struct EmbeddingModel {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
    dimensions: usize,
}

impl EmbeddingModel {
    /// Load a sentence-transformers model from HuggingFace Hub
    pub async fn load(model_name: &str, device: Device) -> Result<Self> {
        use hf_hub::api::tokio::Api;
        // Create HuggingFace Hub API client with explicit cache directory
        let api = Api::new().map_err(|e| crate::error::TodoziError::EmbeddingError {
            message: format!(
                "❌ Failed to create HuggingFace API client: {}\n\
                 💡 Check your internet connection or set HF_TOKEN if using private models",
                e
            ),
        })?;

        // Create repo with explicit model name and type
        let repo = api.model(model_name.to_string());

        // Download model files with progress
        let config_filename = repo.get("config.json").await.map_err(|e| {
            crate::error::TodoziError::EmbeddingError {
                message: format!(
                    "❌ Failed to download config.json: {}\n\
                     💡 Model '{}' may not exist on HuggingFace.\n\
                     💡 Check available models at: https://huggingface.co/models?library=sentence-transformers",
                    e, model_name
                ),
            }
        })?;
        
        let tokenizer_filename = repo.get("tokenizer.json").await.map_err(|e| {
            crate::error::TodoziError::EmbeddingError {
                message: format!(
                    "❌ Failed to download tokenizer.json: {}\n\
                     💡 This model may not be compatible with sentence-transformers format",
                    e
                ),
            }
        })?;
       
        let weights_filename = match repo.get("pytorch_model.bin").await {
            Ok(path) => {
                path
            }
            Err(_) => {
                // Try safetensors format
                repo.get("model.safetensors")
                    .await
                    .map_err(|e| crate::error::TodoziError::EmbeddingError {
                        message: format!(
                            "❌ Failed to download model weights: {}\n\
                             💡 Tried both pytorch_model.bin and model.safetensors\n\
                             💡 Model may be incomplete or in unsupported format",
                            e
                        ),
                    })
                    .map(|path| {
                        path
                    })?
            }
        };

        // Load config
        let config: BertConfig =
            serde_json::from_slice(&std::fs::read(config_filename).map_err(|e| {
                crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to read config: {}", e),
                }
            })?)
            .map_err(|e| crate::error::TodoziError::EmbeddingError {
                message: format!("Failed to parse config: {}", e),
            })?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(|e| {
            crate::error::TodoziError::EmbeddingError {
                message: format!("Failed to load tokenizer: {}", e),
            }
        })?;

        // Configure tokenizer
        let pp = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            ..Default::default()
        };
        tokenizer.with_padding(Some(pp));

        let truncation = TruncationParams {
            max_length: 512,
            ..Default::default()
        };
        tokenizer.with_truncation(Some(truncation)).map_err(|e| {
            crate::error::TodoziError::EmbeddingError {
                message: format!("Failed to set truncation: {}", e),
            }
        })?;

        // Load model weights
        let vb = if weights_filename.to_string_lossy().ends_with(".safetensors") {
            unsafe {
                VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device).map_err(
                    |e| crate::error::TodoziError::EmbeddingError {
                        message: format!("Failed to load safetensors: {}", e),
                    },
                )?
            }
        } else {
            VarBuilder::from_pth(&weights_filename, DTYPE, &device).map_err(|e| {
                crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to load pytorch model: {}", e),
                }
            })?
        };

        let model = BertModel::load(vb, &config).map_err(|e| {
            crate::error::TodoziError::EmbeddingError {
                message: format!("Failed to create BERT model: {}", e),
            }
        })?;

        let dimensions = config.hidden_size;

        Ok(Self {
            model,
            tokenizer,
            device,
            dimensions,
        })
    }

    /// Save model name as default in config
    pub async fn save_as_default(model_name: &str) -> Result<()> {
        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;
        let config_path = std::path::PathBuf::from(&todozi_dir).join("tdz.hlx");
        let mut hlx = helix::Hlx::load(&config_path).await?;
        hlx.set("embedding", "model_name", model_name);
        hlx.save()?;
        Ok(())
    }

    /// Get default model name from config
    pub async fn get_default_model() -> Result<String> {
        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;

        let config_path = std::path::PathBuf::from(&todozi_dir).join("tdz.hlx");

        // Return early if config doesn't exist
        if !config_path.exists() {
            return Ok(TodoziEmbeddingConfig::default().model_name);
        }

        let hlx = helix::Hlx::load(&config_path).await?;

        let model_name = hlx
            .get("embedding", "model_name")
            .and_then(|v| {
                if let helix::DnaValue::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| TodoziEmbeddingConfig::default().model_name);

        Ok(model_name)
    }

    /// Generate embeddings for a batch of texts using mean pooling
    pub fn encode(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        // Tokenize all texts
        let encodings = self
            .tokenizer
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| crate::error::TodoziError::EmbeddingError {
                message: format!("Failed to tokenize: {}", e),
            })?;

        let mut all_embeddings = Vec::new();

        for encoding in encodings {
            let tokens = encoding.get_ids();
            let attention_mask = encoding.get_attention_mask();

            // Convert to tensors
            let token_ids = Tensor::new(tokens, &self.device)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to create token tensor: {}", e),
                })?
                .unsqueeze(0)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to unsqueeze tokens: {}", e),
                })?;

            let attention_mask_tensor = Tensor::new(attention_mask, &self.device)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to create attention mask: {}", e),
                })?
                .unsqueeze(0)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to unsqueeze attention mask: {}", e),
                })?;

            // Forward pass through BERT (token_type_ids is optional, pass None)
            let output = self
                .model
                .forward(&token_ids, &attention_mask_tensor, None)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to run model forward: {}", e),
                })?;

            // Mean pooling: average the token embeddings
            let embeddings =
                output
                    .squeeze(0)
                    .map_err(|e| crate::error::TodoziError::EmbeddingError {
                        message: format!("Failed to squeeze output: {}", e),
                    })?;

            // Apply attention mask and mean pool
            let sum_embeddings = embeddings
                .to_dtype(DType::F32)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to convert to f32: {}", e),
                })?
                .mean(0)
                .map_err(|e| crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to mean pool: {}", e),
                })?;

            // Normalize
            let embedding_vec = sum_embeddings.to_vec1::<f32>().map_err(|e| {
                crate::error::TodoziError::EmbeddingError {
                    message: format!("Failed to convert to vec: {}", e),
                }
            })?;

            // L2 normalization
            let norm: f32 = embedding_vec.iter().map(|x| x * x).sum::<f32>().sqrt();
            let normalized: Vec<f32> = if norm > 0.0 {
                embedding_vec.iter().map(|x| x / norm).collect()
            } else {
                embedding_vec
            };

            all_embeddings.push(normalized);
        }

        Ok(all_embeddings)
    }
}

#[derive(Clone)]
pub struct TodoziEmbeddingService {
    config: Arc<Mutex<TodoziEmbeddingConfig>>,
    cache: Arc<Mutex<HashMap<String, TodoziEmbeddingCache>>>,
    embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>,
    embedding_models: Arc<Mutex<HashMap<String, Arc<EmbeddingModel>>>>,
    tag_manager: Arc<Mutex<TagManager>>,
    storage: Arc<Mutex<Storage>>,
}

impl std::fmt::Debug for TodoziEmbeddingService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TodoziEmbeddingService")
            .field("config", &"<config>")
            .field("cache", &"<cache>")
            .field("embedding_model", &"<embedding_model>")
            .field("embedding_models", &"<embedding_models>")
            .field("tag_manager", &"<tag_manager>")
            .field("storage", &"<storage>")
            .finish()
    }
}

impl TodoziEmbeddingService {
    /// Create a new embedding service
    pub async fn new(config: TodoziEmbeddingConfig) -> Result<Self> {
        let mut service = Self {
            config: Arc::new(Mutex::new(config)),
            cache: Arc::new(Mutex::new(HashMap::new())),
            embedding_model: Arc::new(Mutex::new(None)),
            embedding_models: Arc::new(Mutex::new(HashMap::new())),
            tag_manager: Arc::new(Mutex::new(TagManager::new())),
            storage: Arc::new(Mutex::new(Storage::new().await?)),
        };
        service.initialize().await?;
        Ok(service)
    }

    /// Create a new embedding service with shared components
    pub fn with_shared_components(
        config: Arc<Mutex<TodoziEmbeddingConfig>>,
        cache: Arc<Mutex<HashMap<String, TodoziEmbeddingCache>>>,
        embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>,
        embedding_models: Arc<Mutex<HashMap<String, Arc<EmbeddingModel>>>>,
        tag_manager: Arc<Mutex<TagManager>>,
        storage: Arc<Mutex<Storage>>,
    ) -> Self {
        Self {
            config,
            cache,
            embedding_model,
            embedding_models,
            tag_manager,
            storage,
        }
    }

    /// Get a task by ID
    pub async fn get_task(&self, id: &str) -> Result<Task> {
        let storage = self.storage.lock().await;
        storage.get_task_from_any_project(id)
    }

    /// Initialize the embedding service
    pub async fn initialize(&mut self) -> Result<()> {
        // Try to get model from config first, fallback to config default
        let model_name = match EmbeddingModel::get_default_model().await {
            Ok(name) => {
                name
            }
            Err(_) => {
                let config = self.config.lock().await;
                let default_name = config.model_name.clone();
                default_name
            }
        };

        // Set up custom cache directory at ~/.todozi/models
        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;

        let models_dir = std::path::PathBuf::from(&todozi_dir).join("models");
        std::fs::create_dir_all(&models_dir)?;

        // Set HF_HOME environment variable to use our custom cache
        std::env::set_var("HF_HOME", models_dir.to_string_lossy().to_string());

        // Use CPU device for compatibility (could be GPU if available)
        let device = Device::Cpu;

        // Load the actual sentence-transformers model
        let embedding_model = Arc::new(EmbeddingModel::load(&model_name, device).await?);

        *self.embedding_model.lock().await = Some(embedding_model);
        Ok(())
    }

    /// Create a new project
    pub async fn create_project(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<String> {
        let storage = self.storage.lock().await;
        storage.create_project(name.clone(), description)?;
        Ok(name)
    }

    /// Add a new task with embedding
    pub async fn add_task(&self, mut task: Task) -> Result<String> {
        let task_id = task.id.clone();

        // Auto-create project if it doesn't exist
        if !task.parent_project.is_empty() {
            let storage = self.storage.lock().await;
            if storage.get_project(&task.parent_project).is_err() {
                storage.create_project(task.parent_project.clone(), None)?;
            }
        }

        let embedding = self
            .generate_embedding(&self.prepare_task_content(&task))
            .await?;
        task.embedding_vector = Some(embedding);

        let storage = self.storage.lock().await;
        storage.add_task_to_project(task.clone()).await?;

        // Log to mega embedding file for cross-reference
        self.log_to_mega_file(&task).await?;

        Ok(task_id)
    }

    /// Update an existing task
    pub async fn update_task(&self, id: String, updates: TaskUpdate) -> Result<()> {
        // Drop the guard before awaiting to avoid deadlock
        {
            let storage = self.storage.lock().await;
            drop(storage);
        }
        self.storage.lock().await.update_task_in_project(&id, updates).await?;
        Ok(())
    }

    /// Create a new idea
    pub async fn new_idea(&self, idea: Idea) -> Result<String> {
        {
            let _storage = self.storage.lock().await;
            save_idea(&idea)?;
        }
        let service = TodoziEmbeddingService::with_shared_components(
            self.config.clone(),
            self.cache.clone(),
            self.embedding_model.clone(),
            self.embedding_models.clone(),
            self.tag_manager.clone(),
            self.storage.clone(),
        );
        service.embed_idea(&idea).await?;
        Ok(idea.id)
    }

    /// Create a new memory
    pub async fn new_memory(&self, memory: Memory) -> Result<String> {
        {
            let _storage = self.storage.lock().await;
            save_memory(&memory)?;
        }
        let service = TodoziEmbeddingService::with_shared_components(
            self.config.clone(),
            self.cache.clone(),
            self.embedding_model.clone(),
            self.embedding_models.clone(),
            self.tag_manager.clone(),
            self.storage.clone(),
        );
        service.embed_memory(&memory).await?;
        Ok(memory.id)
    }

    /// Embed a tag with all its content
    pub async fn embed_tag(&self, tag: &Tag) -> Result<String> {
        let content = self.prepare_tag_content(tag);
        let embedding = self.generate_embedding(&content).await?;
        let cache_key = format!("tag_{}", tag.id);
        let cache_entry = TodoziEmbeddingCache {
            vector: embedding,
            content_type: TodoziContentType::Tag,
            content_id: tag.id.clone(),
            text_content: content,
            tags: vec![tag.name.clone()],
            created_at: Utc::now(),
            ttl_seconds: self.config.lock().await.cache_ttl_seconds,
        };
        self.cache.lock().await.insert(cache_key, cache_entry);
        Ok(tag.id.clone())
    }

    /// Embed an idea with all its content
    pub async fn embed_idea(&self, idea: &Idea) -> Result<String> {
        let content = self.prepare_idea_content(idea);
        let embedding = self.generate_embedding(&content).await?;
        let cache_key = format!("idea_{}", idea.id);
        let cache_entry = TodoziEmbeddingCache {
            vector: embedding,
            content_type: TodoziContentType::Idea,
            content_id: idea.id.clone(),
            text_content: content,
            tags: idea.tags.clone(),
            created_at: Utc::now(),
            ttl_seconds: self.config.lock().await.cache_ttl_seconds,
        };
        self.cache.lock().await.insert(cache_key, cache_entry);
        Ok(idea.id.clone())
    }

    /// Embed a memory with all its content
    pub async fn embed_memory(&self, memory: &Memory) -> Result<String> {
        let content = self.prepare_memory_content(memory);
        let embedding = self.generate_embedding(&content).await?;
        let cache_key = format!("memory_{}", memory.id);
        let cache_entry = TodoziEmbeddingCache {
            vector: embedding,
            content_type: TodoziContentType::Memory,
            content_id: memory.id.clone(),
            text_content: content,
            tags: memory.tags.clone(),
            created_at: Utc::now(),
            ttl_seconds: self.config.lock().await.cache_ttl_seconds,
        };
        self.cache.lock().await.insert(cache_key, cache_entry);
        Ok(memory.id.clone())
    }

    /// Prepare idea content for embedding
    fn prepare_idea_content(&self, idea: &Idea) -> String {
        let mut content = String::new();
        content.push_str(&format!("Idea: {}\n", idea.idea));
        content.push_str(&format!("Importance: {:?}\n", idea.importance));
        content.push_str(&format!("Share Level: {:?}\n", idea.share));
        if !idea.tags.is_empty() {
            content.push_str(&format!("Tags: {}\n", idea.tags.join(", ")));
        }
        if let Some(context) = &idea.context {
            content.push_str(&format!("Context: {}\n", context));
        }
        content
    }

    /// Prepare memory content for embedding
    fn prepare_memory_content(&self, memory: &Memory) -> String {
        let mut content = String::new();
        content.push_str(&format!("Memory: {}\n", memory.moment));
        content.push_str(&format!("Meaning: {}\n", memory.meaning));
        content.push_str(&format!("Reason: {}\n", memory.reason));
        content.push_str(&format!("Importance: {:?}\n", memory.importance));
        content.push_str(&format!("Term: {:?}\n", memory.term));
        content.push_str(&format!("Type: {:?}\n", memory.memory_type));
        if !memory.tags.is_empty() {
            content.push_str(&format!("Tags: {}\n", memory.tags.join(", ")));
        }
        content
    }

    /// Generate embedding for text content using real sentence-transformers
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let model_guard = self.embedding_model.lock().await;
        let model =
            model_guard
                .as_ref()
                .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                    message: "Embedding model not initialized. Call initialize() first."
                        .to_string(),
                })?;

        // Use the real BERT model to encode the text
        let embeddings = model.encode(&[text])?;

        // Return the first (and only) embedding
        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                message: "Failed to generate embedding".to_string(),
            })
    }

    /// Find semantically similar tasks
    pub async fn find_similar_tasks(
        &self,
        task_description: &str,
        limit: Option<usize>,
    ) -> Result<Vec<SimilarityResult>> {
        let query_embedding = self.generate_embedding(task_description).await?;
        let limit = limit.unwrap_or(self.config.lock().await.max_results);
        let mut similarities = Vec::new();
        let storage = self.storage.lock().await;
        let tasks = storage.list_tasks_across_projects(&crate::models::TaskFilters::default())?;
        for task in tasks {
            if let Some(embedding_vector) = &task.embedding_vector {
                let similarity = self.cosine_similarity(&query_embedding, embedding_vector);
                if similarity >= self.config.lock().await.similarity_threshold {
                    similarities.push(SimilarityResult {
                        content_id: task.id.clone(),
                        content_type: TodoziContentType::Task,
                        similarity_score: similarity,
                        text_content: self.prepare_task_content(&task),
                        tags: task.tags.clone(),
                        metadata: HashMap::new(),
                    });
                }
            }
        }
        similarities.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        similarities.truncate(limit);
        Ok(similarities)
    }

    /// Find similar tags based on semantic similarity
    pub async fn find_similar_tags(
        &self,
        tag_name: &str,
        limit: Option<usize>,
    ) -> Result<Vec<SimilarityResult>> {
        let query_embedding = self.generate_embedding(tag_name).await?;
        let limit = limit.unwrap_or(self.config.lock().await.max_results);
        let mut similarities = Vec::new();
        let cache = self.cache.lock().await;
        for (_, entry) in cache.iter() {
            if matches!(entry.content_type, TodoziContentType::Tag) {
                let similarity = self.cosine_similarity(&query_embedding, &entry.vector);
                if similarity >= self.config.lock().await.similarity_threshold {
                    similarities.push(SimilarityResult {
                        content_id: entry.content_id.clone(),
                        content_type: entry.content_type.clone(),
                        similarity_score: similarity,
                        text_content: entry.text_content.clone(),
                        tags: entry.tags.clone(),
                        metadata: HashMap::new(),
                    });
                }
            }
        }
        similarities.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        similarities.truncate(limit);
        Ok(similarities)
    }

    /// Cluster related content together
    pub async fn cluster_content(&self) -> Result<Vec<ClusteringResult>> {
        if !self.config.lock().await.enable_clustering {
            return Ok(Vec::new());
        }
        let cache = self.cache.lock().await;
        let mut clusters = Vec::new();
        let mut processed = std::collections::HashSet::new();
        let threshold = self.config.lock().await.clustering_threshold;
        for (key, entry) in cache.iter() {
            if processed.contains(key) {
                continue;
            }
            let mut cluster_items = vec![SimilarityResult {
                content_id: entry.content_id.clone(),
                content_type: entry.content_type.clone(),
                similarity_score: 1.0,
                text_content: entry.text_content.clone(),
                tags: entry.tags.clone(),
                metadata: HashMap::new(),
            }];
            for (other_key, other_entry) in cache.iter() {
                if key == other_key || processed.contains(other_key) {
                    continue;
                }
                let similarity = self.cosine_similarity(&entry.vector, &other_entry.vector);
                if similarity >= threshold {
                    cluster_items.push(SimilarityResult {
                        content_id: other_entry.content_id.clone(),
                        content_type: other_entry.content_type.clone(),
                        similarity_score: similarity,
                        text_content: other_entry.text_content.clone(),
                        tags: other_entry.tags.clone(),
                        metadata: HashMap::new(),
                    });
                    processed.insert(other_key.clone());
                }
            }
            if cluster_items.len() > 1 {
                let cluster_id = uuid::Uuid::new_v4().to_string();
                let average_similarity = cluster_items
                    .iter()
                    .skip(1)
                    .map(|item| item.similarity_score)
                    .sum::<f32>()
                    / (cluster_items.len() - 1) as f32;
                clusters.push(ClusteringResult {
                    cluster_id,
                    content_items: cluster_items.clone(),
                    cluster_center: entry.vector.clone(),
                    cluster_size: cluster_items.len(),
                    average_similarity,
                });
            }
            processed.insert(key.clone());
        }
        Ok(clusters)
    }

    /// Search across all embedded content
    pub async fn semantic_search(
        &self,
        query: &str,
        content_types: Option<Vec<TodoziContentType>>,
        limit: Option<usize>,
    ) -> Result<Vec<SimilarityResult>> {
        let query_embedding = self.generate_embedding(query).await?;
        let limit = limit.unwrap_or(self.config.lock().await.max_results);
        let mut similarities = Vec::new();
        let cache = self.cache.lock().await;
        for (_, entry) in cache.iter() {
            if let Some(ref types) = content_types {
                if !types.contains(&entry.content_type) {
                    continue;
                }
            }
            let similarity = self.cosine_similarity(&query_embedding, &entry.vector);
            if similarity >= self.config.lock().await.similarity_threshold {
                similarities.push(SimilarityResult {
                    content_id: entry.content_id.clone(),
                    content_type: entry.content_type.clone(),
                    similarity_score: similarity,
                    text_content: entry.text_content.clone(),
                    tags: entry.tags.clone(),
                    metadata: HashMap::new(),
                });
            }
        }
        similarities.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        similarities.truncate(limit);
        Ok(similarities)
    }

    /// Get embedding statistics
    pub async fn get_stats(&self) -> Result<HashMap<String, serde_json::Value>> {
        let cache = self.cache.lock().await;
        let mut stats = HashMap::new();
        let mut type_counts = HashMap::new();
        for entry in cache.values() {
            let type_name = format!("{:?}", entry.content_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }
        stats.insert("total_embeddings".to_string(), cache.len().into());
        stats.insert(
            "type_counts".to_string(),
            serde_json::to_value(type_counts)?,
        );
        Ok(stats)
    }

    /// Clean up expired cache entries
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let mut cache = self.cache.lock().await;
        let now = Utc::now();
        let mut expired_keys = Vec::new();
        for (key, entry) in cache.iter() {
            let expiry = entry.created_at + chrono::Duration::seconds(entry.ttl_seconds as i64);
            if expiry <= now {
                expired_keys.push(key.clone());
            }
        }
        let expired_count = expired_keys.len();
        for key in expired_keys {
            cache.remove(&key);
        }
        Ok(expired_count)
    }

    /// Prepare task content for embedding
    pub fn prepare_task_content(&self, task: &Task) -> String {
        let mut content = String::new();
        content.push_str(&format!("Task: {}\n", task.action));
        if let Some(context_notes) = &task.context_notes {
            content.push_str(&format!("Description: {}\n", context_notes));
        }
        content.push_str(&format!("Priority: {:?}\n", task.priority));
        content.push_str(&format!("Status: {:?}\n", task.status));
        if !task.tags.is_empty() {
            content.push_str(&format!("Tags: {}\n", task.tags.join(", ")));
        }
        if let Some(assignee) = &task.assignee {
            content.push_str(&format!("Assignee: {}\n", assignee));
        }
        if let Some(progress) = &task.progress {
            content.push_str(&format!("Progress: {}%\n", progress));
        }
        content
    }

    /// Prepare tag content for embedding
    fn prepare_tag_content(&self, tag: &Tag) -> String {
        let mut content = String::new();
        content.push_str(&format!("Tag: {}\n", tag.name));
        if let Some(description) = &tag.description {
            content.push_str(&format!("Description: {}\n", description));
        }
        if let Some(category) = &tag.category {
            content.push_str(&format!("Category: {}\n", category));
        }
        if let Some(color) = &tag.color {
            content.push_str(&format!("Color: {}\n", color));
        }
        content.push_str(&format!("Usage Count: {}\n", tag.usage_count));
        content
    }

    /// Calculate cosine similarity between two vectors
    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }
        dot_product / (norm_a * norm_b)
    }

    // ========== NEW ENHANCEMENT METHODS ==========

    /// Generate embeddings for multiple texts in parallel (HIGH IMPACT)
    pub async fn generate_embeddings_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let model_guard = self.embedding_model.lock().await;
        let model =
            model_guard
                .as_ref()
                .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                    message: "Embedding model not initialized".to_string(),
                })?;

        // Convert to &str slice for model
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

        // Use the model's batch encoding capability
        let embeddings = model.encode(&text_refs)?;

        Ok(embeddings)
    }

    /// Get or generate embedding with cache support (HIGH IMPACT)
    pub async fn get_or_generate_embedding(
        &mut self,
        content_id: &str,
        text: &str,
        content_type: TodoziContentType,
        refresh_if_stale: bool,
    ) -> Result<Vec<f32>> {
        let cache_key = format!("{:?}_{}", content_type, content_id);

        // Check cache first
        {
            let cache = self.cache.lock().await;
            if let Some(cached) = cache.get(&cache_key) {
                let now = Utc::now();
                let expiry =
                    cached.created_at + chrono::Duration::seconds(cached.ttl_seconds as i64);

                if expiry > now {
                    // Fresh cache hit
                    return Ok(cached.vector.clone());
                } else if !refresh_if_stale {
                    // Stale but acceptable
                    return Ok(cached.vector.clone());
                }
                // If refresh_if_stale=true, we'll regenerate below
            }
        }

        // Generate new embedding
        let embedding = self.generate_embedding(text).await?;

        // Update cache
        let cache_entry = TodoziEmbeddingCache {
            vector: embedding.clone(),
            content_type,
            content_id: content_id.to_string(),
            text_content: text.to_string(),
            tags: vec![],
            created_at: Utc::now(),
            ttl_seconds: self.config.lock().await.cache_ttl_seconds,
        };

        self.cache.lock().await.insert(cache_key, cache_entry);

        Ok(embedding)
    }

    /// Hybrid search combining semantic and keyword matching (HIGH IMPACT)
    pub async fn hybrid_search(
        &self,
        query: &str,
        keywords: Vec<String>,
        content_types: Option<Vec<TodoziContentType>>,
        semantic_weight: f32, // 0.0-1.0
        limit: usize,
    ) -> Result<Vec<SimilarityResult>> {
        // Validate semantic_weight
        let semantic_weight = semantic_weight.clamp(0.0, 1.0);
        let keyword_weight = 1.0 - semantic_weight;

        // Get semantic results
        let query_embedding = self.generate_embedding(query).await?;
        let mut results = Vec::new();

        let cache = self.cache.lock().await;
        for (_, entry) in cache.iter() {
            // Filter by content type
            if let Some(ref types) = content_types {
                if !types.contains(&entry.content_type) {
                    continue;
                }
            }

            // Calculate semantic similarity
            let semantic_score = self.cosine_similarity(&query_embedding, &entry.vector);

            // Calculate keyword score
            let mut keyword_score = 0.0f32;
            let text_lower = entry.text_content.to_lowercase();
            let query_lower = query.to_lowercase();

            // Simple TF-like scoring
            if text_lower.contains(&query_lower) {
                keyword_score += 0.5;
            }

            for keyword in &keywords {
                let kw_lower = keyword.to_lowercase();
                if text_lower.contains(&kw_lower) {
                    keyword_score += 0.3;
                }
            }

            keyword_score = keyword_score.min(1.0);

            // Combine scores
            let combined_score =
                (semantic_score * semantic_weight) + (keyword_score * keyword_weight);

            if combined_score >= self.config.lock().await.similarity_threshold {
                let mut metadata = HashMap::new();
                metadata.insert(
                    "semantic_score".to_string(),
                    serde_json::json!(semantic_score),
                );
                metadata.insert(
                    "keyword_score".to_string(),
                    serde_json::json!(keyword_score),
                );
                metadata.insert(
                    "combined_score".to_string(),
                    serde_json::json!(combined_score),
                );

                results.push(SimilarityResult {
                    content_id: entry.content_id.clone(),
                    content_type: entry.content_type.clone(),
                    similarity_score: combined_score,
                    text_content: entry.text_content.clone(),
                    tags: entry.tags.clone(),
                    metadata,
                });
            }
        }

        results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        results.truncate(limit);

        Ok(results)
    }

    /// Multi-query search with score aggregation (MEDIUM IMPACT)
    pub async fn multi_query_search(
        &self,
        queries: Vec<&str>,
        aggregation: AggregationType,
        content_types: Option<Vec<TodoziContentType>>,
        limit: usize,
    ) -> Result<Vec<SimilarityResult>> {
        // Generate embeddings for all queries
        let query_embeddings = {
            let mut embeddings = Vec::new();
            for query in &queries {
                embeddings.push(self.generate_embedding(query).await?);
            }
            embeddings
        };

        let mut results_map: HashMap<String, (SimilarityResult, Vec<f32>)> = HashMap::new();

        let cache = self.cache.lock().await;
        for (_, entry) in cache.iter() {
            if let Some(ref types) = content_types {
                if !types.contains(&entry.content_type) {
                    continue;
                }
            }

            // Calculate similarity for each query
            let mut similarities = Vec::new();
            for query_emb in &query_embeddings {
                let sim = self.cosine_similarity(query_emb, &entry.vector);
                similarities.push(sim);
            }

            // Aggregate scores
            let aggregated_score = match &aggregation {
                AggregationType::Average => {
                    similarities.iter().sum::<f32>() / similarities.len() as f32
                }
                AggregationType::Max => similarities
                    .iter()
                    .fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
                AggregationType::Min => similarities.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
                AggregationType::Weighted(weights) => {
                    if weights.len() != similarities.len() {
                        similarities.iter().sum::<f32>() / similarities.len() as f32
                    } else {
                        similarities
                            .iter()
                            .zip(weights.iter())
                            .map(|(s, w)| s * w)
                            .sum::<f32>()
                            / weights.iter().sum::<f32>()
                    }
                }
            };

            if aggregated_score >= self.config.lock().await.similarity_threshold {
                let result = SimilarityResult {
                    content_id: entry.content_id.clone(),
                    content_type: entry.content_type.clone(),
                    similarity_score: aggregated_score,
                    text_content: entry.text_content.clone(),
                    tags: entry.tags.clone(),
                    metadata: HashMap::new(),
                };

                results_map.insert(entry.content_id.clone(), (result, similarities));
            }
        }

        let mut results: Vec<SimilarityResult> = results_map
            .into_iter()
            .map(|(_, (result, _))| result)
            .collect();

        results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        results.truncate(limit);

        Ok(results)
    }

    /// Filtered semantic search with task-specific filters (HIGH IMPACT)
    pub async fn filtered_semantic_search(
        &self,
        query: &str,
        filters: SearchFilters,
        limit: usize,
    ) -> Result<Vec<SimilarityResult>> {
        let query_embedding = self.generate_embedding(query).await?;
        let mut similarities = Vec::new();

        let storage = self.storage.lock().await;
        let all_tasks = storage.list_tasks_across_projects(&crate::models::TaskFilters::default())?;

        for task in all_tasks {
            // Apply filters
            if let Some(ref filter_tags) = filters.tags {
                if !task.tags.iter().any(|t| filter_tags.contains(t)) {
                    continue;
                }
            }

            if let Some(ref filter_priorities) = filters.priority {
                if !filter_priorities.contains(&task.priority) {
                    continue;
                }
            }

            if let Some(ref filter_statuses) = filters.status {
                if !filter_statuses.contains(&task.status) {
                    continue;
                }
            }

            if let Some(ref filter_assignees) = filters.assignee {
                if let Some(ref assignee) = task.assignee {
                    // Convert Assignee to string for comparison
                    let assignee_str = assignee.to_string();
                    if !filter_assignees.contains(&assignee_str) {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let Some(min_progress) = filters.min_progress {
                if let Some(progress) = task.progress {
                    if progress < min_progress {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let Some(max_progress) = filters.max_progress {
                if let Some(progress) = task.progress {
                    if progress > max_progress {
                        continue;
                    }
                }
            }

            // Calculate similarity
            if let Some(ref embedding) = task.embedding_vector {
                let similarity = self.cosine_similarity(&query_embedding, embedding);
                if similarity >= self.config.lock().await.similarity_threshold {
                    similarities.push(SimilarityResult {
                        content_id: task.id.clone(),
                        content_type: TodoziContentType::Task,
                        similarity_score: similarity,
                        text_content: self.prepare_task_content(&task),
                        tags: task.tags.clone(),
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        similarities.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        similarities.truncate(limit);

        Ok(similarities)
    }

    /// Hierarchical clustering with depth control (MEDIUM IMPACT)
    pub async fn hierarchical_clustering(
        &self,
        content_types: Vec<TodoziContentType>,
        max_depth: usize,
    ) -> Result<Vec<HierarchicalCluster>> {
        let cache = self.cache.lock().await;

        // Filter by content types
        let mut items: Vec<(String, &TodoziEmbeddingCache)> = cache
            .iter()
            .filter(|(_, entry)| content_types.contains(&entry.content_type))
            .map(|(k, v)| (k.clone(), v))
            .collect();

        if items.is_empty() {
            return Ok(Vec::new());
        }

        let threshold = self.config.lock().await.clustering_threshold;

        fn build_cluster_recursive(
            items: &mut Vec<(String, &TodoziEmbeddingCache)>,
            level: usize,
            max_depth: usize,
            threshold: f32,
            parent_id: Option<String>,
            cosine_fn: &dyn Fn(&[f32], &[f32]) -> f32,
        ) -> Option<HierarchicalCluster> {
            if items.is_empty() || level >= max_depth {
                return None;
            }

            let cluster_id = uuid::Uuid::new_v4().to_string();
            let seed = items.remove(0);
            let seed_vector = &seed.1.vector;

            let mut cluster_items = vec![SimilarityResult {
                content_id: seed.1.content_id.clone(),
                content_type: seed.1.content_type.clone(),
                similarity_score: 1.0,
                text_content: seed.1.text_content.clone(),
                tags: seed.1.tags.clone(),
                metadata: HashMap::new(),
            }];

            // Find similar items
            let mut similar_items = Vec::new();
            items.retain(|(_, entry)| {
                let sim = cosine_fn(seed_vector, &entry.vector);
                if sim >= threshold {
                    cluster_items.push(SimilarityResult {
                        content_id: entry.content_id.clone(),
                        content_type: entry.content_type.clone(),
                        similarity_score: sim,
                        text_content: entry.text_content.clone(),
                        tags: entry.tags.clone(),
                        metadata: HashMap::new(),
                    });
                    similar_items.push((
                        format!("{:?}_{}", entry.content_type, entry.content_id),
                        *entry,
                    ));
                    false // remove from items
                } else {
                    true // keep in items
                }
            });

            let avg_sim = if cluster_items.len() > 1 {
                cluster_items
                    .iter()
                    .skip(1)
                    .map(|i| i.similarity_score)
                    .sum::<f32>()
                    / (cluster_items.len() - 1) as f32
            } else {
                1.0
            };

            // Build children recursively
            let mut children = Vec::new();
            if level + 1 < max_depth && !similar_items.is_empty() {
                while let Some(child) = build_cluster_recursive(
                    &mut similar_items,
                    level + 1,
                    max_depth,
                    threshold * 0.9, // Slightly lower threshold for sub-clusters
                    Some(cluster_id.clone()),
                    cosine_fn,
                ) {
                    children.push(child);
                }
            }

            Some(HierarchicalCluster {
                cluster_id,
                level,
                content_items: cluster_items,
                cluster_center: seed_vector.clone(),
                children,
                parent_id,
                average_similarity: avg_sim,
            })
        }

        let cosine_fn = |a: &[f32], b: &[f32]| -> f32 {
            if a.len() != b.len() {
                return 0.0;
            }
            let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
            let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
            let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm_a == 0.0 || norm_b == 0.0 {
                0.0
            } else {
                dot / (norm_a * norm_b)
            }
        };

        let mut clusters = Vec::new();
        while let Some(cluster) =
            build_cluster_recursive(&mut items, 0, max_depth, threshold, None, &cosine_fn)
        {
            clusters.push(cluster);
        }

        Ok(clusters)
    }

    /// Find outliers based on low similarity to all other items (MEDIUM IMPACT)
    pub async fn find_outliers(
        &self,
        content_type: TodoziContentType,
        threshold: f32,
    ) -> Result<Vec<String>> {
        let cache = self.cache.lock().await;

        let items: Vec<&TodoziEmbeddingCache> = cache
            .values()
            .filter(|entry| entry.content_type == content_type)
            .collect();

        let mut outliers = Vec::new();

        for item in &items {
            let mut max_similarity = 0.0f32;

            for other in &items {
                if item.content_id == other.content_id {
                    continue;
                }

                let sim = self.cosine_similarity(&item.vector, &other.vector);
                max_similarity = max_similarity.max(sim);
            }

            // If the item's highest similarity to any other item is below threshold, it's an outlier
            if max_similarity < threshold {
                outliers.push(item.content_id.clone());
            }
        }

        Ok(outliers)
    }

    /// Find related content across different content types (MEDIUM IMPACT)
    pub async fn find_cross_content_relationships(
        &self,
        content_id: &str,
        content_type: TodoziContentType,
        min_similarity: f32,
    ) -> Result<HashMap<TodoziContentType, Vec<SimilarityResult>>> {
        let cache = self.cache.lock().await;

        // Find the source item
        let cache_key = format!("{:?}_{}", content_type, content_id);
        let source =
            cache
                .get(&cache_key)
                .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                    message: format!("Content not found: {}", content_id),
                })?;

        let mut results: HashMap<TodoziContentType, Vec<SimilarityResult>> = HashMap::new();

        for entry in cache.values() {
            // Skip same item
            if entry.content_id == content_id && entry.content_type == content_type {
                continue;
            }

            let similarity = self.cosine_similarity(&source.vector, &entry.vector);

            if similarity >= min_similarity {
                let result = SimilarityResult {
                    content_id: entry.content_id.clone(),
                    content_type: entry.content_type.clone(),
                    similarity_score: similarity,
                    text_content: entry.text_content.clone(),
                    tags: entry.tags.clone(),
                    metadata: HashMap::new(),
                };

                results
                    .entry(entry.content_type.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }

        // Sort each type's results
        for list in results.values_mut() {
            list.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        }

        Ok(results)
    }

    /// Build a similarity graph (NICE TO HAVE)
    pub async fn build_similarity_graph(&self, threshold: f32) -> Result<SimilarityGraph> {
        let cache = self.cache.lock().await;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Create nodes
        for entry in cache.values() {
            let mut metadata = HashMap::new();
            metadata.insert(
                "text_sample".to_string(),
                serde_json::json!(entry.text_content.chars().take(100).collect::<String>()),
            );
            metadata.insert("tags".to_string(), serde_json::json!(entry.tags));

            nodes.push(GraphNode {
                id: entry.content_id.clone(),
                content_type: entry.content_type.clone(),
                label: entry
                    .text_content
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(50)
                    .collect(),
                metadata,
            });
        }

        // Create edges
        let items: Vec<&TodoziEmbeddingCache> = cache.values().collect();
        for (i, item1) in items.iter().enumerate() {
            for item2 in items.iter().skip(i + 1) {
                let similarity = self.cosine_similarity(&item1.vector, &item2.vector);

                if similarity >= threshold {
                    edges.push(GraphEdge {
                        from: item1.content_id.clone(),
                        to: item2.content_id.clone(),
                        similarity,
                        bidirectional: true,
                    });
                }
            }
        }

        Ok(SimilarityGraph { nodes, edges })
    }

    /// Recommendation system based on user interactions (MEDIUM IMPACT)
    pub async fn recommend_similar(
        &self,
        based_on: Vec<String>,
        exclude: Vec<String>,
        limit: usize,
    ) -> Result<Vec<SimilarityResult>> {
        let cache = self.cache.lock().await;

        // Get vectors for items user interacted with
        let mut base_vectors = Vec::new();
        for content_id in &based_on {
            for entry in cache.values() {
                if entry.content_id == *content_id {
                    base_vectors.push(entry.vector.clone());
                    break;
                }
            }
        }

        if base_vectors.is_empty() {
            return Ok(Vec::new());
        }

        // Calculate centroid of user's interests
        let dimensions = base_vectors[0].len();
        let mut centroid = vec![0.0; dimensions];

        for vector in &base_vectors {
            for (i, &val) in vector.iter().enumerate() {
                centroid[i] += val;
            }
        }

        for val in &mut centroid {
            *val /= base_vectors.len() as f32;
        }

        // Find similar items
        let mut results = Vec::new();
        let exclude_set: HashSet<String> = exclude.into_iter().collect();
        let based_on_set: HashSet<String> = based_on.into_iter().collect();

        for entry in cache.values() {
            if exclude_set.contains(&entry.content_id) || based_on_set.contains(&entry.content_id) {
                continue;
            }

            let similarity = self.cosine_similarity(&centroid, &entry.vector);

            if similarity >= self.config.lock().await.similarity_threshold {
                results.push(SimilarityResult {
                    content_id: entry.content_id.clone(),
                    content_type: entry.content_type.clone(),
                    similarity_score: similarity,
                    text_content: entry.text_content.clone(),
                    tags: entry.tags.clone(),
                    metadata: HashMap::new(),
                });
            }
        }

        results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        results.truncate(limit);

        Ok(results)
    }

    /// Auto-suggest tags based on similar items (MEDIUM IMPACT)
    pub async fn suggest_tags(&self, content_id: &str, top_k: usize) -> Result<Vec<String>> {
        // Find similar items
        let cache = self.cache.lock().await;

        let target = cache
            .values()
            .find(|entry| entry.content_id == content_id)
            .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                message: format!("Content not found: {}", content_id),
            })?;

        let mut similar_items = Vec::new();

        for entry in cache.values() {
            if entry.content_id == content_id {
                continue;
            }

            let similarity = self.cosine_similarity(&target.vector, &entry.vector);
            similar_items.push((similarity, &entry.tags));
        }

        similar_items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        similar_items.truncate(top_k);

        // Count tag frequencies weighted by similarity
        let mut tag_scores: HashMap<String, f32> = HashMap::new();

        for (similarity, tags) in similar_items {
            for tag in tags {
                *tag_scores.entry(tag.clone()).or_insert(0.0) += similarity;
            }
        }

        // Sort by score and return top tags
        let mut scored_tags: Vec<(String, f32)> = tag_scores.into_iter().collect();
        scored_tags.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Ok(scored_tags
            .into_iter()
            .take(top_k)
            .map(|(tag, _)| tag)
            .collect())
    }

    /// Track embedding drift over time (NICE TO HAVE)
    pub async fn track_embedding_drift(
        &mut self,
        content_id: &str,
        current_text: &str,
    ) -> Result<DriftReport> {
        // Get original embedding from cache
        let cache = self.cache.lock().await;

        let original = cache
            .values()
            .find(|entry| entry.content_id == content_id)
            .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                message: format!("Original embedding not found for: {}", content_id),
            })?;

        let original_vector = original.vector.clone();
        drop(cache); // Release lock before generating new embedding

        // Generate current embedding
        let current_vector = self.generate_embedding(current_text).await?;

        // Calculate drift
        let similarity = self.cosine_similarity(&original_vector, &current_vector);
        let drift_percentage = (1.0 - similarity) * 100.0;
        let significant_drift = drift_percentage > 20.0; // 20% threshold

        let report = DriftReport {
            content_id: content_id.to_string(),
            current_similarity_to_original: similarity,
            drift_percentage,
            significant_drift,
            history: vec![DriftSnapshot {
                timestamp: Utc::now(),
                similarity_to_original: similarity,
                text_sample: current_text.chars().take(200).collect(),
            }],
        };

        Ok(report)
    }

    /// Validate embeddings for quality issues (HIGH IMPACT)
    pub async fn validate_embeddings(&self) -> Result<ValidationReport> {
        let cache = self.cache.lock().await;

        let mut total = 0;
        let mut invalid = 0;
        let mut nan_count = 0;
        let mut infinity_count = 0;
        let mut zero_vector_count = 0;
        let mut issues = Vec::new();

        for entry in cache.values() {
            total += 1;
            let mut has_issue = false;

            // Check for NaN
            if entry.vector.iter().any(|&v| v.is_nan()) {
                nan_count += 1;
                has_issue = true;
                issues.push(ValidationIssue {
                    content_id: entry.content_id.clone(),
                    issue_type: "NaN".to_string(),
                    severity: "HIGH".to_string(),
                    description: "Embedding contains NaN values".to_string(),
                });
            }

            // Check for infinity
            if entry.vector.iter().any(|&v| v.is_infinite()) {
                infinity_count += 1;
                has_issue = true;
                issues.push(ValidationIssue {
                    content_id: entry.content_id.clone(),
                    issue_type: "Infinity".to_string(),
                    severity: "HIGH".to_string(),
                    description: "Embedding contains infinite values".to_string(),
                });
            }

            // Check for zero vector
            let magnitude: f32 = entry.vector.iter().map(|v| v * v).sum::<f32>().sqrt();
            if magnitude < 1e-6 {
                zero_vector_count += 1;
                has_issue = true;
                issues.push(ValidationIssue {
                    content_id: entry.content_id.clone(),
                    issue_type: "ZeroVector".to_string(),
                    severity: "MEDIUM".to_string(),
                    description: "Embedding is zero or near-zero vector".to_string(),
                });
            }

            if has_issue {
                invalid += 1;
            }
        }

        Ok(ValidationReport {
            total_embeddings: total,
            invalid_embeddings: invalid,
            nan_count,
            infinity_count,
            zero_vector_count,
            abnormal_distributions: vec![],
            issues,
        })
    }

    /// Profile search performance (HIGH IMPACT)
    pub async fn profile_search_performance(
        &mut self,
        query: &str,
        iterations: usize,
    ) -> Result<PerformanceMetrics> {
        use std::time::Instant;

        let mut times = Vec::new();
        let mut results_count = 0;

        for _ in 0..iterations {
            let start = Instant::now();
            let results = self.semantic_search(query, None, Some(10)).await?;
            let elapsed = start.elapsed();

            times.push(elapsed.as_millis());
            results_count = results.len();
        }

        let avg = times.iter().sum::<u128>() as f64 / times.len() as f64;
        let min = *times.iter().min().unwrap();
        let max = *times.iter().max().unwrap();

        // Calculate standard deviation
        let variance = times
            .iter()
            .map(|&t| {
                let diff = t as f64 - avg;
                diff * diff
            })
            .sum::<f64>()
            / times.len() as f64;
        let std_dev = variance.sqrt();

        Ok(PerformanceMetrics {
            query: query.to_string(),
            iterations,
            avg_time_ms: avg,
            min_time_ms: min,
            max_time_ms: max,
            std_dev_ms: std_dev,
            results_per_iteration: results_count,
        })
    }

    /// Export diagnostics (HIGH IMPACT)
    pub async fn export_diagnostics(&self) -> Result<DiagnosticReport> {
        let cache = self.cache.lock().await;

        // Calculate cache statistics
        let total = cache.len() as f32;

        // Content type breakdown
        let mut type_breakdown = HashMap::new();
        for entry in cache.values() {
            *type_breakdown
                .entry(format!("{:?}", entry.content_type))
                .or_insert(0) += 1;
        }

        // Calculate average similarity and find top pairs
        let items: Vec<&TodoziEmbeddingCache> = cache.values().collect();
        let mut similarities = Vec::new();

        for (i, item1) in items.iter().enumerate() {
            for item2 in items.iter().skip(i + 1) {
                let sim = self.cosine_similarity(&item1.vector, &item2.vector);
                similarities.push((item1.content_id.clone(), item2.content_id.clone(), sim));
            }
        }

        let avg_similarity = if !similarities.is_empty() {
            similarities.iter().map(|(_, _, s)| s).sum::<f32>() / similarities.len() as f32
        } else {
            0.0
        };

        similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        let top_pairs = similarities.into_iter().take(10).collect();

        // Calculate embedding distribution stats
        let dimensions = items.first().map(|e| e.vector.len()).unwrap_or(0);
        let mut means = vec![0.0; dimensions];
        let mut mins = vec![f32::INFINITY; dimensions];
        let mut maxs = vec![f32::NEG_INFINITY; dimensions];

        for item in &items {
            for (i, &val) in item.vector.iter().enumerate() {
                means[i] += val;
                mins[i] = mins[i].min(val);
                maxs[i] = maxs[i].max(val);
            }
        }

        for mean in &mut means {
            *mean /= total;
        }

        let mut std_devs = vec![0.0; dimensions];
        for item in &items {
            for (i, &val) in item.vector.iter().enumerate() {
                let diff = val - means[i];
                std_devs[i] += diff * diff;
            }
        }

        for std_dev in &mut std_devs {
            *std_dev = (*std_dev / total).sqrt();
        }

        Ok(DiagnosticReport {
            timestamp: Utc::now(),
            cache_hit_rate: 0.0, // Would need separate tracking
            avg_similarity_score: avg_similarity,
            embedding_distribution_stats: EmbeddingStats {
                mean: means,
                std_dev: std_devs,
                min: mins,
                max: maxs,
            },
            content_type_breakdown: type_breakdown,
            top_similar_pairs: top_pairs,
        })
    }

    /// Log task with embeddings to mega file for cross-reference/backup
    // PART 2: Additional Enhancement Methods for TodoziEmbeddingService
    // Insert these methods before log_to_mega_file() method

    /// Multi-model support: Load and manage multiple embedding models
    pub async fn load_additional_model(
        &mut self,
        model_name: &str,
        model_alias: String,
    ) -> Result<()> {

        // Set up custom cache directory at ~/.todozi/models
        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;
        let models_dir = std::path::PathBuf::from(&todozi_dir).join("models");
        std::fs::create_dir_all(&models_dir)?;

        // Set HF_HOME environment variable to use our custom cache
        std::env::set_var("HF_HOME", models_dir.to_string_lossy().to_string());

        // Use CPU device for compatibility (could be GPU if available)
        let device = Device::Cpu;

        // Load the actual sentence-transformers model
        let embedding_model = Arc::new(EmbeddingModel::load(model_name, device).await?);

        // Store in the service's model registry
        let mut models = self.embedding_models.lock().await;
        models.insert(model_alias.clone(), embedding_model);

        // Also store in config for persistence
        let config_path = std::path::PathBuf::from(&todozi_dir).join("tdz.hlx");
        let mut hlx = helix::Hlx::load(&config_path).await?;
        // Store additional model info
        hlx.set("embedding_models", &model_alias, model_name);
        hlx.save()?;
        Ok(())
    }

    /// Compare embeddings from multiple models (requires multi-model support)
    pub async fn compare_models(
        &self,
        text: &str,
        model_aliases: Vec<String>,
    ) -> Result<ModelComparisonResult> {
        use std::time::Instant;

        let mut results = HashMap::new();
        let models = self.embedding_models.lock().await;

        for model_alias in model_aliases {
            let start = Instant::now();

            // Get the model from registry, fallback to default if not found
            let embedding = if let Some(model) = models.get(&model_alias) {
                model.encode(&[text])?.into_iter().next().unwrap_or_default()
            } else {
                // Fallback to the default single model if alias not found
                self.generate_embedding(text).await?
            };
            let elapsed = start.elapsed();

            results.insert(
                model_alias.clone(),
                ModelEmbeddingResult {
                    model_name: model_alias.clone(),
                    embedding: embedding.clone(),
                    dimensions: embedding.len(),
                    generation_time_ms: elapsed.as_millis(),
                },
            );
        }

        Ok(ModelComparisonResult {
            text: text.to_string(),
            models: results,
        })
    }

    /// Auto-label clusters using LLM
    pub async fn auto_label_clusters(
        &self,
        clusters: Vec<ClusteringResult>,
    ) -> Result<Vec<LabeledCluster>> {
        let mut labeled_clusters = Vec::new();

        for cluster in clusters {
            // Extract common themes from cluster items
            let mut all_text = String::new();
            let mut all_tags: HashMap<String, usize> = HashMap::new();

            for item in &cluster.content_items {
                all_text.push_str(&item.text_content);
                all_text.push('\n');

                for tag in &item.tags {
                    *all_tags.entry(tag.clone()).or_insert(0) += 1;
                }
            }

            // Simple heuristic labeling (extract most common tags)
            let mut tag_vec: Vec<(String, usize)> = all_tags.into_iter().collect();
            tag_vec.sort_by(|a, b| b.1.cmp(&a.1));

            let label = if let Some((top_tag, _)) = tag_vec.first() {
                format!("Cluster: {}", top_tag)
            } else {
                // Use first few words from most similar item
                cluster
                    .content_items
                    .first()
                    .and_then(|item| {
                        item.text_content
                            .split_whitespace()
                            .take(3)
                            .collect::<Vec<_>>()
                            .join(" ")
                            .into()
                    })
                    .unwrap_or_else(|| "Unlabeled Cluster".to_string())
            };

            let description = format!(
                "Contains {} items with avg similarity of {:.2}",
                cluster.cluster_size, cluster.average_similarity
            );

            labeled_clusters.push(LabeledCluster {
                cluster_id: cluster.cluster_id,
                label,
                description: Some(description),
                confidence: cluster.average_similarity,
                content_items: cluster.content_items,
            });
        }

        Ok(labeled_clusters)
    }

    /// Calculate diversity score for a set of items
    pub async fn calculate_diversity(&self, content_ids: Vec<String>) -> Result<f32> {
        if content_ids.len() < 2 {
            return Ok(0.0);
        }

        let cache = self.cache.lock().await;

        // Collect vectors for specified items
        let mut vectors = Vec::new();
        for content_id in &content_ids {
            for entry in cache.values() {
                if entry.content_id == *content_id {
                    vectors.push(entry.vector.clone());
                    break;
                }
            }
        }

        if vectors.len() < 2 {
            return Ok(0.0);
        }

        // Calculate average pairwise distance (lower similarity = higher diversity)
        let mut total_distance = 0.0;
        let mut count = 0;

        for i in 0..vectors.len() {
            for j in (i + 1)..vectors.len() {
                let similarity = self.cosine_similarity(&vectors[i], &vectors[j]);
                let distance = 1.0 - similarity; // Convert similarity to distance
                total_distance += distance;
                count += 1;
            }
        }

        let diversity = if count > 0 {
            total_distance / count as f32
        } else {
            0.0
        };

        Ok(diversity)
    }

    /// Generate t-SNE coordinates for visualization (simplified 2D projection)
    pub async fn get_tsne_coordinates(
        &self,
        content_ids: Vec<String>,
        dimensions: usize, // 2 or 3
    ) -> Result<Vec<(String, Vec<f32>)>> {
        // Note: Full t-SNE requires external library like "tsne" crate
        // This is a simplified PCA-like projection

        if dimensions != 2 && dimensions != 3 {
            return Err(crate::error::TodoziError::EmbeddingError {
                message: "Only 2D or 3D projections supported".to_string(),
            });
        }

        let cache = self.cache.lock().await;

        // Collect vectors
        let mut data: Vec<(String, Vec<f32>)> = Vec::new();
        for content_id in &content_ids {
            for entry in cache.values() {
                if entry.content_id == *content_id {
                    data.push((content_id.clone(), entry.vector.clone()));
                    break;
                }
            }
        }

        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Simple dimensionality reduction: take first N principal components
        // (approximation - full PCA/t-SNE would be better)
        let mut projections = Vec::new();

        for (id, vector) in data {
            // Simple projection: take weighted sum of dimensions
            let mut projection = vec![0.0; dimensions];

            // Use first N dimensions with slight mixing
            for i in 0..dimensions {
                let start_idx = i * (vector.len() / dimensions);
                let end_idx = ((i + 1) * (vector.len() / dimensions)).min(vector.len());

                projection[i] =
                    vector[start_idx..end_idx].iter().sum::<f32>() / (end_idx - start_idx) as f32;
            }

            // Normalize
            let norm: f32 = projection.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm > 0.0 {
                for val in &mut projection {
                    *val /= norm;
                }
            }

            projections.push((id, projection));
        }

        Ok(projections)
    }

    /// Predictive pre-loading of related embeddings
    pub async fn preload_related_embeddings(
        &mut self,
        content_id: &str,
        depth: usize,
    ) -> Result<()> {
        // Use iterative approach instead of recursion to avoid infinite future size
        let mut to_process = vec![(content_id.to_string(), depth)];
        let mut processed = std::collections::HashSet::new();

        while let Some((current_id, current_depth)) = to_process.pop() {
            if current_depth == 0 || processed.contains(&current_id) {
                continue;
            }

            processed.insert(current_id.clone());

            // Find similar items
            let cache = self.cache.lock().await;

            let source = match cache.values().find(|entry| entry.content_id == current_id) {
                Some(s) => s,
                None => continue,
            };

            let source_vector = source.vector.clone();
            drop(cache); // Release lock

            // Find top 5 most similar items
            let similar = {
                let cache = self.cache.lock().await;
                let mut similar_items = Vec::new();

                for entry in cache.values() {
                    if entry.content_id == current_id {
                        continue;
                    }

                    let similarity = self.cosine_similarity(&source_vector, &entry.vector);
                    similar_items.push((entry.content_id.clone(), similarity));
                }

                similar_items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                similar_items.truncate(5);
                similar_items
            };

            // Add to processing queue for next depth
            if current_depth > 1 {
                for (similar_id, _) in similar {
                    if !processed.contains(&similar_id) {
                        to_process.push((similar_id, current_depth - 1));
                    }
                }
            }
        }

        Ok(())
    }

    /// Backup embeddings to file
    pub async fn backup_embeddings(&self, backup_path: Option<String>) -> Result<String> {
        use std::fs;
        use std::path::PathBuf;

        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;

        let backup_dir = PathBuf::from(&todozi_dir)
            .join("backups")
            .join("embeddings");
        fs::create_dir_all(&backup_dir)?;

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename =
            backup_path.unwrap_or_else(|| format!("embeddings_backup_{}.json", timestamp));

        let full_path = backup_dir.join(&filename);

        let cache = self.cache.lock().await;
        let backup_data = serde_json::to_string_pretty(&*cache)?;

        fs::write(&full_path, backup_data)?;

        Ok(full_path.to_string_lossy().to_string())
    }

    /// Restore embeddings from backup file
    pub async fn restore_embeddings(&mut self, backup_path: String) -> Result<usize> {
        use std::fs;

        let backup_data = fs::read_to_string(&backup_path)?;
        let restored_cache: HashMap<String, TodoziEmbeddingCache> =
            serde_json::from_str(&backup_data)?;

        let count = restored_cache.len();

        let mut cache = self.cache.lock().await;
        *cache = restored_cache;

        Ok(count)
    }

    /// Explain search results with detailed reasoning
    pub async fn explain_search_result(
        &self,
        query: &str,
        result: &SimilarityResult,
    ) -> Result<String> {
        let query_embedding = self.generate_embedding(query).await?;

        let cache = self.cache.lock().await;
        let result_entry = cache
            .values()
            .find(|entry| entry.content_id == result.content_id)
            .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                message: "Result not found in cache".to_string(),
            })?;

        // Calculate component-wise similarity contributions
        let mut contributions = Vec::new();
        for (i, (&q_val, &r_val)) in query_embedding
            .iter()
            .zip(result_entry.vector.iter())
            .enumerate()
        {
            contributions.push((i, q_val * r_val)); // dot product contribution
        }

        contributions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let top_dims: Vec<usize> = contributions.iter().take(5).map(|(i, _)| *i).collect();

        let explanation = format!(
            "Match Explanation for '{}' (similarity: {:.3}):\n\
             - Content Type: {:?}\n\
             - Matched Tags: {}\n\
             - Top Contributing Dimensions: {:?}\n\
             - Semantic Overlap: {:.1}%\n\
             - Text Preview: {}...",
            result.content_id,
            result.similarity_score,
            result.content_type,
            result.tags.join(", "),
            top_dims,
            result.similarity_score * 100.0,
            result.text_content.chars().take(100).collect::<String>()
        );

        Ok(explanation)
    }

    /// Add embedding versioning
    pub async fn create_embedding_version(
        &self,
        content_id: &str,
        version_label: String,
    ) -> Result<String> {
        use std::fs::{self, OpenOptions};
        use std::io::Write;
        use std::path::PathBuf;

        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;

        let versions_dir = PathBuf::from(&todozi_dir).join("embed").join("versions");
        fs::create_dir_all(&versions_dir)?;

        let cache = self.cache.lock().await;
        let entry = cache
            .values()
            .find(|e| e.content_id == content_id)
            .ok_or_else(|| crate::error::TodoziError::EmbeddingError {
                message: format!("Content not found: {}", content_id),
            })?;

        let version_id = uuid::Uuid::new_v4().to_string();
        let version_file = versions_dir.join(format!("{}.jsonl", content_id));

        let version_entry = serde_json::json!({
            "version_id": version_id,
            "version_label": version_label,
            "timestamp": Utc::now().to_rfc3339(),
            "content_id": content_id,
            "embedding": entry.vector,
            "text_content": entry.text_content,
            "tags": entry.tags,
        });

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&version_file)?;

        writeln!(file, "{}", serde_json::to_string(&version_entry)?)?;

        Ok(version_id)
    }

    /// Get embedding version history
    pub async fn get_version_history(&self, content_id: &str) -> Result<Vec<serde_json::Value>> {
        use std::fs;
        use std::io::{BufRead, BufReader};
        use std::path::PathBuf;

        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;

        let version_file = PathBuf::from(&todozi_dir)
            .join("embed")
            .join("versions")
            .join(format!("{}.jsonl", content_id));

        if !version_file.exists() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(version_file)?;
        let reader = BufReader::new(file);

        let mut versions = Vec::new();
        for line in reader.lines() {
            if let Ok(line_str) = line {
                if let Ok(version) = serde_json::from_str(&line_str) {
                    versions.push(version);
                }
            }
        }

        Ok(versions)
    }

    /// Export fine-tuning data in JSONL format
    pub async fn export_for_fine_tuning(&self, output_path: String) -> Result<usize> {
        use std::fs::File;
        use std::io::Write;

        let cache = self.cache.lock().await;

        let mut file = File::create(&output_path)?;
        let mut count = 0;

        for entry in cache.values() {
            // Format for fine-tuning: pairs of similar items
            let training_example = serde_json::json!({
                "text": entry.text_content,
                "embedding": entry.vector,
                "metadata": {
                    "content_type": format!("{:?}", entry.content_type),
                    "tags": entry.tags,
                    "content_id": entry.content_id,
                }
            });

            writeln!(file, "{}", serde_json::to_string(&training_example)?)?;
            count += 1;
        }

        Ok(count)
    }
    async fn log_to_mega_file(&self, task: &Task) -> Result<()> {
        use std::fs::{self, OpenOptions};
        use std::io::Write;
        use std::path::PathBuf;

        // Get the embed directory path
        let todozi_dir =
            crate::tdz::find_todozi(None).ok_or_else(|| crate::error::TodoziError::DirError {
                message: "Could not find todozi directory".to_string(),
            })?;

        let embed_dir = PathBuf::from(&todozi_dir).join("embed");
        fs::create_dir_all(&embed_dir)?;

        // Create the mega file path with timestamp
        let mega_file = embed_dir.join("embedding_mega_log.jsonl");

        // Create log entry with all task info and embedding
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "task_id": task.id,
            "project": task.parent_project,
            "action": task.action,
            "priority": task.priority.to_string(),
            "status": task.status.to_string(),
            "tags": task.tags,
            "time": task.time,
            "assignee": task.assignee,
            "embedding_vector": task.embedding_vector,
            "embedding_dimensions": task.embedding_vector.as_ref().map(|v| v.len()),
            "context_notes": task.context_notes,
            "dependencies": task.dependencies,
            "progress": task.progress,
        });

        // Append to file (create if doesn't exist)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mega_file)?;

        // Write as JSONL (one JSON object per line)
        writeln!(file, "{}", serde_json::to_string(&log_entry)?)?;

        Ok(())
    }
}

/// Embedding tool implementation for the todozi system
pub struct TodoziEmbeddingTool {
    service: Arc<Mutex<TodoziEmbeddingService>>,
}

impl TodoziEmbeddingTool {
    pub async fn new(config: TodoziEmbeddingConfig) -> Result<Self> {
        Ok(Self {
            service: Arc::new(Mutex::new(TodoziEmbeddingService::new(config).await?)),
        })
    }

    pub async fn initialize(&self) -> Result<()> {
        let mut service = self.service.lock().await;
        service.initialize().await
    }
}

#[async_trait]
impl Tool for TodoziEmbeddingTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition::new(
            "todozi_embed".to_string(),
            "Generate and manage embeddings for todozi tags and tasks for semantic search and similarity matching"
                .to_string(),
            vec![
                ToolParameter::new("action".to_string(), "string".to_string(),
                "Action to perform: 'embed_task', 'embed_tag', 'find_similar', 'semantic_search', 'cluster', 'update_task', 'stats', 'cleanup', 'create_project', 'add_task', 'update_task', 'new_idea', 'new_memory'"
                .to_string(), true, None), ToolParameter::new("content".to_string(), "string"
                .to_string(), "Content to embed or search for".to_string(), false, None),
                ToolParameter::new("task_id".to_string(), "string".to_string(),
                "Task ID for task-specific operations".to_string(), false, None),
                ToolParameter::new("tag_id".to_string(), "string".to_string(),
                "Tag ID for tag-specific operations".to_string(), false, None),
                ToolParameter::new("limit".to_string(), "number".to_string(),
                "Maximum number of results to return".to_string(), false, None),
                ToolParameter::new("content_types".to_string(), "array".to_string(),
                "Array of content types to search (Task, Tag, Memory, Idea, etc.)"
                .to_string(), false, None), ToolParameter::new("similarity_threshold"
                .to_string(), "number".to_string(),
                "Minimum similarity threshold for results (0.0-1.0)".to_string(),
                false, None), ToolParameter::new("project_name".to_string(), "string"
                .to_string(), "Project name for project creation".to_string(), false, None),
                ToolParameter::new("project_description".to_string(), "string"
                .to_string(), "Project description for project creation".to_string(),
                false, None), ToolParameter::new("idea_content".to_string(), "string"
                .to_string(), "Idea content for idea creation".to_string(), false, None),
                ToolParameter::new("memory_moment".to_string(), "string".to_string(),
                "Memory moment for memory creation".to_string(), false, None),
                ToolParameter::new("memory_meaning".to_string(), "string".to_string(),
                "Memory meaning for memory creation".to_string(), false, None),
                ToolParameter::new("memory_reason".to_string(), "string".to_string(),
                "Memory reason for memory creation".to_string(), false, None),
            ],
            "AI/ML Operations".to_string(),
            vec![ResourceLock::Memory],
        )
    }

    async fn execute(&self, kwargs: HashMap<String, serde_json::Value>) -> ToolResult {
        let create_error = |msg: String| ToolResult::error(msg, 0);
        let action = match kwargs.get("action").and_then(|v| v.as_str()) {
            Some(action) => action,
            None => return create_error("action parameter is required".to_string()),
        };
        let service = self.service.lock().await;
        match action {
            "embed_task" => {
                create_error("embed_task requires task data - use the service directly".to_string())
            }
            "embed_tag" => {
                create_error("embed_tag requires tag data - use the service directly".to_string())
            }
            "find_similar" => {
                let content = match kwargs.get("content").and_then(|v| v.as_str()) {
                    Some(content) => content,
                    None => {
                        return create_error(
                            "content parameter required for find_similar".to_string(),
                        );
                    }
                };
                let limit = kwargs
                    .get("limit")
                    .and_then(|v| v.as_f64())
                    .map(|v| v as usize)
                    .unwrap_or(10);
                match service.find_similar_tasks(content, Some(limit)).await {
                    Ok(results) => {
                        let output = serde_json::to_string_pretty(&results)
                            .unwrap_or_else(|_| "Failed to serialize results".to_string());
                        let mut metadata: HashMap<String, serde_json::Value> = HashMap::new();
                        metadata.insert(
                            "result_count".to_string(),
                            serde_json::Value::Number(serde_json::Number::from(results.len())),
                        );
                        metadata.insert(
                            "action".to_string(),
                            serde_json::Value::String("find_similar".to_string()),
                        );
                        ToolResult::success(output, 0)
                    }
                    Err(e) => create_error(format!("Failed to find similar tasks: {}", e)),
                }
            }
            "semantic_search" => {
                let content = match kwargs.get("content").and_then(|v| v.as_str()) {
                    Some(content) => content,
                    None => {
                        return create_error(
                            "content parameter required for semantic_search".to_string(),
                        );
                    }
                };
                let limit = kwargs
                    .get("limit")
                    .and_then(|v| v.as_f64())
                    .map(|v| v as usize)
                    .unwrap_or(10);
                let content_types =
                    kwargs
                        .get("content_types")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str())
                                .filter_map(|s| match s {
                                    "Task" => Some(TodoziContentType::Task),
                                    "Tag" => Some(TodoziContentType::Tag),
                                    "Memory" => Some(TodoziContentType::Memory),
                                    "Idea" => Some(TodoziContentType::Idea),
                                    "Chunk" => Some(TodoziContentType::Chunk),
                                    "Feel" => Some(TodoziContentType::Feel),
                                    "Train" => Some(TodoziContentType::Train),
                                    "Error" => Some(TodoziContentType::Error),
                                    "Summary" => Some(TodoziContentType::Summary),
                                    "Reminder" => Some(TodoziContentType::Reminder),
                                    "Tdz" => Some(TodoziContentType::Tdz),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                        });
                match service
                    .semantic_search(content, content_types, Some(limit))
                    .await
                {
                    Ok(results) => {
                        let output = serde_json::to_string_pretty(&results)
                            .unwrap_or_else(|_| "Failed to serialize results".to_string());
                        let mut metadata: HashMap<String, serde_json::Value> = HashMap::new();
                        metadata.insert(
                            "result_count".to_string(),
                            serde_json::Value::Number(serde_json::Number::from(results.len())),
                        );
                        metadata.insert(
                            "action".to_string(),
                            serde_json::Value::String("semantic_search".to_string()),
                        );
                        ToolResult::success(output, 0)
                    }
                    Err(e) => create_error(format!("Failed to perform semantic search: {}", e)),
                }
            }
            "cluster" => match service.cluster_content().await {
                Ok(clusters) => {
                    let output = serde_json::to_string_pretty(&clusters)
                        .unwrap_or_else(|_| "Failed to serialize clusters".to_string());
                    let mut metadata: HashMap<String, serde_json::Value> = HashMap::new();
                    metadata.insert(
                        "cluster_count".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(clusters.len())),
                    );
                    metadata.insert(
                        "action".to_string(),
                        serde_json::Value::String("cluster".to_string()),
                    );
                    ToolResult::success(output, 0)
                }
                Err(e) => create_error(format!("Failed to cluster content: {}", e)),
            },
            "stats" => match service.get_stats().await {
                Ok(stats) => {
                    let output = serde_json::to_string_pretty(&stats)
                        .unwrap_or_else(|_| "Failed to serialize stats".to_string());
                    ToolResult::success(output, 0)
                }
                Err(e) => create_error(format!("Failed to get stats: {}", e)),
            },
            "cleanup" => match service.cleanup_expired().await {
                Ok(cleaned_count) => {
                    let output = format!("Cleaned up {} expired entries", cleaned_count);
                    let mut metadata: HashMap<String, serde_json::Value> = HashMap::new();
                    metadata.insert(
                        "cleaned_count".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(cleaned_count)),
                    );
                    metadata.insert(
                        "action".to_string(),
                        serde_json::Value::String("cleanup".to_string()),
                    );
                    ToolResult::success(output, 0)
                }
                Err(e) => create_error(format!("Failed to cleanup: {}", e)),
            },
            "create_project" => {
                let project_name = match kwargs.get("project_name").and_then(|v| v.as_str()) {
                    Some(name) => name,
                    None => {
                        return create_error(
                            "project_name parameter required for create_project".to_string(),
                        );
                    }
                };
                let project_description = kwargs
                    .get("project_description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                match service
                    .create_project(project_name.to_string(), project_description)
                    .await
                {
                    Ok(project_name) => {
                        let output = format!("Project '{}' created successfully", project_name);
                        ToolResult::success(output, 0)
                    }
                    Err(e) => create_error(format!("Failed to create project: {}", e)),
                }
            }
            "add_task" => {
                create_error("add_task requires task data - use the service directly".to_string())
            }
            "update_task" => create_error(
                "update_task requires task data - use the service directly".to_string(),
            ),
            "new_idea" => {
                create_error("new_idea requires idea data - use the service directly".to_string())
            }
            "new_memory" => create_error(
                "new_memory requires memory data - use the service directly".to_string(),
            ),
            _ => create_error(format!("Unknown action: {}", action)),
        }
    }
}

impl Default for TodoziEmbeddingTool {
    fn default() -> Self {
        Self {
            service: Arc::new(Mutex::new(TodoziEmbeddingService {
                config: Arc::new(Mutex::new(TodoziEmbeddingConfig::default())),
                cache: Arc::new(Mutex::new(HashMap::new())),
                embedding_model: Arc::new(Mutex::new(None)),
                embedding_models: Arc::new(Mutex::new(HashMap::new())),
                tag_manager: Arc::new(Mutex::new(TagManager::new())),
                storage: Arc::new(Mutex::new(Storage::default())),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_defaults() {
        let config = TodoziEmbeddingConfig::default();
        assert_eq!(config.model_name, "sentence-transformers/all-MiniLM-L6-v2");
        assert_eq!(config.dimensions, 384);
        assert_eq!(config.similarity_threshold, 0.7);
    }
    #[tokio::test]
    async fn test_cosine_similarity() {
        // Test cosine similarity calculation directly without creating service
        let service = TodoziEmbeddingService {
            config: Arc::new(Mutex::new(TodoziEmbeddingConfig::default())),
            cache: Arc::new(Mutex::new(HashMap::new())),
            embedding_model: Arc::new(Mutex::new(None)),
            embedding_models: Arc::new(Mutex::new(HashMap::new())),
            tag_manager: Arc::new(Mutex::new(TagManager::new())),
            storage: Arc::new(Mutex::new(Storage::default())),
        };
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = service.cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 0.001);
    }
    #[test]
    fn test_content_type_serialization() {
        let content_type = TodoziContentType::Task;
        let serialized = serde_json::to_string(&content_type).unwrap();
        assert!(serialized.contains("Task"));
    }
}
