/**
import { Priority, Status, Assignee, TrainingDataType, QueueStatus, Task, TaskUpdate, QueueItem } from '../todozi/models.js';

import http from 'http';
import { TodoziServer } from '../todozi/server.js';
import { ServerConfig } from '../todozi/server.js';

 * Example 5 — Practical usage of TodoziServer, ApiKeys, Agents, and TrainingData
 *
 * What this example does:
 * 1) Create and register an API key pair.
 * 2) Start the Todozi server.
 * 3) Initialize the system.
 * 4) Create a custom agent and save it.
 * 5) Create training data.
 * 6) Create a task using the server API (with API key auth).
 * 7) Process a chat message that extracts tasks, memories, ideas, etc.
 * 8) Plan a queue item and start/end a queue session.
 * 9) Print system stats and various lists.
 *
 * Run:
 *   node example5.js
 *
 * Requirements:
 * - Node.js >= 16
 * - This file should be placed in the same project context as server.js
 */


// Import from the server and models
// --- Helper: minimal HTTP client for talking to our server ---
function request(method, path, headers = {}, body = null) {
  return new Promise((resolve, reject) => {
    const isJson = body !== null && !(body instanceof Buffer) && typeof body !== 'string';
    const payload = isJson ? JSON.stringify(body) : body;

    const requestHeaders = {
      'Content-Type': isJson ? 'application/json' : 'application/json',
      'Host': '127.0.0.1:8636',
      'Content-Length': payload ? Buffer.byteLength(payload) : 0,
      ...headers
    };

    const req = http.request({
      hostname: '127.0.0.1',
      port: 8636,
      method,
      path,
      headers: requestHeaders
    }, (res) => {
      const chunks = [];
      res.on('data', (chunk) => chunks.push(chunk));
      res.on('end', () => {
        const text = Buffer.concat(chunks).toString('utf8');
        try {
          const json = JSON.parse(text);
          resolve({ status: res.statusCode, headers: res.headers, body: json, raw: text });
        } catch {
          resolve({ status: res.statusCode, headers: res.headers, body: text, raw: text });
        }
      });
    });

    req.on('error', reject);
    if (payload) req.write(payload);
    req.end();
  });
}

// --- Demonstrate the API and server in a single script ---
(async function main() {
  // 1) Prepare server
  const config = new ServerConfig({ host: '127.0.0.1', port: 8636, max_connections: 50 });
  const server = await TodoziServer.new(config);
  await server.start();

  // Give the server a moment to fully start
  await new Promise(r => setTimeout(r, 200));

  // 2) Initialize system (agents, etc.)
  console.log('➤ Initialize system');
  const initResp = await request('GET', '/init');
  console.log('Init response:', initResp.body);

  // 3) Create API keys
  console.log('\n➤ Creating API key');
  const apiKeyResp = await request('POST', '/api/register');
  console.log('API Key:', apiKeyResp.body);

  const { public_key, private_key } = apiKeyResp.body;
  if (!public_key || !private_key) {
    console.error('Failed to create API key. Aborting.');
    process.exit(1);
  }

  const authHeaders = {
    'X-API-Key': public_key,
    'X-API-Private-Key': private_key
  };

  // 4) Create a custom agent
  console.log('\n➤ Creating a custom agent');
  const customAgent = {
    name: 'Doc Writer',
    description: 'Writes and reviews documentation based on tasks and ideas.',
    category: 'documentation'
  };
  const agentResp = await request('POST', '/agents', authHeaders, customAgent);
  console.log('Agent created:', agentResp.body);

  // 5) Create training data
  console.log('\n➤ Creating training data');
  const trainingItem = {
    data_type: 'instruction',
    prompt: 'Summarize the following task into a concise one-liner:',
    completion: 'Task summary: <one-liner>',
    context: 'Task summarization',
    tags: ['docs', 'summary'],
    quality_score: 0.9,
    source: 'example5'
  };
  const trainingResp = await request('POST', '/training', authHeaders, trainingItem);
  console.log('Training item created:', trainingResp.body);

  // 6) Create a task via the server API (authenticated)
  console.log('\n➤ Creating a task via API (authenticated)');
  const newTask = {
    action: 'Write onboarding docs for the SDK',
    time: '2 hours',
    priority: 'high',
    parent_project: 'general',
    status: 'todo'
  };
  const taskResp = await request('POST', '/tasks', authHeaders, newTask);
  console.log('Task created:', taskResp.body);

  // 7) Process a chat message (extracts tasks, memories, ideas, etc.)
  console.log('\n➤ Processing a chat message');
  const chatInput = {
    message:
      'Here is a mixed input:\n' +
      '<todozi>Draft API reference;1 day;high;docs;todo;human;documentation,api</todozi>\n' +
      '<memory>Meeting with QA;We agreed on test-first;process improvement;high;long;standard;meeting,qa</memory>\n' +
      '<idea>Auto-generate doc snippets from code;share;high;automation,docs</idea>\n' +
      '<train>instruction;Generate a helpful commit message;Follow conventional commits;git,workflow;0.8;example5</train>\n' +
      '<error>Build timeout;CI takes > 10m;high;performance;ci;build,timeout</error>\n' +
      '<feel>excited;9;Ready to ship docs!</feel>\n'
  };
  const chatResp = await request('POST', '/chat/process', authHeaders, chatInput);
  console.log('Chat processed:', chatResp.body);

  // 8) Plan a queue item and start/end a session
  console.log('\n➤ Planning a queue item');
  const planResp = await request('POST', '/queue/plan', authHeaders, {
    taskName: 'Review draft docs',
    taskDescription: 'Review and improve the drafted SDK documentation',
    priority: 'medium',
    projectId: 'docs'
  });
  console.log('Queue item planned:', planResp.body);

  const queueItemId = planResp.body?.id || planResp.body?.queue?.id;
  if (queueItemId) {
    console.log('\n➤ Starting a queue session');
    const startResp = await request('POST', `/queue/start/${queueItemId}`, authHeaders, {});
    console.log('Session started:', startResp.body);

    // Simulate some work
    await new Promise(r => setTimeout(r, 500));

    console.log('\n➤ Ending the queue session');
    const sessionId = startResp.body?.session_id || startResp.body?.id;
    const endResp = await request('POST', `/queue/end/${sessionId}`, authHeaders, {});
    console.log('Session ended:', endResp.body);
  }

  // 9) Fetch system stats and lists
  console.log('\n➤ System stats');
  const statsResp = await request('GET', '/stats');
  console.log('Stats:', statsResp.body);

  console.log('\n➤ Listing agents (all and available)');
  const agentsResp = await request('GET', '/agents');
  console.log('Agents:', agentsResp.body);

  const agentsAvailResp = await request('GET', '/agents/available');
  console.log('Available agents:', agentsAvailResp.body);

  console.log('\n➤ Listing all tasks');
  const tasksResp = await request('GET', '/tasks');
  console.log('Tasks:', tasksResp.body);

  console.log('\n➤ Listing training data and stats');
  const trainingListResp = await request('GET', '/training');
  console.log('Training data:', trainingListResp.body);

  const trainingStatsResp = await request('GET', '/training/stats');
  console.log('Training stats:', trainingStatsResp.body);

  console.log('\n➤ Queue lists (backlog, active, complete)');
  const backlogResp = await request('GET', '/queue/list/backlog');
  console.log('Backlog:', backlogResp.body);
  const activeResp = await request('GET', '/queue/list/active');
  console.log('Active:', activeResp.body);
  const completeResp = await request('GET', '/queue/list/complete');
  console.log('Complete:', completeResp.body);

  console.log('\n✅ Example 5 completed successfully.');
  process.exit(0);
})().catch((err) => {
  console.error('❌ Example 5 failed:', err);
  process.exit(1);
});