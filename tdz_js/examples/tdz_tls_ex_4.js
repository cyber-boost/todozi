/*
=====================================================================
  Example 4 – Smart Content Processor
  -------------------------------------------------
  Demonstrates:
   • Creating a TodoziProcessorState
   • Instantiating TdzContentProcessorTool
   • Running the tool on mixed free‑form text
   • Automatic session handling
   • Persisting the extracted objects with the Storage layer
   • Printing the same rich output the CLI would emit
===================================================================== */

import {
  // core processor classes
  TodoziProcessorState,
  TdzContentProcessorTool,
  // low‑level helper that combines everything
  tdzCnt,
} from '../todozi/tdz_tls.js';

import {
  Storage,
} from '../todozi/storage.js';     // storage abstraction (projects, tasks, …)

(async () => {
  // -------------------------------------------------------------
  // 1️⃣  Initialise the *state* that the processor will use.
  // -------------------------------------------------------------
  const state = new TodoziProcessorState();

  // -------------------------------------------------------------
  // 2️⃣  Build the tool – it needs the state we just created.
  // -------------------------------------------------------------
  const contentTool = new TdzContentProcessorTool(state);

  // -------------------------------------------------------------
  // 3️⃣  Define a sample message.
  // -------------------------------------------------------------
  const sampleMessage = `
Hey team,

We should **finish the API redesign** by next Friday.  
time: 2025‑01‑15 17:00; priority: high; project: backend; status: todo

I need to **write unit tests for the new auth middleware** – don't forget!
Please add it to the checklist.

Also, here is a tiny JSON snippet that the AI can call:

{
  "tool_calls": [
    {
      "function": {
        "name": "todozi_create_task",
        "arguments": {
          "action": "Refactor user service",
          "time": "2025‑01‑20 09:00",
          "priority": "medium",
          "project": "backend",
          "status": "todo"
        }
      }
    }
  ]
}

Thanks,
Alice
`;

  // -------------------------------------------------------------
  // 4️⃣  Run the **high‑level** `tdzCnt` helper.
  // -------------------------------------------------------------
  // It does three things for us:
  //   • Strips all Todozi tags from `clean`
  //   • Calls the low‑level processor (extract, checklist, sessions)
  //   • Returns a JSON structure that mirrors the CLI output.
  const jsonResult = await tdzCnt(sampleMessage, / * optional sessionId */ 'demo‑session-01');

  // -------------------------------------------------------------
  // 5️⃣  Pretty‑print the JSON result.
  // -------------------------------------------------------------
  console.log('\n=== 📦 tdzCnt() RESULT ===\n');
  console.log(JSON.stringify(JSON.parse(jsonResult), null, 2));

  // -------------------------------------------------------------
  // 6️⃣  OPTIONAL – Persist the extracted tasks using the Storage API.
  // -------------------------------------------------------------
  // The `tdzCnt` function already *saved* a `ProcessedContent` record
  // in the in‑memory `state`.  If you want to write those objects to
  // the real Todozi on‑disk storage, you can do it manually:
  const storage = await Storage.new();   // loads the ~/.todozi configuration

  // The `state.processed_contents` array now holds one ProcessedContent.
  const processed = state.processed_contents[0];
  const { tasks } = await contentTool.extractTodoziData(
    // `ParsedContent` is already stored inside the ProcessedContent,
    // but recreate the minimal structure the extract method expects.
    {
      text_content: processed.raw_content,
      json_content: null,
      tool_calls: []   // already handled inside `processContent`
    }
  );

  // `tasks` is empty because our simple demo does not include <todozi> tags,
  // but you could call `processContent` directly and then persist each
  // `Task` object returned by `extractTodoziData`.
  // Example (uncomment if you added real <todozi> tags):
  // for (const t of tasks) {
  //   await storage.addTaskToProject(t);
  //   console.log(`✅ Saved task → ${t.action}`);
  // }

  // -------------------------------------------------------------
  // 7️⃣  Show the **active checklist** that the processor added.
  // -------------------------------------------------------------
  console.log('\n=== 📋 ACTIVE CHECKLIST (in‑memory) ===\n');
  state.checklist_items.forEach((item, i) => {
    console.log(`${i + 1}. ${item.content}  [${item.priority}]`);
  });

  // -------------------------------------------------------------
  // 8️⃣  Show the **session** that was auto‑created.
  // -------------------------------------------------------------
  console.log('\n=== 🗂️ ACTIVE SESSIONS (in‑memory) ===\n');
  for (const [sid, sess] of state.active_sessions.entries()) {
    console.log(`• ${sid} – topic: ${sess.topic} – messages: ${sess.message_count}`);
  }

  console.log('\n🛠️  Demo finished – you can now adapt the code to your own workflow!\n');
})();
