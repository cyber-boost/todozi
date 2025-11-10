import { 
    Task, 
    Priority, 
    Status, 
    Assignee
} from '../todozi/models.js';
import { TagManager, TagSearchEngine } from '../todozi/tags.js';
import { TodoziEmbeddingService } from '../todozi/emb.js';
import { Storage } from '../todozi/storage.js';
import { processChatMessageExtended } from '../todozi/todozi.js';

async function setupDevProject() {
    console.log('🚀 Setting up Todozi development project management...');
    
    // Initialize storage
    const storage = await Storage.new();
    
    // Initialize tag manager
    const tagManager = TagManager.new();
    const searchEngine = TagSearchEngine.new(tagManager);
    
    // Create project-specific tags
    const frontendTag = await tagManager.createTag({
        name: 'frontend',
        description: 'Frontend development tasks',
        category: 'component',
        color: '#FF6B6B'
    });
    
    const backendTag = await tagManager.createTag({
        name: 'backend',
        description: 'Backend development tasks',
        category: 'component',
        color: '#4ECDC4'
    });
    
    const databaseTag = await tagManager.createTag({
        name: 'database',
        description: 'Database related tasks',
        category: 'component',
        color: '#45B7D1'
    });
    
    const bugTag = await tagManager.createTag({
        name: 'bug',
        description: 'Bug fixes and troubleshooting',
        category: 'type',
        color: '#FF6B6B'
    });
    
    const featureTag = await tagManager.createTag({
        name: 'feature',
        description: 'New feature development',
        category: 'type',
        color: '#95E77E'
    });
    
    // Set up tag relationships
    await tagManager.addTagRelationship(frontendTag, featureTag);
    await tagManager.addTagRelationship(backendTag, featureTag);
    await tagManager.addTagRelationship(databaseTag, backendTag);
    
    console.log('✅ Tags created and relationships established');
    
    return { storage, tagManager, searchEngine };
}

async function createDevelopmentTasks(storage, tagManager) {
    console.log('📝 Creating development tasks...');
    
    // Create a complex development message with multiple task types
    const devMessage = `
Project: WebApp Redesign

<todozi>Implement responsive navbar component; 4 hours; high; WebApp Redesign; in_progress; ai; frontend,component,ui;nav-design-mockup;Based on Figma designs from design team</todozi>

<todozi>Optimize database queries for user dashboard; 2 days; critical; WebApp Redesign; todo; collaborative; backend,database,performance;user-auth-module;Fix slow loading issue;75</todozi>

<todozi>Write unit tests for authentication service; 6 hours; medium; WebApp Redesign; todo; agent=coder; backend,testing;auth-service-tests;Ensure 80% code coverage</todozi>

<todozi>Design database schema for new features; 1 day; high; WebApp Redesign; in_progress; human; database,design;schema-v2-design;Include scalability considerations;60</todozi>

<todozi>Setup CI/CD pipeline; 1 week; high; WebApp Redesign; todo; devops; infrastructure,automation;pipeline-config;Use GitHub Actions</todozi>
`;
    
    // Process the message to extract tasks
    const content = processChatMessageExtended(devMessage, 'dev-team');
    
    // Save all tasks
    for (const task of content.tasks) {
        await storage.addTaskToProject(task);
        
        // Update tag usage counts
        if (task.tags) {
            for (const tagName of task.tags) {
                await tagManager.incrementTagUsage(tagName);
            }
        }
    }
    
    console.log(`✅ Created ${content.tasks.length} development tasks`);
    
    // Save other content types
    for (const memory of content.memories) {
        await storage.saveMemory(memory);
    }
    
    for (const idea of content.ideas) {
        await storage.saveIdea(idea);
    }
    
    return content.tasks;
}

async function demonstrateTagFeatures(tagManager, searchEngine) {
    console.log('\n🏷️ Demonstrating tag management features...');
    
    // Search for tags by category
    const componentTags = tagManager.getTagsByCategory('component');
    console.log('📦 Component tags:', componentTags.map(t => t.name));
    
    // Search for tags with usage statistics
    const mostUsedTags = tagManager.getMostUsedTags(5);
    console.log('🔥 Most used tags:');
    mostUsedTags.forEach(tag => {
        console.log(`  - ${tag.name}: ${tag.usage_count} uses`);
    });
    
    // Get tag relationships
    const frontendTag = tagManager.getTagByName('frontend');
    if (frontendTag) {
        const relatedTags = tagManager.getRelatedTags(frontendTag.id);
        console.log(`🔗 Tags related to 'frontend':`, relatedTags.map(t => t.name));
    }
    
    // Advanced search
    const searchQuery = TagSearchQuery.default()
        .category('component')
        .min_usage(1)
        .sort_by('Usage')
        .limit(10);
    
    const searchResults = searchEngine.advancedSearch(searchQuery);
    console.log('🔍 Search results for component tags with usage > 0:');
    searchResults.forEach(tag => {
        console.log(`  - ${tag.name} (${tag.category}): ${tag.usage_count} uses`);
    });
    
    // Get tag statistics
    const stats = tagManager.getTagStatistics();
    console.log('\n📊 Tag Statistics:');
    console.log(`  Total tags: ${stats.total_tags}`);
    console.log(`  Total categories: ${stats.total_categories}`);
    console.log(`  Average usage: ${stats.average_usage.toFixed(2)}`);
    console.log(`  Relationships per tag: ${stats.relationshipsPerTag().toFixed(2)}`);
}

async function demonstrateSemanticSearch(storage) {
    console.log('\n🔍 Demonstrating semantic search capabilities...');
    
    // Initialize embedding service
    const embeddingService = await TodoziEmbeddingService.new();
    
    // Search for similar tasks
    const searchQuery = "optimize performance";
    const similarTasks = await embeddingService.findSimilarTasks(searchQuery, 5);
    
    console.log(`\n🎯 Tasks similar to "${searchQuery}":`);
    similarTasks.forEach((result, index) => {
        console.log(`${index + 1}. ${result.text_content.split('\n')[0]} (${(result.similarity_score * 100).toFixed(1)}% similar)`);
    });
    
    // Semantic search across content types
    const semanticResults = await embeddingService.semanticSearch(
        "database optimization",
        ['Task', 'Memory', 'Idea'],
        10
    );
    
    console.log('\n🧠 Semantic search results across all content:');
    semanticResults.forEach((result, index) => {
        const preview = result.text_content.split('\n')[0].substring(0, 60);
        console.log(`${index + 1}. [${result.content_type}] ${preview}... (${(result.similarity_score * 100).toFixed(1)}%)`);
    });
    
    // Cluster content to find related items
    const clusters = await embeddingService.clusterContent();
    console.log(`\n🔗 Found ${clusters.length} semantic clusters`);
    clusters.slice(0, 3).forEach((cluster, index) => {
        console.log(`Cluster ${index + 1}: ${cluster.cluster_size} items, ${(cluster.average_similarity * 100).toFixed(1)}% avg similarity`);
    });
}

async function demonstrateTaskWorkflow(storage, tagManager) {
    console.log('\n🔄 Demonstrating task workflow...');
    
    // Get all active tasks
    const activeTasks = await storage.listTasksAcrossProjects({ status: Status.Todo });
    console.log(`📋 Found ${activeTasks.length} active tasks`);
    
    // Update a task status
    if (activeTasks.length > 0) {
        const task = activeTasks[0];
        console.log(`\n⚡ Updating task: ${task.action}`);
        
        const updates = {
            status: Status.InProgress,
            progress: 25
        };
        
        await storage.updateTaskInProject(task.id, updates);
        console.log(`✅ Task updated to ${updates.status} with ${updates.progress}% progress`);
    }
    
    // Search tasks by tags
    const frontendTasks = await storage.listTasksAcrossProjects({
        tags: ['frontend']
    });
    console.log(`\n🎨 Found ${frontendTasks.length} frontend tasks`);
    
    // Search tasks by priority
    const criticalTasks = await storage.listTasksAcrossProjects({
        priority: Priority.Critical
    });
    console.log(`⚠️ Found ${criticalTasks.length} critical tasks`);
    
    // Get project statistics
    const allTasks = await storage.listTasksAcrossProjects({});
    const completedTasks = await storage.listTasksAcrossProjects({ status: Status.Done });
    
    console.log('\n📈 Project Statistics:');
    console.log(`  Total tasks: ${allTasks.length}`);
    console.log(`  Completed: ${completedTasks.length}`);
    console.log(`  Completion rate: ${((completedTasks.length / allTasks.length) * 100).toFixed(1)}%`);
}

async function runInteractiveDemo() {
    console.log('🎯 Todozi Development Project Management Demo\n');
    console.log('=' .repeat(50));
    
    try {
        // Set up the project
        const { storage, tagManager, searchEngine } = await setupDevProject();
        
        // Create development tasks
        const tasks = await createDevelopmentTasks(storage, tagManager);
        
        // Demonstrate tag features
        await demonstrateTagFeatures(tagManager, searchEngine);
        
        // Demonstrate semantic search
        await demonstrateSemanticSearch(storage);
        
        // Demonstrate task workflow
        await demonstrateTaskWorkflow(storage, tagManager);
        
        console.log('\n✅ Demo completed successfully!');
        console.log('\n💡 Key features demonstrated:');
        console.log('  • Task creation with structured format');
        console.log('  • Tag management and relationships');
        console.log('  • Semantic search capabilities');
        console.log('  • Task workflow and status updates');
        console.log('  • Project statistics and reporting');
        
    } catch (error) {
        console.error('❌ Demo failed:', error.message);
        console.error(error.stack);
    }
}

// Export for use in other modules
    setupDevProject,
    createDevelopmentTasks,
    demonstrateTagFeatures,
    demonstrateSemanticSearch,
    demonstrateTaskWorkflow,
    runInteractiveDemo
};

// Run the demo if this file is executed directly
// Run if executed directly
    runInteractiveDemo();
}

/*
# Example 3: Managing a Software Development Project with Todozi

This example demonstrates how to use the Todozi system to manage a real software development project, including task creation, tag management, and semantic search features.

## Usage Instructions

Save this example as `dev-project-demo.js` and run it:

/ *
bash
node dev-project-demo.js
*/