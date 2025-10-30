/**
 * Todozi – Browser‑only JavaScript client
 *
 * The client is a thin wrapper around the Todozi REST API.
 * It uses the native `fetch` API that is available in all modern browsers.
 *
 * ---------------------------------------------------------------
 * How to use:
 *
 *   // 1️⃣  Load the script (as a module or classic script)
 *   // 2️⃣  Create an instance
 *          const client = new TodoziClient('http://127.0.0.1:8636');
 *          // Or with API token:
 *          const client = new TodoziClient('http://127.0.0.1:8636', 'your-api-token');
 *
 *   // 3️⃣  Call any method – all return a Promise that resolves to JSON
 *          client.health().then(console.log);
 *
 * ---------------------------------------------------------------
 * If you prefer a classic (non‑module) script, the file automatically
 * registers `window.TodoziClient = TodoziClient`.
 */

const DEFAULT_BASE_URL = 'http://127.0.0.1:8636';

/**
 * TodoziClient
 *
 * All methods map 1‑to‑1 to the server endpoints.
 * Every request is made with `Content‑Type: application/json` and the
 * response is parsed with `response.json()`.
 */
class TodoziClient {
  /**
   * @param {string} baseUrl  Base URL of the Todozi server (default = DEFAULT_BASE_URL)
   * @param {string} apiToken  API token for authentication (optional)
   */
  constructor(baseUrl = DEFAULT_BASE_URL, apiToken = null) {
    this.baseUrl = baseUrl.replace(/\/+$/, ''); // strip trailing slash
    this.apiToken = apiToken;
  }

  /**
   * Low‑level helper – performs a fetch call and returns parsed JSON.
   * @param {string} endpoint  Path that will be appended to `baseUrl`
   * @param {object} [options] Fetch options (method, body, headers …)
   */
  async request(endpoint, options = {}) {
    const url = `${this.baseUrl}${endpoint}`;

    const headers = {
      'Content-Type': 'application/json',
      ...(options.headers || {})
    };

    // Add API token header if available
    if (this.apiToken) {
      headers['x-api-token'] = this.apiToken;
    }

    const response = await fetch(url, {
      headers,
      ...options
    });

    if (!response.ok) {
      // Try to pull a JSON error payload – if that fails we fall back to plain text.
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

    // Empty response (204 No Content) → return `null` rather than trying to parse JSON.
    if (response.status === 204) return null;

    return await response.json();
  }

  /* ------------------------------------------------------------------ *
   *  Health & System
   * ------------------------------------------------------------------ */
  health()                 { return this.request('/tdz/health'); }
  getStats()               { return this.request('/tdz/stats'); }
  initialize()             { return this.request('/tdz/init'); }

  /* ------------------------------------------------------------------ *
   *  Task Management
   * ------------------------------------------------------------------ */
  getTasks()               { return this.request('/tdz/tasks'); }
  createTask(task)         { return this.request('/tdz/tasks', { method: 'POST', body: JSON.stringify(task) }); }
  getTask(id)              { return this.request(`/tdz/tasks/${id}`); }
  updateTask(id, task)     { return this.request(`/tdz/tasks/${id}`, { method: 'PUT', body: JSON.stringify(task) }); }
  deleteTask(id)           { return this.request(`/tdz/tasks/${id}`, { method: 'DELETE' }); }
  searchTasks(query)       { return this.request(`/tdz/tasks/search?q=${encodeURIComponent(query)}`); }

  /* ------------------------------------------------------------------ *
   *  Memory Management (basic & enhanced)
   * ------------------------------------------------------------------ */
  getMemories()                { return this.request('/tdz/memories'); }
  createMemory(memory)         { return this.request('/tdz/memories', { method: 'POST', body: JSON.stringify(memory) }); }
  getMemoryTypes()             { return this.request('/tdz/memories/types'); }
  getSecretMemories()          { return this.request('/tdz/memories/secret'); }
  getHumanMemories()           { return this.request('/tdz/memories/human'); }
  getShortMemories()           { return this.request('/tdz/memories/short'); }
  getLongMemories()            { return this.request('/tdz/memories/long'); }
  getEmotionalMemories(emotion){ return this.request(`/tdz/memories/emotional/${emotion}`); }
  getMemory(id)                { return this.request(`/tdz/memories/${id}`); }
  updateMemory(id, memory)     { return this.request(`/tdz/memories/${id}`, { method: 'PUT', body: JSON.stringify(memory) }); }
  deleteMemory(id)             { return this.request(`/tdz/memories/${id}`, { method: 'DELETE' }); }

  /* ------------------------------------------------------------------ *
   *  Idea Management (basic & enhanced)
   * ------------------------------------------------------------------ */
  getIdeas()                { return this.request('/tdz/ideas'); }
  createIdea(idea)          { return this.request('/tdz/ideas', { method: 'POST', body: JSON.stringify(idea) }); }
  getIdea(id)               { return this.request(`/tdz/ideas/${id}`); }
  updateIdea(id, idea)      { return this.request(`/tdz/ideas/${id}`, { method: 'PUT', body: JSON.stringify(idea) }); }
  deleteIdea(id)            { return this.request(`/tdz/ideas/${id}`, { method: 'DELETE' }); }

  /* ------------------------------------------------------------------ *
   *  Agent Management
   * ------------------------------------------------------------------ */
  getAgents()               { return this.request('/tdz/agents'); }
  getAgent(id)              { return this.request(`/tdz/agents/${id}`); }
  getAvailableAgents()      { return this.request('/tdz/agents/available'); }
  getAgentStatus(id)        { return this.request(`/tdz/agents/${id}/status`); }
  createAgent(agent)        { return this.request('/tdz/agents', { method: 'POST', body: JSON.stringify(agent) }); }
  updateAgent(id, agent)    { return this.request(`/tdz/agents/${id}`, { method: 'PUT', body: JSON.stringify(agent) }); }
  deleteAgent(id)           { return this.request(`/tdz/agents/${id}`, { method: 'DELETE' }); }

  /* ------------------------------------------------------------------ *
   *  Chunking Management (basic & enhanced)
   * ------------------------------------------------------------------ */
  getChunks()               { return this.request('/tdz/chunks'); }
  createChunk(chunk)        { return this.request('/tdz/chunks', { method: 'POST', body: JSON.stringify(chunk) }); }
  getReadyChunks()          { return this.request('/tdz/chunks/ready'); }
  getChunkGraph()           { return this.request('/tdz/chunks/graph'); }
  getChunk(id)              { return this.request(`/tdz/chunks/${id}`); }
  updateChunk(id, chunk)    { return this.request(`/tdz/chunks/${id}`, { method: 'PUT', body: JSON.stringify(chunk) }); }
  deleteChunk(id)           { return this.request(`/tdz/chunks/${id}`, { method: 'DELETE' }); }

  /* ------------------------------------------------------------------ *
   *  Chat Processing
   * ------------------------------------------------------------------ */
  processChat(message, id) {
    return this.request('/tdz/chat/process', {
      method: 'POST',
      body: JSON.stringify({ message, id })
    });
  }

  chatWithAgent(agentId, chatData) {
    return this.request(`/tdz/chat/agent/${agentId}`, {
      method: 'POST',
      body: JSON.stringify(chatData)
    });
  }

  getChatHistory()          { return this.request('/tdz/chat/history'); }

  /* ------------------------------------------------------------------ *
   *  Project Management
   * ------------------------------------------------------------------ */
  getProjects()                 { return this.request('/tdz/projects'); }
  createProject(project)        { return this.request('/tdz/projects', { method: 'POST', body: JSON.stringify(project) }); }
  getProject(name)              { return this.request(`/tdz/projects/${name}`); }
  updateProject(name, project)  { return this.request(`/tdz/projects/${name}`, { method: 'PUT', body: JSON.stringify(project) }); }
  deleteProject(name)           { return this.request(`/tdz/projects/${name}`, { method: 'DELETE' }); }

  /* ------------------------------------------------------------------ *
   *  Training‑Data System
   * ------------------------------------------------------------------ */
  getTrainingData()               { return this.request('/tdz/training'); }
  createTrainingData(td)          { return this.request('/tdz/training', { method: 'POST', body: JSON.stringify(td) }); }
  getTrainingDataById(id)         { return this.request(`/tdz/training/${id}`); }
  updateTrainingData(id, td)      { return this.request(`/tdz/training/${id}`, { method: 'PUT', body: JSON.stringify(td) }); }
  deleteTrainingData(id)          { return this.request(`/tdz/training/${id}`, { method: 'DELETE' }); }
  exportTrainingData()            { return this.request('/tdz/training/export'); }
  getTrainingStats()              { return this.request('/tdz/training/stats'); }

  /* ------------------------------------------------------------------ *
   *  Analytics & Time‑Tracking
   * ------------------------------------------------------------------ */
  getTaskAnalytics()            { return this.request('/tdz/analytics/tasks'); }
  getAgentAnalytics()           { return this.request('/tdz/analytics/agents'); }
  getPerformanceAnalytics()     { return this.request('/tdz/analytics/performance'); }
  startTimeTracking(taskId)     { return this.request(`/tdz/time/start/${taskId}`,   { method: 'POST' }); }
  stopTimeTracking(taskId)      { return this.request(`/tdz/time/stop/${taskId}`,    { method: 'POST' }); }
  getTimeReport()               { return this.request('/tdz/time/report'); }

  /* ------------------------------------------------------------------ *
   *  Feeling Management
   * ------------------------------------------------------------------ */
  getFeelings()                     { return this.request('/tdz/feelings'); }
  createFeeling(feeling)            { return this.request('/tdz/feelings', { method: 'POST', body: JSON.stringify(feeling) }); }
  getFeeling(id)                    { return this.request(`/tdz/feelings/${id}`); }
  updateFeeling(id, feeling)        { return this.request(`/tdz/feelings/${id}`, { method: 'PUT', body: JSON.stringify(feeling) }); }
  deleteFeeling(id)                 { return this.request(`/tdz/feelings/${id}`, { method: 'DELETE' }); }
  searchFeelings(query)             { return this.request(`/tdz/feelings/search?q=${encodeURIComponent(query)}`); }

  /* ------------------------------------------------------------------ *
   *  Queue Management
   * ------------------------------------------------------------------ */
  planQueueItem(queueData)          { return this.request('/tdz/queue/plan',    { method: 'POST', body: JSON.stringify(queueData) }); }
  getQueueItems()                   { return this.request('/tdz/queue/list'); }
  getBacklogItems()                 { return this.request('/tdz/queue/list/backlog'); }
  getActiveItems()                  { return this.request('/tdz/queue/list/active'); }
  getCompleteItems()                { return this.request('/tdz/queue/list/complete'); }
  startQueueSession(itemId)         { return this.request(`/tdz/queue/start/${itemId}`,   { method: 'POST' }); }
  endQueueSession(sessionId)        { return this.request(`/tdz/queue/end/${sessionId}`,   { method: 'POST' }); }

  /* ------------------------------------------------------------------ *
   *  API‑Key Management
   * ------------------------------------------------------------------ */
  registerApiKey()                  { return this.request('/tdz/api/register', { method: 'POST' }); }
  checkApiKey(authData)             { return this.request('/tdz/api/check',    { method: 'POST', body: JSON.stringify(authData) }); }

  /* ------------------------------------------------------------------ *
   *  Backup & Restore
   * ------------------------------------------------------------------ */
  createBackup()                    { return this.request('/tdz/backup', { method: 'POST' }); }
  getBackups()                      { return this.request('/tdz/backups'); }
  restoreBackup(name)               { return this.request(`/tdz/restore/${name}`, { method: 'POST' }); }
}

/* --------------------------------------------------------------- *
 *  Export / Global registration
 * --------------------------------------------------------------- */

// ES Module export (default)
export default TodoziClient;

if (typeof define === 'function' && define.amd) {
  // AMD (RequireJS)
  define(() => TodoziClient);
} else if (typeof exports === 'object' && typeof module !== 'undefined') {
  // CommonJS (not used in browsers, but kept for completeness)
  module.exports = { TodoziClient };
} else {
  // Classic script – expose a global variable
  window.TodoziClient = TodoziClient;
}