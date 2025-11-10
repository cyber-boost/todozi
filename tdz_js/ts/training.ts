import {
    TrainingData,
    TrainingDataClass,
    TrainingDataType,
    TodoziError,
    Result,
    parseTrainingDataType
} from './models.js';
import { DateUtils, ArrayUtils } from './utils.js';

// Training data management functionality
export class TrainingDataManager {
    private trainingData: Map<string, TrainingData> = new Map();

    constructor() {
        this.trainingData = new Map();
    }

    static new(): TrainingDataManager {
        return new TrainingDataManager();
    }

    async createTrainingData(
        dataType: TrainingDataType | string,
        prompt: string,
        completion: string,
        source: string,
        context?: string,
        tags: string[] = [],
        qualityScore?: number
    ): Promise<Result<string>> {
        try {
            let parsedType: TrainingDataType;

            if (typeof dataType === 'string') {
                const parsed = parseTrainingDataType(dataType);
                if (parsed instanceof TodoziError) return parsed;
                parsedType = parsed;
            } else {
                parsedType = dataType;
            }

            const trainingData = new TrainingDataClass(
                undefined,
                parsedType,
                prompt,
                completion,
                context,
                tags,
                qualityScore,
                source
            );

            this.trainingData.set(trainingData.id, trainingData);
            return { ok: true, value: trainingData.id };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Training data creation failed')
            };
        }
    }

    getTrainingData(id: string): TrainingData | undefined {
        return this.trainingData.get(id);
    }

    getAllTrainingData(): TrainingData[] {
        return Array.from(this.trainingData.values());
    }

    async updateTrainingData(
        id: string,
        updates: Partial<{
            dataType: TrainingDataType | string;
            prompt: string;
            completion: string;
            context: string;
            tags: string[];
            qualityScore: number;
            source: string;
        }>
    ): Promise<Result<void>> {
        const trainingData = this.trainingData.get(id);
        if (!trainingData) {
            return {
                ok: false,
                error: TodoziError.notFound(`Training data ${id} not found`)
            };
        }

        try {
            if (updates.dataType !== undefined) {
                let parsedType: TrainingDataType;
                if (typeof updates.dataType === 'string') {
                    const parsed = parseTrainingDataType(updates.dataType);
                    if (parsed instanceof TodoziError) return parsed;
                    parsedType = parsed;
                } else {
                    parsedType = updates.dataType;
                }
                (trainingData as TrainingDataClass).dataType = parsedType;
            }

            if (updates.prompt !== undefined) {
                (trainingData as TrainingDataClass).prompt = updates.prompt;
            }

            if (updates.completion !== undefined) {
                (trainingData as TrainingDataClass).completion = updates.completion;
            }

            if (updates.context !== undefined) {
                (trainingData as TrainingDataClass).context = updates.context;
            }

            if (updates.tags !== undefined) {
                (trainingData as TrainingDataClass).tags = updates.tags;
            }

            if (updates.qualityScore !== undefined) {
                (trainingData as TrainingDataClass).qualityScore = updates.qualityScore;
            }

            if (updates.source !== undefined) {
                (trainingData as TrainingDataClass).source = updates.source;
            }

            (trainingData as TrainingDataClass).updatedAt = new Date();
            this.trainingData.set(id, trainingData);

            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Training data update failed')
            };
        }
    }

    async deleteTrainingData(id: string): Promise<Result<void>> {
        if (!this.trainingData.has(id)) {
            return {
                ok: false,
                error: TodoziError.notFound(`Training data ${id} not found`)
            };
        }

        this.trainingData.delete(id);
        return { ok: true, value: undefined };
    }

    getTrainingDataByType(dataType: TrainingDataType): TrainingData[] {
        return Array.from(this.trainingData.values()).filter(td => td.dataType === dataType);
    }

    getTrainingDataBySource(source: string): TrainingData[] {
        return Array.from(this.trainingData.values()).filter(td => td.source === source);
    }

    getTrainingDataByTag(tag: string): TrainingData[] {
        return Array.from(this.trainingData.values()).filter(td => td.tags.includes(tag));
    }

    getTrainingDataByQuality(minQuality?: number, maxQuality?: number): TrainingData[] {
        return Array.from(this.trainingData.values()).filter(td => {
            if (td.qualityScore === undefined) return minQuality === undefined;
            if (minQuality !== undefined && td.qualityScore < minQuality) return false;
            if (maxQuality !== undefined && td.qualityScore > maxQuality) return false;
            return true;
        });
    }

    getRecentTrainingData(limit: number = 50): TrainingData[] {
        return Array.from(this.trainingData.values())
            .sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime())
            .slice(0, limit);
    }

    getTrainingDataByDateRange(from: Date, to: Date): TrainingData[] {
        return Array.from(this.trainingData.values()).filter(td =>
            td.createdAt >= from && td.createdAt <= to
        );
    }

    searchTrainingData(query: string): TrainingData[] {
        const lowerQuery = query.toLowerCase();
        return Array.from(this.trainingData.values()).filter(td =>
            td.prompt.toLowerCase().includes(lowerQuery) ||
            td.completion.toLowerCase().includes(lowerQuery) ||
            td.context?.toLowerCase().includes(lowerQuery) ||
            td.tags.some(tag => tag.toLowerCase().includes(lowerQuery)) ||
            td.source.toLowerCase().includes(lowerQuery)
        );
    }

    getAllTags(): string[] {
        const tags = new Set<string>();
        this.trainingData.forEach(td => {
            td.tags.forEach(tag => tags.add(tag));
        });
        return Array.from(tags);
    }

    getTagStatistics(): Map<string, number> {
        const stats = new Map<string, number>();
        this.trainingData.forEach(td => {
            td.tags.forEach(tag => {
                stats.set(tag, (stats.get(tag) || 0) + 1);
            });
        });
        return stats;
    }

    getSourceStatistics(): Map<string, number> {
        const stats = new Map<string, number>();
        this.trainingData.forEach(td => {
            stats.set(td.source, (stats.get(td.source) || 0) + 1);
        });
        return stats;
    }

    getTrainingDataStatistics(): {
        totalTrainingData: number;
        trainingDataByType: Record<TrainingDataType, number>;
        averageQualityScore: number;
        totalTags: number;
        totalSources: number;
        qualityDistribution: {
            high: number; // >= 0.8
            medium: number; // 0.6-0.8
            low: number; // < 0.6
            unrated: number;
        };
    } {
        const totalTrainingData = this.trainingData.size;
        const trainingDataByType = {
            [TrainingDataType.Instruction]: 0,
            [TrainingDataType.Completion]: 0,
            [TrainingDataType.Conversation]: 0,
            [TrainingDataType.Code]: 0,
            [TrainingDataType.Analysis]: 0,
            [TrainingDataType.Planning]: 0,
            [TrainingDataType.Review]: 0,
            [TrainingDataType.Documentation]: 0,
            [TrainingDataType.Example]: 0,
            [TrainingDataType.Test]: 0,
            [TrainingDataType.Validation]: 0
        };

        let totalQualityScore = 0;
        let ratedItems = 0;
        let totalTags = 0;
        const qualityDistribution = { high: 0, medium: 0, low: 0, unrated: 0 };

        this.trainingData.forEach(td => {
            trainingDataByType[td.dataType]++;
            totalTags += td.tags.length;

            if (td.qualityScore !== undefined) {
                totalQualityScore += td.qualityScore;
                ratedItems++;

                if (td.qualityScore >= 0.8) {
                    qualityDistribution.high++;
                } else if (td.qualityScore >= 0.6) {
                    qualityDistribution.medium++;
                } else {
                    qualityDistribution.low++;
                }
            } else {
                qualityDistribution.unrated++;
            }
        });

        return {
            totalTrainingData,
            trainingDataByType,
            averageQualityScore: ratedItems > 0 ? totalQualityScore / ratedItems : 0,
            totalTags,
            totalSources: this.getSourceStatistics().size,
            qualityDistribution
        };
    }

    // Bulk operations
    async bulkCreateTrainingData(
        data: Array<{
            dataType: TrainingDataType | string;
            prompt: string;
            completion: string;
            source: string;
            context?: string;
            tags?: string[];
            qualityScore?: number;
        }>
    ): Promise<Result<string[]>> {
        const results: string[] = [];
        const errors: TodoziError[] = [];

        for (const item of data) {
            const result = await this.createTrainingData(
                item.dataType,
                item.prompt,
                item.completion,
                item.source,
                item.context,
                item.tags,
                item.qualityScore
            );

            if (result.ok) {
                results.push(result.value);
            } else {
                errors.push(result.error);
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(
                    `Bulk creation failed: ${errors.length} errors out of ${data.length} items`
                )
            };
        }

        return { ok: true, value: results };
    }

    async bulkDeleteTrainingData(ids: string[]): Promise<Result<void>> {
        const errors: string[] = [];

        for (const id of ids) {
            const result = await this.deleteTrainingData(id);
            if (!result.ok) {
                errors.push(id);
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(
                    `Bulk deletion failed for IDs: ${errors.join(', ')}`
                )
            };
        }

        return { ok: true, value: undefined };
    }

    // Export functionality
    exportToJSON(): string {
        const data = Array.from(this.trainingData.values()).map(td => ({
            id: td.id,
            dataType: td.dataType,
            prompt: td.prompt,
            completion: td.completion,
            context: td.context,
            tags: td.tags,
            qualityScore: td.qualityScore,
            source: td.source,
            createdAt: td.createdAt.toISOString(),
            updatedAt: td.updatedAt.toISOString()
        }));

        return JSON.stringify(data, null, 2);
    }

    exportToCSV(): string {
        const headers = [
            'id',
            'dataType',
            'prompt',
            'completion',
            'context',
            'tags',
            'qualityScore',
            'source',
            'createdAt',
            'updatedAt'
        ];

        const rows = Array.from(this.trainingData.values()).map(td => [
            td.id,
            td.dataType,
            `"${td.prompt.replace(/"/g, '""')}"`,
            `"${td.completion.replace(/"/g, '""')}"`,
            td.context ? `"${td.context.replace(/"/g, '""')}"` : '',
            `"${td.tags.join(';')}"`,
            td.qualityScore?.toString() || '',
            td.source,
            td.createdAt.toISOString(),
            td.updatedAt.toISOString()
        ]);

        return [headers.join(','), ...rows.map(row => row.join(','))].join('\n');
    }

    // Import functionality
    async importFromJSON(jsonData: string): Promise<Result<number>> {
        try {
            const data = JSON.parse(jsonData);
            if (!Array.isArray(data)) {
                return {
                    ok: false,
                    error: TodoziError.validation('Invalid JSON format: expected array')
                };
            }

            let importedCount = 0;
            for (const item of data) {
                const result = await this.createTrainingData(
                    item.dataType,
                    item.prompt,
                    item.completion,
                    item.source,
                    item.context,
                    item.tags || [],
                    item.qualityScore
                );

                if (result.ok) {
                    importedCount++;
                }
            }

            return { ok: true, value: importedCount };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'JSON import failed')
            };
        }
    }

    // Quality assessment
    async assessQuality(id: string, newScore: number): Promise<Result<void>> {
        if (newScore < 0 || newScore > 1) {
            return {
                ok: false,
                error: TodoziError.validation('Quality score must be between 0 and 1')
            };
        }

        return await this.updateTrainingData(id, { qualityScore: newScore });
    }

    // Data validation and cleanup
    async validateTrainingData(): Promise<Result<{
        validItems: number;
        invalidItems: number;
        issues: string[];
    }>> {
        const issues: string[] = [];
        let validItems = 0;
        let invalidItems = 0;

        for (const [id, td] of this.trainingData.entries()) {
            let isValid = true;

            if (!td.prompt || td.prompt.trim().length === 0) {
                issues.push(`Training data ${id}: empty prompt`);
                isValid = false;
            }

            if (!td.completion || td.completion.trim().length === 0) {
                issues.push(`Training data ${id}: empty completion`);
                isValid = false;
            }

            if (!td.source || td.source.trim().length === 0) {
                issues.push(`Training data ${id}: empty source`);
                isValid = false;
            }

            if (td.qualityScore !== undefined && (td.qualityScore < 0 || td.qualityScore > 1)) {
                issues.push(`Training data ${id}: invalid quality score ${td.qualityScore}`);
                isValid = false;
            }

            if (isValid) {
                validItems++;
            } else {
                invalidItems++;
            }
        }

        return {
            ok: true,
            value: { validItems, invalidItems, issues }
        };
    }

    clear(): void {
        this.trainingData.clear();
    }
}

// Training data generator for creating synthetic training data
export class TrainingDataGenerator {
    private manager: TrainingDataManager;

    constructor(manager?: TrainingDataManager) {
        this.manager = manager || TrainingDataManager.new();
    }

    async generateFromConversations(
        conversations: Array<{
            messages: Array<{ role: string; content: string }>;
            context?: string;
            quality?: number;
        }>,
        source: string = 'conversation'
    ): Promise<Result<number>> {
        let generatedCount = 0;

        for (const conversation of conversations) {
            try {
                // Convert conversation to training format
                const prompt = this.formatConversationAsPrompt(conversation.messages);
                const completion = this.formatConversationAsCompletion(conversation.messages);

                if (prompt && completion) {
                    const result = await this.manager.createTrainingData(
                        TrainingDataType.Conversation,
                        prompt,
                        completion,
                        source,
                        conversation.context,
                        ['conversation', 'generated'],
                        conversation.quality
                    );

                    if (result.ok) {
                        generatedCount++;
                    }
                }
            } catch (error) {
                // Continue with other conversations
                console.warn('Failed to generate training data from conversation:', error);
            }
        }

        return { ok: true, value: generatedCount };
    }

    async generateFromTasks(
        tasks: Array<{
            action: string;
            context?: string;
            priority?: string;
            tags?: string[];
            quality?: number;
        }>,
        source: string = 'tasks'
    ): Promise<Result<number>> {
        let generatedCount = 0;

        for (const task of tasks) {
            try {
                const prompt = `Task: ${task.action}\n${task.context ? `Context: ${task.context}\n` : ''}Generate a structured approach to complete this task.`;

                const completion = `Priority: ${task.priority || 'medium'}\nTags: ${task.tags?.join(', ') || 'general'}\n\nApproach:\n1. Analyze the task requirements\n2. Break down into actionable steps\n3. Identify necessary resources\n4. Execute the plan\n5. Review and refine`;

                const result = await this.manager.createTrainingData(
                    TrainingDataType.Planning,
                    prompt,
                    completion,
                    source,
                    task.context,
                    task.tags || ['task', 'planning'],
                    task.quality
                );

                if (result.ok) {
                    generatedCount++;
                }
            } catch (error) {
                console.warn('Failed to generate training data from task:', error);
            }
        }

        return { ok: true, value: generatedCount };
    }

    private formatConversationAsPrompt(messages: Array<{ role: string; content: string }>): string | null {
        const userMessages = messages.filter(m => m.role === 'user');
        if (userMessages.length === 0) return null;

        return userMessages.map(m => m.content).join('\n\n');
    }

    private formatConversationAsCompletion(messages: Array<{ role: string; content: string }>): string | null {
        const assistantMessages = messages.filter(m => m.role === 'assistant');
        if (assistantMessages.length === 0) return null;

        return assistantMessages.map(m => m.content).join('\n\n');
    }

    getManager(): TrainingDataManager {
        return this.manager;
    }
}

// Export singleton instances
export const trainingDataManager = TrainingDataManager.new();
export const trainingDataGenerator = new TrainingDataGenerator(trainingDataManager);
