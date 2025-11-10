# Todozi Client Technical Documentation

## Overview

The Todozi Client is a Python library that provides a structured interface to the Todozi API. This implementation is a translation from Rust code with enhanced error handling, type safety, and configuration management.

## Key Features

- **Result-based error handling** with idiomatic Result[T, E] pattern
- **Configurable endpoint mapping** with multiple URL construction styles
- **Comprehensive input validation** and safe parameter access
- **Async HTTP client** with timeout support
- **Structured logging** for observability
- **Type-safe enumerations** for HTTP methods and endpoint styles

## Architecture and Design

### Core Design Principles

1. **Functional Error Handling**: Uses a Result monad pattern for consistent error propagation
2. **Separation of Concerns**: Clear separation between parsing, configuration, and execution
3. **Extensibility**: Configurable endpoint mappings allow easy API evolution
4. **Safety**: Safe parameter access and input validation prevent runtime errors

### Module Structure

```
Todozi Client
├── Error Handling (Result/TodoziError)
├── Data Models (TdzCommand, EndpointConfig)
├── Configuration (TodoziConfig, EndpointStyle)
├── Utilities (safe_get_param, validate_command)
└── Core Functions (parsing, execution, processing)
```

## Data Types and Models

### Result Type

```python
class Result:
    """
    A functional Result[T, E] type for error handling.
    
    Properties:
        is_ok: bool - True if result contains success value
        is_err: bool - True if result contains error
        
    Methods:
        unwrap() -> Any: Returns success value or raises error
        unwrap_or(default: Any) -> Any: Returns value or default
        map_or(default: Any, f: Callable) -> Any: Applies function to value or returns default
        map_err(f: Callable) -> Result: Transforms error value
    """
```

### TdzCommand Model

```python
@dataclass
class TdzCommand:
    """
    Represents a parsed Todozi command.
    
    Attributes:
        command: str - Action to perform (list, get, create, update, delete, run, search)
        target: str - Resource type (task, agent, memory, etc.)
        parameters: List[str] - Positional parameters for the command
        options: Dict[str, str] - Key-value options for the command
    """
```

### Endpoint Configuration System

```python
class EndpointStyle(Enum):
    """
    Defines how URL endpoints are constructed from commands.
    
    Values:
        STATIC: Fixed path (e.g., "/health")
        PARAM: Single parameter substitution (e.g., "/tasks/{p0}")
        PARAMS_2: Two parameter substitution (e.g., "/feelings/{p0}/{p1}")
        QUERY: Query parameter construction (e.g., "/tasks/search?q={p0}")
    """

@dataclass
class EndpointConfig:
    """
    Configuration for a specific command-target combination.
    
    Attributes:
        style: EndpointStyle - How to construct the URL
        path: str - Base path template for the endpoint
    """
```

## Core Functions and Methods

### Command Parsing

```python
def parse_tdz_command(text: str) -> Result[List[TdzCommand], TodoziError]:
    """
    Extracts TdzCommand objects from text containing <tdz>...</tdz> tags.
    
    Parameters:
        text: str - Input text containing zero or more command blocks
        
    Returns:
        Result[List[TdzCommand], TodoziError]: 
            Success: List of parsed commands
            Error: TodoziError with parsing failure details
            
    Command Format:
        <tdz>command; target; param1; param2; key=value; key2=value2</tdz>
    
    Example:
        Input: "<tdz>create; task; urgent; priority=high</tdz>"
        Output: TdzCommand("create", "task", ["urgent"], {"priority": "high"})
    """
```

### Endpoint Resolution

```python
def get_endpoint_path(command: TdzCommand) -> str:
    """
    Resolves the API endpoint path for a given command.
    
    Parameters:
        command: TdzCommand - The command to resolve
        
    Returns:
        str - Fully constructed endpoint path
        
    Resolution Process:
        1. Lookup (command, target) in endpoint mapping
        2. Apply EndpointStyle template substitution
        3. Fallback to "/{target}" if no mapping exists
    """
```

### Request Body Construction

```python
def build_request_body(command: TdzCommand) -> Result[dict, TodoziError]:
    """
    Constructs JSON request body based on command target and options.
    
    Parameters:
        command: TdzCommand - Command with options to convert to JSON
        
    Returns:
        Result[dict, TodoziError]: Structured request body
        
    Supported Targets:
        - task: Action, time, priority, project, status, assignee, tags
        - memory: Moment, meaning, reason, importance, term, memory_type, emotion
        - agent: Name, description, capabilities
        - feeling: Emotion, intensity, description, context, tags
        - training: Data type, prompt, completion, source, context, tags, quality_score
    """
```

### Command Execution

```python
async def execute_tdz_command(
    command: TdzCommand,
    base_url: str,
    api_key: Optional[str] = None,
    timeout_total: float = DEFAULT_TIMEOUT_TOTAL_SECONDS,
) -> Result[dict, TodoziError]:
    """
    Executes a single TdzCommand against the Todozi API.
    
    Parameters:
        command: TdzCommand - Command to execute
        base_url: str - Base URL of the Todozi API
        api_key: Optional[str] - API key for authentication
        timeout_total: float - Total request timeout in seconds (default: 30)
        
    Returns:
        Result[dict, TodoziError]: API response JSON or error
        
    HTTP Method Mapping:
        list/get/search → GET
        create → POST  
        update → PUT
        delete → DELETE
        run/execute → POST
    """
```

### Batch Processing

```python
async def process_tdz_commands(
    text: str,
    base_url: str,
    api_key: Optional[str] = None,
    timeout_total: float = DEFAULT_TIMEOUT_TOTAL_SECONDS,
) -> Result[List[dict], TodoziError]:
    """
    Processes multiple TdzCommands from text and executes sequentially.
    
    Parameters:
        text: str - Text containing multiple <tdz> command blocks
        base_url: str - Base URL of the Todozi API
        api_key: Optional[str] - API key for authentication
        timeout_total: float - Timeout per request in seconds
        
    Returns:
        Result[List[dict], TodoziError]: List of API responses or first error encountered
        
    Note: Commands are executed in order; first error terminates the sequence
    """
```

## Configuration System

### Endpoint Mapping

The `TodoziConfig` class defines the mapping between command-target pairs and their corresponding API endpoints:

```python
class TodoziConfig:
    """
    Centralized endpoint configuration with fallback behavior.
    
    Usage:
        config = TodoziConfig()
        path = config.get_endpoint(command)
    
    Fallback Strategy:
        If (command, target) not found, defaults to "/{target}"
        
    Extensibility:
        Add new entries to _endpoints dictionary to support new API endpoints
    """
```

### Default Mappings

The library includes comprehensive mappings for:

- **Core Resources**: tasks, agents, memories, ideas, training
- **Analytics**: tasks, agents, performance statistics  
- **Time Tracking**: start/stop timers, reporting
- **Queue Management**: backlog, active, complete items
- **Chat System**: processing, history
- **Error Management**: listing, searching, CRUD operations

## Error Handling

### TodoziError Class

```python
class TodoziError(Exception):
    """
    Standardized error type for Todozi client operations.
    
    Attributes:
        message: str - Human-readable error description
    """
```

### Error Scenarios

The library handles the following error conditions:

1. **Parsing Errors**: Malformed command syntax, regex failures
2. **Validation Errors**: Missing required command fields
3. **Network Errors**: Timeouts, connection failures, HTTP errors
4. **JSON Errors**: Malformed API responses
5. **Configuration Errors**: Unknown command-target combinations

### Error Recovery

- **Safe Parameter Access**: `safe_get_param` prevents index errors
- **Default Values**: Strategic use of defaults for optional parameters
- **Graceful Degradation**: Unknown targets return empty request bodies

## Performance Considerations

### Timeout Management

- **Default Timeout**: 30 seconds total per request
- **Configurable**: Timeout can be adjusted per invocation
- **Async Operations**: Non-blocking HTTP requests using aiohttp

### Memory Efficiency

- **Lazy Parsing**: Commands parsed on demand from text
- **Streaming Responses**: aiohttp handles large responses efficiently
- **Minimal Data Copying**: Direct parameter mapping to request bodies

## Dependencies and Requirements

### Runtime Dependencies

```python
# Required packages
aiohttp>=3.8.0      # Async HTTP client
dataclasses         # Data class support (Python 3.7+)
typing_extensions   # Enhanced type hints (if needed)
```

### Python Version Compatibility

- **Minimum**: Python 3.7+
- **Recommended**: Python 3.8+ for improved async features
- **Type Hints**: Full type annotation support

### Environment Requirements

```python
def find_todozi(s: Optional[str] = None) -> Optional[str]:
    """
    Locates Todozi configuration directory.
    
    Returns:
        Optional[str]: Path to ~/.todozi or subdirectory
        
    Environment Variables:
        HOME - Required for configuration directory discovery
    """
```

## Usage Examples

### Basic Single Command

```python
import asyncio
from todozi_client import process_tdz_commands

async def list_tasks():
    text = "<tdz>list; tasks</tdz>"
    result = await process_tdz_commands(
        text, 
        base_url="https://api.todozi.com",
        api_key="your-api-key"
    )
    
    if result.is_ok:
        tasks = result.unwrap()
        print(f"Found {len(tasks)} tasks")
    else:
        error = result.unwrap()
        print(f"Error: {error.message}")

asyncio.run(list_tasks())
```

### Complex Command with Options

```python
text = """
<tdz>create; task; urgent project; priority=high; project=marketing; tags=urgent,client</tdz>
<tdz>update; agent; agent123; status=active; capabilities=chat,analysis</tdz>
"""

result = await process_tdz_commands(text, base_url, api_key)
```

### Custom Configuration

```python
from todozi_client import TodoziConfig, EndpointConfig, EndpointStyle

# Extend with custom endpoints
config = TodoziConfig()
config._endpoints[("custom", "operation")] = EndpointConfig(
    EndpointStyle.PARAM, 
    "/custom/{p0}"
)
```

## Limitations and Constraints

### Technical Limitations

1. **Sequential Execution**: Commands execute sequentially, not concurrently
2. **Error Propagation**: First error terminates batch processing
3. **Response Size**: Large responses must fit in memory
4. **Authentication**: Currently supports only API key authentication

### API Constraints

1. **Command Syntax**: Strict `<tdz>...</tdz>` tag requirements
2. **Parameter Order**: Positional parameters must be in correct order
3. **Option Parsing**: Simple key=value format without nesting
4. **URL Length**: Query-style endpoints subject to URL length limits

### Error Handling Constraints

- No automatic retry mechanism for transient failures
- Limited error context preservation in batch operations
- No circuit breaker pattern for API failure detection

## Testing Considerations

### Unit Testing

- Mock aiohttp responses for HTTP testing
- Test all EndpointStyle variations
- Validate error conditions and recovery

### Integration Testing

- Test against real API with valid credentials
- Verify endpoint mappings match actual API
- Test timeout and network failure scenarios

## Future Enhancements

### Planned Improvements

1. **Concurrent Execution**: Parallel command processing
2. **Enhanced Authentication**: OAuth2, JWT support
3. **Response Streaming**: Handle large responses efficiently
4. **Retry Logic**: Configurable retry with exponential backoff
5. **Metrics Collection**: Performance monitoring and logging

### Extension Points

- Custom endpoint configuration loading
- Plugin system for additional command types
- Response transformation hooks
- Custom authentication providers

This documentation provides comprehensive coverage of the Todozi Client implementation. For specific usage questions or bug reports, consult the source code comments and test cases.