// Brand types for better type safety
declare const __brand: unique symbol;
type ChunkId = string & { readonly __brand: typeof __brand };
type ModuleName = string & { readonly __brand: typeof __brand };

// Result type for better error handling
type Result<T, E = Error> = {
  success: true;
  data: T;
} | {
  success: false;
  error: E;
};

// Event emitter for state changes
type StateChangeEvent = {
  type: 'chunk_added' | 'chunk_updated' | 'chunk_completed' | 'chunk_validated';
  chunkId: ChunkId;
  timestamp: Date;
};

interface IEventEmitter {
  on(event: string, listener: (...args: any[]) => void): void;
  emit(event: string, ...args: any[]): void;
}

// Configuration options
interface CodeGraphConfig {
  dateFormat?: Intl.DateTimeFormatOptions;
  tokenCountingStrategy?: 'simple' | 'cleaned';
  enableEvents?: boolean;
}

// Interfaces for better abstraction
interface IProjectState {
  readonly totalLines: number;
  readonly maxLines: number;
  readonly currentModule: string;
  readonly dependencies: ReadonlyArray<ModuleName>;
  readonly completedModules: ReadonlyArray<ModuleName>;
  readonly pendingModules: ReadonlyArray<ModuleName>;
  readonly globalVariables: ReadonlyMap<string, string>;
  readonly createdAt: Date;
  readonly updatedAt: Date;
  
  toStateString(): string;
  addCompletedModule(module: ModuleName): void;
  addPendingModule(module: ModuleName): void;
  setGlobalVariable(key: string, value: string): void;
  incrementLines(lines: number): void;
}

interface IContextWindow {
  readonly previousClass: string;
  readonly currentClass: string;
  readonly nextPlanned: string;
  readonly globalVarsInScope: ReadonlyArray<string>;
  readonly importsUsed: ReadonlyArray<string>;
  readonly functionSignatures: ReadonlyMap<string, string>;
  readonly errorPatternsSeen: ReadonlyArray<string>;
  readonly createdAt: Date;
  readonly updatedAt: Date;
  
  toContextString(): string;
  addImport(importStatement: string): void;
  addFunctionSignature(name: string, signature: string): void;
  addErrorPattern(pattern: string): void;
  setCurrentClass(className: string): void;
}

interface ICodeChunk {
  readonly chunkId: ChunkId;
  readonly status: ChunkStatus;
  readonly dependencies: ReadonlyArray<ChunkId>;
  readonly code: string;
  readonly tests: string;
  readonly validated: boolean;
  readonly level: ChunkingLevel;
  readonly estimatedTokens: number;
  readonly createdAt: Date;
  readonly updatedAt: Date;
  
  addDependency(dep: ChunkId): void;
  setCode(code: string): void;
  setTests(tests: string): void;
  markCompleted(): void;
  markValidated(): void;
  markFailed(): void;
}

// Enums
export enum ChunkingLevel {
  Project = 'project',
  Module = 'module',
  Class = 'class',
  Method = 'method',
  Block = 'block',
}

export enum ChunkStatus {
  Pending = 'pending',
  InProgress = 'in_progress',
  Completed = 'completed',
  Validated = 'validated',
  Failed = 'failed',
}

// Utility functions for creating branded types
function createChunkId(id: string): ChunkId {
  if (!id.trim()) throw new Error('Chunk ID cannot be empty');
  return id as ChunkId;
}

function createModuleName(name: string): ModuleName {
  if (!name.trim()) throw new Error('Module name cannot be empty');
  return name as ModuleName;
}

// Main implementation classes
export class ChunkingLevelMethods {
  static maxTokens(level: ChunkingLevel): number {
    const tokenMap = {
      [ChunkingLevel.Project]: 100,
      [ChunkingLevel.Module]: 500,
      [ChunkingLevel.Class]: 1000,
      [ChunkingLevel.Method]: 300,
      [ChunkingLevel.Block]: 100,
    };
    return tokenMap[level];
  }

  static description(level: ChunkingLevel): string {
    const descriptions = {
      [ChunkingLevel.Project]: "High-level project planning and architecture",
      [ChunkingLevel.Module]: "Major system components and interfaces",
      [ChunkingLevel.Class]: "Class definitions and major functions",
      [ChunkingLevel.Method]: "Individual methods and helper functions",
      [ChunkingLevel.Block]: "Small code blocks and error handling",
    };
    return descriptions[level];
  }

  static example(level: ChunkingLevel): string {
    const examples = {
      [ChunkingLevel.Project]: "Build web scraper with database storage",
      [ChunkingLevel.Module]: "Create database handler module",
      [ChunkingLevel.Class]: "Implement DatabaseConnection class",
      [ChunkingLevel.Method]: "Write insert_record method",
      [ChunkingLevel.Block]: "Add error handling for connection timeout",
    };
    return examples[level];
  }

  static fromString(level: string): Result<ChunkingLevel> {
    const normalized = level.toLowerCase().trim();
    switch (normalized) {
      case 'project':
        return { success: true, data: ChunkingLevel.Project };
      case 'module':
        return { success: true, data: ChunkingLevel.Module };
      case 'class':
        return { success: true, data: ChunkingLevel.Class };
      case 'method':
        return { success: true, data: ChunkingLevel.Method };
      case 'block':
        return { success: true, data: ChunkingLevel.Block };
      default:
        return { 
          success: false, 
          error: new Error(`Invalid chunking level: ${level}`)
        };
    }
  }
}

export class ProjectState implements IProjectState {
  public readonly totalLines: number;
  public readonly maxLines: number;
  public readonly currentModule: string;
  public readonly dependencies: ReadonlyArray<ModuleName>;
  public readonly completedModules: ReadonlyArray<ModuleName>;
  public readonly pendingModules: ReadonlyArray<ModuleName>;
  public readonly globalVariables: ReadonlyMap<string, string>;
  public readonly createdAt: Date;
  public readonly updatedAt: Date;

  private readonly _config: CodeGraphConfig;
  private readonly _dateFormatter: Intl.DateTimeFormat;

  constructor(data: {
    totalLines?: number;
    maxLines: number;
    currentModule?: string;
    dependencies?: ReadonlyArray<ModuleName>;
    completedModules?: ReadonlyArray<ModuleName>;
    pendingModules?: ReadonlyArray<ModuleName>;
    globalVariables?: ReadonlyMap<string, string>;
    createdAt?: Date;
    updatedAt?: Date;
  }, config: CodeGraphConfig = {}) {
    const now = new Date();
    this._config = config;
    this._dateFormatter = new Intl.DateTimeFormat(
      'en-US',
      config.dateFormat || {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
      }
    );

    this.totalLines = data.totalLines ?? 0;
    this.maxLines = data.maxLines;
    this.currentModule = data.currentModule ?? '';
    this.dependencies = Object.freeze(data.dependencies ?? []);
    this.completedModules = Object.freeze(data.completedModules ?? []);
    this.pendingModules = Object.freeze(data.pendingModules ?? []);
    this.globalVariables = new Map(data.globalVariables ?? []);
    this.createdAt = data.createdAt ?? now;
    this.updatedAt = data.updatedAt ?? now;
  }

  static new(maxLines: number, config?: CodeGraphConfig): ProjectState {
    return new ProjectState({ maxLines }, config);
  }

  private formatDate(date: Date): string {
    return this._dateFormatter.format(date);
  }

  toStateString(): string {
    return `<project_state>
- Total lines written: ${this.totalLines}/${this.maxLines}
- Current module: ${this.currentModule}
- Dependencies: ${this.dependencies.join(', ')}
- Completed modules: ${this.completedModules.join(', ')}
- Pending modules: ${this.pendingModules.join(', ')}
- Global variables: ${Array.from(this.globalVariables.entries())
  .map(([k, v]) => `${k}=${v}`)
  .join(', ')}
- Created: ${this.formatDate(this.createdAt)}
- Updated: ${this.formatDate(this.updatedAt)}
</project_state>`;
  }

  addCompletedModule(module: ModuleName): ProjectState {
    if (this.completedModules.includes(module)) {
      return this;
    }

    return new ProjectState(
      {
        ...this,
        completedModules: [...this.completedModules, module],
        updatedAt: new Date()
      },
      this._config
    );
  }

  addPendingModule(module: ModuleName): ProjectState {
    if (this.pendingModules.includes(module)) {
      return this;
    }

    return new ProjectState(
      {
        ...this,
        pendingModules: [...this.pendingModules, module],
        updatedAt: new Date()
      },
      this._config
    );
  }

  setGlobalVariable(key: string, value: string): ProjectState {
    const newGlobalVars = new Map(this.globalVariables);
    newGlobalVars.set(key, value);

    return new ProjectState(
      {
        ...this,
        globalVariables: newGlobalVars,
        updatedAt: new Date()
      },
      this._config
    );
  }

  incrementLines(lines: number): ProjectState {
    return new ProjectState(
      {
        ...this,
        totalLines: this.totalLines + lines,
        updatedAt: new Date()
      },
      this._config
    );
  }
}

export class ContextWindow implements IContextWindow {
  public readonly previousClass: string;
  public readonly currentClass: string;
  public readonly nextPlanned: string;
  public readonly globalVarsInScope: ReadonlyArray<string>;
  public readonly importsUsed: ReadonlyArray<string>;
  public readonly functionSignatures: ReadonlyMap<string, string>;
  public readonly errorPatternsSeen: ReadonlyArray<string>;
  public readonly createdAt: Date;
  public readonly updatedAt: Date;

  private readonly _config: CodeGraphConfig;
  private readonly _dateFormatter: Intl.DateTimeFormat;

  constructor(data: {
    previousClass?: string;
    currentClass?: string;
    nextPlanned?: string;
    globalVarsInScope?: ReadonlyArray<string>;
    importsUsed?: ReadonlyArray<string>;
    functionSignatures?: ReadonlyMap<string, string>;
    errorPatternsSeen?: ReadonlyArray<string>;
    createdAt?: Date;
    updatedAt?: Date;
  }, config: CodeGraphConfig = {}) {
    const now = new Date();
    this._config = config;
    this._dateFormatter = new Intl.DateTimeFormat(
      'en-US',
      config.dateFormat || {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
      }
    );

    this.previousClass = data.previousClass ?? '';
    this.currentClass = data.currentClass ?? '';
    this.nextPlanned = data.nextPlanned ?? '';
    this.globalVarsInScope = Object.freeze(data.globalVarsInScope ?? []);
    this.importsUsed = Object.freeze(data.importsUsed ?? []);
    this.functionSignatures = new Map(data.functionSignatures ?? []);
    this.errorPatternsSeen = Object.freeze(data.errorPatternsSeen ?? []);
    this.createdAt = data.createdAt ?? now;
    this.updatedAt = data.updatedAt ?? now;
  }

  static new(config?: CodeGraphConfig): ContextWindow {
    return new ContextWindow({}, config);
  }

  private formatDate(date: Date): string {
    return this._dateFormatter.format(date);
  }

  toContextString(): string {
    return `<context_window>
- Previous class: ${this.previousClass}
- Current class: ${this.currentClass}
- Next planned: ${this.nextPlanned}
- Global variables in scope: ${this.globalVarsInScope.join(', ')}
- Imports used: ${this.importsUsed.join(', ')}
- Function signatures: ${Array.from(this.functionSignatures.entries())
  .map(([k, v]) => `${k}: ${v}`)
  .join(', ')}
- Error patterns seen: ${this.errorPatternsSeen.join(', ')}
- Created: ${this.formatDate(this.createdAt)}
- Updated: ${this.formatDate(this.updatedAt)}
</context_window>`;
  }

  addImport(importStatement: string): ContextWindow {
    if (this.importsUsed.includes(importStatement)) {
      return this;
    }

    return new ContextWindow(
      {
        ...this,
        importsUsed: [...this.importsUsed, importStatement],
        updatedAt: new Date()
      },
      this._config
    );
  }

  addFunctionSignature(name: string, signature: string): ContextWindow {
    const newSignatures = new Map(this.functionSignatures);
    newSignatures.set(name, signature);

    return new ContextWindow(
      {
        ...this,
        functionSignatures: newSignatures,
        updatedAt: new Date()
      },
      this._config
    );
  }

  addErrorPattern(pattern: string): ContextWindow {
    if (this.errorPatternsSeen.includes(pattern)) {
      return this;
    }

    return new ContextWindow(
      {
        ...this,
        errorPatternsSeen: [...this.errorPatternsSeen, pattern],
        updatedAt: new Date()
      },
      this._config
    );
  }

  setCurrentClass(className: string): ContextWindow {
    return new ContextWindow(
      {
        ...this,
        previousClass: this.currentClass,
        currentClass: className,
        updatedAt: new Date()
      },
      this._config
    );
  }
}

export class CodeChunk implements ICodeChunk {
  public readonly chunkId: ChunkId;
  public readonly status: ChunkStatus;
  public readonly dependencies: ReadonlyArray<ChunkId>;
  public readonly code: string;
  public readonly tests: string;
  public readonly validated: boolean;
  public readonly level: ChunkingLevel;
  public readonly estimatedTokens: number;
  public readonly createdAt: Date;
  public readonly updatedAt: Date;

  private readonly _config: CodeGraphConfig;

  constructor(data: {
    chunkId: ChunkId;
    status?: ChunkStatus;
    dependencies?: ReadonlyArray<ChunkId>;
    code?: string;
    tests?: string;
    validated?: boolean;
    level: ChunkingLevel;
    estimatedTokens?: number;
    createdAt?: Date;
    updatedAt?: Date;
  }, config: CodeGraphConfig = {}) {
    const now = new Date();
    this._config = config;

    this.chunkId = data.chunkId;
    this.status = data.status ?? ChunkStatus.Pending;
    this.dependencies = Object.freeze(data.dependencies ?? []);
    this.code = data.code ?? '';
    this.tests = data.tests ?? '';
    this.validated = data.validated ?? false;
    this.level = data.level;
    this.estimatedTokens = data.estimatedTokens ?? this.estimateTokens(data.code ?? '');
    this.createdAt = data.createdAt ?? now;
    this.updatedAt = data.updatedAt ?? now;
  }

  static new(chunkId: string, level: ChunkingLevel, config?: CodeGraphConfig): CodeChunk {
    return new CodeChunk(
      { chunkId: createChunkId(chunkId), level },
      config
    );
  }

  private estimateTokens(code: string): number {
    if (this._config.tokenCountingStrategy === 'cleaned') {
      // Remove comments and count meaningful tokens
      const cleanedCode = code
        .replace(/\/\/.*$/gm, '')
        .replace(/\/\*[\s\S]*?\*\//g, '');
      return cleanedCode.split(/\s+/).filter(token => token.length > 0).length;
    }
    // Simple whitespace splitting
    return code.split(/\s+/).filter(Boolean).length;
  }

  addDependency(dep: ChunkId): CodeChunk {
    if (this.dependencies.includes(dep)) {
      return this;
    }

    return new CodeChunk(
      {
        ...this,
        dependencies: [...this.dependencies, dep],
        updatedAt: new Date()
      },
      this._config
    );
  }

  setCode(code: string): CodeChunk {
    return new CodeChunk(
      {
        ...this,
        code,
        estimatedTokens: this.estimateTokens(code),
        updatedAt: new Date()
      },
      this._config
    );
  }

  setTests(tests: string): CodeChunk {
    return new CodeChunk(
      {
        ...this,
        tests,
        updatedAt: new Date()
      },
      this._config
    );
  }

  markCompleted(): CodeChunk {
    return new CodeChunk(
      {
        ...this,
        status: ChunkStatus.Completed,
        updatedAt: new Date()
      },
      this._config
    );
  }

  markValidated(): CodeChunk {
    return new CodeChunk(
      {
        ...this,
        validated: true,
        status: ChunkStatus.Validated,
        updatedAt: new Date()
      },
      this._config
    );
  }

  markFailed(): CodeChunk {
    return new CodeChunk(
      {
        ...this,
        status: ChunkStatus.Failed,
        updatedAt: new Date()
      },
      this._config
    );
  }
}

export class CodeGenerationGraph {
  private readonly chunks: Map<ChunkId, CodeChunk>;
  private projectState: ProjectState;
  private contextWindow: ContextWindow;
  private readonly _config: CodeGraphConfig;
  private readonly _eventEmitter: IEventEmitter;

  constructor(
    maxLines: number,
    config: CodeGraphConfig = {},
    eventEmitter: IEventEmitter = new EventEmitter()
  ) {
    this._config = config;
    this._eventEmitter = eventEmitter;
    this.chunks = new Map();
    this.projectState = ProjectState.new(maxLines, config);
    this.contextWindow = ContextWindow.new(config);
  }

  private validateChunkId(chunkId: string): ChunkId {
    if (!chunkId.trim()) {
      throw new Error('Chunk ID cannot be empty');
    }
    return createChunkId(chunkId);
  }

  private validateDependencies(deps: string[]): ChunkId[] {
    return deps.map(dep => this.validateChunkId(dep));
  }

  addChunk(chunkId: string, level: ChunkingLevel, deps: string[] = []): Result<void> {
    try {
      const id = this.validateChunkId(chunkId);
      const dependencies = this.validateDependencies(deps);

      if (this.chunks.has(id)) {
        return { 
          success: false, 
          error: new Error(`Chunk with ID ${chunkId} already exists`) 
        };
      }

      const invalidDeps = dependencies.filter(dep => !this.chunks.has(dep));
      if (invalidDeps.length > 0) {
        return { 
          success: false, 
          error: new Error(`Dependencies do not exist: ${invalidDeps.join(', ')}`) 
        };
      }

      const chunk = CodeChunk.new(chunkId, level, this._config);
      let finalChunk = chunk;
      for (const dep of dependencies) {
        finalChunk = finalChunk.addDependency(dep);
      }

      this.chunks.set(id, finalChunk);

      if (this._config.enableEvents) {
        this._eventEmitter.emit('chunkAdded', { type: 'chunk_added', chunkId: id, timestamp: new Date() });
      }

      return { success: true, data: undefined };
    } catch (error) {
      return { success: false, error: error as Error };
    }
  }

  getReadyChunks(): ChunkId[] {
    return Array.from(this.chunks.entries())
      .filter(([_, chunk]) => chunk.status === ChunkStatus.Pending)
      .filter(([_, chunk]) => chunk.dependencies.every(dep => {
        const depChunk = this.chunks.get(dep);
        return depChunk?.status === ChunkStatus.Completed || 
               depChunk?.status === ChunkStatus.Validated;
      }))
      .map(([chunkId]) => chunkId);
  }

  getChunk(chunkId: string): CodeChunk | undefined {
    return this.chunks.get(this.validateChunkId(chunkId));
  }

  updateChunkCode(chunkId: string, code: string): Result<void> {
    const id = this.validateChunkId(chunkId);
    const chunk = this.chunks.get(id);
    
    if (!chunk) {
      return { 
        success: false, 
        error: new Error(`Chunk ${chunkId} not found`) 
      };
    }

    const updatedChunk = chunk.setCode(code);
    this.chunks.set(id, updatedChunk);
    
    this.projectState = this.projectState.incrementLines(
      code.split('\n').length
    );

    if (this._config.enableEvents) {
      this._eventEmitter.emit('chunkUpdated', { 
        type: 'chunk_updated', 
        chunkId: id, 
        timestamp: new Date() 
      });
    }

    return { success: true, data: undefined };
  }

  updateChunkTests(chunkId: string, tests: string): Result<void> {
    const id = this.validateChunkId(chunkId);
    const chunk = this.chunks.get(id);
    
    if (!chunk) {
      return { 
        success: false, 
        error: new Error(`Chunk ${chunkId} not found`) 
      };
    }

    const updatedChunk = chunk.setTests(tests);
    this.chunks.set(id, updatedChunk);
    
    return { success: true, data: undefined };
  }

  markChunkCompleted(chunkId: string): Result<void> {
    const id = this.validateChunkId(chunkId);
    const chunk = this.chunks.get(id);
    
    if (!chunk) {
      return { 
        success: false, 
        error: new Error(`Chunk ${chunkId} not found`) 
      };
    }

    const updatedChunk = chunk.markCompleted();
    this.chunks.set(id, updatedChunk);
    
    this.projectState = this.projectState.addCompletedModule(
      createModuleName(chunkId)
    );

    if (this._config.enableEvents) {
      this._eventEmitter.emit('chunkCompleted', { 
        type: 'chunk_completed', 
        chunkId: id, 
        timestamp: new Date() 
      });
    }

    return { success: true, data: undefined };
  }

  markChunkValidated(chunkId: string): Result<void> {
    const id = this.validateChunkId(chunkId);
    const chunk = this.chunks.get(id);
    
    if (!chunk) {
      return { 
        success: false, 
        error: new Error(`Chunk ${chunkId} not found`) 
      };
    }

    const updatedChunk = chunk.markValidated();
    this.chunks.set(id, updatedChunk);

    if (this._config.enableEvents) {
      this._eventEmitter.emit('chunkValidated', { 
        type: 'chunk_validated', 
        chunkId: id, 
        timestamp: new Date() 
      });
    }

    return { success: true, data: undefined };
  }

  getProjectSummary(): string {
    const completedCount = Array.from(this.chunks.values())
      .filter(c => c.status === ChunkStatus.Completed || c.status === ChunkStatus.Validated)
      .length;
    
    const totalCount = this.chunks.size;
    
    const pendingCount = Array.from(this.chunks.values())
      .filter(c => c.status === ChunkStatus.Pending)
      .length;
    
    const inProgressCount = Array.from(this.chunks.values())
      .filter(c => c.status === ChunkStatus.InProgress)
      .length;

    return `<project_summary>
- Total chunks: ${totalCount}
- Completed: ${completedCount}
- In progress: ${inProgressCount}
- Pending: ${pendingCount}
- Project state: ${this.projectState.toStateString()}
- Context window: ${this.contextWindow.toContextString()}
</project_summary>`;
  }

  getNextChunkToWorkOn(): ChunkId | undefined {
    return this.getReadyChunks()[0];
  }

  getChunksByLevel(level: ChunkingLevel): CodeChunk[] {
    return Array.from(this.chunks.values()).filter(c => c.level === level);
  }

  getDependencyChain(chunkId: string): string[] {
    const id = this.validateChunkId(chunkId);
    const chain: string[] = [];
    const visited = new Set<string>();
    
    this.buildDependencyChain(id, chain, visited);
    return chain;
  }

  private buildDependencyChain(
    chunkId: ChunkId, 
    chain: string[], 
    visited: Set<string>
  ): void {
    if (visited.has(chunkId)) {
      return;
    }
    
    visited.add(chunkId);
    
    const chunk = this.chunks.get(chunkId);
    if (!chunk) {
      return;
    }

    for (const dep of chunk.dependencies) {
      this.buildDependencyChain(dep, chain, visited);
    }
    
    chain.push(chunkId);
  }

  // Serialize/deserialize methods for persistence
  serialize(): string {
    const data = {
      chunks: Array.from(this.chunks.entries()),
      projectState: this.projectState,
      contextWindow: this.contextWindow,
      config: this._config
    };
    return JSON.stringify(data, (key, value) => {
      if (value instanceof Map) {
        return Array.from(value.entries());
      }
      return value;
    });
  }

  static deserialize(json: string, eventEmitter?: IEventEmitter): CodeGenerationGraph {
    const data = JSON.parse(json);
    const graph = new CodeGenerationGraph(0, data.config, eventEmitter);
    
    // Restore chunks
    const restoredChunks = new Map(
      data.chunks.map(([id, chunkData]: [string, any]) => [
        id,
        new CodeChunk(chunkData, data.config)
      ])
    );
    
    // Restore project state
    graph.projectState = new ProjectState({
      ...data.projectState,
      globalVariables: new Map(data.projectState.globalVariables)
    }, data.config);
    
    // Restore context window
    graph.contextWindow = new ContextWindow({
      ...data.contextWindow,
      functionSignatures: new Map(data.contextWindow.functionSignatures)
    }, data.config);
    
    (graph as any).chunks = restoredChunks;
    
    return graph;
  }
}

// Utility functions for parsing
export function parseChunkingFormat(chunkText: string): Result<CodeChunk> {
  const startTag = "<chunk>";
  const endTag = "</chunk>";
  
  const start = chunkText.indexOf(startTag);
  if (start === -1) {
    return { success: false, error: new Error("Missing <chunk> start tag") };
  }
  
  const end = chunkText.indexOf(endTag);
  if (end === -1) {
    return { success: false, error: new Error("Missing </chunk> end tag") };
  }
  
  const content = chunkText.substring(start + startTag.length, end);
  const parts = content.split(';').map(s => s.trim());
  
  if (parts.length < 3) {
    return { 
      success: false, 
      error: new Error("Invalid chunk format: need at least 3 parts (id; level; description)") 
    };
  }
  
  const chunkId = parts[0];
  const levelResult = ChunkingLevelMethods.fromString(parts[1]);
  
  if (!levelResult.success) {
    return levelResult;
  }
  
  const chunk = CodeChunk.new(chunkId, levelResult.data);
  
  if (parts.length > 3) {
    const dependencies = parts[3]
      .split(',')
      .map(s => s.trim())
      .filter(s => s.length > 0)
      .map(dep => createChunkId(dep));
    
    let finalChunk = chunk;
    for (const dep of dependencies) {
      finalChunk = finalChunk.addDependency(dep);
    }
    
    if (parts.length > 4) {
      return { success: true, data: finalChunk.setCode(parts[4]) };
    }
    
    return { success: true, data: finalChunk };
  }
  
  return { success: true, data: chunk };
}

export function processChunkingMessage(message: string): Result<CodeChunk[]> {
  const chunks: CodeChunk[] = [];
  const chunkPattern = /<chunk>.*?<\/chunk>/gs;
  const matches = message.match(chunkPattern);
  
  if (!matches) {
    return { success: true, data: [] };
  }
  
  for (const chunkText of matches) {
    const result = parseChunkingFormat(chunkText);
    if (result.success) {
      chunks.push(result.data);
    } else {
      console.warn(`Warning: Failed to parse chunk: ${result.error.message}`);
    }
  }
  
  return { success: true, data: chunks };
}

// Simple EventEmitter implementation
class EventEmitter implements IEventEmitter {
  private events: Map<string, Function[]> = new Map();

  on(event: string, listener: Function): void {
    if (!this.events.has(event)) {
      this.events.set(event, []);
    }
    this.events.get(event)!.push(listener);
  }

  emit(event: string, ...args: any[]): void {
    const listeners = this.events.get(event) || [];
    listeners.forEach(listener => listener(...args));
  }
}

// Export the main class as default
export default CodeGenerationGraph;
