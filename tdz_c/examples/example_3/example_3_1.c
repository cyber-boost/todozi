// example3.c - Agent Task Assignment and Statistics Example
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Include the agent management system
extern struct AgentManager* agent_manager_new();
extern void agent_manager_free(struct AgentManager* manager);
extern char* agent_manager_create_agent(struct AgentManager* manager, struct Agent* agent);
extern struct Agent* agent_manager_get_agent(const struct AgentManager* manager, const char* agent_id);
extern struct Agent** agent_manager_get_available_agents(const struct AgentManager* manager, int* count);
extern struct Agent* agent_manager_find_best_agent(const struct AgentManager* manager, const char* required_specialization, const char* preferred_capability);
extern char* agent_manager_assign_task_to_agent(struct AgentManager* manager, char* task_id, const char* agent_id, char* project_id);
extern int agent_manager_complete_agent_assignment(struct AgentManager* manager, const char* task_id);
extern struct AgentStatistics* agent_manager_get_agent_statistics(const struct AgentManager* manager);
extern void agent_statistics_free(struct AgentStatistics* stats);
extern double agent_statistics_completion_rate(const struct AgentStatistics* stats);

// Helper functions (externally defined)
extern time_t get_current_time();
extern void string_array_free(char** array, int count);
extern int string_array_copy(char*** dest, int* dest_count, char** src, int src_count);

int main() {
    printf("=== Agent Task Assignment and Statistics Example ===\n\n");

    // Create agent manager
    struct AgentManager* manager = agent_manager_new();
    if (!manager) {
        printf("Failed to create agent manager\n");
        return 1;
    }

    // Create agents with different specializations and capabilities
    struct Agent agent1 = {
        .name = "Data Analyst Pro",
        .description = "Expert in data analysis and visualization",
        .metadata.status = 0, // AGENT_STATUS_AVAILABLE
        .created_at = get_current_time(),
        .updated_at = get_current_time()
    };
    char* data_capabilities[] = {"Python", "SQL", "Tableau"};
    char* data_specializations[] = {"Data Analysis", "Reporting"};
    string_array_copy(&agent1.capabilities, &agent1.capabilities_count, data_capabilities, 3);
    string_array_copy(&agent1.specializations, &agent1.specializations_count, data_specializations, 2);

    struct Agent agent2 = {
        .name = "Web Developer Elite",
        .description = "Full-stack web development specialist",
        .metadata.status = 0, // AGENT_STATUS_AVAILABLE
        .created_at = get_current_time(),
        .updated_at = get_current_time()
    };
    char* web_capabilities[] = {"JavaScript", "React", "Node.js"};
    char* web_specializations[] = {"Web Development", "UI/UX"};
    string_array_copy(&agent2.capabilities, &agent2.capabilities_count, web_capabilities, 3);
    string_array_copy(&agent2.specializations, &agent2.specializations_count, web_specializations, 2);

    // Register agents
    char* id1 = agent_manager_create_agent(manager, &agent1);
    char* id2 = agent_manager_create_agent(manager, &agent2);
    
    if (!id1 || !id2) {
        printf("Failed to create agents\n");
        agent_manager_free(manager);
        free(id1);
        free(id2);
        return 1;
    }

    printf("Created agents:\n");
    printf("- %s: %s\n", id1, agent1.name);
    printf("- %s: %s\n\n", id2, agent2.name);

    // Find best agent for a task
    printf("Finding best agent for 'Web Development' task...\n");
    struct Agent* best_agent = agent_manager_find_best_agent(manager, "Web Development", "React");
    if (best_agent) {
        printf("Best agent found: %s (ID: %s)\n\n", best_agent->name, best_agent->id);
    } else {
        printf("No suitable agent found\n\n");
    }

    // Assign tasks to agents
    printf("Assigning tasks...\n");
    char* task1_id = agent_manager_assign_task_to_agent(manager, "task-001", id2, "project-web-app");
    char* task2_id = agent_manager_assign_task_to_agent(manager, "task-002", id1, "project-data-report");
    
    if (task1_id && task2_id) {
        printf("Successfully assigned:\n");
        printf("- Task %s to agent %s\n", task1_id, id2);
        printf("- Task %s to agent %s\n\n", task2_id, id1);
    } else {
        printf("Failed to assign tasks\n\n");
    }

    // Check available agents after assignments
    int available_count;
    struct Agent** available_agents = agent_manager_get_available_agents(manager, &available_count);
    printf("Available agents after assignments: %d\n", available_count);
    for (int i = 0; i < available_count; i++) {
        printf("- %s: %s\n", available_agents[i]->id, available_agents[i]->name);
    }
    free(available_agents);
    printf("\n");

    // Complete a task
    printf("Completing task %s...\n", task1_id);
    if (agent_manager_complete_agent_assignment(manager, task1_id) == 0) {
        printf("Task completed successfully\n\n");
    } else {
        printf("Failed to complete task\n\n");
    }

    // Get updated statistics
    struct AgentStatistics* stats = agent_manager_get_agent_statistics(manager);
    if (stats) {
        printf("Agent System Statistics:\n");
        printf("- Total Agents: %d\n", stats->total_agents);
        printf("- Available Agents: %d\n", stats->available_agents);
        printf("- Busy Agents: %d\n", stats->busy_agents);
        printf("- Total Assignments: %d\n", stats->total_assignments);
        printf("- Completed Assignments: %d\n", stats->completed_assignments);
        printf("- Completion Rate: %.2f%%\n\n", agent_statistics_completion_rate(stats));
        agent_statistics_free(stats);
    }

    // Clean up
    free(id1);
    free(id2);
    free(task1_id);
    free(task2_id);
    agent_manager_free(manager);

    printf("Example completed successfully!\n");
    return 0;
}
