// Example 1: Basic Search Engine Usage
import { SearchEngine, SearchDataType } from '../todozi/index.js';

async function runSearchExample() {
    // Initialize search engine
    const searchEngine = new SearchEngine();
    
    // Sample data to index
    const sampleData = {
        tasks: [
            {
                action: "Implement user authentication system",
                status: "todo",
                priority: "high",
                tags: ["security", "backend", "api"],
                createdAt: new Date('2024-01-15').toISOString()
            },
            {
                action: "Update landing page design",
                status: "in_progress",
                priority: "medium",
                tags: ["frontend", "ui", "design"],
                createdAt: new Date('2024-01-20').toISOString()
            }
        ],
        memories: [
            {
                moment: "Team meeting discussion",
                meaning: "Decision to use JWT for authentication",
                reason: "Better security practices",
                tags: ["meeting", "authentication", "security"],
                createdAt: new Date('2024-01-10').toISOString()
            }
        ],
        ideas: [
            {
                idea: "Add two-factor authentication",
                context: "Security enhancement",
                tags: ["security", "feature"],
                createdAt: new Date('2024-01-18').toISOString()
            }
        ],
        errors: [
            {
                title: "CORS configuration issue",
                description: "API requests failing due to CORS policy",
                source: "Frontend API calls",
                tags: ["api", "frontend", "cors"],
                createdAt: new Date('2024-01-22').toISOString()
            }
        ],
        training_data: [
            {
                prompt: "How to handle JWT tokens?",
                completion: "Store tokens securely and implement refresh mechanism",
                tags: ["authentication", "security"],
                createdAt: new Date('2024-01-12').toISOString()
            }
        ]
    };

    // Update search index with sample data
    searchEngine.updateIndex(sampleData);

    // Example 1: Basic search for authentication-related content
    console.log("🔍 Search Results for 'authentication':");
    const results1 = searchEngine.search("authentication");
    
    console.log(`📋 Tasks: ${results1.taskResults.length}`);
    results1.taskResults.forEach(result => {
        console.log(`   - ${result.task.action} (Score: ${result.score.toFixed(2)})`);
    });
    
    console.log(`🧠 Memories: ${results1.memoryResults.length}`);
    results1.memoryResults.forEach(result => {
        console.log(`   - ${result.memory.moment} (Score: ${result.score.toFixed(2)})`);
    });

    // Example 2: Search with filters
    console.log("\n🔍 Search Results for 'security' (Tasks only, recent):");
    const searchOptions = {
        dataTypes: [SearchDataType.Tasks], // Only search tasks
        since: new Date('2024-01-01').toISOString(), // Only items from 2024
        limit: 5 // Limit results
    };
    
    const results2 = searchEngine.search("security", searchOptions);
    console.log(`📋 Security-related tasks: ${results2.taskResults.length}`);
    results2.taskResults.forEach(result => {
        console.log(`   - ${result.task.action} [${result.task.status}]`);
    });

    // Example 3: Get search analytics
    const analytics = searchEngine.getSearchAnalytics();
    console.log("\n📊 Search Analytics:");
    console.log(`   Total indexed items: ${analytics.totalIndexedItems}`);
    console.log(`   Tasks: ${analytics.tasksCount}`);
    console.log(`   Memories: ${analytics.memoriesCount}`);
    console.log(`   Ideas: ${analytics.ideasCount}`);
    console.log(`   Errors: ${analytics.errorsCount}`);
    console.log(`   Training data: ${analytics.trainingCount}`);

    // Example 4: Search suggestions
    console.log("\n💡 Search Suggestions for 'api':");
    const suggestions = searchEngine.getSearchSuggestions("api", 5);
    console.log("   Suggestions:", suggestions);

    // Example 5: Advanced search with criteria
    console.log("\n🔍 Advanced Search: High priority tasks");
    const criteria = {
        taskCriteria: {
            priority: "high",
            status: "todo"
        }
    };
    
    const advancedResults = searchEngine.advancedSearch(criteria);
    console.log(`📋 High priority todo tasks: ${advancedResults.taskResults.length}`);
    advancedResults.taskResults.forEach(result => {
        console.log(`   - ${result.task.action}`);
    });
}

// Run the example
runSearchExample().catch(console.error);

/*
# Search Engine Usage Example

Here's a practical example showing how to use the SearchEngine class from the provided code:

## Example: Creating and Using the Search Engine

## Example Output:

/ *
🔍 Search Results for 'authentication':
📋 Tasks: 1
   - Implement user authentication system (Score: 0.85)
🧠 Memories: 1
   - Team meeting discussion (Score: 0.72)

🔍 Search Results for 'security' (Tasks only, recent):
📋 Security-related tasks: 1
   - Implement user authentication system [todo]

📊 Search Analytics:
   Total indexed items: 5
   Tasks: 2
   Memories: 1
   Ideas: 1
   Errors: 1
   Training data: 1

💡 Search Suggestions for 'api':
   Suggestions: ["security", "backend", "frontend", "cors"]

🔍 Advanced Search: High priority tasks
📋 High priority todo tasks: 1
   - Implement user authentication system
*/