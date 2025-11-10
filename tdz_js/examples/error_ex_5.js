// example5_complete_workflow.js
// This example demonstrates a complete Todozi workflow

import { TodoziError, ErrorManager } from '../todozi/error.js';
import { TodoziHandler } from '../todozi/cli.js';
import { TodoziEmbeddingService, TodoziEmbeddingConfig } from '../todozi/emb.js';
import { Task, TaskCollection, Memory, Idea } from '../todozi/models.js';
import { Tag, TagManager } from '../todozi/tags.js';
import { processChatMessageExtended } from '../todozi/todozi.js';

/**
 * Example 5: Complete Todozi Workflow
 * 
 * This example demonstrates:
 * 1. Error handling and management
 * 2. Task creation and management
 * 3. Embedding generation and semantic search
 * 4. Memory and idea storage
 * 5. Tag management
 * 6. Processing complex chat messages
 */

class ExampleWorkflow {
    constructor() {
        this.errorManager = new ErrorManager();
        this.tagManager = new TagManager();
        this.embeddingConfig = TodoziEmbeddingConfig.default();
        this.storage = {
            tasks: new Map(),
            memories: new Map(),
            ideas: new Map(),
            errors: new Map()
        };
        this.embeddingService = null;
    }

    async initialize() {
        console.log('🔧 Initializing Todozi Example Workflow...');
        
        // Initialize embedding service
        this.embeddingService = await TodoziEmbeddingService.new(this.embeddingConfig);
        console.log('✅ Embedding service initialized');
        
        // Initialize tag manager
        await this.initializeTags();
        console.log('✅ Tag manager initialized');
        
        console.log('🚀 Todozi workflow ready!\n');
    }

    async initializeTags() {
        // Create common tags
        const tags = [
            { name: 'work', category: 'domain' },
            { name: 'personal', category: 'domain' },
            { name: 'urgent', category: 'priority' },
            { name: 'important', category: 'priority' },
            { name: 'frontend', category: 'tech' },
            { name: 'backend', category: 'tech' },
            { name: 'bug', category: 'type' },
            { name: 'feature', category: 'type' }
        ];

        for (const tagData of tags) {
            const tag = new Tag(tagData);
            const id = await this.tagManager.createTag(tag);
            console.log(`  Created tag: ${tag.name} (${id})`);
        }
    }

    async demonstrateErrorHandling() {
        console.log('🔍 Demonstrating Error Handling\n');
        
        try {
            // Simulate various error scenarios
            const errors = [
                TodoziError.taskNotFound('task_123'),
                TodoziError.invalidPriority('super_high'),
                TodoziError.validation('Invalid input provided'),
                TodoziError.storage('Could not connect to database')
            ];

            for (const error of errors) {
                const errorId = await this.errorManager.createError(error);
                console.log(`📝 Created error: ${error.type} (${errorId})`);
            }

            // Show unresolved errors
            const unresolvedErrors = this.errorManager.getUnresolvedErrors();
            console.log(`\n❌ Unresolved errors: ${unresolvedErrors.length}`);
            
            // Resolve one error
            if (unresolvedErrors.length > 0) {
                const firstErrorId = unresolvedErrors[0].id;
                await this.errorManager.resolveError(firstErrorId, 'Fixed by example workflow');
                console.log(`✅ Resolved error: ${firstErrorId}\n`);
            }

        } catch (error) {
            console.error(`❌ Error in error handling demo: ${error.message}`);
        }
    }

    async demonstrateTaskManagement() {
        console.log('📋 Demonstrating Task Management\n');
        
        // Create sample tasks
        const tasks = [
            {
                action: 'Fix responsive layout on mobile',
                time: '2 hours',
                priority: 'high',
                parentProject: 'website-redesign',
                status: 'todo',
                assignee: 'human',
                tags: ['frontend', 'bug'],
                contextNotes: 'Homepage layout breaks on iPhone screens'
            },
            {
                action: 'Implement user authentication system',
                time: '8 hours',
                priority: 'critical',
                parentProject: 'website-redesign',
                status: 'in_progress',
                assignee: 'ai',
                tags: ['backend', 'feature'],
                contextNotes: 'Need OAuth2 + JWT implementation'
            },
            {
                action: 'Review pull requests',
                time: '1 hour',
                priority: 'medium',
                parentProject: 'general',
                status: 'todo',
                assignee: 'human',
                tags: ['work', 'important'],
                contextNotes: 'Check PRs #42, #43, #44'
            }
        ];

        for (const taskData of tasks) {
            const task = new Task({
                userId: 'user123',
                action: taskData.action,
                time: taskData.time,
                priority: taskData.priority,
                parentProject: taskData.parentProject,
                status: taskData.status,
                assignee: taskData.assignee,
                tags: taskData.tags,
                contextNotes: taskData.contextNotes
            });

            this.storage.tasks.set(task.id, task);
            
            // Generate embedding for the task
            const taskContent = this.embeddingService.prepareTaskContent(task);
            const embedding = await this.embeddingService.generateEmbedding(taskContent);
            task.embeddingVector = embedding;
            
            console.log(`📝 Created task: ${task.id}`);
            console.log(`   Action: ${task.action}`);
            console.log(`   Priority: ${task.priority}`);
            console.log(`   Project: ${task.parentProject}`);
            console.log(`   Status: ${task.status}`);
            console.log(`   Tags: ${task.tags.join(', ')}\n`);
        }
    }

    async demonstrateSemanticSearch() {
        console.log('🔍 Demonstrating Semantic Search\n');
        
        const searchQueries = [
            'fix mobile layout',
            'authentication system',
            'review code changes',
            'frontend bug'
        ];

        for (const query of searchQueries) {
            console.log(`Searching for: "${query}"`);
            
            const results = await this.embeddingService.findSimilarTasks(query, 3);
            
            if (results.length === 0) {
                console.log('  No similar tasks found\n');
                continue;
            }

            console.log(`  Found ${results.length} similar tasks:`);
            for (const result of results) {
                const similarityPercent = (result.similarityScore * 100).toFixed(1);
                console.log(`    - ${result.matchedContent.split('\n')[0]} (${similarityPercent}% similar)\n`);
            }
        }
    }

    async demonstrateMemoryStorage() {
        console.log('🧠 Demonstrating Memory Storage\n');
        
        const memories = [
            {
                moment: 'Client meeting about redesign requirements',
                meaning: 'Key stakeholder feedback on current website',
                reason: 'To understand client expectations',
                importance: 'high',
                term: 'long',
                memoryType: 'standard',
                tags: ['client', 'requirements', 'meeting']
            },
            {
                moment: 'Team discussed using React for frontend',
                meaning: 'Decision on frontend technology stack',
                reason: 'To standardize development approach',
                importance: 'medium',
                term: 'long',
                memoryType: 'standard',
                tags: ['technology', 'decision', 'frontend']
            }
        ];

        for (const memoryData of memories) {
            const memory = {
                id: `memory_${Date.now()}`,
                userId: 'user123',
                projectId: 'website-redesign',
                status: 'active',
                ...memoryData,
                createdAt: new Date(),
                updatedAt: new Date()
            };

            this.storage.memories.set(memory.id, memory);
            
            // Generate embedding for memory
            const memoryContent = this.embeddingService.prepareMemoryContent(memory);
            const embedding = await this.embeddingService.generateEmbedding(memoryContent);
            
            console.log(`💾 Stored memory: ${memory.id}`);
            console.log(`   Moment: ${memory.moment}`);
            console.log(`   Meaning: ${memory.meaning}\n`);
        }
    }

    async demonstrateIdeaStorage() {
        console.log('💡 Demonstrating Idea Storage\n');
        
        const ideas = [
            {
                idea: 'Add dark mode toggle to improve user experience',
                share: 'team',
                importance: 'medium',
                tags: ['ux', 'feature'],
                context: 'Based on user feedback about eye strain'
            },
            {
                idea: 'Create automated testing pipeline for CI/CD',
                share: 'team',
                importance: 'high',
                tags: ['automation', 'testing', 'devops'],
                context: 'To catch bugs before deployment'
            }
        ];

        for (const ideaData of ideas) {
            const idea = {
                id: `idea_${Date.now()}`,
                idea: ideaData.idea,
                projectId: 'website-redesign',
                status: 'active',
                share: 'team',
                importance: ideaData.importance,
                tags: ideaData.tags,
                context: ideaData.context,
                createdAt: new Date(),
                updatedAt: new Date()
            };

            this.storage.ideas.set(idea.id, idea);
            
            console.log(`💾 Stored idea: ${idea.id}`);
            console.log(`   Idea: ${idea.idea}`);
            console.log(`   Importance: ${idea.importance}\n`);
        }
    }

    async demonstrateTagManagement() {
        console.log('🏷️  Demonstrating Tag Management\n');
        
        // Search for tasks by tag
        const frontendTasks = Array.from(this.storage.tasks.values())
            .filter(task => task.tags.includes('frontend'));
        
        console.log(`🔍 Found ${frontendTasks.length} frontend tasks:`);
        for (const task of frontendTasks) {
            console.log(`  - ${task.action}\n`);
        }

        // Get tag statistics
        const stats = this.tagManager.getTagStatistics();
        console.log('📊 Tag Statistics:');
        console.log(`   Total tags: ${stats.total_tags}`);
        console.log(`   Total categories: ${stats.total_categories}`);
        console.log(`   Average usage: ${stats.average_usage.toFixed(2)}\n`);
    }

    async demonstrateChatProcessing() {
        console.log('💬 Demonstrating Chat Message Processing\n');
        
        const complexMessage = `
I need to work on a few things:

<todozi>Fix CSS grid layout on dashboard;3 hours;high;website-redesign;todo;human;frontend,layout;dashboard_grid;Fix alignment issues on mobile screens;0</todozi>

<memory>Discussed API design with team;Decision on REST vs GraphQL;To choose best approach for our use case;high;long;standard;api,design,decision</memory>

<idea>Add real-time collaboration features;team;high;collaboration,realtime;Allow multiple users to edit simultaneously</idea>

<error>Database connection timeout;Frequent disconnections during peak hours;high;database;server;timeout,database;Needs investigation</error>
        `;

        console.log('Processing complex chat message with multiple types of content...\n');
        
        const content = processChatMessageExtended(complexMessage, 'user123');
        
        console.log('📊 Extracted Content:');
        console.log(`   📋 Tasks: ${content.tasks.length}`);
        console.log(`   🧠 Memories: ${content.memories.length}`);
        console.log(`   💡 Ideas: ${content.ideas.length}`);
        console.log(`   ❌ Errors: ${content.errors.length}\n`);

        // Process each type of content
        for (const task of content.tasks) {
            this.storage.tasks.set(task.id, task);
            console.log(`✅ Saved task: ${task.action}`);
        }

        for (const memory of content.memories) {
            this.storage.memories.set(memory.id, memory);
            console.log(`✅ Saved memory: ${memory.moment}`);
        }

        for (const idea of content.ideas) {
            this.storage.ideas.set(idea.id, idea);
            console.log(`✅ Saved idea: ${idea.idea}`);
        }

        for (const error of content.errors) {
            this.storage.errors.set(error.id, error);
            await this.errorManager.createError(
                TodoziError.api(`Application error: ${error.title}`)
            );
            console.log(`✅ Saved error: ${error.title}`);
        }
    }

    async demonstrateWorkflow() {
        console.log('🚀 Demonstrating Complete Workflow\n');
        
        // Step 1: Create a task based on an error
        const recentErrors = this.storage.errors;
        if (recentErrors.size > 0) {
            const firstError = Array.from(recentErrors.values())[0];
            
            const task = new Task({
                userId: 'user123',
                action: `Fix: ${firstError.title}`,
                time: '4 hours',
                priority: 'high',
                parentProject: 'website-redesign',
                status: 'todo',
                assignee: 'ai',
                tags: ['bug', 'priority_high'],
                contextNotes: firstError.description
            });

            this.storage.tasks.set(task.id, task);
            console.log(`📝 Created task from error: ${task.action}`);
        }

        // Step 2: Find related ideas
        const relatedIdeas = Array.from(this.storage.ideas.values())
            .filter(idea => idea.tags.includes('feature') || idea.tags.includes('bug'));
        
        console.log(`\n💡 Found ${relatedIdeas.length} related ideas:`);
        for (const idea of relatedIdeas.slice(0, 2)) {
            console.log(`   - ${idea.idea}`);
        }

        // Step 3: Show final status
        console.log('\n📊 Final Status:');
        console.log(`   Total tasks: ${this.storage.tasks.size}`);
        console.log(`   Total memories: ${this.storage.memories.size}`);
        console.log(`   Total ideas: ${this.storage.ideas.size}`);
        console.log(`   Total errors: ${this.storage.errors.size}`);
        console.log(`   Unresolved errors: ${this.errorManager.getUnresolvedErrors().length}`);
    }

    async run() {
        try {
            await this.initialize();
            await this.demonstrateErrorHandling();
            await this.demonstrateTaskManagement();
            await this.demonstrateSemanticSearch();
            await this.demonstrateMemoryStorage();
            await this.demonstrateIdeaStorage();
            await this.demonstrateTagManagement();
            await this.demonstrateChatProcessing();
            await this.demonstrateWorkflow();
            
            console.log('\n🎉 Example workflow completed successfully!');
            console.log('This demonstrates the integration of all Todozi components.\n');
            
        } catch (error) {
            console.error(`\n❌ Workflow failed: ${error.message}`);
            console.error(error.stack);
        }
    }
}

// Run the example
const workflow = new ExampleWorkflow();
workflow.run().catch(console.error);


/*
# Example 5: Complete Todozi Workflow Example

This example demonstrates a comprehensive workflow using the Todozi system, showing how errors, tasks, embeddings, and storage work together in a real-world scenario.

## How to Run This Example

1. **Save the files** as shown above in your project directory
2. **Install dependencies** (if using external packages):

/ *
bash
   npm install uuid luxon

/ *
3. **Run the example**:
*/