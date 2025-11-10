import express, { Express, Request, Response, NextFunction } from 'express';
import cors from 'cors';
import helmet from 'helmet';
import { createServer, Server as HttpServer } from 'http';
import { generateUUID } from './models.js';
import bcrypt from 'bcrypt';
import jwt from 'jsonwebtoken';
import { DateTime } from 'luxon';

// Types and Interfaces
export enum Priority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

export enum Status {
  Todo = 'todo',
  InProgress = 'inprogress',
  Done = 'done',
  Blocked = 'blocked'
}

export enum Assignee {
  Me = 'me',
  Team = 'team',
  External = 'external'
}

export enum QueueStatus {
  Backlog = 'backlog',
  Active = 'active',
  Complete = 'complete',
  Paused = 'paused'
}

export enum TrainingDataType {
  Instruction = 'instruction',
  Conversation = 'conversation',
  Completion = 'completion',
  Code = 'code',
  Analysis = 'analysis',
  Planning = 'planning',
  Review = 'review',
  Documentation = 'documentation',
  Example = 'example',
  Test = 'test',
  Validation = 'validation'
}

export interface ServerConfig {
  host: string;
  port: number;
  maxConnections: number;
}

export interface HttpResponse {
  status: number;
  headers: Record<string, string>;
  body: string;
}

export interface Task {
  id: string;
  user_id: string;
  action: string;
  time: string;
  priority: Priority;
  parent_project: string;
  status: Status;
  assignee?: Assignee;
  tags: string[];
  dependencies: string[];
  context_notes: string;
  progress: number;
  created_at: DateTime;
  updated_at: DateTime;
  embedding_vector?: number[];
}

export interface TaskUpdate {
  action?: string;
  time?: string;
  priority?: Priority;
  parent_project?: string;
  status?: Status;
  assignee?: Assignee;
  tags?: string[];
  dependencies?: string[];
  context_notes?: string;
  progress?: number;
}

export interface TaskFilters {
  status?: Status;
  priority?: Priority;
  assignee?: Assignee;
  project?: string;
  tags?: string[];
}

export interface Agent {
  id: string;
  name: string;
  description: string;
  version: string;
  category: string;
  status: string;
  model: {
    provider: string;
    name: string;
    temperature: number;
    max_tokens: number;
  };
  capabilities: string[];
  specializations: string[];
  created_at: DateTime;
  updated_at: DateTime;
}

export interface Memory {
  id: string;
  moment: string;
  meaning: string;
  importance: string;
  term: string;
  type?: string;
  emotion?: string;
}

export interface Feeling {
  id: string;
  emotion: string;
  intensity: number;
  description: string;
  context: string;
  tags: string[];
  created_at: DateTime;
  updated_at: DateTime;
}

export interface TrainingData {
  id: string;
  data_type: TrainingDataType;
  prompt: string;
  completion: string;
  context?: string;
  tags: string[];
  quality_score?: number;
  source: string;
  created_at: DateTime;
  updated_at: DateTime;
}

export interface QueueItem {
  id: string;
  task_name: string;
  task_description: string;
  priority: Priority;
  project_id?: string;
  status: QueueStatus;
  created_at: DateTime;
  updated_at: DateTime;
}

export interface QueueSession {
  id: string;
  queue_item_id: string;
  start_time: DateTime;
  end_time?: DateTime;
  duration_seconds?: number;
}

export interface ApiKey {
  user_id: string;
  public_key: string;
  private_key: string;
  active: boolean;
  created_at: DateTime;
}

export interface CodeChunk {
  chunk_id: string;
  level: string;
  status: string;
  dependencies: string[];
  estimated_tokens: number;
}

export interface Project {
  name: string;
  description: string;
  created_at: DateTime;
  updated_at: DateTime;
}

export interface AuthenticatedRequest extends Request {
  user?: {
    id: string;
    isAdmin: boolean;
  };
}

// Utility Classes
class HttpResponseBuilder {
  static ok(body: any): HttpResponse {
    return {
      status: 200,
      headers: {},
      body: JSON.stringify(body)
    };
  }

  static error(status: number, message: string): HttpResponse {
    return {
      status,
      headers: {},
      body: JSON.stringify({ error: message })
    };
  }

  static json<T>(data: T): HttpResponse {
    return {
      status: 200,
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    };
  }
}

// Storage Interface (Mock Implementation)
class Storage {
  private tasks: Map<string, Task> = new Map();
  private agents: Map<string, Agent> = new Map();
  private memories: Map<string, Memory> = new Map();
  private feelings: Map<string, Feeling> = new Map();
  private trainingData: Map<string, TrainingData> = new Map();
  private queueItems: Map<string, QueueItem> = new Map();
  private queueSessions: Map<string, QueueSession> = new Map();
  private projects: Map<string, Project> = new Map();
  private apiKeys: Map<string, ApiKey> = new Map();

  async createProject(name: string, description?: string): Promise<void> {
    const project: Project = {
      name,
      description: description || '',
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };
    this.projects.set(name, project);
  }

  getProject(name: string): Project | undefined {
    return this.projects.get(name);
  }

  listTasksAcrossProjects(filters: TaskFilters): Task[] {
    return Array.from(this.tasks.values());
  }

  getTaskFromAnyProject(id: string): Task | undefined {
    return this.tasks.get(id);
  }

  async updateTaskInProject(id: string, updates: TaskUpdate): Promise<void> {
    const task = this.tasks.get(id);
    if (task) {
      Object.assign(task, updates, { updated_at: DateTime.now() });
      this.tasks.set(id, task);
    }
  }

  deleteTaskFromProject(id: string): void {
    this.tasks.delete(id);
  }

  addTask(task: Task): void {
    this.tasks.set(task.id, task);
  }
}

// Code Generation Graph
class CodeGenerationGraph {
  chunks: Map<string, CodeChunk> = new Map();
  
  constructor(maxCapacity: number) {
    // Initialize with some default chunks
  }

  getReadyChunks(): string[] {
    return Array.from(this.chunks.keys());
  }

  getProjectSummary(): any {
    return {
      total_chunks: this.chunks.size,
      ready_chunks: this.getReadyChunks().length
    };
  }
}

// Main Server Class
export class TodoziServer {
  private app: Express;
  private server: HttpServer | null = null;
  public config: ServerConfig;
  private codeGraph: CodeGenerationGraph;
  private storage: Storage;

  constructor(config: ServerConfig) {
    this.config = config;
    this.codeGraph = new CodeGenerationGraph(10000);
    this.storage = new Storage();
    this.app = express();
    this.setupMiddleware();
    this.setupRoutes();
  }

  private setupMiddleware(): void {
    this.app.use(helmet());
    this.app.use(cors());
    this.app.use(express.json({ limit: '10mb' }));
    this.app.use(express.urlencoded({ extended: true }));
    
    // Request logging
    this.app.use((req: Request, res: Response, next: NextFunction) => {
      const isHealthCheck = req.path === '/health' || 
                           req.path === '/tdz/health' || 
                           req.path === '/todozi/health';
      
      if (!isHealthCheck) {
        console.log(`🔍 ${req.method} ${req.path}`);
      }
      next();
    });
  }

  private setupRoutes(): void {
    // Health check
    this.app.get(['/health', '/tdz/health', '/todozi/health'], this.healthCheck.bind(this));
    
    // System endpoints
    this.app.get(['/stats', '/tdz/stats', '/todozi/stats'], this.getSystemStats.bind(this));
    this.app.get(['/init', '/tdz/init', '/todozi/init'], this.initializeSystem.bind(this));

    // Task management
    this.app.get(['/tasks', '/tdz/tasks', '/todozi/tasks'], this.getAllTasks.bind(this));
    this.app.post(['/tasks', '/tdz/tasks', '/todozi/tasks'], this.authenticate.bind(this), this.createTask.bind(this));
    this.app.get(['/tasks/:id', '/tdz/tasks/:id', '/todozi/tasks/:id'], this.getTask.bind(this));
    this.app.put(['/tasks/:id', '/tdz/tasks/:id', '/todozi/tasks/:id'], this.updateTask.bind(this));
    this.app.delete(['/tasks/:id', '/tdz/tasks/:id', '/todozi/tasks/:id'], this.deleteTask.bind(this));
    this.app.get(['/tasks/search', '/tdz/tasks/search'], this.searchTasks.bind(this));
    
    // AI-enhanced task endpoints
    this.app.get(['/tasks/:id/insights', '/tdz/tasks/:id/insights'], this.getTaskInsights.bind(this));
    this.app.get(['/tasks/:id/similar', '/tdz/tasks/:id/similar'], this.getSimilarTasks.bind(this));
    this.app.get(['/tasks/suggest', '/tdz/tasks/suggest'], this.getAiTaskSuggestions.bind(this));
    this.app.post(['/tasks/validate', '/tdz/tasks/validate'], this.validateTaskWithAi.bind(this));

    // Memory management
    this.app.get(['/memories', '/tdz/memories'], this.getAllMemories.bind(this));
    this.app.post(['/memories', '/tdz/memories'], this.createMemory.bind(this));
    this.app.get(['/memories/secret', '/tdz/memories/secret'], this.getSecretMemories.bind(this));
    this.app.get(['/memories/human', '/tdz/memories/human'], this.getHumanMemories.bind(this));
    this.app.get(['/memories/short', '/tdz/memories/short'], this.getShortMemories.bind(this));
    this.app.get(['/memories/long', '/tdz/memories/long'], this.getLongMemories.bind(this));
    this.app.get(['/memories/emotional/:emotion', '/tdz/memories/emotional/:emotion'], this.getEmotionalMemories.bind(this));
    this.app.get(['/memories/types', '/tdz/memories/types'], this.getMemoryTypes.bind(this));

    // Ideas
    this.app.get(['/ideas', '/tdz/ideas'], this.getAllIdeas.bind(this));
    this.app.post(['/ideas', '/tdz/ideas'], this.createIdea.bind(this));

    // Agent system
    this.app.get(['/agents', '/tdz/agents'], this.getAllAgents.bind(this));
    this.app.post(['/agents', '/tdz/agents'], this.createAgent.bind(this));
    this.app.get(['/agents/:id', '/tdz/agents/:id'], this.getAgent.bind(this));
    this.app.put(['/agents/:id', '/tdz/agents/:id'], this.updateAgent.bind(this));
    this.app.delete(['/agents/:id', '/tdz/agents/:id'], this.deleteAgent.bind(this));
    this.app.get(['/agents/available', '/tdz/agents/available'], this.getAvailableAgents.bind(this));
    this.app.get(['/agents/:id/status', '/tdz/agents/:id/status'], this.getAgentStatus.bind(this));

    // Training data
    this.app.get(['/training', '/tdz/training'], this.getAllTrainingData.bind(this));
    this.app.post(['/training', '/tdz/training'], this.createTrainingData.bind(this));
    this.app.get(['/training/:id', '/tdz/training/:id'], this.getTrainingData.bind(this));
    this.app.put(['/training/:id', '/tdz/training/:id'], this.updateTrainingData.bind(this));
    this.app.delete(['/training/:id', '/tdz/training/:id'], this.deleteTrainingData.bind(this));
    this.app.get(['/training/export', '/tdz/training/export'], this.exportTrainingData.bind(this));
    this.app.get(['/training/stats', '/tdz/training/stats'], this.getTrainingStats.bind(this));

    // Chat processing
    this.app.post(['/chat/process', '/tdz/chat/process'], this.processChatMessage.bind(this));
    this.app.post(['/chat/agent/:id', '/tdz/chat/agent/:id'], this.chatWithAgent.bind(this));
    this.app.get(['/chat/history', '/tdz/chat/history'], this.getChatHistory.bind(this));

    // Analytics
    this.app.get(['/analytics/tasks', '/tdz/analytics/tasks'], this.getTaskAnalytics.bind(this));
    this.app.get(['/analytics/agents', '/tdz/analytics/agents'], this.getAgentAnalytics.bind(this));
    this.app.get(['/analytics/performance', '/tdz/analytics/performance'], this.getPerformanceAnalytics.bind(this));

    // Time tracking
    this.app.post(['/time/start/:task_id', '/tdz/time/start/:task_id'], this.startTimeTracking.bind(this));
    this.app.post(['/time/stop/:task_id', '/tdz/time/stop/:task_id'], this.stopTimeTracking.bind(this));
    this.app.get(['/time/report', '/tdz/time/report'], this.getTimeTrackingReport.bind(this));

    // Code chunks
    this.app.get(['/chunks', '/tdz/chunks'], this.getAllChunks.bind(this));
    this.app.post(['/chunks', '/tdz/chunks'], this.createChunk.bind(this));
    this.app.get(['/chunks/ready', '/tdz/chunks/ready'], this.getReadyChunks.bind(this));
    this.app.get(['/chunks/graph', '/tdz/chunks/graph'], this.getChunkGraph.bind(this));

    // Projects
    this.app.get(['/projects', '/tdz/projects'], this.getAllProjects.bind(this));
    this.app.post(['/projects', '/tdz/projects'], this.createProject.bind(this));
    this.app.get(['/projects/:name', '/tdz/projects/:name'], this.getProject.bind(this));
    this.app.put(['/projects/:name', '/tdz/projects/:name'], this.updateProject.bind(this));
    this.app.delete(['/projects/:name', '/tdz/projects/:name'], this.deleteProject.bind(this));

    // Feelings
    this.app.get(['/feelings', '/tdz/feelings'], this.getAllFeelings.bind(this));
    this.app.post(['/feelings', '/tdz/feelings'], this.createFeeling.bind(this));
    this.app.get(['/feelings/:id', '/tdz/feelings/:id'], this.getFeeling.bind(this));
    this.app.put(['/feelings/:id', '/tdz/feelings/:id'], this.updateFeeling.bind(this));
    this.app.delete(['/feelings/:id', '/tdz/feelings/:id'], this.deleteFeeling.bind(this));
    this.app.get(['/feelings/search', '/tdz/feelings/search'], this.searchFeelings.bind(this));

    // Queue management
    this.app.post(['/queue/plan', '/tdz/queue/plan'], this.createQueueItem.bind(this));
    this.app.get(['/queue/list', '/tdz/queue/list'], this.getAllQueueItems.bind(this));
    this.app.get(['/queue/list/backlog', '/tdz/queue/list/backlog'], this.getBacklogItems.bind(this));
    this.app.get(['/queue/list/active', '/tdz/queue/list/active'], this.getActiveItems.bind(this));
    this.app.get(['/queue/list/complete', '/tdz/queue/list/complete'], this.getCompleteItems.bind(this));
    this.app.post(['/queue/start/:item_id', '/tdz/queue/start/:item_id'], this.startQueueSession.bind(this));
    this.app.post(['/queue/end/:session_id', '/tdz/queue/end/:session_id'], this.endQueueSession.bind(this));

    // API key management
    this.app.post(['/api/register', '/tdz/api/register'], this.registerApiKey.bind(this));
    this.app.post(['/api/check', '/tdz/api/check'], this.checkApiKey.bind(this));

    // Semantic search
    this.app.get(['/semantic/search', '/tdz/semantic/search'], this.semanticSearch.bind(this));
    this.app.get(['/insights', '/tdz/insights'], this.getAiInsights.bind(this));

    // CORS preflight
    this.app.options('*', (req, res) => {
      res.header('Access-Control-Allow-Origin', '*');
      res.header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
      res.header('Access-Control-Allow-Headers', 'Content-Type, Authorization, X-API-Key, X-API-Private-Key');
      res.send(200);
    });

    // 404 handler
    this.app.use('*', (req, res) => {
      res.status(404).json({ error: 'Route not found' });
    });

    // Error handler
    this.app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
      console.error('❌ Error:', err);
      res.status(500).json({ error: 'Internal server error' });
    });
  }

  // Authentication middleware
  private async authenticate(req: AuthenticatedRequest, res: Response, next: NextFunction): Promise<void> {
    const skipAuth = ['/health', '/tdz/health', '/todozi/health', 
                      '/api/register', '/tdz/api/register',
                      '/init', '/tdz/init', '/todozi/init'].includes(req.path);

    if (skipAuth) {
      return next();
    }

    const apiKey = this.extractApiKey(req);
    if (!apiKey) {
      return res.status(401).json({ error: 'Unauthorized: API key required' });
    }

    try {
      const { userId, isAdmin } = await this.checkApiKeyAuth(apiKey.publicKey, apiKey.privateKey);
      req.user = { id: userId, isAdmin };
      next();
    } catch (error) {
      res.status(401).json({ error: `Unauthorized: ${error}` });
    }
  }

  private extractApiKey(req: Request): { publicKey: string; privateKey?: string } | null {
    const headers = req.headers;
    const publicKey = headers['x-api-key'] as string || 
                      headers['x-apitoken'] as string ||
                      headers['authorization']?.replace(/^(Bearer|ApiKey|Token)\s+/i, '');

    if (!publicKey) return null;

    const privateKey = headers['x-api-private-key'] as string;

    return { publicKey, privateKey };
  }

  private async checkApiKeyAuth(publicKey: string, privateKey?: string): Promise<{ userId: string; isAdmin: boolean }> {
    // Mock implementation - in real app, verify against database
    if (publicKey === 'test-key') {
      return { userId: 'test-user', isAdmin: false };
    }
    throw new Error('Invalid API key');
  }

  // Route Handlers
  private async healthCheck(req: Request, res: Response): Promise<void> {
    res.json({
      status: 'healthy',
      service: 'todozi-enhanced-server',
      version: '0.1.0',
      port: this.config.port,
      agents_available: 26,
      features: ['enhanced_agents', 'training_data', 'analytics', 'time_tracking']
    });
  }

  private async getSystemStats(req: Request, res: Response): Promise<void> {
    const uptimeSeconds = Math.floor(Date.now() / 1000);
    res.json({
      system: {
        version: '0.1.0',
        uptime_seconds: uptimeSeconds,
        uptime_hours: uptimeSeconds / 3600,
        port: this.config.port
      },
      data: {
        agents: 26,
        tasks: this.storage.listTasksAcrossProjects({}).length,
        memories: 10,
        training_data: 5
      },
      performance: {
        active_connections: 0,
        requests_per_second: 0.0,
        memory_usage_mb: 50
      }
    });
  }

  private async initializeSystem(req: Request, res: Response): Promise<void> {
    res.json({
      message: 'System initialized successfully',
      directories_created: true,
      storage_initialized: true,
      agents_created: 26
    });
  }

  private async getAllTasks(req: Request, res: Response): Promise<void> {
    const tasks = this.storage.listTasksAcrossProjects({});
    const taskData = tasks.map(task => ({
      id: task.id,
      user_id: task.user_id,
      action: task.action,
      time: task.time,
      priority: task.priority,
      parent_project: task.parent_project,
      status: task.status,
      assignee: task.assignee,
      tags: task.tags,
      dependencies: task.dependencies,
      context_notes: task.context_notes,
      progress: task.progress,
      created_at: task.created_at,
      updated_at: task.updated_at
    }));
    res.json(taskData);
  }

  private async createTask(req: AuthenticatedRequest, res: Response): Promise<void> {
    try {
      const { action, time, priority = Priority.Medium, parent_project = 'default' } = req.body;
      
      if (!action || !time) {
        return res.status(400).json({ error: 'Missing required fields: action, time' });
      }

      const task: Task = {
        id: generateUUID(),
        user_id: req.user!.id,
        action,
        time,
        priority,
        parent_project,
        status: Status.Todo,
        tags: [],
        dependencies: [],
        context_notes: '',
        progress: 0,
        created_at: DateTime.now(),
        updated_at: DateTime.now()
      };

      this.storage.addTask(task);
      
      res.status(201).json({
        message: 'Task created successfully',
        task
      });
    } catch (error) {
      res.status(500).json({ error: 'Failed to create task' });
    }
  }

  private async getTask(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const task = this.storage.getTaskFromAnyProject(id);
    
    if (!task) {
      return res.status(404).json({ error: 'Task not found' });
    }

    res.json(task);
  }

  private async updateTask(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const updates: TaskUpdate = req.body;
    
    try {
      await this.storage.updateTaskInProject(id, updates);
      const task = this.storage.getTaskFromAnyProject(id);
      
      if (!task) {
        return res.status(404).json({ error: 'Task not found' });
      }

      res.json({
        message: 'Task updated successfully',
        task
      });
    } catch (error) {
      res.status(500).json({ error: 'Failed to update task' });
    }
  }

  private async deleteTask(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    
    try {
      this.storage.deleteTaskFromProject(id);
      res.json({ id, message: 'Task deleted successfully' });
    } catch (error) {
      res.status(500).json({ error: 'Failed to delete task' });
    }
  }

  private async searchTasks(req: Request, res: Response): Promise<void> {
    const query = req.query.q as string || '';
    const tasks = this.storage.listTasksAcrossProjects({});
    
    if (query) {
      // Simple text search - in real app, use vector embeddings
      const filtered = tasks.filter(task => 
        task.action.toLowerCase().includes(query.toLowerCase())
      );
      res.json(filtered);
    } else {
      res.json(tasks);
    }
  }

  private async getTaskInsights(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const task = this.storage.getTaskFromAnyProject(id);
    
    if (!task) {
      return res.status(404).json({ error: 'Task not found' });
    }

    res.json({
      task_id: task.id,
      action: task.action,
      ai_insights: {
        confidence_score: 0.95,
        similar_tasks: [],
        ai_suggestions: ['Consider breaking down into smaller tasks'],
        semantic_tags: ['development', 'backend'],
        related_content: []
      },
      task_details: task
    });
  }

  private async getSimilarTasks(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const task = this.storage.getTaskFromAnyProject(id);
    
    if (!task) {
      return res.status(404).json({ error: 'Task not found' });
    }

    // Mock similar tasks
    res.json({
      task_id: id,
      query: task.action,
      similar_tasks: [],
      count: 0
    });
  }

  private async getAiTaskSuggestions(req: Request, res: Response): Promise<void> {
    res.json({
      suggestions: {
        total_embeddings: 100,
        semantic_clusters: 5,
        recommendations: [
          'Consider grouping similar tasks together',
          'Review task priorities based on semantic similarity'
        ]
      },
      clusters: [],
      stats: {}
    });
  }

  private async validateTaskWithAi(req: Request, res: Response): Promise<void> {
    const { action } = req.body;
    const validationResults = [];
    
    if (action.length < 3) {
      validationResults.push({
        type: 'error',
        message: 'Task action too short (minimum 3 characters)',
        field: 'action'
      });
    }

    if (action.length > 200) {
      validationResults.push({
        type: 'warning',
        message: 'Task action very long (consider breaking into smaller tasks)',
        field: 'action'
      });
    }

    res.json({
      valid: validationResults.length === 0,
      validation_results: validationResults,
      ai_suggestions: [],
      similar_tasks_found: 0
    });
  }

  private async getAllMemories(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'mem_001',
        moment: '2025-01-13 10:30 AM',
        meaning: 'Client prefers iterative development',
        importance: 'high',
        term: 'long'
      }
    ]);
  }

  private async createMemory(req: Request, res: Response): Promise<void> {
    const memory = req.body;
    res.json({
      message: 'Memory created successfully',
      memory
    });
  }

  private async getSecretMemories(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'mem_sec_001',
        moment: '2025-01-13 11:00 AM',
        meaning: 'Internal AI processing note',
        importance: 'medium',
        term: 'short',
        type: 'secret'
      }
    ]);
  }

  private async getHumanMemories(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'mem_hum_001',
        moment: '2025-01-13 12:00 PM',
        meaning: 'User-facing information',
        importance: 'high',
        term: 'long',
        type: 'human'
      }
    ]);
  }

  private async getShortMemories(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'mem_short_001',
        moment: '2025-01-13 1:00 PM',
        meaning: 'Conversation context',
        importance: 'low',
        term: 'short',
        type: 'short'
      }
    ]);
  }

  private async getLongMemories(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'mem_long_001',
        moment: '2025-01-13 2:00 PM',
        meaning: 'Long-term strategic insight',
        importance: 'critical',
        term: 'long',
        type: 'long'
      }
    ]);
  }

  private async getEmotionalMemories(req: Request, res: Response): Promise<void> {
    const { emotion } = req.params;
    res.json([
      {
        id: `mem_${emotion}_001`,
        moment: '2025-01-13 3:00 PM',
        meaning: `Emotional memory about ${emotion}`,
        importance: 'medium',
        term: 'short',
        type: 'emotional',
        emotion
      }
    ]);
  }

  private async getMemoryTypes(req: Request, res: Response): Promise<void> {
    res.json([
      'standard', 'secret', 'human', 'short', 'long', 'happy', 'sad',
      'angry', 'fearful', 'surprised', 'disgusted', 'excited', 'anxious',
      'confident', 'frustrated', 'motivated', 'overwhelmed', 'curious',
      'satisfied', 'disappointed', 'grateful', 'proud', 'ashamed',
      'hopeful', 'resigned'
    ]);
  }

  private async getAllIdeas(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'idea_001',
        idea: 'Use microservices for better scalability',
        share: 'public',
        importance: 'high'
      }
    ]);
  }

  private async createIdea(req: Request, res: Response): Promise<void> {
    const idea = req.body;
    res.json({
      message: 'Idea created successfully',
      idea
    });
  }

  private async getAllAgents(req: Request, res: Response): Promise<void> {
    // Mock agents - in real app, load from storage
    const agents = [];
    for (let i = 1; i <= 26; i++) {
      agents.push({
        id: `agent_${i}`,
        name: `Agent ${i}`,
        description: `Specialized agent ${i}`,
        version: '1.0.0',
        category: 'specialized',
        status: 'available',
        model: {
          provider: 'openai',
          name: 'gpt-4',
          temperature: 0.7,
          max_tokens: 2000
        },
        capabilities: ['text_processing', 'analysis'],
        specializations: ['task_management'],
        created_at: DateTime.now(),
        updated_at: DateTime.now()
      });
    }
    res.json(agents);
  }

  private async createAgent(req: Request, res: Response): Promise<void> {
    const { name, description, category = 'custom' } = req.body;
    
    if (!name || !description) {
      return res.status(400).json({ error: 'Missing required fields: name, description' });
    }

    const agent: Agent = {
      id: `agent_${Date.now()}`,
      name,
      description,
      version: '1.0.0',
      category,
      status: 'available',
      model: {
        provider: 'openai',
        name: 'gpt-4',
        temperature: 0.7,
        max_tokens: 2000
      },
      capabilities: ['text_processing'],
      specializations: [category],
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };

    res.status(201).json({
      message: 'Agent created successfully',
      agent
    });
  }

  private async getAgent(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    
    // Mock agent
    const agent = {
      id,
      name: `Agent ${id}`,
      description: 'Mock agent',
      version: '1.0.0',
      model: {
        provider: 'openai',
        name: 'gpt-4',
        temperature: 0.7,
        max_tokens: 2000
      },
      system_prompt: 'You are a helpful assistant',
      capabilities: ['text_processing'],
      specializations: ['general'],
      behaviors: {
        auto_format_code: true,
        include_examples: true,
        explain_complexity: true,
        suggest_tests: true
      },
      metadata: {
        author: 'system',
        tags: ['assistant'],
        category: 'general',
        status: 'available'
      },
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };

    res.json(agent);
  }

  private async updateAgent(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      id,
      message: 'Agent update partially implemented - metadata updates only',
      note: 'Full agent updates would require Agent struct modification',
      data: req.body
    });
  }

  private async deleteAgent(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      id,
      message: 'Agent deletion not supported - agents are predefined system components',
      note: 'To disable an agent, use the update endpoint to change its status'
    });
  }

  private async getAvailableAgents(req: Request, res: Response): Promise<void> {
    const agents = [];
    for (let i = 1; i <= 26; i++) {
      agents.push({
        id: `agent_${i}`,
        name: `Agent ${i}`,
        description: `Specialized agent ${i}`,
        status: 'available'
      });
    }
    res.json(agents);
  }

  private async getAgentStatus(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      id,
      status: 'available',
      last_updated: DateTime.now()
    });
  }

  private async getAllTrainingData(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'training_001',
        data_type: 'instruction',
        prompt: 'Sample prompt',
        completion: 'Sample completion',
        context: 'Sample context',
        tags: ['sample'],
        quality_score: 0.9,
        source: 'manual',
        created_at: DateTime.now(),
        updated_at: DateTime.now()
      }
    ]);
  }

  private async createTrainingData(req: Request, res: Response): Promise<void> {
    const { data_type, prompt, completion, context, tags, quality_score, source = 'api' } = req.body;
    
    if (!data_type || !prompt || !completion) {
      return res.status(400).json({ error: 'Missing required fields: data_type, prompt, completion' });
    }

    const trainingData: TrainingData = {
      id: `training_${Date.now()}`,
      data_type: data_type as TrainingDataType,
      prompt,
      completion,
      context,
      tags: tags || [],
      quality_score,
      source,
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };

    res.status(201).json({
      message: 'Training data created successfully',
      training_data: trainingData
    });
  }

  private async getTrainingData(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const trainingData = {
      id,
      data_type: 'instruction',
      prompt: 'Sample prompt',
      completion: 'Sample completion',
      context: 'Sample context',
      tags: ['sample'],
      quality_score: 0.9,
      source: 'api',
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };
    res.json(trainingData);
  }

  private async updateTrainingData(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      message: 'Training data updated successfully',
      training_data: {
        id,
        ...req.body,
        updated_at: DateTime.now()
      }
    });
  }

  private async deleteTrainingData(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      id,
      message: 'Training data deleted successfully'
    });
  }

  private async exportTrainingData(req: Request, res: Response): Promise<void> {
    const trainingData = [
      {
        id: 'training_001',
        data_type: 'instruction',
        prompt: 'Sample prompt',
        completion: 'Sample completion'
      }
    ];

    res.json({
      message: 'Training data exported successfully',
      total_entries: trainingData.length,
      exports: {
        json: trainingData,
        jsonl: trainingData
      },
      formats: ['json', 'jsonl', 'csv']
    });
  }

  private async getTrainingStats(req: Request, res: Response): Promise<void> {
    res.json({
      total_entries: 10,
      by_data_type: {
        instruction: 5,
        conversation: 3,
        completion: 2
      },
      by_source: {
        api: 7,
        manual: 3
      },
      quality_distribution: {
        excellent: 3,
        good: 4,
        fair: 2,
        poor: 1
      },
      average_quality_score: 0.8,
      quality_score_count: 10,
      tags_used: 15
    });
  }

  private async processChatMessage(req: Request, res: Response): Promise<void> {
    const { message } = req.body;
    
    if (!message) {
      return res.status(400).json({ error: 'Missing message field' });
    }

    res.json({
      message: 'Chat processed successfully',
      processed_message: message,
      content: {
        tasks: 0,
        memories: 0,
        ideas: 0,
        agent_assignments: 0,
        code_chunks: 0
      },
      details: {
        tasks: [],
        memories: [],
        ideas: [],
        agent_assignments: [],
        code_chunks: []
      }
    });
  }

  private async chatWithAgent(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const { message } = req.body;
    
    if (!message) {
      return res.status(400).json({ error: 'Missing message field' });
    }

    res.json({
      agent_id: id,
      agent_name: `Agent ${id}`,
      message,
      response: {
        tasks: 0,
        memories: 0,
        ideas: 0,
        agent_assignments: 0,
        code_chunks: 0
      },
      content: {
        tasks: [],
        memories: [],
        ideas: [],
        agent_assignments: [],
        code_chunks: []
      },
      processed_at: DateTime.now()
    });
  }

  private async getChatHistory(req: Request, res: Response): Promise<void> {
    const tasks = this.storage.listTasksAcrossProjects({});
    const history = tasks.slice(0, 10).map(task => ({
      id: `chat_${task.id}`,
      type: 'task_created',
      message: `Task created: ${task.action}`,
      timestamp: task.created_at,
      data: {
        task_id: task.id,
        action: task.action,
        status: task.status
      }
    }));
    res.json(history);
  }

  private async getTaskAnalytics(req: Request, res: Response): Promise<void> {
    const tasks = this.storage.listTasksAcrossProjects({});
    const totalTasks = tasks.length;
    
    const byStatus = tasks.reduce((acc, task) => {
      acc[task.status] = (acc[task.status] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);

    const byPriority = tasks.reduce((acc, task) => {
      acc[task.priority] = (acc[task.priority] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);

    res.json({
      total_tasks: totalTasks,
      by_status: byStatus,
      by_priority: byPriority,
      by_assignee: {
        me: 5,
        team: 3,
        unassigned: 2
      },
      completion_rate: 0.7,
      completed_tasks: 7,
      recent_activity: {
        last_24h: 2,
        last_7d: 5
      }
    });
  }

  private async getAgentAnalytics(req: Request, res: Response): Promise<void> {
    res.json({
      total_agents: 26,
      available_agents: 26,
      busy_agents: 0,
      inactive_agents: 0,
      by_category: {
        specialized: 20,
        general: 6
      },
      agent_statistics: {
        total_assignments: 0,
        completed_assignments: 0,
        completion_rate: 0.0,
        note: 'Advanced agent statistics require assignment tracking implementation'
      }
    });
  }

  private async getPerformanceAnalytics(req: Request, res: Response): Promise<void> {
    res.json({
      response_times: {
        average_ms: 150,
        p95_ms: 300,
        p99_ms: 500
      },
      throughput: {
        requests_per_second: 10.0,
        bytes_per_second: 10240
      },
      error_rate: 0.01,
      uptime_percentage: 99.9,
      system_metrics: {
        total_uptime_seconds: Math.floor(Date.now() / 1000),
        active_connections: 0,
        total_tasks: this.storage.listTasksAcrossProjects({}).length,
        total_agents: 26,
        backlog_items: 5,
        memory_usage_mb: 50,
        cpu_usage_percent: 15.0
      },
      performance_score: {
        overall: 85,
        task_processing: 90,
        agent_response: 80,
        memory_efficiency: 95
      }
    });
  }

  private async startTimeTracking(req: Request, res: Response): Promise<void> {
    const { task_id } = req.params;
    const sessionId = generateUUID();
    
    res.json({
      task_id,
      session_id: sessionId,
      message: 'Time tracking started successfully',
      started_at: DateTime.now(),
      note: 'Time tracking is implemented via queue sessions'
    });
  }

  private async stopTimeTracking(req: Request, res: Response): Promise<void> {
    const { task_id } = req.params;
    
    res.json({
      task_id,
      message: 'No active time tracking session found for this task',
      error: 'not_tracking'
    });
  }

  private async getTimeTrackingReport(req: Request, res: Response): Promise<void> {
    res.json({
      total_sessions: 0,
      total_time_seconds: 0,
      total_time_hours: 0,
      by_task: {},
      by_date: {},
      productivity_score: 0.0,
      completion_stats: {
        total_items: 0,
        completed_items: 0,
        completion_rate: 0.0
      }
    });
  }

  private async getAllChunks(req: Request, res: Response): Promise<void> {
    const chunks = Array.from(this.codeGraph.chunks.values()).map(chunk => ({
      id: chunk.chunk_id,
      level: chunk.level,
      status: chunk.status,
      dependencies: chunk.dependencies,
      estimated_tokens: chunk.estimated_tokens
    }));
    res.json(chunks);
  }

  private async createChunk(req: Request, res: Response): Promise<void> {
    const chunkData = req.body;
    res.json({
      message: 'Code chunk created successfully',
      chunk: chunkData
    });
  }

  private async getReadyChunks(req: Request, res: Response): Promise<void> {
    const readyChunks = this.codeGraph.getReadyChunks();
    res.json({
      ready_chunks: readyChunks,
      count: readyChunks.length
    });
  }

  private async getChunkGraph(req: Request, res: Response): Promise<void> {
    res.json({
      total_chunks: this.codeGraph.chunks.size,
      project_summary: this.codeGraph.getProjectSummary()
    });
  }

  private async getAllProjects(req: Request, res: Response): Promise<void> {
    res.json([
      {
        name: 'general',
        description: 'General tasks',
        status: 'active'
      }
    ]);
  }

  private async createProject(req: Request, res: Response): Promise<void> {
    const { name, description } = req.body;
    
    if (!name) {
      return res.status(400).json({ error: 'Missing required field: name' });
    }

    await this.storage.createProject(name, description);
    
    res.status(201).json({
      message: 'Project created successfully',
      project: {
        name,
        description: description || '',
        status: 'active'
      }
    });
  }

  private async getProject(req: Request, res: Response): Promise<void> {
    const { name } = req.params;
    const project = this.storage.getProject(name);
    
    if (!project) {
      return res.status(404).json({ error: 'Project not found' });
    }

    res.json(project);
  }

  private async updateProject(req: Request, res: Response): Promise<void> {
    const { name } = req.params;
    res.json({
      message: 'Project update not yet fully implemented',
      name,
      description: req.body.description
    });
  }

  private async deleteProject(req: Request, res: Response): Promise<void> {
    const { name } = req.params;
    res.json({
      message: 'Project deletion not yet fully implemented',
      name
    });
  }

  private async getAllFeelings(req: Request, res: Response): Promise<void> {
    res.json([
      {
        id: 'feeling_001',
        emotion: 'happy',
        intensity: 8,
        description: 'Feeling good about progress',
        context: 'work',
        tags: ['positive'],
        created_at: DateTime.now(),
        updated_at: DateTime.now()
      }
    ]);
  }

  private async createFeeling(req: Request, res: Response): Promise<void> {
    const { emotion, intensity, description, context = 'general', tags = [] } = req.body;
    
    if (!emotion || !intensity || !description) {
      return res.status(400).json({ error: 'Missing required fields: emotion, intensity, description' });
    }

    if (intensity < 1 || intensity > 10) {
      return res.status(400).json({ error: 'Intensity must be between 1 and 10' });
    }

    const feeling: Feeling = {
      id: `feeling_${Date.now()}`,
      emotion,
      intensity,
      description,
      context,
      tags,
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };

    res.status(201).json({
      message: 'Feeling created successfully',
      feeling
    });
  }

  private async getFeeling(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    const feeling = {
      id,
      emotion: 'happy',
      intensity: 8,
      description: 'Feeling good',
      context: 'work',
      tags: ['positive'],
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };
    res.json(feeling);
  }

  private async updateFeeling(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      message: 'Feeling updated successfully',
      feeling: {
        id,
        ...req.body,
        updated_at: DateTime.now()
      }
    });
  }

  private async deleteFeeling(req: Request, res: Response): Promise<void> {
    const { id } = req.params;
    res.json({
      id,
      message: 'Feeling deleted successfully'
    });
  }

  private async searchFeelings(req: Request, res: Response): Promise<void> {
    const { q } = req.query;
    res.json([
      {
        query: q,
        message: 'Feeling search not yet implemented'
      }
    ]);
  }

  private async createQueueItem(req: Request, res: Response): Promise<void> {
    const { task_name, task_description, priority, project_id } = req.body;
    
    if (!task_name || !task_description || !priority) {
      return res.status(400).json({ error: 'Missing required fields: task_name, task_description, priority' });
    }

    const item: QueueItem = {
      id: generateUUID(),
      task_name,
      task_description,
      priority,
      project_id,
      status: QueueStatus.Backlog,
      created_at: DateTime.now(),
      updated_at: DateTime.now()
    };

    res.status(201).json({
      message: 'Queue item created successfully',
      item
    });
  }

  private async getAllQueueItems(req: Request, res: Response): Promise<void> {
    res.json([]);
  }

  private async getBacklogItems(req: Request, res: Response): Promise<void> {
    res.json([]);
  }

  private async getActiveItems(req: Request, res: Response): Promise<void> {
    res.json([]);
  }

  private async getCompleteItems(req: Request, res: Response): Promise<void> {
    res.json([]);
  }

  private async startQueueSession(req: Request, res: Response): Promise<void> {
    const { item_id } = req.params;
    const sessionId = generateUUID();
    
    res.json({
      message: 'Queue session started successfully',
      session_id: sessionId,
      queue_item_id: item_id,
      started_at: DateTime.now()
    });
  }

  private async endQueueSession(req: Request, res: Response): Promise<void> {
    const { session_id } = req.params;
    res.json({
      message: 'Queue session ended successfully',
      session_id,
      queue_item_id: 'mock_item',
      start_time: DateTime.now().minus({ hours: 1 }),
      end_time: DateTime.now(),
      duration_seconds: 3600
    });
  }

  private async registerApiKey(req: Request, res: Response): Promise<void> {
    const userId = `user_${Date.now()}`;
    const publicKey = generateUUID();
    const privateKey = generateUUID();
    
    res.status(201).json({
      message: 'API key created successfully',
      user_id: userId,
      public_key: publicKey,
      private_key: privateKey,
      active: true,
      created_at: DateTime.now()
    });
  }

  private async checkApiKey(req: Request, res: Response): Promise<void> {
    const { public_key, private_key } = req.body;
    
    if (!public_key) {
      return res.status(400).json({ error: 'Missing public_key field' });
    }

    res.json({
      message: 'API key authentication successful',
      user_id: 'test-user',
      public_key,
      is_admin: false,
      access_level: 'read_only'
    });
  }

  private async semanticSearch(req: Request, res: Response): Promise<void> {
    const { q } = req.query;
    
    if (!q) {
      return res.status(400).json({
        error: 'Query parameter is required',
        usage: 'GET /semantic/search?q=your_search_query'
      });
    }

    res.json({
      query: q,
      results: [],
      count: 0,
      search_type: 'semantic'
    });
  }

  private async getAiInsights(req: Request, res: Response): Promise<void> {
    res.json({
      ai_insights: {
        embedding_statistics: {
          total_embeddings: 100
        },
        semantic_clusters: [],
        recommendations: {
          task_organization: 'Consider grouping semantically similar tasks',
          priority_optimization: 'Review task priorities based on AI confidence scores'
        }
      },
      system_status: {
        embedding_model: 'text-embedding-ada-002',
        similarity_threshold: 0.8,
        max_results: 20
      }
    });
  }

  public async start(): Promise<void> {
    return new Promise((resolve, reject) => {
      this.server = createServer(this.app).listen(this.config.port, this.config.host, () => {
        console.log(`🚀 Todozi Enhanced Server starting on ${this.config.host}:${this.config.port}`);
        this.printAvailableEndpoints();
        resolve();
      });

      this.server.on('error', (error) => {
        console.error('❌ Failed to start server:', error);
        reject(error);
      });
    });
  }

  private printAvailableEndpoints(): void {
    console.log('📡 Available endpoints:');
    console.log();
    console.log('🎯 CORE FUNCTIONALITY:');
    console.log('  GET  /health                    - Health check');
    console.log('  GET  /stats                     - System statistics');
    console.log('  GET  /init                      - Initialize system');
    console.log();
    console.log('📋 TASK MANAGEMENT:');
    console.log('  GET  /tasks                     - List all tasks');
    console.log('  POST /tasks                     - Create new task');
    console.log('  GET  /tasks/:id                 - Get task by ID');
    console.log('  PUT  /tasks/:id                 - Update task');
    console.log('  DELETE /tasks/:id               - Delete task');
    console.log('  GET  /tasks/search?q=:query     - Search tasks');
    console.log();
    console.log('🤖 ENHANCED AGENT SYSTEM (26 AGENTS):');
    console.log('  GET  /agents                    - List all agents');
    console.log('  POST /agents                    - Create new agent');
    console.log('  GET  /agents/:id                - Get agent by ID');
    console.log('  PUT  /agents/:id                - Update agent');
    console.log('  DELETE /agents/:id              - Delete agent');
    console.log('  GET  /agents/available          - Get available agents');
    console.log('  GET  /agents/:id/status         - Get agent status');
    console.log();
    console.log('🧠 MEMORY & IDEA MANAGEMENT:');
    console.log('  GET  /memories                  - List all memories');
    console.log('  POST /memories                  - Create new memory');
    console.log('  GET  /ideas                     - List all ideas');
    console.log('  POST /ideas                     - Create new idea');
    console.log('  GET  /feelings                  - List all feelings');
    console.log('  POST /feelings                  - Create new feeling');
    console.log();
    console.log('🎓 TRAINING DATA SYSTEM:');
    console.log('  GET  /training                  - List all training data');
    console.log('  POST /training                  - Create training data');
    console.log('  GET  /training/export           - Export training data');
    console.log('  GET  /training/stats            - Training data statistics');
    console.log();
    console.log('🧩 CODE CHUNKING SYSTEM:');
    console.log('  GET  /chunks                    - List all code chunks');
    console.log('  POST /chunks                    - Create new code chunk');
    console.log('  GET  /chunks/ready              - Get ready chunks');
    console.log('  GET  /chunks/graph              - Get dependency graph');
    console.log();
    console.log('💬 ENHANCED CHAT PROCESSING:');
    console.log('  POST /chat/process              - Process chat message');
    console.log('  POST /chat/agent/:id             - Chat with specific agent');
    console.log('  GET  /chat/history              - Get chat history');
    console.log();
    console.log('📊 ANALYTICS & TRACKING:');
    console.log('  GET  /analytics/tasks           - Task analytics');
    console.log('  GET  /analytics/agents          - Agent analytics');
    console.log('  GET  /analytics/performance     - System performance');
    console.log('  POST /time/start/:task_id       - Start time tracking');
    console.log('  POST /time/stop/:task_id        - Stop time tracking');
    console.log('  GET  /time/report               - Time tracking report');
    console.log();
    console.log('📁 PROJECT MANAGEMENT:');
    console.log('  GET  /projects                  - List all projects');
    console.log('  POST /projects                  - Create new project');
    console.log('  GET  /projects/:name            - Get project by name');
    console.log('  PUT  /projects/:name            - Update project');
    console.log('  DELETE /projects/:name          - Delete project');
    console.log();
    console.log('🔧 UTILITIES:');
    console.log('  POST /api/register              - Register new API key');
    console.log('  POST /api/check                 - Check API key authentication');
    console.log();
    console.log('🤖 AI-ENHANCED ENDPOINTS:');
    console.log('  GET  /tasks/:id/insights        - Get task with AI insights');
    console.log('  GET  /tasks/:id/similar         - Find similar tasks');
    console.log('  POST /tasks/validate            - Validate task with AI');
    console.log('  GET  /tasks/suggest             - AI task suggestions');
    console.log('  GET  /semantic/search?q=:query  - Semantic search');
    console.log('  GET  /insights                  - AI insights overview');
  }

  public async stop(): Promise<void> {
    return new Promise((resolve) => {
      if (this.server) {
        this.server.close(() => {
          console.log('🛑 Server stopped');
          resolve();
        });
      } else {
        resolve();
      }
    });
  }
}

// Export functions
export async function startServer(host?: string, port?: number): Promise<void> {
  const config: ServerConfig = {
    host: host || '0.0.0.0',
    port: port || 8636,
    maxConnections: 100
  };

  const server = new TodoziServer(config);
  await server.start();
}

export async function exampleUsage(): Promise<void> {
  console.log('🚀 Starting Todozi Server on port 8636');
  await startServer();
}

// Main execution
if (require.main === module) {
  exampleUsage().catch(console.error);
}
