// Configuration
const API_BASE = "http://127.0.0.1:8636";
let API_TOKEN = null;
let currentView = "tasks";
let currentProject = "all";
let currentTasks = [];
let currentMemories = [];
let currentIdeas = [];
let currentAgents = [];
let currentTrainingData = [];
let currentQueueItems = [];
let currentFeelings = []; // This should be currentErrors
let currentProjects = [];
let currentTaskId = null;
let currentMemoryId = null;
let currentIdeaId = null;
let currentAgentId = null;
let currentTrainingId = null;
let currentQueueId = null;
let currentFeelingId = null; // This should be currentErrorId
let currentQueueTab = "all";

// DOM Elements (will be assigned in DOMContentLoaded)
let taskList,
  memoryList,
  ideaList,
  agentList,
  trainingList,
  queueList,
  feelingList,
  apiKeyList,
  chatMessages,
  chatInput,
  agentSelect,
  toast,
  projectSelect;

// API Helper Functions
function setApiToken(token) {
  API_TOKEN = token;
  if (token) {
    localStorage.setItem("todozi_api_token", token);
  } else {
    localStorage.removeItem("todozi_api_token");
  }
}

function loadApiToken() {
  const token = localStorage.getItem("todozi_api_token");
  if (token) {
    API_TOKEN = token;
  }
}

async function apiRequest(endpoint, options = {}) {
  try {
    const url = `${API_BASE}${endpoint}`;

    const headers = {
      "Content-Type": "application/json",
      ...options.headers,
    };

    if (API_TOKEN) {
      headers["x-api-token"] = API_TOKEN;
    }

    const response = await fetch(url, {
      headers,
      ...options,
    });

    if (!response.ok) {
      let errMsg = `HTTP ${response.status}`;
      try {
        const responseClone = response.clone();
        const errJson = await responseClone.json();
        errMsg += ` – ${JSON.stringify(errJson)}`;
      } catch (_) {
        try {
          const responseClone = response.clone();
          const txt = await responseClone.text();
          errMsg += ` – ${txt}`;
        } catch (__) {
          errMsg += ` – ${response.statusText}`;
        }
      }
      throw new Error(errMsg);
    }

    if (response.status === 204) return null;

    return await response.json();
  } catch (error) {
    console.error("API request failed:", error);
    showToast(error.message || "Network error. Please check your connection.");
    throw error;
  }
}

// UI Functions
function switchView(viewType) {
  currentView = viewType;

  // Update nav
  document.querySelectorAll(".nav-item").forEach((item) => {
    item.classList.remove("active");
  });
  event.target?.closest(".nav-item")?.classList.add("active");

  // Hide all views
  const viewIds = [
    "tasksView",
    "memoriesView",
    "ideasView",
    "analyticsView",
    "agentsView",
    "trainingView",
    "queueView",
    "feelingsView",
    "chatView",
    "apiKeysView",
  ];
  viewIds.forEach((id) => {
    const element = document.getElementById(id);
    if (element) element.style.display = "none";
  });

  // Show selected view
  const selectedView = document.getElementById(`${viewType}View`);
  if (selectedView) {
    selectedView.style.display = "block";
  }

  const titleElement = document.getElementById("currentViewTitle");
  if (titleElement) {
    titleElement.textContent = getViewTitle(viewType);
  }

  // Load data for the view
  switch (viewType) {
    case "tasks":
      loadTasks();
      break;
    case "memories":
      loadMemories();
      break;
    case "ideas":
      loadIdeas();
      break;
    case "agents":
      loadAgents();
      break;
    case "training":
      loadTrainingData();
      break;
    case "queue":
      loadQueueItems();
      break;
    case "feelings":
      loadErrors(); // Changed from loadFeelings to loadErrors
      break;
    case "chat":
      loadAgentsForChat();
      break;
    case "api-keys":
      loadApiKeys();
      break;
  }
}

function getViewTitle(viewType) {
  const titles = {
    tasks: "Tasks",
    memories: "Memories",
    ideas: "Ideas",
    analytics: "Analytics Dashboard",
    agents: "AI Agents",
    training: "Training Data",
    queue: "Queue",
    feelings: "Error Tracking", // Updated title
    chat: "Chat",
    "api-keys": "API Keys",
  };
  return titles[viewType] || viewType;
}

function switchQueueTab(tab) {
  currentQueueTab = tab;
  document
    .querySelectorAll(".tab")
    .forEach((t) => t.classList.remove("active"));
  event.target.classList.add("active");
  loadQueueItems();
}

function showToast(message) {
  if (!toast) return; // DOM not ready yet

  toast.textContent = message;
  toast.classList.add("show");

  setTimeout(() => {
    toast.classList.remove("show");
  }, 3000);
}

function closeModal(modalId) {
  document.getElementById(modalId).classList.remove("active");
}

function showCreateModal() {
  document.getElementById("createTaskName").value = "";
  document.getElementById("createTaskTime").value = "";
  document.getElementById("createTaskProject").value = "";
  document.getElementById("createTaskDescription").value = "";
  document.getElementById("createTaskPriority").value = "medium";
  document.getElementById("createTaskStatus").value = "todo";
  document.getElementById("createTaskTags").value = "";
  document.getElementById("createTaskDependencies").value = "";
  document.getElementById("createTaskAssignee").value = "ai";
  document.getElementById("createTaskProgress").value = "";
  document.getElementById("createTaskModal").classList.add("active");
}

function showCreateMemoryModal() {
  document.getElementById("createMemoryMoment").value = "";
  document.getElementById("createMemoryMeaning").value = "";
  document.getElementById("createMemoryReason").value = "";
  document.getElementById("createMemoryImportance").value = "medium";
  document.getElementById("createMemoryTerm").value = "long";
  document.getElementById("createMemoryType").value = "standard";
  document.getElementById("createMemoryTags").value = "";
  document.getElementById("createMemoryModal").classList.add("active");
}

function showCreateIdeaModal() {
  document.getElementById("createIdeaContent").value = "";
  document.getElementById("createIdeaShare").value = "private";
  document.getElementById("createIdeaImportance").value = "medium";
  document.getElementById("createIdeaTags").value = "";
  document.getElementById("createIdeaContext").value = "";
  document.getElementById("createIdeaModal").classList.add("active");
}

function showCreateAgentModal() {
  document.getElementById("createAgentId").value = "";
  document.getElementById("createAgentName").value = "";
  document.getElementById("createAgentDescription").value = "";
  document.getElementById("createAgentCategory").value = "general";
  document.getElementById("createAgentModelProvider").value = "todozi";
  document.getElementById("createAgentModelName").value = "baton";
  document.getElementById("createAgentTemperature").value = "0.2";
  document.getElementById("createAgentMaxTokens").value = "4096";
  document.getElementById("createAgentCapabilities").value = "";
  document.getElementById("createAgentSpecializations").value = "";
  document.getElementById("createAgentTags").value = "";
  document.getElementById("createAgentModal").classList.add("active");
}

function showCreateTrainingModal() {
  document.getElementById("createTrainingDataType").value = "instruction";
  document.getElementById("createTrainingPrompt").value = "";
  document.getElementById("createTrainingCompletion").value = "";
  document.getElementById("createTrainingContext").value = "";
  document.getElementById("createTrainingTags").value = "";
  document.getElementById("createTrainingQuality").value = "";
  document.getElementById("createTrainingSource").value = "manual";
  document.getElementById("createTrainingModal").classList.add("active");
}

function showCreateQueueModal() {
  document.getElementById("createQueueTaskName").value = "";
  document.getElementById("createQueueTaskDescription").value = "";
  document.getElementById("createQueuePriority").value = "medium";
  document.getElementById("createQueueProjectId").value = "";
  document.getElementById("createQueueModal").classList.add("active");
}

function showCreateFeelingModal() {
  // Updated for error tracking
  document.getElementById("createFeelingEmotion").value = "medium"; // Changed from "happy" to "medium" for error severity
  document.getElementById("createFeelingDescription").value = ""; // This is now the error title
  document.getElementById("createFeelingContext").value = ""; // This is now the error description
  document.getElementById("createFeelingTags").value = ""; // This is now the error context
  document.getElementById("createFeelingModal").classList.add("active");
}

function showTaskDetail(task) {
  currentTaskId = task.id;
  document.getElementById("taskDetailName").value = task.action || "";
  document.getElementById("taskDetailTime").value = task.time || "";
  document.getElementById("taskDetailProject").value =
    task.parent_project || "";
  document.getElementById("taskDetailDescription").value =
    task.context_notes || "";
  document.getElementById("taskDetailPriority").value =
    task.priority || "medium";
  document.getElementById("taskDetailStatus").value = task.status || "todo";
  document.getElementById("taskDetailTags").value = task.tags || "";
  document.getElementById("taskDetailDependencies").value =
    task.dependencies || "";
  document.getElementById("taskDetailAssignee").value = task.assignee || "ai";
  document.getElementById("taskDetailProgress").value = task.progress || "";
  document.getElementById("taskDetailContext").value = task.context_notes || "";
  document.getElementById("taskDetailModal").classList.add("active");
}

function showMemoryDetail(memory) {
  currentMemoryId = memory.id;
  document.getElementById("memoryDetailMoment").value = memory.moment || "";
  document.getElementById("memoryDetailMeaning").value = memory.meaning || "";
  document.getElementById("memoryDetailReason").value = memory.reason || "";
  document.getElementById("memoryDetailImportance").value =
    memory.importance || "medium";
  document.getElementById("memoryDetailTerm").value = memory.term || "short";
  document.getElementById("memoryDetailType").value = memory.type || "standard";
  document.getElementById("memoryDetailTags").value = memory.tags || "";
  document.getElementById("memoryDetailModal").classList.add("active");
}

function showIdeaDetail(idea) {
  currentIdeaId = idea.id;
  document.getElementById("ideaDetailContent").value = idea.idea || "";
  document.getElementById("ideaDetailShare").value = idea.share || "private";
  document.getElementById("ideaDetailImportance").value =
    idea.importance || "medium";
  document.getElementById("ideaDetailTags").value = idea.tags || "";
  document.getElementById("ideaDetailContext").value = idea.context || "";
  document.getElementById("ideaDetailModal").classList.add("active");
}

function showAgentDetail(agent) {
  currentAgentId = agent.id;
  document.getElementById("agentDetailName").value = agent.name || "";
  document.getElementById("agentDetailDescription").value =
    agent.description || "";
  document.getElementById("agentDetailCategory").value =
    agent.category || "general";
  document.getElementById("agentDetailModelProvider").value =
    agent.model_provider || "todozi";
  document.getElementById("agentDetailModelName").value =
    agent.model_name || "";
  document.getElementById("agentDetailTemperature").value =
    agent.temperature || "0.2";
  document.getElementById("agentDetailMaxTokens").value =
    agent.max_tokens || "4096";
  document.getElementById("agentDetailCapabilities").value =
    agent.capabilities || "";
  document.getElementById("agentDetailSpecializations").value =
    agent.specializations || "";
  document.getElementById("agentDetailTags").value = agent.tags || "";
  document.getElementById("agentDetailModal").classList.add("active");
}

function showTrainingDetail(training) {
  currentTrainingId = training.id;
  document.getElementById("trainingDetailDataType").value =
    training.data_type || "instruction";
  document.getElementById("trainingDetailPrompt").value = training.prompt || "";
  document.getElementById("trainingDetailCompletion").value =
    training.completion || "";
  document.getElementById("trainingDetailContext").value =
    training.context || "";
  document.getElementById("trainingDetailTags").value = training.tags || "";
  document.getElementById("trainingDetailQuality").value =
    training.quality_score || "";
  document.getElementById("trainingDetailSource").value =
    training.source || "manual";
  document.getElementById("trainingDetailModal").classList.add("active");
}

function showQueueDetail(queueItem) {
  currentQueueId = queueItem.id;
  document.getElementById("queueDetailTaskName").value =
    queueItem.task_name || "";
  document.getElementById("queueDetailTaskDescription").value =
    queueItem.task_description || "";
  document.getElementById("queueDetailPriority").value =
    queueItem.priority || "medium";
  document.getElementById("queueDetailProjectId").value =
    queueItem.project_id || "";
  document.getElementById("queueDetailStatus").value =
    queueItem.status || "backlog";
  document.getElementById("queueDetailModal").classList.add("active");
}

function showFeelingDetail(feeling) {
  currentFeelingId = feeling.id;
  document.getElementById("feelingDetailEmotion").value =
    feeling.emotion || "happy";
  document.getElementById("feelingDetailIntensity").value =
    feeling.intensity || "5";
  document.getElementById("detailIntensityValue").textContent =
    feeling.intensity || "5";
  document.getElementById("feelingDetailDescription").value =
    feeling.description || "";
  document.getElementById("feelingDetailContext").value = feeling.context || "";
  document.getElementById("feelingDetailTags").value = feeling.tags || "";
  document.getElementById("feelingDetailModal").classList.add("active");
}

// Task Functions
async function loadTasks() {
  try {
    const response = await apiRequest("/tasks");
    currentTasks = response.tasks ? JSON.parse(response.tasks) : [];
    renderTasks(currentTasks);
    updateStats();
    document.getElementById("tasksCount").textContent = currentTasks.length;
  } catch (error) {
    console.error("Failed to load tasks:", error);
    showToast("Failed to load tasks");
    renderTasks([]);
  }
}

function renderTasks(tasks) {
  if (!taskList) return; // DOM not ready yet

  if (!tasks || tasks.length === 0) {
    taskList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">📋</div>
                <div class="empty-state-title">No tasks yet</div>
                <div class="empty-state-desc">Create your first task to get started</div>
                <button class="btn btn-primary" style="margin-top: 20px;" onclick="showCreateModal()">Create Your First Task</button>
            </div>
        `;
    return;
  }

  taskList.innerHTML = tasks
    .map(
      (task) => `
      <div class="task-item ${task.status === "completed" ? "completed" : ""}" onclick="showTaskDetail(${JSON.stringify(task).replace(/"/g, "&quot;")})">
          <div class="task-checkbox ${task.status === "completed" ? "checked" : ""}"
               onclick="event.stopPropagation(); toggleTaskStatus('${task.id}', '${task.status}')"></div>
          <div class="task-content">
              <div class="task-title">${task.action || "Untitled Task"}</div>
              <div class="task-meta">
                  <span class="task-tag ${getPriorityClass(task.priority)}">${task.priority || "medium"}</span>
                  <span class="task-tag ${getStatusClass(task.status)}">${task.status || "todo"}</span>
                  ${task.parent_project ? `<span class="task-tag tag-design">${task.parent_project}</span>` : ""}
                  ${task.assignee ? `<span class="task-tag tag-design">${task.assignee}</span>` : ""}
              </div>
          </div>
          <button class="task-expand-btn" onclick="event.stopPropagation(); showTaskDetail(${JSON.stringify(task).replace(/"/g, "&quot;")})">⋮</button>
      </div>
  `,
    )
    .join("");
}

function getPriorityClass(priority) {
  switch (priority) {
    case "urgent":
      return "tag-urgent";
    case "high":
      return "tag-important";
    case "medium":
      return "tag-design";
    default:
      return "tag-design";
  }
}

function getStatusClass(status) {
  switch (status) {
    case "completed":
      return "badge-success";
    case "in-progress":
      return "badge-warning";
    case "todo":
      return "badge-primary";
    default:
      return "badge-primary";
  }
}

async function createTask() {
  const name = document.getElementById("createTaskName").value.trim();
  if (!name) {
    showToast("Please enter a task name");
    return;
  }

  try {
    const task = {
      action: name,
      time: document.getElementById("createTaskTime").value,
      parent_project: document.getElementById("createTaskProject").value,
      context_notes: document.getElementById("createTaskDescription").value,
      priority: document.getElementById("createTaskPriority").value,
      status: document.getElementById("createTaskStatus").value,
      tags: document.getElementById("createTaskTags").value,
      dependencies: document.getElementById("createTaskDependencies").value,
      assignee: document.getElementById("createTaskAssignee").value,
      progress:
        parseInt(document.getElementById("createTaskProgress").value) || 0,
      project: currentProject !== "all" ? currentProject : undefined,
    };

    const response = await apiRequest("/tasks", {
      method: "POST",
      body: JSON.stringify(task),
    });

    showToast("Task created successfully");
    closeModal("createTaskModal");
    loadTasks();
  } catch (error) {
    console.error("Failed to create task:", error);
    showToast("Failed to create task");
  }
}

async function updateTask() {
  if (!currentTaskId) return;

  try {
    const task = {
      action: document.getElementById("taskDetailName").value,
      time: document.getElementById("taskDetailTime").value,
      parent_project: document.getElementById("taskDetailProject").value,
      context_notes: document.getElementById("taskDetailDescription").value,
      priority: document.getElementById("taskDetailPriority").value,
      status: document.getElementById("taskDetailStatus").value,
      tags: document.getElementById("taskDetailTags").value,
      dependencies: document.getElementById("taskDetailDependencies").value,
      assignee: document.getElementById("taskDetailAssignee").value,
      progress:
        parseInt(document.getElementById("taskDetailProgress").value) || 0,
    };

    const response = await apiRequest(`/tasks/${currentTaskId}`, {
      method: "PUT",
      body: JSON.stringify(task),
    });

    showToast("Task updated successfully");
    closeModal("taskDetailModal");
    loadTasks();
  } catch (error) {
    console.error("Failed to update task:", error);
    showToast("Failed to update task");
  }
}

async function deleteTask() {
  if (!currentTaskId) return;

  if (!confirm("Are you sure you want to delete this task?")) {
    return;
  }

  try {
    const response = await apiRequest(`/tasks/${currentTaskId}`, {
      method: "DELETE",
    });

    showToast("Task deleted successfully");
    closeModal("taskDetailModal");
    loadTasks();
  } catch (error) {
    console.error("Failed to delete task:", error);
    showToast("Failed to delete task");
  }
}

async function toggleTaskStatus(taskId, currentStatus) {
  try {
    const newStatus = currentStatus === "completed" ? "todo" : "completed";
    const response = await apiRequest(`/tasks/${taskId}`, {
      method: "PUT",
      body: JSON.stringify({ status: newStatus }),
    });

    loadTasks();
  } catch (error) {
    console.error("Failed to update task status:", error);
    showToast("Failed to update task status");
  }
}

// Memory Functions
async function loadMemories() {
  try {
    let url = "/memories";
    if (currentProject && currentProject !== "all") {
      url += `?project=${encodeURIComponent(currentProject)}`;
    }
    const response = await apiRequest(url);
    currentMemories = response.memories ? JSON.parse(response.memories) : [];
    renderMemories(currentMemories);
    document.getElementById("memoriesCount").textContent =
      currentMemories.length;
  } catch (error) {
    console.error("Failed to load memories:", error);
    showToast("Failed to load memories");
    renderMemories([]);
  }
}

function renderMemories(memories) {
  if (!memoryList) return; // DOM not ready yet

  if (!memories || memories.length === 0) {
    memoryList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">🧠</div>
                <div class="empty-state-title">No memories yet</div>
                <div class="empty-state-desc">Create your first memory to capture important moments</div>
            </div>
        `;
    return;
  }

  memoryList.innerHTML = memories
    .map(
      (memory) => `
      <div class="memory-card" onclick="showMemoryDetail(${JSON.stringify(memory).replace(/"/g, "&quot;")})">
          <div class="memory-header">
              <div class="memory-title">${memory.moment || "Untitled Memory"}</div>
              <div class="memory-type ${getMemoryTypeClass(memory.type)}">${memory.type || "standard"}</div>
          </div>
          <div class="memory-content">
              ${memory.meaning ? `<div><strong>Meaning:</strong> ${memory.meaning}</div>` : ""}
              ${memory.reason ? `<div><strong>Reason:</strong> ${memory.reason}</div>` : ""}
          </div>
          <div class="memory-meta">
              <span class="badge ${getMemoryTypeClass(memory.importance)}">${memory.importance || "medium"}</span>
              <span class="badge ${getMemoryTypeClass(memory.term)}">${memory.term || "short"}</span>
              ${memory.tags ? `<span class="badge badge-primary">${memory.tags}</span>` : ""}
          </div>
      </div>
  `,
    )
    .join("");
}

function getMemoryTypeClass(type) {
  const classes = {
    secret: "badge-danger",
    human: "badge-success",
    short: "badge-warning",
    long: "badge-primary",
    emotional: "badge-danger",
  };
  return classes[type] || "badge-primary";
}

async function createMemory() {
  const moment = document.getElementById("createMemoryMoment").value.trim();
  if (!moment) {
    showToast("Please enter a moment");
    return;
  }

  try {
    const memory = {
      moment: moment,
      meaning: document.getElementById("createMemoryMeaning").value,
      reason: document.getElementById("createMemoryReason").value,
      importance: document.getElementById("createMemoryImportance").value,
      term: document.getElementById("createMemoryTerm").value,
      type: document.getElementById("createMemoryType").value,
      tags: document.getElementById("createMemoryTags").value,
      project: currentProject !== "all" ? currentProject : undefined,
    };

    const response = await apiRequest("/memories", {
      method: "POST",
      body: JSON.stringify(memory),
    });

    showToast("Memory created successfully");
    closeModal("createMemoryModal");
    loadMemories();
  } catch (error) {
    console.error("Failed to create memory:", error);
    showToast("Failed to create memory");
  }
}

async function updateMemory() {
  if (!currentMemoryId) return;

  try {
    const memory = {
      moment: document.getElementById("memoryDetailMoment").value,
      meaning: document.getElementById("memoryDetailMeaning").value,
      reason: document.getElementById("memoryDetailReason").value,
      importance: document.getElementById("memoryDetailImportance").value,
      term: document.getElementById("memoryDetailTerm").value,
      type: document.getElementById("memoryDetailType").value,
      tags: document.getElementById("memoryDetailTags").value,
    };

    const response = await apiRequest(`/memories/${currentMemoryId}`, {
      method: "PUT",
      body: JSON.stringify(memory),
    });

    showToast("Memory updated successfully");
    closeModal("memoryDetailModal");
    loadMemories();
  } catch (error) {
    console.error("Failed to update memory:", error);
    showToast("Failed to update memory");
  }
}

async function deleteMemory() {
  if (!currentMemoryId) return;

  if (!confirm("Are you sure you want to delete this memory?")) {
    return;
  }

  try {
    const response = await apiRequest(`/memories/${currentMemoryId}`, {
      method: "DELETE",
    });

    showToast("Memory deleted successfully");
    closeModal("memoryDetailModal");
    loadMemories();
  } catch (error) {
    console.error("Failed to delete memory:", error);
    showToast("Failed to delete memory");
  }
}

// Idea Functions
async function loadIdeas() {
  try {
    let url = "/ideas";
    if (currentProject && currentProject !== "all") {
      url += `?project=${encodeURIComponent(currentProject)}`;
    }
    const response = await apiRequest(url);
    currentIdeas = response.ideas ? JSON.parse(response.ideas) : [];
    renderIdeas(currentIdeas);
    document.getElementById("ideasCount").textContent = currentIdeas.length;
  } catch (error) {
    console.error("Failed to load ideas:", error);
    showToast("Failed to load ideas");
    renderIdeas([]);
  }
}

function renderIdeas(ideas) {
  if (!ideaList) return; // DOM not ready yet

  if (!ideas || ideas.length === 0) {
    ideaList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">💡</div>
                <div class="empty-state-title">No ideas yet</div>
                <div class="empty-state-desc">Capture your thoughts and ideas here</div>
            </div>
        `;
    return;
  }

  ideaList.innerHTML = ideas
    .map(
      (idea) => `
      <div class="idea-card" onclick="showIdeaDetail(${JSON.stringify(idea).replace(/"/g, "&quot;")})">
          <div class="idea-header">
              <div class="idea-title">Idea</div>
              <div class="idea-share ${getShareClass(idea.share)}">${idea.share || "private"}</div>
          </div>
          <div class="idea-content">
              ${idea.idea || "No content"}
          </div>
          <div class="idea-meta">
              <span class="badge ${getShareClass(idea.importance)}">${idea.importance || "medium"}</span>
              ${idea.tags ? `<span class="badge badge-primary">${idea.tags}</span>` : ""}
          </div>
      </div>
  `,
    )
    .join("");
}

function getShareClass(share) {
  const classes = {
    private: "badge-danger",
    team: "badge-warning",
    public: "badge-success",
  };
  return classes[share] || "badge-primary";
}

async function createIdea() {
  const ideaContent = document.getElementById("createIdeaContent").value.trim();
  if (!ideaContent) {
    showToast("Please enter an idea");
    return;
  }

  try {
    const idea = {
      idea: ideaContent,
      share: document.getElementById("createIdeaShare").value,
      importance: document.getElementById("createIdeaImportance").value,
      tags: document.getElementById("createIdeaTags").value,
      context: document.getElementById("createIdeaContext").value,
      project: currentProject !== "all" ? currentProject : undefined,
    };

    const response = await apiRequest("/ideas", {
      method: "POST",
      body: JSON.stringify(idea),
    });

    showToast("Idea created successfully");
    closeModal("createIdeaModal");
    loadIdeas();
  } catch (error) {
    console.error("Failed to create idea:", error);
    showToast("Failed to create idea");
  }
}

async function updateIdea() {
  if (!currentIdeaId) return;

  try {
    const idea = {
      idea: document.getElementById("ideaDetailContent").value,
      share: document.getElementById("ideaDetailShare").value,
      importance: document.getElementById("ideaDetailImportance").value,
      tags: document.getElementById("ideaDetailTags").value,
      context: document.getElementById("ideaDetailContext").value,
    };

    const response = await apiRequest(`/ideas/${currentIdeaId}`, {
      method: "PUT",
      body: JSON.stringify(idea),
    });

    showToast("Idea updated successfully");
    closeModal("ideaDetailModal");
    loadIdeas();
  } catch (error) {
    console.error("Failed to update idea:", error);
    showToast("Failed to update idea");
  }
}

async function deleteIdea() {
  if (!currentIdeaId) return;

  if (!confirm("Are you sure you want to delete this idea?")) {
    return;
  }

  try {
    const response = await apiRequest(`/ideas/${currentIdeaId}`, {
      method: "DELETE",
    });

    showToast("Idea deleted successfully");
    closeModal("ideaDetailModal");
    loadIdeas();
  } catch (error) {
    console.error("Failed to delete idea:", error);
    showToast("Failed to delete idea");
  }
}

// Agent Functions
async function loadAgents() {
  try {
    const response = await apiRequest("/agents");
    currentAgents = response.agents ? JSON.parse(response.agents) : [];
    renderAgents(currentAgents);
    document.getElementById("agentsCount").textContent = currentAgents.length;
  } catch (error) {
    console.error("Failed to load agents:", error);
    showToast("Failed to load agents");
    renderAgents([]);
  }
}

function renderAgents(agents) {
  if (!agentList) return; // DOM not ready yet

  if (!agents || agents.length === 0) {
    agentList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">🤖</div>
                <div class="empty-state-title">No agents yet</div>
                <div class="empty-state-desc">Create your first AI agent to assist with tasks</div>
            </div>
        `;
    return;
  }

  agentList.innerHTML = agents
    .map(
      (agent) => `
      <div class="agent-card" onclick="showAgentDetail(${JSON.stringify(agent).replace(/"/g, "&quot;")})">
          <div class="agent-header">
              <div class="agent-icon">${agent.name ? agent.name.charAt(0) : "A"}</div>
              <div class="agent-name">${agent.name || "Unnamed Agent"}</div>
          </div>
          <div class="agent-description">
              ${agent.description || "No description"}
          </div>
          <div class="agent-meta">
              <div class="agent-meta-item">
                  <span class="badge badge-primary">${agent.category || "general"}</span>
              </div>
              <div class="agent-meta-item">
                  <span class="badge badge-success">${agent.model_provider || "todozi"}:${agent.model_name || "baton"}</span>
              </div>
          </div>
      </div>
  `,
    )
    .join("");
}

async function createAgent() {
  const agentId = document.getElementById("createAgentId").value.trim();
  const agentName = document.getElementById("createAgentName").value.trim();
  if (!agentId || !agentName) {
    showToast("Please enter agent ID and name");
    return;
  }

  try {
    const agent = {
      id: agentId,
      name: agentName,
      description: document.getElementById("createAgentDescription").value,
      category: document.getElementById("createAgentCategory").value,
      model_provider: document.getElementById("createAgentModelProvider").value,
      model_name: document.getElementById("createAgentModelName").value,
      temperature: parseFloat(
        document.getElementById("createAgentTemperature").value,
      ),
      max_tokens: parseInt(
        document.getElementById("createAgentMaxTokens").value,
      ),
      capabilities: document.getElementById("createAgentCapabilities").value,
      specializations: document.getElementById("createAgentSpecializations")
        .value,
      tags: document.getElementById("createAgentTags").value,
    };

    const response = await apiRequest("/agents", {
      method: "POST",
      body: JSON.stringify(agent),
    });

    showToast("Agent created successfully");
    closeModal("createAgentModal");
    loadAgents();
  } catch (error) {
    console.error("Failed to create agent:", error);
    showToast("Failed to create agent");
  }
}

async function updateAgent() {
  if (!currentAgentId) return;

  try {
    const agent = {
      name: document.getElementById("agentDetailName").value,
      description: document.getElementById("agentDetailDescription").value,
      category: document.getElementById("agentDetailCategory").value,
      model_provider: document.getElementById("agentDetailModelProvider").value,
      model_name: document.getElementById("agentDetailModelName").value,
      temperature: parseFloat(
        document.getElementById("agentDetailTemperature").value,
      ),
      max_tokens: parseInt(
        document.getElementById("agentDetailMaxTokens").value,
      ),
      capabilities: document.getElementById("agentDetailCapabilities").value,
      specializations: document.getElementById("agentDetailSpecializations")
        .value,
      tags: document.getElementById("agentDetailTags").value,
    };

    const response = await apiRequest(`/agents/${currentAgentId}`, {
      method: "PUT",
      body: JSON.stringify(agent),
    });

    showToast("Agent updated successfully");
    closeModal("agentDetailModal");
    loadAgents();
  } catch (error) {
    console.error("Failed to update agent:", error);
    showToast("Failed to update agent");
  }
}

async function deleteAgent() {
  if (!currentAgentId) return;

  if (!confirm("Are you sure you want to delete this agent?")) {
    return;
  }

  try {
    const response = await apiRequest(`/agents/${currentAgentId}`, {
      method: "DELETE",
    });

    showToast("Agent deleted successfully");
    closeModal("agentDetailModal");
    loadAgents();
  } catch (error) {
    console.error("Failed to delete agent:", error);
    showToast("Failed to delete agent");
  }
}

// Training Data Functions
async function loadTrainingData() {
  try {
    const response = await apiRequest("/training");
    currentTrainingData = response.training_data
      ? JSON.parse(response.training_data)
      : [];
    renderTrainingData(currentTrainingData);
    document.getElementById("trainingCount").textContent =
      currentTrainingData.length;
  } catch (error) {
    console.error("Failed to load training data:", error);
    showToast("Failed to load training data");
    renderTrainingData([]);
  }
}

function renderTrainingData(trainingData) {
  if (!trainingList) return; // DOM not ready yet

  if (!trainingData || trainingData.length === 0) {
    trainingList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">🎓</div>
                <div class="empty-state-title">No training data yet</div>
                <div class="empty-state-desc">Create training data to improve AI models</div>
            </div>
        `;
    return;
  }

  trainingList.innerHTML = trainingData
    .map(
      (data) => `
      <div class="training-card" onclick="showTrainingDetail(${JSON.stringify(data).replace(/"/g, "&quot;")})">
          <div class="training-header">
              <div class="training-title">${data.data_type || "instruction"}</div>
              <div class="training-type ${getTrainingTypeClass(data.data_type)}">${data.data_type || "instruction"}</div>
          </div>
          <div class="training-content">
              ${data.prompt ? `<div><strong>Prompt:</strong> ${data.prompt.substring(0, 100)}${data.prompt.length > 100 ? "..." : ""}</div>` : ""}
              ${data.completion ? `<div><strong>Completion:</strong> ${data.completion.substring(0, 100)}${data.completion.length > 100 ? "..." : ""}</div>` : ""}
          </div>
          <div class="training-meta">
              ${data.quality_score ? `<span class="badge ${data.quality_score > 0.8 ? "badge-success" : data.quality_score > 0.5 ? "badge-warning" : "badge-danger"}">Quality: ${data.quality_score}</span>` : ""}
              ${data.source ? `<span class="badge badge-primary">${data.source}</span>` : ""}
              ${data.tags ? `<span class="badge badge-primary">${data.tags}</span>` : ""}
          </div>
      </div>
  `,
    )
    .join("");
}

function getTrainingTypeClass(type) {
  const classes = {
    instruction: "badge-primary",
    completion: "badge-success",
    conversation: "badge-warning",
    code: "badge-danger",
    analysis: "badge-primary",
    planning: "badge-success",
    review: "badge-warning",
    documentation: "badge-danger",
    example: "badge-primary",
    test: "badge-success",
    validation: "badge-warning",
  };
  return classes[type] || "badge-primary";
}

async function createTrainingData() {
  const prompt = document.getElementById("createTrainingPrompt").value.trim();
  const completion = document
    .getElementById("createTrainingCompletion")
    .value.trim();
  if (!prompt || !completion) {
    showToast("Please enter both prompt and completion");
    return;
  }

  try {
    const trainingData = {
      data_type: document.getElementById("createTrainingDataType").value,
      prompt: prompt,
      completion: completion,
      context: document.getElementById("createTrainingContext").value,
      tags: document.getElementById("createTrainingTags").value,
      quality_score:
        parseFloat(document.getElementById("createTrainingQuality").value) ||
        null,
      source: document.getElementById("createTrainingSource").value,
    };

    const response = await apiRequest("/training", {
      method: "POST",
      body: JSON.stringify(trainingData),
    });

    showToast("Training data created successfully");
    closeModal("createTrainingModal");
    loadTrainingData();
  } catch (error) {
    console.error("Failed to create training data:", error);
    showToast("Failed to create training data");
  }
}

async function updateTrainingData() {
  if (!currentTrainingId) return;

  try {
    const trainingData = {
      data_type: document.getElementById("trainingDetailDataType").value,
      prompt: document.getElementById("trainingDetailPrompt").value,
      completion: document.getElementById("trainingDetailCompletion").value,
      context: document.getElementById("trainingDetailContext").value,
      tags: document.getElementById("trainingDetailTags").value,
      quality_score:
        parseFloat(document.getElementById("trainingDetailQuality").value) ||
        null,
      source: document.getElementById("trainingDetailSource").value,
    };

    const response = await apiRequest(`/training/${currentTrainingId}`, {
      method: "PUT",
      body: JSON.stringify(trainingData),
    });

    showToast("Training data updated successfully");
    closeModal("trainingDetailModal");
    loadTrainingData();
  } catch (error) {
    console.error("Failed to update training data:", error);
    showToast("Failed to update training data");
  }
}

async function deleteTrainingData() {
  if (!currentTrainingId) return;

  if (!confirm("Are you sure you want to delete this training data?")) {
    return;
  }

  try {
    const response = await apiRequest(`/training/${currentTrainingId}`, {
      method: "DELETE",
    });

    showToast("Training data deleted successfully");
    closeModal("trainingDetailModal");
    loadTrainingData();
  } catch (error) {
    console.error("Failed to delete training data:", error);
    showToast("Failed to delete training data");
  }
}

// Queue Functions
async function loadQueueItems() {
  try {
    let endpoint = "/queue/list";
    if (currentQueueTab === "backlog") endpoint = "/queue/list/backlog";
    if (currentQueueTab === "active") endpoint = "/queue/list/active";
    if (currentQueueTab === "complete") endpoint = "/queue/list/complete";

    // Add project filter
    if (currentProject && currentProject !== "all") {
      endpoint +=
        (endpoint.includes("?") ? "&" : "?") +
        `project=${encodeURIComponent(currentProject)}`;
    }

    const response = await apiRequest(endpoint);
    currentQueueItems = response.queue ? JSON.parse(response.queue) : [];
    renderQueueItems(currentQueueItems);
    document.getElementById("queueCount").textContent =
      currentQueueItems.length;
  } catch (error) {
    console.error("Failed to load queue items:", error);
    showToast("Failed to load queue items");
    renderQueueItems([]);
  }
}

function renderQueueItems(queueItems) {
  if (!queueList) return; // DOM not ready yet

  if (!queueItems || queueItems.length === 0) {
    queueList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">📋</div>
                <div class="empty-state-title">No queue items yet</div>
                <div class="empty-state-desc">Create your first queue item to organize tasks</div>
            </div>
        `;
    return;
  }

  queueList.innerHTML = queueItems
    .map(
      (item) => `
      <div class="queue-card" onclick="showQueueDetail(${JSON.stringify(item).replace(/"/g, "&quot;")})">
          <div class="queue-header">
              <div class="queue-title">${item.task_name || "Untitled Task"}</div>
              <div class="queue-status ${getQueueStatusClass(item.status)}">${item.status || "backlog"}</div>
          </div>
          <div class="queue-description">
              ${item.task_description || "No description"}
          </div>
          <div class="queue-meta">
              <span class="badge ${getQueueStatusClass(item.priority)}">${item.priority || "medium"}</span>
              ${item.project_id ? `<span class="badge badge-primary">${item.project_id}</span>` : ""}
          </div>
      </div>
  `,
    )
    .join("");
}

function getQueueStatusClass(status) {
  const classes = {
    backlog: "badge-primary",
    active: "badge-warning",
    complete: "badge-success",
  };
  return classes[status] || "badge-primary";
}

async function createQueueItem() {
  const taskName = document.getElementById("createQueueTaskName").value.trim();
  if (!taskName) {
    showToast("Please enter a task name");
    return;
  }

  try {
    const queueItem = {
      task_name: taskName,
      task_description: document.getElementById("createQueueTaskDescription")
        .value,
      priority: document.getElementById("createQueuePriority").value,
      project_id: document.getElementById("createQueueProjectId").value || null,
      project: currentProject !== "all" ? currentProject : undefined,
    };

    const response = await apiRequest("/queue/plan", {
      method: "POST",
      body: JSON.stringify(queueItem),
    });

    showToast("Queue item created successfully");
    closeModal("createQueueModal");
    loadQueueItems();
  } catch (error) {
    console.error("Failed to create queue item:", error);
    showToast("Failed to create queue item");
  }
}

async function updateQueueItem() {
  if (!currentQueueId) return;

  try {
    const queueItem = {
      task_name: document.getElementById("queueDetailTaskName").value,
      task_description: document.getElementById("queueDetailTaskDescription")
        .value,
      priority: document.getElementById("queueDetailPriority").value,
      project_id: document.getElementById("queueDetailProjectId").value || null,
      status: document.getElementById("queueDetailStatus").value,
    };

    const response = await apiRequest(`/queue/list`, {
      method: "PUT",
      body: JSON.stringify(queueItem),
    });

    showToast("Queue item updated successfully");
    closeModal("queueDetailModal");
    loadQueueItems();
  } catch (error) {
    console.error("Failed to update queue item:", error);
    showToast("Failed to update queue item");
  }
}

async function deleteQueueItem() {
  if (!currentQueueId) return;

  if (!confirm("Are you sure you want to delete this queue item?")) {
    return;
  }

  try {
    const response = await apiRequest(`/queue/list`, {
      method: "DELETE",
    });

    showToast("Queue item deleted successfully");
    closeModal("queueDetailModal");
    loadQueueItems();
  } catch (error) {
    console.error("Failed to delete queue item:", error);
    showToast("Failed to delete queue item");
  }
}

// Error Tracking Functions (replaces Feeling Functions)
async function loadErrors() {
  // Changed from loadFeelings to loadErrors
  try {
    let url = "/errors"; // Changed from /feelings to /errors
    if (currentProject && currentProject !== "all") {
      url += `?project=${encodeURIComponent(currentProject)}`;
    }
    const response = await apiRequest(url);
    currentFeelings = response.errors // Changed from response.feelings to response.errors
      ? JSON.parse(response.errors) // Changed from response.feelings to response.errors
      : [];
    renderFeelings(currentFeelings); // Keep the same function name for UI consistency
    document.getElementById("feelingsCount").textContent = // Keep the same element ID for UI consistency
      currentFeelings.length;
  } catch (error) {
    console.error("Failed to load errors:", error); // Updated error message
    showToast("Failed to load errors"); // Updated toast message
    renderFeelings([]);
  }
}

function renderFeelings(feelings) {
  // Keep the same function name for UI consistency
  if (!feelingList) return; // DOM not ready yet

  if (!feelings || feelings.length === 0) {
    feelingList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">⚠️</div>
                <div class="empty-state-title">No errors tracked yet</div>
                <div class="empty-state-desc">Track system errors and issues here</div>
            </div>
        `;
    return;
  }

  feelingList.innerHTML = feelings
    .map(
      (feeling) => `  // Keep the same variable name for UI consistency
      <div class="feeling-card" onclick="showFeelingDetail(${JSON.stringify(feeling).replace(/"/g, "&quot;")})">
          <div class="feeling-header">
              <div class="feeling-title">${feeling.title || feeling.emotion || "Error"}  // Updated to show error title
              </div>
              <div class="feeling-emotion ${getFeelingClass(feeling.severity || feeling.emotion)}">
                  ${feeling.severity || feeling.emotion || "unknown"}
              </div>
          </div>
          <div class="feeling-content">
              ${feeling.description || "No description"}
          </div>
          <div class="feeling-meta">
              ${feeling.tags ? `<span class="badge badge-primary">${feeling.tags}</span>` : ""}
              ${feeling.context ? `<span class="badge badge-warning">${feeling.context}</span>` : ""}
          </div>
      </div>
  `,
    )
    .join("");
}

function getFeelingClass(emotion) {
  // Keep the same function name for UI consistency
  const classes = {
    happy: "badge-success",
    sad: "badge-primary",
    angry: "badge-danger",
    fearful: "badge-warning",
    surprised: "badge-warning",
    disgusted: "badge-danger",
    excited: "badge-success",
    anxious: "badge-warning",
    confident: "badge-success",
    frustrated: "badge-danger",
    motivated: "badge-success",
    overwhelmed: "badge-warning",
    curious: "badge-primary",
    satisfied: "badge-success",
    disappointed: "badge-primary",
    grateful: "badge-success",
    proud: "badge-success",
    ashamed: "badge-danger",
    hopeful: "badge-success",
    resigned: "badge-primary",
    // Add error severity classes
    low: "badge-success",
    medium: "badge-warning",
    high: "badge-danger",
    critical: "badge-danger",
  };
  return classes[emotion] || "badge-primary";
}

async function createFeeling() {
  // Keep the same function name for UI consistency
  const title = document // Changed from description to title
    .getElementById("createFeelingDescription") // Keep the same element ID for UI consistency
    .value.trim();
  if (!title) {
    // Changed validation to check title
    showToast("Please enter an error title"); // Updated toast message
    return;
  }

  try {
    const feeling = {
      // Keep the same variable name for UI consistency
      title: title, // Changed from description to title
      description: document.getElementById("createFeelingContext").value, // Swapped description and context fields
      source: "manual", // Added required source field
      severity: document.getElementById("createFeelingEmotion").value, // Keep the same element ID for UI consistency
      category: "runtime", // Added required category field
      context: document.getElementById("createFeelingTags").value, // Swapped description and context fields
      project: currentProject !== "all" ? currentProject : undefined,
    };

    const response = await apiRequest("/errors", {
      // Changed from /feelings to /errors
      method: "POST",
      body: JSON.stringify(feeling), // Keep the same variable name for UI consistency
    });

    showToast("Error tracked successfully"); // Updated toast message
    closeModal("createFeelingModal"); // Keep the same modal ID for UI consistency
    loadErrors(); // Changed from loadFeelings to loadErrors
  } catch (error) {
    console.error("Failed to track error:", error); // Updated error message
    showToast("Failed to track error"); // Updated toast message
  }
}

async function updateFeeling() {
  // Keep the same function name for UI consistency
  if (!currentFeelingId) return; // Keep the same variable name for UI consistency

  try {
    const feeling = {
      // Keep the same variable name for UI consistency
      title: document.getElementById("feelingDetailDescription").value, // Changed from description to title
      description: document.getElementById(
        "feelingDetailContext", // Swapped description and context fields
      ).value,
      severity: document.getElementById("feelingDetailEmotion").value, // Keep the same element ID for UI consistency
      context: document.getElementById("feelingDetailTags").value, // Swapped description and context fields
    };

    const response = await apiRequest(
      `/errors/${currentFeelingId}`, // Changed from /feelings to /errors
      {
        method: "PUT",
        body: JSON.stringify(feeling), // Keep the same variable name for UI consistency
      },
    );

    showToast("Error updated successfully"); // Updated toast message
    closeModal("feelingDetailModal"); // Keep the same modal ID for UI consistency
    loadErrors(); // Changed from loadFeelings to loadErrors
  } catch (error) {
    console.error("Failed to update error:", error); // Updated error message
    showToast("Failed to update error"); // Updated toast message
  }
}

async function deleteFeeling() {
  // Keep the same function name for UI consistency
  if (!currentFeelingId) return; // Keep the same variable name for UI consistency

  if (!confirm("Are you sure you want to delete this error record?")) {
    // Updated confirmation message
    return;
  }

  try {
    const response = await apiRequest(
      `/errors/${currentFeelingId}`, // Changed from /feelings to /errors
      {
        method: "DELETE",
      },
    );

    showToast("Error record deleted successfully"); // Updated toast message
    closeModal("feelingDetailModal"); // Keep the same modal ID for UI consistency
    loadErrors(); // Changed from loadFeelings to loadErrors
  } catch (error) {
    console.error("Failed to delete error record:", error); // Updated error message
    showToast("Failed to delete error record"); // Updated toast message
  }
}

// Chat Functions
async function loadAgentsForChat() {
  try {
    const response = await apiRequest("/agents");
    const agents = response.agents ? JSON.parse(response.agents) : [];
    agentSelect.innerHTML = '<option value="">Select an agent</option>';
    agents.forEach((agent) => {
      const option = document.createElement("option");
      option.value = agent.id;
      option.textContent = agent.name;
      agentSelect.appendChild(option);
    });
  } catch (error) {
    console.error("Failed to load agents for chat:", error);
  }
}

async function sendMessage() {
  const message = chatInput.value.trim();
  if (!message) return;

  // Add user message to chat
  addMessageToChat("user", message);
  chatInput.value = "";

  try {
    const agentId = agentSelect.value;
    let response;

    if (agentId) {
      // Chat with specific agent
      response = await apiRequest(`/chat/agent/${agentId}`, {
        method: "POST",
        body: JSON.stringify({ message: message }),
      });
    } else {
      // Process chat message
      response = await apiRequest("/chat/process", {
        method: "POST",
        body: JSON.stringify({ message: message }),
      });
    }

    // Add agent response to chat
    addMessageToChat("agent", response.result || "Response received");
  } catch (error) {
    console.error("Failed to send message:", error);
    addMessageToChat(
      "agent",
      "Sorry, I encountered an error processing your message.",
    );
  }
}

function addMessageToChat(sender, message) {
  const messageDiv = document.createElement("div");
  messageDiv.className = `chat-message chat-${sender}`;
  messageDiv.innerHTML = `<div>${message}</div>`;
  chatMessages.appendChild(messageDiv);
  chatMessages.scrollTop = chatMessages.scrollHeight;
}

// API Key Functions
async function loadApiKeys() {
  try {
    const response = await apiRequest("/api/list");
    const apiKeys = response.keys ? response.keys : [];
    renderApiKeys(apiKeys);
  } catch (error) {
    console.error("Failed to load API keys:", error);
    showToast("Failed to load API keys");
    renderApiKeys([]);
  }
}

function renderApiKeys(apiKeys) {
  if (!apiKeyList) return; // DOM not ready yet

  if (!apiKeys || apiKeys.length === 0) {
    apiKeyList.innerHTML = `
            <div class="empty-state">
                <div class="empty-state-icon">🔑</div>
                <div class="empty-state-title">No API keys yet</div>
                <div class="empty-state-desc">Generate API keys to access Todozi programmatically</div>
            </div>
        `;
    return;
  }

  apiKeyList.innerHTML = apiKeys
    .map(
      (key) => `
      <div class="api-key-card">
          <div class="api-key-header">
              <div class="api-key-title">API Key</div>
              <div class="api-key-status ${key.active ? "badge-success" : "badge-danger"}">
                  ${key.active ? "Active" : "Inactive"}
              </div>
          </div>
          <div class="api-key-content">
              <div><strong>Public:</strong> ${key.public_key}</div>
              <div><strong>Created:</strong> ${new Date(key.created_at).toLocaleDateString()}</div>
          </div>
          <div class="api-key-meta">
              ${key.user_id ? `<span class="badge badge-primary">User: ${key.user_id}</span>` : ""}
          </div>
      </div>
  `,
    )
    .join("");
}

async function createApiKey() {
  try {
    const response = await apiRequest("/api/register", {
      method: "POST",
    });

    showToast("API key created successfully");
    loadApiKeys();
  } catch (error) {
    console.error("Failed to create API key:", error);
    showToast("Failed to create API key");
  }
}

// Stats Functions
function updateStats() {
  const total = currentTasks.length;
  const completed = currentTasks.filter(
    (task) => task.status === "completed",
  ).length;
  const inProgress = currentTasks.filter(
    (task) => task.status === "in-progress",
  ).length;

  document.getElementById("totalTasks").textContent = total;
  document.getElementById("completedTasks").textContent = completed;
  document.getElementById("inProgressTasks").textContent = inProgress;
}

// Search Functionality
document.getElementById("searchInput").addEventListener("input", function (e) {
  const searchTerm = e.target.value.toLowerCase();
  if (searchTerm.length > 2) {
    // In a real implementation, you would call the search API
    // For now, we'll just filter the current data
    switch (currentView) {
      case "tasks":
        const filteredTasks = currentTasks.filter(
          (task) =>
            (task.action && task.action.toLowerCase().includes(searchTerm)) ||
            (task.description &&
              task.description.toLowerCase().includes(searchTerm)) ||
            (task.tags && task.tags.toLowerCase().includes(searchTerm)),
        );
        renderTasks(filteredTasks);
        break;
      case "memories":
        const filteredMemories = currentMemories.filter(
          (memory) =>
            (memory.moment &&
              memory.moment.toLowerCase().includes(searchTerm)) ||
            (memory.meaning &&
              memory.meaning.toLowerCase().includes(searchTerm)) ||
            (memory.tags && memory.tags.toLowerCase().includes(searchTerm)),
        );
        renderMemories(filteredMemories);
        break;
      case "ideas":
        const filteredIdeas = currentIdeas.filter(
          (idea) =>
            (idea.idea && idea.idea.toLowerCase().includes(searchTerm)) ||
            (idea.tags && idea.tags.toLowerCase().includes(searchTerm)),
        );
        renderIdeas(filteredIdeas);
        break;
    }
  } else if (searchTerm.length === 0) {
    switch (currentView) {
      case "tasks":
        renderTasks(currentTasks);
        break;
      case "memories":
        renderMemories(currentMemories);
        break;
      case "ideas":
        renderIdeas(currentIdeas);
        break;
    }
  }
});

// Initialize
document.addEventListener("DOMContentLoaded", function () {
  // Assign DOM elements
  taskList = document.getElementById("taskList");
  memoryList = document.getElementById("memoryList");
  ideaList = document.getElementById("ideaList");
  agentList = document.getElementById("agentList");
  trainingList = document.getElementById("trainingList");
  queueList = document.getElementById("queueList");
  feelingList = document.getElementById("feelingList");
  apiKeyList = document.getElementById("apiKeyList");
  chatMessages = document.getElementById("chatMessages");
  chatInput = document.getElementById("chatInput");
  agentSelect = document.getElementById("agentSelect");
  projectSelect = document.getElementById("projectSelect");
  toast = document.getElementById("toast");

  loadApiToken();
  loadProjects();
  loadTasks();

  // Initialize intensity sliders
  const createIntensitySlider = document.getElementById(
    "createFeelingIntensity",
  );
  const detailIntensitySlider = document.getElementById(
    "feelingDetailIntensity",
  );

  if (createIntensitySlider) {
    createIntensitySlider.addEventListener("input", function () {
      document.getElementById("intensityValue").textContent = this.value;
    });
  }

  if (detailIntensitySlider) {
    detailIntensitySlider.addEventListener("input", function () {
      document.getElementById("detailIntensityValue").textContent = this.value;
    });
  }

  // Chat input handler
  if (chatInput) {
    chatInput.addEventListener("keypress", function (e) {
      if (e.key === "Enter") {
        sendMessage();
      }
    });
  }
});

// Project Functions
async function loadProjects() {
  try {
    const response = await apiRequest("/projects");
    currentProjects = response.projects || [];
    renderProjectSelector();
  } catch (error) {
    console.error("Failed to load projects:", error);
    showToast("Failed to load projects");
    // Set some default projects for UI testing
    currentProjects = [
      {
        name: "Personal",
        description: "Personal tasks and ideas",
        status: "active",
      },
      { name: "Work", description: "Work-related projects", status: "active" },
      {
        name: "Learning",
        description: "Learning and development",
        status: "active",
      },
    ];
    renderProjectSelector();
  }
}

function renderProjectSelector() {
  if (!projectSelect) return;

  // Clear existing options except "All Projects" and "Add New"
  projectSelect.innerHTML = `
         <option value="all">All Projects</option>
         <option value="add-new">+ Add New Project</option>
     `;

  // Add project options
  currentProjects.forEach((project) => {
    const option = document.createElement("option");
    option.value = project.name || project;
    option.textContent = project.name || project;
    if (currentProject === (project.name || project)) {
      option.selected = true;
    }
    projectSelect.appendChild(option);
  });
}

function changeProject() {
  if (!projectSelect) return;

  const selectedValue = projectSelect.value;

  if (selectedValue === "add-new") {
    // Reset dropdown and show create project modal
    projectSelect.value = currentProject;
    showCreateProjectModal();
  } else {
    currentProject = selectedValue;
    // Reload current view with new project filter
    switch (currentView) {
      case "tasks":
        loadTasks();
        break;
      case "memories":
        loadMemories();
        break;
      case "ideas":
        loadIdeas();
        break;
      case "queue":
        loadQueueItems();
        break;
      case "feelings":
        loadErrors(); // Changed from loadFeelings to loadErrors
        break;
      // Other views don't need project filtering yet
    }
  }
}

function showCreateProjectModal() {
  document.getElementById("createProjectName").value = "";
  document.getElementById("createProjectDescription").value = "";
  document.getElementById("createProjectPriority").value = "medium";
  document.getElementById("createProjectStatus").value = "active";
  document.getElementById("createProjectColor").value = "blue";
  document.getElementById("createProjectModal").classList.add("active");
}

async function createProject() {
  const name = document.getElementById("createProjectName").value.trim();
  if (!name) {
    showToast("Please enter a project name");
    return;
  }

  try {
    const project = {
      name: name,
      description: document.getElementById("createProjectDescription").value,
      priority: document.getElementById("createProjectPriority").value,
      status: document.getElementById("createProjectStatus").value,
      color: document.getElementById("createProjectColor").value,
    };

    const response = await apiRequest("/projects", {
      method: "POST",
      body: JSON.stringify(project),
    });

    showToast("Project created successfully");
    closeModal("createProjectModal");
    loadProjects();
  } catch (error) {
    console.error("Failed to create project:", error);
    showToast("Failed to create project");
  }
}

// Keyboard shortcuts
document.addEventListener("keydown", function (e) {
  // Cmd/Ctrl + N for new task
  if (e.key === "n" && (e.metaKey || e.ctrlKey)) {
    e.preventDefault();
    if (currentView === "tasks") {
      showCreateModal();
    }
  }

  // Escape to close modals
  if (e.key === "Escape") {
    document.querySelectorAll(".modal-overlay.active").forEach((modal) => {
      modal.classList.remove("active");
    });
  }
});

// Close modal when clicking overlay
document.querySelectorAll(".modal-overlay").forEach((overlay) => {
  overlay.addEventListener("click", function (e) {
    if (e.target === overlay) {
      overlay.classList.remove("active");
    }
  });
});
