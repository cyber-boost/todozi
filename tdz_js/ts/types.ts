/**
 * TypeScript translation of the Rust CLI command structure
 * This file contains all the command definitions and data structures
 * for the task management system
 */

// ===== Core Data Types =====

export interface Task {
  id: string;
  // Additional task properties would be defined here
}

export interface Memory {
  id: string;
  moment: string;
  meaning: string;
  reason: string;
  importance?: 'low' | 'medium' | 'high';
  term?: 'short' | 'medium' | 'long';
  memoryType?: 'standard' | 'secret' | 'human' | 'emotional';
  tags?: string[];
}

export interface Idea {
  id: string;
  idea: string;
  share?: 'private' | 'public' | 'team';
  importance?: 'low' | 'medium' | 'high';
  tags?: string[];
  context?: string;
}

export interface Error {
  id: string;
  title: string;
  description: string;
  severity?: 'low' | 'medium' | 'high' | 'critical';
  category?: 'runtime' | 'compile' | 'logic' | 'system';
  source: string;
  context?: string;
  tags?: string[];
  resolved?: boolean;
  resolution?: string;
}

export interface TrainingData {
  id: string;
  dataType?: 'instruction' | 'conversation' | 'code' | 'documentation';
  prompt: string;
  completion: string;
  context?: string;
  tags?: string[];
  quality?: number; // 0.0 to 1.0
  source?: 'manual' | 'generated' | 'collected';
}

export interface Feeling {
  id: string;
  // Additional feeling properties
}

export interface AgentAssignment {
  id: string;
  agentId: string;
  taskId: string;
  projectId: string;
}

export interface CodeChunk {
  id: string;
  // Additional code chunk properties
}

// ===== Command Types =====

export type Commands =
  | InitCommand
  | AddCommand
  | ListCommand
  | ShowCommand
  | UpdateCommand
  | CompleteCommand
  | FixConsistencyCommand
  | CheckStructureCommand
  | EnsureStructureCommand
  | RegisterCommand
  | RegistrationStatusCommand
  | ClearRegistrationCommand
  | DeleteCommand
  | ProjectCommand
  | SearchCommand
  | StatsCommand
  | BackupCommand
  | ListBackupsCommand
  | RestoreCommand
  | MemoryCommand
  | IdeaCommand
  | AgentCommand
  | EmbCommand
  | ErrorCommand
  | TrainCommand
  | ChatCommand
  | SearchAllCommand
  | MaestroCommand
  | ServerCommand
  | MLCommand
  | IndDemoCommand
  | QueueCommand
  | ApiCommand
  | TdzCntCommand
  | ExportEmbeddingsCommand
  | MigrateCommand
  | TuiCommand
  | ExtractCommand
  | StrategyCommand
  | StepsCommand;

// Base command interface
interface BaseCommand {
  command: string;
}

// ===== Main Commands =====

interface InitCommand extends BaseCommand {
  command: 'init';
}

interface AddCommand extends BaseCommand {
  command: 'add';
  subcommand: AddSubCommands;
}

interface ListCommand extends BaseCommand {
  command: 'list';
  subcommand: ListSubCommands;
}

interface ShowCommand extends BaseCommand {
  command: 'show';
  subcommand: ShowSubCommands;
}

interface UpdateCommand extends BaseCommand {
  command: 'update';
  id: string;
  action?: string;
  time?: string;
  priority?: 'low' | 'medium' | 'high' | 'urgent';
  project?: string;
  status?: string;
  assignee?: string;
  tags?: string;
  dependencies?: string;
  context?: string;
  progress?: number; // 0-100
}

interface CompleteCommand extends BaseCommand {
  command: 'complete';
  id: string;
}

interface FixConsistencyCommand extends BaseCommand {
  command: 'fix-consistency';
}

interface CheckStructureCommand extends BaseCommand {
  command: 'check-structure';
}

interface EnsureStructureCommand extends BaseCommand {
  command: 'ensure-structure';
}

interface RegisterCommand extends BaseCommand {
  command: 'register';
  serverUrl?: string;
}

interface RegistrationStatusCommand extends BaseCommand {
  command: 'registration-status';
}

interface ClearRegistrationCommand extends BaseCommand {
  command: 'clear-registration';
}

interface DeleteCommand extends BaseCommand {
  command: 'delete';
  id: string;
}

interface ProjectCommand extends BaseCommand {
  command: 'project';
  subcommand: ProjectSubCommands;
}

interface SearchCommand extends BaseCommand {
  command: 'search';
  subcommand: SearchSubCommands;
}

interface StatsCommand extends BaseCommand {
  command: 'stats';
  subcommand: StatsSubCommands;
}

interface BackupCommand extends BaseCommand {
  command: 'backup';
  subcommand: BackupSubCommands;
}

interface ListBackupsCommand extends BaseCommand {
  command: 'list-backups';
}

interface RestoreCommand extends BaseCommand {
  command: 'restore';
  backupName: string;
}

interface MemoryCommand extends BaseCommand {
  command: 'memory';
  subcommand: MemorySubCommands;
}

interface IdeaCommand extends BaseCommand {
  command: 'idea';
  subcommand: IdeaSubCommands;
}

interface AgentCommand extends BaseCommand {
  command: 'agent';
  subcommand: AgentSubCommands;
}

interface EmbCommand extends BaseCommand {
  command: 'emb';
  subcommand: EmbSubCommands;
}

interface ErrorCommand extends BaseCommand {
  command: 'error';
  subcommand: ErrorSubCommands;
}

interface TrainCommand extends BaseCommand {
  command: 'train';
  subcommand: TrainingSubCommands;
}

interface ChatCommand extends BaseCommand {
  command: 'chat';
  message: string;
}

interface SearchAllCommand extends BaseCommand {
  command: 'search-all';
  query: string;
  types?: string;
}

interface MaestroCommand extends BaseCommand {
  command: 'maestro';
  subcommand: MaestroSubCommands;
}

interface ServerCommand extends BaseCommand {
  command: 'server';
  subcommand: ServerSubCommands;
}

interface MLCommand extends BaseCommand {
  command: 'ml';
  subcommand: MLSubCommands;
}

interface IndDemoCommand extends BaseCommand {
  command: 'ind-demo';
}

interface QueueCommand extends BaseCommand {
  command: 'queue';
  subcommand: QueueSubCommands;
}

interface ApiCommand extends BaseCommand {
  command: 'api';
  subcommand: ApiSubCommands;
}

interface TdzCntCommand extends BaseCommand {
  command: 'tdz-cnt';
  content: string;
  sessionId?: string;
  noChecklist: boolean;
  /** @description Do not create a session for this content */
  noSession: boolean;
}

interface ExportEmbeddingsCommand extends BaseCommand {
  command: 'export-embeddings';
  output?: string;
}

interface MigrateCommand extends BaseCommand {
  command: 'migrate';
  dryRun: boolean;
  verbose: boolean;
  force: boolean;
  cleanup: boolean;
}

interface TuiCommand extends BaseCommand {
  command: 'tui';
}

interface ExtractCommand extends BaseCommand {
  command: 'extract';
  content?: string;
  file?: string;
  outputFormat?: 'json' | 'csv' | 'md';
  human: boolean;
}

interface StrategyCommand extends BaseCommand {
  command: 'strategy';
  content?: string;
  file?: string;
  outputFormat?: 'json' | 'csv' | 'md';
  human: boolean;
}

interface StepsCommand extends BaseCommand {
  command: 'steps';
  subcommand: StepsSubCommands;
}

// ===== Subcommand Types =====

export type AddSubCommands = TaskCommand;

export type ListSubCommands = TasksCommand;

export type ShowSubCommands = ShowTaskCommand;

export type ProjectSubCommands =
  | ProjectCreateCommand
  | ProjectListCommand
  | ProjectShowCommand
  | ProjectArchiveCommand
  | ProjectDeleteCommand
  | ProjectUpdateCommand;

export type SearchSubCommands = SearchTasksCommand;

export type StatsSubCommands = StatsShowCommand;

export type BackupSubCommands = BackupCreateCommand;

export type MemorySubCommands =
  | MemoryCreateCommand
  | MemoryCreateSecretCommand
  | MemoryCreateHumanCommand
  | MemoryCreateEmotionalCommand
  | MemoryListCommand
  | MemoryShowCommand
  | MemoryTypesCommand;

export type IdeaSubCommands =
  | IdeaCreateCommand
  | IdeaListCommand
  | IdeaShowCommand;

export type AgentSubCommands =
  | AgentListCommand
  | AgentShowCommand
  | AgentCreateCommand
  | AgentAssignCommand
  | AgentUpdateCommand
  | AgentDeleteCommand;

export type EmbSubCommands =
  | EmbSetModelCommand
  | EmbShowModelCommand
  | EmbListModelsCommand;

export type ErrorSubCommands =
  | ErrorCreateCommand
  | ErrorListCommand
  | ErrorShowCommand
  | ErrorResolveCommand
  | ErrorDeleteCommand;

export type TrainingSubCommands =
  | TrainingCreateCommand
  | TrainingListCommand
  | TrainingShowCommand
  | TrainingStatsCommand
  | TrainingExportCommand
  | TrainingCollectCommand
  | TrainingUpdateCommand
  | TrainingDeleteCommand;

export type MaestroSubCommands =
  | MaestroInitCommand
  | MaestroCollectConversationCommand
  | MaestroCollectToolCommand
  | MaestroListCommand
  | MaestroStatsCommand
  | MaestroExportCommand
  | MaestroIntegrateCommand;

export type ServerSubCommands =
  | ServerStartCommand
  | ServerStatusCommand
  | ServerEndpointsCommand;

export type MLSubCommands =
  | MLProcessCommand
  | MLTrainCommand
  | MLListCommand
  | MLShowCommand
  | MLLoadCommand
  | MLSaveCommand
  | MLTestCommand
  | MLGenerateTrainingDataCommand
  | MLAdvancedProcessCommand
  | MLAdvancedTrainCommand
  | MLAdvancedInferCommand;

export type QueueSubCommands =
  | QueuePlanCommand
  | QueueListCommand
  | QueueBacklogCommand
  | QueueActiveCommand
  | QueueCompleteCommand
  | QueueStartCommand
  | QueueEndCommand;

export type ApiSubCommands =
  | ApiRegisterCommand
  | ApiListCommand
  | ApiCheckCommand
  | ApiDeactivateCommand
  | ApiActivateCommand
  | ApiRemoveCommand;

export type StepsSubCommands =
  | StepsShowCommand
  | StepsAddCommand
  | StepsUpdateCommand
  | StepsDoneCommand
  | StepsArchiveCommand;

// ===== Specific Subcommand Interfaces =====

interface TaskCommand {
  subcommand: 'task';
  action: string;
  time: string;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  project: string;
  status?: string;
  assignee?: string;
  tags?: string;
  dependencies?: string;
  context?: string;
  progress?: number;
}

interface TasksCommand {
  subcommand: 'tasks';
  project?: string;
  status?: string;
  priority?: 'low' | 'medium' | 'high' | 'urgent';
  assignee?: string;
  tags?: string;
  search?: string;
}

interface ShowTaskCommand {
  subcommand: 'task';
  id: string;
}

interface ProjectCreateCommand {
  subcommand: 'create';
  name: string;
  description?: string;
}

interface ProjectListCommand {
  subcommand: 'list';
}

interface ProjectShowCommand {
  subcommand: 'show';
  name: string;
}

interface ProjectArchiveCommand {
  subcommand: 'archive';
  name: string;
}

interface ProjectDeleteCommand {
  subcommand: 'delete';
  name: string;
}

interface ProjectUpdateCommand {
  subcommand: 'update';
  name: string;
  newName?: string;
  description?: string;
  status?: string;
}

interface SearchTasksCommand {
  subcommand: 'tasks';
  query: string;
}

interface StatsShowCommand {
  subcommand: 'show';
}

interface BackupCreateCommand {
  subcommand: 'create';
}

interface MemoryCreateCommand {
  subcommand: 'create';
  moment: string;
  meaning: string;
  reason: string;
  importance?: 'low' | 'medium' | 'high';
  term?: 'short' | 'medium' | 'long';
  memoryType?: 'standard' | 'secret' | 'human' | 'emotional';
  tags?: string;
}

interface MemoryCreateSecretCommand {
  subcommand: 'create-secret';
  moment: string;
  meaning: string;
  reason: string;
  importance?: 'low' | 'medium' | 'high';
  term?: 'short' | 'medium' | 'long';
  tags?: string;
}

interface MemoryCreateHumanCommand {
  subcommand: 'create-human';
  moment: string;
  meaning: string;
  reason: string;
  importance?: 'low' | 'medium' | 'high';
  term?: 'short' | 'medium' | 'long';
  tags?: string;
}

interface MemoryCreateEmotionalCommand {
  subcommand: 'create-emotional';
  moment: string;
  meaning: string;
  reason: string;
  emotion: string;
  importance?: 'low' | 'medium' | 'high';
  term?: 'short' | 'medium' | 'long';
  tags?: string;
}

interface MemoryListCommand {
  subcommand: 'list';
  importance?: 'low' | 'medium' | 'high';
  term?: 'short' | 'medium' | 'long';
  memoryType?: 'standard' | 'secret' | 'human' | 'emotional';
}

interface MemoryShowCommand {
  subcommand: 'show';
  id: string;
}

interface MemoryTypesCommand {
  subcommand: 'types';
}

interface IdeaCreateCommand {
  subcommand: 'create';
  idea: string;
  share?: 'private' | 'public' | 'team';
  importance?: 'low' | 'medium' | 'high';
  tags?: string;
  context?: string;
}

interface IdeaListCommand {
  subcommand: 'list';
  share?: 'private' | 'public' | 'team';
  importance?: 'low' | 'medium' | 'high';
}

interface IdeaShowCommand {
  subcommand: 'show';
  id: string;
}

interface AgentListCommand {
  subcommand: 'list';
}

interface AgentShowCommand {
  subcommand: 'show';
  id: string;
}

interface AgentCreateCommand {
  subcommand: 'create';
  id: string;
  name: string;
  description: string;
  category?: string;
  capabilities?: string;
  specializations?: string;
  modelProvider?: string;
  modelName?: string;
  temperature?: number;
  maxTokens?: number;
  tags?: string;
  systemPrompt?: string;
  promptTemplate?: string;
  autoFormatCode?: boolean;
  includeExamples?: boolean;
  explainComplexity?: boolean;
  suggestTests?: boolean;
  tools?: string;
  maxResponseLength?: number;
  timeoutSeconds?: number;
  requestsPerMinute?: number;
  tokensPerHour?: number;
}

interface AgentAssignCommand {
  subcommand: 'assign';
  agentId: string;
  taskId: string;
  projectId: string;
}

interface AgentUpdateCommand {
  subcommand: 'update';
  id: string;
  name?: string;
  description?: string;
  category?: string;
  capabilities?: string;
  specializations?: string;
  systemPrompt?: string;
  promptTemplate?: string;
  modelProvider?: string;
  modelName?: string;
  temperature?: number;
  maxTokens?: number;
  tags?: string;
  autoFormatCode?: boolean;
  includeExamples?: boolean;
  explainComplexity?: boolean;
  suggestTests?: boolean;
  tools?: string;
  maxResponseLength?: number;
  timeoutSeconds?: number;
  requestsPerMinute?: number;
  tokensPerHour?: number;
}

interface AgentDeleteCommand {
  subcommand: 'delete';
  id: string;
}

interface EmbSetModelCommand {
  subcommand: 'set-model';
  modelName: string;
}

interface EmbShowModelCommand {
  subcommand: 'show-model';
}

interface EmbListModelsCommand {
  subcommand: 'list-models';
}

interface ErrorCreateCommand {
  subcommand: 'create';
  title: string;
  description: string;
  severity?: 'low' | 'medium' | 'high' | 'critical';
  category?: 'runtime' | 'compile' | 'logic' | 'system';
  source: string;
  context?: string;
  tags?: string;
}

interface ErrorListCommand {
  subcommand: 'list';
  severity?: 'low' | 'medium' | 'high' | 'critical';
  category?: 'runtime' | 'compile' | 'logic' | 'system';
  unresolvedOnly?: boolean;
}

interface ErrorShowCommand {
  subcommand: 'show';
  id: string;
}

interface ErrorResolveCommand {
  subcommand: 'resolve';
  id: string;
  resolution?: string;
}

interface ErrorDeleteCommand {
  subcommand: 'delete';
  id: string;
}

interface TrainingCreateCommand {
  subcommand: 'create';
  dataType?: 'instruction' | 'conversation' | 'code' | 'documentation';
  prompt: string;
  completion: string;
  context?: string;
  tags?: string;
  quality?: number;
  source?: 'manual' | 'generated' | 'collected';
}

interface TrainingListCommand {
  subcommand: 'list';
  dataType?: 'instruction' | 'conversation' | 'code' | 'documentation';
  minQuality?: number;
}

interface TrainingShowCommand {
  subcommand: 'show';
  id: string;
}

interface TrainingStatsCommand {
  subcommand: 'stats';
}

interface TrainingExportCommand {
  subcommand: 'export';
  format?: 'json' | 'csv' | 'yaml';
  dataType?: 'instruction' | 'conversation' | 'code' | 'documentation';
  minQuality?: number;
  outputFile?: string;
}

interface TrainingCollectCommand {
  subcommand: 'collect';
  message: string;
}

interface TrainingUpdateCommand {
  subcommand: 'update';
  id: string;
  dataType?: 'instruction' | 'conversation' | 'code' | 'documentation';
  prompt?: string;
  completion?: string;
  context?: string;
  tags?: string;
  quality?: number;
  source?: 'manual' | 'generated' | 'collected';
}

interface TrainingDeleteCommand {
  subcommand: 'delete';
  id: string;
}

interface MaestroInitCommand {
  subcommand: 'init';
}

interface MaestroCollectConversationCommand {
  subcommand: 'collect-conversation';
  sessionId: string;
  conversation: string;
  contextLength?: number;
  toolCalls?: string;
  response: string;
  responseTimeMs?: number;
}

interface MaestroCollectToolCommand {
  subcommand: 'collect-tool';
  sessionId: string;
  toolName: string;
  toolCall: string;
  executionTimeMs?: number;
  success: boolean;
  resultSummary: string;
}

interface MaestroListCommand {
  subcommand: 'list';
  sessionId?: string;
  interactionType?: string;
  limit?: number;
}

interface MaestroStatsCommand {
  subcommand: 'stats';
}

interface MaestroExportCommand {
  subcommand: 'export';
  output: string;
}

interface MaestroIntegrateCommand {
  subcommand: 'integrate';
}

interface ServerStartCommand {
  subcommand: 'start';
  host?: string;
  port?: number;
}

interface ServerStatusCommand {
  subcommand: 'status';
}

interface ServerEndpointsCommand {
  subcommand: 'endpoints';
}

interface MLProcessCommand {
  subcommand: 'process';
  text: string;
  useMl?: boolean;
  model?: string;
}

interface MLTrainCommand {
  subcommand: 'train';
  data: string;
  modelName?: string;
  epochs?: number;
}

interface MLListCommand {
  subcommand: 'list';
}

interface MLShowCommand {
  subcommand: 'show';
  modelName: string;
}

interface MLLoadCommand {
  subcommand: 'load';
  modelName: string;
  path: string;
}

interface MLSaveCommand {
  subcommand: 'save';
  modelName: string;
  output: string;
}

interface MLTestCommand {
  subcommand: 'test';
  testData: string;
  modelName?: string;
}

interface MLGenerateTrainingDataCommand {
  subcommand: 'generate-training-data';
  output: string;
  samples?: number;
}

interface MLAdvancedProcessCommand {
  subcommand: 'advanced-process';
  text: string;
  analytics?: boolean;
}

interface MLAdvancedTrainCommand {
  subcommand: 'advanced-train';
  data: string;
  epochs?: number;
}

interface MLAdvancedInferCommand {
  subcommand: 'advanced-infer';
  text: string;
  detailed?: boolean;
}

interface QueuePlanCommand {
  subcommand: 'plan';
  taskName: string;
  taskDescription: string;
  priority?: 'low' | 'medium' | 'high' | 'urgent';
  projectId?: string;
}

interface QueueListCommand {
  subcommand: 'list';
  status?: 'backlog' | 'active' | 'complete';
}

interface QueueBacklogCommand {
  subcommand: 'backlog';
}

interface QueueActiveCommand {
  subcommand: 'active';
}

interface QueueCompleteCommand {
  subcommand: 'complete';
}

interface QueueStartCommand {
  subcommand: 'start';
  queueItemId: string;
}

interface QueueEndCommand {
  subcommand: 'end';
  sessionId: string;
}

interface ApiRegisterCommand {
  subcommand: 'register';
  userId?: string;
}

interface ApiListCommand {
  subcommand: 'list';
  activeOnly?: boolean;
}

interface ApiCheckCommand {
  subcommand: 'check';
  publicKey: string;
  privateKey?: string;
}

interface ApiDeactivateCommand {
  subcommand: 'deactivate';
  userId: string;
}

interface ApiActivateCommand {
  subcommand: 'activate';
  userId: string;
}

interface ApiRemoveCommand {
  subcommand: 'remove';
  userId: string;
}

interface StepsShowCommand {
  subcommand: 'show';
  taskId: string;
}

interface StepsAddCommand {
  subcommand: 'add';
  taskId: string;
  step: string;
}

interface StepsUpdateCommand {
  subcommand: 'update';
  taskId: string;
  stepIndex: number;
  newStep: string;
}

interface StepsDoneCommand {
  subcommand: 'done';
  taskId: string;
}

interface StepsArchiveCommand {
  subcommand: 'archive';
  taskId: string;
}

// ===== Data Structures =====

export type QueueStatus = 'backlog' | 'active' | 'complete';

export interface QueueItem {
  id: string;
  taskName: string;
  taskDescription: string;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  projectId?: string;
  status: QueueStatus;
  createdAt: Date;
  updatedAt: Date;
}

export interface TaskUpdate {
  id: string;
  action?: string;
  time?: string;
  priority?: 'low' | 'medium' | 'high' | 'urgent';
  project?: string;
  status?: string;
  assignee?: string;
  tags?: string;
  dependencies?: string;
  context?: string;
  progress?: number; // 0-100
}

export interface SearchOptions {
  limit?: number;
  dataTypes?: string;
  since?: string;
  until?: string;
}

export class SearchEngine {
  constructor() {}

  updateIndex(content: ChatContent): void {
    // Implementation would go here
    // Currently a placeholder as per Rust implementation
  }

  search(query: string, options: SearchOptions): SearchResults {
    return {
      taskResults: [],
      memoryResults: [],
      ideaResults: [],
      errorResults: [],
      trainingResults: [],
    };
  }
}

export interface ChatContent {
  tasks: Task[];
  memories: Memory[];
  ideas: Idea[];
  agentAssignments: AgentAssignment[];
  codeChunks: CodeChunk[];
  errors: Error[];
  trainingData: TrainingData[];
  feelings: Feeling[];
}

export interface SearchResults {
  taskResults: Task[];
  memoryResults: Memory[];
  ideaResults: Idea[];
  errorResults: Error[];
  trainingResults: TrainingData[];
}

// ===== Default Values =====

export const DEFAULT_VALUES = {
  register: {
    serverUrl: 'https://todozi.com',
  },
  task: {
    status: 'todo',
  },
  memory: {
    importance: 'medium' as const,
    term: 'short' as const,
    memoryType: 'standard' as const,
  },
  memoryHuman: {
    importance: 'high' as const,
    term: 'long' as const,
  },
  idea: {
    share: 'private' as const,
    importance: 'medium' as const,
  },
  error: {
    severity: 'medium' as const,
    category: 'runtime' as const,
  },
  training: {
    dataType: 'instruction' as const,
    quality: 0.8, // Example default quality
    source: 'manual' as const,
  },
  agent: {
    category: 'general' as const,
    modelProvider: 'todozi' as const,
    modelName: 'baton' as const,
    temperature: 0.2,
    maxTokens: 4096,
  },
  server: {
    host: '127.0.0.1',
    port: 8636,
  },
  ml: {
    model: 'todozi' as const,
    modelName: 'todozi-tag-processor' as const,
    epochs: 10,
  },
  queue: {
    priority: 'medium' as const,
  },
  extract: {
    outputFormat: 'json' as const,
  },
  searchAll: {
    types: 'tasks,memories,ideas,errors',
  },
  maestro: {
    contextLength: 0,
    responseTimeMs: 1000,
    executionTimeMs: 500,
    limit: 10,
  },
  exportEmbeddings: {
    output: 'todozi_embeddings.hlx',
  },
} as const;

// ===== Type Guards and Utilities =====

/**
 * Type guard to check if a command is of a specific type
 */
export function isCommandType<T extends Commands['command']>(
  command: Commands,
  type: T
): command is Extract<Commands, { command: T }> {
  return command.command === type;
}

/**
 * Helper to create commands with proper typing
 */
export function createCommand<T extends Commands>(
  commandData: Omit<T, 'command'> & { command: T['command'] }
): T {
  return commandData as T;
}

// ===== Validation Types =====

/**
 * Type for mutually exclusive fields (like content vs file in extract/strategy)
 */
export type ContentOrFileArgs =
  | { content: string; file?: never }
  | { content?: never; file: string };

/**
 * Enhanced Extract type with mutual exclusion
 */
export type EnhancedExtractArgs = ContentOrFileArgs & {
  outputFormat?: 'json' | 'csv' | 'md';
  human: boolean;
};

/**
 * Enhanced Strategy type with mutual exclusion
 */
export type EnhancedStrategyArgs = ContentOrFileArgs & {
  outputFormat?: 'json' | 'csv' | 'md';
  human: boolean;
};

// ===== Error Types =====

export class CommandExecutionError extends Error {
  constructor(
    message: string,
    public readonly command: Commands,
    public readonly cause?: Error
  ) {
    super(message);
    this.name = 'CommandExecutionError';
  }
}

// ===== Runtime Validation =====

/**
 * Validates that either content or file is provided (not both)
 */
export function validateContentOrFile(args: ContentOrFileArgs): void {
  if (!args.content && !args.file) {
    throw new Error('Either content or file must be provided');
  }
  if (args.content && args.file) {
    throw new Error('Cannot provide both content and file');
  }
}

/**
 * Validates progress is within 0-100 range
 */
export function validateProgress(progress?: number): void {
  if (progress !== undefined && (progress < 0 || progress > 100)) {
    throw new Error('Progress must be between 0 and 100');
  }
}

/**
 * Validates quality is within 0-1 range
 */
export function validateQuality(quality?: number): void {
  if (quality !== undefined && (quality < 0 || quality > 1)) {
    throw new Error('Quality must be between 0 and 1');
  }
}
