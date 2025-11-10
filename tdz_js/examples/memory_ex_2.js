// example2.js

import { MemoryManager, MemoryImportance, MemoryTerm } from '../todozi/memory.js';

async function runMemoryExample() {
    // Initialize memory manager
    const memoryManager = new MemoryManager();
    
    // Create different types of memories
    console.log("Creating memories...");
    
    // 1. Standard memory
    const standardMemoryId = await memoryManager.createMemory({
        moment: "Implemented user authentication",
        meaning: "Security feature for user accounts",
        reason: "Required for user-specific data protection",
        importance: MemoryImportance.High,
        term: MemoryTerm.Long,
        memoryType: { type: 'Standard' },
        tags: ['security', 'authentication', 'backend']
    });
    console.log(`Created standard memory with ID: ${standardMemoryId}`);
    
    // 2. Emotional memory
    const emotionalMemoryId = await memoryManager.createMemory({
        moment: "User expressed frustration with login process",
        meaning: "User experience needs improvement",
        reason: "High support ticket volume",
        importance: MemoryImportance.Critical,
        term: MemoryTerm.Short,
        memoryType: { type: 'Emotional', emotion: 'frustrated' },
        tags: ['ux', 'support', 'login']
    });
    console.log(`Created emotional memory with ID: ${emotionalMemoryId}`);
    
    // 3. Secret memory (AI-only)
    const secretMemoryId = await memoryManager.createMemory({
        moment: "Identified potential security vulnerability",
        meaning: "System weakness in password reset flow",
        reason: "Code review findings",
        importance: MemoryImportance.Critical,
        term: MemoryTerm.Short,
        memoryType: { type: 'Secret' },
        tags: ['security', 'vulnerability', 'password']
    });
    console.log(`Created secret memory with ID: ${secretMemoryId}`);
    
    // 4. Human-visible memory
    const humanMemoryId = await memoryManager.createMemory({
        moment: "Team agreed on new design system",
        meaning: "Standardized UI components for consistency",
        reason: "Inconsistent user interfaces across products",
        importance: MemoryImportance.Medium,
        term: MemoryTerm.Long,
        memoryType: { type: 'Human' },
        tags: ['design', 'ui', 'standards']
    });
    console.log(`Created human memory with ID: ${humanMemoryId}`);
    
    // Search memories
    console.log("\nSearching memories for 'security':");
    const securityMemories = memoryManager.searchMemories('security');
    securityMemories.forEach(memory => {
        console.log(`- ${memory.moment} (${memory.memoryType.type})`);
    });
    
    // Get memories by importance
    console.log("\nCritical importance memories:");
    const criticalMemories = memoryManager.getMemoriesByImportance(MemoryImportance.Critical);
    criticalMemories.forEach(memory => {
        console.log(`- ${memory.moment} (${memory.importance})`);
    });
    
    // Get memories by tag
    console.log("\nMemories tagged with 'ux':");
    const uxMemories = memoryManager.getMemoriesByTag('ux');
    uxMemories.forEach(memory => {
        console.log(`- ${memory.moment}`);
    });
    
    // Update a memory
    console.log("\nUpdating memory...");
    await memoryManager.updateMemory(standardMemoryId, {
        meaning: "Enhanced security feature with MFA support",
        tags: ['security', 'authentication', 'backend', 'mfa']
    });
    
    const updatedMemory = memoryManager.getMemory(standardMemoryId);
    console.log(`Updated memory: ${updatedMemory.meaning}`);
    console.log(`Tags: ${updatedMemory.tags.join(', ')}`);
    
    // Get memory statistics
    console.log("\nMemory Statistics:");
    const stats = memoryManager.getMemoryStatistics();
    console.log(`Total memories: ${stats.totalMemories}`);
    console.log(`Critical memories: ${stats.criticalMemories}`);
    console.log(`Unique tags: ${stats.uniqueTags}`);
    console.log(`Secret memories: ${stats.secretMemories}`);
    
    // Get tag statistics
    console.log("\nTag Usage:");
    const tagStats = memoryManager.getTagStatistics();
    for (const [tag, count] of tagStats) {
        console.log(`${tag}: ${count} memories`);
    }
    
    // Get recent memories
    console.log("\nMost recent memory:");
    const recent = memoryManager.getRecentMemories(1);
    console.log(`${recent[0].moment} (${new Date(recent[0].createdAt).toLocaleString()})`);
}

// Run the example
runMemoryExample().catch(console.error);

/*
Here's a practical example demonstrating how to use the MemoryManager class to create, search, and manage memories:

This example demonstrates:

1. **Creating different memory types**:
   - Standard memories
   - Emotional memories with specific emotions
   - Secret memories (AI-only)
   - Human-visible memories

2. **Memory management operations**:
   - Creating memories with full metadata
   - Searching memories by content
   - Filtering by importance level
   - Retrieving memories by tags
   - Updating existing memories

3. **Analytics capabilities**:
   - Getting memory statistics
   - Analyzing tag usage
   - Retrieving recent memories

Key features shown:
- Proper use of memory types and subtypes
- Tag management and search
- Importance and term categorization
- Memory updates with partial data
- Statistical analysis of memory collections
- Error handling for missing memories

To run this example:
1. Save as `example2.js`
2. Ensure `uuid` package is installed (`npm install uuid`)
3. Run with `node example2.js`

The output will show:
- Created memory IDs
- Search results for different criteria
- Updated memory details
- Statistical summaries
- Tag usage reports
- Recent memory identification

This demonstrates the core functionality of the MemoryManager for organizing and retrieving different types of memories in an AI system.
*/