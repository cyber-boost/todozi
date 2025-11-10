import { generateUUID } from './models.js';
import { cloneDeep } from 'lodash';

// Enums
export enum MemoryImportance {
    Low = 'Low',
    Medium = 'Medium',
    High = 'High',
    Critical = 'Critical'
}

export enum MemoryTerm {
    Short = 'Short',
    Long = 'Long'
}

export enum ItemStatus {
    Active = 'Active',
    Inactive = 'Inactive'
}

// Discriminated union for MemoryType
export type MemoryType = 
    | { type: 'Standard' }
    | { type: 'Secret' } 
    | { type: 'Human' }
    | { type: 'Emotional'; emotion: string };

// Helper function to create MemoryType instances
export const MemoryType = {
    Standard: (): MemoryType => ({ type: 'Standard' }),
    Secret: (): MemoryType => ({ type: 'Secret' }),
    Human: (): MemoryType => ({ type: 'Human' }),
    Emotional: (emotion: string): MemoryType => ({ type: 'Emotional', emotion })
};

// Interfaces
export interface MemoryCreateInput {
    user_id: string;
    project_id?: string;
    moment: string;
    meaning: string;
    reason: string;
    importance: MemoryImportance;
    term: MemoryTerm;
    memory_type: MemoryType;
    tags?: string[];
}

export interface Memory extends MemoryCreateInput {
    id: string;
    status: ItemStatus;
    created_at: Date;
    updated_at: Date;
    tags: string[];
}

export interface MemoryStatistics {
    total_memories: number;
    short_term_memories: number;
    long_term_memories: number;
    critical_memories: number;
    unique_tags: number;
    secret_memories: number;
    human_memories: number;
    emotional_memories: number;
    standard_memories: number;
}

// Custom Error class
export class TodoziError extends Error {
    constructor(
        message: string,
        public code?: string
    ) {
        super(message);
        this.name = 'TodoziError';
    }
}

// Type Guards
function isMemoryImportance(value: string): value is MemoryImportance {
    return Object.values(MemoryImportance).includes(value as MemoryImportance);
}

function isMemoryTerm(value: string): value is MemoryTerm {
    return Object.values(MemoryTerm).includes(value as MemoryTerm);
}

// MemoryUpdate Builder Class
export class MemoryUpdate {
    private constructor(
        public moment?: string,
        public meaning?: string,
        public reason?: string,
        public importance?: MemoryImportance,
        public term?: MemoryTerm,
        public tags?: string[]
    ) {}

    static new(): MemoryUpdate {
        return new MemoryUpdate();
    }

    withMoment(moment: string): MemoryUpdate {
        return new MemoryUpdate(
            moment,
            this.meaning,
            this.reason,
            this.importance,
            this.term,
            this.tags
        );
    }

    withMeaning(meaning: string): MemoryUpdate {
        return new MemoryUpdate(
            this.moment,
            meaning,
            this.reason,
            this.importance,
            this.term,
            this.tags
        );
    }

    withReason(reason: string): MemoryUpdate {
        return new MemoryUpdate(
            this.moment,
            this.meaning,
            reason,
            this.importance,
            this.term,
            this.tags
        );
    }

    withImportance(importance: MemoryImportance): MemoryUpdate {
        return new MemoryUpdate(
            this.moment,
            this.meaning,
            this.reason,
            importance,
            this.term,
            this.tags
        );
    }

    withTerm(term: MemoryTerm): MemoryUpdate {
        return new MemoryUpdate(
            this.moment,
            this.meaning,
            this.reason,
            this.importance,
            term,
            this.tags
        );
    }

    withTags(tags: string[]): MemoryUpdate {
        return new MemoryUpdate(
            this.moment,
            this.meaning,
            this.reason,
            this.importance,
            this.term,
            tags
        );
    }
}

// MemoryManager Class
export class MemoryManager {
    public memories: Map<string, Memory> = new Map();
    public memory_tags: Map<string, string[]> = new Map();

    static new(): MemoryManager {
        return new MemoryManager();
    }

    async createMemory(memoryInput: MemoryCreateInput): Promise<string> {
        // Input validation
        if (!memoryInput.user_id) {
            throw new TodoziError('user_id is required');
        }
        if (!memoryInput.moment || !memoryInput.meaning || !memoryInput.reason) {
            throw new TodoziError('moment, meaning, and reason are required');
        }

        const memory: Memory = {
            id: generateUUID(),
            ...memoryInput,
            status: ItemStatus.Active,
            tags: memoryInput.tags || [],
            created_at: new Date(),
            updated_at: new Date()
        };

        // Deep clone to prevent reference sharing
        this.memory_tags.set(memory.id, cloneDeep(memory.tags));
        this.memories.set(memory.id, cloneDeep(memory));
        
        return memory.id;
    }

    getMemory(memoryId: string): Memory | undefined {
        return this.memories.get(memoryId);
    }

    getAllMemories(): Memory[] {
        return Array.from(this.memories.values());
    }

    async updateMemory(
        memoryId: string,
        updates: MemoryUpdate
    ): Promise<void> {
        const memory = this.memories.get(memoryId);
        
        if (!memory) {
            throw new TodoziError(`Memory ${memoryId} not found`);
        }

        if (updates.moment !== undefined) memory.moment = updates.moment;
        if (updates.meaning !== undefined) memory.meaning = updates.meaning;
        if (updates.reason !== undefined) memory.reason = updates.reason;
        if (updates.importance !== undefined) memory.importance = updates.importance;
        if (updates.term !== undefined) memory.term = updates.term;
        
        memory.updated_at = new Date();

        if (updates.tags !== undefined) {
            memory.tags = cloneDeep(updates.tags);
            this.memory_tags.set(memoryId, cloneDeep(updates.tags));
        }

        // Update the memory in the map
        this.memories.set(memoryId, cloneDeep(memory));
    }

    async deleteMemory(memoryId: string): Promise<void> {
        if (this.memories.delete(memoryId)) {
            this.memory_tags.delete(memoryId);
        } else {
            throw new TodoziError(`Memory ${memoryId} not found`);
        }
    }

    searchMemories(query: string): Memory[] {
        const queryLower = query.toLowerCase();
        return Array.from(this.memories.values()).filter(memory => 
            memory.moment.toLowerCase().includes(queryLower) ||
            memory.meaning.toLowerCase().includes(queryLower) ||
            memory.reason.toLowerCase().includes(queryLower) ||
            memory.tags.some(tag => tag.toLowerCase().includes(queryLower))
        );
    }

    getMemoriesByImportance(importance: MemoryImportance): Memory[] {
        return Array.from(this.memories.values())
            .filter(memory => memory.importance === importance);
    }

    getMemoriesByTerm(term: MemoryTerm): Memory[] {
        return Array.from(this.memories.values())
            .filter(memory => memory.term === term);
    }

    getMemoriesByTag(tag: string): Memory[] {
        const tagLower = tag.toLowerCase();
        return Array.from(this.memories.values())
            .filter(memory => 
                memory.tags.some(t => t.toLowerCase() === tagLower)
            );
    }

    getRecentMemories(limit: number): Memory[] {
        return Array.from(this.memories.values())
            .sort((a, b) => b.created_at.getTime() - a.created_at.getTime())
            .slice(0, limit);
    }

    getCriticalMemories(): Memory[] {
        return Array.from(this.memories.values())
            .filter(memory => 
                memory.importance === MemoryImportance.High ||
                memory.importance === MemoryImportance.Critical
            );
    }

    getShortTermMemories(): Memory[] {
        return this.getMemoriesByTerm(MemoryTerm.Short);
    }

    getLongTermMemories(): Memory[] {
        return this.getMemoriesByTerm(MemoryTerm.Long);
    }

    getMemoriesByType(memoryType: MemoryType): Memory[] {
        return Array.from(this.memories.values())
            .filter(memory => {
                if (memoryType.type === 'Emotional') {
                    return memory.memory_type.type === 'Emotional' && 
                           memory.memory_type.emotion === memoryType.emotion;
                }
                return memory.memory_type.type === memoryType.type;
            });
    }

    getSecretMemories(): Memory[] {
        return this.getMemoriesByType(MemoryType.Secret());
    }

    getHumanMemories(): Memory[] {
        return this.getMemoriesByType(MemoryType.Human());
    }

    getEmotionalMemories(emotion: string): Memory[] {
        return Array.from(this.memories.values())
            .filter(memory => 
                memory.memory_type.type === 'Emotional' && 
                memory.memory_type.emotion === emotion
            );
    }

    getAllTags(): string[] {
        const allTags = new Set<string>();
        for (const tags of this.memory_tags.values()) {
            for (const tag of tags) {
                allTags.add(tag);
            }
        }
        return Array.from(allTags);
    }

    getTagStatistics(): Map<string, number> {
        const stats = new Map<string, number>();
        for (const tags of this.memory_tags.values()) {
            for (const tag of tags) {
                stats.set(tag, (stats.get(tag) || 0) + 1);
            }
        }
        return stats;
    }

    getMemoryStatistics(): MemoryStatistics {
        const totalMemories = this.memories.size;
        const shortTerm = this.getShortTermMemories().length;
        const longTerm = this.getLongTermMemories().length;
        const critical = this.getCriticalMemories().length;
        const uniqueTags = this.getAllTags().length;
        const secret = this.getSecretMemories().length;
        const human = this.getHumanMemories().length;
        const standard = this.getMemoriesByType(MemoryType.Standard()).length;
        
        // Efficient emotional memories counting - O(n) instead of O(n*m)
        const emotional = Array.from(this.memories.values()).filter(memory => 
            memory.memory_type.type === 'Emotional'
        ).length;

        return {
            total_memories: totalMemories,
            short_term_memories: shortTerm,
            long_term_memories: longTerm,
            critical_memories: critical,
            unique_tags: uniqueTags,
            secret_memories: secret,
            human_memories: human,
            emotional_memories: emotional,
            standard_memories: standard
        };
    }
}

// Enhanced MemoryStatistics with percentage calculations
export class EnhancedMemoryStatistics implements MemoryStatistics {
    total_memories: number;
    short_term_memories: number;
    long_term_memories: number;
    critical_memories: number;
    unique_tags: number;
    secret_memories: number;
    human_memories: number;
    emotional_memories: number;
    standard_memories: number;

    constructor(stats: MemoryStatistics) {
        Object.assign(this, stats);
    }

    shortTermPercentage(): number {
        return this.total_memories === 0 ? 0 : 
            (this.short_term_memories / this.total_memories) * 100;
    }

    longTermPercentage(): number {
        return this.total_memories === 0 ? 0 : 
            (this.long_term_memories / this.total_memories) * 100;
    }

    criticalPercentage(): number {
        return this.total_memories === 0 ? 0 : 
            (this.critical_memories / this.total_memories) * 100;
    }
}

// Enhanced parseMemoryFormat function with better error handling
export function parseMemoryFormat(memoryText: string, userId: string): Memory {
    const startTag = '<memory>';
    const endTag = '</memory>';
    
    const startIndex = memoryText.indexOf(startTag);
    if (startIndex === -1) {
        throw new TodoziError('Missing <memory> start tag');
    }
    
    const endIndex = memoryText.indexOf(endTag, startIndex);
    if (endIndex === -1) {
        throw new TodoziError('Missing </memory> end tag');
    }

    const content = memoryText.substring(startIndex + startTag.length, endIndex);
    const parts = content.split(';').map(s => s.trim()).filter(s => s.length > 0);

    if (parts.length < 6) {
        throw new TodoziError(
            'Invalid memory format: need at least 6 parts (type; moment; meaning; reason; importance; term)'
        );
    }

    const emotionList = [
        'happy', 'sad', 'angry', 'fearful', 'surprised', 'disgusted', 'excited',
        'anxious', 'confident', 'frustrated', 'motivated', 'overwhelmed', 'curious',
        'satisfied', 'disappointed', 'grateful', 'proud', 'ashamed', 'hopeful',
        'resigned'
    ];

    const memoryTypeStr = parts[0].toLowerCase();
    let memoryType: MemoryType;

    if (emotionList.includes(memoryTypeStr)) {
        memoryType = MemoryType.Emotional(memoryTypeStr);
    } else {
        switch (memoryTypeStr) {
            case 'standard':
                memoryType = MemoryType.Standard();
                break;
            case 'secret':
                memoryType = MemoryType.Secret();
                break;
            case 'human':
                memoryType = MemoryType.Human();
                break;
            default:
                memoryType = MemoryType.Standard();
        }
    }

    // Validate importance
    if (!isMemoryImportance(parts[4])) {
        throw new TodoziError(`Invalid memory importance: ${parts[4]}`);
    }

    // Validate term
    if (!isMemoryTerm(parts[5])) {
        throw new TodoziError(`Invalid memory term: ${parts[5]}`);
    }

    const tags = parts.length > 6 && parts[6].length > 0 
        ? parts[6].split(',').map(s => s.trim()).filter(s => s.length > 0)
        : [];

    return {
        id: generateUUID(),
        user_id: userId,
        project_id: undefined,
        status: ItemStatus.Active,
        moment: parts[1],
        meaning: parts[2],
        reason: parts[3],
        importance: parts[4] as MemoryImportance,
        term: parts[5] as MemoryTerm,
        memory_type: memoryType,
        tags,
        created_at: new Date(),
        updated_at: new Date()
    };
}

// Test Suite (using Jest-like syntax for clarity)
export const testMemoryManager = () => {
    console.log('Testing MemoryManager...');
    
    // Test 1: Memory Manager Creation
    const manager = MemoryManager.new();
    console.assert(manager.memories.size === 0, 'Manager should start empty');
    console.assert(manager.memory_tags.size === 0, 'Tags should start empty');
    
    // Test 2: Memory Update Builder
    const update = MemoryUpdate.new()
        .withMoment('New moment')
        .withMeaning('New meaning')
        .withImportance(MemoryImportance.High);
    
    console.assert(update.moment === 'New moment', 'Builder should set moment');
    console.assert(update.meaning === 'New meaning', 'Builder should set meaning');
    console.assert(update.importance === MemoryImportance.High, 'Builder should set importance');
    
    // Test 3: Memory Statistics Percentages
    const stats = new EnhancedMemoryStatistics({
        total_memories: 10,
        short_term_memories: 6,
        long_term_memories: 4,
        critical_memories: 2,
        unique_tags: 8,
        secret_memories: 1,
        human_memories: 2,
        emotional_memories: 3,
        standard_memories: 4
    });
    
    console.assert(stats.shortTermPercentage() === 60, 'Short term percentage should be 60%');
    console.assert(stats.longTermPercentage() === 40, 'Long term percentage should be 40%');
    console.assert(stats.criticalPercentage() === 20, 'Critical percentage should be 20%');
    
    const emptyStats = new EnhancedMemoryStatistics({
        total_memories: 0,
        short_term_memories: 0,
        long_term_memories: 0,
        critical_memories: 0,
        unique_tags: 0,
        secret_memories: 0,
        human_memories: 0,
        emotional_memories: 0,
        standard_memories: 0
    });
    
    console.assert(emptyStats.shortTermPercentage() === 0, 'Empty stats should return 0%');
    console.assert(emptyStats.longTermPercentage() === 0, 'Empty stats should return 0%');
    console.assert(emptyStats.criticalPercentage() === 0, 'Empty stats should return 0%');
    
    // Test 4: Parse Memory Format
    const memoryText = '<memory>standard; 2025-01-13 10:30 AM; Client prefers iterative development; Affects testing cycle; high; long; client,development,iterative</memory>';
    const memory = parseMemoryFormat(memoryText, 'user_123');
    
    console.assert(memory.moment === '2025-01-13 10:30 AM', 'Moment should be parsed correctly');
    console.assert(memory.meaning === 'Client prefers iterative development', 'Meaning should be parsed correctly');
    console.assert(memory.reason === 'Affects testing cycle', 'Reason should be parsed correctly');
    console.assert(memory.importance === MemoryImportance.High, 'Importance should be parsed correctly');
    console.assert(memory.term === MemoryTerm.Long, 'Term should be parsed correctly');
    console.assert(memory.memory_type.type === 'Standard', 'Type should be Standard');
    console.assert(memory.tags.length === 3, 'Should have 3 tags');
    
    console.log('All tests passed!');
};
