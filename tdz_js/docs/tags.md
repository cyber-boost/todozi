# Todozi Tag Management System Documentation

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

The Todozi Tag Management System is a comprehensive JavaScript library for managing tags, categories, and relationships in a tagging system. It provides features for creating, updating, deleting, searching, and analyzing tags with support for categories, relationships, and usage tracking.

### Key Features
- Tag creation, update, and deletion
- Category management
- Tag relationships
- Advanced search capabilities
- Fuzzy search and suggestions
- Usage tracking and statistics
- Bulk operations

## Architecture

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Tag Management System                          │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────────────────┐ │
│  │   TagManager     │  │ TagSearchEngine  │  │     TagSearchQuery         │ │
│  │                  │  │                  │  │                            │ │
│  │ - tags           │  │ - tagManager     │  │ - name_contains            │ │
│  │ - tag_relations  │  │                  │  │ - description_contains     │ │
│  │ - category_tags  │  │                  │  │ - category                 │ │
│  │                  │  │                  │  │ - color                    │ │
│  │ + createTag()    │  │ + advancedSearch()│  │ - min_usage                │ │
│  │ + updateTag()    │  │ + fuzzySearch()   │  │ - max_usage                │ │
│  │ + deleteTag()    │  │ + getSuggestions()│  │ - sort_by                  │ │
│  │ + ...            │  │                  │  │ - limit                    │ │
│  └──────────────────┘  └──────────────────┘  └────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────────────────┐ │
│  │      Tag         │  │   TagUpdate      │  │    TagStatistics           │ │
│  │                  │  │                  │  │                            │ │
│  │ - id             │  │ - name           │  │ - total_tags               │ │
│  │ - name           │  │ - description    │  │ - total_categories         │ │
│  │ - description    │  │ - color          │  │ - total_relationships      │ │
│  │ - color          │  │ - category       │  │ - average_usage            │ │
│  │ - category       │  │                  │  │                            │ │
│  │ - usage_count    │  │ + name()         │  │ + relationshipsPerTag()    │ │
│  │ - created_at     │  │ + description()  │  │                            │ │
│  │ - updated_at     │  │ + color()        │  │                            │ │
│  │                  │  │ + category()     │  │                            │ │
│  └──────────────────┘  └──────────────────┘  └────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────────────────────┐ │
│  │  TodoziError     │  │ ValidationError  │  │    TagSortBy               │ │
│  │                  │  │                  │  │                            │ │
│  │ - type           │  │ - type           │  │ - Name                     │ │
│  │ - message        │  │ - message        │  │ - Usage                    │ │
│  │                  │  │                  │  │ - Created                  │ │
│  └──────────────────┘  └──────────────────┘  │ - Updated                  │ │
│                                               └────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Classes and Methods

### TodoziError
Base error class for the Todozi system.

#### Constructor
```javascript
new TodoziError(type, message)
```

**Parameters:**
- `type` (string): Error type identifier
- `message` (string): Error message

### ValidationError
Specific error class for validation failures.

#### Constructor
```javascript
new ValidationError(message)
```

**Parameters:**
- `message` (string): Validation error message

### Tag
Represents a tag in the system.

#### Constructor
```javascript
new Tag({
    id = '',
    name,
    description = null,
    color = null,
    category = null,
    usage_count = 0,
    created_at = new Date(),
    updated_at = new Date()
} = {})
```

**Parameters:**
- `id` (string): Unique identifier for the tag
- `name` (string): Tag name (required)
- `description` (string|null): Tag description
- `color` (string|null): Tag color
- `category` (string|null): Tag category
- `usage_count` (number): Number of times tag has been used
- `created_at` (Date): Creation timestamp
- `updated_at` (Date): Last update timestamp

### TagUpdate
Builder class for tag updates.

#### Constructor
```javascript
new TagUpdate()
```

#### Static Methods
##### new()
Creates a new TagUpdate instance.

```javascript
TagUpdate.new()
```

**Returns:** TagUpdate instance

#### Methods
##### name(name)
Sets the name for update.

```javascript
tagUpdate.name(name)
```

**Parameters:**
- `name` (string): New tag name

**Returns:** TagUpdate (for chaining)

##### description(description)
Sets the description for update.

```javascript
tagUpdate.description(description)
```

**Parameters:**
- `description` (string): New tag description

**Returns:** TagUpdate (for chaining)

##### color(color)
Sets the color for update.

```javascript
tagUpdate.color(color)
```

**Parameters:**
- `color` (string): New tag color

**Returns:** TagUpdate (for chaining)

##### category(category)
Sets the category for update.

```javascript
tagUpdate.category(category)
```

**Parameters:**
- `category` (string): New tag category

**Returns:** TagUpdate (for chaining)

### TagStatistics
Represents statistics about the tag system.

#### Constructor
```javascript
new TagStatistics({
    total_tags = 0,
    total_categories = 0,
    total_relationships = 0,
    average_usage = 0.0
} = {})
```

**Parameters:**
- `total_tags` (number): Total number of tags
- `total_categories` (number): Total number of categories
- `total_relationships` (number): Total number of tag relationships
- `average_usage` (number): Average tag usage count

#### Methods
##### relationshipsPerTag()
Calculates the average number of relationships per tag.

```javascript
tagStatistics.relationshipsPerTag()
```

**Returns:** number - Average relationships per tag

### TagManager
Main class for managing tags, categories, and relationships.

#### Constructor
```javascript
new TagManager()
```

#### Static Methods
##### new()
Creates a new TagManager instance.

```javascript
TagManager.new()
```

**Returns:** TagManager instance

#### Methods
##### createTag(tag)
Creates a new tag.

```javascript
async tagManager.createTag(tag)
```

**Parameters:**
- `tag` (Tag): Tag object to create

**Returns:** Promise<string> - ID of created tag

**Throws:** ValidationError if tag is invalid

##### getTag(tagId)
Retrieves a tag by ID.

```javascript
tagManager.getTag(tagId)
```

**Parameters:**
- `tagId` (string): Tag ID

**Returns:** Tag|undefined - Tag object or undefined if not found

##### getTagByName(name)
Retrieves a tag by name.

```javascript
tagManager.getTagByName(name)
```

**Parameters:**
- `name` (string): Tag name

**Returns:** Tag|undefined - Tag object or undefined if not found

##### getAllTags()
Retrieves all tags.

```javascript
tagManager.getAllTags()
```

**Returns:** Tag[] - Array of all tags

##### updateTag(tagId, updates)
Updates a tag with provided updates.

```javascript
async tagManager.updateTag(tagId, updates)
```

**Parameters:**
- `tagId` (string): ID of tag to update
- `updates` (TagUpdate): Updates to apply

**Returns:** Promise<void>

**Throws:** ValidationError if tag not found

##### deleteTag(tagId)
Deletes a tag.

```javascript
async tagManager.deleteTag(tagId)
```

**Parameters:**
- `tagId` (string): ID of tag to delete

**Returns:** Promise<void>

**Throws:** ValidationError if tag not found

##### addTagRelationship(tagId, relatedTagId)
Adds a relationship between two tags.

```javascript
async tagManager.addTagRelationship(tagId, relatedTagId)
```

**Parameters:**
- `tagId` (string): First tag ID
- `relatedTagId` (string): Second tag ID

**Returns:** Promise<void>

**Throws:** ValidationError if either tag not found

##### getRelatedTags(tagId)
Retrieves tags related to a given tag.

```javascript
tagManager.getRelatedTags(tagId)
```

**Parameters:**
- `tagId` (string): Tag ID

**Returns:** Tag[] - Array of related tags

##### searchTags(query)
Searches tags by name or description.

```javascript
tagManager.searchTags(query)
```

**Parameters:**
- `query` (string): Search query

**Returns:** Tag[] - Array of matching tags

##### getTagsByCategory(category)
Retrieves tags in a specific category.

```javascript
tagManager.getTagsByCategory(category)
```

**Parameters:**
- `category` (string): Category name

**Returns:** Tag[] - Array of tags in category

##### getAllCategories()
Retrieves all categories.

```javascript
tagManager.getAllCategories()
```

**Returns:** string[] - Array of category names

##### incrementTagUsage(tagName)
Increments usage count for a tag.

```javascript
async tagManager.incrementTagUsage(tagName)
```

**Parameters:**
- `tagName` (string): Name of tag to increment

**Returns:** Promise<void>

##### getMostUsedTags(limit)
Retrieves most used tags.

```javascript
tagManager.getMostUsedTags(limit)
```

**Parameters:**
- `limit` (number): Maximum number of tags to return

**Returns:** Tag[] - Array of most used tags

##### getRecentTags(limit)
Retrieves most recently created tags.

```javascript
tagManager.getRecentTags(limit)
```

**Parameters:**
- `limit` (number): Maximum number of tags to return

**Returns:** Tag[] - Array of recent tags

##### getTagStatistics()
Retrieves tag system statistics.

```javascript
tagManager.getTagStatistics()
```

**Returns:** TagStatistics - Tag system statistics

##### bulkCreateTags(tagNames, category)
Creates multiple tags in a category.

```javascript
async tagManager.bulkCreateTags(tagNames, category)
```

**Parameters:**
- `tagNames` (string[]): Array of tag names
- `category` (string): Category for tags

**Returns:** Promise<string[]> - Array of created tag IDs

##### mergeTags(primaryTagId, duplicateTagIds)
Merges duplicate tags into a primary tag.

```javascript
async tagManager.mergeTags(primaryTagId, duplicateTagIds)
```

**Parameters:**
- `primaryTagId` (string): ID of primary tag
- `duplicateTagIds` (string[]): Array of duplicate tag IDs

**Returns:** Promise<void>

**Throws:** ValidationError if primary tag not found

### TagSearchQuery
Configuration object for advanced searches.

#### Constructor
```javascript
new TagSearchQuery({
    name_contains = null,
    description_contains = null,
    category = null,
    color = null,
    min_usage = null,
    max_usage = null,
    sort_by = TagSortBy.Name,
    limit = null
} = {})
```

**Parameters:**
- `name_contains` (string|null): Filter by name substring
- `description_contains` (string|null): Filter by description substring
- `category` (string|null): Filter by category
- `color` (string|null): Filter by color
- `min_usage` (number|null): Minimum usage count
- `max_usage` (number|null): Maximum usage count
- `sort_by` (TagSortBy): Sort order
- `limit` (number|null): Maximum results

#### Static Methods
##### default()
Creates a default TagSearchQuery.

```javascript
TagSearchQuery.default()
```

**Returns:** TagSearchQuery instance

### TagSortBy
Enumeration of sort options.

**Values:**
- `Name`: Sort by tag name
- `Usage`: Sort by usage count
- `Created`: Sort by creation date
- `Updated`: Sort by update date

### TagSearchEngine
Advanced search engine for tags.

#### Constructor
```javascript
new TagSearchEngine(tagManager)
```

**Parameters:**
- `tagManager` (TagManager): Tag manager instance

#### Static Methods
##### new(tagManager)
Creates a new TagSearchEngine.

```javascript
TagSearchEngine.new(tagManager)
```

**Parameters:**
- `tagManager` (TagManager): Tag manager instance

**Returns:** TagSearchEngine instance

#### Methods
##### advancedSearch(query)
Performs advanced search with filters.

```javascript
tagSearchEngine.advancedSearch(query)
```

**Parameters:**
- `query` (TagSearchQuery): Search configuration

**Returns:** Tag[] - Array of matching tags

##### fuzzySearch(query, maxDistance)
Performs fuzzy search using Levenshtein distance.

```javascript
tagSearchEngine.fuzzySearch(query, maxDistance)
```

**Parameters:**
- `query` (string): Search query
- `maxDistance` (number): Maximum edit distance

**Returns:** [Tag, number][] - Array of [tag, distance] pairs

##### getSuggestions(currentTags, limit)
Gets tag suggestions based on current tags.

```javascript
tagSearchEngine.getSuggestions(currentTags, limit)
```

**Parameters:**
- `currentTags` (string[]): Array of current tag names
- `limit` (number): Maximum suggestions to return

**Returns:** string[] - Array of suggested tag names

### levenshteinDistance(s1, s2)
Calculates Levenshtein distance between two strings.

```javascript
levenshteinDistance(s1, s2)
```

**Parameters:**
- `s1` (string): First string
- `s2` (string): Second string

**Returns:** number - Levenshtein distance

## Usage Examples

### Basic Tag Management

```javascript
const { TagManager, Tag } = require('./todozi');

// Create a tag manager
const tagManager = TagManager.new();

// Create a tag
const tag = new Tag({
    name: 'Work',
    description: 'Work-related tasks',
    color: '#FF0000',
    category: 'Productivity'
});

// Create the tag in the system
const tagId = await tagManager.createTag(tag);
console.log(`Created tag with ID: ${tagId}`);

// Retrieve the tag
const retrievedTag = tagManager.getTag(tagId);
console.log('Retrieved tag:', retrievedTag);

// Update the tag
const updates = {
    description: 'Work and professional tasks',
    color: '#00FF00'
};
await tagManager.updateTag(tagId, updates);

// Get all tags
const allTags = tagManager.getAllTags();
console.log('All tags:', allTags);

// Delete the tag
await tagManager.deleteTag(tagId);
```

### Tag Relationships

```javascript
const { TagManager, Tag } = require('./todozi');

const tagManager = TagManager.new();

// Create related tags
const tag1 = new Tag({ name: 'JavaScript' });
const tag2 = new Tag({ name: 'Programming' });
const tag3 = new Tag({ name: 'Web Development' });

const id1 = await tagManager.createTag(tag1);
const id2 = await tagManager.createTag(tag2);
const id3 = await tagManager.createTag(tag3);

// Add relationships
await tagManager.addTagRelationship(id1, id2); // JavaScript -> Programming
await tagManager.addTagRelationship(id1, id3); // JavaScript -> Web Development

// Get related tags
const relatedTags = tagManager.getRelatedTags(id1);
console.log('JavaScript is related to:', relatedTags.map(t => t.name));
```

### Advanced Search

```javascript
const { TagManager, TagSearchEngine, TagSearchQuery, TagSortBy } = require('./todozi');

const tagManager = TagManager.new();
const searchEngine = TagSearchEngine.new(tagManager);

// Create some sample tags
const tags = [
    new Tag({ name: 'JavaScript', category: 'Programming', usage_count: 15 }),
    new Tag({ name: 'Python', category: 'Programming', usage_count: 12 }),
    new Tag({ name: 'React', category: 'Frontend', usage_count: 20 }),
    new Tag({ name: 'Node.js', category: 'Backend', usage_count: 18 })
];

for (const tag of tags) {
    await tagManager.createTag(tag);
}

// Advanced search
const query = new TagSearchQuery({
    category: 'Programming',
    min_usage: 10,
    sort_by: TagSortBy.Usage,
    limit: 5
});

const results = searchEngine.advancedSearch(query);
console.log('Programming tags with usage >= 10:', results.map(t => t.name));

// Fuzzy search
const fuzzyResults = searchEngine.fuzzySearch('Javascrpit', 2); // Typo intentional
console.log('Fuzzy search results:', fuzzyResults.map(([tag, distance]) => 
    `${tag.name} (distance: ${distance})`));
```

### Tag Statistics

```javascript
const { TagManager, Tag } = require('./todozi');

const tagManager = TagManager.new();

// Create some tags with relationships and usage
const tag1 = new Tag({ name: 'Project A', category: 'Work', usage_count: 5 });
const tag2 = new Tag({ name: 'Project B', category: 'Work', usage_count: 3 });
const tag3 = new Tag({ name: 'Meeting', category: 'Work', usage_count: 8 });

const id1 = await tagManager.createTag(tag1);
const id2 = await tagManager.createTag(tag2);
const id3 = await tagManager.createTag(tag3);

// Add some relationships
await tagManager.addTagRelationship(id1, id2);
await tagManager.addTagRelationship(id1, id3);

// Get statistics
const stats = tagManager.getTagStatistics();
console.log('Tag Statistics:');
console.log(`Total tags: ${stats.total_tags}`);
console.log(`Total categories: ${stats.total_categories}`);
console.log(`Total relationships: ${stats.total_relationships}`);
console.log(`Average usage: ${stats.average_usage.toFixed(2)}`);
console.log(`Relationships per tag: ${stats.relationshipsPerTag().toFixed(2)}`);
```

### Bulk Operations and Merging

```javascript
const { TagManager } = require('./todozi');

const tagManager = TagManager.new();

// Bulk create tags
const tagNames = ['Urgent', 'Important', 'Low Priority'];
const category = 'Priority';
const createdIds = await tagManager.bulkCreateTags(tagNames, category);

console.log('Created tags:', createdIds);

// Increment usage for some tags
await tagManager.incrementTagUsage('Urgent');
await tagManager.incrementTagUsage('Urgent');
await tagManager.incrementTagUsage('Important');

// Get most used tags
const mostUsed = tagManager.getMostUsedTags(3);
console.log('Most used tags:', mostUsed.map(t => `${t.name} (${t.usage_count})`));

// Merge duplicate tags (example scenario)
// Assume we have duplicate tags that need to be merged
const primaryTag = new Tag({ name: 'Meeting' });
const duplicateTag1 = new Tag({ name: 'Meeting', usage_count: 3 });
const duplicateTag2 = new Tag({ name: 'Meeting', usage_count: 2 });

const primaryId = await tagManager.createTag(primaryTag);
const dupId1 = await tagManager.createTag(duplicateTag1);
const dupId2 = await tagManager.createTag(duplicateTag2);

// Merge duplicates into primary
await tagManager.mergeTags(primaryId, [dupId1, dupId2]);

const mergedTag = tagManager.getTag(primaryId);
console.log(`Merged tag usage count: ${mergedTag.usage_count}`);
```

## Design Patterns

### Builder Pattern
The `TagUpdate` class implements the Builder pattern for constructing tag update operations:

```javascript
const updates = TagUpdate.new()
    .name('New Name')
    .description('New Description')
    .color('#FF0000')
    .category('New Category');
```

### Factory Pattern
The `new()` static methods in `TagManager` and `TagSearchEngine` implement the Factory pattern:

```javascript
const tagManager = TagManager.new();
const searchEngine = TagSearchEngine.new(tagManager);
```

### Strategy Pattern
The search functionality uses the Strategy pattern with different search algorithms:

```javascript
// Exact match search
tagManager.searchTags('query');

// Advanced search with filters
searchEngine.advancedSearch(query);

// Fuzzy search with edit distance
searchEngine.fuzzySearch('query', maxDistance);
```

### Repository Pattern
The `TagManager` class acts as a repository for tag data, abstracting the underlying data structure:

```javascript
// Data access is abstracted through the manager
await tagManager.createTag(tag);
const tag = tagManager.getTag(tagId);
await tagManager.updateTag(tagId, updates);
await tagManager.deleteTag(tagId);
```

### Observer Pattern (Implicit)
The system implicitly supports observer-like behavior through usage tracking:

```javascript
// Usage is tracked and can be observed through statistics
await tagManager.incrementTagUsage('TagName');
const stats = tagManager.getTagStatistics();
```

## Performance Analysis

### Time Complexity

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| createTag | O(1) | Average case |
| getTag | O(1) | Hash map lookup |
| updateTag | O(1) | Average case |
| deleteTag | O(n) | Where n is number of relationships |
| addTagRelationship | O(1) | Average case |
| getRelatedTags | O(k) | Where k is number of related tags |
| searchTags | O(n*m) | Where n is number of tags, m is query length |
| advancedSearch | O(n*log(n)) | Includes sorting |
| fuzzySearch | O(n*m²) | Where m is average tag name length |
| getSuggestions | O(n*k) | Where n is current tags, k is related tags |

### Space Complexity

| Component | Space Complexity | Notes |
|-----------|------------------|-------|
| tags | O(n) | Where n is number of tags |
| tag_relationships | O(r) | Where r is number of relationships |
| category_tags | O(c + n) | Where c is categories, n is tags |
| TagSearchEngine | O(1) | References existing data |

### Performance Optimization Recommendations

1. **Indexing**: For large datasets, consider indexing frequently searched fields
2. **Caching**: Cache search results for frequently accessed queries
3. **Batch Operations**: Use bulk operations for multiple tag creations
4. **Pagination**: Implement pagination for large result sets
5. **Lazy Loading**: Load relationships only when needed

### Memory Usage

The system uses JavaScript Maps for efficient key-value storage:
- Tags are stored by ID for O(1) lookup
- Relationships are stored as arrays of IDs
- Categories map to arrays of tag IDs

Memory usage scales linearly with the number of tags and relationships.

## Security Considerations

### Input Validation

The system implements validation through custom error types:

```javascript
class ValidationError extends TodoziError {
    constructor(message) {
        super('ValidationError', message);
    }
}
```

### Data Sanitization

While the current implementation doesn't include explicit sanitization, it's recommended to:

1. **Validate tag names and descriptions** for length and content
2. **Sanitize user input** before creating tags
3. **Validate category names** to prevent injection attacks

### Access Control

The system doesn't implement access control by default. For production use:

1. **Implement user authentication** before tag operations
2. **Add permission checks** for tag creation/deletion
3. **Implement tag ownership** for multi-user scenarios

### Error Handling

Proper error handling prevents information leakage:

```javascript
try {
    await tagManager.updateTag(tagId, updates);
} catch (error) {
    if (error instanceof ValidationError) {
        // Handle validation error
        console.error('Validation failed:', error.message);
    } else {
        // Handle other errors
        console.error('Operation failed');
    }
}
```

### Rate Limiting

For web applications, implement rate limiting on tag operations to prevent abuse.

## Testing Strategies

### Unit Testing

```javascript
const { TagManager, Tag, TagUpdate, ValidationError } = require('./todozi');

describe('TagManager', () => {
    let tagManager;
    let tagId;

    beforeEach(() => {
        tagManager = TagManager.new();
    });

    test('should create a tag', async () => {
        const tag = new Tag({ name: 'Test Tag' });
        tagId = await tagManager.createTag(tag);
        expect(tagId).toBeDefined();
        expect(tagManager.getTag(tagId)).toBeDefined();
    });

    test('should update a tag', async () => {
        const tag = new Tag({ name: 'Original' });
        tagId = await tagManager.createTag(tag);
        
        const updates = TagUpdate.new().name('Updated');
        await tagManager.updateTag(tagId, updates);
        
        const updatedTag = tagManager.getTag(tagId);
        expect(updatedTag.name).toBe('Updated');
    });

    test('should throw error for non-existent tag', async () => {
        const updates = TagUpdate.new().name('Test');
        await expect(tagManager.updateTag('non-existent', updates))
            .rejects.toThrow(ValidationError);
    });
});
```

### Integration Testing

```javascript
describe('Tag Relationships', () => {
    test('should manage tag relationships correctly', async () => {
        const tagManager = TagManager.new();
        
        const tag1 = new Tag({ name: 'Tag1' });
        const tag2 = new Tag({ name: 'Tag2' });
        
        const id1 = await tagManager.createTag(tag1);
        const id2 = await tagManager.createTag(tag2);
        
        await tagManager.addTagRelationship(id1, id2);
        
        const related = tagManager.getRelatedTags(id1);
        expect(related).toHaveLength(1);
        expect(related[0].id).toBe(id2);
    });
});
```

### Performance Testing

```javascript
describe('Performance', () => {
    test('should handle large number of tags', async () => {
        const tagManager = TagManager.new();
        const startTime = Date.now();
        
        // Create 1000 tags
        for (let i = 0; i < 1000; i++) {
            const tag = new Tag({ name: `Tag${i}` });
            await tagManager.createTag(tag);
        }
        
        const endTime = Date.now();
        const duration = endTime - startTime;
        
        expect(duration).toBeLessThan(5000); // Should complete in under 5 seconds
    });
});
```

### Fuzzy Search Testing

```javascript
describe('Fuzzy Search', () => {
    test('should find similar tags with typos', () => {
        const tagManager = TagManager.new();
        const searchEngine = TagSearchEngine.new(tagManager);
        
        const tag = new Tag({ name: 'JavaScript' });
        tagManager.createTag(tag);
        
        const results = searchEngine.fuzzySearch('Javascrpit', 2);
        expect(results).toHaveLength(1);
        expect(results[0][0].name).toBe('JavaScript');
    });
});
```

## Deployment Instructions

### Prerequisites

1. Node.js version 12 or higher
2. npm or yarn package manager

### Installation

```bash
# Install dependencies
npm install uuid

# Or with yarn
yarn add uuid
```

### Basic Setup

```javascript
// index.js
const { TagManager, Tag } = require('./todozi');

async function main() {
    const tagManager = TagManager.new();
    
    // Your tag management logic here
    const tag = new Tag({ name: 'Example' });
    const tagId = await tagManager.createTag(tag);
    
    console.log('Tag created with ID:', tagId);
}

main().catch(console.error);
```

### Web Application Integration

```javascript
// Express.js example
const express = require('express');
const { TagManager, Tag } = require('./todozi');

const app = express();
const tagManager = TagManager.new();

app.use(express.json());

app.post('/tags', async (req, res) => {
    try {
        const tag = new Tag(req.body);
        const tagId = await tagManager.createTag(tag);
        res.status(201).json({ id: tagId });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

app.get('/tags', (req, res) => {
    const tags = tagManager.getAllTags();
    res.json(tags);
});

app.listen(3000, () => {
    console.log('Server running on port 3000');
});
```

### Database Integration

For persistent storage, integrate with a database:

```javascript
// Example with a simple in-memory persistence layer
class PersistentTagManager {
    constructor(storage) {
        this.storage = storage; // Could be a database adapter
        this.tagManager = TagManager.new();
        this.loadFromStorage();
    }
    
    async loadFromStorage() {
        // Load tags from persistent storage
        const storedTags = await this.storage.getAllTags();
        for (const tagData of storedTags) {
            this.tagManager.tags.set(tagData.id, tagData);
        }
    }
    
    async saveToStorage() {
        // Save tags to persistent storage
        const tags = this.tagManager.getAllTags();
        await this.storage.saveTags(tags);
    }
    
    async createTag(tag) {
        const tagId = await this.tagManager.createTag(tag);
        await this.saveToStorage();
        return tagId;
    }
    
    // Implement other methods similarly...
}
```

## Troubleshooting Guide

### Common Issues

#### 1. Tag Not Found Errors

**Symptom:** `ValidationError: Tag {id} not found`

**Solution:** 
- Verify the tag ID exists using `tagManager.getTag(tagId)`
- Check if the tag was properly created
- Ensure you're using the correct tag manager instance

```javascript
// Debugging approach
const tag = tagManager.getTag(tagId);
if (!tag) {
    console.log(`Tag with ID ${tagId} not found`);
    // List all available tags
    console.log('Available tags:', tagManager.getAllTags().map(t => t.id));
}
```

#### 2. Relationship Management Issues

**Symptom:** Related tags not appearing correctly

**Solution:**
- Verify both tags exist before creating relationships
- Check that relationship methods are called correctly
- Ensure related tags haven't been deleted

```javascript
// Debugging relationships
console.log('Tag exists:', tagManager.getTag(tagId) !== undefined);
console.log('Related tag exists:', tagManager.getTag(relatedTagId) !== undefined);
console.log('Relationships:', tagManager.tag_relationships.get(tagId));
```

#### 3. Search Results Empty

**Symptom:** Search returns no results

**Solution:**
- Check search query syntax
- Verify tags exist with matching criteria
- Test with broader search terms

```javascript
// Debugging search
console.log('Total tags:', tagManager.tags.size);
console.log('Sample tags:', tagManager.getAllTags().slice(0, 5));

// Test basic search
const allTags = tagManager.getAllTags();
console.log('All tag names:', allTags.map(t => t.name));
```

### Performance Issues

#### 1. Slow Search Operations

**Symptoms:** Long response times for search operations

**Solutions:**
- Implement indexing for frequently searched fields
- Use pagination for large result sets
- Consider caching frequently accessed search results

```javascript
// Add simple caching
class CachedTagManager extends TagManager {
    constructor() {
        super();
        this.searchCache = new Map();
    }
    
    searchTags(query) {
        if (this.searchCache.has(query)) {
            return this.searchCache.get(query);
        }
        
        const results = super.searchTags(query);
        this.searchCache.set(query, results);
        return results;
    }
}
```

#### 2. Memory Usage Growth

**Symptoms:** Increasing memory consumption over time

**Solutions:**
- Monitor tag and relationship counts
- Implement cleanup for unused tags
- Use weak references where appropriate

```javascript
// Memory monitoring
function logMemoryUsage() {
    const usage = process.memoryUsage();
    console.log('Memory usage:');
    console.log(`RSS: ${Math.round(usage.rss / 1024 / 1024)} MB`);
    console.log(`Heap: ${Math.round(usage.heapUsed / 1024 / 1024)} MB`);
}

// Periodic cleanup example
async function cleanupUnusedTags(tagManager, daysThreshold = 30) {
    const cutoffDate = new Date();
    cutoffDate.setDate(cutoffDate.getDate() - daysThreshold);
    
    const tags = tagManager.getAllTags();
    for (const tag of tags) {
        if (tag.usage_count === 0 && tag.created_at < cutoffDate) {
            await tagManager.deleteTag(tag.id);
        }
    }
}
```

### Data Integrity Issues

#### 1. Inconsistent Category Counts

**Symptoms:** Statistics show incorrect category counts

**Solutions:**
- Verify category-tag mappings during updates
- Implement consistency checks
- Add validation for category operations

```javascript
// Add consistency check
function verifyCategoryIntegrity(tagManager) {
    const categoryCounts = new Map();
    
    // Count categories from tags
    for (const tag of tagManager.tags.values()) {
        if (tag.category) {
            const count = categoryCounts.get(tag.category) || 0;
            categoryCounts.set(tag.category, count + 1);
        }
    }
    
    // Compare with stored category mappings
    for (const [category, tagIds] of tagManager.category_tags.entries()) {
        const expectedCount = categoryCounts.get(category) || 0;
        if (tagIds.length !== expectedCount) {
            console.warn(`Category ${category} inconsistency: expected ${expectedCount}, got ${tagIds.length}`);
        }
    }
}
```

This comprehensive documentation covers all aspects of the Todozi Tag Management System, providing detailed explanations of classes, methods, usage examples, and best practices for deployment and troubleshooting.