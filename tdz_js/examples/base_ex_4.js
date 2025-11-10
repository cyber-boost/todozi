// dynamic-loader.js
import { // Core classes Tool, ToolRegistry, ToolResult,  // Helper factories for definition & parameters createToolParameter, createToolParameterWithDefault, createToolDefinition, createToolDefinitionWithLocks,  // Error handling utilities ErrorHandler, ErrorType, ToolError, } from '../todozi/base.js';

import fs from 'fs';
import path from 'path';
import { fileURLToPath, pathToFileURL } from 'url';
import { ToolRegistry } from './base.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function loadAllTools(dir) {
  const registry = new ToolRegistry();
  const files = fs.readdirSync(dir).filter(f => f.endsWith('.js'));

  for (const file of files) {
    const filePath = pathToFileURL(path.join(dir, file)).href;
    const module = await import(filePath);
    const ToolClass = module.default || module;
    // Assume each file exports a subclass of Tool as default
    const toolInstance = new ToolClass();
    registry.register(toolInstance);
    console.log(`✅ Loaded tool "${toolInstance.name()}" from ${file}`);
  }
  return registry;
}

// Example usage:
(async () => {
  const registry = await loadAllTools(path.join(__dirname, 'tools'));
  console.log('🧩 Total tools loaded:', registry.toolCount());
})();

/*
// example4.js
// -----------------------------------------------------
// 1️⃣  Import the building blocks from base.js
// -----------------------------------------------------
// -----------------------------------------------------
// 2️⃣  Define the *Echo* tool
// -----------------------------------------------------
// The tool simply echoes back a string supplied by the caller.
// It demonstrates:
//   • parameter declaration + defaults
//   • runtime validation
//   • proper use of ToolResult (success / error)
// -----------------------------------------------------
class EchoTool extends Tool {
  // -----------------------------------------------------------------
  // 2.1  Provide the tool *definition* – this is static meta‑data.
  // -----------------------------------------------------------------
  definition() {
    // Parameters:
    //   • message  – the string we want to echo (required)
    //   • times    – how many times to repeat it (optional, default = 1)
    const messageParam = createToolParameter('message', 'string', 'Message that will be echoed', true);
    const timesParam   = createToolParameterWithDefault(
      'times',         // name
      'number',        // type
      'Number of repetitions', // description
      false,           // required?
      1                // default value
    );

    // The tool belongs to the “Utility” category and does **not**
    // need any special resource locks.
    return createToolDefinition(
      'echo',                  // name
      'Echoes a string back to the caller', // description
      'Utility',               // category
      [messageParam, timesParam] // parameters
    );
  }

  // -----------------------------------------------------------------
  // 2.2  The **execute** method – the real work happens here.
  // -----------------------------------------------------------------
  async execute(kwargs) {
    // -----------------------------------------------------------------
    // Validate parameters (the registry already does a basic type check,
    // but we can add domain‑specific validation here).
    // -----------------------------------------------------------------
    const { message, times } = kwargs;

    // Guard against negative repetition counts
    if (times < 0) {
      const err = new ToolError(
        `times cannot be negative (got ${times})`,
        ErrorType.ValidationError
      );
      // Convert the error into a ToolResult using the central handler
      return ErrorHandler.handleError(err, 'EchoTool.execute');
    }

    // Build the output (repeat the message `times` times)
    const repeated = Array(times).fill(message).join(' ');

    // Return a **successful** ToolResult.
    return ToolResult.success(repeated, 0);
  }
}

// -----------------------------------------------------
// 3️⃣  Register the tool in a registry
// -----------------------------------------------------
const registry = new ToolRegistry();
registry.register(new EchoTool());

// Show the tool count – should be 1
console.log('🔧 Registered tools:', registry.toolCount());

// -----------------------------------------------------
// 4️⃣  Helper to run a tool via the registry
// -----------------------------------------------------
async function runTool(name, kwargs) {
  console.log(`\n🚀 Running tool "${name}" with arguments:`, kwargs);

  // 4.1  Let the registry validate & execute the tool.
  const result = await registry.executeTool(name, kwargs);

  // 4.2  Inspect the result.
  if (result.success) {
    console.log('✅ Success →', result.toString());
  } else {
    console.error('❌ Failure →', result.error);
    console.error('Metadata  →', result.metadata);
  }
}

// -----------------------------------------------------
// 5️⃣  Demonstrate a few calls
// -----------------------------------------------------
(async () => {
  // 5.1  Good call – only the required param
  await runTool('echo', { message: 'Hello, world!' });

  // 5.2  Good call – specify repeat count
  await runTool('echo', { message: 'Repeat', times: 3 });

  // 5.3  Bad call – missing required param (validation inside registry)
  await runTool('echo', { times: 2 });

  // 5.4  Bad call – domain validation (negative times)
  await runTool('echo', { message: 'Oops', times: -5 });

  // 5.5  Show the tool definition in Ollama‑compatible JSON
  console.log('\n📝 Ollama format for the "echo" tool:');
  console.log(JSON.stringify(registry.getToolDefinitions()[0], null, 2));
})();

/ *
## Example 4 – Creating, Registering & Using a Custom Tool  

In this example we will:

1. **Define a new tool** (`EchoTool`) that simply returns the string the user passes in.  
2. **Create a `ToolDefinition`** that describes the tool to the CLI / Ollama.  
3. **Register the tool** in a `ToolRegistry`.  
4. **Run the tool** via the registry (including parameter validation and proper error handling).  

All of the heavy‑lifting (parameter objects, validation helpers, `ToolResult`, etc.) lives in **`base.js`**, so we only need to import what we need.

---  

### 1️⃣  Project layout

/*
project/
│
├─ base.js          ← the core library you posted
├─ example4.js      ← our demo (the code below)
└─ package.json     ← tiny npm wrapper (optional)

/*
> **Tip** – if you want to run the demo without installing a package manager, just copy `base.js` next to `example4.js` and run `node example4.js`.

---  

### 2️⃣  The demo (`example4.js`)

---  

### 3️⃣  What the example demonstrates  

| Step | What you’ll see | Why it matters |
|------|-----------------|----------------|
| **Tool definition** | `createToolDefinition` creates a JSON‑serialisable description of the tool (name, description, parameters, category). | The CLI / UI can automatically expose the tool to users, or feed it to an LLM (e.g. Ollama) that can call the tool. |
| **Tool implementation** | `EchoTool` extends the abstract `Tool` class and implements `definition()` **and** `execute()`. | Guarantees a uniform contract for all tools (parameter validation, error handling). |
| **Registry** | `ToolRegistry.register()` stores the tool under its name (`echo`). `registry.toolCount()` proves registration. | Centralised lookup: the CLI can dynamically load *any* tool that follows the contract. |
| **Execution** | `registry.executeTool(name, kwargs)` automatically checks: <br> • Tool existence <br> • Required params <br> • Parameter type conformity | You don’t have to repeat validation logic in every tool – the registry does it. |
| **Error handling** | Wrong calls (missing `message` or negative `times`) are turned into a `ToolResult` with `success = false`. The `metadata.error_type` is set via `ErrorHandler`. | Allows the caller (CLI or LLM) to react to failures without throwing uncaught exceptions. |
| **Ollama format** | The final `console.log` prints the tool definition in the exact shape the Ollama tool‑call API expects. | You can paste this JSON directly into an Ollama model prompt to give the model knowledge of the new tool. |

---  

### 4️⃣  Run the demo  

# 1️⃣  If you don’t already have a `package.json`, initialise one (optional)
npm init -y

# 2️⃣  Install a tiny helper for pretty‑printing (optional)
npm i chalk   # just for colors – not required

# 3️⃣  Execute the example
node example4.js

**Expected output (colourised if you installed `chalk`):**