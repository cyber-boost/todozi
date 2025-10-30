// Todozi TypeScript Definitions

export interface Task {
  id: string;
  user_id: string;
  action: string;
  time: string;
  priority: string;
  status: string;
  parent_project: string;
  tags: string[];
  progress?: number;
  created_at: string;
}

export interface Memory {
  id: string;
  moment: string;
  meaning: string;
  reason: string;
  importance: string;
  tags: string[];
}

export interface Idea {
  id: string;
  idea: string;
  importance: string;
  tags: string[];
}

export interface ApiKey {
  user_id: string;
  public_key: string;
  active: boolean;
}

export interface ApiKeyAuth {
  user_id: string;
  is_admin: boolean;
}

export interface Project {
  name: string;
  description?: string;
  status: string;
  created_at: string;
}

export class Todozi {
  constructor();

  // Core task operations
  task(action: string): string;
  urgent(action: string): string;
  high(action: string): string;
  low(action: string): string;

  // Search and find
  find(query: string): Task[];
  aiFind(query: string): Task[];

  // Task management
  done(taskId: string): void;
  start(taskId: string): void;
  all(): Task[];

  // Memory and ideas
  remember(moment: string, meaning: string): string;
  idea(idea: string): string;

  // Statistics
  stats(): string;

  // Advanced operations
  createTask(action: string, priority?: string, project?: string, time?: string, context?: string): Task;
  updateTaskStatus(taskId: string, status: string): void;
  searchTasks(query: string, semantic?: boolean, limit?: number): Task[];
  createProject(name: string, description?: string): void;
  listProjects(): string[];
  projectTasks(projectName: string): Task[];

  // Memory management
  createMemory(moment: string, meaning: string, reason: string): Task;
  listMemories(): Memory[];
  findMemories(query: string): Memory[];

  // Idea management
  createIdea(idea: string, context?: string): Task;
  listIdeas(): Idea[];
  findIdeas(query: string): Idea[];

  // Queue management
  queueAdd(taskName: string, description: string): string;
  queueList(): any[];
  queueBacklog(): any[];
  queueActive(): any[];
  queueStart(itemId: string): string;
  queueComplete(sessionId: string): void;

  // API Key management
  createApiKey(): ApiKey;
  createApiKeyWithUserId(userId: string): ApiKey;
  getApiKey(userId: string): ApiKey;
  getApiKeyByPublic(publicKey: string): ApiKey;
  listApiKeys(): ApiKey[];
  listActiveApiKeys(): ApiKey[];
  checkApiKeyAuth(publicKey: string, privateKey?: string): ApiKeyAuth;
  deactivateApiKey(userId: string): void;
  removeApiKey(userId: string): ApiKey;

  // CLI operations
  cliAddTask(content: string, priority?: string): void;
  cliListTasks(): Task[];
  cliShowTask(taskId: string): Task;
  cliUpdateTask(taskId: string, action?: string, priority?: string, status?: string): void;
  cliCompleteTask(taskId: string): void;
  cliDeleteTask(taskId: string): void;
  cliSearchTasks(query: string): Task[];
  cliCreateBackup(): string;
  cliListBackups(): string[];
  cliRestoreBackup(backupName: string): void;
  cliFixConsistency(): void;
  cliCreateMemory(moment: string, meaning: string, reason: string): void;
  cliCreateIdea(content: string): void;
  cliChat(message: string): string;
  cliRegisterWithServer(serverUrl: string): void;
  cliGetRegistrationStatus(): any;
  cliClearRegistration(): void;

  // System operations
  todoziInit(): void;
  todoziInitWithAutoRegistration(): void;
  todoziBegin(): void;
  getTdzApiKey(): string;
  ensureTodoziInitialized(): void;
  tdzfp(): boolean;

  // Additional methods...
  [key: string]: any;
}
