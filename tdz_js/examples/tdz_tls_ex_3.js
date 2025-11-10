import { TagManager, TagSearchEngine, Tag, TagUpdate, TagSortBy, TagSearchQuery } from '../todozi/tags.js';

async function demonstrateAdvancedTagManagement() {
    // Initialize tag manager
    const tagManager = TagManager.new();
    const searchEngine = TagSearchEngine.new(tagManager);
    
    // Create a set of related tags
    const webDevTags = [
        { name: 'javascript', description: 'JavaScript programming language', category: 'programming' },
        { name: 'react', description: 'React UI framework', category: 'frontend' },
        { name: 'nodejs', description: 'Node.js runtime', category: 'backend' },
        { name: 'typescript', description: 'TypeScript superset of JavaScript', category: 'programming' },
        { name: 'frontend', description: 'Frontend development', category: 'development' }
    ];
    
    const databaseTags = [
        { name: 'postgresql', description: 'PostgreSQL database', category: 'database' },
        { name: 'mongodb', description: 'MongoDB NoSQL database', category: 'database' },
        { name: 'sql', description: 'SQL query language', category: 'database' },
        { name: 'nosql', description: 'NoSQL databases', category: 'database' }
    ];
    
    // Create all tags
    console.log('Creating tags...');
    const createdTagIds = [];
    
    for (const tagData of [...webDevTags, ...databaseTags]) {
        const tag = new Tag(tagData);
        const tagId = await tagManager.createTag(tag);
        createdTagIds.push(tagId);
        console.log(`✓ Created tag: ${tag.name}`);
    }
    
    // Establish relationships between related tags
    console.log('\nEstablishing tag relationships...');
    const relationships = [
        ['javascript', 'typescript'],
        ['javascript', 'react'],
        ['nodejs', 'javascript'],
        ['postgresql', 'sql'],
        ['mongodb', 'nosql']
    ];
    
    for (const [tag1, tag2] of relationships) {
        const tag1Obj = tagManager.getTagByName(tag1);
        const tag2Obj = tagManager.getTagByName(tag2);
        if (tag1Obj && tag2Obj) {
            await tagManager.addTagRelationship(tag1Obj.id, tag2Obj.id);
            console.log(`✓ Related: ${tag1} <-> ${tag2}`);
        }
    }
    
    // Increment usage for some tags to demonstrate statistics
    await tagManager.incrementTagUsage('javascript');
    await tagManager.incrementTagUsage('javascript');
    await tagManager.incrementTagUsage('react');
    await tagManager.incrementTagUsage('postgresql');
    
    // Demo 1: Basic search functionality
    console.log('\n=== DEMO 1: Basic Tag Search ===');
    const searchResults = tagManager.searchTags('script');
    console.log(`Search results for 'script':`);
    searchResults.forEach(tag => {
        console.log(`  - ${tag.name} (${tag.category}): ${tag.description}`);
    });
    
    // Demo 2: Advanced search with filters
    console.log('\n=== DEMO 2: Advanced Search ===');
    const query = TagSearchQuery.default()
        .category('database')
        .min_usage(1)
        .sort_by(TagSortBy.Usage)
        .limit(5);
    
    const advancedResults = searchEngine.advancedSearch(query);
    console.log('Database tags with usage >= 1, sorted by usage:');
    advancedResults.forEach(tag => {
        console.log(`  - ${tag.name}: ${tag.usage_count} uses`);
    });
    
    // Demo 3: Fuzzy search for typos
    console.log('\n=== DEMO 3: Fuzzy Search ===');
    const fuzzyResults = searchEngine.fuzzySearch('javascrpt', 2); // max distance = 2
    console.log('Fuzzy search for "javascrpt" (likely meant "javascript"):');
    fuzzyResults.forEach(([tag, distance]) => {
        console.log(`  - ${tag.name} (distance: ${distance})`);
    });
    
    // Demo 4: Get related tags
    console.log('\n=== DEMO 4: Related Tags ===');
    const jsTag = tagManager.getTagByName('javascript');
    if (jsTag) {
        const relatedTags = tagManager.getRelatedTags(jsTag.id);
        console.log(`Tags related to 'javascript':`);
        relatedTags.forEach(tag => {
            console.log(`  - ${tag.name}: ${tag.description}`);
        });
    }
    
    // Demo 5: Get suggestions based on current tags
    console.log('\n=== DEMO 5: Tag Suggestions ===');
    const currentTags = ['javascript', 'react'];
    const suggestions = searchEngine.getSuggestions(currentTags, 3);
    console.log(`Suggestions for tags [${currentTags.join(', ')}]:`);
    suggestions.forEach(suggestion => {
        console.log(`  - ${suggestion}`);
    });
    
    // Demo 6: Statistics and analytics
    console.log('\n=== DEMO 6: Tag Statistics ===');
    const stats = tagManager.getTagStatistics();
    console.log(`Total tags: ${stats.total_tags}`);
    console.log(`Total categories: ${stats.total_categories}`);
    console.log(`Total relationships: ${stats.total_relationships}`);
    console.log(`Average usage: ${stats.average_usage.toFixed(2)}`);
    console.log(`Relationships per tag: ${stats.relationshipsPerTag().toFixed(2)}`);
    
    // Demo 7: Most used and recent tags
    console.log('\n=== DEMO 7: Analytics ===');
    const mostUsed = tagManager.getMostUsedTags(3);
    console.log('Top 3 most used tags:');
    mostUsed.forEach(tag => {
        console.log(`  - ${tag.name}: ${tag.usage_count} uses`);
    });
    
    const recentTags = tagManager.getRecentTags(3);
    console.log('\nTop 3 most recent tags:');
    recentTags.forEach(tag => {
        console.log(`  - ${tag.name} (created: ${tag.created_at.toLocaleDateString()})`);
    });
    
    // Demo 8: Update and delete operations
    console.log('\n=== DEMO 8: Tag Management Operations ===');
    const reactTag = tagManager.getTagByName('react');
    if (reactTag) {
        // Update the tag
        const update = TagUpdate.new()
            .description('React JavaScript library for building UIs')
            .color('#61DAFB');
        
        await tagManager.updateTag(reactTag.id, update);
        console.log(`✓ Updated 'react' tag with new description and color`);
        
        // Show updated tag
        const updatedTag = tagManager.getTag(reactTag.id);
        console.log(`Updated: ${updatedTag.name} - ${updatedTag.description} (${updatedTag.color})`);
    }
    
    // Demo 9: Bulk operations
    console.log('\n=== DEMO 9: Bulk Operations ===');
    const newTagNames = ['vue', 'angular', 'svelte'];
    const bulkIds = await tagManager.bulkCreateTags(newTagNames, 'frontend');
    console.log(`Bulk created ${bulkIds.length} frontend tags`);
    
    // Demo 10: Category management
    console.log('\n=== DEMO 10: Category Management ===');
    const categories = tagManager.getAllCategories();
    console.log('All categories:');
    categories.forEach(category => {
        const categoryTags = tagManager.getTagsByCategory(category);
        console.log(`  - ${category}: ${categoryTags.length} tags`);
        categoryTags.slice(0, 3).forEach(tag => {
            console.log(`    * ${tag.name}`);
        });
        if (categoryTags.length > 3) {
            console.log(`    ... and ${categoryTags.length - 3} more`);
        }
    });
    
    return {
        totalTags: createdTagIds.length,
        relationships: relationships.length,
        statistics: stats
    };
}

// Error handling wrapper
async function runDemo() {
    try {
        const results = await demonstrateAdvancedTagManagement();
        console.log('\n✅ Demo completed successfully!');
        console.log(`Processed ${results.totalTags} tags with ${results.relationships} relationships`);
    } catch (error) {
        console.error('❌ Demo failed:', error.message);
        if (error.type === 'ValidationError') {
            console.error('Validation error details:', error.details);
        }
    }
}

// Run the demo if this file is executed directly
runDemo();

export {
    demonstrateAdvancedTagManagement,
    runDemo
};

/*
# Example 3: Advanced Tag Management with Fuzzy Search

This example demonstrates how to use the TagManager and TagSearchEngine classes to implement sophisticated tag management with fuzzy search capabilities, auto-suggestions, and relationship tracking.

## Key Features Demonstrated:

1. **Tag Creation**: Creating multiple tags with metadata
2. **Relationship Management**: Establishing relationships between related tags
3. **Search Capabilities**:
   - Basic text search
   - Advanced search with filters
   - Fuzzy search for typo tolerance
4. **Tag Suggestions**: Getting suggestions based on existing tag relationships
5. **Analytics**: Usage statistics, most used tags, recent tags
6. **CRUD Operations**: Update and delete tag operations
7. **Bulk Operations**: Creating multiple tags at once
8. **Category Management**: Organizing tags by categories

## Usage Example:

This example shows how the tag management system can be used to build a sophisticated tagging system with features like:
- Auto-completion suggestions
- Fuzzy matching for typos
- Relationship tracking between related concepts
- Category-based organization
- Usage analytics

The system is particularly useful for applications like:
- Content management systems
- Task management tools
- Knowledge bases
- Documentation platforms
- Code repositories
*/