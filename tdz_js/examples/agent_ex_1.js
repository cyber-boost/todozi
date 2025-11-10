import { AgentManager, AgentUpdate, parseAgentAssignmentFormat } from './agent.js';

// Example demonstrating comprehensive agent management
async function demonstrateAgentWorkflow() {
    // Initialize the agent manager
    const agentManager = new AgentManager();
    await agentManager.loadAgents();
    
    console.log('🤖 AGENT MANAGEMENT DEMONSTRATION\n');
    
    // 1. Create specialized agents
    console.log('1. Creating specialized agents...');
    
    // Create a coding agent
    const coderAgentId = await agentManager.createAgent({
        name: 'Code Specialist',
        description: 'Expert in software development and code review',
        capabilities: ['coding', 'debugging', 'code_review'],
        specializations: ['javascript', 'python', 'rust'],
        metadata: { status: 'available' }
    });
    
    // Create a testing agent
    const testerAgentId = await agentManager.createAgent({
        name: 'Quality Assurance',
        description: 'Specialized in testing and quality assurance',
        capabilities: ['testing', 'qa', 'automation'],
        specializations: ['unit_tests', 'integration_tests'],
        metadata: { status: 'available' }
    });
    
    console.log(`✅ Created agents: ${coderAgentId}, ${testerAgentId}\n`);
    
    // 2. Update an agent's capabilities
    console.log('2. Updating agent capabilities...');
    
    const agentUpdate = AgentUpdate.new()
        .capabilities(['coding', 'debugging', 'code_review', 'architecture'])
        .specializations(['javascript', 'python', 'rust', 'typescript']);
    
    await agentManager.updateAgent(coderAgentId, agentUpdate);
    console.log('✅ Enhanced coder agent capabilities\n');
    
    // 3. Find the best agent for a task
    console.log('3. Finding best agent for JavaScript task...');
    
    const bestAgent = agentManager.findBestAgent('javascript', 'code_review');
    if (bestAgent) {
        console.log(`🎯 Best agent found: ${bestAgent.name} (${bestAgent.id})`);
        console.log(`   Capabilities: ${bestAgent.capabilities.join(', ')}`);
        console.log(`   Specializations: ${bestAgent.specializations.join(', ')}\n`);
    }
    
    // 4. Assign tasks to agents
    console.log('4. Assigning tasks to agents...');
    
    const taskId1 = 'fix_login_bug_001';
    const projectId = 'web_app_v2';
    
    await agentManager.assignTaskToAgent(taskId1, coderAgentId, projectId);
    console.log(`✅ Task "${taskId1}" assigned to coder agent\n`);
    
    // 5. Parse agent assignment from chat message
    console.log('5. Parsing agent assignment from chat message...');
    
    const chatMessage = `
    <todozi_agent>${testerAgentId};create_test_suite_001;${projectId}</todozi_agent>
    Please create comprehensive tests for the new authentication module.
    `;
    
    try {
        const assignment = parseAgentAssignmentFormat(chatMessage);
        console.log(`📋 Parsed assignment: Agent ${assignment.agent_id} → Task ${assignment.task_id}`);
        console.log(`   Project: ${assignment.project_id}\n`);
        
        // Execute the assignment
        await agentManager.assignTaskToAgent(
            assignment.task_id, 
            assignment.agent_id, 
            assignment.project_id
        );
    } catch (error) {
        console.log(`❌ Failed to parse assignment: ${error.message}\n`);
    }
    
    // 6. Check agent assignments and status
    console.log('6. Current agent assignments:');
    
    const coderAssignments = agentManager.getAgentAssignments(coderAgentId);
    const testerAssignments = agentManager.getAgentAssignments(testerAgentId);
    
    console.log(`📋 Coder agent assignments: ${coderAssignments.length}`);
    console.log(`📋 Tester agent assignments: ${testerAssignments.length}\n`);
    
    // 7. Complete a task and free up the agent
    console.log('7. Completing task and freeing agent...');
    
    await agentManager.completeAgentAssignment(taskId1);
    console.log(`✅ Task "${taskId1}" completed, agent status updated\n`);
    
    // 8. Get agent statistics
    console.log('8. Agent system statistics:');
    
    const stats = agentManager.getAgentStatistics();
    console.log(`📊 Total agents: ${stats.total_agents}`);
    console.log(`📊 Available agents: ${stats.available_agents}`);
    console.log(`📊 Busy agents: ${stats.busy_agents}`);
    console.log(`💯 Completion rate: ${stats.completionRate().toFixed(1)}%\n`);
    
    // 9. Search for agents by capability
    console.log('9. Searching for testing-capable agents:');
    
    const testingAgents = agentManager.getAgentsByCapability('testing');
    console.log(`🔍 Found ${testingAgents.length} testing agents:`);
    testingAgents.forEach(agent => {
        console.log(`   - ${agent.name}: ${agent.capabilities.join(', ')}`);
    });
}

// Run the demonstration
demonstrateAgentWorkflow().catch(console.error);

/*
Here's a practical example demonstrating agent management and task assignment using the AgentManager class:

## Example: Creating and Managing AI Agents for Task Assignment

## Expected Output:

/ *
🤖 AGENT MANAGEMENT DEMONSTRATION

1. Creating specialized agents...
✅ Created agents: agent_abc123, agent_def456

2. Updating agent capabilities...
✅ Enhanced coder agent capabilities

3. Finding best agent for JavaScript task...
🎯 Best agent found: Code Specialist (agent_abc123)
   Capabilities: coding, debugging, code_review, architecture
   Specializations: javascript, python, rust, typescript

4. Assigning tasks to agents...
✅ Task "fix_login_bug_001" assigned to coder agent

5. Parsing agent assignment from chat message...
📋 Parsed assignment: Agent agent_def456 → Task create_test_suite_001
   Project: web_app_v2

6. Current agent assignments:
📋 Coder agent assignments: 1
📋 Tester agent assignments: 1

7. Completing task and freeing agent...
✅ Task "fix_login_bug_001" completed, agent status updated

8. Agent system statistics:
📊 Total agents: 2
📊 Available agents: 2
📊 Busy agents: 0
💯 Completion rate: 50.0%

9. Searching for testing-capable agents:
🔍 Found 1 testing agents:
   - Quality Assurance: testing, qa, automation
*/