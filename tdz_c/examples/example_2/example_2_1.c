#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "agent.c" // Include the agent implementation

// Helper function to create an agent with basic information
struct Agent* create_sample_agent(const char* name, const char* description) {
    struct Agent* agent = malloc(sizeof(struct Agent));
    if (!agent) return NULL;
    
    agent->name = malloc(strlen(name) + 1);
    strcpy(agent->name, name);
    
    agent->description = malloc(strlen(description) + 1);
    strcpy(agent->description, description);
    
    // Initialize capabilities
    agent->capabilities = malloc(2 * sizeof(char*));
    agent->capabilities[0] = malloc(20);
    strcpy(agent->capabilities[0], "data_analysis");
    agent->capabilities[1] = malloc(15);
    strcpy(agent->capabilities[1], "reporting");
    agent->capabilities_count = 2;
    
    // Initialize specializations
    agent->specializations = malloc(2 * sizeof(char*));
    agent->specializations[0] = malloc(20);
    strcpy(agent->specializations[0], "financial_audit");
    agent->specializations[1] = malloc(15);
    strcpy(agent->specializations[1], "compliance");
    agent->specializations_count = 2;
    
    agent->metadata.status = AGENT_STATUS_AVAILABLE;
    agent->created_at = time(NULL);
    agent->updated_at = time(NULL);
    
    return agent;
}

// Helper function to print agent information
void print_agent_info(struct Agent* agent) {
    if (!agent) {
        printf("Agent not found\n");
        return;
    }
    
    printf("Agent ID: %s\n", agent->id);
    printf("Name: %s\n", agent->name);
    printf("Description: %s\n", agent->description);
    printf("Status: %d\n", agent->metadata.status);
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

// Helper function to print assignment information
void print_assignment_info(struct AgentAssignment* assignment) {
    if (!assignment) {
        printf("Assignment not found\n");
        return;
    }
    
    printf("Task ID: %s\n", assignment->task_id);
    printf("Agent ID: %s\n", assignment->agent_id);
    printf("Project ID: %s\n", assignment->project_id);
    printf("Status: %d\n\n", assignment->status);
}

int main() {
    // Initialize agent manager
    struct AgentManager* manager = agent_manager_new();
    if (!manager) {
        printf("Failed to create agent manager\n");
        return 1;
    }
    
    printf("=== Agent Management System Demo ===\n\n");
    
    // Create sample agents
    printf("1. Creating sample agents...\n");
    struct Agent* agent1 = create_sample_agent("Audit Agent", "Handles financial audits");
    struct Agent* agent2 = create_sample_agent("Compliance Agent", "Manages compliance tasks");
    
    // Register agents with the manager
    char* agent1_id = agent_manager_create_agent(manager, agent1);
    char* agent2_id = agent_manager_create_agent(manager, agent2);
    
    if (!agent1_id || !agent2_id) {
        printf("Failed to create agents\n");
        agent_manager_free(manager);
        return 1;
    }
    
    printf("Created Agent 1 with ID: %s\n", agent1_id);
    printf("Created Agent 2 with ID: %s\n\n", agent2_id);
    
    // Retrieve and display agent information
    printf("2. Retrieving agent information...\n");
    struct Agent* retrieved_agent = agent_manager_get_agent(manager, agent1_id);
    print_agent_info(retrieved_agent);
    
    // Update agent information
    printf("3. Updating agent information...\n");
    struct AgentUpdate* update = agent_update_new();
    agent_update_description(update, "Specialized financial audit expert");
    agent_update_specializations(update, (char*[]){"financial_audit", "risk_assessment"}, 2);
    
    if (agent_manager_update_agent(manager, agent1_id, update) == 0) {
        printf("Agent updated successfully\n");
        retrieved_agent = agent_manager_get_agent(manager, agent1_id);
        print_agent_info(retrieved_agent);
    }
    agent_update_free(update);
    
    // Assign tasks to agents
    printf("4. Assigning tasks to agents...\n");
    char* task1_id = agent_manager_assign_task_to_agent(manager, "TASK-001", agent1_id, "PROJECT-A");
    char* task2_id = agent_manager_assign_task_to_agent(manager, "TASK-002", agent2_id, "PROJECT-B");
    
    if (task1_id && task2_id) {
        printf("Assigned Task 1 (TASK-001) to Agent 1\n");
        printf("Assigned Task 2 (TASK-002) to Agent 2\n\n");
    }
    
    // Check agent status after assignment
    printf("5. Checking agent status after assignment...\n");
    retrieved_agent = agent_manager_get_agent(manager, agent1_id);
    printf("Agent 1 status: %d (should be BUSY)\n", retrieved_agent->metadata.status);
    retrieved_agent = agent_manager_get_agent(manager, agent2_id);
    printf("Agent 2 status: %d (should be BUSY)\n\n", retrieved_agent->metadata.status);
    
    // Get assignments for an agent
    printf("6. Retrieving assignments for Agent 1...\n");
    int assignment_count;
    struct AgentAssignment** assignments = agent_manager_get_agent_assignments(manager, agent1_id, &assignment_count);
    if (assignments && assignment_count > 0) {
        print_assignment_info(assignments[0]);
    }
    
    // Complete an assignment
    printf("7. Completing assignment for Task 1...\n");
    if (agent_manager_complete_agent_assignment(manager, task1_id) == 0) {
        printf("Task 1 completed successfully\n");
        
        // Check agent status after completion
        retrieved_agent = agent_manager_get_agent(manager, agent1_id);
        printf("Agent 1 status: %d (should be AVAILABLE)\n\n", retrieved_agent->metadata.status);
    }
    
    // Find best agent for a specialization
    printf("8. Finding best agent for financial_audit specialization...\n");
    struct Agent* best_agent = agent_manager_find_best_agent(manager, "financial_audit", "data_analysis");
    if (best_agent) {
        printf("Best agent for financial_audit: %s\n\n", best_agent->name);
    }
    
    // Get agent statistics
    printf("9. Retrieving agent statistics...\n");
    struct AgentStatistics* stats = agent_manager_get_agent_statistics(manager);
    if (stats) {
        printf("Total Agents: %d\n", stats->total_agents);
        printf("Available Agents: %d\n", stats->available_agents);
        printf("Busy Agents: %d\n", stats->busy_agents);
        printf("Completed Assignments: %d/%d\n", stats->completed_assignments, stats->total_assignments);
        printf("Completion Rate: %.2f%%\n\n", agent_statistics_completion_rate(stats));
        agent_statistics_free(stats);
    }
    
    // Clean up
    free(agent1_id);
    free(agent2_id);
    free(task1_id);
    free(task2_id);
    agent_manager_free(manager);
    
    printf("Demo completed successfully!\n");
    return 0;
}
