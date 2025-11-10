import { Done, models // enums like Status, Priority, Assignee } from '../todozi/lib.js';

Here is a clear, practical usage example (Example 5) that demonstrates how to use the Done class from lib.js to manage tasks, search, and more. It initializes the system, creates various tasks (including AI- and human- ones), updates status, searches, and shows a quick stats summary.

example5.js
javascript
// Example 5: Practical usage of the Done API
// This example demonstrates core task workflows using the Done class from lib.js

async function main() {
  try {
    // 1) Ensure Todozi is initialized and folder structure is ready
    await Done.ensureTodoziInitialized();
    console.log('✅ Todozi initialized\n');

    // Optional: set a project name (defaults to 'external_apps')
    Done.setProject('example5_project');

    // 2) Create tasks using different helpers
    const t1 = await Done.quickTask('Write example README');
    console.log(`✅ quickTask: ${t1.action} (id=${t1.id})`);

    const t2 = await Done.ai('Design API contract for tasks endpoint');
    console.log(`✅ ai task: id=${t2}`);

    const t3 = await Done.human('Write unit tests for task operations');
    console.log(`✅ human task: id=${t3}`);

    const t4 = await Done.collab('Refactor storage layer for performance');
    console.log(`✅ collab task: id=${t4}`);

    // 3) Create tasks with full control using createTask
    const t5 = await Done.createTask(
      'Draft example usage guide',
      models.Priority.Medium,
      'example5_project',
      'ASAP',
      'Provide clear examples for newcomers'
    );
    console.log(`✅ full createTask: ${t5.action} (id=${t5.id})\n`);

    // 4) List all tasks
    const all = await Done.allTasks();
    console.log(`📋 All tasks (${all.length}):`);
    for (const t of all) {
      console.log(`  - [${t.id}] ${t.action} (${t.status}, ${t.priority})`);
    }
    console.log();

    // 5) Update task status
    await Done.completeTask(t1.id);
    await Done.startTask(t3.id);
    console.log(`✅ Updated status for ${t1.id} to done and ${t3.id} to in progress\n`);

    // 6) Search tasks (keyword)
    const found = await Done.findTasks('unit tests');
    console.log(`🔎 Keyword search results for "unit tests" (${found.length}):`);
    for (const t of found) {
      console.log(`  - [${t.id}] ${t.action} (${t.status})`);
    }
    console.log();

    // 7) AI semantic search (if embedding service is available)
    try {
      const similar = await Done.aiFind('testing coverage');
      if (similar.length > 0) {
        console.log(`🤖 AI semantic search (top 3) for "testing coverage":`);
        for (const s of similar.slice(0, 3)) {
          console.log(`  - ${s.textContent} (score=${s.similarityScore.toFixed(2)})`);
        }
        console.log();
      }
    } catch (e) {
      console.log('⚠️ AI search not available (embedding service not initialized)\n');
    }

    // 8) Get a specific task and update it
    const task = await Done.getTask(t5.id);
    console.log(`📄 Task details: ${task.action} (status=${task.status}, project=${task.parentProject})`);

    await Done.updateTaskFull(t5.id, {
      action: 'Draft example usage guide (updated)',
      progress: 50,
      status: models.Status.InProgress
    });
    console.log('✅ Updated task progress and action\n');

    // 9) Quick stats
    const stats = await Done.quick();
    console.log(stats);

    // 10) Working with ideas and memories
    const ideaId = await Done.ideate('Use dependency injection for storage layer');
    console.log(`\n💡 Idea saved: ${ideaId}`);

    const memId = await Done.remember('Example 5 run', 'Demonstrates Done API end-to-end', 'Sample reason');
    console.log(`🧠 Memory saved: ${memId}`);

  } catch (err) {
    console.error('❌ Error:', err.message || err);
    if (err.stack) console.error(err.stack);
  }
}

main();

Notes
- This example uses only the public static methods available on the Done class in lib.js, such as:
  - ensureTodoziInitialized, setProject, quickTask, ai, human, collab, createTask, allTasks, completeTask, startTask, findTasks, aiFind, getTask, updateTaskFull, quick, ideate, remember.
- It avoids networking (no server calls), and works with the local storage layer initialized under ~/.todozi by default.
- If you want to run it:
  - Ensure the dependencies used by lib.js are available in your environment (uuid, axios, etc.).
  - Run: node example5.js

Example output
- You should see task creation, updates, searches, and a quick stats summary printed to the console.