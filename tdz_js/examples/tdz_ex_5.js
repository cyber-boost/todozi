import http from 'http';

Below is a self-contained, executable Node.js example (Example 5) that uses tdz.js-style parsing and command execution to demonstrate processing multiple <tdz> tags. It includes a tiny mock API server so you can run it without any external services. The script:

- Parses a block of <tdz>...</tdz> commands.
- Builds the correct request per command (GET/POST/PUT).
- Sends them to a mock server.
- Prints the server’s JSON responses.

How to run:
- Requires Node.js v18+ (for global fetch).
- Save the code as example5.js and run: node example5.js

Code:
javascript
'use strict';

/**
 * Example 5: Parse and execute multiple <tdz> commands against a mock API.
 * - Demonstrates parsing, request building, routing, and basic server mocking.
 * - No external dependencies; uses Node's global fetch (Node 18+).
 */

/******************************
 * Minimal tdz.js core re-exports
 ******************************/
class TdzCommand {
  constructor(command, target, parameters, options) {
    this.command = command;
    this.target = target;
    this.parameters = parameters;
    this.options = options;
  }
}

class TodoziError extends Error {
  constructor(message, type = 'ValidationError') {
    super(message);
    this.name = 'TodoziError';
    this.type = type;
  }
}

function parseTdzCommand(text) {
  const commands = [];
  const re = /<tdz>(.*?)<\/tdz>/g;
  let match;

  while ((match = re.exec(text)) !== null) {
    const content = match[1].trim();
    const parts = content.split(';').map(s => s.trim());

    if (parts.length === 0) continue;

    const command = parts[0].toLowerCase();
    const target = parts[1] ? parts[1].toLowerCase() : '';
    const parameters = [];
    const options = {};

    for (let i = 2; i < parts.length; i++) {
      const part = parts[i];
      if (part.includes('=')) {
        const kv = part.split('=', 2);
        if (kv.length === 2) {
          options[kv[0].toLowerCase()] = kv[1];
        }
      } else {
        parameters.push(part);
      }
    }

    commands.push(new TdzCommand(command, target, parameters, options));
  }

  return commands;
}

function getEndpointPath(command) {
  const param0 = command.parameters[0] || '';
  const param1 = command.parameters[1] || '';

  switch (command.command) {
    case 'list':
    case 'get':
      switch (command.target) {
        case 'health': return '/health';
        case 'tasks':
          return command.command === 'list' ? '/tasks' : `/tasks/${param0}`;
        default:
          return `/${command.target}`;
      }
    case 'create':
      switch (command.target) {
        case 'task': return '/tasks';
        case 'idea': return '/ideas';
        default:
          return `/${command.target}`;
      }
    case 'update':
      if (command.target === 'task') return `/tasks/${param0}`;
      return `/${command.target}`;
    case 'search':
      if (command.target === 'tasks') return `/tasks/search?q=${param0}`;
      return `/${command.target}`;
    case 'run':
      switch (command.target) {
        case 'chat': return '/chat/process';
        default: return `/${command.target}`;
      }
    default:
      return `/${command.target}`;
  }
}

function buildRequestBody(command) {
  switch (command.target) {
    case 'task':
      return {
        action: command.options.action || '',
        time: command.options.time || '',
        priority: command.options.priority || '',
        project: command.options.project || '',
        status: command.options.status || '',
        assignee: command.options.assignee,
        tags: command.options.tags ? command.options.tags.split(',').map(s => s.trim()) : []
      };
    case 'idea':
      return {
        idea: command.options.idea || '',
        share: command.options.share || '',
        importance: command.options.importance || ''
      };
    default:
      return null;
  }
}

function buildRunBody(command) {
  switch (command.target) {
    case 'chat':
      return {
        message: command.options.message || '',
        context: command.options.context
      };
    default:
      return null;
  }
}

async function executeTdzCommand(command, baseUrl, apiKey) {
  const url = `${baseUrl.replace(/\/$/, '')}${getEndpointPath(command)}`;

  let requestOptions = {
    method: 'GET',
    headers: {}
  };

  switch (command.command) {
    case 'list':
    case 'get':
    case 'search':
      requestOptions.method = 'GET';
      break;
    case 'create':
      requestOptions.method = 'POST';
      break;
    case 'update':
      requestOptions.method = 'PUT';
      break;
    case 'run':
    case 'execute':
      requestOptions.method = 'POST';
      break;
    default:
      throw new TodoziError(`Unknown command: ${command.command}`);
  }

  if (apiKey) {
    requestOptions.headers['X-API-Key'] = apiKey;
  }

  if (command.command === 'create' || command.command === 'update') {
    const body = buildRequestBody(command);
    requestOptions.headers['Content-Type'] = 'application/json';
    requestOptions.body = JSON.stringify(body);
  } else if (command.command === 'run' || command.command === 'execute') {
    const body = buildRunBody(command);
    requestOptions.headers['Content-Type'] = 'application/json';
    requestOptions.body = JSON.stringify(body);
  }

  const response = await fetch(url, requestOptions);

  if (!response.ok) {
    const errorText = await response.text().catch(() => '');
    throw new TodoziError(`HTTP error ${response.status}: ${errorText}`);
  }

  const result = await response.json().catch(e => {
    throw new TodoziError(`JSON parse error: ${e.message}`);
  });

  return result;
}

async function processTdzCommands(text, baseUrl, apiKey) {
  const commands = parseTdzCommand(text);
  const results = [];

  for (const command of commands) {
    const result = await executeTdzCommand(command, baseUrl, apiKey);
    results.push({ command, result });
  }

  return results;
}

/******************************
 * Mock API server for demo
 ******************************/

function createMockApiServer(port = 0) {
  const server = http.createServer((req, res) => {
    // Enable CORS for local testing
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET,POST,PUT,DELETE,OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type,X-API-Key');

    if (req.method === 'OPTIONS') {
      res.writeHead(204);
      return res.end();
    }

    const send = (status, obj) => {
      res.writeHead(status, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify(obj, null, 2));
    };

    const notFound = () => send(404, { error: 'Not Found', path: req.url });

    try {
      const url = new URL(req.url, `http://${req.headers.host}`);
      const path = url.pathname;
      const method = req.method;

      // Health
      if (path === '/health' && method === 'GET') {
        return send(200, { status: 'ok', service: 'mock-todozi', time: new Date().toISOString() });
      }

      // Tasks list
      if (path === '/tasks' && method === 'GET') {
        return send(200, {
          tasks: [
            { id: 't-101', action: 'Write docs', status: 'in_progress', priority: 'medium', project: 'docs' },
            { id: 't-102', action: 'Review PR #42', status: 'todo', priority: 'high', project: 'core' }
          ],
          count: 2
        });
      }

      // Create task
      if (path === '/tasks' && method === 'POST') {
        let body = '';
        req.on('data', chunk => body += chunk);
        req.on('end', () => {
          const data = JSON.parse(body || '{}');
          return send(201, {
            id: 't-103',
            ...data,
            created: true
          });
        });
        return;
      }

      // Get task by id
      if (path.startsWith('/tasks/') && method === 'GET') {
        const id = path.split('/').pop();
        return send(200, { id, action: 'Mock task ' + id, status: 'todo', priority: 'low', project: 'general' });
      }

      // Update task
      if (path.startsWith('/tasks/') && method === 'PUT') {
        const id = path.split('/').pop();
        let body = '';
        req.on('data', chunk => body += chunk);
        req.on('end', () => {
          const data = JSON.parse(body || '{}');
          return send(200, {
            id,
            updated: true,
            patch: data
          });
        });
        return;
      }

      // Search tasks
      if (path === '/tasks/search' && method === 'GET') {
        const q = url.searchParams.get('q') || '';
        return send(200, {
          query: q,
          results: [
            { id: 't-201', action: `Search result for "${q}"`, status: 'todo', priority: 'medium', project: 'search' }
          ],
          count: 1
        });
      }

      // Chat run
      if (path === '/chat/process' && method === 'POST') {
        let body = '';
        req.on('data', chunk => body += chunk);
        req.on('end', () => {
          const data = JSON.parse(body || '{}');
          return send(200, {
            response: `Echo: ${data.message || ''}`,
            context: data.context || null,
            model: 'mock-gpt',
            time: new Date().toISOString()
          });
        });
        return;
      }

      // Ideas
      if (path === '/ideas' && method === 'POST') {
        let body = '';
        req.on('data', chunk => body += chunk);
        req.on('end', () => {
          const data = JSON.parse(body || '{}');
          return send(201, { id: 'i-301', ...data, created: true });
        });
        return;
      }

      return notFound();
    } catch (err) {
      return send(500, { error: 'Server error', message: String(err) });
    }
  });

  return new Promise((resolve, reject) => {
    server.listen(port, () => {
      const addr = server.address();
      resolve({ server, port: addr.port });
    });
  });
}

/******************************
 * Demo runner
 ******************************/
async function main() {
  // Start mock API on a free port
  const { server, port } = await createMockApiServer(0);
  const baseUrl = `http://127.0.0.1:${port}`;
  const apiKey = 'demo-api-key-123';

  // Blocks of <tdz> commands to parse and execute
  const blocks = [
    // 1) List all tasks
    `<tdz>list; tasks</tdz>`,

    // 2) Get a specific task
    `<tdz>get; tasks; t-101</tdz>`,

    // 3) Create a new task with options
    `<tdz>create; task; ; action=Write Example 5; time=1h; priority=high; project=examples; status=todo; tags=example,tdz; assignee=human</tdz>`,

    // 4) Update the created task
    `<tdz>update; task; t-103; action=Write Example 5 — finalized; status=in_progress</tdz>`,

    // 5) Search tasks
    `<tdz>search; tasks; docs</tdz>`,

    // 6) Run a chat
    `<tdz>run; chat; message="Hello from Example 5"; context=cli</tdz>`,

    // 7) Create an idea
    `<tdz>create; idea; ; idea="Improve TDZ parser error messages"; share=private; importance=medium</tdz>`
  ];

  console.log('Running Example 5 with mock API at', baseUrl);
  console.log('-----------------------------------------------------------\n');

  for (const text of blocks) {
    console.log('Input:', text);
    try {
      const results = await processTdzCommands(text, baseUrl, apiKey);
      for (const { command, result } of results) {
        console.log('Executed:', command.command, '→', command.target);
        console.log('Response:', result);
      }
    } catch (err) {
      console.error('Failed:', err.name || 'Error', '-', err.message || String(err));
    }
    console.log('-----------------------------------------------------------\n');
  }

  server.close();
}

main().catch(err => {
  console.error('Fatal:', err);
  process.exit(1);
});

Notes:
- The mock server is intentionally simplistic and only implements the endpoints used in the example.
- You can extend getEndpointPath, buildRequestBody, and buildRunBody to cover more commands and targets.
- In a real integration, you would point baseUrl to your live Todozi server and remove the mock.