// example2.js - Agent Workflow Management
import { AgentManager } from '../todozi/agent.js';

async function runAgentWorkflowExample() {
    try {
        // Initialize agent manager
        const agentManager = new AgentManager();
        await agentManager.loadAgents();

        // Create a specialized coding agent
        const coderAgentId = await agentManager.createAgent({
            name: "Senior Coder",
            description: "Experienced software developer",
            capabilities: ["code_review", "debugging", "architecture"],
            specializations: ["javascript", "python", "system_design"],
            metadata: {
                status: models.AgentStatus.Available
            }
        });

        console.log(`Created coder agent with ID: ${coderAgentId}`);

        // Create a project management agent
        const pmAgentId = await agentManager.createAgent({
            name: "Project Manager",
            description: "Task coordinator and progress tracker",
            capabilities: ["planning", "scheduling", "resource_allocation"],
            specializations: ["agile", "scrum", "project_tracking"],
            metadata: {
                status: models.AgentStatus.Available
            }
        });

        console.log(`Created PM agent with ID: ${pmAgentId}`);

        // Get available agents
        const availableAgents = agentManager.getAvailableAgents();
        console.log(`\nAvailable agents: ${availableAgents.length}`);

        // Find best agent for a JavaScript task
        const bestAgent = agentManager.findBestAgent("javascript", "code_review");
        console.log(`Best agent for JS code review: ${bestAgent?.name || 'None found'}`);

        // Simulate task assignment
        const taskId = "task_12345";
        const projectId = "web_app_project";

        // Assign task to coder agent
        await agentManager.assignTaskToAgent(taskId, coderAgentId, projectId);
        console.log(`\nAssigned task ${taskId} to ${agentManager.getAgent(coderAgentId).name}`);

        // Check agent status
        console.log(`Coder agent status: ${agentManager.getAgent(coderAgentId).metadata.status}`);

        // Get assignments for the agent
        const coderAssignments = agentManager.getAgentAssignments(coderAgentId);
        console.log(`Coder has ${coderAssignments.length} assignment(s)`);

        // Get assignments for the task
        const taskAssignments = agentManager.getTaskAssignments(taskId);
        console.log(`Task has ${taskAssignments.length} assignment(s)`);

        // Complete the assignment
        await agentManager.completeAgentAssignment(taskId);
        console.log(`\nCompleted assignment for task ${taskId}`);

        // Verify agent is now available again
        console.log(`Coder agent status: ${agentManager.getAgent(coderAgentId).metadata.status}`);

        // Show agent statistics
        const stats = agentManager.getAgentStatistics();
        console.log(`\nAgent Statistics:`);
        console.log(`  Total Agents: ${stats.total_agents}`);
        console.log(`  Available: ${stats.available_agents}`);
        console.log(`  Busy: ${stats.busy_agents}`);
        console.log(`  Total Assignments: ${stats.total_assignments}`);
        console.log(`  Completion Rate: ${stats.completionRate().toFixed(2)}%`);

        // Update agent capabilities
        await agentManager.updateAgent(
            coderAgentId,
            new models.AgentUpdate()
                .capabilities(["code_review", "debugging", "architecture", "testing"])
                .specializations(["javascript", "python", "rust", "system_design"])
        );

        console.log(`\nUpdated coder agent capabilities and specializations`);

        // List all agents with their details
        console.log(`\nAll Agents:`);
        const allAgents = agentManager.getAllAgents();
        allAgents.forEach(agent => {
            console.log(`  - ${agent.name} (${agent.metadata.status})`);
            console.log(`    Specializations: ${agent.specializations.join(', ')}`);
            console.log(`    Capabilities: ${agent.capabilities.join(', ')}`);
        });

    } catch (error) {
        console.error('Error in agent workflow example:', error);
    }
}

// Run the example
runAgentWorkflowExample();

/*
Here's a practical example demonstrating how to use the AgentManager to create, assign, and manage AI agents for task execution:

This example demonstrates:

1. **Agent Creation**: Creating specialized agents with specific capabilities
2. **Agent Discovery**: Finding available agents and the best agent for a task
3. **Task Assignment**: Assigning tasks to agents and tracking assignments
4. **Status Management**: Updating agent statuses during task execution
5. **Statistics Tracking**: Monitoring agent performance and utilization
6. **Agent Updates**: Modifying agent capabilities and specializations
7. **Agent Listing**: Displaying all agents with their details

Key features shown:
- Creating agents with UUIDs and metadata
- Finding agents by specialization/capability
- Managing agent availability status
- Tracking task assignments
- Calculating completion statistics
- Updating agent properties
- Comprehensive agent listing

To run this example:
1. Save as `example2.js`
2. Ensure the required modules are available
3. Execute with: `node example2.js`

The output will show the complete agent workflow from creation to task assignment and completion, demonstrating how the AgentManager handles real-world agent coordination scenarios.
*/