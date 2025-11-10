import { Agent, QueueCollection, QueueItem, QueueSession, AssignmentStatus, AgentAssignment, Priority, Status } from '../todozi/models.js';

class AgentWorkflowManager {
    constructor(storage, embeddingService) {
        this.storage = storage;
        this.embeddingService = embeddingService;
        this.queue = new QueueCollection();
        this.activeAssignments = new Map();
    }

    /**
     * Create a specialized agent for a specific domain
     */
    async createSpecializedAgent(config) {
        const agent = Agent.new(config.id, config.name, config.description);
        
        // Configure agent capabilities
        agent.capabilities = config.capabilities || [];
        agent.specializations = config.specializations || [];
        agent.metadata.category = config.category || 'general';
        
        // Set up model configuration
        if (config.model) {
            agent.model = {
                provider: config.model.provider || 'anthropic',
                name: config.model.name || 'claude-3-opus-20240229',
                temperature: config.model.temperature || 0.2,
                maxTokens: config.model.maxTokens || 4096
            };
        }
        
        // Configure tools and behaviors
        if (config.tools) {
            agent.tools = config.tools.map(toolName => ({
                name: toolName,
                enabled: true,
                config: null
            }));
        }
        
        // Save the agent
        await this.storage.saveAgent(agent);
        console.log(`✅ Created specialized agent: ${agent.id}`);
        
        return agent;
    }

    /**
     * Analyze a task and determine the best assignment strategy
     */
    async analyzeTaskForAssignment(task) {
        const analysis = {
            recommendedAssignee: null,
            confidence: 0,
            reasoning: [],
            similarTasks: []
        };

        // Use embedding service to find similar completed tasks
        const similarTasks = await this.embeddingService.findSimilarTasks(
            task.action, 
            5
        );
        analysis.similarTasks = similarTasks;

        // Count assignments by type in similar tasks
        const assigneeCounts = {
            ai: 0,
            human: 0,
            agent: {}
        };

        for (const result of similarTasks) {
            const similarTask = result.task;
            if (similarTask.assignee) {
                if (similarTask.assignee === 'ai') {
                    assigneeCounts.ai++;
                } else if (similarTask.assignee === 'human') {
                    assigneeCounts.human++;
                } else if (similarTask.assignee.type === 'Agent') {
                    const agentName = similarTask.assignee.name;
                    assigneeCounts.agent[agentName] = 
                        (assigneeCounts.agent[agentName] || 0) + 1;
                }
            }
        }

        // Determine recommendation based on patterns
        if (assigneeCounts.ai > assigneeCounts.human && 
            Object.values(assigneeCounts.agent).every(count => count <= assigneeCounts.ai)) {
            analysis.recommendedAssignee = 'ai';
            analysis.confidence = assigneeCounts.ai / similarTasks.length;
            analysis.reasoning.push(`AI has handled ${assigneeCounts.ai} similar tasks`);
        } else if (assigneeCounts.human > assigneeCounts.ai && 
                   Object.values(assigneeCounts.agent).every(count => count <= assigneeCounts.human)) {
            analysis.recommendedAssignee = 'human';
            analysis.confidence = assigneeCounts.human / similarTasks.length;
            analysis.reasoning.push(`Humans have handled ${assigneeCounts.human} similar tasks`);
        } else {
            // Find the best specific agent
            const topAgent = Object.entries(assigneeCounts.agent)
                .sort((a, b) => b[1] - a[1])[0];
            
            if (topAgent && topAgent[1] > 0) {
                analysis.recommendedAssignee = { type: 'Agent', name: topAgent[0] };
                analysis.confidence = topAgent[1] / similarTasks.length;
                analysis.reasoning.push(`Agent ${topAgent[0]} has handled ${topAgent[1]} similar tasks`);
            } else {
                // Default to collaborative for complex tasks
                if (task.priority === Priority.Critical || task.priority === Priority.Urgent) {
                    analysis.recommendedAssignee = 'collaborative';
                    analysis.confidence = 0.5;
                    analysis.reasoning.push('High priority task suggests collaborative approach');
                } else {
                    analysis.recommendedAssignee = 'ai';
                    analysis.confidence = 0.3;
                    analysis.reasoning.push('Defaulting to AI for new task type');
                }
            }
        }

        // Check for specific keywords that might override the recommendation
        const actionLower = task.action.toLowerCase();
        if (actionLower.includes('review') || actionLower.includes('approve')) {
            analysis.recommendedAssignee = 'human';
            analysis.reasoning.push('Task involves review/approval - requires human oversight');
        } else if (actionLower.includes('debug') || actionLower.includes('fix bug')) {
            analysis.recommendedAssignee = { type: 'Agent', name: 'coder' };
            analysis.reasoning.push('Debugging task - specialized coder agent recommended');
        } else if (actionLower.includes('design') || actionLower.includes('ui')) {
            analysis.recommendedAssignee = { type: 'Agent', name: 'designer' };
            analysis.reasoning.push('Design task - specialized designer agent recommended');
        }

        return analysis;
    }

    /**
     * Assign a task to the appropriate handler and queue it
     */
    async assignAndQueueTask(task, forceAssignee = null) {
        // Determine the assignee
        let assignee = forceAssignee;
        if (!assignee) {
            const analysis = await this.analyzeTaskForAssignment(task);
            assignee = analysis.recommendedAssignee;
            console.log(`🤖 Task analysis: ${analysis.reasoning.join(', ')}`);
        }

        // Create queue items based on assignee type
        const queueItems = [];
        
        if (assignee === 'collaborative') {
            // Create separate queue items for AI and human
            const aiItem = QueueItem.new(
                `AI: ${task.action}`,
                `AI portion of collaborative task: ${task.contextNotes || task.action}`,
                task.priority,
                task.parentProject
            );
            
            const humanItem = QueueItem.new(
                `Human: ${task.action}`,
                `Human portion of collaborative task: ${task.contextNotes || task.action}`,
                task.priority,
                task.parentProject
            );
            
            queueItems.push(aiItem, humanItem);
            
            // Save both queue items
            this.queue.addItem(aiItem);
            this.queue.addItem(humanItem);
            
            // Start AI session immediately
            const aiSessionId = this.queue.startSession(aiItem.id);
            console.log(`🚀 Started AI session: ${aiSessionId}`);
            
        } else if (assignee === 'ai') {
            const item = QueueItem.new(
                `AI: ${task.action}`,
                task.contextNotes || `AI processing: ${task.action}`,
                task.priority,
                task.parentProject
            );
            queueItems.push(item);
            this.queue.addItem(item);
            
            // Start AI session immediately
            const sessionId = this.queue.startSession(item.id);
            console.log(`🚀 Started AI session: ${sessionId}`);
            
        } else if (assignee === 'human') {
            const item = QueueItem.new(
                `Human: ${task.action}`,
                task.contextNotes || task.action,
                task.priority,
                task.parentProject
            );
            queueItems.push(item);
            this.queue.addItem(item);
            console.log(`📋 Task queued for human action: ${item.id}`);
            
        } else if (assignee && assignee.type === 'Agent') {
            // Create agent assignment
            const assignment = new AgentAssignment({
                agentId: assignee.name,
                taskId: task.id,
                projectId: task.parentProject,
                status: AssignmentStatus.Assigned
            });
            
            // Save assignment
            await this.storage.saveAgentAssignment(assignment);
            this.activeAssignments.set(task.id, assignment);
            
            const item = QueueItem.new(
                `${assignee.name}: ${task.action}`,
                `Agent ${assignee.name} assigned to: ${task.action}`,
                task.priority,
                task.parentProject
            );
            queueItems.push(item);
            this.queue.addItem(item);
            
            console.log(`🤖 Task assigned to agent ${assignee.name}: ${task.id}`);
        }

        // Update task with assignment info
        const updates = {
            assignee: assignee,
            status: Status.InProgress
        };
        
        if (task.assignee === 'collaborative') {
            updates.contextNotes = 
                (task.contextNotes || '') + 
                `\n[Collaborative: AI Session ${queueItems[0].id}, Human Queue ${queueItems[1].id}]`;
        }
        
        await this.storage.updateTaskInProject(task.id, updates);
        
        return {
            task,
            assignee,
            queueItems,
            sessions: queueItems.map(item => this.queue.getSession(
                this.queue.getActiveSessions().find(s => s.queueItemId === item.id)?.id
            )).filter(Boolean)
        };
    }

    /**
     * Monitor and manage active queue sessions
     */
    async monitorActiveSessions() {
        const activeSessions = this.queue.getActiveSessions();
        
        console.log(`📊 Monitoring ${activeSessions.length} active sessions`);
        
        for (const session of activeSessions) {
            const duration = session.getCurrentDuration();
            const item = this.queue.getItem(session.queueItemId);
            
            // Check for sessions that have been running too long
            if (duration > 3600) { // 1 hour
                console.log(`⚠️ Long-running session detected: ${session.id} (${Math.floor(duration/60)} minutes)`);
                
                // Could implement logic to:
                // - Send notification
                // - Escalate to human
                // - Auto-timeout
            }
            
            // Check if AI sessions need results
            if (item.taskName.startsWith('AI:') && !item.taskDescription.includes('Result:')) {
                console.log(`🔄 Checking AI session ${session.id} for results...`);
                
                // Simulate AI completing the task
                if (Math.random() > 0.7) { // 30% chance of completion
                    const result = `AI completed: ${item.taskDescription}`;
                    item.taskDescription += `\nResult: ${result}`;
                    item.status = 'complete';
                    
                    await this.queue.endSession(session.id);
                    console.log(`✅ AI session completed: ${session.id}`);
                }
            }
        }
    }

    /**
     * Get workflow statistics and insights
     */
    async getWorkflowStats() {
        const allItems = this.queue.getAllItems();
        const activeSessions = this.queue.getActiveSessions();
        
        const stats = {
            queueItems: {
                total: allItems.length,
                backlog: this.queue.getBacklogItems().length,
                active: this.queue.getActiveItems().length,
                complete: this.queue.getCompleteItems().length
            },
            sessions: {
                active: activeSessions.length,
                totalDuration: activeSessions.reduce((sum, s) => sum + s.getCurrentDuration(), 0),
                avgDuration: activeSessions.length > 0 
                    ? activeSessions.reduce((sum, s) => sum + s.getCurrentDuration(), 0) / activeSessions.length
                    : 0
            },
            assignments: {
                active: this.activeAssignments.size,
                byAgent: {}
            }
        };

        // Count assignments by agent
        for (const assignment of this.activeAssignments.values()) {
            stats.assignments.byAgent[assignment.agentId] = 
                (stats.assignments.byAgent[assignment.agentId] || 0) + 1;
        }

        // Find bottlenecks
        const agentBacklog = {};
        for (const item of this.queue.getActiveItems()) {
            if (item.taskName.includes(':')) {
                const agentName = item.taskName.split(':')[0];
                agentBacklog[agentName] = (agentBacklog[agentName] || 0) + 1;
            }
        }

        if (Object.keys(agentBacklog).length > 0) {
            stats.bottlenecks = Object.entries(agentBacklog)
                .sort((a, b) => b[1] - a[1])
                .slice(0, 3);
        }

        return stats;
    }

    /**
     * Optimize queue based on priorities and agent availability
     */
    async optimizeQueue() {
        const backlogItems = this.queue.getBacklogItems();
        
        // Sort by priority (urgent first)
        backlogItems.sort((a, b) => {
            const priorityOrder = { urgent: 4, critical: 3, high: 2, medium: 1, low: 0 };
            return priorityOrder[b.priority] - priorityOrder[a.priority];
        });

        console.log(`🔧 Optimizing ${backlogItems.length} backlog items...`);
        
        // Recommend which items to start based on agent availability
        for (const item of backlogItems.slice(0, 5)) { // Check top 5
            if (item.taskName.startsWith('Human:')) {
                console.log(`💡 Ready for human pickup: ${item.id} - ${item.taskDescription}`);
            } else if (item.taskName.includes(':')) {
                const agentName = item.taskName.split(':')[0];
                const agentBacklog = this.queue.getActiveItems()
                    .filter(i => i.taskName.startsWith(agentName)).length;
                
                if (agentBacklog < 3) { // Agent has capacity
                    console.log(`💡 Agent ${agentName} available for: ${item.id}`);
                }
            }
        }
    }
}

// Usage example
async function demonstrateAgentWorkflow() {
    const workflowManager = new AgentWorkflowManager(/* storage, embeddingService */);
    
    // Create specialized agents
    await workflowManager.createSpecializedAgent({
        id: 'security-reviewer',
        name: 'Security Reviewer',
        description: 'Specialized agent for security code reviews and vulnerability analysis',
        category: 'security',
        capabilities: ['security_analysis', 'vulnerability_scan', 'compliance_check'],
        specializations: ['owasp', 'cryptographic', 'network_security'],
        model: {
            provider: 'anthropic',
            name: 'claude-3-opus-20240229',
            temperature: 0.1
        }
    });

    await workflowManager.createSpecializedAgent({
        id: 'performance-optimizer',
        name: 'Performance Optimizer',
        description: 'Agent specialized in code performance analysis and optimization',
        category: 'performance',
        capabilities: ['performance_analysis', 'bottleneck_detection', 'optimization'],
        specializations: ['algorithms', 'database', 'memory_management']
    });

    // Example tasks to demonstrate the workflow
    const tasks = [
        {
            id: 'task_1',
            action: 'Review authentication module for security vulnerabilities',
            contextNotes: 'OAuth 2.0 implementation in microservices architecture',
            priority: Priority.Critical,
            parentProject: 'security-audit'
        },
        {
            id: 'task_2',
            action: 'Optimize database queries in reporting module',
            contextNotes: 'Reports taking >30 seconds to generate',
            priority: Priority.High,
            parentProject: 'performance'
        },
        {
            id: 'task_3',
            action: 'Design new user dashboard UI',
            contextNotes: 'Modern, responsive design with dark mode support',
            priority: Priority.Medium,
            parentProject: 'frontend'
        }
    ];

    // Process each task through the workflow
    for (const taskData of tasks) {
        console.log(`\n📋 Processing task: ${taskData.action}`);
        
        const result = await workflowManager.assignAndQueueTask(taskData);
        
        console.log(`  Assigned to: ${JSON.stringify(result.assignee)}`);
        console.log(`  Queue items created: ${result.queueItems.length}`);
        
        if (result.sessions.length > 0) {
            console.log(`  Active sessions: ${result.sessions.map(s => s.id).join(', ')}`);
        }
    }

    // Monitor workflow
    console.log('\n📊 Workflow Statistics:');
    const stats = await workflowManager.getWorkflowStats();
    
    console.log(`  Queue Items: ${stats.queueItems.total} total`);
    console.log(`    - Backlog: ${stats.queueItems.backlog}`);
    console.log(`    - Active: ${stats.queueItems.active}`);
    console.log(`    - Complete: ${stats.queueItems.complete}`);
    
    console.log(`  Sessions: ${stats.sessions.active} active`);
    console.log(`    - Avg Duration: ${Math.floor(stats.sessions.avgDuration)}s`);
    
    if (stats.bottlenecks) {
        console.log('\n⚠️  Current Bottlenecks:');
        for (const [agent, count] of stats.bottlenecks) {
            console.log(`    ${agent}: ${count} items`);
        }
    }

    // Optimize queue
    console.log('\n🔧 Queue Optimization:');
    await workflowManager.optimizeQueue();
    
    // Monitor active sessions
    console.log('\n📡 Session Monitoring:');
    await workflowManager.monitorActiveSessions();
}

// Run the demonstration
demonstrateAgentWorkflow().catch(console.error);

/ *
# Example 3: Todozi Agent Assignment and Queue Management

This example demonstrates how to use Todozi's agent assignment and queue management system to automatically route tasks to appropriate handlers (AI, human, or specific agents) and track their progress through work queues.

## Key Features Demonstrated:

1. **Agent Creation**: Specialized agents for different domains (security, performance, etc.)

2. **Task Analysis**: Uses embedding similarity to analyze past task completions and recommend optimal assignees

3. **Intelligent Assignment**: Automatically routes tasks to AI, human, collaborative, or specific agents based on:
   - Historical patterns from similar tasks
   - Task keywords and context
   - Priority levels

4. **Queue Management**: 
   - Creates separate queue items for collaborative tasks
   - Tracks session durations and progress
   - Monitors for bottlenecks and long-running sessions

5. **Workflow Monitoring**:
   - Real-time statistics on queue status
   - Agent workload tracking
   - Bottleneck identification
   - Optimization recommendations

6. **Session Handling**:
   - Automatic session creation for AI tasks
   - Duration monitoring and timeout detection
   - Result collection and completion tracking

This example shows how Todozi can intelligently manage complex workflows by combining semantic search, pattern recognition, and intelligent task routing to optimize task completion across different types of workers (AI, humans, and specialized agents).