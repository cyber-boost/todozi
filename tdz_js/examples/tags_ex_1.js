
import { Tag, TagManager, TagSearchEngine, TagSearchQuery, TagSortBy } from '../todozi/tags.js';

class KnowledgeBase {
    constructor() {
        this.tagManager = TagManager.new();
        this.searchEngine = TagSearchEngine.new(this.tagManager);
        this.knowledgeItems = new Map();
    }

    // Initialize with common knowledge categories
    async initializeDefaultCategories() {
        const categories = [
            'programming', 'science', 'philosophy', 'history', 'technology',
            'productivity', 'health', 'business', 'art', 'literature'
        ];
        
        for (const category of categories) {
            await this.tagManager.createTag(new Tag({
                name: category,
                category: 'knowledge-domain',
                description: `${category} related knowledge`,
                color: this.getRandomColor()
            }));
        }
    }

    // Create tags for specific topics
    async addTopic(topicName, description, parentCategory, relatedTopics = []) {
        const topicTag = new Tag({
            name: topicName,
            description: description,
            category: parentCategory,
            color: '#4A90E2'
        });
        
        const topicId = await this.tagManager.createTag(topicTag);
        
        // Create relationships with related topics
        for (const relatedTopic of relatedTopics) {
            const relatedTag = this.tagManager.getTagByName(relatedTopic);
            if (relatedTag) {
                await this.tagManager.addTagRelationship(topicId, relatedTag.id);
            }
        }
        
        return topicId;
    }

    // Search for related knowledge topics
    findRelatedTopics(topicName, limit = 5) {
        const topic = this.tagManager.getTagByName(topicName);
        if (!topic) return [];
        
        return this.tagManager.getRelatedTags(topic.id).slice(0, limit);
    }

    // Fuzzy search for topics with spelling variations
    fuzzySearchTopics(query, maxDistance = 2) {
        return this.searchEngine.fuzzySearch(query, maxDistance);
    }

    // Get topic suggestions based on current interests
    getTopicSuggestions(currentInterests, limit = 10) {
        return this.searchEngine.getSuggestions(currentInterests, limit);
    }

    // Advanced search with filters
    searchKnowledge(query, filters = {}) {
        const searchQuery = new TagSearchQuery({
            name_contains: query,
            category: filters.category,
            min_usage: filters.minPopularity,
            max_usage: filters.maxPopularity,
            sort_by: filters.sortBy || TagSortBy.Usage,
            limit: filters.limit || 20
        });
        
        return this.searchEngine.advancedSearch(searchQuery);
    }

    // Track usage of topics (when someone reads/studies a topic)
    async trackTopicUsage(topicName) {
        await this.tagManager.incrementTagUsage(topicName);
    }

    // Get popular topics
    getPopularTopics(limit = 10) {
        return this.tagManager.getMostUsedTags(limit);
    }

    // Merge duplicate topics
    async mergeTopics(primaryTopic, duplicateTopics) {
        const primaryTag = this.tagManager.getTagByName(primaryTopic);
        if (!primaryTag) throw new Error(`Topic not found: ${primaryTopic}`);
        
        const duplicateIds = duplicateTopics.map(name => {
            const tag = this.tagManager.getTagByName(name);
            return tag ? tag.id : null;
        }).filter(id => id !== null);
        
        await this.tagManager.mergeTags(primaryTag.id, duplicateIds);
    }

    // Get statistics about knowledge organization
    getKnowledgeStats() {
        const stats = this.tagManager.getTagStatistics();
        
        return {
            totalTopics: stats.total_tags,
            totalCategories: stats.total_categories,
            topicRelationships: stats.total_relationships,
            avgTopicUsage: stats.average_usage.toFixed(2),
            relationshipsPerTopic: stats.relationshipsPerTag().toFixed(2),
            categories: this.tagManager.getAllCategories()
        };
    }

    // Helper function for random colors
    getRandomColor() {
        const colors = ['#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FECA57', '#FF9FF3'];
        return colors[Math.floor(Math.random() * colors.length)];
    }
}

// Usage Example
async function demonstrateKnowledgeBase() {
    const knowledgeBase = new KnowledgeBase();
    
    // Initialize with default categories
    await knowledgeBase.initializeDefaultCategories();
    
    // Add programming-related topics with relationships
    await knowledgeBase.addTopic('javascript', 'JavaScript programming language', 'programming', 
        ['web-development', 'nodejs']);
    
    await knowledgeBase.addTopic('python', 'Python programming language', 'programming',
        ['data-science', 'machine-learning', 'web-development']);
    
    await knowledgeBase.addTopic('machine-learning', 'Machine learning algorithms', 'science',
        ['python', 'data-science', 'artificial-intelligence']);
    
    await knowledgeBase.addTopic('react', 'React library for UI development', 'programming',
        ['javascript', 'web-development', 'frontend']);
    
    // Track usage (simulate studying topics)
    await knowledgeBase.trackTopicUsage('javascript');
    await knowledgeBase.trackTopicUsage('javascript');
    await knowledgeBase.trackTopicUsage('python');
    await knowledgeBase.trackTopicUsage('machine-learning');
    
    // Demonstrate searches
    console.log('=== Related Topics for "javascript" ===');
    const related = knowledgeBase.findRelatedTopics('javascript');
    related.forEach(topic => console.log(`- ${topic.name}: ${topic.description}`));
    
    console.log('\n=== Popular Topics ===');
    const popular = knowledgeBase.getPopularTopics(3);
    popular.forEach(topic => console.log(`- ${topic.name} (used ${topic.usage_count} times)`));
    
    console.log('\n=== Fuzzy Search for "mashine-lerning" (misspelled) ===');
    const fuzzyResults = knowledgeBase.fuzzySearchTopics('mashine-lerning', 3);
    fuzzyResults.forEach(([topic, distance]) => 
        console.log(`- ${topic.name} (distance: ${distance})`)
    );
    
    console.log('\n=== Topic Suggestions based on current interests ===');
    const suggestions = knowledgeBase.getTopicSuggestions(['javascript', 'web-development']);
    console.log('Suggestions:', suggestions);
    
    console.log('\n=== Knowledge Base Statistics ===');
    const stats = knowledgeBase.getKnowledgeStats();
    console.log(stats);
}

// Run the demonstration
demonstrateKnowledgeBase().catch(console.error);

/*
Here's a practical example demonstrating the tag management functionality - creating a tagging system for a personal knowledge base:

## Example: Personal Knowledge Management with Tags

This example shows how to use the tag system to organize notes, articles, and research topics with semantic relationships.

## Key Features Demonstrated:

1. **Hierarchical Organization**: Categories contain specific topics
2. **Semantic Relationships**: Topics are connected meaningfully (JavaScript → Web Development)
3. **Usage Tracking**: Monitor which topics get the most attention
4. **Smart Search**: Fuzzy search handles typos, related topic discovery
5. **Topic Suggestions**: Recommends related areas of study
6. **Analytics**: Provides insights into knowledge organization

This example shows how the tag system can be extended beyond simple labeling to create intelligent knowledge management systems that understand content relationships and user behavior patterns.
*/