# Todozi Content Extraction System Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Data Models](#data-models)
7. [Configuration](#configuration)
8. [Error Handling](#error-handling)
9. [Security Considerations](#security-considerations)
10. [Performance Analysis](#performance-analysis)
11. [Testing Strategies](#testing-strategies)
12. [Deployment Instructions](#deployment-instructions)
13. [Troubleshooting](#troubleshooting)

## Overview

The Todozi Content Extraction System is a sophisticated JavaScript module designed to extract structured data from unstructured text content. It leverages AI-powered APIs to identify tasks, memories, ideas, errors, and training data from input content, then formats and stores this information in various structured formats.

### Key Features
- **Multi-format Input**: Accept content from inline text or files
- **AI-Powered Extraction**: Utilizes Todozi's API for intelligent content parsing
- **Structured Output**: Supports JSON, CSV, and Markdown formats
- **Auto-Embedding**: Automatically saves extracted content to project files
- **Human-Readable Checklists**: Generates formatted checklists for review
- **History Tracking**: Maintains comprehensive logs of extracted content

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Todozi Content Extraction                    │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   Public    │  │   Core      │  │   Helper    │  │   Models    │ │
│  │  Functions  │  │  Logic      │  │  Functions  │  │             │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
│         │                 │                │               │        │
│         ▼                 ▼                ▼               ▼        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │ extractContent│ │ extractWithEndpoint│ │ makeRequest │ │ Task/Memory │ │
│  │ strategyContent│ │ loadConfig │ │ tdzCnt      │ │ Idea/...    │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │
│         │                 │                │               │        │
│         ▼                 ▼                ▼               ▼        │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                        Todozi API                               │ │
│  └─────────────────────────────────────────────────────────────────┘ │
│                              │                                       │
│                              ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │                    Data Storage & Formatting                    │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │ │
│  │  │   History   │  │   Files     │  │  Checklists │              │ │
│  │  │   Logging   │  │   Saving    │  │  Generation │              │ │
│  │  └─────────────┘  └─────────────┘  └─────────────┘              │ │
│  └─────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

### Design Patterns Used
1. **Factory Pattern**: Used in `ExtractResponse` and extracted item classes
2. **Facade Pattern**: Public API functions provide simplified interfaces
3. **Strategy Pattern**: Different endpoints for different extraction types
4. **Singleton Pattern**: Configuration loading and API key management
5. **Observer Pattern**: History logging for extracted content

## Core Components

### 1. ExtractResponse Classes

#### ExtractResponse
Main container for all extracted content types.

```javascript
class ExtractResponse {
    constructor() {
        this.tasks = [];
        this.memories = [];
        this.ideas = [];
        this.errors = [];
        this.training_data = [];
        this.raw_tags = [];
    }
}
```

#### ExtractedTask
Represents a task extracted from content.

```javascript
class ExtractedTask {
    constructor(action, time, priority, project, status, assignee, tags) {
        this.action = action;
        this.time = time;
        this.priority = priority;
        this.project = project;
        this.status = status;
        this.assignee = assignee || null;
        this.tags = tags || [];
    }
}
```

#### ExtractedMemory
Represents a memory extracted from content.

```javascript
class ExtractedMemory {
    constructor(moment, meaning, reason, importance, term) {
        this.moment = moment;
        this.meaning = meaning;
        this.reason = reason;
        this.importance = importance;
        this.term = term;
    }
}
```

#### ExtractedIdea
Represents an idea extracted from content.

```javascript
class ExtractedIdea {
    constructor(idea, share, importance) {
        this.idea = idea;
        this.share = share;
        this.importance = importance;
    }
}
```

#### ExtractedError
Represents an error extracted from content.

```javascript
class ExtractedError {
    constructor(title, description, severity, category) {
        this.title = title;
        this.description = description;
        this.severity = severity;
        this.category = category;
    }
}
```

#### ExtractedTrainingData
Represents training data extracted from content.

```javascript
class ExtractedTrainingData {
    constructor(prompt, completion, data_type) {
        this.prompt = prompt;
        this.completion = completion;
        this.data_type = data_type;
    }
}
```

### 2. Helper Functions

#### makeRequest(url, payload, apiKey)
Makes HTTP requests to Todozi API.

**Parameters:**
- `url` (string): API endpoint URL
- `payload` (object): Request body data
- `apiKey` (string): Authentication API key

**Returns:** Promise resolving to API response JSON

**Throws:** TodoziError on request failure

#### loadConfig()
Loads configuration from `.todozi/tdz.hlx` file.

**Returns:** Promise resolving to config object with `user_id` and `fingerprint`

**Throws:** TodoziError on configuration loading failure

### 3. Main Logic

#### extractWithEndpoint(content, filePath, outputFormat, human, endpoint)
Core extraction function that processes content through Todozi API.

**Parameters:**
- `content` (string): Inline content to extract from
- `filePath` (string): Path to file containing content
- `outputFormat` (string): Output format ('json', 'csv', 'md', 'markdown')
- `human` (boolean): Whether to generate human-readable checklist
- `endpoint` (string): API endpoint to use ('plan', 'strategic')

**Returns:** Promise resolving to formatted output string

**Throws:** TodoziError on various failures

## API Reference

### Public Functions

#### extractContent(content, filePath, outputFormat, human)
Extracts content using the "plan" endpoint.

**Parameters:**
- `content` (string): Inline content to extract from
- `filePath` (string): Path to file containing content
- `outputFormat` (string): Output format ('json', 'csv', 'md', 'markdown')
- `human` (boolean): Whether to generate human-readable checklist

**Returns:** Promise resolving to formatted output string

**Example:**
```javascript
const result = await extractContent("Meeting notes with action items", null, "json", true);
```

#### strategyContent(content, filePath, outputFormat, human)
Extracts content using the "strategic" endpoint.

**Parameters:**
- `content` (string): Inline content to extract from
- `filePath` (string): Path to file containing content
- `outputFormat` (string): Output format ('json', 'csv', 'md', 'markdown')
- `human` (boolean): Whether to generate human-readable checklist

**Returns:** Promise resolving to formatted output string

**Example:**
```javascript
const result = await strategyContent(null, "/path/to/strategy.doc", "md", true);
```

### Formatting Functions

#### formatAsCSV(response)
Formats ExtractResponse as CSV.

**Parameters:**
- `response` (ExtractResponse): Response to format

**Returns:** CSV formatted string

#### formatAsMarkdown(response)
Formats ExtractResponse as Markdown.

**Parameters:**
- `response` (ExtractResponse): Response to format

**Returns:** Markdown formatted string

#### formatAsHumanChecklist(response)
Formats ExtractResponse as human-readable checklist.

**Parameters:**
- `response` (ExtractResponse): Response to format

**Returns:** Markdown checklist formatted string

### Utility Functions

#### logToHistory(task)
Logs task to history mega file.

**Parameters:**
- `task` (Task): Task to log

**Throws:** TodoziError on logging failure

## Usage Examples

### Basic Content Extraction

```javascript
import { extractContent } from './todozi-extract.js';

// Extract from inline content
const content = `
Project meeting notes:
- Review Q4 budget allocation (High priority, due next week)
- Schedule team building event (Medium priority, by month end)
- Update documentation for new API endpoints (Low priority, ongoing)
`;

const result = await extractContent(content, null, 'json', true);
console.log(result);
```

### File-Based Extraction

```javascript
import { extractContent } from './todozi-extract.js';

// Extract from file with Markdown output
const result = await extractContent(null, '/path/to/meeting-notes.txt', 'md', false);
console.log(result);
```

### Strategic Content Analysis

```javascript
import { strategyContent } from './todozi-extract.js';

const strategicContent = `
Company Q4 Strategy:
- Expand into European markets (High importance, 18-month timeline)
- Launch new product line (Critical importance, 12-month timeline)
- Improve customer retention (High importance, ongoing)
- Reduce operational costs by 15% (Medium importance, 6-month timeline)
`;

const result = await strategyContent(strategicContent, null, 'json', true);
console.log(result);
```

### CSV Output Generation

```javascript
import { extractContent } from './todozi-extract.js';

const content = "Project kickoff: Design UI mockups (High priority, due Friday) - John";

const csvResult = await extractContent(content, null, 'csv', false);
console.log(csvResult);
```

## Data Models

### Task Model
```javascript
class Task {
    constructor(
        userId,
        action,
        time,
        priority,
        projectId,
        status,
        assignee,
        tags,
        dependencies,
        context,
        progress
    ) {
        // Implementation details in models.js
    }
}
```

### Priority Enum
```javascript
const Priority = {
    Low: 'Low',
    Medium: 'Medium',
    High: 'High',
    Critical: 'Critical'
};
```

### Status Enum
```javascript
const Status = {
    Todo: 'Todo',
    InProgress: 'InProgress',
    Done: 'Done',
    Blocked: 'Blocked'
};
```

## Configuration

### Configuration File Structure
The system expects a configuration file at `~/.todozi/tdz.hlx`:

```json
{
  "registration": {
    "user_id": "user-123",
    "fingerprint": "device-fingerprint-456"
  }
}
```

### Environment Variables
- `HOME`: Required for locating configuration and history files
- `TODOZI_API_KEY`: Optional API key override (typically managed by `getTdzApiKey`)

## Error Handling

### Custom Error Types
```javascript
class TodoziError extends Error {
    constructor(message) {
        super(message);
        this.name = 'TodoziError';
    }
}
```

### Common Error Scenarios
1. **Missing Input**: Either content or file must be provided
2. **API Request Failures**: Network issues or API errors
3. **Configuration Issues**: Missing or malformed config files
4. **File System Errors**: Permission or I/O issues
5. **Authentication Failures**: Invalid API keys

### Error Recovery Strategies
- Graceful degradation to default values
- Detailed error logging for debugging
- User-friendly error messages
- Retry mechanisms for transient failures

## Security Considerations

### Data Protection
1. **API Key Management**: Secure storage and transmission
2. **Content Privacy**: Sensitive data handling through Todozi API
3. **File System Permissions**: Secure configuration and history file access
4. **Network Security**: HTTPS communication with Todozi API

### Authentication
- Bearer token authentication with Todozi API
- User identification through config file
- Device fingerprinting for session tracking

### Input Validation
- Content sanitization before API transmission
- File path validation to prevent directory traversal
- Format validation for configuration files

## Performance Analysis

### Time Complexity
- **API Request**: O(1) network latency + O(n) content processing
- **Content Parsing**: O(n) where n is the number of extracted items
- **File Operations**: O(1) for single file operations
- **Formatting**: O(n) where n is the number of extracted items

### Memory Usage
- **Input Content**: O(n) where n is content size
- **Response Storage**: O(m) where m is number of extracted items
- **Configuration**: O(1) constant memory overhead

### Optimization Strategies
1. **Batch Processing**: Process multiple files in parallel
2. **Caching**: Cache configuration and API keys
3. **Streaming**: Stream large content instead of loading entirely
4. **Connection Pooling**: Reuse HTTP connections for multiple requests

## Testing Strategies

### Unit Tests
```javascript
// Test extractWithEndpoint with mock API response
describe('extractWithEndpoint', () => {
    it('should extract tasks from content', async () => {
        // Mock implementation
    });
    
    it('should handle API errors gracefully', async () => {
        // Mock implementation
    });
});
```

### Integration Tests
```javascript
// Test with actual Todozi API
describe('Todozi API Integration', () => {
    it('should successfully extract content', async () => {
        // Integration test implementation
    });
});
```

### Mock Data
```javascript
const mockApiResponse = {
    tasks: [
        {
            action: "Review budget",
            time: "next week",
            priority: "High",
            project: "Q4 Planning",
            status: "Todo"
        }
    ],
    memories: [],
    ideas: []
};
```

## Deployment Instructions

### Prerequisites
1. Node.js v14+ installed
2. Todozi API access and valid API key
3. Write permissions to user home directory

### Installation Steps
```bash
# Clone repository
git clone https://github.com/todozi/todozi-extract.git
cd todozi-extract

# Install dependencies
npm install

# Create configuration directory
mkdir -p ~/.todozi

# Create basic configuration
echo '{"registration":{"user_id":"your-user-id"}}' > ~/.todozi/tdz.hlx
```

### Environment Setup
```bash
# Set API key (if not using getTdzApiKey mechanism)
export TODOZI_API_KEY="your-api-key"
```

### Usage as Module
```javascript
import { extractContent, strategyContent } from './todozi-extract.js';

// Use in your application
const result = await extractContent("Your content here", null, "json", true);
```

## Troubleshooting

### Common Issues

#### 1. API Key Not Found
**Symptom:** "API request failed: Unauthorized"
**Solution:** 
- Verify API key is set via `getTdzApiKey`
- Check environment variables
- Confirm Todozi account status

#### 2. Configuration File Missing
**Symptom:** "Could not get HOME environment variable"
**Solution:**
- Ensure `HOME` environment variable is set
- Create `~/.todozi/tdz.hlx` with proper JSON structure

#### 3. File Permission Errors
**Symptom:** "Failed to create history directory"
**Solution:**
- Check write permissions on home directory
- Verify `~/.todozi` directory permissions

#### 4. Network Connectivity Issues
**Symptom:** "API request failed: Network error"
**Solution:**
- Check internet connectivity
- Verify Todozi API endpoint availability
- Check firewall/proxy settings

### Debugging Steps

1. **Enable Verbose Logging**
```javascript
// Add before calling extraction functions
process.env.DEBUG = 'todozi:*';
```

2. **Check Raw API Response**
```javascript
// The system already logs raw responses
console.log("🔍 Raw API Response:");
console.log(JSON.stringify(apiResponse, null, 2));
```

3. **Validate Input Content**
```javascript
// Ensure content is properly formatted
if (!content && !filePath) {
    throw new TodoziError("Either content or file must be provided");
}
```

### Performance Monitoring

#### Response Time Tracking
```javascript
const startTime = Date.now();
const result = await extractContent(content, null, 'json', false);
const endTime = Date.now();
console.log(`Extraction took ${endTime - startTime}ms`);
```

#### Memory Usage Monitoring
```javascript
const used = process.memoryUsage();
console.log(`Memory usage: ${Math.round(used.heapUsed / 1024 / 1024 * 100) / 100} MB`);
```

This comprehensive documentation provides a complete overview of the Todozi Content Extraction System, covering all aspects from architecture and implementation details to deployment and troubleshooting. The system is designed for robustness, scalability, and ease of use while maintaining security and performance standards.