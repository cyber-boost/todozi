import { MemoryManager } from '../todozi/memory.js';
import { TodoziEmbeddingService } from '../todozi/emb.js';
import { TagManager } from '../todozi/tags.js';

import { 
    Priority, 
    Status, 
    Assignee,
    MemoryImportance, 
    MemoryType
} from '../todozi/models.js';
import { parseTodoziFormat } from '../todozi/todozi.js';

class AdvancedTodoziWorkflow {
    constructor() {
        this.memoryManager = new MemoryManager();
        this.embeddingService = null;
        this.tagManager = TagManager.new();
        this.tasks = new Map();
        this.agentAssignments = new Map();
    }

    async initialize() {
        // Initialize the embedding service for semantic search
        this.embeddingService = await TodoziEmbeddingService.new();
        
        // Create some default tags
        await this.setupDefaultTags();
        
        console.log('🚀 Advanced Todozi Workflow initialized');
    }

    async setupDefaultTags() {
        const defaultTags = [
            { name: 'development', category: 'technical' },
            { name: 'documentation', category: 'knowledge' },
            { name: 'testing', category: 'quality' },
            { name: 'planning', category: 'management' },
            { name: 'urgent', category: 'priority' }
        ];

        for (const tag of defaultTags) {
            await this.tagManager.createTag(tag);
        }
    }

    async createTaskFromText(taskText) {
        try {
            // Parse the task using Todozi format
            const task = parseTodoziFormat(taskText);
            
            // Store the task
            this.tasks.set(task.id, task);
            
            // Generate semantic embedding for the task
            if (this.embeddingService) {
                const taskContent = this.prepareTaskContent(task);
                task.embeddingVector = await this.embeddingService.generateEmbedding(taskContent);
            }
            
            // Update tag usage
            for (const tagName of task.tags) {
                await this.tagManager.incrementTagUsage(tagName);
            }
            
            // Store a memory about creating this task
            await this.memoryManager.createMemory({
                moment: `Created task: ${task.action}`,
                meaning: 'Task was added to the system for tracking',
                reason: 'User action via workflow automation',
                importance: MemoryImportance.Medium,
                term: MemoryTerm.Short,
                memoryType: MemoryType.Standard,
                tags: task.tags
            });
            
            console.log(`✅ Task created: ${task.action} (ID: ${task.id})`);
            return task;
            
        } catch (error) {
            console.error(`❌ Failed to create task: ${error.message}`);
            throw error;
        }
    }

    async createIntelligentTask(description, context = {}) {
        // Use AI to suggest optimal task parameters
        const suggestedParams = await this.suggestTaskParameters(description, context);
        
        const taskText = `<todozi>${suggestedParams.action}; ${suggestedParams.time}; ${suggestedParams.priority}; ${suggestedParams.project}; ${suggestedParams.status}; ${suggestedParams.assignee}; ${suggestedParams.tags.join(',')}</todozi>`;
        
        const task = await this.createTaskFromText(taskText);
        
        // Store the reasoning as a memory
        await this.memoryManager.createMemory({
            moment: `AI-assisted task creation: ${task.action}`,
            meaning: 'System analyzed requirements and suggested optimal parameters',
            reason: `Based on analysis: ${suggestedParams.reasoning}`,
            importance: MemoryImportance.Medium,
            term: MemoryTerm.Short,
            memoryType: MemoryType.Human,
            tags: ['ai-assisted', 'automation', 'optimization']
        });
        
        return task;
    }

    async suggestTaskParameters(description, context) {
        // Analyze similar tasks using semantic search
        const similarTasks = await this.embeddingService.findSimilarTasks(description, 5);
        
        // Analyze the description for keywords
        const keywords = this.extractKeywords(description);
        
        // Suggest parameters based on analysis
        const suggestions = {
            action: description,
            time: this.estimateTime(description, similarTasks),
            priority: this.suggestPriority(keywords, similarTasks),
            project: context.project || 'default',
            status: Status.Todo,
            assignee: this.suggestAssignee(keywords, similarTasks),
            tags: this.suggestTags(keywords, similarTasks),
            reasoning: `Analyzed ${similarTasks.length} similar tasks and identified keywords: ${keywords.join(', ')}`
        };
        
        return suggestions;
    }

    extractKeywords(text) {
        const keywords = [];
        const keywordPatterns = {
            'urgent': ['urgent', 'asap', 'immediately', 'critical'],
            'development': ['code', 'develop', 'program', 'implement', 'build'],
            'testing': ['test', 'verify', 'validate', 'check', 'qa'],
            'documentation': ['document', 'write', 'explain', 'guide', 'manual'],
            'planning': ['plan', 'design', 'architecture', 'strategy', 'roadmap']
        };
        
        for (const [category, patterns] of Object.entries(keywordPatterns)) {
            if (patterns.some(pattern => text.toLowerCase().includes(pattern))) {
                keywords.push(category);
            }
        }
        
        return keywords;
    }

    estimateTime(description, similarTasks) {
        if (similarTasks.length === 0) return '2 hours';
        
        // Calculate average time from similar tasks
        const times = similarTasks.map(task => {
            const timeMatch = task.textContent.match(/(\d+)\s*(hour|hours|hr)/);
            return timeMatch ? parseInt(timeMatch[1]) : 2;
        });
        
        const avgTime = Math.round(times.reduce((a, b) => a + b, 0) / times.length);
        return `${avgTime} hours`;
    }

    suggestPriority(keywords, similarTasks) {
        if (keywords.includes('urgent')) return Priority.Critical;
        
        // Count priority distribution in similar tasks
        const priorityCounts = similarTasks.reduce((counts, task) => {
            const priority = task.priority || Priority.Medium;
            counts[priority] = (counts[priority] || 0) + 1;
            return counts;
        }, {});
        
        // Return the most common priority
        return Object.entries(priorityCounts)
            .sort(([,a], [,b]) => b - a)[0]?.[0] || Priority.Medium;
    }

    suggestAssignee(keywords, similarTasks) {
        if (keywords.includes('development') || keywords.includes('code')) {
            return Assignee.Agent('coder');
        }
        if (keywords.includes('testing') || keywords.includes('qa')) {
            return Assignee.Agent('tester');
        }
        if (keywords.includes('planning') || keywords.includes('design')) {
            return Assignee.Agent('planner');
        }
        
        // Default to collaborative for mixed requirements
        return Assignee.Collaborative;
    }

    suggestTags(keywords, similarTasks) {
        const tags = new Set(keywords);
        
        // Add tags from similar tasks
        for (const task of similarTasks) {
            if (task.tags) {
                task.tags.forEach(tag => tags.add(tag));
            }
        }
        
        return Array.from(tags).slice(0, 5); // Limit to 5 tags
    }

    async findRelatedTasks(taskId) {
        const task = this.tasks.get(taskId);
        if (!task) {
            throw new Error(`Task not found: ${taskId}`);
        }
        
        // Use semantic search to find related tasks
        if (this.embeddingService && task.embeddingVector) {
            const relatedTasks = await this.embeddingService.semanticSearch(
                this.prepareTaskContent(task),
                ['Task'],
                10
            );
            
            return relatedTasks
                .filter(result => result.content_id !== taskId)
                .map(result => ({
                    task: this.tasks.get(result.content_id),
                    similarity: result.similarity_score,
                    reason: 'Semantic similarity'
                }));
        }
        
        return [];
    }

    async createMemoryFromTask(task, memoryType, customContext = null) {
        const memory = {
            moment: `Task: ${task.action}`,
            meaning: `Task in project ${task.parent_project} with status ${task.status}`,
            reason: customContext || 'Automated memory creation from task state',
            importance: this.mapTaskPriorityToMemoryImportance(task.priority),
            term: MemoryTerm.Short,
            memoryType: memoryType,
            tags: task.tags
        };
        
        const memoryId = await this.memoryManager.createMemory(memory);
        console.log(`🧠 Memory created for task ${task.id}: ${memoryType}`);
        return memoryId;
    }

    mapTaskPriorityToMemoryImportance(taskPriority) {
        switch (taskPriority) {
            case Priority.Critical: return MemoryImportance.Critical;
            case Priority.High: return MemoryImportance.High;
            case Priority.Medium: return MemoryImportance.Medium;
            case Priority.Low: return MemoryImportance.Low;
            default: return MemoryImportance.Medium;
        }
    }

    prepareTaskContent(task) {
        let content = `Task: ${task.action}\n`;
        content += `Project: ${task.parent_project}\n`;
        content += `Priority: ${task.priority}\n`;
        content += `Status: ${task.status}\n`;
        
        if (task.context_notes) {
            content += `Context: ${task.context_notes}\n`;
        }
        
        if (task.tags.length > 0) {
            content += `Tags: ${task.tags.join(', ')}\n`;
        }
        
        return content;
    }

    async generateTaskInsights() {
        const allTasks = Array.from(this.tasks.values());
        const memories = this.memoryManager.getAllMemories();
        const tagStats = this.tagManager.getTagStatistics();
        
        const insights = {
            totalTasks: allTasks.length,
            tasksByStatus: this.groupTasksByStatus(allTasks),
            tasksByPriority: this.groupTasksByPriority(allTasks),
            topTags: this.tagManager.getMostUsedTags(5),
            memoriesByImportance: this.groupMemoriesByImportance(memories),
            recentMemories: this.memoryManager.getRecentMemories(10),
            tagStatistics: tagStats,
            recommendations: []
        };
        
        // Generate recommendations
        insights.recommendations = await this.generateRecommendations(insights);
        
        return insights;
    }

    groupTasksByStatus(tasks) {
        return tasks.reduce((groups, task) => {
            groups[task.status] = (groups[task.status] || 0) + 1;
            return groups;
        }, {});
    }

    groupTasksByPriority(tasks) {
        return tasks.reduce((groups, task) => {
            groups[task.priority] = (groups[task.priority] || 0) + 1;
            return groups;
        }, {});
    }

    groupMemoriesByImportance(memories) {
        return memories.reduce((groups, memory) => {
            groups[memory.importance] = (groups[memory.importance] || 0) + 1;
            return groups;
        }, {});
    }

    async generateRecommendations(insights) {
        const recommendations = [];
        
        // High priority overdue tasks
        if (insights.tasksByStatus[Status.Todo] > 5) {
            recommendations.push({
                type: 'workflow',
                message: `You have ${insights.tasksByStatus[Status.Todo]} pending tasks. Consider prioritizing or delegating.`,
                priority: 'medium'
            });
        }
        
        // Critical memories needing attention
        const criticalMemories = insights.memoriesByImportance[MemoryImportance.Critical] || 0;
        if (criticalMemories > 3) {
            recommendations.push({
                type: 'memory',
                message: `You have ${criticalMemories} critical memories. Review them for potential action items.`,
                priority: 'high'
            });
        }
        
        // Tag management suggestions
        if (insights.tagStatistics.totalTags > 20) {
            recommendations.push({
                type: 'organization',
                message: `Consider merging similar tags to reduce complexity (${insights.tagStatistics.totalTags} tags currently).`,
                priority: 'low'
            });
        }
        
        return recommendations;
    }

    async completeTask(taskId, completionNotes = null) {
        const task = this.tasks.get(taskId);
        if (!task) {
            throw new Error(`Task not found: ${taskId}`);
        }
        
        // Update task status
        task.status = Status.Done;
        task.updatedAt = new Date().toISOString();
        
        // Create completion memory
        await this.memoryManager.createMemory({
            moment: `Completed task: ${task.action}`,
            meaning: `Task successfully completed in project ${task.parent_project}`,
            reason: completionNotes || 'Task marked as done by user',
            importance: MemoryImportance.Medium,
            term: MemoryTerm.Short,
            memoryType: MemoryType.Human,
            tags: ['completed', task.parent_project]
        });
        
        // Store in embedding service for future reference
        if (this.embeddingService) {
            const completionContent = `COMPLETED: ${this.prepareTaskContent(task)}\nNotes: ${completionNotes || 'No notes provided'}`;
            await this.embeddingService.generateEmbedding(completionContent);
        }
        
        console.log(`✅ Task completed: ${task.action}`);
        
        // Suggest next related tasks
        const relatedTasks = await this.findRelatedTasks(taskId);
        if (relatedTasks.length > 0) {
            console.log(`🔗 Related tasks you might want to work on next:`);
            relatedTasks.slice(0, 3).forEach(({ task, similarity }) => {
                console.log(`   • ${task.action} (${(similarity * 100).toFixed(1)}% similar)`);
            });
        }
        
        return task;
    }
}

// Example usage
async function demonstrateAdvancedWorkflow() {
    const workflow = new AdvancedTodoziWorkflow();
    await workflow.initialize();
    
    console.log('\n=== Creating Tasks ===');
    
    // Create tasks manually
    const task1 = await workflow.createTaskFromText(
        '<todozi>Implement user authentication system; 4 hours; high; web-app; todo; agent=coder; authentication,security,backend</todozi>'
    );
    
    const task2 = await workflow.createTaskFromText(
        '<todozi>Write API documentation; 2 hours; medium; web-app; todo; agent=coder; documentation,api</todozi>'
    );
    
    console.log('\n=== AI-Assisted Task Creation ===');
    
    // Let AI create and optimize a task
    const aiTask = await workflow.createIntelligentTask(
        'Need to test the payment integration before launch',
        { project: 'web-app' }
    );
    
    console.log('\n=== Finding Related Tasks ===');
    
    // Find tasks related to authentication
    const relatedToAuth = await workflow.findRelatedTasks(task1.id);
    console.log(`Found ${relatedToAuth.length} tasks related to authentication`);
    
    console.log('\n=== Creating Memories ===');
    
    // Create different types of memories
    await workflow.createMemoryFromTask(task1, MemoryType.Secret, 
        'This uses OAuth2 with JWT tokens - important security consideration');
    
    await workflow.createMemoryFromTask(task2, MemoryType.Standard);
    
    console.log('\n=== Task Completion ===');
    
    // Complete a task and get recommendations
    await workflow.completeTask(task1.id, 'OAuth2 integration complete with refresh token support');
    
    console.log('\n=== Generating Insights ===');
    
    // Get comprehensive insights
    const insights = await workflow.generateTaskInsights();
    console.log('\n📊 Workflow Insights:');
    console.log(`  Total Tasks: ${insights.totalTasks}`);
    console.log(`  Tasks by Status:`, insights.tasksByStatus);
    console.log(`  Tasks by Priority:`, insights.tasksByPriority);
    console.log(`  Top Tags:`, insights.topTags.map(t => t.name));
    
    if (insights.recommendations.length > 0) {
        console.log('\n💡 Recommendations:');
        insights.recommendations.forEach(rec => {
            console.log(`  [${rec.priority.toUpperCase()}] ${rec.message}`);
        });
    }
    
    console.log('\n✅ Advanced workflow demonstration complete!');
}

// Run the demonstration
// Run if executed directly
    demonstrateAdvancedWorkflow().catch(console.error);
}

    AdvancedTodoziWorkflow
};

/*
# Example 3: Advanced Todozi Task Management with Memory Integration

This example demonstrates how to use the Todozi system to create a comprehensive task management workflow with memory tracking, semantic search, and intelligent task assignment.

## Key Features Demonstrated

1. **Intelligent Task Creation**: The system analyzes task descriptions and suggests optimal parameters (time estimates, priority, assignee, tags) based on semantic similarity to existing tasks.

2. **Semantic Search Integration**: Uses embeddings to find related tasks and content, enabling intelligent task recommendations and organization.

3. **Memory-Task Integration**: Automatically creates memories when tasks are created and completed, maintaining a comprehensive record of the workflow.

4. **Tag Management**: Tracks tag usage, suggests relevant tags, and provides statistics for better organization.

5. **Workflow Analytics**: Generates insights about task distribution, completion patterns, and provides actionable recommendations.

6. **AI-Assisted Assignments**: Intelligently assigns tasks to specialized agents (coder, tester, planner) based on task requirements.

This example showcases how the Todozi system can be used to create an intelligent, self-organizing task management workflow that learns from patterns and provides valuable insights for productivity optimization.
*/