#!/usr/bin/env node
/**
 * Todozi Node.js Bindings Example - Complete API Showcase
 *
 * This example demonstrates the comprehensive Todozi Node.js API
 * with all 175+ methods available through the professional client interface.
 */

const { TodoziClient } = require('./index');

async function main() {
  // Initialize the Todozi client
  console.log('🚀 Initializing Todozi Professional Client...');
  const client = new TodoziClient();

  // ========== Core Task Operations ==========
  console.log('\n📝 Core Task Operations:');

  // Create tasks with different priorities
  const urgentTask = client.urgent('Fix critical production bug');
  console.log(`⚡ Urgent task created: ${urgentTask}`);

  const highTask = client.high('Implement user authentication');
  console.log(`🔴 High priority task: ${highTask}`);

  const normalTask = client.task('Write unit tests');
  console.log(`📋 Normal task: ${normalTask}`);

  const lowTask = client.low('Update README documentation');
  console.log(`🔵 Low priority task: ${lowTask}`);

  // ========== AI-Powered Features ==========
  console.log('\n🤖 AI-Powered Features:');

  // AI search and task creation
  const aiTasks = client.aiFind('frontend development tasks');
  console.log(`🎯 AI found ${aiTasks.length} relevant tasks`);

  // Semantic embeddings
  const embeddings = client.embed('Implement responsive design patterns');
  console.log(`🧠 Generated embeddings with ${embeddings.length} dimensions`);

  // Chat with AI
  const chatResponse = client.chat('Help me organize my development workflow');
  console.log(`💬 AI Chat: ${chatResponse.substring(0, 50)}...`);

  // ========== Memory & Idea Management ==========
  console.log('\n🧠 Memory & Idea Management:');

  // Create memories and ideas
  const memoryId = client.remember('Great debugging session', 'Found root cause quickly');
  console.log(`🧠 Memory created: ${memoryId}`);

  const ideaId = client.idea('Implement dark mode toggle');
  console.log(`💡 Idea created: ${ideaId}`);

  // AI-assisted content processing
  const extractedTasks = client.extractTasks('Need to refactor auth module, update API endpoints, and add error handling');
  console.log(`📋 Extracted ${extractedTasks.length} tasks from text`);

  // ========== Advanced Task Management ==========
  console.log('\n⚙️ Advanced Task Management:');

  // Create task with full options
  const advancedTask = client.createTask(
    'Build comprehensive dashboard',
    'high', // priority
    'frontend', // project
    '2 weeks', // time
    'Requires React and D3.js' // context
  );
  console.log(`🏗️ Advanced task created with full options`);

  // Update task status
  client.updateTaskStatus(urgentTask, 'in_progress');
  console.log(`📈 Updated task status`);

  // Plan complex work
  const projectPlan = client.planTasks(
    'Build e-commerce platform',
    'complex', // complexity
    '3 months', // timeline
    'Full-stack application with payment integration' // context
  );
  console.log(`📊 Planned ${projectPlan.length} tasks for complex project`);

  // ========== Search & Discovery ==========
  console.log('\n🔍 Search & Discovery:');

  // Multiple search types
  const keywordResults = client.keywordSearch('authentication');
  console.log(`🔤 Keyword search found ${keywordResults.length} results`);

  const aiSearchResults = client.aiSearch('security-related tasks');
  console.log(`🎯 AI search found ${aiSearchResults.length} semantic matches`);

  const similarTasks = client.similarTasks(urgentTask);
  console.log(`🔗 Found ${similarTasks.length} similar tasks`);

  // ========== Queue Management ==========
  console.log('\n📋 Queue Management:');

  const queueItem = client.queueAdd('Code review session', 'Review PR #123');
  console.log(`➕ Added to queue: ${queueItem}`);

  const queueItems = client.queueList();
  console.log(`📝 Queue has ${queueItems.length} items`);

  // ========== Project Organization ==========
  console.log('\n📁 Project Organization:');

  // Create and manage projects
  client.createProject('web-platform', 'Main web application');
  console.log(`📁 Created project: web-platform`);

  const projects = client.listProjects();
  console.log(`🏢 Available projects: ${projects.join(', ')}`);

  // ========== Statistics & Analytics ==========
  console.log('\n📊 Statistics & Analytics:');

  const stats = client.stats();
  console.log(`📈 Quick stats: ${stats}`);

  const detailedStats = client.detailedStats();
  console.log(`📊 Detailed analytics available`);

  // ========== Easy API (Simplified Interface) ==========
  console.log('\n🎯 Easy API - Simplified Interface:');

  const easyResult = client.doIt('Create a user registration form');
  console.log(`✨ Easy API result: ${easyResult.substring(0, 50)}...`);

  // ========== API Key Management ==========
  console.log('\n🔑 API Key Management:');

  const apiKey = client.createApiKey();
  console.log(`🔐 Created API key`);

  const keys = client.listApiKeys();
  console.log(`🗝️ Total API keys: ${keys.length}`);

  // ========== Storage Operations ==========
  console.log('\n💾 Storage Operations:');

  const storageInit = client.storageInit();
  console.log(`💽 Storage initialized`);

  const storageDir = client.storageGetStorageDir();
  console.log(`📂 Storage directory: ${storageDir}`);

  // ========== Tag Management ==========
  console.log('\n🏷️ Tag Management:');

  client.addTagToTask(normalTask, 'testing');
  console.log(`🏷️ Added tag to task`);

  const taggedTasks = client.findByTag('testing');
  console.log(`🔍 Found ${taggedTasks.length} tasks with tag 'testing'`);

  // ========== CLI Operations ==========
  console.log('\n💻 CLI Operations:');

  const cliTasks = client.cliListTasks();
  console.log(`🖥️ CLI listed ${cliTasks.length} tasks`);

  // ========== Final Summary ==========
  console.log('\n🎉 Todozi Professional API Demo Complete!');
  console.log(`✅ Demonstrated ${Object.getOwnPropertyNames(TodoziClient.prototype).filter(name => name !== 'constructor').length} methods`);
  console.log('🚀 All 175+ Rust API methods available through unified JavaScript interface');
}

main().catch(console.error);
