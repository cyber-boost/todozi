import axios, { AxiosInstance, AxiosResponse } from 'axios';
import { generateUUID } from './models.js';
import { promises as fs } from 'fs';
import { homedir } from 'os';
import { join, dirname } from 'path';
import { format } from 'date-fns';

// Enums for type safety
export enum Priority {
  Low = 'Low',
  Medium = 'Medium',
  High = 'High',
  Critical = 'Critical'
}

export enum Status {
  Todo = 'Todo',
  InProgress = 'InProgress',
  Done = 'Done',
  Blocked = 'Blocked'
}

export enum MemoryImportance {
  Low = 'Low',
  Medium = 'Medium',
  High = 'High',
  Critical = 'Critical'
}

export enum MemoryTerm {
  Short = 'Short',
  Medium = 'Medium',
  Long = 'Long',
  Permanent = 'Permanent'
}

export enum ShareLevel {
  Private = 'Private',
  Team = 'Team',
  Public = 'Public'
}

export enum ItemStatus {
  Active = 'Active',
  Archived = 'Archived',
  Deleted = 'Deleted'
}

export enum MemoryType {
  Standard = 'Standard',
  Episodic = 'Episodic',
  Semantic = 'Semantic',
  Procedural = 'Procedural'
}

// Interface definitions
export interface ExtractResponse {
  tasks: ExtractedTask[];
  memories: ExtractedMemory[];
  ideas: ExtractedIdea[];
  errors: ExtractedError[];
  trainingData: ExtractedTrainingData[];
  rawTags: string[];
}

export interface ExtractedTask {
  action: string;
  time: string;
  priority: string;
  project: string;
  status: string;
  assignee?: string;
  tags: string[];
}

export interface ExtractedMemory {
  moment: string;
  meaning: string;
  reason: string;
  importance: string;
  term: string;
}

export interface ExtractedIdea {
  idea: string;
  share: string;
  importance: string;
}

export interface ExtractedError {
  title: string;
  description: string;
  severity: string;
  category: string;
}

export interface ExtractedTrainingData {
  prompt: string;
  completion: string;
  dataType: string;
}

export interface Task {
  id: string;
  userId: string;
  action: string;
  time: string;
  priority: Priority;
  parentProject: string;
  status: Status;
  assignee?: string;
  tags: string[];
  dependencies: string[];
  context?: string;
  progress?: number;
  createdAt: Date;
  updatedAt: Date;
}

export interface Memory {
  id: string;
  userId: string;
  projectId?: string;
  status: ItemStatus;
  moment: string;
  meaning: string;
  reason: string;
  importance: MemoryImportance;
  term: MemoryTerm;
  memoryType: MemoryType;
  tags: string[];
  createdAt: Date;
  updatedAt: Date;
}

export interface Idea {
  id: string;
  idea: string;
  projectId?: string;
  status: ItemStatus;
  share: ShareLevel;
  importance: string;
  tags: string[];
  context?: string;
  createdAt: Date;
  updatedAt: Date;
}

// Custom Error Classes
export class TodoziError extends Error {
  constructor(
    public type: 'validation' | 'io' | 'api' | 'config' | 'serialization',
    message: string,
    public details?: any
  ) {
    super(message);
    this.name = 'TodoziError';
  }

  static validation(message: string): TodoziError {
    return new TodoziError('validation', message);
  }

  static io(message: string, details?: any): TodoziError {
    return new TodoziError('io', message, details);
  }

  static api(message: string, details?: any): TodoziError {
    return new TodoziError('api', message, details);
  }

  static config(message: string): TodoziError {
    return new TodoziError('config', message);
  }

  static serialization(message: string): TodoziError {
    return new TodoziError('serialization', message);
  }

  static fromError(error: any): TodoziError {
    if (error instanceof TodoziError) {
      return error;
    }
    return new TodoziError('api', error.message || 'Unknown error', error);
  }
}

// Logger Interface and Implementation
export interface Logger {
  info(message: string): void;
  debug(message: string): void;
  error(message: string): void;
}

export class ConsoleLogger implements Logger {
  info(message: string): void {
    console.log(`ℹ️  ${message}`);
  }

  debug(message: string): void {
    console.log(`🔍 ${message}`);
  }

  error(message: string): void {
    console.error(`❌ ${message}`);
  }
}

// Type Guards
export function isValidExtractedTask(obj: any): obj is ExtractedTask {
  return obj && 
    typeof obj.action === 'string' && 
    typeof obj.time === 'string' && 
    typeof obj.priority === 'string' &&
    typeof obj.project === 'string' &&
    typeof obj.status === 'string' &&
    Array.isArray(obj.tags);
}

export function isValidExtractedMemory(obj: any): obj is ExtractedMemory {
  return obj && 
    typeof obj.moment === 'string' && 
    typeof obj.meaning === 'string' && 
    typeof obj.reason === 'string' &&
    typeof obj.importance === 'string' &&
    typeof obj.term === 'string';
}

export function isValidExtractedIdea(obj: any): obj is ExtractedIdea {
  return obj && 
    typeof obj.idea === 'string' && 
    typeof obj.share === 'string' && 
    typeof obj.importance === 'string';
}

export function isValidExtractedError(obj: any): obj is ExtractedError {
  return obj && 
    typeof obj.title === 'string' && 
    typeof obj.description === 'string' && 
    typeof obj.severity === 'string' &&
    typeof obj.category === 'string';
}

export function isValidExtractedTrainingData(obj: any): obj is ExtractedTrainingData {
  return obj && 
    typeof obj.prompt === 'string' && 
    typeof obj.completion === 'string' && 
    typeof obj.dataType === 'string';
}

// Utility Functions
export function parsePriority(priority: string): Priority {
  const parsed = Object.values(Priority).find(p => p === priority);
  return parsed || Priority.Medium;
}

export function parseStatus(status: string): Status {
  const parsed = Object.values(Status).find(s => s === status);
  return parsed || Status.Todo;
}

export function parseMemoryImportance(importance: string): MemoryImportance {
  const parsed = Object.values(MemoryImportance).find(i => i === importance);
  return parsed || MemoryImportance.Medium;
}

export function parseShareLevel(share: string): ShareLevel {
  const parsed = Object.values(ShareLevel).find(s => s === share);
  return parsed || ShareLevel.Private;
}

export function parseMemoryTerm(term: string): MemoryTerm {
  const parsed = Object.values(MemoryTerm).find(t => t === term);
  return parsed || MemoryTerm.Short;
}

// Configuration Management
export interface UserConfig {
  userId: string;
  fingerprint: string;
}

export async function loadUserConfig(): Promise<UserConfig> {
  const configPath = join(homedir(), '.todozi', 'tdz.hlx');
  
  try {
    const configContent = await fs.readFile(configPath, 'utf-8');
    const config = JSON.parse(configContent);
    
    return {
      userId: config.registration?.userId || '',
      fingerprint: config.registration?.fingerprint || ''
    };
  } catch (error) {
    return {
      userId: '',
      fingerprint: ''
    };
  }
}

// Hash function for project names
export async function hashProjectName(projectName: string): Promise<string> {
  const encoder = new TextEncoder();
  const data = encoder.encode(projectName);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// Embedding Service (Mock)
export class TodoziEmbeddingService {
  constructor(private config: any) {}

  async addTask(task: Task): Promise<string> {
    // Mock implementation - replace with actual embedding logic
    task.id = generateUUID();
    return task.id;
  }

  async newMemory(memory: Memory): Promise<string> {
    // Mock implementation - replace with actual embedding logic
    memory.id = generateUUID();
    return memory.id;
  }

  async newIdea(idea: Idea): Promise<string> {
    // Mock implementation - replace with actual embedding logic
    idea.id = generateUUID();
    return idea.id;
  }
}

// History Logger
export async function logToHistory(task: Task): Promise<void> {
  const historyDir = join(homedir(), '.todozi', 'history', 'core');
  
  try {
    await fs.mkdir(historyDir, { recursive: true });
    
    const megaFilePath = join(historyDir, 'mega');
    const timestamp = format(new Date(), 'yyyy-MM-dd HH:mm:ss UTC');
    
    const logEntry = `[${timestamp}] EXTRACTED_TASK: ${task.action} | Project: ${task.parentProject} | Priority: ${task.priority} | Status: ${task.status} | Tags: ${task.tags.join(', ')}\n`;
    
    await fs.appendFile(megaFilePath, logEntry, 'utf-8');
  } catch (error) {
    throw TodoziError.io(`Failed to write to history: ${error instanceof Error ? error.message : String(error)}`, error);
  }
}

// API Client Class
export class TodoziApiClient {
  private client: AxiosInstance;
  private apiKey: string = '';
  private userConfig: UserConfig = { userId: '', fingerprint: '' };
  private logger: Logger;

  constructor(logger?: Logger) {
    this.logger = logger || new ConsoleLogger();
    this.client = axios.create({
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  async initialize(): Promise<void> {
    this.apiKey = await this.getApiKey();
    this.userConfig = await loadUserConfig();
    
    this.logger.info(`API Key: ${this.apiKey ? '(configured)' : '(empty)'}`);
  }

  private async getApiKey(): Promise<string> {
    // Mock implementation - replace with actual API key retrieval
    return process.env.TODZI_API_KEY || '';
  }

  private async readContent(content?: string, filePath?: string): Promise<string> {
    if (content && filePath) {
      throw TodoziError.validation('Cannot provide both content and file');
    }
    
    if (!content && !filePath) {
      throw TodoziError.validation('Either content or file must be provided');
    }
    
    if (filePath) {
      try {
        return await fs.readFile(filePath, 'utf-8');
      } catch (error) {
        throw TodoziError.io(`Failed to read file ${filePath}: ${error instanceof Error ? error.message : String(error)}`, error);
      }
    }
    
    return content!;
  }

  async extractWithEndpoint(
    content?: string,
    filePath?: string,
    outputFormat: string = 'json',
    human: boolean = false,
    endpoint: string = 'plan'
  ): Promise<string> {
    try {
      const inputContent = await this.readContent(content, filePath);
      
      const url = `https://todozi.com/api/tdz/${endpoint}`;
      const payload = {
        content: inputContent,
        extractAll: true,
        model: 'gpt-oss:120b',
        language: 'english',
        userId: this.userConfig.userId,
        fingerprint: this.userConfig.fingerprint
      };

      this.logger.info(`Sending request to: ${url}`);
      this.logger.debug(`Payload: ${JSON.stringify(payload, null, 2)}`);

      const response: AxiosResponse = await this.client.post(url, payload, {
        headers: {
          'Authorization': `Bearer ${this.apiKey}`
        }
      });

      if (response.status !== 200) {
        const errorText = response.data || 'Unknown error';
        throw TodoziError.api(`API request failed: ${errorText}`);
      }

      this.logger.debug(`Raw API Response: ${JSON.stringify(response.data, null, 2)}`);

      const extractResponse = this.parseApiResponse(response.data);
      
      await this.processAndSaveContent(extractResponse);

      const output = this.formatOutput(extractResponse, outputFormat);

      if (human) {
        await this.generateHumanChecklist(extractResponse, endpoint);
      }

      return output;
    } catch (error) {
      if (error instanceof TodoziError) {
        throw error;
      }
      throw TodoziError.fromError(error);
    }
  }

  private parseApiResponse(apiResponse: any): ExtractResponse {
    const extractResponse: ExtractResponse = {
      tasks: [],
      memories: [],
      ideas: [],
      errors: [],
      trainingData: [],
      rawTags: []
    };

    // Extract tasks
    if (Array.isArray(apiResponse.tasks)) {
      for (const task of apiResponse.tasks) {
        if (isValidExtractedTask(task)) {
          extractResponse.tasks.push(task);
        }
      }
    }

    // Extract memories
    if (Array.isArray(apiResponse.memories)) {
      for (const memory of apiResponse.memories) {
        if (isValidExtractedMemory(memory)) {
          extractResponse.memories.push(memory);
        }
      }
    }

    // Extract ideas
    if (Array.isArray(apiResponse.ideas)) {
      for (const idea of apiResponse.ideas) {
        if (isValidExtractedIdea(idea)) {
          extractResponse.ideas.push(idea);
        }
      }
    }

    // Extract errors
    if (Array.isArray(apiResponse.errors)) {
      for (const error of apiResponse.errors) {
        if (isValidExtractedError(error)) {
          extractResponse.errors.push(error);
        }
      }
    }

    // Extract training data
    if (Array.isArray(apiResponse.training_data)) {
      for (const data of apiResponse.training_data) {
        if (isValidExtractedTrainingData(data)) {
          extractResponse.trainingData.push(data);
        }
      }
    }

    // Extract raw tags
    if (Array.isArray(apiResponse.raw_tags)) {
      for (const tag of apiResponse.raw_tags) {
        if (typeof tag === 'string') {
          extractResponse.rawTags.push(tag);
        }
      }
    }

    return extractResponse;
  }

  private async processAndSaveContent(extractResponse: ExtractResponse): Promise<void> {
    if (extractResponse.tasks.length === 0 && 
        extractResponse.memories.length === 0 && 
        extractResponse.ideas.length === 0) {
      return;
    }

    const embeddingService = new TodoziEmbeddingService({});
    this.logger.info('Auto-embedding and saving extracted content...');

    const primaryProject = extractResponse.tasks[0]?.project || 'Default Project';
    const primaryProjectId = await hashProjectName(primaryProject);

    // Save tasks
    if (extractResponse.tasks.length > 0) {
      this.logger.info(`Saving ${extractResponse.tasks.length} extracted tasks...`);
      
      for (const extractedTask of extractResponse.tasks) {
        const projectId = await hashProjectName(extractedTask.project);
        
        const task: Task = {
          id: generateUUID(),
          userId: this.userConfig.userId,
          action: extractedTask.action,
          time: extractedTask.time,
          priority: parsePriority(extractedTask.priority),
          parentProject: projectId,
          status: parseStatus(extractedTask.status),
          assignee: extractedTask.assignee,
          tags: extractedTask.tags,
          dependencies: [],
          context: undefined,
          progress: undefined,
          createdAt: new Date(),
          updatedAt: new Date()
        };

        const taskId = await embeddingService.addTask(task);
        this.logger.info(`Saved task: ${extractedTask.action} (ID: ${taskId})`);
        
        await logToHistory(task);
      }
    }

    // Save memories
    if (extractResponse.memories.length > 0) {
      this.logger.info(`Saving ${extractResponse.memories.length} extracted memories...`);
      
      for (const extractedMemory of extractResponse.memories) {
        const memory: Memory = {
          id: generateUUID(),
          userId: this.userConfig.userId,
          projectId: primaryProjectId,
          status: ItemStatus.Active,
          moment: extractedMemory.moment,
          meaning: extractedMemory.meaning,
          reason: extractedMemory.reason,
          importance: parseMemoryImportance(extractedMemory.importance),
          term: parseMemoryTerm(extractedMemory.term),
          memoryType: MemoryType.Standard,
          tags: [],
          createdAt: new Date(),
          updatedAt: new Date()
        };

        const memoryId = await embeddingService.newMemory(memory);
        this.logger.info(`Saved memory: ${extractedMemory.moment} (ID: ${memoryId})`);
      }
    }

    // Save ideas
    if (extractResponse.ideas.length > 0) {
      this.logger.info(`Saving ${extractResponse.ideas.length} extracted ideas...`);
      
      for (const extractedIdea of extractResponse.ideas) {
        const idea: Idea = {
          id: generateUUID(),
          idea: extractedIdea.idea,
          projectId: primaryProjectId,
          status: ItemStatus.Active,
          share: parseShareLevel(extractedIdea.share),
          importance: extractedIdea.importance,
          tags: [],
          context: undefined,
          createdAt: new Date(),
          updatedAt: new Date()
        };

        const ideaId = await embeddingService.newIdea(idea);
        this.logger.info(`Saved idea: ${extractedIdea.idea} (ID: ${ideaId})`);
      }
    }
  }

  private formatOutput(response: ExtractResponse, outputFormat: string): string {
    switch (outputFormat.toLowerCase()) {
      case 'json':
        return this.formatAsJson(response);
      case 'csv':
        return this.formatAsCsv(response);
      case 'md':
      case 'markdown':
        return this.formatAsMarkdown(response);
      default:
        throw TodoziError.validation(`Unsupported output format: ${outputFormat}`);
    }
  }

  private formatAsJson(response: ExtractResponse): string {
    try {
      return JSON.stringify(response, null, 2);
    } catch (error) {
      throw TodoziError.serialization(`JSON formatting error: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  private formatAsCsv(response: ExtractResponse): string {
    const csv: string[] = [];
    
    // Tasks CSV
    if (response.tasks.length > 0) {
      csv.push('Type,Action,Time,Priority,Project,Status,Assignee,Tags');
      for (const task of response.tasks) {
        csv.push([
          'Task',
          `"${task.action.replace(/"/g, '""')}"`,
          `"${task.time}"`,
          task.priority,
          task.project,
          task.status,
          task.assignee || '',
          `"${task.tags.join(', ')}"`
        ].join(','));
      }
    }
    
    return csv.join('\n');
  }

  private formatAsMarkdown(response: ExtractResponse): string {
    const md: string[] = [];
    
    md.push('# Extracted Content\n');
    
    // Tasks
    if (response.tasks.length > 0) {
      md.push('## Tasks\n');
      for (let i = 0; i < response.tasks.length; i++) {
        const task = response.tasks[i];
        md.push(`${i + 1}. **${task.action}**`);
        md.push(`   - Time: ${task.time}`);
        md.push(`   - Priority: ${task.priority}`);
        md.push(`   - Project: ${task.project}`);
        md.push(`   - Status: ${task.status}`);
        if (task.assignee) {
          md.push(`   - Assignee: ${task.assignee}`);
        }
        if (task.tags.length > 0) {
          md.push(`   - Tags: ${task.tags.join(', ')}`);
        }
        md.push('');
      }
    }
    
    // Memories
    if (response.memories.length > 0) {
      md.push('## Memories\n');
      for (const memory of response.memories) {
        md.push(`- **${memory.moment}**: ${memory.meaning}`);
        md.push(`  - Reason: ${memory.reason}`);
        md.push(`  - Importance: ${memory.importance}`);
        md.push(`  - Term: ${memory.term}\n`);
      }
    }
    
    // Ideas
    if (response.ideas.length > 0) {
      md.push('## Ideas\n');
      for (const idea of response.ideas) {
        md.push(`- **${idea.idea}** (${idea.importance})`);
        md.push(`  - Share: ${idea.share}\n`);
      }
    }
    
    // Raw tags
    if (response.rawTags.length > 0) {
      md.push('## Raw Tags\n');
      md.push('```');
      for (const tag of response.rawTags) {
        md.push(tag);
      }
      md.push('```');
    }
    
    return md.join('\n');
  }

  private async generateHumanChecklist(response: ExtractResponse, endpoint: string): Promise<void> {
    const checklist = this.formatAsHumanChecklist(response);
    const timestamp = format(new Date(), 'yyyyMMdd_HHmmss');
    const checklistFilename = `todozi_checklist_${endpoint}_${timestamp}.md`;
    
    try {
      await fs.writeFile(checklistFilename, checklist, 'utf-8');
      this.logger.info(`Human checklist saved to: ${checklistFilename}`);
    } catch (error) {
      throw TodoziError.io(`Failed to write human checklist: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  private formatAsHumanChecklist(response: ExtractResponse): string {
    const checklist: string[] = [];
    
    checklist.push('# 📋 Todozi Human Checklist\n');
    checklist.push(`Generated: ${format(new Date(), 'yyyy-MM-dd HH:mm:ss UTC')}\n`);
    checklist.push('---\n');
    
    // Tasks as checkboxes
    if (response.tasks.length > 0) {
      checklist.push('## 📝 Tasks\n');
      for (const task of response.tasks) {
        checklist.push(`- [ ] **${task.action}**`);
        checklist.push(`  - 📁 Project: \`${task.project}\``);
        checklist.push(`  - ⏱️ Time: \`${task.time}\``);
        checklist.push(`  - 🎯 Priority: \`${task.priority}\``);
        checklist.push(`  - 📊 Status: \`${task.status}\``);
        
        if (task.assignee) {
          checklist.push(`  - 👤 Assignee: \`${task.assignee}\``);
        }
        
        if (task.tags.length > 0) {
          checklist.push(`  - 🏷️ Tags: ${task.tags.map(t => `\`${t}\``).join(', ')}`);
        }
        
        checklist.push('');
      }
    }
    
    // Memories as checkboxes
    if (response.memories.length > 0) {
      checklist.push('## 🧠 Memories to Record\n');
      for (const memory of response.memories) {
        checklist.push(`- [ ] **${memory.moment}**`);
        checklist.push(`  - 💡 Meaning: ${memory.meaning}`);
        checklist.push(`  - 🎯 Reason: ${memory.reason}`);
        checklist.push(`  - 📊 Importance: \`${memory.importance}\``);
        checklist.push(`  - ⏰ Term: \`${memory.term}\`\n`);
      }
    }
    
    // Ideas as checkboxes
    if (response.ideas.length > 0) {
      checklist.push('## 💡 Ideas to Explore\n');
      for (const idea of response.ideas) {
        checklist.push(`- [ ] **${idea.idea}**`);
        checklist.push(`  - 🔒 Share Level: \`${idea.share}\``);
        checklist.push(`  - ⭐ Importance: \`${idea.importance}\`\n`);
      }
    }
    
    // Errors as checkboxes
    if (response.errors.length > 0) {
      checklist.push('## ❌ Errors to Fix\n');
      for (const error of response.errors) {
        checklist.push(`- [ ] **${error.title}**`);
        checklist.push(`  - 📝 Description: ${error.description}`);
        checklist.push(`  - 🔥 Severity: \`${error.severity}\``);
        checklist.push(`  - 📂 Category: \`${error.category}\`\n`);
      }
    }
    
    // Training data as checkboxes
    if (response.trainingData.length > 0) {
      checklist.push('## 🎓 Training Data to Review\n');
      for (const data of response.trainingData) {
        checklist.push(`- [ ] **${data.prompt}**`);
        checklist.push(`  - 📦 Type: \`${data.dataType}\``);
        checklist.push(`  - ✅ Completion: ${data.completion}\n`);
      }
    }
    
    // Summary
    checklist.push('---\n');
    checklist.push('## 📊 Summary\n');
    checklist.push(`- Total Tasks: **${response.tasks.length}**`);
    checklist.push(`- Total Memories: **${response.memories.length}**`);
    checklist.push(`- Total Ideas: **${response.ideas.length}**`);
    checklist.push(`- Total Errors: **${response.errors.length}**`);
    checklist.push(`- Total Training Items: **${response.trainingData.length}**`);
    checklist.push(`\n**Grand Total:** ${response.tasks.length + response.memories.length + response.ideas.length + response.errors.length + response.trainingData.length} items`);
    
    return checklist.join('\n');
  }

  // Public API methods
  async extractContent(
    content?: string,
    filePath?: string,
    outputFormat: string = 'json',
    human: boolean = false
  ): Promise<string> {
    return this.extractWithEndpoint(content, filePath, outputFormat, human, 'plan');
  }

  async strategyContent(
    content?: string,
    filePath?: string,
    outputFormat: string = 'json',
    human: boolean = false
  ): Promise<string> {
    return this.extractWithEndpoint(content, filePath, outputFormat, human, 'strategic');
  }

  destroy(): void {
    // Cleanup if needed
  }
}

// Export a default instance for convenience
export const todoziClient = new TodoziApiClient();
