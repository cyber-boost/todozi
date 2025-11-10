# Todozi Extraction Pipeline Technical Documentation

## Overview

The Todozi Extraction Pipeline is a Python-based system for processing and extracting structured information from unstructured text content. It provides a complete, runnable implementation of the Rust extraction pipeline with enhanced error handling, configuration management, and structured data validation.

## Architecture and Design Decisions

### Core Architecture Patterns
- **Hierarchical Error Handling**: Structured exception hierarchy for different failure modes
- **Configuration Management**: Centralized config loading with precedence rules (CLI > Environment > HLX file)
- **API Client Abstraction**: Async HTTP client with proper resource management
- **Data Validation**: Pydantic models for type-safe data structures
- **Dependency Injection**: Testable service interfaces with default implementations
- **Structured Logging**: Configurable logging with structlog integration

### Key Design Decisions
1. **Async-First Design**: All I/O operations use asyncio for optimal performance
2. **Defensive Programming**: Extensive input validation and error handling
3. **Separation of Concerns**: Clear boundaries between data models, API clients, and business logic
4. **Extensibility**: Plugin architecture for embedding services and output formats

## Dependencies and Requirements

### Runtime Requirements
- **Python**: 3.9 or higher
- **Core Dependencies**:
  - `pydantic==1.10.15`: Data validation and serialization
  - `aiohttp`: Async HTTP client
  - `aiofiles`: Async file operations
  - `structlog`: Structured logging (optional)

### Optional Dependencies
- `structlog`: Enhanced logging capabilities (falls back to standard logging if unavailable)

## Configuration Management

### `TodoziConfig` Class

```python
@dataclass
class TodoziConfig:
    api_key: str
    user_id: str = ""
    fingerprint: str = ""
```

#### Configuration Loading Precedence
1. **CLI Arguments**: Highest priority, direct user input
2. **Environment Variables**: `TDZ_API_KEY`, fallback option
3. **HLX Configuration File**: `~/.todozi/tdz.hlx` JSON file for persistent settings

#### Methods

##### `load(cls, cli_api_key: Optional[str] = None, cli_user_id: Optional[str] = None, cli_fingerprint: Optional[str] = None) -> "TodoziConfig"`

**Purpose**: Load configuration from multiple sources with proper precedence

**Parameters**:
- `cli_api_key`: Command-line provided API key (highest priority)
- `cli_user_id`: Command-line provided user ID
- `cli_fingerprint`: Command-line provided fingerprint

**Returns**: Fully initialized `TodoziConfig` instance

**Exceptions**:
- `ConfigError`: Failed to read HLX configuration file
- `ValidationError`: Missing required API key

**Usage Example**:
```python
config = await TodoziConfig.load(
    cli_api_key="key123",
    cli_user_id="user456"
)
```

## Error Handling Hierarchy

### Exception Classes

#### `TodoziError` (Base Exception)
**Purpose**: Root exception for all Todozi-related errors

#### `ValidationError(TodoziError)`
**Purpose**: Invalid input data or configuration validation failures
**Usage**: Data validation, configuration checks

#### `APIError(TodoziError)`
**Purpose**: External API communication failures
**Usage**: HTTP errors, network issues, API response validation

#### `ConfigError(TodoziError)`
**Purpose**: Configuration loading and parsing errors
**Usage**: File I/O errors, malformed configuration

## Data Models (Pydantic)

### Core Extraction Models

#### `ExtractedTask`
**Purpose**: Represents extracted task information

**Fields**:
- `action: str` - Task description/action
- `time: str` - Time specification
- `priority: str` - Priority level
- `project: str` - Project association
- `status: str` - Current status
- `assignee: Optional[str]` - Responsible person
- `tags: List[str]` - Associated tags

#### `ExtractedMemory`
**Purpose**: Represents extracted memory information

**Fields**:
- `moment: str` - Memory moment/event
- `meaning: str` - Significance/meaning
- `reason: str` - Reason for remembering
- `importance: str` - Importance level
- `term: str` - Classification term

#### `ExtractedIdea`
**Purpose**: Represents extracted idea information

**Fields**:
- `idea: str` - Idea description
- `share: str` - Sharing specification
- `importance: str` - Importance level

#### `ExtractedError`
**Purpose**: Represents extracted error information

**Fields**:
- `title: str` - Error title
- `description: str` - Detailed description
- `severity: str` - Severity level
- `category: str` - Error category

#### `ExtractedTrainingData`
**Purpose**: Represents training data pairs

**Fields**:
- `prompt: str` - Training prompt
- `completion: str` - Expected completion
- `data_type: str` - Data type classification

### Response Container

#### `ExtractResponse`
**Purpose**: Container for all extracted data types

**Fields**:
- `tasks: List[ExtractedTask]` - Extracted tasks
- `memories: List[ExtractedMemory]` - Extracted memories
- `ideas: List[ExtractedIdea]` - Extracted ideas
- `errors: List[ExtractedError]` - Extracted errors
- `training_data: List[ExtractedTrainingData]` - Training data
- `raw_tags: List[str]` - Unprocessed tags

**Methods**:
- `to_json(indent: int = 2, ensure_ascii: bool = False) -> str`: Serialize to JSON format

## API Client

### `TodoziAPIClient` Class

```python
class TodoziAPIClient:
    def __init__(self, session: ClientSession, api_key: str) -> None
```

#### Methods

##### `extract_content(self, endpoint: str, content: str, user_id: str, fingerprint: str, model: str = DEFAULT_MODEL, language: str = DEFAULT_LANGUAGE, extract_all: bool = True) -> Dict[str, Any]`

**Purpose**: Send extraction request to Todozi API

**Parameters**:
- `endpoint: str` - API endpoint path
- `content: str` - Text content to extract from
- `user_id: str` - User identification
- `fingerprint: str` - Client fingerprint
- `model: str` - AI model to use (default: "gpt-oss:120b")
- `language: str` - Content language (default: "english")
- `extract_all: bool` - Whether to extract all data types

**Returns**: Raw API response as dictionary

**Exceptions**:
- `APIError`: HTTP errors or network failures

**Performance Considerations**:
- Timeout: 120 seconds total
- Async HTTP requests for non-blocking operation

### Context Manager

#### `get_api_client(config: TodoziConfig)`

**Purpose**: Async context manager for API client with proper session management

**Usage Example**:
```python
async with get_api_client(config) as client:
    response = await client.extract_content("extract", content, user_id, fingerprint)
```

## Embedding Service

### Service Interface Classes

#### `TaskLike`
**Purpose**: Extended task representation for embedding service

**Fields**: Includes additional metadata beyond basic extraction

#### `MemoryLike`
**Purpose**: Extended memory representation for embedding service

#### `IdeaLike`
**Purpose**: Extended idea representation for embedding service

### `TodoziEmbeddingService` Class

**Purpose**: Default implementation of embedding service for testing and development

**Methods**:
- `add_task(task: TaskLike) -> str`: Store task and return ID
- `new_memory(memory: MemoryLike) -> str`: Store memory and return ID
- `new_idea(idea: IdeaLike) -> str`: Store idea and return ID

## Parsing and Serialization

### Response Parsing

#### `parse_extract_response(api_response: Dict[str, Any]) -> ExtractResponse`

**Purpose**: Safely parse raw API response into validated Pydantic models

**Parameters**: Raw API response dictionary
**Returns**: Validated `ExtractResponse` instance
**Exceptions**: `ValidationError` on malformed API response

**Defensive Features**:
- Type checking for all list elements
- Graceful handling of missing or malformed data
- Comprehensive validation with meaningful error messages

### Output Formatting

#### `format_as_csv(response: ExtractResponse) -> str`

**Purpose**: Format extracted data as CSV
**Returns**: CSV-formatted string with proper escaping
**Features**: Handles all data types with type-specific columns

#### `format_as_markdown(response: ExtractResponse) -> str`

**Purpose**: Format extracted data as Markdown
**Returns**: Markdown-formatted string with hierarchical structure
**Features**: Proper section organization and formatting

## Utility Functions

### `hash_project_name(name: str) -> str`
**Purpose**: Generate SHA1 hash for project names
**Use Case**: Creating unique identifiers for projects

### `now_utc() -> datetime`
**Purpose**: Get current UTC timestamp
**Use Case**: Time-stamping operations

### `format_timestamp_for_filename(dt: Optional[datetime] = None) -> str`
**Purpose**: Format timestamp for file naming
**Format**: `YYYYMMDD_HHMMSS`

### `format_timestamp_for_display(dt: Optional[datetime] = None) -> str`
**Purpose**: Format timestamp for human-readable display
**Format**: `YYYY-MM-DD HH:MM:SS UTC`

## Logging System

### Configuration
- **Default**: INFO level and above
- **JSON Format**: Enabled via `TODOZI_LOG_JSON=1` environment variable
- **Fallback**: Standard logging if structlog unavailable

### Loggers
- Primary logger: `todozi.extract`
- Structured logging with context information

## Performance Considerations

### Async Operations
- All I/O operations use async/await for concurrent execution
- HTTP requests with 120-second timeout to handle large content
- File operations use aiofiles for non-blocking I/O

### Memory Management
- Pydantic models enforce memory-efficient data structures
- Streaming processing for large content (implementation detail)
- Proper resource cleanup via context managers

## Error Handling and Edge Cases

### Input Validation
- API key presence validation
- Content type and length checks (implied)
- HLX configuration file parsing with error recovery

### API Communication
- HTTP status code validation (400+ treated as errors)
- Network timeout handling
- Response JSON parsing with validation

### Data Processing
- Malformed API response handling
- Missing optional fields gracefully handled
- Type coercion and validation at all stages

## Usage Patterns

### Command Line Interface
```bash
# Extract from text content
python todozi_extract.py extract --content "your text" --format json

# Extract from file with human-readable output
python todozi_extract.py strategy --file path/to/file.txt --format md --human
```

### Programmatic Usage
```python
# Basic extraction flow
config = await TodoziConfig.load(api_key="your_key")
async with get_api_client(config) as client:
    response_data = await client.extract_content(
        "extract", 
        content_text, 
        config.user_id, 
        config.fingerprint
    )
    parsed_response = parse_extract_response(response_data)
    print(parsed_response.to_json())
```

## Technical Constraints and Limitations

### Current Limitations
- **HLX Integration**: Registry and HLX parts not fully implemented
- **Error Recovery**: Limited retry mechanisms for transient failures
- **Rate Limiting**: No built-in API rate limiting handling
- **Large Files**: Memory-intensive for very large content files

### Dependencies
- **Pydantic Version**: Locked to 1.10.15 for compatibility
- **Python Version**: Requires 3.9+ for modern async features
- **Network**: Requires internet connectivity for API calls

## Testing Considerations

### Testable Components
- `TodoziEmbeddingService`: In-memory implementation for testing
- Configuration loading: Mockable file and environment dependencies
- API client: Mockable HTTP responses
- Parsing functions: Pure functions with clear input/output

### Mock Strategies
- Mock `aiohttp.ClientSession` for API testing
- Mock file system for configuration testing
- Dependency injection for service testing

This documentation provides comprehensive coverage of the Todozi Extraction Pipeline architecture, components, and usage patterns. The system is designed for reliability, testability, and extensibility while maintaining clear separation of concerns and robust error handling.