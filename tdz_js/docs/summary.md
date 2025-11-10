# Summary Manager Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Classes](#classes)
4. [Functions](#functions)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Summary Manager is a JavaScript module designed to manage, store, and manipulate text summaries with associated metadata such as tags, priority levels, and context. It provides a comprehensive set of tools for creating, retrieving, updating, and deleting summaries, along with advanced querying capabilities.

### Key Features
- Create and manage summaries with unique identifiers
- Assign tags, priorities, and context to summaries
- Search and filter summaries by various criteria
- Generate statistics and analytics
- Parse formatted summary strings

## Architecture

### System Architecture Diagram
```
┌─────────────────────────────────────────────────────────────────┐
│                        Summary Manager                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   Summaries     │  │  Summary Tags   │  │   Statistics    │  │
│  │     Map         │  │     Map         │  │   Functions     │  │
│  │                 │  │                 │  │                 │  │
│  │  id → summary   │  │  id → [tags]    │  │  - Tag Stats    │  │
│  │                 │  │                 │  │  - Summary Stats│  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                        Public Methods                           │
│  - CRUD Operations (create, get, update, delete)                │
│  - Query Methods (search, filter by priority/tag)               │
│  - Statistical Methods (get statistics)                         │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                      Helper Classes                             │
├─────────────────────────────────────────────────────────────────┤
│  SummaryUpdate - Builder pattern for updates                    │
│  SummaryStatistics - Statistical data container                 │
│  SummaryPriority - Priority level constants                     │
│  parseSummaryFormat - String parser                             │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow
1. **Input**: Summaries are created through direct object creation or string parsing
2. **Storage**: Summaries are stored in Maps with UUID keys
3. **Processing**: Queries and operations are performed on the stored data
4. **Output**: Results are returned as objects or statistical data

## Classes

### SummaryManager

The main class for managing summaries and their associated metadata.

#### Constructor
```javascript
new SummaryManager()
```

Initializes a new SummaryManager instance with empty Maps for storing summaries and tags.

#### Properties
- `summaries`: Map - Stores summary objects with UUID keys
- `summary_tags`: Map - Stores tag arrays associated with summary IDs

#### Methods

##### createSummary(summary)
Creates a new summary with a unique ID and timestamps.

**Parameters:**
- `summary` (Object): Summary object to create
  - `content` (string): Summary content
  - `priority` (string): Priority level (from SummaryPriority)
  - `context` (string, optional): Context information
  - `tags` (Array<string>, optional): Tags associated with the summary

**Returns:** Promise<string> - UUID of the created summary

**Example:**
```javascript
const summaryId = await manager.createSummary({
  content: "Project completed successfully",
  priority: SummaryPriority.High,
  context: "Final project delivery",
  tags: ["project", "completion", "success"]
});
```

##### getSummary(summaryId)
Retrieves a summary by its ID.

**Parameters:**
- `summaryId` (string): UUID of the summary to retrieve

**Returns:** Object | undefined - Summary object or undefined if not found

##### getAllSummaries()
Retrieves all summaries.

**Returns:** Array<Object> - Array of all summary objects

##### updateSummary(summaryId, updates)
Updates an existing summary with new values.

**Parameters:**
- `summaryId` (string): UUID of the summary to update
- `updates` (Object): Object containing fields to update
  - `content` (string, optional): New content
  - `context` (string, optional): New context
  - `priority` (string, optional): New priority
  - `tags` (Array<string>, optional): New tags

**Returns:** Promise<void>

**Throws:** Error - If summary with given ID is not found

##### deleteSummary(summaryId)
Deletes a summary by its ID.

**Parameters:**
- `summaryId` (string): UUID of the summary to delete

**Returns:** Promise<void>

**Throws:** Error - If summary with given ID is not found

##### searchSummaries(query)
Searches summaries by content, tags, or context.

**Parameters:**
- `query` (string): Search query string

**Returns:** Array<Object> - Array of matching summary objects

##### getSummariesByPriority(priority)
Retrieves summaries with a specific priority level.

**Parameters:**
- `priority` (string): Priority level to filter by

**Returns:** Array<Object> - Array of summary objects with matching priority

##### getSummariesByTag(tag)
Retrieves summaries that contain a specific tag.

**Parameters:**
- `tag` (string): Tag to filter by

**Returns:** Array<Object> - Array of summary objects with matching tag

##### getRecentSummaries(limit)
Retrieves the most recently created summaries.

**Parameters:**
- `limit` (number): Maximum number of summaries to return

**Returns:** Array<Object> - Array of recent summary objects

##### getHighPrioritySummaries()
Retrieves all summaries with high or critical priority.

**Returns:** Array<Object> - Array of high-priority summary objects

##### getAllTags()
Retrieves all unique tags across all summaries.

**Returns:** Array<string> - Array of unique tags

##### getTagStatistics()
Calculates statistics for tags (count of each tag).

**Returns:** Object - Object mapping tags to their occurrence counts

##### getSummaryStatistics()
Calculates overall summary statistics.

**Returns:** SummaryStatistics - Statistical data about summaries

### SummaryUpdate

Builder pattern class for creating summary updates.

#### Constructor
```javascript
new SummaryUpdate()
```

Initializes a new SummaryUpdate instance with all fields set to undefined.

#### Static Methods

##### new()
Creates a new SummaryUpdate instance.

**Returns:** SummaryUpdate - New instance

#### Methods

##### setContent(content)
Sets the content field.

**Parameters:**
- `content` (string): Content to set

**Returns:** SummaryUpdate - This instance for chaining

##### setContext(context)
Sets the context field.

**Parameters:**
- `context` (string): Context to set

**Returns:** SummaryUpdate - This instance for chaining

##### setPriority(priority)
Sets the priority field.

**Parameters:**
- `priority` (string): Priority to set

**Returns:** SummaryUpdate - This instance for chaining

##### setTags(tags)
Sets the tags field.

**Parameters:**
- `tags` (Array<string>): Tags to set

**Returns:** SummaryUpdate - This instance for chaining

### SummaryStatistics

Container class for summary statistical data.

#### Constructor
```javascript
new SummaryStatistics(totalSummaries, highPrioritySummaries, uniqueTags)
```

**Parameters:**
- `totalSummaries` (number): Total number of summaries
- `highPrioritySummaries` (number): Number of high-priority summaries
- `uniqueTags` (number): Number of unique tags

#### Properties
- `total_summaries` (number): Total number of summaries
- `high_priority_summaries` (number): Number of high-priority summaries
- `unique_tags` (number): Number of unique tags

#### Methods

##### highPriorityPercentage()
Calculates the percentage of high-priority summaries.

**Returns:** number - Percentage of high-priority summaries (0-100)

### SummaryPriority

Enumeration of priority levels.

#### Values
- `Low`: "low"
- `Medium`: "medium"
- `High`: "high"
- `Critical`: "critical"

## Functions

### parseSummaryFormat(summaryText)
Parses a formatted summary string into a summary object.

**Parameters:**
- `summaryText` (string): Formatted summary string in the format:
  `<summary>content; priority; context; tag1,tag2,tag3</summary>`

**Returns:** Object - Parsed summary object with:
- `id` (string): UUID
- `content` (string): Summary content
- `context` (string, optional): Context information
- `priority` (string): Priority level
- `tags` (Array<string>): Tags
- `created_at` (Date): Creation timestamp
- `updated_at` (Date): Update timestamp

**Throws:** Error - If format is invalid or missing required elements

## Usage Examples

### Basic Usage
```javascript
const { SummaryManager, SummaryPriority } = require('./summary-manager');

// Create manager instance
const manager = new SummaryManager();

// Create a new summary
const summaryId = await manager.createSummary({
  content: "Meeting with stakeholders completed",
  priority: SummaryPriority.High,
  context: "Quarterly review",
  tags: ["meeting", "stakeholders", "review"]
});

// Retrieve the summary
const summary = manager.getSummary(summaryId);
console.log(summary);

// Update the summary
await manager.updateSummary(summaryId, {
  content: "Meeting with stakeholders completed successfully",
  tags: ["meeting", "stakeholders", "review", "success"]
});

// Search for summaries
const results = manager.searchSummaries("stakeholders");
console.log(results);
```

### Using SummaryUpdate Builder
```javascript
const { SummaryUpdate, SummaryPriority } = require('./summary-manager');

const update = SummaryUpdate.new()
  .setContent("Updated project status")
  .setContext("Weekly update")
  .setPriority(SummaryPriority.Medium)
  .setTags(["project", "status", "weekly"]);

await manager.updateSummary(summaryId, update);
```

### Parsing Formatted Strings
```javascript
const { parseSummaryFormat } = require('./summary-manager');

const summaryText = "<summary>Project completed successfully; high; Final project delivery; project,completion,success</summary>";
const summary = parseSummaryFormat(summaryText);

const summaryId = await manager.createSummary(summary);
```

### Getting Statistics
```javascript
// Get all tags
const allTags = manager.getAllTags();
console.log("All tags:", allTags);

// Get tag statistics
const tagStats = manager.getTagStatistics();
console.log("Tag statistics:", tagStats);

// Get summary statistics
const summaryStats = manager.getSummaryStatistics();
console.log(`High priority percentage: ${summaryStats.highPriorityPercentage()}%`);
```

### Filtering Summaries
```javascript
// Get summaries by priority
const highPrioritySummaries = manager.getSummariesByPriority(SummaryPriority.High);

// Get summaries by tag
const projectSummaries = manager.getSummariesByTag("project");

// Get recent summaries
const recentSummaries = manager.getRecentSummaries(5);

// Get high priority summaries
const urgentSummaries = manager.getHighPrioritySummaries();
```

## Design Patterns

### Builder Pattern
The `SummaryUpdate` class implements the Builder pattern to provide a fluent interface for constructing summary update objects.

**Benefits:**
- Improves code readability
- Allows for optional field setting
- Provides method chaining for better developer experience

### Repository Pattern
The `SummaryManager` class acts as a repository for summary objects, encapsulating storage and retrieval logic.

**Benefits:**
- Separates data access logic from business logic
- Provides a consistent interface for data operations
- Enables easier testing and maintenance

### Factory Pattern
The `SummaryUpdate.new()` static method serves as a factory for creating new instances.

**Benefits:**
- Provides a clear entry point for object creation
- Allows for potential future enhancements to the creation process

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| createSummary | O(1) | Map insertion |
| getSummary | O(1) | Map lookup |
| getAllSummaries | O(n) | Array conversion |
| updateSummary | O(1) | Map lookup + update |
| deleteSummary | O(1) | Map deletion |
| searchSummaries | O(n*m) | n=summaries, m=query length |
| getSummariesByPriority | O(n) | Linear filtering |
| getSummariesByTag | O(n*t) | n=summaries, t=avg tags per summary |
| getRecentSummaries | O(n log n) | Sorting operation |
| getHighPrioritySummaries | O(n) | Linear filtering |
| getAllTags | O(n*t) | n=summaries, t=avg tags per summary |
| getTagStatistics | O(n*t) | n=summaries, t=avg tags per summary |
| getSummaryStatistics | O(n) | Dependent on getHighPrioritySummaries |

### Space Complexity
- **Storage**: O(n*t) where n is the number of summaries and t is the average number of tags per summary
- **Search Operations**: O(1) additional space
- **Statistics Operations**: O(t) additional space for tag storage

### Performance Optimization Recommendations
1. **Indexing**: For large datasets, consider implementing indexes for frequently queried fields like priority and tags
2. **Caching**: Cache frequently accessed statistics to avoid recomputation
3. **Pagination**: Implement pagination for large result sets in getAllSummaries and search operations
4. **Batch Operations**: Add batch create/update/delete methods for bulk operations

## Security Considerations

### Input Validation
- The system currently lacks comprehensive input validation
- Implement validation for:
  - Content length limits
  - Tag count limits
  - Priority value validation
  - Context length limits

### Data Sanitization
- Consider implementing HTML/content sanitization to prevent XSS attacks
- Validate and sanitize tags to prevent injection attacks

### Access Control
- The current implementation lacks access control mechanisms
- For production use, implement:
  - User authentication
  - Role-based access control
  - Summary ownership tracking

### Error Handling
- Error messages should not expose internal implementation details
- Implement proper error logging without sensitive information disclosure

## Testing Strategies

### Unit Tests
The module includes built-in test cases covering:
- SummaryManager creation
- SummaryUpdate builder pattern
- SummaryStatistics calculations
- parseSummaryFormat functionality

### Test Coverage Areas
1. **CRUD Operations**: Test create, read, update, delete functionality
2. **Query Methods**: Test search, filter, and retrieval methods
3. **Edge Cases**: Test with empty data, invalid inputs, boundary conditions
4. **Error Conditions**: Test error handling and exception cases
5. **Statistical Methods**: Test statistical calculations and edge cases

### Testing Framework
The module uses Node.js built-in assert module for testing. For more comprehensive testing, consider:

```javascript
// Example expanded test
function testSummaryManagerCRUD() {
  const manager = new SummaryManager();
  
  // Test creation
  const summaryId = await manager.createSummary({
    content: "Test summary",
    priority: SummaryPriority.Medium
  });
  
  // Test retrieval
  const summary = manager.getSummary(summaryId);
  assert.strictEqual(summary.content, "Test summary");
  
  // Test update
  await manager.updateSummary(summaryId, {
    content: "Updated summary"
  });
  const updatedSummary = manager.getSummary(summaryId);
  assert.strictEqual(updatedSummary.content, "Updated summary");
  
  // Test deletion
  await manager.deleteSummary(summaryId);
  const deletedSummary = manager.getSummary(summaryId);
  assert.strictEqual(deletedSummary, undefined);
  
  console.log('testSummaryManagerCRUD passed');
}
```

### Integration Testing
- Test with actual data persistence layers
- Test with concurrent access scenarios
- Test performance with large datasets

## Deployment Instructions

### Prerequisites
- Node.js version 12 or higher
- npm package manager

### Installation
```bash
# Install dependencies
npm install uuid

# Save the module code to summary-manager.js
```

### Basic Usage Setup
```javascript
// In your application
const { SummaryManager, SummaryPriority } = require('./summary-manager');

const manager = new SummaryManager();

// Use the manager in your application logic
```

### Production Considerations
1. **Persistence**: The current implementation uses in-memory storage. For production, implement:
   - Database integration (MongoDB, PostgreSQL, etc.)
   - Data serialization/deserialization
   - Backup and recovery mechanisms

2. **Configuration**: 
   - Environment-specific configurations
   - Logging setup
   - Error reporting integration

3. **Monitoring**:
   - Performance metrics collection
   - Health check endpoints
   - Usage analytics

### Docker Deployment
```dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

EXPOSE 3000

CMD ["node", "app.js"]
```

## Troubleshooting Guide

### Common Issues

#### 1. Summary Not Found Errors
**Symptoms**: `Error: Summary [id] not found`
**Causes**: 
- Incorrect summary ID
- Summary was deleted
- Typo in summary ID

**Solutions**:
```javascript
// Verify summary exists before operations
if (manager.getSummary(summaryId)) {
  await manager.updateSummary(summaryId, updates);
} else {
  console.log("Summary not found");
}
```

#### 2. Invalid Priority Values
**Symptoms**: Unexpected behavior with priority filtering
**Causes**: Using invalid priority strings

**Solutions**:
```javascript
// Always use predefined priority constants
const { SummaryPriority } = require('./summary-manager');

// Correct
summary.priority = SummaryPriority.High;

// Incorrect
summary.priority = "high"; // May work but not recommended
```

#### 3. Parsing Errors
**Symptoms**: `Error: Missing <summary> start tag` or similar
**Causes**: Malformed summary format strings

**Solutions**:
```javascript
try {
  const summary = parseSummaryFormat(summaryText);
} catch (error) {
  console.error("Invalid summary format:", error.message);
  // Handle the error appropriately
}
```

#### 4. Performance Issues with Large Datasets
**Symptoms**: Slow search and filter operations
**Causes**: Linear search through large collections

**Solutions**:
- Implement indexing for frequently queried fields
- Add pagination for large result sets
- Consider database-level optimizations

### Debugging Tips

1. **Enable Logging**:
```javascript
// Add logging to track operations
console.log(`Created summary with ID: ${summaryId}`);
```

2. **Validate Data**:
```javascript
// Check data integrity
console.log("Current summaries count:", manager.summaries.size);
```

3. **Use Error Handling**:
```javascript
try {
  await manager.updateSummary(summaryId, updates);
} catch (error) {
  console.error("Update failed:", error.message);
}
```

### Migration Guide
When upgrading versions, check for:
- Breaking changes in method signatures
- New required dependencies
- Deprecated functionality
- Data structure changes

This comprehensive documentation provides a complete reference for understanding, implementing, and maintaining the Summary Manager module. It covers all aspects from basic usage to advanced deployment considerations.