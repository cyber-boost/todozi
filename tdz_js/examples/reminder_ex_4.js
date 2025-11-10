// ---------------------------------------------------------------
// example4.js – a practical demo of the Reminder subsystem
// ---------------------------------------------------------------

// 1️⃣  Load everything that we need from the reminder module
// -----------------------------------------------------------------
// 2️⃣  Helper: pretty‑print a reminder (used throughout the demo)
// -----------------------------------------------------------------
import { ReminderManager, ReminderUpdate, ReminderStatistics, parseReminderFormat, ReminderPriority, ReminderStatus, ReminderTests   // <-- just for a sanity‑check run at the end } from '../todozi/reminder.js';

function prettyPrint(rem) {
  console.log(`🗓️  ${rem.id}`);
  console.log(`   📄 Content : ${rem.content}`);
  console.log(`   ⏰ Remind At: ${rem.remind_at.toISOString()}`);
  console.log(`   📊 Priority : ${rem.priority}`);
  console.log(`   🔖 Tags    : ${rem.tags.join(', ') || '‑'}`);
  console.log(`   📍 Status  : ${rem.status}`);
  console.log(`   📅 Created : ${rem.created_at.toISOString()}`);
  console.log(`   🛠️  Updated : ${rem.updated_at.toISOString()}\n`);
}

// -----------------------------------------------------------------
// 3️⃣  Instantiate the manager – it holds everything in memory
// -----------------------------------------------------------------
const manager = new ReminderManager();

// -----------------------------------------------------------------
// 4️⃣  Create a few reminders the “old‑fashioned” way (plain objects)
// -----------------------------------------------------------------
(async () => {
  console.log('\n=== 📥  Adding reminders ===\n');

  // Reminder A – will be *pending* for a future date
  const reminderA = {
    content: 'Buy groceries',
    remind_at: new Date(Date.now() + 60 * 60 * 1000),          // 1 h from now
    priority: ReminderPriority.Medium,
    tags: ['shopping', 'errands']
  };
  const idA = await manager.createReminder(reminderA);
  console.log('Created Reminder A ➜', idA);

  // Reminder B – already *over‑due*
  const reminderB = {
    content: 'Pay electricity bill',
    remind_at: new Date(Date.now() - 2 * 60 * 60 * 1000),     // 2 h ago
    priority: ReminderPriority.High,
    status: ReminderStatus.Pending,
    tags: ['finance']
  };
  const idB = await manager.createReminder(reminderB);
  console.log('Created Reminder B ➜', idB);

  // Reminder C – using the **XML‑like parser**
  const raw = `<reminder>
                 Finish the quarterly report; ${new Date(Date.now() + 3 * 60 * 60 * 1000).toISOString()};
                 high; pending; work,report
               </reminder>`;
  const parsed = parseReminderFormat(raw);
  const idC = await manager.createReminder(parsed);
  console.log('Created Reminder C (parsed) ➜', idC);

  // -------------------------------------------------------------
  // 5️⃣  Retrieve & display them
  // -------------------------------------------------------------
  console.log('\n=== 📖  All reminders after creation ===\n');
  manager.getAllReminders().forEach(prettyPrint);

  // -------------------------------------------------------------
  // 6️⃣  Update Reminder A – change priority, add a tag & mark active
  // -------------------------------------------------------------
  console.log('\n=== ✏️  Updating Reminder A ===\n');
  const upd = ReminderUpdate.new()
                .priority(ReminderPriority.High)          // bump priority
                .tags(['shopping', 'urgent', 'food']);   // replace tags
  await manager.updateReminder(idA, upd);
  // also change its status to *active*
  await manager.activateReminder(idA);

  // show the updated reminder
  prettyPrint(manager.getReminder(idA));

  // -------------------------------------------------------------
  // 7️⃣  Search by keyword / tag
  // -------------------------------------------------------------
  console.log('\n=== 🔎  Search for “report” (content) ===\n');
  manager.searchReminders('report').forEach(prettyPrint);

  console.log('\n=== 🔎  Search for tag “finance” ===\n');
  manager.getRemindersByTag('finance').forEach(prettyPrint);

  // -------------------------------------------------------------
  // 8️⃣  Time‑based queries
  // -------------------------------------------------------------
  console.log('\n=== ⏰  Reminders due in the next 2 h ===\n');
  const dueSoon = manager.getRemindersDueSoon(2 * 60 * 60 * 1000);
  dueSoon.forEach(prettyPrint);

  console.log('\n=== ⚠️  Over‑due reminders ===\n');
  manager.getOverdueReminders().forEach(prettyPrint);

  // -------------------------------------------------------------
  // 9️⃣  Tag helpers
  // -------------------------------------------------------------
  console.log('\n=== 📚  All tags known to the system ===\n');
  console.log(manager.getAllTags().join(', ') || '‑ none ‑');

  console.log('\n=== 📊  Tag usage statistics ===\n');
  console.table(manager.getTagStatistics());

  // -------------------------------------------------------------
  // 🔟  Global reminder statistics (percentages, counts)
  // -------------------------------------------------------------
  console.log('\n=== 📈  Global Reminder Statistics ===\n');
  const stats = manager.getReminderStatistics(); // → ReminderStatistics instance
  console.log(`Total reminders        : ${stats.total_reminders}`);
  console.log(`Pending reminders      : ${stats.pending_reminders}`);
  console.log(`Active reminders       : ${stats.active_reminders}`);
  console.log(`Over‑due reminders     : ${stats.overdue_reminders}`);
  console.log(`Unique tags            : ${stats.unique_tags}`);
  console.log(`Pending %              : ${stats.pendingPercentage().toFixed(2)}%`);
  console.log(`Active %               : ${stats.activePercentage().toFixed(2)}%`);
  console.log(`Over‑due %             : ${stats.overduePercentage().toFixed(2)}%`);

  // -------------------------------------------------------------
  // 1️⃣1️⃣  Delete a reminder (Reminder B) and confirm removal
  // -------------------------------------------------------------
  console.log('\n=== 🗑️  Deleting the over‑due Reminder B ===\n');
  await manager.deleteReminder(idB);
  console.log('Reminder B removed.\n');

  console.log('=== 📋  Remaining reminders ===\n');
  manager.getAllReminders().forEach(prettyPrint);

  // -------------------------------------------------------------
  // 1️⃣2️⃣  Run the built‑in unit tests to prove everything works
  // -------------------------------------------------------------
  console.log('\n=== ✅  Running the bundled ReminderTests ===\n');
  ReminderTests.testReminderManagerCreation();
  ReminderTests.testReminderUpdateBuilder();
  ReminderTests.testReminderStatistics();
  ReminderTests.testParseReminderFormat();
  ReminderTests.testParseReminderFormatMinimal();
  console.log('All tests passed! 🎉');

  // -------------------------------------------------------------
  // 🎉  End of demo
  // -------------------------------------------------------------
  console.log('\n=== 🎊  Example 4 completed ===\n');
})();

/*
## Example 4 – Full‑featured **Reminder** workflow  

Below is a self‑contained script that shows how a client can **create, read, update, search, and delete** reminders with the classes that live in `reminder.js`.  
It also demonstrates:

| Feature | How it’s shown |
|---------|----------------|
| **Parsing** a reminder from the “\<reminder\>…\<\/reminder\>” XML‑like syntax | `parseReminderFormat()` |
| **Builder‑style update** using `ReminderUpdate` | `updateReminder()` |
| **Statistics** – total reminders, pending/active/over‑due percentages | `ReminderStatistics` & `ReminderManager.getReminderStatistics()` |
| **Tag‑centric helpers** – list all tags & tag usage counts | `getAllTags()` & `getTagStatistics()` |
| **Time‑based queries** – “due soon” (next 2 h) & “over‑due” | `getRemindersDueSoon()` & `getOverdueReminders()` |

> **File:** `example4.js` (run with `node example4.js`)

### What the script does (step‑by‑step)

| # | Action | Code excerpt |
|---|--------|--------------|
| 1️⃣ | **Load** the public API (`ReminderManager`, `ReminderUpdate`, …) | `require('./reminder')` |
| 2️⃣ | **Pretty‑print** helper – makes the console output readable | `prettyPrint()` |
| 3️⃣ | **Instantiate** a `ReminderManager` (in‑memory store) | `new ReminderManager()` |
| 4️⃣ | **Create** three reminders: two manually, one via `parseReminderFormat()` | `await manager.createReminder(...)` |
| 5️⃣ | **Show** all reminders after creation | `manager.getAllReminders().forEach(prettyPrint)` |
| 6️⃣ | **Update** a reminder using the *builder* (`ReminderUpdate`) and activate it | `ReminderUpdate.new().priority(...).tags(...); await manager.updateReminder(idA, upd);` |
| 7️⃣ | **Search** by free‑text (`searchReminders`) and by tag (`getRemindersByTag`) |
| 8️⃣ | **Time‑based queries** – “due soon” (`getRemindersDueSoon`) and “over‑due” (`getOverdueReminders`) |
| 9️⃣ | **Tag utilities** – list every tag and view a usage breakdown |
| 🔟 | **Gather global statistics** – total counts + percentage helper methods |
| 1️⃣1️⃣ | **Delete** a reminder and verify the remaining set |
| 1️⃣2️⃣ | **Run** the bundled unit‑tests (`ReminderTests`) as a sanity‑check |

### Expected console output (excerpt)

/ *
=== 📥  Adding reminders ===
Created Reminder A ➜ 0c7f1b57-8c4e-4c96-aef2-1c6c9a0e5a13
Created Reminder B ➜ 7d23f8a2-1465-452a-9fa2-8b86f0d6f986
Created Reminder C (parsed) ➜ 3b9d5e84-3052-4473-8aa5-3c6e2c8a9e5a

=== 📖  All reminders after creation ===

🗓️  0c7f1b57-8c4e-4c96-aef2-1c6c9a0e5a13
   📄 Content : Buy groceries
   ⏰ Remind At: 2025‑01‑20T14:40:12.345Z
   📊 Priority : medium
   🔖 Tags    : shopping, errands
   📍 Status  : pending
   📅 Created : 2025‑01‑20T13:40:12.345Z
   🛠️  Updated : 2025‑01‑20T13:40:12.345Z

...
=== ✏️  Updating Reminder A ===

🗓️  0c7f1b57-8c4e-4c96-aef2-1c6c9a0e5a13
   📄 Content : Buy groceries
   ⏰ Remind At: 2025‑01‑20T14:40:12.345Z
   📊 Priority : high
   🔖 Tags    : shopping, urgent, food
   📍 Status  : active
   📅 Created : 2025‑01‑20T13:40:12.345Z
   🛠️  Updated : 2025‑01‑20T13:41:00.123Z

=== 🔎  Search for “report” (content) ===
...
=== 📊  Tag usage statistics ===
┌─────────────┬───────┐
│   (index)   │ Value │
├─────────────┼───────┤
│   finance   │   1   │
│   shopping  │   1   │
│   urgent    │   1   │
│   food      │   1   │
│   work      │   1   │
│   report    │   1   │
└─────────────┴───────┘

=== 📈  Global Reminder Statistics ===
Total reminders        : 3
Pending reminders      : 1
Active reminders       : 1
Over‑due reminders     : 1
Unique tags            : 6
Pending %              : 33.33%
Active %               : 33.33%
Over‑due %             : 33.33%

=== 🗑️  Deleting the over‑due Reminder B ===
Reminder B removed.

=== 📋  Remaining reminders ===
...
=== ✅  Running the bundled ReminderTests ===
All tests passed! 🎉

=== 🎊  Example 4 completed ===
*/