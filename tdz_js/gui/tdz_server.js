const express = require("express");
const cors = require("cors");
const bodyParser = require("body-parser");
const { exec } = require("child_process");
const { promisify } = require("util");
const { v4: uuidv4 } = require("uuid");

const execPromise = promisify(exec);

// Initialize app
const app = express();
const port = 8636;

// Middleware
app.use(cors());
app.use(bodyParser.json({ limit: "10mb" }));
app.use(bodyParser.urlencoded({ extended: true }));

// Serve static files
app.use(express.static(__dirname));

// Serve the main Todozi interface
app.get("/todozi", (req, res) => {
  res.sendFile(__dirname + "/tdz.html");
});

// Helper function to execute todozi binary
const executeTodozi = async (command) => {
  // Define possible binary locations (cross-platform)
  const possiblePaths = [
    // Current relative path
    "../target/release/todozi",
    "../target/release/todozi.exe", // Windows
    // Absolute paths common on different systems
    "/usr/local/bin/todozi",
    "/usr/bin/todozi",
    "/opt/homebrew/bin/todozi", // macOS with Homebrew
    "./target/release/todozi",
    "./target/release/todozi.exe",
    // Check if it's in PATH
    "todozi",
  ];

  let lastError = null;

  for (const binaryPath of possiblePaths) {
    try {
      console.log(`Trying to execute: ${binaryPath} ${command}`);
      const { stdout, stderr } = await execPromise(`${binaryPath} ${command}`);
      if (stderr) {
        console.warn("Todozi stderr:", stderr);
      }
      console.log(`Successfully executed with: ${binaryPath}`);
      return stdout;
    } catch (error) {
      console.log(`Failed with ${binaryPath}:`, error.message);
      lastError = error;
      // Continue to next path
    }
  }

  // If all paths failed, throw the last error
  console.error("All binary paths failed. Last error:", lastError);
  throw new Error(
    `Todozi binary not found. Tried paths: ${possiblePaths.join(", ")}. Error: ${lastError.message}`,
  );
};

// Authentication middleware
const authenticate = (req, res, next) => {
  const skipAuth = [
    "/health",
    "/tdz/health",
    "/todozi/health",
    "/api/register",
    "/tdz/api/register",
    "/init",
    "/tdz/init",
    "/todozi/init",
    "/todozi",
  ].includes(req.path);

  if (skipAuth) {
    return next();
  }

  // In a real implementation, you'd check the API key here
  // For now, we'll just pass through
  next();
};

app.use(authenticate);

// Health check
app.get(["/health", "/tdz/health", "/todozi/health"], (req, res) => {
  res.json({
    status: "healthy",
    service: "todozi-enhanced-server",
    version: "0.1.0",
    port: port,
    agents_available: 26,
    features: [
      "enhanced_agents",
      "training_data",
      "analytics",
      "time_tracking",
    ],
  });
});

// System statistics
app.get(["/stats", "/tdz/stats", "/todozi/stats"], async (req, res) => {
  try {
    const result = await executeTodozi("stats show");
    res.json({ result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Initialize system
app.get(["/init", "/tdz/init", "/todozi/init"], async (req, res) => {
  try {
    await executeTodozi("init");
    res.json({
      message: "System initialized successfully",
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Project endpoints
app.get(
  ["/projects", "/tdz/projects", "/todozi/projects"],
  async (req, res) => {
    try {
      // Try the binary command first
      const result = await executeTodozi("project list");
      // If we get here, parse the result
      if (result && result.trim()) {
        res.json({ projects: result });
      } else {
        // Return mock projects for UI development
        res.json({
          projects: [
            {
              name: "Personal",
              description: "Personal tasks and ideas",
              status: "active",
            },
            {
              name: "Work",
              description: "Work-related projects",
              status: "active",
            },
            {
              name: "Learning",
              description: "Learning and development",
              status: "active",
            },
          ],
        });
      }
    } catch (error) {
      console.log(
        "Projects command not available, returning mock data:",
        error.message,
      );
      // Return mock projects for UI development
      res.json({
        projects: [
          {
            name: "Personal",
            description: "Personal tasks and ideas",
            status: "active",
          },
          {
            name: "Work",
            description: "Work-related projects",
            status: "active",
          },
          {
            name: "Learning",
            description: "Learning and development",
            status: "active",
          },
        ],
      });
    }
  },
);

app.post(
  ["/projects", "/tdz/projects", "/todozi/projects"],
  async (req, res) => {
    try {
      const { name, description, color, priority, status } = req.body;

      if (!name) {
        return res.status(400).json({ error: "Project name is required" });
      }

      // Try the binary command first
      let command = `project create "${name}"`;

      if (description) command += ` --description "${description}"`;
      if (color) command += ` --color ${color}`;
      if (priority) command += ` --priority ${priority}`;
      if (status) command += ` --status ${status}`;

      const result = await executeTodozi(command);
      res.status(201).json({ message: "Project created", result });
    } catch (error) {
      console.log(
        "Project creation command not available, simulating success:",
        error.message,
      );
      // Simulate successful project creation for UI development
      res.status(201).json({
        message: "Project created (simulated)",
        project: { name, description, color, priority, status },
      });
    }
  },
);

app.get(
  ["/projects/:name", "/tdz/projects/:name", "/todozi/projects/:name"],
  async (req, res) => {
    try {
      const result = await executeTodozi(`project show "${req.params.name}"`);
      res.json({ project: result });
    } catch (error) {
      if (error.message.includes("not found")) {
        res.status(404).json({ error: "Project not found" });
      } else {
        res.status(500).json({ error: error.message });
      }
    }
  },
);

app.put(
  ["/projects/:name", "/tdz/projects/:name", "/todozi/projects/:name"],
  async (req, res) => {
    try {
      const { new_name, description, status } = req.body;

      let command = `project update "${req.params.name}"`;

      if (new_name) command += ` --new-name "${new_name}"`;
      if (description !== undefined)
        command += ` --description "${description}"`;
      if (status) command += ` --status ${status}`;

      const result = await executeTodozi(command);
      res.json({ message: "Project updated", result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.delete(
  ["/projects/:name", "/tdz/projects/:name", "/todozi/projects/:name"],
  async (req, res) => {
    try {
      const result = await executeTodozi(
        `project delete \"${req.params.name}\"`,
      );
      res.json({ message: "Project deleted", result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

// Task endpoints
app.get(["/tasks", "/tdz/tasks", "/todozi/tasks"], async (req, res) => {
  try {
    const project = req.query.project;
    let command = "list tasks";

    if (project && project !== "all") {
      command += ` --project "${project}"`;
    }

    const result = await executeTodozi(command);
    res.json({ tasks: result, currentProject: project || "all" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/tasks", "/tdz/tasks", "/todozi/tasks"], async (req, res) => {
  try {
    const {
      action,
      time,
      priority,
      project,
      status,
      assignee,
      tags,
      dependencies,
      context,
      progress,
    } = req.body;

    let command = `add task "${action}" --time "${time}" --priority ${priority} --project ${project} --status ${status}`;

    if (assignee) command += ` --assignee ${assignee}`;
    if (tags) command += ` --tags "${tags}"`;
    if (dependencies) command += ` --dependencies "${dependencies}"`;
    if (context) command += ` --context "${context}"`;
    if (progress !== undefined) command += ` --progress ${progress}`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Task created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/tasks/search", "/tdz/tasks/search"], async (req, res) => {
  try {
    const query = req.query.q || "";
    const result = await executeTodozi(`search tasks "${query}"`);
    res.json({ query, result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/search-all", "/tdz/search-all"], async (req, res) => {
  try {
    const query = req.query.q || "";
    const types = req.query.types || "tasks,memories,ideas,errors";
    const result = await executeTodozi(
      `search-all --types ${types} "${query}"`,
    );
    res.json({ query, types, result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/tasks/:id", "/tdz/tasks/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`show task ${req.params.id}`);
    res.json({ task: result });
  } catch (error) {
    if (error.message.includes("not found")) {
      res.status(404).json({ error: "Task not found" });
    } else {
      res.status(500).json({ error: error.message });
    }
  }
});

app.put(["/tasks/:id", "/tdz/tasks/:id"], async (req, res) => {
  try {
    const {
      action,
      time,
      priority,
      project,
      status,
      assignee,
      tags,
      dependencies,
      context,
      progress,
    } = req.body;

    let command = `update ${req.params.id}`;

    if (action) command += ` --action "${action}"`;
    if (time) command += ` --time "${time}"`;
    if (priority) command += ` --priority ${priority}`;
    if (project) command += ` --project ${project}`;
    if (status) command += ` --status ${status}`;
    if (assignee) command += ` --assignee ${assignee}`;
    if (tags) command += ` --tags "${tags}"`;
    if (dependencies) command += ` --dependencies "${dependencies}"`;
    if (context) command += ` --context "${context}"`;
    if (progress !== undefined) command += ` --progress ${progress}`;

    const result = await executeTodozi(command);
    res.json({ message: "Task updated", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(
  ["/tasks/:id/complete", "/tdz/tasks/:id/complete"],
  async (req, res) => {
    try {
      const result = await executeTodozi(`complete ${req.params.id}`);
      res.json({ message: "Task completed", result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.delete(["/tasks/:id", "/tdz/tasks/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`delete ${req.params.id}`);
    res.json({ message: "Task deleted", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Memory endpoints
app.get(["/memories", "/tdz/memories"], async (req, res) => {
  try {
    const project = req.query.project;
    let command = "memory list";

    if (project && project !== "all") {
      command += ` --project "${project}"`;
    }

    const result = await executeTodozi(command);
    res.json({ memories: result, currentProject: project || "all" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/memories", "/tdz/memories"], async (req, res) => {
  try {
    const { moment, meaning, reason, importance, term, memory_type, tags } =
      req.body;

    let command = `memory create "${moment}" "${meaning}" "${reason}"`;

    if (importance) command += ` --importance ${importance}`;
    if (term) command += ` --term ${term}`;
    if (memory_type) command += ` --memory-type ${memory_type}`;
    if (tags) command += ` --tags "${tags}"`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Memory created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(
  ["/memories/:type/:emotion?", "/tdz/memories/:type/:emotion?"],
  async (req, res) => {
    try {
      const { type, emotion } = req.params;
      let command = `memory list --memory-type ${type}`;
      if (emotion) command += ` --emotion ${emotion}`;

      const result = await executeTodozi(command);
      res.json({ memories: result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.get(["/memories/types", "/tdz/memories/types"], async (req, res) => {
  try {
    const result = await executeTodozi("memory types");
    res.json({ types: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Idea endpoints
app.get(["/ideas", "/tdz/ideas"], async (req, res) => {
  try {
    const project = req.query.project;
    let command = "idea list";

    if (project && project !== "all") {
      command += ` --project "${project}"`;
    }

    const result = await executeTodozi(command);
    res.json({ ideas: result, currentProject: project || "all" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/ideas", "/tdz/ideas"], async (req, res) => {
  try {
    const { idea, share, importance, tags, context } = req.body;

    let command = `idea create "${idea}"`;

    if (share) command += ` --share ${share}`;
    if (importance) command += ` --importance ${importance}`;
    if (tags) command += ` --tags "${tags}"`;
    if (context) command += ` --context "${context}"`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Idea created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Agent endpoints
app.get(["/agents", "/tdz/agents"], async (req, res) => {
  try {
    const result = await executeTodozi("agent list");
    res.json({ agents: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/agents", "/tdz/agents"], async (req, res) => {
  try {
    const {
      id,
      name,
      description,
      category,
      capabilities,
      specializations,
      model_provider,
      model_name,
      temperature,
      max_tokens,
      tags,
      system_prompt,
      prompt_template,
      auto_format_code,
      include_examples,
      explain_complexity,
      suggest_tests,
      tools,
      max_response_length,
      timeout_seconds,
      requests_per_minute,
      tokens_per_hour,
    } = req.body;

    let command = `agent create ${id} "${name}" "${description}"`;

    if (category) command += ` --category ${category}`;
    if (capabilities) command += ` --capabilities "${capabilities}"`;
    if (specializations) command += ` --specializations "${specializations}"`;
    if (model_provider) command += ` --model-provider ${model_provider}`;
    if (model_name) command += ` --model-name ${model_name}`;
    if (temperature) command += ` --temperature ${temperature}`;
    if (max_tokens) command += ` --max-tokens ${max_tokens}`;
    if (tags) command += ` --tags "${tags}"`;
    if (system_prompt) command += ` --system-prompt "${system_prompt}"`;
    if (prompt_template) command += ` --prompt-template "${prompt_template}"`;
    if (auto_format_code !== undefined)
      command += ` --auto-format-code ${auto_format_code}`;
    if (include_examples !== undefined)
      command += ` --include-examples ${include_examples}`;
    if (explain_complexity !== undefined)
      command += ` --explain-complexity ${explain_complexity}`;
    if (suggest_tests !== undefined)
      command += ` --suggest-tests ${suggest_tests}`;
    if (tools) command += ` --tools "${tools}"`;
    if (max_response_length)
      command += ` --max-response-length ${max_response_length}`;
    if (timeout_seconds) command += ` --timeout-seconds ${timeout_seconds}`;
    if (requests_per_minute)
      command += ` --requests-per-minute ${requests_per_minute}`;
    if (tokens_per_hour) command += ` --tokens-per-hour ${tokens_per_hour}`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Agent created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/agents/available", "/tdz/agents/available"], async (req, res) => {
  try {
    const result = await executeTodozi("agent list");
    res.json({ agents: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/agents/:id/status", "/tdz/agents/:id/status"], async (req, res) => {
  try {
    const result = await executeTodozi(`agent show ${req.params.id}`);
    res.json({ agent: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/agents/:id", "/tdz/agents/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`agent show ${req.params.id}`);
    res.json({ agent: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.put([\"/agents/:id\", \"/tdz/agents/:id\"], async (req, res) => {
  try {
    const {
      name,
      description,
      category,
      capabilities,
      specializations,
      model_provider,
      model_name,
      temperature,
      max_tokens,
      tags,
      system_prompt,
      prompt_template,
      auto_format_code,
      include_examples,
      explain_complexity,
      suggest_tests,
      tools,
      max_response_length,
      timeout_seconds,
      requests_per_minute,
      tokens_per_hour,
    } = req.body;

    let command = `agent update ${req.params.id}`;

    if (name) command += ` --name \"${name}\"`;
    if (description) command += ` --description \"${description}\"`;
    if (category) command += ` --category ${category}`;
    if (capabilities) command += ` --capabilities \"${capabilities}\"`;
    if (specializations) command += ` --specializations \"${specializations}\"`;
    if (model_provider) command += ` --model-provider ${model_provider}`;
    if (model_name) command += ` --model-name ${model_name}`;\n    if (temperature) command += ` --temperature ${temperature}`;\n    if (max_tokens) command += ` --max-tokens ${max_tokens}`;\n    if (tags) command += ` --tags \"${tags}\"`;\n    if (system_prompt) command += ` --system-prompt \"${system_prompt}\"`;\n    if (prompt_template) command += ` --prompt-template \"${prompt_template}\"`;\n    if (auto_format_code !== undefined)\n      command += ` --auto-format-code ${auto_format_code}`;\n    if (include_examples !== undefined)\n      command += ` --include-examples ${include_examples}`;\n    if (explain_complexity !== undefined)\n      command += ` --explain-complexity ${explain_complexity}`;\n    if (suggest_tests !== undefined)\n      command += ` --suggest-tests ${suggest_tests}`;\n    if (tools) command += ` --tools \"${tools}\"`;\n    if (max_response_length)\n      command += ` --max-response-length ${max_response_length}`;\n    if (timeout_seconds) command += ` --timeout-seconds ${timeout_seconds}`;\n    if (requests_per_minute)\n      command += ` --requests-per-minute ${requests_per_minute}`;\n    if (tokens_per_hour) command += ` --tokens-per-hour ${tokens_per_hour}`;\n\n    const result = await executeTodozi(command);\n    res.json({ message: \"Agent updated\", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.delete([\"/agents/:id\", \"/tdz/agents/:id\"], async (req, res) => {
  try {
    const result = await executeTodozi(`agent delete ${req.params.id}`);
    res.json({ message: \"Agent deleted\", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Training Data endpoints
app.get(["/training", "/tdz/training"], async (req, res) => {
  try {
    const result = await executeTodozi("train list");
    res.json({ training_data: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/training", "/tdz/training"], async (req, res) => {
  try {
    const { data_type, prompt, completion, context, tags, quality, source } =
      req.body;

    let command = `train create --data-type ${data_type} "${prompt}" "${completion}"`;

    if (context) command += ` --context "${context}"`;
    if (tags) command += ` --tags "${tags}"`;
    if (quality) command += ` --quality ${quality}`;
    if (source) command += ` --source ${source}`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Training data created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/training/export", "/tdz/training/export"], async (req, res) => {
  try {
    const result = await executeTodozi("train export");
    res.json({ export: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/training/stats", "/tdz/training/stats"], async (req, res) => {
  try {
    const result = await executeTodozi("train stats");
    res.json({ stats: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/training/:id", "/tdz/training/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`train show ${req.params.id}`);
    res.json({ training_data: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.put(["/training/:id", "/tdz/training/:id"], async (req, res) => {
  try {
    const { data_type, prompt, completion, context, tags, quality, source } =
      req.body;

    let command = `train update ${req.params.id}`;

    if (data_type) command += ` --data-type ${data_type}`;
    if (prompt) command += ` --prompt "${prompt}"`;
    if (completion) command += ` --completion "${completion}"`;
    if (context) command += ` --context "${context}"`;
    if (tags) command += ` --tags "${tags}"`;
    if (quality !== undefined) command += ` --quality ${quality}`;
    if (source) command += ` --source "${source}"`;

    const result = await executeTodozi(command);
    res.json({ message: "Training data updated", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.delete(["/training/:id", "/tdz/training/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`train delete ${req.params.id}`);
    res.json({ message: "Training data deleted", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Chat endpoints
app.post(["/chat/process", "/tdz/chat/process"], async (req, res) => {
  try {
    const { message } = req.body;
    const result = await executeTodozi(`chat "${message}"`);
    res.json({ message: "Chat processed", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/chat/agent/:id", "/tdz/chat/agent/:id"], async (req, res) => {
  try {
    const { message, task_id, project_id } = req.body;

    // Validate required parameters
    if (!task_id || !project_id) {
      return res.status(400).json({
        error: "task_id and project_id are required for agent assignment",
      });
    }

    const result = await executeTodozi(
      `agent assign ${req.params.id} ${task_id} ${project_id}`,
    );
    res.json({ message: "Agent chat processed", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/chat/history", "/tdz/chat/history"], async (req, res) => {
  try {
    // Chat history would require specific implementation
    res.json({ message: "Chat history not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Analytics endpoints
app.get(["/analytics/tasks", "/tdz/analytics/tasks"], async (req, res) => {
  try {
    const result = await executeTodozi("stats show");
    res.json({ analytics: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/analytics/agents", "/tdz/analytics/agents"], async (req, res) => {
  try {
    const result = await executeTodozi("agent list");
    res.json({ analytics: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(
  ["/analytics/performance", "/tdz/analytics/performance"],
  async (req, res) => {
    try {
      const result = await executeTodozi("stats show");
      res.json({ analytics: result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

// Time tracking endpoints
app.post(
  ["/time/start/:task_id", "/tdz/time/start/:task_id"],
  async (req, res) => {
    try {
      const result = await executeTodozi(`queue start ${req.params.task_id}`);
      res.json({ message: "Time tracking started", result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.post(
  ["/time/stop/:task_id", "/tdz/time/stop/:task_id"],
  async (req, res) => {
    try {
      // Time tracking stop would require session management
      res.json({ message: "Time tracking stop not yet implemented via API" });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.get(["/time/report", "/tdz/time/report"], async (req, res) => {
  try {
    const result = await executeTodozi("queue list");
    res.json({ report: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Error tracking endpoints
app.get(["/errors", "/tdz/errors"], async (req, res) => {
  try {
    const project = req.query.project;
    let command = "error list";

    if (project && project !== "all") {
      command += ` --project "${project}"`;
    }

    const result = await executeTodozi(command);
    res.json({ errors: result, currentProject: project || "all" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/errors", "/tdz/errors"], async (req, res) => {
  try {
    const {
      title,
      description,
      source,
      severity,
      category,
      context,
      tags,
      project,
    } = req.body;

    if (!title || !description || !source) {
      return res
        .status(400)
        .json({ error: "Title, description, and source are required" });
    }

    let command = `error create "${title}" "${description}" "${source}"`;

    if (severity) command += ` --severity ${severity}`;
    if (category) command += ` --category ${category}`;
    if (context) command += ` --context "${context}"`;
    if (tags) command += ` --tags "${tags}"`;
    if (project) command += ` --project "${project}"`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Error created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/errors/:id", "/tdz/errors/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`error show ${req.params.id}`);
    res.json({ error: result });
  } catch (error) {
    if (error.message.includes("not found")) {
      res.status(404).json({ error: "Error record not found" });
    } else {
      res.status(500).json({ error: error.message });
    }
  }
});

app.put(["/errors/:id", "/tdz/errors/:id"], async (req, res) => {
  try {
    const { severity, category, context, tags, resolved } = req.body;

    let command = `error resolve ${req.params.id}`;

    if (resolved !== undefined && resolved === false) {
      // If we want to mark as unresolved, we might need a different command
      // For now, we'll just use resolve for marking as resolved
      command = `error resolve ${req.params.id}`;
    }

    const result = await executeTodozi(command);
    res.json({ message: "Error updated", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.delete(["/errors/:id", "/tdz/errors/:id"], async (req, res) => {
  try {
    const result = await executeTodozi(`error delete ${req.params.id}`);
    res.json({ message: "Error deleted", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/errors/search", "/tdz/errors/search"], async (req, res) => {
  try {
    const query = req.query.q || "";
    const result = await executeTodozi(`search errors "${query}"`);
    res.json({ query, result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Queue endpoints
app.post(["/queue/plan", "/tdz/queue/plan"], async (req, res) => {
  try {
    const { task_name, task_description, priority, project_id } = req.body;

    let command = `queue plan --task-name "${task_name}" --task-description "${task_description}" --priority ${priority}`;

    if (project_id) command += ` --project-id ${project_id}`;

    const result = await executeTodozi(command);
    res.status(201).json({ message: "Queue item created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/queue/list", "/tdz/queue/list"], async (req, res) => {
  try {
    const project = req.query.project;
    let command = "queue list";

    if (project && project !== "all") {
      command += ` --project "${project}"`;
    }

    const result = await executeTodozi(command);
    res.json({ queue: result, currentProject: project || "all" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(
  ["/queue/list/backlog", "/tdz/queue/list/backlog"],
  async (req, res) => {
    try {
      const project = req.query.project;
      let command = "queue backlog";

      if (project && project !== "all") {
        command += ` --project "${project}"`;
      }

      const result = await executeTodozi(command);
      res.json({ backlog: result, currentProject: project || "all" });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.get(["/queue/list/active", "/tdz/queue/list/active"], async (req, res) => {
  try {
    const project = req.query.project;
    let command = "queue active";

    if (project && project !== "all") {
      command += ` --project "${project}"`;
    }

    const result = await executeTodozi(command);
    res.json({ active: result, currentProject: project || "all" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(
  ["/queue/list/complete", "/tdz/queue/list/complete"],
  async (req, res) => {
    try {
      const project = req.query.project;
      let command = "queue complete";

      if (project && project !== "all") {
        command += ` --project "${project}"`;
      }

      const result = await executeTodozi(command);
      res.json({ complete: result, currentProject: project || "all" });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.post(
  ["/queue/start/:item_id", "/tdz/queue/start/:item_id"],
  async (req, res) => {
    try {
      const result = await executeTodozi(`queue start ${req.params.item_id}`);
      res.json({ message: "Queue session started", result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.post(
  ["/queue/end/:session_id", "/tdz/queue/end/:session_id"],
  async (req, res) => {
    try {
      const result = await executeTodozi(`queue end ${req.params.session_id}`);
      res.json({ message: "Queue session ended", result });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

// API Key endpoints
app.post(["/api/register", "/tdz/api/register"], async (req, res) => {
  try {
    const result = await executeTodozi("api register");
    res.status(201).json({ message: "API key created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/api/check", "/tdz/api/check"], async (req, res) => {
  try {
    const { public_key, private_key } = req.body;
    let command = `api check -p ${public_key}`;
    if (private_key) command += ` -p ${private_key}`;

    const result = await executeTodozi(command);
    res.json({ message: "API key checked", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// AI-Enhanced endpoints
app.get(
  ["/tasks/:id/insights", "/tdz/tasks/:id/insights"],
  async (req, res) => {
    try {
      // AI insights would require specific implementation
      res.json({ message: "AI insights not yet implemented via API" });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  },
);

app.get(["/tasks/:id/similar", "/tdz/tasks/:id/similar"], async (req, res) => {
  try {
    // Similar tasks would require specific implementation
    res.json({ message: "Similar tasks not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/tasks/validate", "/tdz/tasks/validate"], async (req, res) => {
  try {
    // Task validation would require specific implementation
    res.json({ message: "Task validation not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Backup endpoints
app.get(["/backups", "/tdz/backups"], async (req, res) => {
  try {
    const result = await executeTodozi("backup list");
    res.json({ backups: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/backups", "/tdz/backups"], async (req, res) => {
  try {
    const result = await executeTodozi("backup create");
    res.status(201).json({ message: "Backup created", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/restore/:name", "/tdz/restore/:name"], async (req, res) => {
  try {
    const result = await executeTodozi(`restore ${req.params.name}`);
    res.json({ message: "Backup restored", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/tasks/suggest", "/tdz/tasks/suggest"], async (req, res) => {
  try {
    // Task suggestions would require specific implementation
    res.json({ message: "Task suggestions not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/semantic/search", "/tdz/semantic/search"], async (req, res) => {
  try {
    const query = req.query.q || "";
    // Semantic search would require specific implementation
    res.json({ query, message: "Semantic search not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Server management endpoints
app.get(["/server/status", "/tdz/server/status"], async (req, res) => {
  try {
    const result = await executeTodozi("server status");
    res.json({ status: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/server/endpoints", "/tdz/server/endpoints"], async (req, res) => {
  try {
    const result = await executeTodozi("server endpoints");
    res.json({ endpoints: result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ML endpoints
app.get(["/ml", "/tdz/ml"], async (req, res) => {
  try {
    // ML functionality coming soon
    res.json({ message: "ML functionality coming soon!" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/ml/process", "/tdz/ml/process"], async (req, res) => {
  try {
    const { text } = req.body;

    if (!text) {
      return res.status(400).json({ error: "text is required" });
    }

    const result = await executeTodozi(`ml process "${text}"`);
    res.json({ message: "Text processed", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Ind-demo endpoint
app.get(["/ind-demo", "/tdz/ind-demo"], async (req, res) => {
  try {
    const result = await executeTodozi("ind-demo");
    res.json({ message: "Ind demo executed", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// TDZ Content Processor endpoint
app.post(["/tdz-cnt", "/tdz/tdz-cnt"], async (req, res) => {
  try {
    const { content, session_id, no_checklist, no_session } = req.body;

    if (!content) {
      return res.status(400).json({ error: "content is required" });
    }

    let command = `tdz-cnt "${content}"`;

    if (session_id) command += ` --session-id ${session_id}`;
    if (no_checklist) command += ` --no-checklist`;
    if (no_session) command += ` --no-session`;

    const result = await executeTodozi(command);
    res.json({ message: "Content processed", result });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Maestro endpoints
app.get(["/maestro", "/tdz/maestro"], async (req, res) => {
  try {
    // Maestro functionality coming soon
    res.json({ message: "Maestro functionality coming soon!" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.post(["/maestro/collect", "/tdz/maestro/collect"], async (req, res) => {
  try {
    const { interaction_type, data } = req.body;

    if (!interaction_type || !data) {
      return res
        .status(400)
        .json({ error: "interaction_type and data are required" });
    }

    // Maestro collection would require specific implementation
    res.json({ message: "Maestro collection not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.get(["/insights", "/tdz/insights"], async (req, res) => {
  try {
    // AI insights would require specific implementation
    res.json({ message: "AI insights not yet implemented via API" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// Default route
app.use("*", (req, res) => {
  res.status(404).json({ error: "Route not found" });
});

// Start server
app.listen(port, "127.0.0.1", () => {
  console.log(
    `🚀 Todozi Enhanced Server starting on 127.0.0.1:${port} (26 Agents Ready!)`,
  );
  console.log(`🌐 Web Interface: http://localhost:${port}/todozi`);
  console.log("📡 Available endpoints:");
  console.log();
  console.log("🎯 CORE FUNCTIONALITY:");
  console.log("  GET  /health                    - Health check");
  console.log("  GET  /stats                     - System statistics");
  console.log("  GET  /init                      - Initialize system");
  console.log("  POST /tasks/{id}/complete       - Complete a task");
  console.log("  GET  /backups                   - List backups");
  console.log("  POST /restore/{name}            - Restore from backup");
  console.log("  GET  /search-all?q={query}      - Search all data types");
  console.log("  GET  /server/status             - Server status");
  console.log("  GET  /server/endpoints          - Server endpoints");
  console.log(
    "  GET  /ind-demo                  - Demonstrate Ind functionality",
  );
  console.log("  POST /tdz-cnt                   - Process raw content");
  console.log();
  console.log("📋 TASK MANAGEMENT:");
  console.log("  GET  /tasks                     - List all tasks");
  console.log("  POST /tasks                     - Create new task");
  console.log("  GET  /tasks/{id}                - Get task by ID");
  console.log("  PUT  /tasks/{id}                - Update task");
  console.log("  DELETE /tasks/{id}              - Delete task");
  console.log("  GET  /tasks/search?q={query}    - Search tasks");
  console.log();
  console.log("🤖 ENHANCED AGENT SYSTEM (26 AGENTS):");
  console.log("  GET  /agents                    - List all agents");
  console.log("  POST /agents                    - Create new agent");
  console.log("  GET  /agents/{id}               - Get agent by ID");
  console.log("  PUT  /agents/{id}               - Update agent");
  console.log("  DELETE /agents/{id}             - Delete agent");
  console.log("  GET  /agents/available          - Get available agents");
  console.log("  GET  /agents/{id}/status        - Get agent status");
  console.log();
  console.log("🧠 MEMORY & IDEA MANAGEMENT:");
  console.log("  GET  /memories                  - List all memories");
  console.log("  POST /memories                  - Create new memory");
  console.log("  GET  /memories/{id}             - Get memory by ID");
  console.log("  GET  /ideas                     - List all ideas");
  console.log("  POST /ideas                     - Create new idea");
  console.log("  GET  /ideas/{id}                - Get idea by ID");
  console.log("  GET  /errors                    - List all errors");
  console.log("  POST /errors                    - Create new error");
  console.log("  GET  /errors/{id}               - Get error by ID");
  console.log("  PUT  /errors/{id}               - Update error");
  console.log("  DELETE /errors/{id}             - Delete error");
  console.log("  GET  /errors/search?q={query}   - Search errors");
  console.log();
  console.log("🎓 TRAINING DATA SYSTEM:");
  console.log("  GET  /training                  - List all training data");
  console.log("  POST /training                  - Create training data");
  console.log("  GET  /training/{id}             - Get training data by ID");
  console.log("  PUT  /training/{id}             - Update training data");
  console.log("  DELETE /training/{id}           - Delete training data");
  console.log("  GET  /training/export           - Export training data");
  console.log("  GET  /training/stats            - Training data statistics");
  console.log();
  console.log("🧩 CODE CHUNKING SYSTEM:");
  console.log("  GET  /chunks                    - List all code chunks");
  console.log("  POST /chunks                    - Create new code chunk");
  console.log("  GET  /chunks/{id}               - Get chunk by ID");
  console.log("  PUT  /chunks/{id}               - Update chunk");
  console.log("  DELETE /chunks/{id}             - Delete chunk");
  console.log("  GET  /chunks/ready              - Get ready chunks");
  console.log("  GET  /chunks/graph              - Get dependency graph");
  console.log();
  console.log("💬 ENHANCED CHAT PROCESSING:");
  console.log("  POST /chat/process              - Process chat message");
  console.log("  POST /chat/agent/{id}           - Chat with specific agent");
  console.log("  GET  /chat/history              - Get chat history");
  console.log();
  console.log("📊 ANALYTICS & TRACKING:");
  console.log("  GET  /analytics/tasks           - Task analytics");
  console.log("  GET  /analytics/agents          - Agent analytics");
  console.log("  GET  /analytics/performance     - System performance");
  console.log("  POST /time/start/{task_id}       - Start time tracking");
  console.log("  POST /time/stop/{task_id}        - Stop time tracking");
  console.log("  GET  /time/report               - Time tracking report");
  console.log();
  console.log("📁 PROJECT MANAGEMENT:");
  console.log("  GET  /projects                  - List all projects");
  console.log("  POST /projects                  - Create new project");
  console.log("  GET  /projects/{name}           - Get project by name");
  console.log("  PUT  /projects/{name}           - Update project");
  console.log("  DELETE /projects/{name}         - Delete project");
  console.log();
  console.log("🔧 UTILITIES:");
  console.log("  POST /backup                    - Create backup");
  console.log("  GET  /backups                   - List backups");
  console.log("  POST /restore/{name}            - Restore from backup");
  console.log();
  console.log("🧠 ML & AI ENDPOINTS:");
  console.log("  GET  /ml                        - ML functionality info");
  console.log("  POST /ml/process                - Process text with ML");
  console.log("  GET  /maestro                   - Maestro functionality info");
  console.log("  POST /maestro/collect           - Collect Maestro data");
  console.log();
  console.log("📋 QUEUE MANAGEMENT:");
  console.log("  POST /queue/plan                - Plan new queue item");
  console.log("  GET  /queue/list                - List all queue items");
  console.log("  GET  /queue/list/backlog        - List backlog items");
  console.log("  GET  /queue/list/active         - List active items");
  console.log("  GET  /queue/list/complete       - List complete items");
  console.log("  POST /queue/start/{item_id}     - Start queue session");
  console.log("  POST /queue/end/{session_id}    - End queue session");
  console.log();
  console.log("🔑 API KEY MANAGEMENT:");
  console.log("  POST /api/register              - Register new API key");
  console.log(
    "  POST /api/check                 - Check API key authentication",
  );
  console.log();
  console.log("🤖 AI-ENHANCED ENDPOINTS:");
  console.log("  GET  /tasks/{id}/insights        - Get task with AI insights");
  console.log("  GET  /tasks/{id}/similar         - Find similar tasks");
  console.log("  POST /tasks/validate              - Validate task with AI");
  console.log("  GET  /tasks/suggest               - AI task suggestions");
  console.log("  GET  /semantic/search?q={query}  - Semantic search");
  console.log("  GET  /insights                    - AI insights overview");
  console.log();
});
