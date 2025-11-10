import { Summary, SummaryClass, SummaryPriority, TodoziError, Result, Task, Memory, Idea, ErrorRecord } from './models.js';
import { DateUtils } from './utils.js';

// Summary generation functionality
export class SummaryManager {
    private summaries: Map<string, Summary> = new Map();

    constructor() {
        this.summaries = new Map();
    }

    static new(): SummaryManager {
        return new SummaryManager();
    }

    async createSummary(
        content: string,
        priority: SummaryPriority,
        context?: string,
        tags: string[] = []
    ): Promise<Result<string>> {
        try {
            const summary = new SummaryClass(
                undefined,
                content,
                context,
                priority,
                tags
            );

            this.summaries.set(summary.id, summary);
            return { ok: true, value: summary.id };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Summary creation failed')
            };
        }
    }

    getSummary(summaryId: string): Summary | undefined {
        return this.summaries.get(summaryId);
    }

    getAllSummaries(): Summary[] {
        return Array.from(this.summaries.values());
    }

    async updateSummary(
        summaryId: string,
        updates: Partial<{
            content: string;
            context: string;
            priority: SummaryPriority;
            tags: string[];
        }>
    ): Promise<Result<void>> {
        const summary = this.summaries.get(summaryId);
        if (!summary) {
            return {
                ok: false,
                error: TodoziError.notFound(`Summary ${summaryId} not found`)
            };
        }

        try {
            if (updates.content !== undefined) {
                (summary as SummaryClass).content = updates.content;
            }
            if (updates.context !== undefined) {
                (summary as SummaryClass).context = updates.context;
            }
            if (updates.priority !== undefined) {
                (summary as SummaryClass).priority = updates.priority;
            }
            if (updates.tags !== undefined) {
                (summary as SummaryClass).tags = updates.tags;
            }

            (summary as SummaryClass).updatedAt = new Date();
            this.summaries.set(summaryId, summary);

            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Summary update failed')
            };
        }
    }

    async deleteSummary(summaryId: string): Promise<Result<void>> {
        if (!this.summaries.has(summaryId)) {
            return {
                ok: false,
                error: TodoziError.notFound(`Summary ${summaryId} not found`)
            };
        }

        this.summaries.delete(summaryId);
        return { ok: true, value: undefined };
    }

    getSummariesByPriority(priority: SummaryPriority): Summary[] {
        return Array.from(this.summaries.values()).filter(s => s.priority === priority);
    }

    getSummariesByTag(tag: string): Summary[] {
        return Array.from(this.summaries.values()).filter(s => s.tags.includes(tag));
    }

    getRecentSummaries(limit: number = 10): Summary[] {
        return Array.from(this.summaries.values())
            .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
            .slice(0, limit);
    }

    getSummariesByDateRange(from: Date, to: Date): Summary[] {
        return Array.from(this.summaries.values()).filter(s =>
            s.createdAt >= from && s.createdAt <= to
        );
    }

    getAllTags(): string[] {
        const tags = new Set<string>();
        this.summaries.forEach(summary => {
            summary.tags.forEach(tag => tags.add(tag));
        });
        return Array.from(tags);
    }

    getTagStatistics(): Map<string, number> {
        const stats = new Map<string, number>();
        this.summaries.forEach(summary => {
            summary.tags.forEach(tag => {
                stats.set(tag, (stats.get(tag) || 0) + 1);
            });
        });
        return stats;
    }

    getSummaryStatistics(): {
        totalSummaries: number;
        summariesByPriority: Record<SummaryPriority, number>;
        totalTags: number;
        averageTagsPerSummary: number;
    } {
        const totalSummaries = this.summaries.size;
        const summariesByPriority = {
            [SummaryPriority.Low]: 0,
            [SummaryPriority.Medium]: 0,
            [SummaryPriority.High]: 0,
            [SummaryPriority.Critical]: 0
        };

        let totalTags = 0;

        this.summaries.forEach(summary => {
            summariesByPriority[summary.priority]++;
            totalTags += summary.tags.length;
        });

        return {
            totalSummaries,
            summariesByPriority,
            totalTags,
            averageTagsPerSummary: totalSummaries > 0 ? totalTags / totalSummaries : 0
        };
    }

    clear(): void {
        this.summaries.clear();
    }
}

// Summary generator for creating summaries from various data sources
export class SummaryGenerator {
    private summaryManager: SummaryManager;

    constructor(summaryManager?: SummaryManager) {
        this.summaryManager = summaryManager || SummaryManager.new();
    }

    async generateTaskSummary(tasks: Task[], context?: string): Promise<Result<string>> {
        try {
            const completedTasks = tasks.filter(t => t.status === 'done' || t.status === 'completed');
            const activeTasks = tasks.filter(t => t.status === 'in_progress' || t.status === 'todo');

            const summary = `Task Summary (${DateUtils.formatDate(new Date())}):
• Total Tasks: ${tasks.length}
• Completed: ${completedTasks.length}
• Active/In Progress: ${activeTasks.length}
• Completion Rate: ${tasks.length > 0 ? Math.round((completedTasks.length / tasks.length) * 100) : 0}%

Recent Activity:
${completedTasks.slice(-5).map(t => `✓ ${t.action} (${DateUtils.getRelativeTime(t.updatedAt)})`).join('\n')}

Next Priorities:
${activeTasks.slice(0, 5).map(t => `• ${t.action} (${t.priority})`).join('\n')}`;

            return await this.summaryManager.createSummary(
                summary,
                SummaryPriority.Medium,
                context,
                ['tasks', 'productivity', 'progress']
            );
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Task summary generation failed')
            };
        }
    }

    async generateMemorySummary(memories: Memory[], context?: string): Promise<Result<string>> {
        try {
            const recentMemories = memories
                .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
                .slice(0, 10);

            const memoryTypes = memories.reduce((acc, m) => {
                acc[m.memoryType] = (acc[m.memoryType] || 0) + 1;
                return acc;
            }, {} as Record<string, number>);

            const summary = `Memory Summary (${DateUtils.formatDate(new Date())}):
• Total Memories: ${memories.length}
• Memory Types: ${Object.entries(memoryTypes).map(([type, count]) => `${type}: ${count}`).join(', ')}

Recent Memories:
${recentMemories.map(m => `• ${m.moment} (${m.importance}, ${DateUtils.getRelativeTime(m.createdAt)})`).join('\n')}

Key Insights:
${recentMemories.filter(m => m.importance === 'high' || m.importance === 'critical').map(m => `• ${m.meaning}`).join('\n')}`;

            return await this.summaryManager.createSummary(
                summary,
                SummaryPriority.High,
                context,
                ['memories', 'insights', 'reflection']
            );
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Memory summary generation failed')
            };
        }
    }

    async generateIdeaSummary(ideas: Idea[], context?: string): Promise<Result<string>> {
        try {
            const breakthroughIdeas = ideas.filter(i => i.importance === 'breakthrough');
            const teamIdeas = ideas.filter(i => i.share === 'team' || i.share === 'public');

            const summary = `Idea Summary (${DateUtils.formatDate(new Date())}):
• Total Ideas: ${ideas.length}
• Breakthrough Ideas: ${breakthroughIdeas.length}
• Team/Public Ideas: ${teamIdeas.length}

Breakthrough Ideas:
${breakthroughIdeas.slice(0, 5).map(i => `🚀 ${i.idea}`).join('\n')}

Recent Team Ideas:
${teamIdeas.slice(0, 5).map(i => `👥 ${i.idea}`).join('\n')}

Trending Topics:
${this.extractTrendingTopics(ideas).slice(0, 5).map(topic => `• ${topic}`).join('\n')}`;

            return await this.summaryManager.createSummary(
                summary,
                SummaryPriority.High,
                context,
                ['ideas', 'innovation', 'creativity']
            );
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Idea summary generation failed')
            };
        }
    }

    async generateErrorSummary(errors: ErrorRecord[], context?: string): Promise<Result<string>> {
        try {
            const unresolvedErrors = errors.filter(e => !e.resolved);
            const criticalErrors = errors.filter(e => e.severity === 'critical');
            const recentErrors = errors
                .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
                .slice(0, 10);

            const errorCategories = errors.reduce((acc, e) => {
                acc[e.category] = (acc[e.category] || 0) + 1;
                return acc;
            }, {} as Record<string, number>);

            const summary = `Error Summary (${DateUtils.formatDate(new Date())}):
• Total Errors: ${errors.length}
• Unresolved: ${unresolvedErrors.length}
• Critical: ${criticalErrors.length}
• Resolution Rate: ${errors.length > 0 ? Math.round(((errors.length - unresolvedErrors.length) / errors.length) * 100) : 0}%

Error Categories:
${Object.entries(errorCategories).map(([category, count]) => `• ${category}: ${count}`).join('\n')}

Recent Critical Issues:
${criticalErrors.slice(0, 5).map(e => `🚨 ${e.title} (${e.severity})`).join('\n')}

Unresolved Issues:
${unresolvedErrors.slice(0, 5).map(e => `⚠️ ${e.title} (${e.category})`).join('\n')}`;

            return await this.summaryManager.createSummary(
                summary,
                unresolvedErrors.length > 0 ? SummaryPriority.Critical : SummaryPriority.Medium,
                context,
                ['errors', 'issues', 'monitoring']
            );
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Error summary generation failed')
            };
        }
    }

    async generateComprehensiveSummary(
        data: {
            tasks?: Task[];
            memories?: Memory[];
            ideas?: Idea[];
            errors?: ErrorRecord[];
        },
        context?: string
    ): Promise<Result<string>> {
        try {
            const summaries: string[] = [];

            if (data.tasks && data.tasks.length > 0) {
                const taskResult = await this.generateTaskSummary(data.tasks, context);
                if (taskResult.ok) summaries.push(taskResult.value);
            }

            if (data.memories && data.memories.length > 0) {
                const memoryResult = await this.generateMemorySummary(data.memories, context);
                if (memoryResult.ok) summaries.push(memoryResult.value);
            }

            if (data.ideas && data.ideas.length > 0) {
                const ideaResult = await this.generateIdeaSummary(data.ideas, context);
                if (ideaResult.ok) summaries.push(ideaResult.value);
            }

            if (data.errors && data.errors.length > 0) {
                const errorResult = await this.generateErrorSummary(data.errors, context);
                if (errorResult.ok) summaries.push(errorResult.value);
            }

            const comprehensiveSummary = `Comprehensive System Summary (${DateUtils.formatDate(new Date())})

${summaries.join('\n\n---\n\n')}

System Health Overview:
• Tasks: ${data.tasks?.length || 0}
• Memories: ${data.memories?.length || 0}
• Ideas: ${data.ideas?.length || 0}
• Errors: ${data.errors?.length || 0}

Generated at: ${new Date().toISOString()}`;

            return await this.summaryManager.createSummary(
                comprehensiveSummary,
                SummaryPriority.Critical,
                context,
                ['comprehensive', 'system', 'overview', 'health']
            );
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Comprehensive summary generation failed')
            };
        }
    }

    private extractTrendingTopics(ideas: Idea[]): string[] {
        const topicCounts = new Map<string, number>();

        ideas.forEach(idea => {
            // Simple topic extraction from idea content
            const words = idea.idea.toLowerCase().split(/\s+/);
            words.forEach(word => {
                if (word.length > 3) { // Only consider meaningful words
                    topicCounts.set(word, (topicCounts.get(word) || 0) + 1);
                }
            });

            // Also consider tags as topics
            idea.tags.forEach(tag => {
                topicCounts.set(tag, (topicCounts.get(tag) || 0) + 2); // Weight tags higher
            });
        });

        return Array.from(topicCounts.entries())
            .sort(([, a], [, b]) => b - a)
            .slice(0, 10)
            .map(([topic]) => topic);
    }

    getManager(): SummaryManager {
        return this.summaryManager;
    }
}

// Export singleton instances
export const summaryManager = SummaryManager.new();
export const summaryGenerator = new SummaryGenerator(summaryManager);
