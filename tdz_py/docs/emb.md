# Todozi Embedding Service Technical Documentation

## Overview

The Todozi Embedding Service is a comprehensive semantic embedding system designed for task management and content organization. It provides text embedding generation, similarity search, clustering, and analytics capabilities using sentence-transformers models.

## Architecture and Design Decisions

### Core Architecture
- **Service-Oriented Design**: Modular components with clear separation of concerns
- **Embedding-Centric**: All content types (tasks, tags, ideas, memories) are converted to text embeddings
- **In-Memory Cache**: Optimized for frequent similarity operations with TTL-based caching
- **Extensible Model Support**: Multiple embedding model support with comparison capabilities

### Key Design Decisions
1. **Sentence-Transformers Integration**: Leverages pre-trained models for high-quality embeddings
2. **Cosine Similarity**: Standard similarity metric for semantic search
3. **Hierarchical Caching**: Multi-level caching strategy with LRU eviction
4. **Content-Type Agnostic**: Unified embedding approach across different content types
5. **Batch Processing Support**: Efficient handling of multiple embeddings simultaneously

## Dependencies and Requirements

### Core Dependencies
```python
# Required packages
sentence-transformers >= 2.2.0
torch >= 1.9.0
transformers >= 4.20.0
tokenizers >= 0.12.0
pydantic >= 1.9.0
numpy >= 1.21.0
```

### Optional Dependencies
- `tqdm`: For progress indication during batch operations
- `scikit-learn`: Enhanced clustering algorithms (planned)

## Data Models

### Configuration Models

#### `TodoziEmbeddingConfig`
**Purpose**: Main configuration container for embedding service

**Fields**:
- `model_name` (str): Pre-trained model identifier (default: "sentence-transformers/all-MiniLM-L6-v2")
- `dimensions` (int): Expected embedding dimensions (default: 384)
- `similarity_threshold` (float): Minimum similarity score for results (default: 0.7)
- `max_results` (int): Maximum results per query (default: 50)
- `cache_ttl_seconds` (int): Cache time-to-live in seconds (default: 86400)
- `enable_clustering` (bool): Enable clustering functionality (default: True)
- `clustering_threshold` (float): Similarity threshold for clustering (default: 0.8)

#### `TodoziEmbeddingCache`
**Purpose**: Cache entry for embedding vectors

**Fields**:
- `vector` (List[float]): Embedding vector
- `content_type` (str): Type of content (Task, Tag, Idea, Memory)
- `content_id` (str): Unique identifier
- `text_content` (str): Original text content
- `tags` (List[str]): Associated tags
- `created_at` (datetime): Creation timestamp
- `ttl_seconds` (int): Time-to-live duration

### Content Models

#### `Task`
**Purpose**: Represents a todo item with full metadata

**Fields**:
- `id` (str): Unique identifier
- `action` (str): Task description
- `parent_project` (str): Project association
- `priority` (str): Priority level (Low, Medium, High, Critical)
- `status` (str): Current status (todo, pending, in_progress, etc.)
- `tags` (List[str]): Categorization labels
- `time` (Optional[float]): Estimated time requirement
- `assignee` (Optional[str]): Responsible person
- `context_notes` (Optional[str]): Additional context
- `dependencies` (List[str]): Dependent task IDs
- `progress` (Optional[int]): Completion percentage
- `embedding_vector` (Optional[List[float]]): Cached embedding

#### `Tag`, `Idea`, `Memory`
**Purpose**: Additional content types with type-specific fields

## Core Components

### `EmbeddingModel` Class

**Purpose**: Wrapper around sentence-transformers for embedding generation

#### Methods

##### `__init__(model_name: str, device: str = "cpu")`
**Parameters**:
- `model_name`: Pre-trained model identifier
- `device`: Computation device ("cpu" or "cuda")

**Exceptions**:
- `RuntimeError`: If sentence-transformers is not available

##### `encode(texts: List[str]) -> List[List[float]]`
**Parameters**:
- `texts`: List of text strings to embed

**Returns**: List of embedding vectors

**Behavior**:
- Lazy model loading on first call
- Automatic normalization of embeddings
- Fallback to random vectors if ML backend unavailable

##### `encode_single(text: str) -> List[float]`
**Parameters**:
- `text`: Single text string to embed

**Returns**: Single embedding vector

### `TodoziEmbeddingService` Class

**Purpose**: Main service class providing embedding operations

#### Initialization

##### `__init__(config: TodoziEmbeddingConfig, tag_manager: Optional[TagManager] = None, storage: Optional[Storage] = None)`
**Parameters**:
- `config`: Service configuration
- `tag_manager`: Optional tag management component
- `storage`: Optional storage backend

#### Core Operations

##### `initialize(device: str = "cpu") -> None`
**Purpose**: Initialize embedding model

**Parameters**:
- `device`: Computation device

##### `generate_embedding(text: str) -> List[float]`
**Purpose**: Generate embedding for single text

**Parameters**:
- `text`: Input text

**Returns**: Embedding vector

##### `generate_embeddings_batch(texts: List[str]) -> List[List[float]]`
**Purpose**: Generate embeddings for multiple texts

**Parameters**:
- `texts`: List of input texts

**Returns**: List of embedding vectors

**Performance**: Optimized batch processing

#### Search Operations

##### `find_similar_tasks(task_description: str, limit: Optional[int] = None) -> List[SimilarityResult]`
**Purpose**: Find tasks similar to given description

**Parameters**:
- `task_description`: Query text
- `limit`: Maximum results (defaults to config.max_results)

**Returns**: List of similar tasks with similarity scores

##### `semantic_search(query: str, content_types: Optional[List[str]] = None, limit: Optional[int] = None) -> List[SimilarityResult]`
**Purpose**: General semantic search across content types

**Parameters**:
- `query`: Search query
- `content_types`: Filter by content types
- `limit`: Maximum results

**Returns**: Similarity results across all cached content

##### `hybrid_search(query: str, keywords: List[str], content_types: Optional[List[str]], semantic_weight: float = 0.7, limit: int = 20) -> List[SimilarityResult]`
**Purpose**: Combine semantic and keyword search

**Parameters**:
- `query`: Semantic query
- `keywords`: Keyword list
- `content_types`: Content type filter
- `semantic_weight`: Weight for semantic vs keyword (0.0-1.0)
- `limit`: Maximum results

**Returns**: Hybrid search results

#### Clustering Operations

##### `cluster_content() -> List[ClusteringResult]`
**Purpose**: Cluster cached content by similarity

**Returns**: List of clusters with member items

**Algorithm**: Greedy similarity-based clustering

##### `hierarchical_clustering(content_types: List[str], max_depth: int = 2) -> List[HierarchicalCluster]`
**Purpose**: Create hierarchical clusters

**Parameters**:
- `content_types`: Content types to include
- `max_depth`: Maximum hierarchy depth

**Returns**: Hierarchical cluster tree

#### Analytics and Diagnostics

##### `validate_embeddings() -> ValidationReport`
**Purpose**: Validate embedding quality and integrity

**Returns**: Validation report with issues

**Checks**:
- NaN values
- Infinite values
- Zero vectors
- Distribution abnormalities

##### `profile_search_performance(query: str, iterations: int = 10) -> PerformanceMetrics`
**Purpose**: Profile search performance

**Parameters**:
- `query`: Test query
- `iterations`: Number of test runs

**Returns**: Performance statistics

##### `export_diagnostics() -> DiagnosticReport`
**Purpose**: Export service diagnostics

**Returns**: Comprehensive diagnostic report

## Utility Components

### `LRUEmbeddingCache` Class

**Purpose**: Least Recently Used cache with memory limits

#### Methods

##### `__init__(max_memory_mb: int)`
**Parameters**:
- `max_memory_mb`: Maximum cache size in megabytes

##### `get(key: str) -> Optional[TodoziEmbeddingCache]`
**Parameters**:
- `key`: Cache key

**Returns**: Cached entry or None

**Behavior**: Updates access count and moves to front

##### `insert(key: str, value: TodoziEmbeddingCache)`
**Parameters**:
- `key`: Cache key
- `value`: Cache entry

**Behavior**: Evicts LRU entries if memory limit exceeded

### `TodoziEmbeddingTool` Class

**Purpose**: Simplified wrapper for tool integration

## Technical Constraints and Limitations

### Model Constraints
- **Model Compatibility**: Requires sentence-transformers compatible models
- **Dimension Consistency**: All embeddings must have consistent dimensions
- **Memory Requirements**: Large models may require significant RAM

### Performance Considerations
- **Cache Size**: Large caches impact memory usage
- **Batch Size**: Very large batches may cause memory issues
- **Similarity Calculations**: O(n²) complexity for full similarity matrix

### Limitations
- **No Persistent Storage**: Cache is in-memory only (backup/restore available)
- **Single Node**: No distributed clustering support
- **Limited Scalability**: Designed for single-user scale

## Error Handling and Edge Cases

### Common Error Scenarios

#### Model Loading Failures
```python
try:
    await service.initialize()
except RuntimeError as e:
    # Handle missing dependencies
    print(f"Model initialization failed: {e}")
```

#### Invalid Embeddings
```python
report = await service.validate_embeddings()
if report.invalid_embeddings > 0:
    for issue in report.issues:
        print(f"Issue with {issue.content_id}: {issue.description}")
```

#### Empty Results
```python
results = await service.semantic_search("query")
if not results:
    # Handle no matches
    print("No similar content found")
```

### Edge Cases
- **Zero-length texts**: Handled with fallback embeddings
- **Duplicate content**: Deduplication not automatic
- **Cache expiration**: TTL-based refresh mechanism
- **Model version changes**: No automatic model migration

## Usage Examples

### Basic Initialization
```python
config = TodoziEmbeddingConfig(
    model_name="sentence-transformers/all-MiniLM-L6-v2",
    similarity_threshold=0.75
)
service = TodoziEmbeddingService(config)
await service.initialize()
```

### Adding and Searching Tasks
```python
# Create task
task = Task(
    id="task_001",
    action="Implement user authentication",
    parent_project="webapp",
    tags=["auth", "security", "backend"]
)
await service.add_task(task)

# Search similar tasks
results = await service.find_similar_tasks("user login system", limit=5)
for result in results:
    print(f"Similarity: {result.similarity_score:.3f} - {result.text_content}")
```

### Advanced Analytics
```python
# Performance profiling
metrics = await service.profile_search_performance("important task", iterations=100)
print(f"Average search time: {metrics.avg_time_ms:.2f}ms")

# Clustering analysis
clusters = await service.cluster_content()
for cluster in clusters:
    print(f"Cluster {cluster.cluster_id}: {cluster.cluster_size} items")
```

### Backup and Recovery
```python
# Backup embeddings
backup_path = await service.backup_embeddings()
print(f"Backup created: {backup_path}")

# Restore from backup
count = await service.restore_embeddings(backup_path)
print(f"Restored {count} embeddings")
```

## Performance Optimization

### Cache Optimization
- **TTL Configuration**: Adjust based on update frequency
- **Memory Limits**: Set appropriate max_memory_mb for LRU cache
- **Access Patterns**: Frequently accessed items stay in cache

### Search Optimization
- **Threshold Tuning**: Higher thresholds reduce result processing
- **Batch Operations**: Use batch methods for multiple embeddings
- **Content Filtering**: Limit search to relevant content types

### Memory Management
- **Vector Size**: Consider lower-dimensional models for memory constraints
- **Cache Eviction**: Monitor cache hit rates for optimal size
- **Garbage Collection**: Regular cache validation and cleanup

## Integration Patterns

### Web Service Integration
```python
class EmbeddingAPI:
    def __init__(self, service: TodoziEmbeddingService):
        self.service = service
    
    async def search_endpoint(self, query: str, limit: int = 10):
        results = await self.service.semantic_search(query, limit=limit)
        return {"results": self._serialize_results(results)}
```

### Batch Processing
```python
async def process_task_batch(tasks: List[Task], service: TodoziEmbeddingService):
    # Prepare content in batch
    texts = [service.prepare_task_content(task) for task in tasks]
    
    # Generate embeddings in batch
    embeddings = await service.generate_embeddings_batch(texts)
    
    # Update tasks with embeddings
    for task, embedding in zip(tasks, embeddings):
        task.embedding_vector = embedding
```

This documentation provides comprehensive coverage of the Todozi Embedding Service architecture, components, and usage patterns. The service is designed for flexibility and extensibility while maintaining performance for semantic search and content organization tasks.