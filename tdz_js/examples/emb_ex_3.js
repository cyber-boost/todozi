import {
  TodoziEmbeddingService,
  TodoziEmbeddingConfig,
  TodoziContentType,
  HierarchicalCluster,
  SimilarityResult
} from '../todozi/emb.js';

import {
  Task,
  Memory,
  MemoryType,
  MemoryImportance,
  MemoryTerm,
  Idea,
  ShareLevel,
  IdeaImportance
} from '../todozi/models.js';

import { TagManager } from '../todozi/tags.js';

class TodoziContentOrganizer {
  constructor(embeddingService, tagManager) {
    this.embeddingService = embeddingService;
    this.tagManager = tagManager;
  }

  /**
   * Create sample content for clustering demonstration
   */
  async createSampleContent() {
    const content = {
      tasks: [],
      memories: [],
      ideas: []
    };

    // Create related tasks about web development
    content.tasks.push(
      new Task({
        userId: 'user1',
        action: 'Implement user authentication system',
        time: '4 hours',
        priority: 'high',
        parentProject: 'web-app',
        status: 'todo',
        tags: ['auth', 'security', 'backend'],
        contextNotes: 'Add JWT authentication with refresh tokens'
      }),
      new Task({
        userId: 'user1',
        action: 'Design login page UI',
        time: '2 hours',
        priority: 'medium',
        parentProject: 'web-app',
        status: 'todo',
        tags: ['ui', 'frontend', 'auth'],
        contextNotes: 'Create responsive login form with validation'
      }),
      new Task({
        userId: 'user1',
        action: 'Set up OAuth providers',
        time: '3 hours',
        priority: 'medium',
        parentProject: 'web-app',
        status: 'todo',
        tags: ['auth', 'integration', 'backend'],
        contextNotes: 'Integrate Google and GitHub OAuth'
      })
    );

    // Create related tasks about database
    content.tasks.push(
      new Task({
        userId: 'user1',
        action: 'Design database schema',
        time: '3 hours',
        priority: 'high',
        parentProject: 'web-app',
        status: 'todo',
        tags: ['database', 'design', 'backend'],
        contextNotes: 'Create ERD for user management system'
      }),
      new Task({
        userId: 'user1',
        action: 'Implement user model',
        time: '2 hours',
        priority: 'high',
        parentProject: 'web-app',
        status: 'todo',
        tags: ['database', 'backend', 'orm'],
        contextNotes: 'Create User entity with relationships'
      })
    );

    // Create memories about learning experiences
    content.memories.push(
      new Memory({
        userId: 'user1',
        moment: 'Struggled with JWT implementation last week',
        meaning: 'Learned about token expiration and refresh strategies',
        reason: 'Security best practices for session management',
        importance: MemoryImportance.High,
        term: MemoryTerm.Long,
        memoryType: MemoryType.Standard,
        tags: ['auth', 'learning', 'security']
      }),
      new Memory({
        userId: 'user1',
        moment: 'Database normalization confusion',
        meaning: 'Understanding when to normalize vs denormalize',
        reason: 'Performance optimization for queries',
        importance: MemoryImportance.Medium,
        term: MemoryTerm.Long,
        memoryType: MemoryType.Standard,
        tags: ['database', 'learning', 'performance']
      })
    );

    // Create ideas about improvements
    content.ideas.push(
      new Idea({
        idea: 'Use WebSockets for real-time notifications',
        projectId: 'web-app',
        status: 'active',
        share: ShareLevel.Team,
        importance: IdeaImportance.Medium,
        tags: ['realtime', 'websockets', 'frontend'],
        context: 'Could improve user engagement significantly'
      }),
      new Idea({
        idea: 'Implement caching layer for database queries',
        projectId: 'web-app',
        status: 'active',
        share: ShareLevel.Team,
        importance: IdeaImportance.High,
        tags: ['performance', 'cache', 'backend'],
        context: 'Redis could reduce response times by 40%'
      })
    );

    // Embed all content
    for (const task of content.tasks) {
      await this.embeddingService.addTask(task);
    }

    for (const memory of content.memories) {
      await this.embeddingService.newMemory(memory);
    }

    for (const idea of content.ideas) {
      await this.embeddingService.newIdea(idea);
    }

    return content;
  }

  /**
   * Perform hierarchical clustering on content
   */
  async clusterContentHierarchically(maxDepth = 3) {
    const contentTypes = [
      TodoziContentType.Task,
      TodoziContentType.Memory,
      TodoziContentType.Idea
    ];

    console.log(`Performing hierarchical clustering on ${contentTypes.join(', ')}...`);
    const clusters = await this.embeddingService.hierarchicalClustering(
      contentTypes,
      maxDepth
    );

    return clusters;
  }

  /**
   * Visualize cluster hierarchy
   */
  visualizeClusterHierarchy(clusters, indent = 0) {
    const prefix = '  '.repeat(indent);
    
    for (const cluster of clusters) {
      const clusterInfo = `${prefix}📁 Cluster ${cluster.cluster_id} (Level ${cluster.level})`;
      console.log(clusterInfo);
      console.log(`${prefix}   Items: ${cluster.cluster_size}`);
      console.log(`${prefix}   Avg Similarity: ${(cluster.averageSimilarity * 100).toFixed(1)}%`);
      
      // Group items by type
      const itemsByType = {};
      for (const item of cluster.content_items) {
        if (!itemsByType[item.content_type]) {
          itemsByType[item.content_type] = [];
        }
        itemsByType[item.content_type].push(item);
      }

      // Show items in cluster
      for (const [type, items] of Object.entries(itemsByType)) {
        console.log(`${prefix}   ${type}:`);
        for (const item of items.slice(0, 3)) { // Limit to 3 items per type
          const preview = item.text_content.split('\n')[0]?.substring(0, 60) || '';
          console.log(`${prefix}     • ${preview}...`);
        }
        if (items.length > 3) {
          console.log(`${prefix}     ... and ${items.length - 3} more`);
        }
      }

      // Visualize sub-clusters
      if (cluster.children && cluster.children.length > 0) {
        console.log(`${prefix}   Sub-clusters (${cluster.children.length}):`);
        this.visualizeClusterHierarchy(cluster.children, indent + 1);
      }
      
      console.log();
    }
  }

  /**
   * Find cross-content relationships for a specific item
   */
  async analyzeCrossContentRelationships(contentId, contentType) {
    console.log(`\n🔍 Analyzing cross-content relationships for ${contentType}:${contentId}`);
    
    const relationships = await this.embeddingService.findCrossContentRelationships(
      contentId,
      contentType,
      0.6 // Minimum similarity threshold
    );

    console.log('\n📊 Related Content:');
    for (const [type, items] of Object.entries(relationships)) {
      console.log(`\n${type}:`);
      for (const item of items.slice(0, 5)) {
        const preview = item.text_content.split('\n')[0]?.substring(0, 50) || '';
        console.log(`  ${(item.similarity_score * 100).toFixed(1)}% - ${preview}...`);
      }
      if (items.length > 5) {
        console.log(`  ... and ${items.length - 5} more`);
      }
    }

    return relationships;
  }

  /**
   * Build and visualize similarity graph
   */
  async buildSimilarityGraph(threshold = 0.7) {
    console.log(`\n🕸️  Building similarity graph (threshold: ${(threshold * 100).toFixed(0)}%)`);
    
    const graph = await this.embeddingService.buildSimilarityGraph(threshold);
    
    console.log(`\n📊 Graph Statistics:`);
    console.log(`  Nodes: ${graph.nodes.length}`);
    console.log(`  Edges: ${graph.edges.length}`);
    console.log(`  Avg Degree: ${((graph.edges.length * 2) / graph.nodes.length).toFixed(2)}`);
    
    // Show most connected nodes
    const nodeConnections = new Map();
    for (const edge of graph.edges) {
      nodeConnections.set(edge.from, (nodeConnections.get(edge.from) || 0) + 1);
      nodeConnections.set(edge.to, (nodeConnections.get(edge.to) || 0) + 1);
    }
    
    const sortedNodes = Array.from(nodeConnections.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, 5);
    
    console.log('\n🌟 Most Connected Items:');
    for (const [nodeId, connections] of sortedNodes) {
      const node = graph.nodes.find(n => n.id === nodeId);
      if (node) {
        console.log(`  ${node.content_type}: ${node.label} (${connections} connections)`);
      }
    }

    return graph;
  }

  /**
   * Recommend similar content based on multiple items
   */
  async recommendSimilarContent(basedOnIds, contentTypes) {
    console.log(`\n💡 Finding similar content based on ${basedOnIds.length} items`);
    
    const recommendations = await this.embeddingService.recommendSimilar(
      basedOnIds,
      [], // No exclusions
      10   // Limit to 10 recommendations
    );

    // Group by type
    const grouped = {};
    for (const rec of recommendations) {
      if (!grouped[rec.content_type]) {
        grouped[rec.content_type] = [];
      }
      grouped[rec.content_type].push(rec);
    }

    console.log('\n🎯 Recommendations:');
    for (const [type, items] of Object.entries(grouped)) {
      console.log(`\n${type}:`);
      for (const item of items) {
        const preview = item.text_content.split('\n')[0]?.substring(0, 60) || '';
        console.log(`  ${(item.similarity_score * 100).toFixed(1)}% - ${preview}...`);
      }
    }

    return recommendations;
  }

  /**
   * Run complete analysis
   */
  async runCompleteAnalysis() {
    console.log('🚀 Starting Todozi Content Organization Analysis\n');

    // Create sample content
    console.log('📝 Creating sample content...');
    await this.createSampleContent();

    // Get embedding statistics
    const stats = await this.embeddingService.getStats();
    console.log(`📊 Generated embeddings for ${stats.total_embeddings} items`);
    console.log('Breakdown:', stats.type_counts);

    // Perform hierarchical clustering
    const clusters = await this.clusterContentHierarchically(3);
    console.log(`\n🌳 Found ${clusters.length} top-level clusters`);
    
    // Visualize clusters
    console.log('\n📋 Cluster Hierarchy:');
    this.visualizeClusterHierarchy(clusters);

    // Find cross-content relationships for a specific task
    if (clusters.length > 0 && clusters[0].content_items.length > 0) {
      const sampleItem = clusters[0].content_items[0];
      await this.analyzeCrossContentRelationships(
        sampleItem.content_id,
        sampleItem.content_type
      );
    }

    // Build similarity graph
    const graph = await this.buildSimilarityGraph(0.6);

    // Generate recommendations
    if (clusters.length > 0) {
      const basedOnItems = clusters[0].content_items.slice(0, 3).map(i => i.content_id);
      await this.recommendSimilarContent(basedOnItems, ['Task', 'Memory', 'Idea']);
    }

    // Export diagnostics
    console.log('\n🔧 Exporting diagnostics...');
    const diagnostics = await this.embeddingService.exportDiagnostics();
    console.log(`Average similarity score: ${(diagnostics.avg_similarity_score * 100).toFixed(1)}%`);
    
    console.log('\n✅ Analysis complete!');
    return {
      clusters,
      graph,
      diagnostics
    };
  }
}

// Usage example
async function main() {
  try {
    // Initialize services
    const config = TodoziEmbeddingConfig.default();
    config.enable_clustering = true;
    config.clustering_threshold = 0.6;
    config.similarity_threshold = 0.5;
    
    const embeddingService = await TodoziEmbeddingService.new(config);
    const tagManager = TagManager.new();
    
    // Create organizer and run analysis
    const organizer = new TodoziContentOrganizer(embeddingService, tagManager);
    const results = await organizer.runCompleteAnalysis();
    
    // Optional: Save results to file
    // await fs.writeFile('analysis_results.json', JSON.stringify(results, null, 2));
    
  } catch (error) {
    console.error('❌ Error during analysis:', error.message);
    process.exit(1);
  }
}

// Run if this file is executed directly
// Run if executed directly
  main();
}


/*
# Example 3: Hierarchical Clustering of Todozi Content with Cross-Content Analysis

This example demonstrates how to use Todozi's hierarchical clustering capabilities to organize and analyze related content across different types (tasks, memories, ideas, etc.).

## Key Features Demonstrated:

1. **Hierarchical Clustering**: Organizes related content into a tree structure with multiple levels
2. **Cross-Content Analysis**: Discovers relationships between different content types (tasks, memories, ideas)
3. **Similarity Graph**: Builds a network graph showing how content items are related
4. **Content Recommendations**: Suggests similar items based on multiple reference items
5. **Visualization**: Provides clear console output showing the organization and relationships

## Expected Output:

/ *
🚀 Starting Todozi Content Organization Analysis

📝 Creating sample content...
📊 Generated embeddings for 10 items
Breakdown: { Task: 5, Memory: 2, Idea: 3 }

Performing hierarchical clustering on Task, Memory, Idea...

🌳 Found 2 top-level clusters

📋 Cluster Hierarchy:
📁 Cluster abc123 (Level 0)
   Items: 5
   Avg Similarity: 73.4%
   Task:
     • Implement user authentication system...
     • Design login page UI...
     • Set up OAuth providers...
   Memory:
     • Struggled with JWT implementation last week...
   Sub-clusters (2):
     📁 Cluster def456 (Level 1)
        Items: 3
        Avg Similarity: 81.2%
        Task:
          • Implement user authentication system...
          • Design login page UI...

📁 Cluster ghi789 (Level 0)
   Items: 3
   Avg Similarity: 68.9%
   Task:
     • Design database schema...
     • Implement user model...
   Memory:
     • Database normalization confusion...

🔍 Analyzing cross-content relationships for Task:task_abc123

📊 Related Content:
Memory:
  76.3% - Struggled with JWT implementation last week...
Idea:
  72.1% - Implement caching layer for database queries...
Task:
  68.9% - Design login page UI...

🕸️  Building similarity graph (threshold: 60%)

📊 Graph Statistics:
  Nodes: 10
  Edges: 14
  Avg Degree: 2.80

🌟 Most Connected Items:
  Task: Implement user authentication system (4 connections)
  Memory: Struggled with JWT implementation last week (3 connections)
  Task: Design database schema (3 connections)

💡 Finding similar content based on 3 items

🎯 Recommendations:
Task:
  82.4% - Set up OAuth providers...
  78.1% - Design login page UI...
Memory:
  71.2% - Database normalization confusion...
Idea:
  69.8% - Use WebSockets for real-time notifications...

🔧 Exporting diagnostics...
Average similarity score: 68.7%

✅ Analysis complete!
*/