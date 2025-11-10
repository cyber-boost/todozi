// Example 2: Creating a Project Management Dashboard with Todozi
// This example demonstrates how to use the Todozi library to create a simple project management dashboard
// that can create projects, add tasks, and display project statistics.


import { Done, Priority, Status } from '../todozi/lib.js';

class ProjectDashboard {
    constructor() {
        this.currentProject = 'general';
    }

    async initialize() {
        console.log('🚀 Initializing Todozi Dashboard...');
        await Done.init();
        console.log('✅ Todozi initialized successfully!');
    }

    async createNewProject(projectName, description) {
        try {
            await Done.createProject(projectName, description);
            console.log(`✅ Project '${projectName}' created successfully!`);
            this.currentProject = projectName;
        } catch (error) {
            console.error(`❌ Failed to create project: ${error.message}`);
        }
    }

    async addTaskToProject(action, priority = Priority.Medium, time = 'ASAP') {
        try {
            const task = await Done.createTask(
                action,
                priority,
                this.currentProject,
                time,
                `Added via dashboard to ${this.currentProject}`
            );
            console.log(`✅ Task added: ${task.action} (ID: ${task.id})`);
            return task.id;
        } catch (error) {
            console.error(`❌ Failed to add task: ${error.message}`);
        }
    }

    async updateTaskStatus(taskId, newStatus) {
        try {
            await Done.updateTaskStatus(taskId, newStatus);
            console.log(`✅ Task ${taskId} status updated to ${newStatus}`);
        } catch (error) {
            console.error(`❌ Failed to update task: ${error.message}`);
        }
    }

    async displayProjectStats() {
        try {
            const tasks = await Done.tasks(this.currentProject);
            const totalTasks = tasks.length;
            const completedTasks = tasks.filter(t => t.status === Status.Done).length;
            const inProgressTasks = tasks.filter(t => t.status === Status.InProgress).length;
            const pendingTasks = tasks.filter(t => t.status === Status.Todo).length;
            
            console.log(`\n📊 Project Statistics for '${this.currentProject}':`);
            console.log(`   Total Tasks: ${totalTasks}`);
            console.log(`   Completed: ${completedTasks}`);
            console.log(`   In Progress: ${inProgressTasks}`);
            console.log(`   Pending: ${pendingTasks}`);
            console.log(`   Completion Rate: ${totalTasks > 0 ? Math.round((completedTasks/totalTasks)*100) : 0}%`);
        } catch (error) {
            console.error(`❌ Failed to get project stats: ${error.message}`);
        }
    }

    async displayProjectTasks() {
        try {
            const tasks = await Done.tasks(this.currentProject);
            console.log(`\n📋 Tasks in '${this.currentProject}':`);
            if (tasks.length === 0) {
                console.log('   No tasks found.');
                return;
            }
            
            tasks.forEach((task, index) => {
                console.log(`   ${index + 1}. ${task.action}`);
                console.log(`      ID: ${task.id}`);
                console.log(`      Priority: ${task.priority}`);
                console.log(`      Status: ${task.status}`);
                console.log(`      Created: ${new Date(task.createdAt).toLocaleDateString()}`);
                console.log('');
            });
        } catch (error) {
            console.error(`❌ Failed to get project tasks: ${error.message}`);
        }
    }

    async searchTasks(query) {
        try {
            const results = await Done.findTasks(query);
            console.log(`\n🔍 Search results for '${query}':`);
            if (results.length === 0) {
                console.log('   No tasks found.');
                return;
            }
            
            results.slice(0, 5).forEach((task, index) => {
                console.log(`   ${index + 1}. ${task.action} (${task.parentProject})`);
                console.log(`      ID: ${task.id}`);
                console.log(`      Status: ${task.status}`);
                console.log('');
            });
        } catch (error) {
            console.error(`❌ Search failed: ${error.message}`);
        }
    }

    async generateProjectReport() {
        try {
            console.log('\n=== PROJECT DASHBOARD REPORT ===');
            await this.displayProjectStats();
            await this.displayProjectTasks();
            
            // Quick stats
            const overdueTasks = (await Done.allTasks()).filter(t => {
                try {
                    return new Date(t.time) < new Date() && t.status !== Status.Done;
                } catch {
                    return false;
                }
            }).length;
            
            console.log(`⚠️  Overdue Tasks: ${overdueTasks}`);
            console.log('================================\n');
        } catch (error) {
            console.error(`❌ Failed to generate report: ${error.message}`);
        }
    }
}

// Example usage
async function runDashboardDemo() {
    const dashboard = new ProjectDashboard();
    
    try {
        // Initialize the dashboard
        await dashboard.initialize();
        
        // Create a new project
        await dashboard.createNewProject('Website Redesign', 'Redesign company website for better UX');
        
        // Add some tasks
        const taskId1 = await dashboard.addTaskToProject(
            'Create wireframes for homepage',
            Priority.High,
            '2023-12-15'
        );
        
        const taskId2 = await dashboard.addTaskToProject(
            'Write content for about page',
            Priority.Medium,
            '2023-12-20'
        );
        
        const taskId3 = await dashboard.addTaskToProject(
            'Implement responsive design',
            Priority.Critical,
            '2023-12-25'
        );
        
        // Update a task status
        if (taskId1) {
            await dashboard.updateTaskStatus(taskId1, Status.InProgress);
        }
        
        // Display project information
        await dashboard.generateProjectReport();
        
        // Search for tasks
        await dashboard.searchTasks('homepage');
        
        console.log('\n🎉 Dashboard demo completed successfully!');
    } catch (error) {
        console.error(`Demo failed: ${error.message}`);
    }
}

// Run the demo if this file is executed directly
// Run if executed directly
    runDashboardDemo().catch(console.error);
}
