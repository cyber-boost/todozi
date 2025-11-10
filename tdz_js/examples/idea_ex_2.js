
import { IdeaManager, IdeaUpdate, ShareLevel, IdeaImportance } from '../todozi/idea.js';

async function demonstrateIdeaManager() {
    // Create an instance of IdeaManager
    const ideaManager = new IdeaManager();

    try {
        // 1. Create ideas
        const ideaId1 = await ideaManager.createIdea({
            idea: "Implement AI-powered task prioritization",
            share: ShareLevel.TEAM,
            importance: IdeaImportance.HIGH,
            tags: ["ai", "productivity", "automation"],
            context: "Improve task management efficiency"
        });

        const ideaId2 = await ideaManager.createIdea({
            idea: "Add dark mode to the application",
            share: ShareLevel.PUBLIC,
            importance: IdeaImportance.MEDIUM,
            tags: ["ui", "ux", "accessibility"],
            context: "User request for eye comfort"
        });

        console.log("Created ideas:", ideaId1, ideaId2);

        // 2. Retrieve a specific idea
        const retrievedIdea = ideaManager.getIdea(ideaId1);
        console.log("Retrieved idea:", retrievedIdea.idea);

        // 3. Update an idea
        const updates = IdeaUpdate.new()
            .withImportance(IdeaImportance.BREAKTHROUGH)
            .withTags(["ai", "ml", "productivity", "innovation"]);
        
        await ideaManager.updateIdea(ideaId1, updates);
        console.log("Updated idea importance and tags");

        // 4. Search ideas
        const searchResults = ideaManager.searchIdeas("AI");
        console.log("Search results for 'AI':", searchResults.length, "ideas found");

        // 5. Get ideas by criteria
        const teamIdeas = ideaManager.getTeamIdeas();
        const publicIdeas = ideaManager.getPublicIdeas();
        const breakthroughIdeas = ideaManager.getBreakthroughIdeas();
        
        console.log(`Team ideas: ${teamIdeas.length}`);
        console.log(`Public ideas: ${publicIdeas.length}`);
        console.log(`Breakthrough ideas: ${breakthroughIdeas.length}`);

        // 6. Get ideas by tag
        const aiTagIdeas = ideaManager.getIdeasByTag("ai");
        console.log(`Ideas tagged with 'ai': ${aiTagIdeas.length}`);

        // 7. Get all tags and statistics
        const allTags = ideaManager.getAllTags();
        const tagStats = ideaManager.getTagStatistics();
        console.log("All tags:", allTags);
        console.log("Tag statistics:", tagStats);

        // 8. Get idea statistics
        const stats = ideaManager.getIdeaStatistics();
        console.log("Idea statistics:");
        console.log(`  Total ideas: ${stats.totalIdeas}`);
        console.log(`  Public ideas: ${stats.publicIdeas} (${stats.publicPercentage().toFixed(1)}%)`);
        console.log(`  Team ideas: ${stats.teamIdeas} (${stats.teamPercentage().toFixed(1)}%)`);
        console.log(`  Breakthrough ideas: ${stats.breakthroughIdeas} (${stats.breakthroughPercentage().toFixed(1)}%)`);
        console.log(`  Unique tags: ${stats.uniqueTags}`);

        // 9. Get recent ideas
        const recentIdeas = ideaManager.getRecentIdeas(5);
        console.log(`Most recent 5 ideas: ${recentIdeas.length} found`);

        // 10. Delete an idea
        await ideaManager.deleteIdea(ideaId2);
        console.log("Deleted idea:", ideaId2);

        // Verify deletion
        const allIdeas = ideaManager.getAllIdeas();
        console.log(`Remaining ideas after deletion: ${allIdeas.length}`);

    } catch (error) {
        console.error("Error in idea management:", error.message);
    }
}

// Run the demonstration
demonstrateIdeaManager();

/*
Here's a practical example demonstrating how to use the `IdeaManager` class to manage ideas with various operations:

This example shows how to:

1. Create ideas with different properties (idea text, sharing level, importance, tags, context)
2. Retrieve specific ideas by ID
3. Update idea properties using the builder pattern
4. Search ideas by text content
5. Filter ideas by sharing level, importance, and tags
6. Get statistics about ideas and tags
7. Retrieve recent ideas
8. Delete ideas

Key features demonstrated:
- Creating ideas with `createIdea()`
- Retrieving ideas with `getIdea()` and `getAllIdeas()`
- Updating ideas with `updateIdea()` and the `IdeaUpdate` builder
- Searching with `searchIdeas()`
- Filtering by criteria using specialized methods
- Getting statistics with `getIdeaStatistics()` and `getTagStatistics()`
- Managing tags with `getAllTags()`
- Deleting ideas with `deleteIdea()`

The example handles both public/team/private sharing levels and different importance levels (low/medium/high/breakthrough). It also shows proper error handling when operations might fail.
*/