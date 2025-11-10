
// Create an instance of IdeaManager
import { IdeaManager, IdeaUpdate, parseIdeaFormat } from '../todozi/idea.js';
import { ShareLevel, IdeaImportance, ItemStatus } from '../todozi/models.js';

const ideaManager = new IdeaManager();

// Example 1: Creating individual ideas
async function createSampleIdeas() {
    try {
        // Create a breakthrough idea
        const breakthroughIdea = {
            idea: "Develop AI-powered task prioritization system",
            share: ShareLevel.PUBLIC,
            importance: IdeaImportance.BREAKTHROUGH,
            tags: ["AI", "prioritization", "productivity"],
            context: "Would revolutionize how people manage complex projects"
        };
        const idea1Id = await ideaManager.createIdea(breakthroughIdea);
        
        // Create a private business idea
        const businessIdea = {
            idea: "Create subscription-based code review service for startups",
            share: ShareLevel.PRIVATE,
            importance: IdeaImportance.HIGH,
            tags: ["business", "code-review", "startup"],
            context: "Market research shows demand for affordable code quality services"
        };
        const idea2Id = await ideaManager.createIdea(businessIdea);
        
        // Create a team collaboration idea
        const teamIdea = {
            idea: "Build real-time collaborative documentation platform",
            share: ShareLevel.TEAM,
            importance: IdeaImportance.MEDIUM,
            tags: ["collaboration", "documentation", "real-time"],
            context: "Team needs better tools for simultaneous documentation"
        };
        const idea3Id = await ideaManager.createIdea(teamIdea);
        
        console.log("✅ Created ideas with IDs:", idea1Id, idea2Id, idea3Id);
        
    } catch (error) {
        console.error("❌ Error creating ideas:", error.message);
    }
}

// Example 2: Parsing ideas from formatted text
function parseTextIdeas() {
    try {
        const ideaText = `
        <idea>Implement machine learning for bug prediction; share; 2; ml,bug-prediction,quality; Using historical data to predict potential bugs in new code</idea>
        <idea>Create AI-powered code snippet generator; don't share; 3; ai,code-generation,tools; Internal tool for faster development</idea>
        `;
        
        // Split text and parse each idea
        const ideaBlocks = ideaText.split('<idea>').slice(1);
        for (const block of ideaBlocks) {
            const formattedIdea = `<idea>${block}`;
            const idea = parseIdeaFormat(formattedIdea);
            ideaManager.createIdea(idea);
        }
        
        console.log("✅ Parsed and stored ideas from formatted text");
        
    } catch (error) {
        console.error("❌ Error parsing ideas:", error.message);
    }
}

// Example 3: Searching and analyzing ideas
async function analyzeIdeas() {
    // Search for AI-related ideas
    const aiIdeas = ideaManager.searchIdeas("AI");
    console.log(`🔍 Found ${aiIdeas.length} AI-related ideas:`);
    aiIdeas.forEach(idea => console.log(`  - ${idea.idea}`));
    
    // Get breakthrough ideas
    const breakthroughs = ideaManager.getBreakthroughIdeas();
    console.log(`💡 Found ${breakthroughs.length} breakthrough ideas:`);
    breakthroughs.forEach(idea => console.log(`  - ${idea.idea}`));
    
    // Get ideas by tag
    const collaborationIdeas = ideaManager.getIdeasByTag("collaboration");
    console.log(`🤝 Found ${collaborationIdeas.length} collaboration ideas:`);
    collaborationIdeas.forEach(idea => console.log(`  - ${idea.idea}`));
    
    // Get recent ideas (last 5)
    const recent = ideaManager.getRecentIdeas(5);
    console.log("📅 Recent ideas:", recent.map(i => i.idea));
}

// Example 4: Updating an idea
async function updateIdeaExample() {
    const allIdeas = ideaManager.getAllIdeas();
    if (allIdeas.length > 0) {
        const firstIdea = allIdeas[0];
        
        const updates = IdeaUpdate.new()
            .withTags([...firstIdea.tags, "enhanced", "updated"])
            .withContext(`${firstIdea.context} - Updated with new insights`)
            .withImportance(IdeaImportance.HIGH);
        
        await ideaManager.updateIdea(firstIdea.id, updates);
        console.log("🔄 Updated first idea with enhanced tags and context");
    }
}

// Example 5: Statistical analysis
function showStatistics() {
    const stats = ideaManager.getIdeaStatistics();
    const tagStats = ideaManager.getTagStatistics();
    const allTags = ideaManager.getAllTags();
    
    console.log("\n📊 IDEA STATISTICS");
    console.log("==================");
    console.log(`Total Ideas: ${stats.totalIdeas}`);
    console.log(`Public: ${stats.publicIdeas} (${stats.publicPercentage().toFixed(1)}%)`);
    console.log(`Private: ${stats.privateIdeas} (${stats.privatePercentage().toFixed(1)}%)`);
    console.log(`Team: ${stats.teamIdeas} (${stats.teamPercentage().toFixed(1)}%)`);
    console.log(`Breakthrough: ${stats.breakthroughIdeas} (${stats.breakthroughPercentage().toFixed(1)}%)`);
    console.log(`Unique Tags: ${stats.uniqueTags}`);
    
    console.log("\n🏷️ TAG USAGE STATISTICS");
    console.log("======================");
    for (const [tag, count] of tagStats.entries()) {
        console.log(`${tag}: ${count} ideas`);
    }
}

// Example 6: Advanced search and filtering
async function advancedSearchExamples() {
    // Search by multiple criteria
    const highPriorityPublic = ideaManager.getAllIdeas().filter(idea => 
        idea.importance === IdeaImportance.HIGH && idea.share === ShareLevel.PUBLIC
    );
    console.log(`⭐ High-priority public ideas: ${highPriorityPublic.length}`);
    
    // Search with partial matching
    const developmentIdeas = ideaManager.searchIdeas("develop");
    console.log(`🔧 Development-related ideas: ${developmentIdeas.length}`);
    
    // Get ideas by importance level
    [IdeaImportance.LOW, IdeaImportance.MEDIUM, IdeaImportance.HIGH, IdeaImportance.BREAKTHROUGH]
        .forEach(importance => {
            const ideas = ideaManager.getIdeasByImportance(importance);
            console.log(`${importance.toUpperCase()} importance ideas: ${ideas.length}`);
        });
}

// Run all examples
async function runExamples() {
    console.log("🚀 IDEA MANAGEMENT SYSTEM EXAMPLES\n");
    
    await createSampleIdeas();
    parseTextIdeas();
    await analyzeIdeas();
    await updateIdeaExample();
    showStatistics();
    await advancedSearchExamples();
    
    console.log("\n✅ All examples completed successfully!");
}

runExamples().catch(console.error);

/*
# Idea Management Example

Here's a practical example demonstrating how to use the IdeaManager class to create, organize, and analyze ideas:

## Key Features Demonstrated:

1. **Idea Creation**: Creating ideas with different share levels, importance, and tags
2. **Text Parsing**: Converting formatted text into structured ideas
3. **Search & Filtering**: Finding ideas by content, tags, importance, and share level
4. **Idea Updates**: Modifying existing ideas using the fluent API
5. **Statistics**: Getting insights about your idea collection
6. **Advanced Analysis**: Complex filtering and categorization

## Sample Output:

/ *
🚀 IDEA MANAGEMENT SYSTEM EXAMPLES

✅ Created ideas with IDs: 123-abc 456-def 789-ghi
✅ Parsed and stored ideas from formatted text
🔍 Found 2 AI-related ideas:
  - Develop AI-powered task prioritization system
  - Create AI-powered code snippet generator
💡 Found 1 breakthrough ideas:
  - Develop AI-powered task prioritization system
🤝 Found 1 collaboration ideas:
  - Build real-time collaborative documentation platform
📅 Recent ideas: [array of recent ideas]
🔄 Updated first idea with enhanced tags and context

📊 IDEA STATISTICS
==================
Total Ideas: 5
Public: 2 (40.0%)
Private: 2 (40.0%)
Team: 1 (20.0%)
Breakthrough: 1 (20.0%)
Unique Tags: 8

🏷️ TAG USAGE STATISTICS
======================
AI: 2 ideas
prioritization: 1 ideas
productivity: 1 ideas
// ... more tag stats

✅ All examples completed successfully!
*/