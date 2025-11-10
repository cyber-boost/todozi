import { AgentManager, AgentUpdate, AgentStatistics } from '../todozi/agent.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { TagManager, TagSearchEngine, TagSearchQuery, TagSortBy } from '../todozi/tags.js';
import { processChatMessageExtended } from '../todozi/todozi.js';
import { Priority, Status, AgentStatus, AssignmentStatus } from '../todozi/models.js';

class TaskManagementWorkflow {
  constructor() {
    this.agentManager = new AgentManager();
    this.embeddingService = null;
    this.tagManager = TagManager.new();
    this.searchEngine = TagSearchEngine.new(this.tagManager);
  }

  async initialize() {
    // Initialize the agent manager with default agents
    await this.agentManager.loadAgents();
    
    // Initialize embedding service
    const config = TodoziEmbeddingConfig.default();
    this.embeddingService = await TodoziEmbeddingService.new(config);
    
    // Create some tags for categorization
    await this.setupTags();
    
    console.log('✅ Task Management Workflow initialized');
  }

  async setupTags() {
    const tags = [
      { name: 'development', category: 'technical', description: 'Software development tasks' },
      { name: 'bug-fix', category: 'technical', description: 'Bug fixing tasks' },
      { name: 'documentation', category: 'technical', description: 'Documentation tasks' },
      { name: 'planning', category: 'management', description: 'Project planning tasks' },
      { name: 'review', category: 'quality', description: 'Code review and testing' },
      { name: 'urgent', category: 'priority', description: 'High priority urgent tasks' }
    ];

    for (const tagData of tags) {
      await this.tagManager.createTag(tagData);
    }

    // Create tag relationships
    const devTag = this.tagManager.getTagByName('development');
    const bugTag = this.tagManager.getTagByName('bug-fix');
    const docTag = this.tagManager.getTagByName('documentation');
    
    if (devTag && bugTag) {
      await this.tagManager.addTagRelationship(devTag.id, bugTag.id);
    }
    if (devTag && docTag) {
      await this.tagManager.addTagRelationship(devTag.id, docTag.id);
    }
  }

  async createSpecializedAgents() {
    // Create a backend development agent
    const backendAgent = {
      name: 'Backend Developer',
      description: 'Specializes in server-side development, APIs, and databases',
      capabilities: ['api_development', 'database_design', 'backend_testing'],
      specializations: ['node.js', 'python', 'postgresql', 'redis'],
      metadata: {
        category: 'development',
        status: AgentStatus.Available
      }
    };

    const backendAgentId = await this.agentManager.createAgent(backendAgent);
    
    // Create a frontend development agent
    const frontendAgent = {
      name: 'Frontend Developer',
      description: 'Specializes in UI/UX development and client-side technologies',
      capabilities: ['ui_development', 'responsive_design', 'frontend_testing'],
      specializations: ['react', 'typescript', 'css', 'javascript'],
      metadata: {
        category: 'development',
        status: AgentStatus.Available
      }
    };

    const frontendAgentId = await this.agentManager.createAgent(frontendAgent);
    
    // Create a QA agent
    const qaAgent = {
      name: 'QA Engineer',
      description: 'Specializes in quality assurance and testing',
      capabilities: ['test_planning', 'automation', 'bug_reporting'],
      specializations: ['selenium', 'jest', 'cypress', 'performance_testing'],
      metadata: {
        category: 'quality',
        status: AgentStatus.Available
      }
    };

    const qaAgentId = await this.agentManager.createAgent(qaAgent);

    return { backendAgentId, frontendAgentId, qaAgentId };
  }

  async processComplexChatMessage() {
    const message = `
      Working on the user authentication system. Need to implement JWT token refresh mechanism.
      <todozi>Implement JWT refresh endpoint;2 hours;high;backend-api;in_progress;agent:backend</todozi>
      
      Also need to update the login UI to handle token expiration gracefully.
      <todozi>Update login UI for token expiration;1 hour;medium;frontend-ui;todo;agent:frontend</todozi>
      
      Remember to write unit tests for the refresh endpoint.
      <todozi>Write unit tests for JWT refresh;1 hour;high;backend-api;todo;agent:backend</todozi>
      
      <memory>JWT token expiration causing session issues;Implement refresh mechanism to maintain user sessions;High priority security fix;high;short</memory>
      
      <idea>Implement automatic token refresh with retry logic;private;high</idea>
      
      <error>JWT tokens expiring too frequently;Tokens have short expiration time causing poor UX;medium;security;frontend</error>
    `;

    const content = processChatMessageExtended(message, 'user123');
    
    console.log('📊 Processed Chat Content:');
    console.log(`  Tasks: ${content.tasks.length}`);
    console.log(`  Memories: ${content.memories.length}`);
    console.log(`  Ideas: ${content.ideas.length}`);
    console.log(`  Errors: ${content.errors.length}`);
    
    return content;
  }

  async assignTasksToBestAgents(content) {
    const assignments = [];
    
    for (const task of content.tasks) {
      // Find the best agent based on task requirements
      const bestAgent = this.agentManager.findBestAgent(
        task.parent_project.replace('-', '_'), // Convert project name to specialization
        task.priority === 'high' ? 'backend_testing' : null
      );
      
      if (bestAgent) {
        const assignment = await this.agentManager.assignTaskToAgent(
          task.id,
          bestAgent.id,
          task.parent_project
        );
        
        assignments.push({
          task: task.action,
          agent: bestAgent.name,
          assignmentId: assignment
        });
        
        // Generate embedding for the task
        const taskContent = `${task.action} ${task.context_notes || ''}`;
        await this.embeddingService.generateEmbedding(taskContent);
        
        console.log(`✅ Assigned task "${task.action}" to ${bestAgent.name}`);
      } else {
        console.log(`⚠️  No available agent found for task: ${task.action}`);
      }
    }
    
    return assignments;
  }

  async semanticTaskSearch(query) {
    console.log(`🔍 Performing semantic search for: "${query}"`);
    
    const results = await this.embeddingService.semanticSearch(query, ['Task'], 5);
    
    console.log('📋 Search Results:');
    results.forEach((result, index) => {
      console.log(`  ${index + 1}. Similarity: ${(result.similarity_score * 100).toFixed(1)}%`);
      console.log(`     Content: ${result.text_content.substring(0, 100)}...`);
      console.log(`     Tags: ${result.tags.join(', ') || 'None'}`);
      console.log();
    });
    
    return results;
  }

  async analyzeTagStatistics() {
    const stats = this.tagManager.getTagStatistics();
    
    console.log('📊 Tag Statistics:');
    console.log(`  Total Tags: ${stats.total_tags}`);
    console.log(`  Categories: ${stats.total_categories}`);
    console.log(`  Relationships: ${stats.total_relationships}`);
    console.log(`  Avg Usage: ${stats.average_usage.toFixed(2)}`);
    console.log(`  Relationships/Tag: ${stats.relationshipsPerTag().toFixed(2)}`);
    
    // Get most used tags
    const mostUsed = this.tagManager.getMostUsedTags(5);
    console.log('\n🏷️  Most Used Tags:');
    mostUsed.forEach((tag, index) => {
      console.log(`  ${index + 1}. ${tag.name} (${tag.usage_count} uses)`);
    });
    
    return stats;
  }

  async advancedTagSearch() {
    console.log('🔍 Advanced Tag Search:');
    
    // Search for technical tags with usage >= 2
    const query = TagSearchQuery.default()
      .category('technical')
      .min_usage(2)
      .sort_by(TagSortBy.Usage)
      .limit(10);
    
    const results = this.searchEngine.advancedSearch(query);
    
    console.log(`Found ${results.length} technical tags with usage >= 2:`);
    results.forEach((tag, index) => {
      console.log(`  ${index + 1}. ${tag.name} (${tag.usage_count} uses) - ${tag.category}`);
    });
    
    // Fuzzy search for similar tags
    console.log('\n🔀 Fuzzy Search for tags similar to "develop":');
    const fuzzyResults = this.searchEngine.fuzzySearch('develop', 2);
    fuzzyResults.forEach(([tag, distance]) => {
      console.log(`  ${tag.name} (distance: ${distance})`);
    });
    
    return results;
  }

  async getAgentPerformanceMetrics() {
    const stats = this.agentManager.getAgentStatistics();
    
    console.log('📈 Agent Performance Metrics:');
    console.log(`  Total Agents: ${stats.total_agents}`);
    console.log(`  Available: ${stats.available_agents}`);
    console.log(`  Busy: ${stats.busy_agents}`);
    console.log(`  Inactive: ${stats.inactive_agents}`);
    console.log(`  Total Assignments: ${stats.total_assignments}`);
    console.log(`  Completed Assignments: ${stats.completed_assignments}`);
    console.log(`  Completion Rate: ${stats.completionRate().toFixed(1)}%`);
    
    // Get individual agent assignments
    const allAgents = this.agentManager.getAllAgents();
    console.log('\n👥 Agent Assignment Details:');
    for (const agent of allAgents) {
      const assignments = this.agentManager.getAgentAssignments(agent.id);
      const completed = assignments.filter(a => a.status === AssignmentStatus.Completed).length;
      const total = assignments.length;
      
      console.log(`  ${agent.name}: ${completed}/${total} assignments completed`);
    }
    
    return stats;
  }

  async completeTaskWorkflow(taskId, agentId) {
    console.log(`🔄 Completing task ${taskId} assigned to agent ${agentId}`);
    
    // Mark the assignment as complete
    await this.agentManager.completeAgentAssignment(taskId);
    
    // Update agent status back to available
    await this.agentManager.updateAgentStatus(agentId, AgentStatus.Available);
    
    console.log(`✅ Task completed and agent ${agentId} is now available`);
  }

  async suggestRelatedContent(taskContent) {
    console.log(`💡 Finding content related to: "${taskContent}"`);
    
    // Use hybrid search with both semantic and keyword matching
    const keywords = taskContent.toLowerCase().split(' ').filter(w => w.length > 3);
    const results = await this.embeddingService.hybridSearch(
      taskContent,
      keywords,
      ['Task', 'Memory', 'Idea'],
      0.7, // semantic weight
      5
    );
    
    console.log('🔗 Related Content:');
    results.forEach((result, index) => {
      const score = result.similarity_score;
      const type = result.content_type;
      const preview = result.text_content.substring(0, 80);
      
      console.log(`  ${index + 1}. [${type}] ${score.toFixed(3)} - ${preview}...`);
      
      if (result.metadata) {
        console.log(`     Semantic: ${result.metadata.semantic_score?.toFixed(3) || 'N/A'}, ` +
                   `Keyword: ${result.metadata.keyword_score?.toFixed(3) || 'N/A'}`);
      }
    });
    
    return results;
  }

  async runWorkflowDemo() {
    console.log('🚀 Starting AI-Powered Task Management Workflow Demo\n');
    
    try {
      // Initialize the workflow
      await this.initialize();
      
      // Create specialized agents
      const agentIds = await this.createSpecializedAgents();
      console.log('\n🤖 Created specialized agents');
      
      // Process a complex chat message
      const content = await this.processComplexChatMessage();
      
      // Assign tasks to best agents
      console.log('\n📋 Assigning tasks to agents...');
      const assignments = await this.assignTasksToBestAgents(content);
      
      // Analyze tag statistics
      await this.analyzeTagStatistics();
      
      // Perform semantic search
      await this.semanticTaskSearch('JWT authentication token refresh');
      
      // Advanced tag search
      await this.advancedTagSearch();
      
      // Get agent performance metrics
      await this.getAgentPerformanceMetrics();
      
      // Suggest related content
      await this.suggestRelatedContent('user authentication security');
      
      // Simulate completing a task
      if (assignments.length > 0) {
        const firstAssignment = assignments[0];
        const agent = this.agentManager.getAllAgents().find(a => a.name === firstAssignment.agent);
        if (agent) {
          // In a real scenario, we'd get the actual task ID
          console.log('\n✨ Workflow completed successfully!');
        }
      }
      
    } catch (error) {
      console.error('❌ Workflow error:', error.message);
      throw error;
    }
  }
}

// Run the workflow demo
async function main() {
  const workflow = new TaskManagementWorkflow();
  await workflow.runWorkflowDemo();
}

// Uncomment to run the demo directly
// main().catch(console.error);

/*
# Example 3: AI-Powered Task Management Workflow

This example demonstrates how to use Todozi's agent management system with semantic search capabilities to create an intelligent task workflow. We'll set up agents, create tasks with various content types, and use embeddings to find similar tasks and suggest assignments.

## Key Features Demonstrated:

1. **Agent Management**: 
   - Creating specialized agents with capabilities and specializations
   - Finding the best agent for a task based on requirements
   - Tracking agent assignments and completion status

2. **Semantic Search**:
   - Using embeddings to find semantically similar tasks
   - Hybrid search combining semantic and keyword matching
   - Content suggestions based on similarity scores

3. **Tag Management**:
   - Creating categorized tags with relationships
   - Advanced tag search with multiple filters
   - Fuzzy search for similar tag names
   - Usage statistics and performance metrics

4. **Content Processing**:
   - Parsing complex chat messages with multiple content types
   - Processing tasks, memories, ideas, and errors simultaneously
   - Maintaining relationships between different content types

5. **Workflow Integration**:
   - Complete task lifecycle from creation to assignment to completion
   - Performance metrics and statistics
   - Intelligent content recommendations

This example shows how the various Todozi components work together to create an intelligent task management system that can understand content semantics, suggest appropriate assignments, and maintain organized categorization through tags and embeddings.
*/