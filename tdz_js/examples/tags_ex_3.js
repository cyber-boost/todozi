
import { TagManager, TagSearchEngine, TagSearchQuery, TagSortBy } from './tags.js';

async function setupDevelopmentTags() {
    // Initialize the tag manager
    const tagManager = TagManager.new();
    const searchEngine = TagSearchEngine.new(tagManager);
    
    console.log('🏷️ Setting up Software Development Tag System...\n');
    
    // 1. Create technology-related tags
    const techTags = [
        { name: 'javascript', category: 'technology', color: '#f7df1e', description: 'JavaScript programming language' },
        { name: 'typescript', category: 'technology', color: '#3178c6', description: 'TypeScript superset of JavaScript' },
        { name: 'react', category: 'technology', color: '#61dafb', description: 'React UI library' },
        { name: 'nodejs', category: 'technology', color: '#339933', description: 'Node.js runtime environment' },
        { name: 'docker', category: 'technology', color: '#2496ed', description: 'Docker containerization' }
    ];
    
    const techTagIds = await tagManager.bulkCreateTags(techTags, 'technology');
    console.log(`✅ Created ${techTagIds.length} technology tags`);
    
    // 2. Create project phase tags
    const phaseTags = [
        { name: 'planning', category: 'phase', color: '#9c27b0', description: 'Initial planning phase' },
        { name: 'development', category: 'phase', color: '#2196f3', description: 'Active development phase' },
        { name: 'testing', category: 'phase', color: '#ff9800', description: 'Testing and QA phase' },
        { name: 'deployment', category: 'phase', color: '#4caf50', description: 'Deployment to production' }
    ];
    
    const phaseTagIds = await tagManager.bulkCreateTags(phaseTags, 'phase');
    console.log(`✅ Created ${phaseTagIds.length} phase tags`);
    
    // 3. Create priority tags
    const priorityTags = [
        { name: 'critical', category: 'priority', color: '#f44336', description: 'Critical priority issues' },
        { name: 'high', category: 'priority', color: '#ff5722', description: 'High priority tasks' },
        { name: 'medium', category: 'priority', color: '#ffc107', description: 'Medium priority tasks' },
        { name: 'low', category: 'priority', color: '#8bc34a', description: 'Low priority tasks' }
    ];
    
    const priorityTagIds = await tagManager.bulkCreateTags(priorityTags, 'priority');
    console.log(`✅ Created ${priorityTagIds.length} priority tags`);
    
    // 4. Create team-related tags
    const teamTags = [
        { name: 'frontend', category: 'team', color: '#e91e63', description: 'Frontend development team' },
        { name: 'backend', category: 'team', color: '#673ab7', description: 'Backend development team' },
        { name: 'devops', category: 'team', color: '#00bcd4', description: 'DevOps team' },
        { name: 'design', category: 'team', color: '#795548', description: 'UI/UX design team' }
    ];
    
    const teamTagIds = await tagManager.bulkCreateTags(teamTags, 'team');
    console.log(`✅ Created ${teamTagIds.length} team tags\n`);
    
    return { tagManager, searchEngine, techTagIds, phaseTagIds, priorityTagIds, teamTagIds };
}

async function establishTagRelationships(tagManager) {
    console.log('🔗 Establishing tag relationships...\n');
    
    // Get tag IDs by name
    const javascriptTag = tagManager.getTagByName('javascript');
    const typescriptTag = tagManager.getTagByName('typescript');
    const reactTag = tagManager.getTagByName('react');
    const nodejsTag = tagManager.getTagByName('nodejs');
    const dockerTag = tagManager.getTagByName('docker');
    
    // Create relationships between related technologies
    if (javascriptTag && typescriptTag) {
        await tagManager.addTagRelationship(javascriptTag.id, typescriptTag.id);
        await tagManager.addTagRelationship(typescriptTag.id, javascriptTag.id);
        console.log('🔗 JavaScript ↔ TypeScript');
    }
    
    if (javascriptTag && reactTag) {
        await tagManager.addTagRelationship(javascriptTag.id, reactTag.id);
        await tagManager.addTagRelationship(reactTag.id, javascriptTag.id);
        console.log('🔗 JavaScript ↔ React');
    }
    
    if (nodejsTag && dockerTag) {
        await tagManager.addTagRelationship(nodejsTag.id, dockerTag.id);
        await tagManager.addTagRelationship(dockerTag.id, nodejsTag.id);
        console.log('🔗 Node.js ↔ Docker');
    }
    
    console.log('\n');
}

async function demonstrateSearchCapabilities(searchEngine, tagManager) {
    console.log('🔍 Demonstrating search capabilities...\n');
    
    // 1. Basic search by name
    console.log('1️⃣ Searching for tags containing "script":');
    const scriptResults = tagManager.searchTags('script');
    scriptResults.forEach(tag => {
        console.log(`   - ${tag.name} (${tag.category})`);
    });
    
    // 2. Advanced search with multiple criteria
    console.log('\n2️⃣ Advanced search - All technology tags:');
    const techQuery = TagSearchQuery.default()
        .category('technology')
        .sort_by(TagSortBy.Name)
        .limit(10);
    const techResults = searchEngine.advancedSearch(techQuery);
    techResults.forEach(tag => {
        console.log(`   - ${tag.name}: ${tag.description || 'No description'}`);
    });
    
    // 3. Fuzzy search for misspellings
    console.log('\n3️⃣ Fuzzy search for "javasript" (misspelled):');
    const fuzzyResults = searchEngine.fuzzySearch('javasript', 2);
    fuzzyResults.forEach(([tag, distance]) => {
        console.log(`   - Found: ${tag.name} (distance: ${distance})`);
    });
    
    // 4. Search by category with usage threshold
    console.log('\n4️⃣ Priority tags used at least once:');
    const priorityQuery = TagSearchQuery.default()
        .category('priority')
        .min_usage(1)
        .sort_by(TagSortBy.Usage);
    const priorityResults = searchEngine.advancedSearch(priorityQuery);
    priorityResults.forEach(tag => {
        console.log(`   - ${tag.name} (used ${tag.usage_count} times)`);
    });
}

async function simulateTagUsage(tagManager) {
    console.log('\n📊 Simulating tag usage patterns...\n');
    
    // Simulate task tagging
    const usagePatterns = [
        ['javascript', 'frontend', 'development', 'high'],
        ['typescript', 'frontend', 'development', 'medium'],
        ['react', 'frontend', 'testing', 'critical'],
        ['nodejs', 'backend', 'deployment', 'high'],
        ['docker', 'devops', 'deployment', 'critical'],
        ['javascript', 'planning', 'low']
    ];
    
    for (const tags of usagePatterns) {
        for (const tagName of tags) {
            await tagManager.incrementTagUsage(tagName);
        }
    }
    
    console.log('📈 Tag usage statistics:');
    
    // Get most used tags
    const mostUsed = tagManager.getMostUsedTags(5);
    console.log('\nTop 5 most used tags:');
    mostUsed.forEach((tag, index) => {
        console.log(`${index + 1}. ${tag.name} - used ${tag.usage_count} times`);
    });
    
    // Get related tags for JavaScript
    const jsTag = tagManager.getTagByName('javascript');
    if (jsTag) {
        const relatedTags = tagManager.getRelatedTags(jsTag.id);
        console.log(`\nTags related to JavaScript:`);
        relatedTags.forEach(tag => {
            console.log(`   - ${tag.name} (${tag.category})`);
        });
    }
}

async function demonstrateTagManagement(tagManager) {
    console.log('\n⚙️ Demonstrating tag management features...\n');
    
    // 1. Update a tag
    console.log('1️⃣ Updating JavaScript tag description:');
    const jsTag = tagManager.getTagByName('javascript');
    if (jsTag) {
        await tagManager.updateTag(jsTag.id, {
            description: 'JavaScript programming language (updated with ES2024 features)'
        });
        const updatedTag = tagManager.getTag(jsTag.id);
        console.log(`   Updated description: "${updatedTag.description}"`);
    }
    
    // 2. Get all categories
    console.log('\n2️⃣ All tag categories:');
    const categories = tagManager.getAllCategories();
    categories.forEach(category => {
        const tagsInCategory = tagManager.getTagsByCategory(category);
        console.log(`   - ${category}: ${tagsInCategory.length} tags`);
    });
    
    // 3. Get system statistics
    console.log('\n3️⃣ System statistics:');
    const stats = tagManager.getTagStatistics();
    console.log(`   - Total tags: ${stats.total_tags}`);
    console.log(`   - Total categories: ${stats.total_categories}`);
    console.log(`   - Total relationships: ${stats.total_relationships}`);
    console.log(`   - Average usage per tag: ${stats.average_usage.toFixed(2)}`);
    console.log(`   - Relationships per tag: ${stats.relationshipsPerTag().toFixed(2)}`);
}

async function demonstrateSmartRecommendations(searchEngine, tagManager) {
    console.log('\n🤖 Demonstrating smart tag recommendations...\n');
    
    // Scenario: User is working on frontend tasks
    const currentTags = ['javascript', 'frontend'];
    
    // Get recommendations based on current tags
    const recommendations = searchEngine.getSuggestions(currentTags, 5);
    
    console.log(`Current tags: ${currentTags.join(', ')}`);
    console.log('Recommended related tags:');
    
    for (const recommendation of recommendations) {
        const tag = tagManager.getTagByName(recommendation);
        if (tag) {
            console.log(`   - ${tag.name} (${tag.category}): ${tag.description || 'No description'}`);
        }
    }
}

async function demonstrateTagCleanup(tagManager) {
    console.log('\n🧹 Demonstrating tag cleanup and merging...\n');
    
    // Create some duplicate/similar tags
    await tagManager.createTag({
        name: 'js',
        category: 'technology',
        usage_count: 3
    });
    
    await tagManager.createTag({
        name: 'jscript',
        category: 'technology', 
        usage_count: 1
    });
    
    console.log('Before merge:');
    const jsTags = tagManager.searchTags('js');
    jsTags.forEach(tag => {
        console.log(`   - ${tag.name} (usage: ${tag.usage_count})`);
    });
    
    // Find the primary JavaScript tag
    const primaryJsTag = tagManager.getTagByName('javascript');
    const duplicateTags = [
        tagManager.getTagByName('js')?.id,
        tagManager.getTagByName('jscript')?.id
    ].filter(Boolean);
    
    if (primaryJsTag && duplicateTags.length > 0) {
        await tagManager.mergeTags(primaryJsTag.id, duplicateTags);
        
        console.log('\nAfter merge:');
        const mergedTag = tagManager.getTag(primaryJsTag.id);
        console.log(`   - ${mergedTag.name} (usage: ${mergedTag.usage_count})`);
        console.log(`   - Duplicates merged successfully`);
    }
}

// Main execution
async function main() {
    try {
        console.log('🚀 Tag Management System Demo\n');
        console.log('='.repeat(50));
        
        // Setup the tag system
        const { tagManager, searchEngine } = await setupDevelopmentTags();
        
        // Establish relationships between tags
        await establishTagRelationships(tagManager);
        
        // Demonstrate search capabilities
        await demonstrateSearchCapabilities(searchEngine, tagManager);
        
        // Simulate tag usage
        await simulateTagUsage(tagManager);
        
        // Show management features
        await demonstrateTagManagement(tagManager);
        
        // Show smart recommendations
        await demonstrateSmartRecommendations(searchEngine, tagManager);
        
        // Demonstrate cleanup operations
        await demonstrateTagCleanup(tagManager);
        
        console.log('\n✨ Demo completed successfully!');
        
    } catch (error) {
        console.error('❌ Demo failed:', error.message);
    }
}

// Run the demo
// Run if executed directly
    main();
}

    setupDevelopmentTags,
    establishTagRelationships,
    demonstrateSearchCapabilities,
    simulateTagUsage,
    demonstrateTagManagement,
    demonstrateSmartRecommendations,
    demonstrateTagCleanup
};

/*
# Example 3: Advanced Tag Management System for Software Development Projects

This example demonstrates a comprehensive tag management system using the `TagManager` and `TagSearchEngine` classes to organize and manage tags for a software development team.

## Key Features Demonstrated:

### 1. **Bulk Tag Creation**
- Creating multiple tags at once with `bulkCreateTags()`
- Organizing tags by categories (technology, phase, priority, team)

### 2. **Tag Relationships**
- Establishing bidirectional relationships between related tags
- Creating semantic connections (e.g., JavaScript ↔ TypeScript)

### 3. **Advanced Search Capabilities**
- Simple text search in tag names and descriptions
- Complex queries with multiple filters (category, usage thresholds)
- Fuzzy search handling typos and misspellings
- Sorting results by different criteria (name, usage, creation date)

### 4. **Usage Tracking**
- Incrementing tag usage counts
- Getting most-used tags
- Finding related tags based on established relationships

### 5. **Tag Management**
- Updating tag properties
- Retrieving all categories
- Getting comprehensive system statistics

### 6. **Smart Recommendations**
- Suggesting related tags based on current selection
- Leveraging tag relationships for intelligent recommendations

### 7. **Tag Cleanup**
- Merging duplicate or similar tags
- Consolidating usage statistics during merge operations

This example showcases a practical implementation for organizing tags in a software development context, demonstrating how the tag system can help categorize and organize complex information with relationships, smart search, and intelligent features.
*/