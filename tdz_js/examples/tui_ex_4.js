import { v4 as uuidv4 } from 'uuid';
import chalk from 'chalk';
import path from 'path';
import { fileURLToPath } from 'url';

// -------------------------------------------------------------------
// 1️⃣  Helpers to resolve the repo’s src folder (where all modules live
// -------------------------------------------------------------------
const __filename = fileURLToPath(import.meta.url);
const __dirname  = path.dirname(__filename);
const SRC = path.join(__dirname, 'src');

// -------------------------------------------------------------------
// 2️⃣  Import the pieces we need
// -------------------------------------------------------------------
import { Storage } from '../src/storage.js';
import { TagManager } from '../src/tags.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../src/emb.js';
import { TuiService, DisplayConfig, TaskDisplay, TaskListDisplay } from '../src/tui.js';
import { Task } from '../src/models.js';

// -------------------------------------------------------------------
// 3️⃣  Small utility – pretty‑print a list of tasks
// -------------------------------------------------------------------
function prettyPrintTask(task) {
  const pri = chalk.bold(task.priority);
  const stat = chalk[task.status === 'Done' ? 'green' : 'yellow'](task.status);
  const tags = task.tags.length ? `#${task.tags.join(' #')}` : '';
  return `${chalk.cyan(task.id)} ${task.action} ${tags}\n   ${stat} • ${pri} • ${task.time}`;
}

// -------------------------------------------------------------------
// 4️⃣  Main async IIFE – everything runs inside this block
// -------------------------------------------------------------------
(async () => {
  console.log(chalk.blue('\n=== 🚀 Smart‑Todo Demo – Example 4 ===\n'));

  // ---------------------------------------------------------------
  // 1️⃣ Initialise storage (creates ~/.todozi if it does not exist)
  // ---------------------------------------------------------------
  console.log(chalk.gray('🔧 Initialising storage…'));
  const storage = await Storage.new();           // loads/creates config & dirs
  // (We do not persist tasks here – the demo works in‑memory)

  // ---------------------------------------------------------------
  // 2️⃣  Create a TagManager and a few tags
  // ---------------------------------------------------------------
  console.log(chalk.gray('🏷️  Setting up TagManager…'));
  const tagMgr = TagManager.new();

  const tagIds = await Promise.all([
    tagMgr.createTag({ name: 'frontend', description: 'UI / client side', category: 'area' }),
    tagMgr.createTag({ name: 'backend',  description: 'Server / API',       category: 'area' }),
    tagMgr.createTag({ name: 'bug',      description: 'Defect',           category: 'type' })
  ]);
  console.log(chalk.green('✅  Created tags:'), tagIds.map(id => tagMgr.getTag(id).name).join(', '));

  // ---------------------------------------------------------------
  // 3️⃣  Create a few sample tasks (using the Task model)
  // ---------------------------------------------------------------
  console.log(chalk.gray('\n🗒️  Creating sample tasks…'));

  const sampleTasks = [
    new Task({
      userId: 'demo_user',
      action: 'Implement login page',
      time: '2h',
      priority: 'High',
      parentProject: 'demo',
      status: 'Todo',
      tags: ['frontend'],
    }),
    new Task({
      userId: 'demo_user',
      action: 'Fix API rate‑limit bug',
      time: '1h',
      priority: 'Critical',
      parentProject: 'demo',
      status: 'Todo',
      tags: ['backend', 'bug'],
    }),
    new Task({
      userId: 'demo_user',
      action: 'Write unit tests for auth service',
      time: '3h',
      priority: 'Medium',
      parentProject: 'demo',
      status: 'Todo',
      tags: ['backend'],
    })
  ];

  for (const t of sampleTasks) {
    console.log(prettyPrintTask(t));
  }

  // ---------------------------------------------------------------
  // 4️⃣  Initialise the embedding service (loads the default model)
  // ---------------------------------------------------------------
  console.log(chalk.gray('\n🔎 Loading embedding model…'));
  const embConfig = TodoziEmbeddingConfig.default();
  const embService = await TodoziEmbeddingService.new(embConfig);
  await embService.initialize();                     // loads sentence‑transformer

  // ---------------------------------------------------------------
  // 5️⃣  Embed every task and store the vector in the task object
  // ---------------------------------------------------------------
  console.log(chalk.gray('📐 Generating embeddings for tasks…'));
  for (const task of sampleTasks) {
    const content = `
      Action: ${task.action}
      Tags: ${task.tags.join(', ')}
      Project: ${task.parentProject}
      Priority: ${task.priority}
    `.trim();

    const [vec] = await embService.generateEmbeddingsBatch([content]); // returns [embedding]
    task.embeddingVector = vec;
  }

  // ---------------------------------------------------------------
  // 6️⃣  Persist the tasks – here we use the in‑memory ProjectTaskContainer
  // ---------------------------------------------------------------
  console.log(chalk.gray('💾 Storing tasks in a temporary container…'));
  // The storage layer in the repo expects a “project task container”.  
  // For the demo we cheat a little and just keep them in an array.
  // (In a real CLI you would call storage.addTaskToProject(task))
  const allTaskIds = sampleTasks.map(t => t.id);

  // ---------------------------------------------------------------
  // 7️⃣  Find tasks similar to a new query
  // ---------------------------------------------------------------
  const query = 'User cannot log in – authentication error';
  console.log(chalk.gray(`\n🔎 Searching for tasks similar to: "${query}"`));

  const similar = await embService.findSimilarTasks(query, 5);
  if (similar.length === 0) {
    console.log(chalk.yellow('⚠️  No similar tasks found (embedding model is a stub).'));
  } else {
    console.log(chalk.green(`✅ Found ${similar.length} similar task(s):`));
    similar.forEach((sim, i) => {
      const task = sampleTasks.find(t => t.id === sim.content_id);
      console.log(`${i + 1}. ${chalk.cyan(task.id)} – ${task.action}`);
      console.log(`   similarity: ${(sim.similarity_score * 100).toFixed(1)}%`);
    });
  }

  // ---------------------------------------------------------------
  // 8️⃣  Render a *TaskListDisplay* with the TUI service
  // ---------------------------------------------------------------
  console.log(chalk.gray('\n📊 Rendering TUI view (TaskListDisplay)…'));

  // Build a fake “TaskDisplay” array – normally you would get these
  // from `await tuiService.displayTask(taskId)`.
  const taskDisplays = sampleTasks.map(t => new TaskDisplay({
    task: t,
    similarTasks: [],           // we could fill with similar‑task data here
    aiSuggestions: [],         // or AI‑generated ideas
    semanticTags: [],          // …etc.
    confidenceScore: 0,
    relatedContent: []
  }));

  const listDisplay = new TaskListDisplay({
    tasks: taskDisplays,
    totalCount: taskDisplays.length,
    aiSummary: 'All demo tasks are pending; focus on the critical backend bug first.',
    semanticClusters: []       // clustering is optional
  });

  // Default display config (indexed colours)
  const dispCfg = DisplayConfig.default ? DisplayConfig.default() : new DisplayConfig();

  // The render method simply returns a string – we print it.
  console.log('\n' + listDisplay.render(dispCfg));

  // ---------------------------------------------------------------
  // End of demo
  // ---------------------------------------------------------------
  console.log(chalk.blue('\n=== 🎉 Demo complete ===\n'));
})();
