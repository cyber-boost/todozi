import { config } from './config.js';
import { logger } from './logger.js';
import { fileURLToPath, pathToFileURL } from 'url';
import path from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import Todozi SDK (ES modules) - lazy loaded
const todoziPath = path.resolve(__dirname, '../../todozi');

let Storage, initStorage, TodoziHandler, processChatMessageExtended, Task, Priority, Status, Assignee, MemoryManager, IdeaManager;

async function loadTodoziModules() {
  if (Storage) return; // Already loaded
  
  const storageModule = await import(pathToFileURL(path.join(todoziPath, 'storage.js')).href);
  Storage = storageModule.Storage;
  initStorage = storageModule.initStorage;
  
  const cliModule = await import(pathToFileURL(path.join(todoziPath, 'cli.js')).href);
  TodoziHandler = cliModule.TodoziHandler;
  
  const todoziModule = await import(pathToFileURL(path.join(todoziPath, 'todozi.js')).href);
  processChatMessageExtended = todoziModule.processChatMessageExtended;
  
  const modelsModule = await import(pathToFileURL(path.join(todoziPath, 'models.js')).href);
  Task = modelsModule.Task;
  Priority = modelsModule.Priority;
  Status = modelsModule.Status;
  Assignee = modelsModule.Assignee;
  
  const memoryModule = await import(pathToFileURL(path.join(todoziPath, 'memory.js')).href);
  MemoryManager = memoryModule.MemoryManager;
  
  const ideaModule = await import(pathToFileURL(path.join(todoziPath, 'idea.js')).href);
  IdeaManager = ideaModule.IdeaManager;
}

// Initialize Todozi SDK
let storage = null;
let handler = null;
let memoryManager = null;
let ideaManager = null;
let initialized = false;

async function ensureInitialized() {
  if (initialized && storage && handler) {
    return;
  }

  try {
    // Load Todozi modules if not already loaded
    await loadTodoziModules();
    
    // Initialize storage directory structure
    await initStorage();
    
    // Create storage instance
    storage = await Storage.new();
    
    // Initialize handler
    handler = await TodoziHandler.new(storage);
    
    // Initialize managers
    memoryManager = new MemoryManager();
    ideaManager = new IdeaManager();
    
    initialized = true;
    logger.info('✅ Todozi SDK initialized successfully');
  } catch (error) {
    logger.error(`Failed to initialize Todozi SDK: ${error.message}`, error);
    throw error;
  }
}

function parseTagsFromText(text) {
  // Extract Todozi tags from text
  const tags = [];
  const re = /<(\w+)>(.*?)<\/\1>/gs;
  let m;
  while ((m = re.exec(text)) !== null) {
    tags.push(m[1]);
  }
  return [...new Set(tags)];
}

export const todoziService = {
  isConfigured: () => true, // Always configured when using SDK

  async tdz_cnt(content, sessionId) {
    await ensureInitialized();
    
    try {
      // Use processChatMessageExtended to parse content
      const userId = sessionId || 'chat_user';
      const parsed = processChatMessageExtended(content, userId);
      
      // Save parsed content to storage
      const created = [];
      
      // Create tasks
      for (const task of parsed.tasks || []) {
        try {
          const createdTask = await handler.handleAddCommand({
            type: 'Task',
            action: task.action || '',
            time: task.time || '2 hours',
            priority: task.priority || Priority.Medium,
            project: task.parent_project || task.project || 'general',
            status: task.status || Status.Todo,
            assignee: task.assignee || Assignee.Collaborative,
            tags: task.tags ? (Array.isArray(task.tags) ? task.tags.join(',') : task.tags) : '',
            context: task.context_notes || task.context || null
          });
          created.push(createdTask);
        } catch (e) {
          logger.warn(`Failed to create task: ${e.message}`);
        }
      }
      
      // Create memories
      for (const memory of parsed.memories || []) {
        try {
          const tags = memory.tags ? (Array.isArray(memory.tags) ? memory.tags : memory.tags.split(',').map(s => s.trim())) : [];
          await memoryManager.createMemory({
            moment: memory.moment || '',
            meaning: memory.meaning || '',
            reason: memory.reason || '',
            importance: memory.importance || 'medium',
            term: memory.term || 'long',
            type: memory.type || 'standard',
            tags: tags
          });
        } catch (e) {
          logger.warn(`Failed to create memory: ${e.message}`);
        }
      }
      
      // Create ideas
      for (const idea of parsed.ideas || []) {
        try {
          const tags = idea.tags ? (Array.isArray(idea.tags) ? idea.tags : idea.tags.split(',').map(s => s.trim())) : [];
          await ideaManager.createIdea({
            idea: idea.idea || '',
            share: idea.share || 'team',
            importance: idea.importance || 'medium',
            tags: tags,
            context: idea.context || null
          });
        } catch (e) {
          logger.warn(`Failed to create idea: ${e.message}`);
        }
      }
      
      return {
        process: 'success',
        created_tasks: created.map(t => t?.action || t?.id || 'task'),
        clean: content,
        tags: parseTagsFromText(content),
        parsed_content: parsed
      };
    } catch (e) {
      logger.error(`tdz_cnt failed: ${e.message}`, e);
      return { 
        process: 'error', 
        error: e.message, 
        clean: content,
        tags: parseTagsFromText(content)
      };
    }
  },

  async list() {
    await ensureInitialized();
    
    try {
      // Use storage directly to get tasks
      const tasks = await storage.listTasksAcrossProjects({});
      return tasks || [];
    } catch (e) {
      logger.error(`Failed to list tasks: ${e.message}`);
      return [];
    }
  },

  async stats() {
    await ensureInitialized();
    
    try {
      // Get stats directly from storage
      const allTasks = await storage.listTasksAcrossProjects({});
      const activeTasks = await storage.listTasksAcrossProjects({ status: Status.Todo });
      const completedTasks = await storage.listTasksAcrossProjects({ status: Status.Done });
      
      const total = allTasks.length;
      const active = activeTasks.length;
      const completed = completedTasks.length;
      
      return {
        total_tasks: total,
        active_tasks: active,
        completed_tasks: completed,
        stats_string: `Total: ${total}, Active: ${active}, Done: ${completed}`
      };
    } catch (e) {
      logger.error(`Failed to get stats: ${e.message}`);
      return {
        total_tasks: 0,
        active_tasks: 0,
        completed_tasks: 0,
        stats_string: 'Total: 0, Active: 0, Done: 0'
      };
    }
  },

  async find(query) {
    await ensureInitialized();
    
    try {
      // Use storage search method
      const tasks = await storage.searchTasks(query);
      return tasks || [];
    } catch (e) {
      logger.error(`Failed to search tasks: ${e.message}`);
      return [];
    }
  },

  async ai_find(query) {
    await ensureInitialized();
    
    try {
      // Use semantic search from storage
      const tasks = await storage.searchTasksSemantic(query, 20);
      // Extract task objects from similarity results
      return tasks.map(result => result.task || result).filter(Boolean);
    } catch (e) {
      logger.error(`Failed to AI search tasks: ${e.message}`);
      return [];
    }
  },

  async get(id) {
    await ensureInitialized();
    
    try {
      // Use storage to get task directly
      const task = await storage.getTaskFromAnyProject(id);
      return task || null;
    } catch (e) {
      logger.error(`Failed to get task: ${e.message}`);
      return null;
    }
  },

  async complete(id) {
    await ensureInitialized();
    
    try {
      // Update task status to done
      const task = await storage.updateTaskInProject(id, { status: Status.Done });
      return task || { id, status: 'done' };
    } catch (e) {
      logger.error(`Failed to complete task: ${e.message}`);
      throw e;
    }
  },

  async delete(id) {
    await ensureInitialized();
    
    try {
      await storage.deleteTaskFromProject(id);
      return { id, deleted: true };
    } catch (e) {
      logger.error(`Failed to delete task: ${e.message}`);
      throw e;
    }
  },

  async create_task(action, priority = 'normal', project = null, timeEstimate = null, context = null) {
    await ensureInitialized();
    
    try {
      // Normalize priority
      const priorityEnum = Priority.fromStr ? Priority.fromStr(priority) : (Priority[priority.charAt(0).toUpperCase() + priority.slice(1)] || Priority.Medium);
      const statusEnum = Status.Todo;
      const assigneeEnum = Assignee.Collaborative;
      
      // Create task using Task model
      const task = Task.newFull(
        'chat_user',
        action,
        timeEstimate || '2 hours',
        priorityEnum,
        project || 'general',
        statusEnum,
        assigneeEnum,
        [], // tags
        [], // dependencies
        context || null,
        0 // progress
      );
      
      // Save to storage
      await storage.addTaskToProject(task);
      
      return task;
    } catch (e) {
      logger.error(`Failed to create task: ${e.message}`);
      throw e;
    }
  },

  async remember(moment, meaning) {
    await ensureInitialized();
    
    try {
      const memoryId = await memoryManager.createMemory({
        moment: moment,
        meaning: meaning,
        reason: 'Captured from chat',
        importance: 'medium',
        term: 'long',
        type: 'standard',
        tags: []
      });
      const memory = memoryManager.getMemory(memoryId);
      return memory;
    } catch (e) {
      logger.error(`Failed to create memory: ${e.message}`);
      throw e;
    }
  },

  async idea(text) {
    await ensureInitialized();
    
    try {
      const ideaId = await ideaManager.createIdea({
        idea: text,
        share: 'team',
        importance: 'medium',
        tags: [],
        context: null
      });
      const idea = ideaManager.getIdea(ideaId);
      return idea;
    } catch (e) {
      logger.error(`Failed to create idea: ${e.message}`);
      throw e;
    }
  },

  parseTagsFromText,
};
