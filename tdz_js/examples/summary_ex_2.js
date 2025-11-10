
import { SummaryManager, SummaryPriority } from './summary.js';

async function runSummaryExample() {
    // Initialize the summary manager
    const summaryManager = new SummaryManager();

    // Create summaries with different priorities and tags
    const summaryId1 = await summaryManager.createSummary({
        content: "Project planning meeting completed successfully",
        priority: SummaryPriority.High,
        tags: ["meeting", "planning", "project"]
    });

    const summaryId2 = await summaryManager.createSummary({
        content: "Weekly team sync - discussed blockers and progress",
        priority: SummaryPriority.Medium,
        tags: ["meeting", "sync", "team"]
    });

    const summaryId3 = await summaryManager.createSummary({
        content: "Server maintenance scheduled for weekend",
        priority: SummaryPriority.Critical,
        tags: ["maintenance", "server", "infrastructure"]
    });

    console.log("Created summaries:", [summaryId1, summaryId2, summaryId3]);

    // Retrieve a specific summary
    const summary = summaryManager.getSummary(summaryId1);
    console.log("\nRetrieved summary:", summary);

    // Update a summary
    await summaryManager.updateSummary(summaryId2, {
        content: "Weekly team sync - discussed blockers, progress, and next sprint goals",
        tags: ["meeting", "sync", "team", "sprint"]
    });

    // Search summaries
    const searchResults = summaryManager.searchSummaries("team");
    console.log("\nSearch results for 'team':", searchResults.map(s => s.content));

    // Get summaries by priority
    const highPrioritySummaries = summaryManager.getHighPrioritySummaries();
    console.log("\nHigh priority summaries count:", highPrioritySummaries.length);

    // Get summaries by tag
    const meetingSummaries = summaryManager.getSummariesByTag("meeting");
    console.log("\nMeeting summaries count:", meetingSummaries.length);

    // Get statistics
    const stats = summaryManager.getSummaryStatistics();
    console.log("\nSummary statistics:", {
        total: stats.total_summaries,
        highPriority: stats.high_priority_summaries,
        uniqueTags: stats.unique_tags,
        highPriorityPercentage: stats.highPriorityPercentage()
    });

    // Get tag statistics
    const tagStats = summaryManager.getTagStatistics();
    console.log("\nTag usage statistics:", tagStats);

    // Delete a summary
    await summaryManager.deleteSummary(summaryId3);
    console.log("\nDeleted summary. Total summaries now:", summaryManager.getAllSummaries().length);
}

// Run the example
runSummaryExample().catch(console.error);

/*
Here's a practical example demonstrating how to use the `SummaryManager` class to create, manage, and search summaries:

This example demonstrates:

1. **Creating summaries** with different priorities and tags
2. **Retrieving** a specific summary by ID
3. **Updating** summary content and tags
4. **Searching** summaries by keyword
5. **Filtering** summaries by priority and tags
6. **Getting statistics** about summaries and tags
7. **Deleting** summaries

Key features shown:
- Using `SummaryPriority` enum for priority levels
- Working with tags for categorization
- Searching across content and tags
- Getting statistical insights
- Managing summary lifecycle (create, read, update, delete)

When you run this example, you'll see output like:

/ *
Created summaries: [ 'uuid-1', 'uuid-2', 'uuid-3' ]

Retrieved summary: {
  id: 'uuid-1',
  content: 'Project planning meeting completed successfully',
  priority: 'high',
  tags: [ 'meeting', 'planning', 'project' ],
  created_at: 2023-05-15T10:30:00.000Z,
  updated_at: 2023-05-15T10:30:00.000Z
}

Search results for 'team': [
  'Weekly team sync - discussed blockers, progress, and next sprint goals'
]

High priority summaries count: 2
Meeting summaries count: 2

Summary statistics: {
  total: 3,
  highPriority: 2,
  uniqueTags: 7,
  highPriorityPercentage: 66.66666666666666
}

Tag usage statistics: {
  meeting: 2,
  planning: 1,
  project: 1,
  sync: 1,
  team: 1,
  sprint: 1,
  maintenance: 1,
  server: 1,
  infrastructure: 1
}

Deleted summary. Total summaries now: 2
*/