// Example 2: Advanced Tag Management with Search and Relationships


import { TagManager, Tag, TagSearchEngine, TagSearchQuery, TagSortBy } from '../todozi/tags.js';

async function runAdvancedTagExample() {
    // Initialize tag manager
    const tagManager = TagManager.new();
    
    // Create sample tags with categories and colors
    const workTag = new Tag({
        name: 'work',
        description: 'Work-related tasks',
        category: 'project',
        color: '#FF0000'
    });
    
    const personalTag = new Tag({
        name: 'personal',
        description: 'Personal tasks',
        category: 'life',
        color: '#00FF00'
    });
    
    const urgentTag = new Tag({
        name: 'urgent',
        description: 'High priority items',
        category: 'priority',
        color: '#FF0000'
    });
    
    const meetingTag = new Tag({
        name: 'meeting',
        description: 'Meetings and discussions',
        category: 'work',
        color: '#0000FF'
    });
    
    // Add tags to manager
    const workTagId = await tagManager.createTag(workTag);
    const personalTagId = await tagManager.createTag(personalTag);
    const urgentTagId = await tagManager.createTag(urgentTag);
    const meetingTagId = await tagManager.createTag(meetingTag);
    
    // Create tag relationships
    await tagManager.addTagRelationship(workTagId, meetingTagId);
    await tagManager.addTagRelationship(urgentTagId, workTagId);
    await tagManager.addTagRelationship(urgentTagId, personalTagId);
    
    // Increment usage counts
    await tagManager.incrementTagUsage('work');
    await tagManager.incrementTagUsage('work');
    await tagManager.incrementTagUsage('urgent');
    await tagManager.incrementTagUsage('meeting');
    
    // Get related tags
    const relatedToWork = tagManager.getRelatedTags(workTagId);
    console.log('Tags related to "work":');
    relatedToWork.forEach(tag => console.log(`  - ${tag.name}`));
    
    // Search tags using TagSearchEngine
    const searchEngine = TagSearchEngine.new(tagManager);
    
    // Advanced search for work-related tags
    const workQuery = new TagSearchQuery({
        category: 'work',
        sort_by: TagSortBy.Name
    });
    
    const workResults = searchEngine.advancedSearch(workQuery);
    console.log('\nWork category tags:');
    workResults.forEach(tag => console.log(`  - ${tag.name}`));
    
    // Fuzzy search for tags similar to "metting" (typo)
    const fuzzyResults = searchEngine.fuzzySearch('metting', 2);
    console.log('\nFuzzy search for "metting":');
    fuzzyResults.forEach(([tag, distance]) => 
        console.log(`  - ${tag.name} (distance: ${distance})`));
    
    // Get tag suggestions based on current tags
    const suggestions = searchEngine.getSuggestions(['work', 'urgent'], 5);
    console.log('\nTag suggestions for ["work", "urgent"]:');
    suggestions.forEach(tagName => console.log(`  - ${tagName}`));
    
    // Get tag statistics
    const stats = tagManager.getTagStatistics();
    console.log('\nTag Statistics:');
    console.log(`  Total tags: ${stats.total_tags}`);
    console.log(`  Total categories: ${stats.total_categories}`);
    console.log(`  Total relationships: ${stats.total_relationships}`);
    console.log(`  Average usage: ${stats.average_usage.toFixed(2)}`);
    console.log(`  Relationships per tag: ${stats.relationshipsPerTag().toFixed(2)}`);
    
    // Get most used tags
    const mostUsed = tagManager.getMostUsedTags(3);
    console.log('\nMost used tags:');
    mostUsed.forEach(tag => console.log(`  - ${tag.name} (${tag.usage_count} uses)`));
    
    // Get recent tags
    const recentTags = tagManager.getRecentTags(3);
    console.log('\nRecently created tags:');
    recentTags.forEach(tag => console.log(`  - ${tag.name} (${tag.created_at.toISOString()})`));
    
    // Get all categories
    const categories = tagManager.getAllCategories();
    console.log('\nAll categories:');
    categories.forEach(category => console.log(`  - ${category}`));
    
    // Get tags by category
    const workTags = tagManager.getTagsByCategory('work');
    console.log('\nTags in "work" category:');
    workTags.forEach(tag => console.log(`  - ${tag.name}`));
}

// Run the example
runAdvancedTagExample().catch(console.error);

/*
This example demonstrates:

1. **Tag Creation**: Creating tags with names, descriptions, categories, and colors
2. **Tag Relationships**: Establishing relationships between tags (e.g., work is related to meetings)
3. **Tag Search Engine**: Using advanced search capabilities with filters and sorting
4. **Fuzzy Search**: Finding tags with similar names to handle typos
5. **Tag Suggestions**: Recommending tags based on existing tag usage
6. **Statistics**: Getting insights about tag usage and relationships
7. **Category Management**: Organizing tags into categories and retrieving them
8. **Usage Tracking**: Incrementing and analyzing tag usage counts

The example shows how to use the TagManager and TagSearchEngine classes to create a sophisticated tag management system with search, relationships, and analytics capabilities.
*/