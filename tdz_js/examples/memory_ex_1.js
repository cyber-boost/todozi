
// CLI Memory Handler Class
import { MemoryManager, parseMemoryFormat, MemoryImportance, MemoryTerm } from '../todozi/memory.js';

class MemoryCLI {
    constructor() {
        this.memoryManager = new MemoryManager();
    }
    
    // Create different types of memories
    async createEmotionalMemory(moment, meaning, reason, emotion, importance = 'medium', term = 'short', tags = []) {
        const memory = {
            moment,
            meaning,
            reason,
            importance: importance,
            term: term,
            memoryType: { type: 'Emotional', emotion },
            tags,
            userId: 'cli_user',
            projectId: null,
            status: 'Active'
        };
        
        try {
            const id = await this.memoryManager.createMemory(memory);
            console.log(`✅ Emotional memory created (ID: ${id})`);
            console.log(`   Emotion: ${emotion}`);
            console.log(`   Moment: ${moment}`);
            return id;
        } catch (error) {
            console.error(`❌ Failed to create memory: ${error.message}`);
            throw error;
        }
    }
    
    async createSecretMemory(moment, meaning, reason, importance = 'medium', term = 'long', tags = []) {
        const memory = {
            moment,
            meaning,
            reason,
            importance: importance,
            term: term,
            memoryType: { type: 'Secret' },
            tags,
            userId: 'cli_user',
            projectId: null,
            status: 'Active'
        };
        
        try {
            const id = await this.memoryManager.createMemory(memory);
            console.log(`✅ Secret memory created (ID: ${id}) - AI-Only`);
            console.log(`   Moment: ${moment}`);
            return id;
        } catch (error) {
            console.error(`❌ Failed to create secret memory: ${error.message}`);
            throw error;
        }
    }
    
    // Parse memory from text format
    parseMemoryFromText(memoryText) {
        try {
            return parseMemoryFormat(memoryText, 'cli_user');
        } catch (error) {
            console.error(`❌ Failed to parse memory text: ${error.message}`);
            throw error;
        }
    }
    
    // Search memories with various filters
    searchMemoriesWithFilters(query, filters = {}) {
        let results = [];
        
        if (query) {
            results = this.memoryManager.searchMemories(query);
        } else {
            results = this.memoryManager.getAllMemories();
        }
        
        // Apply additional filters
        if (filters.importance) {
            results = results.filter(memory => memory.importance === filters.importance);
        }
        
        if (filters.term) {
            results = results.filter(memory => memory.term === filters.term);
        }
        
        if (filters.memoryType) {
            results = results.filter(memory => {
                if (typeof filters.memoryType === 'string') {
                    return memory.memoryType.type === filters.memoryType;
                }
                return memory.memoryType.type === filters.memoryType.type && 
                      (!filters.memoryType.emotion || memory.memoryType.emotion === filters.memoryType.emotion);
            });
        }
        
        if (filters.tags && filters.tags.length > 0) {
            results = results.filter(memory => 
                filters.tags.some(tag => memory.tags.includes(tag))
            );
        }
        
        return results;
    }
    
    // Display memory statistics
    displayMemoryStatistics() {
        const stats = this.memoryManager.getMemoryStatistics();
        const tagStats = this.memoryManager.getTagStatistics();
        
        console.log('📊 Memory Statistics');
        console.log('━━━━━━━━━━━━━━━━━━━━━');
        console.log(`Total Memories: ${stats.totalMemories}`);
        console.log(`Short-term: ${stats.shortTermMemories} (${stats.shortTermPercentage().toFixed(1)}%)`);
        console.log(`Long-term: ${stats.longTermMemories} (${stats.longTermPercentage().toFixed(1)}%)`);
        console.log(`Critical: ${stats.criticalMemories} (${stats.criticalPercentage().toFixed(1)}%)`);
        console.log(`Emotional: ${stats.emotionalMemories}`);
        console.log(`Secret: ${stats.secretMemories}`);
        console.log(`Human: ${stats.humanMemories}`);
        console.log(`Standard: ${stats.standardMemories}`);
        console.log(`Unique Tags: ${stats.uniqueTags}`);
        
        console.log('\n🏷️  Tag Statistics');
        console.log('━━━━━━━━━━━━━━━━━━');
        for (const [tag, count] of tagStats.entries()) {
            console.log(`${tag}: ${count} memories`);
        }
    }
    
    // Export memories in a structured format
    exportMemories(format = 'detailed') {
        const allMemories = this.memoryManager.getAllMemories();
        
        switch (format) {
            case 'simple':
                return allMemories.map(memory => ({
                    id: memory.id.substring(0, 8),
                    type: memory.memoryType.type,
                    emotion: memory.memoryType.emotion || 'None',
                    moment: memory.moment.substring(0, 50) + (memory.moment.length > 50 ? '...' : ''),
                    importance: memory.importance,
                    term: memory.term,
                    tags: memory.tags.join(', ')
                }));
                
            case 'detailed':
                return allMemories.map(memory => ({
                    id: memory.id,
                    type: memory.memoryType,
                    moment: memory.moment,
                    meaning: memory.meaning,
                    reason: memory.reason,
                    importance: memory.importance,
                    term: memory.term,
                    tags: memory.tags,
                    created: memory.createdAt,
                    updated: memory.updatedAt
                }));
                
            default:
                return allMemories;
        }
    }
}

// Usage Examples
async function demonstrateMemoryCLI() {
    console.log('🧠 Todozi Memory Manager Demo');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
    
    const memoryCLI = new MemoryCLI();
    
    // Create emotional memories
    console.log('1. Creating Emotional Memories');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    
    await memoryCLI.createEmotionalMemory(
        "Successfully implemented the new authentication system",
        "Felt accomplished seeing the team's hard work pay off",
        "The system improved security and user experience",
        "proud",
        MemoryImportance.High,
        MemoryTerm.Long,
        ["achievement", "development", "security"]
    );
    
    await memoryCLI.createEmotionalMemory(
        "Production outage lasted 3 hours",
        "Frustrated with the deployment process",
        "Lack of proper testing caused the issue",
        "frustrated",
        MemoryImportance.Critical,
        MemoryTerm.Long,
        ["incident", "deployment", "devops"]
    );
    
    // Create secret memories (AI-only)
    console.log('\n2. Creating Secret Memories');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    
    await memoryCLI.createSecretMemory(
        "User asked about competitor's pricing strategy",
        "User is researching market competition",
        "Need to provide helpful but non-specific information",
        MemoryImportance.Medium,
        MemoryTerm.Short,
        ["competition", "pricing", "sensitive"]
    );
    
    // Parse memory from text format
    console.log('\n3. Parsing Memory from Text Format');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    
    const memoryText = `<memory>happy;Team meeting was very productive;Collaboration led to great ideas;Everyone contributed equally;high;short;meeting,collaboration,planning</memory>`;
    
    try {
        const parsedMemory = memoryCLI.parseMemoryFromText(memoryText);
        const memoryId = await memoryCLI.memoryManager.createMemory(parsedMemory);
        console.log(`✅ Parsed and created memory (ID: ${memoryId.substring(0, 8)})`);
    } catch (error) {
        console.error(`❌ Failed to parse memory text`);
    }
    
    // Search and display results
    console.log('\n4. Searching Memories');
    console.log('━━━━━━━━━━━━━━━━━━━━━━');
    
    console.log('🔍 Searching for "security" related memories:');
    const securityMemories = memoryCLI.searchMemoriesWithFilters("security");
    securityMemories.forEach((memory, index) => {
        console.log(`   ${index + 1}. ${memory.moment} (${memory.importance})`);
    });
    
    console.log('\n🔍 High importance memories:');
    const highPriorityMemories = memoryCLI.searchMemoriesWithFilters(null, {
        importance: MemoryImportance.High
    });
    highPriorityMemories.forEach((memory, index) => {
        console.log(`   ${index + 1}. ${memory.moment}`);
    });
    
    // Display statistics
    console.log('\n5. Memory Statistics');
    console.log('━━━━━━━━━━━━━━━━━━━━');
    memoryCLI.displayMemoryStatistics();
    
    // Export memories
    console.log('\n6. Exporting Memories');
    console.log('━━━━━━━━━━━━━━━━━━');
    
    const exportedMemories = memoryCLI.exportMemories('simple');
    console.log('Exported Memories (Simple Format):');
    exportedMemories.forEach((memory, index) => {
        console.log(`   ${index + 1}. [${memory.type}] ${memory.moment}`);
        console.log(`      Importance: ${memory.importance}, Tags: ${memory.tags}`);
    });
}

// Run the demonstration
demonstrateMemoryCLI().catch(console.error);

/*
## Example: Memory Manager with CLI Interface

This example demonstrates how to use the MemoryManager class from a CLI interface to create, search, and analyze different types of memories.

## Example Output:

/ *
🧠 Todozi Memory Manager Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Creating Emotional Memories
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Emotional memory created (ID: abc123de)
   Emotion: proud
   Moment: Successfully implemented the new authentication system

2. Creating Secret Memories
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Secret memory created (ID: def456gh) - AI-Only
   Moment: User asked about competitor's pricing strategy

4. Searching Memories
━━━━━━━━━━━━━━━━━━━━━━
🔍 Searching for "security" related memories:
   1. Successfully implemented the new authentication system (High)

🔍 High importance memories:
   1. Successfully implemented the new authentication system
   2. Production outage lasted 3 hours

5. Memory Statistics
━━━━━━━━━━━━━━━━━━━━
📊 Memory Statistics
━━━━━━━━━━━━━━━━━━━━━
Total Memories: 3
Short-term: 1 (33.3%)
Long-term: 2 (66.7%)
Critical: 1 (33.3%)
Emotional: 2
Secret: 1
Human: 0
Standard: 0
Unique Tags: 8
*/