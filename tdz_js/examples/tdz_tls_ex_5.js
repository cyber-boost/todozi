import { tdzCnt, initializeTdzContentProcessor, createTdzContentProcessorTool, TodoziProcessorState } from '../todozi/tdz_tls.js';

import fs from 'fs';
import path from 'path';
import readline from 'readline';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

Below is a self-contained Node.js example that demonstrates how to use the Todozi content processing functions from tdz_tls.js. It shows both a simple tdzCnt call and direct usage of TdzContentProcessorTool with programmatic state.

Save as example5.js and run with: node example5.js "Your text with <todozi>...<memory>...<idea>..." or node example5.js @path/to/input.txt

You can also run interactively if no text is provided: node example5.js

javascript
#!/usr/bin/env node

// example5.js
// Demonstrates practical usage of tdz_tls.js content processing: tdzCnt and TdzContentProcessorTool

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

function parseArgs(argv) {
  const args = { text: null, sessionId: 'example-session', extractChecklist: true, autoSession: true, help: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '-h' || a === '--help') args.help = true;
    else if ((a === '-s' || a === '--session') && argv[i + 1]) args.sessionId = argv[++i];
    else if (a === '--no-checklist') args.extractChecklist = false;
    else if (a === '--no-session') args.autoSession = false;
    else if (!a.startsWith('-')) args.text = a;
  }
  return args;
}

function showUsage() {
  console.log('Usage: node example5.js [options] <text|@file>');
  console.log('');
  console.log('Options:');
  console.log('  -s, --session <id>     Session ID (default: example-session)');
  console.log('  --no-checklist         Disable automatic checklist extraction');
  console.log('  --no-session           Disable automatic session management');
  console.log('  -h, --help             Show this help message');
  console.log('');
  console.log('Examples:');
  console.log('  node example5.js "Let\'s implement login <todozi>action=Login;time=2h;priority=high;project=Auth;status=todo</todozi>"');
  console.log('  node example5.js -s my-session-123 --no-checklist @./sample.txt');
  console.log('  node example5.js  (interactive mode)');
}

async function readStdin() {
  return new Promise((resolve) => {
    let buf = '';
    rl.on('line', (line) => {
      if (line.trim() === '') {
        rl.close();
      } else {
        buf += (buf ? '\n' : '') + line;
      }
    });
    rl.on('close', () => resolve(buf));
  });
}

function readFileOrText(input) {
  if (!input) return '';
  if (input.startsWith('@')) {
    const filePath = path.resolve(process.cwd(), input.slice(1));
    return fs.readFileSync(filePath, 'utf8');
  }
  return input;
}

// Example 5 — Practical usage of tdzCnt and TdzContentProcessorTool
(async function main() {
  const args = parseArgs(process.argv.slice(2));

  if (args.help) {
    showUsage();
    process.exit(0);
  }

  let content = args.text;
  if (!content) {
    // Interactive mode
    console.log('Enter text to process (finish with an empty line):');
    content = await readStdin();
  }

  const cleanInput = readFileOrText(content).trim();
  if (!cleanInput) {
    console.error('No content provided. Use -h for usage.');
    process.exit(1);
  }

  // Sample input showcasing tags recognized by the processor
  const sample = `
    We need to set up CI/CD. Don't forget to document the deployment steps.
    Also, let's create a memory about the incident.
    <memory>standard;Outage at 02:15 UTC;Root cause was a misconfigured timeout;Post-mortem;high;short;postmortem,incident</memory>
    <idea>Use feature flags for risky changes;private;medium;flags,rollout</idea>
    <todozi>action=Set up CI/CD;time=4h;priority=high;project=DevOps;status=todo;ai;ci,cd,devops</todozi>
  `;

  // Use either the provided content or the sample for demonstration
  const textToProcess = cleanInput || sample;

  console.log('=== Using tdzCnt (high-level API) ===');
  const tdzResultJson = await tdzCnt(textToProcess, args.sessionId);
  console.log(tdzResultJson);

  console.log('\n=== Using TdzContentProcessorTool (programmatic state) ===');
  const state = await initializeTdzContentProcessor();
  const tool = createTdzContentProcessorTool(state);

  const result = await tool.execute({
    content: textToProcess,
    session_id: args.sessionId,
    extract_checklist: args.extractChecklist,
    auto_session: args.autoSession
  });

  if (!result.success) {
    console.error('Processing failed:', result.output);
    process.exit(1);
  }

  console.log(result.output);

  // Example: Inspect the processor's state after processing
  console.log('\n=== Processor State Snapshot ===');
  console.log(`Active sessions: ${state.active_sessions.size}`);
  console.log(`Recent actions (last 3): ${state.recent_actions.slice(-3).map(a => a.action_type).join(', ') || 'none'}`);
  console.log(`Checklist items: ${state.checklist_items.length}`);
  console.log(`Processed contents: ${state.processed_contents.length}`);

  // Example: Directly call extractChecklistItems for a custom text
  console.log('\n=== Direct Checklist Extraction (no session) ===');
  const checklistItems = tool.extractChecklistItems(textToProcess);
  if (checklistItems.length > 0) {
    for (const item of checklistItems) {
      console.log(`- [${item.priority}] ${item.content} (${item.source})`);
    }
  } else {
    console.log('No checklist items found.');
  }

  process.exit(0);
})().catch((err) => {
  console.error('Unhandled error:', err);
  process.exit(1);
});