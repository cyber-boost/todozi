#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <uuid/uuid.h>

// Include the agent.c file or its header here
// For this example, we assume the structures and functions are available

void print_agent_info(const struct Agent* agent) {
    if (!agent) return;
    
    printf("Agent ID: %s\n", agent->id);
    printf("Name: %s\n", agent->name);
    printf("Description: %s\n", agent->description);
    printf("Status: %d\n", agent->metadata.status);
    printf("Created: %s", ctime(&agent->created_at));
    printf("Updated: %s", ctime(&agent->updated_at));
    
    printf("Capabilities: ");
    for (int i = 0; i < agent->capabilities_count; i++) {
        printf("%s ", agent->capabilities[i]);
    }
    printf("\n");
    
    printf("Specializations: ");
    for (int i = 0; i < agent->specializations_count; i++) {
        printf("%s ", agent->specializations[i]);
    }
    printf("\n\n");
}

void print_assignment_info(const struct AgentAssignment* assignment) {
    if (!assignment) return;
    
    printf("Task ID: %s\n", assignment->task_id);
    printf("Agent ID: %s\n", assignment->agent_id);
    printf("Project ID: %s\n", assignment->project_id);
    printf("Assigned: %s", ctime(&assignment->assigned_at));
    printf("Status: %d\n\n", assignment->status);
}

int main() {
    // Initialize agent manager
    struct AgentManager* manager = agent_manager_new();
    if (!manager) {
        printf("Failed to create agent manager\n");
        return 1;
    }
    
    // Create some agents
    struct Agent agent1;
    agent1.name = "Data Analyzer";
    agent1.description = "Analyzes large datasets";
    char* caps1[] = {"data_processing", "statistics"};
    agent1.capabilities = caps1;
    agent1.capabilities_count = 2;
    char* specs1[] = {"data_science", "machine_learning"};
    agent1.specializations = specs1;
    agent1.specializations_count = 2;
    agent1.metadata.status = AGENT_STATUS_AVAILABLE;
    
    char* agent_id1 = agent_manager_create_agent(manager, &agent1);
    if (!agent_id1) {
        printf("Failed to create agent 1\n");
        agent_manager_free(manager);
        return 1;
    }
    printf("Created agent 1 with ID: %s\n\n", agent_id1);
    
    struct Agent agent2;
    agent2.name = "Web Scraper";
    agent2.description = "Extracts data from websites";
    char* caps2[] = {"web_scraping", "data_extraction"};
    agent2.capabilities = caps2;
    agent2.capabilities_count = 2;
    char* specs2[] = {"data_collection", "web_technologies"};
    agent2.specializations = specs2;
    agent2.specializations_count = 2;
    agent2.metadata.status = AGENT_STATUS_AVAILABLE;
    
    char* agent_id2 = agent_manager_create_agent(manager, &agent2);
    if (!agent_id2) {
        printf("Failed to create agent 2\n");
        free(agent_id1);
        agent_manager_free(manager);
        return 1;
    }
    printf("Created agent 2 with ID: %s\n\n", agent_id2);
    
    // Display all agents
    printf("=== All Agents ===\n");
    int count;
    struct Agent** all_agents = agent_manager_get_all_agents(manager, &count);
    for (int i = 0; i < count; i++) {
        print_agent_info(all_agents[i]);
    }
    
    // Assign a task to an agent
    char task_id1[] = "TASK-001";
    char project_id1[] = "PROJECT-ABC";
    
    char* assignment_result = agent_manager_assign_task_to_agent(manager, task_id1, agent_id1, project_id1);
    if (!assignment_result) {
        printf("Failed to assign task to agent\n");
        free(agent_id1);
        free(agent_id2);
        agent_manager_free(manager);
        return 1;
    }
    printf("Assigned task %s to agent %s\n\n", assignment_result, agent_id1);
    free(assignment_result);
    
    // Check agent status after assignment
    struct Agent* updated_agent = agent_manager_get_agent(manager, agent_id1);
    printf("=== Agent After Assignment ===\n");
    print_agent_info(updated_agent);
    
    // Get assignments for agent
    printf("=== Assignments for Agent %s ===\n", agent_id1);
    int assignment_count;
    struct AgentAssignment** assignments = agent_manager_get_agent_assignments(manager, agent_id1, &assignment_count);
    for (int i = 0; i < assignment_count; i++) {
        print_assignment_info(assignments[i]);
    }
    free(assignments); // Note: Only free the array, not the assignments themselves
    
    // Complete the assignment
    if (agent_manager_complete_agent_assignment(manager, task_id1) == 0) {
        printf("Completed assignment for task %s\n\n", task_id1);
    }
    
    // Check agent status after completion
    updated_agent = agent_manager_get_agent(manager, agent_id1);
    printf("=== Agent After Completion ===\n");
    print_agent_info(updated_agent);
    
    // Find best agent for a task
    struct Agent* best_agent = agent_manager_find_best_agent(manager, "data_science", "data_processing");
    if (best_agent) {
        printf("=== Best Agent for Data Science Task ===\n");
        print_agent_info(best_agent);
    }
    
    // Get agent statistics
    struct AgentStatistics* stats = agent_manager_get_agent_statistics(manager);
    if (stats) {
        printf("=== Agent Statistics ===\n");
        printf("Total Agents: %d\n", stats->total_agents);
        printf("Available Agents: %d\n", stats->available_agents);
        printf("Busy Agents: %d\n", stats->busy_agents);
        printf("Inactive Agents: %d\n", stats->inactive_agents);
        printf("Total Assignments: %d\n", stats->total_assignments);
        printf("Completed Assignments: %d\n", stats->completed_assignments);
        printf("Completion Rate: %.2f%%\n\n", agent_statistics_completion_rate(stats));
        agent_statistics_free(stats);
    }
    
    // Update an agent
    struct AgentUpdate* update = agent_update_new();
    if (update) {
        agent_update_description(update, "Advanced data analyzer with AI capabilities");
        agent_update_capabilities(update, (char*[]){"data_processing", "AI_analysis", "visualization"}, 3);
        
        if (agent_manager_update_agent(manager, agent_id2, update) == 0) {
            printf("Updated agent %s\n", agent_id2);
            struct Agent* updated_agent2 = agent_manager_get_agent(manager, agent_id2);
            printf("=== Updated Agent ===\n");
            print_agent_info(updated_agent2);
        }
        agent_update_free(update);
    }
    
    // Cleanup
    free(agent_id1);
    free(agent_id2);
    agent_manager_free(manager);
    
    return 0;
}
