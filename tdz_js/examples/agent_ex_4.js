// -------------------------------------------------
// 2️⃣  example‑4.js – a self‑contained demo
// -------------------------------------------------
/**
 * What the script does
 * -------------------
 * 1️⃣ Loads the Todozi storage layer (creates the folder
 *    structure if it doesn’t exist).
 * 2️⃣ Instantiates an `AgentManager` and loads any agents that
 *    are already on disk (the default agents are created
 *    automatically by `storage.createDefaultAgents()`).
 * 3️⃣ Creates a **new custom agent** (`reporter`) that will be
 *    used to produce a short status report.
 * 4️⃣ Parses a `<todozi_agent>` tag that you could have typed
 *    in a normal Todozi message, turning it into an assignment
 *    object.
 * 5️⃣ Calls `AgentManager.assignTaskToAgent()` – the agent’s
 *    status flips to *busy* and the assignment is stored in
 *    memory.
 * 6️⃣ Marks the assignment as complete, which automatically
 *    returns the agent to the *available* state.
 * 7️⃣ Prints a small statistics report (`AgentManager.getAgentStatistics()`).
 *
 * Run it:
 *   node example‑4.js
 */

import path from 'path';
import { fileURLToPath } from 'url';
import { v4 as uuidv4 } from 'uuid';
import { DateTime } from 'luxon';

import {
  AgentManager,
  AgentUpdate,
  parseAgentAssignmentFormat,
  AgentStatistics
} from '../todozi/agent.js';               // <-- the file you posted

import { Storage } from '../todozi/storage.js'; // storage wrapper used by AgentManager

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

(async () => {
  // -------------------------------------------------
  //  A. Initialise storage (creates ~/.todozi if missing)
  // -------------------------------------------------
  const storage = await Storage.new();   // reads config, creates default agents, etc.

  // -------------------------------------------------
  //  B. Initialise the manager & load existing agents
  // -------------------------------------------------
  const mgr = new AgentManager();
  await mgr.loadAgents();                // pulls every agent from storage into the map

  console.log('\n🧑‍💻  Existing agents in the system:');
  mgr.getAllAgents().forEach(a => {
    console.log(`   • ${a.id} – ${a.name} (status: ${a.metadata.status})`);
  });

  // -------------------------------------------------
  //  C. Create a **custom** agent (if it does not already exist)
  // -------------------------------------------------
  const customId = 'reporter';
  if (!mgr.getAgent(customId)) {
    // Minimal definition – you can add capabilities, model, etc.
    const newAgent = {
      id: customId,
      name: 'Daily‑Reporter',
      description: 'Creates a short status‑report for a project',
      capabilities: ['summarise', 'write'],
      specializations: ['project‑report'],
      metadata: {
        status: 'available',
        category: 'reporting',
        author: 'example‑4',
        tags: ['report', 'automation']
      },
      systemPrompt: `You are a concise reporter. Summarise the given project
                     status in ≤ 3 sentences, focusing on blockers and next steps.`,
      model: {
        provider: 'anthropic',
        name: 'claude-3-haiku-20240307',
        temperature: 0.1,
        maxTokens: 1024
      }
    };

    // Persist the agent (the same flow that `todozi agent create …` uses)
    await storage.saveAgent(newAgent);
    // Also add it to the in‑memory map
    mgr.agents.set(customId, { ...newAgent });
    console.log('\n✅  Custom agent “reporter” created and saved.');
  }

  // -------------------------------------------------
  //  D. Prepare a *real* Todozi task that we want the agent to handle
  // -------------------------------------------------
  const task = {
    id: uuidv4(),
    action: 'Generate a one‑sentence summary of the Q2 roadmap',
    time: '2h',
    priority: 'medium',
    parentProject: 'product',
    status: 'todo',
    assignee: null,
    tags: ['report', 'q2'],
    contextNotes: null,
    progress: null,
    created_at: DateTime.utc().toISO(),
    updated_at: DateTime.utc().toISO()
  };

  // Save the task – normally `todozi add …` would do this.
  await storage.addTaskToProject(task);
  console.log('\n🗒️  Task stored:');
  console.log(`   ${task.id} – ${task.action}`);

  // -------------------------------------------------
  //  E. Parse a `<todozi_agent>` tag (the same parser used
  //     by `todozi chat …` when it sees an assignment)
  // -------------------------------------------------
  const tagText = `<todozi_agent>
    ${customId} ; ${task.id} ; ${task.parentProject}
  </todozi_agent>`;

  const assignment = parseAgentAssignmentFormat(tagText);
  console.log('\n🔖  Parsed agent‑assignment tag:');
  console.log(assignment);

  // -------------------------------------------------
  //  F. Assign the task to the agent (status becomes “busy”)
  // -------------------------------------------------
  await mgr.assignTaskToAgent(
    assignment.task_id,
    assignment.agent_id,
    assignment.project_id
  );
  console.log('\n🚀  Assignment performed.');
  console.log(`   Agent ${assignment.agent_id} is now ${mgr.getAgent(customId).metadata.status}`);

  // -------------------------------------------------
  //  G. (Optional) Simulate the AI doing its work …
  // -------------------------------------------------
  // In a real system the agent would now be called, generate
  // an answer and store the result somewhere. For the demo we
  // just wait a second.
  await new Promise(r => setTimeout(r, 1000));

  // -------------------------------------------------
  //  H. Mark the assignment as completed – the agent becomes “available”
  // -------------------------------------------------
  await mgr.completeAgentAssignment(assignment.task_id);
  console.log('\n✅  Assignment completed.');
  console.log(`   Agent ${customId} status → ${mgr.getAgent(customId).metadata.status}`);

  // -------------------------------------------------
  //  I. Show a quick statistics overview
  // -------------------------------------------------
  const stats = mgr.getAgentStatistics();   // → instance of AgentStatistics
  console.log('\n📊  Agent statistics');
  console.log(`   Total agents          : ${stats.total_agents}`);
  console.log(`   Available agents      : ${stats.available_agents}`);
  console.log(`   Busy agents           : ${stats.busy_agents}`);
  console.log(`   Inactive agents       : ${stats.inactive_agents}`);
  console.log(`   Total assignments     : ${stats.total_assignments}`);
  console.log(`   Completed assignments : ${stats.completed_assignments}`);
  console.log(`   Completion rate %     : ${stats.completionRate().toFixed(2)}%`);

  // -------------------------------------------------
  //  J. Clean‑up (optional) – delete the demo task & agent
  // -------------------------------------------------
  // await storage.deleteTaskFromProject(task.id);
  // await storage.deleteAgent(customId);
  // console.log('\n🧹  Demo clean‑up done.');
})().catch(err => {
  console.error('\n❌  Example failed:', err);
  process.exit(1);
});

/*
## Example 4 – Full **Agent Workflow**  
*Creating an agent, assigning a Todozi task to it (using the `<todozi_agent>` tag parser), completing the assignment and printing statistics.*

/ *
bash
# -------------------------------------------------
# 1️⃣  Install the library (once) – you already have it
# -------------------------------------------------
npm i uuid luxon

/ *
### What you’ll see when you run the script
*/