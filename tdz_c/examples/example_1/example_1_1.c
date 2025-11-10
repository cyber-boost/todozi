#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "agent.c"  // Include the provided agent.c file

// Helper function to create a sample agent
struct Agent* create_sample_agent(const char* name, const char* description) {
    struct Agent* agent = malloc(sizeof(struct Agent));
    if (!agent) return NULL;
    
    agent->name = malloc(strlen(name) + 1);
    agent->description = malloc(strlen(description) + 1);
    strcpy(agent->name, name);
    strcpy(agent->description, description);
    
    // Sample capabilities
    char* caps[] = {"data_analysis", "reporting", "research"};
    string_array_copy(&agent->capabilities, &agent->capabilities_count, caps, 3);
    
    // Sample specializations
    char* specs[] = {"finance", "marketing"};
    string_array_copy(&agent->specializations, &agent->specializations_count, specs, 2);
    
    agent->metadata.status = AGENT_STATUS_AVAILABLE;
    agent->created_at = get_current_time();
    agent->updated_at = get_current_time();
    agent->id = NULL; // Will be set by manager
    
    return agent;
}

int main() {
    // Initialize agent manager
    struct AgentManager* manager = agent_manager_new();
    if (!manager) {
        printf("Failed to create agent manager\n");
        return 1;
    }
    
    // Create sample agents
    struct Agent* agent1 = create_sample_agent("Data Analyst", "Specializes in financial data analysis");
    struct Agent* agent2 = create_sample_agent("Marketing Specialist", "Expert in marketing campaign analysis");
    
    // Register agents with manager
    char* id1 = agent_manager_create_agent(manager, agent1);
    char* id2 = agent_manager_create_agent(manager, agent2);
    
    if (!id1 || !id2) {
        printf("Failed to create agents\n");
        agent_manager_free(manager);
        return 1;
    }
    
    printf("Created agents with IDs: %s and %s\n", id1, id2);
    
    // Update agent1's capabilities
    struct AgentUpdate* update = agent_update_new();
    char* new_caps[] = {"data_analysis", "visualization", "forecasting"};
    agent_update_capabilities(update, new_caps, 3);
    agent_update_description(update, "Senior financial data analyst with visualization skills");
    
    if (agent_manager_update_agent(manager, id1, update) == 0) {
        printf("Updated agent %s successfully\n", id1);
    }
    
    agent_update_free(update);
    
    // Find agents by specialization
    int count;
    struct Agent** finance_agents = agent_manager_get_agents_by_specialization(manager, "finance", &count);
    printf("Found %d agents with finance specialization\n", count);
    
    if (count > 0) {
        printf("First finance agent: %s\n", finance_agents[0]->name);
    }
    free(finance_agents); // Note: Don't free individual agents, just the array
    
    // Find agents by capability
    struct Agent** analysis_agents = agent_manager_get_agents_by_capability(manager, "data_analysis", &count);
    printf("Found %d agents with data_analysis capability\n", count);
    free(analysis_agents);
    
    // Assign a task to agent1
    char* assignment_id = agent_manager_assign_task_to_agent(manager, "task_001", id1, "project_alpha");
    if (assignment_id) {
        printf("Assigned task task_001 to agent %s with assignment ID: %s\n", id1, assignment_id);
        free(assignment_id);
    }
    
    // Check agent status
    struct Agent* assigned_agent = agent_manager_get_agent(manager, id1);
    printf("Agent %s status: %s\n", id1, 
           assigned_agent->metadata.status == AGENT_STATUS_BUSY ? "BUSY" : "AVAILABLE");
    
    // Complete the assignment
    if (agent_manager_complete_agent_assignment(manager, "task_001") == 0) {
        printf("Completed assignment for task task_001\n");
        printf("Agent %s status: %s\n", id1,
               assigned_agent->metadata.status == AGENT_STATUS_BUSY ? "BUSY" : "AVAILABLE");
    }
    
    // Get agent statistics
    struct AgentStatistics* stats = agent_manager_get_agent_statistics(manager);
    if (stats) {
        printf("\n--- Agent Statistics ---\n");
        printf("Total agents: %d\n", stats->total_agents);
        printf("Available agents: %d\n", stats->available_agents);
        printf("Busy agents: %d\n", stats->busy_agents);
        printf("Completion rate: %.2f%%\n", agent_statistics_completion_rate(stats));
        agent_statistics_free(stats);
    }
    
    // Find best agent for a task
    struct Agent* best_agent = agent_manager_find_best_agent(manager, "finance", "forecasting");
    if (best_agent) {
        printf("Best agent for finance/forecasting task: %s\n", best_agent->name);
    }
    
    // Get all assignments for an agent
    int assignment_count;
    struct AgentAssignment** assignments = agent_manager_get_agent_assignments(manager, id1, &assignment_count);
    printf("Agent %s has %d assignments\n", id1, assignment_count);
    if (assignment_count > 0) {
        printf("First assignment task ID: %s\n", assignments[0]->task_id);
    }
    free(assignments); // Free the array (not individual assignments)
    
    // Cleanup
    free(id1);
    free(id2);
    agent_manager_free(manager);
    
    return 0;
}
