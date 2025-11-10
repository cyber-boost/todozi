# Tool Definitions Library Technical Documentation

## Overview

The Tool Definitions Library is a production-ready Python translation of a Rust "tool" library implementing a comprehensive tool management system. This library provides a structured framework for defining, validating, and executing tools with sophisticated error handling, serialization capabilities, and configuration management.

## Architecture and Design Decisions

### Core Design Principles

- **Type Safety**: Robust type validation system supporting JSON-compatible types
- **Error Resilience**: Comprehensive error handling with detailed categorization
- **Serialization Compatibility**: Full support for JSON and Ollama API formats
- **Interface Segregation**: Clear separation between abstractions and implementations
- **Configuration-Driven**: Flexible behavior through `ToolConfig` class

### Module Structure

```
tool_defs.py
├── Enums (ResourceLock, ErrorType)
├── Data Structures (ToolParameter, ToolDefinition, ToolResult)
├── Error Handling (ToolError, ErrorHandler)
├── Core Abstractions (Tool, ToolRegistryTrait)
├── Implementations (ToolRegistry)
├── Factory Helpers
└── Internal Tests
```

## Core Components

### ResourceLock Enum

**Purpose**: Defines resource access restrictions for tool operations.

```python
class ResourceLock(Enum):
    FILESYSTEM_WRITE = "filesystem_write"
    FILESYSTEM_READ = "filesystem_read"
    GIT = "git"
    MEMORY = "memory"
    SHELL = "shell"
    NETWORK = "network"
```

**Methods**:
- `display_name`: Returns PascalCase representation (e.g., "FilesystemWrite")
- `as_str()`: Returns snake_case value for serialization
- `__str__()`: Defaults to `display_name`

**Usage Example**:
```python
lock = ResourceLock.FILESYSTEM_WRITE
print(lock.display_name)  # "FilesystemWrite"
print(lock.as_str())      # "filesystem_write"
```

### ErrorType Enum

**Purpose**: Standardized error categorization for consistent error handling.

```python
class ErrorType(Enum):
    VALIDATION_ERROR = "validation_error"
    PERMISSION_ERROR = "permission_error"
    # ... additional error types
```

## Data Structures

### ToolParameter Class

**Purpose**: Defines individual tool parameters with validation metadata.

**Properties**:
- `name` (str): Parameter identifier
- `type_` (str): JSON-compatible type (string, number, boolean, array, object, integer, null)
- `description` (str): Human-readable description
- `required` (bool): Whether parameter is mandatory
- `default` (Any): Optional default value

**Methods**:
- `to_dict()`: Serializes to JSON-friendly dictionary

**Usage Example**:
```python
param = ToolParameter(
    name="file_path",
    type_="string",
    description="Path to target file",
    required=True
)
serialized = param.to_dict()
```

### ToolDefinition Class

**Purpose**: Complete specification of a tool's interface and capabilities.

**Properties**:
- `name` (str): Tool identifier
- `description` (str): Tool functionality description
- `parameters` (List[ToolParameter]): Input parameters
- `category` (str): Functional categorization
- `resource_locks` (List[ResourceLock]): Required resource access

**Key Methods**:

#### `to_ollama_format() -> Dict[str, Any]`
Serializes tool definition to Ollama LLM tool-call API format.

**Output Structure**:
```json
{
    "type": "function",
    "function": {
        "name": "tool_name",
        "description": "tool_description",
        "parameters": {
            "type": "object",
            "properties": {
                "param_name": {
                    "type": "param_type",
                    "description": "param_description"
                }
            },
            "required": ["mandatory_params"]
        }
    }
}
```

#### `validate() -> List[str]`
Performs consistency validation on tool definition.

**Validation Rules**:
- No duplicate parameter names
- Valid parameter types (string, number, boolean, array, object, integer, null)
- Returns list of error messages (empty list indicates validation success)

### ToolResult Class

**Purpose**: Standardized result container for tool executions.

**Properties**:
- `success` (bool): Execution status
- `output` (str): Primary result data
- `error` (Optional[str]): Error message if failed
- `execution_time_ms` (int): Performance metric
- `metadata` (Optional[Dict[str, Any]]): Additional context
- `recovery_context` (Optional[Dict[str, Any]]): Recovery information

**Factory Methods**:
- `success(output: str, execution_time_ms: int = 0)`: Successful result constructor
- `error(error: str, execution_time_ms: int = 0)`: Error result constructor

**Serialization**:
- `to_dict(exclude_none: bool = True)`: Converts to JSON-compatible dictionary

## Error Handling System

### ToolError Exception

**Purpose**: Domain-specific exception with error categorization.

```python
class ToolError(Exception):
    def __init__(
        self,
        message: str,
        error_type: ErrorType,
        details: Optional[Dict[str, Any]] = None
    )
```

### ErrorHandler Class

**Purpose**: Centralized error processing and validation utilities.

#### `_is_valid_type(value: Any, expected_type: str) -> bool`
Validates value against expected JSON type with extended support.

**Supported Types**: string, number, boolean, array, object, integer, null

**Special Cases**:
- `null` type accepts `None` values
- Unknown types return `True` to avoid false positives

#### `handle_error(error: BaseException, context: str) -> ToolResult`
Converts exceptions to standardized ToolResult with error classification.

**Error Mapping**:
- `ToolError`: Preserves original error type and details
- `OSError/IOError`: Mapped to `RESOURCE_ERROR`
- `TimeoutError`: Mapped to `TIMEOUT_ERROR`
- Other exceptions: Mapped to `INTERNAL_ERROR`

#### Validation Helpers
- `validate_required_params()`: Checks for missing required parameters
- `validate_string_param()`: String validation with length and pattern constraints
- `create_success_result()` / `create_error_result()`: Result factory methods

## Core Abstractions

### Tool Abstract Base Class

**Purpose**: Defines interface that all concrete tools must implement.

**Required Implementations**:
- `definition` property: Returns `ToolDefinition`
- `execute(kwargs: Dict[str, Any])` method: Async tool execution

**Default Implementation**:
- `validate_parameters()`: Validates input against tool definition

### ToolRegistryTrait Interface

**Purpose**: Defines registry contract for tool management.

```python
class ToolRegistryTrait(ABC):
    @abstractmethod
    def has_tool(self, name: str) -> bool
```

## ToolRegistry Implementation

### Configuration

**ToolConfig Class**:
- `validate_parameters`: Enable/disable parameter validation
- `strict_mode`: Enable strict validation mode
- `default_timeout_ms`: Default execution timeout

### Core Functionality

#### Registration Management
- `register(tool: Tool)`: Add tool to registry
- `unregister(name: str)`: Remove tool by name
- `clear()`: Remove all tools

#### Tool Discovery
- `get_tool(name: str) -> Optional[Tool]`: Retrieve tool by name
- `get_all_tools() -> List[Tool]`: List all registered tools
- `has_tool(name: str) -> bool`: Check tool existence

#### Execution Pipeline
```python
async def execute_tool(tool_name: str, kwargs: Dict[str, Any]) -> ToolResult
```

**Execution Flow**:
1. Tool resolution by name
2. Parameter validation (if enabled)
3. Async tool execution
4. Result packaging with metadata

#### Serialization Support
- `get_tool_definitions() -> List[Dict[str, Any]]`: Ollama format for all tools

### Context Manager Support
```python
with ToolRegistry() as registry:
    registry.register(my_tool)
    result = await registry.execute_tool("tool_name", params)
# Automatic cleanup on exit
```

## Factory Helpers

**Purpose**: Mirror Rust create_* API for fluent object creation.

**Available Factories**:
- `create_tool_parameter()`: Basic parameter without default
- `create_tool_parameter_with_default()`: Parameter with default value
- `create_tool_definition()`: Definition without resource locks
- `create_tool_definition_with_locks()`: Definition with resource locks

## Technical Constraints and Limitations

### Type System Limitations
- Limited to JSON-compatible types
- No support for complex Python-specific types
- Type validation is runtime-based (not compile-time)

### Performance Considerations
- Parameter validation adds overhead (configurable via ToolConfig)
- Large tool registries may impact memory usage
- Async execution requires proper event loop management

### Error Handling Constraints
- Error classification is heuristic-based for non-ToolError exceptions
- Recovery context must be manually populated
- Metadata serialization may expose sensitive information

## Dependencies and Requirements

### Python Version
- Python 3.7+ (for `from __future__ import annotations`)
- Async/await support required

### Standard Library Dependencies
- `json`: Serialization support
- `logging`: Structured logging
- `re`: Pattern validation
- `dataclasses`: Value object implementation
- `enum`: Enumeration types
- `abc`: Abstract base classes
- `typing`: Type hints

## Testing Framework

### Internal Test Suite
Mirrors Rust #[cfg(test)] functionality with comprehensive validation:

**Test Coverage**:
- Tool parameter creation and serialization
- Ollama format generation
- Registry operations (registration, querying)
- Error handler validation logic
- Tool result display formatting
- Definition validation rules

**Execution**:
```bash
python tool_defs.py
# Runs internal test suite with success/failure reporting
```

## Usage Patterns

### Basic Tool Implementation
```python
class FileReadTool(Tool):
    @property
    def definition(self) -> ToolDefinition:
        return create_tool_definition(
            name="file_read",
            description="Read file contents",
            category="File Operations",
            parameters=[create_tool_parameter("path", "string", "File path", True)]
        )
    
    async def execute(self, kwargs: Dict[str, Any]) -> ToolResult:
        try:
            with open(kwargs["path"], 'r') as f:
                content = f.read()
            return ToolResult.success(content)
        except Exception as e:
            return ErrorHandler.handle_error(e, "file_read")
```

### Registry Management
```python
# Configuration-driven registry
config = ToolConfig(validate_parameters=True, strict_mode=False)
registry = ToolRegistry(config)

# Tool registration
registry.register(FileReadTool())

# Batch execution
results = []
for tool_request in requests:
    result = await registry.execute_tool(
        tool_request.name, 
        tool_request.params
    )
    results.append(result)
```

### Error Recovery Pattern
```python
try:
    result = await tool.execute(params)
    if not result.success:
        # Check error type for recovery strategy
        if result.metadata.get("error_type") == "validation_error":
            # Retry with corrected parameters
            pass
except ToolError as e:
    # Domain-specific error handling
    logger.error(f"Tool execution failed: {e}")
```

This documentation provides comprehensive coverage of the Tool Definitions Library's architecture, components, and usage patterns. The library offers a robust foundation for tool management systems with production-ready error handling, serialization, and configuration capabilities.