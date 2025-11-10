// example4.c - Task Assignment and Statistics Tracking

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include the agent management system
extern struct AgentManager* agent_manager_new();
extern void agent_manager_free(struct AgentManager* manager);
extern char* agent_manager_create_agent(struct AgentManager* manager, struct Agent* agent);
extern struct Agent* agent_manager_get_agent(const struct AgentManager* manager, const char* agent_id);
extern struct Agent** agent_manager_get_available_agents(const struct AgentManager* manager, int* count);
extern char* agent_manager_assign_task_to_agent(struct AgentManager* manager, char* task_id, const char* agent_id, char* project_id);
extern int agent_manager_complete_agent_assignment(struct AgentManager* manager, const char* task_id);
extern struct AgentStatistics* agent_manager_get_agent_statistics(const struct AgentManager* manager);
extern void agent_statistics_free(struct AgentStatistics* stats);
extern double agent_statistics_completion_rate(const struct AgentStatistics* stats);

// Helper functions from agent.c
extern time_t get_current_time();
extern int string_array_copy(char*** dest, int* dest_count, char** src, int src_count);

int main() {
    printf("=== Example 4: Task Assignment and Statistics ===\n\n");

    // Create agent manager
    struct AgentManager* manager = agent_manager_new();
    if (!manager) {
        printf("Failed to create agent manager\n");
        return 1;
    }

    // Create developer agents
    struct Agent agent1 = {0};
    agent1.name = "Alice Developer";
    agent1.description = "Senior backend developer";
    char* backend_caps[] = {"Python", "Django", "PostgreSQL"};
    char* backend_specs[] = {"Backend Development", "Database Design"};
    string_array_copy(&agent1.capabilities, &agent1.capabilities_count, backend_caps, 3);
    string_array_copy(&agent1.specializations, &agent1.specializations_count, backend_specs, 2);
    agent1.metadata.status = 0; // AGENT_STATUS_AVAILABLE
    agent1.created_at = get_current_time();
    agent1.updated_at = get_current_time();

    struct Agent agent2 = {0};
    agent2.name = "Bob Frontend";
    agent2.description = "React specialist";
    char* frontend_caps[] = {"JavaScript", "React", "CSS"};
    char* frontend_specs[] = {"Frontend Development", "UI/UX"};
    string_array_copy(&agent2.capabilities, &agent2.capabilities_count, frontend_caps, 3);
    string_array_copy(&agent2.specializations, &agent2.specializations_count, frontend_specs, 2);
    agent2.metadata.status = 0; // AGENT_STATUS_AVAILABLE
    agent2.created_at = get_current_time();
    agent2.updated_at = get_current_time();

    // Register agents
    char* id1 = agent_manager_create_agent(manager, &agent1);
    char* id2 = agent_manager_create_agent(manager, &agent2);
    
    if (!id1 || !id2) {
        printf("Failed to create agents\n");
        agent_manager_free(manager);
        return 1;
    }

    printf("Created agents:\n");
    printf("- %s (ID: %s)\n", agent1.name, id1);
    printf("- %s (ID: %s)\n\n", agent2.name, id2);

    // Assign tasks
    char task1_id[] = "TASK-001";
    char project_id[] = "PROJECT-ABC";
    
    char* assignment1 = agent_manager_assign_task_to_agent(manager, task1_id, id1, project_id);
    if (assignment1) {
        printf("Assigned task %s to agent %s\n", task1_id, agent1.name);
        free(assignment1);
    } else {
        printf("Failed to assign task %s\n", task1_id);
    }

    char task2_id[] = "TASK-002";
    char* assignment2 = agent_manager_assign_task_to_agent(manager, task2_id, id2, project_id);
    if (assignment2) {
        printf("Assigned task %s to agent %s\n", task2_id, agent2.name);
        free(assignment2);
    } else {
        printf("Failed to assign task %s\n", task2_id);
    }

    // Check available agents (should be none now)
    int available_count;
    struct Agent** available = agent_manager_get_available_agents(manager, &available_count);
    printf("\nAvailable agents: %d\n", available_count);
    free(available);

    // Complete a task
    if (agent_manager_complete_agent_assignment(manager, task1_id) == 0) {
        printf("\nCompleted task %s\n", task1_id);
    }

    // Check statistics
    struct AgentStatistics* stats = agent_manager_get_agent_statistics(manager);
    if (stats) {
        printf("\n=== System Statistics ===\n");
        printf("Total agents: %d\n", stats->total_agents);
        printf("Available agents: %d\n", stats->available_agents);
        printf("Busy agents: %d\n", stats->busy_agents);
        printf("Total assignments: %d\n", stats->total_assignments);
        printf("Completed assignments: %d\n", stats->completed_assignments);
        printf("Completion rate: %.2f%%\n", agent_statistics_completion_rate(stats));
        agent_statistics_free(stats);
    }

    // Clean up
    free(id1);
    free(id2);
    agent_manager_free(manager);

    return 0;
}
