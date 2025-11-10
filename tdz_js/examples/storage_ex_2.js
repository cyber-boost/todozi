// Example 2: Creating and Managing Projects with Tasks


import { Storage } from '../todozi/storage.js';
import { Project } from '../todozi/models.js';
import { v4: uuidv4 } from 'uuid.js';

async function demonstrateProjectManagement() {
    try {
        // Initialize storage system
        const storage = await Storage.new();
        
        // Create a new project
        const project = new Project("Website Redesign", "Complete overhaul of company website");
        await storage.createProject(project.name, project.description);
        console.log(`✅ Created project: ${project.name}`);
        
        // Add tasks to the project
        const tasks = [
            {
                id: uuidv4(),
                action: "Design new homepage layout",
                time: "4 hours",
                priority: "high",
                parent_project: project.name,
                status: "todo",
                assignee: "designer",
                tags: ["design", "ui"],
                context_notes: "Focus on mobile-first approach",
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
            },
            {
                id: uuidv4(),
                action: "Implement responsive navigation",
                time: "3 hours",
                priority: "medium",
                parent_project: project.name,
                status: "todo",
                assignee: "coder",
                tags: ["frontend", "javascript"],
                dependencies: [],
                context_notes: "Use React components",
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
            },
            {
                id: uuidv4(),
                action: "Write SEO optimized content",
                time: "2 hours",
                priority: "medium",
                parent_project: project.name,
                status: "todo",
                assignee: "content-writer",
                tags: ["content", "seo"],
                context_notes: "Target keywords: web design, responsive sites",
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
            }
        ];
        
        // Add tasks to project
        for (const task of tasks) {
            await storage.addTaskToProject(task);
            console.log(`✅ Added task: ${task.action}`);
        }
        
        // Retrieve and display project information
        const retrievedProject = await storage.getProject(project.name);
        const projectTasks = await storage.getProjectTasks(project.name);
        
        console.log('\n📋 Project Details:');
        console.log(`Name: ${retrievedProject.name}`);
        console.log(`Description: ${retrievedProject.description}`);
        console.log(`Status: ${retrievedProject.status}`);
        console.log(`Tasks: ${projectTasks.length}`);
        
        console.log('\n📝 Project Tasks:');
        projectTasks.forEach(task => {
            console.log(`- [${task.status}] ${task.action} (${task.priority})`);
            if (task.assignee) {
                console.log(`    Assignee: ${task.assignee}`);
            }
            if (task.tags && task.tags.length > 0) {
                console.log(`    Tags: ${task.tags.join(', ')}`);
            }
        });
        
        // Update a task status
        if (projectTasks.length > 0) {
            const firstTask = projectTasks[0];
            await storage.updateTaskInProject(firstTask.id, { status: 'in_progress' });
            console.log(`\n🔄 Updated task status: ${firstTask.action} -> in_progress`);
        }
        
        // Get project statistics
        const stats = await storage.getProjectStats(project.name);
        console.log('\n📊 Project Statistics:');
        console.log(`Total Tasks: ${stats.totalTasks}`);
        console.log(`Active Tasks: ${stats.activeTasks}`);
        console.log(`Completed Tasks: ${stats.completedTasks}`);
        
        // List all projects
        const allProjects = await storage.listProjects();
        console.log('\n📚 All Projects:');
        allProjects.forEach(p => {
            console.log(`- ${p.name}: ${p.description || 'No description'}`);
        });
        
    } catch (error) {
        console.error('❌ Error in project management demonstration:', error);
    }
}

// Run the demonstration
demonstrateProjectManagement();