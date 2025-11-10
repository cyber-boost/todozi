// example2.js - Advanced Search Engine Usage


// Create and populate search engine with sample data
import { SearchEngine } from './search.js';

function createSampleData() {
    const engine = new SearchEngine();
    
    // Sample tasks
    const tasks = [
        {
            id: 'task1',
            action: 'Implement user authentication',
            time: '2 hours',
            priority: 'high',
            parentProject: 'web-app',
            status: 'in_progress',
            assignee: 'ai',
            tags: ['security', 'backend', 'authentication'],
            createdAt: '2023-05-15T10:00:00Z'
        },
        {
            id: 'task2',
            action: 'Design homepage UI',
            time: '3 hours',
            priority: 'medium',
            parentProject: 'web-app',
            status: 'todo',
            assignee: 'human',
            tags: ['frontend', 'design', 'ui'],
            createdAt: '2023-05-16T09:00:00Z'
        }
    ];
    
    // Sample memories
    const memories = [
        {
            id: 'mem1',
            moment: 'User login failed multiple times',
            meaning: 'Authentication system might have issues',
            reason: 'Observed during testing',
            importance: 'high',
            term: 'short',
            tags: ['user-experience', 'security'],
            createdAt: '2023-05-15T14:30:00Z'
        }
    ];
    
    // Sample ideas
    const ideas = [
        {
            id: 'idea1',
            idea: 'Add dark mode toggle',
            share: 'team',
            importance: 'medium',
            tags: ['ui', 'accessibility'],
            createdAt: '2023-05-14T11:00:00Z'
        }
    ];
    
    // Sample errors
    const errors = [
        {
            id: 'err1',
            title: 'Database connection timeout',
            description: 'Failed to connect to PostgreSQL database',
            severity: 'high',
            category: 'database',
            source: 'backend-service',
            tags: ['database', 'connection'],
            createdAt: '2023-05-15T16:45:00Z'
        }
    ];
    
    // Add data to search engine
    engine.updateIndex({
        tasks,
        memories,
        ideas,
        errors,
        agentAssignments: [],
        codeChunks: [],
        training_data: [],
        feelings: []
    });
    
    return engine;
}

// Demonstrate various search capabilities
function demonstrateSearch() {
    const engine = createSampleData();
    
    console.log('=== Todozi Search Engine Demo ===\n');
    
    // 1. Basic search
    console.log('1. Basic Search for "authentication":');
    let results = engine.search('authentication');
    console.log(`   Found ${results.taskResults.length} tasks, ${results.memoryResults.length} memories`);
    
    // 2. Search with options
    console.log('\n2. Search with date filtering:');
    results = engine.search('ui', {
        since: '2023-05-15T00:00:00Z',
        dataTypes: ['Tasks', 'Ideas']
    });
    console.log(`   Found ${results.taskResults.length} tasks, ${results.ideaResults.length} ideas`);
    
    // 3. Tag-based search
    console.log('\n3. Search by tag "security":');
    results = engine.search('security');
    console.log(`   Found in ${results.taskResults.length} tasks and ${results.errorResults.length} errors`);
    
    // 4. Advanced search
    console.log('\n4. Advanced search for high-priority tasks:');
    results = engine.advancedSearch({
        taskCriteria: { priority: 'high' }
    });
    console.log(`   Found ${results.taskResults.length} high-priority tasks`);
    
    // 5. Analytics
    console.log('\n5. Search Analytics:');
    const analytics = engine.getSearchAnalytics();
    console.log(`   Total items: ${analytics.totalIndexedItems}`);
    console.log(`   Tasks: ${analytics.tasksCount}, Errors: ${analytics.errorsCount}`);
    
    // 6. Suggestions
    console.log('\n6. Search Suggestions for "auth":');
    const suggestions = engine.getSearchSuggestions('auth', 5);
    console.log(`   Suggestions: ${suggestions.join(', ')}`);
}

// Run the demonstration
demonstrateSearch();

/*
Here's a practical example demonstrating how to use the SearchEngine class with real data and search scenarios:

This example shows:

1. **Data Population**: How to create and populate the search engine with different types of content
2. **Basic Search**: Simple text search across all content types
3. **Filtered Search**: Using search options to filter by date and content types
4. **Tag Searching**: Finding content through associated tags
5. **Advanced Search**: Using structured criteria for precise filtering
6. **Analytics**: Getting insights about indexed content
7. **Suggestions**: Providing autocomplete-style search suggestions

Key features demonstrated:
- Multi-content type search (tasks, memories, ideas, errors)
- Date range filtering
- Content type filtering
- Tag-based matching
- Relevance scoring
- Search analytics
- Query suggestions
- Advanced filtering criteria

To run this example:
1. Save as `example2.js`
2. Ensure `search.js` is in the same directory
3. Run with `node example2.js`

The output will show practical search results and demonstrate how the engine handles different query types and filtering options.
*/