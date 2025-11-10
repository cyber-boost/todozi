// project-summary-example.js

import { SummaryManager, SummaryUpdate, SummaryPriority, parseSummaryFormat } from '../todozi/summary.js';

class ProjectProgressTracker {
    constructor() {
        this.summaryManager = new SummaryManager();
        this.projectTags = new Map(); // Track tags by project
    }

    // Create a summary for a completed project milestone
    async createMilestoneSummary(projectName, milestone, priority, context = null, tags = []) {
        const summary = {
            content: `Milestone completed: ${milestone}`,
            context: context || `Project: ${projectName}`,
            priority: priority,
            tags: [...tags, 'milestone', projectName.toLowerCase().replace(' ', '-')]
        };

        const summaryId = await this.summaryManager.createSummary(summary);
        
        // Track project-specific tags
        if (!this.projectTags.has(projectName)) {
            this.projectTags.set(projectName, new Set());
        }
        tags.forEach(tag => this.projectTags.get(projectName).add(tag));
        
        console.log(`✅ Created milestone summary: ${milestone}`);
        return summaryId;
    }

    // Create summary from formatted text
    parseAndCreateSummary(formattedText) {
        try {
            const summary = parseSummaryFormat(formattedText);
            return this.summaryManager.createSummary(summary);
        } catch (error) {
            console.error('❌ Failed to parse summary:', error.message);
            throw error;
        }
    }

    // Update a summary when progress changes
    async updateProgress(summaryId, newContent, updatedTags = null) {
        const update = SummaryUpdate.new()
            .setContent(newContent)
            .setTags(updatedTags);

        await this.summaryManager.updateSummary(summaryId, update);
        console.log(`📝 Updated summary ${summaryId}`);
    }

    // Get all high priority summaries for review
    getCriticalSummaries() {
        return this.summaryManager.getHighPrioritySummaries();
    }

    // Search summaries across all projects
    searchProjectSummaries(query, projectName = null) {
        let results = this.summaryManager.searchSummaries(query);
        
        if (projectName) {
            results = results.filter(summary => 
                summary.tags.includes(projectName.toLowerCase().replace(' ', '-'))
            );
        }
        
        return results;
    }

    // Get statistics for project reporting
    generateProjectReport(projectName) {
        const allSummaries = this.summaryManager.getAllSummaries();
        const projectSummaries = allSummaries.filter(summary => 
            summary.tags.includes(projectName.toLowerCase().replace(' ', '-'))
        );

        const stats = {
            totalMilestones: projectSummaries.length,
            highPriority: projectSummaries.filter(s => 
                s.priority === SummaryPriority.High || s.priority === SummaryPriority.Critical
            ).length,
            recentSummaries: this.summaryManager.getRecentSummaries(5).filter(s =>
                s.tags.includes(projectName.toLowerCase().replace(' ', '-'))
            ),
            tagStatistics: this.summaryManager.getTagStatistics()
        };

        return stats;
    }

    // Display recent summaries in a formatted way
    displayRecentProgress(limit = 10) {
        const recent = this.summaryManager.getRecentSummaries(limit);
        
        console.log('\n📊 Recent Project Progress:');
        console.log('══════════════════════════════════════');
        
        recent.forEach((summary, index) => {
            const priorityEmoji = {
                [SummaryPriority.Low]: '🟢',
                [SummaryPriority.Medium]: '🟡',
                [SummaryPriority.High]: '🟠',
                [SummaryPriority.Critical]: '🔴'
            }[summary.priority] || '⚪';

            console.log(`${index + 1}. ${priorityEmoji} ${summary.content}`);
            console.log(`   Context: ${summary.context || 'No context'}`);
            console.log(`   Tags: ${summary.tags.join(', ')}`);
            console.log(`   Created: ${summary.created_at.toLocaleDateString()}`);
            console.log('   ──────────────────────────');
        });
    }
}

// Usage Example
async function demonstrateSummaryTracking() {
    const tracker = new ProjectProgressTracker();

    try {
        // Create milestone summaries
        const milestone1 = await tracker.createMilestoneSummary(
            'Website Redesign',
            'Homepage layout completed',
            SummaryPriority.High,
            'Frontend team delivered responsive design',
            ['frontend', 'design', 'responsive']
        );

        const milestone2 = await tracker.createMilestoneSummary(
            'Website Redesign',
            'Backend API integration',
            SummaryPriority.Critical,
            'REST API endpoints working with frontend',
            ['backend', 'api', 'integration']
        );

        const milestone3 = await tracker.parseAndCreateSummary(
            "<summary>Database optimization completed; medium; Improved query performance by 40%; performance,optimization,database</summary>"
        );

        // Update a summary
        await tracker.updateProgress(
            milestone2,
            'Backend API integration - All endpoints tested and deployed',
            ['backend', 'api', 'integration', 'tested', 'deployed']
        );

        // Search for specific summaries
        const apiSummaries = tracker.searchProjectSummaries('API', 'Website Redesign');
        console.log(`\n🔍 Found ${apiSummaries.length} API-related summaries`);

        // Generate project report
        const report = tracker.generateProjectReport('Website Redesign');
        console.log('\n📈 Project Report:');
        console.log(`Total milestones: ${report.totalMilestones}`);
        console.log(`High priority items: ${report.highPriority}`);

        // Display recent progress
        tracker.displayRecentProgress();

        // Get tag statistics
        const tagStats = tracker.summaryManager.getTagStatistics();
        console.log('\n🏷️ Tag Usage Statistics:');
        Object.entries(tagStats).forEach(([tag, count]) => {
            console.log(`   ${tag}: ${count} summaries`);
        });

    } catch (error) {
        console.error('Error in summary tracking:', error);
    }
}

// Run the demonstration
demonstrateSummaryTracking();


/*
Here's a practical example that demonstrates how to use the SummaryManager to track and manage project summaries:

## Example: Project Progress Summary Tracker

## Sample Output:

/ *
✅ Created milestone summary: Homepage layout completed
✅ Created milestone summary: Backend API integration
✅ Created milestone summary: Database optimization completed
📝 Updated summary summary_abc123

📊 Recent Project Progress:
══════════════════════════════════════
1. 🔴 Backend API integration - All endpoints tested and deployed
   Context: REST API endpoints working with frontend
   Tags: backend, api, integration, tested, deployed
   Created: 12/15/2023
   ──────────────────────────
2. 🟡 Database optimization completed
   Context: Improved query performance by 40%
   Tags: performance, optimization, database
   Created: 12/14/2023
   ──────────────────────────

📈 Project Report:
Total milestones: 3
High priority items: 1

🏷️ Tag Usage Statistics:
   backend: 1 summaries
   api: 1 summaries
   frontend: 1 summaries
   performance: 1 summaries
*/