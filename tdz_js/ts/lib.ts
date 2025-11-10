// types/index.ts
export enum Priority {
  Critical = "critical",
  Urgent = "urgent",
  High = "high",
  Medium = "medium",
  Low = "low"
}

export enum Status {
  Todo = "todo",
  InProgress = "in_progress",
  Done = "done",
  Blocked = "blocked"
}

export enum Assignee {
  Human = "human",
  Ai = "ai",
  Collaborative = "collaborative"
}

export interface Task {
  id: string;
  userId: string;
  action: string;
  time: string;
  priority: Priority;
  parentProject: string;
  status: Status;
  assignee?: Assignee;
  tags: string[];
  dependencies: string[];
  contextNotes?: string;
  progress?: number;
  createdAt: Date;
  updatedAt: Date;
  embeddingVector?: number[];
}

export interface TaskFilters {
  project?: string;
  status?: Status;
  priority?: Priority;
  assignee?: Assignee;
  tags?: string[];
  search?: string;
}

export interface TaskUpdate {
  action?: string;
  priority?: Priority;
  status?: Status;
  parentProject?: string;
  assignee?: Assignee;
  tags?: string[];
  dependencies?: string[];
  contextNotes?: string;
  progress?: number;
}

export interface Config {
  version: string;
  defaultProject: string;
  autoBackup: boolean;
  backupInterval: string;
  aiEnabled: boolean;
  defaultAssignee: string;
  dateFormat: string;
  timezone: string;
}

export interface RegistrationInfo {
  apiKey: string;
  userId?: string;
  fingerprint?: string;
  serverUrl: string;
}

export interface ChatContent {
  message: string;
  response: string;
  tasks: Task[];
}

export interface SimilarityResult {
  textContent: string;
  similarityScore: number;
  contentType: string;
  id: string;
}

export interface TodoziEmbeddingConfig {
  model: string;
  dimensions: number;
  batchSize: number;
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
  importance: IdeaImportance;
  tags: string[];
  context?: string;
  createdAt: Date;
  updatedAt: Date;
}

export enum ItemStatus {
  Active = "active",
  Archived = "archived",
  Deleted = "deleted"
}

export enum MemoryImportance {
  Low = "low",
  Medium = "medium",
  High = "high",
  Critical = "critical"
}

export enum MemoryTerm {
  Short = "short",
  Medium = "medium",
  Long = "long"
}

export enum MemoryType {
  Standard = "standard",
  Episodic = "episodic",
  Semantic = "semantic",
  Procedural = "procedural"
}

export enum ShareLevel {
  Private = "private",
  Team = "team",
  Public = "public"
}

export enum IdeaImportance {
  Low = "low",
  Medium = "medium",
  High = "high",
  Breakthrough = "breakthrough"
}

export interface QueueItem {
  id: string;
  taskName: string;
  taskDescription: string;
  priority: Priority;
  status: QueueStatus;
  createdAt: Date;
  updatedAt: Date;
}

export enum QueueStatus {
  Pending = "pending",
  Active = "active",
  Complete = "complete",
  Backlog = "backlog"
}

export interface Project {
  name: string;
  description?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface Tag {
  id: string;
  name: string;
  description?: string;
  color?: string;
  category?: string;
  usageCount: number;
  createdAt: Date;
  updatedAt: Date;
}

export interface Agent {
  id: string;
  name: string;
  description: string;
  capabilities: string[];
  specializations: string[];
  category: string;
  author?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface AgentAssignment {
  id: string;
  agentId: string;
  taskId: string;
  status: AssignmentStatus;
  assignedAt: Date;
  completedAt?: Date;
}

export enum AssignmentStatus {
  Pending = "pending",
  Active = "active",
  Complete = "complete",
  Failed = "failed"
}

export interface ClusteringResult {
  clusterId: string;
  items: string[];
  centroid: number[];
  score: number;
}

export interface ApiKey {
  id: string;
  userId: string;
  key: string;
  permissions: string[];
  createdAt: Date;
  expiresAt?: Date;
}

export interface ApiKeyCollection {
  keys: Record<string, ApiKey>;
  lastUpdated: Date;
}

export interface Reminder {
  id: string;
  title: string;
  time: Date;
  priority: ReminderPriority;
  taskId?: string;
  completed: boolean;
  createdAt: Date;
}

export enum ReminderPriority {
  Low = "low",
  Medium = "medium",
  High = "high",
  Critical = "critical"
}

export interface Error {
  id: string;
  message: string;
  errorType: ErrorType;
  stackTrace?: string;
  context?: Record<string, any>;
  createdAt: Date;
  resolvedAt?: Date;
}

export enum ErrorType {
  ValidationError = "validation_error",
  SystemError = "system_error",
  NetworkError = "network_error",
  DatabaseError = "database_error",
  ApiError = "api_error"
}

export interface ProjectTaskContainer {
  projectHash: string;
  projectName: string;
  tasks: Task[];
  metadata: Record<string, any>;
  createdAt: Date;
  updatedAt: Date;
}

export interface Summary {
  id: string;
  title: string;
  content: string;
  projectId?: string;
  priority: SummaryPriority;
  createdAt: Date;
  updatedAt: Date;
}

export enum SummaryPriority {
  Low = "low",
  Medium = "medium",
  High = "high"
}

export interface TrainingData {
  id: string;
  input: string;
  output: string;
  dataType: TrainingDataType;
  metadata?: Record<string, any>;
  createdAt: Date;
}

export enum TrainingDataType {
  Task = "task",
  Memory = "memory",
  Idea = "idea",
  Chat = "chat"
}

export interface ToolResult {
  success: boolean;
  output: string;
  error?: string;
  executionTimeMs: number;
  metadata?: Record<string, any>;
  recoveryContext?: string;
}

export interface ToolParameter {
  name: string;
  type: string;
  required: boolean;
  description: string;
  defaultValue?: any;
}

export interface ToolDefinition {
  name: string;
  description: string;
  category: string;
  parameters: ToolParameter[];
  locks: ResourceLock[];
}

export interface ResourceLock {
  type: string;
  id: string;
  timeout?: number;
}

export enum TodoziContentType {
  Task = "task",
  Memory = "memory",
  Idea = "idea",
  Error = "error"
}

export interface PerformanceMetrics {
  queryTime: number;
  resultsCount: number;
  cacheHitRate: number;
  memoryUsage: number;
}

export interface SimilarityGraph {
  nodes: Array<{
    id: string;
    label: string;
    cluster?: string;
  }>;
  edges: Array<{
    source: string;
    target: string;
    weight: number;
  }>;
}

export interface ModelComparisonResult {
  model: string;
  accuracy: number;
  speed: number;
  memoryUsage: number;
  recommendations: string[];
}

export interface LabeledCluster {
  clusterId: string;
  label: string;
  items: string[];
  confidence: number;
}

export interface DriftReport {
  contentId: string;
  driftScore: number;
  recommendations: string[];
}

export interface ValidationReport {
  isValid: boolean;
  errors: string[];
  warnings: string[];
  score: number;
}

export interface MigrationReport {
  success: boolean;
  itemsMigrated: number;
  errors: string[];
  duration: number;
}

export interface ProjectStats {
  totalTasks: number;
  completedTasks: number;
  averageCompletionTime: number;
  priorityDistribution: Record<Priority, number>;
}

export interface SearchAnalytics {
  totalQueries: number;
  averageResponseTime: number;
  topQueries: Array<{
    query: string;
    count: number;
  }>;
}

export interface SearchResults {
  tasks: Task[];
  memories: Memory[];
  ideas: Idea[];
  totalCount: number;
  queryTime: number;
}
