# SearchEngine Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Classes and Methods](#classes-and-methods)
4. [Usage Examples](#usage-examples)
5. [Design Patterns](#design-patterns)
6. [Performance Analysis](#performance-analysis)
7. [Security Considerations](#security-considerations)
8. [Testing Strategies](#testing-strategies)
9. [Deployment Instructions](#deployment-instructions)
10. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The SearchEngine is a JavaScript class designed to index and search through different types of content including tasks, memories, ideas, errors, and training data. It provides both basic text search functionality and advanced filtering capabilities with scoring and analytics features.

### Key Features
- Multi-type content indexing (tasks, memories, ideas, errors, training data)
- Text-based search with relevance scoring
- Tag-based search capabilities
- Advanced filtering with custom criteria
- Search analytics and suggestions
- Configurable search options (date ranges, limits, data type filtering)

## Architecture

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                    SearchEngine Class                       │
├─────────────────────────────────────────────────────────────┤
│ Properties:                                                 │
│ - tasks[]                                                   │
│ - memories[]                                                │
│ - ideas[]                                                   │
│ - errors[]                                                  │
│ - training_data[]                                           │
│ - tags[]                                                    │
├─────────────────────────────────────────────────────────────┤
│ Methods:                                                    │
│ - updateIndex(content)                                      │
│ - search(query, options)                                    │
│ - matchesQuery(query, primaryText, secondaryText, tags)     │
│ - calculateRelevanceScore(query, text, tags)                │
│ - getSearchAnalytics()                                      │
│ - getSearchSuggestions(query, limit)                        │
│ - extractKeywords(text, keywords)                           │
│ - advancedSearch(criteria)                                  │
│ - matchesAdvancedCriteria(task, criteria)                   │
│ - matchesAdvancedMemoryCriteria(memory, criteria)           │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    SearchResults Class                      │
├─────────────────────────────────────────────────────────────┤
│ Properties:                                                 │
│ - taskResults[]                                             │
│ - memoryResults[]                                           │
│ - ideaResults[]                                             │
│ - errorResults[]                                            │
│ - trainingResults[]                                         │
├─────────────────────────────────────────────────────────────┤
│ Methods:                                                    │
│ - totalResults()                                            │
│ - hasResults()                                              │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    SearchOptions Class                      │
├─────────────────────────────────────────────────────────────┤
│ Properties:                                                 │
│ - dataTypes                                                 │
│ - since                                                     │
│ - until                                                     │
│ - limit                                                     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                   SearchDataType Enum                       │
├─────────────────────────────────────────────────────────────┤
│ Values:                                                     │
│ - Tasks                                                     │
│ - Memories                                                  │
│ - Ideas                                                     │
│ - Errors                                                    │
│ - Training                                                  │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow Diagram

```
Input Data → updateIndex() → Internal Collections
                                ↓
Search Query → search() → matchesQuery() → calculateRelevanceScore()
                                ↓
                        Filter by Options (dataTypes, date ranges, limits)
                                ↓
                            SearchResults
```

## Classes and Methods

### SearchEngine Class

#### Constructor
```javascript
constructor()
```
Initializes a new SearchEngine instance with empty collections for all content types.

**Properties:**
- `tasks` (Array): Collection of task objects
- `memories` (Array): Collection of memory objects
- `ideas` (Array): Collection of idea objects
- `errors` (Array): Collection of error objects
- `training_data` (Array): Collection of training data objects
- `tags` (Array): Collection of tags (currently unused in implementation)

#### updateIndex(content)
Adds new content to the search index.

**Parameters:**
- `content` (Object): Object containing arrays of different content types
  - `content.tasks` (Array): Array of task objects to add
  - `content.memories` (Array): Array of memory objects to add
  - `content.ideas` (Array): Array of idea objects to add
  - `content.errors` (Array): Array of error objects to add
  - `content.training_data` (Array): Array of training data objects to add

**Returns:** void

**Example:**
```javascript
const engine = new SearchEngine();
engine.updateIndex({
    tasks: [{ action: "Complete project", tags: ["work", "urgent"] }],
    memories: [{ moment: "Meeting", meaning: "Important discussion", tags: ["meeting"] }]
});
```

#### search(query, options)
Performs a text-based search across all indexed content types.

**Parameters:**
- `query` (String): Search query string
- `options` (SearchOptions, optional): Search configuration options

**Returns:** SearchResults - Object containing sorted search results

**SearchOptions Properties:**
- `dataTypes` (Array): Array of SearchDataType values to include in search
- `since` (Date): Only include results created after this date
- `until` (Date): Only include results created before this date
- `limit` (Number): Maximum number of results per content type (default: 50)

**Example:**
```javascript
const results = engine.search("project", {
    dataTypes: [SearchDataType.Tasks],
    limit: 10
});
```

#### matchesQuery(query, primaryText, secondaryText, tags)
Checks if a query matches any of the provided text fields or tags.

**Parameters:**
- `query` (String): Lowercase search query
- `primaryText` (String): Primary text field to search
- `secondaryText` (String|null): Secondary text field to search (optional)
- `tags` (Array): Array of tag strings to search

**Returns:** Boolean - True if query matches any field

#### calculateRelevanceScore(query, text, tags)
Calculates a relevance score for search results based on text matching.

**Parameters:**
- `query` (String): Lowercase search query
- `text` (String): Text to evaluate for relevance
- `tags` (Array): Array of tags to evaluate

**Returns:** Number - Relevance score (higher is more relevant)

**Scoring Algorithm:**
1. Full text match: +1.0
2. Word matches: +0.7 per word
3. Tag matches: +0.5 per tag
4. Length penalty: Score multiplied by 1.0 / max(text.length/100, 1.0)

#### getSearchAnalytics()
Returns analytics about the current search index.

**Returns:** Object - Analytics data
- `totalIndexedItems` (Number): Total items across all collections
- `tasksCount` (Number): Number of tasks
- `memoriesCount` (Number): Number of memories
- `ideasCount` (Number): Number of ideas
- `errorsCount` (Number): Number of errors
- `trainingCount` (Number): Number of training data items

#### getSearchSuggestions(query, limit)
Generates search suggestions based on indexed keywords.

**Parameters:**
- `query` (String): Partial query to match suggestions against
- `limit` (Number): Maximum number of suggestions to return

**Returns:** Array - Array of suggested keywords

#### extractKeywords(text, keywords)
Extracts keywords from text and updates a keyword frequency map.

**Parameters:**
- `text` (String): Text to extract keywords from
- `keywords` (Map): Map to store keyword frequencies

**Returns:** void

#### advancedSearch(criteria)
Performs advanced search with structured criteria.

**Parameters:**
- `criteria` (Object): Search criteria object
  - `taskCriteria` (Object): Criteria for filtering tasks
  - `memoryCriteria` (Object): Criteria for filtering memories

**Returns:** SearchResults - Search results matching advanced criteria

#### matchesAdvancedCriteria(task, criteria)
Checks if a task matches advanced search criteria.

**Parameters:**
- `task` (Object): Task object to evaluate
- `criteria` (Object): Criteria to match against
  - `status` (String, optional): Required task status
  - `priority` (String, optional): Required task priority
  - `assignee` (String, optional): Required task assignee
  - `requiredTag` (String, optional): Required tag

**Returns:** Boolean - True if task matches all criteria

#### matchesAdvancedMemoryCriteria(memory, criteria)
Checks if a memory matches advanced search criteria.

**Parameters:**
- `memory` (Object): Memory object to evaluate
- `criteria` (Object): Criteria to match against
  - `importance` (String, optional): Required memory importance
  - `term` (String, optional): Required memory term
  - `requiredTag` (String, optional): Required tag

**Returns:** Boolean - True if memory matches all criteria

### SearchOptions Class

#### Constructor
```javascript
constructor()
```
Initializes default search options.

**Properties:**
- `dataTypes` (null|Array): Data types to include in search (null = all)
- `since` (null|Date): Minimum creation date
- `until` (null|Date): Maximum creation date
- `limit` (Number): Results limit per content type (default: 50)

### SearchDataType Enum

Enumeration of supported content types for search filtering.

**Values:**
- `Tasks`: Task content type
- `Memories`: Memory content type
- `Ideas`: Idea content type
- `Errors`: Error content type
- `Training`: Training data content type

### SearchResults Class

#### Constructor
```javascript
constructor()
```
Initializes empty search results collections.

**Properties:**
- `taskResults` (Array): Task search results
- `memoryResults` (Array): Memory search results
- `ideaResults` (Array): Idea search results
- `errorResults` (Array): Error search results
- `trainingResults` (Array): Training data search results

#### totalResults()
Calculates the total number of results across all content types.

**Returns:** Number - Total results count

#### hasResults()
Checks if any results were found.

**Returns:** Boolean - True if any results exist

## Usage Examples

### Basic Setup and Indexing

```javascript
// Create search engine instance
const engine = new SearchEngine();

// Add content to index
engine.updateIndex({
    tasks: [
        {
            action: "Complete quarterly report",
            status: "pending",
            priority: "high",
            assignee: "john.doe",
            tags: ["report", "quarterly", "finance"],
            createdAt: new Date("2023-10-01")
        }
    ],
    memories: [
        {
            moment: "Team meeting",
            meaning: "Discussed project timeline and deliverables",
            importance: "high",
            tags: ["meeting", "project", "timeline"],
            createdAt: new Date("2023-10-02")
        }
    ],
    ideas: [
        {
            idea: "Implement new caching strategy",
            context: "Performance optimization",
            tags: ["optimization", "caching", "performance"],
            createdAt: new Date("2023-10-03")
        }
    ]
});
```

### Basic Text Search

```javascript
// Simple text search
const results = engine.search("project");
console.log(`Found ${results.totalResults()} results`);

// Search with options
const filteredResults = engine.search("report", {
    dataTypes: [SearchDataType.Tasks],
    since: new Date("2023-09-01"),
    until: new Date("2023-12-31"),
    limit: 5
});

if (filteredResults.hasResults()) {
    console.log("Task results:", filteredResults.taskResults);
}
```

### Advanced Search

```javascript
// Advanced search with structured criteria
const advancedResults = engine.advancedSearch({
    taskCriteria: {
        status: "pending",
        priority: "high",
        requiredTag: "finance"
    },
    memoryCriteria: {
        importance: "high",
        requiredTag: "project"
    }
});

console.log("Advanced search results:", advancedResults);
```

### Analytics and Suggestions

```javascript
// Get search analytics
const analytics = engine.getSearchAnalytics();
console.log("Search analytics:", analytics);

// Get search suggestions
const suggestions = engine.getSearchSuggestions("proj", 5);
console.log("Search suggestions:", suggestions);
```

### Complete Example Application

```javascript
class SearchApplication {
    constructor() {
        this.engine = new SearchEngine();
        this.initializeData();
    }

    initializeData() {
        this.engine.updateIndex({
            tasks: [
                {
                    action: "Prepare marketing presentation",
                    status: "in-progress",
                    priority: "medium",
                    assignee: "marketing-team",
                    tags: ["presentation", "marketing", "q4"],
                    createdAt: new Date("2023-10-15")
                },
                {
                    action: "Fix login authentication bug",
                    status: "pending",
                    priority: "high",
                    assignee: "dev-team",
                    tags: ["bug", "authentication", "security"],
                    createdAt: new Date("2023-10-20")
                }
            ],
            memories: [
                {
                    moment: "Client feedback session",
                    meaning: "Client requested additional features for dashboard",
                    importance: "medium",
                    tags: ["client", "feedback", "dashboard"],
                    createdAt: new Date("2023-10-18")
                }
            ]
        });
    }

    performSearch(query, options = {}) {
        const results = this.engine.search(query, options);
        this.displayResults(results);
        return results;
    }

    displayResults(results) {
        console.log(`\n=== Search Results (${results.totalResults()} found) ===`);
        
        if (results.taskResults.length > 0) {
            console.log("\nTasks:");
            results.taskResults.forEach((result, index) => {
                console.log(`${index + 1}. ${result.task.action} (Score: ${result.score.toFixed(2)})`);
            });
        }

        if (results.memoryResults.length > 0) {
            console.log("\nMemories:");
            results.memoryResults.forEach((result, index) => {
                console.log(`${index + 1}. ${result.memory.moment} - ${result.memory.meaning} (Score: ${result.score.toFixed(2)})`);
            });
        }
    }

    showAnalytics() {
        const analytics = this.engine.getSearchAnalytics();
        console.log("\n=== Search Analytics ===");
        console.log(`Total indexed items: ${analytics.totalIndexedItems}`);
        console.log(`Tasks: ${analytics.tasksCount}`);
        console.log(`Memories: ${analytics.memoriesCount}`);
    }
}

// Usage
const app = new SearchApplication();
app.performSearch("marketing");
app.showAnalytics();
```

## Design Patterns

### 1. Factory Pattern
The SearchResults and SearchOptions classes act as factories for creating standardized result and option objects.

### 2. Strategy Pattern
Different search strategies are implemented:
- Basic text search (`search` method)
- Advanced criteria search (`advancedSearch` method)
- Analytics gathering (`getSearchAnalytics` method)

### 3. Builder Pattern
SearchOptions provides a configurable way to build search parameters.

### 4. Composite Pattern
SearchResults composes multiple result arrays into a single interface.

### 5. Decorator Pattern
Search results are decorated with relevance scores.

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| updateIndex | O(n) | Where n is total items added |
| search | O(n*m) | Where n is total indexed items, m is average item size |
| matchesQuery | O(k) | Where k is total text length + tags |
| calculateRelevanceScore | O(w) | Where w is number of words in query |
| getSearchAnalytics | O(1) | Simple property access |
| getSearchSuggestions | O(n*k) | Where n is items, k is average keywords per item |

### Space Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| Index Storage | O(n*m) | Where n is items, m is average item size |
| Search Results | O(r) | Where r is result count |
| Keywords Map | O(k) | Where k is unique keywords |

### Performance Optimization Recommendations

1. **Indexing Optimization:**
   ```javascript
   // Pre-process text for faster searching
   class OptimizedSearchEngine extends SearchEngine {
       preprocessText(text) {
           return text.toLowerCase().trim();
       }
       
       // Use indexing for large datasets
       buildInvertedIndex() {
           this.invertedIndex = new Map();
           // Implementation for faster lookups
       }
   }
   ```

2. **Caching Results:**
   ```javascript
   class CachedSearchEngine extends SearchEngine {
       constructor() {
           super();
           this.searchCache = new Map();
       }
       
       search(query, options = {}) {
           const cacheKey = `${query}-${JSON.stringify(options)}`;
           if (this.searchCache.has(cacheKey)) {
               return this.searchCache.get(cacheKey);
           }
           
           const results = super.search(query, options);
           this.searchCache.set(cacheKey, results);
           return results;
       }
   }
   ```

3. **Pagination for Large Results:**
   ```javascript
   searchWithPagination(query, page = 1, pageSize = 10) {
       const allResults = this.search(query);
       const startIndex = (page - 1) * pageSize;
       const endIndex = startIndex + pageSize;
       
       return {
           results: {
               taskResults: allResults.taskResults.slice(startIndex, endIndex),
               memoryResults: allResults.memoryResults.slice(startIndex, endIndex)
               // ... other result types
           },
           pagination: {
               currentPage: page,
               pageSize: pageSize,
               totalResults: allResults.totalResults(),
               totalPages: Math.ceil(allResults.totalResults() / pageSize)
           }
       };
   }
   ```

## Security Considerations

### 1. Input Validation
```javascript
validateSearchQuery(query) {
    if (typeof query !== 'string') {
        throw new Error('Search query must be a string');
    }
    
    if (query.length > 1000) {
        throw new Error('Search query too long');
    }
    
    // Prevent injection attacks
    const sanitizedQuery = query.replace(/[<>]/g, '');
    return sanitizedQuery;
}
```

### 2. Content Sanitization
```javascript
sanitizeContent(content) {
    // Remove potentially harmful content
    const sanitized = { ...content };
    
    if (sanitized.action) {
        sanitized.action = this.escapeHtml(sanitized.action);
    }
    
    if (sanitized.tags) {
        sanitized.tags = sanitized.tags.map(tag => this.escapeHtml(tag));
    }
    
    return sanitized;
}

escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    
    return text.replace(/[&<>"']/g, m => map[m]);
}
```

### 3. Rate Limiting
```javascript
class RateLimitedSearchEngine extends SearchEngine {
    constructor() {
        super();
        this.searchCounts = new Map();
        this.maxSearchesPerMinute = 100;
    }
    
    search(query, options = {}) {
        const clientId = this.getClientId();
        const now = Date.now();
        const minuteAgo = now - 60000;
        
        // Clean old entries
        if (this.searchCounts.has(clientId)) {
            const counts = this.searchCounts.get(clientId).filter(time => time > minuteAgo);
            this.searchCounts.set(clientId, counts);
        } else {
            this.searchCounts.set(clientId, []);
        }
        
        // Check rate limit
        if (this.searchCounts.get(clientId).length >= this.maxSearchesPerMinute) {
            throw new Error('Rate limit exceeded');
        }
        
        // Record search
        this.searchCounts.get(clientId).push(now);
        
        return super.search(query, options);
    }
    
    getClientId() {
        // Implementation depends on environment
        return 'anonymous'; // Placeholder
    }
}
```

## Testing Strategies

### Unit Tests

```javascript
describe('SearchEngine', () => {
    let engine;
    
    beforeEach(() => {
        engine = new SearchEngine();
    });
    
    describe('updateIndex', () => {
        it('should add content to appropriate collections', () => {
            const content = {
                tasks: [{ action: 'test task' }],
                memories: [{ moment: 'test memory' }]
            };
            
            engine.updateIndex(content);
            
            expect(engine.tasks).toHaveLength(1);
            expect(engine.memories).toHaveLength(1);
            expect(engine.tasks[0].action).toBe('test task');
        });
    });
    
    describe('search', () => {
        beforeEach(() => {
            engine.updateIndex({
                tasks: [
                    { action: 'Complete project', tags: ['work'], createdAt: new Date() }
                ]
            });
        });
        
        it('should find matching content', () => {
            const results = engine.search('project');
            expect(results.taskResults).toHaveLength(1);
            expect(results.taskResults[0].task.action).toBe('Complete project');
        });
        
        it('should respect data type filters', () => {
            const results = engine.search('project', {
                dataTypes: [SearchDataType.Memories]
            });
            expect(results.taskResults).toHaveLength(0);
        });
    });
    
    describe('calculateRelevanceScore', () => {
        it('should calculate higher scores for exact matches', () => {
            const score1 = engine.calculateRelevanceScore('test', 'test string', []);
            const score2 = engine.calculateRelevanceScore('test', 'string with test word', []);
            expect(score1).toBeGreaterThan(score2);
        });
    });
});
```

### Integration Tests

```javascript
describe('SearchEngine Integration', () => {
    it('should handle complex search scenarios', () => {
        const engine = new SearchEngine();
        
        // Add diverse content
        engine.updateIndex({
            tasks: [
                {
                    action: 'Fix critical bug in authentication',
                    status: 'pending',
                    priority: 'high',
                    tags: ['bug', 'security', 'authentication'],
                    createdAt: new Date('2023-01-01')
                }
            ],
            memories: [
                {
                    moment: 'Security review meeting',
                    meaning: 'Discussed authentication vulnerabilities',
                    tags: ['security', 'meeting', 'authentication'],
                    createdAt: new Date('2023-01-02')
                }
            ]
        });
        
        // Search for security-related content
        const results = engine.search('security');
        
        // Should find both task and memory
        expect(results.taskResults).toHaveLength(1);
        expect(results.memoryResults).toHaveLength(1);
        
        // Task should have higher relevance due to exact match
        expect(results.taskResults[0].score).toBeGreaterThan(results.memoryResults[0].score);
    });
});
```

### Performance Tests

```javascript
describe('SearchEngine Performance', () => {
    it('should handle large datasets efficiently', () => {
        const engine = new SearchEngine();
        
        // Create large dataset
        const largeDataset = {
            tasks: Array.from({ length: 10000 }, (_, i) => ({
                action: `Task ${i}`,
                tags: [`tag${i % 100}`],
                createdAt: new Date()
            }))
        };
        
        const startTime = performance.now();
        engine.updateIndex(largeDataset);
        const indexTime = performance.now() - startTime;
        
        const searchStartTime = performance.now();
        const results = engine.search('tag50');
        const searchTime = performance.now() - searchStartTime;
        
        expect(indexTime).toBeLessThan(5000); // Should index in < 5 seconds
        expect(searchTime).toBeLessThan(1000); // Should search in < 1 second
        expect(results.taskResults.length).toBeGreaterThan(0);
    });
});
```

## Deployment Instructions

### 1. Environment Setup

```bash
# Install Node.js dependencies (if using in Node environment)
npm init -y
npm install --save-dev jest # for testing

# Create deployment directory
mkdir search-engine-deployment
cd search-engine-deployment
```

### 2. File Structure

```
search-engine-deployment/
├── src/
│   ├── search-engine.js
│   └── index.js
├── tests/
│   └── search-engine.test.js
├── package.json
└── README.md
```

### 3. Package Configuration

```json
{
  "name": "search-engine",
  "version": "1.0.0",
  "description": "JavaScript search engine implementation",
  "main": "src/search-engine.js",
  "scripts": {
    "test": "jest",
    "start": "node src/index.js"
  },
  "dependencies": {},
  "devDependencies": {
    "jest": "^29.0.0"
  }
}
```

### 4. Module Export

```javascript
// src/search-engine.js
// ... existing code ...

// Export for Node.js modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        SearchEngine,
        SearchOptions,
        SearchResults,
        SearchDataType
    };
}

// Export for ES6 modules
if (typeof exports !== 'undefined') {
    exports.SearchEngine = SearchEngine;
    exports.SearchOptions = SearchOptions;
    exports.SearchResults = SearchResults;
    exports.SearchDataType = SearchDataType;
}
```

### 5. Usage in Different Environments

#### Browser Usage
```html
<!DOCTYPE html>
<html>
<head>
    <title>Search Engine Demo</title>
</head>
<body>
    <script src="src/search-engine.js"></script>
    <script>
        const engine = new SearchEngine();
        // ... use engine
    </script>
</body>
</html>
```

#### Node.js Usage
```javascript
// src/index.js
const { SearchEngine, SearchDataType } = require('./search-engine');

const engine = new SearchEngine();
// ... implementation

module.exports = { engine };
```

### 6. Docker Deployment

```dockerfile
# Dockerfile
FROM node:16-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

EXPOSE 3000

CMD ["npm", "start"]
```

```bash
# Build and run
docker build -t search-engine .
docker run -p 3000:3000 search-engine
```

## Troubleshooting Guide

### Common Issues and Solutions

#### 1. No Search Results Found

**Symptoms:** Search returns empty results when content exists

**Diagnosis:**
```javascript
// Check if content is properly indexed
console.log('Tasks count:', engine.tasks.length);
console.log('First task:', engine.tasks[0]);

// Check search query
console.log('Search query:', query);
console.log('Lowercase query:', query.toLowerCase());
```

**Solutions:**
- Verify content was added via `updateIndex()`
- Check that search terms match indexed content
- Ensure case sensitivity handling

#### 2. Slow Search Performance

**Symptoms:** Search operations take too long

**Diagnosis:**
```javascript
console.time('search');
const results = engine.search('query');
console.timeEnd('search');

// Check index size
console.log('Total indexed items:', engine.getSearchAnalytics().totalIndexedItems);
```

**Solutions:**
- Implement result caching
- Add pagination for large result sets
- Consider inverted indexing for large datasets
- Optimize relevance scoring algorithm

#### 3. Incorrect Relevance Scoring

**Symptoms:** Results appear in wrong order

**Diagnosis:**
```javascript
const results = engine.search('query');
results.taskResults.forEach(result => {
    console.log(`${result.task.action}: ${result.score}`);
});
```

**Solutions:**
- Review scoring algorithm parameters
- Add debug logging to `calculateRelevanceScore`
- Consider custom scoring weights

#### 4. Date Filtering Issues

**Symptoms:** Date-based filtering not working correctly

**Diagnosis:**
```javascript
// Check date formats
console.log('Since date:', options.since);
console.log('Task created date:', task.createdAt);

// Verify date comparison
console.log('Date comparison:', task.createdAt >= options.since);
```

**Solutions:**
- Ensure consistent date formats
- Validate date objects before comparison
- Handle timezone differences

#### 5. Memory Leaks

**Symptoms:** Application memory usage grows over time

**Diagnosis:**
```javascript
// Monitor object references
console.log('Tasks array size:', engine.tasks.length);
console.log('Search cache size:', engine.searchCache?.size || 'N/A');

// Use performance monitoring
if (performance.memory) {
    console.log('Memory usage:', performance.memory);
}
```

**Solutions:**
- Implement result caching with expiration
- Use weak references where appropriate
- Regular cleanup of old search results

### Debugging Tools

```javascript
class DebugSearchEngine extends SearchEngine {
    search(query, options = {}) {
        console.log('=== Search Debug Info ===');
        console.log('Query:', query);
        console.log('Options:', options);
        console.log('Index stats:', this.getSearchAnalytics());
        
        const startTime = performance.now();
        const results = super.search(query, options);
        const endTime = performance.now();
        
        console.log('Search time:', (endTime - startTime).toFixed(2), 'ms');
        console.log('Results count:', results.totalResults());
        
        return results;
    }
    
    updateIndex(content) {
        console.log('Updating index with:', Object.keys(content));
        super.updateIndex(content);
        console.log('Index updated. New stats:', this.getSearchAnalytics());
    }
}
```

### Error Handling

```javascript
class RobustSearchEngine extends SearchEngine {
    search(query, options = {}) {
        try {
            // Validate inputs
            if (!query || typeof query !== 'string') {
                throw new Error('Invalid search query');
            }
            
            if (options.since && !(options.since instanceof Date)) {
                throw new Error('Invalid since date');
            }
            
            return super.search(query, options);
        } catch (error) {
            console.error('Search error:', error.message);
            return new SearchResults(); // Return empty results on error
        }
    }
}
```

This comprehensive documentation provides a complete understanding of the SearchEngine implementation, covering all aspects from basic usage to advanced deployment and troubleshooting scenarios.