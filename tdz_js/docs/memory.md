# Memory Manager Documentation

## Table of Contents
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Classes](#classes)
   - [MemoryManager](#memorymanager)
   - [MemoryUpdate](#memoryupdate)
   - [MemoryStatistics](#memorystatistics)
4. [Functions](#functions)
5. [Enums](#enums)
6. [Usage Examples](#usage-examples)
7. [Design Patterns](#design-patterns)
8. [Performance Analysis](#performance-analysis)
9. [Security Considerations](#security-considerations)
10. [Testing Strategies](#testing-strategies)
11. [Deployment Instructions](#deployment-instructions)
12. [Troubleshooting Guide](#troubleshooting-guide)

## Overview

The Memory Manager is a comprehensive JavaScript module designed to manage and organize memories with various attributes such as importance, term, type, and tags. It provides functionality for creating, retrieving, updating, and deleting memories, as well as advanced querying capabilities.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        MemoryManager Module                         │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────┐    ┌──────────────────────┐              │
│  │   MemoryManager      │    │   MemoryUpdate       │              │
│  │                      │    │                      │              │
│  │ - memories: Map      │    │ - moment             │              │
│  │ - memoryTags: Map    │    │ - meaning            │              │
│  │                      │    │ - reason             │              │
│  │ + createMemory()     │    │ - importance         │              │
│  │ + getMemory()        │    │ - term               │              │
│  │ + getAllMemories()   │    │ - tags               │              │
│  │ + updateMemory()     │    │                      │              │
│  │ + deleteMemory()     │    │ + new()              │              │
│  │ + searchMemories()   │    │ + withMoment()       │              │
│  │ + ... (various       │    │ + withMeaning()      │              │
│  │    query methods)    │    │ + withReason()       │              │
│  └──────────────────────┘    │ + withImportance()   │              │
│                              │ + withTerm()         │              │
│  ┌──────────────────────┐    │ + withTags()         │              │
│  │  MemoryStatistics    │    └──────────────────────┘              │
│  │                      │                                          │
│  │ - totalMemories      │    ┌──────────────────────┐              │
│  │ - shortTermMemories  │    │  parseMemoryFormat   │              │
│  │ - longTermMemories   │    └──────────────────────┘              │
│  │ - ...                │                                          │
│  │                      │    ┌──────────────────────┐              │
│  │ + shortTermPercent() │    │  MemoryImportance    │              │
│  │ + longTermPercent()  │    └──────────────────────┘              │
│  │ + criticalPercent()  │                                          │
│  └──────────────────────┘    ┌──────────────────────┐              │
│                              │    MemoryTerm        │              │
│                              └──────────────────────┘              │
└─────────────────────────────────────────────────────────────────────┘
```

## Classes

### MemoryManager

The main class responsible for managing memories and providing various query capabilities.

#### Constructor

```javascript
new MemoryManager()
```

Initializes a new MemoryManager instance with empty memories and memoryTags maps.

#### Methods

##### createMemory(memory)
Creates a new memory with a unique ID and timestamps.

**Parameters:**
- `memory` (Object): The memory object to create
  - `moment` (string): The moment description
  - `meaning` (string): The meaning of the memory
  - `reason` (string): The reason for the memory
  - `importance` (string): The importance level (from MemoryImportance)
  - `term` (string): The term type (from MemoryTerm)
  - `memoryType` (Object): The type of memory
  - `tags` (Array): Array of tags associated with the memory

**Returns:**
- `Promise<string>`: The unique ID of the created memory

**Example:**
```javascript
const memoryId = await memoryManager.createMemory({
  moment: "First day at new job",
  meaning: "Beginning of career journey",
  reason: "Started new position",
  importance: MemoryImportance.High,
  term: MemoryTerm.Long,
  memoryType: { type: "Standard" },
  tags: ["career", "new beginnings"]
});
```

##### getMemory(memoryId)
Retrieves a memory by its ID.

**Parameters:**
- `memoryId` (string): The unique ID of the memory

**Returns:**
- `Object|undefined`: The memory object or undefined if not found

##### getAllMemories()
Retrieves all memories.

**Returns:**
- `Array`: Array of all memory objects

##### updateMemory(memoryId, updates)
Updates an existing memory with new values.

**Parameters:**
- `memoryId` (string): The unique ID of the memory to update
- `updates` (Object): Object containing the fields to update
  - `moment` (string, optional): New moment description
  - `meaning` (string, optional): New meaning
  - `reason` (string, optional): New reason
  - `importance` (string, optional): New importance level
  - `term` (string, optional): New term type
  - `tags` (Array, optional): New tags array

**Returns:**
- `Promise<void>`

**Throws:**
- `Error`: If memory with given ID is not found

##### deleteMemory(memoryId)
Deletes a memory by its ID.

**Parameters:**
- `memoryId` (string): The unique ID of the memory to delete

**Returns:**
- `Promise<void>`

**Throws:**
- `Error`: If memory with given ID is not found

##### searchMemories(query)
Searches memories by a query string across multiple fields.

**Parameters:**
- `query` (string): The search query

**Returns:**
- `Array`: Array of matching memory objects

##### getMemoriesByImportance(importance)
Retrieves memories by importance level.

**Parameters:**
- `importance` (string): The importance level to filter by

**Returns:**
- `Array`: Array of memory objects with the specified importance

##### getMemoriesByTerm(term)
Retrieves memories by term type.

**Parameters:**
- `term` (string): The term type to filter by

**Returns:**
- `Array`: Array of memory objects with the specified term

##### getMemoriesByTag(tag)
Retrieves memories by tag.

**Parameters:**
- `tag` (string): The tag to filter by

**Returns:**
- `Array`: Array of memory objects with the specified tag

##### getRecentMemories(limit)
Retrieves the most recently created memories.

**Parameters:**
- `limit` (number): Maximum number of memories to return

**Returns:**
- `Array`: Array of recent memory objects

##### getCriticalMemories()
Retrieves memories with high or critical importance.

**Returns:**
- `Array`: Array of critical memory objects

##### getShortTermMemories()
Retrieves memories with short term.

**Returns:**
- `Array`: Array of short-term memory objects

##### getLongTermMemories()
Retrieves memories with long term.

**Returns:**
- `Array`: Array of long-term memory objects

##### getMemoriesByType(memoryType)
Retrieves memories by type.

**Parameters:**
- `memoryType` (Object): The memory type to filter by
  - `type` (string): The type of memory
  - `emotion` (string, optional): The emotion (for Emotional type)

**Returns:**
- `Array`: Array of memory objects with the specified type

##### getSecretMemories()
Retrieves memories of type "Secret".

**Returns:**
- `Array`: Array of secret memory objects

##### getHumanMemories()
Retrieves memories of type "Human".

**Returns:**
- `Array`: Array of human memory objects

##### getEmotionalMemories(emotion)
Retrieves memories of type "Emotional" with a specific emotion.

**Parameters:**
- `emotion` (string): The emotion to filter by

**Returns:**
- `Array`: Array of emotional memory objects with the specified emotion

##### getAllTags()
Retrieves all unique tags.

**Returns:**
- `Array`: Array of all unique tags

##### getTagStatistics()
Retrieves statistics about tag usage.

**Returns:**
- `Map`: Map with tags as keys and usage counts as values

##### getMemoryStatistics()
Retrieves comprehensive statistics about all memories.

**Returns:**
- `MemoryStatistics`: Object containing memory statistics

### MemoryUpdate

A builder class for creating memory update objects.

#### Constructor

```javascript
new MemoryUpdate()
```

Initializes a new MemoryUpdate instance with all fields undefined.

#### Static Methods

##### new()
Creates a new MemoryUpdate instance.

**Returns:**
- `MemoryUpdate`: New MemoryUpdate instance

#### Methods

##### withMoment(moment)
Sets the moment field.

**Parameters:**
- `moment` (string): The moment description

**Returns:**
- `MemoryUpdate`: The current instance for chaining

##### withMeaning(meaning)
Sets the meaning field.

**Parameters:**
- `meaning` (string): The meaning of the memory

**Returns:**
- `MemoryUpdate`: The current instance for chaining

##### withReason(reason)
Sets the reason field.

**Parameters:**
- `reason` (string): The reason for the memory

**Returns:**
- `MemoryUpdate`: The current instance for chaining

##### withImportance(importance)
Sets the importance field.

**Parameters:**
- `importance` (string): The importance level

**Returns:**
- `MemoryUpdate`: The current instance for chaining

##### withTerm(term)
Sets the term field.

**Parameters:**
- `term` (string): The term type

**Returns:**
- `MemoryUpdate`: The current instance for chaining

##### withTags(tags)
Sets the tags field.

**Parameters:**
- `tags` (Array): Array of tags

**Returns:**
- `MemoryUpdate`: The current instance for chaining

### MemoryStatistics

A class representing memory statistics.

#### Constructor

```javascript
new MemoryStatistics(data)
```

**Parameters:**
- `data` (Object): Object containing statistics data
  - `totalMemories` (number, optional): Total number of memories
  - `shortTermMemories` (number, optional): Number of short-term memories
  - `longTermMemories` (number, optional): Number of long-term memories
  - `criticalMemories` (number, optional): Number of critical memories
  - `uniqueTags` (number, optional): Number of unique tags
  - `secretMemories` (number, optional): Number of secret memories
  - `humanMemories` (number, optional): Number of human memories
  - `emotionalMemories` (number, optional): Number of emotional memories
  - `standardMemories` (number, optional): Number of standard memories

#### Methods

##### shortTermPercentage()
Calculates the percentage of short-term memories.

**Returns:**
- `number`: Percentage of short-term memories

##### longTermPercentage()
Calculates the percentage of long-term memories.

**Returns:**
- `number`: Percentage of long-term memories

##### criticalPercentage()
Calculates the percentage of critical memories.

**Returns:**
- `number`: Percentage of critical memories

## Functions

### parseMemoryFormat(memoryText, userId)
Parses a memory from a formatted text string.

**Parameters:**
- `memoryText` (string): The formatted memory text
- `userId` (string): The user ID to associate with the memory

**Returns:**
- `Object`: Parsed memory object

**Throws:**
- `Error`: If the memory format is invalid

### parseImportance(importanceStr)
Parses an importance string into a MemoryImportance value.

**Parameters:**
- `importanceStr` (string): The importance string to parse

**Returns:**
- `string`: MemoryImportance value

**Throws:**
- `Error`: If the importance string is invalid

### parseTerm(termStr)
Parses a term string into a MemoryTerm value.

**Parameters:**
- `termStr` (string): The term string to parse

**Returns:**
- `string`: MemoryTerm value

**Throws:**
- `Error`: If the term string is invalid

## Enums

### MemoryImportance
Enumeration of memory importance levels.

```javascript
const MemoryImportance = {
    Low: 'Low',
    Medium: 'Medium',
    High: 'High',
    Critical: 'Critical'
};
```

### MemoryTerm
Enumeration of memory term types.

```javascript
const MemoryTerm = {
    Short: 'Short',
    Long: 'Long'
};
```

## Usage Examples

### Basic Memory Management

```javascript
const { MemoryManager, MemoryImportance, MemoryTerm } = require('./memory-manager');

// Create a new memory manager
const memoryManager = new MemoryManager();

// Create a new memory
const memoryId = await memoryManager.createMemory({
  moment: "First day at new job",
  meaning: "Beginning of career journey",
  reason: "Started new position",
  importance: MemoryImportance.High,
  term: MemoryTerm.Long,
  memoryType: { type: "Standard" },
  tags: ["career", "new beginnings"]
});

// Retrieve the memory
const memory = memoryManager.getMemory(memoryId);
console.log(memory);

// Update the memory
const updates = new MemoryUpdate()
  .withMeaning("Exciting start to career journey")
  .withTags(["career", "new beginnings", "excitement"]);
await memoryManager.updateMemory(memoryId, updates);

// Search memories
const results = memoryManager.searchMemories("career");
console.log(results);

// Delete the memory
await memoryManager.deleteMemory(memoryId);
```

### Memory Statistics

```javascript
// Get memory statistics
const stats = memoryManager.getMemoryStatistics();
console.log(`Total memories: ${stats.totalMemories}`);
console.log(`Short-term percentage: ${stats.shortTermPercentage()}%`);
console.log(`Critical percentage: ${stats.criticalPercentage()}%`);

// Get tag statistics
const tagStats = memoryManager.getTagStatistics();
for (const [tag, count] of tagStats) {
  console.log(`${tag}: ${count}`);
}
```

### Memory Querying

```javascript
// Get memories by importance
const criticalMemories = memoryManager.getMemoriesByImportance(MemoryImportance.Critical);

// Get memories by term
const longTermMemories = memoryManager.getLongTermMemories();

// Get memories by tag
const careerMemories = memoryManager.getMemoriesByTag("career");

// Get recent memories
const recentMemories = memoryManager.getRecentMemories(5);

// Get emotional memories
const happyMemories = memoryManager.getEmotionalMemories("happy");
```

### Parsing Memory Format

```javascript
const { parseMemoryFormat } = require('./memory-manager');

const memoryText = "<memory>standard;First day at new job;Beginning of career journey;Started new position;high;long;career,new beginnings</memory>";
const userId = "user123";

try {
  const parsedMemory = parseMemoryFormat(memoryText, userId);
  console.log(parsedMemory);
} catch (error) {
  console.error("Failed to parse memory:", error.message);
}
```

## Design Patterns

### 1. Builder Pattern
The `MemoryUpdate` class implements the Builder pattern to provide a fluent interface for creating memory update objects.

### 2. Factory Pattern
The `MemoryUpdate.new()` method acts as a factory method for creating new instances.

### 3. Repository Pattern
The `MemoryManager` class acts as a repository for memory objects, providing CRUD operations and query capabilities.

### 4. Strategy Pattern
The various query methods in `MemoryManager` implement different strategies for retrieving memories based on different criteria.

### 5. Data Transfer Object (DTO)
The `MemoryStatistics` class serves as a DTO for transferring memory statistics data.

## Performance Analysis

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| createMemory | O(1) | Adding to Map is constant time |
| getMemory | O(1) | Map lookup is constant time |
| getAllMemories | O(n) | Where n is the number of memories |
| updateMemory | O(1) | Map lookup and update is constant time |
| deleteMemory | O(1) | Map deletion is constant time |
| searchMemories | O(n*m) | Where n is memories and m is average fields per memory |
| getMemoriesBy* | O(n) | Linear scan through all memories |
| getRecentMemories | O(n log n) | Due to sorting operation |

### Space Complexity

| Component | Complexity | Notes |
|-----------|------------|-------|
| memories Map | O(n) | Where n is the number of memories |
| memoryTags Map | O(n*t) | Where n is memories and t is average tags per memory |

### Optimization Recommendations

1. **Indexing**: For large datasets, consider implementing indexes for frequently queried fields like tags, importance, and term.

2. **Caching**: Cache frequently accessed statistics to avoid recalculating them.

3. **Pagination**: Implement pagination for large result sets to avoid memory issues.

4. **Database Integration**: For production use, consider integrating with a database for persistence and advanced querying capabilities.

## Security Considerations

### 1. Input Validation
- All input should be validated before processing
- The `parseMemoryFormat` function includes basic validation but should be enhanced for production use

### 2. Data Sanitization
- Memory content should be sanitized to prevent injection attacks
- Tags and other user-provided strings should be sanitized

### 3. Access Control
- The current implementation doesn't include access control
- In a real application, implement proper authentication and authorization

### 4. Error Handling
- Sensitive information should not be exposed in error messages
- Implement proper error logging without exposing internal details

### 5. Memory Limits
- Implement limits on memory size and count to prevent resource exhaustion
- Consider implementing quotas per user

## Testing Strategies

### Unit Tests

```javascript
// Example test structure
describe('MemoryManager', () => {
  let memoryManager;
  
  beforeEach(() => {
    memoryManager = new MemoryManager();
  });
  
  describe('createMemory', () => {
    it('should create a memory with a unique ID', async () => {
      const memory = {
        moment: 'Test moment',
        meaning: 'Test meaning',
        reason: 'Test reason',
        importance: MemoryImportance.Medium,
        term: MemoryTerm.Short,
        memoryType: { type: 'Standard' },
        tags: ['test']
      };
      
      const id = await memoryManager.createMemory(memory);
      expect(id).toBeDefined();
      expect(typeof id).toBe('string');
    });
  });
  
  // Additional tests for other methods...
});
```

### Integration Tests

```javascript
describe('MemoryManager Integration', () => {
  it('should handle full CRUD lifecycle', async () => {
    const memoryManager = new MemoryManager();
    
    // Create
    const memory = {
      moment: 'Integration test',
      meaning: 'Testing full lifecycle',
      reason: 'Ensure CRUD works',
      importance: MemoryImportance.High,
      term: MemoryTerm.Long,
      memoryType: { type: 'Standard' },
      tags: ['integration', 'test']
    };
    
    const id = await memoryManager.createMemory(memory);
    
    // Read
    const retrieved = memoryManager.getMemory(id);
    expect(retrieved).toBeDefined();
    expect(retrieved.moment).toBe(memory.moment);
    
    // Update
    const updates = new MemoryUpdate()
      .withMeaning('Updated meaning');
    await memoryManager.updateMemory(id, updates);
    
    const updated = memoryManager.getMemory(id);
    expect(updated.meaning).toBe('Updated meaning');
    
    // Delete
    await memoryManager.deleteMemory(id);
    const deleted = memoryManager.getMemory(id);
    expect(deleted).toBeUndefined();
  });
});
```

### Performance Tests

```javascript
describe('MemoryManager Performance', () => {
  it('should handle large numbers of memories efficiently', async () => {
    const memoryManager = new MemoryManager();
    const startTime = Date.now();
    
    // Create 10,000 memories
    for (let i = 0; i < 10000; i++) {
      await memoryManager.createMemory({
        moment: `Memory ${i}`,
        meaning: `Meaning ${i}`,
        reason: `Reason ${i}`,
        importance: MemoryImportance.Medium,
        term: MemoryTerm.Short,
        memoryType: { type: 'Standard' },
        tags: [`tag${i % 100}`]
      });
    }
    
    const creationTime = Date.now() - startTime;
    expect(creationTime).toBeLessThan(5000); // Should complete in under 5 seconds
    
    // Test search performance
    const searchStartTime = Date.now();
    const results = memoryManager.searchMemories('tag50');
    const searchTime = Date.now() - searchStartTime;
    
    expect(searchTime).toBeLessThan(1000); // Should complete in under 1 second
  });
});
```

## Deployment Instructions

### Prerequisites
- Node.js v12 or higher
- npm or yarn package manager

### Installation

1. Install dependencies:
```bash
npm install uuid
```

2. Copy the memory-manager.js file to your project

3. Import the module in your application:
```javascript
const { MemoryManager, MemoryImportance, MemoryTerm } = require('./memory-manager');
```

### Configuration

The MemoryManager doesn't require any special configuration for basic use. However, for production environments, consider:

1. **Persistence**: Implement database storage for memory persistence
2. **Caching**: Add caching layer for frequently accessed data
3. **Logging**: Implement proper logging for monitoring and debugging

### Environment Variables

No environment variables are required for basic operation.

### Deployment Checklist

- [ ] Install required dependencies
- [ ] Test memory operations in your environment
- [ ] Implement proper error handling
- [ ] Set up monitoring and logging
- [ ] Configure backup and recovery procedures
- [ ] Implement security measures
- [ ] Set up performance monitoring

## Troubleshooting Guide

### Common Issues

#### 1. "Memory not found" errors
**Symptoms**: `Error: Memory {id} not found`
**Causes**: 
- Attempting to access a memory that doesn't exist
- Using an incorrect memory ID
**Solutions**:
- Verify the memory ID is correct
- Check if the memory was properly created
- Use `getAllMemories()` to list all available memories

#### 2. Parsing errors with memory format
**Symptoms**: `Error: Missing <memory> start tag` or similar
**Causes**: 
- Incorrectly formatted memory text
- Missing required fields
**Solutions**:
- Ensure the memory text follows the correct format: `<memory>type;moment;meaning;reason;importance;term;tags</memory>`
- Check that all required fields are present

#### 3. Performance issues with large datasets
**Symptoms**: Slow response times for queries
**Causes**: 
- Large number of memories without proper indexing
- Complex search queries
**Solutions**:
- Implement database indexing
- Add pagination for large result sets
- Cache frequently accessed data

### Debugging Steps

1. **Check memory existence**:
```javascript
const allMemories = memoryManager.getAllMemories();
console.log('Total memories:', allMemories.length);
```

2. **Verify memory structure**:
```javascript
const memory = memoryManager.getMemory(memoryId);
console.log('Memory structure:', JSON.stringify(memory, null, 2));
```

3. **Test individual components**:
```javascript
// Test creation
try {
  const testId = await memoryManager.createMemory(testMemory);
  console.log('Creation successful:', testId);
} catch (error) {
  console.error('Creation failed:', error.message);
}
```

### Logging Best Practices

Implement comprehensive logging for troubleshooting:

```javascript
// Add logging to critical operations
class MemoryManager {
  async createMemory(memory) {
    console.log('Creating memory:', memory.moment);
    try {
      // ... creation logic
      console.log('Memory created successfully:', newMemory.id);
      return newMemory.id;
    } catch (error) {
      console.error('Memory creation failed:', error.message);
      throw error;
    }
  }
}
```

### Performance Monitoring

Implement performance monitoring for production environments:

```javascript
class MemoryManager {
  async createMemory(memory) {
    const startTime = Date.now();
    try {
      // ... creation logic
      const duration = Date.now() - startTime;
      console.log(`Memory creation took ${duration}ms`);
      return newMemory.id;
    } catch (error) {
      const duration = Date.now() - startTime;
      console.error(`Memory creation failed after ${duration}ms:`, error.message);
      throw error;
    }
  }
}
```

This comprehensive documentation covers all aspects of the Memory Manager module, providing detailed information about its architecture, usage, performance characteristics, and deployment considerations.