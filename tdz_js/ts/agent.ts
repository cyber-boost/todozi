import {
    Agent,
    AgentClass,
    AgentTool,
    AgentBehaviors,
    AgentConstraints,
    AgentMetadata,
    AgentStatus,
    ModelConfig,
    RateLimit,
    Result,
    Task,
    Memory,
    Idea,
    ErrorRecord
} from './models.js';
import { TodoziError } from './error.js';
import { DateUtils, ArrayUtils } from './utils.js';

// Agent management system
export class AgentManager {
    private agents: Map<string, Agent> = new Map();

    constructor() {
        this.agents = new Map();
    }

    static new(): AgentManager {
        return new AgentManager();
    }

    async createAgent(agentData: {
        id: string;
        name: string;
        description: string;
        version?: string;
        model?: Partial<ModelConfig>;
        systemPrompt?: string;
        promptTemplate?: string;
        capabilities?: string[];
        specializations?: string[];
        tools?: AgentTool[];
        behaviors?: Partial<AgentBehaviors>;
        constraints?: Partial<AgentConstraints>;
        metadata?: Partial<AgentMetadata>;
    }): Promise<Result<string>> {
        try {
            // Check if agent already exists
            if (this.agents.has(agentData.id)) {
                return {
                    ok: false,
                    error: TodoziError.validation(`Agent with ID ${agentData.id} already exists`)
                };
            }

            const agent = new AgentClass(
                agentData.id,
                agentData.name,
                agentData.description,
                agentData.version,
                agentData.model as ModelConfig,
                agentData.systemPrompt,
                agentData.promptTemplate,
                agentData.capabilities,
                agentData.specializations,
                agentData.tools,
                agentData.behaviors as AgentBehaviors,
                agentData.constraints as AgentConstraints,
                agentData.metadata as AgentMetadata
            );

            this.agents.set(agent.id, agent);
            return { ok: true, value: agent.id };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Agent creation failed')
            };
        }
    }

    getAgent(agentId: string): Agent | undefined {
        return this.agents.get(agentId);
    }

    getAllAgents(): Agent[] {
        return Array.from(this.agents.values());
    }

    async updateAgent(
        agentId: string,
        updates: Partial<{
            name: string;
            description: string;
            version: string;
            model: Partial<ModelConfig>;
            systemPrompt: string;
            promptTemplate: string;
            capabilities: string[];
            specializations: string[];
            tools: AgentTool[];
            behaviors: Partial<AgentBehaviors>;
            constraints: Partial<AgentConstraints>;
            metadata: Partial<AgentMetadata>;
        }>
    ): Promise<Result<void>> {
        const agent = this.agents.get(agentId);
        if (!agent) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        try {
            const agentClass = agent as AgentClass;

            if (updates.name !== undefined) agentClass.name = updates.name;
            if (updates.description !== undefined) agentClass.description = updates.description;
            if (updates.version !== undefined) agentClass.version = updates.version;
            if (updates.model !== undefined) {
                agentClass.model = { ...agentClass.model, ...updates.model };
            }
            if (updates.systemPrompt !== undefined) agentClass.systemPrompt = updates.systemPrompt;
            if (updates.promptTemplate !== undefined) agentClass.promptTemplate = updates.promptTemplate;
            if (updates.capabilities !== undefined) agentClass.capabilities = updates.capabilities;
            if (updates.specializations !== undefined) agentClass.specializations = updates.specializations;
            if (updates.tools !== undefined) agentClass.tools = updates.tools;
            if (updates.behaviors !== undefined) {
                agentClass.behaviors = { ...agentClass.behaviors, ...updates.behaviors };
            }
            if (updates.constraints !== undefined) {
                agentClass.constraints = { ...agentClass.constraints, ...updates.constraints };
            }
            if (updates.metadata !== undefined) {
                agentClass.metadata = { ...agentClass.metadata, ...updates.metadata };
            }

            agentClass.updatedAt = new Date();
            this.agents.set(agentId, agentClass);

            return { ok: true, value: undefined };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, 'Agent update failed')
            };
        }
    }

    async deleteAgent(agentId: string): Promise<Result<void>> {
        if (!this.agents.has(agentId)) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        this.agents.delete(agentId);
        return { ok: true, value: undefined };
    }

    getAgentsByStatus(status: AgentStatus): Agent[] {
        return Array.from(this.agents.values()).filter(agent => agent.metadata.status === status);
    }

    getAgentsByCapability(capability: string): Agent[] {
        return Array.from(this.agents.values()).filter(agent =>
            agent.capabilities.includes(capability)
        );
    }

    getAgentsBySpecialization(specialization: string): Agent[] {
        return Array.from(this.agents.values()).filter(agent =>
            agent.specializations.includes(specialization)
        );
    }

    getAvailableAgents(): Agent[] {
        return this.getAgentsByStatus(AgentStatus.Available);
    }

    getBusyAgents(): Agent[] {
        return this.getAgentsByStatus(AgentStatus.Busy);
    }

    async setAgentStatus(agentId: string, status: AgentStatus): Promise<Result<void>> {
        const agent = this.agents.get(agentId);
        if (!agent) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        (agent as AgentClass).metadata.status = status;
        (agent as AgentClass).updatedAt = new Date();

        return { ok: true, value: undefined };
    }

    getAgentsByCategory(category: string): Agent[] {
        return Array.from(this.agents.values()).filter(agent =>
            agent.metadata.category === category
        );
    }

    getAgentsByTag(tag: string): Agent[] {
        return Array.from(this.agents.values()).filter(agent =>
            agent.metadata.tags.includes(tag)
        );
    }

    getAllCategories(): string[] {
        const categories = new Set<string>();
        this.agents.forEach(agent => {
            categories.add(agent.metadata.category);
        });
        return Array.from(categories);
    }

    getAllTags(): string[] {
        const tags = new Set<string>();
        this.agents.forEach(agent => {
            agent.metadata.tags.forEach(tag => tags.add(tag));
        });
        return Array.from(tags);
    }

    getAgentStatistics(): {
        totalAgents: number;
        agentsByStatus: Record<AgentStatus, number>;
        agentsByCategory: Record<string, number>;
        totalCapabilities: number;
        totalSpecializations: number;
        averageCapabilitiesPerAgent: number;
        averageSpecializationsPerAgent: number;
    } {
        const totalAgents = this.agents.size;
        const agentsByStatus = {
            [AgentStatus.Active]: 0,
            [AgentStatus.Inactive]: 0,
            [AgentStatus.Busy]: 0,
            [AgentStatus.Available]: 0
        };

        const agentsByCategory: Record<string, number> = {};
        let totalCapabilities = 0;
        let totalSpecializations = 0;

        this.agents.forEach(agent => {
            agentsByStatus[agent.metadata.status]++;

            const category = agent.metadata.category;
            agentsByCategory[category] = (agentsByCategory[category] || 0) + 1;

            totalCapabilities += agent.capabilities.length;
            totalSpecializations += agent.specializations.length;
        });

        return {
            totalAgents,
            agentsByStatus,
            agentsByCategory,
            totalCapabilities,
            totalSpecializations,
            averageCapabilitiesPerAgent: totalAgents > 0 ? totalCapabilities / totalAgents : 0,
            averageSpecializationsPerAgent: totalAgents > 0 ? totalSpecializations / totalAgents : 0
        };
    }

    searchAgents(query: string): Agent[] {
        const lowerQuery = query.toLowerCase();
        return Array.from(this.agents.values()).filter(agent =>
            agent.name.toLowerCase().includes(lowerQuery) ||
            agent.description.toLowerCase().includes(lowerQuery) ||
            agent.capabilities.some(cap => cap.toLowerCase().includes(lowerQuery)) ||
            agent.specializations.some(spec => spec.toLowerCase().includes(lowerQuery)) ||
            agent.metadata.tags.some(tag => tag.toLowerCase().includes(lowerQuery))
        );
    }

    // Agent assignment methods
    async assignTaskToAgent(agentId: string, taskId: string, projectId: string): Promise<Result<void>> {
        const agent = this.agents.get(agentId);
        if (!agent) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        if (agent.metadata.status !== AgentStatus.Available) {
            return {
                ok: false,
                error: TodoziError.validation(`Agent ${agentId} is not available for assignment`)
            };
        }

        // Set agent status to busy
        await this.setAgentStatus(agentId, AgentStatus.Busy);

        // Here you would typically create an AgentAssignment record
        // For now, we'll just return success

        return { ok: true, value: undefined };
    }

    async releaseAgentFromTask(agentId: string): Promise<Result<void>> {
        const agent = this.agents.get(agentId);
        if (!agent) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        if (agent.metadata.status !== AgentStatus.Busy) {
            return {
                ok: false,
                error: TodoziError.validation(`Agent ${agentId} is not currently busy`)
            };
        }

        // Set agent status back to available
        await this.setAgentStatus(agentId, AgentStatus.Available);

        return { ok: true, value: undefined };
    }

    clear(): void {
        this.agents.clear();
    }
}

// Agent runner for executing agent tasks
export class AgentRunner {
    private manager: AgentManager;

    constructor(manager?: AgentManager) {
        this.manager = manager || AgentManager.new();
    }

    async executeTaskWithAgent(
        agentId: string,
        task: Task,
        context?: {
            memories?: Memory[];
            ideas?: Idea[];
            errors?: ErrorRecord[];
        }
    ): Promise<Result<{
        result: string;
        executionTime: number;
        metadata: Record<string, any>;
    }>> {
        const agent = this.manager.getAgent(agentId);
        if (!agent) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        if (!agent.capabilities.includes('task_execution')) {
            return {
                ok: false,
                error: TodoziError.validation(`Agent ${agentId} does not have task execution capability`)
            };
        }

        const startTime = Date.now();

        try {
            // Simulate agent execution (in a real implementation, this would call an AI service)
            const result = await this.simulateAgentExecution(agent, task, context);

            const executionTime = Date.now() - startTime;

            return {
                ok: true,
                value: {
                    result,
                    executionTime,
                    metadata: {
                        agentId,
                        taskId: task.id,
                        capabilities: agent.capabilities,
                        model: agent.model
                    }
                }
            };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Agent execution failed for task ${task.id}`)
            };
        }
    }

    async generateInsightsWithAgent(
        agentId: string,
        data: {
            tasks?: Task[];
            memories?: Memory[];
            ideas?: Idea[];
            errors?: ErrorRecord[];
        },
        insightType: 'summary' | 'analysis' | 'recommendations'
    ): Promise<Result<{
        insights: string[];
        executionTime: number;
        confidence: number;
    }>> {
        const agent = this.manager.getAgent(agentId);
        if (!agent) {
            return {
                ok: false,
                error: TodoziError.notFound(`Agent ${agentId} not found`)
            };
        }

        if (!agent.capabilities.includes('data_analysis')) {
            return {
                ok: false,
                error: TodoziError.validation(`Agent ${agentId} does not have data analysis capability`)
            };
        }

        const startTime = Date.now();

        try {
            const insights = await this.simulateInsightGeneration(agent, data, insightType);
            const executionTime = Date.now() - startTime;

            return {
                ok: true,
                value: {
                    insights,
                    executionTime,
                    confidence: 0.85 // Simulated confidence score
                }
            };
        } catch (error) {
            return {
                ok: false,
                error: TodoziError.fromError(error as Error, `Insight generation failed`)
            };
        }
    }

    private async simulateAgentExecution(
        agent: Agent,
        task: Task,
        context?: {
            memories?: Memory[];
            ideas?: Idea[];
            errors?: ErrorRecord[];
        }
    ): Promise<string> {
        // This is a simulation - in a real implementation, this would call an AI service
        await new Promise(resolve => setTimeout(resolve, 100)); // Simulate processing time

        const contextStr = context ? `
Context:
${context.memories?.length ? `- Memories: ${context.memories.length}` : ''}
${context.ideas?.length ? `- Ideas: ${context.ideas.length}` : ''}
${context.errors?.length ? `- Errors: ${context.errors.length}` : ''}
` : '';

        return `${agent.name} executed task "${task.action}" with the following result:

Task Details:
- Action: ${task.action}
- Priority: ${task.priority}
- Status: ${task.status}
- Project: ${task.parentProject}
${task.contextNotes ? `- Context: ${task.contextNotes}` : ''}
${contextStr}

Execution Result:
✅ Task analyzed and processed successfully
📊 Identified key requirements and dependencies
🎯 Generated actionable steps for completion
🔍 Provided detailed implementation guidance

The task has been processed using ${agent.model.provider} ${agent.model.name} model.
Execution completed with high confidence.`;
    }

    private async simulateInsightGeneration(
        agent: Agent,
        data: {
            tasks?: Task[];
            memories?: Memory[];
            ideas?: Idea[];
            errors?: ErrorRecord[];
        },
        insightType: 'summary' | 'analysis' | 'recommendations'
    ): Promise<string[]> {
        // This is a simulation - in a real implementation, this would call an AI service
        await new Promise(resolve => setTimeout(resolve, 150)); // Simulate processing time

        const insights: string[] = [];

        switch (insightType) {
            case 'summary':
                if (data.tasks?.length) {
                    insights.push(`Found ${data.tasks.length} tasks with ${data.tasks.filter(t => t.status === 'done').length} completed`);
                }
                if (data.memories?.length) {
                    insights.push(`Analyzed ${data.memories.length} memories, identifying ${data.memories.filter(m => m.importance === 'high').length} high-importance items`);
                }
                if (data.ideas?.length) {
                    insights.push(`Discovered ${data.ideas.length} ideas including ${data.ideas.filter(i => i.importance === 'breakthrough').length} breakthrough concepts`);
                }
                if (data.errors?.length) {
                    insights.push(`Identified ${data.errors.length} errors with ${data.errors.filter(e => !e.resolved).length} unresolved issues`);
                }
                break;

            case 'analysis':
                insights.push('Productivity patterns show peak performance in morning hours');
                insights.push('Error rates correlate with task complexity levels');
                insights.push('Memory retention improves with regular review cycles');
                insights.push('Idea generation peaks during collaborative sessions');
                break;

            case 'recommendations':
                insights.push('Implement daily task prioritization to improve completion rates');
                insights.push('Establish weekly memory review sessions for better retention');
                insights.push('Create dedicated time blocks for high-complexity tasks');
                insights.push('Set up automated error monitoring and alerting systems');
                break;
        }

        return insights;
    }

    getManager(): AgentManager {
        return this.manager;
    }
}

// Agent factory for creating common agent types
export class AgentFactory {
    static createCoderAgent(manager: AgentManager): Promise<Result<string>> {
        return manager.createAgent({
            id: 'coder',
            name: 'Code Assistant',
            description: 'Specialized in software development and programming tasks',
            capabilities: [
                'code_development',
                'code_review',
                'debugging',
                'refactoring',
                'testing',
                'documentation'
            ],
            specializations: [
                'rust', 'python', 'javascript', 'typescript',
                'go', 'sql', 'docker', 'kubernetes'
            ],
            tools: [
                { name: 'code_executor', enabled: true },
                { name: 'linter', enabled: true },
                { name: 'test_runner', enabled: true },
                { name: 'debugger', enabled: true }
            ],
            behaviors: {
                autoFormatCode: true,
                includeExamples: true,
                explainComplexity: true,
                suggestTests: true
            },
            constraints: {
                maxResponseLength: 10000,
                timeoutSeconds: 300,
                rateLimit: {
                    requestsPerMinute: 10,
                    tokensPerHour: 100000
                }
            },
            metadata: {
                author: 'system',
                tags: ['development', 'programming', 'technical'],
                category: 'technical',
                status: AgentStatus.Available
            },
            systemPrompt: 'You are an expert software developer with deep knowledge of multiple programming languages and best practices. Your role is to write clean, efficient, and well-documented code while following language-specific conventions and idioms.'
        });
    }

    static createAnalystAgent(manager: AgentManager): Promise<Result<string>> {
        return manager.createAgent({
            id: 'analyst',
            name: 'Data Analyst',
            description: 'Specialized in data analysis and insights generation',
            capabilities: [
                'data_analysis',
                'pattern_recognition',
                'trend_analysis',
                'reporting',
                'visualization'
            ],
            specializations: [
                'statistics', 'machine_learning', 'data_visualization',
                'predictive_modeling', 'business_intelligence'
            ],
            tools: [
                { name: 'data_processor', enabled: true },
                { name: 'visualizer', enabled: true },
                { name: 'statistical_analyzer', enabled: true }
            ],
            behaviors: {
                autoFormatCode: false,
                includeExamples: true,
                explainComplexity: true,
                suggestTests: false
            },
            metadata: {
                author: 'system',
                tags: ['analysis', 'data', 'insights'],
                category: 'analytical',
                status: AgentStatus.Available
            },
            systemPrompt: 'You are an expert data analyst skilled in identifying patterns, trends, and insights from complex datasets. You provide clear, actionable analysis with supporting evidence.'
        });
    }

    static createManagerAgent(manager: AgentManager): Promise<Result<string>> {
        return manager.createAgent({
            id: 'manager',
            name: 'Project Manager',
            description: 'Specialized in project management and task coordination',
            capabilities: [
                'task_management',
                'resource_allocation',
                'scheduling',
                'risk_assessment',
                'communication'
            ],
            specializations: [
                'agile', 'scrum', 'kanban', 'resource_planning',
                'stakeholder_management', 'quality_assurance'
            ],
            tools: [
                { name: 'task_tracker', enabled: true },
                { name: 'scheduler', enabled: true },
                { name: 'risk_analyzer', enabled: true }
            ],
            behaviors: {
                autoFormatCode: false,
                includeExamples: false,
                explainComplexity: false,
                suggestTests: false
            },
            metadata: {
                author: 'system',
                tags: ['management', 'coordination', 'planning'],
                category: 'managerial',
                status: AgentStatus.Available
            },
            systemPrompt: 'You are an expert project manager focused on efficient task coordination, resource optimization, and successful project delivery. You prioritize tasks and manage workflows effectively.'
        });
    }

    static async createStandardAgents(manager: AgentManager): Promise<Result<string[]>> {
        const results: string[] = [];
        const errors: TodoziError[] = [];

        const agentCreators = [
            () => this.createCoderAgent(manager),
            () => this.createAnalystAgent(manager),
            () => this.createManagerAgent(manager)
        ];

        for (const creator of agentCreators) {
            const result = await creator();
            if (result.ok) {
                results.push(result.value);
            } else {
                errors.push(result.error);
            }
        }

        if (errors.length > 0) {
            return {
                ok: false,
                error: TodoziError.validation(`Failed to create ${errors.length} standard agents`)
            };
        }

        return { ok: true, value: results };
    }
}

// Export singleton instances
export const agentManager = AgentManager.new();
export const agentRunner = new AgentRunner(agentManager);
