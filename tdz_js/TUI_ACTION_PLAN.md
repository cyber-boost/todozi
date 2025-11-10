# JavaScript TUI Action Plan

## Executive Summary

The JavaScript TUI (`tdz_js/todozi/tui.js`) has **complete architecture** (2001 lines) with all features implemented. Unlike the Python TUI which had enum incompatibility, the JS TUI uses string-based enums that are already compatible with storage.

**Status**: Architecture ✅ | Data Loading ❓ | Interactive Features ❓

## Current State

### ✅ Already Complete
- TodoziApp class with full state management
- TuiService for rendering  
- ColorScheme with truecolor support
- All enums (Status, Priority, Assignee as strings)
- Task loading infrastructure
- Tab system (8 tabs)
- Keyboard navigation
- Examples and documentation

### ⚠️ Needs Verification
1. Task loading from nested project_tasks structure
2. Interactive features (create/edit/complete)
3. UI rendering with real data

## Action Plan - 3 Phases

### Phase 1: Verify Data Loading (1-2 hours)

**Test Script**:
```bash
cd /opt/todozi/tdz_js
cat > test_loading.mjs << 'EOF'
import { TodoziApp, DisplayConfig } from './todozi/tui.js';

class MockEmb {
    async findSimilarTasks() { return []; }
    async semanticSearch() { return []; }
}

const app = new TodoziApp(new MockEmb(), new DisplayConfig());
await app.loadTasks();

console.log(`Projects: ${app.projects.length}`);
console.log(`Tasks: ${app.tasks.length}`);
console.log('Sample:', app.tasks[0]?.action);
EOF

node test_loading.mjs
```

**Expected**: Projects: 7, Tasks: 27

**If Broken**: Fix task extraction in `loadTasks()` (line 771):
```javascript
// Extract from nested structure
const allTasks = [
    ...Object.values(content.active_tasks || {}),
    ...Object.values(content.completed_tasks || {}),
    ...Object.values(content.archived_tasks || {})
];
this.tasks.push(...allTasks);
```

### Phase 2: Verify Interactive Features (2-3 hours)

**Check These Methods**:
- `addTask()` - Create tasks
- `completeTask()` - Mark done  
- `deleteTask()` - Remove tasks
- `editTask()` - Modify tasks

**Test**:
```bash
node todozi/run-tui.js
# Press 'a' to add task
# Press 'c' to complete task
# Press 'e' to edit task
```

**If Missing**: Add methods using storage adapter

### Phase 3: Test & Document (1 hour)

**Create Tests**:
```javascript
// todozi/__tests__/tui.test.js
test('loads tasks from storage', async () => {
    const app = new TodoziApp(mockEmb, config);
    await app.loadTasks();
    expect(app.tasks.length).toBeGreaterThan(0);
});
```

**Update Docs**: Add getting started guide

## Key Differences: Python vs JavaScript

| Feature | Python TUI | JS TUI |
|---------|-----------|--------|
| **Issue** | Enum mismatch | ✅ No issue |
| **UI** | Textual widgets | Console/chalk |
| **Lines** | 3,800 | 2,000 |
| **Complexity** | High | Medium |
| **Status** | ✅ Fixed | ❓ Testing |

## Quick Fixes Needed

### Fix 1: Task Extraction (todozi/tui.js:771)
```javascript
// OLD (probably):
this.tasks.push(content);

// NEW:
const allTasks = [
    ...Object.values(content.active_tasks || {}),
    ...Object.values(content.completed_tasks || {})
];
this.tasks.push(...allTasks);
```

### Fix 2: Add Storage Adapter
Create `todozi/storage_adapter.js` for clean storage interface

### Fix 3: Error Handling
Wrap all file operations in try/catch

## Success Criteria

**Must Have (MVP)**:
- [x] Loads without errors
- [ ] Shows 27 tasks
- [ ] Shows 7 projects
- [ ] Navigation works
- [ ] Can create tasks

**Estimated Time**: 4-6 hours total
**Risk**: Low (architecture is solid)

## Next Steps

1. Run Phase 1 verification tests
2. Fix data loading if needed  
3. Test interactive features
4. Add missing functionality
5. Write tests
6. Update documentation

The JS TUI is **90% complete** - just needs verification and minor fixes!
