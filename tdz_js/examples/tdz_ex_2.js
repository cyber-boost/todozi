// example2.js - Using TDZ commands to manage tasks and agents


import { parseTdzCommand, executeTdzCommand } from '../todozi/tdz.js';
import { findTodozi } from '../todozi/tdz.js';

async function runExample() {
  // Example 1: Create a new task using TDZ command
  const taskCommand = `
    <tdz>
      create; task; 
      action=Implement user authentication;
      time=4 hours;
      priority=high;
      project=web-app;
      status=todo;
      assignee=agent:auth-service
    </tdz>
  `;

  // Example 2: Update an existing agent
  const agentCommand = `
    <tdz>
      update; agent; auth-service;
      capabilities=authentication,security;
      specializations=OAuth2,JWT
    </tdz>
  `;

  // Example 3: Search for tasks
  const searchCommand = `
    <tdz>
      search; tasks; user login
    </tdz>
  `;

  // Parse all commands
  const commands = parseTdzCommand(
    `${taskCommand}\n${agentCommand}\n${searchCommand}`
  );

  // Execute each command
  const baseUrl = 'http://localhost:8636'; // Your Todozi server URL
  const apiKey = 'your-api-key'; // Your API key

  for (const command of commands) {
    try {
      const result = await executeTdzCommand(command, baseUrl, apiKey);
      console.log('✅ Command executed successfully:');
      console.log(JSON.stringify(result, null, 2));
    } catch (error) {
      console.error('❌ Command execution failed:');
      console.error(error.message);
    }
  }
}

// Run the example
runExample().catch(console.error);

/*
Here's **Example 2** demonstrating how to use the `<tdz>` command format to interact with Todozi's API through the `tdz.js` module:

**Key Features Demonstrated:**

1. **Task Creation**:
   - Creates a high-priority authentication task
   - Assigns it to a specialized agent
   - Sets project and time estimates

2. **Agent Management**:
   - Updates an existing agent's capabilities
   - Specifies authentication specializations

3. **Content Search**:
   - Searches for tasks related to "user login"
   - Demonstrates query-based retrieval

4. **Command Processing**:
   - Parses multiple TDZ commands from text
   - Executes them against Todozi API
   - Handles both success and error cases

**To Use This Example:**
1. Save as `example2.js`
2. Update `baseUrl` to your Todozi server address
3. Replace `apiKey` with your valid API key
4. Run with: `node example2.js`

**Expected Output:**

/ *
✅ Command executed successfully:
{
  "id": "task_12345",
  "action": "Implement user authentication",
  "status": "created"
}
✅ Command executed successfully:
{
  "id": "agent_auth-service",
  "updated": true
}
✅ Command executed successfully:
{
  "results": [
    {
      "id": "task_67890",
      "action": "Design user login page"
    }
  ]
}
*/