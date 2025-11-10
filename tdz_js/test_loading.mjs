import { TodoziApp, DisplayConfig } from './todozi/tui.js';

class MockEmb {
    async findSimilarTasks() { return []; }
    async semanticSearch() { return []; }
}

console.log('🧪 Testing TUI Data Loading...\n');

const app = new TodoziApp(new MockEmb(), new DisplayConfig());
await app.loadTasks();

console.log(`📁 Projects: ${app.projects.length}`);
console.log(`📋 Tasks: ${app.tasks.length}`);

if (app.tasks.length > 0) {
    console.log(`\n✅ Sample task:`);
    console.log(`   Action: ${app.tasks[0]?.action}`);
    console.log(`   Status: ${app.tasks[0]?.status}`);
    console.log(`   Priority: ${app.tasks[0]?.priority}`);
    console.log(`   Project: ${app.tasks[0]?.parentProject}`);
} else {
    console.log('\n⚠️  No tasks loaded!');
}

console.log('\n📊 Summary:');
console.log(`   Expected - Projects: 7, Tasks: 27`);
console.log(`   Actual   - Projects: ${app.projects.length}, Tasks: ${app.tasks.length}`);

if (app.tasks.length > 0 && app.projects.length > 0) {
    console.log('\n✨ Phase 1: PASSED - Data loading works!');
} else {
    console.log('\n❌ Phase 1: FAILED - Data loading needs investigation');
}
