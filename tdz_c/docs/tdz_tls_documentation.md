# Todozi Content Processor - Comprehensive Documentation

## 1. Overview

The Todozi Content Processor is a sophisticated C library designed for processing conversational content, extracting actionable items, managing checklists, and handling tool calls. It provides natural language processing capabilities with JSON integration for intelligent content management systems.

## 2. Architecture

### 2.1 System Architecture Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Input Content │ -> │ Content Parser   │ -> │ Data Extractor  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Tool Processor │ -> │ State Manager    │ -> │ Response Gen    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Output/Store  │    │  Session Mgmt    │    │ Final Response  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### 2.2 Component Relationships

```
tdz_cnt() (Main Entry Point)
    ├── parse_raw_content()
    │   ├── parse_json_content()
    │   └── parse_text_content()
    ├── extract_todozi_data()
    │   ├── regex pattern matching
    │   └── natural language extraction
    ├── process_tool_calls()
    │   ├── task creation
    │   ├── search operations
    │   ├── memory creation
    │   └── idea generation
    ├── clean_content()
    └── generate_response()
        └── state-aware formatting
```

## 3. Data Structures

### 3.1 Primary Structures

#### ChecklistItem
```c
struct ChecklistItem {
    char* id;           // UUID identifier
    char* content;      // Task description
    char* priority;     // Priority level
    int completed;      // Completion status
    time_t created_at;  // Creation timestamp
    char* source;       // Source of the item
};
```

#### ProcessedContent
```c
struct ProcessedContent {
    char* id;                           // Unique identifier
    char* session_id;                   // Session context
    char* raw_content;                  // Original content
    char* cleaned_content;              // Processed content
    time_t timestamp;                   // Processing time
    ExtractedAction* extracted_items;   // Extracted actions
    size_t extracted_items_count;       // Action count
    ChecklistItem* checklist_items;     // Generated checklist
    size_t checklist_items_count;       // Checklist item count
    ProcessedAction* tool_calls;        // Processed tool calls
    size_t tool_calls_count;            // Tool call count
    ProcessingStats processing_stats;   // Performance metrics
};
```

#### TodoziProcessorState
```c
struct TodoziProcessorState {
    HashMap* active_sessions;           // Session management
    ProcessedAction* recent_actions;    // Action history
    size_t recent_actions_count;        // Action count
    size_t recent_actions_capacity;     // Storage capacity
    ChecklistItem* checklist_items;     // Global checklist
    size_t checklist_items_count;       // Item count
    size_t checklist_items_capacity;    // Storage capacity
    ProcessedContent* processed_contents; // Content history
    size_t processed_contents_count;    // Content count
    size_t processed_contents_capacity; // Storage capacity
};
```

### 3.2 Support Structures

#### HashMap Implementation
Simple hash map for session management with chaining collision resolution.

```c
typedef struct HashMapEntry {
    char* key;
    void* value;
    struct HashMapEntry* next;
} HashMapEntry;

typedef struct {
    HashMapEntry** buckets;
    size_t size;
} HashMap;
```

## 4. Core Functions & API Reference

### 4.1 Initialization Functions

#### `todozi_processor_state_new()`
```c
TodoziProcessorState* todozi_processor_state_new(void)
```
**Purpose**: Create and initialize a new processor state instance

**Parameters**: None

**Returns**: 
- `TodoziProcessorState*` - Pointer to newly allocated state
- `NULL` - If memory allocation fails

**Memory Management**: Caller must free with `todozi_processor_state_free()`

**Usage Example**:
```c
TodoziProcessorState* state = todozi_processor_state_new();
if (!state) {
    // Handle initialization error
}
```

#### `tdz_content_processor_tool_new()`
```c
TdzContentProcessorTool* tdz_content_processor_tool_new(TodoziProcessorState* state)
```
**Purpose**: Create a content processing tool with associated state

**Parameters**:
- `state` - Pre-initialized processor state

**Returns**: 
- `TdzContentProcessorTool*` - New tool instance
- `NULL` - If allocation fails or state is invalid

**Usage Example**:
```c
TdzContentProcessorTool* tool = tdz_content_processor_tool_new(state);
```

### 4.2 Content Processing Functions

#### `tdz_cnt()` - Main Processing Function
```c
char* tdz_cnt(const char* content, const char* session_id)
```
**Purpose**: Primary content processing entry point

**Parameters**:
- `content` - Input content to process (text or JSON)
- `session_id` - Session identifier for context

**Returns**: Processed response string (caller must free)

**Processing Flow**:
1. Parse content (JSON or text)
2. Extract todozi-specific data
3. Process tool calls
4. Clean and format content
5. Generate comprehensive response

**Usage Example**:
```c
const char* input = "{"content": "We need to fix the bug", "tool_calls": []}";
char* result = tdz_cnt(input, "session_123");
printf("Processed: %s\n", result);
free(result);
```

#### `parse_raw_content()`
```c
ParsedContent* parse_raw_content(const char* content)
```
**Purpose**: Parse input content, automatically detecting JSON or text format

**Parameters**: `content` - Input string

**Returns**: `ParsedContent*` structure containing parsed data

**Algorithm**:
- Attempt JSON parsing first
- Fall back to text parsing if JSON invalid
- Extract content and tool calls from JSON structure

#### `extract_todozi_data()`
```c
ExtractionResult* extract_todozi_data(TdzContentProcessorTool* tool, ParsedContent* parsed)
```
**Purpose**: Extract todozi-specific patterns and tags from content

**Parameters**:
- `tool` - Processing tool with pattern database
- `parsed` - Pre-parsed content

**Extraction Patterns**:
- XML-like tags: `<todozi>`, `<memory>`, `<idea>`
- Natural language patterns: "we should", "I need to", "don't forget"
- Tool calls from JSON structure

### 4.3 Tool Call Processing

#### `process_tool_calls()`
```c
ProcessingResult* process_tool_calls(TdzContentProcessorTool* tool, json_object* tool_calls)
```
**Purpose**: Process JSON tool call arrays

**Supported Tool Types**:
- `create_task` / `add_task` - Task creation
- `search` / `list` - Information retrieval
- `update` / `complete` - Status updates
- `memory` - Memory creation
- `idea` - Idea generation

**Tool Call JSON Structure**:
```json
{
    "function": {
        "name": "create_task",
        "parameters": {...}
    }
}
```

### 4.4 Memory Management Functions

#### Resource Cleanup Functions
```c
void todozi_processor_state_free(TodoziProcessorState* state);
void tdz_content_processor_tool_free(TdzContentProcessorTool* tool);
void parsed_content_free(ParsedContent* parsed);
void extraction_result_free(ExtractionResult* result);
void processing_result_free(ProcessingResult* result);
void checklist_items_free(ChecklistItem* items, size_t count);
```

## 5. Design Patterns Used

### 5.1 Factory Pattern
- `todozi_processor_state_new()` - State object factory
- `tdz_content_processor_tool_new()` - Tool object factory

### 5.2 Strategy Pattern
- Content parsing strategy (JSON vs Text)
- Tool call processing strategies

### 5.3 Observer Pattern (Implicit)
- Session state tracking
- Recent actions history

### 5.4 Composite Pattern
- Nested content structures
- Hierarchical data extraction

## 6. Performance Analysis

### 6.1 Time Complexity
| Operation | Complexity | Notes |
|-----------|------------|-------|
| Content Parsing | O(n) | Linear to content length |
| Regex Extraction | O(n*m) | n=content length, m=pattern count |
| Hash Map Operations | O(1) avg, O(n) worst | With good hash distribution |
| Tool Call Processing | O(k) | k=number of tool calls |

### 6.2 Space Complexity
| Component | Memory Usage | Notes |
|-----------|--------------|-------|
| Processor State | O(s + a + c) | s=sessions, a=actions, c=contents |
| Content Processing | O(n) | n=content length |
| Extraction Results | O(t + p) | t=tags, p=patterns |

### 6.3 Optimization Strategies
- Dynamic array resizing (geometric progression)
- UUID caching for frequent item creation
- Pattern pre-compilation
- Memory pooling for frequent allocations

## 7. Security Considerations

### 7.1 Input Validation
- NULL pointer checks throughout
- Buffer length validation
- JSON structure validation

### 7.2 Memory Safety
- Comprehensive cleanup functions
- Deep copying for state persistence
- Bounds checking on dynamic arrays

### 7.3 Command Execution Safety
- `execute_binary_command()` uses fork/exec with argument validation
- Pipe-based output capture prevents shell injection
- Process isolation with proper cleanup

### 7.4 Session Security
- UUID-based session identification
- Session activity tracking
- Automatic session cleanup in state destruction

## 8. Error Handling

### 8.1 Error Types
```c
typedef enum {
    TODOZI_SUCCESS = 0,
    TODOZI_ERROR_VALIDATION,    // Input validation failed
    TODOZI_ERROR_STORAGE,       // Memory allocation failed
    TODOZI_ERROR_PARSE          // Content parsing failed
} TodoziError;
```

### 8.2 Error Recovery Strategies
- Graceful degradation on parsing failures
- Fallback to text processing when JSON invalid
- NULL-safe memory operations
- Comprehensive cleanup on error conditions

## 9. Testing Strategies

### 9.1 Unit Test Categories

#### Content Parsing Tests
```c
// Test JSON parsing
void test_json_parsing() {
    const char* json_content = "{\"content\": \"test\", \"tool_calls\": []}";
    ParsedContent* parsed = parse_raw_content(json_content);
    assert(parsed != NULL);
    assert(parsed->json_content != NULL);
    parsed_content_free(parsed);
}

// Test text parsing fallback
void test_text_parsing() {
    const char* text_content = "Simple text content";
    ParsedContent* parsed = parse_raw_content(text_content);
    assert(parsed != NULL);
    assert(parsed->text_content != NULL);
    parsed_content_free(parsed);
}
```

#### Extraction Tests
```c
void test_tag_extraction() {
    const char* content = "Content with <todozi>tag</todozi> inside";
    ParsedContent* parsed = parse_text_content(content);
    ExtractionResult* result = extract_todozi_data(tool, parsed);
    assert(result->extracted_tags_count == 1);
    extraction_result_free(result);
    parsed_content_free(parsed);
}
```

### 9.2 Integration Tests
- End-to-end processing pipeline
- State persistence across operations
- Session management continuity

### 9.3 Performance Tests
- Large content processing benchmarks
- Memory usage profiling
- Concurrent access patterns

## 10. Deployment Instructions

### 10.1 Build Dependencies
```bash
# Required libraries
sudo apt-get install libjson-c-dev libuuid-dev

# Compilation flags
gcc -o todozi_processor todozi_processor.c -ljson-c -luuid
```

### 10.2 Integration Steps

#### Basic Integration
```c
#include "todozi_processor.h"

// Initialize processor
TodoziProcessorState* state = initialize_tdz_content_processor();

// Process content
char* result = tdz_cnt(user_content, session_id);

// Use result
process_result(result);

// Cleanup
free(result);
todozi_processor_state_free(state);
```

#### Advanced Configuration
```c
// Custom pattern initialization
TdzContentProcessorTool* tool = tdz_content_processor_tool_new(state);
// Add custom patterns if needed
```

### 10.3 Platform Considerations
- POSIX-compliant systems required
- UUID library availability
- JSON-C library version compatibility
- Fork/exec functionality for command execution

## 11. Usage Examples

### 11.1 Basic Content Processing
```c
#include "todozi_processor.h"

int main() {
    const char* content = "We need to fix the database connection issue.";
    char* processed = tdz_cnt(content, "tech_support_session");
    
    printf("Input: %s\n", content);
    printf("Processed: %s\n", processed);
    
    free(processed);
    return 0;
}
```

### 11.2 JSON Tool Call Processing
```c
const char* json_content = 
    "{\"content\": \"Create a task for documentation\", "
    "\"tool_calls\": [{\"function\": {\"name\": \"create_task\"}}]}";

char* result = tdz_cnt(json_content, "doc_session");
```

### 11.3 Session Management
```c
TodoziProcessorState* state = todozi_processor_state_new();

// Process multiple messages in same session
tdz_cnt("First message", "session_1");
tdz_cnt("Follow up message", "session_1");
tdz_cnt("Different topic", "session_2");

// State maintains session context and history
```

## 12. Troubleshooting Guide

### 12.1 Common Issues

#### Memory Leaks
**Symptoms**: Increasing memory usage over time
**Solution**: Ensure proper cleanup sequence:
```c
// Correct cleanup order
processing_result_free(processing);
extraction_result_free(extraction);
tdz_content_processor_tool_free(tool);
todozi_processor_state_free(state);
```

#### JSON Parsing Failures
**Symptoms**: Content processed as plain text when JSON expected
**Debugging**: 
```c
// Check JSON validity before processing
json_object* test_json = json_tokener_parse(content);
if (!test_json) {
    printf("Invalid JSON, will use text processing\n");
}
```

#### Pattern Extraction Issues
**Symptoms**: Expected patterns not being extracted
**Debugging**: Verify pattern initialization and content formatting

### 12.2 Debugging Techniques

#### Runtime Debugging
```c
// Enable debug output
#define TODOZI_DEBUG 1

#ifdef TODOZI_DEBUG
    printf("Processing content: %s\n", content);
    printf("Extracted %zu tags\n", extraction->extracted_tags_count);
#endif
```

#### Memory Debugging
Use tools like Valgrind:
```bash
valgrind --leak-check=full ./todozi_processor
```

### 12.3 Performance Optimization

#### For High-Volume Processing
- Pre-initialize processor state
- Reuse sessions for related content
- Implement custom memory pools
- Batch process related content

## 13. API Reference Summary

### Core Functions
| Function | Purpose | Memory Responsibility |
|----------|---------|----------------------|
| `tdz_cnt()` | Main processing entry | Caller frees result |
| `initialize_tdz_content_processor()` | State initialization | Caller frees state |
| `parse_raw_content()` | Content parsing | Caller frees result |

### Support Functions
| Function Category | Examples | Purpose |
|-------------------|----------|---------|
| Memory Management | `*_free()` functions | Resource cleanup |
| Extraction | `extract_*()` functions | Data pattern extraction |
| Processing | `process_*()` functions | Content transformation |

## 14. Future Enhancements

### Planned Features
- Thread-safe operations for concurrent access
- Plugin architecture for custom processors
- Persistent storage backends
- Advanced NLP integration
- Real-time streaming processing

### Compatibility Roadmap
- Additional JSON library support
- Windows compatibility layer
- WebAssembly compilation target
- Mobile platform adaptations

This documentation provides comprehensive coverage of the Todozi Content Processor system. The implementation demonstrates robust content processing capabilities with extensible architecture and thorough error handling.