# Todozi Enhanced Server - Technical Documentation

## Overview

The Todozi Enhanced Server is a comprehensive, async HTTP/TCP server implementing a sophisticated task management and AI-enhanced productivity system. This Python translation maintains the functionality and API semantics of the original Rust implementation while providing a self-contained, runnable server with in-memory storage for development and testing purposes.

## Architecture

### Core Components

#### Server Architecture
- **Async I/O Model**: Built on `asyncio` for high-concurrency request handling
- **Protocol**: HTTP/1.1 over TCP with custom request parsing
- **Storage**: In-memory global state (`STORE` dictionary) with persistent-like interfaces
- **Modular Design**: Separate components for storage, embeddings, AI services, and HTTP handling

#### Key Design Decisions
- **Memory-First Storage**: Uses global dictionaries for rapid prototyping while maintaining persistence interfaces
- **Feature Flags**: TUI functionality controlled via `TDZ_ENABLE_TUI` environment variable
- **Agent System**: Pre-configured with 26 system agents for various capabilities
- **Embedding Integration**: Semantic search and AI insights via embedding service

## Core Classes and Methods

### `TodoziServer` Class

**Purpose**: Main server class handling HTTP connections and request routing

#### Constructor
```python
def __init__(self, config: ServerConfig)
```
- **Parameters**:
  - `config`: ServerConfig object containing host, port, and connection limits
- **Initializes**:
  - Storage instance
  - Code generation graph (10,000 capacity)
  - Embedded services

#### Primary Methods

##### `start()`
```python
async def start(self) -> None
```
Starts the async TCP server and begins accepting connections.

**Flow**:
1. Binds to configured host/port
2. Prints available endpoints
3. Enters infinite serve loop

##### `handle_connection()`
```python
async def handle_connection(self, reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None
```
Handles individual client connections.

**Error Handling**:
- Connection errors logged but not propagated
- Response truncation at `MAX_BUFFER_SIZE` (8KB)
- Graceful writer cleanup

##### `handle_request()`
```python
async def handle_request(self, request: HttpRequest) -> HttpResponse
```
Main request routing and processing logic.

**Authentication Flow**:
1. CORS preflight (OPTIONS) handling
2. Public endpoint whitelist (health checks, API registration)
3. API key extraction from multiple header variations
4. Admin privileges based on private key presence

### Data Models

#### `HttpRequest`
```python
@dataclass
class HttpRequest:
    method: str
    path: str
    headers: Dict[str, str]
    body: str
```
HTTP request representation with parsed components.

#### `HttpResponse`
```python
@dataclass
class HttpResponse:
    status: int
    headers: Dict[str, str]
    body: str
```
HTTP response with convenience methods:
- `ok()`: 200 status with body
- `error()`: Error status with JSON error message
- `json()`: 200 status with JSON-serialized data

#### `ServerConfig`
```python
@dataclass
class ServerConfig:
    host: str = "0.0.0.0"
    port: int = 8636
    max_connections: int = 100
```
Server configuration with sensible defaults.

## Storage System

### `Storage` Class

In-memory storage abstraction providing persistence interfaces.

#### Key Methods

##### `create_project()`
```python
def create_project(self, name: str, description: Optional[str]) -> None
```
Creates a new project with metadata tracking.

##### `get_task_from_any_project()`
```python
def get_task_from_any_project(self, task_id: str) -> Dict[str, Any]
```
Searches global tasks and all project tasks for matching ID.

**Error**: Raises `KeyError` if task not found

##### `update_task_in_project()`
```python
async def update_task_in_project(self, task_id: str, updates: Dict[str, Any]) -> None
```
Applies updates to task, updating timestamp automatically.

## AI Services

### `TodoziEmbeddingService` Class

**Purpose**: Semantic search and AI-powered task analysis

#### Key Features
- Model: sentence-transformers/all-MiniLM-L6-v2
- Task embedding and similarity search
- Content clustering
- Diagnostic exports

#### Primary Methods

##### `add_task()`
```python
async def add_task(self, task: Dict[str, Any]) -> str
```
Adds task to embedding system with automatic vector generation.

##### `semantic_search()`
```python
async def semantic_search(self, query: str, content_types: Optional[List[str]] = None, limit: Optional[int] = 20) -> List[Dict[str, Any]]
```
Finds semantically similar content with similarity scoring.

##### `cluster_content()`
```python
async def cluster_content(self) -> List[Dict[str, Any]]
```
Groups content into semantic clusters for insights.

### `TuiService` Class

**Purpose**: Task visualization and AI insights (feature-flagged)

**Dependencies**: Requires `TDZ_ENABLE_TUI=1` environment variable

## API Endpoints

### Core System
- `GET /health` - Health check with system status
- `GET /stats` - Comprehensive system statistics
- `GET /init` - System initialization

### Task Management
```python
# Create task with AI embedding
POST /tasks
{
    "action": "Implement user authentication",
    "time": "2 hours",
    "priority": "high"
}

# Semantic task search
GET /tasks/search?q=authentication

# AI-powered task insights (TUI required)
GET /tasks/{id}/insights
```

### Agent System
```python
# List all 26 system agents
GET /agents

# Create custom agent
POST /agents
{
    "name": "Code Review Specialist",
    "description": "Specializes in code quality analysis",
    "category": "code"
}
```

### Memory & Training Data
- **Memories**: Long-term context storage
- **Ideas**: Creative concept tracking  
- **Training Data**: AI model training corpus
- **Feelings**: Emotional state tracking

### Queue Management
```python
# Time tracking integration
POST /time/start/{task_id}
POST /time/stop/{task_id}

# Queue session management
POST /queue/start/{item_id}
POST /queue/end/{session_id}
```

## Authentication System

### API Key Management
```python
# Register new API key
POST /api/register
# Response: 
{
    "user_id": "user_abc123",
    "public_key": "pk_...",
    "private_key": "sk_...",
    "active": true
}

# Authentication check
POST /api/check
{
    "public_key": "pk_...",
    "private_key": "sk_..."  # Optional for admin
}
```

**Privilege Model**:
- Public key only: Read-only access
- Public + private key: Admin privileges

## Error Handling

### HTTP Status Codes
- `200`: Success
- `400`: Bad request (validation errors)
- `401`: Unauthorized (authentication failure)
- `404`: Not found (resource missing)
- `500`: Internal server error

### Exception Handling
```python
try:
    # Route processing
except ValueError as ve:
    return HttpResponse.error(400, f"Bad request: {ve}")
except KeyError as ke:
    return HttpResponse.error(404, f"Not found: {ke}")
except PermissionError as pe:
    return HttpResponse.error(401, f"Unauthorized: {pe}")
except Exception as e:
    return HttpResponse.error(500, "Internal Server Error")
```

## Performance Considerations

### Memory Usage
- **Global State**: All data stored in `STORE` dictionary
- **Connection Limits**: Configurable max connections (default: 100)
- **Buffer Size**: 8KB request limit prevents memory exhaustion

### Async Patterns
- Non-blocking I/O for high concurrency
- Connection pooling via `asyncio.start_server`
- Stream-based request processing

### Search Performance
- Semantic search uses embedding vectors for fast similarity matching
- In-memory storage enables rapid data access
- Clustering operations async to prevent blocking

## Dependencies and Requirements

### Core Dependencies
```python
import asyncio      # Async I/O
import json         # JSON serialization
import uuid         # ID generation
from datetime import datetime, timezone  # Timestamp handling
```

### External Services
- **Embedding Service**: `todozi.emb.TodoziEmbeddingService`
- **Memory Manager**: `todozi.memory.MemoryManager` 
- **Idea Manager**: `todozi.idea.IdeaManager`

### Environment Variables
```bash
TDZ_ENABLE_TUI=1    # Enable TUI features (default: disabled)
```

## Configuration

### Server Configuration
```python
config = ServerConfig(
    host="0.0.0.0",      # Bind address
    port=8636,           # Listen port
    max_connections=100  # Concurrent connections
)
```

### Embedding Configuration
```python
embedding_config = {
    "model_name": "sentence-transformers/all-MiniLM-L6-v2"
}
```

## Usage Examples

### Basic Server Startup
```python
async def main():
    await start_server("localhost", 8636)

if __name__ == "__main__":
    asyncio.run(main())
```

### Task Creation with AI
```python
# Create task with semantic embedding
task_data = {
    "action": "Refactor authentication middleware",
    "time": "3 hours",
    "priority": "medium",
    "tags": ["security", "refactor"]
}

# The task will be automatically embedded for semantic search
```

### Semantic Search
```python
# Find similar tasks
similar_tasks = await embedding_service.semantic_search(
    "user login system", 
    limit=10
)
```

## Limitations and Constraints

### Storage Limitations
- **Volatile Data**: All data lost on server restart
- **Memory Bound**: Limited by available RAM
- **No Persistence**: No database or file system backing

### Feature Limitations
- **TUI Dependency**: Advanced insights require TUI feature flag
- **Agent Customization**: Limited agent modification capabilities
- **Queue Persistence**: Session data not persisted across restarts

### Performance Constraints
- **Single-threaded**: Async but single-process
- **No Caching**: Repeated operations may be expensive
- **Embedding Overhead**: AI operations add computational cost

## Future Enhancements

### Planned Improvements
1. **Persistent Storage**: Database integration
2. **Distributed Architecture**: Multi-server deployment
3. **Advanced Analytics**: Real-time performance metrics
4. **Plugin System**: Extensible agent capabilities

### Integration Points
- External authentication providers
- Cloud storage backends
- Message queue systems for async processing
- Monitoring and observability tools

This documentation provides comprehensive coverage of the Todozi Enhanced Server implementation. The system represents a sophisticated balance between AI-powered functionality and practical task management, suitable for both development experimentation and potential production deployment with persistent storage integration.