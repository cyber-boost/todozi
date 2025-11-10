# Todozi Content Processing System Technical Documentation

## Overview

The Todozi Content Processing System is a comprehensive framework for extracting structured data from conversational content, processing tool calls, and managing todo items, memories, ideas, and other structured data types. The system provides both high-level processing functions and granular tool-based processing capabilities.

## Architecture and Design Decisions

### Core Architecture Principles
- **Modular Design**: Separates content parsing, data extraction, tool execution, and state management
- **Async-First**: Built on asyncio for non-blocking I/O operations
- **Shared State Management**: Thread-safe state management with locking mechanisms
- **Extensible Tool System**: Abstract base class for tool implementations
- **Pydantic Validation**: Strong typing and validation for all data models

### Key Design Decisions
1. **Dual Processing Approach**: Both high-level (`tdz_cnt`) and tool-based processing paths
2. **Tag-based Extraction**: Uses XML-style tags for structured data extraction
3. **Session Management**: Automatic conversation session tracking and management
4. **Natural Language Processing**: Heuristic-based extraction of checklist items from conversational text
5. **Error Resilience**: Graceful handling of malformed content and external tool failures

## Core Components

### 1. Error Handling System

#### `TodoziError` Class
```python
class TodoziError(Exception):
    def __init__(self, message: str, code: Optional[int] = None):
        super().__init__(message)
        self.code = code
```

**Description**: Base exception class for all Todozi-specific errors with optional error codes.

**Methods**:
- `storage_error(message: str) -> "TodoziError"`: Creates storage-related errors (code 500)
- `validation_error(message: str) -> "TodoziError"`: Creates validation errors (code 400)

**Usage Pattern**:
```python
raise TodoziError.validation_error("Invalid content format")
```

### 2. Tool Framework

#### Type Definitions
```python
JsonValue = Any  # Flexible JSON value type
ResourceLock = str  # Resource locking labels
```

#### `ToolDefinition` Model
**Purpose**: Defines metadata for tools including parameters, descriptions, and resource requirements.

**Fields**:
- `name`: Unique tool identifier
- `description`: Human-readable tool description
- `parameters`: List of parameter definitions
- `category`: Tool categorization
- `resource_locks`: Resources the tool requires

**Factory Method**:
- `new()`: Creates a new tool definition with validated parameters

#### `ToolResult` Model
**Purpose**: Standardized result format for tool executions.

**Fields**:
- `success`: Boolean indicating operation success
- `output`: Result content or error message
- `code`: Numeric status code

**Factory Methods**:
- `success(output: str, code: int = 0)`: Creates successful result
- `error(message: str, code: int = 400)`: Creates error result

#### `Tool` Abstract Base Class
```python
class Tool(ABC):
    @property
    @abstractmethod
    def definition(self) -> ToolDefinition: ...
    
    @abstractmethod
    async def execute(self, kwargs: Dict[str, Any]) -> ToolResult: ...
```

**Contract**: All tools must implement the `definition` property and `execute` method.

### 3. Domain Models

#### Core Data Structures

**`ChecklistItem`**: Represents individual todo items
```python
class ChecklistItem(BaseModel):
    id: str
    content: str
    priority: str  # "high", "medium", "low"
    completed: bool
    created_at: datetime
    source: str  # "natural_language", "tag", "tool_call"
```

**`ProcessedContent`**: Tracks content processing metadata
```python
class ProcessedContent(BaseModel):
    id: str
    session_id: str
    raw_content: str
    cleaned_content: str
    timestamp: datetime
    extracted_items: List[str]
    checklist_items: List[ChecklistItem]
    tool_calls: List[ExtractedAction]
    processing_stats: ProcessingStats
```

**`ConversationSession`**: Manages conversation context
```python
class ConversationSession(BaseModel):
    id: str
    start_time: datetime
    last_activity: datetime
    topic: str
    participant_count: int
    message_count: int
```

#### State Management

**`TodoziProcessorState`**: Central state container
```python
class TodoziProcessorState(BaseModel):
    active_sessions: Dict[str, ConversationSession]
    recent_actions: List[ProcessedAction]
    checklist_items: List[ChecklistItem]
    processed_contents: List[ProcessedContent]
```

**Key Methods**:
- `add_checklist_item()`: Adds items with automatic validation
- `add_recent_action()`: Maintains rolling window of recent actions (max 100)
- `save_processed_content()`: Archives processed content with metadata

### 4. Content Models (`ChatContent`)

**Purpose**: Structured representation of extracted content types.

**Supported Content Types**:
- **Tasks**: Action items with priority and context
- **Memories**: Significant moments with meaning and reasoning
- **Ideas**: Creative concepts and insights
- **Code Chunks**: Code snippets with language specification
- **Errors**: Error reports with title and detail
- **Training Data**: Learning material
- **Feelings**: Emotional context
- **Summaries**: Content summaries
- **Reminders**: Time-based prompts
- **Agent Assignments**: Agent task delegations

## Core Processing Components

### 1. Content Parser (`parse_chat_message_extended`)

**Function Signature**:
```python
def parse_chat_message_extended(content: str, system_hint: str) -> ChatContent
```

**Processing Logic**:
- **Tag Parsing**: Extracts content between XML-style tags
- **Heuristic Processing**: Applies pattern matching for each content type
- **Normalization**: Cleans and standardizes extracted content

**Supported Tags**:
- `<todozi>`: Task items with priority detection
- `<memory>`: Memory items in "moment; meaning; reason" format
- `<idea>`: Simple idea strings
- `<error>`: Error reports in "title::detail" or "title - detail" format
- `<chunk lang="...">`: Code blocks with optional language specification
- Various other tags for feelings, summaries, reminders, etc.

**Example Usage**:
```python
content = "Let's <todozi>p:high Fix critical bug; urgent</todozi>"
result = parse_chat_message_extended(content, "system")
# result.tasks[0] contains TaskItem with action="Fix critical bug", priority="high"
```

### 2. Content Processor Tool (`TdzContentProcessorTool`)

#### Tool Definition
- **Name**: `tdz_content_processor`
- **Category**: Content Processing
- **Resource Locks**: FilesystemRead, FilesystemWrite

#### Parameters
1. `content` (required): Raw content to process (string)
2. `session_id` (optional): Conversation session identifier
3. `extract_checklist` (optional): Enable natural language checklist extraction
4. `auto_session` (optional): Enable automatic session management

#### Processing Pipeline

**`process_content` Method Flow**:
1. **Parse Raw Content**: Determine if content is JSON or plain text
2. **Data Extraction**: Extract tags and tool calls
3. **Tool Call Processing**: Execute identified tool calls
4. **Content Cleaning**: Remove extracted tags from original content
5. **State Updates**: Update sessions, actions, and checklist items
6. **Response Generation**: Compile processing summary

**Content Type Detection**:
```python
def parse_raw_content(self, content: str) -> ParsedContent:
    try:
        json_value = json.loads(content)
        return self.parse_json_content(json_value)
    except Exception:
        return self.parse_text_content(content)
```

**JSON Content Processing**:
- Extracts content from `content`, `message`, and `choices` fields
- Identifies tool calls from `tool_calls` array
- Handles nested JSON structures recursively

### 3. Natural Language Processing

#### Pattern Extraction
```python
natural_language_patterns = [
    r"we should", r"I need to", r"let's", r"we need to",
    r"don't forget", r"remember to", r"make sure",
    r"important:", r"note:", r"todo:", r"add to checklist"
]
```

**Extraction Logic**:
- Matches patterns case-insensitively
- Extracts complete sentences or thought fragments
- Applies length constraints (10-200 characters)
- Deduplicates similar extractions

#### Checklist Item Extraction
```python
def extract_checklist_items(self, text: str) -> List[ChecklistItem]
```

**Patterns Recognized**:
- "add to checklist/list/todo"
- "we need to/should have/do"
- Reminder patterns: "don't forget to", "remember to"
- Imperative patterns: "make sure to", "need to", "have to", "must"

**Item Creation**:
- Generates unique UUID for each item
- Sets default priority to "medium"
- Tracks extraction source as "natural_language"
- Prevents duplicate items through content normalization

### 4. Tool Call Processing

**Supported Tool Types**:
- **Task Management**: `create_task`, `add_task`, `complete_task`
- **Search Operations**: `search`, `list`
- **Update Operations**: `update`, `complete`
- **Knowledge Management**: `memory`, `idea`

**Execution Pattern**:
```python
async def process_tool_calls(self, tool_calls: List[JsonValue]) -> ProcessingResult
```

**Tool Call Identification**:
- Extracts function name from tool call objects
- Uses keyword matching for tool classification
- Falls back to `unknown_tool_call` for unrecognized tools

**External Command Execution**:
```python
async def execute_binary_command(self, command: str, args: List[str])
```
- Runs blocking subprocess commands asynchronously
- Captures stdout/stderr for result reporting
- Maintains async compatibility through thread pooling

## High-Level Processing Function

### `tdz_cnt` Function
```python
async def tdz_cnt(content: str, session_id: Optional[str] = None) -> str
```

**Processing Flow**:
1. **Parse Content**: Extract all structured data using `parse_chat_message_extended`
2. **Process Items**: Execute appropriate actions for each content type
3. **Content Cleaning**: Remove all Todozi tags from original content
4. **Response Generation**: Compile processing summary and clean content
5. **Traditional Processing**: Execute tool-based processing for compatibility

**Item Processing**:
- **Tasks**: Calls `Done.create_task()` with extracted parameters
- **Memories**: Calls `Memories.create()` with moment/meaning/reason
- **Ideas**: Calls `Ideas.create()` with idea content
- **Errors**: Logs errors through `storage.save_error()`

**Response Format**:
```json
{
    "process": "success",
    "original": "original content",
    "clean": "cleaned content",
    "clean_with_response": "content with system response",
    "processed_items": 5,
    "items_detail": ["Task: Fix bug", "Memory: Important moment"],
    "traditional_processing": "tool processing output"
}
```

## Integration Points

### External System Integration

#### `Done` Class (Task Management)
```python
class Done:
    @staticmethod
    async def create_task(action: str, priority: Optional[str], 
                         parent_project: Optional[str], time: Optional[str], 
                         context_notes: Optional[str]) -> None
```

**Priority Handling**: Converts string priorities to enum values with fallback to "Medium"

#### `Memories` Class (Memory Storage)
```python
class Memories:
    @staticmethod
    async def create(moment: str, meaning: str, reason: str) -> None
```

**Storage Integration**: Creates memory objects with timestamps and unique IDs

#### `Ideas` Class (Idea Management)
```python
class Ideas:
    @staticmethod
    async def create(idea: str) -> None
```

### Storage Integration
```python
class storage:
    @staticmethod
    def save_error(err: ErrorItem) -> None
```
- Integrates with application logging system
- Uses dedicated error logger channel

## Technical Constraints and Limitations

### Performance Considerations

**Memory Usage**:
- State maintains rolling windows (100 recent actions)
- Processed content storage should be monitored for large-scale usage
- Natural language pattern matching uses compiled regex for efficiency

**Processing Time**:
- JSON parsing attempts before text processing
- Regex compilation cached where possible
- Async subprocess execution prevents blocking

**Scalability**:
- In-memory state suitable for moderate loads
- Large-scale deployments require external state storage
- Session management designed for concurrent access

### Error Handling and Edge Cases

**Content Parsing**:
- Malformed JSON falls back to text processing
- Invalid regex patterns raise `TodoziError`
- Missing required parameters return appropriate error codes

**External Dependencies**:
- Tool command failures are captured and reported
- External system unavailability handled gracefully
- All external calls include exception handling

**State Consistency**:
- Locking mechanism ensures thread-safe state updates
- UUID generation prevents ID collisions
- Timestamp tracking maintains temporal consistency

## Dependencies and Requirements

### Core Dependencies
- **Python 3.8+**: Async/await syntax and typing support
- **Pydantic**: Data validation and serialization
- **asyncio**: Asynchronous execution framework
- **uuid**: Unique identifier generation
- **datetime**: Temporal data handling
- **re**: Regular expression processing
- **json**: JSON serialization/deserialization
- **subprocess**: External command execution

### Optional Dependencies
- **todozi.lib**: Task management library
- **todozi.storage**: Persistent storage backend
- **logging**: Error logging infrastructure

## Testing and Validation

### Test Suite
```python
async def test_tdz_cnt_basic()
async def test_checklist_extraction()
```

**Test Coverage**:
- Basic content processing functionality
- Checklist extraction from natural language
- Error handling and edge cases
- Integration with external systems

### Demo Execution
```python
if __name__ == "__main__":
    asyncio.run(demo())
```

**Demo Features**:
- End-to-end processing demonstration
- Checklist extraction showcase
- Tool execution examples

## Usage Examples

### Basic Content Processing
```python
# Process content with task extraction
content = """
We need to fix the database connection issue.
<todozi>p:high Investigate connection pool; production issue</todozi>
<memory>Database outage; Learned about connection limits; Prevent future issues</memory>
"""

result = await tdz_cnt(content, "session-123")
print(json.loads(result)["clean_with_response"])
```

### Tool-Based Processing
```python
# Use the content processor tool directly
state = await initialize_tdz_content_processor()
tool = create_tdz_content_processor_tool(state)

result = await tool.execute({
    "content": "We should prioritize the security audit",
    "extract_checklist": True,
    "session_id": "security-review"
})

if result.success:
    print("Processing completed:", result.output)
```

### Custom Tool Integration
```python
class CustomTodoziTool(Tool):
    @property
    def definition(self) -> ToolDefinition:
        return ToolDefinition.new(
            name="custom_processor",
            description="Custom content processor",
            parameters=[create_tool_parameter("data", "string", "Input data", True)],
            category="Custom",
            resource_locks=[]
        )
    
    async def execute(self, kwargs: Dict[str, Any]) -> ToolResult:
        # Custom processing logic
        return ToolResult.success("Custom processing completed")
```

This documentation provides comprehensive coverage of the Todozi Content Processing System's architecture, components, and usage patterns. The system's modular design and extensive error handling make it suitable for production use in conversational AI and content processing applications.