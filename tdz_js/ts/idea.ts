// Enums and types
enum ShareLevel {
  Public = 'public',
  Team = 'team',
  Private = 'private'
}

enum IdeaImportance {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Breakthrough = 'breakthrough'
}

enum ItemStatus {
  Active = 'active',
  Completed = 'completed',
  Archived = 'archived'
}

interface Idea {
  id: string;
  idea: string;
  project_id?: string;
  status: ItemStatus;
  share: ShareLevel;
  importance: IdeaImportance;
  tags: string[];
  context?: string;
  created_at: Date;
  updated_at: Date;
}

class TodoziError extends Error {
  constructor(
    public message: string,
    public type: 'ValidationError' = 'ValidationError'
  ) {
    super(message);
    this.name = 'TodoziError';
  }
}

type Result<T> = T | TodoziError;

class IdeaManager {
  public ideas: Map<string, Idea>;
  public ideaTags: Map<string, string[]>;

  constructor() {
    this.ideas = new Map();
    this.ideaTags = new Map();
  }

  async createIdea(idea: Idea): Promise<Result<string>> {
    const newIdea = {
      ...idea,
      id: crypto.randomUUID(),
      created_at: new Date(),
      updated_at: new Date()
    };
    
    this.ideaTags.set(newIdea.id, [...newIdea.tags]);
    this.ideas.set(newIdea.id, newIdea);
    
    return newIdea.id;
  }

  getIdea(ideaId: string): Idea | undefined {
    return this.ideas.get(ideaId);
  }

  getAllIdeas(): Idea[] {
    return Array.from(this.ideas.values());
  }

  async updateIdea(
    ideaId: string,
    updates: IdeaUpdate
  ): Promise<Result<void>> {
    const idea = this.ideas.get(ideaId);
    
    if (!idea) {
      return new TodoziError(`Idea ${ideaId} not found`);
    }
    
    if (updates.idea !== undefined) {
      idea.idea = updates.idea;
    }
    
    if (updates.share !== undefined) {
      idea.share = updates.share;
    }
    
    if (updates.importance !== undefined) {
      idea.importance = updates.importance;
    }
    
    if (updates.tags !== undefined) {
      idea.tags = [...updates.tags];
      this.ideaTags.set(ideaId, [...updates.tags]);
    }
    
    if (updates.context !== undefined) {
      idea.context = updates.context;
    }
    
    idea.updated_at = new Date();
    
    return undefined;
  }

  async deleteIdea(ideaId: string): Promise<Result<void>> {
    const deleted = this.ideas.delete(ideaId);
    
    if (deleted) {
      this.ideaTags.delete(ideaId);
      return undefined;
    } else {
      return new TodoziError(`Idea ${ideaId} not found`);
    }
  }

  searchIdeas(query: string): Idea[] {
    const queryLower = query.toLowerCase();
    
    return Array.from(this.ideas.values()).filter(idea => {
      const matchesText = idea.idea.toLowerCase().includes(queryLower);
      const matchesTags = idea.tags.some(tag => 
        tag.toLowerCase().includes(queryLower)
      );
      const matchesContext = idea.context?.toLowerCase().includes(queryLower) ?? false;
      
      return matchesText || matchesTags || matchesContext;
    });
  }

  getIdeasByImportance(importance: IdeaImportance): Idea[] {
    return Array.from(this.ideas.values()).filter(idea => 
      idea.importance === importance
    );
  }

  getIdeasByShareLevel(shareLevel: ShareLevel): Idea[] {
    return Array.from(this.ideas.values()).filter(idea => 
      idea.share === shareLevel
    );
  }

  getIdeasByTag(tag: string): Idea[] {
    const tagLower = tag.toLowerCase();
    
    return Array.from(this.ideas.values()).filter(idea => 
      idea.tags.some(t => t.toLowerCase() === tagLower)
    );
  }

  getPublicIdeas(): Idea[] {
    return this.getIdeasByShareLevel(ShareLevel.Public);
  }

  getTeamIdeas(): Idea[] {
    return this.getIdeasByShareLevel(ShareLevel.Team);
  }

  getPrivateIdeas(): Idea[] {
    return this.getIdeasByShareLevel(ShareLevel.Private);
  }

  getBreakthroughIdeas(): Idea[] {
    return this.getIdeasByImportance(IdeaImportance.Breakthrough);
  }

  getRecentIdeas(limit: number): Idea[] {
    return Array.from(this.ideas.values())
      .sort((a, b) => b.created_at.getTime() - a.created_at.getTime())
      .slice(0, limit);
  }

  getAllTags(): string[] {
    const allTags = new Set<string>();
    
    for (const tags of this.ideaTags.values()) {
      for (const tag of tags) {
        allTags.add(tag);
      }
    }
    
    return Array.from(allTags);
  }

  getTagStatistics(): Record<string, number> {
    const stats: Record<string, number> = {};
    
    for (const tags of this.ideaTags.values()) {
      for (const tag of tags) {
        stats[tag] = (stats[tag] || 0) + 1;
      }
    }
    
    return stats;
  }

  getIdeaStatistics(): IdeaStatistics {
    const totalIdeas = this.ideas.size;
    const publicIdeas = this.getPublicIdeas().length;
    const teamIdeas = this.getTeamIdeas().length;
    const privateIdeas = this.getPrivateIdeas().length;
    const breakthroughIdeas = this.getBreakthroughIdeas().length;
    const uniqueTags = this.getAllTags().length;
    
    return new IdeaStatistics({
      totalIdeas,
      publicIdeas,
      teamIdeas,
      privateIdeas,
      breakthroughIdeas,
      uniqueTags
    });
  }
}

class IdeaUpdate {
  public idea?: string;
  public share?: ShareLevel;
  public importance?: IdeaImportance;
  public tags?: string[];
  public context?: string;

  constructor() {}

  static new(): IdeaUpdate {
    return new IdeaUpdate();
  }

  setIdea(idea: string): IdeaUpdate {
    this.idea = idea;
    return this;
  }

  setShare(share: ShareLevel): IdeaUpdate {
    this.share = share;
    return this;
  }

  setImportance(importance: IdeaImportance): IdeaUpdate {
    this.importance = importance;
    return this;
  }

  setTags(tags: string[]): IdeaUpdate {
    this.tags = tags;
    return this;
  }

  setContext(context: string): IdeaUpdate {
    this.context = context;
    return this;
  }
}

class IdeaStatistics {
  public totalIdeas: number;
  public publicIdeas: number;
  public teamIdeas: number;
  public privateIdeas: number;
  public breakthroughIdeas: number;
  public uniqueTags: number;

  constructor(data: {
    totalIdeas: number;
    publicIdeas: number;
    teamIdeas: number;
    privateIdeas: number;
    breakthroughIdeas: number;
    uniqueTags: number;
  }) {
    this.totalIdeas = data.totalIdeas;
    this.publicIdeas = data.publicIdeas;
    this.teamIdeas = data.teamIdeas;
    this.privateIdeas = data.privateIdeas;
    this.breakthroughIdeas = data.breakthroughIdeas;
    this.uniqueTags = data.uniqueTags;
  }

  publicPercentage(): number {
    if (this.totalIdeas === 0) return 0;
    return (this.publicIdeas / this.totalIdeas) * 100;
  }

  teamPercentage(): number {
    if (this.totalIdeas === 0) return 0;
    return (this.teamIdeas / this.totalIdeas) * 100;
  }

  privatePercentage(): number {
    if (this.totalIdeas === 0) return 0;
    return (this.privateIdeas / this.totalIdeas) * 100;
  }

  breakthroughPercentage(): number {
    if (this.totalIdeas === 0) return 0;
    return (this.breakthroughIdeas / this.totalIdeas) * 100;
  }
}

function parseIdeaFormat(ideaText: string): Result<Idea> {
  const startTag = "<idea>";
  const endTag = "</idea>";
  
  const start = ideaText.indexOf(startTag);
  if (start === -1) {
    return new TodoziError("Missing <idea> start tag");
  }
  
  const end = ideaText.indexOf(endTag);
  if (end === -1) {
    return new TodoziError("Missing </idea> end tag");
  }
  
  const content = ideaText.substring(start + startTag.length, end);
  const parts = content.split(';').map(s => s.trim());
  
  if (parts.length < 3) {
    return new TodoziError(
      "Invalid idea format: need at least 3 parts (idea; share; importance)"
    );
  }
  
  let share: ShareLevel;
  switch (parts[1].toLowerCase()) {
    case "share":
      share = ShareLevel.Public;
      break;
    case "dont share":
    case "don't share":
    case "private":
      share = ShareLevel.Private;
      break;
    case "team":
      share = ShareLevel.Team;
      break;
    default:
      share = ShareLevel.Private;
  }
  
  const tags = parts.length > 3 && parts[3] !== "" 
    ? parts[3].split(',').map(s => s.trim())
    : [];
  
  const context = parts.length > 4 && parts[4] !== "" 
    ? parts[4] 
    : undefined;
  
  let importance: IdeaImportance;
  switch (parts[2].toLowerCase()) {
    case "low":
      importance = IdeaImportance.Low;
      break;
    case "medium":
      importance = IdeaImportance.Medium;
      break;
    case "high":
      importance = IdeaImportance.High;
      break;
    case "breakthrough":
      importance = IdeaImportance.Breakthrough;
      break;
    default:
      return new TodoziError("Invalid idea importance");
  }
  
  return {
    id: crypto.randomUUID(),
    idea: parts[0],
    status: ItemStatus.Active,
    share,
    importance,
    tags,
    context,
    created_at: new Date(),
    updated_at: new Date()
  };
}

// Tests
function testIdeaManagerCreation() {
  const manager = new IdeaManager();
  console.assert(manager.ideas.size === 0);
  console.assert(manager.ideaTags.size === 0);
  console.log("testIdeaManagerCreation passed");
}

function testIdeaUpdateBuilder() {
  const update = IdeaUpdate.new()
    .setIdea("New idea")
    .setShare(ShareLevel.Public)
    .setImportance(IdeaImportance.High);
    
  console.assert(update.idea === "New idea");
  console.assert(update.share === ShareLevel.Public);
  console.assert(update.importance === IdeaImportance.High);
  console.log("testIdeaUpdateBuilder passed");
}

function testIdeaStatisticsPercentages() {
  const stats = new IdeaStatistics({
    totalIdeas: 10,
    publicIdeas: 4,
    teamIdeas: 3,
    privateIdeas: 3,
    breakthroughIdeas: 2,
    uniqueTags: 8
  });
  
  console.assert(stats.publicPercentage() === 40.0);
  console.assert(stats.teamPercentage() === 30.0);
  console.assert(stats.privatePercentage() === 30.0);
  console.assert(stats.breakthroughPercentage() === 20.0);
  
  const emptyStats = new IdeaStatistics({
    totalIdeas: 0,
    publicIdeas: 0,
    teamIdeas: 0,
    privateIdeas: 0,
    breakthroughIdeas: 0,
    uniqueTags: 0
  });
  
  console.assert(emptyStats.publicPercentage() === 0.0);
  console.assert(emptyStats.teamPercentage() === 0.0);
  console.assert(emptyStats.privatePercentage() === 0.0);
  console.assert(emptyStats.breakthroughPercentage() === 0.0);
  console.log("testIdeaStatisticsPercentages passed");
}

function testParseIdeaFormat() {
  const ideaText = "<idea>Use microservices for better scalability; share; high; architecture,microservices,scalability; This will improve deployment speed</idea>";
  const result = parseIdeaFormat(ideaText);
  
  if (result instanceof TodoziError) {
    console.error("Failed to parse idea format");
    return;
  }
  
  console.assert(result.idea === "Use microservices for better scalability");
  console.assert(result.share === ShareLevel.Public);
  console.assert(result.importance === IdeaImportance.High);
  console.assert(result.tags.length === 3);
  console.assert(result.tags[0] === "architecture");
  console.assert(result.tags[1] === "microservices");
  console.assert(result.tags[2] === "scalability");
  console.assert(result.context === "This will improve deployment speed");
  console.log("testParseIdeaFormat passed");
}

function testParseIdeaFormatMinimal() {
  const ideaText = "<idea>Simple idea; private; low</idea>";
  const result = parseIdeaFormat(ideaText);
  
  if (result instanceof TodoziError) {
    console.error("Failed to parse idea format");
    return;
  }
  
  console.assert(result.idea === "Simple idea");
  console.assert(result.share === ShareLevel.Private);
  console.assert(result.importance === IdeaImportance.Low);
  console.assert(result.tags.length === 0);
  console.assert(result.context === undefined);
  console.log("testParseIdeaFormatMinimal passed");
}

// Run tests
testIdeaManagerCreation();
testIdeaUpdateBuilder();
testIdeaStatisticsPercentages();
testParseIdeaFormat();
testParseIdeaFormatMinimal();
