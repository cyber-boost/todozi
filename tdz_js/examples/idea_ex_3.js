
/**
import { IdeaManager, IdeaUpdate, IdeaStatistics } from '../todozi/idea.js';
import { ItemStatus, ShareLevel, IdeaImportance } from '../todozi/models.js';

 * Example 3: Creative Ideas Management System
 * 
 * This example demonstrates a complete idea management workflow using the IdeaManager class.
 * It simulates a creative team's brainstorming and idea tracking system.
 */

class CreativeIdeasApp {
  constructor() {
    this.ideaManager = new IdeaManager();
    this.init();
  }

  async init() {
    console.log('🚀 Initializing Creative Ideas Management System...\n');
    
    // Add sample ideas to demonstrate functionality
    await this.addSampleIdeas();
    
    // Show initial state
    this.showDashboard();
    
    // Demonstrate various operations
    await this.demonstrateSearch();
    await this.demonstrateFilters();
    await this.demonstrateUpdates();
    await this.demonstrateStatistics();
    
    console.log('\n✅ Creative Ideas Management Demo Complete!');
  }

  async addSampleIdeas() {
    console.log('📝 Adding sample creative ideas...\n');
    
    // AI-powered content platform idea
    const aiPlatformId = await this.ideaManager.createIdea({
      idea: "AI-powered content creation platform that generates personalized learning materials based on user learning styles",
      share: ShareLevel.PUBLIC,
      importance: IdeaImportance.BREAKTHROUGH,
      tags: ["AI", "education", "personalization", "content"],
      context: "For the educational technology sector - could revolutionize how students learn"
    });

    // Mobile app for local communities
    const communityAppId = await this.ideaManager.createIdea({
      idea: "Mobile app for connecting local communities through skill sharing and micro-volunteering",
      share: ShareLevel.TEAM,
      importance: IdeaImportance.HIGH,
      tags: ["mobile", "community", "social", "local"],
      context: "Focus on neighborhoods - people help neighbors with small tasks and share skills"
    });

    // Sustainability dashboard
    const sustainabilityId = await this.ideaManager.createIdea({
      idea: "Real-time carbon footprint tracking dashboard for small businesses",
      share: ShareLevel.PRIVATE,
      importance: IdeaImportance.MEDIUM,
      tags: ["sustainability", "analytics", "business", "environment"],
      context: "Helps businesses monitor and reduce their environmental impact"
    });

    // Virtual collaboration space
    const vrcollabId = await this.ideaManager.createIdea({
      idea: "VR collaboration space for remote teams with virtual whiteboards and 3D modeling",
      share: ShareLevel.PUBLIC,
      importance: IdeaImportance.HIGH,
      tags: ["VR", "collaboration", "remote", "3d"],
      context: "Improve remote team collaboration through immersive technology"
    });

    // Health monitoring device
    const healthId = await this.ideaManager.createIdea({
      idea: "Wearable device for early detection of health issues through continuous monitoring",
      share: ShareLevel.PRIVATE,
      importance: IdeaImportance.BREAKTHROUGH,
      tags: ["healthcare", "wearables", "monitoring", "prevention"],
      context: "Focus on preventative healthcare through early warning systems"
    });

    console.log(`✅ Created ${this.ideaManager.getAllIdeas().length} sample ideas\n`);
  }

  showDashboard() {
    const allIdeas = this.ideaManager.getAllIdeas();
    const stats = this.ideaManager.getIdeaStatistics();
    
    console.log('📊 IDEAS DASHBOARD');
    console.log('═'.repeat(50));
    console.log(`Total Ideas: ${stats.totalIdeas}`);
    console.log(`Public: ${stats.publicIdeas} | Team: ${stats.teamIdeas} | Private: ${stats.privateIdeas}`);
    console.log(`Breakthrough Ideas: ${stats.breakthroughIdeas}`);
    console.log(`Unique Tags: ${stats.uniqueTags}`);
    
    console.log('\n📋 RECENT IDEAS:');
    const recentIdeas = this.ideaManager.getRecentIdeas(3);
    recentIdeas.forEach((idea, index) => {
      const preview = idea.idea.length > 60 ? idea.idea.substring(0, 57) + '...' : idea.idea;
      console.log(`${index + 1}. ${preview}`);
      console.log(`   Tags: ${idea.tags.join(', ')}`);
      console.log(`   Importance: ${idea.importance} | Share: ${idea.share}\n`);
    });
  }

  async demonstrateSearch() {
    console.log('🔍 SEARCH FUNCTIONALITY');
    console.log('═'.repeat(50));
    
    // Search by keyword
    console.log('\n1️⃣ Searching for "AI":');
    const aiIdeas = this.ideaManager.searchIdeas("AI");
    aiIdeas.forEach(idea => {
      console.log(`   • ${idea.idea} (${idea.importance})`);
    });
    
    // Search by tag
    console.log('\n2️⃣ Searching for ideas with "healthcare" tag:');
    const healthIdeas = this.ideaManager.searchIdeas("healthcare");
    healthIdeas.forEach(idea => {
      console.log(`   • ${idea.idea} (${idea.importance})`);
    });
    
    // Search in context
    console.log('\n3️⃣ Searching for "education" in contexts:');
    const educationIdeas = this.ideaManager.searchIdeas("education");
    educationIdeas.forEach(idea => {
      console.log(`   • ${idea.idea}`);
      console.log(`     Context: ${idea.context || 'No context'}\n`);
    });
  }

  async demonstrateFilters() {
    console.log('\n🎯 FILTERED VIEWS');
    console.log('═'.repeat(50));
    
    // Filter by importance
    console.log('\n🚨 BREAKTHROUGH IDEAS:');
    const breakthroughIdeas = this.ideaManager.getBreakthroughIdeas();
    breakthroughIdeas.forEach(idea => {
      console.log(`   • ${idea.idea}`);
      console.log(`     Tags: ${idea.tags.join(', ')}\n`);
    });
    
    // Filter by share level
    console.log('🌐 PUBLIC IDEAS:');
    const publicIdeas = this.ideaManager.getPublicIdeas();
    publicIdeas.forEach(idea => {
      console.log(`   • ${idea.idea} (${idea.importance})`);
    });
    
    // Filter by tag
    console.log('\n🏷️  IDEAS WITH "mobile" TAG:');
    const mobileIdeas = this.ideaManager.getIdeasByTag("mobile");
    mobileIdeas.forEach(idea => {
      console.log(`   • ${idea.idea}`);
    });
  }

  async demonstrateUpdates() {
    console.log('\n📝 UPDATING IDEAS');
    console.log('═'.repeat(50));
    
    // Find an idea to update
    const ideas = this.ideaManager.getAllIdeas();
    const targetIdea = ideas.find(idea => idea.idea.includes("AI"));
    
    if (targetIdea) {
      console.log(`\nUpdating: "${targetIdea.idea.substring(0, 40)}..."`);
      
      // Create update using builder pattern
      const update = IdeaUpdate.new()
        .withImportance(IdeaImportance.BREAKTHROUGH)
        .withTags([...targetIdea.tags, "machine-learning", "personalization"])
        .withContext("Enhanced with neural networks for better personalization");
      
      await this.ideaManager.updateIdea(targetIdea.id, update);
      
      // Show updated idea
      const updated = this.ideaManager.getIdea(targetIdea.id);
      console.log('✅ Updated successfully!');
      console.log(`   New importance: ${updated.importance}`);
      console.log(`   New tags: ${updated.tags.join(', ')}`);
      console.log(`   New context: ${updated.context}\n`);
    }
    
    // Demonstrate changing share level
    const communityIdea = ideas.find(idea => idea.idea.includes("communities"));
    if (communityIdea) {
      console.log(`Making community idea PUBLIC...`);
      await this.ideaManager.updateIdea(communityIdea.id, {
        share: ShareLevel.PUBLIC
      });
      console.log(`✅ Share level updated to: ${ShareLevel.PUBLIC}\n`);
    }
  }

  async demonstrateStatistics() {
    console.log('📊 DETAILED STATISTICS');
    console.log('═'.repeat(50));
    
    const stats = this.ideaManager.getIdeaStatistics();
    
    console.log(`\n📈 USAGE STATISTICS:`);
    console.log(`   Total ideas tracked: ${stats.totalIdeas}`);
    console.log(`   Breakthrough potential: ${stats.breakthroughIdeas} (${stats.breakthroughPercentage().toFixed(1)}%)`);
    console.log(`   Public visibility: ${stats.publicIdeas} (${stats.publicPercentage().toFixed(1)}%)`);
    console.log(`   Team collaboration: ${stats.teamIdeas} (${stats.teamPercentage().toFixed(1)}%)`);
    console.log(`   Private development: ${stats.privateIdeas} (${stats.privatePercentage().toFixed(1)}%)`);
    
    // Tag statistics
    console.log('\n🏷️  TAG ANALYSIS:');
    const tagStats = this.ideaManager.getTagStatistics();
    const sortedTags = Array.from(tagStats.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, 5);
    
    sortedTags.forEach(([tag, count]) => {
      console.log(`   ${tag}: ${count} ideas`);
    });
    
    // All tags
    console.log('\n📋 ALL TAGS IN SYSTEM:');
    const allTags = this.ideaManager.getAllTags();
    console.log(`   Total unique tags: ${allTags.length}`);
    console.log(`   Tags: ${allTags.join(', ')}`);
    
    // Recent additions
    console.log('\n⏰ MOST RECENT:');
    const mostRecent = this.ideaManager.getRecentIdeas(2);
    mostRecent.forEach(idea => {
      const timeAgo = this.getTimeAgo(idea.createdAt);
      console.log(`   ${timeAgo}: "${idea.idea.substring(0, 45)}..."`);
    });
  }

  getTimeAgo(dateString) {
    const created = new Date(dateString);
    const now = new Date();
    const diffMs = now - created;
    const diffMins = Math.floor(diffMs / 60000);
    
    if (diffMins < 1) return "Just now";
    if (diffMins < 60) return `${diffMins} minutes ago`;
    
    const diffHours = Math.floor(diffMins / 60);
    if (diffHours < 24) return `${diffHours} hours ago`;
    
    const diffDays = Math.floor(diffHours / 24);
    return `${diffDays} days ago`;
  }

  async demonstrateDeletion() {
    console.log('\n🗑️  DEMONSTRATING DELETION');
    console.log('═'.repeat(50));
    
    // Add a temporary idea to delete
    const tempId = await this.ideaManager.createIdea({
      idea: "Temporary idea for deletion demo",
      share: ShareLevel.PRIVATE,
      importance: IdeaImportance.LOW,
      tags: ["temporary", "demo"],
      context: "This will be deleted"
    });
    
    console.log(`\nCreated temporary idea: ${tempId}`);
    console.log(`Total ideas before deletion: ${this.ideaManager.getAllIdeas().length}`);
    
    // Delete the idea
    try {
      await this.ideaManager.deleteIdea(tempId);
      console.log(`✅ Successfully deleted idea ${tempId}`);
      console.log(`Total ideas after deletion: ${this.ideaManager.getAllIdeas().length}`);
    } catch (error) {
      console.error(`❌ Failed to delete: ${error.message}`);
    }
  }

  async demonstrateErrorHandling() {
    console.log('\n⚠️  ERROR HANDLING DEMO');
    console.log('═'.repeat(50));
    
    // Try to update non-existent idea
    try {
      await this.ideaManager.updateIdea("non-existent-id", {
        idea: "This should fail"
      });
    } catch (error) {
      console.log(`✅ Caught expected error: ${error.message}`);
    }
    
    // Try to delete non-existent idea
    try {
      await this.ideaManager.deleteIdea("another-fake-id");
    } catch (error) {
      console.log(`✅ Caught expected error: ${error.message}`);
    }
    
    // Try to get non-existent idea
    const nonExistent = this.ideaManager.getIdea("fake-id");
    console.log(`✅ Non-existent idea returns: ${nonExistent === undefined ? 'undefined' : 'value'}`);
  }
}

// Run the demonstration
async function main() {
  try {
    const app = new CreativeIdeasApp();
    
    // Additional demonstrations
    await app.demonstrateDeletion();
    await app.demonstrateErrorHandling();
    
    // Final dashboard
    console.log('\n🏁 FINAL DASHBOARD STATE');
    console.log('═'.repeat(50));
    const finalStats = app.ideaManager.getIdeaStatistics();
    console.log(`Final idea count: ${finalStats.totalIdeas}`);
    console.log(`Most common tag: ${app.ideaManager.getAllTags()[0] || 'None'}`);
    
  } catch (error) {
    console.error('❌ Application error:', error.message);
  }
}

// Run if this file is executed directly
// Run if executed directly
  main();
}


/*
Here's a comprehensive example that demonstrates how to use the `IdeaManager` class from `idea.js` with a practical command-line interface for managing creative ideas:

This example demonstrates a complete creative ideas management system that showcases:

1. **Idea Creation**: Creating ideas with different properties (importance, share level, tags, context)
2. **Search Functionality**: Searching ideas by keywords, tags, and content
3. **Filtering**: Filtering by importance, share level, and tags
4. **Updates**: Using the builder pattern to update ideas
5. **Statistics**: Generating comprehensive statistics and insights
6. **Error Handling**: Demonstrating proper error handling for invalid operations
7. **Deletion**: Safely removing ideas from the system

The example creates a realistic scenario where a creative team manages their innovation pipeline, making it easy to understand how the IdeaManager class can be used in real applications.
*/