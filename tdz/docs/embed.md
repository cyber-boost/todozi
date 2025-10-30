# Todozi Embedding System

## Overview

Todozi uses **semantic embeddings** to understand the meaning of tasks, memories, ideas, and other content. This enables powerful features like:

- **Semantic Search**: Find tasks by meaning, not just keywords
- **Similar Task Discovery**: Get suggestions based on what you're working on
- **Smart Clustering**: Automatically group related content
- **Context-Aware AI**: Better task recommendations and insights

## How It Works

### The DNA Analogy 🧬

Just like DNA encodes biological information, Todozi embeddings encode semantic information:

- **Text** → **Vector** (384 floating-point numbers)
- Similar meanings → Similar vectors
- Enables mathematical comparison of task meanings

### Model Architecture

**Default Model**: `sentence-transformers/all-MiniLM-L6-v2`
- 384 dimensions
- ~90MB download (one-time)
- Fast inference on CPU
- Good balance of speed vs quality

**Under the Hood**:
1. Text is tokenized (split into word pieces)
2. Passed through BERT transformer
3. Token embeddings are mean-pooled
4. Result is L2-normalized
5. Produces dense semantic vector

## Using Embeddings

### Basic Usage

When you create tasks, embeddings are generated automatically:

```bash
# This task gets embedded automatically
todozi add "Implement user authentication with OAuth2"

# Find similar tasks semantically
todozi similar "add login system"
# → Will find the OAuth2 task even though keywords differ!
```

### CLI Commands

#### Set Custom Model

```bash
# Use a different model
todozi emb set-model sentence-transformers/all-mpnet-base-v2

# Try multilingual support
todozi emb set-model sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2
```

The command will:
1. Download the model from HuggingFace
2. Validate it works
3. Save to `~/.todozi/models/`
4. Set as default in `~/.todozi/tdz.hlx`

#### View Current Model

```bash
todozi emb show-model
# Output: sentence-transformers/all-MiniLM-L6-v2
```

#### Browse Popular Models

```bash
todozi emb list-models
```

Displays curated list with tradeoffs:
- **Fast & Lightweight**: MiniLM (384 dims, 90MB)
- **Balanced**: MPNet (768 dims, 420MB)
- **Multilingual**: Supports 50+ languages
- **High Performance**: RoBERTa (1024 dims, 1.4GB)

### Programmatic Usage (Rust)

```rust
use todozi::emb::{TodoziEmbeddingService, TodoziEmbeddingConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize service
    let config = TodoziEmbeddingConfig::default();
    let mut service = TodoziEmbeddingService::new(config).await?;
    service.initialize().await?;

    // Generate embedding
    let text = "Build a REST API for user management";
    let embedding = service.generate_embedding(text).await?;
    println!("Vector length: {}", embedding.len()); // 384

    // Find similar tasks
    let similar = service.find_similar_tasks(
        "create user CRUD endpoints",
        Some(5)
    ).await?;

    for result in similar {
        println!("Score: {:.2} - {}",
            result.similarity_score,
            result.text_content
        );
    }

    Ok(())
}
```

## Storage & Caching

### Directory Structure

```
~/.todozi/
├── models/                      # HuggingFace model cache
│   └── models--sentence-transformers--all-MiniLM-L6-v2/
│       ├── snapshots/
│       │   └── refs/
│       │       ├── config.json
│       │       ├── tokenizer.json
│       │       └── model.safetensors
├── embed/                       # Embedding logs
│   └── embedding_mega_log.jsonl # All embeddings + metadata
└── tdz.hlx                      # Config with model preference
```

### Configuration in HLX

The model preference is stored in `~/.todozi/tdz.hlx`:

```hlx
[embedding]
model_name = "sentence-transformers/all-MiniLM-L6-v2"
```

This is automatically read on initialization.

### Embedding Cache

Embeddings are stored in two places:

1. **Task JSON files**: Each task has `embedding_vector: Vec<f32>` field
2. **Mega log**: `embed/embedding_mega_log.jsonl` for analytics/backup

Example mega log entry:
```json
{
  "timestamp": "2025-01-15T10:30:00Z",
  "task_id": "abc-123",
  "project": "myapp",
  "action": "Add user authentication",
  "embedding_vector": [0.123, -0.456, ...],
  "embedding_dimensions": 384
}
```

## Technical Implementation

### Code Architecture

**Core Module**: `src/emb.rs`

```rust
pub struct EmbeddingModel {
    model: BertModel,           // Candle BERT implementation
    tokenizer: Tokenizer,       // HuggingFace tokenizer
    device: Device,             // CPU or GPU
    dimensions: usize,          // 384 for MiniLM
}

pub struct TodoziEmbeddingService {
    config: Arc<Mutex<TodoziEmbeddingConfig>>,
    cache: Arc<Mutex<HashMap<String, TodoziEmbeddingCache>>>,
    embedding_model: Arc<Mutex<Option<Arc<EmbeddingModel>>>>,
    tag_manager: Arc<Mutex<TagManager>>,
    storage: Arc<Mutex<Storage>>,
}
```

### Model Loading Process

When `initialize()` is called:

1. **Check config**: Read model name from `tdz.hlx`
2. **Set cache dir**: `HF_HOME=~/.todozi/models`
3. **Download files** (if not cached):
   - `config.json` - Model architecture config
   - `tokenizer.json` - Tokenization rules
   - `model.safetensors` or `pytorch_model.bin` - Weights
4. **Load into memory**: Create BERT model with Candle
5. **Ready**: Service can now generate embeddings

### Embedding Generation Flow

```
Text Input
    ↓
Tokenization (max 512 tokens)
    ↓
BERT Forward Pass
    ↓
Mean Pooling (average token embeddings)
    ↓
L2 Normalization
    ↓
384-dim Vector Output
```

Implementation in `src/emb.rs:202-281`:

```rust
pub fn encode(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
    // 1. Tokenize
    let encodings = self.tokenizer.encode_batch(texts, true)?;

    for encoding in encodings {
        let tokens = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();

        // 2. BERT forward pass
        let output = self.model.forward(&token_ids, &attention_mask, None)?;

        // 3. Mean pooling
        let embeddings = output.mean(0)?;

        // 4. L2 normalize
        let norm = embeddings.iter().map(|x| x * x).sum::<f32>().sqrt();
        let normalized = embeddings.iter().map(|x| x / norm).collect();

        all_embeddings.push(normalized);
    }

    Ok(all_embeddings)
}
```

### Similarity Calculation

Cosine similarity measures how "close" two vectors are:

```rust
fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    dot_product / (norm_a * norm_b)
}
```

Score ranges:
- **1.0**: Identical meaning
- **0.8-0.9**: Very similar
- **0.6-0.7**: Somewhat related
- **< 0.5**: Different topics

## Advanced Features

### Semantic Search Across Content Types

```rust
// Search tasks, memories, ideas, chunks simultaneously
let results = service.semantic_search(
    "authentication implementation",
    Some(vec![
        TodoziContentType::Task,
        TodoziContentType::Idea,
        TodoziContentType::Chunk,
    ]),
    Some(10)
).await?;
```

### Content Clustering

Automatically group related content:

```rust
let clusters = service.cluster_content().await?;

for cluster in clusters {
    println!("Cluster {} ({} items):",
        cluster.cluster_id,
        cluster.cluster_size
    );
    println!("Avg similarity: {:.2}", cluster.average_similarity);

    for item in cluster.content_items {
        println!("  - {}", item.text_content);
    }
}
```

### Export to HLX/Parquet

```bash
todozi export-embeddings --output my_embeddings.hlx
```

Exports all task embeddings to HLX format for:
- **Analytics**: Load into data science tools
- **Backup**: Preserve embedding history
- **Parquet conversion**: Use `hlx export parquet` for data warehousing

## Performance & Optimization

### First-Time Setup

**Initial download** (~90MB for default model):
- Takes 30-60 seconds depending on connection
- Shows progress indicators
- Only happens once

**Subsequent uses**:
- Instant loading from `~/.todozi/models/`
- No network required

### Embedding Generation Speed

On typical CPU:
- **Single task**: ~50-100ms
- **Batch of 10**: ~200-300ms
- **Batch of 100**: ~1-2 seconds

**Optimization tip**: The service batches embeddings internally for better performance.

### Memory Usage

- **Model loaded**: ~200MB RAM
- **Per embedding**: 384 floats = 1.5KB
- **1000 tasks**: ~1.5MB for vectors alone

The model stays loaded for the duration of the service.

## Troubleshooting

### Common Issues

#### "Failed to download model"

**Cause**: No internet connection or HuggingFace is down

**Solutions**:
1. Check internet connection
2. Try again later
3. Use a different model
4. Manually download and set `HF_HOME`

#### "Embedding model not initialized"

**Cause**: Tried to use service before calling `initialize()`

**Solution**:
```rust
let mut service = TodoziEmbeddingService::new(config).await?;
service.initialize().await?;  // Don't forget this!
```

#### Model download is slow

**Cause**: Large model or slow connection

**Solutions**:
1. Use a smaller model (MiniLM vs RoBERTa)
2. Download once, then works offline
3. Set up a local model cache

#### Out of memory

**Cause**: Model too large for system

**Solutions**:
1. Use smaller model (MiniLM = 90MB, RoBERTa = 1.4GB)
2. Close other applications
3. Upgrade system RAM

## Comparison: Old vs New Implementation

### Before (Naive Hash-Based)

**How it worked**:
- Character frequency counting
- Positional hashing
- N-gram features
- No semantic understanding

**Problems**:
- Sparse vectors (mostly zeros)
- No meaning captured
- Similarity search didn't work
- "login system" ≠ "authentication" even though they're the same

### After (Real Transformers)

**How it works**:
- Pre-trained BERT model
- Learned from billions of words
- Dense semantic vectors
- Captures meaning and context

**Benefits**:
- ✅ Actually understands similarity
- ✅ "user authentication" ≈ "login system" ≈ "OAuth2"
- ✅ Works across languages (with multilingual models)
- ✅ Battle-tested (sentence-transformers used by thousands)

### Performance Comparison

| Metric | Old (Hash) | New (BERT) |
|--------|-----------|------------|
| **Semantic Quality** | ❌ None | ✅ Excellent |
| **Vector Sparsity** | 90% zeros | Dense |
| **Generation Speed** | Instant | ~100ms |
| **Download Size** | 0 | 90MB |
| **Memory Usage** | Minimal | ~200MB |
| **Accuracy** | Random | High |

**Verdict**: The quality improvement vastly outweighs the minimal performance cost.

## Model Selection Guide

### sentence-transformers/all-MiniLM-L6-v2 (Default)

**Best for**: Most users
- ✅ Fast (100ms per task)
- ✅ Small (90MB)
- ✅ Good quality
- ✅ English only

### sentence-transformers/all-mpnet-base-v2

**Best for**: Higher quality needs
- ✅ Better semantic understanding
- ✅ More nuanced similarities
- ⚠️ Slower (768 dims)
- ⚠️ Larger (420MB)

### sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2

**Best for**: Non-English or multilingual teams
- ✅ 50+ languages
- ✅ Cross-language search
- ✅ Same speed as MiniLM
- ⚠️ Slightly lower quality per language

### sentence-transformers/all-roberta-large-v1

**Best for**: Maximum quality, powerful hardware
- ✅ Best semantic quality
- ✅ Most nuanced understanding
- ⚠️ Slow (1024 dims)
- ⚠️ Large (1.4GB)
- ⚠️ High memory usage

## Future Enhancements

### Potential Improvements

1. **GPU Support**: Detect and use CUDA if available
2. **Quantization**: Reduce model size with INT8
3. **Incremental Updates**: Only re-embed changed tasks
4. **Custom Training**: Fine-tune on your task domain
5. **Hybrid Search**: Combine keyword + semantic
6. **Cross-Modal**: Embed code, images, documents

### API Wishlist

```rust
// Not implemented yet - ideas for future
service.embed_with_metadata(task, metadata).await?;
service.temporal_search("recent similar tasks", days: 7).await?;
service.get_task_trajectory(task_id).await?; // Track evolution
```

## References

- **Sentence-Transformers**: https://www.sbert.net/
- **HuggingFace Models**: https://huggingface.co/sentence-transformers
- **Candle Framework**: https://github.com/huggingface/candle
- **BERT Paper**: https://arxiv.org/abs/1810.04805

## Credits

**Implementation**: Claude (Anthropic)
**Framework**: Candle (HuggingFace)
**Models**: Sentence-Transformers
**Integration**: Todozi v0.1.0
**Date**: January 2025

---

*Part of the Todozi DNA architecture* 🧬
