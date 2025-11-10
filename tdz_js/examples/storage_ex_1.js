// todozi-project-manager.js
import { Storage, saveProject, loadProject, listProjects, loadProjectTaskContainer, saveProjectTaskContainer } from './storage.js';

class ProjectManager {
    constructor(storage) {
        this.storage = storage;
    }
    
    /**
     * Create a new project with initial tasks
     */
    async createProject(name, description, initialTasks = []) {
        try {
            // Create the project
            const project = {
                name: name,
                description: description || `Tasks for ${name} project`
            };
            
            await saveProject(project);
            console.log(`✅ Created project: ${name}`);
            
            // Add initial tasks to the project
            for (const taskData of initialTasks) {
                await this.addTaskToProject(name, taskData);
            }
            
            return project;
        } catch (error) {
            console.error(`❌ Failed to create project ${name}:`, error.message);
            throw error;
        }
    }
    
    /**
     * Add a task to a specific project
     */
    async addTaskToProject(projectName, taskData) {
        try {
            const task = {
                id: crypto.randomUUID(),
                action: taskData.action || 'Untitled task',
                parent_project: projectName,
                status: taskData.status || 'todo',
                priority: taskData.priority || 'medium',
                context_notes: taskData.context || null,
                tags: taskData.tags || [],
                dependencies: taskData.dependencies || [],
                created_at: new Date(),
                updated_at: new Date(),
                embedding_vector: null
            };
            
            await this.storage.addTaskToProject(task);
            console.log(`📋 Added task to ${projectName}: ${task.action}`);
            return task.id;
        } catch (error) {
            console.error(`❌ Failed to add task to ${projectName}:`, error.message);
            throw error;
        }
    }
    
    /**
     * Get all tasks from a project
     */
    async getProjectTasks(projectName) {
        try {
            const container = await loadProjectTaskContainer(projectName);
            const tasks = container.get_all_tasks();
            
            console.log(`📊 Found ${tasks.length} tasks in project: ${projectName}`);
            return tasks;
        } catch (error) {
            console.error(`❌ Failed to get tasks for ${projectName}:`, error.message);
            throw error;
        }
    }
    
    /**
     * Update task status in a project
     */
    async updateTaskStatus(projectName, taskId, newStatus) {
        try {
            const container = await loadProjectTaskContainer(projectName);
            const updatedTask = container.update_task_status(taskId, newStatus);
            
            if (updatedTask) {
                await saveProjectTaskContainer(container);
                console.log(`🔄 Updated task ${taskId} to status: ${newStatus}`);
                return updatedTask;
            } else {
                throw new Error(`Task ${taskId} not found in project ${projectName}`);
            }
        } catch (error) {
            console.error(`❌ Failed to update task status:`, error.message);
            throw error;
        }
    }
    
    /**
     * Get project statistics
     */
    async getProjectStats(projectName) {
        try {
            const tasks = await this.getProjectTasks(projectName);
            
            const stats = {
                total: tasks.length,
                todo: tasks.filter(t => t.status === 'todo').length,
                in_progress: tasks.filter(t => t.status === 'in_progress').length,
                done: tasks.filter(t => t.status === 'done').length,
                archived: tasks.filter(t => t.status === 'archived').length,
                priorities: {
                    low: tasks.filter(t => t.priority === 'low').length,
                    medium: tasks.filter(t => t.priority === 'medium').length,
                    high: tasks.filter(t => t.priority === 'high').length,
                    critical: tasks.filter(t => t.priority === 'critical').length
                }
            };
            
            return stats;
        } catch (error) {
            console.error(`❌ Failed to get stats for ${projectName}:`, error.message);
            throw error;
        }
    }
    
    /**
     * List all projects with summary information
     */
    async listAllProjects() {
        try {
            const projects = await listProjects();
            const projectSummaries = [];
            
            for (const project of projects) {
                const tasks = await this.getProjectTasks(project.name);
                const stats = await this.getProjectStats(project.name);
                
                projectSummaries.push({
                    name: project.name,
                    description: project.description,
                    taskCount: tasks.length,
                    stats: stats
                });
            }
            
            return projectSummaries;
        } catch (error) {
            console.error(`❌ Failed to list projects:`, error.message);
            throw error;
        }
    }
}

// Usage Example
async function main() {
    try {
        // Initialize storage
        await initStorage();
        const storage = await Storage.new();
        const projectManager = new ProjectManager(storage);
        
        // Create a web development project
        const webProjectTasks = [
            {
                action: "Design homepage layout",
                priority: "high",
                status: "todo",
                tags: ["design", "frontend"]
            },
            {
                action: "Set up database schema",
                priority: "medium",
                status: "in_progress",
                tags: ["backend", "database"]
            },
            {
                action: "Write API documentation",
                priority: "low",
                status: "todo",
                tags: ["documentation", "api"]
            }
        ];
        
        await projectManager.createProject(
            "web-app", 
            "Full-stack web application development", 
            webProjectTasks
        );
        
        // Create a mobile app project
        const mobileProjectTasks = [
            {
                action: "Create mobile UI mockups",
                priority: "high",
                status: "todo",
                tags: ["design", "mobile"]
            },
            {
                action: "Set up React Native project",
                priority: "medium",
                status: "todo",
                tags: ["setup", "react-native"]
            }
        ];
        
        await projectManager.createProject(
            "mobile-app",
            "Cross-platform mobile application",
            mobileProjectTasks
        );
        
        // Add additional tasks to web project
        await projectManager.addTaskToProject("web-app", {
            action: "Implement user authentication",
            priority: "critical",
            status: "todo",
            tags: ["auth", "security"]
        });
        
        // Update a task status
        const webTasks = await projectManager.getProjectTasks("web-app");
        if (webTasks.length > 0) {
            await projectManager.updateTaskStatus("web-app", webTasks[0].id, "in_progress");
        }
        
        // Get project statistics
        const webStats = await projectManager.getProjectStats("web-app");
        console.log("📈 Web App Project Stats:", webStats);
        
        // List all projects
        const allProjects = await projectManager.listAllProjects();
        console.log("📁 All Projects Summary:");
        allProjects.forEach(project => {
            console.log(`- ${project.name}: ${project.taskCount} tasks`);
            console.log(`  Status: ${project.stats.todo} todo, ${project.stats.in_progress} in progress, ${project.stats.done} done`);
        });
        
    } catch (error) {
        console.error("Application error:", error);
    }
}

// Run the example
main().catch(console.error);

/*
# Example 1: Creating and Managing Projects with Tasks

This example demonstrates how to use the Todozi storage system to create projects, add tasks, and manage them programmatically.

## Key Features Demonstrated:

1. **Project Creation**: Creates new projects with descriptions
2. **Task Management**: Adds, updates, and retrieves tasks within projects
3. **Status Tracking**: Updates task status (todo → in_progress → done)
4. **Statistics**: Provides project-level statistics and summaries
5. **Error Handling**: Comprehensive error handling for storage operations

## Sample Output:

/ *
✅ Created project: web-app
📋 Added task to web-app: Design homepage layout
📋 Added task to web-app: Set up database schema
📋 Added task to web-app: Write API documentation
✅ Created project: mobile-app
📋 Added task to mobile-app: Create mobile UI mockups
📋 Added task to mobile-app: Set up React Native project
📋 Added task to web-app: Implement user authentication
🔄 Updated task task_abc123 to status: in_progress
📈 Web App Project Stats: { total: 4, todo: 2, in_progress: 2, done: 0, archived: 0, priorities: {...} }
📁 All Projects Summary:
- web-app: 4 tasks
  Status: 2 todo, 2 in progress, 0 done
- mobile-app: 2 tasks
  Status: 2 todo, 0 in progress, 0 done
*/