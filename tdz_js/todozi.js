/**
 * Todozi - AI/Human Task Management System
 *
 * Professional Node.js bindings for the Todozi task management system written in Rust.
 *
 * Usage:
 *   const { TodoziClient } = require('./index.js');
 *   const client = new TodoziClient();
 *
 *   // Core operations
 *   client.task("Complete project");
 *   client.aiFind("machine learning tasks");
 *   client.embed("Analyze this text");
 *
 *   // All 175 Rust API methods available through this unified interface
 */

const { Todozi } = require('./binding');

/**
 * Professional JavaScript client for Todozi - complete Rust API wrapper
 *
 * This class provides a clean, professional interface to all Todozi functionality,
 * wrapping the complete Rust API with JavaScript-friendly method names and documentation.
 * All 175 Rust API methods are available through this unified interface.
 */
class TodoziClient {
  constructor() {
    this.client = new Todozi();
  }

  // ========== Core Task Operations ==========
  task(action) {
    return this.client.task(action);
  }

  urgent(action) {
    return this.client.urgent(action);
  }

  high(action) {
    return this.client.high(action);
  }

  low(action) {
    return this.client.low(action);
  }

  find(query) {
    return this.client.find(query);
  }

  aiFind(query) {
    return this.client.aiFind(query);
  }

  done(taskId) {
    return this.client.done(taskId);
  }

  start(taskId) {
    return this.client.start(taskId);
  }

  all() {
    return this.client.all();
  }

  remember(moment, meaning) {
    return this.client.remember(moment, meaning);
  }

  idea(idea) {
    return this.client.idea(idea);
  }

  aiTask(action) {
    return this.client.aiTask(action);
  }

  humanTask(action) {
    return this.client.humanTask(action);
  }

  collabTask(action) {
    return this.client.collabTask(action);
  }

  createProject(name, description = null) {
    return this.client.createProject(name, description);
  }

  listProjects() {
    return this.client.listProjects();
  }

  projectTasks(projectName) {
    return this.client.projectTasks(projectName);
  }

  createMemory(moment, meaning, reason) {
    return this.client.createMemory(moment, meaning, reason);
  }

  listMemories() {
    return this.client.listMemories();
  }

  createIdea(idea) {
    return this.client.createIdea(idea);
  }

  listIdeas() {
    return this.client.listIdeas();
  }

  stats() {
    return this.client.stats();
  }

  detailedStats() {
    return this.client.detailedStats();
  }

  setProject(projectName) {
    return this.client.setProject(projectName);
  }

  // ========== Initialization & Setup ==========
  todoziInit() {
    return this.client.todoziInit();
  }

  todoziInitWithAutoRegistration() {
    return this.client.todoziInitWithAutoRegistration();
  }

  todoziBegin() {
    return this.client.todoziBegin();
  }

  getTdzApiKey() {
    return this.client.getTdzApiKey();
  }

  ensureTodoziInitialized() {
    return this.client.ensureTodoziInitialized();
  }

  tdzfp() {
    return this.client.tdzfp();
  }

  // ========== Done API (Advanced Task Operations) ==========
  doneInit() {
    return this.client.doneInit();
  }

  doneApiKey() {
    return this.client.doneApiKey();
  }

  doneStorage() {
    return this.client.doneStorage();
  }

  doneEmbeddingService() {
    return this.client.doneEmbeddingService();
  }

  doneTypes() {
    return this.client.doneTypes();
  }

  doneSampleTask() {
    return this.client.doneSampleTask();
  }

  doneEmbeddingConfig() {
    return this.client.doneEmbeddingConfig();
  }

  createTask(action, priority = null, project = null, time = null, context = null) {
    return this.client.createTask(action, priority, project, time, context);
  }

  searchTasks(query, semantic = false, limit = null) {
    return this.client.searchTasks(query, semantic, limit);
  }

  updateTaskStatus(taskId, status) {
    return this.client.updateTaskStatus(taskId, status);
  }

  planTasks(goal, complexity = null, timeline = null, context = null) {
    return this.client.planTasks(goal, complexity, timeline, context);
  }

  listTasks() {
    return this.client.listTasks();
  }

  getTask(taskId) {
    return this.client.getTask(taskId);
  }

  deleteTask(taskId) {
    return this.client.deleteTask(taskId);
  }

  quickTask(action) {
    return this.client.quickTask(action);
  }

  findTasks(query) {
    return this.client.findTasks(query);
  }

  findTasksAi(query) {
    return this.client.findTasksAi(query);
  }

  allTasks() {
    return this.client.allTasks();
  }

  completeTask(taskId) {
    return this.client.completeTask(taskId);
  }

  startTask(taskId) {
    return this.client.startTask(taskId);
  }

  extractTaskActions(content) {
    return this.client.extractTaskActions(content);
  }

  planTaskActions(goal) {
    return this.client.planTaskActions(goal);
  }

  doneProcessChat(message, userId) {
    return this.client.doneProcessChat(message, userId);
  }

  tdzCnt(content, sessionId = null) {
    return this.client.tdzCnt(content, sessionId);
  }

  doneCreateStorage() {
    return this.client.doneCreateStorage();
  }

  doneCreateEmbeddingService() {
    return this.client.doneCreateEmbeddingService();
  }

  // ========== Actions API ==========
  complete(taskId) {
    return this.client.complete(taskId);
  }

  delete(taskId) {
    return this.client.delete(taskId);
  }

  get(taskId) {
    return this.client.get(taskId);
  }

  list() {
    return this.client.list();
  }

  begin(taskId) {
    return this.client.begin(taskId);
  }

  // ========== Project Management ==========
  deleteProject(projectName) {
    return this.client.deleteProject(projectName);
  }

  // ========== Memory Management ==========
  importantMemory(moment, meaning, reason) {
    return this.client.importantMemory(moment, meaning, reason);
  }

  findMemories(query) {
    return this.client.findMemories(query);
  }

  // ========== Idea Management ==========
  breakthroughIdea(idea) {
    return this.client.breakthroughIdea(idea);
  }

  findIdeas(query) {
    return this.client.findIdeas(query);
  }

  // ========== Queue Management ==========
  queueAdd(taskName, description) {
    return this.client.queueAdd(taskName, description);
  }

  queueList() {
    return this.client.queueList();
  }

  queueBacklog() {
    return this.client.queueBacklog();
  }

  queueActive() {
    return this.client.queueActive();
  }

  queueStart(itemId) {
    return this.client.queueStart(itemId);
  }

  queueComplete(sessionId) {
    return this.client.queueComplete(sessionId);
  }

  // ========== Search & Find API ==========
  tdzFind(query) {
    return this.client.tdzFind(query);
  }

  aiSearch(query) {
    return this.client.aiSearch(query);
  }

  keywordSearch(query) {
    return this.client.keywordSearch(query);
  }

  smartSearch(query) {
    return this.client.smartSearch(query);
  }

  aiTasks(query) {
    return this.client.aiTasks(query);
  }

  keywordTasks(query) {
    return this.client.keywordTasks(query);
  }

  similarTasks(taskId) {
    return this.client.similarTasks(taskId);
  }

  fastSearch(query) {
    return this.client.fastSearch(query);
  }

  deepSearch(query) {
    return this.client.deepSearch(query);
  }

  // ========== Embedding API ==========
  embed(text) {
    return this.client.embed(text);
  }

  similar(query) {
    return this.client.similar(query);
  }

  similarTasksEmb(query) {
    return this.client.similarTasksEmb(query);
  }

  cluster() {
    return this.client.cluster();
  }

  embedStats() {
    return this.client.embedStats();
  }

  embedTask(taskId) {
    return this.client.embedTask(taskId);
  }

  // ========== Easy API (Simplified Interface) ==========
  doIt(what) {
    return this.client.doIt(what);
  }

  easyFind(what) {
    return this.client.easyFind(what);
  }

  easyRemember(what) {
    return this.client.easyRemember(what);
  }

  easyIdea(what) {
    return this.client.easyIdea(what);
  }

  easyDone(taskId) {
    return this.client.easyDone(taskId);
  }

  seeAll() {
    return this.client.seeAll();
  }

  // ========== Tags API ==========
  findByTag(tagName) {
    return this.client.findByTag(tagName);
  }

  addTagToTask(taskId, tag) {
    return this.client.addTagToTask(taskId, tag);
  }

  removeTagFromTask(taskId, tag) {
    return this.client.removeTagFromTask(taskId, tag);
  }

  // ========== Configuration API ==========
  getProject() {
    return this.client.getProject();
  }

  // ========== Storage API ==========
  storageInit() {
    return this.client.storageInit();
  }

  storageCheckFolderStructure() {
    return this.client.storageCheckFolderStructure();
  }

  storageEnsureFolderStructure() {
    return this.client.storageEnsureFolderStructure();
  }

  storageIsRegistered() {
    return this.client.storageIsRegistered();
  }

  storageClearRegistration() {
    return this.client.storageClearRegistration();
  }

  storageListProjects() {
    return this.client.storageListProjects();
  }

  storageLoadProject(name) {
    return this.client.storageLoadProject(name);
  }

  storageSaveProject(name) {
    return this.client.storageSaveProject(name);
  }

  storageDeleteProjectByName(name) {
    return this.client.storageDeleteProjectByName(name);
  }

  storageGetStorageDir() {
    return this.client.storageGetStorageDir();
  }

  storageLoadConfig() {
    return this.client.storageLoadConfig();
  }

  storageSaveConfig() {
    return this.client.storageSaveConfig();
  }

  storageLoadTaskCollection(name) {
    return this.client.storageLoadTaskCollection(name);
  }

  storageSaveTaskCollection(name) {
    return this.client.storageSaveTaskCollection(name);
  }

  storageGetRegistrationInfo() {
    return this.client.storageGetRegistrationInfo();
  }

  storageRegisterWithServer(serverUrl) {
    return this.client.storageRegisterWithServer(serverUrl);
  }

  // ========== TDZ Commands API ==========
  tdzExecuteCommand(command) {
    return this.client.tdzExecuteCommand(command);
  }

  tdzParseCommand(input) {
    return this.client.tdzParseCommand(input);
  }

  // ========== Extract API ==========
  extractContent(text) {
    return this.client.extractContent(text);
  }

  strategyContent(text) {
    return this.client.strategyContent(text);
  }

  // ========== Reminder API ==========
  activateReminder(reminderId) {
    return this.client.activateReminder(reminderId);
  }

  // ========== Emb API ==========
  addTaskEmb(task) {
    return this.client.addTaskEmb(task);
  }

  // ========== TDZ TLS API ==========
  addChecklistItem(item) {
    return this.client.addChecklistItem(item);
  }

  addRecentAction(action) {
    return this.client.addRecentAction(action);
  }

  // ========== Tags API ==========
  addTagRelationship(tag1, tag2) {
    return this.client.addTagRelationship(tag1, tag2);
  }

  bulkCreateTags(tags, category = null) {
    return this.client.bulkCreateTags(tags, category);
  }

  createTag(name, description = null, category = null) {
    return this.client.createTag(name, description, category);
  }

  getAllCategories() {
    return this.client.getAllCategories();
  }

  // ========== Models API ==========
  addItem(content, priority) {
    return this.client.addItem(content, priority);
  }

  // ========== Storage API ==========
  addQueueItem(content, priority) {
    return this.client.addQueueItem(content, priority);
  }

  addTaskToProject(task) {
    return this.client.addTaskToProject(task);
  }

  archiveProject(projectName) {
    return this.client.archiveProject(projectName);
  }

  clearRegistration() {
    return this.client.clearRegistration();
  }

  loadProject(projectName) {
    return this.client.loadProject(projectName);
  }

  saveProject(project) {
    return this.client.saveProject(project);
  }

  saveTask(task) {
    return this.client.saveTask(task);
  }

  loadTask(taskId) {
    return this.client.loadTask(taskId);
  }

  // ========== Search API ==========
  advancedSearch(query) {
    return this.client.advancedSearch(query);
  }

  // ========== Models API ==========
  addKey(key) {
    return this.client.addKey(key);
  }

  addTask(task) {
    return this.client.addTask(task);
  }

  // ========== Search API ==========
  tagsAdvancedSearch(query) {
    return this.client.tagsAdvancedSearch(query);
  }

  // ========== Chunking API ==========
  addChunk(chunkId, level, deps) {
    return this.client.addChunk(chunkId, level, deps);
  }

  addCompletedModule(module) {
    return this.client.addCompletedModule(module);
  }

  addDependency(dep) {
    return this.client.addDependency(dep);
  }

  addErrorPattern(pattern) {
    return this.client.addErrorPattern(pattern);
  }

  addFunctionSignature(name, signature) {
    return this.client.addFunctionSignature(name, signature);
  }

  addImport(importStmt) {
    return this.client.addImport(importStmt);
  }

  addPendingModule(module) {
    return this.client.addPendingModule(module);
  }

  // ========== Agent API ==========
  deleteAgent(agentId) {
    return this.client.deleteAgent(agentId);
  }

  // ========== API Key Management ==========
  createApiKey() {
    return this.client.createApiKey();
  }

  createApiKeyWithUserId(userId) {
    return this.client.createApiKeyWithUserId(userId);
  }

  getApiKey(userId) {
    return this.client.getApiKey(userId);
  }

  getApiKeyByPublic(publicKey) {
    return this.client.getApiKeyByPublic(publicKey);
  }

  listApiKeys() {
    return this.client.listApiKeys();
  }

  listActiveApiKeys() {
    return this.client.listActiveApiKeys();
  }

  checkApiKeyAuth(publicKey, privateKey = null) {
    return this.client.checkApiKeyAuth(publicKey, privateKey);
  }

  deactivateApiKey(userId) {
    return this.client.deactivateApiKey(userId);
  }

  removeApiKey(userId) {
    return this.client.removeApiKey(userId);
  }

  // ========== CLI Operations ==========
  cliAddTask(content, priority = null) {
    return this.client.cliAddTask(content, priority);
  }

  cliListTasks() {
    return this.client.cliListTasks();
  }

  cliShowTask(taskId) {
    return this.client.cliShowTask(taskId);
  }

  cliUpdateTask(taskId, action = null, priority = null, status = null) {
    return this.client.cliUpdateTask(taskId, action, priority, status);
  }

  cliCompleteTask(taskId) {
    return this.client.cliCompleteTask(taskId);
  }

  cliDeleteTask(taskId) {
    return this.client.cliDeleteTask(taskId);
  }

  cliSearchTasks(query) {
    return this.client.cliSearchTasks(query);
  }

  cliCreateBackup() {
    return this.client.cliCreateBackup();
  }

  cliListBackups() {
    return this.client.cliListBackups();
  }

  cliRestoreBackup(backupName) {
    return this.client.cliRestoreBackup(backupName);
  }

  cliFixConsistency() {
    return this.client.cliFixConsistency();
  }

  cliCreateMemory(moment, meaning, reason) {
    return this.client.cliCreateMemory(moment, meaning, reason);
  }

  cliCreateIdea(content) {
    return this.client.cliCreateIdea(content);
  }

  cliChat(message) {
    return this.client.cliChat(message);
  }

  cliRegisterWithServer(serverUrl) {
    return this.client.cliRegisterWithServer(serverUrl);
  }

  cliGetRegistrationStatus() {
    return this.client.cliGetRegistrationStatus();
  }

  cliClearRegistration() {
    return this.client.cliClearRegistration();
  }

  // ========== Chat & AI ==========
  chat(message) {
    return this.client.chat(message);
  }

  // ========== Content Processing ==========
  extractTasks(content, context = null) {
    return this.client.extractTasks(content, context);
  }
}

module.exports = { TodoziClient, Todozi };
