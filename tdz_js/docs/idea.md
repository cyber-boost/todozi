# Todozi Idea Management System - Comprehensive Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Usage Examples](#usage-examples)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Security Considerations](#security-considerations)
9. [Testing Strategies](#testing-strategies)
10. [Deployment Instructions](#deployment-instructions)
11. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Todozi Idea Management System is a comprehensive solution for organizing, categorizing, and managing ideas within a collaborative environment. This system provides functionality for creating, updating, searching, and analyzing ideas with support for different sharing levels, importance ratings, and tagging systems.

### Key Features
- **Idea Management**: Create, read, update, and delete ideas
- **Categorization**: Tag-based organization and filtering
- **Sharing Control**: Public, team, and private sharing levels
- **Importance Ranking**: Breakthrough to low importance categorization
- **Search Functionality**: Full-text search across ideas, tags, and context
- **Analytics**: Statistics and metrics on idea distribution
- **Import/Export**: Text-based idea format parsing

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          Client Applications                        │
├─────────────────────────────────────────────────────────────────────┤
│                        Idea Management API                          │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐  ┌─────────┐ │
│  │ IdeaManager │  │ IdeaUpdate   │  │ IdeaStatistics  │  │ Parsers │ │
│  │             │  │              │  │                 │  │         │ │
│  │ - CRUD Ops  │  │ - Builder    │  │ - Aggregation   │  │ - Text  │ │
│  │ - Search    │  │ - Mutations  │  │ - Calculations  │  │ - Format│ │
│  │ - Filters   │  │              │  │                 │  │         │ │
│  └─────────────┘  └──────────────┘  └─────────────────┘  └─────────┘ │
├─────────────────────────────────────────────────────────────────────┤
│                           Data Models                               │
├─────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐  ┌─────────┐ │
│  │    Idea     │  │ ItemStatus   │  │ ShareLevel      │  │ Error   │ │
│  │             │  │              │  │                 │  │         │ │
│  │ - Core Data │  │ - Enums      │  │ - Enums         │  │ - Types │ │
│  │ - Metadata  │  │              │  │                 │  │         │ │
│  └─────────────┘  └──────────────┘  └─────────────────┘  └─────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

### Component Relationships

```
IdeaManager ◄───► IdeaUpdate (uses for updates)
IdeaManager ◄───► IdeaStatistics (generates stats)
IdeaManager ◄───► Idea (manages instances)
IdeaManager ◄───► parseIdeaFormat (imports data)
```

## Core Components

### IdeaManager Class

The primary class responsible for all idea management operations.

#### Constructor
```javascript
constructor()
```

**Description**: Initializes the IdeaManager with empty data structures.

**Internal State**:
- `ideas`: Map storing idea objects with UUID keys
- `idea_tags`: Map storing tag arrays for efficient tag-based queries

#### Methods

##### createIdea(idea)
```javascript
async createIdea(idea) → Promise<string>
```

**Description**: Creates a new idea with generated UUID and timestamps.

**Parameters**:
- `idea` (Object): Idea object to be created
  - `idea` (string): The idea text content
  - `share` (ShareLevel): Sharing level (PUBLIC, TEAM, PRIVATE)
  - `importance` (IdeaImportance): Importance level (1-5)
  - `tags` (Array<string>): Array of tag strings
  - `context` (string, optional): Additional context information

**Returns**: 
- `Promise<string>`: UUID of the created idea

**Example**:
```javascript
const ideaId = await ideaManager.createIdea({
    idea: "Implement microservices architecture",
    share: ShareLevel.TEAM,
    importance: IdeaImportance.HIGH,
    tags: ["architecture", "scalability"],
    context: "System performance review"
});
```

##### getIdea(ideaId)
```javascript
getIdea(ideaId) → Object|null
```

**Description**: Retrieves a specific idea by its UUID.

**Parameters**:
- `ideaId` (string): UUID of the idea to retrieve

**Returns**: 
- `Object|null`: Idea object if found, null otherwise

##### getAllIdeas()
```javascript
getAllIdeas() → Array<Object>
```

**Description**: Returns all ideas in the system.

**Returns**: 
- `Array<Object>`: Array of all idea objects

##### updateIdea(ideaId, updates)
```javascript
async updateIdea(ideaId, updates) → Promise<void>
```

**Description**: Updates an existing idea with provided changes.

**Parameters**:
- `ideaId` (string): UUID of the idea to update
- `updates` (Object): Object containing fields to update
  - `idea` (string, optional): New idea text
  - `share` (ShareLevel, optional): New sharing level
  - `importance` (IdeaImportance, optional): New importance level
  - `tags` (Array<string>, optional): New tags array
  - `context` (string, optional): New context information

**Throws**:
- `TodoziError`: If idea with given ID is not found

**Example**:
```javascript
await ideaManager.updateIdea(ideaId, {
    importance: IdeaImportance.BREAKTHROUGH,
    tags: ["innovation", "breakthrough"]
});
```

##### deleteIdea(ideaId)
```javascript
async deleteIdea(ideaId) → Promise<void>
```

**Description**: Deletes an idea and its associated tags.

**Parameters**:
- `ideaId` (string): UUID of the idea to delete

**Throws**:
- `TodoziError`: If idea with given ID is not found

##### searchIdeas(query)
```javascript
searchIdeas(query) → Array<Object>
```

**Description**: Performs case-insensitive search across idea content, tags, and context.

**Parameters**:
- `query` (string): Search term to match

**Returns**: 
- `Array<Object>`: Array of matching idea objects

##### getIdeasByImportance(importance)
```javascript
getIdeasByImportance(importance) → Array<Object>
```

**Description**: Filters ideas by importance level.

**Parameters**:
- `importance` (IdeaImportance): Importance level to filter by

**Returns**: 
- `Array<Object>`: Array of ideas with matching importance

##### getIdeasByShareLevel(shareLevel)
```javascript
getIdeasByShareLevel(shareLevel) → Array<Object>
```

**Description**: Filters ideas by sharing level.

**Parameters**:
- `shareLevel` (ShareLevel): Sharing level to filter by

**Returns**: 
- `Array<Object>`: Array of ideas with matching share level

##### getIdeasByTag(tag)
```javascript
getIdeasByTag(tag) → Array<Object>
```

**Description**: Filters ideas by tag (case-insensitive).

**Parameters**:
- `tag` (string): Tag to filter by

**Returns**: 
- `Array<Object>`: Array of ideas containing the tag

##### getPublicIdeas()
```javascript
getPublicIdeas() → Array<Object>
```

**Description**: Returns all publicly shared ideas.

**Returns**: 
- `Array<Object>`: Array of public ideas

##### getTeamIdeas()
```javascript
getTeamIdeas() → Array<Object>
```

**Description**: Returns all team-shared ideas.

**Returns**: 
- `Array<Object>`: Array of team ideas

##### getPrivateIdeas()
```javascript
getPrivateIdeas() → Array<Object>
```

**Description**: Returns all private ideas.

**Returns**: 
- `Array<Object>`: Array of private ideas

##### getBreakthroughIdeas()
```javascript
getBreakthroughIdeas() → Array<Object>
```

**Description**: Returns all breakthrough importance ideas.

**Returns**: 
- `Array<Object>`: Array of breakthrough ideas

##### getRecentIdeas(limit)
```javascript
getRecentIdeas(limit) → Array<Object>
```

**Description**: Returns most recently created ideas.

**Parameters**:
- `limit` (number): Maximum number of ideas to return

**Returns**: 
- `Array<Object>`: Array of recent ideas, sorted by creation date

##### getAllTags()
```javascript
getAllTags() → Array<string>
```

**Description**: Returns all unique tags in the system.

**Returns**: 
- `Array<string>`: Array of unique tag strings

##### getTagStatistics()
```javascript
getTagStatistics() → Map<string, number>
```

**Description**: Returns tag usage statistics.

**Returns**: 
- `Map<string, number>`: Map of tag names to usage counts

##### getIdeaStatistics()
```javascript
getIdeaStatistics() → IdeaStatistics
```

**Description**: Generates comprehensive statistics about ideas.

**Returns**: 
- `IdeaStatistics`: Object containing various statistical metrics

### IdeaUpdate Class

A builder pattern implementation for constructing idea updates.

#### Constructor
```javascript
constructor()
```

**Description**: Initializes an empty IdeaUpdate object.

#### Static Methods

##### new()
```javascript
static new() → IdeaUpdate
```

**Description**: Factory method to create a new IdeaUpdate instance.

**Returns**: 
- `IdeaUpdate`: New instance

#### Methods

##### withIdea(idea)
```javascript
withIdea(idea) → IdeaUpdate
```

**Description**: Sets the idea text.

**Parameters**:
- `idea` (string): Idea text content

**Returns**: 
- `IdeaUpdate`: This instance for chaining

##### withShare(share)
```javascript
withShare(share) → IdeaUpdate
```

**Description**: Sets the sharing level.

**Parameters**:
- `share` (ShareLevel): Sharing level

**Returns**: 
- `IdeaUpdate`: This instance for chaining

##### withImportance(importance)
```javascript
withImportance(importance) → IdeaUpdate
```

**Description**: Sets the importance level.

**Parameters**:
- `importance` (IdeaImportance): Importance level

**Returns**: 
- `IdeaUpdate`: This instance for chaining

##### withTags(tags)
```javascript
withTags(tags) → IdeaUpdate
```

**Description**: Sets the tags array.

**Parameters**:
- `tags` (Array<string>): Array of tags

**Returns**: 
- `IdeaUpdate`: This instance for chaining

##### withContext(context)
```javascript
withContext(context) → IdeaUpdate
```

**Description**: Sets the context information.

**Parameters**:
- `context` (string): Context information

**Returns**: 
- `IdeaUpdate`: This instance for chaining

### IdeaStatistics Class

Container for idea statistics and metrics.

#### Constructor
```javascript
constructor({ 
    totalIdeas = 0, 
    publicIdeas = 0, 
    teamIdeas = 0, 
    privateIdeas = 0, 
    breakthroughIdeas = 0, 
    uniqueTags = 0 
} = {})
```

**Description**: Initializes statistics with provided values or defaults.

**Parameters**:
- `totalIdeas` (number): Total number of ideas
- `publicIdeas` (number): Number of public ideas
- `teamIdeas` (number): Number of team ideas
- `privateIdeas` (number): Number of private ideas
- `breakthroughIdeas` (number): Number of breakthrough ideas
- `uniqueTags` (number): Number of unique tags

#### Methods

##### publicPercentage()
```javascript
publicPercentage() → number
```

**Description**: Calculates percentage of public ideas.

**Returns**: 
- `number`: Percentage as decimal (0.0-100.0)

##### teamPercentage()
```javascript
teamPercentage() → number
```

**Description**: Calculates percentage of team ideas.

**Returns**: 
- `number`: Percentage as decimal (0.0-100.0)

##### privatePercentage()
```javascript
privatePercentage() → number
```

**Description**: Calculates percentage of private ideas.

**Returns**: 
- `number`: Percentage as decimal (0.0-100.0)

##### breakthroughPercentage()
```javascript
breakthroughPercentage() → number
```

**Description**: Calculates percentage of breakthrough ideas.

**Returns**: 
- `number`: Percentage as decimal (0.0-100.0)

### parseIdeaFormat(ideaText)
```javascript
parseIdeaFormat(ideaText) → Idea
```

**Description**: Parses idea text in custom format and returns Idea object.

**Parameters**:
- `ideaText` (string): Text in format `<idea>idea text; share level; importance; tags; context</idea>`

**Returns**: 
- `Idea`: Parsed Idea object

**Format Details**:
- Start tag: `<idea>`
- End tag: `</idea>`
- Parts separated by semicolons:
  1. Idea text
  2. Share level ("share", "dont share", "don't share", "private", "team")
  3. Importance (1-5)
  4. Tags (comma-separated, optional)
  5. Context (optional)

**Example**:
```javascript
const idea = parseIdeaFormat("<idea>Microservices architecture; team; 4; architecture,scalability; System redesign</idea>");
```

## Usage Examples

### Basic Idea Management

```javascript
const { IdeaManager, IdeaUpdate } = require('./idea-manager');

// Initialize manager
const ideaManager = new IdeaManager();

// Create ideas
const ideaId1 = await ideaManager.createIdea({
    idea: "Implement caching layer",
    share: ShareLevel.TEAM,
    importance: IdeaImportance.HIGH,
    tags: ["performance", "optimization"],
    context: "Database query optimization"
});

const ideaId2 = await ideaManager.createIdea({
    idea: "Redesign user interface",
    share: ShareLevel.PUBLIC,
    importance: IdeaImportance.BREAKTHROUGH,
    tags: ["ui", "ux", "redesign"],
    context: "User experience improvement"
});

// Retrieve ideas
const idea = ideaManager.getIdea(ideaId1);
const allIdeas = ideaManager.getAllIdeas();

// Update ideas using builder pattern
await ideaManager.updateIdea(ideaId1, 
    IdeaUpdate.new()
        .withImportance(IdeaImportance.BREAKTHROUGH)
        .withTags(["performance", "optimization", "breakthrough"])
        .withContext("Critical system optimization")
);

// Search ideas
const searchResults = ideaManager.searchIdeas("optimization");

// Filter ideas
const breakthroughIdeas = ideaManager.getBreakthroughIdeas();
const publicIdeas = ideaManager.getPublicIdeas();
const ideasWithTag = ideaManager.getIdeasByTag("ui");

// Delete ideas
await ideaManager.deleteIdea(ideaId1);
```

### Statistics and Analytics

```javascript
// Get comprehensive statistics
const stats = ideaManager.getIdeaStatistics();

console.log(`Total ideas: ${stats.totalIdeas}`);
console.log(`Public ideas: ${stats.publicPercentage().toFixed(2)}%`);
console.log(`Breakthrough ideas: ${stats.breakthroughPercentage().toFixed(2)}%`);

// Get tag statistics
const tagStats = ideaManager.getTagStatistics();
for (const [tag, count] of tagStats) {
    console.log(`${tag}: ${count} ideas`);
}

// Get recent ideas
const recentIdeas = ideaManager.getRecentIdeas(5);
console.log("5 most recent ideas:", recentIdeas);
```

### Text Format Parsing

```javascript
const { parseIdeaFormat } = require('./idea-manager');

// Parse various formats
const idea1 = parseIdeaFormat("<idea>Database sharding; share; 3; database,scaling; Horizontal scaling</idea>");
const idea2 = parseIdeaFormat("<idea>API rate limiting; team; 4; security,api; Prevent abuse</idea>");
const idea3 = parseIdeaFormat("<idea>Mobile app redesign; private; 5; mobile,ui; User experience</idea>");

// Add parsed ideas to manager
const ideaId = await ideaManager.createIdea(idea1);
```

### Advanced Filtering and Search

```javascript
// Complex search scenarios
const searchResults = ideaManager.searchIdeas("database");

// Filter by multiple criteria
const highImportancePublicIdeas = ideaManager.getIdeasByImportance(IdeaImportance.HIGH)
    .filter(idea => idea.share === ShareLevel.PUBLIC);

// Get ideas by date range (custom implementation)
const recentBreakthroughIdeas = ideaManager.getBreakthroughIdeas()
    .filter(idea => {
        const oneWeekAgo = new Date();
        oneWeekAgo.setDate(oneWeekAgo.getDate() - 7);
        return idea.createdAt > oneWeekAgo;
    });
```

## Design Patterns

### Builder Pattern
The `IdeaUpdate` class implements the Builder pattern to provide a fluent interface for constructing idea updates.

**Benefits**:
- Immutable update objects
- Method chaining for readability
- Type-safe construction
- Optional parameter handling

### Repository Pattern
The `IdeaManager` class acts as a repository for idea objects, providing a clean interface for data access operations.

**Benefits**:
- Encapsulation of data access logic
- Separation of concerns
- Testability
- Consistent API

### Factory Pattern
The `parseIdeaFormat` function acts as a factory method for creating Idea objects from text format.

**Benefits**:
- Centralized object creation
- Input validation
- Format parsing abstraction

### Strategy Pattern
Filtering methods like `getIdeasByImportance` and `getIdeasByTag` implement different filtering strategies.

**Benefits**:
- Reusable filtering logic
- Easy to extend with new filters
- Consistent interface

## Performance Analysis

### Time Complexity

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| createIdea | O(1) | Hash map insertion |
| getIdea | O(1) | Hash map lookup |
| getAllIdeas | O(n) | Array conversion |
| updateIdea | O(1) | Hash map lookup + update |
| deleteIdea | O(1) | Hash map deletion |
| searchIdeas | O(n*m) | n=ideas, m=average text length |
| getIdeasBy* | O(n) | Linear filtering |
| getRecentIdeas | O(n log n) | Sorting required |
| getAllTags | O(n*t) | n=ideas, t=average tags per idea |
| getTagStatistics | O(n*t) | Tag counting |

### Space Complexity

| Component | Space Complexity | Notes |
|-----------|------------------|-------|
| ideas Map | O(n) | n=number of ideas |
| idea_tags Map | O(n*t) | t=average tags per idea |
| getAllIdeas result | O(n) | Array copy |
| searchIdeas result | O(k) | k=matching ideas |

### Performance Optimizations

1. **Hash Maps for Fast Lookup**: O(1) access to ideas by ID
2. **Separate Tag Storage**: Dedicated map for tag-based queries
3. **In-place Updates**: Direct modification of existing objects
4. **Lazy Array Conversion**: Only convert to arrays when needed

### Scalability Considerations

- **Memory Usage**: Current implementation stores all data in memory
- **Large Dataset Performance**: Search operations may become slow with large datasets
- **Concurrent Access**: No built-in concurrency control
- **Persistence**: No built-in data persistence

## Security Considerations

### Input Validation

1. **Idea Content Validation**: No explicit validation of idea content
2. **Tag Sanitization**: No sanitization of tag inputs
3. **Context Validation**: No validation of context data
4. **ID Generation**: Uses cryptographically secure UUID v4

### Access Control

1. **Share Level Enforcement**: Application-level enforcement required
2. **User Authentication**: No built-in authentication system
3. **Authorization**: No built-in authorization system
4. **Data Isolation**: No multi-tenant data isolation

### Injection Prevention

1. **Text Parsing**: Safe parsing with proper error handling
2. **Search Queries**: No SQL injection risk (in-memory data)
3. **Tag Processing**: Safe array operations

### Recommendations

1. **Add Input Sanitization**: Validate and sanitize all user inputs
2. **Implement Authentication**: Add user authentication system
3. **Add Authorization**: Implement role-based access control
4. **Data Encryption**: Consider encrypting sensitive idea content
5. **Audit Logging**: Add logging for idea operations

## Testing Strategies

### Unit Tests

```javascript
// Test idea creation
describe('IdeaManager.createIdea', () => {
    it('should create idea with generated ID', async () => {
        const manager = new IdeaManager();
        const idea = { idea: 'Test idea', share: ShareLevel.PUBLIC, importance: IdeaImportance.HIGH, tags: [] };
        const id = await manager.createIdea(idea);
        expect(id).toBeDefined();
        expect(manager.getIdea(id)).toBeDefined();
    });
});

// Test idea updates
describe('IdeaManager.updateIdea', () => {
    it('should update idea fields', async () => {
        const manager = new IdeaManager();
        const id = await manager.createIdea({ idea: 'Original', share: ShareLevel.PRIVATE, importance: IdeaImportance.LOW, tags: [] });
        await manager.updateIdea(id, { idea: 'Updated', share: ShareLevel.PUBLIC });
        const updated = manager.getIdea(id);
        expect(updated.idea).toBe('Updated');
        expect(updated.share).toBe(ShareLevel.PUBLIC);
    });
});
```

### Integration Tests

```javascript
// Test search functionality
describe('IdeaManager.searchIdeas', () => {
    it('should find ideas by content', () => {
        const manager = new IdeaManager();
        manager.createIdea({ idea: 'Database optimization', share: ShareLevel.PUBLIC, importance: IdeaImportance.HIGH, tags: ['database'] });
        const results = manager.searchIdeas('database');
        expect(results.length).toBe(1);
        expect(results[0].idea).toContain('Database');
    });
});
```

### Performance Tests

```javascript
// Test with large datasets
describe('IdeaManager performance', () => {
    it('should handle 10000 ideas efficiently', () => {
        const manager = new IdeaManager();
        const start = Date.now();
        
        // Create 10000 ideas
        for (let i = 0; i < 10000; i++) {
            manager.createIdea({
                idea: `Idea ${i}`,
                share: ShareLevel.PUBLIC,
                importance: IdeaImportance.MEDIUM,
                tags: [`tag${i % 100}`]
            });
        }
        
        const creationTime = Date.now() - start;
        expect(creationTime).toBeLessThan(5000); // Should be under 5 seconds
        
        // Test search performance
        const searchStart = Date.now();
        const results = manager.searchIdeas('Idea 5000');
        const searchTime = Date.now() - searchStart;
        expect(searchTime).toBeLessThan(100); // Should be under 100ms
    });
});
```

### Test Coverage Areas

1. **CRUD Operations**: Create, Read, Update, Delete
2. **Search Functionality**: Text search across fields
3. **Filtering**: By importance, share level, tags
4. **Statistics**: All statistical calculations
5. **Error Handling**: Invalid inputs, missing data
6. **Edge Cases**: Empty datasets, boundary conditions
7. **Performance**: Large dataset handling
8. **Integration**: Component interaction

## Deployment Instructions

### Prerequisites

1. **Node.js**: Version 14.0 or higher
2. **npm**: Package manager
3. **Dependencies**: uuid package

### Installation

```bash
# Clone repository
git clone <repository-url>
cd todozi-idea-management

# Install dependencies
npm install uuid

# Install development dependencies (optional)
npm install --save-dev jest
```

### Configuration

Create a configuration file `config.js`:

```javascript
module.exports = {
    app: {
        port: process.env.PORT || 3000,
        environment: process.env.NODE_ENV || 'development'
    },
    database: {
        // Add database configuration if needed
    }
};
```

### Environment Variables

```bash
# Set environment variables
export NODE_ENV=production
export PORT=8080
```

### Running Tests

```bash
# Run unit tests
npm test

# Run with coverage
npm run test:coverage

# Run performance tests
npm run test:performance
```

### Production Deployment

1. **Build Process**: No build required for Node.js
2. **Process Management**: Use PM2 or similar
3. **Load Balancing**: Consider clustering for high availability
4. **Monitoring**: Implement application monitoring
5. **Logging**: Configure structured logging

### Container Deployment

Dockerfile:
```dockerfile
FROM node:14-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 3000
CMD ["node", "server.js"]
```

Docker Compose:
```yaml
version: '3.8'
services:
  idea-manager:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
    volumes:
      - ./data:/app/data
```

## Troubleshooting Guide

### Common Issues

#### 1. Idea Not Found Errors

**Symptoms**: `TodoziError: Idea <id> not found`

**Causes**:
- Invalid UUID format
- Idea was deleted
- Typo in idea ID
- Concurrent modification issues

**Solutions**:
```javascript
// Verify idea exists before operations
if (ideaManager.getIdea(ideaId)) {
    await ideaManager.updateIdea(ideaId, updates);
} else {
    console.log('Idea not found');
}
```

#### 2. Search Returns No Results

**Symptoms**: Empty search results despite matching content

**Causes**:
- Case sensitivity issues
- Special characters in search terms
- Partial word matching expectations

**Solutions**:
```javascript
// Use more flexible search
const flexibleSearch = (query) => {
    const normalizedQuery = query.toLowerCase();
    return ideaManager.getAllIdeas().filter(idea => 
        JSON.stringify(idea).toLowerCase().includes(normalizedQuery)
    );
};
```

#### 3. Performance Degradation

**Symptoms**: Slow response times with large datasets

**Causes**:
- Inefficient search algorithms
- Memory pressure
- Lack of indexing

**Solutions**:
```javascript
// Add simple indexing for tags
class IdeaManager {
    constructor() {
        this.ideas = new Map();
        this.tagIndex = new Map(); // Add tag index
    }
    
    // Update index when adding ideas
    async createIdea(idea) {
        const id = await super.createIdea(idea);
        // Update tag index
        idea.tags.forEach(tag => {
            if (!this.tagIndex.has(tag)) {
                this.tagIndex.set(tag, new Set());
            }
            this.tagIndex.get(tag).add(id);
        });
        return id;
    }
}
```

#### 4. Memory Leaks

**Symptoms**: Increasing memory usage over time

**Causes**:
- Circular references
- Uncleaned event listeners
- Large dataset accumulation

**Solutions**:
```javascript
// Monitor memory usage
const used = process.memoryUsage();
console.log({
    rss: Math.round(used.rss / 1024 / 1024 * 100) / 100 + " MB",
    heapTotal: Math.round(used.heapTotal / 1024 / 1024 * 100) / 100 + " MB",
    heapUsed: Math.round(used.heapUsed / 1024 / 1024 * 100) / 100 + " MB"
});

// Implement cleanup methods
class IdeaManager {
    clear() {
        this.ideas.clear();
        this.idea_tags.clear();
    }
}
```

### Debugging Techniques

1. **Logging**: Add comprehensive logging for operations
2. **Profiling**: Use Node.js profiler for performance issues
3. **Memory Snapshots**: Take heap snapshots to identify leaks
4. **Unit Testing**: Isolate components for targeted debugging

### Monitoring and Metrics

```javascript
// Add basic monitoring
class IdeaManager {
    constructor() {
        this.metrics = {
            ideasCreated: 0,
            ideasUpdated: 0,
            ideasDeleted: 0,
            searchOperations: 0
        };
    }
    
    async createIdea(idea) {
        this.metrics.ideasCreated++;
        // ... implementation
    }
    
    getMetrics() {
        return { ...this.metrics };
    }
}
```

This comprehensive documentation provides a complete overview of the Todozi Idea Management System, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.