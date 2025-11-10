
// Create a new instance of MemoryManager
import { MemoryManager, MemoryImportance, MemoryTerm } from './memory.js';

const memoryManager = new MemoryManager();

// Function to demonstrate creating different types of memories
async function demonstrateMemoryCreation() {
  console.log('🧠 Creating memories...\n');
  
  // Standard memory
  const standardMemoryId = await memoryManager.createMemory({
    moment: 'Meeting with the development team about the new project',
    meaning: 'We discussed technical requirements and made important decisions',
    reason: 'To ensure everyone is aligned on the project direction',
    importance: MemoryImportance.High,
    term: MemoryTerm.Short,
    memoryType: { type: 'Standard' },
    tags: ['work', 'meeting', 'planning']
  });
  console.log(`Standard memory created with ID: ${standardMemoryId}`);

  // Emotional memory
  const emotionalMemoryId = await memoryManager.createMemory({
    moment: 'Received positive feedback on my presentation',
    meaning: 'I felt validated and recognized for my work',
    reason: 'My presentation skills are improving',
    importance: MemoryImportance.Medium,
    term: MemoryTerm.Long,
    memoryType: { type: 'Emotional', emotion: 'happy' },
    tags: ['work', 'recognition', 'personal-growth']
  });
  console.log(`Emotional memory created with ID: ${emotionalMemoryId}`);

  // Secret memory (AI-only)
  const secretMemoryId = await memoryManager.createMemory({
    moment: 'The user seems stressed about deadlines',
    meaning: 'User workload is increasing, need to offer more help',
    reason: 'To provide better support and anticipate needs',
    importance: MemoryImportance.High,
    term: MemoryTerm.Short,
    memoryType: { type: 'Secret' },
    tags: ['user', 'stress', 'support']
  });
  console.log(`Secret memory created with ID: ${secretMemoryId}`);

  // Human-visible memory
  const humanMemoryId = await memoryManager.createMemory({
    moment: 'User mentioned they enjoy working on creative projects',
    meaning: 'Creative work is a source of satisfaction for the user',
    reason: 'To remember their interests for future task suggestions',
    importance: MemoryImportance.Medium,
    term: MemoryTerm.Long,
    memoryType: { type: 'Human' },
    tags: ['interests', 'creativity', 'satisfaction']
  });
  console.log(`Human-visible memory created with ID: ${humanMemoryId}\n`);
  
  return {
    standardMemoryId,
    emotionalMemoryId,
    secretMemoryId,
    humanMemoryId
  };
}

// Function to demonstrate searching and filtering memories
async function demonstrateMemorySearch() {
  console.log('🔍 Searching for memories...\n');
  
  // Search by query
  console.log('🔎 Memories containing "team":');
  const teamMemories = memoryManager.searchMemories('team');
  console.log(`Found ${teamMemories.length} memories`);
  teamMemories.forEach(memory => {
    console.log(`  - ${memory.moment} (${memory.importance})`);
  });
  console.log();

  // Get memories by importance
  console.log('📊 High importance memories:');
  const highImportanceMemories = memoryManager.getMemoriesByImportance(MemoryImportance.High);
  console.log(`Found ${highImportanceMemories.length} memories`);
  highImportanceMemories.forEach(memory => {
    console.log(`  - ${memory.moment} (${memory.term})`);
  });
  console.log();

  // Get memories by term
  console.log('⏳ Short-term memories:');
  const shortTermMemories = memoryManager.getShortTermMemories();
  console.log(`Found ${shortTermMemories.length} memories`);
  shortTermMemories.forEach(memory => {
    console.log(`  - ${memory.moment} (${memory.importance})`);
  });
  console.log();

  // Get memories by tag
  console.log('🏷️ Memories tagged with "work":');
  const workMemories = memoryManager.getMemoriesByTag('work');
  console.log(`Found ${workMemories.length} memories`);
  workMemories.forEach(memory => {
    console.log(`  - ${memory.moment} (${memory.term})`);
  });
  console.log();

  // Get memories by type
  console.log('🎭 Emotional memories (happy):');
  const happyMemories = memoryManager.getEmotionalMemories('happy');
  console.log(`Found ${happyMemories.length} memories`);
  happyMemories.forEach(memory => {
    console.log(`  - ${memory.moment} (${memory.meaning})`);
  });
  console.log();

  // Get recent memories
  console.log('🕒 Recent memories (limit 2):');
  const recentMemories = memoryManager.getRecentMemories(2);
  console.log(`Found ${recentMemories.length} memories`);
  recentMemories.forEach(memory => {
    console.log(`  - ${memory.moment} (${memory.createdAt})`);
  });
  console.log();
}

// Function to demonstrate updating memories
async function demonstrateMemoryUpdate(memoryIds) {
  console.log('✏️ Updating a memory...\n');
  
  // Update a memory
  try {
    await memoryManager.updateMemory(memoryIds.standardMemoryId, {
      moment: 'Meeting with the development team about the new project',
      meaning: 'We discussed technical requirements, made important decisions, and set a timeline',
      tags: ['work', 'meeting', 'planning', 'deadline']
    });
    console.log(`Memory ${memoryIds.standardMemoryId} updated successfully`);
    
    // Get the updated memory
    const updatedMemory = memoryManager.getMemory(memoryIds.standardMemoryId);
    console.log(`Updated moment: ${updatedMemory.moment}`);
    console.log(`Updated meaning: ${updatedMemory.meaning}`);
    console.log(`Updated tags: ${updatedMemory.tags.join(', ')}`);
    console.log();
  } catch (error) {
    console.error(`Error updating memory: ${error.message}`);
  }
}

// Function to demonstrate memory statistics
async function demonstrateMemoryStatistics() {
  console.log('📈 Memory Statistics:\n');
  
  // Get all tags
  console.log('🏷️ All tags:');
  const allTags = memoryManager.getAllTags();
  console.log(`Found ${allTags.length} unique tags: ${allTags.join(', ')}`);
  console.log();

  // Get tag statistics
  console.log('📊 Tag statistics:');
  const tagStats = memoryManager.getTagStatistics();
  tagStats.forEach((count, tag) => {
    console.log(`  ${tag}: ${count} memories`);
  });
  console.log();

  // Get memory statistics
  console.log('🧮 Overall memory statistics:');
  const memoryStats = memoryManager.getMemoryStatistics();
  console.log(`Total memories: ${memoryStats.totalMemories}`);
  console.log(`Short-term: ${memoryStats.shortTermMemories} (${memoryStats.shortTermPercentage().toFixed(1)}%)`);
  console.log(`Long-term: ${memoryStats.longTermMemories} (${memoryStats.longTermPercentage().toFixed(1)}%)`);
  console.log(`Critical: ${memoryStats.criticalMemories} (${memoryStats.criticalPercentage().toFixed(1)}%)`);
  console.log(`Secret: ${memoryStats.secretMemories}`);
  console.log(`Human: ${memoryStats.humanMemories}`);
  console.log(`Emotional: ${memoryStats.emotionalMemories}`);
  console.log();
}

// Function to demonstrate memory deletion
async function demonstrateMemoryDeletion(memoryIds) {
  console.log('🗑️ Deleting a memory...\n');
  
  try {
    await memoryManager.deleteMemory(memoryIds.secretMemoryId);
    console.log(`Memory ${memoryIds.secretMemoryId} deleted successfully`);
    
    // Verify deletion
    try {
      const deletedMemory = memoryManager.getMemory(memoryIds.secretMemoryId);
      console.log(`Error: Memory still exists: ${deletedMemory}`);
    } catch (error) {
      console.log(`Confirmed: Memory with ID ${memoryIds.secretMemoryId} no longer exists`);
    }
    console.log();
  } catch (error) {
    console.error(`Error deleting memory: ${error.message}`);
  }
}

// Main function to run all demonstrations
async function runExample() {
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('    Memory Management System Example     ');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  
  try {
    // Create memories
    const memoryIds = await demonstrateMemoryCreation();
    
    // Search and filter memories
    await demonstrateMemorySearch();
    
    // Update a memory
    await demonstrateMemoryUpdate(memoryIds);
    
    // Show memory statistics
    await demonstrateMemoryStatistics();
    
    // Delete a memory
    await demonstrateMemoryDeletion(memoryIds);
    
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('           Example Complete              ');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  } catch (error) {
    console.error(`Example failed: ${error.message}`);
  }
}

// Run the example
runExample();

/*
# Memory Management System - Example Implementation

In this example, we'll demonstrate how to use the Memory Management System to create, manage, and query different types of memories. This could be useful for an AI assistant to keep track of important information, emotional states, and experiences.

## MemoryManagerExample.js

## How to Use This Example

To run this example, save the code to a file called `MemoryManagerExample.js` and then execute it with Node.js:

/ *
bash
node MemoryManagerExample.js

/ *
## Expected Output

When you run this example, you should see output similar to this:
*/